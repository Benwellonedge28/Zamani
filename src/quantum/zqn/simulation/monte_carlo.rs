//! Zamani Quantum Noise (ZQN) — Production Monte Carlo Engine.
//!
//! `src/quantum/zqn/simulation/monte_carlo.rs`
//!
//! # Purpose
//!
//! This module owns generic Monte Carlo execution over a caller-supplied trial
//! evaluator.
//!
//! It provides:
//!
//! - deterministic indexed random streams;
//! - reproducible Monte Carlo execution;
//! - streaming accumulation;
//! - bounded execution;
//! - externally terminated execution;
//! - cancellation;
//! - explicit resource limits;
//! - numerically stable online statistics;
//! - optional confidence intervals;
//! - failure accounting;
//! - no materialization of all observations;
//! - deterministic execution independent of worker ordering;
//! - integration boundaries for simulation, QEC, characterization and
//!   benchmarking.
//!
//! This module deliberately does NOT implement quantum channels, probability
//! distributions, state evolution, QEC decoding, routing, scheduling,
//! calibration or hardware execution.
//!
//! # Ownership
//!
//! This file owns:
//!
//! - Monte Carlo execution policy;
//! - trial invocation;
//! - deterministic sample identity;
//! - per-sample RNG construction;
//! - online statistical accumulation;
//! - sample/failure accounting;
//! - explicit cancellation checks;
//! - explicit Monte Carlo resource limits;
//! - confidence-interval calculation from caller-supplied critical values;
//! - streaming execution contracts;
//! - Monte Carlo-specific errors.
//!
//! # Does not own
//!
//! This file does NOT own:
//!
//! - canonical quantum IR;
//! - `QubitId`;
//! - `PhysicalQubitId`;
//! - quantum channels;
//! - Kraus operators;
//! - Choi matrices;
//! - probability distributions;
//! - noise-model semantics;
//! - fault semantics;
//! - calibration;
//! - characterization protocols;
//! - QEC decoding;
//! - routing;
//! - scheduling;
//! - hardware APIs;
//! - vendor APIs;
//! - benchmark methodology;
//! - serialization schemas;
//! - global RNG state.
//!
//! Those responsibilities remain in their owning modules.
//!
//! # Canonical quantum identity
//!
//! Monte Carlo itself is a resource-independent mathematical execution
//! mechanism. It therefore does not require `QubitId`.
//!
//! When a caller associates a Monte Carlo experiment with quantum resources,
//! that higher-level caller MUST use:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! This file deliberately does not create another quantum-resource ID.
//!
//! # Architectural position
//!
//! ```text
//!                 quantum::ir
//!                      │
//!                      ▼
//!               noise / channel
//!                      │
//!                      ▼
//!               trial evaluator
//!                      │
//!                      ▼
//!        ┌──────────────────────────┐
//!        │ simulation::monte_carlo  │
//!        └────────────┬─────────────┘
//!                     │
//!          ┌──────────┼───────────┐
//!          ▼          ▼           ▼
//!       QEC       characterization benchmarking
//!          │          │           │
//!          └──────────┼───────────┘
//!                     ▼
//!                statistics
//! ```
//!
//! # Write once, scale everywhere
//!
//! There is deliberately NO semantic maximum for:
//!
//! - sample count;
//! - number of operations;
//! - number of quantum resources;
//! - circuit depth;
//! - number of machines;
//! - number of workers;
//! - number of observations.
//!
//! This file contains no:
//!
//! ```text
//! MAX_SAMPLES
//! MAX_SHOTS
//! MAX_QUBITS
//! MAX_OPERATIONS
//! MAX_WORKERS
//! ```
//!
//! An execution is limited only by:
//!
//! - caller-selected policy;
//! - runtime resource availability;
//! - explicit memory limits;
//! - cancellation;
//! - evaluator behavior;
//! - host/platform representational limits.
//!
//! The statistical accumulator itself uses O(1) memory with respect to sample
//! count.
//!
//! # Infinity semantics
//!
//! "Infinity" means that the Monte Carlo semantic model has no artificial
//! finite sample ceiling.
//!
//! It does NOT mean that a machine can execute infinitely many trials.
//!
//! For effectively unbounded execution, callers should use:
//!
//! ```text
//! run_until_cancelled()
//! ```
//!
//! or:
//!
//! ```text
//! run_stream()
//! ```
//!
//! and provide an external termination policy.
//!
//! # Determinism
//!
//! Determinism is a first-class property.
//!
//! Every trial receives an RNG derived from:
//!
//! ```text
//! master seed
//! execution identity
//! sample index
//! ```
//!
//! The RNG for sample N does not depend on samples 0..N-1.
//!
//! Therefore a deterministic execution can be partitioned across workers
//! without changing the random stream assigned to each logical sample.
//!
//! This is fundamentally different from sharing one mutable RNG across worker
//! threads.
//!
//! # Parallel determinism
//!
//! The semantic sample identity is the sample index.
//!
//! Therefore:
//!
//! ```text
//! sequential:
//!   sample 0
//!   sample 1
//!   sample 2
//!
//! parallel:
//!   worker A -> sample 0
//!   worker B -> sample 2
//!   worker C -> sample 1
//! ```
//!
//! produces the same per-sample random stream, assuming the same execution
//! identity and seed.
//!
//! The accumulation of floating-point statistics can still depend on reduction
//! order if callers manually combine partial accumulators. For strict
//! reproducibility, callers should combine partial accumulators in a stable
//! sample-index order.
//!
//! # Randomness boundary
//!
//! The default implementation uses an explicit `StdRng` constructed for each
//! logical sample.
//!
//! No global RNG exists.
//!
//! No thread-local RNG exists.
//!
//! No wall-clock entropy is used.
//!
//! No memory-address entropy is used.
//!
//! No hidden mutable randomness exists.
//!
//! The seed is explicit execution data.
//!
//! # RNG algorithm policy
//!
//! The exact RNG algorithm is an execution implementation detail rather than a
//! ZQN semantic contract.
//!
//! Scientific reproducibility records should therefore preserve the RNG policy
//! identity in the surrounding execution/provenance layer.
//!
//! This implementation uses the repository's existing `rand` dependency and
//! `StdRng`.
//!
//! # Numerical safety
//!
//! Trial values must be finite real numbers.
//!
//! This module rejects:
//!
//! - NaN;
//! - positive infinity;
//! - negative infinity.
//!
//! It never silently transforms:
//!
//! ```text
//! NaN -> 0
//! infinity -> finite
//! invalid -> valid
//! ```
//!
//! Such transformations would corrupt scientific results.
//!
//! # Statistical accumulator
//!
//! The accumulator uses an online Welford-style algorithm.
//!
//! It does not store all observations.
//!
//! Memory consumption is therefore independent of the number of samples:
//!
//! ```text
//! samples:      N
//! stored data:  O(1)
//! ```
//!
//! It provides:
//!
//! - count;
//! - mean;
//! - population second central moment;
//! - sample variance;
//! - standard deviation;
//! - standard error;
//! - minimum;
//! - maximum;
//! - optional confidence interval.
//!
//! # Statistical assumptions
//!
//! The Monte Carlo engine does NOT claim that every sequence of trials is
//! independent merely because it uses different RNG streams.
//!
//! Independence is a property of the supplied trial evaluator/model.
//!
//! The evaluator is responsible for defining the physical/statistical process.
//!
//! This engine only guarantees independent RNG stream construction according to
//! its deterministic seed derivation.
//!
//! # Confidence intervals
//!
//! Confidence intervals are intentionally not tied to a hard-coded confidence
//! level.
//!
//! The caller supplies a finite positive critical value, for example a normal
//! approximation's critical value.
//!
//! This prevents this low-level engine from silently assuming a statistical
//! model that may not be appropriate for the experiment.
//!
//! # Failure policy
//!
//! A trial may fail.
//!
//! The caller must explicitly choose one of:
//!
//! - fail immediately;
//! - record the failure and continue.
//!
//! Failed trials are NOT silently converted into numerical observations.
//!
//! # Resource policy
//!
//! Resource limits are caller/runtime policy, not semantic limits.
//!
//! The engine supports:
//!
//! - maximum successful samples;
//! - maximum attempted samples;
//! - maximum failures;
//! - periodic cancellation checks.
//!
//! No default maximum sample count is imposed.
//!
//! # Cancellation
//!
//! Cancellation is represented by a caller-supplied trait.
//!
//! This allows integration with:
//!
//! - runtime cancellation;
//! - user interruption;
//! - distributed job cancellation;
//! - scheduler cancellation;
//! - timeout controllers;
//! - resource managers.
//!
//! The engine does not own the cancellation mechanism.
//!
//! # Streaming
//!
//! The preferred interface for enormous executions is streaming execution.
//!
//! The engine can execute a supplied iterator of sample indices without
//! materializing all results.
//!
//! This makes it possible to process:
//!
//! - millions of trials;
//! - billions of trials;
//! - distributed trial ranges;
//! - externally generated sample IDs;
//! - effectively unbounded streams,
//!
//! subject to available resources and the caller's termination policy.
//!
//! # Integration with sampler.rs
//!
//! `simulation::sampler` owns probability-distribution sampling.
//!
//! Monte Carlo owns repeated trial execution.
//!
//! The relationship is:
//!
//! ```text
//! NoiseModel
//!     │
//!     ▼
//! Distribution<T>
//!     │
//!     ▼
//! sampler
//!     │
//!     ▼
//! trial evaluator
//!     │
//!     ▼
//! monte_carlo
//! ```
//!
//! Monte Carlo must not duplicate `Distribution<T>` semantics.
//!
//! # Integration with channel_engine.rs
//!
//! `simulation::channel_engine` owns deterministic application of channels to
//! state representations.
//!
//! Monte Carlo may invoke channel evolution repeatedly through a trial
//! evaluator:
//!
//! ```text
//! Monte Carlo
//!      │
//!      ▼
//! trial
//!      │
//!      ▼
//! ChannelEngine
//!      │
//!      ▼
//! observable
//! ```
//!
//! The channel engine remains responsible for channel application.
//!
//! # Integration with trajectory.rs
//!
//! A trajectory engine may use Monte Carlo to aggregate many stochastic
//! trajectories.
//!
//! Monte Carlo does not define trajectory physics.
//!
//! # Integration with deterministic.rs
//!
//! Deterministic simulation and Monte Carlo are complementary:
//!
//! deterministic.rs
//!     exact/controlled evolution
//!
//! monte_carlo.rs
//!     repeated stochastic execution
//!
//! They must not duplicate one another.
//!
//! # Integration with QEC
//!
//! QEC may provide a trial evaluator that:
//!
//! 1. constructs a noisy execution;
//! 2. generates faults;
//! 3. executes the fault-tolerant circuit;
//! 4. decodes the result;
//! 5. returns a numerical observable such as logical failure = 0/1.
//!
//! Monte Carlo then estimates the expectation of that observable.
//!
//! Monte Carlo does not perform syndrome decoding.
//!
//! # Integration with characterization
//!
//! Characterization may use Monte Carlo to estimate quantities from repeated
//! noisy experiments.
//!
//! Protocol definition remains in `characterization`.
//!
//! # Integration with benchmarking
//!
//! Benchmarking may use this module for repeated stochastic executions.
//!
//! Benchmark methodology remains outside this file.
//!
//! # Integration with calibration
//!
//! Calibration values can be captured by the trial evaluator or execution
//! context.
//!
//! Monte Carlo does not own calibration state.
//!
//! # Integration with runtime
//!
//! The runtime should supply:
//!
//! - master seed;
//! - execution identity;
//! - cancellation;
//! - resource policy;
//! - target information;
//! - calibration identity;
//! - worker allocation.
//!
//! Monte Carlo supplies:
//!
//! - reproducible trial execution;
//! - online statistics;
//! - explicit failure accounting.
//!
//! # Integration with canonical quantum IR
//!
//! This file does not modify the canonical quantum IR.
//!
//! A trial evaluator may receive IR operations and use the canonical:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! ```
//!
//! where resource identity is required.
//!
//! Monte Carlo itself remains generic over the observable type's conversion to
//! `f64`.
//!
//! # Serialization
//!
//! The Monte Carlo accumulator is intentionally represented as ordinary public
//! values and can be serialized by a higher-level ZQN schema.
//!
//! RNG implementation state is NOT a wire-format contract.
//!
//! A reproducible execution record should additionally preserve:
//!
//! - ZQN version;
//! - Monte Carlo policy version;
//! - master seed;
//! - execution identity;
//! - RNG policy identity;
//! - noise-model identity;
//! - target identity;
//! - calibration identity;
//! - sample range;
//! - resource policy.
//!
//! Those belong to the surrounding provenance/serialization layers.
//!
//! # Security
//!
//! `StdRng` is a pseudorandom generator, not a cryptographic RNG.
//!
//! This module MUST NOT be used for:
//!
//! - cryptographic keys;
//! - authentication tokens;
//! - secrets;
//! - cryptographic nonces.
//!
//! # Thread safety
//!
//! `MonteCarloEngine` is immutable and contains only configuration.
//!
//! It can therefore be shared between workers when its configuration is shared.
//!
//! The trial evaluator itself is owned by the caller and must obey its own
//! thread-safety requirements.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! =============================================================================
//! Implementation
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;

