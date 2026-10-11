
#![forbid(unsafe_code)]

//! UTF-8-safe source cursor for the Zamani frontend lexer.
//!
//! File: src/frontend/lexer/cursor.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Provide the single source of truth for traversing source text and
//! tracking source positions during lexical analysis.
//!
//! # Responsibilities
//!
//! This module owns:
//! - UTF-8-safe character traversal;
//! - zero-based UTF-8 byte offsets;
//! - one-based line and Unicode-scalar column positions;
//! - CRLF, CR, LF, U+2028 and U+2029 newline tracking;
//! - non-consuming character lookahead;
//! - bounded character consumption;
//! - source slicing at validated UTF-8 boundaries;
//! - checkpoint creation and restoration;
//! - deterministic end-of-input behavior.
//!
//! This module does not own:
//! - token identities or token construction;
//! - keyword, operator or dialect classification;
//! - literal parsing or escape decoding;
//! - comment nesting or lexical recovery policy;
//! - diagnostic classification;
//! - AST, semantic analysis or IR construction;
//! - quantum operations, HDL semantics or hardware discovery;
//! - source-size policy or global allocation budgets.
//!
//! # Position contract
//!
//! `byte_offset` is a zero-based UTF-8 byte offset into the original source.
//! `line` and `column` are one-based.
//! `column` counts Unicode scalar values, not bytes, grapheme clusters,
//! terminal cells or UTF-16 code units.
//!
//! CRLF is treated as one logical newline when consumed as a pair.
//! A standalone CR, LF, U+2028 or U+2029 is also a logical newline.
//!
//! Tabs count as one Unicode scalar column. Display-width calculation,
//! tab expansion and grapheme-aware rendering belong to diagnostics/tooling.
//!
//! # Checkpoints
//!
//! A checkpoint is valid only for the cursor/source from which it was
//! obtained. The checkpoint includes newline state so restoration is
//! deterministic, including when positioned between CR and LF.
//!
//! # Scalability
//!
//! The cursor imposes no language-level maximum input length. Byte offsets
//! use `usize`, as required for indexing Rust strings. Consequently, the
//! maximum representable source offset is bounded by the host architecture,
//! the string's actual allocation, and available resources.
//!
//! This is not a claim that a finite computer can process an infinite input.
//! Callers processing untrusted source must enforce configured budgets in
//! `LexerConfig` and the scanner.
//!
//! # Integration contract
//!
//! - `scanner.rs` owns one cursor for each source input.
//! - `identifier.rs`, `number.rs`, `string.rs`, `character.rs`, `comment.rs`,
//!   `whitespace.rs`, `annotation.rs` and `quantum.rs` consume this API rather
//!   than maintaining competing byte/line/column counters.
//! - `span.rs` owns the canonical token/diagnostic `Span` representation.
//!   The scanner converts `CursorPosition` pairs into that representation.
//! - `diagnostics.rs` uses the documented position convention.
//! - `recovery.rs` may use checkpoints when speculative scanning is needed.
//! - `config.rs` and `limits.rs` own configurable resource budgets.
//! - `token.rs` continues to own tokens and does not duplicate cursor state.
//! - Grammar rules define lexical syntax; they do not change cursor semantics.
//!
//! No external dependency is required.

use core::fmt;
use core::ops::Range;

/// A stable snapshot of a cursor's externally visible source position.
///
/// Byte offsets are zero-based. Lines and columns are one-based.
///
/// The column counts Unicode scalar values since the most recent logical
/// newline. It does not attempt to measure terminal display width.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CursorPosition {
    /// Zero-based UTF-8 byte offset into the original source.
    pub byte_offset: usize,

    /// One-based source line.
    pub line: usize,

    /// One-based Unicode-scalar column.
    pub column: usize,
}

impl CursorPosition {
    /// Construct a position from explicit coordinates.
    ///
    /// This constructor does not establish that the coordinates correspond
    /// to a real source position. Positions produced by `Cursor` do.
    #[must_use]
    pub const fn new(byte_offset: usize, line: usize, column: usize) -> Self {
        Self {
            byte_offset,
            line,
            column,
        }
    }

