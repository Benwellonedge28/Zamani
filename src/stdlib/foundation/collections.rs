
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani foundational collections
//!
//! Path: `src/stdlib/foundation/collections.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: safe Rust only.
//!
//! # Purpose
//!
//! Provide reusable, target-independent collection APIs for the Zamani
//! standard library.
//!
//! This module owns:
//!
//! - `List<T>`: dynamically sized, ordered, duplicate-preserving storage.
//! - `Map<K, V>`: dynamically sized hash-based key/value storage.
//! - `CollectionError`: structured collection-operation failures.
//! - `CollectionKind`: identifies the affected collection.
//!
//! # Ownership boundaries
//!
//! This module does not own:
//!
//! - source grammar, lexer, parser, AST, or semantic types;
//! - canonical classical collection implementations;
//! - tensor, matrix, vector, or multidimensional storage;
//! - quantum registers, states, or quantum IR;
//! - HDL signals, registers, or hardware storage;
//! - distributed storage, sharding, replication, or coordination;
//! - runtime allocation policy or resource budgets;
//! - scheduling, device discovery, or hardware placement;
//! - compiler IR or backend representations.
//!
//! Specialized classical collection semantics remain owned by
//! `crate::classical::collections`.
//!
//! # Scalability
//!
//! No arbitrary universal collection-length or capacity ceiling is imposed.
//!
//! Storage grows according to the underlying allocator and available
//! resources. Concrete storage remains subject to:
//!
//! - Rust's `usize` indexing and length representation;
//! - element layout and representability;
//! - allocator behavior;
//! - process and operating-system resource availability.
//!
//! This module does not promise literal infinite physical storage.
//!
//! Fallible operations are provided for callers that need allocation
//! failures reported through `Result` instead of relying on infallible
//! allocation behavior.
//!
//! Capacity is a reservation request, not a semantic maximum.
//!
//! # Determinism
//!
//! `List<T>` preserves insertion order.
//!
//! `Map<K, V>` uses `HashMap`. Its iteration order is unspecified and
//! must not be relied upon for serialization, reproducible builds,
//! consensus, or deterministic computation.
//!
//! Use `crate::classical::collections::ClassicalMap` when deterministic
//! ordered-map behavior is required.
//!
//! # Failure and mutation guarantees
//!
//! - A failed `try_push` leaves the existing list unchanged.
//! - A failed `try_insert` leaves the existing list unchanged.
//! - A failed `try_insert` with an invalid index returns an error.
//! - `try_extend` is incremental: earlier successful additions remain
//!   if a later reservation fails.
//! - A failed map `try_insert` does not change existing entries.
//! - Map replacement does not require additional capacity.
//! - Map `try_extend` is incremental: earlier successful insertions and
//!   replacements remain if a later insertion fails.
//!
//! Iterators may have external side effects. Those effects cannot
//! generally be rolled back after an allocation failure.
//!
//! # Compatibility
//!
//! `crate::stdlib::collections` should re-export the public types here.
//! `crate::stdlib::core` should re-export the same `List` and `Map`
//! rather than define competing implementations.
//!
//! # Freeze rule
//!
//! Changes to ordering guarantees, duplicate handling, key equivalence,
//! allocation failure behavior, or public iterator types require
//! compatibility review and regression tests.

use core::borrow::Borrow;
use core::fmt;
use core::hash::Hash;
use core::ops::{Index, IndexMut};

use std::collections::hash_map;
use std::collections::HashMap;
use std::collections::TryReserveError;

// =============================================================================
// Standard collection interoperability
// =============================================================================

/// Standard-library collection types available without introducing
/// additional Zamani-specific storage implementations.
///
/// These are Rust standard-library types, not competing Zamani
/// collection authorities.
pub use std::collections::{
    BTreeMap,
    BTreeSet,
    BinaryHeap,
    HashSet,
    LinkedList,
    VecDeque,
};

// =============================================================================
// Error model
// =============================================================================

/// Identifies the general-purpose collection involved in an operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CollectionKind {
    /// Dynamically sized ordered list.
    List,

    /// Dynamically sized hash-based map.
    Map,
}

impl fmt::Display for CollectionKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::List => f.write_str("list"),
            Self::Map => f.write_str("map"),
        }
    }
}

/// Structured errors returned by fallible collection operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CollectionError {
    /// The underlying collection could not reserve the requested capacity.
    AllocationFailed {
        /// Collection whose allocation failed.
        collection: CollectionKind,

        /// Additional elements or entries requested.
        requested_additional: usize,

        /// Number of successfully processed input items before failure.
        ///
        /// For a single-item operation this is zero.
        /// For an incremental extension it counts successful additions
        /// or replacements performed by that operation.
        items_processed: usize,
    },

    /// A requested list index or insertion position is invalid.
    InvalidIndex {
        /// Collection whose operation received the invalid index.
        collection: CollectionKind,

        /// Requested index.
        index: usize,

        /// Collection length at validation time.
        len: usize,

        /// Whether the index is an insertion position.
        ///
        /// Insertion permits `index == len`.
        /// Element access and removal require `index < len`.
        insertion_position: bool,
    },
}

