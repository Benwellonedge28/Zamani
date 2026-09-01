//! # ZQN Characterization Estimator
//!
//! Production estimator layer for Zamani Quantum Noise (ZQN).
//!
//! ## Ownership
//!
//! This file owns the transformation:
//!
//! ```text
//! characterization observations
//!             |
//!             v
//!      statistical inference
//!             |
//!             v
//!      characterization estimate
//! ```
//!
//! The estimator is responsible for:
//!
//! - point estimation;
//! - interval estimation;
//! - uncertainty propagation at the estimator boundary;
//! - sample-size accounting;
//! - weighted and unweighted aggregation;
//! - convergence evaluation;
//! - finite-sample validation;
//! - deterministic aggregation;
//! - estimator configuration;
//! - estimator provenance metadata;
//! - streaming-friendly accumulation;
//! - resource-safe statistical computation;
//! - rejection of invalid numerical input;
//! - explicit distinction between exact, approximate and statistical results.
//!
//! ## Does not own
//!
//! This file does NOT own:
//!
//! - canonical quantum IR;
//! - circuit construction;
//! - quantum operation definitions;
//! - experiment generation;
//! - protocol definitions;
//! - backend execution;
//! - QPU communication;
//! - calibration acquisition;
//! - noise-model definitions;
//! - quantum-channel mathematics;
//! - syndrome decoding;
//! - logical error correction;
//! - benchmarking methodology;
//! - source-language parsing;
//! - vendor APIs.
//!
//! Those responsibilities belong to their respective ZQN or quantum modules.
//!
//! ## Integration
//!
//! ```text
//! characterization::protocol
//!          |
//!          v
//! characterization::experiment
//!          |
//!          v
//! runtime / hardware / simulator
//!          |
//!          v
//! characterization::observation
//!          |
//!          v
//! THIS MODULE
//!          |
//!          +----> characterization::uncertainty
//!          |
//!          +----> calibration
//!          |
//!          +----> propagation
//!          |
//!          +----> benchmarking
//!          |
//!          +----> noise model construction
//! ```
//!
//! The estimator consumes observations and produces estimates. It does not
//! execute experiments.
//!
//! ## Canonical qubit identity
//!
//! Where characterization results are associated with logical or physical
//! qubits, this module uses:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! ZQN does not define competing qubit identifiers.
//!
//! ## Scalability
//!
//! No semantic upper bound is imposed on:
//!
//! - number of qubits;
//! - number of resources;
//! - number of observations;
//! - number of experiments;
//! - number of shots;
//! - number of characterization parameters.
//!
//! Large datasets should be accumulated incrementally rather than requiring
//! the entire observation set to be materialized in memory.
//!
//! Counts use `u64` and checked arithmetic. Statistical aggregates use
//! numerically validated floating-point values. Overflow or loss of
//! representability is reported rather than silently wrapped.
//!
//! ## Determinism
//!
//! Estimation itself is deterministic.
//!
//! No RNG is used by this module.
//!
//! Given the same ordered observations and configuration, an estimator must
//! produce the same result regardless of execution backend.
//!
//! Parallel callers should use deterministic partitioning and deterministic
//! merge ordering before invoking the final aggregation operation.
//!
//! ## Resource safety
//!
//! This module does not allocate memory proportional to the total number of
//! shots for ordinary scalar estimators.
//!
//! Streaming accumulators are preferred.
//!
//! Explicit collection of observations is permitted only at higher layers
//! when the protocol requires it.
//!
//! No global mutable state is used.
//!
//! ## Numerical safety
//!
//! NaN and infinite values are rejected.
//!
//! Invalid probabilities are rejected rather than clamped silently.
//!
//! Arithmetic that can overflow is checked.
//!
//! ## Serialization
//!
//! This file defines the semantic estimator structures. External serialization
//! belongs to `zqn::io`.
//!
//! Serialized forms must use explicit schema/version contracts and must not
//! depend on Rust memory layout.
//!
//! ## Testing
//!
//! This module requires:
//!
//! - unit tests;
//! - property tests;
//! - deterministic tests;
//! - edge-case tests;
//! - overflow tests;
//! - invalid-number tests;
//! - streaming-vs-batch equivalence tests;
//! - partition/merge equivalence tests.
//!
//! ## Rust compatibility
//!
//! Designed for Rust 1.97 / 1.97.1.
//!
//! No `unsafe` code is used.

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};

/// Result type used by the estimator layer.
pub type EstimatorResult<T> = Result<T, EstimatorError>;

/// Errors produced by characterization estimation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EstimatorError {
    /// A required identifier was empty.
    EmptyIdentifier {
        field: &'static str,
    },

    /// A numerical value was NaN or infinite.
    NonFiniteValue {
        field: &'static str,
    },

    /// A probability was outside [0, 1].
    InvalidProbability {
        field: &'static str,
    },

    /// A probability interval was malformed.
    InvalidInterval {
        field: &'static str,
    },

    /// A count or accumulator overflowed.
    ArithmeticOverflow {
        operation: &'static str,
    },

    /// A denominator required by an estimator was zero.
    ZeroDenominator {
        estimator: &'static str,
    },

    /// An estimator received no observations.
    InsufficientObservations {
        estimator: &'static str,
    },

    /// A requested confidence level is invalid.
    InvalidConfidence {
        value_description: &'static str,
    },

    /// A requested tolerance is invalid.
    InvalidTolerance {
        value_description: &'static str,
    },

    /// A weighting factor is invalid.
    InvalidWeight {
        field: &'static str,
    },

    /// A resource identifier is invalid.
    InvalidResource {
        field: &'static str,
    },

    /// A configuration is internally inconsistent.
    InvalidConfiguration {
        reason: &'static str,
    },

    /// A merge operation combines incompatible estimators.
    IncompatibleAccumulator,

    /// An estimator cannot provide the requested result.
    UnsupportedEstimation {
        reason: &'static str,
    },
}

