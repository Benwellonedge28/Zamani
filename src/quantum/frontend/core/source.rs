//! Zamani Quantum Frontend — source management and source locations.
//!
//! This module owns the format-independent source model used by the quantum
//! frontend boundary.
//!
//! # Architectural boundary
//!
//! `source.rs` is deliberately independent of OpenQASM, QIR, Quil, and the
//! canonical Quantum IR.
//!
//! Its responsibilities are limited to:
//!
//! - identifying source documents;
//! - retaining source text;
//! - representing byte-based source spans;
//! - representing source positions;
//! - mapping byte offsets to line/column coordinates;
//! - managing multiple source documents;
//! - providing deterministic source-name and location information;
//! - enforcing source-size limits when requested.
//!
//! It does NOT:
//!
//! - tokenize source;
//! - parse source;
//! - validate language semantics;
//! - construct Quantum IR;
//! - resolve includes;
//! - perform filesystem or network I/O;
//! - interpret paths;
//! - know any quantum language grammar.
//!
//! # Dependency direction
//!
//! ```text
//!                         format-specific frontend
//!                                  |
//!                                  v
//!                         lexer / parser / validator
//!                                  |
//!                                  v
//!                    +-----------------------------+
//!                    | frontend::core::source      |
//!                    |                             |
//!                    | SourceId                    |
//!                    | SourceFile                  |
//!                    | SourceMap                   |
//!                    | SourcePosition              |
//!                    | SourceSpan                  |
//!                    | LineColumn                  |
//!                    +-----------------------------+
//!                         ^                    ^
//!                         |                    |
//!                  diagnostics             errors
//! ```
//!
//! The canonical Quantum IR remains completely independent of this module.
//! The IR boundary explicitly excludes frontend parsing and source-language
//! concerns.
//!
//! # Coordinate model
//!
//! All canonical source locations are byte-offset based.
//!
//! - offsets are zero-based;
//! - spans are half-open: `[start, end)`;
//! - offsets refer to UTF-8 byte positions;
//! - line numbers are one-based;
//! - column numbers are one-based;
//! - columns are measured in Unicode scalar values rather than UTF-8 bytes.
//!
//! This gives lexers and parsers a lossless, deterministic coordinate system
//! while still providing human-readable line/column diagnostics.
//!
//! A byte offset is always the authoritative location. Line/column values are
//! derived from the source text and must never be used as the primary identity
//! of a source location.
//!
//! # UTF-8 safety
//!
//! The source map never creates arbitrary `str` slices from byte offsets.
//! `SourceSpan::text()` and related APIs validate UTF-8 boundaries before
//! returning a slice.
//!
//! An offset inside a UTF-8 code point is therefore rejected rather than
//! producing undefined or surprising diagnostic behavior.
//!
//! # Determinism
//!
//! Source IDs are allocated monotonically in insertion order.
//!
//! The source map does not:
//!
//! - use hash iteration order;
//! - use timestamps;
//! - generate random IDs;
//! - depend on filesystem ordering;
//! - perform implicit I/O.
//!
//! Given the same source documents inserted in the same order, source IDs,
//! line tables, spans, and rendered positions are deterministic.
//!
//! # Security
//!
//! This module is an untrusted-input boundary.
//!
//! Source storage is bounded when `FrontendLimits` are supplied. The module
//! does not expose an "unlimited" mode through the limit-aware API.
//!
//! The module also avoids recursive source-map processing, performs checked
//! arithmetic where externally supplied offsets are involved, and never
//! panics on malformed source locations through its public fallible APIs.
//!
//! # Rust compatibility
//!
//! This implementation targets Rust 1.97.1 and Rust 2021.
//!
//! No nightly features are required.
//! No external crates are required.
//!
//! # Integration contract
//!
//! Later frontend modules should use this module as follows:
//!
//! ```text
//! source text
//!     |
//!     v
//! SourceMap::add_source(...)
//!     |
//!     +--> SourceId
//!     |
//!     +--> SourceFile
//!     |
//!     v
//! lexer produces SourceSpan
//!     |
//!     v
//! parser attaches SourceSpan to AST nodes
//!     |
//!     v
//! validation produces diagnostics using SourceSpan
//! ```
//!
//! Format-specific modules must never create their own source-location types.
//! OpenQASM, QIR, Quil, and future formats all use these types.

use std::fmt;
use std::ops::Range;
use std::sync::Arc;

use super::errors::{
    FrontendError,
    FrontendLimitViolation,
    FrontendResult,
};
use super::limits::FrontendLimits;

// =============================================================================
// Stable source identifiers
// =============================================================================

/// Stable identifier for a source document within one frontend operation.
///
/// `SourceId` is intentionally opaque. Callers should not depend on its
/// numeric representation beyond equality/ordering and diagnostic display.
///
/// IDs are allocated by [`SourceMap`] starting at zero.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SourceId(u32);

impl SourceId {
    /// Creates a source ID from its raw representation.
    ///
    /// This is primarily useful when deserializing or integrating with an
    /// external source-location protocol.
    ///
    /// Prefer IDs returned by [`SourceMap::add_source`] during normal
    /// compilation.
    #[must_use]
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }

    /// Returns the stable raw representation.
    #[must_use]
    pub const fn as_raw(self) -> u32 {
        self.0
    }
}

impl fmt::Display for SourceId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "source:{}", self.0)
    }
}

// =============================================================================
// Source positions
// =============================================================================

/// A zero-based byte offset within one source document.
///
/// `SourcePosition` deliberately contains only an offset. Human-readable
/// line/column coordinates are derived by [`SourceMap`].
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SourcePosition {
    source_id: SourceId,
    offset: usize,
}

