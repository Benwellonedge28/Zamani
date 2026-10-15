#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR Incremental Compilation — Integration Tests
//!
//! Path: src/zuir/tests/incremental.rs
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: unsafe code is prohibited.
//!
//! STATUS
//!
//! Proposed integration-test module. Integrate with the actual ZUIR test
//! hierarchy before treating this file as part of the compiling repository.
//!
//! PURPOSE
//!
//! Exercise the public incremental-compilation contracts across:
//!
//! - dependency graph construction and mutation;
//! - deterministic dependency traversal;
//! - transitive and multi-root invalidation;
//! - cycle-safe invalidation;
//! - stale-plan detection;
//! - stable cache-key derivation;
//! - dependency-order independence;
//! - relevant-input sensitivity;
//! - duplicate dependency rejection;
//! - configurable graph scale;
//! - grammar, frontend, semantic, IR, dialect, and transformation
//!   dependency relationships.
//!
//! AUTHORITY
//!
//! - src/zuir/incremental/dependencies.rs
//! - src/zuir/incremental/invalidation.rs
//! - src/zuir/incremental/cache_keys.rs
//! - src/zuir/fingerprint/stable_hash.rs
//! - src/zuir/version.rs
//! - src/zuir/ZUIR-SPEC.md
//! - grammar/DESIGN.md
//! - grammar/Zamani.g4
//! - grammar/grammar.md
//! - grammar/Zamani-Grammar.md
//! - grammar/specification/semantic-model.md
//! - grammar/specification/scalability-model.md
//!
//! OWNERSHIP
//!
//! This file owns test fixtures, assertions, and regression scenarios only.
//! Production graph algorithms, invalidation semantics, fingerprinting,
//! serialization, and cache storage remain owned by their implementation
//! modules.
//!
//! DOES NOT OWN
//!
//! - A second dependency graph implementation.
//! - A second cache-key algorithm.
//! - Cache persistence or eviction.
//! - Parsing or interpreting the grammar.
//! - Semantic equivalence of arbitrary programs.
//! - Physical target feasibility or execution.
//!
//! INTEGRATION
//!
//! This module consumes the public APIs re-exported from:
//!
//! crate::zuir::incremental
//! crate::zuir::fingerprint::stable_hash
//! crate::zuir::version
//!
//! The parent test module must include this file, and the ZUIR root must
//! expose the production modules. If the repository's test architecture
//! differs, adapt the module declaration rather than duplicating APIs.
//!
//! DETERMINISM CONTRACT
//!
//! Equivalent graph structures and equivalent cache-key inputs must produce
//! equivalent ordered results and identical keys, irrespective of insertion
//! order where the production API promises order independence.
//!
//! SCALABILITY CONTRACT
//!
//! Tests use dynamically generated graphs and collections. Test fixture
//! sizes are workload parameters, not universal ZUIR limits. No finite test
//! proves infinite scalability; resource exhaustion remains possible.
//!
//! POCO-REAF CONTRACT
//!
//! Portable artifacts must not accidentally acquire target-specific cache
//! identity. Target-specific artifacts must include a fingerprint whenever
//! target properties can affect their output.
//!
//! FREEZE CRITERIA
//!
//! - Public imports match the actual implementation.
//! - Every test passes against the real crate.
//! - Grammar dependency tests test dependency registration, not pretend to
//!   parse grammar files.
//! - Cache-key sensitivity tests modify one input dimension at a time.
//! - Stress workloads remain configurable and do not define language limits.
//! - No unsafe code or nondeterministic test assertions are introduced.

use std::collections::BTreeSet;

use crate::zuir::fingerprint::stable_hash::{
    fingerprint,
    Fingerprint,
};
use crate::zuir::incremental::{
    CacheKey,
    CacheKeyError,
    CacheKeyInput,
    DependencyFingerprint,
    DependencyGraph,
    DependencyKey,
    DependencyKind,
    InvalidationError,
    InvalidationPlanner,
};
use crate::zuir::version::{
    ZuirFormatVersion,
    ZuirSchemaVersion,
    CURRENT_ZUIR_FORMAT_VERSION,
    CURRENT_ZUIR_SCHEMA_VERSION,
};

