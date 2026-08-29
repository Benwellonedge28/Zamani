//! Zamani Quantum Routing — Directed Routing Test Suite
//!
//! `src/quantum/routing/tests/directed.rs`
//!
//! # Responsibility
//!
//! Production-grade tests for direction-sensitive physical connectivity and
//! gate execution semantics.
//!
//! This file verifies the distinction between:
//!
//! 1. structural physical connectivity;
//! 2. directed physical reachability;
//! 3. bidirectional physical adjacency;
//! 4. gate-specific executability;
//! 5. directional gate support;
//! 6. explicit supported/unsupported gate declarations;
//! 7. physical-qubit availability;
//! 8. physical-edge availability;
//! 9. path finding over directed connectivity;
//! 10. topology-level gate validation;
//! 11. deterministic behavior;
//! 12. invalid physical references;
//! 13. self-interaction rejection;
//! 14. separation between graph connectivity and gate executability.
//!
//! # Architectural rule
//!
//! A physical connection and a physically executable quantum operation are
//! different concepts.
//!
//! For example:
//!
//! ```text
//! p0 ─────► p1
//! ```
//!
//! may mean:
//!
//! ```text
//! physical reachability:
//!     p0 -> p1       yes
//!     p1 -> p0       no
//!
//! structural connection:
//!     p0 <-> p1      yes
//!
//! gate support:
//!     CX(p0, p1)     yes
//!     CX(p1, p0)     no
//! ```
//!
//! The tests in this file intentionally verify these concepts independently.
//!
//! # Integration contract
//!
//! This test module consumes only stable public routing APIs from:
//!
//! ```text
//! routing/types.rs
//! routing/errors.rs
//! routing/topology.rs
//! routing/path.rs
//! ```
//!
//! It intentionally does NOT depend on:
//!
//! ```text
//! routing/mapping.rs
//! routing/layout.rs
//! routing/candidates.rs
//! routing/algorithms/*
//! routing/router.rs
//! routing/verification.rs
//! routing/transpiler.rs
//! quantum compiler IR
//! hardware providers
//! ```
//!
//! This is deliberate.
//!
//! The topology and path contracts must be independently correct before
//! higher-level routing algorithms consume them.
//!
//! # Future integration
//!
//! Higher-level routing tests may reuse the exact directed-topology fixtures
//! established here.
//!
//! In particular:
//!
//! ```text
//! directed.rs
//!      │
//!      ├──► algorithms/shortest_path.rs
//!      ├──► algorithms/lookahead.rs
//!      ├──► algorithms/sabre.rs
//!      ├──► candidates.rs
//!      ├──► router.rs
//!      └──► verification.rs
//! ```
//!
//! Those modules must consume the same topology semantics rather than
//! redefining directional connectivity.
//!
//! # Production invariants
//!
//! The following invariants are required:
//!
//! - directed adjacency is not automatically bidirectional;
//! - structural connectivity may exist even when directed reachability does
//!   not;
//! - `has_connection(a, b)` describes an underlying physical connection and
//!   therefore may be true in both directions for a directed edge;
//! - `is_adjacent(a, b)` is direction-sensitive;
//! - `is_bidirectionally_adjacent(a, b)` is direction-sensitive;
//! - gate support is direction-sensitive;
//! - an explicitly unsupported gate is never executable;
//! - an unavailable qubit cannot execute a gate;
//! - an unavailable edge cannot execute a gate;
//! - non-adjacent qubits cannot execute a two-qubit gate;
//! - unknown physical qubits are rejected;
//! - a qubit cannot interact with itself;
//! - path finding respects directed adjacency;
//! - path finding remains deterministic;
//! - structural connectivity analysis must not be confused with directed
//!   reachability;
//! - gate names are normalized consistently by the topology API.
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
//! ```text
//! PhysicalTopology
//!        │
//!        ├── structural connectivity
//!        │
//!        ├── directed connectivity
//!        │
//!        ├── gate support
//!        │
//!        └── availability
//!                │
//!                ▼
//!            PathFinder
//!                │
//!                ▼
//!        higher-level routing
//! ```

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use crate::quantum::routing::errors::RoutingError;
use crate::quantum::routing::path::PathFinder;
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

