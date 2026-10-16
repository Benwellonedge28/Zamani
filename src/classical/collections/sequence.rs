
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani classical collections: dynamically sized sequences.
//!
//! Path: src/classical/collections/sequence.rs
//! Rust baseline: 1.97 or later
//! Edition: 2021
//! Safety: unsafe Rust is forbidden.
//!
//! # Purpose
//!
//! Provide an ordered, dynamically sized collection whose element order and
//! duplicate values are observable. Unlike a set, a sequence preserves every
//! inserted element unless an explicit operation removes or replaces it.
//!
//! # Authority and ownership
//!
//! This module owns:
//! - `ClassicalSequence<T>` and its sequence-specific invariants;
//! - sequence-local errors and checked positional operations;
//! - fallible allocation-aware sequence mutation;
//! - iteration and range access over sequence elements.
//!
//! This module does not own:
//! - source syntax, lexer, parser, or frontend AST;
//! - canonical semantic type descriptors or canonical values;
//! - source-language ownership and borrowing analysis;
//! - multidimensional array/tensor shapes;
//! - classical or quantum IR;
//! - scheduling, device selection, placement, or execution;
//! - global resource, security, or compilation policy.
//!
//! # Semantic contract
//!
//! - Element order is preserved.
//! - Duplicate values are permitted.
//! - Indexing is zero-based.
//! - Ranges are half-open: `start..end`.
//! - Insertion accepts positions in `0..=len`.
//! - Element access and removal accept indices in `0..len`.
//! - A successful insertion shifts later elements to the right.
//! - A successful removal shifts later elements to the left.
//! - `try_insert` leaves the sequence unchanged if reservation fails.
//! - `try_extend` is incremental; its error reports the number of elements
//!   successfully appended before failure.
//! - `try_append` reserves before changing either sequence.
//! - No implicit element conversion is performed.
//!
//! # POCO-REAF and scalability
//!
//! There is no universal semantic maximum sequence length. Storage grows
//! dynamically using `Vec<T>`. Actual capacity is constrained by the host
//! representation, allocator, element layout, and available resources.
//!
//! Rust's `usize` is the concrete indexing and length representation. This
//! module cannot represent an infinite in-memory sequence or guarantee that
//! arbitrary allocations succeed. Symbolic, distributed, lazy, or out-of-core
//! sequences require separate abstractions and runtime integration.
//!
//! Resource budgets must be enforced by shared resource/security policy,
//! not by introducing arbitrary language-wide sequence-size constants.
//!
//! # Complexity
//!
//! For n elements:
//! - indexed access: O(1);
//! - push/pop at the end: amortized O(1), excluding allocation;
//! - insertion/removal at an arbitrary position: O(n);
//! - range access: O(1) to obtain a borrowed slice;
//! - iteration: O(n);
//! - extending by k elements: O(k) amortized, plus any element movement.
//!
//! # Integration contract
//!
//! `collections/mod.rs` registers and exports this module.
//!
//! `collections/vector.rs` remains the existing contiguous vector API.
//! Where both APIs are used, adapters must explicitly preserve the semantics
//! expected by the caller. A sequence must not silently deduplicate elements.
//!
//! `collections/array.rs`, `matrix.rs`, and `tensor.rs` own multidimensional
//! shape and indexing semantics.
//!
//! `types/aggregate.rs` owns semantic aggregate type descriptors.
//! `memory/` owns source-level ownership and borrowing analysis.
//!
//! `grammar/types/classical.g4` documents `Sequence<T>` as a source-level
//! type form. `grammar/classical/classical.g4` is a relevant classical-domain
//! grammar boundary. The canonical parser and frontend AST remain authoritative.
//!
//! Semantic analysis decides whether a source-level operation is valid.
//! Lowering adapts validated operations to the existing canonical IR.
//! This module must not create a second AST, type system, or IR.
//!
//! # Freeze rule
//!
//! Changes to duplicate behavior, positional semantics, partial-update
//! guarantees, range behavior, or public errors require compatibility review.

use core::fmt;
use core::iter::FusedIterator;
use core::ops::Range;
use std::collections::TryReserveError;

