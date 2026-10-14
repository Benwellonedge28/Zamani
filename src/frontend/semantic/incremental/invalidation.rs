
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Incremental Semantic Invalidation
//!
//! Path: `src/frontend/semantic/incremental/invalidation.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! ## PURPOSE
//!
//! Compute which semantic facts must be invalidated after AST nodes or
//! semantic inputs change, and produce a deterministic recomputation order.
//!
//! This module coordinates invalidation planning. It does not execute
//! semantic passes, mutate semantic facts, modify the AST, or perform
//! target-specific resource planning.
//!
//! ## STATUS
//!
//! Implementation candidate. Integration is complete only when this file
//! is declared by `incremental/mod.rs`, the semantic module hierarchy
//! exposes that module, and the acceptance tests pass in the repository.
//!
//! ## AUTHORITY
//!
//! - The canonical AST owns `NodeId`.
//! - `dependencies.rs` owns semantic dependency relationships.
//! - The semantic model owns semantic facts and their validity.
//! - The semantic session or incremental coordinator owns recomputation,
//!   cache mutation, and publication of newly validated results.
//! - The canonical grammar and language specifications own source meaning.
//! - Downstream IR and target-planning components own their own artifacts
//!   and target-specific invalidation.
//!
//! ## DEPENDENCY CONVENTION
//!
//! `dependent -> prerequisite` means that a semantic fact depends on
//! another fact. A change to a prerequisite invalidates the prerequisite
//! and every fact that transitively depends on it.
//!
//! Recalculation must process prerequisites before their dependents.
//!
//! ## TRANSACTIONAL PLANNING
//!
//! Planning is read-only. It does not remove graph edges, change node
//! membership, mutate cached facts, or mark model entries valid or invalid.
//! A caller must apply the plan to its own cache and semantic model.
//!
//! A plan is returned only after the entire affected set has a valid
//! topological order. Dependency-graph errors are propagated unchanged
//! through the public error wrapper.
//!
//! ## SCALABILITY
//!
//! Sparse ordered collections provide deterministic behavior without
//! imposing a language-level limit on the number of AST nodes or semantic
//! facts. Traversal is iterative through `DependencyGraph`.
//!
//! Actual work is necessarily bounded by available memory, address space,
//! execution time, and the representational limits of the host and Rust
//! collection types. This module does not promise physically infinite
//! computation.
//!
//! Configurable operational budgets belong to the semantic session or
//! incremental coordinator. Budget exhaustion must be reported as an
//! incomplete operation, never as successful complete invalidation.
//!
//! ## DETERMINISM
//!
//! Changed nodes, affected nodes, and graph scheduling use the canonical
//! `NodeId` ordering and the deterministic ordering implemented by
//! `DependencyGraph`. Duplicate change notifications do not duplicate
//! nodes in the resulting plan.
//!
//! ## INTEGRATION CONTRACT
//!
//! - `incremental/mod.rs` declares `dependencies` and `invalidation`.
//! - The coordinator records dependencies before requesting a plan.
//! - The coordinator builds a plan before destructive graph or cache edits.
//! - Invalidated facts are recomputed in `recompute_order()` order.
//! - Deleted nodes are handled by planning before graph removal, so their
//!   dependents remain discoverable.
//! - A failed plan must not be treated as a successful partial plan.
//! - This module does not inspect grammar productions, AST variants,
//!   semantic type rules, hardware capacities, or IR instructions.
//!
//! ## FREEZE CRITERIA
//!
//! - Public API and module exports are integrated.
//! - Empty, single-root, multi-root, duplicate-root, untracked-root,
//!   transitive-dependency, and deterministic-order tests pass.
//! - Graph errors are propagated rather than suppressed.
//! - Planning leaves the dependency graph unchanged.
//! - The complete workspace builds and relevant lint and test suites pass.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use crate::frontend::ast::node::node_id::NodeId;

use super::dependencies::{DependencyGraph, DependencyGraphError};

/// A complete, deterministic plan for invalidating and recomputing
/// semantic facts.
///
/// The plan is an immutable description of work. Creating it does not
/// invalidate facts or execute any semantic analysis.
///
/// `changed` contains the explicitly reported changed nodes.
/// `affected` contains those nodes and all transitively dependent nodes.
/// `recompute_order` contains every affected node exactly once, with
/// prerequisites preceding dependents.
///
/// Nodes are retained in the plan even if they have not been registered
/// in the graph. This is intentional: newly created or previously
/// untracked nodes can still be reported as changed.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InvalidationPlan {
    changed: BTreeSet<NodeId>,
    affected: BTreeSet<NodeId>,
    recompute_order: Vec<NodeId>,
}

