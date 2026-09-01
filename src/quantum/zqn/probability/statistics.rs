//! Zamani Quantum Noise (ZQN) — Probability Statistics.
//!
//! Production statistical foundation for ZQN probability distributions and
//! weighted observation streams.
//!
//! # Ownership
//!
//! This file owns:
//!
//! - numerically stable weighted statistics;
//! - streaming weighted mean;
//! - weighted population variance;
//! - weighted standard deviation;
//! - second raw moment;
//! - minimum and maximum;
//! - total weight;
//! - effective sample size;
//! - Shannon entropy;
//! - distribution-level descriptive statistics;
//! - deterministic statistical accumulation;
//! - mergeable statistical accumulators;
//! - explicit numerical validation;
//! - statistical-domain errors.
//!
//! This file does NOT own:
//!
//! - the mathematical definition of `Probability`;
//! - construction of `Distribution`;
//! - RNGs;
//! - sampling;
//! - confidence intervals;
//! - Bayesian inference;
//! - calibration;
//! - quantum channels;
//! - faults;
//! - noise models;
//! - QEC;
//! - benchmarking protocols;
//! - hardware;
//! - qubit identity;
//! - resource-policy ownership;
//! - serialization schemas.
//!
//! Those concerns remain in their respective owning subsystems.
//!
//! # Architectural position
//!
//! ```text
//! probability.rs
//!      │
//!      ▼
//! distribution.rs
//!      │
//!      ▼
//! statistics.rs
//!      │
//!      ├───────────────┐
//!      ▼               ▼
//! characterization   benchmarking
//!      │
//!      ▼
//!     ZQN
//! ```
//!
//! `statistics.rs` is therefore a consumer of probability semantics.
//!
//! # Canonical quantum identity
//!
//! This file intentionally does not define or import another `QubitId`.
//!
//! If a higher-level statistical result is associated with a quantum resource,
//! that higher layer must use:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! No duplicate quantum identity is permitted here.
//!
//! # Write once, scale everywhere
//!
//! There is no semantic maximum for:
//!
//! - number of observations;
//! - number of distribution outcomes;
//! - number of qubits;
//! - number of resources;
//! - number of operations.
//!
//! The accumulator is O(1) in additional memory.
//!
//! The practical limits are determined by:
//!
//! - the input iterator;
//! - the host's numerical representation;
//! - available execution resources;
//! - explicit runtime/resource policy.
//!
//! A `u128` count is used for observation counts. This is a representation
//! choice, not a quantum-machine-size limit.
//!
//! # Numerical policy
//!
//! All observation values and positive weights must be finite.
//!
//! Weights must satisfy:
//!
//! ```text
//! weight >= 0
//! ```
//!
//! Zero weight is accepted and ignored.
//!
//! Negative, NaN and infinite weights are rejected.
//!
//! No invalid numerical value is silently:
//!
//! - clamped;
//! - converted;
//! - normalized;
//! - discarded.
//!
//! # Variance
//!
//! Weighted variance is computed with a weighted Welford/Pébay-style online
//! recurrence rather than by directly calculating:
//!
//! ```text
//! E[X²] - E[X]²
//! ```
//!
//! This substantially reduces catastrophic cancellation for large offsets.
//!
//! The mathematical population variance is:
//!
//! ```text
//! Var(X) = Σ wᵢ (xᵢ - μ)² / Σ wᵢ
//! ```
//!
//! A tiny negative floating-point residual may occur because of roundoff.
//! Such a residual is converted to zero only when it is demonstrably within
//! an explicit machine-rounding bound. A materially negative result is an
//! error and is never hidden.
//!
//! # Parallel and distributed reduction
//!
//! `merge()` permits independently accumulated statistics to be combined
//! without retaining the original observations.
//!
//! This is important for:
//!
//! - parallel simulation;
//! - distributed characterization;
//! - streaming telemetry;
//! - large benchmark workloads.
//!
//! The order of floating-point reduction can affect the last representable
//! bits. Therefore bit-for-bit deterministic distributed execution requires
//! the caller to impose a deterministic merge order.
//!
//! # Entropy
//!
//! Shannon entropy is:
//!
//! ```text
//! H(P) = -Σ pᵢ ln(pᵢ)
//! ```
//!
//! `entropy_nats()` returns nats.
//!
//! `entropy_bits()` returns bits.
//!
//! Zero-probability terms contribute zero entropy and are valid.
//!
//! # Effective sample size
//!
//! For positive weights:
//!
//! ```text
//! ESS = (Σw)² / Σw²
//! ```
//!
//! This is a diagnostic of weight concentration, not an assertion that the
//! observations are IID.
//!
//! # Determinism
//!
//! This module owns no RNG and performs no sampling.
//!
//! Given the same ordered input and same floating-point environment, the
//! result is deterministic.
//!
//! Parallel callers must use deterministic reduction ordering when exact
//! reproducibility is required.
//!
//! # Resource safety
//!
//! The core accumulator:
//!
//! - performs no allocation proportional to input size;
//! - performs no recursion;
//! - performs no I/O;
//! - performs no network access;
//! - has no global mutable state;
//! - has no global RNG;
//! - contains no unsafe Rust.
//!
//! # Serialization
//!
//! Statistical results do not define an external wire format here.
//!
//! Versioned serialization belongs to `zqn::io`.
//!
//! # Integration
//!
//! Direct distribution integration is provided by:
//!
//! ```text
//! DistributionStatistics::from_distribution()
//! ```
//!
//! Generic streaming integration is provided by:
//!
//! ```text
//! WeightedStatistics::from_iter()
//! ```
//!
//! Therefore characterization, simulation and benchmarking can consume
//! statistics without depending on the internal representation of
//! `Distribution<T>`.
//!
//! # Rust compatibility
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! # Definition of done
//!
//! This file is complete when:
//!
//! 1. all numerical inputs are validated;
//! 2. statistics are streaming;
//! 3. weighted variance is numerically stable;
//! 4. invalid variance is never silently hidden;
//! 5. distribution validation uses caller-supplied tolerance;
//! 6. entropy is deterministic;
//! 7. effective sample size is validated;
//! 8. accumulators can be merged;
//! 9. no RNG is owned;
//! 10. no qubit identity is duplicated;
//! 11. no machine-size limit is hard-coded;
//! 12. no unsafe Rust exists;
//! 13. public APIs depend only on stable sibling contracts;
//! 14. tests cover edge cases, numerical stability, merging and scaling.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;

