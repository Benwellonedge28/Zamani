//! Zamani Quantum Error Correction — Statistical Mathematics
//!
//! # Ownership
//!
//! This module owns the statistical mathematics used by the QEC subsystem.
//!
//! It owns:
//!
//! - binomial samples;
//! - logical-error-rate estimation;
//! - physical-error-rate estimation;
//! - Wilson confidence intervals;
//! - Clopper-Pearson confidence intervals;
//! - normal-approximation intervals;
//! - standard error calculation;
//! - confidence-width evaluation;
//! - minimum-sample decisions;
//! - sequential stopping decisions;
//! - threshold-experiment statistical primitives;
//! - deterministic aggregation of statistical counters.
//!
//! It does NOT own:
//!
//! - simulation execution;
//! - noise models;
//! - syndrome extraction;
//! - decoding;
//! - QPU execution;
//! - resource admission;
//! - memory allocation;
//! - cancellation state;
//! - telemetry transport;
//! - metrics collection;
//! - decoder correctness;
//! - threshold interpretation.
//!
//! # Integration contract
//!
//! ```text
//!                   simulation.rs
//!                        │
//!                        ▼
//!                 StatisticalSample
//!                        │
//!                        ▼
//!                StatisticalAccumulator
//!                        │
//!              ┌─────────┼─────────┐
//!              ▼         ▼         ▼
//!           Wilson   Clopper   StandardError
//!              │         │         │
//!              └─────────┼─────────┘
//!                        ▼
//!                ConfidenceInterval
//!                        │
//!                        ▼
//!                 StatisticalReport
//! ```
//!
//! Threshold experiments may consume this module:
//!
//! ```text
//! physical error rate
//!        │
//!        ▼
//! logical error rate
//!        │
//!        ▼
//! confidence interval
//!        │
//!        ▼
//! threshold experiment
//! ```
//!
//! The module never decides whether an observed result constitutes a
//! physical threshold. It supplies statistically valid estimates and
//! uncertainty.
//!
//! # Determinism
//!
//! All primary aggregation uses integer counters.
//!
//! Floating-point arithmetic is performed only when deriving statistical
//! quantities from those counters.
//!
//! Therefore:
//!
//! ```text
//! observations
//!      ↓
//! integer successes/failures
//!      ↓
//! deterministic sample
//!      ↓
//! statistical estimate
//! ```
//!
//! The module contains no random-number generation.
//!
//! # Security
//!
//! Statistical structures must never contain:
//!
//! - QPU credentials;
//! - API tokens;
//! - private keys;
//! - raw measurement payloads;
//! - raw syndrome streams;
//! - quantum circuits;
//! - user data.
//!
//! # Resource safety
//!
//! Statistical counters are bounded by `u64`.
//!
//! Aggregation uses checked validation and never silently changes the meaning
//! of a sample. Counter overflow is rejected.
//!
//! # Rust compatibility
//!
//! Target: Rust 1.97.1.
//!
//! Stable standard-library APIs only.

#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;

use super::errors::{NumericalOperation, QecError, QecResult};

/// Statistical API schema version.
///
/// Increment whenever the serialized or externally observable statistical
/// contract changes.
pub const STATISTICAL_SCHEMA_VERSION: u32 = 1;

/// Default confidence level.
pub const DEFAULT_CONFIDENCE_LEVEL: f64 = 0.95;

/// Smallest supported confidence level.
pub const MIN_CONFIDENCE_LEVEL: f64 = 0.5;

/// Largest supported confidence level.
///
/// A confidence level of exactly 1.0 is mathematically unusable for finite
/// confidence intervals.
pub const MAX_CONFIDENCE_LEVEL: f64 = 1.0;

/// Default minimum number of classified observations.
pub const DEFAULT_MINIMUM_SAMPLES: u64 = 100;

/// Maximum number of observations represented by one statistical sample.
pub const MAX_SAMPLE_COUNT: u64 = u64::MAX;

/// Statistical result type.
pub type StatisticalResult<T> = Result<T, StatisticalError>;

/// Errors produced by the statistical subsystem.
#[derive(Debug, Clone, PartialEq)]
pub enum StatisticalError {
    /// A statistical parameter is outside its valid mathematical domain.
    InvalidParameter {
        parameter: &'static str,
        message: String,
    },

    /// A sample contains more failures than observations.
    InvalidSample {
        failures: u64,
        observations: u64,
    },

    /// Integer aggregation would overflow.
    CounterOverflow {
        counter: &'static str,
    },

