
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani classical numeric linear algebra.
//!
//! File: src/classical/numeric/linear_algebra.rs
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: unsafe Rust is forbidden.
//!
//! # Architectural contract
//!
//! PURPOSE
//!
//! Provide checked, target-independent linear-algebra operations over the
//! repository's existing matrix container and canonical finite floating-point
//! representation.
//!
//! AUTHORITY
//!
//! - Matrix storage and indexing:
//!   `crate::classical::collections::matrix::ClassicalMatrix`.
//! - Concrete floating-point representation and arithmetic:
//!   `crate::quantum::ir::classical::float::ClassicalFloat`.
//! - Arithmetic syntax:
//!   `grammar/expressions/arithmetic.g4`.
//! - Expression composition:
//!   `grammar/expressions/expressions.g4`.
//!
//! OWNS
//!
//! - Dimension compatibility checks for operations implemented here.
//! - Checked matrix addition and subtraction.
//! - Checked scalar multiplication.
//! - Checked matrix multiplication.
//! - Matrix transpose.
//! - Matrix-vector multiplication.
//! - Vector dot product and Euclidean norm.
//! - Gaussian-elimination linear-system solving with partial pivoting.
//! - Determinant calculation using elimination.
//! - Operation-specific errors and tests.
//!
//! DOES NOT OWN
//!
//! - Matrix, tensor, or vector storage.
//! - Numeric representation, precision, conversions, or promotion.
//! - Source grammar, tokens, AST, or expression precedence.
//! - Symbolic expressions, type inference, or overload resolution.
//! - Canonical classical or quantum IR.
//! - Allocation policy, resource discovery, scheduling, or hardware placement.
//! - Parallel execution, BLAS integration, or target-specific kernels.
//!
//! NUMERIC CONTRACT
//!
//! All concrete values use `ClassicalFloat`. Intermediate arithmetic uses its
//! checked operations, so non-finite values are not silently accepted.
//! Algorithms use exact zero checks for pivots; they do not silently introduce
//! an arbitrary tolerance. Numerically ill-conditioned systems may require
//! explicit tolerance/precision policy at a higher layer.
//!
//! SCALABILITY
//!
//! No fixed matrix dimension, vector length, machine count, register width, or
//! tensor-rank limit is introduced. Concrete matrix dimensions use the
//! collection API's `usize` indexing model, so unrepresentable products and
//! failed allocations are reported instead of wrapped.
//!
//! This does not promise infinite storage or compute. Work is bounded by the
//! caller's resources and explicit policy. Dense algorithms necessarily need
//! storage proportional to their concrete input and output sizes.
//!
//! INTEGRATION
//!
//! 1. Register this module in `src/classical/numeric/mod.rs`.
//! 2. Expose `numeric` through `src/classical/mod.rs` without replacing any
//!    existing module declarations.
//! 3. Semantic analysis checks operand types and dimensions before lowering or
//!    invoking concrete evaluation.
//! 4. Concrete matrix operations can call this API. Symbolic expressions must
//!    remain symbolic in their existing semantic/IR owner.
//! 5. Lowering adapts validated operations to canonical IR; this file does not
//!    create a second IR or modify grammar.
//!
//! FREEZE RULE
//!
//! Changes to error behavior, pivoting semantics, numerical guarantees, or
//! public function signatures require compatibility review and regression
//! tests.

use core::fmt;

use crate::classical::collections::matrix::{
    ClassicalMatrix,
    ClassicalMatrixError,
};
use crate::quantum::ir::classical::float::{
    ClassicalFloat,
    ClassicalFloatError,
};

