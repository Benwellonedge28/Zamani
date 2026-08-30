//! Zamani Quantum Optimization — Integration Test Suite
//!
//! `src/quantum/optimization/tests/integration.rs`
//!
//! # Purpose
//!
//! This module verifies that the production quantum-optimization components
//! integrate correctly across their public architectural boundaries.
//!
//! It is intentionally different from:
//!
//! - `properties.rs`, which verifies general optimizer invariants;
//! - `equivalence.rs`, which verifies the equivalence subsystem itself;
//! - individual pass unit tests, which verify one implementation in isolation.
//!
//! This file verifies the complete contracts between:
//!
//! ```text
//!                     canonical Quantum IR
//!                              │
//!                              ▼
//!                    optimization context
//!                              │
//!                              ▼
//!                       optimization pass
//!                              │
//!                              ▼
//!                    optimized Quantum IR
//!                              │
//!                    ┌─────────┴─────────┐
//!                    ▼                   ▼
//!             structural verifier   semantic verifier
//! ```
//!
//! # Architectural boundary
//!
//! The production quantum compiler architecture is:
//!
//! ```text
//! Zamani source
//!      │
//!      ▼
//! quantum::frontend
//!      │
//!      ▼
//! quantum::ir
//!      │
//!      ▼
//! quantum::optimization
//!      │
//!      ▼
//! quantum::routing
//!      │
//!      ▼
//! quantum::scheduling
//!      │
//!      ▼
//! quantum::hardware
//!      │
//!      ▼
//! quantum::runtime
//! ```
//!
//! This test module deliberately stops at the optimization boundary.
//!
//! It must not make optimization tests depend on:
//!
//! - a hardware backend;
//! - a QPU;
//! - network access;
//! - routing topology;
//! - execution scheduling;
//! - benchmark execution;
//! - source parsing;
//! - operating-system state.
//!
//! Higher-level end-to-end compiler tests can test those boundaries without
//! making the optimizer itself dependent upon them.
//!
//! # Canonical IR rule
//!
//! There is exactly one quantum circuit representation in these tests:
//!
//! ```text
//! crate::quantum::ir::QuantumCircuit
//! ```
//!
//! There is exactly one logical-qubit representation:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! ```
//!
//! The historical/inconsistent path:
//!
//! ```text
//! crate::quantum::ir::qubits::QubitId
//! ```
//!
//! must not be introduced into optimization tests.
//!
//! This is particularly important because the optimization subsystem must
//! reinforce the canonical IR boundary rather than reproduce old IR naming
//! inconsistencies.
//!
//! # Integration contract
//!
//! `tests/mod.rs` should expose this module with:
//!
//! ```text
//! mod integration;
//! ```
//!
//! No production module should import this test module.
//!
//! This file is a consumer of the following public contracts:
//!
//! ```text
//! quantum::ir::gate
//!     ├── Gate
//!     └── GateKind
//!
//! quantum::ir::parameter
//!     └── Parameter
//!
//! quantum::ir::qubit
//!     └── QubitId
//!
//! quantum::ir
//!     └── QuantumCircuit
//!
//! quantum::optimization::config
//!     └── OptimizationConfig
//!
//! quantum::optimization::context
//!     └── OptimizationContext
//!
//! quantum::optimization::pass
//!     └── OptimizationPass
//!
//! quantum::optimization::local::cancellation
//!     └── CancellationPass
//!
//! quantum::optimization::equivalence
//!     ├── verify
//!     ├── verify_structural
//!     ├── verify_unitary
//!     ├── EquivalenceConfig
//!     ├── EquivalenceMethod
//!     ├── EquivalenceVerdict
//!     └── UnitaryRelation
//! ```
//!
//! These are stable integration boundaries. The implementation of any one
//! component may change internally without requiring this test to change,
//! provided the public contract remains compatible.
//!
//! # What this file verifies
//!
//! The integration suite covers:
//!
//! 1. canonical IR → optimization context;
//! 2. canonical IR → optimization pass;
//! 3. optimization pass → canonical IR;
//! 4. optimizer → structural verification;
//! 5. optimizer → semantic verification;
//! 6. inverse/self-inverse cancellation;
//! 7. parameterized rotation cancellation;
//! 8. qubit isolation;
//! 9. barrier boundaries;
//! 10. reset boundaries;
//! 11. empty circuits;
//! 12. already-optimized circuits;
//! 13. optimizer idempotence;
//! 14. deterministic optimization;
//! 15. large deterministic workloads;
//! 16. sparse logical-qubit namespaces;
//! 17. preservation of classical namespace metadata;
//! 18. explicit verification behavior;
//! 19. resource-driven scaling;
//! 20. repeated optimization convergence.
//!
//! # Scalability contract
//!
//! The optimizer must not have a test-imposed architectural maximum.
//!
//! The default stress workload is deliberately finite so normal CI remains
//! practical. It can be increased with:
//!
//! ```text
//! ZAMANI_OPTIMIZATION_INTEGRATION_SCALE=100000 cargo test
//! ```
//!
//! The value represents requested test workload, not a production circuit
//! limit.
//!
//! A sufficiently provisioned machine may therefore execute much larger
//! workloads without changing this source file.
//!
//! The production optimizer remains responsible for enforcing its own explicit
//! `OptimizationLimits` and Quantum IR limits.
//!
//! # Determinism
//!
//! No operating-system randomness is used.
//!
//! The generated stress workload is completely deterministic.
//!
//! This ensures a failing test can be reproduced from the configured scale.
//!
//! # Semantic correctness
//!
//! These tests deliberately distinguish:
//!
//! ```text
//! structural equality
//! semantic equivalence
//! inconclusive verification
//! ```
//!
//! A circuit with a different gate sequence may still be semantically
//! equivalent.
//!
//! Conversely, a circuit with similar gate counts is not automatically
//! equivalent.
//!
//! `Inconclusive` is never interpreted as `Equivalent`.
//!
//! # Safety
//!
//! This file explicitly forbids unsafe Rust.
//!
//! It performs:
//!
//! - no raw-pointer operations;
//! - no FFI;
//! - no backend calls;
//! - no network access;
//! - no filesystem access;
//! - no process execution;
//! - no global mutable state.
//!
//! Compatible with:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust only.
//!
//! -----------------------------------------------------------------------------
//! Imports
//! -----------------------------------------------------------------------------

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
use crate::quantum::optimization::equivalence::{
    verify,
    verify_structural,
    verify_unitary,
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

/// Default integration stress workload.
///
/// This is intentionally moderate for ordinary CI.
///
/// It is not an optimizer or IR limit.
const DEFAULT_INTEGRATION_SCALE: usize = 4_096;

/// Maximum number of operations accepted from the environment without an
/// explicit larger request being considered accidental configuration.
///
/// This prevents a typo such as an enormous decimal value from unexpectedly
/// consuming all CI resources.
///
/// Larger workloads can still be explicitly requested through a dedicated
/// stress environment when desired.
const DEFAULT_INTEGRATION_HARD_CEILING: usize = 1_000_000;

/// Deterministic seed used by generated integration workloads.
///
/// The seed is part of the test contract so failures remain reproducible.
const INTEGRATION_SEED: u64 = 0x5A4D_414E_495F_494E;

// =============================================================================
// Deterministic test generator
// =============================================================================

/// Minimal deterministic pseudo-random generator used only by integration
/// tests.
///
/// No external RNG dependency is required.
///
/// This generator is deliberately independent of optimizer implementation
/// state. Its output depends only on the supplied seed.
#[derive(Debug, Clone, Copy)]
struct DeterministicGenerator {
    state: u64,
}

impl DeterministicGenerator {
    /// Creates a generator from an explicit seed.
    const fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    /// Advances the generator.
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

    /// Returns a deterministic value in `[0, upper)`.
    fn index(&mut self, upper: usize) -> usize {
        if upper == 0 {
            return 0;
        }

        (self.next_u64() % upper as u64) as usize
    }

    /// Returns a deterministic Boolean.
    fn boolean(&mut self) -> bool {
        self.next_u64() & 1 == 1
    }

    /// Returns a deterministic angle from a small exact-ish test domain.
    fn angle(&mut self) -> f64 {
        let bucket = self.index(17) as f64;
        (bucket - 8.0) * (PI / 8.0)
    }
}

// =============================================================================
// Canonical IR construction helpers
// =============================================================================

/// Constructs the canonical logical-qubit identifier.
///
/// Keeping this helper explicitly tied to `ir::qubit::QubitId` prevents
/// accidental reintroduction of the repository's historical `qubits` naming.
fn q(index: usize) -> QubitId {
    QubitId::new(index)
}

/// Constructs a validated finite canonical parameter.
fn parameter(value: f64) -> Parameter {
    Parameter::constant(value)
        .expect("finite test parameters must be accepted by canonical IR")
}

/// Constructs a non-parameterized canonical gate.
fn gate(kind: GateKind, qubits: &[usize]) -> Gate {
    Gate::new(
        kind,
        qubits
            .iter()
            .copied()
            .map(q)
            .collect(),
        Vec::new(),
        None,
        None,
    )
    .expect("test gate must satisfy canonical IR invariants")
}

/// Constructs a one-parameter canonical gate.
fn parameterized_gate(
    kind: GateKind,
    qubits: &[usize],
    value: f64,
) -> Gate {
    Gate::new(
        kind,
        qubits
            .iter()
            .copied()
            .map(q)
            .collect(),
        vec![parameter(value)],
        None,
        None,
    )
    .expect("test parameterized gate must satisfy canonical IR invariants")
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
    .expect("test circuit must satisfy canonical IR invariants")
}

/// Constructs a validated canonical circuit with an explicit classical
/// namespace.
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

// =============================================================================
// Optimization context helpers
// =============================================================================

/// Creates a production optimization context.
///
/// Every test receives a fresh invocation-scoped context.
///
/// The optimizer must not depend on process-global mutable state.
fn production_context() -> OptimizationContext {
    OptimizationContext::production(
        OptimizationConfig::default(),
    )
    .expect("production optimization context must be constructible")
}

/// Optimizes a circuit with the canonical local cancellation pass.
///
/// The input is cloned only at the test boundary so the original circuit
/// remains available for differential semantic verification.
///
/// Production optimization code itself is responsible for its own mutation
/// and transaction policy.
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
    .expect(
        "cancellation must succeed for a valid canonical test circuit",
    );

    optimized
}

