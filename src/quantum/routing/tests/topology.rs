//! Zamani Quantum Routing — Topology Test Suite
//!
//! `src/quantum/routing/tests/topology.rs`
//!
//! # Purpose
//!
//! Production-level tests for `routing::topology`.
//!
//! This file verifies the complete topology contract without depending on
//! routing algorithms, layout algorithms, mapping implementations, or the
//! compiler IR.
//!
//! The tests intentionally exercise the public topology API exactly as later
//! routing modules are expected to consume it.
//!
//! # Covered contract
//!
//! - empty-topology rejection;
//! - physical-qubit registration;
//! - duplicate-qubit rejection;
//! - missing-edge endpoint rejection;
//! - self-loop rejection;
//! - duplicate-edge rejection;
//! - undirected connectivity;
//! - directed connectivity;
//! - direction-sensitive neighbors;
//! - direction-insensitive graph neighbors;
//! - adjacency semantics;
//! - bidirectional adjacency;
//! - deterministic qubit iteration;
//! - deterministic edge iteration;
//! - line topology;
//! - ring topology;
//! - grid topology;
//! - isolated topology;
//! - connected-component analysis;
//! - degree analysis;
//! - incoming/outgoing degree;
//! - physical-qubit availability;
//! - unavailable qubits;
//! - unavailable edges;
//! - gate-specific support;
//! - directional gate support;
//! - explicitly unsupported gates;
//! - unsupported gates on non-adjacent qubits;
//! - gate validation;
//! - invalid gate names;
//! - physical-pair validation;
//! - topology metadata;
//! - topology/device/provider identity;
//! - edge properties;
//! - qubit properties;
//! - gate properties;
//! - calibration validation;
//! - deterministic topology construction;
//! - structural versus executable connectivity;
//! - validation after construction;
//! - large-but-reasonable topology construction;
//! - overflow-safe grid construction.
//!
//! # Integration contract
//!
//! This test file intentionally imports only:
//!
//! ```text
//! crate::quantum::routing::errors
//! crate::quantum::routing::topology
//! crate::quantum::routing::types
//! ```
//!
//! It does not import:
//!
//! - `router.rs`;
//! - `layout.rs`;
//! - `mapping.rs`;
//! - `path.rs`;
//! - `algorithms/*`;
//! - `transpiler.rs`;
//! - compiler IR;
//! - hardware providers.
//!
//! This keeps topology tests independent and prevents circular test
//! dependencies.
//!
//! # Production invariant
//!
//! A topology must be a trustworthy source of physical connectivity truth.
//! Tests therefore distinguish:
//!
//! 1. physical graph connectivity;
//! 2. directed structural reachability;
//! 3. gate-specific executability;
//! 4. runtime availability.
//!
//! These concepts must never be conflated.
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
//! topology.rs
//!      │
//!      ▼
//! topology contract
//!      │
//!      ├── mapping.rs
//!      ├── path.rs
//!      ├── candidates.rs
//!      ├── layout.rs
//!      ├── algorithms/*
//!      ├── router.rs
//!      └── verification.rs
//! ```
//!
//! The tests in this file therefore establish the foundation on which those
//! later modules rely.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::collections::BTreeMap;
use std::time::Duration;

use crate::quantum::routing::errors::RoutingError;
use crate::quantum::routing::topology::{
    GateProperties,
    PhysicalQubitProperties,
    PhysicalTopology,
    TopologyBuilder,
    TopologyMetadata,
    TwoQubitProperties,
};
use crate::quantum::routing::types::{
    EdgeDirection,
    PhysicalEdge,
    PhysicalQubitId,
};

// =============================================================================
// Test helpers
// =============================================================================

fn q(index: usize) -> PhysicalQubitId {
    PhysicalQubitId::new(index)
}

fn available_qubit() -> PhysicalQubitProperties {
    PhysicalQubitProperties::default()
}

fn unavailable_qubit() -> PhysicalQubitProperties {
    PhysicalQubitProperties {
        available: false,
        ..PhysicalQubitProperties::default()
    }
}

fn basic_builder(count: usize) -> TopologyBuilder {
    let mut builder = TopologyBuilder::named("test-topology");

    for index in 0..count {
        builder = builder
            .qubit(q(index), available_qubit())
            .expect("test qubit registration must succeed");
    }

    builder
}

fn assert_routing_error(result: Result<(), RoutingError>) {
    assert!(
        result.is_err(),
        "expected routing operation to fail, but it succeeded"
    );
}

// =============================================================================
// Construction
// =============================================================================

#[test]
fn rejects_empty_topology() {
    let result = PhysicalTopology::isolated(0);

    assert!(
        matches!(result, Err(RoutingError::EmptyTopology)),
        "expected EmptyTopology, got {result:?}"
    );
}

#[test]
fn builder_rejects_empty_topology() {
    let result = TopologyBuilder::new().build();

    assert!(
        matches!(result, Err(RoutingError::EmptyTopology)),
        "expected EmptyTopology, got {result:?}"
    );
}

#[test]
fn registers_physical_qubits() {
    let topology = PhysicalTopology::isolated(4)
        .expect("four isolated qubits should be valid");

    assert_eq!(topology.qubit_count(), 4);
    assert!(topology.contains(q(0)));
    assert!(topology.contains(q(1)));
    assert!(topology.contains(q(2)));
    assert!(topology.contains(q(3)));
    assert!(!topology.contains(q(4)));
}

