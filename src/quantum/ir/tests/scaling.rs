//! Zamani Quantum IR — Scalability and Resource-Proportionality Tests.
//!
//! # Purpose
//!
//! This module verifies the scalability contract of the canonical Zamani
//! Quantum IR.
//!
//! The fundamental property being protected is:
//!
//! ```text
//! declared quantum namespace
//!             ≠
//! allocated compiler storage
//!             ≠
//! physical machine capacity
//! ```
//!
//! A circuit must be able to describe a very large finite logical namespace
//! without allocating one object per declared qubit merely because the
//! namespace is large.
//!
//! The tests therefore emphasize:
//!
//! - sparse large namespaces;
//! - namespace/operation independence;
//! - checked identifier arithmetic;
//! - explicit resource policies;
//! - monotonic policy behavior;
//! - operation-count proportionality;
//! - validation proportionality to actual semantic content;
//! - analysis proportionality to actual semantic content;
//! - absence of fixed architectural qubit ceilings;
//! - canonical `quantum::ir::qubit::QubitId` usage;
//! - deterministic behavior;
//! - graceful rejection at representational boundaries;
//! - no unsafe implementation requirements.
//!
//! # Important scalability interpretation
//!
//! "Infinity" is not a value that can be allocated or represented by a finite
//! Rust process. The production contract is instead:
//!
//! ```text
//! any finite representable namespace
//!         |
//!         v
//! subject only to explicit policy + actual available resources
//! ```
//!
//! These tests therefore never claim that a machine with infinite resources
//! exists. They verify that the IR does not introduce an artificial finite
//! quantum-machine ceiling.
//!
//! # What this file does NOT test
//!
//! This file deliberately does not test:
//!
//! - hardware topology;
//! - hardware capacity;
//! - routing;
//! - scheduling;
//! - calibration;
//! - pulse synthesis;
//! - backend execution;
//! - simulator state vectors;
//! - simulator memory;
//! - QEC decoding;
//! - optimizer quality;
//! - frontend parsing;
//! - vendor APIs.
//!
//! Those are downstream concerns.
//!
//! # Integration contract
//!
//! This module consumes the public contracts of:
//!
//! - `quantum::ir::circuit`;
//! - `quantum::ir::gate`;
//! - `quantum::ir::identity`;
//! - `quantum::ir::limits`;
//! - `quantum::ir::qubit`;
//! - `quantum::ir::analysis`;
//! - `quantum::ir::validation`.
//!
//! It intentionally does not access private fields.
//!
//! The canonical qubit path is always:
//!
//! ```text
//! quantum::ir::qubit
//! ```
//!
//! Never use the compatibility alias:
//!
//! ```text
//! quantum::ir::qubits
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
//! The module additionally uses `#![forbid(unsafe_code)]` so the no-unsafe
//! requirement is compiler-enforced.
//!
//! # Architectural invariant
//!
//! A test value appearing in this file is a test input, not an architectural
//! maximum.
//!
//! In particular, no value in this file may be interpreted as:
//!
//! - the maximum number of Zamani qubits;
//! - the maximum number of physical qubits;
//! - the maximum register size;
//! - a hardware capability;
//! - a backend limit.
//!
//! Production limits are supplied explicitly through `QuantumIrLimits`.
//!
//! -----------------------------------------------------------------------------
//! Test implementation
//! -----------------------------------------------------------------------------

#![forbid(unsafe_code)]

use crate::quantum::ir::analysis::{
    analyze,
    analyze_with_limits,
};
use crate::quantum::ir::circuit::{
    CircuitError,
    QuantumCircuit,
};
use crate::quantum::ir::gate::{
    Gate,
    GateKind,
};
use crate::quantum::ir::limits::QuantumIrLimits;
use crate::quantum::ir::qubit::{
    PhysicalQubitId,
    QubitId,
};
use crate::quantum::ir::validation::validate_circuit_with_limits;

// =============================================================================
// Test-policy helpers
// =============================================================================