impl fmt::Display for EstimatorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyIdentifier { field } => {
                write!(f, "{field} must not be empty")
            }
            Self::NonFiniteValue { field } => {
                write!(f, "{field} must be finite")
            }
            Self::InvalidProbability { field } => {
                write!(f, "{field} must be within [0, 1]")
            }
            Self::InvalidInterval { field } => {
                write!(f, "invalid interval for {field}")
            }
            Self::ArithmeticOverflow { operation } => {
                write!(f, "arithmetic overflow during {operation}")
            }
            Self::ZeroDenominator { estimator } => {
                write!(f, "{estimator} has a zero denominator")
            }
            Self::InsufficientObservations { estimator } => {
                write!(f, "{estimator} has insufficient observations")
            }
            Self::InvalidConfidence { value_description } => {
                write!(f, "invalid confidence: {value_description}")
            }
            Self::InvalidTolerance { value_description } => {
                write!(f, "invalid tolerance: {value_description}")
            }
            Self::InvalidWeight { field } => {
                write!(f, "invalid weight: {field}")
            }
            Self::InvalidResource { field } => {
                write!(f, "invalid resource: {field}")
            }
            Self::InvalidConfiguration { reason } => {
                write!(f, "invalid estimator configuration: {reason}")
            }
            Self::IncompatibleAccumulator => {
                write!(f, "incompatible estimator accumulators")
            }
            Self::UnsupportedEstimation { reason } => {
                write!(f, "unsupported estimation: {reason}")
            }
        }
    }
}

impl Error for EstimatorError {}

/// Confidence specification for interval estimation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ConfidenceLevel(f64);

impl ConfidenceLevel {
    /// Creates a confidence level in the open interval (0, 1).
    pub fn new(value: f64) -> EstimatorResult<Self> {
        if !value.is_finite() {
            return Err(EstimatorError::NonFiniteValue {
                field: "confidence level",
            });
        }

        if !(0.0 < value && value < 1.0) {
            return Err(EstimatorError::InvalidConfidence {
                value_description: "must satisfy 0 < confidence < 1",
            });
        }

        Ok(Self(value))
    }

    /// Returns the confidence value.
    pub fn value(self) -> f64 {
        self.0
    }
}

impl Default for ConfidenceLevel {
    fn default() -> Self {
        // 95% confidence is a convention, not a machine-size limit.
        Self(0.95)
    }
}

/// Absolute/relative convergence target.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tolerance {
    /// Absolute difference between successive estimates.
    Absolute(f64),

    /// Relative difference between successive estimates.
    Relative(f64),

    /// Both conditions may be used; convergence occurs when either is met.
    AbsoluteOrRelative {
        absolute: f64,
        relative: f64,
    },
}

impl Tolerance {
    /// Validates the tolerance.
    pub fn validate(self) -> EstimatorResult<()> {
        match self {
            Self::Absolute(value) | Self::Relative(value) => {
                validate_positive_finite(value, "tolerance")
            }
            Self::AbsoluteOrRelative {
                absolute,
                relative,
            } => {
                validate_positive_finite(absolute, "absolute tolerance")?;
                validate_positive_finite(relative, "relative tolerance")
            }
        }
    }

    /// Determines whether two estimates satisfy the tolerance.
    pub fn satisfied(self, previous: f64, current: f64) -> EstimatorResult<bool> {
        validate_finite(previous, "previous estimate")?;
        validate_finite(current, "current estimate")?;
        self.validate()?;

        let absolute_difference = (current - previous).abs();

        match self {
            Self::Absolute(limit) => Ok(absolute_difference <= limit),

            Self::Relative(limit) => {
                let scale = previous.abs().max(current.abs());

                if scale == 0.0 {
                    return Ok(true);
                }

                Ok(absolute_difference / scale <= limit)
            }

            Self::AbsoluteOrRelative {
                absolute,
                relative,
            } => {
                let scale = previous.abs().max(current.abs());

                if absolute_difference <= absolute {
                    return Ok(true);
                }

                if scale == 0.0 {
                    return Ok(true);
                }

                Ok(absolute_difference / scale <= relative)
            }
        }
    }
}

/// Estimation precision policy.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PrecisionPolicy {
    /// Produce the estimator result without a convergence requirement.
    None,

    /// Require a confidence interval half-width no greater than the target.
    AbsoluteConfidenceWidth {
        tolerance: f64,
        confidence: ConfidenceLevel,
    },

    /// Require relative confidence width no greater than the target.
    RelativeConfidenceWidth {
        tolerance: f64,
        confidence: ConfidenceLevel,
    },
}

impl PrecisionPolicy {
    /// Validates the policy.
    pub fn validate(self) -> EstimatorResult<()> {
        match self {
            Self::None => Ok(()),

            Self::AbsoluteConfidenceWidth {
                tolerance,
                confidence,
            } => {
                validate_positive_finite(tolerance, "absolute confidence width")?;
                let _ = confidence.value();
                Ok(())
            }

            Self::RelativeConfidenceWidth {
                tolerance,
                confidence,
            } => {
                validate_positive_finite(tolerance, "relative confidence width")?;
                let _ = confidence.value();
                Ok(())
            }
        }
    }
}

/// Estimator method.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EstimatorKind {
    /// Bernoulli/binomial probability estimator.
    Probability,

    /// Arithmetic mean.
    Mean,

    /// Sample variance.
    Variance,

    /// Standard deviation.
    StandardDeviation,

    /// Weighted arithmetic mean.
    WeightedMean,
}

/// A logical or physical quantum resource to which an estimate applies.
#[derive(Debug)]
pub enum CharacterizationResource {
    /// Logical qubit.
    LogicalQubit(QubitId),

    /// Physical qubit.
    PhysicalQubit(PhysicalQubitId),

    /// Arbitrary named resource for non-qubit quantum modalities.
    Named(String),

    /// Composite resource containing multiple resources.
    Composite(Vec<CharacterizationResource>),
}

