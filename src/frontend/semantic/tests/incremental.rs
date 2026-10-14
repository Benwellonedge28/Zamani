#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Incremental Semantic Analysis — Integration Tests
//!
//! Path: src/frontend/semantic/tests/incremental.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! PURPOSE
//!
//! Verify the integrated contracts of:
//! - DependencyGraph;
//! - InvalidationPlan;
//! - SemanticCache;
//! - invalidate_changed.
//!
//! These tests exercise invalidation, cache validity, recomputation lifecycle,
//! deterministic ordering, graph integrity, and large representable AST IDs.
//!
//! OWNERSHIP
//!
//! The production implementations remain in:
//! - semantic/incremental/dependencies.rs
//! - semantic/incremental/invalidation.rs
//! - semantic/incremental/cache.rs
//! - semantic/incremental/mod.rs
//!
//! This file owns integration assertions only. It does not implement another
//! dependency graph, cache, semantic analyzer, AST, or IR.
//!
//! GRAMMAR AND DOMAIN BOUNDARY
//!
//! The canonical grammar is grammar/Zamani.g4, composed with the canonical
//! lexer and parser grammars. The semantic model specification defines
//! source-language meaning. These tests start at the semantic dependency
//! boundary and therefore do not parse source text or duplicate grammar rules.
//!
//! Classical, quantum, HDL, hybrid, tensor, distributed, and future-domain
//! facts use the same dependency and invalidation contracts. Domain-specific
//! meaning remains outside this infrastructure.
//!
//! SCALABILITY
//!
//! Tests must not encode hardware capacity limits as language limits.
//! NodeId values identify AST nodes; they do not represent hardware capacity.
//! The suite checks large representable identities and graph growth without
//! claiming physically infinite memory, address space, or execution time.
//!
//! DETERMINISM
//!
//! Tests compare structured results and ordered IDs, not Debug-formatted
//! strings. They do not sort observed results before comparison, because doing
//! so would conceal nondeterminism in the implementation.
//!
//! INTEGRATION CONTRACT
//!
//! The parent semantic module must register this test module, and the
//! frontend module hierarchy must expose semantic::incremental.
//!
//! FREEZE CRITERIA
//!
//! This suite passes together with the production module tests, workspace
//! tests, formatting, and lint checks. A passing test suite alone does not
//! prove that the entire semantic subsystem is production-ready.

use crate::frontend::ast::node::node_id::NodeId;
use crate::frontend::semantic::incremental::{
    invalidate_changed, CacheError, CacheState, DependencyGraph,
    DependencyGraphError, InvalidationError, InvalidationPlan,
    IncrementalError, SemanticCache,
};

use std::collections::BTreeSet;

// -----------------------------------------------------------------------------
// Shared test helpers
// -----------------------------------------------------------------------------

/// Construct a valid AST node ID for a test.
///
/// Zero is deliberately not accepted by NodeId.
fn node(value: u64) -> NodeId {
    NodeId::new(value).expect("test node ID must be nonzero")
}

/// Construct a dependency chain:
///
///     2 -> 1
///     3 -> 2
///     4 -> 3
///
/// An edge `dependent -> prerequisite` means that the dependent semantic fact
/// reads or otherwise depends on its prerequisite.
fn chain_graph() -> DependencyGraph {
    let mut graph = DependencyGraph::new();

    graph
        .add_dependency(node(2), node(1))
        .expect("first edge should be valid");

    graph
        .add_dependency(node(3), node(2))
        .expect("second edge should be valid");

    graph
        .add_dependency(node(4), node(3))
        .expect("third edge should be valid");

    graph
}

/// Seed each supplied node with a value that identifies the node.
///
/// The values stand in for semantic facts. Their contents have no hardware-
/// specific meaning and are intentionally independent of the AST grammar.
fn seeded_cache(nodes: impl IntoIterator<Item = NodeId>) -> SemanticCache<String> {
    let mut cache = SemanticCache::new();

    for id in nodes {
        cache
            .insert_valid(id, format!("semantic-fact-{id}"))
            .expect("seeding a cache entry should succeed");
    }

    cache
}

/// Collect node IDs without changing their observed ordering.
fn ordered_nodes(
    nodes: impl IntoIterator<Item = NodeId>,
) -> Vec<NodeId> {
    nodes.into_iter().collect()
}

// -----------------------------------------------------------------------------
// Public API and baseline contracts
// -----------------------------------------------------------------------------

