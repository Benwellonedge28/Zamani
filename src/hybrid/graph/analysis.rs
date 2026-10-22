
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Hybrid Computing — Read-Only Graph Analysis
//!
//! Path: `src/hybrid/graph/analysis.rs`
//! Edition: Rust 2021
//! Minimum supported Rust: 1.97
//! Safety: Unsafe Rust is forbidden.
//!
//! ## Purpose
//!
//! Provide deterministic, read-only analyses over the canonical hybrid graph
//! and its separately stored edge set.
//!
//! This module derives structural information useful to graph verification,
//! compiler planning, diagnostics, portability assessment, and runtime
//! integration. It does not change the graph or the meaning of its nodes
//! and edges.
//!
//! ## Canonical ownership
//!
//! - `graph/model.rs` owns `HybridGraph`, `HybridNode`, and canonical IR refs.
//! - `graph/edges.rs` owns `HybridEdgeSet`, `HybridEdge`, and edge semantics.
//! - `domain/composition.rs` owns domain-instance composition.
//! - `domain/identity.rs` owns semantic domain identifiers.
//! - `config.rs` owns explicit operational budgets.
//! - `errors.rs` owns structured hybrid errors.
//! - `ids.rs` owns graph-scoped hybrid identities.
//! - `quantum::ir` remains the canonical quantum semantic representation.
//!
//! This module does not create a second IR or a second authoritative graph.
//!
//! ## Scope
//!
//! This module provides:
//!
//! - graph and edge-set consistency checks;
//! - reusable node, incoming-edge, and outgoing-edge indexes;
//! - node and edge counts;
//! - semantic-domain and domain-crossing statistics;
//! - isolated-node, source-node, and sink-node identification;
//! - deterministic reachability queries;
//! - deterministic topological ordering when one exists;
//! - explicit reporting when a complete topological order does not exist;
//! - connected-component analysis treating directed edges as undirected;
//! - configurable operational-budget enforcement.
//!
//! ## Important semantic rules
//!
//! A directed cycle is not automatically invalid. Feedback, iteration, and
//! loop-carried relationships can be valid. A failed topological-order query
//! therefore reports the absence of a topological order; it does not declare
//! the source program invalid.
//!
//! Structural reachability does not prove data availability, type
//! compatibility, causal correctness, domain-contract satisfaction, or
//! target feasibility. Those properties remain the responsibility of the
//! corresponding verification and integration layers.
//!
//! Domain crossings are calculated from the semantic domain identities of
//! the endpoint nodes. A change of domain instance alone does not necessarily
//! constitute a change of semantic domain.
//!
//! ## Scalability
//!
//! This module imposes no fixed node, edge, domain, or traversal ceiling.
//! It uses growable collections and checked work accounting. An explicitly
//! configured validation budget can reject excessive analysis work.
//! Budget exhaustion is an operational failure, not a semantic restriction
//! on the represented Zamani program.
//!
//! Actual execution remains bounded by available memory, address space,
//! processing time, and the finite representation of machine integers.
//!
//! ## Determinism
//!
//! Indexes and result sets use ordered collections. Traversal roots and
//! adjacency lists follow typed identifier order where canonical ordering is
//! required. Graph declaration order is not modified.
//!
//! IDs are identities, not content hashes. This module does not claim that
//! independently allocated graphs receive reproducible IDs.
//!
//! ## Integration
//!
//! Consumers should construct one `HybridGraphAnalysis` for a graph and edge
//! set, then reuse its indexes for multiple queries. This avoids repeated
//! linear scans through the canonical model.
//!
//! The graph builder may invoke this module after graph and edge construction.
//! The verifier remains responsible for semantic and whole-graph validity.
//! The compiler pipeline and runtime may consume analysis results but must
//! not infer physical placement, scheduling, or device capabilities here.

use std::collections::{BTreeMap, BTreeSet, VecDeque};

use crate::hybrid::config::{BudgetKind, HybridConfig};
use crate::hybrid::domain::identity::DomainId;
use crate::hybrid::errors::{HybridError, HybridErrorCode};
use crate::hybrid::ids::{HybridEdgeId, HybridNodeId};

use super::edges::{
    EdgeKind,
    EdgeValidationReport,
    HybridEdge,
    HybridEdgeSet,
};
use super::model::{HybridGraph, HybridNode};

