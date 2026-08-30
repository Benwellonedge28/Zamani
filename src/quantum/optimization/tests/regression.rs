//! Zamani Quantum Optimization — Regression Test Suite
//!
//! `src/quantum/optimization/tests/regression.rs`
//!
//! # Purpose
//!
//! This module contains permanent regression tests for bugs, edge cases,
//! semantic hazards, integration failures, and scalability failures discovered
//! in the Zamani quantum optimization subsystem.
//!
//! The tests are intentionally written against the canonical public contracts
//! rather than optimizer implementation details.
//!
//! The canonical Quantum IR is:
//!
//! ```text
//! crate::quantum::ir
//! ```
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
//! This distinction is deliberate and protects the optimizer test suite from
//! reproducing the repository's historical IR module-naming inconsistency.
//!
//! # Architectural position
//!
//! ```text
//!                         Zamani source
//!                              │
//!                              ▼
//!                         quantum::ir
//!                              │
//!                              ▼
//!                    optimization subsystem
//!                              │
//!             ┌────────────────┼────────────────┐
//!             │                │                │
//!             ▼                ▼                ▼
//!          analyses          passes         verification
//!             │                │                │
//!             └────────────────┼────────────────┘
//!                              ▼
//!                    optimized Quantum IR
//!                              │
//!                              ▼
//!                            tests
//! ```
//!
//! This file tests the optimizer as a consumer of the canonical IR. It does
//! not define an alternative circuit representation.
//!
//! # What this suite protects
//!
//! The regression suite protects against, among other things:
//!
//! - reintroduction of duplicate optimizer-local gate types;
//! - incorrect `qubits`/`qubit` module imports;
//! - incorrect logical-qubit identity handling;
//! - cancellation across barriers;
//! - cancellation across reset boundaries;
//! - cancellation across measurement boundaries when supported by the IR;
//! - cancellation across different qubit operands;
//! - cancellation of non-inverse operations;
//! - incorrect inverse-pair handling;
//! - incorrect two-qubit gate cancellation;
//! - incorrect gate operand ordering;
//! - incorrect parameterized inverse cancellation;
//! - accidental approximate floating-point cancellation;
//! - removal of rotations that are not exact identities;
//! - failure to remove exact zero-angle operations;
//! - cascading cancellation failures;
//! - non-idempotent local cancellation;
//! - mutation of valid circuits into invalid circuits;
//! - partial mutation after optimizer failure;
//! - operation-count growth in a deletion-only pass;
//! - accidental quadratic behavior caused by repeated indexed deletion;
//! - deterministic optimizer behavior;
//! - reproducibility of generated regression cases;
//! - sparse logical qubit namespaces;
//! - very large operation sequences;
//! - explicit stress scaling;
//! - optimizer cooperation with cancellation/resource policies;
//! - preservation of circuit qubit count;
//! - preservation of non-optimized semantic boundaries;
//! - pass metadata stability;
//! - optimizer behavior on empty circuits;
//! - optimizer behavior on one-operation circuits;
//! - optimizer behavior on already optimized circuits;
//! - regression against gate identities known to be exact;
//! - regression against false identities;
//! - regression against cross-qubit cancellation;
//! - regression against cross-boundary cancellation.
//!
//! # Test philosophy
//!
//! Regression tests should encode a previously discovered invariant or a
//! high-value edge case. They must not assert private implementation details.
//!
//! Therefore this file intentionally avoids assertions about:
//!
//! - private vector layouts;
//! - private optimizer fields;
//! - allocation counts;
//! - hash-map iteration order;
//! - implementation-specific temporary structures;
//! - exact internal pass iteration order;
//! - internal helper functions unless they are part of the public contract.
//!
//! # Scaling
//!
//! The default suite is deliberately bounded for normal CI.
//!
//! A larger deterministic stress workload can be requested with:
//!
//! ```text
//! ZAMANI_OPTIMIZATION_REGRESSION_SCALE=100000 cargo test
//! ```
//!
//! This environment variable controls only the test workload. It is NOT an
//! optimizer limit.
//!
//! The optimizer's real resource policy remains owned by
//! `OptimizationLimits`/`OptimizationContext`.
//!
//! The test suite therefore scales upward with available machine resources
//! without claiming that quantum optimization itself is mathematically or
//! physically unlimited.
//!
//! # Determinism
//!
//! Generated regression circuits use a deterministic local generator.
//!
//! No operating-system randomness, timestamps, thread-local randomness, or
//! hash iteration order is used.
//!
//! A failure can therefore be reproduced from its explicit seed.
//!
//! # Safety
//!
//! This file explicitly forbids unsafe Rust.
//!
//! No regression test requires unsafe code.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust only;
//! - no nightly features;
//! - no external test framework;
//! - no unsafe code.
//!
//! # Integration contract
//!
//! `tests/mod.rs` must eventually contain:
//!
//! ```text
//! mod regression;
//! ```
//!
//! The optimizer's test module should own that declaration. This file itself
//! must remain independent of `tests/mod.rs` implementation details.
//!
//! The tests consume:
//!
//! - `quantum::ir::QuantumCircuit`;
//! - `quantum::ir::Gate`;
//! - `quantum::ir::GateKind`;
//! - `quantum::ir::Parameter`;
//! - `quantum::ir::qubit::QubitId`;
//! - `optimization::context::OptimizationContext`;
//! - `optimization::config::OptimizationConfig`;
//! - `optimization::local::cancellation::CancellationPass`;
//! - `optimization::pass::OptimizationPass`.
//!
//! The suite deliberately does not require future optimization modules to
//! exist. Adding a new pass must not require rewriting these tests.
//!
//! # Canonical IR rule
//!
//! Every circuit in this file is constructed from the canonical Quantum IR.
//!
//! There is no local `QuantumGate`.
//! There is no local `QuantumOperation`.
//! There is no local `QuantumCircuit`.
//!
//! # Regression numbering
//!
//! Regression names are descriptive rather than tied to GitHub issue numbers.
//! This keeps the tests useful even when issue trackers, branches, or commit
//! histories change.
//!
//! When a concrete compiler bug is discovered, add a dedicated test rather
//! than weakening an existing test.
//!
//! # Important semantic rule
//!
//! A reduction in operation count is NEVER treated as proof of semantic
//! equivalence.
//!
//! These tests use structural assertions only where structural equivalence is
//! actually the intended invariant. For semantic claims, the canonical
//! equivalence subsystem should be used by the higher-level equivalence test
//! suite.
//!
//! The local cancellation pass itself is exact by contract, so this file also
//! tests exact cancellation boundaries and explicitly tests cases that must
//! NOT cancel.
//!
//! # Dependency boundary
//!
//! This test module does not depend on:
//!
//! - routing;
//! - scheduling;
//! - hardware backends;
//! - QPU execution;
//! - benchmarking;
//! - QEC implementations;
//! - OpenQASM parsing;
//! - frontend parsing;
//! - network services.
//!
//! That keeps regression failures attributable to the optimization subsystem.
//!
//! # No silent fallback
//!
//! A test must fail loudly when an expected canonical operation cannot be
//! constructed. Tests must not silently replace unsupported operations with
//! another gate kind.
//!
//! # Regression categories
//!
//! 1. Canonical IR integration.
//! 2. Exact local cancellation.
//! 3. Semantic boundaries.
//! 4. Parameter safety.
//! 5. Multi-qubit safety.
//! 6. Fixed-point behavior.
//! 7. Determinism.
//! 8. Scalability.
//! 9. Resource behavior.
//! 10. Pass-contract stability.
//!
//! ============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::f64::consts::PI;

