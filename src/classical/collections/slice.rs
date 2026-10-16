
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Safe, borrowed classical collection views for Zamani.
//!
//! Path: src/classical/collections/slice.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: unsafe Rust is forbidden.
//!
//! # Purpose
//!
//! Provide immutable and mutable borrowed views over existing contiguous
//! storage. A view may describe a multidimensional region with non-unit
//! strides without copying the underlying elements.
//!
//! # Ownership contract
//!
//! This module owns `ClassicalSlice`, `ClassicalSliceMut`, and
//! `ClassicalSliceError`.
//!
//! It does not own array storage, shape semantics, element types, source
//! syntax, AST nodes, semantic analysis, canonical IR, numerical operations,
//! hardware placement, scheduling, or execution.
//!
//! `ClassicalArrayShape` and `ClassicalArrayError` are reused from `array.rs`.
//! The array module remains the compatibility entry point for these types
//! until shape ownership is migrated and re-exported from `shape.rs`.
//!
//! # Semantics
//!
//! - Views borrow existing storage and never allocate element storage.
//! - Dimensions and indexing follow the shared row-major shape contract.
//! - A view's strides are measured in elements, not bytes.
//! - A zero-length dimension is valid.
//! - Rank zero is rejected by the shared array-shape contract.
//! - Slice ranges are half-open: `start..end`.
//! - Slice steps must be greater than zero.
//! - Nested views preserve the parent's strides and backing allocation.
//! - Mutable views require an exclusive Rust borrow, preventing safe aliasing.
//! - Indexing and offset arithmetic are checked.
//!
//! # POCO-REAF
//!
//! No universal maximum rank, dimension, or slice length is imposed.
//! Concrete views are bounded by the representable host slice and available
//! resources. A view does not imply that a logical collection can be
//! allocated or materialized.
//!
//! Symbolic source-level dimensions must be resolved by semantic analysis
//! before construction of these concrete runtime views.
//!
//! # Integration
//!
//! - `array.rs` owns `ClassicalArray<T>` and its storage.
//! - `array.rs` / `shape.rs` own the shared `ClassicalArrayShape` contract.
//! - `vector.rs`, `matrix.rs`, and `tensor.rs` may consume these view APIs.
//! - Grammar delegates own slice syntax only.
//! - Frontend AST and semantic analysis own source-level slicing meaning,
//!   type checks, borrow checks, and lifetime checks.
//! - Canonical classical IR owns the compiler's canonical representation.
//! - Lowering may use these runtime views only where the target/runtime
//!   representation supports equivalent semantics.
//!
//! # Freeze rule
//!
//! Changes to range semantics, step semantics, strides, aliasing guarantees,
//! or public errors require compatibility review and regression tests.

use core::fmt;
use core::ops::Range;

use super::array::{
    ClassicalArray,
    ClassicalArrayError,
    ClassicalArrayShape,
};

/// Errors produced by safe classical slice views.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClassicalSliceError {
    /// An underlying array or shape operation failed.
    Array(ClassicalArrayError),

    /// The number of backing elements differs from the shape's logical size.
    BackingLengthMismatch {
        /// Number of elements required by the shape.
        expected: usize,
        /// Number of elements supplied by the backing slice.
        actual: usize,
    },

    /// A slice step must be greater than zero.
    ZeroStep,

    /// Calculating a physical element offset overflowed `usize`.
    OffsetOverflow,

    /// Calculating a physical stride overflowed `usize`.
    StrideOverflow,

    /// The host could not reserve metadata for a view.
    MetadataAllocationFailed {
        /// Number of metadata entries requested.
        entries: usize,
    },

    /// A requested operation requires a contiguous view.
    NotContiguous,
}

