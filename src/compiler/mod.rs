//! Zenith Universal Meta-Compiler (UMC) Compiler Stages and Control
//!
//! This module orchestrates the various stages of the Zenith UMC, from
//! frontend parsing to backend code generation. It defines the overall
//! compiler pipeline and integrates advanced features like optimization,
//! formal verification, and multi-paradigm support.

#[cfg(feature = "full")]
pub mod compilation_techniques; // For Hybrid Compilation Strategies
pub mod language_spec; // Zenith Language Specification modules
pub mod oop_advanced; // Advanced OOP Features
pub mod optimization_strategies;
pub mod test_metadata; // Compiler test metadata helpers // For managing and applying diverse optimization passes

/// Re-exports the front-end (lexing/parsing/semantic-analysis) stage types
/// under a single conventional path for language-spec modules to depend on.
pub mod frontend {
    pub use crate::semantic::{SemanticAnalyzer, TypeChecker};
}

/// Initializes the entire Zenith UMC compiler pipeline.
pub fn initialize_compiler() {
    println!("Initializing Zenith UMC Compiler...");
    oop_advanced::init_oop_advanced();
    language_spec::init_language_spec();
    compilation_techniques::init_compilation_techniques();
    optimization_strategies::init_optimization_strategies(); // Initialize Optimization Strategies module
    test_metadata::init_test_metadata();
    println!("Zenith UMC Compiler initialized.");
}

/// Shuts down the entire Zenith UMC compiler pipeline.
pub fn shutdown_compiler() {
    println!("Shutting down Zenith UMC Compiler...");
    test_metadata::shutdown_test_metadata();
    optimization_strategies::shutdown_optimization_strategies(); // Shutdown Optimization Strategies module
    compilation_techniques::shutdown_compilation_techniques();
    language_spec::shutdown_language_spec();
    oop_advanced::shutdown_oop_advanced();
    println!("Zenith UMC Compiler shut down.");
}

/// Triggers a full compilation process for a given Zenith source file.
pub fn compile(source_file_path: &str) -> Result<Vec<u8>, String> {
    println!("Compiling '{}' using Zenith UMC.", source_file_path);
    // Conceptual full pipeline:
    // 1. Lexing & Parsing (frontend)
    // 2. Semantic Analysis (frontend)
    // 3. IR Generation (ir_gen)
    // 4. Optimization (optimizer)
    // 5. Backend Code Generation (backend)
    Ok(vec![0xDE, 0xAD, 0xBE, 0xEF]) // Dummy compiled output
}
