//! Source-file and source-location infrastructure for the quantum frontend.
//!
//! # Architectural boundary
//!
//! This module is deliberately format-independent.
//!
//! It provides the canonical source-location model used by:
//!
//! - frontend lexers;
//! - frontend parsers;
//! - semantic validators;
//! - diagnostics;
//! - importers;
//! - exporters when reporting source-origin information.
//!
//! It must not depend on:
//!
//! - OpenQASM;
//! - QIR;
//! - Quil;
//! - any other external format;
//! - `quantum::ir` semantic types;
//! - parser or lexer implementations;
//! - format-specific diagnostics.
//!
//! # Coordinate model
//!
//! Source offsets are always UTF-8 byte offsets.
//!
//! Line numbers are one-based.
//!
//! Columns are one-based Unicode scalar-value columns, not byte columns.
//! This means a multi-byte UTF-8 character occupies one column while its
//! underlying source offset may advance by several bytes.
//!
//! Example:
//!
//! ```text
//! abc
//! ^
//! column 1
//! ```
//!
//! # Span model
//!
//! [`SourceSpan`] uses a half-open range:
//!
//! ```text
//! start <= offset < end
//! ```
//!
//! Therefore:
//!
//! - an empty span has `start == end`;
//! - the length is `end - start`;
//! - adjacent spans can share a boundary without overlapping.
//!
//! # Safety
//!
//! Source text is untrusted input. This module therefore:
//!
//! - never assumes UTF-8 boundaries from callers;
//! - validates source ranges;
//! - avoids panics for ordinary invalid user input;
//! - uses checked arithmetic for externally supplied offsets;
//! - keeps source storage immutable after insertion;
//! - never performs filesystem or network I/O.
//!
//! The lexer/parser layer is responsible for enforcing maximum source size.
//! This module is responsible for maintaining correct source identity and
//! location information once a source has been accepted.

use std::fmt;
use std::sync::Arc;

/// Stable identifier for a source stored in a [`SourceMap`].
///
/// `SourceId` is intentionally opaque. Consumers must not depend on its
/// internal numeric representation beyond using it as an identifier.
///
/// IDs are assigned monotonically by [`SourceMap::add`].
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SourceId(u32);

impl SourceId {
    /// Creates a source identifier from its raw numeric representation.
    ///
    /// This is primarily intended for deserialization, testing, and
    /// integration with external source-reference systems.
    ///
    /// # Safety
    ///
    /// This constructor does not verify that the ID exists in a particular
    /// [`SourceMap`]. Callers must validate membership through the map.
    #[must_use]
    pub const fn from_raw(raw: u32) -> Self {
        Self(raw)
    }

    /// Returns the raw numeric representation.
    #[must_use]
    pub const fn as_raw(self) -> u32 {
        self.0
    }
}

impl fmt::Display for SourceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "source#{}", self.0)
    }
}

/// A zero-based byte offset into a source file.
///
/// Offsets are measured in bytes because Rust strings are UTF-8 byte
/// sequences. An offset is only suitable for slicing a source string when it
/// lies on a UTF-8 character boundary.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SourceOffset(u32);

impl SourceOffset {
    /// Creates an offset from a raw byte offset.
    #[must_use]
    pub const fn from_raw(raw: u32) -> Self {
        Self(raw)
    }

    /// Returns the underlying byte offset.
    #[must_use]
    pub const fn as_raw(self) -> u32 {
        self.0
    }

    /// Returns whether this offset is zero.
    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.0 == 0
    }

    /// Returns the offset as `usize`.
    #[must_use]
    pub const fn as_usize(self) -> usize {
        self.0 as usize
    }

    /// Adds a byte count, returning `None` on overflow.
    #[must_use]
    pub const fn checked_add(self, amount: u32) -> Option<Self> {
        match self.0.checked_add(amount) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }

    /// Returns the distance between two offsets when `end >= self`.
    #[must_use]
    pub const fn checked_distance(self, end: Self) -> Option<u32> {
        end.0.checked_sub(self.0)
    }
}

impl From<u32> for SourceOffset {
    fn from(value: u32) -> Self {
        Self::from_raw(value)
    }
}

impl From<SourceOffset> for u32 {
    fn from(value: SourceOffset) -> Self {
        value.as_raw()
    }
}

/// A one-based line number.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct LineNumber(u32);

