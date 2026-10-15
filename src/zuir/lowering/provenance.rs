#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # ZUIR Lowering Provenance Adapter
//!
//! Path: `src/zuir/lowering/provenance.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: 1.97
//! Safety: Unsafe Rust is prohibited.
//!
//! ## PURPOSE
//!
//! Bridge lowering-local provenance records into the canonical ZUIR
//! `SourceMap`, preserving source identity, source ranges, and descriptions.
//!
//! ## STATUS
//!
//! Proposed implementation. Repository compilation and integration tests
//! are required before this file can be marked frozen.
//!
//! ## AUTHORITY
//!
//! - `grammar/DESIGN.md`
//! - `grammar/Zamani.g4`
//! - `grammar/specification/semantic-model.md`
//! - `src/zuir/ids.rs`
//! - `src/zuir/source_map.rs`
//! - `src/zuir/lowering/context.rs`
//! - `src/zuir/ZUIR-SPEC.md`
//!
//! ## OWNS
//!
//! - Translation of lowering provenance into canonical source origins.
//! - The explicit resolver boundary between lowering-local and typed IDs.
//! - Validation and reporting for provenance attachment.
//! - Atomicity of a single destination entity's provenance update.
//!
//! ## DOES_NOT_OWN
//!
//! - Source loading, URI interpretation, source identity allocation,
//!   or source text storage.
//! - Canonical source spans, source entities, or source-map data structures.
//! - AST construction, semantic analysis, or source-to-IR lowering.
//! - Diagnostic rendering, source-map serialization, or content hashing.
//! - Hardware discovery, resource allocation, or execution.
//!
//! ## INTEGRATION CONTRACT
//!
//! `context.rs` owns `EntityKey` and `ProvenanceOrigin`.
//! `source_map.rs` owns `SourceEntity`, `SourceId`, `SourceSpan`,
//! `SourceOrigin`, `OriginKind`, and `SourceMap`.
//!
//! Lowering-local identities are opaque strings and MUST NOT be parsed
//! or cast into typed ZUIR identifiers. The caller supplies the already
//! resolved destination `SourceEntity`; a `ProvenanceResolver` explicitly
//! resolves each lowering-local source key to a registered `SourceId`.
//!
//! The resolver MUST use authoritative identity mappings. It MUST NOT
//! guess IDs from names, numeric suffixes, filenames, or source offsets.
//!
//! A source range is interpreted as a half-open UTF-8 byte range
//! `[start_byte, end_byte)`. The canonical `SourceMap` remains responsible
//! for validating registered sources and source bounds.
//!
//! ## SCALABILITY
//!
//! The adapter uses dynamically sized collections and introduces no fixed
//! source, entity, operation, qubit, device, or provenance-count limit.
//! It validates a complete destination update before inserting its origins.
//! Actual memory, address-space, and allocator limits still apply.
//!
//! ## POCO-REAF
//!
//! Provenance is descriptive metadata and MUST NOT change program semantics.
//! Missing source ranges remain missing. Unsupported mappings fail explicitly.
//! The adapter does not imply that a destination is executable or that a
//! lowering transformation preserves semantics.
//!
//! ## ERROR CONTRACT
//!
//! Unresolved source identities, missing registered source documents, and
//! invalid spans are returned as explicit errors. The adapter never silently
//! drops an origin or fabricates a source location.
//!
//! ## FREEZE CRITERIA
//!
//! - The parent module declares `provenance` exactly once.
//! - This adapter compiles against the canonical source-map and context APIs.
//! - Identity resolution is explicit and tested.
//! - Missing ranges and multiple origins are preserved.
//! - Invalid batches leave the source map unchanged.
//! - Formatting, linting, and repository tests pass.

use core::fmt;
use std::collections::BTreeMap;
use std::error::Error as StdError;

use crate::zuir::ids::SourceId;
use crate::zuir::source_map::{
    OriginKind, SourceEntity, SourceMap, SourceMapError, SourceOrigin, SourceSpan,
};

use super::context::{EntityKey, LoweringContext, ProvenanceOrigin};

/// Resolves lowering-local source identities to canonical source IDs.
///
/// Implementations must consult an authoritative mapping created by the
/// source-management or frontend integration layer. Returning `None` means
/// that the identity cannot currently be resolved; it does not authorize
/// inventing an identifier.
///
/// The trait deliberately does not require a concrete source registry or
/// filesystem implementation.
pub trait ProvenanceResolver {
    /// Resolve a lowering-local source key to its canonical source ID.
    fn resolve_source(&self, source: &EntityKey) -> Option<SourceId>;
}

