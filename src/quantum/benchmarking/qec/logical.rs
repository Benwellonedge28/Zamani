//! Zamani Quantum Benchmarking — Logical Error Benchmarking.
//!
//! Production-grade statistical benchmarking of logical quantum-error
//! correction outcomes.
//!
//! # Purpose
//!
//! This module measures the logical performance of a quantum error-correction
//! system from validated logical outcomes produced by the canonical QEC
//! subsystem.
//!
//! It answers questions such as:
//!
//! - What is the observed logical error rate?
//! - What is the logical success rate?
//! - What is the statistical uncertainty?
//! - How many trials were actually classifiable?
//! - How many outcomes were unknown?
//! - What happened separately for logical X/Y/Z failures?
//! - How does logical error rate change with code distance?
//! - What physical-error rate was associated with the experiment?
//! - Can two logical-error measurements be compared safely?
//! - Is the experiment statistically sufficient for a requested target?
//!
//! # Architectural ownership
//!
//! This module OWNS:
//!
//! - logical-error benchmark configuration;
//! - trial aggregation;
//! - logical X/Y/Z failure counting;
//! - unknown-outcome handling;
//! - logical success/error rates;
//! - Wilson confidence intervals;
//! - benchmark-level statistical validity;
//! - logical-error benchmark result objects;
//! - deterministic result summaries;
//! - code-distance sweep aggregation;
//! - logical-error suppression calculations;
//! - logical-error benchmark comparison helpers.
//!
//! This module DOES NOT own:
//!
//! - stabilizer algebra;
//! - logical-equivalence mathematics;
//! - decoder algorithms;
//! - MWPM;
//! - Union-Find;
//! - syndrome extraction;
//! - surface-code construction;
//! - physical noise generation;
//! - QPU execution;
//! - simulator execution;
//! - resource allocation;
//! - capability authorization;
//! - telemetry transport;
//! - persistence;
//! - report serialization.
//!
//! Those responsibilities remain in the canonical QEC subsystem and the
//! surrounding benchmarking architecture.
//!
//! # Dependency direction
//!
//! The intended production dependency graph is:
//!
//! ```text
//! quantum::error_correction
//!         │
//!         ├── logical::LogicalOutcome
//!         │
//!         └── decoder_result::DecodeResult
//!                    │
//!                    ▼
//!      benchmarking::qec::logical
//!                    │
//!          ┌─────────┼─────────┐
//!          ▼         ▼         ▼
//!      statistics  metrics   result
//!          │         │         │
//!          └─────────┼─────────┘
//!                    ▼
//!             BenchmarkResult
//! ```
//!
//! In the completed benchmarking architecture:
//!
//! ```text
//! QEC execution
//!      │
//!      ▼
//! DecodeResult
//!      │
//!      ▼
//! Logical equivalence analysis
//!      │
//!      ▼
//! LogicalOutcome
//!      │
//!      ▼
//! this module
//!      │
//!      ├── logical error rate
//!      ├── logical success rate
//!      ├── confidence interval
//!      ├── X/Y/Z decomposition
//!      ├── unknown accounting
//!      └── distance-sweep analysis
//!      │
//!      ▼
//! universal BenchmarkResult
//! ```
//!
//! # Critical correctness rule
//!
//! A decoder correction is NOT automatically a successful logical correction.
//!
//! The canonical QEC architecture explicitly separates:
//!
//! ```text
//! physical error
//!       +
//! decoder correction
//!       │
//!       ▼
//! residual error
//!       │
//!       ▼
//! logical-equivalence analysis
//!       │
//!       ▼
//! LogicalOutcome
//! ```
//!
//! This benchmark therefore accepts `LogicalOutcome` rather than attempting to
//! infer logical correctness from a decoder correction alone.
//!
//! # Statistical definition
//!
//! For `N` classifiable trials and `E` logical failures:
//!
//! ```text
//! logical_error_rate = E / N
//!
//! logical_success_rate = 1 - logical_error_rate
//! ```
//!
//! Logical X/Y/Z error rates use the same denominator unless an explicit
//! alternative analysis is introduced later.
//!
//! Confidence intervals use the Wilson score interval for a binomial
//! proportion. The interval is computed without an external numerical
//! dependency so this module remains independently buildable.
//!
//! # Unknown outcomes
//!
//! Production QEC experiments can legitimately produce outcomes for which
//! logical classification is unavailable. Silently treating those outcomes as
//! success would bias the result.
//!
//! Therefore the caller must explicitly choose one of:
//!
//! - `Exclude` — exclude unknown outcomes from the binomial denominator but
//!   report them explicitly;
//! - `CountAsFailure` — conservatively count unknown outcomes as failures;
//! - `Reject` — fail the benchmark if any unknown outcome occurs.
//!
//! The default is `Reject` because a production benchmark should fail closed
//! rather than silently change the scientific meaning of an experiment.
//!
//! # Code-distance sweeps
//!
//! Logical-error benchmarking commonly studies:
//!
//! ```text
//! distance 3
//! distance 5
//! distance 7
//! ...
//! ```
//!
//! `LogicalBenchmarkSweep` provides a deterministic container for those
//! measurements and exposes suppression/improvement helpers.
//!
//! It does NOT claim to estimate a QEC threshold. Threshold estimation belongs
//! in `benchmarking::qec::threshold`.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021
//!
//! No nightly features are required.
//!
//! # Integration contract
//!
//! This file intentionally depends only on:
//!
//! - the standard library;
//! - the canonical QEC `LogicalOutcome` type.
//!
//! It does not depend on future benchmarking files.
//!
//! Future modules can consume it as follows:
//!
//! ```text
//! benchmarking::qec::logical
//!         │
//!         ├── LogicalBenchmarkConfig
//!         ├── LogicalTrial
//!         ├── LogicalBenchmark
//!         └── LogicalBenchmarkResult
//!                 │
//!                 ▼
//! benchmarking::core::result::BenchmarkResult
//! ```
//!
//! The future universal result layer should wrap this result instead of
//! changing the semantics of this module.
//!
//! # Security and resource safety
//!
//! This module:
//!
//! - performs no unsafe operations;
//! - allocates only bounded collections supplied by the caller;
//! - never executes quantum hardware;
//! - never accepts credentials;
//! - never stores backend secrets;
//! - never performs network access;
//! - never prints diagnostics;
//! - never uses global mutable state;
//! - never silently discards invalid observations.
//!
//! `LogicalBenchmark::with_capacity` allows callers to establish an explicit
//! capacity before accepting observations.
//!
//! # Scientific interpretation
//!
//! A logical-error rate is an empirical result under a particular:
//!
//! - code;
//! - code distance;
//! - decoder;
//! - physical-noise model;
//! - circuit schedule;
//! - syndrome-extraction procedure;
//! - number of correction rounds;
//! - measurement procedure;
//! - backend calibration;
//! - compiler configuration;
//! - sampling procedure.
//!
//! This module therefore records experiment metadata supplied by the caller,
//! but deliberately does not pretend that logical error rate is a universal
//! property independent of experimental conditions.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use core::fmt;

use crate::quantum::error_correction::logical::LogicalOutcome;

// ============================================================================
// Public constants
// ============================================================================

/// Stable benchmark identifier.
pub const LOGICAL_ERROR_BENCHMARK_ID: &str =
    "qec.logical_error_rate";

/// Semantic schema version for this benchmark result.
pub const LOGICAL_ERROR_RESULT_SCHEMA_VERSION: u32 = 1;