use rand::rngs::StdRng;
use rand::SeedableRng;

// =============================================================================
// Public version
// =============================================================================

/// Version of the Monte Carlo execution contract.
///
/// This is a semantic/protocol version and is not a resource limit.
pub const MONTE_CARLO_MODEL_VERSION: u16 = 1;

// =============================================================================
// Cancellation
// =============================================================================

/// Caller-owned cancellation source.
///
/// Implementations should be cheap because the engine may call this method
/// between trials.
pub trait CancellationToken {
    /// Returns `true` when execution should stop.
    fn is_cancelled(&self) -> bool;
}

/// A cancellation token that never requests cancellation.
#[derive(Debug, Clone, Copy, Default)]
pub struct NeverCancel;

impl CancellationToken for NeverCancel {
    #[inline]
    fn is_cancelled(&self) -> bool {
        false
    }
}

// =============================================================================
// Sample identity
// =============================================================================

/// Logical identity of a Monte Carlo trial.
///
/// This is not a qubit identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SampleIndex(u128);

impl SampleIndex {
    /// Creates a sample index.
    #[must_use]
    pub const fn new(value: u128) -> Self {
        Self(value)
    }

    /// Returns the underlying logical sample number.
    #[must_use]
    pub const fn get(self) -> u128 {
        self.0
    }
}

