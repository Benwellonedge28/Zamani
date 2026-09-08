//! Canonical source span infrastructure for the native Zamani frontend AST.
//!
//! # Architectural role
//!
//! `Span` is the source-location primitive used by the native Zamani AST.
//! It records *where* a source-level construct came from without knowing
//! anything about:
//!
//! - semantic analysis;
//! - types;
//! - quantum hardware;
//! - qubit counts;
//! - topology;
//! - routing;
//! - scheduling;
//! - calibration;
//! - QEC;
//! - runtime state;
//! - compiler backends;
//! - QIR;
//! - LLVM;
//! - MLIR;
//! - operating-system resources.
//!
//! The intended compilation boundary is:
//!
//! ```text
//! source text
//!     │
//!     ▼
//! lexer / parser
//!     │
//!     ▼
//! native Zamani AST
//!     │
//!     ├── NodeId
//!     ├── NodeKind
//!     ├── Span  ← this module
//!     └── metadata
//!     │
//!     ▼
//! structural validation
//!     │
//!     ▼
//! semantic analysis
//!     │
//!     ▼
//! semantic model
//!     │
//!     ▼
//! ZUIR
//!     │
//!     ▼
//! domain / target / hardware lowering
//! ```
//!
//! # Coordinate contract
//!
//! The canonical span coordinate is a zero-based UTF-8 **byte offset**.
//!
//! A span is half-open:
//!
//! ```text
//! [start, end)
//! ```
//!
//! Therefore:
//!
//! - `start <= end` is required;
//! - `start == end` represents an empty/point span;
//! - `len = end - start`;
//! - adjacent spans `[a,b)` and `[b,c)` do not overlap;
//! - slicing must only occur at valid UTF-8 boundaries;
//! - EOF is representable;
//! - an empty source has the valid span `[0,0)`.
//!
//! This module deliberately does not store line/column information.
//! Line/column information is derived from source text by the source-map layer.
//! This prevents duplicated and potentially inconsistent coordinate systems.
//!
//! # Scalability
//!
//! `Span` does not encode:
//!
//! - maximum program size;
//! - maximum number of AST nodes;
//! - maximum number of qubits;
//! - maximum number of registers;
//! - machine size;
//! - hardware size.
//!
//! Coordinates use `u64`, rather than a machine-dependent `usize`, so the
//! serialized AST coordinate model remains stable across 32-bit and 64-bit
//! compilation hosts.
//!
//! This does not claim that a Rust process can literally allocate infinite
//! memory. "Infinity" in the POCO-REAF architecture means that the source
//! representation introduces no artificial machine-size limit beyond the
//! representable coordinate space and resources actually available.
//!
//! Operational compiler limits belong in configurable frontend/compiler policy,
//! not in this type.
//!
//! # Safety
//!
//! This module contains no `unsafe` code.
//!
//! All externally supplied coordinate arithmetic uses checked operations.
//! No string slicing operation is performed by this type, because UTF-8
//! boundary validation requires access to the corresponding source text.
//!
//! # Integration contract
//!
//! This file is intentionally independent.
//!
//! It may depend only on the Rust standard library and Serde.
//!
//! It is consumed by:
//!
//! - `node.rs`;
//! - AST nodes;
//! - source maps;
//! - diagnostics;
//! - parser;
//! - structural validation;
//! - visitors/traversal;
//! - AST serialization;
//! - semantic provenance;
//! - lowering provenance.
//!
//! It must never depend on:
//!
//! - semantic analysis;
//! - ZUIR;
//! - quantum IR;
//! - quantum hardware;
//! - optimization;
//! - routing;
//! - scheduling;
//! - runtime;
//! - backend implementations.
//!
//! # Rust compatibility
//!
//! Designed for Rust 1.97 / Rust 1.97.1, edition 2021.
//!
//! No nightly features are used.

use serde::{Deserialize, Serialize};
use std::cmp::{max, min};
use std::fmt;
use std::ops::Range;

/// Stable source-file identity.
///
/// `SourceId` identifies the source unit from which a span originated.
/// It does not itself prove that the source is registered in a source map.
///
/// Source-map ownership and validation belong to the source infrastructure.
///
/// `u64` is deliberately used so the serialized coordinate model is not
/// coupled to the width of the compiler host's `usize`.
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    Deserialize,
)]
#[repr(transparent)]
pub struct SourceId(u64);

