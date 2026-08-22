//! Zamani Quantum IR — Integration Contract Tests.
//!
//! This module is the cross-component test suite for the canonical,
//! hardware-independent quantum IR.
//!
//! # Purpose
//!
//! Individual IR modules contain their own focused unit tests. This file tests
//! the contracts BETWEEN those modules:
//!
//! ```text
//! limits
//!    │
//!    ├── qubits
//!    ├── parameters
//!    ├── measurements
//!    └── gates
//!          │
//!          ▼
//!      QuantumCircuit
//!          │
//!          ├── validation
//!          └── analysis
//! ```
//!
//! These tests deliberately exercise only the public IR contracts. They do not
//! access private implementation fields. This makes the suite useful as a
//! compatibility boundary: if an implementation changes internally while the
//! public IR contract remains valid, these tests should continue to pass.
//!
//! # Test principles
//!
//! The suite verifies:
//!
//! - valid IR construction;
//! - invalid IR rejection;
//! - finite parameter enforcement;
//! - logical namespace enforcement;
//! - classical namespace enforcement;
//! - operation-count limits;
//! - metadata limits;
//! - measurement/barrier limits;
//! - atomic mutation;
//! - failed mutations do not change the circuit;
//! - whole-circuit validation;
//! - deterministic analysis;
//! - deterministic gate histograms;
//! - deterministic logical-qubit accounting;
//! - explicit IR versioning;
//! - explicit circuit identity;
//! - safe replacement/insertion/removal;
//! - construction from operation sequences;
//! - validation of externally reconstructed/untrusted IR;
//! - analysis work-budget enforcement;
//! - separation of logical and physical qubit identities.
//!
//! # Architectural boundary
//!
//! These tests must NOT test:
//!
//! - physical topology;
//! - routing;
//! - scheduling;
//! - calibration;
//! - backend execution;
//! - pulse generation;
//! - QPU communication;
//! - error-correction decoding;
//! - optimization algorithms.
//!
//! Those are downstream concerns.
//!
//! # Rust compatibility
//!
//! Rust 1.97.1.
//!
//! No nightly features.
//! No external test dependencies.

use super::circuit::{CircuitError, QuantumCircuit};
use super::gate::{Gate, GateKind};
use super::identity::{CircuitId, IrVersion};
use super::limits::QuantumIrLimits;
use super::parameter::Parameter;
use super::qubits::{PhysicalQubitId, QubitId};
use super::analysis::{
    analyze,
    analyze_with_limits,
    basic_statistics,
    basic_statistics_with_limits,
};

// =============================================================================
// Test helpers
// =============================================================================

/// Creates a valid one-qubit gate without parameters.
fn gate(
    kind: GateKind,
    qubit: usize,
) -> Gate {
    Gate::new(
        kind,
        vec![QubitId::new(qubit)],
        Vec::new(),
        None,
        None,
    )
    .expect("test gate must be valid")
}

/// Creates a valid two-qubit gate.
fn two_qubit_gate(
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
    .expect("test two-qubit gate must be valid")
}

/// Creates a valid parameterized one-qubit gate.
fn parameterized_gate(
    kind: GateKind,
    qubit: usize,
    value: f64,
) -> Gate {
    let parameter =
        Parameter::constant(value)
            .expect("finite test parameter must be valid");

    Gate::new(
        kind,
        vec![QubitId::new(qubit)],
        vec![parameter],
        None,
        None,
    )
    .expect("parameterized test gate must be valid")
}

/// Creates a circuit with explicit small limits.
///
/// Small limits are preferable for security/contract tests because failures
/// can be demonstrated without allocating large structures.
fn small_limits() -> QuantumIrLimits {
    QuantumIrLimits::production()
        .with_max_qubits(8)
        .with_max_classical_bits(8)
        .with_max_operations(16)
        .with_max_operands(8)
        .with_max_parameters(8)
        .with_max_metadata_bytes(64)
        .with_max_depth(16)
        .with_max_measurements(4)
        .with_max_barriers(4)
        .with_max_validation_steps(1_000)
        .with_max_analysis_steps(1_000)
}

