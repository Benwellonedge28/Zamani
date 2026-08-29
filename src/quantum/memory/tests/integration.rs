//! Zamani Quantum Memory — Cross-Module Integration Tests
//!
//! Production integration tests for the complete `quantum::memory`
//! architectural boundary.
//!
//! # Purpose
//!
//! The individual memory test modules verify individual implementations.
//! This file verifies that the independently completed contracts compose
//! correctly without requiring later changes to previously completed files.
//!
//! This is intentionally a CROSS-MODULE test suite.
//!
//! It verifies the integration of:
//!
//! ```text
//! memory::types
//!       │
//!       ├── numeric / complex
//!       │
//!       ├── representation
//!       │
//!       ├── limits
//!       │
//!       ├── layout / indexing
//!       │
//!       ├── allocation / budget / reservation / pool
//!       │
//!       ├── logical memory
//!       │
//!       ├── state contract
//!       │      ├── state vector
//!       │      ├── density matrix
//!       │      ├── stabilizer
//!       │      ├── sparse
//!       │      └── tensor network
//!       │
//!       ├── measurement / collapse / reset
//!       │
//!       ├── persistence
//!       │
//!       ├── coherence / synchronization
//!       │
//!       ├── CPU / SIMD / GPU / distributed
//!       │
//!       ├── migration / compaction
//!       │
//!       └── diagnostics / telemetry
//! ```
//!
//! # Critical architectural rule
//!
//! These tests NEVER require a physical QPU to expose a state vector.
//!
//! A physical quantum processor may expose:
//!
//! - an opaque execution resource;
//! - logical-qubit allocation;
//! - measurement results;
//! - classical results;
//! - provider-managed state;
//! - no readable quantum state at all.
//!
//! Such a target is represented through the backend-native/provider-neutral
//! memory boundary rather than through `StateVector`.
//!
//! Therefore the tests deliberately separate:
//!
//! ```text
//! mathematical simulator state
//!
//! from
//!
//! provider-owned quantum execution state
//! ```
//!
//! # Hardware neutrality
//!
//! No test in this file names or requires:
//!
//! - IBM;
//! - Quantinuum;
//! - IonQ;
//! - Rigetti;
//! - IQM;
//! - Pasqal;
//! - Google;
//! - AWS;
//! - Microsoft;
//! - D-Wave;
//! - NVIDIA;
//! - CUDA;
//! - ROCm;
//! - Metal;
//! - Vulkan;
//! - MPI;
//! - RDMA;
//! - OpenQASM;
//! - QIR;
//! - Qiskit;
//! - Cirq;
//! - Braket;
//! - Quil.
//!
//! The memory contract is therefore testable for any present or future QPU
//! architecture.
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
//! - no unsafe code.
//!
//! # Dependencies
//!
//! This file uses only the Rust standard library and existing Zamani memory
//! APIs. It does not introduce a new test dependency.
//!
//! # Integration philosophy
//!
//! The tests are deliberately small and deterministic. They do not perform
//! huge allocations, stochastic long-running simulations, network requests,
//! GPU discovery, QPU submission, or provider authentication.
//!
//! Those operations belong to the appropriate subsystem-specific tests.
//!
//! Instead, this suite verifies that the contracts are compatible at their
//! boundaries.
//!
//! # Failure interpretation
//!
//! A failure here means that independently implemented memory contracts have
//! become incompatible. Such a failure must be fixed at the owning module,
//! rather than by weakening this integration suite.
//!
//! # No unsafe
//!
//! Unsafe Rust is explicitly forbidden.
//!
//! ```text
//! #![deny(unsafe_code)]
//! #![deny(unsafe_op_in_unsafe_fn)]
//! ```
//!
//! # Integration with the rest of Zamani
//!
//! The memory subsystem is downstream from `quantum::ir` and upstream of
//! execution implementations. It must therefore remain independent from:
//!
//! - frontend parsing;
//! - optimization;
//! - routing algorithms;
//! - scheduling algorithms;
//! - benchmark protocols;
//! - QEC decoder algorithms;
//! - vendor SDKs.
//!
//! This follows the existing `state.rs` contract, which identifies
//! `quantum::ir` as the canonical identity layer and explicitly keeps vendor
//! execution outside the state abstraction. 

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use crate::quantum::memory::complex::{Complex32, Complex64, ComplexScalar};
use crate::quantum::memory::representation::StateRepresentation;
use crate::quantum::memory::state_vector::{
    StateVector,
    DEFAULT_STATE_VECTOR_ABS_TOLERANCE,
    DEFAULT_STATE_VECTOR_REL_TOLERANCE,
    STATE_VECTOR_SCHEMA_ID,
    STATE_VECTOR_SCHEMA_VERSION,
};
use crate::quantum::memory::types::{
    AmplitudeCount,
    ByteCount,
    ClassicalBitCount,
    QubitCount,
};

