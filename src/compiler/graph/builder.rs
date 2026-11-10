
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Universal Meta-Compiler — Compilation Graph Builder.
//!
//! Path: src/compiler/graph/builder.rs
//! Edition: Rust 2021
//! Minimum supported Rust version: 1.97
//! Safety: Unsafe Rust is forbidden.
//!
//! # Purpose
//!
//! Incrementally constructs the canonical compiler compilation graph
//! and its independently owned directed edge set.
//!
//! The builder coordinates references to entities owned by other
//! authoritative repository subsystems.
//!
//! It does not create another program IR or semantic model.
//!
//! # Canonical ownership
//!
//! - graph/model.rs owns CompilationGraph and CompilationGraphNode.
//! - graph/edges.rs owns CompilationGraphEdge and CompilationGraphEdgeSet.
//! - This module owns incremental construction and final composition.
//!
//! # Non-responsibilities
//!
//! This module does not own:
//!
//! - Source grammar, parsing or AST construction.
//! - Source-level semantic validation.
//! - ZUIR or domain-specific IR.
//! - Dependency semantic interpretation.
//! - Domain registration.
//! - Hardware discovery or allocation.
//! - Backend selection or code generation.
//! - Physical scheduling or routing.
//! - Runtime execution.
//!
//! # Scalability
//!
//! No fixed architectural maximum is imposed on:
//!
//! - Nodes.
//! - Edges.
//! - Computational domains.
//! - Metadata entries.
//! - Identifier lengths.
//! - Provider types.
//! - Graph relationships.
//!
//! Optional limits are explicitly supplied through GraphBuilderConfig.
//!
//! Without configured limits, the builder grows according to available
//! implementation resources.
//!
//! This does not imply literal infinite memory or computation.
//!
//! # Determinism
//!
//! - Node insertion order is preserved.
//! - Edge insertion order is preserved.
//! - Metadata uses BTreeMap.
//! - Identity indexes use BTreeSet.
//! - No randomized iteration order is exposed.
//!
//! # Transactionality
//!
//! Failed insertion operations do not modify the staged graph.
//!
//! Build consumes the builder and either returns a completely validated
//! graph/edge-set pair or returns an error.
//!
//! # Integration
//!
//! - super::model
//! - super::edges
//! - crate::compiler::analysis
//! - crate::compiler::planning
//! - crate::compiler::pipeline
//! - crate::compiler::ir
//! - crate::compiler::artifacts
//!
//! Semantic and domain verification remain downstream responsibilities.

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use super::edges::{
    CompilationGraphEdge,
    CompilationGraphEdgeId,
    CompilationGraphEdgeSet,
    CompilationGraphEdgeKind,
    GraphEdgesError,
};

use super::model::{
    CanonicalEntityRef,
    CompilationGraph,
    CompilationGraphId,
    CompilationGraphNode,
    CompilationGraphNodeId,
    CompilationGraphNodeKind,
    GraphModelError,
};

// ============================================================================
// API VERSION
// ============================================================================

/// Public API version of the compilation graph builder.
pub const COMPILATION_GRAPH_BUILDER_API_VERSION: u16 = 1;

// ============================================================================
// RESULT
// ============================================================================

/// Result type for graph builder operations.
pub type GraphBuilderResult<T> = Result<T, GraphBuilderError>;

// ============================================================================
// ERRORS
// ============================================================================

/// Errors produced by compilation graph construction.
#[derive(Debug)]
#[non_exhaustive]
pub enum GraphBuilderError {

    /// A node identity has already been registered.
    DuplicateNode {
        node_id: String,
    },

    /// An edge identity has already been registered.
    DuplicateEdge {
        edge_id: String,
    },

    /// An edge references a node not registered in this builder.
    UnknownEndpoint {
        node_id: String,
    },

    /// An identity belongs to another graph.
    GraphIdentityMismatch {
        expected: String,
        actual: String,
    },

    /// A configured node budget was exceeded.
    NodeBudgetExceeded {
        limit: u128,
        requested: u128,
    },