impl fmt::Display for ClassicalSliceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Array(error) => write!(f, "{error}"),
            Self::BackingLengthMismatch { expected, actual } => write!(
                f,
                "slice backing length mismatch: expected {expected}, received {actual}"
            ),
            Self::ZeroStep => {
                f.write_str("slice step must be greater than zero")
            }
            Self::OffsetOverflow => {
                f.write_str("slice physical offset overflowed")
            }
            Self::StrideOverflow => {
                f.write_str("slice physical stride overflowed")
            }
            Self::MetadataAllocationFailed { entries } => write!(
                f,
                "unable to reserve metadata for {entries} slice entries"
            ),
            Self::NotContiguous => {
                f.write_str("operation requires contiguous slice storage")
            }
        }
    }
}

impl std::error::Error for ClassicalSliceError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Array(error) => Some(error),
            _ => None,
        }
    }
}

impl From<ClassicalArrayError> for ClassicalSliceError {
    fn from(value: ClassicalArrayError) -> Self {
        Self::Array(value)
    }
}

/// Immutable borrowed view of a row-major collection.
///
/// `data` is the entire backing slice. `offset` identifies the first logical
/// element's physical offset in that slice. `strides` maps logical coordinates
/// to physical element offsets.
///
/// Strides and offsets are measured in elements, not bytes.
#[derive(Debug, Clone)]
pub struct ClassicalSlice<'a, T> {
    data: &'a [T],
    shape: ClassicalArrayShape,
    strides: Vec<usize>,
    offset: usize,
}

/// Mutable borrowed view of a row-major collection.
///
/// The exclusive borrow prevents other safe Rust references from mutating or
/// observing the same backing elements while this view is alive.
#[derive(Debug)]
pub struct ClassicalSliceMut<'a, T> {
    data: &'a mut [T],
    shape: ClassicalArrayShape,
    strides: Vec<usize>,
    offset: usize,
}

impl<'a, T> ClassicalSlice<'a, T> {
    /// Creates a contiguous immutable view.
    ///
    /// The shape's logical element count must be representable and must equal
    /// `data.len()`. No element storage is allocated.
    pub fn new(
        data: &'a [T],
        shape: ClassicalArrayShape,
    ) -> Result<Self, ClassicalSliceError> {
        let expected = shape.logical_size()?;

        if expected != data.len() {
            return Err(ClassicalSliceError::BackingLengthMismatch {
                expected,
                actual: data.len(),
            });
        }

        let strides = contiguous_strides(&shape)?;

        Ok(Self {
            data,
            shape,
            strides,
            offset: 0,
        })
    }

    /// Creates an immutable view over a densely stored classical array.
    ///
    /// Sparse and unmaterialized arrays cannot be borrowed as a contiguous
    /// slice without changing their storage semantics.
    pub fn from_array(
        array: &'a ClassicalArray<T>,
    ) -> Result<Self, ClassicalSliceError> {
        let data = array
            .as_dense_slice()
            .ok_or(ClassicalSliceError::Array(
                ClassicalArrayError::StorageNotMaterialized,
            ))?;

        let shape = clone_shape(array.shape())?;
        Self::new(data, shape)
    }

    /// Returns the logical shape of this view.
    #[must_use]
    pub fn shape(&self) -> &ClassicalArrayShape {
        &self.shape
    }

    /// Returns the rank of this view.
    #[must_use]
    pub fn rank(&self) -> usize {
        self.shape.rank()
    }

    /// Returns the logical element count, if representable.
    pub fn logical_size(&self) -> Result<usize, ClassicalSliceError> {
        Ok(self.shape.logical_size()?)
    }

