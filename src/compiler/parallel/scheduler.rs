
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Universal Meta-Compiler — Dependency-Aware Parallel Scheduler.
//!
//! File:
//!     src/compiler/parallel/scheduler.rs
//!
//! Rust:
//!     Edition 2021
//!     Minimum supported version: Rust 1.97
//!
//! Safety:
//!     Safe Rust only. Unsafe Rust is forbidden.
//!
//! # Purpose
//!
//! Executes compilation units in dependency-ready waves.
//!
//! The scheduler consumes the canonical dependency plan produced by:
//!
//!     crate::compiler::parallel::dependencies
//!
//! Independent units within a wave may execute concurrently through:
//!
//!     crate::compiler::parallel::workers
//!
//! # Ownership
//!
//! This module owns:
//!
//! - Dependency-aware compilation execution.
//! - Dependency readiness checks.
//! - Blocking dependents after prerequisite failures.
//! - Scheduler-level cancellation handling.
//! - Deterministic result collection.
//! - Scheduler execution reporting.
//!
//! It does not own:
//!
//! - Dependency discovery.
//! - Dependency graph construction.
//! - Source parsing.
//! - Semantic analysis.
//! - Incremental invalidation.
//! - Worker-thread implementation.
//! - Physical hardware allocation.
//! - Backend selection.
//! - Runtime scheduling.
//! - Distributed execution.
//! - Compiler IR.
//!
//! # Architecture
//!
//! Frontend / semantic dependency discovery
//!                 |
//!                 v
//!       DependencyContext
//!                 |
//!                 v
//!         DependencyPlan
//!                 |
//!                 v
//!       ParallelScheduler
//!                 |
//!          +------+
//!          |
//!          v
//!     Dependency wave
//!          |
//!          v
//!       WorkerPool
//!          |
//!          v
//!     Ordered outcomes
//!
//! # Dependency semantics
//!
//! The canonical dependency graph expresses:
//!
//!     dependent -> dependency
//!
//! A dependency must complete successfully before its dependent can run.
//!
//! Only ordering dependencies participate in compilation readiness.
//!
//! Data-flow, feedback, synchronization and other non-ordering relationships
//! remain owned by the canonical dependency context.
//!
//! # Failure semantics
//!
//! A failed or panicked compilation unit prevents its dependents from
//! executing.
//!
//! Independent compilation branches remain eligible for execution.
//!
//! A blocked unit is not treated as a failed compilation. It is a unit that
//! could not legally begin because one or more prerequisites did not succeed.
//!
//! # Cancellation
//!
//! Uses the canonical classical concurrency CancellationToken.
//!
//! Cancellation is cooperative. Already-running jobs cannot be forcibly
//! interrupted by this scheduler.
//!
//! # Determinism
//!
//! The scheduler returns results in the canonical dependency-first order.
//!
//! Completion order and worker assignment order do not affect result order.
//!
//! # Scalability
//!
//! No universal maximum is imposed on:
//!
//! - Compilation units.
//! - Dependency waves.
//! - Dependencies.
//! - Dependents.
//! - Worker threads.
//! - Computing domains.
//! - CPUs.
//! - GPUs.
//! - FPGAs.
//! - QPUs.
//! - Distributed nodes.
//!
//! Actual execution is constrained by available memory, operating-system
//! resources, configured worker budgets, and the compilation workload.
//!
//! No finite implementation can promise infinite physical resources.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use crate::classical::concurrency::CancellationToken;

use crate::compiler::parallel::dependencies::{
    DependencyPlan,
};

use crate::compiler::parallel::workers::{
    WorkerPool,
    WorkerPoolError,
    WorkerTaskOutcome,
};

// ============================================================================
// API VERSION
// ============================================================================

/// Public API version of the parallel scheduler.
///
/// This version is independent of the Zamani language, grammar, semantic
/// model, IR, backend, and runtime versions.
pub const PARALLEL_SCHEDULER_API_VERSION: u16 = 1;

// ============================================================================
// SCHEDULED UNIT STATE
// ============================================================================

/// Final state of an individual scheduled compilation unit.
///
/// Every unit in the dependency plan receives exactly one final state
/// when the scheduler returns successfully.
#[derive(Debug)]
pub enum ScheduledUnitState<T, E> {
    /// The compilation unit completed successfully.
    Completed(T),

