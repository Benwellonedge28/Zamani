//! Zamani Quantum Optimization — Equivalence Integration Tests
//!
//! Production-grade integration and contract tests for the canonical quantum
//! circuit equivalence subsystem.
//!
//! # Location
//!
//! ```text
//! src/quantum/optimization/tests/equivalence.rs
//! ```
//!
//! # Architectural role
//!
//! This file tests the public contract between:
//!
//! ```text
//! quantum::ir
//!      │
//!      ├── QuantumCircuit
//!      ├── Gate
//!      ├── GateKind
//!      ├── Parameter
//!      └── qubit::QubitId
//!      │
//!      ▼
//! quantum::optimization::equivalence
//!      │
//!      ├── structural equivalence
//!      ├── exact unitary equivalence
//!      ├── global-phase-aware equivalence
//!      ├── bounded verification
//!      ├── deterministic fingerprints
//!      └── explicit inconclusive results
//! ```
//!
//! The tests intentionally consume public APIs rather than private
//! implementation details. This makes the suite stable while the internal
//! verifier evolves from dense-state verification toward future stabilizer,
//! tensor-network, decision-diagram, randomized, symbolic, and
//! certificate-backed engines.
//!
//! # Important repository integration rule
//!
//! The canonical logical-qubit module is:
//!
//! ```text
//! crate::quantum::ir::qubit
//! ```
//!
//! NOT:
//!
//! ```text
//! crate::quantum::ir::qubits
//! ```
//!
//! All logical qubit construction in this file therefore uses:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! ```
//!
//! This deliberately protects the test suite from reproducing the existing
//! IR module-name inconsistency.
//!
//! # Verification contract tested here
//!
//! The equivalence subsystem has three semantically distinct outcomes:
//!
//! ```text
//! Equivalent
//!     = equivalence was proven
//!
//! NotEquivalent
//!     = non-equivalence was proven
//!
//! Inconclusive
//!     = the verifier could not safely establish either result
//! ```
//!
//! `Inconclusive` must never be treated as `Equivalent`.
//!
//! # Scalability contract
//!
//! These tests deliberately cover:
//!
//! - empty circuits;
//! - tiny one-qubit circuits;
//! - two-qubit circuits;
//! - three-qubit circuits;
//! - parameterized circuits;
//! - non-unitary circuits;
//! - unsupported operations;
//! - large operation counts;
//! - bounded verification;
//! - deterministic fingerprints.
//!
//! No test assumes a fixed architectural maximum circuit size.
//!
//! Resource limits are explicitly exercised where a dense verifier would be
//! expected to stop. Future verification engines may handle larger circuits
//! without requiring these tests to change their semantic expectations.
//!
//! # Safety
//!
//! - Rust 1.97 / 1.97.1 compatible.
//! - Rust 2021 compatible.
//! - No `unsafe`.
//! - No direct backend or QPU execution.
//! - No filesystem/network access.
//! - No randomness.
//! - No reliance on optimizer implementation internals.
//!
//! # Integration contract
//!
//! The optimization test module should eventually expose this file through:
//!
//! ```text
//! #[cfg(test)]
//! mod equivalence;
//! ```
//!
//! from:
//!
//! ```text
//! src/quantum/optimization/tests/mod.rs
//! ```
//!
//! and the optimization root should include the test module through its
//! normal `#[cfg(test)]` hierarchy.
//!
//! This file itself intentionally requires no changes when additional
//! optimization passes, analyses, synthesis engines, or verification engines
//! are introduced.
//!
//! -----------------------------------------------------------------------------
//! Imports
//! -----------------------------------------------------------------------------

use std::f64::consts::PI;
use std::time::Duration;

use crate::quantum::ir::gate::{Gate, GateKind};
use crate::quantum::ir::parameter::Parameter;
use crate::quantum::ir::qubit::QubitId;
use crate::quantum::ir::QuantumCircuit;
use crate::quantum::optimization::equivalence::{
    structural_fingerprint,
    verify,
    verify_structural,
    verify_unitary,
    EquivalenceConfig,
    EquivalenceLimits,
    EquivalenceMethod,
    EquivalenceVerdict,
    InconclusiveReason,
    UnitaryRelation,
};

// -----------------------------------------------------------------------------
// Test helpers
// -----------------------------------------------------------------------------

/// Constructs a validated gate with no parameters, no classical destination,
/// and no measurement payload.
fn gate(
    kind: GateKind,
    qubits: &[usize],
) -> Gate {
    Gate::new(
        kind,
        qubits
            .iter()
            .copied()
            .map(QubitId::new)
            .collect(),
        Vec::new(),
        None,
        None,
    )
    .expect("test gate construction must satisfy canonical IR invariants")
}

/// Constructs a validated one-parameter gate.
fn parameterized_gate(
    kind: GateKind,
    qubits: &[usize],
    value: f64,
) -> Gate {
    let parameter = Parameter::constant(value)
        .expect("test parameter must be finite");

    Gate::new(
        kind,
        qubits
            .iter()
            .copied()
            .map(QubitId::new)
            .collect(),
        vec![parameter],
        None,
        None,
    )
    .expect("test parameterized gate must satisfy canonical IR invariants")
}

/// Constructs a validated two-parameter gate.
fn parameterized_gate_2(
    kind: GateKind,
    qubits: &[usize],
    first: f64,
    second: f64,
) -> Gate {
    let first_parameter = Parameter::constant(first)
        .expect("first test parameter must be finite");

    let second_parameter = Parameter::constant(second)
        .expect("second test parameter must be finite");

    Gate::new(
        kind,
        qubits
            .iter()
            .copied()
            .map(QubitId::new)
            .collect(),
        vec![first_parameter, second_parameter],
        None,
        None,
    )
    .expect("two-parameter test gate must satisfy canonical IR invariants")
}

