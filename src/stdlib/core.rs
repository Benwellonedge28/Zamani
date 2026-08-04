//! Zenith Standard Library: Core Utilities
//!
//! This module provides foundational and universally applicable utilities
//! for all Zenith programs, regardless of paradigm. It includes basic data
//! structures, common mathematical operations, string manipulation, and
//! essential input/output functions.

use std::collections::HashMap;
use std::fmt::{self, Display, Formatter}; // For Display trait implementation // For Map

// --- Basic Data Structures (Conceptual) ---

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
        match self.elements.pop() {
            Some(v) => Option::Some(v),
            None => Option::None,
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        match self.elements.get(index) {
            Some(v) => Option::Some(v),
            None => Option::None,
        }
    }

    pub fn len(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

/// A map (dictionary/hash table) from keys to values.
#[derive(Debug, Clone)]
pub struct Map<K, V> {
    entries: HashMap<K, V>,
}

impl<K: Eq + std::hash::Hash, V> Map<K, V> {
    pub fn new() -> Self {
        Map {
            entries: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        match self.entries.insert(key, value) {
            Some(v) => Option::Some(v),
            None => Option::None,
        }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        match self.entries.get(key) {
            Some(v) => Option::Some(v),
            None => Option::None,
        }
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        match self.entries.remove(key) {
            Some(v) => Option::Some(v),
            None => Option::None,
        }
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }
}

/// Represents an optional value: either a value of type `T` or nothing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Option<T> {
    Some(T),
    None,
}

/// Represents either a successful result `Ok(T)` or an error `Err(E)`.?
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Result<T, E> {
    Ok(T),
    Err(E),
}

// --- Basic I/O (Conceptual) ---

/// Prints a string to the standard output.
pub fn println<T: Display>(message: T) {
    // Conceptual: This would interact with the Nimbus OS console API
    // or a VM-specific I/O instruction.
    std::println!("{}", message); // Uses Rust's println for conceptual demonstration
}

/// Reads a line from standard input.
pub fn read_line() -> String {
    // Conceptual: This would interact with the Nimbus OS console API
    // or a VM-specific I/O instruction.
    let mut input = String::new();
    // std::io::stdin().read_line(&mut input).expect("Failed to read line"); // Requires real I/O
    println!("  [StdLib::core] Conceptual: Reading input. Returning dummy string.");
    "conceptual input".to_string()
}

// --- Mathematical Operations (Conceptual) ---

/// Returns the absolute value of a float.
pub fn abs_float(value: f64) -> f64 {
    value.abs()
}

/// Returns the maximum of two comparable values.
pub fn max<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

/// Returns a random float between 0.0 and 1.0 (exclusive).
pub fn rand() -> f64 {
    // Simple deterministic pseudo-random using system time
    // (rand crate not available; this is conceptual anyway)
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.subsec_nanos() as f64)
        .unwrap_or(0.0);
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as f64)
        .unwrap_or(1.0);
    let val = (nanos / 1_000_000_000.0) + secs.fract();
    val.fract()
}

// --- Type Conversion (Conceptual) ---

/// Converts an integer to a string.
pub fn to_string_int(value: i32) -> String {
    value.to_string()
}

/// Converts a float to a string.
pub fn to_string_float(value: f64) -> String {
    value.to_string()
}

// --- Initialization and Shutdown ---

/// Initializes the core standard library components.
pub fn init_core_lib() {
    println!("  - Initializing StdLib Core Utilities...");
    // No-op for now, as these are mostly conceptual interfaces.
}

/// Shuts down the core standard library components.
pub fn shutdown_core_lib() {
    println!("  - Shutting down StdLib Core Utilities...");
    // No-op for now.
}
