
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Universal Meta-Compiler — Parallel Compilation.
//!
//! File:
//!     src/compiler/parallel/mod.rs
//!
//! Rust:
//!     Edition 2021
//!     Minimum supported version: Rust 1.97
//!
//! Safety:
//!     Safe Rust only.
//!     Unsafe Rust is forbidden.
//!
//! # Purpose
//!
//! This module is the public composition boundary for compiler-side
//! parallel compilation infrastructure.
//!
//! It connects four independently implemented components:
//!
//!     dependencies.rs
//!         Adapts the canonical compilation dependency graph into
//!         deterministic dependency-ready waves.
//!
//!     workers.rs
//!         Executes independent jobs using configurable worker resources.
//!
//!     scheduler.rs
//!         Coordinates dependency-aware execution and prerequisite
//!         failure propagation.
//!
//!     determinism.rs
//!         Integrates canonical determinism policy and validates
//!         deterministic result collection.
//!
//! This file exposes their APIs through one stable module namespace.
//!
//! It does not implement another worker pool, dependency graph,
//! scheduler, determinism policy, or compilation engine.
//!
//! # Ownership
//!
//! The parallel subsystem owns:
//!
//! - Compiler-side parallel job execution.
//! - Dependency-ready compilation scheduling.
//! - Worker execution configuration.
//! - Scheduler execution reports.
//! - Parallel compilation result collection.
//! - Compiler-facing determinism integration.
//! - Cooperative cancellation integration.
//!
//! It does not own:
//!
//! - Source parsing or AST construction.
//! - Semantic analysis or ownership checking.
//! - Type inference.
//! - Incremental cache invalidation.
//! - Canonical compilation-unit identity.
//! - Canonical dependency graph construction.
//! - Canonical language-level determinism semantics.
//! - Compiler IR definitions.
//! - Optimization algorithms.
//! - Hardware discovery.
//! - Physical resource allocation.
//! - Backend selection.
//! - Backend code generation.
//! - Runtime execution.
//! - Distributed execution.
//!
//! # Canonical authorities
//!
//! Compilation-unit identity and dependency relationships:
//!
//!     crate::compiler::context::dependency_context
//!
//! Canonical cooperative cancellation:
//!
//!     crate::classical::concurrency
//!
//! Canonical determinism policy:
//!
//!     crate::classical::concurrency::determinism
//!
//! Compiler dependency planning:
//!
//!     crate::compiler::parallel::dependencies
//!
//! Worker execution:
//!
//!     crate::compiler::parallel::workers
//!
//! Dependency-aware execution:
//!
//!     crate::compiler::parallel::scheduler
//!
//! Deterministic result collection:
//!
//!     crate::compiler::parallel::determinism
//!
//! # Dependency direction
//!
//!     Frontend / semantic dependency discovery
//!                     |
//!                     v
//!             DependencyContext
//!                     |
//!                     v
//!               DependencyPlan
//!                     |
//!                     v
//!             ParallelScheduler
//!                     |
//!                     v
//!                 WorkerPool
//!                     |
//!                     v
//!          Deterministic result collection
//!                     |
//!                     v
//!             Compiler pipeline
//!
//! Incremental compilation may determine which units require rebuilding.
//! It remains the authority for fingerprints, cache validation and
//! invalidation.
//!
//! The parallel subsystem executes the work it is given.
//!
//! # Dependency semantics
//!
//! The canonical dependency graph expresses:
//!
//!     dependent -> dependency
//!
//! A dependency must complete successfully before its dependent can run
//! when the relationship requires compilation ordering.
//!
//! Non-ordering relationships must not automatically become compilation
//! prerequisites.
//!
//! # Failure semantics
//!
//! A failed or panicked prerequisite blocks its dependent units.
//!
//! Independent branches may continue executing.
//!
//! A blocked unit is not silently reported as successfully compiled.
//!
//! Cancellation is cooperative. Already-running arbitrary Rust closures
//! cannot be forcibly interrupted.
//!
//! # Determinism
//!
//! Worker completion order must not determine the order in which
//! compilation results are exposed.
//!
//! The canonical dependency plan establishes compilation ordering.
//!
//! The canonical determinism adapter validates result identity,
//! uniqueness and completeness.
//!
//! Ordered collection does not, by itself, guarantee deterministic
//! callback side effects or numerical reproducibility.
//!
//! # POCO-REAF and scalability
//!
//! This module imposes no universal maximum on:
//!
//! - Compilation units.
//! - Dependency edges.
//! - Dependency waves.
//! - Worker threads.
//! - Parallel jobs.
//! - Computing domains.
//! - CPUs.
//! - GPUs.
//! - FPGAs.
//! - QPUs.
//! - Accelerators.
//! - Distributed nodes.
//!
//! Worker concurrency is determined by the existing worker configuration,
//! submitted work, available host resources and operating-system limits.
//!
//! A configured worker budget is an operational resource policy,
//! not a language-level hardware limitation.
//!
//! No finite implementation promises infinite physical memory,
//! operating-system threads or execution capacity.
//!
//! # Grammar boundary
//!
//! Source-level concurrency intent is specified under:
//!
//!     grammar/concurrency/
//!
//! Source-level compilation intent is specified under:
//!
//!     grammar/compile/
//!
//! These grammars do not implement this module.
//!
//! No new source grammar rule is required to expose compiler-side
//! parallel execution.
//!
//! # Compatibility
//!
//! The existing:
//!
//!     crate::compiler::parallel_build
//!
//! is a legacy build-engine path.
//!
//! It must eventually delegate to this subsystem through a compatibility
//! adapter. It must not remain a second authoritative parallel scheduler.
//!
//! This module deliberately does not import or depend on that legacy
//! implementation.
//!
//! # Freeze criteria
//!
//! This module is ready to freeze when:
//!
//! - Every declared child module exists.
//! - Every public re-export resolves.
//! - The compiler root registers this module.
//! - The crate root exposes the required canonical dependencies.
//! - Module-boundary tests pass.
//! - Existing callers compile against the public API.
//! - The legacy parallel build engine no longer provides a competing
//!   authoritative scheduling implementation.
//! - Formatting, compilation, tests and lint checks pass.