impl LineNumber {
    /// First source line.
    pub const FIRST: Self = Self(1);

    /// Creates a line number from a raw one-based value.
    ///
    /// `0` is accepted as a raw value for interoperability, but normal
    /// [`SourcePosition`] values always use line numbers starting at one.
    #[must_use]
    pub const fn from_raw(raw: u32) -> Self {
        Self(raw)
    }

    /// Returns the raw line number.
    #[must_use]
    pub const fn as_raw(self) -> u32 {
        self.0
    }
}

impl From<u32> for LineNumber {
    fn from(value: u32) -> Self {
        Self::from_raw(value)
    }
}

impl From<LineNumber> for u32 {
    fn from(value: LineNumber) -> Self {
        value.as_raw()
    }
}

/// A one-based Unicode scalar-value column.
///
/// Columns are intended for human-readable diagnostics rather than byte
/// indexing. Internally, source offsets remain byte offsets.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ColumnNumber(u32);

impl ColumnNumber {
    /// First source column.
    pub const FIRST: Self = Self(1);

    /// Creates a column number from a raw one-based value.
    #[must_use]
    pub const fn from_raw(raw: u32) -> Self {
        Self(raw)
    }

    /// Returns the raw column number.
    #[must_use]
    pub const fn as_raw(self) -> u32 {
        self.0
    }
}

impl From<u32> for ColumnNumber {
    fn from(value: u32) -> Self {
        Self::from_raw(value)
    }
}

impl From<ColumnNumber> for u32 {
    fn from(value: ColumnNumber) -> Self {
        value.as_raw()
    }
}

/// A byte range within a particular source file.
///
/// The range is half-open:
///
/// ```text
/// start <= offset < end
/// ```
///
/// Empty spans are valid and are useful for diagnostics at insertion points,
/// such as an unexpected token at end-of-file.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SourceSpan {
    source_id: SourceId,
    start: SourceOffset,
    end: SourceOffset,
}

impl SourceSpan {
    /// Creates a span without consulting a [`SourceMap`].
    ///
    /// This constructor verifies only that `start <= end`.
    ///
    /// Use [`SourceSpan::new`] when a source map is available and the range
    /// must also be checked against the source length.
    pub const fn new(
        source_id: SourceId,
        start: SourceOffset,
        end: SourceOffset,
    ) -> Result<Self, SourceSpanError> {
        if start.as_raw() > end.as_raw() {
            return Err(SourceSpanError::ReversedRange);
        }

        Ok(Self {
            source_id,
            start,
            end,
        })
    }

    /// Creates an empty span at a particular source offset.
    #[must_use]
    pub const fn point(source_id: SourceId, offset: SourceOffset) -> Self {
        Self {
            source_id,
            start: offset,
            end: offset,
        }
    }

    /// Creates a span covering an entire source.
    #[must_use]
    pub fn entire(source: &SourceFile) -> Self {
        Self {
            source_id: source.id(),
            start: SourceOffset::from_raw(0),
            end: SourceOffset::from_raw(source.len_bytes()),
        }
    }

    /// Returns the source identifier.
    #[must_use]
    pub const fn source_id(self) -> SourceId {
        self.source_id
    }

    /// Returns the start offset.
    #[must_use]
    pub const fn start(self) -> SourceOffset {
        self.start
    }

    /// Returns the exclusive end offset.
    #[must_use]
    pub const fn end(self) -> SourceOffset {
        self.end
    }

    /// Returns the span length in bytes.
    #[must_use]
    pub const fn len_bytes(self) -> u32 {
        self.end.as_raw() - self.start.as_raw()
    }

