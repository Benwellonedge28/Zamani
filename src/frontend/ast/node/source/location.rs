//! Canonical human-readable source-location infrastructure for the Zamani
//! frontend AST.
//!
//! # Architectural role
//!
//! This module provides [`SourceLocation`], a compact source position that
//! combines the canonical source identity and byte offset with derived
//! human-readable line/column coordinates.
//!
//! The canonical coordinate in the Zamani AST remains the UTF-8 byte offset
//! represented by [`SourceOffset`] and [`Span`]. Line and column information
//! is derived information intended for:
//!
//! - diagnostics;
//! - IDE/LSP tooling;
//! - parser error reporting;
//! - source navigation;
//! - debugging;
//! - human-readable compiler output.
//!
//! This module does not redefine the canonical source coordinate system.
//!
//! # Coordinate model
//!
//! Zamani source locations use:
//!
//! - zero-based source identifiers;
//! - zero-based UTF-8 byte offsets;
//! - zero-based line indices;
//! - zero-based UTF-8 byte columns.
//!
//! A location therefore represents:
//!
//! ```text
//! source#N : line L, byte-column C, byte-offset O
//! ```
//!
//! The canonical identity of a location is:
//!
//! ```text
//! (SourceId, SourceOffset)
//! ```
//!
//! `line` and `column` are derived presentation/indexing information.
//!
//! # Why byte columns?
//!
//! Zamani source is UTF-8. A byte offset is the canonical coordinate because:
//!
//! - lexer/parser infrastructure commonly operates on byte offsets;
//! - spans can represent source ranges without repeatedly converting between
//!   coordinate systems;
//! - byte offsets are deterministic;
//! - serialization remains compact;
//! - the representation is independent of terminal/editor conventions;
//! - source slicing can be validated against UTF-8 boundaries by the source
//!   infrastructure.
//!
//! Unicode scalar-value columns can be calculated by source infrastructure when
//! required by a particular consumer. UTF-16 columns required by LSP are also
//! deliberately not stored here because they are a consumer-specific coordinate
//! system.
//!
//! # Important separation
//!
//! `SourceLocation` is not:
//!
//! - an AST node ID;
//! - a semantic symbol ID;
//! - a type ID;
//! - a quantum resource ID;
//! - a qubit ID;
//! - a hardware location;
//! - a physical qubit coordinate;
//! - a backend location;
//! - an instruction address;
//! - an LLVM location;
//! - a QIR location;
//! - an MLIR location;
//! - a runtime location.
//!
//! It describes only where source text occurs.
//!
//! # POCO-REAF / scalability
//!
//! No machine size, qubit count, register count, topology, vendor, backend,
//! processor architecture, or execution resource is encoded here.
//!
//! The representation therefore remains applicable to:
//!
//! - tiny source files;
//! - very large source files;
//! - classical programs;
//! - quantum programs;
//! - hybrid programs;
//! - HDL extensions;
//! - accelerator programs;
//! - distributed programs;
//! - future computational domains.
//!
//! "Infinity" in the POCO-REAF requirement is interpreted as absence of an
//! artificial language-level size restriction. Actual compilation remains
//! bounded by available address space, memory, CPU time, and configured
//! compiler resource policies.
//!
//! # Dependency contract
//!
//! This module may depend only on:
//!
//! - [`super::span::SourceId`];
//! - [`super::span::SourceOffset`];
//! - Rust standard-library facilities;
//! - Serde for serialization.
//!
//! It must never depend on:
//!
//! - semantic analysis;
//! - ZUIR;
//! - quantum IR;
//! - quantum hardware;
//! - routing;
//! - scheduling;
//! - calibration;
//! - QEC;
//! - resilience;
//! - runtime;
//! - backend implementations.
//!
//! # Integration
//!
//! ```text
//! source text
//!     │
//!     ▼
//! SourceMap / parser
//!     │
//!     ├── SourceId
//!     ├── SourceOffset
//!     └── SourceLocation  ← this module
//!             │
//!             ▼
//!           Span
//!             │
//!             ▼
//!          AST Node
//!             │
//!             ▼
//!        diagnostics / tooling
//! ```
//!
//! The source-map implementation should be the authoritative component for
//! converting arbitrary byte offsets into locations when repeated conversions
//! are required. `SourceLocation::from_source` is provided as a foundational,
//! allocation-free correctness implementation and is appropriate for isolated
//! conversions and tests.
//!
//! # Rust compatibility
//!
//! Designed for Rust 1.97 / Rust 1.97.1, edition 2021.
//!
//! No nightly features are used.
//!
//! # Safety
//!
//! This module contains no `unsafe` code.