/// Constructs a validated three-parameter gate.
fn parameterized_gate_3(
    kind: GateKind,
    qubits: &[usize],
    first: f64,
    second: f64,
    third: f64,
) -> Gate {
    let first_parameter = Parameter::constant(first)
        .expect("first test parameter must be finite");

    let second_parameter = Parameter::constant(second)
        .expect("second test parameter must be finite");

    let third_parameter = Parameter::constant(third)
        .expect("third test parameter must be finite");

    Gate::new(
        kind,
        qubits
            .iter()
            .copied()
            .map(QubitId::new)
            .collect(),
        vec![
            first_parameter,
            second_parameter,
            third_parameter,
        ],
        None,
        None,
    )
    .expect("three-parameter test gate must satisfy canonical IR invariants")
}

/// Creates a circuit from an owned operation list.
fn circuit(
    num_qubits: usize,
    operations: Vec<Gate>,
) -> QuantumCircuit {
    QuantumCircuit::from_operations(
        num_qubits,
        0,
        operations,
    )
    .expect("test circuit must satisfy canonical IR invariants")
}

/// Creates a circuit with an explicit classical namespace.
fn circuit_with_classical_bits(
    num_qubits: usize,
    num_classical_bits: usize,
    operations: Vec<Gate>,
) -> QuantumCircuit {
    QuantumCircuit::from_operations(
        num_qubits,
        num_classical_bits,
        operations,
    )
    .expect("test circuit must satisfy canonical IR invariants")
}

/// Asserts that a verification report proves equivalence.
fn assert_equivalent(
    report: &crate::quantum::optimization::equivalence::EquivalenceReport,
) {
    assert_eq!(
        report.verdict,
        EquivalenceVerdict::Equivalent,
        "expected proven equivalence, got {report:?}"
    );

    assert!(
        report.is_equivalent(),
        "equivalence report must expose is_equivalent() == true"
    );

    assert!(
        !report.is_not_equivalent(),
        "equivalent report must not report non-equivalence"
    );

    assert!(
        !report.is_inconclusive(),
        "equivalent report must not report inconclusive"
    );
}

/// Asserts that a verification report proves non-equivalence.
fn assert_not_equivalent(
    report: &crate::quantum::optimization::equivalence::EquivalenceReport,
) {
    assert_eq!(
        report.verdict,
        EquivalenceVerdict::NotEquivalent,
        "expected proven non-equivalence, got {report:?}"
    );

    assert!(
        report.is_not_equivalent(),
        "non-equivalent report must expose is_not_equivalent() == true"
    );

    assert!(
        !report.is_equivalent(),
        "non-equivalent report must not report equivalence"
    );

    assert!(
        !report.is_inconclusive(),
        "non-equivalent report must not report inconclusive"
    );
}

/// Asserts that a verification report is explicitly inconclusive.
fn assert_inconclusive(
    report: &crate::quantum::optimization::equivalence::EquivalenceReport,
) {
    assert_eq!(
        report.verdict,
        EquivalenceVerdict::Inconclusive,
        "expected inconclusive verification, got {report:?}"
    );

    assert!(
        report.is_inconclusive(),
        "inconclusive report must expose is_inconclusive() == true"
    );

    assert!(
        !report.is_equivalent(),
        "inconclusive verification must never be equivalent"
    );

    assert!(
        !report.is_not_equivalent(),
        "inconclusive verification must not claim non-equivalence"
    );
}

// ============================================================================
// Structural equivalence
// ============================================================================

#[test]
fn identical_empty_circuits_are_structurally_equivalent() {
    let left = circuit(0, Vec::new());
    let right = circuit(0, Vec::new());

    let report = verify_structural(&left, &right)
        .expect("structural verification must execute");

    assert_equivalent(&report);
    assert!(report.structurally_equal);
    assert_eq!(report.qubits, 0);
    assert_eq!(report.left_operations, 0);
    assert_eq!(report.right_operations, 0);
    assert_eq!(report.max_error, 0.0);
    assert!(report.global_phase.is_none());
}

#[test]
fn identical_nonempty_circuits_are_structurally_equivalent() {
    let operations = vec![
        gate(GateKind::H, &[0]),
        gate(GateKind::CX, &[0, 1]),
        gate(GateKind::T, &[1]),
    ];

    let left = circuit(2, operations.clone());
    let right = circuit(2, operations);

    let report = verify_structural(&left, &right)
        .expect("structural verification must execute");

    assert_equivalent(&report);
    assert!(report.structurally_equal);
    assert_eq!(report.left_operations, 3);
    assert_eq!(report.right_operations, 3);
}

#[test]
fn structural_verification_rejects_different_gate_order() {
    let left = circuit(
        1,
        vec![
            gate(GateKind::H, &[0]),
            gate(GateKind::X, &[0]),
        ],
    );

    let right = circuit(
        1,
        vec![
            gate(GateKind::X, &[0]),
            gate(GateKind::H, &[0]),
        ],
    );

    let report = verify_structural(&left, &right)
        .expect("structural verification must execute");

    assert_not_equivalent(&report);

    assert_eq!(
        report.difference,
        Some(
            crate::quantum::optimization::equivalence::Difference::StructuralOperation {
                index: 0,
            }
        )
    );
}

