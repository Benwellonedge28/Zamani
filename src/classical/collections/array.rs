
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani classical collections: multidimensional arrays.
//!
//! Path: src/classical/collections/array.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: safe Rust only.
//!
//! # Purpose
//!
//! Provide a target-independent array container for classical computation.
//! This module owns array shape, checked indexing, and the choice between
//! dense, sparse, and unmaterialized storage.
//!
//! # Ownership
//!
//! This module owns:
//! - `ClassicalArrayShape` and its shape/indexing invariants;
//! - `ClassicalArrayStorage<T>` and its storage invariants;
//! - `ClassicalArray<T>` and its element-access contract;
//! - `ClassicalArrayError` for local failures.
//!
//! This module does not own:
//! - source syntax, AST construction, or parsing;
//! - canonical semantic types or canonical IR values;
//! - numeric operations or implicit conversions;
//! - ownership/borrowing analysis for Zamani source programs;
//! - hardware allocation, placement, or scheduling;
//! - execution of expressions;
//! - global resource or security policy.
//!
//! # POCO-REAF and scalability
//!
//! There is no universal maximum array length, dimension, or rank imposed by
//! this module. Shape metadata is independent of element storage.
//!
//! Logical element counts and flattened indices use `usize`, the host's
//! addressable indexing type. A product that cannot be represented by `usize`
//! is reported as an error when a concrete element count or flat index is
//! required. The dimensions themselves remain available as metadata.
//!
//! This is not a claim of infinite physical memory. Concrete storage remains
//! subject to available resources and the host's representation limits.
//!
//! Dense allocation is explicit and fallible. Sparse storage stores only
//! present entries. Unmaterialized storage records shape without allocating
//! element storage.
//!
//! # Determinism
//!
//! Sparse entries use `BTreeMap`, providing deterministic key ordering.
//! Array shape preserves dimension order. No randomized map iteration is
//! exposed by this module.
//!
//! # Integration contract
//!
//! - `collections/mod.rs` registers and exports this module.
//! - `types/aggregate.rs` remains the owner of aggregate type descriptors.
//! - `values/` and `quantum::ir::classical::value` remain the owners of
//!   canonical classical values.
//! - `quantum::ir::classical::array` remains the canonical IR array boundary.
//! - Semantic analysis determines the element type and source-level legality.
//! - Lowering adapts validated array operations to the canonical IR.
//! - Runtime/resource policy decides whether materialization is permitted.
//! - This module never interprets an array index as a physical address.
//!
//! The container is generic so callers can use canonical value types without
//! this module defining a competing value representation.
//!
//! # Freeze rule
//!
//! Changes to indexing order, empty-dimension semantics, storage behavior,
//! or public error behavior require compatibility review and regression tests.

use core::fmt;
use std::collections::BTreeMap;
use std::ops::Range;

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by classical-array metadata, indexing, and storage APIs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClassicalArrayError {
    /// An array shape must contain at least one dimension.
    EmptyShape,

    /// A multidimensional index has a different rank from the shape.
    RankMismatch {
        /// Number of dimensions in the shape.
        expected: usize,
        /// Number of coordinates supplied.
        actual: usize,
    },

    /// A coordinate is outside its dimension.
    IndexOutOfBounds {
        /// Dimension containing the invalid coordinate.
        dimension: usize,
        /// Requested coordinate.
        index: usize,
        /// Dimension length (exclusive upper bound).
        bound: usize,
    },

    /// The logical element count cannot be represented by `usize`.
    ShapeSizeOverflow,

    /// A flattened-index calculation overflowed `usize`.
    IndexOverflow,

    /// A flat index lies outside the logical array.
    FlatIndexOutOfBounds {
        /// Requested flat index.
        index: usize,
        /// Logical element count.
        size: usize,
    },

    /// A range is invalid for the specified dimension.
    InvalidRange {
        /// Inclusive range start.
        start: usize,
        /// Exclusive range end.
        end: usize,
        /// Dimension length.
        length: usize,
    },

    /// A dense storage vector has the wrong number of elements.
    DenseLengthMismatch {
        /// Number of elements required by the shape.
        expected: usize,
        /// Number of elements supplied.
        actual: usize,
    },

    /// An operation requires storage, but storage is unmaterialized.
    StorageNotMaterialized,

    /// The requested allocation could not reserve the required capacity.
    AllocationFailed {
        /// Number of elements requested.
        elements: usize,
    },
}