impl CollectionError {
    /// Returns the collection associated with the error.
    #[must_use]
    pub const fn collection(&self) -> CollectionKind {
        match self {
            Self::AllocationFailed { collection, .. }
            | Self::InvalidIndex { collection, .. } => *collection,
        }
    }

    /// Returns the additional capacity requested by an allocation error.
    ///
    /// Returns `None` for errors unrelated to allocation.
    #[must_use]
    pub const fn requested_additional(&self) -> Option<usize> {
        match self {
            Self::AllocationFailed {
                requested_additional,
                ..
            } => Some(*requested_additional),
            Self::InvalidIndex { .. } => None,
        }
    }

    /// Returns the number of input items processed before an error.
    ///
    /// Returns `None` when the error is not an allocation failure.
    #[must_use]
    pub const fn items_processed(&self) -> Option<usize> {
        match self {
            Self::AllocationFailed {
                items_processed,
                ..
            } => Some(*items_processed),
            Self::InvalidIndex { .. } => None,
        }
    }

    /// Creates an allocation failure with the correct collection identity.
    const fn allocation_failed(
        collection: CollectionKind,
        requested_additional: usize,
        items_processed: usize,
    ) -> Self {
        Self::AllocationFailed {
            collection,
            requested_additional,
            items_processed,
        }
    }

    /// Creates an invalid-index error.
    const fn invalid_index(
        collection: CollectionKind,
        index: usize,
        len: usize,
        insertion_position: bool,
    ) -> Self {
        Self::InvalidIndex {
            collection,
            index,
            len,
            insertion_position,
        }
    }
}

impl fmt::Display for CollectionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AllocationFailed {
                collection,
                requested_additional,
                items_processed,
            } => write!(
                f,
                "{collection} could not reserve capacity for \
                 {requested_additional} additional entries after \
                 processing {items_processed} input items"
            ),

            Self::InvalidIndex {
                collection,
                index,
                len,
                insertion_position,
            } => {
                if *insertion_position {
                    write!(
                        f,
                        "{collection} insertion position {index} \
                         is invalid for length {len}"
                    )
                } else {
                    write!(
                        f,
                        "{collection} index {index} is out of bounds \
                         for length {len}"
                    )
                }
            }
        }
    }
}

impl std::error::Error for CollectionError {}

// Deliberately do not implement From<TryReserveError>.
//
// A TryReserveError does not identify which collection produced it.
// An unconditional conversion would therefore risk reporting a map
// allocation failure as a list allocation failure, or vice versa.
// Each operation attaches its own collection identity instead.

// =============================================================================
// List
// =============================================================================

/// A dynamically sized, ordered, duplicate-preserving collection.
///
/// `List<T>` uses contiguous `Vec<T>` storage.
///
/// Its ordering and duplicate behavior are observable:
///
/// - insertion order is preserved;
/// - duplicate values are permitted;
/// - indexing is zero-based;
/// - `pop` removes the final element.
///
/// The collection does not impose a universal maximum length.
///
/// Use the `try_*` methods when allocation failures must be reported
/// explicitly.
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct List<T> {
    elements: Vec<T>,
}

impl<T> List<T> {
    // -------------------------------------------------------------------------
    // Construction
    // -------------------------------------------------------------------------

    /// Creates an empty list without allocating element storage.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }

    /// Creates an empty list and fallibly reserves capacity.
    ///
    /// Capacity is an allocation request, not a maximum length.
    pub fn try_with_capacity(
        capacity: usize,
    ) -> Result<Self, CollectionError> {
        let mut elements = Vec::new();

        elements.try_reserve_exact(capacity).map_err(|_| {
            CollectionError::allocation_failed(
                CollectionKind::List,
                capacity,
                0,
            )
        })?;

        Ok(Self { elements })
    }

    /// Wraps an existing vector without copying its elements.
    #[must_use]
    pub fn from_vec(elements: Vec<T>) -> Self {
        Self { elements }
    }

    /// Creates a list by consuming an iterator with fallible allocation.
    ///
    /// If allocation fails, the partially constructed list is dropped.
    /// The error reports how many elements were successfully appended.
    pub fn try_from_iter<I>(
        iterator: I,
    ) -> Result<Self, CollectionError>
    where
        I: IntoIterator<Item = T>,
    {
        let mut list = Self::new();
        list.try_extend(iterator)?;
        Ok(list)
    }

    /// Returns a cloned vector of the list's elements.
    #[must_use]
    pub fn to_vec(&self) -> Vec<T>
    where
        T: Clone,
    {
        self.elements.clone()
    }

    /// Consumes the list and returns its backing vector.
    #[must_use]
    pub fn into_vec(self) -> Vec<T> {
        self.elements
    }

    // -------------------------------------------------------------------------
    // Length and capacity
    // -------------------------------------------------------------------------

    /// Returns the number of stored elements.
    #[must_use]
    pub fn len(&self) -> usize {
        self.elements.len()
    }

    /// Returns whether the list contains no elements.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    /// Returns the currently allocated element capacity.
    ///
    /// Capacity is an implementation detail, not a semantic limit.
    #[must_use]
    pub fn capacity(&self) -> usize {
        self.elements.capacity()
    }

