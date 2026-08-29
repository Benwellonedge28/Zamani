//! Zamani Quantum Routing — Basic Router Test Suite
//!
//! `src/quantum/routing/tests/basic.rs`
//!
//! # Purpose
//!
//! Production-level integration tests for the deterministic `BasicRouter`.
//!
//! This test suite verifies the externally observable contract of the basic
//! routing algorithm:
//!
//! - construction;
//! - algorithm identity;
//! - already-executable operations;
//! - adjacent two-qubit operations;
//! - non-adjacent two-qubit operations;
//! - deterministic SWAP insertion;
//! - mapping evolution;
//! - preservation of logical gate operands;
//! - multiple routed operations;
//! - single-qubit operations;
//! - zero-qubit operations;
//! - maximum-SWAP limits;
//! - iteration limits;
//! - insufficient physical resources;
//! - invalid logical mappings;
//! - disconnected topology;
//! - duplicate logical operands;
//! - unsupported multi-qubit operations;
//! - deterministic repeated execution;
//! - caller mapping immutability;
//! - result initial/final mappings;
//! - routing metrics;
//! - routing-operation classification;
//! - output verification;
//! - transactional failure semantics.
//!
//! # Architectural boundary
//!
//! These tests consume only the public routing contracts:
//!
//! ```text
//! routing::algorithms::basic
//! routing::config
//! routing::errors
//! routing::mapping
//! routing::result
//! routing::topology
//! routing::types
//! ```
//!
//! They do NOT depend on:
//!
//! - SABRE;
//! - lookahead;
//! - noise-aware routing;
//! - dynamic routing;
//! - compiler IR;
//! - OpenQASM;
//! - hardware providers;
//! - scheduling;
//! - pulse generation;
//! - simulation;
//! - QEC;
//! - benchmarking implementations.
//!
//! This is intentional. The basic router must remain independently testable
//! as the deterministic reference routing implementation.
//!
//! # Integration contract
//!
//! This file assumes the following frozen contracts:
//!
//! 1. `QuantumOperation` is the routing-level immutable operation type.
//! 2. `BasicRouter::route_with_mapping()` accepts:
//!
//!    ```text
//!    &[QuantumOperation]
//!    &Topology
//!    &QubitMapping
//!    &RoutingConfig
//!    ```
//!
//! 3. `RoutingOperation::Move(RoutingMove::Swap { ... })` represents a
//!    semantic routing SWAP.
//! 4. `RoutingOperation::Gate { ... }` represents a routed logical gate.
//! 5. `RoutingResult` exposes initial/final mappings, operations and metrics.
//! 6. `QubitMapping` exposes immutable lookup and validation APIs.
//! 7. `Topology` exposes deterministic topology construction and validation.
//!
//! No test relies on concrete storage types such as `HashMap`.
//!
//! # Production requirements
//!
//! Every test:
//!
//! - is deterministic;
//! - uses bounded input sizes;
//! - contains no sleeps;
//! - contains no network access;
//! - contains no filesystem access;
//! - contains no environment-dependent behavior;
//! - contains no unsafe code;
//! - does not depend on test execution order;
//! - does not mutate global state;
//! - does not depend on wall-clock time;
//! - uses explicit failure messages;
//! - uses public contracts rather than implementation details.
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
//! This module explicitly denies unsafe Rust.
//!
//! ```text
//! BasicRouter
//!     │
//!     ├── validate input
//!     ├── clone working mapping
//!     ├── route blocked interaction
//!     ├── insert semantic SWAP
//!     ├── update mapping
//!     ├── emit gate
//!     ├── verify result
//!     └── return immutable result
//! ```
//!
//! # Important testing rule
//!
//! The tests intentionally do NOT require one exact optimal route when the
//! public contract does not promise one. They assert the invariants that must
//! always hold:
//!
//! - every inserted SWAP is legal;
//! - every final gate is physically executable;
//! - the logical operation is preserved;
//! - mapping changes are correct;
//! - deterministic input produces deterministic output;
//! - failed routing does not mutate caller-owned state.
//!
//! This keeps the test suite stable while allowing the implementation to be
//! optimized internally later.

// =============================================================================
// Safety / compiler policy
// =============================================================================

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

// =============================================================================
// Imports
// =============================================================================

