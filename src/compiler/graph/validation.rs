
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Universal Meta-Compiler — Compilation Graph Validation.
//!
//! Path: src/compiler/graph/validation.rs
//! Edition: Rust 2021
//! Minimum supported Rust version: 1.97
//! Safety: Unsafe Rust is forbidden.
//!
//! # Purpose
//!
//! Validate the canonical compiler compilation graph and its separately
//! owned typed edge set.
//!
//! This module coordinates structural validation and optional semantic
//! validation. It does not create another graph representation.
//!
//! # Canonical ownership
//!
//! - `super::model` owns graph and node structure.
//! - `super::edges` owns edge structure and endpoint relationships.
//! - Frontend semantic authorities own source-level meaning.
//! - Canonical IR authorities own domain-specific IR meaning.
//! - Hybrid and distributed authorities own their domain semantics.
//! - Compiler planning owns feasibility analysis.
//! - Runtime and hardware authorities own physical realization.
//!
//! # Non-responsibilities
//!
//! This module does not:
//!
//! - Parse source grammar.
//! - Construct ASTs.
//! - Redefine semantic types or effects.
//! - Define another IR.
//! - Infer missing dependencies.
//! - Select physical devices.
//! - Allocate hardware resources.
//! - Perform physical routing or scheduling.
//! - Assume that every graph must be acyclic.
//! - Claim that structural validity proves target feasibility.
//!
//! # Scalability
//!
//! No arbitrary universal maximum is imposed on graph nodes, edges,
//! identifiers, metadata, or semantic relationships.
//!
//! Collection growth is bounded by available memory, address space,
//! allocation behavior, and any explicit operational budget enforced
//! by a higher-level owner.
//!
//! This module does not promise literal infinite resources.
//!
//! # Determinism
//!
//! Validation visits nodes and edges in their canonical declaration order.
//! Report vectors preserve that order.
//!
//! # Verification levels
//!
//! Structural validation proves local graph and edge-set invariants,
//! graph identity consistency, and endpoint existence.
//!
//! Semantic validation additionally delegates canonical entity resolution,
//! edge semantics, and required-relationship verification to an explicit
//! authoritative resolver.
//!
//! Neither level establishes physical target feasibility.

use std::error::Error;
use std::fmt;

use super::edges::{
    CompilationGraphEdge,
    CompilationGraphEdgeId,
    CompilationGraphEdgeSet,
    GraphEdgesError,
};

use super::model::{
    CanonicalEntityRef,
    CompilationGraph,
    CompilationGraphNode,
    CompilationGraphNodeId,
    GraphModelError,
};

// ============================================================================
// API VERSION
// ============================================================================

/// Public API version of compiler graph validation.
pub const COMPILATION_GRAPH_VALIDATION_API_VERSION: u16 = 1;

/// Validation report schema version.
pub const COMPILATION_GRAPH_VALIDATION_SCHEMA_VERSION: u16 = 1;

// ============================================================================
// RESULT
// ============================================================================

/// Result type for compilation graph validation.
pub type GraphValidationResult<T> =
    Result<T, GraphValidationError>;

// ============================================================================
// ERRORS
// ============================================================================

/// Errors produced by compilation graph validation.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum GraphValidationError {
    /// The graph model is structurally invalid.
    GraphModel {
        source: GraphModelError,
    },

    /// The edge set is structurally invalid.
    GraphEdges {
        source: GraphEdgesError,
    },

    /// The edge set belongs to another graph.
    GraphIdentityMismatch {
        expected: String,
        actual: String,
    },

    /// An authoritative semantic resolver rejected a graph entity.
    EntitySemanticInvalid {
        node_id: String,
        authority: String,
        entity_kind: String,
        entity_id: String,
        reason: String,
    },

    /// An authoritative semantic resolver rejected an edge.
    EdgeSemanticInvalid {
        edge_id: String,
        edge_kind: String,
        reason: String,
    },

    /// The authoritative semantic representation requires a relationship
    /// that is missing or represented incorrectly.
    RequiredRelationshipsInvalid {
        reason: String,
    },

    /// An internal validation accounting operation overflowed.
    SizeOverflow,
}