use serde::{Deserialize, Serialize};
use std::fmt;

use super::span::{SourceId, SourceOffset};

/// Zero-based source line index.
///
/// Line indices are deliberately represented independently from byte offsets.
/// A source map can derive them efficiently from indexed line-start data.
///
/// There is no language-level maximum number of lines.
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
pub struct LineIndex(u64);

impl LineIndex {
    /// Creates a line index from its raw representation.
    #[must_use]
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }

    /// Returns the zero-based raw line index.
    #[must_use]
    pub const fn as_raw(self) -> u64 {
        self.0
    }

    /// Returns the one-based line number used by conventional diagnostics.
    ///
    /// Returns `None` only when the conversion would overflow `u64`, which can
    /// occur only for the theoretically terminal representable index.
    #[must_use]
    pub const fn one_based(self) -> Option<u64> {
        self.0.checked_add(1)
    }

    /// Returns the next line index when representable.
    #[must_use]
    pub const fn checked_next(self) -> Option<Self> {
        match self.0.checked_add(1) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }

    /// Converts this line index to `usize` when representable.
    #[must_use]
    pub fn try_as_usize(self) -> Result<usize, LocationError> {
        usize::try_from(self.0).map_err(|_| LocationError::CoordinateOverflow)
    }
}

impl From<u64> for LineIndex {
    fn from(value: u64) -> Self {
        Self::from_raw(value)
    }
}

impl From<LineIndex> for u64 {
    fn from(value: LineIndex) -> Self {
        value.as_raw()
    }
}

impl fmt::Display for LineIndex {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.one_based() {
            Some(line) => line.fmt(formatter),
            None => write!(formatter, "overflow"),
        }
    }
}

/// Zero-based UTF-8 byte column within a source line.
///
/// This is intentionally a byte column, not a Unicode scalar-value column or
/// UTF-16 code-unit column. Those coordinate systems belong to consumers such
/// as editors and LSP implementations.
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
pub struct ByteColumn(u64);

impl ByteColumn {
    /// Creates a byte column from its raw representation.
    #[must_use]
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }

    /// Returns the raw zero-based byte column.
    #[must_use]
    pub const fn as_raw(self) -> u64 {
        self.0
    }

    /// Returns the one-based display column when representable.
    #[must_use]
    pub const fn one_based(self) -> Option<u64> {
        self.0.checked_add(1)
    }

    /// Advances the column by a number of bytes using checked arithmetic.
    #[must_use]
    pub const fn checked_add(self, amount: u64) -> Option<Self> {
        match self.0.checked_add(amount) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }

    /// Converts this column to `usize` when representable.
    #[must_use]
    pub fn try_as_usize(self) -> Result<usize, LocationError> {
        usize::try_from(self.0).map_err(|_| LocationError::CoordinateOverflow)
    }
}

impl From<u64> for ByteColumn {
    fn from(value: u64) -> Self {
        Self::from_raw(value)
    }
}

impl From<ByteColumn> for u64 {
    fn from(value: ByteColumn) -> Self {
        value.as_raw()
    }
}

impl fmt::Display for ByteColumn {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.one_based() {
            Some(column) => column.fmt(formatter),
            None => write!(formatter, "overflow"),
        }
    }
}