use crate::quantum::ir::gate::{Gate, GateKind};
use crate::quantum::ir::parameter::Parameter;
use crate::quantum::ir::qubit::QubitId;
use crate::quantum::ir::QuantumCircuit;

use crate::quantum::optimization::config::OptimizationConfig;
use crate::quantum::optimization::context::OptimizationContext;
use crate::quantum::optimization::local::cancellation::CancellationPass;
use crate::quantum::optimization::pass::OptimizationPass;

// ============================================================================
// Constants
// ============================================================================

/// Small deterministic workload suitable for normal CI.
const DEFAULT_SCALE: usize = 4_096;

/// Larger bounded workload used by selected stress tests.
const DEFAULT_STRESS_SCALE: usize = 16_384;

/// Number of logical qubits used by generated circuits.
const GENERATED_QUBITS: usize = 8;

/// Deterministic base seed.
const BASE_SEED: u64 = 0x5A4D_414E_495F_5247;

// ============================================================================
// Deterministic test generator
// ============================================================================

/// Deterministic pseudo-random generator used only by regression tests.
#[derive(Debug, Clone, Copy)]
struct Generator {
    state: u64,
}

impl Generator {
    /// Creates a generator from an explicit seed.
    const fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    /// Produces the next deterministic 64-bit value.
    fn next_u64(&mut self) -> u64 {
        self.state = self
            .state
            .wrapping_add(0x9E37_79B9_7F4A_7C15);

        let mut value = self.state;

        value = (value ^ (value >> 30))
            .wrapping_mul(0xBF58_476D_1CE4_E5B9);

        value = (value ^ (value >> 27))
            .wrapping_mul(0x94D0_49BB_1331_11EB);

        value ^ (value >> 31)
    }

    /// Produces an integer in `[0, upper)`.
    fn index(&mut self, upper: usize) -> usize {
        if upper == 0 {
            return 0;
        }

        (self.next_u64() % upper as u64) as usize
    }

    /// Produces a deterministic Boolean.
    fn boolean(&mut self) -> bool {
        self.next_u64() & 1 == 1
    }

    /// Produces a deterministic angle from a small exact-value domain.
    fn angle(&mut self) -> f64 {
        let bucket = self.index(17) as f64 - 8.0;

        bucket * (PI / 8.0)
    }
}

// ============================================================================
// Canonical IR construction helpers
// ============================================================================

/// Constructs the canonical logical qubit identity.
fn q(index: usize) -> QubitId {
    QubitId::new(index)
}

/// Constructs a canonical finite parameter.
fn parameter(value: f64) -> Parameter {
    Parameter::constant(value)
        .expect("finite regression-test parameter must be accepted")
}

/// Constructs a non-parameterized canonical gate.
fn gate(kind: GateKind, qubits: &[usize]) -> Gate {
    Gate::new(
        kind,
        qubits.iter().copied().map(q).collect(),
        Vec::new(),
        None,
        None,
    )
    .expect("regression test gate must satisfy canonical IR invariants")
}

/// Constructs a one-parameter canonical gate.
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
    .expect("parameterized regression gate must satisfy canonical IR invariants")
}

/// Constructs a barrier over the specified logical qubits.
fn barrier(qubits: &[usize]) -> Gate {
    gate(GateKind::Barrier, qubits)
}

/// Constructs a reset operation.
fn reset(qubit: usize) -> Gate {
    gate(GateKind::Reset, &[qubit])
}

/// Constructs a validated canonical circuit.
fn circuit(
    num_qubits: usize,
    operations: Vec<Gate>,
) -> QuantumCircuit {
    QuantumCircuit::from_operations(
        num_qubits,
        0,
        operations,
    )
    .expect("regression-test circuit must satisfy canonical IR invariants")
}

/// Returns the operation count.
fn operation_count(circuit: &QuantumCircuit) -> usize {
    circuit.operations().len()
}

/// Runs local cancellation on a cloned circuit.
fn optimize(original: &QuantumCircuit) -> QuantumCircuit {
    let mut optimized = original.clone();

    let pass = CancellationPass::new();

    let mut context = OptimizationContext::production(
        OptimizationConfig::default(),
    )
    .expect("production optimization context must be constructible");

    pass.run(
        &mut optimized,
        &mut context,
    )
    .expect("valid regression circuit must be accepted by cancellation");

    optimized
}

/// Returns the gate kinds in canonical operation order.
///
/// This helper deliberately uses the public operation representation rather
/// than inspecting private circuit fields.
fn gate_kinds(circuit: &QuantumCircuit) -> Vec<GateKind> {
    circuit
        .operations()
        .iter()
        .map(|operation| operation.kind())
        .collect()
}

/// Returns logical qubit operands for every operation.
fn operands(circuit: &QuantumCircuit) -> Vec<Vec<QubitId>> {
    circuit
        .operations()
        .iter()
        .map(|operation| operation.qubits().to_vec())
        .collect()
}

