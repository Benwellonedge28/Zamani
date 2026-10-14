
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR Source Map and Source Provenance
//!
//! Path: `src/zuir/source_map.rs`
//!
//! ## PURPOSE
//!
//! Track source documents and the source origins associated with ZUIR entities.
//! Support source written by users, imported artifacts, generated code,
//! included sources, macro expansions, and transformed IR.
//!
//! ## AUTHORITY
//!
//! This module conforms to:
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/ids.rs`
//! - `grammar/DESIGN.md`
//! - `grammar/spec/lexical.md`
//! - `grammar/spec/syntax.md`
//! - `grammar/spec/diagnostics.md`
//! - `grammar/specification/semantic-model.md`
//!
//! ## OWNS
//!
//! - Source-document descriptors.
//! - Canonical byte-based source spans.
//! - Optional human-facing source coordinates.
//! - Typed associations between ZUIR entities and source origins.
//! - Validation of source references and source-map consistency.
//!
//! ## DOES NOT OWN
//!
//! - Source loading, file access, network access, or URI interpretation.
//! - Lexing, parsing, semantic analysis, or name resolution.
//! - Diagnostic rendering.
//! - AST or IR entity definitions and ID allocation.
//! - Quantum-specific source-location types.
//! - Serialization formats, migration, or content hashing.
//! - Macro expansion or transformation execution.
//! - Hardware, scheduling, routing, or runtime behavior.
//!
//! ## COORDINATE CONTRACT
//!
//! Byte offsets are zero-based UTF-8 byte offsets. End offsets are exclusive.
//! Human-facing line and column coordinates are one-based. Empty spans are
//! valid. Source identifiers are opaque typed IDs, not filesystem paths.
//!
//! ## DETERMINISM AND SCALABILITY
//!
//! Ordered collections provide deterministic iteration. Associations are
//! dynamically sized. No machine, hardware, source-count, or IR-entity ceiling
//! is defined here. Available memory, address space, and caller policy remain
//! practical limits.
//!
//! ## DEPENDENCY CONTRACT
//!
//! Depends only on the standard library and canonical IDs from `ids.rs`.
//! Does not depend on `errors.rs`, diagnostics, IR node implementations,
//! builders, verifiers, quantum IR, or backends.
//!
//! ## INTEGRATION CONTRACT
//!
//! Frontend adapters register source documents and create spans.
//! IR construction and lowering attach origins to typed entities.
//! Diagnostics query the map to locate relevant source text.
//! Transformations retain, combine, or explicitly omit origins.
//! Serialization must use a validated wire representation and must preserve
//! all source-map information it claims to support.
//!
//! This module is metadata: it never changes the meaning of an IR operation.

use core::fmt;
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;

use crate::zuir::ids::{
    BlockId, DialectId, ExtensionId, FunctionId, ModuleId, OperationId, ProgramId,
    RegionId, SourceId, SymbolId, TypeId, ValueId,
};

/// Result type for source-map operations.
pub type SourceMapResult<T> = Result<T, SourceMapError>;

// =============================================================================
// Source document
// =============================================================================

/// Describes a source artifact without loading or interpreting its contents.
///
/// `uri` is an opaque stable identifier chosen by the source-management
/// layer. It may represent a file, virtual document, generated artifact,
/// embedded source, or another supported source scheme.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SourceDocument {
    uri: String,
    kind: SourceDocumentKind,
    byte_length: Option<u64>,
}

impl SourceDocument {
    /// Creates a source descriptor.
    ///
    /// `byte_length`, when known, is the source's length in bytes, not
    /// characters or display columns.
    pub fn new(
        uri: impl Into<String>,
        kind: SourceDocumentKind,
        byte_length: Option<u64>,
    ) -> SourceMapResult<Self> {
        let uri = uri.into();

        if uri.trim().is_empty() {
            return Err(SourceMapError::EmptySourceUri);
        }

        Ok(Self {
            uri,
            kind,
            byte_length,
        })
    }

    /// Returns the opaque source identifier string.
    #[must_use]
    pub fn uri(&self) -> &str {
        &self.uri
    }

    /// Returns the source-artifact category.
    #[must_use]
    pub const fn kind(&self) -> SourceDocumentKind {
        self.kind
    }

    /// Returns the source length in bytes when known.
    #[must_use]
    pub const fn byte_length(&self) -> Option<u64> {
        self.byte_length
    }
}

/// Broad category of a source artifact.
///
/// This is descriptive metadata. It does not determine language semantics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SourceDocumentKind {
    /// Source authored directly by a user.
    User,

    /// Source imported from another supported format or language.
    Imported,

    /// Source generated by a compiler or other trusted tool.
    Generated,

    /// Source included from another source artifact.
    Included,

    /// A virtual or in-memory source document.
    Virtual,

    /// Source whose category is not known.
    Unknown,
}

// =============================================================================
// Coordinates and spans
// =============================================================================

/// One-based human-facing source position.
///
/// Byte offsets remain the canonical machine-readable coordinates.
/// The column unit must be declared explicitly so Unicode-aware frontends
/// cannot silently disagree about what a column means.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SourcePosition {
    line: u64,
    column: u64,
    column_unit: ColumnUnit,
}

impl SourcePosition {
    /// Creates a one-based source position.
    pub fn new(
        line: u64,
        column: u64,
        column_unit: ColumnUnit,
    ) -> SourceMapResult<Self> {
        if line == 0 {
            return Err(SourceMapError::InvalidCoordinate {
                field: "line",
                value: line,
            });
        }

        if column == 0 {
            return Err(SourceMapError::InvalidCoordinate {
                field: "column",
                value: column,
            });
        }

        Ok(Self {
            line,
            column,
            column_unit,
        })
    }

    /// Returns the one-based line.
    #[must_use]
    pub const fn line(self) -> u64 {
        self.line
    }

    /// Returns the one-based column.
    #[must_use]
    pub const fn column(self) -> u64 {
        self.column
    }

    /// Returns the unit used for the column.
    #[must_use]
    pub const fn column_unit(self) -> ColumnUnit {
        self.column_unit
    }
}

/// Coordinate unit for optional human-facing columns.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ColumnUnit {
    /// Column counts UTF-8 bytes.
    Utf8Byte,

    /// Column counts Unicode scalar values.
    UnicodeScalar,
}

/// Half-open byte span `[start_byte, end_byte)` within a source document.
///
/// The span itself does not imply that source text is present in memory.
/// When the source document's byte length is known, the map validates that
/// the span does not extend beyond that length.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SourceSpan {
    source_id: SourceId,
    start_byte: u64,
    end_byte: u64,
    start_position: Option<SourcePosition>,
    end_position: Option<SourcePosition>,
}

impl SourceSpan {
    /// Creates a validated half-open source span.
    ///
    /// Empty spans are valid and useful for insertion points, generated
    /// entities, EOF positions, and recovery anchors.
    pub fn new(
        source_id: SourceId,
        start_byte: u64,
        end_byte: u64,
        start_position: Option<SourcePosition>,
        end_position: Option<SourcePosition>,
    ) -> SourceMapResult<Self> {
        if end_byte < start_byte {
            return Err(SourceMapError::InvalidSpan {
                start_byte,
                end_byte,
            });
        }

        if let (Some(start), Some(end)) = (start_position, end_position) {
            if end < start {
                return Err(SourceMapError::InvalidPositionOrder);
            }
        }

        Ok(Self {
            source_id,
            start_byte,
            end_byte,
            start_position,
            end_position,
        })
    }

    /// Returns the source-document identity.
    #[must_use]
    pub const fn source_id(&self) -> SourceId {
        self.source_id
    }

    /// Returns the zero-based start byte.
    #[must_use]
    pub const fn start_byte(&self) -> u64 {
        self.start_byte
    }

    /// Returns the exclusive end byte.
    #[must_use]
    pub const fn end_byte(&self) -> u64 {
        self.end_byte
    }

    /// Returns the span length, which is safe because construction validates
    /// the ordering of the endpoints.
    #[must_use]
    pub const fn byte_length(&self) -> u64 {
        self.end_byte - self.start_byte
    }

    /// Returns true for a zero-width span.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.start_byte == self.end_byte
    }

    /// Returns the optional start line and column.
    #[must_use]
    pub const fn start_position(&self) -> Option<SourcePosition> {
        self.start_position
    }

    /// Returns the optional end line and column.
    #[must_use]
    pub const fn end_position(&self) -> Option<SourcePosition> {
        self.end_position
    }
}

// =============================================================================
// Entity identity
// =============================================================================

/// A typed ZUIR entity to which source origins may be attached.
///
/// These are references to canonical IDs, not new entity definitions.
/// Quantum-specific identities remain owned by `quantum::ir`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SourceEntity {
    Program(ProgramId),
    Module(ModuleId),
    Function(FunctionId),
    Region(RegionId),
    Block(BlockId),
    Operation(OperationId),
    Value(ValueId),
    Type(TypeId),
    Dialect(DialectId),
    Symbol(SymbolId),
    Extension(ExtensionId),
}

// =============================================================================
// Origin associations
// =============================================================================

/// Describes why an entity is associated with a source span.
///
/// A single generated IR entity can have multiple origins. For example,
/// an operation created by a macro expansion may relate to both the expansion
/// site and the macro definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OriginKind {
    /// Directly authored source.
    Direct,

    /// Source introduced by a generator.
    Generated,

    /// Source incorporated from another artifact.
    Included,

    /// Source related to macro expansion.
    MacroExpansion,

    /// Source imported from another representation.
    Imported,

    /// Source related to a transformation.
    Transformed,

    /// Origin category cannot be determined.
    Unknown,
}

/// A single origin record for an entity.
///
/// `primary_span` is the preferred source location, if one exists.
/// `related_spans` retain other contributing locations. The latter are sorted
/// and deduplicated by the constructor to make records deterministic.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SourceOrigin {
    kind: OriginKind,
    primary_span: Option<SourceSpan>,
    related_spans: Vec<SourceSpan>,
    label: Option<String>,
}

impl SourceOrigin {
    /// Creates an origin record.
    pub fn new(
        kind: OriginKind,
        primary_span: Option<SourceSpan>,
        mut related_spans: Vec<SourceSpan>,
        label: Option<String>,
    ) -> Self {
        related_spans.sort();
        related_spans.dedup();

        Self {
            kind,
            primary_span,
            related_spans,
            label,
        }
    }

    /// Returns the origin category.
    #[must_use]
    pub const fn kind(&self) -> OriginKind {
        self.kind
    }

    /// Returns the preferred source span, if available.
    #[must_use]
    pub fn primary_span(&self) -> Option<&SourceSpan> {
        self.primary_span.as_ref()
    }

    /// Returns additional contributing spans in deterministic order.
    #[must_use]
    pub fn related_spans(&self) -> &[SourceSpan] {
        &self.related_spans
    }

    /// Returns the optional explanatory label.
    #[must_use]
    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }
}

// =============================================================================
// Errors
// =============================================================================

/// Error returned by source-map construction, insertion, and validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SourceMapError {
    EmptySourceUri,
    DuplicateSource(SourceId),
    UnknownSource(SourceId),
    InvalidSpan {
        start_byte: u64,
        end_byte: u64,
    },
    SpanOutOfBounds {
        source_id: SourceId,
        end_byte: u64,
        source_length: u64,
    },
    InvalidCoordinate {
        field: &'static str,
        value: u64,
    },
    InvalidPositionOrder,
    InvalidSourceReference(SourceId),
}

impl fmt::Display for SourceMapError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptySourceUri => {
                f.write_str("source URI or identifier must not be empty")
            }
            Self::DuplicateSource(id) => {
                write!(f, "source ID {id} is already registered")
            }
            Self::UnknownSource(id) => {
                write!(f, "source ID {id} is not registered")
            }
            Self::InvalidSpan {
                start_byte,
                end_byte,
            } => write!(
                f,
                "invalid source span: end byte {end_byte} precedes start byte {start_byte}"
            ),
            Self::SpanOutOfBounds {
                source_id,
                end_byte,
                source_length,
            } => write!(
                f,
                "source span for {source_id} ends at byte {end_byte}, \
                 beyond the known source length {source_length}"
            ),
            Self::InvalidCoordinate { field, value } => {
                write!(f, "source {field} must be one-based; received {value}")
            }
            Self::InvalidPositionOrder => {
                f.write_str("source end position precedes the start position")
            }
            Self::InvalidSourceReference(id) => {
                write!(f, "source association references unknown source ID {id}")
            }
        }
    }
}

impl Error for SourceMapError {}

// =============================================================================
// Source map
// =============================================================================

/// Deterministic source registry and entity-to-origin association map.
///
/// This type intentionally does not implement a wire format. The canonical
/// ZUIR serialization layer owns encoding, decoding, schema versioning, and
/// migration. That layer must validate deserialized data before accepting it.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SourceMap {
    sources: BTreeMap<SourceId, SourceDocument>,
    origins: BTreeMap<SourceEntity, BTreeSet<SourceOrigin>>,
}

impl SourceMap {
    /// Creates an empty source map.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a source document under an existing typed source ID.
    ///
    /// Duplicate registration is an error. This avoids silently replacing
    /// document identity or invalidating existing spans.
    pub fn register_source(
        &mut self,
        id: SourceId,
        document: SourceDocument,
    ) -> SourceMapResult<()> {
        if self.sources.contains_key(&id) {
            return Err(SourceMapError::DuplicateSource(id));
        }

        self.sources.insert(id, document);
        Ok(())
    }

    /// Looks up a registered source document.
    #[must_use]
    pub fn source(&self, id: SourceId) -> Option<&SourceDocument> {
        self.sources.get(&id)
    }

    /// Iterates over registered sources in deterministic ID order.
    pub fn sources(
        &self,
    ) -> impl Iterator<Item = (SourceId, &SourceDocument)> {
        self.sources.iter().map(|(id, document)| (*id, document))
    }

    /// Returns the number of registered source documents.
    #[must_use]
    pub fn source_count(&self) -> usize {
        self.sources.len()
    }

    /// Returns the number of entities with at least one source origin.
    #[must_use]
    pub fn entity_count(&self) -> usize {
        self.origins.len()
    }

    /// Associates a source origin with a typed entity.
    ///
    /// Returns `true` if a new association was inserted and `false` if the
    /// exact same association was already present. Repeated insertion is
    /// therefore deterministic and idempotent.
    pub fn add_origin(
        &mut self,
        entity: SourceEntity,
        origin: SourceOrigin,
    ) -> SourceMapResult<bool> {
        self.validate_origin(&origin)?;

        Ok(self
            .origins
            .entry(entity)
            .or_default()
            .insert(origin))
    }

    /// Returns all origins for an entity in deterministic order.
    #[must_use]
    pub fn origins_for(
        &self,
        entity: SourceEntity,
    ) -> Option<&BTreeSet<SourceOrigin>> {
        self.origins.get(&entity)
    }

    /// Returns the primary source span for an entity, if any.
    ///
    /// If multiple origin records have primary spans, the deterministic first
    /// record is returned. Call `origins_for` to inspect all origins.
    #[must_use]
    pub fn primary_span_for(
        &self,
        entity: SourceEntity,
    ) -> Option<&SourceSpan> {
        self.origins
            .get(&entity)?
            .iter()
            .find_map(SourceOrigin::primary_span)
    }

    /// Returns whether the map has no registered sources or associations.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.sources.is_empty() && self.origins.is_empty()
    }

    /// Validates every source association in the map.
    ///
    /// This is useful after importing data from another IR or after a
    /// deserializer has reconstructed the map through a trusted adapter.
    pub fn validate(&self) -> SourceMapResult<()> {
        for origin_set in self.origins.values() {
            for origin in origin_set {
                self.validate_origin(origin)?;
            }
        }

        Ok(())
    }

    fn validate_origin(&self, origin: &SourceOrigin) -> SourceMapResult<()> {
        if let Some(span) = origin.primary_span() {
            self.validate_span(span)?;
        }

        for span in origin.related_spans() {
            self.validate_span(span)?;
        }

        Ok(())
    }

    fn validate_span(&self, span: &SourceSpan) -> SourceMapResult<()> {
        let document = self
            .sources
            .get(&span.source_id())
            .ok_or_else(|| SourceMapError::InvalidSourceReference(span.source_id()))?;

        if let Some(source_length) = document.byte_length() {
            if span.end_byte() > source_length {
                return Err(SourceMapError::SpanOutOfBounds {
                    source_id: span.source_id(),
                    end_byte: span.end_byte(),
                    source_length,
                });
            }
        }

        Ok(())
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn source_id(value: u128) -> SourceId {
        SourceId::new(value).expect("test source ID must be nonzero")
    }

    fn operation_id(value: u128) -> OperationId {
        OperationId::new(value).expect("test operation ID must be nonzero")
    }

    fn document(length: Option<u64>) -> SourceDocument {
        SourceDocument::new(
            "memory://tests/main.zm",
            SourceDocumentKind::User,
            length,
        )
        .expect("valid test source")
    }

    fn span(source: SourceId, start: u64, end: u64) -> SourceSpan {
        SourceSpan::new(source, start, end, None, None)
            .expect("valid test span")
    }

    #[test]
    fn registers_and_retrieves_source() {
        let id = source_id(1);
        let mut map = SourceMap::new();

        map.register_source(id, document(Some(32))).unwrap();

        assert_eq!(map.source_count(), 1);
        assert_eq!(map.source(id).unwrap().byte_length(), Some(32));
    }

    #[test]
    fn rejects_duplicate_source_registration() {
        let id = source_id(1);
        let mut map = SourceMap::new();

        map.register_source(id, document(Some(32))).unwrap();

        assert_eq!(
            map.register_source(id, document(Some(32))),
            Err(SourceMapError::DuplicateSource(id))
        );
    }

    #[test]
    fn accepts_empty_spans() {
        let id = source_id(1);
        let empty = span(id, 7, 7);

        assert!(empty.is_empty());
        assert_eq!(empty.byte_length(), 0);
    }

    #[test]
    fn rejects_reversed_spans() {
        let result = SourceSpan::new(source_id(1), 9, 4, None, None);

        assert!(matches!(
            result,
            Err(SourceMapError::InvalidSpan {
                start_byte: 9,
                end_byte: 4
            })
        ));
    }

    #[test]
    fn rejects_zero_based_human_coordinates() {
        assert!(matches!(
            SourcePosition::new(0, 1, ColumnUnit::Utf8Byte),
            Err(SourceMapError::InvalidCoordinate {
                field: "line",
                value: 0
            })
        ));

        assert!(matches!(
            SourcePosition::new(1, 0, ColumnUnit::Utf8Byte),
            Err(SourceMapError::InvalidCoordinate {
                field: "column",
                value: 0
            })
        ));
    }

    #[test]
    fn rejects_span_beyond_known_source_length() {
        let id = source_id(1);
        let mut map = SourceMap::new();
        map.register_source(id, document(Some(10))).unwrap();

        let origin = SourceOrigin::new(
            OriginKind::Direct,
            Some(span(id, 4, 11)),
            Vec::new(),
            None,
        );

        assert!(matches!(
            map.add_origin(SourceEntity::Operation(operation_id(1)), origin),
            Err(SourceMapError::SpanOutOfBounds {
                end_byte: 11,
                source_length: 10,
                ..
            })
        ));
    }

    #[test]
    fn rejects_origin_from_unregistered_source() {
        let mut map = SourceMap::new();
        let missing = source_id(99);

        let origin = SourceOrigin::new(
            OriginKind::Direct,
            Some(span(missing, 0, 1)),
            Vec::new(),
            None,
        );

        assert!(matches!(
            map.add_origin(SourceEntity::Operation(operation_id(1)), origin),
            Err(SourceMapError::InvalidSourceReference(id)) if id == missing
        ));
    }

    #[test]
    fn supports_multiple_origins_for_one_entity() {
        let id = source_id(1);
        let mut map = SourceMap::new();
        map.register_source(id, document(Some(100))).unwrap();

        let entity = SourceEntity::Operation(operation_id(1));
        let first = SourceOrigin::new(
            OriginKind::Direct,
            Some(span(id, 0, 5)),
            Vec::new(),
            Some("source expression".to_owned()),
        );
        let second = SourceOrigin::new(
            OriginKind::MacroExpansion,
            Some(span(id, 20, 30)),
            vec![span(id, 40, 50)],
            Some("macro definition".to_owned()),
        );

        assert!(map.add_origin(entity, first).unwrap());
        assert!(map.add_origin(entity, second).unwrap());
        assert_eq!(map.origins_for(entity).unwrap().len(), 2);
        assert_eq!(map.entity_count(), 1);
    }

    #[test]
    fn repeated_origin_insertion_is_idempotent() {
        let id = source_id(1);
        let mut map = SourceMap::new();
        map.register_source(id, document(Some(20))).unwrap();

        let entity = SourceEntity::Operation(operation_id(1));
        let origin = SourceOrigin::new(
            OriginKind::Direct,
            Some(span(id, 0, 5)),
            Vec::new(),
            None,
        );

        assert!(map.add_origin(entity, origin.clone()).unwrap());
        assert!(!map.add_origin(entity, origin).unwrap());
        assert_eq!(map.entity_count(), 1);
    }

    #[test]
    fn sorts_and_deduplicates_related_spans() {
        let id = source_id(1);
        let origin = SourceOrigin::new(
            OriginKind::Transformed,
            None,
            vec![span(id, 10, 12), span(id, 1, 2), span(id, 10, 12)],
            None,
        );

        assert_eq!(origin.related_spans().len(), 2);
        assert!(origin.related_spans()[0] < origin.related_spans()[1]);
    }

    #[test]
    fn validates_registered_map() {
        let id = source_id(1);
        let mut map = SourceMap::new();
        map.register_source(id, document(Some(100))).unwrap();

        map.add_origin(
            SourceEntity::Operation(operation_id(7)),
            SourceOrigin::new(
                OriginKind::Direct,
                Some(span(id, 10, 20)),
                Vec::new(),
                None,
            ),
        )
        .unwrap();

        assert!(map.validate().is_ok());
    }

    #[test]
    fn supports_large_offsets_without_host_width_assumptions() {
        let start = u64::MAX - 1;
        let end = u64::MAX;
        let result = SourceSpan::new(source_id(1), start, end, None, None);

        assert!(result.is_ok());
        assert_eq!(result.unwrap().byte_length(), 1);
    }
}
