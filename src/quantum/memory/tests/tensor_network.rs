//! Zamani Quantum Memory — Tensor-Network / MPS Integration Tests
//!
//! Production-grade tests for `memory::tensor_network`.
//!
//! # Purpose
//!
//! This module verifies the externally observable mathematical and resource
//! contract of Zamani's Matrix Product State (MPS) representation.
//!
//! The tests deliberately avoid:
//!
//! - `unsafe`;
//! - raw pointers;
//! - vendor-specific APIs;
//! - CUDA/HIP/Metal/Vulkan APIs;
//! - QPU-provider APIs;
//! - external test dependencies;
//! - implementation-specific memory addresses;
//! - nondeterministic global RNG state.
//!
//! The same tests must remain valid when the implementation is later backed
//! by:
//!
//! - CPU kernels;
//! - SIMD kernels;
//! - GPU/device memory;
//! - distributed tensor networks;
//! - memory pools;
//! - accelerator backends;
//! - remote simulators;
//! - hardware validation adapters.
//!
//! # Architectural contract
//!
//! ```text
//!                    Zamani Quantum IR
//!                           |
//!                           v
//!                       executor
//!                           |
//!                           v
//!                    quantum::memory
//!                           |
//!                  TensorNetworkState
//!                           |
//!              +------------+------------+
//!              |            |            |
//!              v            v            v
//!             CPU          GPU      distributed
//!              |            |            |
//!              +------------+------------+
//!                           |
//!                           v
//!                    simulator / verifier
//!                           |
//!                           v
//!                    hardware / QPU
//! ```
//!
//! An actual QPU generally does not expose its complete wavefunction.
//! Therefore these tests validate the MPS representation wherever an MPS is
//! legitimately used, including:
//!
//! - classical simulation;
//! - approximate simulation;
//! - reference simulation;
//! - state-preparation verification;
//! - transpiler verification;
//! - differential testing;
//! - hybrid quantum/classical workflows;
//! - tensor-network simulation of large low-entanglement circuits.
//!
//! They do not require a QPU to expose its internal quantum state.
//!
//! # Representation contract
//!
//! The production implementation defines:
//!
//! ```text
//! |psi> = sum A[0]^s0 A[1]^s1 ... A[n-1]^sn-1 |s0...sn-1>
//! ```
//!
//! with:
//!
//! - physical dimension = 2;
//! - site index = logical qubit index;
//! - little-endian logical basis ordering;
//! - left boundary bond = 1;
//! - right boundary bond = 1;
//! - interior bond dimensions >= 1;
//! - normalized state invariant;
//! - explicit truncation policy.
//!
//! The tests below treat those semantics as stable public contracts.
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
//! # Test categories
//!
//! 1. schema stability;
//! 2. truncation-policy validation;
//! 3. tensor construction;
//! 4. tensor indexing;
//! 5. tensor storage invariants;
//! 6. zero-state construction;
//! 7. computational-basis construction;
//! 8. little-endian ordering;
//! 9. state-vector conversion;
//! 10. product-state conversion;
//! 11. Bell-state conversion;
//! 12. GHZ-state conversion;
//! 13. complex-amplitude preservation;
//! 14. normalization;
//! 15. probability invariants;
//! 16. bond-dimension invariants;
//! 17. truncation-policy enforcement;
//! 18. deterministic construction;
//! 19. invalid-input handling;
//! 20. NaN/infinity rejection;
//! 21. memory-growth protection;
//! 22. precision portability;
//! 23. repeated conversion stability;
//! 24. fidelity-preserving round trips;
//! 25. provider-neutral behavior.
//!
//! # Important production invariant
//!
//! A tensor-network test must never accidentally turn into a dense simulation
//! benchmark of an arbitrarily large circuit. Dense conversion has exponential
//! cost. Tests therefore use deliberately small systems while explicitly
//! testing the dimension and allocation boundaries.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use crate::quantum::memory::complex::{Complex32, Complex64, ComplexScalar};
use crate::quantum::memory::errors::MemoryError;
use crate::quantum::memory::state_vector::StateVector;
use crate::quantum::memory::tensor_network::{
    MpsTensor,
    TensorNetworkState,
    TruncationPolicy,
    DEFAULT_ABSOLUTE_CUTOFF,
    DEFAULT_MAX_BOND_DIMENSION,
    DEFAULT_MAX_DISCARDED_WEIGHT,
    DEFAULT_RELATIVE_CUTOFF,
    DEFAULT_VALIDATION_TOLERANCE,
    QUBIT_PHYSICAL_DIMENSION,
    TENSOR_NETWORK_SCHEMA_ID,
    TENSOR_NETWORK_SCHEMA_VERSION,
};
use crate::quantum::memory::types::QubitCount;

// =============================================================================
// Test constants
// =============================================================================

const F64_TOLERANCE: f64 = 1.0e-10;
const F32_TOLERANCE: f32 = 2.0e-5;

const MAX_SMALL_TEST_QUBITS: usize = 8;

// =============================================================================
// Numerical helpers
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

fn assert_complex64_close(
    actual: Complex64,
    expected: Complex64,
    tolerance: f64,
) {
    assert_close_f64(actual.real(), expected.real(), tolerance);
    assert_close_f64(actual.imaginary(), expected.imaginary(), tolerance);
}

fn assert_complex32_close(
    actual: Complex32,
    expected: Complex32,
    tolerance: f32,
) {
    assert_close_f32(actual.real(), expected.real(), tolerance);
    assert_close_f32(actual.imaginary(), expected.imaginary(), tolerance);
}

