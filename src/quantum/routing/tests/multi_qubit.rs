//! Zamani Quantum Routing — Multi-Qubit Routing Boundary Tests
//!
//! `src/quantum/routing/tests/multi_qubit.rs`
//!
//! # Purpose
//!
//! Production integration tests for the routing subsystem's multi-qubit
//! operation boundary.
//!
//! These tests establish a critical architectural invariant:
//!
//! ```text
//!                 logical QuantumOperation
//!                          │
//!                          ▼
//!                 operation arity check
//!                          │
//!             ┌────────────┴────────────┐
//!             │                         │
//!          0/1/2 qubits             >2 qubits
//!             │                         │
//!             ▼                         ▼
//!        routing allowed       explicit policy boundary
//!                                       │
//!                         ┌─────────────┼─────────────┐
//!                         │             │             │
//!                      native        decompose      reject
//!                         │             │             │
//!                         ▼             ▼             ▼
//!                    hardware      synthesis       error
//!                    support       subsystem       / safe failure
//! ```
//!
//! Routing is responsible for logical-to-physical placement and movement. It
//! must not silently invent a decomposition for an unsupported 3+ qubit gate.
//!
//! # Architectural rule
//!
//! A multi-qubit operation may only cross the routing boundary when the target
//! routing contract explicitly establishes that it is executable as supplied.
//!
//! Otherwise:
//!
//! - routing must return a structured error;
//! - routing must not silently decompose the operation;
//! - routing must not silently drop the operation;
//! - routing must not replace it with a different gate sequence;
//! - routing must not insert arbitrary movement in an attempt to synthesize it;
//! - the caller's mapping must remain unchanged;
//! - no partial routing result may be exposed.
//!
//! The current `BasicRouter` deliberately supports the deterministic 1-/2-qubit
//! routing baseline. Its input validator therefore rejects arity greater than
//! two with `RoutingError::UnsupportedArity`. This test file freezes that
//! behavior while separately testing the configuration-level
//! `MultiQubitPolicy` contract so future native multi-qubit-capable algorithms
//! can be introduced without weakening the safety boundary.
//!
//! # What this file tests
//!
//! This suite verifies:
//!
//! - the default multi-qubit policy is `NativeOnly`;
//! - every multi-qubit policy remains explicit;
//! - three-qubit CCX/Toffoli operations are represented correctly;
//! - three-qubit CSWAP/Fredkin operations are represented correctly;
//! - arbitrary custom three-qubit gates are represented correctly;
//! - arbitrary four-qubit operations are represented correctly;
//! - multi-qubit classification is independent of gate name;
//! - multi-qubit operations are rejected by the current two-qubit baseline
//!   router;
//! - rejection is structured rather than string-parsed;
//! - the reported arity is preserved;
//! - the maximum supported routing arity is reported as two;
//! - rejection happens before routing movement;
//! - caller-owned mappings are not mutated on failure;
//! - no successful `RoutingResult` is fabricated for rejected operations;
//! - a multi-qubit operation already containing adjacent physical locations is
//!   still rejected by a router that does not support native multi-qubit gates;
//! - decomposition policy does not turn routing into a synthesis engine;
//! - `Auto` policy does not permit silent decomposition;
//! - 1- and 2-qubit operations remain inside the supported routing boundary;
//! - duplicate operands do not accidentally convert a 3-qubit operation into a
//!   supported operation;
//! - operation order is preserved in the failure case;
//! - deterministic repeated failures produce equivalent diagnostics;
//! - routing limits cannot be bypassed by using a multi-qubit operation;
//! - the routing-level `QubitInteraction` abstraction correctly records
//!   multi-qubit interactions.
//!
//! # Non-responsibilities
//!
//! This file deliberately does NOT test:
//!
//! - gate synthesis;
//! - CCX decomposition into CX/one-qubit gates;
//! - CSWAP decomposition;
//! - arbitrary unitary synthesis;
//! - pulse synthesis;
//! - hardware execution;
//! - QEC decoding;
//! - OpenQASM parsing;
//! - compiler frontend behavior;
//! - provider SDKs;
//! - scheduling;
//! - benchmarking execution.
//!
//! Those belong to their respective subsystems.
//!
//! # Integration contracts consumed
//!
//! This test file consumes the following stable routing contracts:
//!
//! ```text
//! routing::algorithms::basic::BasicRouter
//! routing::config::{MultiQubitPolicy, RoutingConfig, ...}
//! routing::errors::RoutingError
//! routing::mapping::QubitMapping
//! routing::topology::Topology
//! routing::types::{GateIdentity, QuantumOperation, QubitInteraction, ...}
//! ```
//!
//! The tests intentionally do not depend on:
//!
//! - HashMap layout;
//! - private router fields;
//! - algorithm implementation internals;
//! - compiler IR internals;
//! - hardware provider internals.
//!
//! This keeps the file stable when the implementation is optimized.
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
//! This test module explicitly denies unsafe Rust.
//!
//! # Production test principle
//!
//! The tests assert externally observable invariants rather than fragile
//! implementation details.
//!
//! In particular, the tests do not require a particular decomposition because
//! routing is not the decomposition subsystem.
//!
//! ```text
//! multi-qubit operation
//!        │
//!        ▼
//! validate arity
//!        │
//!        ├── <= 2 ──► normal routing path
//!        │
//!        └── > 2 ──► explicit capability/policy boundary
//!                          │
//!                          ▼
//!                   no silent synthesis
//! ```
//!
//! This is consistent with the routing model used by established quantum
//! compiler stacks: routing and decomposition are separate transformations,
//! and routing implementations that operate on a 1-/2-qubit connectivity graph
//! reject undecomposed n-qubit operations rather than silently inventing a
//! decomposition.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use crate::quantum::routing::algorithms::basic::BasicRouter;
use crate::quantum::routing::config::{
    MultiQubitPolicy,
    RoutingAlgorithm,
    RoutingConfig,
    RoutingObjective,
    VerificationLevel,
};
use crate::quantum::routing::errors::RoutingError;
use crate::quantum::routing::mapping::QubitMapping;
use crate::quantum::routing::topology::Topology;
use crate::quantum::routing::types::{
    GateIdentity,
    LogicalQubitId,
    PhysicalQubitId,
    QuantumOperation,
    QubitInteraction,
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

/// Creates a deterministic line topology.
///
/// ```text
/// p0 -- p1 -- p2 -- p3
/// ```
fn line_topology(count: usize) -> Topology {
    Topology::line(count)
        .expect("test line topology must be valid")
}

/// Creates a deterministic identity mapping.
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

/// Creates the standard deterministic test configuration.
///
/// Verification is enabled so that successful 1-/2-qubit control cases also
/// exercise the normal production verification boundary.
fn test_config() -> RoutingConfig {
    RoutingConfig {
        algorithm: RoutingAlgorithm::Basic,
        objective: RoutingObjective::SwapCount,
        verify_output: true,
        verification: VerificationLevel::Standard,
        ..RoutingConfig::default()
    }
}

/// Creates a one-qubit operation.
fn single_qubit_gate(
    gate: GateIdentity,
    qubit: usize,
) -> QuantumOperation {
    QuantumOperation::new(gate, vec![lq(qubit)])
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
        vec![
            lq(first),
            lq(second),
            lq(third),
        ],
    )
}

