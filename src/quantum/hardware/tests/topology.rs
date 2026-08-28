//! Zamani Quantum Hardware — Production Topology Conformance Tests
//!
//! Path:
//! `src/quantum/hardware/tests/topology.rs`
//!
//! # Purpose
//!
//! This module is the external/public-contract test suite for the canonical
//! hardware topology implementation:
//!
//! `crate::quantum::hardware::topology`
//!
//! It verifies the complete topology contract exposed to:
//!
//! - quantum routing;
//! - quantum scheduling;
//! - backend compatibility;
//! - backend validation;
//! - provider adapters;
//! - device discovery;
//! - benchmarking;
//! - Danga;
//! - simulator/emulator integrations;
//! - future quantum technologies using discrete physical resources.
//!
//! # Architectural rule
//!
//! These tests intentionally depend only on the canonical public topology API.
//!
//! They MUST NOT:
//!
//! - import `quantum::benchmarking`;
//! - import provider adapters;
//! - access private topology fields;
//! - depend on calibration;
//! - depend on backend credentials;
//! - perform network I/O;
//! - require a physical QPU;
//! - require a simulator;
//! - require randomness;
//! - require wall-clock time;
//! - rely on `HashMap` iteration order;
//! - rely on implementation-private data structures.
//!
//! This makes the test suite a stable conformance boundary rather than a
//! white-box implementation test.
//!
//! # Integration contract
//!
//! This file is intended to be included by `src/quantum/hardware/mod.rs` as a
//! test-only module, for example:
//
//! ```text
//! #[cfg(test)]
//! #[path = "tests/topology.rs"]
//! mod topology_tests;
//! ```
//!
//! No production dependency on this file is required.
//!
//! The tests use the canonical public path:
//!
//! `crate::quantum::hardware::topology`
//!
//! Therefore moving the internal implementation of topology does not require
//! rewriting this test suite as long as the stable public API remains intact.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021
//!
//! No nightly features are used.
//!
//! # Test philosophy
//!
//! The suite verifies:
//!
//! 1. construction;
//! 2. resource identity and enumeration;
//! 3. coupling semantics;
//! 4. directed connectivity;
//! 5. bidirectional connectivity;
//! 6. physical adjacency;
//! 7. incoming/outgoing adjacency;
//! 8. deterministic ordering;
//! 9. duplicate/conflict rejection;
//! 10. invalid-resource rejection;
//! 11. deterministic shortest paths;
//! 12. directed versus undirected traversal;
//! 13. distance semantics;
//! 14. strong connectivity;
//! 15. weak/physical connectivity;
//! 16. graph degree calculations;
//! 17. connected-component calculations;
//! 18. topology statistics;
//! 19. density calculations;
//! 20. topology validation;
//! 21. deterministic fingerprints;
//! 22. topology equality/cloning;
//! 23. default construction;
//! 24. schema/version stability;
//! 25. Send/Sync suitability;
//! 26. failed mutation atomicity;
//! 27. constructor invariants;
//! 28. edge cases;
//! 29. regression protection for routing-facing semantics.
//!
//! # Production acceptance rule
//!
//! A topology implementation must pass this entire suite before it is treated
//! as a production-ready canonical hardware topology.

#![cfg(test)]

use crate::quantum::hardware::topology::{
    Coupling,
    Connectivity,
    HardwareTopology,
    PathMode,
    QubitId,
    ResourceId,
    TopologyError,
    TOPOLOGY_SCHEMA_ID,
    TOPOLOGY_SCHEMA_VERSION,
};

// =============================================================================
// Test helpers
// =============================================================================

/// Assert that a topology has the expected resource IDs in canonical order.
fn assert_resources(topology: &HardwareTopology, expected: &[ResourceId]) {
    let actual: Vec<ResourceId> = topology.resources().collect();
    assert_eq!(actual, expected);
}

/// Assert approximate equality for floating-point graph statistics.
///
/// Topology statistics are deterministic, but floating-point representations
/// should be compared with a tolerance rather than relying on implementation
/// details of floating-point formatting.
fn assert_close(actual: f64, expected: f64) {
    const EPSILON: f64 = 1.0e-12;

    assert!(
        (actual - expected).abs() <= EPSILON,
        "expected {}, got {}",
        expected,
        actual
    );
}

/// Compile-time Send + Sync contract.
///
/// The canonical topology is a value object that is safe to transfer between
/// threads. This matters because backend discovery, compilation, scheduling
/// and execution may operate on separate worker threads.
fn assert_send_sync<T: Send + Sync>() {}

// =============================================================================
// Schema and public-contract tests
// =============================================================================

#[test]
fn topology_schema_identity_is_stable() {
    assert_eq!(
        TOPOLOGY_SCHEMA_ID,
        "zamani.quantum.hardware.topology"
    );

    assert_eq!(TOPOLOGY_SCHEMA_VERSION, 1);
}

#[test]
fn topology_types_are_send_and_sync() {
    assert_send_sync::<HardwareTopology>();
    assert_send_sync::<Coupling>();
    assert_send_sync::<TopologyError>();
}

#[test]
fn connectivity_has_stable_machine_names() {
    assert_eq!(
        Connectivity::Bidirectional.as_str(),
        "bidirectional"
    );

    assert_eq!(
        Connectivity::Directed.as_str(),
        "directed"
    );
}

#[test]
fn path_modes_have_stable_machine_names() {
    assert_eq!(
        PathMode::Directed.as_str(),
        "directed"
    );

    assert_eq!(
        PathMode::Undirected.as_str(),
        "undirected"
    );
}

// =============================================================================
// Construction
// =============================================================================

#[test]
fn zero_resource_topology_is_rejected() {
    assert_eq!(
        HardwareTopology::new(0),
        Err(TopologyError::ZeroQubits)
    );
}

#[test]
fn one_resource_topology_is_valid() {
    let topology = HardwareTopology::new(1).unwrap();

    assert_eq!(topology.qubit_count(), 1);
    assert_eq!(topology.resource_count(), 1);
    assert_eq!(topology.coupling_count(), 0);
    assert!(topology.is_empty());
    assert!(topology.validate().is_ok());
}

