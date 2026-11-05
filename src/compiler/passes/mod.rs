
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Compiler Pass Subsystem
//!
//! Path: `src/compiler/passes/mod.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! This module is the public composition boundary for compiler-facing
//! pass infrastructure.
//!
//! It exposes the existing pass manager integration, pass registry
//! integration, dependency ordering, and ordered pipeline.
//!
//! It deliberately does not implement a second pass infrastructure.
//!
//! # Architectural authority
//!
//! The canonical pass subsystem is owned by:
//!
//! - `crate::zuir::pass::contracts`
//! - `crate::zuir::pass::registry`
//! - `crate::zuir::pass::manager`
//! - `crate::zuir::pass::determinism`
//! - `crate::zuir::pass::invalidation`
//!
//! Compiler-facing integration is provided by:
//!
//! - `manager`
//! - `registry`
//! - `ordering`
//! - `pipeline`
//!
//! # Ownership
//!
//! This module owns:
//!
//! - Composition of the compiler pass namespace.
//! - Public module visibility.
//! - Curated compiler-facing re-exports.
//! - Documentation of pass subsystem integration boundaries.
//!
//! This module does not own:
//!
//! - Pass identity or canonical pass descriptors.
//! - Pass versioning or pass error definitions.
//! - Pass implementation registration algorithms.
//! - Pass execution algorithms.
//! - Analysis computation or analysis-result storage.
//! - Analysis invalidation algorithms.
//! - IR definitions or IR verification algorithms.
//! - Semantic analysis or semantic-preservation proofs.
//! - Optimization algorithms.
//! - Hardware discovery or capability matching.
//! - Physical resource allocation.
//! - Physical routing or scheduling.
//! - Runtime execution.
//! - Backend code generation.
//!
//! # Module responsibilities
//!
//! `manager`
//!     Compiler-facing facade over the existing ZUIR operational manager.
//!
//! `registry`
//!     Compiler-facing facade over the existing ZUIR pass registry.
//!
//! `ordering`
//!     Explicit pass dependency declarations and deterministic topological
//!     ordering. It does not execute passes.
//!
//! `pipeline`
//!     Ordered pass composition, descriptor snapshots, structural
//!     validation, and execution delegation to the ZUIR manager.
//!
//! # Canonical contract migration
//!
//! The existing ZUIR operational manager and registry still expose legacy
//! operational pass contract types. The canonical contract definitions
//! separately reside in `crate::zuir::pass::contracts`.
//!
//! The compiler-facing `manager` module exposes these two APIs using
//! explicit `Operational*` names for the legacy operational types and
//! canonical names for the canonical contract types.
//!
//! This module preserves that distinction.
//!
//! It does not pretend that the migration has already been completed.
//!
//! The eventual migration must happen in `crate::zuir::pass::manager`
//! and `crate::zuir::pass::registry`, not by implementing duplicate
//! compiler-side contracts.
//!
//! # Dependency direction
//!
//! ```text
//!                  ZUIR pass contracts
//!                          |
//!             +------------+------------+
//!             |            |            |
//!             v            v            v
//!        determinism   invalidation   registry
//!             |                         |
//!             +------------+------------+
//!                          |
//!                          v
//!                    ZUIR manager
//!                          |
//!                          v
//!                 compiler/passes/manager
//!                          |
//!              +-----------+-----------+
//!              |           |           |
//!              v           v           v
//!           ordering    pipeline   compiler clients
//! ```
//!
//! The diagram describes ownership and integration. It does not imply
//! that every module directly depends on every other module.
//!
//! # Execution-order contract
//!
//! Pass execution order is explicit.
//!
//! - `ordering` calculates a dependency-respecting order.
//! - `pipeline` preserves registration order.
//! - The manager executes registered passes.
//!
//! Calculating a dependency order does not automatically reorder an
//! already constructed pipeline.
//!
//! A caller must deliberately use the ordering result when constructing
//! a pipeline.
//!
//! # Analysis contract
//!
//! Required analyses, invalidated analyses, and analysis preservation
//! declarations belong to the canonical pass contracts and manager.
//!
//! This module does not create another analysis manager.
//!
//! The existing pipeline accepts initial analysis identifiers as
//! caller-provided availability declarations. Such declarations do not
//! themselves compute, store, or prove the freshness of analysis results.
//!
//! Analysis providers and the authoritative manager remain responsible
//! for valid analysis results and invalidation.
//!
//! # Verification contract
//!
//! The compiler pass namespace does not implement an IR verifier.
//!
//! The existing manager accepts an IR verifier through its established
//! interface. The relevant IR owner remains authoritative for the
//! representation's validity rules.
//!
//! # Failure contract
//!
//! A pass may mutate an IR before reporting an error.
//!
//! The pipeline does not promise transactional rollback.
//!
//! Callers must treat IR as potentially modified after an unsuccessful
//! pipeline execution and must discard it or verify it before reuse.
//!
//! # Determinism
//!
//! Ordering and pipeline composition preserve explicit order.
//!
//! Determinism declarations are checked through the existing pass
//! infrastructure. A declaration is a contract, not a proof that
//! arbitrary pass implementation code is deterministic.
//!
//! # Scalability
//!
//! This module introduces no arbitrary limits on:
//!
//! - Number of passes.
//! - Number of analyses.
//! - Number of compiler stages.
//! - Number of computational domains.
//! - Number of target architectures.
//! - Number of hardware devices.
//! - IR size.
//! - Program size.
//!
//! Dynamic collections and the underlying implementations are constrained
//! by actual address-space, representation, allocation, and processing
//! resources.
//!
//! Configured execution budgets remain distinct from semantic limits.
//!
//! The absence of an arbitrary ceiling does not imply infinite physical
//! resources or guaranteed successful allocation.
//!
//! # POCO-REAF
//!
//! Passes may be registered for classical, quantum, hybrid, HDL, AI,
//! tensor, distributed, networking, cryptographic, accelerator, and
//! future computational domains.
//!
//! Adding a new domain must not require adding a closed domain variant
//! to this module.
//!
//! Domain-specific pass implementations and IR semantics remain owned
//! by their respective repository subsystems.
//!
//! # Grammar boundary
//!
//! This module consumes compiler pass contracts and IR representations.
//!
//! It does not define source syntax.
//!
//! `grammar/Zamani.g4` is the root grammar composition boundary.
//! The grammar and semantic specifications require source semantics to
//! remain independent of temporary hardware capacity and implementation
//! details.
//!
//! Consequently, pass registration, dependency ordering, pass execution,
//! and optimization algorithms do not belong in grammar files.
//!
//! # Safety
//!
//! Unsafe Rust is forbidden.
//!
//! This module and its compiler-facing pass subsystem enforce that rule.
//!
//! # Compatibility
//!
//! Existing public APIs in the individual pass modules remain available
//! through their module paths.
//!
//! This module adds a stable composition boundary without renaming or
//! replacing the existing manager, registry, ordering, or pipeline APIs.
//!
//! # Completion criteria
//!
//! This module is complete when:
//!
//! - Every declared child module exists.
//! - Every child module compiles.
//! - Canonical pass contracts retain one authoritative definition.
//! - Pass registration retains one authoritative implementation.
//! - Pass execution retains one authoritative implementation.
//! - Analysis invalidation retains one authoritative implementation.
//! - Public re-exports do not create ambiguous type identities.
//! - Compiler root exports this module.
//! - Unit, integration, and documentation tests pass.
//! - Formatting and Clippy checks pass.