/// Errors returned by fallible classical-sequence operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClassicalSequenceError {
    /// An element index or insertion position is invalid.
    IndexOutOfBounds {
        /// Requested index or insertion position.
        index: usize,
        /// Sequence length when the operation was validated.
        len: usize,
        /// Whether insertion at the end is allowed.
        insertion_position: bool,
    },

    /// A half-open range is invalid for the current sequence length.
    InvalidRange {
        /// Inclusive start.
        start: usize,
        /// Exclusive end.
        end: usize,
        /// Sequence length.
        len: usize,
    },

    /// Additional storage could not be reserved.
    ///
    /// `elements_added` counts elements appended by the current incremental
    /// operation before the failure. It is zero for single-operation
    /// reservations and mutations that fail before changing the sequence.
    AllocationFailed {
        /// Additional capacity requested.
        additional: usize,
        /// Elements appended by the current operation before failure.
        elements_added: usize,
    },
}

impl fmt::Display for ClassicalSequenceError {
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
                        "sequence insertion position {index} is invalid \
                         for length {len}"
                    )
                } else {
                    write!(
                        f,
                        "sequence index {index} is out of bounds \
                         for length {len}"
                    )
                }
            }
            Self::InvalidRange { start, end, len } => write!(
                f,
                "invalid sequence range {start}..{end} for length {len}"
            ),
            Self::AllocationFailed {
                additional,
                elements_added,
            } => write!(
                f,
                "sequence allocation failed while requesting \
                 {additional} additional elements; \
                 {elements_added} elements were appended before failure"
            ),
        }
    }
}

impl std::error::Error for ClassicalSequenceError {}

impl From<TryReserveError> for ClassicalSequenceError {
    fn from(_: TryReserveError) -> Self {
        Self::AllocationFailed {
            additional: 0,
            elements_added: 0,
        }
    }
}

/// A dynamically sized, ordered classical sequence.
///
/// Unlike `ClassicalSet<T>`, this type preserves duplicates and insertion
/// order. Its backing storage is private so every operation maintains the
/// collection's ordinary contiguous-storage invariants.
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ClassicalSequence<T> {
    elements: Vec<T>,
}

impl<T> ClassicalSequence<T> {
    // ---------------------------------------------------------------------
    // Construction
    // ---------------------------------------------------------------------

    /// Creates an empty sequence without allocating element storage.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }

    /// Wraps an existing vector without copying its elements.
    #[must_use]
    pub fn from_vec(elements: Vec<T>) -> Self {
        Self { elements }
    }

    /// Creates an empty sequence and fallibly reserves capacity.
    ///
    /// Capacity is a reservation request, not a maximum length.
    pub fn try_with_capacity(
        capacity: usize,
    ) -> Result<Self, ClassicalSequenceError> {
        let mut elements = Vec::new();

        elements.try_reserve_exact(capacity).map_err(|_| {
            ClassicalSequenceError::AllocationFailed {
                additional: capacity,
                elements_added: 0,
            }
        })?;

        Ok(Self { elements })
    }

    /// Builds a sequence from an iterator using fallible growth.
    ///
    /// The iterator's size hint is not treated as a semantic limit.
    /// If reservation fails, the partially built sequence is dropped.
    pub fn try_from_iter<I>(
        iter: I,
    ) -> Result<Self, ClassicalSequenceError>
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

    /// Clones the sequence using fallible initial allocation.
    ///
    /// If cloning an element itself panics, that panic is not converted into
    /// an allocation error.
    pub fn try_clone(&self) -> Result<Self, ClassicalSequenceError>
    where
        T: Clone,
    {
        let mut result = Self::try_with_capacity(self.len())?;

        for value in &self.elements {
            result.elements.push(value.clone());
        }

        Ok(result)
    }

    // ---------------------------------------------------------------------
    // Length and capacity
    // ---------------------------------------------------------------------

    /// Returns the number of stored elements.
    #[must_use]
    pub fn len(&self) -> usize {
        self.elements.len()
    }

    /// Returns whether the sequence is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    /// Returns the backing storage capacity.
    ///
    /// Capacity is an implementation detail, not a semantic size limit.
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
    /// Existing contents remain unchanged if reservation fails.
    pub fn try_reserve(
        &mut self,
        additional: usize,
    ) -> Result<(), ClassicalSequenceError> {
        self.elements.try_reserve(additional).map_err(|_| {
            ClassicalSequenceError::AllocationFailed {
                additional,
                elements_added: 0,
            }
        })
    }