    /// Removes all elements while retaining allocated capacity.
    pub fn clear(&mut self) {
        self.elements.clear();
    }

    /// Removes all elements after `length`.
    ///
    /// Does nothing when `length` is greater than or equal to the
    /// current length.
    pub fn truncate(&mut self, length: usize) {
        self.elements.truncate(length);
    }

    // -------------------------------------------------------------------------
    // Fallible allocation
    // -------------------------------------------------------------------------

    /// Fallibly reserves capacity for at least `additional` elements.
    ///
    /// Existing contents remain unchanged if reservation fails.
    pub fn try_reserve(
        &mut self,
        additional: usize,
    ) -> Result<(), CollectionError> {
        self.elements.try_reserve(additional).map_err(|_| {
            CollectionError::allocation_failed(
                CollectionKind::List,
                additional,
                0,
            )
        })
    }

    /// Fallibly reserves capacity for at least `additional` elements,
    /// requesting an exact reservation where supported.
    pub fn try_reserve_exact(
        &mut self,
        additional: usize,
    ) -> Result<(), CollectionError> {
        self.elements
            .try_reserve_exact(additional)
            .map_err(|_| {
                CollectionError::allocation_failed(
                    CollectionKind::List,
                    additional,
                    0,
                )
            })
    }

    // -------------------------------------------------------------------------
    // Element access
    // -------------------------------------------------------------------------

    /// Returns a reference to an element, if the index exists.
    #[must_use]
    pub fn get(&self, index: usize) -> Option<&T> {
        self.elements.get(index)
    }

    /// Returns a mutable reference to an element, if the index exists.
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.elements.get_mut(index)
    }

    /// Returns the first element, if present.
    #[must_use]
    pub fn first(&self) -> Option<&T> {
        self.elements.first()
    }

    /// Returns the first element mutably, if present.
    pub fn first_mut(&mut self) -> Option<&mut T> {
        self.elements.first_mut()
    }

    /// Returns the final element, if present.
    #[must_use]
    pub fn last(&self) -> Option<&T> {
        self.elements.last()
    }

    /// Returns the final element mutably, if present.
    pub fn last_mut(&mut self) -> Option<&mut T> {
        self.elements.last_mut()
    }

    /// Returns the elements as a borrowed slice.
    #[must_use]
    pub fn as_slice(&self) -> &[T] {
        self.elements.as_slice()
    }

    /// Returns the elements as a mutable borrowed slice.
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        self.elements.as_mut_slice()
    }

    // -------------------------------------------------------------------------
    // Insertion
    // -------------------------------------------------------------------------

    /// Appends an element.
    ///
    /// This is an infallible convenience operation following Rust
    /// collection conventions. Use `try_push` when allocation failure
    /// must be reported through `Result`.
    pub fn push(&mut self, value: T) {
        self.elements.push(value);
    }

    /// Fallibly appends an element.
    ///
    /// If additional capacity cannot be reserved, existing contents
    /// remain unchanged. The supplied value is dropped on failure.
    pub fn try_push(
        &mut self,
        value: T,
    ) -> Result<(), CollectionError> {
        self.elements.try_reserve(1).map_err(|_| {
            CollectionError::allocation_failed(
                CollectionKind::List,
                1,
                0,
            )
        })?;

        self.elements.push(value);
        Ok(())
    }

    /// Inserts an element at `index`.
    ///
    /// Panics if `index > len()`, following `Vec::insert` semantics.
    /// Use `try_insert` when invalid positions must be reported as errors.
    pub fn insert(
        &mut self,
        index: usize,
        value: T,
    ) {
        self.elements.insert(index, value);
    }

    /// Fallibly inserts an element at `index`.
    ///
    /// Valid insertion positions are `0..=len()`.
    ///
    /// Invalid positions return `CollectionError::InvalidIndex`.
    /// Allocation failure leaves existing contents unchanged.
    pub fn try_insert(
        &mut self,
        index: usize,
        value: T,
    ) -> Result<(), CollectionError> {
        let len = self.elements.len();

        if index > len {
            return Err(CollectionError::invalid_index(
                CollectionKind::List,
                index,
                len,
                true,
            ));
        }

        self.elements.try_reserve(1).map_err(|_| {
            CollectionError::allocation_failed(
                CollectionKind::List,
                1,
                0,
            )
        })?;

        self.elements.insert(index, value);
        Ok(())
    }

    /// Appends all elements from an iterator using ordinary allocation.
    pub fn extend<I>(&mut self, iterator: I)
    where
        I: IntoIterator<Item = T>,
    {
        self.elements.extend(iterator);
    }

    /// Fallibly appends elements from an iterator.
    ///
    /// The operation is incremental. Previously appended elements remain
    /// if a later reservation fails.
    ///
    /// The iterator's lower size hint is used as an initial reservation
    /// request. Its upper size hint is not treated as a semantic maximum.
    ///
    /// The returned error reports the number of successfully appended
    /// elements from this operation.
    pub fn try_extend<I>(
        &mut self,
        iterator: I,
    ) -> Result<(), CollectionError>
    where
        I: IntoIterator<Item = T>,
    {
        let mut iterator = iterator.into_iter();
        let (lower_bound, _) = iterator.size_hint();

        self.elements
            .try_reserve(lower_bound)
            .map_err(|_| {
                CollectionError::allocation_failed(
                    CollectionKind::List,
                    lower_bound,
                    0,
                )
            })?;

        let mut processed = 0usize;

        for item in iterator {
            self.elements.try_reserve(1).map_err(|_| {
                CollectionError::allocation_failed(
                    CollectionKind::List,
                    1,
                    processed,
                )
            })?;

            self.elements.push(item);
            processed = processed.saturating_add(1);
        }

        Ok(())
    }

    // -------------------------------------------------------------------------
    // Removal
    // -------------------------------------------------------------------------

    /// Removes and returns the final element, if present.
    pub fn pop(&mut self) -> Option<T> {
        self.elements.pop()
    }

    /// Removes and returns the element at `index`.
    ///
    /// Returns `None` if the index is out of bounds.
    pub fn remove(&mut self, index: usize) -> Option<T> {
        if index >= self.elements.len() {
            return None;
        }

        Some(self.elements.remove(index))
    }

    /// Removes and returns an element without preserving order.
    ///
    /// The final element is moved into the removed position when needed.
    pub fn swap_remove(&mut self, index: usize) -> Option<T> {
        if index >= self.elements.len() {
            return None;
        }

        Some(self.elements.swap_remove(index))
    }

    /// Retains only elements for which `keep` returns true.
    pub fn retain<F>(&mut self, keep: F)
    where
        F: FnMut(&T) -> bool,
    {
        self.elements.retain(keep);
    }

    // -------------------------------------------------------------------------
    // Iteration
    // -------------------------------------------------------------------------

    /// Returns an immutable iterator over elements.
    pub fn iter(&self) -> core::slice::Iter<'_, T> {
        self.elements.iter()
    }

    /// Returns a mutable iterator over elements.
    pub fn iter_mut(&mut self) -> core::slice::IterMut<'_, T> {
        self.elements.iter_mut()
    }

    /// Returns an owning iterator over elements.
    pub fn into_iter(self) -> std::vec::IntoIter<T> {
        self.elements.into_iter()
    }

    /// Returns an iterator over references to the elements.
    pub fn values(&self) -> core::slice::Iter<'_, T> {
        self.iter()
    }

    /// Returns a mutable iterator over the elements.
    pub fn values_mut(&mut self) -> core::slice::IterMut<'_, T> {
        self.iter_mut()
    }
}

