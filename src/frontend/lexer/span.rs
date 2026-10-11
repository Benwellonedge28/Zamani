#![forbid(unsafe_code)]

//! Canonical source spans for the Zamani frontend.
//!
//! File: src/frontend/lexer/span.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! # Purpose
//!
//! Represent source locations consistently across lexical recognition,
//! diagnostics, parsing, semantic analysis, and developer tooling.
//!
//! # Ownership
//!
//! This module owns:
//! - source start and end positions;
//! - half-open source-range semantics;
//! - byte-range and position accessors;
//! - span construction and invariant validation;
//! - span comparison, hashing, and display;
//! - explicitly checked span-composition operations.
//!
//! This module does not own:
//! - source text or source-file identity;
//! - cursor advancement or Unicode decoding;
//! - token classification;
//! - diagnostic policy or recovery;
//! - AST node identity;
//! - semantic interpretation;
//! - quantum, classical, HDL, or hardware behavior;
//! - target discovery or resource allocation.
//!
//! # Integration contract
//!
//! `cursor.rs` owns `CursorPosition` and its position convention:
//!
//! - byte offsets are zero-based UTF-8 byte offsets;
//! - line numbers are one-based;
//! - columns are one-based Unicode-scalar columns.
//!
//! `scanner.rs` constructs spans using `Span::new(start, end)`.
//! `token.rs` stores a `Span` in each token.
//! `diagnostics.rs` uses spans to identify source ranges.
//! `recovery.rs` uses positions and spans when synchronizing after errors.
//! Parser and AST modules may retain spans for later diagnostics.
//!
//! A span does not contain the source text. Its byte offsets are meaningful
//! relative to the source from which its positions originated. A span alone
//! cannot prove that it belongs to a particular source file.
//!
//! # Range semantics
//!
//! Spans are half-open: `[start, end)`.
//!
//! The start position is included and the end position is excluded.
//! A zero-width span is valid and is used for EOF and insertion diagnostics.
//!
//! A valid span has:
//!
//! - an end byte offset greater than or equal to its start byte offset;
//! - an end position that does not precede its start position;
//! - positions expressed in the same coordinate convention.
//!
//! Construction can validate position ordering, but without source text it
//! cannot independently verify UTF-8 boundaries or recompute line and column
//! coordinates. The cursor is responsible for those guarantees.
//!
//! # Scalability
//!
//! Spans use constant storage and perform constant-time basic access.
//! There is no language-level limit on source length, token count, or hardware
//! capacity. The representable byte range is bounded by `usize`, the host
//! architecture, and the source representation used by the compiler.
//!
//! # Compatibility
//!
//! `Span::new(start, end)` is the canonical constructor used by the current
//! scanner. `Span::try_new(start, end)` provides checked construction.
//! Existing consumers should use accessors rather than depending on the
//! internal representation.
//!
//! No unsafe Rust is required.

use core::fmt;
use core::ops::Range;

use super::cursor::CursorPosition;

/// A half-open source range.
///
/// The span stores the start and end positions but does not own or identify
/// the source file. The source owner must ensure that both positions belong
/// to the same source and use the coordinate convention defined by
/// `CursorPosition`.
///
/// `Span` is intentionally independent of token kinds, parser nodes, and
/// semantic domains.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Span {
    start: CursorPosition,
    end: CursorPosition,
}

/// Errors produced when validating or composing source spans.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum SpanError {
    /// The end byte offset precedes the start byte offset.
    ReversedByteRange {
        start: usize,
        end: usize,
    },

    /// The end source position precedes the start source position.
    ReversedPositionRange,

    /// The requested subspan is outside the parent span.
    OutsideParent {
        parent_start: usize,
        parent_end: usize,
        child_start: usize,
        child_end: usize,
    },

    /// The positions do not satisfy the supported ordering invariants.
    InconsistentPositions,
}

