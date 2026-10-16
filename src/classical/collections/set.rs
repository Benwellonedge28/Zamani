
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani classical collections: deterministic ordered sets.
//!
//! Path: src/classical/collections/set.rs
//! Edition: Rust 2021
//! Rust baseline: 1.97 or later
//! Safety: unsafe Rust is forbidden.
//!
//! # Purpose
//!
//! Provides a target-independent ordered set of unique values.
//!
//! # Ownership and authority
//!
//! This module owns:
//! - `ClassicalSet<T>`, the concrete ordered-set implementation;
//! - ordering and duplicate-equivalence semantics;
//! - set membership, insertion, removal, iteration, and algebra;
//! - fallible backing-storage reservation and collection-local errors.
//!
//! This module does not own:
//! - Zamani source syntax, parser rules, or frontend AST nodes;
//! - canonical semantic types or classical values;
//! - ownership and borrowing analysis for source programs;
//! - canonical classical or quantum IR;
//! - resource discovery, scheduling, distributed placement, or execution;
//! - language-wide resource budgets or serialization formats.
//!
//! # Semantics
//!
//! Values are equivalent when `Ord::cmp` returns `Ordering::Equal`.
//! The element type must implement a lawful `Ord` consistent with `Eq`.
//! Exactly one representative of each equivalence class is stored.
//! Inserting an equivalent value preserves the existing representative.
//! Iteration is always in ascending order.
//!
//! # Scalability
//!
//! There is no language-level maximum set size in this module.
//! Storage grows dynamically and fallible operations expose reservation
//! failures. Actual capacity remains bounded by the host representation,
//! allocator, element layout, and available resources.
//!
//! No finite-memory implementation can guarantee infinite storage.
//! Operational budgets belong to the caller's resource policy.
//!
//! # Complexity
//!
//! For n stored elements:
//! - membership and binary search: O(log n) comparisons;
//! - insertion of a new element: O(n) element movement;
//! - replacement-equivalent insertion: O(log n) comparisons;
//! - removal: O(n) element movement;
//! - iteration: O(n);
//! - set relations and algebra: O(n + m) comparisons for two sorted sets,
//!   excluding the cost of element cloning and allocation.
//!
//! This implementation favors deterministic ordering, compact contiguous
//! storage, and fallible capacity reservation. A different backend can be
//! selected by higher-level lowering when its semantics and guarantees match.
//!
//! # Integration contract
//!
//! `collections/mod.rs` must declare and export this module.
//! `grammar/types/classical.g4` documents the `Set<T>` type form.
//! The authoritative frontend grammar and AST remain unchanged by this file.
//! Semantic analysis owns source-level type checking and mutability rules.
//! `memory/` owns source-level ownership and borrowing semantics.
//! Lowering adapts validated operations to the existing canonical IR.
//! This module must not introduce another AST, type system, or IR.
//!
//! # Freeze rule
//!
//! Changes to equivalence, representative preservation, ordering, or
//! fallible-operation update guarantees require compatibility review.

use core::cmp::Ordering;
use core::fmt;
use core::iter::FusedIterator;
use core::slice;
use std::collections::TryReserveError;

/// Errors returned by fallible ordered-set operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClassicalSetError {
    /// Additional backing storage could not be reserved.
    AllocationFailed {
        /// Additional element capacity requested by the operation.
        additional: usize,
        /// Input elements successfully processed before failure.
        ///
        /// This is zero for direct reservation and insertion operations.
        entries_processed: usize,
    },
}

impl fmt::Display for ClassicalSetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AllocationFailed {
                additional,
                entries_processed,
            } => write!(
                f,
                "set allocation failed while requesting {additional} \
                 additional elements; {entries_processed} input elements \
                 were processed before failure"
            ),
        }
    }
}

impl std::error::Error for ClassicalSetError {}

impl From<TryReserveError> for ClassicalSetError {
    fn from(_: TryReserveError) -> Self {
        Self::AllocationFailed {
            additional: 0,
            entries_processed: 0,
        }
    }
}

