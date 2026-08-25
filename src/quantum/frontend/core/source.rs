//! Canonical source identity and location infrastructure for the Zamani
//! Quantum Frontend.
//!
//! This module is the single source-location authority for the frontend. It
//! is deliberately independent of OpenQASM, QIR, Quil, Quantum IR, parsers,
//! validators, diagnostic rendering, filesystems, networking, and execution.
//!
//! # Architecture
//!
//! All frontend components use this model for source provenance:
//!
//! ```text
//! input bytes/text
//!      │
//!      ▼
//!   SourceMap ──► SourceFile
//!      │             │
//!      │             ├── UTF-8 byte offsets
//!      │             ├── line index
//!      │             └── Unicode-scalar columns
//!      ▼
//!  SourceSpan / SourcePosition
//!      │
//!      ├── lexer tokens
//!      ├── AST nodes
//!      ├── validation diagnostics
//!      ├── lowering provenance
//!      └── import/export diagnostics
//! ```
//!
//! No later frontend file needs to invent another source-location type.
//! Format implementations may convert their internal ranges into this model
//! at their boundary, but the public frontend contracts use [`SourceSpan`].
//!
//! # Coordinate contract
//!
//! * Offsets are zero-based UTF-8 byte offsets.
//! * Spans are half-open: `[start, end)`.
//! * Lines are one-based.
//! * Columns are one-based Unicode scalar-value columns.
//! * The canonical coordinate for slicing is the byte offset.
//! * A byte offset inside a UTF-8 code point is invalid.
//! * EOF is a valid source position, including for an empty source.
//!
//! # Resource and security contract
//!
//! Source text is untrusted. This module performs no filesystem, network,
//! process, or hardware access. It validates all ranges before slicing,
//! rejects source text whose length cannot be represented by the compact
//! frontend coordinate model, uses checked arithmetic for externally supplied
//! offsets, and never panics for ordinary malformed source-location input.
//!
//! The lexer/parser/importer are responsible for enforcing configured
//! `FrontendLimits`; this module enforces the invariants of the source model.
//!
//! # Rust compatibility
//!
//! Rust 2021, Rust 1.97.1.
//! No nightly features and no external crates.

use std::fmt;
use std::ops::Range;
use std::sync::Arc;

/// Stable identifier for a source registered in a [`SourceMap`].
///
/// IDs are monotonically assigned by [`SourceMap::add`], starting at zero.
/// `from_raw` is available for serialization and integration but does not
/// assert that the identifier exists in any particular map.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SourceId(u32);

impl SourceId {
    /// Creates an identifier from its raw representation.
    #[must_use]
    pub const fn from_raw(raw: u32) -> Self {
        Self(raw)
    }

    /// Returns the raw representation.
    #[must_use]
    pub const fn as_raw(self) -> u32 {
        self.0
    }
}

impl fmt::Display for SourceId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "source#{}", self.0)
    }
}

/// Zero-based byte offset into UTF-8 source.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SourceOffset(u32);

impl SourceOffset {
    /// Creates an offset from its raw representation.
    #[must_use]
    pub const fn from_raw(raw: u32) -> Self {
        Self(raw)
    }

    /// Converts a platform-sized byte offset into the compact frontend type.
    #[must_use]
    pub fn try_from_usize(value: usize) -> Result<Self, SourceSpanError> {
        u32::try_from(value)
            .map(Self)
            .map_err(|_| SourceSpanError::OffsetOverflow)
    }

    /// Returns the raw byte offset.
    #[must_use]
    pub const fn as_raw(self) -> u32 {
        self.0
    }

    /// Returns the offset as `usize` for Rust string indexing.
    #[must_use]
    pub const fn as_usize(self) -> usize {
        self.0 as usize
    }

    /// Returns whether this is offset zero.
    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.0 == 0
    }

    /// Adds a byte count without wrapping.
    #[must_use]
    pub const fn checked_add(self, amount: u32) -> Option<Self> {
        match self.0.checked_add(amount) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }

    /// Returns `end - self` when `end >= self`.
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