/// Errors produced by source-location construction or coordinate conversion.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum LocationError {
    /// A platform-sized coordinate could not be represented by the canonical
    /// `u64` coordinate model.
    CoordinateOverflow,

    /// The supplied byte offset does not fit within the supplied source text.
    OffsetOutOfBounds {
        /// The requested offset.
        offset: SourceOffset,

        /// Length of the source in bytes.
        source_length: u64,
    },

    /// The supplied byte offset is not a UTF-8 character boundary.
    ///
    /// Source locations are allowed only at valid UTF-8 boundaries when they
    /// are derived from source text.
    InvalidUtf8Boundary {
        /// The invalid byte offset.
        offset: SourceOffset,
    },

    /// The supplied derived line/column values do not match the supplied
    /// source text and offset.
    InconsistentCoordinates,

    /// A source location belongs to a different source file than another
    /// location with which it was combined.
    DifferentSources {
        /// First source identity.
        left: SourceId,

        /// Second source identity.
        right: SourceId,
    },

    /// A coordinate operation would overflow the canonical `u64` domain.
    ArithmeticOverflow,
}

impl fmt::Display for LocationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CoordinateOverflow => {
                write!(formatter, "source location coordinate overflow")
            }

            Self::OffsetOutOfBounds {
                offset,
                source_length,
            } => {
                write!(
                    formatter,
                    "source offset {} is outside source length {}",
                    offset, source_length
                )
            }

            Self::InvalidUtf8Boundary { offset } => {
                write!(
                    formatter,
                    "source offset {} is not a valid UTF-8 boundary",
                    offset
                )
            }

            Self::InconsistentCoordinates => {
                write!(
                    formatter,
                    "source location line/column coordinates are inconsistent with the source offset"
                )
            }

            Self::DifferentSources { left, right } => {
                write!(
                    formatter,
                    "cannot combine locations from different sources: {} and {}",
                    left, right
                )
            }

            Self::ArithmeticOverflow => {
                write!(
                    formatter,
                    "source location coordinate arithmetic overflowed"
                )
            }
        }
    }
}

impl std::error::Error for LocationError {}

/// A source position with both canonical and derived coordinates.
///
/// # Canonical identity
///
/// The authoritative position is:
///
/// ```text
/// (source, offset)
/// ```
///
/// `line` and `column` are derived coordinates retained because repeatedly
/// calculating them can otherwise become unnecessarily expensive for
/// diagnostics-heavy compiler workloads.
///
/// # Coordinate convention
///
/// All stored coordinates are zero-based.
///
/// For example, the first byte of a file is:
///
/// ```text
/// line   = 0
/// column = 0
/// offset = 0
/// ```
///
/// A conventional human-readable diagnostic can use [`Self::display_line`]
/// and [`Self::display_column`] to obtain one-based coordinates.
///
/// # UTF-8
///
/// `column` is a byte column. Construction through [`Self::from_source`]
/// validates that the offset lies on a UTF-8 boundary.
///
/// Construction through [`Self::new`] is intended for trusted source-map
/// infrastructure where boundary validation may already have been performed.
///
/// # Storage
///
/// The representation is intentionally fixed-width and contains no references
/// to source strings. This allows locations to remain cheap to clone and safe
/// to store in AST nodes, diagnostics, side tables, and serialized artifacts.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    Deserialize,
)]
pub struct SourceLocation {
    source: SourceId,
    offset: SourceOffset,
    line: LineIndex,
    column: ByteColumn,
}

impl SourceLocation {
    /// Creates a source location from canonical and derived coordinates.
    ///
    /// This constructor does not inspect source text. It is therefore suitable
    /// for source-map implementations that have already validated their
    /// coordinates.
    ///
    /// For externally supplied source offsets where UTF-8 validity has not yet
    /// been established, prefer [`Self::from_source`].
    #[must_use]
    pub const fn new(
        source: SourceId,
        offset: SourceOffset,
        line: LineIndex,
        column: ByteColumn,
    ) -> Self {
        Self {
            source,
            offset,
            line,
            column,
        }
    }

    /// Creates the first location of a source.
    ///
    /// This is:
    ///
    /// ```text
    /// source#N, line 0, column 0, offset 0
    /// ```
    #[must_use]
    pub const fn start(source: SourceId) -> Self {
        Self {
            source,
            offset: SourceOffset::from_raw(0),
            line: LineIndex::from_raw(0),
            column: ByteColumn::from_raw(0),
        }
    }

    /// Creates a location at the end of a source string.
    ///
    /// The returned location is the position immediately after the final byte.
    ///
    /// For a source ending with a newline, this therefore points to the first
    /// column of the line following that newline.
    #[must_use]
    pub fn end_of_source(source: SourceId, text: &str) -> Result<Self, LocationError> {
        let offset = SourceOffset::try_from_usize(text.len())
            .map_err(|_| LocationError::CoordinateOverflow)?;

        Self::from_source(source, offset, text)
    }

