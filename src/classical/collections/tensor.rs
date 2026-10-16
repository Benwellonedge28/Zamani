
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani classical collections: arbitrary-rank tensors.
//!
//! Path: src/classical/collections/tensor.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: safe Rust only.
//!
//! # File contract
//!
//! Status: Proposed implementation; repository compilation not yet verified.
//!
//! ## Purpose
//!
//! Provide a rank-independent tensor abstraction over the canonical
//! `ClassicalArray<T>` implementation.
//!
//! ## Authority and ownership
//!
//! This module owns:
//! - `ClassicalTensor<T>`, the public tensor convenience API;
//! - tensor-specific construction and interoperability methods;
//! - `ClassicalTensorError`, which preserves canonical array failures.
//!
//! `collections/array.rs` remains the authority for:
//! - `ClassicalArrayShape` and shape/indexing invariants;
//! - row-major flat indexing and coordinate conversion;
//! - `ClassicalArrayStorage<T>` and storage strategies;
//! - dense, sparse, and unmaterialized storage behavior;
//! - `ClassicalArrayError` and checked storage operations.
//!
//! This module does not own source syntax, AST nodes, semantic types, canonical
//! values, numerical algorithms, tensor-operation registries, compiler IR,
//! optimization, scheduling, target placement, or runtime resource policy.
//!
//! ## POCO-REAF and scalability
//!
//! No fixed maximum tensor rank, dimension, element count, device count,
//! memory capacity, or hardware architecture is imposed here.
//!
//! Rank is represented by the length of the canonical shape's dimension list.
//! Concrete flat indexing uses the host's `usize` representation and returns
//! canonical checked errors if a size or index cannot be represented.
//!
//! Shape-only and sparse tensors can describe large logical tensors without
//! allocating dense element storage. Dense materialization remains subject
//! to available resources and allocator behavior.
//!
//! ## Semantic invariants
//!
//! - Coordinates and flat indices are zero-based.
//! - The last dimension varies fastest (row-major order).
//! - The canonical array layer rejects rank-zero shapes.
//! - Any zero-length dimension yields zero logical elements.
//! - Sparse reads of absent entries return `None`; no default is invented.
//! - Unmaterialized tensors contain shape metadata but no element storage.
//! - Writes never silently materialize an unmaterialized tensor.
//! - Storage and shape semantics are delegated to `ClassicalArray<T>`.
//!
//! ## Integration
//!
//! `collections/mod.rs` registers this module and exports its public API.
//! `collections/array.rs` remains the canonical storage implementation.
//! `collections/shape.rs`, if introduced, must not duplicate array shape
//! authority; it may only provide adapters over the canonical shape.
//! `types/aggregate.rs` owns semantic type descriptors.
//! `grammar/classical/tensor.g4` owns tensor-domain syntax.
//! `grammar/types/classical.g4` owns tensor type syntax.
//! Frontend semantic analysis resolves types, dimensions, and operation
//! legality. Lowering uses the canonical IR boundary; this file creates no IR.
//! Numerical operations, optimization, and resource-aware execution remain
//! downstream concerns.
//!
//! ## Freeze rule
//!
//! Changes to rank semantics, indexing order, empty-dimension behavior,
//! storage behavior, or public error behavior require compatibility review
//! and regression tests.

use core::fmt;

use super::array::{
    ClassicalArray,
    ClassicalArrayError,
    ClassicalArrayShape,
    ClassicalArrayStorage,
    ClassicalArrayStorageKind,
};

/// Errors returned by the tensor convenience API.
///
/// The canonical array error is preserved rather than translated into a
/// second, potentially divergent shape/indexing error model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClassicalTensorError {
    /// A canonical array operation failed.
    Array(ClassicalArrayError),
}

impl fmt::Display for ClassicalTensorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Array(error) => write!(f, "tensor error: {error}"),
        }
    }
}

impl std::error::Error for ClassicalTensorError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Array(error) => Some(error),
        }
    }
}

impl From<ClassicalArrayError> for ClassicalTensorError {
    fn from(error: ClassicalArrayError) -> Self {
        Self::Array(error)
    }
}

