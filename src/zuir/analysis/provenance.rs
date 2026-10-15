
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR Provenance Analysis
//!
//! Canonical path: `src/zuir/analysis/provenance.rs`
//! Edition: Rust 2021
//! Minimum intended Rust version: Rust 1.97 or later.
//! Safety: unsafe Rust is forbidden.
//!
//! STATUS
//!
//! Proposed implementation. Freeze after module integration, compilation,
//! and repository tests have passed.
//!
//! PURPOSE
//!
//! Provide deterministic, read-only analysis of source provenance for
//! explicitly supplied ZUIR entities, using the canonical SourceMap.
//!
//! This analysis preserves all source origins associated with an entity.
//! It reports coverage and origin counts without inventing missing origins,
//! collapsing multiple origins, or changing IR semantics.
//!
//! AUTHORITY
//!
//! - src/zuir/ZUIR-SPEC.md
//! - src/zuir/ids.rs
//! - src/zuir/source_map.rs
//! - src/zuir/ir/operation.rs
//! - src/zuir/ir/region.rs
//! - src/zuir/ir/block.rs
//! - src/zuir/ir/function.rs
//! - src/zuir/ir/module.rs
//! - src/zuir/ir/program.rs
//! - src/zuir/analysis/effects.rs
//! - grammar/DESIGN.md
//! - grammar/specification/semantic-model.md
//! - grammar/data/provenance.g4
//! - grammar/expressions/provenance.g4
//! - grammar/compile/provenance.g4
//! - grammar/security/provenance.g4
//!
//! OWNERSHIP
//!
//! This module owns:
//!
//! - Provenance-analysis snapshots.
//! - Deterministic entity-to-origin analysis results.
//! - Coverage and origin-count summaries.
//! - Analysis-specific errors.
//!
//! DOES_NOT_OWN
//!
//! - SourceDocument, SourceSpan, SourceOrigin, SourceEntity, or SourceMap.
//! - IR entity identity or allocation.
//! - Source parsing, AST construction, or semantic name resolution.
//! - Creation or mutation of source-map associations.
//! - Inference of data-dependency or transformation-lineage edges.
//! - Cryptographic hashes, signatures, attestations, or trust decisions.
//! - Authorization, security policy enforcement, or audit persistence.
//! - Serialization, incremental cache management, or IR mutation.
//! - Quantum-specific provenance representations.
//! - Scheduling, placement, routing, execution, or target selection.
//!
//! INPUTS
//!
//! A validated canonical SourceMap and an explicit iterator of SourceEntity
//! values identifying the entities within the caller's intended scope.
//!
//! The caller is responsible for enumerating the complete scope it wants
//! analyzed. This module does not assume that the SourceMap contains every
//! entity in a program or that an entity without an origin is invalid.
//!
//! OUTPUTS
//!
//! An immutable ProvenanceAnalysis snapshot, or a structured error.
//!
//! Missing origins remain observable as empty origin lists. They are not
//! silently converted into fabricated source locations or proof of origin.
//!
//! DEPENDENCY DIRECTION
//!
//! This module depends on the canonical source-map model and standard
//! library collections only. The source map does not depend on this module.
//!
//! GRAMMAR BOUNDARY
//!
//! The frontend parses provenance syntax and semantic analysis determines
//! its meaning. This module consumes canonical SourceMap data and never
//! parses grammar rules, source text, or provenance expressions.
//!
//! SEMANTIC BOUNDARY
//!
//! Source-origin associations describe where an entity came from. They do
//! not, by themselves, establish a complete derivation graph, prove that a
//! transformation preserved semantics, or authenticate an origin.
//!
//! DETERMINISM
//!
//! Entities are indexed by SourceEntity's canonical ordering. Origins retain
//! the canonical SourceMap ordering. Duplicate requested entities are
//! rejected rather than silently counted twice.
//!
//! SCALABILITY AND POCO-REAF
//!
//! This module defines no fixed maximum for entities, origins, source files,
//! operations, modules, qubits, processors, devices, tensor dimensions,
//! or distributed nodes.
//!
//! It uses dynamically sized standard-library collections. Counts use
//! checked arithmetic. Fallible Vec reservation is used where available.
//! Standard BTreeMap allocation is not generally fallible through a stable
//! fallible-reservation API, so total allocation-failure recovery cannot
//! be guaranteed.
//!
//! No finite process can promise literal infinity. Processing is bounded by
//! available memory, address space, time, and external compilation policies.
//!
//! INVALIDATION
//!
//! This is an immutable snapshot. Recompute it when the source map or
//! requested entity scope changes. Cache invalidation belongs to the caller
//! or pass manager.
//!
//! FREEZE CRITERIA
//!
//! - The canonical SourceMap remains the sole owner of origin associations.
//! - Invalid source maps fail before analysis results are returned.
//! - Duplicate requested entities return a structured error.
//! - Missing origins remain explicitly distinguishable from invalid maps.
//! - Multiple origins are retained without lossy deduplication in this layer.
//! - Summary counters cannot silently overflow.
//! - No unsafe code or fixed hardware-size limits are introduced.
//! - Module registration, unit tests, integration tests, and repository
//!   compilation succeed.

