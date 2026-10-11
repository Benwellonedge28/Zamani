
#![forbid(unsafe_code)]

//! UTF-8-safe source cursor for the Zamani frontend lexer.
//!
//! File: src/frontend/lexer/cursor.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! # Contract
//!
//! - Byte offsets are zero-based UTF-8 byte offsets.
//! - Lines and Unicode-scalar columns are one-based.
//! - CRLF is one logical line break, including across separate consume calls.
//! - CR, LF, U+2028, and U+2029 are also line breaks.
//! - The source is borrowed and is never modified.
//! - Cursor movement never splits a UTF-8 scalar.
//! - Checkpoints restore all position state.
//! - EOF is deterministic and is not an error.
//! - No language-level source-size or hardware-capacity limits are imposed.
//!
//! Display width, grapheme segmentation, token classification, diagnostics,
//! resource budgets, and semantic interpretation belong to other modules.

use core::fmt;
use core::ops::Range;

/// A source position.
///
/// `byte_offset` is zero-based. `line` and `column` are one-based.
/// Columns count Unicode scalar values, not bytes, graphemes, or terminal
/// display cells.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CursorPosition {
    pub byte_offset: usize,
    pub line: usize,
    pub column: usize,
}

impl CursorPosition {
    #[must_use]
    pub const fn new(byte_offset: usize, line: usize, column: usize) -> Self {
        Self {
            byte_offset,
            line,
            column,
        }
    }

    #[must_use]
    pub const fn byte_offset(self) -> usize {
        self.byte_offset
    }

    #[must_use]
    pub const fn line(self) -> usize {
        self.line
    }

    #[must_use]
    pub const fn column(self) -> usize {
        self.column
    }
}

impl Default for CursorPosition {
    fn default() -> Self {
        Self::new(0, 1, 1)
    }
}

impl fmt::Display for CursorPosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}:{} (byte {})",
            self.line, self.column, self.byte_offset
        )
    }
}

/// An opaque cursor checkpoint.
///
/// Checkpoints should be obtained through [`Cursor::checkpoint`].
/// They retain the complete cursor state, including CRLF bookkeeping.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CursorCheckpoint {
    source_identity: usize,
    source_len: usize,
    position: CursorPosition,
    previous_was_cr: bool,
}

impl CursorCheckpoint {
    #[must_use]
    pub const fn position(self) -> CursorPosition {
        self.position
    }

    #[must_use]
    pub const fn byte_offset(self) -> usize {
        self.position.byte_offset
    }
}

/// Errors from explicit cursor validation operations.
///
/// Reaching EOF is not an error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum CursorError {
    /// A range is reversed or exceeds the source length.
    InvalidRange {
        start: usize,
        end: usize,
        source_len: usize,
    },

    /// A byte offset is not a UTF-8 character boundary.
    NotCharBoundary {
        offset: usize,
    },

    /// A checkpoint does not belong to this source identity.
    ForeignCheckpoint,

    /// A checkpoint has invalid or inconsistent position state.
    InvalidCheckpoint,
}

impl fmt::Display for CursorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidRange {
                start,
                end,
                source_len,
            } => write!(
                f,
                "invalid source range {start}..{end} for source length {source_len}"
            ),
            Self::NotCharBoundary { offset } => {
                write!(f, "byte offset {offset} is not a UTF-8 character boundary")
            }
            Self::ForeignCheckpoint => {
                f.write_str("checkpoint belongs to a different source")
            }
            Self::InvalidCheckpoint => {
                f.write_str("checkpoint contains an invalid cursor position")
            }
        }
    }
}

impl std::error::Error for CursorError {}

/// A forward-only UTF-8 source cursor with explicit checkpoint restoration.
///
/// Ordinary traversal, lookahead, and slicing do not allocate.
///
/// The source is immutable for the cursor's lifetime. All cursor movement
/// goes through the same position-update logic, preventing scanners from
/// maintaining conflicting byte, line, and column counters.
#[derive(Debug, Clone)]
pub struct Cursor<'src> {
    source: &'src str,
    byte_offset: usize,
    line: usize,
    column: usize,
    previous_was_cr: bool,
    source_identity: usize,
}