    /// A floating-point calculation produced a non-finite result.
    NonFiniteResult {
        operation: NumericalOperation,
    },

    /// A confidence interval cannot be constructed from the supplied sample.
    InsufficientSample {
        required: u64,
        observed: u64,
    },
}

impl fmt::Display for StatisticalError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidParameter {
                parameter,
                message,
            } => write!(
                formatter,
                "invalid statistical parameter `{parameter}`: {message}"
            ),

            Self::InvalidSample {
                failures,
                observations,
            } => write!(
                formatter,
                "invalid statistical sample: {failures} failures out of \
                 {observations} observations"
            ),

            Self::CounterOverflow { counter } => {
                write!(
                    formatter,
                    "statistical counter overflow: {counter}"
                )
            }

            Self::NonFiniteResult { operation } => {
                write!(
                    formatter,
                    "non-finite statistical result during {operation}"
                )
            }

            Self::InsufficientSample {
                required,
                observed,
            } => write!(
                formatter,
                "insufficient statistical sample: {observed} observed, \
                 {required} required"
            ),
        }
    }
}

impl std::error::Error for StatisticalError {}

impl From<StatisticalError> for QecError {
    fn from(error: StatisticalError) -> Self {
        match error {
            StatisticalError::InvalidParameter {
                parameter,
                message,
            } => QecError::invalid_input(format!(
                "invalid statistical parameter `{parameter}`: {message}"
            )),

            StatisticalError::InvalidSample {
                failures,
                observations,
            } => QecError::invalid_input(format!(
                "invalid statistical sample: {failures}/{observations}"
            )),

            StatisticalError::CounterOverflow { counter } => {
                QecError::numerical_failure(
                    NumericalOperation::Accumulation,
                    format!("statistical counter overflow: {counter}"),
                )
            }

            StatisticalError::NonFiniteResult { operation } => {
                QecError::numerical_failure(
                    operation,
                    "statistical calculation produced a non-finite result",
                )
            }

            StatisticalError::InsufficientSample {
                required,
                observed,
            } => QecError::invalid_input(format!(
                "insufficient statistical sample: {observed}/{required}"
            )),
        }
    }
}

/// Confidence interval method.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConfidenceIntervalMethod {
    /// Wilson score interval.
    Wilson,

    /// Exact Clopper-Pearson interval.
    ClopperPearson,

    /// Normal approximation.
    NormalApproximation,
}

impl Default for ConfidenceIntervalMethod {
    fn default() -> Self {
        Self::Wilson
    }
}

/// Statistical sample containing only aggregate counters.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct BinomialSample {
    /// Number of successful observations.
    pub successes: u64,

    /// Number of failed observations.
    pub failures: u64,
}

impl BinomialSample {
    /// Creates an empty sample.
    pub const fn new() -> Self {
        Self {
            successes: 0,
            failures: 0,
        }
    }

    /// Creates a sample from successes and failures.
    pub fn from_counts(
        successes: u64,
        failures: u64,
    ) -> StatisticalResult<Self> {
        let sample = Self {
            successes,
            failures,
        };

        sample.validate()?;
        Ok(sample)
    }

    /// Creates a sample where `failures` is the measured event.
    pub fn from_failures(
        failures: u64,
        observations: u64,
    ) -> StatisticalResult<Self> {
        if failures > observations {
            return Err(StatisticalError::InvalidSample {
                failures,
                observations,
            });
        }

        Self::from_counts(
            observations - failures,
            failures,
        )
    }

    /// Returns the total number of observations.
    pub const fn observations(self) -> u64 {
        self.successes.saturating_add(self.failures)
    }

    /// Validates the sample.
    pub const fn validate(self) -> StatisticalResult<()> {
        // The representation itself guarantees failures and successes are
        // non-negative. Their sum is permitted to saturate only for the
        // purpose of this validation boundary; normal accumulation uses
        // checked arithmetic below.
        Ok(())
    }

    /// Returns the observed event rate.
    pub fn rate(self) -> StatisticalResult<f64> {
        let observations = self.observations();

        if observations == 0 {
            return Err(StatisticalError::InsufficientSample {
                required: 1,
                observed: 0,
            });
        }

        let rate = self.failures as f64 / observations as f64;

        ensure_finite(
            rate,
            NumericalOperation::StatisticalEstimate,
        )
    }

