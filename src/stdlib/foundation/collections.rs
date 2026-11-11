
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani foundational collections
//!
//! Path: `src/foundation/collections.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: safe Rust only.
//!
//! ## Purpose
//!
//! Provide generic, platform-independent collection abstractions for
//! foundational Zamani APIs.
//!
//! This module supplies:
//!
//! - `List<T>`: dynamically sized, ordered, duplicate-preserving storage.
//! - `Map<K, V>`: dynamically sized hash-based key/value storage.
//! - `CollectionError`: structured allocation failures.
//! - `CollectionKind`: identifies the affected collection.
//!
//! ## Ownership boundaries
//!
//! This module does not own:
//!
//! - Zamani source syntax, AST, or semantic types;
//! - classical array, vector, matrix, tensor, sequence, map, or set semantics;
//! - quantum registers, states, or quantum IR;
//! - HDL signals, registers, or hardware storage;
//! - distributed storage, sharding, replication, or coordination;
//! - runtime allocation policy, scheduling, or device selection;
//! - compiler IR or backend representations.
//!
//! The classical collection subsystem remains authoritative for its
//! specialized collection types.
//!
//! ## Scalability
//!
//! No arbitrary collection-length or capacity ceiling is imposed.
//!
//! A collection grows according to its allocator and available resources.
//! Its concrete representation remains subject to:
//!
//! - Rust's `usize` indexing representation;
//! - the collection's element layout;
//! - allocator behavior;
//! - process and operating-system resource availability.
//!
//! This does not promise literal infinite physical memory.
//!
//! `try_reserve`, `try_reserve_exact`, `try_push`, `try_insert`,
//! and `try_extend` expose allocation failures where practical.
//!
//! Infallible convenience methods are also supplied for interoperability
//! with Rust's standard collection conventions. They may panic or abort
//! under allocator failure, as ordinary standard-library allocation APIs
//! can do. Callers requiring explicit failure handling should use the
//! fallible alternatives.
//!
//! ## Determinism
//!
//! `List<T>` preserves insertion order.
//!
//! `Map<K, V>` uses `HashMap` and does not promise iteration order.
//! Its iteration order may vary between instances and executions.
//!
//! Consumers requiring deterministic key ordering must use the dedicated
//! `ClassicalMap<K, V>` implementation or another explicitly ordered
//! collection.
//!
//! ## Equality and hashing
//!
//! Map keys must satisfy Rust's `Eq` and `Hash` contracts.
//! Equal keys must produce equal hashes.
//!
//! Mutating a stored key in a way that changes its equality or hash
//! behavior while it is in the map is a logic error.
//!
//! ## Integration
//!
//! `src/foundation/mod.rs` exports this module.
//!
//! `src/lib.rs` exposes the `foundation` module.
//!
//! `src/stdlib/collections.rs` re-exports these canonical types for
//! compatibility with existing standard-library consumers.
//!
//! `src/stdlib/core.rs` must stop defining competing `List` and `Map`
//! types and re-export these canonical types instead.
//!
//! `src/classical/collections/` retains ownership of specialized
//! classical collection implementations.
//!
//! Grammar and semantic analysis remain separate authorities.
//!
//! ## Resource and failure semantics
//!
//! Fallible operations report reservation failures without silently
//! truncating a collection.
//!
//! `try_extend` is incremental: successful earlier insertions remain
//! if a later reservation fails.
//!
//! The error reports the number of successfully processed input items.
//!
//! Iterators may have side effects. Those effects cannot generally be
//! rolled back after an allocation failure.
//!
//! ## Freeze rule
//!
//! Changes to ordering guarantees, key equivalence, duplicate handling,
//! allocation failure semantics, or public iterator types require
//! compatibility review and regression tests.

use core::borrow::Borrow;
use core::fmt;
use core::hash::Hash;
use core::iter::FusedIterator;
use core::ops::{Index, IndexMut};

use std::collections::{
    hash_map,
    HashMap,
    HashSet,
};
use std::collections::TryReserveError;

// =============================================================================
// Error model
// =============================================================================