impl From<u64> for SampleIndex {
    fn from(value: u64) -> Self {
        Self(value as u128)
    }
}

// =============================================================================
// Execution identity
// =============================================================================

/// Identity of one Monte Carlo execution stream.
///
/// This value is deliberately opaque to the statistical engine.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExecutionId(u128);

impl ExecutionId {
    /// Creates an execution identity from a caller-defined stable value.
    #[must_use]
    pub const fn from_u128(value: u128) -> Self {
        Self(value)
    }

    /// Returns the underlying value.
    #[must_use]
    pub const fn as_u128(self) -> u128 {
        self.0
    }
}

// =============================================================================
// Seed
// =============================================================================

/// Master seed used for deterministic Monte Carlo execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MonteCarloSeed(u64);

impl MonteCarloSeed {
    /// Creates a seed.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the numeric seed.
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }
}

// =============================================================================
// Failure policy
// =============================================================================

/// Determines what happens when a trial evaluator returns an error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FailurePolicy {
    /// Stop immediately on the first failed trial.
    FailFast,

    /// Record the failure and continue.
    Continue,
}

impl Default for FailurePolicy {
    fn default() -> Self {
        Self::FailFast
    }
}

// =============================================================================
// Resource limits
// =============================================================================

/// Explicit runtime limits for Monte Carlo execution.
///
/// `None` means that this particular policy does not impose a limit.
///
/// These are execution-policy limits, NOT semantic limits on Zamani or quantum
/// machines.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MonteCarloLimits {
    /// Maximum number of attempted trials.
    pub max_attempts: Option<u128>,

    /// Maximum number of successful observations.
    pub max_successes: Option<u128>,

    /// Maximum number of failed trials when using `FailurePolicy::Continue`.
    pub max_failures: Option<u128>,
}

impl Default for MonteCarloLimits {
    fn default() -> Self {
        Self {
            max_attempts: None,
            max_successes: None,
            max_failures: None,
        }
    }
}

// =============================================================================
// Configuration
// =============================================================================

/// Configuration for a Monte Carlo execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MonteCarloConfig {
    /// Execution identity used in deterministic seed derivation.
    pub execution_id: ExecutionId,

    /// Master random seed.
    pub seed: MonteCarloSeed,

    /// Resource limits.
    pub limits: MonteCarloLimits,

    /// Trial failure policy.
    pub failure_policy: FailurePolicy,
}

impl MonteCarloConfig {
    /// Creates a configuration with unlimited execution-policy limits.
    #[must_use]
    pub const fn new(execution_id: ExecutionId, seed: MonteCarloSeed) -> Self {
        Self {
            execution_id,
            seed,
            limits: MonteCarloLimits {
                max_attempts: None,
                max_successes: None,
                max_failures: None,
            },
            failure_policy: FailurePolicy::FailFast,
        }
    }

    /// Sets execution limits.
    #[must_use]
    pub const fn with_limits(mut self, limits: MonteCarloLimits) -> Self {
        self.limits = limits;
        self
    }

    /// Sets failure policy.
    #[must_use]
    pub const fn with_failure_policy(mut self, policy: FailurePolicy) -> Self {
        self.failure_policy = policy;
        self
    }
}

impl Default for MonteCarloConfig {
    fn default() -> Self {
        Self::new(
            ExecutionId::from_u128(0),
            MonteCarloSeed::new(0),
        )
    }
}

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by the Monte Carlo engine.
#[derive(Debug, Clone, PartialEq)]
pub enum MonteCarloError<E> {
    /// The requested sample count is zero.
    ZeroSamplesRequested,

    /// The requested sample count violates the explicit attempt limit.
    AttemptLimitExceeded {
        requested: u128,
        limit: u128,
    },

    /// The requested successful-sample count violates the explicit success
    /// limit.
    SuccessLimitExceeded {
        requested: u128,
        limit: u128,
    },

    /// Execution was cancelled.
    Cancelled,

    /// The trial evaluator failed.
    Trial {
        sample: SampleIndex,
        error: E,
    },

    /// The trial evaluator returned NaN or infinity.
    NonFiniteObservation {
        sample: SampleIndex,
        value: f64,
    },

    /// The internal sample counter would overflow.
    CounterOverflow,

