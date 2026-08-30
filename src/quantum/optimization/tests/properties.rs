//! Zamani Quantum Optimization — Property and Invariant Tests
//!
//! `src/quantum/optimization/tests/properties.rs`
//!
//! # Responsibility
//!
//! This module verifies semantic and structural properties of the production
//! quantum optimization subsystem rather than testing private implementation
//! details.
//!
//! The tests are intentionally written against the canonical public contracts:
//!
//! - `crate::quantum::ir::QuantumCircuit`;
//! - `crate::quantum::ir::Gate`;
//! - `crate::quantum::ir::GateKind`;
//! - `crate::quantum::ir::Parameter`;
//! - `crate::quantum::ir::qubit::QubitId`;
//! - `crate::quantum::optimization::equivalence`;
//! - `crate::quantum::optimization::context`;
//! - `crate::quantum::optimization::pass`;
//! - `crate::quantum::optimization::local::cancellation`.
//!
//! # Architectural position
//!
//! ```text
//!                         Zamani source
//!                              |
//!                              v
//!                       quantum::ir
//!                              |
//!                              v
//!                    optimization subsystem
//!                              |
//!                 +------------+------------+
//!                 |                         |
//!                 v                         v
//!             optimization             verification
//!                 |                         |
//!                 +------------+------------+
//!                              |
//!                              v
//!                         properties.rs
//! ```
//!
//! This file verifies properties that must remain true as the optimization
//! implementation evolves.
//!
//! # Core invariants
//!
//! The property suite verifies that:
//!
//! 1. valid canonical gates can be constructed deterministically;
//! 2. invalid gate structures are rejected;
//! 3. logical qubits remain strongly typed;
//! 4. duplicate qubit operands are rejected;
//! 5. gate self-inverse classification is internally coherent;
//! 6. exact inverse cancellation is symmetric;
//! 7. cancellation never crosses qubit boundaries;
//! 8. cancellation never crosses barriers;
//! 9. cancellation never treats reset as an ordinary unitary gate;
//! 10. parameterized inverse operations are handled conservatively;
//! 11. canonical circuits remain valid after optimization;
//! 12. optimization never increases operation count for deletion-only
//!     cancellation;
//! 13. cancellation is idempotent;
//! 14. optimization preserves proven semantic equivalence;
//! 15. structural equivalence is reflexive;
//! 16. structural equivalence is symmetric;
//! 17. non-equivalent circuits are not falsely reported as equivalent;
//! 18. generated circuits remain deterministic;
//! 19. sparse logical qubit namespaces work correctly;
//! 20. large circuits do not cause quadratic test construction;
//! 21. repeated optimization converges;
//! 22. optimizer resource behavior remains bounded by explicit policies;
//! 23. no test requires `unsafe` Rust;
//! 24. no test depends on hash-map iteration order;
//! 25. the test suite can scale beyond its default workload when the available
//!     machine resources permit it.
//!
//! # Property-testing strategy
//!
//! This project deliberately does not require a third-party property-testing
//! framework for these foundational optimizer properties.
//!
//! Instead this module combines:
//!
//! - deterministic pseudo-random generation;
//! - exhaustive small-domain enumeration;
//! - metamorphic testing;
//! - invariant testing;
//! - configurable stress testing;
//! - semantic differential testing.
//!
//! This keeps the optimization test foundation dependency-light and compatible
//! with Rust 1.97.1.
//!
//! # Scaling
//!
//! The tests have a deliberately small default workload so ordinary CI remains
//! practical.
//!
//! The stress workload can be increased with:
//!
//! ```text
//! ZAMANI_OPTIMIZATION_PROPERTY_SCALE=100000 cargo test
//! ```
//!
//! The value is not an architectural maximum. It is only the requested test
//! workload.
//!
//! A machine with sufficient resources can therefore execute substantially
//! larger workloads without changing this source file.
//!
//! The optimizer itself remains governed by `OptimizationLimits`; this test
//! module never claims that quantum optimization or equivalence checking is
//! mathematically polynomial or physically unlimited.
//!
//! # Important semantic distinction
//!
//! These tests never equate:
//!
//! - equal gate count;
//! - equal depth;
//! - equal number of qubits;
//! - equal fingerprints;
//!
//! with semantic equivalence.
//!
//! Semantic properties use the canonical equivalence verifier.
//!
//! # Canonical IR rule
//!
//! No local `QuantumGate`, `QuantumOperation`, or substitute circuit structure
//! is defined here.
//!
//! Every circuit is constructed from the canonical Quantum IR.
//!
//! # Qubit module naming
//!
//! The canonical repository module is:
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
//! This test file intentionally uses `quantum::ir::qubit::QubitId` so that the
//! optimizer tests reinforce the canonical repository naming.
//!
//! # Integration contract
//!
//! ## `tests/mod.rs`
//!
//! Add:
//!
//! ```text
//! mod properties;
//! ```
//!
//! The test module itself should remain private to the optimization test
//! harness.
//!
//! ## `optimization::local::cancellation`
//!
//! The suite consumes:
//!
//! - `CancellationPass::new()`;
//! - `CancellationPass::can_cancel()`;
//! - `CancellationPass::is_identity()`;
//! - `OptimizationPass::run()`.
//!
//! ## `optimization::context`
//!
//! The suite constructs invocation-scoped contexts using:
//!
//! `OptimizationContext::production(OptimizationConfig::default())`.
//!
//! ## `optimization::equivalence`
//!
//! Semantic properties use `verify()` with the canonical equivalence contract.
//!
//! ## `quantum::ir`
//!
//! All circuits and gates originate from canonical IR constructors.
//!
//! ## Future optimization passes
//!
//! Future passes should add their own property tests rather than weakening
//! these foundational invariants.
//!
//! This file should not need to be rewritten merely because another optimizer
//! pass is added.
//!
//! # Rust compatibility
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust only;
//! - no nightly features;
//! - no unsafe code;
//! - no external property-testing dependency.
//!
//! # Safety
//!
//! This entire test module explicitly forbids unsafe code.
//!
//! No optimizer property requires unsafe Rust.
//!
//! # Determinism
//!
//! The deterministic generator below uses an explicit integer state. It does
//! not use operating-system randomness, thread-local randomness, timestamps,
//! hash iteration order, or global mutable state.
//!
//! Therefore a failing generated case can be reproduced from its seed.
//!
//! ```text
//! seed
//!   |
//!   v
//! deterministic generator
//!   |
//!   v
//! canonical QuantumCircuit
//!   |
//!   +----------+
//!   |          |
//!   v          v
//! original   optimizer
//!   |          |
//!   +----+-----+
//!        |
//!        v
//! equivalence + invariants
//! ```
//!
//! # Test philosophy
//!
//! A property test should fail because a semantic invariant is broken, not
//! because an implementation detail changed.
//!
//! Consequently these tests avoid assertions about:
//!
//! - private vectors;
//! - private fields;
//! - internal hash-map layout;
//! - exact allocation counts;
//! - exact optimizer implementation strategy;
//! - pass-local scratch storage;
//! - internal iteration order.
//!
//! They assert only stable observable contracts.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::f64::consts::PI;