#[test]
fn isolated_resources_are_represented_without_couplings() {
    let topology = HardwareTopology::new(5).unwrap();

    assert_eq!(topology.resource_count(), 5);
    assert_eq!(topology.coupling_count(), 0);
    assert!(topology.is_empty());

    assert_resources(
        &topology,
        &[0, 1, 2, 3, 4],
    );

    for resource in topology.resources() {
        assert_eq!(
            topology.neighbours(resource).unwrap(),
            &[]
        );

        assert_eq!(
            topology.incoming_neighbours(resource).unwrap(),
            &[]
        );

        assert_eq!(
            topology.physical_neighbours(resource).unwrap(),
            &[]
        );

        assert_eq!(
            topology.degree(resource).unwrap(),
            0
        );
    }
}

#[test]
fn default_topology_is_valid_single_resource_hardware() {
    let topology = HardwareTopology::default();

    assert_eq!(topology.qubit_count(), 1);
    assert_eq!(topology.coupling_count(), 0);
    assert!(topology.validate().is_ok());
}

// =============================================================================
// Constructor families
// =============================================================================

#[test]
fn linear_topology_has_expected_structure() {
    let topology = HardwareTopology::linear(5).unwrap();

    assert_eq!(topology.qubit_count(), 5);
    assert_eq!(topology.coupling_count(), 4);

    assert_eq!(
        topology.physical_neighbours(0).unwrap(),
        &[1]
    );

    assert_eq!(
        topology.physical_neighbours(1).unwrap(),
        &[0, 2]
    );

    assert_eq!(
        topology.physical_neighbours(2).unwrap(),
        &[1, 3]
    );

    assert_eq!(
        topology.physical_neighbours(3).unwrap(),
        &[2, 4]
    );

    assert_eq!(
        topology.physical_neighbours(4).unwrap(),
        &[3]
    );

    assert!(topology.validate().is_ok());
}

#[test]
fn linear_one_resource_has_no_edges() {
    let topology = HardwareTopology::linear(1).unwrap();

    assert_eq!(topology.resource_count(), 1);
    assert_eq!(topology.coupling_count(), 0);
    assert!(topology.is_fully_connected());
    assert!(topology.is_strongly_connected());
}

#[test]
fn linear_two_resources_has_one_bidirectional_edge() {
    let topology = HardwareTopology::linear(2).unwrap();

    assert_eq!(topology.coupling_count(), 1);

    assert!(topology.is_connected(0, 1).unwrap());
    assert!(topology.is_connected(1, 0).unwrap());

    assert_eq!(
        topology.coupling(0, 1).unwrap(),
        Some(Coupling::bidirectional(0, 1))
    );

    assert_eq!(
        topology.coupling(1, 0).unwrap(),
        Some(Coupling::bidirectional(0, 1))
    );
}

#[test]
fn ring_of_one_resource_has_no_edge() {
    let topology = HardwareTopology::ring(1).unwrap();

    assert_eq!(topology.resource_count(), 1);
    assert_eq!(topology.coupling_count(), 0);
    assert!(topology.validate().is_ok());
}

#[test]
fn ring_of_two_resources_contains_one_physical_pair() {
    let topology = HardwareTopology::ring(2).unwrap();

    assert_eq!(topology.resource_count(), 2);
    assert_eq!(topology.coupling_count(), 1);

    assert!(topology.is_connected(0, 1).unwrap());
    assert!(topology.is_connected(1, 0).unwrap());

    assert!(topology.validate().is_ok());
}

#[test]
fn ring_of_three_resources_closes_the_cycle() {
    let topology = HardwareTopology::ring(3).unwrap();

    assert_eq!(topology.coupling_count(), 3);

    assert_eq!(
        topology.physical_neighbours(0).unwrap(),
        &[1, 2]
    );

    assert_eq!(
        topology.physical_neighbours(1).unwrap(),
        &[0, 2]
    );

    assert_eq!(
        topology.physical_neighbours(2).unwrap(),
        &[0, 1]
    );

    assert!(topology.is_strongly_connected());
    assert!(topology.is_fully_connected());
}

#[test]
fn fully_connected_topology_has_complete_graph_edge_count() {
    let topology = HardwareTopology::fully_connected(5).unwrap();

    // n(n-1)/2 = 10.
    assert_eq!(topology.coupling_count(), 10);

    assert!(topology.is_fully_connected());
    assert!(topology.is_strongly_connected());
    assert!(topology.validate().is_ok());
}

#[test]
fn fully_connected_single_resource_is_valid() {
    let topology = HardwareTopology::fully_connected(1).unwrap();

    assert_eq!(topology.resource_count(), 1);
    assert_eq!(topology.coupling_count(), 0);
    assert!(topology.is_fully_connected());
}

// =============================================================================
// Coupling constructors
// =============================================================================

#[test]
fn bidirectional_coupling_reports_both_directions() {
    let coupling = Coupling::bidirectional(3, 7);

    assert_eq!(coupling.source, 3);
    assert_eq!(coupling.target, 7);
    assert_eq!(
        coupling.connectivity,
        Connectivity::Bidirectional
    );

    assert!(coupling.contains(3));
    assert!(coupling.contains(7));
    assert!(!coupling.contains(2));

    assert_eq!(coupling.opposite(3), Some(7));
    assert_eq!(coupling.opposite(7), Some(3));
    assert_eq!(coupling.opposite(2), None);

    assert!(coupling.permits_native_direction(3, 7));
    assert!(coupling.permits_native_direction(7, 3));

    assert_eq!(coupling.canonical_pair(), (3, 7));
}

#[test]
fn directed_coupling_reports_only_native_direction() {
    let coupling = Coupling::directed(3, 7);

    assert_eq!(coupling.source, 3);
    assert_eq!(coupling.target, 7);
    assert_eq!(
        coupling.connectivity,
        Connectivity::Directed
    );

    assert!(coupling.permits_native_direction(3, 7));
    assert!(!coupling.permits_native_direction(7, 3));

    assert_eq!(coupling.canonical_pair(), (3, 7));
}

#[test]
fn canonical_pair_is_order_independent() {
    let first = Coupling::directed(2, 9);
    let second = Coupling::directed(9, 2);

    assert_eq!(
        first.canonical_pair(),
        second.canonical_pair()
    );
}

// =============================================================================
// Coupling insertion and validation
// =============================================================================