/// Creates a small valid circuit used by several tests.
fn small_circuit() -> QuantumCircuit {
    QuantumCircuit::try_new_with_limits(
        3,
        3,
        small_limits(),
    )
    .expect("small test circuit must be valid")
}

// =============================================================================
// Limits
// =============================================================================

#[test]
fn production_limits_are_valid() {
    let limits =
        QuantumIrLimits::production();

    limits
        .validate()
        .expect("production limits must be valid");
}

#[test]
fn deny_all_limits_are_valid() {
    let limits =
        QuantumIrLimits::deny_all();

    limits
        .validate()
        .expect("deny-all limits must remain structurally valid");
}

#[test]
fn zero_resource_limits_are_supported() {
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
        .expect("zero resource limits should be legal");
}

#[test]
fn invalid_zero_validation_budget_is_rejected() {
    let limits =
        QuantumIrLimits::production()
            .with_max_validation_steps(0);

    assert!(
        limits.validate().is_err(),
        "validation work budget must be non-zero"
    );
}

#[test]
fn invalid_zero_analysis_budget_is_rejected() {
    let limits =
        QuantumIrLimits::production()
            .with_max_analysis_steps(0);

    assert!(
        limits.validate().is_err(),
        "analysis work budget must be non-zero"
    );
}

#[test]
fn qubit_limit_is_enforced_before_circuit_construction() {
    let limits =
        small_limits().with_max_qubits(2);

    let result =
        QuantumCircuit::try_new_with_limits(
            3,
            0,
            limits,
        );

    assert!(result.is_err());

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
                "unexpected error: {other:?}"
            );
        }
    }
}

#[test]
fn classical_bit_limit_is_enforced_before_circuit_construction() {
    let limits =
        small_limits()
            .with_max_classical_bits(2);

    let result =
        QuantumCircuit::try_new_with_limits(
            0,
            3,
            limits,
        );

    assert!(result.is_err());

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
                "unexpected error: {other:?}"
            );
        }
    }
}

#[test]
fn operation_limit_is_enforced() {
    let limits =
        small_limits().with_max_operations(1);

    let mut circuit =
        QuantumCircuit::try_new_with_limits(
            1,
            0,
            limits,
        )
        .expect("circuit must be constructible");

    circuit
        .push(gate(GateKind::X, 0))
        .expect("first operation must fit");

    let result =
        circuit.push(gate(GateKind::X, 0));

    assert!(result.is_err());
    assert_eq!(circuit.len(), 1);
}

// =============================================================================
// Parameter contract
// =============================================================================

#[test]
fn finite_parameter_is_accepted() {
    let parameter =
        Parameter::constant(1.25)
            .expect("finite parameter must be accepted");

    assert!(parameter.is_constant());
    assert_eq!(
        parameter.as_constant(),
        Some(1.25)
    );
}

#[test]
fn_nan_parameter_is_rejected() {
    let result =
        Parameter::constant(f64::NAN);

    assert!(
        result.is_err(),
        "NaN must never enter the IR"
    );
}

#[test]
fn positive_infinity_parameter_is_rejected() {
    let result =
        Parameter::constant(f64::INFINITY);

    assert!(
        result.is_err(),
        "positive infinity must never enter the IR"
    );
}

#[test]
fn negative_infinity_parameter_is_rejected() {
    let result =
        Parameter::constant(f64::NEG_INFINITY);

    assert!(
        result.is_err(),
        "negative infinity must never enter the IR"
    );
}

#[test]
fn parameterized_gate_requires_its_parameter() {
    let result = Gate::new(
        GateKind::RX,
        vec![QubitId::new(0)],
        Vec::new(),
        None,
        None,
    );

    assert!(
        result.is_err(),
        "RX without a parameter must be rejected"
    );
}

#[test]
fn non_parameterized_gate_rejects_parameters() {
    let parameter =
        Parameter::constant(0.5)
            .expect("finite parameter");

    let result = Gate::new(
        GateKind::X,
        vec![QubitId::new(0)],
        vec![parameter],
        None,
        None,
    );

    assert!(
        result.is_err(),
        "X must not accept parameters"
    );
}

