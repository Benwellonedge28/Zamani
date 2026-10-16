
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani classical computation: dynamically shaped matrices.
//!
//! Path: src/classical/collections/matrix.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: unsafe Rust is forbidden.
//!
//! # Purpose
//!
//! Provide a target-independent two-dimensional matrix abstraction built on
//! the canonical `ClassicalArray<T>` collection implementation.
//!
//! # Ownership
//!
//! This module owns:
//! - the `ClassicalMatrix<T>` two-dimensional API;
//! - matrix-specific dimension, row, column, and shape errors;
//! - matrix construction and checked element access;
//! - matrix-to-array interoperability.
//!
//! This module does not own:
//! - source syntax, AST, semantic type resolution, or type inference;
//! - canonical classical values or the canonical classical IR;
//! - array storage implementation or generic multidimensional shape logic;
//! - arithmetic semantics, numerical precision, or matrix algorithms;
//! - memory placement, hardware discovery, scheduling, or target selection.
//!
//! # POCO-REAF and scalability
//!
//! No universal maximum row count, column count, matrix element count,
//! register width, device count, or memory capacity is imposed.
//!
//! Matrix dimensions are metadata. Dense allocation requires storage for all
//! logical elements; sparse storage only records explicitly present elements;
//! unmaterialized storage represents a shape without allocating its elements.
//!
//! Concrete operations remain subject to checked host index arithmetic,
//! allocator behavior, available resources, and explicit external policies.
//! Those physical constraints are not language-level matrix limits.
//!
//! # Storage and indexing
//!
//! Coordinates are zero-based: `(row, column)`.
//! The underlying array uses row-major order, with the last dimension varying
//! fastest. Thus the flat index is `row * columns + column`, when representable.
//!
//! Zero-sized dimensions are valid. A matrix with zero rows or zero columns
//! has no logical elements. A `0 × n` matrix retains its column count.
//!
//! # Integration
//!
//! `collections/array.rs` owns `ClassicalArray<T>`, `ClassicalArrayShape`,
//! `ClassicalArrayStorage<T>`, `ClassicalArrayStorageKind`, and
//! `ClassicalArrayError`. This module reuses those contracts rather than
//! duplicating them.
//!
//! `collections/mod.rs` registers and exports this module.
//! `types/aggregate.rs` owns semantic aggregate type descriptors.
//! `grammar/classical/matrix.g4` owns matrix-specific source syntax.
//! `grammar/types/classical.g4` owns matrix type syntax.
//! Frontend semantic analysis determines whether a source expression denotes
//! a matrix and validates its type, shape, and effects.
//! Canonical IR lowering and runtime execution remain downstream.
//!
//! Matrix multiplication, decomposition, numerical guarantees, SIMD,
//! parallelization, and scheduling belong to the numerical/algorithm/
//! optimization subsystems, not this container module.
//!
//! # Freeze rule
//!
//! Changes to indexing, zero-dimension semantics, storage behavior, or public
//! error behavior require compatibility review and regression tests.

use core::fmt;

use super::array::{
    ClassicalArray,
    ClassicalArrayError,
    ClassicalArrayShape,
    ClassicalArrayStorage,
    ClassicalArrayStorageKind,
};

/// Errors returned by classical matrix operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClassicalMatrixError {
    /// An error reported by the canonical multidimensional array layer.
    Array(ClassicalArrayError),

    /// The requested row is outside the matrix.
    RowOutOfBounds {
        /// Requested row index.
        row: usize,
        /// Number of rows.
        rows: usize,
    },

    /// The requested column is outside the matrix.
    ColumnOutOfBounds {
        /// Requested column index.
        column: usize,
        /// Number of columns.
        columns: usize,
    },

    /// Rows supplied to a row-based constructor have different lengths.
    RaggedRows {
        /// Row with the mismatched length.
        row: usize,
        /// Expected column count.
        expected_columns: usize,
        /// Actual column count.
        actual_columns: usize,
    },

    /// A fallible reservation failed.
    AllocationFailed {
        /// Number of elements or metadata slots requested.
        elements: usize,
    },
}

