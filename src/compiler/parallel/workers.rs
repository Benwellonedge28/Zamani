
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Compiler — Parallel Worker Execution.
//!
//! File:
//!     src/compiler/parallel/workers.rs
//!
//! Minimum Rust:
//!     Rust 1.97
//!
//! Edition:
//!     Rust 2021
//!
//! Safety:
//!     Safe Rust only.
//!
//! # Purpose
//!
//! Provides a resource-aware, configurable worker execution mechanism for
//! independent compiler jobs.
//!
//! This module is deliberately independent of:
//!
//! - source-language semantics;
//! - AST construction;
//! - IR ownership;
//! - compiler pass ordering;
//! - task dependency resolution;
//! - hardware discovery;
//! - physical device allocation;
//! - distributed execution;
//! - runtime scheduling.
//!
//! Those responsibilities belong to their canonical repository subsystems.
//!
//! # Architecture
//!
//!     Compilation pipeline
//!            |
//!            v
//!     Dependency scheduler
//!            |
//!            v
//!     Worker execution
//!            |
//!       +----+----+
//!       |    |    |
//!       v    v    v
//!      Job  Job  Job
//!       |    |    |
//!       +----+----+
//!            |
//!            v
//!     Deterministic results
//!
//! # Scalability
//!
//! There is no universal maximum worker count.
//!
//! Worker concurrency is determined by:
//!
//! - the number of available jobs;
//! - the configured worker budget;
//! - the host's reported available parallelism when automatic mode is used;
//! - operating-system thread creation capabilities;
//! - actual memory and execution resources.
//!
//! The configured worker budget is an operational resource policy, not a
//! language-level or hardware capability limit.
//!
//! # Cancellation
//!
//! Uses the canonical classical concurrency CancellationToken.
//!
//! Cancellation is cooperative. Already-running arbitrary Rust closures
//! cannot be forcibly interrupted.
//!
//! # Determinism
//!
//! Results are returned in submission order, independently of completion
//! order.
//!
//! Deterministic collection does not imply that arbitrary job side effects
//! or floating-point operations are deterministic.
//!
//! # Panic handling
//!
//! Panics originating inside submitted jobs are caught and represented as
//! structured outcomes when the build uses panic unwinding.
//!
//! A panic-abort build cannot recover from process-aborting panics.
//!
//! # Allocation
//!
//! Storage is proportional to the number of submitted jobs.
//!
//! The module does not claim that finite host memory is unlimited.

use std::any::Any;
use std::fmt;
use std::num::NonZeroUsize;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Mutex;
use std::thread;

use crate::classical::concurrency::CancellationToken;

// ============================================================================
// Configuration
// ============================================================================

/// Configuration for compiler worker execution.
///
/// `worker_budget` is an optional operational concurrency budget.
///
/// When absent, automatic mode uses the parallelism reported by the host.
///
/// No arbitrary global worker ceiling is imposed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorkerPoolConfig {
    worker_budget: Option<NonZeroUsize>,

    /// Stop assigning new jobs after the first failed job.
    ///
    /// Jobs already executing are allowed to finish.
    fail_fast: bool,
}

impl Default for WorkerPoolConfig {
    fn default() -> Self {
        Self {
            worker_budget: None,
            fail_fast: false,
        }
    }
}

impl WorkerPoolConfig {
    /// Create automatic worker configuration.
    pub const fn automatic() -> Self {
        Self {
            worker_budget: None,
            fail_fast: false,
        }
    }

    /// Create configuration with an explicit worker budget.
    ///
    /// A zero budget is rejected.
    pub const fn with_worker_budget(
        worker_budget: NonZeroUsize,
    ) -> Self {
        Self {
            worker_budget: Some(worker_budget),
            fail_fast: false,
        }
    }

    /// Return a modified configuration with fail-fast enabled or disabled.
    pub const fn with_fail_fast(mut self, enabled: bool) -> Self {
        self.fail_fast = enabled;
        self
    }

    /// Return the configured concurrency budget.
    pub const fn worker_budget(&self) -> Option<NonZeroUsize> {
        self.worker_budget
    }

    /// Return whether fail-fast execution is enabled.
    pub const fn fail_fast(&self) -> bool {
        self.fail_fast
    }