fn assert_state_vector_close_f64(
    actual: &StateVector<Complex64>,
    expected: &StateVector<Complex64>,
    tolerance: f64,
) {
    assert_eq!(
        actual.qubit_count(),
        expected.qubit_count(),
        "state-vector qubit counts differ"
    );

    assert_eq!(
        actual.amplitudes().len(),
        expected.amplitudes().len(),
        "state-vector dimensions differ"
    );

    for index in 0..actual.amplitudes().len() {
        let lhs = actual
            .amplitude(index)
            .expect("actual amplitude must exist");

        let rhs = expected
            .amplitude(index)
            .expect("expected amplitude must exist");

        assert_complex64_close(lhs, rhs, tolerance);
    }
}

fn assert_state_vector_close_f32(
    actual: &StateVector<Complex32>,
    expected: &StateVector<Complex32>,
    tolerance: f32,
) {
    assert_eq!(
        actual.qubit_count(),
        expected.qubit_count(),
        "state-vector qubit counts differ"
    );

    assert_eq!(
        actual.amplitudes().len(),
        expected.amplitudes().len(),
        "state-vector dimensions differ"
    );

    for index in 0..actual.amplitudes().len() {
        let lhs = actual
            .amplitude(index)
            .expect("actual amplitude must exist");

        let rhs = expected
            .amplitude(index)
            .expect("expected amplitude must exist");

        assert_complex32_close(lhs, rhs, tolerance);
    }
}

fn assert_probabilities_sum_to_one_f64(
    state: &TensorNetworkState<Complex64>,
) {
    let mut total = 0.0;

    for index in 0..basis_dimension(state.qubit_count().get()) {
        let probability = state
            .probability(index)
            .expect("probability must be defined");

        assert!(
            probability >= -F64_TOLERANCE,
            "probability must not be negative: {probability:?}"
        );

        assert!(
            probability <= 1.0 + F64_TOLERANCE,
            "probability must not exceed one: {probability:?}"
        );

        total += probability;
    }

    assert_close_f64(total, 1.0, F64_TOLERANCE);
}

fn basis_dimension(qubits: usize) -> usize {
    1usize
        .checked_shl(qubits as u32)
        .expect("test qubit count must fit usize")
}

fn bell_state_f64() -> StateVector<Complex64> {
    let inverse_sqrt_two = 1.0 / 2.0_f64.sqrt();

    StateVector::from_amplitudes_normalized(vec![
        c64(inverse_sqrt_two, 0.0),
        c64(0.0, 0.0),
        c64(0.0, 0.0),
        c64(inverse_sqrt_two, 0.0),
    ])
    .expect("Bell state must be valid")
}

fn ghz_state_f64(qubits: usize) -> StateVector<Complex64> {
    assert!(qubits >= 2);

    let dimension = basis_dimension(qubits);
    let inverse_sqrt_two = 1.0 / 2.0_f64.sqrt();

    let mut amplitudes = vec![c64(0.0, 0.0); dimension];

    amplitudes[0] = c64(inverse_sqrt_two, 0.0);
    amplitudes[dimension - 1] = c64(inverse_sqrt_two, 0.0);

    StateVector::from_amplitudes_normalized(amplitudes)
        .expect("GHZ state must be valid")
}

fn complex_phase_state_f64() -> StateVector<Complex64> {
    let inverse_sqrt_two = 1.0 / 2.0_f64.sqrt();

    StateVector::from_amplitudes_normalized(vec![
        c64(inverse_sqrt_two, 0.0),
        c64(0.0, inverse_sqrt_two),
    ])
    .expect("complex phase state must be valid")
}

fn assert_normalized_f64(state: &TensorNetworkState<Complex64>) {
    state
        .validate()
        .expect("tensor-network state must validate");

    let dense = state
        .to_state_vector()
        .expect("small tensor-network state must convert");

    dense
        .validate_normalized()
        .expect("converted state must be normalized");

    assert_close_f64(
        dense.norm_squared(),
        1.0,
        F64_TOLERANCE,
    );
}

fn assert_bond_dimensions_are_valid(
    state: &TensorNetworkState<Complex64>,
) {
    let bonds = state.bond_dimensions();

    assert_eq!(
        bonds.len(),
        state.qubit_count().get() + 1,
        "MPS must expose one boundary entry plus one bond per site"
    );

    assert_eq!(
        bonds.first().copied(),
        Some(1),
        "left boundary bond must be one"
    );

    assert_eq!(
        bonds.last().copied(),
        Some(1),
        "right boundary bond must be one"
    );

    for bond in bonds {
        assert!(bond > 0, "MPS bond dimension must never be zero");
        assert!(
            bond <= DEFAULT_MAX_BOND_DIMENSION,
            "default construction must respect the default bond limit"
        );
    }
}

// =============================================================================
// Schema and public-contract tests
// =============================================================================

#[test]
fn schema_identity_is_stable() {
    assert_eq!(
        TENSOR_NETWORK_SCHEMA_ID,
        "zamani.quantum.memory.tensor_network.mps"
    );

    assert_eq!(
        TENSOR_NETWORK_SCHEMA_VERSION,
        1
    );
}

#[test]
fn physical_dimension_is_fixed_to_qubits() {
    assert_eq!(
        QUBIT_PHYSICAL_DIMENSION,
        2
    );
}

#[test]
fn default_truncation_policy_is_valid() {
    let policy = TruncationPolicy::default();

    policy
        .validate()
        .expect("default truncation policy must be valid");

    assert_eq!(
        policy.max_bond_dimension,
        DEFAULT_MAX_BOND_DIMENSION
    );

    assert_close_f64(
        policy.absolute_cutoff,
        DEFAULT_ABSOLUTE_CUTOFF,
        0.0,
    );

    assert_close_f64(
        policy.relative_cutoff,
        DEFAULT_RELATIVE_CUTOFF,
        0.0,
    );

    assert_close_f64(
        policy.maximum_discarded_weight,
        DEFAULT_MAX_DISCARDED_WEIGHT,
        0.0,
    );

    assert!(policy.allow_truncation);
}