/// Default two-sided confidence level.
///
/// This is the normal probability corresponding approximately to two standard
/// deviations:
///
/// `Phi(2) - Phi(-2) ≈ 0.9544997361`.
pub const DEFAULT_CONFIDENCE_LEVEL: f64 =
    0.954_499_736_103_641_6;

/// Minimum supported confidence level.
///
/// Lower confidence levels are statistically possible but are not useful as a
/// production default and make benchmark comparisons difficult to interpret.
pub const MIN_CONFIDENCE_LEVEL: f64 = 0.5;

/// Maximum supported confidence level.
///
/// This avoids numerical instability from confidence levels arbitrarily close
/// to one.
pub const MAX_CONFIDENCE_LEVEL: f64 =
    0.999_999_999_999;

/// Probability lower bound.
pub const MIN_PROBABILITY: f64 = 0.0;

/// Probability upper bound.
pub const MAX_PROBABILITY: f64 = 1.0;

/// Numerical tolerance used for values expected to lie in [0, 1].
const UNIT_INTERVAL_EPSILON: f64 = 1.0e-15;

/// Maximum finite `usize` capacity that this benchmark will accept by default.
///
/// This is intentionally conservative. Large experiments should be streamed
/// into an aggregator rather than materialized as one vector of trials.
pub const DEFAULT_MAX_MATERIALIZED_TRIALS: usize =
    10_000_000;

// ============================================================================
// Errors
// ============================================================================

/// Errors produced by logical-error benchmarking.
#[derive(Debug, Clone, PartialEq)]
pub enum LogicalBenchmarkError {
    /// Code distance was zero.
    InvalidCodeDistance {
        distance: usize,
    },

    /// A required trial count was zero.
    EmptyExperiment,

    /// Confidence level was invalid.
    InvalidConfidenceLevel {
        value: f64,
    },

    /// A probability was invalid.
    InvalidProbability {
        field: &'static str,
        value: f64,
    },

    /// Physical error rate was invalid.
    InvalidPhysicalErrorRate {
        value: f64,
    },

    /// An outcome was unknown and policy requires rejection.
    UnknownOutcome {
        trial_index: usize,
    },

    /// A comparison requires non-empty measurements.
    EmptyComparison,

    /// A mathematical result became non-finite.
    NonFiniteStatistic {
        statistic: &'static str,
    },

    /// A division or statistical calculation cannot be performed.
    InvalidDenominator,

    /// Requested capacity exceeds the configured safety bound.
    CapacityLimitExceeded {
        requested: usize,
        maximum: usize,
    },

    /// Distances were supplied in a non-increasing order.
    NonMonotonicDistance {
        previous: usize,
        current: usize,
    },

    /// A result contains inconsistent counters.
    InconsistentCounts,
}

impl fmt::Display for LogicalBenchmarkError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidCodeDistance { distance } => {
                write!(
                    formatter,
                    "logical benchmark code distance must be greater than zero, got {}",
                    distance
                )
            }

            Self::EmptyExperiment => {
                formatter.write_str(
                    "logical benchmark contains no trials",
                )
            }

            Self::InvalidConfidenceLevel { value } => {
                write!(
                    formatter,
                    "confidence level must be in [{}, {}], got {}",
                    MIN_CONFIDENCE_LEVEL,
                    MAX_CONFIDENCE_LEVEL,
                    value
                )
            }

            Self::InvalidProbability { field, value } => {
                write!(
                    formatter,
                    "{} must be finite and in [0, 1], got {}",
                    field,
                    value
                )
            }

            Self::InvalidPhysicalErrorRate { value } => {
                write!(
                    formatter,
                    "physical error rate must be finite and in [0, 1], got {}",
                    value
                )
            }

            Self::UnknownOutcome { trial_index } => {
                write!(
                    formatter,
                    "logical outcome at trial {} is unknown",
                    trial_index
                )
            }

            Self::EmptyComparison => {
                formatter.write_str(
                    "logical benchmark comparison requires measurements",
                )
            }

            Self::NonFiniteStatistic { statistic } => {
                write!(
                    formatter,
                    "logical benchmark produced a non-finite {}",
                    statistic
                )
            }

            Self::InvalidDenominator => {
                formatter.write_str(
                    "logical benchmark has no classifiable denominator",
                )
            }

            Self::CapacityLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "requested trial capacity {} exceeds maximum {}",
                    requested,
                    maximum
                )
            }

            Self::NonMonotonicDistance {
                previous,
                current,
            } => {
                write!(
                    formatter,
                    "code distances must be strictly increasing: {} followed by {}",
                    previous,
                    current
                )
            }

            Self::InconsistentCounts => {
                formatter.write_str(
                    "logical benchmark counters are internally inconsistent",
                )
            }
        }
    }
}

impl std::error::Error for LogicalBenchmarkError {}

/// Result alias for this module.
pub type LogicalBenchmarkResult<T> =
    Result<T, LogicalBenchmarkError>;

// ============================================================================
// Unknown outcome policy
// ============================================================================

/// Policy for logical outcomes that could not be classified safely.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub enum UnknownOutcomePolicy {
    /// Exclude unknown outcomes from the statistical denominator.
    ///
    /// Unknown outcomes remain explicitly visible in the result.
    Exclude,

    /// Count unknown outcomes as logical failures.
    ///
    /// This is conservative and appropriate when an unknown result means the
    /// correction could not be demonstrated to succeed.
    CountAsFailure,

    /// Reject the entire experiment if any unknown outcome exists.
    ///
    /// This is the default fail-closed policy.
    Reject,
}

impl UnknownOutcomePolicy {
    /// Stable machine-readable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Exclude => "exclude",
            Self::CountAsFailure => "count_as_failure",
            Self::Reject => "reject",
        }
    }
}

// ============================================================================
// Logical trial
// ============================================================================

/// One logically classified QEC trial.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LogicalTrial {
    /// Logical outcome generated by canonical QEC equivalence analysis.
    pub outcome: LogicalOutcome,
}

impl LogicalTrial {
    /// Creates a trial from a canonical logical outcome.
    #[must_use]
    pub const fn new(outcome: LogicalOutcome) -> Self {
        Self { outcome }
    }

    /// Returns whether the trial is a successful logical correction.
    #[must_use]
    pub const fn is_success(self) -> bool {
        self.outcome.is_success()
    }

    /// Returns whether the trial contains a logical X/Y/Z failure.
    #[must_use]
    pub const fn is_logical_failure(self) -> bool {
        self.outcome.is_logical_failure()
    }

    /// Returns whether the trial is unknown.
    #[must_use]
    pub const fn is_unknown(self) -> bool {
        self.outcome.is_unknown()
    }
}

// ============================================================================
// Configuration
// ============================================================================

/// Configuration for a logical-error benchmark.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LogicalBenchmarkConfig {
    /// Code distance under test.
    pub code_distance: usize,

    /// Confidence level used by the Wilson interval.
    pub confidence_level: f64,

    /// Treatment of unknown logical outcomes.
    pub unknown_outcome_policy: UnknownOutcomePolicy,

    /// Optional physical error rate associated with this experiment.
    ///
    /// This is metadata for logical-vs-physical analysis. This module does not
    /// estimate it.
    pub physical_error_rate: Option<f64>,

    /// Maximum number of materialized trials allowed by this benchmark.
    pub max_materialized_trials: usize,
}

impl LogicalBenchmarkConfig {
    /// Creates a production configuration.
    #[must_use]
    pub fn new(
        code_distance: usize,
    ) -> LogicalBenchmarkResult<Self> {
        Self {
            code_distance,
            confidence_level: DEFAULT_CONFIDENCE_LEVEL,
            unknown_outcome_policy: UnknownOutcomePolicy::Reject,
            physical_error_rate: None,
            max_materialized_trials:
                DEFAULT_MAX_MATERIALIZED_TRIALS,
        }
        .validate()
    }

