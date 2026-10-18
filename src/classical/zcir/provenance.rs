
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # ZCIR — Classical Provenance Adapter
//!
//! Canonical path: `src/classical/zcir/provenance.rs`
//! Rust edition: 2021
//! Minimum supported Rust version: 1.97
//! Safety: Safe Rust only; `unsafe` is prohibited.
//!
//! ## STATUS
//!
//! Implementation candidate. Freeze only after module registration,
//! repository compilation, and the relevant integration tests pass.
//!
//! ## PURPOSE
//!
//! Provide the classical ZCIR integration boundary for source-origin
//! provenance. Preserve the identity, source spans, origin category,
//! related spans, and descriptive labels recorded for classical entities.
//!
//! ## AUTHORITY
//!
//! - `grammar/DESIGN.md`
//! - `grammar/specification/grammar-authority.md`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/specification/poco-reaf.md`
//! - `grammar/spec/provenance.md`
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/ids.rs`
//! - `src/zuir/source_map.rs`
//! - `src/zuir/analysis/provenance.rs`
//! - `src/zuir/lowering/context.rs`
//! - `src/zuir/lowering/provenance.rs`
//! - `src/frontend/semantic/model/provenance.rs`
//!
//! ## OWNS
//!
//! - The ZCIR-facing adapter for recording canonical source origins.
//! - The documented boundary between classical entities and canonical
//!   source-origin associations.
//!
//! ## DOES_NOT_OWN
//!
//! - SourceDocument, SourceSpan, SourceOrigin, SourceEntity, OriginKind,
//!   SourceId, SourceMap, or source-map validation.
//! - Semantic provenance graphs, semantic IDs, or AST provenance.
//! - Classical IR entity definitions or identifier allocation.
//! - Transformation execution, optimization, or proof of semantic
//!   preservation.
//! - Lowering-local provenance identity resolution.
//! - Source parsing, source loading, filesystem access, or URI resolution.
//! - Cryptographic hashing, signatures, authentication, or trust decisions.
//! - Serialization formats, diagnostic rendering, or target selection.
//!
//! ## INTEGRATION CONTRACT
//!
//! `src/zuir/source_map.rs` is the sole owner of canonical source origins
//! and their association with canonical SourceEntity values.
//!
//! This adapter receives an already-resolved SourceEntity and a mutable
//! canonical SourceMap. It never guesses entity IDs from names, filenames,
//! numeric suffixes, instruction positions, or source offsets.
//!
//! Source spans use canonical zero-based UTF-8 byte offsets and exclusive
//! end positions. SourceMap validates source registration and known bounds.
//!
//! Semantic provenance remains owned by
//! `src/frontend/semantic/model/provenance.rs`. Lowering provenance remains
//! owned by `src/zuir/lowering/context.rs` and
//! `src/zuir/lowering/provenance.rs`. Source-origin recording here must not
//! be treated as a replacement for those richer contracts.
//!
//! ## DETERMINISM
//!
//! The adapter introduces no global state, timestamps, random identifiers,
//! host paths, or implicit identity allocation. Canonical origin ordering
//! and duplicate handling remain owned by SourceMap.
//!
//! ## SCALABILITY AND POCO-REAF
//!
//! This file defines no fixed source, entity, operation, device, processor,
//! memory, or provenance-count ceiling. It uses the canonical dynamically
//! sized source-map representation.
//!
//! Available memory, address space, execution time, and caller-imposed
//! operational budgets remain practical constraints. Literal infinite
//! storage or computation is not promised.
//!
//! Provenance is descriptive metadata and must not alter program meaning.
//! Missing origins remain missing; invalid origins are reported explicitly.
//!
//! ## FREEZE CRITERIA
//!
//! - The ZCIR parent module declares this file exactly once.
//! - All public types are canonical ZUIR source-map types.
//! - Invalid and unregistered source spans are rejected by SourceMap.
//! - Repeated insertion is idempotent.
//! - Tests cover valid, invalid, duplicate, and boundary cases.
//! - Repository compilation, formatting, and relevant tests pass.

use crate::zuir::source_map::{
    OriginKind,
    SourceEntity,
    SourceMap,
    SourceMapError,
    SourceOrigin,
    SourceSpan,
};

/// Result of recording a classical entity's source origin.
///
/// `true` means a new association was inserted; `false` means the exact
/// association already existed. Both outcomes are successful and
/// deterministic.
pub type ClassicalProvenanceResult<T> = Result<T, SourceMapError>;

/// Record a source origin for a canonical classical ZCIR entity.
///
/// This function creates a canonical [`SourceOrigin`] and delegates
/// validation and insertion to [`SourceMap::add_origin`]. It does not
/// allocate entity or source IDs, infer source locations, or silently
/// discard invalid provenance.
///
/// `primary_span` is the preferred source location when one is known.
/// `related_spans` identify other contributing locations, such as a macro
/// definition or an included expression. The canonical constructor sorts
/// and deduplicates related spans for deterministic representation.
///
/// The operation is idempotent: recording the same origin for the same
/// entity more than once does not create duplicate associations.
///
/// # Errors
///
/// Returns a [`SourceMapError`] if any span refers to an unregistered source
/// or exceeds a registered source's known byte length, or if canonical
/// source-map validation fails.
///
/// # Example
///
/// ```ignore
/// let inserted = record_origin(
///     &mut source_map,
///     SourceEntity::Operation(operation_id),
///     OriginKind::Direct,
///     Some(span),
///     Vec::new(),
///     Some("classical operation".to_owned()),
/// )?;
/// ```
pub fn record_origin(
    source_map: &mut SourceMap,
    entity: SourceEntity,
    kind: OriginKind,
    primary_span: Option<SourceSpan>,
    related_spans: Vec<SourceSpan>,
    label: Option<String>,
) -> ClassicalProvenanceResult<bool> {
    let origin = SourceOrigin::new(kind, primary_span, related_spans, label);
    source_map.add_origin(entity, origin)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::zuir::ids::{OperationId, SourceId};
    use crate::zuir::source_map::{
        SourceDocument,
        SourceDocumentKind,
    };

    fn source_id(raw: u128) -> SourceId {
        SourceId::new(raw).expect("test source ID must be nonzero")
    }

    fn operation_entity(raw: u128) -> SourceEntity {
        SourceEntity::Operation(
            OperationId::new(raw).expect("test operation ID must be nonzero"),
        )
    }

    fn source_map_with_length(length: u64) -> (SourceMap, SourceId) {
        let id = source_id(1);
        let document = SourceDocument::new(
            "memory://zcir-tests/main.zm",
            SourceDocumentKind::User,
            Some(length),
        )
        .expect("test source document should be valid");

        let mut map = SourceMap::new();
        map.register_source(id, document)
            .expect("test source should register");

        (map, id)
    }

    fn span(source: SourceId, start: u64, end: u64) -> SourceSpan {
        SourceSpan::new(source, start, end, None, None)
            .expect("test span endpoints should be ordered")
    }

    #[test]
    fn records_a_valid_direct_source_origin() {
        let (mut map, source) = source_map_with_length(64);
        let entity = operation_entity(1);

        let inserted = record_origin(
            &mut map,
            entity,
            OriginKind::Direct,
            Some(span(source, 4, 12)),
            Vec::new(),
            Some("authored classical operation".to_owned()),
        )
        .expect("valid provenance should be accepted");

        assert!(inserted);

        let origins = map
            .origins_for(entity)
            .expect("entity should have recorded origins");

        assert_eq!(origins.len(), 1);

        let origin = origins.iter().next().expect("one origin should exist");
        assert_eq!(origin.kind(), OriginKind::Direct);
        assert_eq!(
            origin.primary_span().map(SourceSpan::start_byte),
            Some(4)
        );
        assert_eq!(
            origin.primary_span().map(SourceSpan::end_byte),
            Some(12)
        );
        assert_eq!(origin.label(), Some("authored classical operation"));
    }

    #[test]
    fn recording_the_same_origin_is_idempotent() {
        let (mut map, source) = source_map_with_length(64);
        let entity = operation_entity(2);

        let first = record_origin(
            &mut map,
            entity,
            OriginKind::Generated,
            Some(span(source, 8, 16)),
            Vec::new(),
            Some("generated operation".to_owned()),
        )
        .expect("first insertion should succeed");

        let second = record_origin(
            &mut map,
            entity,
            OriginKind::Generated,
            Some(span(source, 8, 16)),
            Vec::new(),
            Some("generated operation".to_owned()),
        )
        .expect("repeated insertion should succeed");

        assert!(first);
        assert!(!second);
        assert_eq!(
            map.origins_for(entity).map(|origins| origins.len()),
            Some(1)
        );
    }

    #[test]
    fn preserves_multiple_distinct_origins() {
        let (mut map, source) = source_map_with_length(64);
        let entity = operation_entity(3);

        assert!(record_origin(
            &mut map,
            entity,
            OriginKind::Direct,
            Some(span(source, 2, 8)),
            Vec::new(),
            Some("source expression".to_owned()),
        )
        .expect("direct origin should be recorded"));

        assert!(record_origin(
            &mut map,
            entity,
            OriginKind::Transformed,
            Some(span(source, 20, 28)),
            vec![span(source, 30, 34)],
            Some("transformation contribution".to_owned()),
        )
        .expect("transformation origin should be recorded"));

        let origins = map
            .origins_for(entity)
            .expect("entity should retain both origins");

        assert_eq!(origins.len(), 2);
        assert!(origins.iter().any(|origin| {
            origin.kind() == OriginKind::Direct
        }));
        assert!(origins.iter().any(|origin| {
            origin.kind() == OriginKind::Transformed
                && origin.related_spans().len() == 1
        }));
    }

    #[test]
    fn rejects_a_span_that_exceeds_known_source_length() {
        let (mut map, source) = source_map_with_length(16);
        let entity = operation_entity(4);

        let result = record_origin(
            &mut map,
            entity,
            OriginKind::Direct,
            Some(span(source, 8, 17)),
            Vec::new(),
            None,
        );

        assert!(matches!(
            result,
            Err(SourceMapError::SpanOutOfBounds {
                source_id,
                end_byte: 17,
                source_length: 16,
            }) if source_id == source
        ));

        assert!(map.origins_for(entity).is_none());
    }

    #[test]
    fn rejects_an_unregistered_source_reference() {
        let (mut map, _registered_source) = source_map_with_length(64);
        let unknown_source = source_id(2);
        let entity = operation_entity(5);

        let result = record_origin(
            &mut map,
            entity,
            OriginKind::Direct,
            Some(span(unknown_source, 0, 4)),
            Vec::new(),
            None,
        );

        assert!(matches!(
            result,
            Err(SourceMapError::InvalidSourceReference(id))
                if id == unknown_source
        ));

        assert!(map.origins_for(entity).is_none());
    }

    #[test]
    fn accepts_an_empty_span_at_a_valid_source_boundary() {
        let (mut map, source) = source_map_with_length(16);
        let entity = operation_entity(6);

        let inserted = record_origin(
            &mut map,
            entity,
            OriginKind::Direct,
            Some(span(source, 16, 16)),
            Vec::new(),
            Some("end-of-source anchor".to_owned()),
        )
        .expect("valid empty span at EOF should be accepted");

        assert!(inserted);

        let origin = map
            .origins_for(entity)
            .and_then(|origins| origins.iter().next())
            .expect("origin should be present");

        let recorded_span = origin
            .primary_span()
            .expect("primary span should be present");

        assert!(recorded_span.is_empty());
        assert_eq!(recorded_span.start_byte(), 16);
        assert_eq!(recorded_span.end_byte(), 16);
    }

    #[test]
    fn accepts_origins_without_a_source_span() {
        let (mut map, _source) = source_map_with_length(16);
        let entity = operation_entity(7);

        let inserted = record_origin(
            &mut map,
            entity,
            OriginKind::Unknown,
            None,
            Vec::new(),
            Some("origin not available".to_owned()),
        )
        .expect("span-free origin should be valid");

        assert!(inserted);

        let origin = map
            .origins_for(entity)
            .and_then(|origins| origins.iter().next())
            .expect("origin should be present");

        assert!(origin.primary_span().is_none());
        assert_eq!(origin.label(), Some("origin not available"));
    }
}