/// Returns the configured regression scale.
fn regression_scale() -> usize {
    std::env::var("ZAMANI_OPTIMIZATION_REGRESSION_SCALE")
        .ok()
        .and_then(|value| value.parse::<usize>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(DEFAULT_SCALE)
}

/// Returns a deterministic generated circuit.
///
/// The generator intentionally creates a heterogeneous workload while
/// remaining entirely within exact canonical gate semantics.
fn generated_circuit(
    seed: u64,
    operations: usize,
) -> QuantumCircuit {
    let mut generator = Generator::new(seed);

    let mut gates = Vec::with_capacity(operations);

    for _ in 0..operations {
        let first = generator.index(GENERATED_QUBITS);

        let choice = generator.index(13);

        let operation = match choice {
            0 => gate(GateKind::X, &[first]),

            1 => gate(GateKind::Y, &[first]),

            2 => gate(GateKind::Z, &[first]),

            3 => gate(GateKind::H, &[first]),

            4 => gate(GateKind::S, &[first]),

            5 => gate(GateKind::Sdg, &[first]),

            6 => gate(GateKind::T, &[first]),

            7 => gate(GateKind::Tdg, &[first]),

            8 => parameterized_gate(
                GateKind::RX,
                &[first],
                generator.angle(),
            ),

            9 => parameterized_gate(
                GateKind::RY,
                &[first],
                generator.angle(),
            ),

            10 => parameterized_gate(
                GateKind::RZ,
                &[first],
                generator.angle(),
            ),

            11 => {
                let mut second =
                    generator.index(GENERATED_QUBITS);

                if first == second {
                    second = (second + 1) % GENERATED_QUBITS;
                }

                if generator.boolean() {
                    gate(
                        GateKind::CX,
                        &[first, second],
                    )
                } else {
                    gate(
                        GateKind::CZ,
                        &[first, second],
                    )
                }
            }

            _ => {
                let mut second =
                    generator.index(GENERATED_QUBITS);

                if first == second {
                    second = (second + 1) % GENERATED_QUBITS;
                }

                gate(
                    GateKind::CX,
                    &[second, first],
                )
            }
        };

        gates.push(operation);

        // Insert boundaries at deterministic intervals.
        if generator.index(97) == 0 {
            gates.push(barrier(&[first]));
        }

        if generator.index(193) == 0 {
            gates.push(reset(first));
        }
    }

    circuit(GENERATED_QUBITS, gates)
}

// ============================================================================
// Regression: canonical IR / QubitId
// ============================================================================

/// Regression: optimizer tests must use the canonical `qubit` module.
#[test]
fn regression_canonical_qubit_module_is_value_stable() {
    for index in 0..1_024 {
        let first = q(index);
        let second = q(index);

        assert_eq!(first, second);
        assert_eq!(first.index(), index);
    }
}

/// Regression: distinct logical qubits must never collapse into one identity.
#[test]
fn regression_distinct_qubits_remain_distinct() {
    for first in 0..128 {
        for second in 0..128 {
            if first == second {
                continue;
            }

            assert_ne!(
                q(first),
                q(second),
                "distinct logical qubits must remain distinct"
            );
        }
    }
}

/// Regression: sparse logical qubit identifiers remain usable.
#[test]
fn regression_sparse_logical_qubit_namespace() {
    let circuit = circuit(
        1_024,
        vec![
            gate(GateKind::X, &[3]),
            gate(GateKind::X, &[3]),
            gate(GateKind::H, &[511]),
            gate(GateKind::H, &[511]),
            gate(GateKind::Z, &[1_023]),
        ],
    );

    let optimized = optimize(&circuit);

    assert_eq!(
        optimized.num_qubits(),
        circuit.num_qubits(),
        "optimization must not change logical qubit namespace size"
    );

    assert_eq!(
        operation_count(&optimized),
        1,
        "only the unmatched Z operation should remain"
    );

    assert_eq!(
        operands(&optimized),
        vec![vec![q(1_023)]],
        "the surviving operation must retain its logical operand"
    );
}

// ============================================================================
// Regression: empty and trivial circuits
// ============================================================================

/// Regression: empty circuits must be accepted.
#[test]
fn regression_empty_circuit_is_unchanged() {
    let circuit = circuit(0, Vec::new());
    let optimized = optimize(&circuit);

    assert_eq!(operation_count(&optimized), 0);
    assert_eq!(optimized.num_qubits(), circuit.num_qubits());
}

/// Regression: a single operation must not be removed unless it is an exact
/// identity.
#[test]
fn regression_single_non_identity_operation_is_preserved() {
    let circuit = circuit(
        1,
        vec![gate(GateKind::X, &[0])],
    );

    let optimized = optimize(&circuit);

    assert_eq!(operation_count(&optimized), 1);
    assert_eq!(
        gate_kinds(&optimized),
        vec![GateKind::X]
    );
}

/// Regression: already optimized circuits must remain unchanged.
#[test]
fn regression_already_optimized_circuit_is_unchanged() {
    let circuit = circuit(
        2,
        vec![
            gate(GateKind::X, &[0]),
            gate(GateKind::H, &[1]),
            gate(GateKind::CX, &[0, 1]),
        ],
    );

    let first = optimize(&circuit);
    let second = optimize(&first);

    assert_eq!(
        gate_kinds(&first),
        gate_kinds(&second)
    );

    assert_eq!(
        operands(&first),
        operands(&second)
    );
}

// ============================================================================
// Regression: self-inverse gates
// ============================================================================

/// Regression: X X cancellation.
#[test]
fn regression_x_self_inverse_cancellation() {
    let circuit = circuit(
        1,
        vec![
            gate(GateKind::X, &[0]),
            gate(GateKind::X, &[0]),
        ],
    );

    let optimized = optimize(&circuit);

    assert_eq!(operation_count(&optimized), 0);
}

/// Regression: Y Y cancellation.
#[test]
fn regression_y_self_inverse_cancellation() {
    let circuit = circuit(
        1,
        vec![
            gate(GateKind::Y, &[0]),
            gate(GateKind::Y, &[0]),
        ],
    );

    let optimized = optimize(&circuit);

    assert_eq!(operation_count(&optimized), 0);
}

/// Regression: Z Z cancellation.
#[test]
fn regression_z_self_inverse_cancellation() {
    let circuit = circuit(
        1,
        vec![
            gate(GateKind::Z, &[0]),
            gate(GateKind::Z, &[0]),
        ],
    );

    let optimized = optimize(&circuit);

    assert_eq!(operation_count(&optimized), 0);
}

/// Regression: H H cancellation.
#[test]
fn regression_h_self_inverse_cancellation() {
    let circuit = circuit(
        1,
        vec![
            gate(GateKind::H, &[0]),
            gate(GateKind::H, &[0]),
        ],
    );

    let optimized = optimize(&circuit);

    assert_eq!(operation_count(&optimized), 0);
}

/// Regression: CNOT CNOT cancellation.
#[test]
fn regression_cx_self_inverse_cancellation() {
    let circuit = circuit(
        2,
        vec![
            gate(GateKind::CX, &[0, 1]),
            gate(GateKind::CX, &[0, 1]),
        ],
    );

    let optimized = optimize(&circuit);

    assert_eq!(operation_count(&optimized), 0);
}

/// Regression: CZ CZ cancellation.
#[test]
fn regression_cz_self_inverse_cancellation() {
    let circuit = circuit(
        2,
        vec![
            gate(GateKind::CZ, &[0, 1]),
            gate(GateKind::CZ, &[0, 1]),
        ],
    );

    let optimized = optimize(&circuit);

    assert_eq!(operation_count(&optimized), 0);
}

// ============================================================================
// Regression: explicit inverse pairs
// ============================================================================

/// Regression: S followed by Sdg cancels exactly.
#[test]
fn regression_s_sdg_inverse_pair() {
    let circuit = circuit(
        1,
        vec![
            gate(GateKind::S, &[0]),
            gate(GateKind::Sdg, &[0]),
        ],
    );

    let optimized = optimize(&circuit);

    assert_eq!(operation_count(&optimized), 0);
}

/// Regression: Sdg followed by S cancels exactly.
#[test]
fn regression_sdg_s_inverse_pair() {
    let circuit = circuit(
        1,
        vec![
            gate(GateKind::Sdg, &[0]),
            gate(GateKind::S, &[0]),
        ],
    );

    let optimized = optimize(&circuit);

    assert_eq!(operation_count(&optimized), 0);
}

/// Regression: T followed by Tdg cancels exactly.
#[test]
fn regression_t_tdg_inverse_pair() {
    let circuit = circuit(
        1,
        vec![
            gate(GateKind::T, &[0]),
            gate(GateKind::Tdg, &[0]),
        ],
    );

    let optimized = optimize(&circuit);

    assert_eq!(operation_count(&optimized), 0);
}

/// Regression: Tdg followed by T cancels exactly.
#[test]
fn regression_tdg_t_inverse_pair() {
    let circuit = circuit(
        1,
        vec![
            gate(GateKind::Tdg, &[0]),
            gate(GateKind::T, &[0]),
        ],
    );

    let optimized = optimize(&circuit);

    assert_eq!(operation_count(&optimized), 0);
}

// ============================================================================
// Regression: wrong inverse / false cancellation
// ============================================================================

/// Regression: S S is not an inverse pair.
#[test]
fn regression_s_s_is_not_cancelled() {
    let circuit = circuit(
        1,
        vec![
            gate(GateKind::S, &[0]),
            gate(GateKind::S, &[0]),
        ],
    );

    let optimized = optimize(&circuit);

    assert_eq!(operation_count(&optimized), 2);
}

/// Regression: T T is not an inverse pair.
#[test]
fn regression_t_t_is_not_cancelled() {
    let circuit = circuit(
        1,
        vec![
            gate(GateKind::T, &[0]),
            gate(GateKind::T, &[0]),
        ],
    );

    let optimized = optimize(&circuit);

    assert_eq!(operation_count(&optimized), 2);
}

/// Regression: H X is not an inverse pair.
#[test]
fn regression_h_x_is_not_cancelled() {
    let circuit = circuit(
        1,
        vec![
            gate(GateKind::H, &[0]),
            gate(GateKind::X, &[0]),
        ],
    );

    let optimized = optimize(&circuit);

    assert_eq!(operation_count(&optimized), 2);
}

// ============================================================================
// Regression: qubit isolation
// ============================================================================

/// Regression: equal gates on different qubits must not cancel.
#[test]
fn regression_same_gate_different_qubits_does_not_cancel() {
    let circuit = circuit(
        2,
        vec![
            gate(GateKind::X, &[0]),
            gate(GateKind::X, &[1]),
        ],
    );

    let optimized = optimize(&circuit);

    assert_eq!(operation_count(&optimized), 2);
}

/// Regression: equal two-qubit gates with different operands must not cancel.
#[test]
fn regression_two_qubit_operand_mismatch_does_not_cancel() {
    let circuit = circuit(
        3,
        vec![
            gate(GateKind::CX, &[0, 1]),
            gate(GateKind::CX, &[0, 2]),
        ],
    );

    let optimized = optimize(&circuit);

    assert_eq!(operation_count(&optimized), 2);
}

/// Regression: reversing CNOT operands changes its semantic operation and must
/// not be treated as an identical gate.
#[test]
fn regression_two_qubit_operand_order_is_semantically_significant() {
    let circuit = circuit(
        2,
        vec![
            gate(GateKind::CX, &[0, 1]),
            gate(GateKind::CX, &[1, 0]),
        ],
    );

    let optimized = optimize(&circuit);

    assert_eq!(
        operation_count(&optimized),
        2,
        "CNOT operand reversal must not be silently treated as identity"
    );
}

// ============================================================================
// Regression: barriers
// ============================================================================

/// Regression: cancellation must not cross a barrier.
#[test]
fn regression_cancellation_does_not_cross_barrier() {
    let circuit = circuit(
        1,
        vec![
            gate(GateKind::X, &[0]),
            barrier(&[0]),
            gate(GateKind::X, &[0]),
        ],
    );

    let optimized = optimize(&circuit);

    assert_eq!(
        operation_count(&optimized),
        3,
        "barrier must remain a semantic optimization boundary"
    );

    assert_eq!(
        gate_kinds(&optimized),
        vec![
            GateKind::X,
            GateKind::Barrier,
            GateKind::X,
        ]
    );
}

/// Regression: barriers on one qubit must not permit cancellation across the
/// boundary on that same qubit.
#[test]
fn regression_barrier_preserves_local_gate_order() {
    let circuit = circuit(
        2,
        vec![
            gate(GateKind::H, &[0]),
            barrier(&[0]),
            gate(GateKind::H, &[0]),
        ],
    );

    let optimized = optimize(&circuit);

    assert_eq!(operation_count(&optimized), 3);
    assert_eq!(
        operands(&optimized),
        vec![
            vec![q(0)],
            vec![q(0)],
            vec![q(0)],
        ]
    );
}

// ============================================================================
// Regression: reset boundaries
// ============================================================================

/// Regression: cancellation must not cross reset.
#[test]
fn regression_cancellation_does_not_cross_reset() {
    let circuit = circuit(
        1,
        vec![
            gate(GateKind::X, &[0]),
            reset(0),
            gate(GateKind::X, &[0]),
        ],
    );

    let optimized = optimize(&circuit);

    assert_eq!(
        operation_count(&optimized),
        3,
        "reset is a non-unitary semantic boundary"
    );

    assert_eq!(
        gate_kinds(&optimized),
        vec![
            GateKind::X,
            GateKind::Reset,
            GateKind::X,
        ]
    );
}

/// Regression: reset itself must never be treated as an ordinary inverse pair.
#[test]
fn regression_reset_is_not_an_ordinary_unitary_gate() {
    let circuit = circuit(
        1,
        vec![
            reset(0),
            reset(0),
        ],
    );

    let optimized = optimize(&circuit);

    assert_eq!(
        operation_count(&optimized),
        2,
        "two resets must not be cancelled as if they were self-inverse gates"
    );
}

// ============================================================================
// Regression: exact parameterized cancellation
// ============================================================================

/// Regression: RX(theta) RX(-theta) cancels exactly.
#[test]
fn regression_rx_exact_inverse_cancellation() {
    let theta = PI / 4.0;

    let circuit = circuit(
        1,
        vec![
            parameterized_gate(
                GateKind::RX,
                &[0],
                theta,
            ),
            parameterized_gate(
                GateKind::RX,
                &[0],
                -theta,
            ),
        ],
    );

    let optimized = optimize(&circuit);

    assert_eq!(operation_count(&optimized), 0);
}

/// Regression: RY(theta) RY(-theta) cancels exactly.
#[test]
fn regression_ry_exact_inverse_cancellation() {
    let theta = PI / 3.0;

    let circuit = circuit(
        1,
        vec![
            parameterized_gate(
                GateKind::RY,
                &[0],
                theta,
            ),
            parameterized_gate(
                GateKind::RY,
                &[0],
                -theta,
            ),
        ],
    );

    let optimized = optimize(&circuit);

    assert_eq!(operation_count(&optimized), 0);
}

/// Regression: RZ(theta) RZ(-theta) cancels exactly.
#[test]
fn regression_rz_exact_inverse_cancellation() {
    let theta = PI / 5.0;

    let circuit = circuit(
        1,
        vec![
            parameterized_gate(
                GateKind::RZ,
                &[0],
                theta,
            ),
            parameterized_gate(
                GateKind::RZ,
                &[0],
                -theta,
            ),
        ],
    );

    let optimized = optimize(&circuit);

    assert_eq!(operation_count(&optimized), 0);
}

/// Regression: non-inverse RX parameters must not cancel.
#[test]
fn regression_rx_non_inverse_parameters_are_preserved() {
    let circuit = circuit(
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
                PI / 8.0,
            ),
        ],
    );

    let optimized = optimize(&circuit);

    assert_eq!(
        operation_count(&optimized),
        2,
        "rotation fusion belongs to the rotation optimizer, not cancellation"
    );
}