    /// Creates a source location from a UTF-8 source string and byte offset.
    ///
    /// This method validates:
    ///
    /// 1. the offset is within the source;
    /// 2. the offset is a valid UTF-8 boundary;
    /// 3. line and byte-column coordinates are derived deterministically.
    ///
    /// The source itself is borrowed only during construction. No source text
    /// is retained in the resulting location.
    ///
    /// # Complexity
    ///
    /// This foundational implementation scans from the beginning of the source
    /// to the requested offset and is therefore `O(offset)`.
    ///
    /// A production [`SourceMap`](crate::frontend::...) should normally maintain
    /// indexed line starts and use [`Self::new`] after validating the coordinate
    /// so repeated diagnostic conversions can be `O(log n)` or better.
    pub fn from_source(
        source: SourceId,
        offset: SourceOffset,
        text: &str,
    ) -> Result<Self, LocationError> {
        let raw_offset = offset.as_raw();
        let source_length = u64::try_from(text.len())
            .map_err(|_| LocationError::CoordinateOverflow)?;

        if raw_offset > source_length {
            return Err(LocationError::OffsetOutOfBounds {
                offset,
                source_length,
            });
        }

        let byte_offset = offset
            .try_as_usize()
            .map_err(|_| LocationError::CoordinateOverflow)?;

        if !text.is_char_boundary(byte_offset) {
            return Err(LocationError::InvalidUtf8Boundary { offset });
        }

        let prefix = &text[..byte_offset];

        let mut line = 0_u64;
        let mut column = 0_u64;

        for byte in prefix.bytes() {
            if byte == b'\n' {
                line = line
                    .checked_add(1)
                    .ok_or(LocationError::ArithmeticOverflow)?;

                column = 0;
            } else {
                column = column
                    .checked_add(1)
                    .ok_or(LocationError::ArithmeticOverflow)?;
            }
        }

        Ok(Self::new(
            source,
            offset,
            LineIndex::from_raw(line),
            ByteColumn::from_raw(column),
        ))
    }

    /// Returns the source identity.
    #[must_use]
    pub const fn source(self) -> SourceId {
        self.source
    }

    /// Returns the canonical byte offset.
    #[must_use]
    pub const fn offset(self) -> SourceOffset {
        self.offset
    }

    /// Returns the raw canonical byte offset.
    #[must_use]
    pub const fn offset_raw(self) -> u64 {
        self.offset.as_raw()
    }

    /// Returns the zero-based line index.
    #[must_use]
    pub const fn line(self) -> LineIndex {
        self.line
    }

    /// Returns the zero-based raw line index.
    #[must_use]
    pub const fn line_raw(self) -> u64 {
        self.line.as_raw()
    }

    /// Returns the zero-based byte column.
    #[must_use]
    pub const fn column(self) -> ByteColumn {
        self.column
    }

    /// Returns the zero-based raw byte column.
    #[must_use]
    pub const fn column_raw(self) -> u64 {
        self.column.as_raw()
    }

    /// Returns the conventional one-based line number.
    ///
    /// This is intended for human-readable diagnostics.
    #[must_use]
    pub const fn display_line(self) -> Option<u64> {
        self.line.one_based()
    }

    /// Returns the conventional one-based byte column.
    ///
    /// This is intended for human-readable diagnostics.
    #[must_use]
    pub const fn display_column(self) -> Option<u64> {
        self.column.one_based()
    }

    /// Returns whether this location is at the beginning of the source.
    #[must_use]
    pub const fn is_source_start(self) -> bool {
        self.offset.as_raw() == 0
    }

    /// Returns whether this location is on the first source line.
    #[must_use]
    pub const fn is_first_line(self) -> bool {
        self.line.as_raw() == 0
    }

    /// Returns whether this location is at column zero.
    #[must_use]
    pub const fn is_line_start(self) -> bool {
        self.column.as_raw() == 0
    }