#[test]
fn incremental_public_api_is_constructible() {
    let graph = DependencyGraph::new();
    let cache = SemanticCache::<String>::new();

    assert_eq!(graph.node_count(), 0);
    assert_eq!(graph.edge_count(), 0);
    assert!(cache.is_empty());
    assert_eq!(cache.valid_len(), 0);
    assert_eq!(cache.revision(), 0);
}

#[test]
fn empty_invalidation_is_a_no_op() {
    let graph = chain_graph();
    let mut cache = seeded_cache([node(1), node(2)]);

    let revision_before = cache.revision();
    let cache_before = cache.clone();
    let graph_before = graph.clone();

    let (plan, report) = invalidate_changed(
        &graph,
        &mut cache,
        std::iter::empty::<NodeId>(),
    )
    .expect("an empty change set should produce a valid empty plan");

    assert!(plan.is_empty());
    assert_eq!(plan.changed_count(), 0);
    assert_eq!(plan.affected_count(), 0);
    assert!(plan.recompute_order().is_empty());

    assert_eq!(report.affected_entries, 0);
    assert_eq!(report.retained_values, 0);
    assert_eq!(report.revision, revision_before);

    assert_eq!(cache.revision(), revision_before);
    assert_eq!(cache.valid_len(), cache_before.valid_len());
    assert_eq!(cache.get(node(1)), cache_before.get(node(1)));
    assert_eq!(cache.get(node(2)), cache_before.get(node(2)));
    assert_eq!(graph, graph_before);
}

#[test]
fn one_changed_prerequisite_invalidates_all_transitive_dependents() {
    let graph = chain_graph();
    let mut cache = seeded_cache([node(1), node(2), node(3), node(4)]);

    let (plan, report) = invalidate_changed(
        &graph,
        &mut cache,
        [node(1)],
    )
    .expect("the transitive invalidation plan should succeed");

    assert_eq!(
        plan.recompute_order(),
        &[node(1), node(2), node(3), node(4)]
    );

    assert_eq!(plan.changed_count(), 1);
    assert_eq!(plan.affected_count(), 4);
    assert!(plan.was_changed(node(1)));
    assert!(!plan.was_changed(node(4)));

    assert_eq!(report.affected_entries, 4);
    assert_eq!(report.retained_values, 4);

    for id in [node(1), node(2), node(3), node(4)] {
        assert!(plan.affects(id));
        assert_eq!(cache.get(id), None);
        assert_eq!(cache.state(id), Some(CacheState::Invalidated));
        assert_eq!(
            cache.get_stale(id).map(String::as_str),
            Some(format!("semantic-fact-{id}").as_str())
        );
    }
}

#[test]
fn changing_a_dependent_does_not_invalidate_its_prerequisites() {
    let graph = chain_graph();
    let mut cache = seeded_cache([node(1), node(2), node(3), node(4)]);

    let (plan, _) = invalidate_changed(
        &graph,
        &mut cache,
        [node(4)],
    )
    .expect("the invalidation plan should succeed");

    assert_eq!(plan.recompute_order(), &[node(4)]);

    assert_eq!(cache.state(node(1)), Some(CacheState::Valid));
    assert_eq!(cache.state(node(2)), Some(CacheState::Valid));
    assert_eq!(cache.state(node(3)), Some(CacheState::Valid));
    assert_eq!(cache.state(node(4)), Some(CacheState::Invalidated));

    assert_eq!(
        cache.get(node(1)).map(String::as_str),
        Some("semantic-fact-1")
    );
}

#[test]
fn duplicate_changed_nodes_are_deduplicated() {
    let graph = chain_graph();
    let mut cache = seeded_cache([node(1), node(2), node(3), node(4)]);

    let (plan, report) = invalidate_changed(
        &graph,
        &mut cache,
        [node(1), node(1), node(2), node(1), node(2)],
    )
    .expect("duplicate change notifications should be harmless");

    assert_eq!(
        plan.changed_nodes(),
        &BTreeSet::from([node(1), node(2)])
    );

    assert_eq!(
        plan.recompute_order(),
        &[node(1), node(2), node(3), node(4)]
    );

    assert_eq!(plan.changed_count(), 2);
    assert_eq!(plan.affected_count(), 4);
    assert_eq!(report.affected_entries, 4);
    assert_eq!(report.retained_values, 4);
}