/// Identifies the generic collection involved in an allocation failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CollectionKind {
    /// Dynamically sized list.
    List,

    /// Hash-based map.
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

/// Errors produced by fallible foundational collection operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CollectionError {
    /// The underlying collection could not reserve the requested capacity.
    AllocationFailed {
        /// Collection whose allocation failed.
        collection: CollectionKind,

        /// Additional elements or entries requested.
        requested_additional: usize,

        /// Successfully processed input elements or entries before failure.
        ///
        /// This is diagnostic information. It does not contain the
        /// partially constructed collection.
        items_processed: usize,
    },
}

impl CollectionError {
    /// Returns the collection associated with the failure.
    #[must_use]
    pub const fn collection(&self) -> CollectionKind {
        match self {
            Self::AllocationFailed { collection, .. } => *collection,
        }
    }

    /// Returns the additional capacity requested by the failed operation.
    #[must_use]
    pub const fn requested_additional(&self) -> usize {
        match self {
            Self::AllocationFailed {
                requested_additional,
                ..
            } => *requested_additional,
        }
    }

    /// Returns the number of successfully processed input items.
    #[must_use]
    pub const fn items_processed(&self) -> usize {
        match self {
            Self::AllocationFailed {
                items_processed,
                ..
            } => *items_processed,
        }
    }

    fn allocation_failed(
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
                "{collection} allocation failed while requesting \
                 {requested_additional} additional entries after \
                 processing {items_processed} input items"
            ),
        }
    }
}

impl std::error::Error for CollectionError {}

impl From<TryReserveError> for CollectionError {
    fn from(_: TryReserveError) -> Self {
        Self::allocation_failed(
            CollectionKind::List,
            0,
            0,
        )
    }
}

// =============================================================================
// List
// =============================================================================

/// A dynamically sized, ordered, duplicate-preserving collection.
///
/// `List<T>` uses contiguous `Vec<T>` storage.
///
/// It preserves insertion order and supports indexed access.
///
/// It does not impose an arbitrary maximum length.
///
/// Use the `try_*` methods when allocation failure must be reported
/// through a `Result`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct List<T> {
    elements: Vec<T>,
}

impl<T> List<T> {
    // -------------------------------------------------------------------------
    // Construction
    // -------------------------------------------------------------------------

    /// Creates an empty list.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }

    /// Creates an empty list and attempts to reserve the requested capacity.
    ///
    /// Capacity is an allocation request, not a semantic maximum.
    pub fn try_with_capacity(
        capacity: usize,
    ) -> Result<Self, CollectionError> {
        let mut elements = Vec::new();

        elements
            .try_reserve_exact(capacity)
            .map_err(|_| {
                CollectionError::allocation_failed(
                    CollectionKind::List,
                    capacity,
                    0,
                )
            })?;

        Ok(Self { elements })
    }

    /// Creates a list from an existing vector without copying its elements.
    #[must_use]
    pub fn from_vec(elements: Vec<T>) -> Self {
        Self { elements }
    }

    /// Creates a list by consuming an iterator with fallible allocation.
    ///
    /// The iterator's lower size hint is used as an initial reservation
    /// request. Its upper size hint is not treated as a maximum.
    ///
    /// If a later reservation fails, the temporary list is dropped and
    /// the error reports how many items were processed before failure.
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

    /// Returns a copy of the list's elements as a vector.
    ///
    /// This requires cloning each element.
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

    /// Returns true when the list contains no elements.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    /// Returns the currently allocated element capacity.
    ///
    /// Capacity is an implementation detail and not a language-level limit.
    #[must_use]
    pub fn capacity(&self) -> usize {
        self.elements.capacity()
    }

    /// Removes every element while retaining allocated capacity.
    pub fn clear(&mut self) {
        self.elements.clear();
    }

    /// Removes elements after the specified length.
    ///
    /// If `length` is greater than the current length, this does nothing.
    pub fn truncate(&mut self, length: usize) {
        self.elements.truncate(length);
    }

    // -------------------------------------------------------------------------
    // Allocation
    // -------------------------------------------------------------------------