    /// Returns true when this view contains zero logical elements.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.shape.is_empty()
    }

    /// Returns the element stride for each dimension.
    ///
    /// Strides are measured in elements. This metadata is read-only.
    #[must_use]
    pub fn strides(&self) -> &[usize] {
        &self.strides
    }

    /// Returns a checked element reference by multidimensional coordinates.
    pub fn get(
        &self,
        coordinates: &[usize],
    ) -> Result<Option<&'a T>, ClassicalSliceError> {
        // The shared shape performs rank and coordinate-bound validation.
        self.shape.flatten(coordinates)?;

        let physical = physical_offset(
            self.offset,
            coordinates,
            &self.strides,
        )?;

        self.data
            .get(physical)
            .map(Some)
            .ok_or(ClassicalSliceError::OffsetOverflow)
    }

    /// Returns a checked element reference by row-major logical index.
    pub fn get_flat(
        &self,
        index: usize,
    ) -> Result<Option<&'a T>, ClassicalSliceError> {
        let coordinates = coordinates_for_flat(&self.shape, index)?;
        self.get(&coordinates)
    }

    /// Returns a contiguous borrowed slice when this view is contiguous.
    ///
    /// A strided view returns `NotContiguous`; it is never silently copied.
    pub fn as_contiguous_slice(
        &self,
    ) -> Result<&'a [T], ClassicalSliceError> {
        if !is_contiguous(&self.shape, &self.strides)? {
            return Err(ClassicalSliceError::NotContiguous);
        }

        let length = self.shape.logical_size()?;
        let end = self
            .offset
            .checked_add(length)
            .ok_or(ClassicalSliceError::OffsetOverflow)?;

        self.data
            .get(self.offset..end)
            .ok_or(ClassicalSliceError::OffsetOverflow)
    }

    /// Returns a borrowed view of a subrange of one dimension.
    ///
    /// `range` is half-open and `step` must be nonzero. The result borrows the
    /// same backing storage and composes its offset and strides with this view.
    ///
    /// The result can outlive neither this view nor its original backing data.
    pub fn slice(
        &self,
        dimension: usize,
        range: Range<usize>,
        step: usize,
    ) -> Result<Self, ClassicalSliceError> {
        let (shape, strides, offset) = sliced_metadata(
            &self.shape,
            &self.strides,
            self.offset,
            dimension,
            range,
            step,
        )?;

        Ok(Self {
            data: self.data,
            shape,
            strides,
            offset,
        })
    }

    /// Copies logical elements in row-major view order.
    ///
    /// This is an explicit copying operation, unlike creating a view.
    /// The result allocation is fallible.
    pub fn to_vec(&self) -> Result<Vec<T>, ClassicalSliceError>
    where
        T: Clone,
    {
        let length = self.shape.logical_size()?;
        let mut result = Vec::new();

        result
            .try_reserve_exact(length)
            .map_err(|_| ClassicalSliceError::MetadataAllocationFailed {
                entries: length,
            })?;

        for index in 0..length {
            let value = self
                .get_flat(index)?
                .ok_or(ClassicalSliceError::OffsetOverflow)?;
            result.push(value.clone());
        }

        Ok(result)
    }
}

impl<'a, T> ClassicalSliceMut<'a, T> {
    /// Creates a contiguous mutable view.
    ///
    /// The shape's logical element count must equal `data.len()`.
    pub fn new(
        data: &'a mut [T],
        shape: ClassicalArrayShape,
    ) -> Result<Self, ClassicalSliceError> {
        let expected = shape.logical_size()?;

        if expected != data.len() {
            return Err(ClassicalSliceError::BackingLengthMismatch {
                expected,
                actual: data.len(),
            });
        }

        let strides = contiguous_strides(&shape)?;

        Ok(Self {
            data,
            shape,
            strides,
            offset: 0,
        })
    }

    /// Returns the logical shape of this view.
    #[must_use]
    pub fn shape(&self) -> &ClassicalArrayShape {
        &self.shape
    }

    /// Returns the logical element count, if representable.
    pub fn logical_size(&self) -> Result<usize, ClassicalSliceError> {
        Ok(self.shape.logical_size()?)
    }