use super::distribution::{Distribution, DistributionError};
use super::probability::Probability;

/// Portable observation count.
///
/// This is a count representation and is not a semantic limit on a quantum
/// computation.
pub type ObservationCount = u128;

/// Statistical errors produced by ZQN probability analysis.
#[derive(Clone, Debug, PartialEq)]
pub enum StatisticsError {
    /// No positive-weight observations were supplied.
    Empty,

    /// An observation value is NaN or infinite.
    NonFiniteObservation {
        value: f64,
    },

    /// A weight is NaN or infinite.
    NonFiniteWeight {
        weight: f64,
    },

    /// A weight is negative.
    NegativeWeight {
        weight: f64,
    },

    /// Weight accumulation exceeded finite `f64` representation.
    WeightOverflow {
        total: f64,
    },

    /// Total positive weight is zero.
    ZeroTotalWeight,

    /// A calculated statistic is not finite.
    NonFiniteStatistic {
        name: &'static str,
        value: f64,
    },

    /// Variance is materially negative.
    NegativeVariance {
        value: f64,
    },

    /// A supplied tolerance is invalid.
    InvalidTolerance {
        tolerance: f64,
    },

    /// A probability value is invalid.
    InvalidProbability {
        value: f64,
    },

    /// The source distribution failed validation.
    Distribution(DistributionError),

    /// Observation count overflowed its portable representation.
    CountOverflow,

    /// The merge operation received incompatible numerical state.
    IncompatibleAccumulator,

    /// A normalized probability stream did not sum to one.
    NotNormalized {
        total: f64,
        tolerance: f64,
    },
}

impl fmt::Display for StatisticsError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => {
                formatter.write_str("statistical input is empty")
            }

            Self::NonFiniteObservation { value } => {
                write!(
                    formatter,
                    "observation is not finite: {value}"
                )
            }

            Self::NonFiniteWeight { weight } => {
                write!(
                    formatter,
                    "weight is not finite: {weight}"
                )
            }

            Self::NegativeWeight { weight } => {
                write!(
                    formatter,
                    "weight is negative: {weight}"
                )
            }

            Self::WeightOverflow { total } => {
                write!(
                    formatter,
                    "weight accumulation became non-finite: {total}"
                )
            }

            Self::ZeroTotalWeight => {
                formatter.write_str("total positive weight is zero")
            }

            Self::NonFiniteStatistic { name, value } => {
                write!(
                    formatter,
                    "statistic {name} is not finite: {value}"
                )
            }

            Self::NegativeVariance { value } => {
                write!(
                    formatter,
                    "variance is materially negative: {value}"
                )
            }

            Self::InvalidTolerance { tolerance } => {
                write!(
                    formatter,
                    "tolerance must be finite and non-negative: {tolerance}"
                )
            }

            Self::InvalidProbability { value } => {
                write!(
                    formatter,
                    "invalid probability: {value}"
                )
            }

            Self::Distribution(error) => {
                error.fmt(formatter)
            }

            Self::CountOverflow => {
                formatter.write_str("observation count overflow")
            }

            Self::IncompatibleAccumulator => {
                formatter.write_str(
                    "statistical accumulators contain incompatible numerical state",
                )
            }

            Self::NotNormalized {
                total,
                tolerance,
            } => {
                write!(
                    formatter,
                    "probability total {total} is outside tolerance {tolerance}"
                )
            }
        }
    }
}

impl std::error::Error for StatisticsError {}

impl From<DistributionError> for StatisticsError {
    fn from(error: DistributionError) -> Self {
        Self::Distribution(error)
    }
}

/// Numerically stable streaming weighted statistics.
///
/// The accumulator stores a fixed number of scalars regardless of how many
/// observations are consumed.
///
/// The variance is population-style weighted variance, not an unbiased sample
/// estimator.
#[derive(Clone, Copy, Debug)]
pub struct WeightedStatistics {
    count: ObservationCount,
    total_weight: f64,
    mean: f64,
    second_central_moment: f64,
    min: f64,
    max: f64,
    sum_weight_squared: f64,
}