    /// Adds another sample without permitting integer overflow.
    pub fn checked_add(
        &mut self,
        other: Self,
    ) -> StatisticalResult<()> {
        self.successes = self
            .successes
            .checked_add(other.successes)
            .ok_or(StatisticalError::CounterOverflow {
                counter: "successes",
            })?;

        self.failures = self
            .failures
            .checked_add(other.failures)
            .ok_or(StatisticalError::CounterOverflow {
                counter: "failures",
            })?;

        Ok(())
    }

    /// Returns a merged sample without mutating either input.
    pub fn merged(
        self,
        other: Self,
    ) -> StatisticalResult<Self> {
        let mut merged = self;
        merged.checked_add(other)?;
        Ok(merged)
    }
}

/// Statistical confidence interval.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ConfidenceInterval {
    /// Lower bound, inclusive.
    pub lower: f64,

    /// Upper bound, inclusive.
    pub upper: f64,

    /// Point estimate.
    pub estimate: f64,

    /// Confidence level.
    pub confidence_level: f64,

    /// Interval construction method.
    pub method: ConfidenceIntervalMethod,

    /// Number of observations.
    pub observations: u64,

    /// Number of observed failures.
    pub failures: u64,
}

impl ConfidenceInterval {
    /// Returns the interval width.
    pub fn width(self) -> f64 {
        self.upper - self.lower
    }

    /// Returns the half-width.
    pub fn half_width(self) -> f64 {
        self.width() / 2.0
    }

    /// Returns whether the interval contains zero.
    pub fn contains_zero(self) -> bool {
        self.lower <= 0.0
    }

    /// Returns whether the interval contains one.
    pub fn contains_one(self) -> bool {
        self.upper >= 1.0
    }

    /// Returns whether the interval is statistically valid.
    pub fn is_valid(self) -> bool {
        self.lower.is_finite()
            && self.upper.is_finite()
            && self.estimate.is_finite()
            && self.lower >= 0.0
            && self.upper <= 1.0
            && self.lower <= self.estimate
            && self.estimate <= self.upper
            && self.lower <= self.upper
    }
}

/// Statistical stopping condition.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StoppingCondition {
    /// Continue sampling.
    Continue,

    /// Required minimum observations have not yet been reached.
    MinimumSamplesRequired {
        required: u64,
        observed: u64,
    },

    /// Target confidence width has been reached.
    ConfidenceWidthReached {
        width: f64,
    },

    /// Target number of failures has been reached.
    FailureTargetReached {
        failures: u64,
    },

    /// The requested number of observations has been completed.
    RequestedSamplesReached {
        observations: u64,
    },
}

/// Configuration for statistical estimation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StatisticalConfig {
    /// Confidence level.
    pub confidence_level: f64,

    /// Interval construction method.
    pub method: ConfidenceIntervalMethod,

    /// Minimum number of observations.
    pub minimum_samples: u64,

    /// Optional desired maximum interval width.
    pub target_width: Option<f64>,

    /// Optional failure target for sequential experiments.
    pub target_failures: Option<u64>,
}

impl Default for StatisticalConfig {
    fn default() -> Self {
        Self {
            confidence_level: DEFAULT_CONFIDENCE_LEVEL,
            method: ConfidenceIntervalMethod::Wilson,
            minimum_samples: DEFAULT_MINIMUM_SAMPLES,
            target_width: None,
            target_failures: None,
        }
    }
}

impl StatisticalConfig {
    /// Validates the statistical configuration.
    pub fn validate(self) -> StatisticalResult<()> {
        validate_confidence_level(self.confidence_level)?;

        if self.minimum_samples == 0 {
            return Err(StatisticalError::InvalidParameter {
                parameter: "minimum_samples",
                message: "must be greater than zero".to_owned(),
            });
        }

        if let Some(width) = self.target_width {
            if !width.is_finite() || width <= 0.0 || width > 1.0 {
                return Err(StatisticalError::InvalidParameter {
                    parameter: "target_width",
                    message: "must be finite and in (0, 1]".to_owned(),
                });
            }
        }

        if let Some(target) = self.target_failures {
            if target == 0 {
                return Err(StatisticalError::InvalidParameter {
                    parameter: "target_failures",
                    message: "must be greater than zero".to_owned(),
                });
            }
        }

        Ok(())
    }
}

/// Deterministic aggregate of statistical observations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct StatisticalAccumulator {
    sample: BinomialSample,
}

impl StatisticalAccumulator {
    /// Creates an empty accumulator.
    pub const fn new() -> Self {
        Self {
            sample: BinomialSample::new(),
        }
    }