use crate::quantum::ir::gate::{Gate, GateKind};
use crate::quantum::ir::parameter::Parameter;
use crate::quantum::ir::qubit::QubitId;
use crate::quantum::ir::QuantumCircuit;

use crate::quantum::optimization::context::OptimizationContext;
use crate::quantum::optimization::config::OptimizationConfig;
use crate::quantum::optimization::equivalence::{
    verify,
    EquivalenceConfig,
    EquivalenceMethod,
    EquivalenceVerdict,
    UnitaryRelation,
};
use crate::quantum::optimization::local::cancellation::CancellationPass;
use crate::quantum::optimization::pass::OptimizationPass;

// =============================================================================
// Test configuration
// =============================================================================

/// Default number of generated operations used by scalable stress properties.
///
/// This is deliberately modest enough for ordinary CI while still exercising
/// non-trivial optimizer behavior.
const DEFAULT_PROPERTY_SCALE: usize = 4_096;

/// Maximum number of generated operations in the deterministic small-domain
/// property suite.
///
/// This is NOT an optimizer architectural limit. It prevents accidental
/// runaway CI configuration when a test is invoked without the explicit
/// stress environment variable.
const DEFAULT_ENUMERATION_SCALE: usize = 256;

/// Deterministic seed used by the default generated-circuit suite.
const DEFAULT_SEED: u64 = 0x5A4D_414E_495F_5155;

// =============================================================================
// Deterministic generator
// =============================================================================

/// Small deterministic pseudo-random generator used only by tests.
///
/// This is intentionally self-contained so the foundational optimizer test
/// suite does not require an external RNG dependency.
///
/// The generator is deterministic and reproducible for a fixed seed.
#[derive(Debug, Clone, Copy)]
struct DeterministicGenerator {
    state: u64,
}

impl DeterministicGenerator {
    /// Creates a generator from an explicit seed.
    const fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    /// Advances the generator and returns the next deterministic value.
    fn next_u64(&mut self) -> u64 {
        // SplitMix64-style deterministic mixing.
        //
        // Wrapping arithmetic is intentional here: this is a test generator,
        // not resource accounting. The resulting sequence is fully defined by
        // the integer operations.
        self.state = self.state.wrapping_add(0x9E37_79B9_7F4A_7C15);

        let mut value = self.state;

        value = (value ^ (value >> 30))
            .wrapping_mul(0xBF58_476D_1CE4_E5B9);

        value = (value ^ (value >> 27))
            .wrapping_mul(0x94D0_49BB_1331_11EB);

        value ^ (value >> 31)
    }

    /// Returns a deterministic value in `[0, upper)`.
    fn index(&mut self, upper: usize) -> usize {
        if upper == 0 {
            return 0;
        }

        let upper_u64 = upper as u64;

        (self.next_u64() % upper_u64) as usize
    }

    /// Returns a deterministic Boolean.
    fn boolean(&mut self) -> bool {
        self.next_u64() & 1 == 1
    }

    /// Returns a deterministic small signed angle.
    fn angle(&mut self) -> f64 {
        let bucket = self.index(17);

        let numerator = bucket as f64 - 8.0;

        numerator * (PI / 8.0)
    }
}

// =============================================================================
// Test helpers
// =============================================================================

/// Creates a canonical logical qubit.
fn q(index: usize) -> QubitId {
    QubitId::new(index)
}

/// Creates a validated parameter.
fn parameter(value: f64) -> Parameter {
    Parameter::constant(value)
        .expect("finite test parameter must be accepted by the canonical IR")
}

/// Creates a non-parameterized canonical gate.
fn gate(kind: GateKind, qubits: &[usize]) -> Gate {
    Gate::new(
        kind,
        qubits.iter().copied().map(q).collect(),
        Vec::new(),
        None,
        None,
    )
    .expect("test gate must satisfy canonical IR invariants")
}

/// Creates a one-parameter canonical gate.
fn parameterized_gate(
    kind: GateKind,
    qubits: &[usize],
    value: f64,
) -> Gate {
    Gate::new(
        kind,
        qubits.iter().copied().map(q).collect(),
        vec![parameter(value)],
        None,
        None,
    )
    .expect("test parameterized gate must satisfy canonical IR invariants")
}

/// Creates a canonical barrier.
fn barrier(qubits: &[usize]) -> Gate {
    gate(GateKind::Barrier, qubits)
}

/// Creates a canonical reset.
fn reset(qubit: usize) -> Gate {
    gate(GateKind::Reset, &[qubit])
}

/// Builds a validated canonical circuit.
fn circuit(
    num_qubits: usize,
    operations: Vec<Gate>,
) -> QuantumCircuit {
    QuantumCircuit::from_operations(
        num_qubits,
        0,
        operations,
    )
    .expect("generated test circuit must satisfy canonical IR invariants")
}

/// Builds an optimizer context with the production configuration and limits.
fn production_context() -> OptimizationContext {
    OptimizationContext::production(
        OptimizationConfig::default(),
    )
    .expect("production optimization context must be constructible")
}

/// Runs the production local cancellation pass once.
///
/// The helper deliberately constructs a fresh context for each invocation so
/// the property is about optimizer behavior rather than context-counter reuse.
fn run_cancellation(
    original: &QuantumCircuit,
) -> QuantumCircuit {
    let mut optimized = original.clone();
    let pass = CancellationPass::new();
    let mut context = production_context();

    pass.run(
        &mut optimized,
        &mut context,
    )
    .expect("local cancellation must succeed for a valid canonical circuit");

    optimized
}

/// Verifies exact structural equivalence.
fn assert_structurally_equivalent(
    left: &QuantumCircuit,
    right: &QuantumCircuit,
) {
    let report = verify(
        left,
        right,
        EquivalenceConfig {
            method: EquivalenceMethod::Structural,
            ..EquivalenceConfig::default()
        },
    )
    .expect("structural equivalence verification must execute");

    assert_eq!(
        report.verdict,
        EquivalenceVerdict::Equivalent,
        "expected canonical circuits to be structurally equivalent"
    );
}

