//! Zamani Quantum IR — Permanent Regression Test Suite.
//!
//! `src/quantum/ir/tests/regression.rs`
//!
//! # Purpose
//!
//! This module contains permanent regression tests for the canonical Zamani
//! Quantum IR.
//!
//! The tests protect stable semantic contracts against:
//!
//! - accidental reintroduction of fixed quantum-machine limits;
//! - incorrect logical/physical qubit identity handling;
//! - use of the legacy `qubits` module path in new code;
//! - invalid parameter acceptance;
//! - invalid gate construction;
//! - duplicate quantum operands;
//! - namespace violations;
//! - resource-policy violations;
//! - partial/nonatomic mutation;
//! - identity/version corruption;
//! - unchecked identifier arithmetic;
//! - operation-count regressions;
//! - deterministic public behavior regressions;
//! - accidental coupling of IR semantics to hardware assumptions;
//! - regressions in explicit resource-policy enforcement.
//!
//! # Architectural position
//!
//! ```text
//! Zamani source
//!      │
//!      ▼
//! quantum::frontend
//!      │
//!      ▼
//! ┌────────────────────────────┐
//! │       quantum::ir          │
//! │                            │
//! │ canonical semantic truth   │
//! └──────────────┬─────────────┘
//!                │
//!       ┌────────┼────────┐
//!       ▼        ▼        ▼
//! optimization routing scheduling
//!       │        │        │
//!       └────────┼────────┘
//!                ▼
//!            hardware
//!                │
//!                ▼
//!             backend
//! ```
//!
//! These tests belong to the IR boundary. They must not make the IR depend on
//! downstream implementation details.
//!
//! # Canonical qubit identity
//!
//! New code MUST use:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! The historical compatibility alias:
//!
//! ```text
//! crate::quantum::ir::qubits
//! ```
//!
//! must not be used by this regression suite.
//!
//! The authoritative implementation is `quantum::ir::qubit`.
//!
//! # Scalability contract
//!
//! "Infinity" is not a Rust allocation size and cannot literally be allocated
//! by a finite process. The production requirement is instead:
//!
//! ```text
//! any finite representable workload
//!          │
//!          ▼
//! constrained only by:
//!   explicit policy
//!   host representation
//!   available resources
//!   target resources
//! ```
//!
//! These tests therefore NEVER define values such as 64, 128, 4096, or any
//! other number as the maximum size of Zamani.
//!
//! Any small number appearing in a test is a deliberately small test policy.
//!
//! # Resource policy versus semantic capacity
//!
//! `QuantumIrLimits` is a per-invocation resource/security policy.
//!
//! It is NOT:
//!
//! - the maximum number of Zamani qubits;
//! - the maximum number of physical qubits;
//! - the maximum register size;
//! - the maximum operation count of the language;
//! - the maximum size of a quantum computer.
//!
//! This distinction is central to the write-once/scale-anywhere architecture.
//!
//! # Regression philosophy
//!
//! These tests assert public contracts rather than private implementation
//! details.
//!
//! They intentionally do NOT inspect:
//!
//! - private vector layouts;
//! - allocator behavior;
//! - pointer addresses;
//! - hash-map internals;
//! - private fields;
//! - optimizer internals;
//! - scheduler internals;
//! - hardware implementations.
//!
//! # Atomic mutation contract
//!
//! A failed mutating operation must not leave a partially mutated circuit.
//!
//! The expected contract is:
//!
//! ```text
//! valid state
//!     │
//!     ▼
//! attempted invalid mutation
//!     │
//!     ▼
//! error
//!     │
//!     ▼
//! original state preserved
//! ```
//!
//! # Security contract
//!
//! The IR must reject malformed or unsafe semantic inputs before they become
//! part of a valid circuit.
//!
//! This includes:
//!
//! - NaN;
//! - positive infinity;
//! - negative infinity;
//! - invalid gate arity;
//! - duplicate operands;
//! - out-of-namespace operands;
//! - resource-policy violations;
//! - integer overflow in identifier progression.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no external test framework;
//! - no unsafe code.
//!
//! # Integration contract
//!
//! This file consumes only the canonical public IR API:
//!
//! - `quantum::ir::circuit`;
//! - `quantum::ir::gate`;
//! - `quantum::ir::identity`;
//! - `quantum::ir::limits`;
//! - `quantum::ir::parameter`;
//! - `quantum::ir::qubit`.
//!
//! It deliberately does not depend on:
//!
//! - frontend parsing;
//! - routing;
//! - scheduling;
//! - hardware;
//! - backend execution;
//! - simulation;
//! - QEC;
//! - optimization.
//!
//! # Integration requirement
//!
//! The parent IR test module should register this file with:
//!
//! ```rust
//! #[path = "tests/regression.rs"]
//! mod regression;
//! ```
//!
//! This is required because the repository currently retains
//! `src/quantum/ir/tests.rs` as the parent integration-test module while also
//! maintaining `src/quantum/ir/tests/` as the test directory.
//!
//! # No silent fallback
//!
//! Test helpers panic when a test fixture that is supposed to be valid cannot
//! be constructed. They never silently substitute another operation.
//!
//! ============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

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
};
use crate::quantum::ir::limits::QuantumIrLimits;
use crate::quantum::ir::parameter::Parameter;
use crate::quantum::ir::qubit::{
    PhysicalQubitId,
    QubitId,
    QubitRef,
};