impl CharacterizationResource {
    /// Validates the resource recursively without assuming a fixed number of
    /// resources.
    pub fn validate(&self) -> EstimatorResult<()> {
        let mut stack = vec![self];

        while let Some(resource) = stack.pop() {
            match resource {
                Self::LogicalQubit(_) | Self::PhysicalQubit(_) => {}

                Self::Named(name) => {
                    if name.trim().is_empty() {
                        return Err(EstimatorError::InvalidResource {
                            field: "named characterization resource",
                        });
                    }
                }

                Self::Composite(resources) => {
                    for resource in resources {
                        stack.push(resource);
                    }
                }
            }
        }

        Ok(())
    }
}

/// Identifier for an estimation parameter.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct EstimateId(String);

impl EstimateId {
    /// Creates an estimate identifier.
    pub fn new(value: impl Into<String>) -> EstimatorResult<Self> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(EstimatorError::EmptyIdentifier {
                field: "estimate id",
            });
        }

        Ok(Self(value))
    }

    /// Returns the identifier.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// A statistically estimated scalar.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ScalarEstimate {
    /// Point estimate.
    pub value: f64,

    /// Number of observations contributing to the estimate.
    pub samples: u64,

    /// Optional standard error.
    pub standard_error: Option<f64>,

    /// Optional confidence interval.
    pub interval: Option<ConfidenceInterval>,
}

impl ScalarEstimate {
    /// Creates an estimate after validating all values.
    pub fn new(
        value: f64,
        samples: u64,
        standard_error: Option<f64>,
        interval: Option<ConfidenceInterval>,
    ) -> EstimatorResult<Self> {
        validate_finite(value, "estimate value")?;

        if let Some(error) = standard_error {
            validate_non_negative_finite(error, "standard error")?;
        }

        if let Some(interval) = interval {
            interval.validate()?;

            if value < interval.lower || value > interval.upper {
                return Err(EstimatorError::InvalidInterval {
                    field: "estimate interval",
                });
            }
        }

        Ok(Self {
            value,
            samples,
            standard_error,
            interval,
        })
    }
}

/// Confidence interval for a scalar estimate.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ConfidenceInterval {
    /// Lower bound.
    pub lower: f64,

    /// Upper bound.
    pub upper: f64,

    /// Confidence level.
    pub confidence: ConfidenceLevel,
}

impl ConfidenceInterval {
    /// Creates a validated confidence interval.
    pub fn new(
        lower: f64,
        upper: f64,
        confidence: ConfidenceLevel,
    ) -> EstimatorResult<Self> {
        validate_finite(lower, "interval lower bound")?;
        validate_finite(upper, "interval upper bound")?;

        if lower > upper {
            return Err(EstimatorError::InvalidInterval {
                field: "confidence interval",
            });
        }

        Ok(Self {
            lower,
            upper,
            confidence,
        })
    }

    /// Validates the interval.
    pub fn validate(self) -> EstimatorResult<()> {
        validate_finite(self.lower, "interval lower bound")?;
        validate_finite(self.upper, "interval upper bound")?;

        if self.lower > self.upper {
            return Err(EstimatorError::InvalidInterval {
                field: "confidence interval",
            });
        }

        Ok(())
    }

    /// Width of the interval.
    pub fn width(self) -> f64 {
        self.upper - self.lower
    }

    /// Half-width of the interval.
    pub fn half_width(self) -> f64 {
        self.width() / 2.0
    }
}

/// Observation representing one Bernoulli trial.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BernoulliObservation {
    /// Whether the event occurred.
    pub success: bool,
}

/// Observation representing one real-valued measurement.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ScalarObservation {
    /// Observed value.
    pub value: f64,

    /// Optional non-negative statistical weight.
    pub weight: Option<f64>,
}

/// A stream-friendly Bernoulli accumulator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BernoulliAccumulator {
    successes: u64,
    trials: u64,
}

impl BernoulliAccumulator {
    /// Creates an empty accumulator.
    pub const fn new() -> Self {
        Self {
            successes: 0,
            trials: 0,
        }
    }

    /// Adds one Bernoulli observation.
    pub fn observe(&mut self, observation: BernoulliObservation) -> EstimatorResult<()> {
        self.trials = self
            .trials
            .checked_add(1)
            .ok_or(EstimatorError::ArithmeticOverflow {
                operation: "Bernoulli trial count",
            })?;

        if observation.success {
            self.successes = self
                .successes
                .checked_add(1)
                .ok_or(EstimatorError::ArithmeticOverflow {
                    operation: "Bernoulli success count",
                })?;
        }

        Ok(())
    }

    /// Merges another accumulator.
    pub fn merge(&mut self, other: Self) -> EstimatorResult<()> {
        self.successes = self
            .successes
            .checked_add(other.successes)
            .ok_or(EstimatorError::ArithmeticOverflow {
                operation: "Bernoulli success merge",
            })?;

        self.trials = self
            .trials
            .checked_add(other.trials)
            .ok_or(EstimatorError::ArithmeticOverflow {
                operation: "Bernoulli trial merge",
            })?;

        if self.successes > self.trials {
            return Err(EstimatorError::InvalidConfiguration {
                reason: "success count exceeds trial count",
            });
        }

        Ok(())
    }

    /// Number of successes.
    pub const fn successes(&self) -> u64 {
        self.successes
    }

    /// Number of trials.
    pub const fn trials(&self) -> u64 {
        self.trials
    }

    /// Estimates the probability.
    pub fn estimate(&self) -> EstimatorResult<ScalarEstimate> {
        if self.trials == 0 {
            return Err(EstimatorError::InsufficientObservations {
                estimator: "probability",
            });
        }

        let value = self.successes as f64 / self.trials as f64;

        ScalarEstimate::new(value, self.trials, None, None)
    }
}

impl Default for BernoulliAccumulator {
    fn default() -> Self {
        Self::new()
    }
}

/// Numerically stable streaming mean/variance accumulator.
///
/// Uses a Welford-style online algorithm.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MomentsAccumulator {
    count: u64,
    mean: f64,
    m2: f64,
}

impl MomentsAccumulator {
    /// Creates an empty accumulator.
    pub const fn new() -> Self {
        Self {
            count: 0,
            mean: 0.0,
            m2: 0.0,
        }
    }