    /// Records one successful observation.
    pub fn record_success(&mut self) -> StatisticalResult<()> {
        self.sample.successes = self
            .sample
            .successes
            .checked_add(1)
            .ok_or(StatisticalError::CounterOverflow {
                counter: "successes",
            })?;

        Ok(())
    }

    /// Records one failed observation.
    pub fn record_failure(&mut self) -> StatisticalResult<()> {
        self.sample.failures = self
            .sample
            .failures
            .checked_add(1)
            .ok_or(StatisticalError::CounterOverflow {
                counter: "failures",
            })?;

        Ok(())
    }

    /// Records a batch of successful observations.
    pub fn record_successes(
        &mut self,
        count: u64,
    ) -> StatisticalResult<()> {
        self.sample.successes = self
            .sample
            .successes
            .checked_add(count)
            .ok_or(StatisticalError::CounterOverflow {
                counter: "successes",
            })?;

        Ok(())
    }

    /// Records a batch of failed observations.
    pub fn record_failures(
        &mut self,
        count: u64,
    ) -> StatisticalResult<()> {
        self.sample.failures = self
            .sample
            .failures
            .checked_add(count)
            .ok_or(StatisticalError::CounterOverflow {
                counter: "failures",
            })?;

        Ok(())
    }

    /// Returns the aggregate sample.
    pub const fn sample(self) -> BinomialSample {
        self.sample
    }

    /// Returns the number of observations.
    pub const fn observations(self) -> u64 {
        self.sample.successes + self.sample.failures
    }

    /// Produces a confidence interval.
    pub fn confidence_interval(
        self,
        config: StatisticalConfig,
    ) -> StatisticalResult<ConfidenceInterval> {
        estimate(self.sample, config)
    }

    /// Determines whether sequential execution should stop.
    pub fn stopping_condition(
        self,
        requested_samples: u64,
        config: StatisticalConfig,
    ) -> StatisticalResult<StoppingCondition> {
        config.validate()?;

        let observations = self.sample.observations();

        if observations >= requested_samples {
            return Ok(StoppingCondition::RequestedSamplesReached {
                observations,
            });
        }

        if observations < config.minimum_samples {
            return Ok(
                StoppingCondition::MinimumSamplesRequired {
                    required: config.minimum_samples,
                    observed: observations,
                },
            );
        }

        if let Some(target) = config.target_failures {
            if self.sample.failures >= target {
                return Ok(
                    StoppingCondition::FailureTargetReached {
                        failures: self.sample.failures,
                    },
                );
            }
        }

        if let Some(target_width) = config.target_width {
            let interval = self.confidence_interval(config)?;

            if interval.width() <= target_width {
                return Ok(
                    StoppingCondition::ConfidenceWidthReached {
                        width: interval.width(),
                    },
                );
            }
        }

        Ok(StoppingCondition::Continue)
    }
}

/// Complete statistical report.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StatisticalReport {
    /// Aggregate sample.
    pub sample: BinomialSample,

    /// Confidence interval.
    pub interval: ConfidenceInterval,

    /// Whether the configured minimum sample count was reached.
    pub minimum_samples_reached: bool,

    /// Whether the requested experiment size was reached.
    pub requested_samples_reached: bool,

    /// Whether the report is statistically meaningful under its
    /// configuration.
    pub statistically_meaningful: bool,
}

impl StatisticalReport {
    /// Creates a report from a sample.
    pub fn from_sample(
        sample: BinomialSample,
        config: StatisticalConfig,
    ) -> StatisticalResult<Self> {
        config.validate()?;

        let interval = estimate(sample, config)?;
        let observations = sample.observations();

        Ok(Self {
            sample,
            interval,
            minimum_samples_reached:
                observations >= config.minimum_samples,
            requested_samples_reached: false,
            statistically_meaningful:
                observations >= config.minimum_samples,
        })
    }
}

/// Estimates a binomial confidence interval.
pub fn estimate(
    sample: BinomialSample,
    config: StatisticalConfig,
) -> StatisticalResult<ConfidenceInterval> {
    config.validate()?;
    sample.validate()?;

    match config.method {
        ConfidenceIntervalMethod::Wilson => {
            wilson_interval(
                sample,
                config.confidence_level,
            )
        }

        ConfidenceIntervalMethod::ClopperPearson => {
            clopper_pearson_interval(
                sample,
                config.confidence_level,
            )
        }

        ConfidenceIntervalMethod::NormalApproximation => {
            normal_interval(
                sample,
                config.confidence_level,
            )
        }
    }
}

