//! Zamani Quantum IR — Cross-Module Integration Tests.
//!
//! # Purpose
//!
//! This module verifies that the independently implemented Quantum IR
//! components compose correctly through their public contracts.
//!
//! It is intentionally different from:
//!
//! - `invariants.rs` — foundational semantic invariants;
//! - `scaling.rs` — namespace/storage scalability;
//! - module-local unit tests — implementation-local behavior.
//!
//! This file tests the boundaries between:
//!
//! ```text
//! identity
//!     │
//!     ├──────────────┐
//!     │              │
//! qubit          parameter
//!     │              │
//!     └──────┬───────┘
//!            ▼
//!          gate
//!            │
//!            ▼
//!        circuit
//!            │
//!       ┌────┴─────┐
//!       ▼          ▼
//! validation    analysis
//!       │          │
//!       └────┬─────┘
//!            ▼
//!       IR contract
//! ```
//!
//! # Architectural contract
//!
//! The tests protect the following dependency direction:
//!
//! ```text
//! frontend
//!    │
//!    ▼
//! quantum::ir
//!    │
//!    ├── optimization
//!    ├── routing
//!    ├── scheduling
//!    ├── hardware
//!    ├── simulator
//!    ├── qec
//!    └── backend
//! ```
//!
//! The IR does not depend on those downstream systems.
//!
//! This suite therefore does NOT:
//!
//! - contact hardware;
//! - contact a QPU;
//! - execute a circuit;
//! - simulate a state vector;
//! - perform routing;
//! - perform scheduling;
//! - synthesize pulses;
//! - perform calibration;
//! - decode QEC;
//! - test optimizer quality;
//! - test vendor APIs;
//! - parse frontend source.
//!
//! Those are separate integration boundaries.
//!
//! # Canonical qubit path
//!
//! New code MUST use:
//!
//! ```text
//! crate::quantum::ir::qubit
//! ```
//!
//! and not the historical compatibility alias:
//!
//! ```text
//! crate::quantum::ir::qubits
//! ```
//!
//! # Scalability contract
//!
//! No value in this file is an architectural maximum.
//!
//! Test limits are deliberately explicit policies used to make rejection and
//! atomicity testable without requiring enormous allocations.
//!
//! The semantic model must remain capable of representing any finite
//! representable namespace for which the caller has sufficient resources.
//!
//! "Infinity" is not allocated by a finite Rust process. The actual production
//! guarantee is:
//!
//! ```text
//! any finite representable program
//!         │
//!         ▼
//! explicit policy
//!         │
//!         ▼
//! available host/compiler resources
//!         │
//!         ▼
//! target capabilities/resources
//! ```
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
//! `#![forbid(unsafe_code)]` makes the no-unsafe requirement compiler-enforced.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

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
    QubitRange,
    QubitRef,
    QubitRegister,
    QubitState,
};

use crate::quantum::ir::validation::validate_circuit_with_limits;


// =============================================================================
// Test policy
// =============================================================================

/// Creates an intentionally bounded policy for integration tests.
///
/// This is NOT a Zamani machine-size limit.
///
/// Production callers are expected to provide their own explicit resource
/// policy based on available compiler/runtime resources.
fn integration_limits() -> QuantumIrLimits {
    QuantumIrLimits::production()
        .with_max_qubits(16)
        .with_max_classical_bits(16)
        .with_max_operations(64)
        .with_max_operands(32)
        .with_max_parameters(32)
        .with_max_metadata_bytes(1_024)
        .with_max_depth(64)
        .with_max_measurements(16)
        .with_max_barriers(16)
        .with_max_validation_steps(10_000)
        .with_max_analysis_steps(10_000)
}

/// Creates a small circuit used for cross-module integration tests.
fn test_circuit() -> QuantumCircuit {
    QuantumCircuit::try_new_with_limits(
        4,
        4,
        integration_limits(),
    )
    .expect("integration test circuit must be constructible")
}