impl fmt::Display for GraphValidationError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::GraphModel { source } => {
                write!(
                    formatter,
                    "compilation graph model validation failed: {source}"
                )
            }

            Self::GraphEdges { source } => {
                write!(
                    formatter,
                    "compilation graph edge validation failed: {source}"
                )
            }

            Self::GraphIdentityMismatch {
                expected,
                actual,
            } => {
                write!(
                    formatter,
                    "edge set belongs to graph '{actual}', \
                     but graph '{expected}' was supplied"
                )
            }

            Self::EntitySemanticInvalid {
                node_id,
                authority,
                entity_kind,
                entity_id,
                reason,
            } => {
                write!(
                    formatter,
                    "semantic validation failed for node '{node_id}' \
                     referencing '{authority}:{entity_kind}:{entity_id}': \
                     {reason}"
                )
            }

            Self::EdgeSemanticInvalid {
                edge_id,
                edge_kind,
                reason,
            } => {
                write!(
                    formatter,
                    "semantic validation failed for edge '{edge_id}' \
                     of kind '{edge_kind}': {reason}"
                )
            }

            Self::RequiredRelationshipsInvalid {
                reason,
            } => {
                write!(
                    formatter,
                    "required compilation graph relationships are invalid: \
                     {reason}"
                )
            }

            Self::SizeOverflow => {
                formatter.write_str(
                    "compilation graph validation accounting overflowed",
                )
            }
        }
    }
}

impl Error for GraphValidationError {}

impl From<GraphModelError> for GraphValidationError {
    fn from(source: GraphModelError) -> Self {
        Self::GraphModel { source }
    }
}

impl From<GraphEdgesError> for GraphValidationError {
    fn from(source: GraphEdgesError) -> Self {
        Self::GraphEdges { source }
    }
}

// ============================================================================
// SEMANTIC RESOLUTION CONTRACT
// ============================================================================

/// Authoritative semantic resolver for compilation graph validation.
///
/// The resolver connects graph references to the repository subsystem
/// that actually owns the referenced entity.
///
/// Implementations must not treat an unknown entity, unsupported
/// relationship, or unavailable semantic authority as successful
/// verification.
///
/// The compiler graph validator deliberately does not know how to resolve
/// every future domain. New authorities can implement this trait without
/// changing the validator.
///
/// # Responsibilities
///
/// Implementations must:
///
/// - Resolve canonical entity references using their owning authority.
/// - Verify that referenced entities exist and are valid.
/// - Validate edge semantics using the appropriate domain authority.
/// - Check that all required semantic relationships are represented.
/// - Preserve domain-specific cycle and ordering rules.
///
/// # Non-responsibilities
///
/// Implementations must not use this interface to silently:
///
/// - Select hardware.
/// - Allocate physical resources.
/// - Schedule runtime work.
/// - Route physical operations.
/// - Change source-level semantics.
///
/// # Concurrency
///
/// A resolver may be shared by the caller across validation operations
/// when its implementation supports the required synchronization.
///
/// The validator itself maintains no global mutable state.
pub trait CompilationGraphSemanticResolver {
    /// Resolve and validate a canonical entity reference.
    ///
    /// `node` identifies the graph-local reference.
    ///
    /// `entity` identifies the authoritative repository entity.
    ///
    /// Return `Ok(())` only when the owning authority has successfully
    /// resolved and validated the referenced entity.
    fn validate_entity(
        &self,
        node: &CompilationGraphNode,
        entity: &CanonicalEntityRef,
    ) -> Result<(), String>;

    /// Validate the semantics of a typed directed edge.
    ///
    /// The resolver must interpret the edge kind according to the
    /// authoritative language, semantic, or IR contract.
    ///
    /// Structural direction alone does not establish semantic meaning.
    fn validate_edge(
        &self,
        graph: &CompilationGraph,
        edge: &CompilationGraphEdge,
    ) -> Result<(), String>;

    /// Verify that all relationships required by the authoritative
    /// semantic representation are present and correctly represented.
    ///
    /// Structural validation cannot prove that an edge has not been
    /// omitted from an otherwise valid graph.
    ///
    /// This method is therefore required for full semantic verification.
    fn validate_required_relationships(
        &self,
        graph: &CompilationGraph,
        edges: &CompilationGraphEdgeSet,
    ) -> Result<(), String>;
}

// ============================================================================
// VERIFICATION REPORT
// ============================================================================

