
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Universal Meta-Compiler — Parallel Build Orchestration.
//!
//! File:
//!     src/compiler/parallel/build.rs
//!
//! Rust:
//!     Edition 2021
//!     Minimum supported version: Rust 1.97
//!
//! Safety:
//!     Safe Rust only. Unsafe Rust is forbidden.
//!
//! ============================================================================
//! PURPOSE
//! ============================================================================
//!
//! This module coordinates parallel compilation builds using the existing
//! canonical compiler subsystems:
//!
//!     compiler::context::dependency_context
//!     compiler::parallel::dependencies
//!     compiler::parallel::workers
//!     compiler::parallel::scheduler
//!     compiler::parallel::determinism
//!
//! It provides a stable compiler-facing API for executing a complete
//! dependency-aware build.
//!
//! It supports:
//!
//! - independent compilation units;
//! - dependency-constrained compilation;
//! - classical, quantum, AI, HDL, tensor and hybrid compilation units;
//! - distributed compilation units;
//! - configurable worker budgets;
//! - cooperative cancellation;
//! - structured task failures;
//! - panic reporting;
//! - dependency failure propagation;
//! - deterministic result collection;
//! - incremental artifact reuse;
//! - empty and small builds;
//! - builds constrained only by actual resources and configured budgets.
//!
//! ============================================================================
//! OWNERSHIP
//! ============================================================================
//!
//! This module owns:
//!
//! - Build-plan composition.
//! - Build execution orchestration.
//! - Build result aggregation.
//! - Build-level metrics.
//! - The distinction between newly built and reused results.
//!
//! This module does NOT own:
//!
//! - Dependency discovery.
//! - Dependency graph algorithms.
//! - Worker-thread implementation.
//! - Cancellation-token implementation.
//! - Incremental invalidation.
//! - Fingerprint generation.
//! - Artifact cache storage.
//! - Artifact integrity verification.
//! - Source parsing.
//! - Semantic analysis.
//! - Type inference.
//! - Ownership analysis.
//! - Canonical IR definitions.
//! - Optimization.
//! - Backend code generation.
//! - Hardware discovery.
//! - Physical resource allocation.
//! - Runtime execution.
//! - Distributed execution.
//!
//! ============================================================================
//! CANONICAL AUTHORITIES
//! ============================================================================
//!
//! Dependency graph:
//!     crate::compiler::context::dependency_context
//!
//! Dependency-ready waves:
//!     crate::compiler::parallel::dependencies
//!
//! Worker execution:
//!     crate::compiler::parallel::workers
//!
//! Dependency-aware scheduling:
//!     crate::compiler::parallel::scheduler
//!
//! Deterministic result collection:
//!     crate::compiler::parallel::determinism
//!
//! Cancellation:
//!     crate::classical::concurrency::CancellationToken
//!
//! Incremental compilation:
//!     crate::compiler::incremental
//!
//! These modules remain the authoritative owners of their respective
//! contracts. This file composes them without redefining them.
//!
//! ============================================================================
//! INCREMENTAL BUILD CONTRACT
//! ============================================================================
//!
//! Incremental compilation determines which artifacts are valid, dirty,
//! invalidated, or reusable.
//!
//! This module does not independently decide cache validity.
//!
//! A build callback may return:
//!
//!     BuildTaskResult::Built(value)
//!
//! when a new compilation result was produced, or:
//!
//!     BuildTaskResult::Reused(value)
//!
//! when an existing artifact was successfully retrieved and verified.
//!
//! A caller must never report an artifact as Reused merely because a cache
//! entry exists. The caller must first establish that the artifact is valid
//! for the current compilation identity, fingerprints, dependencies,
//! semantic inputs, compiler configuration and target realization.
//!
//! Both Built and Reused are successful dependency prerequisites.
//!
//! A failed, panicked, blocked or cancelled prerequisite is not successful.
//!
//! ============================================================================
//! DETERMINISM
//! ============================================================================
//!
//! The dependency plan defines canonical dependency-first ordering.
//!
//! The scheduler executes dependency-ready waves.
//!
//! The determinism adapter validates and collects every unit result against
//! that canonical order.
//!
//! Completion order, thread identity and worker assignment do not determine
//! the final report order.
//!
//! This does not independently guarantee deterministic callback side effects,
//! floating-point reductions, backend output or runtime execution.
//!
//! Those properties must be established by their respective semantic,
//! optimization, numerical and execution contracts.
//!
//! ============================================================================
//! SCALABILITY
//! ============================================================================
//!
//! No arbitrary maximum is imposed here on:
//!
//! - Compilation units.
//! - Dependency relationships.
//! - Dependency waves.
//! - Computing domains.
//! - Workers.
//! - CPUs.
//! - GPUs.
//! - FPGAs.
//! - QPUs.
//! - Distributed nodes.
//! - Hybrid partitions.
//! - Artifact count.
//!
//! Effective concurrency is delegated to WorkerPool.
//!
//! Actual scalability is constrained by:
//!
//! - available host resources;
//! - the configured worker budget;
//! - operating-system thread limits;
//! - memory availability;
//! - allocation success;
//! - dependency graph structure;
//! - compilation workload;
//! - backend and target capabilities.
//!
//! No finite implementation promises infinite physical resources.
//!
//! ============================================================================
//! FAILURE SEMANTICS
//! ============================================================================
//!
//! Ordinary task errors are retained as Failed outcomes.
//!
//! Panics are retained as Panicked outcomes when panic unwinding is enabled.
//!
//! Failed prerequisites block their dependents.
//!
//! Independent branches remain eligible for execution.
//!
//! Cancellation is cooperative and uses the caller-provided canonical token.
//!
//! This module never cancels a caller-owned token merely because one
//! compilation unit failed.
//!
//! ============================================================================
//! GRAMMAR BOUNDARY
//! ============================================================================
//!
//! No grammar modification is required.
//!
//! The grammar describes portable concurrency, task, ordering, cancellation
//! and determinism intent.
//!
//! This file implements compiler build orchestration, not source-language
//! syntax or source-level concurrency semantics.
//!
//! Relevant grammar authorities:
//!
//!     grammar/Zamani.g4
//!     grammar/concurrency/concurrency.g4
//!     grammar/concurrency/README.md
//!     grammar/concurrency/cancellation.g4
//!     grammar/compile/README.md
//!     grammar/validation/semantic-boundaries.md
//!
//! ============================================================================