    /// Returns whether the span is empty.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.start.as_raw() == self.end.as_raw()
    }

    /// Returns whether an offset lies inside this span.
    ///
    /// The end offset is exclusive.
    #[must_use]
    pub const fn contains(self, offset: SourceOffset) -> bool {
        offset.as_raw() >= self.start.as_raw()
            && offset.as_raw() < self.end.as_raw()
    }

    /// Returns whether this span contains the other span.
    #[must_use]
    pub const fn contains_span(self, other: Self) -> bool {
        self.source_id == other.source_id
            && other.start.as_raw() >= self.start.as_raw()
            && other.end.as_raw() <= self.end.as_raw()
    }

    /// Returns whether two spans overlap.
    #[must_use]
    pub const fn overlaps(self, other: Self) -> bool {
        self.source_id == other.source_id
            && self.start.as_raw() < other.end.as_raw()
            && other.start.as_raw() < self.end.as_raw()
    }

    /// Returns the smallest span containing both spans.
    ///
    /// Returns `None` if the spans belong to different sources.
    #[must_use]
    pub const fn union(self, other: Self) -> Option<Self> {
        if self.source_id != other.source_id {
            return None;
        }

        let start = if self.start.as_raw() < other.start.as_raw() {
            self.start
        } else {
            other.start
        };

        let end = if self.end.as_raw() > other.end.as_raw() {
            self.end
        } else {
            other.end
        };

        Some(Self {
            source_id: self.source_id,
            start,
            end,
        })
    }
}

impl fmt::Display for SourceSpan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}:{}..{}",
            self.source_id,
            self.start.as_raw(),
            self.end.as_raw()
        )
    }
}

/// Errors produced when constructing a source span.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SourceSpanError {
    /// The start offset is greater than the end offset.
    ReversedRange,

    /// A source offset does not fit in the internal representation.
    OffsetOverflow,
}

impl fmt::Display for SourceSpanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ReversedRange => {
                f.write_str("source span start offset exceeds end offset")
            }
            Self::OffsetOverflow => {
                f.write_str("source offset exceeds supported range")
            }
        }
    }
}

impl std::error::Error for SourceSpanError {}

/// A fully resolved source position.
///
/// The byte offset is the canonical coordinate. Line and column are derived
/// coordinates intended primarily for diagnostics and user interfaces.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SourcePosition {
    source_id: SourceId,
    offset: SourceOffset,
    line: LineNumber,
    column: ColumnNumber,
}

impl SourcePosition {
    /// Creates a source position.
    #[must_use]
    pub const fn new(
        source_id: SourceId,
        offset: SourceOffset,
        line: LineNumber,
        column: ColumnNumber,
    ) -> Self {
        Self {
            source_id,
            offset,
            line,
            column,
        }
    }

    /// Returns the source identifier.
    #[must_use]
    pub const fn source_id(self) -> SourceId {
        self.source_id
    }

    /// Returns the byte offset.
    #[must_use]
    pub const fn offset(self) -> SourceOffset {
        self.offset
    }

    /// Returns the one-based line number.
    #[must_use]
    pub const fn line(self) -> LineNumber {
        self.line
    }

    /// Returns the one-based column number.
    #[must_use]
    pub const fn column(self) -> ColumnNumber {
        self.column
    }
}

impl fmt::Display for SourcePosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}:{}:{}",
            self.source_id,
            self.line.as_raw(),
            self.column.as_raw()
        )
    }
}

/// Immutable source text together with its identity and display name.
///
/// Source contents are stored in an [`Arc<str>`] so callers can cheaply retain
/// source text while diagnostics, AST nodes, or other frontend structures are
/// alive without copying the complete source.
#[derive(Clone, Debug)]
pub struct SourceFile {
    id: SourceId,
    name: Arc<str>,
    text: Arc<str>,
    line_starts: Arc<[u32]>,
}

impl SourceFile {
    /// Constructs a source file.
    ///
    /// The source text must already be valid UTF-8, as guaranteed by Rust's
    /// `str` type.
    ///
    /// The line index is built once and subsequently reused for all location
    /// lookups.
    pub fn new(
        id: SourceId,
        name: impl Into<Arc<str>>,
        text: impl Into<Arc<str>>,
    ) -> Result<Self, SourceFileError> {
        let name = name.into();
        let text = text.into();

        let line_starts = build_line_index(&text)?;

        Ok(Self {
            id,
            name,
            text,
            line_starts: line_starts.into(),
        })
    }

    /// Returns the stable source ID.
    #[must_use]
    pub const fn id(&self) -> SourceId {
        self.id
    }

    /// Returns the source display name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the complete source text.
    #[must_use]
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Returns the source length in bytes.
    #[must_use]
    pub fn len_bytes(&self) -> u32 {
        self.text.len() as u32
    }

    /// Returns whether the source is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }

    /// Returns the number of logical source lines.
    ///
    /// An empty source has zero lines.
    ///
    /// A non-empty source always has at least one line.
    #[must_use]
    pub fn line_count(&self) -> u32 {
        if self.text.is_empty() {
            0
        } else {
            self.line_starts.len() as u32
        }
    }