    /// Sets the confidence level.
    pub fn with_confidence_level(
        mut self,
        confidence_level: f64,
    ) -> Self {
        self.confidence_level = confidence_level;
        self
    }

    /// Sets unknown-outcome handling.
    pub fn with_unknown_policy(
        mut self,
        policy: UnknownOutcomePolicy,
    ) -> Self {
        self.unknown_outcome_policy = policy;
        self
    }

    /// Associates a physical error rate with the experiment.
    pub fn with_physical_error_rate(
        mut self,
        physical_error_rate: f64,
    ) -> Self {
        self.physical_error_rate =
            Some(physical_error_rate);
        self
    }

    /// Sets the maximum materialized trial count.
    pub fn with_max_materialized_trials(
        mut self,
        maximum: usize,
    ) -> Self {
        self.max_materialized_trials = maximum;
        self
    }

    /// Validates the configuration.
    pub fn validate(
        &self,
    ) -> LogicalBenchmarkResult<Self> {
        if self.code_distance == 0 {
            return Err(
                LogicalBenchmarkError::InvalidCodeDistance {
                    distance: self.code_distance,
                },
            );
        }

        validate_confidence_level(
            self.confidence_level,
        )?;

        if let Some(rate) =
            self.physical_error_rate
        {
            validate_probability(
                "physical_error_rate",
                rate,
            )
            .map_err(|_| {
                LogicalBenchmarkError::InvalidPhysicalErrorRate {
                    value: rate,
                }
            })?;
        }

        if self.max_materialized_trials == 0 {
            return Err(
                LogicalBenchmarkError::CapacityLimitExceeded {
                    requested: 1,
                    maximum: 0,
                },
            );
        }

        Ok(*self)
    }
}

// ============================================================================
// Wilson confidence interval
// ============================================================================

/// Wilson confidence interval for a binomial proportion.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WilsonInterval {
    /// Observed proportion.
    pub estimate: f64,

    /// Lower confidence bound.
    pub lower: f64,

    /// Upper confidence bound.
    pub upper: f64,

    /// Confidence level.
    pub confidence_level: f64,

    /// Number of successes.
    pub successes: u64,

    /// Number of trials.
    pub trials: u64,
}

impl WilsonInterval {
    /// Returns interval width.
    #[must_use]
    pub fn width(self) -> f64 {
        self.upper - self.lower
    }

    /// Returns the midpoint of the confidence interval.
    #[must_use]
    pub fn midpoint(self) -> f64 {
        (self.lower + self.upper) / 2.0
    }

    /// Returns whether the entire interval lies above a threshold.
    #[must_use]
    pub fn lower_above(
        self,
        threshold: f64,
    ) -> bool {
        self.lower > threshold
    }

    /// Returns whether the entire interval lies below a threshold.
    #[must_use]
    pub fn upper_below(
        self,
        threshold: f64,
    ) -> bool {
        self.upper < threshold
    }
}

// ============================================================================
// Error decomposition
// ============================================================================

/// Logical X/Y/Z error decomposition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct LogicalErrorCounts {
    /// Logical X failures.
    pub logical_x: u64,

    /// Logical Y failures.
    pub logical_y: u64,

    /// Logical Z failures.
    pub logical_z: u64,

    /// Identity/stabilizer-equivalent successes.
    pub successes: u64,

    /// Unknown outcomes.
    pub unknown: u64,
}

impl LogicalErrorCounts {
    /// Creates an empty counter.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            logical_x: 0,
            logical_y: 0,
            logical_z: 0,
            successes: 0,
            unknown: 0,
        }
    }

    /// Total observed trials.
    #[must_use]
    pub const fn total(self) -> u64 {
        self.logical_x
            + self.logical_y
            + self.logical_z
            + self.successes
            + self.unknown
    }

    /// Number of classified trials.
    #[must_use]
    pub const fn classified(self) -> u64 {
        self.logical_x
            + self.logical_y
            + self.logical_z
            + self.successes
    }

    /// Number of logical failures.
    #[must_use]
    pub const fn logical_failures(self) -> u64 {
        self.logical_x
            + self.logical_y
            + self.logical_z
    }

    /// Adds one outcome.
    pub fn record(
        &mut self,
        outcome: LogicalOutcome,
    ) {
        match outcome {
            LogicalOutcome::Identity => {
                self.successes += 1;
            }

            LogicalOutcome::LogicalX => {
                self.logical_x += 1;
            }

            LogicalOutcome::LogicalY => {
                self.logical_y += 1;
            }

            LogicalOutcome::LogicalZ => {
                self.logical_z += 1;
            }

            LogicalOutcome::Unknown => {
                self.unknown += 1;
            }
        }
    }

    /// Validates the internal accounting invariant.
    pub fn validate(
        &self,
    ) -> LogicalBenchmarkResult<()> {
        let expected = self
            .logical_x
            .checked_add(self.logical_y)
            .and_then(|value| {
                value.checked_add(self.logical_z)
            })
            .and_then(|value| {
                value.checked_add(self.successes)
            })
            .and_then(|value| {
                value.checked_add(self.unknown)
            })
            .ok_or(
                LogicalBenchmarkError::InconsistentCounts,
            )?;

        if expected != self.total() {
            return Err(
                LogicalBenchmarkError::InconsistentCounts,
            );
        }

        Ok(())
    }
}

// ============================================================================
// Benchmark result
// ============================================================================

/// Complete logical-error benchmark result.
#[derive(Debug, Clone, PartialEq)]
pub struct LogicalBenchmarkResult {
    /// Benchmark identifier.
    pub benchmark_id: &'static str,

    /// Result schema version.
    pub schema_version: u32,

    /// Code distance.
    pub code_distance: usize,

    /// Total observations received.
    pub total_trials: u64,

    /// Observations included in the statistical denominator.
    pub analyzed_trials: u64,

    /// Logical failures included in the statistical numerator.
    pub logical_failures: u64,

    /// Successful logical outcomes.
    pub successes: u64,

    /// Unknown outcomes.
    pub unknown_outcomes: u64,

    /// Logical X failures.
    pub logical_x_failures: u64,

    /// Logical Y failures.
    pub logical_y_failures: u64,

    /// Logical Z failures.
    pub logical_z_failures: u64,

    /// Observed logical error rate.
    pub logical_error_rate: f64,

    /// Logical success rate.
    pub logical_success_rate: f64,

    /// Confidence interval for logical error rate.
    pub logical_error_confidence_interval: WilsonInterval,

    /// X-specific error rate.
    pub logical_x_error_rate: f64,

    /// Y-specific error rate.
    pub logical_y_error_rate: f64,

    /// Z-specific error rate.
    pub logical_z_error_rate: f64,

    /// Optional physical error rate supplied by the caller.
    pub physical_error_rate: Option<f64>,

    /// Ratio of logical to physical error rate, when defined.
    ///
    /// This is a descriptive ratio, not a threshold estimate.
    pub logical_to_physical_error_ratio: Option<f64>,

    /// Unknown-outcome handling policy.
    pub unknown_outcome_policy: UnknownOutcomePolicy,

    /// Confidence level used.
    pub confidence_level: f64,
}

impl LogicalBenchmarkResult {
    /// Returns true when the benchmark has no unknown outcomes.
    #[must_use]
    pub const fn is_complete(&self) -> bool {
        self.unknown_outcomes == 0
    }

