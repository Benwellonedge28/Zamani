//! Zamani Quantum Routing — Verification Test Suite
//!
//! `src/quantum/routing/tests/verification.rs`
//!
//! # Responsibility
//!
//! Production-grade tests for `routing::verification`.
//!
//! This file verifies the routing verifier as the final correctness boundary
//! between a routing algorithm and subsequent compiler/hardware stages.
//!
//! The verifier must establish that:
//!
//! - the topology is valid;
//! - the initial mapping is valid;
//! - the final mapping is valid;
//! - physical qubits referenced by a route exist;
//! - physical qubits referenced by a route are available;
//! - mappings remain bijective;
//! - logical-to-physical identity is preserved;
//! - physical operands correspond to the current mapping at the exact point
//!   where every gate executes;
//! - SWAP movements are legal and update mapping state correctly;
//! - permutation movements are legal and replace mapping state correctly;
//! - bridge movements have valid topology structure;
//! - barriers reference valid mapped physical resources;
//! - gate arity is respected;
//! - gate operands are unique;
//! - direction-sensitive operations respect directed topology;
//! - explicit gate support is respected;
//! - non-adjacent two-qubit operations are rejected;
//! - measurement/reset/barrier semantics remain well formed;
//! - final mapping replay exactly matches the declared final mapping;
//! - strict verification checks additional global invariants;
//! - resource limits are enforced before expensive verification work;
//! - verification does not mutate caller-owned mapping state;
//! - verification is deterministic;
//! - malformed routing streams fail closed;
//! - disabling verification is explicit rather than accidentally interpreted
//!   as a successful verification;
//! - verification remains independent of routing algorithms, compiler IR,
//!   hardware providers, and quantum simulation.
//!
//! # Architectural position
//!
//! ```text
//! routing algorithm
//!       │
//!       ▼
//! RoutingOperation stream
//!       │
//!       ▼
//! ┌───────────────────────┐
//! │ verification.rs       │
//! │                       │
//! │ structural checks     │
//! │ mapping replay        │
//! │ gate legality         │
//! │ semantic preservation │
//! │ final-state checking  │
//! └───────────┬───────────┘
//!             │
//!       ┌─────┴─────┐
//!       ▼           ▼
//!    success       error
//!       │           │
//!       ▼           ▼
//! hardware      compiler diagnostic
//! lowering
//! ```
//!
//! # Integration contract
//!
//! This test module consumes the stable routing contracts from:
//!
//! ```text
//! routing/types.rs
//! routing/errors.rs
//! routing/topology.rs
//! routing/mapping.rs
//! routing/config.rs
//! routing/result.rs
//! routing/verification.rs
//! ```
//!
//! It intentionally does NOT depend on:
//!
//! - routing algorithms;
//! - router orchestration;
//! - transpiler/compiler IR;
//! - OpenQASM;
//! - hardware provider SDKs;
//! - quantum simulators;
//! - QEC implementations.
//!
//! This keeps verification independently testable.
//!
//! # Important semantic rule
//!
//! A physically adjacent gate is not automatically semantically correct.
//!
//! For every routed gate:
//!
//! ```text
//! mapping(logical_operands[i]) == physical_operands[i]
//! ```
//!
//! immediately before the gate executes.
//!
//! The tests therefore deliberately include cases where the physical operands
//! are legal and adjacent but correspond to the wrong logical qubits.
//!
//! # Mapping replay
//!
//! Verification must replay the operation stream:
//!
//! ```text
//! initial mapping
//!       │
//!       ├── SWAP ──► mapping'
//!       │
//!       ├── gate    ──► mapping'
//!       │
//!       ├── SWAP ──► mapping''
//!       │
//!       └── gate    ──► mapping''
//!                              │
//!                              ▼
//!                       declared final mapping
//! ```
//!
//! The reconstructed mapping and declared final mapping must be identical.
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
//! This test module contains no unsafe code.
//!
//! The module explicitly denies unsafe code so accidental unsafe additions
//! become compilation errors.
//!
//! # Test philosophy
//!
//! Tests prefer semantic assertions over implementation-specific details.
//!
//! In particular, tests should not require a particular routing algorithm to
//! produce a particular route unless that route is itself part of the public
//! contract.
//!
//! Verification tests instead assert:
//!
//! - acceptance of valid routes;
//! - rejection of invalid routes;
//! - preservation of mappings;
//! - preservation of logical operand identity;
//! - deterministic behavior;
//! - structured failure classification.
//!
//! This allows SABRE, shortest-path, lookahead, noise-aware, and future
//! algorithms to evolve without rewriting the verification contract.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use crate::quantum::routing::config::VerificationLevel;
use crate::quantum::routing::errors::{
    RoutingError,
    RoutingErrorKind,
};
use crate::quantum::routing::mapping::QubitMapping;
use crate::quantum::routing::topology::{
    PhysicalTopology,
    TopologyBuilder,
};
use crate::quantum::routing::types::{
    GateIdentity,
    LogicalQubitId,
    PhysicalQubitId,
    QubitInteraction,
    RoutingMove,
    RoutingOperation,
};
use crate::quantum::routing::verification::{
    RoutingVerificationInput,
    RoutingVerifier,
    VERIFIER_VERSION,
};

// =============================================================================
// Test helpers
// =============================================================================

fn l(index: usize) -> LogicalQubitId {
    LogicalQubitId::new(index)
}

fn p(index: usize) -> PhysicalQubitId {
    PhysicalQubitId::new(index)
}

fn line_topology(count: usize) -> PhysicalTopology {
    PhysicalTopology::line(count)
        .expect("test line topology must be valid")
}

fn bidirectional_pair() -> PhysicalTopology {
    TopologyBuilder::named("verification-bidirectional")
        .add_qubit(p(0))
        .expect("p0 registration must succeed")
        .add_qubit(p(1))
        .expect("p1 registration must succeed")
        .undirected_edge(p(0), p(1))
        .expect("undirected edge must succeed")
        .build()
        .expect("bidirectional test topology must build")
}

fn bidirectional_chain(count: usize) -> PhysicalTopology {
    let mut builder = TopologyBuilder::named("verification-chain");

    for index in 0..count {
        builder = builder
            .add_qubit(p(index))
            .expect("test qubit registration must succeed");
    }

    for index in 0..count.saturating_sub(1) {
        builder = builder
            .undirected_edge(p(index), p(index + 1))
            .expect("test chain edge must succeed");
    }

    builder
        .build()
        .expect("bidirectional chain must build")
}

fn directed_pair() -> PhysicalTopology {
    TopologyBuilder::named("verification-directed")
        .add_qubit(p(0))
        .expect("p0 registration must succeed")
        .add_qubit(p(1))
        .expect("p1 registration must succeed")
        .directed_edge(p(0), p(1))
        .expect("directed edge must succeed")
        .build()
        .expect("directed topology must build")
}

fn directed_pair_with_cx() -> PhysicalTopology {
    TopologyBuilder::named("verification-directed-cx")
        .add_qubit(p(0))
        .expect("p0 registration must succeed")
        .add_qubit(p(1))
        .expect("p1 registration must succeed")
        .directed_edge(p(0), p(1))
        .expect("directed edge must succeed")
        .supported_gate("cx", p(0), p(1))
        .expect("forward CX support must succeed")
        .build()
        .expect("directed CX topology must build")
}

