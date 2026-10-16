
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani classical collections: deterministic ordered maps.
//!
//! Rust baseline: 1.97+
//! Edition: 2021
//! Safety: unsafe Rust is forbidden.
//!
//! # Purpose
//!
//! Provide a target-independent associative collection mapping unique ordered
//! keys to values. This implementation keeps entries sorted by key and
//! exposes deterministic iteration.
//!
//! # Ownership and architecture
//!
//! This module owns:
//! - `ClassicalMap<K, V>`, an ordered concrete map implementation;
//! - its key-equivalence and iteration-order contract;
//! - checked, fallible capacity growth and insertion;
//! - map-local allocation errors.
//!
//! It does not own:
//! - source syntax, parser rules, or frontend AST nodes;
//! - canonical semantic types or canonical classical values;
//! - the language-wide definition of `Map<K, V>`;
//! - ownership/borrowing analysis for Zamani programs;
//! - canonical IR, serialization, or hashing formats;
//! - concurrency, persistence, distributed placement, or hardware mapping;
//! - global resource policy or runtime execution.
//!
//! # Key semantics
//!
//! Keys are considered equivalent when `Ord::cmp` returns `Ordering::Equal`.
//! Key types must implement a lawful `Ord` consistent with `Eq`.
//!
//! This concrete implementation requires ordered keys. It does NOT mean that
//! the Zamani language's general `Map<K, V>` type must require `Ord`: semantic
//! analysis and lowering may select a different implementation when the
//! program's key semantics or target require one.
//!
//! # Determinism
//!
//! Iteration is always in ascending key order. No randomized hashing,
//! hash seed, or insertion-order dependence is exposed.
//!
//! # Allocation and scalability
//!
//! There is no universal maximum entry count, key size, or value size.
//! Storage grows dynamically. `try_reserve`, `try_with_capacity`, `try_insert`,
//! and `try_extend` expose recoverable capacity-reservation failures.
//!
//! A failed `try_insert` does not change the map. `try_extend` is incremental:
//! earlier successful entries remain if a later insertion fails, and the
//! error reports how many input entries were processed before failure.
//!
//! No collection can guarantee infinite storage. Concrete size is bounded by
//! host representation, allocator behavior, element layout, and available
//! resources. Higher-level policy must impose any application-specific
//! resource budgets.
//!
//! # Complexity
//!
//! For `n` entries:
//! - lookup and key search: O(log n);
//! - insertion of a new key: O(n), due to maintaining sorted contiguous storage;
//! - replacement of an existing key: O(log n);
//! - removal: O(n);
//! - iteration: O(n).
//!
//! This trade-off provides deterministic order and fallible contiguous
//! allocation. A future tree/hash/concurrent backend may implement the same
//! semantic interface when its complexity or target characteristics justify
//! it. Such a backend must preserve the selected map's declared semantics.
//!
//! # Integration contract
//!
//! - `collections/mod.rs` registers and exports this module.
//! - `grammar/expressions/maps.g4` owns map-literal syntax.
//! - `grammar/types/map.g4`, `grammar/types/map-types.g4`, and the canonical
//!   generic type grammar own map type syntax.
//! - The canonical frontend AST remains the owner of parsed expressions.
//! - Semantic analysis validates key/value types, equality requirements,
//!   mutability, effects, ownership, and source-level legality.
//! - `memory/` owns language-level ownership and borrowing semantics.
//! - Lowering adapts validated operations to the existing canonical IR.
//! - This module does not introduce a second AST, type system, or IR.
//! - Serialization and canonical hashing remain owned by shared infrastructure.
//!
//! # Freeze rule
//!
//! Changes to key-equivalence, ordering, allocation failure behavior, or
//! partial-update guarantees require compatibility review and regression
//! tests.

use core::cmp::Ordering;
use core::fmt;
use core::iter::FusedIterator;
use core::slice;
use std::collections::TryReserveError;

/// Errors returned by fallible ordered-map operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClassicalMapError {
    /// Additional backing storage could not be reserved.
    ///
    /// `additional` is the additional number of entries requested by the
    /// failed reservation. `entries_processed` is zero for a direct reserve
    /// or insertion failure. For `try_extend`, it counts input entries
    /// successfully processed before the failed entry.
    AllocationFailed {
        /// Additional entry capacity requested.
        additional: usize,
        /// Number of input entries successfully processed by the operation.
        entries_processed: usize,
    },
}

impl fmt::Display for ClassicalMapError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AllocationFailed {
                additional,
                entries_processed,
            } => write!(
                f,
                "map allocation failed while requesting {additional} \
                 additional entries; {entries_processed} input entries \
                 were processed before failure"
            ),
        }
    }
}

