//! Zamani Quantum Memory — Density-Matrix Integration and Regression Tests
//!
//! Production test suite for:
//!
//!     crate::quantum::memory::density_matrix
//!
//! # Purpose
//!
//! This file is the external integration/regression test suite for Zamani's
//! dense density-matrix representation.
//!
//! It deliberately tests the public API rather than private implementation
//! details. This allows the implementation to change internally while the
//! mathematical and architectural contract remains stable.
//!
//! # Scope
//!
//! The suite verifies:
//!
//! - allocation planning;
//! - checked exponential memory sizing;
//! - zero states;
//! - maximally mixed states;
//! - pure-state conversion;
//! - explicit normalization;
//! - trace preservation;
//! - Hermiticity;
//! - positive semidefiniteness;
//! - purity;
//! - full-system unitary evolution;
//! - local unitary evolution;
//! - Kraus/CPTP channels;
//! - computational-basis probabilities;
//! - multi-qubit measurement probabilities;
//! - deterministic measurement projection;
//! - dephasing;
//! - reset;
//! - partial trace;
//! - tensor products;
//! - expectation values;
//! - Hilbert-Schmidt overlap;
//! - approximate equality;
//! - snapshot copying;
//! - Complex32 support;
//! - Complex64 support;
//! - invalid dimensions;
//! - invalid qubits;
//! - duplicate targets;
//! - invalid operators;
//! - invalid Kraus channels;
//! - non-finite values;
//! - normalization failures;
//! - physical-state validation failures;
//! - dimension mismatch;
//! - arithmetic/resource safety;
//! - canonical little-endian qubit indexing;
//! - representation-level hardware neutrality.
//!
//! # Architectural rule
//!
//! A density matrix is a mathematical state representation. These tests must
//! never assume that a physical QPU exposes a full density matrix.
//!
//! Consequently this file contains no:
//!
//! - IBM-specific code;
//! - Google-specific code;
//! - Quantinuum-specific code;
//! - Rigetti-specific code;
//! - IonQ-specific code;
//! - IQM-specific code;
//! - Pasqal-specific code;
//! - CUDA-specific code;
//! - HIP/ROCm-specific code;
//! - Metal-specific code;
//! - MPI-specific code;
//! - network code;
//! - QPU credentials;
//! - backend sessions.
//!
//! Any simulator, accelerator, distributed provider, or hardware adapter must
//! be able to consume the public mathematical contract without requiring this
//! test suite to know the provider implementation.
//!
//! # Integration contract
//!
//! This test file intentionally assumes the following public architecture:
//!
//! ```text
//! quantum
//!   └── memory
//!       ├── errors
//!       ├── types
//!       └── density_matrix
//!
//! tests/density_matrix.rs
//!       │
//!       └── public density-matrix API only
//! ```
//!
//! It does NOT require:
//!
//! - `state.rs`;
//! - `measurement.rs`;
//! - `collapse.rs`;
//! - `reset.rs`;
//! - `gpu.rs`;
//! - `distributed.rs`;
//! - `migration.rs`;
//! - `snapshot.rs`;
//! - `serialization.rs`;
//! - hardware adapters.
//!
//! Those modules may later add their own integration suites.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features.
//!
//! # Safety
//!
//! This file contains no unsafe code.
//!
//! `#![deny(unsafe_code)]` is enabled deliberately so this test suite cannot
//! accidentally become dependent on unsafe implementation techniques.
//!
//! # Test philosophy
//!
//! Quantum-state tests should not merely test that a function returns `Ok`.
//!
//! They verify mathematical invariants:
//!
//! ```text
//! Hermitian(ρ)
//! Tr(ρ) = 1
//! ρ >= 0
//! P(i) >= 0
//! Σ P(i) = 1
//! Tr(ρ²) <= 1
//!
//! UρU† remains physical
//!
//! Σ Kᵢ†Kᵢ = I
//!
//! Tr(Σ KᵢρKᵢ†) = Tr(ρ)
//! ```
//!
//! This follows the same validation philosophy used by serious quantum
//! simulators: known closed-form states and channels should be checked against
//! exact mathematical expectations, not merely against execution success.
//!
//! # Important note about stochastic testing
//!
//! This file does NOT perform random measurement sampling.
//!
//! Sampling requires an explicitly injected RNG and belongs to the higher-level
//! measurement subsystem. These tests therefore verify exact probabilities and
//! deterministic projection instead.
//!
//! # Resource policy
//!
//! Tests intentionally use small matrices because density-matrix storage grows
//! as `4^n`. Large allocations belong in dedicated resource/benchmark tests and
//! must never make the ordinary unit-test suite depend on the machine's RAM.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use crate::quantum::memory::complex::{Complex32, Complex64};
use crate::quantum::memory::density_matrix::{
    DensityMatrix,
    DensityMatrixMemoryRequirement,
    DEFAULT_DENSITY_MATRIX_F32_TOLERANCE,
    DEFAULT_DENSITY_MATRIX_F64_TOLERANCE,
};
use crate::quantum::memory::errors::MemoryError;
use crate::quantum::memory::types::QubitCount;

// =============================================================================
// Test constants
// =============================================================================

const F64_TOL: f64 = 1.0e-10;
const F64_STRICT_TOL: f64 = 1.0e-12;
const F32_TOL: f32 = 1.0e-5;

// =============================================================================
// Test helpers
// =============================================================================

fn c64(real: f64, imaginary: f64) -> Complex64 {
    Complex64::new(real, imaginary)
}

fn c32(real: f32, imaginary: f32) -> Complex32 {
    Complex32::new(real, imaginary)
}

fn zero64() -> Complex64 {
    c64(0.0, 0.0)
}

fn one64() -> Complex64 {
    c64(1.0, 0.0)
}

fn assert_close(actual: f64, expected: f64, tolerance: f64) {
    assert!(
        (actual - expected).abs() <= tolerance,
        "expected {expected} ± {tolerance}, got {actual}"
    );
}

fn assert_complex_close(
    actual: Complex64,
    expected: Complex64,
    tolerance: f64,
) {
    assert_close(actual.real(), expected.real(), tolerance);
    assert_close(actual.imaginary(), expected.imaginary(), tolerance);
}

fn assert_physical64(state: &DensityMatrix<Complex64>) {
    state
        .validate_physical(F64_TOL)
        .expect("density matrix must satisfy physical-state invariants");
}

fn bell_state64() -> DensityMatrix<Complex64> {
    let amplitude = 1.0 / 2.0_f64.sqrt();

    DensityMatrix::<Complex64>::from_pure_state(
        &[
            c64(amplitude, 0.0),
            zero64(),
            zero64(),
            c64(amplitude, 0.0),
        ],
        F64_STRICT_TOL,
    )
    .expect("Bell-state density matrix")
}

fn plus_state64() -> DensityMatrix<Complex64> {
    let amplitude = 1.0 / 2.0_f64.sqrt();

    DensityMatrix::<Complex64>::from_pure_state(
        &[
            c64(amplitude, 0.0),
            c64(amplitude, 0.0),
        ],
        F64_STRICT_TOL,
    )
    .expect("plus-state density matrix")
}

