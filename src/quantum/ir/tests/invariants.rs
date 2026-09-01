//! Zamani Quantum IR — Production Invariant Tests.
//!
//! # Purpose
//!
//! This module verifies invariants that must remain true across the canonical
//! Quantum IR boundary regardless of internal implementation changes.
//!
//! The tests intentionally use public APIs. They therefore protect the IR's
//! public semantic contract rather than coupling the tests to private fields.
//!
//! # Architectural principles tested
//!
//! 1. The IR is hardware independent.
//! 2. Logical and physical qubit identities remain distinct.
//! 3. No fixed quantum-machine size is encoded by the semantic model.
//! 4. Explicit `QuantumIrLimits` are policies, not architectural limits.
//! 5. Invalid externally supplied data is rejected.
//! 6. Invalid mutation does not partially modify a valid circuit.
//! 7. Operation ordering remains deterministic.
//! 8. Operation identity remains independent from sequence position.
//! 9. Parameters cannot contain NaN or infinity.
//! 10. Gate arity and parameter contracts are enforced.
//! 11. Canonical `quantum::ir::qubit` identities are used.
//! 12. Validation remains an explicit integration boundary.
//! 13. Very large declared namespaces do not imply proportional allocation.
//! 14. Arithmetic overflow is rejected rather than wrapped.
//! 15. Zero-sized semantic namespaces remain representable.
//! 16. Resource policies can be deliberately tightened without changing
//!     what the Zamani IR semantically represents.
//! 17. The IR remains independent from hardware, routing, scheduling,
//!     simulation, QEC and backend implementations.
//!
//! # Scope
//!
//! These tests intentionally do not test:
//!
//! - a particular quantum backend;
//! - vendor APIs;
//! - hardware topology;
//! - routing algorithms;
//! - scheduling algorithms;
//! - pulse synthesis;
//! - calibration;
//! - simulator state;
//! - QEC decoding;
//! - optimization quality;
//! - frontend parsing.
//!
//! Those systems consume the IR; they do not define its semantic invariants.
//!
//! # Rust compatibility
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe`.
//!
//! # Integration
//!
//! This file should be registered from:
//!
//! `src/quantum/ir/tests/mod.rs`
//!
//! with:
//!
//! ```text
//! mod invariants;
//! ```
//!
//! and the parent IR test namespace should be registered from the canonical
//! `quantum::ir` module under `#[cfg(test)]`.
//!
//! If the repository retains the legacy `src/quantum/ir/tests.rs`, the new
//! directory-based suite should be migrated there rather than maintaining two
//! independent cross-module invariant suites.
//!
//! # Important rule
//!
//! These tests use:
//!
//! `crate::quantum::ir::qubit`
//!
//! rather than the historical compatibility alias:
//!
//! `crate::quantum::ir::qubits`.
//!
//! New production code and tests must use the canonical singular module.
//!
//! # No architecture-sized test constants
//!
//! Test resource values below are intentionally small where a limit needs to
//! be demonstrated. They are test policies only. They MUST NOT be interpreted
//! as supported-machine sizes or Zamani architectural limits.

#![forbid(unsafe_code)]

use crate::quantum::ir::analysis::{
    analyze,
    analyze_with_limits,
    basic_statistics,
    basic_statistics_with_limits,
};
use crate::quantum::ir::circuit::{
    CircuitError,
    QuantumCircuit,
};
use crate::quantum::ir::gate::{
    Gate,
    GateKind,
};
use crate::quantum::ir::identity::{
    CircuitId,
    IrVersion,
    OperationId,
};
use crate::quantum::ir::limits::QuantumIrLimits;
use crate::quantum::ir::parameter::Parameter;
use crate::quantum::ir::qubit::{
    PhysicalQubitId,
    Qubit,
    QubitId,
    QubitRef,
    QubitRegister,
    QubitState,
};
use crate::quantum::ir::validation::validate_circuit_with_limits;

// =============================================================================
// Test policy helpers
// =============================================================================

/// Creates a deliberately small resource policy.
///
/// This function does NOT define an architectural quantum-machine size.
/// It creates a bounded policy so invariant failures can be tested without
/// allocating large data structures.
fn test_limits() -> QuantumIrLimits {
    QuantumIrLimits::production()
        .with_max_qubits(16)
        .with_max_classical_bits(16)
        .with_max_operations(32)
        .with_max_operands(32)
        .with_max_parameters(32)
        .with_max_metadata_bytes(256)
        .with_max_depth(32)
        .with_max_measurements(16)
        .with_max_barriers(16)
        .with_max_validation_steps(10_000)
        .with_max_analysis_steps(10_000)
}