impl fmt::Display for ClassicalArrayError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyShape => {
                f.write_str("classical array shape cannot be empty")
            }
            Self::RankMismatch { expected, actual } => {
                write!(
                    f,
                    "array rank mismatch: expected {expected}, received {actual}"
                )
            }
            Self::IndexOutOfBounds {
                dimension,
                index,
                bound,
            } => {
                write!(
                    f,
                    "array coordinate {index} is out of bounds in dimension \
                     {dimension}, whose length is {bound}"
                )
            }
            Self::ShapeSizeOverflow => {
                f.write_str(
                    "array element count exceeds the host index representation",
                )
            }
            Self::IndexOverflow => {
                f.write_str("array flat-index calculation overflowed")
            }
            Self::FlatIndexOutOfBounds { index, size } => {
                write!(
                    f,
                    "flat index {index} is outside array size {size}"
                )
            }
            Self::InvalidRange { start, end, length } => {
                write!(
                    f,
                    "invalid array range {start}..{end} for dimension length {length}"
                )
            }
            Self::DenseLengthMismatch { expected, actual } => {
                write!(
                    f,
                    "dense storage length mismatch: expected {expected}, received {actual}"
                )
            }
            Self::StorageNotMaterialized => {
                f.write_str("array element storage has not been materialized")
            }
            Self::AllocationFailed { elements } => {
                write!(
                    f,
                    "unable to reserve storage for {elements} array elements"
                )
            }
        }
    }
}

impl std::error::Error for ClassicalArrayError {}

// =============================================================================
// Shape
// =============================================================================

/// Target-independent metadata describing an array's logical dimensions.
///
/// Dimensions are ordered from the outermost dimension to the innermost
/// dimension. Flattening uses row-major order.
///
/// A zero-length dimension is valid and gives the array zero logical elements.
/// The shape must contain at least one dimension; scalar values are not arrays.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ClassicalArrayShape {
    dimensions: Vec<usize>,
}

impl ClassicalArrayShape {
    /// Constructs a shape from its ordered dimensions.
    ///
    /// This allocates only shape metadata, not element storage. It rejects
    /// rank zero but permits zero-length dimensions.
    pub fn new(
        dimensions: impl Into<Vec<usize>>,
    ) -> Result<Self, ClassicalArrayError> {
        let dimensions = dimensions.into();

        if dimensions.is_empty() {
            return Err(ClassicalArrayError::EmptyShape);
        }

        Ok(Self { dimensions })
    }

    /// Constructs a one-dimensional shape.
    pub fn one_dimensional(
        length: usize,
    ) -> Result<Self, ClassicalArrayError> {
        Self::new(vec![length])
    }

    /// Returns the number of dimensions.
    #[must_use]
    pub fn rank(&self) -> usize {
        self.dimensions.len()
    }

    /// Returns the dimensions in row-major order.
    #[must_use]
    pub fn dimensions(&self) -> &[usize] {
        &self.dimensions
    }

    /// Returns one dimension's length.
    pub fn dimension(
        &self,
        dimension: usize,
    ) -> Result<usize, ClassicalArrayError> {
        self.dimensions
            .get(dimension)
            .copied()
            .ok_or(ClassicalArrayError::IndexOutOfBounds {
                dimension,
                index: dimension,
                bound: self.rank(),
            })
    }

    /// Returns the logical number of elements, if representable by `usize`.
    ///
    /// This calculation allocates no element storage. A zero dimension makes
    /// the logical size zero even if other dimensions are very large.
    pub fn logical_size(&self) -> Result<usize, ClassicalArrayError> {
        if self.dimensions.contains(&0) {
            return Ok(0);
        }

        self.dimensions.iter().try_fold(1usize, |size, dimension| {
            size.checked_mul(*dimension)
                .ok_or(ClassicalArrayError::ShapeSizeOverflow)
        })
    }

