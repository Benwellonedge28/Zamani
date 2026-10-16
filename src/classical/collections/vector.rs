
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani classical computation: dynamically sized vectors.
//!
//! Path: src/classical/collections/vector.rs
//! Edition: Rust 2021
//! Rust baseline: 1.97 or later
//! Safety: unsafe Rust is forbidden.
//!
//! # Purpose
//!
//! Provide a target-independent, ordered, one-dimensional collection whose
//! length can grow or shrink independently of its element type.
//!
//! # Architecture and authority
//!
//! This module owns:
//! - the `ClassicalVector<T>` runtime-neutral collection abstraction;
//! - vector-specific allocation and index errors;
//! - checked, fallible operations on vector storage;
//! - deterministic element ordering and iteration.
//!
//! This module does not own:
//! - Zamani source syntax or AST nodes;
//! - semantic type resolution or type inference;
//! - the canonical classical value representation;
//! - multidimensional array shape or tensor semantics;
//! - SIMD widths or hardware vector registers;
//! - memory placement, scheduling, or target selection;
//! - compiler IR, quantum IR, or backend execution.
//!
//! # POCO-REAF
//!
//! Vector length is program data. This module establishes no universal
//! maximum vector length, element count, SIMD width, device count, or memory
//! capacity.
//!
//! Storage uses Rust's dynamically sized `Vec<T>`. Its representable capacity
//! is bounded by the host platform, allocator, element layout, and available
//! resources. Those physical limits are not language-level vector limits.
//!
//! Operations that may need additional storage expose fallible alternatives
//! using `Vec::try_reserve` or `Vec::try_reserve_exact`.
//!
//! No implementation can guarantee that an arbitrarily large vector fits
//! in finite memory. Callers must apply any required compilation, runtime,
//! security, and resource budgets outside this collection abstraction.
//!
//! # Semantic contract
//!
//! - Elements have a stable logical order.
//! - Indexing is zero-based.
//! - `len()` reports the current element count.
//! - Index-based access does not allocate.
//! - Growth is explicit.
//! - Failed fallible growth does not silently truncate existing elements.
//! - A fallible extension may have appended some elements before a later
//!   allocation fails; the error reports how many were appended.
//! - Iteration follows element order.
//! - This module does not perform implicit numeric conversions.
//! - This module does not assume `T: Copy`, `T: Default`, or `T: Clone`
//!   unless a particular method requires that bound.
//!
//! # Integration contract
//!
//! `collections/mod.rs` registers and exports this module.
//!
//! `collections/array.rs` owns multidimensional array shapes, row-major
//! indexing, sparse array storage, and unmaterialized array declarations.
//! A vector is not a replacement for that API.
//!
//! `values/` supplies the element type selected by the caller.
//!
//! `types/aggregate.rs` owns semantic aggregate type descriptors.
//!
//! `grammar/classical/vector.g4` owns vector-related source syntax.
//! Semantic analysis decides whether a source expression denotes a vector.
//! The grammar does not construct this Rust container.
//!
//! The canonical classical IR and `quantum::ir` remain downstream semantic
//! boundaries. Lowering code must explicitly adapt validated vector
//! operations to those representations.
//!
//! # Determinism
//!
//! Element ordering is deterministic. This module does not use randomized
//! hash iteration or target-dependent ordering.
//!
//! # Freeze rule
//!
//! Changes to index semantics, mutation behavior, allocation error behavior,
//! or ordering require API compatibility review and regression tests.

use core::fmt;
use core::iter::FusedIterator;
use core::ops::Range;
use std::collections::TryReserveError;

// =============================================================================
// Errors
// =============================================================================