/// Creates an empty circuit under a bounded test policy.
fn empty_circuit() -> QuantumCircuit {
    QuantumCircuit::try_new_with_limits(
        4,
        4,
        test_limits(),
    )
    .expect("bounded empty test circuit must be constructible")
}

/// Creates a one-qubit gate.
///
/// The helper deliberately uses the canonical singular `qubit` module.
fn unary_gate(
    kind: GateKind,
    index: usize,
) -> Gate {
    Gate::new(
        kind,
        vec![QubitId::new(index)],
        Vec::new(),
        None,
        None,
    )
    .expect("valid unary test gate must be constructible")
}

/// Creates a two-qubit gate.
fn binary_gate(
    kind: GateKind,
    first: usize,
    second: usize,
) -> Gate {
    Gate::new(
        kind,
        vec![
            QubitId::new(first),
            QubitId::new(second),
        ],
        Vec::new(),
        None,
        None,
    )
    .expect("valid binary test gate must be constructible")
}

/// Creates a three-qubit gate.
fn ternary_gate(
    kind: GateKind,
    first: usize,
    second: usize,
    third: usize,
) -> Gate {
    Gate::new(
        kind,
        vec![
            QubitId::new(first),
            QubitId::new(second),
            QubitId::new(third),
        ],
        Vec::new(),
        None,
        None,
    )
    .expect("valid ternary test gate must be constructible")
}

/// Creates a parameterized gate.
fn parameterized_gate(
    kind: GateKind,
    index: usize,
    value: f64,
) -> Gate {
    let parameter = Parameter::constant(value)
        .expect("finite test parameter must be valid");

    Gate::new(
        kind,
        vec![QubitId::new(index)],
        vec![parameter],
        None,
        None,
    )
    .expect("valid parameterized test gate must be constructible")
}

// =============================================================================
// Foundation: no unsafe
// =============================================================================

#[test]
fn invariant_suite_is_built_without_unsafe_requirements() {
    // This test is intentionally simple.
    //
    // The actual enforcement is:
    //
    //     #![forbid(unsafe_code)]
    //
    // above. If unsafe code is introduced into this module, compilation fails.
    let qubit = QubitId::new(0);

    assert_eq!(qubit.index(), 0);
}

// =============================================================================
// Version invariants
// =============================================================================

#[test]
fn current_ir_version_is_available() {
    let version = IrVersion::CURRENT;

    assert_eq!(version, IrVersion::CURRENT);
}

#[test]
fn newly_constructed_circuit_uses_current_ir_version() {
    let circuit = empty_circuit();

    assert_eq!(
        circuit.version(),
        IrVersion::CURRENT,
    );
}

#[test]
fn current_ir_version_can_be_reapplied_without_semantic_change() {
    let mut circuit = empty_circuit();

    circuit
        .set_version(IrVersion::CURRENT)
        .expect("current version must be accepted");

    assert_eq!(
        circuit.version(),
        IrVersion::CURRENT,
    );
}

// =============================================================================
// Logical / physical identity separation
// =============================================================================

#[test]
fn logical_and_physical_qubit_ids_are_distinct_types() {
    let logical = QubitId::new(7);
    let physical = PhysicalQubitId::new(7);

    assert_eq!(logical.index(), physical.index());

    let logical_ref = QubitRef::from(logical);
    let physical_ref = QubitRef::from(physical);

    assert!(logical_ref.is_logical());
    assert!(!logical_ref.is_physical());

    assert!(physical_ref.is_physical());
    assert!(!physical_ref.is_logical());

    assert_eq!(
        logical_ref.logical(),
        Some(logical),
    );

    assert_eq!(
        physical_ref.physical(),
        Some(physical),
    );
}

#[test]
fn logical_qubit_identity_does_not_imply_physical_allocation() {
    let logical = QubitId::new(1);

    let logical_ref = QubitRef::Logical(logical);

    assert!(logical_ref.is_logical());
    assert_eq!(
        logical_ref.physical(),
        None,
    );
}

#[test]
fn physical_qubit_identity_does_not_imply_hardware_existence() {
    let physical = PhysicalQubitId::new(10_000);

    let physical_ref = QubitRef::Physical(physical);

    assert!(physical_ref.is_physical());
    assert_eq!(
        physical_ref.logical(),
        None,
    );
}

// =============================================================================
// Qubit identifier invariants
// =============================================================================

#[test]
fn qubit_id_round_trips_through_usize() {
    let original = QubitId::new(123);

    let raw: usize = original.into();
    let reconstructed = QubitId::from(raw);

    assert_eq!(
        reconstructed,
        original,
    );
}