impl WeightedStatistics {
    /// Creates an empty accumulator.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            count: 0,
            total_weight: 0.0,
            mean: 0.0,
            second_central_moment: 0.0,
            min: 0.0,
            max: 0.0,
            sum_weight_squared: 0.0,
        }
    }

    /// Returns the number of positive-weight observations.
    #[must_use]
    pub const fn count(&self) -> ObservationCount {
        self.count
    }

    /// Returns the accumulated positive weight.
    #[must_use]
    pub const fn total_weight(&self) -> f64 {
        self.total_weight
    }

    /// Returns the weighted mean.
    pub fn mean(&self) -> Result<f64, StatisticsError> {
        self.require_non_empty()?;
        finite_statistic("mean", self.mean)
    }

    /// Returns the weighted population variance.
    ///
    /// This is:
    ///
    /// `Σ wᵢ(xᵢ - μ)² / Σwᵢ`
    pub fn variance(&self) -> Result<f64, StatisticsError> {
        self.require_non_empty()?;

        let variance = self.second_central_moment / self.total_weight;

        if !variance.is_finite() {
            return Err(StatisticsError::NonFiniteStatistic {
                name: "variance",
                value: variance,
            });
        }

        if variance >= 0.0 {
            return Ok(variance);
        }

        // A tiny negative result can arise solely from floating-point
        // cancellation in a long stream or a merged reduction.
        //
        // We do not clamp arbitrary negative values. Only a residual inside
        // a conservative machine-rounding envelope is accepted as zero.
        let scale = self
            .second_central_moment
            .abs()
            .max(self.mean.abs() * self.mean.abs())
            .max(1.0);

        let rounding_bound = f64::EPSILON * scale * 32.0;

        if variance >= -rounding_bound {
            Ok(0.0)
        } else {
            Err(StatisticsError::NegativeVariance {
                value: variance,
            })
        }
    }

    /// Returns the weighted population standard deviation.
    pub fn standard_deviation(&self) -> Result<f64, StatisticsError> {
        let variance = self.variance()?;
        let deviation = variance.sqrt();

        finite_statistic("standard_deviation", deviation)
    }

    /// Returns the second raw moment:
    ///
    /// `E[X²]`.
    pub fn second_moment(&self) -> Result<f64, StatisticsError> {
        let mean = self.mean()?;
        let variance = self.variance()?;

        let second = variance + mean * mean;

        finite_statistic("second_moment", second)
    }

    /// Returns the smallest observed value.
    pub fn min(&self) -> Result<f64, StatisticsError> {
        self.require_non_empty()?;
        finite_statistic("min", self.min)
    }

    /// Returns the largest observed value.
    pub fn max(&self) -> Result<f64, StatisticsError> {
        self.require_non_empty()?;
        finite_statistic("max", self.max)
    }

    /// Returns the effective sample size:
    ///
    /// `ESS = (Σw)² / Σw²`.
    pub fn effective_sample_size(&self) -> Result<f64, StatisticsError> {
        self.require_non_empty()?;

        if self.sum_weight_squared <= 0.0
            || !self.sum_weight_squared.is_finite()
        {
            return Err(StatisticsError::NonFiniteStatistic {
                name: "sum_weight_squared",
                value: self.sum_weight_squared,
            });
        }

        let squared_total = self.total_weight * self.total_weight;

        if !squared_total.is_finite() {
            return Err(StatisticsError::NonFiniteStatistic {
                name: "squared_total_weight",
                value: squared_total,
            });
        }

        let ess = squared_total / self.sum_weight_squared;

        finite_statistic("effective_sample_size", ess)
    }

    /// Adds one observation.
    ///
    /// A zero weight is valid and contributes nothing.
    pub fn add(
        &mut self,
        value: f64,
        weight: f64,
    ) -> Result<(), StatisticsError> {
        validate_observation(value)?;
        validate_weight(weight)?;

        if weight == 0.0 {
            return Ok(());
        }

        let new_total = self
            .total_weight
            .checked_add(weight)
            .ok_or(StatisticsError::WeightOverflow {
                total: f64::INFINITY,
            })?;

        if !new_total.is_finite() {
            return Err(StatisticsError::WeightOverflow {
                total: new_total,
            });
        }

        let weight_squared = weight * weight;

        if !weight_squared.is_finite() {
            return Err(StatisticsError::NonFiniteStatistic {
                name: "weight_squared",
                value: weight_squared,
            });
        }

        let new_sum_weight_squared = self
            .sum_weight_squared
            .checked_add(weight_squared)
            .ok_or(StatisticsError::WeightOverflow {
                total: self.sum_weight_squared,
            })?;

        if !new_sum_weight_squared.is_finite() {
            return Err(StatisticsError::WeightOverflow {
                total: new_sum_weight_squared,
            });
        }

        if self.count == 0 {
            self.count = 1;
            self.total_weight = weight;
            self.mean = value;
            self.second_central_moment = 0.0;
            self.min = value;
            self.max = value;
            self.sum_weight_squared = weight_squared;

            return Ok(());
        }

        let delta = value - self.mean;

        if !delta.is_finite() {
            return Err(StatisticsError::NonFiniteStatistic {
                name: "delta",
                value: delta,
            });
        }

        let relative_weight = weight / new_total;

        if !relative_weight.is_finite() {
            return Err(StatisticsError::NonFiniteStatistic {
                name: "relative_weight",
                value: relative_weight,
            });
        }

        let new_mean = self.mean + relative_weight * delta;

        if !new_mean.is_finite() {
            return Err(StatisticsError::NonFiniteStatistic {
                name: "mean",
                value: new_mean,
            });
        }

        let contribution = weight * delta * (value - new_mean);

        if !contribution.is_finite() {
            return Err(StatisticsError::NonFiniteStatistic {
                name: "variance_contribution",
                value: contribution,
            });
        }

        let new_m2 = self
            .second_central_moment
            .checked_add(contribution)
            .ok_or(StatisticsError::NonFiniteStatistic {
                name: "second_central_moment",
                value: f64::INFINITY,
            })?;

        if !new_m2.is_finite() {
            return Err(StatisticsError::NonFiniteStatistic {
                name: "second_central_moment",
                value: new_m2,
            });
        }

        let new_count = self
            .count
            .checked_add(1)
            .ok_or(StatisticsError::CountOverflow)?;

        // Do not clamp `new_m2` here.
        //
        // A negative residual must remain visible until `variance()`, where
        // it can be distinguished from legitimate numerical roundoff.
        self.count = new_count;
        self.total_weight = new_total;
        self.mean = new_mean;
        self.second_central_moment = new_m2;
        self.min = self.min.min(value);
        self.max = self.max.max(value);
        self.sum_weight_squared = new_sum_weight_squared;

        Ok(())
    }

    /// Consumes an iterator of `(value, weight)` observations.
    ///
    /// The iterator is consumed exactly once.
    pub fn from_iter<I>(
        iter: I,
    ) -> Result<Self, StatisticsError>
    where
        I: IntoIterator<Item = (f64, f64)>,
    {
        let mut statistics = Self::new();

        for (value, weight) in iter {
            statistics.add(value, weight)?;
        }

        statistics.finish()
    }

    /// Finalizes the accumulator and rejects empty positive-weight input.
    pub fn finish(self) -> Result<Self, StatisticsError> {
        self.require_non_empty()?;
        Ok(self)
    }

    /// Merges another accumulator into this accumulator.
    ///
    /// This is the weighted parallel-combine operation. It does not require
    /// retaining either source observation stream.
    ///
    /// The merge formula is:
    ///
    /// ```text
    /// δ = μ₂ - μ₁
    /// μ = μ₁ + δ * W₂ / (W₁ + W₂)
    /// M₂ = M₂₁ + M₂₂ + δ² W₁ W₂ / (W₁ + W₂)
    /// ```
    ///
    /// `other` is copied; neither accumulator is mutated through aliasing.
    pub fn merge(
        &mut self,
        other: &Self,
    ) -> Result<(), StatisticsError> {
        if other.count == 0 {
            return Ok(());
        }

        other.validate_internal_state()?;

        if self.count == 0 {
            *self = *other;
            return Ok(());
        }

        self.validate_internal_state()?;

        let combined_weight = self
            .total_weight
            .checked_add(other.total_weight)
            .ok_or(StatisticsError::WeightOverflow {
                total: f64::INFINITY,
            })?;

        if !combined_weight.is_finite()
            || combined_weight <= 0.0
        {
            return Err(StatisticsError::WeightOverflow {
                total: combined_weight,
            });
        }

        let delta = other.mean - self.mean;

        if !delta.is_finite() {
            return Err(StatisticsError::NonFiniteStatistic {
                name: "merge_delta",
                value: delta,
            });
        }

        let weight_ratio = other.total_weight / combined_weight;

        if !weight_ratio.is_finite() {
            return Err(StatisticsError::NonFiniteStatistic {
                name: "merge_weight_ratio",
                value: weight_ratio,
            });
        }

        let combined_mean = self.mean + delta * weight_ratio;

        if !combined_mean.is_finite() {
            return Err(StatisticsError::NonFiniteStatistic {
                name: "merged_mean",
                value: combined_mean,
            });
        }

        let cross_weight =
            self.total_weight
                * other.total_weight
                / combined_weight;

        if !cross_weight.is_finite() {
            return Err(StatisticsError::NonFiniteStatistic {
                name: "merge_cross_weight",
                value: cross_weight,
            });
        }

        let cross_term = delta * delta * cross_weight;

        if !cross_term.is_finite() {
            return Err(StatisticsError::NonFiniteStatistic {
                name: "merge_cross_term",
                value: cross_term,
            });
        }

        let combined_m2 = self
            .second_central_moment
            .checked_add(other.second_central_moment)
            .ok_or(StatisticsError::NonFiniteStatistic {
                name: "merged_second_central_moment",
                value: f64::INFINITY,
            })?
            .checked_add(cross_term)
            .ok_or(StatisticsError::NonFiniteStatistic {
                name: "merged_second_central_moment",
                value: f64::INFINITY,
            })?;

        if !combined_m2.is_finite() {
            return Err(StatisticsError::NonFiniteStatistic {
                name: "merged_second_central_moment",
                value: combined_m2,
            });
        }

        let combined_count = self
            .count
            .checked_add(other.count)
            .ok_or(StatisticsError::CountOverflow)?;

        let combined_weight_squared = self
            .sum_weight_squared
            .checked_add(other.sum_weight_squared)
            .ok_or(StatisticsError::NonFiniteStatistic {
                name: "merged_sum_weight_squared",
                value: f64::INFINITY,
            })?;

        if !combined_weight_squared.is_finite() {
            return Err(StatisticsError::NonFiniteStatistic {
                name: "merged_sum_weight_squared",
                value: combined_weight_squared,
            });
        }

        self.count = combined_count;
        self.total_weight = combined_weight;
        self.mean = combined_mean;
        self.second_central_moment = combined_m2;
        self.min = self.min.min(other.min);
        self.max = self.max.max(other.max);
        self.sum_weight_squared = combined_weight_squared;

        Ok(())
    }

    fn require_non_empty(&self) -> Result<(), StatisticsError> {
        if self.count == 0 || self.total_weight <= 0.0 {
            return Err(StatisticsError::Empty);
        }

        Ok(())
    }

    fn validate_internal_state(&self) -> Result<(), StatisticsError> {
        if self.count == 0 {
            if self.total_weight != 0.0
                || self.sum_weight_squared != 0.0
            {
                return Err(StatisticsError::IncompatibleAccumulator);
            }

            return Ok(());
        }

        if !self.total_weight.is_finite()
            || self.total_weight <= 0.0
            || !self.mean.is_finite()
            || !self.second_central_moment.is_finite()
            || !self.min.is_finite()
            || !self.max.is_finite()
            || !self.sum_weight_squared.is_finite()
            || self.sum_weight_squared <= 0.0
        {
            return Err(StatisticsError::IncompatibleAccumulator);
        }

        Ok(())
    }
}

