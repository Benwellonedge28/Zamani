
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani classical collections: sparse one-dimensional storage.
//!
//! Path: src/classical/collections/sparse.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: safe Rust only.
//!
//! # File contract
//!
//! Status: Proposed implementation; compilation and repository integration
//! must be verified in the actual checkout.
//!
//! ## Purpose
//!
//! Expose a convenient sparse one-dimensional collection API while delegating
//! storage, indexing, and sparse-entry ordering to `ClassicalArray<T>`.
//!
//! ## Authority and ownership
//!
//! This module owns:
//! - `ClassicalSparse<T>`, a one-dimensional sparse collection façade;
//! - `ClassicalSparseError`, errors specific to the façade and wrapped array
//!   failures;
//! - validation that an adapted array is one-dimensional and sparse.
//!
//! `collections/array.rs` remains the sole owner of:
//! - `ClassicalArray<T>` and `ClassicalArrayStorage<T>`;
//! - sparse map storage and sparse entry ordering;
//! - flat-index validation;
//! - insert, remove, lookup, and storage invariants;
//! - `ClassicalArrayError`.
//!
//! This module does not create a second map, sparse tensor representation,
//! shape model, semantic type system, IR, allocator, or runtime.
//!
//! ## Semantic contract
//!
//! - The logical index domain is `0..logical_len`.
//! - Indices are zero-based.
//! - Absent entries return `None`; this API never invents a default value.
//! - Insertion replaces an existing value at the same index and returns the
//!   previous value, if any.
//! - Removal affects only an explicitly stored entry.
//! - Iteration visits explicitly stored entries in ascending index order.
//! - `logical_len()` is the index-domain length, while `stored_len()` is the
//!   number of entries actually present.
//! - This façade never converts sparse storage into dense storage.
//! - Writes to an unmaterialized or dense array are not accepted as sparse
//!   operations: `from_array` rejects those storage kinds.
//!
//! ## POCO-REAF and scalability
//!
//! There is no universal semantic maximum for the logical length or number of
//! entries. Logical length is represented by the existing host-indexed array
//! contract. Sparse storage grows according to actual insertions rather than
//! allocating every logical position.
//!
//! A finite process cannot guarantee infinite physical storage. The logical
//! index domain is bounded by the existing host representation, and actual
//! storage is bounded by the allocator and available resources.
//!
//! `BTreeMap` does not expose a stable fallible-reservation API. Consequently,
//! insertion can still encounter allocator-level panic/abort behavior; this
//! façade does not falsely promise to convert every allocation failure into a
//! recoverable error. A repository-wide recoverable-allocation guarantee
//! requires a storage-owner/API decision in `array.rs` or a replacement
//! storage abstraction, not a duplicate map introduced here.
//!
//! ## Integration contract
//!
//! - `collections/mod.rs` registers `pub mod sparse;` and re-exports
//!   `ClassicalSparse` and `ClassicalSparseError`.
//! - `collections/array.rs` remains the canonical sparse storage owner.
//! - `collections/tensor.rs` continues using `ClassicalArray<T>` directly for
//!   arbitrary-rank sparse tensors.
//! - `collections/vector.rs` remains the ordered dense/dynamic vector API;
//!   this type is not a replacement for it.
//! - `collections/shape.rs` remains the canonical shape metadata authority.
//! - `types/aggregate.rs` owns source-level aggregate type descriptors.
//! - Grammar files own syntax; semantic analysis resolves source-level types
//!   and operation legality.
//! - Lowering adapts validated operations to the canonical IR. This module
//!   constructs no AST or IR and performs no execution scheduling.
//!
//! ## Freeze rule
//!
//! Changes to index semantics, absent-entry behavior, storage-kind
//! restrictions, iteration order, or wrapped error behavior require
//! compatibility review and regression tests.

use core::fmt;

use super::array::{
    ClassicalArray,
    ClassicalArrayError,
    ClassicalArrayShape,
    ClassicalArrayStorageKind,
};

/// Errors returned by sparse one-dimensional collection operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClassicalSparseError {
    /// The supplied array is not one-dimensional.
    InvalidRank {
        /// Rank required by this façade.
        expected: usize,
        /// Rank of the supplied array.
        actual: usize,
    },

    /// The supplied array does not use sparse storage.
    ///
    /// This prevents the façade from silently converting dense or
    /// unmaterialized arrays into a different representation.
    NotSparse {
        /// Actual storage kind.
        actual: ClassicalArrayStorageKind,
    },

    /// An operation failed in the canonical array layer.
    Array(ClassicalArrayError),
}

