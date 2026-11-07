#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani — Universal Computing Language
//!
//! Core library and public module composition root.
//!
//! Rust edition: 2021
//! Minimum supported Rust version: 1.97.1
//!
//! # Mission
//!
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.
//!
//! Zamani provides a unified programming language and computing
//! architecture for classical, quantum, hardware-description,
//! hybrid, distributed, AI, tensor, networking, cryptographic,
//! embedded and future computational domains.
//!
//! # Responsibilities
//!
//! This crate root owns:
//!
//! - Public crate module composition.
//! - Stable top-level module paths.
//! - Crate-wide safety requirements.
//! - Library version exposure.
//! - Runtime initialization delegation.
//! - Temporary legacy API compatibility.
//!
//! It does not own:
//!
//! - Source grammar.
//! - Lexical analysis.
//! - Parsing.
//! - AST construction.
//! - Semantic analysis.
//! - Intermediate representation definitions.
//! - Compiler pipeline implementation.
//! - Optimization algorithms.
//! - Hardware discovery.
//! - Physical resource allocation.
//! - Backend code generation.
//! - Runtime scheduling.
//! - Physical execution.
//!
//! Those responsibilities belong to their authoritative subsystems.
//!
//! # Architectural authorities
//!
//! Grammar:
//!     grammar/
//!
//! Frontend:
//!     src/frontend/
//!
//! Semantic analysis:
//!     src/frontend/semantic/
//!
//! Universal intermediate representation:
//!     src/zuir/
//!
//! Classical computation:
//!     src/classical/
//!
//! Quantum computation:
//!     src/quantum/
//!
//! Hardware description:
//!     src/hdl/
//!
//! Artificial intelligence:
//!     src/ai/
//!
//! Hybrid computation:
//!     src/hybrid/
//!
//! Distributed computation:
//!     src/distributed/
//!
//! Compiler:
//!     src/compiler/
//!
//! Backend implementations:
//!     src/backend/
//!
//! Runtime:
//!     src/runtime/
//!
//! The crate root exposes existing implementations.
//! It must not establish competing authorities.
//!
//! # POCO-REAF and scalability
//!
//! The library must not introduce arbitrary universal limits on:
//!
//! - Processors.
//! - CPU cores.
//! - Threads.
//! - GPUs.
//! - FPGAs.
//! - QPUs.
//! - Accelerators.
//! - Distributed participants.
//! - Memory.
//! - Storage.
//! - Qubits.
//! - Register widths.
//! - Tensor dimensions.
//! - Tensor rank.
//! - Modules.
//! - Functions.
//! - Compiler stages.
//! - Optimization passes.
//! - Computational domains.
//!
//! Actual execution is constrained by available resources,
//! implementation capabilities, operational budgets and
//! target-specific requirements.
//!
//! No finite implementation promises literal infinite computation.
//!
//! # Safety
//!
//! Unsafe Rust is forbidden throughout this crate.
//!
//! No unsafe blocks, unsafe functions or unsafe implementations
//! may be introduced.
//!
//! All external inputs and serialized representations must be
//! validated at their appropriate subsystem boundaries.

// ============================================================================
// CRATE IDENTITY
// ============================================================================

/// Installed Zamani package version.
///
/// This is the Cargo package version, not the language version,
/// grammar version, compiler API version or IR schema version.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Returns the installed Zamani package version.
#[must_use]
pub const fn version() -> &'static str {
    VERSION
}

// ============================================================================
// LANGUAGE AND FRONTEND INFRASTRUCTURE
// ============================================================================

/// Legacy AST compatibility module.
///
/// The canonical AST ownership must remain with the frontend
/// architecture during the migration.
pub mod ast;

/// Canonical lexical implementation.
pub mod lexer;

/// Legacy parser implementation.
///
/// New frontend integration must use the canonical frontend
/// parser once its public composition boundary is available.
pub mod parser;

/// Source-file identity and source-location infrastructure.
pub mod source_map;

/// Source-level error reporting compatibility.
pub mod error_reporting;

// ============================================================================
// COMPILER
// ============================================================================

/// Canonical Zamani compiler composition root.
///
/// All compiler infrastructure must be integrated through
/// this module rather than duplicated at crate level.
///
/// The compiler coordinates frontend, semantic, IR,
/// optimization, planning, lowering, backend and artifact
/// subsystems.
pub mod compiler;

// ============================================================================
// CANONICAL LEGACY AND COMPATIBILITY INFRASTRUCTURE
// ============================================================================

/// Legacy compiler type compatibility.
///
/// New compiler-owned types should live under
/// `crate::compiler` unless an established shared authority
/// requires a different location.
pub mod compiler_types;