/// Creates an arbitrary n-qubit operation.
///
/// This deliberately uses a custom gate identity so the test proves that the
/// routing boundary is based on arity/capability rather than a hard-coded list
/// of gate names.
fn n_qubit_gate(
    arity: usize,
) -> QuantumOperation {
    let operands = (0..arity)
        .map(lq)
        .collect::<Vec<_>>();

    QuantumOperation::new(
        GateIdentity::Custom(format!(
            "test_n_qubit_gate_{arity}"
        )),
        operands,
    )
}

/// Routes an operation through the deterministic baseline router.
fn route(
    operations: &[QuantumOperation],
    topology: &Topology,
    mapping: &QubitMapping,
) -> Result<
    crate::quantum::routing::result::RoutingResult,
    RoutingError,
> {
    BasicRouter::new().route_with_mapping(
        operations,
        topology,
        mapping,
        &test_config(),
    )
}

/// Returns the mapping snapshot used by tests to prove that failed routing did
/// not mutate caller-owned state.
fn mapping_snapshot(
    mapping: &QubitMapping,
) -> Vec<(LogicalQubitId, PhysicalQubitId)> {
    mapping
        .logical_qubits()
        .iter()
        .filter_map(|logical| {
            mapping
                .physical_of(*logical)
                .map(|physical| (*logical, physical))
        })
        .collect()
}