use std::error::Error;
use std::fmt;

use crate::classical::concurrency::CancellationToken;

use crate::compiler::context::dependency_context::DependencyContext;

use crate::compiler::parallel::dependencies::{
    DependencyPlan,
    DependencyPlanError,
};

use crate::compiler::parallel::determinism::{
    OrderedCompilationResult,
    ParallelDeterminism,
    ParallelDeterminismPlan,
    ParallelDeterminismError,
};

use crate::compiler::parallel::scheduler::{
    ParallelScheduler,
    ScheduleReport,
    ScheduledUnitState,
    SchedulerError,
};

use crate::compiler::parallel::workers::WorkerPool;

// ============================================================================
// API VERSION
// ============================================================================

/// Public API version for parallel build orchestration.
///
/// This version is independent of the Zamani language, grammar, semantic
/// model, canonical IR, backend ABI and runtime versions.
pub const PARALLEL_BUILD_API_VERSION: u16 = 1;

// ============================================================================
// BUILD TASK RESULT
// ============================================================================

/// Describes how a successful build unit obtained its result.
///
/// `Built` means the callback performed the required compilation work.
///
/// `Reused` means the callback retrieved and validated an existing artifact
/// using the canonical incremental-compilation and cache contracts.
///
/// Both variants represent successful completion for dependency purposes.
///
/// The enum does not own or define artifact storage.
#[derive(Debug)]
pub enum BuildTaskResult<T> {
    /// A new compilation result was produced.
    Built(T),