impl std::error::Error for ClassicalMapError {}

impl From<TryReserveError> for ClassicalMapError {
    fn from(_: TryReserveError) -> Self {
        Self::AllocationFailed {
            additional: 0,
            entries_processed: 0,
        }
    }
}

/// A deterministic ordered map from unique keys to values.
///
/// Entries are stored in ascending key order. Key equivalence is defined by
/// `K::cmp(key) == Ordering::Equal`; callers must supply an `Ord`
/// implementation consistent with the key type's equality semantics.
///
/// This container is generic over `V` and does not define or convert the
/// semantic type of its values.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ClassicalMap<K, V> {
    entries: Vec<(K, V)>,
}

impl<K, V> Default for ClassicalMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> ClassicalMap<K, V> {
    /// Creates an empty map without allocating entry storage.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    /// Creates an empty map and fallibly reserves capacity for `capacity`
    /// entries.
    ///
    /// The requested capacity is a reservation request, not a semantic
    /// maximum. The map can grow beyond it through fallible insertion.
    pub fn try_with_capacity(
        capacity: usize,
    ) -> Result<Self, ClassicalMapError> {
        let mut entries = Vec::new();

        entries
            .try_reserve_exact(capacity)
            .map_err(|_| ClassicalMapError::AllocationFailed {
                additional: capacity,
                entries_processed: 0,
            })?;

        Ok(Self { entries })
    }

    /// Builds a map from an iterator using fallible incremental insertion.
    ///
    /// If allocation fails, the partially built map is dropped and the error
    /// is returned. Duplicate keys replace earlier values, as with `insert`.
    pub fn try_from_iter<I>(iter: I) -> Result<Self, ClassicalMapError>
    where
        I: IntoIterator<Item = (K, V)>,
        K: Ord,
    {
        let mut map = Self::new();
        map.try_extend(iter)?;
        Ok(map)
    }

    /// Returns the number of key/value pairs.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns true when the map contains no entries.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Returns the current backing-vector capacity.
    ///
    /// This is an implementation detail useful for diagnostics and capacity
    /// planning, not a language-level map limit.
    #[must_use]
    pub fn capacity(&self) -> usize {
        self.entries.capacity()
    }

    /// Removes all entries while retaining allocated capacity.
    pub fn clear(&mut self) {
        self.entries.clear();
    }

    /// Fallibly reserves capacity for at least `additional` entries beyond
    /// the map's current length.
    ///
    /// On failure, the map's logical contents remain unchanged.
    pub fn try_reserve(
        &mut self,
        additional: usize,
    ) -> Result<(), ClassicalMapError> {
        self.entries
            .try_reserve(additional)
            .map_err(|_| ClassicalMapError::AllocationFailed {
                additional,
                entries_processed: 0,
            })
    }

    /// Fallibly reserves capacity for at least `additional` entries beyond
    /// the current length, requesting an exact reservation where supported.
    pub fn try_reserve_exact(
        &mut self,
        additional: usize,
    ) -> Result<(), ClassicalMapError> {
        self.entries
            .try_reserve_exact(additional)
            .map_err(|_| ClassicalMapError::AllocationFailed {
                additional,
                entries_processed: 0,
            })
    }