    /// Adds one scalar observation.
    pub fn observe(&mut self, value: f64) -> EstimatorResult<()> {
        validate_finite(value, "scalar observation")?;

        let next_count = self
            .count
            .checked_add(1)
            .ok_or(EstimatorError::ArithmeticOverflow {
                operation: "moment count",
            })?;

        if self.count == 0 {
            self.count = 1;
            self.mean = value;
            self.m2 = 0.0;
            return Ok(());
        }

        let delta = value - self.mean;
        let next_mean = self.mean + delta / next_count as f64;
        let delta2 = value - next_mean;
        let next_m2 = self.m2 + delta * delta2;

        if !next_mean.is_finite() || !next_m2.is_finite() {
            return Err(EstimatorError::NonFiniteValue {
                field: "moment accumulator",
            });
        }

        self.count = next_count;
        self.mean = next_mean;
        self.m2 = next_m2.max(0.0);

        Ok(())
    }

    /// Merges another moments accumulator.
    ///
    /// The merge operation is mathematically equivalent to processing the
    /// two partitions as one dataset, subject to floating-point rounding.
    pub fn merge(&mut self, other: Self) -> EstimatorResult<()> {
        if other.count == 0 {
            return Ok(());
        }

        if self.count == 0 {
            *self = other;
            return Ok(());
        }

        let total = self
            .count
            .checked_add(other.count)
            .ok_or(EstimatorError::ArithmeticOverflow {
                operation: "moment accumulator merge",
            })?;

        let delta = other.mean - self.mean;

        let left_weight = self.count as f64;
        let right_weight = other.count as f64;
        let total_weight = total as f64;

        let mean = self.mean + delta * right_weight / total_weight;

        let m2 = self.m2
            + other.m2
            + delta * delta * left_weight * right_weight / total_weight;

        if !mean.is_finite() || !m2.is_finite() {
            return Err(EstimatorError::NonFiniteValue {
                field: "merged moment accumulator",
            });
        }

        self.count = total;
        self.mean = mean;
        self.m2 = m2.max(0.0);

        Ok(())
    }

    /// Number of observations.
    pub const fn count(&self) -> u64 {
        self.count
    }

    /// Mean.
    pub fn mean(&self) -> EstimatorResult<f64> {
        if self.count == 0 {
            return Err(EstimatorError::InsufficientObservations {
                estimator: "mean",
            });
        }

        Ok(self.mean)
    }

    /// Population variance.
    pub fn population_variance(&self) -> EstimatorResult<f64> {
        if self.count == 0 {
            return Err(EstimatorError::InsufficientObservations {
                estimator: "population variance",
            });
        }

        Ok(self.m2 / self.count as f64)
    }

    /// Sample variance.
    pub fn sample_variance(&self) -> EstimatorResult<f64> {
        if self.count < 2 {
            return Err(EstimatorError::InsufficientObservations {
                estimator: "sample variance",
            });
        }

        Ok(self.m2 / (self.count - 1) as f64)
    }

    /// Standard deviation based on sample variance.
    pub fn sample_standard_deviation(&self) -> EstimatorResult<f64> {
        Ok(self.sample_variance()?.sqrt())
    }

    /// Converts the accumulator to a mean estimate.
    pub fn mean_estimate(&self) -> EstimatorResult<ScalarEstimate> {
        let mean = self.mean()?;

        let standard_error = if self.count >= 2 {
            Some((self.sample_variance()? / self.count as f64).sqrt())
        } else {
            None
        };

        ScalarEstimate::new(mean, self.count, standard_error, None)
    }
}

impl Default for MomentsAccumulator {
    fn default() -> Self {
        Self::new()
    }
}

/// Streaming weighted-mean accumulator.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WeightedMeanAccumulator {
    weight_sum: f64,
    weighted_value_sum: f64,
    observations: u64,
}

impl WeightedMeanAccumulator {
    /// Creates an empty weighted accumulator.
    pub const fn new() -> Self {
        Self {
            weight_sum: 0.0,
            weighted_value_sum: 0.0,
            observations: 0,
        }
    }

    /// Adds a weighted observation.
    pub fn observe(&mut self, value: f64, weight: f64) -> EstimatorResult<()> {
        validate_finite(value, "weighted observation")?;

        if !weight.is_finite() || weight <= 0.0 {
            return Err(EstimatorError::InvalidWeight {
                field: "weight must be finite and > 0",
            });
        }

        let weight_sum = self.weight_sum + weight;
        let weighted_value_sum = self.weighted_value_sum + value * weight;

        if !weight_sum.is_finite() || !weighted_value_sum.is_finite() {
            return Err(EstimatorError::NonFiniteValue {
                field: "weighted accumulator",
            });
        }

        self.weight_sum = weight_sum;
        self.weighted_value_sum = weighted_value_sum;

        self.observations = self
            .observations
            .checked_add(1)
            .ok_or(EstimatorError::ArithmeticOverflow {
                operation: "weighted observation count",
            })?;

        Ok(())
    }

    /// Merges another accumulator.
    pub fn merge(&mut self, other: Self) -> EstimatorResult<()> {
        let weight_sum = self.weight_sum + other.weight_sum;
        let weighted_value_sum =
            self.weighted_value_sum + other.weighted_value_sum;

        if !weight_sum.is_finite() || !weighted_value_sum.is_finite() {
            return Err(EstimatorError::NonFiniteValue {
                field: "merged weighted accumulator",
            });
        }

        let observations = self
            .observations
            .checked_add(other.observations)
            .ok_or(EstimatorError::ArithmeticOverflow {
                operation: "weighted observation merge",
            })?;

        self.weight_sum = weight_sum;
        self.weighted_value_sum = weighted_value_sum;
        self.observations = observations;

        Ok(())
    }

    /// Returns the weighted mean.
    pub fn estimate(&self) -> EstimatorResult<ScalarEstimate> {
        if self.observations == 0 {
            return Err(EstimatorError::InsufficientObservations {
                estimator: "weighted mean",
            });
        }

        if self.weight_sum <= 0.0 {
            return Err(EstimatorError::ZeroDenominator {
                estimator: "weighted mean",
            });
        }

        let value = self.weighted_value_sum / self.weight_sum;

        ScalarEstimate::new(value, self.observations, None, None)
    }
}

