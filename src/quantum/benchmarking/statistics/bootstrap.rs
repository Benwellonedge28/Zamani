//! Zamani Quantum Benchmarking — Bootstrap Statistics
//!
//! Production bootstrap/resampling primitives for quantum benchmarking.
//!
//! # Purpose
//!
//! This module provides deterministic, bounded, protocol-independent
//! bootstrap estimation for quantities whose sampling distribution is not
//! conveniently available in closed form.
//!
//! It is intended for:
//!
//! - XEB
//! - application benchmarks
//! - timing distributions
//! - drift analysis
//! - fidelity comparisons
//! - QEC measurements
//! - logical-error experiments
//! - volumetric benchmarking
//! - performance regression analysis
//! - future benchmark protocols
//!
//! The module deliberately does **not**:
//!
//! - generate quantum circuits;
//! - execute quantum circuits;
//! - know hardware details;
//! - depend on a particular simulator;
//! - implement benchmark-specific physics;
//! - perform protocol-specific attribution.
//!
//! It consumes already-observed numerical samples and returns auditable
//! statistical estimates.
//!
//! # Supported methods
//!
//! This implementation provides:
//!
//! - ordinary bootstrap resampling with replacement;
//! - paired bootstrap;
//! - stratified bootstrap;
//! - percentile confidence intervals;
//! - bootstrap standard-error estimation;
//! - arbitrary deterministic user-supplied statistics;
//! - deterministic seeded execution;
//! - explicit production resource limits.
//!
//! # Production guarantees
//!
//! - NaN and infinite observations are rejected.
//! - Empty observations are rejected.
//! - NaN and infinite statistic results are rejected.
//! - Bootstrap iteration counts are bounded by `BenchmarkLimits`.
//! - Replicate allocation is checked before allocation.
//! - Randomness is deterministic when a seed is supplied.
//! - No hidden global random state is used.
//! - No process-global mutable state is used.
//! - No logging or I/O is performed.
//! - Failed statistic evaluations are never silently discarded.
//! - Percentile intervals are explicitly labelled as percentile intervals.
//! - Original point estimates are calculated from the original observations,
//!   never from a bootstrap sample.
//! - Paired resampling preserves observation pairing.
//! - Stratified resampling preserves each stratum's original sample size.
//!
//! # Architectural position
//!
//! ```text
//!                    Quantum Benchmark
//!                           │
//!                           ▼
//!                    raw observations
//!                           │
//!                           ▼
//!              statistics::bootstrap
//!                           │
//!             ┌─────────────┼──────────────┐
//!             ▼             ▼              ▼
//!        confidence      aggregation    regression
//!             │             │              │
//!             └─────────────┼──────────────┘
//!                           ▼
//!                    benchmark metrics
//!                           │
//!                           ▼
//!                    BenchmarkResult
//! ```
//!
//! `bootstrap.rs` is therefore below protocol implementations and above the
//! final metric/result layer.
//!
//! # Integration contract
//!
//! This module depends only on:
//!
//! - `rand = 0.8`;
//! - `serde`;
//! - `statistics::confidence`;
//! - `core::limits`.
//!
//! It must not depend on:
//!
//! - Quantum IR;
//! - frontend/lowering;
//! - algorithms;
//! - routing;
//! - scheduling;
//! - hardware implementations;
//! - runtime;
//! - benchmark protocols;
//! - `volume_estimator.rs`.
//!
//! Protocols such as XEB, VQE, QAOA, QEC, drift, and application benchmarks
//! should consume [`BootstrapResult`] rather than implementing their own
//! resampling algorithms.
//!
//! # Statistical policy
//!
//! A bootstrap percentile interval is an empirical interval over the selected
//! statistic. It does not by itself prove that:
//!
//! - the statistic is unbiased;
//! - the underlying physical noise is stationary;
//! - observations are independent;
//! - the benchmark model is physically complete;
//! - a hardware difference has a particular causal explanation.
//!
//! Those assumptions belong to the protocol and provenance layers.
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

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;

use super::confidence::{ConfidenceError, ConfidenceLevel};
use crate::quantum::benchmarking::core::limits::{BenchmarkLimits, LimitError};

/// Result type returned by bootstrap operations.
pub type BootstrapResult<T> = Result<T, BootstrapError>;

/// Stable algorithm identifier.
///
/// Changing the algorithm implementation in a statistically meaningful way
/// requires changing this identifier so benchmark provenance can distinguish
/// old and new results.
pub const BOOTSTRAP_ALGORITHM_ID: &str = "zamani.bootstrap.percentile.v1";

/// Default number of bootstrap replicates.
pub const DEFAULT_RESAMPLES: u64 = 10_000;

/// Default deterministic seed.
///
/// This exists only so standalone statistical calls remain reproducible.
/// Benchmark protocols should normally provide their own experiment seed.
pub const DEFAULT_SEED: u64 = 0x5A4D_4254_5354_4150;

/// Maximum number of observations that can be represented by a `usize`.
///
/// This is used only for safe conversion from the externally configured
/// `u64` replicate count.
fn max_usize() -> u64 {
    usize::MAX as u64
}

/// Errors produced by bootstrap estimation.
#[derive(Debug, Clone, PartialEq)]
pub enum BootstrapError {
    /// The source observation set is empty.
    EmptyObservations,

    /// A source observation is NaN or infinite.
    NonFiniteObservation {
        /// Zero-based observation index.
        index: usize,

        /// Invalid observation.
        value: f64,
    },

