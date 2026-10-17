
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Contract and regression tests for Zamani classical collections.
//!
//! Path: src/classical/tests/collections.rs
//! Rust baseline: 1.97+
//! Edition: 2021
//! Safety: unsafe Rust is forbidden.
//!
//! # File contract
//!
//! Status: Test implementation proposed; compilation and execution are
//! not verified against a local repository checkout.
//!
//! ## Purpose
//!
//! Verify public collection contracts across arrays, vectors, matrices,
//! tensors, sequences, maps, sets, borrowed slices, sparse collections,
//! and iterator utilities.
//!
//! ## Authority
//!
//! The public APIs under `src/classical/collections/` are the implementation
//! under test. The normative scalability and POCO-REAF specifications govern
//! the scalability expectations exercised here.
//!
//! ## Ownership
//!
//! This file owns tests only. It does not define collection implementations,
//! source-language semantics, canonical values, types, AST, or IR.
//!
//! ## Integration
//!
//! Include this file as a child test module of `collections/mod.rs` using
//! the integration snippet supplied after this file. Tests then import the
//! collection module's public exports using `super::*`.
//!
//! ## Scalability
//!
//! Tests avoid relying on allocator-specific capacities, universal size
//! limits, or assumptions that all logical elements must be materialized.
//! Large-metadata tests use representable shapes without allocating their
//! logical element storage.
//!
//! ## Freeze rule
//!
//! A changed assertion must be justified by a deliberate public-contract
//! change, not by weakening a failing regression test to fit an implementation
//! defect.

use super::*;

// ============================================================================
// Shared test helpers
// ============================================================================

fn shape(dimensions: &[usize]) -> ClassicalArrayShape {
    ClassicalArrayShape::new(dimensions.to_vec())
        .expect("the test shape must be valid")
}

// ============================================================================
// Public API and basic contracts
// ============================================================================

#[test]
fn collection_public_exports_are_usable() {
    let array = ClassicalArray::<u32>::unmaterialized(
        ClassicalArrayShape::one_dimensional(3)
            .expect("valid shape"),
    );
    let vector = ClassicalVector::<u32>::new();
    let matrix = ClassicalMatrix::<u32>::unmaterialized(2, 3)
        .expect("valid matrix");
    let tensor = ClassicalTensor::<u32>::unmaterialized(shape(&[2, 3]));
    let sequence = ClassicalSequence::<u32>::new();
    let map = ClassicalMap::<u32, u32>::new();
    let set = ClassicalSet::<u32>::new();
    let sparse = ClassicalSparse::<u32>::new(3)
        .expect("valid sparse collection");

    assert_eq!(array.logical_size(), Ok(3));
    assert!(vector.is_empty());
    assert_eq!(matrix.shape(), (2, 3));
    assert_eq!(tensor.rank(), 2);
    assert!(sequence.is_empty());
    assert!(map.is_empty());
    assert!(set.is_empty());
    assert_eq!(sparse.logical_len(), 3);
}

#[test]
fn empty_collections_are_valid_and_reusable() {
    let mut vector = ClassicalVector::<u32>::new();
    let mut sequence = ClassicalSequence::<u32>::new();
    let mut map = ClassicalMap::<u32, u32>::new();
    let mut set = ClassicalSet::<u32>::new();

    vector.try_push(7).expect("vector growth");
    sequence.try_push(7).expect("sequence growth");
    map.insert(7, 49);
    set.insert(7);

    assert_eq!(vector.as_slice(), &[7]);
    assert_eq!(sequence.as_slice(), &[7]);
    assert_eq!(map.get(&7), Some(&49));
    assert!(set.contains(&7));
}

#[test]
fn concrete_collection_errors_implement_standard_error() {
    fn accepts_error(_: &dyn std::error::Error) {}

    let array_error = ClassicalArrayError::EmptyShape;
    let vector_error = ClassicalVectorError::IndexOutOfBounds {
        index: 0,
        len: 0,
        insertion_position: false,
    };
    let sequence_error = ClassicalSequenceError::InvalidRange {
        start: 2,
        end: 1,
        len: 3,
    };

    accepts_error(&array_error);
    accepts_error(&vector_error);
    accepts_error(&sequence_error);
}

// ============================================================================
// Array and shape contracts
// ============================================================================

