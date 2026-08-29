//! Zamani Quantum Routing — Shortest-Path Routing Test Suite
//!
//! `src/quantum/routing/tests/shortest_path.rs`
//!
//! # Responsibility
//!
//! Production-level tests for deterministic shortest-path routing.
//!
//! This file verifies the public contracts of:
//!
//! - `routing::path::PathFinder`;
//! - `routing::path::PathResult`;
//! - `routing::path::PathSearchConfig`;
//! - `routing::algorithms::shortest_path::ShortestPathRouter`;
//! - `routing::topology::PhysicalTopology`;
//! - `routing::topology::TopologyBuilder`;
//! - `routing::types::PhysicalQubitId`;
//! - `routing::errors::RoutingError`.
//!
//! The tests deliberately do not inspect private implementation details.
//! They test observable routing behavior and invariants.
//!
//! # What this file guarantees
//!
//! The shortest-path subsystem must:
//!
//! - find the minimum-hop path on an unweighted topology;
//! - include both source and target in a returned path;
//! - return a zero-edge path when source == target;
//! - produce deterministic results;
//! - use deterministic tie-breaking for equal-length paths;
//! - work in both traversal directions on undirected topology;
//! - reject invalid physical-qubit references;
//! - fail cleanly when no route exists;
//! - respect maximum path-length limits;
//! - respect maximum visited-vertex limits;
//! - reject invalid search configuration;
//! - preserve topology semantics;
//! - avoid mutating topology;
//! - expose correct path distance and edge count;
//! - preserve deterministic behavior across repeated searches;
//! - expose the stable shortest-path algorithm identity;
//! - integrate the algorithm with `PathFinder`;
//! - avoid unsafe Rust.
//!
//! # Architectural boundary
//!
//! ```text
//! PhysicalTopology
//!        │
//!        ▼
//!     PathFinder
//!        │
//!        ▼
//! ShortestPathRouter
//!        │
//!        ├── Basic routing
//!        ├── Lookahead
//!        ├── SABRE
//!        └── higher-level router
//! ```
//!
//! This test file does NOT test:
//!
//! - SABRE heuristics;
//! - lookahead heuristics;
//! - noise-aware scoring;
//! - layout selection;
//! - hardware-provider APIs;
//! - compiler IR mutation;
//! - OpenQASM parsing;
//! - gate decomposition;
//! - scheduling;
//! - pulse generation;
//! - simulation;
//! - QEC decoding.
//!
//! Those contracts belong to their respective test modules.
//!
//! # Important production invariant
//!
//! Shortest-path routing is a graph primitive. It must not silently assume
//! that structural adjacency means that every quantum gate is executable.
//!
//! Gate-specific executability is tested by topology tests and by the
//! higher-level shortest-path router integration tests.
//!
//! # Determinism
//!
//! No test relies on HashMap iteration order.
//!
//! Equal-length paths must have deterministic tie-breaking.
//!
//! # Transactionality
//!
//! `ShortestPathRouter` must not mutate the caller's topology or mapping while
//! merely performing path searches.
//!
//! Mapping transactionality is covered more deeply by the mapping and
//! transactional routing test suites.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe Rust.
//!
//! # Safety
//!
//! This test module contains no unsafe code.
//!
//! # Integration contract
//!
//! This test intentionally consumes the public routing API exactly as later
//! production modules should consume it.
//!
//! The dependency direction is:
//!
//! ```text
//! types.rs
//!    │
//!    ▼
//! topology.rs
//!    │
//!    ▼
//! path.rs
//!    │
//!    ▼
//! algorithms/shortest_path.rs
//!    │
//!    ▼
//! router.rs
//!    │
//!    ▼
//! verification.rs / transpiler.rs
//! ```
//!
//! If a future implementation changes an internal data structure while these
//! public contracts remain stable, this test file must continue to compile
//! unchanged.
//!
//! This is intentional and supports Zamani's "finish each file once" rule.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use crate::quantum::routing::algorithms::shortest_path::{
    ShortestPathRouter,
    SHORTEST_PATH_ALGORITHM_NAME,
    SHORTEST_PATH_ALGORITHM_VERSION,
};
use crate::quantum::routing::errors::RoutingError;
use crate::quantum::routing::path::{
    PathFinder,
    PathSearchConfig,
};
use crate::quantum::routing::topology::{
    PhysicalTopology,
    TopologyBuilder,
};
use crate::quantum::routing::types::PhysicalQubitId;

