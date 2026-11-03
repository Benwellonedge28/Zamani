
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani AI Optimization — Graph Analysis
//!
//! Path: src/ai/optimization/graph.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Provides target-independent structural analysis of canonical AI
//! computational graphs.
//!
//! This module analyzes graph topology without owning or duplicating
//! the canonical graph representation.
//!
//! ## Ownership
//!
//! This module owns:
//! - graph topology analysis;
//! - deterministic topological ordering;
//! - dependency degree analysis;
//! - root and sink identification;
//! - isolated-node identification;
//! - cycle-related structural reporting;
//! - graph structural diagnostics.
//!
//! It does not own:
//! - model definitions;
//! - model graph construction;
//! - pipeline definitions;
//! - pipeline graph construction;
//! - source AST or semantic analysis;
//! - tensor storage;
//! - computational IR;
//! - graph transformations;
//! - transformation legality;
//! - cost models;
//! - hardware discovery;
//! - resource allocation;
//! - scheduling;
//! - placement;
//! - distributed execution;
//! - runtime execution.
//!
//! ## Canonical graph authorities
//!
//! AI model graph:
//!     crate::ai::model::graph
//!
//! AI pipeline graph:
//!     crate::ai::pipeline::graph
//!
//! AI semantic authority:
//!     crate::frontend::semantic::domains::ai
//!
//! Universal computational IR:
//!     crate::zuir
//!
//! Graph transformations:
//!     crate::ai::optimization::transformations
//!
//! Transformation legality:
//!     crate::ai::optimization::legality
//!
//! Cost estimation:
//!     crate::ai::optimization::cost_model
//!
//! ## Graph semantics
//!
//! Edges are directed.
//!
//! Parallel edges are preserved and independently contribute to
//! incoming and outgoing degree.
//!
//! Self-edges are structurally accepted and reported.
//!
//! Cycles are structurally analyzed rather than silently discarded.
//!
//! A topological ordering exists only when the graph is acyclic.
//!
//! When a cycle exists, the topological prefix and blocked nodes
//! are reported separately.
//!
//! Blocked nodes may include nodes downstream of a cycle. They
//! must not be incorrectly reported as exact cycle members.
//!
//! ## Determinism
//!
//! Nodes are ordered using their canonical Ord implementation.
//!
//! Among simultaneously ready nodes, the smallest canonical
//! identifier is selected first.
//!
//! The same node and edge inputs therefore produce the same report.
//!
//! ## Scalability
//!
//! There is no fixed maximum for:
//! - graph nodes;
//! - graph edges;
//! - graph depth;
//! - fan-in;
//! - fan-out;
//! - connected relationships;
//! - model parameters;
//! - computational stages.
//!
//! Actual capacity is determined by available memory,
//! allocator capacity, platform addressability and explicit
//! operational resource policies.
//!
//! Allocation failure is reported rather than treated as success.
//!
//! The analysis uses iterative algorithms rather than recursive
//! graph traversal.
//!
//! ## Complexity
//!
//! Let V be the number of nodes and E the number of edges.
//!
//! Construction:
//!     O(V log V + E log V)
//!
//! Topological analysis:
//!     O((V + E) log V)
//!
//! Adjacency storage:
//!     O(V + E)
//!
//! No physical execution or hardware-specific cost is inferred.
//!
//! ## Integration
//!
//! Callers obtain nodes and edges from their canonical graph and
//! supply their identifiers to analyze_graph().
//!
//! This keeps graph ownership in the model and pipeline modules.
//!
//! The resulting analysis can be consumed by:
//! - optimization transformations;
//! - optimization legality analysis;
//! - cost models;
//! - compiler optimization planning;
//! - ZUIR integration.
//!
//! No second graph representation is introduced as a source of truth.

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::error::Error;
use std::fmt;

// ============================================================================
// ERRORS
// ============================================================================

/// Errors encountered while analyzing an optimization graph.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum OptimizationGraphError<N> {
    /// The input contains the same canonical node identity more than once.
    DuplicateNode {
        node: N,
    },

    /// An edge references a node that was not declared.
    MissingEndpoint {
        endpoint: N,
        edge_index: usize,
    },

    /// An allocation reservation failed.
    ///
    /// This is an operational resource failure, not a graph semantic error.
    AllocationFailed {
        structure: &'static str,
    },

    /// An internal degree or index calculation overflowed.
    IndexOverflow {
        structure: &'static str,
    },

    /// An internal graph invariant was violated.
    InvariantViolation {
        reason: &'static str,
    },
}

