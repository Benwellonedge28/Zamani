
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani Universal Meta-Compiler
//!
//! Canonical path: src/compiler/mod.rs
//!
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! Primary objective:
//!
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.
//!
//! This module is the composition boundary for the Zamani compiler.
//!
//! It integrates existing compiler subsystems without creating
//! competing implementations of semantic analysis, IR, optimization,
//! hardware management, backend code generation, or runtime execution.
//!
//! # Architectural ownership
//!
//! This module owns:
//!
//! - Compiler subsystem composition.
//! - Public compiler module organization.
//! - Compiler-level integration boundaries.
//! - Compiler facade documentation.
//!
//! It does not own:
//!
//! - Language grammar.
//! - Lexical analysis.
//! - Parsing.
//! - AST construction.
//! - Semantic meaning.
//! - Type inference algorithms.
//! - Ownership semantics.
//! - Canonical IR definitions.
//! - Domain-specific IR definitions.
//! - Optimization algorithms.
//! - Physical hardware discovery.
//! - Device allocation.
//! - Runtime scheduling.
//! - Backend implementation.
//! - Physical execution.
//!
//! # Canonical authorities
//!
//! Source syntax:
//!     grammar/Zamani.g4
//!
//! Grammar architecture:
//!     grammar/DESIGN.md
//!
//! Semantic meaning:
//!     grammar/specification/semantic-model.md
//!
//! Semantic implementation:
//!     crate::frontend::semantic
//!
//! Canonical universal IR:
//!     crate::zuir
//!
//! Classical IR:
//!     crate::classical
//!
//! Quantum IR:
//!     crate::quantum::ir
//!
//! Hardware IR:
//!     crate::hdl::hdlir
//!
//! AI computation:
//!     crate::ai
//!
//! Hybrid computation:
//!     crate::hybrid
//!
//! Backend:
//!     crate::backend
//!
//! Runtime:
//!     crate::runtime
//!
//! # POCO-REAF
//!
//! The compiler must preserve program meaning independently of
//! the particular hardware used for realization.
//!
//! It must support extensible computing domains and target
//! implementations without requiring changes to this composition
//! root whenever a new provider or architecture is registered.
//!
//! No universal hardware-capacity limits are defined here.
//!
//! No arbitrary maximum is imposed on:
//!
//! - CPUs, cores, or workers;
//! - GPUs, FPGAs, ASICs, or QPUs;
//! - devices or distributed participants;
//! - memory or storage;
//! - qubits;
//! - tensor dimensions or rank;
//! - modules, functions, or operations;
//! - compiler stages;
//! - optimization passes;
//! - targets or backends.
//!
//! Operational limits belong to explicit budgets, available
//! resources, implementation constraints, and target capabilities.
//!
//! Literal infinite memory or processing capacity is not promised.
//!
//! Resource exhaustion must be reported explicitly.
//!
//! # Compilation architecture
//!
//! Source
//!   -> Frontend
//!   -> Semantic validation
//!   -> Canonical IR
//!   -> Analysis and verification
//!   -> Semantics-preserving transformations
//!   -> Optimization
//!   -> Capability and feasibility analysis
//!   -> Heterogeneous planning
//!   -> Domain-specific lowering
//!   -> Backend realization
//!   -> Artifact validation
//!   -> Linking and packaging
//!
//! Conditional stages must only execute when applicable.
//!
//! Required stages must never be silently skipped.
//!
//! An unsupported operation must not be silently discarded.
//!
//! A successful compilation result must represent an actual
//! successful compilation, not a fabricated artifact.
//!
//! # Compatibility
//!
//! The crate root already declares:
//!
//!     pub mod compiler;
//!
//! Consequently, no additional declaration in src/lib.rs is needed.


// ============================================================================
// COMPILER FOUNDATION
// ============================================================================

/// Compiler diagnostics and structured error reporting.
///
/// Owns diagnostic representation and reporting integration.
pub mod diagnostics;

/// Compiler configuration, budgets, profiles, policies and targets.
///
/// Uses the existing compiler configuration authority.
pub mod configuration;

/// Compilation contexts, lifecycle, dependencies and provenance.
///
/// Uses the existing context implementations.
pub mod context;


// ============================================================================
// COMPILER ANALYSIS
// ============================================================================

/// Compiler-facing program, dependency, effect, ownership and
/// capability analysis.
///
/// Does not replace frontend semantic analysis.
pub mod analysis;


// ============================================================================
// INTERMEDIATE REPRESENTATION INTEGRATION
// ============================================================================

/// Compiler-facing integration with canonical IR implementations.
///
/// Does not introduce another universal or domain IR.
pub mod ir;


// ============================================================================
// COMPILER PIPELINE
// ============================================================================