#[test]
fn array_shape_preserves_dimension_order() {
    let shape = shape(&[2, 3, 5]);

    assert_eq!(shape.rank(), 3);
    assert_eq!(shape.dimensions(), &[2, 3, 5]);
    assert_eq!(shape.logical_size(), Ok(30));
}

#[test]
fn array_shape_accepts_zero_length_dimensions() {
    let shape = shape(&[4, 0, 9]);

    assert_eq!(shape.rank(), 3);
    assert_eq!(shape.logical_size(), Ok(0));
}

#[test]
fn array_shape_rejects_rank_zero() {
    assert_eq!(
        ClassicalArrayShape::new(Vec::<usize>::new()),
        Err(ClassicalArrayError::EmptyShape)
    );
}

#[test]
fn array_shape_reports_unrepresentable_logical_size() {
    let shape = shape(&[usize::MAX, 2]);

    assert_eq!(
        shape.logical_size(),
        Err(ClassicalArrayError::ShapeSizeOverflow)
    );
}

#[test]
fn array_flatten_and_coordinate_conversion_are_consistent() {
    let shape = shape(&[2, 3, 4]);

    let coordinates = [1, 2, 3];
    let flat = shape.flatten(&coordinates)
        .expect("coordinates must be valid");

    assert_eq!(flat, 23);
    assert_eq!(
        shape.coordinates(flat),
        Ok(vec![1, 2, 3])
    );
}

#[test]
fn array_dense_storage_preserves_row_major_order() {
    let array = ClassicalArray::from_dense(
        shape(&[2, 3]),
        vec![10, 20, 30, 40, 50, 60],
    )
    .expect("valid dense array");

    assert_eq!(array.get(&[0, 0]), Ok(Some(&10)));
    assert_eq!(array.get(&[0, 2]), Ok(Some(&30)));
    assert_eq!(array.get(&[1, 0]), Ok(Some(&40)));
    assert_eq!(array.get(&[1, 2]), Ok(Some(&60)));
}

#[test]
fn array_rejects_flat_indices_outside_logical_size() {
    let mut array = ClassicalArray::from_dense(
        shape(&[2, 2]),
        vec![1, 2, 3, 4],
    )
    .expect("valid dense array");

    assert_eq!(
        array.get_flat(4),
        Err(ClassicalArrayError::FlatIndexOutOfBounds {
            index: 4,
            size: 4,
        })
    );

    assert_eq!(
        array.set_flat(4, 5),
        Err(ClassicalArrayError::FlatIndexOutOfBounds {
            index: 4,
            size: 4,
        })
    );
}

#[test]
fn sparse_array_does_not_materialize_absent_elements() {
    let mut array = ClassicalArray::<u64>::sparse(shape(&[1_000_000]));

    assert_eq!(array.logical_size(), Ok(1_000_000));
    assert_eq!(array.stored_len(), 0);
    assert_eq!(array.get_flat(999_999), Ok(None));

    array.set_flat(999_999, 42).expect("sparse write");

    assert_eq!(array.stored_len(), 1);
    assert_eq!(array.get_flat(999_999), Ok(Some(&42)));
}

#[test]
fn unmaterialized_array_preserves_shape_without_storage() {
    let array = ClassicalArray::<u8>::unmaterialized(shape(&[100, 200]));

    assert_eq!(array.logical_size(), Ok(20_000));
    assert_eq!(array.stored_len(), 0);
    assert_eq!(
        array.storage_kind(),
        ClassicalArrayStorageKind::Unmaterialized
    );
}

#[test]
fn unmaterialized_array_does_not_silently_accept_writes() {
    let mut array = ClassicalArray::<u32>::unmaterialized(shape(&[4]));

    assert_eq!(
        array.set_flat(0, 1),
        Err(ClassicalArrayError::StorageNotMaterialized)
    );
}

// ============================================================================
// Vector contracts
// ============================================================================

#[test]
fn vector_preserves_order_and_duplicates() {
    let vector = ClassicalVector::from_vec(vec![3, 1, 3, 2]);

    assert_eq!(vector.as_slice(), &[3, 1, 3, 2]);
}