/// Result of compilation graph validation.
///
/// Structural validity and semantic completeness are represented
/// independently.
///
/// A structurally valid graph is not automatically semantically complete.
///
/// A semantically complete graph is not automatically feasible on
/// a particular physical target.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompilationGraphValidationReport {
    /// Identity of the validated graph.
    pub graph_id: String,

    /// Graph schema version.
    pub graph_schema_version: u16,

    /// Edge-set schema version.
    pub edge_schema_version: u16,

    /// Number of structurally validated nodes.
    pub validated_nodes: u128,

    /// Number of structurally validated edges.
    pub validated_edges: u128,

    /// Number of canonical entity references resolved by an authority.
    pub resolved_entities: u128,

    /// Number of edges whose semantics were resolved by an authority.
    pub resolved_edges: u128,

    /// Nodes whose canonical entity references were not semantically
    /// resolved because structural-only validation was requested.
    pub unresolved_entities: Vec<CompilationGraphNodeId>,

    /// Edges whose semantics were not semantically resolved because
    /// structural-only validation was requested.
    pub unresolved_edges: Vec<CompilationGraphEdgeId>,

    /// Whether structural graph and edge-set validation completed.
    pub structural_validation_completed: bool,

    /// Whether an authoritative semantic resolver was supplied and
    /// completed its checks.
    pub semantic_validation_completed: bool,

    /// Whether required relationships were checked by an authority.
    pub required_relationships_checked: bool,
}

impl CompilationGraphValidationReport {
    /// Return whether structural validation completed successfully.
    #[must_use]
    pub const fn is_structurally_valid(&self) -> bool {
        self.structural_validation_completed
    }

    /// Return whether all semantic validation obligations completed.
    ///
    /// This requires:
    ///
    /// - Successful structural validation.
    /// - Successful entity resolution.
    /// - Successful edge-semantic validation.
    /// - Successful required-relationship validation.
    #[must_use]
    pub fn is_fully_semantically_valid(&self) -> bool {
        self.structural_validation_completed
            && self.semantic_validation_completed
            && self.required_relationships_checked
            && self.unresolved_entities.is_empty()
            && self.unresolved_edges.is_empty()
    }

    /// Return whether the report contains unresolved entity references.
    #[must_use]
    pub fn has_unresolved_entities(&self) -> bool {
        !self.unresolved_entities.is_empty()
    }

    /// Return whether the report contains unresolved edge semantics.
    #[must_use]
    pub fn has_unresolved_edges(&self) -> bool {
        !self.unresolved_edges.is_empty()
    }

    /// Return whether the graph is structurally validated but still
    /// requires semantic verification.
    #[must_use]
    pub fn requires_semantic_validation(&self) -> bool {
        self.structural_validation_completed
            && !self.is_fully_semantically_valid()
    }
}

// ============================================================================
// VALIDATOR
// ============================================================================

/// Stateless compiler graph validator.
///
/// The validator does not own graph data, semantic registries, target
/// information, caches, or runtime state.
///
/// All authoritative information is supplied through the graph,
/// edge set, and optional semantic resolver.
#[derive(Debug, Default, Clone, Copy)]
pub struct CompilationGraphValidator;

impl CompilationGraphValidator {
    /// Validate graph structure and edge relationships.
    ///
    /// This operation checks:
    ///
    /// - Graph-model invariants.
    /// - Edge-set invariants.
    /// - Graph identity association.
    /// - Source endpoint existence.
    /// - Target endpoint existence.
    ///
    /// It does not resolve canonical entities or prove that all
    /// semantically required dependencies have been represented.
    ///
    /// Cycles, parallel edges, and self-referential edges are preserved
    /// when accepted by the canonical graph and edge-set models.
    pub fn validate_structure(
        graph: &CompilationGraph,
        edges: &CompilationGraphEdgeSet,
    ) -> GraphValidationResult<CompilationGraphValidationReport> {
        graph.validate()?;
        edges.validate_against(graph)?;

        let validated_nodes =
            Self::usize_to_u128(graph.node_count())?;

        let validated_edges =
            Self::usize_to_u128(edges.edge_count())?;

        let unresolved_entities =
            graph.nodes()
                .iter()
                .map(|node| node.id().clone())
                .collect();

        let unresolved_edges =
            edges.edges()
                .iter()
                .map(|edge| edge.id().clone())
                .collect();

        Ok(CompilationGraphValidationReport {
            graph_id: graph.id().to_string(),
            graph_schema_version: graph.schema_version(),
            edge_schema_version: edges.schema_version(),
            validated_nodes,
            validated_edges,
            resolved_entities: 0,
            resolved_edges: 0,
            unresolved_entities,
            unresolved_edges,
            structural_validation_completed: true,
            semantic_validation_completed: false,
            required_relationships_checked: false,
        })
    }