/// A deterministic set containing unique, ordered values.
///
/// Values are stored in ascending order. Two values are equivalent when
/// `T::cmp` returns `Ordering::Equal`.
///
/// The `Ord` implementation for `T` must be lawful and consistent with
/// equality. Types whose ordering can change while stored must not be used.
///
/// This collection is generic over the element type and does not define or
/// convert Zamani's semantic value representation.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ClassicalSet<T> {
    elements: Vec<T>,
}

impl<T> Default for ClassicalSet<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> ClassicalSet<T> {
    /// Creates an empty set without allocating element storage.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }

    /// Creates an empty set and fallibly reserves capacity for `capacity`
    /// elements.
    ///
    /// The requested capacity is not a maximum. The set may grow beyond it.
    pub fn try_with_capacity(
        capacity: usize,
    ) -> Result<Self, ClassicalSetError> {
        let mut elements = Vec::new();

        elements
            .try_reserve_exact(capacity)
            .map_err(|_| ClassicalSetError::AllocationFailed {
                additional: capacity,
                entries_processed: 0,
            })?;

        Ok(Self { elements })
    }

    /// Builds a set from an iterator using fallible incremental insertion.
    ///
    /// Duplicate-equivalent values are ignored after their first occurrence.
    /// On failure, the partially constructed set is dropped.
    pub fn try_from_iter<I>(iter: I) -> Result<Self, ClassicalSetError>
    where
        I: IntoIterator<Item = T>,
        T: Ord,
    {
        let mut set = Self::new();
        set.try_extend(iter)?;
        Ok(set)
    }

    /// Returns the number of distinct stored values.
    #[must_use]
    pub fn len(&self) -> usize {
        self.elements.len()
    }

    /// Returns whether the set contains no values.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    /// Returns the backing vector's current capacity.
    ///
    /// Capacity is an implementation detail, not a semantic size limit.
    #[must_use]
    pub fn capacity(&self) -> usize {
        self.elements.capacity()
    }

    /// Removes all values while retaining allocated capacity.
    pub fn clear(&mut self) {
        self.elements.clear();
    }

    /// Fallibly reserves capacity for at least `additional` elements beyond
    /// the current length.
    ///
    /// Logical contents are unchanged if reservation fails.
    pub fn try_reserve(
        &mut self,
        additional: usize,
    ) -> Result<(), ClassicalSetError> {
        self.elements
            .try_reserve(additional)
            .map_err(|_| ClassicalSetError::AllocationFailed {
                additional,
                entries_processed: 0,
            })
    }

    /// Fallibly reserves capacity for at least `additional` elements,
    /// requesting an exact reservation where supported.
    pub fn try_reserve_exact(
        &mut self,
        additional: usize,
    ) -> Result<(), ClassicalSetError> {
        self.elements
            .try_reserve_exact(additional)
            .map_err(|_| ClassicalSetError::AllocationFailed {
                additional,
                entries_processed: 0,
            })
    }