/// Errors returned by fallible classical-vector operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClassicalVectorError {
    /// An index or insertion position is invalid.
    ///
    /// For ordinary element access, valid indices are `0..len`.
    /// For insertion, valid positions are `0..=len`.
    IndexOutOfBounds {
        /// Requested index or insertion position.
        index: usize,
        /// Vector length when the operation was validated.
        len: usize,
        /// Whether the operation permits insertion at the end.
        insertion_position: bool,
    },

    /// Additional storage could not be reserved.
    ///
    /// `elements_added` is zero for operations that failed before modifying
    /// the vector. For incremental extension, it reports the number of
    /// elements successfully appended before the failure.
    AllocationFailed {
        /// Additional capacity requested by the failed reservation.
        additional: usize,
        /// Elements appended by this operation before failure.
        elements_added: usize,
    },

    /// A requested range is invalid for the current vector length.
    InvalidRange {
        /// Inclusive start position.
        start: usize,
        /// Exclusive end position.
        end: usize,
        /// Vector length.
        len: usize,
    },
}

impl fmt::Display for ClassicalVectorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IndexOutOfBounds {
                index,
                len,
                insertion_position,
            } => {
                if *insertion_position {
                    write!(
                        f,
                        "vector insertion position {index} is invalid \
                         for length {len}"
                    )
                } else {
                    write!(
                        f,
                        "vector index {index} is out of bounds \
                         for length {len}"
                    )
                }
            }

            Self::AllocationFailed {
                additional,
                elements_added,
            } => {
                write!(
                    f,
                    "vector allocation failed while requesting \
                     {additional} additional elements; \
                     {elements_added} elements were appended before failure"
                )
            }

            Self::InvalidRange { start, end, len } => {
                write!(
                    f,
                    "invalid vector range {start}..{end} for length {len}"
                )
            }
        }
    }
}

impl std::error::Error for ClassicalVectorError {}

impl From<TryReserveError> for ClassicalVectorError {
    fn from(_: TryReserveError) -> Self {
        Self::AllocationFailed {
            additional: 0,
            elements_added: 0,
        }
    }
}

// =============================================================================
// ClassicalVector
// =============================================================================

/// A dynamically sized, ordered, one-dimensional classical collection.
///
/// `ClassicalVector<T>` stores elements contiguously using Rust's safe `Vec`.
/// It does not assign hardware identities to elements and does not prescribe
/// how a compiler or runtime realizes vector operations.
///
/// Prefer the `try_*` methods when allocation failure must be handled as a
/// recoverable error rather than relying on infallible growth operations.
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ClassicalVector<T> {
    elements: Vec<T>,
}

impl<T> ClassicalVector<T> {
    // -------------------------------------------------------------------------
    // Construction
    // -------------------------------------------------------------------------

    /// Creates an empty vector without allocating element storage.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }

    /// Wraps an existing vector without copying its elements.
    ///
    /// This preserves the supplied vector's current length and capacity.
    #[must_use]
    pub fn from_vec(elements: Vec<T>) -> Self {
        Self { elements }
    }

    /// Creates an empty vector and fallibly reserves the requested capacity.
    ///
    /// The requested capacity is a reservation request, not a semantic
    /// vector-length limit. A successful reservation does not initialize
    /// elements or change the vector's logical length.
    pub fn try_with_capacity(
        capacity: usize,
    ) -> Result<Self, ClassicalVectorError> {
        let mut elements = Vec::new();

        elements.try_reserve_exact(capacity).map_err(|_| {
            ClassicalVectorError::AllocationFailed {
                additional: capacity,
                elements_added: 0,
            }
        })?;

        Ok(Self { elements })
    }

    /// Creates a vector from an iterator using fallible reservations.
    ///
    /// The iterator's lower size hint is used only as an initial reservation
    /// hint. It is not treated as a semantic maximum.
    ///
    /// If a later reservation fails, the partially constructed vector is
    /// dropped and the allocation error is returned.
    pub fn try_from_iter<I>(iter: I) -> Result<Self, ClassicalVectorError>
    where
        I: IntoIterator<Item = T>,
    {
        let iterator = iter.into_iter();
        let (lower_bound, _) = iterator.size_hint();

        let mut result = Self::try_with_capacity(lower_bound)?;

        for value in iterator {
            result.try_push(value)?;
        }

        Ok(result)
    }