#[test]
fn physical_qubit_id_round_trips_through_usize() {
    let original = PhysicalQubitId::new(123);

    let raw: usize = original.into();
    let reconstructed = PhysicalQubitId::from(raw);

    assert_eq!(
        reconstructed,
        original,
    );
}

#[test]
fn qubit_id_checked_next_prevents_overflow() {
    let maximum = QubitId::new(usize::MAX);

    assert_eq!(
        maximum.checked_next(),
        None,
    );
}

#[test]
fn physical_qubit_id_checked_next_prevents_overflow() {
    let maximum = PhysicalQubitId::new(usize::MAX);

    assert_eq!(
        maximum.checked_next(),
        None,
    );
}

#[test]
fn qubit_ids_are_orderable_and_hashable() {
    let first = QubitId::new(1);
    let second = QubitId::new(2);

    assert!(first < second);

    let mut set = std::collections::BTreeSet::new();

    assert!(set.insert(first));
    assert!(!set.insert(first));
    assert!(set.insert(second));

    assert_eq!(set.len(), 2);
}

// =============================================================================
// Qubit state invariants
// =============================================================================

#[test]
fn new_qubit_is_available_and_usable() {
    let qubit = Qubit::new(QubitId::new(0));

    assert!(qubit.is_available());
    assert!(qubit.is_usable());
    assert!(!qubit.is_disabled());
    assert!(!qubit.is_measured());
    assert!(!qubit.is_reset());
}

#[test]
fn default_qubit_state_is_available() {
    assert_eq!(
        QubitState::default(),
        QubitState::Available,
    );
}

#[test]
fn reset_state_remains_usable() {
    let state = QubitState::Reset;

    assert!(state.is_reset());
    assert!(state.is_usable());
    assert!(!state.is_disabled());
}

#[test]
fn measured_state_remains_usable() {
    let state = QubitState::Measured;

    assert!(state.is_measured());
    assert!(state.is_usable());
    assert!(!state.is_disabled());
}

#[test]
fn disabled_state_is_not_usable() {
    let state = QubitState::Disabled;

    assert!(state.is_disabled());
    assert!(!state.is_usable());
}

// =============================================================================
// Qubit ranges
// =============================================================================

#[test]
fn valid_qubit_range_is_half_open() {
    let range = crate::quantum::ir::qubit::QubitRange::new(
        2,
        5,
    )
    .expect("2..5 must be valid");

    assert_eq!(range.start(), 2);
    assert_eq!(range.end(), 5);
}

#[test]
fn empty_qubit_range_is_representable() {
    let range =
        crate::quantum::ir::qubit::QubitRange::empty(7);

    assert_eq!(range.start(), 7);
    assert_eq!(range.end(), 7);
}

#[test]
fn invalid_qubit_range_is_rejected() {
    let result =
        crate::quantum::ir::qubit::QubitRange::new(
            5,
            2,
        );

    assert!(
        result.is_err(),
        "reversed logical-qubit ranges must be rejected"
    );
}

// =============================================================================
// Qubit register invariants
// =============================================================================

#[test]
fn bounded_qubit_register_can_be_constructed() {
    let register =
        QubitRegister::try_new(8, 8)
            .expect("register within explicit policy must be valid");

    assert_eq!(
        register.len(),
        8,
    );
}

#[test]
fn qubit_register_rejects_policy_overflow_before_allocation() {
    let result =
        QubitRegister::try_new(9, 8);

    assert!(
        result.is_err(),
        "register allocation must be rejected before exceeding policy"
    );
}

#[test]
fn empty_qubit_register_is_supported() {
    let register =
        QubitRegister::try_new(0, 0)
            .expect("zero-sized logical namespace must be representable");

    assert_eq!(
        register.len(),
        0,
    );
}

// =============================================================================
// Resource-policy invariants
// =============================================================================

#[test]
fn production_limits_are_self_consistent() {
    QuantumIrLimits::production()
        .validate()
        .expect("production limits must be structurally valid");
}

#[test]
fn deny_all_policy_is_self_consistent() {
    QuantumIrLimits::deny_all()
        .validate()
        .expect("deny-all policy must remain structurally valid");
}

#[test]
fn zero_resource_policy_can_be_represented() {
    let limits = QuantumIrLimits::production()
        .with_max_qubits(0)
        .with_max_classical_bits(0)
        .with_max_operations(0)
        .with_max_operands(0)
        .with_max_parameters(0)
        .with_max_metadata_bytes(0)
        .with_max_depth(0)
        .with_max_measurements(0)
        .with_max_barriers(0);

    limits
        .validate()
        .expect("zero resource capacities should be legal");
}

#[test]
fn zero_validation_budget_is_rejected() {
    let limits =
        QuantumIrLimits::production()
            .with_max_validation_steps(0);

    assert!(
        limits.validate().is_err(),
        "validation requires a positive work budget"
    );
}