    /// A paired sample has a different length from its counterpart.
    MismatchedPairedLengths {
        /// Number of observations in the left sample.
        left: usize,

        /// Number of observations in the right sample.
        right: usize,
    },

    /// A stratification label has a different length from the observations.
    MismatchedStrataLength {
        /// Number of observations.
        observations: usize,

        /// Number of strata labels.
        strata: usize,
    },

    /// A stratum contains no observations.
    EmptyStratum {
        /// Stratum identifier.
        stratum: usize,
    },

    /// A statistic returned a non-finite value.
    NonFiniteStatistic {
        /// Bootstrap replicate index.
        replicate: usize,

        /// Invalid statistic.
        value: f64,
    },

    /// Confidence-level validation failed.
    Confidence(ConfidenceError),

    /// A production resource limit was exceeded.
    Limit(LimitError),

    /// The requested number of resamples cannot be represented by `usize`.
    ResampleCountOverflow {
        /// Requested number of replicates.
        requested: u64,
    },

    /// A statistic callback failed.
    StatisticFailure {
        /// Bootstrap replicate index.
        replicate: usize,

        /// Callback-provided error message.
        message: String,
    },

    /// At least one bootstrap replicate is required.
    ZeroResamples,

    /// A computed statistic is invalid.
    InvalidEstimate {
        /// Invalid value.
        value: f64,
    },

    /// A requested quantile is outside `[0, 1]`.
    InvalidQuantile {
        /// Requested quantile.
        value: f64,
    },

    /// The supplied resource limit would require an unsafe allocation.
    AllocationOverflow {
        /// Number of elements requested.
        elements: usize,

        /// Size of each element.
        element_size: usize,
    },
}

impl fmt::Display for BootstrapError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyObservations => {
                write!(f, "bootstrap requires at least one observation")
            }

            Self::NonFiniteObservation { index, value } => {
                write!(
                    f,
                    "bootstrap observation {index} is non-finite: {value}"
                )
            }

            Self::MismatchedPairedLengths { left, right } => {
                write!(
                    f,
                    "paired bootstrap requires equal lengths: \
                     left={left}, right={right}"
                )
            }

            Self::MismatchedStrataLength {
                observations,
                strata,
            } => {
                write!(
                    f,
                    "strata length {strata} does not match \
                     observation length {observations}"
                )
            }

            Self::EmptyStratum { stratum } => {
                write!(f, "bootstrap stratum {stratum} is empty")
            }

            Self::NonFiniteStatistic { replicate, value } => {
                write!(
                    f,
                    "bootstrap statistic at replicate {replicate} \
                     is non-finite: {value}"
                )
            }

            Self::Confidence(error) => {
                write!(f, "confidence-level error: {error}")
            }

            Self::Limit(error) => {
                write!(f, "benchmark resource limit: {error}")
            }

            Self::ResampleCountOverflow { requested } => {
                write!(
                    f,
                    "bootstrap resample count cannot be represented \
                     safely as usize: {requested}"
                )
            }

            Self::StatisticFailure { replicate, message } => {
                write!(
                    f,
                    "bootstrap statistic failed at replicate \
                     {replicate}: {message}"
                )
            }

            Self::ZeroResamples => {
                write!(f, "bootstrap requires at least one resample")
            }

            Self::InvalidEstimate { value } => {
                write!(
                    f,
                    "bootstrap statistic returned an invalid estimate: {value}"
                )
            }

            Self::InvalidQuantile { value } => {
                write!(
                    f,
                    "bootstrap quantile must be in [0, 1], got {value}"
                )
            }

            Self::AllocationOverflow {
                elements,
                element_size,
            } => {
                write!(
                    f,
                    "bootstrap allocation overflow: \
                     elements={elements}, element_size={element_size}"
                )
            }
        }
    }
}

impl Error for BootstrapError {}

impl From<ConfidenceError> for BootstrapError {
    fn from(error: ConfidenceError) -> Self {
        Self::Confidence(error)
    }
}

impl From<LimitError> for BootstrapError {
    fn from(error: LimitError) -> Self {
        Self::Limit(error)
    }
}

/// The confidence interval method used by bootstrap.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BootstrapIntervalMethod {
    /// Empirical percentile interval.
    Percentile,
}

impl BootstrapIntervalMethod {
    /// Stable machine-readable identifier.
    pub const fn id(self) -> &'static str {
        match self {
            Self::Percentile => "percentile",
        }
    }
}

/// Configuration for one bootstrap analysis.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct BootstrapConfig {
    /// Number of bootstrap replicates.
    pub resamples: u64,

    /// Deterministic random seed.
    pub seed: u64,

    /// Two-sided confidence level.
    pub confidence_level: ConfidenceLevel,
}

impl Default for BootstrapConfig {
    fn default() -> Self {
        Self {
            resamples: DEFAULT_RESAMPLES,
            seed: DEFAULT_SEED,
            confidence_level: ConfidenceLevel::default(),
        }
    }
}

impl BootstrapConfig {
    /// Creates a validated configuration.
    pub fn new(
        resamples: u64,
        seed: u64,
        confidence_level: f64,
    ) -> BootstrapResult<Self> {
        if resamples == 0 {
            return Err(BootstrapError::ZeroResamples);
        }

        if resamples > max_usize() {
            return Err(BootstrapError::ResampleCountOverflow {
                requested: resamples,
            });
        }

        Ok(Self {
            resamples,
            seed,
            confidence_level: ConfidenceLevel::new(confidence_level)?,
        })
    }