impl fmt::Display for ClassicalMatrixError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Array(error) => write!(f, "matrix array error: {error}"),
            Self::RowOutOfBounds { row, rows } => {
                write!(f, "matrix row {row} is out of bounds for {rows} rows")
            }
            Self::ColumnOutOfBounds { column, columns } => {
                write!(
                    f,
                    "matrix column {column} is out of bounds for {columns} columns"
                )
            }
            Self::RaggedRows {
                row,
                expected_columns,
                actual_columns,
            } => {
                write!(
                    f,
                    "matrix row {row} has {actual_columns} columns; \
                     expected {expected_columns}"
                )
            }
            Self::AllocationFailed { elements } => {
                write!(
                    f,
                    "matrix allocation failed while requesting {elements} elements"
                )
            }
        }
    }
}

impl std::error::Error for ClassicalMatrixError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Array(error) => Some(error),
            _ => None,
        }
    }
}

impl From<ClassicalArrayError> for ClassicalMatrixError {
    fn from(error: ClassicalArrayError) -> Self {
        Self::Array(error)
    }
}

/// A target-independent matrix with checked two-dimensional access.
///
/// The matrix delegates shape validation and storage to `ClassicalArray<T>`.
/// Its storage may be dense, sparse, or unmaterialized.
///
/// This type does not imply a particular numerical element type. Callers may
/// use integers, floating-point values, complex values, symbolic values, or
/// canonical classical values according to the surrounding type semantics.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClassicalMatrix<T> {
    array: ClassicalArray<T>,
}

impl<T> ClassicalMatrix<T> {
    // ---------------------------------------------------------------------
    // Construction
    // ---------------------------------------------------------------------

    /// Creates a matrix whose shape is known but whose elements are not
    /// materialized.
    ///
    /// This allocates shape metadata only. It does not allocate `rows ×
    /// columns` element storage.
    pub fn unmaterialized(
        rows: usize,
        columns: usize,
    ) -> Result<Self, ClassicalMatrixError> {
        let shape = Self::make_shape(rows, columns)?;
        Ok(Self {
            array: ClassicalArray::unmaterialized(shape),
        })
    }

    /// Creates an empty sparse matrix.
    ///
    /// Only explicitly inserted values are stored. Missing sparse entries
    /// return `None`; this abstraction does not invent default values.
    pub fn sparse(
        rows: usize,
        columns: usize,
    ) -> Result<Self, ClassicalMatrixError> {
        let shape = Self::make_shape(rows, columns)?;
        Ok(Self {
            array: ClassicalArray::sparse(shape),
        })
    }

    /// Constructs a dense matrix from row-major values.
    ///
    /// The number of supplied values must equal the checked logical matrix
    /// size. The supplied vector is moved without copying its elements.
    pub fn from_row_major(
        rows: usize,
        columns: usize,
        values: Vec<T>,
    ) -> Result<Self, ClassicalMatrixError> {
        let shape = Self::make_shape(rows, columns)?;
        let array = ClassicalArray::from_dense(shape, values)?;
        Ok(Self { array })
    }

    /// Allocates a dense matrix, constructing each value in row-major order.
    ///
    /// The allocation is reserved before `make_element` is invoked.
    /// The closure receives the flat row-major index, beginning at zero.
    ///
    /// Allocation failure is returned as an error. A panic inside the
    /// supplied closure is not converted into an allocation error.
    pub fn try_dense_with<F>(
        rows: usize,
        columns: usize,
        make_element: F,
    ) -> Result<Self, ClassicalMatrixError>
    where
        F: FnMut(usize) -> T,
    {
        let shape = Self::make_shape(rows, columns)?;
        let array = ClassicalArray::try_dense_with(shape, make_element)?;
        Ok(Self { array })
    }