/// Error returned by concrete linear-algebra operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LinearAlgebraError {
    /// An operation failed in the canonical matrix container.
    Matrix(ClassicalMatrixError),

    /// Checked floating-point arithmetic failed.
    Float(ClassicalFloatError),

    /// The matrix dimensions do not satisfy the operation's requirements.
    DimensionMismatch {
        /// Operation that required compatible dimensions.
        operation: &'static str,
        /// Left/input row count, when applicable.
        left_rows: usize,
        /// Left/input column count, when applicable.
        left_columns: usize,
        /// Right/input row count, when applicable.
        right_rows: usize,
        /// Right/input column count, when applicable.
        right_columns: usize,
    },

    /// A vector's length does not match the required dimension.
    VectorLengthMismatch {
        /// Length required by the operation.
        expected: usize,
        /// Length supplied by the caller.
        actual: usize,
    },

    /// A coordinate exists logically but has no concrete stored value.
    ///
    /// This can occur with sparse or unmaterialized matrices. The operation
    /// does not silently interpret missing storage as numeric zero.
    MissingElement {
        /// Row containing the missing value.
        row: usize,
        /// Column containing the missing value.
        column: usize,
    },

    /// A requested allocation could not reserve enough memory.
    AllocationFailed {
        /// Number of elements requested.
        elements: usize,
    },

    /// The linear system has no usable pivot at the current elimination step.
    SingularMatrix {
        /// Elimination column in which the singularity was detected.
        column: usize,
    },

    /// The operation requires a square matrix.
    NonSquareMatrix {
        /// Number of rows.
        rows: usize,
        /// Number of columns.
        columns: usize,
    },

    /// An operation's result could not be represented by the collection API.
    SizeOverflow,
}

impl fmt::Display for LinearAlgebraError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Matrix(error) => {
                write!(f, "matrix operation failed: {error}")
            }
            Self::Float(error) => {
                write!(f, "linear-algebra numeric operation failed: {error}")
            }
            Self::DimensionMismatch {
                operation,
                left_rows,
                left_columns,
                right_rows,
                right_columns,
            } => write!(
                f,
                "{operation} requires compatible dimensions; \
                 left/input is {left_rows}x{left_columns}, \
                 right/input is {right_rows}x{right_columns}"
            ),
            Self::VectorLengthMismatch { expected, actual } => write!(
                f,
                "vector length mismatch: expected {expected}, received {actual}"
            ),
            Self::MissingElement { row, column } => write!(
                f,
                "matrix element ({row}, {column}) has no concrete stored value"
            ),
            Self::AllocationFailed { elements } => write!(
                f,
                "unable to reserve storage for {elements} elements"
            ),
            Self::SingularMatrix { column } => write!(
                f,
                "matrix is singular at elimination column {column}"
            ),
            Self::NonSquareMatrix { rows, columns } => write!(
                f,
                "operation requires a square matrix, received {rows}x{columns}"
            ),
            Self::SizeOverflow => {
                f.write_str("linear-algebra result size is not representable")
            }
        }
    }
}

impl std::error::Error for LinearAlgebraError {}

impl From<ClassicalMatrixError> for LinearAlgebraError {
    fn from(error: ClassicalMatrixError) -> Self {
        Self::Matrix(error)
    }
}

impl From<ClassicalFloatError> for LinearAlgebraError {
    fn from(error: ClassicalFloatError) -> Self {
        Self::Float(error)
    }
}

/// Result type for concrete linear-algebra operations.
pub type LinearAlgebraResult<T> = Result<T, LinearAlgebraError>;

/// Concrete matrix representation used by this module.
///
/// This is an alias to the existing collection type, not a new matrix owner.
pub type NumericMatrix = ClassicalMatrix<ClassicalFloat>;

/// Returns a stored matrix element or a structured error.
///
/// A missing sparse/unmaterialized element is not implicitly treated as zero:
/// callers must materialize data or choose a storage-aware algorithm.
fn element(
    matrix: &NumericMatrix,
    row: usize,
    column: usize,
) -> LinearAlgebraResult<ClassicalFloat> {
    matrix
        .get(row, column)?
        .copied()
        .ok_or(LinearAlgebraError::MissingElement { row, column })
}

/// Reserves output storage fallibly before filling it.
fn reserve_values(
    elements: usize,
) -> LinearAlgebraResult<Vec<ClassicalFloat>> {
    let mut values = Vec::new();
    values
        .try_reserve_exact(elements)
        .map_err(|_| LinearAlgebraError::AllocationFailed { elements })?;
    Ok(values)
}