// -----------------------------------------------------------------------------
// Compiler-facing modules
// -----------------------------------------------------------------------------

/// Compiler-facing integration with the canonical ZUIR pass manager.
///
/// This module provides operational pass aliases, canonical contract
/// re-exports, verification interfaces, cancellation, execution budgets,
/// and manager configuration.
pub mod manager;

/// Explicit pass dependencies and deterministic topological ordering.
///
/// This module does not register or execute pass implementations.
pub mod ordering;

/// Ordered compiler pass composition and execution.
///
/// This module delegates execution to the existing ZUIR pass manager.
pub mod pipeline;

/// Compiler-facing access to the canonical ZUIR pass registry.
///
/// This module does not implement a second registry.
pub mod registry;

// -----------------------------------------------------------------------------
// Canonical pass contracts
// -----------------------------------------------------------------------------

/// Canonical pass identity.
pub use manager::PassId;

/// Canonical pass descriptor.
pub use manager::PassDescriptor;

/// Canonical pass version.
pub use manager::PassVersion;

/// Canonical pass error.
pub use manager::PassError;

/// Canonical pass error category.
pub use manager::PassErrorKind;

/// Canonical completed-pass outcome.
pub use manager::PassOutcome;

// -----------------------------------------------------------------------------
// Compiler-facing manager API
// -----------------------------------------------------------------------------

/// Compiler-facing alias for the existing ZUIR pass manager.
///
/// No second pass manager is created.
pub use manager::CompilerPassManager;

/// Compiler-facing operational pass trait.
pub use manager::CompilerOperationalPass;

/// Compiler-facing pass execution context.
pub use manager::CompilerPassContext;

/// Compiler-facing IR verifier interface.
pub use manager::CompilerIrVerifier;

/// Compiler-facing manager configuration.
pub use manager::CompilerPassManagerConfig;

/// Compiler-facing pipeline execution report.
pub use manager::CompilerPipelineReport;

/// Cooperative cancellation token.
pub use manager::CancellationToken;

/// Execution budget.
pub use manager::ExecutionBudget;