/// One-based source line number.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct LineNumber(u32);

impl LineNumber {
    /// First user-visible source line.
    pub const FIRST: Self = Self(1);

    /// Creates a line number from its raw representation.
    #[must_use]
    pub const fn from_raw(raw: u32) -> Self {
        Self(raw)
    }

    /// Returns the raw line number.
    #[must_use]
    pub const fn as_raw(self) -> u32 {
        self.0
    }

    /// Returns the next line number without overflowing.
    #[must_use]
    pub const fn checked_next(self) -> Option<Self> {
        match self.0.checked_add(1) {
            Some(value) => Some(Self(value)),
            None => None,
        }
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

/// One-based Unicode-scalar source column number.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ColumnNumber(u32);

impl ColumnNumber {
    /// First user-visible source column.
    pub const FIRST: Self = Self(1);

    /// Creates a column number from its raw representation.
    #[must_use]
    pub const fn from_raw(raw: u32) -> Self {
        Self(raw)
    }

    /// Returns the raw column number.
    #[must_use]
    pub const fn as_raw(self) -> u32 {
        self.0
    }

    /// Returns the next column without overflowing.
    #[must_use]
    pub const fn checked_next(self) -> Option<Self> {
        match self.0.checked_add(1) {
            Some(value) => Some(Self(value)),
            None => None,
        }
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

/// A one-based line/column pair suitable for diagnostics and editor APIs.
///
/// Byte offset information remains available through [`SourcePosition`].
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct LineColumn {
    line: LineNumber,
    column: ColumnNumber,
}

impl LineColumn {
    /// Creates a line/column pair.
    #[must_use]
    pub const fn new(line: LineNumber, column: ColumnNumber) -> Self {
        Self { line, column }
    }

    /// Returns the one-based line.
    #[must_use]
    pub const fn line(self) -> LineNumber {
        self.line
    }

    /// Returns the one-based column.
    #[must_use]
    pub const fn column(self) -> ColumnNumber {
        self.column
    }
}

impl fmt::Display for LineColumn {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}:{}",
            self.line.as_raw(),
            self.column.as_raw()
        )
    }
}

/// A half-open byte range associated with one source.
///
/// The range is `[start, end)`. Empty spans are valid and are used for
/// insertion-point diagnostics such as an unexpected token at EOF.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SourceSpan {
    source_id: SourceId,
    start: SourceOffset,
    end: SourceOffset,
}

impl SourceSpan {
    /// Creates a span after checking its ordering invariant.
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

    /// Creates a span from platform-sized byte offsets.
    pub fn from_usize(
        source_id: SourceId,
        start: usize,
        end: usize,
    ) -> Result<Self, SourceSpanError> {
        let start = SourceOffset::try_from_usize(start)?;
        let end = SourceOffset::try_from_usize(end)?;

        Self::new(source_id, start, end)
    }

    /// Creates an empty span at `offset`.
    #[must_use]
    pub const fn point(source_id: SourceId, offset: SourceOffset) -> Self {
        Self {
            source_id,
            start: offset,
            end: offset,
        }
    }

    /// Creates a span covering the complete source text.
    #[must_use]
    pub fn entire(source: &SourceFile) -> Self {
        Self {
            source_id: source.id(),
            start: SourceOffset::from_raw(0),
            end: SourceOffset::from_raw(source.len_bytes()),
        }
    }

    /// Returns this span as a platform-sized range.
    #[must_use]
    pub fn as_range_usize(self) -> Range<usize> {
        self.start.as_usize()..self.end.as_usize()
    }

    /// Returns the source identifier.
    #[must_use]
    pub const fn source_id(self) -> SourceId {
        self.source_id
    }

    /// Returns the inclusive start offset.
    #[must_use]
    pub const fn start(self) -> SourceOffset {
        self.start
    }

    /// Returns the exclusive end offset.
    #[must_use]
    pub const fn end(self) -> SourceOffset {
        self.end
    }

    /// Returns the byte length.
    #[must_use]
    pub const fn len_bytes(self) -> u32 {
        self.end.as_raw() - self.start.as_raw()
    }

