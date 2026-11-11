
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Standard Library — Scientific Linear Algebra.
//!
//! Path:
//!     src/stdlib/scientific/linear_algebra.rs
//!
//! Rust:
//!     Edition 2021
//!     Minimum version 1.97
//!
//! Safety:
//!     Unsafe Rust is forbidden.
//!
//! # Purpose
//!
//! This module exposes the canonical Zamani classical linear-algebra
//! implementation through the standard-library scientific API.
//!
//! It provides a stable public entry point for mathematical operations
//! used by classical, AI, scientific, hybrid and other computational
//! domains.
//!
//! # Canonical ownership
//!
//! This module delegates to:
//!
//!     crate::classical::numeric::linear_algebra
//!
//! Matrix storage belongs to:
//!
//!     crate::classical::collections::matrix
//!
//! Array and tensor storage belong to:
//!
//!     crate::classical::collections
//!
//! Floating-point semantics belong to:
//!
//!     crate::quantum::ir::classical::float
//!
//! # Responsibilities
//!
//! This module owns:
//!
//! - The standard-library linear-algebra API path.
//! - Public re-exports of canonical numerical operations.
//! - Standard-library-facing mathematical documentation.
//! - Compatibility between standard-library consumers and the
//!   canonical classical numerical implementation.
//!
//! # Does not own
//!
//! This module does not implement or redefine:
//!
//! - Matrix storage.
//! - Vector storage.
//! - Tensor storage.
//! - Floating-point semantics.
//! - Integer semantics.
//! - Numerical precision or promotion.
//! - Matrix algorithms.
//! - Classical IR.
//! - Quantum IR.
//! - HDL IR.
//! - ZUIR.
//! - Symbolic expression evaluation.
//! - Type checking.
//! - Hardware discovery.
//! - Resource allocation.
//! - Scheduling.
//! - Parallel execution.
//! - Distributed execution.
//! - GPU, FPGA, ASIC or QPU selection.
//! - Backend code generation.
//!
//! These responsibilities belong to their existing authoritative
//! repository subsystems.
//!
//! # POCO-REAF and scalability
//!
//! This module introduces no arbitrary universal limits on:
//!
//! - Matrix dimensions.
//! - Vector lengths.
//! - Tensor dimensions.
//! - Tensor rank.
//! - Number of processors.
//! - Number of accelerators.
//! - Number of execution providers.
//! - Distributed participants.
//! - Memory capacity.
//!
//! Concrete operations are subject to:
//!
//! - Available memory.
//! - Representable dimensions.
//! - Checked allocation.
//! - Numeric representation.
//! - Execution time.
//! - Explicit runtime resource policies.
//!
//! No implementation can promise literal infinite physical storage
//! or computation.
//!
//! The same mathematical API remains independent of the target
//! selected by the compiler and runtime.
//!
//! # Numerical behavior
//!
//! Concrete floating-point operations use the canonical ClassicalFloat.
//!
//! Arithmetic failures are returned through LinearAlgebraError.
//!
//! Missing sparse or unmaterialized elements are not silently
//! interpreted as zero.
//!
//! Singular systems are reported explicitly.
//!
//! Numerical tolerance must be supplied explicitly by an algorithm
//! that defines such a policy. This facade does not introduce an
//! implicit global epsilon.
//!
//! # Integration
//!
//! The dependency direction is:
//!
//!     stdlib::scientific::linear_algebra
//!                  |
//!                  v
//!     classical::numeric::linear_algebra
//!                  |
//!                  v
//!     classical::collections::matrix
//!                  |
//!                  v
//!     canonical ClassicalArray storage
//!
//! Semantic analysis determines operand types, shapes and operation
//! validity before concrete evaluation or lowering.
//!
//! The compiler and runtime determine how operations are realized.
//!
//! # Compatibility
//!
//! Existing canonical classical APIs remain available.
//!
//! This module adds a standard-library-facing path without removing
//! or replacing those APIs.
//!
//! # Freeze criteria
//!
//! This file is complete when:
//!
//! - All re-exports resolve.
//! - No numerical algorithm is duplicated.
//! - No competing matrix or scalar representation is introduced.
//! - Canonical errors remain visible to callers.
//! - Public operations are tested through this module path.
//! - Rust 1.97 compatibility is preserved.
//! - Unsafe Rust remains prohibited.
//! - Repository formatting, compilation, tests and lint pass.

// =============================================================================
// Canonical numeric representation
// =============================================================================

/// Canonical finite floating-point representation.
///
/// This is the existing semantic representation, not a new
/// standard-library floating-point type.
pub use crate::quantum::ir::classical::float::ClassicalFloat;

// =============================================================================
// Canonical matrix representation
// =============================================================================

/// Canonical dynamically shaped matrix.
///
/// The underlying storage remains owned by classical collections.
pub use crate::classical::collections::matrix::{
    ClassicalMatrix,
    ClassicalMatrixError,
};

// =============================================================================
// Canonical linear-algebra implementation
// =============================================================================

/// Canonical operation errors.
pub use crate::classical::numeric::linear_algebra::{
    LinearAlgebraError,
    LinearAlgebraResult,
};

/// Canonical concrete numeric matrix representation.
pub use crate::classical::numeric::linear_algebra::NumericMatrix;