impl SourceId {
    /// Creates a source identifier from its stable raw representation.
    ///
    /// This constructor does not verify that the identifier exists in a
    /// particular source map.
    #[must_use]
    pub const fn from_raw(raw: u64) -> Self {
        Self(raw)
    }

    /// Returns the stable raw identifier.
    #[must_use]
    pub const fn as_raw(self) -> u64 {
        self.0
    }

    /// Returns whether this is the zero/default source identifier.
    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.0 == 0
    }
}

impl From<u64> for SourceId {
    fn from(value: u64) -> Self {
        Self::from_raw(value)
    }
}

impl From<SourceId> for u64 {
    fn from(value: SourceId) -> Self {
        value.as_raw()
    }
}

impl fmt::Display for SourceId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "source#{}", self.0)
    }
}

/// Zero-based UTF-8 byte offset.
///
/// This is the canonical coordinate used by [`Span`].
///
/// A `SourceOffset` is an offset, not a byte index that has already been
/// proven to be a UTF-8 scalar boundary. Boundary validation requires source
/// text and therefore belongs to the source-map/string layer.
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    Deserialize,
)]
#[repr(transparent)]
pub struct SourceOffset(u64);

impl SourceOffset {
    /// Creates an offset from its raw representation.
    ///
    /// This is intentionally infallible because the representation itself is
    /// already the complete `u64` coordinate domain.
    #[must_use]
    pub const fn from_raw(raw: u64) -> Self {
        Self(raw)
    }

    /// Converts a platform-sized offset into the canonical coordinate type.
    ///
    /// This is checked so callers on a platform with a wider `usize` cannot
    /// silently truncate an offset.
    #[must_use]
    pub fn try_from_usize(value: usize) -> Result<Self, SpanError> {
        u64::try_from(value)
            .map(Self)
            .map_err(|_| SpanError::OffsetOverflow)
    }

    /// Returns the raw offset.
    #[must_use]
    pub const fn as_raw(self) -> u64 {
        self.0
    }

    /// Returns the offset as `usize` when representable.
    #[must_use]
    pub fn try_as_usize(self) -> Result<usize, SpanError> {
        usize::try_from(self.0).map_err(|_| SpanError::OffsetOverflow)
    }

    /// Returns whether this offset is zero.
    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.0 == 0
    }

    /// Adds an offset using checked arithmetic.
    #[must_use]
    pub const fn checked_add(self, amount: u64) -> Option<Self> {
        match self.0.checked_add(amount) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }

    /// Subtracts an offset using checked arithmetic.
    #[must_use]
    pub const fn checked_sub(self, amount: u64) -> Option<Self> {
        match self.0.checked_sub(amount) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }

    /// Calculates `end - self` when `end >= self`.
    #[must_use]
    pub const fn checked_distance(self, end: Self) -> Option<u64> {
        end.0.checked_sub(self.0)
    }
}

impl From<u64> for SourceOffset {
    fn from(value: u64) -> Self {
        Self::from_raw(value)
    }
}

impl From<SourceOffset> for u64 {
    fn from(value: SourceOffset) -> Self {
        value.as_raw()
    }
}

impl fmt::Display for SourceOffset {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

/// Errors produced when constructing or manipulating source spans.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum SpanError {
    /// The end coordinate precedes the start coordinate.
    ReversedRange {
        /// Start coordinate supplied by the caller.
        start: SourceOffset,

        /// End coordinate supplied by the caller.
        end: SourceOffset,
    },

    /// A platform-sized value could not be represented by the canonical
    /// coordinate type.
    OffsetOverflow,

    /// An arithmetic operation would overflow the canonical coordinate space.
    ArithmeticOverflow,

    /// A source range does not fit the requested platform representation.
    RangeConversionOverflow,
}

impl fmt::Display for SpanError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ReversedRange { start, end } => {
                write!(
                    formatter,
                    "source span has reversed range: start={} end={}",
                    start, end
                )
            }
            Self::OffsetOverflow => {
                write!(formatter, "source offset cannot be represented")
            }
            Self::ArithmeticOverflow => {
                write!(formatter, "source span coordinate arithmetic overflowed")
            }
            Self::RangeConversionOverflow => {
                write!(
                    formatter,
                    "source span cannot be converted to the requested range representation"
                )
            }
        }
    }
}