use core::fmt;
use std::collections::BTreeMap;
use std::error::Error;

use crate::zuir::ids::SourceEntity;
use crate::zuir::source_map::{
    SourceMap,
    SourceMapError,
    SourceOrigin,
    SourceSpan,
};

/// Result type for provenance analysis.
pub type ProvenanceAnalysisResult<T> =
    Result<T, ProvenanceAnalysisError>;

/// Errors detected while constructing a provenance-analysis snapshot.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ProvenanceAnalysisError {
    /// The canonical source map failed validation.
    InvalidSourceMap {
        source: SourceMapError,
    },

    /// The same entity was supplied more than once.
    ///
    /// Duplicate entities often indicate a traversal or scope-construction
    /// bug. Rejecting them avoids misleading coverage and count summaries.
    DuplicateEntity {
        entity: SourceEntity,
    },

    /// A summary counter exceeded the representable usize range.
    CountOverflow {
        counter: &'static str,
    },

    /// Reservation of a dynamically sized origin vector failed.
    AllocationFailure {
        entity: SourceEntity,
        reason: String,
    },
}

impl fmt::Display for ProvenanceAnalysisError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidSourceMap { source } => {
                write!(f, "cannot analyze an invalid source map: {source}")
            }

            Self::DuplicateEntity { entity } => {
                write!(
                    f,
                    "entity {entity:?} occurs more than once in \
                     the provenance-analysis input"
                )
            }

            Self::CountOverflow { counter } => {
                write!(
                    f,
                    "provenance-analysis counter {counter} overflowed"
                )
            }

            Self::AllocationFailure { entity, reason } => {
                write!(
                    f,
                    "could not reserve provenance storage for \
                     entity {entity:?}: {reason}"
                )
            }
        }
    }
}

impl Error for ProvenanceAnalysisError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidSourceMap { source } => Some(source),

            Self::DuplicateEntity { .. }
            | Self::CountOverflow { .. }
            | Self::AllocationFailure { .. } => None,
        }
    }
}

/// Provenance summary for an explicitly supplied analysis scope.
///
/// The result stores an entry for every requested entity, including entities
/// without source origins. This lets callers distinguish "analyzed, no
/// origins recorded" from "not included in this analysis".
///
/// Origin records are copied from the canonical SourceMap. The analysis does
/// not mutate or take ownership of the source map itself.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProvenanceAnalysis {
    entities: BTreeMap<SourceEntity, Vec<SourceOrigin>>,
    entity_count: usize,
    entities_with_origins: usize,
    entities_without_origins: usize,
    origin_count: usize,
    origins_with_primary_span: usize,
    related_span_count: usize,
}