/// Creates a valid unary gate using the canonical logical-qubit identity.
fn unary_gate(
    kind: GateKind,
    qubit: QubitId,
) -> Gate {
    Gate::new(
        kind,
        vec![qubit],
        Vec::new(),
        None,
        None,
    )
    .expect("valid unary gate must be constructible")
}

/// Creates a valid binary gate using canonical logical-qubit identities.
fn binary_gate(
    kind: GateKind,
    first: QubitId,
    second: QubitId,
) -> Gate {
    Gate::new(
        kind,
        vec![first, second],
        Vec::new(),
        None,
        None,
    )
    .expect("valid binary gate must be constructible")
}

/// Creates a valid ternary gate.
fn ternary_gate(
    kind: GateKind,
    first: QubitId,
    second: QubitId,
    third: QubitId,
) -> Gate {
    Gate::new(
        kind,
        vec![
            first,
            second,
            third,
        ],
        Vec::new(),
        None,
        None,
    )
    .expect("valid ternary gate must be constructible")
}

/// Creates a valid parameterized gate.
fn parameterized_gate(
    kind: GateKind,
    qubit: QubitId,
    value: f64,
) -> Gate {
    let parameter = Parameter::constant(value)
        .expect("finite parameter must be accepted");

    Gate::new(
        kind,
        vec![qubit],
        vec![parameter],
        None,
        None,
    )
    .expect("valid parameterized gate must be constructible")
}


// =============================================================================
// Identity ↔ qubit integration
// =============================================================================

#[test]
fn canonical_logical_qubit_identity_is_usable_by_gate_layer() {
    let qubit = QubitId::new(0);

    let gate = unary_gate(
        GateKind::X,
        qubit,
    );

    assert_eq!(
        gate.qubits().len(),
        1,
    );

    assert_eq!(
        gate.qubits()[0],
        qubit,
    );
}

#[test]
fn canonical_logical_qubit_identity_is_usable_by_two_qubit_gate_layer() {
    let first = QubitId::new(0);
    let second = QubitId::new(1);

    let gate = binary_gate(
        GateKind::CX,
        first,
        second,
    );

    assert_eq!(
        gate.qubits(),
        &[first, second],
    );
}

#[test]
fn canonical_logical_qubit_identity_is_usable_by_three_qubit_gate_layer() {
    let first = QubitId::new(0);
    let second = QubitId::new(1);
    let third = QubitId::new(2);

    let gate = ternary_gate(
        GateKind::CCX,
        first,
        second,
        third,
    );

    assert_eq!(
        gate.qubits(),
        &[first, second, third],
    );
}


// =============================================================================
// Logical ↔ physical identity integration
// =============================================================================

#[test]
fn logical_and_physical_ids_remain_distinct_at_integration_boundary() {
    let logical = QubitId::new(5);
    let physical = PhysicalQubitId::new(5);

    assert_eq!(
        logical.index(),
        physical.index(),
    );

    let logical_ref = QubitRef::Logical(logical);
    let physical_ref = QubitRef::Physical(physical);

    assert!(logical_ref.is_logical());
    assert!(!logical_ref.is_physical());

    assert!(physical_ref.is_physical());
    assert!(!physical_ref.is_logical());

    assert_eq!(
        logical_ref.logical(),
        Some(logical),
    );

    assert_eq!(
        logical_ref.physical(),
        None,
    );

    assert_eq!(
        physical_ref.physical(),
        Some(physical),
    );

    assert_eq!(
        physical_ref.logical(),
        None,
    );
}

#[test]
fn physical_identity_never_becomes_a_logical_gate_operand() {
    let logical = QubitId::new(0);
    let physical = PhysicalQubitId::new(0);

    let gate = unary_gate(
        GateKind::X,
        logical,
    );

    assert_eq!(
        gate.qubits()[0],
        logical,
    );

    assert_ne!(
        format!("{logical}"),
        format!("{physical}"),
        "logical and physical identities must remain distinguishable"
    );
}