#[test]
fn parameterized_gate_with_finite_parameter_is_valid() {
    let operation =
        parameterized_gate(
            GateKind::RX,
            0,
            0.5,
        );

    assert_eq!(
        operation.kind(),
        GateKind::RX
    );

    assert_eq!(
        operation.parameters().len(),
        1
    );
}

// =============================================================================
// Gate structural contract
// =============================================================================

#[test]
fn single_qubit_gate_accepts_one_operand() {
    let operation =
        gate(GateKind::X, 0);

    assert_eq!(
        operation.qubits().len(),
        1
    );
}

#[test]
fn two_qubit_gate_accepts_two_operands() {
    let operation =
        two_qubit_gate(
            GateKind::CX,
            0,
            1,
        );

    assert_eq!(
        operation.qubits().len(),
        2
    );
}

#[test]
fn two_qubit_gate_rejects_one_operand() {
    let result = Gate::new(
        GateKind::CX,
        vec![QubitId::new(0)],
        Vec::new(),
        None,
        None,
    );

    assert!(result.is_err());
}

#[test]
fn three_qubit_gate_rejects_two_operands() {
    let result = Gate::new(
        GateKind::CCX,
        vec![
            QubitId::new(0),
            QubitId::new(1),
        ],
        Vec::new(),
        None,
        None,
    );

    assert!(result.is_err());
}

#[test]
fn duplicate_qubit_operands_are_rejected() {
    let result = Gate::new(
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
        "an operation must not contain duplicate logical operands"
    );
}

#[test]
fn empty_barrier_is_rejected() {
    let result = Gate::new(
        GateKind::Barrier,
        Vec::new(),
        Vec::new(),
        None,
        None,
    );

    assert!(
        result.is_err(),
        "barriers require at least one operand"
    );
}

#[test]
fn reset_requires_exactly_one_qubit() {
    let result = Gate::new(
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
        "reset must target exactly one logical qubit"
    );
}

// =============================================================================
// Circuit construction
// =============================================================================

#[test]
fn circuit_construction_is_fallible_and_bounded() {
    let circuit =
        small_circuit();

    assert_eq!(
        circuit.num_qubits(),
        3
    );

    assert_eq!(
        circuit.num_classical_bits(),
        3
    );

    assert!(circuit.is_empty());
}

#[test]
fn circuit_starts_with_current_ir_version() {
    let circuit =
        small_circuit();

    assert_eq!(
        circuit.version(),
        IrVersion::CURRENT
    );
}

#[test]
fn circuit_identity_can_be_explicitly_assigned() {
    let id =
        CircuitId::new(42);

    let circuit =
        QuantumCircuit::with_identity(
            id,
            2,
            2,
            small_limits(),
        )
        .expect("identity-bearing circuit must be valid");

    assert_eq!(
        circuit.id(),
        id
    );
}

#[test]
fn circuit_identity_can_be_changed_without_rebuilding() {
    let mut circuit =
        small_circuit();

    let id =
        CircuitId::new(99);

    circuit.set_id(id);

    assert_eq!(
        circuit.id(),
        id
    );
}

#[test]
fn current_ir_version_is_supported() {
    let mut circuit =
        small_circuit();

    circuit
        .set_version(IrVersion::CURRENT)
        .expect("current IR version must be supported");

    assert_eq!(
        circuit.version(),
        IrVersion::CURRENT
    );
}

// =============================================================================
// Circuit mutation contract
// =============================================================================

#[test]
fn push_appends_valid_operation() {
    let mut circuit =
        small_circuit();

    circuit
        .push(gate(GateKind::H, 0))
        .expect("valid operation must be appended");

    assert_eq!(circuit.len(), 1);
    assert_eq!(
        circuit.first().map(Gate::kind),
        Some(GateKind::H)
    );
}

#[test]
fn insert_preserves_order() {
    let mut circuit =
        small_circuit();

    circuit
        .push(gate(GateKind::X, 0))
        .expect("X must be inserted");

    circuit
        .push(gate(GateKind::Z, 0))
        .expect("Z must be inserted");

    circuit
        .insert(
            1,
            gate(GateKind::H, 0),
        )
        .expect("H must be inserted");

    assert_eq!(
        circuit
            .get(0)
            .map(Gate::kind),
        Some(GateKind::X)
    );

    assert_eq!(
        circuit
            .get(1)
            .map(Gate::kind),
        Some(GateKind::H)
    );

    assert_eq!(
        circuit
            .get(2)
            .map(Gate::kind),
        Some(GateKind::Z)
    );
}