impl<T> From<Vec<T>> for List<T> {
    fn from(elements: Vec<T>) -> Self {
        Self::from_vec(elements)
    }
}

impl<T> From<List<T>> for Vec<T> {
    fn from(list: List<T>) -> Self {
        list.into_vec()
    }
}

impl<T> FromIterator<T> for List<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iterator: I) -> Self {
        Self {
            elements: iterator.into_iter().collect(),
        }
    }
}

impl<T> Extend<T> for List<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iterator: I) {
        self.elements.extend(iterator);
    }
}

impl<T> IntoIterator for List<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.elements.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a List<T> {
    type Item = &'a T;
    type IntoIter = core::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.elements.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut List<T> {
    type Item = &'a mut T;
    type IntoIter = core::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.elements.iter_mut()
    }
}

impl<T> Index<usize> for List<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.elements[index]
    }
}

impl<T> IndexMut<usize> for List<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.elements[index]
    }
}

// =============================================================================
// Map
// =============================================================================

/// A dynamically sized hash-based map.
///
/// `Map<K, V>` maps unique keys to values.
///
/// Key requirements:
///
/// - `K: Eq + Hash` for operations that insert or look up keys;
/// - equal keys must produce equal hashes;
/// - key equality and hash behavior must not change while a key is stored.
///
/// Iteration order is unspecified. Use `ClassicalMap` when deterministic
/// key ordering is required.
///
/// The map does not impose an arbitrary universal entry-count limit.
#[derive(Debug, Clone)]
pub struct Map<K, V> {
    entries: HashMap<K, V>,
}

impl<K: Eq + Hash, V: PartialEq> PartialEq for Map<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.entries == other.entries
    }
}

impl<K: Eq + Hash, V: Eq> Eq for Map<K, V> {}

impl<K, V> Map<K, V> {
    // -------------------------------------------------------------------------
    // Construction
    // -------------------------------------------------------------------------

    /// Creates an empty map.
    #[must_use]
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    /// Creates an empty map and fallibly reserves capacity.
    ///
    /// Capacity is an allocation request, not a semantic maximum.
    pub fn try_with_capacity(
        capacity: usize,
    ) -> Result<Self, CollectionError> {
        let mut entries = HashMap::new();

        entries.try_reserve(capacity).map_err(|_| {
            CollectionError::allocation_failed(
                CollectionKind::Map,
                capacity,
                0,
            )
        })?;

        Ok(Self { entries })
    }