#[test]
fn valid_bidirectional_coupling_can_be_added() {
    let mut topology = HardwareTopology::new(3).unwrap();

    topology
        .add_bidirectional_coupling(0, 1)
        .unwrap();

    assert_eq!(topology.coupling_count(), 1);
    assert!(topology.validate().is_ok());
}

#[test]
fn valid_directed_coupling_can_be_added() {
    let mut topology = HardwareTopology::new(3).unwrap();

    topology
        .add_directed_coupling(0, 1)
        .unwrap();

    assert_eq!(topology.coupling_count(), 1);
    assert!(topology.validate().is_ok());
}

#[test]
fn self_coupling_is_rejected() {
    let mut topology = HardwareTopology::new(2).unwrap();

    assert_eq!(
        topology.add_bidirectional_coupling(0, 0),
        Err(TopologyError::SelfCoupling { qubit: 0 })
    );

    assert_eq!(topology.coupling_count(), 0);
    assert!(topology.validate().is_ok());
}

#[test]
fn directed_self_coupling_is_rejected() {
    let mut topology = HardwareTopology::new(2).unwrap();

    assert_eq!(
        topology.add_directed_coupling(1, 1),
        Err(TopologyError::SelfCoupling { qubit: 1 })
    );

    assert_eq!(topology.coupling_count(), 0);
    assert!(topology.validate().is_ok());
}

#[test]
fn invalid_source_resource_is_rejected() {
    let mut topology = HardwareTopology::new(2).unwrap();

    assert_eq!(
        topology.add_bidirectional_coupling(2, 1),
        Err(TopologyError::InvalidQubit {
            qubit: 2,
            qubit_count: 2
        })
    );

    assert_eq!(topology.coupling_count(), 0);
}

#[test]
fn invalid_target_resource_is_rejected() {
    let mut topology = HardwareTopology::new(2).unwrap();

    assert_eq!(
        topology.add_bidirectional_coupling(0, 2),
        Err(TopologyError::InvalidQubit {
            qubit: 2,
            qubit_count: 2
        })
    );

    assert_eq!(topology.coupling_count(), 0);
}

#[test]
fn invalid_resource_is_rejected_by_queries() {
    let topology = HardwareTopology::new(2).unwrap();

    let expected = TopologyError::InvalidQubit {
        qubit: 2,
        qubit_count: 2,
    };

    assert_eq!(
        topology.neighbours(2),
        Err(expected.clone())
    );

    assert_eq!(
        topology.incoming_neighbours(2),
        Err(expected.clone())
    );

    assert_eq!(
        topology.physical_neighbours(2),
        Err(expected.clone())
    );

    assert_eq!(
        topology.degree(2),
        Err(expected.clone())
    );

    assert_eq!(
        topology.out_degree(2),
        Err(expected.clone())
    );

    assert_eq!(
        topology.in_degree(2),
        Err(expected)
    );
}

#[test]
fn duplicate_bidirectional_coupling_is_rejected() {
    let mut topology = HardwareTopology::new(2).unwrap();

    topology
        .add_bidirectional_coupling(0, 1)
        .unwrap();

    assert_eq!(
        topology.add_bidirectional_coupling(0, 1),
        Err(TopologyError::DuplicateCoupling {
            source: 0,
            target: 1
        })
    );

    assert_eq!(topology.coupling_count(), 1);
}

#[test]
fn reverse_duplicate_bidirectional_coupling_is_rejected() {
    let mut topology = HardwareTopology::new(2).unwrap();

    topology
        .add_bidirectional_coupling(0, 1)
        .unwrap();

    assert_eq!(
        topology.add_bidirectional_coupling(1, 0),
        Err(TopologyError::DuplicateCoupling {
            source: 1,
            target: 0
        })
    );

    assert_eq!(topology.coupling_count(), 1);
}

#[test]
fn duplicate_directed_coupling_is_rejected() {
    let mut topology = HardwareTopology::new(2).unwrap();

    topology
        .add_directed_coupling(0, 1)
        .unwrap();

    assert_eq!(
        topology.add_directed_coupling(0, 1),
        Err(TopologyError::DuplicateCoupling {
            source: 0,
            target: 1
        })
    );

    assert_eq!(topology.coupling_count(), 1);
}

#[test]
fn opposite_directed_couplings_are_allowed() {
    let mut topology = HardwareTopology::new(2).unwrap();

    topology
        .add_directed_coupling(0, 1)
        .unwrap();

    topology
        .add_directed_coupling(1, 0)
        .unwrap();

    assert_eq!(topology.coupling_count(), 2);

    assert!(topology.is_connected(0, 1).unwrap());
    assert!(topology.is_connected(1, 0).unwrap());

    assert!(topology.validate().is_ok());
}

#[test]
fn bidirectional_coupling_conflicts_with_existing_forward_directed_edge() {
    let mut topology = HardwareTopology::new(2).unwrap();

    topology
        .add_directed_coupling(0, 1)
        .unwrap();

    assert_eq!(
        topology.add_bidirectional_coupling(0, 1),
        Err(TopologyError::DuplicateCoupling {
            source: 0,
            target: 1
        })
    );

    assert_eq!(topology.coupling_count(), 1);
}

#[test]
fn bidirectional_coupling_conflicts_with_existing_reverse_directed_edge() {
    let mut topology = HardwareTopology::new(2).unwrap();

    topology
        .add_directed_coupling(1, 0)
        .unwrap();

    assert_eq!(
        topology.add_bidirectional_coupling(0, 1),
        Err(TopologyError::DuplicateCoupling {
            source: 0,
            target: 1
        })
    );

    assert_eq!(topology.coupling_count(), 1);
}

#[test]
fn failed_mutation_is_atomic() {
    let mut topology = HardwareTopology::linear(4).unwrap();
    let before = topology.clone();

    let result = topology.add_bidirectional_coupling(1, 2);

    assert_eq!(
        result,
        Err(TopologyError::DuplicateCoupling {
            source: 1,
            target: 2
        })
    );

    assert_eq!(topology, before);
    assert!(topology.validate().is_ok());
}

// =============================================================================
// Connectivity semantics
// =============================================================================