    /// Returns the byte offset at which a line begins.
    ///
    /// Lines are one-based.
    #[must_use]
    pub fn line_start(&self, line: LineNumber) -> Option<SourceOffset> {
        if line.as_raw() == 0 {
            return None;
        }

        self.line_starts
            .get((line.as_raw() - 1) as usize)
            .copied()
            .map(SourceOffset::from_raw)
    }

    /// Resolves a byte offset into a line and Unicode-scalar column.
    ///
    /// Returns `None` when the offset is outside the source or is not a valid
    /// UTF-8 character boundary.
    #[must_use]
    pub fn position_at(&self, offset: SourceOffset) -> Option<SourcePosition> {
        let offset_usize = offset.as_usize();

        if offset_usize > self.text.len() || !self.text.is_char_boundary(offset_usize) {
            return None;
        }

        let line_index = match self
            .line_starts
            .binary_search_by(|start| start.cmp(&offset.as_raw()))
        {
            Ok(index) => index,
            Err(index) => index.saturating_sub(1),
        };

        let line_start = *self.line_starts.get(line_index)? as usize;

        let column = self.text[line_start..offset_usize]
            .chars()
            .count()
            .checked_add(1)?;

        let column = u32::try_from(column).ok()?;
        let line = u32::try_from(line_index.checked_add(1)?).ok()?;

        Some(SourcePosition::new(
            self.id,
            offset,
            LineNumber::from_raw(line),
            ColumnNumber::from_raw(column),
        ))
    }

    /// Resolves a span's starting position.
    #[must_use]
    pub fn start_position(&self, span: SourceSpan) -> Option<SourcePosition> {
        if span.source_id() != self.id {
            return None;
        }

        self.position_at(span.start())
    }

    /// Resolves a span's ending position.
    ///
    /// Because spans are half-open, this is the position immediately after
    /// the final byte represented by the span.
    #[must_use]
    pub fn end_position(&self, span: SourceSpan) -> Option<SourcePosition> {
        if span.source_id() != self.id {
            return None;
        }

        self.position_at(span.end())
    }

    /// Returns the source text covered by a span.
    ///
    /// Returns `None` if:
    ///
    /// - the span belongs to another source;
    /// - the offsets exceed the source length;
    /// - either offset is not a UTF-8 character boundary.
    #[must_use]
    pub fn slice(&self, span: SourceSpan) -> Option<&str> {
        if span.source_id() != self.id {
            return None;
        }

        let start = span.start().as_usize();
        let end = span.end().as_usize();

        if start > end
            || end > self.text.len()
            || !self.text.is_char_boundary(start)
            || !self.text.is_char_boundary(end)
        {
            return None;
        }

        self.text.get(start..end)
    }

    /// Returns the line containing the specified byte offset.
    ///
    /// The returned range includes the line terminator when one exists.
    #[must_use]
    pub fn line_span_at(&self, offset: SourceOffset) -> Option<SourceSpan> {
        let position = self.position_at(offset)?;

        let index = (position.line().as_raw() - 1) as usize;
        let start = *self.line_starts.get(index)?;

        let end = self
            .line_starts
            .get(index + 1)
            .copied()
            .unwrap_or_else(|| self.text.len() as u32);

        Some(SourceSpan {
            source_id: self.id,
            start: SourceOffset::from_raw(start),
            end: SourceOffset::from_raw(end),
        })
    }

    /// Returns the text of the line containing the specified offset.
    #[must_use]
    pub fn line_text_at(&self, offset: SourceOffset) -> Option<&str> {
        let span = self.line_span_at(offset)?;
        self.slice(span)
    }

    /// Returns all line-start byte offsets.
    ///
    /// The returned slice is immutable and may be retained cheaply.
    #[must_use]
    pub fn line_starts(&self) -> &[u32] {
        &self.line_starts
    }
}

/// Errors encountered while constructing a [`SourceFile`].
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SourceFileError {
    /// The source length cannot be represented by the frontend's compact
    /// 32-bit offset representation.
    SourceTooLarge,

    /// The source contains more lines than can be represented.
    TooManyLines,
}

