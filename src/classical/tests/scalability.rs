
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani classical scalability conformance tests.
//!
//! Path: src/classical/tests/scalability.rs
//! Edition: Rust 2021
//! Minimum supported Rust version: Rust 1.97
//! Safety: unsafe Rust is forbidden.
//!
//! STATUS
//!   Test implementation for the canonical classical-array API.
//!   Repository-wide compilation and execution must still be verified.
//!
//! PURPOSE
//!   Detect regressions that introduce artificial semantic limits,
//!   unchecked shape/index arithmetic, incorrect multidimensional indexing,
//!   or unnecessary materialization of logical array metadata.
//!
//! AUTHORITY
//!   - grammar/specification/semantic-model.md
//!   - grammar/specification/scalability-model.md
//!   - grammar/specification/poco-reaf.md
//!   - src/quantum/ir/classical/array.rs
//!
//! OWNERSHIP
//!   The canonical array implementation owns shape, index, and array
//!   semantics. This file tests those contracts; it does not redefine them.
//!
//! INTEGRATION
//!   The classical test module must register this file as a child module.
//!   The canonical classical IR must expose its existing `array` module.
//!
//! SCALABILITY CONTRACT
//!   No test value below is a universal language or architecture limit.
//!   Test dimensions are selected to exercise representative boundary
//!   conditions, not to define the largest supported program.
//!
//!   Tests must never attempt to allocate an array proportional to an
//!   enormous logical shape just to prove that its metadata is representable.
//!
//! COMPLETION
//!   Run this test module through the actual repository's Rust build.
//!   Passing these tests is necessary but insufficient for production
//!   readiness across every classical computing subsystem.

use crate::quantum::ir::classical::array::{
    ClassicalArrayError,
    ClassicalArrayIndex,
    ClassicalArrayShape,
};

/// Constructs a shape and verifies its principal invariants.
///
/// Keeping the assertion logic in one helper ensures that tests check
/// the same basic contract for shapes of different ranks.
fn assert_valid_shape(shape: &ClassicalArrayShape) {
    assert!(!shape.dimensions().is_empty());
    assert_eq!(shape.rank(), shape.dimensions().len());
    assert!(shape.dimensions().iter().all(|dimension| *dimension > 0));
    assert_eq!(shape.validate(), Ok(()));
}

/// Empty dimensions are not valid array shapes.
///
/// This is a structural rule of the current canonical array API, not
/// a restriction on the number of dimensions a valid array may have.
#[test]
fn empty_shape_is_rejected_explicitly() {
    assert!(matches!(
        ClassicalArrayShape::new([]),
        Err(ClassicalArrayError::EmptyShape)
    ));
}

/// A zero-sized dimension must not be silently normalized to another size.
#[test]
fn zero_dimension_is_rejected_explicitly() {
    assert!(matches!(
        ClassicalArrayShape::new([3, 0, 5]),
        Err(ClassicalArrayError::InvalidDimension)
    ));

    assert!(matches!(
        ClassicalArrayShape::one_dimensional(0),
        Err(ClassicalArrayError::InvalidDimension)
    ));
}

/// Shape construction stores metadata and does not require element storage.
///
/// A very large logical length can be described without constructing a
/// collection containing that many values.
#[test]
fn large_logical_length_does_not_require_element_materialization() {
    let shape = ClassicalArrayShape::one_dimensional(usize::MAX)
        .expect("one representable dimension should be accepted");

    assert_valid_shape(&shape);
    assert_eq!(shape.rank(), 1);
    assert_eq!(shape.dimension(0), Some(usize::MAX));
    assert_eq!(shape.size(), usize::MAX);
    assert_eq!(shape.dimension(1), None);
}

/// Shape products that cannot be represented by the host index type
/// must fail instead of wrapping or saturating.
#[test]
fn overflowing_shape_product_is_rejected() {
    assert!(matches!(
        ClassicalArrayShape::new([usize::MAX, 2]),
        Err(ClassicalArrayError::ShapeSizeOverflow)
    ));

    assert!(matches!(
        ClassicalArrayShape::new([usize::MAX, 2, 3]),
        Err(ClassicalArrayError::ShapeSizeOverflow)
    ));
}