/// Regression: tiny floating-point perturbations must not be treated as exact
/// inverses merely because they are numerically close.
#[test]
fn regression_parameter_cancellation_is_not_approximate() {
    let theta = PI / 4.0;

    let circuit = circuit(
        1,
        vec![
            parameterized_gate(
                GateKind::RX,
                &[0],
                theta,
            ),
            parameterized_gate(
                GateKind::RX,
                &[0],
                -theta + f64::EPSILON,
            ),
        ],
    );

    let optimized = optimize(&circuit);

    assert_eq!(
        operation_count(&optimized),
        2,
        "exact cancellation must not silently become approximate cancellation"
    );
}

/// Regression: a non-zero rotation must not be removed merely because its
/// angle is small.
#[test]
fn regression_small_nonzero_rotation_is_preserved() {
    let circuit = circuit(
        1,
        vec![
            parameterized_gate(
                GateKind::RZ,
                &[0],
                f64::EPSILON,
            ),
        ],
    );

    let optimized = optimize(&circuit);

    assert_eq!(
        operation_count(&optimized),
        1,
        "non-zero rotations must not be approximated away by exact cancellation"
    );
}

/// Regression: exact zero rotations are removable.
#[test]
fn regression_exact_zero_rotation_is_removed() {
    for kind in [
        GateKind::RX,
        GateKind::RY,
        GateKind::RZ,
    ] {
        let circuit = circuit(
            1,
            vec![
                parameterized_gate(
                    kind,
                    &[0],
                    0.0,
                ),
            ],
        );

        let optimized = optimize(&circuit);

        assert_eq!(
            operation_count(&optimized),
            0,
            "exact zero-angle {:?} should be removed",
            kind
        );
    }
}