#[test]
fn builder_rejects_duplicate_physical_qubits() {
    let result = TopologyBuilder::named("duplicate-qubit")
        .add_qubit(q(0))
        .expect("first qubit should succeed")
        .add_qubit(q(0));

    assert!(
        matches!(result, Err(RoutingError::InvalidTopology(_))),
        "expected duplicate-qubit topology error, got {result:?}"
    );
}

#[test]
fn builder_rejects_missing_edge_endpoint() {
    let result = TopologyBuilder::named("missing-endpoint")
        .add_qubit(q(0))
        .expect("qubit 0 should succeed")
        .undirected_edge(q(0), q(1));

    assert!(
        matches!(result, Err(RoutingError::InvalidPhysicalQubit(_))),
        "expected InvalidPhysicalQubit, got {result:?}"
    );
}

#[test]
fn builder_rejects_self_loop() {
    let result = TopologyBuilder::named("self-loop")
        .add_qubit(q(0))
        .expect("qubit 0 should succeed")
        .undirected_edge(q(0), q(0));

    assert!(
        matches!(result, Err(RoutingError::InvalidTopology(_))),
        "expected InvalidTopology, got {result:?}"
    );
}

#[test]
fn builder_rejects_duplicate_edges_at_build_time() {
    let result = basic_builder(2)
        .undirected_edge(q(0), q(1))
        .expect("first edge should succeed")
        .undirected_edge(q(0), q(1))
        .expect("builder may collect the edge")
        .build();

    assert!(
        matches!(result, Err(RoutingError::InvalidTopology(_))),
        "expected duplicate-edge rejection, got {result:?}"
    );
}

// =============================================================================
// Basic undirected topology
// =============================================================================

#[test]
fn undirected_edge_is_adjacent_in_both_directions() {
    let topology = basic_builder(2)
        .undirected_edge(q(0), q(1))
        .expect("edge should succeed")
        .build()
        .expect("topology should build");

    assert!(topology.is_adjacent(q(0), q(1)));
    assert!(topology.is_adjacent(q(1), q(0)));
    assert!(topology.is_bidirectionally_adjacent(q(0), q(1)));
    assert!(topology.is_bidirectionally_adjacent(q(1), q(0)));
    assert!(topology.has_connection(q(0), q(1)));
    assert!(topology.has_connection(q(1), q(0)));
}

#[test]
fn undirected_neighbors_are_deterministic() {
    let topology = basic_builder(4)
        .undirected_edge(q(0), q(3))
        .expect("edge should succeed")
        .undirected_edge(q(0), q(1))
        .expect("edge should succeed")
        .undirected_edge(q(0), q(2))
        .expect("edge should succeed")
        .build()
        .expect("topology should build");

    assert_eq!(
        topology.neighbors(q(0)),
        vec![q(1), q(2), q(3)]
    );

    assert_eq!(
        topology.undirected_neighbors(q(0)),
        vec![q(1), q(2), q(3)]
    );
}

#[test]
fn self_adjacency_is_always_false() {
    let topology = PhysicalTopology::line(3)
        .expect("line topology should build");

    assert!(!topology.is_adjacent(q(0), q(0)));
    assert!(!topology.is_bidirectionally_adjacent(q(0), q(0)));
    assert!(!topology.has_connection(q(0), q(0)));
}

#[test]
fn non_adjacent_qubits_are_not_adjacent() {
    let topology = PhysicalTopology::line(4)
        .expect("line topology should build");

    assert!(!topology.is_adjacent(q(0), q(2)));
    assert!(!topology.is_adjacent(q(2), q(0)));
    assert!(topology.has_connection(q(0), q(1)));
    assert!(!topology.has_connection(q(0), q(2)));
}

// =============================================================================
// Directed topology
// =============================================================================

#[test]
fn directed_edge_is_direction_sensitive() {
    let topology = basic_builder(2)
        .directed_edge(q(0), q(1))
        .expect("directed edge should succeed")
        .build()
        .expect("topology should build");

    assert!(topology.is_adjacent(q(0), q(1)));
    assert!(!topology.is_adjacent(q(1), q(0)));

    assert!(!topology.is_bidirectionally_adjacent(q(0), q(1)));

    assert!(topology.has_connection(q(0), q(1)));
    assert!(topology.has_connection(q(1), q(0)));
}

#[test]
fn directed_edge_has_directional_neighbors() {
    let topology = basic_builder(3)
        .directed_edge(q(0), q(1))
        .expect("edge should succeed")
        .directed_edge(q(1), q(2))
        .expect("edge should succeed")
        .build()
        .expect("topology should build");

    assert_eq!(topology.neighbors(q(0)), vec![q(1)]);
    assert_eq!(topology.neighbors(q(1)), vec![q(2)]);
    assert!(topology.neighbors(q(2)).is_empty());
}

#[test]
fn directed_graph_keeps_undirected_graph_analysis() {
    let topology = basic_builder(3)
        .directed_edge(q(0), q(1))
        .expect("edge should succeed")
        .directed_edge(q(1), q(2))
        .expect("edge should succeed")
        .build()
        .expect("topology should build");

    assert!(topology.is_connected());

    assert_eq!(
        topology.connected_components(),
        vec![vec![q(0), q(1), q(2)]]
    );
}