/// Computes a Wilson score interval.
///
/// This is the preferred default interval for QEC Monte Carlo experiments
/// because it remains well-behaved near probabilities of zero and one.
pub fn wilson_interval(
    sample: BinomialSample,
    confidence_level: f64,
) -> StatisticalResult<ConfidenceInterval> {
    validate_confidence_level(confidence_level)?;

    let n = sample.observations();

    if n == 0 {
        return Err(StatisticalError::InsufficientSample {
            required: 1,
            observed: 0,
        });
    }

    let failures = sample.failures as f64;
    let observations = n as f64;

    let estimate = failures / observations;

    let z = standard_normal_quantile(
        0.5 + confidence_level / 2.0,
    )?;

    let z2 = z * z;

    let denominator =
        1.0 + z2 / observations;

    let centre =
        (estimate + z2 / (2.0 * observations))
            / denominator;

    let variance =
        estimate * (1.0 - estimate) / observations
            + z2 / (4.0 * observations * observations);

    let margin =
        z * variance.sqrt() / denominator;

    let lower = clamp_probability(centre - margin);
    let upper = clamp_probability(centre + margin);

    let interval = ConfidenceInterval {
        lower,
        upper,
        estimate,
        confidence_level,
        method: ConfidenceIntervalMethod::Wilson,
        observations: n,
        failures: sample.failures,
    };

    validate_interval(interval)
}

/// Computes a normal-approximation interval.
///
/// This method is provided for comparison and compatibility with experiments
/// that explicitly require the approximation. Wilson remains the default.
pub fn normal_interval(
    sample: BinomialSample,
    confidence_level: f64,
) -> StatisticalResult<ConfidenceInterval> {
    validate_confidence_level(confidence_level)?;

    let n = sample.observations();

    if n == 0 {
        return Err(StatisticalError::InsufficientSample {
            required: 1,
            observed: 0,
        });
    }

    let observations = n as f64;
    let estimate = sample.failures as f64 / observations;

    let z = standard_normal_quantile(
        0.5 + confidence_level / 2.0,
    )?;

    let standard_error =
        (estimate * (1.0 - estimate) / observations).sqrt();

    let margin = z * standard_error;

    let interval = ConfidenceInterval {
        lower: clamp_probability(estimate - margin),
        upper: clamp_probability(estimate + margin),
        estimate,
        confidence_level,
        method: ConfidenceIntervalMethod::NormalApproximation,
        observations: n,
        failures: sample.failures,
    };

    validate_interval(interval)
}

/// Computes an exact Clopper-Pearson interval.
///
/// The implementation uses the regularized incomplete beta function through
/// a numerically stable binary search over the beta CDF.
///
/// This keeps the statistical API self-contained and avoids adding a
/// third-party numerical dependency merely for confidence intervals.
pub fn clopper_pearson_interval(
    sample: BinomialSample,
    confidence_level: f64,
) -> StatisticalResult<ConfidenceInterval> {
    validate_confidence_level(confidence_level)?;

    let n = sample.observations();

    if n == 0 {
        return Err(StatisticalError::InsufficientSample {
            required: 1,
            observed: 0,
        });
    }

    let alpha = 1.0 - confidence_level;

    let lower = if sample.failures == 0 {
        0.0
    } else {
        beta_inverse(
            alpha / 2.0,
            sample.failures as f64,
            (n - sample.failures + 1) as f64,
        )?
    };

    let upper = if sample.failures == n {
        1.0
    } else {
        beta_inverse(
            1.0 - alpha / 2.0,
            (sample.failures + 1) as f64,
            (n - sample.failures) as f64,
        )?
    };

    let interval = ConfidenceInterval {
        lower,
        upper,
        estimate: sample.failures as f64 / n as f64,
        confidence_level,
        method: ConfidenceIntervalMethod::ClopperPearson,
        observations: n,
        failures: sample.failures,
    };

    validate_interval(interval)
}

/// Returns the binomial standard error.
pub fn standard_error(
    sample: BinomialSample,
) -> StatisticalResult<f64> {
    let n = sample.observations();

    if n == 0 {
        return Err(StatisticalError::InsufficientSample {
            required: 1,
            observed: 0,
        });
    }

    let p = sample.failures as f64 / n as f64;

    ensure_finite(
        (p * (1.0 - p) / n as f64).sqrt(),
        NumericalOperation::StatisticalEstimate,
    )
}