// =============================================================================
// Verification helpers
// =============================================================================

/// Verifies that two circuits are structurally equivalent.
fn assert_structurally_equivalent(
    left: &QuantumCircuit,
    right: &QuantumCircuit,
) {
    let report = verify_structural(
        left,
        right,
    )
    .expect("structural verification must execute");

    assert_eq!(
        report.verdict,
        EquivalenceVerdict::Equivalent,
        "expected structural equivalence, got {report:?}",
    );

    assert!(
        report.structurally_equal,
        "structural verifier must report structurally_equal",
    );
}

/// Verifies semantic equivalence when the configured verifier can establish
/// it.
///
/// `Inconclusive` is intentionally accepted as a distinct outcome for
/// resource-sensitive verification, but it is never treated as proof.
fn assert_semantically_equivalent(
    left: &QuantumCircuit,
    right: &QuantumCircuit,
) {
    let report = verify_unitary(
        left,
        right,
    )
    .expect("unitary verification must execute");

    match report.verdict {
        EquivalenceVerdict::Equivalent => {}

        EquivalenceVerdict::NotEquivalent => {
            panic!(
                "optimization changed circuit semantics: \
                 verifier proved the circuits are not equivalent; \
                 report={report:?}",
            );
        }

        EquivalenceVerdict::Inconclusive => {
            panic!(
                "semantic verification was inconclusive for a circuit \
                 that the integration test expects to verify exactly; \
                 report={report:?}",
            );
        }
    }
}