/// A deterministic resolver backed by explicit source-key mappings.
///
/// This helper is suitable for adapters that already possess a complete
/// source identity table. It does not allocate IDs or infer missing entries.
#[derive(Debug, Clone, Default)]
pub struct SourceIdentityMap {
    entries: BTreeMap<EntityKey, SourceId>,
}

impl SourceIdentityMap {
    /// Create an empty source identity map.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert an authoritative mapping.
    ///
    /// Returns the previous ID if this key was already present. Replacing a
    /// mapping is explicit so callers can detect identity-table conflicts.
    pub fn insert(
        &mut self,
        source: EntityKey,
        id: SourceId,
    ) -> Option<SourceId> {
        self.entries.insert(source, id)
    }

    /// Look up an existing mapping without modifying it.
    #[must_use]
    pub fn get(&self, source: &EntityKey) -> Option<SourceId> {
        self.entries.get(source).copied()
    }

    /// Return the number of explicit source mappings.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Return whether this identity map contains no mappings.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

impl ProvenanceResolver for SourceIdentityMap {
    fn resolve_source(&self, source: &EntityKey) -> Option<SourceId> {
        self.get(source)
    }
}

/// Summary of a successful provenance attachment operation.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ProvenanceReport {
    /// Number of origin records examined.
    pub examined: usize,

    /// Number of new origin records inserted into the source map.
    pub inserted: usize,

    /// Number of origin records that were already present.
    pub already_present: usize,
}

/// Failure while resolving or attaching lowering provenance.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ProvenanceError {
    /// A lowering-local source key has no authoritative mapping.
    UnresolvedSource {
        /// Source key that could not be resolved.
        source: EntityKey,
    },

    /// The resolved source ID has no registered source document.
    UnregisteredSource {
        /// Resolved canonical source ID.
        source_id: SourceId,
    },

    /// The origin's byte range extends beyond a known source length.
    SpanOutOfBounds {
        /// Source document involved.
        source_id: SourceId,
        /// Exclusive end offset supplied by the origin.
        end_byte: u64,
        /// Registered source length.
        source_length: u64,
    },

    /// Canonical span or source-map validation failed.
    SourceMap(SourceMapError),
}

impl fmt::Display for ProvenanceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnresolvedSource { source } => write!(
                formatter,
                "cannot resolve lowering provenance source \
                 '{}:{}'",
                source.namespace(),
                source.value()
            ),
            Self::UnregisteredSource { source_id } => write!(
                formatter,
                "provenance refers to unregistered source {source_id}"
            ),
            Self::SpanOutOfBounds {
                source_id,
                end_byte,
                source_length,
            } => write!(
                formatter,
                "provenance span for source {source_id} ends at byte \
                 {end_byte}, beyond its known length {source_length}"
            ),
            Self::SourceMap(error) => {
                write!(formatter, "source-map provenance error: {error}")
            }
        }
    }
}

impl StdError for ProvenanceError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::SourceMap(error) => Some(error),
            _ => None,
        }
    }
}

impl From<SourceMapError> for ProvenanceError {
    fn from(error: SourceMapError) -> Self {
        Self::SourceMap(error)
    }
}

/// A prepared origin ready to be inserted into the canonical source map.
///
/// Preparation performs identity resolution and span validation before any
/// mutation of the caller's source map occurs.
#[derive(Debug, Clone, PartialEq, Eq)]
struct PreparedOrigin {
    origin: SourceOrigin,
}

/// Attach every recorded provenance origin for one destination entity.
///
/// The destination is supplied as a canonical typed `SourceEntity`. This
/// function deliberately does not try to convert a lowering-local
/// `EntityKey` into a typed ID.
///
/// All origins are resolved and validated before insertion starts. Therefore,
/// an ordinary validation failure leaves the source map unchanged. Once
/// preparation succeeds, the exclusive mutable borrow prevents concurrent
/// changes to the map during insertion.
///
/// Origins without byte ranges are retained as source-free canonical origins
/// with a label containing the registered source URI and optional description.
/// This preserves the source identity descriptively without inventing offsets.
///
/// # Errors
///
/// Returns an error if a source identity cannot be resolved, the corresponding
/// document is not registered, a span is invalid, or the source map rejects an
/// origin. No origin is silently discarded.
pub fn attach_provenance(
    context: &LoweringContext,
    destination: SourceEntity,
    destination_key: &EntityKey,
    resolver: &impl ProvenanceResolver,
    source_map: &mut SourceMap,
) -> Result<ProvenanceReport, ProvenanceError> {
    let Some(origins) = context.provenance(destination_key) else {
        return Ok(ProvenanceReport::default());
    };

    let mut prepared = Vec::new();

    // Phase one: resolve and validate the entire batch before mutation.
    for origin in origins {
        prepared.push(prepare_origin(origin, resolver, source_map)?);
    }

    // Phase two: commit all prepared origins.
    //
    // Each origin has already been validated against this map. Since the
    // caller retains the exclusive borrow, the validation conditions cannot
    // be invalidated by another writer between preparation and insertion.
    let mut report = ProvenanceReport {
        examined: prepared.len(),
        inserted: 0,
        already_present: 0,
    };

    for candidate in prepared {
        if source_map.add_origin(destination, candidate.origin)? {
            report.inserted += 1;
        } else {
            report.already_present += 1;
        }
    }

    Ok(report)
}