    /// The compilation unit returned an application error.
    Failed(E),

    /// The compilation unit panicked during execution.
    ///
    /// Panics are reported by the worker executor when panic unwinding
    /// is enabled.
    Panicked {
        message: String,
    },

    /// The compilation unit was not executed because one or more
    /// ordering prerequisites did not complete successfully.
    Blocked {
        prerequisites: Vec<crate::compiler::context::dependency_context::CompilationUnitId>,
    },

    /// The compilation unit was not executed because cancellation
    /// or worker-level fail-fast prevented its assignment.
    Cancelled,
}

impl<T, E> ScheduledUnitState<T, E> {
    /// Whether the unit completed successfully.
    #[must_use]
    pub fn is_success(&self) -> bool {
        matches!(self, Self::Completed(_))
    }

    /// Whether the unit failed during execution.
    #[must_use]
    pub fn is_failure(&self) -> bool {
        matches!(
            self,
            Self::Failed(_) | Self::Panicked { .. }
        )
    }

    /// Whether the unit was blocked by an unsuccessful prerequisite.
    #[must_use]
    pub fn is_blocked(&self) -> bool {
        matches!(self, Self::Blocked { .. })
    }

    /// Whether execution of the unit was cancelled or not started.
    #[must_use]
    pub fn is_cancelled(&self) -> bool {
        matches!(self, Self::Cancelled)
    }

    /// Whether the unit produced a successful result.
    ///
    /// A blocked or cancelled unit is not considered successful.
    #[must_use]
    pub fn is_terminal_success(&self) -> bool {
        self.is_success()
    }
}

// ============================================================================
// SCHEDULED UNIT RESULT
// ============================================================================

/// Result for one compilation unit.
///
/// Results are returned in the canonical dependency-first order.
#[derive(Debug)]
pub struct ScheduledUnitOutcome<T, E> {
    unit: crate::compiler::context::dependency_context::CompilationUnitId,
    state: ScheduledUnitState<T, E>,
}

impl<T, E> ScheduledUnitOutcome<T, E> {
    /// Returns the stable compilation-unit identity.
    #[must_use]
    pub fn unit(
        &self,
    ) -> &crate::compiler::context::dependency_context::CompilationUnitId {
        &self.unit
    }

    /// Returns the final execution state.
    #[must_use]
    pub fn state(&self) -> &ScheduledUnitState<T, E> {
        &self.state
    }

    /// Consumes the result and returns its identity and state.
    #[must_use]
    pub fn into_parts(
        self,
    ) -> (
        crate::compiler::context::dependency_context::CompilationUnitId,
        ScheduledUnitState<T, E>,
    ) {
        (self.unit, self.state)
    }
}

// ============================================================================
// SCHEDULER ERRORS
// ============================================================================

/// Errors raised by dependency-aware scheduling.
#[derive(Debug)]
#[non_exhaustive]
pub enum SchedulerError {
    /// The supplied dependency plan violates its public invariants.
    InvalidDependencyPlan {
        message: String,
    },

    /// Worker execution could not be established.
    WorkerPool(WorkerPoolError),

    /// A scheduler accounting value exceeded the host representation.
    CounterOverflow,
}

impl fmt::Display for SchedulerError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidDependencyPlan { message } => {
                write!(
                    formatter,
                    "invalid parallel dependency plan: {message}"
                )
            }

            Self::WorkerPool(error) => {
                write!(
                    formatter,
                    "parallel worker execution failed: {error}"
                )
            }

            Self::CounterOverflow => {
                formatter.write_str(
                    "parallel scheduler accounting overflowed",
                )
            }
        }
    }
}

impl Error for SchedulerError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::WorkerPool(error) => Some(error),
            _ => None,
        }
    }
}

impl From<WorkerPoolError> for SchedulerError {
    fn from(error: WorkerPoolError) -> Self {
        Self::WorkerPool(error)
    }
}

// ============================================================================
// SCHEDULE REPORT
// ============================================================================

/// Complete report from dependency-aware parallel execution.
#[derive(Debug)]
pub struct ScheduleReport<T, E> {
    outcomes: Vec<ScheduledUnitOutcome<T, E>>,

    /// Total worker threads successfully created across all waves.
    worker_threads_created: usize,

