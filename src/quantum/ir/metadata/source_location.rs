//! Zamani Quantum IR — Source Locations
//!
//! Production-grade, hardware-independent source-coordinate metadata for the
//! canonical Zamani Quantum Intermediate Representation.
//!
//! # Architectural role
//!
//! `source_location.rs` owns source-coordinate information only.
//!
//! It answers:
//!
//! > Where in the originating source artifact did this IR object originate?
//!
//! It does NOT answer:
//!
//! - what the quantum operation means;
//! - which logical qubit is used;
//! - which physical qubit is selected;
//! - which hardware executes the operation;
//! - how the program is optimized;
//! - how operations are routed;
//! - how operations are scheduled;
//! - how a pulse is synthesized;
//! - how source syntax is parsed;
//! - how source files are loaded;
//! - how source content is stored;
//! - how source content is hashed;
//! - how source files are authenticated.
//!
//! Those responsibilities belong to their respective subsystems.
//!
//! # Architectural boundary
//!
//! The intended dependency direction is:
//!
//! ```text
//! Zamani source
//!      │
//!      ▼
//! frontend/parser
//!      │
//!      │ creates source coordinates
//!      ▼
//! source_location.rs
//!      │
//!      ▼
//! quantum::ir
//!      │
//!      ├── operation
//!      ├── gate
//!      ├── program
//!      ├── region
//!      ├── provenance
//!      └── diagnostics
//! ```
//!
//! Downstream transformations may preserve source locations, but this module
//! never depends on those transformations.
//!
//! # Universal-program principle
//!
//! Source locations are independent of quantum-machine size.
//!
//! This module therefore contains no:
//!
//! - qubit-count limit;
//! - operation-count limit;
//! - register-size limit;
//! - topology assumption;
//! - hardware assumption;
//! - vendor assumption;
//! - architecture-specific coordinate system.
//!
//! The same representation works for a one-line program and a very large
//! generated/distributed program, subject only to the finite resources of the
//! host process and explicitly configured compiler policies.
//!
//! # Coordinate model
//!
//! A source location may contain:
//!
//! - source-document identity;
//! - byte offsets;
//! - line/column coordinates;
//! - optional end coordinates;
//! - optional generated-source origin;
//! - optional macro/include expansion ancestry.
//!
//! Byte offsets are the canonical machine-readable coordinate.
//!
//! Line and column information is diagnostic metadata and must never be used as
//! the authoritative identity of a source span.
//!
//! # Why byte offsets are canonical
//!
//! Byte offsets:
//!
//! - are independent of terminal width;
//! - are independent of proportional fonts;
//! - are stable under the same source encoding;
//! - can address arbitrary UTF-8 source text;
//! - can be used directly by parsers and source maps;
//! - avoid architecture-dependent integer sizes.
//!
//! The canonical offset type is therefore `u64`, not `usize`.
//!
//! `usize` is deliberately not used for semantic source coordinates.
//!
//! # UTF-8
//!
//! Zamani source is expected to be represented as UTF-8 at the frontend
//! boundary.
//!
//! Byte offsets count UTF-8 bytes.
//!
//! Columns may be represented using either:
//!
//! - byte columns;
//! - Unicode scalar-value columns.
//!
//! This module explicitly identifies which coordinate unit is used so that
//! different frontends cannot silently disagree.
//!
//! Grapheme-cluster calculation is intentionally outside this module because
//! it requires Unicode segmentation policy and is a presentation concern.
//!
//! # Zero-based versus one-based coordinates
//!
//! Canonical byte offsets are zero-based.
//!
//! Human-facing line and column coordinates are one-based.
//!
//! Therefore:
//!
//! ```text
//! byte offset: 0, 1, 2, ...
//!
//! line:        1, 2, 3, ...
//! column:      1, 2, 3, ...
//! ```
//!
//! This avoids the common ambiguity where parser offsets and diagnostic
//! coordinates accidentally use different conventions.
//!
//! # Empty spans
//!
//! Empty spans are allowed.
//!
//! They are useful for:
//!
//! - insertion points;
//! - zero-width diagnostics;
//! - generated operations;
//! - EOF locations;
//! - missing-token recovery;
//! - source-map anchors.
//!
//! A span is empty when its start and end offsets are equal.
//!
//! End offsets are exclusive.
//!
//! Therefore:
//!
//! ```text
//! [start, end)
//! ```
//!
//! is the canonical span interval.
//!
//! # Generated code
//!
//! Quantum compilation can involve:
//!
//! - macros;
//! - templates;
//! - generated declarations;
//! - included source;
//! - transformed source;
//! - compiler-generated operations.
//!
//! A single source location therefore cannot always be represented by one
//! physical file coordinate.
//!
//! `SourceOrigin` provides a compact abstraction for identifying whether a
//! location refers to:
//!
//! - user source;
//! - generated source;
//! - included source;
//! - macro expansion;
//! - transformed source;
//! - an unknown/external origin.
//!
//! This module intentionally records origin information without implementing a
//! macro expander or source-map engine.
//!
//! # Source identity
//!
//! A source document is identified by a stable textual identifier rather than
//! by an operating-system-specific path type.
//!
//! This permits:
//!
//! - local files;
//! - virtual documents;
//! - in-memory buffers;
//! - URI-addressed sources;
//! - remote sources;
//! - generated modules;
//! - embedded source;
//! - distributed compilation.
//!
//! The identifier is opaque to this module.
//!
//! Examples include:
//!
//! ```text
//! file:///workspace/main.zm
//! memory://module/123
//! generated://frontend/module/7
//! stdlib://quantum/core
//! ```
//!
//! This module does not dereference or access those identifiers.
//!
//! # Security
//!
//! Source locations are metadata and MUST NOT be treated as trusted paths.
//!
//! In particular, this module does not:
//!
//! - open files;
//! - canonicalize filesystem paths;
//! - follow symlinks;
//! - access the network;
//! - execute commands;
//! - interpret URIs;
//! - read source content.
//!
//! A diagnostic renderer or source manager may perform those operations under
//! an appropriate security policy.
//!
//! # Determinism
//!
//! The semantic portion of a source location is deterministic when its source
//! identifier and coordinates are deterministic.
//!
//! This module does not store:
//!
//! - memory addresses;
//! - process IDs;
//! - wall-clock timestamps;
//! - thread IDs;
//! - filesystem metadata;
//! - host-specific compiler state.
//!
//! Therefore source locations can safely participate in deterministic IR
//! metadata and provenance.
//!
//! # Canonical ordering
//!
//! Source locations implement deterministic ordering:
//!
//! 1. source identifier;
//! 2. start byte offset;
//! 3. end byte offset;
//! 4. coordinate metadata;
//! 5. origin metadata.
//!
//! This ordering is intended for deterministic diagnostics and collections.
//!
//! # Serialization
//!
//! This file owns the in-memory source-location model.
//!
//! The canonical serialized representation remains owned by
//! `quantum::ir::serialization`.
//!
//! Serialization implementations MUST preserve:
//!
//! - source identifier;
//! - start offset;
//! - end offset;
//! - coordinate units;
//! - line/column values when present;
//! - origin information;
//! - expansion ancestry when present.
//!
//! No serializer may silently discard source information.
//!
//! # Hashing
//!
//! This file does not implement cryptographic hashing.
//!
//! Canonical semantic hashing belongs to `hash.rs`.
//!
//! If a source location participates in a canonical hash, its fields must be
//! encoded deterministically according to the hashing subsystem's canonical
//! encoding contract.
//!
//! # Versioning
//!
//! The source-location representation is part of the Quantum IR metadata
//! contract.
//!
//! It must not invent a second global IR version system.
//!
//! IR schema/version negotiation belongs to `identity.rs` and the serialization
//! compatibility layer.
//!
//! # Integration contract
//!
//! `metadata/mod.rs`
//!     Re-exports this module.
//!
//! `provenance.rs`
//!     May store source locations as transformation/source lineage metadata.
//!
//! `operation.rs`
//!     May attach a source location to an operation.
//!
//! `program.rs`
//!     May attach source locations to program-level declarations.
//!
//! `region.rs`
//!     May attach source locations to structured regions.
//!
//! `validation.rs`
//!     May use source locations to produce diagnostics.
//!
//! `quantum::frontend`
//!     Creates source locations while parsing/importing source.
//!
//! `quantum::ir::serialization`
//!     Serializes and deserializes the representation.
//!
//! `quantum::ir::hash`
//!     May incorporate source metadata when the selected hashing policy says
//!     source metadata is semantic.
//!
//! No dependency is required on `quantum::ir::qubit`.
//!
//! A source location is deliberately independent from logical and physical
//! qubit identity. If a higher-level metadata structure needs to associate a
//! source location with a qubit, that higher-level structure should reference
//! the canonical `quantum::ir::qubit::QubitId`.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe`.
//!
//! Only the Rust standard library is used.
//!
//! # Invariants
//!
//! - byte offsets are zero-based;
//! - end offsets are exclusive;
//! - end offset must be greater than or equal to start offset;
//! - human-facing line/column values are one-based when present;
//! - a source identifier is never interpreted as a filesystem path here;
//! - no coordinate uses `usize` as its semantic representation;
//! - no source location contains hardware-specific state;
//! - no source location owns source-file contents;
//! - no source location silently truncates coordinates.
//!
//! # Design principle
//!
//! This file is intentionally complete at its own abstraction boundary.
//!
//! Changes to:
//!
//! - gates;
//! - qubits;
//! - operations;
//! - routing;
//! - scheduling;
//! - hardware;
//! - QEC;
//! - simulation;
//! - pulse compilation;
//! - target backends
//!
//! do not require modifications to this module.
//!
//! -----------------------------------------------------------------------------
//! Implementation
//! -----------------------------------------------------------------------------

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::cmp::Ordering;
use std::fmt;