    /// Determine the effective number of workers.
    ///
    /// The number of workers cannot exceed the number of submitted jobs.
    ///
    /// An empty job collection requires no worker threads.
    pub fn resolve_worker_count(
        &self,
        job_count: usize,
    ) -> usize {
        if job_count == 0 {
            return 0;
        }

        let available = self
            .worker_budget
            .map(NonZeroUsize::get)
            .unwrap_or_else(|| {
                thread::available_parallelism()
                    .map(NonZeroUsize::get)
                    .unwrap_or(1)
            });

        available.min(job_count)
    }
}

// ============================================================================
// Errors
// ============================================================================

/// Errors raised while establishing worker execution.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorkerPoolError {
    /// Operating-system thread creation failed.
    ThreadSpawn {
        worker_index: usize,
        message: String,
    },
}

impl fmt::Display for WorkerPoolError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ThreadSpawn {
                worker_index,
                message,
            } => write!(
                f,
                "failed to spawn compiler worker {worker_index}: {message}"
            ),
        }
    }
}

impl std::error::Error for WorkerPoolError {}

// ============================================================================
// Worker outcomes
// ============================================================================

/// Outcome of an individual worker job.
///
/// The index is implicit in the position of this outcome in the execution
/// report.
///
/// Every submitted job has exactly one outcome when execution completes.
#[derive(Debug)]
pub enum WorkerTaskOutcome<T, E> {
    /// The job completed successfully.
    Completed(T),

    /// The job returned an ordinary application error.
    Failed(E),

    /// The job panicked and its panic was caught.
    Panicked {
        message: String,
    },

    /// The job was not started because cancellation or fail-fast stopped
    /// further job assignment.
    Cancelled,
}

impl<T, E> WorkerTaskOutcome<T, E> {
    pub fn is_success(&self) -> bool {
        matches!(self, Self::Completed(_))
    }

    pub fn is_failure(&self) -> bool {
        matches!(
            self,
            Self::Failed(_) | Self::Panicked { .. }
        )
    }

    pub fn is_cancelled(&self) -> bool {
        matches!(self, Self::Cancelled)
    }

    pub fn into_result(self) -> Option<Result<T, E>> {
        match self {
            Self::Completed(value) => Some(Ok(value)),
            Self::Failed(error) => Some(Err(error)),
            Self::Panicked { .. } | Self::Cancelled => None,
        }
    }
}

// ============================================================================
// Execution report
// ============================================================================

/// Complete deterministic worker execution report.
///
/// Outcomes always retain the original submission order.
#[derive(Debug)]
pub struct WorkerExecutionReport<T, E> {
    outcomes: Vec<WorkerTaskOutcome<T, E>>,
    worker_count: usize,
    started_jobs: usize,
}

impl<T, E> WorkerExecutionReport<T, E> {
    /// Return outcomes in submission order.
    pub fn outcomes(&self) -> &[WorkerTaskOutcome<T, E>] {
        &self.outcomes
    }

    /// Consume the report and return ordered outcomes.
    pub fn into_outcomes(self) -> Vec<WorkerTaskOutcome<T, E>> {
        self.outcomes
    }

    /// Number of worker threads successfully created.
    pub const fn worker_count(&self) -> usize {
        self.worker_count
    }

    /// Number of jobs actually started.
    pub const fn started_jobs(&self) -> usize {
        self.started_jobs
    }

    /// Total submitted jobs.
    pub fn job_count(&self) -> usize {
        self.outcomes.len()
    }

    /// Number of successful jobs.
    pub fn successful_jobs(&self) -> usize {
        self.outcomes
            .iter()
            .filter(|outcome| outcome.is_success())
            .count()
    }

    /// Number of failed or panicked jobs.
    pub fn failed_jobs(&self) -> usize {
        self.outcomes
            .iter()
            .filter(|outcome| outcome.is_failure())
            .count()
    }

    /// Number of jobs not started.
    pub fn cancelled_jobs(&self) -> usize {
        self.outcomes
            .iter()
            .filter(|outcome| outcome.is_cancelled())
            .count()
    }

    pub fn is_empty(&self) -> bool {
        self.outcomes.is_empty()
    }

