
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani Universal Compiler
//!
//! Path: `src/compiler/mod.rs`
//!
//! Language: Zamani
//! Edition: Rust 2021
//! Minimum supported Rust version: 1.97
//!
//! # Mission
//!
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.
//!
//! POCO-REAF.
//!
//! The Zamani compiler coordinates the compilation of validated
//! Zamani programs into verified, portable and target-compatible
//! computational artifacts.
//!
//! It provides the shared compilation infrastructure for the
//! existing classical, quantum, HDL, AI, tensor, hybrid,
//! distributed, networking, cryptographic and other computational
//! domains supported by the repository.
//!
//! # Architectural ownership
//!
//! This module owns:
//!
//! - Compiler subsystem composition.
//! - Compiler module visibility.
//! - Compiler architectural boundaries.
//! - Compiler composition API versioning.
//! - Compiler-wide safety requirements.
//!
//! It does not own:
//!
//! - Source grammar.
//! - Lexical analysis.
//! - Parsing.
//! - AST construction.
//! - Source-level semantic authority.
//! - Canonical intermediate representation definitions.
//! - Domain-specific intermediate representation definitions.
//! - Physical hardware discovery.
//! - Physical resource allocation.
//! - Runtime scheduling.
//! - Backend implementation.
//! - Physical program execution.
//!
//! Those responsibilities remain with their existing
//! authoritative repository subsystems.
//!
//! # Canonical architectural authorities
//!
//! Language grammar:
//!     grammar/Zamani.g4
//!
//! Language specifications:
//!     grammar/specification/
//!     grammar/spec/
//!
//! Grammar conformance:
//!     grammar/grammar.md
//!
//! Frontend:
//!     crate::frontend
//!
//! Semantic authority:
//!     crate::frontend::semantic
//!
//! Universal IR:
//!     crate::zuir
//!
//! Classical computation:
//!     crate::classical
//!
//! Quantum computation:
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
//! Distributed computation:
//!     crate::distributed
//!
//! Backend implementation:
//!     crate::backend
//!
//! Runtime:
//!     crate::runtime
//!
//! # Compilation architecture
//!
//! ```text
//! Zamani source
//!      |
//!      v
//! Canonical frontend
//!      |
//!      v
//! Semantic validation
//!      |
//!      v
//! Canonical semantic model
//!      |
//!      v
//! Canonical IR construction
//!      |
//!      v
//! IR verification
//!      |
//!      v
//! Compiler analysis
//!      |
//!      v
//! Dependency and effect analysis
//!      |
//!      v
//! Verified transformations
//!      |
//!      v
//! Optimization
//!      |
//!      v
//! Capability negotiation
//!      |
//!      v
//! Feasibility assessment
//!      |
//!      v
//! Resource and realization planning
//!      |
//!      v
//! Domain-specific lowering
//!      |
//!      v
//! Backend realization
//!      |
//!      v
//! Artifact validation
//!      |
//!      v
//! Linking and packaging
//!      |
//!      v
//! Runtime handoff
//!      |
//!      v
//! Actual execution
//! ```
//!
//! This describes the intended integration architecture.
//! Module registration alone does not establish that every
//! stage is fully implemented or production-ready.
//!
//! # POCO-REAF and scalability
//!
//! The compiler must not introduce arbitrary universal
//! computational or hardware ceilings.
//!
//! No universal maximum is established here for:
//!
//! - CPUs, cores, processors or threads;
//! - GPUs, FPGAs, ASICs or QPUs;
//! - accelerators or heterogeneous devices;
//! - distributed participants;
//! - memory or storage;
//! - physical or logical qubits;
//! - tensor dimensions or rank;
//! - register widths;
//! - modules, functions or operations;
//! - compilation stages;
//! - optimization passes;
//! - targets or backend providers.
//!
//! Operational budgets are permitted.
//!
//! Such budgets belong to the existing configuration,
//! policy and resource-management implementations.
//!
//! They must not become semantic limitations on valid
//! Zamani programs.
//!
//! Actual feasibility depends on:
//!
//! - Program requirements.
//! - Available capabilities.
//! - Explicit constraints.
//! - Developer preferences.
//! - Planning hints.
//! - Operational budgets.
//! - Actual resource availability.
//!
//! These concepts must remain distinct.
//!
//! Unknown capability must not mean unlimited capability.
//!
//! Infeasible computation must not be reported as feasible.
//!
//! Unsupported operations must not be silently discarded.
//!
//! Resource exhaustion must produce an explicit failure.
//!
//! Literal infinite physical resources are not promised.
//!
//! # Extensibility
//!
//! Existing registries and provider interfaces are the
//! extension mechanisms for compiler functionality.
//!
//! New providers and implementations must integrate through
//! their existing authoritative interfaces.
//!
//! This composition root must not become a closed catalogue
//! of hardware, vendors, computing domains or algorithms.
//!
//! # Domain-specific compilation
//!
//! Not every program requires every computing domain.
//!
//! Classical, quantum, HDL, AI, hybrid and distributed
//! compilation paths must be activated according to validated
//! program semantics and applicable requirements.
//!
//! Conditional stages may be inactive when they are genuinely
//! inapplicable.
//!
//! Required stages must never be silently skipped.
//!
//! # Determinism
//!
//! Compiler composition must not introduce nondeterminism.
//!
//! Deterministic behavior is coordinated through the existing:
//!
//! - Pipeline ordering.
//! - Dependency graph.
//! - Parallel compilation.
//! - Incremental compilation.
//! - Reproducibility.
//! - Artifact serialization.
//!
//! Actual deterministic-build guarantees must be established
//! by those implementations and their verification evidence.
//!
//! # Security
//!
//! Unsafe Rust is forbidden throughout this module hierarchy.
//!
//! Compiler inputs, serialized representations, providers,
//! extensions and generated artifacts must be validated
//! by their respective authoritative security boundaries.
//!
//! This composition root does not replace those mechanisms.
//!
//! # Integration rule
//!
//! Every declared module below corresponds to an existing
//! repository component.
//!
//! Do not declare speculative modules.
//!
//! Do not create duplicate subsystem authorities.
//!
//! Do not remove an existing module without first migrating
//! its public consumers and compatibility contracts.