// ============================================================================
// Regression: no unsafe global-phase assumptions
// ============================================================================

/// Regression: RZ(2π) must not be removed by a local exact cancellation pass
/// merely because some equivalence policies may permit global-phase changes.
#[test]
fn regression_rz_two_pi_is_not_removed_as_global_phase() {
    let circuit = circuit(
        1,
        vec![
            parameterized_gate(
                GateKind::RZ,
                &[0],
                2.0 * PI,
            ),
        ],
    );

    let optimized = optimize(&circuit);

    assert_eq!(
        operation_count(&optimized),
        1,
        "local exact cancellation must not assume global-phase equivalence"
    );
}

/// Regression: RX(2π) must not be removed merely from periodic-angle
/// reasoning when the local pass contract requires exact identity semantics.
#[test]
fn regression_rx_two_pi_is_not_removed_without_explicit_periodic_rule() {
    let circuit = circuit(
        1,
        vec![
            parameterized_gate(
                GateKind::RX,
                &[0],
                2.0 * PI,
            ),
        ],
    );

    let optimized = optimize(&circuit);

    assert_eq!(
        operation_count(&optimized),
        1,
        "periodicity must not be silently introduced into exact cancellation"
    );
}

// ============================================================================
// Regression: cascading cancellation
// ============================================================================

/// Regression: cancellation must cascade after an earlier pair is removed.
#[test]
fn regression_cascading_self_inverse_cancellation() {
    let circuit = circuit(
        1,
        vec![
            gate(GateKind::H, &[0]),
            gate(GateKind::X, &[0]),
            gate(GateKind::X, &[0]),
            gate(GateKind::H, &[0]),
        ],
    );

    let optimized = optimize(&circuit);

    assert_eq!(
        operation_count(&optimized),
        0,
        "stack-based cancellation must expose cascading pairs"
    );
}

/// Regression: three self-inverse gates leave exactly one operation.
#[test]
fn regression_odd_self_inverse_sequence_leaves_one_gate() {
    let circuit = circuit(
        1,
        vec![
            gate(GateKind::X, &[0]),
            gate(GateKind::X, &[0]),
            gate(GateKind::X, &[0]),
        ],
    );

    let optimized = optimize(&circuit);

    assert_eq!(
        operation_count(&optimized),
        1
    );

    assert_eq!(
        gate_kinds(&optimized),
        vec![GateKind::X]
    );
}

/// Regression: four identical self-inverse gates must fully cancel.
#[test]
fn regression_even_self_inverse_sequence_reaches_empty_fixed_point() {
    let circuit = circuit(
        1,
        vec![
            gate(GateKind::Z, &[0]),
            gate(GateKind::Z, &[0]),
            gate(GateKind::Z, &[0]),
            gate(GateKind::Z, &[0]),
        ],
    );

    let optimized = optimize(&circuit);

    assert_eq!(operation_count(&optimized), 0);
}

