
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Incremental Semantic Analysis
//!
//! Path: `src/frontend/semantic/incremental/mod.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! ## PURPOSE
//!
//! This module is the public orchestration boundary for incremental semantic
//! analysis. It composes the dependency graph, invalidation planner, and
//! semantic cache into one consistent API.
//!
//! It provides the building blocks needed to:
//!
//! - record semantic dependencies;
//! - calculate transitive invalidation after source or semantic changes;
//! - determine deterministic prerequisite-first recomputation order;
//! - invalidate cached facts without exposing stale values as valid;
//! - report planning and cache lifecycle failures explicitly.
//!
//! The module does not execute semantic passes. The semantic session or
//! pipeline remains responsible for recomputing facts and publishing results.
//!
//! ## STATUS
//!
//! Integration module. Consider this module frozen only after the parent
//! frontend modules expose it and the workspace acceptance checks pass.
//!
//! ## AUTHORITY
//!
//! - `frontend::ast::node::node_id` owns canonical AST node identity.
//! - `dependencies.rs` owns dependency relationships and graph algorithms.
//! - `invalidation.rs` owns invalidation closure and recomputation ordering.
//! - `cache.rs` owns cached values and their validity lifecycle.
//! - The semantic model owns the meaning of cached semantic facts.
//! - The canonical grammar and semantic specifications own source meaning.
//! - Downstream IRs and target planners own their representations and
//!   target-specific decisions.
//!
//! ## OWNERSHIP
//!
//! This module owns:
//!
//! - composition and visibility of incremental submodules;
//! - stable re-exports of the incremental API;
//! - a unified error boundary for orchestration operations;
//! - the operation that plans invalidation before applying it to a cache.
//!
//! This module does not own:
//!
//! - AST construction or mutation;
//! - source grammar or parser behavior;
//! - name resolution, type checking, effects, or domain rules;
//! - the dependency graph's internal representation;
//! - the invalidation algorithm;
//! - cache entry storage or state transitions;
//! - semantic recomputation or pass scheduling;
//! - classical IR, quantum IR, HDL IR, or target-specific planning;
//! - physical hardware discovery, allocation, routing, or execution.
//!
//! ## DEPENDENCY DIRECTION
//!
//! ```text
//! canonical AST / NodeId
//!          |
//!          v
//! incremental/mod.rs
//!          |
//!          +---- dependencies.rs
//!          |
//!          +---- invalidation.rs
//!          |
//!          +---- cache.rs
//!          |
//!          v
//! semantic session / coordinator
//!          |
//!          v
//! semantic pipeline and model
//!          |
//!          v
//! canonical downstream IR boundaries
//! ```
//!
//! The dependency graph and cache use the same canonical `NodeId` type.
//! The invalidation planner consumes the graph and produces a complete plan.
//! This module composes those existing contracts without implementing a
//! competing graph, cache, or invalidation algorithm.
//!
//! ## INVALIDATION CONTRACT
//!
//! `invalidate_changed` performs the following operations in order:
//!
//! 1. Build a complete invalidation plan from the supplied graph.
//! 2. If planning fails, return an error without modifying the cache.
//! 3. Apply the plan to the cache.
//! 4. If the cache operation fails, propagate its error.
//! 5. Return both the plan and the cache invalidation report.
//!
//! The dependency graph is borrowed immutably and is never modified by this
//! operation. The cache implementation checks revision exhaustion before
//! changing entries, so a failed cache invalidation does not partially apply
//! the requested plan.
//!
//! The caller must ensure that the graph does not change between planning
//! and applying the resulting plan. This module does not provide internal
//! synchronization or a graph-version transaction.
//!
//! ## RECOMPUTATION CONTRACT
//!
//! After successful invalidation:
//!
//! - cached facts in the affected set must not be read as valid;
//! - recomputation must follow `InvalidationPlan::recompute_order()`;
//! - each result must be published only after its semantic pass succeeds;
//! - failed recomputations must be marked failed;
//! - the coordinator must update dependency relationships consistently with
//!   the inputs actually consumed by each recomputed fact.
//!
//! A successful invalidation operation is not proof that recomputation has
//! completed or that the program is semantically valid.
//!
//! ## GRAMMAR AND DOMAIN BOUNDARIES
//!
//! This module does not interpret grammar productions. `grammar/Zamani.g4`
//! and the subordinate grammar hierarchy own syntax; the semantic model
//! specification owns language meaning.
//!
//! Classical, quantum, HDL, hybrid, AI/data, distributed, and future domains
//! use the same incremental infrastructure. Domain-specific facts and rules
//! remain owned by their respective semantic components.
//!
//! ## SCALABILITY
//!
//! This module introduces no language-level limit on the number of AST nodes,
//! semantic facts, resource requirements, computational domains, devices, or
//! hardware capacities.
//!
//! Actual operations remain bounded by available memory, execution time,
//! address space, and Rust's representational limits. Any configurable
//! operational budgets belong to the semantic coordinator and must report
//! incomplete work explicitly rather than treating it as successful analysis.
//!
//! ## DETERMINISM
//!
//! Deterministic ordering is provided by `DependencyGraph` and
//! `InvalidationPlan`. This module does not introduce random ordering,
//! global mutable state, clock-dependent behavior, or hardware-dependent
//! semantic decisions.
//!
//! ## ERROR HANDLING
//!
//! Graph-planning failures and cache lifecycle failures are preserved as
//! distinct error variants. Neither is silently converted into success.
//! No partial invalidation plan is returned as a successful result.
//!
//! ## FREEZE CRITERIA
//!
//! - All incremental submodules are declared and exported exactly once.
//! - Public re-exports refer to the canonical implementations.
//! - Planning errors leave the cache untouched.
//! - Cache errors are propagated without being discarded.
//! - Successful orchestration returns both the complete plan and report.
//! - Tests cover empty changes, transitive changes, stale-value hiding,
//!   deterministic order, and graph immutability.
//! - Parent module integration, formatting, tests, and workspace checks pass.