impl ProvenanceAnalysis {
    /// Analyze provenance for an explicit collection of canonical entities.
    ///
    /// The source map is validated before results are constructed.
    ///
    /// `entities` defines the analysis scope; it is not inferred from the
    /// source map because the map may not contain every IR entity.
    ///
    /// # Errors
    ///
    /// Returns an error when:
    ///
    /// - the source map contains an invalid source association;
    /// - the input repeats an entity identity;
    /// - a summary counter cannot be represented;
    /// - reserving an origin vector fails.
    ///
    /// # Missing provenance
    ///
    /// An entity with no recorded origin is retained with an empty origin
    /// vector. Missing provenance is not, by itself, an invalid IR state.
    ///
    /// # Complexity
    ///
    /// Let E be the number of requested entities, S the number of registered
    /// sources, O the number of source-origin associations validated, and R
    /// the number of origin records selected for requested entities.
    ///
    /// The operation includes source-map validation and ordered-map indexing.
    /// Its costs depend on the actual input sizes and collection operations;
    /// no fixed machine-size ceiling is imposed.
    pub fn analyze<I>(
        source_map: &SourceMap,
        entities: I,
    ) -> ProvenanceAnalysisResult<Self>
    where
        I: IntoIterator<Item = SourceEntity>,
    {
        source_map
            .validate()
            .map_err(|source| {
                ProvenanceAnalysisError::InvalidSourceMap { source }
            })?;

        let mut indexed = BTreeMap::new();
        let mut entities_with_origins = 0usize;
        let mut entities_without_origins = 0usize;
        let mut origin_count = 0usize;
        let mut origins_with_primary_span = 0usize;
        let mut related_span_count = 0usize;

        for entity in entities {
            if indexed.contains_key(&entity) {
                return Err(
                    ProvenanceAnalysisError::DuplicateEntity { entity },
                );
            }

            let mut origins = Vec::new();

            if let Some(source_origins) = source_map.origins_for(entity) {
                origins
                    .try_reserve(source_origins.len())
                    .map_err(|error| {
                        ProvenanceAnalysisError::AllocationFailure {
                            entity,
                            reason: error.to_string(),
                        }
                    })?;

                for origin in source_origins {
                    origin_count = origin_count.checked_add(1).ok_or(
                        ProvenanceAnalysisError::CountOverflow {
                            counter: "origin_count",
                        },
                    )?;

                    if origin.primary_span().is_some() {
                        origins_with_primary_span =
                            origins_with_primary_span
                                .checked_add(1)
                                .ok_or(
                                    ProvenanceAnalysisError::CountOverflow {
                                        counter: "origins_with_primary_span",
                                    },
                                )?;
                    }

                    related_span_count = related_span_count
                        .checked_add(origin.related_spans().len())
                        .ok_or(
                            ProvenanceAnalysisError::CountOverflow {
                                counter: "related_span_count",
                            },
                        )?;

                    origins.push(origin.clone());
                }
            }

            if origins.is_empty() {
                entities_without_origins =
                    entities_without_origins.checked_add(1).ok_or(
                        ProvenanceAnalysisError::CountOverflow {
                            counter: "entities_without_origins",
                        },
                    )?;
            } else {
                entities_with_origins =
                    entities_with_origins.checked_add(1).ok_or(
                        ProvenanceAnalysisError::CountOverflow {
                            counter: "entities_with_origins",
                        },
                    )?;
            }

            indexed.insert(entity, origins);
        }

        let entity_count = entities_with_origins
            .checked_add(entities_without_origins)
            .ok_or(ProvenanceAnalysisError::CountOverflow {
                counter: "entity_count",
            })?;

        Ok(Self {
            entities: indexed,
            entity_count,
            entities_with_origins,
            entities_without_origins,
            origin_count,
            origins_with_primary_span,
            related_span_count,
        })
    }

    /// Number of distinct entities included in this analysis.
    #[must_use]
    pub const fn entity_count(&self) -> usize {
        self.entity_count
    }

    /// Number of analyzed entities with at least one origin.
    #[must_use]
    pub const fn entities_with_origins(&self) -> usize {
        self.entities_with_origins
    }

    /// Number of analyzed entities without recorded origins.
    #[must_use]
    pub const fn entities_without_origins(&self) -> usize {
        self.entities_without_origins
    }