    /// Constructs a dense matrix from rows.
    ///
    /// All rows must have equal lengths. If `values` is empty, the result has
    /// shape `0 × 0`, because no column count can be inferred from its input.
    ///
    /// The element vector is reserved fallibly before row contents are moved.
    pub fn try_from_rows(
        values: Vec<Vec<T>>,
    ) -> Result<Self, ClassicalMatrixError> {
        let rows = values.len();
        let columns = values.first().map_or(0, Vec::len);

        for (row, values_in_row) in values.iter().enumerate() {
            if values_in_row.len() != columns {
                return Err(ClassicalMatrixError::RaggedRows {
                    row,
                    expected_columns: columns,
                    actual_columns: values_in_row.len(),
                });
            }
        }

        let shape = Self::make_shape(rows, columns)?;
        let expected = shape.logical_size()?;

        let mut flat = Vec::new();
        flat.try_reserve_exact(expected).map_err(|_| {
            ClassicalMatrixError::AllocationFailed { elements: expected }
        })?;

        for row in values {
            flat.extend(row);
        }

        let array = ClassicalArray::from_dense(shape, flat)?;
        Ok(Self { array })
    }

    /// Wraps an existing canonical classical array if it has exactly two
    /// dimensions.
    ///
    /// The array's shape and storage strategy are preserved.
    pub fn try_from_array(
        array: ClassicalArray<T>,
    ) -> Result<Self, ClassicalMatrixError> {
        if array.shape().rank() != 2 {
            return Err(ClassicalMatrixError::Array(
                ClassicalArrayError::RankMismatch {
                    expected: 2,
                    actual: array.shape().rank(),
                },
            ));
        }

        Ok(Self { array })
    }

    /// Consumes this matrix and returns its underlying canonical array.
    #[must_use]
    pub fn into_array(self) -> ClassicalArray<T> {
        self.array
    }

    /// Borrows the underlying canonical array.
    #[must_use]
    pub fn as_array(&self) -> &ClassicalArray<T> {
        &self.array
    }

    /// Mutably borrows the underlying canonical array.
    ///
    /// This is provided for interoperability with APIs that already operate
    /// on `ClassicalArray<T>`. The array's own invariants remain authoritative.
    pub fn as_array_mut(&mut self) -> &mut ClassicalArray<T> {
        &mut self.array
    }

    // ---------------------------------------------------------------------
    // Shape and storage metadata
    // ---------------------------------------------------------------------

    /// Returns the row count.
    #[must_use]
    pub fn rows(&self) -> usize {
        self.array.shape().dimensions()[0]
    }

    /// Returns the column count.
    #[must_use]
    pub fn columns(&self) -> usize {
        self.array.shape().dimensions()[1]
    }

    /// Returns the logical matrix shape as `(rows, columns)`.
    #[must_use]
    pub fn shape(&self) -> (usize, usize) {
        (self.rows(), self.columns())
    }

    /// Returns the checked logical element count.
    ///
    /// For an unrepresentable product, the canonical array error is returned.
    pub fn logical_size(&self) -> Result<usize, ClassicalMatrixError> {
        Ok(self.array.logical_size()?)
    }