fn builder_with_qubits(count: usize) -> TopologyBuilder {
    let mut builder = TopologyBuilder::named("directed-routing-test");

    for index in 0..count {
        builder = builder
            .add_qubit(q(index))
            .expect("test physical-qubit registration must succeed");
    }

    builder
}

fn one_way_topology() -> PhysicalTopology {
    builder_with_qubits(2)
        .directed_edge(q(0), q(1))
        .expect("directed edge 0 -> 1 must succeed")
        .build()
        .expect("one-way topology must be valid")
}

fn directed_chain_topology() -> PhysicalTopology {
    builder_with_qubits(4)
        .directed_edge(q(0), q(1))
        .expect("edge 0 -> 1 must succeed")
        .directed_edge(q(1), q(2))
        .expect("edge 1 -> 2 must succeed")
        .directed_edge(q(2), q(3))
        .expect("edge 2 -> 3 must succeed")
        .build()
        .expect("directed chain must be valid")
}

fn bidirectional_gate_topology() -> PhysicalTopology {
    builder_with_qubits(2)
        .undirected_edge(q(0), q(1))
        .expect("undirected edge must succeed")
        .supported_gate("cx", q(0), q(1))
        .expect("forward CX support must succeed")
        .supported_gate("cx", q(1), q(0))
        .expect("reverse CX support must succeed")
        .build()
        .expect("bidirectional gate topology must be valid")
}

// =============================================================================
// Directed structural connectivity
// =============================================================================

#[test]
fn directed_edge_is_forward_adjacent_only() {
    let topology = one_way_topology();

    assert!(topology.is_adjacent(q(0), q(1)));
    assert!(!topology.is_adjacent(q(1), q(0)));
}

#[test]
fn directed_edge_is_not_bidirectionally_adjacent() {
    let topology = one_way_topology();

    assert!(!topology.is_bidirectionally_adjacent(q(0), q(1)));
    assert!(!topology.is_bidirectionally_adjacent(q(1), q(0)));
}

#[test]
fn directed_edge_still_represents_an_underlying_connection() {
    let topology = one_way_topology();

    // `has_connection` intentionally represents the existence of an
    // underlying physical connection rather than directed traversal.
    assert!(topology.has_connection(q(0), q(1)));
    assert!(topology.has_connection(q(1), q(0)));
}

#[test]
fn directed_neighbors_are_outgoing_neighbors() {
    let topology = directed_chain_topology();

    assert_eq!(topology.neighbors(q(0)), vec![q(1)]);
    assert_eq!(topology.neighbors(q(1)), vec![q(2)]);
    assert_eq!(topology.neighbors(q(2)), vec![q(3)]);
    assert!(topology.neighbors(q(3)).is_empty());
}

#[test]
fn reverse_direction_has_no_outgoing_neighbor() {
    let topology = one_way_topology();

    assert!(topology.neighbors(q(1)).is_empty());
}

#[test]
fn directed_topology_remains_structurally_connected() {
    let topology = directed_chain_topology();

    // Connectivity analysis is intentionally graph-structural rather than
    // equivalent to directed reachability.
    assert!(topology.is_connected());

    assert_eq!(
        topology.connected_components(),
        vec![vec![q(0), q(1), q(2), q(3)]]
    );
}

// =============================================================================
// Directed degree semantics
// =============================================================================