    /// Returns a location advanced by a number of bytes on the same line.
    ///
    /// This method is useful only when the caller knows that `amount` does not
    /// cross a newline. It performs no source-text inspection.
    ///
    /// For arbitrary source movement use a source-map operation instead.
    #[must_use]
    pub fn checked_advance_column(
        self,
        amount: u64,
    ) -> Result<Self, LocationError> {
        let offset = self
            .offset
            .checked_add(amount)
            .ok_or(LocationError::ArithmeticOverflow)?;

        let column = self
            .column
            .checked_add(amount)
            .ok_or(LocationError::ArithmeticOverflow)?;

        Ok(Self::new(self.source, offset, self.line, column))
    }

    /// Returns a location with a different derived line/column while preserving
    /// the canonical source and byte offset.
    ///
    /// This is intended for source-map implementations that calculate
    /// coordinates using indexed line information.
    ///
    /// The method does not inspect source text.
    #[must_use]
    pub const fn with_coordinates(
        self,
        line: LineIndex,
        column: ByteColumn,
    ) -> Self {
        Self::new(self.source, self.offset, line, column)
    }

    /// Validates this location against the supplied source text.
    ///
    /// This is useful at trust boundaries such as:
    ///
    /// - deserialization;
    /// - source-map loading;
    /// - compiler-server requests;
    /// - plugin-provided source locations.
    pub fn validate_against(
        self,
        text: &str,
    ) -> Result<(), LocationError> {
        let expected = Self::from_source(self.source, self.offset, text)?;

        if expected.line != self.line || expected.column != self.column {
            return Err(LocationError::InconsistentCoordinates);
        }

        Ok(())
    }

    /// Returns the UTF-8 byte slice corresponding to this location's byte
    /// offset when the supplied source is compatible.
    ///
    /// This method returns the suffix beginning at the location. It does not
    /// allocate.
    ///
    /// The method exists primarily for source-map and diagnostic infrastructure
    /// and intentionally performs boundary validation.
    pub fn suffix<'source>(
        self,
        source: &'source str,
    ) -> Result<&'source str, LocationError> {
        let offset = self
            .offset
            .try_as_usize()
            .map_err(|_| LocationError::CoordinateOverflow)?;

        if offset > source.len() {
            return Err(LocationError::OffsetOutOfBounds {
                offset: self.offset,
                source_length: u64::try_from(source.len())
                    .map_err(|_| LocationError::CoordinateOverflow)?,
            });
        }

        if !source.is_char_boundary(offset) {
            return Err(LocationError::InvalidUtf8Boundary {
                offset: self.offset,
            });
        }

        Ok(&source[offset..])
    }

    /// Calculates the byte distance to another location in the same source.
    ///
    /// Returns `None` if the other location precedes this location.
    ///
    /// Returns [`LocationError::DifferentSources`] when the locations refer to
    /// different source units.
    pub fn checked_distance_to(
        self,
        other: Self,
    ) -> Result<Option<u64>, LocationError> {
        if self.source != other.source {
            return Err(LocationError::DifferentSources {
                left: self.source,
                right: other.source,
            });
        }

        Ok(other.offset.as_raw().checked_sub(self.offset.as_raw()))
    }

    /// Returns the minimum of two locations from the same source.
    ///
    /// # Errors
    ///
    /// Returns [`LocationError::DifferentSources`] if the locations belong to
    /// different source units.
    pub fn try_min(self, other: Self) -> Result<Self, LocationError> {
        if self.source != other.source {
            return Err(LocationError::DifferentSources {
                left: self.source,
                right: other.source,
            });
        }

        Ok(if self.offset <= other.offset {
            self
        } else {
            other
        })
    }

    /// Returns the maximum of two locations from the same source.
    ///
    /// # Errors
    ///
    /// Returns [`LocationError::DifferentSources`] if the locations belong to
    /// different source units.
    pub fn try_max(self, other: Self) -> Result<Self, LocationError> {
        if self.source != other.source {
            return Err(LocationError::DifferentSources {
                left: self.source,
                right: other.source,
            });
        }

        Ok(if self.offset >= other.offset {
            self
        } else {
            other
        })
    }
}

impl Default for SourceLocation {
    fn default() -> Self {
        Self::start(SourceId::from_raw(0))
    }
}