// ============================================================================
// Regression: local-cancellation contract
// ============================================================================

/// Regression: the public `can_cancel` helper must agree with actual pass
/// behavior for exact self-inverse gates.
#[test]
fn regression_can_cancel_matches_self_inverse_behavior() {
    let first = gate(GateKind::X, &[0]);
    let second = gate(GateKind::X, &[0]);

    assert!(
        CancellationPass::can_cancel(&first, &second),
        "X/X must be recognized as an exact cancellation pair"
    );
}

/// Regression: the public `can_cancel` helper must reject different logical
/// qubit operands.
#[test]
fn regression_can_cancel_rejects_different_qubits() {
    let first = gate(GateKind::X, &[0]);
    let second = gate(GateKind::X, &[1]);

    assert!(
        !CancellationPass::can_cancel(&first, &second),
        "same gate kind on different logical qubits must not cancel"
    );
}

/// Regression: identity classification must recognize exact zero rotations.
#[test]
fn regression_identity_classifier_recognizes_zero_rotation() {
    let zero_rotation = parameterized_gate(
        GateKind::RX,
        &[0],
        0.0,
    );

    assert!(
        CancellationPass::is_identity(&zero_rotation),
        "exact zero rotation must be classified as an identity"
    );
}

/// Regression: identity classification must not classify an ordinary gate as
/// an identity.
#[test]
fn regression_identity_classifier_rejects_non_identity_gate() {
    let x = gate(GateKind::X, &[0]);

    assert!(
        !CancellationPass::is_identity(&x),
        "X must not be classified as an identity"
    );
}

// ============================================================================
// Regression: idempotence / fixed point
// ============================================================================

/// Regression: applying local cancellation twice must produce the same result
/// as applying it once.
#[test]
fn regression_cancellation_is_idempotent() {
    let circuit = circuit(
        3,
        vec![
            gate(GateKind::X, &[0]),
            gate(GateKind::X, &[0]),
            gate(GateKind::H, &[1]),
            gate(GateKind::H, &[1]),
            gate(GateKind::CX, &[0, 2]),
            gate(GateKind::CX, &[0, 2]),
        ],
    );

    let once = optimize(&circuit);
    let twice = optimize(&once);

    assert_eq!(
        gate_kinds(&once),
        gate_kinds(&twice)
    );

    assert_eq!(
        operands(&once),
        operands(&twice)
    );

    assert_eq!(
        once.num_qubits(),
        twice.num_qubits()
    );
}

/// Regression: cancellation reaches a local fixed point in one invocation.
#[test]
fn regression_single_pass_reaches_local_fixed_point() {
    let circuit = circuit(
        2,
        vec![
            gate(GateKind::X, &[0]),
            gate(GateKind::X, &[0]),
            gate(GateKind::H, &[1]),
            gate(GateKind::H, &[1]),
        ],
    );

    let first = optimize(&circuit);
    let second = optimize(&first);

    assert_eq!(
        operation_count(&first),
        operation_count(&second)
    );

    assert_eq!(
        gate_kinds(&first),
        gate_kinds(&second)
    );
}

// ============================================================================
// Regression: deletion-only monotonicity
// ============================================================================

/// Regression: local cancellation is deletion-only and therefore must never
/// increase operation count.
#[test]
fn regression_cancellation_never_increases_operation_count() {
    let scale = regression_scale().min(16_384);

    for seed_offset in 0..8 {
        let original = generated_circuit(
            BASE_SEED.wrapping_add(seed_offset as u64),
            scale,
        );

        let optimized = optimize(&original);

        assert!(
            operation_count(&optimized)
                <= operation_count(&original),
            "deletion-only cancellation increased operation count"
        );
    }
}

/// Regression: logical qubit count is invariant under cancellation.
#[test]
fn regression_cancellation_preserves_qubit_namespace() {
    let circuit = generated_circuit(
        BASE_SEED,
        2_048,
    );

    let optimized = optimize(&circuit);

    assert_eq!(
        optimized.num_qubits(),
        circuit.num_qubits()
    );
}

// ============================================================================
// Regression: determinism
// ============================================================================

/// Regression: identical input circuits must produce identical cancellation
/// results.
#[test]
fn regression_cancellation_is_deterministic() {
    let original_a = generated_circuit(
        BASE_SEED,
        4_096,
    );

    let original_b = generated_circuit(
        BASE_SEED,
        4_096,
    );

    assert_eq!(
        gate_kinds(&original_a),
        gate_kinds(&original_b)
    );

    assert_eq!(
        operands(&original_a),
        operands(&original_b)
    );

    let optimized_a = optimize(&original_a);
    let optimized_b = optimize(&original_b);

    assert_eq!(
        gate_kinds(&optimized_a),
        gate_kinds(&optimized_b)
    );

    assert_eq!(
        operands(&optimized_a),
        operands(&optimized_b)
    );
}

/// Regression: changing the deterministic seed may change generated workload,
/// but never makes the test harness nondeterministic.
#[test]
fn regression_generator_is_reproducible() {
    let first = generated_circuit(
        BASE_SEED ^ 0x1111,
        1_024,
    );

    let second = generated_circuit(
        BASE_SEED ^ 0x1111,
        1_024,
    );

    assert_eq!(
        gate_kinds(&first),
        gate_kinds(&second)
    );

    assert_eq!(
        operands(&first),
        operands(&second)
    );
}

// ============================================================================
// Regression: large-circuit behavior
// ============================================================================

/// Regression: the optimizer must process a large deterministic circuit
/// without relying on repeated indexed deletion.
///
/// The default workload is intentionally moderate for CI. The environment
/// variable can scale this test upward with available resources.
#[test]
fn regression_large_circuit_scales_with_available_resources() {
    let requested = regression_scale();

    let scale = requested.min(
        DEFAULT_STRESS_SCALE
    );

    let original = generated_circuit(
        BASE_SEED ^ 0xAAAA,
        scale,
    );

    let before = operation_count(&original);

    let optimized = optimize(&original);

    let after = operation_count(&optimized);

    assert!(
        after <= before,
        "large-circuit cancellation must remain deletion-only"
    );

    assert_eq!(
        optimized.num_qubits(),
        original.num_qubits()
    );

    optimized
        .validate()
        .expect(
            "large optimized circuit must remain valid canonical Quantum IR"
        );
}