#[test]
fn vector_mutation_preserves_indexing_semantics() {
    let mut vector = ClassicalVector::from_vec(vec![10, 20, 30]);

    vector.try_insert(1, 15).expect("valid insertion");

    assert_eq!(vector.as_slice(), &[10, 15, 20, 30]);
    assert_eq!(vector.get(2), Some(&20));

    assert_eq!(vector.try_remove(1), Ok(15));
    assert_eq!(vector.as_slice(), &[10, 20, 30]);
}

#[test]
fn vector_reports_invalid_indices_without_panicking() {
    let mut vector = ClassicalVector::from_vec(vec![10, 20]);

    assert_eq!(vector.get(2), None);

    assert!(matches!(
        vector.try_insert(3, 30),
        Err(ClassicalVectorError::IndexOutOfBounds { .. })
    ));

    assert!(matches!(
        vector.try_remove(2),
        Err(ClassicalVectorError::IndexOutOfBounds { .. })
    ));
}

#[test]
fn vector_fallible_growth_preserves_existing_elements() {
    let mut vector = ClassicalVector::from_vec(vec![1, 2, 3]);

    vector.try_reserve(16).expect("reservation should succeed");

    assert_eq!(vector.as_slice(), &[1, 2, 3]);

    vector.try_extend([4, 5, 6]).expect("extension");

    assert_eq!(vector.as_slice(), &[1, 2, 3, 4, 5, 6]);
}

#[test]
fn vector_grows_beyond_small_fixed_inline_sizes() {
    let mut vector = ClassicalVector::new();

    vector.try_extend(0usize..4096).expect("vector growth");

    assert_eq!(vector.len(), 4096);
    assert_eq!(vector.first(), Some(&0));
    assert_eq!(vector.last(), Some(&4095));
}

#[test]
fn vector_clear_leaves_a_valid_reusable_collection() {
    let mut vector = ClassicalVector::from_vec(vec![1, 2, 3]);

    vector.clear();

    assert!(vector.is_empty());
    assert_eq!(vector.as_slice(), &[]);

    vector.try_push(9).expect("repopulation");

    assert_eq!(vector.as_slice(), &[9]);
}

// ============================================================================
// Matrix and tensor contracts
// ============================================================================

#[test]
fn matrix_uses_row_major_indexing() {
    let matrix = ClassicalMatrix::from_row_major(
        2,
        3,
        vec![10, 20, 30, 40, 50, 60],
    )
    .expect("valid matrix");

    assert_eq!(matrix.get(0, 0), Ok(Some(&10)));
    assert_eq!(matrix.get(0, 2), Ok(Some(&30)));
    assert_eq!(matrix.get(1, 0), Ok(Some(&40)));
    assert_eq!(matrix.get(1, 2), Ok(Some(&60)));
}

#[test]
fn matrix_keeps_zero_dimension_metadata() {
    let matrix = ClassicalMatrix::<u8>::unmaterialized(0, 12)
        .expect("valid zero-row matrix");

    assert_eq!(matrix.shape(), (0, 12));
    assert_eq!(matrix.logical_size(), Ok(0));
    assert!(matrix.is_empty());
}

#[test]
fn matrix_round_trips_through_the_array_boundary() {
    let matrix = ClassicalMatrix::from_row_major(
        2,
        2,
        vec![1, 2, 3, 4],
    )
    .expect("valid matrix");

    let array = matrix.into_array();
    let matrix = ClassicalMatrix::try_from_array(array)
        .expect("two-dimensional array");

    assert_eq!(matrix.shape(), (2, 2));
    assert_eq!(matrix.get(1, 1), Ok(Some(&4)));
}

#[test]
fn tensor_supports_arbitrary_rank_metadata() {
    let tensor = ClassicalTensor::<u8>::unmaterialized(
        shape(&[2, 3, 4, 5, 6]),
    );

    assert_eq!(tensor.rank(), 5);
    assert_eq!(tensor.dimensions(), &[2, 3, 4, 5, 6]);
    assert_eq!(tensor.logical_size(), Ok(720));
    assert_eq!(tensor.stored_len(), 0);
}

#[test]
fn tensor_supports_large_logical_shapes_without_dense_allocation() {
    let tensor = ClassicalTensor::<u8>::unmaterialized(
        shape(&[1_000_000, 1_000_000, 1_000]),
    );

    assert_eq!(
        tensor.logical_size(),
        Ok(1_000_000_000_000_000)
    );
    assert_eq!(tensor.stored_len(), 0);
    assert_eq!(
        tensor.storage_kind(),
        ClassicalArrayStorageKind::Unmaterialized
    );
}