fn mapping_01() -> QubitMapping {
    QubitMapping::from_assignments([
        (l(0), p(0)),
        (l(1), p(1)),
    ])
    .expect("test mapping must be valid")
}

fn mapping_012() -> QubitMapping {
    QubitMapping::from_assignments([
        (l(0), p(0)),
        (l(1), p(1)),
        (l(2), p(2)),
    ])
    .expect("test mapping must be valid")
}

fn mapping_0123() -> QubitMapping {
    QubitMapping::from_assignments([
        (l(0), p(0)),
        (l(1), p(1)),
        (l(2), p(2)),
        (l(3), p(3)),
    ])
    .expect("test mapping must be valid")
}

fn cx(logical_a: usize, logical_b: usize, physical_a: usize, physical_b: usize) -> RoutingOperation {
    RoutingOperation::Gate {
        gate: GateIdentity::Cx,
        operands: vec![p(physical_a), p(physical_b)],
        logical_operands: vec![l(logical_a), l(logical_b)],
    }
}

fn x(logical: usize, physical: usize) -> RoutingOperation {
    RoutingOperation::Gate {
        gate: GateIdentity::X,
        operands: vec![p(physical)],
        logical_operands: vec![l(logical)],
    }
}

fn h(logical: usize, physical: usize) -> RoutingOperation {
    RoutingOperation::Gate {
        gate: GateIdentity::H,
        operands: vec![p(physical)],
        logical_operands: vec![l(logical)],
    }
}

fn measure(logical: usize, physical: usize) -> RoutingOperation {
    RoutingOperation::Gate {
        gate: GateIdentity::Measure,
        operands: vec![p(physical)],
        logical_operands: vec![l(logical)],
    }
}

fn reset(logical: usize, physical: usize) -> RoutingOperation {
    RoutingOperation::Gate {
        gate: GateIdentity::Reset,
        operands: vec![p(physical)],
        logical_operands: vec![l(logical)],
    }
}

fn barrier(operands: &[usize]) -> RoutingOperation {
    RoutingOperation::Barrier {
        operands: operands.iter().copied().map(p).collect(),
    }
}

fn swap(a: usize, b: usize) -> RoutingOperation {
    RoutingOperation::Move(RoutingMove::Swap {
        a: p(a),
        b: p(b),
    })
}

fn bridge(
    a: usize,
    middle: usize,
    b: usize,
    gate: GateIdentity,
) -> RoutingOperation {
    RoutingOperation::Move(RoutingMove::Bridge {
        a: p(a),
        bridge: p(middle),
        b: p(b),
        gate,
    })
}

fn permutation(assignments: &[(usize, usize)]) -> RoutingOperation {
    RoutingOperation::Move(RoutingMove::Permutation {
        mapping: assignments
            .iter()
            .copied()
            .map(|(logical, physical)| (l(logical), p(physical)))
            .collect(),
    })
}

fn verify(
    topology: &PhysicalTopology,
    initial: &QubitMapping,
    final_mapping: &QubitMapping,
    original: &[QubitInteraction],
    operations: &[RoutingOperation],
    level: VerificationLevel,
) -> Result<crate::quantum::routing::verification::VerificationReport, RoutingError> {
    let initial_snapshot = initial.snapshot();
    let final_snapshot = final_mapping.snapshot();

    let input = RoutingVerificationInput::new(
        topology,
        &initial_snapshot,
        &final_snapshot,
        original,
        operations,
        level,
    );

    RoutingVerifier::new().verify(&input)
}

fn assert_verification_error(
    result: Result<
        crate::quantum::routing::verification::VerificationReport,
        RoutingError,
    >,
) -> RoutingError {
    match result {
        Ok(report) => panic!(
            "expected verification failure, but verification succeeded: {report:?}"
        ),
        Err(error) => error,
    }
}

fn error_kind(error: &RoutingError) -> &RoutingErrorKind {
    &error.kind
}

fn interaction(
    gate: GateIdentity,
    operands: &[usize],
) -> QubitInteraction {
    QubitInteraction::new(
        operands.iter().copied().map(l).collect(),
        gate,
    )
}

fn basic_original_cx() -> Vec<QubitInteraction> {
    vec![interaction(GateIdentity::Cx, &[0, 1])]
}

// =============================================================================
// Version and construction
// =============================================================================

#[test]
fn verifier_version_is_stable_and_non_empty() {
    assert!(!VERIFIER_VERSION.is_empty());
    assert!(VERIFIER_VERSION.starts_with("zamani-routing-verifier-"));
}

#[test]
fn verifier_is_stateless_and_constructible() {
    let first = RoutingVerifier::new();
    let second = RoutingVerifier::new();

    assert_eq!(first, second);
}

#[test]
fn verifier_is_copyable() {
    let verifier = RoutingVerifier::new();
    let copied = verifier;

    assert_eq!(verifier, copied);
}

// =============================================================================
// Valid basic routes
// =============================================================================

#[test]
fn accepts_valid_single_qubit_gate() {
    let topology = line_topology(2);
    let mapping = mapping_01();

    let report = verify(
        &topology,
        &mapping,
        &mapping,
        &[interaction(GateIdentity::X, &[0])],
        &[x(0, 0)],
        VerificationLevel::Standard,
    )
    .expect("valid single-qubit route must verify");

    assert!(report.all_checks_passed());
    assert_eq!(report.operations_checked, 1);
    assert_eq!(report.gates_checked, 1);
    assert_eq!(report.movements_checked, 0);
    assert!(report.final_mapping_matches);
}

#[test]
fn accepts_valid_two_qubit_gate() {
    let topology = bidirectional_pair();
    let mapping = mapping_01();

    let report = verify(
        &topology,
        &mapping,
        &mapping,
        &basic_original_cx(),
        &[cx(0, 1, 0, 1)],
        VerificationLevel::Standard,
    )
    .expect("valid two-qubit route must verify");

    assert!(report.all_checks_passed());
    assert_eq!(report.gates_checked, 1);
    assert_eq!(report.operations_checked, 1);
}

#[test]
fn accepts_valid_measurement() {
    let topology = bidirectional_pair();
    let mapping = mapping_01();

    let report = verify(
        &topology,
        &mapping,
        &mapping,
        &[interaction(GateIdentity::Measure, &[0])],
        &[measure(0, 0)],
        VerificationLevel::Standard,
    )
    .expect("valid measurement must verify");

    assert!(report.all_checks_passed());
}

#[test]
fn accepts_valid_reset() {
    let topology = bidirectional_pair();
    let mapping = mapping_01();

    let report = verify(
        &topology,
        &mapping,
        &mapping,
        &[interaction(GateIdentity::Reset, &[0])],
        &[reset(0, 0)],
        VerificationLevel::Standard,
    )
    .expect("valid reset must verify");

    assert!(report.all_checks_passed());
}