// =============================================================================
// Result
// =============================================================================

/// Result type returned by source-location constructors and validators.
pub type SourceLocationResult<T> = Result<T, SourceLocationError>;

// =============================================================================
// Coordinate unit
// =============================================================================

/// Unit used by a source column coordinate.
///
/// Byte offsets are always UTF-8 byte offsets and are independent of this
/// enum. The enum describes only the optional human-facing column coordinate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ColumnUnit {
    /// Column counts UTF-8 bytes.
    Utf8Byte,

    /// Column counts Unicode scalar values (`char` values in Rust).
    UnicodeScalar,
}

impl ColumnUnit {
    /// Returns the stable serialization identifier.
    #[must_use]
    pub const fn id(self) -> u8 {
        match self {
            Self::Utf8Byte => 1,
            Self::UnicodeScalar => 2,
        }
    }

    /// Returns the stable textual identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Utf8Byte => "utf8-byte",
            Self::UnicodeScalar => "unicode-scalar",
        }
    }
}

impl Default for ColumnUnit {
    fn default() -> Self {
        Self::Utf8Byte
    }
}

impl fmt::Display for ColumnUnit {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Source identifier
// =============================================================================

/// Opaque identifier for a source document.
///
/// The identifier is intentionally a string rather than `PathBuf`.
///
/// This permits local, virtual, generated, embedded and URI-addressed source
/// documents without making the IR depend on a particular filesystem.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SourceId(String);

impl SourceId {
    /// Creates a source identifier.
    ///
    /// The value is opaque. This type does not validate URI or filesystem-path
    /// syntax because interpretation belongs to the source-management layer.
    pub fn new(value: impl Into<String>) -> SourceLocationResult<Self> {
        let value = value.into();

        if value.is_empty() {
            return Err(SourceLocationError::EmptySourceId);
        }

        Ok(Self(value))
    }