#[test]
fn outgoing_degree_respects_direction() {
    let topology = builder_with_qubits(3)
        .directed_edge(q(0), q(1))
        .expect("edge 0 -> 1 must succeed")
        .directed_edge(q(2), q(1))
        .expect("edge 2 -> 1 must succeed")
        .build()
        .expect("topology must be valid");

    assert_eq!(topology.outgoing_degree(q(0)), 1);
    assert_eq!(topology.outgoing_degree(q(1)), 0);
    assert_eq!(topology.outgoing_degree(q(2)), 1);
}

#[test]
fn incoming_degree_respects_direction() {
    let topology = builder_with_qubits(3)
        .directed_edge(q(0), q(1))
        .expect("edge 0 -> 1 must succeed")
        .directed_edge(q(2), q(1))
        .expect("edge 2 -> 1 must succeed")
        .build()
        .expect("topology must be valid");

    assert_eq!(topology.incoming_degree(q(0)), 0);
    assert_eq!(topology.incoming_degree(q(1)), 2);
    assert_eq!(topology.incoming_degree(q(2)), 0);
}

#[test]
fn directed_degree_does_not_fake_reverse_edges() {
    let topology = one_way_topology();

    assert_eq!(topology.outgoing_degree(q(0)), 1);
    assert_eq!(topology.outgoing_degree(q(1)), 0);

    assert_eq!(topology.incoming_degree(q(0)), 0);
    assert_eq!(topology.incoming_degree(q(1)), 1);
}

// =============================================================================
// Directed path finding
// =============================================================================

#[test]
fn path_finder_respects_forward_direction() {
    let topology = directed_chain_topology();
    let finder = PathFinder::new();

    let path = finder
        .shortest_path(&topology, q(0), q(3))
        .expect("0 -> 3 must be reachable");

    assert_eq!(
        path.vertices(),
        &[q(0), q(1), q(2), q(3)]
    );

    assert_eq!(path.edge_count(), 3);
    assert_eq!(path.distance(), 3);
}

#[test]
fn path_finder_rejects_reverse_direction_when_no_reverse_edges_exist() {
    let topology = directed_chain_topology();
    let finder = PathFinder::new();

    let result = finder.shortest_path(&topology, q(3), q(0));

    assert!(
        result.is_err(),
        "reverse traversal must not be synthesized from a directed topology"
    );
}

#[test]
fn path_finder_allows_zero_length_directed_path() {
    let topology = directed_chain_topology();
    let finder = PathFinder::new();

    let path = finder
        .shortest_path(&topology, q(2), q(2))
        .expect("a qubit must be reachable from itself");

    assert_eq!(path.vertices(), &[q(2)]);
    assert_eq!(path.edge_count(), 0);
    assert_eq!(path.distance(), 0);
}

#[test]
fn directed_path_is_deterministic() {
    let topology = builder_with_qubits(4)
        .directed_edge(q(0), q(1))
        .expect("edge 0 -> 1 must succeed")
        .directed_edge(q(1), q(3))
        .expect("edge 1 -> 3 must succeed")
        .directed_edge(q(0), q(2))
        .expect("edge 0 -> 2 must succeed")
        .directed_edge(q(2), q(3))
        .expect("edge 2 -> 3 must succeed")
        .build()
        .expect("diamond topology must be valid");

    let finder = PathFinder::new();

    let first = finder
        .shortest_path(&topology, q(0), q(3))
        .expect("forward path must exist");

    let second = finder
        .shortest_path(&topology, q(0), q(3))
        .expect("forward path must exist");

    assert_eq!(first, second);
}

#[test]
fn directed_path_uses_only_outgoing_edges() {
    let topology = builder_with_qubits(4)
        .directed_edge(q(0), q(1))
        .expect("edge 0 -> 1 must succeed")
        .directed_edge(q(2), q(1))
        .expect("edge 2 -> 1 must succeed")
        .directed_edge(q(2), q(3))
        .expect("edge 2 -> 3 must succeed")
        .build()
        .expect("topology must be valid");

    let finder = PathFinder::new();

    let path = finder
        .shortest_path(&topology, q(2), q(1))
        .expect("2 -> 1 must be reachable");

    assert_eq!(path.vertices(), &[q(2), q(1)]);

    let reverse = finder.shortest_path(&topology, q(1), q(2));

    assert!(
        reverse.is_err(),
        "1 -> 2 must not be reachable without a reverse edge"
    );
}