#[test]
fn accepts_valid_barrier() {
    let topology = bidirectional_chain(3);
    let mapping = mapping_012();

    let report = verify(
        &topology,
        &mapping,
        &mapping,
        &[],
        &[barrier(&[0, 1, 2])],
        VerificationLevel::Basic,
    )
    .expect("valid barrier must verify");

    assert!(report.all_checks_passed());
    assert_eq!(report.barriers_checked, 1);
}

// =============================================================================
// Verification level semantics
// =============================================================================

#[test]
fn none_level_is_explicitly_not_requested() {
    let topology = bidirectional_pair();
    let mapping = mapping_01();

    let report = verify(
        &topology,
        &mapping,
        &mapping,
        &basic_original_cx(),
        &[],
        VerificationLevel::None,
    )
    .expect("disabled verification should return explicit not-requested result");

    assert_eq!(report.level, VerificationLevel::None);
    assert_eq!(report.operations_checked, 0);
    assert_eq!(report.passed_checks, 0);
    assert!(!report.final_mapping_matches);
    assert!(!report.all_checks_passed());
}

#[test]
fn basic_level_performs_structural_verification() {
    let topology = bidirectional_pair();
    let mapping = mapping_01();

    let report = verify(
        &topology,
        &mapping,
        &mapping,
        &[],
        &[swap(0, 1)],
        VerificationLevel::Basic,
    )
    .expect("valid structural route must verify");

    assert!(report.all_checks_passed());
    assert!(report.structural_checks > 0);
    assert!(report.mapping_checks > 0);
}

#[test]
fn standard_level_performs_semantic_replay() {
    let topology = bidirectional_pair();
    let mapping = mapping_01();

    let report = verify(
        &topology,
        &mapping,
        &mapping,
        &basic_original_cx(),
        &[cx(0, 1, 0, 1)],
        VerificationLevel::Standard,
    )
    .expect("standard verification must verify semantic route");

    assert!(report.preservation_checks > 0);
    assert!(report.final_mapping_matches);
}

#[test]
fn strict_level_adds_global_invariant_checks() {
    let topology = bidirectional_pair();
    let mapping = mapping_01();

    let report = verify(
        &topology,
        &mapping,
        &mapping,
        &basic_original_cx(),
        &[cx(0, 1, 0, 1)],
        VerificationLevel::Strict,
    )
    .expect("strict verification must succeed for valid route");

    assert!(report.all_checks_passed());
    assert_eq!(report.level, VerificationLevel::Strict);
    assert!(report.mapping_checks >= 2);
}

// =============================================================================
// Mapping replay
// =============================================================================

#[test]
fn valid_swap_replay_matches_final_mapping() {
    let topology = bidirectional_pair();
    let initial = mapping_01();

    let mut expected = mapping_01();
    expected
        .swap_physical(p(0), p(1))
        .expect("test SWAP must succeed");

    let report = verify(
        &topology,
        &initial,
        &expected,
        &[],
        &[swap(0, 1)],
        VerificationLevel::Standard,
    )
    .expect("valid SWAP replay must verify");

    assert!(report.final_mapping_matches);
    assert_eq!(
        expected.physical_of(l(0)),
        Some(p(1))
    );
    assert_eq!(
        expected.physical_of(l(1)),
        Some(p(0))
    );
}

#[test]
fn multiple_swaps_are_replayed_in_order() {
    let topology = bidirectional_chain(3);
    let initial = mapping_012();

    let mut expected = mapping_012();
    expected
        .swap_physical(p(0), p(1))
        .expect("first SWAP must succeed");
    expected
        .swap_physical(p(1), p(2))
        .expect("second SWAP must succeed");

    let report = verify(
        &topology,
        &initial,
        &expected,
        &[],
        &[swap(0, 1), swap(1, 2)],
        VerificationLevel::Strict,
    )
    .expect("multiple SWAP replay must verify");

    assert!(report.final_mapping_matches);
    assert_eq!(
        expected.physical_of(l(0)),
        Some(p(1))
    );
    assert_eq!(
        expected.physical_of(l(1)),
        Some(p(2))
    );
    assert_eq!(
        expected.physical_of(l(2)),
        Some(p(0))
    );
}

#[test]
fn gate_after_swap_must_use_updated_mapping() {
    let topology = bidirectional_pair();
    let initial = mapping_01();

    let mut expected = mapping_01();
    expected
        .swap_physical(p(0), p(1))
        .expect("SWAP must succeed");

    let operations = vec![
        swap(0, 1),
        x(0, 1),
    ];

    verify(
        &topology,
        &initial,
        &expected,
        &[interaction(GateIdentity::X, &[0])],
        &operations,
        VerificationLevel::Standard,
    )
    .expect("gate must use logical qubit's post-SWAP physical location");
}

#[test]
fn gate_using_pre_swap_location_is_rejected() {
    let topology = bidirectional_pair();
    let initial = mapping_01();

    let mut expected = mapping_01();
    expected
        .swap_physical(p(0), p(1))
        .expect("SWAP must succeed");

    let error = assert_verification_error(verify(
        &topology,
        &initial,
        &expected,
        &[interaction(GateIdentity::X, &[0])],
        &[
            swap(0, 1),
            x(0, 0),
        ],
        VerificationLevel::Standard,
    ));

    assert!(matches!(
        error_kind(&error),
        RoutingErrorKind::Verification(_)
    ));
}

// =============================================================================
// Logical/physical semantic preservation
// =============================================================================

#[test]
fn wrong_logical_operand_for_correct_physical_operand_is_rejected() {
    let topology = bidirectional_pair();
    let mapping = mapping_01();

    let error = assert_verification_error(verify(
        &topology,
        &mapping,
        &mapping,
        &[interaction(GateIdentity::Cx, &[0, 1])],
        &[cx(1, 0, 0, 1)],
        VerificationLevel::Standard,
    ));

    assert!(matches!(
        error_kind(&error),
        RoutingErrorKind::Verification(_)
    ));
}

#[test]
fn swapped_logical_operand_order_is_rejected_for_directional_gate() {
    let topology = directed_pair_with_cx();
    let mapping = mapping_01();

    let error = assert_verification_error(verify(
        &topology,
        &mapping,
        &mapping,
        &[interaction(GateIdentity::Cx, &[0, 1])],
        &[cx(1, 0, 1, 0)],
        VerificationLevel::Standard,
    ));

    assert!(matches!(
        error_kind(&error),
        RoutingErrorKind::Verification(_)
    ));
}

#[test]
fn logical_operand_count_must_match_physical_operand_count() {
    let topology = bidirectional_pair();
    let mapping = mapping_01();

    let operation = RoutingOperation::Gate {
        gate: GateIdentity::Cx,
        operands: vec![p(0), p(1)],
        logical_operands: vec![l(0)],
    };

    let error = assert_verification_error(verify(
        &topology,
        &mapping,
        &mapping,
        &basic_original_cx(),
        &[operation],
        VerificationLevel::Standard,
    ));

    assert!(matches!(
        error_kind(&error),
        RoutingErrorKind::Verification(_)
    ));
}