// ---------------------------------------------------------------------------
// Test fixtures
// ---------------------------------------------------------------------------

fn key(value: &str) -> DependencyKey {
    DependencyKey::new(value)
        .unwrap_or_else(|error| panic!("invalid test dependency key {value:?}: {error}"))
}

fn digest(value: &str) -> Fingerprint {
    fingerprint("zuir.incremental.integration-test", value)
        .unwrap_or_else(|error| panic!("failed to fingerprint test input: {error}"))
}

fn add_edge(
    graph: &mut DependencyGraph,
    dependent: &str,
    prerequisite: &str,
    kind: DependencyKind,
) {
    graph
        .add_dependency(key(dependent), key(prerequisite), kind)
        .unwrap_or_else(|error| {
            panic!(
                "failed to add dependency {dependent:?} -> {prerequisite:?}: {error}"
            )
        });
}

fn cache_input<'a>(
    identity: &'a DependencyKey,
    dependencies: &'a [DependencyFingerprint],
) -> CacheKeyInput<'a> {
    CacheKeyInput {
        artifact_kind: "zuir.integration-test",
        artifact_identity: identity,
        compiler_identity: "zamani-incremental-test",
        schema_version: CURRENT_ZUIR_SCHEMA_VERSION,
        format_version: CURRENT_ZUIR_FORMAT_VERSION,
        direct_input: digest("direct-input"),
        configuration_fingerprint: digest("configuration"),
        pass_pipeline_fingerprint: digest("pass-pipeline"),
        semantic_environment_fingerprint: digest("semantic-environment"),
        target_fingerprint: None,
        dependencies,
    }
}

fn derive_key(input: &CacheKeyInput<'_>) -> CacheKey {
    CacheKey::derive(input)
        .unwrap_or_else(|error| panic!("cache-key derivation failed: {error}"))
}

// ---------------------------------------------------------------------------
// Dependency graph contracts
// ---------------------------------------------------------------------------

#[test]
fn empty_graph_has_no_nodes_or_edges() {
    let graph = DependencyGraph::new();

    assert_eq!(graph.node_count(), 0);
    assert_eq!(graph.edge_count(), 0);
    assert_eq!(graph.generation(), 0);
    assert!(graph.validate().is_ok());
}

#[test]
fn registering_the_same_node_is_idempotent() {
    let mut graph = DependencyGraph::new();
    let artifact = key("source:main.zm");

    assert!(graph.add_node(artifact.clone()).expect("first insertion"));
    let generation = graph.generation();

    assert!(!graph.add_node(artifact).expect("duplicate insertion"));
    assert_eq!(graph.generation(), generation);
    assert_eq!(graph.node_count(), 1);
    assert_eq!(graph.edge_count(), 0);
}

#[test]
fn duplicate_dependency_insertion_is_idempotent() {
    let mut graph = DependencyGraph::new();

    assert!(
        graph
            .add_dependency(
                key("zuir:main"),
                key("semantic:main"),
                DependencyKind::Semantic,
            )
            .expect("first edge")
    );

    let generation = graph.generation();

    assert!(
        !graph
            .add_dependency(
                key("zuir:main"),
                key("semantic:main"),
                DependencyKind::Semantic,
            )
            .expect("duplicate edge")
    );

    assert_eq!(graph.generation(), generation);
    assert_eq!(graph.edge_count(), 1);
    assert!(graph.validate().is_ok());
}

#[test]
fn distinct_dependency_reasons_for_the_same_pair_are_preserved() {
    let mut graph = DependencyGraph::new();

    add_edge(
        &mut graph,
        "zuir:main",
        "semantic:main",
        DependencyKind::Semantic,
    );

    add_edge(
        &mut graph,
        "zuir:main",
        "semantic:main",
        DependencyKind::Extension,
    );

    assert_eq!(graph.edge_count(), 2);
    assert!(graph.validate().is_ok());
}