// =============================================================================
// Test helpers
// =============================================================================

fn q(index: usize) -> PhysicalQubitId {
    PhysicalQubitId::new(index)
}

fn line_topology(count: usize) -> PhysicalTopology {
    PhysicalTopology::line(count)
        .expect("test line topology must be valid")
}

fn builder_with_qubits(count: usize) -> TopologyBuilder {
    let mut builder = TopologyBuilder::named("shortest-path-test");

    for index in 0..count {
        builder = builder
            .add_qubit(q(index))
            .expect("test qubit registration must succeed");
    }

    builder
}

fn diamond_topology() -> PhysicalTopology {
    builder_with_qubits(4)
        .undirected_edge(q(0), q(1))
        .expect("edge 0-1 must succeed")
        .undirected_edge(q(1), q(3))
        .expect("edge 1-3 must succeed")
        .undirected_edge(q(0), q(2))
        .expect("edge 0-2 must succeed")
        .undirected_edge(q(2), q(3))
        .expect("edge 2-3 must succeed")
        .build()
        .expect("diamond topology must be valid")
}

fn assert_disconnected<T>(
    result: Result<T, RoutingError>,
) {
    assert!(
        result.is_err(),
        "expected shortest-path search to fail for disconnected qubits"
    );
}

// =============================================================================
// Basic shortest-path correctness
// =============================================================================

#[test]
fn shortest_path_finds_minimum_hop_path_on_line() {
    let topology = line_topology(5);
    let finder = PathFinder::new();

    let result = finder
        .shortest_path(&topology, q(0), q(4))
        .expect("0 -> 4 must be reachable");

    assert_eq!(
        result.vertices(),
        &[q(0), q(1), q(2), q(3), q(4)]
    );

    assert_eq!(result.edge_count(), 4);
    assert_eq!(result.distance(), 4);
    assert_eq!(result.source(), q(0));
    assert_eq!(result.target(), q(4));
}

#[test]
fn shortest_path_is_zero_edges_for_same_source_and_target() {
    let topology = line_topology(5);
    let finder = PathFinder::new();

    let result = finder
        .shortest_path(&topology, q(2), q(2))
        .expect("source equal to target must be routable");

    assert_eq!(result.vertices(), &[q(2)]);
    assert_eq!(result.edge_count(), 0);
    assert_eq!(result.distance(), 0);
    assert_eq!(result.source(), q(2));
    assert_eq!(result.target(), q(2));
    assert!(result.is_trivial());
    assert!(!result.is_empty());
}

#[test]
fn shortest_path_works_in_reverse_direction_on_undirected_topology() {
    let topology = line_topology(5);
    let finder = PathFinder::new();

    let result = finder
        .shortest_path(&topology, q(4), q(0))
        .expect("4 -> 0 must be reachable");

    assert_eq!(
        result.vertices(),
        &[q(4), q(3), q(2), q(1), q(0)]
    );

    assert_eq!(result.edge_count(), 4);
    assert_eq!(result.distance(), 4);
}

#[test]
fn shortest_distance_matches_shortest_path_distance() {
    let topology = line_topology(8);
    let finder = PathFinder::new();

    let path = finder
        .shortest_path(&topology, q(1), q(7))
        .expect("1 -> 7 must be reachable");

    let distance = finder
        .shortest_distance(&topology, q(1), q(7))
        .expect("distance 1 -> 7 must be computable");

    assert_eq!(path.distance(), distance);
    assert_eq!(distance, 6);
}

// =============================================================================
// Determinism
// =============================================================================

#[test]
fn shortest_path_is_deterministic_across_repeated_runs() {
    let topology = diamond_topology();
    let finder = PathFinder::new();

    let first = finder
        .shortest_path(&topology, q(0), q(3))
        .expect("first search must succeed");

    for _ in 0..100 {
        let repeated = finder
            .shortest_path(&topology, q(0), q(3))
            .expect("repeated search must succeed");

        assert_eq!(
            repeated.vertices(),
            first.vertices(),
            "shortest-path output must remain deterministic"
        );

        assert_eq!(
            repeated.distance(),
            first.distance(),
            "shortest-path distance must remain deterministic"
        );
    }
}