    /// Creates a map by consuming an iterator with fallible allocation.
    ///
    /// Duplicate keys replace earlier values.
    /// If allocation fails, the partially constructed map is dropped.
    pub fn try_from_iter<I>(
        iterator: I,
    ) -> Result<Self, CollectionError>
    where
        K: Eq + Hash,
        I: IntoIterator<Item = (K, V)>,
    {
        let mut map = Self::new();
        map.try_extend(iterator)?;
        Ok(map)
    }

    /// Consumes the map and returns its underlying `HashMap`.
    #[must_use]
    pub fn into_hash_map(self) -> HashMap<K, V> {
        self.entries
    }

    /// Returns a reference to the underlying `HashMap`.
    #[must_use]
    pub fn as_hash_map(&self) -> &HashMap<K, V> {
        &self.entries
    }

    // -------------------------------------------------------------------------
    // Length and capacity
    // -------------------------------------------------------------------------

    /// Returns the number of entries.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns whether the map is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Returns the currently allocated entry capacity.
    #[must_use]
    pub fn capacity(&self) -> usize {
        self.entries.capacity()
    }

    /// Removes all entries while retaining allocated capacity.
    pub fn clear(&mut self) {
        self.entries.clear();
    }

    // -------------------------------------------------------------------------
    // Fallible allocation
    // -------------------------------------------------------------------------

    /// Fallibly reserves capacity for at least `additional` entries.
    pub fn try_reserve(
        &mut self,
        additional: usize,
    ) -> Result<(), CollectionError> {
        self.entries.try_reserve(additional).map_err(|_| {
            CollectionError::allocation_failed(
                CollectionKind::Map,
                additional,
                0,
            )
        })
    }

    /// Fallibly reserves capacity for at least `additional` entries,
    /// requesting an exact reservation where supported.
    pub fn try_reserve_exact(
        &mut self,
        additional: usize,
    ) -> Result<(), CollectionError> {
        // HashMap exposes try_reserve rather than a separate exact-reserve
        // operation. This method preserves a consistent collection API;
        // the implementation uses the allocator strategy provided by
        // the standard HashMap implementation.
        self.try_reserve(additional)
    }

    // -------------------------------------------------------------------------
    // Lookup
    // -------------------------------------------------------------------------

    /// Returns a reference to the value associated with `key`.
    ///
    /// Borrowed lookup is supported when `K` implements `Borrow<Q>` and
    /// the borrowed type has matching equality and hashing behavior.
    #[must_use]
    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Eq + Hash + Borrow<Q>,
        Q: Eq + Hash + ?Sized,
    {
        self.entries.get(key)
    }

    /// Returns a mutable reference to the value associated with `key`.
    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
    where
        K: Eq + Hash + Borrow<Q>,
        Q: Eq + Hash + ?Sized,
    {
        self.entries.get_mut(key)
    }

    /// Returns whether the map contains `key`.
    #[must_use]
    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        K: Eq + Hash + Borrow<Q>,
        Q: Eq + Hash + ?Sized,
    {
        self.entries.contains_key(key)
    }

    // -------------------------------------------------------------------------
    // Insertion and replacement
    // -------------------------------------------------------------------------

    /// Inserts or replaces an entry.
    ///
    /// Returns the previous value if the key already existed.
    ///
    /// Use `try_insert` when allocation failure must be reported.
    pub fn insert(
        &mut self,
        key: K,
        value: V,
    ) -> Option<V>
    where
        K: Eq + Hash,
    {
        self.entries.insert(key, value)
    }

    /// Fallibly inserts or replaces an entry.
    ///
    /// Replacing an existing value does not require additional capacity.
    /// A new key is inserted only after capacity has been reserved.
    ///
    /// Existing entries remain unchanged if reservation fails.
    pub fn try_insert(
        &mut self,
        key: K,
        value: V,
    ) -> Result<Option<V>, CollectionError>
    where
        K: Eq + Hash,
    {
        if let Some(existing) = self.entries.get_mut(&key) {
            return Ok(Some(core::mem::replace(existing, value)));
        }

        self.entries.try_reserve(1).map_err(|_| {
            CollectionError::allocation_failed(
                CollectionKind::Map,
                1,
                0,
            )
        })?;

        Ok(self.entries.insert(key, value))
    }

    /// Builder-style insertion.
    ///
    /// Returns the map so insertions can be chained.
    pub fn with(
        mut self,
        key: K,
        value: V,
    ) -> Self
    where
        K: Eq + Hash,
    {
        self.insert(key, value);
        self
    }

    /// Fallible builder-style insertion.
    pub fn try_with(
        mut self,
        key: K,
        value: V,
    ) -> Result<Self, CollectionError>
    where
        K: Eq + Hash,
    {
        self.try_insert(key, value)?;
        Ok(self)
    }

    /// Extends the map from an iterator using ordinary allocation.
    ///
    /// Duplicate keys replace earlier values.
    pub fn extend<I>(&mut self, iterator: I)
    where
        K: Eq + Hash,
        I: IntoIterator<Item = (K, V)>,
    {
        self.entries.extend(iterator);
    }