/// Checks that two matrices have equal shapes.
fn require_same_shape(
    operation: &'static str,
    left: &NumericMatrix,
    right: &NumericMatrix,
) -> LinearAlgebraResult<()> {
    let (left_rows, left_columns) = left.shape();
    let (right_rows, right_columns) = right.shape();

    if (left_rows, left_columns) != (right_rows, right_columns) {
        return Err(LinearAlgebraError::DimensionMismatch {
            operation,
            left_rows,
            left_columns,
            right_rows,
            right_columns,
        });
    }

    Ok(())
}

/// Computes a checked elementwise sum.
///
/// Both operands must have identical shapes.
pub fn add(
    left: &NumericMatrix,
    right: &NumericMatrix,
) -> LinearAlgebraResult<NumericMatrix> {
    elementwise_binary("matrix addition", left, right, |a, b| {
        a.checked_add(b)
    })
}

/// Computes a checked elementwise difference.
///
/// Both operands must have identical shapes.
pub fn subtract(
    left: &NumericMatrix,
    right: &NumericMatrix,
) -> LinearAlgebraResult<NumericMatrix> {
    elementwise_binary("matrix subtraction", left, right, |a, b| {
        a.checked_sub(b)
    })
}

fn elementwise_binary<F>(
    operation: &'static str,
    left: &NumericMatrix,
    right: &NumericMatrix,
    mut apply: F,
) -> LinearAlgebraResult<NumericMatrix>
where
    F: FnMut(ClassicalFloat, ClassicalFloat)
        -> Result<ClassicalFloat, ClassicalFloatError>,
{
    require_same_shape(operation, left, right)?;

    let (rows, columns) = left.shape();
    let size = rows
        .checked_mul(columns)
        .ok_or(LinearAlgebraError::SizeOverflow)?;
    let mut output = reserve_values(size)?;

    for row in 0..rows {
        for column in 0..columns {
            let a = element(left, row, column)?;
            let b = element(right, row, column)?;
            output.push(apply(a, b)?);
        }
    }

    Ok(ClassicalMatrix::from_row_major(rows, columns, output)?)
}

/// Multiplies every matrix element by a finite scalar using checked arithmetic.
pub fn scale(
    matrix: &NumericMatrix,
    scalar: ClassicalFloat,
) -> LinearAlgebraResult<NumericMatrix> {
    let (rows, columns) = matrix.shape();
    let size = rows
        .checked_mul(columns)
        .ok_or(LinearAlgebraError::SizeOverflow)?;
    let mut output = reserve_values(size)?;

    for row in 0..rows {
        for column in 0..columns {
            output.push(element(matrix, row, column)?.checked_mul(scalar)?);
        }
    }

    Ok(ClassicalMatrix::from_row_major(rows, columns, output)?)
}

/// Computes the matrix product `left × right`.
///
/// If `left` has shape `(m, k)`, `right` must have shape `(k, n)` and the
/// result has shape `(m, n)`. The implementation checks dimension products
/// and uses checked scalar arithmetic.
pub fn multiply(
    left: &NumericMatrix,
    right: &NumericMatrix,
) -> LinearAlgebraResult<NumericMatrix> {
    let (left_rows, left_columns) = left.shape();
    let (right_rows, right_columns) = right.shape();

    if left_columns != right_rows {
        return Err(LinearAlgebraError::DimensionMismatch {
            operation: "matrix multiplication",
            left_rows,
            left_columns,
            right_rows,
            right_columns,
        });
    }

    let output_size = left_rows
        .checked_mul(right_columns)
        .ok_or(LinearAlgebraError::SizeOverflow)?;
    let mut output = reserve_values(output_size)?;

    for row in 0..left_rows {
        for column in 0..right_columns {
            let mut sum = ClassicalFloat::zero();

            for inner in 0..left_columns {
                let product = element(left, row, inner)?
                    .checked_mul(element(right, inner, column)?)?;
                sum = sum.checked_add(product)?;
            }

            output.push(sum);
        }
    }

    Ok(ClassicalMatrix::from_row_major(
        left_rows,
        right_columns,
        output,
    )?)
}