// ============================================================================
// COMPONENT MODULES
// ============================================================================

/// Dependency planning over the canonical compiler dependency graph.
///
/// This module does not create a second dependency graph.
pub mod dependencies;

/// Compiler-facing integration with the canonical determinism policy.
///
/// This module does not define a competing language-level determinism policy.
pub mod determinism;

/// Dependency-aware parallel compilation scheduling.
///
/// This module delegates job execution to the canonical worker pool.
pub mod scheduler;

/// Configurable execution of independent compiler jobs.
///
/// This module does not determine compilation dependency ordering.
pub mod workers;

// ============================================================================
// CANONICAL CANCELLATION CONTRACT
// ============================================================================

/// Cancellation token used by the parallel worker and scheduler APIs.
///
/// This is an alias of the existing classical concurrency cancellation
/// implementation, not a new parallel-specific cancellation mechanism.
///
/// Note that `compiler::context` currently also exposes a separately
/// implemented cancellation type. Those two implementations must be
/// unified at their owning context boundary before the wider compiler
/// API is considered fully integrated.
pub use crate::classical::concurrency::CancellationToken
    as ParallelCancellationToken;

// ============================================================================
// DEPENDENCY PLANNING API
// ============================================================================

/// Dependency-plan types and errors.
pub use self::dependencies::{
    DependencyPlan,
    DependencyPlanError,
    DependencyWave,
    PARALLEL_DEPENDENCY_API_VERSION,
};

// ============================================================================
// DETERMINISM API
// ============================================================================

/// Deterministic result collection and compiler-facing policy integration.
pub use self::determinism::{
    OrderedCompilationResult,
    ParallelDeterminism,
    ParallelDeterminismError,
    ParallelDeterminismPlan,
    PARALLEL_DETERMINISM_API_VERSION,
};

// ============================================================================
// SCHEDULER API
// ============================================================================

/// Dependency-aware scheduling and execution reports.
pub use self::scheduler::{
    ParallelScheduler,
    ScheduleReport,
    ScheduledUnitOutcome,
    ScheduledUnitState,
    SchedulerError,
    PARALLEL_SCHEDULER_API_VERSION,
};

// ============================================================================
// WORKER EXECUTION API
// ============================================================================

/// Configurable worker execution and structured job outcomes.
pub use self::workers::{
    WorkerExecutionReport,
    WorkerPool,
    WorkerPoolConfig,
    WorkerPoolError,
    WorkerTaskOutcome,
};

// ============================================================================
// MODULE-BOUNDARY TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::compiler::context::dependency_context::DependencyContext;

    #[test]
    fn all_parallel_components_are_exposed() {
        let workers = WorkerPool::new(WorkerPoolConfig::automatic());

        let scheduler = ParallelScheduler::new(workers);

        assert_eq!(
            scheduler.workers().config(),
            WorkerPoolConfig::automatic()
        );

        let determinism = ParallelDeterminism::unspecified();

        let context = DependencyContext::new();

        let dependency_plan =
            DependencyPlan::from_context(&context)
                .expect("an empty dependency context must be valid");

        let collection_plan =
            determinism
                .for_dependency_plan(&dependency_plan)
                .expect("an empty dependency plan must be valid");

        assert!(collection_plan.is_empty());
    }

    #[test]
    fn canonical_api_versions_are_exposed() {
        assert_eq!(PARALLEL_DEPENDENCY_API_VERSION, 1);
        assert_eq!(PARALLEL_DETERMINISM_API_VERSION, 1);
        assert_eq!(PARALLEL_SCHEDULER_API_VERSION, 1);
    }

    #[test]
    fn empty_worker_budget_configuration_is_automatic() {
        let configuration = WorkerPoolConfig::automatic();

        let workers = WorkerPool::new(configuration);

        assert_eq!(workers.worker_count(0), 0);
    }

    #[test]
    fn empty_dependency_context_produces_no_waves() {
        let context = DependencyContext::new();

        let plan = DependencyPlan::from_context(&context)
            .expect("empty context must produce a valid plan");

        assert!(plan.is_empty());
        assert_eq!(plan.wave_count(), 0);
        assert_eq!(plan.unit_count(), 0);
    }

    #[test]
    fn parallel_cancellation_is_the_canonical_classical_token() {
        use std::any::TypeId;

        assert_eq!(
            TypeId::of::<ParallelCancellationToken>(),
            TypeId::of::<
                crate::classical::concurrency::CancellationToken
            >()
        );
    }
}