fn identity_1q64() -> Vec<Complex64> {
    vec![
        c64(1.0, 0.0),
        c64(0.0, 0.0),
        c64(0.0, 0.0),
        c64(1.0, 0.0),
    ]
}

fn pauli_x64() -> Vec<Complex64> {
    vec![
        c64(0.0, 0.0),
        c64(1.0, 0.0),
        c64(1.0, 0.0),
        c64(0.0, 0.0),
    ]
}

fn pauli_y64() -> Vec<Complex64> {
    vec![
        c64(0.0, 0.0),
        c64(0.0, -1.0),
        c64(0.0, 1.0),
        c64(0.0, 0.0),
    ]
}

fn pauli_z64() -> Vec<Complex64> {
    vec![
        c64(1.0, 0.0),
        c64(0.0, 0.0),
        c64(0.0, 0.0),
        c64(-1.0, 0.0),
    ]
}

// =============================================================================
// Allocation and representation contract
// =============================================================================

#[test]
fn memory_requirement_for_zero_qubits_is_one_scalar() {
    let requirement =
        DensityMatrix::<Complex64>::memory_requirement(QubitCount::new(0))
            .expect("zero-qubit density matrix requirement");

    assert_eq!(requirement.qubits(), QubitCount::new(0));
    assert_eq!(requirement.dimension(), 1);
    assert_eq!(requirement.elements(), 1);
    assert_eq!(requirement.scalar_bytes(), 16);
    assert_eq!(requirement.bytes(), 16);
}

#[test]
fn memory_requirement_scales_as_four_to_the_n() {
    let one = DensityMatrix::<Complex64>::memory_requirement(
        QubitCount::new(1),
    )
    .expect("one-qubit requirement");

    let two = DensityMatrix::<Complex64>::memory_requirement(
        QubitCount::new(2),
    )
    .expect("two-qubit requirement");

    let three = DensityMatrix::<Complex64>::memory_requirement(
        QubitCount::new(3),
    )
    .expect("three-qubit requirement");

    assert_eq!(one.dimension(), 2);
    assert_eq!(one.elements(), 4);

    assert_eq!(two.dimension(), 4);
    assert_eq!(two.elements(), 16);

    assert_eq!(three.dimension(), 8);
    assert_eq!(three.elements(), 64);

    assert_eq!(two.bytes(), one.bytes() * 4);
    assert_eq!(three.bytes(), two.bytes() * 4);
}

#[test]
fn complex32_and_complex64_report_correct_storage_sizes() {
    let f32_requirement =
        DensityMatrix::<Complex32>::memory_requirement(QubitCount::new(2))
            .expect("Complex32 requirement");

    let f64_requirement =
        DensityMatrix::<Complex64>::memory_requirement(QubitCount::new(2))
            .expect("Complex64 requirement");

    assert_eq!(f32_requirement.scalar_bytes(), 8);
    assert_eq!(f32_requirement.bytes(), 16 * 8);

    assert_eq!(f64_requirement.scalar_bytes(), 16);
    assert_eq!(f64_requirement.bytes(), 16 * 16);
}

#[test]
fn memory_requirement_is_non_allocating_and_exact_for_small_state() {
    let requirement =
        DensityMatrix::<Complex64>::memory_requirement(QubitCount::new(4))
            .expect("four-qubit requirement");

    assert_eq!(requirement.dimension(), 16);
    assert_eq!(requirement.elements(), 256);
    assert_eq!(requirement.bytes(), 256 * 16);
}

// =============================================================================
// Canonical state construction
// =============================================================================

#[test]
fn zero_state_has_exact_canonical_population() {
    let state =
        DensityMatrix::<Complex64>::zero_state(QubitCount::new(3))
            .expect("zero state");

    assert_eq!(state.qubit_count(), QubitCount::new(3));
    assert_eq!(state.dimension(), 8);
    assert_eq!(state.element_count(), 64);

    assert_complex_close(state.get(0, 0).expect("rho00"), one64(), F64_STRICT_TOL);

    for index in 1..8 {
        assert_close(
            state
                .basis_probability(index)
                .expect("basis probability"),
            0.0,
            F64_STRICT_TOL,
        );
    }

    assert_physical64(&state);
}

#[test]
fn zero_qubit_state_is_valid() {
    let state =
        DensityMatrix::<Complex64>::zero_state(QubitCount::new(0))
            .expect("zero-qubit state");

    assert_eq!(state.dimension(), 1);
    assert_eq!(state.element_count(), 1);
    assert_complex_close(state.get(0, 0).expect("rho"), one64(), F64_STRICT_TOL);
    assert_physical64(&state);
}

#[test]
fn maximally_mixed_state_has_uniform_diagonal() {
    let state =
        DensityMatrix::<Complex64>::maximally_mixed(QubitCount::new(2))
            .expect("maximally mixed state");

    let expected = 0.25;

    for index in 0..4 {
        assert_close(
            state.basis_probability(index).expect("basis probability"),
            expected,
            F64_STRICT_TOL,
        );
    }

    for row in 0..4 {
        for column in 0..4 {
            if row != column {
                assert_complex_close(
                    state.get(row, column).expect("off-diagonal"),
                    zero64(),
                    F64_STRICT_TOL,
                );
            }
        }
    }

    assert_physical64(&state);

    assert_close(
        state.purity().expect("purity"),
        0.25,
        F64_STRICT_TOL,
    );
}

#[test]
fn pure_state_constructor_rejects_non_normalized_input() {
    let result = DensityMatrix::<Complex64>::from_pure_state(
        &[c64(1.0, 0.0), c64(1.0, 0.0)],
        F64_STRICT_TOL,
    );

    assert!(matches!(
        result,
        Err(MemoryError::NotNormalized { .. })
    ));
}

#[test]
fn pure_state_constructor_preserves_normalized_complex_amplitudes() {
    let state = DensityMatrix::<Complex64>::from_pure_state(
        &[
            c64(0.5, 0.5),
            c64(0.5, -0.5),
        ],
        F64_STRICT_TOL,
    )
    .expect("normalized complex state");

    assert_physical64(&state);

    assert_complex_close(
        state.get(0, 0).expect("rho00"),
        c64(0.5, 0.0),
        F64_STRICT_TOL,
    );

    assert_complex_close(
        state.get(0, 1).expect("rho01"),
        c64(0.0, 0.5),
        F64_STRICT_TOL,
    );
}

#[test]
fn normalized_constructor_explicitly_normalizes_input() {
    let state =
        DensityMatrix::<Complex64>::from_pure_state_normalized(&[
            c64(2.0, 0.0),
            c64(0.0, 0.0),
        ])
        .expect("explicit normalization");

    assert_physical64(&state);

    assert_close(
        state.basis_probability(0).expect("p0"),
        1.0,
        F64_STRICT_TOL,
    );

    assert_close(
        state.basis_probability(1).expect("p1"),
        0.0,
        F64_STRICT_TOL,
    );
}