#[test]
fn multiple_independent_changes_produce_a_complete_order() {
    let mut graph = DependencyGraph::new();

    graph
        .add_dependency(node(3), node(1))
        .expect("edge should be valid");

    graph
        .add_dependency(node(4), node(2))
        .expect("edge should be valid");

    let mut cache =
        seeded_cache([node(1), node(2), node(3), node(4), node(5)]);

    let (plan, report) = invalidate_changed(
        &graph,
        &mut cache,
        [node(2), node(1)],
    )
    .expect("multiple changed roots should be supported");

    assert_eq!(
        plan.recompute_order(),
        &[node(1), node(2), node(3), node(4)]
    );

    assert_eq!(plan.affected_count(), 4);
    assert_eq!(report.affected_entries, 4);
    assert_eq!(report.retained_values, 4);

    assert_eq!(cache.state(node(5)), Some(CacheState::Valid));
    assert_eq!(cache.get(node(5)).map(String::as_str), Some("semantic-fact-5"));
}

#[test]
fn an_untracked_changed_node_is_still_included_in_the_plan() {
    let graph = DependencyGraph::new();
    let mut cache = SemanticCache::<String>::new();

    let (plan, report) = invalidate_changed(
        &graph,
        &mut cache,
        [node(99)],
    )
    .expect("an untracked changed node should be representable");

    assert_eq!(plan.changed_count(), 1);
    assert_eq!(plan.affected_count(), 1);
    assert_eq!(plan.recompute_order(), &[node(99)]);

    assert_eq!(report.affected_entries, 1);
    assert_eq!(report.retained_values, 0);
    assert_eq!(cache.state(node(99)), Some(CacheState::Invalidated));
    assert_eq!(cache.get(node(99)), None);
}

// -----------------------------------------------------------------------------
// Dependency graph invariants
// -----------------------------------------------------------------------------

#[test]
fn dependency_graph_is_unchanged_by_invalidation_planning_and_application() {
    let graph = chain_graph();
    let before = graph.clone();
    let mut cache = seeded_cache([node(1), node(2), node(3), node(4)]);

    invalidate_changed(&graph, &mut cache, [node(2)])
        .expect("invalidation should succeed");

    assert_eq!(graph, before);
    graph
        .validate()
        .expect("the graph should remain structurally valid");
}

#[test]
fn duplicate_dependency_edges_are_idempotent() {
    let mut graph = DependencyGraph::new();

    assert_eq!(
        graph
            .add_dependency(node(2), node(1))
            .expect("first edge should succeed"),
        true
    );

    let before = graph.clone();

    assert_eq!(
        graph
            .add_dependency(node(2), node(1))
            .expect("repeated edge should be accepted"),
        false
    );

    assert_eq!(graph, before);
    assert_eq!(graph.edge_count(), 1);

    graph
        .validate()
        .expect("the graph should remain valid");
}

#[test]
fn dependency_cycles_are_rejected_without_mutating_the_graph() {
    let mut graph = DependencyGraph::new();

    graph
        .add_dependency(node(2), node(1))
        .expect("first edge should be valid");

    let before = graph.clone();

    let error = graph
        .add_dependency(node(1), node(2))
        .expect_err("the second edge would create a cycle");

    assert!(matches!(error, DependencyGraphError::Cycle { .. }));
    assert_eq!(graph, before);

    graph
        .validate()
        .expect("rejecting a cycle should preserve graph validity");
}

#[test]
fn replacing_dependencies_is_atomic_when_a_cycle_would_be_created() {
    let mut graph = DependencyGraph::new();

    graph
        .add_dependency(node(2), node(1))
        .expect("first edge should be valid");

    graph
        .add_dependency(node(3), node(2))
        .expect("second edge should be valid");

    let before = graph.clone();

    let error = graph
        .replace_dependencies(node(1), [node(3)])
        .expect_err("replacement would introduce a cycle");

    assert!(matches!(error, DependencyGraphError::Cycle { .. }));
    assert_eq!(graph, before);

    graph
        .validate()
        .expect("failed replacement must preserve graph invariants");
}

#[test]
fn graph_topological_order_places_prerequisites_before_dependents() {
    let graph = chain_graph();

    let order = graph
        .topological_order()
        .expect("the acyclic graph should have a topological order");

    let positions: std::collections::BTreeMap<NodeId, usize> = order
        .iter()
        .copied()
        .enumerate()
        .map(|(position, id)| (id, position))
        .collect();

    for (dependent, prerequisite) in [
        (node(2), node(1)),
        (node(3), node(2)),
        (node(4), node(3)),
    ] {
        assert!(
            positions[&prerequisite] < positions[&dependent],
            "prerequisite {prerequisite} must precede dependent {dependent}"
        );
    }
}