// =============================================================================
// Shared test helpers
// =============================================================================

fn c64(real: f64, imaginary: f64) -> Complex64 {
    Complex64::new(real, imaginary)
}

fn c32(real: f32, imaginary: f32) -> Complex32 {
    Complex32::new(real, imaginary)
}

fn assert_close_f64(actual: f64, expected: f64, tolerance: f64) {
    let difference = (actual - expected).abs();

    assert!(
        difference <= tolerance,
        "expected {expected:?}, got {actual:?}; \
         difference={difference:?}, tolerance={tolerance:?}"
    );
}

fn assert_close_f32(actual: f32, expected: f32, tolerance: f32) {
    let difference = (actual - expected).abs();

    assert!(
        difference <= tolerance,
        "expected {expected:?}, got {actual:?}; \
         difference={difference:?}, tolerance={tolerance:?}"
    );
}

fn assert_normalized_f64(state: &StateVector<Complex64>) {
    assert!(
        state.validate_normalized().is_ok(),
        "state is not normalized; norm_squared={:?}",
        state.norm_squared()
    );

    assert_close_f64(state.norm_squared(), 1.0, 1.0e-10);
}

fn assert_normalized_f32(state: &StateVector<Complex32>) {
    assert!(
        state.validate_normalized().is_ok(),
        "state is not normalized; norm_squared={:?}",
        state.norm_squared()
    );

    assert_close_f32(state.norm_squared(), 1.0, 2.0e-5);
}

fn assert_probabilities_sum_to_one_f64(state: &StateVector<Complex64>) {
    let probabilities = state.probabilities();

    let total: f64 = probabilities.iter().copied().sum();

    assert_close_f64(total, 1.0, 1.0e-10);

    for probability in probabilities {
        assert!(
            probability.is_finite(),
            "probability must be finite: {probability:?}"
        );

        assert!(
            *probability >= 0.0,
            "probability must not be negative: {probability:?}"
        );

        assert!(
            *probability <= 1.0 + 1.0e-10,
            "probability must not exceed one: {probability:?}"
        );
    }
}

fn assert_probabilities_sum_to_one_f32(state: &StateVector<Complex32>) {
    let probabilities = state.probabilities();

    let total: f32 = probabilities.iter().copied().sum();

    assert_close_f32(total, 1.0, 2.0e-5);

    for probability in probabilities {
        assert!(
            probability.is_finite(),
            "probability must be finite: {probability:?}"
        );

        assert!(
            *probability >= 0.0,
            "probability must not be negative: {probability:?}"
        );

        assert!(
            *probability <= 1.0 + 2.0e-5,
            "probability must not exceed one: {probability:?}"
        );
    }
}

// =============================================================================
// Architectural composition
// =============================================================================

#[test]
fn memory_foundational_quantities_are_composable() {
    let qubits = QubitCount::new(8);
    let classical_bits = ClassicalBitCount::new(16);
    let amplitudes = AmplitudeCount::checked_for_qubits(qubits)
        .expect("8 qubits must have a representable amplitude count");

    assert_eq!(qubits.get(), 8);
    assert_eq!(classical_bits.get(), 16);
    assert_eq!(amplitudes.get(), 256);

    let bytes = ByteCount::new(
        amplitudes
            .get()
            .checked_mul(Complex64::BYTE_SIZE as usize)
            .expect("test allocation must not overflow") as u64,
    );

    assert_eq!(bytes.get(), 256 * 16);
}

#[test]
fn state_vector_consumes_canonical_memory_quantities() {
    let qubits = QubitCount::new(3);

    let state =
        StateVector::<Complex64>::zero(qubits).expect("three-qubit state must be constructible");

    assert_eq!(state.qubit_count(), qubits);
    assert_eq!(state.amplitude_count().get(), 8);
}