#[test]
fn bidirectional_edge_is_connected_in_both_directions() {
    let topology = HardwareTopology::from_couplings(
        2,
        [Coupling::bidirectional(0, 1)],
    )
    .unwrap();

    assert!(topology.is_connected(0, 1).unwrap());
    assert!(topology.is_connected(1, 0).unwrap());

    assert!(topology.is_physically_adjacent(0, 1).unwrap());
    assert!(topology.is_physically_adjacent(1, 0).unwrap());

    assert!(topology.has_physical_connection(0, 1).unwrap());
    assert!(topology.has_physical_connection(1, 0).unwrap());
}

#[test]
fn directed_edge_is_connected_only_in_native_direction() {
    let topology = HardwareTopology::from_couplings(
        2,
        [Coupling::directed(0, 1)],
    )
    .unwrap();

    assert!(topology.is_connected(0, 1).unwrap());
    assert!(!topology.is_connected(1, 0).unwrap());

    assert!(topology.is_physically_adjacent(0, 1).unwrap());
    assert!(topology.is_physically_adjacent(1, 0).unwrap());

    assert!(topology.has_physical_connection(0, 1).unwrap());
    assert!(topology.has_physical_connection(1, 0).unwrap());
}

#[test]
fn directed_edge_has_correct_outgoing_and_incoming_neighbours() {
    let topology = HardwareTopology::from_couplings(
        2,
        [Coupling::directed(0, 1)],
    )
    .unwrap();

    assert_eq!(
        topology.neighbours(0).unwrap(),
        &[1]
    );

    assert_eq!(
        topology.neighbours(1).unwrap(),
        &[]
    );

    assert_eq!(
        topology.incoming_neighbours(0).unwrap(),
        &[]
    );

    assert_eq!(
        topology.incoming_neighbours(1).unwrap(),
        &[0]
    );
}

#[test]
fn physical_neighbours_ignore_native_direction() {
    let topology = HardwareTopology::from_couplings(
        2,
        [Coupling::directed(0, 1)],
    )
    .unwrap();

    assert_eq!(
        topology.physical_neighbours(0).unwrap(),
        &[1]
    );

    assert_eq!(
        topology.physical_neighbours(1).unwrap(),
        &[0]
    );
}

#[test]
fn coupling_lookup_respects_native_direction() {
    let topology = HardwareTopology::from_couplings(
        2,
        [Coupling::directed(0, 1)],
    )
    .unwrap();

    assert_eq!(
        topology.coupling(0, 1).unwrap(),
        Some(Coupling::directed(0, 1))
    );

    assert_eq!(
        topology.coupling(1, 0).unwrap(),
        None
    );
}

#[test]
fn bidirectional_coupling_lookup_works_in_both_directions() {
    let topology = HardwareTopology::from_couplings(
        2,
        [Coupling::bidirectional(0, 1)],
    )
    .unwrap();

    assert_eq!(
        topology.coupling(0, 1).unwrap(),
        Some(Coupling::bidirectional(0, 1))
    );

    assert_eq!(
        topology.coupling(1, 0).unwrap(),
        Some(Coupling::bidirectional(0, 1))
    );
}

#[test]
fn coupling_collection_is_canonically_sorted() {
    let topology = HardwareTopology::from_couplings(
        5,
        [
            Coupling::bidirectional(4, 3),
            Coupling::bidirectional(2, 0),
            Coupling::bidirectional(1, 4),
            Coupling::directed(0, 4),
        ],
    );

    // The final coupling above intentionally conflicts with the bidirectional
    // pair 1/4 only if it uses the same physical pair; it does not, so this is
    // a valid topology.
    let topology = topology.unwrap();

    let couplings = topology.couplings();

    assert_eq!(
        couplings,
        &[
            Coupling::bidirectional(1, 4),
            Coupling::bidirectional(2, 0),
            Coupling::bidirectional(4, 3),
            Coupling::directed(0, 4),
        ]
        .iter()
        .copied()
        .collect::<Vec<_>>()
        .as_slice()
    );

    // The public API promises deterministic ordering. Verify it directly.
    assert!(
        couplings
            .windows(2)
            .all(|window| window[0] < window[1])
    );
}

// =============================================================================
// Path semantics
// =============================================================================

#[test]
fn shortest_path_on_linear_topology_is_deterministic() {
    let topology = HardwareTopology::linear(5).unwrap();

    assert_eq!(
        topology.shortest_path(0, 4).unwrap(),
        vec![0, 1, 2, 3, 4]
    );

    assert_eq!(
        topology.shortest_path(4, 0).unwrap(),
        vec![4, 3, 2, 1, 0]
    );
}

#[test]
fn source_equal_to_target_has_single_vertex_path() {
    let topology = HardwareTopology::linear(5).unwrap();

    assert_eq!(
        topology.shortest_path(2, 2).unwrap(),
        vec![2]
    );
}

#[test]
fn source_equal_to_target_has_zero_distance() {
    let topology = HardwareTopology::linear(5).unwrap();

    assert_eq!(
        topology.distance(2, 2).unwrap(),
        0
    );

    assert_eq!(
        topology.distance_with_mode(
            2,
            2,
            PathMode::Directed
        )
        .unwrap(),
        0
    );

    assert_eq!(
        topology.distance_with_mode(
            2,
            2,
            PathMode::Undirected
        )
        .unwrap(),
        0
    );
}

#[test]
fn directed_shortest_path_respects_native_direction() {
    let topology = HardwareTopology::from_couplings(
        3,
        [
            Coupling::directed(0, 1),
            Coupling::directed(1, 2),
        ],
    )
    .unwrap();

    assert_eq!(
        topology
            .shortest_path_with_mode(
                0,
                2,
                PathMode::Directed
            )
            .unwrap(),
        vec![0, 1, 2]
    );

    assert_eq!(
        topology.distance_with_mode(
            0,
            2,
            PathMode::Directed
        )
        .unwrap(),
        2
    );

    assert_eq!(
        topology.shortest_path_with_mode(
            2,
            0,
            PathMode::Directed
        ),
        Err(TopologyError::NoPath {
            source: 2,
            target: 0
        })
    );
}