#[test]
fn duplicate_logical_operands_are_rejected() {
    let topology = bidirectional_pair();
    let mapping = mapping_01();

    let operation = RoutingOperation::Gate {
        gate: GateIdentity::Cx,
        operands: vec![p(0), p(1)],
        logical_operands: vec![l(0), l(0)],
    };

    let error = assert_verification_error(verify(
        &topology,
        &mapping,
        &mapping,
        &[],
        &[operation],
        VerificationLevel::Standard,
    ));

    assert!(matches!(
        error_kind(&error),
        RoutingErrorKind::Verification(_)
    ));
}

#[test]
fn duplicate_physical_operands_are_rejected() {
    let topology = bidirectional_pair();
    let mapping = mapping_01();

    let operation = RoutingOperation::Gate {
        gate: GateIdentity::Cx,
        operands: vec![p(0), p(0)],
        logical_operands: vec![l(0), l(1)],
    };

    let error = assert_verification_error(verify(
        &topology,
        &mapping,
        &mapping,
        &[],
        &[operation],
        VerificationLevel::Standard,
    ));

    assert!(matches!(
        error_kind(&error),
        RoutingErrorKind::Verification(_)
    ));
}

#[test]
fn unknown_logical_qubit_is_rejected() {
    let topology = bidirectional_pair();
    let mapping = mapping_01();

    let error = assert_verification_error(verify(
        &topology,
        &mapping,
        &mapping,
        &[interaction(GateIdentity::X, &[2])],
        &[x(2, 0)],
        VerificationLevel::Standard,
    ));

    assert!(matches!(
        error_kind(&error),
        RoutingErrorKind::Verification(_)
    ));
}

#[test]
fn unknown_logical_qubit_in_routed_gate_is_rejected() {
    let topology = bidirectional_pair();
    let mapping = mapping_01();

    let error = assert_verification_error(verify(
        &topology,
        &mapping,
        &mapping,
        &[],
        &[x(2, 0)],
        VerificationLevel::Standard,
    ));

    assert!(matches!(
        error_kind(&error),
        RoutingErrorKind::Verification(_)
    ));
}

// =============================================================================
// Physical resource validation
// =============================================================================

#[test]
fn nonexistent_physical_gate_operand_is_rejected() {
    let topology = bidirectional_pair();
    let mapping = mapping_01();

    let error = assert_verification_error(verify(
        &topology,
        &mapping,
        &mapping,
        &[],
        &[x(0, 99)],
        VerificationLevel::Standard,
    ));

    assert!(matches!(
        error_kind(&error),
        RoutingErrorKind::Verification(_)
    ));
}

#[test]
fn nonexistent_physical_swap_operand_is_rejected() {
    let topology = bidirectional_pair();
    let mapping = mapping_01();

    let error = assert_verification_error(verify(
        &topology,
        &mapping,
        &mapping,
        &[],
        &[swap(0, 99)],
        VerificationLevel::Basic,
    ));

    assert!(matches!(
        error_kind(&error),
        RoutingErrorKind::Verification(_)
    ));
}

#[test]
fn unavailable_physical_resources_must_be_rejected() {
    let topology = TopologyBuilder::named("verification-unavailable")
        .qubit(
            p(0),
            crate::quantum::routing::topology::PhysicalQubitProperties {
                available: true,
                ..Default::default()
            },
        )
        .expect("p0 registration must succeed")
        .qubit(
            p(1),
            crate::quantum::routing::topology::PhysicalQubitProperties {
                available: false,
                ..Default::default()
            },
        )
        .expect("p1 registration must succeed")
        .undirected_edge(p(0), p(1))
        .expect("edge must succeed")
        .build()
        .expect("test topology must build");

    let mapping = QubitMapping::from_assignments([
        (l(0), p(0)),
        (l(1), p(1)),
    ])
    .expect("mapping construction must succeed");

    let error = assert_verification_error(verify(
        &topology,
        &mapping,
        &mapping,
        &[interaction(GateIdentity::X, &[1])],
        &[x(1, 1)],
        VerificationLevel::Standard,
    ));

    assert!(matches!(
        error_kind(&error),
        RoutingErrorKind::Verification(_)
    ));
}

// =============================================================================
// Topology legality
// =============================================================================

#[test]
fn non_adjacent_two_qubit_gate_is_rejected() {
    let topology = line_topology(3);
    let mapping = mapping_012();

    let error = assert_verification_error(verify(
        &topology,
        &mapping,
        &mapping,
        &[interaction(GateIdentity::Cx, &[0, 2])],
        &[cx(0, 2, 0, 2)],
        VerificationLevel::Standard,
    ));

    assert!(matches!(
        error_kind(&error),
        RoutingErrorKind::Verification(_)
    ));
}

#[test]
fn adjacent_two_qubit_gate_is_accepted() {
    let topology = line_topology(3);
    let mapping = mapping_012();

    verify(
        &topology,
        &mapping,
        &mapping,
        &[interaction(GateIdentity::Cx, &[0, 1])],
        &[cx(0, 1, 0, 1)],
        VerificationLevel::Standard,
    )
    .expect("adjacent two-qubit gate must verify");
}

#[test]
fn reverse_direction_on_directed_topology_is_rejected() {
    let topology = directed_pair_with_cx();
    let mapping = mapping_01();

    let error = assert_verification_error(verify(
        &topology,
        &mapping,
        &mapping,
        &[interaction(GateIdentity::Cx, &[1, 0])],
        &[cx(1, 0, 1, 0)],
        VerificationLevel::Standard,
    ));

    assert!(matches!(
        error_kind(&error),
        RoutingErrorKind::Verification(_)
    ));
}

#[test]
fn forward_direction_on_directed_topology_is_accepted() {
    let topology = directed_pair_with_cx();
    let mapping = mapping_01();

    verify(
        &topology,
        &mapping,
        &mapping,
        &[interaction(GateIdentity::Cx, &[0, 1])],
        &[cx(0, 1, 0, 1)],
        VerificationLevel::Standard,
    )
    .expect("forward CX must verify");
}

#[test]
fn structural_connection_does_not_override_gate_direction() {
    let topology = directed_pair_with_cx();
    let mapping = mapping_01();

    assert!(topology.has_connection(p(0), p(1)));
    assert!(topology.has_connection(p(1), p(0)));
    assert!(topology.is_adjacent(p(0), p(1)));
    assert!(!topology.is_adjacent(p(1), p(0)));

    let error = assert_verification_error(verify(
        &topology,
        &mapping,
        &mapping,
        &[interaction(GateIdentity::Cx, &[1, 0])],
        &[cx(1, 0, 1, 0)],
        VerificationLevel::Standard,
    ));

    assert!(matches!(
        error_kind(&error),
        RoutingErrorKind::Verification(_)
    ));
}

// =============================================================================
// Gate shape validation
// =============================================================================

#[test]
fn zero_operand_gate_is_rejected() {
    let topology = bidirectional_pair();
    let mapping = mapping_01();

    let operation = RoutingOperation::Gate {
        gate: GateIdentity::X,
        operands: vec![],
        logical_operands: vec![],
    };

    let error = assert_verification_error(verify(
        &topology,
        &mapping,
        &mapping,
        &[],
        &[operation],
        VerificationLevel::Standard,
    ));

    assert!(matches!(
        error_kind(&error),
        RoutingErrorKind::Verification(_)
    ));
}