    /// Returns the source identifier as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the identifier and returns its string representation.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for SourceId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for SourceId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl TryFrom<String> for SourceId {
    type Error = SourceLocationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<&str> for SourceId {
    type Error = SourceLocationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

// =============================================================================
// Byte offset
// =============================================================================

/// Zero-based UTF-8 byte offset into a source document.
///
/// The value is intentionally `u64`, rather than `usize`, so the semantic IR
/// representation does not depend on host pointer width.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ByteOffset(u64);

impl ByteOffset {
    /// The first byte of a source document.
    pub const ZERO: Self = Self(0);

    /// Creates an offset.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the numeric offset.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }

    /// Returns an offset advanced by `amount`.
    ///
    /// Returns `None` instead of wrapping when the addition exceeds the
    /// representable coordinate range.
    #[must_use]
    pub const fn checked_add(self, amount: u64) -> Option<Self> {
        match self.0.checked_add(amount) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }

    /// Returns the distance between two ordered offsets.
    ///
    /// Returns `None` if `end` precedes `self`.
    #[must_use]
    pub const fn checked_distance(self, end: Self) -> Option<u64> {
        end.0.checked_sub(self.0)
    }
}

impl Default for ByteOffset {
    fn default() -> Self {
        Self::ZERO
    }
}

impl From<u64> for ByteOffset {
    fn from(value: u64) -> Self {
        Self::new(value)
    }
}

impl From<ByteOffset> for u64 {
    fn from(value: ByteOffset) -> u64 {
        value.value()
    }
}

impl fmt::Display for ByteOffset {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

// =============================================================================
// Line
// =============================================================================

/// One-based human-facing source line.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Line(u64);

impl Line {
    /// The first source line.
    pub const FIRST: Self = Self(1);

    /// Creates a one-based line number.
///
/// `0` is rejected because line numbers are intentionally one-based.
    pub const fn new(value: u64) -> SourceLocationResult<Self> {
        if value == 0 {
            return Err(SourceLocationError::InvalidLine { value });
        }

        Ok(Self(value))
    }

    /// Returns the numeric line number.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }
}

impl Default for Line {
    fn default() -> Self {
        Self::FIRST
    }
}

impl TryFrom<u64> for Line {
    type Error = SourceLocationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl fmt::Display for Line {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

// =============================================================================
// Column
// =============================================================================

/// One-based human-facing source column.
///
/// The unit is specified separately by [`ColumnUnit`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Column(u64);

impl Column {
    /// The first source column.
    pub const FIRST: Self = Self(1);

    /// Creates a one-based column.
    pub const fn new(value: u64) -> SourceLocationResult<Self> {
        if value == 0 {
            return Err(SourceLocationError::InvalidColumn { value });
        }

        Ok(Self(value))
    }

    /// Returns the numeric column.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }
}

impl Default for Column {
    fn default() -> Self {
        Self::FIRST
    }
}

impl TryFrom<u64> for Column {
    type Error = SourceLocationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl fmt::Display for Column {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

// =============================================================================
// Source coordinate
// =============================================================================

/// A source coordinate describing one position in a source document.
///
/// The byte offset is authoritative. Line/column information is optional
/// diagnostic metadata.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SourceCoordinate {
    byte_offset: ByteOffset,
    line: Option<Line>,
    column: Option<Column>,
    column_unit: ColumnUnit,
}

impl SourceCoordinate {
    /// Creates a coordinate containing only the canonical byte offset.
    #[must_use]
    pub const fn from_byte_offset(byte_offset: ByteOffset) -> Self {
        Self {
            byte_offset,
            line: None,
            column: None,
            column_unit: ColumnUnit::Utf8Byte,
        }
    }

    /// Creates a coordinate containing byte offset and one-based
    /// line/column information.
    pub const fn new(
        byte_offset: ByteOffset,
        line: Line,
        column: Column,
        column_unit: ColumnUnit,
    ) -> Self {
        Self {
            byte_offset,
            line: Some(line),
            column: Some(column),
            column_unit,
        }
    }

    /// Returns the canonical byte offset.
    #[must_use]
    pub const fn byte_offset(self) -> ByteOffset {
        self.byte_offset
    }

    /// Returns the optional line.
    #[must_use]
    pub const fn line(self) -> Option<Line> {
        self.line
    }

    /// Returns the optional column.
    #[must_use]
    pub const fn column(self) -> Option<Column> {
        self.column
    }

    /// Returns the column unit.
    #[must_use]
    pub const fn column_unit(self) -> ColumnUnit {
        self.column_unit
    }

    /// Returns whether line information is available.
    #[must_use]
    pub const fn has_line_column(self) -> bool {
        self.line.is_some() && self.column.is_some()
    }
}

impl Default for SourceCoordinate {
    fn default() -> Self {
        Self::from_byte_offset(ByteOffset::ZERO)
    }
}

// =============================================================================
// Source span
// =============================================================================

/// Half-open source interval `[start, end)`.
///
/// The start and end byte offsets are the authoritative span coordinates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SourceSpan {
    start: SourceCoordinate,
    end: SourceCoordinate,
}

impl SourceSpan {
    /// Creates a source span.
    ///
    /// The end byte offset must not precede the start byte offset.
    pub const fn new(
        start: SourceCoordinate,
        end: SourceCoordinate,
    ) -> SourceLocationResult<Self> {
        if end.byte_offset().value() < start.byte_offset().value() {
            return Err(SourceLocationError::InvalidSpan {
                start: start.byte_offset(),
                end: end.byte_offset(),
            });
        }

        Ok(Self { start, end })
    }

    /// Creates a zero-width span at one coordinate.
    #[must_use]
    pub const fn point(coordinate: SourceCoordinate) -> Self {
        Self {
            start: coordinate,
            end: coordinate,
        }
    }

    /// Creates a span from byte offsets.
    pub const fn from_byte_offsets(
        start: ByteOffset,
        end: ByteOffset,
    ) -> SourceLocationResult<Self> {
        Self::new(
            SourceCoordinate::from_byte_offset(start),
            SourceCoordinate::from_byte_offset(end),
        )
    }

    /// Returns the start coordinate.
    #[must_use]
    pub const fn start(self) -> SourceCoordinate {
        self.start
    }

    /// Returns the end coordinate.
    #[must_use]
    pub const fn end(self) -> SourceCoordinate {
        self.end
    }

    /// Returns the start byte offset.
    #[must_use]
    pub const fn start_offset(self) -> ByteOffset {
        self.start.byte_offset()
    }

    /// Returns the exclusive end byte offset.
    #[must_use]
    pub const fn end_offset(self) -> ByteOffset {
        self.end.byte_offset()
    }

    /// Returns the byte length of this span.
    ///
    /// Since the span invariant guarantees `end >= start`, this operation
    /// cannot fail.
    #[must_use]
    pub const fn byte_length(self) -> u64 {
        self.end_offset().value() - self.start_offset().value()
    }

    /// Returns whether the span has zero byte length.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.start_offset().value() == self.end_offset().value()
    }

    /// Returns whether this span contains a byte offset.
    ///
    /// The end coordinate is exclusive.
    #[must_use]
    pub const fn contains_offset(self, offset: ByteOffset) -> bool {
        offset.value() >= self.start_offset().value()
            && offset.value() < self.end_offset().value()
    }

    /// Returns whether this span contains another span.
    #[must_use]
    pub const fn contains_span(self, other: Self) -> bool {
        other.start_offset().value() >= self.start_offset().value()
            && other.end_offset().value() <= self.end_offset().value()
    }

    /// Returns whether this span overlaps another span.
    ///
    /// Empty spans are considered non-overlapping.
    #[must_use]
    pub const fn overlaps(self, other: Self) -> bool {
        !self.is_empty()
            && !other.is_empty()
            && self.start_offset().value() < other.end_offset().value()
            && other.start_offset().value() < self.end_offset().value()
    }

    /// Returns the span covering both source spans.
    ///
    /// This requires the spans to refer to the same source document at the
    /// higher `SourceLocation` layer. At this level only coordinates are
    /// combined.
    pub const fn covering(self, other: Self) -> SourceLocationResult<Self> {
        let start = if self.start_offset().value() <= other.start_offset().value() {
            self.start
        } else {
            other.start
        };

        let end = if self.end_offset().value() >= other.end_offset().value() {
            self.end
        } else {
            other.end
        };

        Self::new(start, end)
    }
}

impl PartialOrd for SourceSpan {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SourceSpan {
    fn cmp(&self, other: &Self) -> Ordering {
        self.start_offset()
            .cmp(&other.start_offset())
            .then_with(|| self.end_offset().cmp(&other.end_offset()))
            .then_with(|| self.start.cmp(&other.start))
            .then_with(|| self.end.cmp(&other.end))
    }
}

// =============================================================================
// Source origin
// =============================================================================

/// Semantic origin of source text.
///
/// This describes how source text came to exist without implementing the
/// source-generation mechanism itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SourceOriginKind {
    /// Text directly authored by the user.
    User,

    /// Text generated by a compiler/frontend/tool.
    Generated,

    /// Text included from another source document.
    Included,

    /// Text produced through macro/template expansion.
    MacroExpansion,

    /// Text produced by a source-to-source transformation.
    Transformed,

    /// Origin is external or not known.
    Unknown,
}

impl SourceOriginKind {
    /// Stable serialization identifier.
    #[must_use]
    pub const fn id(self) -> u8 {
        match self {
            Self::User => 1,
            Self::Generated => 2,
            Self::Included => 3,
            Self::MacroExpansion => 4,
            Self::Transformed => 5,
            Self::Unknown => 255,
        }
    }

    /// Stable textual identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::User => "user",
            Self::Generated => "generated",
            Self::Included => "included",
            Self::MacroExpansion => "macro-expansion",
            Self::Transformed => "transformed",
            Self::Unknown => "unknown",
        }
    }
}

impl Default for SourceOriginKind {
    fn default() -> Self {
        Self::User
    }
}

impl fmt::Display for SourceOriginKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Source origin
// =============================================================================

/// Describes the origin of a source location.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SourceOrigin {
    kind: SourceOriginKind,
    parent: Option<Box<SourceLocation>>,
    description: Option<String>,
}

impl SourceOrigin {
    /// Creates an origin with no parent.
    #[must_use]
    pub const fn new(kind: SourceOriginKind) -> Self {
        Self {
            kind,
            parent: None,
            description: None,
        }
    }