impl InvalidationPlan {
    /// Construct a complete invalidation plan for one or more changed nodes.
    ///
    /// The operation:
    ///
    /// 1. Deduplicates the changed-node input.
    /// 2. Computes the transitive invalidation closure of every changed node.
    /// 3. Computes a deterministic topological order for the complete closure.
    /// 4. Returns the plan only if ordering succeeds.
    ///
    /// The dependency graph is never modified.
    ///
    /// An empty change set produces an empty plan.
    ///
    /// # Errors
    ///
    /// Returns [`InvalidationError::DependencyGraph`] if the graph cannot
    /// provide a valid complete topological order for the affected set.
    pub fn build<I>(
        graph: &DependencyGraph,
        changed_nodes: I,
    ) -> Result<Self, InvalidationError>
    where
        I: IntoIterator<Item = NodeId>,
    {
        let changed: BTreeSet<NodeId> = changed_nodes.into_iter().collect();

        let mut affected = BTreeSet::new();

        for changed_node in &changed {
            affected.extend(graph.invalidate_from(*changed_node));
        }

        let recompute_order = graph.topological_order_for(&affected)?;

        // Defensive invariant: a successful order must cover the complete
        // affected set, without duplicate nodes.
        //
        // DependencyGraph currently guarantees this contract. Keep the
        // check here so future graph implementations cannot silently return
        // an incomplete plan.
        if recompute_order.len() != affected.len()
            || recompute_order.iter().copied().collect::<BTreeSet<_>>() != affected
        {
            return Err(InvalidationError::IncompleteOrder {
                affected_count: affected.len(),
                ordered_count: recompute_order.len(),
            });
        }

        Ok(Self {
            changed,
            affected,
            recompute_order,
        })
    }

    /// Explicitly changed nodes, in deterministic order.
    pub fn changed_nodes(&self) -> &BTreeSet<NodeId> {
        &self.changed
    }

    /// All nodes that must be invalidated, including transitive dependents.
    pub fn affected_nodes(&self) -> &BTreeSet<NodeId> {
        &self.affected
    }

    /// Nodes in deterministic prerequisite-first recomputation order.
    ///
    /// Every affected node appears exactly once.
    pub fn recompute_order(&self) -> &[NodeId] {
        &self.recompute_order
    }

    /// Number of distinct explicitly changed nodes.
    pub fn changed_count(&self) -> usize {
        self.changed.len()
    }

    /// Number of affected nodes.
    pub fn affected_count(&self) -> usize {
        self.affected.len()
    }

    /// Whether no nodes need invalidation or recomputation.
    pub fn is_empty(&self) -> bool {
        self.affected.is_empty()
    }

    /// Whether a node belongs to the invalidation closure.
    pub fn affects(&self, node: NodeId) -> bool {
        self.affected.contains(&node)
    }

    /// Whether a node was explicitly supplied as changed.
    pub fn was_changed(&self, node: NodeId) -> bool {
        self.changed.contains(&node)
    }
}

/// Errors that prevent creation of a complete invalidation plan.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum InvalidationError {
    /// The dependency graph could not provide a complete valid order.
    DependencyGraph(DependencyGraphError),

    /// Defensive check: the ordering did not contain exactly the affected
    /// nodes. A partial order must never be presented as a complete plan.
    IncompleteOrder {
        /// Number of distinct affected nodes.
        affected_count: usize,

        /// Number of nodes returned by the ordering operation.
        ordered_count: usize,
    },
}

impl fmt::Display for InvalidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DependencyGraph(error) => {
                write!(formatter, "cannot plan semantic invalidation: {error}")
            }
            Self::IncompleteOrder {
                affected_count,
                ordered_count,
            } => write!(
                formatter,
                "incomplete invalidation order: expected {affected_count} \
                 distinct nodes, received {ordered_count}"
            ),
        }
    }
}

impl Error for InvalidationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::DependencyGraph(error) => Some(error),
            Self::IncompleteOrder { .. } => None,
        }
    }
}

impl From<DependencyGraphError> for InvalidationError {
    fn from(error: DependencyGraphError) -> Self {
        Self::DependencyGraph(error)
    }
}