    /// Returns a reference to an equivalent stored value, if present.
    #[must_use]
    pub fn get<Q>(&self, value: &Q) -> Option<&T>
    where
        T: Ord + core::borrow::Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.search(value)
            .ok()
            .map(|index| &self.elements[index])
    }

    /// Returns whether an equivalent value is present.
    #[must_use]
    pub fn contains<Q>(&self, value: &Q) -> bool
    where
        T: Ord + core::borrow::Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.search(value).is_ok()
    }

    /// Inserts a value if no equivalent value is already present.
    ///
    /// Returns `true` if the set changed and `false` if an equivalent value
    /// already existed. In the duplicate case, the original representative
    /// remains stored.
    ///
    /// This convenience method may invoke the standard allocator when a new
    /// value requires growth. Use `try_insert` when allocation failure must
    /// be returned rather than handled by the standard allocation path.
    pub fn insert(&mut self, value: T) -> bool
    where
        T: Ord,
    {
        match self.search(&value) {
            Ok(_) => false,
            Err(index) => {
                self.elements.insert(index, value);
                true
            }
        }
    }

    /// Fallibly inserts a value if it is not already present.
    ///
    /// Returns `Ok(true)` if inserted, `Ok(false)` if an equivalent value
    /// already existed, and an error if required storage could not be
    /// reserved. On reservation failure, the existing set is unchanged.
    pub fn try_insert(
        &mut self,
        value: T,
    ) -> Result<bool, ClassicalSetError>
    where
        T: Ord,
    {
        let index = match self.search(&value) {
            Ok(_) => return Ok(false),
            Err(index) => index,
        };

        self.elements
            .try_reserve(1)
            .map_err(|_| ClassicalSetError::AllocationFailed {
                additional: 1,
                entries_processed: 0,
            })?;

        self.elements.insert(index, value);
        Ok(true)
    }

    /// Extends the set from an iterator using fallible insertion.
    ///
    /// Duplicate-equivalent values are ignored. The operation is incremental:
    /// if a later reservation fails, earlier successful insertions remain.
    /// `entries_processed` counts successfully processed input values,
    /// including duplicates.
    pub fn try_extend<I>(
        &mut self,
        iter: I,
    ) -> Result<(), ClassicalSetError>
    where
        I: IntoIterator<Item = T>,
        T: Ord,
    {
        let mut entries_processed = 0usize;

        for value in iter {
            if let Err(error) = self.try_insert(value) {
                return Err(match error {
                    ClassicalSetError::AllocationFailed { additional, .. } => {
                        ClassicalSetError::AllocationFailed {
                            additional,
                            entries_processed,
                        }
                    }
                });
            }

            // A Vec cannot contain more than usize::MAX elements.
            // Saturation prevents counter overflow without changing the
            // collection's insertion or error behavior.
            entries_processed = entries_processed.saturating_add(1);
        }

        Ok(())
    }

    /// Removes an equivalent value and returns the stored representative.
    pub fn remove<Q>(&mut self, value: &Q) -> Option<T>
    where
        T: Ord + core::borrow::Borrow<Q>,
        Q: Ord + ?Sized,
    {
        let index = self.search(value).ok()?;
        Some(self.elements.remove(index))
    }

    /// Removes all values for which `keep` returns `false`.
    ///
    /// The retained values remain ordered.
    pub fn retain<F>(&mut self, mut keep: F)
    where
        F: FnMut(&T) -> bool,
    {
        self.elements.retain(|value| keep(value));
    }

    /// Returns the smallest value, if present.
    #[must_use]
    pub fn first(&self) -> Option<&T> {
        self.elements.first()
    }

    /// Returns the largest value, if present.
    #[must_use]
    pub fn last(&self) -> Option<&T> {
        self.elements.last()
    }

    /// Removes and returns the smallest value, if present.
    pub fn pop_first(&mut self) -> Option<T> {
        if self.elements.is_empty() {
            None
        } else {
            Some(self.elements.remove(0))
        }
    }

    /// Removes and returns the largest value, if present.
    pub fn pop_last(&mut self) -> Option<T> {
        self.elements.pop()
    }

    /// Returns an iterator over values in ascending order.
    pub fn iter(
        &self,
    ) -> impl DoubleEndedIterator<Item = &T>
           + ExactSizeIterator
           + FusedIterator {
        self.elements.iter()
    }

    /// Returns the stored values as a read-only sorted slice.
    ///
    /// The slice prevents callers from mutating elements through this
    /// reference, which could invalidate the ordering invariant.
    #[must_use]
    pub fn as_slice(&self) -> &[T] {
        &self.elements
    }

    /// Consumes the set and returns its sorted values.
    ///
    /// The resulting vector is ordered and contains no equivalent duplicates.
    #[must_use]
    pub fn into_vec(self) -> Vec<T> {
        self.elements
    }

    /// Returns whether every value in `self` is also in `other`.
    #[must_use]
    pub fn is_subset(&self, other: &Self) -> bool
    where
        T: Ord,
    {
        let mut left = self.elements.iter().peekable();
        let mut right = other.elements.iter().peekable();

        while let Some(value) = left.next() {
            loop {
                match right.peek() {
                    Some(candidate) if *candidate < value => {
                        right.next();
                    }
                    Some(candidate) if *candidate == value => {
                        right.next();
                        break;
                    }
                    _ => return false,
                }
            }
        }

        true
    }

    /// Returns whether `self` contains every value in `other`.
    #[must_use]
    pub fn is_superset(&self, other: &Self) -> bool
    where
        T: Ord,
    {
        other.is_subset(self)
    }

    /// Returns whether the two sets contain exactly the same values.
    #[must_use]
    pub fn is_disjoint(&self, other: &Self) -> bool
    where
        T: Ord,
    {
        let mut left = self.elements.iter().peekable();
        let mut right = other.elements.iter().peekable();

        while let (Some(a), Some(b)) = (left.peek(), right.peek()) {
            match a.cmp(b) {
                Ordering::Less => {
                    left.next();
                }
                Ordering::Greater => {
                    right.next();
                }
                Ordering::Equal => return false,
            }
        }

        true
    }

    /// Returns a lazy iterator over the intersection of two ordered sets.
    ///
    /// Each yielded value is borrowed from `self`; no element is cloned.
    pub fn intersection<'a>(
        &'a self,
        other: &'a Self,
    ) -> impl Iterator<Item = &'a T> + 'a
    where
        T: Ord + 'a,
    {
        let mut left = self.elements.iter().peekable();
        let mut right = other.elements.iter().peekable();

        core::iter::from_fn(move || {
            loop {
                let a = left.peek()?;
                let b = right.peek()?;

                match a.cmp(b) {
                    Ordering::Less => {
                        left.next();
                    }
                    Ordering::Greater => {
                        right.next();
                    }
                    Ordering::Equal => {
                        let result = *left.next()?;
                        right.next();
                        return Some(result);
                    }
                }
            }
        })
    }

    /// Returns a lazy iterator over values in `self` but not in `other`.
    ///
    /// Each yielded value is borrowed from `self`; no element is cloned.
    pub fn difference<'a>(
        &'a self,
        other: &'a Self,
    ) -> impl Iterator<Item = &'a T> + 'a
    where
        T: Ord + 'a,
    {
        let mut left = self.elements.iter().peekable();
        let mut right = other.elements.iter().peekable();

        core::iter::from_fn(move || {
            loop {
                let a = left.peek()?;

                match right.peek() {
                    None => return left.next(),
                    Some(b) => match a.cmp(b) {
                        Ordering::Less => return left.next(),
                        Ordering::Greater => {
                            right.next();
                        }
                        Ordering::Equal => {
                            left.next();
                            right.next();
                        }
                    },
                }
            }
        })
    }

    /// Returns a lazy iterator over the symmetric difference.
    ///
    /// The result contains values present in exactly one of the two sets.
    /// Values are borrowed from their originating set and yielded in order.
    pub fn symmetric_difference<'a>(
        &'a self,
        other: &'a Self,
    ) -> impl Iterator<Item = &'a T> + 'a
    where
        T: Ord + 'a,
    {
        let mut left = self.elements.iter().peekable();
        let mut right = other.elements.iter().peekable();

        core::iter::from_fn(move || {
            loop {
                match (left.peek(), right.peek()) {
                    (None, None) => return None,
                    (Some(_), None) => return left.next(),
                    (None, Some(_)) => return right.next(),
                    (Some(a), Some(b)) => match a.cmp(b) {
                        Ordering::Less => return left.next(),
                        Ordering::Greater => return right.next(),
                        Ordering::Equal => {
                            left.next();
                            right.next();
                        }
                    },
                }
            }
        })
    }

    /// Creates the union of two sets, using fallible insertion.
    ///
    /// The result contains one representative per equivalence class.
    /// When equivalent values occur in both inputs, the representative from
    /// `self` is retained. Values unique to `other` are cloned from `other`.
    pub fn try_union(
        &self,
        other: &Self,
    ) -> Result<Self, ClassicalSetError>
    where
        T: Ord + Clone,
    {
        let mut result = Self::try_with_capacity(self.len())?;

        // Clone in sorted order and reserve through the fallible API.
        for value in &self.elements {
            result.try_insert(value.clone())?;
        }

        for value in &other.elements {
            result.try_insert(value.clone())?;
        }

        Ok(result)
    }

    /// Creates the intersection of two sets using fallible insertion.
    pub fn try_intersection(
        &self,
        other: &Self,
    ) -> Result<Self, ClassicalSetError>
    where
        T: Ord + Clone,
    {
        let mut result = Self::new();

        for value in self.intersection(other) {
            result.try_insert(value.clone())?;
        }

        Ok(result)
    }

    /// Creates the difference `self - other` using fallible insertion.
    pub fn try_difference(
        &self,
        other: &Self,
    ) -> Result<Self, ClassicalSetError>
    where
        T: Ord + Clone,
    {
        let mut result = Self::new();

        for value in self.difference(other) {
            result.try_insert(value.clone())?;
        }

        Ok(result)
    }

    /// Creates the symmetric difference using fallible insertion.
    pub fn try_symmetric_difference(
        &self,
        other: &Self,
    ) -> Result<Self, ClassicalSetError>
    where
        T: Ord + Clone,
    {
        let mut result = Self::new();

        for value in self.symmetric_difference(other) {
            result.try_insert(value.clone())?;
        }

        Ok(result)
    }

    /// Validates strict ordering and uniqueness.
    ///
    /// This is useful at trust boundaries and in diagnostics. Ordinary
    /// operations preserve the invariant without requiring this check.
    pub fn validate(&self) -> Result<(), ClassicalSetInvariantError>
    where
        T: Ord,
    {
        for (index, pair) in self.elements.windows(2).enumerate() {
            if pair[0] >= pair[1] {
                return Err(ClassicalSetInvariantError {
                    index,
                });
            }
        }

        Ok(())
    }

    fn search<Q>(&self, value: &Q) -> Result<usize, usize>
    where
        T: Ord + core::borrow::Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.elements.binary_search_by(|stored| {
            stored.borrow().cmp(value)
        })
    }
}