    /// A previously produced and validated result was reused.
    Reused(T),
}

impl<T> BuildTaskResult<T> {
    /// Returns whether this result was newly built.
    #[must_use]
    pub const fn is_built(&self) -> bool {
        matches!(self, Self::Built(_))
    }

    /// Returns whether this result was reused.
    #[must_use]
    pub const fn is_reused(&self) -> bool {
        matches!(self, Self::Reused(_))
    }

    /// Returns a reference to the underlying result.
    #[must_use]
    pub fn value(&self) -> &T {
        match self {
            Self::Built(value) | Self::Reused(value) => value,
        }
    }

    /// Consumes the result and returns its underlying value.
    #[must_use]
    pub fn into_value(self) -> T {
        match self {
            Self::Built(value) | Self::Reused(value) => value,
        }
    }
}

// ============================================================================
// BUILD PLAN
// ============================================================================

/// Immutable plan for one parallel compiler build.
///
/// A plan combines:
///
/// - the canonical dependency plan;
/// - the canonical determinism policy;
/// - the deterministic result-collection plan.
///
/// The dependency graph and deterministic ordering are not independently
/// reconstructed by this module.
///
/// Build plans are immutable and may be reused for multiple executions
/// provided the caller continues to use the same valid dependency snapshot
/// and compilation inputs.
#[derive(Clone, Debug)]
pub struct ParallelBuildPlan {
    dependency_plan: DependencyPlan,
    determinism_plan: ParallelDeterminismPlan,
}

impl ParallelBuildPlan {
    /// Constructs a build plan from an existing canonical dependency plan.
    ///
    /// The dependency plan remains the source of truth for unit identities
    /// and dependency-first ordering.
    pub fn from_dependency_plan(
        dependency_plan: DependencyPlan,
        determinism: ParallelDeterminism,
    ) -> Result<Self, ParallelBuildCoordinatorError> {
        let determinism_plan =
            determinism.for_dependency_plan(&dependency_plan)?;

        Ok(Self {
            dependency_plan,
            determinism_plan,
        })
    }

    /// Constructs a build plan from the canonical dependency context.
    ///
    /// Dependency discovery and graph construction must already have been
    /// completed by the frontend, semantic layer or build-system adapter.
    pub fn from_context(
        context: &DependencyContext,
        determinism: ParallelDeterminism,
    ) -> Result<Self, ParallelBuildCoordinatorError> {
        let dependency_plan =
            DependencyPlan::from_context(context)?;

        Self::from_dependency_plan(
            dependency_plan,
            determinism,
        )
    }

    /// Returns the canonical dependency plan.
    #[must_use]
    pub fn dependency_plan(&self) -> &DependencyPlan {
        &self.dependency_plan
    }

    /// Returns the deterministic result-collection plan.
    #[must_use]
    pub fn determinism_plan(&self) -> &ParallelDeterminismPlan {
        &self.determinism_plan
    }

    /// Returns the number of compilation units.
    #[must_use]
    pub fn unit_count(&self) -> usize {
        self.dependency_plan.unit_count()
    }

    /// Returns the number of dependency-ready waves.
    #[must_use]
    pub fn wave_count(&self) -> usize {
        self.dependency_plan.wave_count()
    }

    /// Returns the number of dependency relationships.
    #[must_use]
    pub fn dependency_count(&self) -> usize {
        self.dependency_plan.edge_count()
    }

    /// Returns whether the build plan is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.dependency_plan.is_empty()
    }

    /// Returns whether the dependency structure contains parallelism.
    ///
    /// This describes potential parallelism, not guaranteed physical
    /// concurrency.
    #[must_use]
    pub fn has_parallelism(&self) -> bool {
        self.dependency_plan.has_parallelism()
    }
}