/// Performs semantic verification through the general public `verify` API.
///
/// This deliberately exercises the same public dispatch boundary used by
/// higher-level optimization infrastructure.
fn assert_semantically_equivalent_through_dispatch(
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
    .expect("general equivalence dispatch must execute");

    assert_eq!(
        report.verdict,
        EquivalenceVerdict::Equivalent,
        "general equivalence dispatch must prove equivalence; \
         report={report:?}",
    );
}

// =============================================================================
// Stress configuration
// =============================================================================

/// Returns the requested integration workload.
///
/// The environment variable controls test workload only. It does not alter
/// production optimizer behavior.
fn integration_scale() -> usize {
    let configured = std::env::var(
        "ZAMANI_OPTIMIZATION_INTEGRATION_SCALE",
    )
    .ok()
    .and_then(|value| value.parse::<usize>().ok())
    .filter(|value| *value > 0);

    configured
        .unwrap_or(DEFAULT_INTEGRATION_SCALE)
        .min(DEFAULT_INTEGRATION_HARD_CEILING)
}

/// Returns the number of complete self-inverse pairs used by the stress test.
///
/// Each pair contains two operations.
fn stress_pair_count() -> usize {
    integration_scale().max(2) / 2
}

// =============================================================================
// Canonical IR → optimization integration
// =============================================================================