#[test]
fn structural_verification_rejects_different_operation_lengths() {
    let left = circuit(
        1,
        vec![gate(GateKind::H, &[0])],
    );

    let right = circuit(
        1,
        vec![
            gate(GateKind::H, &[0]),
            gate(GateKind::I, &[0]),
        ],
    );

    let report = verify_structural(&left, &right)
        .expect("structural verification must execute");

    assert_not_equivalent(&report);

    assert!(matches!(
        report.difference,
        Some(
            crate::quantum::optimization::equivalence::Difference::StructuralOperation {
                index: 1,
            }
        )
    ));
}

#[test]
fn structural_verification_rejects_different_qubit_namespaces() {
    let left = circuit(1, vec![gate(GateKind::H, &[0])]);
    let right = circuit(2, vec![gate(GateKind::H, &[0])]);

    let report = verify_structural(&left, &right)
        .expect("structural verification must execute");

    assert_not_equivalent(&report);

    assert_eq!(
        report.difference,
        Some(
            crate::quantum::optimization::equivalence::Difference::QubitCount {
                left: 1,
                right: 2,
            }
        )
    );
}

#[test]
fn structural_verification_rejects_different_classical_namespaces() {
    let left = circuit_with_classical_bits(
        1,
        0,
        vec![gate(GateKind::H, &[0])],
    );

    let right = circuit_with_classical_bits(
        1,
        1,
        vec![gate(GateKind::H, &[0])],
    );

    let report = verify_structural(&left, &right)
        .expect("structural verification must execute");

    assert_not_equivalent(&report);

    assert_eq!(
        report.difference,
        Some(
            crate::quantum::optimization::equivalence::Difference::ClassicalBitCount {
                left: 0,
                right: 1,
            }
        )
    );
}

// ============================================================================
// Canonical semantic equivalence
// ============================================================================

#[test]
fn identity_gate_is_semantically_equivalent_to_empty_circuit() {
    let left = circuit(1, vec![gate(GateKind::I, &[0])]);
    let right = circuit(1, Vec::new());

    let report = verify_unitary(&left, &right)
        .expect("unitary verification must execute");

    assert_equivalent(&report);
    assert!(!report.structurally_equal);
}

#[test]
fn double_self_inverse_single_qubit_gates_are_identity() {
    let gates = [
        GateKind::X,
        GateKind::Y,
        GateKind::Z,
        GateKind::H,
        GateKind::CX,
    ];

    for kind in gates {
        let (qubits, count) = if kind == GateKind::CX {
            (vec![0usize, 1usize], 2usize)
        } else {
            (vec![0usize], 1usize)
        };

        let left = circuit(
            count,
            vec![
                gate(kind, &qubits),
                gate(kind, &qubits),
            ],
        );

        let right = circuit(count, Vec::new());

        let report = verify_unitary(&left, &right)
            .expect("self-inverse verification must execute");

        assert_equivalent(&report);
    }
}

#[test]
fn double_controlled_self_inverse_gates_are_identity() {
    let cases = [
        (GateKind::CX, vec![0usize, 1usize]),
        (GateKind::CY, vec![0usize, 1usize]),
        (GateKind::CZ, vec![0usize, 1usize]),
        (GateKind::CH, vec![0usize, 1usize]),
        (GateKind::SWAP, vec![0usize, 1usize]),
        (GateKind::CCX, vec![0usize, 1usize, 2usize]),
        (GateKind::CSWAP, vec![0usize, 1usize, 2usize]),
    ];

    for (kind, qubits) in cases {
        let num_qubits = qubits
            .iter()
            .copied()
            .max()
            .map(|value| value + 1)
            .unwrap_or(0);

        let left = circuit(
            num_qubits,
            vec![
                gate(kind, &qubits),
                gate(kind, &qubits),
            ],
        );

        let right = circuit(num_qubits, Vec::new());

        let report = verify_unitary(&left, &right)
            .expect("controlled self-inverse verification must execute");

        assert_equivalent(&report);
    }
}

#[test]
fn t_followed_by_tdg_is_identity() {
    let left = circuit(
        1,
        vec![
            gate(GateKind::T, &[0]),
            gate(GateKind::Tdg, &[0]),
        ],
    );

    let right = circuit(1, Vec::new());

    let report = verify_unitary(&left, &right)
        .expect("T/Tdg equivalence must execute");

    assert_equivalent(&report);
}

#[test]
fn s_followed_by_sdg_is_identity() {
    let left = circuit(
        1,
        vec![
            gate(GateKind::S, &[0]),
            gate(GateKind::Sdg, &[0]),
        ],
    );

    let right = circuit(1, Vec::new());

    let report = verify_unitary(&left, &right)
        .expect("S/Sdg equivalence must execute");

    assert_equivalent(&report);
}

#[test]
fn rotation_composition_is_semantically_equivalent() {
    let first = 0.37;
    let second = -0.91;

    let cases = [
        GateKind::RX,
        GateKind::RY,
        GateKind::RZ,
        GateKind::Phase,
        GateKind::U1,
    ];

    for kind in cases {
        let left = circuit(
            1,
            vec![
                parameterized_gate(kind, &[0], first),
                parameterized_gate(kind, &[0], second),
            ],
        );

        let right = circuit(
            1,
            vec![
                parameterized_gate(
                    kind,
                    &[0],
                    first + second,
                ),
            ],
        );

        let report = verify_unitary(&left, &right)
            .expect("rotation composition verification must execute");

        assert_equivalent(&report);
    }
}

#[test]
fn inverse_rotation_is_identity() {
    let theta = 1.23456789;

    for kind in [
        GateKind::RX,
        GateKind::RY,
        GateKind::RZ,
        GateKind::Phase,
        GateKind::U1,
    ] {
        let left = circuit(
            1,
            vec![
                parameterized_gate(kind, &[0], theta),
                parameterized_gate(kind, &[0], -theta),
            ],
        );

        let right = circuit(1, Vec::new());

        let report = verify_unitary(&left, &right)
            .expect("inverse rotation verification must execute");

        assert_equivalent(&report);
    }
}