impl fmt::Display for SourceFileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SourceTooLarge => {
                f.write_str("source file is too large for the frontend source model")
            }
            Self::TooManyLines => {
                f.write_str("source file contains too many lines for the frontend source model")
            }
        }
    }
}

impl std::error::Error for SourceFileError {}

/// Collection of immutable source files known to the frontend.
///
/// `SourceMap` is deliberately unaware of filesystems. A caller supplies the
/// source name and source contents; loading policy belongs to the caller or
/// to a future format-specific include resolver.
///
/// This makes the source infrastructure suitable for:
///
/// - ordinary files;
/// - in-memory editor buffers;
/// - embedded sources;
/// - generated sources;
/// - tests;
/// - sandboxed compilation environments.
#[derive(Clone, Debug, Default)]
pub struct SourceMap {
    files: Vec<SourceFile>,
}

impl SourceMap {
    /// Creates an empty source map.
    #[must_use]
    pub const fn new() -> Self {
        Self { files: Vec::new() }
    }

    /// Returns the number of registered sources.
    #[must_use]
    pub fn len(&self) -> usize {
        self.files.len()
    }

    /// Returns whether the source map contains no sources.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.files.is_empty()
    }

    /// Adds a source and returns its stable identifier.
    ///
    /// IDs are assigned monotonically.
    ///
    /// The source map owns the resulting immutable source object.
    pub fn add(
        &mut self,
        name: impl Into<Arc<str>>,
        text: impl Into<Arc<str>>,
    ) -> Result<SourceId, SourceFileError> {
        let raw_id =
            u32::try_from(self.files.len()).map_err(|_| SourceFileError::TooManyLines)?;

        let id = SourceId::from_raw(raw_id);
        let source = SourceFile::new(id, name, text)?;

        self.files.push(source);

        Ok(id)
    }

    /// Returns a source by ID.
    #[must_use]
    pub fn get(&self, id: SourceId) -> Option<&SourceFile> {
        self.files.get(id.as_raw() as usize)
    }

    /// Returns a source by ID, or a structured lookup error.
    pub fn require(&self, id: SourceId) -> Result<&SourceFile, SourceLookupError> {
        self.get(id).ok_or(SourceLookupError::UnknownSource(id))
    }

    /// Returns the source text for a span.
    #[must_use]
    pub fn slice(&self, span: SourceSpan) -> Option<&str> {
        self.get(span.source_id())?.slice(span)
    }

    /// Resolves a byte offset into a source position.
    #[must_use]
    pub fn position_at(&self, source_id: SourceId, offset: SourceOffset) -> Option<SourcePosition> {
        self.get(source_id)?.position_at(offset)
    }

    /// Returns the line containing a byte offset.
    #[must_use]
    pub fn line_span_at(
        &self,
        source_id: SourceId,
        offset: SourceOffset,
    ) -> Option<SourceSpan> {
        self.get(source_id)?.line_span_at(offset)
    }

    /// Returns an iterator over all registered source files.
    pub fn iter(&self) -> impl Iterator<Item = &SourceFile> {
        self.files.iter()
    }
}

/// Error returned when a source ID cannot be resolved.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SourceLookupError {
    /// The requested source does not exist in the map.
    UnknownSource(SourceId),
}

impl fmt::Display for SourceLookupError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownSource(id) => write!(f, "unknown source: {id}"),
        }
    }
}

impl std::error::Error for SourceLookupError {}