/// Resolve and validate one lowering origin without mutating the source map.
fn prepare_origin(
    origin: &ProvenanceOrigin,
    resolver: &impl ProvenanceResolver,
    source_map: &SourceMap,
) -> Result<PreparedOrigin, ProvenanceError> {
    let source_id = resolver
        .resolve_source(origin.source())
        .ok_or_else(|| ProvenanceError::UnresolvedSource {
            source: origin.source().clone(),
        })?;

    let document = source_map
        .source(source_id)
        .ok_or(ProvenanceError::UnregisteredSource { source_id })?;

    let primary_span = match origin.byte_range() {
        Some((start_byte, end_byte)) => {
            if let Some(source_length) = document.byte_length() {
                if end_byte > source_length {
                    return Err(ProvenanceError::SpanOutOfBounds {
                        source_id,
                        end_byte,
                        source_length,
                    });
                }
            }

            Some(SourceSpan::new(
                source_id,
                start_byte,
                end_byte,
                None,
                None,
            )?)
        }
        None => None,
    };

    let label = match (primary_span.is_some(), origin.description()) {
        (true, Some(description)) => Some(description.to_owned()),
        (true, None) => None,
        (false, Some(description)) => Some(format!(
            "Source {}: {}",
            document.uri(),
            description
        )),
        (false, None) => Some(format!("Source {}", document.uri())),
    };

    Ok(PreparedOrigin {
        origin: SourceOrigin::new(
            OriginKind::Transformed,
            primary_span,
            Vec::new(),
            label,
        ),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::zuir::lowering::context::{
        EntityKind, LoweringContext, LoweringPolicy, ProvenanceOrigin,
    };
    use crate::zuir::source_map::{SourceDocument, SourceDocumentKind};

    fn source_id(raw: u128) -> SourceId {
        SourceId::new(raw).expect("valid test source ID")
    }

    fn source_key(value: &str) -> EntityKey {
        EntityKey::new(EntityKind::Source, "test.source", value)
            .expect("valid source key")
    }

    fn destination_key() -> EntityKey {
        EntityKey::new(
            EntityKind::Operation,
            "test.destination",
            "operation-1",
        )
        .expect("valid destination key")
    }

    fn source_map(id: SourceId, length: Option<u64>) -> SourceMap {
        let mut map = SourceMap::new();
        let document = SourceDocument::new(
            "memory://tests/main.zm",
            SourceDocumentKind::User,
            length,
        )
        .expect("valid source document");

        map.register_source(id, document)
            .expect("source registration succeeds");
        map
    }

    fn context_with_origin(
        destination: &EntityKey,
        source: EntityKey,
        range: Option<(u64, u64)>,
        description: Option<&str>,
    ) -> LoweringContext {
        let mut context = LoweringContext::new(LoweringPolicy::default());

        let (start, end) = match range {
            Some((start, end)) => (Some(start), Some(end)),
            None => (None, None),
        };

        let origin = ProvenanceOrigin::new(
            source,
            start,
            end,
            description.map(str::to_owned),
        )
        .expect("valid provenance origin");

        context.add_provenance(destination.clone(), origin);
        context
    }

    #[test]
    fn attaches_a_resolved_source_range() {
        let source = source_key("main.zm");
        let destination = destination_key();
        let id = source_id(1);

        let context = context_with_origin(
            &destination,
            source.clone(),
            Some((4, 12)),
            Some("source operation"),
        );

        let mut resolver = SourceIdentityMap::new();
        resolver.insert(source, id);

        let mut map = source_map(id, Some(32));

        let report = attach_provenance(
            &context,
            SourceEntity::Operation(
                crate::zuir::ids::OperationId::new(7)
                    .expect("valid operation ID"),
            ),
            &destination,
            &resolver,
            &mut map,
        )
        .expect("provenance attachment succeeds");

        assert_eq!(report.examined, 1);
        assert_eq!(report.inserted, 1);
        assert_eq!(report.already_present, 0);
    }

    #[test]
    fn repeated_attachment_is_idempotent() {
        let source = source_key("main.zm");
        let destination = destination_key();
        let id = source_id(2);

        let context = context_with_origin(
            &destination,
            source.clone(),
            Some((0, 5)),
            None,
        );

        let mut resolver = SourceIdentityMap::new();
        resolver.insert(source, id);

        let mut map = source_map(id, Some(16));
        let entity = SourceEntity::Operation(
            crate::zuir::ids::OperationId::new(8)
                .expect("valid operation ID"),
        );

        let first = attach_provenance(
            &context,
            entity,
            &destination,
            &resolver,
            &mut map,
        )
        .expect("first attachment succeeds");

        let second = attach_provenance(
            &context,
            entity,
            &destination,
            &resolver,
            &mut map,
        )
        .expect("second attachment succeeds");

        assert_eq!(first.inserted, 1);
        assert_eq!(second.inserted, 0);
        assert_eq!(second.already_present, 1);
        assert_eq!(
            map.origins_for(entity).map(|origins| origins.len()),
            Some(1)
        );
    }

    #[test]
    fn missing_ranges_do_not_create_fake_spans() {
        let source = source_key("generated.zm");
        let destination = destination_key();
        let id = source_id(3);

        let context = context_with_origin(
            &destination,
            source.clone(),
            None,
            Some("generated from a semantic construct"),
        );

        let mut resolver = SourceIdentityMap::new();
        resolver.insert(source, id);

        let mut map = source_map(id, None);
        let entity = SourceEntity::Operation(
            crate::zuir::ids::OperationId::new(9)
                .expect("valid operation ID"),
        );

        attach_provenance(
            &context,
            entity,
            &destination,
            &resolver,
            &mut map,
        )
        .expect("source-free provenance is valid");

        let origins = map.origins_for(entity).expect("origin is recorded");
        let origin = origins.iter().next().expect("one origin");

        assert!(origin.primary_span().is_none());
        assert!(
            origin
                .label()
                .expect("descriptive label")
                .contains("generated from a semantic construct")
        );
    }

    #[test]
    fn unresolved_source_is_an_explicit_error() {
        let source = source_key("missing.zm");
        let destination = destination_key();
        let context = context_with_origin(
            &destination,
            source,
            Some((0, 1)),
            None,
        );

        let resolver = SourceIdentityMap::new();
        let mut map = SourceMap::new();
        let entity = SourceEntity::Operation(
            crate::zuir::ids::OperationId::new(10)
                .expect("valid operation ID"),
        );

        let result = attach_provenance(
            &context,
            entity,
            &destination,
            &resolver,
            &mut map,
        );

        assert!(matches!(
            result,
            Err(ProvenanceError::UnresolvedSource { .. })
        ));
        assert!(map.is_empty());
    }

    #[test]
    fn invalid_batch_does_not_partially_mutate_the_map() {
        let first_source = source_key("first.zm");
        let second_source = source_key("second.zm");
        let destination = destination_key();
        let first_id = source_id(11);
        let second_id = source_id(12);

        let mut context = LoweringContext::new(LoweringPolicy::default());

        context.add_provenance(
            destination.clone(),
            ProvenanceOrigin::new(
                first_source.clone(),
                Some(0),
                Some(4),
                Some("valid origin".to_owned()),
            )
            .expect("valid origin"),
        );

        context.add_provenance(
            destination.clone(),
            ProvenanceOrigin::new(
                second_source.clone(),
                Some(0),
                Some(50),
                Some("out-of-bounds origin".to_owned()),
            )
            .expect("valid symbolic range"),
        );

        let mut resolver = SourceIdentityMap::new();
        resolver.insert(first_source, first_id);
        resolver.insert(second_source, second_id);

        let mut map = source_map(first_id, Some(10));
        map.register_source(
            second_id,
            SourceDocument::new(
                "memory://tests/second.zm",
                SourceDocumentKind::User,
                Some(8),
            )
            .expect("valid second source"),
        )
        .expect("second source registration succeeds");

        let entity = SourceEntity::Operation(
            crate::zuir::ids::OperationId::new(13)
                .expect("valid operation ID"),
        );

        let result = attach_provenance(
            &context,
            entity,
            &destination,
            &resolver,
            &mut map,
        );

        assert!(matches!(
            result,
            Err(ProvenanceError::SpanOutOfBounds { .. })
        ));
        assert!(map.origins_for(entity).is_none());
    }

    #[test]
    fn empty_context_provenance_is_a_no_op() {
        let context = LoweringContext::new(LoweringPolicy::default());
        let destination = destination_key();
        let resolver = SourceIdentityMap::new();
        let mut map = SourceMap::new();

        let report = attach_provenance(
            &context,
            SourceEntity::Operation(
                crate::zuir::ids::OperationId::new(14)
                    .expect("valid operation ID"),
            ),
            &destination,
            &resolver,
            &mut map,
        )
        .expect("empty provenance is valid");

        assert_eq!(report, ProvenanceReport::default());
        assert!(map.is_empty());
    }
}