#[test]
fn removing_one_dependency_reason_preserves_other_reasons() {
    let mut graph = DependencyGraph::new();

    add_edge(
        &mut graph,
        "zuir:main",
        "semantic:main",
        DependencyKind::Semantic,
    );

    add_edge(
        &mut graph,
        "zuir:main",
        "semantic:main",
        DependencyKind::Extension,
    );

    assert!(
        graph
            .remove_dependency(
                &key("zuir:main"),
                &key("semantic:main"),
                DependencyKind::Semantic,
            )
            .expect("remove semantic edge")
    );

    assert_eq!(graph.edge_count(), 1);
    assert!(graph.validate().is_ok());

    let prerequisites = graph
        .prerequisites_of(&key("zuir:main"))
        .expect("prerequisite lookup");

    assert_eq!(prerequisites.len(), 1);
    assert_eq!(prerequisites[0].kind(), DependencyKind::Extension);
}

#[test]
fn removing_a_node_does_not_leave_dangling_edges() {
    let mut graph = DependencyGraph::new();

    add_edge(
        &mut graph,
        "ir:main",
        "semantic:main",
        DependencyKind::Ir,
    );

    add_edge(
        &mut graph,
        "analysis:main",
        "ir:main",
        DependencyKind::Analysis,
    );

    assert!(graph.remove_node(&key("ir:main")).expect("remove node"));
    assert!(graph.validate().is_ok());

    assert!(!graph.contains_node(&key("ir:main")));

    // Removing an absent node is an idempotent no-op.
    assert!(!graph.remove_node(&key("ir:main")).expect("remove absent node"));
}

#[test]
fn unknown_artifacts_are_not_mistaken_for_dependency_free_artifacts() {
    let graph = DependencyGraph::new();

    assert!(graph.prerequisites_of(&key("missing:artifact")).is_err());
    assert!(graph.dependents_of(&key("missing:artifact")).is_err());
}

#[test]
fn dependency_traversal_is_deterministic() {
    let mut first = DependencyGraph::new();
    let mut second = DependencyGraph::new();

    let edges = [
        ("ir:main", "semantic:main"),
        ("ir:main", "types:main"),
        ("ir:main", "grammar:main"),
        ("analysis:main", "ir:main"),
    ];

    for (dependent, prerequisite) in edges {
        add_edge(
            &mut first,
            dependent,
            prerequisite,
            DependencyKind::Semantic,
        );
    }

    for (dependent, prerequisite) in edges.into_iter().rev() {
        add_edge(
            &mut second,
            dependent,
            prerequisite,
            DependencyKind::Semantic,
        );
    }

    assert!(first.validate().is_ok());
    assert!(second.validate().is_ok());

    assert_eq!(
        first.prerequisites_of(&key("ir:main")).expect("first traversal"),
        second
            .prerequisites_of(&key("ir:main"))
            .expect("second traversal"),
    );

    assert_eq!(
        first.dependents_of(&key("ir:main")).expect("first dependents"),
        second
            .dependents_of(&key("ir:main"))
            .expect("second dependents"),
    );
}

// ---------------------------------------------------------------------------
// Invalidation contracts
// ---------------------------------------------------------------------------

#[test]
fn invalidation_includes_the_changed_artifact_and_all_transitive_dependents() {
    let mut graph = DependencyGraph::new();

    // dependent -> prerequisite
    add_edge(
        &mut graph,
        "semantic:main",
        "source:main.zm",
        DependencyKind::Source,
    );

    add_edge(
        &mut graph,
        "zuir:main",
        "semantic:main",
        DependencyKind::Semantic,
    );

    add_edge(
        &mut graph,
        "analysis:main",
        "zuir:main",
        DependencyKind::Analysis,
    );

    add_edge(
        &mut graph,
        "codegen:main",
        "analysis:main",
        DependencyKind::Transformation,
    );

    let plan = InvalidationPlanner::new()
        .plan(&graph, &key("source:main.zm"))
        .expect("invalidation plan");

    let expected = [
        "analysis:main",
        "codegen:main",
        "semantic:main",
        "source:main.zm",
        "zuir:main",
    ]
    .into_iter()
    .map(key)
    .collect::<Vec<_>>();

    assert_eq!(plan.invalidated(), expected.as_slice());
    assert_eq!(plan.len(), expected.len());
    assert!(!plan.is_empty());
    assert!(plan.is_current_for(&graph));
}