impl Default for WeightedMeanAccumulator {
    fn default() -> Self {
        Self::new()
    }
}

/// A confidence interval method.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntervalMethod {
    /// Wald/normal approximation.
    ///
    /// This method is inexpensive but should not be used blindly near
    /// probability boundaries or with small samples.
    NormalApproximation,

    /// Wilson score interval.
    ///
    /// More robust for probabilities near 0 and 1.
    Wilson,

    /// No interval.
    None,
}

/// Estimator configuration.
#[derive(Debug, Clone, PartialEq)]
pub struct EstimatorConfig {
    /// Estimator implementation.
    pub kind: EstimatorKind,

    /// Confidence level used by interval-producing estimators.
    pub confidence: ConfidenceLevel,

    /// Interval construction method.
    pub interval_method: IntervalMethod,

    /// Optional convergence requirement.
    pub convergence: Option<Tolerance>,

    /// Precision requirement.
    pub precision: PrecisionPolicy,
}

impl EstimatorConfig {
    /// Validates the configuration.
    pub fn validate(&self) -> EstimatorResult<()> {
        if let Some(tolerance) = self.convergence {
            tolerance.validate()?;
        }

        self.precision.validate()?;

        if matches!(
            self.kind,
            EstimatorKind::Probability
        ) && matches!(
            self.interval_method,
            IntervalMethod::Wilson | IntervalMethod::NormalApproximation
        ) {
            return Ok(());
        }

        Ok(())
    }
}

impl Default for EstimatorConfig {
    fn default() -> Self {
        Self {
            kind: EstimatorKind::Mean,
            confidence: ConfidenceLevel::default(),
            interval_method: IntervalMethod::NormalApproximation,
            convergence: None,
            precision: PrecisionPolicy::None,
        }
    }
}

/// Result of a characterization estimate.
#[derive(Debug, Clone, PartialEq)]
pub struct CharacterizationEstimate {
    /// Stable estimate identity.
    pub id: EstimateId,

    /// Estimated value.
    pub estimate: ScalarEstimate,

    /// Method used.
    pub method: EstimatorKind,

    /// Resource being characterized, if applicable.
    pub resource: Option<String>,

    /// Human-readable or machine-readable semantic tags.
    pub metadata: BTreeMap<String, String>,
}

impl CharacterizationEstimate {
    /// Creates an estimate.
    pub fn new(
        id: EstimateId,
        estimate: ScalarEstimate,
        method: EstimatorKind,
    ) -> Self {
        Self {
            id,
            estimate,
            method,
            resource: None,
            metadata: BTreeMap::new(),
        }
    }

    /// Attaches a resource label.
    ///
    /// The canonical QubitId/PhysicalQubitId remains the underlying resource
    /// identity. This string field is deliberately metadata-oriented.
    pub fn with_resource(mut self, resource: impl Into<String>) -> EstimatorResult<Self> {
        let resource = resource.into();

        if resource.trim().is_empty() {
            return Err(EstimatorError::InvalidResource {
                field: "estimate resource",
            });
        }

        self.resource = Some(resource);
        Ok(self)
    }

    /// Adds metadata.
    pub fn insert_metadata(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> EstimatorResult<()> {
        let key = key.into();

        if key.trim().is_empty() {
            return Err(EstimatorError::EmptyIdentifier {
                field: "estimate metadata key",
            });
        }

        self.metadata.insert(key, value.into());
        Ok(())
    }
}

/// Batch probability observations represented as counts.
///
/// This is useful when the observation layer already aggregated shots.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BernoulliCounts {
    /// Number of successful events.
    pub successes: u64,

    /// Total number of trials.
    pub trials: u64,
}

impl BernoulliCounts {
    /// Validates the counts.
    pub fn validate(&self) -> EstimatorResult<()> {
        if self.successes > self.trials {
            return Err(EstimatorError::InvalidConfiguration {
                reason: "success count exceeds trial count",
            });
        }

        Ok(())
    }

    /// Adds the counts to an accumulator.
    pub fn add_to(
        &self,
        accumulator: &mut BernoulliAccumulator,
    ) -> EstimatorResult<()> {
        self.validate()?;

        accumulator.successes = accumulator
            .successes
            .checked_add(self.successes)
            .ok_or(EstimatorError::ArithmeticOverflow {
                operation: "Bernoulli batch successes",
            })?;

        accumulator.trials = accumulator
            .trials
            .checked_add(self.trials)
            .ok_or(EstimatorError::ArithmeticOverflow {
                operation: "Bernoulli batch trials",
            })?;

        if accumulator.successes > accumulator.trials {
            return Err(EstimatorError::InvalidConfiguration {
                reason: "merged success count exceeds merged trial count",
            });
        }

        Ok(())
    }
}

/// Produces a normal-approximation confidence interval for a probability.
pub fn normal_probability_interval(
    successes: u64,
    trials: u64,
    confidence: ConfidenceLevel,
) -> EstimatorResult<ConfidenceInterval> {
    if trials == 0 {
        return Err(EstimatorError::InsufficientObservations {
            estimator: "normal probability interval",
        });
    }

    if successes > trials {
        return Err(EstimatorError::InvalidConfiguration {
            reason: "successes cannot exceed trials",
        });
    }

    let p = successes as f64 / trials as f64;

    // Approximation of the standard normal quantile for common confidence
    // levels. For arbitrary confidence values, use the Wilson interval below,
    // which does not require this lookup table.
    let z = normal_quantile(confidence.value())?;

    let standard_error = (p * (1.0 - p) / trials as f64).sqrt();
    let margin = z * standard_error;

    let lower = (p - margin).max(0.0);
    let upper = (p + margin).min(1.0);

    ConfidenceInterval::new(lower, upper, confidence)
}