    /// Fallibly extends the map from an iterator.
    ///
    /// This operation is incremental. Earlier successful insertions
    /// and replacements remain if a later insertion fails.
    ///
    /// `items_processed` counts successful input entries, including
    /// replacements of existing keys.
    pub fn try_extend<I>(
        &mut self,
        iterator: I,
    ) -> Result<(), CollectionError>
    where
        K: Eq + Hash,
        I: IntoIterator<Item = (K, V)>,
    {
        let mut processed = 0usize;

        for (key, value) in iterator {
            if let Err(error) = self.try_insert(key, value) {
                return Err(match error {
                    CollectionError::AllocationFailed {
                        collection,
                        requested_additional,
                        ..
                    } => CollectionError::allocation_failed(
                        collection,
                        requested_additional,
                        processed,
                    ),
                    other => other,
                });
            }

            processed = processed.saturating_add(1);
        }

        Ok(())
    }

    // -------------------------------------------------------------------------
    // Removal
    // -------------------------------------------------------------------------

    /// Removes an entry and returns its value, if present.
    pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        K: Eq + Hash + Borrow<Q>,
        Q: Eq + Hash + ?Sized,
    {
        self.entries.remove(key)
    }

    /// Removes an entry and returns its stored key and value.
    pub fn remove_entry<Q>(
        &mut self,
        key: &Q,
    ) -> Option<(K, V)>
    where
        K: Eq + Hash + Borrow<Q>,
        Q: Eq + Hash + ?Sized,
    {
        self.entries.remove_entry(key)
    }

    /// Retains only entries for which `keep` returns true.
    ///
    /// Keys are immutable through the callback, preserving hash-map
    /// key invariants.
    pub fn retain<F>(&mut self, keep: F)
    where
        K: Eq + Hash,
        F: FnMut(&K, &mut V) -> bool,
    {
        self.entries.retain(keep);
    }

    // -------------------------------------------------------------------------
    // Iteration
    // -------------------------------------------------------------------------

    /// Returns an iterator over key/value pairs.
    ///
    /// Iteration order is unspecified.
    pub fn iter(&self) -> hash_map::Iter<'_, K, V> {
        self.entries.iter()
    }

    /// Returns a mutable iterator over key/value pairs.
    ///
    /// Keys cannot be mutated through this iterator.
    pub fn iter_mut(&mut self) -> hash_map::IterMut<'_, K, V> {
        self.entries.iter_mut()
    }

    /// Returns an iterator over keys.
    pub fn keys(&self) -> hash_map::Keys<'_, K, V> {
        self.entries.keys()
    }

    /// Returns an iterator over values.
    pub fn values(&self) -> hash_map::Values<'_, K, V> {
        self.entries.values()
    }

    /// Returns a mutable iterator over values.
    pub fn values_mut(&mut self) -> hash_map::ValuesMut<'_, K, V> {
        self.entries.values_mut()
    }
}

impl<K, V> Default for Map<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: Eq + Hash, V> From<HashMap<K, V>> for Map<K, V> {
    fn from(entries: HashMap<K, V>) -> Self {
        Self { entries }
    }
}

impl<K, V> From<Map<K, V>> for HashMap<K, V> {
    fn from(map: Map<K, V>) -> Self {
        map.entries
    }
}

impl<K: Eq + Hash, V> FromIterator<(K, V)> for Map<K, V> {
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iterator: I) -> Self {
        Self {
            entries: iterator.into_iter().collect(),
        }
    }
}

impl<K: Eq + Hash, V> Extend<(K, V)> for Map<K, V> {
    fn extend<I: IntoIterator<Item = (K, V)>>(&mut self, iterator: I) {
        self.entries.extend(iterator);
    }
}

impl<K: Eq + Hash, V> IntoIterator for Map<K, V> {
    type Item = (K, V);
    type IntoIter = hash_map::IntoIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.entries.into_iter()
    }
}

impl<'a, K, V> IntoIterator for &'a Map<K, V> {
    type Item = (&'a K, &'a V);
    type IntoIter = hash_map::Iter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.entries.iter()
    }
}

impl<'a, K, V> IntoIterator for &'a mut Map<K, V> {
    type Item = (&'a K, &'a mut V);
    type IntoIter = hash_map::IterMut<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.entries.iter_mut()
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::{
        CollectionError,
        CollectionKind,
        List,
        Map,
    };

    use std::collections::HashMap;

    // -------------------------------------------------------------------------
    // List tests
    // -------------------------------------------------------------------------

    #[test]
    fn list_starts_empty() {
        let list = List::<u32>::new();

        assert!(list.is_empty());
        assert_eq!(list.len(), 0);
        assert_eq!(list.capacity(), 0);
    }

    #[test]
    fn list_preserves_order_and_duplicates() {
        let mut list = List::new();

        list.push(4);
        list.push(4);
        list.push(2);
        list.push(4);

        assert_eq!(list.as_slice(), &[4, 4, 2, 4]);
    }

    #[test]
    fn list_supports_indexed_access() {
        let mut list = List::from(vec![10, 20, 30]);

        assert_eq!(list.get(1), Some(&20));

        list[1] = 25;

        assert_eq!(list[1], 25);
        assert_eq!(list.get(9), None);
    }

    #[test]
    fn list_supports_borrowed_iteration() {
        let list = List::from(vec![1, 2, 3]);

        let values: Vec<_> = list.iter().copied().collect();

        assert_eq!(values, vec![1, 2, 3]);
    }