// =============================================================================
// Directional gate support
// =============================================================================

#[test]
fn directional_gate_support_is_not_implied_by_structural_adjacency() {
    let topology = one_way_topology();

    assert!(topology.is_adjacent(q(0), q(1)));
    assert!(!topology.is_adjacent(q(1), q(0)));

    // A gate must still be checked independently.
    assert!(topology.supports_gate("cx", q(0), q(1)));
    assert!(!topology.supports_gate("cx", q(1), q(0)));
}

#[test]
fn explicit_forward_gate_support_is_executable() {
    let topology = builder_with_qubits(2)
        .directed_edge(q(0), q(1))
        .expect("edge must succeed")
        .supported_gate("cx", q(0), q(1))
        .expect("forward CX support must succeed")
        .build()
        .expect("topology must build");

    assert!(topology.has_explicit_gate_support(
        "cx",
        q(0),
        q(1)
    ));

    assert!(topology.supports_gate(
        "cx",
        q(0),
        q(1)
    ));
}

#[test]
fn explicit_forward_gate_support_does_not_enable_reverse_gate() {
    let topology = builder_with_qubits(2)
        .directed_edge(q(0), q(1))
        .expect("edge must succeed")
        .supported_gate("cx", q(0), q(1))
        .expect("forward CX support must succeed")
        .build()
        .expect("topology must build");

    assert!(topology.supports_gate(
        "cx",
        q(0),
        q(1)
    ));

    assert!(!topology.supports_gate(
        "cx",
        q(1),
        q(0)
    ));
}

#[test]
fn explicit_unsupported_reverse_gate_overrides_structural_connection() {
    let topology = builder_with_qubits(2)
        .undirected_edge(q(0), q(1))
        .expect("edge must succeed")
        .supported_gate("cx", q(0), q(1))
        .expect("forward CX support must succeed")
        .unsupported_gate("cx", q(1), q(0))
        .expect("reverse unsupported declaration must succeed")
        .build()
        .expect("topology must build");

    assert!(topology.supports_gate(
        "cx",
        q(0),
        q(1)
    ));

    assert!(!topology.supports_gate(
        "cx",
        q(1),
        q(0)
    ));

    assert!(topology.has_explicit_gate_support(
        "cx",
        q(0),
        q(1)
    ));

    assert!(topology.has_explicit_gate_support(
        "cx",
        q(1),
        q(0)
    ));
}

#[test]
fn both_directions_can_be_explicitly_supported() {
    let topology = bidirectional_gate_topology();

    assert!(topology.supports_gate(
        "cx",
        q(0),
        q(1)
    ));

    assert!(topology.supports_gate(
        "cx",
        q(1),
        q(0)
    ));
}

#[test]
fn directional_gate_support_is_case_insensitive_and_trimmed() {
    let topology = builder_with_qubits(2)
        .directed_edge(q(0), q(1))
        .expect("edge must succeed")
        .supported_gate("  Cx  ", q(0), q(1))
        .expect("normalized gate support must succeed")
        .build()
        .expect("topology must build");

    assert!(topology.supports_gate(
        "cx",
        q(0),
        q(1)
    ));

    assert!(topology.supports_gate(
        "CX",
        q(0),
        q(1)
    ));

    assert!(topology.supports_gate(
        " cX ",
        q(0),
        q(1)
    ));

    assert!(!topology.supports_gate(
        "cx",
        q(1),
        q(0)
    ));
}

// =============================================================================
// Gate validation
// =============================================================================