    /// Returns an independent vector with cloned elements.
    ///
    /// Allocation is fallible. The element type must implement `Clone`.
    /// If cloning an element itself panics, the panic is not converted into
    /// an allocation error.
    pub fn try_clone(&self) -> Result<Self, ClassicalVectorError>
    where
        T: Clone,
    {
        let mut result = Self::try_with_capacity(self.len())?;

        for value in &self.elements {
            result.elements.push(value.clone());
        }

        Ok(result)
    }

    // -------------------------------------------------------------------------
    // Shape and capacity
    // -------------------------------------------------------------------------

    /// Returns the number of elements currently stored.
    #[must_use]
    pub fn len(&self) -> usize {
        self.elements.len()
    }

    /// Returns whether the vector contains no elements.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    /// Returns the current storage capacity.
    ///
    /// Capacity is an implementation detail, not the vector's logical length
    /// or a promise that future allocations will succeed.
    #[must_use]
    pub fn capacity(&self) -> usize {
        self.elements.capacity()
    }

    /// Returns the number of elements that can be added without reallocation.
    #[must_use]
    pub fn remaining_capacity(&self) -> usize {
        self.capacity().saturating_sub(self.len())
    }

    /// Fallibly reserves capacity for at least `additional` more elements.
    ///
    /// Existing elements and their order remain unchanged if reservation fails.
    pub fn try_reserve(
        &mut self,
        additional: usize,
    ) -> Result<(), ClassicalVectorError> {
        self.elements.try_reserve(additional).map_err(|_| {
            ClassicalVectorError::AllocationFailed {
                additional,
                elements_added: 0,
            }
        })
    }

    /// Fallibly reserves capacity for at least `additional` more elements,
    /// requesting an allocation as close as possible to the required size.
    pub fn try_reserve_exact(
        &mut self,
        additional: usize,
    ) -> Result<(), ClassicalVectorError> {
        self.elements
            .try_reserve_exact(additional)
            .map_err(|_| ClassicalVectorError::AllocationFailed {
                additional,
                elements_added: 0,
            })
    }

    /// Returns the logical element count as a `usize`.
    ///
    /// The underlying representation already uses `usize` for indexing and
    /// length. No narrowing conversion or fixed-width semantic length is used.
    #[must_use]
    pub fn logical_size(&self) -> usize {
        self.len()
    }

    // -------------------------------------------------------------------------
    // Read-only access
    // -------------------------------------------------------------------------

    /// Returns a shared reference to the element at `index`, if present.
    #[must_use]
    pub fn get(&self, index: usize) -> Option<&T> {
        self.elements.get(index)
    }