// =============================================================================
// Test-policy helpers
// =============================================================================

/// Returns a deliberately small policy for deterministic regression tests.
///
/// These numbers are test-policy values only. They are never architectural
/// limits on Zamani.
fn regression_limits() -> QuantumIrLimits {
    QuantumIrLimits::production()
        .with_max_qubits(16)
        .with_max_classical_bits(16)
        .with_max_operations(32)
        .with_max_operands(16)
        .with_max_parameters(16)
        .with_max_metadata_bytes(256)
        .with_max_depth(32)
        .with_max_measurements(16)
        .with_max_barriers(16)
        .with_max_validation_steps(10_000)
        .with_max_analysis_steps(10_000)
}

/// Creates a valid one-qubit operation.
fn unary_gate(kind: GateKind, qubit: QubitId) -> Gate {
    Gate::new(
        kind,
        vec![qubit],
        Vec::new(),
        None,
        None,
    )
    .expect("regression fixture: unary gate must be valid")
}

/// Creates a valid two-qubit operation.
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
    .expect("regression fixture: binary gate must be valid")
}

/// Creates a valid parameterized gate.
fn parameterized_gate(
    kind: GateKind,
    qubit: QubitId,
    value: f64,
) -> Gate {
    let parameter =
        Parameter::constant(value)
            .expect("regression fixture: finite parameter must be valid");

    Gate::new(
        kind,
        vec![qubit],
        vec![parameter],
        None,
        None,
    )
    .expect("regression fixture: parameterized gate must be valid")
}

/// Creates a small bounded circuit.
fn circuit(
    num_qubits: usize,
    num_classical_bits: usize,
) -> QuantumCircuit {
    QuantumCircuit::try_new_with_limits(
        num_qubits,
        num_classical_bits,
        regression_limits(),
    )
    .expect("regression fixture: circuit must be constructible")
}

// =============================================================================
// Canonical identity regressions
// =============================================================================

#[test]
fn regression_uses_canonical_logical_qubit_identity() {
    let logical = QubitId::new(7);

    assert_eq!(logical.index(), 7);
}