    /// The confidence critical value is invalid.
    InvalidCriticalValue {
        value: f64,
    },

    /// A confidence interval could not be computed because there are not enough
    /// observations.
    InsufficientSamplesForInterval,

    /// The configured failure limit was exceeded.
    FailureLimitExceeded {
        failures: u128,
        limit: u128,
    },
}

impl<E: fmt::Display> fmt::Display for MonteCarloError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ZeroSamplesRequested => {
                write!(f, "zero Monte Carlo samples were requested")
            }

            Self::AttemptLimitExceeded { requested, limit } => write!(
                f,
                "requested {} Monte Carlo attempts but the execution limit is {}",
                requested, limit
            ),

            Self::SuccessLimitExceeded { requested, limit } => write!(
                f,
                "requested {} successful Monte Carlo samples but the execution limit is {}",
                requested, limit
            ),

            Self::Cancelled => write!(f, "Monte Carlo execution was cancelled"),

            Self::Trial { sample, error } => write!(
                f,
                "Monte Carlo trial {} failed: {}",
                sample.get(),
                error
            ),

            Self::NonFiniteObservation { sample, value } => write!(
                f,
                "Monte Carlo trial {} produced a non-finite observation: {}",
                sample.get(),
                value
            ),

            Self::CounterOverflow => {
                write!(f, "Monte Carlo execution counter overflowed")
            }

            Self::InvalidCriticalValue { value } => write!(
                f,
                "invalid confidence-interval critical value: {}",
                value
            ),

            Self::InsufficientSamplesForInterval => {
                write!(f, "at least two observations are required for the requested interval")
            }

            Self::FailureLimitExceeded { failures, limit } => write!(
                f,
                "Monte Carlo failure count {} exceeded the configured limit {}",
                failures,
                limit
            ),
        }
    }
}

impl<E: fmt::Debug + fmt::Display> std::error::Error for MonteCarloError<E> {}

// =============================================================================
// Online statistics
// =============================================================================

/// Numerically stable online statistics accumulator.
///
/// The accumulator stores O(1) state regardless of the number of observations.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OnlineStatistics {
    count: u128,
    mean: f64,
    m2: f64,
    minimum: Option<f64>,
    maximum: Option<f64>,
}

impl Default for OnlineStatistics {
    fn default() -> Self {
        Self::new()
    }
}

impl OnlineStatistics {
    /// Creates an empty accumulator.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            count: 0,
            mean: 0.0,
            m2: 0.0,
            minimum: None,
            maximum: None,
        }
    }

    /// Adds one finite observation.
    ///
    /// This method rejects non-finite values rather than repairing them.
    pub fn observe(&mut self, value: f64) -> Result<(), StatisticsError> {
        if !value.is_finite() {
            return Err(StatisticsError::NonFiniteObservation { value });
        }

        self.count = self
            .count
            .checked_add(1)
            .ok_or(StatisticsError::CountOverflow)?;

        if self.minimum.map_or(true, |minimum| value < minimum) {
            self.minimum = Some(value);
        }

        if self.maximum.map_or(true, |maximum| value > maximum) {
            self.maximum = Some(value);
        }

        let count = self.count as f64;
        let delta = value - self.mean;

        self.mean += delta / count;

        let delta_after = value - self.mean;
        self.m2 += delta * delta_after;

        // Rounding may produce a tiny negative value for pathological inputs.
        // A negative second central moment is mathematically impossible.
        // We therefore reject rather than silently clamp it.
        if self.m2 < 0.0 {
            return Err(StatisticsError::NumericalFailure);
        }

        Ok(())
    }

    /// Number of observations.
    #[must_use]
    pub const fn count(&self) -> u128 {
        self.count
    }

    /// Returns the sample mean.
    #[must_use]
    pub const fn mean(&self) -> Option<f64> {
        if self.count == 0 {
            None
        } else {
            Some(self.mean)
        }
    }

    /// Returns the population variance.
    #[must_use]
    pub fn population_variance(&self) -> Option<f64> {
        if self.count == 0 {
            None
        } else {
            Some(self.m2 / self.count as f64)
        }
    }

    /// Returns the unbiased sample variance.
    #[must_use]
    pub fn sample_variance(&self) -> Option<f64> {
        if self.count < 2 {
            None
        } else {
            Some(self.m2 / (self.count - 1) as f64)
        }
    }

    /// Returns the population standard deviation.
    #[must_use]
    pub fn population_standard_deviation(&self) -> Option<f64> {
        self.population_variance().map(f64::sqrt)
    }

    /// Returns the sample standard deviation.
    #[must_use]
    pub fn sample_standard_deviation(&self) -> Option<f64> {
        self.sample_variance().map(f64::sqrt)
    }

    /// Returns the standard error of the sample mean.
    #[must_use]
    pub fn standard_error(&self) -> Option<f64> {
        match self.sample_variance() {
            Some(variance) => Some((variance / self.count as f64).sqrt()),
            None => None,
        }
    }

    /// Returns the minimum observation.
    #[must_use]
    pub const fn minimum(&self) -> Option<f64> {
        self.minimum
    }

    /// Returns the maximum observation.
    #[must_use]
    pub const fn maximum(&self) -> Option<f64> {
        self.maximum
    }

    /// Returns a confidence interval around the mean using a caller-supplied
    /// critical value.
    ///
    /// For example, a caller using a normal approximation can supply the
    /// appropriate normal critical value.
    ///
    /// This method does not decide which statistical model is scientifically
    /// appropriate.
    pub fn confidence_interval(
        &self,
        critical_value: f64,
    ) -> Result<ConfidenceInterval, StatisticsError> {
        if !critical_value.is_finite() || critical_value <= 0.0 {
            return Err(StatisticsError::InvalidCriticalValue {
                value: critical_value,
            });
        }

        let mean = self
            .mean()
            .ok_or(StatisticsError::InsufficientSamples)?;

        let standard_error = self
            .standard_error()
            .ok_or(StatisticsError::InsufficientSamples)?;

        let margin = critical_value * standard_error;

        if !margin.is_finite() {
            return Err(StatisticsError::NumericalFailure);
        }

        Ok(ConfidenceInterval {
            estimate: mean,
            lower: mean - margin,
            upper: mean + margin,
            critical_value,
        })
    }

    /// Combines another accumulator into this one.
    ///
    /// This permits distributed Monte Carlo execution without storing every
    /// observation.
    ///
    /// The caller controls reduction ordering when strict floating-point
    /// reproducibility is required.
    pub fn merge(&mut self, other: &Self) -> Result<(), StatisticsError> {
        if other.count == 0 {
            return Ok(());
        }

        if self.count == 0 {
            *self = *other;
            return Ok(());
        }

        let combined_count = self
            .count
            .checked_add(other.count)
            .ok_or(StatisticsError::CountOverflow)?;

        let left_count = self.count as f64;
        let right_count = other.count as f64;
        let total_count = combined_count as f64;

        let delta = other.mean - self.mean;

        self.mean += delta * (right_count / total_count);

        self.m2 += other.m2
            + delta * delta * (left_count * right_count / total_count);

        self.count = combined_count;

        self.minimum = match (self.minimum, other.minimum) {
            (Some(a), Some(b)) => Some(a.min(b)),
            (None, Some(b)) => Some(b),
            (Some(a), None) => Some(a),
            (None, None) => None,
        };

        self.maximum = match (self.maximum, other.maximum) {
            (Some(a), Some(b)) => Some(a.max(b)),
            (None, Some(b)) => Some(b),
            (Some(a), None) => Some(a),
            (None, None) => None,
        };

        if !self.mean.is_finite() || !self.m2.is_finite() || self.m2 < 0.0 {
            return Err(StatisticsError::NumericalFailure);
        }

        Ok(())
    }
}