#[test]
fn default_validation_tolerance_is_positive_and_finite() {
    assert!(DEFAULT_VALIDATION_TOLERANCE.is_finite());
    assert!(DEFAULT_VALIDATION_TOLERANCE > 0.0);
}

// =============================================================================
// Truncation-policy validation
// =============================================================================

#[test]
fn zero_maximum_bond_dimension_is_rejected() {
    let policy = TruncationPolicy {
        max_bond_dimension: 0,
        ..TruncationPolicy::default()
    };

    assert!(policy.validate().is_err());
}

#[test]
fn negative_absolute_cutoff_is_rejected() {
    let policy = TruncationPolicy {
        absolute_cutoff: -1.0,
        ..TruncationPolicy::default()
    };

    assert!(policy.validate().is_err());
}

#[test]
fn negative_relative_cutoff_is_rejected() {
    let policy = TruncationPolicy {
        relative_cutoff: -1.0,
        ..TruncationPolicy::default()
    };

    assert!(policy.validate().is_err());
}

#[test]
fn negative_discarded_weight_is_rejected() {
    let policy = TruncationPolicy {
        maximum_discarded_weight: -1.0,
        ..TruncationPolicy::default()
    };

    assert!(policy.validate().is_err());
}

#[test]
fn discarded_weight_above_one_is_rejected() {
    let policy = TruncationPolicy {
        maximum_discarded_weight: 1.0 + f64::EPSILON,
        ..TruncationPolicy::default()
    };

    assert!(policy.validate().is_err());
}

#[test]
fn nan_absolute_cutoff_is_rejected() {
    let policy = TruncationPolicy {
        absolute_cutoff: f64::NAN,
        ..TruncationPolicy::default()
    };

    assert!(policy.validate().is_err());
}

#[test]
fn infinity_relative_cutoff_is_rejected() {
    let policy = TruncationPolicy {
        relative_cutoff: f64::INFINITY,
        ..TruncationPolicy::default()
    };

    assert!(policy.validate().is_err());
}

#[test]
fn negative_infinity_discarded_weight_is_rejected() {
    let policy = TruncationPolicy {
        maximum_discarded_weight: f64::NEG_INFINITY,
        ..TruncationPolicy::default()
    };

    assert!(policy.validate().is_err());
}

// =============================================================================
// MpsTensor construction and indexing
// =============================================================================

#[test]
fn tensor_zero_constructor_creates_correct_shape() {
    let tensor =
        MpsTensor::<Complex64>::zeros(2, 3)
            .expect("valid tensor dimensions must succeed");

    assert_eq!(tensor.left_dimension(), 2);
    assert_eq!(tensor.right_dimension(), 3);
    assert_eq!(
        tensor.element_count(),
        2 * QUBIT_PHYSICAL_DIMENSION * 3
    );
}

#[test]
fn tensor_zero_constructor_initializes_all_values_to_zero() {
    let tensor =
        MpsTensor::<Complex64>::zeros(2, 2)
            .expect("valid tensor dimensions must succeed");

    for value in tensor.values() {
        assert_complex64_close(
            *value,
            c64(0.0, 0.0),
            F64_TOLERANCE,
        );
    }
}

#[test]
fn tensor_zero_constructor_rejects_zero_left_dimension() {
    let result = MpsTensor::<Complex64>::zeros(0, 1);

    assert!(result.is_err());
}

#[test]
fn tensor_zero_constructor_rejects_zero_right_dimension() {
    let result = MpsTensor::<Complex64>::zeros(1, 0);

    assert!(result.is_err());
}

#[test]
fn tensor_from_values_accepts_exact_shape() {
    let values = vec![
        c64(1.0, 0.0),
        c64(0.0, 0.0),
        c64(0.0, 0.0),
        c64(1.0, 0.0),
    ];

    let tensor =
        MpsTensor::<Complex64>::from_values(1, 2, values)
            .expect("correct number of values must succeed");

    assert_eq!(tensor.element_count(), 4);
}

#[test]
fn tensor_from_values_rejects_wrong_element_count() {
    let result =
        MpsTensor::<Complex64>::from_values(
            1,
            2,
            vec![c64(1.0, 0.0)],
        );

    assert!(result.is_err());
}

#[test]
fn tensor_from_values_rejects_non_finite_values() {
    let result =
        MpsTensor::<Complex64>::from_values(
            1,
            1,
            vec![
                c64(f64::NAN, 0.0),
                c64(0.0, 0.0),
            ],
        );

    assert!(result.is_err());
}

#[test]
fn tensor_indexing_is_bounds_checked() {
    let tensor =
        MpsTensor::<Complex64>::zeros(2, 3)
            .expect("valid tensor dimensions");

    assert!(tensor.get(0, 0, 0).is_ok());
    assert!(tensor.get(1, 1, 2).is_ok());

    assert!(tensor.get(2, 0, 0).is_err());
    assert!(tensor.get(0, 2, 0).is_err());
    assert!(tensor.get(0, 0, 3).is_err());
}

#[test]
fn tensor_mutation_preserves_shape() {
    let mut tensor =
        MpsTensor::<Complex64>::zeros(1, 1)
            .expect("valid tensor dimensions");

    {
        let value = tensor
            .get_mut(0, 0, 0)
            .expect("valid tensor element");

        *value = c64(1.0, 0.0);
    }

    assert_complex64_close(
        tensor
            .get(0, 0, 0)
            .expect("valid tensor element"),
        c64(1.0, 0.0),
        F64_TOLERANCE,
    );

    assert_eq!(tensor.left_dimension(), 1);
    assert_eq!(tensor.right_dimension(), 1);
}