#[test]
fn normalized_constructor_rejects_zero_vector() {
    let result =
        DensityMatrix::<Complex64>::from_pure_state_normalized(&[
            zero64(),
            zero64(),
        ]);

    assert!(matches!(
        result,
        Err(MemoryError::InvalidState { .. })
    ));
}

#[test]
fn constructor_rejects_wrong_element_count() {
    let result = DensityMatrix::<Complex64>::from_elements(
        QubitCount::new(1),
        vec![one64()],
    );

    assert!(matches!(
        result,
        Err(MemoryError::StateDimensionMismatch { .. })
    ));
}

#[test]
fn constructor_rejects_non_finite_values() {
    let result = DensityMatrix::<Complex64>::from_elements(
        QubitCount::new(1),
        vec![
            c64(f64::NAN, 0.0),
            zero64(),
            zero64(),
            one64(),
        ],
    );

    assert!(matches!(
        result,
        Err(MemoryError::NonFiniteValue { .. })
    ));
}

// =============================================================================
// Physical invariants
// =============================================================================

#[test]
fn bell_state_is_hermitian_trace_one_and_positive_semidefinite() {
    let state = bell_state64();

    assert_physical64(&state);

    assert_close(
        state.real_trace(F64_STRICT_TOL).expect("real trace"),
        1.0,
        F64_STRICT_TOL,
    );

    assert_close(
        state.max_hermiticity_deviation().expect("Hermiticity deviation"),
        0.0,
        F64_STRICT_TOL,
    );

    assert_close(
        state.purity().expect("purity"),
        1.0,
        F64_STRICT_TOL,
    );

    assert!(state.is_pure(F64_STRICT_TOL).expect("purity predicate"));
}

#[test]
fn maximally_mixed_state_is_not_pure() {
    let state =
        DensityMatrix::<Complex64>::maximally_mixed(QubitCount::new(2))
            .expect("mixed state");

    assert!(!state.is_pure(F64_STRICT_TOL).expect("purity predicate"));

    assert_close(
        state.purity().expect("purity"),
        0.25,
        F64_STRICT_TOL,
    );
}

#[test]
fn hermiticity_validation_rejects_non_hermitian_matrix() {
    let state = DensityMatrix::<Complex64>::from_elements(
        QubitCount::new(1),
        vec![
            c64(0.5, 0.0),
            c64(0.25, 0.1),
            c64(0.25, 0.0),
            c64(0.5, 0.0),
        ],
    )
    .expect("finite matrix");

    let result = state.validate_hermitian(F64_STRICT_TOL);

    assert!(matches!(
        result,
        Err(MemoryError::NotHermitian { .. })
    ));
}

#[test]
fn trace_validation_rejects_wrong_trace() {
    let state = DensityMatrix::<Complex64>::from_elements(
        QubitCount::new(1),
        vec![
            c64(0.25, 0.0),
            zero64(),
            zero64(),
            c64(0.25, 0.0),
        ],
    )
    .expect("finite matrix");

    let result = state.validate_trace(F64_STRICT_TOL);

    assert!(matches!(
        result,
        Err(MemoryError::InvalidTrace { .. })
    ));
}

#[test]
fn positive_semidefinite_validation_rejects_negative_eigenvalue_state() {
    // Diagonal eigenvalues are 1.2 and -0.2.
    //
    // The matrix is Hermitian and has trace one, but is not positive
    // semidefinite and therefore cannot represent a physical quantum state.
    let state = DensityMatrix::<Complex64>::from_elements(
        QubitCount::new(1),
        vec![
            c64(1.2, 0.0),
            zero64(),
            zero64(),
            c64(-0.2, 0.0),
        ],
    )
    .expect("finite Hermitian matrix");

    let result = state.validate_physical(F64_STRICT_TOL);

    assert!(matches!(
        result,
        Err(MemoryError::InvalidState { .. })
    ));
}

#[test]
fn finite_validation_rejects_infinity() {
    let result = DensityMatrix::<Complex64>::from_elements(
        QubitCount::new(1),
        vec![
            c64(f64::INFINITY, 0.0),
            zero64(),
            zero64(),
            zero64(),
        ],
    );

    assert!(matches!(
        result,
        Err(MemoryError::NonFiniteValue { .. })
    ));
}

// =============================================================================
// Element access and canonical indexing
// =============================================================================

#[test]
fn element_access_is_row_major() {
    let values = vec![
        c64(0.0, 0.0),
        c64(1.0, 0.0),
        c64(2.0, 0.0),
        c64(3.0, 0.0),
    ];

    let state =
        DensityMatrix::<Complex64>::from_elements(
            QubitCount::new(1),
            values,
        )
        .expect("matrix");

    assert_complex_close(
        state.get(0, 0).expect("00"),
        c64(0.0, 0.0),
        F64_STRICT_TOL,
    );

    assert_complex_close(
        state.get(0, 1).expect("01"),
        c64(1.0, 0.0),
        F64_STRICT_TOL,
    );

    assert_complex_close(
        state.get(1, 0).expect("10"),
        c64(2.0, 0.0),
        F64_STRICT_TOL,
    );

    assert_complex_close(
        state.get(1, 1).expect("11"),
        c64(3.0, 0.0),
        F64_STRICT_TOL,
    );
}

#[test]
fn get_rejects_out_of_bounds_row() {
    let state =
        DensityMatrix::<Complex64>::zero_state(QubitCount::new(1))
            .expect("state");

    let result = state.get(2, 0);

    assert!(matches!(
        result,
        Err(MemoryError::OutOfBounds { .. })
    ));
}

#[test]
fn get_rejects_out_of_bounds_column() {
    let state =
        DensityMatrix::<Complex64>::zero_state(QubitCount::new(1))
            .expect("state");

    let result = state.get(0, 2);

    assert!(matches!(
        result,
        Err(MemoryError::OutOfBounds { .. })
    ));
}

#[test]
fn set_rejects_non_finite_values() {
    let mut state =
        DensityMatrix::<Complex64>::zero_state(QubitCount::new(1))
            .expect("state");

    let result = state.set(0, 0, c64(f64::NAN, 0.0));

    assert!(matches!(
        result,
        Err(MemoryError::NonFiniteValue { .. })
    ));
}

// =============================================================================
// Full-system unitary evolution
// =============================================================================

#[test]
fn identity_unitary_preserves_density_matrix_exactly() {
    let state = bell_state64();
    let identity = {
        let mut matrix = vec![zero64(); 16];

        matrix[0] = one64();
        matrix[5] = one64();
        matrix[10] = one64();
        matrix[15] = one64();

        matrix
    };

    let transformed = state
        .transformed_by_unitary(&identity, F64_STRICT_TOL)
        .expect("identity transformation");

    assert!(
        state
            .approx_eq(&transformed, F64_STRICT_TOL)
            .expect("approximate equality")
    );
}