/// A single dimension may reach the host index type's representable limit.
///
/// This checks the difference between a representable logical size and an
/// overflowing product. It does not allocate storage of that size.
#[test]
fn representable_host_boundary_is_not_rejected_arbitrarily() {
    let shape = ClassicalArrayShape::new([usize::MAX])
        .expect("the largest representable single dimension is valid");

    assert_eq!(shape.size(), usize::MAX);
    assert_eq!(shape.rank(), 1);
    assert_eq!(shape.validate(), Ok(()));
}

/// Increasing rank must not encounter an arbitrary semantic rank ceiling.
///
/// All dimensions are one, so the test exercises rank metadata without
/// creating a large logical array or allocating element storage.
///
/// The selected rank is a test workload, not a maximum supported rank.
#[test]
fn high_rank_shapes_are_supported_without_element_allocation() {
    let dimensions = vec![1usize; 1024];

    let shape = ClassicalArrayShape::new(dimensions.clone())
        .expect("a high-rank shape with a representable size is valid");

    assert_valid_shape(&shape);
    assert_eq!(shape.rank(), dimensions.len());
    assert_eq!(shape.dimensions(), dimensions.as_slice());
    assert_eq!(shape.size(), 1);

    for axis in 0..dimensions.len() {
        assert_eq!(shape.dimension(axis), Some(1));
    }

    assert_eq!(shape.dimension(dimensions.len()), None);
}

/// Rank and shape must be determined by the input, not a fixed rank table.
#[test]
fn shape_rank_tracks_input_dimensions() {
    for rank in 1..=128 {
        let dimensions = vec![1usize; rank];

        let shape = ClassicalArrayShape::new(dimensions.clone())
            .expect("all-one dimensions have a representable product");

        assert_eq!(shape.rank(), rank);
        assert_eq!(shape.dimensions(), dimensions.as_slice());
        assert_eq!(shape.size(), 1);
    }
}

/// Multidimensional flattening must use row-major indexing correctly.
#[test]
fn multidimensional_flattening_is_correct() {
    let shape = ClassicalArrayShape::new([2, 3, 4])
        .expect("shape product is representable");

    assert_eq!(shape.size(), 24);

    let cases = [
        (vec![0, 0, 0], 0usize),
        (vec![0, 0, 1], 1),
        (vec![0, 1, 0], 4),
        (vec![1, 0, 0], 12),
        (vec![1, 2, 3], 23),
    ];

    for (indices, expected_flat) in cases {
        let flattened = shape
            .flatten(&indices)
            .expect("valid coordinates must flatten");

        assert_eq!(flattened.value(), expected_flat);
    }
}

/// Flattening and unflattening must preserve valid logical coordinates.
#[test]
fn flatten_and_unflatten_round_trip() {
    let shape = ClassicalArrayShape::new([3, 5, 7, 2])
        .expect("shape product is representable");

    let coordinates = [
        vec![0, 0, 0, 0],
        vec![2, 4, 6, 1],
        vec![1, 3, 2, 0],
        vec![0, 4, 0, 1],
    ];

    for original in coordinates {
        let flat = shape
            .flatten(&original)
            .expect("coordinates are within bounds");

        let reconstructed = shape
            .unflatten(flat)
            .expect("flattened index is within bounds");

        assert_eq!(reconstructed, original);
    }
}

/// A one-dimensional array can use the full representable index domain.
#[test]
fn largest_valid_logical_index_round_trips() {
    let shape = ClassicalArrayShape::one_dimensional(usize::MAX)
        .expect("single dimension is representable");

    let final_index = usize::MAX - 1;

    let flat = shape
        .flatten(&[final_index])
        .expect("the final valid index must flatten");

    assert_eq!(flat.value(), final_index);

    let reconstructed = shape
        .unflatten(flat)
        .expect("the final valid index must unflatten");

    assert_eq!(reconstructed, vec![final_index]);
}

/// The largest representable size is not itself a valid flattened index.
///
/// Valid indices are strictly less than the logical size.
#[test]
fn index_equal_to_size_is_out_of_bounds() {
    let shape = ClassicalArrayShape::one_dimensional(7)
        .expect("positive length is valid");

    assert!(matches!(
        ClassicalArrayIndex::new(7).validate(shape.size()),
        Err(ClassicalArrayError::SparseIndexOutOfBounds {
            index: 7,
            size: 7,
        })
    ));
}

/// Coordinates must have exactly the rank required by the shape.
#[test]
fn flatten_rejects_rank_mismatch() {
    let shape = ClassicalArrayShape::new([2, 3, 4])
        .expect("shape product is representable");

    assert!(matches!(
        shape.flatten(&[1, 2]),
        Err(ClassicalArrayError::RankMismatch {
            expected: 3,
            actual: 2,
        })
    ));

    assert!(matches!(
        shape.flatten(&[1, 2, 3, 0]),
        Err(ClassicalArrayError::RankMismatch {
            expected: 3,
            actual: 4,
        })
    ));
}