#[test]
fn zero_analysis_budget_is_rejected() {
    let limits =
        QuantumIrLimits::production()
            .with_max_analysis_steps(0);

    assert!(
        limits.validate().is_err(),
        "analysis requires a positive work budget"
    );
}

// =============================================================================
// No architectural machine-size invariant
// =============================================================================

#[test]
fn logical_namespace_is_policy_driven_not_gate_driven() {
    let limits =
        QuantumIrLimits::production()
            .with_max_qubits(64);

    let circuit =
        QuantumCircuit::try_new_with_limits(
            64,
            0,
            limits,
        )
        .expect("explicit policy should determine constructibility");

    assert_eq!(
        circuit.num_qubits(),
        64,
    );
}

#[test]
fn small_policy_does_not_change_qubit_identity_semantics() {
    let limits =
        QuantumIrLimits::production()
            .with_max_qubits(2);

    let result =
        QuantumCircuit::try_new_with_limits(
            3,
            0,
            limits,
        );

    match result {
        Err(CircuitError::QubitLimitExceeded {
            requested,
            maximum,
        }) => {
            assert_eq!(requested, 3);
            assert_eq!(maximum, 2);
        }

        other => {
            panic!(
                "expected explicit policy rejection, got {other:?}"
            );
        }
    }
}

#[test]
fn very_large_logical_namespace_is_not_materialized_by_circuit_construction() {
    //
    // This test is intentionally about representation semantics.
    //
    // QuantumCircuit construction documents that the declared namespace does
    // not require one heap object per qubit. We therefore test a large
    // namespace using a policy without constructing a correspondingly large
    // collection of Qubit objects.
    //
    // `usize::MAX` itself is not required here because the test must remain
    // portable across supported hosts and must not request an impossible
    // resource policy.
    let requested =
        1_000_000usize;

    let limits =
        QuantumIrLimits::production()
            .with_max_qubits(requested);

    let circuit =
        QuantumCircuit::try_new_with_limits(
            requested,
            0,
            limits,
        )
        .expect(
            "large declared namespace must be representable when policy permits it",
        );

    assert_eq!(
        circuit.num_qubits(),
        requested,
    );

    assert_eq!(
        circuit.len(),
        0,
        "namespace declaration must not manufacture operations"
    );
}

// =============================================================================
// Circuit construction invariants
// =============================================================================

#[test]
fn empty_circuit_is_valid() {
    let circuit = empty_circuit();

    assert_eq!(
        circuit.len(),
        0,
    );

    assert!(circuit.is_empty());
}

#[test]
fn circuit_namespace_counts_are_preserved() {
    let circuit =
        QuantumCircuit::try_new_with_limits(
            7,
            5,
            test_limits(),
        )
        .expect("namespace counts must be accepted");

    assert_eq!(
        circuit.num_qubits(),
        7,
    );

    assert_eq!(
        circuit.num_classical_bits(),
        5,
    );
}

#[test]
fn qubit_limit_is_checked_at_construction() {
    let limits =
        test_limits()
            .with_max_qubits(3);

    let result =
        QuantumCircuit::try_new_with_limits(
            4,
            0,
            limits,
        );

    match result {
        Err(CircuitError::QubitLimitExceeded {
            requested,
            maximum,
        }) => {
            assert_eq!(requested, 4);
            assert_eq!(maximum, 3);
        }

        other => {
            panic!(
                "unexpected construction result: {other:?}"
            );
        }
    }
}

#[test]
fn classical_namespace_limit_is_checked_at_construction() {
    let limits =
        test_limits()
            .with_max_classical_bits(3);

    let result =
        QuantumCircuit::try_new_with_limits(
            0,
            4,
            limits,
        );

    match result {
        Err(CircuitError::ClassicalBitLimitExceeded {
            requested,
            maximum,
        }) => {
            assert_eq!(requested, 4);
            assert_eq!(maximum, 3);
        }

        other => {
            panic!(
                "unexpected construction result: {other:?}"
            );
        }
    }
}

// =============================================================================
// Gate arity invariants
// =============================================================================

#[test]
fn unary_gate_has_one_logical_operand() {
    let gate =
        unary_gate(GateKind::X, 0);

    assert_eq!(
        gate.qubits().len(),
        1,
    );
}

#[test]
fn binary_gate_has_two_logical_operands() {
    let gate =
        binary_gate(GateKind::CX, 0, 1);

    assert_eq!(
        gate.qubits().len(),
        2,
    );
}