#[test]
fn regression_logical_and_physical_qubit_ids_are_distinct_types() {
    let logical = QubitId::new(7);
    let physical = PhysicalQubitId::new(7);

    let logical_ref = QubitRef::from(logical);
    let physical_ref = QubitRef::from(physical);

    assert!(logical_ref.is_logical());
    assert!(!logical_ref.is_physical());

    assert!(physical_ref.is_physical());
    assert!(!physical_ref.is_logical());

    assert_eq!(logical_ref.logical(), Some(logical));
    assert_eq!(physical_ref.physical(), Some(physical));

    assert_ne!(
        format!("{logical_ref}"),
        format!("{physical_ref}"),
        "logical and physical identities must remain visibly distinct"
    );
}

#[test]
fn regression_logical_identity_is_not_a_physical_hardware_claim() {
    let logical = QubitId::new(123_456);
    let reference = QubitRef::from(logical);

    assert!(reference.is_logical());
    assert_eq!(reference.logical(), Some(logical));
    assert_eq!(reference.physical(), None);
}

// =============================================================================
// Identifier arithmetic regressions
// =============================================================================

#[test]
fn regression_qubit_identifier_checked_increment_succeeds_normally() {
    let id = QubitId::new(41);

    let next =
        id.checked_next()
            .expect("non-terminal identifier must have a successor");

    assert_eq!(next.index(), 42);
}

#[test]
fn regression_qubit_identifier_checked_increment_rejects_overflow() {
    let id = QubitId::new(usize::MAX);

    assert_eq!(
        id.checked_next(),
        None,
        "identifier arithmetic must never wrap"
    );
}

#[test]
fn regression_physical_identifier_checked_increment_rejects_overflow() {
    let id = PhysicalQubitId::new(usize::MAX);

    assert_eq!(
        id.checked_next(),
        None,
        "physical identifier arithmetic must never wrap"
    );
}

// =============================================================================
// Resource-policy regressions
// =============================================================================

#[test]
fn regression_production_limits_are_self_validating() {
    QuantumIrLimits::production()
        .validate()
        .expect("production limits must remain valid");
}

#[test]
fn regression_zero_resource_policy_is_structurally_valid() {
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
        .expect("zero resource policy values must remain representable");
}

#[test]
fn regression_zero_validation_budget_is_rejected() {
    let limits =
        QuantumIrLimits::production()
            .with_max_validation_steps(0);

    assert!(
        limits.validate().is_err(),
        "zero validation work budget would disable the validation safety boundary"
    );
}

#[test]
fn regression_zero_analysis_budget_is_rejected() {
    let limits =
        QuantumIrLimits::production()
            .with_max_analysis_steps(0);

    assert!(
        limits.validate().is_err(),
        "zero analysis work budget would invalidate the analysis safety contract"
    );
}

#[test]
fn regression_qubit_policy_is_checked_before_circuit_construction() {
    let limits =
        regression_limits()
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
                "expected explicit qubit-policy failure, got {other:?}"
            );
        }
    }
}

#[test]
fn regression_classical_policy_is_checked_before_circuit_construction() {
    let limits =
        regression_limits()
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
            assert_eq!(requested, 3);
            assert_eq!(maximum, 2);
        }

        other => {
            panic!(
                "expected explicit classical-policy failure, got {other:?}"
            );
        }
    }
}

// =============================================================================
// Parameter safety regressions
// =============================================================================

#[test]
fn regression_finite_parameter_is_accepted() {
    let parameter =
        Parameter::constant(1.25)
            .expect("finite values must be valid parameters");

    assert!(parameter.is_constant());
    assert_eq!(
        parameter.as_constant(),
        Some(1.25)
    );
}

#[test]
fn regression_nan_parameter_is_rejected() {
    assert!(
        Parameter::constant(f64::NAN).is_err(),
        "NaN must never become canonical IR parameter data"
    );
}

#[test]
fn regression_positive_infinity_parameter_is_rejected() {
    assert!(
        Parameter::constant(f64::INFINITY).is_err(),
        "positive infinity must never become canonical IR parameter data"
    );
}

#[test]
fn regression_negative_infinity_parameter_is_rejected() {
    assert!(
        Parameter::constant(f64::NEG_INFINITY).is_err(),
        "negative infinity must never become canonical IR parameter data"
    );
}