/// An invalid coordinate must identify the dimension and bound.
#[test]
fn flatten_rejects_out_of_bounds_coordinates() {
    let shape = ClassicalArrayShape::new([2, 3, 4])
        .expect("shape product is representable");

    assert!(matches!(
        shape.flatten(&[2, 0, 0]),
        Err(ClassicalArrayError::IndexOutOfBounds {
            dimension: 0,
            index: 2,
            bound: 2,
        })
    ));

    assert!(matches!(
        shape.flatten(&[1, 3, 0]),
        Err(ClassicalArrayError::IndexOutOfBounds {
            dimension: 1,
            index: 3,
            bound: 3,
        })
    ));

    assert!(matches!(
        shape.flatten(&[1, 2, 4]),
        Err(ClassicalArrayError::IndexOutOfBounds {
            dimension: 2,
            index: 4,
            bound: 4,
        })
    ));
}

/// An index outside the logical size must not be unflattened.
#[test]
fn unflatten_rejects_out_of_bounds_index() {
    let shape = ClassicalArrayShape::new([2, 3, 4])
        .expect("shape product is representable");

    assert!(matches!(
        shape.unflatten(ClassicalArrayIndex::new(shape.size())),
        Err(ClassicalArrayError::SparseIndexOutOfBounds {
            index: 24,
            size: 24,
        })
    ));
}

/// A single-element shape has exactly one valid flattened index.
#[test]
fn singleton_shape_has_one_valid_index() {
    let shape = ClassicalArrayShape::new([1, 1, 1, 1])
        .expect("all-one dimensions are valid");

    assert_eq!(shape.size(), 1);
    assert_eq!(
        shape.flatten(&[0, 0, 0, 0]).map(ClassicalArrayIndex::value),
        Ok(0)
    );
    assert_eq!(
        shape.unflatten(ClassicalArrayIndex::new(0)),
        Ok(vec![0, 0, 0, 0])
    );
}

/// A zero flattened index must map to the origin of a valid shape.
#[test]
fn zero_index_maps_to_origin() {
    let shape = ClassicalArrayShape::new([7, 11, 13])
        .expect("shape product is representable");

    assert_eq!(
        shape.unflatten(ClassicalArrayIndex::new(0)),
        Ok(vec![0, 0, 0])
    );

    assert_eq!(
        shape.flatten(&[0, 0, 0]).map(ClassicalArrayIndex::value),
        Ok(0)
    );
}

/// Shape validation must remain stable when called repeatedly.
#[test]
fn repeated_validation_is_deterministic() {
    let shape = ClassicalArrayShape::new([2, 3, 5, 7])
        .expect("shape product is representable");

    let first = shape.validate();
    let second = shape.validate();
    let third = shape.validate();

    assert_eq!(first, second);
    assert_eq!(second, third);
    assert_eq!(shape.size(), 210);
}

/// Shape metadata must be independent of unrelated host-side collections.
///
/// This test guards against accidental coupling to ambient mutable state.
/// It does not claim that allocating an equivalent dense array is feasible.
#[test]
fn shape_metadata_is_stable_across_unrelated_allocations() {
    let shape = ClassicalArrayShape::new([17, 19, 23])
        .expect("shape product is representable");

    let original_dimensions = shape.dimensions().to_vec();
    let original_size = shape.size();

    let unrelated: Vec<usize> = (0..4096).collect();

    assert_eq!(unrelated.len(), 4096);
    assert_eq!(shape.dimensions(), original_dimensions.as_slice());
    assert_eq!(shape.size(), original_size);
    assert_eq!(shape.validate(), Ok(()));
}

/// Checked shape construction must distinguish representable and
/// unrepresentable products, without saturating either result.
#[test]
fn shape_product_boundary_is_checked() {
    let valid = ClassicalArrayShape::new([usize::MAX / 2, 2])
        .expect("the product is representable when the host permits it");

    assert_eq!(
        valid.size(),
        (usize::MAX / 2) * 2
    );

    assert!(matches!(
        ClassicalArrayShape::new([usize::MAX / 2 + 1, 2]),
        Err(ClassicalArrayError::ShapeSizeOverflow)
    ));
}