#[test]
fn multi_root_invalidation_is_sorted_deduplicated_and_deterministic() {
    let mut graph = DependencyGraph::new();

    add_edge(
        &mut graph,
        "semantic:main",
        "source:main.zm",
        DependencyKind::Source,
    );

    add_edge(
        &mut graph,
        "zuir:main",
        "semantic:main",
        DependencyKind::Semantic,
    );

    add_edge(
        &mut graph,
        "analysis:main",
        "zuir:main",
        DependencyKind::Analysis,
    );

    let planner = InvalidationPlanner::new();

    let first = planner
        .plan_many(
            &graph,
            [
                key("source:main.zm"),
                key("semantic:main"),
                key("source:main.zm"),
            ],
        )
        .expect("first multi-root plan");

    let second = planner
        .plan_many(
            &graph,
            [
                key("semantic:main"),
                key("source:main.zm"),
            ],
        )
        .expect("second multi-root plan");

    assert_eq!(first, second);
    assert_eq!(first.changed_roots().len(), 2);
    assert_eq!(first.invalidated().len(), 4);
}

#[test]
fn cyclic_dependency_graphs_do_not_cause_unbounded_invalidation() {
    let mut graph = DependencyGraph::new();

    add_edge(
        &mut graph,
        "artifact:a",
        "artifact:b",
        DependencyKind::Semantic,
    );

    add_edge(
        &mut graph,
        "artifact:b",
        "artifact:c",
        DependencyKind::Semantic,
    );

    add_edge(
        &mut graph,
        "artifact:c",
        "artifact:a",
        DependencyKind::Semantic,
    );

    let plan = InvalidationPlanner::new()
        .plan(&graph, &key("artifact:a"))
        .expect("cycle-safe invalidation");

    assert_eq!(plan.len(), 3);
    assert!(plan.contains(&key("artifact:a")));
    assert!(plan.contains(&key("artifact:b")));
    assert!(plan.contains(&key("artifact:c")));
    assert!(graph.validate().is_ok());
}

#[test]
fn empty_invalidation_roots_are_rejected() {
    let graph = DependencyGraph::new();

    let result = InvalidationPlanner::new().plan_many(
        &graph,
        std::iter::empty::<DependencyKey>(),
    );

    assert!(matches!(result, Err(InvalidationError::EmptyRoots)));
}

#[test]
fn unknown_invalidation_roots_are_rejected() {
    let graph = DependencyGraph::new();

    let result = InvalidationPlanner::new()
        .plan(&graph, &key("missing:source"));

    assert!(matches!(
        result,
        Err(InvalidationError::Dependency(_))
    ));
}

#[test]
fn a_plan_becomes_stale_after_a_graph_mutation() {
    let mut graph = DependencyGraph::new();

    graph
        .add_node(key("source:main.zm"))
        .expect("register source");

    let planner = InvalidationPlanner::new();

    let plan = planner
        .plan(&graph, &key("source:main.zm"))
        .expect("plan");

    assert!(plan.is_current_for(&graph));
    planner.ensure_current(&graph, &plan).expect("current plan");

    graph
        .add_node(key("source:other.zm"))
        .expect("mutate graph");

    assert!(!plan.is_current_for(&graph));

    assert!(matches!(
        planner.ensure_current(&graph, &plan),
        Err(InvalidationError::StalePlan { .. })
    ));
}

#[test]
fn planning_does_not_mutate_the_dependency_graph() {
    let mut graph = DependencyGraph::new();

    add_edge(
        &mut graph,
        "ir:main",
        "semantic:main",
        DependencyKind::Ir,
    );

    let generation_before = graph.generation();
    let nodes_before = graph.node_count();
    let edges_before = graph.edge_count();

    let _plan = InvalidationPlanner::new()
        .plan(&graph, &key("semantic:main"))
        .expect("plan");

    assert_eq!(graph.generation(), generation_before);
    assert_eq!(graph.node_count(), nodes_before);
    assert_eq!(graph.edge_count(), edges_before);
}