/// An arbitrary-rank classical tensor backed by `ClassicalArray<T>`.
///
/// `ClassicalTensor<T>` does not restrict the element type. It can store
/// canonical Zamani values, numeric values, symbolic values, or other types
/// supported by the surrounding semantic system.
///
/// A tensor's logical shape is independent of its storage representation.
/// Dense storage contains every logical element; sparse storage contains only
/// explicitly stored elements; unmaterialized storage contains no elements.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClassicalTensor<T> {
    array: ClassicalArray<T>,
}

impl<T> ClassicalTensor<T> {
    // ---------------------------------------------------------------------
    // Construction
    // ---------------------------------------------------------------------

    /// Creates an unmaterialized tensor from an existing canonical shape.
    ///
    /// This records shape metadata without allocating element storage.
    pub fn unmaterialized(
        shape: ClassicalArrayShape,
    ) -> Self {
        Self {
            array: ClassicalArray::unmaterialized(shape),
        }
    }

    /// Creates an empty sparse tensor from an existing canonical shape.
    ///
    /// Only explicitly inserted values consume element storage.
    pub fn sparse(
        shape: ClassicalArrayShape,
    ) -> Self {
        Self {
            array: ClassicalArray::sparse(shape),
        }
    }

    /// Creates a dense tensor from an existing shape and row-major values.
    ///
    /// The number of supplied elements must equal the canonical shape's
    /// logical size. The array layer validates this invariant.
    pub fn from_dense(
        shape: ClassicalArrayShape,
        values: Vec<T>,
    ) -> Result<Self, ClassicalTensorError> {
        Ok(Self {
            array: ClassicalArray::from_dense(shape, values)?,
        })
    }

    /// Creates a dense tensor by generating elements in row-major order.
    ///
    /// The canonical array layer reserves storage fallibly before calling
    /// `make_element`. The closure receives the zero-based flat element index.
    ///
    /// A closure panic is not caught or translated into a tensor error.
    pub fn try_dense_with<F>(
        shape: ClassicalArrayShape,
        make_element: F,
    ) -> Result<Self, ClassicalTensorError>
    where
        F: FnMut(usize) -> T,
    {
        Ok(Self {
            array: ClassicalArray::try_dense_with(shape, make_element)?,
        })
    }

    /// Constructs a shape from the supplied dimensions without materializing
    /// element storage.
    ///
    /// At least one dimension is required by the canonical array shape
    /// contract. No fixed maximum rank is imposed.
    pub fn unmaterialized_with_dimensions(
        dimensions: Vec<usize>,
    ) -> Result<Self, ClassicalTensorError> {
        let shape = ClassicalArrayShape::new(dimensions)?;
        Ok(Self::unmaterialized(shape))
    }

    /// Creates an empty sparse tensor from its dimensions.
    pub fn sparse_with_dimensions(
        dimensions: Vec<usize>,
    ) -> Result<Self, ClassicalTensorError> {
        let shape = ClassicalArrayShape::new(dimensions)?;
        Ok(Self::sparse(shape))
    }

    /// Creates a dense tensor from its dimensions and row-major values.
    pub fn from_dense_with_dimensions(
        dimensions: Vec<usize>,
        values: Vec<T>,
    ) -> Result<Self, ClassicalTensorError> {
        let shape = ClassicalArrayShape::new(dimensions)?;
        Self::from_dense(shape, values)
    }

    /// Creates a dense tensor from its dimensions and a value generator.
    pub fn try_dense_with_dimensions<F>(
        dimensions: Vec<usize>,
        make_element: F,
    ) -> Result<Self, ClassicalTensorError>
    where
        F: FnMut(usize) -> T,
    {
        let shape = ClassicalArrayShape::new(dimensions)?;
        Self::try_dense_with(shape, make_element)
    }

    /// Wraps an existing canonical array without copying its elements.
    ///
    /// Any nonzero rank accepted by `ClassicalArrayShape` is valid for a
    /// tensor. The underlying array and its storage strategy remain intact.
    pub fn from_array(array: ClassicalArray<T>) -> Self {
        Self { array }
    }