/// Build an invalidation plan for a collection of changed nodes.
///
/// This is a convenience entry point for incremental coordinators that
/// prefer a function over `InvalidationPlan::build`.
///
/// The graph is borrowed immutably and is never modified.
pub fn plan_invalidation<I>(
    graph: &DependencyGraph,
    changed_nodes: I,
) -> Result<InvalidationPlan, InvalidationError>
where
    I: IntoIterator<Item = NodeId>,
{
    InvalidationPlan::build(graph, changed_nodes)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn node(value: u64) -> NodeId {
        NodeId::new(value).expect("test node IDs must be nonzero")
    }

    fn graph_with_chain() -> DependencyGraph {
        // 3 depends on 2, and 2 depends on 1.
        let mut graph = DependencyGraph::new();

        graph
            .add_dependency(node(2), node(1))
            .expect("first edge must be valid");
        graph
            .add_dependency(node(3), node(2))
            .expect("second edge must be valid");

        graph
    }

    #[test]
    fn empty_changes_produce_empty_plan() {
        let graph = graph_with_chain();

        let plan =
            InvalidationPlan::build(&graph, []).expect("empty plan must succeed");

        assert!(plan.is_empty());
        assert_eq!(plan.changed_count(), 0);
        assert_eq!(plan.affected_count(), 0);
        assert!(plan.recompute_order().is_empty());
    }

    #[test]
    fn prerequisite_change_invalidates_all_transitive_dependents() {
        let graph = graph_with_chain();

        let plan = plan_invalidation(&graph, [node(1)])
            .expect("valid graph must produce a plan");

        assert_eq!(
            plan.affected_nodes(),
            &BTreeSet::from([node(1), node(2), node(3)])
        );
        assert_eq!(
            plan.recompute_order(),
            &[node(1), node(2), node(3)]
        );
    }

    #[test]
    fn dependent_change_does_not_invalidate_its_prerequisites() {
        let graph = graph_with_chain();

        let plan = plan_invalidation(&graph, [node(3)])
            .expect("valid graph must produce a plan");

        assert_eq!(
            plan.affected_nodes(),
            &BTreeSet::from([node(3)])
        );
        assert_eq!(plan.recompute_order(), &[node(3)]);
    }

    #[test]
    fn multiple_roots_are_deduplicated_and_merged() {
        let graph = graph_with_chain();

        let plan = plan_invalidation(&graph, [node(1), node(2), node(1)])
            .expect("valid graph must produce a plan");

        assert_eq!(
            plan.changed_nodes(),
            &BTreeSet::from([node(1), node(2)])
        );
        assert_eq!(
            plan.affected_nodes(),
            &BTreeSet::from([node(1), node(2), node(3)])
        );
        assert_eq!(plan.changed_count(), 2);
        assert_eq!(plan.affected_count(), 3);
        assert_eq!(
            plan.recompute_order(),
            &[node(1), node(2), node(3)]
        );
    }

    #[test]
    fn an_untracked_changed_node_is_still_included() {
        let graph = graph_with_chain();

        let plan = plan_invalidation(&graph, [node(99)])
            .expect("untracked change must still be representable");

        assert_eq!(
            plan.affected_nodes(),
            &BTreeSet::from([node(99)])
        );
        assert_eq!(plan.recompute_order(), &[node(99)]);
        assert!(plan.was_changed(node(99)));
    }

    #[test]
    fn independent_nodes_have_deterministic_order() {
        let mut graph = DependencyGraph::new();

        graph.track_node(node(30));
        graph.track_node(node(10));
        graph.track_node(node(20));

        let plan = plan_invalidation(&graph, [node(30), node(10), node(20)])
            .expect("independent nodes must be orderable");

        assert_eq!(
            plan.recompute_order(),
            &[node(10), node(20), node(30)]
        );
    }

    #[test]
    fn planning_does_not_mutate_the_dependency_graph() {
        let graph = graph_with_chain();
        let before = graph.clone();

        let _plan = plan_invalidation(&graph, [node(1)])
            .expect("valid graph must produce a plan");

        assert_eq!(graph, before);
    }

    #[test]
    fn plan_queries_report_membership_correctly() {
        let graph = graph_with_chain();

        let plan = plan_invalidation(&graph, [node(1)])
            .expect("valid graph must produce a plan");

        assert!(plan.affects(node(1)));
        assert!(plan.affects(node(2)));
        assert!(plan.affects(node(3)));

        assert!(plan.was_changed(node(1)));
        assert!(!plan.was_changed(node(2)));
        assert!(!plan.was_changed(node(3)));
        assert!(!plan.affects(node(99)));
    }

    #[test]
    fn disconnected_changes_do_not_invalidate_unrelated_nodes() {
        let mut graph = graph_with_chain();
        graph.track_node(node(100));
        graph.track_node(node(200));

        let plan = plan_invalidation(&graph, [node(100)])
            .expect("valid graph must produce a plan");

        assert_eq!(
            plan.affected_nodes(),
            &BTreeSet::from([node(100)])
        );
        assert_eq!(plan.recompute_order(), &[node(100)]);
    }
}