    /// Returns the value associated with `key`, if present.
    #[must_use]
    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Ord + core::borrow::Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.search(key)
            .ok()
            .map(|index| &self.entries[index].1)
    }

    /// Returns a mutable reference to the value associated with `key`.
    ///
    /// The key and map ordering remain unchanged.
    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
    where
        K: Ord + core::borrow::Borrow<Q>,
        Q: Ord + ?Sized,
    {
        let index = self.search(key).ok()?;
        Some(&mut self.entries[index].1)
    }

    /// Returns true if `key` is present.
    #[must_use]
    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        K: Ord + core::borrow::Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.search(key).is_ok()
    }

    /// Inserts or replaces a key/value pair.
    ///
    /// If an equivalent key already exists, its value is replaced and the
    /// previous value is returned. The existing stored key is retained.
    ///
    /// If the key is new, this infallible convenience method may invoke the
    /// standard allocator through `Vec::insert`. Prefer `try_insert` when
    /// allocation failure must be handled explicitly.
    pub fn insert(&mut self, key: K, value: V) -> Option<V>
    where
        K: Ord,
    {
        match self.search(&key) {
            Ok(index) => Some(core::mem::replace(
                &mut self.entries[index].1,
                value,
            )),
            Err(index) => {
                self.entries.insert(index, (key, value));
                None
            }
        }
    }

    /// Fallibly inserts or replaces a key/value pair.
    ///
    /// A replacement does not allocate. For a new key, capacity is reserved
    /// before the map is modified. If reservation fails, the map remains
    /// unchanged and ownership of `key` and `value` is dropped with the
    /// returned error path.
    pub fn try_insert(
        &mut self,
        key: K,
        value: V,
    ) -> Result<Option<V>, ClassicalMapError>
    where
        K: Ord,
    {
        match self.search(&key) {
            Ok(index) => Ok(Some(core::mem::replace(
                &mut self.entries[index].1,
                value,
            ))),
            Err(index) => {
                self.entries
                    .try_reserve(1)
                    .map_err(|_| ClassicalMapError::AllocationFailed {
                        additional: 1,
                        entries_processed: 0,
                    })?;

                self.entries.insert(index, (key, value));
                Ok(None)
            }
        }
    }

    /// Extends the map from an iterator using fallible insertion.
    ///
    /// Duplicate keys replace existing values. This operation is incremental:
    /// if an allocation fails, earlier successful updates are retained.
    /// `entries_processed` in the error counts successful input entries,
    /// including replacements.
    pub fn try_extend<I>(
        &mut self,
        iter: I,
    ) -> Result<(), ClassicalMapError>
    where
        I: IntoIterator<Item = (K, V)>,
        K: Ord,
    {
        let mut entries_processed = 0usize;

        for (key, value) in iter {
            if let Err(error) = self.try_insert(key, value) {
                return Err(match error {
                    ClassicalMapError::AllocationFailed { additional, .. } => {
                        ClassicalMapError::AllocationFailed {
                            additional,
                            entries_processed,
                        }
                    }
                });
            }

            entries_processed = entries_processed.saturating_add(1);
        }

        Ok(())
    }

    /// Removes a key and returns its value, if present.
    pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        K: Ord + core::borrow::Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.remove_entry(key).map(|(_, value)| value)
    }

    /// Removes a key and returns the stored key/value pair, if present.
    pub fn remove_entry<Q>(&mut self, key: &Q) -> Option<(K, V)>
    where
        K: Ord + core::borrow::Borrow<Q>,
        Q: Ord + ?Sized,
    {
        let index = self.search(key).ok()?;
        Some(self.entries.remove(index))
    }

    /// Returns the smallest key and its value, if present.
    #[must_use]
    pub fn first_key_value(&self) -> Option<(&K, &V)> {
        self.entries
            .first()
            .map(|(key, value)| (key, value))
    }

    /// Returns the largest key and its value, if present.
    #[must_use]
    pub fn last_key_value(&self) -> Option<(&K, &V)> {
        self.entries
            .last()
            .map(|(key, value)| (key, value))
    }

    /// Returns an iterator over key/value pairs in ascending key order.
    pub fn iter(
        &self,
    ) -> impl DoubleEndedIterator<Item = (&K, &V)>
           + ExactSizeIterator
           + FusedIterator
    {
        self.entries.iter().map(|(key, value)| (key, value))
    }

    /// Returns a mutable iterator over values in ascending key order.
    ///
    /// Keys are not mutable through this iterator, preserving map ordering.
    pub fn iter_mut(
        &mut self,
    ) -> impl DoubleEndedIterator<Item = (&K, &mut V)>
           + ExactSizeIterator
           + FusedIterator
    {
        self.entries
            .iter_mut()
            .map(|(key, value)| (&*key, value))
    }

    /// Returns an iterator over keys in ascending key order.
    pub fn keys(
        &self,
    ) -> impl DoubleEndedIterator<Item = &K>
           + ExactSizeIterator
           + FusedIterator
    {
        self.entries.iter().map(|(key, _)| key)
    }

    /// Returns an iterator over values in ascending key order.
    ///
    /// Value order therefore follows the corresponding key order.
    pub fn values(
        &self,
    ) -> impl DoubleEndedIterator<Item = &V>
           + ExactSizeIterator
           + FusedIterator
    {
        self.entries.iter().map(|(_, value)| value)
    }

    /// Returns a mutable iterator over values in ascending key order.
    pub fn values_mut(
        &mut self,
    ) -> impl DoubleEndedIterator<Item = &mut V>
           + ExactSizeIterator
           + FusedIterator
    {
        self.entries.iter_mut().map(|(_, value)| value)
    }

    /// Removes and returns the smallest key/value pair.
    pub fn pop_first(&mut self) -> Option<(K, V)> {
        if self.entries.is_empty() {
            None
        } else {
            Some(self.entries.remove(0))
        }
    }

    /// Removes and returns the largest key/value pair.
    pub fn pop_last(&mut self) -> Option<(K, V)> {
        self.entries.pop()
    }

    /// Retains only entries for which `keep` returns true.
    ///
    /// The callback receives an immutable key and mutable value. Keys cannot
    /// be changed, so the sorted-key invariant remains intact.
    pub fn retain<F>(&mut self, mut keep: F)
    where
        F: FnMut(&K, &mut V) -> bool,
    {
        self.entries
            .retain_mut(|(key, value)| keep(key, value));
    }

    /// Returns the underlying entries as a borrowed slice.
    ///
    /// Entries are exposed read-only to prevent callers from changing key
    /// order or creating duplicate equivalent keys.
    #[must_use]
    pub fn as_entries(&self) -> &[(K, V)] {
        &self.entries
    }

    fn search<Q>(&self, key: &Q) -> Result<usize, usize>
    where
        K: Ord + core::borrow::Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.entries
            .binary_search_by(|(stored_key, _)| {
                match stored_key.borrow().cmp(key) {
                    Ordering::Less => Ordering::Less,
                    Ordering::Equal => Ordering::Equal,
                    Ordering::Greater => Ordering::Greater,
                }
            })
    }
}