    /// Borrows the underlying canonical array.
    #[must_use]
    pub fn as_array(&self) -> &ClassicalArray<T> {
        &self.array
    }

    /// Mutably borrows the underlying canonical array.
    ///
    /// The array continues to enforce all shape, indexing, and storage
    /// invariants. This method does not expose its private fields.
    pub fn as_array_mut(&mut self) -> &mut ClassicalArray<T> {
        &mut self.array
    }

    /// Consumes the tensor and returns the underlying canonical array.
    #[must_use]
    pub fn into_array(self) -> ClassicalArray<T> {
        self.array
    }

    // ---------------------------------------------------------------------
    // Shape and storage metadata
    // ---------------------------------------------------------------------

    /// Returns the canonical tensor shape.
    #[must_use]
    pub fn shape(&self) -> &ClassicalArrayShape {
        self.array.shape()
    }

    /// Returns the number of dimensions.
    #[must_use]
    pub fn rank(&self) -> usize {
        self.array.shape().rank()
    }

    /// Returns all dimensions in their canonical order.
    #[must_use]
    pub fn dimensions(&self) -> &[usize] {
        self.array.shape().dimensions()
    }

    /// Returns the length of a dimension, or a canonical bounds error.
    pub fn dimension(
        &self,
        dimension: usize,
    ) -> Result<usize, ClassicalTensorError> {
        Ok(self.array.shape().dimension(dimension)?)
    }

    /// Returns the logical element count if representable by `usize`.
    ///
    /// This does not materialize elements. A zero dimension makes the logical
    /// size zero even when other dimensions are large.
    pub fn logical_size(&self) -> Result<usize, ClassicalTensorError> {
        Ok(self.array.logical_size()?)
    }