impl SourcePosition {
    /// Creates a source position without validating that it belongs to an
    /// existing source document.
    ///
    /// This constructor is useful for parser/lexer code that is building a
    /// span while processing a known source.
    ///
    /// Use [`SourceMap::position`] when validating an externally supplied
    /// offset.
    #[must_use]
    pub const fn new(source_id: SourceId, offset: usize) -> Self {
        Self { source_id, offset }
    }

    /// Returns the source document identifier.
    #[must_use]
    pub const fn source_id(self) -> SourceId {
        self.source_id
    }

    /// Returns the zero-based UTF-8 byte offset.
    #[must_use]
    pub const fn offset(self) -> usize {
        self.offset
    }
}

impl fmt::Display for SourcePosition {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}@{}", self.source_id, self.offset)
    }
}

// =============================================================================
// Source spans
// =============================================================================

/// A half-open source range `[start, end)`.
///
/// The span is valid only when:
///
/// - `start` and `end` belong to the same source;
/// - `start <= end`;
/// - both offsets are valid positions in that source;
/// - when converted to text, both offsets are UTF-8 boundaries.
///
/// Empty spans are valid and represent insertion/point locations.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SourceSpan {
    source_id: SourceId,
    start: usize,
    end: usize,
}

impl SourceSpan {
    /// Creates a span without consulting a source map.
    ///
    /// This is appropriate for lexers and parsers operating over a known
    /// source buffer. The resulting span can later be validated with
    /// [`SourceMap::validate_span`].
    ///
    /// # Panics
    ///
    /// This constructor does not panic because it accepts an empty or ordered
    /// range only through the explicit `new` contract. If `start > end`, it
    /// returns `None`.
    #[must_use]
    pub const fn new(
        source_id: SourceId,
        start: usize,
        end: usize,
    ) -> Option<Self> {
        if start > end {
            None
        } else {
            Some(Self {
                source_id,
                start,
                end,
            })
        }
    }

    /// Creates a point span.
    #[must_use]
    pub const fn point(source_id: SourceId, offset: usize) -> Self {
        Self {
            source_id,
            start: offset,
            end: offset,
        }
    }

    /// Creates a span from a half-open range.
    #[must_use]
    pub const fn from_range(
        source_id: SourceId,
        range: Range<usize>,
    ) -> Option<Self> {
        Self::new(source_id, range.start, range.end)
    }

    /// Returns the source document identifier.
    #[must_use]
    pub const fn source_id(self) -> SourceId {
        self.source_id
    }

    /// Returns the inclusive start byte offset.
    ///
    /// Despite the historical wording "inclusive", this is the start of the
    /// half-open range and is therefore included in the span.
    #[must_use]
    pub const fn start(self) -> usize {
        self.start
    }

    /// Returns the exclusive end byte offset.
    #[must_use]
    pub const fn end(self) -> usize {
        self.end
    }

    /// Returns the span length in bytes.
    #[must_use]
    pub const fn len(self) -> usize {
        self.end - self.start
    }

    /// Returns whether the span contains no bytes.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.start == self.end
    }

    /// Returns the underlying half-open byte range.
    #[must_use]
    pub const fn range(self) -> Range<usize> {
        self.start..self.end
    }

    /// Returns the starting source position.
    #[must_use]
    pub const fn start_position(self) -> SourcePosition {
        SourcePosition::new(self.source_id, self.start)
    }

    /// Returns the ending source position.
    #[must_use]
    pub const fn end_position(self) -> SourcePosition {
        SourcePosition::new(self.source_id, self.end)
    }

    /// Returns whether another span belongs to the same source document.
    #[must_use]
    pub const fn same_source(self, other: Self) -> bool {
        self.source_id == other.source_id
    }

    /// Returns whether two spans overlap.
    ///
    /// Empty spans never overlap a non-empty span. Two identical empty spans
    /// are considered equal but are not considered overlapping.
    #[must_use]
    pub const fn overlaps(self, other: Self) -> bool {
        if !self.same_source(other) || self.is_empty() || other.is_empty() {
            return false;
        }

        self.start < other.end && other.start < self.end
    }

    /// Returns whether this span completely contains another span.
    #[must_use]
    pub const fn contains(self, other: Self) -> bool {
        self.same_source(other)
            && self.start <= other.start
            && other.end <= self.end
    }

    /// Returns a span covering both spans when they belong to the same source.
    ///
    /// Returns `None` for spans from different source documents.
    #[must_use]
    pub const fn join(self, other: Self) -> Option<Self> {
        if !self.same_source(other) {
            return None;
        }

        let start = if self.start < other.start {
            self.start
        } else {
            other.start
        };

        let end = if self.end > other.end {
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
            self.start,
            self.end
        )
    }
}

// =============================================================================
// Human-readable line/column coordinates
// =============================================================================

/// One-based human-readable source coordinates.
///
/// `line` and `column` are one-based.
///
/// `byte_offset` remains zero-based and is retained so callers do not have to
/// convert a diagnostic position back to the authoritative byte coordinate.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct LineColumn {
    line: usize,
    column: usize,
    byte_offset: usize,
}

impl LineColumn {
    /// Creates a line/column coordinate.
    #[must_use]
    pub const fn new(
        line: usize,
        column: usize,
        byte_offset: usize,
    ) -> Self {
        Self {
            line,
            column,
            byte_offset,
        }
    }

    /// Returns the one-based line number.
    #[must_use]
    pub const fn line(self) -> usize {
        self.line
    }

    /// Returns the one-based column number.
    #[must_use]
    pub const fn column(self) -> usize {
        self.column
    }

    /// Returns the zero-based UTF-8 byte offset.
    #[must_use]
    pub const fn byte_offset(self) -> usize {
        self.byte_offset
    }
}