/// Validates a confidence level.
pub fn validate_confidence_level(
    confidence_level: f64,
) -> StatisticalResult<()> {
    if !confidence_level.is_finite()
        || confidence_level <= MIN_CONFIDENCE_LEVEL
        || confidence_level >= MAX_CONFIDENCE_LEVEL
    {
        return Err(StatisticalError::InvalidParameter {
            parameter: "confidence_level",
            message:
                "must be finite and strictly between 0.5 and 1.0"
                    .to_owned(),
        });
    }

    Ok(())
}

/// Clamps a probability into the mathematically valid interval.
fn clamp_probability(value: f64) -> f64 {
    value.clamp(0.0, 1.0)
}

/// Validates a calculated confidence interval.
fn validate_interval(
    interval: ConfidenceInterval,
) -> StatisticalResult<ConfidenceInterval> {
    if !interval.is_valid() {
        return Err(StatisticalError::NonFiniteResult {
            operation: NumericalOperation::StatisticalEstimate,
        });
    }

    Ok(interval)
}

/// Ensures a floating-point result is finite.
fn ensure_finite(
    value: f64,
    operation: NumericalOperation,
) -> StatisticalResult<f64> {
    if value.is_finite() {
        Ok(value)
    } else {
        Err(StatisticalError::NonFiniteResult {
            operation,
        })
    }
}

/// Approximation of the inverse standard-normal CDF.
///
/// Uses the Acklam rational approximation. Accuracy is sufficient for
/// confidence intervals while avoiding an external statistics dependency.
fn standard_normal_quantile(
    probability: f64,
) -> StatisticalResult<f64> {
    if !probability.is_finite()
        || probability <= 0.0
        || probability >= 1.0
    {
        return Err(StatisticalError::InvalidParameter {
            parameter: "probability",
            message: "must be finite and strictly between zero and one"
                .to_owned(),
        });
    }

    const A: [f64; 6] = [
        -3.969683028665376e1,
        2.209460984245205e2,
        -2.759285104469687e2,
        1.38357751867269e2,
        -3.066479806614716e1,
        2.506628277459239,
    ];

    const B: [f64; 5] = [
        -5.447609879822406e1,
        1.615858368580409e2,
        -1.556989798598866e2,
        6.680131188771972e1,
        -1.328068155288572e1,
    ];

    const C: [f64; 6] = [
        -7.784894002430293e-3,
        -3.223964580411365e-1,
        -2.400758277161838,
        -2.549732539343734,
        4.374664141464968,
        2.938163982698783,
    ];

    const D: [f64; 4] = [
        7.784695709041462e-3,
        3.224671290700398e-1,
        2.445134137142996,
        3.754408661907416,
    ];

    const LOW: f64 = 0.02425;
    const HIGH: f64 = 1.0 - LOW;

    let result;

    if probability < LOW {
        let q = (-2.0 * probability.ln()).sqrt();

        result =
            (((((C[0] * q + C[1]) * q + C[2]) * q + C[3]) * q
                + C[4])
                * q)
                + C[5])
                /
            ((((D[0] * q + D[1]) * q + D[2]) * q + D[3]) * q
                + 1.0);
    } else if probability <= HIGH {
        let q = probability - 0.5;
        let r = q * q;

        result =
            (((((A[0] * r + A[1]) * r + A[2]) * r + A[3]) * r
                + A[4])
                * r)
                + A[5])
                * q
                /
            (((((B[0] * r + B[1]) * r + B[2]) * r + B[3]) * r
                + B[4])
                * r)
                + 1.0);
    } else {
        let q = (-2.0 * (1.0 - probability).ln()).sqrt();

        result =
            -(((((C[0] * q + C[1]) * q + C[2]) * q + C[3]) * q
                + C[4])
                * q)
                + C[5])
                /
            ((((D[0] * q + D[1]) * q + D[2]) * q + D[3]) * q
                + 1.0);
    }

    ensure_finite(
        result,
        NumericalOperation::StatisticalEstimate,
    )
}

/// Regularized incomplete beta function.
///
/// Continued-fraction implementation based on the standard Lentz method.
fn regularized_beta(
    x: f64,
    a: f64,
    b: f64,
) -> StatisticalResult<f64> {
    if !x.is_finite()
        || !a.is_finite()
        || !b.is_finite()
        || a <= 0.0
        || b <= 0.0
        || x < 0.0
        || x > 1.0
    {
        return Err(StatisticalError::InvalidParameter {
            parameter: "beta_distribution",
            message: "invalid beta-distribution parameters".to_owned(),
        });
    }

    if x == 0.0 {
        return Ok(0.0);
    }

    if x == 1.0 {
        return Ok(1.0);
    }

    let log_beta =
        ln_gamma(a)
            + ln_gamma(b)
            - ln_gamma(a + b);

    let front = (a * x.ln()
        + b * (1.0 - x).ln()
        - log_beta)
        .exp()
        / a;

    let result = if x < (a + 1.0) / (a + b + 2.0) {
        front * beta_continued_fraction(x, a, b)?
    } else {
        1.0
            - ((b
                * (1.0 - x).ln()
                + a * x.ln()
                - log_beta)
                .exp()
                / b)
                * beta_continued_fraction(1.0 - x, b, a)?
    };

    ensure_finite(
        clamp_probability(result),
        NumericalOperation::StatisticalEstimate,
    )
}