    /// Return the zero-based UTF-8 byte offset.
    #[must_use]
    pub const fn byte_offset(self) -> usize {
        self.byte_offset
    }

    /// Return the one-based line number.
    #[must_use]
    pub const fn line(self) -> usize {
        self.line
    }

    /// Return the one-based column number.
    #[must_use]
    pub const fn column(self) -> usize {
        self.column
    }
}

impl Default for CursorPosition {
    fn default() -> Self {
        Self {
            byte_offset: 0,
            line: 1,
            column: 1,
        }
    }
}

impl fmt::Display for CursorPosition {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}:{} (byte {})",
            self.line, self.column, self.byte_offset
        )
    }
}

/// A checkpoint for restoring a cursor to an earlier position.
///
/// Obtain checkpoints from `Cursor::checkpoint`. Fields are private so
/// callers cannot accidentally manufacture inconsistent internal state.
///
/// A checkpoint is tied to the source and cursor that created it. The
/// cursor validates source identity and basic position invariants before
/// restoring it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CursorCheckpoint {
    source_identity: usize,
    position: CursorPosition,
    previous_was_cr: bool,
}

impl CursorCheckpoint {
    /// Return the saved source position.
    #[must_use]
    pub const fn position(self) -> CursorPosition {
        self.position
    }

    /// Return the saved zero-based UTF-8 byte offset.
    #[must_use]
    pub const fn byte_offset(self) -> usize {
        self.position.byte_offset
    }
}

/// Errors produced by cursor operations.
///
/// Ordinary end-of-input is not an error. `peek` and `consume` return
/// `None` at EOF. Errors are reserved for invalid ranges or checkpoints.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum CursorError {
    /// The requested byte range is outside the source or is not ordered.
    InvalidRange {
        start: usize,
        end: usize,
        source_len: usize,
    },

    /// A range endpoint splits a UTF-8 encoded character.
    NotCharBoundary {
        offset: usize,
    },

    /// The checkpoint was created by a different source allocation.
    ForeignCheckpoint,

    /// The checkpoint's internal coordinates cannot describe a valid
    /// position within the current source.
    InvalidCheckpoint,
}

impl fmt::Display for CursorError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidRange {
                start,
                end,
                source_len,
            } => write!(
                formatter,
                "invalid source range {start}..{end} for source length {source_len}"
            ),
            Self::NotCharBoundary { offset } => {
                write!(formatter, "byte offset {offset} is not a UTF-8 character boundary")
            }
            Self::ForeignCheckpoint => {
                formatter.write_str("checkpoint belongs to a different source")
            }
            Self::InvalidCheckpoint => {
                formatter.write_str("checkpoint contains an invalid cursor position")
            }
        }
    }
}

impl std::error::Error for CursorError {}

/// A forward-only, UTF-8-safe cursor over borrowed source text.
///
/// `Cursor` borrows the source and never allocates while performing ordinary
/// traversal, lookahead, checkpointing or source slicing.
///
/// The cursor is intentionally independent of token kinds, dialects,
/// computational domains, source filenames and diagnostic storage.
#[derive(Debug, Clone)]
pub struct Cursor<'src> {
    source: &'src str,
    byte_offset: usize,
    line: usize,
    column: usize,

    /// Whether the most recently consumed character was CR.
    ///
    /// This lets CRLF count as one newline even when the caller consumes
    /// the two characters through separate operations.
    previous_was_cr: bool,

    /// Identity of the borrowed source allocation, used to reject ordinary
    /// cross-source checkpoint misuse.
    source_identity: usize,
}

impl<'src> Cursor<'src> {
    /// Create a cursor at the beginning of `source`.
    ///
    /// The initial position is byte offset 0, line 1, column 1.
    #[must_use]
    pub fn new(source: &'src str) -> Self {
        Self {
            source,
            byte_offset: 0,
            line: 1,
            column: 1,
            previous_was_cr: false,
            source_identity: source.as_ptr() as usize,
        }
    }