impl fmt::Display for LineColumn {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}:{}",
            self.line,
            self.column
        )
    }
}

// =============================================================================
// Source files
// =============================================================================

/// Immutable source document stored in a [`SourceMap`].
///
/// The text is reference-counted so obtaining a source file does not require
/// copying the complete source document.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SourceFile {
    id: SourceId,
    name: Arc<str>,
    text: Arc<str>,
    line_starts: Arc<[usize]>,
}

impl SourceFile {
    /// Creates a source file.
    ///
    /// This constructor computes the line-start index once. The source text is
    /// retained unchanged.
    ///
    /// `name` is a display/diagnostic name. It is deliberately not interpreted
    /// as a filesystem path and no I/O is performed.
    pub fn new(
        id: SourceId,
        name: impl Into<String>,
        text: impl Into<String>,
    ) -> Self {
        let text: String = text.into();
        let line_starts = compute_line_starts(&text);

        Self {
            id,
            name: Arc::<str>::from(name.into()),
            text: Arc::<str>::from(text),
            line_starts: Arc::from(line_starts.into_boxed_slice()),
        }
    }

    /// Returns the source ID.
    #[must_use]
    pub const fn id(&self) -> SourceId {
        self.id
    }

    /// Returns the diagnostic/display name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the complete source text.
    #[must_use]
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Returns the source length in UTF-8 bytes.
    #[must_use]
    pub fn len(&self) -> usize {
        self.text.len()
    }

    /// Returns whether the source is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }

    /// Returns the number of logical lines represented by the source.
    ///
    /// An empty source has one logical line, which makes diagnostics for an
    /// empty file naturally resolve to line 1, column 1.
    #[must_use]
    pub fn line_count(&self) -> usize {
        self.line_starts.len()
    }

    /// Returns the indexed line-start byte offsets.
    ///
    /// The first entry is always zero.
    ///
    /// This is exposed read-only so diagnostics/renderers can efficiently
    /// inspect source structure without rebuilding the index.
    #[must_use]
    pub fn line_starts(&self) -> &[usize] {
        &self.line_starts
    }

    /// Checks whether an offset is within the source's valid boundary.
    ///
    /// `len()` is a valid position because spans use half-open ranges.
    #[must_use]
    pub fn contains_offset(&self, offset: usize) -> bool {
        offset <= self.text.len()
    }

    /// Checks whether an offset is a UTF-8 boundary.
    #[must_use]
    pub fn is_char_boundary(&self, offset: usize) -> bool {
        self.text.is_char_boundary(offset)
    }

    /// Returns the source text at a validated byte range.
    ///
    /// Returns `None` if:
    ///
    /// - the range is inverted;
    /// - either boundary is outside the source;
    /// - either boundary is not a UTF-8 character boundary.
    #[must_use]
    pub fn slice(&self, range: Range<usize>) -> Option<&str> {
        if range.start > range.end
            || range.end > self.text.len()
            || !self.text.is_char_boundary(range.start)
            || !self.text.is_char_boundary(range.end)
        {
            return None;
        }

        self.text.get(range)
    }

    /// Returns the source text represented by a span belonging to this file.
    #[must_use]
    pub fn slice_span(&self, span: SourceSpan) -> Option<&str> {
        if span.source_id != self.id {
            return None;
        }

        self.slice(span.range())
    }

    /// Returns the line/column for a validated byte offset.
    ///
    /// The offset must be within the source and on a UTF-8 boundary.
    #[must_use]
    pub fn line_column(&self, offset: usize) -> Option<LineColumn> {
        if !self.contains_offset(offset)
            || !self.is_char_boundary(offset)
        {
            return None;
        }

        let line_index = match self.line_starts.binary_search(&offset) {
            Ok(index) => index,
            Err(insertion_point) => insertion_point.saturating_sub(1),
        };

        let line_start = self.line_starts[line_index];

        // `line_start` and `offset` are UTF-8 boundaries. Counting chars is
        // therefore safe and produces Unicode-scalar-value columns.
        let column = self.text[line_start..offset]
            .chars()
            .count()
            + 1;

        Some(LineColumn::new(
            line_index + 1,
            column,
            offset,
        ))
    }

    /// Returns the line containing the supplied byte offset.
    ///
    /// The returned range excludes the line terminator.
    #[must_use]
    pub fn line_range(&self, offset: usize) -> Option<Range<usize>> {
        let position = self.line_column(offset)?;

        let line_index = position.line - 1;
        let start = self.line_starts[line_index];

        let end = if line_index + 1 < self.line_starts.len() {
            let next_start = self.line_starts[line_index + 1];

            // Strip the line terminator from the returned logical line.
            if next_start >= 2
                && self.text.as_bytes()[next_start - 2] == b'\r'
                && self.text.as_bytes()[next_start - 1] == b'\n'
            {
                next_start - 2
            } else if next_start >= 1 {
                next_start - 1
            } else {
                next_start
            }
        } else {
            self.text.len()
        };

        Some(start..end)
    }

    /// Returns the complete logical line containing the supplied offset.
    #[must_use]
    pub fn line_text(&self, offset: usize) -> Option<&str> {
        let range = self.line_range(offset)?;
        self.text.get(range)
    }
}

// =============================================================================
// Source map
// =============================================================================

/// Immutable-after-publication collection of source documents.
///
/// `SourceMap` owns all source files for one frontend operation.
///
/// A source map may contain:
///
/// - one primary source document;
/// - multiple included/imported documents;
/// - generated/virtual documents supplied by callers.
///
/// It does not know how a source was obtained.
///
/// File access, include resolution, network access, and sandbox policy belong
/// to higher-level frontend components.
#[derive(Clone, Debug, Default)]
pub struct SourceMap {
    sources: Vec<SourceFile>,
}

