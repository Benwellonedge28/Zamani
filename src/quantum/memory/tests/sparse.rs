//! Zamani Quantum Memory — Sparse-State Integration Tests
//!
//! Production-grade black-box tests for `quantum::memory::sparse`.
//!
//! # Purpose
//!
//! This module verifies the externally observable contract of
//! `SparseState` without depending on:
//!
//! - implementation-private fields;
//! - raw pointers;
//! - `unsafe` code;
//! - a particular CPU architecture;
//! - SIMD instruction sets;
//! - GPU vendors;
//! - QPU vendors;
//! - network transports;
//! - distributed-memory implementations;
//! - routing algorithms;
//! - scheduling algorithms;
//! - compiler syntax;
//! - OpenQASM;
//! - benchmark implementations;
//! - QEC decoder implementations.
//!
//! The same tests must remain valid when sparse-state storage is backed by:
//!
//! ```text
//!                 Quantum IR
//!                     |
//!                     v
//!                  Executor
//!                     |
//!                     v
//!              quantum::memory
//!                     |
//!               SparseState
//!                     |
//!       +-------------+-------------+
//!       |             |             |
//!      CPU           GPU       distributed
//!       |             |             |
//!       +-------------+-------------+
//!                     |
//!                     v
//!              hardware adapters
//! ```
//!
//! A physical QPU normally does not expose its complete wavefunction. In that
//! situation the backend must use an appropriate backend-native memory
//! representation. These tests therefore validate the sparse representation
//! itself rather than requiring every QPU to expose sparse amplitudes.
//!
//! # Architectural boundary
//!
//! `SparseState` owns sparse pure-state storage and state-local mathematical
//! transformations.
//!
//! The following remain outside this test module:
//!
//! - logical/physical qubit identity;
//! - quantum IR;
//! - routing;
//! - scheduling;
//! - hardware topology;
//! - QPU sessions;
//! - GPU allocation;
//! - distributed communication;
//! - measurement RNG policy;
//! - QEC decoding;
//! - benchmark protocols.
//!
//! Those systems must preserve the mathematical and memory-safety invariants
//! tested here when integrating with sparse state storage.
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
//! - no `unsafe`.
//!
//! # Integration contract
//!
//! This file intentionally consumes only public APIs from:
//!
//! - `memory::sparse`;
//! - `memory::complex`;
//! - `memory::errors`;
//! - `memory::types`.
//!
//! It must not require changes merely because a later implementation adds:
//!
//! - CPU acceleration;
//! - SIMD;
//! - GPU execution;
//! - distributed execution;
//! - memory pools;
//! - snapshots;
//! - checkpointing;
//! - migration;
//! - hardware adapters.
//!
//! # Required invariants
//!
//! These tests enforce the following sparse-state properties:
//!
//! 1. no invalid basis index is accepted;
//! 2. no invalid qubit position is accepted;
//! 3. no non-finite amplitude is accepted;
//! 4. explicit zero amplitudes are not retained;
//! 5. duplicate basis entries are merged;
//! 6. insertion does not silently normalize the state;
//! 7. explicit normalization produces unit norm;
//! 8. sparse storage never requires dense `2^n` allocation merely to
//!    represent `|0...0>`;
//! 9. deterministic support ordering is preserved;
//! 10. matrix application preserves the declared sparse-state semantics;
//! 11. measurement/collapse primitives can project support correctly;
//! 12. explicit pruning reports discarded probability mass;
//! 13. pruning is never implicit;
//! 14. state addition requires equal dimensions;
//! 15. inner products respect complex conjugation;
//! 16. fidelity is bounded and correct for valid pure states;
//! 17. `Complex32` and `Complex64` are both supported;
//! 18. zero-qubit states remain valid;
//! 19. arithmetic/basis-boundary conditions do not panic;
//! 20. the sparse representation remains provider-neutral.
//!
//! # Important note
//!
//! These are integration tests, not a replacement for unit tests inside
//! `sparse.rs`. They deliberately test the public contract from outside the
//! implementation so that refactoring the storage implementation does not
//! require rewriting the tests.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use crate::quantum::memory::complex::{Complex32, Complex64};
use crate::quantum::memory::errors::MemoryError;
use crate::quantum::memory::sparse::{
    PruneReport,
    SparseState,
    DEFAULT_PRUNE_PROBABILITY_THRESHOLD_F64,
    MAX_INDEX_BITS,
    SINGLE_QUBIT_MATRIX_ELEMENTS,
    SPARSE_STATE_SCHEMA_ID,
    SPARSE_STATE_SCHEMA_VERSION,
    TWO_QUBIT_MATRIX_ELEMENTS,
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

fn assert_normalized_f64(state: &SparseState<Complex64>) {
    state
        .is_normalized(1.0e-10)
        .expect("normalization query must succeed");

    assert!(
        state
            .is_normalized(1.0e-10)
            .expect("normalization query must succeed"),
        "sparse state is not normalized: norm_squared={:?}",
        state.norm_squared()
    );

    assert_close_f64(state.norm_squared(), 1.0, 1.0e-10);
}

fn assert_normalized_f32(state: &SparseState<Complex32>) {
    assert!(
        state
            .is_normalized(2.0e-5)
            .expect("normalization query must succeed"),
        "sparse state is not normalized: norm_squared={:?}",
        state.norm_squared()
    );

    assert_close_f64(state.norm_squared(), 1.0, 2.0e-5);
}

fn assert_support(
    state: &SparseState<Complex64>,
    expected: &[(usize, Complex64)],
) {
    let actual: Vec<(usize, Complex64)> = state
        .iter()
        .map(|(&basis, &amplitude)| (basis, amplitude))
        .collect();

    assert_eq!(actual.len(), expected.len());

    for (actual_entry, expected_entry) in actual.iter().zip(expected) {
        assert_eq!(actual_entry.0, expected_entry.0);

        assert_complex64_close(
            actual_entry.1,
            expected_entry.1,
            1.0e-12,
        );
    }
}

fn assert_probability(
    state: &SparseState<Complex64>,
    basis: usize,
    expected: f64,
) {
    assert_close_f64(
        state
            .probability(basis)
            .expect("probability query must succeed"),
        expected,
        1.0e-12,
    );
}

// =============================================================================
// Schema and architectural contract
// =============================================================================

#[test]
fn schema_identity_is_stable() {
    assert_eq!(
        SPARSE_STATE_SCHEMA_ID,
        "zamani.quantum.memory.sparse"
    );

    assert_eq!(SPARSE_STATE_SCHEMA_VERSION, 1);
}

#[test]
fn public_matrix_dimensions_are_correct() {
    assert_eq!(SINGLE_QUBIT_MATRIX_ELEMENTS, 4);
    assert_eq!(TWO_QUBIT_MATRIX_ELEMENTS, 16);
}

#[test]
fn default_pruning_threshold_is_positive_and_finite() {
    assert!(DEFAULT_PRUNE_PROBABILITY_THRESHOLD_F64 > 0.0);
    assert!(DEFAULT_PRUNE_PROBABILITY_THRESHOLD_F64.is_finite());
}

#[test]
fn_index_bit_contract_is_valid() {
    assert!(MAX_INDEX_BITS > 0);
    assert!(MAX_INDEX_BITS <= usize::BITS as usize);
}

// =============================================================================
// Construction
// =============================================================================

#[test]
fn zero_qubit_state_is_valid() {
    let state =
        SparseState::<Complex64>::zero(0).expect("zero-qubit state");

    assert_eq!(state.qubits(), 0);
    assert_eq!(state.qubit_count(), QubitCount::ZERO);
    assert_eq!(state.support_len(), 1);
    assert!(!state.is_empty());

    assert_complex64_close(
        state.amplitude(0).expect("basis 0"),
        Complex64::ONE,
        1.0e-12,
    );

    assert_normalized_f64(&state);
    state.validate().expect("zero-qubit state is valid");
}

#[test]
fn empty_register_is_normalized_zero_qubit_state() {
    let state = SparseState::<Complex64>::empty_register();

    assert_eq!(state.qubits(), 0);
    assert_eq!(state.support_len(), 1);

    assert_normalized_f64(&state);
    state.validate().expect("empty register is valid");
}

#[test]
fn zero_state_is_sparse_even_for_large_qubit_count() {
    let state =
        SparseState::<Complex64>::zero(30).expect("30-qubit zero state");

    assert_eq!(state.qubits(), 30);
    assert_eq!(state.support_len(), 1);
    assert_eq!(state.amplitude_count().get(), 1);

    assert_complex64_close(
        state.amplitude(0).expect("basis 0"),
        Complex64::ONE,
        1.0e-12,
    );

    state.validate().expect("state must be valid");
}

#[test]
fn basis_state_is_sparse() {
    let basis = 1usize << 19;

    let state =
        SparseState::<Complex64>::basis_state(20, basis)
            .expect("valid 20-qubit basis state");

    assert_eq!(state.support_len(), 1);
    assert_eq!(state.max_basis_index(), Some(basis));

    assert_complex64_close(
        state.amplitude(basis).expect("basis amplitude"),
        Complex64::ONE,
        1.0e-12,
    );

    assert_normalized_f64(&state);
}

#[test]
fn from_amplitude_accepts_nonzero_finite_amplitude() {
    let amplitude = c64(0.25, -0.5);

    let state =
        SparseState::<Complex64>::from_amplitude(3, 5, amplitude)
            .expect("finite amplitude must be accepted");

    assert_eq!(state.support_len(), 1);
    assert_eq!(state.amplitude_count().get(), 1);

    assert_complex64_close(
        state.amplitude(5).expect("basis amplitude"),
        amplitude,
        1.0e-12,
    );

    assert!(!state.is_normalized(1.0e-12).unwrap());
}

#[test]
fn from_amplitude_does_not_silently_normalize() {
    let amplitude = c64(2.0, 0.0);

    let state =
        SparseState::<Complex64>::from_amplitude(1, 0, amplitude)
            .expect("finite amplitude must be accepted");

    assert_close_f64(state.norm_squared(), 4.0, 1.0e-12);
}

#[test]
fn from_amplitude_zero_produces_empty_support() {
    let state =
        SparseState::<Complex64>::from_amplitude(
            3,
            2,
            Complex64::ZERO,
        )
        .expect("zero amplitude is valid input");

    assert!(state.is_empty());
    assert_eq!(state.support_len(), 0);
    assert_close_f64(state.norm_squared(), 0.0, 1.0e-12);
}

#[test]
fn from_entries_merges_duplicate_basis_indices() {
    let state =
        SparseState::<Complex64>::from_entries(
            2,
            [
                (0, c64(0.25, 0.0)),
                (0, c64(0.75, 0.0)),
            ],
        )
        .expect("duplicate entries must be merged");

    assert_eq!(state.support_len(), 1);

    assert_complex64_close(
        state.amplitude(0).expect("basis amplitude"),
        Complex64::ONE,
        1.0e-12,
    );
}

#[test]
fn duplicate_entries_that_cancel_are_removed() {
    let state =
        SparseState::<Complex64>::from_entries(
            2,
            [
                (1, c64(1.0, 0.0)),
                (1, c64(-1.0, 0.0)),
            ],
        )
        .expect("cancellation must be accepted");

    assert_eq!(state.support_len(), 0);
    assert!(state.is_empty());
}

#[test]
fn explicit_zero_entries_are_not_stored() {
    let state =
        SparseState::<Complex64>::from_entries(
            3,
            [
                (0, Complex64::ZERO),
                (1, Complex64::ZERO),
                (7, Complex64::ONE),
            ],
        )
        .expect("zero entries are valid");

    assert_eq!(state.support_len(), 1);
    assert_eq!(state.basis_indices().copied().collect::<Vec<_>>(), vec![7]);
}

// =============================================================================
// Input validation
// =============================================================================

#[test]
fn basis_index_equal_to_dimension_is_rejected() {
    let result =
        SparseState::<Complex64>::basis_state(3, 8);

    assert!(result.is_err());
}

#[test]
fn basis_index_above_dimension_is_rejected() {
    let result =
        SparseState::<Complex64>::basis_state(3, usize::MAX);

    assert!(result.is_err());
}

#[test]
fn zero_qubit_state_rejects_nonzero_basis_index() {
    let result =
        SparseState::<Complex64>::basis_state(0, 1);

    assert!(result.is_err());
}

#[test]
fn qubit_count_at_index_width_is_rejected() {
    let result =
        SparseState::<Complex64>::zero(MAX_INDEX_BITS);

    assert!(result.is_err());
}

#[test]
fn maximum_representable_qubit_position_is_addressable() {
    if MAX_INDEX_BITS == 0 {
        return;
    }

    let qubit = MAX_INDEX_BITS - 1;

    let state =
        SparseState::<Complex64>::zero(MAX_INDEX_BITS)
            .expect("maximum representable qubit count must be accepted");

    assert!(state.contains_basis(0).is_ok());

    let result =
        state.probability(1usize << qubit);

    assert!(result.is_ok());
}

#[test]
fn out_of_range_qubit_position_is_rejected() {
    let state =
        SparseState::<Complex64>::zero(3)
            .expect("valid state");

    assert!(state.probability(8).is_err());

    let mut mutable =
        state.clone();

    let matrix = [
        Complex64::ONE,
        Complex64::ZERO,
        Complex64::ZERO,
        Complex64::ONE,
    ];

    assert!(
        mutable
            .apply_single_qubit_matrix(3, matrix)
            .is_err()
    );
}

#[test]
fn duplicate_two_qubit_positions_are_rejected() {
    let mut state =
        SparseState::<Complex64>::zero(2)
            .expect("valid state");

    let matrix = [Complex64::ONE; TWO_QUBIT_MATRIX_ELEMENTS];

    assert!(
        state
            .apply_two_qubit_matrix(0, 0, matrix)
            .is_err()
    );
}

#[test]
fn duplicate_projected_qubit_positions_are_rejected() {
    let mut state =
        SparseState::<Complex64>::zero(2)
            .expect("valid state");

    assert!(
        state
            .project_qubits(&[0, 0], &[false, false])
            .is_err()
    );
}

#[test]
fn mismatched_projection_lengths_are_rejected() {
    let mut state =
        SparseState::<Complex64>::zero(2)
            .expect("valid state");

    assert!(
        state
            .project_qubits(&[0], &[false, true])
            .is_err()
    );
}

#[test]
fn non_finite_nan_amplitude_is_rejected() {
    let result =
        SparseState::<Complex64>::from_amplitude(
            2,
            0,
            c64(f64::NAN, 0.0),
        );

    assert!(result.is_err());
}

#[test]
fn non_finite_infinite_amplitude_is_rejected() {
    let result =
        SparseState::<Complex64>::from_amplitude(
            2,
            0,
            c64(f64::INFINITY, 0.0),
        );

    assert!(result.is_err());
}

#[test]
fn non_finite_negative_infinite_amplitude_is_rejected() {
    let result =
        SparseState::<Complex64>::from_amplitude(
            2,
            0,
            c64(f64::NEG_INFINITY, 0.0),
        );

    assert!(result.is_err());
}

#[test]
fn non_finite_matrix_entry_is_rejected() {
    let mut state =
        SparseState::<Complex64>::zero(1)
            .expect("valid state");

    let matrix = [
        c64(f64::NAN, 0.0),
        Complex64::ZERO,
        Complex64::ZERO,
        Complex64::ONE,
    ];

    assert!(
        state
            .apply_single_qubit_matrix(0, matrix)
            .is_err()
    );
}

// =============================================================================
// Deterministic support and access
// =============================================================================

#[test]
fn support_iteration_is_deterministic_and_sorted() {
    let state =
        SparseState::<Complex64>::from_entries(
            4,
            [
                (15, Complex64::ONE),
                (3, Complex64::ONE),
                (8, Complex64::ONE),
                (1, Complex64::ONE),
            ],
        )
        .expect("valid sparse state");

    let indices: Vec<usize> =
        state.basis_indices().copied().collect();

    assert_eq!(indices, vec![1, 3, 8, 15]);
}

#[test]
fn iterator_returns_matching_basis_and_amplitudes() {
    let state =
        SparseState::<Complex64>::from_entries(
            3,
            [
                (1, c64(0.5, 0.0)),
                (6, c64(0.0, 0.5)),
            ],
        )
        .expect("valid sparse state");

    let entries: Vec<(usize, Complex64)> = state
        .iter()
        .map(|(&basis, &amplitude)| (basis, amplitude))
        .collect();

    assert_eq!(entries.len(), 2);

    assert_eq!(entries[0].0, 1);
    assert_complex64_close(
        entries[0].1,
        c64(0.5, 0.0),
        1.0e-12,
    );

    assert_eq!(entries[1].0, 6);
    assert_complex64_close(
        entries[1].1,
        c64(0.0, 0.5),
        1.0e-12,
    );
}

#[test]
fn missing_basis_amplitude_is_zero() {
    let state =
        SparseState::<Complex64>::basis_state(4, 7)
            .expect("valid basis state");

    assert_complex64_close(
        state.amplitude(0).expect("valid basis"),
        Complex64::ZERO,
        1.0e-12,
    );

    assert_complex64_close(
        state.amplitude(6).expect("valid basis"),
        Complex64::ZERO,
        1.0e-12,
    );
}

#[test]
fn contains_basis_distinguishes_support_from_zero_amplitude() {
    let state =
        SparseState::<Complex64>::basis_state(3, 5)
            .expect("valid basis state");

    assert!(
        state
            .contains_basis(5)
            .expect("valid basis query")
    );

    assert!(
        !state
            .contains_basis(4)
            .expect("valid basis query")
    );
}

// =============================================================================
// Mutation
// =============================================================================

#[test]
fn set_amplitude_inserts_new_support() {
    let mut state =
        SparseState::<Complex64>::zero(3)
            .expect("valid state");

    state
        .set_amplitude(5, c64(0.25, 0.5))
        .expect("valid amplitude");

    assert_eq!(state.support_len(), 2);

    assert_complex64_close(
        state.amplitude(5).expect("basis amplitude"),
        c64(0.25, 0.5),
        1.0e-12,
    );
}

#[test]
fn set_amplitude_replaces_existing_value() {
    let mut state =
        SparseState::<Complex64>::basis_state(3, 2)
            .expect("valid state");

    state
        .set_amplitude(2, c64(0.25, -0.5))
        .expect("valid amplitude");

    assert_eq!(state.support_len(), 1);

    assert_complex64_close(
        state.amplitude(2).expect("basis amplitude"),
        c64(0.25, -0.5),
        1.0e-12,
    );
}

#[test]
fn set_zero_amplitude_removes_support() {
    let mut state =
        SparseState::<Complex64>::basis_state(3, 2)
            .expect("valid state");

    state
        .set_amplitude(2, Complex64::ZERO)
        .expect("zero is valid");

    assert_eq!(state.support_len(), 0);
    assert!(!state.contains_basis(2).unwrap());
}

#[test]
fn add_amplitude_updates_existing_support() {
    let mut state =
        SparseState::<Complex64>::from_amplitude(
            2,
            1,
            c64(0.25, 0.0),
        )
        .expect("valid state");

    state
        .add_amplitude(1, c64(0.75, 0.0))
        .expect("valid amplitude");

    assert_complex64_close(
        state.amplitude(1).expect("basis amplitude"),
        Complex64::ONE,
        1.0e-12,
    );
}

#[test]
fn add_amplitude_cancellation_removes_entry() {
    let mut state =
        SparseState::<Complex64>::from_amplitude(
            2,
            1,
            Complex64::ONE,
        )
        .expect("valid state");

    state
        .add_amplitude(1, -Complex64::ONE)
        .expect("valid cancellation");

    assert_eq!(state.support_len(), 0);
}

#[test]
fn remove_amplitude_returns_previous_value() {
    let mut state =
        SparseState::<Complex64>::from_amplitude(
            2,
            3,
            c64(0.25, -0.5),
        )
        .expect("valid state");

    let removed = state
        .remove_amplitude(3)
        .expect("valid removal")
        .expect("entry must exist");

    assert_complex64_close(
        removed,
        c64(0.25, -0.5),
        1.0e-12,
    );

    assert_eq!(state.support_len(), 0);
}

#[test]
fn remove_missing_amplitude_returns_none() {
    let mut state =
        SparseState::<Complex64>::zero(3)
            .expect("valid state");

    assert_eq!(
        state.remove_amplitude(7).unwrap(),
        None
    );
}

#[test]
fn clear_support_creates_zero_vector_without_changing_dimension() {
    let mut state =
        SparseState::<Complex64>::basis_state(8, 17)
            .expect("valid state");

    state.clear_support();

    assert_eq!(state.qubits(), 8);
    assert_eq!(state.support_len(), 0);
    assert_close_f64(
        state.norm_squared(),
        0.0,
        1.0e-12,
    );
}

#[test]
fn reset_to_zero_restores_normalized_zero_basis_state() {
    let mut state =
        SparseState::<Complex64>::basis_state(8, 17)
            .expect("valid state");

    state.reset_to_zero();

    assert_eq!(state.qubits(), 8);
    assert_eq!(state.support_len(), 1);

    assert_complex64_close(
        state.amplitude(0).unwrap(),
        Complex64::ONE,
        1.0e-12,
    );

    assert_normalized_f64(&state);
}

// =============================================================================
// Norm, probability and normalization
// =============================================================================

#[test]
fn norm_squared_is_sum_of_amplitude_probabilities() {
    let state =
        SparseState::<Complex64>::from_entries(
            2,
            [
                (0, c64(0.5, 0.0)),
                (1, c64(0.0, 0.5)),
                (2, c64(0.5, 0.0)),
                (3, c64(0.0, -0.5)),
            ],
        )
        .expect("valid state");

    assert_close_f64(
        state.norm_squared(),
        1.0,
        1.0e-12,
    );

    assert_normalized_f64(&state);
}

#[test]
fn probability_is_amplitude_modulus_squared() {
    let state =
        SparseState::<Complex64>::from_amplitude(
            2,
            1,
            c64(0.3, 0.4),
        )
        .expect("valid state");

    assert_probability(&state, 1, 0.25);
}

#[test]
fn probabilities_return_only_sparse_support() {
    let state =
        SparseState::<Complex64>::from_entries(
            3,
            [
                (1, c64(0.5, 0.0)),
                (6, c64(0.0, 0.5)),
            ],
        )
        .expect("valid state");

    let probabilities =
        state.probabilities().expect("valid probabilities");

    assert_eq!(
        probabilities,
        vec![(1, 0.25), (6, 0.25)]
    );
}

#[test]
fn normalize_f64_produces_unit_norm() {
    let mut state =
        SparseState::<Complex64>::from_entries(
            1,
            [
                (0, c64(3.0, 0.0)),
                (1, c64(4.0, 0.0)),
            ],
        )
        .expect("valid state");

    state.normalize_f64().expect("normalization");

    assert_normalized_f64(&state);

    assert_probability(&state, 0, 0.36);
    assert_probability(&state, 1, 0.64);
}

#[test]
fn normalized_returns_normalized_clone_without_mutating_original() {
    let state =
        SparseState::<Complex64>::from_entries(
            1,
            [
                (0, c64(3.0, 0.0)),
                (1, c64(4.0, 0.0)),
            ],
        )
        .expect("valid state");

    let normalized =
        state.normalized().expect("normalization");

    assert_close_f64(
        state.norm_squared(),
        25.0,
        1.0e-12,
    );

    assert_normalized_f64(&normalized);
}

#[test]
fn normalization_of_zero_state_is_rejected() {
    let mut state =
        SparseState::<Complex64>::zero(2)
            .expect("valid state");

    state.clear_support();

    assert!(state.normalize_f64().is_err());
}

#[test]
fn normalization_preserves_support() {
    let value = 1.0 / 2.0_f64.sqrt();

    let mut state =
        SparseState::<Complex64>::from_entries(
            1,
            [
                (0, c64(value, 0.0)),
                (1, c64(value, 0.0)),
            ],
        )
        .expect("valid state");

    let before = state.support_len();

    state.normalize_f64().expect("normalization");

    assert_eq!(state.support_len(), before);
    assert_normalized_f64(&state);
}

// =============================================================================
// Single-qubit linear algebra
// =============================================================================

#[test]
fn identity_single_qubit_matrix_preserves_zero_state() {
    let mut state =
        SparseState::<Complex64>::zero(3)
            .expect("valid state");

    let identity = [
        Complex64::ONE,
        Complex64::ZERO,
        Complex64::ZERO,
        Complex64::ONE,
    ];

    state
        .apply_single_qubit_matrix(1, identity)
        .expect("identity");

    assert_support(
        &state,
        &[(0, Complex64::ONE)],
    );
}

#[test]
fn pauli_x_moves_zero_to_one() {
    let mut state =
        SparseState::<Complex64>::zero(1)
            .expect("valid state");

    let x = [
        Complex64::ZERO,
        Complex64::ONE,
        Complex64::ONE,
        Complex64::ZERO,
    ];

    state
        .apply_single_qubit_matrix(0, x)
        .expect("X gate");

    assert_eq!(state.support_len(), 1);

    assert_complex64_close(
        state.amplitude(0).unwrap(),
        Complex64::ZERO,
        1.0e-12,
    );

    assert_complex64_close(
        state.amplitude(1).unwrap(),
        Complex64::ONE,
        1.0e-12,
    );

    assert_normalized_f64(&state);
}

#[test]
fn pauli_x_is_its_own_inverse() {
    let mut state =
        SparseState::<Complex64>::basis_state(3, 5)
            .expect("valid state");

    let x = [
        Complex64::ZERO,
        Complex64::ONE,
        Complex64::ONE,
        Complex64::ZERO,
    ];

    state
        .apply_single_qubit_matrix(1, x)
        .expect("first X");

    state
        .apply_single_qubit_matrix(1, x)
        .expect("second X");

    assert_support(
        &state,
        &[(5, Complex64::ONE)],
    );
}

#[test]
fn hadamard_creates_two_equal_probabilities() {
    let mut state =
        SparseState::<Complex64>::zero(1)
            .expect("valid state");

    let inverse_sqrt_two =
        1.0 / 2.0_f64.sqrt();

    let h = [
        c64(inverse_sqrt_two, 0.0),
        c64(inverse_sqrt_two, 0.0),
        c64(inverse_sqrt_two, 0.0),
        c64(-inverse_sqrt_two, 0.0),
    ];

    state
        .apply_single_qubit_matrix(0, h)
        .expect("Hadamard");

    assert_eq!(state.support_len(), 2);

    assert_probability(&state, 0, 0.5);
    assert_probability(&state, 1, 0.5);

    assert_normalized_f64(&state);
}

#[test]
fn single_qubit_matrix_can_create_new_sparse_support() {
    let mut state =
        SparseState::<Complex64>::basis_state(4, 0)
            .expect("valid state");

    let h = {
        let value =
            1.0 / 2.0_f64.sqrt();

        [
            c64(value, 0.0),
            c64(value, 0.0),
            c64(value, 0.0),
            c64(-value, 0.0),
        ]
    };

    state
        .apply_single_qubit_matrix(2, h)
        .expect("Hadamard");

    assert_eq!(state.support_len(), 2);
    assert_probability(&state, 0, 0.5);
    assert_probability(&state, 4, 0.5);

    assert_normalized_f64(&state);
}

// =============================================================================
// Two-qubit linear algebra
// =============================================================================

#[test]
fn identity_two_qubit_matrix_preserves_state() {
    let mut state =
        SparseState::<Complex64>::basis_state(4, 9)
            .expect("valid state");

    let mut identity = [Complex64::ZERO; TWO_QUBIT_MATRIX_ELEMENTS];

    identity[0] = Complex64::ONE;
    identity[5] = Complex64::ONE;
    identity[10] = Complex64::ONE;
    identity[15] = Complex64::ONE;

    state
        .apply_two_qubit_matrix(1, 3, identity)
        .expect("identity");

    assert_support(
        &state,
        &[(9, Complex64::ONE)],
    );
}

#[test]
fn cnot_maps_control_one_target_zero_to_control_one_target_one() {
    let mut state =
        SparseState::<Complex64>::basis_state(2, 1)
            .expect("valid state");

    let cnot = [
        Complex64::ONE,
        Complex64::ZERO,
        Complex64::ZERO,
        Complex64::ZERO,

        Complex64::ZERO,
        Complex64::ONE,
        Complex64::ZERO,
        Complex64::ZERO,

        Complex64::ZERO,
        Complex64::ZERO,
        Complex64::ZERO,
        Complex64::ONE,

        Complex64::ZERO,
        Complex64::ZERO,
        Complex64::ONE,
        Complex64::ZERO,
    ];

    state
        .apply_two_qubit_matrix(0, 1, cnot)
        .expect("CNOT");

    assert_eq!(state.support_len(), 1);

    assert_complex64_close(
        state.amplitude(1).unwrap(),
        Complex64::ZERO,
        1.0e-12,
    );

    assert_complex64_close(
        state.amplitude(3).unwrap(),
        Complex64::ONE,
        1.0e-12,
    );
}

#[test]
fn two_qubit_matrix_preserves_normalization_for_unitary() {
    let mut state =
        SparseState::<Complex64>::zero(2)
            .expect("valid state");

    let h = 1.0 / 2.0_f64.sqrt();

    let matrix = [
        c64(h, 0.0),
        c64(h, 0.0),
        c64(h, 0.0),
        c64(h, 0.0),

        c64(h, 0.0),
        c64(-h, 0.0),
        c64(h, 0.0),
        c64(-h, 0.0),

        c64(h, 0.0),
        c64(h, 0.0),
        c64(-h, 0.0),
        c64(-h, 0.0),

        c64(h, 0.0),
        c64(-h, 0.0),
        c64(-h, 0.0),
        c64(h, 0.0),
    ];

    state
        .apply_two_qubit_matrix(0, 1, matrix)
        .expect("unitary operation");

    assert_normalized_f64(&state);
}

// =============================================================================
// Projection / collapse primitives
// =============================================================================

#[test]
fn project_qubit_keeps_only_requested_bit_value() {
    let mut state =
        SparseState::<Complex64>::from_entries(
            2,
            [
                (0, Complex64::ONE),
                (1, Complex64::ONE),
                (2, Complex64::ONE),
                (3, Complex64::ONE),
            ],
        )
        .expect("valid state");

    state
        .project_qubit(0, true)
        .expect("projection");

    assert_eq!(
        state
            .basis_indices()
            .copied()
            .collect::<Vec<_>>(),
        vec![1, 3]
    );
}

#[test]
fn project_qubit_zero_keeps_zero_bit_support() {
    let mut state =
        SparseState::<Complex64>::from_entries(
            2,
            [
                (0, Complex64::ONE),
                (1, Complex64::ONE),
                (2, Complex64::ONE),
                (3, Complex64::ONE),
            ],
        )
        .expect("valid state");

    state
        .project_qubit(1, false)
        .expect("projection");

    assert_eq!(
        state
            .basis_indices()
            .copied()
            .collect::<Vec<_>>(),
        vec![0, 1]
    );
}

#[test]
fn project_basis_keeps_exact_basis_state() {
    let mut state =
        SparseState::<Complex64>::from_entries(
            3,
            [
                (1, c64(0.25, 0.0)),
                (2, c64(0.5, 0.0)),
                (6, c64(0.75, 0.0)),
            ],
        )
        .expect("valid state");

    state
        .project_basis(2)
        .expect("projection");

    assert_eq!(state.support_len(), 1);

    assert_complex64_close(
        state.amplitude(2).unwrap(),
        c64(0.5, 0.0),
        1.0e-12,
    );
}

#[test]
fn project_basis_missing_state_produces_zero_support() {
    let mut state =
        SparseState::<Complex64>::basis_state(3, 2)
            .expect("valid state");

    state
        .project_basis(5)
        .expect("valid projection");

    assert!(state.is_empty());
}

#[test]
fn project_multiple_qubits_filters_by_all_constraints() {
    let mut state =
        SparseState::<Complex64>::from_entries(
            3,
            [
                (0, Complex64::ONE),
                (1, Complex64::ONE),
                (2, Complex64::ONE),
                (3, Complex64::ONE),
                (4, Complex64::ONE),
                (5, Complex64::ONE),
                (6, Complex64::ONE),
                (7, Complex64::ONE),
            ],
        )
        .expect("valid state");

    state
        .project_qubits(
            &[0, 2],
            &[true, true],
        )
        .expect("projection");

    assert_eq!(
        state
            .basis_indices()
            .copied()
            .collect::<Vec<_>>(),
        vec![5, 7]
    );
}

// =============================================================================
// Explicit pruning
// =============================================================================

#[test]
fn pruning_is_explicit_and_reports_removed_probability() {
    let small =
        c64(1.0e-8, 0.0);

    let mut state =
        SparseState::<Complex64>::from_entries(
            1,
            [
                (0, Complex64::ONE),
                (1, small),
            ],
        )
        .expect("valid state");

    let report =
        state
            .prune_below(1.0e-12)
            .expect("pruning");

    assert_eq!(report.examined, 2);
    assert_eq!(report.removed, 1);
    assert!(report.discarded_probability > 0.0);
    assert_eq!(state.support_len(), 1);
}

#[test]
fn pruning_does_not_implicitly_normalize() {
    let mut state =
        SparseState::<Complex64>::from_entries(
            1,
            [
                (0, c64(1.0, 0.0)),
                (1, c64(0.1, 0.0)),
            ],
        )
        .expect("valid state");

    let before =
        state.norm_squared();

    state
        .prune_below(0.02)
        .expect("explicit pruning");

    let after =
        state.norm_squared();

    assert!(
        after < before,
        "pruning must not silently renormalize discarded probability"
    );
}

#[test]
fn pruning_zero_threshold_keeps_all_nonzero_support() {
    let mut state =
        SparseState::<Complex64>::from_entries(
            2,
            [
                (0, c64(0.1, 0.0)),
                (1, c64(0.2, 0.0)),
                (2, c64(0.3, 0.0)),
            ],
        )
        .expect("valid state");

    let report =
        state
            .prune_below(0.0)
            .expect("zero threshold");

    assert_eq!(report.removed, 0);
    assert_eq!(state.support_len(), 3);
}

#[test]
fn negative_pruning_threshold_is_rejected() {
    let mut state =
        SparseState::<Complex64>::zero(2)
            .expect("valid state");

    assert!(
        state
            .prune_below(-1.0)
            .is_err()
    );
}

#[test]
fn nan_pruning_threshold_is_rejected() {
    let mut state =
        SparseState::<Complex64>::zero(2)
            .expect("valid state");

    assert!(
        state
            .prune_below(f64::NAN)
            .is_err()
    );
}

#[test]
fn infinite_pruning_threshold_is_rejected() {
    let mut state =
        SparseState::<Complex64>::zero(2)
            .expect("valid state");

    assert!(
        state
            .prune_below(f64::INFINITY)
            .is_err()
    );
}

// =============================================================================
// State combination
// =============================================================================

#[test]
fn add_state_requires_equal_qubit_dimensions() {
    let mut left =
        SparseState::<Complex64>::zero(2)
            .expect("valid state");

    let right =
        SparseState::<Complex64>::zero(3)
            .expect("valid state");

    assert!(
        left
            .add_state(&right)
            .is_err()
    );
}

#[test]
fn add_state_combines_matching_support() {
    let mut left =
        SparseState::<Complex64>::from_amplitude(
            2,
            1,
            c64(0.25, 0.0),
        )
        .expect("valid state");

    let right =
        SparseState::<Complex64>::from_amplitude(
            2,
            1,
            c64(0.75, 0.0),
        )
        .expect("valid state");

    left.add_state(&right)
        .expect("same-dimensional addition");

    assert_complex64_close(
        left.amplitude(1).unwrap(),
        Complex64::ONE,
        1.0e-12,
    );
}

#[test]
fn add_state_is_linear_and_does_not_normalize() {
    let mut left =
        SparseState::<Complex64>::basis_state(1, 0)
            .expect("valid state");

    let right =
        SparseState::<Complex64>::basis_state(1, 0)
            .expect("valid state");

    left.add_state(&right)
        .expect("addition");

    assert_close_f64(
        left.norm_squared(),
        4.0,
        1.0e-12,
    );
}

// =============================================================================
// Inner product and fidelity
// =============================================================================

#[test]
fn inner_product_of_identical_normalized_state_is_one() {
    let value =
        1.0 / 2.0_f64.sqrt();

    let state =
        SparseState::<Complex64>::from_entries(
            1,
            [
                (0, c64(value, 0.0)),
                (1, c64(value, 0.0)),
            ],
        )
        .expect("valid state");

    let overlap =
        state
            .inner_product(&state)
            .expect("inner product");

    assert_complex64_close(
        overlap,
        Complex64::ONE,
        1.0e-12,
    );
}

#[test]
fn inner_product_uses_complex_conjugation() {
    let left =
        SparseState::<Complex64>::from_amplitude(
            1,
            0,
            c64(0.0, 1.0),
        )
        .expect("valid state");

    let right =
        SparseState::<Complex64>::from_amplitude(
            1,
            0,
            Complex64::ONE,
        )
        .expect("valid state");

    let overlap =
        left
            .inner_product(&right)
            .expect("inner product");

    assert_complex64_close(
        overlap,
        c64(0.0, -1.0),
        1.0e-12,
    );
}

#[test]
fn inner_product_requires_equal_dimensions() {
    let left =
        SparseState::<Complex64>::zero(2)
            .expect("valid state");

    let right =
        SparseState::<Complex64>::zero(3)
            .expect("valid state");

    assert!(
        left
            .inner_product(&right)
            .is_err()
    );
}

#[test]
fn orthogonal_basis_states_have_zero_inner_product() {
    let left =
        SparseState::<Complex64>::basis_state(3, 1)
            .expect("valid state");

    let right =
        SparseState::<Complex64>::basis_state(3, 2)
            .expect("valid state");

    let overlap =
        left
            .inner_product(&right)
            .expect("inner product");

    assert_complex64_close(
        overlap,
        Complex64::ZERO,
        1.0e-12,
    );
}

#[test]
fn fidelity_of_identical_basis_states_is_one() {
    let state =
        SparseState::<Complex64>::basis_state(8, 37)
            .expect("valid state");

    let fidelity =
        state
            .fidelity(&state)
            .expect("fidelity");

    assert_close_f64(
        fidelity,
        1.0,
        1.0e-12,
    );
}

#[test]
fn fidelity_of_orthogonal_basis_states_is_zero() {
    let left =
        SparseState::<Complex64>::basis_state(3, 1)
            .expect("valid state");

    let right =
        SparseState::<Complex64>::basis_state(3, 6)
            .expect("valid state");

    let fidelity =
        left
            .fidelity(&right)
            .expect("fidelity");

    assert_close_f64(
        fidelity,
        0.0,
        1.0e-12,
    );
}

#[test]
fn fidelity_requires_equal_dimensions() {
    let left =
        SparseState::<Complex64>::zero(2)
            .expect("valid state");

    let right =
        SparseState::<Complex64>::zero(3)
            .expect("valid state");

    assert!(
        left
            .fidelity(&right)
            .is_err()
    );
}

// =============================================================================
// Scaling
// =============================================================================

#[test]
fn complex_scaling_preserves_support_for_nonzero_factor() {
    let mut state =
        SparseState::<Complex64>::basis_state(2, 1)
            .expect("valid state");

    state
        .scale(c64(0.5, 0.5))
        .expect("complex scaling");

    assert_eq!(state.support_len(), 1);

    assert_complex64_close(
        state.amplitude(1).unwrap(),
        c64(0.5, 0.5),
        1.0e-12,
    );
}

#[test]
fn complex_zero_scaling_clears_support() {
    let mut state =
        SparseState::<Complex64>::basis_state(2, 1)
            .expect("valid state");

    state
        .scale(Complex64::ZERO)
        .expect("zero scaling");

    assert!(state.is_empty());
}

#[test]
fn real_f64_scaling_is_supported() {
    let mut state =
        SparseState::<Complex64>::basis_state(1, 0)
            .expect("valid state");

    state
        .scale_real_f64(0.5)
        .expect("real scaling");

    assert_complex64_close(
        state.amplitude(0).unwrap(),
        c64(0.5, 0.0),
        1.0e-12,
    );
}

#[test]
fn real_f64_nonfinite_scaling_is_rejected() {
    let mut state =
        SparseState::<Complex64>::basis_state(1, 0)
            .expect("valid state");

    assert!(
        state
            .scale_real_f64(f64::NAN)
            .is_err()
    );

    assert!(
        state
            .scale_real_f64(f64::INFINITY)
            .is_err()
    );

    assert!(
        state
            .scale_real_f64(f64::NEG_INFINITY)
            .is_err()
    );
}

// =============================================================================
// Storage accounting
// =============================================================================

#[test]
fn storage_bytes_are_nonzero_for_nonempty_support() {
    let state =
        SparseState::<Complex64>::basis_state(10, 7)
            .expect("valid state");

    assert!(
        state
            .storage_bytes()
            .expect("storage estimate")
            > 0
    );
}

#[test]
fn sparse_storage_depends_on_support_not_dense_hilbert_dimension() {
    let small =
        SparseState::<Complex64>::zero(8)
            .expect("valid state");

    let large =
        SparseState::<Complex64>::zero(30)
            .expect("valid state");

    assert_eq!(
        small.support_len(),
        large.support_len()
    );

    assert_eq!(
        small.storage_bytes().unwrap(),
        large.storage_bytes().unwrap()
    );
}

#[test]
fn storage_estimate_grows_with_support() {
    let one =
        SparseState::<Complex64>::basis_state(8, 0)
            .expect("valid state");

    let many =
        SparseState::<Complex64>::from_entries(
            8,
            [
                (0, Complex64::ONE),
                (1, Complex64::ONE),
                (2, Complex64::ONE),
                (3, Complex64::ONE),
            ],
        )
        .expect("valid state");

    assert!(
        many.storage_bytes().unwrap()
            > one.storage_bytes().unwrap()
    );
}

// =============================================================================
// Validation
// =============================================================================

#[test]
fn validate_accepts_valid_normalized_state() {
    let value =
        1.0 / 2.0_f64.sqrt();

    let state =
        SparseState::<Complex64>::from_entries(
            2,
            [
                (0, c64(value, 0.0)),
                (3, c64(value, 0.0)),
            ],
        )
        .expect("valid state");

    state
        .validate()
        .expect("valid state must validate");
}

#[test]
fn validation_does_not_require_normalization() {
    let state =
        SparseState::<Complex64>::from_amplitude(
            2,
            0,
            c64(2.0, 0.0),
        )
        .expect("finite state");

    state
        .validate()
        .expect("unnormalized finite state is still structurally valid");
}

#[test]
fn mutation_preserves_structural_validity() {
    let mut state =
        SparseState::<Complex64>::zero(4)
            .expect("valid state");

    state
        .set_amplitude(3, c64(0.25, 0.5))
        .expect("set");

    state
        .add_amplitude(3, c64(-0.25, -0.5))
        .expect("cancel");

    state
        .validate()
        .expect("state remains structurally valid");
}

// =============================================================================
// Complex32 portability
// =============================================================================

#[test]
fn complex32_zero_state_is_supported() {
    let state =
        SparseState::<Complex32>::zero(4)
            .expect("Complex32 must be supported");

    assert_eq!(state.qubits(), 4);
    assert_eq!(state.support_len(), 1);

    assert_complex32_close(
        state.amplitude(0).unwrap(),
        Complex32::ONE,
        1.0e-6,
    );

    assert_normalized_f32(&state);
}

#[test]
fn complex32_hadamard_state_is_supported() {
    let mut state =
        SparseState::<Complex32>::zero(1)
            .expect("Complex32 state");

    let value =
        1.0_f32 / 2.0_f32.sqrt();

    let h = [
        c32(value, 0.0),
        c32(value, 0.0),
        c32(value, 0.0),
        c32(-value, 0.0),
    ];

    state
        .apply_single_qubit_matrix(0, h)
        .expect("Complex32 Hadamard");

    assert_eq!(state.support_len(), 2);

    assert_close_f64(
        state.probability(0).unwrap(),
        0.5,
        2.0e-5,
    );

    assert_close_f64(
        state.probability(1).unwrap(),
        0.5,
        2.0e-5,
    );

    assert_normalized_f32(&state);
}

#[test]
fn complex32_normalization_is_supported() {
    let value =
        1.0_f32 / 2.0_f32.sqrt();

    let mut state =
        SparseState::<Complex32>::from_entries(
            1,
            [
                (0, c32(value, 0.0)),
                (1, c32(value, 0.0)),
            ],
        )
        .expect("Complex32 state");

    state
        .normalize_f32()
        .expect("Complex32 normalization");

    assert_normalized_f32(&state);
}

#[test]
fn complex32_nonfinite_amplitude_is_rejected() {
    let result =
        SparseState::<Complex32>::from_amplitude(
            1,
            0,
            c32(f32::NAN, 0.0),
        );

    assert!(result.is_err());
}

#[test]
fn complex32_real_scaling_is_supported() {
    let mut state =
        SparseState::<Complex32>::basis_state(1, 0)
            .expect("Complex32 state");

    state
        .scale_real_f32(0.25)
        .expect("Complex32 scaling");

    assert_complex32_close(
        state.amplitude(0).unwrap(),
        c32(0.25, 0.0),
        1.0e-6,
    );
}

// =============================================================================
// State-operation invariants
// =============================================================================

#[test]
fn unitary_single_qubit_sequence_preserves_norm() {
    let mut state =
        SparseState::<Complex64>::zero(3)
            .expect("valid state");

    let h_value =
        1.0 / 2.0_f64.sqrt();

    let h = [
        c64(h_value, 0.0),
        c64(h_value, 0.0),
        c64(h_value, 0.0),
        c64(-h_value, 0.0),
    ];

    let x = [
        Complex64::ZERO,
        Complex64::ONE,
        Complex64::ONE,
        Complex64::ZERO,
    ];

    for qubit in 0..3 {
        state
            .apply_single_qubit_matrix(qubit, h)
            .expect("Hadamard");
    }

    state
        .apply_single_qubit_matrix(0, x)
        .expect("Pauli-X");

    state
        .apply_single_qubit_matrix(2, x)
        .expect("Pauli-X");

    assert_normalized_f64(&state);
}

#[test]
fn permutation_by_two_x_operations_returns_original_basis_state() {
    let original =
        SparseState::<Complex64>::basis_state(6, 37)
            .expect("valid state");

    let mut state =
        original.clone();

    let x = [
        Complex64::ZERO,
        Complex64::ONE,
        Complex64::ONE,
        Complex64::ZERO,
    ];

    state
        .apply_single_qubit_matrix(1, x)
        .expect("X");

    state
        .apply_single_qubit_matrix(1, x)
        .expect("inverse X");

    assert_eq!(state, original);
}

#[test]
fn applying_identity_does_not_change_state() {
    let original =
        SparseState::<Complex64>::from_entries(
            4,
            [
                (1, c64(0.25, 0.5)),
                (6, c64(-0.5, 0.25)),
            ],
        )
        .expect("valid state");

    let mut state =
        original.clone();

    let identity = [
        Complex64::ONE,
        Complex64::ZERO,
        Complex64::ZERO,
        Complex64::ONE,
    ];

    state
        .apply_single_qubit_matrix(3, identity)
        .expect("identity");

    assert_eq!(state, original);
}

#[test]
fn state_evolution_never_changes_qubit_count() {
    let mut state =
        SparseState::<Complex64>::zero(12)
            .expect("valid state");

    let initial_qubits =
        state.qubits();

    let h_value =
        1.0 / 2.0_f64.sqrt();

    let h = [
        c64(h_value, 0.0),
        c64(h_value, 0.0),
        c64(h_value, 0.0),
        c64(-h_value, 0.0),
    ];

    for qubit in 0..12 {
        state
            .apply_single_qubit_matrix(qubit, h)
            .expect("Hadamard");
    }

    assert_eq!(
        state.qubits(),
        initial_qubits
    );
}

// =============================================================================
// Provider-neutral integration contract
// =============================================================================

#[test]
fn sparse_state_has_no_vendor_specific_requirement() {
    // This test intentionally performs only representation-level operations.
    //
    // A CPU implementation, SIMD implementation, GPU implementation,
    // distributed implementation, or QPU-adapter implementation must not
    // change the mathematical result of these operations.
    let mut state =
        SparseState::<Complex64>::zero(4)
            .expect("valid state");

    let h_value =
        1.0 / 2.0_f64.sqrt();

    let h = [
        c64(h_value, 0.0),
        c64(h_value, 0.0),
        c64(h_value, 0.0),
        c64(-h_value, 0.0),
    ];

    state
        .apply_single_qubit_matrix(0, h)
        .expect("Hadamard");

    state
        .apply_single_qubit_matrix(2, h)
        .expect("Hadamard");

    assert_normalized_f64(&state);
    assert_eq!(state.qubits(), 4);
}

#[test]
fn sparse_state_does_not_assume_physical_qpu_state_visibility() {
    // A sparse representation is a simulator/storage representation.
    //
    // Real hardware integrations may instead use BackendState or another
    // representation. The sparse-state API itself remains hardware-neutral.
    let state =
        SparseState::<Complex64>::basis_state(2, 0)
            .expect("valid sparse representation");

    assert_eq!(state.support_len(), 1);
    assert_normalized_f64(&state);
}

// =============================================================================
// Regression tests for common failure modes
// =============================================================================

#[test]
fn no_operation_should_create_duplicate_support_entries() {
    let mut state =
        SparseState::<Complex64>::zero(3)
            .expect("valid state");

    state
        .add_amplitude(3, c64(0.25, 0.0))
        .expect("add");

    state
        .add_amplitude(3, c64(0.25, 0.0))
        .expect("add");

    state
        .add_amplitude(3, c64(0.5, 0.0))
        .expect("add");

    assert_eq!(state.support_len(), 2);
    assert_eq!(
        state.basis_indices().copied().collect::<Vec<_>>(),
        vec![0, 3]
    );
}

#[test]
fn cancellation_after_matrix_application_removes_zero_support() {
    let mut state =
        SparseState::<Complex64>::basis_state(1, 0)
            .expect("valid state");

    let x = [
        Complex64::ZERO,
        Complex64::ONE,
        Complex64::ONE,
        Complex64::ZERO,
    ];

    state
        .apply_single_qubit_matrix(0, x)
        .expect("X");

    state
        .apply_single_qubit_matrix(0, x)
        .expect("X");

    assert_eq!(state.support_len(), 1);
    assert_eq!(
        state.basis_indices().copied().collect::<Vec<_>>(),
        vec![0]
    );
}

#[test]
fn projection_preserves_dimension() {
    let mut state =
        SparseState::<Complex64>::from_entries(
            10,
            [
                (0, Complex64::ONE),
                (1, Complex64::ONE),
                (512, Complex64::ONE),
                (513, Complex64::ONE),
            ],
        )
        .expect("valid state");

    state
        .project_qubit(9, true)
        .expect("projection");

    assert_eq!(state.qubits(), 10);
}

#[test]
fn cloning_produces_independent_sparse_state() {
    let original =
        SparseState::<Complex64>::basis_state(4, 5)
            .expect("valid state");

    let mut clone =
        original.clone();

    clone
        .set_amplitude(7, Complex64::ONE)
        .expect("mutation");

    assert_eq!(original.support_len(), 1);
    assert_eq!(clone.support_len(), 2);

    assert!(
        !original
            .contains_basis(7)
            .expect("basis query")
    );

    assert!(
        clone
            .contains_basis(7)
            .expect("basis query")
    );
}

#[test]
fn state_equality_is_representation_value_equality() {
    let left =
        SparseState::<Complex64>::from_entries(
            3,
            [
                (0, c64(0.5, 0.0)),
                (1, c64(0.5, 0.0)),
            ],
        )
        .expect("valid state");

    let right =
        SparseState::<Complex64>::from_entries(
            3,
            [
                (1, c64(0.5, 0.0)),
                (0, c64(0.5, 0.0)),
            ],
        )
        .expect("valid state");

    assert_eq!(left, right);
}

// =============================================================================
// Error taxonomy smoke tests
// =============================================================================

#[test]
fn invalid_operations_return_errors_instead_of_panicking() {
    let mut state =
        SparseState::<Complex64>::zero(2)
            .expect("valid state");

    let single_matrix = [
        Complex64::ONE,
        Complex64::ZERO,
        Complex64::ZERO,
        Complex64::ONE,
    ];

    let two_matrix =
        [Complex64::ONE; TWO_QUBIT_MATRIX_ELEMENTS];

    assert!(
        state
            .apply_single_qubit_matrix(2, single_matrix)
            .is_err()
    );

    assert!(
        state
            .apply_two_qubit_matrix(0, 0, two_matrix)
            .is_err()
    );

    assert!(
        state
            .project_qubit(2, false)
            .is_err()
    );

    assert!(
        state
            .project_basis(4)
            .is_err()
    );

    assert!(
        state
            .probability(4)
            .is_err()
    );
}

// =============================================================================
// Final invariant sweep
// =============================================================================

#[test]
fn complete_sparse_state_contract_smoke_test() {
    let value =
        1.0 / 2.0_f64.sqrt();

    let mut state =
        SparseState::<Complex64>::from_entries(
            3,
            [
                (0, c64(value, 0.0)),
                (7, c64(value, 0.0)),
            ],
        )
        .expect("valid sparse state");

    // Construction.
    assert_eq!(state.qubits(), 3);
    assert_eq!(state.support_len(), 2);

    // Structural validity.
    state
        .validate()
        .expect("state must validate");

    // Numerical validity.
    assert_normalized_f64(&state);

    // Probability contract.
    assert_probability(&state, 0, 0.5);
    assert_probability(&state, 7, 0.5);

    // State-local transformation.
    let x = [
        Complex64::ZERO,
        Complex64::ONE,
        Complex64::ONE,
        Complex64::ZERO,
    ];

    state
        .apply_single_qubit_matrix(0, x)
        .expect("X");

    // A unitary operation must preserve normalization.
    assert_normalized_f64(&state);

    // Projection.
    state
        .project_qubit(0, true)
        .expect("projection");

    // The projected state remains structurally valid.
    state
        .validate()
        .expect("projected state must validate");

    // Dimension must remain unchanged.
    assert_eq!(state.qubits(), 3);

    // Storage remains sparse.
    assert!(state.support_len() <= 2);
}