impl fmt::Display for SpanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ReversedByteRange { start, end } => {
                write!(
                    f,
                    "span end byte offset {end} precedes start byte offset {start}"
                )
            }
            Self::ReversedPositionRange => {
                f.write_str("span end position precedes start position")
            }
            Self::OutsideParent {
                parent_start,
                parent_end,
                child_start,
                child_end,
            } => write!(
                f,
                "child span {child_start}..{child_end} is outside \
                 parent span {parent_start}..{parent_end}"
            ),
            Self::InconsistentPositions => {
                f.write_str("span positions are internally inconsistent")
            }
        }
    }
}

impl std::error::Error for SpanError {}

impl Span {
    /// Construct a span from source positions.
    ///
    /// The canonical scanner interface uses this constructor.
    ///
    /// # Panics
    ///
    /// Panics if the end byte offset precedes the start byte offset or if
    /// the end position precedes the start position.
    ///
    /// Use [`Span::try_new`] when positions originate from an untrusted
    /// external source or when construction must be recoverable.
    #[must_use]
    pub fn new(start: CursorPosition, end: CursorPosition) -> Self {
        match Self::try_new(start, end) {
            Ok(span) => span,
            Err(error) => panic!("invalid source span: {error}"),
        }
    }

    /// Construct a span while checking its ordering invariants.
    ///
    /// This validates ordering only. It cannot verify source-file identity,
    /// UTF-8 character boundaries, or whether line and column values match
    /// the source text because the source text is not stored in a span.
    pub fn try_new(
        start: CursorPosition,
        end: CursorPosition,
    ) -> Result<Self, SpanError> {
        if end.byte_offset() < start.byte_offset() {
            return Err(SpanError::ReversedByteRange {
                start: start.byte_offset(),
                end: end.byte_offset(),
            });
        }

        if end < start {
            return Err(SpanError::ReversedPositionRange);
        }

        Ok(Self { start, end })
    }

    /// Construct a zero-width span at a source position.
    ///
    /// This is appropriate for EOF tokens and diagnostics describing an
    /// insertion point.
    #[must_use]
    pub const fn at(position: CursorPosition) -> Self {
        Self {
            start: position,
            end: position,
        }
    }

    /// Return the starting source position.
    #[must_use]
    pub const fn start(&self) -> CursorPosition {
        self.start
    }

    /// Return the ending source position.
    #[must_use]
    pub const fn end(&self) -> CursorPosition {
        self.end
    }

    /// Return the starting zero-based UTF-8 byte offset.
    #[must_use]
    pub const fn start_byte(&self) -> usize {
        self.start.byte_offset()
    }

    /// Return the ending zero-based UTF-8 byte offset.
    #[must_use]
    pub const fn end_byte(&self) -> usize {
        self.end.byte_offset()
    }

    /// Return the half-open UTF-8 byte range.
    ///
    /// The range is relative to the source from which this span originated.
    #[must_use]
    pub fn byte_range(&self) -> Range<usize> {
        self.start_byte()..self.end_byte()
    }

    /// Return the number of UTF-8 bytes covered by the span.
    #[must_use]
    pub const fn byte_len(&self) -> usize {
        self.end_byte() - self.start_byte()
    }

    /// Return the starting one-based line number.
    #[must_use]
    pub const fn start_line(&self) -> usize {
        self.start.line()
    }

    /// Return the ending one-based line number.
    #[must_use]
    pub const fn end_line(&self) -> usize {
        self.end.line()
    }

    /// Return the starting one-based Unicode-scalar column.
    #[must_use]
    pub const fn start_column(&self) -> usize {
        self.start.column()
    }

    /// Return the ending one-based Unicode-scalar column.
    #[must_use]
    pub const fn end_column(&self) -> usize {
        self.end.column()
    }