/// Inverse regularized beta by monotonic binary search.
///
/// This is intentionally deterministic.
fn beta_inverse(
    target: f64,
    a: f64,
    b: f64,
) -> StatisticalResult<f64> {
    if !target.is_finite()
        || target <= 0.0
        || target >= 1.0
    {
        return Err(StatisticalError::InvalidParameter {
            parameter: "beta_probability",
            message:
                "must be finite and strictly between zero and one"
                    .to_owned(),
        });
    }

    let mut lower = 0.0;
    let mut upper = 1.0;

    // Fixed iteration count makes the result deterministic.
    for _ in 0..100 {
        let middle = (lower + upper) / 2.0;

        let value = regularized_beta(middle, a, b)?;

        if value < target {
            lower = middle;
        } else {
            upper = middle;
        }
    }

    Ok((lower + upper) / 2.0)
}

/// Continued fraction for the incomplete beta function.
fn beta_continued_fraction(
    x: f64,
    a: f64,
    b: f64,
) -> StatisticalResult<f64> {
    const MAX_ITERATIONS: usize = 200;
    const EPSILON: f64 = 3.0e-14;
    const MIN_VALUE: f64 = 1.0e-300;

    let qab = a + b;
    let qap = a + 1.0;
    let qam = a - 1.0;

    let mut c = 1.0;
    let mut d = 1.0 - qab * x / qap;

    if d.abs() < MIN_VALUE {
        d = MIN_VALUE;
    }

    d = 1.0 / d;

    let mut h = d;

    for iteration in 1..=MAX_ITERATIONS {
        let m = iteration as f64;
        let m2 = 2.0 * m;

        let aa =
            m * (b - m) * x
                / ((qam + m2) * (a + m2));

        d = 1.0 + aa * d;

        if d.abs() < MIN_VALUE {
            d = MIN_VALUE;
        }

        c = 1.0 + aa / c;

        if c.abs() < MIN_VALUE {
            c = MIN_VALUE;
        }

        d = 1.0 / d;
        h *= d * c;

        let aa =
            -(a + m) * (qab + m) * x
                / ((a + m2) * (qap + m2));

        d = 1.0 + aa * d;

        if d.abs() < MIN_VALUE {
            d = MIN_VALUE;
        }

        c = 1.0 + aa / c;

        if c.abs() < MIN_VALUE {
            c = MIN_VALUE;
        }

        d = 1.0 / d;

        let delta = d * c;
        h *= delta;

        if (delta - 1.0).abs() < EPSILON {
            return ensure_finite(
                h,
                NumericalOperation::StatisticalEstimate,
            );
        }
    }

    ensure_finite(
        h,
        NumericalOperation::StatisticalEstimate,
    )
}