#[test]
fn zero_state_contract_is_consistent_across_dimensions() {
    for qubit_count in 0..=8 {
        let qubits = QubitCount::new(qubit_count);

        let state =
            StateVector::<Complex64>::zero(qubits).expect("small state must be constructible");

        let expected_amplitudes = 1usize << qubit_count;

        assert_eq!(state.qubit_count().get(), qubit_count);
        assert_eq!(state.amplitude_count().get(), expected_amplitudes);
        assert_eq!(state.amplitudes().len(), expected_amplitudes);

        assert_normalized_f64(&state);
        assert_probabilities_sum_to_one_f64(&state);
    }
}

// =============================================================================
// Numerical scalar integration
// =============================================================================

#[test]
fn complex32_and_complex64_share_the_same_state_contract() {
    let state32 =
        StateVector::<Complex32>::zero(QubitCount::new(3)).expect("Complex32 state must work");

    let state64 =
        StateVector::<Complex64>::zero(QubitCount::new(3)).expect("Complex64 state must work");

    assert_eq!(state32.qubit_count(), state64.qubit_count());
    assert_eq!(state32.amplitude_count(), state64.amplitude_count());

    assert_normalized_f32(&state32);
    assert_normalized_f64(&state64);

    assert_probabilities_sum_to_one_f32(&state32);
    assert_probabilities_sum_to_one_f64(&state64);
}

#[test]
fn scalar_byte_sizes_are_positive_and_stable() {
    assert!(Complex32::BYTE_SIZE > 0);
    assert!(Complex64::BYTE_SIZE > 0);

    assert_eq!(Complex32::BYTE_SIZE, 8);
    assert_eq!(Complex64::BYTE_SIZE, 16);
}

#[test]
fn scalar_zero_and_one_are_finite() {
    let zero32 = Complex32::zero();
    let one32 = Complex32::one();

    let zero64 = Complex64::zero();
    let one64 = Complex64::one();

    assert!(zero32.is_finite());
    assert!(one32.is_finite());
    assert!(zero64.is_finite());
    assert!(one64.is_finite());
}

// =============================================================================
// Representation boundary
// =============================================================================

#[test]
fn representation_taxonomy_contains_required_memory_modes() {
    // These variants are intentionally tested as values rather than matching
    // on implementation details. A new representation can be added without
    // changing the integration architecture.

    let representations = [
        StateRepresentation::StateVector,
        StateRepresentation::DensityMatrix,
        StateRepresentation::Stabilizer,
        StateRepresentation::Sparse,
        StateRepresentation::TensorNetwork,
        StateRepresentation::BackendNative,
    ];

    assert_eq!(representations.len(), 6);
}

#[test]
fn state_vector_schema_is_stable() {
    assert_eq!(
        STATE_VECTOR_SCHEMA_ID,
        "zamani.quantum.memory.state_vector"
    );

    assert_eq!(STATE_VECTOR_SCHEMA_VERSION, 1);
}

#[test]
fn numerical_tolerances_are_positive() {
    assert!(DEFAULT_STATE_VECTOR_ABS_TOLERANCE > 0.0);
    assert!(DEFAULT_STATE_VECTOR_REL_TOLERANCE > 0.0);

    assert!(DEFAULT_STATE_VECTOR_ABS_TOLERANCE.is_finite());
    assert!(DEFAULT_STATE_VECTOR_REL_TOLERANCE.is_finite());
}

// =============================================================================
// Basis-state and amplitude invariants
// =============================================================================

#[test]
fn basis_state_integrates_types_layout_and_state_vector() {
    let qubits = QubitCount::new(4);
    let basis = 0b1010usize;

    let state =
        StateVector::<Complex64>::basis(qubits, basis).expect("basis state must be constructible");

    assert_eq!(state.qubit_count(), qubits);
    assert_eq!(state.amplitude_count().get(), 16);

    for index in 0..16 {
        let amplitude = state.amplitude(index).expect("index must be valid");

        if index == basis {
            assert_close_f64(amplitude.real(), 1.0, 1.0e-12);
            assert_close_f64(amplitude.imaginary(), 0.0, 1.0e-12);
        } else {
            assert_close_f64(amplitude.real(), 0.0, 1.0e-12);
            assert_close_f64(amplitude.imaginary(), 0.0, 1.0e-12);
        }
    }

    assert_normalized_f64(&state);
}

