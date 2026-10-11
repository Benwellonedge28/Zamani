#![forbid(unsafe_code)]

//! Annotation-marker recognition for the Zamani frontend lexer.
//!
//! File: `src/frontend/lexer/annotation.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Recognize the single-character annotation marker `@` while preserving
//! the parser's responsibility for the complete annotation construct.
//!
//! For example, the source:
//!
//!     @quantum(targets = q)
//!
//! is lexed compositionally. This module recognizes only `@`. The canonical
//! identifier and operator recognizers process the remaining source.
//!
//! # Ownership
//!
//! This module owns:
//! - recognition of the ASCII annotation marker;
//! - the marker's source start and end positions;
//! - a small, source-oriented result type for scanner integration.
//!
//! This module does not own:
//! - annotation names or a list of reserved annotation names;
//! - qualified names or annotation arguments;
//! - annotation expressions or nested annotation values;
//! - annotation AST nodes or semantic validation;
//! - compiler directives, policies, effects, or capabilities;
//! - classical, quantum, HDL, or hardware-specific semantics;
//! - source cursor implementation or token construction.
//!
//! # Integration contract
//!
//! - `cursor.rs` supplies `Cursor` and `CursorPosition`.
//! - `scanner.rs` calls `scan_annotation_marker` before generic operator
//!   recognition and emits `TokenKind::At`.
//! - `token_kind.rs` owns the canonical `At` token identity.
//! - `token.rs` and `span.rs` own the emitted token and its source span.
//! - The parser owns complete annotation syntax.
//! - Semantic analysis validates annotation names, values, and effects.
//! - `grammar/lexer/annotations.g4` is the normative lexical counterpart.
//!
//! This module deliberately does not construct a `Token` directly. That
//! keeps token-budget checks, span creation, token accounting, and error
//! handling centralized in `scanner.rs`.
//!
//! # Scalability and determinism
//!
//! The recognizer has constant work for one marker and imposes no limit on
//! annotation count, name length, argument count, nesting, program size,
//! or target hardware. Those concerns are handled by the parser, semantic
//! layer, and configurable implementation resource budgets.
//!
//! The only spelling recognized here is `@`. Annotation names remain open
//! to future extensions without modifying this module.

use super::cursor::{Cursor, CursorPosition};

/// A successfully recognized annotation marker.
///
/// The marker always corresponds to the single ASCII character `@`.
/// Positions use the coordinate convention defined by `CursorPosition`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AnnotationMarker {
    start: CursorPosition,
    end: CursorPosition,
}

impl AnnotationMarker {
    /// Creates a marker result from positions supplied by the cursor.
    ///
    /// This constructor is useful for integration and testing. Production
    /// scanning should normally obtain values through
    /// [`scan_annotation_marker`].
    #[must_use]
    pub const fn new(start: CursorPosition, end: CursorPosition) -> Self {
        Self { start, end }
    }

    /// Returns the marker's start position.
    #[must_use]
    pub const fn start(self) -> CursorPosition {
        self.start
    }

    /// Returns the marker's end position.
    #[must_use]
    pub const fn end(self) -> CursorPosition {
        self.end
    }

    /// Returns the marker's half-open UTF-8 byte range.
    ///
    /// Since `@` is ASCII, a valid marker occupies exactly one byte.
    #[must_use]
    pub fn byte_range(self) -> core::ops::Range<usize> {
        self.start.byte_offset()..self.end.byte_offset()
    }

    /// Returns the number of source bytes occupied by the marker.
    ///
    /// A result created by the scanner always has a length of one byte.
    /// The method uses saturating subtraction so even a manually constructed
    /// invalid value cannot underflow.
    #[must_use]
    pub const fn byte_len(self) -> usize {
        self.end
            .byte_offset()
            .saturating_sub(self.start.byte_offset())
    }
}

/// Recognizes an annotation marker at the current cursor position.
///
/// On success, consumes exactly one ASCII `@` and returns its source
/// positions. If the next character is not `@`, returns `None` and leaves
/// the cursor unchanged.
///
/// The function intentionally accepts a bare marker. Whether a name must
/// follow it is a parser-level decision, not a lexical error.
///
/// This function does not consume an annotation name, whitespace, a
/// qualified path, parentheses, arguments, or any other following text.
///
/// # Examples
///
/// ```ignore
/// let mut cursor = Cursor::new("@quantum(target = q)");
///
/// let marker = scan_annotation_marker(&mut cursor)
///     .expect("the marker should be present");
///
/// assert_eq!(marker.byte_len(), 1);
/// assert_eq!(cursor.remaining(), "quantum(target = q)");
/// ```
///
/// The `ignore` attribute in this documentation example avoids imposing
/// assumptions about the visibility of `Cursor::remaining` on downstream
/// documentation builds.
#[must_use]
pub fn scan_annotation_marker(
    cursor: &mut Cursor<'_>,
) -> Option<AnnotationMarker> {
    if !cursor.starts_with("@") {
        return None;
    }

    let start = cursor.position();

    // `starts_with` was checked immediately above, and `@` is one ASCII
    // character. With an exclusively borrowed cursor, this consumption
    // cannot be invalidated by another scanner operation.
    if !cursor.consume_str("@") {
        // Defensive fallback: a failed consume must not produce a token.
        // Cursor implementations should make this branch unreachable.
        return None;
    }

    let end = cursor.position();

    Some(AnnotationMarker { start, end })
}