#[test]
fn insert_out_of_range_is_rejected_without_mutation() {
    let mut circuit =
        small_circuit();

    circuit
        .push(gate(GateKind::X, 0))
        .expect("initial operation must be valid");

    let before =
        circuit.operations().to_vec();

    let result =
        circuit.insert(
            99,
            gate(GateKind::H, 0),
        );

    assert!(result.is_err());

    assert_eq!(
        circuit.operations(),
        before.as_slice()
    );
}

#[test]
fn replace_is_atomic() {
    let mut circuit =
        small_circuit();

    circuit
        .push(gate(GateKind::X, 0))
        .expect("initial operation must be valid");

    let old =
        circuit
            .replace(
                0,
                gate(GateKind::H, 0),
            )
            .expect("replacement must be valid");

    assert_eq!(
        old.kind(),
        GateKind::X
    );

    assert_eq!(
        circuit.first().map(Gate::kind),
        Some(GateKind::H)
    );
}

#[test]
fn failed_replace_does_not_remove_original_operation() {
    let mut circuit =
        small_circuit();

    circuit
        .push(gate(GateKind::X, 0))
        .expect("initial operation must be valid");

    let invalid_gate =
        Gate::new(
            GateKind::CX,
            vec![QubitId::new(0)],
            Vec::new(),
            None,
            None,
        );

    assert!(invalid_gate.is_err());

    assert_eq!(
        circuit.first().map(Gate::kind),
        Some(GateKind::X)
    );
}

#[test]
fn remove_returns_the_removed_operation() {
    let mut circuit =
        small_circuit();

    circuit
        .push(gate(GateKind::X, 0))
        .expect("operation must be valid");

    let removed =
        circuit
            .remove(0)
            .expect("operation must exist");

    assert_eq!(
        removed.kind(),
        GateKind::X
    );

    assert!(circuit.is_empty());
}

#[test]
fn remove_out_of_range_is_rejected_without_mutation() {
    let mut circuit =
        small_circuit();

    circuit
        .push(gate(GateKind::X, 0))
        .expect("operation must be valid");

    let before =
        circuit.operations().to_vec();

    let result =
        circuit.remove(99);

    assert!(result.is_err());

    assert_eq!(
        circuit.operations(),
        before.as_slice()
    );
}

#[test]
fn clear_removes_all_operations() {
    let mut circuit =
        small_circuit();

    circuit
        .push(gate(GateKind::X, 0))
        .expect("X must be valid");

    circuit
        .push(gate(GateKind::H, 1))
        .expect("H must be valid");

    assert_eq!(circuit.len(), 2);

    circuit.clear();

    assert_eq!(circuit.len(), 0);
    assert!(circuit.is_empty());
}

// =============================================================================
// Namespace safety
// =============================================================================

#[test]
fn out_of_range_logical_qubit_is_rejected_by_circuit() {
    let mut circuit =
        small_circuit();

    let result =
        circuit.push(
            gate(GateKind::X, 99)
        );

    assert!(result.is_err());

    assert!(
        circuit.is_empty(),
        "failed insertion must not mutate the circuit"
    );
}

#[test]
fn operation_using_valid_qubits_is_accepted() {
    let mut circuit =
        small_circuit();

    circuit
        .push(
            two_qubit_gate(
                GateKind::CX,
                0,
                2,
            ),
        )
        .expect("both logical qubits belong to the circuit");

    assert_eq!(circuit.len(), 1);
}

#[test]
fn logical_and_physical_qubit_ids_are_distinct_types() {
    let logical =
        QubitId::new(3);

    let physical =
        PhysicalQubitId::new(3);

    assert_eq!(
        logical.index(),
        physical.index()
    );

    // This assertion intentionally verifies semantic identity through the
    // distinct APIs. The types themselves are not interchangeable.
    assert_eq!(logical, QubitId::new(3));
    assert_eq!(
        physical,
        PhysicalQubitId::new(3)
    );
}