// ============================================================================
// BUILD ERRORS
// ============================================================================

/// Errors produced by parallel build orchestration.
#[derive(Debug)]
#[non_exhaustive]
pub enum ParallelBuildCoordinatorError {
    /// The canonical dependency graph could not be converted into a plan.
    DependencyPlan(DependencyPlanError),

    /// Deterministic result-collection planning or validation failed.
    Determinism(ParallelDeterminismError),

    /// Dependency-aware execution failed.
    Scheduler(SchedulerError),
}

impl fmt::Display for ParallelBuildCoordinatorError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DependencyPlan(error) => {
                write!(
                    formatter,
                    "parallel build dependency planning failed: {error}"
                )
            }

            Self::Determinism(error) => {
                write!(
                    formatter,
                    "parallel build determinism validation failed: {error}"
                )
            }

            Self::Scheduler(error) => {
                write!(
                    formatter,
                    "parallel build execution failed: {error}"
                )
            }
        }
    }
}

impl Error for ParallelBuildCoordinatorError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::DependencyPlan(error) => Some(error),
            Self::Determinism(error) => Some(error),
            Self::Scheduler(error) => Some(error),
        }
    }
}

impl From<DependencyPlanError> for ParallelBuildCoordinatorError {
    fn from(error: DependencyPlanError) -> Self {
        Self::DependencyPlan(error)
    }
}

impl From<ParallelDeterminismError> for ParallelBuildCoordinatorError {
    fn from(error: ParallelDeterminismError) -> Self {
        Self::Determinism(error)
    }
}

impl From<SchedulerError> for ParallelBuildCoordinatorError {
    fn from(error: SchedulerError) -> Self {
        Self::Scheduler(error)
    }
}

// ============================================================================
// BUILD REPORT
// ============================================================================

/// Complete result of a parallel build.
///
/// Results are returned in canonical dependency-first order.
///
/// A successful result can represent either a newly built artifact or a
/// successfully reused artifact.
///
/// Failed, panicked, blocked and cancelled states remain explicit.
#[derive(Debug)]
pub struct ParallelBuildReport<T, E> {
    outcomes: Vec<
        OrderedCompilationResult<
            ScheduledUnitState<BuildTaskResult<T>, E>,
        >,
    >,

    /// Total worker threads created across all dependency waves.
    worker_threads_created: usize,

    /// Maximum worker count used for an individual dependency wave.
    maximum_workers_in_wave: usize,

    /// Number of compilation callbacks actually started.
    started_units: usize,
}

impl<T, E> ParallelBuildReport<T, E> {
    /// Returns all outcomes in canonical dependency-first order.
    #[must_use]
    pub fn outcomes(
        &self,
    ) -> &[OrderedCompilationResult<
        ScheduledUnitState<BuildTaskResult<T>, E>,
    >] {
        &self.outcomes
    }

    /// Consumes the report and returns ordered outcomes.
    #[must_use]
    pub fn into_outcomes(
        self,
    ) -> Vec<
        OrderedCompilationResult<
            ScheduledUnitState<BuildTaskResult<T>, E>,
        >,
    > {
        self.outcomes
    }

    /// Returns the total worker threads created across all waves.
    #[must_use]
    pub const fn worker_threads_created(&self) -> usize {
        self.worker_threads_created
    }

    /// Returns the maximum worker count used in one wave.
    #[must_use]
    pub const fn maximum_workers_in_wave(&self) -> usize {
        self.maximum_workers_in_wave
    }

    /// Returns the number of callbacks actually started.
    #[must_use]
    pub const fn started_units(&self) -> usize {
        self.started_units
    }

    /// Returns the number of planned compilation units.
    #[must_use]
    pub fn unit_count(&self) -> usize {
        self.outcomes.len()
    }