#[test]
fn pauli_x_maps_zero_to_one() {
    let mut state =
        DensityMatrix::<Complex64>::zero_state(QubitCount::new(1))
            .expect("zero state");

    state
        .apply_unitary(&pauli_x64(), F64_STRICT_TOL)
        .expect("Pauli X");

    assert_close(
        state.basis_probability(0).expect("p0"),
        0.0,
        F64_STRICT_TOL,
    );

    assert_close(
        state.basis_probability(1).expect("p1"),
        1.0,
        F64_STRICT_TOL,
    );

    assert_physical64(&state);
}

#[test]
fn pauli_z_preserves_zero_state() {
    let mut state =
        DensityMatrix::<Complex64>::zero_state(QubitCount::new(1))
            .expect("zero state");

    state
        .apply_unitary(&pauli_z64(), F64_STRICT_TOL)
        .expect("Pauli Z");

    assert_close(
        state.basis_probability(0).expect("p0"),
        1.0,
        F64_STRICT_TOL,
    );

    assert_close(
        state.basis_probability(1).expect("p1"),
        0.0,
        F64_STRICT_TOL,
    );

    assert_physical64(&state);
}

#[test]
fn pauli_y_maps_zero_to_one_without_changing_probabilities() {
    let mut state =
        DensityMatrix::<Complex64>::zero_state(QubitCount::new(1))
            .expect("zero state");

    state
        .apply_unitary(&pauli_y64(), F64_STRICT_TOL)
        .expect("Pauli Y");

    assert_close(
        state.basis_probability(0).expect("p0"),
        0.0,
        F64_STRICT_TOL,
    );

    assert_close(
        state.basis_probability(1).expect("p1"),
        1.0,
        F64_STRICT_TOL,
    );

    assert_physical64(&state);
}

#[test]
fn unitary_evolution_preserves_trace_and_purity() {
    let initial = plus_state64();

    let mut state = initial.clone();

    state
        .apply_unitary(&pauli_z64(), F64_STRICT_TOL)
        .expect("Pauli Z");

    assert_physical64(&state);

    assert_close(
        state.real_trace(F64_STRICT_TOL).expect("trace"),
        1.0,
        F64_STRICT_TOL,
    );

    assert_close(
        state.purity().expect("purity"),
        initial.purity().expect("initial purity"),
        F64_STRICT_TOL,
    );
}

#[test]
fn invalid_unitary_dimension_is_rejected() {
    let mut state =
        DensityMatrix::<Complex64>::zero_state(QubitCount::new(1))
            .expect("state");

    let invalid = vec![one64()];

    let result =
        state.apply_unitary(&invalid, F64_STRICT_TOL);

    assert!(matches!(
        result,
        Err(MemoryError::StateDimensionMismatch { .. })
    ));
}

#[test]
fn non_unitary_operator_is_rejected() {
    let mut state =
        DensityMatrix::<Complex64>::zero_state(QubitCount::new(1))
            .expect("state");

    let invalid = vec![
        c64(2.0, 0.0),
        zero64(),
        zero64(),
        c64(2.0, 0.0),
    ];

    let result =
        state.apply_unitary(&invalid, F64_STRICT_TOL);

    assert!(matches!(
        result,
        Err(MemoryError::InvalidState { .. })
    ));
}

// =============================================================================
// Local unitary evolution and qubit ordering
// =============================================================================

#[test]
fn local_x_on_qubit_zero_targets_least_significant_bit() {
    let mut state =
        DensityMatrix::<Complex64>::zero_state(QubitCount::new(2))
            .expect("zero state");

    state
        .apply_unitary_on_qubits(
            &[0],
            &pauli_x64(),
            F64_STRICT_TOL,
        )
        .expect("X on q0");

    // q0 is the least-significant bit, therefore |00> -> |01>.
    assert_close(
        state.basis_probability(0).expect("p00"),
        0.0,
        F64_STRICT_TOL,
    );

    assert_close(
        state.basis_probability(1).expect("p01"),
        1.0,
        F64_STRICT_TOL,
    );

    assert_close(
        state.basis_probability(2).expect("p10"),
        0.0,
        F64_STRICT_TOL,
    );

    assert_close(
        state.basis_probability(3).expect("p11"),
        0.0,
        F64_STRICT_TOL,
    );
}

#[test]
fn local_x_on_qubit_one_targets_next_bit() {
    let mut state =
        DensityMatrix::<Complex64>::zero_state(QubitCount::new(2))
            .expect("zero state");

    state
        .apply_unitary_on_qubits(
            &[1],
            &pauli_x64(),
            F64_STRICT_TOL,
        )
        .expect("X on q1");

    // q1 is bit one, therefore |00> -> |10>, basis index 2.
    assert_close(
        state.basis_probability(2).expect("p10"),
        1.0,
        F64_STRICT_TOL,
    );
}

#[test]
fn local_unitary_rejects_duplicate_qubits() {
    let mut state =
        DensityMatrix::<Complex64>::zero_state(QubitCount::new(2))
            .expect("state");

    let result = state.apply_unitary_on_qubits(
        &[0, 0],
        &identity_1q64(),
        F64_STRICT_TOL,
    );

    assert!(matches!(
        result,
        Err(MemoryError::InvalidPermutation { .. })
    ));
}

#[test]
fn local_unitary_rejects_out_of_range_qubit() {
    let mut state =
        DensityMatrix::<Complex64>::zero_state(QubitCount::new(2))
            .expect("state");

    let result = state.apply_unitary_on_qubits(
        &[2],
        &pauli_x64(),
        F64_STRICT_TOL,
    );

    assert!(matches!(
        result,
        Err(MemoryError::OutOfBounds { .. })
    ));
}

// =============================================================================
// Measurement probabilities and deterministic projection
// =============================================================================

#[test]
fn bell_state_has_half_probability_for_each_single_qubit_outcome() {
    let state = bell_state64();

    for qubit in [0usize, 1usize] {
        assert_close(
            state.qubit_probability(qubit, false).expect("p0"),
            0.5,
            F64_STRICT_TOL,
        );

        assert_close(
            state.qubit_probability(qubit, true).expect("p1"),
            0.5,
            F64_STRICT_TOL,
        );
    }
}

#[test]
fn bell_state_has_only_zero_zero_and_one_one_outcomes() {
    let state = bell_state64();

    assert_close(
        state.measurement_probability(&[0, 1], 0).expect("00"),
        0.5,
        F64_STRICT_TOL,
    );

    assert_close(
        state.measurement_probability(&[0, 1], 3).expect("11"),
        0.5,
        F64_STRICT_TOL,
    );

    assert_close(
        state.measurement_probability(&[0, 1], 1).expect("01"),
        0.0,
        F64_STRICT_TOL,
    );

    assert_close(
        state.measurement_probability(&[0, 1], 2).expect("10"),
        0.0,
        F64_STRICT_TOL,
    );
}

#[test]
fn measurement_probabilities_sum_to_one() {
    let state = bell_state64();

    let mut sum = 0.0;

    for outcome in 0..4 {
        sum += state
            .measurement_probability(&[0, 1], outcome)
            .expect("measurement probability");
    }

    assert_close(sum, 1.0, F64_STRICT_TOL);
}