#[test]
fn tensor_reports_unrepresentable_logical_size() {
    let tensor = ClassicalTensor::<u8>::unmaterialized(
        shape(&[usize::MAX, 2]),
    );

    assert_eq!(
        tensor.logical_size(),
        Err(ClassicalTensorError::Array(
            ClassicalArrayError::ShapeSizeOverflow
        ))
    );
}

#[test]
fn tensor_sparse_writes_do_not_densify_storage() {
    let mut tensor = ClassicalTensor::<u32>::sparse(shape(&[100, 100]));

    tensor.set(&[99, 99], 7).expect("sparse write");

    assert_eq!(tensor.stored_len(), 1);
    assert_eq!(tensor.get(&[99, 99]), Ok(Some(&7)));
    assert_eq!(tensor.get(&[0, 0]), Ok(None));
    assert_eq!(
        tensor.storage_kind(),
        ClassicalArrayStorageKind::Sparse
    );
}

// ============================================================================
// Sequence contracts
// ============================================================================

#[test]
fn sequence_preserves_duplicates_and_insertion_order() {
    let sequence = ClassicalSequence::from_vec(vec![4, 2, 4, 1]);

    assert_eq!(sequence.as_slice(), &[4, 2, 4, 1]);
}

#[test]
fn sequence_insert_and_remove_preserve_order() {
    let mut sequence = ClassicalSequence::from_vec(vec![1, 3, 4]);

    sequence.try_insert(1, 2).expect("valid insertion");

    assert_eq!(sequence.as_slice(), &[1, 2, 3, 4]);

    assert_eq!(sequence.remove(2), 3);
    assert_eq!(sequence.as_slice(), &[1, 2, 4]);
}

#[test]
fn sequence_rejects_invalid_insertions_and_ranges() {
    let mut sequence = ClassicalSequence::from_vec(vec![1, 2, 3]);

    assert!(matches!(
        sequence.try_insert(5, 4),
        Err(ClassicalSequenceError::IndexOutOfBounds { .. })
    ));

    assert!(matches!(
        sequence.get_range(3..1),
        Err(ClassicalSequenceError::InvalidRange { .. })
    ));
}

#[test]
fn sequence_search_returns_first_matching_position() {
    let sequence = ClassicalSequence::from_vec(vec![4, 2, 4, 1]);

    assert_eq!(sequence.position(|value| *value == 4), Some(0));
    assert_eq!(sequence.position(|value| *value == 1), Some(3));
    assert_eq!(sequence.position(|value| *value == 99), None);
}

#[test]
fn sequence_clear_keeps_the_collection_reusable() {
    let mut sequence = ClassicalSequence::from_vec(vec![1, 2, 3]);

    sequence.clear();

    assert!(sequence.is_empty());

    sequence.try_push(8).expect("repopulation");

    assert_eq!(sequence.as_slice(), &[8]);
}

// ============================================================================
// Map contracts
// ============================================================================

#[test]
fn map_iteration_is_sorted_and_deterministic() {
    let mut map = ClassicalMap::new();

    map.insert(3, "three");
    map.insert(1, "one");
    map.insert(2, "two");

    assert_eq!(
        map.iter().map(|(key, value)| (*key, *value)).collect::<Vec<_>>(),
        vec![(1, "one"), (2, "two"), (3, "three")]
    );
}

#[test]
fn map_replacement_does_not_create_duplicate_keys() {
    let mut map = ClassicalMap::new();

    map.insert(1, 10);
    map.insert(1, 20);

    assert_eq!(map.len(), 1);
    assert_eq!(map.get(&1), Some(&20));
}

#[test]
fn map_handles_extreme_representable_keys() {
    let mut map = ClassicalMap::new();

    map.insert(usize::MAX, "maximum");
    map.insert(0, "zero");

    assert_eq!(map.len(), 2);
    assert_eq!(map.get(&usize::MAX), Some(&"maximum"));
    assert_eq!(map.get(&0), Some(&"zero"));
}