/// Summary statistics derived from a validated hybrid graph and edge set.
///
/// Counts use `u128` so that accounting does not need to narrow counts to a
/// platform-sized integer before reporting them. The graph's actual
/// collections are still constrained by the host's representable memory and
/// address space.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct GraphAnalysisSummary {
    /// Number of nodes in the graph.
    pub node_count: u128,

    /// Number of edges in the edge set.
    pub edge_count: u128,

    /// Number of distinct semantic domains referenced by graph nodes.
    ///
    /// Domain instances without graph nodes are not counted here.
    pub participating_domain_count: u128,

    /// Number of edges whose endpoint nodes belong to different semantic
    /// domains. This is derived from the domain identities, not from edge
    /// labels or metadata.
    pub cross_domain_edge_count: u128,

    /// Number of edges explicitly declared as domain-boundary edges.
    pub declared_domain_boundary_edge_count: u128,

    /// Number of nodes without incoming edges.
    pub source_node_count: u128,

    /// Number of nodes without outgoing edges.
    pub sink_node_count: u128,

    /// Number of nodes without incoming or outgoing edges.
    pub isolated_node_count: u128,

    /// Maximum incoming-edge count of any node.
    pub maximum_in_degree: u128,

    /// Maximum outgoing-edge count of any node.
    pub maximum_out_degree: u128,

    /// Number of weakly connected components.
    ///
    /// Edge direction is ignored for this statistic. An empty graph has
    /// zero components.
    pub weakly_connected_component_count: u128,
}

/// A deterministic topological-order result.
///
/// A cyclic graph is not necessarily semantically invalid. The `Cyclic`
/// variant identifies nodes left after Kahn's algorithm and does not decide
/// whether the corresponding cycles are legal for the source program.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TopologicalOrder {
    /// Every node appears exactly once in prerequisite-before-dependent order.
    Order(Vec<HybridNodeId>),

    /// No complete topological ordering exists for the supplied edge set.
    ///
    /// `remaining_nodes` contains nodes that Kahn's algorithm could not
    /// remove. It may include nodes downstream of a cycle; it is not
    /// guaranteed to contain only the exact members of each cycle.
    Cyclic {
        remaining_nodes: Vec<HybridNodeId>,
    },
}

impl TopologicalOrder {
    /// Returns the complete ordering if one exists.
    #[must_use]
    pub fn order(&self) -> Option<&[HybridNodeId]> {
        match self {
            Self::Order(order) => Some(order),
            Self::Cyclic { .. } => None,
        }
    }

    /// Returns whether a complete topological order exists.
    #[must_use]
    pub const fn is_acyclic(&self) -> bool {
        matches!(self, Self::Order(_))
    }
}

/// Reusable read-only indexes and derived analysis for one graph/edge-set pair.
///
/// The graph and edge set are borrowed, not copied. The configuration is
/// copied so the analysis object can enforce the same explicit work policy
/// during later reachability queries.
///
/// Do not retain this object after mutating or replacing the graph or edge
/// set it borrows. The Rust borrow rules prevent mutation of the borrowed
/// objects while this analysis instance is alive.
#[derive(Debug)]
pub struct HybridGraphAnalysis<'a> {
    graph: &'a HybridGraph,
    edges: &'a HybridEdgeSet,
    config: HybridConfig,

    nodes_by_id: BTreeMap<HybridNodeId, &'a HybridNode>,
    outgoing_by_node: BTreeMap<HybridNodeId, Vec<&'a HybridEdge>>,
    incoming_by_node: BTreeMap<HybridNodeId, Vec<&'a HybridEdge>>,

    summary: GraphAnalysisSummary,
    edge_validation: EdgeValidationReport,
}