/// Returns the transpose of a matrix.
///
/// Transposition copies elements into a new row-major matrix. The input is
/// unchanged.
pub fn transpose(
    matrix: &NumericMatrix,
) -> LinearAlgebraResult<NumericMatrix> {
    let (rows, columns) = matrix.shape();
    let size = rows
        .checked_mul(columns)
        .ok_or(LinearAlgebraError::SizeOverflow)?;
    let mut output = reserve_values(size)?;

    for column in 0..columns {
        for row in 0..rows {
            output.push(element(matrix, row, column)?);
        }
    }

    Ok(ClassicalMatrix::from_row_major(columns, rows, output)?)
}

/// Computes `matrix × vector`.
pub fn multiply_vector(
    matrix: &NumericMatrix,
    vector: &[ClassicalFloat],
) -> LinearAlgebraResult<Vec<ClassicalFloat>> {
    let (rows, columns) = matrix.shape();

    if columns != vector.len() {
        return Err(LinearAlgebraError::VectorLengthMismatch {
            expected: columns,
            actual: vector.len(),
        });
    }

    let mut output = reserve_values(rows)?;

    for row in 0..rows {
        let mut sum = ClassicalFloat::zero();

        for (column, value) in vector.iter().copied().enumerate() {
            let product = element(matrix, row, column)?.checked_mul(value)?;
            sum = sum.checked_add(product)?;
        }

        output.push(sum);
    }

    Ok(output)
}

/// Computes the checked dot product of two vectors.
pub fn dot(
    left: &[ClassicalFloat],
    right: &[ClassicalFloat],
) -> LinearAlgebraResult<ClassicalFloat> {
    if left.len() != right.len() {
        return Err(LinearAlgebraError::VectorLengthMismatch {
            expected: left.len(),
            actual: right.len(),
        });
    }

    let mut sum = ClassicalFloat::zero();

    for (a, b) in left.iter().copied().zip(right.iter().copied()) {
        sum = sum.checked_add(a.checked_mul(b)?)?;
    }

    Ok(sum)
}

/// Computes the Euclidean norm of a vector.
///
/// This operation uses the canonical checked square-root implementation.
pub fn norm2(vector: &[ClassicalFloat]) -> LinearAlgebraResult<ClassicalFloat> {
    let squared_norm = dot(vector, vector)?;
    Ok(squared_norm.sqrt()?)
}