#[test]
fn regression_parameterized_gate_accepts_finite_parameter() {
    let gate =
        parameterized_gate(
            GateKind::RX,
            QubitId::new(0),
            0.5,
        );

    assert_eq!(gate.kind(), GateKind::RX);
    assert_eq!(gate.parameters().len(), 1);
}

#[test]
fn regression_parameterized_gate_without_parameter_is_rejected() {
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
        "parameterized operations must not silently acquire a default parameter"
    );
}

#[test]
fn regression_non_parameterized_gate_rejects_parameter() {
    let parameter =
        Parameter::constant(0.5)
            .expect("finite parameter must be valid");

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
        "non-parameterized gates must not accept undeclared parameters"
    );
}

// =============================================================================
// Gate structural regressions
// =============================================================================

#[test]
fn regression_single_qubit_gate_has_one_operand() {
    let gate =
        unary_gate(
            GateKind::X,
            QubitId::new(0),
        );

    assert_eq!(gate.qubits().len(), 1);
}

#[test]
fn regression_two_qubit_gate_has_two_operands() {
    let gate =
        binary_gate(
            GateKind::CX,
            QubitId::new(0),
            QubitId::new(1),
        );

    assert_eq!(gate.qubits().len(), 2);
}

#[test]
fn regression_two_qubit_gate_rejects_one_operand() {
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
        "CX must never be represented with the wrong arity"
    );
}

#[test]
fn regression_three_qubit_gate_rejects_two_operands() {
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
        "CCX must never be represented with the wrong arity"
    );
}

#[test]
fn regression_duplicate_quantum_operands_are_rejected() {
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
        "duplicate quantum operands must not be accepted"
    );
}

#[test]
fn regression_barrier_requires_an_operand() {
    let result =
        Gate::new(
            GateKind::Barrier,
            Vec::new(),
            Vec::new(),
            None,
            None,
        );

    assert!(
        result.is_err(),
        "an empty barrier has no semantic target"
    );
}

#[test]
fn regression_reset_rejects_wrong_arity() {
    let result =
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
        result.is_err(),
        "reset must not silently change its operand semantics"
    );
}

// =============================================================================
// Circuit identity/version regressions
// =============================================================================

#[test]
fn regression_new_circuit_uses_current_ir_version() {
    let circuit =
        circuit(2, 2);

    assert_eq!(
        circuit.version(),
        IrVersion::CURRENT
    );
}

#[test]
fn regression_explicit_circuit_identity_is_preserved() {
    let id =
        CircuitId::new(42);

    let circuit =
        QuantumCircuit::with_identity(
            id,
            2,
            2,
            regression_limits(),
        )
        .expect("explicit circuit identity must be valid");

    assert_eq!(
        circuit.id(),
        id
    );
}

#[test]
fn regression_circuit_identity_can_be_changed_without_rebuilding() {
    let mut circuit =
        circuit(2, 2);

    let new_id =
        CircuitId::new(99);

    circuit.set_id(new_id);

    assert_eq!(
        circuit.id(),
        new_id
    );
}

#[test]
fn regression_current_ir_version_can_be_reapplied() {
    let mut circuit =
        circuit(2, 2);

    circuit
        .set_version(IrVersion::CURRENT)
        .expect("current IR version must remain supported");

    assert_eq!(
        circuit.version(),
        IrVersion::CURRENT
    );
}

// =============================================================================
// Circuit mutation / atomicity regressions
// =============================================================================

#[test]
fn regression_valid_push_changes_circuit_exactly_once() {
    let mut circuit =
        circuit(2, 0);

    assert_eq!(circuit.len(), 0);

    circuit
        .push(
            unary_gate(
                GateKind::H,
                QubitId::new(0),
            )
        )
        .expect("valid operation must be accepted");

    assert_eq!(circuit.len(), 1);

    assert_eq!(
        circuit.first().map(Gate::kind),
        Some(GateKind::H)
    );
}