    /// Largest number of worker threads used for one wave.
    maximum_workers_in_wave: usize,

    /// Number of jobs actually started by the worker executor.
    started_units: usize,
}

impl<T, E> ScheduleReport<T, E> {
    /// Returns all outcomes in deterministic dependency-first order.
    #[must_use]
    pub fn outcomes(&self) -> &[ScheduledUnitOutcome<T, E>] {
        &self.outcomes
    }

    /// Consumes the report and returns all ordered outcomes.
    #[must_use]
    pub fn into_outcomes(self) -> Vec<ScheduledUnitOutcome<T, E>> {
        self.outcomes
    }

    /// Returns the number of worker threads created across all waves.
    #[must_use]
    pub const fn worker_threads_created(&self) -> usize {
        self.worker_threads_created
    }

    /// Returns the maximum worker count used in a single wave.
    #[must_use]
    pub const fn maximum_workers_in_wave(&self) -> usize {
        self.maximum_workers_in_wave
    }

    /// Returns the number of compilation jobs actually started.
    #[must_use]
    pub const fn started_units(&self) -> usize {
        self.started_units
    }

    /// Returns the total number of compilation units.
    #[must_use]
    pub fn unit_count(&self) -> usize {
        self.outcomes.len()
    }

    /// Returns the number of successfully completed units.
    #[must_use]
    pub fn successful_units(&self) -> usize {
        self.outcomes
            .iter()
            .filter(|result| result.state.is_success())
            .count()
    }

    /// Returns the number of units that failed or panicked.
    #[must_use]
    pub fn failed_units(&self) -> usize {
        self.outcomes
            .iter()
            .filter(|result| result.state.is_failure())
            .count()
    }

    /// Returns the number of units blocked by prerequisites.
    #[must_use]
    pub fn blocked_units(&self) -> usize {
        self.outcomes
            .iter()
            .filter(|result| result.state.is_blocked())
            .count()
    }

    /// Returns the number of cancelled or unstarted units.
    #[must_use]
    pub fn cancelled_units(&self) -> usize {
        self.outcomes
            .iter()
            .filter(|result| result.state.is_cancelled())
            .count()
    }

    /// Whether the plan contained no compilation units.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.outcomes.is_empty()
    }

    /// Whether every compilation unit completed successfully.
    ///
    /// An empty plan is considered successfully completed.
    #[must_use]
    pub fn is_success(&self) -> bool {
        self.outcomes
            .iter()
            .all(|result| result.state.is_success())
    }

    /// Whether every unit has a final state other than cancellation.
    ///
    /// Blocked units are terminal results, not unfinished work.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.outcomes
            .iter()
            .all(|result| !result.state.is_cancelled())
    }
}

// ============================================================================
// PARALLEL SCHEDULER
// ============================================================================

/// Dependency-aware compilation scheduler.
///
/// This type coordinates the existing dependency planner and worker pool.
/// It does not implement either subsystem again.
#[derive(Clone, Debug, Default)]
pub struct ParallelScheduler {
    workers: WorkerPool,
}

impl ParallelScheduler {
    /// Creates a scheduler using the supplied worker pool.
    ///
    /// The worker pool determines concurrency through its existing
    /// configuration and resource-budget policy.
    #[must_use]
    pub const fn new(workers: WorkerPool) -> Self {
        Self { workers }
    }

    /// Creates a scheduler using automatic worker configuration.
    #[must_use]
    pub const fn automatic() -> Self {
        Self {
            workers: WorkerPool::automatic(),
        }
    }

    /// Returns the worker pool used by this scheduler.
    #[must_use]
    pub const fn workers(&self) -> &WorkerPool {
        &self.workers
    }