#[test]
fn measurement_with_two_operands_is_rejected() {
    let topology = bidirectional_pair();
    let mapping = mapping_01();

    let operation = RoutingOperation::Gate {
        gate: GateIdentity::Measure,
        operands: vec![p(0), p(1)],
        logical_operands: vec![l(0), l(1)],
    };

    let error = assert_verification_error(verify(
        &topology,
        &mapping,
        &mapping,
        &[],
        &[operation],
        VerificationLevel::Standard,
    ));

    assert!(matches!(
        error_kind(&error),
        RoutingErrorKind::Verification(_)
    ));
}

#[test]
fn reset_with_two_operands_is_rejected() {
    let topology = bidirectional_pair();
    let mapping = mapping_01();

    let operation = RoutingOperation::Gate {
        gate: GateIdentity::Reset,
        operands: vec![p(0), p(1)],
        logical_operands: vec![l(0), l(1)],
    };

    let error = assert_verification_error(verify(
        &topology,
        &mapping,
        &mapping,
        &[],
        &[operation],
        VerificationLevel::Standard,
    ));

    assert!(matches!(
        error_kind(&error),
        RoutingErrorKind::Verification(_)
    ));
}

#[test]
fn barrier_duplicate_physical_operands_are_rejected() {
    let topology = bidirectional_pair();
    let mapping = mapping_01();

    let error = assert_verification_error(verify(
        &topology,
        &mapping,
        &mapping,
        &[],
        &[barrier(&[0, 0])],
        VerificationLevel::Basic,
    ));

    assert!(matches!(
        error_kind(&error),
        RoutingErrorKind::Verification(_)
    ));
}

// =============================================================================
// SWAP legality
// =============================================================================

#[test]
fn swap_self_loop_is_rejected() {
    let topology = bidirectional_pair();
    let mapping = mapping_01();

    let error = assert_verification_error(verify(
        &topology,
        &mapping,
        &mapping,
        &[],
        &[swap(0, 0)],
        VerificationLevel::Basic,
    ));

    assert!(matches!(
        error_kind(&error),
        RoutingErrorKind::Verification(_)
    ));
}

#[test]
fn swap_between_non_adjacent_qubits_is_rejected() {
    let topology = line_topology(3);
    let mapping = mapping_012();

    let error = assert_verification_error(verify(
        &topology,
        &mapping,
        &mapping,
        &[],
        &[swap(0, 2)],
        VerificationLevel::Basic,
    ));

    assert!(matches!(
        error_kind(&error),
        RoutingErrorKind::Verification(_)
    ));
}

#[test]
fn swap_between_adjacent_qubits_is_accepted() {
    let topology = bidirectional_pair();
    let mapping = mapping_01();

    let mut final_mapping = mapping_01();
    final_mapping
        .swap_physical(p(0), p(1))
        .expect("SWAP must succeed");

    verify(
        &topology,
        &mapping,
        &final_mapping,
        &[],
        &[swap(0, 1)],
        VerificationLevel::Basic,
    )
    .expect("legal adjacent SWAP must verify");
}

#[test]
fn swap_requires_both_directions_or_explicit_bidirectional_swap_support() {
    let topology = directed_pair();
    let mapping = mapping_01();

    let error = assert_verification_error(verify(
        &topology,
        &mapping,
        &mapping,
        &[],
        &[swap(0, 1)],
        VerificationLevel::Basic,
    ));

    assert!(matches!(
        error_kind(&error),
        RoutingErrorKind::Verification(_)
    ));
}

// =============================================================================
// Permutation validation
// =============================================================================

#[test]
fn valid_permutation_replays_to_declared_mapping() {
    let topology = bidirectional_chain(3);
    let initial = mapping_012();

    let final_mapping = QubitMapping::from_assignments([
        (l(0), p(2)),
        (l(1), p(0)),
        (l(2), p(1)),
    ])
    .expect("permutation target mapping must be valid");

    let report = verify(
        &topology,
        &initial,
        &final_mapping,
        &[],
        &[permutation(&[(0, 2), (1, 0), (2, 1)])],
        VerificationLevel::Standard,
    )
    .expect("valid permutation must verify");

    assert!(report.final_mapping_matches);
    assert_eq!(report.permutations_checked, 1);
}

#[test]
fn empty_permutation_is_rejected() {
    let topology = bidirectional_chain(2);
    let mapping = mapping_01();

    let error = assert_verification_error(verify(
        &topology,
        &mapping,
        &mapping,
        &[],
        &[permutation(&[])],
        VerificationLevel::Basic,
    ));

    assert!(matches!(
        error_kind(&error),
        RoutingErrorKind::Verification(_)
    ));
}

#[test]
fn duplicate_logical_permutation_entry_is_rejected() {
    let topology = bidirectional_chain(2);
    let mapping = mapping_01();

    let error = assert_verification_error(verify(
        &topology,
        &mapping,
        &mapping,
        &[],
        &[permutation(&[(0, 0), (0, 1)])],
        VerificationLevel::Basic,
    ));

    assert!(matches!(
        error_kind(&error),
        RoutingErrorKind::Verification(_)
    ));
}

#[test]
fn duplicate_physical_permutation_entry_is_rejected() {
    let topology = bidirectional_chain(2);
    let mapping = mapping_01();

    let error = assert_verification_error(verify(
        &topology,
        &mapping,
        &mapping,
        &[],
        &[permutation(&[(0, 0), (1, 0)])],
        VerificationLevel::Basic,
    ));

    assert!(matches!(
        error_kind(&error),
        RoutingErrorKind::Verification(_)
    ));
}

#[test]
fn permutation_changing_logical_qubit_set_is_rejected() {
    let topology = bidirectional_chain(3);
    let mapping = mapping_012();

    let error = assert_verification_error(verify(
        &topology,
        &mapping,
        &mapping,
        &[],
        &[permutation(&[(0, 0), (1, 1), (9, 2)])],
        VerificationLevel::Basic,
    ));

    assert!(matches!(
        error_kind(&error),
        RoutingErrorKind::Verification(_)
    ));
}

#[test]
fn permutation_referencing_unknown_physical_qubit_is_rejected() {
    let topology = bidirectional_chain(2);
    let mapping = mapping_01();

    let error = assert_verification_error(verify(
        &topology,
        &mapping,
        &mapping,
        &[],
        &[permutation(&[(0, 0), (1, 99)])],
        VerificationLevel::Basic,
    ));

    assert!(matches!(
        error_kind(&error),
        RoutingErrorKind::Verification(_)
    ));
}

// =============================================================================
// Bridge validation
// =============================================================================

#[test]
fn valid_bridge_structure_is_accepted() {
    let topology = bidirectional_chain(3);
    let mapping = mapping_012();

    verify(
        &topology,
        &mapping,
        &mapping,
        &[],
        &[bridge(0, 1, 2, GateIdentity::Cx)],
        VerificationLevel::Basic,
    )
    .expect("valid bridge structure must verify");
}

#[test]
fn bridge_with_duplicate_endpoint_is_rejected() {
    let topology = bidirectional_chain(3);
    let mapping = mapping_012();

    let error = assert_verification_error(verify(
        &topology,
        &mapping,
        &mapping,
        &[],
        &[bridge(0, 0, 2, GateIdentity::Cx)],
        VerificationLevel::Basic,
    ));

    assert!(matches!(
        error_kind(&error),
        RoutingErrorKind::Verification(_)
    ));
}