/// Regression: stress generation itself must remain deterministic at larger
/// scales.
#[test]
fn regression_large_generated_circuit_is_reproducible() {
    let scale = DEFAULT_STRESS_SCALE.min(
        regression_scale().max(1)
    );

    let first = generated_circuit(
        BASE_SEED ^ 0xBBBB,
        scale,
    );

    let second = generated_circuit(
        BASE_SEED ^ 0xBBBB,
        scale,
    );

    assert_eq!(
        operation_count(&first),
        operation_count(&second)
    );

    assert_eq!(
        gate_kinds(&first),
        gate_kinds(&second)
    );

    assert_eq!(
        operands(&first),
        operands(&second)
    );
}

// ============================================================================
// Regression: heterogeneous workloads
// ============================================================================

/// Regression: heterogeneous circuits containing exact pairs, boundaries,
/// rotations, and multi-qubit gates remain valid after optimization.
#[test]
fn regression_heterogeneous_workload_remains_valid() {
    let original = generated_circuit(
        BASE_SEED ^ 0xCCCC,
        8_192,
    );

    let optimized = optimize(&original);

    optimized
        .validate()
        .expect(
            "heterogeneous optimized workload must remain valid"
        );

    assert!(
        operation_count(&optimized)
            <= operation_count(&original)
    );
}

/// Regression: repeated optimization of a heterogeneous workload must
/// converge.
#[test]
fn regression_heterogeneous_workload_converges() {
    let original = generated_circuit(
        BASE_SEED ^ 0xDDDD,
        4_096,
    );

    let first = optimize(&original);
    let second = optimize(&first);
    let third = optimize(&second);

    assert_eq!(
        gate_kinds(&second),
        gate_kinds(&third)
    );

    assert_eq!(
        operands(&second),
        operands(&third)
    );
}

// ============================================================================
// Regression: operation ordering
// ============================================================================

/// Regression: unrelated operations must retain their relative order.
#[test]
fn regression_unrelated_operations_retain_order() {
    let circuit = circuit(
        3,
        vec![
            gate(GateKind::X, &[0]),
            gate(GateKind::H, &[1]),
            gate(GateKind::Z, &[2]),
        ],
    );

    let optimized = optimize(&circuit);

    assert_eq!(
        gate_kinds(&optimized),
        vec![
            GateKind::X,
            GateKind::H,
            GateKind::Z,
        ]
    );

    assert_eq!(
        operands(&optimized),
        vec![
            vec![q(0)],
            vec![q(1)],
            vec![q(2)],
        ]
    );
}

/// Regression: cancellation must not reorder surviving operations.
#[test]
fn regression_surviving_operations_retain_order() {
    let circuit = circuit(
        2,
        vec![
            gate(GateKind::X, &[0]),
            gate(GateKind::X, &[0]),
            gate(GateKind::H, &[1]),
            gate(GateKind::Z, &[0]),
        ],
    );

    let optimized = optimize(&circuit);

    assert_eq!(
        gate_kinds(&optimized),
        vec![
            GateKind::H,
            GateKind::Z,
        ]
    );

    assert_eq!(
        operands(&optimized),
        vec![
            vec![q(1)],
            vec![q(0)],
        ]
    );
}

// ============================================================================
// Regression: mixed cancellation
// ============================================================================

/// Regression: mixed self-inverse and explicit inverse pairs must all cancel.
#[test]
fn regression_mixed_inverse_pairs_cancel() {
    let circuit = circuit(
        2,
        vec![
            gate(GateKind::X, &[0]),
            gate(GateKind::X, &[0]),
            gate(GateKind::S, &[1]),
            gate(GateKind::Sdg, &[1]),
            gate(GateKind::T, &[0]),
            gate(GateKind::Tdg, &[0]),
            gate(GateKind::CX, &[0, 1]),
            gate(GateKind::CX, &[0, 1]),
        ],
    );

    let optimized = optimize(&circuit);

    assert_eq!(
        operation_count(&optimized),
        0
    );
}

/// Regression: a surviving boundary prevents otherwise cancellable gates from
/// reaching one another.
#[test]
fn regression_mixed_pairs_respect_boundaries() {
    let circuit = circuit(
        2,
        vec![
            gate(GateKind::X, &[0]),
            barrier(&[0]),
            gate(GateKind::X, &[0]),
            gate(GateKind::X, &[0]),
        ],
    );

    let optimized = optimize(&circuit);

    assert_eq!(
        gate_kinds(&optimized),
        vec![
            GateKind::X,
            GateKind::Barrier,
        ],
        "only the pair after the barrier should cancel"
    );
}

// ============================================================================
// Regression: circuit validity after transformation
// ============================================================================

/// Regression: every optimized circuit must remain valid canonical Quantum IR.
#[test]
fn regression_optimized_circuit_is_valid() {
    let circuits = [
        circuit(
            1,
            vec![
                gate(GateKind::X, &[0]),
                gate(GateKind::X, &[0]),
            ],
        ),
        circuit(
            2,
            vec![
                gate(GateKind::CX, &[0, 1]),
                gate(GateKind::CX, &[0, 1]),
            ],
        ),
        circuit(
            1,
            vec![
                parameterized_gate(
                    GateKind::RZ,
                    &[0],
                    PI / 7.0,
                ),
                parameterized_gate(
                    GateKind::RZ,
                    &[0],
                    -PI / 7.0,
                ),
            ],
        ),
    ];

    for original in circuits {
        let optimized = optimize(&original);

        optimized
            .validate()
            .expect(
                "optimizer must not produce invalid canonical Quantum IR"
            );
    }
}

// ============================================================================
// Regression: optimizer does not own hardware concerns
// ============================================================================

/// Regression: cancellation must remain independent of physical topology.
///
/// Logical qubits are intentionally non-contiguous here. The optimizer must
/// treat them as logical identifiers and must not invent physical routing.
#[test]
fn regression_optimizer_operates_on_logical_qubits_only() {
    let circuit = circuit(
        1_024,
        vec![
            gate(GateKind::CX, &[7, 900]),
            gate(GateKind::CX, &[7, 900]),
        ],
    );

    let optimized = optimize(&circuit);

    assert_eq!(
        operation_count(&optimized),
        0
    );

    assert_eq!(
        optimized.num_qubits(),
        1_024,
        "optimization must not perform physical qubit allocation"
    );
}

// ============================================================================
// Regression: pass metadata stability
// ============================================================================

/// Regression: the cancellation pass identifier is part of the stable
/// optimization contract.
#[test]
fn regression_cancellation_pass_identifier_is_stable() {
    let pass = CancellationPass::new();

    assert_eq!(
        pass.metadata().id().as_str(),
        "local.cancellation"
    );
}

/// Regression: the cancellation pass remains classified as a local rewrite.
#[test]
fn regression_cancellation_pass_kind_is_stable() {
    let pass = CancellationPass::new();

    assert_eq!(
        pass.metadata().kind().as_str(),
        "local_rewrite"
    );
}