#[test]
fn equal_length_paths_use_deterministic_tie_breaking() {
    let topology = diamond_topology();
    let finder = PathFinder::new();

    let result = finder
        .shortest_path(&topology, q(0), q(3))
        .expect("diamond route must exist");

    /*
     * There are two equally short paths:
     *
     *     0 -> 1 -> 3
     *     0 -> 2 -> 3
     *
     * The routing contract specifies deterministic tie-breaking.
     *
     * The current path implementation defines equal-cost alternatives by
     * deterministic physical-qubit ordering, therefore the lower-ID branch
     * must be selected.
     */
    assert_eq!(
        result.vertices(),
        &[q(0), q(1), q(3)]
    );

    assert_eq!(result.distance(), 2);
}

#[test]
fn topology_insertion_order_does_not_change_shortest_path() {
    let topology_a = builder_with_qubits(4)
        .undirected_edge(q(0), q(2))
        .expect("edge 0-2 must succeed")
        .undirected_edge(q(2), q(3))
        .expect("edge 2-3 must succeed")
        .undirected_edge(q(0), q(1))
        .expect("edge 0-1 must succeed")
        .undirected_edge(q(1), q(3))
        .expect("edge 1-3 must succeed")
        .build()
        .expect("topology A must be valid");

    let topology_b = builder_with_qubits(4)
        .undirected_edge(q(1), q(3))
        .expect("edge 1-3 must succeed")
        .undirected_edge(q(0), q(1))
        .expect("edge 0-1 must succeed")
        .undirected_edge(q(2), q(3))
        .expect("edge 2-3 must succeed")
        .undirected_edge(q(0), q(2))
        .expect("edge 0-2 must succeed")
        .build()
        .expect("topology B must be valid");

    let finder = PathFinder::new();

    let path_a = finder
        .shortest_path(&topology_a, q(0), q(3))
        .expect("topology A route must exist");

    let path_b = finder
        .shortest_path(&topology_b, q(0), q(3))
        .expect("topology B route must exist");

    assert_eq!(
        path_a.vertices(),
        path_b.vertices(),
        "topology insertion order must not influence routing"
    );
}

// =============================================================================
// Disconnected topology
// =============================================================================

#[test]
fn shortest_path_rejects_disconnected_qubits() {
    let topology = builder_with_qubits(4)
        .undirected_edge(q(0), q(1))
        .expect("edge 0-1 must succeed")
        .undirected_edge(q(2), q(3))
        .expect("edge 2-3 must succeed")
        .build()
        .expect("disconnected topology is still structurally valid");

    let finder = PathFinder::new();

    let result = finder.shortest_path(
        &topology,
        q(0),
        q(3),
    );

    assert_disconnected(result);
}

#[test]
fn shortest_distance_rejects_disconnected_qubits() {
    let topology = builder_with_qubits(4)
        .undirected_edge(q(0), q(1))
        .expect("edge 0-1 must succeed")
        .undirected_edge(q(2), q(3))
        .expect("edge 2-3 must succeed")
        .build()
        .expect("disconnected topology is structurally valid");

    let finder = PathFinder::new();

    let result = finder.shortest_distance(
        &topology,
        q(0),
        q(3),
    );

    assert!(
        result.is_err(),
        "distance calculation must fail for disconnected qubits"
    );
}

// =============================================================================
// Invalid physical-qubit references
// =============================================================================

#[test]
fn shortest_path_rejects_unknown_source_qubit() {
    let topology = line_topology(4);
    let finder = PathFinder::new();

    let result = finder.shortest_path(
        &topology,
        q(99),
        q(0),
    );

    assert!(
        result.is_err(),
        "unknown source qubit must be rejected"
    );
}

#[test]
fn shortest_path_rejects_unknown_target_qubit() {
    let topology = line_topology(4);
    let finder = PathFinder::new();

    let result = finder.shortest_path(
        &topology,
        q(0),
        q(99),
    );

    assert!(
        result.is_err(),
        "unknown target qubit must be rejected"
    );
}

#[test]
fn shortest_distance_rejects_unknown_source_qubit() {
    let topology = line_topology(4);
    let finder = PathFinder::new();

    let result = finder.shortest_distance(
        &topology,
        q(99),
        q(0),
    );

    assert!(
        result.is_err(),
        "unknown source must be rejected by distance search"
    );
}