impl std::error::Error for SpanError {}

/// A canonical source range belonging to a source unit.
///
/// Spans use the half-open interval:
///
/// ```text
/// [start, end)
/// ```
///
/// # Invariants
///
/// A `Span` constructed through [`Span::new`] always satisfies:
///
/// ```text
/// start <= end
/// ```
///
/// The type itself does not verify that the offsets are within the length of
/// the corresponding source file. That validation requires the source map and
/// belongs there.
///
/// # Empty spans
///
/// `start == end` is valid and represents an empty source location.
///
/// This is useful for:
///
/// - EOF diagnostics;
/// - insertion points;
/// - parser recovery;
/// - generated nodes;
/// - zero-width syntax constructs.
///
/// # Identity
///
/// A span is a location, not an identity. Two distinct AST nodes may
/// legitimately have equal spans.
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    Deserialize,
)]
pub struct Span {
    source: SourceId,
    start: SourceOffset,
    end: SourceOffset,
}

impl Span {
    /// Creates a validated half-open source span.
    ///
    /// # Errors
    ///
    /// Returns [`SpanError::ReversedRange`] when `end < start`.
    #[must_use]
    pub const fn new(
        source: SourceId,
        start: SourceOffset,
        end: SourceOffset,
    ) -> Result<Self, SpanError> {
        if start.as_raw() > end.as_raw() {
            return Err(SpanError::ReversedRange { start, end });
        }

        Ok(Self {
            source,
            start,
            end,
        })
    }

    /// Creates a span from `usize` byte offsets.
    ///
    /// This is the preferred parser convenience constructor when lexer/parser
    /// positions are represented by `usize`.
    #[must_use]
    pub fn from_usize(
        source: SourceId,
        start: usize,
        end: usize,
    ) -> Result<Self, SpanError> {
        let start = SourceOffset::try_from_usize(start)?;
        let end = SourceOffset::try_from_usize(end)?;

        Self::new(source, start, end)
    }

    /// Creates an empty span at the supplied source offset.
    #[must_use]
    pub const fn point(source: SourceId, offset: SourceOffset) -> Self {
        Self {
            source,
            start: offset,
            end: offset,
        }
    }

    /// Creates an empty span at a `usize` byte offset.
    #[must_use]
    pub fn point_usize(
        source: SourceId,
        offset: usize,
    ) -> Result<Self, SpanError> {
        let offset = SourceOffset::try_from_usize(offset)?;
        Ok(Self::point(source, offset))
    }

    /// Returns the source identity.
    #[must_use]
    pub const fn source(self) -> SourceId {
        self.source
    }

    /// Returns the starting byte offset.
    #[must_use]
    pub const fn start(self) -> SourceOffset {
        self.start
    }

    /// Returns the ending byte offset.
    #[must_use]
    pub const fn end(self) -> SourceOffset {
        self.end
    }

    /// Returns the starting byte offset as `u64`.
    #[must_use]
    pub const fn start_raw(self) -> u64 {
        self.start.as_raw()
    }

    /// Returns the ending byte offset as `u64`.
    #[must_use]
    pub const fn end_raw(self) -> u64 {
        self.end.as_raw()
    }