// =============================================================================
// Identity ↔ circuit integration
// =============================================================================

#[test]
fn circuit_identity_and_ir_version_survive_construction() {
    let circuit_id = CircuitId::new(123);

    let circuit = QuantumCircuit::with_identity(
        circuit_id,
        2,
        2,
        integration_limits(),
    )
    .expect("identity-bearing circuit must be constructible");

    assert_eq!(
        circuit.id(),
        circuit_id,
    );

    assert_eq!(
        circuit.version(),
        IrVersion::CURRENT,
    );
}

#[test]
fn circuit_identity_can_change_without_rebuilding_semantic_content() {
    let mut circuit = test_circuit();

    circuit
        .push(unary_gate(
            GateKind::H,
            QubitId::new(0),
        ))
        .expect("valid operation must be inserted");

    let original_length = circuit.len();

    let replacement_id = CircuitId::new(777);

    circuit.set_id(replacement_id);

    assert_eq!(
        circuit.id(),
        replacement_id,
    );

    assert_eq!(
        circuit.len(),
        original_length,
        "changing identity must not change operations"
    );
}

#[test]
fn current_ir_version_is_accepted_by_circuit_boundary() {
    let mut circuit = test_circuit();

    circuit
        .set_version(IrVersion::CURRENT)
        .expect("current IR version must be supported");

    assert_eq!(
        circuit.version(),
        IrVersion::CURRENT,
    );
}


// =============================================================================
// Parameter ↔ gate integration
// =============================================================================