impl<'src> Cursor<'src> {
    /// Create a cursor at the beginning of `source`.
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

    /// Return the source length in UTF-8 bytes.
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

    /// Return the unconsumed source as a borrowed string slice.
    #[must_use]
    pub fn remaining(&self) -> &'src str {
        // The invariant is maintained by construction and all movement methods.
        &self.source[self.byte_offset..]
    }

    /// Return whether the cursor is at EOF.
    #[must_use]
    pub const fn is_eof(&self) -> bool {
        self.byte_offset == self.source.len()
    }

    /// Look at the next Unicode scalar without consuming it.
    #[must_use]
    pub fn peek(&self) -> Option<char> {
        self.remaining().chars().next()
    }

    /// Look ahead by Unicode scalar count.
    ///
    /// `peek_n(0)` is equivalent to `peek()`.
    /// Work is proportional to the requested lookahead distance.
    #[must_use]
    pub fn peek_n(&self, distance: usize) -> Option<char> {
        self.remaining().chars().nth(distance)
    }

    /// Test the next scalar without advancing.
    #[must_use]
    pub fn peek_is(&self, expected: char) -> bool {
        self.peek() == Some(expected)
    }

    /// Test an exact source prefix without advancing.
    ///
    /// No Unicode normalization or case folding is performed.
    #[must_use]
    pub fn starts_with(&self, text: &str) -> bool {
        self.remaining().starts_with(text)
    }

    /// Consume and return the next Unicode scalar.
    ///
    /// Returns `None` at EOF without changing the cursor.
    pub fn consume(&mut self) -> Option<char> {
        let ch = self.peek()?;
        self.advance_position(ch);
        Some(ch)
    }

    /// Consume the next scalar only when it equals `expected`.
    pub fn consume_if(&mut self, expected: char) -> bool {
        if self.peek() != Some(expected) {
            return false;
        }

        // The preceding peek guarantees a scalar is available.
        let _ = self.consume();
        true
    }

    /// Consume an exact source prefix.
    ///
    /// A mismatch leaves all cursor state unchanged.
    /// An empty prefix matches without advancing.
    pub fn consume_str(&mut self, text: &str) -> bool {
        if !self.starts_with(text) {
            return false;
        }

        let end = self.byte_offset + text.len();

        // Consume scalars rather than modifying the byte offset directly:
        // this preserves line, column, and CRLF state.
        while self.byte_offset < end {
            // The matched string prefix ends on a UTF-8 boundary.
            let _ = self.consume();
        }

        true
    }

    /// Consume consecutive scalars accepted by `predicate`.
    ///
    /// Returns the number of Unicode scalars consumed, not bytes.
    /// The predicate is called on the first nonmatching scalar, if present,
    /// but is never called at EOF.
    pub fn consume_while<F>(&mut self, mut predicate: F) -> usize
    where
        F: FnMut(char) -> bool,
    {
        let mut count = 0usize;

        while let Some(ch) = self.peek() {
            if !predicate(ch) {
                break;
            }

            let _ = self.consume();
            count += 1;
        }

        count
    }

    /// Consume one logical newline when present.
    ///
    /// CRLF is consumed together. Standalone CR, LF, U+2028, and U+2029
    /// are also recognized. Returns `false` without advancing otherwise.
    pub fn consume_newline(&mut self) -> bool {
        match self.peek() {
            Some('\r') => {
                let _ = self.consume();
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

    /// Save the current position for speculative scanning.
    #[must_use]
    pub const fn checkpoint(&self) -> CursorCheckpoint {
        CursorCheckpoint {
            source_identity: self.source_identity,
            source_len: self.source.len(),
            position: self.position(),
            previous_was_cr: self.previous_was_cr,
        }
    }

    /// Restore a checkpoint from this source.
    ///
    /// Validation is performed before state changes, so a failed restore
    /// leaves the cursor unchanged.
    pub fn restore(
        &mut self,
        checkpoint: CursorCheckpoint,
    ) -> Result<(), CursorError> {
        if checkpoint.source_identity != self.source_identity
            || checkpoint.source_len != self.source.len()
        {
            return Err(CursorError::ForeignCheckpoint);
        }

        let p = checkpoint.position;

        if p.byte_offset > self.source.len()
            || !self.source.is_char_boundary(p.byte_offset)
            || p.line == 0
            || p.column == 0
        {
            return Err(CursorError::InvalidCheckpoint);
        }

        let (line, column, previous_was_cr) =
            Self::coordinates_after(&self.source[..p.byte_offset]);

        if (line, column, previous_was_cr)
            != (p.line, p.column, checkpoint.previous_was_cr)
        {
            return Err(CursorError::InvalidCheckpoint);
        }

        self.byte_offset = p.byte_offset;
        self.line = p.line;
        self.column = p.column;
        self.previous_was_cr = checkpoint.previous_was_cr;

        Ok(())
    }

    /// Return a validated borrowed slice of the original source.
    ///
    /// Both endpoints must be ordered, in bounds, and UTF-8 boundaries.
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

        Ok(&self.source[range])
    }

    /// Return the text consumed since `checkpoint`.
    ///
    /// The checkpoint must belong to this source and must not point beyond
    /// the current cursor position. This method does not change cursor state.
    pub fn slice_from(
        &self,
        checkpoint: CursorCheckpoint,
    ) -> Result<&'src str, CursorError> {
        if checkpoint.source_identity != self.source_identity
            || checkpoint.source_len != self.source.len()
        {
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

    /// The sole implementation of forward position updates.
    fn advance_position(&mut self, ch: char) {
        // A valid str has enough representable byte length for this addition.
        self.byte_offset += ch.len_utf8();

        match ch {
            '\r' => {
                self.line += 1;
                self.column = 1;
                self.previous_was_cr = true;
            }
            '\n' if self.previous_was_cr => {
                // CR already advanced the line; LF completes the same newline.
                self.column = 1;
                self.previous_was_cr = false;
            }
            '\n' | '\u{2028}' | '\u{2029}' => {
                self.line += 1;
                self.column = 1;
                self.previous_was_cr = false;
            }
            _ => {
                self.column += 1;
                self.previous_was_cr = false;
            }
        }
    }

    /// Recompute coordinates for checkpoint validation only.
    ///
    /// Normal cursor movement is incremental. This validation path is linear
    /// in the checkpoint offset and should not be used for every character.
    fn coordinates_after(prefix: &str) -> (usize, usize, bool) {
        let mut line = 1usize;
        let mut column = 1usize;
        let mut previous_was_cr = false;

        for ch in prefix.chars() {
            match ch {
                '\r' => {
                    line += 1;
                    column = 1;
                    previous_was_cr = true;
                }
                '\n' if previous_was_cr => {
                    column = 1;
                    previous_was_cr = false;
                }
                '\n' | '\u{2028}' | '\u{2029}' => {
                    line += 1;
                    column = 1;
                    previous_was_cr = false;
                }
                _ => {
                    column += 1;
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
    fn empty_source_and_eof_are_deterministic() {
        let mut c = Cursor::new("");
        assert!(c.is_eof());
        assert_eq!(c.peek(), None);
        assert_eq!(c.consume(), None);
        assert_eq!(c.position(), CursorPosition::new(0, 1, 1));
    }

    #[test]
    fn tracks_utf8_bytes_and_scalar_columns() {
        let mut c = Cursor::new("aλ🙂z");

        for expected in ['a', 'λ', '🙂', 'z'] {
            assert_eq!(c.consume(), Some(expected));
        }

        assert_eq!(c.position(), CursorPosition::new(8, 1, 5));
        assert!(c.is_eof());
    }

    #[test]
    fn tracks_all_supported_newlines() {
        let mut c = Cursor::new("a\r\nb\rc\nd\u{2028}e\u{2029}f");

        while c.consume().is_some() {}

        assert_eq!(c.line(), 6);
        assert_eq!(c.column(), 2);
    }

    #[test]
    fn newline_helper_consumes_crlf_atomically() {
        let mut c = Cursor::new("\r\nx");

        assert!(c.consume_newline());
        assert_eq!(c.position(), CursorPosition::new(2, 2, 1));
        assert_eq!(c.consume(), Some('x'));
        assert_eq!(c.position(), CursorPosition::new(3, 2, 2));
    }

    #[test]
    fn crlf_state_survives_checkpoint_restore() {
        let mut c = Cursor::new("\r\nx");

        assert_eq!(c.consume(), Some('\r'));
        let checkpoint = c.checkpoint();

        assert_eq!(c.consume(), Some('\n'));
        c.restore(checkpoint).expect("checkpoint is valid");

        assert_eq!(c.position(), CursorPosition::new(1, 2, 1));
        assert_eq!(c.consume(), Some('\n'));
        assert_eq!(c.position(), CursorPosition::new(2, 2, 1));
    }

    #[test]
    fn lookahead_and_failed_matches_do_not_advance() {
        let mut c = Cursor::new("αβ");

        let before = c.position();
        assert_eq!(c.peek_n(1), Some('β'));
        assert!(!c.consume_if('x'));
        assert!(!c.consume_str("αx"));
        assert_eq!(c.position(), before);

        assert!(c.consume_str("α"));
        assert_eq!(c.remaining(), "β");
    }

    #[test]
    fn consume_while_counts_scalars() {
        let mut c = Cursor::new("αβ12");

        assert_eq!(c.consume_while(char::is_alphabetic), 2);
        assert_eq!(c.byte_offset(), 4);
        assert_eq!(c.remaining(), "12");
    }

    #[test]
    fn slices_validate_bounds_and_utf8_boundaries() {
        let c = Cursor::new("aλz");

        assert_eq!(c.slice(1..3).expect("valid slice"), "λ");
        assert!(matches!(
            c.slice(2..3),
            Err(CursorError::NotCharBoundary { offset: 2 })
        ));
        assert!(matches!(
            c.slice(0..99),
            Err(CursorError::InvalidRange { .. })
        ));
        assert!(matches!(
            c.slice(3..1),
            Err(CursorError::InvalidRange { .. })
        ));
    }

    #[test]
    fn slice_from_checkpoint_returns_consumed_source() {
        let mut c = Cursor::new("let λ = 1");
        let start = c.checkpoint();

        assert!(c.consume_str("let λ"));
        assert_eq!(c.slice_from(start).expect("valid slice"), "let λ");
    }

    #[test]
    fn checkpoint_restores_unicode_position() {
        let mut c = Cursor::new("αβ\nγ");
        assert_eq!(c.consume(), Some('α'));
        let checkpoint = c.checkpoint();

        assert_eq!(c.consume(), Some('β'));
        assert_eq!(c.consume(), Some('\n'));

        c.restore(checkpoint).expect("valid checkpoint");
        assert_eq!(c.position(), CursorPosition::new(2, 1, 2));
        assert_eq!(c.peek(), Some('β'));
    }

    #[test]
    fn tabs_count_as_one_scalar_column() {
        let mut c = Cursor::new("\tx");

        assert_eq!(c.consume(), Some('\t'));
        assert_eq!(c.position(), CursorPosition::new(1, 1, 2));
    }

    #[test]
    fn repeated_eof_observation_does_not_change_state() {
        let mut c = Cursor::new("x");
        assert_eq!(c.consume(), Some('x'));
        let eof = c.position();

        for _ in 0..10 {
            assert!(c.is_eof());
            assert_eq!(c.peek(), None);
            assert_eq!(c.consume(), None);
            assert_eq!(c.position(), eof);
        }
    }
}
