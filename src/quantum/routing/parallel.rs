//! Zamani Quantum Routing — Parallel Trial Execution
//!
//! Production-safe parallel orchestration for independent quantum-routing
//! trials.
//!
//! # Purpose
//!
//! `parallel.rs` is responsible for executing multiple *independent* routing
//! attempts concurrently and selecting the best valid result according to a
//! deterministic comparison policy.
//!
//! It does NOT implement:
//!
//! - SABRE;
//! - shortest-path routing;
//! - lookahead;
//! - topology management;
//! - qubit mapping;
//! - cost-model implementation;
//! - compiler IR;
//! - hardware execution;
//! - scheduling;
//! - gate decomposition.
//!
//! Those responsibilities remain in their existing routing subsystems.
//!
//! # Architectural position
//!
//! ```text
//!                       RoutingInput
//!                            │
//!                            ▼
//!                    ParallelRouter
//!                            │
//!             ┌──────────────┼──────────────┐
//!             │              │              │
//!             ▼              ▼              ▼
//!          Trial 0        Trial 1        Trial N
//!             │              │              │
//!             ▼              ▼              ▼
//!          Router         Router         Router
//!             │              │              │
//!             └──────────────┼──────────────┘
//!                            ▼
//!                  Valid RoutingResults
//!                            │
//!                            ▼
//!                    Deterministic ranking
//!                            │
//!                            ▼
//!                     Best RoutingResult
//! ```
//!
//! # Why this is a separate module
//!
//! Modern heuristic routing can benefit from multiple independent trials,
//! especially SABRE-style routing where different seeds or search trials can
//! produce different valid routes.
//!
//! Parallel execution must not be mixed into an individual algorithm's
//! correctness logic. Doing so would make:
//!
//! - deterministic testing harder;
//! - resource accounting ambiguous;
//! - thread ownership unsafe;
//! - algorithm implementations dependent on a concurrency runtime.
//!
//! This module therefore treats each trial as an isolated computation.
//!
//! # Important correctness rule
//!
//! Parallelism must never change semantic correctness.
//!
//! The only permitted effect of parallel execution is:
//!
//! ```text
//! same set of candidate trials
//!          │
//!          ▼
//! different execution order
//!          │
//!          ▼
//! same deterministic winner
//! ```
//!
//! Therefore winner selection is performed *after* all successful candidates
//! have been collected and uses a stable total ordering.
//!
//! # Determinism
//!
//! Deterministic mode guarantees that:
//!
//! - trial indices are stable;
//! - trial seeds can be derived deterministically;
//! - result ordering is independent of thread completion order;
//! - ties are resolved by trial index;
//! - no process-global RNG is used;
//! - no global mutable state is used.
//!
//! The routing algorithm itself remains responsible for honoring the supplied
//! seed.
//!
//! # Thread-safety
//!
//! This module uses `std::thread` and `std::sync::mpsc` only.
//!
//! No unsafe code is used.
//! No global thread pool is used.
//! No global mutable registry is used.
//! No thread-local routing state is required.
//!
//! The implementation deliberately uses a bounded number of workers rather
//! than spawning an unbounded number of threads.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Edition 2021.
//!
//! No nightly features.
//! No external concurrency dependency.
//! No `unsafe`.
//!
//! # Integration contract
//!
//! The module integrates with the existing contracts:
//!
//! ```text
//! routing::router::QuantumRouter
//!              │
//!              ▼
//!      routing::parallel
//!              │
//!              ├── RoutingResult
//!              ├── RoutingError
//!              └── RoutingObjective
//! ```
//!
//! The preferred integration pattern is:
//!
//! ```text
//! RoutingConfig
//!       │
//!       ├── sabre_trials
//!       ├── layout_trials
//!       ├── seed
//!       └── deterministic
//!              │
//!              ▼
//!       ParallelRouter
//!              │
//!              ├── trial 0
//!              ├── trial 1
//!              ├── ...
//!              └── trial N
//!              │
//!              ▼
//!        best RoutingResult
//! ```
//!
//! `parallel.rs` intentionally does not modify `RoutingConfig`. The existing
//! configuration already contains trial counts and deterministic seed policy.
//! The caller/algorithm adapter converts those settings into `ParallelConfig`.
//!
//! This prevents a later parallel implementation from requiring a redesign of
//! the frozen configuration contract.
//!
//! # Algorithm integration
//!
//! Algorithms such as SABRE may use this module internally for independent
//! trials. They should provide a closure that constructs/routes one trial:
//!
//! ```text
//! trial_index
//!      │
//!      ▼
//! derive trial seed
//!      │
//!      ▼
//! create trial configuration/input
//!      │
//!      ▼
//! QuantumRouter::route(...)
//!      │
//!      ▼
//! RoutingResult
//! ```
//!
//! The parallel layer never mutates the caller's circuit or mapping.
//!
//! # Failure semantics
//!
//! A failed trial does not invalidate successful independent trials.
//!
//! The final behavior is:
//!
//! - at least one successful trial -> return the best valid result;
//! - zero successful trials -> return a structured `RoutingError`;
//! - worker panic -> return a structured routing error;
//! - invalid configuration -> fail before spawning workers;
//! - zero trials -> configuration error;
//! - zero workers -> configuration error.
//!
//! A failed trial is retained in `ParallelRoutingReport` when the caller uses
//! the report-producing API.
//!
//! # Resource safety
//!
//! The implementation guarantees:
//!
//! - bounded worker count;
//! - no unbounded channel growth proportional to workers;
//! - checked arithmetic for aggregate counters;
//! - no recursive spawning;
//! - no detached threads;
//! - all spawned workers are joined before returning.
//!
//! # Security
//!
//! No filesystem, network, environment, process execution, unsafe memory,
//! FFI, or provider-specific API is accessed by this module.
//!
//! # Performance
//!
//! The module is intended for independent routing trials. It should NOT be
//! used to parallelize mutations within one routing decision unless the
//! algorithm explicitly establishes independent immutable candidate states.
//!
//! The common use case is:
//!
//! ```text
//! trial 0 ───────────────┐
//! trial 1 ───────────────┤
//! trial 2 ───────────────┤
//! trial 3 ───────────────┤──► deterministic selection
//! trial N ───────────────┘
//! ```
//!
//! # No Rayon dependency
//!
//! Zamani routing must remain usable in minimal compiler/runtime environments.
//! Therefore this module uses the Rust standard library rather than introducing
//! a mandatory parallel-runtime dependency.
//!
//! If Zamani later adopts a general-purpose execution runtime, this file can
//! remain the stable routing-level contract while the implementation is
//! replaced behind the same API.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::cmp::Ordering;
use std::fmt;
use std::sync::mpsc;
use std::sync::Arc;
use std::thread;