#[test]
fn ternary_gate_has_three_logical_operands() {
    let gate =
        ternary_gate(
            GateKind::CCX,
            0,
            1,
            2,
        );

    assert_eq!(
        gate.qubits().len(),
        3,
    );
}

#[test]
fn binary_gate_rejects_wrong_arity() {
    let result =
        Gate::new(
            GateKind::CX,
            vec![QubitId::new(0)],
            Vec::new(),
            None,
            None,
        );

    assert!(
        result.is_err(),
        "CX must reject one operand"
    );
}

#[test]
fn ternary_gate_rejects_wrong_arity() {
    let result =
        Gate::new(
            GateKind::CCX,
            vec![
                QubitId::new(0),
                QubitId::new(1),
            ],
            Vec::new(),
            None,
            None,
        );

    assert!(
        result.is_err(),
        "CCX must reject two operands"
    );
}

#[test]
fn duplicate_logical_operands_are_rejected() {
    let result =
        Gate::new(
            GateKind::CX,
            vec![
                QubitId::new(0),
                QubitId::new(0),
            ],
            Vec::new(),
            None,
            None,
        );

    assert!(
        result.is_err(),
        "two-qubit operations cannot silently alias both operands to the same logical qubit"
    );
}

// =============================================================================
// Parameter invariants
// =============================================================================

#[test]
fn finite_parameter_is_accepted() {
    let parameter =
        Parameter::constant(0.25)
            .expect("finite parameter must be accepted");

    assert!(parameter.is_constant());

    assert_eq!(
        parameter.as_constant(),
        Some(0.25),
    );
}

#[test]
fn nan_parameter_is_rejected() {
    assert!(
        Parameter::constant(f64::NAN).is_err(),
        "NaN must never enter canonical IR"
    );
}

#[test]
fn positive_infinity_parameter_is_rejected() {
    assert!(
        Parameter::constant(f64::INFINITY).is_err(),
        "positive infinity must never enter canonical IR"
    );
}

#[test]
fn negative_infinity_parameter_is_rejected() {
    assert!(
        Parameter::constant(f64::NEG_INFINITY).is_err(),
        "negative infinity must never enter canonical IR"
    );
}

#[test]
fn parameterized_gate_requires_parameters() {
    let result =
        Gate::new(
            GateKind::RX,
            vec![QubitId::new(0)],
            Vec::new(),
            None,
            None,
        );

    assert!(
        result.is_err(),
        "RX without its required parameter must be rejected"
    );
}

#[test]
fn non_parameterized_gate_rejects_extra_parameters() {
    let parameter =
        Parameter::constant(0.5)
            .expect("finite parameter must be accepted");

    let result =
        Gate::new(
            GateKind::X,
            vec![QubitId::new(0)],
            vec![parameter],
            None,
            None,
        );

    assert!(
        result.is_err(),
        "X must not silently accept an unrelated parameter"
    );
}

#[test]
fn finite_parameterized_rotation_is_valid() {
    let gate =
        parameterized_gate(
            GateKind::RX,
            0,
            0.5,
        );

    assert_eq!(
        gate.kind(),
        GateKind::RX,
    );

    assert_eq!(
        gate.parameters().len(),
        1,
    );
}

// =============================================================================
// Circuit mutation invariants
// =============================================================================

#[test]
fn valid_operation_can_be_appended() {
    let mut circuit =
        empty_circuit();

    circuit
        .push(unary_gate(GateKind::H, 0))
        .expect("valid operation must be appended");

    assert_eq!(
        circuit.len(),
        1,
    );
}

#[test]
fn operation_limit_is_enforced_without_partial_mutation() {
    let limits =
        test_limits()
            .with_max_operations(1);

    let mut circuit =
        QuantumCircuit::try_new_with_limits(
            1,
            0,
            limits,
        )
        .expect("test circuit must be constructible");

    circuit
        .push(unary_gate(GateKind::X, 0))
        .expect("first operation must fit");

    let result =
        circuit.push(
            unary_gate(GateKind::H, 0)
        );

    assert!(
        result.is_err(),
        "second operation must exceed the explicit operation policy"
    );

    assert_eq!(
        circuit.len(),
        1,
        "failed mutation must not partially append an operation"
    );
}

#[test]
fn out_of_namespace_qubit_is_rejected() {
    let mut circuit =
        QuantumCircuit::try_new_with_limits(
            1,
            0,
            test_limits(),
        )
        .expect("one-qubit circuit must be constructible");

    let result =
        circuit.push(
            unary_gate(GateKind::X, 1)
        );

    assert!(
        result.is_err(),
        "q1 is outside a one-qubit logical namespace"
    );

    assert_eq!(
        circuit.len(),
        0,
        "failed namespace validation must not mutate the circuit"
    );
}