    /// Returns true when this view contains zero logical elements.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.shape.is_empty()
    }

    /// Returns the element stride for each dimension.
    #[must_use]
    pub fn strides(&self) -> &[usize] {
        &self.strides
    }

    /// Returns a checked immutable reference by coordinates.
    pub fn get(
        &self,
        coordinates: &[usize],
    ) -> Result<Option<&T>, ClassicalSliceError> {
        self.shape.flatten(coordinates)?;

        let physical = physical_offset(
            self.offset,
            coordinates,
            &self.strides,
        )?;

        self.data
            .get(physical)
            .map(Some)
            .ok_or(ClassicalSliceError::OffsetOverflow)
    }

    /// Returns a checked mutable reference by coordinates.
    pub fn get_mut(
        &mut self,
        coordinates: &[usize],
    ) -> Result<Option<&mut T>, ClassicalSliceError> {
        self.shape.flatten(coordinates)?;

        let physical = physical_offset(
            self.offset,
            coordinates,
            &self.strides,
        )?;

        self.data
            .get_mut(physical)
            .map(Some)
            .ok_or(ClassicalSliceError::OffsetOverflow)
    }

    /// Replaces an element at the given coordinates.
    pub fn set(
        &mut self,
        coordinates: &[usize],
        value: T,
    ) -> Result<T, ClassicalSliceError> {
        let slot = self
            .get_mut(coordinates)?
            .ok_or(ClassicalSliceError::OffsetOverflow)?;

        Ok(core::mem::replace(slot, value))
    }

    /// Returns a checked immutable reference by row-major logical index.
    pub fn get_flat(
        &self,
        index: usize,
    ) -> Result<Option<&T>, ClassicalSliceError> {
        let coordinates = coordinates_for_flat(&self.shape, index)?;
        self.get(&coordinates)
    }

    /// Returns a checked mutable reference by row-major logical index.
    pub fn get_flat_mut(
        &mut self,
        index: usize,
    ) -> Result<Option<&mut T>, ClassicalSliceError> {
        let coordinates = coordinates_for_flat(&self.shape, index)?;
        self.get_mut(&coordinates)
    }

    /// Replaces an element by row-major logical index.
    pub fn set_flat(
        &mut self,
        index: usize,
        value: T,
    ) -> Result<T, ClassicalSliceError> {
        let coordinates = coordinates_for_flat(&self.shape, index)?;
        self.set(&coordinates, value)
    }

    /// Returns a mutable borrowed view of a subrange of one dimension.
    ///
    /// The returned view exclusively borrows this view's backing data for the
    /// duration of the child borrow. Safe Rust therefore prevents concurrent
    /// mutable use of the parent while the child view remains alive.
    pub fn slice(
        &mut self,
        dimension: usize,
        range: Range<usize>,
        step: usize,
    ) -> Result<ClassicalSliceMut<'_, T>, ClassicalSliceError> {
        let (shape, strides, offset) = sliced_metadata(
            &self.shape,
            &self.strides,
            self.offset,
            dimension,
            range,
            step,
        )?;

        Ok(ClassicalSliceMut {
            data: self.data,
            shape,
            strides,
            offset,
        })
    }

    /// Returns a contiguous borrowed slice when this view is contiguous.
    pub fn as_contiguous_slice(
        &self,
    ) -> Result<&[T], ClassicalSliceError> {
        if !is_contiguous(&self.shape, &self.strides)? {
            return Err(ClassicalSliceError::NotContiguous);
        }

        let length = self.shape.logical_size()?;
        let end = self
            .offset
            .checked_add(length)
            .ok_or(ClassicalSliceError::OffsetOverflow)?;

        self.data
            .get(self.offset..end)
            .ok_or(ClassicalSliceError::OffsetOverflow)
    }

    /// Returns a mutable contiguous slice when this view is contiguous.
    pub fn as_contiguous_slice_mut(
        &mut self,
    ) -> Result<&mut [T], ClassicalSliceError> {
        if !is_contiguous(&self.shape, &self.strides)? {
            return Err(ClassicalSliceError::NotContiguous);
        }

        let length = self.shape.logical_size()?;
        let end = self
            .offset
            .checked_add(length)
            .ok_or(ClassicalSliceError::OffsetOverflow)?;

        self.data
            .get_mut(self.offset..end)
            .ok_or(ClassicalSliceError::OffsetOverflow)
    }
}

// =============================================================================
// Shared checked metadata helpers
// =============================================================================