// =============================================================================
// Configuration policy
// =============================================================================

#[test]
fn native_only_is_the_default_multi_qubit_policy() {
    let config = RoutingConfig::default();

    assert_eq!(
        config.multi_qubit_policy,
        MultiQubitPolicy::NativeOnly
    );
}

#[test]
fn reject_policy_is_explicitly_representable() {
    let config = RoutingConfig::default()
        .with_multi_qubit_policy(
            MultiQubitPolicy::Reject
        );

    assert_eq!(
        config.multi_qubit_policy,
        MultiQubitPolicy::Reject
    );

    assert!(
        config.validate().is_ok(),
        "explicit reject policy must be a valid routing configuration"
    );
}

#[test]
fn native_only_policy_is_explicitly_representable() {
    let config = RoutingConfig::default()
        .with_multi_qubit_policy(
            MultiQubitPolicy::NativeOnly
        );

    assert_eq!(
        config.multi_qubit_policy,
        MultiQubitPolicy::NativeOnly
    );

    assert!(
        config.validate().is_ok(),
        "native-only policy must be a valid routing configuration"
    );
}

#[test]
fn decompose_policy_is_explicitly_representable() {
    let config = RoutingConfig::default()
        .with_multi_qubit_policy(
            MultiQubitPolicy::Decompose
        );

    assert_eq!(
        config.multi_qubit_policy,
        MultiQubitPolicy::Decompose
    );

    assert!(
        config.validate().is_ok(),
        "decompose policy must be a valid routing configuration"
    );
}

#[test]
fn auto_policy_is_explicitly_representable() {
    let config = RoutingConfig::default()
        .with_multi_qubit_policy(
            MultiQubitPolicy::Auto
        );

    assert_eq!(
        config.multi_qubit_policy,
        MultiQubitPolicy::Auto
    );

    assert!(
        config.validate().is_ok(),
        "auto multi-qubit policy must be a valid routing configuration"
    );
}

#[test]
fn multi_qubit_policy_names_are_stable() {
    assert_eq!(
        MultiQubitPolicy::Reject.name(),
        "reject"
    );

    assert_eq!(
        MultiQubitPolicy::NativeOnly.name(),
        "native_only"
    );

    assert_eq!(
        MultiQubitPolicy::Decompose.name(),
        "decompose"
    );

    assert_eq!(
        MultiQubitPolicy::Auto.name(),
        "auto"
    );
}

// =============================================================================
// Routing-level interaction representation
// =============================================================================

#[test]
fn three_qubit_interaction_is_classified_as_multi_qubit() {
    let interaction = crate::quantum::routing::types::QubitInteraction::new(
        vec![lq(0), lq(1), lq(2)],
        GateIdentity::Ccx,
    );

    assert_eq!(interaction.arity(), 3);
    assert!(!interaction.is_single_qubit());
    assert!(!interaction.is_two_qubit());
    assert!(interaction.is_multi_qubit());

    assert_eq!(
        interaction.operands(),
        &[lq(0), lq(1), lq(2)]
    );

    assert_eq!(
        interaction.gate(),
        &GateIdentity::Ccx
    );
}