/// Indicates a violation of the ordered-set invariant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClassicalSetInvariantError {
    /// Index of the first element in the offending adjacent pair.
    pub index: usize,
}

impl fmt::Display for ClassicalSetInvariantError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ordered-set invariant violated at elements {} and {}",
            self.index,
            self.index.saturating_add(1)
        )
    }
}

impl std::error::Error for ClassicalSetInvariantError {}

impl<T: Ord> FromIterator<T> for ClassicalSet<T> {
    /// Builds a set from an iterator.
    ///
    /// This standard trait is infallible at the API level and may use the
    /// standard allocator failure path. Use `try_from_iter` to handle
    /// reservation failures explicitly.
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut set = Self::new();

        for value in iter {
            set.insert(value);
        }

        set
    }
}

impl<T: Ord> Extend<T> for ClassicalSet<T> {
    /// Inserts values from an iterator.
    ///
    /// Duplicate-equivalent values are ignored. This standard trait may use
    /// the standard allocator failure path. Use `try_extend` for fallibility.
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for value in iter {
            self.insert(value);
        }
    }
}

impl<T> IntoIterator for ClassicalSet<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.elements.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a ClassicalSet<T> {
    type Item = &'a T;
    type IntoIter = slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.elements.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::{ClassicalSet, ClassicalSetError};