    /// Returns true when the shape describes zero logical elements.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.dimensions.contains(&0)
    }

    /// Validates coordinates and returns their row-major flat index.
    ///
    /// The last dimension varies fastest. No allocation is performed.
    pub fn flatten(
        &self,
        coordinates: &[usize],
    ) -> Result<usize, ClassicalArrayError> {
        if coordinates.len() != self.rank() {
            return Err(ClassicalArrayError::RankMismatch {
                expected: self.rank(),
                actual: coordinates.len(),
            });
        }

        let mut flat = 0usize;

        for (dimension, (&coordinate, &length)) in coordinates
            .iter()
            .zip(self.dimensions.iter())
            .enumerate()
        {
            if coordinate >= length {
                return Err(ClassicalArrayError::IndexOutOfBounds {
                    dimension,
                    index: coordinate,
                    bound: length,
                });
            }

            flat = flat
                .checked_mul(length)
                .and_then(|value| value.checked_add(coordinate))
                .ok_or(ClassicalArrayError::IndexOverflow)?;
        }

        Ok(flat)
    }

    /// Converts a valid row-major flat index to coordinates.
    pub fn coordinates(
        &self,
        flat_index: usize,
    ) -> Result<Vec<usize>, ClassicalArrayError> {
        let size = self.logical_size()?;

        if flat_index >= size {
            return Err(ClassicalArrayError::FlatIndexOutOfBounds {
                index: flat_index,
                size,
            });
        }

        let mut remainder = flat_index;
        let mut coordinates = vec![0; self.rank()];

        for dimension in (0..self.rank()).rev() {
            let length = self.dimensions[dimension];

            // A valid flat index cannot exist if any dimension is zero.
            // The preceding flat-index check establishes that this shape is
            // nonempty and that all divisors below are nonzero.
            coordinates[dimension] = remainder % length;
            remainder /= length;
        }

        Ok(coordinates)
    }

    /// Returns a validated range for a dimension.
    ///
    /// Empty ranges are permitted when `start == end`.
    pub fn validate_range(
        &self,
        dimension: usize,
        range: Range<usize>,
    ) -> Result<Range<usize>, ClassicalArrayError> {
        let length = self.dimension(dimension)?;

        if range.start > range.end || range.end > length {
            return Err(ClassicalArrayError::InvalidRange {
                start: range.start,
                end: range.end,
                length,
            });
        }

        Ok(range)
    }

    /// Produces a new shape by replacing one dimension's length.
    ///
    /// This changes metadata only and does not copy or reshape storage.
    pub fn with_dimension(
        &self,
        dimension: usize,
        length: usize,
    ) -> Result<Self, ClassicalArrayError> {
        if dimension >= self.rank() {
            return Err(ClassicalArrayError::IndexOutOfBounds {
                dimension,
                index: dimension,
                bound: self.rank(),
            });
        }

        let mut dimensions = self.dimensions.clone();
        dimensions[dimension] = length;
        Self::new(dimensions)
    }

    /// Returns a lazy iterator over representable flat indices.
    ///
    /// The iterator requires a representable logical size, but does not
    /// allocate a vector of indices.
    pub fn flat_indices(
        &self,
    ) -> Result<Range<usize>, ClassicalArrayError> {
        Ok(0..self.logical_size()?)
    }
}

// =============================================================================
// Storage
// =============================================================================

/// Concrete storage strategy for a logical classical array.
///
/// `Sparse` stores only explicitly present entries. A missing sparse entry
/// reads as `None`; this type does not invent a default element value.
///
/// `Unmaterialized` contains no element storage and is suitable for shape-only
/// declarations and planning.
///
/// Storage does not own the semantic type of its elements.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClassicalArrayStorage<T> {
    /// Contiguous row-major element storage.
    Dense(Vec<T>),

    /// Deterministically ordered, explicitly present elements by flat index.
    Sparse(BTreeMap<usize, T>),

    /// Shape metadata exists, but no elements have been materialized.
    Unmaterialized,
}

/// Describes a storage strategy without exposing its internal contents.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ClassicalArrayStorageKind {
    /// Contiguous row-major storage.
    Dense,
    /// Ordered sparse storage.
    Sparse,
    /// Shape-only storage.
    Unmaterialized,
}