// -----------------------------------------------------------------------------
// Determinism
// -----------------------------------------------------------------------------

#[test]
fn equivalent_change_sets_produce_identical_plans() {
    let graph = chain_graph();

    let first = InvalidationPlan::build(
        &graph,
        [node(3), node(1), node(2), node(1)],
    )
    .expect("first plan should succeed");

    let second = InvalidationPlan::build(
        &graph,
        [node(2), node(1), node(3)],
    )
    .expect("second plan should succeed");

    assert_eq!(first, second);
    assert_eq!(
        first.recompute_order(),
        &[node(1), node(2), node(3), node(4)]
    );
}

#[test]
fn repeated_independent_runs_produce_identical_invalidation_results() {
    fn run_once() -> (InvalidationPlan, Vec<Option<CacheState>>, Vec<NodeId>) {
        let graph = chain_graph();
        let mut cache =
            seeded_cache([node(1), node(2), node(3), node(4)]);

        let (plan, _) = invalidate_changed(
            &graph,
            &mut cache,
            [node(1), node(3)],
        )
        .expect("invalidation should succeed");

        let states = [node(1), node(2), node(3), node(4)]
            .into_iter()
            .map(|id| cache.state(id))
            .collect();

        let nodes = ordered_nodes(cache.nodes());

        (plan, states, nodes)
    }

    let first = run_once();
    let second = run_once();

    assert_eq!(first, second);
}

#[test]
fn cache_node_iteration_is_deterministic() {
    let mut cache = SemanticCache::new();

    for id in [node(9), node(2), node(7), node(1), node(5)] {
        cache
            .insert_valid(id, id.get())
            .expect("cache insertion should succeed");
    }

    assert_eq!(
        ordered_nodes(cache.nodes()),
        vec![node(1), node(2), node(5), node(7), node(9)]
    );
}

// -----------------------------------------------------------------------------
// Cache validity and recomputation lifecycle
// -----------------------------------------------------------------------------

#[test]
fn invalidated_values_are_not_visible_as_current_semantic_facts() {
    let graph = chain_graph();
    let mut cache = seeded_cache([node(1), node(2), node(3), node(4)]);

    invalidate_changed(&graph, &mut cache, [node(2)])
        .expect("invalidation should succeed");

    for id in [node(2), node(3), node(4)] {
        assert_eq!(cache.get(id), None);
        assert!(
            cache.get_stale(id).is_some(),
            "the old value may be retained only for explicit stale inspection"
        );
    }

    assert_eq!(cache.get(node(1)).map(String::as_str), Some("semantic-fact-1"));
}

#[test]
fn recomputation_publishes_a_new_valid_value() {
    let graph = chain_graph();
    let mut cache = seeded_cache([node(1), node(2), node(3), node(4)]);

    invalidate_changed(&graph, &mut cache, [node(2)])
        .expect("invalidation should succeed");

    cache
        .begin_recompute(node(2))
        .expect("invalidated entries may be recomputed");

    assert_eq!(cache.state(node(2)), Some(CacheState::Computing));
    assert_eq!(cache.get(node(2)), None);

    cache
        .publish(node(2), String::from("recomputed-fact"))
        .expect("a computing entry may publish its result");

    assert_eq!(cache.state(node(2)), Some(CacheState::Valid));
    assert_eq!(
        cache.get(node(2)).map(String::as_str),
        Some("recomputed-fact")
    );
}

#[test]
fn failed_recomputation_never_exposes_a_stale_value_as_valid() {
    let graph = chain_graph();
    let mut cache = seeded_cache([node(1), node(2), node(3), node(4)]);

    invalidate_changed(&graph, &mut cache, [node(2)])
        .expect("invalidation should succeed");

    cache
        .begin_recompute(node(2))
        .expect("recomputation should start");

    cache
        .mark_failed(node(2))
        .expect("a computing entry may be marked failed");

    assert_eq!(cache.state(node(2)), Some(CacheState::Failed));
    assert_eq!(cache.get(node(2)), None);
    assert_eq!(
        cache.get_stale(node(2)).map(String::as_str),
        Some("semantic-fact-2")
    );
}

#[test]
fn a_failed_recomputation_can_be_retried() {
    let mut cache = SemanticCache::new();

    cache
        .begin_recompute(node(1))
        .expect("a missing entry may begin computation");

    cache
        .mark_failed(node(1))
        .expect("the active computation may fail");

    cache
        .begin_recompute(node(1))
        .expect("a failed computation may be retried");

    cache
        .publish(node(1), String::from("recovered"))
        .expect("the retry should publish a valid result");

    assert_eq!(cache.state(node(1)), Some(CacheState::Valid));
    assert_eq!(cache.get(node(1)).map(String::as_str), Some("recovered"));
}