#[test]
fn directed_degrees_are_direction_sensitive() {
    let topology = basic_builder(3)
        .directed_edge(q(0), q(1))
        .expect("edge should succeed")
        .directed_edge(q(2), q(1))
        .expect("edge should succeed")
        .build()
        .expect("topology should build");

    assert_eq!(topology.outgoing_degree(q(0)), 1);
    assert_eq!(topology.outgoing_degree(q(1)), 0);
    assert_eq!(topology.outgoing_degree(q(2)), 1);

    assert_eq!(topology.incoming_degree(q(0)), 0);
    assert_eq!(topology.incoming_degree(q(1)), 2);
    assert_eq!(topology.incoming_degree(q(2)), 0);
}

#[test]
fn edge_direction_is_preserved() {
    let topology = basic_builder(2)
        .directed_edge(q(0), q(1))
        .expect("edge should succeed")
        .build()
        .expect("topology should build");

    let edge = topology
        .edge(q(0), q(1))
        .expect("edge should exist");

    assert_eq!(edge.a(), q(0));
    assert_eq!(edge.b(), q(1));
    assert_eq!(edge.direction(), EdgeDirection::Forward);
}

#[test]
fn undirected_edge_preserves_undirected_direction() {
    let topology = basic_builder(2)
        .undirected_edge(q(0), q(1))
        .expect("edge should succeed")
        .build()
        .expect("topology should build");

    let edge = topology
        .edge(q(0), q(1))
        .expect("edge should exist");

    assert_eq!(edge.direction(), EdgeDirection::Undirected);
}

// =============================================================================
// Topology constructors
// =============================================================================

#[test]
fn line_topology_has_expected_shape() {
    let topology = PhysicalTopology::line(5)
        .expect("line topology should build");

    assert_eq!(topology.qubit_count(), 5);
    assert_eq!(topology.edge_count(), 4);

    assert_eq!(topology.neighbors(q(0)), vec![q(1)]);
    assert_eq!(
        topology.neighbors(q(2)),
        vec![q(1), q(3)]
    );
    assert_eq!(topology.neighbors(q(4)), vec![q(3)]);

    assert!(topology.is_connected());
}

#[test]
fn line_topology_rejects_zero_qubits() {
    let result = PhysicalTopology::line(0);

    assert!(
        matches!(result, Err(RoutingError::EmptyTopology)),
        "expected EmptyTopology, got {result:?}"
    );
}

#[test]
fn ring_topology_has_expected_shape() {
    let topology = PhysicalTopology::ring(5)
        .expect("ring topology should build");

    assert_eq!(topology.qubit_count(), 5);
    assert_eq!(topology.edge_count(), 5);
    assert!(topology.is_connected());

    for index in 0..5 {
        assert_eq!(topology.degree(q(index)), 2);
    }
}

#[test]
fn one_qubit_ring_has_no_self_loop() {
    let topology = PhysicalTopology::ring(1)
        .expect("one-qubit ring should build");

    assert_eq!(topology.qubit_count(), 1);
    assert_eq!(topology.edge_count(), 0);
    assert!(!topology.is_adjacent(q(0), q(0)));
    assert!(topology.is_connected());
}

#[test]
fn ring_topology_rejects_zero_qubits() {
    let result = PhysicalTopology::ring(0);

    assert!(
        matches!(result, Err(RoutingError::EmptyTopology)),
        "expected EmptyTopology, got {result:?}"
    );
}

#[test]
fn grid_topology_has_expected_shape() {
    let topology = PhysicalTopology::grid(2, 3)
        .expect("2x3 grid should build");

    assert_eq!(topology.qubit_count(), 6);
    assert_eq!(topology.edge_count(), 7);
    assert!(topology.is_connected());

    assert_eq!(topology.degree(q(0)), 2);
    assert_eq!(topology.degree(q(1)), 3);
    assert_eq!(topology.degree(q(2)), 2);
    assert_eq!(topology.degree(q(3)), 2);
    assert_eq!(topology.degree(q(4)), 3);
    assert_eq!(topology.degree(q(5)), 2);
}

#[test]
fn grid_topology_uses_deterministic_row_major_ids() {
    let topology = PhysicalTopology::grid(2, 3)
        .expect("grid should build");

    assert_eq!(
        topology.qubits().collect::<Vec<_>>(),
        vec![q(0), q(1), q(2), q(3), q(4), q(5)]
    );

    assert_eq!(topology.neighbors(q(0)), vec![q(1), q(3)]);
    assert_eq!(
        topology.neighbors(q(4)),
        vec![q(1), q(3), q(5)]
    );
}

#[test]
fn grid_topology_rejects_zero_dimension() {
    let zero_rows = PhysicalTopology::grid(0, 3);
    let zero_columns = PhysicalTopology::grid(3, 0);

    assert!(matches!(
        zero_rows,
        Err(RoutingError::EmptyTopology)
    ));

    assert!(matches!(
        zero_columns,
        Err(RoutingError::EmptyTopology)
    ));
}

#[test]
fn isolated_topology_has_one_component_per_qubit() {
    let topology = PhysicalTopology::isolated(4)
        .expect("isolated topology should build");

    assert!(!topology.is_connected());
    assert_eq!(
        topology.connected_components(),
        vec![
            vec![q(0)],
            vec![q(1)],
            vec![q(2)],
            vec![q(3)],
        ]
    );
}

// =============================================================================
// Connected components
// =============================================================================

#[test]
fn disconnected_topology_reports_components_deterministically() {
    let topology = basic_builder(6)
        .undirected_edge(q(0), q(1))
        .expect("edge should succeed")
        .undirected_edge(q(1), q(2))
        .expect("edge should succeed")
        .undirected_edge(q(3), q(4))
        .expect("edge should succeed")
        .build()
        .expect("topology should build");

    assert!(!topology.is_connected());

    assert_eq!(
        topology.connected_components(),
        vec![
            vec![q(0), q(1), q(2)],
            vec![q(3), q(4)],
            vec![q(5)],
        ]
    );
}