impl Default for WeightedStatistics {
    fn default() -> Self {
        Self::new()
    }
}

/// Fixed-size statistics for a validated ZQN probability distribution.
#[derive(Clone, Copy, Debug)]
pub struct DistributionStatistics {
    weighted: WeightedStatistics,
    entropy_nats: f64,
}

impl DistributionStatistics {
    /// Computes statistics directly from a `Distribution<T>`.
    ///
    /// The distribution is validated with the caller-supplied tolerance.
    /// No implicit renormalization occurs.
    pub fn from_distribution<T>(
        distribution: &Distribution<T>,
        tolerance: f64,
    ) -> Result<Self, StatisticsError>
    where
        T: Copy + Into<f64>,
    {
        validate_tolerance(tolerance)?;
        distribution.validate(tolerance)?;

        let mut weighted = WeightedStatistics::new();
        let mut entropy_nats = 0.0_f64;

        for (outcome, probability) in distribution.iter() {
            let value = (*outcome).into();
            let p = probability.get();

            weighted.add(value, p)?;

            if !p.is_finite() || p <= 0.0 || p > 1.0 {
                return Err(StatisticsError::InvalidProbability {
                    value: p,
                });
            }

            let contribution = -p * p.ln();

            if !contribution.is_finite() {
                return Err(StatisticsError::NonFiniteStatistic {
                    name: "entropy_nats",
                    value: contribution,
                });
            }

            entropy_nats = entropy_nats
                .checked_add(contribution)
                .ok_or(StatisticsError::NonFiniteStatistic {
                    name: "entropy_nats",
                    value: f64::INFINITY,
                })?;

            if !entropy_nats.is_finite() {
                return Err(StatisticsError::NonFiniteStatistic {
                    name: "entropy_nats",
                    value: entropy_nats,
                });
            }
        }

        Ok(Self {
            weighted: weighted.finish()?,
            entropy_nats,
        })
    }