    /// Return the original source.
    #[must_use]
    pub const fn source(&self) -> &'src str {
        self.source
    }

    /// Return the complete source length in UTF-8 bytes.
    #[must_use]
    pub const fn source_len_bytes(&self) -> usize {
        self.source.len()
    }

    /// Return the current zero-based UTF-8 byte offset.
    #[must_use]
    pub const fn byte_offset(&self) -> usize {
        self.byte_offset
    }

    /// Return the current one-based line.
    #[must_use]
    pub const fn line(&self) -> usize {
        self.line
    }

    /// Return the current one-based Unicode-scalar column.
    #[must_use]
    pub const fn column(&self) -> usize {
        self.column
    }

    /// Return the current source position.
    #[must_use]
    pub const fn position(&self) -> CursorPosition {
        CursorPosition::new(self.byte_offset, self.line, self.column)
    }

    /// Return the remaining source as a borrowed string slice.
    ///
    /// The cursor invariant guarantees that `byte_offset` is always a
    /// UTF-8 character boundary within the source.
    #[must_use]
    pub fn remaining(&self) -> &'src str {
        // Safe because all cursor movement preserves UTF-8 boundaries.
        &self.source[self.byte_offset..]
    }

    /// Return true when the cursor has reached the end of the source.
    #[must_use]
    pub const fn is_eof(&self) -> bool {
        self.byte_offset == self.source.len()
    }

    /// Return the next Unicode scalar without advancing.
    #[must_use]
    pub fn peek(&self) -> Option<char> {
        self.remaining().chars().next()
    }

    /// Return the Unicode scalar after the next `distance` scalars.
    ///
    /// `peek_n(0)` is equivalent to `peek()`. This operation does not
    /// advance the cursor. Lookahead is character-based, not byte-based.
    ///
    /// The work is proportional to the requested lookahead distance.
    #[must_use]
    pub fn peek_n(&self, distance: usize) -> Option<char> {
        self.remaining().chars().nth(distance)
    }

    /// Return the next Unicode scalar if it equals `expected`.
    ///
    /// This does not advance the cursor.
    #[must_use]
    pub fn peek_is(&self, expected: char) -> bool {
        self.peek() == Some(expected)
    }

    /// Return true if the remaining source begins with `text`.
    ///
    /// This compares exact source spelling. It performs no normalization
    /// and makes no assumptions about identifiers or dialects.
    #[must_use]
    pub fn starts_with(&self, text: &str) -> bool {
        self.remaining().starts_with(text)
    }

    /// Consume and return the next Unicode scalar.
    ///
    /// Returns `None` at EOF. Every successful call advances by exactly
    /// the UTF-8 byte length of the returned scalar.
    ///
    /// Newline coordinates are updated consistently for CRLF, standalone
    /// CR, LF, U+2028 and U+2029.
    pub fn consume(&mut self) -> Option<char> {
        let ch = self.peek()?;
        self.advance_position(ch);
        Some(ch)
    }

    /// Consume the next scalar only if it equals `expected`.
    ///
    /// Returns `true` and advances on a match; otherwise returns `false`
    /// without changing any cursor state.
    pub fn consume_if(&mut self, expected: char) -> bool {
        if self.peek() == Some(expected) {
            // The preceding peek guarantees that consume succeeds.
            let _ = self.consume();
            true
        } else {
            false
        }
    }

    /// Consume `text` if the remaining source starts with it.
    ///
    /// An empty string matches and leaves the cursor unchanged.
    /// This operation is exact and does not normalize Unicode.
    pub fn consume_str(&mut self, text: &str) -> bool {
        if !self.starts_with(text) {
            return false;
        }

        // `starts_with` establishes that the exact byte prefix is present.
        // A valid `str` prefix always ends at a UTF-8 boundary.
        let end = self.byte_offset + text.len();

        // Advance through characters rather than directly changing the
        // byte offset so line, column and CRLF state remain correct.
        while self.byte_offset < end {
            // A matched prefix cannot end in the middle of a UTF-8 scalar.
            let _ = self.consume();
        }

        true
    }

    /// Consume characters while `predicate` returns true.
    ///
    /// Returns the number of consumed Unicode scalars.
    ///
    /// The predicate is called once per examined scalar, including the
    /// first scalar that does not match. It is never called at EOF.
    pub fn consume_while<F>(&mut self, mut predicate: F) -> usize
    where
        F: FnMut(char) -> bool,
    {
        let mut consumed = 0usize;

        while let Some(ch) = self.peek() {
            if !predicate(ch) {
                break;
            }

            let _ = self.consume();
            consumed = consumed.saturating_add(1);
        }

        consumed
    }

    /// Consume a logical newline if one starts at the current position.
    ///
    /// CRLF is consumed as one logical newline and one operation.
    /// Standalone CR, LF, U+2028 and U+2029 are also recognized.
    ///
    /// Returns `true` if a newline was consumed.
    pub fn consume_newline(&mut self) -> bool {
        match self.peek() {
            Some('\r') => {
                let _ = self.consume();

                // CR has already advanced the line. Consuming LF immediately
                // after CR must not advance the line a second time.
                let _ = self.consume_if('\n');
                true
            }
            Some('\n' | '\u{2028}' | '\u{2029}') => {
                let _ = self.consume();
                true
            }
            _ => false,
        }
    }

    /// Save the current position for possible restoration.
    ///
    /// Checkpoints preserve CRLF state as well as visible coordinates.
    #[must_use]
    pub const fn checkpoint(&self) -> CursorCheckpoint {
        CursorCheckpoint {
            source_identity: self.source_identity,
            position: self.position(),
            previous_was_cr: self.previous_was_cr,
        }
    }

    /// Restore a checkpoint created from this source.
    ///
    /// Returns an error if the checkpoint belongs to a different source
    /// or its position is invalid for this cursor.
    pub fn restore(
        &mut self,
        checkpoint: CursorCheckpoint,
    ) -> Result<(), CursorError> {
        if checkpoint.source_identity != self.source_identity {
            return Err(CursorError::ForeignCheckpoint);
        }

        let position = checkpoint.position;

        if position.byte_offset > self.source.len()
            || !self.source.is_char_boundary(position.byte_offset)
            || position.line == 0
            || position.column == 0
        {
            return Err(CursorError::InvalidCheckpoint);
        }

        // Verify that the visible coordinates agree with the source prefix.
        // This prevents restoration to internally inconsistent coordinates.
        let prefix = &self.source[..position.byte_offset];
        let (expected_line, expected_column, expected_previous_was_cr) =
            Self::coordinates_after(prefix);

        if expected_line != position.line
            || expected_column != position.column
            || expected_previous_was_cr != checkpoint.previous_was_cr
        {
            return Err(CursorError::InvalidCheckpoint);
        }

        self.byte_offset = position.byte_offset;
        self.line = position.line;
        self.column = position.column;
        self.previous_was_cr = checkpoint.previous_was_cr;

        Ok(())
    }

    /// Return a validated borrowed source slice for a byte range.
    ///
    /// Both endpoints must be UTF-8 character boundaries and the range
    /// must be ordered and contained in the source.
    pub fn slice(&self, range: Range<usize>) -> Result<&'src str, CursorError> {
        if range.start > range.end || range.end > self.source.len() {
            return Err(CursorError::InvalidRange {
                start: range.start,
                end: range.end,
                source_len: self.source.len(),
            });
        }

        if !self.source.is_char_boundary(range.start) {
            return Err(CursorError::NotCharBoundary {
                offset: range.start,
            });
        }

        if !self.source.is_char_boundary(range.end) {
            return Err(CursorError::NotCharBoundary {
                offset: range.end,
            });
        }

        // Both endpoints have been validated.
        Ok(&self.source[range])
    }

    /// Return the source slice from a saved checkpoint to the current
    /// cursor position.
    ///
    /// The checkpoint must belong to this source and must not point beyond
    /// the current position. Cursor coordinates are not modified.
    pub fn slice_from(
        &self,
        checkpoint: CursorCheckpoint,
    ) -> Result<&'src str, CursorError> {
        if checkpoint.source_identity != self.source_identity {
            return Err(CursorError::ForeignCheckpoint);
        }

        let start = checkpoint.position.byte_offset;

        if start > self.byte_offset {
            return Err(CursorError::InvalidRange {
                start,
                end: self.byte_offset,
                source_len: self.source.len(),
            });
        }

        self.slice(start..self.byte_offset)
    }

    /// Advance through one scalar and update position state.
    ///
    /// This private method is the sole owner of normal cursor advancement.
    fn advance_position(&mut self, ch: char) {
        self.byte_offset += ch.len_utf8();

        match ch {
            '\r' => {
                self.line = self.line.saturating_add(1);
                self.column = 1;
                self.previous_was_cr = true;
            }
            '\n' if self.previous_was_cr => {
                // CR already advanced the line. LF completes CRLF.
                self.column = 1;
                self.previous_was_cr = false;
            }
            '\n' | '\u{2028}' | '\u{2029}' => {
                self.line = self.line.saturating_add(1);
                self.column = 1;
                self.previous_was_cr = false;
            }
            _ => {
                self.column = self.column.saturating_add(1);
                self.previous_was_cr = false;
            }
        }
    }

    /// Recompute position state from a source prefix.
    ///
    /// Used only to validate a checkpoint. Normal traversal remains
    /// incremental and does not rescan earlier source.
    fn coordinates_after(prefix: &str) -> (usize, usize, bool) {
        let mut line = 1usize;
        let mut column = 1usize;
        let mut previous_was_cr = false;

        for ch in prefix.chars() {
            match ch {
                '\r' => {
                    line = line.saturating_add(1);
                    column = 1;
                    previous_was_cr = true;
                }
                '\n' if previous_was_cr => {
                    column = 1;
                    previous_was_cr = false;
                }
                '\n' | '\u{2028}' | '\u{2029}' => {
                    line = line.saturating_add(1);
                    column = 1;
                    previous_was_cr = false;
                }
                _ => {
                    column = column.saturating_add(1);
                    previous_was_cr = false;
                }
            }
        }

        (line, column, previous_was_cr)
    }
}