impl fmt::Display for ClassicalSparseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidRank { expected, actual } => {
                write!(
                    f,
                    "sparse collection requires rank {expected}, received rank {actual}"
                )
            }
            Self::NotSparse { actual } => {
                write!(
                    f,
                    "sparse collection requires sparse storage, received {actual:?}"
                )
            }
            Self::Array(error) => write!(f, "sparse collection array error: {error}"),
        }
    }
}

impl std::error::Error for ClassicalSparseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Array(error) => Some(error),
            Self::InvalidRank { .. } | Self::NotSparse { .. } => None,
        }
    }
}

impl From<ClassicalArrayError> for ClassicalSparseError {
    fn from(error: ClassicalArrayError) -> Self {
        Self::Array(error)
    }
}

/// A sparse one-dimensional collection backed by canonical array storage.
///
/// `ClassicalSparse<T>` stores values only at explicitly populated indices.
/// It does not allocate a dense element for every logical index.
///
/// The backing array owns all storage and indexing behavior. This wrapper
/// supplies a simpler one-dimensional API and refuses to adapt arrays whose
/// storage kind or rank would violate its contract.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClassicalSparse<T> {
    array: ClassicalArray<T>,
}

impl<T> ClassicalSparse<T> {
    /// Creates an empty sparse collection with the specified logical length.
    ///
    /// No element values are allocated. Shape metadata and the empty sparse
    /// storage structure are created by the canonical array implementation.
    ///
    /// The length is not an allocation request for `logical_len` elements.
    /// Only explicit insertions add stored values.
    pub fn new(logical_len: usize) -> Result<Self, ClassicalSparseError> {
        let shape = ClassicalArrayShape::new(vec![logical_len])?;
        let array = ClassicalArray::sparse(shape);
        Ok(Self { array })
    }

    /// Wraps an existing one-dimensional sparse array without copying values.
    ///
    /// Arrays with a rank other than one, dense storage, or unmaterialized
    /// storage are rejected rather than silently converted.
    pub fn from_array(
        array: ClassicalArray<T>,
    ) -> Result<Self, ClassicalSparseError> {
        let actual_rank = array.shape().rank();

        if actual_rank != 1 {
            return Err(ClassicalSparseError::InvalidRank {
                expected: 1,
                actual: actual_rank,
            });
        }

        let actual_storage = array.storage_kind();

        if actual_storage != ClassicalArrayStorageKind::Sparse {
            return Err(ClassicalSparseError::NotSparse {
                actual: actual_storage,
            });
        }

        Ok(Self { array })
    }

    /// Returns the logical number of valid indices.
    ///
    /// This is independent of how many values have actually been inserted.
    #[must_use]
    pub fn logical_len(&self) -> usize {
        self.array.shape().dimensions()[0]
    }

    /// Returns the number of explicitly stored entries.
    #[must_use]
    pub fn stored_len(&self) -> usize {
        self.array.stored_len()
    }