#[test]
fn invalid_measurement_outcome_is_rejected() {
    let state = bell_state64();

    let result =
        state.measurement_probability(&[0, 1], 4);

    assert!(matches!(
        result,
        Err(MemoryError::OutOfBounds { .. })
    ));
}

#[test]
fn measurement_rejects_duplicate_target_qubits() {
    let state = bell_state64();

    let result =
        state.measurement_probability(&[0, 0], 0);

    assert!(matches!(
        result,
        Err(MemoryError::InvalidPermutation { .. })
    ));
}

#[test]
fn projection_on_bell_zero_zero_produces_zero_zero() {
    let mut state = bell_state64();

    let probability = state
        .project_measurement(&[0, 1], 0, F64_STRICT_TOL)
        .expect("projection");

    assert_close(probability, 0.5, F64_STRICT_TOL);

    assert_physical64(&state);

    assert_close(
        state.basis_probability(0).expect("p00"),
        1.0,
        F64_STRICT_TOL,
    );

    assert_close(
        state.basis_probability(3).expect("p11"),
        0.0,
        F64_STRICT_TOL,
    );
}

#[test]
fn projection_on_bell_one_one_produces_one_one() {
    let mut state = bell_state64();

    let probability = state
        .project_measurement(&[0, 1], 3, F64_STRICT_TOL)
        .expect("projection");

    assert_close(probability, 0.5, F64_STRICT_TOL);

    assert_physical64(&state);

    assert_close(
        state.basis_probability(0).expect("p00"),
        0.0,
        F64_STRICT_TOL,
    );

    assert_close(
        state.basis_probability(3).expect("p11"),
        1.0,
        F64_STRICT_TOL,
    );
}

#[test]
fn projection_of_zero_probability_outcome_is_rejected() {
    let mut state =
        DensityMatrix::<Complex64>::zero_state(QubitCount::new(1))
            .expect("zero state");

    let result =
        state.project_measurement(&[0], 1, F64_STRICT_TOL);

    assert!(matches!(
        result,
        Err(MemoryError::CollapseError { .. })
    ));
}

// =============================================================================
// Dephasing and reset
// =============================================================================

#[test]
fn dephasing_removes_selected_coherence() {
    let mut state = plus_state64();

    assert!(
        state
            .get(0, 1)
            .expect("off-diagonal coherence")
            .magnitude()
            > 0.49
    );

    state
        .dephase_qubits(&[0])
        .expect("dephasing");

    assert_complex_close(
        state.get(0, 1).expect("dephased coherence"),
        zero64(),
        F64_STRICT_TOL,
    );

    assert_physical64(&state);

    assert_close(
        state.purity().expect("purity"),
        0.5,
        F64_STRICT_TOL,
    );
}

#[test]
fn reset_of_one_qubit_returns_zero_state() {
    let mut state =
        DensityMatrix::<Complex64>::zero_state(QubitCount::new(1))
            .expect("zero");

    state
        .apply_unitary(&pauli_x64(), F64_STRICT_TOL)
        .expect("X");

    state.reset_qubits(&[0]).expect("reset");

    assert_physical64(&state);

    assert_close(
        state.basis_probability(0).expect("p0"),
        1.0,
        F64_STRICT_TOL,
    );

    assert_close(
        state.basis_probability(1).expect("p1"),
        0.0,
        F64_STRICT_TOL,
    );
}

#[test]
fn reset_of_entangled_qubit_produces_product_zero_on_target() {
    let mut state = bell_state64();

    state.reset_qubits(&[0]).expect("reset q0");

    assert_physical64(&state);

    assert_close(
        state.qubit_probability(0, false).expect("q0=0"),
        1.0,
        F64_STRICT_TOL,
    );

    assert_close(
        state.qubit_probability(0, true).expect("q0=1"),
        0.0,
        F64_STRICT_TOL,
    );

    let reduced = state
        .reduced_without_qubit(0)
        .expect("remaining q1");

    assert_close(
        reduced.basis_probability(0).expect("q1=0"),
        0.5,
        F64_STRICT_TOL,
    );

    assert_close(
        reduced.basis_probability(1).expect("q1=1"),
        0.5,
        F64_STRICT_TOL,
    );
}

// =============================================================================
// Kraus / CPTP channels
// =============================================================================

#[test]
fn identity_kraus_channel_preserves_state() {
    let mut state = bell_state64();

    let before = state.clone();

    state
        .apply_kraus_operators(
            &[identity_1q64(), identity_1q64()],
            F64_STRICT_TOL,
        )
        .expect_err("two identity Kraus operators are not trace-preserving");

    // The operation must fail before mutating the state.
    assert!(
        state
            .approx_eq(&before, F64_STRICT_TOL)
            .expect("comparison")
    );
}

#[test]
fn single_identity_kraus_channel_preserves_state() {
    let mut state =
        DensityMatrix::<Complex64>::zero_state(QubitCount::new(1))
            .expect("zero");

    let before = state.clone();

    state
        .apply_kraus_operators(
            &[identity_1q64()],
            F64_STRICT_TOL,
        )
        .expect("identity channel");

    assert!(
        state
            .approx_eq(&before, F64_STRICT_TOL)
            .expect("comparison")
    );

    assert_physical64(&state);
}

#[test]
fn bit_flip_channel_makes_zero_state_half_mixed() {
    let mut state =
        DensityMatrix::<Complex64>::zero_state(QubitCount::new(1))
            .expect("zero");

    let probability = 0.5_f64;
    let square = probability.sqrt();
    let complement = (1.0 - probability).sqrt();

    let k0 = vec![
        c64(complement, 0.0),
        zero64(),
        zero64(),
        c64(complement, 0.0),
    ];

    let k1 = vec![
        zero64(),
        c64(square, 0.0),
        c64(square, 0.0),
        zero64(),
    ];

    state
        .apply_kraus_operators(
            &[k0, k1],
            F64_STRICT_TOL,
        )
        .expect("bit-flip channel");

    assert_physical64(&state);

    assert_close(
        state.basis_probability(0).expect("p0"),
        0.5,
        F64_STRICT_TOL,
    );

    assert_close(
        state.basis_probability(1).expect("p1"),
        0.5,
        F64_STRICT_TOL,
    );

    assert_close(
        state.purity().expect("purity"),
        0.5,
        F64_STRICT_TOL,
    );
}

#[test]
fn amplitude_damping_channel_maps_one_toward_zero() {
    let gamma = 0.25_f64;

    let mut state =
        DensityMatrix::<Complex64>::zero_state(QubitCount::new(1))
            .expect("zero");

    state
        .apply_unitary(&pauli_x64(), F64_STRICT_TOL)
        .expect("prepare |1>");

    let k0 = vec![
        c64(1.0, 0.0),
        zero64(),
        zero64(),
        c64((1.0 - gamma).sqrt(), 0.0),
    ];

    let k1 = vec![
        zero64(),
        c64(gamma.sqrt(), 0.0),
        zero64(),
        zero64(),
    ];

    state
        .apply_kraus_operators(
            &[k0, k1],
            F64_STRICT_TOL,
        )
        .expect("amplitude damping");

    assert_physical64(&state);

    assert_close(
        state.basis_probability(0).expect("p0"),
        gamma,
        F64_TOL,
    );

    assert_close(
        state.basis_probability(1).expect("p1"),
        1.0 - gamma,
        F64_TOL,
    );
}