impl<T> ClassicalArrayStorage<T> {
    /// Returns the storage strategy.
    #[must_use]
    pub fn kind(&self) -> ClassicalArrayStorageKind {
        match self {
            Self::Dense(_) => ClassicalArrayStorageKind::Dense,
            Self::Sparse(_) => ClassicalArrayStorageKind::Sparse,
            Self::Unmaterialized => ClassicalArrayStorageKind::Unmaterialized,
        }
    }

    /// Returns the number of explicitly stored elements.
    ///
    /// This is not necessarily the logical array size: sparse storage can
    /// contain fewer elements than the shape describes.
    #[must_use]
    pub fn stored_len(&self) -> usize {
        match self {
            Self::Dense(values) => values.len(),
            Self::Sparse(values) => values.len(),
            Self::Unmaterialized => 0,
        }
    }

    /// Returns true if this representation has no explicitly stored values.
    #[must_use]
    pub fn has_no_stored_values(&self) -> bool {
        self.stored_len() == 0
    }
}

// =============================================================================
// Array
// =============================================================================

/// A logical classical array with explicit shape and storage.
///
/// Array indexing is always checked. Missing sparse entries and unmaterialized
/// storage return `None` from `get_flat`; callers can distinguish the storage
/// kind when that distinction matters.
///
/// The array is generic over `T` so canonical classical values can be used
/// directly without duplicating their definitions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClassicalArray<T> {
    shape: ClassicalArrayShape,
    storage: ClassicalArrayStorage<T>,
}

impl<T> ClassicalArray<T> {
    /// Creates a shape-only array without allocating element storage.
    pub fn unmaterialized(
        shape: ClassicalArrayShape,
    ) -> Self {
        Self {
            shape,
            storage: ClassicalArrayStorage::Unmaterialized,
        }
    }

    /// Creates an empty sparse array with the specified shape.
    ///
    /// Only the sparse map's initial empty state and shape metadata are
    /// constructed; no element values are allocated.
    pub fn sparse(
        shape: ClassicalArrayShape,
    ) -> Self {
        Self {
            shape,
            storage: ClassicalArrayStorage::Sparse(BTreeMap::new()),
        }
    }

    /// Creates a dense array from row-major values.
    ///
    /// The supplied element count must equal the shape's logical size.
    pub fn from_dense(
        shape: ClassicalArrayShape,
        values: Vec<T>,
    ) -> Result<Self, ClassicalArrayError> {
        let expected = shape.logical_size()?;

        if values.len() != expected {
            return Err(ClassicalArrayError::DenseLengthMismatch {
                expected,
                actual: values.len(),
            });
        }

        Ok(Self {
            shape,
            storage: ClassicalArrayStorage::Dense(values),
        })
    }

    /// Allocates a dense array by calling `make_element` for each position.
    ///
    /// Capacity reservation is fallible. The closure is invoked only after
    /// capacity has been reserved successfully. A closure panic is not
    /// converted into an allocation error.
    ///
    /// Callers should apply explicit resource policies before requesting a
    /// potentially large dense allocation.
    pub fn try_dense_with<F>(
        shape: ClassicalArrayShape,
        mut make_element: F,
    ) -> Result<Self, ClassicalArrayError>
    where
        F: FnMut(usize) -> T,
    {
        let elements = shape.logical_size()?;
        let mut values = Vec::new();

        values
            .try_reserve_exact(elements)
            .map_err(|_| ClassicalArrayError::AllocationFailed { elements })?;

        for index in 0..elements {
            values.push(make_element(index));
        }

        Ok(Self {
            shape,
            storage: ClassicalArrayStorage::Dense(values),
        })
    }

    /// Returns the logical shape.
    #[must_use]
    pub fn shape(&self) -> &ClassicalArrayShape {
        &self.shape
    }

    /// Returns the representable logical element count.
    pub fn logical_size(&self) -> Result<usize, ClassicalArrayError> {
        self.shape.logical_size()
    }

    /// Returns the number of explicitly stored elements.
    #[must_use]
    pub fn stored_len(&self) -> usize {
        self.storage.stored_len()
    }

    /// Returns the current storage strategy.
    #[must_use]
    pub fn storage_kind(&self) -> ClassicalArrayStorageKind {
        self.storage.kind()
    }

    /// Returns a shared reference to the storage representation.
    #[must_use]
    pub fn storage(&self) -> &ClassicalArrayStorage<T> {
        &self.storage
    }