    #[test]
    fn list_supports_mutable_iteration() {
        let mut list = List::from(vec![1, 2, 3]);

        for value in &mut list {
            *value *= 2;
        }

        assert_eq!(list.as_slice(), &[2, 4, 6]);
    }

    #[test]
    fn list_supports_owned_iteration() {
        let list = List::from(vec![1, 2, 3]);

        let values: Vec<_> = list.into_iter().collect();

        assert_eq!(values, vec![1, 2, 3]);
    }

    #[test]
    fn list_supports_vector_conversion() {
        let list = List::from(vec![1, 2, 3]);

        let values: Vec<_> = list.into();

        assert_eq!(values, vec![1, 2, 3]);
    }

    #[test]
    fn list_try_push_and_try_reserve_work() {
        let mut list = List::try_with_capacity(2).unwrap();

        list.try_push(10).unwrap();
        list.try_push(20).unwrap();
        list.try_reserve(4).unwrap();

        assert_eq!(list.as_slice(), &[10, 20]);
        assert!(list.capacity() >= list.len());
    }

    #[test]
    fn list_try_insert_preserves_order() {
        let mut list = List::from(vec![1, 3]);

        list.try_insert(1, 2).unwrap();

        assert_eq!(list.as_slice(), &[1, 2, 3]);
    }

    #[test]
    fn list_try_insert_rejects_invalid_position() {
        let mut list = List::from(vec![1, 2]);

        let error = list.try_insert(4, 3).unwrap_err();

        assert_eq!(
            error,
            CollectionError::InvalidIndex {
                collection: CollectionKind::List,
                index: 4,
                len: 2,
                insertion_position: true,
            }
        );

        assert_eq!(list.as_slice(), &[1, 2]);
    }

    #[test]
    fn list_removal_operations_work() {
        let mut list = List::from(vec![10, 20, 30, 40]);

        assert_eq!(list.remove(1), Some(20));
        assert_eq!(list.pop(), Some(40));
        assert_eq!(list.remove(20), None);

        assert_eq!(list.as_slice(), &[10, 30]);
    }

    #[test]
    fn list_swap_remove_does_not_promise_order() {
        let mut list = List::from(vec![10, 20, 30]);

        assert_eq!(list.swap_remove(0), Some(10));
        assert_eq!(list.len(), 2);
        assert!(list.as_slice().contains(&20));
        assert!(list.as_slice().contains(&30));
    }

    #[test]
    fn list_try_extend_supports_unknown_upper_size_hint() {
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

        let mut list = List::new();

        list.try_extend(UnknownUpper {
            next: 0,
            end: 100,
        })
        .unwrap();

        assert_eq!(list.len(), 100);
        assert_eq!(list.first(), Some(&0));
        assert_eq!(list.last(), Some(&99));
    }

    #[test]
    fn list_try_from_iter_works() {
        let list = List::try_from_iter(0..5).unwrap();

        assert_eq!(list.as_slice(), &[0, 1, 2, 3, 4]);
    }

    #[test]
    fn list_retain_and_clear_work() {
        let mut list = List::from(vec![1, 2, 3, 4]);

        list.retain(|value| value % 2 == 0);

        assert_eq!(list.as_slice(), &[2, 4]);

        list.clear();

        assert!(list.is_empty());
    }

    // -------------------------------------------------------------------------
    // Map tests
    // -------------------------------------------------------------------------

    #[test]
    fn map_starts_empty() {
        let map = Map::<String, u32>::new();

        assert!(map.is_empty());
        assert_eq!(map.len(), 0);
    }

    #[test]
    fn map_inserts_and_retrieves_values() {
        let mut map = Map::new();

        assert_eq!(map.insert("answer", 42), None);
        assert_eq!(map.get("answer"), Some(&42));
        assert!(map.contains_key("answer"));
        assert_eq!(map.len(), 1);
    }

    #[test]
    fn map_replaces_existing_values() {
        let mut map = Map::new();

        assert_eq!(map.insert("answer", 41), None);
        assert_eq!(map.insert("answer", 42), Some(41));
        assert_eq!(map.get("answer"), Some(&42));
        assert_eq!(map.len(), 1);
    }

    #[test]
    fn map_supports_borrowed_string_lookup() {
        let mut map = Map::new();

        map.insert(String::from("alpha"), 1);

        assert_eq!(map.get("alpha"), Some(&1));
        assert_eq!(map.get("missing"), None);
        assert!(map.contains_key("alpha"));
    }

    #[test]
    fn map_supports_mutable_lookup() {
        let mut map = Map::new();

        map.insert("count", 1);
        *map.get_mut("count").unwrap() = 2;

        assert_eq!(map.get("count"), Some(&2));
    }

    #[test]
    fn map_try_insert_and_try_reserve_work() {
        let mut map = Map::try_with_capacity(2).unwrap();

        map.try_insert("one", 1).unwrap();
        map.try_insert("two", 2).unwrap();
        map.try_reserve(4).unwrap();

        assert_eq!(map.len(), 2);
        assert_eq!(map.get("two"), Some(&2));
    }