#[test]
fn bridge_with_non_adjacent_endpoint_is_rejected() {
    let topology = bidirectional_chain(4);
    let mapping = mapping_0123();

    let error = assert_verification_error(verify(
        &topology,
        &mapping,
        &mapping,
        &[],
        &[bridge(0, 2, 3, GateIdentity::Cx)],
        VerificationLevel::Basic,
    ));

    assert!(matches!(
        error_kind(&error),
        RoutingErrorKind::Verification(_)
    ));
}

// =============================================================================
// Final mapping verification
// =============================================================================

#[test]
fn incorrect_declared_final_mapping_is_rejected() {
    let topology = bidirectional_pair();
    let initial = mapping_01();

    let declared_final = mapping_01();

    let error = assert_verification_error(verify(
        &topology,
        &initial,
        &declared_final,
        &[],
        &[swap(0, 1)],
        VerificationLevel::Standard,
    ));

    assert!(matches!(
        error_kind(&error),
        RoutingErrorKind::Verification(_)
    ));
}

#[test]
fn declared_final_mapping_with_unknown_physical_qubit_is_rejected() {
    let topology = bidirectional_pair();
    let initial = mapping_01();

    let declared_final = QubitMapping::from_assignments([
        (l(0), p(0)),
        (l(1), p(99)),
    ])
    .expect("mapping itself can represent physical identity independently");

    let error = assert_verification_error(verify(
        &topology,
        &initial,
        &declared_final,
        &[],
        &[],
        VerificationLevel::Basic,
    ));

    assert!(matches!(
        error_kind(&error),
        RoutingErrorKind::Verification(_)
    ));
}

#[test]
fn final_mapping_must_remain_bijective() {
    let topology = bidirectional_pair();
    let initial = mapping_01();

    // The mapping implementation itself prevents a collision. This test
    // therefore verifies the verifier's acceptance of the authoritative
    // mapping invariant rather than fabricating invalid private state.
    initial
        .validate()
        .expect("authoritative mapping must remain bijective");

    verify(
        &topology,
        &initial,
        &initial,
        &[],
        &[],
        VerificationLevel::Strict,
    )
    .expect("valid bijective mapping must verify strictly");
}

// =============================================================================
// Original interaction validation
// =============================================================================

#[test]
fn original_interaction_with_zero_arity_is_rejected() {
    let topology = bidirectional_pair();
    let mapping = mapping_01();

    let original = [
        interaction(GateIdentity::X, &[]),
    ];

    let error = assert_verification_error(verify(
        &topology,
        &mapping,
        &mapping,
        &original,
        &[],
        VerificationLevel::Standard,
    ));

    assert!(matches!(
        error_kind(&error),
        RoutingErrorKind::Verification(_)
    ));
}

#[test]
fn original_interaction_with_duplicate_logical_operand_is_rejected() {
    let topology = bidirectional_pair();
    let mapping = mapping_01();

    let original = [
        interaction(GateIdentity::Cx, &[0, 0]),
    ];

    let error = assert_verification_error(verify(
        &topology,
        &mapping,
        &mapping,
        &original,
        &[],
        VerificationLevel::Standard,
    ));

    assert!(matches!(
        error_kind(&error),
        RoutingErrorKind::Verification(_)
    ));
}

#[test]
fn original_interaction_requires_initial_mapping() {
    let topology = bidirectional_pair();
    let mapping = mapping_01();

    let original = [
        interaction(GateIdentity::X, &[7]),
    ];

    let error = assert_verification_error(verify(
        &topology,
        &mapping,
        &mapping,
        &original,
        &[],
        VerificationLevel::Standard,
    ));

    assert!(matches!(
        error_kind(&error),
        RoutingErrorKind::Verification(_)
    ));
}

#[test]
fn original_interactions_can_be_empty_for_structural_verification() {
    let topology = bidirectional_pair();
    let mapping = mapping_01();

    verify(
        &topology,
        &mapping,
        &mapping,
        &[],
        &[x(0, 0)],
        VerificationLevel::Basic,
    )
    .expect("structural verification should not require original interactions");
}

// =============================================================================
// Operation limits
// =============================================================================

#[test]
fn zero_operation_limit_rejects_nonempty_operation_stream() {
    let topology = bidirectional_pair();
    let mapping = mapping_01();

    let initial_snapshot = mapping.snapshot();
    let final_snapshot = mapping.snapshot();

    let input = RoutingVerificationInput::new(
        &topology,
        &initial_snapshot,
        &final_snapshot,
        &[],
        &[x(0, 0)],
        VerificationLevel::Basic,
    )
    .with_max_operations(0);

    let error = assert_verification_error(
        RoutingVerifier::new().verify(&input)
    );

    assert!(matches!(
        error_kind(&error),
        RoutingErrorKind::Verification(_)
    ));
}

#[test]
fn operation_limit_is_enforced() {
    let topology = bidirectional_pair();
    let mapping = mapping_01();

    let initial_snapshot = mapping.snapshot();
    let final_snapshot = mapping.snapshot();

    let operations = vec![
        x(0, 0),
        x(0, 0),
        x(0, 0),
    ];

    let input = RoutingVerificationInput::new(
        &topology,
        &initial_snapshot,
        &final_snapshot,
        &[],
        &operations,
        VerificationLevel::Basic,
    )
    .with_max_operations(2);

    let error = assert_verification_error(
        RoutingVerifier::new().verify(&input)
    );

    assert!(matches!(
        error_kind(&error),
        RoutingErrorKind::Verification(_)
    ));
}

#[test]
fn zero_arity_limit_rejects_nonempty_stream() {
    let topology = bidirectional_pair();
    let mapping = mapping_01();

    let initial_snapshot = mapping.snapshot();
    let final_snapshot = mapping.snapshot();

    let input = RoutingVerificationInput::new(
        &topology,
        &initial_snapshot,
        &final_snapshot,
        &[],
        &[x(0, 0)],
        VerificationLevel::Basic,
    )
    .with_max_arity(0);

    let error = assert_verification_error(
        RoutingVerifier::new().verify(&input)
    );

    assert!(matches!(
        error_kind(&error),
        RoutingErrorKind::Verification(_)
    ));
}

// =============================================================================
// Mapping immutability / transaction safety
// =============================================================================

#[test]
fn verification_does_not_mutate_initial_mapping() {
    let topology = bidirectional_pair();
    let initial = mapping_01();

    let before = initial.snapshot();

    let mut expected = mapping_01();
    expected
        .swap_physical(p(0), p(1))
        .expect("SWAP must succeed");

    verify(
        &topology,
        &initial,
        &expected,
        &[],
        &[swap(0, 1)],
        VerificationLevel::Strict,
    )
    .expect("valid route must verify");

    assert_eq!(initial.snapshot(), before);
}