#[test]
fn validate_gate_accepts_supported_forward_operation() {
    let topology = builder_with_qubits(2)
        .directed_edge(q(0), q(1))
        .expect("edge must succeed")
        .supported_gate("cx", q(0), q(1))
        .expect("forward CX support must succeed")
        .build()
        .expect("topology must build");

    topology
        .validate_gate("cx", q(0), q(1))
        .expect("supported forward gate must validate");
}

#[test]
fn validate_gate_rejects_unsupported_reverse_operation() {
    let topology = builder_with_qubits(2)
        .directed_edge(q(0), q(1))
        .expect("edge must succeed")
        .supported_gate("cx", q(0), q(1))
        .expect("forward CX support must succeed")
        .build()
        .expect("topology must build");

    let result = topology.validate_gate(
        "cx",
        q(1),
        q(0),
    );

    assert!(
        matches!(
            result,
            Err(RoutingError::UnsupportedDirectedGate { .. })
        ),
        "expected UnsupportedDirectedGate, got {result:?}"
    );
}

#[test]
fn validate_gate_rejects_explicitly_unsupported_reverse_operation() {
    let topology = builder_with_qubits(2)
        .undirected_edge(q(0), q(1))
        .expect("edge must succeed")
        .supported_gate("cx", q(0), q(1))
        .expect("forward CX support must succeed")
        .unsupported_gate("cx", q(1), q(0))
        .expect("reverse CX rejection must succeed")
        .build()
        .expect("topology must build");

    let result = topology.validate_gate(
        "cx",
        q(1),
        q(0),
    );

    assert!(
        matches!(
            result,
            Err(RoutingError::UnsupportedDirectedGate { .. })
        ),
        "expected UnsupportedDirectedGate, got {result:?}"
    );
}

#[test]
fn validate_gate_rejects_non_adjacent_directional_operation() {
    let topology = builder_with_qubits(3)
        .directed_edge(q(0), q(1))
        .expect("edge must succeed")
        .directed_edge(q(1), q(2))
        .expect("edge must succeed")
        .build()
        .expect("topology must build");

    let result = topology.validate_gate(
        "cx",
        q(0),
        q(2),
    );

    assert!(
        matches!(
            result,
            Err(RoutingError::UnsupportedDirectedGate { .. })
        ),
        "expected UnsupportedDirectedGate, got {result:?}"
    );
}

#[test]
fn validate_gate_rejects_same_physical_qubit() {
    let topology = PhysicalTopology::line(2)
        .expect("line topology must be valid");

    let result = topology.validate_gate(
        "cx",
        q(0),
        q(0),
    );

    assert!(
        result.is_err(),
        "two-qubit gate using the same physical qubit must be rejected"
    );
}

#[test]
fn validate_gate_rejects_unknown_source_qubit() {
    let topology = PhysicalTopology::line(2)
        .expect("line topology must be valid");

    let result = topology.validate_gate(
        "cx",
        q(99),
        q(1),
    );

    assert!(
        matches!(
            result,
            Err(RoutingError::InvalidPhysicalQubit(_))
        ),
        "expected InvalidPhysicalQubit, got {result:?}"
    );
}

#[test]
fn validate_gate_rejects_unknown_target_qubit() {
    let topology = PhysicalTopology::line(2)
        .expect("line topology must be valid");

    let result = topology.validate_gate(
        "cx",
        q(0),
        q(99),
    );

    assert!(
        matches!(
            result,
            Err(RoutingError::InvalidPhysicalQubit(_))
        ),
        "expected InvalidPhysicalQubit, got {result:?}"
    );
}

#[test]
fn validate_gate_rejects_empty_gate_name() {
    let topology = PhysicalTopology::line(2)
        .expect("line topology must be valid");

    let result = topology.validate_gate(
        "   ",
        q(0),
        q(1),
    );

    assert!(
        matches!(
            result,
            Err(RoutingError::UnsupportedGate(_))
        ),
        "expected UnsupportedGate, got {result:?}"
    );
}