/// Returns an explicit bounded policy suitable for tests.
///
/// This is intentionally a test policy and must never be interpreted as a
/// Zamani architectural limit.
fn bounded_test_limits() -> QuantumIrLimits {
    QuantumIrLimits::production()
        .with_max_qubits(32)
        .with_max_classical_bits(32)
        .with_max_operations(128)
        .with_max_operands(32)
        .with_max_parameters(32)
        .with_max_depth(128)
        .with_max_measurements(32)
        .with_max_barriers(32)
        .with_max_validation_steps(10_000)
        .with_max_analysis_steps(10_000)
}

/// Creates a circuit with an explicitly supplied policy.
///
/// Keeping this helper here means the individual scaling tests depend only on
/// the public circuit contract and do not need to know how policy construction
/// is implemented internally.
fn circuit(
    num_qubits: usize,
    limits: QuantumIrLimits,
) -> QuantumCircuit {
    QuantumCircuit::try_new_with_limits(
        num_qubits,
        0,
        limits,
    )
    .expect("test circuit must be constructible under its explicit policy")
}

/// Creates a valid unary operation.
///
/// The canonical logical-qubit identity comes from `quantum::ir::qubit`.
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

/// Creates a valid binary operation.
///
/// The helper intentionally accepts `QubitId` rather than raw integers so the
/// tests exercise the same canonical identity type used by production IR.
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

// =============================================================================
// Foundation: no unsafe
// =============================================================================

/// The actual no-unsafe guarantee is enforced by `#![forbid(unsafe_code)]`.
///
/// This test exists only to keep the module's basic compilation contract
/// exercised.
#[test]
fn scalability_tests_compile_without_unsafe() {
    let logical = QubitId::new(0);
    let physical = PhysicalQubitId::new(0);

    assert_eq!(logical.index(), physical.index());
}

// =============================================================================
// Canonical identity usage
// =============================================================================

/// Logical and physical identifiers remain distinct even when their numeric
/// indexes happen to be identical.
#[test]
fn logical_and_physical_identity_domains_remain_distinct() {
    let logical = QubitId::new(0);
    let physical = PhysicalQubitId::new(0);

    assert_eq!(logical.index(), physical.index());

    // Compile-time type distinction is exercised by constructing each type
    // independently. There is deliberately no conversion that silently turns
    // a logical identifier into a physical allocation.
    assert_ne!(
        format!("{logical}"),
        format!("{physical}"),
        "logical and physical display identities must remain distinguishable"
    );
}

/// The canonical logical identifier can represent the highest value of its
/// underlying platform index type.
///
/// This does not allocate a register or a circuit.
#[test]
fn logical_identifier_reaches_platform_index_boundary_without_wrapping() {
    let maximum = QubitId::new(usize::MAX);

    assert_eq!(
        maximum.index(),
        usize::MAX,
    );

    assert_eq!(
        maximum.checked_next(),
        None,
        "identifier increment must reject overflow rather than wrap"
    );
}

/// The physical identifier has the same overflow-safe identity behavior.
#[test]
fn physical_identifier_reaches_platform_index_boundary_without_wrapping() {
    let maximum = PhysicalQubitId::new(usize::MAX);

    assert_eq!(
        maximum.index(),
        usize::MAX,
    );

    assert_eq!(
        maximum.checked_next(),
        None,
        "physical identifier increment must reject overflow rather than wrap"
    );
}

// =============================================================================
// Namespace/storage separation
// =============================================================================

/// A very large finite logical namespace must not require construction of one
/// `Qubit` object per logical qubit.
///
/// The circuit constructor's contract is that namespace declaration is metadata
/// rather than materialized per-qubit storage.
#[test]
fn very_large_namespace_is_representable_without_per_qubit_construction() {
    let limits = QuantumIrLimits::unbounded();

    let circuit = QuantumCircuit::try_new_with_limits(
        usize::MAX,
        0,
        limits,
    )
    .expect(
        "an unbounded policy must permit the representable logical namespace \
         boundary"
    );

    assert_eq!(
        circuit.num_qubits(),
        usize::MAX,
    );

    assert_eq!(
        circuit.len(),
        0,
        "declaring a namespace must not create operations"
    );
}