    /// Creates an origin with a parent source location.
    ///
    /// Parent chains should normally be bounded by the compiler's explicit
    /// metadata/resource policy. This type itself imposes no architectural
    /// depth limit.
    #[must_use]
    pub fn with_parent(
        kind: SourceOriginKind,
        parent: SourceLocation,
    ) -> Self {
        Self {
            kind,
            parent: Some(Box::new(parent)),
            description: None,
        }
    }

    /// Adds an optional human-readable description.
    pub fn with_description(
        mut self,
        description: impl Into<String>,
    ) -> SourceLocationResult<Self> {
        let description = description.into();

        if description.is_empty() {
            return Err(SourceLocationError::EmptyDescription);
        }

        self.description = Some(description);
        Ok(self)
    }

    /// Returns the origin kind.
    #[must_use]
    pub const fn kind(&self) -> SourceOriginKind {
        self.kind
    }

    /// Returns the parent source location.
    #[must_use]
    pub fn parent(&self) -> Option<&SourceLocation> {
        self.parent.as_deref()
    }

    /// Returns the optional description.
    #[must_use]
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Returns whether this origin has an expansion/inclusion parent.
    #[must_use]
    pub fn has_parent(&self) -> bool {
        self.parent.is_some()
    }
}

// =============================================================================
// Source location
// =============================================================================

/// Complete source location associated with an IR object.
///
/// This is the primary type that other Quantum IR modules should store.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SourceLocation {
    source: SourceId,
    span: SourceSpan,
    origin: Option<SourceOrigin>,
}

impl SourceLocation {
    /// Creates a source location.
    pub fn new(
        source: SourceId,
        span: SourceSpan,
    ) -> Self {
        Self {
            source,
            span,
            origin: None,
        }
    }