    /// Executes a complete dependency plan.
    ///
    /// # Arguments
    ///
    /// `plan`:
    ///     Immutable dependency-ready plan created from the canonical
    ///     DependencyContext.
    ///
    /// `cancellation`:
    ///     The repository's canonical cooperative cancellation token.
    ///
    /// `task`:
    ///     Compilation operation for one ready compilation unit.
    ///
    /// # Execution semantics
    ///
    /// 1. Validate the dependency plan.
    /// 2. Process dependency waves in order.
    /// 3. Identify units whose ordering prerequisites succeeded.
    /// 4. Mark units with unsuccessful prerequisites as blocked.
    /// 5. Execute ready units using the existing WorkerPool.
    /// 6. Preserve individual success, error, panic and cancellation states.
    /// 7. Return results in canonical dependency-first order.
    ///
    /// Independent branches may continue after another branch fails.
    ///
    /// # Cancellation
    ///
    /// If the supplied cancellation token becomes cancelled, no subsequent
    /// wave is submitted. All remaining units receive Cancelled outcomes.
    ///
    /// The worker pool handles cancellation within an active wave.
    ///
    /// # Failure
    ///
    /// A failed or panicked prerequisite blocks its dependents.
    ///
    /// It does not automatically prevent unrelated branches from running.
    ///
    /// # Determinism
    ///
    /// The order of returned outcomes is independent of worker completion
    /// order.
    ///
    /// # Callback requirements
    ///
    /// The callback must be safe to invoke concurrently.
    ///
    /// It must not rely on invocation order unless it implements its own
    /// synchronization.
    pub fn execute<T, E, F>(
        &self,
        plan: &DependencyPlan,
        cancellation: &CancellationToken,
        task: F,
    ) -> Result<ScheduleReport<T, E>, SchedulerError>
    where
        T: Send,
        E: Send,
        F: Fn(
                &crate::compiler::context::dependency_context::CompilationUnitId,
                &CancellationToken,
            ) -> Result<T, E>
            + Sync,
    {
        let wave_lookup = validate_dependency_plan(plan)?;

        let mut states = BTreeMap::<
            crate::compiler::context::dependency_context::CompilationUnitId,
            ScheduledUnitState<T, E>,
        >::new();

        let mut worker_threads_created = 0usize;
        let mut maximum_workers_in_wave = 0usize;
        let mut started_units = 0usize;

        for wave in plan.waves() {
            if cancellation.is_cancelled() {
                break;
            }

            let mut ready_units = Vec::new();

            // ------------------------------------------------------------
            // Determine which units in this wave are legally executable.
            // ------------------------------------------------------------

            for unit in wave.units() {
                let dependencies = plan
                    .context()
                    .ordering_dependencies_of(unit)
                    .ok_or_else(|| {
                        SchedulerError::InvalidDependencyPlan {
                            message: format!(
                                "unit '{unit}' is absent from the canonical dependency context"
                            ),
                        }
                    })?;

                let mut unsuccessful_prerequisites = Vec::new();

                for dependency in dependencies {
                    match states.get(&dependency) {
                        Some(ScheduledUnitState::Completed(_)) => {}

                        Some(_) => {
                            unsuccessful_prerequisites.push(dependency);
                        }

                        None => {
                            return Err(
                                SchedulerError::InvalidDependencyPlan {
                                    message: format!(
                                        "prerequisite '{dependency}' of '{unit}' has no completed earlier-wave result"
                                    ),
                                },
                            );
                        }
                    }
                }

                if unsuccessful_prerequisites.is_empty() {
                    ready_units.push(unit.clone());
                } else {
                    states.insert(
                        unit.clone(),
                        ScheduledUnitState::Blocked {
                            prerequisites: unsuccessful_prerequisites,
                        },
                    );
                }
            }

            if ready_units.is_empty() {
                continue;
            }

            // ------------------------------------------------------------
            // Execute only the units declared ready by dependency analysis.
            // ------------------------------------------------------------

            let execution = self.workers.execute(
                ready_units.len(),
                cancellation,
                |index, token| {
                    task(&ready_units[index], token)
                },
            )?;

            worker_threads_created = worker_threads_created
                .checked_add(execution.worker_count())
                .ok_or(SchedulerError::CounterOverflow)?;

            maximum_workers_in_wave = maximum_workers_in_wave
                .max(execution.worker_count());

            started_units = started_units
                .checked_add(execution.started_jobs())
                .ok_or(SchedulerError::CounterOverflow)?;

            // ------------------------------------------------------------
            // Preserve each worker result under its original unit identity.
            // ------------------------------------------------------------

            let worker_outcomes = execution.into_outcomes();

            if worker_outcomes.len() != ready_units.len() {
                return Err(
                    SchedulerError::InvalidDependencyPlan {
                        message:
                            "worker executor returned an unexpected outcome count"
                                .to_owned(),
                    },
                );
            }

            for (unit, outcome) in ready_units
                .into_iter()
                .zip(worker_outcomes.into_iter())
            {
                let state = match outcome {
                    WorkerTaskOutcome::Completed(value) => {
                        ScheduledUnitState::Completed(value)
                    }

                    WorkerTaskOutcome::Failed(error) => {
                        ScheduledUnitState::Failed(error)
                    }

                    WorkerTaskOutcome::Panicked { message } => {
                        ScheduledUnitState::Panicked { message }
                    }

                    WorkerTaskOutcome::Cancelled => {
                        ScheduledUnitState::Cancelled
                    }
                };

                states.insert(unit, state);
            }
        }

        // ------------------------------------------------------------
        // Give every remaining unit an explicit final state.
        // ------------------------------------------------------------

        for unit in plan.ordered_units() {
            if states.contains_key(unit) {
                continue;
            }

            let state = if cancellation.is_cancelled() {
                ScheduledUnitState::Cancelled
            } else {
                return Err(
                    SchedulerError::InvalidDependencyPlan {
                        message: format!(
                            "unit '{unit}' was not assigned a final scheduler state"
                        ),
                    },
                );
            };

            states.insert(unit.clone(), state);
        }

        // ------------------------------------------------------------
        // Collect results in the canonical dependency-first order.
        // ------------------------------------------------------------

        let mut outcomes = Vec::with_capacity(plan.unit_count());

        for unit in plan.ordered_units() {
            let state = states.remove(unit).ok_or_else(|| {
                SchedulerError::InvalidDependencyPlan {
                    message: format!(
                        "unit '{unit}' disappeared during result collection"
                    ),
                }
            })?;

            outcomes.push(ScheduledUnitOutcome {
                unit: unit.clone(),
                state,
            });
        }

        Ok(ScheduleReport {
            outcomes,
            worker_threads_created,
            maximum_workers_in_wave,
            started_units,
        })
    }
}

