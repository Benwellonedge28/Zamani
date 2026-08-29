//! Zamani Quantum Memory — StateVector Integration Tests
//!
//! Production-grade integration tests for `memory::state_vector`.
//!
//! # Purpose
//!
//! This file verifies the externally observable contract of the canonical
//! dense pure-state representation without depending on implementation
//! details, raw pointers, unsafe code, a particular CPU instruction set,
//! accelerator API, or quantum-hardware vendor.
//!
//! The same mathematical contract must remain true when the implementation is
//! later optimized by:
//!
//! - CPU kernels;
//! - SIMD kernels;
//! - GPU/device kernels;
//! - distributed-state implementations;
//! - memory pools;
//! - migration infrastructure;
//! - hardware validation adapters.
//!
//! # Architectural boundary
//!
//! ```text
//!                     quantum::ir
//!                         |
//!                         v
//!                    executor
//!                         |
//!                         v
//!                  quantum::memory
//!                         |
//!                    StateVector
//!                         |
//!              +----------+----------+
//!              |          |          |
//!              v          v          v
//!             CPU        GPU     distributed
//!              |          |          |
//!              +----------+----------+
//!                         |
//!                         v
//!                 hardware adapters
//! ```
//!
//! These tests therefore test the mathematical state-vector contract, not a
//! particular backend.
//!
//! # Rust compatibility
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021
//! - stable Rust
//! - no nightly features
//! - no unsafe code
//! - no external test dependencies
//!
//! # Integration contract
//!
//! This test module consumes the public contracts of:
//!
//! - `memory::state_vector`;
//! - `memory::complex`;
//! - `memory::types`;
//! - `memory::errors`.
//!
//! It intentionally does not depend on:
//!
//! - GPU implementation;
//! - SIMD implementation;
//! - allocator implementation;
//! - persistence implementation;
//! - distributed implementation;
//! - hardware providers;
//! - OpenQASM;
//! - routing;
//! - scheduling;
//! - optimization;
//! - benchmarking;
//! - QEC decoders.
//!
//! Those systems must preserve the behavior tested here when they integrate
//! with StateVector.
//!
//! # Test philosophy
//!
//! The tests verify:
//!
//! 1. construction invariants;
//! 2. dimensional invariants;
//! 3. numerical validity;
//! 4. qubit-ordering semantics;
//! 5. standard gates;
//! 6. arbitrary matrix operations;
//! 7. controlled operations;
//! 8. multi-qubit operations;
//! 9. probabilities;
//! 10. measurement;
//! 11. reset;
//! 12. permutation;
//! 13. tensor products;
//! 14. inner products;
//! 15. fidelity;
//! 16. normalization;
//! 17. cloning/replacement;
//! 18. metadata;
//! 19. invalid-input handling;
//! 20. precision portability between Complex32 and Complex64;
//! 21. numerical invariants after sequences of operations;
//! 22. provider-neutral behavior.
//!
//! # Important invariant
//!
//! The tests never assume that a physical QPU exposes its wavefunction.
//! Physical QPU implementations that do not expose a state vector must use
//! another memory representation. These tests apply to the StateVector
//! representation wherever that representation is legitimately used.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use crate::quantum::memory::complex::{Complex32, Complex64, ComplexScalar};
use crate::quantum::memory::errors::MemoryError;
use crate::quantum::memory::state_vector::{
    BasisMeasurement, StateVector, StateVectorMetadata, QubitMeasurement,
    DEFAULT_STATE_VECTOR_ABS_TOLERANCE, DEFAULT_STATE_VECTOR_REL_TOLERANCE,
    MAX_INDEXABLE_QUBITS, STATE_VECTOR_SCHEMA_ID, STATE_VECTOR_SCHEMA_VERSION,
};
use crate::quantum::memory::types::QubitCount;

// =============================================================================
// Test helpers
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
        "expected {expected:?}, got {actual:?}; difference={difference:?}, tolerance={tolerance:?}"
    );
}

fn assert_close_f32(actual: f32, expected: f32, tolerance: f32) {
    let difference = (actual - expected).abs();

    assert!(
        difference <= tolerance,
        "expected {expected:?}, got {actual:?}; difference={difference:?}, tolerance={tolerance:?}"
    );
}

fn assert_complex64_close(actual: Complex64, expected: Complex64, tolerance: f64) {
    assert_close_f64(actual.real(), expected.real(), tolerance);
    assert_close_f64(actual.imaginary(), expected.imaginary(), tolerance);
}

fn assert_complex32_close(actual: Complex32, expected: Complex32, tolerance: f32) {
    assert_close_f32(actual.real(), expected.real(), tolerance);
    assert_close_f32(actual.imaginary(), expected.imaginary(), tolerance);
}

fn assert_normalized_f64(state: &StateVector<Complex64>) {
    assert!(
        state.validate_normalized().is_ok(),
        "state is not normalized: norm_squared={:?}",
        state.norm_squared()
    );

    assert_close_f64(state.norm_squared(), 1.0, 1.0e-10);
}

fn assert_normalized_f32(state: &StateVector<Complex32>) {
    assert!(
        state.validate_normalized().is_ok(),
        "state is not normalized: norm_squared={:?}",
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
            probability >= 0.0,
            "probability must not be negative: {probability:?}"
        );
        assert!(
            probability <= 1.0 + 1.0e-10,
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
            probability >= 0.0,
            "probability must not be negative: {probability:?}"
        );
        assert!(
            probability <= 1.0 + 2.0e-5,
            "probability must not exceed one: {probability:?}"
        );
    }
}

fn assert_error(result: Result<(), MemoryError>) {
    assert!(result.is_err(), "operation unexpectedly succeeded");
}

fn assert_measurement_shape<S: ComplexScalar>(
    measurement: &BasisMeasurement<S>,
    qubits: usize,
) {
    assert_eq!(measurement.bits.len(), qubits);
    assert!(measurement.probability >= S::zero().real());
    assert!(measurement.probability <= S::one().real());
}

fn assert_qubit_measurement_shape<R>(
    measurement: &QubitMeasurement<R>,
    expected_outcome: u8,
) where
    R: Copy + PartialOrd + From<u8>,
{
    assert_eq!(measurement.outcome, expected_outcome);
}

fn metadata_is_valid<S: ComplexScalar>(
    metadata: StateVectorMetadata,
    expected_qubits: usize,
    expected_amplitudes: usize,
    expected_bytes: u64,
) {
    assert_eq!(metadata.qubits.get(), expected_qubits);
    assert_eq!(metadata.amplitudes.get(), expected_amplitudes);
    assert_eq!(metadata.bytes.get(), expected_bytes);
    assert_eq!(metadata.bytes_per_amplitude, S::BYTE_SIZE);
}

// =============================================================================
// Schema and architectural contract
// =============================================================================

#[test]
fn schema_identity_is_stable() {
    assert_eq!(
        STATE_VECTOR_SCHEMA_ID,
        "zamani.quantum.memory.state_vector"
    );

    assert_eq!(STATE_VECTOR_SCHEMA_VERSION, 1);
}

#[test]
fn numerical_tolerance_contract_is_explicit() {
    assert!(DEFAULT_STATE_VECTOR_ABS_TOLERANCE > 0.0);
    assert!(DEFAULT_STATE_VECTOR_REL_TOLERANCE > 0.0);
}

#[test]
fn indexability_ceiling_is_valid() {
    assert!(MAX_INDEXABLE_QUBITS > 0);
    assert!(MAX_INDEXABLE_QUBITS < usize::BITS as usize);
}

// =============================================================================
// Construction
// =============================================================================

#[test]
fn zero_qubit_state_is_valid() {
    let state =
        StateVector::<Complex64>::zero(QubitCount::new(0)).expect("zero-qubit state must work");

    assert_eq!(state.qubit_count().get(), 0);
    assert_eq!(state.amplitude_count().get(), 1);

    assert_complex64_close(state.amplitude(0).expect("amplitude 0"), c64(1.0, 0.0), 1.0e-12);

    assert_normalized_f64(&state);
}