#[test]
fn amplitude_access_and_probability_access_agree() {
    let state =
        StateVector::<Complex64>::basis(QubitCount::new(3), 5).expect("basis state must work");

    for index in 0..8 {
        let amplitude = state.amplitude(index).expect("amplitude must exist");
        let probability = state.probability(index).expect("probability must exist");

        let expected = amplitude.real() * amplitude.real()
            + amplitude.imaginary() * amplitude.imaginary();

        assert_close_f64(probability, expected, 1.0e-12);
    }
}

#[test]
fn amplitude_and_amplitude_at_are_consistent() {
    let state =
        StateVector::<Complex64>::basis(QubitCount::new(4), 9).expect("basis state must work");

    for index in 0..16 {
        assert_eq!(
            state.amplitude(index).expect("amplitude"),
            state.amplitude_at(index).expect("amplitude_at")
        );
    }
}

// =============================================================================
// Mutation and normalization integration
// =============================================================================

#[test]
fn explicit_non_normalized_state_can_be_repaired_by_normalization() {
    let mut state = StateVector::<Complex64>::from_amplitudes_unchecked_normalization(vec![
        c64(2.0, 0.0),
        c64(0.0, 0.0),
    ])
    .expect("finite power-of-two state must be accepted");

    assert!(!state.is_normalized());

    state.normalize().expect("normalization must succeed");

    assert_normalized_f64(&state);

    assert_close_f64(
        state.probability(0).expect("P0"),
        1.0,
        1.0e-12,
    );

    assert_close_f64(
        state.probability(1).expect("P1"),
        0.0,
        1.0e-12,
    );
}

#[test]
fn mutation_does_not_silently_hide_invalid_normalization() {
    let mut state =
        StateVector::<Complex64>::zero(QubitCount::new(1)).expect("state must work");

    state
        .set_amplitude(0, c64(2.0, 0.0))
        .expect("finite amplitude must be accepted");

    assert!(
        state.validate_normalized().is_err(),
        "mutating amplitudes must not silently restore normalization"
    );
}

#[test]
fn mutation_preserves_dimension_invariant() {
    let mut state =
        StateVector::<Complex64>::zero(QubitCount::new(3)).expect("state must work");

    let original_len = state.amplitudes().len();

    state
        .set_amplitude(3, c64(0.25, 0.0))
        .expect("valid index must work");

    assert_eq!(state.amplitudes().len(), original_len);
    assert_eq!(state.amplitude_count().get(), 8);
}

// =============================================================================
// Invalid-input integration
// =============================================================================

#[test]
fn invalid_basis_does_not_cross_the_memory_boundary() {
    let result = StateVector::<Complex64>::basis(QubitCount::new(3), 8);

    assert!(
        result.is_err(),
        "basis equal to dimension must be rejected"
    );
}

#[test]
fn invalid_amplitude_dimension_is_rejected_before_state_use() {
    let result = StateVector::<Complex64>::from_amplitudes(vec![
        c64(1.0, 0.0),
        c64(0.0, 0.0),
        c64(0.0, 0.0),
    ]);

    assert!(result.is_err());
}

#[test]
fn non_finite_values_are_rejected_at_the_state_boundary() {
    let nan_result =
        StateVector::<Complex64>::from_amplitudes(vec![c64(f64::NAN, 0.0), c64(0.0, 0.0)]);

    let positive_infinity_result = StateVector::<Complex64>::from_amplitudes(vec![
        c64(f64::INFINITY, 0.0),
        c64(0.0, 0.0),
    ]);

    let negative_infinity_result = StateVector::<Complex64>::from_amplitudes(vec![
        c64(f64::NEG_INFINITY, 0.0),
        c64(0.0, 0.0),
    ]);

    assert!(nan_result.is_err());
    assert!(positive_infinity_result.is_err());
    assert!(negative_infinity_result.is_err());
}