    /// Returns the number of successfully completed units.
    #[must_use]
    pub fn successful_units(&self) -> usize {
        self.outcomes
            .iter()
            .filter(|outcome| {
                matches!(
                    outcome.value(),
                    ScheduledUnitState::Completed(_)
                )
            })
            .count()
    }

    /// Returns the number of units newly built.
    #[must_use]
    pub fn built_units(&self) -> usize {
        self.outcomes
            .iter()
            .filter(|outcome| {
                matches!(
                    outcome.value(),
                    ScheduledUnitState::Completed(
                        BuildTaskResult::Built(_)
                    )
                )
            })
            .count()
    }

    /// Returns the number of units whose artifacts were reused.
    #[must_use]
    pub fn reused_units(&self) -> usize {
        self.outcomes
            .iter()
            .filter(|outcome| {
                matches!(
                    outcome.value(),
                    ScheduledUnitState::Completed(
                        BuildTaskResult::Reused(_)
                    )
                )
            })
            .count()
    }

    /// Returns the number of failed or panicked units.
    #[must_use]
    pub fn failed_units(&self) -> usize {
        self.outcomes
            .iter()
            .filter(|outcome| {
                matches!(
                    outcome.value(),
                    ScheduledUnitState::Failed(_)
                        | ScheduledUnitState::Panicked { .. }
                )
            })
            .count()
    }

    /// Returns the number of units blocked by unsuccessful prerequisites.
    #[must_use]
    pub fn blocked_units(&self) -> usize {
        self.outcomes
            .iter()
            .filter(|outcome| {
                matches!(
                    outcome.value(),
                    ScheduledUnitState::Blocked { .. }
                )
            })
            .count()
    }

    /// Returns the number of cancelled or unstarted units.
    #[must_use]
    pub fn cancelled_units(&self) -> usize {
        self.outcomes
            .iter()
            .filter(|outcome| {
                matches!(
                    outcome.value(),
                    ScheduledUnitState::Cancelled
                )
            })
            .count()
    }

    /// Returns whether the report contains no units.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.outcomes.is_empty()
    }

    /// Returns whether every unit completed successfully.
    ///
    /// An empty build is a successful empty build.
    #[must_use]
    pub fn is_success(&self) -> bool {
        self.outcomes.iter().all(|outcome| {
            matches!(
                outcome.value(),
                ScheduledUnitState::Completed(_)
            )
        })
    }

    /// Returns whether every unit has a non-cancelled final state.
    ///
    /// Failed and blocked units are terminal outcomes.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.outcomes.iter().all(|outcome| {
            !matches!(
                outcome.value(),
                ScheduledUnitState::Cancelled
            )
        })
    }
}

// ============================================================================
// BUILD COORDINATOR
// ============================================================================

/// Compiler-facing parallel build coordinator.
///
/// This type composes the existing scheduler and determinism contracts.
///
/// It does not create another worker pool, dependency graph, cache,
/// invalidation planner or compiler pipeline.
#[derive(Clone, Debug)]
pub struct ParallelBuildCoordinator {
    scheduler: ParallelScheduler,
    determinism: ParallelDeterminism,
}

impl Default for ParallelBuildCoordinator {
    fn default() -> Self {
        Self::automatic()
    }
}

impl ParallelBuildCoordinator {
    /// Creates a coordinator from an existing scheduler and canonical
    /// determinism adapter.
    #[must_use]
    pub const fn new(
        scheduler: ParallelScheduler,
        determinism: ParallelDeterminism,
    ) -> Self {
        Self {
            scheduler,
            determinism,
        }
    }

    /// Creates a coordinator using automatic worker configuration and the
    /// canonical unspecified determinism policy.
    ///
    /// Deterministic result collection is still performed. The unspecified
    /// policy does not mean that results are returned in arbitrary order.
    #[must_use]
    pub const fn automatic() -> Self {
        Self {
            scheduler: ParallelScheduler::automatic(),
            determinism: ParallelDeterminism::unspecified(),
        }
    }