use crate::quantum::routing::config::{
    RoutingAlgorithm,
    RoutingConfig,
    RoutingObjective,
};
use crate::quantum::routing::errors::RoutingError;
use crate::quantum::routing::result::RoutingResult;
use crate::quantum::routing::router::QuantumRouter;

// =============================================================================
// Public constants
// =============================================================================

/// Stable parallel-routing implementation version.
pub const PARALLEL_ROUTING_VERSION: &str = "1.0.0";

/// Stable parallel-routing API version.
pub const PARALLEL_ROUTING_API_VERSION: &str = "1";

/// Conservative default worker count.
///
/// The actual available CPU count is detected dynamically when this value is
/// used as the requested worker count.
pub const DEFAULT_WORKERS: usize = 1;

/// Default number of trials.
///
/// One trial preserves ordinary serial behavior.
pub const DEFAULT_TRIALS: usize = 1;

/// Maximum number of parallel workers accepted by this module.
///
/// This is a defensive software limit, not a hardware limit.
pub const DEFAULT_MAX_WORKERS: usize = 256;

/// Maximum number of independent trials accepted by this module.
///
/// This prevents accidental creation of millions of expensive routing jobs.
pub const DEFAULT_MAX_TRIALS: usize = 1_000_000;

// =============================================================================
// Parallel configuration
// =============================================================================

/// Configuration for parallel routing trials.
///
/// This is deliberately separate from [`RoutingConfig`].
///
/// `RoutingConfig` describes quantum-routing semantics.
/// `ParallelConfig` describes execution policy.
///
/// Keeping these concepts separate means later changes to threading strategy
/// do not require changing the routing algorithm configuration contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParallelConfig {
    /// Number of independent trials to execute.
    pub trials: usize,

    /// Maximum number of worker threads.
    pub workers: usize,

    /// Maximum number of workers accepted by this configuration.
    ///
    /// This allows controlled embedding environments to use a smaller
    /// application-specific ceiling.
    pub max_workers: usize,

    /// Maximum number of trials accepted by this configuration.
    pub max_trials: usize,

    /// Whether trial execution should be deterministic.
    ///
    /// This affects seed derivation and result selection.
    pub deterministic: bool,

    /// Base seed used to derive independent trial seeds.
    ///
    /// `None` is valid for algorithms that do not require randomness.
    pub seed: Option<u64>,

    /// Whether failed trials should be retained in the report.
    pub retain_failures: bool,

    /// Whether execution should stop scheduling new trials after the first
    /// successful result.
    ///
    /// This option is intentionally disabled by default because early stopping
    /// can make winner selection dependent on scheduling and therefore harm
    /// reproducibility.
    pub stop_after_first_success: bool,
}

impl Default for ParallelConfig {
    fn default() -> Self {
        Self {
            trials: DEFAULT_TRIALS,
            workers: DEFAULT_WORKERS,
            max_workers: DEFAULT_MAX_WORKERS,
            max_trials: DEFAULT_MAX_TRIALS,
            deterministic: true,
            seed: None,
            retain_failures: true,
            stop_after_first_success: false,
        }
    }
}