#[test]
fn undirected_shortest_path_ignores_native_direction() {
    let topology = HardwareTopology::from_couplings(
        3,
        [
            Coupling::directed(0, 1),
            Coupling::directed(1, 2),
        ],
    )
    .unwrap();

    assert_eq!(
        topology
            .shortest_path_with_mode(
                2,
                0,
                PathMode::Undirected
            )
            .unwrap(),
        vec![2, 1, 0]
    );

    assert_eq!(
        topology.distance_with_mode(
            2,
            0,
            PathMode::Undirected
        )
        .unwrap(),
        2
    );
}

#[test]
fn directed_path_failure_is_not_hidden_by_physical_adjacency() {
    let topology = HardwareTopology::from_couplings(
        2,
        [Coupling::directed(0, 1)],
    )
    .unwrap();

    assert_eq!(
        topology.shortest_path_with_mode(
            1,
            0,
            PathMode::Directed
        ),
        Err(TopologyError::NoPath {
            source: 1,
            target: 0
        })
    );

    assert_eq!(
        topology
            .shortest_path_with_mode(
                1,
                0,
                PathMode::Undirected
            )
            .unwrap(),
        vec![1, 0]
    );
}

#[test]
fn disconnected_topology_returns_no_path() {
    let topology = HardwareTopology::new(3).unwrap();

    assert_eq!(
        topology.shortest_path(0, 2),
        Err(TopologyError::NoPath {
            source: 0,
            target: 2
        })
    );
}

#[test]
fn invalid_path_endpoints_are_rejected() {
    let topology = HardwareTopology::new(3).unwrap();

    assert_eq!(
        topology.shortest_path(3, 0),
        Err(TopologyError::InvalidQubit {
            qubit: 3,
            qubit_count: 3
        })
    );

    assert_eq!(
        topology.shortest_path(0, 3),
        Err(TopologyError::InvalidQubit {
            qubit: 3,
            qubit_count: 3
        })
    );
}

#[test]
fn shortest_path_tie_breaking_is_deterministic() {
    let topology = HardwareTopology::from_couplings(
        4,
        [
            Coupling::bidirectional(0, 2),
            Coupling::bidirectional(0, 1),
            Coupling::bidirectional(2, 3),
            Coupling::bidirectional(1, 3),
        ],
    )
    .unwrap();

    // Both [0,1,3] and [0,2,3] have equal length. The topology contract
    // requires deterministic adjacency traversal. Sorted adjacency makes the
    // smaller neighbour win.
    assert_eq!(
        topology.shortest_path(0, 3).unwrap(),
        vec![0, 1, 3]
    );
}

// =============================================================================
// Connectivity analysis
// =============================================================================

#[test]
fn linear_topology_is_strongly_connected() {
    let topology = HardwareTopology::linear(6).unwrap();

    assert!(topology.is_strongly_connected());
}

#[test]
fn directed_chain_is_not_strongly_connected() {
    let topology = HardwareTopology::from_couplings(
        4,
        [
            Coupling::directed(0, 1),
            Coupling::directed(1, 2),
            Coupling::directed(2, 3),
        ],
    )
    .unwrap();

    assert!(!topology.is_strongly_connected());
}

#[test]
fn directed_cycle_is_strongly_connected() {
    let topology = HardwareTopology::from_couplings(
        4,
        [
            Coupling::directed(0, 1),
            Coupling::directed(1, 2),
            Coupling::directed(2, 3),
            Coupling::directed(3, 0),
        ],
    )
    .unwrap();

    assert!(topology.is_strongly_connected());
}

#[test]
fn directed_chain_is_physically_connected() {
    let topology = HardwareTopology::from_couplings(
        4,
        [
            Coupling::directed(0, 1),
            Coupling::directed(1, 2),
            Coupling::directed(2, 3),
        ],
    )
    .unwrap();

    assert!(topology.is_fully_connected());
}

#[test]
fn disconnected_topology_is_not_fully_connected() {
    let topology = HardwareTopology::from_couplings(
        4,
        [Coupling::bidirectional(0, 1)],
    )
    .unwrap();

    assert!(!topology.is_fully_connected());
}

#[test]
fn connected_components_are_counted_correctly() {
    let topology = HardwareTopology::from_couplings(
        7,
        [
            Coupling::bidirectional(0, 1),
            Coupling::bidirectional(1, 2),
            Coupling::bidirectional(3, 4),
            Coupling::bidirectional(5, 6),
        ],
    )
    .unwrap();

    assert_eq!(
        topology.connected_components(),
        3
    );
}

#[test]
fn isolated_resources_form_individual_components() {
    let topology = HardwareTopology::new(4).unwrap();

    assert_eq!(
        topology.connected_components(),
        4
    );

    assert!(!topology.is_fully_connected());
}

// =============================================================================
// Degree analysis
// =============================================================================

#[test]
fn linear_degrees_are_correct() {
    let topology = HardwareTopology::linear(5).unwrap();

    assert_eq!(topology.degree(0).unwrap(), 1);
    assert_eq!(topology.degree(1).unwrap(), 2);
    assert_eq!(topology.degree(2).unwrap(), 2);
    assert_eq!(topology.degree(3).unwrap(), 2);
    assert_eq!(topology.degree(4).unwrap(), 1);

    assert_eq!(topology.maximum_degree(), 2);
    assert_eq!(topology.minimum_degree(), 1);

    assert_close(
        topology.average_degree(),
        1.6,
    );
}

#[test]
fn directed_degrees_are_separated_correctly() {
    let topology = HardwareTopology::from_couplings(
        3,
        [
            Coupling::directed(0, 1),
            Coupling::directed(1, 2),
        ],
    )
    .unwrap();

    assert_eq!(topology.out_degree(0).unwrap(), 1);
    assert_eq!(topology.out_degree(1).unwrap(), 1);
    assert_eq!(topology.out_degree(2).unwrap(), 0);

    assert_eq!(topology.in_degree(0).unwrap(), 0);
    assert_eq!(topology.in_degree(1).unwrap(), 1);
    assert_eq!(topology.in_degree(2).unwrap(), 1);

    // Physical degree ignores native direction.
    assert_eq!(topology.degree(0).unwrap(), 1);
    assert_eq!(topology.degree(1).unwrap(), 2);
    assert_eq!(topology.degree(2).unwrap(), 1);
}