#[test]
fn connected_component_order_is_stable() {
    let topology = basic_builder(5)
        .undirected_edge(q(3), q(4))
        .expect("edge should succeed")
        .undirected_edge(q(0), q(2))
        .expect("edge should succeed")
        .build()
        .expect("topology should build");

    assert_eq!(
        topology.connected_components(),
        vec![
            vec![q(0), q(2)],
            vec![q(1)],
            vec![q(3), q(4)],
        ]
    );
}

// =============================================================================
// Determinism
// =============================================================================

#[test]
fn physical_qubit_iteration_is_deterministic() {
    let topology = basic_builder(5)
        .build()
        .expect("topology should build");

    let first = topology.qubits().collect::<Vec<_>>();
    let second = topology.qubits().collect::<Vec<_>>();

    assert_eq!(first, second);
    assert_eq!(
        first,
        vec![q(0), q(1), q(2), q(3), q(4)]
    );
}

#[test]
fn edge_iteration_is_deterministic() {
    let topology = basic_builder(4)
        .undirected_edge(q(2), q(3))
        .expect("edge should succeed")
        .undirected_edge(q(0), q(1))
        .expect("edge should succeed")
        .undirected_edge(q(1), q(2))
        .expect("edge should succeed")
        .build()
        .expect("topology should build");

    let first = topology.edges().copied().collect::<Vec<_>>();
    let second = topology.edges().copied().collect::<Vec<_>>();

    assert_eq!(first, second);
    assert_eq!(first.len(), 3);
}

// =============================================================================
// Availability
// =============================================================================

#[test]
fn all_new_qubits_are_available_by_default() {
    let topology = PhysicalTopology::isolated(3)
        .expect("topology should build");

    assert_eq!(
        topology.available_qubits().collect::<Vec<_>>(),
        vec![q(0), q(1), q(2)]
    );

    assert!(topology.unavailable_qubits().next().is_none());
}

#[test]
fn unavailable_qubit_is_not_routing_available() {
    let mut builder = TopologyBuilder::named("availability");

    builder = builder
        .qubit(q(0), unavailable_qubit())
        .expect("qubit 0 should succeed")
        .add_qubit(q(1))
        .expect("qubit 1 should succeed");

    let topology = builder
        .undirected_edge(q(0), q(1))
        .expect("edge should succeed")
        .build()
        .expect("topology should build");

    assert!(!topology.is_available(q(0)));
    assert!(topology.is_available(q(1)));

    assert_eq!(
        topology.unavailable_qubits().collect::<Vec<_>>(),
        vec![q(0)]
    );

    assert_eq!(
        topology.available_qubits().collect::<Vec<_>>(),
        vec![q(1)]
    );

    assert!(!topology.supports_gate("cx", q(0), q(1)));
}

#[test]
fn qubit_availability_can_be_changed_without_changing_graph() {
    let mut topology = PhysicalTopology::line(3)
        .expect("line topology should build");

    assert!(topology.is_available(q(1)));
    assert!(topology.is_adjacent(q(0), q(1)));
    assert!(topology.is_adjacent(q(1), q(2)));

    topology
        .set_qubit_available(q(1), false)
        .expect("existing qubit should be mutable");

    assert!(!topology.is_available(q(1)));
    assert!(topology.is_adjacent(q(0), q(1)));
    assert!(topology.is_adjacent(q(1), q(2)));

    assert!(!topology.supports_gate("cx", q(0), q(1)));
}

#[test]
fn changing_unknown_qubit_availability_is_rejected() {
    let mut topology = PhysicalTopology::line(2)
        .expect("line topology should build");

    let result = topology.set_qubit_available(q(9), false);

    assert!(
        matches!(result, Err(RoutingError::InvalidPhysicalQubit(_))),
        "expected InvalidPhysicalQubit, got {result:?}"
    );
}

#[test]
fn edge_availability_can_be_disabled_without_removing_edge() {
    let mut topology = PhysicalTopology::line(2)
        .expect("line topology should build");

    assert!(topology.has_connection(q(0), q(1)));
    assert!(topology.supports_gate("cx", q(0), q(1)));

    topology
        .set_edge_available(q(0), q(1), false)
        .expect("existing edge should be mutable");

    assert!(topology.has_connection(q(0), q(1)));
    assert!(!topology.supports_gate("cx", q(0), q(1)));
}

#[test]
fn changing_unknown_edge_availability_is_rejected() {
    let mut topology = PhysicalTopology::line(2)
        .expect("line topology should build");

    let result = topology.set_edge_available(q(0), q(9), false);

    assert!(
        matches!(result, Err(RoutingError::InvalidTopology(_))),
        "expected InvalidTopology, got {result:?}"
    );
}

// =============================================================================
// Gate-specific support
// =============================================================================

#[test]
fn structural_adjacency_does_not_require_explicit_gate_registration() {
    let topology = PhysicalTopology::line(2)
        .expect("line topology should build");

    assert!(topology.is_adjacent(q(0), q(1)));
    assert!(!topology.has_explicit_gate_support(
        "cx",
        q(0),
        q(1)
    ));

    assert!(topology.supports_gate("cx", q(0), q(1)));
}