    /// Number of source-origin records across the analyzed scope.
    #[must_use]
    pub const fn origin_count(&self) -> usize {
        self.origin_count
    }

    /// Number of origin records that contain a primary source span.
    ///
    /// This is a record count, not a count of unique spans or source files.
    #[must_use]
    pub const fn origins_with_primary_span(&self) -> usize {
        self.origins_with_primary_span
    }

    /// Number of related-span references across all selected origins.
    ///
    /// A span referenced by multiple origins is counted multiple times.
    #[must_use]
    pub const fn related_span_count(&self) -> usize {
        self.related_span_count
    }

    /// Whether the analyzed scope contains no entities.
    ///
    /// An empty scope does not establish complete provenance coverage for
    /// a program or prove that a program has no source origins.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.entity_count == 0
    }

    /// Look up all source origins recorded for an analyzed entity.
    ///
    /// Returns `None` when the entity was not part of this analysis.
    /// Returns `Some(&[])` when the entity was analyzed but no origins
    /// were recorded for it.
    #[must_use]
    pub fn origins_for(
        &self,
        entity: SourceEntity,
    ) -> Option<&[SourceOrigin]> {
        self.entities.get(&entity).map(Vec::as_slice)
    }

    /// Return the primary span selected by the canonical source-origin order.
    ///
    /// This is a convenience query. If multiple origin records have primary
    /// spans, it returns the first one in the canonical ordering. Use
    /// `origins_for` when every contributing origin must be inspected.
    #[must_use]
    pub fn primary_span_for(
        &self,
        entity: SourceEntity,
    ) -> Option<&SourceSpan> {
        self.origins_for(entity)?
            .iter()
            .find_map(SourceOrigin::primary_span)
    }

    /// Iterate over analyzed entities in deterministic identity order.
    ///
    /// Every requested entity is present, including entities with no origins.
    pub fn entities(
        &self,
    ) -> impl Iterator<Item = (SourceEntity, &[SourceOrigin])> {
        self.entities
            .iter()
            .map(|(entity, origins)| (*entity, origins.as_slice()))
    }

    /// Iterate over analyzed entities that have no recorded origins.
    ///
    /// This reports missing associations only. It does not decide whether
    /// provenance is required by the relevant IR or compilation policy.
    pub fn entities_without_recorded_origins(
        &self,
    ) -> impl Iterator<Item = SourceEntity> + '_ {
        self.entities
            .iter()
            .filter(|(_, origins)| origins.is_empty())
            .map(|(entity, _)| *entity)
    }

    /// Iterate over analyzed entities that have at least one origin.
    pub fn entities_with_recorded_origins(
        &self,
    ) -> impl Iterator<Item = SourceEntity> + '_ {
        self.entities
            .iter()
            .filter(|(_, origins)| !origins.is_empty())
            .map(|(entity, _)| *entity)
    }
}