// =============================================================================
// Whole-circuit validation
// =============================================================================

#[test]
fn valid_circuit_passes_whole_ir_validation() {
    let mut circuit =
        small_circuit();

    circuit
        .push(gate(GateKind::H, 0))
        .expect("H must be valid");

    circuit
        .push(
            two_qubit_gate(
                GateKind::CX,
                0,
                1,
            ),
        )
        .expect("CX must be valid");

    circuit
        .push(
            parameterized_gate(
                GateKind::RX,
                2,
                0.25,
            ),
        )
        .expect("RX must be valid");

    circuit
        .validate()
        .expect("complete circuit must validate");
}

#[test]
fn empty_circuit_is_valid_under_default_contract() {
    let circuit =
        small_circuit();

    circuit
        .validate()
        .expect("empty circuit must be valid");
}

#[test]
fn circuit_created_from_operations_validates() {
    let operations = vec![
        gate(GateKind::H, 0),
        two_qubit_gate(
            GateKind::CX,
            0,
            1,
        ),
    ];

    let circuit =
        QuantumCircuit::from_operations_with_limits(
            2,
            0,
            operations,
            small_limits(),
        )
        .expect(
            "valid operation sequence must construct a circuit",
        );

    circuit
        .validate()
        .expect("constructed circuit must validate");
}

#[test]
fn invalid_operation_sequence_is_rejected() {
    let invalid_gate =
        Gate::new(
            GateKind::CX,
            vec![QubitId::new(0)],
            Vec::new(),
            None,
            None,
        );

    assert!(invalid_gate.is_err());
}

// =============================================================================
// Metadata/resource policy
// =============================================================================

#[test]
fn metadata_is_bounded_by_circuit_limits() {
    let limits =
        small_limits()
            .with_max_metadata_bytes(4);

    let mut circuit =
        QuantumCircuit::try_new_with_limits(
            1,
            0,
            limits,
        )
        .expect("circuit must be valid");

    let result =
        circuit.set_name(
            Some("toolong".to_string())
        );

    assert!(result.is_err());
    assert!(
        circuit.metadata().name().is_none(),
        "failed metadata update must be atomic"
    );
}

#[test]
fn valid_metadata_is_stored() {
    let mut circuit =
        small_circuit();

    circuit
        .set_name(
            Some("bell".to_string())
        )
        .expect("name fits limits");

    circuit
        .set_source(
            Some("test".to_string())
        )
        .expect("source fits limits");

    circuit
        .set_compiler_version(
            Some("1.0".to_string())
        )
        .expect("compiler version fits limits");

    assert_eq!(
        circuit.metadata().name(),
        Some("bell")
    );

    assert_eq!(
        circuit.metadata().source(),
        Some("test")
    );

    assert_eq!(
        circuit.metadata().compiler_version(),
        Some("1.0")
    );
}

#[test]
fn replacing_limits_is_atomic() {
    let mut circuit =
        small_circuit();

    let original =
        *circuit.limits();

    let invalid =
        QuantumIrLimits::production()
            .with_max_qubits(1);

    let result =
        circuit.set_limits(invalid);

    assert!(result.is_err());

    assert_eq!(
        *circuit.limits(),
        original,
        "failed limit replacement must not mutate the circuit"
    );
}

// =============================================================================
// Deterministic analysis
// =============================================================================

#[test]
fn analysis_counts_basic_operation_categories() {
    let mut circuit =
        small_circuit();

    circuit
        .push(gate(GateKind::H, 0))
        .expect("H must be valid");

    circuit
        .push(
            two_qubit_gate(
                GateKind::CX,
                0,
                1,
            ),
        )
        .expect("CX must be valid");

    circuit
        .push(
            parameterized_gate(
                GateKind::RX,
                2,
                0.5,
            ),
        )
        .expect("RX must be valid");

    let statistics =
        analyze(&circuit)
            .expect("analysis must succeed");

    assert_eq!(
        statistics.operation_count(),
        3
    );

    assert_eq!(
        statistics.single_qubit_operations(),
        2
    );

    assert_eq!(
        statistics.two_qubit_operations(),
        1
    );

    assert_eq!(
        statistics.multi_qubit_operations(),
        0
    );

    assert_eq!(
        statistics.parameterized_operations(),
        1
    );
}