impl fmt::Display for SourceLocation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let line = self.display_line().unwrap_or(u64::MAX);
        let column = self.display_column().unwrap_or(u64::MAX);

        write!(
            formatter,
            "{}:{}:{}",
            self.source,
            line,
            column
        )
    }
}

/// A source-location range described by two [`SourceLocation`] values.
///
/// This is intentionally separate from [`super::span::Span`].
///
/// `Span` remains the canonical compact source range used by AST nodes.
/// `SourceLocationRange` is a convenience representation for diagnostics and
/// tooling when line/column information for both endpoints is already known.
///
/// It must never become a second competing source-range abstraction.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    PartialEq,
    Serialize,
    Deserialize,
)]
pub struct SourceLocationRange {
    start: SourceLocation,
    end: SourceLocation,
}

impl SourceLocationRange {
    /// Creates a validated source-location range.
    ///
    /// Both locations must belong to the same source and `start` must not
    /// follow `end`.
    pub fn new(
        start: SourceLocation,
        end: SourceLocation,
    ) -> Result<Self, LocationError> {
        if start.source != end.source {
            return Err(LocationError::DifferentSources {
                left: start.source,
                right: end.source,
            });
        }

        if start.offset > end.offset {
            return Err(LocationError::InconsistentCoordinates);
        }

        Ok(Self { start, end })
    }

    /// Creates a point range containing one source location.
    #[must_use]
    pub const fn point(location: SourceLocation) -> Self {
        Self {
            start: location,
            end: location,
        }
    }

    /// Returns the starting location.
    #[must_use]
    pub const fn start(self) -> SourceLocation {
        self.start
    }

    /// Returns the ending location.
    #[must_use]
    pub const fn end(self) -> SourceLocation {
        self.end
    }

    /// Returns the source identity.
    #[must_use]
    pub const fn source(self) -> SourceId {
        self.start.source()
    }

    /// Returns whether the range is empty.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.start.offset() == self.end.offset()
    }

    /// Returns the byte length represented by the range.
    #[must_use]
    pub const fn byte_len(self) -> u64 {
        self.end.offset_raw() - self.start.offset_raw()
    }

    /// Returns whether the range contains a canonical source offset.
    ///
    /// Membership follows the half-open convention:
    ///
    /// ```text
    /// [start, end)
    /// ```
    #[must_use]
    pub const fn contains_offset(self, offset: SourceOffset) -> bool {
        self.start.offset_raw() <= offset.as_raw()
            && offset.as_raw() < self.end.offset_raw()
    }

    /// Returns whether this range completely contains another range.
    #[must_use]
    pub const fn contains(self, other: Self) -> bool {
        self.source() == other.source()
            && self.start.offset_raw() <= other.start.offset_raw()
            && other.end.offset_raw() <= self.end.offset_raw()
    }

    /// Returns whether this range overlaps another range.
    ///
    /// Empty ranges do not overlap non-empty ranges.
    #[must_use]
    pub const fn overlaps(self, other: Self) -> bool {
        if self.source() != other.source() {
            return false;
        }

        self.start.offset_raw() < other.end.offset_raw()
            && other.start.offset_raw() < self.end.offset_raw()
    }

    /// Returns a range spanning both input ranges.
    ///
    /// The locations must belong to the same source.
    pub fn union(self, other: Self) -> Result<Self, LocationError> {
        if self.source() != other.source() {
            return Err(LocationError::DifferentSources {
                left: self.source(),
                right: other.source(),
            });
        }

        let start = self.start.try_min(other.start)?;
        let end = self.end.try_max(other.end)?;

        Self::new(start, end)
    }
}

impl fmt::Display for SourceLocationRange {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.start == self.end {
            return self.start.fmt(formatter);
        }

        write!(formatter, "{}..{}", self.start, self.end)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn source() -> SourceId {
        SourceId::from_raw(7)
    }

    #[test]
    fn start_is_zero_based() {
        let location = SourceLocation::start(source());

        assert_eq!(location.source(), source());
        assert_eq!(location.offset_raw(), 0);
        assert_eq!(location.line_raw(), 0);
        assert_eq!(location.column_raw(), 0);
        assert_eq!(location.display_line(), Some(1));
        assert_eq!(location.display_column(), Some(1));
        assert!(location.is_source_start());
        assert!(location.is_first_line());
        assert!(location.is_line_start());
    }