#[test]
fn verification_does_not_mutate_declared_final_mapping() {
    let topology = bidirectional_pair();
    let initial = mapping_01();

    let mut declared_final = mapping_01();
    declared_final
        .swap_physical(p(0), p(1))
        .expect("SWAP must succeed");

    let before = declared_final.snapshot();

    verify(
        &topology,
        &initial,
        &declared_final,
        &[],
        &[swap(0, 1)],
        VerificationLevel::Strict,
    )
    .expect("valid route must verify");

    assert_eq!(declared_final.snapshot(), before);
}

// =============================================================================
// Complex semantic replay
// =============================================================================

#[test]
fn complex_route_replay_preserves_logical_identity() {
    let topology = bidirectional_chain(4);
    let initial = mapping_0123();

    let mut expected = mapping_0123();

    expected
        .swap_physical(p(1), p(2))
        .expect("first SWAP must succeed");

    expected
        .swap_physical(p(2), p(3))
        .expect("second SWAP must succeed");

    let operations = vec![
        h(0, 0),
        swap(1, 2),
        x(1, 2),
        swap(2, 3),
        x(1, 3),
    ];

    let original = vec![
        interaction(GateIdentity::H, &[0]),
        interaction(GateIdentity::X, &[1]),
        interaction(GateIdentity::X, &[1]),
    ];

    let report = verify(
        &topology,
        &initial,
        &expected,
        &original,
        &operations,
        VerificationLevel::Strict,
    )
    .expect("complex route must verify");

    assert!(report.all_checks_passed());
    assert_eq!(report.swaps_checked, 2);
    assert_eq!(report.gates_checked, 3);
    assert!(report.final_mapping_matches);
}

#[test]
fn measurement_after_mapping_change_must_follow_logical_qubit() {
    let topology = bidirectional_pair();
    let initial = mapping_01();

    let mut expected = mapping_01();
    expected
        .swap_physical(p(0), p(1))
        .expect("SWAP must succeed");

    let original = vec![
        interaction(GateIdentity::Measure, &[0]),
    ];

    verify(
        &topology,
        &initial,
        &expected,
        &original,
        &[
            swap(0, 1),
            measure(0, 1),
        ],
        VerificationLevel::Strict,
    )
    .expect("measurement must follow logical qubit after SWAP");
}

#[test]
fn measurement_on_wrong_post_swap_location_is_rejected() {
    let topology = bidirectional_pair();
    let initial = mapping_01();

    let mut expected = mapping_01();
    expected
        .swap_physical(p(0), p(1))
        .expect("SWAP must succeed");

    let error = assert_verification_error(verify(
        &topology,
        &initial,
        &expected,
        &[interaction(GateIdentity::Measure, &[0])],
        &[
            swap(0, 1),
            measure(0, 0),
        ],
        VerificationLevel::Strict,
    ));

    assert!(matches!(
        error_kind(&error),
        RoutingErrorKind::Verification(_)
    ));
}

// =============================================================================
// Multi-qubit operations
// =============================================================================

#[test]
fn unsupported_multi_qubit_operation_is_not_silently_decomposed() {
    let topology = bidirectional_chain(3);
    let mapping = mapping_012();

    let operation = RoutingOperation::Gate {
        gate: GateIdentity::Ccx,
        operands: vec![p(0), p(1), p(2)],
        logical_operands: vec![l(0), l(1), l(2)],
    };

    let result = verify(
        &topology,
        &mapping,
        &mapping,
        &[interaction(GateIdentity::Ccx, &[0, 1, 2])],
        &[operation],
        VerificationLevel::Standard,
    );

    // The verifier may accept a native multi-qubit operation only according to
    // the topology contract. It must never mutate the operation into a
    // decomposition itself.
    //
    // Current topology semantics require pairwise structural adjacency for
    // native multi-qubit verification, so the line topology must reject this
    // CCX.
    assert!(result.is_err());
}

#[test]
fn native_three_qubit_shape_must_not_use_duplicate_operands() {
    let topology = bidirectional_chain(3);
    let mapping = mapping_012();

    let operation = RoutingOperation::Gate {
        gate: GateIdentity::Ccx,
        operands: vec![p(0), p(1), p(1)],
        logical_operands: vec![l(0), l(1), l(2)],
    };

    let error = assert_verification_error(verify(
        &topology,
        &mapping,
        &mapping,
        &[],
        &[operation],
        VerificationLevel::Standard,
    ));

    assert!(matches!(
        error_kind(&error),
        RoutingErrorKind::Verification(_)
    ));
}

// =============================================================================
// Final mapping mismatch diagnostics
// =============================================================================

#[test]
fn mapping_mismatch_is_reported_as_verification_error() {
    let topology = bidirectional_pair();
    let initial = mapping_01();

    let error = assert_verification_error(verify(
        &topology,
        &initial,
        &initial,
        &[],
        &[swap(0, 1)],
        VerificationLevel::Standard,
    ));

    assert!(matches!(
        error_kind(&error),
        RoutingErrorKind::Verification(_)
    ));

    assert_eq!(
        error.context.stage,
        Some(crate::quantum::routing::errors::RoutingStage::Verification)
    );
}

#[test]
fn verification_error_contains_operation_context_when_available() {
    let topology = bidirectional_pair();
    let mapping = mapping_01();

    let error = assert_verification_error(verify(
        &topology,
        &mapping,
        &mapping,
        &[],
        &[cx(0, 1, 0, 0)],
        VerificationLevel::Standard,
    ));

    assert!(matches!(
        error_kind(&error),
        RoutingErrorKind::Verification(_)
    ));

    assert_eq!(
        error.context.stage,
        Some(crate::quantum::routing::errors::RoutingStage::Verification)
    );
}

// =============================================================================
// Determinism
// =============================================================================

#[test]
fn identical_valid_inputs_produce_identical_reports() {
    let topology = bidirectional_chain(3);
    let mapping = mapping_012();

    let original = vec![
        interaction(GateIdentity::X, &[0]),
        interaction(GateIdentity::Cx, &[0, 1]),
    ];

    let operations = vec![
        x(0, 0),
        cx(0, 1, 0, 1),
    ];

    let first = verify(
        &topology,
        &mapping,
        &mapping,
        &original,
        &operations,
        VerificationLevel::Strict,
    )
    .expect("first verification must succeed");

    let second = verify(
        &topology,
        &mapping,
        &mapping,
        &original,
        &operations,
        VerificationLevel::Strict,
    )
    .expect("second verification must succeed");

    assert_eq!(first, second);
}

#[test]
fn deterministic_failure_classification_is_stable() {
    let topology = bidirectional_pair();
    let mapping = mapping_01();

    let first = assert_verification_error(verify(
        &topology,
        &mapping,
        &mapping,
        &[],
        &[swap(0, 99)],
        VerificationLevel::Basic,
    ));

    let second = assert_verification_error(verify(
        &topology,
        &mapping,
        &mapping,
        &[],
        &[swap(0, 99)],
        VerificationLevel::Basic,
    ));

    assert_eq!(first.kind, second.kind);
    assert_eq!(first.context, second.context);
}

// =============================================================================
// Snapshot round-trip
// =============================================================================