    /// Returns the underlying weighted statistics.
    #[must_use]
    pub const fn weighted(&self) -> WeightedStatistics {
        self.weighted
    }

    /// Returns the number of non-zero-probability outcomes.
    #[must_use]
    pub const fn outcome_count(&self) -> ObservationCount {
        self.weighted.count()
    }

    /// Returns the total probability.
    #[must_use]
    pub const fn total_probability(&self) -> f64 {
        self.weighted.total_weight()
    }

    /// Returns the expectation.
    pub fn expectation(&self) -> Result<f64, StatisticsError> {
        self.weighted.mean()
    }

    /// Returns the population variance.
    pub fn variance(&self) -> Result<f64, StatisticsError> {
        self.weighted.variance()
    }

    /// Returns the standard deviation.
    pub fn standard_deviation(
        &self,
    ) -> Result<f64, StatisticsError> {
        self.weighted.standard_deviation()
    }

    /// Returns the second raw moment.
    pub fn second_moment(
        &self,
    ) -> Result<f64, StatisticsError> {
        self.weighted.second_moment()
    }

    /// Returns the smallest numeric outcome.
    pub fn min(&self) -> Result<f64, StatisticsError> {
        self.weighted.min()
    }

    /// Returns the largest numeric outcome.
    pub fn max(&self) -> Result<f64, StatisticsError> {
        self.weighted.max()
    }

    /// Returns Shannon entropy in nats.
    #[must_use]
    pub const fn entropy_nats(&self) -> f64 {
        self.entropy_nats
    }

    /// Returns Shannon entropy in bits.
    pub fn entropy_bits(&self) -> Result<f64, StatisticsError> {
        let bits =
            self.entropy_nats / core::f64::consts::LN_2;

        finite_statistic("entropy_bits", bits)
    }