#[test]
fn tensor_values_mut_can_be_used_without_changing_storage_size() {
    let mut tensor =
        MpsTensor::<Complex64>::zeros(2, 2)
            .expect("valid tensor dimensions");

    let length_before = tensor.values().len();

    for value in tensor.values_mut() {
        *value = c64(0.25, 0.0);
    }

    assert_eq!(
        tensor.values().len(),
        length_before
    );
}

// =============================================================================
// Zero-state construction
// =============================================================================

#[test]
fn zero_state_with_one_qubit_is_correct() {
    let state =
        TensorNetworkState::<Complex64>::zero(
            QubitCount::new(1),
            TruncationPolicy::default(),
        )
        .expect("one-qubit zero state must construct");

    assert_eq!(
        state.qubit_count().get(),
        1
    );

    assert_complex64_close(
        state.amplitude(0).expect("amplitude 0"),
        c64(1.0, 0.0),
        F64_TOLERANCE,
    );

    assert_complex64_close(
        state.amplitude(1).expect("amplitude 1"),
        c64(0.0, 0.0),
        F64_TOLERANCE,
    );

    assert_normalized_f64(&state);
    assert_bond_dimensions_are_valid(&state);
}

#[test]
fn zero_state_has_correct_dimension_for_small_systems() {
    for qubits in 1..=MAX_SMALL_TEST_QUBITS {
        let state =
            TensorNetworkState::<Complex64>::zero(
                QubitCount::new(qubits),
                TruncationPolicy::default(),
            )
            .expect("small zero state must construct");

        assert_eq!(
            state.qubit_count().get(),
            qubits
        );

        assert_complex64_close(
            state.amplitude(0).expect("zero basis amplitude"),
            c64(1.0, 0.0),
            F64_TOLERANCE,
        );

        let last =
            basis_dimension(qubits) - 1;

        assert_complex64_close(
            state.amplitude(last).expect("last basis amplitude"),
            c64(0.0, 0.0),
            F64_TOLERANCE,
        );

        assert_normalized_f64(&state);
        assert_bond_dimensions_are_valid(&state);
    }
}

#[test]
fn zero_state_has_product_bond_dimensions() {
    let state =
        TensorNetworkState::<Complex64>::zero(
            QubitCount::new(6),
            TruncationPolicy::default(),
        )
        .expect("zero state must construct");

    let bonds = state.bond_dimensions();

    assert!(bonds.iter().all(|bond| *bond == 1));
}

// =============================================================================
// Basis-state construction and little-endian ordering
// =============================================================================

#[test]
fn basis_state_zero_matches_zero_state() {
    let policy = TruncationPolicy::default();

    let zero =
        TensorNetworkState::<Complex64>::zero(
            QubitCount::new(4),
            policy,
        )
        .expect("zero state");

    let basis =
        TensorNetworkState::<Complex64>::basis(
            QubitCount::new(4),
            0,
            policy,
        )
        .expect("basis state");

    let zero_dense =
        zero.to_state_vector()
            .expect("dense zero state");

    let basis_dense =
        basis.to_state_vector()
            .expect("dense basis state");

    assert_state_vector_close_f64(
        &basis_dense,
        &zero_dense,
        F64_TOLERANCE,
    );
}

#[test]
fn basis_state_preserves_exact_basis_index() {
    let qubits = 4;
    let basis_index = 0b1010usize;

    let state =
        TensorNetworkState::<Complex64>::basis(
            QubitCount::new(qubits),
            basis_index,
            TruncationPolicy::default(),
        )
        .expect("basis state must construct");

    for index in 0..basis_dimension(qubits) {
        let amplitude =
            state.amplitude(index)
                .expect("amplitude must exist");

        if index == basis_index {
            assert_complex64_close(
                amplitude,
                c64(1.0, 0.0),
                F64_TOLERANCE,
            );
        } else {
            assert_complex64_close(
                amplitude,
                c64(0.0, 0.0),
                F64_TOLERANCE,
            );
        }
    }
}

#[test]
fn basis_state_demonstrates_little_endian_qubit_ordering() {
    let state =
        TensorNetworkState::<Complex64>::basis(
            QubitCount::new(3),
            0b001,
            TruncationPolicy::default(),
        )
        .expect("basis state must construct");

    assert_close_f64(
        state.probability(0b001)
            .expect("probability must exist"),
        1.0,
        F64_TOLERANCE,
    );

    assert_close_f64(
        state.probability(0b100)
            .expect("probability must exist"),
        0.0,
        F64_TOLERANCE,
    );
}

#[test]
fn basis_index_equal_to_dimension_is_rejected() {
    let result =
        TensorNetworkState::<Complex64>::basis(
            QubitCount::new(3),
            8,
            TruncationPolicy::default(),
        );

    assert!(result.is_err());
}

// =============================================================================
// State-vector conversion
// =============================================================================

#[test]
fn zero_state_round_trips_through_state_vector() {
    let policy = TruncationPolicy::default();

    let mps =
        TensorNetworkState::<Complex64>::zero(
            QubitCount::new(5),
            policy,
        )
        .expect("MPS construction");

    let dense =
        mps.to_state_vector()
            .expect("MPS to dense conversion");

    let expected =
        StateVector::<Complex64>::zero(
            QubitCount::new(5),
        )
        .expect("dense zero state");

    assert_state_vector_close_f64(
        &dense,
        &expected,
        F64_TOLERANCE,
    );
}

#[test]
fn basis_state_round_trips_through_state_vector() {
    let qubits = 5;
    let basis = 0b10101usize;

    let mps =
        TensorNetworkState::<Complex64>::basis(
            QubitCount::new(qubits),
            basis,
            TruncationPolicy::default(),
        )
        .expect("MPS construction");

    let dense =
        mps.to_state_vector()
            .expect("MPS to dense conversion");

    let expected =
        StateVector::<Complex64>::basis(
            QubitCount::new(qubits),
            basis,
        )
        .expect("dense basis state");

    assert_state_vector_close_f64(
        &dense,
        &expected,
        F64_TOLERANCE,
    );
}