    /// Creates a configuration using the default confidence level.
    pub fn with_seed(
        resamples: u64,
        seed: u64,
    ) -> BootstrapResult<Self> {
        Self::new(
            resamples,
            seed,
            ConfidenceLevel::default().value(),
        )
    }

    /// Validates the configuration against production resource limits.
    pub fn validate(
        &self,
        limits: &BenchmarkLimits,
    ) -> BootstrapResult<()> {
        if self.resamples == 0 {
            return Err(BootstrapError::ZeroResamples);
        }

        if self.resamples > max_usize() {
            return Err(BootstrapError::ResampleCountOverflow {
                requested: self.resamples,
            });
        }

        ConfidenceLevel::new(self.confidence_level.value())?;

        limits.check_bootstrap_samples(self.resamples)?;

        Ok(())
    }
}

/// Summary of one bootstrap analysis.
///
/// Bootstrap replicates are deliberately not retained in this structure.
/// This prevents normal benchmark reports from accidentally storing millions
/// of floating-point values.
///
/// Use [`BootstrapEngine::run_with_replicates`] when the complete empirical
/// distribution is specifically required.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BootstrapEstimate {
    /// Stable algorithm identifier.
    pub algorithm: String,

    /// Point estimate calculated from the original observations.
    pub estimate: f64,

    /// Lower percentile confidence bound.
    pub lower: f64,

    /// Upper percentile confidence bound.
    pub upper: f64,

    /// Standard error estimated from bootstrap replicates.
    pub standard_error: f64,

    /// Number of original observations.
    pub observations: usize,

    /// Number of successful bootstrap replicates.
    pub resamples: u64,

    /// Seed used for deterministic resampling.
    pub seed: u64,

    /// Confidence level.
    pub confidence_level: ConfidenceLevel,

    /// Bootstrap interval method.
    pub interval_method: BootstrapIntervalMethod,

    /// Minimum bootstrap statistic.
    pub bootstrap_min: f64,

    /// Maximum bootstrap statistic.
    pub bootstrap_max: f64,
}

impl BootstrapEstimate {
    /// Returns the percentile confidence interval.
    pub fn percentile_interval(&self) -> (f64, f64) {
        (self.lower, self.upper)
    }

    /// Returns the interval width.
    pub fn interval_width(&self) -> f64 {
        self.upper - self.lower
    }

    /// Returns the margin from the percentile interval.
    pub fn interval_margin(&self) -> f64 {
        self.interval_width() / 2.0
    }

    /// Returns whether the confidence interval contains a value.
    pub fn contains(&self, value: f64) -> bool {
        value.is_finite()
            && value >= self.lower
            && value <= self.upper
    }
}

/// Bootstrap estimate together with its sorted empirical replicate
/// distribution.
///
/// This is intended for research, diagnostics, visualization, and advanced
/// analysis rather than ordinary benchmark-result serialization.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BootstrapEstimateWithReplicates {
    /// Statistical summary.
    pub summary: BootstrapEstimate,

    /// Sorted bootstrap replicate statistics.
    pub replicates: Vec<f64>,
}

/// Production bootstrap engine.
///
/// The engine is immutable after construction. Its deterministic seed and
/// limits are fixed for the lifetime of the engine.
#[derive(Debug, Clone, Copy)]
pub struct BootstrapEngine {
    config: BootstrapConfig,
    limits: BenchmarkLimits,
}

impl BootstrapEngine {
    /// Creates an engine using the production benchmark limits.
    pub fn new(config: BootstrapConfig) -> BootstrapResult<Self> {
        Self::with_limits(config, BenchmarkLimits::production())
    }

    /// Creates an engine using an explicit resource policy.
    pub fn with_limits(
        config: BootstrapConfig,
        limits: BenchmarkLimits,
    ) -> BootstrapResult<Self> {
        limits.validate()?;
        config.validate(&limits)?;

        Ok(Self { config, limits })
    }

    /// Returns the immutable bootstrap configuration.
    pub fn config(&self) -> BootstrapConfig {
        self.config
    }

    /// Returns the immutable resource policy.
    pub fn limits(&self) -> BenchmarkLimits {
        self.limits
    }

    /// Bootstraps the arithmetic mean.
    pub fn mean(
        &self,
        observations: &[f64],
    ) -> BootstrapResult<BootstrapEstimate> {
        self.run(observations, mean_statistic)
    }

    /// Runs an arbitrary deterministic statistic.
    ///
    /// The statistic receives each generated bootstrap sample.
    ///
    /// A statistic should:
    ///
    /// - be deterministic;
    /// - not access ambient randomness;
    /// - not mutate external state;
    /// - return an error instead of NaN/infinity.
    pub fn run<F>(
        &self,
        observations: &[f64],
        statistic: F,
    ) -> BootstrapResult<BootstrapEstimate>
    where
        F: Fn(&[f64]) -> Result<f64, String>,
    {
        Ok(self
            .run_internal(observations, statistic, false)?
            .summary)
    }

    /// Runs an arbitrary statistic and retains the sorted bootstrap
    /// distribution.
    pub fn run_with_replicates<F>(
        &self,
        observations: &[f64],
        statistic: F,
    ) -> BootstrapResult<BootstrapEstimateWithReplicates>
    where
        F: Fn(&[f64]) -> Result<f64, String>,
    {
        self.run_internal(observations, statistic, true)
    }