    /// Returns true if the logical index domain has length zero.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.logical_len() == 0
    }

    /// Returns true if no values are explicitly stored.
    #[must_use]
    pub fn has_no_stored_entries(&self) -> bool {
        self.stored_len() == 0
    }

    /// Returns the canonical array storage representation by shared reference.
    ///
    /// This is useful for integration with array and tensor APIs without
    /// copying or converting the stored entries.
    #[must_use]
    pub fn as_array(&self) -> &ClassicalArray<T> {
        &self.array
    }

    /// Consumes the façade and returns its canonical sparse array.
    ///
    /// No conversion or element copying occurs.
    #[must_use]
    pub fn into_array(self) -> ClassicalArray<T> {
        self.array
    }

    /// Looks up an explicitly stored value by index.
    ///
    /// Returns `Ok(None)` when the index is valid but has no stored value.
    /// An index outside `0..logical_len()` returns a checked array error.
    pub fn get(&self, index: usize) -> Result<Option<&T>, ClassicalSparseError> {
        Ok(self.array.get_flat(index)?)
    }

    /// Looks up an explicitly stored value mutably by index.
    ///
    /// This does not insert an absent entry. Use `insert` to create or replace
    /// a value at an index.
    pub fn get_mut(
        &mut self,
        index: usize,
    ) -> Result<Option<&mut T>, ClassicalSparseError> {
        Ok(self.array.get_flat_mut(index)?)
    }

    /// Inserts a value or replaces the value at an existing index.
    ///
    /// Returns the previous value when the index was already populated.
    /// Returns `None` when the index was previously absent.
    ///
    /// This method does not densify the collection. As with the canonical
    /// `BTreeMap` storage, allocator-level failure is not guaranteed to be
    /// recoverable on stable Rust.
    pub fn insert(
        &mut self,
        index: usize,
        value: T,
    ) -> Result<Option<T>, ClassicalSparseError> {
        Ok(self.array.set_flat(index, value)?)
    }

    /// Removes and returns an explicitly stored value.
    ///
    /// Returns `Ok(None)` if the index is valid but not populated.
    pub fn remove(&mut self, index: usize) -> Result<Option<T>, ClassicalSparseError> {
        Ok(self.array.remove_flat(index)?)
    }

    /// Removes every stored entry while retaining the logical index domain.
    ///
    /// The backing array remains sparse. No dense elements are allocated.
    pub fn clear(&mut self) -> Result<(), ClassicalSparseError> {
        Ok(self.array.clear_sparse()?)
    }

    /// Iterates over explicitly stored entries in ascending index order.
    ///
    /// Each item is `(index, &value)`. Absent logical positions are skipped;
    /// this iterator never traverses the entire logical domain merely to
    /// discover absent entries.
    ///
    /// For `n` stored entries, iteration is O(n) and does not materialize an
    /// intermediate vector of indices.
    pub fn iter(&self) -> impl Iterator<Item = (usize, &T)> + '_ {
        self.array.sparse_entries().into_iter().flatten()
    }

    /// Returns the underlying array's storage kind.
    ///
    /// This is always `Sparse` for a valid `ClassicalSparse<T>`. Exposing the
    /// kind makes the invariant observable to diagnostics and integration
    /// code without exposing mutable access to the array's private fields.
    #[must_use]
    pub fn storage_kind(&self) -> ClassicalArrayStorageKind {
        self.array.storage_kind()
    }

    /// Checks the façade's structural invariants.
    ///
    /// This is intended for integration assertions and tests. Normal methods
    /// preserve these invariants through construction and canonical array
    /// validation.
    pub fn validate(&self) -> Result<(), ClassicalSparseError> {
        let actual_rank = self.array.shape().rank();

        if actual_rank != 1 {
            return Err(ClassicalSparseError::InvalidRank {
                expected: 1,
                actual: actual_rank,
            });
        }

        let actual_storage = self.array.storage_kind();

        if actual_storage != ClassicalArrayStorageKind::Sparse {
            return Err(ClassicalSparseError::NotSparse {
                actual: actual_storage,
            });
        }

        Ok(())
    }
}