#[test]
fn shortest_distance_rejects_unknown_target_qubit() {
    let topology = line_topology(4);
    let finder = PathFinder::new();

    let result = finder.shortest_distance(
        &topology,
        q(0),
        q(99),
    );

    assert!(
        result.is_err(),
        "unknown target must be rejected by distance search"
    );
}

// =============================================================================
// Path-search configuration
// =============================================================================

#[test]
fn default_path_search_configuration_is_valid() {
    let config = PathSearchConfig::default();

    assert!(
        config.validate().is_ok(),
        "production default path configuration must validate"
    );

    assert!(
        config.deterministic,
        "production path search must be deterministic by default"
    );

    assert!(
        !config.allow_unavailable,
        "production path search must reject unavailable resources by default"
    );

    assert!(
        config.max_visited_vertices > 0,
        "default visited-vertex limit must be positive"
    );
}

#[test]
fn zero_max_visited_vertices_is_rejected() {
    let config = PathSearchConfig::default()
        .with_max_visited_vertices(0);

    let result = config.validate();

    assert!(
        matches!(result, Err(RoutingError::InvalidConfiguration(_))),
        "zero visited-vertex limit must be rejected, got {result:?}"
    );
}

#[test]
fn zero_max_path_edges_is_rejected() {
    let config = PathSearchConfig::default()
        .with_max_path_edges(Some(0));

    let result = config.validate();

    assert!(
        matches!(result, Err(RoutingError::InvalidConfiguration(_))),
        "zero path-edge limit must be rejected, got {result:?}"
    );
}

#[test]
fn finite_max_path_edges_allows_exact_limit() {
    let config = PathSearchConfig::default()
        .with_max_path_edges(Some(4));

    assert!(
        config.validate().is_ok(),
        "positive path-edge limit must validate"
    );

    let finder = PathFinder::with_config(config)
        .expect("valid path-search configuration must construct");

    let topology = line_topology(5);

    let result = finder.shortest_path(
        &topology,
        q(0),
        q(4),
    );

    assert!(
        result.is_ok(),
        "path exactly at the configured limit must succeed"
    );

    let path = result.expect("path must exist");

    assert_eq!(path.edge_count(), 4);
}

#[test]
fn_max_path_edges_below_required_route_rejects_route() {
    let config = PathSearchConfig::default()
        .with_max_path_edges(Some(3));

    let finder = PathFinder::with_config(config)
        .expect("valid path-search configuration must construct");

    let topology = line_topology(5);

    let result = finder.shortest_path(
        &topology,
        q(0),
        q(4),
    );

    assert!(
        result.is_err(),
        "route longer than configured maximum must fail"
    );
}

#[test]
fn max_visited_vertices_limit_is_enforced() {
    let config = PathSearchConfig::default()
        .with_max_visited_vertices(1);

    let finder = PathFinder::with_config(config)
        .expect("positive visited-vertex limit must construct");

    let topology = line_topology(5);

    let result = finder.shortest_path(
        &topology,
        q(0),
        q(4),
    );

    assert!(
        result.is_err(),
        "search requiring more than one visited vertex must fail"
    );
}

#[test]
fn path_search_builder_configuration_is_reusable() {
    let config = PathSearchConfig::default()
        .with_max_path_edges(Some(8))
        .with_max_visited_vertices(128)
        .with_allow_unavailable(false)
        .with_deterministic(true);

    assert!(config.validate().is_ok());

    let finder = PathFinder::with_config(config)
        .expect("configured path finder must construct");

    let topology = line_topology(9);

    let result = finder
        .shortest_path(&topology, q(0), q(8))
        .expect("configured path finder must route within limits");

    assert_eq!(result.edge_count(), 8);
    assert_eq!(result.distance(), 8);
}

// =============================================================================
// Larger topologies
// =============================================================================

#[test]
fn shortest_path_handles_large_linear_topology() {
    let topology = line_topology(1_000);
    let finder = PathFinder::new();

    let result = finder
        .shortest_path(&topology, q(0), q(999))
        .expect("large line topology must be routable");

    assert_eq!(result.edge_count(), 999);
    assert_eq!(result.distance(), 999);
    assert_eq!(result.source(), q(0));
    assert_eq!(result.target(), q(999));
}