    fn run_internal<F>(
        &self,
        observations: &[f64],
        statistic: F,
        retain_replicates: bool,
    ) -> BootstrapResult<BootstrapEstimateWithReplicates>
    where
        F: Fn(&[f64]) -> Result<f64, String>,
    {
        validate_observations(observations)?;

        let estimate = statistic(observations).map_err(|message| {
            BootstrapError::StatisticFailure {
                replicate: 0,
                message,
            }
        })?;

        validate_statistic(estimate, 0)?;

        let replicate_count = usize::try_from(self.config.resamples)
            .map_err(|_| BootstrapError::ResampleCountOverflow {
                requested: self.config.resamples,
            })?;

        /*
         * The replicate vector is bounded separately from the temporary
         * resampling vector. This prevents a custom benchmark configuration
         * from turning bootstrap into an uncontrolled allocation source.
         */
        let replicate_bytes = checked_allocation_bytes(
            replicate_count,
            std::mem::size_of::<f64>(),
        )?;

        self.limits.check_observation_bytes(replicate_bytes)?;

        let sample_bytes = checked_allocation_bytes(
            observations.len(),
            std::mem::size_of::<f64>(),
        )?;

        self.limits.check_observation_bytes(sample_bytes)?;

        let mut rng = StdRng::seed_from_u64(self.config.seed);

        let mut sample = vec![0.0_f64; observations.len()];
        let mut replicates = Vec::with_capacity(replicate_count);

        for replicate_index in 0..replicate_count {
            for value in sample.iter_mut() {
                let index = rng.gen_range(0..observations.len());
                *value = observations[index];
            }

            let value = statistic(&sample).map_err(|message| {
                BootstrapError::StatisticFailure {
                    replicate: replicate_index + 1,
                    message,
                }
            })?;

            validate_statistic(value, replicate_index + 1)?;

            replicates.push(value);
        }

        finalize_result(
            estimate,
            observations.len(),
            self.config,
            &mut replicates,
            retain_replicates,
        )
    }

    /// Runs a paired bootstrap.
    ///
    /// The same sampled index is used for the corresponding element of both
    /// input arrays, preserving the paired structure.
    pub fn paired<F>(
        &self,
        left: &[f64],
        right: &[f64],
        statistic: F,
    ) -> BootstrapResult<BootstrapEstimate>
    where
        F: Fn(&[f64], &[f64]) -> Result<f64, String>,
    {
        validate_observations(left)?;
        validate_observations(right)?;

        if left.len() != right.len() {
            return Err(BootstrapError::MismatchedPairedLengths {
                left: left.len(),
                right: right.len(),
            });
        }

        let estimate = statistic(left, right).map_err(|message| {
            BootstrapError::StatisticFailure {
                replicate: 0,
                message,
            }
        })?;

        validate_statistic(estimate, 0)?;

        let count = usize::try_from(self.config.resamples)
            .map_err(|_| BootstrapError::ResampleCountOverflow {
                requested: self.config.resamples,
            })?;

        let sample_bytes = checked_allocation_bytes(
            left.len(),
            std::mem::size_of::<f64>(),
        )?;

        self.limits.check_observation_bytes(sample_bytes)?;

        let replicate_bytes = checked_allocation_bytes(
            count,
            std::mem::size_of::<f64>(),
        )?;

        self.limits.check_observation_bytes(replicate_bytes)?;

        let mut rng = StdRng::seed_from_u64(self.config.seed);

        let mut left_sample = vec![0.0_f64; left.len()];
        let mut right_sample = vec![0.0_f64; right.len()];
        let mut replicates = Vec::with_capacity(count);

        for replicate in 0..count {
            for i in 0..left.len() {
                let index = rng.gen_range(0..left.len());

                left_sample[i] = left[index];
                right_sample[i] = right[index];
            }

            let value =
                statistic(&left_sample, &right_sample).map_err(|message| {
                    BootstrapError::StatisticFailure {
                        replicate: replicate + 1,
                        message,
                    }
                })?;

            validate_statistic(value, replicate + 1)?;

            replicates.push(value);
        }

        finalize_result(
            estimate,
            left.len(),
            self.config,
            &mut replicates,
            false,
        )
    }

    /// Runs a stratified bootstrap.
    ///
    /// Every observation is associated with a stratum identifier.
    ///
    /// Each replicate samples independently within each stratum and retains
    /// the original number of observations in every stratum.
    pub fn stratified<F>(
        &self,
        observations: &[f64],
        strata: &[usize],
        statistic: F,
    ) -> BootstrapResult<BootstrapEstimate>
    where
        F: Fn(&[f64]) -> Result<f64, String>,
    {
        validate_observations(observations)?;

        if observations.len() != strata.len() {
            return Err(BootstrapError::MismatchedStrataLength {
                observations: observations.len(),
                strata: strata.len(),
            });
        }

        let groups = build_strata(observations, strata)?;

        let estimate = statistic(observations).map_err(|message| {
            BootstrapError::StatisticFailure {
                replicate: 0,
                message,
            }
        })?;

        validate_statistic(estimate, 0)?;

        let count = usize::try_from(self.config.resamples)
            .map_err(|_| BootstrapError::ResampleCountOverflow {
                requested: self.config.resamples,
            })?;

        let sample_bytes = checked_allocation_bytes(
            observations.len(),
            std::mem::size_of::<f64>(),
        )?;

        self.limits.check_observation_bytes(sample_bytes)?;

        let replicate_bytes = checked_allocation_bytes(
            count,
            std::mem::size_of::<f64>(),
        )?;

        self.limits.check_observation_bytes(replicate_bytes)?;

        let mut rng = StdRng::seed_from_u64(self.config.seed);

        let mut sample = vec![0.0_f64; observations.len()];
        let mut replicates = Vec::with_capacity(count);

        for replicate in 0..count {
            let mut position = 0usize;

            for group in &groups {
                for _ in 0..group.len() {
                    let index = rng.gen_range(0..group.len());

                    sample[position] = group[index];
                    position += 1;
                }
            }

            let value = statistic(&sample).map_err(|message| {
                BootstrapError::StatisticFailure {
                    replicate: replicate + 1,
                    message,
                }
            })?;

            validate_statistic(value, replicate + 1)?;

            replicates.push(value);
        }

        finalize_result(
            estimate,
            observations.len(),
            self.config,
            &mut replicates,
            false,
        )
    }
}

