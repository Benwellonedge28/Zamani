
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani classical collection shapes.
//!
//! Path: src/classical/collections/shape.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: safe Rust only.
//!
//! # Purpose
//!
//! Own the concrete, target-independent shape of a classical collection.
//! A shape records ordered dimension lengths and provides checked operations
//! for dimension lookup, logical-size calculation, row-major indexing,
//! coordinate conversion, dimension replacement, and range validation.
//!
//! # Ownership
//!
//! This module owns `ClassicalArrayShape` and its invariants.
//!
//! `super::array::ClassicalArrayError` remains the compatible error type in
//! this integration step. The array module owns storage, allocation of
//! elements, and array access behavior.
//!
//! This module does not own source syntax, AST nodes, symbolic expressions,
//! semantic type resolution, array storage, numerical operations, hardware
//! placement, scheduling, runtime execution, or canonical IR.
//!
//! # Shape semantics
//!
//! - Dimensions are ordered from outermost to innermost.
//! - Flattening uses row-major order; the last dimension varies fastest.
//! - A zero-length dimension is valid and makes the logical size zero.
//! - Rank zero is rejected because this type describes arrays, not scalars.
//! - No universal maximum rank or dimension length is imposed.
//! - Arithmetic that must produce an unrepresentable `usize` result fails
//!   explicitly instead of wrapping.
//!
//! # POCO-REAF
//!
//! Shape metadata is independent of collection storage. Creating or changing
//! shape metadata does not allocate elements or imply that a target can
//! materialize the described collection.
//!
//! `usize` is used for concrete host indexing because this type is intended
//! for runtime collection shapes. Source-level symbolic or unresolved
//! dimensions must be represented by the semantic/type layer and resolved
//! before being converted to this concrete shape.
//!
//! # Integration contract
//!
//! - `collections/array.rs` re-exports `ClassicalArrayShape` for compatibility.
//! - Array, vector, matrix, tensor, and slice implementations reuse this type.
//! - Collection adapters must not define competing shape/indexing semantics.
//! - Grammar files define syntax only; they do not define runtime dimensions.
//! - Semantic validation resolves and validates symbolic dimension expressions.
//! - Canonical classical IR remains owned by the existing IR subsystem.
//!
//! # Freeze rule
//!
//! Changes to rank-zero behavior, zero-dimension behavior, dimension ordering,
//! row-major indexing, or public error behavior require compatibility review
//! and regression tests.

use core::ops::Range;

use super::array::ClassicalArrayError;

/// Concrete ordered dimensions for a classical array-like collection.
///
/// The type stores shape metadata only. It does not allocate collection
/// elements and does not guarantee that the logical collection can be
/// materialized in memory.
///
/// Dimensions are interpreted in row-major order. For example, dimensions
/// `[2, 3]` describe six logical elements, with coordinates `[0, 0]` through
/// `[1, 2]`.
///
/// Zero-length dimensions are permitted. An empty dimension list is rejected:
/// scalar values are not represented by this array-shape type.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ClassicalArrayShape {
    dimensions: Vec<usize>,
}

impl ClassicalArrayShape {
    /// Constructs a shape from ordered dimension lengths.
    ///
    /// This constructor receives concrete dimensions and allocates no element
    /// storage. It rejects rank zero but accepts dimensions whose length is
    /// zero. The caller is responsible for producing the input `Vec`; any
    /// allocation needed to construct that input occurs before this call.
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
        let mut dimensions = Vec::new();

        dimensions
            .try_reserve_exact(1)
            .map_err(|_| Self::metadata_allocation_error(1))?;

        dimensions.push(length);