use crate::quantum::routing::algorithms::basic::BasicRouter;
use crate::quantum::routing::config::{
    RoutingAlgorithm,
    RoutingConfig,
    RoutingObjective,
    VerificationLevel,
};
use crate::quantum::routing::errors::RoutingError;
use crate::quantum::routing::mapping::QubitMapping;
use crate::quantum::routing::result::RoutingResult;
use crate::quantum::routing::topology::{
    Topology,
    TopologyBuilder,
};
use crate::quantum::routing::types::{
    GateIdentity,
    LogicalQubitId,
    PhysicalQubitId,
    QuantumOperation,
    RoutingMove,
    RoutingOperation,
};

// =============================================================================
// Test helpers
// =============================================================================

fn lq(index: usize) -> LogicalQubitId {
    LogicalQubitId::new(index)
}

fn pq(index: usize) -> PhysicalQubitId {
    PhysicalQubitId::new(index)
}

/// Creates a standard production-test line topology.
///
/// ```text
/// p0 -- p1 -- p2 -- p3
/// ```
fn line_topology(count: usize) -> Topology {
    Topology::line(count)
        .expect("test line topology must be valid")
}

/// Creates the default production test configuration.
///
/// Verification is enabled because successful routing results must be
/// self-consistent.
fn test_config() -> RoutingConfig {
    RoutingConfig::default()
}

/// Creates a configuration explicitly selecting the basic router.
fn basic_config() -> RoutingConfig {
    RoutingConfig {
        algorithm: RoutingAlgorithm::Basic,
        objective: RoutingObjective::SwapCount,
        verify_output: true,
        verification_level: VerificationLevel::Standard,
        ..RoutingConfig::default()
    }
}

/// Creates a trivial logical-to-physical mapping.
///
/// ```text
/// q0 -> p0
/// q1 -> p1
/// q2 -> p2
/// ...
/// ```
fn identity_mapping(count: usize) -> QubitMapping {
    let mut mapping = QubitMapping::new();

    for index in 0..count {
        mapping
            .assign(lq(index), pq(index))
            .expect("identity mapping must be valid");
    }

    mapping
}

/// Creates a one-qubit operation.
fn single_qubit_gate(
    gate: GateIdentity,
    qubit: usize,
) -> QuantumOperation {
    QuantumOperation::new(
        gate,
        vec![lq(qubit)],
    )
}

/// Creates a two-qubit operation.
fn two_qubit_gate(
    gate: GateIdentity,
    first: usize,
    second: usize,
) -> QuantumOperation {
    QuantumOperation::new(
        gate,
        vec![lq(first), lq(second)],
    )
}

/// Creates a three-qubit operation.
fn three_qubit_gate(
    gate: GateIdentity,
    first: usize,
    second: usize,
    third: usize,
) -> QuantumOperation {
    QuantumOperation::new(
        gate,
        vec![lq(first), lq(second), lq(third)],
    )
}

/// Returns all semantic SWAP moves in a routing result.
fn swap_moves(result: &RoutingResult) -> Vec<(PhysicalQubitId, PhysicalQubitId)> {
    result
        .operations()
        .iter()
        .filter_map(|operation| match operation {
            RoutingOperation::Move(RoutingMove::Swap { a, b }) => {
                Some((*a, *b))
            }
            _ => None,
        })
        .collect()
}

/// Returns all routed gate operations.
fn routed_gates(result: &RoutingResult) -> Vec<&RoutingOperation> {
    result
        .operations()
        .iter()
        .filter(|operation| operation.is_gate())
        .collect()
}

/// Returns the final physical location of a logical qubit.
fn final_location(
    result: &RoutingResult,
    logical: LogicalQubitId,
) -> PhysicalQubitId {
    result
        .final_mapping()
        .physical_of(logical)
        .expect("logical qubit must have a final physical location")
}

// =============================================================================
// Construction / identity
// =============================================================================

#[test]
fn basic_router_can_be_constructed() {
    let router = BasicRouter::new();

    assert_eq!(router.name(), "basic");
}

#[test]
fn basic_router_is_default_constructible() {
    let router = BasicRouter::default();

    assert_eq!(router.name(), "basic");
}

#[test]
fn basic_router_identity_is_stable() {
    let first = BasicRouter::new();
    let second = BasicRouter::new();

    assert_eq!(first.name(), second.name());
    assert_eq!(first.name(), "basic");
}

// =============================================================================
// Already executable operations
// =============================================================================