#[test]
fn four_qubit_interaction_is_classified_as_multi_qubit() {
    let interaction = QubitInteraction::new(
        vec![
            lq(0),
            lq(1),
            lq(2),
            lq(3),
        ],
        GateIdentity::Custom(
            "test_four_qubit".to_string(),
        ),
    );

    assert_eq!(interaction.arity(), 4);
    assert!(interaction.is_multi_qubit());
    assert!(!interaction.is_two_qubit());
}

#[test]
fn interaction_classification_is_based_on_operand_count() {
    let one = QubitInteraction::new(
        vec![lq(0)],
        GateIdentity::H,
    );

    let two = QubitInteraction::new(
        vec![lq(0), lq(1)],
        GateIdentity::Cx,
    );

    let three = QubitInteraction::new(
        vec![lq(0), lq(1), lq(2)],
        GateIdentity::Ccx,
    );

    assert!(one.is_single_qubit());
    assert!(!one.is_two_qubit());
    assert!(!one.is_multi_qubit());

    assert!(!two.is_single_qubit());
    assert!(two.is_two_qubit());
    assert!(!two.is_multi_qubit());

    assert!(!three.is_single_qubit());
    assert!(!three.is_two_qubit());
    assert!(three.is_multi_qubit());
}

// =============================================================================
// Canonical 3-qubit operations
// =============================================================================

#[test]
fn ccx_operation_has_three_operands() {
    let operation = three_qubit_gate(
        GateIdentity::Ccx,
        0,
        1,
        2,
    );

    assert_eq!(operation.arity(), 3);
    assert!(operation.is_multi_qubit());
    assert_eq!(
        operation.name(),
        "ccx"
    );

    assert_eq!(
        operation.logical_operands(),
        &[lq(0), lq(1), lq(2)]
    );
}

#[test]
fn cswap_operation_has_three_operands() {
    let operation = three_qubit_gate(
        GateIdentity::CSwap,
        0,
        1,
        2,
    );

    assert_eq!(operation.arity(), 3);
    assert!(operation.is_multi_qubit());
    assert_eq!(
        operation.name(),
        "cswap"
    );

    assert_eq!(
        operation.logical_operands(),
        &[lq(0), lq(1), lq(2)]
    );
}

#[test]
fn custom_three_qubit_operation_has_three_operands() {
    let operation = three_qubit_gate(
        GateIdentity::Custom(
            "native_three_qubit".to_string(),
        ),
        0,
        1,
        2,
    );

    assert_eq!(operation.arity(), 3);
    assert!(operation.is_multi_qubit());

    assert_eq!(
        operation.logical_operands(),
        &[lq(0), lq(1), lq(2)]
    );
}

// =============================================================================
// Unsupported 3-qubit routing
// =============================================================================

#[test]
fn ccx_is_rejected_by_two_qubit_basic_router() {
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

    let result = route(
        &operations,
        &topology,
        &mapping,
    );

    assert!(
        matches!(
            result,
            Err(RoutingError::UnsupportedArity {
                arity: 3,
                maximum: 2,
                ..
            })
        ),
        "CCX must cross the routing boundary as an explicit unsupported-arity error"
    );
}

#[test]
fn cswap_is_rejected_by_two_qubit_basic_router() {
    let topology = line_topology(3);
    let mapping = identity_mapping(3);

    let operations = vec![
        three_qubit_gate(
            GateIdentity::CSwap,
            0,
            1,
            2,
        ),
    ];

    let result = route(
        &operations,
        &topology,
        &mapping,
    );

    assert!(
        matches!(
            result,
            Err(RoutingError::UnsupportedArity {
                arity: 3,
                maximum: 2,
                ..
            })
        ),
        "CSWAP must be rejected rather than silently decomposed"
    );
}