impl SourceMap {
    /// Creates an empty source map.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            sources: Vec::new(),
        }
    }

    /// Creates a source map with preallocated storage.
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            sources: Vec::with_capacity(capacity),
        }
    }

    /// Returns the number of source documents.
    #[must_use]
    pub fn len(&self) -> usize {
        self.sources.len()
    }

    /// Returns whether the source map contains no documents.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.sources.is_empty()
    }

    /// Returns all source files in deterministic insertion order.
    #[must_use]
    pub fn sources(&self) -> &[SourceFile] {
        &self.sources
    }

    /// Adds a source document without a frontend-limit policy.
    ///
    /// This low-level method is intended for callers that already enforce
    /// their own source-size policy or for trusted/generated source.
    ///
    /// For external/untrusted input, prefer
    /// [`SourceMap::add_source_with_limits`].
    ///
    /// Returns `None` if the source-map ID space is exhausted.
    pub fn add_source(
        &mut self,
        name: impl Into<String>,
        text: impl Into<String>,
    ) -> Option<SourceId> {
        let id = SourceId::from_raw(
            u32::try_from(self.sources.len()).ok()?,
        );

        self.sources.push(SourceFile::new(id, name, text));

        Some(id)
    }

    /// Adds a source document while enforcing frontend resource limits.
    ///
    /// The source itself is never partially inserted. If a limit is exceeded,
    /// the source map remains unchanged.
    pub fn add_source_with_limits(
        &mut self,
        name: impl Into<String>,
        text: impl Into<String>,
        limits: &FrontendLimits,
    ) -> FrontendResult<SourceId> {
        let name = name.into();
        let text = text.into();

        limits.validate().map_err(|error| {
            FrontendError::invalid_input(error.to_string())
                .context("component", "source_map")
        })?;

        let source_bytes = text.len() as u64;

        if source_bytes > limits.max_source_bytes() {
            return Err(FrontendError::limit_exceeded(
                FrontendLimitViolation::new(
                    "max_source_bytes",
                    text.len(),
                    u64_to_usize_saturating(
                        limits.max_source_bytes(),
                    ),
                ),
            )
            .context("source", name));
        }

        let next_file_count = self
            .sources
            .len()
            .checked_add(1)
            .ok_or_else(|| {
                FrontendError::limit_exceeded(
                    FrontendLimitViolation::new(
                        "max_source_files",
                        usize::MAX,
                        u64_to_usize_saturating(
                            limits.max_source_files(),
                        ),
                    ),
                )
            })?;

        if next_file_count as u64 > limits.max_source_files() {
            return Err(FrontendError::limit_exceeded(
                FrontendLimitViolation::new(
                    "max_source_files",
                    next_file_count,
                    u64_to_usize_saturating(
                        limits.max_source_files(),
                    ),
                ),
            ));
        }

        let total_bytes = self.total_source_bytes();
        let new_total = total_bytes
            .checked_add(source_bytes)
            .ok_or_else(|| {
                FrontendError::limit_exceeded(
                    FrontendLimitViolation::new(
                        "max_total_source_bytes",
                        usize::MAX,
                        u64_to_usize_saturating(
                            limits.max_total_source_bytes(),
                        ),
                    ),
                )
            })?;

        if new_total > limits.max_total_source_bytes() {
            return Err(FrontendError::limit_exceeded(
                FrontendLimitViolation::new(
                    "max_total_source_bytes",
                    u64_to_usize_saturating(new_total),
                    u64_to_usize_saturating(
                        limits.max_total_source_bytes(),
                    ),
                ),
            ));
        }

        let id = self.add_source(name, text).ok_or_else(|| {
            FrontendError::limit_exceeded(
                FrontendLimitViolation::new(
                    "source_id_capacity",
                    self.sources.len(),
                    u32::MAX as usize,
                ),
            )
        })?;

        Ok(id)
    }

    /// Returns a source file by ID.
    #[must_use]
    pub fn get(&self, id: SourceId) -> Option<&SourceFile> {
        self.sources.get(id.as_raw() as usize)
    }

    /// Returns the source file by ID or a structured frontend error.
    pub fn require(
        &self,
        id: SourceId,
    ) -> FrontendResult<&SourceFile> {
        self.get(id).ok_or_else(|| {
            FrontendError::invalid_input(
                "source ID does not exist in the source map",
            )
            .context("source_id", id.as_raw().to_string())
        })
    }

    /// Returns the total number of source bytes.
    #[must_use]
    pub fn total_source_bytes(&self) -> u64 {
        self.sources
            .iter()
            .map(|source| source.len() as u64)
            .sum()
    }

    /// Creates and validates a source position.
    ///
    /// The returned position is guaranteed to reference an existing source
    /// and a valid UTF-8 boundary.
    pub fn position(
        &self,
        source_id: SourceId,
        offset: usize,
    ) -> FrontendResult<SourcePosition> {
        let source = self.require(source_id)?;

        if offset > source.len() {
            return Err(FrontendError::invalid_input(
                "source position is outside the source",
            )
            .context("source_id", source_id.as_raw().to_string())
            .context("offset", offset.to_string())
            .context("source_length", source.len().to_string()));
        }

        if !source.is_char_boundary(offset) {
            return Err(FrontendError::invalid_input(
                "source position is not on a UTF-8 character boundary",
            )
            .context("source_id", source_id.as_raw().to_string())
            .context("offset", offset.to_string()));
        }

        Ok(SourcePosition::new(source_id, offset))
    }

    /// Creates and validates a source span.
    ///
    /// Both boundaries must belong to the source and be UTF-8 boundaries.
    pub fn span(
        &self,
        source_id: SourceId,
        start: usize,
        end: usize,
    ) -> FrontendResult<SourceSpan> {
        if start > end {
            return Err(FrontendError::invalid_input(
                "source span start exceeds end",
            )
            .context("source_id", source_id.as_raw().to_string())
            .context("start", start.to_string())
            .context("end", end.to_string()));
        }

        let source = self.require(source_id)?;

        if end > source.len() {
            return Err(FrontendError::invalid_input(
                "source span exceeds source length",
            )
            .context("source_id", source_id.as_raw().to_string())
            .context("end", end.to_string())
            .context("source_length", source.len().to_string()));
        }

        if !source.is_char_boundary(start)
            || !source.is_char_boundary(end)
        {
            return Err(FrontendError::invalid_input(
                "source span boundary is not a UTF-8 character boundary",
            )
            .context("source_id", source_id.as_raw().to_string())
            .context("start", start.to_string())
            .context("end", end.to_string()));
        }

        // `start <= end` was checked above, so this construction cannot fail.
        Ok(SourceSpan::new(source_id, start, end)
            .expect("validated source span must be ordered"))
    }

    /// Validates an existing source span.
    pub fn validate_span(
        &self,
        span: SourceSpan,
    ) -> FrontendResult<()> {
        let _ = self.span(
            span.source_id(),
            span.start(),
            span.end(),
        )?;

        Ok(())
    }

    /// Converts a source position into one-based line/column coordinates.
    pub fn line_column(
        &self,
        position: SourcePosition,
    ) -> FrontendResult<LineColumn> {
        let source = self.require(position.source_id())?;

        source
            .line_column(position.offset())
            .ok_or_else(|| {
                FrontendError::invalid_input(
                    "source position is invalid for its source",
                )
                .context(
                    "source_id",
                    position.source_id().as_raw().to_string(),
                )
                .context(
                    "offset",
                    position.offset().to_string(),
                )
            })
    }

    /// Converts a span's start position into one-based line/column
    /// coordinates.
    pub fn span_start_line_column(
        &self,
        span: SourceSpan,
    ) -> FrontendResult<LineColumn> {
        self.line_column(span.start_position())
    }

    /// Converts a span's end position into one-based line/column coordinates.
    pub fn span_end_line_column(
        &self,
        span: SourceSpan,
    ) -> FrontendResult<LineColumn> {
        self.line_column(span.end_position())
    }

    /// Returns the exact source text represented by a span.
    pub fn text(
        &self,
        span: SourceSpan,
    ) -> FrontendResult<&str> {
        let source = self.require(span.source_id())?;

        source
            .slice_span(span)
            .ok_or_else(|| {
                FrontendError::invalid_input(
                    "source span is not a valid UTF-8 range",
                )
                .context(
                    "source_id",
                    span.source_id().as_raw().to_string(),
                )
                .context("start", span.start().to_string())
                .context("end", span.end().to_string())
            })
    }

    /// Returns the complete logical line containing a span's start.
    pub fn line_text(
        &self,
        span: SourceSpan,
    ) -> FrontendResult<&str> {
        let source = self.require(span.source_id())?;

        source
            .line_text(span.start())
            .ok_or_else(|| {
                FrontendError::invalid_input(
                    "source span start does not identify a valid line",
                )
                .context(
                    "source_id",
                    span.source_id().as_raw().to_string(),
                )
                .context("offset", span.start().to_string())
            })
    }
}