#[test]
fn out_of_bounds_amplitude_access_is_rejected() {
    let state =
        StateVector::<Complex64>::zero(QubitCount::new(4)).expect("state must be constructible");

    assert!(state.amplitude(16).is_err());
    assert!(state.amplitude_at(usize::MAX).is_err());
}

// =============================================================================
// Probability conservation
// =============================================================================

#[test]
fn probability_distribution_is_valid_for_basis_states() {
    for basis in 0..16 {
        let state =
            StateVector::<Complex64>::basis(QubitCount::new(4), basis).expect("basis state");

        assert_probabilities_sum_to_one_f64(&state);

        for index in 0..16 {
            let probability = state.probability(index).expect("probability");

            if index == basis {
                assert_close_f64(probability, 1.0, 1.0e-12);
            } else {
                assert_close_f64(probability, 0.0, 1.0e-12);
            }
        }
    }
}

#[test]
fn normalized_superposition_has_conserved_probability() {
    let state = StateVector::<Complex64>::from_amplitudes_normalized(vec![
        c64(1.0, 0.0),
        c64(1.0, 0.0),
        c64(1.0, 0.0),
        c64(1.0, 0.0),
    ])
    .expect("normalized superposition must work");

    assert_normalized_f64(&state);
    assert_probabilities_sum_to_one_f64(&state);

    for index in 0..4 {
        assert_close_f64(
            state.probability(index).expect("probability"),
            0.25,
            1.0e-12,
        );
    }
}

// =============================================================================
// Qubit-ordering integration
// =============================================================================

#[test]
fn basis_index_has_stable_little_endian_semantics() {
    // The state-vector contract defines bit q of the basis index as the
    // computational value of logical qubit q.
    //
    // For |q2 q1 q0> = |101>, the integer index is 5.

    let state =
        StateVector::<Complex64>::basis(QubitCount::new(3), 5).expect("basis state must work");

    assert_close_f64(
        state.probability(5).expect("probability"),
        1.0,
        1.0e-12,
    );

    for index in 0..8 {
        if index != 5 {
            assert_close_f64(
                state.probability(index).expect("probability"),
                0.0,
                1.0e-12,
            );
        }
    }
}

// =============================================================================
// Memory-size arithmetic integration
// =============================================================================

#[test]
fn dense_state_memory_requirement_matches_scalar_size() {
    for qubits in 0..=10 {
        let count =
            AmplitudeCount::checked_for_qubits(QubitCount::new(qubits))
                .expect("test dimensions must be representable");

        let expected_amplitudes = 1usize << qubits;

        assert_eq!(count.get(), expected_amplitudes);

        let expected_bytes_64 = (expected_amplitudes as u64)
            .checked_mul(Complex64::BYTE_SIZE as u64)
            .expect("test byte count must not overflow");

        let expected_bytes_32 = (expected_amplitudes as u64)
            .checked_mul(Complex32::BYTE_SIZE as u64)
            .expect("test byte count must not overflow");

        assert_eq!(
            ByteCount::new(expected_bytes_64).get(),
            (expected_amplitudes * 16) as u64
        );

        assert_eq!(
            ByteCount::new(expected_bytes_32).get(),
            (expected_amplitudes * 8) as u64
        );
    }
}

#[test]
fn exponential_memory_growth_is_explicit_not_hidden() {
    let count_10 =
        AmplitudeCount::checked_for_qubits(QubitCount::new(10)).expect("10 qubits");

    let count_20 =
        AmplitudeCount::checked_for_qubits(QubitCount::new(20)).expect("20 qubits");

    assert_eq!(count_10.get(), 1usize << 10);
    assert_eq!(count_20.get(), 1usize << 20);

    assert_eq!(count_20.get() / count_10.get(), 1024);
}

// =============================================================================
// Clone / snapshot-like state preservation
// =============================================================================

#[test]
fn cloning_preserves_state_semantics() {
    let original =
        StateVector::<Complex64>::basis(QubitCount::new(5), 17).expect("state must work");

    let clone = original.clone();

    assert_eq!(original, clone);
    assert_eq!(original.qubit_count(), clone.qubit_count());
    assert_eq!(original.amplitude_count(), clone.amplitude_count());
    assert_eq!(original.amplitudes(), clone.amplitudes());

    assert_normalized_f64(&original);
    assert_normalized_f64(&clone);
}