    #[test]
    fn first_line_ascii_coordinates_are_correct() {
        let text = "abc";

        let location =
            SourceLocation::from_source(source(), SourceOffset::from_raw(2), text)
                .expect("valid source location");

        assert_eq!(location.offset_raw(), 2);
        assert_eq!(location.line_raw(), 0);
        assert_eq!(location.column_raw(), 2);
        assert_eq!(location.display_line(), Some(1));
        assert_eq!(location.display_column(), Some(3));
    }

    #[test]
    fn newline_resets_column() {
        let text = "abc\ndef";

        let location =
            SourceLocation::from_source(source(), SourceOffset::from_raw(4), text)
                .expect("valid source location");

        assert_eq!(location.line_raw(), 1);
        assert_eq!(location.column_raw(), 0);
        assert_eq!(location.display_line(), Some(2));
        assert_eq!(location.display_column(), Some(1));
    }

    #[test]
    fn newline_coordinates_continue_correctly() {
        let text = "abc\ndef";

        let location =
            SourceLocation::from_source(source(), SourceOffset::from_raw(6), text)
                .expect("valid source location");

        assert_eq!(location.line_raw(), 1);
        assert_eq!(location.column_raw(), 2);
    }

    #[test]
    fn eof_is_valid() {
        let text = "abc\ndef";

        let location =
            SourceLocation::end_of_source(source(), text)
                .expect("EOF must be representable");

        assert_eq!(location.offset_raw(), 7);
        assert_eq!(location.line_raw(), 1);
        assert_eq!(location.column_raw(), 3);
    }

    #[test]
    fn empty_source_has_valid_eof() {
        let text = "";

        let location =
            SourceLocation::end_of_source(source(), text)
                .expect("empty source must have an EOF location");

        assert_eq!(location.offset_raw(), 0);
        assert_eq!(location.line_raw(), 0);
        assert_eq!(location.column_raw(), 0);
    }

    #[test]
    fn unicode_uses_utf8_byte_columns() {
        let text = "aéz";

        // `a` = 1 byte.
        // `é` = 2 bytes.
        // Therefore the byte offset immediately before `z` is 3.
        let location =
            SourceLocation::from_source(source(), SourceOffset::from_raw(3), text)
                .expect("valid UTF-8 boundary");

        assert_eq!(location.line_raw(), 0);
        assert_eq!(location.column_raw(), 3);
    }

    #[test]
    fn unicode_non_boundary_is_rejected() {
        let text = "aéz";

        // Offset 2 is in the middle of the two-byte UTF-8 encoding of `é`.
        let result =
            SourceLocation::from_source(source(), SourceOffset::from_raw(2), text);

        assert!(matches!(
            result,
            Err(LocationError::InvalidUtf8Boundary { .. })
        ));
    }

    #[test]
    fn out_of_bounds_is_rejected() {
        let text = "abc";

        let result =
            SourceLocation::from_source(source(), SourceOffset::from_raw(4), text);

        assert!(matches!(
            result,
            Err(LocationError::OffsetOutOfBounds { .. })
        ));
    }

    #[test]
    fn validation_accepts_consistent_location() {
        let text = "one\ntwo";

        let location =
            SourceLocation::from_source(source(), SourceOffset::from_raw(5), text)
                .expect("valid source location");

        assert!(location.validate_against(text).is_ok());
    }

    #[test]
    fn validation_rejects_inconsistent_coordinates() {
        let text = "one\ntwo";

        let location = SourceLocation::new(
            source(),
            SourceOffset::from_raw(5),
            LineIndex::from_raw(0),
            ByteColumn::from_raw(5),
        );

        assert!(matches!(
            location.validate_against(text),
            Err(LocationError::InconsistentCoordinates)
        ));
    }

    #[test]
    fn suffix_is_non_allocating_and_correct() {
        let text = "abc\ndef";

        let location =
            SourceLocation::from_source(source(), SourceOffset::from_raw(4), text)
                .expect("valid source location");

        let suffix = location.suffix(text).expect("valid suffix");

        assert_eq!(suffix, "def");
    }