#[test]
fn snapshot_round_trip_preserves_mapping_for_verification() {
    let topology = bidirectional_chain(4);
    let mapping = mapping_0123();

    let snapshot = mapping.snapshot();

    let report = verify(
        &topology,
        &mapping,
        &mapping,
        &[],
        &[],
        VerificationLevel::Strict,
    )
    .expect("snapshot-compatible mapping must verify");

    assert!(report.final_mapping_matches);
    assert_eq!(
        snapshot.logical_to_physical(),
        mapping.snapshot().logical_to_physical()
    );
}

// =============================================================================
// Regression tests for dangerous verifier mistakes
// =============================================================================

#[test]
fn verifier_must_not_accept_adjacent_gate_with_wrong_logical_identity() {
    let topology = bidirectional_pair();
    let mapping = mapping_01();

    // Both physical operands are valid and adjacent. The route is still wrong
    // because the logical operation asks for q0 -> q1 while the physical
    // operands claim q1 -> q0.
    let error = assert_verification_error(verify(
        &topology,
        &mapping,
        &mapping,
        &[interaction(GateIdentity::Cx, &[0, 1])],
        &[cx(1, 0, 0, 1)],
        VerificationLevel::Strict,
    ));

    assert!(matches!(
        error_kind(&error),
        RoutingErrorKind::Verification(_)
    ));
}

#[test]
fn verifier_must_not_accept_correct_final_mapping_when_intermediate_gate_is_wrong() {
    let topology = bidirectional_pair();
    let initial = mapping_01();

    let mut expected = mapping_01();
    expected
        .swap_physical(p(0), p(1))
        .expect("SWAP must succeed");

    // Final mapping is correct, but the gate between the SWAP and final state
    // is associated with the wrong physical location.
    let error = assert_verification_error(verify(
        &topology,
        &initial,
        &expected,
        &[interaction(GateIdentity::X, &[0])],
        &[
            swap(0, 1),
            x(0, 0),
        ],
        VerificationLevel::Strict,
    ));

    assert!(matches!(
        error_kind(&error),
        RoutingErrorKind::Verification(_)
    ));
}

#[test]
fn verifier_must_not_accept_wrong_declared_final_mapping_even_if_all_gates_are_legal() {
    let topology = bidirectional_pair();
    let initial = mapping_01();

    let mut actual_final = mapping_01();
    actual_final
        .swap_physical(p(0), p(1))
        .expect("SWAP must succeed");

    let error = assert_verification_error(verify(
        &topology,
        &initial,
        &mapping_01(),
        &[],
        &[swap(0, 1)],
        VerificationLevel::Strict,
    ));

    assert!(matches!(
        error_kind(&error),
        RoutingErrorKind::Verification(_)
    ));

    assert_ne!(
        actual_final.logical_to_physical(),
        mapping_01().logical_to_physical()
    );
}

// =============================================================================
// Empty streams
// =============================================================================

#[test]
fn empty_operation_stream_with_identical_mappings_is_valid() {
    let topology = bidirectional_pair();
    let mapping = mapping_01();

    let report = verify(
        &topology,
        &mapping,
        &mapping,
        &[],
        &[],
        VerificationLevel::Strict,
    )
    .expect("empty route with identical mappings must verify");

    assert!(report.all_checks_passed());
    assert!(report.final_mapping_matches);
    assert_eq!(report.operations_checked, 0);
}

#[test]
fn empty_operation_stream_with_different_final_mapping_is_invalid() {
    let topology = bidirectional_pair();
    let initial = mapping_01();

    let mut final_mapping = mapping_01();
    final_mapping
        .swap_physical(p(0), p(1))
        .expect("SWAP must succeed");

    let error = assert_verification_error(verify(
        &topology,
        &initial,
        &final_mapping,
        &[],
        &[],
        VerificationLevel::Strict,
    ));

    assert!(matches!(
        error_kind(&error),
        RoutingErrorKind::Verification(_)
    ));
}

// =============================================================================
// Large-but-reasonable verification
// =============================================================================

#[test]
fn verifier_handles_a_reasonable_large_operation_stream() {
    let topology = bidirectional_pair();
    let mapping = mapping_01();

    let operations: Vec<_> = (0..10_000)
        .map(|_| x(0, 0))
        .collect();

    let original = vec![
        interaction(GateIdentity::X, &[0]),
    ];

    let initial_snapshot = mapping.snapshot();
    let final_snapshot = mapping.snapshot();

    let input = RoutingVerificationInput::new(
        &topology,
        &initial_snapshot,
        &final_snapshot,
        &original,
        &operations,
        VerificationLevel::Basic,
    );

    let report = RoutingVerifier::new()
        .verify(&input)
        .expect("10,000-operation verification should remain bounded");

    assert!(report.all_checks_passed());
    assert_eq!(report.operations_checked, 10_000);
    assert_eq!(report.gates_checked, 10_000);
}

// =============================================================================
// Public API regression
// =============================================================================

#[test]
fn verify_strict_forces_strict_level() {
    let topology = bidirectional_pair();
    let mapping = mapping_01();

    let initial_snapshot = mapping.snapshot();
    let final_snapshot = mapping.snapshot();

    let input = RoutingVerificationInput::new(
        &topology,
        &initial_snapshot,
        &final_snapshot,
        &basic_original_cx(),
        &[cx(0, 1, 0, 1)],
        VerificationLevel::Basic,
    );

    let report = RoutingVerifier::new()
        .verify_strict(&input)
        .expect("forced strict verification must succeed");

    assert_eq!(report.level, VerificationLevel::Strict);
    assert!(report.all_checks_passed());
}

// =============================================================================
// Contract-level assertions
// =============================================================================

#[test]
fn successful_standard_report_contains_verifier_version() {
    let topology = bidirectional_pair();
    let mapping = mapping_01();

    let report = verify(
        &topology,
        &mapping,
        &mapping,
        &[],
        &[],
        VerificationLevel::Standard,
    )
    .expect("valid route must verify");

    assert_eq!(report.verifier_version, VERIFIER_VERSION);
}

#[test]
fn successful_report_has_consistent_check_counts() {
    let topology = bidirectional_pair();
    let mapping = mapping_01();

    let report = verify(
        &topology,
        &mapping,
        &mapping,
        &basic_original_cx(),
        &[cx(0, 1, 0, 1)],
        VerificationLevel::Strict,
    )
    .expect("valid route must verify");

    assert_eq!(
        report.total_checks(),
        report.structural_checks
            + report.mapping_checks
            + report.executability_checks
            + report.preservation_checks
    );

    assert!(report.passed_checks <= report.total_checks());
    assert!(report.all_checks_passed());
}

#[test]
fn verification_report_summary_is_successful_for_valid_route() {
    let topology = bidirectional_pair();
    let mapping = mapping_01();

    let report = verify(
        &topology,
        &mapping,
        &mapping,
        &basic_original_cx(),
        &[cx(0, 1, 0, 1)],
        VerificationLevel::Standard,
    )
    .expect("valid route must verify");

    let summary = report.summary();

    assert_eq!(summary.status, crate::quantum::routing::result::VerificationStatus::Passed);
    assert_eq!(summary.level, VerificationLevel::Standard);
    assert!(summary.all_checks_passed());
    assert_eq!(
        summary.verifier_version.as_deref(),
        Some(VERIFIER_VERSION)
    );
}