pub mod cache;
pub mod dependencies;
pub mod invalidation;

use std::error::Error;
use std::fmt;

use crate::frontend::ast::node::node_id::NodeId;

// -----------------------------------------------------------------------------
// Public API
// -----------------------------------------------------------------------------

/// Dependency graph and graph-operation errors.
pub use self::dependencies::{DependencyGraph, DependencyGraphError};

/// Cache state, cache errors, and cache invalidation reports.
pub use self::cache::{
    CacheError,
    CacheState,
    InvalidationReport,
    SemanticCache,
};

/// Complete invalidation plans and planning errors.
pub use self::invalidation::{
    plan_invalidation,
    InvalidationError,
    InvalidationPlan,
};

/// Unified errors from incremental orchestration operations.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum IncrementalError {
    /// A complete invalidation plan could not be constructed.
    Invalidation(InvalidationError),

    /// The cache could not apply the complete invalidation plan.
    Cache(CacheError),
}

impl fmt::Display for IncrementalError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Invalidation(error) => {
                write!(formatter, "incremental invalidation planning failed: {error}")
            }
            Self::Cache(error) => {
                write!(formatter, "incremental cache invalidation failed: {error}")
            }
        }
    }
}

impl Error for IncrementalError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Invalidation(error) => Some(error),
            Self::Cache(error) => Some(error),
        }
    }
}

impl From<InvalidationError> for IncrementalError {
    fn from(error: InvalidationError) -> Self {
        Self::Invalidation(error)
    }
}

impl From<CacheError> for IncrementalError {
    fn from(error: CacheError) -> Self {
        Self::Cache(error)
    }
}

// -----------------------------------------------------------------------------
// Orchestration
// -----------------------------------------------------------------------------

/// Plan invalidation for changed AST nodes and apply the complete plan to
/// the semantic cache.
///
/// This function is the convenience entry point for a semantic session that
/// already owns its dependency graph and cache.
///
/// # Behavior
///
/// - Duplicate changed node IDs are deduplicated by the planner.
/// - The affected set includes changed nodes and all transitive dependents.
/// - Recompute order is deterministic and prerequisite-first.
/// - The graph is borrowed immutably and is not modified.
/// - Cached values in the affected set become unavailable through
///   [`SemanticCache::get`] until successfully recomputed and published.
/// - An empty change set is a no-op for the cache.
/// - Planning and cache errors are propagated explicitly.
///
/// # Concurrency contract
///
/// The caller must prevent dependency-graph changes between planning and
/// application of the plan. This function does not lock the graph or cache.
///
/// # Errors
///
/// Returns [`IncrementalError::Invalidation`] if a complete plan cannot be
/// constructed, or [`IncrementalError::Cache`] if the cache cannot apply it.
/// A failed cache application does not partially mutate the cache under the
/// revision-exhaustion contract of [`SemanticCache::apply_invalidation`].
///
/// # Example
///
/// ```ignore
/// let (plan, report) = invalidate_changed(
///     &dependency_graph,
///     &mut semantic_cache,
///     changed_node_ids,
/// )?;
///
/// for node in plan.recompute_order() {
///     // The owning semantic session recomputes this node.
///     // Begin recomputation, then publish only after successful analysis.
///     recompute_semantic_fact(*node)?;
/// }
/// ```
///
/// The example is illustrative: `recompute_semantic_fact` belongs to the
/// caller's semantic pipeline and is not defined by this module.
pub fn invalidate_changed<V, I>(
    graph: &DependencyGraph,
    cache: &mut SemanticCache<V>,
    changed_nodes: I,
) -> Result<(InvalidationPlan, InvalidationReport), IncrementalError>
where
    I: IntoIterator<Item = NodeId>,
{
    // Plan first. No cache state changes if planning fails.
    let plan = InvalidationPlan::build(graph, changed_nodes)?;

    // The cache applies the entire plan or returns an error before mutation.
    let report = cache.apply_invalidation(&plan)?;

    Ok((plan, report))
}