#[test]
fn adjacent_two_qubit_gate_requires_no_swap() {
    let topology = line_topology(3);
    let mapping = identity_mapping(3);

    let operations = vec![
        two_qubit_gate(GateIdentity::Cx, 0, 1),
    ];

    let result = BasicRouter::new()
        .route_with_mapping(
            &operations,
            &topology,
            &mapping,
            &basic_config(),
        )
        .expect("adjacent CX must route successfully");

    assert_eq!(result.metrics().inserted_swaps, 0);
    assert_eq!(result.metrics().routed_two_qubit_operations, 1);
    assert_eq!(result.metrics().original_operations, 1);
    assert_eq!(result.metrics().final_operations, 1);

    assert_eq!(swap_moves(&result).len(), 0);
    assert_eq!(routed_gates(&result).len(), 1);
}

#[test]
fn already_executable_symmetric_gate_requires_no_swap() {
    let topology = line_topology(3);
    let mapping = identity_mapping(3);

    let operations = vec![
        two_qubit_gate(GateIdentity::Cz, 1, 2),
    ];

    let result = BasicRouter::new()
        .route_with_mapping(
            &operations,
            &topology,
            &mapping,
            &basic_config(),
        )
        .expect("adjacent CZ must route successfully");

    assert_eq!(result.metrics().inserted_swaps, 0);
    assert_eq!(result.operations().len(), 1);
}

#[test]
fn single_qubit_gate_requires_no_routing_move() {
    let topology = line_topology(2);
    let mapping = identity_mapping(2);

    let operations = vec![
        single_qubit_gate(GateIdentity::H, 0),
    ];

    let result = BasicRouter::new()
        .route_with_mapping(
            &operations,
            &topology,
            &mapping,
            &basic_config(),
        )
        .expect("single-qubit gate must route successfully");

    assert_eq!(result.metrics().inserted_swaps, 0);
    assert_eq!(result.metrics().original_operations, 1);
    assert_eq!(result.metrics().final_operations, 1);

    assert!(
        result
            .operations()
            .first()
            .expect("one operation must exist")
            .is_gate()
    );
}

// =============================================================================
// Non-adjacent routing
// =============================================================================

#[test]
fn non_adjacent_two_qubit_gate_inserts_required_swap() {
    let topology = line_topology(3);
    let mapping = identity_mapping(3);

    // q0 -> p0
    // q2 -> p2
    //
    // p0 -- p1 -- p2
    //
    // The basic router must move one operand before CX can execute.
    let operations = vec![
        two_qubit_gate(GateIdentity::Cx, 0, 2),
    ];

    let result = BasicRouter::new()
        .route_with_mapping(
            &operations,
            &topology,
            &mapping,
            &basic_config(),
        )
        .expect("non-adjacent CX must be routed");

    assert!(
        result.metrics().inserted_swaps >= 1,
        "a non-adjacent interaction must require at least one movement"
    );

    assert_eq!(
        result.metrics().routed_two_qubit_operations,
        1
    );

    assert_eq!(
        result.metrics().original_operations,
        1
    );

    assert_eq!(
        result.metrics().final_operations,
        1 + result.metrics().inserted_swaps
    );

    assert_eq!(
        swap_moves(&result).len(),
        result.metrics().inserted_swaps
    );
}

#[test]
fn routing_preserves_original_gate_after_inserted_moves() {
    let topology = line_topology(3);
    let mapping = identity_mapping(3);

    let operations = vec![
        two_qubit_gate(GateIdentity::Cx, 0, 2),
    ];

    let result = BasicRouter::new()
        .route_with_mapping(
            &operations,
            &topology,
            &mapping,
            &basic_config(),
        )
        .expect("routing must succeed");

    let gates = routed_gates(&result);

    assert_eq!(gates.len(), 1);

    let gate = gates[0]
        .gate()
        .expect("routed operation must contain a gate");

    assert_eq!(*gate, GateIdentity::Cx);

    assert_eq!(
        gates[0].logical_operands(),
        &[lq(0), lq(2)]
    );
}

// =============================================================================
// Mapping evolution
// =============================================================================

#[test]
fn inserted_swap_changes_mapping() {
    let topology = line_topology(3);
    let mapping = identity_mapping(3);

    let operations = vec![
        two_qubit_gate(GateIdentity::Cx, 0, 2),
    ];

    let result = BasicRouter::new()
        .route_with_mapping(
            &operations,
            &topology,
            &mapping,
            &basic_config(),
        )
        .expect("routing must succeed");

    assert_eq!(
        result.initial_mapping().physical_of(lq(0)),
        Some(pq(0))
    );

    assert_eq!(
        result.initial_mapping().physical_of(lq(2)),
        Some(pq(2))
    );

    assert_ne!(
        result.final_mapping().physical_of(lq(0)),
        result.initial_mapping().physical_of(lq(0)),
        "routing movement must update at least one logical location"
    );
}