    /// Fallibly reserves capacity for at least `additional` more elements,
    /// requesting an exact reservation where supported.
    pub fn try_reserve_exact(
        &mut self,
        additional: usize,
    ) -> Result<(), ClassicalSequenceError> {
        self.elements
            .try_reserve_exact(additional)
            .map_err(|_| ClassicalSequenceError::AllocationFailed {
                additional,
                elements_added: 0,
            })
    }

    /// Removes all elements while retaining allocated capacity.
    pub fn clear(&mut self) {
        self.elements.clear();
    }

    /// Shortens the sequence to `new_len`.
    ///
    /// If `new_len` is greater than the current length, nothing is added.
    pub fn truncate(&mut self, new_len: usize) {
        self.elements.truncate(new_len);
    }

    // ---------------------------------------------------------------------
    // Read access
    // ---------------------------------------------------------------------

    /// Returns a shared reference to an element, if the index is valid.
    #[must_use]
    pub fn get(&self, index: usize) -> Option<&T> {
        self.elements.get(index)
    }

    /// Returns a mutable reference to an element, if the index is valid.
    #[must_use]
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.elements.get_mut(index)
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

    /// Returns the elements as an immutable slice.
    #[must_use]
    pub fn as_slice(&self) -> &[T] {
        self.elements.as_slice()
    }