/// Errors produced by the online statistics accumulator.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StatisticsError {
    /// An observation was NaN or infinite.
    NonFiniteObservation { value: f64 },

    /// The observation count overflowed.
    CountOverflow,

    /// Floating-point arithmetic produced an invalid state.
    NumericalFailure,

    /// A statistical interval was requested without enough observations.
    InsufficientSamples,

    /// The supplied critical value is invalid.
    InvalidCriticalValue { value: f64 },
}

impl fmt::Display for StatisticsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFiniteObservation { value } => {
                write!(f, "non-finite observation: {}", value)
            }

            Self::CountOverflow => {
                write!(f, "observation count overflowed")
            }

            Self::NumericalFailure => {
                write!(f, "online statistical accumulation became numerically invalid")
            }

            Self::InsufficientSamples => {
                write!(f, "insufficient observations for the requested statistic")
            }

            Self::InvalidCriticalValue { value } => {
                write!(f, "invalid critical value: {}", value)
            }
        }
    }
}

impl std::error::Error for StatisticsError {}

/// A confidence interval around an estimated mean.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ConfidenceInterval {
    /// Point estimate.
    pub estimate: f64,

    /// Lower interval boundary.
    pub lower: f64,

    /// Upper interval boundary.
    pub upper: f64,

    /// Caller-supplied critical value.
    pub critical_value: f64,
}

// =============================================================================
// Execution result
// =============================================================================

/// Result of a Monte Carlo execution.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MonteCarloResult {
    /// Number of evaluator invocations.
    pub attempts: u128,

    /// Number of successful observations.
    pub successes: u128,

    /// Number of failed evaluator invocations.
    pub failures: u128,

    /// Online statistics over successful observations.
    pub statistics: OnlineStatistics,

    /// True when execution stopped because the caller cancelled it.
    pub cancelled: bool,
}

impl MonteCarloResult {
    /// Returns the estimated mean.
    #[must_use]
    pub fn mean(&self) -> Option<f64> {
        self.statistics.mean()
    }

    /// Returns the standard error of the mean.
    #[must_use]
    pub fn standard_error(&self) -> Option<f64> {
        self.statistics.standard_error()
    }

    /// Returns a confidence interval using a caller-supplied critical value.
    pub fn confidence_interval(
        &self,
        critical_value: f64,
    ) -> Result<ConfidenceInterval, StatisticsError> {
        self.statistics.confidence_interval(critical_value)
    }
}

// =============================================================================
// Trial RNG
// =============================================================================

/// Constructs a deterministic RNG for one logical sample.
///
/// The derivation deliberately does not depend on worker identity or execution
/// order.
///
/// `splitmix64` is used only as a deterministic seed-mixing function. It is not
/// used as a cryptographic primitive.
fn rng_for_sample(
    seed: MonteCarloSeed,
    execution_id: ExecutionId,
    sample: SampleIndex,
) -> StdRng {
    let mut x = seed.get();

    x ^= fold_u128(execution_id.as_u128());
    x = splitmix64(x);

    x ^= fold_u128(sample.get());
    x = splitmix64(x);

    // A single 64-bit deterministic value is expanded into the seed accepted
    // by StdRng. The exact RNG algorithm remains an implementation detail.
    StdRng::seed_from_u64(x)
}

/// Folds a 128-bit value into a deterministic 64-bit value.
#[inline]
fn fold_u128(value: u128) -> u64 {
    let low = value as u64;
    let high = (value >> 64) as u64;

    low ^ high.rotate_left(29)
}

/// SplitMix64 mixing function.
///
/// This is deterministic integer mixing, not cryptographic hashing.
#[inline]
fn splitmix64(mut value: u64) -> u64 {
    value = value.wrapping_add(0x9E37_79B9_7F4A_7C15);

    let mut z = value;

    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);

    z ^ (z >> 31)
}

// =============================================================================
// Trial evaluator
// =============================================================================

/// Trait implemented by a Monte Carlo trial evaluator.
///
/// The evaluator receives:
///
/// - logical sample identity;
/// - deterministic RNG for that sample.
///
/// The evaluator returns one finite scalar observable.
///
/// The scalar may represent, for example:
///
/// - logical failure indicator;
/// - energy;
/// - expectation contribution;
/// - fidelity contribution;
/// - error count;
/// - measurement statistic;
/// - characterization observable.
///
/// Quantum semantics remain owned by the caller.
pub trait TrialEvaluator {
    /// Error type returned by the trial.
    type Error;

    /// Executes one logical Monte Carlo trial.
    fn evaluate(
        &mut self,
        sample: SampleIndex,
        rng: &mut StdRng,
    ) -> Result<f64, Self::Error>;
}

/// Blanket implementation for closures.
///
/// This permits simple integration without creating a dedicated evaluator type.
impl<F, E> TrialEvaluator for F
where
    F: FnMut(SampleIndex, &mut StdRng) -> Result<f64, E>,
{
    type Error = E;

    fn evaluate(
        &mut self,
        sample: SampleIndex,
        rng: &mut StdRng,
    ) -> Result<f64, Self::Error> {
        self(sample, rng)
    }
}