    /// Returns true when logical error rate is strictly below physical error
    /// rate.
    #[must_use]
    pub fn suppresses_physical_error(
        &self,
    ) -> Option<bool> {
        self.physical_error_rate
            .map(|physical| {
                self.logical_error_rate < physical
            })
    }

    /// Returns true when the upper confidence bound is below the supplied
    /// physical error rate.
    ///
    /// This is a stronger statistical statement than comparing point
    /// estimates.
    #[must_use]
    pub fn statistically_below(
        &self,
        physical_error_rate: f64,
    ) -> LogicalBenchmarkResult<bool> {
        validate_probability(
            "physical_error_rate",
            physical_error_rate,
        )?;

        Ok(self
            .logical_error_confidence_interval
            .upper
            < physical_error_rate)
    }

    /// Returns whether the result contains enough information for a finite
    /// logical-error estimate.
    #[must_use]
    pub const fn has_valid_denominator(
        &self,
    ) -> bool {
        self.analyzed_trials > 0
    }

    /// Returns a stable textual status.
    #[must_use]
    pub const fn status(&self) -> &'static str {
        if self.analyzed_trials == 0 {
            "no-classifiable-data"
        } else if self.unknown_outcomes > 0 {
            "partial"
        } else {
            "complete"
        }
    }
}

// ============================================================================
// Streaming accumulator
// ============================================================================

/// Incremental logical-error benchmark accumulator.
///
/// This type is the preferred production path for large experiments because
/// it does not require retaining every trial in memory.
///
/// The accumulator is deterministic and can be fed by:
///
/// - simulator results;
/// - QPU results;
/// - streaming QEC;
/// - distributed workers after deterministic aggregation;
/// - replay fixtures.
#[derive(Debug, Clone)]
pub struct LogicalBenchmark {
    config: LogicalBenchmarkConfig,
    counts: LogicalErrorCounts,
    next_trial_index: usize,
}

impl LogicalBenchmark {
    /// Creates an empty benchmark.
    pub fn new(
        config: LogicalBenchmarkConfig,
    ) -> LogicalBenchmarkResult<Self> {
        config.validate()?;

        Ok(Self {
            config,
            counts: LogicalErrorCounts::new(),
            next_trial_index: 0,
        })
    }

    /// Creates an empty benchmark with an explicit materialization capacity.
    ///
    /// The capacity is only a safety declaration for callers that intend to
    /// materialize observations before feeding them to the accumulator.
    pub fn with_capacity(
        config: LogicalBenchmarkConfig,
        capacity: usize,
    ) -> LogicalBenchmarkResult<Self> {
        config.validate()?;

        if capacity > config.max_materialized_trials {
            return Err(
                LogicalBenchmarkError::CapacityLimitExceeded {
                    requested: capacity,
                    maximum: config.max_materialized_trials,
                },
            );
        }

        Ok(Self {
            config,
            counts: LogicalErrorCounts::new(),
            next_trial_index: 0,
        })
    }

    /// Returns the immutable benchmark configuration.
    #[must_use]
    pub const fn config(
        &self,
    ) -> LogicalBenchmarkConfig {
        self.config
    }

    /// Returns current counters.
    #[must_use]
    pub const fn counts(
        &self,
    ) -> LogicalErrorCounts {
        self.counts
    }

    /// Returns the number of observations received.
    #[must_use]
    pub const fn total_trials(
        &self,
    ) -> u64 {
        self.counts.total()
    }

    /// Records one logical outcome.
    pub fn record(
        &mut self,
        outcome: LogicalOutcome,
    ) -> LogicalBenchmarkResult<()> {
        let trial_index = self.next_trial_index;

        if outcome.is_unknown()
            && matches!(
                self.config.unknown_outcome_policy,
                UnknownOutcomePolicy::Reject
            )
        {
            return Err(
                LogicalBenchmarkError::UnknownOutcome {
                    trial_index,
                },
            );
        }

        self.counts.record(outcome);

        self.next_trial_index = self
            .next_trial_index
            .checked_add(1)
            .ok_or(
                LogicalBenchmarkError::CapacityLimitExceeded {
                    requested: usize::MAX,
                    maximum: self
                        .config
                        .max_materialized_trials,
                },
            )?;

        Ok(())
    }

    /// Records an entire iterator of outcomes.
    ///
    /// If the operation fails under `Reject`, the accumulator may already
    /// contain earlier successfully accepted observations. Callers that need
    /// transactional behavior should validate their input before calling this
    /// method.
    pub fn extend<I>(
        &mut self,
        outcomes: I,
    ) -> LogicalBenchmarkResult<()>
    where
        I: IntoIterator<Item = LogicalOutcome>,
    {
        for outcome in outcomes {
            self.record(outcome)?;
        }

        Ok(())
    }

    /// Finalizes the benchmark into an immutable statistical result.
    pub fn finalize(
        &self,
    ) -> LogicalBenchmarkResult<LogicalBenchmarkResult> {
        self.counts.validate()?;

        let total = self.counts.total();

        if total == 0 {
            return Err(
                LogicalBenchmarkError::EmptyExperiment,
            );
        }

        let (analyzed, failures) =
            match self.config.unknown_outcome_policy {
                UnknownOutcomePolicy::Exclude => (
                    self.counts.classified(),
                    self.counts.logical_failures(),
                ),

                UnknownOutcomePolicy::CountAsFailure => (
                    total,
                    self.counts
                        .logical_failures()
                        .checked_add(
                            self.counts.unknown,
                        )
                        .ok_or(
                            LogicalBenchmarkError::InconsistentCounts,
                        )?,
                ),

                UnknownOutcomePolicy::Reject => {
                    if self.counts.unknown > 0 {
                        return Err(
                            LogicalBenchmarkError::UnknownOutcome {
                                trial_index: self
                                    .next_trial_index
                                    .saturating_sub(1),
                            },
                        );
                    }

                    (
                        self.counts.classified(),
                        self.counts.logical_failures(),
                    )
                }
            };

        if analyzed == 0 {
            return Err(
                LogicalBenchmarkError::InvalidDenominator,
            );
        }

        if failures > analyzed {
            return Err(
                LogicalBenchmarkError::InconsistentCounts,
            );
        }

        let logical_error_rate =
            failures as f64 / analyzed as f64;

        let logical_success_rate =
            1.0 - logical_error_rate;

        validate_probability(
            "logical_error_rate",
            logical_error_rate,
        )?;

        validate_probability(
            "logical_success_rate",
            logical_success_rate,
        )?;

        let interval = wilson_interval(
            failures,
            analyzed,
            self.config.confidence_level,
        )?;

        let denominator = analyzed as f64;

        let logical_x_error_rate =
            self.counts.logical_x as f64
                / denominator;

        let logical_y_error_rate =
            self.counts.logical_y as f64
                / denominator;

        let logical_z_error_rate =
            self.counts.logical_z as f64
                / denominator;

        validate_probability(
            "logical_x_error_rate",
            logical_x_error_rate,
        )?;

        validate_probability(
            "logical_y_error_rate",
            logical_y_error_rate,
        )?;

        validate_probability(
            "logical_z_error_rate",
            logical_z_error_rate,
        )?;

        let logical_to_physical_error_ratio =
            match self.config.physical_error_rate {
                Some(physical) if physical > 0.0 => {
                    let ratio =
                        logical_error_rate / physical;

                    if !ratio.is_finite() {
                        return Err(
                            LogicalBenchmarkError::NonFiniteStatistic {
                                statistic:
                                    "logical-to-physical error ratio",
                            },
                        );
                    }

                    Some(ratio)
                }

                _ => None,
            };

        Ok(LogicalBenchmarkResult {
            benchmark_id:
                LOGICAL_ERROR_BENCHMARK_ID,

            schema_version:
                LOGICAL_ERROR_RESULT_SCHEMA_VERSION,

            code_distance:
                self.config.code_distance,

            total_trials: total,

            analyzed_trials: analyzed,

            logical_failures: failures,

            successes: self.counts.successes,

            unknown_outcomes:
                self.counts.unknown,

            logical_x_failures:
                self.counts.logical_x,

            logical_y_failures:
                self.counts.logical_y,

            logical_z_failures:
                self.counts.logical_z,

            logical_error_rate,

            logical_success_rate,

            logical_error_confidence_interval:
                interval,

            logical_x_error_rate,

            logical_y_error_rate,

            logical_z_error_rate,

            physical_error_rate:
                self.config.physical_error_rate,

            logical_to_physical_error_ratio,

            unknown_outcome_policy:
                self.config.unknown_outcome_policy,

            confidence_level:
                self.config.confidence_level,
        })
    }
}