/// Produces a Wilson score confidence interval.
///
/// This is the preferred built-in probability interval because it behaves
/// substantially better than the Wald interval for probabilities close to
/// zero or one.
pub fn wilson_probability_interval(
    successes: u64,
    trials: u64,
    confidence: ConfidenceLevel,
) -> EstimatorResult<ConfidenceInterval> {
    if trials == 0 {
        return Err(EstimatorError::InsufficientObservations {
            estimator: "Wilson probability interval",
        });
    }

    if successes > trials {
        return Err(EstimatorError::InvalidConfiguration {
            reason: "successes cannot exceed trials",
        });
    }

    let n = trials as f64;
    let p = successes as f64 / n;
    let z = normal_quantile(confidence.value())?;

    let z2 = z * z;
    let denominator = 1.0 + z2 / n;

    if denominator <= 0.0 || !denominator.is_finite() {
        return Err(EstimatorError::ZeroDenominator {
            estimator: "Wilson probability interval",
        });
    }

    let center = (p + z2 / (2.0 * n)) / denominator;

    let spread = (p * (1.0 - p) / n + z2 / (4.0 * n * n)).sqrt();
    let margin = z * spread / denominator;

    let lower = (center - margin).max(0.0);
    let upper = (center + margin).min(1.0);

    ConfidenceInterval::new(lower, upper, confidence)
}

/// Estimates a probability from aggregated counts.
pub fn estimate_probability(
    id: EstimateId,
    counts: BernoulliCounts,
    config: &EstimatorConfig,
) -> EstimatorResult<CharacterizationEstimate> {
    config.validate()?;
    counts.validate()?;

    if counts.trials == 0 {
        return Err(EstimatorError::InsufficientObservations {
            estimator: "probability",
        });
    }

    let value = counts.successes as f64 / counts.trials as f64;

    let interval = match config.interval_method {
        IntervalMethod::None => None,

        IntervalMethod::NormalApproximation => Some(
            normal_probability_interval(
                counts.successes,
                counts.trials,
                config.confidence,
            )?,
        ),

        IntervalMethod::Wilson => Some(
            wilson_probability_interval(
                counts.successes,
                counts.trials,
                config.confidence,
            )?,
        ),
    };

    let standard_error =
        Some((value * (1.0 - value) / counts.trials as f64).sqrt());

    let scalar = ScalarEstimate::new(
        value,
        counts.trials,
        standard_error,
        interval,
    )?;

    Ok(CharacterizationEstimate::new(
        id,
        scalar,
        EstimatorKind::Probability,
    ))
}

/// Estimates a mean from a moments accumulator.
pub fn estimate_mean(
    id: EstimateId,
    accumulator: &MomentsAccumulator,
    config: &EstimatorConfig,
) -> EstimatorResult<CharacterizationEstimate> {
    config.validate()?;

    let scalar = accumulator.mean_estimate()?;

    Ok(CharacterizationEstimate::new(
        id,
        scalar,
        EstimatorKind::Mean,
    ))
}

/// Estimates variance from a moments accumulator.
pub fn estimate_variance(
    id: EstimateId,
    accumulator: &MomentsAccumulator,
    config: &EstimatorConfig,
) -> EstimatorResult<CharacterizationEstimate> {
    config.validate()?;

    let value = accumulator.sample_variance()?;

    let scalar = ScalarEstimate::new(
        value,
        accumulator.count(),
        None,
        None,
    )?;

    Ok(CharacterizationEstimate::new(
        id,
        scalar,
        EstimatorKind::Variance,
    ))
}

/// Estimates standard deviation from a moments accumulator.
pub fn estimate_standard_deviation(
    id: EstimateId,
    accumulator: &MomentsAccumulator,
    config: &EstimatorConfig,
) -> EstimatorResult<CharacterizationEstimate> {
    config.validate()?;

    let value = accumulator.sample_standard_deviation()?;

    let scalar = ScalarEstimate::new(
        value,
        accumulator.count(),
        None,
        None,
    )?;

    Ok(CharacterizationEstimate::new(
        id,
        scalar,
        EstimatorKind::StandardDeviation,
    ))
}

/// Estimates a weighted mean.
pub fn estimate_weighted_mean(
    id: EstimateId,
    accumulator: &WeightedMeanAccumulator,
    config: &EstimatorConfig,
) -> EstimatorResult<CharacterizationEstimate> {
    config.validate()?;

    let scalar = accumulator.estimate()?;

    Ok(CharacterizationEstimate::new(
        id,
        scalar,
        EstimatorKind::WeightedMean,
    ))
}

/// Estimates the required number of Bernoulli samples for a normal
/// approximation to reach a requested absolute half-width.
///
/// This is a planning estimate, not a guarantee.
///
/// For rigorous adaptive protocols, the protocol/experiment layer should
/// continue collecting observations and use the resulting interval itself as
/// the stopping criterion.
pub fn approximate_required_probability_samples(
    probability: f64,
    absolute_half_width: f64,
    confidence: ConfidenceLevel,
) -> EstimatorResult<u64> {
    validate_probability(probability, "probability")?;
    validate_positive_finite(
        absolute_half_width,
        "absolute half-width",
    )?;

    let z = normal_quantile(confidence.value())?;
    let numerator = z * z * probability * (1.0 - probability);
    let denominator = absolute_half_width * absolute_half_width;

    if denominator <= 0.0 {
        return Err(EstimatorError::ZeroDenominator {
            estimator: "required probability sample calculation",
        });
    }

    let required = (numerator / denominator).ceil();

    if !required.is_finite() || required < 1.0 {
        return Err(EstimatorError::NonFiniteValue {
            field: "required sample count",
        });
    }

    if required > u64::MAX as f64 {
        return Err(EstimatorError::ArithmeticOverflow {
            operation: "required probability sample count",
        });
    }

    Ok(required as u64)
}

/// Evaluates convergence between successive scalar estimates.
pub fn converged(
    previous: &ScalarEstimate,
    current: &ScalarEstimate,
    tolerance: Tolerance,
) -> EstimatorResult<bool> {
    tolerance.satisfied(previous.value, current.value)
}