/// Compiler stage contracts, ordering, execution, failures,
/// recovery and orchestration.
pub mod pipeline;


// ============================================================================
// COMPILER PASS MANAGEMENT
// ============================================================================

/// Compiler-facing pass registration, ordering, management
/// and pipeline composition.
///
/// The canonical pass authority remains in ZUIR.
pub mod passes;


// ============================================================================
// EXISTING COMPILER IMPLEMENTATIONS
// ============================================================================

/// Generic specialization and monomorphization.
pub mod monomorphizer;

/// Compiler type-inference integration.
///
/// Semantic type authority remains with the frontend.
pub mod type_inference;

/// Existing ownership and borrowing integration.
pub mod borrow_checker;

/// Existing compilation-strategy selection.
pub mod compilation_techniques;

/// Existing optimization-strategy integration.
pub mod optimization_strategies;

/// Existing instruction-fusion implementation.
pub mod instruction_fusion;

/// Existing telemetry-informed optimization integration.
pub mod sro;

/// Existing compiler linking implementation.
pub mod linker;

/// Existing incremental compilation implementation.
pub mod incremental;

/// Existing parallel compilation implementation.
pub mod parallel_build;

/// Existing JIT and IR execution integration.
pub mod jit;

/// Existing hardware partitioning integration.
///
/// Physical device allocation remains outside this module.
pub mod hardware_partitioner;

/// Existing hybrid compilation integration.
pub mod hybrid_pipeline;

/// Existing hybrid compilation profiling.
pub mod hybrid_profiles;

/// Existing backend source synthesis.
pub mod ssbe;

/// Existing compilation safety inspection.
pub mod safety_guard;

/// Existing WebAssembly backend integration.
pub mod wasm_backend;

/// Existing WebAssembly control-flow preparation.
pub mod wasm_cfg;

/// Existing LLVM backend integration.
pub mod llvm_backend;

/// Existing intermediate-representation exporters.
pub mod ir_exporters;

/// Existing advanced object-oriented compilation integration.
pub mod oop_advanced;

/// Existing compiler test metadata.
pub mod test_metadata;

/// Existing compiler fuzzing infrastructure.
pub mod fuzzing_harness;

/// Existing specialized IR feature integration.
pub mod unique_ir_features;


// ============================================================================
// ARCHITECTURAL VERSION
// ============================================================================

/// Compiler composition API version.
///
/// This version is independent of:
///
/// - Zamani language version.
/// - Grammar version.
/// - Semantic-model version.
/// - ZUIR schema version.
/// - Domain IR versions.
/// - Backend ABI versions.
/// - Runtime versions.
///
/// Increment when the public compiler composition contract
/// changes incompatibly.
pub const COMPILER_API_VERSION: u16 = 1;

/// Returns the compiler composition API version.
#[must_use]
pub const fn api_version() -> u16 {
    COMPILER_API_VERSION
}


// ============================================================================
// ARCHITECTURAL INVARIANTS
// ============================================================================

/// The compiler composition root must preserve these invariants.
///
/// 1. Frontend owns source parsing and AST construction.
/// 2. Frontend semantic analysis owns source meaning.
/// 3. Canonical IR modules own their representations.
/// 4. The optimizer owns optimization algorithms.
/// 5. Compiler planning does not allocate physical devices.
/// 6. Backend implementations own target-specific emission.
/// 7. Runtime implementations own physical execution.
/// 8. Unsupported operations produce explicit diagnostics.
/// 9. Missing capability evidence does not establish support.
/// 10. Transformations preserve applicable semantic contracts.
/// 11. Resource exhaustion is never silently converted to success.
/// 12. No arbitrary universal hardware ceilings are introduced.
///
/// This function exposes the architecture contract without
/// performing compilation or modifying global state.
#[must_use]
pub const fn architecture_version() -> u16 {
    COMPILER_API_VERSION
}


// ============================================================================
// MODULE CONTRACT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compiler_api_version_is_stable_and_nonzero() {
        assert_eq!(api_version(), COMPILER_API_VERSION);
        assert_ne!(COMPILER_API_VERSION, 0);
    }

    #[test]
    fn architecture_version_matches_public_api() {
        assert_eq!(
            architecture_version(),
            COMPILER_API_VERSION
        );
    }

    #[test]
    fn existing_compiler_subsystems_are_exposed() {
        let _ = core::any::type_name::<
            configuration::CompilationBudget
        >();

        let _ = core::any::type_name::<
            pipeline::PipelineOrchestrator
        >();
    }

    #[test]
    fn compiler_ir_integration_is_exposed() {
        assert_eq!(
            ir::api_version(),
            ir::COMPILER_IR_API_VERSION
        );
    }
}