fn clone_shape(
    shape: &ClassicalArrayShape,
) -> Result<ClassicalArrayShape, ClassicalSliceError> {
    let dimensions = shape.dimensions();
    let mut copied = Vec::new();

    copied
        .try_reserve_exact(dimensions.len())
        .map_err(|_| ClassicalSliceError::MetadataAllocationFailed {
            entries: dimensions.len(),
        })?;

    copied.extend_from_slice(dimensions);
    Ok(ClassicalArrayShape::new(copied)?)
}

fn contiguous_strides(
    shape: &ClassicalArrayShape,
) -> Result<Vec<usize>, ClassicalSliceError> {
    let rank = shape.rank();
    let mut strides = Vec::new();

    strides
        .try_reserve_exact(rank)
        .map_err(|_| ClassicalSliceError::MetadataAllocationFailed {
            entries: rank,
        })?;
    strides.resize(rank, 0);

    // Empty views have no valid element coordinates. Zero strides avoid
    // requiring irrelevant products to fit when another dimension is zero.
    if shape.is_empty() {
        return Ok(strides);
    }

    let mut stride = 1usize;

    for dimension in (0..rank).rev() {
        strides[dimension] = stride;
        stride = stride
            .checked_mul(shape.dimensions()[dimension])
            .ok_or(ClassicalSliceError::StrideOverflow)?;
    }

    Ok(strides)
}

fn coordinates_for_flat(
    shape: &ClassicalArrayShape,
    flat_index: usize,
) -> Result<Vec<usize>, ClassicalSliceError> {
    // Delegate all bounds and size-overflow semantics to the shared shape.
    Ok(shape.coordinates(flat_index)?)
}

fn physical_offset(
    base_offset: usize,
    coordinates: &[usize],
    strides: &[usize],
) -> Result<usize, ClassicalSliceError> {
    if coordinates.len() != strides.len() {
        return Err(ClassicalSliceError::Array(
            ClassicalArrayError::RankMismatch {
                expected: strides.len(),
                actual: coordinates.len(),
            },
        ));
    }

    let mut offset = base_offset;

    for (&coordinate, &stride) in coordinates.iter().zip(strides.iter()) {
        let contribution = coordinate
            .checked_mul(stride)
            .ok_or(ClassicalSliceError::OffsetOverflow)?;

        offset = offset
            .checked_add(contribution)
            .ok_or(ClassicalSliceError::OffsetOverflow)?;
    }

    Ok(offset)
}

fn sliced_metadata(
    parent_shape: &ClassicalArrayShape,
    parent_strides: &[usize],
    parent_offset: usize,
    dimension: usize,
    range: Range<usize>,
    step: usize,
) -> Result<(ClassicalArrayShape, Vec<usize>, usize), ClassicalSliceError> {
    if step == 0 {
        return Err(ClassicalSliceError::ZeroStep);
    }

    let dimension_length = parent_shape.dimension(dimension)?;

    if range.start > range.end || range.end > dimension_length {
        return Err(ClassicalSliceError::Array(
            ClassicalArrayError::InvalidRange {
                start: range.start,
                end: range.end,
                length: dimension_length,
            },
        ));
    }

    if parent_strides.len() != parent_shape.rank() {
        return Err(ClassicalSliceError::Array(
            ClassicalArrayError::RankMismatch {
                expected: parent_shape.rank(),
                actual: parent_strides.len(),
            },
        ));
    }

    let selected_length = if range.start == range.end {
        0
    } else {
        // ceil((end - start) / step), without an overflowing addition.
        let distance = range.end - range.start;
        1 + (distance - 1) / step
    };

    let mut dimensions = Vec::new();
    dimensions
        .try_reserve_exact(parent_shape.rank())
        .map_err(|_| ClassicalSliceError::MetadataAllocationFailed {
            entries: parent_shape.rank(),
        })?;
    dimensions.extend_from_slice(parent_shape.dimensions());
    dimensions[dimension] = selected_length;

    let shape = ClassicalArrayShape::new(dimensions)?;

    let mut strides = Vec::new();
    strides
        .try_reserve_exact(parent_strides.len())
        .map_err(|_| ClassicalSliceError::MetadataAllocationFailed {
            entries: parent_strides.len(),
        })?;
    strides.extend_from_slice(parent_strides);

    strides[dimension] = parent_strides[dimension]
        .checked_mul(step)
        .ok_or(ClassicalSliceError::StrideOverflow)?;

    let delta = range.start
        .checked_mul(parent_strides[dimension])
        .ok_or(ClassicalSliceError::OffsetOverflow)?;

    let offset = parent_offset
        .checked_add(delta)
        .ok_or(ClassicalSliceError::OffsetOverflow)?;

    Ok((shape, strides, offset))
}