/// Solves `A × x = b` using Gaussian elimination with partial pivoting.
///
/// This routine requires a square, concretely readable matrix and a right-hand
/// side with matching length. It selects the largest absolute pivot among
/// the remaining rows. An exactly zero pivot is treated as singular; no
/// implicit epsilon or precision policy is imposed.
///
/// Complexity is cubic in the matrix dimension, and the working copy requires
/// quadratic storage. Large problems should be routed by a higher-level
/// planner to suitable sparse, iterative, distributed, or accelerator-backed
/// algorithms when those implementations are available.
pub fn solve_linear_system(
    matrix: &NumericMatrix,
    right_hand_side: &[ClassicalFloat],
) -> LinearAlgebraResult<Vec<ClassicalFloat>> {
    let (rows, columns) = matrix.shape();

    if rows != columns {
        return Err(LinearAlgebraError::NonSquareMatrix { rows, columns });
    }

    if right_hand_side.len() != rows {
        return Err(LinearAlgebraError::VectorLengthMismatch {
            expected: rows,
            actual: right_hand_side.len(),
        });
    }

    let cell_count = rows
        .checked_mul(columns)
        .ok_or(LinearAlgebraError::SizeOverflow)?;

    let mut coefficients = reserve_values(cell_count)?;
    for row in 0..rows {
        for column in 0..columns {
            coefficients.push(element(matrix, row, column)?);
        }
    }

    let mut rhs = reserve_values(rows)?;
    rhs.extend_from_slice(right_hand_side);

    for pivot_column in 0..rows {
        let mut pivot_row = pivot_column;
        let mut pivot_magnitude =
            coefficients[pivot_column * columns + pivot_column]
                .get()
                .abs();

        for candidate_row in (pivot_column + 1)..rows {
            let magnitude =
                coefficients[candidate_row * columns + pivot_column]
                    .get()
                    .abs();

            if magnitude > pivot_magnitude {
                pivot_magnitude = magnitude;
                pivot_row = candidate_row;
            }
        }

        if pivot_magnitude == 0.0 {
            return Err(LinearAlgebraError::SingularMatrix {
                column: pivot_column,
            });
        }

        if pivot_row != pivot_column {
            for column in 0..columns {
                coefficients.swap(
                    pivot_column * columns + column,
                    pivot_row * columns + column,
                );
            }
            rhs.swap(pivot_column, pivot_row);
        }

        let pivot = coefficients[pivot_column * columns + pivot_column];

        for row in (pivot_column + 1)..rows {
            let factor = coefficients[row * columns + pivot_column]
                .checked_div(pivot)?;

            coefficients[row * columns + pivot_column] =
                ClassicalFloat::zero();

            for column in (pivot_column + 1)..columns {
                let reduction = factor.checked_mul(
                    coefficients[pivot_column * columns + column],
                )?;

                let index = row * columns + column;
                coefficients[index] =
                    coefficients[index].checked_sub(reduction)?;
            }

            let rhs_reduction = factor.checked_mul(rhs[pivot_column])?;
            rhs[row] = rhs[row].checked_sub(rhs_reduction)?;
        }
    }

    let mut solution = reserve_values(rows)?;

    for _ in 0..rows {
        solution.push(ClassicalFloat::zero());
    }

    for row in (0..rows).rev() {
        let mut value = rhs[row];

        for column in (row + 1)..columns {
            let term = coefficients[row * columns + column]
                .checked_mul(solution[column])?;
            value = value.checked_sub(term)?;
        }

        let diagonal = coefficients[row * columns + row];
        if diagonal.is_zero() {
            return Err(LinearAlgebraError::SingularMatrix { column: row });
        }

        solution[row] = value.checked_div(diagonal)?;
    }

    Ok(solution)
}

/// Computes the determinant using elimination with partial pivoting.
///
/// The determinant is zero when an exactly zero pivot is encountered. Row
/// swaps reverse the determinant sign. All products and updates use checked
/// finite floating-point arithmetic.
pub fn determinant(
    matrix: &NumericMatrix,
) -> LinearAlgebraResult<ClassicalFloat> {
    let (rows, columns) = matrix.shape();

    if rows != columns {
        return Err(LinearAlgebraError::NonSquareMatrix { rows, columns });
    }

    if rows == 0 {
        // The determinant of the empty square matrix is the multiplicative
        // identity. This is useful for compositional algorithms.
        return Ok(ClassicalFloat::one());
    }

    let cell_count = rows
        .checked_mul(columns)
        .ok_or(LinearAlgebraError::SizeOverflow)?;
    let mut work = reserve_values(cell_count)?;

    for row in 0..rows {
        for column in 0..columns {
            work.push(element(matrix, row, column)?);
        }
    }

    let mut determinant = ClassicalFloat::one();
    let mut negate = false;

    for pivot_column in 0..columns {
        let mut pivot_row = pivot_column;
        let mut pivot_magnitude =
            work[pivot_column * columns + pivot_column].get().abs();

        for candidate_row in (pivot_column + 1)..rows {
            let magnitude =
                work[candidate_row * columns + pivot_column].get().abs();

            if magnitude > pivot_magnitude {
                pivot_magnitude = magnitude;
                pivot_row = candidate_row;
            }
        }

        if pivot_magnitude == 0.0 {
            return Ok(ClassicalFloat::zero());
        }

        if pivot_row != pivot_column {
            for column in 0..columns {
                work.swap(
                    pivot_column * columns + column,
                    pivot_row * columns + column,
                );
            }
            negate = !negate;
        }

        let pivot = work[pivot_column * columns + pivot_column];
        determinant = determinant.checked_mul(pivot)?;

        for row in (pivot_column + 1)..rows {
            let factor =
                work[row * columns + pivot_column].checked_div(pivot)?;

            work[row * columns + pivot_column] = ClassicalFloat::zero();

            for column in (pivot_column + 1)..columns {
                let reduction =
                    factor.checked_mul(work[pivot_column * columns + column])?;
                let index = row * columns + column;
                work[index] = work[index].checked_sub(reduction)?;
            }
        }
    }

    if negate {
        determinant = ClassicalFloat::zero().checked_sub(determinant)?;
    }

    Ok(determinant)
}

