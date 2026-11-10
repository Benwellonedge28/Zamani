
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani Compiler — Graph Transformations
//!
//! Path: src/compiler/graph/transformation.rs
//! Rust edition: 2021
//! Minimum supported Rust version: 1.97
//! Safety: unsafe Rust is forbidden.
//!
//! ## Purpose
//!
//! Provides immutable, declarative transformations over the canonical
//! compiler graph and its separately owned edge set.
//!
//! A transformation is a sequence of graph edits applied to an existing
//! graph. The operation either produces a completely validated new graph
//! and edge set or returns an error. The input objects are never mutated.
//!
//! ## Canonical ownership
//!
//! This module owns:
//!
//! - GraphEdit.
//! - GraphTransformation.
//! - TransformedCompilationGraph.
//! - GraphTransformationError.
//! - Atomic application of graph edits.
//!
//! It does not own:
//!
//! - CompilationGraph or graph-node semantics.
//! - CompilationGraphEdgeSet or edge semantics.
//! - Source grammar, AST or semantic analysis.
//! - ZUIR or domain-specific IR.
//! - Pipeline scheduling or execution.
//! - Physical resource allocation.
//! - Hardware discovery.
//! - Routing or placement.
//! - Backend code generation.
//! - Runtime execution.
//!
//! ## Canonical integration
//!
//! Graph model:
//!     super::model
//!
//! Graph relationships:
//!     super::edges
//!
//! Compiler dependency analysis:
//!     crate::compiler::analysis::dependency
//!
//! Pipeline dependency ordering:
//!     crate::compiler::pipeline::ordering
//!
//! Parallel compilation:
//!     crate::compiler::parallel::dependencies
//!
//! Incremental compilation:
//!     crate::compiler::incremental::invalidation
//!
//! Hybrid computation:
//!     crate::hybrid
//!
//! Canonical domain IRs:
//!     crate::zuir
//!     crate::classical
//!     crate::quantum::ir
//!     crate::hdl::hdlir
//!     crate::ai
//!
//! ## Scalability
//!
//! No arbitrary maximum is imposed on:
//!
//! - Graph nodes.
//! - Graph edges.
//! - Transformation operations.
//! - Computational domains.
//! - Node classifications.
//! - Edge classifications.
//! - Metadata entries.
//!
//! Actual limits are determined by available memory, address space,
//! allocation behavior, execution time and operational budgets owned
//! by the appropriate higher-level subsystem.
//!
//! Resource exhaustion is not semantic invalidity.
//!
//! ## Determinism
//!
//! - Existing node declaration order is preserved.
//! - Existing edge declaration order is preserved.
//! - Newly added nodes and edges are appended in transformation order.
//! - Replacement preserves the original position.
//! - Metadata remains owned by the canonical graph and edge models.
//! - Ordered maps are used for transformation working indexes.
//!
//! ## Atomicity
//!
//! Edits are applied to private working collections.
//!
//! The canonical graph and edge set are reconstructed and validated only
//! after all edits have been processed.
//!
//! If any operation or final validation fails, no transformed graph is
//! returned and the original graph remains unchanged.
//!
//! ## Cycle policy
//!
//! This module does not impose a DAG restriction.
//!
//! Feedback edges, self-edges, parallel edges and cycles are preserved
//! when accepted by the canonical edge model.
//!
//! Pipeline ordering and other consumers must apply their own
//! relationship-specific cycle rules.

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

use super::edges::{
    CompilationGraphEdge,
    CompilationGraphEdgeId,
    CompilationGraphEdgeSet,
    GraphEdgesError,
};

use super::model::{
    CompilationGraph,
    CompilationGraphId,
    CompilationGraphNode,
    CompilationGraphNodeId,
    GraphModelError,
};

// ============================================================================
// API VERSION
// ============================================================================

/// Public API version of compiler graph transformations.
pub const COMPILATION_GRAPH_TRANSFORMATION_API_VERSION: u16 = 1;

// ============================================================================
// RESULT
// ============================================================================

/// Result returned by graph transformation operations.
pub type GraphTransformationResult<T> =
    Result<T, GraphTransformationError>;

// ============================================================================
// ERRORS
// ============================================================================