#[cfg(test)]
mod tests {
    use super::{scan_annotation_marker, AnnotationMarker};
    use crate::frontend::lexer::cursor::Cursor;

    #[test]
    fn recognizes_at_marker() {
        let mut cursor = Cursor::new("@");
        let marker = scan_annotation_marker(&mut cursor)
            .expect("expected annotation marker");

        assert_eq!(marker.byte_len(), 1);
        assert_eq!(marker.byte_range(), 0..1);
        assert!(cursor.is_eof());
    }

    #[test]
    fn consumes_only_the_marker() {
        let mut cursor = Cursor::new("@quantum(targets = q)");
        let marker = scan_annotation_marker(&mut cursor)
            .expect("expected annotation marker");

        assert_eq!(marker.byte_range(), 0..1);
        assert_eq!(cursor.remaining(), "quantum(targets = q)");
    }

    #[test]
    fn accepts_a_bare_marker() {
        let mut cursor = Cursor::new("@");
        assert!(scan_annotation_marker(&mut cursor).is_some());
        assert!(cursor.is_eof());
    }

    #[test]
    fn rejects_non_marker_without_consuming_input() {
        for source in ["", "quantum", " ", "＠", "#", "a", "🙂"] {
            let mut cursor = Cursor::new(source);
            let before = cursor.position();

            assert!(
                scan_annotation_marker(&mut cursor).is_none(),
                "unexpected marker in {source:?}"
            );
            assert_eq!(
                cursor.position(),
                before,
                "cursor moved for {source:?}"
            );
            assert_eq!(cursor.remaining(), source);
        }
    }

    #[test]
    fn recognizes_consecutive_markers_individually() {
        let mut cursor = Cursor::new("@@name");

        let first = scan_annotation_marker(&mut cursor)
            .expect("expected first marker");
        let second = scan_annotation_marker(&mut cursor)
            .expect("expected second marker");

        assert_eq!(first.byte_range(), 0..1);
        assert_eq!(second.byte_range(), 1..2);
        assert_eq!(cursor.remaining(), "name");
    }

    #[test]
    fn source_positions_include_preceding_unicode_bytes() {
        let mut cursor = Cursor::new("λ@name");

        // The Greek letter occupies two UTF-8 bytes.
        assert!(cursor.consume().is_some());

        let marker = scan_annotation_marker(&mut cursor)
            .expect("expected annotation marker");

        assert_eq!(marker.byte_range(), 2..3);
        assert_eq!(marker.byte_len(), 1);
        assert_eq!(cursor.remaining(), "name");
    }

    #[test]
    fn does_not_consume_following_unicode_or_punctuation() {
        let mut cursor = Cursor::new("@名(値 = 🙂)");
        let marker = scan_annotation_marker(&mut cursor)
            .expect("expected annotation marker");

        assert_eq!(marker.byte_range(), 0..1);
        assert_eq!(cursor.remaining(), "名(値 = 🙂)");
    }

    #[test]
    fn marker_byte_range_is_half_open() {
        let mut cursor = Cursor::new("x@y");
        assert!(cursor.consume().is_some());

        let marker = scan_annotation_marker(&mut cursor)
            .expect("expected annotation marker");

        assert_eq!(marker.byte_range(), 1..2);
        assert_eq!(marker.byte_range().end - marker.byte_range().start, 1);
        assert_eq!(cursor.remaining(), "y");
    }

    #[test]
    fn manually_constructed_marker_reports_non_underflowing_length() {
        let mut cursor = Cursor::new("@");
        let actual = scan_annotation_marker(&mut cursor)
            .expect("expected annotation marker");

        assert_eq!(
            AnnotationMarker::new(actual.start(), actual.end()),
            actual
        );
        assert_eq!(actual.byte_len(), 1);
    }

    #[test]
    fn recognition_is_deterministic() {
        for source in ["@", "@name", "@@", "name", "🙂@", ""] {
            let mut first_cursor = Cursor::new(source);
            let mut second_cursor = Cursor::new(source);

            let first = scan_annotation_marker(&mut first_cursor);
            let second = scan_annotation_marker(&mut second_cursor);

            assert_eq!(first, second);
            assert_eq!(first_cursor.position(), second_cursor.position());
            assert_eq!(first_cursor.remaining(), second_cursor.remaining());
        }
    }
}