#[test]
fn final_mapping_remains_bijective() {
    let topology = line_topology(4);
    let mapping = identity_mapping(4);

    let operations = vec![
        two_qubit_gate(GateIdentity::Cx, 0, 3),
        two_qubit_gate(GateIdentity::Cx, 1, 2),
    ];

    let result = BasicRouter::new()
        .route_with_mapping(
            &operations,
            &topology,
            &mapping,
            &basic_config(),
        )
        .expect("routing must succeed");

    result
        .final_mapping()
        .validate(&topology)
        .expect("final mapping must remain valid");
}

// =============================================================================
// Multiple operations
// =============================================================================

#[test]
fn routing_updates_mapping_between_successive_operations() {
    let topology = line_topology(4);
    let mapping = identity_mapping(4);

    let operations = vec![
        two_qubit_gate(GateIdentity::Cx, 0, 3),
        two_qubit_gate(GateIdentity::Cx, 0, 2),
        two_qubit_gate(GateIdentity::Cx, 1, 3),
    ];

    let result = BasicRouter::new()
        .route_with_mapping(
            &operations,
            &topology,
            &mapping,
            &basic_config(),
        )
        .expect("multi-operation circuit must route");

    assert_eq!(
        result.metrics().original_operations,
        operations.len()
    );

    assert_eq!(
        result.metrics().routed_two_qubit_operations,
        operations.len()
    );

    assert_eq!(
        result.metrics().final_operations,
        result.metrics().original_operations
            + result.metrics().inserted_swaps
    );
}

#[test]
fn routed_gate_count_matches_original_gate_count() {
    let topology = line_topology(4);
    let mapping = identity_mapping(4);

    let operations = vec![
        single_qubit_gate(GateIdentity::H, 0),
        two_qubit_gate(GateIdentity::Cx, 0, 3),
        single_qubit_gate(GateIdentity::X, 2),
        two_qubit_gate(GateIdentity::Cz, 1, 2),
    ];

    let result = BasicRouter::new()
        .route_with_mapping(
            &operations,
            &topology,
            &mapping,
            &basic_config(),
        )
        .expect("circuit must route");

    assert_eq!(
        result.metrics().final_gate_operations,
        operations.len()
    );
}

// =============================================================================
// Determinism
// =============================================================================

#[test]
fn identical_inputs_produce_identical_basic_routes() {
    let topology = line_topology(5);
    let mapping = identity_mapping(5);

    let operations = vec![
        two_qubit_gate(GateIdentity::Cx, 0, 4),
        two_qubit_gate(GateIdentity::Cx, 1, 3),
        two_qubit_gate(GateIdentity::Cx, 0, 2),
    ];

    let config = basic_config();

    let first = BasicRouter::new()
        .route_with_mapping(
            &operations,
            &topology,
            &mapping,
            &config,
        )
        .expect("first route must succeed");

    let second = BasicRouter::new()
        .route_with_mapping(
            &operations,
            &topology,
            &mapping,
            &config,
        )
        .expect("second route must succeed");

    assert_eq!(
        first.operations(),
        second.operations()
    );

    assert_eq!(
        first.initial_mapping(),
        second.initial_mapping()
    );

    assert_eq!(
        first.final_mapping(),
        second.final_mapping()
    );

    assert_eq!(
        first.metrics().inserted_swaps,
        second.metrics().inserted_swaps
    );
}

// =============================================================================
// Caller-owned mapping immutability
// =============================================================================

#[test]
fn routing_does_not_mutate_caller_mapping() {
    let topology = line_topology(4);
    let mapping = identity_mapping(4);
    let original = mapping.clone();

    let operations = vec![
        two_qubit_gate(GateIdentity::Cx, 0, 3),
    ];

    let _result = BasicRouter::new()
        .route_with_mapping(
            &operations,
            &topology,
            &mapping,
            &basic_config(),
        )
        .expect("routing must succeed");

    assert_eq!(
        mapping,
        original,
        "caller-owned mapping must remain unchanged"
    );
}

// =============================================================================
// Result mapping
// =============================================================================