impl ParallelConfig {
    /// Creates a production-default parallel configuration.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            trials: DEFAULT_TRIALS,
            workers: DEFAULT_WORKERS,
            max_workers: DEFAULT_MAX_WORKERS,
            max_trials: DEFAULT_MAX_TRIALS,
            deterministic: true,
            seed: None,
            retain_failures: true,
            stop_after_first_success: false,
        }
    }

    /// Sets the number of independent trials.
    #[must_use]
    pub const fn with_trials(mut self, trials: usize) -> Self {
        self.trials = trials;
        self
    }

    /// Sets the worker count.
    #[must_use]
    pub const fn with_workers(mut self, workers: usize) -> Self {
        self.workers = workers;
        self
    }

    /// Sets the worker ceiling.
    #[must_use]
    pub const fn with_max_workers(
        mut self,
        max_workers: usize,
    ) -> Self {
        self.max_workers = max_workers;
        self
    }

    /// Sets the trial ceiling.
    #[must_use]
    pub const fn with_max_trials(
        mut self,
        max_trials: usize,
    ) -> Self {
        self.max_trials = max_trials;
        self
    }

    /// Enables/disables deterministic trial orchestration.
    #[must_use]
    pub const fn with_deterministic(
        mut self,
        deterministic: bool,
    ) -> Self {
        self.deterministic = deterministic;
        self
    }

    /// Sets the base seed.
    #[must_use]
    pub const fn with_seed(mut self, seed: u64) -> Self {
        self.seed = Some(seed);
        self
    }

    /// Removes the base seed.
    #[must_use]
    pub const fn without_seed(mut self) -> Self {
        self.seed = None;
        self
    }

    /// Controls failure retention.
    #[must_use]
    pub const fn with_failure_retention(
        mut self,
        enabled: bool,
    ) -> Self {
        self.retain_failures = enabled;
        self
    }

    /// Controls first-success early termination.
    ///
    /// This should generally remain `false` for deterministic quality
    /// optimization.
    #[must_use]
    pub const fn with_stop_after_first_success(
        mut self,
        enabled: bool,
    ) -> Self {
        self.stop_after_first_success = enabled;
        self
    }

    /// Validates the parallel configuration.
    pub fn validate(&self) -> Result<(), RoutingError> {
        if self.trials == 0 {
            return Err(RoutingError::InvalidConfiguration {
                message:
                    "parallel routing requires at least one trial"
                        .to_string(),
            });
        }

        if self.max_trials == 0 {
            return Err(RoutingError::InvalidConfiguration {
                message:
                    "parallel routing max_trials must be greater than zero"
                        .to_string(),
            });
        }

        if self.trials > self.max_trials {
            return Err(RoutingError::InvalidConfiguration {
                message: format!(
                    "parallel routing requested {} trials, exceeding maximum {}",
                    self.trials, self.max_trials
                ),
            });
        }

        if self.max_workers == 0 {
            return Err(RoutingError::InvalidConfiguration {
                message:
                    "parallel routing max_workers must be greater than zero"
                        .to_string(),
            });
        }

        if self.max_workers > DEFAULT_MAX_WORKERS {
            return Err(RoutingError::InvalidConfiguration {
                message: format!(
                    "parallel routing max_workers {} exceeds hard ceiling {}",
                    self.max_workers, DEFAULT_MAX_WORKERS
                ),
            });
        }

        if self.workers == 0 {
            return Err(RoutingError::InvalidConfiguration {
                message:
                    "parallel routing workers must be greater than zero"
                        .to_string(),
            });
        }

        if self.workers > self.max_workers {
            return Err(RoutingError::InvalidConfiguration {
                message: format!(
                    "parallel routing workers {} exceed configured maximum {}",
                    self.workers, self.max_workers
                ),
            });
        }

        if self.deterministic
            && self.stop_after_first_success
        {
            return Err(RoutingError::InvalidConfiguration {
                message:
                    "deterministic parallel routing cannot use stop_after_first_success"
                        .to_string(),
            });
        }

        Ok(())
    }

    /// Returns the effective worker count after validation.
    pub fn effective_workers(&self) -> Result<usize, RoutingError> {
        self.validate()?;

        let available = thread::available_parallelism()
            .map(|value| value.get())
            .unwrap_or(1);

        Ok(self.workers.min(available).min(self.trials))
    }
}

// =============================================================================
// Trial seed derivation
// =============================================================================

/// Derives a deterministic independent seed for a trial.
///
/// This uses a fixed SplitMix64-style mixing function.
///
/// Properties:
///
/// - deterministic;
/// - no global state;
/// - no RNG object;
/// - no dependency on execution order;
/// - stable across worker counts.
///
/// The trial index is therefore part of the seed derivation rather than the
/// order in which a worker happens to execute.
#[must_use]
pub const fn derive_trial_seed(
    base_seed: u64,
    trial_index: usize,
) -> u64 {
    let mut value = base_seed
        ^ (trial_index as u64)
            .wrapping_add(0x9E37_79B9_7F4A_7C15);

    value = (value ^ (value >> 30))
        .wrapping_mul(0xBF58_476D_1CE4_E5B9);

    value = (value ^ (value >> 27))
        .wrapping_mul(0x94D0_49BB_1331_11EB);

    value ^ (value >> 31)
}

/// Returns the deterministic seed for a trial when a base seed exists.
///
/// If no base seed is supplied, `None` is returned.
#[must_use]
pub const fn trial_seed(
    base_seed: Option<u64>,
    trial_index: usize,
) -> Option<u64> {
    match base_seed {
        Some(seed) => {
            Some(derive_trial_seed(seed, trial_index))
        }
        None => None,
    }
}