    /// Returns effective sample size of the distribution weights.
    pub fn effective_sample_size(
        &self,
    ) -> Result<f64, StatisticsError> {
        self.weighted.effective_sample_size()
    }
}

/// Computes Shannon entropy in nats from validated `Probability` values.
///
/// Zero is valid and contributes zero entropy.
pub fn entropy_nats<I>(
    probabilities: I,
) -> Result<f64, StatisticsError>
where
    I: IntoIterator<Item = Probability>,
{
    let mut entropy = 0.0_f64;
    let mut seen = false;

    for probability in probabilities {
        seen = true;

        let p = probability.value();

        if !p.is_finite() || !(0.0..=1.0).contains(&p) {
            return Err(StatisticsError::InvalidProbability {
                value: p,
            });
        }

        if p == 0.0 {
            continue;
        }

        let contribution = -p * p.ln();

        if !contribution.is_finite() {
            return Err(StatisticsError::NonFiniteStatistic {
                name: "entropy_nats",
                value: contribution,
            });
        }

        entropy = entropy
            .checked_add(contribution)
            .ok_or(StatisticsError::NonFiniteStatistic {
                name: "entropy_nats",
                value: f64::INFINITY,
            })?;
    }

    if !seen {
        return Err(StatisticsError::Empty);
    }

    finite_statistic("entropy_nats", entropy)
}

/// Computes Shannon entropy in bits from validated probabilities.
pub fn entropy_bits<I>(
    probabilities: I,
) -> Result<f64, StatisticsError>
where
    I: IntoIterator<Item = Probability>,
{
    let nats = entropy_nats(probabilities)?;

    finite_statistic(
        "entropy_bits",
        nats / core::f64::consts::LN_2,
    )
}

/// Computes a weighted mean from a streaming observation source.
pub fn weighted_mean<I>(
    observations: I,
) -> Result<f64, StatisticsError>
where
    I: IntoIterator<Item = (f64, f64)>,
{
    WeightedStatistics::from_iter(observations)?.mean()
}

/// Computes weighted population variance from a streaming source.
pub fn weighted_variance<I>(
    observations: I,
) -> Result<f64, StatisticsError>
where
    I: IntoIterator<Item = (f64, f64)>,
{
    WeightedStatistics::from_iter(observations)?.variance()
}

/// Computes weighted population standard deviation.
pub fn weighted_standard_deviation<I>(
    observations: I,
) -> Result<f64, StatisticsError>
where
    I: IntoIterator<Item = (f64, f64)>,
{
    WeightedStatistics::from_iter(observations)?
        .standard_deviation()
}

/// Computes the weighted second raw moment.
pub fn weighted_second_moment<I>(
    observations: I,
) -> Result<f64, StatisticsError>
where
    I: IntoIterator<Item = (f64, f64)>,
{
    WeightedStatistics::from_iter(observations)?
        .second_moment()
}

/// Computes effective sample size from a streaming weighted source.
pub fn effective_sample_size<I>(
    observations: I,
) -> Result<f64, StatisticsError>
where
    I: IntoIterator<Item = (f64, f64)>,
{
    WeightedStatistics::from_iter(observations)?
        .effective_sample_size()
}

/// Validates a numerical tolerance supplied by a caller.
pub fn validate_tolerance(
    tolerance: f64,
) -> Result<(), StatisticsError> {
    if !tolerance.is_finite() || tolerance < 0.0 {
        return Err(StatisticsError::InvalidTolerance {
            tolerance,
        });
    }

    Ok(())
}

fn validate_observation(
    value: f64,
) -> Result<(), StatisticsError> {
    if !value.is_finite() {
        return Err(StatisticsError::NonFiniteObservation {
            value,
        });
    }

    Ok(())
}

fn validate_weight(
    weight: f64,
) -> Result<(), StatisticsError> {
    if !weight.is_finite() {
        return Err(StatisticsError::NonFiniteWeight {
            weight,
        });
    }

    if weight < 0.0 {
        return Err(StatisticsError::NegativeWeight {
            weight,
        });
    }

    Ok(())
}