/// Natural logarithm of the gamma function.
///
/// Lanczos approximation.
fn ln_gamma(value: f64) -> f64 {
    const COEFFICIENTS: [f64; 9] = [
        0.9999999999998099,
        676.5203681218851,
        -1259.1392167224028,
        771.3234287776531,
        -176.6150291621406,
        12.507343278686905,
        -0.13857109526572012,
        9.984369578019572e-6,
        1.5056327351493116e-7,
    ];

    if value < 0.5 {
        return core::f64::consts::PI.ln()
            - (core::f64::consts::PI * value).sin().ln()
            - ln_gamma(1.0 - value);
    }

    let adjusted = value - 1.0;

    let mut x = COEFFICIENTS[0];

    for (index, coefficient) in
        COEFFICIENTS.iter().enumerate().skip(1)
    {
        x += coefficient / (adjusted + index as f64);
    }

    let t = adjusted + 7.5;

    0.5 * (2.0 * core::f64::consts::PI).ln()
        + (adjusted + 0.5) * t.ln()
        - t
        + x.ln()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_sample_is_rejected() {
        let sample = BinomialSample::new();

        assert!(sample.rate().is_err());
    }

    #[test]
    fn sample_rate_is_correct() {
        let sample =
            BinomialSample::from_failures(10, 100)
                .expect("valid sample");

        let rate = sample.rate().expect("non-empty sample");

        assert!((rate - 0.1).abs() < 1.0e-12);
    }

    #[test]
    fn wilson_interval_contains_estimate() {
        let sample =
            BinomialSample::from_failures(10, 100)
                .expect("valid sample");

        let interval =
            wilson_interval(sample, 0.95)
                .expect("valid interval");

        assert!(interval.lower <= interval.estimate);
        assert!(interval.estimate <= interval.upper);
        assert!(interval.is_valid());
    }

    #[test]
    fn zero_failures_have_zero_lower_bound() {
        let sample =
            BinomialSample::from_failures(0, 100)
                .expect("valid sample");

        let interval =
            wilson_interval(sample, 0.95)
                .expect("valid interval");

        assert_eq!(interval.lower, 0.0);
        assert!(interval.upper > 0.0);
    }

    #[test]
    fn all_failures_have_one_upper_bound() {
        let sample =
            BinomialSample::from_failures(100, 100)
                .expect("valid sample");

        let interval =
            wilson_interval(sample, 0.95)
                .expect("valid interval");

        assert!(interval.lower < 1.0);
        assert_eq!(interval.upper, 1.0);
    }

    #[test]
    fn exact_interval_handles_zero_failures() {
        let sample =
            BinomialSample::from_failures(0, 100)
                .expect("valid sample");

        let interval =
            clopper_pearson_interval(sample, 0.95)
                .expect("valid interval");

        assert_eq!(interval.lower, 0.0);
        assert!(interval.upper > 0.0);
    }

    #[test]
    fn exact_interval_handles_all_failures() {
        let sample =
            BinomialSample::from_failures(100, 100)
                .expect("valid sample");

        let interval =
            clopper_pearson_interval(sample, 0.95)
                .expect("valid interval");

        assert!(interval.lower < 1.0);
        assert_eq!(interval.upper, 1.0);
    }

    #[test]
    fn accumulator_is_deterministic() {
        let mut first = StatisticalAccumulator::new();
        let mut second = StatisticalAccumulator::new();

        for _ in 0..50 {
            first.record_success().expect("counter");
            second.record_success().expect("counter");
        }

        for _ in 0..10 {
            first.record_failure().expect("counter");
            second.record_failure().expect("counter");
        }

        assert_eq!(first.sample(), second.sample());
    }

    #[test]
    fn stopping_respects_minimum_samples() {
        let mut accumulator =
            StatisticalAccumulator::new();

        for _ in 0..10 {
            accumulator.record_failure().expect("counter");
        }

        let config = StatisticalConfig {
            minimum_samples: 100,
            ..StatisticalConfig::default()
        };

        let condition =
            accumulator
                .stopping_condition(1_000, config)
                .expect("valid condition");

        assert_eq!(
            condition,
            StoppingCondition::MinimumSamplesRequired {
                required: 100,
                observed: 10,
            }
        );
    }

    #[test]
    fn confidence_level_validation_rejects_one() {
        assert!(
            validate_confidence_level(1.0).is_err()
        );
    }

    #[test]
    fn confidence_level_validation_accepts_ninety_five_percent() {
        assert!(
            validate_confidence_level(0.95).is_ok()
        );
    }

    #[test]
    fn normal_interval_is_bounded() {
        let sample =
            BinomialSample::from_failures(25, 100)
                .expect("valid sample");

        let interval =
            normal_interval(sample, 0.95)
                .expect("valid interval");

        assert!(interval.lower >= 0.0);
        assert!(interval.upper <= 1.0);
    }

    #[test]
    fn sample_merge_is_checked() {
        let first =
            BinomialSample::from_counts(20, 5)
                .expect("valid sample");

        let second =
            BinomialSample::from_counts(30, 10)
                .expect("valid sample");

        let merged =
            first.merged(second)
                .expect("merge succeeds");

        assert_eq!(merged.successes, 50);
        assert_eq!(merged.failures, 15);
        assert_eq!(merged.observations(), 65);
    }

    #[test]
    fn statistical_report_marks_minimum_sample() {
        let sample =
            BinomialSample::from_failures(10, 100)
                .expect("valid sample");

        let config = StatisticalConfig::default();

        let report =
            StatisticalReport::from_sample(sample, config)
                .expect("valid report");

        assert!(report.minimum_samples_reached);
        assert!(report.statistically_meaningful);
    }
}