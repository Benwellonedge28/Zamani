
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani Universal Meta-Compiler
//!
//! Canonical path: `src/compiler/mod.rs`
//! Rust edition: 2021
//! Minimum supported Rust version: 1.97
//!
//! # Mission
//!
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.
//!
//! The Zamani compiler transforms validated Zamani programs into
//! portable, target-independent and target-specific compilation
//! artifacts while preserving program semantics.
//!
//! It provides the composition boundary for compiler infrastructure
//! across classical, quantum, hybrid, AI, tensor, HDL, neuromorphic,
//! distributed, networking, cryptographic and future computing domains.
//!
//! # Architectural responsibility
//!
//! This module owns:
//!
//! - Compiler subsystem composition.
//! - Public compiler module organization.
//! - Compiler infrastructure integration boundaries.
//! - Compiler composition API versioning.
//!
//! This module does not own:
//!
//! - Source grammar.
//! - Lexical analysis.
//! - Parsing.
//! - AST construction.
//! - Source-level semantic authority.
//! - Canonical IR definitions.
//! - Domain-specific IR definitions.
//! - Physical hardware discovery.
//! - Physical resource allocation.
//! - Runtime scheduling.
//! - Backend implementation.
//! - Physical program execution.
//!
//! # Canonical architectural authorities
//!
//! Source grammar:
//!     grammar/Zamani.g4
//!
//! Semantic specification:
//!     grammar/specification/semantic-model.md
//!
//! Semantic implementation:
//!     crate::frontend::semantic
//!
//! Universal intermediate representation:
//!     crate::zuir
//!
//! Classical computing:
//!     crate::classical
//!
//! Quantum computing:
//!     crate::quantum::ir
//!
//! Hardware description:
//!     crate::hdl::hdlir
//!
//! AI computation:
//!     crate::ai
//!
//! Hybrid computation:
//!     crate::hybrid
//!
//! Distributed computing:
//!     crate::distributed
//!
//! Backend implementation:
//!     crate::backend
//!
//! Runtime execution:
//!     crate::runtime
//!
//! Compiler modules integrate with these authorities.
//! They must not establish competing representations or semantics.
//!
//! # Compilation architecture
//!
//! Source
//!   -> Frontend
//!   -> Semantic validation
//!   -> Canonical IR construction
//!   -> IR verification
//!   -> Program and dependency analysis
//!   -> Ownership and effect analysis
//!   -> Semantics-preserving transformations
//!   -> Optimization
//!   -> Capability negotiation
//!   -> Feasibility analysis
//!   -> Heterogeneous resource planning
//!   -> Domain-specific lowering
//!   -> Backend selection
//!   -> Backend realization
//!   -> Artifact validation
//!   -> Linking and packaging
//!   -> Runtime handoff
//!
//! This is the intended architectural flow, not a claim that every
//! stage is already fully implemented or integrated.
//!
//! # POCO-REAF and scalability
//!
//! The compiler must not impose universal fixed hardware limits.
//!
//! No arbitrary maximum is established here for:
//!
//! - CPUs, cores, threads or workers;
//! - GPUs, FPGAs, ASICs or QPUs;
//! - accelerators or heterogeneous devices;
//! - distributed participants;
//! - memory or storage;
//! - qubits;
//! - tensor dimensions or rank;
//! - modules, functions or operations;
//! - compiler stages;
//! - optimization passes;
//! - targets or backend providers.
//!
//! Resource budgets are operational policies, not semantic limits.
//!
//! Target capabilities describe what a target can support.
//!
//! Physical resource availability determines actual feasibility.
//!
//! An unknown capability must not be interpreted as unlimited support.
//!
//! An unsupported operation must not be silently discarded.
//!
//! Resource exhaustion must produce an explicit failure.
//!
//! Literal infinite physical resources are not promised.
//!
//! # Extensibility
//!
//! New computing domains, optimization passes, targets, backends and
//! providers should integrate through their respective registries,
//! contracts and extension interfaces.
//!
//! Adding a provider must not require changing this composition root
//! when the existing extension contracts are sufficient.
//!
//! # Safety
//!
//! Unsafe Rust is forbidden throughout this module hierarchy.
//!
//! Public APIs must document their contracts, error behavior and
//! compatibility expectations.

// ============================================================================
// COMPILER API
// ============================================================================

/// Public compiler interfaces, requests, responses, sessions
/// and compatibility contracts.
pub mod api;

// ============================================================================
// COMPILER FOUNDATION
// ============================================================================

/// Compiler diagnostics, aggregation, rendering and reporting.
pub mod diagnostics;

/// Compiler configuration, operational budgets, policies,
/// profiles, target descriptors and configuration validation.
pub mod configuration;

/// Compilation contexts, lifecycle state, dependencies
/// and compilation provenance.
pub mod context;

/// Compiler-local type definitions and compatibility types.
pub mod compiler_types;

// ============================================================================
// COMPILER ANALYSIS
// ============================================================================

/// Compiler-facing program, dependency, effect, ownership,
/// capability, feasibility and requirement analysis.
///
/// Source-level semantic authority remains in the frontend.
pub mod analysis;

// ============================================================================
// INTERMEDIATE REPRESENTATION INTEGRATION
// ============================================================================

/// Adapters, compatibility, verification, translation
/// and lowering integration for canonical IR systems.
///
/// This module does not define a competing universal IR.
pub mod ir;

// ============================================================================
// COMPILATION PIPELINE
// ============================================================================

/// Compiler stage contracts, standard stages, dependency ordering,
/// execution, failure representation, recovery planning
/// and orchestration.
pub mod pipeline;

// ============================================================================
// COMPILER PASS MANAGEMENT
// ============================================================================