// ============================================================================
// DEPENDENCY PLAN VALIDATION
// ============================================================================

/// Validates the scheduler-facing invariants of a dependency plan.
///
/// Dependency discovery and topological ordering remain owned by
/// DependencyContext and DependencyPlan.
///
/// This function checks that the supplied plan is internally consistent
/// before allowing concurrent execution.
fn validate_dependency_plan(
    plan: &DependencyPlan,
) -> Result<
    BTreeMap<
        crate::compiler::context::dependency_context::CompilationUnitId,
        usize,
    >,
    SchedulerError,
> {
    use crate::compiler::context::dependency_context::CompilationUnitId;

    let ordered_units = plan.ordered_units();

    if ordered_units.len() != plan.unit_count() {
        return Err(SchedulerError::InvalidDependencyPlan {
            message:
                "ordered-unit count differs from the plan's unit count"
                    .to_owned(),
        });
    }

    let mut ordered_set = BTreeSet::<CompilationUnitId>::new();

    for unit in ordered_units {
        if !ordered_set.insert(unit.clone()) {
            return Err(SchedulerError::InvalidDependencyPlan {
                message: format!(
                    "unit '{unit}' occurs more than once in the canonical order"
                ),
            });
        }

        if !plan.context().contains_unit(unit) {
            return Err(SchedulerError::InvalidDependencyPlan {
                message: format!(
                    "unit '{unit}' is absent from the canonical dependency context"
                ),
            });
        }
    }

    let mut wave_lookup =
        BTreeMap::<CompilationUnitId, usize>::new();

    let mut previous_wave_index = None;

    for wave in plan.waves() {
        if wave.is_empty() {
            return Err(SchedulerError::InvalidDependencyPlan {
                message: format!(
                    "dependency wave {} is empty",
                    wave.index()
                ),
            });
        }

        if let Some(previous) = previous_wave_index {
            if wave.index() <= previous {
                return Err(SchedulerError::InvalidDependencyPlan {
                    message:
                        "dependency wave indices are not strictly increasing"
                            .to_owned(),
                });
            }
        }

        previous_wave_index = Some(wave.index());

        for unit in wave.units() {
            if !ordered_set.contains(unit) {
                return Err(SchedulerError::InvalidDependencyPlan {
                    message: format!(
                        "wave {} contains unknown unit '{unit}'",
                        wave.index()
                    ),
                });
            }

            if wave_lookup
                .insert(unit.clone(), wave.index())
                .is_some()
            {
                return Err(SchedulerError::InvalidDependencyPlan {
                    message: format!(
                        "unit '{unit}' occurs in multiple dependency waves"
                    ),
                });
            }
        }
    }

    if wave_lookup.len() != ordered_set.len() {
        return Err(SchedulerError::InvalidDependencyPlan {
            message:
                "dependency waves do not cover every compilation unit"
                    .to_owned(),
        });
    }

    // ------------------------------------------------------------
    // Verify dependency ordering independently of wave construction.
    // ------------------------------------------------------------

    let mut order_positions =
        BTreeMap::<CompilationUnitId, usize>::new();

    for (position, unit) in ordered_units.iter().enumerate() {
        order_positions.insert(unit.clone(), position);
    }

    for unit in ordered_units {
        let unit_wave = wave_lookup
            .get(unit)
            .copied()
            .ok_or_else(|| {
                SchedulerError::InvalidDependencyPlan {
                    message: format!(
                        "unit '{unit}' has no dependency wave"
                    ),
                }
            })?;

        let dependencies = plan
            .context()
            .ordering_dependencies_of(unit)
            .ok_or_else(|| {
                SchedulerError::InvalidDependencyPlan {
                    message: format!(
                        "unit '{unit}' has no dependency metadata"
                    ),
                }
            })?;

        for dependency in dependencies {
            let dependency_wave = wave_lookup
                .get(&dependency)
                .copied()
                .ok_or_else(|| {
                    SchedulerError::InvalidDependencyPlan {
                        message: format!(
                            "dependency '{dependency}' of '{unit}' is missing from the plan"
                        ),
                    }
                })?;

            if dependency_wave >= unit_wave {
                return Err(
                    SchedulerError::InvalidDependencyPlan {
                        message: format!(
                            "dependency '{dependency}' is not in an earlier wave than '{unit}'"
                        ),
                    },
                );
            }

            let dependency_position = order_positions
                .get(&dependency)
                .copied()
                .ok_or_else(|| {
                    SchedulerError::InvalidDependencyPlan {
                        message: format!(
                            "dependency '{dependency}' has no canonical order position"
                        ),
                    }
                })?;

            let unit_position = order_positions
                .get(unit)
                .copied()
                .ok_or_else(|| {
                    SchedulerError::InvalidDependencyPlan {
                        message: format!(
                            "unit '{unit}' has no canonical order position"
                        ),
                    }
                })?;

            if dependency_position >= unit_position {
                return Err(
                    SchedulerError::InvalidDependencyPlan {
                        message: format!(
                            "dependency '{dependency}' does not precede dependent '{unit}' in canonical order"
                        ),
                    },
                );
            }
        }
    }

    Ok(wave_lookup)
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use std::num::NonZeroUsize;

    use crate::compiler::context::dependency_context::{
        CompilationUnit,
        CompilationUnitId,
        CompilationUnitKind,
        DependencyContextBuilder,
        DependencyKind,
    };

    use crate::compiler::parallel::dependencies::DependencyPlan;

    fn build_plan(
        identifiers: &[&str],
        dependencies: &[(&str, &str)],
    ) -> DependencyPlan {
        let mut builder = DependencyContextBuilder::new();

        let mut unique = BTreeSet::new();

        for identifier in identifiers {
            assert!(unique.insert(*identifier));

            let id = CompilationUnitId::new(*identifier)
                .expect("test identifier must be valid");

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
                .expect("test unit registration must succeed");
        }

        for (dependent, dependency) in dependencies {
            let dependent = CompilationUnitId::new(*dependent)
                .expect("test dependent identifier must be valid");

            let dependency = CompilationUnitId::new(*dependency)
                .expect("test dependency identifier must be valid");

            builder
                .add_dependency(
                    dependent,
                    dependency,
                    DependencyKind::ModuleImport,
                )
                .expect("test dependency registration must succeed");
        }

        let context = builder
            .build()
            .expect("test dependency graph must be valid");

        DependencyPlan::from_context(&context)
            .expect("test dependency plan must be valid")
    }

    fn scheduler() -> ParallelScheduler {
        let config = crate::compiler::parallel::workers::WorkerPoolConfig::
            with_worker_budget(
                NonZeroUsize::new(2)
                    .expect("two is non-zero"),
            );

        ParallelScheduler::new(WorkerPool::new(config))
    }

    #[test]
    fn empty_plan_completes_without_workers() {
        let plan = build_plan(&[], &[]);
        let cancellation = CancellationToken::new();

        let report = scheduler()
            .execute(&plan, &cancellation, |_, _| {
                Ok::<(), String>(())
            })
            .expect("empty plan must succeed");

        assert!(report.is_empty());
        assert!(report.is_success());
        assert_eq!(report.worker_threads_created(), 0);
        assert_eq!(report.started_units(), 0);
    }

    #[test]
    fn independent_units_execute_and_keep_canonical_order() {
        let plan = build_plan(
            &["c", "a", "b"],
            &[],
        );

        let cancellation = CancellationToken::new();

        let report = scheduler()
            .execute(&plan, &cancellation, |unit, _| {
                Ok::<String, String>(unit.as_str().to_owned())
            })
            .expect("independent units must execute");

        let actual: Vec<String> = report
            .outcomes()
            .iter()
            .map(|result| result.unit().as_str().to_owned())
            .collect();

        assert_eq!(actual, vec!["a", "b", "c"]);
        assert_eq!(report.successful_units(), 3);
        assert_eq!(report.failed_units(), 0);
        assert_eq!(report.blocked_units(), 0);
    }

    #[test]
    fn failed_prerequisite_blocks_dependents() {
        let plan = build_plan(
            &["application", "library", "top", "independent"],
            &[
                ("application", "library"),
                ("top", "application"),
            ],
        );

        let cancellation = CancellationToken::new();

        let report = scheduler()
            .execute(&plan, &cancellation, |unit, _| {
                if unit.as_str() == "library" {
                    Err("library compilation failed".to_owned())
                } else {
                    Ok(unit.as_str().to_owned())
                }
            })
            .expect("independent branch should remain executable");

        let states: BTreeMap<String, String> = report
            .outcomes()
            .iter()
            .map(|result| {
                let state = match result.state() {
                    ScheduledUnitState::Completed(_) => "completed",
                    ScheduledUnitState::Failed(_) => "failed",
                    ScheduledUnitState::Panicked { .. } => "panicked",
                    ScheduledUnitState::Blocked { .. } => "blocked",
                    ScheduledUnitState::Cancelled => "cancelled",
                };

                (
                    result.unit().as_str().to_owned(),
                    state.to_owned(),
                )
            })
            .collect();

        assert_eq!(states["library"], "failed");
        assert_eq!(states["application"], "blocked");
        assert_eq!(states["top"], "blocked");
        assert_eq!(states["independent"], "completed");

        assert_eq!(report.failed_units(), 1);
        assert_eq!(report.blocked_units(), 2);
        assert_eq!(report.successful_units(), 1);
    }

    #[test]
    fn cancellation_before_execution_starts_no_jobs() {
        let plan = build_plan(
            &["a", "b", "c"],
            &[],
        );

        let cancellation = CancellationToken::new();
        cancellation.cancel();

        let report = scheduler()
            .execute(&plan, &cancellation, |_, _| {
                Ok::<(), String>(())
            })
            .expect("cancellation should be reported");

        assert_eq!(report.started_units(), 0);
        assert_eq!(report.cancelled_units(), 3);
        assert_eq!(report.worker_threads_created(), 0);
    }

    #[test]
    fn non_ordering_relationships_do_not_create_execution_waves() {
        use crate::compiler::context::dependency_context::{
            DependencyContextBuilder,
        };

        let mut builder = DependencyContextBuilder::new();

        for name in ["a", "b"] {
            let id = CompilationUnitId::new(name).unwrap();
            let kind = CompilationUnitKind::new("source").unwrap();

            builder
                .add_unit(
                    CompilationUnit::new(id, kind, name).unwrap(),
                )
                .unwrap();
        }

        let a = CompilationUnitId::new("a").unwrap();
        let b = CompilationUnitId::new("b").unwrap();

        builder
            .add_dependency(
                a,
                b,
                DependencyKind::DataFlow,
            )
            .unwrap();

        let context = builder.build().unwrap();
        let plan = DependencyPlan::from_context(&context).unwrap();

        assert_eq!(plan.wave_count(), 1);
        assert_eq!(plan.maximum_wave_width(), 2);
    }
}