#[test]
fn shortest_path_handles_single_qubit_topology() {
    let topology = PhysicalTopology::isolated(1)
        .expect("single-qubit topology must be valid");

    let finder = PathFinder::new();

    let result = finder
        .shortest_path(&topology, q(0), q(0))
        .expect("single qubit must route to itself");

    assert_eq!(result.vertices(), &[q(0)]);
    assert_eq!(result.edge_count(), 0);
    assert_eq!(result.distance(), 0);
}

#[test]
fn shortest_path_handles_many_queries_without_state_leakage() {
    let topology = line_topology(32);
    let finder = PathFinder::new();

    for source in 0..32 {
        for target in 0..32 {
            let result = finder
                .shortest_path(&topology, q(source), q(target))
                .expect("every pair in a line must be connected");

            assert_eq!(
                result.distance() as usize,
                source.abs_diff(target)
            );

            assert_eq!(
                result.edge_count(),
                source.abs_diff(target)
            );

            assert_eq!(result.source(), q(source));
            assert_eq!(result.target(), q(target));
        }
    }
}

// =============================================================================
// Topology immutability
// =============================================================================

#[test]
fn shortest_path_does_not_mutate_topology() {
    let topology = diamond_topology();
    let before = topology.clone();

    let finder = PathFinder::new();

    let _ = finder
        .shortest_path(&topology, q(0), q(3))
        .expect("diamond route must succeed");

    assert_eq!(
        topology,
        before,
        "path search must never mutate topology"
    );
}

#[test]
fn repeated_queries_do_not_change_topology() {
    let topology = line_topology(16);
    let before = topology.clone();

    let finder = PathFinder::new();

    for _ in 0..100 {
        let _ = finder
            .shortest_path(&topology, q(0), q(15))
            .expect("line route must succeed");
    }

    assert_eq!(
        topology,
        before,
        "repeated path searches must be side-effect free"
    );
}

// =============================================================================
// Path-result contract
// =============================================================================

#[test]
fn path_result_contains_both_endpoints() {
    let topology = line_topology(6);
    let finder = PathFinder::new();

    let result = finder
        .shortest_path(&topology, q(1), q(5))
        .expect("route must succeed");

    let vertices = result.vertices();

    assert_eq!(
        vertices.first().copied(),
        Some(q(1))
    );

    assert_eq!(
        vertices.last().copied(),
        Some(q(5))
    );
}

#[test]
fn path_result_edge_count_is_vertex_count_minus_one() {
    let topology = line_topology(10);
    let finder = PathFinder::new();

    let result = finder
        .shortest_path(&topology, q(2), q(8))
        .expect("route must succeed");

    assert_eq!(
        result.edge_count(),
        result.vertices().len() - 1
    );
}

#[test]
fn path_result_distance_matches_edge_count_for_unweighted_search() {
    let topology = line_topology(20);
    let finder = PathFinder::new();

    let result = finder
        .shortest_path(&topology, q(3), q(17))
        .expect("route must succeed");

    assert_eq!(
        result.distance(),
        result.edge_count() as u64
    );
}

#[test]
fn returned_path_contains_only_adjacent_edges() {
    let topology = line_topology(12);
    let finder = PathFinder::new();

    let result = finder
        .shortest_path(&topology, q(0), q(11))
        .expect("route must succeed");

    for pair in result.vertices().windows(2) {
        let from = pair[0];
        let to = pair[1];

        assert!(
            topology.is_adjacent(from, to),
            "returned path contains a non-adjacent edge: {from:?} -> {to:?}"
        );
    }
}

// =============================================================================
// Algorithm identity / integration
// =============================================================================

#[test]
fn shortest_path_algorithm_has_stable_name() {
    let router = ShortestPathRouter::new();

    assert_eq!(
        router.name(),
        SHORTEST_PATH_ALGORITHM_NAME
    );

    assert_eq!(
        router.name(),
        "shortest_path"
    );
}

#[test]
fn shortest_path_algorithm_has_stable_version() {
    let router = ShortestPathRouter::new();

    assert_eq!(
        router.version(),
        SHORTEST_PATH_ALGORITHM_VERSION
    );

    assert!(
        !router.version().is_empty(),
        "algorithm version must never be empty"
    );
}

#[test]
fn shortest_path_router_uses_path_finder_contract() {
    let router = ShortestPathRouter::new();
    let topology = line_topology(7);

    let result = router
        .path_finder()
        .shortest_path(&topology, q(0), q(6))
        .expect("router's path finder must route a connected topology");

    assert_eq!(
        result.vertices(),
        &[q(0), q(1), q(2), q(3), q(4), q(5), q(6)]
    );

    assert_eq!(result.distance(), 6);
}