    /// Whether every submitted job completed successfully.
    pub fn is_success(&self) -> bool {
        !self.outcomes.is_empty()
            && self.successful_jobs() == self.outcomes.len()
    }

    /// Whether every submitted job completed or was explicitly cancelled.
    pub fn is_complete(&self) -> bool {
        self.outcomes.iter().all(|outcome| {
            !matches!(outcome, WorkerTaskOutcome::Cancelled)
        })
    }
}

// ============================================================================
// Worker pool
// ============================================================================

/// Configurable parallel worker executor.
///
/// The pool is an execution primitive. It does not own logical task identity,
/// dependency ordering, compiler pipeline state, or hardware placement.
#[derive(Clone, Copy, Debug, Default)]
pub struct WorkerPool {
    config: WorkerPoolConfig,
}

impl WorkerPool {
    /// Create a worker pool from explicit configuration.
    pub const fn new(config: WorkerPoolConfig) -> Self {
        Self { config }
    }

    /// Create an automatically configured worker pool.
    pub const fn automatic() -> Self {
        Self::new(WorkerPoolConfig::automatic())
    }

    /// Return the worker configuration.
    pub const fn config(&self) -> WorkerPoolConfig {
        self.config
    }

    /// Return the effective worker count for a job collection.
    pub fn worker_count(&self, job_count: usize) -> usize {
        self.config.resolve_worker_count(job_count)
    }

    /// Execute indexed independent jobs concurrently.
    ///
    /// # Arguments
    ///
    /// `job_count`:
    ///     Number of independent jobs.
    ///
    /// `cancellation`:
    ///     Existing canonical cancellation signal.
    ///
    /// `task`:
    ///     Thread-safe job function receiving its stable submission index
    ///     and the shared cancellation token.
    ///
    /// # Guarantees
    ///
    /// - No worker is created for an empty job collection.
    /// - No more workers than jobs are created.
    /// - Worker concurrency respects the configured budget.
    /// - Results preserve submission order.
    /// - Job panics are converted to structured outcomes.
    /// - Ordinary job errors are preserved.
    /// - Cancellation prevents additional jobs from being assigned.
    /// - Fail-fast prevents additional assignments after an error.
    /// - Already-running jobs are not forcibly interrupted.
    ///
    /// # Important
    ///
    /// Jobs must be independent or already have their dependencies resolved.
    ///
    /// Dependency-aware scheduling belongs to `scheduler.rs`, not this file.
    pub fn execute<T, E, F>(
        &self,
        job_count: usize,
        cancellation: &CancellationToken,
        task: F,
    ) -> Result<WorkerExecutionReport<T, E>, WorkerPoolError>
    where
        T: Send,
        E: Send,
        F: Fn(usize, &CancellationToken) -> Result<T, E> + Sync,
    {
        if job_count == 0 {
            return Ok(WorkerExecutionReport {
                outcomes: Vec::new(),
                worker_count: 0,
                started_jobs: 0,
            });
        }

        let worker_count = self.worker_count(job_count);

        // A cancellation request before execution must not start any job.
        if cancellation.is_cancelled() {
            let outcomes = (0..job_count)
                .map(|_| WorkerTaskOutcome::Cancelled)
                .collect();

            return Ok(WorkerExecutionReport {
                outcomes,
                worker_count: 0,
                started_jobs: 0,
            });
        }

        let next_job = AtomicUsize::new(0);
        let stop_assigning = AtomicBool::new(false);
        let started_jobs = AtomicUsize::new(0);

        let outcomes: Vec<Mutex<Option<WorkerTaskOutcome<T, E>>>> =
            (0..job_count)
                .map(|_| Mutex::new(None))
                .collect();

        let mut spawn_error = None;

        thread::scope(|scope| {
            for worker_index in 0..worker_count {
                let spawn_result = thread::Builder::new()
                    .name(format!("zamani-compiler-worker-{worker_index}"))
                    .spawn_scoped(scope, || {
                        loop {
                            // Cancellation is cooperative. Stop taking new
                            // work as soon as the signal becomes observable.
                            if cancellation.is_cancelled()
                                || stop_assigning.load(Ordering::Acquire)
                            {
                                break;
                            }

                            let index =
                                next_job.fetch_add(1, Ordering::AcqRel);

                            if index >= job_count {
                                break;
                            }

                            // Recheck after claiming an index. This prevents
                            // starting work after a concurrent cancellation
                            // request whenever that request is observable
                            // before the job begins.
                            if cancellation.is_cancelled()
                                || stop_assigning.load(Ordering::Acquire)
                            {
                                let mut slot = outcomes[index]
                                    .lock()
                                    .unwrap_or_else(|poisoned| {
                                        poisoned.into_inner()
                                    });

                                *slot = Some(
                                    WorkerTaskOutcome::Cancelled,
                                );

                                continue;
                            }

                            started_jobs.fetch_add(
                                1,
                                Ordering::Relaxed,
                            );

                            let result = catch_unwind(
                                AssertUnwindSafe(|| {
                                    task(index, cancellation)
                                }),
                            );

                            let outcome = match result {
                                Ok(Ok(value)) => {
                                    WorkerTaskOutcome::Completed(value)
                                }

                                Ok(Err(error)) => {
                                    if self.config.fail_fast {
                                        stop_assigning.store(
                                            true,
                                            Ordering::Release,
                                        );
                                    }

                                    WorkerTaskOutcome::Failed(error)
                                }

                                Err(payload) => {
                                    if self.config.fail_fast {
                                        stop_assigning.store(
                                            true,
                                            Ordering::Release,
                                        );
                                    }

                                    WorkerTaskOutcome::Panicked {
                                        message: panic_message(
                                            payload.as_ref(),
                                        ),
                                    }
                                }
                            };

                            let mut slot = outcomes[index]
                                .lock()
                                .unwrap_or_else(|poisoned| {
                                    poisoned.into_inner()
                                });

                            *slot = Some(outcome);
                        }
                    });

                if let Err(error) = spawn_result {
                    stop_assigning.store(true, Ordering::Release);

                    spawn_error = Some(
                        WorkerPoolError::ThreadSpawn {
                            worker_index,
                            message: error.to_string(),
                        },
                    );

                    break;
                }
            }
        });

        if let Some(error) = spawn_error {
            return Err(error);
        }

        let started_jobs = started_jobs.load(Ordering::Acquire);

        // A worker may stop because cancellation was requested or because
        // fail-fast stopped further assignments. Every claimed slot must
        // nevertheless receive a terminal outcome.
        let outcomes = outcomes
            .into_iter()
            .map(|slot| {
                let mut slot = slot
                    .into_inner()
                    .unwrap_or_else(|poisoned| {
                        poisoned.into_inner()
                    });

                slot.take().unwrap_or(
                    WorkerTaskOutcome::Cancelled,
                )
            })
            .collect();

        Ok(WorkerExecutionReport {
            outcomes,
            worker_count,
            started_jobs,
        })
    }
}