// =============================================================================
// Limit integration
// =============================================================================

/// Extension methods used by `source.rs` to consume the generic frontend
/// limits without exposing the limits implementation's internal fields.
///
/// These methods intentionally keep the actual `FrontendLimits` fields private
/// to `limits.rs`.
trait FrontendLimitsAccess {
    fn max_source_bytes(&self) -> u64;
    fn max_total_source_bytes(&self) -> u64;
    fn max_source_files(&self) -> u64;
}

impl FrontendLimitsAccess for FrontendLimits {
    fn max_source_bytes(&self) -> u64 {
        self.max_source_bytes()
    }

    fn max_total_source_bytes(&self) -> u64 {
        self.max_total_source_bytes()
    }

    fn max_source_files(&self) -> u64 {
        self.max_source_files()
    }
}

// =============================================================================
// Internal helpers
// =============================================================================

/// Computes the byte offset of every logical line start.
///
/// The first line always starts at byte zero.
///
/// Both Unix (`\n`) and Windows (`\r\n`) line endings are recognized. A
/// standalone `\r` is also treated as a line terminator so source diagnostics
/// remain useful for legacy/Mac-style text.
fn compute_line_starts(text: &str) -> Vec<usize> {
    let mut starts = Vec::with_capacity(
        text.as_bytes()
            .iter()
            .filter(|byte| **byte == b'\n')
            .count()
            .saturating_add(1),
    );

    starts.push(0);

    for (index, byte) in text.as_bytes().iter().enumerate() {
        if *byte == b'\n' {
            let next = index.saturating_add(1);

            if next <= text.len() {
                starts.push(next);
            }
        } else if *byte == b'\r' {
            let next = if text.as_bytes().get(index + 1) == Some(&b'\n') {
                index.saturating_add(2)
            } else {
                index.saturating_add(1)
            };

            if next <= text.len()
                && text.as_bytes().get(index + 1) != Some(&b'\n')
            {
                starts.push(next);
            }
        }
    }

    starts
}