    /// Returns true if any dimension has length zero.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.array.shape().is_empty()
    }

    /// Returns the number of explicitly stored elements.
    ///
    /// For sparse or unmaterialized tensors this can be smaller than the
    /// logical element count.
    #[must_use]
    pub fn stored_len(&self) -> usize {
        self.array.stored_len()
    }

    /// Returns the active storage strategy.
    #[must_use]
    pub fn storage_kind(&self) -> ClassicalArrayStorageKind {
        self.array.storage_kind()
    }

    /// Borrows the canonical storage representation.
    #[must_use]
    pub fn storage(&self) -> &ClassicalArrayStorage<T> {
        self.array.storage()
    }

    // ---------------------------------------------------------------------
    // Checked indexing
    // ---------------------------------------------------------------------

    /// Returns an explicitly stored element by multidimensional coordinates.
    ///
    /// Returns `Ok(None)` when a sparse coordinate is absent or storage has
    /// not been materialized. Invalid rank or coordinates return a canonical
    /// array error.
    pub fn get(
        &self,
        coordinates: &[usize],
    ) -> Result<Option<&T>, ClassicalTensorError> {
        Ok(self.array.get(coordinates)?)
    }

    /// Returns a mutable reference to an explicitly stored element.
    ///
    /// An absent sparse entry is not implicitly inserted.
    pub fn get_mut(
        &mut self,
        coordinates: &[usize],
    ) -> Result<Option<&mut T>, ClassicalTensorError> {
        let index = self.array.shape().flatten(coordinates)?;
        Ok(self.array.get_flat_mut(index)?)
    }

    /// Inserts or replaces an explicitly stored element by coordinates.
    ///
    /// Returns the old value if the coordinate already had a stored value.
    /// Sparse storage inserts a new entry when necessary. Unmaterialized
    /// storage rejects the write instead of silently allocating.
    pub fn set(
        &mut self,
        coordinates: &[usize],
        value: T,
    ) -> Result<Option<T>, ClassicalTensorError> {
        Ok(self.array.set(coordinates, value)?)
    }

    /// Reads an element using a row-major flat index.
    pub fn get_flat(
        &self,
        index: usize,
    ) -> Result<Option<&T>, ClassicalTensorError> {
        Ok(self.array.get_flat(index)?)
    }

    /// Returns a mutable reference using a row-major flat index.
    pub fn get_flat_mut(
        &mut self,
        index: usize,
    ) -> Result<Option<&mut T>, ClassicalTensorError> {
        Ok(self.array.get_flat_mut(index)?)
    }

    /// Inserts or replaces an explicitly stored element by flat index.
    pub fn set_flat(
        &mut self,
        index: usize,
        value: T,
    ) -> Result<Option<T>, ClassicalTensorError> {
        Ok(self.array.set_flat(index, value)?)
    }

    /// Removes an explicitly stored element by flat index.
    ///
    /// Dense storage remains positionally stable; its removal behavior is
    /// defined by the canonical array layer. Sparse storage removes the entry.
    pub fn remove_flat(
        &mut self,
        index: usize,
    ) -> Result<Option<T>, ClassicalTensorError> {
        Ok(self.array.remove_flat(index)?)
    }

    /// Returns the row-major flat index for valid coordinates.
    pub fn flat_index(
        &self,
        coordinates: &[usize],
    ) -> Result<usize, ClassicalTensorError> {
        Ok(self.array.shape().flatten(coordinates)?)
    }

    /// Converts a valid flat index into coordinates.
    ///
    /// This uses the canonical shape implementation and returns its checked
    /// error if the index is out of range or the logical size overflows.
    pub fn coordinates(
        &self,
        flat_index: usize,
    ) -> Result<Vec<usize>, ClassicalTensorError> {
        Ok(self.array.shape().coordinates(flat_index)?)
    }

    /// Returns a lazy range of valid flat indices.
    ///
    /// No vector of indices is allocated. The logical size must be
    /// representable by `usize`.
    pub fn flat_indices(
        &self,
    ) -> Result<std::ops::Range<usize>, ClassicalTensorError> {
        Ok(self.array.shape().flat_indices()?)
    }

    // ---------------------------------------------------------------------
    // Storage interoperability
    // ---------------------------------------------------------------------

    /// Returns dense elements as a row-major slice when storage is dense.
    #[must_use]
    pub fn as_dense_slice(&self) -> Option<&[T]> {
        self.array.as_dense_slice()
    }

    /// Consumes the tensor and returns its dense elements, if it is dense.
    ///
    /// Sparse and unmaterialized tensors return `None`; this method does not
    /// choose default values or implicitly allocate a dense representation.
    #[must_use]
    pub fn into_dense(self) -> Option<Vec<T>> {
        self.array.into_dense()
    }

    /// Returns sparse entries in ascending flat-index order.
    ///
    /// Dense and unmaterialized tensors return `None`. Sparse iteration is
    /// deterministic because the canonical storage uses ordered keys.
    #[must_use]
    pub fn sparse_entries(
        &self,
    ) -> Option<impl Iterator<Item = (usize, &T)>> {
        self.array.sparse_entries()
    }

    /// Clears sparse entries according to the canonical array contract.
    ///
    /// Dense and unmaterialized storage return the canonical storage error.
    pub fn clear_sparse(&mut self) -> Result<(), ClassicalTensorError> {
        Ok(self.array.clear_sparse()?)
    }
}

// =============================================================================
// Conversions and traits
// =============================================================================

impl<T> From<ClassicalArray<T>> for ClassicalTensor<T> {
    fn from(array: ClassicalArray<T>) -> Self {
        Self::from_array(array)
    }
}

impl<T> From<ClassicalTensor<T>> for ClassicalArray<T> {
    fn from(tensor: ClassicalTensor<T>) -> Self {
        tensor.into_array()
    }
}

