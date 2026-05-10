
//! Zenith Standard Library: Core Utilities
//!
//! This module provides fundamental types, basic operations, and essential utilities
//! that are core to the Zenith programming language and its runtime.

/// Initializes the core standard library components.
pub fn init_core_lib() {
    println!("  - Initializing StdLib Core Utilities...");
}

/// A basic numeric type, conceptually representing an integer.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Int(i64);

impl Int {
    pub fn new(val: i64) -> Self { Int(val) }
    pub fn add(&self, other: &Int) -> Int { Int(self.0 + other.0) }
    // ... other arithmetic operations
}

/// A basic boolean type.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Bool(bool);

impl Bool {
    pub fn new(val: bool) -> Self { Bool(val) }
    // ... logical operations
}

/// Prints a line to the standard output.
pub fn println(s: &str) {
    println!("[StdLib::core] {}", s);
}

/// Performs a mathematical square root operation.
pub fn sqrt(f: f64) -> f64 {
    f.sqrt()
}

/// Converts a value to its string representation.
pub fn to_string<T: std::fmt::Debug>(val: T) -> String {
    format!("{:?}", val)
}