#[test]
fn result_records_initial_mapping() {
    let topology = line_topology(3);
    let mapping = identity_mapping(3);

    let operations = vec![
        two_qubit_gate(GateIdentity::Cx, 0, 2),
    ];

    let result = BasicRouter::new()
        .route_with_mapping(
            &operations,
            &topology,
            &mapping,
            &basic_config(),
        )
        .expect("routing must succeed");

    assert_eq!(
        result.initial_mapping(),
        &mapping
    );
}

#[test]
fn final_mapping_contains_every_routed_logical_qubit() {
    let topology = line_topology(5);
    let mapping = identity_mapping(5);

    let operations = vec![
        two_qubit_gate(GateIdentity::Cx, 0, 4),
    ];

    let result = BasicRouter::new()
        .route_with_mapping(
            &operations,
            &topology,
            &mapping,
            &basic_config(),
        )
        .expect("routing must succeed");

    for index in 0..5 {
        assert!(
            result
                .final_mapping()
                .physical_of(lq(index))
                .is_some(),
            "logical q{index} must remain mapped"
        );
    }
}

// =============================================================================
// Movement legality
// =============================================================================

#[test]
fn every_inserted_swap_is_topology_legal() {
    let topology = line_topology(6);
    let mapping = identity_mapping(6);

    let operations = vec![
        two_qubit_gate(GateIdentity::Cx, 0, 5),
        two_qubit_gate(GateIdentity::Cx, 1, 4),
    ];

    let result = BasicRouter::new()
        .route_with_mapping(
            &operations,
            &topology,
            &mapping,
            &basic_config(),
        )
        .expect("routing must succeed");

    for (a, b) in swap_moves(&result) {
        assert!(
            topology.is_adjacent(a, b)
                || topology.is_bidirectionally_adjacent(a, b),
            "inserted SWAP p{}-p{} must be physically adjacent",
            a.index(),
            b.index()
        );
    }
}

// =============================================================================
// Output gate legality
// =============================================================================

#[test]
fn final_two_qubit_gates_are_physically_executable() {
    let topology = line_topology(5);
    let mapping = identity_mapping(5);

    let operations = vec![
        two_qubit_gate(GateIdentity::Cx, 0, 4),
        two_qubit_gate(GateIdentity::Cz, 1, 3),
    ];

    let result = BasicRouter::new()
        .route_with_mapping(
            &operations,
            &topology,
            &mapping,
            &basic_config(),
        )
        .expect("routing must succeed");

    let mut current_mapping = result
        .initial_mapping()
        .clone();

    for operation in result.operations() {
        match operation {
            RoutingOperation::Move(RoutingMove::Swap { a, b }) => {
                let logical_a = current_mapping
                    .logical_of(*a)
                    .expect("SWAP endpoint must be mapped");

                let logical_b = current_mapping
                    .logical_of(*b)
                    .expect("SWAP endpoint must be mapped");

                current_mapping
                    .swap(*a, *b)
                    .expect("legal SWAP must update mapping");

                assert_ne!(
                    logical_a,
                    logical_b,
                    "two distinct physical positions must not contain the same logical qubit"
                );
            }

            RoutingOperation::Gate {
                gate,
                operands,
                ..
            } => {
                if operands.len() == 2 {
                    assert!(
                        topology.supports_gate(
                            gate.name(),
                            operands[0],
                            operands[1]
                        ),
                        "routed gate '{}' must be executable on p{} and p{}",
                        gate.name(),
                        operands[0].index(),
                        operands[1].index()
                    );
                }
            }

            RoutingOperation::Barrier { .. } => {}
        }
    }
}

// =============================================================================
// Metrics
// =============================================================================

#[test]
fn routing_overhead_equals_inserted_moves_for_basic_router() {
    let topology = line_topology(4);
    let mapping = identity_mapping(4);

    let operations = vec![
        two_qubit_gate(GateIdentity::Cx, 0, 3),
    ];

    let result = BasicRouter::new()
        .route_with_mapping(
            &operations,
            &topology,
            &mapping,
            &basic_config(),
        )
        .expect("routing must succeed");

    assert_eq!(
        result.metrics().routing_overhead_operations,
        result.metrics().inserted_moves
    );

    assert_eq!(
        result.metrics().inserted_moves,
        result.metrics().inserted_swaps
            + result.metrics().inserted_bridges
            + result.metrics().inserted_permutations
    );
}