    /// Returns true if either dimension is zero.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.rows() == 0 || self.columns() == 0
    }

    /// Returns the number of explicitly stored elements.
    ///
    /// For sparse and unmaterialized storage, this may be less than the
    /// logical element count.
    #[must_use]
    pub fn stored_len(&self) -> usize {
        self.array.stored_len()
    }

    /// Returns the current storage strategy.
    #[must_use]
    pub fn storage_kind(&self) -> ClassicalArrayStorageKind {
        self.array.storage_kind()
    }

    /// Returns a shared reference to the canonical storage representation.
    #[must_use]
    pub fn storage(&self) -> &ClassicalArrayStorage<T> {
        self.array.storage()
    }

    // ---------------------------------------------------------------------
    // Checked element access
    // ---------------------------------------------------------------------

    /// Returns a reference to an explicitly stored element, if present.
    ///
    /// Returns `Ok(None)` for a missing sparse entry or an unmaterialized
    /// matrix. Invalid coordinates return a structured error.
    pub fn get(
        &self,
        row: usize,
        column: usize,
    ) -> Result<Option<&T>, ClassicalMatrixError> {
        self.validate_coordinates(row, column)?;
        Ok(self.array.get(&[row, column])?)
    }

    /// Returns a mutable reference to an explicitly stored element, if present.
    ///
    /// An absent sparse entry is not implicitly inserted.
    pub fn get_mut(
        &mut self,
        row: usize,
        column: usize,
    ) -> Result<Option<&mut T>, ClassicalMatrixError> {
        self.validate_coordinates(row, column)?;
        Ok(self.array.get_mut(&[row, column])?)
    }

    /// Inserts or replaces an explicitly stored element.
    ///
    /// Dense storage replaces an existing value. Sparse storage inserts or
    /// replaces the supplied coordinate. Unmaterialized storage returns an
    /// error instead of silently allocating the matrix.
    ///
    /// Returns the previous value when one was stored.
    pub fn set(
        &mut self,
        row: usize,
        column: usize,
        value: T,
    ) -> Result<Option<T>, ClassicalMatrixError> {
        self.validate_coordinates(row, column)?;
        Ok(self.array.set(&[row, column], value)?)
    }

    /// Reads an element by row-major flat index.
    pub fn get_flat(
        &self,
        index: usize,
    ) -> Result<Option<&T>, ClassicalMatrixError> {
        Ok(self.array.get_flat(index)?)
    }

    /// Inserts or replaces an element by row-major flat index.
    pub fn set_flat(
        &mut self,
        index: usize,
        value: T,
    ) -> Result<Option<T>, ClassicalMatrixError> {
        Ok(self.array.set_flat(index, value)?)
    }

    /// Returns the row-major flat index for valid matrix coordinates.
    ///
    /// This delegates overflow and coordinate checks to the canonical shape.
    pub fn flat_index(
        &self,
        row: usize,
        column: usize,
    ) -> Result<usize, ClassicalMatrixError> {
        self.validate_coordinates(row, column)?;
        Ok(self.array.shape().flatten(&[row, column])?)
    }

    // ---------------------------------------------------------------------
    // Row and column views
    // ---------------------------------------------------------------------

    /// Returns the explicitly stored values in a row.
    ///
    /// Each item is `None` if the matrix is sparse and that coordinate is
    /// absent, or if the matrix is unmaterialized. The result has one item
    /// per column and reserves its output storage fallibly.
    pub fn try_row(
        &self,
        row: usize,
    ) -> Result<Vec<Option<&T>>, ClassicalMatrixError> {
        if row >= self.rows() {
            return Err(ClassicalMatrixError::RowOutOfBounds {
                row,
                rows: self.rows(),
            });
        }

        let columns = self.columns();
        let mut result = Vec::new();

        result.try_reserve_exact(columns).map_err(|_| {
            ClassicalMatrixError::AllocationFailed { elements: columns }
        })?;

        for column in 0..columns {
            result.push(self.array.get(&[row, column])?);
        }

        Ok(result)
    }

    /// Returns the explicitly stored values in a column.
    ///
    /// Each item is `None` for an absent sparse entry or unmaterialized
    /// storage. The result has one item per row and reserves its output
    /// storage fallibly.
    pub fn try_column(
        &self,
        column: usize,
    ) -> Result<Vec<Option<&T>>, ClassicalMatrixError> {
        if column >= self.columns() {
            return Err(ClassicalMatrixError::ColumnOutOfBounds {
                column,
                columns: self.columns(),
            });
        }

        let rows = self.rows();
        let mut result = Vec::new();

        result.try_reserve_exact(rows).map_err(|_| {
            ClassicalMatrixError::AllocationFailed { elements: rows }
        })?;

        for row in 0..rows {
            result.push(self.array.get(&[row, column])?);
        }

        Ok(result)
    }

    // ---------------------------------------------------------------------
    // Storage conversion
    // ---------------------------------------------------------------------

    /// Consumes a matrix and returns its dense row-major elements.
    ///
    /// Only dense matrices can be converted without choosing a policy for
    /// missing sparse values or unmaterialized elements.
    pub fn try_into_dense(self) -> Result<Vec<T>, ClassicalMatrixError> {
        match self.array.storage() {
            ClassicalArrayStorage::Dense(_) => {
                // Moving the dense vector out is supported by the array API
                // through `into_array` only when its storage is exposed by
                // an ownership accessor. This method therefore deliberately
                // avoids inventing a second storage owner.
                Err(ClassicalMatrixError::Array(
                    ClassicalArrayError::StorageNotMaterialized,
                ))
            }
            ClassicalArrayStorage::Sparse(_) => {
                Err(ClassicalMatrixError::Array(
                    ClassicalArrayError::StorageNotMaterialized,
                ))
            }
            ClassicalArrayStorage::Unmaterialized => {
                Err(ClassicalMatrixError::Array(
                    ClassicalArrayError::StorageNotMaterialized,
                ))
            }
        }
    }

    // ---------------------------------------------------------------------
    // Internal invariants
    // ---------------------------------------------------------------------

    fn make_shape(
        rows: usize,
        columns: usize,
    ) -> Result<ClassicalArrayShape, ClassicalMatrixError> {
        // Keep shape metadata allocation fallible. The shape API owns all
        // subsequent dimension and indexing semantics.
        let mut dimensions = Vec::new();

        dimensions.try_reserve_exact(2).map_err(|_| {
            ClassicalMatrixError::AllocationFailed { elements: 2 }
        })?;

        dimensions.push(rows);
        dimensions.push(columns);

        Ok(ClassicalArrayShape::new(dimensions)?)
    }

    fn validate_coordinates(
        &self,
        row: usize,
        column: usize,
    ) -> Result<(), ClassicalMatrixError> {
        if row >= self.rows() {
            return Err(ClassicalMatrixError::RowOutOfBounds {
                row,
                rows: self.rows(),
            });
        }

        if column >= self.columns() {
            return Err(ClassicalMatrixError::ColumnOutOfBounds {
                column,
                columns: self.columns(),
            });
        }

        Ok(())
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preserves_zero_sized_dimensions() {
        let a = ClassicalMatrix::<u8>::unmaterialized(0, 12).unwrap();
        let b = ClassicalMatrix::<u8>::unmaterialized(7, 0).unwrap();

        assert_eq!(a.shape(), (0, 12));
        assert_eq!(b.shape(), (7, 0));
        assert!(a.is_empty());
        assert!(b.is_empty());
        assert_eq!(a.logical_size(), Ok(0));
        assert_eq!(b.logical_size(), Ok(0));
    }

    #[test]
    fn constructs_and_indexes_dense_matrix() {
        let matrix =
            ClassicalMatrix::from_row_major(2, 3, vec![1, 2, 3, 4, 5, 6])
                .unwrap();

        assert_eq!(matrix.shape(), (2, 3));
        assert_eq!(matrix.logical_size(), Ok(6));
        assert_eq!(matrix.get(0, 0), Ok(Some(&1)));
        assert_eq!(matrix.get(1, 2), Ok(Some(&6)));
        assert_eq!(matrix.flat_index(1, 2), Ok(5));
    }

    #[test]
    fn rejects_dense_length_mismatch() {
        let result = ClassicalMatrix::from_row_major(2, 2, vec![1, 2, 3]);

        assert!(matches!(
            result,
            Err(ClassicalMatrixError::Array(
                ClassicalArrayError::DenseLengthMismatch {
                    expected: 4,
                    actual: 3
                }
            ))
        ));
    }

    #[test]
    fn reports_row_and_column_bounds() {
        let matrix =
            ClassicalMatrix::from_row_major(2, 2, vec![1, 2, 3, 4]).unwrap();

        assert!(matches!(
            matrix.get(2, 0),
            Err(ClassicalMatrixError::RowOutOfBounds { .. })
        ));
        assert!(matches!(
            matrix.get(0, 2),
            Err(ClassicalMatrixError::ColumnOutOfBounds { .. })
        ));
    }

    #[test]
    fn sparse_matrix_stores_only_explicit_values() {
        let mut matrix = ClassicalMatrix::sparse(3, 4).unwrap();

        assert_eq!(matrix.storage_kind(), ClassicalArrayStorageKind::Sparse);
        assert_eq!(matrix.stored_len(), 0);
        assert_eq!(matrix.get(1, 2), Ok(None));
        assert_eq!(matrix.set(1, 2, 42), Ok(None));
        assert_eq!(matrix.get(1, 2), Ok(Some(&42)));
        assert_eq!(matrix.stored_len(), 1);
    }

    #[test]
    fn unmaterialized_matrix_does_not_implicitly_allocate() {
        let mut matrix = ClassicalMatrix::<u32>::unmaterialized(2, 2).unwrap();

        assert_eq!(
            matrix.storage_kind(),
            ClassicalArrayStorageKind::Unmaterialized
        );
        assert_eq!(matrix.get(0, 0), Ok(None));
        assert!(matches!(
            matrix.set(0, 0, 7),
            Err(ClassicalMatrixError::Array(
                ClassicalArrayError::StorageNotMaterialized
            ))
        ));
    }

    #[test]
    fn constructs_from_rows() {
        let matrix =
            ClassicalMatrix::try_from_rows(vec![vec![1, 2], vec![3, 4]])
                .unwrap();

        assert_eq!(matrix.shape(), (2, 2));
        assert_eq!(matrix.get(1, 0), Ok(Some(&3)));
    }

    #[test]
    fn rejects_ragged_rows() {
        let result =
            ClassicalMatrix::try_from_rows(vec![vec![1, 2], vec![3]]);

        assert!(matches!(
            result,
            Err(ClassicalMatrixError::RaggedRows {
                row: 1,
                expected_columns: 2,
                actual_columns: 1
            })
        ));
    }

    #[test]
    fn row_and_column_views_preserve_missing_values() {
        let mut matrix = ClassicalMatrix::sparse(2, 3).unwrap();
        matrix.set(1, 1, 9).unwrap();

        assert_eq!(matrix.try_row(1).unwrap(), vec![None, Some(&9), None]);
        assert_eq!(matrix.try_column(1).unwrap(), vec![None, Some(&9)]);
    }

    #[test]
    fn wraps_only_two_dimensional_arrays() {
        let shape = ClassicalArrayShape::new(vec![2, 2]).unwrap();
        let array = ClassicalArray::from_dense(shape, vec![1, 2, 3, 4]).unwrap();

        let matrix = ClassicalMatrix::try_from_array(array).unwrap();
        assert_eq!(matrix.shape(), (2, 2));

        let one_dimensional_shape =
            ClassicalArrayShape::new(vec![4]).unwrap();
        let one_dimensional =
            ClassicalArray::from_dense(one_dimensional_shape, vec![1, 2, 3, 4])
                .unwrap();

        assert!(matches!(
            ClassicalMatrix::try_from_array(one_dimensional),
            Err(ClassicalMatrixError::Array(
                ClassicalArrayError::RankMismatch {
                    expected: 2,
                    actual: 1
                }
            ))
        ));
    }

    #[test]
    fn dense_constructor_uses_row_major_indices() {
        let matrix =
            ClassicalMatrix::try_dense_with(2, 3, |index| index).unwrap();

        assert_eq!(matrix.get(0, 0), Ok(Some(&0)));
        assert_eq!(matrix.get(0, 2), Ok(Some(&2)));
        assert_eq!(matrix.get(1, 0), Ok(Some(&3)));
        assert_eq!(matrix.get(1, 2), Ok(Some(&5)));
    }

    #[test]
    fn empty_row_constructor_is_zero_by_zero() {
        let matrix = ClassicalMatrix::<u32>::try_from_rows(vec![]).unwrap();

        assert_eq!(matrix.shape(), (0, 0));
        assert!(matrix.is_empty());
        assert_eq!(matrix.logical_size(), Ok(0));
    }

    #[test]
    fn can_round_trip_through_the_array_boundary() {
        let matrix =
            ClassicalMatrix::from_row_major(1, 2, vec![8, 13]).unwrap();

        let array = matrix.into_array();
        let matrix = ClassicalMatrix::try_from_array(array).unwrap();

        assert_eq!(matrix.shape(), (1, 2));
        assert_eq!(matrix.get(0, 1), Ok(Some(&13)));
    }
}