    /// Validate graph structure and authoritative semantics.
    ///
    /// This operation additionally checks:
    ///
    /// - Every node's canonical entity reference.
    /// - Every edge's domain-specific semantics.
    /// - Required relationships from authoritative semantic information.
    ///
    /// The resolver must use the actual owning repository authorities.
    ///
    /// This method does not establish target feasibility or physical
    /// execution availability.
    pub fn validate_with_resolver(
        graph: &CompilationGraph,
        edges: &CompilationGraphEdgeSet,
        resolver: &dyn CompilationGraphSemanticResolver,
    ) -> GraphValidationResult<CompilationGraphValidationReport> {
        // Structural validation is always performed first.
        //
        // A semantic resolver must never be allowed to turn an invalid
        // graph structure into a successful result.
        let mut report =
            Self::validate_structure(graph, edges)?;

        // Resolve graph nodes in declaration order.
        for node in graph.nodes() {
            resolver
                .validate_entity(node, node.entity())
                .map_err(|reason| {
                    GraphValidationError::EntitySemanticInvalid {
                        node_id: node.id().to_string(),
                        authority: node.entity().authority().to_owned(),
                        entity_kind: node.entity().entity_kind().to_owned(),
                        entity_id: node.entity().entity_id().to_owned(),
                        reason,
                    }
                })?;

            report.resolved_entities =
                report.resolved_entities
                    .checked_add(1)
                    .ok_or(GraphValidationError::SizeOverflow)?;
        }

        // Resolve edges in declaration order.
        for edge in edges.edges() {
            resolver
                .validate_edge(graph, edge)
                .map_err(|reason| {
                    GraphValidationError::EdgeSemanticInvalid {
                        edge_id: edge.id().to_string(),
                        edge_kind: edge.kind().as_str().to_owned(),
                        reason,
                    }
                })?;

            report.resolved_edges =
                report.resolved_edges
                    .checked_add(1)
                    .ok_or(GraphValidationError::SizeOverflow)?;
        }

        // Structural presence of edges does not prove completeness.
        // The owning semantic authority must verify required relationships.
        resolver
            .validate_required_relationships(graph, edges)
            .map_err(|reason| {
                GraphValidationError::RequiredRelationshipsInvalid {
                    reason,
                }
            })?;

        report.unresolved_entities.clear();
        report.unresolved_edges.clear();

        report.semantic_validation_completed = true;
        report.required_relationships_checked = true;

        Ok(report)
    }

    /// Validate a graph structurally and semantically, returning only
    /// success or a structured error.
    ///
    /// This convenience method is useful for compiler pipeline stages
    /// that do not need to retain the full report.
    pub fn ensure_fully_valid(
        graph: &CompilationGraph,
        edges: &CompilationGraphEdgeSet,
        resolver: &dyn CompilationGraphSemanticResolver,
    ) -> GraphValidationResult<()> {
        let report =
            Self::validate_with_resolver(
                graph,
                edges,
                resolver,
            )?;

        if !report.is_fully_semantically_valid() {
            return Err(
                GraphValidationError::RequiredRelationshipsInvalid {
                    reason: String::from(
                        "semantic validation did not establish \
                         all required validation guarantees",
                    ),
                },
            );
        }

        Ok(())
    }

    fn usize_to_u128(
        value: usize,
    ) -> GraphValidationResult<u128> {
        u128::try_from(value)
            .map_err(|_| GraphValidationError::SizeOverflow)
    }
}

// ============================================================================
// PUBLIC CONVENIENCE FUNCTIONS
// ============================================================================

/// Validate the structural integrity of a compilation graph and edge set.
pub fn validate_compilation_graph(
    graph: &CompilationGraph,
    edges: &CompilationGraphEdgeSet,
) -> GraphValidationResult<CompilationGraphValidationReport> {
    CompilationGraphValidator::validate_structure(
        graph,
        edges,
    )
}

