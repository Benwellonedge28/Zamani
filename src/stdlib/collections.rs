//! Zamani Standard Library: Collections
//!
//! This module defines fundamental collection data structures that are
//! universally available in Zamani programs. These are paradigm-agnostic
//! and provide common ways to store and manage groups of elements.

use std::collections::HashMap; // For conceptual implementation

/// Re-export of `std::collections::HashSet` — used as-is across several
/// stdlib/toolchain modules that need real set semantics (e.g. Fact causal
/// parents, musical concept tags). No custom wrapper is warranted here since
/// none of those consumers need anything beyond the standard API.
pub use std::collections::HashSet;

// Re-export List and Map from core::
// pub use crate::stdlib::core::List;
// pub use crate::stdlib::core::Map;
// For this example, we define them directly here, assuming collections will live here.

/// A dynamically sized list (array) of elements.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct List<T> {
    elements: Vec<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            elements: Vec::new(),
        }
    }

    pub fn push(&mut self, item: T) {
        self.elements.push(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.elements.pop()
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.elements.get(index)
    }

    pub fn len(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.elements.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, T> {
        self.elements.iter_mut()
    }

    pub fn from_vec(elements: Vec<T>) -> Self {
        List { elements }
    }

    pub fn into_vec(self) -> Vec<T> {
        self.elements
    }
}

impl<T> Default for List<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> FromIterator<T> for List<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        List {
            elements: iter.into_iter().collect(),
        }
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
    type IntoIter = std::slice::Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.elements.iter()
    }
}

impl<T> From<Vec<T>> for List<T> {
    fn from(elements: Vec<T>) -> Self {
        List { elements }
    }
}

/// A map (dictionary/hash table) from keys to values.
#[derive(Debug, Clone)]
pub struct Map<K, V> {
    entries: HashMap<K, V>,
}

// Manual PartialEq/Eq (rather than #[derive]) because HashMap<K, V> itself
// only implements PartialEq/Eq when `K: Eq + Hash`, a bound the derive
// macro cannot express on our behalf.
impl<K: Eq + std::hash::Hash, V: PartialEq> PartialEq for Map<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.entries == other.entries
    }
}

impl<K: Eq + std::hash::Hash, V: Eq> Eq for Map<K, V> {}

impl<K: Eq + std::hash::Hash, V> Map<K, V> {
    pub fn new() -> Self {
        Map {
            entries: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.entries.insert(key, value)
    }

    /// Builder-style insert: inserts the entry and returns `self`, useful for
    /// chaining onto a freshly-cloned map (e.g. `map.clone().with(k, v)`)
    /// without needing a separate `let mut` binding.
    pub fn with(mut self, key: K, value: V) -> Self {
        self.insert(key, value);
        self
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.entries.get(key)
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.entries.remove(key)
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<'_, K, V> {
        self.entries.iter()
    }

    pub fn values(&self) -> std::collections::hash_map::Values<'_, K, V> {
        self.entries.values()
    }

    pub fn values_mut(&mut self) -> std::collections::hash_map::ValuesMut<'_, K, V> {
        self.entries.values_mut()
    }

    pub fn keys(&self) -> std::collections::hash_map::Keys<'_, K, V> {
        self.entries.keys()
    }
}

impl<K: Eq + std::hash::Hash, V> Default for Map<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

/// Initializes the collections standard library components.
pub fn init_collections_lib() {
    println!("  - Initializing StdLib Collections...");
}

/// Shuts down the collections standard library components.
pub fn shutdown_collections_lib() {
    println!("  - Shutting down StdLib Collections...");
}