    /// Fallibly reserves capacity for at least `additional` elements.
    ///
    /// The list's logical contents remain unchanged if reservation fails.
    pub fn try_reserve(
        &mut self,
        additional: usize,
    ) -> Result<(), CollectionError> {
        self.elements
            .try_reserve(additional)
            .map_err(|_| {
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

    /// Returns a reference to an element at `index`, if present.
    #[must_use]
    pub fn get(&self, index: usize) -> Option<&T> {
        self.elements.get(index)
    }

    /// Returns a mutable reference to an element at `index`, if present.
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

    /// Returns the last element, if present.
    #[must_use]
    pub fn last(&self) -> Option<&T> {
        self.elements.last()
    }

    /// Returns the last element mutably, if present.
    pub fn last_mut(&mut self) -> Option<&mut T> {
        self.elements.last_mut()
    }

    /// Returns the backing elements as an immutable slice.
    #[must_use]
    pub fn as_slice(&self) -> &[T] {
        self.elements.as_slice()
    }

    /// Returns the backing elements as a mutable slice.
    #[must_use]
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        self.elements.as_mut_slice()
    }

    // -------------------------------------------------------------------------
    // Insertion
    // -------------------------------------------------------------------------

    /// Appends an element.
    ///
    /// This is the standard infallible convenience operation.
    /// Use `try_push` when allocation failure must be reported.
    pub fn push(&mut self, value: T) {
        self.elements.push(value);
    }

    /// Fallibly appends an element.
    ///
    /// If additional capacity is needed and cannot be reserved, the
    /// existing list contents remain unchanged.
    ///
    /// The supplied value is dropped if reservation fails.
    pub fn try_push(
        &mut self,
        value: T,
    ) -> Result<(), CollectionError> {
        self.elements
            .try_reserve(1)
            .map_err(|_| {
                CollectionError::allocation_failed(
                    CollectionKind::List,
                    1,
                    0,
                )
            })?;

        self.elements.push(value);
        Ok(())
    }

    /// Inserts an element at the specified position.
    ///
    /// Panics if `index > len()`, following `Vec::insert` semantics.
    pub fn insert(
        &mut self,
        index: usize,
        value: T,
    ) {
        self.elements.insert(index, value);
    }

    /// Fallibly inserts an element at the specified position.
    ///
    /// Returns an allocation error if reservation fails.
    /// Panics if `index > len()`, matching the ordinary insertion contract.
    pub fn try_insert(
        &mut self,
        index: usize,
        value: T,
    ) -> Result<(), CollectionError> {
        assert!(
            index <= self.elements.len(),
            "list insertion index exceeds list length"
        );

        self.elements
            .try_reserve(1)
            .map_err(|_| {
                CollectionError::allocation_failed(
                    CollectionKind::List,
                    1,
                    0,
                )
            })?;

        self.elements.insert(index, value);
        Ok(())
    }

    /// Appends all items from an iterator.
    ///
    /// This follows standard collection behavior and may allocate
    /// infallibly through the allocator.
    pub fn extend<I>(&mut self, iterator: I)
    where
        I: IntoIterator<Item = T>,
    {
        self.elements.extend(iterator);
    }

    /// Fallibly appends all items from an iterator.
    ///
    /// This operation is incremental. If allocation fails:
    ///
    /// - previously appended elements remain in the list;
    /// - the current and remaining iterator items are not appended;
    /// - the iterator is consumed up to the failure point;
    /// - the returned error reports successfully processed items.
    ///
    /// The iterator's upper size hint is never treated as a limit.
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
            self.elements
                .try_reserve(1)
                .map_err(|_| {
                    CollectionError::allocation_failed(
                        CollectionKind::List,
                        1,
                        processed,
                    )
                })?;

            self.elements.push(item);

            // The count cannot exceed the number of successfully appended
            // elements, which is itself representable by Vec's length.
            processed += 1;
        }

        Ok(())
    }

    // -------------------------------------------------------------------------
    // Removal
    // -------------------------------------------------------------------------

    /// Removes and returns the last element.
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

    /// Removes and returns the element at `index` without preserving order.
    ///
    /// The last element is moved into the removed position when necessary.
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
    // Traversal
    // -------------------------------------------------------------------------