impl<'a> HybridGraphAnalysis<'a> {
    /// Validate and index a graph and its associated edge set.
    ///
    /// This constructor:
    ///
    /// 1. validates the graph using the supplied operational configuration;
    /// 2. validates edge references against that graph;
    /// 3. checks configured validation-work accounting;
    /// 4. builds deterministic lookup and adjacency indexes;
    /// 5. derives summary statistics.
    ///
    /// Required unknown edge-extension semantics are rejected by the
    /// canonical edge validator. Optional unresolved extensions are preserved
    /// in the edge-validation report and are not treated as understood.
    ///
    /// This constructor does not resolve canonical IR references against
    /// their owning domain IRs and does not claim semantic verification.
    pub fn new(
        graph: &'a HybridGraph,
        edges: &'a HybridEdgeSet,
        config: &HybridConfig,
    ) -> Result<Self, HybridError> {
        graph
            .validate_with_config(config)
            .map_err(|error| error.in_component("hybrid graph analysis"))?;

        if edges.graph_id() != graph.id() {
            return Err(HybridError::new(
                HybridErrorCode::InvalidReference,
                format!(
                    "analysis graph {} does not match edge-set graph {}",
                    graph.id(),
                    edges.graph_id()
                ),
            )
            .in_component("hybrid graph analysis"));
        }

        let edge_validation = edges
            .validate_against_with_config(graph, config)
            .map_err(|error| error.in_component("hybrid graph analysis"))?;

        let node_count = count_to_u128(graph.nodes().len(), "graph node count")?;
        let edge_count = count_to_u128(edges.len(), "graph edge count")?;

        let initial_work = checked_count_sum(
            node_count,
            edge_count,
            "initial graph-analysis work",
        )?;

        config
            .check_budget(BudgetKind::ValidationSteps, initial_work)
            .map_err(|error| error.in_component("hybrid graph analysis"))?;

        let mut nodes_by_id = BTreeMap::new();

        for node in graph.nodes() {
            if nodes_by_id.insert(node.id(), node).is_some() {
                // Normally caught by HybridGraph::validate. Keep this
                // invariant check at the index-construction boundary.
                return Err(HybridError::new(
                    HybridErrorCode::DuplicateIdentity,
                    format!(
                        "duplicate node identity {} while indexing graph",
                        node.id()
                    ),
                )
                .in_component("hybrid graph analysis"));
            }
        }

        let mut outgoing_by_node: BTreeMap<HybridNodeId, Vec<&HybridEdge>> =
            BTreeMap::new();
        let mut incoming_by_node: BTreeMap<HybridNodeId, Vec<&HybridEdge>> =
            BTreeMap::new();

        for node_id in nodes_by_id.keys().copied() {
            outgoing_by_node.insert(node_id, Vec::new());
            incoming_by_node.insert(node_id, Vec::new());
        }

        for edge in edges.edges() {
            let outgoing = outgoing_by_node.get_mut(&edge.source()).ok_or_else(|| {
                HybridError::invalid_reference(format!(
                    "edge {} has source node {} missing from the analysis index",
                    edge.id(),
                    edge.source()
                ))
                .in_component("hybrid graph analysis")
            })?;

            outgoing.push(edge);

            let incoming = incoming_by_node.get_mut(&edge.target()).ok_or_else(|| {
                HybridError::invalid_reference(format!(
                    "edge {} has target node {} missing from the analysis index",
                    edge.id(),
                    edge.target()
                ))
                .in_component("hybrid graph analysis")
            })?;

            incoming.push(edge);
        }

        // Canonical ordering makes repeated queries independent of edge
        // declaration order while preserving the underlying edge set.
        for adjacency in outgoing_by_node.values_mut() {
            adjacency.sort_by_key(|edge| edge.id());
        }

        for adjacency in incoming_by_node.values_mut() {
            adjacency.sort_by_key(|edge| edge.id());
        }

        let mut analysis = Self {
            graph,
            edges,
            config: config.clone(),
            nodes_by_id,
            outgoing_by_node,
            incoming_by_node,
            summary: GraphAnalysisSummary::default(),
            edge_validation,
        };

        analysis.summary = analysis.compute_summary()?;
        Ok(analysis)
    }