#[test]
fn fully_connected_degrees_are_correct() {
    let topology = HardwareTopology::fully_connected(5).unwrap();

    for resource in topology.resources() {
        assert_eq!(
            topology.degree(resource).unwrap(),
            4
        );
    }

    assert_eq!(topology.minimum_degree(), 4);
    assert_eq!(topology.maximum_degree(), 4);
    assert_close(topology.average_degree(), 4.0);
}

// =============================================================================
// Statistics
// =============================================================================

#[test]
fn linear_statistics_are_correct() {
    let topology = HardwareTopology::linear(4).unwrap();

    let statistics = topology.statistics();

    assert_eq!(statistics.resource_count, 4);
    assert_eq!(statistics.coupling_count, 3);

    assert_eq!(
        statistics.directed_coupling_count,
        0
    );

    assert_eq!(
        statistics.bidirectional_coupling_count,
        3
    );

    assert_eq!(
        statistics.connected_resource_count,
        4
    );

    assert_eq!(
        statistics.connected_components,
        1
    );

    assert_eq!(statistics.minimum_degree, 1);
    assert_eq!(statistics.maximum_degree, 2);

    assert_close(
        statistics.average_degree,
        1.5
    );

    assert_close(
        statistics.undirected_density,
        0.5
    );

    assert!(statistics.is_connected);
    assert!(statistics.is_fully_connected());

    assert_eq!(statistics.diameter, Some(3));

    assert_close(
        statistics.average_shortest_path.unwrap(),
        5.0 / 3.0
    );
}

#[test]
fn directed_statistics_separate_edge_direction_from_physical_density() {
    let topology = HardwareTopology::from_couplings(
        3,
        [
            Coupling::directed(0, 1),
            Coupling::directed(1, 2),
        ],
    )
    .unwrap();

    let statistics = topology.statistics();

    assert_eq!(
        statistics.coupling_count,
        2
    );

    assert_eq!(
        statistics.directed_coupling_count,
        2
    );

    assert_eq!(
        statistics.bidirectional_coupling_count,
        0
    );

    // Two physical pairs exist out of three possible unordered pairs.
    assert_close(
        statistics.undirected_density,
        2.0 / 3.0
    );

    assert!(statistics.is_connected);
    assert_eq!(statistics.connected_components, 1);
}

#[test]
fn opposite_directed_edges_do_not_count_as_two_physical_pairs() {
    let topology = HardwareTopology::from_couplings(
        2,
        [
            Coupling::directed(0, 1),
            Coupling::directed(1, 0),
        ],
    )
    .unwrap();

    let statistics = topology.statistics();

    assert_eq!(statistics.coupling_count, 2);
    assert_eq!(
        statistics.directed_coupling_count,
        2
    );

    assert_close(
        statistics.undirected_density,
        1.0
    );
}

#[test]
fn disconnected_statistics_report_multiple_components() {
    let topology = HardwareTopology::from_couplings(
        4,
        [Coupling::bidirectional(0, 1)],
    )
    .unwrap();

    let statistics = topology.statistics();

    assert_eq!(
        statistics.connected_components,
        3
    );

    assert_eq!(
        statistics.connected_resource_count,
        2
    );

    assert!(!statistics.is_connected);

    // The topology is disconnected, therefore no finite global diameter or
    // average shortest-path statistic exists.
    assert_eq!(
        statistics.diameter,
        None
    );

    assert_eq!(
        statistics.average_shortest_path,
        None
    );
}

#[test]
fn single_resource_statistics_are_well_defined() {
    let topology = HardwareTopology::new(1).unwrap();

    let statistics = topology.statistics();

    assert_eq!(statistics.resource_count, 1);
    assert_eq!(statistics.coupling_count, 0);
    assert_eq!(statistics.connected_resource_count, 0);
    assert_eq!(statistics.connected_components, 1);

    assert_eq!(statistics.minimum_degree, 0);
    assert_eq!(statistics.maximum_degree, 0);

    assert_close(
        statistics.average_degree,
        0.0
    );

    assert_close(
        statistics.undirected_density,
        0.0
    );

    assert!(statistics.is_connected);
    assert_eq!(statistics.diameter, Some(0));
    assert_eq!(statistics.average_shortest_path, None);
}

// =============================================================================
// Topology validation
// =============================================================================

#[test]
fn all_supported_constructor_families_validate() {
    let topologies = [
        HardwareTopology::new(1).unwrap(),
        HardwareTopology::linear(1).unwrap(),
        HardwareTopology::linear(8).unwrap(),
        HardwareTopology::ring(1).unwrap(),
        HardwareTopology::ring(2).unwrap(),
        HardwareTopology::ring(8).unwrap(),
        HardwareTopology::fully_connected(1).unwrap(),
        HardwareTopology::fully_connected(6).unwrap(),
    ];

    for topology in &topologies {
        assert!(
            topology.validate().is_ok(),
            "topology failed validation: {:?}",
            topology
        );
    }
}

#[test]
fn from_couplings_validates_all_edges() {
    let topology = HardwareTopology::from_couplings(
        5,
        [
            Coupling::directed(0, 1),
            Coupling::bidirectional(1, 2),
            Coupling::directed(2, 3),
            Coupling::bidirectional(3, 4),
        ],
    )
    .unwrap();

    assert!(topology.validate().is_ok());
}

#[test]
fn invalid_coupling_collection_does_not_produce_partial_success() {
    let result = HardwareTopology::from_couplings(
        3,
        [
            Coupling::bidirectional(0, 1),
            Coupling::bidirectional(1, 1),
        ],
    );

    assert_eq!(
        result,
        Err(TopologyError::SelfCoupling { qubit: 1 })
    );
}

#[test]
fn invalid_coupling_endpoint_does_not_produce_partial_success() {
    let result = HardwareTopology::from_couplings(
        3,
        [
            Coupling::bidirectional(0, 1),
            Coupling::bidirectional(1, 3),
        ],
    );

    assert_eq!(
        result,
        Err(TopologyError::InvalidQubit {
            qubit: 3,
            qubit_count: 3
        })
    );
}

// =============================================================================
// Determinism
// =============================================================================

#[test]
fn resources_are_deterministically_ordered() {
    let topology = HardwareTopology::new(8).unwrap();

    let first: Vec<_> = topology.resources().collect();
    let second: Vec<_> = topology.resources().collect();

    assert_eq!(first, second);

    assert_eq!(
        first,
        vec![0, 1, 2, 3, 4, 5, 6, 7]
    );
}