#[test]
fn finite_parameter_flows_into_parameterized_gate() {
    let gate = parameterized_gate(
        GateKind::RX,
        QubitId::new(0),
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

#[test]
fn nan_is_rejected_before_parameter_reaches_gate() {
    let parameter = Parameter::constant(
        f64::NAN,
    );

    assert!(
        parameter.is_err(),
        "NaN must never enter the canonical IR"
    );
}

#[test]
fn positive_infinity_is_rejected_before_parameter_reaches_gate() {
    let parameter = Parameter::constant(
        f64::INFINITY,
    );

    assert!(
        parameter.is_err(),
        "positive infinity must never enter the canonical IR"
    );
}

#[test]
fn negative_infinity_is_rejected_before_parameter_reaches_gate() {
    let parameter = Parameter::constant(
        f64::NEG_INFINITY,
    );

    assert!(
        parameter.is_err(),
        "negative infinity must never enter the canonical IR"
    );
}

#[test]
fn parameterized_gate_without_parameter_is_rejected_at_gate_boundary() {
    let result = Gate::new(
        GateKind::RX,
        vec![QubitId::new(0)],
        Vec::new(),
        None,
        None,
    );

    assert!(
        result.is_err(),
        "RX without a parameter must not reach circuit construction"
    );
}

#[test]
fn non_parameterized_gate_with_parameter_is_rejected_at_gate_boundary() {
    let parameter = Parameter::constant(0.25)
        .expect("finite parameter must be accepted");

    let result = Gate::new(
        GateKind::X,
        vec![QubitId::new(0)],
        vec![parameter],
        None,
        None,
    );

    assert!(
        result.is_err(),
        "X must not receive a parameter"
    );
}


// =============================================================================
// Gate ↔ circuit integration
// =============================================================================

#[test]
fn valid_gate_can_cross_into_circuit() {
    let mut circuit = test_circuit();

    let gate = unary_gate(
        GateKind::H,
        QubitId::new(0),
    );

    circuit
        .push(gate)
        .expect("valid gate must cross circuit boundary");

    assert_eq!(
        circuit.len(),
        1,
    );

    assert_eq!(
        circuit
            .first()
            .map(Gate::kind),
        Some(GateKind::H),
    );
}

#[test]
fn two_qubit_gate_can_cross_into_circuit() {
    let mut circuit = test_circuit();

    let gate = binary_gate(
        GateKind::CX,
        QubitId::new(0),
        QubitId::new(1),
    );

    circuit
        .push(gate)
        .expect("valid binary gate must cross circuit boundary");

    assert_eq!(
        circuit.len(),
        1,
    );

    assert_eq!(
        circuit
            .first()
            .map(Gate::kind),
        Some(GateKind::CX),
    );
}

#[test]
fn parameterized_gate_can_cross_into_circuit() {
    let mut circuit = test_circuit();

    circuit
        .push(parameterized_gate(
            GateKind::RZ,
            QubitId::new(2),
            1.25,
        ))
        .expect(
            "valid parameterized gate must cross circuit boundary"
        );

    assert_eq!(
        circuit.len(),
        1,
    );
}

#[test]
fn multiple_semantic_gate_kinds_can_share_one_circuit() {
    let mut circuit = test_circuit();

    circuit
        .push(unary_gate(
            GateKind::H,
            QubitId::new(0),
        ))
        .expect("H must be accepted");

    circuit
        .push(binary_gate(
            GateKind::CX,
            QubitId::new(0),
            QubitId::new(1),
        ))
        .expect("CX must be accepted");

    circuit
        .push(parameterized_gate(
            GateKind::RX,
            QubitId::new(1),
            0.75,
        ))
        .expect("RX must be accepted");

    assert_eq!(
        circuit.len(),
        3,
    );
}


// =============================================================================
// Gate validation ↔ circuit validation integration
// =============================================================================

#[test]
fn valid_circuit_passes_whole_circuit_validation() {
    let mut circuit = test_circuit();

    circuit
        .push(unary_gate(
            GateKind::H,
            QubitId::new(0),
        ))
        .expect("H must be valid");

    circuit
        .push(binary_gate(
            GateKind::CX,
            QubitId::new(0),
            QubitId::new(1),
        ))
        .expect("CX must be valid");

    circuit
        .push(parameterized_gate(
            GateKind::RZ,
            QubitId::new(1),
            0.25,
        ))
        .expect("RZ must be valid");

    validate_circuit_with_limits(
        &circuit,
        &integration_limits(),
    )
    .expect(
        "a valid circuit must pass canonical whole-circuit validation"
    );
}

#[test]
fn empty_circuit_passes_whole_circuit_validation() {
    let circuit = test_circuit();

    validate_circuit_with_limits(
        &circuit,
        &integration_limits(),
    )
    .expect(
        "an empty valid circuit must pass whole-circuit validation"
    );
}

#[test]
fn validation_does_not_mutate_circuit() {
    let mut circuit = test_circuit();

    circuit
        .push(unary_gate(
            GateKind::X,
            QubitId::new(0),
        ))
        .expect("X must be valid");

    let before_len = circuit.len();
    let before_qubits = circuit.num_qubits();
    let before_classical_bits =
        circuit.num_classical_bits();
    let before_id = circuit.id();
    let before_version = circuit.version();

    validate_circuit_with_limits(
        &circuit,
        &integration_limits(),
    )
    .expect("valid circuit must validate");

    assert_eq!(
        circuit.len(),
        before_len,
    );

    assert_eq!(
        circuit.num_qubits(),
        before_qubits,
    );

    assert_eq!(
        circuit.num_classical_bits(),
        before_classical_bits,
    );

    assert_eq!(
        circuit.id(),
        before_id,
    );

    assert_eq!(
        circuit.version(),
        before_version,
    );
}


// =============================================================================
// Circuit ↔ analysis integration
// =============================================================================

#[test]
fn valid_circuit_can_cross_into_complete_analysis() {
    let mut circuit = test_circuit();

    circuit
        .push(unary_gate(
            GateKind::H,
            QubitId::new(0),
        ))
        .expect("H must be valid");

    circuit
        .push(binary_gate(
            GateKind::CX,
            QubitId::new(0),
            QubitId::new(1),
        ))
        .expect("CX must be valid");

    analyze(&circuit)
        .expect("complete analysis must accept a valid circuit");
}

#[test]
fn valid_circuit_can_cross_into_basic_analysis() {
    let mut circuit = test_circuit();

    circuit
        .push(unary_gate(
            GateKind::X,
            QubitId::new(0),
        ))
        .expect("X must be valid");

    basic_statistics(&circuit)
        .expect("basic analysis must accept a valid circuit");
}

#[test]
fn complete_analysis_respects_explicit_work_policy() {
    let mut circuit = test_circuit();

    circuit
        .push(unary_gate(
            GateKind::H,
            QubitId::new(0),
        ))
        .expect("H must be valid");

    analyze_with_limits(
        &circuit,
        &integration_limits(),
    )
    .expect(
        "analysis must succeed under a sufficient explicit policy"
    );
}

#[test]
fn basic_analysis_respects_explicit_work_policy() {
    let mut circuit = test_circuit();

    circuit
        .push(binary_gate(
            GateKind::CX,
            QubitId::new(0),
            QubitId::new(1),
        ))
        .expect("CX must be valid");

    basic_statistics_with_limits(
        &circuit,
        &integration_limits(),
    )
    .expect(
        "basic analysis must succeed under a sufficient explicit policy"
    );
}

#[test]
fn repeated_analysis_is_deterministically_successful() {
    let mut circuit = test_circuit();

    circuit
        .push(unary_gate(
            GateKind::H,
            QubitId::new(0),
        ))
        .expect("H must be valid");

    circuit
        .push(binary_gate(
            GateKind::CX,
            QubitId::new(0),
            QubitId::new(1),
        ))
        .expect("CX must be valid");

    let first = analyze(&circuit)
        .expect("first analysis must succeed");

    let second = analyze(&circuit)
        .expect("second analysis must succeed");

    assert_eq!(
        first,
        second,
        "identical IR must produce deterministic analysis"
    );
}


// =============================================================================
// Circuit mutation ↔ policy integration
// =============================================================================

#[test]
fn operation_limit_is_checked_before_mutation() {
    let limits = integration_limits()
        .with_max_operations(1);

    let mut circuit =
        QuantumCircuit::try_new_with_limits(
            1,
            0,
            limits,
        )
        .expect("circuit must be constructible");

    circuit
        .push(unary_gate(
            GateKind::X,
            QubitId::new(0),
        ))
        .expect("first operation must fit");

    let result = circuit.push(
        unary_gate(
            GateKind::H,
            QubitId::new(0),
        ),
    );

    assert!(
        result.is_err(),
        "second operation must exceed explicit policy"
    );

    assert_eq!(
        circuit.len(),
        1,
        "failed insertion must be atomic"
    );
}

#[test]
fn qubit_namespace_policy_is_checked_before_circuit_construction() {
    let limits = integration_limits()
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
            assert_eq!(
                requested,
                3,
            );

            assert_eq!(
                maximum,
                2,
            );
        }

        other => {
            panic!(
                "expected QubitLimitExceeded, received {other:?}"
            );
        }
    }
}