    /// A configured edge budget was exceeded.
    EdgeBudgetExceeded {
        limit: u128,
        requested: u128,
    },

    /// A configured metadata budget was exceeded.
    MetadataBudgetExceeded {
        limit: u128,
        requested: u128,
    },

    /// A configured work budget was exceeded.
    WorkBudgetExceeded {
        limit: u128,
        requested: u128,
    },

    /// Arithmetic accounting overflowed.
    AccountingOverflow,

    /// A collection could not reserve the required capacity.
    AllocationFailure {
        collection: &'static str,
        requested: usize,
    },

    /// The underlying graph model rejected construction.
    GraphModel {
        source: GraphModelError,
    },

    /// The underlying graph edge subsystem rejected construction.
    GraphEdges {
        source: GraphEdgesError,
    },
}

impl fmt::Display for GraphBuilderError {

    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {

        match self {

            Self::DuplicateNode { node_id } => {
                write!(
                    formatter,
                    "duplicate compilation graph node: {node_id}"
                )
            }

            Self::DuplicateEdge { edge_id } => {
                write!(
                    formatter,
                    "duplicate compilation graph edge: {edge_id}"
                )
            }

            Self::UnknownEndpoint { node_id } => {
                write!(
                    formatter,
                    "compilation graph edge references unknown node: {node_id}"
                )
            }

            Self::GraphIdentityMismatch {
                expected,
                actual,
            } => {
                write!(
                    formatter,
                    "graph identity mismatch: expected {expected}, got {actual}"
                )
            }

            Self::NodeBudgetExceeded {
                limit,
                requested,
            } => {
                write!(
                    formatter,
                    "graph node budget exceeded: limit {limit}, requested {requested}"
                )
            }

            Self::EdgeBudgetExceeded {
                limit,
                requested,
            } => {
                write!(
                    formatter,
                    "graph edge budget exceeded: limit {limit}, requested {requested}"
                )
            }

            Self::MetadataBudgetExceeded {
                limit,
                requested,
            } => {
                write!(
                    formatter,
                    "graph metadata budget exceeded: limit {limit}, requested {requested}"
                )
            }

            Self::WorkBudgetExceeded {
                limit,
                requested,
            } => {
                write!(
                    formatter,
                    "graph construction work budget exceeded: limit {limit}, requested {requested}"
                )
            }

            Self::AccountingOverflow => {
                formatter.write_str(
                    "compilation graph builder accounting overflow"
                )
            }

            Self::AllocationFailure {
                collection,
                requested,
            } => {
                write!(
                    formatter,
                    "unable to reserve {requested} additional entries for {collection}"
                )
            }

            Self::GraphModel { source } => {
                write!(
                    formatter,
                    "compilation graph model error: {source}"
                )
            }

            Self::GraphEdges { source } => {
                write!(
                    formatter,
                    "compilation graph edge error: {source}"
                )
            }
        }
    }
}

impl Error for GraphBuilderError {

    fn source(
        &self,
    ) -> Option<&(dyn Error + 'static)> {

        match self {

            Self::GraphModel { source } => Some(source),

            Self::GraphEdges { source } => Some(source),

            _ => None,
        }
    }
}

impl From<GraphModelError> for GraphBuilderError {

    fn from(source: GraphModelError) -> Self {

        Self::GraphModel { source }
    }
}

impl From<GraphEdgesError> for GraphBuilderError {

    fn from(source: GraphEdgesError) -> Self {

        Self::GraphEdges { source }
    }
}

// ============================================================================
// CONFIGURATION
// ============================================================================

/// Optional operational limits for graph construction.
///
/// These are operational budgets, not semantic or architectural
/// restrictions.
///
/// None means that this particular budget is not configured.
///
/// Actual memory and address-space limitations still apply.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Default,
)]
pub struct GraphBuilderConfig {

    /// Optional maximum number of graph nodes.
    pub max_nodes: Option<u128>,

    /// Optional maximum number of graph edges.
    pub max_edges: Option<u128>,

    /// Optional maximum estimated metadata bytes.
    pub max_metadata_bytes: Option<u128>,