#[test]
fn bell_state_round_trip_is_exact_with_sufficient_bond_dimension() {
    let source = bell_state_f64();

    let policy = TruncationPolicy {
        max_bond_dimension: 2,
        absolute_cutoff: 0.0,
        relative_cutoff: 0.0,
        maximum_discarded_weight: 0.0,
        allow_truncation: false,
    };

    let mps =
        TensorNetworkState::<Complex64>::from_state_vector(
            &source,
            policy,
        )
        .expect("Bell state must fit exactly in bond dimension two");

    let restored =
        mps.to_state_vector()
            .expect("Bell state must convert back");

    assert_state_vector_close_f64(
        &restored,
        &source,
        F64_TOLERANCE,
    );

    assert_bond_dimensions_are_valid(&mps);
}

#[test]
fn ghz_state_round_trip_preserves_amplitudes() {
    let source = ghz_state_f64(5);

    let policy = TruncationPolicy {
        max_bond_dimension: 2,
        absolute_cutoff: 0.0,
        relative_cutoff: 0.0,
        maximum_discarded_weight: 0.0,
        allow_truncation: false,
    };

    let mps =
        TensorNetworkState::<Complex64>::from_state_vector(
            &source,
            policy,
        )
        .expect("GHZ state must fit bond dimension two");

    let restored =
        mps.to_state_vector()
            .expect("GHZ state must convert back");

    assert_state_vector_close_f64(
        &restored,
        &source,
        F64_TOLERANCE,
    );
}

#[test]
fn complex_amplitudes_survive_round_trip() {
    let source = complex_phase_state_f64();

    let policy = TruncationPolicy {
        max_bond_dimension: 2,
        absolute_cutoff: 0.0,
        relative_cutoff: 0.0,
        maximum_discarded_weight: 0.0,
        allow_truncation: false,
    };

    let mps =
        TensorNetworkState::<Complex64>::from_state_vector(
            &source,
            policy,
        )
        .expect("complex state must convert");

    let restored =
        mps.to_state_vector()
            .expect("complex state must restore");

    assert_state_vector_close_f64(
        &restored,
        &source,
        F64_TOLERANCE,
    );
}

// =============================================================================
// Normalization and probability invariants
// =============================================================================

#[test]
fn zero_state_is_normalized() {
    let state =
        TensorNetworkState::<Complex64>::zero(
            QubitCount::new(6),
            TruncationPolicy::default(),
        )
        .expect("zero state");

    assert_normalized_f64(&state);
}

#[test]
fn basis_state_is_normalized() {
    let state =
        TensorNetworkState::<Complex64>::basis(
            QubitCount::new(6),
            37,
            TruncationPolicy::default(),
        )
        .expect("basis state");

    assert_normalized_f64(&state);
}

#[test]
fn probabilities_are_non_negative_and_sum_to_one() {
    let state =
        TensorNetworkState::<Complex64>::basis(
            QubitCount::new(5),
            0b10101,
            TruncationPolicy::default(),
        )
        .expect("basis state");

    assert_probabilities_sum_to_one_f64(&state);
}

#[test]
fn basis_state_has_probability_one_at_selected_index() {
    let basis = 0b10101usize;

    let state =
        TensorNetworkState::<Complex64>::basis(
            QubitCount::new(5),
            basis,
            TruncationPolicy::default(),
        )
        .expect("basis state");

    assert_close_f64(
        state.probability(basis)
            .expect("basis probability"),
        1.0,
        F64_TOLERANCE,
    );
}

#[test]
fn basis_state_has_zero_probability_elsewhere() {
    let basis = 0b10101usize;

    let state =
        TensorNetworkState::<Complex64>::basis(
            QubitCount::new(5),
            basis,
            TruncationPolicy::default(),
        )
        .expect("basis state");

    for index in 0..basis_dimension(5) {
        if index != basis {
            assert_close_f64(
                state.probability(index)
                    .expect("probability"),
                0.0,
                F64_TOLERANCE,
            );
        }
    }
}

#[test]
fn probability_out_of_bounds_is_rejected() {
    let state =
        TensorNetworkState::<Complex64>::zero(
            QubitCount::new(3),
            TruncationPolicy::default(),
        )
        .expect("zero state");

    assert!(state.probability(8).is_err());
}

// =============================================================================
// Bond-dimension and representation invariants
// =============================================================================

#[test]
fn product_states_have_bond_dimension_one() {
    let state =
        TensorNetworkState::<Complex64>::basis(
            QubitCount::new(8),
            0b10100101,
            TruncationPolicy::default(),
        )
        .expect("basis state");

    let bonds = state.bond_dimensions();

    assert!(bonds.iter().all(|bond| *bond == 1));
}

#[test]
fn bell_state_requires_no_more_than_two_bond_dimension() {
    let source = bell_state_f64();

    let policy = TruncationPolicy {
        max_bond_dimension: 2,
        absolute_cutoff: 0.0,
        relative_cutoff: 0.0,
        maximum_discarded_weight: 0.0,
        allow_truncation: false,
    };

    let state =
        TensorNetworkState::<Complex64>::from_state_vector(
            &source,
            policy,
        )
        .expect("Bell state must fit bond dimension two");

    for bond in state.bond_dimensions() {
        assert!(*bond <= 2);
    }
}

#[test]
fn ghz_state_requires_no_more_than_two_bond_dimension() {
    let source = ghz_state_f64(7);

    let policy = TruncationPolicy {
        max_bond_dimension: 2,
        absolute_cutoff: 0.0,
        relative_cutoff: 0.0,
        maximum_discarded_weight: 0.0,
        allow_truncation: false,
    };

    let state =
        TensorNetworkState::<Complex64>::from_state_vector(
            &source,
            policy,
        )
        .expect("GHZ state must fit bond dimension two");

    for bond in state.bond_dimensions() {
        assert!(*bond <= 2);
    }
}