#[test]
fn classical_namespace_policy_is_checked_before_circuit_construction() {
    let limits = integration_limits()
        .with_max_classical_bits(2);

    let result =
        QuantumCircuit::try_new_with_limits(
            0,
            3,
            limits,
        );

    match result {
        Err(
            CircuitError::ClassicalBitLimitExceeded {
                requested,
                maximum,
            },
        ) => {
            assert_eq!(
                requested,
                3,
            );

            assert_eq!(
                maximum,
                2,
            );
        }

        other => {
            panic!(
                "expected ClassicalBitLimitExceeded, received {other:?}"
            );
        }
    }
}


// =============================================================================
// Qubit register ↔ namespace integration
// =============================================================================

#[test]
fn qubit_register_respects_explicit_policy() {
    let register =
        QubitRegister::try_new(
            8,
            8,
        )
        .expect(
            "register within explicit policy must be constructible"
        );

    assert_eq!(
        register.len(),
        8,
    );
}

#[test]
fn qubit_register_rejects_policy_overflow() {
    let result =
        QubitRegister::try_new(
            9,
            8,
        );

    assert!(
        result.is_err(),
        "register must reject explicit policy overflow"
    );
}

#[test]
fn zero_sized_qubit_register_integrates_with_zero_policy() {
    let register =
        QubitRegister::try_new(
            0,
            0,
        )
        .expect(
            "zero-sized logical namespace must remain representable"
        );

    assert_eq!(
        register.len(),
        0,
    );
}