#[test]
fn shortest_path_router_is_reusable_without_state_leakage() {
    let router = ShortestPathRouter::new();

    let line = line_topology(5);
    let diamond = diamond_topology();

    let line_result = router
        .path_finder()
        .shortest_path(&line, q(0), q(4))
        .expect("line route must succeed");

    let diamond_result = router
        .path_finder()
        .shortest_path(&diamond, q(0), q(3))
        .expect("diamond route must succeed");

    assert_eq!(line_result.distance(), 4);
    assert_eq!(diamond_result.distance(), 2);

    /*
     * The second query must not inherit any search state from the first.
     */
    let repeated_diamond_result = router
        .path_finder()
        .shortest_path(&diamond, q(0), q(3))
        .expect("repeated diamond route must succeed");

    assert_eq!(
        diamond_result.vertices(),
        repeated_diamond_result.vertices()
    );
}

// =============================================================================
// Regression tests for historical implementation hazards
// =============================================================================

#[test]
fn sorted_neighbor_order_is_required_for_deterministic_binary_search() {
    /*
     * The old routing implementation used binary_search over adjacency lists
     * without guaranteeing that those lists were normalized.
     *
     * The production topology contract now owns deterministic normalization.
     *
     * This test intentionally constructs edges in non-sorted insertion order
     * and verifies that shortest-path routing still works.
     */
    let topology = builder_with_qubits(5)
        .undirected_edge(q(0), q(4))
        .expect("edge 0-4 must succeed")
        .undirected_edge(q(0), q(2))
        .expect("edge 0-2 must succeed")
        .undirected_edge(q(0), q(1))
        .expect("edge 0-1 must succeed")
        .undirected_edge(q(2), q(3))
        .expect("edge 2-3 must succeed")
        .undirected_edge(q(3), q(4))
        .expect("edge 3-4 must succeed")
        .build()
        .expect("topology must be valid");

    let finder = PathFinder::new();

    let result = finder
        .shortest_path(&topology, q(1), q(4))
        .expect("route must succeed");

    assert_eq!(
        result.vertices(),
        &[q(1), q(0), q(4)]
    );

    assert_eq!(result.distance(), 2);
}

#[test]
fn shortest_path_does_not_use_non_shortest_detour() {
    let topology = builder_with_qubits(6)
        .undirected_edge(q(0), q(1))
        .expect("edge 0-1 must succeed")
        .undirected_edge(q(1), q(2))
        .expect("edge 1-2 must succeed")
        .undirected_edge(q(2), q(5))
        .expect("edge 2-5 must succeed")
        .undirected_edge(q(0), q(3))
        .expect("edge 0-3 must succeed")
        .undirected_edge(q(3), q(4))
        .expect("edge 3-4 must succeed")
        .undirected_edge(q(4), q(5))
        .expect("edge 4-5 must succeed")
        .undirected_edge(q(0), q(5))
        .expect("edge 0-5 must succeed")
        .build()
        .expect("topology must be valid");

    let finder = PathFinder::new();

    let result = finder
        .shortest_path(&topology, q(0), q(5))
        .expect("direct route must succeed");

    assert_eq!(
        result.vertices(),
        &[q(0), q(5)]
    );

    assert_eq!(result.distance(), 1);
}

#[test]
fn shortest_path_never_returns_empty_path_for_success() {
    let topology = line_topology(4);
    let finder = PathFinder::new();

    let result = finder
        .shortest_path(&topology, q(0), q(3))
        .expect("route must succeed");

    assert!(
        !result.is_empty(),
        "successful path result must contain at least its endpoints"
    );
}

#[test]
fn shortest_path_configuration_can_be_used_by_router_without_rewriting_contracts() {
    let config = PathSearchConfig::default()
        .with_max_path_edges(Some(16))
        .with_max_visited_vertices(128)
        .with_deterministic(true);

    let router = ShortestPathRouter::with_path_config(config)
        .expect("valid path configuration must create router");

    let topology = line_topology(10);

    let result = router
        .path_finder()
        .shortest_path(&topology, q(0), q(9))
        .expect("route must fit configured limits");

    assert_eq!(result.distance(), 9);
    assert_eq!(result.edge_count(), 9);
}