    #[test]
    fn empty_set_has_no_values() {
        let set = ClassicalSet::<i32>::new();

        assert!(set.is_empty());
        assert_eq!(set.len(), 0);
        assert_eq!(set.first(), None);
        assert_eq!(set.last(), None);
        assert!(set.validate().is_ok());
    }

    #[test]
    fn insertion_sorts_and_deduplicates() {
        let mut set = ClassicalSet::new();

        assert!(set.insert(8));
        assert!(set.insert(2));
        assert!(set.insert(5));
        assert!(!set.insert(5));

        assert_eq!(set.as_slice(), &[2, 5, 8]);
        assert_eq!(set.len(), 3);
    }

    #[test]
    fn fallible_insertion_preserves_set_semantics() {
        let mut set = ClassicalSet::new();

        assert_eq!(set.try_insert(4), Ok(true));
        assert_eq!(set.try_insert(4), Ok(false));
        assert_eq!(set.try_insert(1), Ok(true));

        assert_eq!(set.as_slice(), &[1, 4]);
    }

    #[test]
    fn lookup_supports_borrowed_string_queries() {
        let mut set = ClassicalSet::new();

        set.insert(String::from("alpha"));
        set.insert(String::from("beta"));

        assert!(set.contains("alpha"));
        assert_eq!(set.get("beta").map(String::as_str), Some("beta"));
        assert!(!set.contains("gamma"));
    }