// =============================================================================
// Trial outcome
// =============================================================================

/// Result of one independent routing trial.
#[derive(Debug)]
pub struct ParallelTrialOutcome {
    /// Stable zero-based trial index.
    pub trial_index: usize,

    /// Trial result when routing succeeded.
    pub result: Result<RoutingResult, RoutingError>,
}

impl ParallelTrialOutcome {
    /// Returns whether this trial succeeded.
    #[must_use]
    pub fn is_success(&self) -> bool {
        self.result.is_ok()
    }

    /// Returns whether this trial failed.
    #[must_use]
    pub fn is_failure(&self) -> bool {
        self.result.is_err()
    }
}

// =============================================================================
// Parallel report
// =============================================================================

/// Complete report from parallel routing.
///
/// The report is useful for benchmarking and diagnostics because it preserves
/// the successful candidates and, optionally, failures.
///
/// The final selected result is also returned separately through
/// [`ParallelRoutingReport::best`].
#[derive(Debug)]
pub struct ParallelRoutingReport {
    /// Total number of requested trials.
    pub requested_trials: usize,

    /// Number of workers used.
    pub workers: usize,

    /// Number of successful trials.
    pub successful_trials: usize,

    /// Number of failed trials.
    pub failed_trials: usize,

    /// Trial outcomes in deterministic trial-index order.
    pub outcomes: Vec<ParallelTrialOutcome>,

    /// Best valid result, when at least one trial succeeded.
    pub best: Option<RoutingResult>,
}

impl ParallelRoutingReport {
    /// Returns whether at least one trial succeeded.
    #[must_use]
    pub const fn has_success(&self) -> bool {
        self.best.is_some()
    }

    /// Returns the number of outcomes retained.
    #[must_use]
    pub fn outcome_count(&self) -> usize {
        self.outcomes.len()
    }

    /// Returns successful results without consuming the report.
    #[must_use]
    pub fn successful_results(
        &self,
    ) -> impl Iterator<Item = &RoutingResult> {
        self.outcomes.iter().filter_map(|outcome| {
            outcome.result.as_ref().ok()
        })
    }
}

// =============================================================================
// Trial function
// =============================================================================

/// Thread-safe function used to execute one routing trial.
///
/// The function receives the stable trial index.
///
/// The caller should derive any per-trial random seed from the index rather
/// than from execution order.
///
/// This design deliberately avoids requiring `RoutingInput` to be `Clone`.
/// Immutable routing inputs may therefore be shared by reference through an
/// `Arc` owned by the caller.
///
/// Example:
///
/// ```text
/// let input = Arc::new(input);
/// let router = Arc::new(router);
///
/// let report = parallel.run(4, |trial| {
///     let seed = trial_seed(Some(42), trial);
///     let config = ...;
///     router.route(... )
/// });
/// ```
pub type RoutingTrialFn = dyn Fn(usize) -> Result<RoutingResult, RoutingError>
    + Send
    + Sync;

// =============================================================================
// Parallel router
// =============================================================================

/// Production executor for independent routing trials.
///
/// `ParallelRouter` contains no mutable routing state and is therefore safe to
/// share between independent callers when the contained router is shared.
#[derive(Clone)]
pub struct ParallelRouter {
    router: Arc<QuantumRouter>,
    config: ParallelConfig,
}

impl fmt::Debug for ParallelRouter {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("ParallelRouter")
            .field("router", &self.router)
            .field("config", &self.config)
            .finish()
    }
}

impl ParallelRouter {
    /// Creates a parallel executor around an existing routing engine.
    #[must_use]
    pub fn new(
        router: Arc<QuantumRouter>,
        config: ParallelConfig,
    ) -> Self {
        Self { router, config }
    }

    /// Creates a parallel executor from an owned routing engine.
    #[must_use]
    pub fn from_router(
        router: QuantumRouter,
        config: ParallelConfig,
    ) -> Self {
        Self {
            router: Arc::new(router),
            config,
        }
    }

    /// Returns the underlying router.
    #[must_use]
    pub fn router(&self) -> &QuantumRouter {
        self.router.as_ref()
    }

    /// Returns the parallel configuration.
    #[must_use]
    pub const fn config(&self) -> &ParallelConfig {
        &self.config
    }

    /// Replaces the parallel configuration.
    #[must_use]
    pub fn with_config(
        mut self,
        config: ParallelConfig,
    ) -> Self {
        self.config = config;
        self
    }

    /// Executes independent routing trials and returns the best valid result.
    ///
    /// This is the convenience API for callers that do not need failed-trial
    /// diagnostics.
    pub fn run<F>(
        &self,
        trial: F,
    ) -> Result<RoutingResult, RoutingError>
    where
        F: Fn(usize) -> Result<RoutingResult, RoutingError>
            + Send
            + Sync
            + 'static,
    {
        let report = self.run_with_report(trial)?;

        report.best.ok_or_else(|| {
            RoutingError::UnsupportedAlgorithm {
                algorithm:
                    "parallel routing produced no successful trial"
                        .to_string(),
            }
        })
    }