impl<T> AsRef<ClassicalArray<T>> for ClassicalTensor<T> {
    fn as_ref(&self) -> &ClassicalArray<T> {
        self.as_array()
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn shape(dimensions: Vec<usize>) -> ClassicalArrayShape {
        ClassicalArrayShape::new(dimensions).unwrap()
    }

    #[test]
    fn creates_tensor_with_arbitrary_rank() {
        let tensor = ClassicalTensor::<u8>::unmaterialized(shape(vec![
            2, 3, 4, 5, 6,
        ]));

        assert_eq!(tensor.rank(), 5);
        assert_eq!(tensor.dimensions(), &[2, 3, 4, 5, 6]);
        assert_eq!(tensor.logical_size(), Ok(720));
        assert_eq!(tensor.stored_len(), 0);
    }

    #[test]
    fn preserves_zero_dimension_metadata() {
        let tensor =
            ClassicalTensor::<u8>::unmaterialized(shape(vec![7, 0, 11]));

        assert_eq!(tensor.rank(), 3);
        assert_eq!(tensor.dimensions(), &[7, 0, 11]);
        assert_eq!(tensor.logical_size(), Ok(0));
        assert!(tensor.is_empty());
    }

    #[test]
    fn dense_tensor_uses_row_major_order() {
        let tensor = ClassicalTensor::from_dense_with_dimensions(
            vec![2, 3, 2],
            (0..12).collect::<Vec<usize>>(),
        )
        .unwrap();

        assert_eq!(tensor.get(&[0, 0, 0]), Ok(Some(&0)));
        assert_eq!(tensor.get(&[0, 1, 0]), Ok(Some(&2)));
        assert_eq!(tensor.get(&[1, 0, 0]), Ok(Some(&6)));
        assert_eq!(tensor.get(&[1, 2, 1]), Ok(Some(&11)));
        assert_eq!(tensor.flat_index(&[1, 2, 1]), Ok(11));
    }

    #[test]
    fn checks_coordinate_rank_and_bounds() {
        let tensor = ClassicalTensor::from_dense_with_dimensions(
            vec![2, 3],
            vec![0, 1, 2, 3, 4, 5],
        )
        .unwrap();

        assert!(matches!(
            tensor.get(&[1]),
            Err(ClassicalTensorError::Array(
                ClassicalArrayError::RankMismatch { .. }
            ))
        ));

        assert!(matches!(
            tensor.get(&[2, 0]),
            Err(ClassicalTensorError::Array(
                ClassicalArrayError::IndexOutOfBounds { .. }
            ))
        ));
    }

    #[test]
    fn rejects_dense_length_mismatch() {
        let result = ClassicalTensor::from_dense_with_dimensions(
            vec![2, 3],
            vec![1, 2, 3],
        );

        assert!(matches!(
            result,
            Err(ClassicalTensorError::Array(
                ClassicalArrayError::DenseLengthMismatch {
                    expected: 6,
                    actual: 3
                }
            ))
        ));
    }

    #[test]
    fn sparse_tensor_stores_only_explicit_entries() {
        let mut tensor =
            ClassicalTensor::<i32>::sparse(shape(vec![100, 100, 100]));

        assert_eq!(tensor.logical_size(), Ok(1_000_000));
        assert_eq!(tensor.stored_len(), 0);
        assert_eq!(tensor.get(&[5, 6, 7]), Ok(None));

        assert_eq!(tensor.set(&[5, 6, 7], 42), Ok(None));
        assert_eq!(tensor.get(&[5, 6, 7]), Ok(Some(&42)));
        assert_eq!(tensor.stored_len(), 1);
    }

    #[test]
    fn unmaterialized_tensor_does_not_implicitly_allocate() {
        let mut tensor =
            ClassicalTensor::<i32>::unmaterialized(shape(vec![2, 3, 4]));

        assert_eq!(
            tensor.storage_kind(),
            ClassicalArrayStorageKind::Unmaterialized
        );
        assert_eq!(tensor.get(&[0, 0, 0]), Ok(None));

        assert!(matches!(
            tensor.set(&[0, 0, 0], 9),
            Err(ClassicalTensorError::Array(
                ClassicalArrayError::StorageNotMaterialized
            ))
        ));
    }

    #[test]
    fn supports_checked_flat_access() {
        let mut tensor =
            ClassicalTensor::from_dense_with_dimensions(vec![2, 2], vec![1, 2, 3, 4])
                .unwrap();

        assert_eq!(tensor.get_flat(2), Ok(Some(&3)));
        assert_eq!(tensor.set_flat(2, 30), Ok(Some(3)));
        assert_eq!(tensor.get_flat(2), Ok(Some(&30)));

        assert!(matches!(
            tensor.get_flat(4),
            Err(ClassicalTensorError::Array(
                ClassicalArrayError::FlatIndexOutOfBounds { .. }
            ))
        ));
    }

    #[test]
    fn flat_index_coordinate_conversion_round_trips() {
        let tensor =
            ClassicalTensor::<u8>::unmaterialized(shape(vec![2, 3, 4]));

        for flat in tensor.flat_indices().unwrap() {
            let coordinates = tensor.coordinates(flat).unwrap();
            assert_eq!(tensor.flat_index(&coordinates), Ok(flat));
        }
    }

    #[test]
    fn generated_dense_tensor_receives_flat_indices() {
        let tensor = ClassicalTensor::try_dense_with_dimensions(
            vec![2, 3],
            |index| index * 10,
        )
        .unwrap();

        assert_eq!(
            tensor.as_dense_slice(),
            Some(&[0, 10, 20, 30, 40, 50][..])
        );
    }

    #[test]
    fn sparse_iteration_is_ordered_and_deterministic() {
        let mut tensor =
            ClassicalTensor::<u32>::sparse(shape(vec![10, 10]));

        tensor.set(&[9, 0], 90).unwrap();
        tensor.set(&[0, 9], 9).unwrap();
        tensor.set(&[5, 5], 55).unwrap();

        let entries = tensor
            .sparse_entries()
            .unwrap()
            .map(|(index, value)| (index, *value))
            .collect::<Vec<_>>();

        assert_eq!(entries, vec![(9, 9), (55, 55), (90, 90)]);
    }

    #[test]
    fn dense_conversion_preserves_values() {
        let tensor = ClassicalTensor::from_dense_with_dimensions(
            vec![2, 2],
            vec![1, 2, 3, 4],
        )
        .unwrap();

        assert_eq!(tensor.into_dense(), Some(vec![1, 2, 3, 4]));
    }

    #[test]
    fn sparse_tensor_is_not_implicitly_densified() {
        let tensor =
            ClassicalTensor::<u32>::sparse(shape(vec![2, 2]));

        assert_eq!(tensor.into_dense(), None);
    }

    #[test]
    fn canonical_array_round_trip_preserves_shape_and_storage() {
        let array = ClassicalArray::sparse(shape(vec![2, 3, 4]));
        let tensor = ClassicalTensor::from(array);
        let array: ClassicalArray<u32> = tensor.into();

        assert_eq!(array.shape().dimensions(), &[2, 3, 4]);
        assert_eq!(array.storage_kind(), ClassicalArrayStorageKind::Sparse);
    }

    #[test]
    fn rejects_rank_zero_shape_through_canonical_contract() {
        assert!(matches!(
            ClassicalTensor::<u8>::unmaterialized_with_dimensions(vec![]),
            Err(ClassicalTensorError::Array(
                ClassicalArrayError::EmptyShape
            ))
        ));
    }

    #[test]
    fn shape_metadata_can_describe_large_logical_tensors_without_storage() {
        let tensor = ClassicalTensor::<u8>::unmaterialized(shape(vec![
            1_000_000, 1_000_000, 1_000,
        ]));

        assert_eq!(tensor.logical_size(), Ok(1_000_000_000_000_000));
        assert_eq!(tensor.stored_len(), 0);
        assert_eq!(
            tensor.storage_kind(),
            ClassicalArrayStorageKind::Unmaterialized
        );
    }

    #[test]
    fn reports_unrepresentable_logical_size() {
        let tensor = ClassicalTensor::<u8>::unmaterialized(shape(vec![
            usize::MAX, 2,
        ]));

        assert!(matches!(
            tensor.logical_size(),
            Err(ClassicalTensorError::Array(
                ClassicalArrayError::ShapeSizeOverflow
            ))
        ));
    }
}
