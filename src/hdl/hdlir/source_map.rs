
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani HDLIR Source Map and Provenance
//!
//! Path: `src/hdl/hdlir/source_map.rs`
//! Rust edition: 2021.
//! Minimum Rust version: 1.97.
//! Safety: safe Rust only.
//! Status: foundational HDLIR implementation; freeze after repository
//! integration and conformance tests pass.
//!
//! ## Purpose
//!
//! Associate HDLIR entities with the source locations that explain their
//! origin. Preserve multiple contributing locations when lowering or
//! transformations combine, generate, import, or rewrite IR entities.
//!
//! ## Authority
//!
//! This module follows:
//!
//! - `grammar/spec/hdl.md`
//! - `grammar/specification/semantic-model.md`
//! - `src/frontend/ast/node/source/span.rs`
//! - `src/frontend/ast/node/source/mod.rs`
//! - `src/hdl/ids.rs`
//! - `src/hdl/api.rs`
//! - `src/zuir/ZUIR-SPEC.md`
//!
//! The frontend owns canonical source coordinates. The semantic model owns
//! source-level meaning and provenance facts. HDLIR owns associations between
//! its entities and those existing source coordinates.
//!
//! ## Owns
//!
//! - Source-document metadata registered for one HDLIR compilation context.
//! - Validated associations between generic entity keys and source origins.
//! - Origin classification and deterministic ordering of origin records.
//! - Validation that referenced sources exist and known source lengths are
//!   respected.
//!
//! ## Does not own
//!
//! - Source loading, filesystem access, URI interpretation, or source text.
//! - Source-ID allocation or canonical AST source-span definitions.
//! - Parsing, semantic analysis, name resolution, or diagnostic rendering.
//! - HDLIR entity definitions or entity-ID allocation.
//! - ZUIR entity definitions, physical hardware identities, or device handles.
//! - Serialization formats, fingerprints, or transformation algorithms.
//!
//! ## Coordinate contract
//!
//! The canonical source location is the frontend's `Span`: zero-based UTF-8
//! byte offsets using half-open ranges `[start, end)`. Empty spans are valid.
//! This module does not invent a second span or line/column representation.
//!
//! When a source's byte length is known, registered origins must end at or
//! before that length. UTF-8 boundary validation requires source text and is
//! therefore outside this metadata-only module.
//!
//! ## Entity-key contract
//!
//! `SourceMap<K>` is generic over the entity key. `K` must implement `Ord`
//! so associations have deterministic iteration order. The map neither
//! constructs nor interprets keys.
//!
//! Callers should use an approved typed HDLIR identity as `K` once that
//! identity is available. A temporary adapter may use another key type,
//! provided it uniquely identifies entities within that map's scope.
//!
//! Numeric equality across independent ID namespaces does not establish
//! entity identity. Do not use physical pins, device addresses, or runtime
//! handles as keys for portable logical HDLIR entities.
//!
//! ## Determinism and scalability
//!
//! Ordered maps and sorted, deduplicated origin lists provide deterministic
//! iteration for the same keys and origin records. No global state, random
//! identifiers, pointer-derived identities, or universal source/entity
//! capacity limits are introduced.
//!
//! All actual work and storage remain finite and bounded by available
//! resources, representable coordinates, and caller-owned operational
//! budgets. Allocation failure is not converted into successful validation.
//!
//! ## Mutation contract
//!
//! Source registration rejects duplicate IDs instead of replacing existing
//! metadata. Origin insertion validates all spans before changing the map.
//! Batch replacement validates every proposed origin before committing the
//! replacement, so ordinary validation errors leave the previous mapping
//! unchanged.
//!
//! ## Integration
//!
//! - Frontend adapters register source metadata and reuse canonical `Span`.
//! - `lowering/from_semantic.rs` and `lowering/from_zuir.rs` attach origins
//!   to HDLIR entities without reparsing source text.
//! - Builders attach origins when constructing HDLIR entities.
//! - Transformations preserve, combine, or explicitly replace provenance.
//! - Diagnostics query this map to find relevant source locations.
//! - Serialization must preserve provenance only through a separately
//!   specified and validated artifact contract.
//!
//! This module is metadata only. It does not change hardware semantics.