/// Verification policy.
pub use manager::VerificationPolicy;

/// Analysis availability state.
pub use manager::AnalysisState;

/// Pass execution record.
pub use manager::PassExecutionRecord;

// -----------------------------------------------------------------------------
// Explicit legacy operational API
// -----------------------------------------------------------------------------

// These are the operational types currently used by the ZUIR manager.
// They are not aliases for the canonical contract definitions.

/// Legacy operational pass identity.
pub use manager::OperationalPassId;

/// Legacy operational pass descriptor.
pub use manager::OperationalPassDescriptor;

/// Legacy operational pass version.
pub use manager::OperationalPassVersion;

/// Legacy operational pass error.
pub use manager::OperationalPassError;

/// Legacy operational pass error category.
pub use manager::OperationalPassErrorKind;

/// Legacy operational pass outcome.
pub use manager::OperationalPassOutcome;

/// Legacy operational pass trait.
pub use manager::OperationalPass;

// -----------------------------------------------------------------------------
// Registry API
// -----------------------------------------------------------------------------

/// Compiler-facing alias for the existing ZUIR pass registry.
///
/// Registration and duplicate detection remain owned by ZUIR.
pub use registry::CompilerPassRegistry;

/// Existing canonical registry implementation.
///
/// This is re-exported by the manager integration module from ZUIR.
pub use manager::PassRegistry;

// -----------------------------------------------------------------------------
// Ordering API
// -----------------------------------------------------------------------------

/// Explicit dependency between two passes.
pub use ordering::PassDependency;

/// Pass dependency graph.
pub use ordering::PassOrdering;

/// Pass descriptor abstraction accepted by the ordering algorithm.
pub use ordering::PassOrderingDescriptor;

/// Structured pass-ordering error.
pub use ordering::PassOrderingError;

/// Pass-ordering error category.
pub use ordering::PassOrderingErrorKind;

/// Validated deterministic pass-ordering result.
pub use ordering::PassOrderingResult;

// -----------------------------------------------------------------------------
// Pipeline API
// -----------------------------------------------------------------------------

/// Ordered compiler pass pipeline.
pub use pipeline::PassPipeline;

/// Immutable descriptor snapshot for a registered pipeline step.
pub use pipeline::PipelineStep;

/// Structural and determinism validation report for a pipeline.
pub use pipeline::PipelineValidationReport;

// -----------------------------------------------------------------------------
// Canonical determinism API
// -----------------------------------------------------------------------------

/// Pipeline determinism assessment.
pub use manager::DeterminismAssessment;

/// Determinism policy.
pub use manager::DeterminismPolicy;

/// Validate a pass determinism declaration.
pub use manager::validate_pass_determinism;

/// Assess the determinism declarations of an ordered pipeline.
pub use manager::assess_pipeline_determinism;

// -----------------------------------------------------------------------------
// Canonical analysis invalidation API
// -----------------------------------------------------------------------------

/// Calculate analysis invalidations from pass declarations.
///
/// The implementation remains owned by the ZUIR pass subsystem.
pub use manager::compute_invalidated_analyses;

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_pass_contracts_are_available_at_compiler_boundary() {
        let id = PassId::new("compiler.test")
            .expect("qualified pass identity should be accepted");

        let descriptor = PassDescriptor::new(
            id,
            PassVersion::new(1, 0, 0),
            "Compiler pass namespace integration test",
        );

        assert!(descriptor.validate().is_ok());
    }

    #[test]
    fn compiler_manager_is_the_existing_zuir_manager() {
        let manager =
            CompilerPassManager::<u64>::new(CompilerPassManagerConfig::default());

        assert!(manager.is_empty());
        assert_eq!(manager.pass_count(), 0);
    }

    #[test]
    fn compiler_registry_is_available_without_a_second_registry() {
        let registry = CompilerPassRegistry::<u64>::new();

        assert!(registry.is_empty());
        assert_eq!(registry.len(), 0);
    }

    #[test]
    fn pass_ordering_is_available_at_compiler_boundary() {
        let ordering = PassOrdering::<PassId>::new();

        assert!(ordering.is_empty());
        assert_eq!(ordering.dependency_count(), 0);
    }

    #[test]
    fn pipeline_is_available_at_compiler_boundary() {
        let pipeline = PassPipeline::<u64>::with_defaults();

        assert!(pipeline.is_empty());
        assert_eq!(pipeline.len(), 0);
    }

    #[test]
    fn cancellation_is_available_at_compiler_boundary() {
        let cancellation = CancellationToken::new();

        assert!(!cancellation.is_cancelled());

        cancellation.cancel();

        assert!(cancellation.is_cancelled());
    }

    #[test]
    fn determinism_policy_is_available_at_compiler_boundary() {
        let _policy = DeterminismPolicy::default();
    }
}
