
//! Zenith Standard Library: Collections
//!
//! This module provides common data structures suchs as lists, maps, and sets,
//! designed for efficiency and ease of use within Zenith programs.

/// Initializes the collections standard library components.
pub fn init_collections_lib() {
    println!("  - Initializing StdLib Collections...");
}

/// Shuts down the collections standard library components.
pub fn shutdown_collections_lib() {
    println!("  - Shutting down StdLib Collections...");
}

/// A conceptual dynamically-sized list (vector).
#[derive(Debug, Clone)]
pub struct List<T> {
    elements: Vec<T>,
}

impl<T> List<T> {
    pub fn new() -> Self { List { elements: Vec::new() } }
    pub fn push(&mut self, item: T) { self.elements.push(item); }
    pub fn get(&self, index: usize) -> Option<&T> { self.elements.get(index) }
    pub fn len(&self) -> usize { self.elements.len() }
}

/// A conceptual hash map.
#[derive(Debug, Clone)]
pub struct Map<K, V> {
    entries: std::collections::HashMap<K, V>,
}

impl<K: std::cmp::Eq + std::hash::Hash, V> Map<K, V> {
    pub fn new() -> Self { Map { entries: std::collections::HashMap::new() } }
    pub fn insert(&mut self, key: K, value: V) -> Option<V> { self.entries.insert(key, value) }
    pub fn get(&self, key: &K) -> Option<&V> { self.entries.get(key) }
    pub fn len(&self) -> usize { self.entries.len() }
}