    /// Creates a source location directly from a source identifier and byte
    /// offsets.
    pub fn from_byte_offsets(
        source: SourceId,
        start: u64,
        end: u64,
    ) -> SourceLocationResult<Self> {
        let span = SourceSpan::from_byte_offsets(
            ByteOffset::new(start),
            ByteOffset::new(end),
        )?;

        Ok(Self::new(source, span))
    }

    /// Creates a point location at a byte offset.
    pub fn point(
        source: SourceId,
        offset: u64,
    ) -> Self {
        Self::new(
            source,
            SourceSpan::point(SourceCoordinate::from_byte_offset(
                ByteOffset::new(offset),
            )),
        )
    }

    /// Adds source-origin metadata.
    #[must_use]
    pub fn with_origin(
        mut self,
        origin: SourceOrigin,
    ) -> Self {
        self.origin = Some(origin);
        self
    }

    /// Returns the source identifier.
    #[must_use]
    pub fn source(&self) -> &SourceId {
        &self.source
    }

    /// Returns the source span.
    #[must_use]
    pub const fn span(&self) -> SourceSpan {
        self.span
    }

    /// Returns the start byte offset.
    #[must_use]
    pub const fn start_offset(&self) -> ByteOffset {
        self.span.start_offset()
    }

    /// Returns the exclusive end byte offset.
    #[must_use]
    pub const fn end_offset(&self) -> ByteOffset {
        self.span.end_offset()
    }