// ============================================================================
// Direct batch analysis
// ============================================================================

/// Analyze a batch of logical outcomes without requiring the caller to manage
/// an accumulator explicitly.
pub fn analyze_logical_outcomes<I>(
    config: LogicalBenchmarkConfig,
    outcomes: I,
) -> LogicalBenchmarkResult<LogicalBenchmarkResult>
where
    I: IntoIterator<Item = LogicalOutcome>,
{
    let mut benchmark =
        LogicalBenchmark::new(config)?;

    benchmark.extend(outcomes)?;

    benchmark.finalize()
}

/// Analyze canonical `LogicalTrial` values.
pub fn analyze_trials<I>(
    config: LogicalBenchmarkConfig,
    trials: I,
) -> LogicalBenchmarkResult<LogicalBenchmarkResult>
where
    I: IntoIterator<Item = LogicalTrial>,
{
    let mut benchmark =
        LogicalBenchmark::new(config)?;

    for trial in trials {
        benchmark.record(trial.outcome)?;
    }

    benchmark.finalize()
}

// ============================================================================
// Confidence interval implementation
// ============================================================================

/// Computes a Wilson score interval for a binomial proportion.
///
/// This function is intentionally public because future
/// `statistics/confidence.rs` can use it as a compatibility reference and
/// regression fixture.
///
/// `successes / trials` is the estimated proportion.
pub fn wilson_interval(
    successes: u64,
    trials: u64,
    confidence_level: f64,
) -> LogicalBenchmarkResult<WilsonInterval> {
    if trials == 0 {
        return Err(
            LogicalBenchmarkError::InvalidDenominator,
        );
    }

    if successes > trials {
        return Err(
            LogicalBenchmarkError::InconsistentCounts,
        );
    }

    validate_confidence_level(
        confidence_level,
    )?;

    let estimate =
        successes as f64 / trials as f64;

    let alpha =
        1.0 - confidence_level;

    let tail_probability =
        1.0 - alpha / 2.0;

    let z =
        standard_normal_inverse_cdf(tail_probability)?;

    let n = trials as f64;
    let z_squared = z * z;

    let denominator =
        1.0 + z_squared / n;

    let center =
        (estimate + z_squared / (2.0 * n))
            / denominator;

    let spread_numerator =
        (estimate * (1.0 - estimate) / n)
            + z_squared / (4.0 * n * n);

    if spread_numerator < 0.0
        || !spread_numerator.is_finite()
    {
        return Err(
            LogicalBenchmarkError::NonFiniteStatistic {
                statistic:
                    "Wilson spread numerator",
            },
        );
    }

    let spread =
        z * spread_numerator.sqrt()
            / denominator;

    let lower =
        (center - spread).max(0.0);

    let upper =
        (center + spread).min(1.0);

    for (name, value) in [
        ("Wilson estimate", estimate),
        ("Wilson lower bound", lower),
        ("Wilson upper bound", upper),
    ] {
        if !value.is_finite() {
            return Err(
                LogicalBenchmarkError::NonFiniteStatistic {
                    statistic: name,
                },
            );
        }
    }

    Ok(WilsonInterval {
        estimate,
        lower,
        upper,
        confidence_level,
        successes,
        trials,
    })
}

// ============================================================================
// Distance sweep
// ============================================================================

/// One logical-error measurement at one code distance.
#[derive(Debug, Clone, PartialEq)]
pub struct LogicalDistancePoint {
    /// Code distance.
    pub distance: usize,

    /// Logical-error benchmark result.
    pub result: LogicalBenchmarkResult,
}

impl LogicalDistancePoint {
    /// Creates a distance point after validating that the result corresponds
    /// to the supplied distance.
    pub fn new(
        distance: usize,
        result: LogicalBenchmarkResult,
    ) -> LogicalBenchmarkResult<Self> {
        if distance == 0 {
            return Err(
                LogicalBenchmarkError::InvalidCodeDistance {
                    distance,
                },
            );
        }

        if result.code_distance != distance {
            return Err(
                LogicalBenchmarkError::InconsistentCounts,
            );
        }

        Ok(Self { distance, result })
    }
}

/// Deterministic code-distance logical-error sweep.
#[derive(Debug, Clone, Default)]
pub struct LogicalBenchmarkSweep {
    points: Vec<LogicalDistancePoint>,
}

impl LogicalBenchmarkSweep {
    /// Creates an empty sweep.
    #[must_use]
    pub const fn new() -> Self {
        Self { points: Vec::new() }
    }

    /// Creates a sweep with preallocated capacity.
    pub fn with_capacity(
        capacity: usize,
    ) -> LogicalBenchmarkResult<Self> {
        if capacity
            > DEFAULT_MAX_MATERIALIZED_TRIALS
        {
            return Err(
                LogicalBenchmarkError::CapacityLimitExceeded {
                    requested: capacity,
                    maximum:
                        DEFAULT_MAX_MATERIALIZED_TRIALS,
                },
            );
        }

        Ok(Self {
            points: Vec::with_capacity(
                capacity,
            ),
        })
    }

    /// Adds a point.
    ///
    /// Distances must be strictly increasing. This deterministic ordering makes
    /// serialization, comparison and regression analysis stable.
    pub fn push(
        &mut self,
        point: LogicalDistancePoint,
    ) -> LogicalBenchmarkResult<()> {
        if let Some(previous) =
            self.points.last()
        {
            if point.distance
                <= previous.distance
            {
                return Err(
                    LogicalBenchmarkError::NonMonotonicDistance {
                        previous: previous.distance,
                        current: point.distance,
                    },
                );
            }
        }

        self.points.push(point);

        Ok(())
    }

    /// Returns all sweep points.
    #[must_use]
    pub fn points(
        &self,
    ) -> &[LogicalDistancePoint] {
        &self.points
    }

    /// Returns the number of distances measured.
    #[must_use]
    pub fn len(&self) -> usize {
        self.points.len()
    }