#[test]
fn zero_state_has_exact_power_of_two_dimension() {
    for qubits in 0..=8 {
        let state =
            StateVector::<Complex64>::zero(QubitCount::new(qubits)).expect("valid state");

        let expected = 1usize << qubits;

        assert_eq!(state.amplitude_count().get(), expected);
        assert_eq!(state.amplitudes().len(), expected);
        assert_eq!(state.qubit_count().get(), qubits);
    }
}

#[test]
fn basis_state_places_amplitude_at_exact_basis_index() {
    let state =
        StateVector::<Complex64>::basis(QubitCount::new(4), 0b1010).expect("valid basis state");

    for index in 0..16 {
        let amplitude = state.amplitude(index).expect("valid amplitude");

        if index == 0b1010 {
            assert_complex64_close(amplitude, c64(1.0, 0.0), 1.0e-12);
        } else {
            assert_complex64_close(amplitude, c64(0.0, 0.0), 1.0e-12);
        }
    }

    assert_normalized_f64(&state);
}

#[test]
fn basis_index_equal_to_dimension_is_rejected() {
    let result = StateVector::<Complex64>::basis(QubitCount::new(3), 8);

    assert!(result.is_err());
}

#[test]
fn invalid_amplitude_length_is_rejected() {
    let result = StateVector::<Complex64>::from_amplitudes(vec![
        c64(1.0, 0.0),
        c64(0.0, 0.0),
        c64(0.0, 0.0),
    ]);

    assert!(result.is_err());
}

#[test]
fn empty_amplitude_vector_is_rejected() {
    let result = StateVector::<Complex64>::from_amplitudes(Vec::new());

    assert!(result.is_err());
}

#[test]
fn non_finite_nan_amplitude_is_rejected() {
    let result =
        StateVector::<Complex64>::from_amplitudes(vec![c64(f64::NAN, 0.0), c64(0.0, 0.0)]);

    assert!(result.is_err());
}

#[test]
fn non_finite_positive_infinity_amplitude_is_rejected() {
    let result = StateVector::<Complex64>::from_amplitudes(vec![
        c64(f64::INFINITY, 0.0),
        c64(0.0, 0.0),
    ]);

    assert!(result.is_err());
}

#[test]
fn non_finite_negative_infinity_amplitude_is_rejected() {
    let result = StateVector::<Complex64>::from_amplitudes(vec![
        c64(f64::NEG_INFINITY, 0.0),
        c64(0.0, 0.0),
    ]);

    assert!(result.is_err());
}

#[test]
fn normalized_constructor_rejects_zero_vector() {
    let result =
        StateVector::<Complex64>::from_amplitudes_normalized(vec![c64(0.0, 0.0), c64(0.0, 0.0)]);

    assert!(result.is_err());
}