    /// Returns the optional origin metadata.
    #[must_use]
    pub fn origin(&self) -> Option<&SourceOrigin> {
        self.origin.as_ref()
    }

    /// Returns whether this location has origin metadata.
    #[must_use]
    pub fn has_origin(&self) -> bool {
        self.origin.is_some()
    }

    /// Returns whether this location is zero-width.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.span.is_empty()
    }

    /// Returns the number of source bytes covered.
    #[must_use]
    pub const fn byte_length(&self) -> u64 {
        self.span.byte_length()
    }

    /// Returns whether the location refers to the same source document as
    /// another location.
    #[must_use]
    pub fn same_source(&self, other: &Self) -> bool {
        self.source == other.source
    }

    /// Returns whether this location contains another location.
    ///
    /// Locations from different source documents never contain one another.
    #[must_use]
    pub fn contains(&self, other: &Self) -> bool {
        self.same_source(other) && self.span.contains_span(other.span)
    }

    /// Returns whether this location overlaps another location.
    ///
    /// Locations from different source documents never overlap.
    #[must_use]
    pub fn overlaps(&self, other: &Self) -> bool {
        self.same_source(other) && self.span.overlaps(other.span)
    }

    /// Returns a location covering both locations.
    ///
    /// The locations must refer to the same source document.
    ///
    /// Origin metadata is intentionally not merged because combining origins
    /// without a source-map policy can create false provenance.
    pub fn covering(
        &self,
        other: &Self,
    ) -> SourceLocationResult<Self> {
        if !self.same_source(other) {
            return Err(SourceLocationError::DifferentSources {
                first: self.source.clone(),
                second: other.source.clone(),
            });
        }

        Ok(Self::new(
            self.source.clone(),
            self.span.covering(other.span)?,
        ))
    }
}

impl PartialOrd for SourceLocation {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SourceLocation {
    fn cmp(&self, other: &Self) -> Ordering {
        self.source
            .cmp(&other.source)
            .then_with(|| self.span.cmp(&other.span))
            .then_with(|| self.origin.cmp(&other.origin))
    }
}

impl fmt::Display for SourceLocation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}:{}..{}",
            self.source,
            self.start_offset(),
            self.end_offset()
        )
    }
}