impl<'src> From<&'src str> for Cursor<'src> {
    fn from(source: &'src str) -> Self {
        Self::new(source)
    }
}

#[cfg(test)]
mod tests {
    use super::{Cursor, CursorError, CursorPosition};

    #[test]
    fn starts_at_beginning() {
        let cursor = Cursor::new("abc");

        assert_eq!(cursor.position(), CursorPosition::new(0, 1, 1));
        assert_eq!(cursor.peek(), Some('a'));
        assert_eq!(cursor.remaining(), "abc");
        assert!(!cursor.is_eof());
    }

    #[test]
    fn consumes_ascii_without_skipping() {
        let mut cursor = Cursor::new("abc");

        assert_eq!(cursor.consume(), Some('a'));
        assert_eq!(cursor.consume(), Some('b'));
        assert_eq!(cursor.consume(), Some('c'));
        assert_eq!(cursor.consume(), None);
        assert!(cursor.is_eof());
        assert_eq!(cursor.byte_offset(), 3);
        assert_eq!(cursor.position(), CursorPosition::new(3, 1, 4));
    }

    #[test]
    fn counts_utf8_bytes_and_unicode_scalar_columns_separately() {
        let mut cursor = Cursor::new("aλ🙂z");

        assert_eq!(cursor.consume(), Some('a'));
        assert_eq!(cursor.position(), CursorPosition::new(1, 1, 2));

        assert_eq!(cursor.consume(), Some('λ'));
        assert_eq!(cursor.position(), CursorPosition::new(3, 1, 3));

        assert_eq!(cursor.consume(), Some('🙂'));
        assert_eq!(cursor.position(), CursorPosition::new(7, 1, 4));

        assert_eq!(cursor.consume(), Some('z'));
        assert_eq!(cursor.position(), CursorPosition::new(8, 1, 5));
    }