        Ok(Self { dimensions })
    }

    /// Returns the number of dimensions.
    ///
    /// There is no configured maximum rank. The returned value is bounded
    /// only by the representational and allocation limits of the host `Vec`.
    #[must_use]
    pub fn rank(&self) -> usize {
        self.dimensions.len()
    }

    /// Returns the ordered dimension lengths.
    #[must_use]
    pub fn dimensions(&self) -> &[usize] {
        &self.dimensions
    }

    /// Returns the length of a particular dimension.
    ///
    /// The error uses the existing array error contract to preserve
    /// compatibility with callers already using `ClassicalArrayError`.
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

    /// Calculates the logical element count when it fits in `usize`.
    ///
    /// A shape containing any zero-length dimension has zero logical elements,
    /// even if multiplying the other dimension lengths would overflow.
    ///
    /// This operation does not allocate element storage.
    pub fn logical_size(&self) -> Result<usize, ClassicalArrayError> {
        if self.dimensions.contains(&0) {
            return Ok(0);
        }

        self.dimensions.iter().try_fold(1usize, |size, &length| {
            size.checked_mul(length)
                .ok_or(ClassicalArrayError::ShapeSizeOverflow)
        })
    }

    /// Returns whether this shape contains zero logical elements.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.dimensions.contains(&0)
    }

    /// Converts coordinates to a row-major flat index.
    ///
    /// The coordinate count must equal the shape rank. Every coordinate must
    /// be less than the length of its corresponding dimension.
    ///
    /// The method does not allocate. It checks arithmetic rather than
    /// allowing overflow to wrap around.
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

        let mut flat_index = 0usize;

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

            flat_index = flat_index
                .checked_mul(length)
                .and_then(|index| index.checked_add(coordinate))
                .ok_or(ClassicalArrayError::IndexOverflow)?;
        }

        Ok(flat_index)
    }

    /// Converts a valid row-major flat index into dimension coordinates.
    ///
    /// Coordinate-vector reservation is fallible and returns a structured
    /// allocation error if the host cannot reserve the required metadata.
    pub fn coordinates(
        &self,
        flat_index: usize,
    ) -> Result<Vec<usize>, ClassicalArrayError> {
        let logical_size = self.logical_size()?;

        if flat_index >= logical_size {
            return Err(ClassicalArrayError::FlatIndexOutOfBounds {
                index: flat_index,
                size: logical_size,
            });
        }

        // A valid flat index proves that the logical size is nonzero.
        // Therefore every dimension length is nonzero, so division below
        // cannot divide by zero.
        let rank = self.rank();
        let mut coordinates = Vec::new();

        coordinates
            .try_reserve_exact(rank)
            .map_err(|_| Self::metadata_allocation_error(rank))?;

        coordinates.resize(rank, 0);

        let mut remainder = flat_index;

        for dimension in (0..rank).rev() {
            let length = self.dimensions[dimension];
            coordinates[dimension] = remainder % length;
            remainder /= length;
        }

        Ok(coordinates)
    }

    /// Validates and returns a half-open range for a dimension.
    ///
    /// The valid range is `start..end` where `start <= end <= dimension_len`.
    /// Empty ranges are permitted. The returned range is identical to the
    /// input after validation.
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

    /// Returns a new shape with one dimension replaced.
    ///
    /// This changes shape metadata only. It does not reshape, copy, allocate,
    /// truncate, or otherwise modify any collection storage.
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

        let rank = self.rank();
        let mut dimensions = Vec::new();

        dimensions
            .try_reserve_exact(rank)
            .map_err(|_| Self::metadata_allocation_error(rank))?;

        dimensions.extend_from_slice(&self.dimensions);
        dimensions[dimension] = length;

        // The rank remains nonzero because this operation replaces a
        // dimension in an already valid shape.
        Ok(Self { dimensions })
    }

    /// Returns the range of representable row-major flat indices.
    ///
    /// The iterator is lazy and does not allocate a vector of indices.
    /// A logical element count that cannot fit in `usize` returns an error.
    pub fn flat_indices(
        &self,
    ) -> Result<Range<usize>, ClassicalArrayError> {
        Ok(0..self.logical_size()?)
    }

    /// Creates the existing structured error for a failed shape-metadata
    /// reservation.
    ///
    /// The `ShapeMetadataAllocationFailed` variant is added to
    /// `ClassicalArrayError` as described in the integration section below.
    fn metadata_allocation_error(
        dimensions: usize,
    ) -> ClassicalArrayError {
        ClassicalArrayError::ShapeMetadataAllocationFailed { dimensions }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn shape(dimensions: &[usize]) -> ClassicalArrayShape {
        ClassicalArrayShape::new(dimensions.to_vec())
            .expect("test shape should be valid")
    }

    #[test]
    fn rejects_rank_zero() {
        assert_eq!(
            ClassicalArrayShape::new(Vec::<usize>::new()),
            Err(ClassicalArrayError::EmptyShape)
        );
    }

    #[test]
    fn accepts_one_dimensional_shape() {
        let value = ClassicalArrayShape::one_dimensional(7)
            .expect("one-dimensional shape");

        assert_eq!(value.rank(), 1);
        assert_eq!(value.dimensions(), &[7]);
        assert_eq!(value.logical_size(), Ok(7));
    }

    #[test]
    fn preserves_dimension_order() {
        let value = shape(&[2, 3, 5]);

        assert_eq!(value.rank(), 3);
        assert_eq!(value.dimensions(), &[2, 3, 5]);
        assert_eq!(value.logical_size(), Ok(30));
    }

    #[test]
    fn zero_dimension_means_zero_logical_elements() {
        let value = shape(&[usize::MAX, 0, usize::MAX]);

        assert!(value.is_empty());
        assert_eq!(value.logical_size(), Ok(0));
        assert_eq!(value.flat_indices(), Ok(0..0));
    }

    #[test]
    fn reports_logical_size_overflow() {
        let value = shape(&[usize::MAX, 2]);

        assert_eq!(
            value.logical_size(),
            Err(ClassicalArrayError::ShapeSizeOverflow)
        );
    }

    #[test]
    fn flattens_in_row_major_order() {
        let value = shape(&[2, 3]);

        assert_eq!(value.flatten(&[0, 0]), Ok(0));
        assert_eq!(value.flatten(&[0, 1]), Ok(1));
        assert_eq!(value.flatten(&[0, 2]), Ok(2));
        assert_eq!(value.flatten(&[1, 0]), Ok(3));
        assert_eq!(value.flatten(&[1, 1]), Ok(4));
        assert_eq!(value.flatten(&[1, 2]), Ok(5));
    }

    #[test]
    fn rejects_coordinate_rank_mismatch() {
        let value = shape(&[2, 3]);

        assert_eq!(
            value.flatten(&[1]),
            Err(ClassicalArrayError::RankMismatch {
                expected: 2,
                actual: 1,
            })
        );
    }

    #[test]
    fn rejects_out_of_bounds_coordinate() {
        let value = shape(&[2, 3]);

        assert_eq!(
            value.flatten(&[2, 0]),
            Err(ClassicalArrayError::IndexOutOfBounds {
                dimension: 0,
                index: 2,
                bound: 2,
            })
        );
    }

    #[test]
    fn reports_flat_index_overflow() {
        let value = shape(&[usize::MAX, 2]);

        assert_eq!(
            value.flatten(&[usize::MAX - 1, 1]),
            Err(ClassicalArrayError::IndexOverflow)
        );
    }

    #[test]
    fn converts_flat_indices_to_coordinates() {
        let value = shape(&[2, 3]);

        let expected = [
            vec![0, 0],
            vec![0, 1],
            vec![0, 2],
            vec![1, 0],
            vec![1, 1],
            vec![1, 2],
        ];

        for (flat_index, coordinates) in expected.iter().enumerate() {
            assert_eq!(value.coordinates(flat_index), Ok(coordinates.clone()));
            assert_eq!(value.flatten(coordinates), Ok(flat_index));
        }
    }

    #[test]
    fn rejects_coordinate_conversion_for_empty_shape() {
        let value = shape(&[2, 0, 3]);

        assert_eq!(
            value.coordinates(0),
            Err(ClassicalArrayError::FlatIndexOutOfBounds {
                index: 0,
                size: 0,
            })
        );
    }

    #[test]
    fn rejects_flat_index_at_logical_size() {
        let value = shape(&[2, 3]);

        assert_eq!(
            value.coordinates(6),
            Err(ClassicalArrayError::FlatIndexOutOfBounds {
                index: 6,
                size: 6,
            })
        );
    }

    #[test]
    fn dimension_lookup_is_checked() {
        let value = shape(&[4, 8]);

        assert_eq!(value.dimension(0), Ok(4));
        assert_eq!(value.dimension(1), Ok(8));
        assert_eq!(
            value.dimension(2),
            Err(ClassicalArrayError::IndexOutOfBounds {
                dimension: 2,
                index: 2,
                bound: 2,
            })
        );
    }

    #[test]
    fn validates_half_open_ranges() {
        let value = shape(&[8, 4]);

        assert_eq!(value.validate_range(0, 2..7), Ok(2..7));
        assert_eq!(value.validate_range(0, 4..4), Ok(4..4));
        assert_eq!(
            value.validate_range(1, 0..5),
            Err(ClassicalArrayError::InvalidRange {
                start: 0,
                end: 5,
                length: 4,
            })
        );
        assert_eq!(
            value.validate_range(0, 7..2),
            Err(ClassicalArrayError::InvalidRange {
                start: 7,
                end: 2,
                length: 8,
            })
        );
    }

    #[test]
    fn replaces_dimension_without_mutating_original() {
        let original = shape(&[2, 3, 4]);
        let updated = original
            .with_dimension(1, usize::MAX)
            .expect("dimension replacement");

        assert_eq!(original.dimensions(), &[2, 3, 4]);
        assert_eq!(updated.dimensions(), &[2, usize::MAX, 4]);
        assert_eq!(
            updated.logical_size(),
            Err(ClassicalArrayError::ShapeSizeOverflow)
        );
    }

    #[test]
    fn rejects_replacement_of_nonexistent_dimension() {
        let value = shape(&[2, 3]);

        assert_eq!(
            value.with_dimension(2, 10),
            Err(ClassicalArrayError::IndexOutOfBounds {
                dimension: 2,
                index: 2,
                bound: 2,
            })
        );
    }

    #[test]
    fn flat_indices_are_lazy_and_ordered() {
        let value = shape(&[2, 3]);

        assert_eq!(
            value.flat_indices(),
            Ok(0..6)
        );
    }

    #[test]
    fn accepts_large_dimensions_without_allocating_elements() {
        let value = shape(&[usize::MAX]);

        assert_eq!(value.dimensions(), &[usize::MAX]);
        assert_eq!(value.logical_size(), Ok(usize::MAX));
        assert_eq!(value.flatten(&[usize::MAX - 1]), Ok(usize::MAX - 1));
    }

    #[test]
    fn does_not_impose_a_configured_rank_ceiling() {
        let dimensions = vec![1; 128];
        let value = ClassicalArrayShape::new(dimensions.clone())
            .expect("representable rank should be accepted");

        assert_eq!(value.rank(), dimensions.len());
        assert_eq!(value.logical_size(), Ok(1));
    }
}