    /// Returns the elements as a mutable slice.
    ///
    /// This permits in-place element mutation but cannot change sequence
    /// length or backing-storage ownership.
    #[must_use]
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        self.elements.as_mut_slice()
    }

    /// Returns a checked borrowed view of `start..end`.
    ///
    /// Empty ranges are valid when `start == end <= len`.
    pub fn get_range(
        &self,
        range: Range<usize>,
    ) -> Result<&[T], ClassicalSequenceError> {
        if range.start > range.end || range.end > self.len() {
            return Err(ClassicalSequenceError::InvalidRange {
                start: range.start,
                end: range.end,
                len: self.len(),
            });
        }

        // Bounds are validated above; get() still avoids panicking if this
        // code is later modified incorrectly.
        self.elements
            .get(range.clone())
            .ok_or(ClassicalSequenceError::InvalidRange {
                start: range.start,
                end: range.end,
                len: self.len(),
            })
    }

    /// Returns a mutable borrowed view of `start..end`.
    pub fn get_range_mut(
        &mut self,
        range: Range<usize>,
    ) -> Result<&mut [T], ClassicalSequenceError> {
        let len = self.len();

        if range.start > range.end || range.end > len {
            return Err(ClassicalSequenceError::InvalidRange {
                start: range.start,
                end: range.end,
                len,
            });
        }

        self.elements
            .get_mut(range.clone())
            .ok_or(ClassicalSequenceError::InvalidRange {
                start: range.start,
                end: range.end,
                len,
            })
    }

    /// Returns an iterator in sequence order.
    pub fn iter(&self) -> core::slice::Iter<'_, T> {
        self.elements.iter()
    }

    /// Returns a mutable iterator in sequence order.
    pub fn iter_mut(&mut self) -> core::slice::IterMut<'_, T> {
        self.elements.iter_mut()
    }

    /// Returns whether any element satisfies `predicate`.
    pub fn contains_by<F>(&self, mut predicate: F) -> bool
    where
        F: FnMut(&T) -> bool,
    {
        self.elements.iter().any(&mut predicate)
    }

    /// Returns the position of the first element matching `predicate`.
    pub fn position<F>(&self, predicate: F) -> Option<usize>
    where
        F: FnMut(&T) -> bool,
    {
        self.elements.iter().position(predicate)
    }

    // ---------------------------------------------------------------------
    // Insertion
    // ---------------------------------------------------------------------

    /// Appends an element.
    ///
    /// This standard convenience operation may use the allocator's normal
    /// failure path. Use `try_push` when reservation failure must be returned.
    pub fn push(&mut self, value: T) {
        self.elements.push(value);
    }

    /// Fallibly appends an element.
    ///
    /// On reservation failure, the sequence is unchanged.
    pub fn try_push(
        &mut self,
        value: T,
    ) -> Result<(), ClassicalSequenceError> {
        self.elements.try_reserve(1).map_err(|_| {
            ClassicalSequenceError::AllocationFailed {
                additional: 1,
                elements_added: 0,
            }
        })?;

        self.elements.push(value);
        Ok(())
    }

    /// Inserts an element at `index`, shifting later elements to the right.
    ///
    /// Valid insertion positions are `0..=len`. Returns an error for an
    /// invalid position or a reservation failure. The sequence is unchanged
    /// on either failure.
    pub fn try_insert(
        &mut self,
        index: usize,
        value: T,
    ) -> Result<(), ClassicalSequenceError> {
        if index > self.len() {
            return Err(ClassicalSequenceError::IndexOutOfBounds {
                index,
                len: self.len(),
                insertion_position: true,
            });
        }

        self.elements.try_reserve(1).map_err(|_| {
            ClassicalSequenceError::AllocationFailed {
                additional: 1,
                elements_added: 0,
            }
        })?;

        self.elements.insert(index, value);
        Ok(())
    }

    /// Inserts an element at a valid position using the standard allocator
    /// behavior for any required growth.
    ///
    /// Returns `false` if the position is invalid.
    pub fn insert(&mut self, index: usize, value: T) -> bool {
        if index > self.len() {
            return false;
        }

        self.elements.insert(index, value);
        true
    }

    /// Fallibly extends the sequence from an iterator.
    ///
    /// The operation is incremental. If a later reservation fails, earlier
    /// successfully appended elements remain. `elements_added` reports the
    /// number of elements appended by this call before failure.
    pub fn try_extend<I>(
        &mut self,
        iter: I,
    ) -> Result<(), ClassicalSequenceError>
    where
        I: IntoIterator<Item = T>,
    {
        let mut elements_added = 0usize;

        for value in iter {
            if let Err(error) = self.try_push(value) {
                return Err(match error {
                    ClassicalSequenceError::AllocationFailed {
                        additional,
                        ..
                    } => ClassicalSequenceError::AllocationFailed {
                        additional,
                        elements_added,
                    },
                    other => other,
                });
            }

            elements_added = elements_added.saturating_add(1);
        }

        Ok(())
    }

    /// Appends all elements from `other`, leaving `other` empty on success.
    ///
    /// Reservation occurs before either sequence is modified. If reservation
    /// fails, both sequences retain their original contents.
    pub fn try_append(
        &mut self,
        other: &mut Self,
    ) -> Result<(), ClassicalSequenceError> {
        let additional = other.len();

        self.elements.try_reserve(additional).map_err(|_| {
            ClassicalSequenceError::AllocationFailed {
                additional,
                elements_added: 0,
            }
        })?;

        self.elements.append(&mut other.elements);
        Ok(())
    }

    // ---------------------------------------------------------------------
    // Removal and replacement
    // ---------------------------------------------------------------------

    /// Removes and returns the element at `index`.
    ///
    /// Later elements shift left. Returns `None` if the index is invalid.
    pub fn remove(&mut self, index: usize) -> Option<T> {
        if index >= self.len() {
            return None;
        }

        Some(self.elements.remove(index))
    }

    /// Removes and returns the first element, if present.
    pub fn pop_front(&mut self) -> Option<T> {
        if self.elements.is_empty() {
            None
        } else {
            Some(self.elements.remove(0))
        }
    }

    /// Removes and returns the last element, if present.
    pub fn pop_back(&mut self) -> Option<T> {
        self.elements.pop()
    }

    /// Replaces the element at `index` and returns the previous value.
    ///
    /// Returns `Err((index, value))` if the index is invalid. The supplied
    /// value is returned to the caller in that case.
    pub fn replace(
        &mut self,
        index: usize,
        value: T,
    ) -> Result<T, (usize, T)> {
        if index >= self.len() {
            return Err((index, value));
        }

        // The index has been checked, so the slot exists.
        Ok(core::mem::replace(&mut self.elements[index], value))
    }

    /// Retains only the elements for which `keep` returns `true`.
    ///
    /// Relative order of retained elements is preserved.
    pub fn retain<F>(&mut self, mut keep: F)
    where
        F: FnMut(&T) -> bool,
    {
        self.elements.retain(|value| keep(value));
    }

    /// Splits the sequence at `index`.
    ///
    /// The original sequence retains elements before `index`; the returned
    /// sequence contains elements from `index` onward.
    ///
    /// Returns an error if `index > len`. A valid split may allocate storage
    /// for the returned sequence using the standard allocator behavior.
    pub fn split_off(
        &mut self,
        index: usize,
    ) -> Result<Self, ClassicalSequenceError> {
        if index > self.len() {
            return Err(ClassicalSequenceError::IndexOutOfBounds {
                index,
                len: self.len(),
                insertion_position: true,
            });
        }

        Ok(Self {
            elements: self.elements.split_off(index),
        })
    }

    /// Consumes the sequence and returns its backing vector.
    #[must_use]
    pub fn into_vec(self) -> Vec<T> {
        self.elements
    }

    /// Checks the basic collection invariant.
    ///
    /// A contiguous sequence has no ordering or uniqueness invariant to
    /// validate. This method exists for uniform diagnostics and integration
    /// tests, and succeeds for every safely constructed instance.
    pub fn validate(&self) -> Result<(), ClassicalSequenceInvariantError> {
        Ok(())
    }
}