impl<N: fmt::Debug> fmt::Display for OptimizationGraphError<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateNode { node } => {
                write!(f, "duplicate optimization graph node: {node:?}")
            }

            Self::MissingEndpoint {
                endpoint,
                edge_index,
            } => {
                write!(
                    f,
                    "optimization graph edge {edge_index} references \
                     undeclared node {endpoint:?}"
                )
            }

            Self::AllocationFailed { structure } => {
                write!(
                    f,
                    "unable to reserve optimization graph storage: \
                     {structure}"
                )
            }

            Self::IndexOverflow { structure } => {
                write!(
                    f,
                    "optimization graph index overflow: {structure}"
                )
            }

            Self::InvariantViolation { reason } => {
                write!(
                    f,
                    "optimization graph invariant violation: {reason}"
                )
            }
        }
    }
}

impl<N: fmt::Debug> Error for OptimizationGraphError<N> {}

// ============================================================================
// GRAPH ANALYSIS REPORT
// ============================================================================

/// Deterministic structural analysis of a canonical computational graph.
///
/// This is an analysis result, not a graph representation or executable IR.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OptimizationGraphAnalysis<N> {
    node_count: usize,
    edge_count: usize,

    roots: Vec<N>,
    sinks: Vec<N>,
    isolated_nodes: Vec<N>,

    topological_prefix: Vec<N>,
    blocked_nodes: Vec<N>,

    maximum_in_degree: Option<usize>,
    maximum_out_degree: Option<usize>,

    self_loop_count: usize,
}

impl<N> OptimizationGraphAnalysis<N> {
    /// Returns the number of declared graph nodes.
    #[must_use]
    pub const fn node_count(&self) -> usize {
        self.node_count
    }

    /// Returns the number of directed graph edges.
    ///
    /// Parallel edges are counted independently.
    #[must_use]
    pub const fn edge_count(&self) -> usize {
        self.edge_count
    }

    /// Returns true when the graph contains no nodes.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.node_count == 0
    }

    /// Returns true when the graph contains no edges.
    #[must_use]
    pub const fn has_no_edges(&self) -> bool {
        self.edge_count == 0
    }

    /// Returns true when a complete topological ordering exists.
    #[must_use]
    pub fn is_acyclic(&self) -> bool {
        self.blocked_nodes.is_empty()
    }

    /// Returns the number of self-loop edges.
    #[must_use]
    pub const fn self_loop_count(&self) -> usize {
        self.self_loop_count
    }

    /// Returns whether at least one self-loop exists.
    #[must_use]
    pub const fn has_self_loops(&self) -> bool {
        self.self_loop_count > 0
    }

    /// Returns nodes with no incoming edges.
    ///
    /// Isolated nodes are also roots.
    #[must_use]
    pub fn roots(&self) -> &[N] {
        &self.roots
    }

    /// Returns nodes with no outgoing edges.
    ///
    /// Isolated nodes are also sinks.
    #[must_use]
    pub fn sinks(&self) -> &[N] {
        &self.sinks
    }

    /// Returns nodes with neither incoming nor outgoing edges.
    #[must_use]
    pub fn isolated_nodes(&self) -> &[N] {
        &self.isolated_nodes
    }

    /// Returns the complete topological ordering when the graph is acyclic.
    ///
    /// Returns None when one or more nodes are blocked by cyclic
    /// dependencies.
    #[must_use]
    pub fn topological_order(&self) -> Option<&[N]> {
        self.is_acyclic()
            .then_some(&self.topological_prefix)
    }

    /// Returns the nodes successfully emitted by Kahn's algorithm.
    ///
    /// For an acyclic graph this is the complete topological ordering.
    ///
    /// For a cyclic graph this is a valid partial ordering of the
    /// portion that could be processed.
    #[must_use]
    pub fn topological_prefix(&self) -> &[N] {
        &self.topological_prefix
    }

    /// Returns nodes that could not be emitted by Kahn's algorithm.
    ///
    /// Important:
    ///
    /// These nodes may include both cycle members and nodes downstream
    /// of cycles. They are not an exact cycle-membership report.
    #[must_use]
    pub fn blocked_nodes(&self) -> &[N] {
        &self.blocked_nodes
    }

    /// Returns the largest observed incoming degree.
    ///
    /// Returns None for an empty graph.
    #[must_use]
    pub const fn maximum_in_degree(&self) -> Option<usize> {
        self.maximum_in_degree
    }

    /// Returns the largest observed outgoing degree.
    ///
    /// Returns None for an empty graph.
    #[must_use]
    pub const fn maximum_out_degree(&self) -> Option<usize> {
        self.maximum_out_degree
    }

    /// Returns whether the graph has a complete topological ordering.
    #[must_use]
    pub fn has_complete_topological_order(&self) -> bool {
        self.topological_prefix.len() == self.node_count
            && self.blocked_nodes.is_empty()
    }
}