// ============================================================================
// PUBLIC COMPILER API
// ============================================================================

/// Public compiler requests, responses, sessions and
/// compiler-service interfaces.
pub mod api;

// ============================================================================
// COMPILER FOUNDATION
// ============================================================================

/// Structured compiler diagnostics, aggregation,
/// rendering and reporting.
pub mod diagnostics;

/// Compiler configuration, operational budgets,
/// policies, profiles, targets and validation.
pub mod configuration;

/// Compilation contexts, lifecycle state,
/// dependency context and provenance context.
pub mod context;

/// Existing compiler-owned shared types.
pub mod compiler_types;

// ============================================================================
// COMPILER ANALYSIS
// ============================================================================

/// Program, dependency, effects, ownership,
/// requirements, capabilities, cost, provenance
/// and feasibility analysis.
pub mod analysis;

// ============================================================================
// INTERMEDIATE REPRESENTATION INTEGRATION
// ============================================================================

/// Compiler-facing integration with the existing
/// canonical universal and domain-specific IR systems.
///
/// This module does not define another IR.
pub mod ir;

// ============================================================================
// COMPILATION PIPELINE
// ============================================================================

/// Compiler stage contracts, stage definitions,
/// ordering, execution, orchestration, failures
/// and recovery.
pub mod pipeline;

// ============================================================================
// COMPILER PASS MANAGEMENT
// ============================================================================

/// Existing pass management, registration,
/// legality, ordering and preservation contracts.
pub mod passes;

// ============================================================================
// OPTIMIZATION
// ============================================================================

/// Compiler optimization planning, policies,
/// cost models, decisions and validation.
pub mod optimization;

/// Existing instruction-fusion implementation.
///
/// Fusion must preserve the applicable semantics
/// of the representation being transformed.
pub mod instruction_fusion;

// ============================================================================
// OWNERSHIP AND TRANSFORMATION
// ============================================================================

/// Existing compiler ownership and borrow-checking
/// integration.
pub mod borrow_checker;

/// Existing generic specialization and
/// monomorphization implementation.
pub mod monomorphizer;

/// Existing compiler safety inspection.
pub mod safety_guard;

/// Existing telemetry-informed optimization
/// implementation.
pub mod sro;

// ============================================================================
// BACKEND AND LOWERING INTEGRATION
// ============================================================================

/// Compiler-facing backend discovery, registration,
/// capability inspection, compatibility, selection
/// and invocation.
pub mod backends;

/// Existing domain and target-specific lowering
/// coordination.
pub mod lowering;

// ============================================================================
// TARGET AND RESOURCE PLANNING
// ============================================================================

/// Target discovery, capability matching,
/// feasibility, resource planning, partitioning,
/// routing and scheduling plans.
pub mod planning;

// ============================================================================
// COMPILATION EXECUTION INFRASTRUCTURE
// ============================================================================

/// Existing parallel compilation infrastructure.
pub mod parallel;

/// Existing incremental compilation,
/// caching, fingerprints and invalidation.
pub mod incremental;

// ============================================================================
// SPECIALIZED COMPILATION
// ============================================================================

/// Existing specialized static backend emission.
pub mod ssbe;