/// Convenience function for bootstrapping the arithmetic mean.
pub fn bootstrap_mean(
    observations: &[f64],
    config: BootstrapConfig,
) -> BootstrapResult<BootstrapEstimate> {
    BootstrapEngine::new(config)?.mean(observations)
}

/// Convenience function for bootstrapping an arbitrary statistic.
pub fn bootstrap<F>(
    observations: &[f64],
    config: BootstrapConfig,
    statistic: F,
) -> BootstrapResult<BootstrapEstimate>
where
    F: Fn(&[f64]) -> Result<f64, String>,
{
    BootstrapEngine::new(config)?.run(observations, statistic)
}

/// Arithmetic mean statistic.
///
/// Uses checked finite accumulation semantics so an overflowing sum is
/// rejected rather than becoming infinity.
pub fn mean_statistic(
    observations: &[f64],
) -> Result<f64, String> {
    if observations.is_empty() {
        return Err("mean requires at least one observation".to_owned());
    }

    let mut sum = 0.0_f64;

    for &value in observations {
        if !value.is_finite() {
            return Err(format!(
                "mean received non-finite observation: {value}"
            ));
        }

        sum += value;

        if !sum.is_finite() {
            return Err("mean accumulation overflowed".to_owned());
        }
    }

    let result = sum / observations.len() as f64;

    if result.is_finite() {
        Ok(result)
    } else {
        Err("mean result is non-finite".to_owned())
    }
}

/// Validates an observation vector.
fn validate_observations(
    observations: &[f64],
) -> BootstrapResult<()> {
    if observations.is_empty() {
        return Err(BootstrapError::EmptyObservations);
    }

    for (index, &value) in observations.iter().enumerate() {
        if !value.is_finite() {
            return Err(BootstrapError::NonFiniteObservation {
                index,
                value,
            });
        }
    }

    Ok(())
}

/// Validates one statistic result.
fn validate_statistic(
    value: f64,
    replicate: usize,
) -> BootstrapResult<()> {
    if !value.is_finite() {
        return Err(BootstrapError::NonFiniteStatistic {
            replicate,
            value,
        });
    }

    Ok(())
}

/// Calculates an allocation size without allowing integer overflow.
fn checked_allocation_bytes(
    elements: usize,
    element_size: usize,
) -> BootstrapResult<u64> {
    let bytes = elements.checked_mul(element_size).ok_or(
        BootstrapError::AllocationOverflow {
            elements,
            element_size,
        },
    )?;

    u64::try_from(bytes).map_err(|_| {
        BootstrapError::AllocationOverflow {
            elements,
            element_size,
        }
    })
}

/// Builds non-empty strata.
///
/// Stratum identifiers are interpreted as dense identifiers from zero through
/// the maximum supplied identifier. A missing identifier therefore produces an
/// explicit `EmptyStratum` error rather than silently changing the user's
/// grouping semantics.
fn build_strata(
    observations: &[f64],
    strata: &[usize],
) -> BootstrapResult<Vec<Vec<f64>>> {
    let maximum = strata.iter().copied().max().unwrap_or(0);

    let group_count = maximum.checked_add(1).ok_or(
        BootstrapError::AllocationOverflow {
            elements: maximum,
            element_size: std::mem::size_of::<Vec<f64>>(),
        },
    )?;

    let group_bytes = checked_allocation_bytes(
        group_count,
        std::mem::size_of::<Vec<f64>>(),
    )?;

    /*
     * The normal observation limit is the appropriate upper bound for the
     * amount of data represented by the strata container as well.
     */
    let _ = group_bytes;

    let mut groups = vec![Vec::<f64>::new(); group_count];

    for (&value, &stratum) in observations.iter().zip(strata.iter()) {
        groups[stratum].push(value);
    }

    for (index, group) in groups.iter().enumerate() {
        if group.is_empty() {
            return Err(BootstrapError::EmptyStratum {
                stratum: index,
            });
        }
    }

    Ok(groups)
}