// -----------------------------------------------------------------------------
// Integration tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn node(value: u64) -> NodeId {
        NodeId::new(value).expect("test node ID must be nonzero")
    }

    fn graph_with_chain() -> DependencyGraph {
        // Node 2 depends on node 1; node 3 depends on node 2.
        let mut graph = DependencyGraph::new();

        graph
            .add_dependency(node(2), node(1))
            .expect("first dependency must be valid");

        graph
            .add_dependency(node(3), node(2))
            .expect("second dependency must be valid");

        graph
    }

    #[test]
    fn module_exports_the_shared_incremental_types() {
        let graph = DependencyGraph::new();
        let cache = SemanticCache::<String>::new();

        assert_eq!(graph.node_count(), 0);
        assert!(cache.is_empty());
    }

    #[test]
    fn invalidation_orchestrates_transitive_dependents() {
        let graph = graph_with_chain();

        let mut cache = SemanticCache::new();
        cache
            .insert_valid(node(1), String::from("first"))
            .expect("seed must succeed");
        cache
            .insert_valid(node(2), String::from("second"))
            .expect("seed must succeed");
        cache
            .insert_valid(node(3), String::from("third"))
            .expect("seed must succeed");

        let (plan, report) =
            invalidate_changed(&graph, &mut cache, [node(1)])
                .expect("complete invalidation must succeed");

        assert_eq!(
            plan.recompute_order(),
            &[node(1), node(2), node(3)]
        );
        assert_eq!(report.affected_entries, 3);
        assert_eq!(report.retained_values, 3);

        for id in [node(1), node(2), node(3)] {
            assert_eq!(cache.get(id), None);
            assert_eq!(cache.state(id), Some(CacheState::Invalidated));
        }
    }

    #[test]
    fn orchestration_does_not_mutate_the_dependency_graph() {
        let graph = graph_with_chain();
        let before = graph.clone();

        let mut cache = SemanticCache::<String>::new();

        invalidate_changed(&graph, &mut cache, [node(1)])
            .expect("invalidation must succeed");

        assert_eq!(graph, before);
    }

    #[test]
    fn duplicate_changes_are_deduplicated() {
        let graph = graph_with_chain();
        let mut cache = SemanticCache::<String>::new();

        let (plan, report) =
            invalidate_changed(
                &graph,
                &mut cache,
                [node(1), node(1), node(2)],
            )
            .expect("duplicate changes must be accepted");

        assert_eq!(plan.changed_count(), 2);
        assert_eq!(plan.affected_count(), 3);
        assert_eq!(report.affected_entries, 3);
    }

    #[test]
    fn empty_changes_do_not_mutate_the_cache() {
        let graph = graph_with_chain();
        let mut cache = SemanticCache::new();

        cache
            .insert_valid(node(1), String::from("current"))
            .expect("seed must succeed");

        let revision_before = cache.revision();

        let (plan, report) =
            invalidate_changed(&graph, &mut cache, [])
                .expect("empty change set must succeed");

        assert!(plan.is_empty());
        assert_eq!(report.affected_entries, 0);
        assert_eq!(report.retained_values, 0);
        assert_eq!(cache.revision(), revision_before);
        assert_eq!(
            cache.get(node(1)).map(String::as_str),
            Some("current")
        );
    }

    #[test]
    fn a_new_or_untracked_changed_node_is_still_invalidated() {
        let graph = DependencyGraph::new();
        let mut cache = SemanticCache::<String>::new();

        let (plan, report) =
            invalidate_changed(&graph, &mut cache, [node(500)])
                .expect("untracked changed nodes must be supported");

        assert_eq!(plan.recompute_order(), &[node(500)]);
        assert_eq!(report.affected_entries, 1);
        assert_eq!(
            cache.state(node(500)),
            Some(CacheState::Invalidated)
        );
        assert_eq!(cache.get(node(500)), None);
    }

    #[test]
    fn cached_values_remain_hidden_until_recomputation_succeeds() {
        let graph = DependencyGraph::new();
        let mut cache = SemanticCache::new();

        cache
            .insert_valid(node(1), String::from("old result"))
            .expect("seed must succeed");

        invalidate_changed(&graph, &mut cache, [node(1)])
            .expect("invalidation must succeed");

        assert_eq!(cache.get(node(1)), None);
        assert_eq!(
            cache.get_stale(node(1)).map(String::as_str),
            Some("old result")
        );

        cache
            .begin_recompute(node(1))
            .expect("recomputation must start");

        cache
            .publish(node(1), String::from("new result"))
            .expect("successful result must publish");

        assert_eq!(
            cache.get(node(1)).map(String::as_str),
            Some("new result")
        );
    }
}