    /// Creates a coordinator using an existing worker pool.
    ///
    /// The worker pool retains ownership of its configured concurrency
    /// budget and fail-fast behavior.
    #[must_use]
    pub const fn with_worker_pool(
        workers: WorkerPool,
    ) -> Self {
        Self {
            scheduler: ParallelScheduler::new(workers),
            determinism: ParallelDeterminism::unspecified(),
        }
    }

    /// Returns the scheduler used by this coordinator.
    #[must_use]
    pub const fn scheduler(&self) -> &ParallelScheduler {
        &self.scheduler
    }

    /// Returns the canonical determinism adapter.
    #[must_use]
    pub const fn determinism(&self) -> &ParallelDeterminism {
        &self.determinism
    }

    /// Constructs an immutable build plan from a canonical dependency
    /// context.
    pub fn plan(
        &self,
        context: &DependencyContext,
    ) -> Result<ParallelBuildPlan, ParallelBuildCoordinatorError> {
        ParallelBuildPlan::from_context(
            context,
            self.determinism,
        )
    }

    /// Executes an existing immutable build plan.
    ///
    /// The callback receives:
    ///
    /// - the stable compilation-unit identity;
    /// - the canonical cooperative cancellation token.
    ///
    /// It must return Built for newly produced output or Reused for an
    /// existing artifact that has already been validated by the incremental
    /// compilation/cache subsystem.
    ///
    /// The callback may be invoked concurrently for independent units.
    ///
    /// It must therefore be safe for concurrent invocation.
    ///
    /// The callback must not assume a particular worker identity or
    /// completion order.
    pub fn execute<T, E, F>(
        &self,
        plan: &ParallelBuildPlan,
        cancellation: &CancellationToken,
        task: F,
    ) -> Result<
        ParallelBuildReport<T, E>,
        ParallelBuildCoordinatorError,
    >
    where
        T: Send,
        E: Send,
        F: Fn(
                &crate::compiler::context::dependency_context::CompilationUnitId,
                &CancellationToken,
            ) -> Result<BuildTaskResult<T>, E>
            + Sync,
    {
        let schedule: ScheduleReport<BuildTaskResult<T>, E> =
            self.scheduler.execute(
                plan.dependency_plan(),
                cancellation,
                task,
            )?;

        // Preserve scheduler metrics before consuming its outcomes.
        let worker_threads_created =
            schedule.worker_threads_created();

        let maximum_workers_in_wave =
            schedule.maximum_workers_in_wave();

        let started_units =
            schedule.started_units();

        // The scheduler already returns dependency-first results.
        // The determinism adapter independently validates that every
        // planned unit occurs exactly once and that no unexpected unit
        // has entered the report.
        let unordered_results = schedule
            .into_outcomes()
            .into_iter()
            .map(|outcome| outcome.into_parts())
            .collect::<Vec<_>>();

        let outcomes = plan
            .determinism_plan()
            .collect_results(unordered_results)?;

        Ok(ParallelBuildReport {
            outcomes,
            worker_threads_created,
            maximum_workers_in_wave,
            started_units,
        })
    }