#[test]
fn parameterized_controlled_rotation_composition_is_equivalent() {
    let first = 0.25;
    let second = -0.75;

    for kind in [
        GateKind::CRX,
        GateKind::CRY,
        GateKind::CRZ,
    ] {
        let left = circuit(
            2,
            vec![
                parameterized_gate(
                    kind,
                    &[0, 1],
                    first,
                ),
                parameterized_gate(
                    kind,
                    &[0, 1],
                    second,
                ),
            ],
        );

        let right = circuit(
            2,
            vec![
                parameterized_gate(
                    kind,
                    &[0, 1],
                    first + second,
                ),
            ],
        );

        let report = verify_unitary(&left, &right)
            .expect(
                "controlled rotation composition verification must execute",
            );

        assert_equivalent(&report);
    }
}

#[test]
fn u1_and_phase_have_identical_semantics() {
    let theta = 0.8125;

    let left = circuit(
        1,
        vec![parameterized_gate(
            GateKind::U1,
            &[0],
            theta,
        )],
    );

    let right = circuit(
        1,
        vec![parameterized_gate(
            GateKind::Phase,
            &[0],
            theta,
        )],
    );

    let report = verify_unitary(&left, &right)
        .expect("U1/Phase equivalence must execute");

    assert_equivalent(&report);
}

#[test]
fn u2_semantics_are_deterministic() {
    let phi = 0.17;
    let lambda = -0.93;

    let left = circuit(
        1,
        vec![parameterized_gate_2(
            GateKind::U2,
            &[0],
            phi,
            lambda,
        )],
    );

    let right = circuit(
        1,
        vec![parameterized_gate_2(
            GateKind::U2,
            &[0],
            phi,
            lambda,
        )],
    );

    let report = verify_unitary(&left, &right)
        .expect("U2 verification must execute");

    assert_equivalent(&report);
    assert!(report.max_error <= 1.0e-10);
}

#[test]
fn u3_semantics_are_deterministic() {
    let theta = 0.37;
    let phi = -0.22;
    let lambda = 0.91;

    let left = circuit(
        1,
        vec![parameterized_gate_3(
            GateKind::U3,
            &[0],
            theta,
            phi,
            lambda,
        )],
    );

    let right = circuit(
        1,
        vec![parameterized_gate_3(
            GateKind::U3,
            &[0],
            theta,
            phi,
            lambda,
        )],
    );

    let report = verify_unitary(&left, &right)
        .expect("U3 verification must execute");

    assert_equivalent(&report);
}

// ============================================================================
// Multi-qubit semantic coverage
// ============================================================================

#[test]
fn cx_squared_is_identity_for_multiple_qubit_positions() {
    for (control, target) in [
        (0usize, 1usize),
        (1usize, 0usize),
        (0usize, 2usize),
        (2usize, 0usize),
        (1usize, 2usize),
        (2usize, 1usize),
    ] {
        let left = circuit(
            3,
            vec![
                gate(GateKind::CX, &[control, target]),
                gate(GateKind::CX, &[control, target]),
            ],
        );

        let right = circuit(3, Vec::new());

        let report = verify_unitary(&left, &right)
            .expect("CX self-inverse verification must execute");

        assert_equivalent(&report);
    }
}

#[test]
fn swap_squared_is_identity() {
    let left = circuit(
        3,
        vec![
            gate(GateKind::SWAP, &[0, 2]),
            gate(GateKind::SWAP, &[0, 2]),
        ],
    );

    let right = circuit(3, Vec::new());

    let report = verify_unitary(&left, &right)
        .expect("SWAP equivalence must execute");

    assert_equivalent(&report);
}

#[test]
fn toffoli_squared_is_identity() {
    let left = circuit(
        3,
        vec![
            gate(GateKind::CCX, &[0, 1, 2]),
            gate(GateKind::CCX, &[0, 1, 2]),
        ],
    );

    let right = circuit(3, Vec::new());

    let report = verify_unitary(&left, &right)
        .expect("Toffoli equivalence must execute");

    assert_equivalent(&report);
}

#[test]
fn controlled_swap_squared_is_identity() {
    let left = circuit(
        3,
        vec![
            gate(GateKind::CSWAP, &[0, 1, 2]),
            gate(GateKind::CSWAP, &[0, 1, 2]),
        ],
    );

    let right = circuit(3, Vec::new());

    let report = verify_unitary(&left, &right)
        .expect("controlled-SWAP equivalence must execute");

    assert_equivalent(&report);
}

// ============================================================================
// Non-equivalence tests
// ============================================================================

#[test]
fn x_is_not_equivalent_to_identity() {
    let left = circuit(
        1,
        vec![gate(GateKind::X, &[0])],
    );

    let right = circuit(1, Vec::new());

    let report = verify_unitary(&left, &right)
        .expect("verification must execute");

    assert_not_equivalent(&report);

    assert!(matches!(
        report.difference,
        Some(
            crate::quantum::optimization::equivalence::Difference::UnitaryAction {
                ..
            }
        )
    ));
}

#[test]
fn h_is_not_equivalent_to_x() {
    let left = circuit(
        1,
        vec![gate(GateKind::H, &[0])],
    );

    let right = circuit(
        1,
        vec![gate(GateKind::X, &[0])],
    );

    let report = verify_unitary(&left, &right)
        .expect("verification must execute");

    assert_not_equivalent(&report);
}