/// Validate the structural and semantic integrity of a compilation graph
/// through its authoritative semantic resolver.
pub fn validate_compilation_graph_with_resolver(
    graph: &CompilationGraph,
    edges: &CompilationGraphEdgeSet,
    resolver: &dyn CompilationGraphSemanticResolver,
) -> GraphValidationResult<CompilationGraphValidationReport> {
    CompilationGraphValidator::validate_with_resolver(
        graph,
        edges,
        resolver,
    )
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validation_api_version_is_nonzero() {
        assert_ne!(
            COMPILATION_GRAPH_VALIDATION_API_VERSION,
            0
        );
    }

    #[test]
    fn validation_schema_version_is_nonzero() {
        assert_ne!(
            COMPILATION_GRAPH_VALIDATION_SCHEMA_VERSION,
            0
        );
    }

    #[test]
    fn report_does_not_confuse_structural_and_semantic_validity() {
        let report = CompilationGraphValidationReport {
            graph_id: String::from("graph"),
            graph_schema_version: 1,
            edge_schema_version: 1,
            validated_nodes: 2,
            validated_edges: 1,
            resolved_entities: 0,
            resolved_edges: 0,
            unresolved_entities: Vec::new(),
            unresolved_edges: Vec::new(),
            structural_validation_completed: true,
            semantic_validation_completed: false,
            required_relationships_checked: false,
        };

        assert!(report.is_structurally_valid());
        assert!(!report.is_fully_semantically_valid());
        assert!(report.requires_semantic_validation());
    }

    #[test]
    fn semantic_validity_requires_required_relationship_checks() {
        let report = CompilationGraphValidationReport {
            graph_id: String::from("graph"),
            graph_schema_version: 1,
            edge_schema_version: 1,
            validated_nodes: 1,
            validated_edges: 0,
            resolved_entities: 1,
            resolved_edges: 0,
            unresolved_entities: Vec::new(),
            unresolved_edges: Vec::new(),
            structural_validation_completed: true,
            semantic_validation_completed: true,
            required_relationships_checked: false,
        };

        assert!(!report.is_fully_semantically_valid());
    }

    #[test]
    fn semantic_validity_requires_structural_validation() {
        let report = CompilationGraphValidationReport {
            graph_id: String::from("graph"),
            graph_schema_version: 1,
            edge_schema_version: 1,
            validated_nodes: 0,
            validated_edges: 0,
            resolved_entities: 0,
            resolved_edges: 0,
            unresolved_entities: Vec::new(),
            unresolved_edges: Vec::new(),
            structural_validation_completed: false,
            semantic_validation_completed: true,
            required_relationships_checked: true,
        };

        assert!(!report.is_fully_semantically_valid());
    }

    #[test]
    fn unresolved_entities_prevent_full_semantic_validity() {
        let unresolved = CompilationGraphNodeId::new(
            "node-unresolved",
        )
        .expect("valid node identity");

        let report = CompilationGraphValidationReport {
            graph_id: String::from("graph"),
            graph_schema_version: 1,
            edge_schema_version: 1,
            validated_nodes: 1,
            validated_edges: 0,
            resolved_entities: 0,
            resolved_edges: 0,
            unresolved_entities: vec![unresolved],
            unresolved_edges: Vec::new(),
            structural_validation_completed: true,
            semantic_validation_completed: true,
            required_relationships_checked: true,
        };

        assert!(!report.is_fully_semantically_valid());
        assert!(report.has_unresolved_entities());
    }

    #[test]
    fn unresolved_edges_prevent_full_semantic_validity() {
        let unresolved = CompilationGraphEdgeId::new(
            "edge-unresolved",
        )
        .expect("valid edge identity");

        let report = CompilationGraphValidationReport {
            graph_id: String::from("graph"),
            graph_schema_version: 1,
            edge_schema_version: 1,
            validated_nodes: 1,
            validated_edges: 1,
            resolved_entities: 1,
            resolved_edges: 0,
            unresolved_entities: Vec::new(),
            unresolved_edges: vec![unresolved],
            structural_validation_completed: true,
            semantic_validation_completed: true,
            required_relationships_checked: true,
        };

        assert!(!report.is_fully_semantically_valid());
        assert!(report.has_unresolved_edges());
    }

    #[test]
    fn complete_report_is_fully_semantically_valid() {
        let report = CompilationGraphValidationReport {
            graph_id: String::from("graph"),
            graph_schema_version: 1,
            edge_schema_version: 1,
            validated_nodes: 2,
            validated_edges: 1,
            resolved_entities: 2,
            resolved_edges: 1,
            unresolved_entities: Vec::new(),
            unresolved_edges: Vec::new(),
            structural_validation_completed: true,
            semantic_validation_completed: true,
            required_relationships_checked: true,
        };

        assert!(report.is_structurally_valid());
        assert!(report.is_fully_semantically_valid());
        assert!(!report.requires_semantic_validation());
    }

    #[test]
    fn graph_identity_mismatch_error_is_descriptive() {
        let error = GraphValidationError::GraphIdentityMismatch {
            expected: String::from("graph-a"),
            actual: String::from("graph-b"),
        };

        let message = error.to_string();

        assert!(message.contains("graph-a"));
        assert!(message.contains("graph-b"));
    }

    #[test]
    fn semantic_error_is_descriptive() {
        let error = GraphValidationError::EntitySemanticInvalid {
            node_id: String::from("node-1"),
            authority: String::from("crate::zuir"),
            entity_kind: String::from("operation"),
            entity_id: String::from("op-1"),
            reason: String::from("entity not found"),
        };

        let message = error.to_string();

        assert!(message.contains("node-1"));
        assert!(message.contains("crate::zuir"));
        assert!(message.contains("entity not found"));
    }
}