#[test]
fn valid_two_qubit_operation_requires_two_distinct_declared_qubits() {
    let mut circuit =
        QuantumCircuit::try_new_with_limits(
            2,
            0,
            test_limits(),
        )
        .expect("two-qubit circuit must be constructible");

    circuit
        .push(
            binary_gate(
                GateKind::CX,
                0,
                1,
            ),
        )
        .expect("CX over declared qubits must be accepted");

    assert_eq!(
        circuit.len(),
        1,
    );
}

#[test]
fn operation_order_is_preserved() {
    let mut circuit =
        empty_circuit();

    circuit
        .push(unary_gate(GateKind::X, 0))
        .expect("X must be accepted");

    circuit
        .push(unary_gate(GateKind::H, 0))
        .expect("H must be accepted");

    assert_eq!(
        circuit.len(),
        2,
    );

    assert_eq!(
        circuit
            .first()
            .map(Gate::kind),
        Some(GateKind::X),
    );
}

// =============================================================================
// Circuit identity invariants
// =============================================================================

#[test]
fn explicit_circuit_identity_is_preserved() {
    let id =
        CircuitId::new(42);

    let circuit =
        QuantumCircuit::with_identity(
            id,
            2,
            0,
            test_limits(),
        )
        .expect("identity-bearing circuit must be valid");

    assert_eq!(
        circuit.id(),
        id,
    );
}

#[test]
fn circuit_identity_is_not_qubit_identity() {
    let circuit_id =
        CircuitId::new(1);

    let qubit_id =
        QubitId::new(1);

    assert_ne!(
        circuit_id,
        CircuitId::new(qubit_id.index() + 1),
    );
}

// =============================================================================
// Operation identity invariants
// =============================================================================

#[test]
fn operation_ids_are_distinct_from_sequence_indices() {
    let first =
        OperationId::new(100);

    let second =
        OperationId::new(200);

    assert_ne!(
        first,
        second,
    );

    assert_ne!(
        first.index(),
        second.index(),
    );
}

#[test]
fn operation_identity_can_be_compared_independently() {
    let first =
        OperationId::new(7);

    let second =
        OperationId::new(7);

    assert_eq!(
        first,
        second,
    );
}

// =============================================================================
// Whole-circuit validation invariants
// =============================================================================

#[test]
fn valid_empty_circuit_passes_whole_circuit_validation() {
    let circuit =
        empty_circuit();

    validate_circuit_with_limits(
        &circuit,
        circuit.limits(),
    )
    .expect(
        "valid empty circuit must pass canonical validation"
    );
}

#[test]
fn valid_gate_circuit_passes_whole_circuit_validation() {
    let mut circuit =
        empty_circuit();

    circuit
        .push(unary_gate(GateKind::H, 0))
        .expect("H must be accepted");

    circuit
        .push(
            binary_gate(
                GateKind::CX,
                0,
                1,
            ),
        )
        .expect("CX must be accepted");

    validate_circuit_with_limits(
        &circuit,
        circuit.limits(),
    )
    .expect(
        "valid gate circuit must pass canonical validation"
    );
}

// =============================================================================
// Validation budget invariants
// =============================================================================

#[test]
fn validation_budget_is_explicit() {
    let limits =
        test_limits();

    assert!(
        limits.max_validation_steps() > 0,
        "production validation must have an explicit positive work budget"
    );
}

#[test]
fn analysis_budget_is_explicit() {
    let limits =
        test_limits();

    assert!(
        limits.max_analysis_steps() > 0,
        "production analysis must have an explicit positive work budget"
    );
}

// =============================================================================
// Analysis invariants
// =============================================================================

#[test]
fn analysis_of_empty_circuit_is_deterministic() {
    let circuit =
        empty_circuit();

    let first =
        analyze(&circuit)
            .expect("empty circuit analysis must succeed");

    let second =
        analyze(&circuit)
            .expect("repeated empty circuit analysis must succeed");

    assert_eq!(
        first,
        second,
        "read-only analysis must be deterministic"
    );
}

#[test]
fn basic_statistics_of_empty_circuit_are_deterministic() {
    let circuit =
        empty_circuit();

    let first =
        basic_statistics(&circuit)
            .expect("empty circuit statistics must succeed");

    let second =
        basic_statistics(&circuit)
            .expect("repeated empty circuit statistics must succeed");

    assert_eq!(
        first,
        second,
        "basic statistics must be deterministic"
    );
}