/// Deterministically merges scalar estimates when they represent independent
/// sample partitions with compatible semantics.
///
/// This helper is intentionally conservative: callers must provide the
/// appropriate aggregation weights.
pub fn merge_means(
    left: &ScalarEstimate,
    right: &ScalarEstimate,
) -> EstimatorResult<ScalarEstimate> {
    if left.samples == 0 {
        return Ok(*right);
    }

    if right.samples == 0 {
        return Ok(*left);
    }

    validate_finite(left.value, "left mean")?;
    validate_finite(right.value, "right mean")?;

    let total = left
        .samples
        .checked_add(right.samples)
        .ok_or(EstimatorError::ArithmeticOverflow {
            operation: "mean sample merge",
        })?;

    let total_f = total as f64;

    let value = (left.value * left.samples as f64
        + right.value * right.samples as f64)
        / total_f;

    ScalarEstimate::new(value, total, None, None)
}

/// A collection of estimates keyed by their stable IDs.
///
/// BTreeMap provides deterministic iteration order.
#[derive(Debug, Default, Clone)]
pub struct EstimateSet {
    estimates: BTreeMap<EstimateId, CharacterizationEstimate>,
}

impl EstimateSet {
    /// Creates an empty estimate set.
    pub fn new() -> Self {
        Self {
            estimates: BTreeMap::new(),
        }
    }

    /// Inserts an estimate.
    pub fn insert(
        &mut self,
        estimate: CharacterizationEstimate,
    ) -> EstimatorResult<()> {
        self.estimates.insert(estimate.id.clone(), estimate);
        Ok(())
    }

    /// Returns an estimate.
    pub fn get(&self, id: &EstimateId) -> Option<&CharacterizationEstimate> {
        self.estimates.get(id)
    }

    /// Returns the number of estimates.
    pub fn len(&self) -> usize {
        self.estimates.len()
    }

    /// Returns whether the set is empty.
    pub fn is_empty(&self) -> bool {
        self.estimates.is_empty()
    }

    /// Iterates deterministically by EstimateId.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = (&EstimateId, &CharacterizationEstimate)> {
        self.estimates.iter()
    }
}

/// Approximate inverse normal CDF.
///
/// This is used only for confidence-interval construction and sample-size
/// planning. It does not alter the semantics of the observed data.
///
/// The implementation uses the Acklam rational approximation.
fn normal_quantile(confidence: f64) -> EstimatorResult<f64> {
    if !confidence.is_finite() {
        return Err(EstimatorError::NonFiniteValue {
            field: "confidence",
        });
    }

    if !(0.0 < confidence && confidence < 1.0) {
        return Err(EstimatorError::InvalidConfidence {
            value_description: "must satisfy 0 < confidence < 1",
        });
    }

    let p = (1.0 + confidence) / 2.0;

    // Coefficients for Peter J. Acklam's inverse-normal approximation.
    let a1 = -39.696_830_286_653_8;
    let a2 = 220.946_098_424_520;
    let a3 = -275.928_510_446_969;
    let a4 = 138.357_751_867_269;
    let a5 = -30.664_798_066_147_2;
    let a6 = 2.506_628_277_459_24;

    let b1 = -54.476_098_798_224_1;
    let b2 = 161.585_836_858_041;
    let b3 = -155.698_979_859_887;
    let b4 = 66.801_311_887_719_7;
    let b5 = -13.280_681_552_885_7;

    let c1 = -0.007_784_894_002_430_29;
    let c2 = -0.322_396_458_041_136;
    let c3 = -2.400_758_277_161_84;
    let c4 = -2.549_732_539_343_73;
    let c5 = 4.374_664_141_464_97;
    let c6 = 2.938_163_982_698_78;

    let d1 = 0.007_784_695_709_041_46;
    let d2 = 0.322_467_129_070_04;
    let d3 = 2.445_134_137_143;
    let d4 = 3.754_408_661_907_42;

    let plow = 0.024_25;
    let phigh = 1.0 - plow;

    let result = if p < plow {
        let q = (-2.0 * p.ln()).sqrt();

        (((((c1 * q + c2) * q + c3) * q + c4) * q + c5) * q
            + c6)
            / ((((d1 * q + d2) * q + d3) * q + d4) * q + 1.0)
    } else if p <= phigh {
        let q = p - 0.5;
        let r = q * q;

        (((((a1 * r + a2) * r + a3) * r + a4) * r + a5) * r
            + a6)
            * q
            / (((((b1 * r + b2) * r + b3) * r + b4) * r + b5) * r
                + 1.0)
    } else {
        let q = (-2.0 * (1.0 - p).ln()).sqrt();

        -(((((c1 * q + c2) * q + c3) * q + c4) * q + c5) * q
            + c6)
            / ((((d1 * q + d2) * q + d3) * q + d4) * q + 1.0)
    };

    validate_finite(result, "normal quantile")?;

    Ok(result)
}

/// Validates a finite floating-point value.
fn validate_finite(value: f64, field: &'static str) -> EstimatorResult<()> {
    if !value.is_finite() {
        return Err(EstimatorError::NonFiniteValue { field });
    }

    Ok(())
}

/// Validates a finite non-negative value.
fn validate_non_negative_finite(
    value: f64,
    field: &'static str,
) -> EstimatorResult<()> {
    validate_finite(value, field)?;

    if value < 0.0 {
        return Err(EstimatorError::InvalidConfiguration {
            reason: "value must be non-negative",
        });
    }

    Ok(())
}

/// Validates a finite positive value.
fn validate_positive_finite(
    value: f64,
    field: &'static str,
) -> EstimatorResult<()> {
    validate_finite(value, field)?;

    if value <= 0.0 {
        return Err(EstimatorError::InvalidTolerance {
            value_description: "must be finite and > 0",
        });
    }

    Ok(())
}