    /// Returns whether the span is empty.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.start.as_raw() == self.end.as_raw()
    }

    /// Returns whether a byte offset is contained in this half-open span.
    #[must_use]
    pub const fn contains(self, offset: SourceOffset) -> bool {
        offset.as_raw() >= self.start.as_raw()
            && offset.as_raw() < self.end.as_raw()
    }

    /// Returns whether `other` is fully contained by this span.
    #[must_use]
    pub const fn contains_span(self, other: Self) -> bool {
        self.source_id == other.source_id
            && other.start.as_raw() >= self.start.as_raw()
            && other.end.as_raw() <= self.end.as_raw()
    }

    /// Returns whether two non-empty spans overlap.
    #[must_use]
    pub const fn overlaps(self, other: Self) -> bool {
        self.source_id == other.source_id
            && self.start.as_raw() < other.end.as_raw()
            && other.start.as_raw() < self.end.as_raw()
    }

    /// Returns the smallest span containing both spans.
    ///
    /// The operation requires the spans to belong to the same source.
    #[must_use]
    pub const fn union(self, other: Self) -> Option<Self> {
        if self.source_id != other.source_id {
            return None;
        }

        let start = if self.start.as_raw() <= other.start.as_raw() {
            self.start
        } else {
            other.start
        };

        let end = if self.end.as_raw() >= other.end.as_raw() {
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
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}:{}..{}",
            self.source_id,
            self.start.as_raw(),
            self.end.as_raw()
        )
    }
}

/// Errors produced by the compact source-location model.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SourceSpanError {
    /// Start is greater than end.
    ReversedRange,

    /// A platform-sized offset cannot fit into the frontend's 32-bit offset.
    OffsetOverflow,
}

impl fmt::Display for SourceSpanError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ReversedRange => {
                formatter.write_str(
                    "source span start offset exceeds end offset",
                )
            }
            Self::OffsetOverflow => {
                formatter.write_str(
                    "source offset exceeds the frontend supported range",
                )
            }
        }
    }
}

impl std::error::Error for SourceSpanError {}

/// A fully resolved source position.
///
/// The byte offset is canonical. Line/column values are derived coordinates
/// intended for diagnostics, IDEs, and user interfaces.
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

    /// Returns the canonical byte offset.
    #[must_use]
    pub const fn offset(self) -> SourceOffset {
        self.offset
    }

    /// Returns the one-based line.
    #[must_use]
    pub const fn line(self) -> LineNumber {
        self.line
    }

    /// Returns the one-based Unicode-scalar column.
    #[must_use]
    pub const fn column(self) -> ColumnNumber {
        self.column
    }

    /// Returns the line/column portion of this position.
    #[must_use]
    pub const fn line_column(self) -> LineColumn {
        LineColumn::new(self.line, self.column)
    }
}

impl fmt::Display for SourcePosition {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}:{}:{}",
            self.source_id,
            self.line.as_raw(),
            self.column.as_raw()
        )
    }
}

/// Immutable source text and its precomputed line index.
///
/// Source text is stored in `Arc<str>` so diagnostics and AST/provenance
/// consumers can retain it cheaply without repeatedly copying the complete
/// source. `SourceFile` performs no I/O.
#[derive(Clone, Debug)]
pub struct SourceFile {
    id: SourceId,
    name: Arc<str>,
    text: Arc<str>,
    line_starts: Arc<[u32]>,
}

impl SourceFile {
    /// Constructs a source file and validates the compact coordinate limits.
    pub fn new(
        id: SourceId,
        name: impl Into<Arc<str>>,
        text: impl Into<Arc<str>>,
    ) -> Result<Self, SourceFileError> {
        let name = name.into();
        let text = text.into();

        if text.len() > u32::MAX as usize {
            return Err(SourceFileError::SourceTooLarge);
        }

        let line_starts = build_line_index(&text)?;

        Ok(Self {
            id,
            name,
            text,
            line_starts: line_starts.into(),
        })
    }