/// A large namespace containing one operation must remain sparse.
///
/// This protects against implementations that silently materialize the entire
/// namespace merely because one operation references a qubit.
#[test]
fn huge_namespace_with_sparse_semantic_content_remains_sparse() {
    let limits = QuantumIrLimits::unbounded();

    let mut circuit = QuantumCircuit::try_new_with_limits(
        usize::MAX,
        0,
        limits,
    )
    .expect("large sparse namespace must be representable");

    let highest_representable_qubit =
        QubitId::new(usize::MAX - 1);

    circuit
        .push(unary_gate(
            GateKind::X,
            highest_representable_qubit,
        ))
        .expect(
            "the highest in-range logical qubit must be usable without \
             materializing the namespace"
        );

    assert_eq!(
        circuit.num_qubits(),
        usize::MAX,
    );

    assert_eq!(
        circuit.len(),
        1,
    );
}

/// A one-qubit circuit and a huge sparse namespace use the same semantic
/// representation contract: the namespace is a declaration, not an array of
/// allocated qubit objects.
#[test]
fn namespace_size_does_not_change_empty_operation_count() {
    let small = QuantumCircuit::try_new_with_limits(
        1,
        0,
        QuantumIrLimits::unbounded(),
    )
    .expect("small circuit must be valid");

    let large = QuantumCircuit::try_new_with_limits(
        usize::MAX,
        0,
        QuantumIrLimits::unbounded(),
    )
    .expect("large circuit must be valid");

    assert_eq!(
        small.len(),
        large.len(),
    );

    assert!(small.is_empty());
    assert!(large.is_empty());
}

// =============================================================================
// Namespace boundary correctness
// =============================================================================

/// The final valid logical identifier is `num_qubits - 1`.
#[test]
fn final_namespace_identifier_is_valid() {
    let count = 8usize;

    let mut circuit = circuit(
        count,
        bounded_test_limits(),
    );

    let final_qubit = QubitId::new(
        count
            .checked_sub(1)
            .expect("test namespace must contain a qubit"),
    );

    circuit
        .push(unary_gate(
            GateKind::X,
            final_qubit,
        ))
        .expect("last namespace identifier must be valid");

    assert_eq!(
        circuit.len(),
        1,
    );
}

/// The first identifier outside the logical namespace must be rejected.
#[test]
fn_first_identifier_outside_namespace_is_rejected() {
    let count = 8usize;

    let mut circuit = circuit(
        count,
        bounded_test_limits(),
    );

    let out_of_range = QubitId::new(count);

    let result = circuit.push(
        unary_gate(
            GateKind::X,
            out_of_range,
        ),
    );

    assert!(
        matches!(
            result,
            Err(CircuitError::QubitOutOfRange {
                qubit,
                num_qubits,
            }) if qubit == out_of_range && num_qubits == count
        ),
        "the first identifier outside the namespace must be rejected"
    );

    assert_eq!(
        circuit.len(),
        0,
        "failed insertion must not partially modify the circuit"
    );
}

// =============================================================================
// Policy versus architecture
// =============================================================================

/// Tightening a policy must reject more workloads without changing the
/// underlying semantic identity model.
#[test]
fn explicit_policy_can_be_tightened_without_changing_qubit_identity() {
    let permissive = QuantumIrLimits::unbounded();

    let mut large = QuantumCircuit::try_new_with_limits(
        64,
        0,
        permissive,
    )
    .expect("permissive policy must allow the test namespace");

    let target = QubitId::new(63);

    large
        .push(unary_gate(
            GateKind::X,
            target,
        ))
        .expect("target qubit must be valid under permissive policy");

    assert_eq!(
        large.num_qubits(),
        64,
    );

    let restrictive = bounded_test_limits()
        .with_max_qubits(1);

    let result = QuantumCircuit::try_new_with_limits(
        64,
        0,
        restrictive,
    );

    assert!(
        matches!(
            result,
            Err(CircuitError::QubitLimitExceeded {
                requested: 64,
                maximum: 1,
            })
        ),
        "the restrictive policy must reject the namespace explicitly"
    );
}