// ============================================================================
// Panic conversion
// ============================================================================

fn panic_message(payload: &(dyn Any + Send)) -> String {
    if let Some(message) = payload.downcast_ref::<String>() {
        return message.clone();
    }

    if let Some(message) = payload.downcast_ref::<&'static str>() {
        return (*message).to_owned();
    }

    "worker job panicked with a non-string payload".to_owned()
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use std::num::NonZeroUsize;
    use std::sync::atomic::{AtomicUsize, Ordering};

    fn budget(value: usize) -> NonZeroUsize {
        NonZeroUsize::new(value)
            .expect("test budget must be nonzero")
    }

    #[test]
    fn automatic_configuration_is_valid() {
        let config = WorkerPoolConfig::automatic();

        assert_eq!(config.worker_budget(), None);
        assert!(!config.fail_fast());
    }

    #[test]
    fn explicit_worker_budget_is_preserved() {
        let config = WorkerPoolConfig::with_worker_budget(
            budget(8),
        );

        assert_eq!(
            config.worker_budget().map(NonZeroUsize::get),
            Some(8)
        );
    }

    #[test]
    fn effective_workers_never_exceed_job_count() {
        let config = WorkerPoolConfig::with_worker_budget(
            budget(100),
        );

        assert_eq!(config.resolve_worker_count(3), 3);
    }

    #[test]
    fn empty_jobs_require_no_workers() {
        let pool = WorkerPool::automatic();
        let cancellation = CancellationToken::new();

        let report = pool
            .execute::<usize, (), _>(
                0,
                &cancellation,
                |index, _| Ok(index),
            )
            .expect("empty execution should succeed");

        assert_eq!(report.job_count(), 0);
        assert_eq!(report.worker_count(), 0);
        assert_eq!(report.started_jobs(), 0);
    }

    #[test]
    fn execution_preserves_submission_order() {
        let pool = WorkerPool::new(
            WorkerPoolConfig::with_worker_budget(budget(4)),
        );

        let cancellation = CancellationToken::new();

        let report = pool
            .execute(32, &cancellation, |index, _| {
                Ok::<usize, ()>(index)
            })
            .expect("execution should succeed");

        assert_eq!(report.successful_jobs(), 32);

        for (index, outcome) in report.outcomes().iter().enumerate() {
            match outcome {
                WorkerTaskOutcome::Completed(value) => {
                    assert_eq!(*value, index);
                }

                _ => panic!("unexpected worker outcome"),
            }
        }
    }

    #[test]
    fn ordinary_job_errors_are_preserved() {
        let pool = WorkerPool::new(
            WorkerPoolConfig::with_worker_budget(budget(2)),
        );

        let cancellation = CancellationToken::new();

        let report = pool
            .execute(4, &cancellation, |index, _| {
                if index == 2 {
                    Err("expected failure")
                } else {
                    Ok(index)
                }
            })
            .expect("worker execution should complete");

        assert_eq!(report.failed_jobs(), 1);
        assert_eq!(report.successful_jobs(), 3);

        assert!(matches!(
            &report.outcomes()[2],
            WorkerTaskOutcome::Failed("expected failure")
        ));
    }

    #[test]
    fn worker_panics_are_captured() {
        let pool = WorkerPool::new(
            WorkerPoolConfig::with_worker_budget(budget(1)),
        );

        let cancellation = CancellationToken::new();

        let report = pool
            .execute(2, &cancellation, |index, _| {
                assert_ne!(index, 0, "intentional test panic");
                Ok::<usize, ()>(index)
            })
            .expect("worker panic should be contained");

        assert_eq!(report.failed_jobs(), 1);
        assert_eq!(report.successful_jobs(), 1);

        assert!(matches!(
            &report.outcomes()[0],
            WorkerTaskOutcome::Panicked { .. }
        ));
    }

    #[test]
    fn pre_cancelled_execution_starts_no_jobs() {
        let pool = WorkerPool::automatic();
        let cancellation = CancellationToken::new();

        cancellation.cancel();

        let executed = AtomicUsize::new(0);

        let report = pool
            .execute(10, &cancellation, |index, _| {
                executed.fetch_add(1, Ordering::Relaxed);
                Ok::<usize, ()>(index)
            })
            .expect("cancelled execution should return a report");

        assert_eq!(executed.load(Ordering::Relaxed), 0);
        assert_eq!(report.started_jobs(), 0);
        assert_eq!(report.cancelled_jobs(), 10);
        assert_eq!(report.worker_count(), 0);
    }

    #[test]
    fn worker_budget_one_executes_all_jobs() {
        let pool = WorkerPool::new(
            WorkerPoolConfig::with_worker_budget(budget(1)),
        );

        let cancellation = CancellationToken::new();

        let report = pool
            .execute(12, &cancellation, |index, _| {
                Ok::<usize, ()>(index * 2)
            })
            .expect("sequential execution should succeed");

        assert_eq!(report.worker_count(), 1);
        assert_eq!(report.successful_jobs(), 12);
    }

    #[test]
    fn fail_fast_stops_new_job_assignment() {
        let pool = WorkerPool::new(
            WorkerPoolConfig::with_worker_budget(budget(1))
                .with_fail_fast(true),
        );

        let cancellation = CancellationToken::new();

        let report = pool
            .execute(100, &cancellation, |index, _| {
                if index == 0 {
                    Err("stop")
                } else {
                    Ok(index)
                }
            })
            .expect("fail-fast execution should return a report");

        assert_eq!(report.failed_jobs(), 1);
        assert_eq!(report.started_jobs(), 1);
        assert_eq!(report.cancelled_jobs(), 99);
    }
}