// =============================================================================
// Engine
// =============================================================================

/// Production Monte Carlo execution engine.
///
/// The engine contains immutable execution configuration and can therefore be
/// shared between workers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MonteCarloEngine {
    config: MonteCarloConfig,
}

impl MonteCarloEngine {
    /// Creates a Monte Carlo engine.
    #[must_use]
    pub const fn new(config: MonteCarloConfig) -> Self {
        Self { config }
    }

    /// Returns the engine configuration.
    #[must_use]
    pub const fn config(&self) -> MonteCarloConfig {
        self.config
    }

    /// Executes exactly `samples` successful trials unless a failure, limit or
    /// cancellation stops execution.
    ///
    /// Sample indices start at zero.
    pub fn run<Evaluator, Cancel>(
        &self,
        samples: u128,
        evaluator: &mut Evaluator,
        cancellation: &Cancel,
    ) -> Result<MonteCarloResult, MonteCarloError<Evaluator::Error>>
    where
        Evaluator: TrialEvaluator,
        Cancel: CancellationToken,
    {
        if samples == 0 {
            return Err(MonteCarloError::ZeroSamplesRequested);
        }

        if let Some(limit) = self.config.limits.max_successes {
            if samples > limit {
                return Err(MonteCarloError::SuccessLimitExceeded {
                    requested: samples,
                    limit,
                });
            }
        }

        if let Some(limit) = self.config.limits.max_attempts {
            if samples > limit {
                return Err(MonteCarloError::AttemptLimitExceeded {
                    requested: samples,
                    limit,
                });
            }
        }

        let indices = 0u128..samples;

        self.run_indices(indices, evaluator, cancellation)
    }