// ============================================================================
// INTERNAL INDEXED GRAPH
// ============================================================================

/// Internal adjacency representation.
///
/// This is temporary analysis storage. The canonical graph remains owned
/// by the caller.
struct IndexedGraph {
    outgoing: Vec<Vec<usize>>,
    incoming: Vec<Vec<usize>>,
    self_loop_count: usize,
}

fn allocation_error(
    structure: &'static str,
) -> OptimizationGraphError<impl Sized> {
    OptimizationGraphError::AllocationFailed { structure }
}

// ============================================================================
// GRAPH ANALYSIS
// ============================================================================

/// Analyze a canonical directed graph without taking ownership of it.
///
/// `nodes` supplies canonical node identities.
///
/// `edges` supplies directed `(source, destination)` identity pairs.
///
/// Parallel edges are preserved.
///
/// The input graph is not modified.
///
/// Node identities must implement Ord and Clone so that analysis is
/// deterministic and can retain identifiers in its report.
///
/// # Errors
///
/// Returns:
///
/// - DuplicateNode when node identities are repeated.
/// - MissingEndpoint when an edge references an undeclared node.
/// - AllocationFailed when a fallible vector allocation fails.
/// - IndexOverflow when a degree or index cannot be represented.
/// - InvariantViolation when internal consistency checks fail.
///
/// # Scalability
///
/// There are no application-defined graph-size limits.
///
/// The process remains subject to actual memory and address-space limits.
///
/// # Example
///
/// ```
/// use zamani::ai::optimization::graph::analyze_graph;
///
/// let nodes = vec![1_u64, 2, 3, 4];
///
/// let edges = vec![
///     (1_u64, 2_u64),
///     (2, 3),
///     (1, 3),
/// ];
///
/// let result = analyze_graph(nodes, edges).unwrap();
///
/// assert_eq!(result.node_count(), 4);
/// assert_eq!(result.edge_count(), 3);
/// assert!(result.is_acyclic());
/// assert_eq!(
///     result.topological_order(),
///     Some(&[1, 2, 3, 4][..])
/// );
/// ```
pub fn analyze_graph<N, NI, EI>(
    nodes: NI,
    edges: EI,
) -> Result<OptimizationGraphAnalysis<N>, OptimizationGraphError<N>>
where
    N: Ord + Clone,
    NI: IntoIterator<Item = N>,
    EI: IntoIterator<Item = (N, N)>,
{
    // ------------------------------------------------------------------------
    // Collect and canonicalize node identities.
    // ------------------------------------------------------------------------

    let mut ordered_nodes = Vec::new();

    for node in nodes {
        ordered_nodes
            .try_reserve(1)
            .map_err(|_| OptimizationGraphError::AllocationFailed {
                structure: "node identities",
            })?;

        ordered_nodes.push(node);
    }

    ordered_nodes.sort();

    for pair in ordered_nodes.windows(2) {
        if pair[0] == pair[1] {
            return Err(OptimizationGraphError::DuplicateNode {
                node: pair[0].clone(),
            });
        }
    }

    let node_count = ordered_nodes.len();

    // ------------------------------------------------------------------------
    // Resolve edge endpoints against canonical node identities.
    //
    // Store index pairs rather than introducing another identity system.
    // ------------------------------------------------------------------------

    let mut indexed_edges: Vec<(usize, usize)> = Vec::new();

    let mut outgoing_counts = Vec::new();
    outgoing_counts
        .try_reserve_exact(node_count)
        .map_err(|_| OptimizationGraphError::AllocationFailed {
            structure: "outgoing degree counters",
        })?;

    outgoing_counts.resize(node_count, 0usize);

    let mut incoming_counts = Vec::new();
    incoming_counts
        .try_reserve_exact(node_count)
        .map_err(|_| OptimizationGraphError::AllocationFailed {
            structure: "incoming degree counters",
        })?;

    incoming_counts.resize(node_count, 0usize);

    let mut self_loop_count = 0usize;

    for (edge_index, (source, destination)) in
        edges.into_iter().enumerate()
    {
        let source_index = ordered_nodes
            .binary_search(&source)
            .map_err(|_| OptimizationGraphError::MissingEndpoint {
                endpoint: source,
                edge_index,
            })?;

        let destination_index = ordered_nodes
            .binary_search(&destination)
            .map_err(|_| OptimizationGraphError::MissingEndpoint {
                endpoint: destination,
                edge_index,
            })?;

        outgoing_counts[source_index] = outgoing_counts[source_index]
            .checked_add(1)
            .ok_or(OptimizationGraphError::IndexOverflow {
                structure: "outgoing degree",
            })?;

        incoming_counts[destination_index] =
            incoming_counts[destination_index]
                .checked_add(1)
                .ok_or(OptimizationGraphError::IndexOverflow {
                    structure: "incoming degree",
                })?;

        if source_index == destination_index {
            self_loop_count = self_loop_count
                .checked_add(1)
                .ok_or(OptimizationGraphError::IndexOverflow {
                    structure: "self-loop count",
                })?;
        }

        indexed_edges
            .try_reserve(1)
            .map_err(|_| OptimizationGraphError::AllocationFailed {
                structure: "indexed edges",
            })?;

        indexed_edges.push((source_index, destination_index));
    }

    let edge_count = indexed_edges.len();

    // ------------------------------------------------------------------------
    // Allocate adjacency storage.
    // ------------------------------------------------------------------------

    let mut outgoing = Vec::new();

    outgoing
        .try_reserve_exact(node_count)
        .map_err(|_| OptimizationGraphError::AllocationFailed {
            structure: "outgoing adjacency",
        })?;

    outgoing.resize_with(node_count, Vec::new);

    let mut incoming = Vec::new();

    incoming
        .try_reserve_exact(node_count)
        .map_err(|_| OptimizationGraphError::AllocationFailed {
            structure: "incoming adjacency",
        })?;

    incoming.resize_with(node_count, Vec::new);

    // Reserve each adjacency list according to its actual degree.
    for index in 0..node_count {
        outgoing[index]
            .try_reserve_exact(outgoing_counts[index])
            .map_err(|_| OptimizationGraphError::AllocationFailed {
                structure: "outgoing adjacency entries",
            })?;

        incoming[index]
            .try_reserve_exact(incoming_counts[index])
            .map_err(|_| OptimizationGraphError::AllocationFailed {
                structure: "incoming adjacency entries",
            })?;
    }

    // ------------------------------------------------------------------------
    // Populate adjacency.
    //
    // Parallel edges remain separate entries.
    // ------------------------------------------------------------------------

    for &(source, destination) in &indexed_edges {
        outgoing[source].push(destination);
        incoming[destination].push(source);
    }

    // ------------------------------------------------------------------------
    // Calculate degree summaries and graph boundary nodes.
    // ------------------------------------------------------------------------

    let mut roots = Vec::new();
    let mut sinks = Vec::new();
    let mut isolated_nodes = Vec::new();

    roots
        .try_reserve_exact(node_count)
        .map_err(|_| OptimizationGraphError::AllocationFailed {
            structure: "root nodes",
        })?;

    sinks
        .try_reserve_exact(node_count)
        .map_err(|_| OptimizationGraphError::AllocationFailed {
            structure: "sink nodes",
        })?;

    isolated_nodes
        .try_reserve_exact(node_count)
        .map_err(|_| OptimizationGraphError::AllocationFailed {
            structure: "isolated nodes",
        })?;

    let mut maximum_in_degree = None;
    let mut maximum_out_degree = None;

    for index in 0..node_count {
        let in_degree = incoming[index].len();
        let out_degree = outgoing[index].len();

        maximum_in_degree = Some(
            maximum_in_degree
                .map_or(in_degree, |current: usize| current.max(in_degree)),
        );

        maximum_out_degree = Some(
            maximum_out_degree
                .map_or(out_degree, |current: usize| current.max(out_degree)),
        );

        if in_degree == 0 {
            roots.push(ordered_nodes[index].clone());
        }

        if out_degree == 0 {
            sinks.push(ordered_nodes[index].clone());
        }

        if in_degree == 0 && out_degree == 0 {
            isolated_nodes.push(ordered_nodes[index].clone());
        }
    }

    // ------------------------------------------------------------------------
    // Kahn's topological ordering algorithm.
    //
    // The priority queue ensures that simultaneously ready nodes are
    // selected in canonical identifier order.
    //
    // No recursive traversal is used.
    // ------------------------------------------------------------------------

    let mut remaining_dependencies = Vec::new();

    remaining_dependencies
        .try_reserve_exact(node_count)
        .map_err(|_| OptimizationGraphError::AllocationFailed {
            structure: "topological degree counters",
        })?;

    remaining_dependencies.extend(
        incoming.iter().map(Vec::len),
    );

    let mut ready = BinaryHeap::new();

    ready
        .try_reserve(node_count)
        .map_err(|_| OptimizationGraphError::AllocationFailed {
            structure: "topological ready queue",
        })?;

    for (index, degree) in remaining_dependencies.iter().enumerate() {
        if *degree == 0 {
            ready.push(Reverse(index));
        }
    }

    let mut topological_indices = Vec::new();

    topological_indices
        .try_reserve_exact(node_count)
        .map_err(|_| OptimizationGraphError::AllocationFailed {
            structure: "topological ordering",
        })?;

    while let Some(Reverse(current)) = ready.pop() {
        topological_indices.push(current);

        for &successor in &outgoing[current] {
            let remaining = remaining_dependencies
                .get_mut(successor)
                .ok_or(OptimizationGraphError::InvariantViolation {
                    reason: "successor index is outside the node collection",
                })?;

            *remaining = remaining.checked_sub(1).ok_or(
                OptimizationGraphError::InvariantViolation {
                    reason: "topological degree counter underflow",
                },
            )?;

            if *remaining == 0 {
                ready.push(Reverse(successor));
            }
        }
    }

    // ------------------------------------------------------------------------
    // Determine the blocked nodes.
    //
    // These are nodes that could not be emitted by Kahn's algorithm.
    //
    // They are not necessarily all members of a cycle.
    // ------------------------------------------------------------------------

    let mut blocked_nodes = Vec::new();

    blocked_nodes
        .try_reserve_exact(
            node_count.saturating_sub(topological_indices.len()),
        )
        .map_err(|_| OptimizationGraphError::AllocationFailed {
            structure: "blocked nodes",
        })?;

    for (index, degree) in remaining_dependencies.iter().enumerate() {
        if *degree > 0 {
            blocked_nodes.push(ordered_nodes[index].clone());
        }
    }

    // ------------------------------------------------------------------------
    // Convert the topological index sequence into canonical identifiers.
    // ------------------------------------------------------------------------

    let mut topological_prefix = Vec::new();

    topological_prefix
        .try_reserve_exact(topological_indices.len())
        .map_err(|_| OptimizationGraphError::AllocationFailed {
            structure: "topological identifiers",
        })?;

    for index in topological_indices {
        let node = ordered_nodes.get(index).ok_or(
            OptimizationGraphError::InvariantViolation {
                reason: "topological index is outside the node collection",
            },
        )?;

        topological_prefix.push(node.clone());
    }

    // ------------------------------------------------------------------------
    // Internal consistency checks.
    // ------------------------------------------------------------------------

    if topological_prefix.len() + blocked_nodes.len() != node_count {
        return Err(OptimizationGraphError::InvariantViolation {
            reason: "topological and blocked node counts do not reconcile",
        });
    }

    let indexed_edge_count = outgoing.iter().try_fold(
        0usize,
        |total, adjacency| total.checked_add(adjacency.len()),
    );

    if indexed_edge_count != Some(edge_count) {
        return Err(OptimizationGraphError::InvariantViolation {
            reason: "outgoing adjacency does not match edge count",
        });
    }

    let incoming_edge_count = incoming.iter().try_fold(
        0usize,
        |total, adjacency| total.checked_add(adjacency.len()),
    );

    if incoming_edge_count != Some(edge_count) {
        return Err(OptimizationGraphError::InvariantViolation {
            reason: "incoming adjacency does not match edge count",
        });
    }

    Ok(OptimizationGraphAnalysis {
        node_count,
        edge_count,
        roots,
        sinks,
        isolated_nodes,
        topological_prefix,
        blocked_nodes,
        maximum_in_degree,
        maximum_out_degree,
        self_loop_count,
    })
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    type Node = u64;

    fn analyze(
        nodes: &[Node],
        edges: &[(Node, Node)],
    ) -> OptimizationGraphAnalysis<Node> {
        analyze_graph(
            nodes.iter().copied(),
            edges.iter().copied(),
        )
        .expect("valid graph should be analyzed")
    }

    #[test]
    fn analyzes_empty_graph() {
        let result = analyze_graph(
            Vec::<Node>::new(),
            Vec::<(Node, Node)>::new(),
        )
        .unwrap();

        assert!(result.is_empty());
        assert!(result.has_no_edges());
        assert!(result.is_acyclic());
        assert_eq!(result.topological_order(), Some(&[][..]));
        assert_eq!(result.maximum_in_degree(), None);
        assert_eq!(result.maximum_out_degree(), None);
    }

    #[test]
    fn analyzes_single_isolated_node() {
        let result = analyze(&[1], &[]);

        assert_eq!(result.node_count(), 1);
        assert_eq!(result.edge_count(), 0);
        assert_eq!(result.roots(), &[1]);
        assert_eq!(result.sinks(), &[1]);
        assert_eq!(result.isolated_nodes(), &[1]);
        assert!(result.is_acyclic());
    }

    #[test]
    fn analyzes_linear_graph() {
        let result = analyze(
            &[1, 2, 3, 4],
            &[(1, 2), (2, 3), (3, 4)],
        );

        assert!(result.is_acyclic());

        assert_eq!(
            result.topological_order(),
            Some(&[1, 2, 3, 4][..])
        );

        assert_eq!(result.roots(), &[1]);
        assert_eq!(result.sinks(), &[4]);

        assert_eq!(result.maximum_in_degree(), Some(1));
        assert_eq!(result.maximum_out_degree(), Some(1));
    }

    #[test]
    fn topological_order_is_deterministic() {
        let first = analyze(
            &[4, 3, 2, 1],
            &[(1, 3), (2, 3)],
        );

        let second = analyze(
            &[1, 2, 3, 4],
            &[(2, 3), (1, 3)],
        );

        assert_eq!(
            first.topological_order(),
            second.topological_order()
        );
    }

    #[test]
    fn preserves_parallel_edges() {
        let result = analyze(
            &[1, 2],
            &[(1, 2), (1, 2), (1, 2)],
        );

        assert_eq!(result.edge_count(), 3);
        assert_eq!(result.maximum_out_degree(), Some(3));
        assert_eq!(result.maximum_in_degree(), Some(3));
        assert!(result.is_acyclic());
    }

    #[test]
    fn identifies_fan_in_and_fan_out() {
        let result = analyze(
            &[1, 2, 3, 4, 5],
            &[
                (1, 3),
                (2, 3),
                (3, 4),
                (3, 5),
            ],
        );

        assert_eq!(result.maximum_in_degree(), Some(2));
        assert_eq!(result.maximum_out_degree(), Some(2));
        assert_eq!(result.roots(), &[1, 2]);
        assert_eq!(result.sinks(), &[4, 5]);
    }

    #[test]
    fn identifies_isolated_nodes() {
        let result = analyze(
            &[1, 2, 3, 4],
            &[(1, 2)],
        );

        assert_eq!(result.isolated_nodes(), &[3, 4]);
    }

    #[test]
    fn detects_self_loop() {
        let result = analyze(
            &[1, 2],
            &[(1, 1)],
        );

        assert!(!result.is_acyclic());
        assert_eq!(result.self_loop_count(), 1);
        assert!(result.has_self_loops());
        assert!(result.topological_order().is_none());
        assert_eq!(result.blocked_nodes(), &[1]);
    }

    #[test]
    fn detects_multi_node_cycle() {
        let result = analyze(
            &[1, 2, 3],
            &[
                (1, 2),
                (2, 3),
                (3, 1),
            ],
        );

        assert!(!result.is_acyclic());
        assert!(result.topological_order().is_none());

        assert_eq!(
            result.blocked_nodes(),
            &[1, 2, 3]
        );
    }

    #[test]
    fn distinguishes_cycle_blocked_nodes_from_exact_cycle_members() {
        let result = analyze(
            &[1, 2, 3, 4],
            &[
                (1, 2),
                (2, 1),
                (2, 3),
                (3, 4),
            ],
        );

        assert!(!result.is_acyclic());

        // Nodes 3 and 4 are downstream of the cycle.
        // They are blocked but are not necessarily cycle members.
        assert_eq!(
            result.blocked_nodes(),
            &[1, 2, 3, 4]
        );
    }

    #[test]
    fn retains_acyclic_prefix_when_cycle_exists() {
        let result = analyze(
            &[1, 2, 3, 4],
            &[
                (1, 2),
                (3, 4),
                (4, 3),
            ],
        );

        assert_eq!(result.topological_prefix(), &[1, 2]);
        assert_eq!(result.blocked_nodes(), &[3, 4]);
        assert!(result.topological_order().is_none());
    }

    #[test]
    fn rejects_duplicate_nodes() {
        let result = analyze_graph(
            vec![1_u64, 2, 1],
            Vec::<(u64, u64)>::new(),
        );

        assert!(matches!(
            result,
            Err(OptimizationGraphError::DuplicateNode {
                node: 1
            })
        ));
    }

    #[test]
    fn rejects_missing_source_endpoint() {
        let result = analyze_graph(
            vec![1_u64, 2],
            vec![(3_u64, 2_u64)],
        );

        assert!(matches!(
            result,
            Err(OptimizationGraphError::MissingEndpoint {
                endpoint: 3,
                edge_index: 0
            })
        ));
    }

    #[test]
    fn rejects_missing_destination_endpoint() {
        let result = analyze_graph(
            vec![1_u64, 2],
            vec![(1_u64, 3_u64)],
        );

        assert!(matches!(
            result,
            Err(OptimizationGraphError::MissingEndpoint {
                endpoint: 3,
                edge_index: 0
            })
        ));
    }

    #[test]
    fn empty_graph_has_complete_topological_order() {
        let result = analyze(
            &[],
            &[],
        );

        assert!(result.has_complete_topological_order());
    }

    #[test]
    fn cyclic_graph_does_not_claim_complete_order() {
        let result = analyze(
            &[1, 2],
            &[(1, 2), (2, 1)],
        );

        assert!(!result.has_complete_topological_order());
    }

    #[test]
    fn handles_disconnected_acyclic_components() {
        let result = analyze(
            &[1, 2, 3, 4, 5],
            &[
                (1, 2),
                (3, 4),
            ],
        );

        assert!(result.is_acyclic());

        assert_eq!(
            result.topological_order(),
            Some(&[1, 2, 3, 4, 5][..])
        );

        assert_eq!(result.roots(), &[1, 3, 5]);
        assert_eq!(result.sinks(), &[2, 4, 5]);
    }

    #[test]
    fn accepts_non_numeric_identifiers() {
        let result = analyze_graph(
            vec![
                String::from("input"),
                String::from("hidden"),
                String::from("output"),
            ],
            vec![
                (
                    String::from("input"),
                    String::from("hidden"),
                ),
                (
                    String::from("hidden"),
                    String::from("output"),
                ),
            ],
        )
        .unwrap();

        assert!(result.is_acyclic());

        assert_eq!(
            result.topological_order(),
            Some(
                &[
                    String::from("hidden"),
                    String::from("input"),
                    String::from("output"),
                ][..]
            )
        );
    }
}