    /// Returns the stable source identifier.
    #[must_use]
    pub const fn id(&self) -> SourceId {
        self.id
    }

    /// Returns the caller-supplied display name.
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
        // Construction guarantees that this conversion cannot truncate.
        self.text.len() as u32
    }

    /// Returns whether the source has no bytes.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }

    /// Returns the number of logical lines represented by the source.
    ///
    /// An empty source has zero logical lines. EOF in an empty source is still
    /// represented as line 1, column 1 by [`Self::position_at`].
    ///
    /// A source ending in a line terminator has a final empty logical line.
    #[must_use]
    pub fn line_count(&self) -> u32 {
        self.line_starts.len() as u32
    }

    /// Returns the byte offset at which a one-based line begins.
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

    /// Resolves a valid UTF-8 byte boundary to a source position.
    ///
    /// EOF is valid. For an empty source, EOF resolves to line 1, column 1.
    /// For a non-empty source, EOF resolves to the position immediately after
    /// the final source character. A byte offset inside a UTF-8 code point is
    /// rejected rather than guessed.
    #[must_use]
    pub fn position_at(
        &self,
        offset: SourceOffset,
    ) -> Option<SourcePosition> {
        let offset_usize = offset.as_usize();

        if offset_usize > self.text.len()
            || !self.text.is_char_boundary(offset_usize)
        {
            return None;
        }

        if self.text.is_empty() {
            return Some(SourcePosition::new(
                self.id,
                offset,
                LineNumber::FIRST,
                ColumnNumber::FIRST,
            ));
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

    /// Resolves a span's start position.
    #[must_use]
    pub fn start_position(
        &self,
        span: SourceSpan,
    ) -> Option<SourcePosition> {
        if span.source_id() != self.id {
            return None;
        }

        self.position_at(span.start())
    }

    /// Resolves a span's exclusive end position.
    #[must_use]
    pub fn end_position(
        &self,
        span: SourceSpan,
    ) -> Option<SourcePosition> {
        if span.source_id() != self.id {
            return None;
        }

        self.position_at(span.end())
    }

    /// Returns the exact UTF-8 text covered by a span.
    #[must_use]
    pub fn slice(&self, span: SourceSpan) -> Option<&str> {
        if span.source_id() != self.id {
            return None;
        }

        let range = span.as_range_usize();

        if range.start > range.end
            || range.end > self.text.len()
        {
            return None;
        }

        if !self.text.is_char_boundary(range.start)
            || !self.text.is_char_boundary(range.end)
        {
            return None;
        }

        self.text.get(range)
    }

    /// Returns the complete logical line containing `offset`.
    ///
    /// The returned span includes its line terminator when one exists. EOF is
    /// accepted and resolves to the final line. An empty source yields an
    /// empty `[0, 0)` line span.
    #[must_use]
    pub fn line_span_at(
        &self,
        offset: SourceOffset,
    ) -> Option<SourceSpan> {
        let position = self.position_at(offset)?;

        if self.text.is_empty() {
            return Some(SourceSpan::point(
                self.id,
                SourceOffset::from_raw(0),
            ));
        }

        let index = (position.line().as_raw() - 1) as usize;
        let start = *self.line_starts.get(index)?;

        let end = self
            .line_starts
            .get(index + 1)
            .copied()
            .unwrap_or_else(|| self.len_bytes());

        Some(SourceSpan {
            source_id: self.id,
            start: SourceOffset::from_raw(start),
            end: SourceOffset::from_raw(end),
        })
    }

    /// Returns the exact text of the line containing `offset`.
    #[must_use]
    pub fn line_text_at(
        &self,
        offset: SourceOffset,
    ) -> Option<&str> {
        let span = self.line_span_at(offset)?;
        self.slice(span)
    }

    /// Returns all line-start byte offsets.
    #[must_use]
    pub fn line_starts(&self) -> &[u32] {
        &self.line_starts
    }
}

/// Errors encountered while constructing a [`SourceFile`].
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SourceFileError {
    /// Source bytes cannot be represented by the compact 32-bit offset model.
    SourceTooLarge,

    /// A source line-start index would exceed the supported representation.
    TooManyLines,
}

impl fmt::Display for SourceFileError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SourceTooLarge => formatter.write_str(
                "source file is too large for the frontend source model",
            ),
            Self::TooManyLines => formatter.write_str(
                "source file contains too many lines for the frontend source model",
            ),
        }
    }
}