use core::fmt;
use std::collections::BTreeMap;
use std::error::Error;

// IMPORTANT:
// Use the source identity and span from the existing frontend span module.
// Do not substitute source::source_id::SourceId until the repository's
// documented SourceId canonicalization migration has been completed.
pub use crate::frontend::ast::node::source::span::{SourceId, Span};

/// Result type for source-map operations.
pub type SourceMapResult<T> = Result<T, SourceMapError>;

/// Metadata describing a source artifact without loading its contents.
///
/// `uri` is an opaque identifier chosen by the source-management layer.
/// It is not interpreted as a filesystem path or network location here.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SourceDocument {
    uri: String,
    kind: SourceDocumentKind,
    byte_length: Option<u64>,
}

impl SourceDocument {
    /// Create a source-document descriptor.
    ///
    /// `byte_length`, when present, is measured in UTF-8 bytes.
    ///
    /// # Errors
    ///
    /// Returns [`SourceMapError::EmptySourceUri`] when `uri` is empty or
    /// consists only of whitespace.
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

    /// Return the opaque source identifier string.
    #[must_use]
    pub fn uri(&self) -> &str {
        &self.uri
    }

    /// Return the source-artifact category.
    #[must_use]
    pub const fn kind(&self) -> SourceDocumentKind {
        self.kind
    }

    /// Return the known source length in bytes, if available.
    #[must_use]
    pub const fn byte_length(&self) -> Option<u64> {
        self.byte_length
    }
}

/// Descriptive category of a source artifact.
///
/// This metadata does not determine the meaning of an HDLIR operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SourceDocumentKind {
    /// Source authored directly by a user.
    User,
    /// Source imported from another supported representation.
    Imported,
    /// Source produced by a generator or compiler.
    Generated,
    /// Source included from another source artifact.
    Included,
    /// A virtual or in-memory source artifact.
    Virtual,
    /// The source category is not known.
    Unknown,
}

/// Explains why an HDLIR entity is associated with source locations.
///
/// An entity may have multiple origins. For example, an operation produced
/// by lowering can originate from a source expression and a declaration
/// that determines its type or behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OriginKind {
    /// Directly authored source.
    Direct,
    /// Source introduced by code generation or elaboration.
    Generated,
    /// Source incorporated from another artifact.
    Included,
    /// Source related to macro or template expansion.
    MacroExpansion,
    /// Source imported from another representation.
    Imported,
    /// Source associated with an IR transformation.
    Transformed,
    /// The origin category is not known.
    Unknown,
}

/// One provenance record associated with an HDLIR entity.
///
/// `primary_span` is the preferred source location when one exists.
/// `related_spans` contain other contributing locations and may refer to
/// different source documents.
///
/// Related spans are sorted and deduplicated at construction time. A span
/// appearing as both primary and related is not automatically removed:
/// callers may intentionally preserve its distinct provenance role.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SourceOrigin {
    kind: OriginKind,
    primary_span: Option<Span>,
    related_spans: Vec<Span>,
    label: Option<String>,
}