#[test]
fn analysis_counts_used_qubits_deterministically() {
    let mut circuit =
        small_circuit();

    circuit
        .push(gate(GateKind::X, 2))
        .expect("X must be valid");

    circuit
        .push(gate(GateKind::H, 0))
        .expect("H must be valid");

    circuit
        .push(
            two_qubit_gate(
                GateKind::CX,
                0,
                2,
            ),
        )
        .expect("CX must be valid");

    let statistics =
        analyze(&circuit)
            .expect("analysis must succeed");

    assert_eq!(
        statistics.qubits_used(),
        2
    );
}

#[test]
fn analysis_is_deterministic_for_identical_circuits() {
    let mut first =
        small_circuit();

    let mut second =
        small_circuit();

    let operations = [
        gate(GateKind::H, 0),
        two_qubit_gate(
            GateKind::CX,
            0,
            1,
        ),
        parameterized_gate(
            GateKind::RZ,
            2,
            0.75,
        ),
    ];

    for operation in operations.iter() {
        first
            .push(operation.clone())
            .expect("operation must be valid");

        second
            .push(operation.clone())
            .expect("operation must be valid");
    }

    let first_statistics =
        analyze(&first)
            .expect("first analysis must succeed");

    let second_statistics =
        analyze(&second)
            .expect("second analysis must succeed");

    assert_eq!(
        first_statistics,
        second_statistics,
        "identical logical IR must produce identical analysis"
    );
}

#[test]
fn gate_histogram_is_deterministic() {
    let mut circuit =
        small_circuit();

    circuit
        .push(gate(GateKind::H, 0))
        .expect("H must be valid");

    circuit
        .push(gate(GateKind::X, 1))
        .expect("X must be valid");

    circuit
        .push(gate(GateKind::H, 2))
        .expect("H must be valid");

    let statistics =
        analyze(&circuit)
            .expect("analysis must succeed");

    assert_eq!(
        statistics.gate_count(GateKind::H),
        2
    );

    assert_eq!(
        statistics.gate_count(GateKind::X),
        1
    );

    assert_eq!(
        statistics.gate_count(GateKind::Z),
        0
    );

    let histogram =
        statistics.gate_histogram();

    assert_eq!(
        histogram.len(),
        2
    );

    // Histogram ordering is first-appearance ordering, not hash-map ordering.
    assert_eq!(
        histogram[0].kind(),
        GateKind::H
    );

    assert_eq!(
        histogram[1].kind(),
        GateKind::X
    );
}

#[test]
fn basic_analysis_matches_complete_analysis() {
    let mut circuit =
        small_circuit();

    circuit
        .push(gate(GateKind::H, 0))
        .expect("H must be valid");

    circuit
        .push(
            parameterized_gate(
                GateKind::RX,
                1,
                0.25,
            ),
        )
        .expect("RX must be valid");

    let complete =
        analyze(&circuit)
            .expect("complete analysis must succeed");

    let basic =
        basic_statistics(&circuit)
            .expect("basic analysis must succeed");

    assert_eq!(
        complete.operation_count(),
        basic.operation_count()
    );

    assert_eq!(
        complete.depth(),
        basic.depth()
    );

    assert_eq!(
        complete.parameterized_operations(),
        basic.parameterized_operations()
    );

    assert_eq!(
        complete.unitary_operations(),
        basic.unitary_operations()
    );

    assert_eq!(
        complete.non_unitary_operations(),
        basic.non_unitary_operations()
    );
}

// =============================================================================
// Analysis resource budget
// =============================================================================

#[test]
fn analysis_respects_explicit_work_budget() {
    let limits =
        small_limits()
            .with_max_analysis_steps(1);

    let circuit =
        QuantumCircuit::try_new_with_limits(
            1,
            0,
            limits,
        )
        .expect("empty circuit must fit the limits");

    let result =
        analyze_with_limits(
            &circuit,
            &limits,
        );

    assert!(
        result.is_err(),
        "analysis must respect its explicit work budget"
    );
}