    #[test]
    fn handles_lf_newlines() {
        let mut cursor = Cursor::new("a\nb");

        assert_eq!(cursor.consume(), Some('a'));
        assert_eq!(cursor.consume(), Some('\n'));
        assert_eq!(cursor.position(), CursorPosition::new(2, 2, 1));
        assert_eq!(cursor.consume(), Some('b'));
        assert_eq!(cursor.position(), CursorPosition::new(3, 2, 2));
    }

    #[test]
    fn handles_crlf_as_one_logical_newline() {
        let mut cursor = Cursor::new("a\r\nb");

        assert_eq!(cursor.consume(), Some('a'));
        assert_eq!(cursor.consume(), Some('\r'));
        assert_eq!(cursor.position(), CursorPosition::new(2, 2, 1));

        assert_eq!(cursor.consume(), Some('\n'));
        assert_eq!(cursor.position(), CursorPosition::new(3, 2, 1));

        assert_eq!(cursor.consume(), Some('b'));
        assert_eq!(cursor.position(), CursorPosition::new(4, 2, 2));
    }

    #[test]
    fn handles_standalone_cr_and_unicode_line_separators() {
        for newline in ["\r", "\u{2028}", "\u{2029}"] {
            let source = format!("a{newline}b");
            let mut cursor = Cursor::new(&source);

            assert_eq!(cursor.consume(), Some('a'));
            assert!(cursor.consume_newline());
            assert_eq!(cursor.position().line, 2);
            assert_eq!(cursor.position().column, 1);
            assert_eq!(cursor.consume(), Some('b'));
            assert_eq!(cursor.position().column, 2);
        }
    }