/// Analyze source provenance for an explicitly supplied entity scope.
///
/// This free function is a convenience wrapper around
/// `ProvenanceAnalysis::analyze`.
pub fn analyze_provenance<I>(
    source_map: &SourceMap,
    entities: I,
) -> ProvenanceAnalysisResult<ProvenanceAnalysis>
where
    I: IntoIterator<Item = SourceEntity>,
{
    ProvenanceAnalysis::analyze(source_map, entities)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::zuir::ids::{OperationId, SourceId};
    use crate::zuir::source_map::{
        OriginKind,
        SourceDocument,
        SourceDocumentKind,
        SourceMap,
        SourceOrigin,
        SourceSpan,
    };

    fn source_id(value: u128) -> SourceId {
        SourceId::new(value).expect("test source ID must be nonzero")
    }

    fn operation_id(value: u128) -> OperationId {
        OperationId::new(value).expect("test operation ID must be nonzero")
    }

    fn entity(value: u128) -> SourceEntity {
        SourceEntity::Operation(operation_id(value))
    }

    fn test_source_map() -> SourceMap {
        let mut map = SourceMap::new();
        let source = source_id(1);

        let document = SourceDocument::new(
            "memory://provenance/main.zm",
            SourceDocumentKind::User,
            Some(128),
        )
        .expect("test source document should be valid");

        map.register_source(source, document)
            .expect("test source should register");

        let span = SourceSpan::new(source, 10, 20, None, None)
            .expect("test span should be valid");

        let origin = SourceOrigin::new(
            OriginKind::Direct,
            Some(span),
            Vec::new(),
            Some("authored operation".to_owned()),
        );

        map.add_origin(entity(1), origin)
            .expect("test origin should be valid");

        map
    }

    #[test]
    fn empty_scope_produces_empty_snapshot() {
        let map = test_source_map();

        let analysis = ProvenanceAnalysis::analyze(
            &map,
            std::iter::empty::<SourceEntity>(),
        )
        .expect("empty scope should be analyzable");

        assert!(analysis.is_empty());
        assert_eq!(analysis.entity_count(), 0);
        assert_eq!(analysis.origin_count(), 0);
        assert_eq!(analysis.entities_with_origins(), 0);
        assert_eq!(analysis.entities_without_origins(), 0);
    }

    #[test]
    fn records_entities_with_and_without_origins() {
        let map = test_source_map();

        let analysis = ProvenanceAnalysis::analyze(
            &map,
            [entity(1), entity(2)],
        )
        .expect("valid entity scope should be analyzable");

        assert_eq!(analysis.entity_count(), 2);
        assert_eq!(analysis.entities_with_origins(), 1);
        assert_eq!(analysis.entities_without_origins(), 1);
        assert_eq!(analysis.origin_count(), 1);
        assert_eq!(analysis.origins_with_primary_span(), 1);

        assert_eq!(
            analysis.origins_for(entity(1)).map(<[SourceOrigin]>::len),
            Some(1),
        );

        assert_eq!(
            analysis.origins_for(entity(2)).map(<[SourceOrigin]>::len),
            Some(0),
        );

        assert!(analysis.origins_for(entity(3)).is_none());
    }

    #[test]
    fn rejects_duplicate_entities() {
        let map = test_source_map();

        let error = ProvenanceAnalysis::analyze(
            &map,
            [entity(1), entity(1)],
        )
        .expect_err("duplicate entity should be rejected");

        assert_eq!(
            error,
            ProvenanceAnalysisError::DuplicateEntity {
                entity: entity(1),
            },
        );
    }

    #[test]
    fn keeps_multiple_distinct_origins() {
        let mut map = test_source_map();
        let source = source_id(1);

        let second_span = SourceSpan::new(
            source,
            30,
            40,
            None,
            None,
        )
        .expect("second span should be valid");

        let second_origin = SourceOrigin::new(
            OriginKind::Transformed,
            Some(second_span),
            Vec::new(),
            Some("transformation origin".to_owned()),
        );

        map.add_origin(entity(1), second_origin)
            .expect("second origin should be valid");

        let analysis = ProvenanceAnalysis::analyze(
            &map,
            [entity(1)],
        )
        .expect("multiple origins should be analyzable");

        assert_eq!(analysis.entities_with_origins(), 1);
        assert_eq!(analysis.origin_count(), 2);
        assert_eq!(
            analysis.origins_for(entity(1)).map(<[SourceOrigin]>::len),
            Some(2),
        );
    }

    #[test]
    fn reports_entities_in_deterministic_identity_order() {
        let map = test_source_map();

        let analysis = ProvenanceAnalysis::analyze(
            &map,
            [entity(3), entity(1), entity(2)],
        )
        .expect("valid scope should be analyzable");

        let observed: Vec<_> = analysis
            .entities()
            .map(|(entity, _)| entity)
            .collect();

        let mut expected = vec![entity(1), entity(2), entity(3)];
        expected.sort();

        assert_eq!(observed, expected);
    }

    #[test]
    fn free_function_matches_analysis_api() {
        let map = test_source_map();

        let analysis = analyze_provenance(&map, [entity(1)])
            .expect("free function should delegate successfully");

        assert_eq!(analysis.entity_count(), 1);
        assert_eq!(analysis.origin_count(), 1);
    }
}