fn is_contiguous(
    shape: &ClassicalArrayShape,
    strides: &[usize],
) -> Result<bool, ClassicalSliceError> {
    if strides.len() != shape.rank() {
        return Err(ClassicalSliceError::Array(
            ClassicalArrayError::RankMismatch {
                expected: shape.rank(),
                actual: strides.len(),
            },
        ));
    }

    if shape.is_empty() {
        return Ok(true);
    }

    let mut expected_stride = 1usize;

    for dimension in (0..shape.rank()).rev() {
        if strides[dimension] != expected_stride {
            return Ok(false);
        }

        expected_stride = expected_stride
            .checked_mul(shape.dimensions()[dimension])
            .ok_or(ClassicalSliceError::StrideOverflow)?;
    }

    Ok(true)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn shape(dimensions: &[usize]) -> ClassicalArrayShape {
        ClassicalArrayShape::new(dimensions.to_vec())
            .expect("valid test shape")
    }

    #[test]
    fn creates_a_contiguous_view_without_copying() {
        let data = [10, 20, 30, 40];
        let view = ClassicalSlice::new(&data, shape(&[4]))
            .expect("valid view");

        assert_eq!(view.logical_size(), Ok(4));
        assert_eq!(view.get(&[0]), Ok(Some(&10)));
        assert_eq!(view.get(&[3]), Ok(Some(&40)));
        assert_eq!(view.as_contiguous_slice(), Ok(&data[..]));
    }

    #[test]
    fn rejects_backing_length_mismatch() {
        let data = [1, 2, 3];

        assert_eq!(
            ClassicalSlice::new(&data, shape(&[2, 2])).err(),
            Some(ClassicalSliceError::BackingLengthMismatch {
                expected: 4,
                actual: 3,
            })
        );
    }

    #[test]
    fn indexes_multidimensional_views_in_row_major_order() {
        let data = [0, 1, 2, 3, 4, 5];
        let view = ClassicalSlice::new(&data, shape(&[2, 3]))
            .expect("valid view");

        assert_eq!(view.get(&[0, 2]), Ok(Some(&2)));
        assert_eq!(view.get(&[1, 0]), Ok(Some(&3)));
        assert_eq!(view.get_flat(5), Ok(Some(&5)));
    }

    #[test]
    fn rejects_invalid_coordinates() {
        let data = [0, 1, 2, 3];
        let view = ClassicalSlice::new(&data, shape(&[2, 2]))
            .expect("valid view");

        assert!(matches!(
            view.get(&[2, 0]),
            Err(ClassicalSliceError::Array(
                ClassicalArrayError::IndexOutOfBounds { .. }
            ))
        ));
    }

    #[test]
    fn creates_a_contiguous_subview() {
        let data = [10, 20, 30, 40, 50];
        let view = ClassicalSlice::new(&data, shape(&[5]))
            .expect("valid view");
        let subview = view.slice(0, 1..4, 1).expect("valid subview");

        assert_eq!(subview.shape().dimensions(), &[3]);
        assert_eq!(subview.get_flat(0), Ok(Some(&20)));
        assert_eq!(subview.get_flat(2), Ok(Some(&40)));
        assert_eq!(subview.as_contiguous_slice(), Ok(&data[1..4]));
    }

    #[test]
    fn creates_a_strided_subview() {
        let data = [0, 1, 2, 3, 4, 5, 6, 7];
        let view = ClassicalSlice::new(&data, shape(&[8]))
            .expect("valid view");
        let subview = view.slice(0, 1..8, 2).expect("valid strided view");

        assert_eq!(subview.shape().dimensions(), &[4]);
        assert_eq!(subview.get_flat(0), Ok(Some(&1)));
        assert_eq!(subview.get_flat(1), Ok(Some(&3)));
        assert_eq!(subview.get_flat(3), Ok(Some(&7)));
        assert_eq!(
            subview.as_contiguous_slice().err(),
            Some(ClassicalSliceError::NotContiguous)
        );
    }

    #[test]
    fn slices_one_dimension_of_a_matrix() {
        let data = [0, 1, 2, 3, 4, 5];
        let view = ClassicalSlice::new(&data, shape(&[2, 3]))
            .expect("valid matrix view");
        let columns = view.slice(1, 1..3, 1).expect("valid column view");

        assert_eq!(columns.shape().dimensions(), &[2, 2]);
        assert_eq!(columns.get(&[0, 0]), Ok(Some(&1)));
        assert_eq!(columns.get(&[0, 1]), Ok(Some(&2)));
        assert_eq!(columns.get(&[1, 0]), Ok(Some(&4)));
        assert_eq!(columns.get(&[1, 1]), Ok(Some(&5)));
    }

    #[test]
    fn rejects_zero_step() {
        let data = [1, 2, 3];
        let view = ClassicalSlice::new(&data, shape(&[3]))
            .expect("valid view");

        assert_eq!(
            view.slice(0, 0..3, 0).err(),
            Some(ClassicalSliceError::ZeroStep)
        );
    }

    #[test]
    fn permits_empty_slices() {
        let data = [1, 2, 3];
        let view = ClassicalSlice::new(&data, shape(&[3]))
            .expect("valid view");
        let empty = view.slice(0, 1..1, 1).expect("empty view");

        assert!(empty.is_empty());
        assert_eq!(empty.logical_size(), Ok(0));
        assert_eq!(empty.as_contiguous_slice(), Ok(&data[1..1]));
    }

    #[test]
    fn mutable_view_can_update_backing_storage() {
        let mut data = [1, 2, 3, 4];
        {
            let mut view = ClassicalSliceMut::new(&mut data, shape(&[4]))
                .expect("valid mutable view");

            assert_eq!(view.set(&[1], 20), Ok(2));
            assert_eq!(view.set_flat(3, 40), Ok(4));
        }

        assert_eq!(data, [1, 20, 3, 40]);
    }

    #[test]
    fn mutable_subview_updates_only_selected_elements() {
        let mut data = [0, 1, 2, 3, 4, 5];
        {
            let mut view = ClassicalSliceMut::new(&mut data, shape(&[6]))
                .expect("valid mutable view");
            {
                let mut subview = view
                    .slice(0, 1..6, 2)
                    .expect("valid mutable strided view");

                assert_eq!(subview.set_flat(0, 10), Ok(1));
                assert_eq!(subview.set_flat(2, 50), Ok(5));
            }
        }

        assert_eq!(data, [0, 10, 2, 3, 4, 50]);
    }

    #[test]
    fn converts_logical_view_order_to_a_vector() {
        let data = [0, 1, 2, 3, 4, 5];
        let view = ClassicalSlice::new(&data, shape(&[2, 3]))
            .expect("valid view");
        let columns = view.slice(1, 0..3, 2).expect("valid strided view");

        assert_eq!(columns.to_vec(), Ok(vec![0, 2, 3, 5]));
    }

    #[test]
    fn supports_large_single_dimension_metadata() {
        // No elements are allocated by constructing this shape. A real
        // backing slice of this size cannot generally be materialized.
        let huge = shape(&[usize::MAX]);
        assert_eq!(huge.dimensions(), &[usize::MAX]);
        assert_eq!(huge.logical_size(), Ok(usize::MAX));
    }
}