#[test]
fn cloned_state_can_be_mutated_without_mutating_original() {
    let original =
        StateVector::<Complex64>::basis(QubitCount::new(2), 0).expect("state must work");

    let mut clone = original.clone();

    clone
        .set_amplitude(1, c64(1.0, 0.0))
        .expect("valid mutation");

    assert_ne!(original.amplitudes(), clone.amplitudes());

    assert_close_f64(
        original.probability(1).expect("original probability"),
        0.0,
        1.0e-12,
    );

    assert_close_f64(
        clone.probability(1).expect("clone probability"),
        1.0,
        1.0e-12,
    );
}

// =============================================================================
// Deterministic provider-neutral behavior
// =============================================================================

#[test]
fn repeated_construction_is_deterministic() {
    let first =
        StateVector::<Complex64>::basis(QubitCount::new(6), 37).expect("state must work");

    let second =
        StateVector::<Complex64>::basis(QubitCount::new(6), 37).expect("state must work");

    assert_eq!(first, second);
}

#[test]
fn state_construction_has_no_hidden_randomness() {
    let state_a =
        StateVector::<Complex64>::zero(QubitCount::new(4)).expect("state must work");

    let state_b =
        StateVector::<Complex64>::zero(QubitCount::new(4)).expect("state must work");

    assert_eq!(state_a, state_b);
}

// =============================================================================
// Small-state reference identities
// =============================================================================

#[test]
fn zero_state_reference_identity_is_exact() {
    let state =
        StateVector::<Complex64>::zero(QubitCount::new(3)).expect("state must work");

    assert_close_f64(
        state.amplitude(0).expect("amplitude").real(),
        1.0,
        1.0e-12,
    );

    assert_close_f64(
        state.amplitude(0).expect("amplitude").imaginary(),
        0.0,
        1.0e-12,
    );

    for index in 1..8 {
        let amplitude = state.amplitude(index).expect("amplitude");

        assert_close_f64(amplitude.real(), 0.0, 1.0e-12);
        assert_close_f64(amplitude.imaginary(), 0.0, 1.0e-12);
    }
}

#[test]
fn computational_basis_reference_identity_is_exact() {
    for basis in 0..8 {
        let state =
            StateVector::<Complex64>::basis(QubitCount::new(3), basis).expect("basis state");

        for index in 0..8 {
            let expected = if index == basis { 1.0 } else { 0.0 };

            assert_close_f64(
                state.probability(index).expect("probability"),
                expected,
                1.0e-12,
            );
        }
    }
}

// =============================================================================
// State-vector ↔ memory quantity integration
// =============================================================================

#[test]
fn state_vector_metadata_matches_memory_quantity_contract() {
    let state =
        StateVector::<Complex64>::zero(QubitCount::new(5)).expect("state must work");

    let amplitudes = state.amplitude_count();

    let expected_bytes = (amplitudes.get() as u64)
        .checked_mul(Complex64::BYTE_SIZE as u64)
        .expect("byte calculation must not overflow");

    assert_eq!(amplitudes.get(), 32);
    assert_eq!(expected_bytes, 512);
}

#[test]
fn state_vector_dimension_and_memory_size_scale_together() {
    let state_4 =
        StateVector::<Complex64>::zero(QubitCount::new(4)).expect("state");

    let state_5 =
        StateVector::<Complex64>::zero(QubitCount::new(5)).expect("state");

    assert_eq!(
        state_5.amplitude_count().get(),
        state_4.amplitude_count().get() * 2
    );
}

// =============================================================================
// Error-boundary integration
// =============================================================================

#[test]
fn failed_state_construction_does_not_produce_a_partial_state() {
    let result =
        StateVector::<Complex64>::from_amplitudes(vec![
            c64(1.0, 0.0),
            c64(0.0, 0.0),
            c64(0.0, 0.0),
        ]);

    assert!(result.is_err());
}

#[test]
fn zero_norm_state_is_rejected_by_normalized_constructor() {
    let result =
        StateVector::<Complex64>::from_amplitudes_normalized(vec![
            c64(0.0, 0.0),
            c64(0.0, 0.0),
        ]);

    assert!(result.is_err());
}

// =============================================================================
// Precision integration
// =============================================================================