#[test]
fn map_clear_leaves_a_valid_reusable_collection() {
    let mut map = ClassicalMap::new();

    map.insert(1, 10);
    map.clear();

    assert!(map.is_empty());

    map.insert(2, 20);

    assert_eq!(map.get(&2), Some(&20));
}

// ============================================================================
// Set contracts
// ============================================================================

#[test]
fn set_deduplicates_and_orders_values() {
    let mut set = ClassicalSet::new();

    set.insert(3);
    set.insert(1);
    set.insert(3);
    set.insert(2);

    assert_eq!(set.len(), 3);
    assert_eq!(set.as_slice(), &[1, 2, 3]);
}

#[test]
fn set_removal_and_membership_are_consistent() {
    let mut set = ClassicalSet::new();

    set.insert(1);
    set.insert(2);
    set.insert(3);

    assert!(set.contains(&2));
    assert!(set.remove(&2));
    assert!(!set.contains(&2));
    assert_eq!(set.as_slice(), &[1, 3]);
    assert!(set.validate().is_ok());
}

#[test]
fn set_relations_handle_empty_sets() {
    let empty = ClassicalSet::<i32>::new();
    let populated: ClassicalSet<i32> = [1, 2].into_iter().collect();

    assert!(empty.is_subset(&populated));
    assert!(empty.is_subset(&empty));
    assert!(populated.is_superset(&empty));
    assert!(empty.is_disjoint(&populated));
}

#[test]
fn set_can_grow_beyond_its_initial_capacity() {
    let mut set = ClassicalSet::try_with_capacity(1)
        .expect("small initial capacity");

    for value in 0..32 {
        set.try_insert(value).expect("set growth");
    }

    assert_eq!(set.len(), 32);
    assert!(set.capacity() >= set.len());
    assert!(set.validate().is_ok());
}

// ============================================================================
// Borrowed slice contracts
// ============================================================================

#[test]
fn immutable_slice_reads_without_copying_backing_data() {
    let data = [10, 20, 30, 40];
    let view = ClassicalSlice::new(&data, shape(&[2, 2]))
        .expect("valid view");

    assert_eq!(view.get(&[0, 0]), Ok(Some(&10)));
    assert_eq!(view.get(&[1, 1]), Ok(Some(&40)));
    assert_eq!(data, [10, 20, 30, 40]);
}

#[test]
fn mutable_slice_updates_only_selected_elements() {
    let mut data = [1, 2, 3, 4];

    {
        let mut view = ClassicalSliceMut::new(
            &mut data,
            shape(&[2, 2]),
        )
        .expect("valid mutable view");

        assert_eq!(view.set(&[1, 0], 30), Ok(3));
    }

    assert_eq!(data, [1, 2, 30, 4]);
}

#[test]
fn slice_rejects_backing_length_mismatch() {
    let data = [1, 2, 3];

    assert!(matches!(
        ClassicalSlice::new(&data, shape(&[2, 2])),
        Err(ClassicalSliceError::BackingLengthMismatch {
            expected: 4,
            actual: 3,
        })
    ));
}

#[test]
fn strided_slice_preserves_logical_iteration_order() {
    let data = [0, 1, 2, 3, 4, 5];

    let view = ClassicalSlice::new(&data, shape(&[2, 3]))
        .expect("valid view");

    let selected = view.slice(1, 0..3, 2)
        .expect("valid strided view");

    assert_eq!(selected.to_vec(), Ok(vec![0, 2, 3, 5]));
}

// ============================================================================
// Sparse collection contracts
// ============================================================================

#[test]
fn sparse_collection_tracks_logical_and_stored_lengths_separately() {
    let mut sparse = ClassicalSparse::<u64>::new(1_000_000)
        .expect("valid sparse collection");

    assert_eq!(sparse.logical_len(), 1_000_000);
    assert_eq!(sparse.stored_len(), 0);

    sparse.insert(999_999, 42).expect("sparse insertion");

    assert_eq!(sparse.logical_len(), 1_000_000);
    assert_eq!(sparse.stored_len(), 1);
    assert_eq!(sparse.get(999_999), Ok(Some(&42)));
}