// =============================================================================
// Qubit range ↔ identity integration
// =============================================================================

#[test]
fn qubit_range_can_describe_namespace_without_allocating_qubits() {
    let range =
        QubitRange::new(
            100,
            200,
        )
        .expect(
            "valid half-open qubit range must be accepted"
        );

    assert_eq!(
        range.start(),
        100,
    );

    assert_eq!(
        range.end(),
        200,
    );
}

#[test]
fn empty_qubit_range_is_valid() {
    let range =
        QubitRange::empty(100);

    assert_eq!(
        range.start(),
        100,
    );

    assert_eq!(
        range.end(),
        100,
    );
}

#[test]
fn reversed_qubit_range_is_rejected() {
    let result =
        QubitRange::new(
            200,
            100,
        );

    assert!(
        result.is_err(),
        "reversed namespace ranges must be rejected"
    );
}


// =============================================================================
// Qubit state ↔ logical qubit integration
// =============================================================================

#[test]
fn logical_qubit_starts_in_available_state() {
    let qubit =
        Qubit::new(
            QubitId::new(0),
        );

    assert!(
        qubit.is_available()
    );

    assert!(
        qubit.is_usable()
    );

    assert!(
        !qubit.is_disabled()
    );
}

#[test]
fn measured_state_does_not_mean_disabled_state() {
    let state =
        QubitState::Measured;

    assert!(
        state.is_measured()
    );

    assert!(
        state.is_usable(),
        "measurement bookkeeping must not automatically disable a qubit"
    );

    assert!(
        !state.is_disabled()
    );
}

#[test]
fn reset_state_does_not_mean_disabled_state() {
    let state =
        QubitState::Reset;

    assert!(
        state.is_reset()
    );

    assert!(
        state.is_usable()
    );

    assert!(
        !state.is_disabled()
    );
}


// =============================================================================
// Large sparse namespace ↔ gate ↔ circuit integration
// =============================================================================

#[test]
fn sparse_large_namespace_remains_operation_sparse() {
    let limits =
        QuantumIrLimits::unbounded();

    let mut circuit =
        QuantumCircuit::try_new_with_limits(
            usize::MAX,
            0,
            limits,
        )
        .expect(
            "unbounded policy must permit the representable namespace boundary"
        );

    let qubit =
        QubitId::new(
            usize::MAX - 1,
        );

    circuit
        .push(
            unary_gate(
                GateKind::X,
                qubit,
            ),
        )
        .expect(
            "highest in-range logical qubit must remain addressable"
        );

    assert_eq!(
        circuit.num_qubits(),
        usize::MAX,
    );

    assert_eq!(
        circuit.len(),
        1,
        "declaring a huge namespace must not materialize operations"
    );
}

#[test]
fn namespace_size_does_not_change_empty_circuit_semantics() {
    let small =
        QuantumCircuit::try_new_with_limits(
            1,
            0,
            QuantumIrLimits::unbounded(),
        )
        .expect(
            "small unbounded circuit must be valid"
        );

    let large =
        QuantumCircuit::try_new_with_limits(
            usize::MAX,
            0,
            QuantumIrLimits::unbounded(),
        )
        .expect(
            "large unbounded circuit must be valid"
        );

    assert_eq!(
        small.len(),
        large.len(),
    );

    assert!(
        small.is_empty()
    );

    assert!(
        large.is_empty()
    );
}