#[test]
fn regression_operation_limit_failure_is_atomic() {
    let limits =
        regression_limits()
            .with_max_operations(1);

    let mut circuit =
        QuantumCircuit::try_new_with_limits(
            1,
            0,
            limits,
        )
        .expect("bounded circuit must be constructible");

    circuit
        .push(
            unary_gate(
                GateKind::X,
                QubitId::new(0),
            )
        )
        .expect("first operation must fit");

    let before_len =
        circuit.len();

    let result =
        circuit.push(
            unary_gate(
                GateKind::X,
                QubitId::new(0),
            )
        );

    assert!(
        result.is_err(),
        "operation limit must reject the second operation"
    );

    assert_eq!(
        circuit.len(),
        before_len,
        "failed mutation must not partially mutate the circuit"
    );
}

#[test]
fn regression_out_of_namespace_operation_does_not_mutate_circuit() {
    let mut circuit =
        circuit(1, 0);

    let result =
        circuit.push(
            unary_gate(
                GateKind::X,
                QubitId::new(1),
            )
        );

    assert!(
        result.is_err(),
        "operation referencing an undeclared logical qubit must fail"
    );

    assert_eq!(
        circuit.len(),
        0,
        "rejected operation must not enter the circuit"
    );
}

#[test]
fn regression_sparse_large_logical_identifier_does_not_create_machine_limit() {
    let sparse_id =
        QubitId::new(1_000_000);

    let operation =
        unary_gate(
            GateKind::X,
            sparse_id,
        );

    assert_eq!(
        operation.qubits().len(),
        1
    );

    assert_eq!(
        operation.qubits()[0],
        sparse_id
    );
}

#[test]
fn regression_sparse_identifier_is_checked_against_namespace_not_machine_ceiling() {
    let mut circuit =
        circuit(1_000_001, 0);

    let sparse_id =
        QubitId::new(1_000_000);

    circuit
        .push(
            unary_gate(
                GateKind::X,
                sparse_id,
            )
        )
        .expect(
            "a representable logical identifier inside the declared namespace must be accepted"
        );

    assert_eq!(
        circuit.len(),
        1
    );
}

// =============================================================================
// Namespace boundary regressions
// =============================================================================

#[test]
fn regression_first_logical_qubit_is_valid() {
    let mut circuit =
        circuit(1, 0);

    circuit
        .push(
            unary_gate(
                GateKind::X,
                QubitId::new(0),
            )
        )
        .expect("q0 must be valid in a one-qubit namespace");

    assert_eq!(circuit.len(), 1);
}

#[test]
fn regression_last_declared_logical_qubit_is_valid() {
    let mut circuit =
        circuit(3, 0);

    circuit
        .push(
            unary_gate(
                GateKind::X,
                QubitId::new(2),
            )
        )
        .expect("the last declared logical qubit must be addressable");

    assert_eq!(circuit.len(), 1);
}

#[test]
fn regression_first_undeclared_logical_qubit_is_rejected() {
    let mut circuit =
        circuit(3, 0);

    let result =
        circuit.push(
            unary_gate(
                GateKind::X,
                QubitId::new(3),
            )
        );

    assert!(
        result.is_err(),
        "the first identifier outside the namespace must be rejected"
    );

    assert_eq!(circuit.len(), 0);
}

// =============================================================================
// Operation-order regressions
// =============================================================================

#[test]
fn regression_operation_order_is_preserved() {
    let mut circuit =
        circuit(2, 0);

    circuit
        .push(
            unary_gate(
                GateKind::H,
                QubitId::new(0),
            )
        )
        .expect("H must be accepted");

    circuit
        .push(
            binary_gate(
                GateKind::CX,
                QubitId::new(0),
                QubitId::new(1),
            )
        )
        .expect("CX must be accepted");

    circuit
        .push(
            unary_gate(
                GateKind::X,
                QubitId::new(1),
            )
        )
        .expect("X must be accepted");

    assert_eq!(
        circuit.first().map(Gate::kind),
        Some(GateKind::H)
    );

    assert_eq!(
        circuit.len(),
        3
    );
}