/// Errors produced by graph transformation operations.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum GraphTransformationError {
    /// The graph and edge set do not identify the same graph.
    GraphIdentityMismatch {
        expected: String,
        actual: String,
    },

    /// The graph schema differs from the schema used to create
    /// the transformation.
    GraphSchemaMismatch {
        expected: u16,
        actual: u16,
    },

    /// The edge schema differs from the schema used to create
    /// the transformation.
    EdgeSchemaMismatch {
        expected: u16,
        actual: u16,
    },

    /// A node already exists when an addition is requested.
    DuplicateNode {
        node_id: String,
    },

    /// A node does not exist when replacement or removal is requested.
    MissingNode {
        node_id: String,
    },

    /// An edge already exists when an addition is requested.
    DuplicateEdge {
        edge_id: String,
    },

    /// An edge does not exist when replacement or removal is requested.
    MissingEdge {
        edge_id: String,
    },

    /// A replacement node attempted to change its identity.
    ReplacementNodeIdentityMismatch {
        expected: String,
        actual: String,
    },

    /// A replacement edge attempted to change its identity.
    ReplacementEdgeIdentityMismatch {
        expected: String,
        actual: String,
    },

    /// A node cannot be removed while retained edges reference it.
    NodeHasIncidentEdges {
        node_id: String,
        edge_ids: Vec<String>,
    },

    /// An operation failed canonical graph-model validation.
    GraphModel(GraphModelError),

    /// An operation failed canonical edge validation.
    GraphEdges(GraphEdgesError),
}

impl fmt::Display for GraphTransformationError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::GraphIdentityMismatch {
                expected,
                actual,
            } => {
                write!(
                    formatter,
                    "graph identity mismatch: expected \
                     '{expected}', received '{actual}'"
                )
            }

            Self::GraphSchemaMismatch {
                expected,
                actual,
            } => {
                write!(
                    formatter,
                    "graph schema mismatch: expected \
                     {expected}, received {actual}"
                )
            }

            Self::EdgeSchemaMismatch {
                expected,
                actual,
            } => {
                write!(
                    formatter,
                    "edge schema mismatch: expected \
                     {expected}, received {actual}"
                )
            }

            Self::DuplicateNode { node_id } => {
                write!(
                    formatter,
                    "cannot add existing graph node '{node_id}'"
                )
            }

            Self::MissingNode { node_id } => {
                write!(
                    formatter,
                    "graph node '{node_id}' does not exist"
                )
            }

            Self::DuplicateEdge { edge_id } => {
                write!(
                    formatter,
                    "cannot add existing graph edge '{edge_id}'"
                )
            }

            Self::MissingEdge { edge_id } => {
                write!(
                    formatter,
                    "graph edge '{edge_id}' does not exist"
                )
            }

            Self::ReplacementNodeIdentityMismatch {
                expected,
                actual,
            } => {
                write!(
                    formatter,
                    "replacement node identity mismatch: \
                     expected '{expected}', received '{actual}'"
                )
            }

            Self::ReplacementEdgeIdentityMismatch {
                expected,
                actual,
            } => {
                write!(
                    formatter,
                    "replacement edge identity mismatch: \
                     expected '{expected}', received '{actual}'"
                )
            }

            Self::NodeHasIncidentEdges {
                node_id,
                edge_ids,
            } => {
                write!(
                    formatter,
                    "cannot remove node '{node_id}': \
                     {} incident edge(s) remain",
                    edge_ids.len()
                )
            }

            Self::GraphModel(error) => {
                write!(
                    formatter,
                    "graph model validation failed: {error}"
                )
            }

            Self::GraphEdges(error) => {
                write!(
                    formatter,
                    "graph edge validation failed: {error}"
                )
            }
        }
    }
}

impl Error for GraphTransformationError {}

impl From<GraphModelError> for GraphTransformationError {
    fn from(error: GraphModelError) -> Self {
        Self::GraphModel(error)
    }
}

impl From<GraphEdgesError> for GraphTransformationError {
    fn from(error: GraphEdgesError) -> Self {
        Self::GraphEdges(error)
    }
}

// ============================================================================
// DECLARATIVE EDITS
// ============================================================================