impl SourceOrigin {
    /// Construct an origin record.
    ///
    /// This constructor normalizes the related-span list. Source registration
    /// and byte-bound validation are performed by [`SourceMap`], because
    /// those checks require a compilation context.
    #[must_use]
    pub fn new(
        kind: OriginKind,
        primary_span: Option<Span>,
        mut related_spans: Vec<Span>,
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

    /// Create a direct origin for a single source span.
    #[must_use]
    pub fn direct(span: Span) -> Self {
        Self::new(OriginKind::Direct, Some(span), Vec::new(), None)
    }

    /// Return the origin category.
    #[must_use]
    pub const fn kind(&self) -> OriginKind {
        self.kind
    }

    /// Return the preferred span, if available.
    #[must_use]
    pub const fn primary_span(&self) -> Option<Span> {
        self.primary_span
    }

    /// Return contributing spans in deterministic order.
    #[must_use]
    pub fn related_spans(&self) -> &[Span] {
        &self.related_spans
    }

    /// Return the optional explanatory label.
    #[must_use]
    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    /// Return the primary and related spans without allocating.
    ///
    /// The primary span is yielded first, followed by related spans in their
    /// normalized order. A span can occur twice if the caller deliberately
    /// supplied it in both roles.
    pub fn spans(&self) -> impl Iterator<Item = Span> + '_ {
        self.primary_span
            .into_iter()
            .chain(self.related_spans.iter().copied())
    }
}

/// Errors from source-map registration, mutation, or consistency checking.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SourceMapError {
    /// A source URI or opaque source key was empty.
    EmptySourceUri,

    /// The source ID is already registered.
    DuplicateSource(SourceId),

    /// An origin refers to a source ID that is not registered.
    UnknownSource(SourceId),

    /// A span extends beyond the known length of its source.
    SpanOutOfBounds {
        /// Source identity involved.
        source_id: SourceId,
        /// Exclusive end byte of the span.
        end_byte: u64,
        /// Registered source length in bytes.
        source_length: u64,
    },
}

impl fmt::Display for SourceMapError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptySourceUri => {
                formatter.write_str("source URI or identifier must not be empty")
            }
            Self::DuplicateSource(source_id) => {
                write!(formatter, "source {source_id} is already registered")
            }
            Self::UnknownSource(source_id) => {
                write!(formatter, "source {source_id} is not registered")
            }
            Self::SpanOutOfBounds {
                source_id,
                end_byte,
                source_length,
            } => write!(
                formatter,
                "span for source {source_id} ends at byte {end_byte}, \
                 beyond known source length {source_length}"
            ),
        }
    }
}

impl Error for SourceMapError {}

/// Source metadata and provenance associations for one compilation context.
///
/// `K` is the caller's entity-key type. It must implement `Ord` to guarantee
/// deterministic iteration. The map does not allocate IDs or assume any
/// particular HDLIR entity representation.
#[derive(Debug, Clone)]
pub struct SourceMap<K> {
    sources: BTreeMap<SourceId, SourceDocument>,
    origins: BTreeMap<K, Vec<SourceOrigin>>,
}

impl<K: Ord> Default for SourceMap<K> {
    fn default() -> Self {
        Self {
            sources: BTreeMap::new(),
            origins: BTreeMap::new(),
        }
    }
}

impl<K: Ord> SourceMap<K> {
    /// Create an empty source map.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a source document.
    ///
    /// Registration does not load the source contents or verify that the
    /// opaque URI resolves to a real file.
    ///
    /// # Errors
    ///
    /// Returns [`SourceMapError::DuplicateSource`] when the ID is already
    /// registered. Existing metadata is never silently replaced.
    pub fn register_source(
        &mut self,
        source_id: SourceId,
        document: SourceDocument,
    ) -> SourceMapResult<()> {
        if self.sources.contains_key(&source_id) {
            return Err(SourceMapError::DuplicateSource(source_id));
        }

        self.sources.insert(source_id, document);
        Ok(())
    }

    /// Look up a registered source document.
    #[must_use]
    pub fn source(&self, source_id: SourceId) -> Option<&SourceDocument> {
        self.sources.get(&source_id)
    }

    /// Return the number of registered source documents.
    #[must_use]
    pub fn source_count(&self) -> usize {
        self.sources.len()
    }