#[test]
fn different_rotation_angles_are_not_equivalent() {
    let left = circuit(
        1,
        vec![parameterized_gate(
            GateKind::RZ,
            &[0],
            0.25,
        )],
    );

    let right = circuit(
        1,
        vec![parameterized_gate(
            GateKind::RZ,
            &[0],
            0.75,
        )],
    );

    let report = verify_unitary(&left, &right)
        .expect("verification must execute");

    assert_not_equivalent(&report);
}

#[test]
fn different_control_target_order_is_not_equivalent() {
    let left = circuit(
        2,
        vec![gate(GateKind::CX, &[0, 1])],
    );

    let right = circuit(
        2,
        vec![gate(GateKind::CX, &[1, 0])],
    );

    let report = verify_unitary(&left, &right)
        .expect("verification must execute");

    assert_not_equivalent(&report);
}

// ============================================================================
// Symbolic parameter safety
// ============================================================================

#[test]
fn symbolic_parameters_never_silently_become_equivalent() {
    let symbol = Parameter::symbol("theta")
        .expect("test symbol must be valid");

    let symbolic_gate = Gate::new(
        GateKind::RX,
        vec![QubitId::new(0)],
        vec![symbol],
        None,
        None,
    )
    .expect("symbolic gate must satisfy canonical IR invariants");

    let concrete_gate = parameterized_gate(
        GateKind::RX,
        &[0],
        0.5,
    );

    let left = circuit(1, vec![symbolic_gate]);
    let right = circuit(1, vec![concrete_gate]);

    let report = verify(
        &left,
        &right,
        EquivalenceConfig::default(),
    )
    .expect("symbolic verification request must execute");

    assert_inconclusive(&report);

    assert_eq!(
        report.inconclusive_reason,
        Some(InconclusiveReason::SymbolicParameters)
    );
}

#[test]
fn two_symbolic_circuits_are_not_claimed_equivalent_by_dense_verification() {
    let first = Parameter::symbol("theta")
        .expect("first symbol must be valid");

    let second = Parameter::symbol("phi")
        .expect("second symbol must be valid");

    let left_gate = Gate::new(
        GateKind::RX,
        vec![QubitId::new(0)],
        vec![first],
        None,
        None,
    )
    .expect("left symbolic gate must be valid");

    let right_gate = Gate::new(
        GateKind::RX,
        vec![QubitId::new(0)],
        vec![second],
        None,
        None,
    )
    .expect("right symbolic gate must be valid");

    let left = circuit(1, vec![left_gate]);
    let right = circuit(1, vec![right_gate]);

    let report = verify_unitary(&left, &right)
        .expect("symbolic verification must execute");

    assert_inconclusive(&report);

    assert_eq!(
        report.inconclusive_reason,
        Some(InconclusiveReason::SymbolicParameters)
    );
}

// ============================================================================
// Non-unitary safety
// ============================================================================

#[test]
fn barrier_circuit_is_inconclusive_for_unitary_verification() {
    let barrier = gate(GateKind::Barrier, &[0]);

    let left = circuit(1, vec![barrier]);
    let right = circuit(1, Vec::new());

    let report = verify_unitary(&left, &right)
        .expect("unitary verification must return a report");

    assert_inconclusive(&report);

    assert_eq!(
        report.inconclusive_reason,
        Some(InconclusiveReason::NonUnitaryCircuit)
    );
}

#[test]
fn non_unitary_operations_are_not_treated_as_identity() {
    let barrier = gate(GateKind::Barrier, &[0]);

    let left = circuit(1, vec![barrier]);
    let right = circuit(1, Vec::new());

    let report = verify(
        &left,
        &right,
        EquivalenceConfig {
            method: EquivalenceMethod::Auto {
                relation: UnitaryRelation::UpToGlobalPhase,
            },
            ..EquivalenceConfig::default()
        },
    )
    .expect("automatic verification must execute");

    assert_inconclusive(&report);

    assert_ne!(
        report.verdict,
        EquivalenceVerdict::Equivalent,
        "unsupported non-unitary semantics must never become equivalence"
    );
}

// ============================================================================
// Unsupported gate safety
// ============================================================================

#[test]
fn ecr_is_not_silently_given_dense_semantics() {
    let left = circuit(
        2,
        vec![gate(GateKind::ECR, &[0, 1])],
    );

    let right = circuit(
        2,
        vec![gate(GateKind::I, &[0]), gate(GateKind::I, &[1])],
    );

    let report = verify_unitary(&left, &right)
        .expect("unsupported ECR verification must return a report");

    assert_inconclusive(&report);

    assert_eq!(
        report.inconclusive_reason,
        Some(InconclusiveReason::UnsupportedGate(
            GateKind::ECR
        ))
    );
}

// ============================================================================
// Resource-limit safety
// ============================================================================

#[test]
fn qubit_limit_produces_inconclusive_instead_of_unbounded_allocation() {
    let left = circuit(
        5,
        vec![gate(GateKind::H, &[0])],
    );

    let right = circuit(
        5,
        vec![gate(GateKind::H, &[0])],
    );

    // The circuits are intentionally structurally different only by metadata
    // is not possible here; use a semantically equivalent but structurally
    // different representation to force semantic verification.
    let left = circuit(
        5,
        vec![
            gate(GateKind::H, &[0]),
            gate(GateKind::H, &[0]),
        ],
    );

    let right = circuit(
        5,
        Vec::new(),
    );

    let limits = EquivalenceLimits::new(
        4,
        16,
        16,
        100,
        None,
    )
    .expect("test limits must be valid");

    let report = verify(
        &left,
        &right,
        EquivalenceConfig {
            method: EquivalenceMethod::ExactUnitary {
                relation: UnitaryRelation::UpToGlobalPhase,
            },
            tolerance:
                crate::quantum::optimization::equivalence::EquivalenceTolerance::numerical(),
            limits,
        },
    )
    .expect("bounded verification must return a report");

    assert_inconclusive(&report);

    assert_eq!(
        report.inconclusive_reason,
        Some(InconclusiveReason::QubitLimitExceeded {
            actual: 5,
            maximum: 4,
        })
    );
}