/// One declarative operation against a compiler graph.
///
/// Node and edge identity is owned by the canonical model.
///
/// This enum deliberately does not define domain-specific operation kinds.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GraphEdit {
    /// Add a new node.
    AddNode {
        node: CompilationGraphNode,
    },

    /// Replace an existing node without changing its identity.
    ReplaceNode {
        node_id: CompilationGraphNodeId,
        replacement: CompilationGraphNode,
    },

    /// Remove an existing node.
    ///
    /// All incident edges must be explicitly removed in the same
    /// transformation before the final graph can be produced.
    RemoveNode {
        node_id: CompilationGraphNodeId,
    },

    /// Add a new edge.
    AddEdge {
        edge: CompilationGraphEdge,
    },

    /// Replace an existing edge without changing its identity.
    ReplaceEdge {
        edge_id: CompilationGraphEdgeId,
        replacement: CompilationGraphEdge,
    },

    /// Remove an existing edge.
    RemoveEdge {
        edge_id: CompilationGraphEdgeId,
    },
}

// ============================================================================
// TRANSFORMATION
// ============================================================================

/// Immutable declarative transformation of a compiler graph.
///
/// A transformation is associated with:
///
/// - One graph identity.
/// - One graph schema version.
/// - One edge schema version.
///
/// It may be applied to a graph and edge set with matching identities
/// and schemas.
///
/// Matching identities and schemas do not prove that the supplied graph
/// is the exact content revision from which the transformation was planned.
/// Incremental-compilation and versioning authorities must manage that
/// stronger concurrency guarantee.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphTransformation {
    graph_id: CompilationGraphId,
    graph_schema_version: u16,
    edge_schema_version: u16,
    edits: Vec<GraphEdit>,
}

impl GraphTransformation {
    /// Create an empty transformation associated with a validated graph.
    ///
    /// The graph and edge set are validated before the transformation
    /// can be created.
    pub fn new(
        graph: &CompilationGraph,
        edges: &CompilationGraphEdgeSet,
    ) -> GraphTransformationResult<Self> {
        graph.validate()?;
        edges.validate_against(graph)?;

        Ok(Self {
            graph_id: graph.id().clone(),
            graph_schema_version: graph.schema_version(),
            edge_schema_version: edges.schema_version(),
            edits: Vec::new(),
        })
    }

    /// Return the graph identity this transformation targets.
    #[must_use]
    pub fn graph_id(&self) -> &CompilationGraphId {
        &self.graph_id
    }

    /// Return the graph schema version this transformation targets.
    #[must_use]
    pub const fn graph_schema_version(&self) -> u16 {
        self.graph_schema_version
    }

    /// Return the edge schema version this transformation targets.
    #[must_use]
    pub const fn edge_schema_version(&self) -> u16 {
        self.edge_schema_version
    }

    /// Return all edits in declaration order.
    #[must_use]
    pub fn edits(&self) -> &[GraphEdit] {
        &self.edits
    }

    /// Return the number of declared edits.
    #[must_use]
    pub fn edit_count(&self) -> usize {
        self.edits.len()
    }