// ---------------------------------------------------------------------------
// Cache-key determinism and sensitivity
// ---------------------------------------------------------------------------

#[test]
fn identical_cache_inputs_produce_identical_keys() {
    let identity = key("zuir:module:main");
    let dependencies = vec![
        DependencyFingerprint::new(key("source:main.zm"), digest("source")),
        DependencyFingerprint::new(key("semantic:module:math"), digest("math")),
    ];

    let first = derive_key(&cache_input(&identity, &dependencies));
    let second = derive_key(&cache_input(&identity, &dependencies));

    assert_eq!(first, second);
    assert_eq!(first.to_hex().len(), 64);
}

#[test]
fn dependency_order_does_not_change_the_cache_key() {
    let identity = key("zuir:module:main");

    let first_dependencies = vec![
        DependencyFingerprint::new(key("source:b.zm"), digest("b")),
        DependencyFingerprint::new(key("source:a.zm"), digest("a")),
    ];

    let second_dependencies = vec![
        DependencyFingerprint::new(key("source:a.zm"), digest("a")),
        DependencyFingerprint::new(key("source:b.zm"), digest("b")),
    ];

    assert_eq!(
        derive_key(&cache_input(&identity, &first_dependencies)),
        derive_key(&cache_input(&identity, &second_dependencies)),
    );
}

#[test]
fn duplicate_dependency_identities_are_rejected() {
    let identity = key("zuir:module:main");

    let dependencies = vec![
        DependencyFingerprint::new(key("source:main.zm"), digest("source-v1")),
        DependencyFingerprint::new(key("source:main.zm"), digest("source-v2")),
    ];

    let result = CacheKey::derive(&cache_input(&identity, &dependencies));

    assert!(matches!(
        result,
        Err(CacheKeyError::DuplicateDependency { .. })
    ));
}

#[test]
fn changing_a_dependency_fingerprint_changes_the_cache_key() {
    let identity = key("zuir:module:main");

    let old = vec![DependencyFingerprint::new(
        key("source:main.zm"),
        digest("old-source"),
    )];

    let new = vec![DependencyFingerprint::new(
        key("source:main.zm"),
        digest("new-source"),
    )];

    assert_ne!(
        derive_key(&cache_input(&identity, &old)),
        derive_key(&cache_input(&identity, &new)),
    );
}

#[test]
fn target_specific_and_target_independent_keys_are_distinct() {
    let identity = key("zuir:module:main");
    let dependencies = Vec::new();

    let portable = cache_input(&identity, &dependencies);
    let portable_key = derive_key(&portable);

    let mut target_specific = cache_input(&identity, &dependencies);
    target_specific.target_fingerprint = Some(digest("target-profile-a"));

    let target_key = derive_key(&target_specific);

    assert_ne!(portable_key, target_key);

    let mut another_target = cache_input(&identity, &dependencies);
    another_target.target_fingerprint = Some(digest("target-profile-b"));

    assert_ne!(target_key, derive_key(&another_target));
}