#[test]
fn custom_three_qubit_gate_is_rejected_by_two_qubit_basic_router() {
    let topology = line_topology(3);
    let mapping = identity_mapping(3);

    let operations = vec![
        three_qubit_gate(
            GateIdentity::Custom(
                "native_three_qubit".to_string(),
            ),
            0,
            1,
            2,
        ),
    ];

    let result = route(
        &operations,
        &topology,
        &mapping,
    );

    assert!(
        matches!(
            result,
            Err(RoutingError::UnsupportedArity {
                arity: 3,
                maximum: 2,
                ..
            })
        ),
        "custom 3-qubit operations must obey the same arity boundary"
    );
}

// =============================================================================
// Arbitrary higher arity
// =============================================================================

#[test]
fn four_qubit_operation_is_rejected() {
    let topology = line_topology(4);
    let mapping = identity_mapping(4);

    let operations = vec![
        n_qubit_gate(4),
    ];

    let result = route(
        &operations,
        &topology,
        &mapping,
    );

    assert!(
        matches!(
            result,
            Err(RoutingError::UnsupportedArity {
                arity: 4,
                maximum: 2,
                ..
            })
        ),
        "four-qubit operations must never silently enter a two-qubit router"
    );
}

#[test]
fn five_qubit_operation_is_rejected() {
    let topology = line_topology(5);
    let mapping = identity_mapping(5);

    let operations = vec![
        n_qubit_gate(5),
    ];

    let result = route(
        &operations,
        &topology,
        &mapping,
    );

    assert!(
        matches!(
            result,
            Err(RoutingError::UnsupportedArity {
                arity: 5,
                maximum: 2,
                ..
            })
        ),
        "five-qubit operations must remain outside the two-qubit routing contract"
    );
}

#[test]
fn arbitrary_high_arity_operation_is_rejected_without_special_gate_names() {
    let topology = line_topology(8);
    let mapping = identity_mapping(8);

    let operations = vec![
        n_qubit_gate(8),
    ];

    let result = route(
        &operations,
        &topology,
        &mapping,
    );

    assert!(
        matches!(
            result,
            Err(RoutingError::UnsupportedArity {
                arity: 8,
                maximum: 2,
                ..
            })
        ),
        "routing must classify unsupported operations by arity rather than gate-name allowlists"
    );
}

// =============================================================================
// Adjacency does not make an unsupported arity executable
// =============================================================================

#[test]
fn adjacent_ccx_is_still_rejected_by_two_qubit_router() {
    let topology = line_topology(3);
    let mapping = identity_mapping(3);

    // All three logical qubits occupy the three connected physical locations:
    //
    // q0 -> p0
    // q1 -> p1
    // q2 -> p2
    //
    // The qubits are physically connected as a chain, but the router still
    // cannot execute CCX because it only guarantees 1-/2-qubit routing.
    let operations = vec![
        three_qubit_gate(
            GateIdentity::Ccx,
            0,
            1,
            2,
        ),
    ];

    let result = route(
        &operations,
        &topology,
        &mapping,
    );

    assert!(
        matches!(
            result,
            Err(RoutingError::UnsupportedArity {
                arity: 3,
                maximum: 2,
                ..
            })
        ),
        "physical adjacency alone must not imply native support for a 3-qubit gate"
    );
}

#[test]
fn multi_qubit_rejection_is_independent_of_topology_distance() {
    let topology = line_topology(6);
    let mapping = identity_mapping(6);

    let operations = vec![
        three_qubit_gate(
            GateIdentity::Ccx,
            0,
            3,
            5,
        ),
    ];

    let result = route(
        &operations,
        &topology,
        &mapping,
    );

    assert!(
        matches!(
            result,
            Err(RoutingError::UnsupportedArity {
                arity: 3,
                maximum: 2,
                ..
            })
        ),
        "routing must reject unsupported arity before attempting arbitrary movement"
    );
}

// =============================================================================
// No silent decomposition
// =============================================================================