#[test]
fn adjacency_lists_are_deterministically_sorted() {
    let topology = HardwareTopology::from_couplings(
        6,
        [
            Coupling::bidirectional(3, 1),
            Coupling::bidirectional(5, 2),
            Coupling::bidirectional(4, 1),
            Coupling::bidirectional(2, 0),
        ],
    )
    .unwrap();

    for resource in topology.resources() {
        let neighbours = topology.physical_neighbours(resource).unwrap();

        assert!(
            neighbours
                .windows(2)
                .all(|window| window[0] < window[1]),
            "physical adjacency is not sorted for resource {}",
            resource
        );
    }
}

#[test]
fn same_topology_built_in_different_insertion_orders_has_same_fingerprint() {
    let first = HardwareTopology::from_couplings(
        8,
        [
            Coupling::bidirectional(0, 1),
            Coupling::bidirectional(1, 2),
            Coupling::bidirectional(2, 3),
            Coupling::bidirectional(3, 4),
            Coupling::bidirectional(4, 5),
            Coupling::bidirectional(5, 6),
            Coupling::bidirectional(6, 7),
        ],
    )
    .unwrap();

    let second = HardwareTopology::from_couplings(
        8,
        [
            Coupling::bidirectional(6, 7),
            Coupling::bidirectional(3, 4),
            Coupling::bidirectional(0, 1),
            Coupling::bidirectional(5, 6),
            Coupling::bidirectional(1, 2),
            Coupling::bidirectional(4, 5),
            Coupling::bidirectional(2, 3),
        ],
    )
    .unwrap();

    assert_eq!(
        first,
        second
    );

    assert_eq!(
        first.fingerprint(),
        second.fingerprint()
    );
}

#[test]
fn different_resource_counts_have_different_fingerprints() {
    let first = HardwareTopology::linear(4).unwrap();
    let second = HardwareTopology::linear(5).unwrap();

    assert_ne!(
        first.fingerprint(),
        second.fingerprint()
    );
}

#[test]
fn different_connectivity_semantics_have_different_fingerprints() {
    let directed = HardwareTopology::from_couplings(
        2,
        [Coupling::directed(0, 1)],
    )
    .unwrap();

    let bidirectional = HardwareTopology::from_couplings(
        2,
        [Coupling::bidirectional(0, 1)],
    )
    .unwrap();

    assert_ne!(
        directed.fingerprint(),
        bidirectional.fingerprint()
    );
}

#[test]
fn fingerprint_is_repeatable() {
    let topology = HardwareTopology::ring(12).unwrap();

    let first = topology.fingerprint();
    let second = topology.fingerprint();
    let third = topology.fingerprint();

    assert_eq!(first, second);
    assert_eq!(second, third);

    // The current contract emits a fixed-width hexadecimal u64 string.
    assert_eq!(first.len(), 16);
    assert!(
        first
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit())
    );
}

// =============================================================================
// Clone/equality semantics
// =============================================================================

#[test]
fn cloned_topology_is_equal_to_original() {
    let topology = HardwareTopology::ring(9).unwrap();
    let cloned = topology.clone();

    assert_eq!(topology, cloned);
    assert_eq!(
        topology.fingerprint(),
        cloned.fingerprint()
    );
}

#[test]
fn different_topologies_are_not_equal() {
    let linear = HardwareTopology::linear(5).unwrap();
    let ring = HardwareTopology::ring(5).unwrap();

    assert_ne!(linear, ring);
}

// =============================================================================
// Public membership API
// =============================================================================

#[test]
fn contains_accepts_only_valid_resource_ids() {
    let topology = HardwareTopology::new(4).unwrap();

    assert!(topology.contains(0));
    assert!(topology.contains(1));
    assert!(topology.contains(2));
    assert!(topology.contains(3));

    assert!(!topology.contains(4));
    assert!(!topology.contains(usize::MAX));
}

#[test]
fn resource_and_qubit_counts_are_identical() {
    let topology = HardwareTopology::ring(10).unwrap();

    assert_eq!(
        topology.resource_count(),
        topology.qubit_count()
    );
}

// =============================================================================
// Regression tests for routing-facing semantics
// =============================================================================

#[test]
fn physical_adjacency_must_not_be_used_as_native_connectivity() {
    let topology = HardwareTopology::from_couplings(
        2,
        [Coupling::directed(0, 1)],
    )
    .unwrap();

    // Physically connected:
    assert!(topology.is_physically_adjacent(1, 0).unwrap());

    // But not natively executable in reverse:
    assert!(!topology.is_connected(1, 0).unwrap());

    // Routing that respects native direction must fail:
    assert_eq!(
        topology.shortest_path(1, 0),
        Err(TopologyError::NoPath {
            source: 1,
            target: 0
        })
    );

    // Physical-distance analysis may still traverse it:
    assert_eq!(
        topology.distance_with_mode(
            1,
            0,
            PathMode::Undirected
        )
        .unwrap(),
        1
    );
}

#[test]
fn native_path_and_physical_distance_are_distinct_contracts() {
    let topology = HardwareTopology::from_couplings(
        4,
        [
            Coupling::directed(0, 1),
            Coupling::directed(1, 2),
            Coupling::directed(2, 3),
        ],
    )
    .unwrap();

    assert_eq!(
        topology.distance(0, 3).unwrap(),
        3
    );

    assert_eq!(
        topology.distance_with_mode(
            3,
            0,
            PathMode::Undirected
        )
        .unwrap(),
        3
    );

    assert_eq!(
        topology.distance_with_mode(
            3,
            0,
            PathMode::Directed
        ),
        Err(TopologyError::NoPath {
            source: 3,
            target: 0
        })
    );
}

// =============================================================================
// Large deterministic topology regression
// =============================================================================