    /// Optional maximum builder work units.
    pub max_work_units: Option<u128>,
}

impl GraphBuilderConfig {

    /// Validate that configured limits are representable.
    ///
    /// Zero is permitted and means that no corresponding entries
    /// may be inserted.
    pub fn validate(&self) -> GraphBuilderResult<()> {

        Ok(())
    }

    /// Check the requested node count.
    fn check_nodes(
        &self,
        requested: u128,
    ) -> GraphBuilderResult<()> {

        check_limit(
            self.max_nodes,
            requested,
            GraphBuilderError::NodeBudgetExceeded {
                limit: self.max_nodes.unwrap_or_default(),
                requested,
            },
        )
    }

    /// Check the requested edge count.
    fn check_edges(
        &self,
        requested: u128,
    ) -> GraphBuilderResult<()> {

        check_limit(
            self.max_edges,
            requested,
            GraphBuilderError::EdgeBudgetExceeded {
                limit: self.max_edges.unwrap_or_default(),
                requested,
            },
        )
    }

    /// Check estimated metadata consumption.
    fn check_metadata(
        &self,
        requested: u128,
    ) -> GraphBuilderResult<()> {

        check_limit(
            self.max_metadata_bytes,
            requested,
            GraphBuilderError::MetadataBudgetExceeded {
                limit: self.max_metadata_bytes.unwrap_or_default(),
                requested,
            },
        )
    }

    /// Check builder work consumption.
    fn check_work(
        &self,
        requested: u128,
    ) -> GraphBuilderResult<()> {

        check_limit(
            self.max_work_units,
            requested,
            GraphBuilderError::WorkBudgetExceeded {
                limit: self.max_work_units.unwrap_or_default(),
                requested,
            },
        )
    }
}

fn check_limit(
    limit: Option<u128>,
    requested: u128,
    error: GraphBuilderError,
) -> GraphBuilderResult<()> {

    if let Some(limit) = limit {

        if requested > limit {
            return Err(error);
        }
    }

    Ok(())
}

// ============================================================================
// BUILD RESULT
// ============================================================================

/// Complete result of compilation graph construction.
///
/// The graph and its edge set remain separate canonical structures.
///
/// Neither structure duplicates the other's ownership.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuiltCompilationGraph {

    graph: CompilationGraph,

    edges: CompilationGraphEdgeSet,
}

impl BuiltCompilationGraph {

    /// Return the canonical compilation graph.
    #[must_use]
    pub fn graph(&self) -> &CompilationGraph {

        &self.graph
    }

    /// Return the canonical graph edge set.
    #[must_use]
    pub fn edges(&self) -> &CompilationGraphEdgeSet {

        &self.edges
    }

    /// Consume the result and return both canonical structures.
    #[must_use]
    pub fn into_parts(
        self,
    ) -> (CompilationGraph, CompilationGraphEdgeSet) {

        (self.graph, self.edges)
    }

    /// Return the number of graph nodes.
    #[must_use]
    pub fn node_count(&self) -> usize {

        self.graph.node_count()
    }

    /// Return the number of graph edges.
    #[must_use]
    pub fn edge_count(&self) -> usize {

        self.edges.edge_count()
    }
}

// ============================================================================
// BUILDER
// ============================================================================

/// Incremental compilation graph builder.
///
/// The builder owns temporary construction state.
///
/// It never mutates an already finalized CompilationGraph.
///
/// A builder instance is scoped to one graph identity.
///
/// It is intentionally not internally synchronized.
/// Separate compilation graphs can be built concurrently.
///
/// Concurrent mutation of one builder requires external synchronization.
#[derive(Debug)]
pub struct CompilationGraphBuilder {

    graph_id: CompilationGraphId,

    config: GraphBuilderConfig,

    nodes: Vec<CompilationGraphNode>,

    edges: Vec<CompilationGraphEdge>,

    node_ids: BTreeSet<CompilationGraphNodeId>,

    edge_ids: BTreeSet<CompilationGraphEdgeId>,

    graph_metadata: BTreeMap<String, String>,