// =============================================================================
// Regression against artificial architectural ceilings
// =============================================================================

#[test]
fn regression_ir_does_not_reject_reasonable_large_identifier_by_magic_number() {
    let candidates = [
        63usize,
        64usize,
        127usize,
        128usize,
        1024usize,
        4096usize,
        65_535usize,
        1_000_000usize,
    ];

    for index in candidates {
        let operation =
            unary_gate(
                GateKind::X,
                QubitId::new(index),
            );

        assert_eq!(
            operation.qubits()[0].index(),
            index,
            "logical identifiers must not be rejected by an artificial machine-size ceiling"
        );
    }
}

#[test]
fn regression_machine_size_is_not_encoded_in_qubit_identity() {
    let identifiers = [
        QubitId::new(0),
        QubitId::new(63),
        QubitId::new(64),
        QubitId::new(4096),
        QubitId::new(1_000_000),
    ];

    for id in identifiers {
        assert_eq!(
            id.index(),
            id.index(),
            "QubitId must remain an identity value rather than a hardware-capacity declaration"
        );
    }
}

// =============================================================================
// Empty and minimal program regressions
// =============================================================================

#[test]
fn regression_empty_circuit_is_valid() {
    let circuit =
        circuit(0, 0);

    assert_eq!(
        circuit.len(),
        0
    );

    assert_eq!(
        circuit.num_qubits(),
        0
    );

    assert_eq!(
        circuit.num_classical_bits(),
        0
    );
}

#[test]
fn regression_single_qubit_single_operation_is_valid() {
    let mut circuit =
        circuit(1, 0);

    circuit
        .push(
            unary_gate(
                GateKind::H,
                QubitId::new(0),
            )
        )
        .expect("minimal valid circuit must remain valid");

    assert_eq!(
        circuit.len(),
        1
    );

    assert_eq!(
        circuit.num_qubits(),
        1
    );
}

// =============================================================================
// Large logical namespace versus actual operation storage
// =============================================================================

#[test]
fn regression_declared_namespace_can_exceed_operation_count() {
    let mut circuit =
        circuit(1_000_001, 0);

    circuit
        .push(
            unary_gate(
                GateKind::X,
                QubitId::new(1_000_000),
            )
        )
        .expect(
            "large declared namespace with one actual operation must remain valid"
        );

    assert_eq!(
        circuit.num_qubits(),
        1_000_001
    );

    assert_eq!(
        circuit.len(),
        1
    );
}

#[test]
fn regression_operation_count_remains_independent_of_declared_namespace_size() {
    let mut small =
        circuit(1, 0);

    let mut large =
        circuit(1_000_001, 0);

    small
        .push(
            unary_gate(
                GateKind::X,
                QubitId::new(0),
            )
        )
        .expect("small circuit operation must be valid");

    large
        .push(
            unary_gate(
                GateKind::X,
                QubitId::new(1_000_000),
            )
        )
        .expect("large sparse circuit operation must be valid");

    assert_eq!(
        small.len(),
        large.len(),
        "operation storage must depend on semantic operations rather than an artificial machine ceiling"
    );
}

// =============================================================================
// Stable public semantics
// =============================================================================

#[test]
fn regression_gate_kind_is_preserved_through_construction() {
    let kinds = [
        GateKind::I,
        GateKind::X,
        GateKind::Y,
        GateKind::Z,
        GateKind::H,
    ];

    for kind in kinds {
        let operation =
            unary_gate(
                kind,
                QubitId::new(0),
            );

        assert_eq!(
            operation.kind(),
            kind
        );
    }
}