#[test]
fn invalid_cache_state_transitions_are_rejected() {
    let mut cache = SemanticCache::new();

    cache
        .insert_valid(node(1), String::from("valid"))
        .expect("seeding should succeed");

    let begin_error = cache
        .begin_recompute(node(1))
        .expect_err("a valid entry must be invalidated before recomputation");

    assert!(matches!(
        begin_error,
        CacheError::InvalidTransition { .. }
    ));

    let publish_error = cache
        .publish(node(99), String::from("invalid"))
        .expect_err("a missing, non-computing entry cannot be published");

    assert!(matches!(
        publish_error,
        CacheError::InvalidTransition { .. }
    ));

    let failed_error = cache
        .mark_failed(node(99))
        .expect_err("a missing, non-computing entry cannot be marked failed");

    assert!(matches!(
        failed_error,
        CacheError::InvalidTransition { .. }
    ));
}

#[test]
fn invalidation_of_an_uncached_dependent_creates_an_invalid_entry() {
    let graph = chain_graph();
    let mut cache = SemanticCache::new();

    cache
        .insert_valid(node(1), String::from("prerequisite"))
        .expect("seeding should succeed");

    let (_, report) = invalidate_changed(
        &graph,
        &mut cache,
        [node(1)],
    )
    .expect("invalidation should succeed");

    assert_eq!(report.affected_entries, 4);
    assert_eq!(report.retained_values, 1);

    assert_eq!(cache.state(node(1)), Some(CacheState::Invalidated));
    assert_eq!(cache.state(node(2)), Some(CacheState::Invalidated));
    assert_eq!(cache.state(node(3)), Some(CacheState::Invalidated));
    assert_eq!(cache.state(node(4)), Some(CacheState::Invalidated));

    assert_eq!(cache.get(node(2)), None);
    assert_eq!(cache.get_stale(node(2)), None);
}

#[test]
fn removing_a_missing_cache_entry_is_a_no_op() {
    let mut cache = SemanticCache::<String>::new();
    let revision_before = cache.revision();

    let removed = cache
        .remove(node(1))
        .expect("removing a missing entry should succeed");

    assert_eq!(removed, None);
    assert_eq!(cache.revision(), revision_before);
    assert!(cache.is_empty());
}

#[test]
fn clearing_an_empty_cache_is_a_no_op() {
    let mut cache = SemanticCache::<String>::new();
    let revision_before = cache.revision();

    cache
        .clear()
        .expect("clearing an empty cache should succeed");

    assert_eq!(cache.revision(), revision_before);
    assert!(cache.is_empty());
}

#[test]
fn clearing_a_nonempty_cache_removes_all_tracked_states() {
    let mut cache = seeded_cache([node(1), node(2), node(3)]);
    let revision_before = cache.revision();

    cache
        .clear()
        .expect("clearing a populated cache should succeed");

    assert!(cache.is_empty());
    assert_eq!(cache.valid_len(), 0);
    assert_eq!(cache.state(node(1)), None);
    assert_eq!(cache.state(node(2)), None);
    assert_eq!(cache.state(node(3)), None);
    assert!(cache.revision() > revision_before);
}

// -----------------------------------------------------------------------------
// Large identities and resource-aware growth
// -----------------------------------------------------------------------------

#[test]
fn large_representable_node_ids_are_not_treated_as_hardware_limits() {
    let large_id = node(u64::MAX);

    let mut graph = DependencyGraph::new();
    graph.track_node(large_id);

    let mut cache = SemanticCache::new();
    cache
        .insert_valid(large_id, String::from("large-id-fact"))
        .expect("a representable node ID should be cacheable");

    let (plan, report) = invalidate_changed(
        &graph,
        &mut cache,
        [large_id],
    )
    .expect("large IDs should participate in ordinary invalidation");

    assert_eq!(plan.changed_count(), 1);
    assert_eq!(plan.affected_count(), 1);
    assert_eq!(plan.recompute_order(), &[large_id]);
    assert_eq!(report.affected_entries, 1);
    assert_eq!(cache.state(large_id), Some(CacheState::Invalidated));
    assert_eq!(cache.get(large_id), None);
}