    work_units: u128,

    metadata_bytes: u128,
}

impl CompilationGraphBuilder {

    // ------------------------------------------------------------------------
    // CONSTRUCTION
    // ------------------------------------------------------------------------

    /// Construct a builder with no configured operational ceilings.
    ///
    /// Construction is subject to available system resources.
    pub fn new(
        graph_id: CompilationGraphId,
    ) -> GraphBuilderResult<Self> {

        Self::with_config(
            graph_id,
            GraphBuilderConfig::default(),
        )
    }

    /// Construct a builder with explicit operational budgets.
    pub fn with_config(
        graph_id: CompilationGraphId,
        config: GraphBuilderConfig,
    ) -> GraphBuilderResult<Self> {

        config.validate()?;

        let empty_graph = CompilationGraph::new(
            graph_id.clone(),
            Vec::new(),
            BTreeMap::new(),
        )?;

        let empty_edges = CompilationGraphEdgeSet::new(
            &empty_graph,
            Vec::new(),
            BTreeMap::new(),
        )?;

        let metadata_bytes = checked_add(
            empty_graph.estimated_metadata_bytes()?,
            empty_edges.estimated_metadata_bytes()?,
        )?;

        config.check_metadata(metadata_bytes)?;

        Ok(Self {
            graph_id,
            config,
            nodes: Vec::new(),
            edges: Vec::new(),
            node_ids: BTreeSet::new(),
            edge_ids: BTreeSet::new(),
            graph_metadata: BTreeMap::new(),
            work_units: 0,
            metadata_bytes,
        })
    }

    /// Return the identity of the graph being constructed.
    #[must_use]
    pub fn graph_id(&self) -> &CompilationGraphId {

        &self.graph_id
    }

    /// Return the construction configuration.
    #[must_use]
    pub fn config(&self) -> &GraphBuilderConfig {

        &self.config
    }

    /// Return the current work accounting.
    #[must_use]
    pub const fn work_units(&self) -> u128 {

        self.work_units
    }

    /// Return the current estimated metadata accounting.
    #[must_use]
    pub const fn metadata_bytes(&self) -> u128 {

        self.metadata_bytes
    }

    // ------------------------------------------------------------------------
    // COUNTS
    // ------------------------------------------------------------------------

    /// Return the number of staged nodes.
    #[must_use]
    pub fn node_count(&self) -> usize {

        self.nodes.len()
    }

    /// Return the number of staged edges.
    #[must_use]
    pub fn edge_count(&self) -> usize {

        self.edges.len()
    }

    /// Return whether no nodes have been staged.
    #[must_use]
    pub fn is_empty(&self) -> bool {

        self.nodes.is_empty()
    }

    /// Check whether a node identity is registered.
    #[must_use]
    pub fn contains_node(
        &self,
        id: &CompilationGraphNodeId,
    ) -> bool {

        self.node_ids.contains(id)
    }

    /// Check whether an edge identity is registered.
    #[must_use]
    pub fn contains_edge(
        &self,
        id: &CompilationGraphEdgeId,
    ) -> bool {

        self.edge_ids.contains(id)
    }

    // ------------------------------------------------------------------------
    // GRAPH METADATA
    // ------------------------------------------------------------------------

