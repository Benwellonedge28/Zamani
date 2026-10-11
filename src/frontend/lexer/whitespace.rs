#![forbid(unsafe_code)]

//! Whitespace recognition for the Zamani frontend lexer.
//!
//! File: src/frontend/lexer/whitespace.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Recognize consecutive Unicode whitespace characters without modifying
//! source text or duplicating source-position tracking.
//!
//! # Responsibilities
//!
//! This module owns:
//! - the Rust-side whitespace classification predicate;
//! - recognition of consecutive whitespace characters;
//! - reporting the start and end cursor positions;
//! - reporting the number of Unicode scalar values consumed.
//!
//! This module does not own:
//! - source storage or UTF-8 cursor movement;
//! - byte offsets, line numbers, or columns;
//! - token construction or token identity;
//! - trivia preservation or skipping policy;
//! - diagnostic construction or error recovery;
//! - Unicode normalization;
//! - newline normalization;
//! - parser or AST behavior;
//! - classical, quantum, HDL, or other domain semantics;
//! - hardware discovery or resource allocation.
//!
//! # Integration contract
//!
//! `cursor.rs` provides `Cursor` and `CursorPosition`.
//!
//! `scanner.rs` calls `scan_whitespace` when the next source character is
//! whitespace. It uses the returned positions to construct a token:
//!
//! - kind: `TokenKind::Whitespace`;
//! - channel: `TokenChannel::Trivia`;
//! - span: `Span::new(result.start, result.end)`.
//!
//! The scanner then applies `LexerConfig::trivia`:
//!
//! - `TriviaPolicy::Preserve`: emit the whitespace token;
//! - `TriviaPolicy::Skip`: consume the whitespace without emitting a token.
//!
//! `token.rs`, `token_kind.rs`, `span.rs`, `config.rs`, and `api.rs` remain
//! the owners of their respective contracts.
//!
//! This module intentionally has no dependency on token, span, diagnostics,
//! configuration, parser, or backend implementations.
//!
//! # Position and encoding contract
//!
//! - Byte offsets are zero-based UTF-8 byte offsets.
//! - Lines and columns follow `Cursor`'s one-based position convention.
//! - Scalar counts count Unicode scalar values, not bytes or graphemes.
//! - The source is borrowed and remains unchanged.
//! - No Unicode normalization or case conversion is performed.
//! - CRLF and other recognized line endings are handled by `Cursor`.
//!
//! # Scalability
//!
//! Recognition uses constant auxiliary space and performs work proportional
//! to the number of whitespace characters consumed.
//!
//! There is no language-level maximum whitespace-run length. Actual source
//! size and execution remain constrained by available memory, address
//! space, execution time, and any explicitly configured frontend budgets.
//!
//! # Unicode conformance
//!
//! `char::is_whitespace` is the Rust implementation's classification
//! predicate. The normative lexical specification and canonical ANTLR
//! grammar must define compatible behavior for the supported language
//! version. If the specification adopts a different Unicode property or
//! version, change the classification predicate and its conformance tests
//! together; do not silently normalize the source.
//!
//! This component recognizes whitespace only. Whether a newline separates
//! statements is a parser or language-specification concern.

use super::cursor::{Cursor, CursorPosition};

/// The result of consuming one consecutive whitespace run.
///
/// Positions refer to the original source. `scalar_count` counts Unicode
/// scalar values consumed; it does not count UTF-8 bytes or grapheme
/// clusters.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WhitespaceScan {
    /// Position immediately before the first whitespace character.
    pub start: CursorPosition,

    /// Position immediately after the final whitespace character.
    pub end: CursorPosition,

    /// Number of Unicode scalar values consumed.
    pub scalar_count: usize,
}

impl WhitespaceScan {
    /// Returns the number of UTF-8 source bytes covered by this run.
    ///
    /// The cursor guarantees that the end position is not before the start.
    #[must_use]
    pub const fn byte_len(self) -> usize {
        self.end.byte_offset() - self.start.byte_offset()
    }

    /// Returns whether this result represents an empty run.
    ///
    /// `scan_whitespace` never returns an empty run. This helper is useful
    /// to consumers validating results across abstraction boundaries.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.scalar_count == 0
    }
}

/// Returns whether `ch` is whitespace according to Rust's Unicode
/// whitespace classification.
///
/// This predicate recognizes whitespace, not arbitrary control characters.
/// In particular, a byte-order mark is not treated as whitespace by this
/// predicate.
///
/// The canonical Zamani lexical specification remains authoritative. Keep
/// this function aligned with the Unicode whitespace definition adopted by
/// the language specification and ANTLR grammar.
#[must_use]
pub fn is_whitespace(ch: char) -> bool {
    ch.is_whitespace()
}

/// Consumes one consecutive run of whitespace from `cursor`.
///
/// Returns `None` if the cursor is at EOF or its next character is not
/// whitespace. In either case, the cursor is left unchanged.
///
/// On success, the returned range covers exactly the source characters
/// consumed. The original source is neither allocated nor modified.
///
/// The function delegates all cursor movement and source-position updates
/// to `Cursor`, including UTF-8 traversal and line-ending bookkeeping.
///
/// # Complexity
///
/// Time: O(n), where n is the number of Unicode scalar values consumed.
///
/// Auxiliary space: O(1).
///
/// # Examples
///
/// ```
/// use zamani_compiler::frontend::lexer::cursor::Cursor;
/// use zamani_compiler::frontend::lexer::whitespace::scan_whitespace;
///
/// let mut cursor = Cursor::new(" \t\u{00A0}name");
///
/// let run = scan_whitespace(&mut cursor).expect("whitespace is present");
///
/// assert_eq!(run.scalar_count, 3);
/// assert_eq!(run.byte_len(), 4);
/// assert_eq!(cursor.remaining(), "name");
/// ```
pub fn scan_whitespace(cursor: &mut Cursor<'_>) -> Option<WhitespaceScan> {
    let first = cursor.peek()?;

    if !is_whitespace(first) {
        return None;
    }

    let start = cursor.position();

    // Cursor owns all source-position updates. The predicate does not mutate
    // the cursor and has no side effects.
    let scalar_count = cursor.consume_while(is_whitespace);

    // The initial peek established that at least one character matched.
    // Therefore scalar_count is positive for every successful result.
    debug_assert!(scalar_count > 0);

    let end = cursor.position();

    Some(WhitespaceScan {
        start,
        end,
        scalar_count,
    })
}