    /// Return whether the transformation has no edits.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.edits.is_empty()
    }

    /// Append an addition of a graph node.
    pub fn add_node(
        &mut self,
        node: CompilationGraphNode,
    ) -> &mut Self {
        self.edits.push(GraphEdit::AddNode { node });
        self
    }

    /// Append a replacement of an existing graph node.
    pub fn replace_node(
        &mut self,
        node_id: CompilationGraphNodeId,
        replacement: CompilationGraphNode,
    ) -> &mut Self {
        self.edits.push(
            GraphEdit::ReplaceNode {
                node_id,
                replacement,
            },
        );
        self
    }

    /// Append removal of an existing graph node.
    ///
    /// Any incident edges must also be explicitly removed.
    pub fn remove_node(
        &mut self,
        node_id: CompilationGraphNodeId,
    ) -> &mut Self {
        self.edits.push(
            GraphEdit::RemoveNode { node_id },
        );
        self
    }

    /// Append addition of a graph edge.
    pub fn add_edge(
        &mut self,
        edge: CompilationGraphEdge,
    ) -> &mut Self {
        self.edits.push(GraphEdit::AddEdge { edge });
        self
    }

    /// Append replacement of an existing graph edge.
    pub fn replace_edge(
        &mut self,
        edge_id: CompilationGraphEdgeId,
        replacement: CompilationGraphEdge,
    ) -> &mut Self {
        self.edits.push(
            GraphEdit::ReplaceEdge {
                edge_id,
                replacement,
            },
        );
        self
    }

    /// Append removal of an existing graph edge.
    pub fn remove_edge(
        &mut self,
        edge_id: CompilationGraphEdgeId,
    ) -> &mut Self {
        self.edits.push(
            GraphEdit::RemoveEdge { edge_id },
        );
        self
    }

    /// Apply the transformation and return a new validated graph.
    ///
    /// The input graph and edge set are never mutated.
    ///
    /// The operation is atomic from the caller's perspective:
    ///
    /// - Every edit is applied to private working collections.
    /// - The final graph is reconstructed and validated.
    /// - The final edge set is reconstructed and validated.
    /// - No partial result is returned on failure.
    pub fn apply(
        &self,
        graph: &CompilationGraph,
        edges: &CompilationGraphEdgeSet,
    ) -> GraphTransformationResult<TransformedCompilationGraph> {
        self.validate_input(graph, edges)?;

        // Ordered indexes provide identity lookup without making
        // declaration order depend on map iteration.
        let mut nodes: BTreeMap<
            CompilationGraphNodeId,
            CompilationGraphNode,
        > = BTreeMap::new();

        let mut node_order: Vec<CompilationGraphNodeId> =
            Vec::with_capacity(graph.node_count());

        for node in graph.nodes() {
            let id = node.id().clone();

            nodes.insert(id.clone(), node.clone());
            node_order.push(id);
        }

        let mut edge_map: BTreeMap<
            CompilationGraphEdgeId,
            CompilationGraphEdge,
        > = BTreeMap::new();

        let mut edge_order: Vec<CompilationGraphEdgeId> =
            Vec::with_capacity(edges.edge_count());

        for edge in edges.edges() {
            let id = edge.id().clone();

            edge_map.insert(id.clone(), edge.clone());
            edge_order.push(id);
        }

        // Apply all operations to private working state.
        for edit in &self.edits {
            match edit {
                GraphEdit::AddNode { node } => {
                    node.validate()?;

                    let id = node.id().clone();

                    if nodes.contains_key(&id) {
                        return Err(
                            GraphTransformationError::DuplicateNode {
                                node_id: id.to_string(),
                            },
                        );
                    }

                    nodes.insert(id.clone(), node.clone());
                    node_order.push(id);
                }

                GraphEdit::ReplaceNode {
                    node_id,
                    replacement,
                } => {
                    replacement.validate()?;

                    if replacement.id() != node_id {
                        return Err(
                            GraphTransformationError::
                                ReplacementNodeIdentityMismatch {
                                    expected: node_id.to_string(),
                                    actual: replacement.id().to_string(),
                                },
                        );
                    }

                    if !nodes.contains_key(node_id) {
                        return Err(
                            GraphTransformationError::MissingNode {
                                node_id: node_id.to_string(),
                            },
                        );
                    }

                    nodes.insert(
                        node_id.clone(),
                        replacement.clone(),
                    );
                }

                GraphEdit::RemoveNode { node_id } => {
                    if nodes.remove(node_id).is_none() {
                        return Err(
                            GraphTransformationError::MissingNode {
                                node_id: node_id.to_string(),
                            },
                        );
                    }

                    node_order.retain(|id| id != node_id);
                }

                GraphEdit::AddEdge { edge } => {
                    edge.validate()?;

                    let id = edge.id().clone();

                    if edge_map.contains_key(&id) {
                        return Err(
                            GraphTransformationError::DuplicateEdge {
                                edge_id: id.to_string(),
                            },
                        );
                    }

                    edge_map.insert(id.clone(), edge.clone());
                    edge_order.push(id);
                }

                GraphEdit::ReplaceEdge {
                    edge_id,
                    replacement,
                } => {
                    replacement.validate()?;

                    if replacement.id() != edge_id {
                        return Err(
                            GraphTransformationError::
                                ReplacementEdgeIdentityMismatch {
                                    expected: edge_id.to_string(),
                                    actual: replacement.id().to_string(),
                                },
                        );
                    }

                    if !edge_map.contains_key(edge_id) {
                        return Err(
                            GraphTransformationError::MissingEdge {
                                edge_id: edge_id.to_string(),
                            },
                        );
                    }

                    edge_map.insert(
                        edge_id.clone(),
                        replacement.clone(),
                    );
                }

                GraphEdit::RemoveEdge { edge_id } => {
                    if edge_map.remove(edge_id).is_none() {
                        return Err(
                            GraphTransformationError::MissingEdge {
                                edge_id: edge_id.to_string(),
                            },
                        );
                    }

                    edge_order.retain(|id| id != edge_id);
                }
            }
        }

        // Rebuild nodes in preserved declaration order.
        let mut final_nodes = Vec::with_capacity(node_order.len());

        for id in &node_order {
            let node = nodes
                .get(id)
                .ok_or_else(|| {
                    GraphTransformationError::MissingNode {
                        node_id: id.to_string(),
                    }
                })?;

            final_nodes.push(node.clone());
        }

        // Rebuild edges in preserved declaration order.
        let mut final_edges = Vec::with_capacity(edge_order.len());

        for id in &edge_order {
            let edge = edge_map
                .get(id)
                .ok_or_else(|| {
                    GraphTransformationError::MissingEdge {
                        edge_id: id.to_string(),
                    }
                })?;

            final_edges.push(edge.clone());
        }

        // Give a precise error when a removed node still has
        // retained incident edges.
        Self::validate_no_dangling_edges(
            &nodes,
            &final_edges,
        )?;

        // Reconstruct the canonical graph through its validated API.
        let transformed_graph = CompilationGraph::new_with_schema_version(
            graph.id().clone(),
            graph.schema_version(),
            final_nodes,
            graph.metadata().clone(),
        )?;

        // Reconstruct the canonical edge set through its validated API.
        let transformed_edges = CompilationGraphEdgeSet::from_parts(
            transformed_graph.id().clone(),
            edges.schema_version(),
            final_edges,
            edges.metadata().clone(),
        )?;

        // Validate endpoint existence and graph association.
        transformed_edges.validate_against(&transformed_graph)?;

        Ok(TransformedCompilationGraph {
            graph: transformed_graph,
            edges: transformed_edges,
        })
    }

    /// Validate that the supplied graph and edge set match this
    /// transformation's identity and schema contract.
    fn validate_input(
        &self,
        graph: &CompilationGraph,
        edges: &CompilationGraphEdgeSet,
    ) -> GraphTransformationResult<()> {
        graph.validate()?;
        edges.validate_against(graph)?;

        if graph.id() != &self.graph_id {
            return Err(
                GraphTransformationError::GraphIdentityMismatch {
                    expected: self.graph_id.to_string(),
                    actual: graph.id().to_string(),
                },
            );
        }

        if graph.schema_version() != self.graph_schema_version {
            return Err(
                GraphTransformationError::GraphSchemaMismatch {
                    expected: self.graph_schema_version,
                    actual: graph.schema_version(),
                },
            );
        }

        if edges.schema_version() != self.edge_schema_version {
            return Err(
                GraphTransformationError::EdgeSchemaMismatch {
                    expected: self.edge_schema_version,
                    actual: edges.schema_version(),
                },
            );
        }

        Ok(())
    }

    /// Ensure that every retained edge references retained nodes.
    fn validate_no_dangling_edges(
        nodes: &BTreeMap<
            CompilationGraphNodeId,
            CompilationGraphNode,
        >,
        edges: &[CompilationGraphEdge],
    ) -> GraphTransformationResult<()> {
        for edge in edges {
            let source_exists = nodes.contains_key(edge.source());
            let target_exists = nodes.contains_key(edge.target());

            if source_exists && target_exists {
                continue;
            }

            let missing_node = if !source_exists {
                edge.source()
            } else {
                edge.target()
            };

            let mut incident_edge_ids = edges
                .iter()
                .filter(|candidate| {
                    candidate.source() == missing_node
                        || candidate.target() == missing_node
                })
                .map(|candidate| candidate.id().to_string())
                .collect::<Vec<_>>();

            incident_edge_ids.sort();

            return Err(
                GraphTransformationError::NodeHasIncidentEdges {
                    node_id: missing_node.to_string(),
                    edge_ids: incident_edge_ids,
                },
            );
        }

        Ok(())
    }
}