// =============================================================================
// Qubit availability
// =============================================================================

#[test]
fn unavailable_target_qubit_cannot_execute_directional_gate() {
    let mut topology = PhysicalTopology::line(2)
        .expect("line topology must be valid");

    topology
        .set_qubit_available(q(1), false)
        .expect("qubit 1 must exist");

    assert!(topology.is_adjacent(q(0), q(1)));
    assert!(!topology.is_available(q(1)));

    assert!(!topology.supports_gate(
        "cx",
        q(0),
        q(1)
    ));
}

#[test]
fn unavailable_source_qubit_cannot_execute_directional_gate() {
    let mut topology = PhysicalTopology::line(2)
        .expect("line topology must be valid");

    topology
        .set_qubit_available(q(0), false)
        .expect("qubit 0 must exist");

    assert!(!topology.is_available(q(0)));

    assert!(!topology.supports_gate(
        "cx",
        q(0),
        q(1)
    ));
}

#[test]
fn making_a_qubit_unavailable_does_not_delete_directed_connectivity() {
    let mut topology = PhysicalTopology::line(2)
        .expect("line topology must be valid");

    assert!(topology.is_adjacent(q(0), q(1)));
    assert!(topology.is_adjacent(q(1), q(0)));

    topology
        .set_qubit_available(q(1), false)
        .expect("qubit 1 must exist");

    assert!(topology.is_adjacent(q(0), q(1)));
    assert!(topology.is_adjacent(q(1), q(0)));

    assert!(!topology.supports_gate(
        "cx",
        q(0),
        q(1)
    ));
}

// =============================================================================
// Edge availability
// =============================================================================

#[test]
fn unavailable_directed_edge_remains_structurally_present() {
    let mut topology = one_way_topology();

    assert!(topology.has_connection(q(0), q(1)));
    assert!(topology.is_adjacent(q(0), q(1)));

    topology
        .set_edge_available(q(0), q(1), false)
        .expect("directed edge must exist");

    assert!(topology.has_connection(q(0), q(1)));
    assert!(topology.is_adjacent(q(0), q(1)));

    assert!(!topology.supports_gate(
        "cx",
        q(0),
        q(1)
    ));
}

#[test]
fn unavailable_directed_edge_does_not_create_reverse_reachability() {
    let mut topology = one_way_topology();

    topology
        .set_edge_available(q(0), q(1), false)
        .expect("directed edge must exist");

    assert!(!topology.is_adjacent(q(1), q(0)));

    let finder = PathFinder::new();

    let reverse = finder.shortest_path(
        &topology,
        q(1),
        q(0),
    );

    assert!(
        reverse.is_err(),
        "disabling a forward edge must not synthesize reverse reachability"
    );
}

#[test]
fn changing_unknown_directed_edge_availability_is_rejected() {
    let mut topology = one_way_topology();

    let result = topology.set_edge_available(
        q(0),
        q(99),
        false,
    );

    assert!(
        matches!(
            result,
            Err(RoutingError::InvalidTopology(_))
        ),
        "expected InvalidTopology, got {result:?}"
    );
}

// =============================================================================
// Gate support and physical connectivity must remain separate
// =============================================================================

#[test]
fn adjacency_does_not_mean_every_gate_is_supported() {
    let topology = builder_with_qubits(2)
        .directed_edge(q(0), q(1))
        .expect("edge must succeed")
        .unsupported_gate("cz", q(0), q(1))
        .expect("unsupported CZ declaration must succeed")
        .build()
        .expect("topology must build");

    assert!(topology.is_adjacent(q(0), q(1)));
    assert!(topology.has_connection(q(0), q(1)));

    assert!(!topology.supports_gate(
        "cz",
        q(0),
        q(1)
    ));

    // A different gate may still be structurally/executably available.
    assert!(topology.supports_gate(
        "cx",
        q(0),
        q(1)
    ));
}