/// Existing compiler macro-processing infrastructure.
pub mod macro_engine;

// ============================================================================
// COMPILATION GRAPH
// ============================================================================

/// Existing compilation workflow graph,
/// dependencies, edges, snapshots and transformations.
///
/// This graph coordinates compiler work.
/// It does not replace ZUIR or another program IR.
pub mod graph;

// ============================================================================
// EXTENSION AND PROVIDER INTEGRATION
// ============================================================================

/// Existing compiler extension and provider
/// integration infrastructure.
///
/// Existing extension contracts govern discovery,
/// registration, compatibility, isolation and validation.
pub mod extensions;

// ============================================================================
// REPRODUCIBLE COMPILATION
// ============================================================================

/// Existing compiler reproducibility,
/// deterministic-build and provenance-fingerprinting
/// infrastructure.
pub mod reproducibility;

// ============================================================================
// LINKING AND ARTIFACTS
// ============================================================================

/// Existing symbol resolution, module composition,
/// relocation and linking validation.
pub mod linking;

/// Existing compilation artifacts, manifests,
/// metadata, portability, serialization
/// and validation.
pub mod artifacts;

// ============================================================================
// SECURITY AND ASSURANCE
// ============================================================================

/// Existing compiler security policies,
/// verification, auditing and supply-chain
/// assurance integration.
pub mod security;

/// Existing compiler auditing implementation.
pub mod audit_engine;

// ============================================================================
// PROFILING AND OBSERVABILITY
// ============================================================================

/// Existing compiler profiling events,
/// measurements and aggregation.
pub mod profiling;

// ============================================================================
// TESTING AND FUZZING
// ============================================================================

/// Existing compiler testing infrastructure.
pub mod testing;

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
/// - Domain-specific IR versions.
/// - Backend ABI versions.
/// - Artifact format versions.
/// - Runtime versions.
///
/// Increment this version when the public compiler
/// composition contract changes incompatibly.
pub const COMPILER_API_VERSION: u16 = 1;

/// Returns the compiler composition API version.
#[must_use]
pub const fn api_version() -> u16 {
    COMPILER_API_VERSION
}

/// Returns the compiler architectural contract version.
///
/// This currently shares the composition API version.
/// It is not a separate versioning authority.
#[must_use]
pub const fn architecture_version() -> u16 {
    COMPILER_API_VERSION
}

// ============================================================================
// ARCHITECTURAL INVARIANTS
// ============================================================================

/// Architectural invariants required of the Zamani compiler.
///
/// These are requirements for the participating implementations,
/// not claims that every requirement has already been verified.
///
/// 1. The canonical frontend owns parsing and AST construction.
/// 2. Frontend semantic analysis owns source-level meaning.
/// 3. Canonical IR modules own their representations.
/// 4. Compiler analysis does not replace semantic authority.
/// 5. Transformations preserve applicable program semantics.
/// 6. Capability evidence must be explicit.
/// 7. Unknown capabilities cannot establish feasibility.
/// 8. Requirements, capabilities, preferences, constraints
///    and hints remain distinct.
/// 9. Planning does not allocate physical devices.
/// 10. Backend implementations own target-specific emission.
/// 11. Runtime implementations own physical execution.
/// 12. Conditional stages execute only when applicable.
/// 13. Required stages cannot be silently skipped.
/// 14. Unsupported operations produce explicit failures.
/// 15. Failed verification cannot be reported as success.
/// 16. Resource exhaustion cannot be converted into success.
/// 17. No arbitrary universal hardware ceilings are introduced.
/// 18. Extension providers must follow their registered contracts.
/// 19. Compilation artifacts retain applicable provenance.
/// 20. Incremental compilation cannot bypass mandatory verification.
/// 21. Parallel compilation must preserve required determinism.
/// 22. Domain-specific IR semantics must remain authoritative.
/// 23. Unsafe Rust is prohibited.
// ============================================================================
// COMPOSITION TESTS
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
    fn existing_compiler_subsystems_are_exposed() {
        // These references ensure that the existing composition
        // paths remain available without creating replacement
        // subsystem implementations.

        fn accepts_api(_: &api::SourceInput) {}

        fn accepts_configuration(_: &configuration::CompilerConfig) {}

        fn accepts_analysis(_: &analysis::ProgramAnalysis) {}

        fn accepts_context(_: &context::CompilationContext) {}

        fn accepts_graph(_: &graph::CompilationGraph) {}

        fn accepts_provenance(
            _: &reproducibility::ProvenanceFingerprint,
        ) {}

        let _ = (
            accepts_api,
            accepts_configuration,
            accepts_analysis,
            accepts_context,
            accepts_graph,
            accepts_provenance,
        );
    }
}