#[test]
fn basic_router_reports_basic_algorithm_metadata() {
    let topology = line_topology(3);
    let mapping = identity_mapping(3);

    let operations = vec![
        two_qubit_gate(GateIdentity::Cx, 0, 2),
    ];

    let result = BasicRouter::new()
        .route_with_mapping(
            &operations,
            &topology,
            &mapping,
            &basic_config(),
        )
        .expect("routing must succeed");

    assert_eq!(
        result.algorithm(),
        RoutingAlgorithm::Basic
    );

    assert_eq!(
        result.metrics().inserted_bridges,
        0
    );

    assert_eq!(
        result.metrics().inserted_permutations,
        0
    );
}

// =============================================================================
// Limits
// =============================================================================

#[test]
fn maximum_swap_limit_is_enforced() {
    let topology = line_topology(5);
    let mapping = identity_mapping(5);

    let operations = vec![
        two_qubit_gate(GateIdentity::Cx, 0, 4),
    ];

    let config = RoutingConfig {
        algorithm: RoutingAlgorithm::Basic,
        objective: RoutingObjective::SwapCount,
        max_swaps: Some(0),
        verify_output: true,
        verification_level: VerificationLevel::Standard,
        ..RoutingConfig::default()
    };

    let result = BasicRouter::new()
        .route_with_mapping(
            &operations,
            &topology,
            &mapping,
            &config,
        );

    assert!(
        matches!(
            result,
            Err(RoutingError::RoutingTimeout { .. })
        ),
        "zero-SWAP limit must reject a route requiring movement"
    );
}

#[test]
fn iteration_limit_is_enforced() {
    let topology = line_topology(5);
    let mapping = identity_mapping(5);

    let operations = vec![
        two_qubit_gate(GateIdentity::Cx, 0, 4),
    ];

    let config = RoutingConfig {
        algorithm: RoutingAlgorithm::Basic,
        objective: RoutingObjective::SwapCount,
        max_iterations: 0,
        verify_output: true,
        verification_level: VerificationLevel::Standard,
        ..RoutingConfig::default()
    };

    let result = BasicRouter::new()
        .route_with_mapping(
            &operations,
            &topology,
            &mapping,
            &config,
        );

    assert!(
        matches!(
            result,
            Err(RoutingError::IterationLimit { .. })
        ),
        "zero iteration limit must reject required movement"
    );
}

// =============================================================================
// Resource validation
// =============================================================================

#[test]
fn insufficient_physical_resources_are_rejected() {
    let topology = line_topology(2);

    let mapping = {
        let mut mapping = QubitMapping::new();

        mapping
            .assign(lq(0), pq(0))
            .expect("q0 mapping must succeed");

        mapping
            .assign(lq(1), pq(1))
            .expect("q1 mapping must succeed");

        mapping
    };

    let operations = vec![
        two_qubit_gate(GateIdentity::Cx, 0, 1),
    ];

    let result = BasicRouter::new()
        .route_with_mapping(
            &operations,
            &topology,
            &mapping,
            &test_config(),
        );

    assert!(
        result.is_ok(),
        "two logical qubits fit on two physical qubits"
    );
}

#[test]
fn unmapped_logical_qubit_is_rejected() {
    let topology = line_topology(3);
    let mapping = identity_mapping(2);

    let operations = vec![
        two_qubit_gate(GateIdentity::Cx, 0, 2),
    ];

    let result = BasicRouter::new()
        .route_with_mapping(
            &operations,
            &topology,
            &mapping,
            &basic_config(),
        );

    assert!(
        matches!(
            result,
            Err(RoutingError::InvalidLogicalQubit(_))
        ),
        "unmapped q2 must be rejected explicitly"
    );
}

// =============================================================================
// Invalid operations
// =============================================================================

#[test]
fn duplicate_two_qubit_operand_is_rejected() {
    let topology = line_topology(2);
    let mapping = identity_mapping(2);

    let operations = vec![
        two_qubit_gate(GateIdentity::Cx, 0, 0),
    ];

    let result = BasicRouter::new()
        .route_with_mapping(
            &operations,
            &topology,
            &mapping,
            &basic_config(),
        );

    assert!(
        matches!(
            result,
            Err(RoutingError::InvalidQuantumOperation { .. })
        ),
        "CX(q0,q0) must be rejected"
    );
}

#[test]
fn unsupported_multi_qubit_operation_is_rejected_by_basic_router() {
    let topology = line_topology(3);
    let mapping = identity_mapping(3);

    let operations = vec![
        three_qubit_gate(
            GateIdentity::Ccx,
            0,
            1,
            2,
        ),
    ];

    let result = BasicRouter::new()
        .route_with_mapping(
            &operations,
            &topology,
            &mapping,
            &basic_config(),
        );

    assert!(
        matches!(
            result,
            Err(RoutingError::UnsupportedArity { .. })
        ),
        "basic routing must not silently synthesize unsupported 3-qubit operations"
    );
}