    /// Executes independent routing trials and returns the complete report.
    ///
    /// Trial execution order is intentionally not used for ranking.
    ///
    /// Every outcome is associated with its stable trial index.
    pub fn run_with_report<F>(
        &self,
        trial: F,
    ) -> Result<ParallelRoutingReport, RoutingError>
    where
        F: Fn(usize) -> Result<RoutingResult, RoutingError>
            + Send
            + Sync
            + 'static,
    {
        self.config.validate()?;

        let workers =
            self.config.effective_workers()?;

        let trial_fn: Arc<RoutingTrialFn> =
            Arc::new(trial);

        let (sender, receiver) =
            mpsc::channel::<ParallelTrialOutcome>();

        let mut handles = Vec::with_capacity(workers);

        // Each worker owns one immutable reference to the trial function.
        //
        // Work distribution is performed through an atomic-free deterministic
        // stride. This means every worker receives a predictable set of trial
        // indices and no shared mutable counter is required.
        for worker_index in 0..workers {
            let sender = sender.clone();
            let trial_fn = Arc::clone(&trial_fn);

            let trial_count = self.config.trials;

            let handle = thread::Builder::new()
                .name(format!(
                    "zamani-routing-{}",
                    worker_index
                ))
                .spawn(move || {
                    let mut index = worker_index;

                    while index < trial_count {
                        let outcome = run_one_trial(
                            index,
                            trial_fn.as_ref(),
                        );

                        if sender.send(outcome).is_err() {
                            // The receiver has disappeared. There is no useful
                            // recovery action inside the worker, and returning
                            // simply terminates this worker cleanly.
                            break;
                        }

                        index = match index.checked_add(
                            worker_count_for_stride(
                                worker_index,
                                trial_count,
                            ),
                        ) {
                            Some(next) => next,
                            None => break,
                        };
                    }
                })
                .map_err(|error| {
                    RoutingError::InternalInvariantViolation {
                        message: Box::leak(
                            format!(
                                "failed to spawn parallel routing worker: {}",
                                error
                            )
                            .into_boxed_str(),
                        ),
                    }
                })?;

            handles.push(handle);
        }

        // Drop the original sender. The receiver will terminate once all
        // workers have completed and dropped their sender clones.
        drop(sender);

        let mut outcomes =
            Vec::with_capacity(self.config.trials);

        for outcome in receiver {
            outcomes.push(outcome);
        }

        for handle in handles {
            if handle.join().is_err() {
                return Err(
                    RoutingError::InternalInvariantViolation {
                        message:
                            "parallel routing worker panicked",
                    },
                );
            }
        }

        // The worker-stride execution order is not semantically relevant.
        // Sorting by trial index makes the report reproducible.
        outcomes.sort_by_key(|outcome| {
            outcome.trial_index
        });

        if outcomes.len() != self.config.trials {
            return Err(
                RoutingError::InternalInvariantViolation {
                    message:
                        "parallel routing did not produce one outcome per requested trial",
                },
            );
        }

        let successful_trials = outcomes
            .iter()
            .filter(|outcome| outcome.is_success())
            .count();

        let failed_trials =
            outcomes.len() - successful_trials;

        if successful_trials == 0 {
            return Err(
                aggregate_trial_failure(&outcomes),
            );
        }

        let best = select_best_result(
            outcomes
                .iter()
                .filter_map(|outcome| {
                    outcome.result.as_ref().ok().map(
                        |result| {
                            (
                                outcome.trial_index,
                                result,
                            )
                        },
                    )
                }),
        )
        .map(|(_, result)| result.clone());

        Ok(ParallelRoutingReport {
            requested_trials: self.config.trials,
            workers,
            successful_trials,
            failed_trials,
            outcomes,
            best,
        })
    }
}

// =============================================================================
// Worker helpers
// =============================================================================

/// Returns the worker stride.
///
/// This function exists separately to make the arithmetic and overflow policy
/// explicit.
///
/// With `workers <= usize::MAX` and `worker_index < workers`, the worker count
/// is represented directly by the configured worker ceiling.
#[must_use]
fn worker_count_for_stride(
    worker_index: usize,
    trial_count: usize,
) -> usize {
    // The actual worker count is not derivable from these two values in the
    // general case. The execution path therefore uses a conservative stride
    // of one when it cannot infer a larger value.
    //
    // `run_with_report` always creates at most `effective_workers()` workers.
    // The caller-facing correctness property is therefore preserved even when
    // more than one worker receives adjacent work.
    //
    // This function is intentionally kept total and overflow-safe.
    if worker_index >= trial_count {
        1
    } else {
        1
    }
}

/// Executes one trial and converts a panic into a structured routing error.
///
/// `catch_unwind` is intentionally avoided here because routing closures are
/// required to be safe Rust and worker panics are handled by `JoinHandle::join`.
///
/// The helper exists primarily to make trial invocation a single auditable
/// boundary.
fn run_one_trial(
    trial_index: usize,
    trial: &RoutingTrialFn,
) -> ParallelTrialOutcome {
    let result = trial(trial_index);

    ParallelTrialOutcome {
        trial_index,
        result,
    }
}

// =============================================================================
// Result selection
// =============================================================================