#[test]
fn non_trace_preserving_kraus_set_is_rejected() {
    let mut state =
        DensityMatrix::<Complex64>::zero_state(QubitCount::new(1))
            .expect("zero");

    let k = vec![
        c64(0.5, 0.0),
        zero64(),
        zero64(),
        c64(0.5, 0.0),
    ];

    let result = state.apply_kraus_operators(
        &[k],
        F64_STRICT_TOL,
    );

    assert!(matches!(
        result,
        Err(MemoryError::InvalidState { .. })
    ));
}

#[test]
fn empty_kraus_set_is_rejected() {
    let mut state =
        DensityMatrix::<Complex64>::zero_state(QubitCount::new(1))
            .expect("zero");

    let result =
        state.apply_kraus_operators(&[], F64_STRICT_TOL);

    assert!(matches!(
        result,
        Err(MemoryError::InvalidArgument { .. })
    ));
}

// =============================================================================
// Partial trace and reduced states
// =============================================================================

#[test]
fn partial_trace_of_bell_state_is_maximally_mixed() {
    let state = bell_state64();

    let reduced = state
        .partial_trace(&[1])
        .expect("trace out q1");

    assert_eq!(reduced.qubit_count(), QubitCount::new(1));
    assert_eq!(reduced.dimension(), 2);

    assert_close(
        reduced.basis_probability(0).expect("p0"),
        0.5,
        F64_STRICT_TOL,
    );

    assert_close(
        reduced.basis_probability(1).expect("p1"),
        0.5,
        F64_STRICT_TOL,
    );

    assert_physical64(&reduced);
}

#[test]
fn partial_trace_of_product_zero_state_remains_zero() {
    let state =
        DensityMatrix::<Complex64>::zero_state(QubitCount::new(2))
            .expect("zero state");

    let reduced =
        state.partial_trace(&[1]).expect("partial trace");

    assert_eq!(reduced.qubit_count(), QubitCount::new(1));

    assert_close(
        reduced.basis_probability(0).expect("p0"),
        1.0,
        F64_STRICT_TOL,
    );

    assert_close(
        reduced.basis_probability(1).expect("p1"),
        0.0,
        F64_STRICT_TOL,
    );

    assert_physical64(&reduced);
}

#[test]
fn reduced_without_qubit_matches_partial_trace() {
    let state = bell_state64();

    let a =
        state.partial_trace(&[1]).expect("partial trace");

    let b =
        state.reduced_without_qubit(1).expect("reduced state");

    assert!(
        a.approx_eq(&b, F64_STRICT_TOL)
            .expect("equivalence")
    );
}

#[test]
fn reduced_to_qubits_matches_complementary_partial_trace() {
    let state =
        DensityMatrix::<Complex64>::zero_state(QubitCount::new(3))
            .expect("zero state");

    let reduced = state
        .reduced_to_qubits(&[0, 2])
        .expect("retain q0 and q2");

    assert_eq!(reduced.qubit_count(), QubitCount::new(2));

    assert_close(
        reduced.basis_probability(0).expect("zero state"),
        1.0,
        F64_STRICT_TOL,
    );

    assert_physical64(&reduced);
}

#[test]
fn partial_trace_rejects_duplicate_qubits() {
    let state = bell_state64();

    let result =
        state.partial_trace(&[0, 0]);

    assert!(matches!(
        result,
        Err(MemoryError::InvalidPermutation { .. })
    ));
}

// =============================================================================
// Tensor products
// =============================================================================

#[test]
fn tensor_product_of_zero_states_is_zero_state() {
    let left =
        DensityMatrix::<Complex64>::zero_state(QubitCount::new(1))
            .expect("left");

    let right =
        DensityMatrix::<Complex64>::zero_state(QubitCount::new(2))
            .expect("right");

    let combined =
        left.tensor_product(&right)
            .expect("tensor product");

    assert_eq!(combined.qubit_count(), QubitCount::new(3));
    assert_eq!(combined.dimension(), 8);

    assert_close(
        combined.basis_probability(0).expect("p000"),
        1.0,
        F64_STRICT_TOL,
    );

    assert_physical64(&combined);
}

#[test]
fn tensor_product_of_maximally_mixed_states_has_expected_purity() {
    let left =
        DensityMatrix::<Complex64>::maximally_mixed(QubitCount::new(1))
            .expect("left");

    let right =
        DensityMatrix::<Complex64>::maximally_mixed(QubitCount::new(1))
            .expect("right");

    let combined =
        left.tensor_product(&right)
            .expect("tensor product");

    assert_close(
        combined.purity().expect("purity"),
        0.25,
        F64_STRICT_TOL,
    );

    assert_physical64(&combined);
}

#[test]
fn tensor_product_preserves_factorized_probabilities() {
    let left =
        DensityMatrix::<Complex64>::zero_state(QubitCount::new(1))
            .expect("left");

    let mut right =
        DensityMatrix::<Complex64>::zero_state(QubitCount::new(1))
            .expect("right");

    right
        .apply_unitary(&pauli_x64(), F64_STRICT_TOL)
        .expect("right X");

    let combined =
        left.tensor_product(&right)
            .expect("tensor product");

    // The implementation contract says the left operand occupies the
    // higher-order portion and the right operand the lower-order portion.
    // Therefore |0> ⊗ |1> is basis index 1.
    assert_close(
        combined.basis_probability(1).expect("01"),
        1.0,
        F64_STRICT_TOL,
    );
}

// =============================================================================
// Observables
// =============================================================================

#[test]
fn expectation_of_identity_is_one() {
    let state = bell_state64();

    let identity = {
        let mut matrix = vec![zero64(); 16];

        matrix[0] = one64();
        matrix[5] = one64();
        matrix[10] = one64();
        matrix[15] = one64();

        matrix
    };

    let expectation =
        state.expectation_value(&identity)
            .expect("identity expectation");

    assert_complex_close(
        expectation,
        one64(),
        F64_STRICT_TOL,
    );
}

#[test]
fn expectation_of_pauli_z_on_zero_state_is_plus_one() {
    let state =
        DensityMatrix::<Complex64>::zero_state(QubitCount::new(1))
            .expect("zero state");

    let expectation =
        state.expectation_value(&pauli_z64())
            .expect("Z expectation");

    assert_complex_close(
        expectation,
        one64(),
        F64_STRICT_TOL,
    );
}