#[cfg(test)]
mod tests {
    use super::{
        add,
        determinant,
        dot,
        multiply,
        multiply_vector,
        norm2,
        scale,
        solve_linear_system,
        subtract,
        transpose,
        LinearAlgebraError,
        NumericMatrix,
    };
    use crate::quantum::ir::classical::float::ClassicalFloat;

    fn f(value: f64) -> ClassicalFloat {
        ClassicalFloat::new(value).expect("test values must be finite")
    }

    fn matrix(rows: usize, columns: usize, values: &[f64]) -> NumericMatrix {
        NumericMatrix::from_row_major(
            rows,
            columns,
            values.iter().copied().map(f).collect(),
        )
        .expect("test matrix dimensions must be valid")
    }

    fn values(matrix: &NumericMatrix) -> Vec<f64> {
        let (rows, columns) = matrix.shape();
        let mut result = Vec::new();

        for row in 0..rows {
            for column in 0..columns {
                result.push(
                    *matrix
                        .get(row, column)
                        .expect("valid test coordinate")
                        .expect("test matrix is dense"),
                );
            }
        }

        result.into_iter().map(ClassicalFloat::get).collect()
    }

    #[test]
    fn addition_and_subtraction_are_elementwise() {
        let left = matrix(2, 2, &[1.0, 2.0, 3.0, 4.0]);
        let right = matrix(2, 2, &[4.0, 3.0, 2.0, 1.0]);

        assert_eq!(values(&add(&left, &right).expect("addition")), vec![5.0; 4]);
        assert_eq!(
            values(&subtract(&left, &right).expect("subtraction")),
            vec![-3.0, -1.0, 1.0, 3.0]
        );
    }

    #[test]
    fn incompatible_elementwise_shapes_are_rejected() {
        let left = matrix(2, 2, &[1.0, 2.0, 3.0, 4.0]);
        let right = matrix(1, 4, &[1.0, 2.0, 3.0, 4.0]);

        assert!(matches!(
            add(&left, &right),
            Err(LinearAlgebraError::DimensionMismatch { .. })
        ));
    }

    #[test]
    fn scalar_multiplication_uses_checked_values() {
        let input = matrix(2, 2, &[1.0, -2.0, 3.0, -4.0]);
        let output = scale(&input, f(2.0)).expect("finite scaled matrix");

        assert_eq!(values(&output), vec![2.0, -4.0, 6.0, -8.0]);
    }