/// Converts a u64 policy value into the largest representable `usize`.
///
/// This is used only for diagnostic metadata because the actual policy
/// comparisons remain in `u64`.
fn u64_to_usize_saturating(value: u64) -> usize {
    if value > usize::MAX as u64 {
        usize::MAX
    } else {
        value as usize
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn limits() -> FrontendLimits {
        FrontendLimits::production()
    }

    // -------------------------------------------------------------------------
    // SourceId
    // -------------------------------------------------------------------------

    #[test]
    fn source_id_is_stable_and_orderable() {
        let first = SourceId::from_raw(0);
        let second = SourceId::from_raw(1);

        assert_eq!(first.as_raw(), 0);
        assert_eq!(second.as_raw(), 1);
        assert!(first < second);
        assert_eq!(first.to_string(), "source:0");
    }

    // -------------------------------------------------------------------------
    // SourceSpan
    // -------------------------------------------------------------------------

    #[test]
    fn source_span_is_half_open() {
        let span = SourceSpan::new(SourceId::from_raw(7), 2, 8)
            .expect("ordered span");

        assert_eq!(span.start(), 2);
        assert_eq!(span.end(), 8);
        assert_eq!(span.len(), 6);
        assert!(!span.is_empty());
        assert_eq!(span.range(), 2..8);
    }

    #[test]
    fn reversed_span_is_rejected() {
        assert_eq!(
            SourceSpan::new(SourceId::from_raw(0), 8, 2),
            None
        );
    }

    #[test]
    fn point_span_is_empty() {
        let span = SourceSpan::point(SourceId::from_raw(0), 4);

        assert!(span.is_empty());
        assert_eq!(span.len(), 0);
        assert_eq!(span.start(), 4);
        assert_eq!(span.end(), 4);
    }

    #[test]
    fn span_join_requires_same_source() {
        let a = SourceSpan::new(SourceId::from_raw(0), 1, 4)
            .expect("valid span");
        let b = SourceSpan::new(SourceId::from_raw(0), 7, 9)
            .expect("valid span");
        let c = SourceSpan::new(SourceId::from_raw(1), 2, 5)
            .expect("valid span");

        assert_eq!(
            a.join(b),
            Some(
                SourceSpan::new(SourceId::from_raw(0), 1, 9)
                    .expect("valid span")
            )
        );

        assert_eq!(a.join(c), None);
    }

    #[test]
    fn span_overlap_is_deterministic() {
        let a = SourceSpan::new(SourceId::from_raw(0), 1, 5)
            .expect("valid span");
        let b = SourceSpan::new(SourceId::from_raw(0), 4, 8)
            .expect("valid span");
        let c = SourceSpan::new(SourceId::from_raw(0), 5, 8)
            .expect("valid span");

        assert!(a.overlaps(b));
        assert!(!a.overlaps(c));
    }

    // -------------------------------------------------------------------------
    // UTF-8 source handling
    // -------------------------------------------------------------------------

    #[test]
    fn source_preserves_utf8_text() {
        let source = SourceFile::new(
            SourceId::from_raw(0),
            "unicode.qasm",
            "H q[0]; π q[1];",
        );

        assert_eq!(source.text(), "H q[0]; π q[1];");
        assert!(source.is_char_boundary(8));
    }

    #[test]
    fn invalid_utf8_byte_boundary_is_rejected() {
        let source = SourceFile::new(
            SourceId::from_raw(0),
            "unicode",
            "π",
        );

        // `π` occupies two UTF-8 bytes, so offset 1 is inside the code point.
        assert!(!source.is_char_boundary(1));
        assert!(source.line_column(1).is_none());
        assert!(source.slice(0..1).is_none());
    }

    // -------------------------------------------------------------------------
    // Line indexing
    // -------------------------------------------------------------------------

    #[test]
    fn empty_source_has_one_logical_line() {
        let source = SourceFile::new(
            SourceId::from_raw(0),
            "empty",
            "",
        );

        assert_eq!(source.line_count(), 1);
        assert_eq!(
            source.line_column(0),
            Some(LineColumn::new(1, 1, 0))
        );
        assert_eq!(source.line_text(0), Some(""));
    }

    #[test]
    fn unix_lines_are_indexed_correctly() {
        let source = SourceFile::new(
            SourceId::from_raw(0),
            "test",
            "abc\ndef\nxyz",
        );

        assert_eq!(source.line_count(), 3);
        assert_eq!(
            source.line_column(0),
            Some(LineColumn::new(1, 1, 0))
        );
        assert_eq!(
            source.line_column(4),
            Some(LineColumn::new(2, 1, 4))
        );
        assert_eq!(
            source.line_column(8),
            Some(LineColumn::new(3, 1, 8))
        );

        assert_eq!(source.line_text(0), Some("abc"));
        assert_eq!(source.line_text(4), Some("def"));
        assert_eq!(source.line_text(8), Some("xyz"));
    }

    #[test]
    fn windows_lines_are_indexed_without_crlf_in_line_text() {
        let source = SourceFile::new(
            SourceId::from_raw(0),
            "test",
            "abc\r\ndef\r\nxyz",
        );

        assert_eq!(source.line_count(), 3);
        assert_eq!(source.line_text(0), Some("abc"));
        assert_eq!(source.line_text(5), Some("def"));
        assert_eq!(source.line_text(10), Some("xyz"));
    }

    #[test]
    fn standalone_carriage_returns_are_line_breaks() {
        let source = SourceFile::new(
            SourceId::from_raw(0),
            "legacy",
            "abc\rdef",
        );

        assert_eq!(source.line_count(), 2);
        assert_eq!(source.line_text(0), Some("abc"));
        assert_eq!(source.line_text(4), Some("def"));
    }

    #[test]
    fn unicode_columns_count_unicode_scalars_not_bytes() {
        let source = SourceFile::new(
            SourceId::from_raw(0),
            "unicode",
            "aπ中z",
        );

        // a=column 1, π=column 2, 中=column 3, z=column 4.
        let z_offset = "aπ中".len();

        assert_eq!(
            source.line_column(z_offset),
            Some(LineColumn::new(1, 4, z_offset))
        );
    }

    // -------------------------------------------------------------------------
    // SourceMap
    // -------------------------------------------------------------------------

    #[test]
    fn source_map_allocates_deterministic_ids() {
        let mut map = SourceMap::new();

        let first = map
            .add_source("first.qasm", "OPENQASM 3.1;")
            .expect("first source ID");

        let second = map
            .add_source("second.qasm", "OPENQASM 3.1;")
            .expect("second source ID");

        assert_eq!(first, SourceId::from_raw(0));
        assert_eq!(second, SourceId::from_raw(1));
        assert_eq!(map.len(), 2);
    }

    #[test]
    fn source_map_preserves_insertion_order() {
        let mut map = SourceMap::new();

        map.add_source("a", "A")
            .expect("source ID");
        map.add_source("b", "B")
            .expect("source ID");
        map.add_source("c", "C")
            .expect("source ID");

        let names: Vec<&str> = map
            .sources()
            .iter()
            .map(SourceFile::name)
            .collect();

        assert_eq!(names, vec!["a", "b", "c"]);
    }

    #[test]
    fn source_map_can_retrieve_sources_by_id() {
        let mut map = SourceMap::new();

        let id = map
            .add_source("program.qasm", "H q[0];")
            .expect("source ID");

        let source = map.get(id).expect("source exists");

        assert_eq!(source.id(), id);
        assert_eq!(source.name(), "program.qasm");
        assert_eq!(source.text(), "H q[0];");
    }

    #[test]
    fn missing_source_returns_structured_error() {
        let map = SourceMap::new();

        let error = map
            .require(SourceId::from_raw(99))
            .expect_err("source should not exist");

        assert_eq!(
            error.kind(),
            super::super::errors::FrontendErrorKind::InvalidInput
        );
    }

    // -------------------------------------------------------------------------
    // Validated positions and spans
    // -------------------------------------------------------------------------

    #[test]
    fn source_map_validates_positions() {
        let mut map = SourceMap::new();

        let id = map
            .add_source("test", "abc")
            .expect("source ID");

        assert!(map.position(id, 0).is_ok());
        assert!(map.position(id, 3).is_ok());
        assert!(map.position(id, 4).is_err());
    }

    #[test]
    fn source_map_validates_utf8_positions() {
        let mut map = SourceMap::new();

        let id = map
            .add_source("test", "π")
            .expect("source ID");

        assert!(map.position(id, 0).is_ok());
        assert!(map.position(id, 2).is_ok());

        // Offset 1 is inside π.
        assert!(map.position(id, 1).is_err());
    }

    #[test]
    fn source_map_validates_spans() {
        let mut map = SourceMap::new();

        let id = map
            .add_source("test", "abcdef")
            .expect("source ID");

        let span = map
            .span(id, 1, 4)
            .expect("valid span");

        assert_eq!(
            map.text(span).expect("span text"),
            "bcd"
        );
    }

    #[test]
    fn source_map_rejects_reversed_spans() {
        let mut map = SourceMap::new();

        let id = map
            .add_source("test", "abcdef")
            .expect("source ID");

        assert!(map.span(id, 5, 2).is_err());
    }

    #[test]
    fn source_map_rejects_span_beyond_source() {
        let mut map = SourceMap::new();

        let id = map
            .add_source("test", "abcdef")
            .expect("source ID");

        assert!(map.span(id, 1, 7).is_err());
    }

    #[test]
    fn source_map_rejects_span_inside_utf8_character() {
        let mut map = SourceMap::new();

        let id = map
            .add_source("test", "π")
            .expect("source ID");

        assert!(map.span(id, 1, 2).is_err());
    }

    #[test]
    fn source_map_resolves_span_locations() {
        let mut map = SourceMap::new();

        let id = map
            .add_source("test", "abc\ndef")
            .expect("source ID");

        let span = map
            .span(id, 4, 7)
            .expect("valid span");

        assert_eq!(
            map.span_start_line_column(span)
                .expect("line/column"),
            LineColumn::new(2, 1, 4)
        );

        assert_eq!(
            map.span_end_line_column(span)
                .expect("line/column"),
            LineColumn::new(2, 4, 7)
        );
    }

    #[test]
    fn source_text_is_available_from_span() {
        let mut map = SourceMap::new();

        let id = map
            .add_source("test", "OPENQASM 3.1;")
            .expect("source ID");

        let span = map
            .span(id, 0, 13)
            .expect("valid span");

        assert_eq!(
            map.text(span).expect("text"),
            "OPENQASM 3.1;"
        );
    }

    // -------------------------------------------------------------------------
    // Limits integration
    // -------------------------------------------------------------------------

    #[test]
    fn source_limit_rejects_oversized_source_before_insertion() {
        let mut map = SourceMap::new();

        let limits = FrontendLimits::strict();

        let oversized = "x".repeat(
            limits
                .max_source_bytes_for_tests()
                .saturating_add(1),
        );

        let result = map.add_source_with_limits(
            "large.qasm",
            oversized,
            &limits,
        );

        assert!(result.is_err());
        assert!(map.is_empty());
    }

    #[test]
    fn total_source_limit_is_enforced() {
        let mut map = SourceMap::new();

        let limits = FrontendLimits::strict();

        let per_file = limits
            .max_source_bytes_for_tests()
            .min(64);

        let first = "a".repeat(per_file);

        map.add_source_with_limits(
            "first",
            first,
            &limits,
        )
        .expect("first source");

        // This test uses a local reduced policy so the total boundary can be
        // exercised without allocating large strings.
        let reduced = FrontendLimitsTestBuilder::new()
            .max_source_bytes(64)
            .max_total_source_bytes(64)
            .max_source_files(4)
            .build();

        let mut reduced_map = SourceMap::new();

        reduced_map
            .add_source_with_limits(
                "first",
                "12345678901234567890123456789012",
                &reduced,
            )
            .expect("first source");

        let second = reduced_map.add_source_with_limits(
            "second",
            "12345678901234567890123456789012",
            &reduced,
        );

        assert!(second.is_err());
    }

    #[test]
    fn failed_limit_insert_does_not_mutate_map() {
        let limits = FrontendLimitsTestBuilder::new()
            .max_source_bytes(4)
            .max_total_source_bytes(8)
            .max_source_files(2)
            .build();

        let mut map = SourceMap::new();

        map.add_source_with_limits(
            "first",
            "1234",
            &limits,
        )
        .expect("first source");

        let result = map.add_source_with_limits(
            "too-large",
            "12345",
            &limits,
        );

        assert!(result.is_err());
        assert_eq!(map.len(), 1);
        assert_eq!(map.total_source_bytes(), 4);
    }

    // -------------------------------------------------------------------------
    // Determinism
    // -------------------------------------------------------------------------

    #[test]
    fn identical_sources_produce_identical_source_models() {
        let mut first = SourceMap::new();
        let mut second = SourceMap::new();

        first
            .add_source("program.qasm", "OPENQASM 3.1;\nqbit")
            .expect("source ID");

        second
            .add_source("program.qasm", "OPENQASM 3.1;\nqbit")
            .expect("source ID");

        assert_eq!(first.sources(), second.sources());
        assert_eq!(
            first.total_source_bytes(),
            second.total_source_bytes()
        );
    }

    // -------------------------------------------------------------------------
    // Display contracts
    // -------------------------------------------------------------------------

    #[test]
    fn positions_and_spans_have_deterministic_display() {
        let position = SourcePosition::new(
            SourceId::from_raw(3),
            42,
        );

        let span = SourceSpan::new(
            SourceId::from_raw(3),
            42,
            49,
        )
        .expect("valid span");

        assert_eq!(
            position.to_string(),
            "source:3@42"
        );

        assert_eq!(
            span.to_string(),
            "source:3:42..49"
        );
    }

    // -------------------------------------------------------------------------
    // Test-only limit builder
    // -------------------------------------------------------------------------
    //
    // The production FrontendLimits API intentionally remains immutable and
    // controlled. These helpers are compiled only for tests and avoid making
    // the public production configuration surface unnecessarily mutable.

    #[derive(Clone, Copy)]
    struct FrontendLimitsTestBuilder {
        max_source_bytes: u64,
        max_total_source_bytes: u64,
        max_source_files: u64,
    }

    impl FrontendLimitsTestBuilder {
        fn new() -> Self {
            Self {
                max_source_bytes: 1024,
                max_total_source_bytes: 4096,
                max_source_files: 16,
            }
        }

        fn max_source_bytes(mut self, value: u64) -> Self {
            self.max_source_bytes = value;
            self
        }

        fn max_total_source_bytes(mut self, value: u64) -> Self {
            self.max_total_source_bytes = value;
            self
        }

        fn max_source_files(mut self, value: u64) -> Self {
            self.max_source_files = value;
            self
        }

        fn build(self) -> FrontendLimits {
            FrontendLimits::for_source_tests(
                self.max_source_bytes,
                self.max_total_source_bytes,
                self.max_source_files,
            )
        }
    }
}

// =============================================================================
// Integration hooks for the existing limits module
// =============================================================================
//
// These methods are intentionally implemented in this module rather than
// making FrontendLimits' internal fields public. The production limits module
// therefore retains ownership of its representation while source.rs consumes
// a stable accessor contract.

#[cfg(test)]
mod limit_test_access {
    // Intentionally empty. The test-only constructors are supplied by the
    // limits module's `cfg(test)` implementation contract.
}

Important integration correction before committing this exact file: the current "limits.rs" keeps its fields private, so "source.rs" should consume public accessors rather than trying to access private fields. The existing file already has the limits model but the retrieved portion does not show its complete accessor API.

Therefore, to satisfy your “finish this file once and never re-edit it later” requirement, "source.rs" should be paired with the accessor contract already intended by "limits.rs": "max_source_bytes()", "max_total_source_bytes()", and "max_source_files()". If those accessors are not yet present in the remainder of "limits.rs", they must be finalized in "limits.rs" before this file is considered integrated. I would not make the source module reach into private fields.

There is also one test-support issue in the draft above: the test-only "FrontendLimits::for_source_tests(...)" accessor is not visible in the retrieved portion of "limits.rs". Rather than smuggling test construction into "source.rs", that helper belongs in "limits.rs"'s own "#[cfg(test)]" section.

Final integration contract

Once those existing "limits.rs" accessors/test constructor are present, "source.rs" has this clean dependency boundary:

frontend/core/source.rs
        │
        ├── frontend/core/errors.rs
        │      └── FrontendError
        │      └── FrontendLimitViolation
        │      └── FrontendResult
        │
        └── frontend/core/limits.rs
               └── FrontendLimits

and nothing else.

The Quantum IR remains untouched; that is correct because the IR explicitly excludes frontend parsing/source concerns and remains the canonical representation.

The resulting API is ready for the next frontend files to consume:

- "lexer.rs" → creates "SourceSpan"
- "parser.rs" → attaches "SourceSpan" to AST nodes
- "validation.rs" → reports spans through diagnostics
- "diagnostics.rs" → resolves spans through "SourceMap"
- "importer.rs" → owns a "SourceMap" for the complete import operation
- "include.rs" → adds resolved source documents to the same "SourceMap"

No OpenQASM-specific code is required in "source.rs", so adding or removing OpenQASM, QIR, Quil, or another format later will not require changing this file.