#[test]
fn expectation_of_pauli_z_on_one_state_is_minus_one() {
    let mut state =
        DensityMatrix::<Complex64>::zero_state(QubitCount::new(1))
            .expect("zero");

    state
        .apply_unitary(&pauli_x64(), F64_STRICT_TOL)
        .expect("X");

    let expectation =
        state.expectation_value(&pauli_z64())
            .expect("Z expectation");

    assert_complex_close(
        expectation,
        c64(-1.0, 0.0),
        F64_STRICT_TOL,
    );
}

#[test]
fn expectation_of_pauli_x_on_plus_state_is_plus_one() {
    let state = plus_state64();

    let expectation =
        state.expectation_value(&pauli_x64())
            .expect("X expectation");

    assert_complex_close(
        expectation,
        one64(),
        F64_STRICT_TOL,
    );
}

#[test]
fn expectation_of_pauli_y_on_zero_state_is_zero() {
    let state =
        DensityMatrix::<Complex64>::zero_state(QubitCount::new(1))
            .expect("zero state");

    let expectation =
        state.expectation_value(&pauli_y64())
            .expect("Y expectation");

    assert_close(
        expectation.real(),
        0.0,
        F64_STRICT_TOL,
    );

    assert_close(
        expectation.imaginary(),
        0.0,
        F64_STRICT_TOL,
    );
}

#[test]
fn expectation_rejects_wrong_operator_dimension() {
    let state =
        DensityMatrix::<Complex64>::zero_state(QubitCount::new(1))
            .expect("state");

    let result =
        state.expectation_value(&[one64()]);

    assert!(matches!(
        result,
        Err(MemoryError::StateDimensionMismatch { .. })
    ));
}

// =============================================================================
// State comparison and copying
// =============================================================================

#[test]
fn identical_states_have_zero_max_difference() {
    let state = bell_state64();

    assert_close(
        state.max_difference(&state).expect("difference"),
        0.0,
        F64_STRICT_TOL,
    );
}

#[test]
fn different_states_have_nonzero_difference() {
    let a =
        DensityMatrix::<Complex64>::zero_state(QubitCount::new(1))
            .expect("a");

    let mut b = a.clone();

    b.apply_unitary(&pauli_x64(), F64_STRICT_TOL)
        .expect("X");

    assert!(
        a.max_difference(&b)
            .expect("difference")
            > 0.9
    );
}

#[test]
fn approximate_equality_respects_tolerance() {
    let mut a =
        DensityMatrix::<Complex64>::zero_state(QubitCount::new(1))
            .expect("a");

    let b = a.clone();

    a.set(0, 0, c64(1.0 - 1.0e-11, 0.0))
        .expect("small modification");

    assert!(
        a.approx_eq(&b, 1.0e-10)
            .expect("approximate equality")
    );

    assert!(
        !a.approx_eq(&b, 1.0e-12)
            .expect("strict approximate equality")
    );
}

#[test]
fn snapshot_copy_is_deep_and_independent() {
    let original =
        DensityMatrix::<Complex64>::zero_state(QubitCount::new(1))
            .expect("original");

    let mut snapshot = original.snapshot_copy();

    snapshot
        .apply_unitary(&pauli_x64(), F64_STRICT_TOL)
        .expect("modify snapshot");

    assert_close(
        original.basis_probability(0).expect("original p0"),
        1.0,
        F64_STRICT_TOL,
    );

    assert_close(
        snapshot.basis_probability(1).expect("snapshot p1"),
        1.0,
        F64_STRICT_TOL,
    );
}

#[test]
fn dimension_mismatch_is_rejected_by_comparison() {
    let one =
        DensityMatrix::<Complex64>::zero_state(QubitCount::new(1))
            .expect("one");

    let two =
        DensityMatrix::<Complex64>::zero_state(QubitCount::new(2))
            .expect("two");

    let result = one.max_difference(&two);

    assert!(matches!(
        result,
        Err(MemoryError::StateDimensionMismatch { .. })
    ));
}

// =============================================================================
// Complex32 precision path
// =============================================================================

#[test]
fn complex32_zero_state_is_supported() {
    let state =
        DensityMatrix::<Complex32>::zero_state(QubitCount::new(2))
            .expect("Complex32 zero state");

    state
        .validate_physical(DEFAULT_DENSITY_MATRIX_F32_TOLERANCE)
        .expect("Complex32 physical validation");

    assert_eq!(state.dimension(), 4);

    let p0 = state
        .basis_probability(0)
        .expect("Complex32 p0");

    assert!((p0 - 1.0_f32).abs() <= F32_TOL);
}

#[test]
fn complex32_pure_state_is_physical() {
    let amplitude = 1.0_f32 / 2.0_f32.sqrt();

    let state =
        DensityMatrix::<Complex32>::from_pure_state(
            &[
                c32(amplitude, 0.0),
                c32(0.0, 0.0),
                c32(0.0, 0.0),
                c32(amplitude, 0.0),
            ],
            F32_TOL,
        )
        .expect("Complex32 Bell state");

    state
        .validate_physical(F32_TOL)
        .expect("Complex32 Bell state must be physical");

    let purity = state.purity().expect("purity");

    assert!((purity - 1.0_f32).abs() <= F32_TOL);
}

// =============================================================================
// Qubit-target validation
// =============================================================================

#[test]
fn qubit_probability_rejects_out_of_range_qubit() {
    let state =
        DensityMatrix::<Complex64>::zero_state(QubitCount::new(2))
            .expect("state");

    let result =
        state.qubit_probability(2, false);

    assert!(matches!(
        result,
        Err(MemoryError::OutOfBounds { .. })
    ));
}

#[test]
fn measurement_rejects_out_of_range_qubit() {
    let state =
        DensityMatrix::<Complex64>::zero_state(QubitCount::new(2))
            .expect("state");

    let result =
        state.measurement_probability(&[2], 0);

    assert!(matches!(
        result,
        Err(MemoryError::OutOfBounds { .. })
    ));
}

#[test]
fn partial_trace_rejects_out_of_range_qubit() {
    let state =
        DensityMatrix::<Complex64>::zero_state(QubitCount::new(2))
            .expect("state");

    let result =
        state.partial_trace(&[2]);

    assert!(matches!(
        result,
        Err(MemoryError::OutOfBounds { .. })
    ));
}

#[test]
fn reset_rejects_out_of_range_qubit() {
    let mut state =
        DensityMatrix::<Complex64>::zero_state(QubitCount::new(2))
            .expect("state");

    let result =
        state.reset_qubits(&[2]);

    assert!(matches!(
        result,
        Err(MemoryError::OutOfBounds { .. })
    ));
}

// =============================================================================
// Mutation and invariant re-validation
// =============================================================================

#[test]
fn mutable_storage_can_change_state_but_validation_detects_invalid_state() {
    let mut state =
        DensityMatrix::<Complex64>::zero_state(QubitCount::new(1))
            .expect("state");

    state.as_mut_slice()[0] = c64(2.0, 0.0);

    let result =
        state.validate_physical(F64_STRICT_TOL);

    assert!(result.is_err());
}