    /// Returns an immutable iterator over the elements.
    pub fn iter(&self) -> core::slice::Iter<'_, T> {
        self.elements.iter()
    }

    /// Returns a mutable iterator over the elements.
    pub fn iter_mut(&mut self) -> core::slice::IterMut<'_, T> {
        self.elements.iter_mut()
    }

    /// Consumes the list and returns an owning iterator.
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

impl<T> Default for List<T> {
    fn default() -> Self {
        Self::new()
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

// =============================================================================
// Map
// =============================================================================

/// A dynamically sized hash-based key/value collection.
///
/// `Map<K, V>` uses Rust's `HashMap<K, V>`.
///
/// It provides average-case hash-based lookup and does not promise
/// deterministic iteration order.
///
/// Use `ClassicalMap<K, V>` when deterministic ordered-map behavior
/// is required by a classical computation.
///
/// Keys must implement `Eq + Hash`.
#[derive(Debug, Clone)]
pub struct Map<K, V> {
    entries: HashMap<K, V>,
}

impl<K, V> Map<K, V> {
    // -------------------------------------------------------------------------
    // Construction
    // -------------------------------------------------------------------------

    /// Creates an empty map.
    #[must_use]
    pub fn new() -> Self
    where
        K: Eq + Hash,
    {
        Self {
            entries: HashMap::new(),
        }
    }

    /// Creates an empty map and attempts to reserve capacity.
    ///
    /// Capacity is an allocation request, not a maximum entry count.
    pub fn try_with_capacity(
        capacity: usize,
    ) -> Result<Self, CollectionError>
    where
        K: Eq + Hash,
    {
        let mut entries = HashMap::new();

        entries
            .try_reserve(capacity)
            .map_err(|_| {
                CollectionError::allocation_failed(
                    CollectionKind::Map,
                    capacity,
                    0,
                )
            })?;

        Ok(Self { entries })
    }

    /// Creates a map from an iterator using fallible incremental insertion.
    ///
    /// If an allocation fails, the partially built map is dropped and
    /// the error is returned.
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

    /// Creates a map from an iterator using ordinary allocation behavior.
    pub fn from_iter<I>(iterator: I) -> Self
    where
        K: Eq + Hash,
        I: IntoIterator<Item = (K, V)>,
    {
        Self {
            entries: iterator.into_iter().collect(),
        }
    }

    // -------------------------------------------------------------------------
    // Length and capacity
    // -------------------------------------------------------------------------

    /// Returns the number of stored entries.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns true when the map contains no entries.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Returns the currently allocated entry capacity.
    ///
    /// This is not a semantic or language-level maximum.
    #[must_use]
    pub fn capacity(&self) -> usize {
        self.entries.capacity()
    }

    /// Removes all entries while retaining allocated capacity.
    pub fn clear(&mut self) {
        self.entries.clear();
    }

    // -------------------------------------------------------------------------
    // Allocation
    // -------------------------------------------------------------------------

    /// Fallibly reserves capacity for at least `additional` entries.
    pub fn try_reserve(
        &mut self,
        additional: usize,
    ) -> Result<(), CollectionError> {
        self.entries
            .try_reserve(additional)
            .map_err(|_| {
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
        self.entries
            .try_reserve_exact(additional)
            .map_err(|_| {
                CollectionError::allocation_failed(
                    CollectionKind::Map,
                    additional,
                    0,
                )
            })
    }

    // -------------------------------------------------------------------------
    // Lookup
    // -------------------------------------------------------------------------

    /// Returns the value associated with `key`, if present.
    ///
    /// Borrowed lookup is supported when the key type implements
    /// `Borrow<Q>` and the borrowed form has matching hash/equality behavior.
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

    /// Returns true if the map contains `key`.
    #[must_use]
    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        K: Eq + Hash + Borrow<Q>,
        Q: Eq + Hash + ?Sized,
    {
        self.entries.contains_key(key)
    }

    // -------------------------------------------------------------------------
    // Insertion
    // -------------------------------------------------------------------------

    /// Inserts or replaces an entry.
    ///
    /// If the key already exists, its previous value is returned.
    ///
    /// This is the ordinary infallible convenience operation.
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
    /// Replacing an existing entry does not require additional capacity.
    ///
    /// For a new key, the map reserves capacity before insertion.
    /// If reservation fails, existing map contents remain unchanged.
    ///
    /// The supplied key and value are dropped if reservation fails.
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

        self.entries
            .try_reserve(1)
            .map_err(|_| {
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
    /// Returns the map so that calls can be chained.
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

    /// Extends the map from an iterator.
    ///
    /// Duplicate keys replace earlier values.
    /// This uses ordinary allocator behavior.
    pub fn extend<I>(&mut self, iterator: I)
    where
        K: Eq + Hash,
        I: IntoIterator<Item = (K, V)>,
    {
        self.entries.extend(iterator);
    }

    /// Fallibly extends the map from an iterator.
    ///
    /// The operation is incremental. If allocation fails, earlier
    /// successful insertions and replacements remain in the map.
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
                });
            }

            processed += 1;
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
    /// Keys are immutable through the callback, preserving the hash-map
    /// key invariants.
    pub fn retain<F>(&mut self, keep: F)
    where
        K: Eq + Hash,
        F: FnMut(&K, &mut V) -> bool,
    {
        self.entries.retain(keep);
    }

    // -------------------------------------------------------------------------
    // Traversal
    // -------------------------------------------------------------------------

    /// Returns an iterator over key/value pairs.
    ///
    /// Iteration order is unspecified.
    pub fn iter(&self) -> hash_map::Iter<'_, K, V> {
        self.entries.iter()
    }

    /// Returns a mutable iterator over key/value pairs.
    ///
    /// Keys are immutable through this iterator.
    /// Iteration order is unspecified.
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

    /// Returns the map's entries as an immutable reference.
    ///
    /// This exposes the standard map API without allowing callers to
    /// replace the backing map.
    #[must_use]
    pub fn as_hash_map(&self) -> &HashMap<K, V> {
        &self.entries
    }

    /// Returns the map's entries as a mutable reference.
    ///
    /// The caller receives the standard `HashMap` API. Key invariants
    /// remain governed by Rust's `Eq` and `Hash` contracts.
    pub fn as_hash_map_mut(&mut self) -> &mut HashMap<K, V> {
        &mut self.entries
    }

    /// Consumes this wrapper and returns the underlying hash map.
    #[must_use]
    pub fn into_hash_map(self) -> HashMap<K, V> {
        self.entries
    }
}

impl<K: Eq + Hash, V> Default for Map<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: Eq + Hash, V> From<HashMap<K, V>> for Map<K, V> {
    fn from(entries: HashMap<K, V>) -> Self {
        Self { entries }
    }
}

impl<K: Eq + Hash, V> From<Map<K, V>> for HashMap<K, V> {
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
    fn list_preserves_insertion_order_and_duplicates() {
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
    fn list_can_be_consumed_into_a_vector() {
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

        let mut pairs: Vec<_> = map.iter()
            .map(|(key, value)| (*key, *value))
            .collect();

        pairs.sort_unstable();

        assert_eq!(
            pairs,
            vec![("a", 1), ("b", 2), ("c", 3)]
        );
    }

    // -------------------------------------------------------------------------
    // Shared error tests
    // -------------------------------------------------------------------------

    #[test]
    fn allocation_error_exposes_machine_readable_information() {
        let error = CollectionError::AllocationFailed {
            collection: CollectionKind::Map,
            requested_additional: 8,
            items_processed: 3,
        };

        assert_eq!(error.collection(), CollectionKind::Map);
        assert_eq!(error.requested_additional(), 8);
        assert_eq!(error.items_processed(), 3);

        assert!(error.to_string().contains("map"));
        assert!(error.to_string().contains("8"));
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
    fn hash_set_remains_available_from_foundation() {
        let mut values = HashSet::new();

        values.insert(10);
        values.insert(10);
        values.insert(20);

        assert_eq!(values.len(), 2);
    }
}