    /// Returns whether the span is empty.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.start.as_raw() == self.end.as_raw()
    }

    /// Returns whether the span contains at least one byte.
    #[must_use]
    pub const fn is_non_empty(self) -> bool {
        !self.is_empty()
    }

    /// Returns the number of bytes represented by the span.
    ///
    /// Because construction guarantees `start <= end`, this operation cannot
    /// underflow.
    #[must_use]
    pub const fn len(self) -> u64 {
        self.end.as_raw() - self.start.as_raw()
    }

    /// Returns whether `offset` lies inside this span.
    ///
    /// Membership follows the half-open interval convention:
    ///
    /// ```text
    /// start <= offset < end
    /// ```
    ///
    /// Consequently, `end` is not contained.
    #[must_use]
    pub const fn contains_offset(self, offset: SourceOffset) -> bool {
        self.start.as_raw() <= offset.as_raw()
            && offset.as_raw() < self.end.as_raw()
    }

    /// Returns whether this span contains another span completely.
    ///
    /// Empty spans are treated according to the half-open coordinate model.
    #[must_use]
    pub const fn contains(self, other: Self) -> bool {
        self.source == other.source
            && self.start.as_raw() <= other.start.as_raw()
            && other.end.as_raw() <= self.end.as_raw()
    }

    /// Returns whether this span strictly contains another span.
    ///
    /// Equal spans are not considered strict containment.
    #[must_use]
    pub const fn strictly_contains(self, other: Self) -> bool {
        self.source == other.source
            && self.start.as_raw() < other.start.as_raw()
            && other.end.as_raw() < self.end.as_raw()
    }

    /// Returns whether two spans overlap with non-zero extent.
    ///
    /// Spans from different sources never overlap.
    ///
    /// Adjacent ranges such as `[0,4)` and `[4,8)` do not overlap.
    #[must_use]
    pub const fn overlaps(self, other: Self) -> bool {
        self.source == other.source
            && self.start.as_raw() < other.end.as_raw()
            && other.start.as_raw() < self.end.as_raw()
    }

    /// Returns whether two spans are adjacent.
    ///
    /// Examples:
    ///
    /// ```text
    /// [0,4) + [4,8) => adjacent
    /// ```
    #[must_use]
    pub const fn is_adjacent(self, other: Self) -> bool {
        self.source == other.source
            && (self.end.as_raw() == other.start.as_raw()
                || other.end.as_raw() == self.start.as_raw())
    }

    /// Returns the intersection of two non-empty overlapping spans.
    ///
    /// Returns `None` when the spans belong to different sources or do not
    /// overlap.
    #[must_use]
    pub fn intersection(self, other: Self) -> Option<Self> {
        if !self.overlaps(other) {
            return None;
        }

        let start = max(self.start, other.start);
        let end = min(self.end, other.end);

        // Both spans are valid and overlap, so this construction cannot fail.
        Some(Self {
            source: self.source,
            start,
            end,
        })
    }

    /// Returns the smallest span containing both spans.
    ///
    /// Returns `None` for spans belonging to different source units.
    ///
    /// Empty spans are supported.
    #[must_use]
    pub fn covering(self, other: Self) -> Option<Self> {
        if self.source != other.source {
            return None;
        }

        Some(Self {
            source: self.source,
            start: min(self.start, other.start),
            end: max(self.end, other.end),
        })
    }

    /// Returns the smallest span covering all supplied spans.
    ///
    /// This operation is allocation-free and does not require the spans to be
    /// sorted.
    ///
    /// Returns `None` for an empty iterator or when spans belong to different
    /// source units.
    #[must_use]
    pub fn covering_all<I>(spans: I) -> Option<Self>
    where
        I: IntoIterator<Item = Self>,
    {
        let mut iter = spans.into_iter();
        let first = iter.next()?;

        let mut result = first;

        for span in iter {
            result = result.covering(span)?;
        }

        Some(result)
    }

    /// Returns the relative byte range of this span.
    ///
    /// This method is intentionally fallible because `usize` is platform
    /// dependent while the canonical span coordinates are `u64`.
    #[must_use]
    pub fn as_range_usize(self) -> Result<Range<usize>, SpanError> {
        let start = self.start.try_as_usize()?;
        let end = self.end.try_as_usize()?;

        Ok(start..end)
    }

    /// Returns the canonical byte range as `u64`.
    #[must_use]
    pub const fn as_range_u64(self) -> Range<u64> {
        self.start.as_raw()..self.end.as_raw()
    }

    /// Returns a span with the same source and a new end offset.
    ///
    /// The new end must not precede the existing start.
    #[must_use]
    pub const fn with_end(
        self,
        end: SourceOffset,
    ) -> Result<Self, SpanError> {
        Self::new(self.source, self.start, end)
    }

    /// Returns a span with the same source and a new start offset.
    ///
    /// The new start must not exceed the existing end.
    #[must_use]
    pub const fn with_start(
        self,
        start: SourceOffset,
    ) -> Result<Self, SpanError> {
        Self::new(self.source, start, self.end)
    }

    /// Returns a span shifted by a signed byte delta.
    ///
    /// This operation is checked and never wraps.
    #[must_use]
    pub fn checked_shift(self, delta: i64) -> Result<Self, SpanError> {
        let start = shift_offset(self.start, delta)?;
        let end = shift_offset(self.end, delta)?;

        Self::new(self.source, start, end)
    }

    /// Returns a span expanded on the left by `amount` bytes.
    ///
    /// The operation is checked and never underflows.
    #[must_use]
    pub fn checked_expand_left(
        self,
        amount: u64,
    ) -> Result<Self, SpanError> {
        let start = self
            .start
            .checked_sub(amount)
            .ok_or(SpanError::ArithmeticOverflow)?;

        Self::new(self.source, start, self.end)
    }

    /// Returns a span expanded on the right by `amount` bytes.
    ///
    /// The operation is checked and never wraps.
    #[must_use]
    pub fn checked_expand_right(
        self,
        amount: u64,
    ) -> Result<Self, SpanError> {
        let end = self
            .end
            .checked_add(amount)
            .ok_or(SpanError::ArithmeticOverflow)?;

        Self::new(self.source, self.start, end)
    }

    /// Returns a zero-width span at the beginning of this span.
    #[must_use]
    pub const fn start_point(self) -> Self {
        Self::point(self.source, self.start)
    }

    /// Returns a zero-width span at the end of this span.
    #[must_use]
    pub const fn end_point(self) -> Self {
        Self::point(self.source, self.end)
    }

    /// Returns a compact diagnostic tuple.
    ///
    /// This avoids forcing diagnostic code to depend on the private field
    /// layout.
    #[must_use]
    pub const fn diagnostic_coordinates(
        self,
    ) -> (SourceId, SourceOffset, SourceOffset) {
        (self.source, self.start, self.end)
    }
}