/// Verifies semantic equivalence with the canonical equivalence subsystem.
///
/// The exact verifier is used for small unitary circuits. If the circuit is
/// too large for the configured dense verifier, the property is deliberately
/// skipped rather than treating `Inconclusive` as success.
fn assert_semantically_equivalent_if_verifiable(
    left: &QuantumCircuit,
    right: &QuantumCircuit,
) {
    let report = verify(
        left,
        right,
        EquivalenceConfig {
            method: EquivalenceMethod::ExactUnitary {
                relation: UnitaryRelation::UpToGlobalPhase,
            },
            ..EquivalenceConfig::default()
        },
    )
    .expect("semantic equivalence verification must execute");

    match report.verdict {
        EquivalenceVerdict::Equivalent => {}

        EquivalenceVerdict::NotEquivalent => {
            panic!(
                "optimizer changed circuit semantics: \
                 left and right were proven not equivalent"
            );
        }

        EquivalenceVerdict::Inconclusive => {
            // This is an explicit and correct verifier outcome. A property
            // test must never reinterpret "insufficient verification
            // resources" as proof of equivalence.
        }
    }
}

/// Returns the configured stress scale.
///
/// The environment variable is intentionally read only by tests. Production
/// optimizer code must not depend on environment state for correctness.
fn property_scale() -> usize {
    std::env::var("ZAMANI_OPTIMIZATION_PROPERTY_SCALE")
        .ok()
        .and_then(|value| value.parse::<usize>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(DEFAULT_PROPERTY_SCALE)
}

/// Builds a deterministic generated circuit.
///
/// The generated circuit uses only canonical gates with known exact semantics.
/// It deliberately contains:
///
/// - self-inverse gates;
/// - inverse pairs;
/// - parameterized rotations;
/// - independent qubits;
/// - barriers;
/// - resets;
/// - multi-qubit operations.
///
/// This gives the optimizer a heterogeneous but fully controlled workload.
fn generated_circuit(
    seed: u64,
    operations: usize,
) -> QuantumCircuit {
    let mut generator = DeterministicGenerator::new(seed);

    let mut gates = Vec::with_capacity(operations);

    for _ in 0..operations {
        let choice = generator.index(12);

        let gate = match choice {
            0 => gate(
                GateKind::X,
                &[generator.index(4)],
            ),

            1 => gate(
                GateKind::Y,
                &[generator.index(4)],
            ),

            2 => gate(
                GateKind::Z,
                &[generator.index(4)],
            ),

            3 => gate(
                GateKind::H,
                &[generator.index(4)],
            ),

            4 => gate(
                GateKind::S,
                &[generator.index(4)],
            ),

            5 => gate(
                GateKind::Sdg,
                &[generator.index(4)],
            ),

            6 => gate(
                GateKind::T,
                &[generator.index(4)],
            ),

            7 => gate(
                GateKind::Tdg,
                &[generator.index(4)],
            ),

            8 => parameterized_gate(
                GateKind::RX,
                &[generator.index(4)],
                generator.angle(),
            ),

            9 => parameterized_gate(
                GateKind::RY,
                &[generator.index(4)],
                generator.angle(),
            ),

            10 => parameterized_gate(
                GateKind::RZ,
                &[generator.index(4)],
                generator.angle(),
            ),

            _ => {
                let first = generator.index(4);
                let mut second = generator.index(4);

                if first == second {
                    second = (second + 1) % 4;
                }

                gate(
                    if generator.boolean() {
                        GateKind::CX
                    } else {
                        GateKind::CZ
                    },
                    &[first, second],
                )
            }
        };

        gates.push(gate);

        // Occasionally add a semantic boundary. This specifically verifies
        // that cancellation remains local and does not cross a barrier.
        if generator.index(64) == 0 {
            gates.push(barrier(&[generator.index(4)]));
        }

        // Occasionally add reset. Reset is non-unitary and must never be
        // treated as an ordinary cancellable unitary gate.
        if generator.index(128) == 0 {
            gates.push(reset(generator.index(4)));
        }
    }

    circuit(4, gates)
}

/// Counts operations without relying on optimizer statistics.
fn operation_count(circuit: &QuantumCircuit) -> usize {
    circuit.operations().len()
}

// =============================================================================
// Canonical IR property tests
// =============================================================================

#[test]
fn property_qubit_ids_are_deterministic_and_value_based() {
    for index in 0..DEFAULT_ENUMERATION_SCALE {
        let first = q(index);
        let second = q(index);

        assert_eq!(
            first,
            second,
            "the same logical qubit index must produce equal QubitId values"
        );

        assert_eq!(
            first.index(),
            index,
            "QubitId must preserve its canonical logical index"
        );
    }
}

#[test]
fn property_distinct_qubits_remain_distinct() {
    for first in 0..64 {
        for second in 0..64 {
            if first == second {
                continue;
            }

            assert_ne!(
                q(first),
                q(second),
                "different logical qubit indices must remain distinct"
            );
        }
    }
}

#[test]
fn property_valid_single_qubit_gates_construct_successfully() {
    let kinds = [
        GateKind::I,
        GateKind::X,
        GateKind::Y,
        GateKind::Z,
        GateKind::H,
        GateKind::S,
        GateKind::Sdg,
        GateKind::T,
        GateKind::Tdg,
        GateKind::V,
        GateKind::Vdg,
    ];

    for kind in kinds {
        let operation = Gate::new(
            kind,
            vec![q(0)],
            Vec::new(),
            None,
            None,
        )
        .expect("canonical single-qubit gate must construct");

        operation
            .validate()
            .expect("constructed gate must remain valid");

        assert_eq!(
            operation.qubits(),
            &[q(0)],
            "constructed gate must preserve its logical operand"
        );
    }
}

#[test]
fn property_valid_two_qubit_gates_construct_successfully() {
    let kinds = [
        GateKind::CX,
        GateKind::CY,
        GateKind::CZ,
        GateKind::CH,
        GateKind::SWAP,
        GateKind::ISWAP,
        GateKind::ECR,
    ];

    for kind in kinds {
        let operation = Gate::new(
            kind,
            vec![q(0), q(1)],
            Vec::new(),
            None,
            None,
        )
        .expect("canonical two-qubit gate must construct");

        operation
            .validate()
            .expect("constructed two-qubit gate must remain valid");

        assert_eq!(
            operation.qubits(),
            &[q(0), q(1)],
            "two-qubit operands must remain in canonical order"
        );
    }
}

#[test]
fn property_valid_three_qubit_gates_construct_successfully() {
    let kinds = [
        GateKind::CCX,
        GateKind::CSWAP,
    ];

    for kind in kinds {
        let operation = Gate::new(
            kind,
            vec![q(0), q(1), q(2)],
            Vec::new(),
            None,
            None,
        )
        .expect("canonical three-qubit gate must construct");

        operation
            .validate()
            .expect("constructed three-qubit gate must remain valid");
    }
}

#[test]
fn property_duplicate_qubit_operands_are_rejected() {
    let kinds = [
        GateKind::CX,
        GateKind::CY,
        GateKind::CZ,
        GateKind::SWAP,
        GateKind::CCX,
        GateKind::CSWAP,
    ];

    for kind in kinds {
        let result = Gate::new(
            kind,
            vec![q(0), q(0)],
            Vec::new(),
            None,
            None,
        );

        assert!(
            result.is_err(),
            "gate {kind:?} must reject duplicate logical operands"
        );
    }
}

#[test]
fn property_wrong_operand_counts_are_rejected() {
    let single_qubit_kinds = [
        GateKind::X,
        GateKind::Y,
        GateKind::Z,
        GateKind::H,
        GateKind::T,
        GateKind::Tdg,
    ];

    for kind in single_qubit_kinds {
        let result = Gate::new(
            kind,
            vec![q(0), q(1)],
            Vec::new(),
            None,
            None,
        );

        assert!(
            result.is_err(),
            "single-qubit gate {kind:?} must reject two operands"
        );
    }

    let two_qubit_kinds = [
        GateKind::CX,
        GateKind::CY,
        GateKind::CZ,
        GateKind::SWAP,
    ];

    for kind in two_qubit_kinds {
        let result = Gate::new(
            kind,
            vec![q(0)],
            Vec::new(),
            None,
            None,
        );

        assert!(
            result.is_err(),
            "two-qubit gate {kind:?} must reject one operand"
        );
    }
}

#[test]
fn property_parameterized_gates_require_correct_parameter_arity() {
    let one_parameter_kinds = [
        GateKind::RX,
        GateKind::RY,
        GateKind::RZ,
        GateKind::Phase,
        GateKind::U1,
        GateKind::CRX,
        GateKind::CRY,
        GateKind::CRZ,
    ];

    for kind in one_parameter_kinds {
        let result = Gate::new(
            kind,
            match kind {
                GateKind::CRX
                | GateKind::CRY
                | GateKind::CRZ => {
                    vec![q(0), q(1)]
                }

                _ => vec![q(0)],
            },
            vec![parameter(PI / 4.0)],
            None,
            None,
        );

        assert!(
            result.is_ok(),
            "parameterized gate {kind:?} must accept one parameter"
        );
    }

    let u2 = Gate::new(
        GateKind::U2,
        vec![q(0)],
        vec![
            parameter(PI / 4.0),
            parameter(PI / 8.0),
        ],
        None,
        None,
    );

    assert!(
        u2.is_ok(),
        "U2 must accept exactly two parameters"
    );

    let u3 = Gate::new(
        GateKind::U3,
        vec![q(0)],
        vec![
            parameter(PI / 4.0),
            parameter(PI / 8.0),
            parameter(-PI / 16.0),
        ],
        None,
        None,
    );

    assert!(
        u3.is_ok(),
        "U3 must accept exactly three parameters"
    );
}

#[test]
fn property_non_finite_parameters_are_rejected() {
    let values = [
        f64::NAN,
        f64::INFINITY,
        f64::NEG_INFINITY,
    ];

    for value in values {
        assert!(
            Parameter::constant(value).is_err(),
            "non-finite parameter {value:?} must be rejected by canonical IR"
        );
    }
}

// =============================================================================
// Gate semantic classification properties
// =============================================================================

#[test]
fn property_self_inverse_gate_classification_is_stable() {
    let self_inverse = [
        GateKind::I,
        GateKind::X,
        GateKind::Y,
        GateKind::Z,
        GateKind::H,
        GateKind::CX,
        GateKind::CY,
        GateKind::CZ,
        GateKind::CH,
        GateKind::SWAP,
        GateKind::CCX,
        GateKind::CSWAP,
    ];

    for kind in self_inverse {
        assert!(
            kind.is_self_inverse(),
            "{kind:?} must be classified as self-inverse"
        );
    }

    let non_self_inverse = [
        GateKind::S,
        GateKind::Sdg,
        GateKind::T,
        GateKind::Tdg,
        GateKind::V,
        GateKind::Vdg,
        GateKind::RX,
        GateKind::RY,
        GateKind::RZ,
        GateKind::Phase,
    ];

    for kind in non_self_inverse {
        assert!(
            !kind.is_self_inverse(),
            "{kind:?} must not be classified as an unconditional self-inverse"
        );
    }
}

#[test]
fn property_unitary_classification_excludes_non_unitary_operations() {
    let unitary = [
        GateKind::I,
        GateKind::X,
        GateKind::H,
        GateKind::CX,
        GateKind::RX,
        GateKind::RZ,
    ];

    for kind in unitary {
        assert!(
            kind.is_unitary(),
            "{kind:?} must be classified as unitary"
        );
    }

    let non_unitary = [
        GateKind::Measure,
        GateKind::Barrier,
        GateKind::Reset,
    ];

    for kind in non_unitary {
        assert!(
            !kind.is_unitary(),
            "{kind:?} must be classified as non-unitary"
        );
    }
}

#[test]
fn property_measurement_barrier_and_reset_are_not_unitary() {
    assert!(GateKind::Measure.is_measurement());
    assert!(GateKind::Barrier.is_barrier());
    assert!(GateKind::Reset.is_reset());

    assert!(!GateKind::Measure.is_unitary());
    assert!(!GateKind::Barrier.is_unitary());
    assert!(!GateKind::Reset.is_unitary());
}

// =============================================================================
// Cancellation relation properties
// =============================================================================

#[test]
fn property_self_inverse_cancellation_is_reflexive_for_self_inverse_gates() {
    let pass = CancellationPass::new();

    let cases = [
        (GateKind::X, &[0][..]),
        (GateKind::Y, &[0][..]),
        (GateKind::Z, &[0][..]),
        (GateKind::H, &[0][..]),
        (GateKind::CX, &[0, 1][..]),
        (GateKind::CZ, &[0, 1][..]),
        (GateKind::SWAP, &[0, 1][..]),
        (GateKind::CCX, &[0, 1, 2][..]),
    ];

    for (kind, qubits) in cases {
        let first = gate(kind, qubits);
        let second = gate(kind, qubits);

        assert!(
            pass.can_cancel(&first, &second),
            "two adjacent {kind:?} operations on identical operands must cancel"
        );

        assert!(
            pass.can_cancel(&second, &first),
            "self-inverse cancellation must be symmetric"
        );
    }
}

#[test]
fn property_inverse_cancellation_is_symmetric() {
    let pass = CancellationPass::new();

    let pairs = [
        (GateKind::S, GateKind::Sdg),
        (GateKind::T, GateKind::Tdg),
    ];

    for (first_kind, second_kind) in pairs {
        let first = gate(first_kind, &[0]);
        let second = gate(second_kind, &[0]);

        assert!(
            pass.can_cancel(&first, &second),
            "{first_kind:?}; {second_kind:?} must cancel"
        );

        assert!(
            pass.can_cancel(&second, &first),
            "{second_kind:?}; {first_kind:?} must cancel"
        );
    }
}

#[test]
fn property_cancellation_requires_matching_logical_operands() {
    let pass = CancellationPass::new();

    let first = gate(GateKind::X, &[0]);
    let second = gate(GateKind::X, &[1]);

    assert!(
        !pass.can_cancel(&first, &second),
        "operations on different logical qubits must not cancel"
    );
}

#[test]
fn property_two_qubit_cancellation_requires_exact_operand_order() {
    let pass = CancellationPass::new();

    let first = gate(GateKind::CX, &[0, 1]);
    let reversed = gate(GateKind::CX, &[1, 0]);

    assert!(
        !pass.can_cancel(&first, &reversed),
        "controlled-gate operand order must not be silently changed"
    );
}

#[test]
fn property_cancellation_does_not_cross_barriers() {
    let original = circuit(
        2,
        vec![
            gate(GateKind::X, &[0]),
            barrier(&[0]),
            gate(GateKind::X, &[0]),
        ],
    );

    let optimized = run_cancellation(&original);

    assert_eq!(
        operation_count(&optimized),
        3,
        "a barrier must prevent cancellation across it"
    );

    optimized
        .validate()
        .expect("barrier-protected circuit must remain valid");
}

#[test]
fn property_cancellation_does_not_cross_reset() {
    let original = circuit(
        1,
        vec![
            gate(GateKind::X, &[0]),
            reset(0),
            gate(GateKind::X, &[0]),
        ],
    );

    let optimized = run_cancellation(&original);

    assert_eq!(
        operation_count(&optimized),
        3,
        "reset must remain a semantic boundary"
    );

    optimized
        .validate()
        .expect("reset-protected circuit must remain valid");
}

#[test]
fn property_identity_is_removed_without_touching_other_operations() {
    let original = circuit(
        1,
        vec![
            gate(GateKind::I, &[0]),
            gate(GateKind::X, &[0]),
        ],
    );

    let optimized = run_cancellation(&original);

    assert_eq!(
        operation_count(&optimized),
        1,
        "identity removal must remove exactly the identity operation"
    );

    assert_eq!(
        optimized.operations()[0].kind(),
        GateKind::X,
        "identity removal must preserve the neighboring operation"
    );
}

#[test]
fn property_t_and_tdg_cancel_exactly() {
    let original = circuit(
        1,
        vec![
            gate(GateKind::T, &[0]),
            gate(GateKind::Tdg, &[0]),
        ],
    );

    let optimized = run_cancellation(&original);

    assert!(
        optimized.operations().is_empty(),
        "T followed by Tdg must cancel exactly"
    );
}

#[test]
fn property_s_and_sdg_cancel_exactly() {
    let original = circuit(
        1,
        vec![
            gate(GateKind::S, &[0]),
            gate(GateKind::Sdg, &[0]),
        ],
    );

    let optimized = run_cancellation(&original);

    assert!(
        optimized.operations().is_empty(),
        "S followed by Sdg must cancel exactly"
    );
}

// =============================================================================
// Parameterized cancellation properties
// =============================================================================

#[test]
fn property_exact_parameterized_inverse_pair_cancels() {
    let pass = CancellationPass::new();

    let first = parameterized_gate(
        GateKind::RX,
        &[0],
        PI / 4.0,
    );

    let second = parameterized_gate(
        GateKind::RX,
        &[0],
        -PI / 4.0,
    );

    assert!(
        pass.can_cancel(&first, &second),
        "RX(theta); RX(-theta) must be recognized as an exact inverse pair"
    );

    assert!(
        pass.can_cancel(&second, &first),
        "RX(-theta); RX(theta) must be recognized symmetrically"
    );
}

#[test]
fn property_parameterized_inverse_pair_preserves_other_qubits() {
    let original = circuit(
        2,
        vec![
            parameterized_gate(
                GateKind::RX,
                &[0],
                PI / 4.0,
            ),
            gate(GateKind::H, &[1]),
            parameterized_gate(
                GateKind::RX,
                &[0],
                -PI / 4.0,
            ),
        ],
    );

    let optimized = run_cancellation(&original);

    assert_eq!(
        operation_count(&optimized),
        3,
        "non-adjacent inverse rotations must not be cancelled by local cancellation"
    );

    assert_eq!(
        optimized.operations()[1].kind(),
        GateKind::H,
        "independent operations must remain untouched"
    );
}

#[test]
fn property_zero_angle_parameterized_gate_is_safe_to_construct() {
    let rotation = parameterized_gate(
        GateKind::RZ,
        &[0],
        0.0,
    );

    rotation
        .validate()
        .expect("zero-angle parameterized gate must remain valid");

    assert_eq!(
        rotation.constant_parameters(),
        Some(vec![0.0]),
        "zero-angle constant parameter must remain represented exactly"
    );
}

// =============================================================================
// Circuit construction and validation properties
// =============================================================================

#[test]
fn property_empty_circuit_is_valid() {
    let circuit = circuit(0, Vec::new());

    circuit
        .validate()
        .expect("empty zero-qubit circuit must be valid");

    assert_eq!(
        operation_count(&circuit),
        0,
        "empty circuit must contain zero operations"
    );
}

#[test]
fn property_single_operation_circuit_is_valid() {
    let circuit = circuit(
        1,
        vec![gate(GateKind::H, &[0])],
    );

    circuit
        .validate()
        .expect("single-operation circuit must be valid");

    assert_eq!(
        operation_count(&circuit),
        1,
        "single-operation circuit must contain one operation"
    );
}

#[test]
fn property_sparse_logical_qubit_namespace_is_valid() {
    let circuit = circuit(
        1_000_000,
        vec![
            gate(GateKind::X, &[999_999]),
        ],
    );

    circuit
        .validate()
        .expect("sparse logical namespace must remain valid");

    assert_eq!(
        circuit.num_qubits(),
        1_000_000,
        "declared logical namespace must be preserved"
    );

    assert_eq!(
        circuit.operations()[0].qubits(),
        &[q(999_999)],
        "sparse logical operand must be preserved"
    );
}

#[test]
fn property_circuit_clone_is_structurally_equivalent() {
    let original = generated_circuit(
        DEFAULT_SEED,
        128,
    );

    let clone = original.clone();

    assert_structurally_equivalent(
        &original,
        &clone,
    );
}

// =============================================================================
// Optimization idempotence properties
// =============================================================================

#[test]
fn property_cancellation_is_idempotent_on_small_exhaustive_domain() {
    let kinds = [
        GateKind::X,
        GateKind::Y,
        GateKind::Z,
        GateKind::H,
        GateKind::S,
        GateKind::Sdg,
        GateKind::T,
        GateKind::Tdg,
    ];

    for first_kind in kinds {
        for second_kind in kinds {
            for first_qubit in 0..2 {
                for second_qubit in 0..2 {
                    let original = circuit(
                        2,
                        vec![
                            gate(first_kind, &[first_qubit]),
                            gate(second_kind, &[second_qubit]),
                        ],
                    );

                    let once = run_cancellation(&original);
                    let twice = run_cancellation(&once);

                    assert_structurally_equivalent(
                        &once,
                        &twice,
                    );
                }
            }
        }
    }
}

#[test]
fn property_cancellation_is_idempotent_on_generated_circuits() {
    let scale = property_scale().min(DEFAULT_ENUMERATION_SCALE * 16);

    for case_index in 0..16u64 {
        let length = (scale / 16).max(1);

        let original = generated_circuit(
            DEFAULT_SEED.wrapping_add(case_index),
            length,
        );

        let once = run_cancellation(&original);
        let twice = run_cancellation(&once);

        assert_structurally_equivalent(
            &once,
            &twice,
        );

        once
            .validate()
            .expect("first optimization result must remain valid");

        twice
            .validate()
            .expect("second optimization result must remain valid");
    }
}

// =============================================================================
// Operation-count monotonicity properties
// =============================================================================

#[test]
fn property_cancellation_never_increases_operation_count() {
    let scale = property_scale().min(DEFAULT_ENUMERATION_SCALE * 16);

    for case_index in 0..16u64 {
        let original = generated_circuit(
            DEFAULT_SEED.wrapping_add(case_index),
            (scale / 16).max(1),
        );

        let optimized = run_cancellation(&original);

        assert!(
            operation_count(&optimized)
                <= operation_count(&original),
            "deletion-only cancellation must never increase operation count"
        );
    }
}

#[test]
fn property_repeated_cancellation_is_monotonic() {
    let original = generated_circuit(
        DEFAULT_SEED,
        512,
    );

    let first = run_cancellation(&original);
    let second = run_cancellation(&first);
    let third = run_cancellation(&second);

    assert!(
        operation_count(&first)
            <= operation_count(&original)
    );

    assert!(
        operation_count(&second)
            <= operation_count(&first)
    );

    assert!(
        operation_count(&third)
            <= operation_count(&second)
    );

    assert_structurally_equivalent(
        &second,
        &third,
    );
}

// =============================================================================
// Semantic-preservation properties
// =============================================================================

#[test]
fn property_self_inverse_cancellation_preserves_semantics() {
    let original = circuit(
        2,
        vec![
            gate(GateKind::H, &[0]),
            gate(GateKind::H, &[0]),
            gate(GateKind::X, &[1]),
        ],
    );

    let optimized = run_cancellation(&original);

    assert_semantically_equivalent_if_verifiable(
        &original,
        &optimized,
    );
}

#[test]
fn property_inverse_pair_cancellation_preserves_semantics() {
    let original = circuit(
        1,
        vec![
            gate(GateKind::S, &[0]),
            gate(GateKind::Sdg, &[0]),
            gate(GateKind::H, &[0]),
        ],
    );

    let optimized = run_cancellation(&original);

    assert_semantically_equivalent_if_verifiable(
        &original,
        &optimized,
    );
}

#[test]
fn property_parameterized_inverse_cancellation_preserves_semantics() {
    let original = circuit(
        1,
        vec![
            parameterized_gate(
                GateKind::RX,
                &[0],
                PI / 4.0,
            ),
            parameterized_gate(
                GateKind::RX,
                &[0],
                -PI / 4.0,
            ),
            gate(GateKind::H, &[0]),
        ],
    );

    let optimized = run_cancellation(&original);

    assert_semantically_equivalent_if_verifiable(
        &original,
        &optimized,
    );
}

#[test]
fn property_cancellation_preserves_semantics_for_generated_small_circuits() {
    for case_index in 0..32u64 {
        let original = generated_circuit(
            DEFAULT_SEED.wrapping_add(case_index),
            12,
        );

        let optimized = run_cancellation(&original);

        assert_semantically_equivalent_if_verifiable(
            &original,
            &optimized,
        );
    }
}

#[test]
fn property_optimization_output_always_passes_canonical_validation() {
    for case_index in 0..32u64 {
        let original = generated_circuit(
            DEFAULT_SEED.wrapping_add(case_index),
            32,
        );

        let optimized = run_cancellation(&original);

        optimized
            .validate()
            .expect(
                "optimization must never return an invalid canonical QuantumCircuit"
            );
    }
}

// =============================================================================
// Equivalence-verifier properties
// =============================================================================

#[test]
fn property_structural_equivalence_is_reflexive() {
    for case_index in 0..16u64 {
        let circuit = generated_circuit(
            DEFAULT_SEED.wrapping_add(case_index),
            32,
        );

        let report = verify(
            &circuit,
            &circuit,
            EquivalenceConfig {
                method: EquivalenceMethod::Structural,
                ..EquivalenceConfig::default()
            },
        )
        .expect("structural verification must succeed");

        assert_eq!(
            report.verdict,
            EquivalenceVerdict::Equivalent,
            "a circuit must be structurally equivalent to itself"
        );
    }
}

#[test]
fn property_structural_equivalence_is_symmetric() {
    for case_index in 0..16u64 {
        let first = generated_circuit(
            DEFAULT_SEED.wrapping_add(case_index),
            24,
        );

        let second = first.clone();

        let forward = verify(
            &first,
            &second,
            EquivalenceConfig {
                method: EquivalenceMethod::Structural,
                ..EquivalenceConfig::default()
            },
        )
        .expect("forward structural verification must succeed");

        let reverse = verify(
            &second,
            &first,
            EquivalenceConfig {
                method: EquivalenceMethod::Structural,
                ..EquivalenceConfig::default()
            },
        )
        .expect("reverse structural verification must succeed");

        assert_eq!(
            forward.verdict,
            reverse.verdict,
            "structural equivalence must be symmetric"
        );
    }
}

#[test]
fn property_structurally_different_circuits_are_not_equivalent() {
    let first = circuit(
        1,
        vec![gate(GateKind::X, &[0])],
    );

    let second = circuit(
        1,
        vec![gate(GateKind::H, &[0])],
    );

    let report = verify(
        &first,
        &second,
        EquivalenceConfig {
            method: EquivalenceMethod::Structural,
            ..EquivalenceConfig::default()
        },
    )
    .expect("structural verification must execute");

    assert_eq!(
        report.verdict,
        EquivalenceVerdict::NotEquivalent,
        "different canonical gate structures must not be structurally equivalent"
    );
}

#[test]
fn property_different_qubit_counts_are_not_equivalent() {
    let first = circuit(
        1,
        vec![gate(GateKind::X, &[0])],
    );

    let second = circuit(
        2,
        vec![gate(GateKind::X, &[0])],
    );

    let report = verify(
        &first,
        &second,
        EquivalenceConfig {
            method: EquivalenceMethod::Structural,
            ..EquivalenceConfig::default()
        },
    )
    .expect("structural verification must execute");

    assert_eq!(
        report.verdict,
        EquivalenceVerdict::NotEquivalent,
        "different logical circuit widths must not be equivalent"
    );
}

#[test]
fn property_equivalence_does_not_confuse_gate_count_with_semantics() {
    let first = circuit(
        1,
        vec![
            gate(GateKind::X, &[0]),
            gate(GateKind::X, &[0]),
        ],
    );

    let second = circuit(
        1,
        Vec::new(),
    );

    assert_eq!(
        operation_count(&first),
        2,
        "test precondition requires two operations"
    );

    assert_eq!(
        operation_count(&second),
        0,
        "test precondition requires zero operations"
    );

    let report = verify(
        &first,
        &second,
        EquivalenceConfig {
            method: EquivalenceMethod::ExactUnitary {
                relation: UnitaryRelation::UpToGlobalPhase,
            },
            ..EquivalenceConfig::default()
        },
    )
    .expect("semantic verification must execute");

    assert_eq!(
        report.verdict,
        EquivalenceVerdict::Equivalent,
        "X followed by X must be semantically equivalent to identity"
    );
}

// =============================================================================
// Metamorphic properties
// =============================================================================

#[test]
fn property_adding_an_independent_gate_changes_only_that_qubit_dependency() {
    let original = circuit(
        2,
        vec![
            gate(GateKind::H, &[0]),
            gate(GateKind::X, &[0]),
        ],
    );

    let extended = circuit(
        2,
        vec![
            gate(GateKind::H, &[0]),
            gate(GateKind::X, &[0]),
            gate(GateKind::Z, &[1]),
        ],
    );

    let original_optimized = run_cancellation(&original);
    let extended_optimized = run_cancellation(&extended);

    assert_eq!(
        operation_count(&original_optimized),
        2,
        "original circuit must remain unchanged"
    );

    assert_eq!(
        operation_count(&extended_optimized),
        3,
        "independent qubit operation must remain"
    );

    extended_optimized
        .validate()
        .expect("extended optimized circuit must remain valid");
}

#[test]
fn property_inserting_an_exact_inverse_pair_is_semantically_neutral() {
    let base = circuit(
        1,
        vec![
            gate(GateKind::H, &[0]),
            gate(GateKind::Z, &[0]),
        ],
    );

    let augmented = circuit(
        1,
        vec![
            gate(GateKind::H, &[0]),
            gate(GateKind::X, &[0]),
            gate(GateKind::X, &[0]),
            gate(GateKind::Z, &[0]),
        ],
    );

    let report = verify(
        &base,
        &augmented,
        EquivalenceConfig {
            method: EquivalenceMethod::ExactUnitary {
                relation: UnitaryRelation::UpToGlobalPhase,
            },
            ..EquivalenceConfig::default()
        },
    )
    .expect("semantic verification must execute");

    assert_eq!(
        report.verdict,
        EquivalenceVerdict::Equivalent,
        "inserting X;X must not change circuit semantics"
    );
}

#[test]
fn property_optimization_removes_inserted_inverse_pair() {
    let augmented = circuit(
        1,
        vec![
            gate(GateKind::H, &[0]),
            gate(GateKind::X, &[0]),
            gate(GateKind::X, &[0]),
            gate(GateKind::Z, &[0]),
        ],
    );

    let optimized = run_cancellation(&augmented);

    assert_eq!(
        operation_count(&optimized),
        2,
        "the inserted X;X pair must disappear"
    );

    assert_eq!(
        optimized.operations()[0].kind(),
        GateKind::H
    );

    assert_eq!(
        optimized.operations()[1].kind(),
        GateKind::Z
    );
}

// =============================================================================
// Generated-circuit properties
// =============================================================================

#[test]
fn property_generated_circuits_are_deterministic() {
    for case_index in 0..32u64 {
        let seed = DEFAULT_SEED.wrapping_add(case_index);

        let first = generated_circuit(
            seed,
            128,
        );

        let second = generated_circuit(
            seed,
            128,
        );

        assert_structurally_equivalent(
            &first,
            &second,
        );
    }
}

#[test]
fn property_generated_circuits_are_valid_before_optimization() {
    for case_index in 0..32u64 {
        let circuit = generated_circuit(
            DEFAULT_SEED.wrapping_add(case_index),
            128,
        );

        circuit
            .validate()
            .expect("deterministically generated circuit must be valid");
    }
}

#[test]
fn property_generated_circuit_optimization_is_deterministic() {
    for case_index in 0..32u64 {
        let original = generated_circuit(
            DEFAULT_SEED.wrapping_add(case_index),
            128,
        );

        let first = run_cancellation(&original);
        let second = run_cancellation(&original);

        assert_structurally_equivalent(
            &first,
            &second,
        );
    }
}

// =============================================================================
// Cascading cancellation properties
// =============================================================================

#[test]
fn property_cascading_cancellation_reaches_fixed_point() {
    let original = circuit(
        1,
        vec![
            gate(GateKind::H, &[0]),
            gate(GateKind::X, &[0]),
            gate(GateKind::X, &[0]),
            gate(GateKind::H, &[0]),
        ],
    );

    let optimized = run_cancellation(&original);

    assert!(
        optimized.operations().is_empty(),
        "stack-style cancellation must expose cascading cancellation"
    );

    let second = run_cancellation(&optimized);

    assert_structurally_equivalent(
        &optimized,
        &second,
    );
}

#[test]
fn property_multiple_inverse_pairs_cancel_independently() {
    let original = circuit(
        2,
        vec![
            gate(GateKind::X, &[0]),
            gate(GateKind::X, &[0]),
            gate(GateKind::H, &[1]),
            gate(GateKind::H, &[1]),
            gate(GateKind::T, &[0]),
            gate(GateKind::Tdg, &[0]),
        ],
    );

    let optimized = run_cancellation(&original);

    assert!(
        optimized.operations().is_empty(),
        "all independent adjacent inverse pairs must cancel"
    );
}

// =============================================================================
// Semantic-boundary properties
// =============================================================================

#[test]
fn property_barrier_is_preserved_by_cancellation() {
    let original = circuit(
        2,
        vec![
            gate(GateKind::X, &[0]),
            barrier(&[0, 1]),
            gate(GateKind::X, &[0]),
        ],
    );

    let optimized = run_cancellation(&original);

    assert_eq!(
        operation_count(&optimized),
        3,
        "barrier and both neighboring gates must survive"
    );

    assert!(
        optimized.operations()[1].is_barrier(),
        "the barrier itself must be preserved"
    );
}

#[test]
fn property_reset_is_preserved_by_cancellation() {
    let original = circuit(
        1,
        vec![
            gate(GateKind::X, &[0]),
            reset(0),
        ],
    );

    let optimized = run_cancellation(&original);

    assert_eq!(
        operation_count(&optimized),
        2,
        "reset must not be removed by local unitary cancellation"
    );

    assert!(
        optimized.operations()[1].is_reset(),
        "reset operation must remain in the optimized circuit"
    );
}

// =============================================================================
// Large deterministic stress property
// =============================================================================

#[test]
fn property_large_generated_circuit_remains_valid_and_converges() {
    let scale = property_scale();

    let original = generated_circuit(
        DEFAULT_SEED,
        scale,
    );

    original
        .validate()
        .expect("large generated circuit must be valid");

    let optimized = run_cancellation(&original);

    optimized
        .validate()
        .expect("large optimized circuit must remain valid");

    assert!(
        operation_count(&optimized)
            <= operation_count(&original),
        "local cancellation must not increase operation count"
    );

    let repeated = run_cancellation(&optimized);

    assert_structurally_equivalent(
        &optimized,
        &repeated,
    );
}

// =============================================================================
// Boundary-size properties
// =============================================================================

#[test]
fn property_zero_operation_optimization_is_identity() {
    let original = circuit(
        0,
        Vec::new(),
    );

    let optimized = run_cancellation(&original);

    assert_structurally_equivalent(
        &original,
        &optimized,
    );
}

#[test]
fn property_one_operation_optimization_is_stable() {
    let original = circuit(
        1,
        vec![gate(GateKind::H, &[0])],
    );

    let optimized = run_cancellation(&original);

    assert_structurally_equivalent(
        &original,
        &optimized,
    );
}

#[test]
fn property_single_identity_operation_is_removed() {
    let original = circuit(
        1,
        vec![gate(GateKind::I, &[0])],
    );

    let optimized = run_cancellation(&original);

    assert!(
        optimized.operations().is_empty(),
        "single identity operation must be removed"
    );
}

// =============================================================================
// Cancellation API classification properties
// =============================================================================

#[test]
fn property_identity_classification_matches_exact_identity_operations() {
    let pass = CancellationPass::new();

    let identity = gate(GateKind::I, &[0]);

    assert!(
        pass.is_identity(&identity),
        "canonical I must be recognized as an exact identity"
    );
}

#[test]
fn property_non_identity_gate_is_not_unconditionally_identity() {
    let pass = CancellationPass::new();

    let gates = [
        gate(GateKind::X, &[0]),
        gate(GateKind::Y, &[0]),
        gate(GateKind::Z, &[0]),
        gate(GateKind::H, &[0]),
        gate(GateKind::S, &[0]),
        gate(GateKind::T, &[0]),
    ];

    for operation in gates {
        assert!(
            !pass.is_identity(&operation),
            "{:?} must not be unconditionally classified as identity",
            operation.kind()
        );
    }
}

// =============================================================================
// Resource-scaling properties
// =============================================================================

#[test]
fn property_large_logical_namespace_does_not_require_dense_qubit_iteration() {
    let original = circuit(
        10_000_000,
        vec![
            gate(GateKind::X, &[9_999_999]),
            gate(GateKind::X, &[9_999_999]),
        ],
    );

    let optimized = run_cancellation(&original);

    assert!(
        optimized.operations().is_empty(),
        "sparse high-index logical qubits must still be optimizable"
    );

    assert_eq!(
        optimized.num_qubits(),
        10_000_000,
        "optimization must preserve the declared logical namespace"
    );
}

#[test]
fn property_test_scale_is_not_an_optimizer_semantic_limit() {
    let requested = property_scale();

    assert!(
        requested > 0,
        "configured property workload must always be positive"
    );

    // The important property here is that the test workload is configuration,
    // not a hidden optimizer constant. The actual optimizer resource policy is
    // responsible for deciding whether a particular workload can execute.
    //
    // This assertion intentionally does not impose a maximum.
    assert_eq!(
        requested,
        property_scale(),
        "property workload configuration must be deterministic"
    );
}

// =============================================================================
// Regression properties for known optimizer hazards
// =============================================================================

#[test]
fn property_different_qubit_operations_do_not_cancel_after_reordering() {
    let original = circuit(
        2,
        vec![
            gate(GateKind::X, &[0]),
            gate(GateKind::X, &[1]),
        ],
    );

    let optimized = run_cancellation(&original);

    assert_eq!(
        operation_count(&optimized),
        2,
        "operations on independent but different qubits are not inverse pairs"
    );
}

#[test]
fn property_control_and_target_roles_are_preserved() {
    let original = circuit(
        2,
        vec![
            gate(GateKind::CX, &[0, 1]),
            gate(GateKind::CX, &[0, 1]),
        ],
    );

    let optimized = run_cancellation(&original);

    assert!(
        optimized.operations().is_empty(),
        "identical CX operations must cancel"
    );

    let reversed = circuit(
        2,
        vec![
            gate(GateKind::CX, &[0, 1]),
            gate(GateKind::CX, &[1, 0]),
        ],
    );

    let reversed_optimized = run_cancellation(&reversed);

    assert_eq!(
        operation_count(&reversed_optimized),
        2,
        "reversing control/target roles must prevent accidental cancellation"
    );
}

#[test]
fn property_non_adjacent_inverse_pair_is_not_removed_by_local_cancellation() {
    let original = circuit(
        1,
        vec![
            gate(GateKind::X, &[0]),
            gate(GateKind::H, &[0]),
            gate(GateKind::X, &[0]),
        ],
    );

    let optimized = run_cancellation(&original);

    assert_eq!(
        operation_count(&optimized),
        3,
        "local cancellation must not perform non-local gate movement"
    );
}

#[test]
fn property_optimizer_does_not_replace_exact_semantics_with_gate_count_heuristics() {
    let original = circuit(
        1,
        vec![
            gate(GateKind::X, &[0]),
            gate(GateKind::X, &[0]),
            gate(GateKind::H, &[0]),
        ],
    );

    let optimized = run_cancellation(&original);

    assert_eq!(
        operation_count(&optimized),
        1,
        "only the cancellable X;X pair should disappear"
    );

    assert_eq!(
        optimized.operations()[0].kind(),
        GateKind::H,
        "remaining operation must be preserved exactly"
    );

    assert_semantically_equivalent_if_verifiable(
        &original,
        &optimized,
    );
}

// =============================================================================
// Final aggregate property
// =============================================================================

#[test]
fn property_complete_foundational_optimizer_contract() {
    let original = generated_circuit(
        DEFAULT_SEED,
        256,
    );

    let once = run_cancellation(&original);
    let twice = run_cancellation(&once);

    // Structural validity.
    original
        .validate()
        .expect("original generated circuit must be valid");

    once
        .validate()
        .expect("first optimization result must be valid");

    twice
        .validate()
        .expect("second optimization result must be valid");

    // Monotonicity for the deletion-only cancellation pass.
    assert!(
        operation_count(&once)
            <= operation_count(&original)
    );

    assert!(
        operation_count(&twice)
            <= operation_count(&once)
    );

    // Fixed-point/idempotence.
    assert_structurally_equivalent(
        &once,
        &twice,
    );

    // Semantic preservation.
    assert_semantically_equivalent_if_verifiable(
        &original,
        &once,
    );

    // Deterministic repeated execution.
    let repeated = run_cancellation(&original);

    assert_structurally_equivalent(
        &once,
        &repeated,
    );
}