#[test]
fn amplitude_limit_produces_inconclusive_before_dense_allocation() {
    let left = circuit(
        4,
        vec![gate(GateKind::H, &[0])],
    );

    let right = circuit(
        4,
        vec![
            gate(GateKind::H, &[0]),
            gate(GateKind::H, &[0]),
        ],
    );

    let limits = EquivalenceLimits::new(
        8,
        8,
        16,
        100,
        None,
    )
    .expect("test limits must be valid");

    let report = verify(
        &left,
        &right,
        EquivalenceConfig {
            method: EquivalenceMethod::ExactUnitary {
                relation: UnitaryRelation::UpToGlobalPhase,
            },
            tolerance:
                crate::quantum::optimization::equivalence::EquivalenceTolerance::numerical(),
            limits,
        },
    )
    .expect("bounded verification must return a report");

    assert_inconclusive(&report);

    assert_eq!(
        report.inconclusive_reason,
        Some(InconclusiveReason::AmplitudeLimitExceeded {
            required: 16,
            maximum: 8,
        })
    );
}

#[test]
fn basis_state_limit_produces_inconclusive() {
    let left = circuit(
        3,
        vec![
            gate(GateKind::H, &[0]),
            gate(GateKind::H, &[1]),
        ],
    );

    let right = circuit(
        3,
        Vec::new(),
    );

    let limits = EquivalenceLimits::new(
        8,
        16,
        4,
        100,
        None,
    )
    .expect("test limits must be valid");

    let report = verify(
        &left,
        &right,
        EquivalenceConfig {
            method: EquivalenceMethod::ExactUnitary {
                relation: UnitaryRelation::UpToGlobalPhase,
            },
            tolerance:
                crate::quantum::optimization::equivalence::EquivalenceTolerance::numerical(),
            limits,
        },
    )
    .expect("bounded verification must return a report");

    assert_inconclusive(&report);

    assert_eq!(
        report.inconclusive_reason,
        Some(InconclusiveReason::BasisStateLimitExceeded {
            required: 8,
            maximum: 4,
        })
    );
}

#[test]
fn operation_limit_produces_inconclusive() {
    let left = circuit(
        1,
        vec![
            gate(GateKind::X, &[0]),
            gate(GateKind::X, &[0]),
            gate(GateKind::X, &[0]),
        ],
    );

    let right = circuit(1, Vec::new());

    let limits = EquivalenceLimits::new(
        8,
        256,
        256,
        2,
        None,
    )
    .expect("test limits must be valid");

    let report = verify(
        &left,
        &right,
        EquivalenceConfig {
            method: EquivalenceMethod::ExactUnitary {
                relation: UnitaryRelation::UpToGlobalPhase,
            },
            tolerance:
                crate::quantum::optimization::equivalence::EquivalenceTolerance::numerical(),
            limits,
        },
    )
    .expect("bounded verification must return a report");

    assert_inconclusive(&report);

    assert_eq!(
        report.inconclusive_reason,
        Some(InconclusiveReason::OperationLimitExceeded {
            actual: 3,
            maximum: 2,
        })
    );
}

#[test]
fn zero_duration_limit_is_rejected_at_configuration_boundary() {
    let result = EquivalenceLimits::new(
        8,
        256,
        256,
        100,
        Some(Duration::ZERO),
    );

    assert!(
        result.is_err(),
        "zero-duration verifier policy must be rejected"
    );
}

// ============================================================================
// Numerical tolerance
// ============================================================================

#[test]
fn numerical_tolerance_allows_small_floating_point_error() {
    let left = circuit(
        1,
        vec![parameterized_gate(
            GateKind::RZ,
            &[0],
            PI,
        )],
    );

    let right = circuit(
        1,
        vec![parameterized_gate(
            GateKind::RZ,
            &[0],
            PI + 1.0e-13,
        )],
    );

    let report = verify_unitary(&left, &right)
        .expect("numerical verification must execute");

    assert_equivalent(&report);
}

#[test]
fn exact_tolerance_rejects_a_real_semantic_difference() {
    let left = circuit(
        1,
        vec![parameterized_gate(
            GateKind::RZ,
            &[0],
            0.0,
        )],
    );

    let right = circuit(
        1,
        vec![parameterized_gate(
            GateKind::RZ,
            &[0],
            1.0e-4,
        )],
    );

    let config = EquivalenceConfig {
        method: EquivalenceMethod::ExactUnitary {
            relation: UnitaryRelation::Exact,
        },
        tolerance:
            crate::quantum::optimization::equivalence::EquivalenceTolerance::exact(),
        limits: EquivalenceLimits::conservative(),
    };

    let report = verify(
        &left,
        &right,
        config,
    )
    .expect("exact verification must execute");

    assert_not_equivalent(&report);
}

// ============================================================================
// Global-phase contract
// ============================================================================

#[test]
fn global_phase_relation_is_explicit_in_configuration() {
    let exact = EquivalenceConfig {
        method: EquivalenceMethod::ExactUnitary {
            relation: UnitaryRelation::Exact,
        },
        ..EquivalenceConfig::default()
    };

    let phase_insensitive = EquivalenceConfig {
        method: EquivalenceMethod::ExactUnitary {
            relation: UnitaryRelation::UpToGlobalPhase,
        },
        ..EquivalenceConfig::default()
    };

    assert_ne!(
        exact.method,
        phase_insensitive.method,
        "global-phase policy must be explicit rather than implicit"
    );
}