    /// Returns a checked reference to an element by flat index.
    ///
    /// Returns `Ok(None)` for an absent sparse entry or for unmaterialized
    /// storage. Returns an error if the index is outside the logical shape.
    pub fn get_flat(
        &self,
        index: usize,
    ) -> Result<Option<&T>, ClassicalArrayError> {
        self.validate_flat_index(index)?;

        match &self.storage {
            ClassicalArrayStorage::Dense(values) => Ok(values.get(index)),
            ClassicalArrayStorage::Sparse(values) => Ok(values.get(&index)),
            ClassicalArrayStorage::Unmaterialized => Ok(None),
        }
    }

    /// Returns a checked mutable reference to an explicitly stored element.
    ///
    /// An absent sparse entry is not implicitly inserted. Unmaterialized
    /// storage has no elements to mutate.
    pub fn get_flat_mut(
        &mut self,
        index: usize,
    ) -> Result<Option<&mut T>, ClassicalArrayError> {
        self.validate_flat_index(index)?;

        match &mut self.storage {
            ClassicalArrayStorage::Dense(values) => Ok(values.get_mut(index)),
            ClassicalArrayStorage::Sparse(values) => Ok(values.get_mut(&index)),
            ClassicalArrayStorage::Unmaterialized => Ok(None),
        }
    }

    /// Returns a checked reference using multidimensional coordinates.
    pub fn get(
        &self,
        coordinates: &[usize],
    ) -> Result<Option<&T>, ClassicalArrayError> {
        let index = self.shape.flatten(coordinates)?;
        self.get_flat(index)
    }

    /// Inserts or replaces an explicitly stored element by flat index.
    ///
    /// Sparse insertion stores only the supplied element. Dense assignment
    /// replaces an existing element. Unmaterialized storage rejects writes
    /// rather than silently allocating memory.
    pub fn set_flat(
        &mut self,
        index: usize,
        value: T,
    ) -> Result<Option<T>, ClassicalArrayError> {
        self.validate_flat_index(index)?;

        match &mut self.storage {
            ClassicalArrayStorage::Dense(values) => {
                let slot = values.get_mut(index).ok_or(
                    ClassicalArrayError::FlatIndexOutOfBounds {
                        index,
                        size: values.len(),
                    },
                )?;
                Ok(Some(core::mem::replace(slot, value)))
            }
            ClassicalArrayStorage::Sparse(values) => {
                Ok(values.insert(index, value))
            }
            ClassicalArrayStorage::Unmaterialized => {
                Err(ClassicalArrayError::StorageNotMaterialized)
            }
        }
    }

    /// Inserts or replaces an element using multidimensional coordinates.
    pub fn set(
        &mut self,
        coordinates: &[usize],
        value: T,
    ) -> Result<Option<T>, ClassicalArrayError> {
        let index = self.shape.flatten(coordinates)?;
        self.set_flat(index, value)
    }

    /// Removes an explicitly stored element.
    ///
    /// For dense storage, removal would shift later elements and invalidate
    /// row-major positions, so this method returns `None` without mutation.
    /// For sparse storage, the entry is removed if present.
    pub fn remove_flat(
        &mut self,
        index: usize,
    ) -> Result<Option<T>, ClassicalArrayError> {
        self.validate_flat_index(index)?;

        match &mut self.storage {
            ClassicalArrayStorage::Dense(_) => Ok(None),
            ClassicalArrayStorage::Sparse(values) => Ok(values.remove(&index)),
            ClassicalArrayStorage::Unmaterialized => {
                Err(ClassicalArrayError::StorageNotMaterialized)
            }
        }
    }

    /// Clears explicitly stored sparse entries.
    ///
    /// Dense storage is retained because clearing it would require a
    /// value-specific default. Unmaterialized storage remains unchanged.
    pub fn clear_sparse(&mut self) -> Result<(), ClassicalArrayError> {
        match &mut self.storage {
            ClassicalArrayStorage::Sparse(values) => {
                values.clear();
                Ok(())
            }
            ClassicalArrayStorage::Dense(_) => {
                Err(ClassicalArrayError::StorageNotMaterialized)
            }
            ClassicalArrayStorage::Unmaterialized => {
                Err(ClassicalArrayError::StorageNotMaterialized)
            }
        }
    }