    /// Return the graph analyzed by this instance.
    #[must_use]
    pub const fn graph(&self) -> &'a HybridGraph {
        self.graph
    }

    /// Return the edge set analyzed by this instance.
    #[must_use]
    pub const fn edges(&self) -> &'a HybridEdgeSet {
        self.edges
    }

    /// Return the derived summary.
    #[must_use]
    pub const fn summary(&self) -> &GraphAnalysisSummary {
        &self.summary
    }

    /// Return the result of canonical structural edge validation.
    ///
    /// This includes unresolved optional extension identities. It does not
    /// mean that the semantics of those extensions are understood.
    #[must_use]
    pub const fn edge_validation_report(&self) -> &EdgeValidationReport {
        &self.edge_validation
    }

    /// Look up a node by typed identity.
    #[must_use]
    pub fn node(&self, id: HybridNodeId) -> Option<&'a HybridNode> {
        self.nodes_by_id.get(&id).copied()
    }

    /// Return node identities in deterministic order.
    #[must_use]
    pub fn node_ids(&self) -> impl Iterator<Item = HybridNodeId> + '_ {
        self.nodes_by_id.keys().copied()
    }

    /// Return outgoing edges in deterministic edge-ID order.
    ///
    /// An unknown node yields an empty iterator. Use `contains_node` first
    /// when the caller must distinguish an unknown identity from a node with
    /// no outgoing edges.
    #[must_use]
    pub fn outgoing_edges(
        &self,
        node: HybridNodeId,
    ) -> impl Iterator<Item = &'a HybridEdge> + '_ {
        self.outgoing_by_node
            .get(&node)
            .into_iter()
            .flat_map(|edges| edges.iter().copied())
    }

    /// Return incoming edges in deterministic edge-ID order.
    ///
    /// An unknown node yields an empty iterator. Use `contains_node` first
    /// when the caller must distinguish an unknown identity from a node with
    /// no incoming edges.
    #[must_use]
    pub fn incoming_edges(
        &self,
        node: HybridNodeId,
    ) -> impl Iterator<Item = &'a HybridEdge> + '_ {
        self.incoming_by_node
            .get(&node)
            .into_iter()
            .flat_map(|edges| edges.iter().copied())
    }

    /// Return whether the graph contains the supplied node identity.
    #[must_use]
    pub fn contains_node(&self, node: HybridNodeId) -> bool {
        self.nodes_by_id.contains_key(&node)
    }

    /// Return whether a node is structurally reachable from `start`.
    ///
    /// A node is reachable from itself if it exists in the graph. This query
    /// follows all directed edge kinds; it does not interpret their domain
    /// semantics. The explicit validation-step budget is checked as work is
    /// performed, rather than only after the traversal has completed.
    pub fn is_reachable(
        &self,
        start: HybridNodeId,
        target: HybridNodeId,
    ) -> Result<bool, HybridError> {
        self.require_node(start)?;
        self.require_node(target)?;

        if start == target {
            return Ok(true);
        }

        let mut visited = BTreeSet::new();
        let mut pending = VecDeque::new();
        let mut work = 0_u128;

        visited.insert(start);
        pending.push_back(start);

        while let Some(current) = pending.pop_front() {
            for edge in self.outgoing_edges(current) {
                work = checked_count_sum(work, 1, "reachability work")?;
                self.check_query_work(work)?;

                let next = edge.target();

                if next == target {
                    return Ok(true);
                }

                if visited.insert(next) {
                    pending.push_back(next);
                }
            }
        }

        Ok(false)
    }

    /// Return all nodes reachable from `start`, including `start`.
    ///
    /// Traversal follows every edge kind. The returned set is ordered by
    /// typed node identity. This is structural reachability, not a proof that
    /// every path is executable or semantically valid.
    pub fn reachable_from(
        &self,
        start: HybridNodeId,
    ) -> Result<BTreeSet<HybridNodeId>, HybridError> {
        self.require_node(start)?;

        let mut visited = BTreeSet::new();
        let mut pending = VecDeque::new();
        let mut work = 0_u128;

        visited.insert(start);
        pending.push_back(start);

        while let Some(current) = pending.pop_front() {
            for edge in self.outgoing_edges(current) {
                work = checked_count_sum(work, 1, "reachability work")?;
                self.check_query_work(work)?;

                let next = edge.target();

                if visited.insert(next) {
                    pending.push_back(next);
                }
            }
        }

        Ok(visited)
    }

    /// Return a deterministic topological-order result for all graph edges.
    ///
    /// Every edge is treated as a prerequisite relationship for this
    /// structural query. If feedback or loop edges make the graph cyclic,
    /// the method returns `TopologicalOrder::Cyclic`; it does not reject the
    /// graph or claim that the represented program is invalid.
    ///
    /// If a compiler requires a topological order over a particular subset
    /// of dependency kinds, that policy must be explicit in the relevant
    /// semantic or planning layer rather than silently applied here.
    pub fn topological_order(&self) -> Result<TopologicalOrder, HybridError> {
        let mut in_degree: BTreeMap<HybridNodeId, u128> = BTreeMap::new();

        for node_id in self.nodes_by_id.keys().copied() {
            in_degree.insert(
                node_id,
                count_to_u128(
                    self.incoming_by_node
                        .get(&node_id)
                        .map_or(0, Vec::len),
                    "node in-degree",
                )?,
            );
        }

        let mut ready = BTreeSet::new();

        for (node_id, degree) in &in_degree {
            if *degree == 0 {
                ready.insert(*node_id);
            }
        }

        let mut order = Vec::with_capacity(self.nodes_by_id.len());
        let mut work = 0_u128;

        while let Some(node_id) = ready.pop_first() {
            order.push(node_id);

            for edge in self.outgoing_edges(node_id) {
                work = checked_count_sum(work, 1, "topological analysis work")?;
                self.check_query_work(work)?;

                let target_degree = in_degree.get_mut(&edge.target()).ok_or_else(|| {
                    HybridError::invalid_reference(format!(
                        "edge {} targets node {} missing from the in-degree index",
                        edge.id(),
                        edge.target()
                    ))
                })?;

                *target_degree = target_degree.checked_sub(1).ok_or_else(|| {
                    HybridError::new(
                        HybridErrorCode::InternalInvariantViolation,
                        format!(
                            "in-degree underflow while processing edge {}",
                            edge.id()
                        ),
                    )
                })?;

                if *target_degree == 0 {
                    ready.insert(edge.target());
                }
            }
        }

        if order.len() == self.nodes_by_id.len() {
            return Ok(TopologicalOrder::Order(order));
        }

        let remaining_nodes = in_degree
            .into_iter()
            .filter_map(|(node_id, degree)| (degree > 0).then_some(node_id))
            .collect();

        Ok(TopologicalOrder::Cyclic { remaining_nodes })
    }

    /// Return weakly connected components in deterministic order.
    ///
    /// Edge direction is ignored. Every graph node appears in exactly one
    /// component. Nodes inside each component and components themselves are
    /// ordered by their smallest node identity.
    pub fn weakly_connected_components(
        &self,
    ) -> Result<Vec<Vec<HybridNodeId>>, HybridError> {
        let mut visited = BTreeSet::new();
        let mut components = Vec::new();
        let mut work = 0_u128;

        for start in self.nodes_by_id.keys().copied() {
            if visited.contains(&start) {
                continue;
            }

            let mut component = Vec::new();
            let mut pending = VecDeque::new();

            visited.insert(start);
            pending.push_back(start);

            while let Some(current) = pending.pop_front() {
                component.push(current);

                for edge in self.outgoing_edges(current) {
                    work = checked_count_sum(work, 1, "component analysis work")?;
                    self.check_query_work(work)?;

                    let neighbor = edge.target();

                    if visited.insert(neighbor) {
                        pending.push_back(neighbor);
                    }
                }

                for edge in self.incoming_edges(current) {
                    work = checked_count_sum(work, 1, "component analysis work")?;
                    self.check_query_work(work)?;

                    let neighbor = edge.source();

                    if visited.insert(neighbor) {
                        pending.push_back(neighbor);
                    }
                }
            }

            component.sort();
            components.push(component);
        }

        components.sort_by_key(|component| component.first().copied());
        Ok(components)
    }

    fn compute_summary(&self) -> Result<GraphAnalysisSummary, HybridError> {
        let mut domains: BTreeSet<DomainId> = BTreeSet::new();
        let mut cross_domain_edge_count = 0_u128;
        let mut declared_domain_boundary_edge_count = 0_u128;
        let mut source_node_count = 0_u128;
        let mut sink_node_count = 0_u128;
        let mut isolated_node_count = 0_u128;
        let mut maximum_in_degree = 0_u128;
        let mut maximum_out_degree = 0_u128;
        let mut work = 0_u128;

        for node in self.nodes_by_id.values() {
            let instance = self
                .graph
                .composition()
                .get(node.domain_instance())
                .ok_or_else(|| {
                    HybridError::invalid_reference(format!(
                        "node {} refers to missing domain instance {} during analysis",
                        node.id(),
                        node.domain_instance()
                    ))
                })?;

            domains.insert(
                instance
                    .domain_id()
                    .map_err(|error| error.in_component("hybrid graph analysis"))?,
            );

            let in_degree = count_to_u128(
                self.incoming_by_node
                    .get(&node.id())
                    .map_or(0, Vec::len),
                "node in-degree",
            )?;

            let out_degree = count_to_u128(
                self.outgoing_by_node
                    .get(&node.id())
                    .map_or(0, Vec::len),
                "node out-degree",
            )?;

            maximum_in_degree = maximum_in_degree.max(in_degree);
            maximum_out_degree = maximum_out_degree.max(out_degree);

            if in_degree == 0 {
                source_node_count = checked_count_sum(
                    source_node_count,
                    1,
                    "source-node count",
                )?;
            }

            if out_degree == 0 {
                sink_node_count = checked_count_sum(
                    sink_node_count,
                    1,
                    "sink-node count",
                )?;
            }

            if in_degree == 0 && out_degree == 0 {
                isolated_node_count = checked_count_sum(
                    isolated_node_count,
                    1,
                    "isolated-node count",
                )?;
            }

            work = checked_count_sum(work, 1, "summary-analysis work")?;
            self.check_query_work(work)?;
        }

        for edge in self.edges.edges() {
            work = checked_count_sum(work, 1, "summary-analysis work")?;
            self.check_query_work(work)?;

            let source = self.require_node(edge.source())?;
            let target = self.require_node(edge.target())?;

            let source_instance = self
                .graph
                .composition()
                .get(source.domain_instance())
                .ok_or_else(|| {
                    HybridError::invalid_reference(format!(
                        "source node {} references missing domain instance {}",
                        source.id(),
                        source.domain_instance()
                    ))
                })?;

            let target_instance = self
                .graph
                .composition()
                .get(target.domain_instance())
                .ok_or_else(|| {
                    HybridError::invalid_reference(format!(
                        "target node {} references missing domain instance {}",
                        target.id(),
                        target.domain_instance()
                    ))
                })?;

            let source_domain = source_instance
                .domain_id()
                .map_err(|error| error.in_component("hybrid graph analysis"))?;

            let target_domain = target_instance
                .domain_id()
                .map_err(|error| error.in_component("hybrid graph analysis"))?;

            if source_domain != target_domain {
                cross_domain_edge_count = checked_count_sum(
                    cross_domain_edge_count,
                    1,
                    "cross-domain edge count",
                )?;
            }

            if matches!(edge.kind(), EdgeKind::DomainBoundary) {
                declared_domain_boundary_edge_count = checked_count_sum(
                    declared_domain_boundary_edge_count,
                    1,
                    "declared domain-boundary edge count",
                )?;
            }
        }

        let weakly_connected_component_count = count_to_u128(
            self.weakly_connected_components()?.len(),
            "weakly connected component count",
        )?;

        Ok(GraphAnalysisSummary {
            node_count: count_to_u128(self.nodes_by_id.len(), "node count")?,
            edge_count: count_to_u128(self.edges.len(), "edge count")?,
            participating_domain_count: count_to_u128(
                domains.len(),
                "participating domain count",
            )?,
            cross_domain_edge_count,
            declared_domain_boundary_edge_count,
            source_node_count,
            sink_node_count,
            isolated_node_count,
            maximum_in_degree,
            maximum_out_degree,
            weakly_connected_component_count,
        })
    }

    fn require_node(
        &self,
        node_id: HybridNodeId,
    ) -> Result<&'a HybridNode, HybridError> {
        self.nodes_by_id.get(&node_id).copied().ok_or_else(|| {
            HybridError::invalid_reference(format!(
                "node {node_id} does not exist in analyzed graph {}",
                self.graph.id()
            ))
            .in_component("hybrid graph analysis")
        })
    }

    fn check_query_work(&self, work: u128) -> Result<(), HybridError> {
        self.config
            .check_budget(BudgetKind::ValidationSteps, work)
            .map_err(|error| error.in_component("hybrid graph analysis query"))
    }
}

fn count_to_u128(value: usize, description: &str) -> Result<u128, HybridError> {
    u128::try_from(value).map_err(|_| {
        HybridError::new(
            HybridErrorCode::InvalidRepresentation,
            format!("{description} cannot be represented as u128"),
        )
    })
}

fn checked_count_sum(
    left: u128,
    right: u128,
    description: &str,
) -> Result<u128, HybridError> {
    left.checked_add(right).ok_or_else(|| {
        HybridError::new(
            HybridErrorCode::InvalidRepresentation,
            format!("{description} overflowed its accounting representation"),
        )
    })
}