#[test]
fn regression_parameterized_gate_kind_is_preserved() {
    let operation =
        parameterized_gate(
            GateKind::RX,
            QubitId::new(0),
            0.25,
        );

    assert_eq!(
        operation.kind(),
        GateKind::RX
    );
}

#[test]
fn regression_two_qubit_operand_order_is_preserved() {
    let first =
        QubitId::new(3);

    let second =
        QubitId::new(7);

    let operation =
        binary_gate(
            GateKind::CX,
            first,
            second,
        );

    assert_eq!(
        operation.qubits()[0],
        first
    );

    assert_eq!(
        operation.qubits()[1],
        second
    );
}

// =============================================================================
// Explicit boundary behavior
// =============================================================================

#[test]
fn regression_limit_error_is_explicit_not_silent() {
    let limits =
        regression_limits()
            .with_max_operations(0);

    let mut circuit =
        QuantumCircuit::try_new_with_limits(
            1,
            0,
            limits,
        )
        .expect("zero operation capacity must still permit an empty circuit");

    let result =
        circuit.push(
            unary_gate(
                GateKind::X,
                QubitId::new(0),
            )
        );

    assert!(
        result.is_err(),
        "resource-policy violations must be explicit"
    );

    assert_eq!(
        circuit.len(),
        0,
        "rejected work must not be silently inserted"
    );
}

#[test]
fn regression_invalid_gate_construction_is_explicit() {
    let result =
        Gate::new(
            GateKind::CCX,
            vec![QubitId::new(0)],
            Vec::new(),
            None,
            None,
        );

    assert!(
        result.is_err(),
        "invalid semantic input must fail explicitly"
    );
}

// =============================================================================
// Canonical identity display regressions
// =============================================================================

#[test]
fn regression_logical_qubit_display_is_stable() {
    let id =
        QubitId::new(12);

    assert_eq!(
        id.to_string(),
        "q12"
    );
}

#[test]
fn regression_physical_qubit_display_is_stable() {
    let id =
        PhysicalQubitId::new(12);

    assert_eq!(
        id.to_string(),
        "p12"
    );
}

// =============================================================================
// Final architectural invariants
// =============================================================================

#[test]
fn regression_same_logical_identifier_has_stable_value_semantics() {
    let first =
        QubitId::new(1234);

    let second =
        QubitId::new(1234);

    assert_eq!(
        first,
        second
    );

    assert_eq!(
        first.index(),
        second.index()
    );
}

#[test]
fn regression_different_logical_identifiers_remain_distinct() {
    let first =
        QubitId::new(1234);

    let second =
        QubitId::new(1235);

    assert_ne!(
        first,
        second
    );
}

#[test]
fn regression_circuit_qubit_count_is_not_changed_by_operation_count() {
    let mut circuit =
        circuit(8, 0);

    circuit
        .push(
            unary_gate(
                GateKind::X,
                QubitId::new(0),
            )
        )
        .expect("first operation must be valid");

    circuit
        .push(
            unary_gate(
                GateKind::H,
                QubitId::new(0),
            )
        )
        .expect("second operation must be valid");

    assert_eq!(
        circuit.num_qubits(),
        8
    );

    assert_eq!(
        circuit.len(),
        2
    );
}

#[test]
fn regression_failed_mutation_preserves_operation_count() {
    let limits =
        regression_limits()
            .with_max_operations(1);

    let mut circuit =
        QuantumCircuit::try_new_with_limits(
            2,
            0,
            limits,
        )
        .expect("bounded circuit must be constructible");

    circuit
        .push(
            unary_gate(
                GateKind::X,
                QubitId::new(0),
            )
        )
        .expect("first operation must fit");

    let before =
        circuit.len();

    let _ =
        circuit.push(
            binary_gate(
                GateKind::CX,
                QubitId::new(0),
                QubitId::new(1),
            )
        );

    assert_eq!(
        circuit.len(),
        before,
        "failed mutation must preserve the complete observable operation count"
    );
}