    /// Executes an externally supplied stream of logical sample indices.
    ///
    /// This is the primary API for distributed and effectively unbounded
    /// execution.
    pub fn run_indices<I, Evaluator, Cancel>(
        &self,
        indices: I,
        evaluator: &mut Evaluator,
        cancellation: &Cancel,
    ) -> Result<MonteCarloResult, MonteCarloError<Evaluator::Error>>
    where
        I: IntoIterator<Item = u128>,
        Evaluator: TrialEvaluator,
        Cancel: CancellationToken,
    {
        let mut result = MonteCarloResult {
            attempts: 0,
            successes: 0,
            failures: 0,
            statistics: OnlineStatistics::new(),
            cancelled: false,
        };

        for raw_index in indices {
            if cancellation.is_cancelled() {
                result.cancelled = true;
                return Ok(result);
            }

            let sample = SampleIndex::new(raw_index);

            if let Some(limit) = self.config.limits.max_attempts {
                if result.attempts >= limit {
                    return Err(MonteCarloError::AttemptLimitExceeded {
                        requested: result.attempts.saturating_add(1),
                        limit,
                    });
                }
            }

            result.attempts = result
                .attempts
                .checked_add(1)
                .ok_or(MonteCarloError::CounterOverflow)?;

            let mut rng = rng_for_sample(
                self.config.seed,
                self.config.execution_id,
                sample,
            );

            match evaluator.evaluate(sample, &mut rng) {
                Ok(value) => {
                    if !value.is_finite() {
                        return Err(MonteCarloError::NonFiniteObservation {
                            sample,
                            value,
                        });
                    }

                    result
                        .statistics
                        .observe(value)
                        .map_err(|_| MonteCarloError::NonFiniteObservation {
                            sample,
                            value,
                        })?;

                    result.successes = result
                        .successes
                        .checked_add(1)
                        .ok_or(MonteCarloError::CounterOverflow)?;

                    if let Some(limit) = self.config.limits.max_successes {
                        if result.successes >= limit {
                            return Ok(result);
                        }
                    }
                }

                Err(error) => {
                    result.failures = result
                        .failures
                        .checked_add(1)
                        .ok_or(MonteCarloError::CounterOverflow)?;

                    match self.config.failure_policy {
                        FailurePolicy::FailFast => {
                            return Err(MonteCarloError::Trial {
                                sample,
                                error,
                            });
                        }

                        FailurePolicy::Continue => {
                            if let Some(limit) = self.config.limits.max_failures {
                                if result.failures > limit {
                                    return Err(
                                        MonteCarloError::FailureLimitExceeded {
                                            failures: result.failures,
                                            limit,
                                        },
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(result)
    }

    /// Executes trials indefinitely until the caller cancels execution or the
    /// configured resource policy stops it.
    ///
    /// This method is intentionally finite in physical execution: cancellation
    /// or an explicit limit must eventually determine termination.
    pub fn run_until_cancelled<Evaluator, Cancel>(
        &self,
        evaluator: &mut Evaluator,
        cancellation: &Cancel,
    ) -> Result<MonteCarloResult, MonteCarloError<Evaluator::Error>>
    where
        Evaluator: TrialEvaluator,
        Cancel: CancellationToken,
    {
        let indices = 0u128..;
        self.run_indices(indices, evaluator, cancellation)
    }

    /// Executes trials and invokes a callback after every successful
    /// observation.
    ///
    /// The callback can be used to:
    ///
    /// - stream observations to storage;
    /// - update an external monitor;
    /// - feed a benchmark pipeline;
    /// - transmit results to another process.
    ///
    /// The callback is not allowed to alter the Monte Carlo statistics.
    pub fn run_stream<Evaluator, Cancel, Sink>(
        &self,
        samples: u128,
        evaluator: &mut Evaluator,
        cancellation: &Cancel,
        mut sink: Sink,
    ) -> Result<MonteCarloResult, MonteCarloError<Evaluator::Error>>
    where
        Evaluator: TrialEvaluator,
        Cancel: CancellationToken,
        Sink: FnMut(SampleIndex, f64),
    {
        if samples == 0 {
            return Err(MonteCarloError::ZeroSamplesRequested);
        }

        if let Some(limit) = self.config.limits.max_successes {
            if samples > limit {
                return Err(MonteCarloError::SuccessLimitExceeded {
                    requested: samples,
                    limit,
                });
            }
        }

        if let Some(limit) = self.config.limits.max_attempts {
            if samples > limit {
                return Err(MonteCarloError::AttemptLimitExceeded {
                    requested: samples,
                    limit,
                });
            }
        }

        let mut result = MonteCarloResult {
            attempts: 0,
            successes: 0,
            failures: 0,
            statistics: OnlineStatistics::new(),
            cancelled: false,
        };

        for raw_index in 0u128..samples {
            if cancellation.is_cancelled() {
                result.cancelled = true;
                return Ok(result);
            }

            let sample = SampleIndex::new(raw_index);

            result.attempts = result
                .attempts
                .checked_add(1)
                .ok_or(MonteCarloError::CounterOverflow)?;

            let mut rng = rng_for_sample(
                self.config.seed,
                self.config.execution_id,
                sample,
            );

            match evaluator.evaluate(sample, &mut rng) {
                Ok(value) => {
                    if !value.is_finite() {
                        return Err(MonteCarloError::NonFiniteObservation {
                            sample,
                            value,
                        });
                    }

                    result
                        .statistics
                        .observe(value)
                        .map_err(|_| MonteCarloError::NonFiniteObservation {
                            sample,
                            value,
                        })?;

                    result.successes = result
                        .successes
                        .checked_add(1)
                        .ok_or(MonteCarloError::CounterOverflow)?;

                    sink(sample, value);
                }

                Err(error) => {
                    result.failures = result
                        .failures
                        .checked_add(1)
                        .ok_or(MonteCarloError::CounterOverflow)?;

                    match self.config.failure_policy {
                        FailurePolicy::FailFast => {
                            return Err(MonteCarloError::Trial {
                                sample,
                                error,
                            });
                        }

                        FailurePolicy::Continue => {
                            if let Some(limit) = self.config.limits.max_failures {
                                if result.failures > limit {
                                    return Err(
                                        MonteCarloError::FailureLimitExceeded {
                                            failures: result.failures,
                                            limit,
                                        },
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(result)
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_index_is_stable() {
        let index = SampleIndex::new(123);
        assert_eq!(index.get(), 123);
    }

    #[test]
    fn deterministic_rng_for_same_sample() {
        let mut a = rng_for_sample(
            MonteCarloSeed::new(42),
            ExecutionId::from_u128(7),
            SampleIndex::new(100),
        );

        let mut b = rng_for_sample(
            MonteCarloSeed::new(42),
            ExecutionId::from_u128(7),
            SampleIndex::new(100),
        );

        use rand::RngCore;

        assert_eq!(a.next_u64(), b.next_u64());
        assert_eq!(a.next_u64(), b.next_u64());
        assert_eq!(a.next_u64(), b.next_u64());
    }

    #[test]
    fn different_samples_have_independent_seed_streams() {
        let mut a = rng_for_sample(
            MonteCarloSeed::new(42),
            ExecutionId::from_u128(7),
            SampleIndex::new(1),
        );

        let mut b = rng_for_sample(
            MonteCarloSeed::new(42),
            ExecutionId::from_u128(7),
            SampleIndex::new(2),
        );

        use rand::RngCore;

        assert_ne!(a.next_u64(), b.next_u64());
    }

    #[test]
    fn online_statistics_are_constant_memory() {
        let mut statistics = OnlineStatistics::new();

        statistics.observe(1.0).unwrap();
        statistics.observe(2.0).unwrap();
        statistics.observe(3.0).unwrap();

        assert_eq!(statistics.count(), 3);
        assert_eq!(statistics.mean(), Some(2.0));

        let variance = statistics.sample_variance().unwrap();
        assert!((variance - 1.0).abs() < 1.0e-12);
    }

    #[test]
    fn online_statistics_reject_non_finite_values() {
        let mut statistics = OnlineStatistics::new();

        assert!(statistics.observe(f64::NAN).is_err());
        assert!(statistics.observe(f64::INFINITY).is_err());
        assert!(statistics.observe(f64::NEG_INFINITY).is_err());
    }

    #[test]
    fn statistics_track_extrema() {
        let mut statistics = OnlineStatistics::new();

        statistics.observe(4.0).unwrap();
        statistics.observe(-2.0).unwrap();
        statistics.observe(9.0).unwrap();

        assert_eq!(statistics.minimum(), Some(-2.0));
        assert_eq!(statistics.maximum(), Some(9.0));
    }

    #[test]
    fn confidence_interval_requires_enough_samples() {
        let statistics = OnlineStatistics::new();

        assert!(statistics.confidence_interval(1.96).is_err());
    }

    #[test]
    fn confidence_interval_rejects_invalid_critical_value() {
        let mut statistics = OnlineStatistics::new();

        statistics.observe(1.0).unwrap();
        statistics.observe(2.0).unwrap();

        assert!(statistics.confidence_interval(0.0).is_err());
        assert!(statistics.confidence_interval(-1.0).is_err());
        assert!(statistics.confidence_interval(f64::NAN).is_err());
    }

    #[test]
    fn deterministic_monte_carlo_execution() {
        let config = MonteCarloConfig::new(
            ExecutionId::from_u128(1),
            MonteCarloSeed::new(1234),
        );

        let engine = MonteCarloEngine::new(config);

        let mut evaluator_a =
            |_sample: SampleIndex, rng: &mut StdRng| -> Result<f64, ()> {
                use rand::Rng;
                Ok(rng.gen::<f64>())
            };

        let mut evaluator_b =
            |_sample: SampleIndex, rng: &mut StdRng| -> Result<f64, ()> {
                use rand::Rng;
                Ok(rng.gen::<f64>())
            };

        let a = engine
            .run(1000, &mut evaluator_a, &NeverCancel)
            .unwrap();

        let b = engine
            .run(1000, &mut evaluator_b, &NeverCancel)
            .unwrap();

        assert_eq!(a.attempts, b.attempts);
        assert_eq!(a.successes, b.successes);
        assert_eq!(a.failures, b.failures);
        assert_eq!(a.statistics, b.statistics);
    }

    #[test]
    fn execution_can_run_without_materializing_observations() {
        let config = MonteCarloConfig::new(
            ExecutionId::from_u128(10),
            MonteCarloSeed::new(99),
        );

        let engine = MonteCarloEngine::new(config);

        let mut evaluator =
            |_sample: SampleIndex, _rng: &mut StdRng| -> Result<f64, ()> {
                Ok(1.0)
            };

        let result = engine
            .run(100_000, &mut evaluator, &NeverCancel)
            .unwrap();

        assert_eq!(result.successes, 100_000);
        assert_eq!(result.statistics.mean(), Some(1.0));
    }

    #[test]
    fn cancellation_stops_execution() {
        struct CancelAfter {
            calls: std::cell::Cell<u32>,
            limit: u32,
        }

        impl CancellationToken for CancelAfter {
            fn is_cancelled(&self) -> bool {
                let current = self.calls.get();
                self.calls.set(current + 1);
                current >= self.limit
            }
        }

        let config = MonteCarloConfig::new(
            ExecutionId::from_u128(1),
            MonteCarloSeed::new(1),
        );

        let engine = MonteCarloEngine::new(config);

        let mut evaluator =
            |_sample: SampleIndex, _rng: &mut StdRng| -> Result<f64, ()> {
                Ok(1.0)
            };

        let cancellation = CancelAfter {
            calls: std::cell::Cell::new(0),
            limit: 10,
        };

        let result = engine
            .run(1000, &mut evaluator, &cancellation)
            .unwrap();

        assert!(result.cancelled);
        assert!(result.successes <= 10);
    }

    #[test]
    fn failure_policy_can_continue() {
        let config = MonteCarloConfig::new(
            ExecutionId::from_u128(1),
            MonteCarloSeed::new(1),
        )
        .with_failure_policy(FailurePolicy::Continue);

        let engine = MonteCarloEngine::new(config);

        let mut evaluator =
            |sample: SampleIndex, _rng: &mut StdRng| -> Result<f64, &'static str> {
                if sample.get() % 2 == 0 {
                    Err("intentional test failure")
                } else {
                    Ok(1.0)
                }
            };

        let result = engine
            .run(10, &mut evaluator, &NeverCancel)
            .unwrap();

        assert_eq!(result.attempts, 10);
        assert_eq!(result.failures, 5);
        assert_eq!(result.successes, 5);
        assert_eq!(result.statistics.mean(), Some(1.0));
    }

    #[test]
    fn failure_policy_can_fail_fast() {
        let config = MonteCarloConfig::new(
            ExecutionId::from_u128(1),
            MonteCarloSeed::new(1),
        );

        let engine = MonteCarloEngine::new(config);

        let mut evaluator =
            |_sample: SampleIndex, _rng: &mut StdRng| -> Result<f64, &'static str> {
                Err("intentional test failure")
            };

        let result = engine.run(10, &mut evaluator, &NeverCancel);

        assert!(matches!(
            result,
            Err(MonteCarloError::Trial { .. })
        ));
    }

    #[test]
    fn explicit_success_limit_is_enforced() {
        let config = MonteCarloConfig::new(
            ExecutionId::from_u128(1),
            MonteCarloSeed::new(1),
        )
        .with_limits(MonteCarloLimits {
            max_attempts: None,
            max_successes: Some(10),
            max_failures: None,
        });

        let engine = MonteCarloEngine::new(config);

        let mut evaluator =
            |_sample: SampleIndex, _rng: &mut StdRng| -> Result<f64, ()> {
                Ok(1.0)
            };

        let result = engine.run(11, &mut evaluator, &NeverCancel);

        assert!(matches!(
            result,
            Err(MonteCarloError::SuccessLimitExceeded { .. })
        ));
    }

    #[test]
    fn explicit_attempt_limit_is_enforced() {
        let config = MonteCarloConfig::new(
            ExecutionId::from_u128(1),
            MonteCarloSeed::new(1),
        )
        .with_limits(MonteCarloLimits {
            max_attempts: Some(10),
            max_successes: None,
            max_failures: None,
        });

        let engine = MonteCarloEngine::new(config);

        let mut evaluator =
            |_sample: SampleIndex, _rng: &mut StdRng| -> Result<f64, ()> {
                Ok(1.0)
            };

        let result = engine.run(11, &mut evaluator, &NeverCancel);

        assert!(matches!(
            result,
            Err(MonteCarloError::AttemptLimitExceeded { .. })
        ));
    }

    #[test]
    fn streaming_callback_receives_successful_observations() {
        let config = MonteCarloConfig::new(
            ExecutionId::from_u128(1),
            MonteCarloSeed::new(1),
        );

        let engine = MonteCarloEngine::new(config);

        let mut evaluator =
            |sample: SampleIndex, _rng: &mut StdRng| -> Result<f64, ()> {
                Ok(sample.get() as f64)
            };

        let mut received = Vec::new();

        let result = engine
            .run_stream(
                5,
                &mut evaluator,
                &NeverCancel,
                |sample, value| {
                    received.push((sample.get(), value));
                },
            )
            .unwrap();

        assert_eq!(result.successes, 5);
        assert_eq!(received.len(), 5);
        assert_eq!(received[0], (0, 0.0));
        assert_eq!(received[4], (4, 4.0));
    }

    #[test]
    fn merge_produces_equivalent_statistics() {
        let mut all = OnlineStatistics::new();

        for value in 0..100 {
            all.observe(value as f64).unwrap();
        }

        let mut first = OnlineStatistics::new();

        for value in 0..50 {
            first.observe(value as f64).unwrap();
        }

        let mut second = OnlineStatistics::new();

        for value in 50..100 {
            second.observe(value as f64).unwrap();
        }

        first.merge(&second).unwrap();

        assert_eq!(first.count(), all.count());
        assert!((first.mean().unwrap() - all.mean().unwrap()).abs() < 1.0e-12);
        assert!(
            (first.sample_variance().unwrap()
                - all.sample_variance().unwrap())
                .abs()
                < 1.0e-10
        );
    }

    #[test]
    fn effectively_unbounded_execution_can_be_cancelled() {
        struct ImmediateCancel;

        impl CancellationToken for ImmediateCancel {
            fn is_cancelled(&self) -> bool {
                true
            }
        }

        let config = MonteCarloConfig::new(
            ExecutionId::from_u128(1),
            MonteCarloSeed::new(1),
        );

        let engine = MonteCarloEngine::new(config);

        let mut evaluator =
            |_sample: SampleIndex, _rng: &mut StdRng| -> Result<f64, ()> {
                Ok(1.0)
            };

        let result = engine
            .run_until_cancelled(&mut evaluator, &ImmediateCancel)
            .unwrap();

        assert!(result.cancelled);
        assert_eq!(result.attempts, 0);
    }
}