/// Error type reserved for sequence invariant validation.
///
/// The current sequence representation has no content invariant beyond
/// safe contiguous storage, so validation currently has no failure cases.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClassicalSequenceInvariantError;

impl fmt::Display for ClassicalSequenceInvariantError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("classical sequence invariant violation")
    }
}

impl std::error::Error for ClassicalSequenceInvariantError {}

impl<T> IntoIterator for ClassicalSequence<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.elements.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a ClassicalSequence<T> {
    type Item = &'a T;
    type IntoIter = core::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.elements.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut ClassicalSequence<T> {
    type Item = &'a mut T;
    type IntoIter = core::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.elements.iter_mut()
    }
}

impl<T> From<Vec<T>> for ClassicalSequence<T> {
    fn from(elements: Vec<T>) -> Self {
        Self::from_vec(elements)
    }
}

impl<T> From<ClassicalSequence<T>> for Vec<T> {
    fn from(sequence: ClassicalSequence<T>) -> Self {
        sequence.into_vec()
    }
}

impl<T> FromIterator<T> for ClassicalSequence<T> {
    /// Collects all values in iteration order.
    ///
    /// This standard trait does not expose recoverable allocation errors.
    /// Use `try_from_iter` when the caller must handle reservation failure.
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self {
            elements: iter.into_iter().collect(),
        }
    }
}

impl<T> Extend<T> for ClassicalSequence<T> {
    /// Appends values in iteration order.
    ///
    /// Use `try_extend` when recoverable allocation failure is required.
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.elements.extend(iter);
    }
}

impl<T> AsRef<[T]> for ClassicalSequence<T> {
    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T> AsMut<[T]> for ClassicalSequence<T> {
    fn as_mut(&mut self) -> &mut [T] {
        self.as_mut_slice()
    }
}

impl<T> core::ops::Index<usize> for ClassicalSequence<T> {
    type Output = T;

    /// Indexes a sequence using Rust's conventional indexing behavior.
    ///
    /// Panics if `index >= len`. Use `get` for checked, non-panicking access.
    fn index(&self, index: usize) -> &Self::Output {
        &self.elements[index]
    }
}

impl<T> core::ops::IndexMut<usize> for ClassicalSequence<T> {
    /// Mutably indexes a sequence using Rust's conventional indexing behavior.
    ///
    /// Panics if `index >= len`. Use `get_mut` for checked access.
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.elements[index]
    }
}

impl<T> FusedIterator for std::vec::IntoIter<T> {}

#[cfg(test)]
mod tests {
    use super::{ClassicalSequence, ClassicalSequenceError};

    #[test]
    fn empty_sequence_is_valid() {
        let sequence = ClassicalSequence::<i32>::new();

        assert!(sequence.is_empty());
        assert_eq!(sequence.len(), 0);
        assert_eq!(sequence.first(), None);
        assert_eq!(sequence.last(), None);
        assert!(sequence.validate().is_ok());
    }

    #[test]
    fn preserves_order_and_duplicates() {
        let sequence: ClassicalSequence<i32> =
            [3, 1, 3, 2, 1].into_iter().collect();

        assert_eq!(sequence.as_slice(), &[3, 1, 3, 2, 1]);
    }