/// An unbounded policy does not mean infinite memory. It merely removes the
/// finite application-level policy ceiling.
///
/// This test deliberately uses the representational boundary rather than
/// pretending that the process can allocate an infinite structure.
#[test]
fn unbounded_policy_means_no_application_ceiling_not_infinite_resources() {
    let limits = QuantumIrLimits::unbounded();

    limits
        .validate()
        .expect("unbounded policy itself must be structurally valid");

    let circuit = QuantumCircuit::try_new_with_limits(
        usize::MAX,
        0,
        limits,
    )
    .expect("representable namespace must be accepted by unbounded policy");

    assert_eq!(
        circuit.num_qubits(),
        usize::MAX,
    );
}

// =============================================================================
// Operation scaling
// =============================================================================

/// Operation storage must scale with actual operations, not declared qubit
/// namespace size.
#[test]
fn operation_storage_contract_is_independent_of_declared_namespace() {
    let limits = bounded_test_limits()
        .with_max_qubits(usize::MAX);

    let mut small = QuantumCircuit::try_new_with_limits(
        1,
        0,
        limits,
    )
    .expect("small namespace must be valid");

    let mut large = QuantumCircuit::try_new_with_limits(
        usize::MAX,
        0,
        limits,
    )
    .expect("large namespace must be valid");

    small
        .push(unary_gate(
            GateKind::X,
            QubitId::new(0),
        ))
        .expect("small operation must fit");

    large
        .push(unary_gate(
            GateKind::X,
            QubitId::new(0),
        ))
        .expect("large sparse operation must fit");

    assert_eq!(
        small.len(),
        large.len(),
    );

    assert_eq!(
        small.num_qubits(),
        1,
    );

    assert_eq!(
        large.num_qubits(),
        usize::MAX,
    );
}

/// Increasing the number of operations must be controlled by the explicit
/// operation policy rather than by a hidden machine-size rule.
#[test]
fn operation_limit_is_independent_of_qubit_namespace_size() {
    let limits = bounded_test_limits()
        .with_max_qubits(usize::MAX)
        .with_max_operations(2);

    let mut circuit = QuantumCircuit::try_new_with_limits(
        usize::MAX,
        0,
        limits,
    )
    .expect("large namespace must be permitted");

    circuit
        .push(unary_gate(
            GateKind::X,
            QubitId::new(0),
        ))
        .expect("first operation must fit");

    circuit
        .push(unary_gate(
            GateKind::H,
            QubitId::new(1),
        ))
        .expect("second operation must fit");

    let result = circuit.push(
        unary_gate(
            GateKind::Z,
            QubitId::new(2),
        ),
    );

    assert!(
        matches!(
            result,
            Err(CircuitError::OperationLimitExceeded {
                requested: 3,
                maximum: 2,
            })
        ),
        "operation policy must reject only the requested resource"
    );

    assert_eq!(
        circuit.len(),
        2,
        "failed operation insertion must be atomic"
    );
}

// =============================================================================
// Sparse high-index operations
// =============================================================================

/// High-index operations must work without requiring contiguous logical
/// operation use.
#[test]
fn sparse_operations_can_reference_noncontiguous_logical_ids() {
    let limits = QuantumIrLimits::unbounded();

    let mut circuit = QuantumCircuit::try_new_with_limits(
        usize::MAX,
        0,
        limits,
    )
    .expect("large namespace must be valid");

    let first = QubitId::new(0);
    let middle = QubitId::new(
        usize::MAX / 2,
    );
    let last = QubitId::new(
        usize::MAX - 1,
    );

    circuit
        .push(unary_gate(
            GateKind::X,
            first,
        ))
        .expect("first sparse operation must fit");

    circuit
        .push(unary_gate(
            GateKind::H,
            middle,
        ))
        .expect("middle sparse operation must fit");

    circuit
        .push(unary_gate(
            GateKind::Z,
            last,
        ))
        .expect("last sparse operation must fit");

    assert_eq!(
        circuit.len(),
        3,
    );

    assert_eq!(
        circuit.num_qubits(),
        usize::MAX,
    );
}