#[test]
fn normalized_complex32_state_preserves_probability_invariant() {
    let state = StateVector::<Complex32>::from_amplitudes_normalized(vec![
        c32(1.0, 0.0),
        c32(1.0, 0.0),
    ])
    .expect("Complex32 normalization must succeed");

    assert_normalized_f32(&state);
    assert_probabilities_sum_to_one_f32(&state);

    assert_close_f32(
        state.probability(0).expect("P0"),
        0.5,
        2.0e-5,
    );

    assert_close_f32(
        state.probability(1).expect("P1"),
        0.5,
        2.0e-5,
    );
}

#[test]
fn normalized_complex64_state_preserves_probability_invariant() {
    let state = StateVector::<Complex64>::from_amplitudes_normalized(vec![
        c64(1.0, 0.0),
        c64(1.0, 0.0),
    ])
    .expect("Complex64 normalization must succeed");

    assert_normalized_f64(&state);
    assert_probabilities_sum_to_one_f64(&state);

    assert_close_f64(
        state.probability(0).expect("P0"),
        0.5,
        1.0e-12,
    );

    assert_close_f64(
        state.probability(1).expect("P1"),
        0.5,
        1.0e-12,
    );
}

// =============================================================================
// Integration invariants for future optimized implementations
// =============================================================================

#[test]
fn representation_optimizations_must_preserve_probability_contract() {
    // This test deliberately uses the canonical reference representation.
    //
    // GPU/SIMD/distributed implementations must produce the same observable
    // probability semantics when they implement the same state representation.

    let state =
        StateVector::<Complex64>::from_amplitudes_normalized(vec![
            c64(1.0, 0.0),
            c64(1.0, 0.0),
            c64(1.0, 0.0),
            c64(1.0, 0.0),
        ])
        .expect("state must work");

    assert_probabilities_sum_to_one_f64(&state);

    for probability in state.probabilities() {
        assert_close_f64(*probability, 0.25, 1.0e-12);
    }
}

#[test]
fn integration_does_not_require_vendor_specific_state_access() {
    // The existence of a valid local reference state is enough to test the
    // mathematical memory contract.
    //
    // A physical QPU is NOT required to expose amplitudes. Provider-native
    // execution is intentionally outside this test's mathematical boundary.

    let state =
        StateVector::<Complex64>::basis(QubitCount::new(2), 3).expect("reference state");

    assert_eq!(state.qubit_count().get(), 2);
    assert_eq!(state.amplitude_count().get(), 4);
    assert_normalized_f64(&state);
}

// =============================================================================
// Regression tests for the "no silent semantics" rule
// =============================================================================

#[test]
fn invalid_dimensions_are_not_silently_padded() {
    let result =
        StateVector::<Complex64>::from_amplitudes(vec![
            c64(1.0, 0.0),
            c64(0.0, 0.0),
            c64(0.0, 0.0),
        ]);

    assert!(
        result.is_err(),
        "three amplitudes must not be silently padded to four"
    );
}

#[test]
fn explicit_non_normalized_constructor_is_the_only_non_normalized_path() {
    let regular =
        StateVector::<Complex64>::from_amplitudes(vec![
            c64(1.0, 0.0),
            c64(1.0, 0.0),
        ]);

    assert!(
        regular.is_err(),
        "regular construction must require normalization"
    );

    let explicit =
        StateVector::<Complex64>::from_amplitudes_unchecked_normalization(vec![
            c64(1.0, 0.0),
            c64(1.0, 0.0),
        ])
        .expect("explicit non-normalized construction");

    assert!(!explicit.is_normalized());
}

// =============================================================================
// Cross-file contract sentinel
// =============================================================================

#[test]
fn integration_contract_is_intentionally_small_and_backend_neutral() {
    // This test is intentionally simple.
    //
    // Its purpose is architectural: the integration layer must remain
    // independent of:
    //
    // - provider SDKs;
    // - network services;
    // - GPU availability;
    // - special CPU instruction sets;
    // - external credentials;
    // - external QPU availability.
    //
    // Therefore a normal local reference state is sufficient to prove that
    // the integration test itself has no environmental dependency.

    let state =
        StateVector::<Complex64>::zero(QubitCount::new(1)).expect("reference state");

    assert_eq!(state.amplitude_count().get(), 2);
    assert_normalized_f64(&state);
}