    #[test]
    fn insertion_at_front_middle_and_end() {
        let mut sequence =
            ClassicalSequence::from_vec(vec![1, 3]);

        sequence.try_insert(0, 0).expect("front insertion");
        sequence.try_insert(2, 2).expect("middle insertion");
        sequence.try_insert(4, 4).expect("end insertion");

        assert_eq!(sequence.as_slice(), &[0, 1, 2, 3, 4]);
    }

    #[test]
    fn invalid_insertion_preserves_contents() {
        let mut sequence =
            ClassicalSequence::from_vec(vec![10, 20]);

        let error = sequence.try_insert(3, 30).expect_err("invalid position");

        assert_eq!(
            error,
            ClassicalSequenceError::IndexOutOfBounds {
                index: 3,
                len: 2,
                insertion_position: true,
            }
        );
        assert_eq!(sequence.as_slice(), &[10, 20]);
    }

    #[test]
    fn invalid_infallible_insertion_returns_false() {
        let mut sequence =
            ClassicalSequence::from_vec(vec![10, 20]);

        assert!(!sequence.insert(3, 30));
        assert_eq!(sequence.as_slice(), &[10, 20]);
    }

    #[test]
    fn access_and_mutation_are_checked() {
        let mut sequence =
            ClassicalSequence::from_vec(vec![10, 20, 30]);

        assert_eq!(sequence.get(1), Some(&20));
        assert_eq!(sequence.get(9), None);

        *sequence.get_mut(1).expect("valid index") = 25;

        assert_eq!(sequence.as_slice(), &[10, 25, 30]);
    }

    #[test]
    fn ranges_are_half_open() {
        let sequence =
            ClassicalSequence::from_vec(vec![0, 1, 2, 3, 4]);

        assert_eq!(sequence.get_range(1..4).expect("range"), &[1, 2, 3]);
        assert_eq!(sequence.get_range(2..2).expect("empty range"), &[]);
        assert_eq!(sequence.get_range(5..5).expect("end range"), &[]);
        assert!(sequence.get_range(4..2).is_err());
        assert!(sequence.get_range(0..6).is_err());
    }

    #[test]
    fn mutable_range_changes_elements_without_reordering() {
        let mut sequence =
            ClassicalSequence::from_vec(vec![1, 2, 3, 4]);

        sequence
            .get_range_mut(1..3)
            .expect("valid range")
            .copy_from_slice(&[20, 30]);

        assert_eq!(sequence.as_slice(), &[1, 20, 30, 4]);
    }

    #[test]
    fn removal_preserves_remaining_order() {
        let mut sequence =
            ClassicalSequence::from_vec(vec![1, 2, 3, 4]);

        assert_eq!(sequence.remove(1), Some(2));
        assert_eq!(sequence.remove(99), None);
        assert_eq!(sequence.as_slice(), &[1, 3, 4]);
    }

    #[test]
    fn front_and_back_removal_are_correct() {
        let mut sequence =
            ClassicalSequence::from_vec(vec![1, 2, 3]);

        assert_eq!(sequence.pop_front(), Some(1));
        assert_eq!(sequence.pop_back(), Some(3));
        assert_eq!(sequence.pop_front(), Some(2));
        assert_eq!(sequence.pop_front(), None);
        assert_eq!(sequence.pop_back(), None);
    }

    #[test]
    fn replace_returns_old_value() {
        let mut sequence =
            ClassicalSequence::from_vec(vec![1, 2, 3]);

        assert_eq!(sequence.replace(1, 20), Ok(2));
        assert_eq!(sequence.replace(9, 90), Err((9, 90)));
        assert_eq!(sequence.as_slice(), &[1, 20, 3]);
    }

    #[test]
    fn retain_preserves_relative_order_and_duplicates() {
        let mut sequence =
            ClassicalSequence::from_vec(vec![1, 2, 2, 3, 4, 4]);

        sequence.retain(|value| value % 2 == 0);

        assert_eq!(sequence.as_slice(), &[2, 2, 4, 4]);
    }

    #[test]
    fn split_off_preserves_both_parts() {
        let mut sequence =
            ClassicalSequence::from_vec(vec![1, 2, 3, 4]);

        let tail = sequence.split_off(2).expect("valid split");

        assert_eq!(sequence.as_slice(), &[1, 2]);
        assert_eq!(tail.as_slice(), &[3, 4]);
    }