/// A binary operation can span distant logical identifiers without implying
/// physical adjacency or requiring a physical topology.
#[test]
fn distant_logical_operands_remain_valid_semantically() {
    let limits = QuantumIrLimits::unbounded();

    let mut circuit = QuantumCircuit::try_new_with_limits(
        usize::MAX,
        0,
        limits,
    )
    .expect("large namespace must be valid");

    let first = QubitId::new(0);
    let second = QubitId::new(
        usize::MAX - 1,
    );

    circuit
        .push(binary_gate(
            GateKind::CX,
            first,
            second,
        ))
        .expect(
            "logical distance must not be interpreted as a hardware-topology \
             restriction by the IR"
        );

    assert_eq!(
        circuit.len(),
        1,
    );
}

// =============================================================================
// Validation scalability
// =============================================================================

/// Whole-circuit validation must validate actual semantic content rather than
/// enumerating every declared logical qubit.
///
/// A huge sparse namespace with one operation is therefore a critical
/// regression case.
#[test]
fn validation_handles_huge_sparse_namespace() {
    let limits = QuantumIrLimits::unbounded();

    let mut circuit = QuantumCircuit::try_new_with_limits(
        usize::MAX,
        0,
        limits,
    )
    .expect("large sparse namespace must be constructible");

    circuit
        .push(unary_gate(
            GateKind::X,
            QubitId::new(usize::MAX - 1),
        ))
        .expect("sparse operation must be valid");

    validate_circuit_with_limits(
        &circuit,
        circuit.limits(),
    )
    .expect(
        "validation must succeed without enumerating the entire logical \
         namespace"
    );
}

/// Empty huge namespaces must also validate successfully.
#[test]
fn validation_handles_huge_empty_namespace() {
    let limits = QuantumIrLimits::unbounded();

    let circuit = QuantumCircuit::try_new_with_limits(
        usize::MAX,
        0,
        limits,
    )
    .expect("huge empty namespace must be constructible");

    validate_circuit_with_limits(
        &circuit,
        circuit.limits(),
    )
    .expect(
        "empty huge namespace must validate without per-qubit allocation"
    );
}

// =============================================================================
// Analysis scalability
// =============================================================================

/// Analysis must be able to inspect a huge sparse namespace because analysis
/// should scale with semantic content rather than namespace cardinality.
#[test]
fn analysis_handles_huge_sparse_namespace() {
    let limits = QuantumIrLimits::unbounded();

    let mut circuit = QuantumCircuit::try_new_with_limits(
        usize::MAX,
        0,
        limits,
    )
    .expect("large sparse namespace must be constructible");

    circuit
        .push(unary_gate(
            GateKind::X,
            QubitId::new(usize::MAX - 1),
        ))
        .expect("sparse operation must be valid");

    let statistics = analyze(&circuit)
        .expect(
            "analysis must handle huge sparse namespaces without allocating \
             one entry per logical qubit"
        );

    assert_eq!(
        statistics.num_qubits,
        usize::MAX,
    );

    assert_eq!(
        statistics.operation_count,
        1,
    );
}

/// Explicit analysis limits must constrain work independently from the
/// declared logical namespace.
#[test]
fn analysis_policy_is_independent_of_namespace_size() {
    let limits = QuantumIrLimits::unbounded()
        .with_max_analysis_steps(1);

    let mut circuit = QuantumCircuit::try_new_with_limits(
        usize::MAX,
        0,
        limits,
    )
    .expect("large sparse namespace must be constructible");

    circuit
        .push(unary_gate(
            GateKind::X,
            QubitId::new(usize::MAX - 1),
        ))
        .expect("sparse operation must be valid");

    let result = analyze_with_limits(
        &circuit,
        circuit.limits(),
    );

    // The exact error representation is owned by the analysis/circuit
    // contract. The important scalability invariant is that the operation
    // does not attempt to enumerate `usize::MAX` qubits.
    assert!(
        result.is_err(),
        "an explicitly insufficient analysis-work policy must be rejected"
    );
}

// =============================================================================
// Namespace monotonicity
// =============================================================================