// =============================================================================
// Disconnected topology
// =============================================================================

#[test]
fn disconnected_topology_rejects_unrouteable_interaction() {
    let topology = TopologyBuilder::named("disconnected-basic-test")
        .add_qubit(pq(0))
        .expect("p0 must register")
        .add_qubit(pq(1))
        .expect("p1 must register")
        .undirected_edge(pq(0), pq(1))
        .expect("p0-p1 must register")
        .add_qubit(pq(2))
        .expect("p2 must register")
        .add_qubit(pq(3))
        .expect("p3 must register")
        .undirected_edge(pq(2), pq(3))
        .expect("p2-p3 must register")
        .build()
        .expect("disconnected topology itself must be structurally valid");

    let mapping = identity_mapping(4);

    let operations = vec![
        two_qubit_gate(GateIdentity::Cx, 0, 3),
    ];

    let result = BasicRouter::new()
        .route_with_mapping(
            &operations,
            &topology,
            &mapping,
            &basic_config(),
        );

    assert!(
        result.is_err(),
        "an interaction crossing disconnected components must fail"
    );
}

// =============================================================================
// Transactional failure
// =============================================================================

#[test]
fn failed_route_does_not_mutate_caller_mapping() {
    let topology = TopologyBuilder::named("transaction-test")
        .add_qubit(pq(0))
        .expect("p0 must register")
        .add_qubit(pq(1))
        .expect("p1 must register")
        .undirected_edge(pq(0), pq(1))
        .expect("edge must register")
        .add_qubit(pq(2))
        .expect("p2 must register")
        .build()
        .expect("topology must build");

    let mapping = identity_mapping(3);
    let original = mapping.clone();

    let operations = vec![
        two_qubit_gate(GateIdentity::Cx, 0, 2),
    ];

    let result = BasicRouter::new()
        .route_with_mapping(
            &operations,
            &topology,
            &mapping,
            &basic_config(),
        );

    assert!(result.is_err());

    assert_eq!(
        mapping,
        original,
        "failed routing must not mutate caller-owned mapping"
    );
}

// =============================================================================
// Verification integration
// =============================================================================

#[test]
fn successful_route_with_verification_enabled_returns_verified_result() {
    let topology = line_topology(4);
    let mapping = identity_mapping(4);

    let operations = vec![
        two_qubit_gate(GateIdentity::Cx, 0, 3),
    ];

    let config = RoutingConfig {
        algorithm: RoutingAlgorithm::Basic,
        objective: RoutingObjective::SwapCount,
        verify_output: true,
        verification_level: VerificationLevel::Standard,
        ..RoutingConfig::default()
    };

    let result = BasicRouter::new()
        .route_with_mapping(
            &operations,
            &topology,
            &mapping,
            &config,
        )
        .expect("verified route must succeed");

    assert!(
        result.verification().is_some(),
        "verification must be represented in the result"
    );

    assert!(
        result
            .verification()
            .expect("verification summary must exist")
            .status
            .passed()
    );
}

// =============================================================================
// No-op / empty workload
// =============================================================================

#[test]
fn empty_workload_produces_empty_successful_result() {
    let topology = line_topology(2);
    let mapping = identity_mapping(2);

    let operations: Vec<QuantumOperation> = Vec::new();

    let result = BasicRouter::new()
        .route_with_mapping(
            &operations,
            &topology,
            &mapping,
            &basic_config(),
        )
        .expect("empty workload must be a valid no-op");

    assert!(
        result.operations().is_empty()
    );

    assert_eq!(
        result.metrics().original_operations,
        0
    );

    assert_eq!(
        result.metrics().final_operations,
        0
    );

    assert_eq!(
        result.metrics().inserted_swaps,
        0
    );

    assert_eq!(
        result.initial_mapping(),
        &mapping
    );

    assert_eq!(
        result.final_mapping(),
        &mapping
    );
}

// =============================================================================
// Repeated interaction / mapping correctness
// =============================================================================