fn finite_statistic(
    name: &'static str,
    value: f64,
) -> Result<f64, StatisticsError> {
    if value.is_finite() {
        Ok(value)
    } else {
        Err(StatisticsError::NonFiniteStatistic {
            name,
            value,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::quantum::zqn::probability::distribution::ProbabilityWeight;

    fn probability(value: f64) -> Probability {
        Probability::new(value)
            .expect("test probability must be valid")
    }

    #[test]
    fn new_accumulator_is_empty() {
        let statistics = WeightedStatistics::new();

        assert_eq!(statistics.count(), 0);
        assert_eq!(statistics.total_weight(), 0.0);
        assert_eq!(
            statistics.mean(),
            Err(StatisticsError::Empty)
        );
    }

    #[test]
    fn zero_weight_is_ignored() {
        let mut statistics = WeightedStatistics::new();

        statistics
            .add(123.0, 0.0)
            .expect("zero weight is valid");

        assert_eq!(statistics.count(), 0);
        assert_eq!(
            statistics.mean(),
            Err(StatisticsError::Empty)
        );
    }

    #[test]
    fn negative_weight_is_rejected() {
        let mut statistics = WeightedStatistics::new();

        assert_eq!(
            statistics.add(1.0, -1.0),
            Err(StatisticsError::NegativeWeight {
                weight: -1.0
            })
        );
    }

    #[test]
    fn non_finite_observation_is_rejected() {
        let mut statistics = WeightedStatistics::new();

        assert!(matches!(
            statistics.add(f64::NAN, 1.0),
            Err(
                StatisticsError::NonFiniteObservation { .. }
            )
        ));

        assert!(matches!(
            statistics.add(f64::INFINITY, 1.0),
            Err(
                StatisticsError::NonFiniteObservation { .. }
            )
        ));
    }

    #[test]
    fn non_finite_weight_is_rejected() {
        let mut statistics = WeightedStatistics::new();

        assert!(matches!(
            statistics.add(1.0, f64::NAN),
            Err(StatisticsError::NonFiniteWeight { .. })
        ));

        assert!(matches!(
            statistics.add(1.0, f64::INFINITY),
            Err(StatisticsError::NonFiniteWeight { .. })
        ));
    }

    #[test]
    fn weighted_mean_is_correct() {
        let statistics = WeightedStatistics::from_iter([
            (0.0, 0.25),
            (1.0, 0.50),
            (2.0, 0.25),
        ])
        .expect("valid weighted observations");

        assert!(
            (statistics.mean().unwrap() - 1.0).abs()
                < 1.0e-15
        );
    }

    #[test]
    fn weighted_variance_is_correct() {
        let statistics = WeightedStatistics::from_iter([
            (0.0, 0.25),
            (1.0, 0.50),
            (2.0, 0.25),
        ])
        .expect("valid weighted observations");

        assert!(
            (statistics.variance().unwrap() - 0.5).abs()
                < 1.0e-15
        );
    }

    #[test]
    fn standard_deviation_is_correct() {
        let statistics = WeightedStatistics::from_iter([
            (0.0, 0.25),
            (1.0, 0.50),
            (2.0, 0.25),
        ])
        .expect("valid weighted observations");

        let expected = 0.5_f64.sqrt();

        assert!(
            (statistics.standard_deviation().unwrap() - expected)
                .abs()
                < 1.0e-15
        );
    }

    #[test]
    fn second_moment_is_correct() {
        let statistics = WeightedStatistics::from_iter([
            (0.0, 0.25),
            (1.0, 0.50),
            (2.0, 0.25),
        ])
        .expect("valid weighted observations");

        assert!(
            (statistics.second_moment().unwrap() - 1.5).abs()
                < 1.0e-15
        );
    }

    #[test]
    fn min_and_max_are_correct() {
        let statistics = WeightedStatistics::from_iter([
            (-10.0, 1.0),
            (3.0, 2.0),
            (7.0, 1.0),
        ])
        .expect("valid observations");

        assert_eq!(statistics.min().unwrap(), -10.0);
        assert_eq!(statistics.max().unwrap(), 7.0);
    }

    #[test]
    fn effective_sample_size_is_one_for_single_observation() {
        let statistics =
            WeightedStatistics::from_iter([(42.0, 7.0)])
                .expect("valid observation");

        assert!(
            (statistics.effective_sample_size().unwrap() - 1.0)
                .abs()
                < 1.0e-15
        );
    }

    #[test]
    fn effective_sample_size_equals_count_for_equal_weights() {
        let statistics = WeightedStatistics::from_iter([
            (0.0, 1.0),
            (1.0, 1.0),
            (2.0, 1.0),
            (3.0, 1.0),
        ])
        .expect("valid observations");

        assert!(
            (statistics.effective_sample_size().unwrap() - 4.0)
                .abs()
                < 1.0e-15
        );
    }

    #[test]
    fn merge_matches_single_pass_statistics() {
        let first = WeightedStatistics::from_iter([
            (0.0, 1.0),
            (1.0, 2.0),
        ])
        .expect("valid first partition");

        let second = WeightedStatistics::from_iter([
            (2.0, 1.0),
            (3.0, 2.0),
        ])
        .expect("valid second partition");

        let combined =
            WeightedStatistics::from_iter([
                (0.0, 1.0),
                (1.0, 2.0),
                (2.0, 1.0),
                (3.0, 2.0),
            ])
            .expect("valid combined stream");

        let mut merged = first;
        merged
            .merge(&second)
            .expect("compatible accumulators");

        assert!(
            (merged.mean().unwrap() - combined.mean().unwrap()).abs()
                < 1.0e-14
        );

        assert!(
            (merged.variance().unwrap()
                - combined.variance().unwrap())
                .abs()
                < 1.0e-13
        );

        assert_eq!(merged.count(), combined.count());
    }

    #[test]
    fn distribution_statistics_integrate_with_distribution() {
        let distribution =
            Distribution::from_parts(
                vec![0_u8, 1_u8, 2_u8],
                vec![
                    ProbabilityWeight::new(0.25).unwrap(),
                    ProbabilityWeight::new(0.50).unwrap(),
                    ProbabilityWeight::new(0.25).unwrap(),
                ],
                1.0e-12,
            )
            .expect("valid distribution");

        let statistics =
            DistributionStatistics::from_distribution(
                &distribution,
                1.0e-12,
            )
            .expect("valid distribution statistics");

        assert!(
            (statistics.expectation().unwrap() - 1.0).abs()
                < 1.0e-15
        );

        assert!(
            (statistics.variance().unwrap() - 0.5).abs()
                < 1.0e-15
        );

        assert!(
            (statistics.entropy_bits().unwrap() - 1.5).abs()
                < 1.0e-12
        );
    }

    #[test]
    fn certain_event_has_zero_entropy() {
        let entropy =
            entropy_bits([probability(1.0)])
                .expect("valid entropy");

        assert_eq!(entropy, 0.0);
    }

    #[test]
    fn uniform_two_outcomes_have_one_bit_entropy() {
        let entropy = entropy_bits([
            probability(0.5),
            probability(0.5),
        ])
        .expect("valid entropy");

        assert!(
            (entropy - 1.0).abs() < 1.0e-15
        );
    }

    #[test]
    fn entropy_accepts_zero_probability() {
        let entropy = entropy_bits([
            probability(0.0),
            probability(1.0),
        ])
        .expect("valid entropy");

        assert_eq!(entropy, 0.0);
    }

    #[test]
    fn empty_entropy_stream_is_rejected() {
        let result =
            entropy_nats(std::iter::empty::<Probability>());

        assert_eq!(
            result,
            Err(StatisticsError::Empty)
        );
    }

    #[test]
    fn invalid_tolerance_is_rejected() {
        assert!(matches!(
            validate_tolerance(f64::NAN),
            Err(StatisticsError::InvalidTolerance { .. })
        ));

        assert!(matches!(
            validate_tolerance(f64::INFINITY),
            Err(StatisticsError::InvalidTolerance { .. })
        ));

        assert!(matches!(
            validate_tolerance(-1.0),
            Err(StatisticsError::InvalidTolerance { .. })
        ));

        assert!(
            validate_tolerance(0.0).is_ok()
        );
    }

    #[test]
    fn large_offset_variance_remains_stable() {
        let base = 1.0e12;

        let statistics = WeightedStatistics::from_iter([
            (base - 1.0, 1.0),
            (base, 1.0),
            (base + 1.0, 1.0),
        ])
        .expect("valid observations");

        let variance =
            statistics.variance().expect("finite variance");

        assert!(
            (variance - (2.0 / 3.0)).abs() < 1.0e-6,
            "variance={variance}"
        );
    }

    #[test]
    fn streaming_generated_input_does_not_materialize_samples() {
        let statistics =
            WeightedStatistics::from_iter(
                (0_u64..10_000)
                    .map(|value| (value as f64, 1.0)),
            )
            .expect("generated stream");

        assert_eq!(
            statistics.count(),
            10_000
        );

        assert!(
            statistics.mean().unwrap().is_finite()
        );

        assert!(
            statistics.variance().unwrap().is_finite()
        );
    }

    #[test]
    fn singleton_distribution_has_zero_variance() {
        let distribution =
            Distribution::singleton(7_u8);

        let statistics =
            DistributionStatistics::from_distribution(
                &distribution,
                0.0,
            )
            .expect("valid singleton");

        assert_eq!(
            statistics.expectation().unwrap(),
            7.0
        );

        assert_eq!(
            statistics.variance().unwrap(),
            0.0
        );

        assert_eq!(
            statistics.entropy_nats(),
            0.0
        );
    }

    #[test]
    fn statistics_are_deterministic_for_same_ordered_input() {
        let observations = [
            (-3.0, 0.5),
            (1.0, 1.5),
            (9.0, 2.0),
            (4.0, 0.25),
        ];

        let first =
            WeightedStatistics::from_iter(
                observations,
            )
            .expect("valid observations");

        let second =
            WeightedStatistics::from_iter(
                observations,
            )
            .expect("valid observations");

        assert_eq!(
            first.mean().unwrap().to_bits(),
            second.mean().unwrap().to_bits()
        );

        assert_eq!(
            first.variance().unwrap().to_bits(),
            second.variance().unwrap().to_bits()
        );

        assert_eq!(
            first.effective_sample_size().unwrap().to_bits(),
            second.effective_sample_size().unwrap().to_bits()
        );
    }

    #[test]
    fn negative_variance_is_not_constructed_by_normal_operation() {
        let statistics =
            WeightedStatistics::from_iter([
                (1.0, 1.0),
                (1.0, 1.0),
                (1.0, 1.0),
            ])
            .expect("valid observations");

        assert_eq!(
            statistics.variance().unwrap(),
            0.0
        );
    }

    #[test]
    fn helper_functions_use_streaming_accumulator() {
        let observations = [
            (0.0, 1.0),
            (2.0, 1.0),
        ];

        assert_eq!(
            weighted_mean(observations).unwrap(),
            1.0
        );

        assert!(
            (weighted_variance(observations).unwrap() - 1.0)
                .abs()
                < 1.0e-15
        );

        assert!(
            (weighted_standard_deviation(observations).unwrap()
                - 1.0)
                .abs()
                < 1.0e-15
        );

        assert!(
            (weighted_second_moment(observations).unwrap() - 2.0)
                .abs()
                < 1.0e-15
        );

        assert!(
            (effective_sample_size(observations).unwrap() - 2.0)
                .abs()
                < 1.0e-15
        );
    }
}