    /// Returns a mutable reference to the element at `index`, if present.
    #[must_use]
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.elements.get_mut(index)
    }

    /// Returns the elements as a slice.
    #[must_use]
    pub fn as_slice(&self) -> &[T] {
        self.elements.as_slice()
    }

    /// Returns the elements as a mutable slice.
    ///
    /// The slice permits element mutation but cannot change vector length.
    #[must_use]
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        self.elements.as_mut_slice()
    }

    /// Returns an iterator in logical element order.
    pub fn iter(&self) -> core::slice::Iter<'_, T> {
        self.elements.iter()
    }

    /// Returns a mutable iterator in logical element order.
    pub fn iter_mut(&mut self) -> core::slice::IterMut<'_, T> {
        self.elements.iter_mut()
    }

    /// Returns a checked slice for the half-open range `start..end`.
    pub fn get_range(
        &self,
        range: Range<usize>,
    ) -> Result<&[T], ClassicalVectorError> {
        if range.start > range.end || range.end > self.len() {
            return Err(ClassicalVectorError::InvalidRange {
                start: range.start,
                end: range.end,
                len: self.len(),
            });
        }

        // Range validation establishes start <= end <= len.
        Ok(&self.elements[range])
    }

    /// Returns the first element, if present.
    #[must_use]
    pub fn first(&self) -> Option<&T> {
        self.elements.first()
    }

    /// Returns the last element, if present.
    #[must_use]
    pub fn last(&self) -> Option<&T> {
        self.elements.last()
    }

    // -------------------------------------------------------------------------
    // Element replacement
    // -------------------------------------------------------------------------

    /// Replaces the element at `index`, returning the previous element.
    ///
    /// If `index` is invalid, the operation returns an error and leaves the
    /// vector unchanged.
    pub fn replace(
        &mut self,
        index: usize,
        value: T,
    ) -> Result<T, ClassicalVectorError> {
        let len = self.len();

        let slot = self.elements.get_mut(index).ok_or(
            ClassicalVectorError::IndexOutOfBounds {
                index,
                len,
                insertion_position: false,
            },
        )?;

        Ok(core::mem::replace(slot, value))
    }

    // -------------------------------------------------------------------------
    // Fallible growth
    // -------------------------------------------------------------------------

    /// Appends one element, reserving capacity fallibly when necessary.
    ///
    /// On allocation failure, existing elements remain unchanged. The supplied
    /// value is consumed by this call and dropped if the operation fails.
    pub fn try_push(
        &mut self,
        value: T,
    ) -> Result<(), ClassicalVectorError> {
        if self.len() == self.capacity() {
            self.try_reserve(1)?;
        }

        self.elements.push(value);
        Ok(())
    }

    /// Inserts an element at `index`, shifting later elements to the right.
    ///
    /// Positions in `0..=len` are valid. Allocation is reserved before
    /// mutation, so allocation failure leaves the vector unchanged.
    pub fn try_insert(
        &mut self,
        index: usize,
        value: T,
    ) -> Result<(), ClassicalVectorError> {
        let len = self.len();

        if index > len {
            return Err(ClassicalVectorError::IndexOutOfBounds {
                index,
                len,
                insertion_position: true,
            });
        }

        self.try_reserve(1)?;
        self.elements.insert(index, value);

        Ok(())
    }

    /// Appends elements from an iterator using fallible growth.
    ///
    /// If a later allocation fails, elements already appended by this
    /// operation remain appended. `elements_added` reports that partial
    /// progress. The original elements are never silently truncated.
    pub fn try_extend<I>(
        &mut self,
        iter: I,
    ) -> Result<(), ClassicalVectorError>
    where
        I: IntoIterator<Item = T>,
    {
        let iterator = iter.into_iter();
        let (lower_bound, _) = iterator.size_hint();

        // Reserving the lower bound up front reduces repeated growth but does
        // not assume that the iterator has a finite semantic maximum.
        if lower_bound > 0 {
            self.try_reserve(lower_bound)?;
        }

        let mut elements_added = 0usize;

        for value in iterator {
            if self.len() == self.capacity() {
                if let Err(error) = self.try_reserve(1) {
                    return Err(match error {
                        ClassicalVectorError::AllocationFailed {
                            additional,
                            ..
                        } => ClassicalVectorError::AllocationFailed {
                            additional,
                            elements_added,
                        },
                        other => other,
                    });
                }
            }

            self.elements.push(value);
            elements_added += 1;
        }

        Ok(())
    }

    /// Appends a repeated value, cloning it for each new element.
    ///
    /// Capacity is reserved before any elements are appended. If allocation
    /// fails, the vector remains unchanged.
    pub fn try_extend_cloned(
        &mut self,
        value: &T,
        count: usize,
    ) -> Result<(), ClassicalVectorError>
    where
        T: Clone,
    {
        self.try_reserve(count)?;

        for _ in 0..count {
            self.elements.push(value.clone());
        }

        Ok(())
    }

    /// Resizes the vector to `new_len`, creating additional elements with
    /// `make_element(index)` when it grows.
    ///
    /// Capacity is reserved before the closure is invoked. When shrinking,
    /// excess elements are dropped. When growing, allocation failure leaves
    /// the original vector unchanged.
    pub fn try_resize_with<F>(
        &mut self,
        new_len: usize,
        mut make_element: F,
    ) -> Result<(), ClassicalVectorError>
    where
        F: FnMut(usize) -> T,
    {
        let old_len = self.len();

        if new_len <= old_len {
            self.elements.truncate(new_len);
            return Ok(());
        }

        let additional = new_len - old_len;
        self.try_reserve(additional)?;

        for index in old_len..new_len {
            self.elements.push(make_element(index));
        }

        Ok(())
    }

    // -------------------------------------------------------------------------
    // Removal and shrinking
    // -------------------------------------------------------------------------

    /// Removes and returns the last element, if present.
    pub fn pop(&mut self) -> Option<T> {
        self.elements.pop()
    }

    /// Removes and returns the element at `index`, shifting later elements.
    ///
    /// Returns an error for an invalid index.
    pub fn try_remove(
        &mut self,
        index: usize,
    ) -> Result<T, ClassicalVectorError> {
        let len = self.len();

        if index >= len {
            return Err(ClassicalVectorError::IndexOutOfBounds {
                index,
                len,
                insertion_position: false,
            });
        }

        Ok(self.elements.remove(index))
    }

    /// Truncates the vector to `new_len` when it is shorter than its current
    /// length. This operation never allocates.
    pub fn truncate(&mut self, new_len: usize) {
        self.elements.truncate(new_len);
    }

    /// Removes all elements without requiring `T: Default`.
    ///
    /// The allocated capacity is retained for potential reuse.
    pub fn clear(&mut self) {
        self.elements.clear();
    }

    /// Removes all elements and returns the owned storage vector.
    #[must_use]
    pub fn into_vec(self) -> Vec<T> {
        self.elements
    }

    // -------------------------------------------------------------------------
    // Searching and predicates
    // -------------------------------------------------------------------------

    /// Returns the first index satisfying `predicate`.
    pub fn position<P>(&self, predicate: P) -> Option<usize>
    where
        P: FnMut(&T) -> bool,
    {
        self.elements.iter().position(predicate)
    }

    /// Returns whether any element satisfies `predicate`.
    pub fn any<P>(&self, predicate: P) -> bool
    where
        P: FnMut(&T) -> bool,
    {
        self.elements.iter().any(predicate)
    }

    /// Returns whether every element satisfies `predicate`.
    ///
    /// Returns `true` for an empty vector, following the standard universal
    /// quantification convention.
    pub fn all<P>(&self, predicate: P) -> bool
    where
        P: FnMut(&T) -> bool,
    {
        self.elements.iter().all(predicate)
    }

    /// Returns the number of elements satisfying `predicate`.
    pub fn count_where<P>(&self, predicate: P) -> usize
    where
        P: FnMut(&T) -> bool,
    {
        self.elements.iter().filter(predicate).count()
    }

    // -------------------------------------------------------------------------
    // Conversion and iteration
    // -------------------------------------------------------------------------

    /// Borrows the underlying standard-library vector as a slice.
    ///
    /// Use this to interoperate with APIs that accept contiguous slices.
    /// Mutation of vector length remains controlled by this abstraction.
    #[must_use]
    pub fn as_vec_slice(&self) -> &[T] {
        self.elements.as_slice()
    }

    /// Consumes the vector and returns its elements in logical order.
    #[must_use]
    pub fn into_elements(self) -> Vec<T> {
        self.elements
    }
}