impl From<Span> for Range<u64> {
    fn from(span: Span) -> Self {
        span.as_range_u64()
    }
}

impl fmt::Display for Span {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}:[{}..{})",
            self.source,
            self.start.as_raw(),
            self.end.as_raw()
        )
    }
}

/// Compatibility alias for frontend code that historically calls the
/// canonical source range a `SourceSpan`.
///
/// `Span` remains the preferred native AST name.
///
/// This alias intentionally does not create a second representation.
pub type SourceSpan = Span;

/// Trait for AST/source structures that expose a source span.
///
/// This trait is deliberately tiny and source-oriented. It does not expose
/// semantic or backend state.
pub trait Spanned {
    /// Returns the source span associated with this value.
    fn span(&self) -> Span;
}

impl Spanned for Span {
    #[inline]
    fn span(&self) -> Span {
        *self
    }
}

/// Trait for structures whose span can be changed by controlled AST
/// transformation infrastructure.
///
/// This is intentionally separate from [`Spanned`] so read-only compiler
/// consumers do not need mutable access.
pub trait SpannedMut: Spanned {
    /// Replaces the current span.
    fn set_span(&mut self, span: Span);
}

impl SpannedMut for Span {
    #[inline]
    fn set_span(&mut self, span: Span) {
        *self = span;
    }
}