#[test]
fn tensor_network_validation_accepts_valid_product_states() {
    for qubits in 1..=MAX_SMALL_TEST_QUBITS {
        let state =
            TensorNetworkState::<Complex64>::zero(
                QubitCount::new(qubits),
                TruncationPolicy::default(),
            )
            .expect("valid product state");

        state
            .validate()
            .expect("valid product MPS must validate");
    }
}

#[test]
fn tensor_network_validation_accepts_valid_entangled_states() {
    let source = ghz_state_f64(6);

    let policy = TruncationPolicy {
        max_bond_dimension: 2,
        absolute_cutoff: 0.0,
        relative_cutoff: 0.0,
        maximum_discarded_weight: 0.0,
        allow_truncation: false,
    };

    let state =
        TensorNetworkState::<Complex64>::from_state_vector(
            &source,
            policy,
        )
        .expect("valid GHZ MPS");

    state
        .validate()
        .expect("valid GHZ MPS must validate");
}

// =============================================================================
// Truncation behavior
// =============================================================================

#[test]
fn truncation_can_be_disabled_explicitly() {
    let policy = TruncationPolicy {
        max_bond_dimension: 2,
        absolute_cutoff: 0.0,
        relative_cutoff: 0.0,
        maximum_discarded_weight: 0.0,
        allow_truncation: false,
    };

    policy
        .validate()
        .expect("policy itself is valid");

    assert!(!policy.allow_truncation);
}

#[test]
fn exact_product_state_requires_no_truncation() {
    let source =
        StateVector::<Complex64>::basis(
            QubitCount::new(6),
            0b101011,
        )
        .expect("basis state");

    let policy = TruncationPolicy {
        max_bond_dimension: 1,
        absolute_cutoff: 0.0,
        relative_cutoff: 0.0,
        maximum_discarded_weight: 0.0,
        allow_truncation: false,
    };

    let state =
        TensorNetworkState::<Complex64>::from_state_vector(
            &source,
            policy,
        )
        .expect("product state must fit exactly");

    let restored =
        state.to_state_vector()
            .expect("restoration");

    assert_state_vector_close_f64(
        &restored,
        &source,
        F64_TOLERANCE,
    );
}

#[test]
fn insufficient_bond_dimension_is_rejected_when_truncation_is_disabled() {
    let source = bell_state_f64();

    let policy = TruncationPolicy {
        max_bond_dimension: 1,
        absolute_cutoff: 0.0,
        relative_cutoff: 0.0,
        maximum_discarded_weight: 0.0,
        allow_truncation: false,
    };

    let result =
        TensorNetworkState::<Complex64>::from_state_vector(
            &source,
            policy,
        );

    assert!(
        result.is_err(),
        "an entangled state requiring rank two must not \
         silently truncate to rank one when truncation is disabled"
    );
}

#[test]
fn sufficiently_large_bond_dimension_does_not_require_approximation() {
    let source = ghz_state_f64(6);

    let policy = TruncationPolicy {
        max_bond_dimension: 6,
        absolute_cutoff: 0.0,
        relative_cutoff: 0.0,
        maximum_discarded_weight: 0.0,
        allow_truncation: false,
    };

    let state =
        TensorNetworkState::<Complex64>::from_state_vector(
            &source,
            policy,
        )
        .expect("GHZ state must fit exactly");

    let restored =
        state.to_state_vector()
            .expect("restoration");

    assert_state_vector_close_f64(
        &restored,
        &source,
        F64_TOLERANCE,
    );
}

// =============================================================================
// Determinism
// =============================================================================

#[test]
fn identical_inputs_produce_identical_mps() {
    let source = ghz_state_f64(5);

    let policy = TruncationPolicy {
        max_bond_dimension: 2,
        absolute_cutoff: 0.0,
        relative_cutoff: 0.0,
        maximum_discarded_weight: 0.0,
        allow_truncation: false,
    };

    let first =
        TensorNetworkState::<Complex64>::from_state_vector(
            &source,
            policy,
        )
        .expect("first conversion");

    let second =
        TensorNetworkState::<Complex64>::from_state_vector(
            &source,
            policy,
        )
        .expect("second conversion");

    assert_eq!(
        first,
        second,
        "MPS conversion must be deterministic for identical inputs"
    );
}

#[test]
fn repeated_round_trips_are_stable() {
    let source = ghz_state_f64(5);

    let policy = TruncationPolicy {
        max_bond_dimension: 2,
        absolute_cutoff: 0.0,
        relative_cutoff: 0.0,
        maximum_discarded_weight: 0.0,
        allow_truncation: false,
    };

    let first =
        TensorNetworkState::<Complex64>::from_state_vector(
            &source,
            policy,
        )
        .expect("initial conversion");

    let dense =
        first.to_state_vector()
            .expect("first restoration");

    let second =
        TensorNetworkState::<Complex64>::from_state_vector(
            &dense,
            policy,
        )
        .expect("second conversion");

    let dense_again =
        second.to_state_vector()
            .expect("second restoration");

    assert_state_vector_close_f64(
        &dense_again,
        &source,
        F64_TOLERANCE,
    );

    assert_eq!(
        first,
        second,
        "exact repeated conversion should remain deterministic"
    );
}

// =============================================================================
// Invalid input and safety behavior
// =============================================================================

#[test]
fn invalid_basis_state_is_rejected_without_panicking() {
    let result =
        TensorNetworkState::<Complex64>::basis(
            QubitCount::new(4),
            16,
            TruncationPolicy::default(),
        );

    assert!(result.is_err());
}