#[test]
fn each_semantically_relevant_cache_input_is_keyed() {
    let identity = key("zuir:module:main");
    let dependencies = Vec::new();

    let baseline_input = cache_input(&identity, &dependencies);
    let baseline = derive_key(&baseline_input);

    let mut changed = cache_input(&identity, &dependencies);
    changed.direct_input = digest("changed-direct-input");
    assert_ne!(baseline, derive_key(&changed));

    let mut changed = cache_input(&identity, &dependencies);
    changed.configuration_fingerprint = digest("changed-configuration");
    assert_ne!(baseline, derive_key(&changed));

    let mut changed = cache_input(&identity, &dependencies);
    changed.pass_pipeline_fingerprint = digest("changed-pipeline");
    assert_ne!(baseline, derive_key(&changed));

    let mut changed = cache_input(&identity, &dependencies);
    changed.semantic_environment_fingerprint = digest("changed-semantic-environment");
    assert_ne!(baseline, derive_key(&changed));

    let mut changed = cache_input(&identity, &dependencies);
    changed.compiler_identity = "different-compiler-build";
    assert_ne!(baseline, derive_key(&changed));

    let mut changed = cache_input(&identity, &dependencies);
    changed.artifact_kind = "different.artifact-kind";
    assert_ne!(baseline, derive_key(&changed));

    let different_identity = key("zuir:module:other");
    let changed = cache_input(&different_identity, &dependencies);
    assert_ne!(baseline, derive_key(&changed));

    let mut changed = cache_input(&identity, &dependencies);
    changed.schema_version = ZuirSchemaVersion::new(
        CURRENT_ZUIR_SCHEMA_VERSION.major(),
        CURRENT_ZUIR_SCHEMA_VERSION.minor(),
        CURRENT_ZUIR_SCHEMA_VERSION
            .patch()
            .checked_add(1)
            .expect("test schema patch increment"),
    );
    assert_ne!(baseline, derive_key(&changed));

    let mut changed = cache_input(&identity, &dependencies);
    changed.format_version = ZuirFormatVersion::new(
        CURRENT_ZUIR_FORMAT_VERSION.major(),
        CURRENT_ZUIR_FORMAT_VERSION.minor(),
        CURRENT_ZUIR_FORMAT_VERSION
            .patch()
            .checked_add(1)
            .expect("test format patch increment"),
    );
    assert_ne!(baseline, derive_key(&changed));
}

// ---------------------------------------------------------------------------
// Grammar and frontend dependency integration
// ---------------------------------------------------------------------------

#[test]
fn grammar_frontend_semantics_and_ir_form_an_explicit_dependency_chain() {
    let mut graph = DependencyGraph::new();

    add_edge(
        &mut graph,
        "frontend:lexer",
        "grammar:lexer-contract",
        DependencyKind::Grammar,
    );

    add_edge(
        &mut graph,
        "frontend:parser",
        "grammar:root",
        DependencyKind::Grammar,
    );

    add_edge(
        &mut graph,
        "frontend:parser",
        "frontend:lexer",
        DependencyKind::Source,
    );

    add_edge(
        &mut graph,
        "semantic:module:main",
        "frontend:parser",
        DependencyKind::SymbolResolution,
    );

    add_edge(
        &mut graph,
        "zuir:module:main",
        "semantic:module:main",
        DependencyKind::Ir,
    );

    add_edge(
        &mut graph,
        "analysis:main",
        "zuir:module:main",
        DependencyKind::Analysis,
    );

    let plan = InvalidationPlanner::new()
        .plan(&graph, &key("grammar:root"))
        .expect("grammar-root invalidation");

    let actual = plan
        .invalidated()
        .iter()
        .map(DependencyKey::as_str)
        .collect::<BTreeSet<_>>();

    let expected = [
        "grammar:root",
        "frontend:parser",
        "semantic:module:main",
        "zuir:module:main",
        "analysis:main",
    ]
    .into_iter()
    .collect::<BTreeSet<_>>();

    assert_eq!(actual, expected);
}

#[test]
fn grammar_and_source_changes_invalidate_the_correct_downstream_artifacts() {
    let mut graph = DependencyGraph::new();

    // A representative dependency model, not a parser implementation.
    add_edge(
        &mut graph,
        "frontend:lexer",
        "grammar:lexer",
        DependencyKind::Grammar,
    );

    add_edge(
        &mut graph,
        "frontend:parser",
        "grammar:root",
        DependencyKind::Grammar,
    );

    add_edge(
        &mut graph,
        "semantic:module:main",
        "frontend:parser",
        DependencyKind::SymbolResolution,
    );

    add_edge(
        &mut graph,
        "zuir:module:main",
        "semantic:module:main",
        DependencyKind::Ir,
    );

    let planner = InvalidationPlanner::new();

    let lexer_plan = planner
        .plan(&graph, &key("grammar:lexer"))
        .expect("lexer grammar plan");

    assert!(lexer_plan.contains(&key("frontend:lexer")));
    assert!(!lexer_plan.contains(&key("frontend:parser")));

    let root_plan = planner
        .plan(&graph, &key("grammar:root"))
        .expect("root grammar plan");

    assert!(root_plan.contains(&key("frontend:parser")));
    assert!(root_plan.contains(&key("semantic:module:main")));
    assert!(root_plan.contains(&key("zuir:module:main")));
}