#[test]
fn incremental_infrastructure_handles_a_growing_dependency_chain() {
    // This is a bounded stress test, not a language-level maximum.
    // The size is intentionally a local test workload and must not be
    // reused as a production semantic limit.
    let count: u64 = 256;
    let mut graph = DependencyGraph::new();

    for current in 2..=count {
        graph
            .add_dependency(node(current), node(current - 1))
            .expect("each chain edge should be valid");
    }

    let mut cache = seeded_cache((1..=count).map(node));

    let (plan, report) = invalidate_changed(
        &graph,
        &mut cache,
        [node(1)],
    )
    .expect("the full chain should be invalidated");

    assert_eq!(plan.affected_count(), count as usize);
    assert_eq!(plan.recompute_order().len(), count as usize);
    assert_eq!(report.affected_entries, count as usize);
    assert_eq!(report.retained_values, count as usize);

    assert_eq!(plan.recompute_order().first(), Some(&node(1)));
    assert_eq!(plan.recompute_order().last(), Some(&node(count)));

    for id in 1..=count {
        assert_eq!(cache.get(node(id)), None);
        assert_eq!(
            cache.state(node(id)),
            Some(CacheState::Invalidated)
        );
    }
}

#[test]
fn topological_order_for_a_subset_is_complete_and_deterministic() {
    let graph = chain_graph();
    let subset = BTreeSet::from([node(2), node(3), node(4)]);

    let first = graph
        .topological_order_for(&subset)
        .expect("subset ordering should succeed");

    let second = graph
        .topological_order_for(&subset)
        .expect("repeated subset ordering should succeed");

    assert_eq!(first, second);
    assert_eq!(first.len(), subset.len());
    assert_eq!(first.iter().copied().collect::<BTreeSet<_>>(), subset);

    // Node 1 is outside the subset and is treated as an already-satisfied
    // prerequisite. The order among the selected nodes remains valid.
    assert_eq!(first, vec![node(2), node(3), node(4)]);
}

// -----------------------------------------------------------------------------
// Error propagation and contract exhaustiveness
// -----------------------------------------------------------------------------

#[test]
fn incremental_error_preserves_its_underlying_error_category() {
    let cache_error = IncrementalError::from(CacheError::RevisionExhausted);

    assert!(matches!(cache_error, IncrementalError::Cache(_)));

    let dependency_error = DependencyGraphError::InvalidGraph {
        nodes: vec![node(1)],
        reason: "test-only invalid graph",
    };

    let invalidation_error = InvalidationError::from(dependency_error);
    let incremental_error = IncrementalError::from(invalidation_error);

    assert!(matches!(
        incremental_error,
        IncrementalError::Invalidation(_)
    ));
}

#[test]
fn cache_revisions_advance_on_successful_mutations() {
    let mut cache = SemanticCache::new();

    let initial = cache.revision();

    cache
        .insert_valid(node(1), String::from("first"))
        .expect("insertion should succeed");

    let after_insert = cache.revision();
    assert!(after_insert > initial);

    cache
        .begin_recompute(node(1))
        .expect_err("a valid entry cannot be recomputed without invalidation");

    // A rejected transition must not advance the revision.
    assert_eq!(cache.revision(), after_insert);

    cache
        .remove(node(1))
        .expect("removal should succeed");

    assert!(cache.revision() > after_insert);
}

#[test]
fn invalidation_does_not_make_unaffected_cache_entries_stale() {
    let graph = chain_graph();
    let mut cache = seeded_cache([
        node(1),
        node(2),
        node(3),
        node(4),
        node(100),
    ]);

    let (_, report) = invalidate_changed(
        &graph,
        &mut cache,
        [node(3)],
    )
    .expect("invalidation should succeed");

    assert_eq!(report.affected_entries, 2);
    assert_eq!(report.retained_values, 2);

    assert_eq!(cache.state(node(1)), Some(CacheState::Valid));
    assert_eq!(cache.state(node(2)), Some(CacheState::Valid));
    assert_eq!(cache.state(node(3)), Some(CacheState::Invalidated));
    assert_eq!(cache.state(node(4)), Some(CacheState::Invalidated));
    assert_eq!(cache.state(node(100)), Some(CacheState::Valid));

    assert_eq!(cache.get(node(1)).map(String::as_str), Some("semantic-fact-1"));
    assert_eq!(cache.get(node(2)).map(String::as_str), Some("semantic-fact-2"));
    assert_eq!(cache.get(node(100)).map(String::as_str), Some("semantic-fact-100"));
}