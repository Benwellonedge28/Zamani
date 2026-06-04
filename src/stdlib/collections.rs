//! Zenith Standard Library: Collections
//!
//! This module defines fundamental collection data structures that are
//! universally available in Zenith programs. These are paradigm-agnostic
//! and provide common ways to store and manage groups of elements.

use std::collections::HashMap; // For conceptual implementation

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

    // Conceptual: Iterator
    // pub fn iter(&self) -> ListIterator<T> { ListIterator { list: self, index: 0 } }
}

/// A map (dictionary/hash table) from keys to values.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Map<K, V> {
    entries: HashMap<K, V>,
}

impl<K, V> Map<K, V> {
    pub fn new() -> Self {
        Map {
            entries: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.entries.insert(key, value)
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
}

/// Initializes the collections standard library components.
pub fn init_collections_lib() {
    println!("  - Initializing StdLib Collections...");
}

/// Shuts down the collections standard library components.
pub fn shutdown_collections_lib() {
    println!("  - Shutting down StdLib Collections...");
}