#[test]
fn basic_analysis_respects_explicit_work_budget() {
    let limits =
        small_limits()
            .with_max_analysis_steps(1);

    let circuit =
        QuantumCircuit::try_new_with_limits(
            1,
            0,
            limits,
        )
        .expect("empty circuit must fit the limits");

    let result =
        basic_statistics_with_limits(
            &circuit,
            &limits,
        );

    assert!(
        result.is_err(),
        "basic analysis must respect its explicit work budget"
    );
}

// =============================================================================
// Analysis resource counters
// =============================================================================

#[test]
fn analysis_reports_declared_namespace_sizes() {
    let circuit =
        small_circuit();

    let statistics =
        analyze(&circuit)
            .expect("analysis must succeed");

    assert_eq!(
        statistics.qubits(),
        3
    );

    assert_eq!(
        statistics.classical_bits(),
        3
    );
}

#[test]
fn analysis_reports_unitary_and_non_unitary_counts() {
    let mut circuit =
        small_circuit();

    circuit
        .push(gate(GateKind::X, 0))
        .expect("X must be valid");

    circuit
        .push(gate(GateKind::Barrier, 0))
        .expect("barrier must be valid");

    let statistics =
        analyze(&circuit)
            .expect("analysis must succeed");

    assert_eq!(
        statistics.unitary_operations(),
        1
    );

    assert_eq!(
        statistics.non_unitary_operations(),
        1
    );
}

#[test]
fn barrier_is_counted_as_non_unitary_ir_operation() {
    let mut circuit =
        small_circuit();

    circuit
        .push(
            gate(
                GateKind::Barrier,
                0,
            ),
        )
        .expect("barrier must be valid");

    let statistics =
        analyze(&circuit)
            .expect("analysis must succeed");

    assert_eq!(
        statistics.barrier_count(),
        1
    );

    assert_eq!(
        statistics.non_unitary_operations(),
        1
    );
}

#[test]
fn reset_is_counted_as_non_unitary_ir_operation() {
    let mut circuit =
        small_circuit();

    circuit
        .push(
            gate(
                GateKind::Reset,
                0,
            ),
        )
        .expect("reset must be valid");

    let statistics =
        analyze(&circuit)
            .expect("analysis must succeed");

    assert_eq!(
        statistics.reset_count(),
        1
    );

    assert_eq!(
        statistics.non_unitary_operations(),
        1
    );
}

// =============================================================================
// Operation sequence contract
// =============================================================================

#[test]
fn into_operations_preserves_order() {
    let operations = vec![
        gate(GateKind::X, 0),
        gate(GateKind::H, 1),
        gate(GateKind::Z, 2),
    ];

    let circuit =
        QuantumCircuit::from_operations_with_limits(
            3,
            0,
            operations,
            small_limits(),
        )
        .expect(
            "operation sequence must be valid",
        );

    let operations =
        circuit.into_operations();

    assert_eq!(
        operations.len(),
        3
    );

    assert_eq!(
        operations[0].kind(),
        GateKind::X
    );

    assert_eq!(
        operations[1].kind(),
        GateKind::H
    );

    assert_eq!(
        operations[2].kind(),
        GateKind::Z
    );
}

#[test]
fn operation_access_is_read_only() {
    let mut circuit =
        small_circuit();

    circuit
        .push(gate(GateKind::X, 0))
        .expect("operation must be valid");

    let operations =
        circuit.operations();

    assert_eq!(
        operations.len(),
        1
    );

    assert_eq!(
        operations[0].kind(),
        GateKind::X
    );
}

// =============================================================================
// Regression tests for invariant preservation
// =============================================================================

#[test]
fn failed_push_does_not_change_operation_count() {
    let mut circuit =
        small_circuit();

    circuit
        .push(gate(GateKind::X, 0))
        .expect("initial operation must be valid");

    let before =
        circuit.len();

    let result =
        circuit.push(
            gate(GateKind::X, 99)
        );

    assert!(result.is_err());
    assert_eq!(
        circuit.len(),
        before
    );
}