/// Applies a checked signed displacement to a source offset.
fn shift_offset(
    offset: SourceOffset,
    delta: i64,
) -> Result<SourceOffset, SpanError> {
    if delta >= 0 {
        let amount = u64::try_from(delta)
            .map_err(|_| SpanError::ArithmeticOverflow)?;

        offset
            .checked_add(amount)
            .ok_or(SpanError::ArithmeticOverflow)
    } else {
        let amount = delta.unsigned_abs();

        offset
            .checked_sub(amount)
            .ok_or(SpanError::ArithmeticOverflow)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn source() -> SourceId {
        SourceId::from_raw(7)
    }

    fn offset(value: u64) -> SourceOffset {
        SourceOffset::from_raw(value)
    }

    #[test]
    fn constructs_valid_half_open_span() {
        let span = Span::new(source(), offset(2), offset(8))
            .expect("valid span");

        assert_eq!(span.source(), source());
        assert_eq!(span.start(), offset(2));
        assert_eq!(span.end(), offset(8));
        assert_eq!(span.len(), 6);
        assert!(!span.is_empty());
    }

    #[test]
    fn accepts_empty_span() {
        let span = Span::point(source(), offset(5));

        assert!(span.is_empty());
        assert_eq!(span.len(), 0);
        assert_eq!(span.start(), offset(5));
        assert_eq!(span.end(), offset(5));
    }

    #[test]
    fn rejects_reversed_range() {
        let error = Span::new(source(), offset(9), offset(4))
            .expect_err("reversed range must fail");

        assert_eq!(
            error,
            SpanError::ReversedRange {
                start: offset(9),
                end: offset(4),
            }
        );
    }

    #[test]
    fn contains_offset_uses_half_open_semantics() {
        let span =
            Span::new(source(), offset(2), offset(8)).expect("valid span");

        assert!(!span.contains_offset(offset(1)));
        assert!(span.contains_offset(offset(2)));
        assert!(span.contains_offset(offset(7)));
        assert!(!span.contains_offset(offset(8)));
    }

    #[test]
    fn contains_span_uses_half_open_semantics() {
        let outer =
            Span::new(source(), offset(2), offset(10)).expect("valid span");

        let inner =
            Span::new(source(), offset(4), offset(8)).expect("valid span");

        assert!(outer.contains(inner));
        assert!(outer.strictly_contains(inner));
        assert!(outer.contains(outer));
        assert!(!outer.strictly_contains(outer));
    }

    #[test]
    fn overlapping_spans_are_detected() {
        let left =
            Span::new(source(), offset(2), offset(8)).expect("valid span");

        let right =
            Span::new(source(), offset(6), offset(12)).expect("valid span");

        assert!(left.overlaps(right));
        assert_eq!(
            left.intersection(right),
            Some(
                Span::new(source(), offset(6), offset(8))
                    .expect("valid span")
            )
        );
    }

    #[test]
    fn adjacent_spans_do_not_overlap() {
        let left =
            Span::new(source(), offset(0), offset(4)).expect("valid span");

        let right =
            Span::new(source(), offset(4), offset(8)).expect("valid span");

        assert!(!left.overlaps(right));
        assert!(left.is_adjacent(right));
    }

    #[test]
    fn different_sources_never_overlap() {
        let first =
            Span::new(source(), offset(0), offset(10)).expect("valid span");

        let second_source = SourceId::from_raw(8);

        let second =
            Span::new(second_source, offset(5), offset(7))
                .expect("valid span");

        assert!(!first.overlaps(second));
        assert!(first.intersection(second).is_none());
        assert!(first.covering(second).is_none());
    }

    #[test]
    fn covering_spans_produces_smallest_common_span() {
        let first =
            Span::new(source(), offset(2), offset(5)).expect("valid span");

        let second =
            Span::new(source(), offset(8), offset(13)).expect("valid span");

        let covered = first.covering(second).expect("same source");

        assert_eq!(covered.start(), offset(2));
        assert_eq!(covered.end(), offset(13));
    }

    #[test]
    fn covering_all_is_order_independent() {
        let spans = [
            Span::new(source(), offset(20), offset(25)).expect("valid span"),
            Span::new(source(), offset(2), offset(5)).expect("valid span"),
            Span::new(source(), offset(10), offset(15)).expect("valid span"),
        ];

        let covered = Span::covering_all(spans)
            .expect("non-empty span collection");

        assert_eq!(covered.start(), offset(2));
        assert_eq!(covered.end(), offset(25));
    }

    #[test]
    fn covering_all_rejects_mixed_sources() {
        let spans = [
            Span::new(source(), offset(0), offset(2)).expect("valid span"),
            Span::new(SourceId::from_raw(8), offset(2), offset(4))
                .expect("valid span"),
        ];

        assert!(Span::covering_all(spans).is_none());
    }

    #[test]
    fn usize_constructor_is_checked() {
        let span = Span::from_usize(source(), 10, 20)
            .expect("normal usize values must fit");

        assert_eq!(span.start_raw(), 10);
        assert_eq!(span.end_raw(), 20);
    }

    #[test]
    fn range_conversion_is_checked() {
        let span =
            Span::new(source(), offset(10), offset(20)).expect("valid span");

        assert_eq!(
            span.as_range_usize().expect("range fits"),
            10..20
        );

        assert_eq!(span.as_range_u64(), 10..20);
    }

    #[test]
    fn checked_shift_supports_positive_delta() {
        let span =
            Span::new(source(), offset(10), offset(20)).expect("valid span");

        let shifted =
            span.checked_shift(5).expect("positive shift fits");

        assert_eq!(shifted.start_raw(), 15);
        assert_eq!(shifted.end_raw(), 25);
    }

    #[test]
    fn checked_shift_supports_negative_delta() {
        let span =
            Span::new(source(), offset(10), offset(20)).expect("valid span");

        let shifted =
            span.checked_shift(-5).expect("negative shift fits");

        assert_eq!(shifted.start_raw(), 5);
        assert_eq!(shifted.end_raw(), 15);
    }

    #[test]
    fn checked_shift_rejects_underflow() {
        let span =
            Span::new(source(), offset(2), offset(5)).expect("valid span");

        assert_eq!(
            span.checked_shift(-3),
            Err(SpanError::ArithmeticOverflow)
        );
    }

    #[test]
    fn checked_expand_left_is_safe() {
        let span =
            Span::new(source(), offset(10), offset(20)).expect("valid span");

        let expanded =
            span.checked_expand_left(5).expect("expansion fits");

        assert_eq!(expanded.start_raw(), 5);
        assert_eq!(expanded.end_raw(), 20);
    }

    #[test]
    fn checked_expand_left_rejects_underflow() {
        let span =
            Span::new(source(), offset(2), offset(5)).expect("valid span");

        assert_eq!(
            span.checked_expand_left(3),
            Err(SpanError::ArithmeticOverflow)
        );
    }

    #[test]
    fn checked_expand_right_is_safe() {
        let span =
            Span::new(source(), offset(10), offset(20)).expect("valid span");

        let expanded =
            span.checked_expand_right(5).expect("expansion fits");

        assert_eq!(expanded.start_raw(), 10);
        assert_eq!(expanded.end_raw(), 25);
    }

    #[test]
    fn checked_expand_right_rejects_overflow() {
        let span = Span::new(
            source(),
            offset(u64::MAX - 1),
            offset(u64::MAX),
        )
        .expect("valid span");

        assert_eq!(
            span.checked_expand_right(1),
            Err(SpanError::ArithmeticOverflow)
        );
    }

    #[test]
    fn start_and_end_points_are_empty() {
        let span =
            Span::new(source(), offset(10), offset(20)).expect("valid span");

        let start = span.start_point();
        let end = span.end_point();

        assert!(start.is_empty());
        assert!(end.is_empty());
        assert_eq!(start.start_raw(), 10);
        assert_eq!(end.start_raw(), 20);
    }

    #[test]
    fn with_start_preserves_validity() {
        let span =
            Span::new(source(), offset(10), offset(20)).expect("valid span");

        let changed =
            span.with_start(offset(12)).expect("valid replacement");

        assert_eq!(changed.start_raw(), 12);
        assert_eq!(changed.end_raw(), 20);
    }

    #[test]
    fn with_end_rejects_reversal() {
        let span =
            Span::new(source(), offset(10), offset(20)).expect("valid span");

        assert_eq!(
            span.with_end(offset(9)),
            Err(SpanError::ReversedRange {
                start: offset(10),
                end: offset(9),
            })
        );
    }

    #[test]
    fn display_is_stable_and_machine_independent() {
        let span =
            Span::new(source(), offset(10), offset(20)).expect("valid span");

        assert_eq!(span.to_string(), "source#7:[10..20)");
    }

    #[test]
    fn serde_round_trip_preserves_span() {
        let span =
            Span::new(source(), offset(10), offset(20)).expect("valid span");

        let encoded =
            serde_json::to_string(&span).expect("span must serialize");

        let decoded: Span =
            serde_json::from_str(&encoded).expect("span must deserialize");

        assert_eq!(decoded, span);
    }

    #[test]
    fn source_id_round_trip_is_stable() {
        let source = SourceId::from_raw(u64::MAX);

        let encoded =
            serde_json::to_string(&source).expect("source id serializes");

        let decoded: SourceId =
            serde_json::from_str(&encoded).expect("source id deserializes");

        assert_eq!(decoded, source);
    }

    #[test]
    fn source_offset_checked_distance_is_safe() {
        let start = SourceOffset::from_raw(10);
        let end = SourceOffset::from_raw(25);

        assert_eq!(start.checked_distance(end), Some(15));
        assert_eq!(end.checked_distance(start), None);
    }

    #[test]
    fn source_offset_checked_add_rejects_overflow() {
        let offset = SourceOffset::from_raw(u64::MAX);

        assert_eq!(offset.checked_add(1), None);
    }

    #[test]
    fn source_offset_checked_sub_rejects_underflow() {
        let offset = SourceOffset::from_raw(0);

        assert_eq!(offset.checked_sub(1), None);
    }
}