/// Checked elementwise matrix addition.
///
/// Requires identical matrix dimensions.
pub use crate::classical::numeric::linear_algebra::add;

/// Computes the determinant using elimination with partial pivoting.
///
/// Requires a square matrix.
pub use crate::classical::numeric::linear_algebra::determinant;

/// Computes the checked dot product of two vectors.
pub use crate::classical::numeric::linear_algebra::dot;

/// Computes the matrix product.
///
/// For A(m × k) and B(k × n), produces C(m × n).
pub use crate::classical::numeric::linear_algebra::multiply;

/// Computes matrix-vector multiplication.
pub use crate::classical::numeric::linear_algebra::multiply_vector;

/// Computes the Euclidean vector norm.
pub use crate::classical::numeric::linear_algebra::norm2;

/// Multiplies a matrix by a finite scalar.
pub use crate::classical::numeric::linear_algebra::scale;

/// Solves a square linear system using Gaussian elimination
/// with partial pivoting.
pub use crate::classical::numeric::linear_algebra::solve_linear_system;

/// Computes elementwise matrix subtraction.
///
/// Requires identical matrix dimensions.
pub use crate::classical::numeric::linear_algebra::subtract;

/// Computes matrix transposition.
pub use crate::classical::numeric::linear_algebra::transpose;

// =============================================================================
// Integration tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::{
        add,
        determinant,
        dot,
        multiply,
        norm2,
        solve_linear_system,
        ClassicalFloat,
        LinearAlgebraError,
        NumericMatrix,
    };

    fn scalar(value: f64) -> ClassicalFloat {
        ClassicalFloat::new(value)
            .expect("test scalar must be finite")
    }

    fn matrix(
        rows: usize,
        columns: usize,
        values: &[f64],
    ) -> NumericMatrix {
        NumericMatrix::from_row_major(
            rows,
            columns,
            values.iter().copied().map(scalar).collect(),
        )
        .expect("test matrix must have valid dimensions")
    }

    fn values(matrix: &NumericMatrix) -> Vec<f64> {
        let (rows, columns) = matrix.shape();

        let mut output = Vec::new();

        for row in 0..rows {
            for column in 0..columns {
                let value = matrix
                    .get(row, column)
                    .expect("valid matrix coordinate")
                    .expect("test matrix is dense");

                output.push(value.get());
            }
        }

        output
    }

    #[test]
    fn standard_library_matrix_api_uses_canonical_implementation() {
        let left = matrix(
            2,
            2,
            &[
                1.0, 2.0,
                3.0, 4.0,
            ],
        );

        let right = matrix(
            2,
            2,
            &[
                5.0, 6.0,
                7.0, 8.0,
            ],
        );

        let result = add(&left, &right)
            .expect("matrix addition");

        assert_eq!(
            values(&result),
            vec![6.0, 8.0, 10.0, 12.0]
        );
    }

    #[test]
    fn matrix_multiplication_uses_canonical_storage() {
        let left = matrix(
            2,
            3,
            &[
                1.0, 2.0, 3.0,
                4.0, 5.0, 6.0,
            ],
        );

        let right = matrix(
            3,
            2,
            &[
                7.0, 8.0,
                9.0, 10.0,
                11.0, 12.0,
            ],
        );

        let result = multiply(&left, &right)
            .expect("compatible matrix multiplication");

        assert_eq!(result.shape(), (2, 2));

        assert_eq!(
            values(&result),
            vec![58.0, 64.0, 139.0, 154.0]
        );
    }

    #[test]
    fn determinant_uses_canonical_numeric_semantics() {
        let input = matrix(
            2,
            2,
            &[
                1.0, 2.0,
                3.0, 4.0,
            ],
        );

        let result = determinant(&input)
            .expect("valid determinant");

        assert_eq!(result.get(), -2.0);
    }

    #[test]
    fn linear_system_solver_is_available_through_stdlib() {
        let coefficients = matrix(
            2,
            2,
            &[
                2.0, 1.0,
                1.0, -1.0,
            ],
        );

        let rhs = [
            scalar(5.0),
            scalar(1.0),
        ];

        let solution = solve_linear_system(
            &coefficients,
            &rhs,
        )
        .expect("nonsingular system");

        assert!((solution[0].get() - 2.0).abs() < 1.0e-12);
        assert!((solution[1].get() - 1.0).abs() < 1.0e-12);
    }

    #[test]
    fn incompatible_dimensions_produce_structured_errors() {
        let left = matrix(2, 3, &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);

        let right = matrix(2, 2, &[1.0, 2.0, 3.0, 4.0]);

        assert!(matches!(
            multiply(&left, &right),
            Err(LinearAlgebraError::DimensionMismatch { .. })
        ));
    }

    #[test]
    fn vector_operations_use_canonical_scalars() {
        let left = [scalar(1.0), scalar(2.0)];
        let right = [scalar(3.0), scalar(4.0)];

        assert_eq!(
            dot(&left, &right)
                .expect("dot product")
                .get(),
            11.0
        );

        let vector = [scalar(3.0), scalar(4.0)];

        assert_eq!(
            norm2(&vector)
                .expect("vector norm")
                .get(),
            5.0
        );
    }
}