// =============================================================================
// Source-location error
// =============================================================================

/// Errors produced by source-location construction and validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SourceLocationError {
    /// A source identifier was empty.
    EmptySourceId,

    /// A line number was zero even though lines are one-based.
    InvalidLine {
        /// Invalid line value.
        value: u64,
    },

    /// A column number was zero even though columns are one-based.
    InvalidColumn {
        /// Invalid column value.
        value: u64,
    },

    /// A span's end preceded its start.
    InvalidSpan {
        /// Start coordinate.
        start: ByteOffset,

        /// End coordinate.
        end: ByteOffset,
    },

    /// A source-location merge attempted to combine different source
    /// documents.
    DifferentSources {
        /// First source.
        first: SourceId,

        /// Second source.
        second: SourceId,
    },

    /// An origin description was empty.
    EmptyDescription,
}

impl fmt::Display for SourceLocationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptySourceId => {
                write!(formatter, "source identifier must not be empty")
            }

            Self::InvalidLine { value } => {
                write!(
                    formatter,
                    "source line must be one-based; received {value}"
                )
            }

            Self::InvalidColumn { value } => {
                write!(
                    formatter,
                    "source column must be one-based; received {value}"
                )
            }

            Self::InvalidSpan { start, end } => {
                write!(
                    formatter,
                    "invalid source span: end offset {end} precedes start offset {start}"
                )
            }

            Self::DifferentSources { first, second } => {
                write!(
                    formatter,
                    "cannot combine source locations from different sources: {first} and {second}"
                )
            }

            Self::EmptyDescription => {
                write!(
                    formatter,
                    "source-origin description must not be empty"
                )
            }
        }
    }
}

impl std::error::Error for SourceLocationError {}

// =============================================================================
// Convenience constructors
// =============================================================================

/// Creates a one-based source coordinate with UTF-8 byte columns.
pub fn coordinate(
    byte_offset: u64,
    line: u64,
    column: u64,
) -> SourceLocationResult<SourceCoordinate> {
    Ok(SourceCoordinate::new(
        ByteOffset::new(byte_offset),
        Line::new(line)?,
        Column::new(column)?,
        ColumnUnit::Utf8Byte,
    ))
}

/// Creates a one-based source coordinate with Unicode scalar columns.
pub fn unicode_coordinate(
    byte_offset: u64,
    line: u64,
    column: u64,
) -> SourceLocationResult<SourceCoordinate> {
    Ok(SourceCoordinate::new(
        ByteOffset::new(byte_offset),
        Line::new(line)?,
        Column::new(column)?,
        ColumnUnit::UnicodeScalar,
    ))
}