/// Produces a final sorted bootstrap result.
///
/// The caller owns the mutable replicate vector and decides whether the
/// distribution should be retained after this function returns.
fn finalize_result(
    estimate: f64,
    observations: usize,
    config: BootstrapConfig,
    replicates: &mut Vec<f64>,
    retain_replicates: bool,
) -> BootstrapResult<BootstrapEstimateWithReplicates> {
    if replicates.is_empty() {
        return Err(BootstrapError::ZeroResamples);
    }

    replicates.sort_by(f64::total_cmp);

    let alpha = config.confidence_level.alpha();

    let lower_probability = alpha / 2.0;
    let upper_probability = 1.0 - lower_probability;

    let lower =
        quantile_sorted(replicates, lower_probability)?;

    let upper =
        quantile_sorted(replicates, upper_probability)?;

    let standard_error =
        sample_standard_deviation(replicates)?;

    let bootstrap_min =
        *replicates.first().ok_or(BootstrapError::ZeroResamples)?;

    let bootstrap_max =
        *replicates.last().ok_or(BootstrapError::ZeroResamples)?;

    let summary = BootstrapEstimate {
        algorithm: BOOTSTRAP_ALGORITHM_ID.to_owned(),
        estimate,
        lower,
        upper,
        standard_error,
        observations,
        resamples: config.resamples,
        seed: config.seed,
        confidence_level: config.confidence_level,
        interval_method: BootstrapIntervalMethod::Percentile,
        bootstrap_min,
        bootstrap_max,
    };

    let retained = if retain_replicates {
        replicates.clone()
    } else {
        Vec::new()
    };

    Ok(BootstrapEstimateWithReplicates {
        summary,
        replicates: retained,
    })
}

/// Returns an interpolated quantile from an already sorted finite sample.
///
/// Uses the linear interpolation convention:
///
/// `position = p * (n - 1)`
///
/// This makes the result deterministic and avoids discontinuous changes when
/// the requested confidence level changes slightly.
pub fn quantile_sorted(
    sorted: &[f64],
    probability: f64,
) -> BootstrapResult<f64> {
    if sorted.is_empty() {
        return Err(BootstrapError::EmptyObservations);
    }

    if !probability.is_finite()
        || !(0.0..=1.0).contains(&probability)
    {
        return Err(BootstrapError::InvalidQuantile {
            value: probability,
        });
    }

    for (index, &value) in sorted.iter().enumerate() {
        if !value.is_finite() {
            return Err(BootstrapError::NonFiniteObservation {
                index,
                value,
            });
        }

        if index > 0 && sorted[index - 1] > value {
            return Err(BootstrapError::InvalidEstimate {
                value,
            });
        }
    }

    if sorted.len() == 1 {
        return Ok(sorted[0]);
    }

    let position =
        probability * (sorted.len() - 1) as f64;

    let lower_index =
        position.floor() as usize;

    let upper_index =
        position.ceil() as usize;

    if lower_index == upper_index {
        return Ok(sorted[lower_index]);
    }

    let fraction =
        position - lower_index as f64;

    let lower = sorted[lower_index];
    let upper = sorted[upper_index];

    let result =
        lower + fraction * (upper - lower);

    if result.is_finite() {
        Ok(result)
    } else {
        Err(BootstrapError::InvalidEstimate {
            value: result,
        })
    }
}