#[test]
fn explicitly_unsupported_gate_does_not_change_structural_connectivity() {
    let topology = builder_with_qubits(2)
        .directed_edge(q(0), q(1))
        .expect("edge must succeed")
        .unsupported_gate("cx", q(0), q(1))
        .expect("unsupported CX declaration must succeed")
        .build()
        .expect("topology must build");

    assert!(topology.is_adjacent(q(0), q(1)));
    assert!(topology.has_connection(q(0), q(1)));
    assert!(!topology.supports_gate(
        "cx",
        q(0),
        q(1)
    ));
}

// =============================================================================
// Deterministic gate-support metadata
// =============================================================================

#[test]
fn directional_gate_support_entries_are_deterministic() {
    let topology = builder_with_qubits(3)
        .undirected_edge(q(0), q(1))
        .expect("edge 0-1 must succeed")
        .undirected_edge(q(1), q(2))
        .expect("edge 1-2 must succeed")
        .supported_gate("zz", q(1), q(2))
        .expect("ZZ support must succeed")
        .supported_gate("cx", q(0), q(1))
        .expect("CX support must succeed")
        .unsupported_gate("cx", q(1), q(0))
        .expect("reverse CX rejection must succeed")
        .build()
        .expect("topology must build");

    let first = topology
        .gate_support_entries()
        .map(|(gate, source, target, properties)| {
            (
                gate.to_owned(),
                source,
                target,
                properties.supported,
            )
        })
        .collect::<Vec<_>>();

    let second = topology
        .gate_support_entries()
        .map(|(gate, source, target, properties)| {
            (
                gate.to_owned(),
                source,
                target,
                properties.supported,
            )
        })
        .collect::<Vec<_>>();

    assert_eq!(first, second);

    assert_eq!(
        first,
        vec![
            ("cx".to_string(), q(0), q(1), true),
            ("cx".to_string(), q(1), q(0), false),
            ("zz".to_string(), q(1), q(2), true),
        ]
    );
}

// =============================================================================
// Directed topology + bidirectional topology comparison
// =============================================================================

#[test]
fn directed_and_undirected_edges_have_distinct_semantics() {
    let directed = one_way_topology();

    let undirected = builder_with_qubits(2)
        .undirected_edge(q(0), q(1))
        .expect("undirected edge must succeed")
        .build()
        .expect("undirected topology must build");

    assert!(directed.is_adjacent(q(0), q(1)));
    assert!(!directed.is_adjacent(q(1), q(0)));

    assert!(undirected.is_adjacent(q(0), q(1)));
    assert!(undirected.is_adjacent(q(1), q(0)));

    assert!(!directed.is_bidirectionally_adjacent(
        q(0),
        q(1)
    ));

    assert!(undirected.is_bidirectionally_adjacent(
        q(0),
        q(1)
    ));
}

#[test]
fn explicit_gate_direction_can_be_more_restrictive_than_undirected_hardware() {
    let topology = builder_with_qubits(2)
        .undirected_edge(q(0), q(1))
        .expect("edge must succeed")
        .supported_gate("cx", q(0), q(1))
        .expect("forward CX support must succeed")
        .unsupported_gate("cx", q(1), q(0))
        .expect("reverse CX rejection must succeed")
        .build()
        .expect("topology must build");

    // The physical connection itself is bidirectional.
    assert!(topology.is_bidirectionally_adjacent(
        q(0),
        q(1)
    ));

    // The gate is not.
    assert!(topology.supports_gate(
        "cx",
        q(0),
        q(1)
    ));

    assert!(!topology.supports_gate(
        "cx",
        q(1),
        q(0)
    ));
}

// =============================================================================
// Invalid topology construction
// =============================================================================

#[test]
fn directed_edge_rejects_self_loop() {
    let result = builder_with_qubits(1)
        .directed_edge(q(0), q(0));

    assert!(
        matches!(
            result,
            Err(RoutingError::InvalidTopology(_))
        ),
        "expected InvalidTopology, got {result:?}"
    );
}