/// Creates a source location from a source identifier and byte offsets.
pub fn location(
    source: SourceId,
    start: u64,
    end: u64,
) -> SourceLocationResult<SourceLocation> {
    SourceLocation::from_byte_offsets(source, start, end)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn source() -> SourceId {
        SourceId::new("memory://test/main.zm")
            .expect("test source identifier must be valid")
    }

    #[test]
    fn source_id_rejects_empty_values() {
        assert_eq!(
            SourceId::new(""),
            Err(SourceLocationError::EmptySourceId)
        );
    }

    #[test]
    fn offsets_are_stable_u64_values() {
        let offset = ByteOffset::new(u64::MAX);

        assert_eq!(offset.value(), u64::MAX);
        assert_eq!(
            offset.checked_add(1),
            None
        );
    }

    #[test]
    fn line_and_column_are_one_based() {
        assert_eq!(
            Line::new(0),
            Err(SourceLocationError::InvalidLine { value: 0 })
        );

        assert_eq!(
            Column::new(0),
            Err(SourceLocationError::InvalidColumn { value: 0 })
        );

        assert_eq!(Line::new(1).unwrap().value(), 1);
        assert_eq!(Column::new(1).unwrap().value(), 1);
    }

    #[test]
    fn span_is_half_open() {
        let span = SourceSpan::from_byte_offsets(
            ByteOffset::new(10),
            ByteOffset::new(20),
        )
        .unwrap();

        assert_eq!(span.byte_length(), 10);
        assert!(span.contains_offset(ByteOffset::new(10)));
        assert!(span.contains_offset(ByteOffset::new(19)));
        assert!(!span.contains_offset(ByteOffset::new(20)));
    }

    #[test]
    fn invalid_span_is_rejected() {
        let result = SourceSpan::from_byte_offsets(
            ByteOffset::new(20),
            ByteOffset::new(10),
        );

        assert_eq!(
            result,
            Err(SourceLocationError::InvalidSpan {
                start: ByteOffset::new(20),
                end: ByteOffset::new(10),
            })
        );
    }

    #[test]
    fn empty_span_is_valid() {
        let span = SourceSpan::point(
            SourceCoordinate::from_byte_offset(
                ByteOffset::new(42),
            ),
        );

        assert!(span.is_empty());
        assert_eq!(span.byte_length(), 0);
    }

    #[test]
    fn source_locations_compare_source_first() {
        let first = SourceLocation::from_byte_offsets(
            SourceId::new("memory://a").unwrap(),
            0,
            10,
        )
        .unwrap();

        let second = SourceLocation::from_byte_offsets(
            SourceId::new("memory://b").unwrap(),
            0,
            10,
        )
        .unwrap();

        assert!(first < second);
    }

    #[test]
    fn source_locations_detect_different_sources() {
        let first = SourceLocation::from_byte_offsets(
            SourceId::new("memory://a").unwrap(),
            0,
            10,
        )
        .unwrap();

        let second = SourceLocation::from_byte_offsets(
            SourceId::new("memory://b").unwrap(),
            5,
            15,
        )
        .unwrap();

        assert!(!first.overlaps(&second));

        assert_eq!(
            first.covering(&second),
            Err(SourceLocationError::DifferentSources {
                first: SourceId::new("memory://a").unwrap(),
                second: SourceId::new("memory://b").unwrap(),
            })
        );
    }

    #[test]
    fn locations_can_be_covered() {
        let first = SourceLocation::from_byte_offsets(
            source(),
            10,
            20,
        )
        .unwrap();

        let second = SourceLocation::from_byte_offsets(
            source(),
            15,
            30,
        )
        .unwrap();

        let covering = first.covering(&second).unwrap();

        assert_eq!(covering.start_offset().value(), 10);
        assert_eq!(covering.end_offset().value(), 30);
    }

    #[test]
    fn origin_can_reference_parent_location() {
        let parent = SourceLocation::from_byte_offsets(
            source(),
            100,
            120,
        )
        .unwrap();

        let origin = SourceOrigin::with_parent(
            SourceOriginKind::MacroExpansion,
            parent.clone(),
        );

        assert_eq!(
            origin.kind(),
            SourceOriginKind::MacroExpansion
        );

        assert_eq!(
            origin.parent(),
            Some(&parent)
        );
    }

    #[test]
    fn coordinate_records_utf8_byte_columns() {
        let coordinate = coordinate(12, 2, 4).unwrap();

        assert_eq!(
            coordinate.byte_offset().value(),
            12
        );

        assert_eq!(
            coordinate.line().unwrap().value(),
            2
        );

        assert_eq!(
            coordinate.column().unwrap().value(),
            4
        );

        assert_eq!(
            coordinate.column_unit(),
            ColumnUnit::Utf8Byte
        );
    }

    #[test]
    fn coordinate_records_unicode_scalar_columns() {
        let coordinate = unicode_coordinate(12, 2, 4).unwrap();

        assert_eq!(
            coordinate.column_unit(),
            ColumnUnit::UnicodeScalar
        );
    }

    #[test]
    fn location_display_is_deterministic() {
        let value = SourceLocation::from_byte_offsets(
            SourceId::new("memory://main.zm").unwrap(),
            4,
            9,
        )
        .unwrap();

        assert_eq!(
            value.to_string(),
            "memory://main.zm:4..9"
        );
    }

    #[test]
    fn origin_description_must_not_be_empty() {
        let result = SourceOrigin::new(
            SourceOriginKind::Generated,
        )
        .with_description("");

        assert_eq!(
            result,
            Err(SourceLocationError::EmptyDescription)
        );
    }

    #[test]
    fn maximum_offsets_are_supported_without_overflow() {
        let span = SourceSpan::from_byte_offsets(
            ByteOffset::new(u64::MAX - 1),
            ByteOffset::new(u64::MAX),
        )
        .unwrap();

        assert_eq!(span.byte_length(), 1);
    }

    #[test]
    fn source_location_is_hashable_and_orderable() {
        use std::collections::{BTreeSet, HashSet};

        let first = SourceLocation::from_byte_offsets(
            source(),
            0,
            1,
        )
        .unwrap();

        let second = SourceLocation::from_byte_offsets(
            source(),
            2,
            3,
        )
        .unwrap();

        let mut ordered = BTreeSet::new();
        ordered.insert(first.clone());
        ordered.insert(second.clone());

        let mut hashed = HashSet::new();
        hashed.insert(first);
        hashed.insert(second);

        assert_eq!(ordered.len(), 2);
        assert_eq!(hashed.len(), 2);
    }
}