    /// Iterate over source documents in deterministic ID order.
    pub fn sources(
        &self,
    ) -> impl Iterator<Item = (SourceId, &SourceDocument)> + '_ {
        self.sources
            .iter()
            .map(|(source_id, document)| (*source_id, document))
    }

    /// Add one origin to an entity.
    ///
    /// The origin is fully validated before the mapping is changed.
    /// Duplicate origin records are ignored. The return value is `true` only
    /// when a new record was inserted.
    ///
    /// # Errors
    ///
    /// Returns an error if any referenced source is unregistered or any span
    /// exceeds a known source length.
    pub fn add_origin(
        &mut self,
        entity: K,
        origin: SourceOrigin,
    ) -> SourceMapResult<bool> {
        self.validate_origin(&origin)?;

        let origins = self.origins.entry(entity).or_default();

        match origins.binary_search(&origin) {
            Ok(_) => Ok(false),
            Err(index) => {
                origins.insert(index, origin);
                Ok(true)
            }
        }
    }

    /// Replace every origin associated with an entity.
    ///
    /// The proposed collection is validated, sorted, and deduplicated before
    /// the existing mapping is changed. If validation fails, the previous
    /// mapping remains intact.
    ///
    /// An empty collection removes the entity's provenance association.
    ///
    /// # Errors
    ///
    /// Returns an error if any proposed origin references an unregistered
    /// source or extends beyond a known source length.
    pub fn replace_origins<I>(
        &mut self,
        entity: K,
        origins: I,
    ) -> SourceMapResult<()>
    where
        I: IntoIterator<Item = SourceOrigin>,
    {
        let mut prepared: Vec<SourceOrigin> = origins.into_iter().collect();

        for origin in &prepared {
            self.validate_origin(origin)?;
        }

        prepared.sort();
        prepared.dedup();

        if prepared.is_empty() {
            self.origins.remove(&entity);
        } else {
            self.origins.insert(entity, prepared);
        }

        Ok(())
    }

    /// Return all origins for an entity, if the entity has an association.
    ///
    /// Returned origins are ordered deterministically.
    #[must_use]
    pub fn origins(&self, entity: &K) -> Option<&[SourceOrigin]> {
        self.origins.get(entity).map(Vec::as_slice)
    }

    /// Return the preferred primary span for an entity.
    ///
    /// The first normalized origin containing a primary span is returned.
    /// Call [`Self::origins`] when all contributing locations are required.
    #[must_use]
    pub fn primary_span(&self, entity: &K) -> Option<Span> {
        self.origins
            .get(entity)?
            .iter()
            .find_map(SourceOrigin::primary_span)
    }

    /// Remove all provenance associated with an entity.
    ///
    /// Returns `true` if an association existed.
    pub fn remove_entity(&mut self, entity: &K) -> bool {
        self.origins.remove(entity).is_some()
    }

    /// Return the number of entities with at least one origin.
    #[must_use]
    pub fn entity_count(&self) -> usize {
        self.origins.len()
    }

    /// Iterate over entity associations in deterministic key order.
    pub fn entity_origins(
        &self,
    ) -> impl Iterator<Item = (&K, &[SourceOrigin])> + '_ {
        self.origins
            .iter()
            .map(|(entity, origins)| (entity, origins.as_slice()))
    }

    /// Validate every origin association in this map.
    ///
    /// This is useful at verification, serialization, and integration
    /// boundaries. It checks metadata consistency, not HDLIR semantics.
    ///
    /// # Errors
    ///
    /// Returns the first invalid source reference or out-of-bounds span in
    /// deterministic entity/origin iteration order.
    pub fn validate(&self) -> SourceMapResult<()> {
        for origins in self.origins.values() {
            for origin in origins {
                self.validate_origin(origin)?;
            }
        }

        Ok(())
    }

    /// Validate one origin without mutating the map.
    fn validate_origin(&self, origin: &SourceOrigin) -> SourceMapResult<()> {
        for span in origin.spans() {
            let source_id = span.source();

            let document = self
                .sources
                .get(&source_id)
                .ok_or(SourceMapError::UnknownSource(source_id))?;

            if let Some(source_length) = document.byte_length() {
                if span.end_raw() > source_length {
                    return Err(SourceMapError::SpanOutOfBounds {
                        source_id,
                        end_byte: span.end_raw(),
                        source_length,
                    });
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frontend::ast::node::source::span::SourceOffset;

    fn source_id(raw: u64) -> SourceId {
        SourceId::from_raw(raw)
    }

    fn span(source_id: SourceId, start: u64, end: u64) -> Span {
        Span::new(
            source_id,
            SourceOffset::from_raw(start),
            SourceOffset::from_raw(end),
        )
        .expect("test span must have ordered endpoints")
    }

    fn document(length: Option<u64>) -> SourceDocument {
        SourceDocument::new(
            "memory://hdlir/test.zm",
            SourceDocumentKind::User,
            length,
        )
        .expect("test document must be valid")
    }

    fn registered_map() -> SourceMap<u64> {
        let mut map = SourceMap::new();
        map.register_source(source_id(1), document(Some(64)))
            .expect("source registration must succeed");
        map
    }

    #[test]
    fn registers_and_looks_up_source_metadata() {
        let mut map = SourceMap::<u64>::new();
        let source = source_id(7);

        map.register_source(source, document(Some(12)))
            .expect("first registration must succeed");

        assert_eq!(map.source_count(), 1);
        assert_eq!(
            map.source(source).map(SourceDocument::byte_length),
            Some(Some(12))
        );
    }

    #[test]
    fn duplicate_source_registration_does_not_replace_metadata() {
        let mut map = SourceMap::<u64>::new();
        let source = source_id(1);

        map.register_source(source, document(Some(10)))
            .expect("first registration must succeed");

        let error = map
            .register_source(source, document(Some(100)))
            .expect_err("duplicate source must be rejected");

        assert_eq!(error, SourceMapError::DuplicateSource(source));
        assert_eq!(
            map.source(source).and_then(SourceDocument::byte_length),
            Some(10)
        );
    }

    #[test]
    fn accepts_empty_and_half_open_spans() {
        let mut map = registered_map();

        assert!(map
            .add_origin(1, SourceOrigin::direct(span(source_id(1), 4, 4)))
            .expect("empty span at a valid point is accepted"));

        assert!(map
            .add_origin(2, SourceOrigin::direct(span(source_id(1), 0, 64)))
            .expect("span ending at source length is accepted"));

        assert!(map.validate().is_ok());
    }

    #[test]
    fn rejects_unregistered_source_references() {
        let mut map = registered_map();
        let unknown = source_id(99);

        let error = map
            .add_origin(1, SourceOrigin::direct(span(unknown, 0, 1)))
            .expect_err("unknown source must be rejected");

        assert_eq!(error, SourceMapError::UnknownSource(unknown));
        assert_eq!(map.entity_count(), 0);
    }

    #[test]
    fn rejects_spans_beyond_known_source_length() {
        let mut map = registered_map();

        let error = map
            .add_origin(
                1,
                SourceOrigin::direct(span(source_id(1), 0, 65)),
            )
            .expect_err("out-of-bounds span must be rejected");

        assert_eq!(
            error,
            SourceMapError::SpanOutOfBounds {
                source_id: source_id(1),
                end_byte: 65,
                source_length: 64,
            }
        );
        assert_eq!(map.entity_count(), 0);
    }

    #[test]
    fn accepts_unbounded_length_metadata_when_length_is_unknown() {
        let mut map = SourceMap::<u64>::new();

        map.register_source(source_id(1), document(None))
            .expect("source registration must succeed");

        map.add_origin(
            1,
            SourceOrigin::direct(span(source_id(1), 0, u64::MAX)),
        )
        .expect("unknown source length does not imply a false upper bound");

        assert!(map.validate().is_ok());
    }

    #[test]
    fn sorts_and_deduplicates_related_spans() {
        let first = span(source_id(1), 2, 4);
        let second = span(source_id(1), 8, 10);

        let origin = SourceOrigin::new(
            OriginKind::Transformed,
            Some(first),
            vec![second, first, second],
            None,
        );

        assert_eq!(origin.related_spans(), &[first, second]);
    }

    #[test]
    fn adds_origins_in_deterministic_order_and_deduplicates() {
        let mut map = registered_map();
        let entity = 42_u64;
        let earlier = SourceOrigin::direct(span(source_id(1), 1, 2));
        let later = SourceOrigin::direct(span(source_id(1), 8, 9));

        assert!(map
            .add_origin(entity, later.clone())
            .expect("valid origin must be added"));

        assert!(map
            .add_origin(entity, earlier.clone())
            .expect("valid origin must be added"));

        assert!(!map
            .add_origin(entity, earlier.clone())
            .expect("duplicate origin is not an error"));

        let origins = map.origins(&entity).expect("entity must have origins");
        assert_eq!(origins.len(), 2);
        assert!(origins[0] < origins[1]);
    }

    #[test]
    fn replacement_is_transactional_on_validation_error() {
        let mut map = registered_map();
        let entity = 3_u64;
        let original = SourceOrigin::direct(span(source_id(1), 1, 2));

        map.add_origin(entity, original.clone())
            .expect("original origin must be accepted");

        let invalid = SourceOrigin::direct(span(source_id(90), 0, 1));
        let error = map
            .replace_origins(entity, vec![invalid])
            .expect_err("replacement must fail");

        assert_eq!(error, SourceMapError::UnknownSource(source_id(90)));
        assert_eq!(map.origins(&entity), Some([original].as_slice()));
    }

    #[test]
    fn replacing_with_empty_origins_removes_the_association() {
        let mut map = registered_map();
        let entity = 4_u64;

        map.add_origin(
            entity,
            SourceOrigin::direct(span(source_id(1), 1, 2)),
        )
        .expect("origin must be accepted");

        map.replace_origins(entity, Vec::new())
            .expect("empty replacement must succeed");

        assert!(map.origins(&entity).is_none());
        assert_eq!(map.entity_count(), 0);
    }

    #[test]
    fn supports_multiple_source_documents_for_combined_origins() {
        let mut map = registered_map();
        let second_source = source_id(2);

        map.register_source(second_source, document(Some(24)))
            .expect("second source must be registered");

        let origin = SourceOrigin::new(
            OriginKind::Transformed,
            Some(span(source_id(1), 2, 5)),
            vec![span(second_source, 4, 9)],
            Some("combined source constructs".to_owned()),
        );

        map.add_origin(12_u64, origin)
            .expect("both source references are valid");

        assert_eq!(
            map.origins(&12).expect("origins exist")[0]
                .related_spans()
                .len(),
            1
        );
        assert!(map.validate().is_ok());
    }

    #[test]
    fn primary_span_returns_the_first_available_primary_location() {
        let mut map = registered_map();
        let entity = 8_u64;
        let expected = span(source_id(1), 7, 11);

        map.add_origin(
            entity,
            SourceOrigin::new(
                OriginKind::Generated,
                None,
                Vec::new(),
                Some("generated entity".to_owned()),
            ),
        )
        .expect("source-free origin is valid");

        map.add_origin(
            entity,
            SourceOrigin::direct(expected),
        )
        .expect("direct origin is valid");

        assert_eq!(map.primary_span(&entity), Some(expected));
    }

    #[test]
    fn source_map_iteration_is_ordered_by_entity_key() {
        let mut map = registered_map();

        for entity in [30_u64, 10, 20] {
            map.add_origin(
                entity,
                SourceOrigin::direct(span(source_id(1), entity, entity + 1)),
            )
            .expect("origin must be accepted");
        }

        let keys: Vec<u64> = map
            .entity_origins()
            .map(|(entity, _)| *entity)
            .collect();

        assert_eq!(keys, vec![10, 20, 30]);
    }

    #[test]
    fn rejects_empty_source_identifiers() {
        let error = SourceDocument::new(
            "  ",
            SourceDocumentKind::User,
            None,
        )
        .expect_err("blank source identifier must be rejected");

        assert_eq!(error, SourceMapError::EmptySourceUri);
    }
}