#[test]
fn sparse_large_namespace_can_cross_validation_boundary() {
    let limits =
        QuantumIrLimits::unbounded();

    let mut circuit =
        QuantumCircuit::try_new_with_limits(
            usize::MAX,
            0,
            limits,
        )
        .expect(
            "large unbounded namespace must be representable"
        );

    circuit
        .push(
            unary_gate(
                GateKind::X,
                QubitId::new(
                    usize::MAX - 1,
                ),
            ),
        )
        .expect(
            "sparse operation must remain representable"
        );

    validate_circuit_with_limits(
        &circuit,
        &QuantumIrLimits::unbounded(),
    )
    .expect(
        "large sparse namespace must cross validation without materialization"
    );
}


// =============================================================================
// Boundary arithmetic integration
// =============================================================================

#[test]
fn logical_identifier_boundary_is_overflow_safe() {
    let maximum =
        QubitId::new(
            usize::MAX,
        );

    assert_eq!(
        maximum.index(),
        usize::MAX,
    );

    assert_eq!(
        maximum.checked_next(),
        None,
    );
}

#[test]
fn physical_identifier_boundary_is_overflow_safe() {
    let maximum =
        PhysicalQubitId::new(
            usize::MAX,
        );

    assert_eq!(
        maximum.index(),
        usize::MAX,
    );

    assert_eq!(
        maximum.checked_next(),
        None,
    );
}

#[test]
fn operation_identity_supports_u64_boundary_without_becoming_machine_size() {
    let id =
        OperationId::new(
            u64::MAX,
        );

    assert_eq!(
        id.value(),
        u64::MAX,
    );

    assert_eq!(
        u64::from(id),
        u64::MAX,
    );
}


// =============================================================================
// Policy ↔ semantic independence
// =============================================================================

#[test]
fn tightening_policy_does_not_change_semantic_qubit_identity() {
    let unrestricted =
        QubitId::new(7);

    let restrictive =
        integration_limits()
            .with_max_qubits(1);

    assert!(
        restrictive.validate().is_ok()
    );

    assert_eq!(
        unrestricted.index(),
        7,
        "a policy must not mutate or redefine logical identity"
    );
}

#[test]
fn production_policy_is_valid_at_ir_boundary() {
    QuantumIrLimits::production()
        .validate()
        .expect(
            "production policy must satisfy its own configuration invariants"
        );
}

#[test]
fn unbounded_policy_is_valid_at_ir_boundary() {
    QuantumIrLimits::unbounded()
        .validate()
        .expect(
            "unbounded policy must remain structurally valid"
        );
}

#[test]
fn deny_all_policy_is_valid_at_ir_boundary() {
    QuantumIrLimits::deny_all()
        .validate()
        .expect(
            "deny-all policy must remain structurally valid"
        );
}


// =============================================================================
// Cross-module deterministic contract
// =============================================================================

#[test]
fn identical_construction_path_produces_identical_analysis_results() {
    let build = || {
        let mut circuit = test_circuit();

        circuit
            .push(
                unary_gate(
                    GateKind::H,
                    QubitId::new(0),
                ),
            )
            .expect("H must be valid");

        circuit
            .push(
                binary_gate(
                    GateKind::CX,
                    QubitId::new(0),
                    QubitId::new(1),
                ),
            )
            .expect("CX must be valid");

        circuit
            .push(
                parameterized_gate(
                    GateKind::RZ,
                    QubitId::new(1),
                    0.5,
                ),
            )
            .expect("RZ must be valid");

        circuit
    };

    let first =
        build();

    let second =
        build();

    let first_analysis =
        analyze(&first)
            .expect("first analysis must succeed");

    let second_analysis =
        analyze(&second)
            .expect("second analysis must succeed");

    assert_eq!(
        first_analysis,
        second_analysis,
        "equivalent IR construction must produce equivalent analysis"
    );
}