    #[test]
    fn matrix_multiplication_computes_expected_product() {
        let left = matrix(2, 3, &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        let right = matrix(3, 2, &[7.0, 8.0, 9.0, 10.0, 11.0, 12.0]);

        let output = multiply(&left, &right).expect("compatible matrices");

        assert_eq!(output.shape(), (2, 2));
        assert_eq!(values(&output), vec![58.0, 64.0, 139.0, 154.0]);
    }

    #[test]
    fn incompatible_matrix_product_is_rejected() {
        let left = matrix(2, 3, &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        let right = matrix(2, 2, &[1.0, 2.0, 3.0, 4.0]);

        assert!(matches!(
            multiply(&left, &right),
            Err(LinearAlgebraError::DimensionMismatch { .. })
        ));
    }

    #[test]
    fn transpose_reorders_rows_and_columns() {
        let input = matrix(2, 3, &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        let output = transpose(&input).expect("transpose");

        assert_eq!(output.shape(), (3, 2));
        assert_eq!(values(&output), vec![1.0, 4.0, 2.0, 5.0, 3.0, 6.0]);
    }

    #[test]
    fn matrix_vector_product_is_checked() {
        let input = matrix(2, 3, &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        let output = multiply_vector(&input, &[f(1.0), f(2.0), f(3.0)])
            .expect("compatible vector");

        assert_eq!(
            output.into_iter().map(ClassicalFloat::get).collect::<Vec<_>>(),
            vec![14.0, 32.0]
        );
    }

    #[test]
    fn dot_product_and_norm_are_correct() {
        assert_eq!(
            dot(&[f(1.0), f(2.0)], &[f(3.0), f(4.0)])
                .expect("dot product")
                .get(),
            11.0
        );

        assert_eq!(norm2(&[f(3.0), f(4.0)]).expect("norm").get(), 5.0);
    }

    #[test]
    fn gaussian_elimination_solves_linear_system() {
        // 2x + y = 5
        //  x - y = 1
        let a = matrix(2, 2, &[2.0, 1.0, 1.0, -1.0]);
        let b = [f(5.0), f(1.0)];

        let solution = solve_linear_system(&a, &b).expect("nonsingular system");
        let actual: Vec<f64> = solution.into_iter().map(ClassicalFloat::get).collect();

        assert!((actual[0] - 2.0).abs() < 1.0e-12);
        assert!((actual[1] - 1.0).abs() < 1.0e-12);
    }

    #[test]
    fn gaussian_elimination_pivots_when_first_diagonal_is_zero() {
        let a = matrix(2, 2, &[0.0, 1.0, 1.0, 1.0]);
        let b = [f(1.0), f(2.0)];

        let solution = solve_linear_system(&a, &b).expect("pivoting system");
        let actual: Vec<f64> = solution.into_iter().map(ClassicalFloat::get).collect();

        assert!((actual[0] - 1.0).abs() < 1.0e-12);
        assert!((actual[1] - 1.0).abs() < 1.0e-12);
    }

    #[test]
    fn singular_system_is_reported() {
        let a = matrix(2, 2, &[1.0, 2.0, 2.0, 4.0]);
        let b = [f(3.0), f(6.0)];

        assert!(matches!(
            solve_linear_system(&a, &b),
            Err(LinearAlgebraError::SingularMatrix { .. })
        ));
    }

    #[test]
    fn determinant_accounts_for_row_swaps() {
        let a = matrix(2, 2, &[0.0, 1.0, 1.0, 0.0]);

        assert_eq!(determinant(&a).expect("determinant").get(), -1.0);
    }

    #[test]
    fn determinant_of_singular_matrix_is_zero() {
        let a = matrix(2, 2, &[1.0, 2.0, 2.0, 4.0]);

        assert_eq!(determinant(&a).expect("determinant").get(), 0.0);
    }

    #[test]
    fn non_square_determinant_is_rejected() {
        let a = matrix(2, 3, &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);

        assert!(matches!(
            determinant(&a),
            Err(LinearAlgebraError::NonSquareMatrix { .. })
        ));
    }

    #[test]
    fn empty_square_determinant_is_one() {
        let empty = NumericMatrix::from_row_major(0, 0, Vec::new())
            .expect("empty square matrix");

        assert_eq!(determinant(&empty).expect("empty determinant").get(), 1.0);
    }

    #[test]
    fn overflow_is_reported_instead_of_returning_infinity() {
        let large = f(f64::MAX);
        let a = NumericMatrix::from_row_major(1, 1, vec![large])
            .expect("single-element matrix");

        assert!(matches!(
            scale(&a, f(2.0)),
            Err(LinearAlgebraError::Float(_))
        ));
    }

    #[test]
    fn vector_length_mismatch_is_reported() {
        let a = matrix(2, 2, &[1.0, 0.0, 0.0, 1.0]);

        assert!(matches!(
            multiply_vector(&a, &[f(1.0)]),
            Err(LinearAlgebraError::VectorLengthMismatch { .. })
        ));
    }
}