    #[test]
    fn map_try_insert_replaces_without_growing() {
        let mut map = Map::new();

        map.try_insert("key", 1).unwrap();

        assert_eq!(map.try_insert("key", 2).unwrap(), Some(1));
        assert_eq!(map.get("key"), Some(&2));
        assert_eq!(map.len(), 1);
    }

    #[test]
    fn map_try_extend_replaces_duplicate_keys() {
        let mut map = Map::new();

        map.try_extend([
            ("one", 1),
            ("two", 2),
            ("one", 3),
        ])
        .unwrap();

        assert_eq!(map.len(), 2);
        assert_eq!(map.get("one"), Some(&3));
    }

    #[test]
    fn map_try_from_iter_works() {
        let map = Map::try_from_iter([
            ("one", 1),
            ("two", 2),
        ])
        .unwrap();

        assert_eq!(map.get("one"), Some(&1));
        assert_eq!(map.get("two"), Some(&2));
    }

    #[test]
    fn map_supports_builder_insertion() {
        let map = Map::new()
            .with("one", 1)
            .with("two", 2);

        assert_eq!(map.get("one"), Some(&1));
        assert_eq!(map.get("two"), Some(&2));
    }

    #[test]
    fn map_supports_fallible_builder_insertion() {
        let map = Map::new()
            .try_with("one", 1)
            .unwrap()
            .try_with("two", 2)
            .unwrap();

        assert_eq!(map.len(), 2);
    }

    #[test]
    fn map_supports_iteration() {
        let mut map = Map::new();

        map.insert("one", 1);
        map.insert("two", 2);

        let mut values: Vec<_> = map.values().copied().collect();
        values.sort_unstable();

        assert_eq!(values, vec![1, 2]);
    }

    #[test]
    fn map_supports_mutable_iteration() {
        let mut map = Map::new();

        map.insert("one", 1);
        map.insert("two", 2);

        for value in map.values_mut() {
            *value += 10;
        }

        assert_eq!(map.get("one"), Some(&11));
        assert_eq!(map.get("two"), Some(&12));
    }

    #[test]
    fn map_supports_removal() {
        let mut map = Map::new();

        map.insert("one", 1);

        assert_eq!(map.remove("one"), Some(1));
        assert_eq!(map.remove("missing"), None);
        assert!(map.is_empty());
    }

    #[test]
    fn map_supports_remove_entry() {
        let mut map = Map::new();

        map.insert(String::from("key"), 7);

        assert_eq!(
            map.remove_entry("key"),
            Some((String::from("key"), 7))
        );
    }

    #[test]
    fn map_supports_retain() {
        let mut map = Map::new();

        map.insert("one", 1);
        map.insert("two", 2);
        map.insert("three", 3);

        map.retain(|_, value| *value % 2 == 1);

        assert_eq!(map.len(), 2);
        assert!(map.contains_key("one"));
        assert!(!map.contains_key("two"));
        assert!(map.contains_key("three"));
    }

    #[test]
    fn map_interoperates_with_hash_map() {
        let mut source = HashMap::new();
        source.insert("one", 1);
        source.insert("two", 2);

        let map = Map::from(source);
        let recovered: HashMap<_, _> = map.into();

        assert_eq!(recovered.len(), 2);
        assert_eq!(recovered.get("one"), Some(&1));
    }

    #[test]
    fn map_iteration_does_not_assume_order() {
        let map = Map::from_iter([
            ("c", 3),
            ("a", 1),
            ("b", 2),
        ]);

        let mut pairs: Vec<_> = map
            .iter()
            .map(|(key, value)| (*key, *value))
            .collect();

        pairs.sort_unstable();

        assert_eq!(
            pairs,
            vec![("a", 1), ("b", 2), ("c", 3)]
        );
    }

    // -------------------------------------------------------------------------
    // Error tests
    // -------------------------------------------------------------------------

    #[test]
    fn allocation_error_exposes_machine_readable_information() {
        let error = CollectionError::allocation_failed(
            CollectionKind::Map,
            8,
            3,
        );

        assert_eq!(error.collection(), CollectionKind::Map);
        assert_eq!(error.requested_additional(), Some(8));
        assert_eq!(error.items_processed(), Some(3));

        assert!(error.to_string().contains("map"));
        assert!(error.to_string().contains("8"));
    }

    #[test]
    fn invalid_index_is_structured() {
        let error = CollectionError::invalid_index(
            CollectionKind::List,
            7,
            3,
            false,
        );

        assert_eq!(error.collection(), CollectionKind::List);
        assert_eq!(error.requested_additional(), None);
        assert_eq!(error.items_processed(), None);
        assert!(error.to_string().contains("out of bounds"));
    }

    #[test]
    fn collection_errors_implement_standard_error() {
        fn accepts_error(_: &dyn std::error::Error) {}

        let error = CollectionError::allocation_failed(
            CollectionKind::List,
            1,
            0,
        );

        accepts_error(&error);
    }

    #[test]
    fn standard_hash_set_remains_available() {
        let mut values = super::HashSet::new();

        values.insert(10);
        values.insert(10);
        values.insert(20);

        assert_eq!(values.len(), 2);
    }
}