impl<K: Ord, V> IntoIterator for ClassicalMap<K, V> {
    type Item = (K, V);
    type IntoIter = std::vec::IntoIter<(K, V)>;

    fn into_iter(self) -> Self::IntoIter {
        self.entries.into_iter()
    }
}

impl<'a, K, V> IntoIterator for &'a ClassicalMap<K, V> {
    type Item = (&'a K, &'a V);
    type IntoIter =
        std::iter::Map<slice::Iter<'a, (K, V)>, fn(&(K, V)) -> (&K, &V)>;

    fn into_iter(self) -> Self::IntoIter {
        fn borrow_pair<K, V>((key, value): &(K, V)) -> (&K, &V) {
            (key, value)
        }

        self.entries.iter().map(borrow_pair::<K, V>)
    }
}

impl<'a, K, V> IntoIterator for &'a mut ClassicalMap<K, V> {
    type Item = (&'a K, &'a mut V);
    type IntoIter = std::iter::Map<
        slice::IterMut<'a, (K, V)>,
        fn(&'a mut (K, V)) -> (&'a K, &'a mut V),
    >;

    fn into_iter(self) -> Self::IntoIter {
        fn borrow_pair_mut<K, V>(
            (key, value): &mut (K, V),
        ) -> (&K, &mut V) {
            (&*key, value)
        }

        self.entries.iter_mut().map(borrow_pair_mut::<K, V>)
    }
}

#[cfg(test)]
mod tests {
    use super::{ClassicalMap, ClassicalMapError};

    #[test]
    fn empty_map_has_no_entries() {
        let map = ClassicalMap::<String, i32>::new();

        assert!(map.is_empty());
        assert_eq!(map.len(), 0);
        assert_eq!(map.get("missing"), None);
    }

    #[test]
    fn insertion_and_lookup_work() {
        let mut map = ClassicalMap::new();

        assert_eq!(map.insert("answer", 42), None);
        assert_eq!(map.get(&"answer"), Some(&42));
        assert!(map.contains_key(&"answer"));
        assert_eq!(map.len(), 1);
    }

    #[test]
    fn replacing_a_key_returns_the_previous_value() {
        let mut map = ClassicalMap::new();

        assert_eq!(map.insert(7, "old"), None);
        assert_eq!(map.insert(7, "new"), Some("old"));
        assert_eq!(map.get(&7), Some(&"new"));
        assert_eq!(map.len(), 1);
    }

    #[test]
    fn iteration_is_sorted_not_insertion_order() {
        let mut map = ClassicalMap::new();

        map.insert(30, "c");
        map.insert(10, "a");
        map.insert(20, "b");

        let pairs: Vec<_> = map.iter().map(|(k, v)| (*k, *v)).collect();

        assert_eq!(pairs, vec![(10, "a"), (20, "b"), (30, "c")]);
    }

    #[test]
    fn borrowed_string_lookup_works() {
        let mut map = ClassicalMap::new();

        map.insert(String::from("alpha"), 1);

        assert_eq!(map.get("alpha"), Some(&1));
        assert!(map.contains_key("alpha"));
        assert_eq!(map.get("missing"), None);
    }

    #[test]
    fn mutable_lookup_changes_only_the_value() {
        let mut map = ClassicalMap::new();

        map.insert(1, 10);
        *map.get_mut(&1).expect("key exists") = 11;

        assert_eq!(map.get(&1), Some(&11));
        assert_eq!(map.keys().copied().collect::<Vec<_>>(), vec![1]);
    }