impl std::error::Error for SourceFileError {}

/// Immutable collection of source files known to one frontend operation.
///
/// `SourceMap` is intentionally not a filesystem abstraction. Callers provide
/// names and already-loaded UTF-8 text. Include resolution and I/O policy
/// therefore remain outside this source-location layer.
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

    /// Returns whether no sources are registered.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.files.is_empty()
    }

    /// Adds an immutable source and returns its stable identifier.
    ///
    /// IDs are assigned monotonically and are never reused while the map
    /// remains alive. Identical source text may be registered more than once;
    /// such registrations intentionally receive different IDs.
    pub fn add(
        &mut self,
        name: impl Into<Arc<str>>,
        text: impl Into<Arc<str>>,
    ) -> Result<SourceId, SourceFileError> {
        let raw_id = u32::try_from(self.files.len())
            .map_err(|_| SourceFileError::TooManyLines)?;

        let id = SourceId::from_raw(raw_id);
        let source = SourceFile::new(id, name, text)?;

        self.files.push(source);

        Ok(id)
    }

    /// Returns a source by identifier.
    #[must_use]
    pub fn get(&self, id: SourceId) -> Option<&SourceFile> {
        self.files.get(id.as_raw() as usize)
    }

    /// Returns a source or a structured lookup error.
    pub fn require(
        &self,
        id: SourceId,
    ) -> Result<&SourceFile, SourceLookupError> {
        self.get(id)
            .ok_or(SourceLookupError::UnknownSource(id))
    }

    /// Returns text covered by a source span.
    #[must_use]
    pub fn slice(&self, span: SourceSpan) -> Option<&str> {
        self.get(span.source_id())?.slice(span)
    }

    /// Resolves a source byte offset to a line/column position.
    #[must_use]
    pub fn position_at(
        &self,
        source_id: SourceId,
        offset: SourceOffset,
    ) -> Option<SourcePosition> {
        self.get(source_id)?.position_at(offset)
    }

    /// Returns the line span containing a source byte offset.
    #[must_use]
    pub fn line_span_at(
        &self,
        source_id: SourceId,
        offset: SourceOffset,
    ) -> Option<SourceSpan> {
        self.get(source_id)?.line_span_at(offset)
    }

    /// Iterates over sources in stable insertion order.
    pub fn iter(&self) -> impl Iterator<Item = &SourceFile> {
        self.files.iter()
    }
}

/// Error returned when a source identifier is not registered in a map.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SourceLookupError {
    /// The requested source does not exist.
    UnknownSource(SourceId),
}

impl fmt::Display for SourceLookupError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownSource(id) => {
                write!(formatter, "unknown source: {id}")
            }
        }
    }
}

impl std::error::Error for SourceLookupError {}