// ============================================================================
// Fingerprint correctness
// ============================================================================

#[test]
fn identical_circuits_have_identical_structural_fingerprints() {
    let left = circuit(
        2,
        vec![
            gate(GateKind::H, &[0]),
            gate(GateKind::CX, &[0, 1]),
        ],
    );

    let right = circuit(
        2,
        vec![
            gate(GateKind::H, &[0]),
            gate(GateKind::CX, &[0, 1]),
        ],
    );

    assert_eq!(
        structural_fingerprint(&left),
        structural_fingerprint(&right)
    );
}

#[test]
fn structurally_different_but_semantically_equivalent_circuits_can_have_different_fingerprints() {
    let left = circuit(
        1,
        vec![
            gate(GateKind::X, &[0]),
            gate(GateKind::X, &[0]),
        ],
    );

    let right = circuit(
        1,
        Vec::new(),
    );

    assert_ne!(
        structural_fingerprint(&left),
        structural_fingerprint(&right),
        "structural fingerprints must not masquerade as semantic equivalence"
    );

    let report = verify_unitary(&left, &right)
        .expect("semantic verification must execute");

    assert_equivalent(&report);
    assert!(!report.structurally_equal);
}

// ============================================================================
// Automatic method-selection contract
// ============================================================================

#[test]
fn_auto_mode_proves_supported_small_unitary_circuits() {
    let left = circuit(
        2,
        vec![
            gate(GateKind::H, &[0]),
            gate(GateKind::CX, &[0, 1]),
            gate(GateKind::CX, &[0, 1]),
        ],
    );

    let right = circuit(
        2,
        vec![gate(GateKind::H, &[0])],
    );

    let report = verify(
        &left,
        &right,
        EquivalenceConfig::default(),
    )
    .expect("automatic verification must execute");

    assert_equivalent(&report);

    assert!(matches!(
        report.method,
        EquivalenceMethod::ExactUnitary {
            relation: UnitaryRelation::UpToGlobalPhase
        }
        | EquivalenceMethod::Structural
    ));
}

#[test]
fn auto_mode_does_not_convert_unsupported_semantics_into_equivalence() {
    let left = circuit(
        2,
        vec![gate(GateKind::ECR, &[0, 1])],
    );

    let right = circuit(
        2,
        vec![gate(GateKind::I, &[0]), gate(GateKind::I, &[1])],
    );

    let report = verify(
        &left,
        &right,
        EquivalenceConfig::default(),
    )
    .expect("automatic verification must execute");

    assert_inconclusive(&report);
}

// ============================================================================
// Cross-method consistency
// ============================================================================

#[test]
fn_structural_and_semantic_verifiers_agree_on_identical_circuits() {
    let circuit = circuit(
        3,
        vec![
            gate(GateKind::H, &[0]),
            gate(GateKind::CX, &[0, 1]),
            gate(GateKind::T, &[2]),
        ],
    );

    let structural = verify_structural(
        &circuit,
        &circuit,
    )
    .expect("structural verification must execute");

    let semantic = verify_unitary(
        &circuit,
        &circuit,
    )
    .expect("semantic verification must execute");

    assert_equivalent(&structural);
    assert_equivalent(&semantic);

    assert!(structural.structurally_equal);
    assert!(semantic.structurally_equal);
}

#[test]
fn structural_and_semantic_verifiers_can_distinguish_identity_transforms() {
    let left = circuit(
        1,
        vec![
            gate(GateKind::H, &[0]),
            gate(GateKind::H, &[0]),
        ],
    );

    let right = circuit(1, Vec::new());

    let structural = verify_structural(
        &left,
        &right,
    )
    .expect("structural verification must execute");

    let semantic = verify_unitary(
        &left,
        &right,
    )
    .expect("semantic verification must execute");

    assert_not_equivalent(&structural);
    assert_equivalent(&semantic);

    assert!(!structural.structurally_equal);
    assert!(!semantic.structurally_equal);
}

// ============================================================================
// Deterministic repeated verification
// ============================================================================

#[test]
fn repeated_verification_is_deterministic() {
    let left = circuit(
        2,
        vec![
            parameterized_gate(
                GateKind::RX,
                &[0],
                0.31,
            ),
            gate(GateKind::CX, &[0, 1]),
            parameterized_gate(
                GateKind::RZ,
                &[1],
                -0.72,
            ),
        ],
    );

    let right = circuit(
        2,
        vec![
            parameterized_gate(
                GateKind::RX,
                &[0],
                0.31,
            ),
            gate(GateKind::CX, &[0, 1]),
            parameterized_gate(
                GateKind::RZ,
                &[1],
                -0.72,
            ),
        ],
    );

    let first = verify_unitary(
        &left,
        &right,
    )
    .expect("first verification must execute");

    let second = verify_unitary(
        &left,
        &right,
    )
    .expect("second verification must execute");

    assert_eq!(first.verdict, second.verdict);
    assert_eq!(first.method, second.method);
    assert_eq!(
        first.structurally_equal,
        second.structurally_equal
    );
    assert_eq!(
        first.left_fingerprint,
        second.left_fingerprint
    );
    assert_eq!(
        first.right_fingerprint,
        second.right_fingerprint
    );
    assert_eq!(
        first.difference,
        second.difference
    );
    assert_eq!(
        first.inconclusive_reason,
        second.inconclusive_reason
    );
}

// ============================================================================
// Large-operation scalability contract
// ============================================================================