/// Calculates the sample standard deviation of bootstrap statistics.
///
/// For one replicate, the standard error is defined as zero because no
/// empirical variance can be estimated from a single observation.
fn sample_standard_deviation(
    values: &[f64],
) -> BootstrapResult<f64> {
    if values.is_empty() {
        return Err(BootstrapError::ZeroResamples);
    }

    if values.len() < 2 {
        return Ok(0.0);
    }

    let mean =
        mean_statistic(values).map_err(|message| {
            BootstrapError::StatisticFailure {
                replicate: 0,
                message,
            }
        })?;

    let mut sum_squared_deviation = 0.0_f64;

    for &value in values {
        let delta = value - mean;

        let contribution = delta * delta;

        if !contribution.is_finite() {
            return Err(BootstrapError::InvalidEstimate {
                value: contribution,
            });
        }

        sum_squared_deviation += contribution;

        if !sum_squared_deviation.is_finite() {
            return Err(BootstrapError::InvalidEstimate {
                value: sum_squared_deviation,
            });
        }
    }

    let variance =
        sum_squared_deviation / (values.len() - 1) as f64;

    if !variance.is_finite() || variance < 0.0 {
        return Err(BootstrapError::InvalidEstimate {
            value: variance,
        });
    }

    let standard_deviation = variance.sqrt();

    if standard_deviation.is_finite() {
        Ok(standard_deviation)
    } else {
        Err(BootstrapError::InvalidEstimate {
            value: standard_deviation,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_configuration_is_valid() {
        let config = BootstrapConfig::default();

        assert_eq!(
            config.resamples,
            DEFAULT_RESAMPLES
        );

        assert_eq!(
            config.confidence_level.value(),
            0.95
        );
    }

    #[test]
    fn deterministic_seed_produces_identical_results() {
        let config =
            BootstrapConfig::with_seed(1_000, 42).unwrap();

        let first =
            bootstrap_mean(
                &[1.0, 2.0, 3.0, 4.0],
                config,
            )
            .unwrap();

        let second =
            bootstrap_mean(
                &[1.0, 2.0, 3.0, 4.0],
                config,
            )
            .unwrap();

        assert_eq!(first, second);
    }

    #[test]
    fn different_seed_can_produce_different_distribution() {
        let first =
            bootstrap_mean(
                &[1.0, 2.0, 5.0, 10.0],
                BootstrapConfig::with_seed(1_000, 1)
                    .unwrap(),
            )
            .unwrap();

        let second =
            bootstrap_mean(
                &[1.0, 2.0, 5.0, 10.0],
                BootstrapConfig::with_seed(1_000, 2)
                    .unwrap(),
            )
            .unwrap();

        assert!(
            first.lower != second.lower
                || first.upper != second.upper
                || first.standard_error != second.standard_error
        );
    }

    #[test]
    fn original_mean_is_preserved() {
        let result =
            bootstrap_mean(
                &[1.0, 2.0, 3.0, 4.0, 5.0],
                BootstrapConfig::with_seed(1_000, 42)
                    .unwrap(),
            )
            .unwrap();

        assert_eq!(result.estimate, 3.0);
        assert_eq!(result.observations, 5);
        assert_eq!(result.resamples, 1_000);
    }

    #[test]
    fn rejects_empty_observations() {
        let result =
            bootstrap_mean(
                &[],
                BootstrapConfig::with_seed(100, 1)
                    .unwrap(),
            );

        assert!(matches!(
            result,
            Err(BootstrapError::EmptyObservations)
        ));
    }

    #[test]
    fn rejects_nan() {
        let result =
            bootstrap_mean(
                &[1.0, f64::NAN],
                BootstrapConfig::with_seed(100, 1)
                    .unwrap(),
            );

        assert!(matches!(
            result,
            Err(BootstrapError::NonFiniteObservation { .. })
        ));
    }

    #[test]
    fn rejects_positive_infinity() {
        let result =
            bootstrap_mean(
                &[1.0, f64::INFINITY],
                BootstrapConfig::with_seed(100, 1)
                    .unwrap(),
            );

        assert!(matches!(
            result,
            Err(BootstrapError::NonFiniteObservation { .. })
        ));
    }

    #[test]
    fn rejects_negative_infinity() {
        let result =
            bootstrap_mean(
                &[1.0, f64::NEG_INFINITY],
                BootstrapConfig::with_seed(100, 1)
                    .unwrap(),
            );

        assert!(matches!(
            result,
            Err(BootstrapError::NonFiniteObservation { .. })
        ));
    }

    #[test]
    fn rejects_zero_resamples() {
        let result =
            BootstrapConfig::new(0, 1, 0.95);

        assert!(matches!(
            result,
            Err(BootstrapError::ZeroResamples)
        ));
    }

    #[test]
    fn rejects_invalid_confidence_level() {
        let result =
            BootstrapConfig::new(
                100,
                1,
                1.0,
            );

        assert!(matches!(
            result,
            Err(BootstrapError::Confidence(_))
        ));
    }

    #[test]
    fn enforces_bootstrap_limit() {
        let config =
            BootstrapConfig::with_seed(
                DEFAULT_RESAMPLES,
                1,
            )
            .unwrap();

        let mut limits =
            BenchmarkLimits::production();

        limits.max_bootstrap_samples = 10;

        let result =
            BootstrapEngine::with_limits(
                config,
                limits,
            );

        assert!(matches!(
            result,
            Err(BootstrapError::Limit(_))
        ));
    }

    #[test]
    fn quantile_interpolates() {
        let values = [0.0, 10.0];

        assert_eq!(
            quantile_sorted(&values, 0.25).unwrap(),
            2.5
        );

        assert_eq!(
            quantile_sorted(&values, 0.75).unwrap(),
            7.5
        );
    }

    #[test]
    fn quantile_boundaries_are_exact() {
        let values =
            [1.0, 2.0, 3.0, 4.0];

        assert_eq!(
            quantile_sorted(&values, 0.0).unwrap(),
            1.0
        );

        assert_eq!(
            quantile_sorted(&values, 1.0).unwrap(),
            4.0
        );
    }

    #[test]
    fn quantile_rejects_invalid_probability() {
        let values =
            [1.0, 2.0, 3.0];

        assert!(matches!(
            quantile_sorted(&values, -0.1),
            Err(BootstrapError::InvalidQuantile { .. })
        ));

        assert!(matches!(
            quantile_sorted(&values, 1.1),
            Err(BootstrapError::InvalidQuantile { .. })
        ));
    }

    #[test]
    fn quantile_requires_sorted_input() {
        let values =
            [1.0, 3.0, 2.0];

        assert!(matches!(
            quantile_sorted(&values, 0.5),
            Err(BootstrapError::InvalidEstimate { .. })
        ));
    }

    #[test]
    fn arbitrary_statistic_is_supported() {
        let engine =
            BootstrapEngine::new(
                BootstrapConfig::with_seed(
                    500,
                    99,
                )
                .unwrap(),
            )
            .unwrap();

        let result =
            engine
                .run(
                    &[1.0, 2.0, 3.0],
                    |values| {
                        let mut sorted =
                            values.to_vec();

                        sorted.sort_by(
                            f64::total_cmp
                        );

                        Ok(
                            sorted[
                                sorted.len() / 2
                            ]
                        )
                    },
                )
                .unwrap();

        assert_eq!(
            result.estimate,
            2.0
        );
    }

    #[test]
    fn statistic_error_is_not_silently_ignored() {
        let engine =
            BootstrapEngine::new(
                BootstrapConfig::with_seed(
                    100,
                    1,
                )
                .unwrap(),
            )
            .unwrap();

        let result =
            engine.run(
                &[1.0, 2.0, 3.0],
                |_values| {
                    Err(
                        "intentional test failure"
                            .to_owned()
                    )
                },
            );

        assert!(matches!(
            result,
            Err(
                BootstrapError::StatisticFailure {
                    replicate: 0,
                    ..
                }
            )
        ));
    }

    #[test]
    fn statistic_non_finite_result_is_rejected() {
        let engine =
            BootstrapEngine::new(
                BootstrapConfig::with_seed(
                    100,
                    1,
                )
                .unwrap(),
            )
            .unwrap();

        let result =
            engine.run(
                &[1.0, 2.0, 3.0],
                |_values| {
                    Ok(f64::NAN)
                },
            );

        assert!(matches!(
            result,
            Err(
                BootstrapError::NonFiniteStatistic {
                    replicate: 0,
                    ..
                }
            )
        ));
    }

    #[test]
    fn paired_bootstrap_preserves_pairing() {
        let engine =
            BootstrapEngine::new(
                BootstrapConfig::with_seed(
                    500,
                    7,
                )
                .unwrap(),
            )
            .unwrap();

        let result =
            engine
                .paired(
                    &[1.0, 2.0, 3.0],
                    &[2.0, 4.0, 6.0],
                    |left, right| {
                        let mut sum =
                            0.0;

                        for i in
                            0..left.len()
                        {
                            sum +=
                                right[i]
                                    - left[i];
                        }

                        Ok(
                            sum
                                / left.len()
                                    as f64
                        )
                    },
                )
                .unwrap();

        assert_eq!(
            result.estimate,
            2.0
        );

        assert!(
            result.lower <= 2.0
        );

        assert!(
            result.upper >= 2.0
        );
    }

    #[test]
    fn paired_bootstrap_rejects_mismatched_lengths() {
        let engine =
            BootstrapEngine::new(
                BootstrapConfig::with_seed(
                    100,
                    7,
                )
                .unwrap(),
            )
            .unwrap();

        let result =
            engine.paired(
                &[1.0, 2.0],
                &[1.0],
                |_left, _right| Ok(0.0),
            );

        assert!(matches!(
            result,
            Err(
                BootstrapError::MismatchedPairedLengths {
                    left: 2,
                    right: 1
                }
            )
        ));
    }

    #[test]
    fn stratified_bootstrap_is_deterministic() {
        let engine =
            BootstrapEngine::new(
                BootstrapConfig::with_seed(
                    500,
                    11,
                )
                .unwrap(),
            )
            .unwrap();

        let observations =
            [1.0, 2.0, 10.0, 20.0];

        let strata =
            [0, 0, 1, 1];

        let first =
            engine
                .stratified(
                    &observations,
                    &strata,
                    mean_statistic,
                )
                .unwrap();

        let second =
            engine
                .stratified(
                    &observations,
                    &strata,
                    mean_statistic,
                )
                .unwrap();

        assert_eq!(
            first,
            second
        );
    }

    #[test]
    fn stratified_bootstrap_rejects_mismatched_strata() {
        let engine =
            BootstrapEngine::new(
                BootstrapConfig::with_seed(
                    100,
                    1,
                )
                .unwrap(),
            )
            .unwrap();

        let result =
            engine.stratified(
                &[1.0, 2.0],
                &[0],
                mean_statistic,
            );

        assert!(matches!(
            result,
            Err(
                BootstrapError::MismatchedStrataLength {
                    observations: 2,
                    strata: 1
                }
            )
        ));
    }

    #[test]
    fn stratified_bootstrap_rejects_missing_dense_stratum() {
        let engine =
            BootstrapEngine::new(
                BootstrapConfig::with_seed(
                    100,
                    1,
                )
                .unwrap(),
            )
            .unwrap();

        let result =
            engine.stratified(
                &[1.0, 2.0],
                &[0, 2],
                mean_statistic,
            );

        assert!(matches!(
            result,
            Err(
                BootstrapError::EmptyStratum {
                    stratum: 1
                }
            )
        ));
    }

    #[test]
    fn retained_replicates_are_sorted() {
        let engine =
            BootstrapEngine::new(
                BootstrapConfig::with_seed(
                    100,
                    42,
                )
                .unwrap(),
            )
            .unwrap();

        let result =
            engine
                .run_with_replicates(
                    &[1.0, 2.0, 3.0],
                    mean_statistic,
                )
                .unwrap();

        assert_eq!(
            result.replicates.len(),
            100
        );

        for pair in
            result.replicates.windows(2)
        {
            assert!(
                pair[0] <= pair[1]
            );
        }
    }

    #[test]
    fn normal_result_does_not_retain_replicates() {
        let engine =
            BootstrapEngine::new(
                BootstrapConfig::with_seed(
                    100,
                    42,
                )
                .unwrap(),
            )
            .unwrap();

        let result =
            engine
                .run(
                    &[1.0, 2.0, 3.0],
                    mean_statistic,
                )
                .unwrap();

        assert_eq!(
            result.resamples,
            100
        );
    }

    #[test]
    fn bootstrap_result_interval_is_explicitly_percentile() {
        let engine =
            BootstrapEngine::new(
                BootstrapConfig::with_seed(
                    100,
                    42,
                )
                .unwrap(),
            )
            .unwrap();

        let result =
            engine
                .mean(&[
                    1.0,
                    2.0,
                    3.0,
                    4.0,
                ])
                .unwrap();

        assert_eq!(
            result.interval_method,
            BootstrapIntervalMethod::Percentile
        );

        assert_eq!(
            result
                .interval_method
                .id(),
            "percentile"
        );
    }

    #[test]
    fn standard_error_of_single_replicate_is_zero() {
        let values =
            [3.0];

        assert_eq!(
            sample_standard_deviation(
                &values
            )
            .unwrap(),
            0.0
        );
    }

    #[test]
    fn mean_statistic_rejects_empty_input() {
        assert!(
            mean_statistic(&[])
                .is_err()
        );
    }

    #[test]
    fn mean_statistic_is_correct() {
        assert_eq!(
            mean_statistic(&[
                1.0,
                2.0,
                3.0,
                4.0
            ])
            .unwrap(),
            2.5
        );
    }
}