    #[test]
    fn newline_helper_consumes_crlf_atomically() {
        let mut cursor = Cursor::new("\r\nx");

        assert!(cursor.consume_newline());
        assert_eq!(cursor.byte_offset(), 2);
        assert_eq!(cursor.position(), CursorPosition::new(2, 2, 1));
        assert_eq!(cursor.consume(), Some('x'));
        assert_eq!(cursor.position(), CursorPosition::new(3, 2, 2));
    }

    #[test]
    fn lookahead_does_not_advance() {
        let cursor = Cursor::new("aλ🙂");

        assert_eq!(cursor.peek(), Some('a'));
        assert_eq!(cursor.peek_n(0), Some('a'));
        assert_eq!(cursor.peek_n(1), Some('λ'));
        assert_eq!(cursor.peek_n(2), Some('🙂'));
        assert_eq!(cursor.peek_n(3), None);
        assert_eq!(cursor.byte_offset(), 0);
    }

    #[test]
    fn consume_if_preserves_state_on_mismatch() {
        let mut cursor = Cursor::new("abc");
        let before = cursor.position();

        assert!(!cursor.consume_if('x'));
        assert_eq!(cursor.position(), before);

        assert!(cursor.consume_if('a'));
        assert_eq!(cursor.byte_offset(), 1);
    }

    #[test]
    fn consume_str_matches_exact_source_text() {
        let mut cursor = Cursor::new("quantum::gate");

        assert!(!cursor.consume_str("Quantum"));
        assert_eq!(cursor.byte_offset(), 0);

        assert!(cursor.consume_str("quantum::"));
        assert_eq!(cursor.remaining(), "gate");
        assert_eq!(cursor.byte_offset(), 9);
    }

    #[test]
    fn consume_str_preserves_newline_tracking() {
        let mut cursor = Cursor::new("a\r\nb");

        assert!(cursor.consume_str("a\r\n"));
        assert_eq!(cursor.position(), CursorPosition::new(3, 2, 1));
    }

    #[test]
    fn consume_while_counts_scalars() {
        let mut cursor = Cursor::new("αβ12!");

        let count = cursor.consume_while(char::is_alphabetic);

        assert_eq!(count, 2);
        assert_eq!(cursor.remaining(), "12!");
        assert_eq!(cursor.byte_offset(), 4);
        assert_eq!(cursor.column(), 3);
    }

    #[test]
    fn checkpoints_restore_all_position_state() {
        let mut cursor = Cursor::new("α\r\nβ");

        let checkpoint = cursor.checkpoint();
        assert_eq!(cursor.consume(), Some('α'));
        assert_eq!(cursor.consume(), Some('\r'));
        assert_eq!(cursor.consume(), Some('\n'));
        assert_eq!(cursor.consume(), Some('β'));

        cursor.restore(checkpoint).expect("same-source checkpoint");

        assert_eq!(cursor.byte_offset(), 0);
        assert_eq!(cursor.line(), 1);
        assert_eq!(cursor.column(), 1);
        assert_eq!(cursor.peek(), Some('α'));
    }