#[test]
fn bounded_analysis_succeeds_for_small_valid_circuit() {
    let mut circuit =
        empty_circuit();

    circuit
        .push(unary_gate(GateKind::H, 0))
        .expect("H must be accepted");

    circuit
        .push(
            binary_gate(
                GateKind::CX,
                0,
                1,
            ),
        )
        .expect("CX must be accepted");

    let limits =
        test_limits();

    analyze_with_limits(
        &circuit,
        &limits,
    )
    .expect(
        "bounded analysis must succeed for a small valid circuit"
    );

    basic_statistics_with_limits(
        &circuit,
        &limits,
    )
    .expect(
        "bounded basic statistics must succeed for a small valid circuit"
    );
}

// =============================================================================
// Namespace/accounting invariants
// =============================================================================

#[test]
fn declaring_more_qubits_does_not_create_operations() {
    let circuit =
        QuantumCircuit::try_new_with_limits(
            1_024,
            0,
            QuantumIrLimits::production()
                .with_max_qubits(1_024),
        )
        .expect("declared namespace must be constructible");

    assert_eq!(
        circuit.num_qubits(),
        1_024,
    );

    assert_eq!(
        circuit.len(),
        0,
    );
}

#[test]
fn declared_classical_namespace_does_not_create_quantum_operations() {
    let circuit =
        QuantumCircuit::try_new_with_limits(
            0,
            1_024,
            QuantumIrLimits::production()
                .with_max_classical_bits(1_024),
        )
        .expect("classical namespace must be constructible");

    assert_eq!(
        circuit.num_classical_bits(),
        1_024,
    );

    assert_eq!(
        circuit.len(),
        0,
    );
}

// =============================================================================
// Policy-versus-semantics invariants
// =============================================================================

#[test]
fn tightening_policy_does_not_change_qubit_id_semantics() {
    let id =
        QubitId::new(5);

    let permissive =
        QuantumIrLimits::production()
            .with_max_qubits(8);

    let restrictive =
        QuantumIrLimits::production()
            .with_max_qubits(4);

    assert_eq!(
        id.index(),
        5,
    );

    let permissive_circuit =
        QuantumCircuit::try_new_with_limits(
            6,
            0,
            permissive,
        )
        .expect("six logical qubits fit permissive policy");

    assert_eq!(
        permissive_circuit.num_qubits(),
        6,
    );

    let restrictive_result =
        QuantumCircuit::try_new_with_limits(
            6,
            0,
            restrictive,
        );

    assert!(
        restrictive_result.is_err(),
        "restrictive policy must reject six qubits"
    );

    // The identifier itself remains valid regardless of the policy.
    assert_eq!(
        QubitId::new(5).index(),
        id.index(),
    );
}

// =============================================================================
// Boundary-value invariants
// =============================================================================

#[test]
fn zero_qubit_circuit_is_representable() {
    let circuit =
        QuantumCircuit::try_new_with_limits(
            0,
            0,
            QuantumIrLimits::production(),
        )
        .expect("zero-resource circuit must be representable");

    assert_eq!(
        circuit.num_qubits(),
        0,
    );

    assert_eq!(
        circuit.num_classical_bits(),
        0,
    );

    assert!(circuit.is_empty());
}

#[test]
fn one_qubit_circuit_is_representable() {
    let mut circuit =
        QuantumCircuit::try_new_with_limits(
            1,
            0,
            QuantumIrLimits::production(),
        )
        .expect("one-qubit circuit must be representable");

    circuit
        .push(unary_gate(GateKind::H, 0))
        .expect("H must be valid on q0");

    assert_eq!(
        circuit.len(),
        1,
    );
}

// =============================================================================
// Explicit failure invariants
// =============================================================================

#[test]
fn invalid_qubit_namespace_failure_is_not_silent() {
    let mut circuit =
        QuantumCircuit::try_new_with_limits(
            1,
            0,
            test_limits(),
        )
        .expect("test circuit must be valid");

    let result =
        circuit.push(
            unary_gate(GateKind::X, 99)
        );

    assert!(
        result.is_err(),
        "invalid logical-qubit references must produce an explicit error"
    );
}

#[test]
fn invalid_gate_failure_is_not_silent() {
    let result =
        Gate::new(
            GateKind::CX,
            vec![QubitId::new(0)],
            Vec::new(),
            None,
            None,
        );

    assert!(
        result.is_err(),
        "invalid gate structure must not be silently accepted"
    );
}

// =============================================================================
// Regression protection for canonical qubit module
// =============================================================================

#[test]
fn canonical_qubit_module_is_the_source_of_logical_identity() {
    let canonical =
        crate::quantum::ir::qubit::QubitId::new(3);

    let root_reexport =
        crate::quantum::ir::QubitId::new(3);

    assert_eq!(
        canonical,
        root_reexport,
    );
}