/// Builds a source line-start index.
///
/// LF, CRLF, and standalone CR are treated as line terminators. For CRLF,
/// only the byte after the `\n` is recorded as the next line start, so the
/// complete CRLF pair remains in the preceding line span.
fn build_line_index(
    text: &str,
) -> Result<Vec<u32>, SourceFileError> {
    if text.is_empty() {
        return Ok(Vec::new());
    }

    let mut starts = Vec::new();
    starts.push(0);

    let bytes = text.as_bytes();
    let mut index = 0usize;

    while index < bytes.len() {
        let next = match bytes[index] {
            b'\n' => index
                .checked_add(1)
                .ok_or(SourceFileError::SourceTooLarge)?,

            b'\r'
                if index + 1 < bytes.len()
                    && bytes[index + 1] == b'\n' =>
            {
                index
                    .checked_add(2)
                    .ok_or(SourceFileError::SourceTooLarge)?
            }

            b'\r' => index
                .checked_add(1)
                .ok_or(SourceFileError::SourceTooLarge)?,

            _ => {
                index += 1;
                continue;
            }
        };

        starts.push(
            u32::try_from(next)
                .map_err(|_| SourceFileError::SourceTooLarge)?,
        );

        index = next;
    }

    if starts.len() > u32::MAX as usize {
        return Err(SourceFileError::TooManyLines);
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
    fn identifiers_round_trip() {
        let id = SourceId::from_raw(42);

        assert_eq!(id.as_raw(), 42);
        assert_eq!(SourceId::from_raw(id.as_raw()), id);
        assert_eq!(id.to_string(), "source#42");
    }

    #[test]
    fn offsets_are_checked_and_convertible() {
        let offset = SourceOffset::from_raw(10);

        assert_eq!(offset.as_usize(), 10);
        assert_eq!(
            offset.checked_add(5),
            Some(SourceOffset::from_raw(15))
        );
        assert_eq!(
            SourceOffset::from_raw(5).checked_distance(offset),
            Some(5)
        );
        assert_eq!(
            offset.checked_distance(SourceOffset::from_raw(5)),
            None
        );
        assert_eq!(
            SourceOffset::try_from_usize(10),
            Ok(offset)
        );
    }

    #[test]
    fn spans_are_half_open() {
        let span = SourceSpan::new(
            SourceId::from_raw(1),
            SourceOffset::from_raw(2),
            SourceOffset::from_raw(5),
        )
        .expect("valid span");

        assert_eq!(span.len_bytes(), 3);
        assert!(span.contains(SourceOffset::from_raw(2)));
        assert!(span.contains(SourceOffset::from_raw(4)));
        assert!(!span.contains(SourceOffset::from_raw(5)));
        assert!(!span.is_empty());
    }

    #[test]
    fn reversed_spans_are_rejected() {
        assert_eq!(
            SourceSpan::new(
                SourceId::from_raw(1),
                SourceOffset::from_raw(10),
                SourceOffset::from_raw(5),
            ),
            Err(SourceSpanError::ReversedRange)
        );
    }

    #[test]
    fn platform_ranges_are_checked() {
        let span = SourceSpan::from_usize(
            SourceId::from_raw(1),
            2,
            5,
        )
        .expect("valid range");

        assert_eq!(span.as_range_usize(), 2..5);
    }

    #[test]
    fn line_index_handles_lf_crlf_and_cr() {
        let lf = SourceFile::new(
            SourceId::from_raw(0),
            "lf",
            "one\ntwo\nthree",
        )
        .unwrap();

        assert_eq!(lf.line_count(), 3);
        assert_eq!(
            lf.line_start(LineNumber::from_raw(2))
                .unwrap()
                .as_raw(),
            4
        );
        assert_eq!(
            lf.line_start(LineNumber::from_raw(3))
                .unwrap()
                .as_raw(),
            8
        );

        let crlf = SourceFile::new(
            SourceId::from_raw(0),
            "crlf",
            "one\r\ntwo\r\nthree",
        )
        .unwrap();

        assert_eq!(crlf.line_count(), 3);
        assert_eq!(
            crlf.line_start(LineNumber::from_raw(2))
                .unwrap()
                .as_raw(),
            5
        );
        assert_eq!(
            crlf.line_start(LineNumber::from_raw(3))
                .unwrap()
                .as_raw(),
            10
        );

        let cr = SourceFile::new(
            SourceId::from_raw(0),
            "cr",
            "one\rtwo\rthree",
        )
        .unwrap();

        assert_eq!(cr.line_count(), 3);
        assert_eq!(
            cr.line_start(LineNumber::from_raw(2))
                .unwrap()
                .as_raw(),
            4
        );
        assert_eq!(
            cr.line_start(LineNumber::from_raw(3))
                .unwrap()
                .as_raw(),
            8
        );
    }

    #[test]
    fn trailing_newline_creates_final_empty_line() {
        let source = SourceFile::new(
            SourceId::from_raw(0),
            "test",
            "one\n",
        )
        .unwrap();

        assert_eq!(source.line_count(), 2);
        assert_eq!(
            source.line_start(LineNumber::from_raw(2))
                .unwrap()
                .as_raw(),
            4
        );
        assert_eq!(
            source.line_text_at(SourceOffset::from_raw(4)),
            Some("")
        );
    }

    #[test]
    fn empty_source_has_valid_eof_position() {
        let source = SourceFile::new(
            SourceId::from_raw(0),
            "empty",
            "",
        )
        .unwrap();

        assert_eq!(source.line_count(), 0);

        let position = source
            .position_at(SourceOffset::from_raw(0))
            .unwrap();

        assert_eq!(position.line(), LineNumber::FIRST);
        assert_eq!(position.column(), ColumnNumber::FIRST);

        let span = source
            .line_span_at(SourceOffset::from_raw(0))
            .unwrap();

        assert!(span.is_empty());
        assert_eq!(source.slice(span), Some(""));
    }

    #[test]
    fn positions_are_one_based() {
        let source = SourceFile::new(
            SourceId::from_raw(0),
            "test",
            "abc\ndef",
        )
        .unwrap();

        let first = source
            .position_at(SourceOffset::from_raw(0))
            .unwrap();

        assert_eq!(first.line().as_raw(), 1);
        assert_eq!(first.column().as_raw(), 1);

        let third = source
            .position_at(SourceOffset::from_raw(2))
            .unwrap();

        assert_eq!(third.line().as_raw(), 1);
        assert_eq!(third.column().as_raw(), 3);

        let second_line = source
            .position_at(SourceOffset::from_raw(4))
            .unwrap();

        assert_eq!(second_line.line().as_raw(), 2);
        assert_eq!(second_line.column().as_raw(), 1);

        let eof = source
            .position_at(SourceOffset::from_raw(7))
            .unwrap();

        assert_eq!(eof.line().as_raw(), 2);
        assert_eq!(eof.column().as_raw(), 4);
    }

    #[test]
    fn unicode_columns_are_scalar_based() {
        let source = SourceFile::new(
            SourceId::from_raw(0),
            "test",
            "aé中x",
        )
        .unwrap();

        let offset =
            SourceOffset::try_from_usize("aé中".len()).unwrap();

        let position = source.position_at(offset).unwrap();

        assert_eq!(position.line().as_raw(), 1);
        assert_eq!(position.column().as_raw(), 4);

        assert_eq!(
            position.line_column(),
            LineColumn::new(
                LineNumber::from_raw(1),
                ColumnNumber::from_raw(4)
            )
        );
    }

    #[test]
    fn invalid_utf8_boundaries_are_rejected() {
        let source = SourceFile::new(
            SourceId::from_raw(0),
            "test",
            "éx",
        )
        .unwrap();

        assert!(
            source
                .position_at(SourceOffset::from_raw(1))
                .is_none()
        );

        let span = SourceSpan::new(
            SourceId::from_raw(0),
            SourceOffset::from_raw(1),
            SourceOffset::from_raw(2),
        )
        .unwrap();

        assert!(source.slice(span).is_none());
    }

    #[test]
    fn slices_are_exact_and_source_scoped() {
        let source = SourceFile::new(
            SourceId::from_raw(0),
            "test",
            "OPENQASM 3;",
        )
        .unwrap();

        let span = SourceSpan::new(
            SourceId::from_raw(0),
            SourceOffset::from_raw(0),
            SourceOffset::from_raw(8),
        )
        .unwrap();

        assert_eq!(source.slice(span), Some("OPENQASM"));

        let foreign = SourceSpan::new(
            SourceId::from_raw(99),
            SourceOffset::from_raw(0),
            SourceOffset::from_raw(1),
        )
        .unwrap();

        assert!(source.slice(foreign).is_none());
    }

    #[test]
    fn line_span_includes_terminator() {
        let source = SourceFile::new(
            SourceId::from_raw(0),
            "test",
            "abc\ndef",
        )
        .unwrap();

        let span = source
            .line_span_at(SourceOffset::from_raw(1))
            .unwrap();

        assert_eq!(source.slice(span), Some("abc\n"));
    }

    #[test]
    fn eof_after_trailing_newline_is_final_empty_line() {
        let source = SourceFile::new(
            SourceId::from_raw(0),
            "test",
            "abc\n",
        )
        .unwrap();

        let eof = SourceOffset::from_raw(4);
        let position = source.position_at(eof).unwrap();

        assert_eq!(position.line().as_raw(), 2);
        assert_eq!(position.column().as_raw(), 1);
        assert_eq!(source.line_text_at(eof), Some(""));
    }

    #[test]
    fn spans_union_only_within_one_source() {
        let first = SourceSpan::new(
            SourceId::from_raw(1),
            SourceOffset::from_raw(2),
            SourceOffset::from_raw(5),
        )
        .unwrap();

        let second = SourceSpan::new(
            SourceId::from_raw(1),
            SourceOffset::from_raw(7),
            SourceOffset::from_raw(10),
        )
        .unwrap();

        let union = first.union(second).unwrap();

        assert_eq!(union.start().as_raw(), 2);
        assert_eq!(union.end().as_raw(), 10);

        let other_source = SourceSpan::new(
            SourceId::from_raw(2),
            SourceOffset::from_raw(1),
            SourceOffset::from_raw(4),
        )
        .unwrap();

        assert!(first.union(other_source).is_none());
    }

    #[test]
    fn source_map_assigns_stable_insertion_ids() {
        let mut map = SourceMap::new();

        let first = map
            .add("first.qasm", "h q[0];")
            .unwrap();

        let second = map
            .add("second.qasm", "x q[1];")
            .unwrap();

        assert_eq!(first.as_raw(), 0);
        assert_eq!(second.as_raw(), 1);
        assert_eq!(map.len(), 2);
        assert!(!map.is_empty());

        assert_eq!(
            map.get(first).unwrap().name(),
            "first.qasm"
        );

        assert_eq!(
            map.get(second).unwrap().name(),
            "second.qasm"
        );
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
    fn source_map_resolves_positions_and_lines() {
        let mut map = SourceMap::new();

        let id = map
            .add("test.qasm", "h q[0];\nx q[1];")
            .unwrap();

        let position = map
            .position_at(id, SourceOffset::from_raw(8))
            .unwrap();

        assert_eq!(position.line().as_raw(), 2);
        assert_eq!(position.column().as_raw(), 1);

        let line = map
            .line_span_at(id, SourceOffset::from_raw(8))
            .unwrap();

        assert_eq!(
            map.slice(line),
            Some("x q[1];")
        );
    }

    #[test]
    fn entire_and_point_spans_are_valid() {
        let source = source();

        let entire = SourceSpan::entire(&source);

        assert_eq!(
            entire.source_id(),
            source.id()
        );
        assert_eq!(entire.start().as_raw(), 0);
        assert_eq!(
            entire.end().as_raw(),
            source.len_bytes()
        );
        assert_eq!(
            source.slice(entire),
            Some(source.text())
        );

        let point = SourceSpan::point(
            SourceId::from_raw(1),
            SourceOffset::from_raw(10),
        );

        assert!(point.is_empty());
        assert_eq!(point.len_bytes(), 0);
        assert!(!point.contains(SourceOffset::from_raw(10)));
    }

    #[test]
    fn source_position_and_span_display_are_stable() {
        let position = SourcePosition::new(
            SourceId::from_raw(3),
            SourceOffset::from_raw(12),
            LineNumber::from_raw(4),
            ColumnNumber::from_raw(7),
        );

        assert_eq!(
            position.to_string(),
            "source#3:4:7"
        );

        let span = SourceSpan::new(
            SourceId::from_raw(3),
            SourceOffset::from_raw(12),
            SourceOffset::from_raw(20),
        )
        .unwrap();

        assert_eq!(
            span.to_string(),
            "source#3:12..20"
        );
    }
}