#[test]
fn sparse_collection_removal_preserves_logical_length() {
    let mut sparse = ClassicalSparse::<u32>::new(8)
        .expect("valid sparse collection");

    sparse.insert(4, 12).expect("insertion");
    assert_eq!(sparse.stored_len(), 1);

    assert_eq!(sparse.remove(4), Ok(Some(12)));
    assert_eq!(sparse.stored_len(), 0);
    assert_eq!(sparse.logical_len(), 8);
}

#[test]
fn sparse_collection_rejects_out_of_bounds_indices() {
    let mut sparse = ClassicalSparse::<u32>::new(4)
        .expect("valid sparse collection");

    assert!(sparse.insert(4, 12).is_err());
    assert!(sparse.get(4).is_err());
}

// ============================================================================
// Iterator utility contracts
// ============================================================================

#[test]
fn iterator_extension_collects_all_items_in_order() {
    let values = (0usize..100)
        .try_collect_vec()
        .expect("fallible iterator collection");

    assert_eq!(values.len(), 100);
    assert_eq!(values.first(), Some(&0));
    assert_eq!(values.last(), Some(&99));
}

#[test]
fn iterator_extension_accepts_unknown_upper_size_hint() {
    struct UnknownUpper {
        next: usize,
        end: usize,
    }

    impl Iterator for UnknownUpper {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.next >= self.end {
                return None;
            }

            let value = self.next;
            self.next += 1;
            Some(value)
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            (0, None)
        }
    }

    let values = UnknownUpper { next: 0, end: 128 }
        .try_collect_vec()
        .expect("unknown upper bound must not impose a limit");

    assert_eq!(values.len(), 128);
    assert_eq!(values.last(), Some(&127));
}

#[test]
fn iterator_extension_supports_non_clone_items() {
    struct NonClone(String);

    let values = (1..=3)
        .map(|number| NonClone(number.to_string()))
        .try_collect_vec()
        .expect("non-clone items must be supported");

    assert_eq!(
        values.iter().map(|value| value.0.as_str()).collect::<Vec<_>>(),
        vec!["1", "2", "3"]
    );
}

// ============================================================================
// Cross-collection interoperability and scalability contracts
// ============================================================================

#[test]
fn vector_and_sequence_keep_distinct_semantics() {
    let vector = ClassicalVector::from_vec(vec![2, 1, 2]);
    let sequence = ClassicalSequence::from_vec(vec![2, 1, 2]);

    assert_eq!(vector.as_slice(), &[2, 1, 2]);
    assert_eq!(sequence.as_slice(), &[2, 1, 2]);

    let set: ClassicalSet<_> = [2, 1, 2].into_iter().collect();

    assert_eq!(set.as_slice(), &[1, 2]);
}

#[test]
fn dense_array_round_trip_preserves_elements_and_shape() {
    let array = ClassicalArray::from_dense(
        shape(&[2, 2]),
        vec![11, 22, 33, 44],
    )
    .expect("valid dense array");

    let dense = array.into_dense()
        .expect("dense storage must remain dense");

    assert_eq!(dense, vec![11, 22, 33, 44]);
}

#[test]
fn tensor_array_round_trip_preserves_sparse_storage() {
    let array = ClassicalArray::<u32>::sparse(shape(&[3, 4]));
    let tensor = ClassicalTensor::from(array);
    let array: ClassicalArray<u32> = tensor.into();

    assert_eq!(array.shape().dimensions(), &[3, 4]);
    assert_eq!(
        array.storage_kind(),
        ClassicalArrayStorageKind::Sparse
    );
    assert_eq!(array.stored_len(), 0);
}

#[test]
fn collection_capacity_is_not_a_semantic_length_limit() {
    let mut vector = ClassicalVector::try_with_capacity(1)
        .expect("small initial capacity");

    vector.try_extend(0..64).expect("vector growth");

    assert_eq!(vector.len(), 64);
    assert!(vector.capacity() >= vector.len());
}

#[test]
fn collection_metadata_does_not_claim_physical_feasibility() {
    // This shape is intentionally not materialized. The test verifies that
    // metadata can be inspected without allocating all logical elements.
    let tensor = ClassicalTensor::<u8>::unmaterialized(
        shape(&[usize::MAX]),
    );

    assert_eq!(tensor.rank(), 1);
    assert_eq!(tensor.dimensions(), &[usize::MAX]);
    assert_eq!(tensor.stored_len(), 0);
}
