//! Zenith Language Core Library
//!
//! This crate provides the Zenith compiler's core components.
//! Extended modules (stdlib, runtime, optimizer, etc.) are preserved as source
//! and will be progressively enabled as Zenith-specific syntax is implemented.

pub mod lexer;
pub mod source_map;
pub mod compiler_types;
pub mod error_reporting;

/// Initialises the Zenith runtime environment.
pub fn initialize_runtime() {
    println!("Zenith Universal Trinity Runtime initialised.");
}