    /// Return whether this span has zero byte length.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.start_byte() == self.end_byte()
    }

    /// Return whether this span has a positive byte length.
    #[must_use]
    pub const fn is_non_empty(&self) -> bool {
        !self.is_empty()
    }

    /// Return whether a byte offset is within the span.
    ///
    /// The end boundary is excluded. For a zero-width span, no byte offset
    /// is contained.
    #[must_use]
    pub const fn contains_byte(&self, offset: usize) -> bool {
        self.start_byte() <= offset && offset < self.end_byte()
    }

    /// Return whether this span fully contains another span.
    ///
    /// Source identity is not represented here, so callers must ensure
    /// both spans belong to the same source.
    #[must_use]
    pub const fn contains_span(&self, other: &Self) -> bool {
        self.start_byte() <= other.start_byte()
            && other.end_byte() <= self.end_byte()
    }

    /// Return whether the byte ranges overlap.
    ///
    /// Adjacent ranges do not overlap. Empty spans do not overlap any range.
    #[must_use]
    pub const fn overlaps(&self, other: &Self) -> bool {
        self.start_byte() < other.end_byte()
            && other.start_byte() < self.end_byte()
    }

    /// Return whether this span ends exactly where another span begins.
    #[must_use]
    pub const fn is_adjacent_to(&self, other: &Self) -> bool {
        self.end_byte() == other.start_byte()
            || other.end_byte() == self.start_byte()
    }

    /// Create a subspan using absolute source positions.
    ///
    /// Both endpoints must lie within this span. This method checks byte
    /// range containment and position ordering; it cannot verify UTF-8
    /// boundaries without access to the source.
    pub fn subspan(
        &self,
        start: CursorPosition,
        end: CursorPosition,
    ) -> Result<Self, SpanError> {
        let child = Self::try_new(start, end)?;

        if !self.contains_span(&child) {
            return Err(SpanError::OutsideParent {
                parent_start: self.start_byte(),
                parent_end: self.end_byte(),
                child_start: child.start_byte(),
                child_end: child.end_byte(),
            });
        }

        Ok(child)
    }

    /// Join two spans that are ordered and non-overlapping.
    ///
    /// The spans must belong to the same source. That condition cannot be
    /// verified because source identity is intentionally not stored here.
    ///
    /// Returns a span from the first span's start to the second span's end.
    pub fn join(self, other: Self) -> Result<Self, SpanError> {
        if self.end_byte() > other.start_byte() {
            return Err(SpanError::ReversedByteRange {
                start: self.end_byte(),
                end: other.start_byte(),
            });
        }

        Self::try_new(self.start, other.end)
    }

    /// Return the smallest byte range enclosing two spans.
    ///
    /// The spans must refer to the same source. The method uses the earlier
    /// start and later end positions; it does not require the ranges to
    /// overlap or be adjacent.
    ///
    /// This operation is useful for diagnostics covering multiple source
    /// constructs. It must not be used to imply that intervening text is
    /// part of a particular token.
    pub fn cover(self, other: Self) -> Result<Self, SpanError> {
        let (start, end) = if self.start_byte() <= other.start_byte() {
            let end = if self.end_byte() >= other.end_byte() {
                self.end
            } else {
                other.end
            };
            (self.start, end)
        } else {
            let end = if self.end_byte() >= other.end_byte() {
                self.end
            } else {
                other.end
            };
            (other.start, end)
        };

        Self::try_new(start, end)
    }
}

impl Default for Span {
    /// The default span is the zero-width span at the beginning of a source.
    fn default() -> Self {
        Self::at(CursorPosition::default())
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}:{}..{}:{} (bytes {}..{})",
            self.start_line(),
            self.start_column(),
            self.end_line(),
            self.end_column(),
            self.start_byte(),
            self.end_byte()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{Span, SpanError};
    use crate::frontend::lexer::cursor::{Cursor, CursorPosition};

    fn position(byte: usize, line: usize, column: usize) -> CursorPosition {
        CursorPosition::new(byte, line, column)
    }

    #[test]
    fn constructs_a_valid_half_open_span() {
        let span = Span::new(position(2, 1, 3), position(7, 1, 8));

        assert_eq!(span.start_byte(), 2);
        assert_eq!(span.end_byte(), 7);
        assert_eq!(span.byte_len(), 5);
        assert_eq!(span.byte_range(), 2..7);
        assert!(!span.is_empty());
    }