    /// Returns whether the sweep is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.points.is_empty()
    }

    /// Returns the first measured point.
    #[must_use]
    pub fn first(
        &self,
    ) -> Option<&LogicalDistancePoint> {
        self.points.first()
    }

    /// Returns the last measured point.
    #[must_use]
    pub fn last(
        &self,
    ) -> Option<&LogicalDistancePoint> {
        self.points.last()
    }

    /// Calculates the ratio:
    ///
    /// ```text
    /// error_rate_at_lower_distance
    /// --------------------------------
    /// error_rate_at_higher_distance
    /// ```
    ///
    /// A value greater than one indicates error suppression as distance
    /// increases.
    pub fn suppression_ratio(
        &self,
        lower_index: usize,
        higher_index: usize,
    ) -> LogicalBenchmarkResult<f64> {
        let lower =
            self.points
                .get(lower_index)
                .ok_or(
                    LogicalBenchmarkError::EmptyComparison,
                )?;

        let higher =
            self.points
                .get(higher_index)
                .ok_or(
                    LogicalBenchmarkError::EmptyComparison,
                )?;

        if higher
            .result
            .logical_error_rate
            <= 0.0
        {
            return Err(
                LogicalBenchmarkError::InvalidDenominator,
            );
        }

        let ratio =
            lower.result.logical_error_rate
                / higher
                    .result
                    .logical_error_rate;

        if !ratio.is_finite() {
            return Err(
                LogicalBenchmarkError::NonFiniteStatistic {
                    statistic:
                        "logical suppression ratio",
                },
            );
        }

        Ok(ratio)
    }

    /// Returns the point with the lowest observed logical-error rate.
    #[must_use]
    pub fn best_observed(
        &self,
    ) -> Option<&LogicalDistancePoint> {
        self.points.iter().min_by(|left, right| {
            left.result
                .logical_error_rate
                .total_cmp(
                    &right
                        .result
                        .logical_error_rate,
                )
        })
    }
}

// ============================================================================
// Benchmark comparison
// ============================================================================

/// Difference between two logical-error measurements.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LogicalBenchmarkComparison {
    /// Error rate of the first benchmark.
    pub first_error_rate: f64,

    /// Error rate of the second benchmark.
    pub second_error_rate: f64,

    /// Absolute difference:
    ///
    /// `first - second`.
    pub absolute_difference: f64,

    /// Relative difference:
    ///
    /// `(first - second) / second`.
    ///
    /// Undefined when the second error rate is zero.
    pub relative_difference: Option<f64>,

    /// Multiplicative improvement:
    ///
    /// `first / second`.
    ///
    /// Values greater than one mean the second benchmark has lower error.
    pub improvement_factor: Option<f64>,
}

impl LogicalBenchmarkComparison {
    /// Compares two benchmark results.
    pub fn between(
        first: &LogicalBenchmarkResult,
        second: &LogicalBenchmarkResult,
    ) -> LogicalBenchmarkResult<Self> {
        if first.analyzed_trials == 0
            || second.analyzed_trials == 0
        {
            return Err(
                LogicalBenchmarkError::InvalidDenominator,
            );
        }

        let first_rate =
            first.logical_error_rate;

        let second_rate =
            second.logical_error_rate;

        let absolute_difference =
            first_rate - second_rate;

        let relative_difference =
            if second_rate > 0.0 {
                let value =
                    absolute_difference
                        / second_rate;

                if !value.is_finite() {
                    return Err(
                        LogicalBenchmarkError::NonFiniteStatistic {
                            statistic:
                                "relative logical-error difference",
                        },
                    );
                }

                Some(value)
            } else {
                None
            };

        let improvement_factor =
            if second_rate > 0.0 {
                let value =
                    first_rate / second_rate;

                if !value.is_finite() {
                    return Err(
                        LogicalBenchmarkError::NonFiniteStatistic {
                            statistic:
                                "logical-error improvement factor",
                        },
                    );
                }

                Some(value)
            } else {
                None
            };

        Ok(Self {
            first_error_rate: first_rate,
            second_error_rate: second_rate,
            absolute_difference,
            relative_difference,
            improvement_factor,
        })
    }
}

// ============================================================================
// Utility validation
// ============================================================================

fn validate_probability(
    field: &'static str,
    value: f64,
) -> LogicalBenchmarkResult<()> {
    if !value.is_finite()
        || value < MIN_PROBABILITY
        || value > MAX_PROBABILITY
    {
        return Err(
            LogicalBenchmarkError::InvalidProbability {
                field,
                value,
            },
        );
    }

    Ok(())
}

fn validate_confidence_level(
    confidence_level: f64,
) -> LogicalBenchmarkResult<()> {
    if !confidence_level.is_finite()
        || confidence_level
            < MIN_CONFIDENCE_LEVEL
        || confidence_level
            > MAX_CONFIDENCE_LEVEL
    {
        return Err(
            LogicalBenchmarkError::InvalidConfidenceLevel {
                value: confidence_level,
            },
        );
    }

    Ok(())
}

// ============================================================================
// Standard normal inverse CDF
// ============================================================================