impl<T> From<ClassicalSparse<T>> for ClassicalArray<T> {
    fn from(sparse: ClassicalSparse<T>) -> Self {
        sparse.into_array()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::classical::collections::array::{
        ClassicalArray,
        ClassicalArrayShape,
    };

    #[test]
    fn new_creates_sparse_collection_without_entries() {
        let sparse = ClassicalSparse::<i32>::new(100).unwrap();

        assert_eq!(sparse.logical_len(), 100);
        assert_eq!(sparse.stored_len(), 0);
        assert!(!sparse.is_empty());
        assert!(sparse.has_no_stored_entries());
        assert_eq!(sparse.storage_kind(), ClassicalArrayStorageKind::Sparse);
    }

    #[test]
    fn zero_length_collection_has_no_valid_indices() {
        let sparse = ClassicalSparse::<i32>::new(0).unwrap();

        assert_eq!(sparse.logical_len(), 0);
        assert!(sparse.is_empty());
        assert_eq!(
            sparse.get(0),
            Err(ClassicalSparseError::Array(
                ClassicalArrayError::FlatIndexOutOfBounds { index: 0, size: 0 }
            ))
        );
    }

    #[test]
    fn insertion_and_lookup_preserve_sparse_semantics() {
        let mut sparse = ClassicalSparse::new(1_000_000).unwrap();

        assert_eq!(sparse.insert(7, "seven"), Ok(None));
        assert_eq!(sparse.insert(900_000, "far away"), Ok(None));
        assert_eq!(sparse.stored_len(), 2);
        assert_eq!(sparse.get(7), Ok(Some(&"seven")));
        assert_eq!(sparse.get(8), Ok(None));
        assert_eq!(sparse.get(900_000), Ok(Some(&"far away")));
    }

    #[test]
    fn insertion_replaces_and_returns_previous_value() {
        let mut sparse = ClassicalSparse::new(5).unwrap();

        assert_eq!(sparse.insert(2, 10), Ok(None));
        assert_eq!(sparse.insert(2, 20), Ok(Some(10)));
        assert_eq!(sparse.get(2), Ok(Some(&20)));
        assert_eq!(sparse.stored_len(), 1);
    }

    #[test]
    fn rejects_out_of_bounds_insertions_and_reads() {
        let mut sparse = ClassicalSparse::<i32>::new(4).unwrap();

        assert!(matches!(
            sparse.insert(4, 99),
            Err(ClassicalSparseError::Array(
                ClassicalArrayError::FlatIndexOutOfBounds {
                    index: 4,
                    size: 4
                }
            ))
        ));

        assert!(matches!(
            sparse.get(usize::MAX),
            Err(ClassicalSparseError::Array(
                ClassicalArrayError::FlatIndexOutOfBounds { .. }
            ))
        ));

        assert_eq!(sparse.stored_len(), 0);
    }

    #[test]
    fn remove_only_removes_explicit_entries() {
        let mut sparse = ClassicalSparse::new(10).unwrap();

        sparse.insert(3, 30).unwrap();

        assert_eq!(sparse.remove(3), Ok(Some(30)));
        assert_eq!(sparse.remove(3), Ok(None));
        assert_eq!(sparse.stored_len(), 0);
    }

    #[test]
    fn iteration_is_sorted_and_skips_absent_positions() {
        let mut sparse = ClassicalSparse::new(100).unwrap();

        sparse.insert(80, "last").unwrap();
        sparse.insert(2, "first").unwrap();
        sparse.insert(17, "middle").unwrap();

        let entries: Vec<_> = sparse.iter().map(|(index, value)| (index, *value)).collect();

        assert_eq!(
            entries,
            vec![(2, "first"), (17, "middle"), (80, "last")]
        );
    }

    #[test]
    fn clear_removes_entries_but_preserves_logical_length() {
        let mut sparse = ClassicalSparse::new(100).unwrap();

        sparse.insert(1, 10).unwrap();
        sparse.insert(99, 20).unwrap();

        sparse.clear().unwrap();

        assert_eq!(sparse.logical_len(), 100);
        assert_eq!(sparse.stored_len(), 0);
        assert_eq!(sparse.get(1), Ok(None));
        assert!(sparse.validate().is_ok());
    }

    #[test]
    fn wraps_existing_sparse_one_dimensional_array() {
        let shape = ClassicalArrayShape::new(vec![12]).unwrap();
        let mut array = ClassicalArray::<i32>::sparse(shape);

        array.set_flat(4, 44).unwrap();

        let sparse = ClassicalSparse::from_array(array).unwrap();

        assert_eq!(sparse.logical_len(), 12);
        assert_eq!(sparse.get(4), Ok(Some(&44)));
    }

    #[test]
    fn rejects_dense_arrays_without_converting_them() {
        let shape = ClassicalArrayShape::new(vec![2]).unwrap();
        let array = ClassicalArray::from_dense(shape, vec![1, 2]).unwrap();

        assert!(matches!(
            ClassicalSparse::from_array(array),
            Err(ClassicalSparseError::NotSparse {
                actual: ClassicalArrayStorageKind::Dense
            })
        ));
    }

    #[test]
    fn rejects_unmaterialized_arrays_without_materializing_them() {
        let shape = ClassicalArrayShape::new(vec![2]).unwrap();
        let array = ClassicalArray::<i32>::unmaterialized(shape);

        assert!(matches!(
            ClassicalSparse::from_array(array),
            Err(ClassicalSparseError::NotSparse {
                actual: ClassicalArrayStorageKind::Unmaterialized
            })
        ));
    }

    #[test]
    fn rejects_multidimensional_arrays() {
        let shape = ClassicalArrayShape::new(vec![2, 3]).unwrap();
        let array = ClassicalArray::<i32>::sparse(shape);

        assert_eq!(
            ClassicalSparse::from_array(array).unwrap_err(),
            ClassicalSparseError::InvalidRank {
                expected: 1,
                actual: 2,
            }
        );
    }

    #[test]
    fn supports_non_clone_values() {
        struct NonClone(String);

        let mut sparse = ClassicalSparse::new(3).unwrap();
        sparse.insert(1, NonClone(String::from("value"))).unwrap();

        assert_eq!(
            sparse.get(1).unwrap().map(|value| value.0.as_str()),
            Some("value")
        );
    }

    #[test]
    fn exposes_canonical_array_without_copying() {
        let mut sparse = ClassicalSparse::new(8).unwrap();
        sparse.insert(6, 42).unwrap();

        assert_eq!(
            sparse.as_array().get_flat(6),
            Ok(Some(&42))
        );

        let array: ClassicalArray<i32> = sparse.into_array();
        assert_eq!(array.storage_kind(), ClassicalArrayStorageKind::Sparse);
        assert_eq!(array.get_flat(6), Ok(Some(&42)));
    }

    #[test]
    fn validates_invariants_after_mutations() {
        let mut sparse = ClassicalSparse::new(16).unwrap();

        sparse.insert(0, 1).unwrap();
        sparse.insert(15, 2).unwrap();
        assert!(sparse.validate().is_ok());

        sparse.remove(0).unwrap();
        assert!(sparse.validate().is_ok());

        sparse.clear().unwrap();
        assert!(sparse.validate().is_ok());
    }
}