// =============================================================================
// Standard collection interoperability
// =============================================================================

impl<T> From<Vec<T>> for ClassicalVector<T> {
    fn from(elements: Vec<T>) -> Self {
        Self::from_vec(elements)
    }
}

impl<T> From<ClassicalVector<T>> for Vec<T> {
    fn from(vector: ClassicalVector<T>) -> Self {
        vector.into_vec()
    }
}

impl<T> AsRef<[T]> for ClassicalVector<T> {
    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T> AsMut<[T]> for ClassicalVector<T> {
    fn as_mut(&mut self) -> &mut [T] {
        self.as_mut_slice()
    }
}

impl<T> core::ops::Deref for ClassicalVector<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<T> IntoIterator for ClassicalVector<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.elements.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a ClassicalVector<T> {
    type Item = &'a T;
    type IntoIter = core::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut ClassicalVector<T> {
    type Item = &'a mut T;
    type IntoIter = core::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

/// An exact-size, fused iterator over classical-vector elements.
///
/// This alias is provided for consumers that want to name the iterator
/// contract without depending on the underlying slice iterator type.
pub type ClassicalVectorIter<'a, T> = core::slice::Iter<'a, T>;

/// A fused-iterator assertion helper for API-level generic constraints.
///
/// The concrete iterator returned by [`ClassicalVector::iter`] implements
/// `ExactSizeIterator` and `FusedIterator`.
#[allow(dead_code)]
fn _assert_vector_iterator_contract<T>(
    iterator: ClassicalVectorIter<'_, T>,
) where
    ClassicalVectorIter<'_, T>: ExactSizeIterator + FusedIterator,
{
    let _ = iterator;
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_vector_has_zero_length() {
        let vector = ClassicalVector::<u32>::new();

        assert!(vector.is_empty());
        assert_eq!(vector.len(), 0);
        assert_eq!(vector.first(), None);
        assert_eq!(vector.last(), None);
    }

    #[test]
    fn wraps_and_preserves_existing_vector() {
        let vector = ClassicalVector::from_vec(vec![3, 5, 8]);

        assert_eq!(vector.as_slice(), &[3, 5, 8]);
        assert_eq!(vector.len(), 3);
    }

    #[test]
    fn supports_fallible_growth_and_ordered_iteration() {
        let mut vector = ClassicalVector::new();

        vector.try_push(10).unwrap();
        vector.try_push(20).unwrap();
        vector.try_push(30).unwrap();

        assert_eq!(vector.iter().copied().collect::<Vec<_>>(), [10, 20, 30]);
    }

    #[test]
    fn checked_access_and_replacement() {
        let mut vector = ClassicalVector::from_vec(vec![1, 2, 3]);

        assert_eq!(vector.get(1), Some(&2));
        assert_eq!(vector.get(3), None);
        assert_eq!(vector.replace(1, 9), Ok(2));
        assert_eq!(vector.as_slice(), &[1, 9, 3]);

        assert!(matches!(
            vector.replace(10, 0),
            Err(ClassicalVectorError::IndexOutOfBounds { .. })
        ));
    }

    #[test]
    fn insertion_checks_end_position_and_order() {
        let mut vector = ClassicalVector::from_vec(vec![1, 3]);

        vector.try_insert(1, 2).unwrap();
        vector.try_insert(3, 4).unwrap();

        assert_eq!(vector.as_slice(), &[1, 2, 3, 4]);

        assert!(matches!(
            vector.try_insert(6, 5),
            Err(ClassicalVectorError::IndexOutOfBounds {
                insertion_position: true,
                ..
            })
        ));
    }

    #[test]
    fn fallible_extension_appends_in_order() {
        let mut vector = ClassicalVector::from_vec(vec![1, 2]);

        vector.try_extend([3, 4, 5]).unwrap();

        assert_eq!(vector.as_slice(), &[1, 2, 3, 4, 5]);
    }

    #[test]
    fn cloned_extension_handles_zero_count() {
        let mut vector = ClassicalVector::from_vec(vec![1, 2]);

        vector.try_extend_cloned(&7, 0).unwrap();
        vector.try_extend_cloned(&9, 3).unwrap();

        assert_eq!(vector.as_slice(), &[1, 2, 9, 9, 9]);
    }

    #[test]
    fn resize_with_receives_each_new_index() {
        let mut vector = ClassicalVector::from_vec(vec![10, 20]);

        vector
            .try_resize_with(5, |index| index * 10)
            .unwrap();

        assert_eq!(vector.as_slice(), &[10, 20, 20, 30, 40]);

        vector.try_resize_with(2, |_| unreachable!()).unwrap();

        assert_eq!(vector.as_slice(), &[10, 20]);
    }

    #[test]
    fn removal_preserves_remaining_order() {
        let mut vector = ClassicalVector::from_vec(vec![10, 20, 30]);

        assert_eq!(vector.try_remove(1), Ok(20));
        assert_eq!(vector.pop(), Some(30));
        assert_eq!(vector.as_slice(), &[10]);

        assert!(matches!(
            vector.try_remove(1),
            Err(ClassicalVectorError::IndexOutOfBounds { .. })
        ));
    }

    #[test]
    fn checked_ranges_accept_empty_and_full_ranges() {
        let vector = ClassicalVector::from_vec(vec![2, 4, 6]);

        assert_eq!(vector.get_range(0..3), Ok(&[2, 4, 6][..]));
        assert_eq!(vector.get_range(1..1), Ok(&[][..]));

        assert!(matches!(
            vector.get_range(0..4),
            Err(ClassicalVectorError::InvalidRange { .. })
        ));

        assert!(matches!(
            vector.get_range(2..1),
            Err(ClassicalVectorError::InvalidRange { .. })
        ));
    }

    #[test]
    fn searches_preserve_element_order() {
        let vector = ClassicalVector::from_vec(vec![4, 7, 8, 7, 10]);

        assert_eq!(vector.position(|value| *value == 7), Some(1));
        assert!(vector.any(|value| value % 2 != 0));
        assert!(!vector.all(|value| value % 2 == 0));
        assert_eq!(vector.count_where(|value| *value == 7), 2);
    }

    #[test]
    fn empty_vector_predicate_semantics_are_defined() {
        let vector = ClassicalVector::<u8>::new();

        assert!(!vector.any(|_| true));
        assert!(vector.all(|_| false));
        assert_eq!(vector.count_where(|_| true), 0);
    }

    #[test]
    fn clone_and_conversion_preserve_elements() {
        let vector = ClassicalVector::from_vec(vec![1, 2, 3]);
        let cloned = vector.try_clone().unwrap();

        assert_eq!(vector, cloned);

        let standard: Vec<_> = cloned.into();
        assert_eq!(standard, vec![1, 2, 3]);
    }

    #[test]
    fn mutable_iteration_changes_elements_without_changing_length() {
        let mut vector = ClassicalVector::from_vec(vec![1, 2, 3]);

        for value in &mut vector {
            *value *= 2;
        }

        assert_eq!(vector.as_slice(), &[2, 4, 6]);
        assert_eq!(vector.len(), 3);
    }

    #[test]
    fn logical_size_tracks_current_length() {
        let mut vector = ClassicalVector::from_vec(vec![1, 2]);

        assert_eq!(vector.logical_size(), 2);

        vector.try_push(3).unwrap();

        assert_eq!(vector.logical_size(), 3);
    }

    #[test]
    fn clear_retains_valid_empty_state() {
        let mut vector = ClassicalVector::from_vec(vec![1, 2, 3]);

        vector.clear();

        assert!(vector.is_empty());
        assert_eq!(vector.len(), 0);
        assert_eq!(vector.as_slice(), &[]);
    }

    #[test]
    fn zero_capacity_construction_is_valid() {
        let vector = ClassicalVector::<u64>::try_with_capacity(0).unwrap();

        assert!(vector.is_empty());
        assert_eq!(vector.len(), 0);
    }

    #[test]
    fn iterator_construction_preserves_order() {
        let vector = ClassicalVector::try_from_iter(0..5).unwrap();

        assert_eq!(vector.as_slice(), &[0, 1, 2, 3, 4]);
    }

    #[test]
    fn vector_has_no_small_fixed_length_limit() {
        // Test growth beyond common small inline-buffer sizes without
        // assuming a specific Vec capacity or allocator growth strategy.
        let mut vector = ClassicalVector::new();

        vector.try_extend(0usize..4096).unwrap();

        assert_eq!(vector.len(), 4096);
        assert_eq!(vector.first(), Some(&0));
        assert_eq!(vector.last(), Some(&4095));
    }
}