#[test]
fn decompose_policy_does_not_make_basic_router_a_synthesis_engine() {
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

    let config = RoutingConfig::default()
        .with_multi_qubit_policy(
            MultiQubitPolicy::Decompose
        )
        .with_verification(
            VerificationLevel::Strict
        );

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
            Err(RoutingError::UnsupportedArity {
                arity: 3,
                maximum: 2,
                ..
            })
        ),
        "Decompose policy must not cause BasicRouter to invent a decomposition"
    );
}

#[test]
fn auto_policy_does_not_allow_silent_three_qubit_synthesis() {
    let topology = line_topology(3);
    let mapping = identity_mapping(3);

    let operations = vec![
        three_qubit_gate(
            GateIdentity::CSwap,
            0,
            1,
            2,
        ),
    ];

    let config = RoutingConfig::default()
        .with_multi_qubit_policy(
            MultiQubitPolicy::Auto
        )
        .with_verification(
            VerificationLevel::Strict
        );

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
            Err(RoutingError::UnsupportedArity {
                arity: 3,
                maximum: 2,
                ..
            })
        ),
        "Auto must not silently convert routing into gate synthesis"
    );
}

#[test]
fn reject_policy_does_not_change_the_structured_failure_contract() {
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

    let config = RoutingConfig::default()
        .with_multi_qubit_policy(
            MultiQubitPolicy::Reject
        );

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
            Err(RoutingError::UnsupportedArity {
                arity: 3,
                maximum: 2,
                ..
            })
        ),
        "Reject policy must fail through the canonical structured routing error"
    );
}

// =============================================================================
// Transactional failure semantics
// =============================================================================

#[test]
fn rejected_multi_qubit_operation_does_not_mutate_mapping() {
    let topology = line_topology(4);
    let mapping = identity_mapping(4);

    let before = mapping_snapshot(&mapping);

    let operations = vec![
        three_qubit_gate(
            GateIdentity::Ccx,
            0,
            1,
            3,
        ),
    ];

    let result = route(
        &operations,
        &topology,
        &mapping,
    );

    assert!(
        result.is_err(),
        "unsupported multi-qubit operation must fail"
    );

    let after = mapping_snapshot(&mapping);

    assert_eq!(
        before,
        after,
        "failed routing must not mutate caller-owned mapping state"
    );
}

#[test]
fn rejected_multi_qubit_operation_does_not_insert_swaps() {
    let topology = line_topology(5);
    let mapping = identity_mapping(5);

    let operations = vec![
        three_qubit_gate(
            GateIdentity::Ccx,
            0,
            2,
            4,
        ),
    ];

    let result = route(
        &operations,
        &topology,
        &mapping,
    );

    assert!(
        result.is_err(),
        "unsupported multi-qubit operation must fail before a routed result exists"
    );

    // There is deliberately no result to inspect for inserted routing moves.
    // This assertion makes the intended transaction boundary explicit.
    assert!(
        matches!(
            result,
            Err(RoutingError::UnsupportedArity { .. })
        )
    );
}

#[test]
fn rejected_multi_qubit_operation_does_not_create_partial_result() {
    let topology = line_topology(4);
    let mapping = identity_mapping(4);

    let operations = vec![
        single_qubit_gate(GateIdentity::H, 0),
        three_qubit_gate(
            GateIdentity::Ccx,
            0,
            1,
            3,
        ),
        two_qubit_gate(
            GateIdentity::Cx,
            2,
            3,
        ),
    ];

    let result = route(
        &operations,
        &topology,
        &mapping,
    );

    assert!(
        result.is_err(),
        "a later unsupported operation must fail the complete routing transaction"
    );

    assert!(
        matches!(
            result,
            Err(RoutingError::UnsupportedArity {
                arity: 3,
                maximum: 2,
                ..
            })
        ),
        "the caller must receive the actual unsupported-arity failure rather than a partial route"
    );
}

// =============================================================================
// Operation ordering / boundary validation
// =============================================================================