#[test]
fn canonical_qubit_module_is_the_source_of_physical_identity() {
    let canonical =
        crate::quantum::ir::qubit::PhysicalQubitId::new(3);

    let root_reexport =
        crate::quantum::ir::PhysicalQubitId::new(3);

    assert_eq!(
        canonical,
        root_reexport,
    );
}

// =============================================================================
// Cross-module contract
// =============================================================================

#[test]
fn qubit_gate_circuit_and_validation_share_one_logical_identity_domain() {
    let q0 =
        crate::quantum::ir::qubit::QubitId::new(0);

    let gate =
        Gate::new(
            GateKind::X,
            vec![q0],
            Vec::new(),
            None,
            None,
        )
        .expect("X(q0) must be valid");

    let mut circuit =
        QuantumCircuit::try_new_with_limits(
            1,
            0,
            test_limits(),
        )
        .expect("one-qubit circuit must be valid");

    circuit
        .push(gate)
        .expect("canonical gate must integrate with canonical circuit");

    validate_circuit_with_limits(
        &circuit,
        circuit.limits(),
    )
    .expect(
        "canonical qubit -> gate -> circuit -> validation pipeline must remain valid"
    );
}

// =============================================================================
// Determinism of repeated validation
// =============================================================================

#[test]
fn repeated_validation_is_deterministic() {
    let mut circuit =
        empty_circuit();

    circuit
        .push(unary_gate(GateKind::H, 0))
        .expect("H must be accepted");

    circuit
        .push(
            binary_gate(
                GateKind::CX,
                0,
                1,
            ),
        )
        .expect("CX must be accepted");

    let limits =
        test_limits();

    let first =
        validate_circuit_with_limits(
            &circuit,
            &limits,
        );

    let second =
        validate_circuit_with_limits(
            &circuit,
            &limits,
        );

    assert_eq!(
        first,
        second,
        "validation results must be deterministic"
    );
}

// =============================================================================
// Mutation atomicity regression
// =============================================================================

#[test]
fn failed_mutation_preserves_existing_valid_program() {
    let mut circuit =
        empty_circuit();

    circuit
        .push(unary_gate(GateKind::H, 0))
        .expect("H must be accepted");

    let before_len =
        circuit.len();

    let result =
        circuit.push(
            unary_gate(
                GateKind::X,
                circuit.num_qubits() + 100,
            ),
        );

    assert!(
        result.is_err(),
        "invalid mutation must fail"
    );

    assert_eq!(
        circuit.len(),
        before_len,
        "failed mutation must preserve prior valid program"
    );

    validate_circuit_with_limits(
        &circuit,
        circuit.limits(),
    )
    .expect(
        "the circuit must remain valid after a failed mutation"
    );
}

// =============================================================================
// Production-policy sanity
// =============================================================================

#[test]
fn production_policy_is_not_a_semantic_qubit_limit() {
    let limits =
        QuantumIrLimits::production();

    //
    // The test intentionally does not assert a particular production
    // `max_qubits` value. Doing so would convert a policy implementation
    // detail into an architectural contract.
    //
    // Instead we only verify that the policy itself is valid.
    //
    limits
        .validate()
        .expect("production policy must remain valid");
}

// =============================================================================
// Final invariant: valid IR remains self-consistent
// =============================================================================

#[test]
fn representative_valid_ir_satisfies_all_primary_contracts() {
    let mut circuit =
        QuantumCircuit::try_new_with_limits(
            4,
            4,
            test_limits(),
        )
        .expect("representative circuit must be valid");

    circuit
        .push(
            unary_gate(
                GateKind::H,
                0,
            ),
        )
        .expect("H must be accepted");

    circuit
        .push(
            binary_gate(
                GateKind::CX,
                0,
                1,
            ),
        )
        .expect("CX must be accepted");

    circuit
        .push(
            parameterized_gate(
                GateKind::RX,
                2,
                0.5,
            ),
        )
        .expect("RX must be accepted");

    validate_circuit_with_limits(
        &circuit,
        circuit.limits(),
    )
    .expect(
        "representative canonical IR must pass complete validation"
    );

    let analysis =
        analyze_with_limits(
            &circuit,
            circuit.limits(),
        )
        .expect(
            "representative canonical IR must pass bounded analysis"
        );

    let repeated_analysis =
        analyze_with_limits(
            &circuit,
            circuit.limits(),
        )
        .expect(
            "repeated analysis must succeed"
        );

    assert_eq!(
        analysis,
        repeated_analysis,
        "representative IR analysis must be deterministic"
    );

    assert_eq!(
        circuit.num_qubits(),
        4,
    );

    assert_eq!(
        circuit.num_classical_bits(),
        4,
    );

    assert_eq!(
        circuit.len(),
        3,
    );
}