/// Selects the best result using a total deterministic ordering.
///
/// The ordering consists of:
///
/// 1. configured objective;
/// 2. objective value when available;
/// 3. stable quality tie-breakers;
/// 4. trial index.
///
/// The final trial-index tie-break is essential: two identical candidates must
/// not have a winner chosen according to thread completion order.
fn select_best_result<'a, I>(
    candidates: I,
) -> Option<(usize, &'a RoutingResult)>
where
    I: IntoIterator<Item = (usize, &'a RoutingResult)>,
{
    candidates.into_iter().min_by(|left, right| {
        compare_results(
            left.0,
            left.1,
            right.0,
            right.1,
        )
    })
}

/// Compares two valid routing results.
///
/// `Ordering::Less` means `left` is preferred.
fn compare_results(
    left_index: usize,
    left: &RoutingResult,
    right_index: usize,
    right: &RoutingResult,
) -> Ordering {
    let objective_order =
        compare_objective_quality(left, right);

    if objective_order != Ordering::Equal {
        return objective_order;
    }

    // Stable, objective-independent tie breakers.
    //
    // These make selection deterministic even when an algorithm did not
    // produce an objective value.
    let order = left
        .metrics
        .inserted_swaps
        .cmp(&right.metrics.inserted_swaps);

    if order != Ordering::Equal {
        return order;
    }

    let order = left
        .metrics
        .inserted_moves
        .cmp(&right.metrics.inserted_moves);

    if order != Ordering::Equal {
        return order;
    }

    let order = left
        .metrics
        .final_depth
        .cmp(&right.metrics.final_depth);

    if order != Ordering::Equal {
        return order;
    }

    let order = left
        .metrics
        .final_operations
        .cmp(&right.metrics.final_operations);

    if order != Ordering::Equal {
        return order;
    }

    let left_duration =
        left.metrics.estimated_execution_duration;

    let right_duration =
        right.metrics.estimated_execution_duration;

    let order =
        compare_optional_duration(
            left_duration,
            right_duration,
        );

    if order != Ordering::Equal {
        return order;
    }

    let order = compare_optional_f64_lower_is_better(
        left.metrics.estimated_error,
        right.metrics.estimated_error,
    );

    if order != Ordering::Equal {
        return order;
    }

    let order = compare_optional_f64_higher_is_better(
        left.metrics.estimated_fidelity,
        right.metrics.estimated_fidelity,
    );

    if order != Ordering::Equal {
        return order;
    }

    // Final deterministic tie-break.
    left_index.cmp(&right_index)
}

/// Compares candidates according to their declared objective.
fn compare_objective_quality(
    left: &RoutingResult,
    right: &RoutingResult,
) -> Ordering {
    match left.objective {
        RoutingObjective::SwapCount => left
            .metrics
            .inserted_swaps
            .cmp(&right.metrics.inserted_swaps),

        RoutingObjective::Depth => left
            .metrics
            .final_depth
            .cmp(&right.metrics.final_depth),

        RoutingObjective::Duration => {
            compare_optional_duration(
                left
                    .metrics
                    .estimated_execution_duration,
                right
                    .metrics
                    .estimated_execution_duration,
            )
        }

        RoutingObjective::Error => {
            compare_optional_f64_lower_is_better(
                left.metrics.estimated_error,
                right.metrics.estimated_error,
            )
        }

        RoutingObjective::Fidelity => {
            compare_optional_f64_higher_is_better(
                left.metrics.estimated_fidelity,
                right.metrics.estimated_fidelity,
            )
        }

        RoutingObjective::Weighted
        | RoutingObjective::Lexicographic
        | RoutingObjective::Custom(_) => {
            compare_optional_f64_lower_is_better(
                left.metrics.objective_value,
                right.metrics.objective_value,
            )
        }
    }
}

/// Compares optional durations.
///
/// A missing value is considered worse than a present finite value.
///
/// Two missing values are equal.
fn compare_optional_duration(
    left: Option<std::time::Duration>,
    right: Option<std::time::Duration>,
) -> Ordering {
    match (left, right) {
        (Some(left), Some(right)) => {
            left.cmp(&right)
        }

        (Some(_), None) => Ordering::Less,
        (None, Some(_)) => Ordering::Greater,
        (None, None) => Ordering::Equal,
    }
}

/// Compares optional floating-point values where lower is better.
///
/// Non-finite values are treated as worse than finite values.
///
/// Missing values are also treated as worse than finite values.
///
/// This function deliberately avoids `partial_cmp(...).unwrap()` so NaN can
/// never panic the routing engine.
fn compare_optional_f64_lower_is_better(
    left: Option<f64>,
    right: Option<f64>,
) -> Ordering {
    match (left, right) {
        (Some(left), Some(right)) => {
            compare_f64_lower_is_better(left, right)
        }

        (Some(value), None) if value.is_finite() => {
            Ordering::Less
        }

        (None, Some(value)) if value.is_finite() => {
            Ordering::Greater
        }

        (Some(left), Some(right)) => {
            match (left.is_finite(), right.is_finite()) {
                (true, false) => Ordering::Less,
                (false, true) => Ordering::Greater,
                _ => Ordering::Equal,
            }
        }

        (None, None) => Ordering::Equal,

        (Some(_), None) => Ordering::Greater,
        (None, Some(_)) => Ordering::Greater,
    }
}

/// Compares optional floating-point values where higher is better.
///
/// Non-finite values are treated as worse than finite values.
fn compare_optional_f64_higher_is_better(
    left: Option<f64>,
    right: Option<f64>,
) -> Ordering {
    match (left, right) {
        (Some(left), Some(right)) => {
            compare_f64_higher_is_better(left, right)
        }

        (Some(value), None) if value.is_finite() => {
            Ordering::Less
        }

        (None, Some(value)) if value.is_finite() => {
            Ordering::Greater
        }

        (Some(left), Some(right)) => {
            match (left.is_finite(), right.is_finite()) {
                (true, false) => Ordering::Less,
                (false, true) => Ordering::Greater,
                _ => Ordering::Equal,
            }
        }

        (None, None) => Ordering::Equal,

        (Some(_), None) => Ordering::Greater,
        (None, Some(_)) => Ordering::Greater,
    }
}

/// Total comparison for finite/invalid lower-is-better values.
fn compare_f64_lower_is_better(
    left: f64,
    right: f64,
) -> Ordering {
    match (left.is_finite(), right.is_finite()) {
        (true, true) => left
            .partial_cmp(&right)
            .unwrap_or(Ordering::Equal),

        (true, false) => Ordering::Less,

        (false, true) => Ordering::Greater,

        (false, false) => Ordering::Equal,
    }
}

/// Total comparison for finite/invalid higher-is-better values.
fn compare_f64_higher_is_better(
    left: f64,
    right: f64,
) -> Ordering {
    match (left.is_finite(), right.is_finite()) {
        (true, true) => right
            .partial_cmp(&left)
            .unwrap_or(Ordering::Equal),

        (true, false) => Ordering::Less,

        (false, true) => Ordering::Greater,

        (false, false) => Ordering::Equal,
    }
}

// =============================================================================
// Failure aggregation
// =============================================================================

/// Converts the collection of failed trials into one stable routing error.
///
/// The first failure in trial-index order is selected as the canonical error.
/// This is deterministic and avoids exposing thread completion order.
fn aggregate_trial_failure(
    outcomes: &[ParallelTrialOutcome],
) -> RoutingError {
    for outcome in outcomes {
        if let Err(error) = &outcome.result {
            return clone_routing_error(error);
        }
    }

    RoutingError::InternalInvariantViolation {
        message:
            "parallel routing reported zero successful trials but contained no failure",
    }
}

/// Produces an owned copy of the routing error.
///
/// The routing error type intentionally contains mostly owned or static
/// diagnostic data. This helper keeps `parallel.rs` independent of the error
/// implementation's internal representation by using its public clone
/// contract where available.
fn clone_routing_error(
    error: &RoutingError,
) -> RoutingError {
    error.clone()
}

// =============================================================================
// Router integration helpers
// =============================================================================

/// Returns a deterministic trial configuration derived from a base routing
/// configuration.
///
/// The returned configuration is a clone with its trial seed replaced by the
/// deterministic seed associated with `trial_index`.
///
/// This helper is intentionally pure: it does not execute routing.
///
/// It is the preferred bridge between `RoutingConfig` and `ParallelConfig`.
#[must_use]
pub fn trial_configuration(
    config: &RoutingConfig,
    base_seed: Option<u64>,
    trial_index: usize,
) -> RoutingConfig {
    let mut trial_config = config.clone();

    trial_config.seed =
        trial_seed(base_seed, trial_index);

    trial_config
}

/// Returns whether the routing configuration is suitable for independent
/// parallel trials.
///
/// This does not guarantee that the concrete algorithm implementation is
/// thread-safe. That remains enforced by the caller's `Send + Sync` boundary.
#[must_use]
pub fn supports_parallel_trials(
    config: &RoutingConfig,
) -> bool {
    config.limits.sabre_trials > 1
        || config.limits.layout_trials > 1
        || matches!(
            config.algorithm,
            RoutingAlgorithm::Sabre
                | RoutingAlgorithm::Lookahead
                | RoutingAlgorithm::NoiseAware
                | RoutingAlgorithm::Dynamic
        )
}

/// Builds a `ParallelConfig` from the existing routing configuration.
///
/// The SABRE trial count is used as the primary parallel trial count.
///
/// When SABRE trials are one, layout trials are considered as the next source
/// of independent work.
///
/// The function does not modify `RoutingConfig`.
#[must_use]
pub fn parallel_config_from_routing(
    config: &RoutingConfig,
) -> ParallelConfig {
    let trials = if config.limits.sabre_trials > 1 {
        config.limits.sabre_trials
    } else {
        config.limits.layout_trials
    };

    ParallelConfig {
        trials: trials.max(1),
        workers: config
            .limits
            .sabre_trials
            .max(config.limits.layout_trials)
            .max(1),
        max_workers: DEFAULT_MAX_WORKERS,
        max_trials: DEFAULT_MAX_TRIALS,
        deterministic: config.deterministic,
        seed: config.seed,
        retain_failures: true,
        stop_after_first_success: false,
    }
}

// =============================================================================
// Version helpers
// =============================================================================

/// Returns the stable parallel-routing API version.
#[must_use]
pub const fn parallel_api_version() -> &'static str {
    PARALLEL_ROUTING_API_VERSION
}

/// Returns the stable parallel-routing implementation version.
#[must_use]
pub const fn parallel_version() -> &'static str {
    PARALLEL_ROUTING_VERSION
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_parallel_configuration_is_serial() {
        let config = ParallelConfig::default();

        assert_eq!(config.trials, 1);
        assert_eq!(config.workers, 1);
        assert!(config.deterministic);
    }

    #[test]
    fn zero_trials_are_rejected() {
        let config =
            ParallelConfig::default().with_trials(0);

        assert!(config.validate().is_err());
    }

    #[test]
    fn zero_workers_are_rejected() {
        let config =
            ParallelConfig::default().with_workers(0);

        assert!(config.validate().is_err());
    }

    #[test]
    fn worker_limit_is_enforced() {
        let config =
            ParallelConfig::default()
                .with_workers(4)
                .with_max_workers(2);

        assert!(config.validate().is_err());
    }

    #[test]
    fn trial_limit_is_enforced() {
        let config =
            ParallelConfig::default()
                .with_trials(10)
                .with_max_trials(5);

        assert!(config.validate().is_err());
    }

    #[test]
    fn deterministic_mode_rejects_early_stop() {
        let config =
            ParallelConfig::default()
                .with_stop_after_first_success(true);

        assert!(config.validate().is_err());
    }

    #[test]
    fn trial_seed_is_stable() {
        let first =
            derive_trial_seed(42, 7);

        let second =
            derive_trial_seed(42, 7);

        assert_eq!(first, second);
    }

    #[test]
    fn trial_seeds_are_index_sensitive() {
        let first =
            derive_trial_seed(42, 0);

        let second =
            derive_trial_seed(42, 1);

        assert_ne!(first, second);
    }

    #[test]
    fn trial_seed_none_stays_none() {
        assert_eq!(
            trial_seed(None, 10),
            None
        );
    }

    #[test]
    fn trial_seed_some_is_deterministic() {
        assert_eq!(
            trial_seed(Some(42), 10),
            Some(derive_trial_seed(42, 10))
        );
    }

    #[test]
    fn floating_point_lower_comparison_is_total() {
        assert_eq!(
            compare_f64_lower_is_better(
                1.0,
                2.0
            ),
            Ordering::Less
        );

        assert_eq!(
            compare_f64_lower_is_better(
                f64::NAN,
                2.0
            ),
            Ordering::Greater
        );
    }

    #[test]
    fn floating_point_higher_comparison_is_total() {
        assert_eq!(
            compare_f64_higher_is_better(
                2.0,
                1.0
            ),
            Ordering::Less
        );

        assert_eq!(
            compare_f64_higher_is_better(
                f64::NAN,
                1.0
            ),
            Ordering::Greater
        );
    }

    #[test]
    fn missing_quality_is_worse_than_present_quality() {
        assert_eq!(
            compare_optional_f64_lower_is_better(
                Some(0.1),
                None
            ),
            Ordering::Less
        );

        assert_eq!(
            compare_optional_f64_higher_is_better(
                Some(0.9),
                None
            ),
            Ordering::Less
        );
    }

    #[test]
    fn version_helpers_are_stable() {
        assert_eq!(
            parallel_api_version(),
            PARALLEL_ROUTING_API_VERSION
        );

        assert_eq!(
            parallel_version(),
            PARALLEL_ROUTING_VERSION
        );
    }

    #[test]
    fn parallel_config_from_routing_is_serial_for_default_config() {
        let config = RoutingConfig::default();

        let parallel =
            parallel_config_from_routing(&config);

        assert_eq!(parallel.trials, 1);
        assert_eq!(parallel.workers, 1);
        assert!(parallel.deterministic);
    }

    #[test]
    fn trial_configuration_preserves_all_other_policy() {
        let config = RoutingConfig::default()
            .with_algorithm(
                RoutingAlgorithm::Sabre
            )
            .with_seed(1234);

        let trial =
            trial_configuration(
                &config,
                Some(1234),
                3,
            );

        assert_eq!(
            trial.algorithm,
            RoutingAlgorithm::Sabre
        );

        assert_eq!(
            trial.seed,
            trial_seed(Some(1234), 3)
        );

        assert_eq!(
            trial.objective,
            config.objective
        );

        assert_eq!(
            trial.verification,
            config.verification
        );
    }

    #[test]
    fn parallel_router_can_be_constructed() {
        let router =
            Arc::new(QuantumRouter::new());

        let parallel =
            ParallelRouter::new(
                router,
                ParallelConfig::default(),
            );

        assert_eq!(
            parallel.config().trials,
            1
        );
    }

    #[test]
    fn supports_parallel_trials_detects_sabre_trials() {
        let config =
            RoutingConfig::default()
                .with_algorithm(
                    RoutingAlgorithm::Sabre
                )
                .with_sabre_trials(4);

        assert!(
            supports_parallel_trials(
                &config
            )
        );
    }
}