#[test]
fn directed_edge_rejects_unknown_source() {
    let result = builder_with_qubits(1)
        .directed_edge(q(99), q(0));

    assert!(
        matches!(
            result,
            Err(RoutingError::InvalidPhysicalQubit(_))
        ),
        "expected InvalidPhysicalQubit, got {result:?}"
    );
}

#[test]
fn directed_edge_rejects_unknown_target() {
    let result = builder_with_qubits(1)
        .directed_edge(q(0), q(99));

    assert!(
        matches!(
            result,
            Err(RoutingError::InvalidPhysicalQubit(_))
        ),
        "expected InvalidPhysicalQubit, got {result:?}"
    );
}

#[test]
fn duplicate_directed_edge_is_rejected() {
    let result = builder_with_qubits(2)
        .directed_edge(q(0), q(1))
        .expect("first directed edge must succeed")
        .directed_edge(q(0), q(1))
        .expect("builder may collect duplicate")
        .build();

    assert!(
        matches!(
            result,
            Err(RoutingError::InvalidTopology(_))
        ),
        "expected InvalidTopology, got {result:?}"
    );
}

// =============================================================================
// Regression guards for routing semantics
// =============================================================================

#[test]
fn reverse_gate_is_not_made_legal_by_structural_connection_alone() {
    let topology = builder_with_qubits(2)
        .directed_edge(q(0), q(1))
        .expect("edge must succeed")
        .supported_gate("cx", q(0), q(1))
        .expect("forward CX must succeed")
        .build()
        .expect("topology must build");

    // This assertion is the critical regression guard for the routing layer.
    //
    // A routing algorithm must never use:
    //
    //     has_connection(a, b)
    //
    // as a substitute for:
    //
    //     supports_gate(gate, a, b)
    //
    // because the latter contains operation-direction semantics.
    assert!(topology.has_connection(q(1), q(0)));
    assert!(!topology.supports_gate(
        "cx",
        q(1),
        q(0)
    ));
}

#[test]
fn directed_path_and_gate_direction_agree() {
    let topology = builder_with_qubits(3)
        .directed_edge(q(0), q(1))
        .expect("edge 0 -> 1 must succeed")
        .directed_edge(q(1), q(2))
        .expect("edge 1 -> 2 must succeed")
        .supported_gate("cx", q(0), q(1))
        .expect("CX 0 -> 1 must succeed")
        .supported_gate("cx", q(1), q(2))
        .expect("CX 1 -> 2 must succeed")
        .build()
        .expect("topology must build");

    let finder = PathFinder::new();

    let path = finder
        .shortest_path(&topology, q(0), q(2))
        .expect("0 -> 2 path must exist");

    assert_eq!(
        path.vertices(),
        &[q(0), q(1), q(2)]
    );

    assert!(topology.supports_gate(
        "cx",
        q(0),
        q(1)
    ));

    assert!(topology.supports_gate(
        "cx",
        q(1),
        q(2)
    ));

    assert!(!topology.supports_gate(
        "cx",
        q(1),
        q(0)
    ));

    assert!(!topology.supports_gate(
        "cx",
        q(2),
        q(1)
    ));
}

#[test]
fn directed_topology_does_not_allow_reverse_path_even_when_structurally_connected() {
    let topology = builder_with_qubits(3)
        .directed_edge(q(0), q(1))
        .expect("edge 0 -> 1 must succeed")
        .directed_edge(q(1), q(2))
        .expect("edge 1 -> 2 must succeed")
        .build()
        .expect("topology must build");

    assert!(topology.is_connected());

    let finder = PathFinder::new();

    assert!(
        finder
            .shortest_path(&topology, q(0), q(2))
            .is_ok()
    );

    assert!(
        finder
            .shortest_path(&topology, q(2), q(0))
            .is_err()
    );
}