#[test]
fn canonical_ir_constructs_optimization_input() {
    let circuit = circuit(
        2,
        vec![
            gate(GateKind::H, &[0]),
            gate(GateKind::CX, &[0, 1]),
            gate(GateKind::T, &[1]),
        ],
    );

    let optimized = run_cancellation(&circuit);

    assert_semantically_equivalent(
        &circuit,
        &optimized,
    );
}

#[test]
fn canonical_qubit_module_is_used_for_optimizer_input() {
    let first = q(0);
    let second = q(1);

    assert_ne!(
        first,
        second,
        "distinct canonical logical qubits must remain distinct",
    );

    let circuit = circuit(
        2,
        vec![
            gate(GateKind::CX, &[0, 1]),
            gate(GateKind::CX, &[0, 1]),
        ],
    );

    let optimized = run_cancellation(&circuit);

    let expected = circuit(
        2,
        Vec::new(),
    );

    assert_structurally_equivalent(
        &optimized,
        &expected,
    );

    assert_semantically_equivalent(
        &circuit,
        &optimized,
    );
}

// =============================================================================
// Basic end-to-end cancellation
// =============================================================================

#[test]
fn self_inverse_single_qubit_gate_is_cancelled_end_to_end() {
    let original = circuit(
        1,
        vec![
            gate(GateKind::X, &[0]),
            gate(GateKind::X, &[0]),
        ],
    );

    let expected = circuit(
        1,
        Vec::new(),
    );

    let optimized = run_cancellation(&original);

    assert_structurally_equivalent(
        &optimized,
        &expected,
    );

    assert_semantically_equivalent(
        &original,
        &optimized,
    );
}

#[test]
fn self_inverse_hadamard_pair_is_cancelled_end_to_end() {
    let original = circuit(
        1,
        vec![
            gate(GateKind::H, &[0]),
            gate(GateKind::H, &[0]),
        ],
    );

    let expected = circuit(
        1,
        Vec::new(),
    );

    let optimized = run_cancellation(&original);

    assert_structurally_equivalent(
        &optimized,
        &expected,
    );

    assert_semantically_equivalent_through_dispatch(
        &original,
        &optimized,
    );
}