    /// Returns the sparse entries in ascending flat-index order.
    ///
    /// Dense and unmaterialized storage return `None`, because neither is
    /// represented by a sparse index map.
    #[must_use]
    pub fn sparse_entries(
        &self,
    ) -> Option<impl Iterator<Item = (usize, &T)>> {
        match &self.storage {
            ClassicalArrayStorage::Sparse(values) => {
                Some(values.iter().map(|(&index, value)| (index, value)))
            }
            ClassicalArrayStorage::Dense(_) |
            ClassicalArrayStorage::Unmaterialized => None,
        }
    }

    /// Returns dense values as a slice when the array is densely stored.
    #[must_use]
    pub fn as_dense_slice(&self) -> Option<&[T]> {
        match &self.storage {
            ClassicalArrayStorage::Dense(values) => Some(values.as_slice()),
            ClassicalArrayStorage::Sparse(_) |
            ClassicalArrayStorage::Unmaterialized => None,
        }
    }

    /// Consumes the array and returns its dense vector, if present.
    #[must_use]
    pub fn into_dense(self) -> Option<Vec<T>> {
        match self.storage {
            ClassicalArrayStorage::Dense(values) => Some(values),
            ClassicalArrayStorage::Sparse(_) |
            ClassicalArrayStorage::Unmaterialized => None,
        }
    }