#[test]
fn large_operation_sequence_is_handled_without_architectural_size_assumptions() {
    const OPERATION_COUNT: usize = 10_000;

    let mut operations = Vec::with_capacity(OPERATION_COUNT);

    for _ in 0..OPERATION_COUNT {
        operations.push(gate(GateKind::I, &[0]));
    }

    let left = circuit(
        1,
        operations,
    );

    let right = circuit(
        1,
        Vec::new(),
    );

    let report = verify_unitary(
        &left,
        &right,
    )
    .expect("large operation verification must execute");

    assert_equivalent(&report);

    assert_eq!(
        report.left_operations,
        OPERATION_COUNT
    );

    assert_eq!(
        report.right_operations,
        0
    );
}

#[test]
fn repeated_identity_blocks_remain_semantically_equivalent_at_scale() {
    const BLOCK_COUNT: usize = 2_000;

    let mut left_operations =
        Vec::with_capacity(BLOCK_COUNT * 2);

    for _ in 0..BLOCK_COUNT {
        left_operations.push(gate(GateKind::H, &[0]));
        left_operations.push(gate(GateKind::H, &[0]));
    }

    let left = circuit(
        1,
        left_operations,
    );

    let right = circuit(
        1,
        Vec::new(),
    );

    let report = verify_unitary(
        &left,
        &right,
    )
    .expect("scaled semantic verification must execute");

    assert_equivalent(&report);
}

// ============================================================================
// Logical-qubit namespace integration
// ============================================================================

#[test]
fn logical_qubit_ids_are_canonical_ir_qubit_ids() {
    let first = QubitId::new(0);
    let second = QubitId::new(17);

    assert_eq!(first.index(), 0);
    assert_eq!(second.index(), 17);

    assert_ne!(first, second);
}

#[test]
fn noncontiguous_logical_qubit_operands_are_supported_when_in_namespace() {
    let left = circuit(
        4,
        vec![
            gate(GateKind::CX, &[0, 3]),
            gate(GateKind::CX, &[0, 3]),
        ],
    );

    let right = circuit(4, Vec::new());

    let report = verify_unitary(
        &left,
        &right,
    )
    .expect(
        "noncontiguous logical-qubit semantic verification must execute",
    );

    assert_equivalent(&report);
}

// ============================================================================
// Configuration validation
// ============================================================================

#[test]
fn negative_tolerance_is_rejected() {
    let result =
        crate::quantum::optimization::equivalence::EquivalenceTolerance::new(
            -1.0,
            0.0,
        );

    assert!(
        result.is_err(),
        "negative absolute tolerance must be rejected"
    );
}

#[test]
fn nonfinite_tolerance_is_rejected() {
    let result =
        crate::quantum::optimization::equivalence::EquivalenceTolerance::new(
            f64::NAN,
            0.0,
        );

    assert!(
        result.is_err(),
        "NaN tolerance must be rejected"
    );
}

#[test]
fn negative_relative_tolerance_is_rejected() {
    let result =
        crate::quantum::optimization::equivalence::EquivalenceTolerance::new(
            0.0,
            -1.0,
        );

    assert!(
        result.is_err(),
        "negative relative tolerance must be rejected"
    );
}

#[test]
fn zero_qubit_limit_is_rejected() {
    let result = EquivalenceLimits::new(
        0,
        256,
        256,
        100,
        None,
    );

    assert!(
        result.is_err(),
        "zero maximum qubits must be rejected"
    );
}

#[test]
fn zero_amplitude_limit_is_rejected() {
    let result = EquivalenceLimits::new(
        8,
        0,
        256,
        100,
        None,
    );

    assert!(
        result.is_err(),
        "zero amplitude limit must be rejected"
    );
}

#[test]
fn zero_operation_limit_is_rejected() {
    let result = EquivalenceLimits::new(
        8,
        256,
        256,
        0,
        None,
    );

    assert!(
        result.is_err(),
        "zero operation limit must be rejected"
    );
}

// ============================================================================
// Test invariants
// ============================================================================

#[test]
fn inconclusive_is_never_equivalent() {
    let symbolic = Parameter::symbol("theta")
        .expect("test symbol must be valid");

    let symbolic_gate = Gate::new(
        GateKind::RX,
        vec![QubitId::new(0)],
        vec![symbolic],
        None,
        None,
    )
    .expect("symbolic gate must be valid");

    let left = circuit(
        1,
        vec![symbolic_gate],
    );

    let right = circuit(
        1,
        vec![parameterized_gate(
            GateKind::RX,
            &[0],
            0.5,
        )],
    );

    let report = verify_unitary(
        &left,
        &right,
    )
    .expect("verification must execute");

    assert_inconclusive(&report);

    assert_ne!(
        report.verdict,
        EquivalenceVerdict::Equivalent
    );
}

#[test]
fn not_equivalent_is_never_inconclusive() {
    let left = circuit(
        1,
        vec![gate(GateKind::X, &[0])],
    );

    let right = circuit(
        1,
        Vec::new(),
    );

    let report = verify_unitary(
        &left,
        &right,
    )
    .expect("verification must execute");

    assert_not_equivalent(&report);

    assert_ne!(
        report.verdict,
        EquivalenceVerdict::Inconclusive
    );
}

#[test]
fn equivalent_is_never_not_equivalent() {
    let left = circuit(
        1,
        vec![
            gate(GateKind::H, &[0]),
            gate(GateKind::H, &[0]),
        ],
    );

    let right = circuit(
        1,
        Vec::new(),
    );

    let report = verify_unitary(
        &left,
        &right,
    )
    .expect("verification must execute");

    assert_equivalent(&report);

    assert_ne!(
        report.verdict,
        EquivalenceVerdict::NotEquivalent
    );
}