    #[test]
    fn fallible_insert_and_reserve_work() {
        let mut map = ClassicalMap::try_with_capacity(2).unwrap();

        map.try_insert(1, "one").unwrap();
        map.try_insert(2, "two").unwrap();
        map.try_reserve(3).unwrap();

        assert_eq!(map.len(), 2);
        assert!(map.capacity() >= 2);
    }

    #[test]
    fn try_extend_processes_entries_and_replaces_duplicates() {
        let mut map = ClassicalMap::new();

        map.try_extend([(2, "old"), (1, "one"), (2, "new")])
            .unwrap();

        assert_eq!(map.len(), 2);
        assert_eq!(map.get(&2), Some(&"new"));
        assert_eq!(
            map.iter().map(|(k, _)| *k).collect::<Vec<_>>(),
            vec![1, 2]
        );
    }

    #[test]
    fn try_from_iter_builds_a_deterministic_map() {
        let map = ClassicalMap::try_from_iter([(3, "c"), (1, "a"), (2, "b")])
            .unwrap();

        assert_eq!(
            map.iter().map(|(key, _)| *key).collect::<Vec<_>>(),
            vec![1, 2, 3]
        );
    }

    #[test]
    fn remove_returns_the_value_and_preserves_order() {
        let mut map = ClassicalMap::new();

        map.insert(3, "c");
        map.insert(1, "a");
        map.insert(2, "b");

        assert_eq!(map.remove(&2), Some("b"));
        assert_eq!(map.remove(&99), None);
        assert_eq!(
            map.keys().copied().collect::<Vec<_>>(),
            vec![1, 3]
        );
    }

    #[test]
    fn remove_entry_returns_the_original_key_and_value() {
        let mut map = ClassicalMap::new();

        map.insert(String::from("key"), 9);

        assert_eq!(
            map.remove_entry("key"),
            Some((String::from("key"), 9))
        );
        assert!(map.is_empty());
    }

    #[test]
    fn first_and_last_key_value_are_ordered() {
        let mut map = ClassicalMap::new();

        map.insert(8, "last");
        map.insert(2, "first");
        map.insert(5, "middle");

        assert_eq!(map.first_key_value(), Some((&2, &"first")));
        assert_eq!(map.last_key_value(), Some((&8, &"last")));
        assert_eq!(map.pop_first(), Some((2, "first")));
        assert_eq!(map.pop_last(), Some((8, "last")));
        assert_eq!(map.iter().map(|(k, _)| *k).collect::<Vec<_>>(), vec![5]);
    }

    #[test]
    fn retain_can_update_values_without_changing_keys() {
        let mut map = ClassicalMap::new();

        map.insert(1, 10);
        map.insert(2, 20);
        map.insert(3, 30);

        map.retain(|key, value| {
            *value += 1;
            *key != 2
        });

        assert_eq!(
            map.iter().map(|(k, v)| (*k, *v)).collect::<Vec<_>>(),
            vec![(1, 11), (3, 31)]
        );
    }

    #[test]
    fn clear_keeps_the_map_reusable() {
        let mut map = ClassicalMap::new();

        map.insert(1, 1);
        map.clear();

        assert!(map.is_empty());

        map.insert(2, 2);
        assert_eq!(map.get(&2), Some(&2));
    }

    #[test]
    fn large_keys_do_not_imply_a_fixed_entry_limit() {
        let mut map = ClassicalMap::new();

        map.insert(usize::MAX, "largest representable key");
        map.insert(0, "zero");

        assert_eq!(map.get(&usize::MAX), Some(&"largest representable key"));
        assert_eq!(map.len(), 2);
    }

    #[test]
    fn mutable_iteration_preserves_key_order() {
        let mut map = ClassicalMap::new();

        map.insert(2, 20);
        map.insert(1, 10);

        for (_, value) in &mut map {
            *value += 1;
        }

        assert_eq!(
            map.iter().map(|(k, v)| (*k, *v)).collect::<Vec<_>>(),
            vec![(1, 11), (2, 21)]
        );
    }

    #[test]
    fn entries_are_exposed_read_only() {
        let mut map = ClassicalMap::new();

        map.insert(2, "b");
        map.insert(1, "a");

        assert_eq!(map.as_entries(), &[(1, "a"), (2, "b")]);
    }

    #[test]
    fn allocation_error_contract_is_explicit() {
        let error = ClassicalMapError::AllocationFailed {
            additional: 4,
            entries_processed: 3,
        };

        assert!(error.to_string().contains("4 additional entries"));
        assert!(error.to_string().contains("3 input entries"));
    }
}