    fn validate_flat_index(
        &self,
        index: usize,
    ) -> Result<(), ClassicalArrayError> {
        let size = self.shape.logical_size()?;

        if index < size {
            Ok(())
        } else {
            Err(ClassicalArrayError::FlatIndexOutOfBounds {
                index,
                size,
            })
        }
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_rank_zero_shape() {
        assert_eq!(
            ClassicalArrayShape::new(Vec::new()),
            Err(ClassicalArrayError::EmptyShape)
        );
    }

    #[test]
    fn permits_zero_length_dimensions() {
        let shape = ClassicalArrayShape::new(vec![3, 0, 7]).unwrap();

        assert_eq!(shape.rank(), 3);
        assert_eq!(shape.logical_size(), Ok(0));
        assert!(shape.is_empty());
        assert_eq!(shape.flat_indices(), Ok(0..0));
    }

    #[test]
    fn computes_row_major_flat_indices() {
        let shape = ClassicalArrayShape::new(vec![2, 3, 4]).unwrap();

        assert_eq!(shape.flatten(&[0, 0, 0]), Ok(0));
        assert_eq!(shape.flatten(&[0, 1, 0]), Ok(4));
        assert_eq!(shape.flatten(&[1, 0, 0]), Ok(12));
        assert_eq!(shape.flatten(&[1, 2, 3]), Ok(23));
    }

    #[test]
    fn round_trips_flat_indices_and_coordinates() {
        let shape = ClassicalArrayShape::new(vec![2, 3, 4]).unwrap();

        for index in shape.flat_indices().unwrap() {
            let coordinates = shape.coordinates(index).unwrap();
            assert_eq!(shape.flatten(&coordinates), Ok(index));
        }
    }

    #[test]
    fn rejects_rank_mismatch() {
        let shape = ClassicalArrayShape::new(vec![2, 3]).unwrap();

        assert_eq!(
            shape.flatten(&[1]),
            Err(ClassicalArrayError::RankMismatch {
                expected: 2,
                actual: 1,
            })
        );
    }

    #[test]
    fn rejects_out_of_bounds_coordinates() {
        let shape = ClassicalArrayShape::new(vec![2, 3]).unwrap();

        assert_eq!(
            shape.flatten(&[2, 0]),
            Err(ClassicalArrayError::IndexOutOfBounds {
                dimension: 0,
                index: 2,
                bound: 2,
            })
        );
    }

    #[test]
    fn rejects_unrepresentable_logical_size() {
        let shape =
            ClassicalArrayShape::new(vec![usize::MAX, 2]).unwrap();

        assert_eq!(
            shape.logical_size(),
            Err(ClassicalArrayError::ShapeSizeOverflow)
        );
    }

    #[test]
    fn shape_metadata_does_not_materialize_values() {
        let shape = ClassicalArrayShape::new(vec![1_000_000, 1_000_000])
            .unwrap();
        let array = ClassicalArray::<u8>::unmaterialized(shape);

        assert_eq!(array.logical_size(), Ok(1_000_000_000_000));
        assert_eq!(array.stored_len(), 0);
        assert_eq!(
            array.storage_kind(),
            ClassicalArrayStorageKind::Unmaterialized
        );
    }

    #[test]
    fn sparse_array_stores_only_present_elements() {
        let shape = ClassicalArrayShape::new(vec![10, 10]).unwrap();
        let mut array = ClassicalArray::sparse(shape);

        assert_eq!(array.set(&[2, 3], 42), Ok(None));
        assert_eq!(array.get(&[2, 3]), Ok(Some(&42)));
        assert_eq!(array.get(&[0, 0]), Ok(None));
        assert_eq!(array.stored_len(), 1);
    }

    #[test]
    fn sparse_entries_are_deterministically_ordered() {
        let shape = ClassicalArrayShape::one_dimensional(10).unwrap();
        let mut array = ClassicalArray::sparse(shape);

        array.set_flat(7, "seven").unwrap();
        array.set_flat(2, "two").unwrap();
        array.set_flat(5, "five").unwrap();

        let entries: Vec<_> = array
            .sparse_entries()
            .unwrap()
            .map(|(index, value)| (index, *value))
            .collect();

        assert_eq!(
            entries,
            vec![(2, "two"), (5, "five"), (7, "seven")]
        );
    }

    #[test]
    fn dense_storage_requires_exact_length() {
        let shape = ClassicalArrayShape::new(vec![2, 2]).unwrap();

        assert_eq!(
            ClassicalArray::from_dense(shape, vec![1, 2, 3]),
            Err(ClassicalArrayError::DenseLengthMismatch {
                expected: 4,
                actual: 3,
            })
        );
    }

    #[test]
    fn dense_access_and_replacement_are_checked() {
        let shape = ClassicalArrayShape::new(vec![2, 2]).unwrap();
        let mut array =
            ClassicalArray::from_dense(shape, vec![10, 20, 30, 40])
                .unwrap();

        assert_eq!(array.get(&[1, 0]), Ok(Some(&30)));
        assert_eq!(array.set(&[1, 0], 99), Ok(Some(30)));
        assert_eq!(array.get_flat(2), Ok(Some(&99)));
        assert_eq!(
            array.get_flat(4),
            Err(ClassicalArrayError::FlatIndexOutOfBounds {
                index: 4,
                size: 4,
            })
        );
    }

    #[test]
    fn unmaterialized_storage_rejects_writes() {
        let shape = ClassicalArrayShape::one_dimensional(5).unwrap();
        let mut array = ClassicalArray::<u32>::unmaterialized(shape);

        assert_eq!(
            array.set_flat(0, 1),
            Err(ClassicalArrayError::StorageNotMaterialized)
        );
    }

    #[test]
    fn sparse_remove_and_clear_preserve_shape() {
        let shape = ClassicalArrayShape::one_dimensional(5).unwrap();
        let mut array = ClassicalArray::sparse(shape);

        array.set_flat(3, 30).unwrap();
        assert_eq!(array.remove_flat(3), Ok(Some(30)));
        assert_eq!(array.stored_len(), 0);

        array.set_flat(1, 10).unwrap();
        array.clear_sparse().unwrap();

        assert_eq!(array.stored_len(), 0);
        assert_eq!(array.logical_size(), Ok(5));
    }

    #[test]
    fn validates_dimension_ranges() {
        let shape = ClassicalArrayShape::new(vec![4, 8]).unwrap();

        assert_eq!(shape.validate_range(1, 2..7), Ok(2..7));
        assert_eq!(
            shape.validate_range(1, 7..2),
            Err(ClassicalArrayError::InvalidRange {
                start: 7,
                end: 2,
                length: 8,
            })
        );
    }

    #[test]
    fn dense_factory_handles_empty_shapes() {
        let shape = ClassicalArrayShape::new(vec![0, 4]).unwrap();
        let array = ClassicalArray::<u8>::try_dense_with(shape, |_| 1)
            .unwrap();

        assert_eq!(array.as_dense_slice(), Some([].as_slice()));
        assert_eq!(array.logical_size(), Ok(0));
    }
}