    #[test]
    fn split_off_at_boundaries_is_valid() {
        let mut sequence =
            ClassicalSequence::from_vec(vec![1, 2]);

        let tail = sequence.split_off(0).expect("split at start");

        assert!(sequence.is_empty());
        assert_eq!(tail.as_slice(), &[1, 2]);

        let empty = sequence.split_off(0).expect("split empty sequence");
        assert!(empty.is_empty());
    }

    #[test]
    fn fallible_extension_preserves_input_order_and_duplicates() {
        let mut sequence =
            ClassicalSequence::from_vec(vec![1]);

        sequence.try_extend([2, 2, 3]).expect("extension");

        assert_eq!(sequence.as_slice(), &[1, 2, 2, 3]);
    }

    #[test]
    fn append_moves_elements_and_empties_source() {
        let mut left =
            ClassicalSequence::from_vec(vec![1, 2]);
        let mut right =
            ClassicalSequence::from_vec(vec![2, 3]);

        left.try_append(&mut right).expect("append");

        assert_eq!(left.as_slice(), &[1, 2, 2, 3]);
        assert!(right.is_empty());
    }

    #[test]
    fn clone_is_independent() {
        let mut original =
            ClassicalSequence::from_vec(vec![1, 2, 3]);
        let mut cloned = original.try_clone().expect("clone");

        cloned.replace(0, 99).expect("replace");

        assert_eq!(original.as_slice(), &[1, 2, 3]);
        assert_eq!(cloned.as_slice(), &[99, 2, 3]);
    }

    #[test]
    fn truncate_never_grows() {
        let mut sequence =
            ClassicalSequence::from_vec(vec![1, 2, 3]);

        sequence.truncate(2);
        assert_eq!(sequence.as_slice(), &[1, 2]);

        sequence.truncate(usize::MAX);
        assert_eq!(sequence.as_slice(), &[1, 2]);

        sequence.truncate(0);
        assert!(sequence.is_empty());
    }

    #[test]
    fn capacity_is_not_a_semantic_limit() {
        let mut sequence =
            ClassicalSequence::try_with_capacity(1).expect("capacity");

        for value in 0..64 {
            sequence.try_push(value).expect("growth");
        }

        assert_eq!(sequence.len(), 64);
        assert!(sequence.capacity() >= sequence.len());
    }

    #[test]
    fn clear_retains_capacity() {
        let mut sequence =
            ClassicalSequence::from_vec(vec![1, 2, 3]);
        let capacity = sequence.capacity();

        sequence.clear();

        assert!(sequence.is_empty());
        assert!(sequence.capacity() >= capacity);
    }

    #[test]
    fn iterators_preserve_sequence_order() {
        let sequence =
            ClassicalSequence::from_vec(vec![3, 1, 3, 2]);

        assert_eq!(
            sequence.iter().copied().collect::<Vec<_>>(),
            [3, 1, 3, 2]
        );

        assert_eq!(
            (&sequence).into_iter().copied().collect::<Vec<_>>(),
            [3, 1, 3, 2]
        );
    }

    #[test]
    fn search_uses_the_first_matching_position() {
        let sequence =
            ClassicalSequence::from_vec(vec![4, 2, 4, 1]);

        assert_eq!(sequence.position(|value| *value == 4), Some(0));
        assert_eq!(sequence.position(|value| *value == 1), Some(3));
        assert_eq!(sequence.position(|value| *value == 99), None);
        assert!(sequence.contains_by(|value| *value == 2));
    }

    #[test]
    fn conversion_traits_preserve_values() {
        let sequence = ClassicalSequence::from(vec![1, 2, 2, 3]);
        let values: Vec<_> = sequence.into();

        assert_eq!(values, [1, 2, 2, 3]);
    }

    #[test]
    fn indexing_and_mutable_indexing_follow_rust_conventions() {
        let mut sequence =
            ClassicalSequence::from_vec(vec![10, 20]);

        assert_eq!(sequence[0], 10);
        sequence[1] = 25;
        assert_eq!(sequence[1], 25);
    }

    #[test]
    fn error_display_is_available() {
        let error = ClassicalSequenceError::InvalidRange {
            start: 3,
            end: 1,
            len: 4,
        };

        assert!(error.to_string().contains("3..1"));
    }
}