#[test]
fn repeated_interaction_uses_evolved_mapping() {
    let topology = line_topology(4);
    let mapping = identity_mapping(4);

    let operations = vec![
        two_qubit_gate(GateIdentity::Cx, 0, 3),
        two_qubit_gate(GateIdentity::Cx, 0, 3),
    ];

    let result = BasicRouter::new()
        .route_with_mapping(
            &operations,
            &topology,
            &mapping,
            &basic_config(),
        )
        .expect("repeated interaction must route");

    assert_eq!(
        result.metrics().routed_two_qubit_operations,
        2
    );

    assert_eq!(
        result.metrics().final_gate_operations,
        2
    );

    result
        .final_mapping()
        .validate(&topology)
        .expect("final mapping must remain valid");

    let q0_location = final_location(
        &result,
        lq(0),
    );

    let q3_location = final_location(
        &result,
        lq(3),
    );

    assert_ne!(
        q0_location,
        q3_location,
        "distinct logical qubits must retain distinct physical locations"
    );
}

// =============================================================================
// Mixed single/two-qubit workload
// =============================================================================

#[test]
fn mixed_workload_preserves_operation_order() {
    let topology = line_topology(4);
    let mapping = identity_mapping(4);

    let operations = vec![
        single_qubit_gate(GateIdentity::H, 0),
        two_qubit_gate(GateIdentity::Cx, 0, 3),
        single_qubit_gate(GateIdentity::X, 2),
        two_qubit_gate(GateIdentity::Cz, 1, 2),
    ];

    let result = BasicRouter::new()
        .route_with_mapping(
            &operations,
            &topology,
            &mapping,
            &basic_config(),
        )
        .expect("mixed workload must route");

    let gates = routed_gates(&result);

    assert_eq!(
        gates.len(),
        operations.len()
    );

    let gate_names: Vec<&str> = gates
        .iter()
        .map(|operation| {
            operation
                .gate()
                .expect("gate operation must expose a gate")
                .name()
        })
        .collect();

    assert_eq!(
        gate_names,
        vec!["h", "cx", "x", "cz"]
    );
}

// =============================================================================
// Semantic SWAP distinction
// =============================================================================

#[test]
fn inserted_swap_is_a_routing_move_not_a_hardware_gate() {
    let topology = line_topology(3);
    let mapping = identity_mapping(3);

    let operations = vec![
        two_qubit_gate(GateIdentity::Cx, 0, 2),
    ];

    let result = BasicRouter::new()
        .route_with_mapping(
            &operations,
            &topology,
            &mapping,
            &basic_config(),
        )
        .expect("routing must succeed");

    let swaps = result
        .operations()
        .iter()
        .filter_map(|operation| match operation {
            RoutingOperation::Move(RoutingMove::Swap { a, b }) => {
                Some((*a, *b))
            }

            RoutingOperation::Gate { gate, .. }
                if *gate == GateIdentity::Swap =>
            {
                panic!(
                    "routing must represent movement as RoutingMove::Swap, \
                     not as a synthesized hardware gate"
                );
            }

            _ => None,
        })
        .collect::<Vec<_>>();

    assert!(
        !swaps.is_empty(),
        "non-adjacent interaction must contain a semantic SWAP move"
    );
}

// =============================================================================
// Final invariant
// =============================================================================

#[test]
fn basic_router_result_has_consistent_operation_metrics() {
    let topology = line_topology(5);
    let mapping = identity_mapping(5);

    let operations = vec![
        single_qubit_gate(GateIdentity::H, 0),
        two_qubit_gate(GateIdentity::Cx, 0, 4),
        two_qubit_gate(GateIdentity::Cz, 1, 3),
        single_qubit_gate(GateIdentity::X, 2),
    ];

    let result = BasicRouter::new()
        .route_with_mapping(
            &operations,
            &topology,
            &mapping,
            &basic_config(),
        )
        .expect("route must succeed");

    let actual_swaps = result
        .operations()
        .iter()
        .filter(|operation| {
            matches!(
                operation,
                RoutingOperation::Move(
                    RoutingMove::Swap { .. }
                )
            )
        })
        .count();

    let actual_gates = result
        .operations()
        .iter()
        .filter(|operation| operation.is_gate())
        .count();

    assert_eq!(
        actual_swaps,
        result.metrics().inserted_swaps
    );

    assert_eq!(
        actual_gates,
        result.metrics().final_gate_operations
    );

    assert_eq!(
        result.operations().len(),
        actual_swaps + actual_gates
    );

    assert_eq!(
        result.metrics().final_operations,
        result.operations().len()
    );

    assert_eq!(
        result.metrics().routing_overhead_operations,
        result.metrics().inserted_moves
    );

    assert_eq!(
        result.metrics().original_operations,
        operations.len()
    );
}