#[test]
fn invalid_truncation_policy_is_rejected_before_construction() {
    let policy = TruncationPolicy {
        max_bond_dimension: 0,
        ..TruncationPolicy::default()
    };

    let result =
        TensorNetworkState::<Complex64>::zero(
            QubitCount::new(3),
            policy,
        );

    assert!(result.is_err());
}

#[test]
fn zero_qubit_dense_conversion_is_not_used_as_an_implicit_mps_state() {
    let result =
        TensorNetworkState::<Complex64>::zero(
            QubitCount::new(0),
            TruncationPolicy::default(),
        );

    /*
     * A tensor network representing a non-empty qubit system is the contract
     * currently exposed by this implementation. This test documents that the
     * implementation must not silently invent an empty-site MPS convention.
     */
    assert!(result.is_err());
}

#[test]
fn finite_non_normalized_input_is_normalized_or_rejected_by_the_contract() {
    let source =
        StateVector::<Complex64>::from_amplitudes_unchecked_normalization(
            vec![
                c64(2.0, 0.0),
                c64(0.0, 0.0),
            ],
        )
        .expect("finite state-vector input must be constructible");

    let policy = TruncationPolicy {
        max_bond_dimension: 1,
        absolute_cutoff: 0.0,
        relative_cutoff: 0.0,
        maximum_discarded_weight: 0.0,
        allow_truncation: false,
    };

    let result =
        TensorNetworkState::<Complex64>::from_state_vector(
            &source,
            policy,
        );

    match result {
        Ok(state) => {
            state
                .validate()
                .expect("accepted MPS must validate");

            assert_normalized_f64(&state);
        }
        Err(_) => {
            /*
             * Rejecting an invalid normalization is also acceptable for a
             * production state representation. What is forbidden is silently
             * returning a malformed normalized-state object.
             */
        }
    }
}

// =============================================================================
// Precision portability
// =============================================================================

#[test]
fn zero_state_supports_complex32() {
    let state =
        TensorNetworkState::<Complex32>::zero(
            QubitCount::new(4),
            TruncationPolicy::default(),
        )
        .expect("Complex32 zero MPS must construct");

    let dense =
        state.to_state_vector()
            .expect("Complex32 MPS must convert");

    let expected =
        StateVector::<Complex32>::zero(
            QubitCount::new(4),
        )
        .expect("Complex32 dense zero state");

    assert_state_vector_close_f32(
        &dense,
        &expected,
        F32_TOLERANCE,
    );
}

#[test]
fn basis_state_supports_complex32() {
    let basis = 0b1011usize;

    let state =
        TensorNetworkState::<Complex32>::basis(
            QubitCount::new(4),
            basis,
            TruncationPolicy::default(),
        )
        .expect("Complex32 basis MPS must construct");

    let dense =
        state.to_state_vector()
            .expect("Complex32 MPS must convert");

    let expected =
        StateVector::<Complex32>::basis(
            QubitCount::new(4),
            basis,
        )
        .expect("Complex32 dense basis state");

    assert_state_vector_close_f32(
        &dense,
        &expected,
        F32_TOLERANCE,
    );
}

#[test]
fn complex32_bell_state_round_trip_preserves_state() {
    let inverse_sqrt_two =
        1.0_f32 / 2.0_f32.sqrt();

    let source =
        StateVector::<Complex32>::from_amplitudes_normalized(
            vec![
                c32(inverse_sqrt_two, 0.0),
                c32(0.0, 0.0),
                c32(0.0, 0.0),
                c32(inverse_sqrt_two, 0.0),
            ],
        )
        .expect("Complex32 Bell state");

    let policy = TruncationPolicy {
        max_bond_dimension: 2,
        absolute_cutoff: 0.0,
        relative_cutoff: 0.0,
        maximum_discarded_weight: 0.0,
        allow_truncation: false,
    };

    let mps =
        TensorNetworkState::<Complex32>::from_state_vector(
            &source,
            policy,
        )
        .expect("Complex32 Bell MPS");

    let restored =
        mps.to_state_vector()
            .expect("Complex32 Bell restoration");

    assert_state_vector_close_f32(
        &restored,
        &source,
        F32_TOLERANCE,
    );
}

// =============================================================================
// Mathematical regression tests
// =============================================================================

#[test]
fn single_qubit_zero_state_has_expected_probability_distribution() {
    let state =
        TensorNetworkState::<Complex64>::zero(
            QubitCount::new(1),
            TruncationPolicy::default(),
        )
        .expect("zero state");

    assert_close_f64(
        state.probability(0).expect("P0"),
        1.0,
        F64_TOLERANCE,
    );

    assert_close_f64(
        state.probability(1).expect("P1"),
        0.0,
        F64_TOLERANCE,
    );
}

#[test]
fn single_qubit_one_state_has_expected_probability_distribution() {
    let state =
        TensorNetworkState::<Complex64>::basis(
            QubitCount::new(1),
            1,
            TruncationPolicy::default(),
        )
        .expect("one state");

    assert_close_f64(
        state.probability(0).expect("P0"),
        0.0,
        F64_TOLERANCE,
    );

    assert_close_f64(
        state.probability(1).expect("P1"),
        1.0,
        F64_TOLERANCE,
    );
}

#[test]
fn bell_state_has_expected_computational_probabilities() {
    let source = bell_state_f64();

    let state =
        TensorNetworkState::<Complex64>::from_state_vector(
            &source,
            TruncationPolicy {
                max_bond_dimension: 2,
                absolute_cutoff: 0.0,
                relative_cutoff: 0.0,
                maximum_discarded_weight: 0.0,
                allow_truncation: false,
            },
        )
        .expect("Bell MPS");

    assert_close_f64(
        state.probability(0).expect("P00"),
        0.5,
        F64_TOLERANCE,
    );

    assert_close_f64(
        state.probability(1).expect("P01"),
        0.0,
        F64_TOLERANCE,
    );

    assert_close_f64(
        state.probability(2).expect("P10"),
        0.0,
        F64_TOLERANCE,
    );

    assert_close_f64(
        state.probability(3).expect("P11"),
        0.5,
        F64_TOLERANCE,
    );
}