/// Legacy intermediate representation generation.
///
/// This module remains available for existing callers while
/// migration to canonical IR construction proceeds.
pub mod ir_gen;

/// Intermediate representation verification compatibility.
pub mod ir_verify;

/// Legacy semantic analysis compatibility.
///
/// Canonical semantic authority belongs to the frontend.
pub mod semantic;

/// Existing optimization infrastructure.
pub mod optimizer;

/// Core language primitives.
pub mod core_lang_primitives;

// ============================================================================
// COMPUTATIONAL DOMAINS
// ============================================================================

/// Artificial intelligence and machine-learning computation.
pub mod ai;

/// Distributed computation infrastructure.
pub mod distributed;

/// Quantum computation infrastructure.
pub mod quantum;

/// Nano-scale computation primitives.
pub mod nano;

/// Nimbus computational infrastructure.
pub mod nimbus;

/// Nimbus operating-system integration.
pub mod nimbus_os;

// ============================================================================
// BACKEND AND RUNTIME
// ============================================================================

/// Canonical backend implementation and code-generation boundary.
pub mod backend;

/// Runtime infrastructure.
///
/// Physical resource acquisition, scheduling and execution
/// belong to the runtime rather than the compiler.
pub mod runtime;

// ============================================================================
// STANDARD LIBRARY AND TOOLCHAIN
// ============================================================================

/// Zamani standard-library integration.
pub mod stdlib;

/// Compiler and language tooling.
pub mod toolchain;

/// Zamani project configuration.
pub mod zamani_project_config;

// ============================================================================
// LEGACY RUNTIME INITIALIZATION COMPATIBILITY
// ============================================================================

/// Initializes the Zamani runtime.
///
/// This function preserves the original crate-level entry point
/// while delegating actual initialization to the runtime owner.
///
/// Runtime initialization errors and lifecycle management must
/// eventually be exposed through a structured runtime API.
pub fn initialize_runtime() {
    runtime::initialize_runtime();
}

// ============================================================================
// LEGACY COMPILATION COMPATIBILITY
// ============================================================================

/// Compiles source using the legacy frontend pipeline.
///
/// # Deprecation
///
/// This entry point is retained for source compatibility.
///
/// New production integrations must use the canonical compiler
/// service exposed through `crate::compiler::api`.
///
/// This function currently uses:
///
/// - `crate::lexer`
/// - `crate::parser`
/// - `crate::semantic`
/// - `crate::ir_gen`
///
/// It does not establish those legacy modules as the permanent
/// frontend or IR authorities.
///
/// # Arguments
///
/// * `source` - Zamani source text.
///
/// # Returns
///
/// The generated legacy IR module or a collection of
/// compilation diagnostics.
///
/// # Errors
///
/// Returns diagnostics when lexical, parsing or semantic
/// validation fails.
///
/// # Portability
///
/// This compatibility function does not guarantee that the
/// generated IR is portable, target-compatible or executable.
///
/// Use the canonical compiler service for production
/// compilation workflows.
#[deprecated(
    since = "0.2.0",
    note = "Use the canonical compiler service in crate::compiler::api"
)]
pub fn compile(
    source: &str,
) -> Result<ir_gen::IrModule, Vec<String>> {
    use std::sync::Arc;

    use source_map::{FileId, SourceFile};

    let file_id = FileId::new(1);

    let source_file = Arc::new(SourceFile::new(
        "<stdin>".to_owned(),
        source.to_owned(),
    ));

    let lexer = lexer::Lexer::new(file_id, source_file);

    let mut parser = parser::Parser::new(lexer);

    let program = parser.parse_program();

    let parse_errors = parser.get_errors().clone();

    if !parse_errors.is_empty() {
        return Err(
            parse_errors
                .iter()
                .map(|error| {
                    format!("ParseError: {}", error.message)
                })
                .collect(),
        );
    }

    let mut semantic_analyzer =
        semantic::SemanticAnalyzer::new();

    let semantic_errors =
        semantic_analyzer.analyze(&program);

    if !semantic_errors.is_empty() {
        return Err(
            semantic_errors
                .iter()
                .map(|error| {
                    format!("SemanticError: {}", error.message)
                })
                .collect(),
        );
    }

    let mut ir_generator =
        ir_gen::IrGenerator::new();

    let module = ir_generator.generate(&program);

    Ok(module)
}

// ============================================================================
// CRATE-ROOT CONTRACT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn package_version_is_available() {
        assert!(!VERSION.is_empty());
    }

    #[test]
    fn version_function_matches_package_version() {
        assert_eq!(version(), VERSION);
    }

    #[test]
    fn compiler_composition_is_available() {
        assert_ne!(
            compiler::api_version(),
            0
        );
    }
}