    #[test]
    fn permits_zero_width_spans() {
        let position = position(4, 1, 5);
        let span = Span::at(position);

        assert!(span.is_empty());
        assert_eq!(span.byte_len(), 0);
        assert_eq!(span.start(), position);
        assert_eq!(span.end(), position);
    }

    #[test]
    fn rejects_reversed_byte_ranges() {
        let result = Span::try_new(position(5, 1, 6), position(4, 1, 5));

        assert!(matches!(
            result,
            Err(SpanError::ReversedByteRange {
                start: 5,
                end: 4
            })
        ));
    }

    #[test]
    fn rejects_reversed_positions() {
        let result = Span::try_new(position(3, 2, 1), position(4, 1, 2));

        assert!(matches!(
            result,
            Err(SpanError::ReversedPositionRange)
        ));
    }

    #[test]
    fn byte_containment_obeys_half_open_semantics() {
        let span = Span::new(position(3, 1, 4), position(6, 1, 7));

        assert!(!span.contains_byte(2));
        assert!(span.contains_byte(3));
        assert!(span.contains_byte(5));
        assert!(!span.contains_byte(6));
    }

    #[test]
    fn span_containment_includes_zero_width_boundary_spans() {
        let parent = Span::new(position(2, 1, 3), position(8, 1, 9));
        let child = Span::at(position(8, 1, 9));

        assert!(parent.contains_span(&child));
    }

    #[test]
    fn detects_overlap_without_treating_adjacent_ranges_as_overlapping() {
        let first = Span::new(position(0, 1, 1), position(4, 1, 5));
        let overlapping = Span::new(position(3, 1, 4), position(6, 1, 7));
        let adjacent = Span::new(position(4, 1, 5), position(8, 1, 9));

        assert!(first.overlaps(&overlapping));
        assert!(!first.overlaps(&adjacent));
        assert!(first.is_adjacent_to(&adjacent));
    }

    #[test]
    fn creates_a_contained_subspan() {
        let parent = Span::new(position(2, 1, 3), position(10, 1, 11));
        let child = parent
            .subspan(position(4, 1, 5), position(7, 1, 8))
            .expect("subspan should be contained");

        assert_eq!(child.byte_range(), 4..7);
    }

    #[test]
    fn rejects_a_subspan_outside_the_parent() {
        let parent = Span::new(position(2, 1, 3), position(5, 1, 6));
        let result = parent.subspan(position(4, 1, 5), position(7, 1, 8));

        assert!(matches!(result, Err(SpanError::OutsideParent { .. })));
    }

    #[test]
    fn joins_ordered_spans() {
        let first = Span::new(position(0, 1, 1), position(3, 1, 4));
        let second = Span::new(position(3, 1, 4), position(8, 1, 9));

        let joined = first.join(second).expect("spans are ordered");

        assert_eq!(joined.byte_range(), 0..8);
    }

    #[test]
    fn rejects_out_of_order_join() {
        let first = Span::new(position(4, 1, 5), position(8, 1, 9));
        let second = Span::new(position(0, 1, 1), position(3, 1, 4));

        assert!(first.join(second).is_err());
    }

    #[test]
    fn covers_overlapping_spans() {
        let first = Span::new(position(2, 1, 3), position(6, 1, 7));
        let second = Span::new(position(4, 1, 5), position(9, 1, 10));

        let cover = first.cover(second).expect("valid enclosing span");

        assert_eq!(cover.byte_range(), 2..9);
    }

    #[test]
    fn default_is_the_start_of_a_source() {
        let span = Span::default();

        assert_eq!(span.start(), CursorPosition::default());
        assert_eq!(span.end(), CursorPosition::default());
        assert!(span.is_empty());
    }

    #[test]
    fn cursor_positions_produce_consistent_utf8_byte_spans() {
        let source = "aλ界";
        let mut cursor = Cursor::new(source);

        let start = cursor.position();
        cursor.consume();
        cursor.consume();
        let end = cursor.position();

        let span = Span::new(start, end);

        assert_eq!(span.byte_range(), 0..3);
        assert_eq!(&source[span.byte_range()], "aλ");
    }
}