/// Increasing a namespace must not invalidate an operation that was already
/// valid at the smaller namespace, provided the referenced identifiers remain
/// in range.
#[test]
fn increasing_namespace_preserves_existing_low_index_operations() {
    let small_limits = QuantumIrLimits::unbounded();

    let mut small = QuantumCircuit::try_new_with_limits(
        1,
        0,
        small_limits,
    )
    .expect("small circuit must be valid");

    small
        .push(unary_gate(
            GateKind::X,
            QubitId::new(0),
        ))
        .expect("operation must be valid");

    let mut larger = QuantumCircuit::try_new_with_limits(
        2,
        0,
        QuantumIrLimits::unbounded(),
    )
    .expect("larger circuit must be valid");

    larger
        .push(unary_gate(
            GateKind::X,
            QubitId::new(0),
        ))
        .expect("same low-index operation must remain valid");

    assert_eq!(
        small.len(),
        larger.len(),
    );
}

// =============================================================================
// Boundary arithmetic
// =============================================================================

/// Calculating the last valid qubit identifier must use checked arithmetic.
#[test]
fn last_identifier_calculation_is_overflow_safe() {
    let count = usize::MAX;

    let last = count
        .checked_sub(1)
        .expect("maximum representable namespace has a valid final index");

    assert_eq!(
        last,
        usize::MAX - 1,
    );

    let qubit = QubitId::new(last);

    assert_eq!(
        qubit.index(),
        usize::MAX - 1,
    );
}

/// An empty namespace must not require subtraction from zero.
#[test]
fn empty_namespace_has_no_last_identifier() {
    let count = 0usize;

    assert_eq!(
        count.checked_sub(1),
        None,
    );
}

// =============================================================================
// Explicit finite scaling points
// =============================================================================

/// Tests several finite namespace sizes while treating every value as a test
/// workload rather than an architectural maximum.
///
/// The values are generated from the platform integer width rather than
/// encoding assumptions such as "64 qubits" into the implementation contract.
#[test]
fn finite_namespace_sizes_follow_one_semantic_model() {
    let bit_width = usize::BITS as usize;

    // These are deliberately tiny relative to the platform representable
    // domain. They exercise the same constructor with progressively larger
    // finite namespaces without allocating a per-qubit representation.
    let mut sizes = Vec::new();

    sizes.push(0usize);

    if bit_width > 1 {
        sizes.push(1usize);
    }

    if bit_width > 2 {
        sizes.push(2usize);
    }

    if bit_width > 3 {
        sizes.push(
            1usize
                .checked_shl(
                    (bit_width.min(8) - 1) as u32,
                )
                .unwrap_or(1),
        );
    }

    sizes.push(
        usize::MAX,
    );

    for size in sizes {
        let circuit = QuantumCircuit::try_new_with_limits(
            size,
            0,
            QuantumIrLimits::unbounded(),
        )
        .expect(
            "every finite representable namespace must use the same \
             constructor contract"
        );

        assert_eq!(
            circuit.num_qubits(),
            size,
            "namespace size must be preserved exactly"
        );

        assert_eq!(
            circuit.len(),
            0,
            "namespace declaration must not materialize operations"
        );
    }
}

// =============================================================================
// Operation-count growth without namespace growth
// =============================================================================

/// Operation growth is independent of namespace growth.
///
/// This deliberately constructs only a modest number of operations. The test
/// is about the scaling dimension, not about exhausting machine memory.
#[test]
fn operation_growth_is_separate_from_namespace_growth() {
    let operation_count = 32usize;

    let limits = QuantumIrLimits::unbounded()
        .with_max_operations(operation_count);

    let mut circuit = QuantumCircuit::try_new_with_limits(
        usize::MAX,
        0,
        limits,
    )
    .expect("large sparse namespace must be valid");

    for index in 0..operation_count {
        let qubit_index = index % 2;

        circuit
            .push(unary_gate(
                if index % 2 == 0 {
                    GateKind::X
                } else {
                    GateKind::H
                },
                QubitId::new(qubit_index),
            ))
            .expect("operation must fit explicit policy");
    }

    assert_eq!(
        circuit.len(),
        operation_count,
    );

    assert_eq!(
        circuit.num_qubits(),
        usize::MAX,
    );
}