// ============================================================================
// TRANSFORMED RESULT
// ============================================================================

/// A successfully transformed, structurally validated graph and edge set.
///
/// The graph and its edges remain separate canonical objects.
/// This wrapper only guarantees that they belong together and that
/// their structural relationship has been validated.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransformedCompilationGraph {
    graph: CompilationGraph,
    edges: CompilationGraphEdgeSet,
}

impl TransformedCompilationGraph {
    /// Return the transformed canonical graph.
    #[must_use]
    pub const fn graph(&self) -> &CompilationGraph {
        &self.graph
    }

    /// Return the transformed canonical edge set.
    #[must_use]
    pub const fn edges(&self) -> &CompilationGraphEdgeSet {
        &self.edges
    }

    /// Consume the result and return its canonical parts.
    #[must_use]
    pub fn into_parts(
        self,
    ) -> (CompilationGraph, CompilationGraphEdgeSet) {
        (self.graph, self.edges)
    }

    /// Revalidate the graph and its edge set.
    pub fn validate(&self) -> GraphTransformationResult<()> {
        self.graph.validate()?;
        self.edges.validate_against(&self.graph)?;
        Ok(())
    }

    /// Return the total number of graph nodes.
    #[must_use]
    pub fn node_count(&self) -> usize {
        self.graph.node_count()
    }