/// Inverse CDF of the standard normal distribution.
///
/// This uses Peter John Acklam's rational approximation. The implementation is
/// self-contained so this benchmark does not introduce a numerical dependency
/// merely for confidence intervals.
///
/// Accuracy is sufficient for benchmark confidence intervals across the
/// supported confidence range.
fn standard_normal_inverse_cdf(
    probability: f64,
) -> LogicalBenchmarkResult<f64> {
    if !probability.is_finite()
        || probability <= 0.0
        || probability >= 1.0
    {
        return Err(
            LogicalBenchmarkError::InvalidProbability {
                field:
                    "normal quantile probability",
                value: probability,
            },
        );
    }

    // Coefficients for the Acklam approximation.
    const A1: f64 =
        -3.969683028665376e1;
    const A2: f64 =
        2.209460984245205e2;
    const A3: f64 =
        -2.759285104469687e2;
    const A4: f64 =
        1.383577518672690e2;
    const A5: f64 =
        -3.066479806614716e1;
    const A6: f64 =
        2.506628277459239e0;

    const B1: f64 =
        -5.447609879822406e1;
    const B2: f64 =
        1.615858368580409e2;
    const B3: f64 =
        -1.556989798598866e2;
    const B4: f64 =
        6.680131188771972e1;
    const B5: f64 =
        -1.328068155288572e1;

    const C1: f64 =
        -7.784894002430293e-3;
    const C2: f64 =
        -3.223964580411365e-1;
    const C3: f64 =
        -2.400758277161838e0;
    const C4: f64 =
        -2.549732539343734e0;
    const C5: f64 =
        4.374664141464968e0;
    const C6: f64 =
        2.938163982698783e0;

    const D1: f64 =
        7.784695709041462e-3;
    const D2: f64 =
        3.224671290700398e-1;
    const D3: f64 =
        2.445134137142996e0;
    const D4: f64 =
        3.754408661907416e0;

    const LOWER: f64 = 0.02425;
    const UPPER: f64 = 1.0 - LOWER;

    let result = if probability < LOWER {
        let q =
            (-2.0 * probability.ln()).sqrt();

        (((((C1 * q + C2) * q + C3) * q
            + C4)
            * q
            + C5)
            * q
            + C6)
            / ((((D1 * q + D2) * q + D3)
                * q
                + D4)
                * q
                + 1.0)
    } else if probability <= UPPER {
        let q =
            probability - 0.5;

        let r = q * q;

        (((((A1 * r + A2) * r + A3) * r
            + A4)
            * r
            + A5)
            * r
            + A6)
            * q
            / (((((B1 * r + B2) * r + B3) * r
                + B4)
                * r
                + B5)
                * r
                + 1.0)
    } else {
        let q =
            (-2.0 * (1.0 - probability).ln())
                .sqrt();

        -(((((C1 * q + C2) * q + C3) * q
            + C4)
            * q
            + C5)
            * q
            + C6)
            / ((((D1 * q + D2) * q + D3)
                * q
                + D4)
                * q
                + 1.0)
    };

    if !result.is_finite() {
        return Err(
            LogicalBenchmarkError::NonFiniteStatistic {
                statistic:
                    "standard-normal inverse CDF",
            },
        );
    }

    Ok(result)
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn outcome_sequence(
        success_count: usize,
        x_count: usize,
        y_count: usize,
        z_count: usize,
    ) -> Vec<LogicalOutcome> {
        let mut values = Vec::new();

        values.extend(
            std::iter::repeat(
                LogicalOutcome::Identity,
            )
            .take(success_count),
        );

        values.extend(
            std::iter::repeat(
                LogicalOutcome::LogicalX,
            )
            .take(x_count),
        );

        values.extend(
            std::iter::repeat(
                LogicalOutcome::LogicalY,
            )
            .take(y_count),
        );

        values.extend(
            std::iter::repeat(
                LogicalOutcome::LogicalZ,
            )
            .take(z_count),
        );

        values
    }

    #[test]
    fn default_configuration_is_valid() {
        let config =
            LogicalBenchmarkConfig::new(3)
                .expect(
                    "distance 3 must be valid",
                );

        assert_eq!(
            config.code_distance,
            3
        );

        assert_eq!(
            config.unknown_outcome_policy,
            UnknownOutcomePolicy::Reject
        );
    }

    #[test]
    fn zero_distance_is_rejected() {
        assert!(
            LogicalBenchmarkConfig::new(0)
                .is_err()
        );
    }

    #[test]
    fn logical_error_rate_is_correct() {
        let config =
            LogicalBenchmarkConfig::new(3)
                .expect("valid config");

        let outcomes =
            outcome_sequence(90, 5, 3, 2);

        let result =
            analyze_logical_outcomes(
                config,
                outcomes,
            )
            .expect("analysis must succeed");

        assert_eq!(
            result.total_trials,
            100
        );

        assert_eq!(
            result.logical_failures,
            10
        );

        assert_eq!(
            result.analyzed_trials,
            100
        );

        assert!(
            (result.logical_error_rate - 0.10)
                .abs()
                < 1.0e-12
        );

        assert!(
            (result.logical_success_rate - 0.90)
                .abs()
                < 1.0e-12
        );
    }

    #[test]
    fn logical_components_share_the_statistical_denominator() {
        let config =
            LogicalBenchmarkConfig::new(3)
                .expect("valid config");

        let outcomes =
            outcome_sequence(90, 5, 3, 2);

        let result =
            analyze_logical_outcomes(
                config,
                outcomes,
            )
            .expect("analysis must succeed");

        assert!(
            (result.logical_x_error_rate - 0.05)
                .abs()
                < 1.0e-12
        );

        assert!(
            (result.logical_y_error_rate - 0.03)
                .abs()
                < 1.0e-12
        );

        assert!(
            (result.logical_z_error_rate - 0.02)
                .abs()
                < 1.0e-12
        );
    }

    #[test]
    fn zero_logical_errors_have_zero_estimate() {
        let config =
            LogicalBenchmarkConfig::new(3)
                .expect("valid config");

        let outcomes =
            outcome_sequence(100, 0, 0, 0);

        let result =
            analyze_logical_outcomes(
                config,
                outcomes,
            )
            .expect("analysis must succeed");

        assert_eq!(
            result.logical_error_rate,
            0.0
        );

        assert_eq!(
            result.logical_error_confidence_interval
                .lower,
            0.0
        );

        assert!(
            result
                .logical_error_confidence_interval
                .upper
                > 0.0
        );
    }

    #[test]
    fn all_logical_errors_have_rate_one() {
        let config =
            LogicalBenchmarkConfig::new(3)
                .expect("valid config");

        let outcomes =
            outcome_sequence(0, 30, 20, 50);

        let result =
            analyze_logical_outcomes(
                config,
                outcomes,
            )
            .expect("analysis must succeed");

        assert_eq!(
            result.logical_error_rate,
            1.0
        );

        assert_eq!(
            result.logical_success_rate,
            0.0
        );

        assert_eq!(
            result
                .logical_error_confidence_interval
                .upper,
            1.0
        );
    }

    #[test]
    fn unknown_outcomes_are_rejected_by_default() {
        let config =
            LogicalBenchmarkConfig::new(3)
                .expect("valid config");

        let mut benchmark =
            LogicalBenchmark::new(config)
                .expect("valid benchmark");

        benchmark
            .record(LogicalOutcome::Identity)
            .expect("identity accepted");

        let result =
            benchmark.record(
                LogicalOutcome::Unknown,
            );

        assert!(matches!(
            result,
            Err(
                LogicalBenchmarkError::UnknownOutcome {
                    trial_index: 1
                }
            )
        ));
    }

    #[test]
    fn unknown_outcomes_can_be_excluded_explicitly() {
        let config =
            LogicalBenchmarkConfig::new(3)
                .expect("valid config")
                .with_unknown_policy(
                    UnknownOutcomePolicy::Exclude,
                );

        let outcomes = vec![
            LogicalOutcome::Identity,
            LogicalOutcome::Identity,
            LogicalOutcome::LogicalX,
            LogicalOutcome::Unknown,
        ];

        let result =
            analyze_logical_outcomes(
                config,
                outcomes,
            )
            .expect("analysis must succeed");

        assert_eq!(
            result.total_trials,
            4
        );

        assert_eq!(
            result.unknown_outcomes,
            1
        );

        assert_eq!(
            result.analyzed_trials,
            3
        );

        assert!(
            (result.logical_error_rate
                - 1.0 / 3.0)
                .abs()
                < 1.0e-12
        );
    }

    #[test]
    fn unknown_outcomes_can_be_counted_as_failures() {
        let config =
            LogicalBenchmarkConfig::new(3)
                .expect("valid config")
                .with_unknown_policy(
                    UnknownOutcomePolicy::CountAsFailure,
                );

        let outcomes = vec![
            LogicalOutcome::Identity,
            LogicalOutcome::Identity,
            LogicalOutcome::LogicalX,
            LogicalOutcome::Unknown,
        ];

        let result =
            analyze_logical_outcomes(
                config,
                outcomes,
            )
            .expect("analysis must succeed");

        assert_eq!(
            result.analyzed_trials,
            4
        );

        assert_eq!(
            result.logical_failures,
            2
        );

        assert!(
            (result.logical_error_rate - 0.5)
                .abs()
                < 1.0e-12
        );
    }

    #[test]
    fn wilson_interval_contains_point_estimate() {
        let interval =
            wilson_interval(
                10,
                100,
                DEFAULT_CONFIDENCE_LEVEL,
            )
            .expect("valid interval");

        assert!(
            interval.lower
                <= interval.estimate
        );

        assert!(
            interval.estimate
                <= interval.upper
        );

        assert!(
            interval.lower >= 0.0
        );

        assert!(
            interval.upper <= 1.0
        );
    }

    #[test]
    fn wilson_interval_is_symmetric_for_half_rate() {
        let interval =
            wilson_interval(
                50,
                100,
                DEFAULT_CONFIDENCE_LEVEL,
            )
            .expect("valid interval");

        let lower_distance =
            interval.estimate
                - interval.lower;

        let upper_distance =
            interval.upper
                - interval.estimate;

        assert!(
            (lower_distance - upper_distance)
                .abs()
                < 1.0e-12
        );
    }

    #[test]
    fn invalid_confidence_is_rejected() {
        assert!(
            wilson_interval(
                10,
                100,
                1.0,
            )
            .is_err()
        );

        assert!(
            wilson_interval(
                10,
                100,
                0.1,
            )
            .is_err()
        );
    }

    #[test]
    fn successes_cannot_exceed_trials() {
        assert!(
            wilson_interval(
                101,
                100,
                DEFAULT_CONFIDENCE_LEVEL,
            )
            .is_err()
        );
    }

    #[test]
    fn empty_experiment_is_rejected() {
        let config =
            LogicalBenchmarkConfig::new(3)
                .expect("valid config");

        let result =
            analyze_logical_outcomes(
                config,
                std::iter::empty(),
            );

        assert!(matches!(
            result,
            Err(
                LogicalBenchmarkError::EmptyExperiment
            )
        ));
    }

    #[test]
    fn streaming_and_batch_analysis_match() {
        let outcomes =
            outcome_sequence(80, 8, 7, 5);

        let config =
            LogicalBenchmarkConfig::new(5)
                .expect("valid config");

        let batch =
            analyze_logical_outcomes(
                config,
                outcomes.clone(),
            )
            .expect("batch must succeed");

        let mut streaming =
            LogicalBenchmark::new(config)
                .expect("streaming must succeed");

        for outcome in outcomes {
            streaming
                .record(outcome)
                .expect("record must succeed");
        }

        let streamed =
            streaming
                .finalize()
                .expect("finalize must succeed");

        assert_eq!(
            batch,
            streamed
        );
    }

    #[test]
    fn distance_sweep_requires_increasing_distance() {
        let config3 =
            LogicalBenchmarkConfig::new(3)
                .expect("valid config");

        let result3 =
            analyze_logical_outcomes(
                config3,
                outcome_sequence(99, 1, 0, 0),
            )
            .expect("analysis must succeed");

        let point3 =
            LogicalDistancePoint::new(
                3,
                result3,
            )
            .expect("point must be valid");

        let config5 =
            LogicalBenchmarkConfig::new(5)
                .expect("valid config");

        let result5 =
            analyze_logical_outcomes(
                config5,
                outcome_sequence(100, 0, 0, 0),
            )
            .expect("analysis must succeed");

        let point5 =
            LogicalDistancePoint::new(
                5,
                result5,
            )
            .expect("point must be valid");

        let mut sweep =
            LogicalBenchmarkSweep::new();

        sweep.push(point3)
            .expect("first point accepted");

        sweep.push(point5)
            .expect("higher distance accepted");

        assert_eq!(
            sweep.len(),
            2
        );
    }

    #[test]
    fn distance_sweep_rejects_non_monotonic_distance() {
        let config5 =
            LogicalBenchmarkConfig::new(5)
                .expect("valid config");

        let result5 =
            analyze_logical_outcomes(
                config5,
                outcome_sequence(100, 0, 0, 0),
            )
            .expect("analysis must succeed");

        let point5 =
            LogicalDistancePoint::new(
                5,
                result5,
            )
            .expect("point must be valid");

        let config3 =
            LogicalBenchmarkConfig::new(3)
                .expect("valid config");

        let result3 =
            analyze_logical_outcomes(
                config3,
                outcome_sequence(100, 0, 0, 0),
            )
            .expect("analysis must succeed");

        let point3 =
            LogicalDistancePoint::new(
                3,
                result3,
            )
            .expect("point must be valid");

        let mut sweep =
            LogicalBenchmarkSweep::new();

        sweep.push(point5)
            .expect("first point accepted");

        let result =
            sweep.push(point3);

        assert!(matches!(
            result,
            Err(
                LogicalBenchmarkError::NonMonotonicDistance {
                    previous: 5,
                    current: 3
                }
            )
        ));
    }

    #[test]
    fn suppression_ratio_detects_improvement() {
        let config3 =
            LogicalBenchmarkConfig::new(3)
                .expect("valid config");

        let result3 =
            analyze_logical_outcomes(
                config3,
                outcome_sequence(90, 10, 0, 0),
            )
            .expect("analysis must succeed");

        let point3 =
            LogicalDistancePoint::new(
                3,
                result3,
            )
            .expect("point must be valid");

        let config5 =
            LogicalBenchmarkConfig::new(5)
                .expect("valid config");

        let result5 =
            analyze_logical_outcomes(
                config5,
                outcome_sequence(99, 1, 0, 0),
            )
            .expect("analysis must succeed");

        let point5 =
            LogicalDistancePoint::new(
                5,
                result5,
            )
            .expect("point must be valid");

        let mut sweep =
            LogicalBenchmarkSweep::new();

        sweep.push(point3)
            .expect("first point");

        sweep.push(point5)
            .expect("second point");

        let ratio =
            sweep
                .suppression_ratio(0, 1)
                .expect("ratio must exist");

        assert!(
            (ratio - 10.0).abs()
                < 1.0e-12
        );
    }

    #[test]
    fn comparison_is_deterministic() {
        let config =
            LogicalBenchmarkConfig::new(3)
                .expect("valid config");

        let first =
            analyze_logical_outcomes(
                config,
                outcome_sequence(90, 10, 0, 0),
            )
            .expect("first analysis");

        let second =
            analyze_logical_outcomes(
                config,
                outcome_sequence(95, 5, 0, 0),
            )
            .expect("second analysis");

        let comparison =
            LogicalBenchmarkComparison::between(
                &first,
                &second,
            )
            .expect("comparison");

        assert!(
            comparison.absolute_difference
                > 0.0
        );

        assert_eq!(
            comparison.improvement_factor,
            Some(2.0)
        );
    }

    #[test]
    fn physical_error_ratio_is_reported() {
        let config =
            LogicalBenchmarkConfig::new(3)
                .expect("valid config")
                .with_physical_error_rate(0.2);

        let result =
            analyze_logical_outcomes(
                config,
                outcome_sequence(90, 10, 0, 0),
            )
            .expect("analysis");

        assert_eq!(
            result.logical_to_physical_error_ratio,
            Some(0.5)
        );

        assert_eq!(
            result.suppresses_physical_error(),
            Some(true)
        );
    }

    #[test]
    fn statistical_below_physical_error_uses_upper_bound() {
        let config =
            LogicalBenchmarkConfig::new(3)
                .expect("valid config");

        let result =
            analyze_logical_outcomes(
                config,
                outcome_sequence(1000, 0, 0, 0),
            )
            .expect("analysis");

        assert!(
            result
                .statistically_below(0.1)
                .expect("comparison")
        );
    }

    #[test]
    fn normal_inverse_cdf_two_sigma_is_close_to_two() {
        let value =
            standard_normal_inverse_cdf(
                0.977_249_868_051_820_8,
            )
            .expect("quantile");

        assert!(
            (value - 2.0).abs()
                < 1.0e-8
        );
    }

    #[test]
    fn logical_trial_helpers_are_correct() {
        assert!(
            LogicalTrial::new(
                LogicalOutcome::Identity,
            )
            .is_success()
        );

        assert!(
            LogicalTrial::new(
                LogicalOutcome::LogicalX,
            )
            .is_logical_failure()
        );

        assert!(
            LogicalTrial::new(
                LogicalOutcome::Unknown,
            )
            .is_unknown()
        );
    }

    #[test]
    fn result_schema_identity_is_stable() {
        let config =
            LogicalBenchmarkConfig::new(3)
                .expect("valid config");

        let result =
            analyze_logical_outcomes(
                config,
                outcome_sequence(100, 0, 0, 0),
            )
            .expect("analysis");

        assert_eq!(
            result.benchmark_id,
            LOGICAL_ERROR_BENCHMARK_ID
        );

        assert_eq!(
            result.schema_version,
            LOGICAL_ERROR_RESULT_SCHEMA_VERSION
        );
    }
}