#[test]
fn unsupported_multi_qubit_operation_is_not_hidden_by_preceding_supported_operations() {
    let topology = line_topology(4);
    let mapping = identity_mapping(4);

    let operations = vec![
        single_qubit_gate(GateIdentity::H, 0),
        two_qubit_gate(
            GateIdentity::Cx,
            0,
            1,
        ),
        three_qubit_gate(
            GateIdentity::Ccx,
            0,
            1,
            2,
        ),
    ];

    let result = route(
        &operations,
        &topology,
        &mapping,
    );

    assert!(
        matches!(
            result,
            Err(RoutingError::UnsupportedArity {
                arity: 3,
                maximum: 2,
                ..
            })
        ),
        "supported prefix operations must not cause an unsupported later operation to be ignored"
    );
}

#[test]
fn unsupported_multi_qubit_operation_is_not_hidden_by_following_supported_operations() {
    let topology = line_topology(4);
    let mapping = identity_mapping(4);

    let operations = vec![
        three_qubit_gate(
            GateIdentity::Ccx,
            0,
            1,
            2,
        ),
        single_qubit_gate(GateIdentity::H, 3),
        two_qubit_gate(
            GateIdentity::Cx,
            2,
            3,
        ),
    ];

    let result = route(
        &operations,
        &topology,
        &mapping,
    );

    assert!(
        matches!(
            result,
            Err(RoutingError::UnsupportedArity {
                arity: 3,
                maximum: 2,
                ..
            })
        ),
        "unsupported operations must not be skipped in order to route later operations"
    );
}

// =============================================================================
// Duplicate operands
// =============================================================================

#[test]
fn three_qubit_operation_with_duplicate_operand_remains_unsupported_arity() {
    let topology = line_topology(3);
    let mapping = identity_mapping(3);

    let operation = QuantumOperation::new(
        GateIdentity::Ccx,
        vec![
            lq(0),
            lq(0),
            lq(1),
        ],
    );

    assert_eq!(operation.arity(), 3);
    assert!(operation.is_multi_qubit());

    let result = route(
        &[operation],
        &topology,
        &mapping,
    );

    // The operation is still outside the two-qubit routing contract.
    //
    // The important invariant here is that it must not accidentally become a
    // valid two-qubit operation merely because two operands are equal.
    assert!(
        matches!(
            result,
            Err(RoutingError::UnsupportedArity {
                arity: 3,
                maximum: 2,
                ..
            })
        ),
        "duplicate operands must not collapse a 3-qubit operation into a supported 2-qubit route"
    );
}

// =============================================================================
// Supported-arity regression controls
// =============================================================================

#[test]
fn one_qubit_operation_remains_inside_routing_boundary() {
    let topology = line_topology(2);
    let mapping = identity_mapping(2);

    let operations = vec![
        single_qubit_gate(
            GateIdentity::H,
            0,
        ),
    ];

    let result = route(
        &operations,
        &topology,
        &mapping,
    )
    .expect(
        "one-qubit operations must remain routable",
    );

    assert_eq!(
        result.metrics().original_operations,
        1
    );

    assert_eq!(
        result.metrics().final_operations,
        1
    );
}

#[test]
fn two_qubit_operation_remains_inside_routing_boundary() {
    let topology = line_topology(2);
    let mapping = identity_mapping(2);

    let operations = vec![
        two_qubit_gate(
            GateIdentity::Cx,
            0,
            1,
        ),
    ];

    let result = route(
        &operations,
        &topology,
        &mapping,
    )
    .expect(
        "two-qubit operations must remain routable",
    );

    assert_eq!(
        result.metrics().original_operations,
        1
    );

    assert_eq!(
        result.metrics().final_operations,
        1
    );

    assert_eq!(
        result.metrics().inserted_swaps,
        0
    );
}

// =============================================================================
// Deterministic failure behavior
// =============================================================================