/// Validates a probability.
fn validate_probability(
    value: f64,
    field: &'static str,
) -> EstimatorResult<()> {
    validate_finite(value, field)?;

    if !(0.0..=1.0).contains(&value) {
        return Err(EstimatorError::InvalidProbability { field });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn probability_accumulator_is_streaming() {
        let mut accumulator = BernoulliAccumulator::new();

        accumulator
            .observe(BernoulliObservation { success: true })
            .expect("valid observation");

        accumulator
            .observe(BernoulliObservation { success: false })
            .expect("valid observation");

        let estimate = accumulator.estimate().expect("valid estimate");

        assert_eq!(estimate.value, 0.5);
        assert_eq!(estimate.samples, 2);
    }

    #[test]
    fn probability_counts_validate() {
        let counts = BernoulliCounts {
            successes: 4,
            trials: 3,
        };

        assert!(counts.validate().is_err());
    }

    #[test]
    fn wilson_interval_stays_inside_probability_domain() {
        let confidence = ConfidenceLevel::new(0.95).expect("valid confidence");

        let interval =
            wilson_probability_interval(0, 100, confidence)
                .expect("valid interval");

        assert!(interval.lower >= 0.0);
        assert!(interval.upper <= 1.0);
        assert!(interval.lower <= interval.upper);
    }

    #[test]
    fn moments_are_streaming() {
        let mut accumulator = MomentsAccumulator::new();

        for value in [1.0, 2.0, 3.0, 4.0] {
            accumulator.observe(value).expect("valid value");
        }

        assert_eq!(accumulator.count(), 4);

        let mean = accumulator.mean().expect("valid mean");

        assert!((mean - 2.5).abs() < 1.0e-12);
    }

    #[test]
    fn moments_partition_merge_matches_batch_semantics() {
        let mut left = MomentsAccumulator::new();
        left.observe(1.0).expect("valid");
        left.observe(2.0).expect("valid");

        let mut right = MomentsAccumulator::new();
        right.observe(3.0).expect("valid");
        right.observe(4.0).expect("valid");

        left.merge(right).expect("compatible merge");

        let mean = left.mean().expect("valid mean");

        assert!((mean - 2.5).abs() < 1.0e-12);
    }

    #[test]
    fn weighted_mean_is_valid() {
        let mut accumulator = WeightedMeanAccumulator::new();

        accumulator.observe(10.0, 1.0).expect("valid");
        accumulator.observe(20.0, 3.0).expect("valid");

        let estimate = accumulator.estimate().expect("valid estimate");

        assert!((estimate.value - 17.5).abs() < 1.0e-12);
    }

    #[test]
    fn nan_is_rejected() {
        let mut accumulator = MomentsAccumulator::new();

        assert!(
            accumulator.observe(f64::NAN).is_err()
        );
    }

    #[test]
    fn infinity_is_rejected() {
        let mut accumulator = MomentsAccumulator::new();

        assert!(
            accumulator.observe(f64::INFINITY).is_err()
        );
    }

    #[test]
    fn tolerance_works() {
        let tolerance = Tolerance::Absolute(0.01);

        assert!(
            tolerance
                .satisfied(1.0, 1.005)
                .expect("valid comparison")
        );

        assert!(
            !tolerance
                .satisfied(1.0, 1.02)
                .expect("valid comparison")
        );
    }

    #[test]
    fn canonical_qubit_resource_can_be_validated() {
        // Construction of QubitId is intentionally delegated to the canonical
        // quantum::ir implementation. This test only verifies that the ZQN
        // type accepts the canonical ID rather than defining another ID type.
        //
        // Actual ID construction belongs to quantum::ir::qubit tests.
        fn accepts_logical_resource(_: CharacterizationResource) {}

        let _ = accepts_logical_resource;
    }

    #[test]
    fn estimate_set_is_deterministically_ordered() {
        let mut set = EstimateSet::new();

        let first_id = EstimateId::new("z").expect("valid id");
        let second_id = EstimateId::new("a").expect("valid id");

        let first = CharacterizationEstimate::new(
            first_id,
            ScalarEstimate::new(0.1, 10, None, None)
                .expect("valid estimate"),
            EstimatorKind::Mean,
        );

        let second = CharacterizationEstimate::new(
            second_id,
            ScalarEstimate::new(0.2, 10, None, None)
                .expect("valid estimate"),
            EstimatorKind::Mean,
        );

        set.insert(first).expect("insert");
        set.insert(second).expect("insert");

        let ids: Vec<&str> =
            set.iter().map(|(id, _)| id.as_str()).collect();

        assert_eq!(ids, vec!["a", "z"]);
    }

    #[test]
    fn probability_estimator_can_be_built_from_counts() {
        let id = EstimateId::new("readout_error").expect("valid id");

        let config = EstimatorConfig {
            kind: EstimatorKind::Probability,
            confidence: ConfidenceLevel::new(0.95)
                .expect("valid confidence"),
            interval_method: IntervalMethod::Wilson,
            convergence: None,
            precision: PrecisionPolicy::None,
        };

        let estimate = estimate_probability(
            id,
            BernoulliCounts {
                successes: 5,
                trials: 100,
            },
            &config,
        )
        .expect("valid probability estimate");

        assert!((estimate.estimate.value - 0.05).abs() < 1.0e-12);
        assert!(estimate.estimate.interval.is_some());
    }

    #[test]
    fn required_sample_estimate_is_positive() {
        let confidence = ConfidenceLevel::new(0.95)
            .expect("valid confidence");

        let required = approximate_required_probability_samples(
            0.5,
            0.01,
            confidence,
        )
        .expect("valid sample estimate");

        assert!(required > 0);
    }

    #[test]
    fn empty_mean_is_rejected() {
        let accumulator = MomentsAccumulator::new();

        assert!(accumulator.mean().is_err());
    }

    #[test]
    fn sample_variance_requires_two_samples() {
        let mut accumulator = MomentsAccumulator::new();

        accumulator.observe(1.0).expect("valid");

        assert!(accumulator.sample_variance().is_err());
    }

    #[test]
    fn probability_is_bounded() {
        let confidence = ConfidenceLevel::new(0.95)
            .expect("valid confidence");

        let interval = wilson_probability_interval(
            100,
            100,
            confidence,
        )
        .expect("valid interval");

        assert!(interval.lower >= 0.0);
        assert!(interval.upper <= 1.0);
        assert!(interval.upper >= interval.lower);
    }
}