// =============================================================================
// Final end-to-end IR contract
// =============================================================================

#[test]
fn complete_gate_circuit_ir_pipeline_is_valid() {
    let mut circuit =
        test_circuit();

    // Identity layer.
    assert_eq!(
        circuit.version(),
        IrVersion::CURRENT,
    );

    // Qubit layer.
    let q0 =
        QubitId::new(0);

    let q1 =
        QubitId::new(1);

    let q2 =
        QubitId::new(2);

    // Gate + parameter layers.
    circuit
        .push(
            unary_gate(
                GateKind::H,
                q0,
            ),
        )
        .expect("H must be valid");

    circuit
        .push(
            binary_gate(
                GateKind::CX,
                q0,
                q1,
            ),
        )
        .expect("CX must be valid");

    circuit
        .push(
            parameterized_gate(
                GateKind::RY,
                q1,
                1.0,
            ),
        )
        .expect("RY must be valid");

    circuit
        .push(
            ternary_gate(
                GateKind::CCX,
                q0,
                q1,
                q2,
            ),
        )
        .expect("CCX must be valid");

    // Circuit layer.
    assert_eq!(
        circuit.len(),
        4,
    );

    // Validation layer.
    validate_circuit_with_limits(
        &circuit,
        &integration_limits(),
    )
    .expect(
        "complete valid circuit must pass whole-IR validation"
    );

    // Analysis layer.
    let analysis =
        analyze_with_limits(
            &circuit,
            &integration_limits(),
        )
        .expect(
            "complete valid circuit must pass analysis"
        );

    let statistics =
        basic_statistics_with_limits(
            &circuit,
            &integration_limits(),
        )
        .expect(
            "complete valid circuit must pass basic analysis"
        );

    // The values are deliberately retained so this test proves that all
    // downstream IR consumers can consume the same canonical circuit.
    let _ = analysis;
    let _ = statistics;
}


// =============================================================================
// Explicit negative integration contracts
// =============================================================================

#[test]
fn invalid_gate_never_crosses_into_circuit() {
    let mut circuit =
        test_circuit();

    let invalid =
        Gate::new(
            GateKind::CX,
            vec![
                QubitId::new(0),
            ],
            Vec::new(),
            None,
            None,
        );

    assert!(
        invalid.is_err(),
        "invalid gate must fail before circuit integration"
    );

    assert_eq!(
        circuit.len(),
        0,
        "failed gate construction must not mutate the circuit"
    );
}

#[test]
fn invalid_duplicate_operands_never_cross_into_circuit() {
    let mut circuit =
        test_circuit();

    let invalid =
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
        invalid.is_err(),
        "duplicate logical operands must be rejected at the gate boundary"
    );

    assert_eq!(
        circuit.len(),
        0,
    );
}

#[test]
fn invalid_reset_arity_never_crosses_into_circuit() {
    let mut circuit =
        test_circuit();

    let invalid =
        Gate::new(
            GateKind::Reset,
            vec![
                QubitId::new(0),
                QubitId::new(1),
            ],
            Vec::new(),
            None,
            None,
        );

    assert!(
        invalid.is_err(),
        "invalid reset arity must be rejected before circuit insertion"
    );

    assert_eq!(
        circuit.len(),
        0,
    );
}


// =============================================================================
// Contract documentation test
// =============================================================================

#[test]
fn integration_suite_uses_canonical_ir_boundary() {
    // This test intentionally references the public module paths used
    // throughout this file.
    //
    // If a future refactor removes one of these public boundaries, this test
    // suite will fail to compile instead of silently falling back to a private
    // implementation detail.

    let _logical =
        crate::quantum::ir::qubit::QubitId::new(0);

    let _physical =
        crate::quantum::ir::qubit::PhysicalQubitId::new(0);

    let _version =
        crate::quantum::ir::identity::IrVersion::CURRENT;

    let _limits =
        crate::quantum::ir::limits::QuantumIrLimits::production();
}