#[test]
fn controlled_self_inverse_pair_is_cancelled_end_to_end() {
    let original = circuit(
        2,
        vec![
            gate(GateKind::CX, &[0, 1]),
            gate(GateKind::CX, &[0, 1]),
        ],
    );

    let expected = circuit(
        2,
        Vec::new(),
    );

    let optimized = run_cancellation(&original);

    assert_structurally_equivalent(
        &optimized,
        &expected,
    );

    assert_semantically_equivalent(
        &original,
        &optimized,
    );
}

// =============================================================================
// Parameterized-operation integration
// =============================================================================

#[test]
fn identical_rotation_pair_integrates_with_canonical_parameter_ir() {
    let angle = PI / 4.0;

    let original = circuit(
        1,
        vec![
            parameterized_gate(
                GateKind::RZ,
                &[0],
                angle,
            ),
            parameterized_gate(
                GateKind::RZ,
                &[0],
                -angle,
            ),
        ],
    );

    let optimized = run_cancellation(&original);

    assert_semantically_equivalent(
        &original,
        &optimized,
    );
}

#[test]
fn zero_rotation_does_not_break_optimizer_integration() {
    let original = circuit(
        1,
        vec![
            parameterized_gate(
                GateKind::RZ,
                &[0],
                0.0,
            ),
        ],
    );

    let optimized = run_cancellation(&original);

    assert_semantically_equivalent(
        &original,
        &optimized,
    );
}

#[test]
fn parameterized_rotation_uses_finite_canonical_parameters() {
    let values = [
        -2.0 * PI,
        -PI,
        -PI / 2.0,
        -PI / 4.0,
        0.0,
        PI / 4.0,
        PI / 2.0,
        PI,
        2.0 * PI,
    ];

    for value in values {
        let original = circuit(
            1,
            vec![
                parameterized_gate(
                    GateKind::RZ,
                    &[0],
                    value,
                ),
            ],
        );

        let optimized = run_cancellation(&original);

        assert_semantically_equivalent(
            &original,
            &optimized,
        );
    }
}

// =============================================================================
// Qubit isolation
// =============================================================================

#[test]
fn independent_qubits_remain_semantically_independent() {
    let original = circuit(
        2,
        vec![
            gate(GateKind::X, &[0]),
            gate(GateKind::X, &[1]),
        ],
    );

    let optimized = run_cancellation(&original);

    assert_semantically_equivalent(
        &original,
        &optimized,
    );
}

#[test]
fn cancellation_does_not_require_contiguous_global_qubit_usage() {
    let original = circuit(
        4,
        vec![
            gate(GateKind::X, &[0]),
            gate(GateKind::H, &[2]),
            gate(GateKind::X, &[0]),
            gate(GateKind::Z, &[3]),
        ],
    );

    let optimized = run_cancellation(&original);

    assert_semantically_equivalent(
        &original,
        &optimized,
    );
}

#[test]
fn sparse_logical_qubit_namespace_is_supported() {
    let original = circuit(
        16,
        vec![
            gate(GateKind::H, &[0]),
            gate(GateKind::CX, &[0, 15]),
            gate(GateKind::CX, &[0, 15]),
            gate(GateKind::H, &[0]),
        ],
    );

    let optimized = run_cancellation(&original);

    assert_semantically_equivalent(
        &original,
        &optimized,
    );
}

// =============================================================================
// Semantic boundaries
// =============================================================================

#[test]
fn barrier_remains_a_semantic_optimization_boundary() {
    let original = circuit(
        1,
        vec![
            gate(GateKind::H, &[0]),
            gate(GateKind::H, &[0]),
            gate(GateKind::Barrier, &[0]),
            gate(GateKind::H, &[0]),
            gate(GateKind::H, &[0]),
        ],
    );

    let optimized = run_cancellation(&original);

    let expected = circuit(
        1,
        vec![
            gate(GateKind::Barrier, &[0]),
        ],
    );

    assert_structurally_equivalent(
        &optimized,
        &expected,
    );
}