    /// Return the total number of graph edges.
    #[must_use]
    pub fn edge_count(&self) -> usize {
        self.edges.edge_count()
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::BTreeMap;

    use super::super::edges::{
        CompilationGraphEdge,
        CompilationGraphEdgeKind,
    };

    use super::super::model::{
        CanonicalEntityRef,
        CompilationGraphNodeKind,
    };

    fn graph() -> CompilationGraph {
        CompilationGraph::new(
            CompilationGraphId::new("test.graph")
                .expect("valid graph identity"),
            vec![
                node("a"),
                node("b"),
                node("c"),
            ],
            BTreeMap::new(),
        )
        .expect("valid graph")
    }

    fn node(id: &str) -> CompilationGraphNode {
        CompilationGraphNode::new(
            CompilationGraphNodeId::new(id)
                .expect("valid node identity"),
            CompilationGraphNodeKind::new(
                "compiler.test.node",
            )
            .expect("valid node kind"),
            CanonicalEntityRef::new(
                "compiler.test",
                "node",
                id,
            )
            .expect("valid entity reference"),
            None,
            BTreeMap::new(),
        )
        .expect("valid node")
    }

    fn edge(
        id: &str,
        kind: &str,
        source: &str,
        target: &str,
    ) -> CompilationGraphEdge {
        CompilationGraphEdge::new(
            CompilationGraphEdgeId::new(id)
                .expect("valid edge identity"),
            CompilationGraphEdgeKind::new(kind)
                .expect("valid edge kind"),
            CompilationGraphNodeId::new(source)
                .expect("valid source"),
            CompilationGraphNodeId::new(target)
                .expect("valid target"),
            BTreeMap::new(),
        )
        .expect("valid edge")
    }

    fn edge_set(
        graph: &CompilationGraph,
    ) -> CompilationGraphEdgeSet {
        CompilationGraphEdgeSet::new(
            graph,
            vec![
                edge(
                    "edge.ab",
                    "dependency.data",
                    "a",
                    "b",
                ),
                edge(
                    "edge.bc",
                    "dependency.control",
                    "b",
                    "c",
                ),
            ],
            BTreeMap::new(),
        )
        .expect("valid edge set")
    }

    fn node_id(value: &str) -> CompilationGraphNodeId {
        CompilationGraphNodeId::new(value)
            .expect("valid node identity")
    }

    fn edge_id(value: &str) -> CompilationGraphEdgeId {
        CompilationGraphEdgeId::new(value)
            .expect("valid edge identity")
    }

    #[test]
    fn empty_transformation_preserves_graph() {
        let graph = graph();
        let edges = edge_set(&graph);

        let transformation =
            GraphTransformation::new(&graph, &edges)
                .expect("valid transformation");

        let result = transformation
            .apply(&graph, &edges)
            .expect("empty transformation succeeds");

        assert_eq!(result.graph(), &graph);
        assert_eq!(result.edges(), &edges);
    }

    #[test]
    fn adding_node_preserves_existing_declaration_order() {
        let graph = graph();
        let edges = edge_set(&graph);

        let mut transformation =
            GraphTransformation::new(&graph, &edges)
                .expect("valid transformation");

        transformation.add_node(node("d"));

        let result = transformation
            .apply(&graph, &edges)
            .expect("node addition succeeds");

        let ids: Vec<_> = result
            .graph()
            .nodes()
            .iter()
            .map(|node| node.id().as_str())
            .collect();

        assert_eq!(ids, vec!["a", "b", "c", "d"]);
    }

    #[test]
    fn replacing_node_preserves_its_position() {
        let graph = graph();
        let edges = edge_set(&graph);

        let replacement = CompilationGraphNode::new(
            node_id("b"),
            CompilationGraphNodeKind::new(
                "compiler.test.replacement",
            )
            .expect("valid kind"),
            CanonicalEntityRef::new(
                "compiler.test",
                "replacement",
                "b",
            )
            .expect("valid entity"),
            Some("replacement".to_string()),
            BTreeMap::new(),
        )
        .expect("valid replacement");

        let mut transformation =
            GraphTransformation::new(&graph, &edges)
                .expect("valid transformation");

        transformation.replace_node(
            node_id("b"),
            replacement,
        );

        let result = transformation
            .apply(&graph, &edges)
            .expect("replacement succeeds");

        assert_eq!(
            result.graph().nodes()[1].kind().as_str(),
            "compiler.test.replacement"
        );
    }

    #[test]
    fn removing_node_requires_explicit_edge_removal() {
        let graph = graph();
        let edges = edge_set(&graph);

        let mut transformation =
            GraphTransformation::new(&graph, &edges)
                .expect("valid transformation");

        transformation.remove_node(node_id("b"));

        let result = transformation.apply(&graph, &edges);

        assert!(matches!(
            result,
            Err(
                GraphTransformationError::NodeHasIncidentEdges {
                    ..
                }
            )
        ));
    }

    #[test]
    fn node_and_incident_edges_can_be_removed_atomically() {
        let graph = graph();
        let edges = edge_set(&graph);

        let mut transformation =
            GraphTransformation::new(&graph, &edges)
                .expect("valid transformation");

        transformation
            .remove_edge(edge_id("edge.ab"))
            .remove_edge(edge_id("edge.bc"))
            .remove_node(node_id("b"));

        let result = transformation
            .apply(&graph, &edges)
            .expect("complete removal succeeds");

        assert_eq!(result.node_count(), 2);
        assert_eq!(result.edge_count(), 0);
        assert!(result.validate().is_ok());
    }

    #[test]
    fn adding_edge_between_existing_nodes_succeeds() {
        let graph = graph();
        let edges = edge_set(&graph);

        let mut transformation =
            GraphTransformation::new(&graph, &edges)
                .expect("valid transformation");

        transformation.add_edge(
            edge(
                "edge.ac",
                "dependency.data",
                "a",
                "c",
            ),
        );

        let result = transformation
            .apply(&graph, &edges)
            .expect("edge addition succeeds");

        assert_eq!(result.edge_count(), 3);
    }

    #[test]
    fn adding_node_and_edge_in_one_transformation_succeeds() {
        let graph = graph();
        let edges = edge_set(&graph);

        let mut transformation =
            GraphTransformation::new(&graph, &edges)
                .expect("valid transformation");

        transformation
            .add_node(node("d"))
            .add_edge(
                edge(
                    "edge.cd",
                    "dependency.data",
                    "c",
                    "d",
                ),
            );

        let result = transformation
            .apply(&graph, &edges)
            .expect("combined transformation succeeds");

        assert_eq!(result.node_count(), 4);
        assert_eq!(result.edge_count(), 3);
    }

    #[test]
    fn duplicate_node_is_rejected() {
        let graph = graph();
        let edges = edge_set(&graph);

        let mut transformation =
            GraphTransformation::new(&graph, &edges)
                .expect("valid transformation");

        transformation.add_node(node("a"));

        assert!(matches!(
            transformation.apply(&graph, &edges),
            Err(GraphTransformationError::DuplicateNode { .. })
        ));
    }

    #[test]
    fn duplicate_edge_is_rejected() {
        let graph = graph();
        let edges = edge_set(&graph);

        let mut transformation =
            GraphTransformation::new(&graph, &edges)
                .expect("valid transformation");

        transformation.add_edge(
            edge(
                "edge.ab",
                "dependency.data",
                "a",
                "b",
            ),
        );

        assert!(matches!(
            transformation.apply(&graph, &edges),
            Err(GraphTransformationError::DuplicateEdge { .. })
        ));
    }

    #[test]
    fn missing_node_replacement_is_rejected() {
        let graph = graph();
        let edges = edge_set(&graph);

        let mut transformation =
            GraphTransformation::new(&graph, &edges)
                .expect("valid transformation");

        transformation.replace_node(
            node_id("missing"),
            node("missing"),
        );

        assert!(matches!(
            transformation.apply(&graph, &edges),
            Err(GraphTransformationError::MissingNode { .. })
        ));
    }

    #[test]
    fn missing_edge_removal_is_rejected() {
        let graph = graph();
        let edges = edge_set(&graph);

        let mut transformation =
            GraphTransformation::new(&graph, &edges)
                .expect("valid transformation");

        transformation.remove_edge(edge_id("missing"));

        assert!(matches!(
            transformation.apply(&graph, &edges),
            Err(GraphTransformationError::MissingEdge { .. })
        ));
    }

    #[test]
    fn failed_transformation_does_not_mutate_input() {
        let graph = graph();
        let edges = edge_set(&graph);

        let original_graph = graph.clone();
        let original_edges = edges.clone();

        let mut transformation =
            GraphTransformation::new(&graph, &edges)
                .expect("valid transformation");

        transformation
            .add_node(node("d"))
            .add_node(node("d"));

        assert!(transformation.apply(&graph, &edges).is_err());

        assert_eq!(graph, original_graph);
        assert_eq!(edges, original_edges);
    }

    #[test]
    fn parallel_edges_are_preserved() {
        let graph = graph();

        let edges = CompilationGraphEdgeSet::new(
            &graph,
            vec![
                edge("edge.one", "dependency.data", "a", "b"),
                edge("edge.two", "dependency.control", "a", "b"),
            ],
            BTreeMap::new(),
        )
        .expect("parallel edges are valid");

        let mut transformation =
            GraphTransformation::new(&graph, &edges)
                .expect("valid transformation");

        transformation.add_edge(
            edge(
                "edge.three",
                "dependency.feedback",
                "b",
                "a",
            ),
        );

        let result = transformation
            .apply(&graph, &edges)
            .expect("cycle and parallel edges are valid");

        assert_eq!(result.edge_count(), 3);
    }

    #[test]
    fn transformation_preserves_graph_identity() {
        let graph = graph();
        let edges = edge_set(&graph);

        let transformation =
            GraphTransformation::new(&graph, &edges)
                .expect("valid transformation");

        let result = transformation
            .apply(&graph, &edges)
            .expect("transformation succeeds");

        assert_eq!(result.graph().id(), graph.id());
    }

    #[test]
    fn transformed_graph_remains_valid() {
        let graph = graph();
        let edges = edge_set(&graph);

        let mut transformation =
            GraphTransformation::new(&graph, &edges)
                .expect("valid transformation");

        transformation
            .add_node(node("d"))
            .add_edge(
                edge(
                    "edge.cd",
                    "dependency.data",
                    "c",
                    "d",
                ),
            );

        let result = transformation
            .apply(&graph, &edges)
            .expect("transformation succeeds");

        assert!(result.validate().is_ok());
    }
}