/// Compiler pass management, registration, ordering,
/// preservation and pipeline composition.
///
/// Canonical IR pass semantics remain with their authoritative
/// IR and optimization subsystems.
pub mod passes;

// ============================================================================
// OPTIMIZATION
// ============================================================================

/// Compiler optimization planning, policies, cost models,
/// profitability, decisions and validation.
pub mod optimization;

/// Existing instruction-fusion implementation.
pub mod instruction_fusion;

// ============================================================================
// COMPILER ANALYSIS AND TRANSFORMATION IMPLEMENTATIONS
// ============================================================================

/// Existing compiler ownership and borrowing integration.
pub mod borrow_checker;

/// Existing generic specialization and monomorphization.
pub mod monomorphizer;

/// Existing compiler safety inspection.
pub mod safety_guard;

/// Existing telemetry-informed optimization implementation.
pub mod sro;

// ============================================================================
// TARGETS, BACKENDS AND LOWERING
// ============================================================================

/// Backend discovery, registration, capability inspection,
/// compatibility, selection and invocation contracts.
pub mod backends;

/// Domain and target-specific lowering infrastructure.
pub mod lowering;

// ============================================================================
// HETEROGENEOUS RESOURCE AND EXECUTION PLANNING
// ============================================================================

/// Target discovery, capability matching, resource planning,
/// partitioning, scheduling and feasibility infrastructure.
pub mod planning;

// ============================================================================
// COMPILATION EXECUTION INFRASTRUCTURE
// ============================================================================

/// Parallel build execution, dependencies, determinism,
/// scheduling and worker management.
pub mod parallel;

/// Incremental compilation, fingerprints, caching,
/// invalidation and persistence.
pub mod incremental;

// ============================================================================
// HYBRID AND SPECIALIZED COMPILATION
// ============================================================================

/// Existing specialized static backend emission integration.
pub mod ssbe;

/// Existing compiler macro processing infrastructure.
pub mod macro_engine;

// ============================================================================
// LINKING AND ARTIFACTS
// ============================================================================

/// Symbol management, composition, relocation and
/// linking validation.
pub mod linking;

/// Compilation artifacts, manifests, metadata, portability,
/// serialization and artifact validation.
pub mod artifacts;

// ============================================================================
// SECURITY AND ASSURANCE
// ============================================================================

/// Compiler security policies, verification,
/// supply-chain assurance and auditing.
pub mod security;

/// Existing compiler audit infrastructure.
pub mod audit_engine;

// ============================================================================
// PROFILING AND OBSERVABILITY
// ============================================================================

/// Compiler profiling events, measurements and aggregation.
pub mod profiling;

// ============================================================================
// TESTING AND FUZZING INFRASTRUCTURE
// ============================================================================

/// Existing compiler fuzzing infrastructure.
pub mod fuzzing_harness;

// ============================================================================
// COMPILER COMPOSITION API VERSION
// ============================================================================

/// Public compiler composition API version.
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
/// Increment when the public composition contract changes
/// incompatibly.
pub const COMPILER_API_VERSION: u16 = 1;

/// Returns the compiler composition API version.
#[must_use]
pub const fn api_version() -> u16 {
    COMPILER_API_VERSION
}

// ============================================================================
// ARCHITECTURAL CONTRACT
// ============================================================================

/// Returns the version of the compiler architectural contract.
///
/// This currently shares the composition API version.
/// It is not a separate implementation or version authority.
#[must_use]
pub const fn architecture_version() -> u16 {
    COMPILER_API_VERSION
}

// ============================================================================
// ARCHITECTURAL INVARIANTS
// ============================================================================

/// Compiler composition must preserve the following invariants:
///
/// 1. The frontend owns source parsing and AST construction.
/// 2. Frontend semantic analysis owns source-level meaning.
/// 3. Canonical IR modules own their representations.
/// 4. Compiler analysis does not replace semantic authority.
/// 5. Optimization preserves applicable semantic contracts.
/// 6. Capability evidence must be explicit and verifiable.
/// 7. Unknown capabilities must not establish feasibility.
/// 8. Planning does not allocate physical devices.
/// 9. Backend implementations own target-specific emission.
/// 10. Runtime implementations own physical execution.
/// 11. Conditional compilation stages execute only when applicable.
/// 12. Required stages cannot be silently skipped.
/// 13. Unsupported operations produce explicit diagnostics.
/// 14. Failed verification cannot be reported as successful.
/// 15. Resource exhaustion cannot be converted into success.
/// 16. No arbitrary universal hardware ceilings are introduced.
/// 17. Extension providers must follow their registered contracts.
///
/// These invariants describe architectural requirements.
/// They do not assert that every requirement has already been
/// implemented throughout the repository.

// ============================================================================
// MODULE CONTRACT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compiler_api_version_is_nonzero() {
        assert_ne!(COMPILER_API_VERSION, 0);
    }

    #[test]
    fn public_api_version_matches_constant() {
        assert_eq!(api_version(), COMPILER_API_VERSION);
    }

    #[test]
    fn architecture_version_matches_composition_version() {
        assert_eq!(architecture_version(), COMPILER_API_VERSION);
    }

    #[test]
    fn compiler_ir_integration_is_exposed() {
        assert_ne!(ir::COMPILER_IR_API_VERSION, 0);
    }

    #[test]
    fn pipeline_api_is_exposed() {
        assert_ne!(pipeline::PIPELINE_API_VERSION, 0);
    }

    #[test]
    fn configuration_api_is_exposed() {
        assert_ne!(configuration::CONFIGURATION_MODULE_API_VERSION, 0);
    }

    #[test]
    fn backend_api_is_exposed() {
        assert_ne!(backends::COMPILER_BACKENDS_API_VERSION, 0);
    }
}