#[test]
fn moderate_linear_topology_remains_correct() {
    // Large enough to catch accidental assumptions about small fixed device
    // sizes, while remaining fast enough for every CI run.
    const RESOURCE_COUNT: usize = 256;

    let topology = HardwareTopology::linear(RESOURCE_COUNT).unwrap();

    assert_eq!(
        topology.resource_count(),
        RESOURCE_COUNT
    );

    assert_eq!(
        topology.coupling_count(),
        RESOURCE_COUNT - 1
    );

    assert_eq!(
        topology.degree(0).unwrap(),
        1
    );

    assert_eq!(
        topology.degree(RESOURCE_COUNT - 1).unwrap(),
        1
    );

    assert_eq!(
        topology.degree(RESOURCE_COUNT / 2).unwrap(),
        2
    );

    assert_eq!(
        topology.distance(
            0,
            RESOURCE_COUNT - 1
        )
        .unwrap(),
        RESOURCE_COUNT - 1
    );

    assert!(topology.validate().is_ok());
}

#[test]
fn moderate_ring_topology_has_expected_connectivity() {
    const RESOURCE_COUNT: usize = 128;

    let topology = HardwareTopology::ring(RESOURCE_COUNT).unwrap();

    assert_eq!(
        topology.coupling_count(),
        RESOURCE_COUNT
    );

    assert!(topology.is_fully_connected());
    assert!(topology.is_strongly_connected());

    for resource in topology.resources() {
        assert_eq!(
            topology.degree(resource).unwrap(),
            2
        );
    }

    assert_eq!(
        topology.minimum_degree(),
        2
    );

    assert_eq!(
        topology.maximum_degree(),
        2
    );

    assert_close(
        topology.average_degree(),
        2.0
    );

    assert!(topology.validate().is_ok());
}

// =============================================================================
// Public error semantics
// =============================================================================

#[test]
fn topology_errors_have_nonempty_display_messages() {
    let errors = [
        TopologyError::ZeroQubits,
        TopologyError::ZeroResources,
        TopologyError::InvalidQubit {
            qubit: 10,
            qubit_count: 2,
        },
        TopologyError::InvalidResource {
            resource: 10,
            resource_count: 2,
        },
        TopologyError::SelfCoupling { qubit: 1 },
        TopologyError::DuplicateCoupling {
            source: 0,
            target: 1,
        },
        TopologyError::MissingCoupling {
            source: 0,
            target: 1,
        },
        TopologyError::NoPath {
            source: 0,
            target: 1,
        },
        TopologyError::InvalidTopology {
            message: "test invariant violation".to_string(),
        },
        TopologyError::NumericOverflow {
            operation: "test",
        },
    ];

    for error in &errors {
        assert!(
            !error.to_string().trim().is_empty(),
            "error must have a useful display message"
        );
    }
}

#[test]
fn missing_coupling_error_can_be_constructed_as_public_contract() {
    let error = TopologyError::MissingCoupling {
        source: 1,
        target: 2,
    };

    assert_eq!(
        error.to_string(),
        "no native coupling exists from 1 to 2"
    );
}

// =============================================================================
// Regression guard: no hidden duplicate adjacency
// =============================================================================

#[test]
fn every_physical_neighbour_relation_is_symmetric() {
    let topology = HardwareTopology::from_couplings(
        8,
        [
            Coupling::directed(0, 1),
            Coupling::directed(2, 1),
            Coupling::bidirectional(2, 3),
            Coupling::directed(5, 4),
            Coupling::bidirectional(6, 7),
        ],
    )
    .unwrap();

    for source in topology.resources() {
        let neighbours =
            topology.physical_neighbours(source).unwrap();

        for &target in neighbours {
            assert!(
                topology
                    .physical_neighbours(target)
                    .unwrap()
                    .contains(&source),
                "physical adjacency must be symmetric: {} -- {}",
                source,
                target
            );
        }
    }
}

#[test]
fn native_adjacency_matches_coupling_direction() {
    let topology = HardwareTopology::from_couplings(
        5,
        [
            Coupling::directed(0, 1),
            Coupling::directed(2, 1),
            Coupling::bidirectional(3, 4),
        ],
    )
    .unwrap();

    assert_eq!(
        topology.neighbours(0).unwrap(),
        &[1]
    );

    assert_eq!(
        topology.neighbours(1).unwrap(),
        &[]
    );

    assert_eq!(
        topology.neighbours(2).unwrap(),
        &[1]
    );

    assert_eq!(
        topology.neighbours(3).unwrap(),
        &[4]
    );

    assert_eq!(
        topology.neighbours(4).unwrap(),
        &[3]
    );

    assert_eq!(
        topology.incoming_neighbours(1).unwrap(),
        &[0, 2]
    );
}

// =============================================================================
// End-to-end topology contract
// =============================================================================

#[test]
fn production_topology_contract_smoke_test() {
    let topology = HardwareTopology::from_couplings(
        6,
        [
            Coupling::bidirectional(0, 1),
            Coupling::directed(1, 2),
            Coupling::directed(2, 3),
            Coupling::bidirectional(3, 4),
            Coupling::directed(4, 5),
        ],
    )
    .unwrap();

    // Construction.
    assert_eq!(topology.resource_count(), 6);

    // Structural validity.
    assert!(topology.validate().is_ok());

    // Physical connectivity.
    assert!(topology.is_fully_connected());

    // Native directed connectivity is not necessarily strong.
    assert!(!topology.is_strongly_connected());

    // Native routing works in the forward direction.
    assert_eq!(
        topology.shortest_path(0, 5).unwrap(),
        vec![0, 1, 2, 3, 4, 5]
    );

    // Reverse native routing is not guaranteed.
    assert_eq!(
        topology.shortest_path(5, 0),
        Err(TopologyError::NoPath {
            source: 5,
            target: 0
        })
    );

    // Physical distance remains available independently.
    assert_eq!(
        topology
            .distance_with_mode(
                5,
                0,
                PathMode::Undirected
            )
            .unwrap(),
        5
    );

    // Graph statistics are available.
    let statistics = topology.statistics();

    assert_eq!(
        statistics.resource_count,
        6
    );

    assert_eq!(
        statistics.coupling_count,
        5
    );

    assert_eq!(
        statistics.directed_coupling_count,
        3
    );

    assert_eq!(
        statistics.bidirectional_coupling_count,
        2
    );

    assert_eq!(
        statistics.connected_components,
        1
    );

    assert!(statistics.is_connected);

    // Provenance identity exists and is deterministic.
    let fingerprint = topology.fingerprint();

    assert_eq!(
        fingerprint.len(),
        16
    );

    assert_eq!(
        fingerprint,
        topology.fingerprint()
    );
}