/// Builds the line-start index for a UTF-8 source.
///
/// Both LF and CRLF are treated as line terminators. A standalone CR is also
/// treated as a line terminator so that source diagnostics remain sensible
/// for legacy text inputs.
///
/// For CRLF, only the byte after `\n` is recorded as the next line start, so
/// the CRLF pair remains part of the preceding line's source span.
fn build_line_index(text: &str) -> Result<Vec<u32>, SourceFileError> {
    if text.len() > u32::MAX as usize {
        return Err(SourceFileError::SourceTooLarge);
    }

    if text.is_empty() {
        return Ok(Vec::new());
    }

    let mut starts = Vec::new();

    starts.push(0);

    let bytes = text.as_bytes();
    let mut index = 0usize;

    while index < bytes.len() {
        match bytes[index] {
            b'\n' => {
                let next = index
                    .checked_add(1)
                    .ok_or(SourceFileError::SourceTooLarge)?;

                let next = u32::try_from(next).map_err(|_| SourceFileError::SourceTooLarge)?;

                starts.push(next);
                index = next as usize;
            }
            b'\r' => {
                if index + 1 < bytes.len() && bytes[index + 1] == b'\n' {
                    let next = index
                        .checked_add(2)
                        .ok_or(SourceFileError::SourceTooLarge)?;

                    let next =
                        u32::try_from(next).map_err(|_| SourceFileError::SourceTooLarge)?;

                    starts.push(next);
                    index = next as usize;
                } else {
                    let next = index
                        .checked_add(1)
                        .ok_or(SourceFileError::SourceTooLarge)?;

                    let next =
                        u32::try_from(next).map_err(|_| SourceFileError::SourceTooLarge)?;

                    starts.push(next);
                    index = next as usize;
                }
            }
            _ => {
                index += 1;
            }
        }

        if starts.len() > u32::MAX as usize {
            return Err(SourceFileError::TooManyLines);
        }
    }

    Ok(starts)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn source() -> SourceFile {
        SourceFile::new(
            SourceId::from_raw(7),
            "example.qasm",
            "OPENQASM 3;\nqubit[2] q;\n// hello\nx q[0];",
        )
        .expect("test source must be valid")
    }

    #[test]
    fn source_id_round_trips() {
        let id = SourceId::from_raw(42);

        assert_eq!(id.as_raw(), 42);
        assert_eq!(SourceId::from_raw(id.as_raw()), id);
        assert_eq!(id.to_string(), "source#42");
    }

    #[test]
    fn source_offset_operations_are_checked() {
        let offset = SourceOffset::from_raw(10);

        assert_eq!(offset.as_usize(), 10);
        assert_eq!(offset.checked_add(5), Some(SourceOffset::from_raw(15)));
        assert_eq!(
            SourceOffset::from_raw(5).checked_distance(offset),
            Some(5)
        );
        assert_eq!(
            offset.checked_distance(SourceOffset::from_raw(5)),
            None
        );
    }

    #[test]
    fn spans_are_half_open() {
        let id = SourceId::from_raw(1);
        let span = SourceSpan::new(
            id,
            SourceOffset::from_raw(2),
            SourceOffset::from_raw(5),
        )
        .expect("span must be valid");

        assert_eq!(span.len_bytes(), 3);
        assert!(!span.is_empty());
        assert!(span.contains(SourceOffset::from_raw(2)));
        assert!(span.contains(SourceOffset::from_raw(4)));
        assert!(!span.contains(SourceOffset::from_raw(5)));
    }

    #[test]
    fn reversed_spans_are_rejected() {
        let result = SourceSpan::new(
            SourceId::from_raw(1),
            SourceOffset::from_raw(10),
            SourceOffset::from_raw(5),
        );

        assert_eq!(result, Err(SourceSpanError::ReversedRange));
    }

    #[test]
    fn span_union_requires_same_source() {
        let first = SourceSpan::new(
            SourceId::from_raw(1),
            SourceOffset::from_raw(2),
            SourceOffset::from_raw(5),
        )
        .expect("valid span");

        let second = SourceSpan::new(
            SourceId::from_raw(1),
            SourceOffset::from_raw(7),
            SourceOffset::from_raw(10),
        )
        .expect("valid span");

        let union = first.union(second).expect("same source");

        assert_eq!(union.start().as_raw(), 2);
        assert_eq!(union.end().as_raw(), 10);

        let other_source = SourceSpan::new(
            SourceId::from_raw(2),
            SourceOffset::from_raw(1),
            SourceOffset::from_raw(4),
        )
        .expect("valid span");

        assert!(first.union(other_source).is_none());
    }

    #[test]
    fn source_line_index_handles_lf() {
        let source =
            SourceFile::new(SourceId::from_raw(0), "test", "one\ntwo\nthree").expect("valid source");

        assert_eq!(source.line_count(), 3);
        assert_eq!(source.line_start(LineNumber::from_raw(1)).unwrap().as_raw(), 0);
        assert_eq!(source.line_start(LineNumber::from_raw(2)).unwrap().as_raw(), 4);
        assert_eq!(source.line_start(LineNumber::from_raw(3)).unwrap().as_raw(), 8);
    }

    #[test]
    fn source_line_index_handles_crlf() {
        let source =
            SourceFile::new(SourceId::from_raw(0), "test", "one\r\ntwo\r\nthree").expect("valid source");

        assert_eq!(source.line_count(), 3);
        assert_eq!(source.line_start(LineNumber::from_raw(1)).unwrap().as_raw(), 0);
        assert_eq!(source.line_start(LineNumber::from_raw(2)).unwrap().as_raw(), 5);
        assert_eq!(source.line_start(LineNumber::from_raw(3)).unwrap().as_raw(), 10);
    }

    #[test]
    fn source_line_index_handles_standalone_cr() {
        let source =
            SourceFile::new(SourceId::from_raw(0), "test", "one\rtwo\rthree").expect("valid source");

        assert_eq!(source.line_count(), 3);
        assert_eq!(source.line_start(LineNumber::from_raw(2)).unwrap().as_raw(), 4);
        assert_eq!(source.line_start(LineNumber::from_raw(3)).unwrap().as_raw(), 8);
    }

    #[test]
    fn empty_source_has_zero_lines() {
        let source =
            SourceFile::new(SourceId::from_raw(0), "empty", "").expect("valid source");

        assert_eq!(source.line_count(), 0);
        assert!(source.is_empty());
        assert_eq!(source.len_bytes(), 0);
    }

    #[test]
    fn position_is_one_based() {
        let source = SourceFile::new(SourceId::from_raw(0), "test", "abc\ndef").expect("valid");

        let position = source
            .position_at(SourceOffset::from_raw(0))
            .expect("position");

        assert_eq!(position.line().as_raw(), 1);
        assert_eq!(position.column().as_raw(), 1);

        let position = source
            .position_at(SourceOffset::from_raw(2))
            .expect("position");

        assert_eq!(position.line().as_raw(), 1);
        assert_eq!(position.column().as_raw(), 3);

        let position = source
            .position_at(SourceOffset::from_raw(4))
            .expect("position");

        assert_eq!(position.line().as_raw(), 2);
        assert_eq!(position.column().as_raw(), 1);
    }

    #[test]
    fn unicode_columns_are_scalar_based() {
        let source = SourceFile::new(
            SourceId::from_raw(0),
            "test",
            "aé中x",
        )
        .expect("valid");

        // `é` occupies two UTF-8 bytes and `中` occupies three, but both
        // count as one human-readable column.
        let x_offset = "aé中".len();

        let position = source
            .position_at(SourceOffset::from_raw(
                u32::try_from(x_offset).expect("small test offset"),
            ))
            .expect("position");

        assert_eq!(position.line().as_raw(), 1);
        assert_eq!(position.column().as_raw(), 4);
    }

    #[test]
    fn invalid_utf8_boundary_is_rejected_for_position() {
        let source = SourceFile::new(SourceId::from_raw(0), "test", "éx").expect("valid");

        // Byte offset 1 is inside the two-byte UTF-8 encoding of `é`.
        assert!(source.position_at(SourceOffset::from_raw(1)).is_none());
    }

    #[test]
    fn invalid_utf8_boundary_is_rejected_for_slice() {
        let source = SourceFile::new(SourceId::from_raw(0), "test", "éx").expect("valid");

        let span = SourceSpan::new(
            SourceId::from_raw(0),
            SourceOffset::from_raw(1),
            SourceOffset::from_raw(2),
        )
        .expect("range itself is ordered");

        assert!(source.slice(span).is_none());
    }

    #[test]
    fn source_slice_returns_exact_text() {
        let source = SourceFile::new(
            SourceId::from_raw(0),
            "test",
            "OPENQASM 3;",
        )
        .expect("valid");

        let span = SourceSpan::new(
            SourceId::from_raw(0),
            SourceOffset::from_raw(0),
            SourceOffset::from_raw(8),
        )
        .expect("valid");

        assert_eq!(source.slice(span), Some("OPENQASM"));
    }

    #[test]
    fn line_span_includes_line_terminator() {
        let source =
            SourceFile::new(SourceId::from_raw(0), "test", "abc\ndef").expect("valid");

        let span = source
            .line_span_at(SourceOffset::from_raw(1))
            .expect("line span");

        assert_eq!(source.slice(span), Some("abc\n"));
    }

    #[test]
    fn final_line_has_no_artificial_terminator() {
        let source =
            SourceFile::new(SourceId::from_raw(0), "test", "abc\ndef").expect("valid");

        let span = source
            .line_span_at(SourceOffset::from_raw(5))
            .expect("line span");

        assert_eq!(source.slice(span), Some("def"));
    }

    #[test]
    fn line_text_is_resolved_without_copying() {
        let source =
            SourceFile::new(SourceId::from_raw(0), "test", "abc\ndef").expect("valid");

        assert_eq!(
            source.line_text_at(SourceOffset::from_raw(5)),
            Some("def")
        );
    }

    #[test]
    fn source_map_assigns_stable_ids() {
        let mut map = SourceMap::new();

        let first = map.add("first.qasm", "x q[0];").expect("source");
        let second = map.add("second.qasm", "h q[1];").expect("source");

        assert_eq!(first.as_raw(), 0);
        assert_eq!(second.as_raw(), 1);
        assert_eq!(map.len(), 2);
        assert!(!map.is_empty());

        assert_eq!(map.get(first).unwrap().name(), "first.qasm");
        assert_eq!(map.get(second).unwrap().name(), "second.qasm");
    }

    #[test]
    fn source_map_rejects_unknown_sources() {
        let map = SourceMap::new();
        let id = SourceId::from_raw(99);

        assert!(map.get(id).is_none());
        assert_eq!(
            map.require(id),
            Err(SourceLookupError::UnknownSource(id))
        );
    }

    #[test]
    fn source_map_resolves_positions() {
        let mut map = SourceMap::new();

        let id = map.add("test.qasm", "h q[0];\nx q[1];").expect("source");

        let position = map
            .position_at(id, SourceOffset::from_raw(8))
            .expect("position");

        assert_eq!(position.line().as_raw(), 2);
        assert_eq!(position.column().as_raw(), 1);
    }

    #[test]
    fn source_map_slice_rejects_foreign_span() {
        let mut map = SourceMap::new();

        let first = map.add("first", "abc").expect("source");
        let _second = map.add("second", "xyz").expect("source");

        let span = SourceSpan::new(
            SourceId::from_raw(first.as_raw() + 1),
            SourceOffset::from_raw(0),
            SourceOffset::from_raw(1),
        )
        .expect("valid span");

        assert!(map.slice(span).is_some());

        let foreign = SourceSpan::new(
            SourceId::from_raw(100),
            SourceOffset::from_raw(0),
            SourceOffset::from_raw(1),
        )
        .expect("valid span");

        assert!(map.slice(foreign).is_none());
    }

    #[test]
    fn entire_source_span_is_correct() {
        let source = source();
        let span = SourceSpan::entire(&source);

        assert_eq!(span.source_id(), source.id());
        assert_eq!(span.start().as_raw(), 0);
        assert_eq!(span.end().as_raw(), source.len_bytes());
        assert_eq!(source.slice(span), Some(source.text()));
    }

    #[test]
    fn point_span_is_empty() {
        let span = SourceSpan::point(
            SourceId::from_raw(1),
            SourceOffset::from_raw(10),
        );

        assert!(span.is_empty());
        assert_eq!(span.len_bytes(), 0);
        assert!(!span.contains(SourceOffset::from_raw(10)));
    }

    #[test]
    fn spans_from_different_sources_do_not_overlap() {
        let first = SourceSpan::new(
            SourceId::from_raw(1),
            SourceOffset::from_raw(0),
            SourceOffset::from_raw(10),
        )
        .expect("valid");

        let second = SourceSpan::new(
            SourceId::from_raw(2),
            SourceOffset::from_raw(0),
            SourceOffset::from_raw(10),
        )
        .expect("valid");

        assert!(!first.overlaps(second));
        assert!(!first.contains_span(second));
    }

    #[test]
    fn source_position_display_is_stable() {
        let position = SourcePosition::new(
            SourceId::from_raw(3),
            SourceOffset::from_raw(12),
            LineNumber::from_raw(4),
            ColumnNumber::from_raw(7),
        );

        assert_eq!(position.to_string(), "source#3:4:7");
    }

    #[test]
    fn source_span_display_is_stable() {
        let span = SourceSpan::new(
            SourceId::from_raw(3),
            SourceOffset::from_raw(12),
            SourceOffset::from_raw(20),
        )
        .expect("valid");

        assert_eq!(span.to_string(), "source#3:12..20");
    }
}