#[test]
fn repeated_ccx_failures_are_deterministic() {
    let topology = line_topology(4);
    let mapping = identity_mapping(4);

    let operations = vec![
        three_qubit_gate(
            GateIdentity::Ccx,
            0,
            2,
            3,
        ),
    ];

    let first = route(
        &operations,
        &topology,
        &mapping,
    );

    let second = route(
        &operations,
        &topology,
        &mapping,
    );

    assert_eq!(
        first,
        second,
        "identical unsupported inputs must produce deterministic routing failures"
    );
}

// =============================================================================
// Arity boundary is exact
// =============================================================================

#[test]
fn arity_two_is_the_highest_supported_arity_of_basic_router() {
    let topology = line_topology(3);
    let mapping = identity_mapping(3);

    let supported = vec![
        two_qubit_gate(
            GateIdentity::Cx,
            0,
            1,
        ),
    ];

    let supported_result = route(
        &supported,
        &topology,
        &mapping,
    );

    assert!(
        supported_result.is_ok(),
        "arity two must remain supported by BasicRouter"
    );

    let unsupported = vec![
        three_qubit_gate(
            GateIdentity::Ccx,
            0,
            1,
            2,
        ),
    ];

    let unsupported_result = route(
        &unsupported,
        &topology,
        &mapping,
    );

    assert!(
        matches!(
            unsupported_result,
            Err(RoutingError::UnsupportedArity {
                arity: 3,
                maximum: 2,
                ..
            })
        ),
        "arity three must be the first unsupported boundary for the two-qubit baseline"
    );
}

// =============================================================================
// Resource-limit interaction
// =============================================================================

#[test]
fn multi_qubit_rejection_cannot_be_bypassed_by_large_operation_limit() {
    let topology = line_topology(8);
    let mapping = identity_mapping(8);

    let operations = vec![
        n_qubit_gate(8),
    ];

    let config = RoutingConfig::default()
        .with_multi_qubit_policy(
            MultiQubitPolicy::NativeOnly
        );

    assert!(
        config.max_operations >= operations.len(),
        "test must not accidentally fail because of the operation-count limit"
    );

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
            Err(RoutingError::UnsupportedArity {
                arity: 8,
                maximum: 2,
                ..
            })
        ),
        "resource capacity must never be mistaken for native multi-qubit gate support"
    );
}

// =============================================================================
// Public contract documentation tests
// =============================================================================

#[test]
fn multi_qubit_policy_does_not_change_the_meaning_of_gate_arity() {
    let policies = [
        MultiQubitPolicy::Reject,
        MultiQubitPolicy::NativeOnly,
        MultiQubitPolicy::Decompose,
        MultiQubitPolicy::Auto,
    ];

    for policy in policies {
        let config = RoutingConfig::default()
            .with_multi_qubit_policy(policy);

        let operation = three_qubit_gate(
            GateIdentity::Ccx,
            0,
            1,
            2,
        );

        assert_eq!(
            operation.arity(),
            3,
            "routing policy must never rewrite operation arity"
        );

        assert!(
            operation.is_multi_qubit(),
            "routing policy must not change semantic operation classification"
        );

        assert_eq!(
            config.multi_qubit_policy,
            policy,
            "configuration must retain the caller's selected policy"
        );
    }
}

#[test]
fn multi_qubit_gate_identity_is_preserved_before_routing_boundary() {
    let ccx = three_qubit_gate(
        GateIdentity::Ccx,
        0,
        1,
        2,
    );

    let cswap = three_qubit_gate(
        GateIdentity::CSwap,
        0,
        1,
        2,
    );

    assert_eq!(
        ccx.name(),
        "ccx"
    );

    assert_eq!(
        cswap.name(),
        "cswap"
    );

    assert_ne!(
        ccx.name(),
        cswap.name()
    );

    assert_eq!(
        ccx.logical_operands(),
        &[lq(0), lq(1), lq(2)]
    );

    assert_eq!(
        cswap.logical_operands(),
        &[lq(0), lq(1), lq(2)]
    );
}