#[test]
fn failed_insert_does_not_change_operation_order() {
    let mut circuit =
        small_circuit();

    circuit
        .push(gate(GateKind::X, 0))
        .expect("X must be valid");

    circuit
        .push(gate(GateKind::Z, 1))
        .expect("Z must be valid");

    let before =
        circuit.operations().to_vec();

    let result =
        circuit.insert(
            1,
            gate(GateKind::CX, 0),
        );

    assert!(result.is_err());

    assert_eq!(
        circuit.operations(),
        before.as_slice()
    );
}

#[test]
fn circuit_validation_remains_available_for_untrusted_ir_boundary() {
    let circuit =
        small_circuit();

    // The public API guarantees validity during construction, but the explicit
    // validation boundary remains part of the contract for future deserializers,
    // replay systems, frontends, optimizers, and external IR producers.
    circuit
        .validate()
        .expect(
            "trusted construction must still pass canonical validation"
        );
}

// =============================================================================
// Gate semantic classification
// =============================================================================

#[test]
fn gate_kind_classification_is_stable() {
    assert!(
        GateKind::X.is_unitary()
    );

    assert!(
        !GateKind::Measure.is_unitary()
    );

    assert!(
        !GateKind::Barrier.is_unitary()
    );

    assert!(
        !GateKind::Reset.is_unitary()
    );

    assert!(
        GateKind::RX.is_parameterized()
    );

    assert!(
        !GateKind::X.is_parameterized()
    );

    assert!(
        GateKind::Measure.is_measurement()
    );

    assert!(
        GateKind::Barrier.is_barrier()
    );

    assert!(
        GateKind::Reset.is_reset()
    );
}

#[test]
fn gate_operand_contract_is_stable() {
    assert!(
        GateKind::X
            .operand_count()
            .accepts(1)
    );

    assert!(
        !GateKind::X
            .operand_count()
            .accepts(2)
    );

    assert!(
        GateKind::CX
            .operand_count()
            .accepts(2)
    );

    assert!(
        GateKind::CCX
            .operand_count()
            .accepts(3)
    );

    assert!(
        GateKind::Barrier
            .operand_count()
            .accepts(1)
    );

    assert!(
        GateKind::Barrier
            .operand_count()
            .accepts(8)
    );
}

#[test]
fn parameter_arity_contract_is_stable() {
    assert_eq!(
        GateKind::X.parameter_count(),
        0
    );

    assert_eq!(
        GateKind::RX.parameter_count(),
        1
    );

    assert_eq!(
        GateKind::U2.parameter_count(),
        2
    );

    assert_eq!(
        GateKind::U3.parameter_count(),
        3
    );
}

// =============================================================================
// Final integration path
// =============================================================================

#[test]
fn complete_ir_pipeline_contract() {
    // 1. Resource policy.
    let limits =
        small_limits();

    // 2. Circuit namespace.
    let mut circuit =
        QuantumCircuit::try_new_with_limits(
            3,
            3,
            limits,
        )
        .expect(
            "circuit namespace must be valid"
        );

    // 3. Logical operations.
    circuit
        .push(gate(GateKind::H, 0))
        .expect("H must be valid");

    circuit
        .push(
            two_qubit_gate(
                GateKind::CX,
                0,
                1,
            ),
        )
        .expect("CX must be valid");

    circuit
        .push(
            parameterized_gate(
                GateKind::RZ,
                2,
                1.0,
            ),
        )
        .expect("RZ must be valid");

    // 4. Whole-IR validation.
    circuit
        .validate()
        .expect(
            "canonical validation must succeed"
        );

    // 5. Deterministic analysis.
    let statistics =
        analyze(&circuit)
            .expect(
                "canonical analysis must succeed"
            );

    assert_eq!(
        statistics.operation_count(),
        3
    );

    assert_eq!(
        statistics.qubits_used(),
        3
    );

    assert_eq!(
        statistics.single_qubit_operations(),
        2
    );

    assert_eq!(
        statistics.two_qubit_operations(),
        1
    );

    assert_eq!(
        statistics.parameterized_operations(),
        1
    );

    // The pipeline must remain entirely hardware-independent.
    assert_eq!(
        circuit.num_qubits(),
        3
    );
}