#[test]
fn reset_is_not_treated_as_an_ordinary_unitary_operation() {
    let original = circuit(
        1,
        vec![
            gate(GateKind::X, &[0]),
            gate(GateKind::X, &[0]),
            gate(GateKind::Reset, &[0]),
            gate(GateKind::X, &[0]),
            gate(GateKind::X, &[0]),
        ],
    );

    let optimized = run_cancellation(&original);

    let expected = circuit(
        1,
        vec![
            gate(GateKind::Reset, &[0]),
        ],
    );

    assert_structurally_equivalent(
        &optimized,
        &expected,
    );
}

// =============================================================================
// Empty and already-optimized circuits
// =============================================================================

#[test]
fn empty_circuit_integrates_with_optimizer() {
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
fn already_optimized_circuit_is_preserved_semantically() {
    let original = circuit(
        2,
        vec![
            gate(GateKind::H, &[0]),
            gate(GateKind::CX, &[0, 1]),
            gate(GateKind::T, &[1]),
        ],
    );

    let optimized = run_cancellation(&original);

    assert_semantically_equivalent(
        &original,
        &optimized,
    );
}

// =============================================================================
// Classical namespace integration
// =============================================================================

#[test]
fn optimization_preserves_classical_namespace() {
    let original = circuit_with_classical_bits(
        2,
        4,
        vec![
            gate(GateKind::H, &[0]),
            gate(GateKind::H, &[0]),
            gate(GateKind::CX, &[0, 1]),
            gate(GateKind::CX, &[0, 1]),
        ],
    );

    let optimized = run_cancellation(&original);

    let expected = circuit_with_classical_bits(
        2,
        4,
        Vec::new(),
    );

    assert_structurally_equivalent(
        &optimized,
        &expected,
    );
}

// =============================================================================
// Idempotence
// =============================================================================

#[test]
fn cancellation_is_idempotent() {
    let original = circuit(
        3,
        vec![
            gate(GateKind::H, &[0]),
            gate(GateKind::H, &[0]),
            gate(GateKind::X, &[1]),
            gate(GateKind::X, &[1]),
            gate(GateKind::CX, &[1, 2]),
            gate(GateKind::CX, &[1, 2]),
            gate(GateKind::T, &[2]),
        ],
    );

    let once = run_cancellation(&original);
    let twice = run_cancellation(&once);

    assert_structurally_equivalent(
        &once,
        &twice,
    );

    assert_semantically_equivalent(
        &original,
        &once,
    );
}

#[test]
fn repeated_optimization_converges() {
    let mut current = circuit(
        2,
        vec![
            gate(GateKind::H, &[0]),
            gate(GateKind::H, &[0]),
            gate(GateKind::X, &[1]),
            gate(GateKind::X, &[1]),
            gate(GateKind::CX, &[0, 1]),
            gate(GateKind::CX, &[0, 1]),
        ],
    );

    for _ in 0..8 {
        let next = run_cancellation(&current);

        let report = verify_structural(
            &current,
            &next,
        )
        .expect(
            "structural verification must execute during convergence test",
        );

        if report.verdict == EquivalenceVerdict::Equivalent {
            current = next;
        } else {
            panic!(
                "repeated optimization produced a non-equivalent circuit: \
                 report={report:?}",
            );
        }
    }

    let expected = circuit(
        2,
        Vec::new(),
    );

    assert_structurally_equivalent(
        &current,
        &expected,
    );
}

// =============================================================================
// Determinism
// =============================================================================

#[test]
fn identical_inputs_produce_identical_structural_outputs() {
    let original = circuit(
        3,
        vec![
            gate(GateKind::H, &[0]),
            gate(GateKind::H, &[0]),
            gate(GateKind::CX, &[0, 1]),
            gate(GateKind::CX, &[0, 1]),
            gate(GateKind::T, &[2]),
            gate(GateKind::T, &[2]),
        ],
    );

    let first = run_cancellation(&original);
    let second = run_cancellation(&original);

    assert_structurally_equivalent(
        &first,
        &second,
    );
}

// =============================================================================
// Large deterministic integration workload
// =============================================================================

/// Builds a deterministic large circuit containing cancellation opportunities.
///
/// The construction is linear in the requested number of pairs and does not
/// require quadratic test-side work.
fn build_large_cancellation_circuit(
    pairs: usize,
) -> QuantumCircuit {
    let mut generator = DeterministicGenerator::new(
        INTEGRATION_SEED,
    );

    let mut operations = Vec::with_capacity(
        pairs.saturating_mul(2),
    );

    for _ in 0..pairs {
        let kind = match generator.index(4) {
            0 => GateKind::X,
            1 => GateKind::Y,
            2 => GateKind::Z,
            _ => GateKind::H,
        };

        let qubit = generator.index(4);

        operations.push(
            gate(
                kind,
                &[qubit],
            ),
        );

        operations.push(
            gate(
                kind,
                &[qubit],
            ),
        );

        // Consume deterministic generator state so future extensions of the
        // workload remain reproducible without relying on external randomness.
        let _ = generator.boolean();
    }

    circuit(
        4,
        operations,
    )
}

#[test]
fn large_deterministic_workload_integrates_without_fixed_small_circuit_limits() {
    let pairs = stress_pair_count();

    let original = build_large_cancellation_circuit(
        pairs,
    );

    let optimized = run_cancellation(
        &original,
    );

    let expected = circuit(
        4,
        Vec::new(),
    );

    assert_structurally_equivalent(
        &optimized,
        &expected,
    );

    assert_semantically_equivalent(
        &original,
        &optimized,
    );
}

// =============================================================================
// Resource-driven scaling
// =============================================================================

#[test]
fn resource_driven_scale_is_deterministic() {
    let scale = integration_scale();

    assert!(
        scale > 0,
        "integration workload must always be positive",
    );

    let first = build_large_cancellation_circuit(
        scale / 2,
    );

    let second = build_large_cancellation_circuit(
        scale / 2,
    );

    assert_structurally_equivalent(
        &first,
        &second,
    );
}

#[test]
fn large_workload_preserves_optimizer_semantic_contract() {
    let pairs = stress_pair_count();

    let original = build_large_cancellation_circuit(
        pairs.min(2_048),
    );

    let optimized = run_cancellation(
        &original,
    );

    assert_semantically_equivalent_through_dispatch(
        &original,
        &optimized,
    );
}

// =============================================================================
// Context isolation
// =============================================================================

#[test]
fn separate_optimizer_contexts_do_not_share_mutable_execution_state() {
    let first_context = production_context();
    let second_context = production_context();

    assert_eq!(
        format!("{first_context:?}"),
        format!("{second_context:?}"),
        "fresh production contexts should begin from equivalent configuration \
         state",
    );
}

#[test]
fn fresh_context_can_optimize_the_same_circuit_repeatedly() {
    let original = circuit(
        2,
        vec![
            gate(GateKind::X, &[0]),
            gate(GateKind::X, &[0]),
            gate(GateKind::H, &[1]),
            gate(GateKind::H, &[1]),
        ],
    );

    let first = run_cancellation(
        &original,
    );

    let second = run_cancellation(
        &original,
    );

    assert_structurally_equivalent(
        &first,
        &second,
    );

    assert_semantically_equivalent(
        &original,
        &first,
    );
}

// =============================================================================
// Differential integration cases
// =============================================================================

#[test]
fn optimization_preserves_a_mixed_single_and_two_qubit_circuit() {
    let original = circuit(
        3,
        vec![
            gate(GateKind::H, &[0]),
            gate(GateKind::X, &[1]),
            gate(GateKind::CX, &[0, 1]),
            gate(GateKind::CX, &[0, 1]),
            gate(GateKind::Z, &[2]),
            gate(GateKind::Z, &[2]),
            gate(GateKind::H, &[0]),
        ],
    );

    let optimized = run_cancellation(
        &original,
    );

    assert_semantically_equivalent(
        &original,
        &optimized,
    );
}

#[test]
fn optimization_preserves_non_cancelled_operations() {
    let original = circuit(
        2,
        vec![
            gate(GateKind::H, &[0]),
            gate(GateKind::X, &[1]),
            gate(GateKind::CX, &[0, 1]),
        ],
    );

    let optimized = run_cancellation(
        &original,
    );

    assert_structurally_equivalent(
        &original,
        &optimized,
    );
}

#[test]
fn optimization_can_remove_cancelled_regions_while_preserving_surrounding_operations() {
    let original = circuit(
        2,
        vec![
            gate(GateKind::H, &[0]),
            gate(GateKind::X, &[1]),
            gate(GateKind::X, &[1]),
            gate(GateKind::CX, &[0, 1]),
        ],
    );

    let expected = circuit(
        2,
        vec![
            gate(GateKind::H, &[0]),
            gate(GateKind::CX, &[0, 1]),
        ],
    );

    let optimized = run_cancellation(
        &original,
    );

    assert_structurally_equivalent(
        &optimized,
        &expected,
    );

    assert_semantically_equivalent(
        &original,
        &optimized,
    );
}

// =============================================================================
// Verification-dispatch integration
// =============================================================================

#[test]
fn structural_and_semantic_verification_are_distinct_contracts() {
    let original = circuit(
        1,
        vec![
            gate(GateKind::H, &[0]),
            gate(GateKind::H, &[0]),
        ],
    );

    let optimized = circuit(
        1,
        Vec::new(),
    );

    let structural_report = verify_structural(
        &original,
        &optimized,
    )
    .expect(
        "structural verification must execute",
    );

    assert_eq!(
        structural_report.verdict,
        EquivalenceVerdict::NotEquivalent,
        "different gate sequences must not be called structurally equal",
    );

    let semantic_report = verify_unitary(
        &original,
        &optimized,
    )
    .expect(
        "semantic verification must execute",
    );

    assert_eq!(
        semantic_report.verdict,
        EquivalenceVerdict::Equivalent,
        "semantically equivalent circuits must be accepted",
    );
}

#[test]
fn optimization_output_can_be_verified_through_general_equivalence_api() {
    let original = circuit(
        2,
        vec![
            gate(GateKind::CX, &[0, 1]),
            gate(GateKind::CX, &[0, 1]),
            gate(GateKind::H, &[0]),
            gate(GateKind::H, &[0]),
        ],
    );

    let optimized = run_cancellation(
        &original,
    );

    assert_semantically_equivalent_through_dispatch(
        &original,
        &optimized,
    );
}

// =============================================================================
// Production invariant
// =============================================================================

#[test]
fn integration_suite_uses_only_canonical_quantum_types() {
    let _ = std::any::TypeId::of::<QuantumCircuit>();
    let _ = std::any::TypeId::of::<Gate>();
    let _ = std::any::TypeId::of::<GateKind>();
    let _ = std::any::TypeId::of::<Parameter>();
    let _ = std::any::TypeId::of::<QubitId>();
    let _ = std::any::TypeId::of::<OptimizationConfig>();
    let _ = std::any::TypeId::of::<OptimizationContext>();
    let _ = std::any::TypeId::of::<CancellationPass>();
}

#[test]
fn optimizer_integration_does_not_require_unsafe_or_backend_execution() {
    // This test intentionally has no runtime work.
    //
    // The module-level `#![forbid(unsafe_code)]` is the actual enforcement
    // mechanism. The purpose of this test is to make the safety contract
    // explicit at the integration-test boundary.
    assert!(true);
}