#[test]
fn normalized_constructor_produces_unit_norm() {
    let state = StateVector::<Complex64>::from_amplitudes_normalized(vec![
        c64(1.0, 0.0),
        c64(1.0, 0.0),
    ])
    .expect("normalization must succeed");

    assert_normalized_f64(&state);

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

#[test]
fn explicitly_non_normalized_constructor_preserves_input_without_silent_normalization() {
    let state = StateVector::<Complex64>::from_amplitudes_unchecked_normalization(vec![
        c64(1.0, 0.0),
        c64(1.0, 0.0),
    ])
    .expect("finite power-of-two input must be accepted");

    assert_close_f64(state.norm_squared(), 2.0, 1.0e-12);
    assert!(!state.is_normalized());
}

#[test]
fn normalize_repairs_finite_non_normalized_state() {
    let mut state = StateVector::<Complex64>::from_amplitudes_unchecked_normalization(vec![
        c64(2.0, 0.0),
        c64(0.0, 0.0),
    ])
    .expect("valid finite state");

    assert!(!state.is_normalized());

    state.normalize().expect("normalization must succeed");

    assert_normalized_f64(&state);
    assert_close_f64(
        state.probability(0).expect("P0"),
        1.0,
        1.0e-12,
    );
}

// =============================================================================
// Amplitude access
// =============================================================================

#[test]
fn amplitude_and_amplitude_at_are_equivalent() {
    let state =
        StateVector::<Complex64>::basis(QubitCount::new(3), 5).expect("valid state");

    for index in 0..8 {
        assert_eq!(
            state.amplitude(index).expect("amplitude"),
            state.amplitude_at(index).expect("amplitude_at")
        );
    }
}

#[test]
fn amplitude_out_of_bounds_is_rejected() {
    let state =
        StateVector::<Complex64>::zero(QubitCount::new(3)).expect("valid state");

    assert!(state.amplitude(8).is_err());
    assert!(state.amplitude_at(usize::MAX).is_err());
}

#[test]
fn set_amplitude_rejects_non_finite_values() {
    let mut state =
        StateVector::<Complex64>::zero(QubitCount::new(1)).expect("valid state");

    assert!(state
        .set_amplitude(0, c64(f64::NAN, 0.0))
        .is_err());

    assert!(state
        .set_amplitude(0, c64(f64::INFINITY, 0.0))
        .is_err());
}

#[test]
fn set_amplitude_checks_bounds() {
    let mut state =
        StateVector::<Complex64>::zero(QubitCount::new(2)).expect("valid state");

    assert!(state
        .set_amplitude(4, c64(0.0, 0.0))
        .is_err());
}

#[test]
fn mutable_amplitude_access_is_explicit() {
    let mut state =
        StateVector::<Complex64>::zero(QubitCount::new(1)).expect("valid state");

    {
        let amplitudes = state.amplitudes_mut();

        amplitudes[0] = c64(0.0, 0.0);
        amplitudes[1] = c64(1.0, 0.0);
    }

    assert_complex64_close(state.amplitude(0).expect("a0"), c64(0.0, 0.0), 1.0e-12);
    assert_complex64_close(state.amplitude(1).expect("a1"), c64(1.0, 0.0), 1.0e-12);

    assert_normalized_f64(&state);
}

// =============================================================================
// Metadata and resource accounting
// =============================================================================

#[test]
fn metadata_for_complex64_is_exact() {
    let state =
        StateVector::<Complex64>::zero(QubitCount::new(3)).expect("valid state");

    let metadata = state.metadata();

    metadata_is_valid::<Complex64>(metadata, 3, 8, 128);

    assert_eq!(
        state.required_bytes().expect("byte requirement").get(),
        128
    );
}

#[test]
fn metadata_for_complex32_is_exact() {
    let state =
        StateVector::<Complex32>::zero(QubitCount::new(3)).expect("valid state");

    let metadata = state.metadata();

    metadata_is_valid::<Complex32>(metadata, 3, 8, 64);

    assert_eq!(
        state.required_bytes().expect("byte requirement").get(),
        64
    );
}

#[test]
fn required_bytes_follow_exponential_state_vector_growth() {
    for qubits in 0..=8 {
        let state =
            StateVector::<Complex64>::zero(QubitCount::new(qubits)).expect("valid state");

        let expected_amplitudes = 1usize << qubits;
        let expected_bytes = (expected_amplitudes * 16) as u64;

        assert_eq!(
            state.required_bytes().expect("bytes").get(),
            expected_bytes
        );
    }
}

#[test]
fn dimension_overflow_is_rejected_before_allocation() {
    let impossible_qubits = QubitCount::new(usize::BITS as usize);

    let result = StateVector::<Complex64>::zero(impossible_qubits);

    assert!(result.is_err());
}

// =============================================================================
// Norms and probabilities
// =============================================================================

#[test]
fn probability_of_basis_state_is_squared_amplitude_magnitude() {
    let state = StateVector::<Complex64>::from_amplitudes_normalized(vec![
        c64(1.0, 0.0),
        c64(1.0, 0.0),
    ])
    .expect("valid state");

    assert_close_f64(state.probability(0).expect("P0"), 0.5, 1.0e-12);
    assert_close_f64(state.probability(1).expect("P1"), 0.5, 1.0e-12);
}

#[test]
fn probabilities_sum_to_one_for_uniform_two_qubit_state() {
    let mut state =
        StateVector::<Complex64>::zero(QubitCount::new(2)).expect("valid state");

    state.h(0).expect("H0");
    state.h(1).expect("H1");

    assert_probabilities_sum_to_one_f64(&state);

    for probability in state.probabilities() {
        assert_close_f64(probability, 0.25, 1.0e-12);
    }
}

#[test]
fn total_probability_equals_norm_squared() {
    let state =
        StateVector::<Complex64>::basis(QubitCount::new(4), 9).expect("valid state");

    assert_close_f64(
        state.total_probability(),
        state.norm_squared(),
        1.0e-12,
    );

    assert_close_f64(state.total_probability(), 1.0, 1.0e-12);
}

#[test]
fn probability_zero_and_one_are_complementary() {
    let mut state =
        StateVector::<Complex64>::zero(QubitCount::new(2)).expect("valid state");

    state.h(0).expect("H");

    for qubit in 0..2 {
        let zero = state.probability_zero(qubit).expect("P0");
        let one = state.probability_one(qubit).expect("P1");

        assert_close_f64(zero + one, 1.0, 1.0e-10);
    }
}

#[test]
fn invalid_probability_qubit_is_rejected() {
    let state =
        StateVector::<Complex64>::zero(QubitCount::new(2)).expect("valid state");

    assert!(state.probability_zero(2).is_err());
    assert!(state.probability_one(2).is_err());
    assert!(state.qubit_measurement_probabilities(2).is_err());
}

// =============================================================================
// Qubit ordering
// =============================================================================

#[test]
fn little_endian_q0_is_least_significant_bit() {
    let mut state =
        StateVector::<Complex64>::zero(QubitCount::new(3)).expect("valid state");

    state.x(0).expect("X q0");

    assert_close_f64(state.probability(1).expect("P001"), 1.0, 1.0e-12);
    assert_close_f64(state.probability(2).expect("P010"), 0.0, 1.0e-12);
    assert_close_f64(state.probability(4).expect("P100"), 0.0, 1.0e-12);
}

#[test]
fn little_endian_q1_is_second_bit() {
    let mut state =
        StateVector::<Complex64>::zero(QubitCount::new(3)).expect("valid state");

    state.x(1).expect("X q1");

    assert_close_f64(state.probability(2).expect("P010"), 1.0, 1.0e-12);
    assert_close_f64(state.probability(1).expect("P001"), 0.0, 1.0e-12);
    assert_close_f64(state.probability(4).expect("P100"), 0.0, 1.0e-12);
}

#[test]
fn little_endian_q2_is_most_significant_bit_for_three_qubits() {
    let mut state =
        StateVector::<Complex64>::zero(QubitCount::new(3)).expect("valid state");

    state.x(2).expect("X q2");

    assert_close_f64(state.probability(4).expect("P100"), 1.0, 1.0e-12);
}

// =============================================================================
// Pauli gates
// =============================================================================

#[test]
fn pauli_x_flips_zero_to_one() {
    let mut state =
        StateVector::<Complex64>::zero(QubitCount::new(1)).expect("valid state");

    state.x(0).expect("X");

    assert_complex64_close(state.amplitude(0).expect("a0"), c64(0.0, 0.0), 1.0e-12);
    assert_complex64_close(state.amplitude(1).expect("a1"), c64(1.0, 0.0), 1.0e-12);

    assert_normalized_f64(&state);
}

#[test]
fn pauli_x_twice_is_identity() {
    let mut state =
        StateVector::<Complex64>::basis(QubitCount::new(1), 1).expect("state");

    state.x(0).expect("X1");
    state.x(0).expect("X2");

    assert_complex64_close(state.amplitude(0).expect("a0"), c64(0.0, 0.0), 1.0e-12);
    assert_complex64_close(state.amplitude(1).expect("a1"), c64(1.0, 0.0), 1.0e-12);
}

#[test]
fn pauli_y_maps_zero_to_i_one() {
    let mut state =
        StateVector::<Complex64>::zero(QubitCount::new(1)).expect("state");

    state.y(0).expect("Y");

    assert_complex64_close(state.amplitude(0).expect("a0"), c64(0.0, 0.0), 1.0e-12);
    assert_complex64_close(state.amplitude(1).expect("a1"), c64(0.0, 1.0), 1.0e-12);

    assert_normalized_f64(&state);
}

#[test]
fn pauli_z_leaves_zero_and_negates_one() {
    let mut zero =
        StateVector::<Complex64>::basis(QubitCount::new(1), 0).expect("zero");

    zero.z(0).expect("Z");

    assert_complex64_close(zero.amplitude(0).expect("a0"), c64(1.0, 0.0), 1.0e-12);

    let mut one =
        StateVector::<Complex64>::basis(QubitCount::new(1), 1).expect("one");

    one.z(0).expect("Z");

    assert_complex64_close(one.amplitude(1).expect("a1"), c64(-1.0, 0.0), 1.0e-12);
}

// =============================================================================
// Hadamard
// =============================================================================

#[test]
fn hadamard_creates_plus_state() {
    let mut state =
        StateVector::<Complex64>::zero(QubitCount::new(1)).expect("state");

    state.h(0).expect("H");

    let scale = 1.0 / 2.0_f64.sqrt();

    assert_complex64_close(
        state.amplitude(0).expect("a0"),
        c64(scale, 0.0),
        1.0e-12,
    );

    assert_complex64_close(
        state.amplitude(1).expect("a1"),
        c64(scale, 0.0),
        1.0e-12,
    );

    assert_normalized_f64(&state);
}

#[test]
fn hadamard_twice_is_identity() {
    let mut state =
        StateVector::<Complex64>::basis(QubitCount::new(1), 1).expect("state");

    state.h(0).expect("H1");
    state.h(0).expect("H2");

    assert_complex64_close(
        state.amplitude(0).expect("a0"),
        c64(0.0, 0.0),
        1.0e-12,
    );

    assert_complex64_close(
        state.amplitude(1).expect("a1"),
        c64(1.0, 0.0),
        1.0e-12,
    );
}

// =============================================================================
// Phase and rotations
// =============================================================================

#[test]
fn phase_gate_preserves_probabilities() {
    let mut state =
        StateVector::<Complex64>::zero(QubitCount::new(1)).expect("state");

    state.h(0).expect("H");

    let before = state.probabilities();

    state.phase(0, core::f64::consts::FRAC_PI_2).expect("phase");

    let after = state.probabilities();

    assert_eq!(before.len(), after.len());

    for (left, right) in before.iter().zip(after.iter()) {
        assert_close_f64(*left, *right, 1.0e-12);
    }

    assert_normalized_f64(&state);
}

#[test]
fn rz_zero_angle_is_identity() {
    let state =
        StateVector::<Complex64>::basis(QubitCount::new(1), 1).expect("state");

    let mut transformed = state.clone();

    transformed.rz(0, 0.0).expect("RZ");

    assert_eq!(transformed, state);
}

#[test]
fn rx_zero_angle_is_identity() {
    let state =
        StateVector::<Complex64>::basis(QubitCount::new(1), 1).expect("state");

    let mut transformed = state.clone();

    transformed.rx(0, 0.0).expect("RX");

    assert_eq!(transformed, state);
}

#[test]
fn ry_zero_angle_is_identity() {
    let state =
        StateVector::<Complex64>::basis(QubitCount::new(1), 1).expect("state");

    let mut transformed = state.clone();

    transformed.ry(0, 0.0).expect("RY");

    assert_eq!(transformed, state);
}

#[test]
fn rx_pi_maps_zero_to_minus_i_one() {
    let mut state =
        StateVector::<Complex64>::zero(QubitCount::new(1)).expect("state");

    state.rx(0, core::f64::consts::PI).expect("RX(pi)");

    assert_close_f64(state.probability(0).expect("P0"), 0.0, 1.0e-10);
    assert_close_f64(state.probability(1).expect("P1"), 1.0, 1.0e-10);

    assert_close_f64(
        state.amplitude(1).expect("a1").imaginary().abs(),
        1.0,
        1.0e-10,
    );

    assert_normalized_f64(&state);
}

#[test]
fn ry_pi_maps_zero_to_one() {
    let mut state =
        StateVector::<Complex64>::zero(QubitCount::new(1)).expect("state");

    state.ry(0, core::f64::consts::PI).expect("RY(pi)");

    assert_close_f64(state.probability(0).expect("P0"), 0.0, 1.0e-10);
    assert_close_f64(state.probability(1).expect("P1"), 1.0, 1.0e-10);

    assert_normalized_f64(&state);
}

// =============================================================================
// Controlled operations
// =============================================================================

#[test]
fn cnot_does_nothing_when_control_is_zero() {
    let mut state =
        StateVector::<Complex64>::basis(QubitCount::new(2), 1).expect("state");

    state.cnot(0, 1).expect("CNOT");

    assert_close_f64(state.probability(1).expect("P01"), 1.0, 1.0e-12);
    assert_close_f64(state.probability(3).expect("P11"), 0.0, 1.0e-12);
}

#[test]
fn cnot_flips_target_when_control_is_one() {
    let mut state =
        StateVector::<Complex64>::basis(QubitCount::new(2), 1).expect("state");

    state.cnot(0, 1).expect("CNOT");

    // q0 is one in basis index 1; q1 is the target.
    // A canonical CNOT must therefore map |01> to |11>.
    assert_close_f64(state.probability(3).expect("P11"), 1.0, 1.0e-12);
}

#[test]
fn cnot_creates_bell_state() {
    let mut state =
        StateVector::<Complex64>::zero(QubitCount::new(2)).expect("state");

    state.h(0).expect("H");
    state.cnot(0, 1).expect("CNOT");

    let scale = 1.0 / 2.0_f64.sqrt();

    assert_complex64_close(
        state.amplitude(0).expect("a00"),
        c64(scale, 0.0),
        1.0e-12,
    );

    assert_complex64_close(
        state.amplitude(3).expect("a11"),
        c64(scale, 0.0),
        1.0e-12,
    );

    assert_close_f64(state.probability(1).expect("P01"), 0.0, 1.0e-12);
    assert_close_f64(state.probability(2).expect("P10"), 0.0, 1.0e-12);

    assert_normalized_f64(&state);
}

#[test]
fn controlled_z_changes_only_control_target_one_component() {
    let mut state =
        StateVector::<Complex64>::basis(QubitCount::new(2), 3).expect("state");

    state.cz(0, 1).expect("CZ");

    assert_complex64_close(
        state.amplitude(3).expect("a11"),
        c64(-1.0, 0.0),
        1.0e-12,
    );

    assert_normalized_f64(&state);
}

#[test]
fn controlled_operation_rejects_same_qubit() {
    let mut state =
        StateVector::<Complex64>::zero(QubitCount::new(2)).expect("state");

    assert!(
        state
            .cnot(0, 0)
            .is_err(),
        "control and target must be distinct"
    );

    assert!(
        state
            .cz(1, 1)
            .is_err(),
        "control and target must be distinct"
    );
}

// =============================================================================
// Arbitrary single-qubit matrix
// =============================================================================

#[test]
fn arbitrary_single_qubit_x_matrix_matches_pauli_x() {
    let matrix = [
        [c64(0.0, 0.0), c64(1.0, 0.0)],
        [c64(1.0, 0.0), c64(0.0, 0.0)],
    ];

    let mut direct =
        StateVector::<Complex64>::basis(QubitCount::new(1), 0).expect("state");

    let mut matrix_state = direct.clone();

    direct.x(0).expect("X");
    matrix_state
        .apply_single_qubit_matrix(0, matrix)
        .expect("matrix");

    assert_eq!(direct, matrix_state);
}

#[test]
fn arbitrary_single_qubit_identity_preserves_state() {
    let matrix = [
        [c64(1.0, 0.0), c64(0.0, 0.0)],
        [c64(0.0, 0.0), c64(1.0, 0.0)],
    ];

    let state =
        StateVector::<Complex64>::basis(QubitCount::new(2), 2).expect("state");

    let mut transformed = state.clone();

    transformed
        .apply_single_qubit_matrix(1, matrix)
        .expect("identity");

    assert_eq!(state, transformed);
}

#[test]
fn arbitrary_single_qubit_matrix_rejects_invalid_qubit() {
    let matrix = [
        [c64(1.0, 0.0), c64(0.0, 0.0)],
        [c64(0.0, 0.0), c64(1.0, 0.0)],
    ];

    let mut state =
        StateVector::<Complex64>::zero(QubitCount::new(1)).expect("state");

    assert!(
        state
            .apply_single_qubit_matrix(1, matrix)
            .is_err()
    );
}

// =============================================================================
// Arbitrary controlled matrix
// =============================================================================

#[test]
fn arbitrary_controlled_x_matches_cnot() {
    let x = [
        [c64(0.0, 0.0), c64(1.0, 0.0)],
        [c64(1.0, 0.0), c64(0.0, 0.0)],
    ];

    let mut direct =
        StateVector::<Complex64>::basis(QubitCount::new(2), 1).expect("state");

    let mut controlled =
        StateVector::<Complex64>::basis(QubitCount::new(2), 1).expect("state");

    direct.cnot(0, 1).expect("CNOT");

    controlled
        .apply_controlled_single_qubit_matrix(0, 1, x)
        .expect("controlled X");

    assert_eq!(direct, controlled);
}

#[test]
fn arbitrary_controlled_matrix_does_not_modify_control_zero_sector() {
    let x = [
        [c64(0.0, 0.0), c64(1.0, 0.0)],
        [c64(1.0, 0.0), c64(0.0, 0.0)],
    ];

    let mut state =
        StateVector::<Complex64>::basis(QubitCount::new(2), 0).expect("state");

    state
        .apply_controlled_single_qubit_matrix(0, 1, x)
        .expect("controlled X");

    assert_close_f64(state.probability(0).expect("P00"), 1.0, 1.0e-12);
}

#[test]
fn arbitrary_controlled_matrix_rejects_same_qubit() {
    let identity = [
        [c64(1.0, 0.0), c64(0.0, 0.0)],
        [c64(0.0, 0.0), c64(1.0, 0.0)],
    ];

    let mut state =
        StateVector::<Complex64>::zero(QubitCount::new(2)).expect("state");

    assert!(
        state
            .apply_controlled_single_qubit_matrix(0, 0, identity)
            .is_err()
    );
}

// =============================================================================
// Arbitrary two-qubit matrices
// =============================================================================

#[test]
fn arbitrary_two_qubit_identity_preserves_state() {
    let identity = [
        [c64(1.0, 0.0), c64(0.0, 0.0), c64(0.0, 0.0), c64(0.0, 0.0)],
        [c64(0.0, 0.0), c64(1.0, 0.0), c64(0.0, 0.0), c64(0.0, 0.0)],
        [c64(0.0, 0.0), c64(0.0, 0.0), c64(1.0, 0.0), c64(0.0, 0.0)],
        [c64(0.0, 0.0), c64(0.0, 0.0), c64(0.0, 0.0), c64(1.0, 0.0)],
    ];

    let state =
        StateVector::<Complex64>::basis(QubitCount::new(3), 5).expect("state");

    let mut transformed = state.clone();

    transformed
        .apply_two_qubit_matrix(0, 2, identity)
        .expect("identity");

    assert_eq!(state, transformed);
}

#[test]
fn arbitrary_two_qubit_matrix_can_implement_swap() {
    let swap = [
        [c64(1.0, 0.0), c64(0.0, 0.0), c64(0.0, 0.0), c64(0.0, 0.0)],
        [c64(0.0, 0.0), c64(0.0, 0.0), c64(1.0, 0.0), c64(0.0, 0.0)],
        [c64(0.0, 0.0), c64(1.0, 0.0), c64(0.0, 0.0), c64(0.0, 0.0)],
        [c64(0.0, 0.0), c64(0.0, 0.0), c64(0.0, 0.0), c64(1.0, 0.0)],
    ];

    let mut state =
        StateVector::<Complex64>::basis(QubitCount::new(2), 1).expect("state");

    state
        .apply_two_qubit_matrix(0, 1, swap)
        .expect("SWAP matrix");

    assert_close_f64(state.probability(2).expect("P10"), 1.0, 1.0e-12);
}

#[test]
fn arbitrary_two_qubit_matrix_rejects_duplicate_qubits() {
    let identity = [
        [c64(1.0, 0.0), c64(0.0, 0.0), c64(0.0, 0.0), c64(0.0, 0.0)],
        [c64(0.0, 0.0), c64(1.0, 0.0), c64(0.0, 0.0), c64(0.0, 0.0)],
        [c64(0.0, 0.0), c64(0.0, 0.0), c64(1.0, 0.0), c64(0.0, 0.0)],
        [c64(0.0, 0.0), c64(0.0, 0.0), c64(0.0, 0.0), c64(1.0, 0.0)],
    ];

    let mut state =
        StateVector::<Complex64>::zero(QubitCount::new(2)).expect("state");

    assert!(
        state
            .apply_two_qubit_matrix(0, 0, identity)
            .is_err()
    );
}

// =============================================================================
// Multi-qubit matrices
// =============================================================================

#[test]
fn multi_qubit_identity_preserves_state() {
    let identity = vec![
        c64(1.0, 0.0),
        c64(0.0, 0.0),
        c64(0.0, 0.0),
        c64(0.0, 0.0),
        c64(0.0, 0.0),
        c64(1.0, 0.0),
        c64(0.0, 0.0),
        c64(0.0, 0.0),
        c64(0.0, 0.0),
        c64(0.0, 0.0),
        c64(1.0, 0.0),
        c64(0.0, 0.0),
        c64(0.0, 0.0),
        c64(0.0, 0.0),
        c64(0.0, 0.0),
        c64(1.0, 0.0),
    ];

    let state =
        StateVector::<Complex64>::basis(QubitCount::new(2), 3).expect("state");

    let mut transformed = state.clone();

    transformed
        .apply_multi_qubit_matrix(&[0, 1], &identity)
        .expect("identity");

    assert_eq!(state, transformed);
}

#[test]
fn multi_qubit_matrix_can_apply_x_to_selected_qubit() {
    // Matrix is X in a one-qubit local basis.
    let x = vec![
        c64(0.0, 0.0),
        c64(1.0, 0.0),
        c64(1.0, 0.0),
        c64(0.0, 0.0),
    ];

    let mut state =
        StateVector::<Complex64>::basis(QubitCount::new(2), 0).expect("state");

    state
        .apply_multi_qubit_matrix(&[1], &x)
        .expect("single selected qubit");

    assert_close_f64(state.probability(2).expect("P10"), 1.0, 1.0e-12);
}

#[test]
fn multi_qubit_matrix_respects_selected_qubit_order() {
    // This matrix maps local |01> -> |10>.
    //
    // With qubits [1, 0], local position 0 means q1 and local position 1
    // means q0. Starting from global |q1 q0> = |01>, this should therefore
    // produce |10>.
    let swap_local = vec![
        c64(1.0, 0.0),
        c64(0.0, 0.0),
        c64(0.0, 0.0),
        c64(0.0, 0.0),
        c64(0.0, 0.0),
        c64(0.0, 0.0),
        c64(0.0, 0.0),
        c64(1.0, 0.0),
        c64(0.0, 0.0),
        c64(0.0, 0.0),
        c64(1.0, 0.0),
        c64(0.0, 0.0),
        c64(0.0, 0.0),
        c64(0.0, 0.0),
        c64(0.0, 0.0),
        c64(1.0, 0.0),
    ];

    let mut state =
        StateVector::<Complex64>::basis(QubitCount::new(2), 1).expect("state");

    // A complete permutation matrix for swapping local bits:
    // |00> -> |00>
    // |01> -> |10>
    // |10> -> |01>
    // |11> -> |11>
    let swap_local = vec![
        c64(1.0, 0.0),
        c64(0.0, 0.0),
        c64(0.0, 0.0),
        c64(0.0, 0.0),
        c64(0.0, 0.0),
        c64(0.0, 0.0),
        c64(1.0, 0.0),
        c64(0.0, 0.0),
        c64(0.0, 0.0),
        c64(1.0, 0.0),
        c64(0.0, 0.0),
        c64(0.0, 0.0),
        c64(0.0, 0.0),
        c64(0.0, 0.0),
        c64(0.0, 0.0),
        c64(1.0, 0.0),
    ];

    state
        .apply_multi_qubit_matrix(&[1, 0], &swap_local)
        .expect("ordered multi-qubit operation");

    assert_close_f64(state.probability(2).expect("P10"), 1.0, 1.0e-12);
}

#[test]
fn multi_qubit_matrix_rejects_empty_qubit_list() {
    let mut state =
        StateVector::<Complex64>::zero(QubitCount::new(2)).expect("state");

    let identity = vec![
        c64(1.0, 0.0),
    ];

    assert!(
        state
            .apply_multi_qubit_matrix(&[], &identity)
            .is_err()
    );
}

#[test]
fn multi_qubit_matrix_rejects_duplicate_qubits() {
    let mut state =
        StateVector::<Complex64>::zero(QubitCount::new(2)).expect("state");

    let identity = vec![
        c64(1.0, 0.0),
        c64(0.0, 0.0),
        c64(0.0, 0.0),
        c64(1.0, 0.0),
    ];

    assert!(
        state
            .apply_multi_qubit_matrix(&[0, 0], &identity)
            .is_err()
    );
}

#[test]
fn multi_qubit_matrix_rejects_wrong_matrix_dimension() {
    let mut state =
        StateVector::<Complex64>::zero(QubitCount::new(2)).expect("state");

    let wrong_matrix = vec![c64(1.0, 0.0)];

    assert!(
        state
            .apply_multi_qubit_matrix(&[0, 1], &wrong_matrix)
            .is_err()
    );
}

// =============================================================================
// SWAP and permutation
// =============================================================================

#[test]
fn swap_gate_exchanges_qubit_positions() {
    let mut state =
        StateVector::<Complex64>::basis(QubitCount::new(2), 1).expect("state");

    state.swap(0, 1).expect("SWAP");

    assert_close_f64(state.probability(2).expect("P10"), 1.0, 1.0e-12);
}

#[test]
fn swap_same_qubit_is_identity() {
    let state =
        StateVector::<Complex64>::basis(QubitCount::new(3), 5).expect("state");

    let mut transformed = state.clone();

    transformed.swap(1, 1).expect("self-swap");

    assert_eq!(state, transformed);
}

#[test]
fn swap_is_involutory() {
    let state =
        StateVector::<Complex64>::basis(QubitCount::new(3), 5).expect("state");

    let mut transformed = state.clone();

    transformed.swap(0, 2).expect("SWAP1");
    transformed.swap(0, 2).expect("SWAP2");

    assert_eq!(state, transformed);
}

#[test]
fn permutation_reorders_basis_states() {
    let state =
        StateVector::<Complex64>::basis(QubitCount::new(3), 1).expect("state");

    let permuted = state.permuted(&[2, 1, 0]).expect("permutation");

    assert_close_f64(permuted.probability(4).expect("P100"), 1.0, 1.0e-12);
}

#[test]
fn permutation_inverse_restores_state() {
    let state =
        StateVector::<Complex64>::basis(QubitCount::new(4), 0b1011).expect("state");

    let permutation = [2, 0, 3, 1];

    let once = state.permuted(&permutation).expect("permutation");

    // Inverse of [2, 0, 3, 1] is [1, 3, 0, 2].
    let inverse = [1, 3, 0, 2];

    let restored = once.permuted(&inverse).expect("inverse permutation");

    assert_eq!(state, restored);
}

#[test]
fn permutation_rejects_wrong_length() {
    let state =
        StateVector::<Complex64>::zero(QubitCount::new(3)).expect("state");

    assert!(state.permuted(&[0, 1]).is_err());
}

#[test]
fn permutation_rejects_duplicate_positions() {
    let state =
        StateVector::<Complex64>::zero(QubitCount::new(3)).expect("state");

    assert!(state.permuted(&[0, 0, 2]).is_err());
}

#[test]
fn permutation_rejects_out_of_range_positions() {
    let state =
        StateVector::<Complex64>::zero(QubitCount::new(3)).expect("state");

    assert!(state.permuted(&[0, 1, 3]).is_err());
}

// =============================================================================
// Measurement
// =============================================================================

#[test]
fn deterministic_qubit_measurement_selects_one_when_sample_is_below_p1() {
    let mut state =
        StateVector::<Complex64>::zero(QubitCount::new(1)).expect("state");

    state.h(0).expect("H");

    let measurement = state.measure_qubit(0, 0.1).expect("measurement");

    assert_eq!(measurement.outcome, 1);
    assert_close_f64(measurement.probability, 0.5, 1.0e-12);

    assert_close_f64(state.probability(1).expect("P1"), 1.0, 1.0e-12);
    assert_close_f64(state.probability(0).expect("P0"), 0.0, 1.0e-12);
}

#[test]
fn deterministic_qubit_measurement_selects_zero_when_sample_is_at_or_above_p1() {
    let mut state =
        StateVector::<Complex64>::zero(QubitCount::new(1)).expect("state");

    state.h(0).expect("H");

    let measurement = state.measure_qubit(0, 0.9).expect("measurement");

    assert_eq!(measurement.outcome, 0);
    assert_close_f64(measurement.probability, 0.5, 1.0e-12);

    assert_close_f64(state.probability(0).expect("P0"), 1.0, 1.0e-12);
    assert_close_f64(state.probability(1).expect("P1"), 0.0, 1.0e-12);
}

#[test]
fn measurement_sample_zero_is_valid() {
    let mut state =
        StateVector::<Complex64>::basis(QubitCount::new(1), 0).expect("state");

    let measurement = state.measure_qubit(0, 0.0).expect("sample zero");

    assert_eq!(measurement.outcome, 0);
}

#[test]
fn measurement_sample_just_below_one_is_valid() {
    let mut state =
        StateVector::<Complex64>::basis(QubitCount::new(1), 1).expect("state");

    let measurement = state
        .measure_qubit(0, 1.0 - f64::EPSILON)
        .expect("sample");

    assert_eq!(measurement.outcome, 1);
}

#[test]
fn measurement_sample_one_is_rejected() {
    let mut state =
        StateVector::<Complex64>::zero(QubitCount::new(1)).expect("state");

    assert!(
        state
            .measure_qubit(0, 1.0)
            .is_err()
    );
}

#[test]
fn measurement_negative_sample_is_rejected() {
    let mut state =
        StateVector::<Complex64>::zero(QubitCount::new(1)).expect("state");

    assert!(
        state
            .measure_qubit(0, -0.01)
            .is_err()
    );
}

#[test]
fn measurement_nan_sample_is_rejected() {
    let mut state =
        StateVector::<Complex64>::zero(QubitCount::new(1)).expect("state");

    assert!(
        state
            .measure_qubit(0, f64::NAN)
            .is_err()
    );
}

#[test]
fn measurement_infinity_sample_is_rejected() {
    let mut state =
        StateVector::<Complex64>::zero(QubitCount::new(1)).expect("state");

    assert!(
        state
            .measure_qubit(0, f64::INFINITY)
            .is_err()
    );
}

#[test]
fn measurement_invalid_qubit_is_rejected() {
    let mut state =
        StateVector::<Complex64>::zero(QubitCount::new(1)).expect("state");

    assert!(
        state
            .measure_qubit(1, 0.5)
            .is_err()
    );
}

#[test]
fn basis_measurement_collapses_to_selected_basis_state() {
    let mut state =
        StateVector::<Complex64>::zero(QubitCount::new(2)).expect("state");

    state.h(0).expect("H0");
    state.h(1).expect("H1");

    let measurement = state.measure_basis(0.1).expect("basis measurement");

    assert_measurement_shape(&measurement, 2);

    assert_close_f64(
        state.probability(measurement.basis_index).expect("selected P"),
        1.0,
        1.0e-10,
    );

    for index in 0..4 {
        if index != measurement.basis_index {
            assert_close_f64(state.probability(index).expect("P"), 0.0, 1.0e-10);
        }
    }

    assert_normalized_f64(&state);
}

#[test]
fn basis_measurement_sample_zero_selects_first_nonzero_basis_state() {
    let mut state =
        StateVector::<Complex64>::basis(QubitCount::new(2), 0).expect("state");

    let measurement = state.measure_basis(0.0).expect("measurement");

    assert_eq!(measurement.basis_index, 0);
    assert_eq!(measurement.bits, vec![false, false]);
}

#[test]
fn basis_measurement_sample_one_is_rejected() {
    let mut state =
        StateVector::<Complex64>::zero(QubitCount::new(2)).expect("state");

    assert!(state.measure_basis(1.0).is_err());
}

#[test]
fn measurement_preserves_normalization() {
    let mut state =
        StateVector::<Complex64>::zero(QubitCount::new(3)).expect("state");

    state.h(0).expect("H0");
    state.h(1).expect("H1");
    state.h(2).expect("H2");

    state.measure_qubit(1, 0.25).expect("measurement");

    assert_normalized_f64(&state);
}

// =============================================================================
// Reset
// =============================================================================

#[test]
fn reset_qubit_maps_one_to_zero() {
    let mut state =
        StateVector::<Complex64>::basis(QubitCount::new(1), 1).expect("state");

    state.reset_qubit(0).expect("reset");

    assert_close_f64(state.probability(0).expect("P0"), 1.0, 1.0e-12);
    assert_close_f64(state.probability(1).expect("P1"), 0.0, 1.0e-12);
}

#[test]
fn reset_qubit_maps_superposition_to_zero() {
    let mut state =
        StateVector::<Complex64>::zero(QubitCount::new(1)).expect("state");

    state.h(0).expect("H");
    state.reset_qubit(0).expect("reset");

    assert_close_f64(state.probability(0).expect("P0"), 1.0, 1.0e-12);
    assert_close_f64(state.probability(1).expect("P1"), 0.0, 1.0e-12);

    assert_normalized_f64(&state);
}

#[test]
fn reset_all_restores_zero_state() {
    let mut state =
        StateVector::<Complex64>::zero(QubitCount::new(3)).expect("state");

    state.h(0).expect("H0");
    state.x(1).expect("X1");
    state.y(2).expect("Y2");

    state.reset_all();

    let expected =
        StateVector::<Complex64>::zero(QubitCount::new(3)).expect("expected");

    assert_eq!(state, expected);
}

#[test]
fn reset_invalid_qubit_is_rejected() {
    let mut state =
        StateVector::<Complex64>::zero(QubitCount::new(2)).expect("state");

    assert!(state.reset_qubit(2).is_err());
}

// =============================================================================
// Inner product and fidelity
// =============================================================================

#[test]
fn inner_product_of_identical_normalized_states_is_one() {
    let state =
        StateVector::<Complex64>::basis(QubitCount::new(3), 5).expect("state");

    let overlap = state.inner_product(&state).expect("inner product");

    assert_complex64_close(overlap, c64(1.0, 0.0), 1.0e-12);
}

#[test]
fn inner_product_of_orthogonal_basis_states_is_zero() {
    let first =
        StateVector::<Complex64>::basis(QubitCount::new(3), 1).expect("first");

    let second =
        StateVector::<Complex64>::basis(QubitCount::new(3), 6).expect("second");

    let overlap = first.inner_product(&second).expect("inner product");

    assert_complex64_close(overlap, c64(0.0, 0.0), 1.0e-12);
}

#[test]
fn inner_product_uses_conjugate_of_left_state() {
    let first =
        StateVector::<Complex64>::from_amplitudes(vec![c64(0.0, 1.0), c64(0.0, 0.0)])
            .expect("normalized state");

    let second =
        StateVector::<Complex64>::basis(QubitCount::new(1), 0).expect("state");

    let overlap = first.inner_product(&second).expect("inner product");

    assert_complex64_close(overlap, c64(0.0, -1.0), 1.0e-12);
}

#[test]
fn fidelity_of_identical_states_is_one() {
    let state =
        StateVector::<Complex64>::basis(QubitCount::new(4), 9).expect("state");

    assert_close_f64(state.fidelity(&state).expect("fidelity"), 1.0, 1.0e-12);
}

#[test]
fn fidelity_of_orthogonal_states_is_zero() {
    let first =
        StateVector::<Complex64>::basis(QubitCount::new(2), 0).expect("first");

    let second =
        StateVector::<Complex64>::basis(QubitCount::new(2), 3).expect("second");

    assert_close_f64(first.fidelity(&second).expect("fidelity"), 0.0, 1.0e-12);
}

#[test]
fn inner_product_rejects_different_dimensions() {
    let first =
        StateVector::<Complex64>::basis(QubitCount::new(1), 0).expect("first");

    let second =
        StateVector::<Complex64>::basis(QubitCount::new(2), 0).expect("second");

    assert!(first.inner_product(&second).is_err());
}

#[test]
fn fidelity_rejects_different_dimensions() {
    let first =
        StateVector::<Complex64>::basis(QubitCount::new(1), 0).expect("first");

    let second =
        StateVector::<Complex64>::basis(QubitCount::new(2), 0).expect("second");

    assert!(first.fidelity(&second).is_err());
}

// =============================================================================
// Expectation values
// =============================================================================

#[test]
fn expectation_of_z_on_zero_is_plus_one() {
    let state =
        StateVector::<Complex64>::basis(QubitCount::new(1), 0).expect("state");

    let z = [
        [c64(1.0, 0.0), c64(0.0, 0.0)],
        [c64(0.0, 0.0), c64(-1.0, 0.0)],
    ];

    let expectation = state
        .expectation_single_qubit(0, z)
        .expect("expectation");

    assert_complex64_close(expectation, c64(1.0, 0.0), 1.0e-12);
}

#[test]
fn expectation_of_z_on_one_is_minus_one() {
    let state =
        StateVector::<Complex64>::basis(QubitCount::new(1), 1).expect("state");

    let z = [
        [c64(1.0, 0.0), c64(0.0, 0.0)],
        [c64(0.0, 0.0), c64(-1.0, 0.0)],
    ];

    let expectation = state
        .expectation_single_qubit(0, z)
        .expect("expectation");

    assert_complex64_close(expectation, c64(-1.0, 0.0), 1.0e-12);
}

#[test]
fn expectation_of_z_on_plus_is_zero() {
    let mut state =
        StateVector::<Complex64>::zero(QubitCount::new(1)).expect("state");

    state.h(0).expect("H");

    let z = [
        [c64(1.0, 0.0), c64(0.0, 0.0)],
        [c64(0.0, 0.0), c64(-1.0, 0.0)],
    ];

    let expectation = state
        .expectation_single_qubit(0, z)
        .expect("expectation");

    assert_complex64_close(expectation, c64(0.0, 0.0), 1.0e-12);
}

#[test]
fn expectation_rejects_invalid_qubit() {
    let state =
        StateVector::<Complex64>::zero(QubitCount::new(1)).expect("state");

    let identity = [
        [c64(1.0, 0.0), c64(0.0, 0.0)],
        [c64(0.0, 0.0), c64(1.0, 0.0)],
    ];

    assert!(
        state
            .expectation_single_qubit(1, identity)
            .is_err()
    );
}

// =============================================================================
// Tensor products
// =============================================================================

#[test]
fn tensor_product_has_sum_of_qubit_counts() {
    let left =
        StateVector::<Complex64>::basis(QubitCount::new(2), 1).expect("left");

    let right =
        StateVector::<Complex64>::basis(QubitCount::new(3), 5).expect("right");

    let combined = left.tensor_product(&right).expect("tensor");

    assert_eq!(combined.qubit_count().get(), 5);
    assert_eq!(combined.amplitude_count().get(), 32);
}

#[test]
fn tensor_product_uses_documented_little_endian_composition() {
    let left =
        StateVector::<Complex64>::basis(QubitCount::new(1), 1).expect("left");

    let right =
        StateVector::<Complex64>::basis(QubitCount::new(1), 0).expect("right");

    let combined = left.tensor_product(&right).expect("tensor");

    // self occupies the higher-order qubit and other the lower-order qubit.
    assert_close_f64(combined.probability(2).expect("P10"), 1.0, 1.0e-12);
}

#[test]
fn tensor_product_of_normalized_states_is_normalized() {
    let mut left =
        StateVector::<Complex64>::zero(QubitCount::new(2)).expect("left");

    let mut right =
        StateVector::<Complex64>::zero(QubitCount::new(2)).expect("right");

    left.h(0).expect("H");
    right.h(1).expect("H");

    let combined = left.tensor_product(&right).expect("tensor");

    assert_normalized_f64(&combined);
}

#[test]
fn tensor_product_preserves_finite_amplitudes() {
    let left =
        StateVector::<Complex64>::from_amplitudes_normalized(vec![
            c64(1.0, 0.0),
            c64(0.0, 1.0),
        ])
        .expect("left");

    let right =
        StateVector::<Complex64>::from_amplitudes_normalized(vec![
            c64(1.0, 0.0),
            c64(1.0, 0.0),
        ])
        .expect("right");

    let combined = left.tensor_product(&right).expect("tensor");

    for amplitude in combined.amplitudes() {
        assert!(amplitude.is_finite());
    }
}

// =============================================================================
// Cloning and replacement
// =============================================================================

#[test]
fn deep_clone_is_independent() {
    let original =
        StateVector::<Complex64>::basis(QubitCount::new(2), 1).expect("original");

    let mut clone = original.deep_clone();

    clone.x(1).expect("mutate clone");

    assert_ne!(original, clone);
    assert_close_f64(original.probability(1).expect("original P1"), 1.0, 1.0e-12);
}

#[test]
fn replace_from_copies_state_of_same_dimension() {
    let source =
        StateVector::<Complex64>::basis(QubitCount::new(2), 3).expect("source");

    let mut destination =
        StateVector::<Complex64>::basis(QubitCount::new(2), 0).expect("destination");

    destination.replace_from(&source).expect("replace");

    assert_eq!(destination, source);
}

#[test]
fn replace_from_rejects_different_dimensions() {
    let source =
        StateVector::<Complex64>::basis(QubitCount::new(2), 3).expect("source");

    let mut destination =
        StateVector::<Complex64>::basis(QubitCount::new(3), 0).expect("destination");

    assert!(destination.replace_from(&source).is_err());
}

// =============================================================================
// State validation
// =============================================================================

#[test]
fn validate_accepts_valid_zero_state() {
    let state =
        StateVector::<Complex64>::zero(QubitCount::new(4)).expect("state");

    assert!(state.validate().is_ok());
}

#[test]
fn validate_accepts_valid_complex_state() {
    let state =
        StateVector::<Complex64>::from_amplitudes_normalized(vec![
            c64(0.5, 0.0),
            c64(0.0, 0.5),
            c64(0.5, 0.0),
            c64(0.0, -0.5),
        ])
        .expect("state");

    assert!(state.validate().is_ok());
    assert!(state.validate_normalized().is_ok());
}

#[test]
fn explicit_non_normalized_state_can_be_detected() {
    let state =
        StateVector::<Complex64>::from_amplitudes_unchecked_normalization(vec![
            c64(2.0, 0.0),
            c64(0.0, 0.0),
        ])
        .expect("state");

    assert!(state.validate().is_ok());
    assert!(state.validate_normalized().is_err());
}

// =============================================================================
// Complex32 portability
// =============================================================================

#[test]
fn complex32_zero_state_is_supported() {
    let state =
        StateVector::<Complex32>::zero(QubitCount::new(2)).expect("Complex32 state");

    assert_eq!(state.amplitude_count().get(), 4);
    assert_complex32_close(
        state.amplitude(0).expect("a0"),
        c32(1.0, 0.0),
        1.0e-6,
    );

    assert_normalized_f32(&state);
}

#[test]
fn complex32_hadamard_preserves_normalization() {
    let mut state =
        StateVector::<Complex32>::zero(QubitCount::new(1)).expect("state");

    state.h(0).expect("H");

    let expected = 1.0_f32 / 2.0_f32.sqrt();

    assert_complex32_close(
        state.amplitude(0).expect("a0"),
        c32(expected, 0.0),
        2.0e-5,
    );

    assert_complex32_close(
        state.amplitude(1).expect("a1"),
        c32(expected, 0.0),
        2.0e-5,
    );

    assert_normalized_f32(&state);
}

#[test]
fn complex32_probabilities_are_non_negative_and_normalized() {
    let mut state =
        StateVector::<Complex32>::zero(QubitCount::new(3)).expect("state");

    state.h(0).expect("H0");
    state.h(1).expect("H1");
    state.h(2).expect("H2");

    assert_probabilities_sum_to_one_f32(&state);
}

// =============================================================================
// Unitary-operation invariants
// =============================================================================

#[test]
fn standard_unitaries_preserve_norm() {
    let mut state =
        StateVector::<Complex64>::zero(QubitCount::new(3)).expect("state");

    state.h(0).expect("H");
    assert_normalized_f64(&state);

    state.x(1).expect("X");
    assert_normalized_f64(&state);

    state.y(2).expect("Y");
    assert_normalized_f64(&state);

    state.z(0).expect("Z");
    assert_normalized_f64(&state);

    state.rx(1, 0.73).expect("RX");
    assert_normalized_f64(&state);

    state.ry(2, -1.17).expect("RY");
    assert_normalized_f64(&state);

    state.rz(0, 2.31).expect("RZ");
    assert_normalized_f64(&state);

    state.cnot(0, 1).expect("CNOT");
    assert_normalized_f64(&state);

    state.cz(1, 2).expect("CZ");
    assert_normalized_f64(&state);

    state.swap(0, 2).expect("SWAP");
    assert_normalized_f64(&state);
}

#[test]
fn probabilities_remain_valid_after_nontrivial_circuit() {
    let mut state =
        StateVector::<Complex64>::zero(QubitCount::new(4)).expect("state");

    state.h(0).expect("H0");
    state.ry(1, 0.3).expect("RY1");
    state.rx(2, -0.7).expect("RX2");
    state.rz(3, 1.2).expect("RZ3");
    state.cnot(0, 1).expect("CNOT01");
    state.cnot(2, 3).expect("CNOT23");
    state.cz(1, 3).expect("CZ13");
    state.swap(0, 2).expect("SWAP02");

    state.validate().expect("valid state");
    assert_normalized_f64(&state);
    assert_probabilities_sum_to_one_f64(&state);
}

// =============================================================================
// Measurement / basis ordering invariants
// =============================================================================

#[test]
fn basis_measurement_bits_match_little_endian_basis_index() {
    let mut state =
        StateVector::<Complex64>::basis(QubitCount::new(4), 0b1011).expect("state");

    let measurement = state.measure_basis(0.0).expect("measurement");

    assert_eq!(measurement.basis_index, 0b1011);

    // bits are returned in q0, q1, q2, q3 order.
    assert_eq!(
        measurement.bits,
        vec![true, true, false, true]
    );
}

#[test]
fn deterministic_basis_measurement_on_basis_state_is_exact() {
    let mut state =
        StateVector::<Complex64>::basis(QubitCount::new(5), 0b10101).expect("state");

    let measurement = state.measure_basis(0.999999).expect("measurement");

    assert_eq!(measurement.basis_index, 0b10101);
    assert_eq!(
        measurement.bits,
        vec![true, false, true, false, true]
    );
    assert_close_f64(measurement.probability, 1.0, 1.0e-12);
}

// =============================================================================
// Provider-neutrality invariants
// =============================================================================

#[test]
fn state_vector_contains_no_provider_specific_metadata() {
    let state =
        StateVector::<Complex64>::zero(QubitCount::new(3)).expect("state");

    let metadata = state.metadata();

    // The metadata contract contains only representation information.
    assert_eq!(metadata.qubits.get(), 3);
    assert_eq!(metadata.amplitudes.get(), 8);
    assert_eq!(metadata.bytes_per_amplitude, 16);
}

#[test]
fn state_vector_can_be_used_as_a_reference_model_for_backend_validation() {
    // This test intentionally models the role expected from future hardware
    // adapters: construct a canonical state, execute a mathematically
    // equivalent operation, and compare the observable state.
    //
    // No vendor SDK or hardware API belongs in this memory test.
    let mut reference =
        StateVector::<Complex64>::zero(QubitCount::new(2)).expect("reference");

    reference.h(0).expect("H");
    reference.cnot(0, 1).expect("CNOT");

    let expected = StateVector::<Complex64>::from_amplitudes_normalized(vec![
        c64(1.0 / 2.0_f64.sqrt(), 0.0),
        c64(0.0, 0.0),
        c64(0.0, 0.0),
        c64(1.0 / 2.0_f64.sqrt(), 0.0),
    ])
    .expect("expected");

    assert_close_f64(
        reference.fidelity(&expected).expect("fidelity"),
        1.0,
        1.0e-10,
    );
}

// =============================================================================
// Error-contract smoke tests
// =============================================================================

#[test]
fn invalid_inputs_return_errors_instead_of_panicking() {
    let mut state =
        StateVector::<Complex64>::zero(QubitCount::new(2)).expect("state");

    assert!(state.amplitude(4).is_err());
    assert!(state.probability(4).is_err());
    assert!(state.probability_zero(2).is_err());
    assert!(state.probability_one(2).is_err());
    assert!(state.reset_qubit(2).is_err());
    assert!(state.measure_qubit(2, 0.5).is_err());
    assert!(state.measure_basis(1.0).is_err());
}

#[test]
fn valid_operations_do_not_return_errors() {
    let mut state =
        StateVector::<Complex64>::zero(QubitCount::new(3)).expect("state");

    assert_error(Ok(()).map(|_| ()));

    state.h(0).expect("H");
    state.x(1).expect("X");
    state.y(2).expect("Y");
    state.cnot(0, 1).expect("CNOT");
    state.cz(1, 2).expect("CZ");
    state.swap(0, 2).expect("SWAP");
    state.reset_qubit(1).expect("reset");

    assert_normalized_f64(&state);
}

// =============================================================================
// Long operation sequence / stability
// =============================================================================

#[test]
fn repeated_unitary_operations_remain_numerically_stable() {
    let mut state =
        StateVector::<Complex64>::zero(QubitCount::new(2)).expect("state");

    for iteration in 0..256usize {
        let angle = (iteration as f64) * 0.017;

        state.rx(0, angle).expect("RX");
        state.ry(1, -angle).expect("RY");
        state.cnot(0, 1).expect("CNOT");
        state.rz(0, angle * 0.5).expect("RZ");

        assert_normalized_f64(&state);
        assert_probabilities_sum_to_one_f64(&state);
    }
}

#[test]
fn cloning_and_replacing_preserve_exact_basis_states() {
    for basis in 0..8 {
        let original =
            StateVector::<Complex64>::basis(QubitCount::new(3), basis).expect("state");

        let clone = original.deep_clone();

        assert_eq!(clone, original);

        let mut destination =
            StateVector::<Complex64>::zero(QubitCount::new(3)).expect("destination");

        destination
            .replace_from(&clone)
            .expect("replacement");

        assert_eq!(destination, original);
    }
}