    #[test]
    fn checkpoint_restores_state_between_cr_and_lf() {
        let mut cursor = Cursor::new("\r\nx");

        assert_eq!(cursor.consume(), Some('\r'));
        let checkpoint = cursor.checkpoint();

        assert_eq!(cursor.consume(), Some('\n'));
        assert_eq!(cursor.position(), CursorPosition::new(2, 2, 1));

        cursor.restore(checkpoint).expect("valid checkpoint");
        assert_eq!(cursor.position(), CursorPosition::new(1, 2, 1));

        assert_eq!(cursor.consume(), Some('\n'));
        assert_eq!(cursor.position(), CursorPosition::new(2, 2, 1));
    }

    #[test]
    fn slice_accepts_valid_utf8_ranges() {
        let cursor = Cursor::new("aλ🙂z");

        assert_eq!(cursor.slice(0..1).expect("ASCII slice"), "a");
        assert_eq!(cursor.slice(1..3).expect("Greek scalar"), "λ");
        assert_eq!(cursor.slice(3..7).expect("emoji scalar"), "🙂");
        assert_eq!(cursor.slice(0..8).expect("whole source"), "aλ🙂z");
        assert_eq!(cursor.slice(8..8).expect("empty suffix"), "");
    }

    #[test]
    fn slice_rejects_out_of_bounds_ranges() {
        let cursor = Cursor::new("abc");

        assert!(matches!(
            cursor.slice(0..4),
            Err(CursorError::InvalidRange { .. })
        ));

        assert!(matches!(
            cursor.slice(2..1),
            Err(CursorError::InvalidRange { .. })
        ));
    }

    #[test]
    fn slice_rejects_non_character_boundaries() {
        let cursor = Cursor::new("λ");

        assert!(matches!(
            cursor.slice(1..2),
            Err(CursorError::NotCharBoundary { offset: 1 })
        ));

        assert!(matches!(
            cursor.slice(0..1),
            Err(CursorError::NotCharBoundary { offset: 1 })
        ));
    }

    #[test]
    fn slice_from_checkpoint_returns_consumed_text() {
        let mut cursor = Cursor::new("let λ = 1");

        let start = cursor.checkpoint();
        assert!(cursor.consume_str("let "));
        assert!(cursor.consume_str("λ"));

        assert_eq!(cursor.slice_from(start).expect("valid slice"), "let λ");
    }

    #[test]
    fn empty_source_is_already_at_eof() {
        let mut cursor = Cursor::new("");

        assert!(cursor.is_eof());
        assert_eq!(cursor.peek(), None);
        assert_eq!(cursor.consume(), None);
        assert_eq!(cursor.position(), CursorPosition::new(0, 1, 1));
    }

    #[test]
    fn repeated_eof_observation_is_deterministic() {
        let mut cursor = Cursor::new("x");

        assert_eq!(cursor.consume(), Some('x'));
        let eof_position = cursor.position();

        for _ in 0..10 {
            assert!(cursor.is_eof());
            assert_eq!(cursor.peek(), None);
            assert_eq!(cursor.consume(), None);
            assert_eq!(cursor.position(), eof_position);
        }
    }

    #[test]
    fn tab_counts_as_one_scalar_column() {
        let mut cursor = Cursor::new("\tx");

        assert_eq!(cursor.consume(), Some('\t'));
        assert_eq!(cursor.position(), CursorPosition::new(1, 1, 2));
        assert_eq!(cursor.consume(), Some('x'));
        assert_eq!(cursor.position(), CursorPosition::new(2, 1, 3));
    }

    #[test]
    fn mixed_newlines_produce_deterministic_positions() {
        let mut cursor = Cursor::new("a\r\nb\rc\nd\u{2028}e\u{2029}f");

        let mut observed = Vec::new();

        while let Some(ch) = cursor.consume() {
            if matches!(ch, '\r' | '\n' | '\u{2028}' | '\u{2029}') {
                observed.push((ch, cursor.position()));
            }
        }

        assert_eq!(
            observed,
            vec![
                ('\r', CursorPosition::new(2, 2, 1)),
                ('\n', CursorPosition::new(3, 2, 1)),
                ('\r', CursorPosition::new(5, 3, 1)),
                ('\n', CursorPosition::new(7, 4, 1)),
                ('\u{2028}', CursorPosition::new(9, 5, 1)),
                ('\u{2029}', CursorPosition::new(13, 6, 1)),
            ]
        );
    }
}
