
//! Zenith Universal Meta-Compiler (UMC): Language Specification Modules
//!
//! This module aggregates and manages all conceptual language specification components
//! for Zenith. It defines new keywords, declarative syntax, and attributes that extend
//! Zenith's core language to directly support advanced AGI capabilities, multi-paradigm
//! constructs, and inherent security/ethics.
//!
//! Inspired by the UBUNTU grammar, this layer elevates many runtime and library features
//! to first-class language constructs, enabling more expressive and auditable AGI development.

pub mod ai_cognition; // For keywords like infer, learn, assert
pub mod concurrency_actors; // For actor keyword
pub mod explainability_transparency; // For explain, transparent keywords
pub mod declarative_system_directives; // For self_adjust, version blocks
// pub mod security_ethics_attributes; // For #[safety], #[ethics] attributes
// pub mod advanced_types_syntax; // For dependent, linear types syntax


/// Initializes all Zenith language specification modules.
pub fn init_language_spec() {
    println!("  - Initializing Zenith Language Specification Modules...");
    ai_cognition::init_ai_cognition_keywords();
    concurrency_actors::init_concurrency_actors_keywords();
    explainability_transparency::init_explainability_transparency_keywords();
    declarative_system_directives::init_declarative_system_directives_keywords(); // Initialize Declarative System Directives module
    println!("  - Zenith Language Specification Modules initialized.");
}

/// Shuts down all Zenith language specification modules.
pub fn shutdown_language_spec() {
    println!("  - Shutting down Zenith Language Specification Modules...");
    declarative_system_directives::shutdown_declarative_system_directives_keywords(); // Shutdown Declarative System Directives module
    explainability_transparency::shutdown_explainability_transparency_keywords();
    concurrency_actors::shutdown_concurrency_actors_keywords();
    ai_cognition::shutdown_ai_cognition_keywords();
    println!("  - Zenith Language Specification Modules shut down.");
}