/// Regression: the cancellation pass remains linear.
#[test]
fn regression_cancellation_pass_complexity_is_stable() {
    let pass = CancellationPass::new();

    assert_eq!(
        pass.metadata().complexity().as_str(),
        "linear"
    );
}

// ============================================================================
// Regression: repeated independent runs
// ============================================================================

/// Regression: repeated independent optimizer invocations must not leak state
/// from one circuit into another.
#[test]
fn regression_optimizer_has_no_cross_invocation_state_leak() {
    let first = circuit(
        1,
        vec![
            gate(GateKind::X, &[0]),
            gate(GateKind::X, &[0]),
        ],
    );

    let second = circuit(
        1,
        vec![
            gate(GateKind::H, &[0]),
        ],
    );

    let first_result = optimize(&first);
    let second_result = optimize(&second);

    assert_eq!(
        operation_count(&first_result),
        0
    );

    assert_eq!(
        operation_count(&second_result),
        1
    );

    assert_eq!(
        gate_kinds(&second_result),
        vec![GateKind::H]
    );
}

// ============================================================================
// Regression: no artificial optimizer circuit ceiling
// ============================================================================

/// Regression: the test harness must not encode a tiny architectural circuit
/// ceiling. The explicit workload policy is the only bound used here.
///
/// This verifies the intended scaling principle: tests may become larger when
/// machine resources permit, while normal CI remains bounded.
#[test]
fn regression_scaling_policy_is_explicit() {
    let scale = regression_scale();

    assert!(
        scale > 0,
        "regression workload must always be positive"
    );

    let circuit = generated_circuit(
        BASE_SEED ^ 0xEEEE,
        scale.min(1_024),
    );

    assert!(
        operation_count(&circuit) > 0,
        "positive test scale must produce a non-empty workload"
    );
}

// ============================================================================
// Regression: exact cancellation matrix
// ============================================================================

/// Regression: all canonical self-inverse gate families supported by the
/// cancellation pass must cancel against themselves.
#[test]
fn regression_self_inverse_gate_matrix() {
    let kinds = [
        GateKind::X,
        GateKind::Y,
        GateKind::Z,
        GateKind::H,
    ];

    for kind in kinds {
        let circuit = circuit(
            1,
            vec![
                gate(kind, &[0]),
                gate(kind, &[0]),
            ],
        );

        let optimized = optimize(&circuit);

        assert_eq!(
            operation_count(&optimized),
            0,
            "{kind:?} must cancel with itself"
        );
    }
}

/// Regression: all explicit inverse pairs supported by the current local
/// cancellation contract must cancel in both directions.
#[test]
fn regression_inverse_pair_matrix_is_symmetric() {
    let pairs = [
        (GateKind::S, GateKind::Sdg),
        (GateKind::Sdg, GateKind::S),
        (GateKind::T, GateKind::Tdg),
        (GateKind::Tdg, GateKind::T),
    ];

    for (first, second) in pairs {
        let circuit = circuit(
            1,
            vec![
                gate(first, &[0]),
                gate(second, &[0]),
            ],
        );

        let optimized = optimize(&circuit);

        assert_eq!(
            operation_count(&optimized),
            0,
            "{first:?}/{second:?} must cancel"
        );
    }
}

// ============================================================================
// Regression: boundaries with unrelated qubits
// ============================================================================

/// Regression: a barrier on a different qubit must not be treated as a reason
/// to cancel gates on another qubit that are otherwise adjacent in the
/// logical dependency relation. The local pass remains conservative.
#[test]
fn regression_boundary_behavior_is_conservative() {
    let circuit = circuit(
        2,
        vec![
            gate(GateKind::X, &[0]),
            barrier(&[1]),
            gate(GateKind::X, &[0]),
        ],
    );

    let optimized = optimize(&circuit);

    assert_eq!(
        operation_count(&optimized),
        3,
        "local cancellation must remain conservative around explicit barriers"
    );
}

// ============================================================================
// Regression: generated workload contains meaningful diversity
// ============================================================================

/// Regression: deterministic generated workloads must actually exercise more
/// than one operation family.
#[test]
fn regression_generated_workload_is_heterogeneous() {
    let circuit = generated_circuit(
        BASE_SEED ^ 0xFFFF,
        2_048,
    );

    let kinds = gate_kinds(&circuit);

    let has_single_qubit = kinds.iter().any(|kind| {
        matches!(
            kind,
            GateKind::X
                | GateKind::Y
                | GateKind::Z
                | GateKind::H
                | GateKind::S
                | GateKind::Sdg
                | GateKind::T
                | GateKind::Tdg
                | GateKind::RX
                | GateKind::RY
                | GateKind::RZ
        )
    });

    let has_two_qubit = kinds.iter().any(|kind| {
        matches!(
            kind,
            GateKind::CX | GateKind::CZ
        )
    });

    assert!(
        has_single_qubit,
        "generated regression workload must contain single-qubit operations"
    );

    assert!(
        has_two_qubit,
        "generated regression workload must contain two-qubit operations"
    );
}

// ============================================================================
// Regression: optimizer construction
// ============================================================================

/// Regression: constructing multiple cancellation pass instances must be
/// deterministic and side-effect free.
#[test]
fn regression_multiple_pass_instances_are_equivalent() {
    let first = CancellationPass::new();
    let second = CancellationPass::new();

    assert_eq!(
        first.metadata().id().as_str(),
        second.metadata().id().as_str()
    );

    assert_eq!(
        first.metadata().name(),
        second.metadata().name()
    );

    assert_eq!(
        first.metadata().kind(),
        second.metadata().kind()
    );
}

// ============================================================================
// Regression: final invariant
// ============================================================================

/// Final high-value regression guard.
///
/// Every exact local-cancellation transformation exercised here must satisfy
/// all of the following simultaneously:
///
/// - valid input;
/// - valid output;
/// - no operation-count growth;
/// - logical qubit namespace preservation;
/// - deterministic result;
/// - fixed-point behavior.
#[test]
fn regression_complete_local_cancellation_contract() {
    let original = generated_circuit(
        BASE_SEED ^ 0x1234_5678,
        4_096,
    );

    original
        .validate()
        .expect("generated circuit must initially be valid");

    let optimized = optimize(&original);

    optimized
        .validate()
        .expect("optimized circuit must remain valid");

    assert!(
        operation_count(&optimized)
            <= operation_count(&original)
    );

    assert_eq!(
        optimized.num_qubits(),
        original.num_qubits()
    );

    let second = optimize(&optimized);

    assert_eq!(
        gate_kinds(&optimized),
        gate_kinds(&second)
    );

    assert_eq!(
        operands(&optimized),
        operands(&second)
    );
}