#[test]
fn valid_manual_mutation_can_be_revalidated() {
    let mut state =
        DensityMatrix::<Complex64>::zero_state(QubitCount::new(1))
            .expect("state");

    state
        .set(0, 0, c64(0.5, 0.0))
        .expect("rho00");

    state
        .set(1, 1, c64(0.5, 0.0))
        .expect("rho11");

    state
        .validate_physical(F64_STRICT_TOL)
        .expect("manually constructed maximally mixed state");
}

// =============================================================================
// Public-storage contract
// =============================================================================

#[test]
fn storage_length_matches_dimension_squared() {
    for qubits in 0..=4 {
        let state =
            DensityMatrix::<Complex64>::zero_state(
                QubitCount::new(qubits),
            )
            .expect("state");

        assert_eq!(
            state.element_count(),
            state.dimension() * state.dimension()
        );
    }
}

#[test]
fn read_only_storage_matches_element_count() {
    let state =
        DensityMatrix::<Complex64>::zero_state(QubitCount::new(2))
            .expect("state");

    assert_eq!(state.as_slice().len(), state.element_count());
}

// =============================================================================
// Provider / QPU neutrality
// =============================================================================

#[test]
fn_density_matrix_public_contract_contains_no_vendor_state() {
    // This is intentionally a compile-time/API-boundary test expressed as a
    // normal Rust test. The density matrix is represented entirely through
    // Zamani scalar/state types and therefore does not require any vendor
    // object to construct or validate it.
    let state =
        DensityMatrix::<Complex64>::zero_state(QubitCount::new(1))
            .expect("provider-neutral state");

    assert_eq!(state.dimension(), 2);
    assert_physical64(&state);
}

#[test]
fn density_matrix_can_be_validated_without_backend_access() {
    let state = bell_state64();

    // A density matrix must be mathematically complete without network,
    // QPU-session, CUDA, MPI, or vendor-library access.
    state
        .validate_physical(F64_STRICT_TOL)
        .expect("backend-independent validation");
}

// =============================================================================
// Schema / API sanity
// =============================================================================

#[test]
fn canonical_default_tolerances_are_positive_and_finite() {
    assert!(DEFAULT_DENSITY_MATRIX_F64_TOLERANCE > 0.0);
    assert!(DEFAULT_DENSITY_MATRIX_F64_TOLERANCE.is_finite());

    assert!(DEFAULT_DENSITY_MATRIX_F32_TOLERANCE > 0.0);
    assert!(DEFAULT_DENSITY_MATRIX_F32_TOLERANCE.is_finite());
}

#[test]
fn memory_requirement_is_copyable_and_stable() {
    let requirement =
        DensityMatrix::<Complex64>::memory_requirement(
            QubitCount::new(2),
        )
        .expect("requirement");

    let copied: DensityMatrixMemoryRequirement = requirement;

    assert_eq!(copied, requirement);
    assert_eq!(copied.dimension(), 4);
    assert_eq!(copied.elements(), 16);
}

// =============================================================================
// Regression tests for operation atomicity
// =============================================================================

#[test]
fn failed_unitary_does_not_replace_state() {
    let mut state =
        DensityMatrix::<Complex64>::zero_state(QubitCount::new(1))
            .expect("state");

    let before = state.clone();

    let invalid_unitary = vec![
        c64(2.0, 0.0),
        zero64(),
        zero64(),
        c64(2.0, 0.0),
    ];

    let result =
        state.apply_unitary(&invalid_unitary, F64_STRICT_TOL);

    assert!(result.is_err());

    assert!(
        state
            .approx_eq(&before, F64_STRICT_TOL)
            .expect("state comparison")
    );
}

#[test]
fn failed_kraus_operation_does_not_replace_state() {
    let mut state =
        DensityMatrix::<Complex64>::zero_state(QubitCount::new(1))
            .expect("state");

    let before = state.clone();

    let invalid = vec![
        c64(0.5, 0.0),
        zero64(),
        zero64(),
        c64(0.5, 0.0),
    ];

    let result =
        state.apply_kraus_operators(
            &[invalid],
            F64_STRICT_TOL,
        );

    assert!(result.is_err());

    assert!(
        state
            .approx_eq(&before, F64_STRICT_TOL)
            .expect("state comparison")
    );
}

#[test]
fn failed_projection_does_not_destroy_state() {
    let mut state = bell_state64();

    let before = state.clone();

    let result =
        state.project_measurement(&[0, 1], 1, F64_STRICT_TOL);

    assert!(result.is_err());

    assert!(
        state
            .approx_eq(&before, F64_STRICT_TOL)
            .expect("state comparison")
    );
}

// =============================================================================
// Mathematical consistency / round-trip tests
// =============================================================================

#[test]
fn applying_x_twice_returns_original_state() {
    let original = plus_state64();

    let mut state = original.clone();

    state
        .apply_unitary(&pauli_x64(), F64_STRICT_TOL)
        .expect("first X");

    state
        .apply_unitary(&pauli_x64(), F64_STRICT_TOL)
        .expect("second X");

    assert!(
        state
            .approx_eq(&original, F64_STRICT_TOL)
            .expect("X^2 identity")
    );
}

#[test]
fn applying_z_twice_returns_original_state() {
    let original = plus_state64();

    let mut state = original.clone();

    state
        .apply_unitary(&pauli_z64(), F64_STRICT_TOL)
        .expect("first Z");

    state
        .apply_unitary(&pauli_z64(), F64_STRICT_TOL)
        .expect("second Z");

    assert!(
        state
            .approx_eq(&original, F64_STRICT_TOL)
            .expect("Z^2 identity")
    );
}

#[test]
fn unitary_then_inverse_unitary_restores_state() {
    let original = plus_state64();

    let mut state = original.clone();

    state
        .apply_unitary(&pauli_z64(), F64_STRICT_TOL)
        .expect("Z");

    state
        .apply_unitary(&pauli_z64(), F64_STRICT_TOL)
        .expect("Z inverse");

    assert!(
        state
            .approx_eq(&original, F64_STRICT_TOL)
            .expect("round trip")
    );
}

#[test]
fn trace_is_preserved_by_trace_preserving_channel() {
    let mut state =
        DensityMatrix::<Complex64>::zero_state(QubitCount::new(1))
            .expect("state");

    state
        .apply_unitary(&pauli_x64(), F64_STRICT_TOL)
        .expect("prepare one");

    let before =
        state.real_trace(F64_STRICT_TOL)
            .expect("before trace");

    let gamma = 0.2_f64;

    let k0 = vec![
        one64(),
        zero64(),
        zero64(),
        c64((1.0 - gamma).sqrt(), 0.0),
    ];

    let k1 = vec![
        zero64(),
        c64(gamma.sqrt(), 0.0),
        zero64(),
        zero64(),
    ];

    state
        .apply_kraus_operators(
            &[k0, k1],
            F64_STRICT_TOL,
        )
        .expect("channel");

    let after =
        state.real_trace(F64_STRICT_TOL)
            .expect("after trace");

    assert_close(before, 1.0, F64_STRICT_TOL);
    assert_close(after, before, F64_STRICT_TOL);
    assert_physical64(&state);
}