#[test]
fn ghz_state_has_only_extreme_basis_components() {
    let source = ghz_state_f64(5);

    let state =
        TensorNetworkState::<Complex64>::from_state_vector(
            &source,
            TruncationPolicy {
                max_bond_dimension: 2,
                absolute_cutoff: 0.0,
                relative_cutoff: 0.0,
                maximum_discarded_weight: 0.0,
                allow_truncation: false,
            },
        )
        .expect("GHZ MPS");

    let dimension = basis_dimension(5);

    assert_close_f64(
        state.probability(0).expect("P00000"),
        0.5,
        F64_TOLERANCE,
    );

    assert_close_f64(
        state.probability(dimension - 1)
            .expect("P11111"),
        0.5,
        F64_TOLERANCE,
    );

    for basis in 1..(dimension - 1) {
        assert_close_f64(
            state.probability(basis)
                .expect("probability"),
            0.0,
            F64_TOLERANCE,
        );
    }
}

// =============================================================================
// Memory-growth and bounded-test behavior
// =============================================================================

#[test]
fn small_mps_does_not_require_dense_exponential_storage_for_construction() {
    /*
     * Construction of a product state should depend linearly on the number of
     * sites. We intentionally do not call `to_state_vector()` here.
     *
     * This protects the test suite from accidentally turning a tensor-network
     * scalability test into an exponential dense-allocation test.
     */
    let state =
        TensorNetworkState::<Complex64>::zero(
            QubitCount::new(64),
            TruncationPolicy::default(),
        )
        .expect("64-qubit product MPS should be representable");

    assert_eq!(
        state.qubit_count().get(),
        64
    );

    assert!(
        state
            .bond_dimensions()
            .iter()
            .all(|bond| *bond == 1)
    );

    state
        .validate()
        .expect("64-qubit product MPS must validate");
}

#[test]
fn large_product_mps_remains_provider_neutral() {
    let state =
        TensorNetworkState::<Complex64>::basis(
            QubitCount::new(128),
            0,
            TruncationPolicy::default(),
        )
        .expect("large product MPS should construct");

    assert_eq!(
        state.qubit_count().get(),
        128
    );

    assert!(
        state
            .bond_dimensions()
            .iter()
            .all(|bond| *bond == 1)
    );
}

// =============================================================================
// Error-contract smoke tests
// =============================================================================

#[test]
fn invalid_operations_return_errors_instead_of_panicking() {
    let state =
        TensorNetworkState::<Complex64>::zero(
            QubitCount::new(2),
            TruncationPolicy::default(),
        )
        .expect("valid state");

    let amplitude_result =
        state.amplitude(4);

    let probability_result =
        state.probability(4);

    assert!(amplitude_result.is_err());
    assert!(probability_result.is_err());
}

#[test]
fn errors_are_structured_memory_errors() {
    let result =
        TensorNetworkState::<Complex64>::basis(
            QubitCount::new(2),
            4,
            TruncationPolicy::default(),
        );

    match result {
        Ok(_) => panic!("invalid basis index unexpectedly succeeded"),
        Err(error) => {
            let _typed_error: MemoryError = error;
        }
    }
}

// =============================================================================
// Clone / equality / state identity
// =============================================================================

#[test]
fn cloned_mps_is_equal_to_original() {
    let state =
        TensorNetworkState::<Complex64>::basis(
            QubitCount::new(5),
            0b10101,
            TruncationPolicy::default(),
        )
        .expect("basis MPS");

    let cloned = state.clone();

    assert_eq!(state, cloned);
}

#[test]
fn equal_states_have_equal_bond_dimensions() {
    let state =
        TensorNetworkState::<Complex64>::basis(
            QubitCount::new(6),
            0b100101,
            TruncationPolicy::default(),
        )
        .expect("basis MPS");

    let cloned = state.clone();

    assert_eq!(
        state.bond_dimensions(),
        cloned.bond_dimensions()
    );
}

// =============================================================================
// Provider-neutral contract
// =============================================================================

#[test]
fn tensor_network_does_not_require_a_hardware_provider() {
    /*
     * This test is intentionally simple but architectural:
     *
     * MPS construction, validation and dense conversion must work without:
     *
     * - IBM;
     * - Google;
     * - IonQ;
     * - Rigetti;
     * - Quantinuum;
     * - AWS Braket;
     * - CUDA;
     * - HIP;
     * - Metal;
     * - Vulkan;
     * - MPI;
     * - RDMA.
     *
     * Hardware adapters integrate above/beside memory rather than becoming a
     * dependency of the mathematical representation.
     */
    let state =
        TensorNetworkState::<Complex64>::zero(
            QubitCount::new(4),
            TruncationPolicy::default(),
        )
        .expect("provider-independent construction");

    state
        .validate()
        .expect("provider-independent validation");
}

#[test]
fn tensor_network_semantics_are_compatible_with_backend_neutral_state_validation() {
    let source = bell_state_f64();

    let mps =
        TensorNetworkState::<Complex64>::from_state_vector(
            &source,
            TruncationPolicy {
                max_bond_dimension: 2,
                absolute_cutoff: 0.0,
                relative_cutoff: 0.0,
                maximum_discarded_weight: 0.0,
                allow_truncation: false,
            },
        )
        .expect("provider-neutral MPS");

    let restored =
        mps.to_state_vector()
            .expect("provider-neutral conversion");

    restored
        .validate_normalized()
        .expect("restored state must be normalized");

    assert_state_vector_close_f64(
        &restored,
        &source,
        F64_TOLERANCE,
    );
}