    #[test]
    fn removal_returns_the_stored_value() {
        let mut set: ClassicalSet<i32> = [1, 3, 5, 7].into_iter().collect();

        assert_eq!(set.remove(&3), Some(3));
        assert_eq!(set.remove(&3), None);
        assert_eq!(set.as_slice(), &[1, 5, 7]);
    }

    #[test]
    fn iteration_is_sorted_and_double_ended() {
        let set: ClassicalSet<i32> = [7, 1, 4, 1, 3].into_iter().collect();
        let mut iter = set.iter();

        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next_back(), Some(&7));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next_back(), Some(&4));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn clear_retains_capacity() {
        let mut set: ClassicalSet<i32> = [1, 2, 3].into_iter().collect();
        let capacity = set.capacity();

        set.clear();

        assert!(set.is_empty());
        assert!(set.capacity() >= capacity);
    }

    #[test]
    fn retain_preserves_order_and_uniqueness() {
        let mut set: ClassicalSet<i32> =
            [1, 2, 3, 4, 5, 6].into_iter().collect();

        set.retain(|value| value % 2 == 0);

        assert_eq!(set.as_slice(), &[2, 4, 6]);
        assert!(set.validate().is_ok());
    }

    #[test]
    fn pop_first_and_last_remove_extremes() {
        let mut set: ClassicalSet<i32> = [2, 4, 6].into_iter().collect();

        assert_eq!(set.pop_first(), Some(2));
        assert_eq!(set.pop_last(), Some(6));
        assert_eq!(set.pop_first(), Some(4));
        assert_eq!(set.pop_last(), None);
    }

    #[test]
    fn subset_superset_and_disjoint_are_correct() {
        let a: ClassicalSet<i32> = [1, 2].into_iter().collect();
        let b: ClassicalSet<i32> = [1, 2, 3].into_iter().collect();
        let c: ClassicalSet<i32> = [4, 5].into_iter().collect();

        assert!(a.is_subset(&b));
        assert!(b.is_superset(&a));
        assert!(!b.is_subset(&a));
        assert!(a.is_disjoint(&c));
        assert!(!a.is_disjoint(&b));
    }

    #[test]
    fn intersection_is_lazy_sorted_and_borrowed() {
        let a: ClassicalSet<i32> = [1, 2, 4, 6].into_iter().collect();
        let b: ClassicalSet<i32> = [2, 3, 4, 5].into_iter().collect();

        assert_eq!(a.intersection(&b).copied().collect::<Vec<_>>(), [2, 4]);
    }

    #[test]
    fn difference_is_correct() {
        let a: ClassicalSet<i32> = [1, 2, 4, 6].into_iter().collect();
        let b: ClassicalSet<i32> = [2, 3, 4, 5].into_iter().collect();

        assert_eq!(a.difference(&b).copied().collect::<Vec<_>>(), [1, 6]);
    }

    #[test]
    fn symmetric_difference_is_correct() {
        let a: ClassicalSet<i32> = [1, 2, 4, 6].into_iter().collect();
        let b: ClassicalSet<i32> = [2, 3, 4, 5].into_iter().collect();

        assert_eq!(
            a.symmetric_difference(&b).copied().collect::<Vec<_>>(),
            [1, 3, 5, 6]
        );
    }

    #[test]
    fn fallible_set_algebra_is_correct() {
        let a: ClassicalSet<i32> = [1, 2, 4].into_iter().collect();
        let b: ClassicalSet<i32> = [2, 3, 4].into_iter().collect();

        assert_eq!(
            a.try_union(&b).expect("union").as_slice(),
            &[1, 2, 3, 4]
        );
        assert_eq!(
            a.try_intersection(&b).expect("intersection").as_slice(),
            &[2, 4]
        );
        assert_eq!(
            a.try_difference(&b).expect("difference").as_slice(),
            &[1]
        );
        assert_eq!(
            a.try_symmetric_difference(&b)
                .expect("symmetric difference")
                .as_slice(),
            &[1, 3]
        );
    }

    #[test]
    fn try_extend_counts_duplicates_as_processed_inputs() {
        let mut set: ClassicalSet<i32> = [1].into_iter().collect();

        set.try_extend([1, 2, 2, 3]).expect("extend");

        assert_eq!(set.as_slice(), &[1, 2, 3]);
    }

    #[test]
    fn construction_from_iterator_deduplicates() {
        let set = ClassicalSet::try_from_iter([3, 2, 3, 1, 2])
            .expect("construction");

        assert_eq!(set.as_slice(), &[1, 2, 3]);
    }

    #[test]
    fn validation_accepts_a_valid_set_after_mutations() {
        let mut set: ClassicalSet<i32> = [9, 1, 5, 3].into_iter().collect();

        set.remove(&5);
        set.insert(4);
        set.retain(|value| *value != 9);

        assert_eq!(set.as_slice(), &[1, 3, 4]);
        assert!(set.validate().is_ok());
    }

    #[test]
    fn empty_set_relations_are_correct() {
        let empty = ClassicalSet::<i32>::new();
        let populated: ClassicalSet<i32> = [1, 2].into_iter().collect();

        assert!(empty.is_subset(&populated));
        assert!(empty.is_subset(&empty));
        assert!(populated.is_superset(&empty));
        assert!(empty.is_disjoint(&populated));
        assert!(empty.intersection(&populated).next().is_none());
        assert_eq!(
            empty.symmetric_difference(&populated).copied().collect::<Vec<_>>(),
            [1, 2]
        );
    }

    #[test]
    fn capacity_request_is_not_a_semantic_limit() {
        let mut set = ClassicalSet::try_with_capacity(1).expect("capacity");

        for value in 0..32 {
            set.try_insert(value).expect("growth");
        }

        assert_eq!(set.len(), 32);
        assert!(set.capacity() >= set.len());
        assert!(set.validate().is_ok());
    }

    #[test]
    fn standard_traits_preserve_set_semantics() {
        let mut set: ClassicalSet<i32> = [1, 2].into_iter().collect();
        set.extend([2, 3, 3]);

        let values: Vec<_> = set.into_iter().collect();
        assert_eq!(values, [1, 2, 3]);
    }

    #[test]
    fn allocation_error_reports_are_structured() {
        let error = ClassicalSetError::AllocationFailed {
            additional: 4,
            entries_processed: 2,
        };

        assert!(error.to_string().contains("4"));
        assert!(error.to_string().contains("2"));
    }
}