#[test]
fn explicitly_supported_gate_is_reported() {
    let topology = basic_builder(2)
        .undirected_edge(q(0), q(1))
        .expect("edge should succeed")
        .supported_gate("cx", q(0), q(1))
        .expect("gate support should succeed")
        .build()
        .expect("topology should build");

    assert!(topology.has_explicit_gate_support(
        "cx",
        q(0),
        q(1)
    ));

    assert!(topology.supports_gate("cx", q(0), q(1)));
}

#[test]
fn explicitly_unsupported_gate_is_not_executable() {
    let topology = basic_builder(2)
        .undirected_edge(q(0), q(1))
        .expect("edge should succeed")
        .unsupported_gate("cx", q(0), q(1))
        .expect("unsupported gate entry should succeed")
        .build()
        .expect("topology should build");

    assert!(topology.has_explicit_gate_support(
        "cx",
        q(0),
        q(1)
    ));

    assert!(!topology.supports_gate("cx", q(0), q(1)));
}

#[test]
fn gate_support_is_direction_sensitive() {
    let topology = basic_builder(2)
        .undirected_edge(q(0), q(1))
        .expect("edge should succeed")
        .supported_gate("cx", q(0), q(1))
        .expect("forward CX should succeed")
        .unsupported_gate("cx", q(1), q(0))
        .expect("reverse CX should succeed")
        .build()
        .expect("topology should build");

    assert!(topology.supports_gate("cx", q(0), q(1)));
    assert!(!topology.supports_gate("cx", q(1), q(0)));

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
fn directed_connectivity_can_be_combined_with_gate_support() {
    let topology = basic_builder(2)
        .directed_edge(q(0), q(1))
        .expect("edge should succeed")
        .supported_gate("cx", q(0), q(1))
        .expect("forward CX should succeed")
        .build()
        .expect("topology should build");

    assert!(topology.supports_gate("cx", q(0), q(1)));
    assert!(!topology.supports_gate("cx", q(1), q(0)));
}

#[test]
fn gate_support_on_non_adjacent_qubits_is_rejected() {
    let result = basic_builder(3)
        .undirected_edge(q(0), q(1))
        .expect("edge should succeed")
        .supported_gate("cx", q(0), q(2))
        .expect_err("non-adjacent gate support must be rejected");

    assert!(
        matches!(result, RoutingError::InvalidTopology(_)),
        "expected InvalidTopology, got {result:?}"
    );
}

#[test]
fn gate_support_rejects_empty_gate_name() {
    let result = basic_builder(2)
        .undirected_edge(q(0), q(1))
        .expect("edge should succeed")
        .supported_gate("   ", q(0), q(1));

    assert!(
        matches!(result, Err(RoutingError::UnsupportedGate(_))),
        "expected UnsupportedGate, got {result:?}"
    );
}

#[test]
fn gate_names_are_normalized_for_lookup() {
    let topology = basic_builder(2)
        .undirected_edge(q(0), q(1))
        .expect("edge should succeed")
        .supported_gate(" CX ", q(0), q(1))
        .expect("gate support should succeed")
        .build()
        .expect("topology should build");

    assert!(topology.supports_gate("cx", q(0), q(1)));
    assert!(topology.supports_gate("CX", q(0), q(1)));
    assert!(topology.has_explicit_gate_support(
        "cX",
        q(0),
        q(1)
    ));
}

#[test]
fn gate_support_entries_are_deterministic() {
    let topology = basic_builder(3)
        .undirected_edge(q(0), q(1))
        .expect("edge should succeed")
        .undirected_edge(q(1), q(2))
        .expect("edge should succeed")
        .supported_gate("zz", q(1), q(2))
        .expect("zz should succeed")
        .supported_gate("cx", q(0), q(1))
        .expect("cx should succeed")
        .build()
        .expect("topology should build");

    let entries = topology
        .gate_support_entries()
        .map(|(gate, source, target, properties)| {
            (gate.to_owned(), source, target, properties.supported)
        })
        .collect::<Vec<_>>();

    assert_eq!(
        entries,
        vec![
            ("cx".to_string(), q(0), q(1), true),
            ("zz".to_string(), q(1), q(2), true),
        ]
    );
}

// =============================================================================
// Gate validation
// =============================================================================

#[test]
fn validate_gate_accepts_supported_operation() {
    let topology = basic_builder(2)
        .undirected_edge(q(0), q(1))
        .expect("edge should succeed")
        .supported_gate("cx", q(0), q(1))
        .expect("gate support should succeed")
        .build()
        .expect("topology should build");

    topology
        .validate_gate("cx", q(0), q(1))
        .expect("supported gate should validate");
}

#[test]
fn validate_gate_rejects_empty_gate_name() {
    let topology = PhysicalTopology::line(2)
        .expect("line topology should build");

    let result = topology.validate_gate("", q(0), q(1));

    assert!(
        matches!(result, Err(RoutingError::UnsupportedGate(_))),
        "expected UnsupportedGate, got {result:?}"
    );
}

#[test]
fn validate_gate_rejects_wrong_direction() {
    let topology = basic_builder(2)
        .directed_edge(q(0), q(1))
        .expect("edge should succeed")
        .supported_gate("cx", q(0), q(1))
        .expect("gate support should succeed")
        .build()
        .expect("topology should build");

    let result = topology.validate_gate("cx", q(1), q(0));

    assert!(
        matches!(
            result,
            Err(RoutingError::UnsupportedDirectedGate { .. })
        ),
        "expected UnsupportedDirectedGate, got {result:?}"
    );
}

#[test]
fn validate_gate_rejects_unavailable_qubit() {
    let mut topology = PhysicalTopology::line(2)
        .expect("line topology should build");

    topology
        .set_qubit_available(q(1), false)
        .expect("qubit should exist");

    let result = topology.validate_gate("cx", q(0), q(1));

    assert!(
        matches!(
            result,
            Err(RoutingError::UnsupportedDirectedGate { .. })
        ),
        "expected unsupported operation, got {result:?}"
    );
}

#[test]
fn validate_gate_rejects_non_adjacent_qubits() {
    let topology = PhysicalTopology::line(3)
        .expect("line topology should build");

    let result = topology.validate_gate("cx", q(0), q(2));

    assert!(
        matches!(
            result,
            Err(RoutingError::UnsupportedDirectedGate { .. })
        ),
        "expected UnsupportedDirectedGate, got {result:?}"
    );
}

#[test]
fn validate_physical_pair_rejects_unknown_source() {
    let topology = PhysicalTopology::line(2)
        .expect("line topology should build");

    let result = topology.validate_physical_pair(q(9), q(1));

    assert!(
        matches!(result, Err(RoutingError::InvalidPhysicalQubit(_))),
        "expected InvalidPhysicalQubit, got {result:?}"
    );
}

#[test]
fn validate_physical_pair_rejects_unknown_target() {
    let topology = PhysicalTopology::line(2)
        .expect("line topology should build");

    let result = topology.validate_physical_pair(q(0), q(9));

    assert!(
        matches!(result, Err(RoutingError::InvalidPhysicalQubit(_))),
        "expected InvalidPhysicalQubit, got {result:?}"
    );
}

#[test]
fn validate_physical_pair_rejects_same_qubit() {
    let topology = PhysicalTopology::line(2)
        .expect("line topology should build");

    let result = topology.validate_physical_pair(q(0), q(0));

    assert!(
        matches!(result, Err(RoutingError::InvalidTopology(_))),
        "expected InvalidTopology, got {result:?}"
    );
}

// =============================================================================
// Properties
// =============================================================================

#[test]
fn default_qubit_properties_are_valid() {
    let properties = PhysicalQubitProperties::default();

    properties
        .validate()
        .expect("default qubit properties must validate");

    assert!(properties.available);
    assert!(properties.t1.is_none());
    assert!(properties.t2.is_none());
    assert!(properties.readout_error.is_none());
    assert!(properties.frequency_hz.is_none());
    assert!(properties.calibration_id.is_none());
}

#[test]
fn physical_qubit_properties_are_preserved() {
    let properties = PhysicalQubitProperties {
        available: true,
        t1: Some(Duration::from_micros(100)),
        t2: Some(Duration::from_micros(80)),
        readout_error: Some(0.01),
        frequency_hz: Some(5.0e9),
        calibration_id: Some("cal-q0".to_string()),
    };

    let mut builder = TopologyBuilder::named("properties");

    builder = builder
        .qubit(q(0), properties.clone())
        .expect("qubit should succeed");

    let topology = builder
        .build()
        .expect("topology should build");

    assert_eq!(
        topology.qubit_properties(q(0)),
        Some(&properties)
    );
}

#[test]
fn edge_properties_are_preserved() {
    let properties = TwoQubitProperties {
        available: true,
        duration: Some(Duration::from_nanos(250)),
        error_rate: Some(0.01),
        fidelity: None,
        calibration_id: Some("cal-e0-1".to_string()),
    };

    let topology = basic_builder(2)
        .undirected_edge_with_properties(
            q(0),
            q(1),
            properties.clone(),
        )
        .expect("edge should succeed")
        .build()
        .expect("topology should build");

    assert_eq!(
        topology.edge_properties(q(0), q(1)),
        Some(&properties)
    );

    assert_eq!(
        topology.edge_properties(q(1), q(0)),
        Some(&properties)
    );
}

#[test]
fn gate_properties_are_preserved() {
    let properties = GateProperties {
        supported: true,
        duration: Some(Duration::from_nanos(300)),
        error_rate: Some(0.005),
        fidelity: Some(0.995),
        calibration_id: Some("cx-cal-0-1".to_string()),
    };

    let topology = basic_builder(2)
        .undirected_edge(q(0), q(1))
        .expect("edge should succeed")
        .gate(
            "cx",
            q(0),
            q(1),
            properties.clone(),
        )
        .expect("gate properties should succeed")
        .build()
        .expect("topology should build");

    assert_eq!(
        topology.gate_properties("cx", q(0), q(1)),
        Some(&properties)
    );
}

#[test]
fn invalid_negative_probability_is_rejected() {
    let properties = PhysicalQubitProperties {
        readout_error: Some(-0.01),
        ..PhysicalQubitProperties::default()
    };

    let result = properties.validate();

    assert!(
        result.is_err(),
        "negative probability must be rejected"
    );
}

#[test]
fn probability_above_one_is_rejected() {
    let properties = PhysicalQubitProperties {
        readout_error: Some(1.01),
        ..PhysicalQubitProperties::default()
    };

    let result = properties.validate();

    assert!(
        result.is_err(),
        "probability above one must be rejected"
    );
}

#[test]
fn invalid_frequency_is_rejected() {
    let properties = PhysicalQubitProperties {
        frequency_hz: Some(-1.0),
        ..PhysicalQubitProperties::default()
    };

    let result = properties.validate();

    assert!(
        result.is_err(),
        "negative frequency must be rejected"
    );
}

#[test]
fn invalid_gate_probability_is_rejected() {
    let properties = GateProperties {
        supported: true,
        error_rate: Some(1.5),
        ..GateProperties::default()
    };

    let result = properties.validate();

    assert!(
        result.is_err(),
        "gate error rate above one must be rejected"
    );
}

#[test]
fn invalid_edge_probability_is_rejected() {
    let properties = TwoQubitProperties {
        error_rate: Some(-0.1),
        ..TwoQubitProperties::default()
    };

    let result = properties.validate();

    assert!(
        result.is_err(),
        "negative edge error rate must be rejected"
    );
}

// =============================================================================
// Metadata
// =============================================================================

#[test]
fn topology_metadata_is_preserved() {
    let metadata = TopologyMetadata {
        name: "production-device".to_string(),
        provider: Some("zamani".to_string()),
        device: Some("zq-128".to_string()),
        revision: Some("rev-3".to_string()),
        topology_id: Some("zq-128-rev3".to_string()),
    };

    let topology = PhysicalTopology::new(
        metadata.clone(),
        {
            let mut qubits = BTreeMap::new();
            qubits.insert(
                q(0),
                PhysicalQubitProperties::default(),
            );
            qubits.insert(
                q(1),
                PhysicalQubitProperties::default(),
            );
            qubits
        },
        vec![PhysicalEdge::undirected(q(0), q(1))],
    )
    .expect("topology should build");

    assert_eq!(topology.metadata(), &metadata);
    assert_eq!(topology.name(), "production-device");
    assert_eq!(topology.provider(), Some("zamani"));
    assert_eq!(topology.device(), Some("zq-128"));
}

#[test]
fn named_metadata_is_not_empty() {
    let metadata = TopologyMetadata::named("line");

    assert!(!metadata.is_empty());
    assert_eq!(metadata.name, "line");
}

#[test]
fn empty_metadata_reports_empty() {
    let metadata = TopologyMetadata::default();

    assert!(metadata.is_empty());
}

// =============================================================================
// Topology validation
// =============================================================================

#[test]
fn valid_topology_passes_validation() {
    let topology = PhysicalTopology::grid(4, 4)
        .expect("grid topology should build");

    topology
        .validate()
        .expect("valid topology must validate");
}

#[test]
fn validation_remains_valid_after_availability_changes() {
    let mut topology = PhysicalTopology::line(5)
        .expect("line topology should build");

    topology
        .set_qubit_available(q(2), false)
        .expect("qubit should exist");

    topology
        .set_edge_available(q(1), q(2), false)
        .expect("edge should exist");

    topology
        .validate()
        .expect("availability changes must not corrupt topology");
}

#[test]
fn isolated_topology_is_structurally_valid() {
    let topology = PhysicalTopology::isolated(8)
        .expect("isolated topology should build");

    topology
        .validate()
        .expect("isolated topology must validate");

    assert_eq!(topology.edge_count(), 0);
}

// =============================================================================
// Physical edge value-object contract
// =============================================================================

#[test]
fn physical_edge_detects_self_loop() {
    let edge = PhysicalEdge::new(
        q(0),
        q(0),
        EdgeDirection::Undirected,
    );

    assert!(edge.is_self_loop());
}

#[test]
fn physical_edge_other_returns_opposite_endpoint() {
    let edge = PhysicalEdge::undirected(q(2), q(7));

    assert_eq!(edge.other(q(2)), Some(q(7)));
    assert_eq!(edge.other(q(7)), Some(q(2)));
    assert_eq!(edge.other(q(9)), None);
}

#[test]
fn physical_edge_constructor_preserves_endpoints() {
    let edge = PhysicalEdge::new(
        q(7),
        q(2),
        EdgeDirection::Forward,
    );

    assert_eq!(edge.a(), q(7));
    assert_eq!(edge.b(), q(2));
    assert_eq!(edge.direction(), EdgeDirection::Forward);
}

// =============================================================================
// Scale / robustness
// =============================================================================

#[test]
fn moderate_line_topology_scales_without_semantic_loss() {
    let count = 512;

    let topology = PhysicalTopology::line(count)
        .expect("512-qubit line should build");

    assert_eq!(topology.qubit_count(), count);
    assert_eq!(
        topology.edge_count(),
        count - 1
    );

    assert_eq!(topology.degree(q(0)), 1);
    assert_eq!(
        topology.degree(q(count / 2)),
        2
    );
    assert_eq!(
        topology.degree(q(count - 1)),
        1
    );

    assert!(topology.is_connected());
}

#[test]
fn large_grid_has_expected_edge_count() {
    let rows = 16;
    let columns = 16;

    let topology = PhysicalTopology::grid(rows, columns)
        .expect("16x16 grid should build");

    let expected_edges =
        rows * (columns - 1)
            + columns * (rows - 1);

    assert_eq!(topology.qubit_count(), rows * columns);
    assert_eq!(topology.edge_count(), expected_edges);
    assert!(topology.is_connected());
}

#[test]
fn grid_dimension_overflow_is_rejected() {
    let result = PhysicalTopology::grid(
        usize::MAX,
        2,
    );

    assert!(
        result.is_err(),
        "overflowing grid dimensions must be rejected"
    );
}

// =============================================================================
// Structural connectivity versus executable connectivity
// =============================================================================

#[test]
fn structural_connection_can_exist_without_gate_executability() {
    let topology = basic_builder(2)
        .undirected_edge(q(0), q(1))
        .expect("edge should succeed")
        .unsupported_gate("cx", q(0), q(1))
        .expect("unsupported gate entry should succeed")
        .build()
        .expect("topology should build");

    assert!(topology.has_connection(q(0), q(1)));
    assert!(topology.is_adjacent(q(0), q(1)));
    assert!(!topology.supports_gate(
        "cx",
        q(0),
        q(1)
    ));
}

#[test]
fn unavailable_edge_preserves_structural_connection() {
    let mut topology = PhysicalTopology::line(2)
        .expect("line topology should build");

    assert!(topology.has_connection(q(0), q(1)));
    assert!(topology.is_adjacent(q(0), q(1)));

    topology
        .set_edge_available(q(0), q(1), false)
        .expect("edge should exist");

    assert!(topology.has_connection(q(0), q(1)));
    assert!(topology.is_adjacent(q(0), q(1)));
    assert!(!topology.supports_gate(
        "cx",
        q(0),
        q(1)
    ));
}

// =============================================================================
// Public API consistency
// =============================================================================

#[test]
fn qubit_count_matches_qubit_iterator_count() {
    let topology = PhysicalTopology::grid(5, 7)
        .expect("grid topology should build");

    assert_eq!(
        topology.qubit_count(),
        topology.qubits().count()
    );
}

#[test]
fn edge_count_matches_edge_iterator_count() {
    let topology = PhysicalTopology::grid(5, 7)
        .expect("grid topology should build");

    assert_eq!(
        topology.edge_count(),
        topology.edges().count()
    );
}

#[test]
fn unavailable_qubits_are_subset_of_registered_qubits() {
    let mut topology = PhysicalTopology::line(5)
        .expect("line topology should build");

    topology
        .set_qubit_available(q(1), false)
        .expect("qubit should exist");

    topology
        .set_qubit_available(q(3), false)
        .expect("qubit should exist");

    let registered = topology
        .qubits()
        .collect::<Vec<_>>();

    for qubit in topology.unavailable_qubits() {
        assert!(registered.contains(&qubit));
    }
}

#[test]
fn available_and_unavailable_qubits_partition_registered_qubits() {
    let mut topology = PhysicalTopology::line(6)
        .expect("line topology should build");

    topology
        .set_qubit_available(q(1), false)
        .expect("qubit should exist");

    topology
        .set_qubit_available(q(4), false)
        .expect("qubit should exist");

    let available = topology
        .available_qubits()
        .collect::<Vec<_>>();

    let unavailable = topology
        .unavailable_qubits()
        .collect::<Vec<_>>();

    let mut combined = available.clone();
    combined.extend(unavailable.iter().copied());
    combined.sort_unstable();

    assert_eq!(
        combined,
        topology.qubits().collect::<Vec<_>>()
    );

    assert_eq!(available.len() + unavailable.len(), 6);
}

// =============================================================================
// Final invariant test
// =============================================================================

#[test]
fn production_topology_contract_holds_end_to_end() {
    let topology = basic_builder(8)
        .undirected_edge(q(0), q(1))
        .expect("edge should succeed")
        .undirected_edge(q(1), q(2))
        .expect("edge should succeed")
        .undirected_edge(q(2), q(3))
        .expect("edge should succeed")
        .undirected_edge(q(3), q(4))
        .expect("edge should succeed")
        .undirected_edge(q(4), q(5))
        .expect("edge should succeed")
        .undirected_edge(q(5), q(6))
        .expect("edge should succeed")
        .undirected_edge(q(6), q(7))
        .expect("edge should succeed")
        .supported_gate("cx", q(0), q(1))
        .expect("CX support should succeed")
        .supported_gate("cx", q(1), q(2))
        .expect("CX support should succeed")
        .unsupported_gate("cx", q(2), q(3))
        .expect("unsupported CX entry should succeed")
        .build()
        .expect("production topology should build");

    topology
        .validate()
        .expect("production topology must validate");

    assert_eq!(topology.qubit_count(), 8);
    assert_eq!(topology.edge_count(), 7);
    assert!(topology.is_connected());

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
        q(2),
        q(3)
    ));

    assert!(topology.has_connection(q(2), q(3)));
    assert!(topology.is_adjacent(q(2), q(3)));

    topology
        .validate_gate("cx", q(0), q(1))
        .expect("supported CX must validate");

    let invalid = topology.validate_gate(
        "cx",
        q(2),
        q(3),
    );

    assert!(
        invalid.is_err(),
        "explicitly unsupported CX must not validate"
    );
}

// =============================================================================
// Compile-time/API-shape smoke checks
// =============================================================================

#[test]
fn topology_public_api_types_are_constructible() {
    let _: PhysicalQubitId = PhysicalQubitId::new(0);
    let _: EdgeDirection = EdgeDirection::Undirected;
    let _: PhysicalEdge =
        PhysicalEdge::undirected(q(0), q(1));

    let _: PhysicalQubitProperties =
        PhysicalQubitProperties::default();

    let _: TwoQubitProperties =
        TwoQubitProperties::default();

    let _: GateProperties =
        GateProperties::default();

    let _: TopologyMetadata =
        TopologyMetadata::default();

    let _: TopologyBuilder =
        TopologyBuilder::new();
}

// =============================================================================
// Helper sanity test
// =============================================================================

#[test]
fn test_helper_rejects_expected_invalid_operation() {
    let topology = PhysicalTopology::line(2)
        .expect("line topology should build");

    assert_routing_error(
        topology
            .validate_physical_pair(q(0), q(0)),
    );
}