// =============================================================================
// Atomic failure under scaling
// =============================================================================

/// Failed operations against a huge namespace must remain atomic.
#[test]
fn failed_high_index_operation_does_not_modify_huge_sparse_circuit() {
    let limits = QuantumIrLimits::unbounded();

    let mut circuit = QuantumCircuit::try_new_with_limits(
        usize::MAX,
        0,
        limits,
    )
    .expect("large namespace must be valid");

    circuit
        .push(unary_gate(
            GateKind::X,
            QubitId::new(usize::MAX - 1),
        ))
        .expect("valid high-index operation must fit");

    let before = circuit.len();

    let invalid = QubitId::new(usize::MAX);

    let result = circuit.push(
        unary_gate(
            GateKind::H,
            invalid,
        ),
    );

    assert!(
        result.is_err(),
        "identifier equal to namespace size must be rejected"
    );

    assert_eq!(
        circuit.len(),
        before,
        "failed insertion must not modify the operation sequence"
    );
}

// =============================================================================
// Policy monotonicity
// =============================================================================

/// A policy that allows a workload must continue to allow it when only an
/// unrelated larger namespace is selected, assuming no other policy changes.
#[test]
fn larger_namespace_does_not_reduce_operation_validity() {
    let limits = QuantumIrLimits::unbounded();

    let mut smaller = QuantumCircuit::try_new_with_limits(
        2,
        0,
        limits,
    )
    .expect("smaller namespace must be valid");

    let mut larger = QuantumCircuit::try_new_with_limits(
        1024,
        0,
        limits,
    )
    .expect("larger namespace must be valid");

    let operation = unary_gate(
        GateKind::X,
        QubitId::new(1),
    );

    smaller
        .push(operation.clone())
        .expect("operation must fit smaller namespace");

    larger
        .push(operation)
        .expect("same operation must fit larger namespace");

    assert_eq!(
        smaller.len(),
        1,
    );

    assert_eq!(
        larger.len(),
        1,
    );
}

// =============================================================================
// Representational boundary
// =============================================================================

/// The IR must not wrap the logical identifier when moving beyond the
/// representable platform index domain.
#[test]
fn representational_boundary_is_rejected_by_identifier_arithmetic() {
    let maximum = QubitId::new(usize::MAX);

    assert!(
        maximum.checked_next().is_none(),
        "the IR must never wrap a logical identifier"
    );
}

/// A representable namespace can contain the highest valid index while the
/// namespace size itself remains one greater than that index.
#[test]
fn maximum_valid_index_and_namespace_size_are_distinct() {
    let namespace_size = usize::MAX;

    let highest_valid_index = namespace_size
        .checked_sub(1)
        .expect("maximum namespace has a final valid index");

    let qubit = QubitId::new(
        highest_valid_index,
    );

    assert_eq!(
        qubit.index(),
        namespace_size - 1,
    );
}

// =============================================================================
// Final scalability contract
// =============================================================================

/// This is the central regression test for the "write once, scale anywhere"
/// requirement.
///
/// The semantic program remains identical while only the declared logical
/// namespace changes.
///
/// No target, topology, vendor, machine size, routing policy, or physical
/// allocation is introduced into the IR.
#[test]
fn one_semantic_program_scales_across_namespace_sizes() {
    let logical_operation = unary_gate(
        GateKind::H,
        QubitId::new(0),
    );

    let namespaces = [
        1usize,
        2usize,
        4usize,
        8usize,
    ];

    for namespace_size in namespaces {
        let mut circuit = QuantumCircuit::try_new_with_limits(
            namespace_size,
            0,
            QuantumIrLimits::unbounded(),
        )
        .expect("namespace must be representable");

        circuit
            .push(logical_operation.clone())
            .expect(
                "the same logical program must remain valid as the available \
                 logical namespace grows"
            );

        assert_eq!(
            circuit.len(),
            1,
        );

        assert_eq!(
            circuit.num_qubits(),
            namespace_size,
        );
    }
}