// ---------------------------------------------------------------------------
// Scalable generated workloads
// ---------------------------------------------------------------------------

#[test]
fn generated_dependency_chain_invalidates_every_downstream_artifact() {
    // This value is only a test workload. It is not a ZUIR size limit.
    let workload_size: usize = std::env::var("ZUIR_INCREMENTAL_TEST_NODES")
        .ok()
        .and_then(|value| value.parse().ok())
        .filter(|value| *value > 0)
        .unwrap_or(2_048);

    let mut graph = DependencyGraph::new();

    for index in 1..workload_size {
        add_edge(
            &mut graph,
            &format!("artifact:{index}"),
            &format!("artifact:{}", index - 1),
            DependencyKind::Transformation,
        );
    }

    assert!(graph.validate().is_ok());

    let plan = InvalidationPlanner::new()
        .plan(&graph, &key("artifact:0"))
        .expect("large chain invalidation");

    assert_eq!(plan.len(), workload_size);
    assert!(plan.contains(&key(&format!("artifact:{}", workload_size - 1))));
}

#[test]
fn generated_disconnected_components_do_not_invalidate_unrelated_artifacts() {
    let component_count: usize = std::env::var("ZUIR_INCREMENTAL_TEST_COMPONENTS")
        .ok()
        .and_then(|value| value.parse().ok())
        .filter(|value| *value > 0)
        .unwrap_or(128);

    let mut graph = DependencyGraph::new();

    for index in 0..component_count {
        let source = format!("source:{index}");
        let semantic = format!("semantic:{index}");
        let ir = format!("ir:{index}");

        add_edge(
            &mut graph,
            &semantic,
            &source,
            DependencyKind::Source,
        );

        add_edge(
            &mut graph,
            &ir,
            &semantic,
            DependencyKind::Ir,
        );
    }

    let plan = InvalidationPlanner::new()
        .plan(&graph, &key("source:0"))
        .expect("component invalidation");

    assert_eq!(plan.len(), 3);
    assert!(plan.contains(&key("source:0")));
    assert!(plan.contains(&key("semantic:0")));
    assert!(plan.contains(&key("ir:0")));

    if component_count > 1 {
        assert!(!plan.contains(&key("source:1")));
        assert!(!plan.contains(&key("semantic:1")));
        assert!(!plan.contains(&key("ir:1")));
    }
}

#[test]
fn cache_key_dependency_order_remains_stable_for_generated_inputs() {
    let identity = key("zuir:generated-module");
    let workload_size: usize = std::env::var("ZUIR_INCREMENTAL_TEST_DEPENDENCIES")
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(256);

    let dependencies = (0..workload_size)
        .map(|index| {
            DependencyFingerprint::new(
                key(&format!("dependency:{index}")),
                digest(&format!("content:{index}")),
            )
        })
        .collect::<Vec<_>>();

    let mut reversed = dependencies.clone();
    reversed.reverse();

    let first = derive_key(&cache_input(&identity, &dependencies));
    let second = derive_key(&cache_input(&identity, &reversed));

    assert_eq!(first, second);
}

// ---------------------------------------------------------------------------
// Error handling and input validation
// ---------------------------------------------------------------------------

#[test]
fn invalid_dependency_keys_are_rejected() {
    assert!(DependencyKey::new("").is_err());
    assert!(DependencyKey::new("   ").is_err());
    assert!(DependencyKey::new("source\0main.zm").is_err());
}

#[test]
fn malformed_cache_key_fields_are_rejected() {
    let identity = key("zuir:module:main");
    let dependencies = Vec::new();

    let mut input = cache_input(&identity, &dependencies);
    input.compiler_identity = "   ";

    assert!(CacheKey::derive(&input).is_err());

    let mut input = cache_input(&identity, &dependencies);
    input.artifact_kind = "";

    assert!(CacheKey::derive(&input).is_err());
}