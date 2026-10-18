
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZCIR Source-Map Integration
//!
//! Path: `src/classical/zcir/source_map.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Expose canonical ZUIR source-map and provenance contracts through the ZCIR
//! namespace. Classical IR consumers can therefore use source locations
//! without introducing a second source-map implementation.
//!
//! ## Authority
//!
//! `crate::zuir::source_map` owns the canonical source-map implementation.
//! `crate::zuir::ids` owns source and IR entity identities.
//!
//! Language syntax and source-level meaning remain governed by the grammar
//! and semantic specifications, including:
//!
//! - `grammar/Zamani.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/specification/poco-reaf.md`
//! - `grammar/validation/semantic-boundaries.md`
//!
//! ## Ownership
//!
//! This module owns only the ZCIR-facing import boundary.
//!
//! It does not own or implement:
//!
//! - Source IDs or identity allocation.
//! - Source documents or source-file loading.
//! - Source spans, coordinate conversion, or source text.
//! - Entity-to-origin storage.
//! - Provenance generation or transformation history.
//! - Diagnostic rendering.
//! - Serialization, migration, or content fingerprints.
//! - Parsing, AST construction, or semantic analysis.
//! - Classical IR entities, quantum IR entities, or backend behavior.
//!
//! ## Coordinate contract
//!
//! The canonical source-map implementation defines the coordinate rules.
//! In particular, byte spans use zero-based UTF-8 byte offsets with an
//! exclusive end. Optional human-facing positions use one-based coordinates
//! with an explicitly declared column unit.
//!
//! This facade does not reinterpret, normalize, truncate, or convert spans.
//!
//! ## Scalability and determinism
//!
//! All storage and validation behavior is inherited from the canonical ZUIR
//! implementation. This module adds no independent collections, fixed-size
//! buffers, counters, caches, or arbitrary source/entity ceilings.
//!
//! The absence of an artificial ceiling does not imply literal infinity.
//! Address space, available memory, configured operational budgets, and the
//! limits of a particular compilation remain real constraints.
//!
//! ## Dependency direction
//!
//! ```text
//! Frontend / semantic provenance
//!              |
//!              v
//!       canonical ZUIR source map
//!              |
//!              v
//!      ZCIR source-map facade
//!              |
//!              +--> classical IR construction
//!              +--> verification diagnostics
//!              +--> optimization and lowering
//!              +--> debugging and tooling
//! ```
//!
//! ZCIR must not depend on a second source-map store, and the canonical ZUIR
//! source-map implementation must not depend on this facade.
//!
//! ## Integration contract
//!
//! - Frontend adapters register source documents with the canonical map.
//! - ZCIR builders associate canonical entities with canonical origins.
//! - Verifiers validate source references and span bounds through the
//!   canonical implementation.
//! - Transformations preserve, combine, or explicitly omit provenance.
//! - Diagnostics consume source spans without changing their meaning.
//! - Serialization uses the canonical versioned persistence contracts.
//! - Quantum and existing classical IR consumers retain their own entity
//!   ownership while sharing the canonical provenance vocabulary.
//!
//! Every exported item below is a direct re-export. No wrapper type or second
//! source of truth is created.
//!
//! ## Freeze criteria
//!
//! - The ZCIR module root declares this module.
//! - The crate root exposes the existing ZUIR and classical modules exactly
//!   once, after their module trees are ready to compile.
//! - Every public name resolves to the canonical ZUIR definition.
//! - The facade's integration tests pass.
//! - The repository formatting, build, test, and lint gates pass.

/// Canonical source-map operation result.
pub use crate::zuir::source_map::SourceMapResult;

/// Canonical source-map error taxonomy.
pub use crate::zuir::source_map::SourceMapError;

/// Canonical source document descriptor.
pub use crate::zuir::source_map::SourceDocument;

/// Canonical source artifact classification.
pub use crate::zuir::source_map::SourceDocumentKind;

/// Canonical one-based source position.
pub use crate::zuir::source_map::SourcePosition;

/// Canonical unit used for source columns.
pub use crate::zuir::source_map::ColumnUnit;

/// Canonical half-open UTF-8 byte span.
pub use crate::zuir::source_map::SourceSpan;

/// Canonical typed ZUIR entity reference for provenance.
pub use crate::zuir::source_map::SourceEntity;

/// Canonical provenance origin classification.
pub use crate::zuir::source_map::OriginKind;

/// Canonical entity source-origin record.
pub use crate::zuir::source_map::SourceOrigin;

/// Canonical deterministic source registry and provenance map.
pub use crate::zuir::source_map::SourceMap;

#[cfg(test)]
mod tests {
    use super::*;

    use crate::zuir::ids::{ProgramId, SourceId};

    /// Compile-time type checking ensures that the ZCIR path exposes the
    /// canonical types rather than creating independent replacements.
    #[test]
    fn facade_uses_canonical_source_map_types() {
        fn accepts_canonical_map(_: &crate::zuir::source_map::SourceMap) {}
        fn accepts_canonical_span(_: &crate::zuir::source_map::SourceSpan) {}
        fn accepts_canonical_source_id(_: crate::zuir::ids::SourceId) {}

        let map = SourceMap::new();
        let source_id = SourceId::new(1).expect("nonzero source ID");
        let span = SourceSpan::new(source_id, 0, 3, None, None)
            .expect("valid half-open source span");

        accepts_canonical_map(&map);
        accepts_canonical_span(&span);
        accepts_canonical_source_id(source_id);

        assert_eq!(span.byte_length(), 3);
        assert!(!span.is_empty());
    }

    #[test]
    fn facade_preserves_source_registration_and_span_validation() {
        let source_id = SourceId::new(1).expect("nonzero source ID");
        let document = SourceDocument::new(
            "memory://zcir-test",
            SourceDocumentKind::Virtual,
            Some(3),
        )
        .expect("valid source document");

        let mut map = SourceMap::new();
        map.register_source(source_id, document)
            .expect("first registration succeeds");

        let span = SourceSpan::new(source_id, 0, 3, None, None)
            .expect("valid span");

        let origin = SourceOrigin::new(
            OriginKind::Direct,
            Some(span),
            Vec::new(),
            None,
        );

        let entity = SourceEntity::Program(
            ProgramId::new(1).expect("nonzero program ID"),
        );

        assert!(map.add_origin(entity, origin).expect("valid origin"));
        assert_eq!(map.source_count(), 1);
        assert_eq!(map.entity_count(), 1);
        assert!(map.primary_span_for(entity).is_some());
        assert!(map.validate().is_ok());
    }

    #[test]
    fn facade_preserves_canonical_duplicate_source_behavior() {
        let source_id = SourceId::new(1).expect("nonzero source ID");
        let document = SourceDocument::new(
            "memory://zcir-duplicate-test",
            SourceDocumentKind::Virtual,
            Some(0),
        )
        .expect("valid source document");

        let mut map = SourceMap::new();

        map.register_source(source_id, document.clone())
            .expect("initial registration succeeds");

        let result = map.register_source(source_id, document);

        assert!(matches!(
            result,
            Err(SourceMapError::DuplicateSource(id)) if id == source_id
        ));

        assert_eq!(map.source_count(), 1);
    }

    #[test]
    fn empty_source_spans_remain_valid() {
        let source_id = SourceId::new(1).expect("nonzero source ID");

        let span = SourceSpan::new(source_id, 0, 0, None, None)
            .expect("zero-width span is valid");

        assert!(span.is_empty());
        assert_eq!(span.byte_length(), 0);
    }
}