    /// Insert or replace descriptive graph metadata.
    ///
    /// Metadata cannot replace canonical semantic information.
    ///
    /// The operation is transactional: invalid input or an exceeded
    /// budget leaves the existing metadata unchanged.
    pub fn set_metadata(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> GraphBuilderResult<()> {

        let key = key.into();
        let value = value.into();

        if key.trim().is_empty()
            || key.chars().any(char::is_control)
        {
            return Err(
                GraphBuilderError::GraphModel {
                    source: GraphModelError::InvalidMetadataKey,
                },
            );
        }

        if value.chars().any(char::is_control) {
            return Err(
                GraphBuilderError::GraphModel {
                    source: GraphModelError::InvalidMetadataValue,
                },
            );
        }

        let mut candidate = self.graph_metadata.clone();

        candidate.insert(key, value);

        let new_bytes = metadata_map_bytes(&candidate)?;

        let base_bytes = self.metadata_bytes
            .checked_sub(
                metadata_map_bytes(&self.graph_metadata)?,
            )
            .ok_or(GraphBuilderError::AccountingOverflow)?;

        let requested = checked_add(
            base_bytes,
            new_bytes,
        )?;

        self.config.check_metadata(requested)?;

        self.graph_metadata = candidate;
        self.metadata_bytes = requested;

        Ok(())
    }

    // ------------------------------------------------------------------------
    // NODE CONSTRUCTION
    // ------------------------------------------------------------------------

    /// Construct and insert a graph node from its canonical components.
    ///
    /// The entity remains owned by its authoritative subsystem.
    ///
    /// This convenience method does not generate or reinterpret IDs.
    pub fn add_node(
        &mut self,
        id: CompilationGraphNodeId,
        kind: CompilationGraphNodeKind,
        entity: CanonicalEntityRef,
        label: Option<String>,
        metadata: BTreeMap<String, String>,
    ) -> GraphBuilderResult<CompilationGraphNodeId> {

        let node = CompilationGraphNode::new(
            id,
            kind,
            entity,
            label,
            metadata,
        )?;

        self.insert_node(node)
    }

    /// Insert an existing validated canonical graph node.
    ///
    /// The supplied identity is preserved.
    ///
    /// Duplicate identities are rejected.
    ///
    /// Failed insertion leaves the builder unchanged.
    pub fn insert_node(
        &mut self,
        node: CompilationGraphNode,
    ) -> GraphBuilderResult<CompilationGraphNodeId> {

        node.validate()?;

        let id = node.id().clone();

        if self.node_ids.contains(&id) {

            return Err(
                GraphBuilderError::DuplicateNode {
                    node_id: id.to_string(),
                },
            );
        }

        let requested_nodes = checked_count(
            self.nodes.len(),
        )?;

        let requested_nodes = checked_add(
            requested_nodes,
            1,
        )?;

        self.config.check_nodes(requested_nodes)?;

        let new_metadata = node.estimated_metadata_bytes()?;

        let requested_metadata = checked_add(
            self.metadata_bytes,
            new_metadata,
        )?;

        self.config.check_metadata(requested_metadata)?;

        let requested_work = checked_add(
            self.work_units,
            1,
        )?;

        self.config.check_work(requested_work)?;

        self.nodes
            .try_reserve(1)
            .map_err(|_| GraphBuilderError::AllocationFailure {
                collection: "compilation graph nodes",
                requested: 1,
            })?;

        self.node_ids.insert(id.clone());

        self.nodes.push(node);

        self.metadata_bytes = requested_metadata;
        self.work_units = requested_work;

        Ok(id)
    }

    // ------------------------------------------------------------------------
    // EDGE CONSTRUCTION
    // ------------------------------------------------------------------------

    /// Construct and insert a directed graph edge.
    ///
    /// Both endpoints must already exist in the builder.
    ///
    /// The relationship kind is open-world and is interpreted by
    /// its authoritative consumer.
    pub fn add_edge(
        &mut self,
        id: CompilationGraphEdgeId,
        kind: CompilationGraphEdgeKind,
        source: CompilationGraphNodeId,
        target: CompilationGraphNodeId,
        metadata: BTreeMap<String, String>,
    ) -> GraphBuilderResult<CompilationGraphEdgeId> {

        let edge = CompilationGraphEdge::new(
            id,
            kind,
            source,
            target,
            metadata,
        )?;

        self.insert_edge(edge)
    }

    /// Insert an existing canonical graph edge.
    ///
    /// This performs early endpoint and uniqueness validation.
    ///
    /// It does not reject cycles, parallel relationships or
    /// self-referential edges.
    pub fn insert_edge(
        &mut self,
        edge: CompilationGraphEdge,
    ) -> GraphBuilderResult<CompilationGraphEdgeId> {

        edge.validate()?;

        let id = edge.id().clone();

        if self.edge_ids.contains(&id) {

            return Err(
                GraphBuilderError::DuplicateEdge {
                    edge_id: id.to_string(),
                },
            );
        }

        self.check_endpoint(edge.source())?;
        self.check_endpoint(edge.target())?;

        let requested_edges = checked_add(
            checked_count(self.edges.len())?,
            1,
        )?;

        self.config.check_edges(requested_edges)?;

        let new_metadata = edge.estimated_metadata_bytes()?;

        let requested_metadata = checked_add(
            self.metadata_bytes,
            new_metadata,
        )?;

        self.config.check_metadata(requested_metadata)?;

        let requested_work = checked_add(
            self.work_units,
            1,
        )?;

        self.config.check_work(requested_work)?;

        self.edges
            .try_reserve(1)
            .map_err(|_| GraphBuilderError::AllocationFailure {
                collection: "compilation graph edges",
                requested: 1,
            })?;

        self.edge_ids.insert(id.clone());

        self.edges.push(edge);

        self.metadata_bytes = requested_metadata;
        self.work_units = requested_work;

        Ok(id)
    }

    // ------------------------------------------------------------------------
    // VALIDATION
    // ------------------------------------------------------------------------

    fn check_endpoint(
        &self,
        id: &CompilationGraphNodeId,
    ) -> GraphBuilderResult<()> {

        if !self.node_ids.contains(id) {

            return Err(
                GraphBuilderError::UnknownEndpoint {
                    node_id: id.to_string(),
                },
            );
        }

        Ok(())
    }

    // ------------------------------------------------------------------------
    // FINALIZATION
    // ------------------------------------------------------------------------

    /// Finalize the complete graph and edge set.
    ///
    /// The resulting graph and edge set are validated independently
    /// by their canonical owning modules.
    ///
    /// This method does not establish:
    ///
    /// - Semantic validity.
    /// - Domain-operation validity.
    /// - Cycle legality.
    /// - Target feasibility.
    /// - Hardware availability.
    /// - Execution correctness.
    ///
    /// Those guarantees belong to downstream authorities.
    pub fn build(
        self,
    ) -> GraphBuilderResult<BuiltCompilationGraph> {

        let graph = CompilationGraph::new(
            self.graph_id,
            self.nodes,
            self.graph_metadata,
        )
        .map_err(GraphBuilderError::from)?;

        graph.validate()
            .map_err(GraphBuilderError::from)?;

        let edges = CompilationGraphEdgeSet::new(
            &graph,
            self.edges,
            BTreeMap::new(),
        )
        .map_err(GraphBuilderError::from)?;

        edges.validate_against(&graph)
            .map_err(GraphBuilderError::from)?;

        Ok(BuiltCompilationGraph {
            graph,
            edges,
        })
    }
}

// ============================================================================
// ACCOUNTING
// ============================================================================

fn checked_add(
    left: u128,
    right: u128,
) -> GraphBuilderResult<u128> {

    left.checked_add(right)
        .ok_or(GraphBuilderError::AccountingOverflow)
}

fn checked_count(
    count: usize,
) -> GraphBuilderResult<u128> {

    u128::try_from(count)
        .map_err(|_| GraphBuilderError::AccountingOverflow)
}

/// Estimate metadata map storage using UTF-8 payload sizes.
///
/// This is accounting information, not an exact heap measurement.
fn metadata_map_bytes(
    metadata: &BTreeMap<String, String>,
) -> GraphBuilderResult<u128> {

    let mut total = 0_u128;

    for (key, value) in metadata {

        total = checked_add(
            total,
            key.len() as u128,
        )?;

        total = checked_add(
            total,
            value.len() as u128,
        )?;
    }

    Ok(total)
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {

    use super::*;

    fn graph_id() -> CompilationGraphId {

        CompilationGraphId::new("compiler.test.graph")
            .expect("valid graph ID")
    }

    fn node_id(
        value: &str,
    ) -> CompilationGraphNodeId {

        CompilationGraphNodeId::new(value)
            .expect("valid node ID")
    }

    fn edge_id(
        value: &str,
    ) -> CompilationGraphEdgeId {

        CompilationGraphEdgeId::new(value)
            .expect("valid edge ID")
    }

    fn node(
        id: &str,
    ) -> CompilationGraphNode {

        CompilationGraphNode::new(
            node_id(id),
            CompilationGraphNodeKind::new(
                "compiler.test.node",
            )
            .expect("valid kind"),
            CanonicalEntityRef::new(
                "compiler.test",
                "entity",
                id,
            )
            .expect("valid entity"),
            None,
            BTreeMap::new(),
        )
        .expect("valid node")
    }

    fn builder() -> CompilationGraphBuilder {

        CompilationGraphBuilder::new(graph_id())
            .expect("valid builder")
    }

    fn edge(
        id: &str,
        source: &str,
        target: &str,
    ) -> CompilationGraphEdge {

        CompilationGraphEdge::new(
            edge_id(id),
            CompilationGraphEdgeKind::new(
                "dependency.data",
            )
            .expect("valid edge kind"),
            node_id(source),
            node_id(target),
            BTreeMap::new(),
        )
        .expect("valid edge")
    }

    #[test]
    fn constructs_empty_graph() {

        let built = builder()
            .build()
            .expect("empty graph is valid");

        assert_eq!(built.node_count(), 0);
        assert_eq!(built.edge_count(), 0);
    }

    #[test]
    fn inserts_nodes_and_preserves_order() {

        let mut builder = builder();

        let first = builder.insert_node(node("first"))
            .expect("first node");

        let second = builder.insert_node(node("second"))
            .expect("second node");

        let built = builder.build()
            .expect("valid graph");

        assert_eq!(
            built.graph().nodes()[0].id(),
            &first
        );

        assert_eq!(
            built.graph().nodes()[1].id(),
            &second
        );
    }

    #[test]
    fn rejects_duplicate_nodes_transactionally() {

        let mut builder = builder();

        builder.insert_node(node("first"))
            .expect("first insertion");

        let error = builder.insert_node(node("first"))
            .expect_err("duplicate must fail");

        assert!(matches!(
            error,
            GraphBuilderError::DuplicateNode { .. }
        ));

        assert_eq!(builder.node_count(), 1);
    }

    #[test]
    fn inserts_edges_between_existing_nodes() {

        let mut builder = builder();

        builder.insert_node(node("a")).unwrap();
        builder.insert_node(node("b")).unwrap();

        builder.insert_edge(edge("edge.ab", "a", "b"))
            .expect("valid edge");

        let built = builder.build()
            .expect("valid graph");

        assert_eq!(built.edge_count(), 1);
    }

    #[test]
    fn rejects_missing_edge_endpoints() {

        let mut builder = builder();

        builder.insert_node(node("a")).unwrap();

        let result = builder.insert_edge(
            edge("edge.invalid", "a", "missing"),
        );

        assert!(matches!(
            result,
            Err(GraphBuilderError::UnknownEndpoint { .. })
        ));

        assert_eq!(builder.edge_count(), 0);
    }

    #[test]
    fn supports_parallel_edges() {

        let mut builder = builder();

        builder.insert_node(node("a")).unwrap();
        builder.insert_node(node("b")).unwrap();

        builder.insert_edge(edge("edge.one", "a", "b")).unwrap();
        builder.insert_edge(edge("edge.two", "a", "b")).unwrap();

        assert_eq!(builder.edge_count(), 2);
    }

    #[test]
    fn supports_feedback_cycles() {

        let mut builder = builder();

        builder.insert_node(node("a")).unwrap();
        builder.insert_node(node("b")).unwrap();

        builder.insert_edge(edge("edge.ab", "a", "b")).unwrap();
        builder.insert_edge(edge("edge.ba", "b", "a")).unwrap();

        let built = builder.build()
            .expect("structural cycles are valid");

        assert_eq!(built.edge_count(), 2);
    }

    #[test]
    fn supports_self_referential_edges() {

        let mut builder = builder();

        builder.insert_node(node("a")).unwrap();

        builder.insert_edge(edge("edge.self", "a", "a")).unwrap();

        assert_eq!(builder.edge_count(), 1);
    }

    #[test]
    fn rejects_duplicate_edges_transactionally() {

        let mut builder = builder();

        builder.insert_node(node("a")).unwrap();
        builder.insert_node(node("b")).unwrap();

        builder.insert_edge(edge("duplicate", "a", "b")).unwrap();

        let result = builder.insert_edge(
            edge("duplicate", "b", "a"),
        );

        assert!(matches!(
            result,
            Err(GraphBuilderError::DuplicateEdge { .. })
        ));

        assert_eq!(builder.edge_count(), 1);
    }

    #[test]
    fn enforces_explicit_node_budget() {

        let config = GraphBuilderConfig {
            max_nodes: Some(1),
            ..Default::default()
        };

        let mut builder = CompilationGraphBuilder::with_config(
            graph_id(),
            config,
        )
        .unwrap();

        builder.insert_node(node("a")).unwrap();

        let result = builder.insert_node(node("b"));

        assert!(matches!(
            result,
            Err(GraphBuilderError::NodeBudgetExceeded { .. })
        ));

        assert_eq!(builder.node_count(), 1);
    }

    #[test]
    fn enforces_explicit_edge_budget() {

        let config = GraphBuilderConfig {
            max_edges: Some(0),
            ..Default::default()
        };

        let mut builder = CompilationGraphBuilder::with_config(
            graph_id(),
            config,
        )
        .unwrap();

        builder.insert_node(node("a")).unwrap();

        let result = builder.insert_edge(
            edge("edge.self", "a", "a"),
        );

        assert!(matches!(
            result,
            Err(GraphBuilderError::EdgeBudgetExceeded { .. })
        ));

        assert_eq!(builder.edge_count(), 0);
    }

    #[test]
    fn metadata_replacement_is_transactional() {

        let mut builder = builder();

        builder.set_metadata("name", "original").unwrap();

        let before = builder.metadata_bytes();

        builder.set_metadata("name", "replacement")
            .unwrap();

        assert_ne!(builder.metadata_bytes(), before);
    }

    #[test]
    fn rejects_invalid_metadata() {

        let mut builder = builder();

        let result = builder.set_metadata(
            "invalid\nkey",
            "value",
        );

        assert!(result.is_err());
        assert_eq!(builder.node_count(), 0);
    }

    #[test]
    fn supports_open_world_node_domains() {

        let mut builder = builder();

        let custom = CompilationGraphNode::new(
            node_id("future"),
            CompilationGraphNodeKind::new(
                "third_party.future_computing_domain",
            )
            .unwrap(),
            CanonicalEntityRef::new(
                "third_party.future_ir",
                "computation",
                "entity-1",
            )
            .unwrap(),
            None,
            BTreeMap::new(),
        )
        .unwrap();

        builder.insert_node(custom).unwrap();

        assert_eq!(builder.node_count(), 1);
    }

    #[test]
    fn supports_unrestricted_default_configuration() {

        let config = GraphBuilderConfig::default();

        assert_eq!(config.max_nodes, None);
        assert_eq!(config.max_edges, None);
        assert_eq!(config.max_metadata_bytes, None);
        assert_eq!(config.max_work_units, None);
    }

    #[test]
    fn work_accounting_is_deterministic() {

        let mut builder = builder();

        assert_eq!(builder.work_units(), 0);

        builder.insert_node(node("a")).unwrap();

        assert_eq!(builder.work_units(), 1);

        builder.insert_node(node("b")).unwrap();

        assert_eq!(builder.work_units(), 2);

        builder.insert_edge(edge("ab", "a", "b")).unwrap();

        assert_eq!(builder.work_units(), 3);
    }

    #[test]
    fn graph_and_edges_are_returned_separately() {

        let mut builder = builder();

        builder.insert_node(node("a")).unwrap();

        let built = builder.build().unwrap();

        let (graph, edges) = built.into_parts();

        assert_eq!(graph.node_count(), 1);
        assert_eq!(edges.edge_count(), 0);
    }
}