    #[test]
    fn same_source_distance_is_checked() {
        let first =
            SourceLocation::new(
                source(),
                SourceOffset::from_raw(2),
                LineIndex::from_raw(0),
                ByteColumn::from_raw(2),
            );

        let second =
            SourceLocation::new(
                source(),
                SourceOffset::from_raw(8),
                LineIndex::from_raw(1),
                ByteColumn::from_raw(4),
            );

        assert_eq!(
            first
                .checked_distance_to(second)
                .expect("same source"),
            Some(6)
        );

        assert_eq!(
            second
                .checked_distance_to(first)
                .expect("same source"),
            None
        );
    }

    #[test]
    fn different_sources_cannot_be_compared_for_distance() {
        let first = SourceLocation::start(source());
        let second = SourceLocation::start(SourceId::from_raw(8));

        assert!(matches!(
            first.checked_distance_to(second),
            Err(LocationError::DifferentSources { .. })
        ));
    }

    #[test]
    fn range_is_half_open() {
        let start =
            SourceLocation::new(
                source(),
                SourceOffset::from_raw(2),
                LineIndex::from_raw(0),
                ByteColumn::from_raw(2),
            );

        let end =
            SourceLocation::new(
                source(),
                SourceOffset::from_raw(5),
                LineIndex::from_raw(0),
                ByteColumn::from_raw(5),
            );

        let range =
            SourceLocationRange::new(start, end)
                .expect("valid range");

        assert!(range.contains_offset(SourceOffset::from_raw(2)));
        assert!(range.contains_offset(SourceOffset::from_raw(4)));
        assert!(!range.contains_offset(SourceOffset::from_raw(5)));
        assert_eq!(range.byte_len(), 3);
    }

    #[test]
    fn empty_range_does_not_overlap_non_empty_range() {
        let point = SourceLocation::start(source());

        let empty = SourceLocationRange::point(point);

        let end =
            SourceLocation::new(
                source(),
                SourceOffset::from_raw(3),
                LineIndex::from_raw(0),
                ByteColumn::from_raw(3),
            );

        let range =
            SourceLocationRange::new(point, end)
                .expect("valid range");

        assert!(!empty.overlaps(range));
    }

    #[test]
    fn ranges_can_be_unioned() {
        let a_start =
            SourceLocation::new(
                source(),
                SourceOffset::from_raw(2),
                LineIndex::from_raw(0),
                ByteColumn::from_raw(2),
            );

        let a_end =
            SourceLocation::new(
                source(),
                SourceOffset::from_raw(5),
                LineIndex::from_raw(0),
                ByteColumn::from_raw(5),
            );

        let b_start =
            SourceLocation::new(
                source(),
                SourceOffset::from_raw(8),
                LineIndex::from_raw(1),
                ByteColumn::from_raw(1),
            );

        let b_end =
            SourceLocation::new(
                source(),
                SourceOffset::from_raw(12),
                LineIndex::from_raw(1),
                ByteColumn::from_raw(5),
            );

        let a =
            SourceLocationRange::new(a_start, a_end)
                .expect("valid range");

        let b =
            SourceLocationRange::new(b_start, b_end)
                .expect("valid range");

        let union = a.union(b).expect("same source");

        assert_eq!(union.start().offset_raw(), 2);
        assert_eq!(union.end().offset_raw(), 12);
    }

    #[test]
    fn serialization_round_trip_preserves_location() {
        let location =
            SourceLocation::new(
                source(),
                SourceOffset::from_raw(17),
                LineIndex::from_raw(3),
                ByteColumn::from_raw(4),
            );

        let encoded =
            serde_json::to_string(&location)
                .expect("serialization must succeed");

        let decoded: SourceLocation =
            serde_json::from_str(&encoded)
                .expect("deserialization must succeed");

        assert_eq!(decoded, location);
    }

    #[test]
    fn deterministic_display() {
        let location =
            SourceLocation::new(
                source(),
                SourceOffset::from_raw(17),
                LineIndex::from_raw(3),
                ByteColumn::from_raw(4),
            );

        assert_eq!(location.to_string(), "source#7:4:5");
    }
}