#[cfg(test)]
mod tests {
    use super::{is_whitespace, scan_whitespace};
    use crate::frontend::lexer::cursor::Cursor;

    #[test]
    fn recognizes_ascii_spaces_and_tabs() {
        let mut cursor = Cursor::new(" \t  name");

        let result = scan_whitespace(&mut cursor).expect("whitespace expected");

        assert_eq!(result.scalar_count, 4);
        assert_eq!(result.byte_len(), 4);
        assert_eq!(result.start.byte_offset(), 0);
        assert_eq!(result.end.byte_offset(), 4);
        assert_eq!(cursor.remaining(), "name");
    }

    #[test]
    fn recognizes_unicode_whitespace_without_normalizing_source() {
        let source = "\u{00A0}\u{2003}\u{202F}x";
        let mut cursor = Cursor::new(source);

        let result = scan_whitespace(&mut cursor).expect("whitespace expected");

        assert_eq!(result.scalar_count, 3);
        assert_eq!(result.byte_len(), "\u{00A0}\u{2003}\u{202F}".len());
        assert_eq!(cursor.remaining(), "x");
    }

    #[test]
    fn returns_none_at_eof_without_moving_cursor() {
        let mut cursor = Cursor::new("");
        let before = cursor.position();

        assert!(scan_whitespace(&mut cursor).is_none());
        assert_eq!(cursor.position(), before);
    }

    #[test]
    fn returns_none_for_non_whitespace_without_moving_cursor() {
        let mut cursor = Cursor::new("alpha");
        let before = cursor.position();

        assert!(scan_whitespace(&mut cursor).is_none());
        assert_eq!(cursor.position(), before);
        assert_eq!(cursor.remaining(), "alpha");
    }

    #[test]
    fn does_not_consume_the_first_non_whitespace_character() {
        let mut cursor = Cursor::new(" \talpha ");

        let result = scan_whitespace(&mut cursor).expect("whitespace expected");

        assert_eq!(result.scalar_count, 2);
        assert_eq!(cursor.remaining(), "alpha ");
    }

    #[test]
    fn preserves_crlf_and_cursor_line_tracking() {
        let mut cursor = Cursor::new(" \r\nx");

        let result = scan_whitespace(&mut cursor).expect("whitespace expected");

        assert_eq!(result.scalar_count, 3);
        assert_eq!(result.byte_len(), 3);
        assert_eq!(cursor.remaining(), "x");
        assert_eq!(cursor.line(), 2);
        assert_eq!(cursor.column(), 1);
    }

    #[test]
    fn recognizes_supported_unicode_line_separators() {
        for separator in ['\n', '\r', '\u{2028}', '\u{2029}'] {
            let mut source = String::new();
            source.push(separator);
            source.push('x');

            let mut cursor = Cursor::new(&source);
            let result =
                scan_whitespace(&mut cursor).expect("line separator expected");

            assert_eq!(result.scalar_count, 1);
            assert_eq!(cursor.remaining(), "x");
            assert_eq!(cursor.line(), 2);
            assert_eq!(cursor.column(), 1);
        }
    }

    #[test]
    fn byte_length_and_scalar_count_are_distinct_for_unicode() {
        let mut cursor = Cursor::new("\u{2003}\u{2003}x");

        let result = scan_whitespace(&mut cursor).expect("whitespace expected");

        assert_eq!(result.scalar_count, 2);
        assert_eq!(result.byte_len(), 6);
        assert!(!result.is_empty());
    }

    #[test]
    fn whitespace_predicate_matches_scan_behavior() {
        let accepted = [' ', '\t', '\n', '\r', '\u{00A0}', '\u{2003}'];

        for ch in accepted {
            assert!(is_whitespace(ch), "{ch:?} should be whitespace");

            let mut source = String::new();
            source.push(ch);
            source.push('x');

            let mut cursor = Cursor::new(&source);
            assert!(scan_whitespace(&mut cursor).is_some());
            assert_eq!(cursor.remaining(), "x");
        }

        for ch in ['a', '0', '_', '+', '\u{FEFF}'] {
            assert!(!is_whitespace(ch), "{ch:?} should not be whitespace");

            let mut source = String::new();
            source.push(ch);

            let mut cursor = Cursor::new(&source);
            assert!(scan_whitespace(&mut cursor).is_none());
            assert_eq!(cursor.byte_offset(), 0);
        }
    }

    #[test]
    fn consumes_a_long_run_without_building_an_intermediate_string() {
        let source = " ".repeat(100_000);
        let mut cursor = Cursor::new(&source);

        let result = scan_whitespace(&mut cursor).expect("whitespace expected");

        assert_eq!(result.scalar_count, source.chars().count());
        assert_eq!(result.byte_len(), source.len());
        assert!(cursor.is_eof());
    }
}