    /// Builds directly from an existing canonical dependency context.
    ///
    /// This is a convenience method equivalent to:
    ///
    ///     let plan = coordinator.plan(context)?;
    ///     coordinator.execute(&plan, cancellation, task)
    ///
    /// Dependency discovery and semantic validation must already have
    /// occurred before this method is called.
    pub fn build<T, E, F>(
        &self,
        context: &DependencyContext,
        cancellation: &CancellationToken,
        task: F,
    ) -> Result<
        ParallelBuildReport<T, E>,
        ParallelBuildCoordinatorError,
    >
    where
        T: Send,
        E: Send,
        F: Fn(
                &crate::compiler::context::dependency_context::CompilationUnitId,
                &CancellationToken,
            ) -> Result<BuildTaskResult<T>, E>
            + Sync,
    {
        let plan = self.plan(context)?;

        self.execute(
            &plan,
            cancellation,
            task,
        )
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::BTreeMap;
    use std::sync::atomic::{AtomicUsize, Ordering};

    use crate::compiler::context::dependency_context::{
        CompilationUnit,
        CompilationUnitId,
        CompilationUnitKind,
        DependencyContextBuilder,
        DependencyKind,
    };

    fn make_id(value: &str) -> CompilationUnitId {
        CompilationUnitId::new(value)
            .expect("test compilation-unit identity must be valid")
    }

    fn make_context(
        identifiers: &[&str],
        dependencies: &[(&str, &str)],
    ) -> DependencyContext {
        let mut builder = DependencyContextBuilder::new();

        for identifier in identifiers {
            let id = make_id(identifier);

            let kind = CompilationUnitKind::new("source")
                .expect("test unit kind must be valid");

            let unit = CompilationUnit::new(
                id,
                kind,
                *identifier,
            )
            .expect("test compilation unit must be valid");

            builder
                .add_unit(unit)
                .expect("test unit must be registered");
        }

        for (dependent, dependency) in dependencies {
            builder
                .add_dependency(
                    make_id(dependent),
                    make_id(dependency),
                    DependencyKind::BuildInput,
                )
                .expect("test dependency must be valid");
        }

        builder
            .build()
            .expect("test dependency graph must be valid")
    }

    #[test]
    fn empty_build_is_successful() {
        let context = make_context(&[], &[]);
        let coordinator = ParallelBuildCoordinator::automatic();
        let cancellation = CancellationToken::new();

        let report = coordinator
            .build(
                &context,
                &cancellation,
                |_unit, _token| {
                    Ok::<BuildTaskResult<()>, String>(
                        BuildTaskResult::Built(()),
                    )
                },
            )
            .expect("empty build must succeed");

        assert!(report.is_empty());
        assert!(report.is_success());
        assert_eq!(report.unit_count(), 0);
        assert_eq!(report.started_units(), 0);
        assert_eq!(report.worker_threads_created(), 0);
    }

    #[test]
    fn results_are_returned_in_dependency_first_order() {
        let context = make_context(
            &["application", "library", "independent"],
            &[("application", "library")],
        );

        let coordinator = ParallelBuildCoordinator::automatic();
        let cancellation = CancellationToken::new();

        let report = coordinator
            .build(
                &context,
                &cancellation,
                |unit, _token| {
                    Ok::<BuildTaskResult<String>, String>(
                        BuildTaskResult::Built(
                            unit.as_str().to_owned(),
                        ),
                    )
                },
            )
            .expect("build must succeed");

        assert!(report.is_success());
        assert_eq!(report.built_units(), 3);
        assert_eq!(report.reused_units(), 0);

        let positions: BTreeMap<&str, usize> = report
            .outcomes()
            .iter()
            .enumerate()
            .map(|(index, outcome)| {
                (outcome.unit().as_str(), index)
            })
            .collect();

        assert!(
            positions["library"] < positions["application"],
            "a dependency must precede its dependent"
        );
    }

    #[test]
    fn validated_reused_results_are_successful_prerequisites() {
        let context = make_context(
            &["application", "library"],
            &[("application", "library")],
        );

        let coordinator = ParallelBuildCoordinator::automatic();
        let cancellation = CancellationToken::new();

        let report = coordinator
            .build(
                &context,
                &cancellation,
                |unit, _token| {
                    if unit.as_str() == "library" {
                        Ok::<BuildTaskResult<String>, String>(
                            BuildTaskResult::Reused(
                                "cached-library".to_owned(),
                            ),
                        )
                    } else {
                        Ok(BuildTaskResult::Built(
                            "new-application".to_owned(),
                        ))
                    }
                },
            )
            .expect("build with reused dependency must succeed");

        assert!(report.is_success());
        assert_eq!(report.successful_units(), 2);
        assert_eq!(report.built_units(), 1);
        assert_eq!(report.reused_units(), 1);

        let library = report
            .outcomes()
            .iter()
            .find(|outcome| outcome.unit().as_str() == "library")
            .expect("library result must exist");

        assert!(matches!(
            library.value(),
            ScheduledUnitState::Completed(
                BuildTaskResult::Reused(_)
            )
        ));
    }

    #[test]
    fn failed_prerequisite_blocks_dependents() {
        let context = make_context(
            &[
                "application",
                "library",
                "independent",
            ],
            &[("application", "library")],
        );

        let coordinator = ParallelBuildCoordinator::automatic();
        let cancellation = CancellationToken::new();

        let report = coordinator
            .build(
                &context,
                &cancellation,
                |unit, _token| {
                    if unit.as_str() == "library" {
                        Err::<BuildTaskResult<()>, String>(
                            "library compilation failed".to_owned(),
                        )
                    } else {
                        Ok(BuildTaskResult::Built(()))
                    }
                },
            )
            .expect("individual compilation failures are report outcomes");

        assert_eq!(report.failed_units(), 1);
        assert_eq!(report.blocked_units(), 1);

        let application = report
            .outcomes()
            .iter()
            .find(|outcome| {
                outcome.unit().as_str() == "application"
            })
            .expect("application result must exist");

        assert!(matches!(
            application.value(),
            ScheduledUnitState::Blocked { .. }
        ));

        let independent = report
            .outcomes()
            .iter()
            .find(|outcome| {
                outcome.unit().as_str() == "independent"
            })
            .expect("independent result must exist");

        assert!(matches!(
            independent.value(),
            ScheduledUnitState::Completed(_)
        ));
    }

    #[test]
    fn pre_cancelled_build_does_not_invoke_callbacks() {
        let context = make_context(
            &["first", "second"],
            &[],
        );

        let coordinator = ParallelBuildCoordinator::automatic();

        let cancellation = CancellationToken::new();
        cancellation.cancel();

        let calls = AtomicUsize::new(0);

        let report = coordinator
            .build(
                &context,
                &cancellation,
                |_unit, _token| {
                    calls.fetch_add(1, Ordering::Relaxed);

                    Ok::<BuildTaskResult<()>, String>(
                        BuildTaskResult::Built(()),
                    )
                },
            )
            .expect("cancellation must produce a report");

        assert_eq!(calls.load(Ordering::Relaxed), 0);
        assert_eq!(report.cancelled_units(), 2);
        assert!(!report.is_success());
        assert!(!report.is_complete());
    }

    #[test]
    fn report_preserves_one_outcome_per_planned_unit() {
        let context = make_context(
            &[
                "classical",
                "quantum",
                "ai",
                "hdl",
                "distributed",
            ],
            &[
                ("quantum", "classical"),
                ("ai", "classical"),
                ("hdl", "classical"),
                ("distributed", "quantum"),
                ("distributed", "ai"),
                ("distributed", "hdl"),
            ],
        );

        let coordinator = ParallelBuildCoordinator::automatic();
        let cancellation = CancellationToken::new();

        let plan = coordinator
            .plan(&context)
            .expect("build plan must be valid");

        let expected_units = plan.unit_count();

        let report = coordinator
            .execute(
                &plan,
                &cancellation,
                |unit, _token| {
                    Ok::<BuildTaskResult<String>, String>(
                        BuildTaskResult::Built(
                            unit.as_str().to_owned(),
                        ),
                    )
                },
            )
            .expect("hybrid dependency build must succeed");

        assert_eq!(report.unit_count(), expected_units);
        assert_eq!(report.successful_units(), expected_units);
        assert_eq!(report.outcomes().len(), expected_units);
    }
}
