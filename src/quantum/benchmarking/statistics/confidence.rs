//! Zamani Quantum Benchmarking — Confidence Intervals
//!
//! Canonical statistical confidence/interval primitives used by the quantum
//! benchmarking subsystem.
//!
//! # Design goals
//!
//! This module is deliberately protocol-independent. It provides reusable
//! confidence-level validation and confidence-interval calculations for:
//!
//! - Quantum Volume
//! - randomized benchmarking
//! - interleaved randomized benchmarking
//! - simultaneous randomized benchmarking
//! - purity/leakage benchmarking
//! - cycle benchmarking
//! - layer fidelity
//! - XEB
//! - random-circuit sampling
//! - SPAM/readout characterization
//! - coherence measurements
//! - drift/stability analysis
//! - application benchmarks
//! - physical and logical error-rate measurements
//! - future benchmark protocols
//!
//! The module does **not** execute circuits, generate circuits, perform
//! resampling, or fit protocol-specific models.
//!
//! Those responsibilities belong to:
//!
//! ```text
//! confidence.rs
//!     │
//!     ├── statistics::bootstrap
//!     ├── statistics::regression
//!     ├── statistics::hypothesis
//!     ├── statistics::aggregation
//!     │
//!     ├── metrics::*
//!     └── protocols::*
//! ```
//!
//! # Statistical policy
//!
//! Confidence intervals are not interchangeable with hypothesis tests.
//! This module therefore exposes both the interval itself and the confidence
//! level used to construct it, while leaving protocol-specific pass/fail
//! decisions to the protocol layer.
//!
//! For binomial proportions, Wilson and Clopper-Pearson intervals are
//! supported. Wilson is the default because it has good finite-sample
//! behaviour without the pathological endpoint behaviour of the naive
//! Wald interval.
//!
//! Clopper-Pearson is available when a conservative exact binomial interval
//! is required.
//!
//! # Numerical policy
//!
//! - NaN and infinity are rejected.
//! - Confidence levels must satisfy `0 < level < 1`.
//! - Probabilities must satisfy `0 <= p <= 1`.
//! - Sample counts must be non-zero.
//! - Integer counts must be internally consistent.
//! - Interval bounds are guaranteed to lie in `[0, 1]`.
//! - No external numerical dependency is required.
//! - Algorithms are deterministic.
//!
//! # Rust compatibility
//!
//! This file is written for Rust 1.97 / 1.97.1.
//!
//! # Integration contract
//!
//! This file depends only on the Rust standard library and `serde`.
//!
//! It intentionally does **not** depend on:
//!
//! - benchmark protocols
//! - quantum IR
//! - execution backends
//! - hardware
//! - runtime
//! - `volume_estimator.rs`
//! - future `bootstrap.rs`
//! - future `regression.rs`
//!
//! Therefore this file can be completed and stabilized independently.
//!
//! Later integration:
//!
//! ```text
//! protocols::*
//!      │
//!      ▼
//! statistics::confidence
//!      │
//!      ├── ConfidenceLevel
//!      ├── ConfidenceInterval
//!      ├── BinomialInterval
//!      └── IntervalMethod
//! ```
//!
//! The existing Quantum Volume estimator should eventually delegate its
//! Wilson calculation to this module rather than maintaining a duplicate
//! implementation.

use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;

/// Result type returned by this module.
pub type ConfidenceResult<T> = Result<T, ConfidenceError>;

/// Minimum mathematically meaningful confidence level.
///
/// A value exactly equal to zero is rejected.
pub const MIN_CONFIDENCE_LEVEL_EXCLUSIVE: f64 = 0.0;

/// Maximum mathematically meaningful confidence level.
///
/// A value exactly equal to one is rejected.
pub const MAX_CONFIDENCE_LEVEL_EXCLUSIVE: f64 = 1.0;

/// Production default confidence level.
///
/// This is a generic statistical default only. Individual protocols may
/// require a different level and must state that explicitly.
pub const DEFAULT_CONFIDENCE_LEVEL: f64 = 0.95;

/// Default interval method for binomial proportions.
pub const DEFAULT_BINOMIAL_METHOD: IntervalMethod = IntervalMethod::Wilson;

/// Errors produced by confidence calculations.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConfidenceError {
    /// Confidence level is NaN or infinite.
    NonFiniteConfidenceLevel {
        value: f64,
    },

    /// Confidence level is outside `(0, 1)`.
    InvalidConfidenceLevel {
        value: f64,
    },

    /// Probability is NaN or infinite.
    NonFiniteProbability {
        value: f64,
    },

    /// Probability is outside `[0, 1]`.
    InvalidProbability {
        value: f64,
    },

    /// Sample count is zero.
    ZeroSamples,

    /// Success count exceeds sample count.
    SuccessesExceedSamples {
        successes: usize,
        samples: usize,
    },

    /// Failure count exceeds sample count.
    FailuresExceedSamples {
        failures: usize,
        samples: usize,
    },

    /// Success and failure counts do not form the supplied total.
    InconsistentCounts {
        successes: usize,
        failures: usize,
        samples: usize,
    },

    /// A numerical operation produced a non-finite result.
    NumericalFailure {
        operation: &'static str,
    },

    /// An interval was internally invalid.
    InvalidInterval {
        lower: f64,
        upper: f64,
    },

    /// A beta-function calculation could not be evaluated safely.
    BetaFunctionFailure,

    /// A requested confidence operation is not supported by the selected
    /// method.
    UnsupportedMethod {
        method: IntervalMethod,
    },
}

impl fmt::Display for ConfidenceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFiniteConfidenceLevel { value } => {
                write!(
                    formatter,
                    "confidence level must be finite, got {value}"
                )
            }

            Self::InvalidConfidenceLevel { value } => {
                write!(
                    formatter,
                    "confidence level must be strictly between 0 and 1, got {value}"
                )
            }

            Self::NonFiniteProbability { value } => {
                write!(
                    formatter,
                    "probability must be finite, got {value}"
                )
            }

            Self::InvalidProbability { value } => {
                write!(
                    formatter,
                    "probability must be in [0, 1], got {value}"
                )
            }

            Self::ZeroSamples => {
                write!(
                    formatter,
                    "confidence interval requires at least one sample"
                )
            }

            Self::SuccessesExceedSamples {
                successes,
                samples,
            } => {
                write!(
                    formatter,
                    "success count {successes} exceeds sample count {samples}"
                )
            }

            Self::FailuresExceedSamples {
                failures,
                samples,
            } => {
                write!(
                    formatter,
                    "failure count {failures} exceeds sample count {samples}"
                )
            }

            Self::InconsistentCounts {
                successes,
                failures,
                samples,
            } => {
                write!(
                    formatter,
                    "inconsistent binomial counts: successes={successes}, \
                     failures={failures}, samples={samples}"
                )
            }

            Self::NumericalFailure { operation } => {
                write!(
                    formatter,
                    "non-finite numerical result while performing {operation}"
                )
            }

            Self::InvalidInterval { lower, upper } => {
                write!(
                    formatter,
                    "invalid confidence interval [{lower}, {upper}]"
                )
            }

            Self::BetaFunctionFailure => {
                write!(
                    formatter,
                    "unable to evaluate the required beta-function operation"
                )
            }

            Self::UnsupportedMethod { method } => {
                write!(
                    formatter,
                    "confidence interval method {method:?} is not supported \
                     for this operation"
                )
            }
        }
    }
}

impl Error for ConfidenceError {}

/// Confidence level.
///
/// A confidence level is represented explicitly instead of passing naked
/// `f64` values throughout the benchmarking subsystem.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ConfidenceLevel(f64);

impl ConfidenceLevel {
    /// Creates a validated confidence level.
    pub fn new(value: f64) -> ConfidenceResult<Self> {
        if !value.is_finite() {
            return Err(ConfidenceError::NonFiniteConfidenceLevel { value });
        }

        if !(MIN_CONFIDENCE_LEVEL_EXCLUSIVE..MAX_CONFIDENCE_LEVEL_EXCLUSIVE)
            .contains(&value)
        {
            return Err(ConfidenceError::InvalidConfidenceLevel { value });
        }

        Ok(Self(value))
    }

    /// Returns the default 95% confidence level.
    pub fn default_level() -> Self {
        // The constant is validated by construction and is therefore safe.
        Self(DEFAULT_CONFIDENCE_LEVEL)
    }

    /// Returns the confidence level as a fraction.
    #[inline]
    pub fn value(self) -> f64 {
        self.0
    }

    /// Returns the confidence level as a percentage.
    #[inline]
    pub fn percent(self) -> f64 {
        self.0 * 100.0
    }

    /// Returns the corresponding two-sided tail probability.
    ///
    /// For example:
    ///
    /// - 95% -> 5%
    /// - 99% -> 1%
    #[inline]
    pub fn alpha(self) -> f64 {
        1.0 - self.0
    }

    /// Returns the probability in one tail of a symmetric two-sided interval.
    #[inline]
    pub fn two_sided_tail_probability(self) -> f64 {
        self.alpha() / 2.0
    }
}

impl Default for ConfidenceLevel {
    fn default() -> Self {
        Self::default_level()
    }
}

impl fmt::Display for ConfidenceLevel {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{:.6}", self.0)
    }
}

/// Confidence interval construction method.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IntervalMethod {
    /// Wilson score interval.
    ///
    /// Recommended default for binomial proportions.
    Wilson,

    /// Exact Clopper-Pearson binomial interval.
    ///
    /// Conservative frequentist interval.
    ClopperPearson,

    /// Wald/normal approximation.
    ///
    /// Provided only for compatibility and explicitly discouraged for
    /// small samples or probabilities near 0 and 1.
    Wald,
}

impl IntervalMethod {
    /// Stable machine-readable identifier.
    pub const fn id(self) -> &'static str {
        match self {
            Self::Wilson => "wilson",
            Self::ClopperPearson => "clopper_pearson",
            Self::Wald => "wald",
        }
    }

    /// Returns whether this is a recommended production method for
    /// binomial proportions.
    pub const fn recommended(self) -> bool {
        matches!(self, Self::Wilson | Self::ClopperPearson)
    }
}

/// A validated confidence interval.
///
/// Bounds are always represented as a closed interval `[lower, upper]`.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ConfidenceInterval {
    /// Lower confidence bound.
    pub lower: f64,

    /// Upper confidence bound.
    pub upper: f64,

    /// Confidence level used to construct the interval.
    pub confidence_level: ConfidenceLevel,

    /// Method used to construct the interval.
    pub method: IntervalMethod,
}

impl ConfidenceInterval {
    /// Constructs a validated interval.
    pub fn new(
        lower: f64,
        upper: f64,
        confidence_level: ConfidenceLevel,
        method: IntervalMethod,
    ) -> ConfidenceResult<Self> {
        if !lower.is_finite() {
            return Err(ConfidenceError::NumericalFailure {
                operation: "confidence interval lower bound",
            });
        }

        if !upper.is_finite() {
            return Err(ConfidenceError::NumericalFailure {
                operation: "confidence interval upper bound",
            });
        }

        if lower < 0.0 || upper > 1.0 || lower > upper {
            return Err(ConfidenceError::InvalidInterval { lower, upper });
        }

        Ok(Self {
            lower,
            upper,
            confidence_level,
            method,
        })
    }

    /// Returns the interval width.
    #[inline]
    pub fn width(self) -> f64 {
        self.upper - self.lower
    }

    /// Returns the midpoint.
    #[inline]
    pub fn midpoint(self) -> f64 {
        (self.lower + self.upper) / 2.0
    }

    /// Returns the half-width / margin of error.
    #[inline]
    pub fn margin(self) -> f64 {
        self.width() / 2.0
    }

    /// Returns whether a value is contained in the interval.
    pub fn contains(self, value: f64) -> bool {
        value.is_finite() && value >= self.lower && value <= self.upper
    }

    /// Returns whether the interval contains the boundary value zero.
    #[inline]
    pub fn contains_zero(self) -> bool {
        self.lower <= 0.0 && self.upper >= 0.0
    }

    /// Returns whether the interval contains the boundary value one.
    #[inline]
    pub fn contains_one(self) -> bool {
        self.lower <= 1.0 && self.upper >= 1.0
    }
}

impl fmt::Display for ConfidenceInterval {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "[{:.12}, {:.12}] @ {:.4}% ({})",
            self.lower,
            self.upper,
            self.confidence_level.percent(),
            self.method.id()
        )
    }
}

/// Result of binomial confidence-interval estimation.
///
/// This structure keeps the raw counts alongside the interval so that
/// benchmark results remain auditable and independently re-analyzable.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct BinomialConfidenceInterval {
    /// Number of successes.
    pub successes: usize,

    /// Number of failures.
    pub failures: usize,

    /// Total number of observations.
    pub samples: usize,

    /// Observed proportion.
    pub proportion: f64,

    /// Confidence interval.
    pub interval: ConfidenceInterval,
}

impl BinomialConfidenceInterval {
    /// Returns the number of observations represented by the result.
    #[inline]
    pub fn samples(self) -> usize {
        self.samples
    }

    /// Returns the observed proportion.
    #[inline]
    pub fn proportion(self) -> f64 {
        self.proportion
    }

    /// Returns the confidence interval.
    #[inline]
    pub fn interval(self) -> ConfidenceInterval {
        self.interval
    }

    /// Returns the lower confidence bound.
    #[inline]
    pub fn lower(self) -> f64 {
        self.interval.lower
    }

    /// Returns the upper confidence bound.
    #[inline]
    pub fn upper(self) -> f64 {
        self.interval.upper
    }

    /// Returns whether the entire interval is strictly greater than a
    /// supplied threshold.
    ///
    /// This is intentionally separate from calculating the interval.
    /// Protocols such as Quantum Volume can choose their own decision rule.
    pub fn lower_strictly_above(self, threshold: f64) -> ConfidenceResult<bool> {
        validate_probability(threshold)?;
        Ok(self.lower() > threshold)
    }

    /// Returns whether the entire interval is greater than or equal to a
    /// supplied threshold.
    pub fn lower_at_least(self, threshold: f64) -> ConfidenceResult<bool> {
        validate_probability(threshold)?;
        Ok(self.lower() >= threshold)
    }

    /// Returns whether the entire interval is strictly below a threshold.
    pub fn upper_strictly_below(self, threshold: f64) -> ConfidenceResult<bool> {
        validate_probability(threshold)?;
        Ok(self.upper() < threshold)
    }
}

/// Validates a probability.
///
/// This helper is public because multiple benchmark protocols need exactly
/// the same boundary and non-finite-value semantics.
pub fn validate_probability(value: f64) -> ConfidenceResult<()> {
    if !value.is_finite() {
        return Err(ConfidenceError::NonFiniteProbability { value });
    }

    if !(0.0..=1.0).contains(&value) {
        return Err(ConfidenceError::InvalidProbability { value });
    }

    Ok(())
}

/// Validates a sample count.
pub fn validate_samples(samples: usize) -> ConfidenceResult<()> {
    if samples == 0 {
        return Err(ConfidenceError::ZeroSamples);
    }

    Ok(())
}

/// Calculates a confidence interval for a probability using the supplied
/// interval method.
///
/// This function accepts a measured probability and a sample count. When raw
/// integer counts are available, prefer [`binomial_interval_from_counts`] so
/// the original observation data remains explicit.
pub fn binomial_interval(
    probability: f64,
    samples: usize,
    confidence_level: ConfidenceLevel,
    method: IntervalMethod,
) -> ConfidenceResult<ConfidenceInterval> {
    validate_probability(probability)?;
    validate_samples(samples)?;

    match method {
        IntervalMethod::Wilson => {
            wilson_interval(probability, samples, confidence_level)
        }

        IntervalMethod::Wald => {
            wald_interval(probability, samples, confidence_level)
        }

        IntervalMethod::ClopperPearson => {
            let successes = probability_to_nearest_count(probability, samples)?;

            binomial_interval_from_counts(
                successes,
                samples - successes,
                confidence_level,
                IntervalMethod::ClopperPearson,
            )
            .map(|result| result.interval)
        }
    }
}

/// Calculates a binomial confidence interval directly from raw counts.
///
/// This is the preferred entry point for benchmark protocols when they have
/// raw shot counts.
pub fn binomial_interval_from_counts(
    successes: usize,
    failures: usize,
    confidence_level: ConfidenceLevel,
    method: IntervalMethod,
) -> ConfidenceResult<BinomialConfidenceInterval> {
    let samples = successes
        .checked_add(failures)
        .ok_or(ConfidenceError::NumericalFailure {
            operation: "binomial sample-count addition",
        })?;

    validate_samples(samples)?;

    let proportion = successes as f64 / samples as f64;

    if !proportion.is_finite() {
        return Err(ConfidenceError::NumericalFailure {
            operation: "binomial proportion calculation",
        });
    }

    let interval = match method {
        IntervalMethod::Wilson => {
            wilson_interval(proportion, samples, confidence_level)?
        }

        IntervalMethod::Wald => {
            wald_interval(proportion, samples, confidence_level)?
        }

        IntervalMethod::ClopperPearson => {
            clopper_pearson_interval(
                successes,
                failures,
                confidence_level,
            )?
        }
    };

    Ok(BinomialConfidenceInterval {
        successes,
        failures,
        samples,
        proportion,
        interval,
    })
}

/// Convenience function for success/failure counts when the caller has a
/// single total sample count.
///
/// This performs all consistency checks before calculating the interval.
pub fn binomial_interval_from_total(
    successes: usize,
    samples: usize,
    confidence_level: ConfidenceLevel,
    method: IntervalMethod,
) -> ConfidenceResult<BinomialConfidenceInterval> {
    validate_samples(samples)?;

    if successes > samples {
        return Err(ConfidenceError::SuccessesExceedSamples {
            successes,
            samples,
        });
    }

    let failures = samples - successes;

    binomial_interval_from_counts(
        successes,
        failures,
        confidence_level,
        method,
    )
}

/// Production-default binomial confidence interval.
///
/// Uses a 95% Wilson score interval.
pub fn default_binomial_interval(
    successes: usize,
    samples: usize,
) -> ConfidenceResult<BinomialConfidenceInterval> {
    binomial_interval_from_total(
        successes,
        samples,
        ConfidenceLevel::default(),
        DEFAULT_BINOMIAL_METHOD,
    )
}

/// Calculates a Wilson score confidence interval.
///
/// Wilson is the default interval for Zamani's binomial benchmark metrics.
pub fn wilson_interval(
    probability: f64,
    samples: usize,
    confidence_level: ConfidenceLevel,
) -> ConfidenceResult<ConfidenceInterval> {
    validate_probability(probability)?;
    validate_samples(samples)?;

    let z = standard_normal_quantile(
        confidence_level.two_sided_tail_probability(),
    )?
    .abs();

    let n = samples as f64;
    let z_squared = z * z;

    let denominator = 1.0 + z_squared / n;

    let center =
        (probability + z_squared / (2.0 * n)) / denominator;

    let variance_term =
        probability * (1.0 - probability) / n
            + z_squared / (4.0 * n * n);

    if !variance_term.is_finite() || variance_term < 0.0 {
        return Err(ConfidenceError::NumericalFailure {
            operation: "Wilson variance term",
        });
    }

    let margin =
        z * variance_term.sqrt() / denominator;

    let lower = clamp_unit_interval(center - margin);
    let upper = clamp_unit_interval(center + margin);

    ConfidenceInterval::new(
        lower,
        upper,
        confidence_level,
        IntervalMethod::Wilson,
    )
}

/// Calculates the Wald/normal approximation interval.
///
/// This method is intentionally available but should generally not be used
/// for production quantum benchmarking at small sample sizes or near
/// probability boundaries.
///
/// Prefer Wilson or Clopper-Pearson.
pub fn wald_interval(
    probability: f64,
    samples: usize,
    confidence_level: ConfidenceLevel,
) -> ConfidenceResult<ConfidenceInterval> {
    validate_probability(probability)?;
    validate_samples(samples)?;

    let z = standard_normal_quantile(
        confidence_level.two_sided_tail_probability(),
    )?
    .abs();

    let n = samples as f64;

    let variance =
        probability * (1.0 - probability) / n;

    if !variance.is_finite() || variance < 0.0 {
        return Err(ConfidenceError::NumericalFailure {
            operation: "Wald variance",
        });
    }

    let margin = z * variance.sqrt();

    let lower = clamp_unit_interval(probability - margin);
    let upper = clamp_unit_interval(probability + margin);

    ConfidenceInterval::new(
        lower,
        upper,
        confidence_level,
        IntervalMethod::Wald,
    )
}

/// Calculates an exact Clopper-Pearson interval.
///
/// The calculation uses inverse regularized incomplete beta functions
/// implemented locally, avoiding a large numerical dependency.
///
/// This method is conservative by construction and is particularly useful
/// when a protocol requires an exact binomial confidence procedure.
pub fn clopper_pearson_interval(
    successes: usize,
    failures: usize,
    confidence_level: ConfidenceLevel,
) -> ConfidenceResult<ConfidenceInterval> {
    let samples = successes
        .checked_add(failures)
        .ok_or(ConfidenceError::NumericalFailure {
            operation: "Clopper-Pearson sample-count addition",
        })?;

    validate_samples(samples)?;

    let alpha = confidence_level.alpha();
    let lower_tail = alpha / 2.0;
    let upper_tail = 1.0 - alpha / 2.0;

    let lower = if successes == 0 {
        0.0
    } else {
        inverse_regularized_beta(
            lower_tail,
            successes as f64,
            failures as f64 + 1.0,
        )?
    };

    let upper = if failures == 0 {
        1.0
    } else {
        inverse_regularized_beta(
            upper_tail,
            successes as f64 + 1.0,
            failures as f64,
        )?
    };

    ConfidenceInterval::new(
        clamp_unit_interval(lower),
        clamp_unit_interval(upper),
        confidence_level,
        IntervalMethod::ClopperPearson,
    )
}

/// Converts a probability to the nearest representable integer count.
///
/// This function exists for APIs where only a probability was supplied.
/// It should not be used when raw counts are available.
fn probability_to_nearest_count(
    probability: f64,
    samples: usize,
) -> ConfidenceResult<usize> {
    validate_probability(probability)?;
    validate_samples(samples)?;

    let raw = probability * samples as f64;

    if !raw.is_finite() {
        return Err(ConfidenceError::NumericalFailure {
            operation: "probability-to-count conversion",
        });
    }

    let rounded = raw.round();

    if rounded < 0.0 || rounded > samples as f64 {
        return Err(ConfidenceError::NumericalFailure {
            operation: "probability-to-count range validation",
        });
    }

    Ok(rounded as usize)
}

/// Calculates the inverse standard-normal CDF.
///
/// This uses the Acklam rational approximation. The approximation is
/// sufficiently accurate for confidence-interval construction while keeping
/// the benchmarking core dependency-light.
///
/// `p` must satisfy `0 < p < 1`.
pub fn standard_normal_quantile(p: f64) -> ConfidenceResult<f64> {
    if !p.is_finite() {
        return Err(ConfidenceError::NumericalFailure {
            operation: "normal quantile input",
        });
    }

    if !(0.0..1.0).contains(&p) {
        return Err(ConfidenceError::InvalidProbability { value: p });
    }

    // Coefficients from the Acklam inverse-normal approximation.
    const A: [f64; 6] = [
        -39.69683028665376,
        220.9460984245205,
        -275.9285104469687,
        138.357.751.867.269,
        -30.66479806614716,
        2.506628277459239,
    ];

    const B: [f64; 5] = [
        -54.47609879822406,
        161.5858368580409,
        -155.6989798598866,
        66.80131188771972,
        -13.28068155288572,
    ];

    const C: [f64; 6] = [
        -0.007784894002430293,
        -0.3223964580411365,
        -2.400758277161838,
        -2.549732539343734,
        4.374664141464968,
        2.938163982698783,
    ];

    const D: [f64; 4] = [
        0.007784695709041462,
        0.3224671290700398,
        2.445134137142996,
        3.754408661907416,
    ];

    const LOW: f64 = 0.02425;
    const HIGH: f64 = 1.0 - LOW;

    let result = if p < LOW {
        let q = (-2.0 * p.ln()).sqrt();

        (((((C[0] * q + C[1]) * q + C[2]) * q + C[3]) * q
            + C[4])
            * q
            + C[5])
            /
            ((((D[0] * q + D[1]) * q + D[2]) * q + D[3]) * q)
                + 1.0)
    } else if p > HIGH {
        let q = (-2.0 * (1.0 - p).ln()).sqrt();

        -(((((C[0] * q + C[1]) * q + C[2]) * q + C[3]) * q
            + C[4])
            * q
            + C[5])
            /
            ((((D[0] * q + D[1]) * q + D[2]) * q + D[3]) * q)
                + 1.0)
    } else {
        let q = p - 0.5;
        let r = q * q;

        (((((A[0] * r + A[1]) * r + A[2]) * r + A[3]) * r
            + A[4])
            * r
            + A[5])
            * q)
            /
            (((((B[0] * r + B[1]) * r + B[2]) * r + B[3]) * r
                + B[4])
                * r)
                + 1.0)
    };

    if !result.is_finite() {
        return Err(ConfidenceError::NumericalFailure {
            operation: "normal quantile evaluation",
        });
    }

    Ok(result)
}

/// Clamps a value into the probability interval.
///
/// This is used only after a validated numerical calculation. It does not
/// replace input validation.
#[inline]
fn clamp_unit_interval(value: f64) -> f64 {
    value.clamp(0.0, 1.0)
}

// ---------------------------------------------------------------------------
// Regularized incomplete beta implementation
// ---------------------------------------------------------------------------
//
// These routines are required for Clopper-Pearson intervals.
//
// The implementation follows the standard continued-fraction formulation
// from Numerical Recipes / Cephes-style algorithms. The tolerance is chosen
// to provide stable benchmark-level results without requiring an external
// special-functions crate.

const BETA_EPSILON: f64 = 3.0e-14;
const BETA_MAX_ITERATIONS: usize = 10_000;
const BETA_FPMIN: f64 = 1.0e-300;

/// Natural logarithm of the beta function.
///
/// Computes:
///
///     ln(Beta(a, b))
///
/// using the Lanczos approximation for `ln(Gamma(x))`.
fn ln_beta(a: f64, b: f64) -> ConfidenceResult<f64> {
    if !a.is_finite() || !b.is_finite() || a <= 0.0 || b <= 0.0 {
        return Err(ConfidenceError::BetaFunctionFailure);
    }

    let value = ln_gamma(a)? + ln_gamma(b)? - ln_gamma(a + b)?;

    if !value.is_finite() {
        return Err(ConfidenceError::BetaFunctionFailure);
    }

    Ok(value)
}

/// Natural logarithm of the gamma function.
///
/// Lanczos approximation with reflection for `x < 0.5`.
fn ln_gamma(x: f64) -> ConfidenceResult<f64> {
    if !x.is_finite() || x <= 0.0 {
        return Err(ConfidenceError::BetaFunctionFailure);
    }

    const COEFFICIENTS: [f64; 9] = [
        0.99999999999980993,
        676.5203681218851,
        -1259.1392167224028,
        771.32342877765313,
        -176.61502916214059,
        12.507343278686905,
        -0.13857109526572012,
        9.9843695780195716e-6,
        1.5056327351493116e-7,
    ];

    const HALF_LOG_TWO_PI: f64 =
        0.91893853320467274178032973640562;

    let result = if x < 0.5 {
        // Euler reflection formula:
        //
        // Gamma(x) Gamma(1-x) = pi / sin(pi x)
        let reflected = std::f64::consts::PI
            - (std::f64::consts::PI * x).sin().abs().ln()
            - ln_gamma(1.0 - x)?;

        reflected
    } else {
        let z = x - 1.0;

        let mut series = COEFFICIENTS[0];

        for (index, coefficient) in
            COEFFICIENTS.iter().enumerate().skip(1)
        {
            series += coefficient / (z + index as f64);
        }

        let t = z + 7.5;

        HALF_LOG_TWO_PI
            + (z + 0.5) * t.ln()
            - t
            + series.ln()
    };

    if !result.is_finite() {
        return Err(ConfidenceError::BetaFunctionFailure);
    }

    Ok(result)
}

/// Continued fraction for the incomplete beta function.
fn beta_continued_fraction(
    a: f64,
    b: f64,
    x: f64,
) -> ConfidenceResult<f64> {
    if a <= 0.0 || b <= 0.0 {
        return Err(ConfidenceError::BetaFunctionFailure);
    }

    if !(0.0..=1.0).contains(&x) {
        return Err(ConfidenceError::InvalidProbability { value: x });
    }

    let qab = a + b;
    let qap = a + 1.0;
    let qam = a - 1.0;

    let mut c = 1.0;
    let mut d = 1.0 - qab * x / qap;

    if d.abs() < BETA_FPMIN {
        d = BETA_FPMIN;
    }

    d = 1.0 / d;

    let mut h = d;

    for m in 1..=BETA_MAX_ITERATIONS {
        let m_f = m as f64;
        let m2 = 2.0 * m_f;

        let aa =
            m_f * (b - m_f) * x
                / ((qam + m2) * (a + m2));

        d = 1.0 + aa * d;

        if d.abs() < BETA_FPMIN {
            d = BETA_FPMIN;
        }

        c = 1.0 + aa / c;

        if c.abs() < BETA_FPMIN {
            c = BETA_FPMIN;
        }

        d = 1.0 / d;

        h *= d * c;

        let aa =
            -((a + m_f) * (qab + m_f) * x)
                / ((a + m2) * (qap + m2));

        d = 1.0 + aa * d;

        if d.abs() < BETA_FPMIN {
            d = BETA_FPMIN;
        }

        c = 1.0 + aa / c;

        if c.abs() < BETA_FPMIN {
            c = BETA_FPMIN;
        }

        d = 1.0 / d;

        let delta = d * c;

        h *= delta;

        if !h.is_finite() {
            return Err(ConfidenceError::BetaFunctionFailure);
        }

        if (delta - 1.0).abs() <= BETA_EPSILON {
            return Ok(h);
        }
    }

    Err(ConfidenceError::NumericalFailure {
        operation: "incomplete beta continued fraction convergence",
    })
}

/// Regularized incomplete beta function.
///
/// Computes:
///
///     I_x(a,b)
///
/// for `a > 0`, `b > 0`, `0 <= x <= 1`.
fn regularized_incomplete_beta(
    x: f64,
    a: f64,
    b: f64,
) -> ConfidenceResult<f64> {
    if !x.is_finite() {
        return Err(ConfidenceError::NonFiniteProbability { value: x });
    }

    if !(0.0..=1.0).contains(&x) {
        return Err(ConfidenceError::InvalidProbability { value: x });
    }

    if a <= 0.0 || b <= 0.0 || !a.is_finite() || !b.is_finite() {
        return Err(ConfidenceError::BetaFunctionFailure);
    }

    if x == 0.0 {
        return Ok(0.0);
    }

    if x == 1.0 {
        return Ok(1.0);
    }

    let log_factor =
        a * x.ln()
            + b * (1.0 - x).ln()
            - ln_beta(a, b)?;

    let factor = log_factor.exp();

    if !factor.is_finite() {
        return Err(ConfidenceError::BetaFunctionFailure);
    }

    let result = if x < (a + 1.0) / (a + b + 2.0) {
        factor
            * beta_continued_fraction(a, b, x)?
            / a
    } else {
        1.0
            - factor
                * beta_continued_fraction(b, a, 1.0 - x)?
                / b
    };

    if !result.is_finite() {
        return Err(ConfidenceError::BetaFunctionFailure);
    }

    Ok(clamp_unit_interval(result))
}

/// Inverse regularized incomplete beta.
///
/// Finds `x` such that:
///
///     I_x(a,b) = target
///
/// using deterministic bisection.
///
/// Bisection is deliberately used instead of a Newton-only method because
/// benchmark reproducibility is more important than shaving a small number
/// of iterations from this relatively inexpensive calculation.
fn inverse_regularized_beta(
    target: f64,
    a: f64,
    b: f64,
) -> ConfidenceResult<f64> {
    validate_probability(target)?;

    if a <= 0.0 || b <= 0.0 || !a.is_finite() || !b.is_finite() {
        return Err(ConfidenceError::BetaFunctionFailure);
    }

    if target == 0.0 {
        return Ok(0.0);
    }

    if target == 1.0 {
        return Ok(1.0);
    }

    let mut lower = 0.0;
    let mut upper = 1.0;

    // The number of iterations is fixed to preserve deterministic behaviour.
    // 256 iterations is vastly beyond what is needed for double precision
    // bisection but gives ample protection against difficult parameter sets.
    for _ in 0..256 {
        let midpoint = lower + (upper - lower) / 2.0;

        let value =
            regularized_incomplete_beta(midpoint, a, b)?;

        if !value.is_finite() {
            return Err(ConfidenceError::BetaFunctionFailure);
        }

        if value < target {
            lower = midpoint;
        } else {
            upper = midpoint;
        }

        if (upper - lower).abs() <= 1.0e-14 {
            break;
        }
    }

    let result = lower + (upper - lower) / 2.0;

    if !result.is_finite() {
        return Err(ConfidenceError::BetaFunctionFailure);
    }

    Ok(clamp_unit_interval(result))
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn confidence_level_accepts_standard_values() {
        let level = ConfidenceLevel::new(0.95).unwrap();

        assert!((level.value() - 0.95).abs() < 1e-15);
        assert!((level.percent() - 95.0).abs() < 1e-12);
        assert!((level.alpha() - 0.05).abs() < 1e-12);
    }

    #[test]
    fn confidence_level_rejects_zero() {
        assert!(matches!(
            ConfidenceLevel::new(0.0),
            Err(ConfidenceError::InvalidConfidenceLevel { .. })
        ));
    }

    #[test]
    fn confidence_level_rejects_one() {
        assert!(matches!(
            ConfidenceLevel::new(1.0),
            Err(ConfidenceError::InvalidConfidenceLevel { .. })
        ));
    }

    #[test]
    fn confidence_level_rejects_nan() {
        assert!(matches!(
            ConfidenceLevel::new(f64::NAN),
            Err(ConfidenceError::NonFiniteConfidenceLevel { .. })
        ));
    }

    #[test]
    fn probability_validation_is_strict() {
        assert!(validate_probability(0.0).is_ok());
        assert!(validate_probability(1.0).is_ok());

        assert!(validate_probability(-0.0001).is_err());
        assert!(validate_probability(1.0001).is_err());
        assert!(validate_probability(f64::NAN).is_err());
        assert!(validate_probability(f64::INFINITY).is_err());
    }

    #[test]
    fn zero_samples_are_rejected() {
        assert!(matches!(
            validate_samples(0),
            Err(ConfidenceError::ZeroSamples)
        ));
    }

    #[test]
    fn wilson_interval_is_valid() {
        let level = ConfidenceLevel::new(0.95).unwrap();

        let interval =
            wilson_interval(0.5, 100, level).unwrap();

        assert!(interval.lower >= 0.0);
        assert!(interval.upper <= 1.0);
        assert!(interval.lower < interval.upper);
        assert!(interval.contains(0.5));
    }

    #[test]
    fn wilson_zero_successes_is_valid() {
        let result =
            binomial_interval_from_total(
                0,
                100,
                ConfidenceLevel::default(),
                IntervalMethod::Wilson,
            )
            .unwrap();

        assert_eq!(result.proportion, 0.0);
        assert_eq!(result.lower(), 0.0);
        assert!(result.upper() > 0.0);
    }

    #[test]
    fn wilson_all_successes_is_valid() {
        let result =
            binomial_interval_from_total(
                100,
                100,
                ConfidenceLevel::default(),
                IntervalMethod::Wilson,
            )
            .unwrap();

        assert_eq!(result.proportion, 1.0);
        assert_eq!(result.upper(), 1.0);
        assert!(result.lower() < 1.0);
    }

    #[test]
    fn binomial_counts_are_preserved() {
        let result =
            binomial_interval_from_counts(
                67,
                33,
                ConfidenceLevel::default(),
                IntervalMethod::Wilson,
            )
            .unwrap();

        assert_eq!(result.successes, 67);
        assert_eq!(result.failures, 33);
        assert_eq!(result.samples, 100);
        assert!((result.proportion - 0.67).abs() < 1e-15);
    }

    #[test]
    fn inconsistent_counts_are_impossible_by_construction() {
        let result =
            binomial_interval_from_total(
                101,
                100,
                ConfidenceLevel::default(),
                IntervalMethod::Wilson,
            );

        assert!(matches!(
            result,
            Err(ConfidenceError::SuccessesExceedSamples { .. })
        ));
    }

    #[test]
    fn threshold_decisions_are_separate_from_interval_construction() {
        let result =
            binomial_interval_from_total(
                90,
                100,
                ConfidenceLevel::default(),
                IntervalMethod::Wilson,
            )
            .unwrap();

        assert!(result.lower_strictly_above(0.5).unwrap());
        assert!(!result.lower_strictly_above(0.99).unwrap());
    }

    #[test]
    fn normal_quantile_is_approximately_standard() {
        let zero =
            standard_normal_quantile(0.5).unwrap();

        assert!(zero.abs() < 1e-10);

        let one_point_nine_six =
            standard_normal_quantile(0.975).unwrap();

        assert!(
            (one_point_nine_six - 1.95996398454005).abs()
                < 1e-7
        );
    }

    #[test]
    fn normal_quantile_rejects_endpoints() {
        assert!(standard_normal_quantile(0.0).is_err());
        assert!(standard_normal_quantile(1.0).is_err());
    }

    #[test]
    fn normal_quantile_rejects_non_finite() {
        assert!(standard_normal_quantile(f64::NAN).is_err());
        assert!(standard_normal_quantile(f64::INFINITY).is_err());
    }

    #[test]
    fn clopper_pearson_zero_successes() {
        let result =
            clopper_pearson_interval(
                0,
                100,
                ConfidenceLevel::default(),
            )
            .unwrap();

        assert_eq!(result.lower, 0.0);
        assert!(result.upper > 0.0);
        assert!(result.upper <= 1.0);
    }

    #[test]
    fn clopper_pearson_all_successes() {
        let result =
            clopper_pearson_interval(
                100,
                0,
                ConfidenceLevel::default(),
            )
            .unwrap();

        assert!(result.lower < 1.0);
        assert_eq!(result.upper, 1.0);
    }

    #[test]
    fn clopper_pearson_is_inside_unit_interval() {
        let result =
            clopper_pearson_interval(
                67,
                33,
                ConfidenceLevel::default(),
            )
            .unwrap();

        assert!(result.lower >= 0.0);
        assert!(result.upper <= 1.0);
        assert!(result.lower <= result.upper);
    }

    #[test]
    fn interval_width_and_margin_are_consistent() {
        let interval =
            wilson_interval(
                0.5,
                100,
                ConfidenceLevel::default(),
            )
            .unwrap();

        assert!(
            (interval.width() - 2.0 * interval.margin()).abs()
                < 1e-15
        );
    }

    #[test]
    fn default_interval_is_wilson_95_percent() {
        let result =
            default_binomial_interval(50, 100)
                .unwrap();

        assert_eq!(
            result.interval.method,
            IntervalMethod::Wilson
        );

        assert!(
            (result.interval.confidence_level.value() - 0.95)
                .abs()
                < 1e-15
        );
    }

    #[test]
    fn interval_serialization_round_trips() {
        let interval =
            wilson_interval(
                0.75,
                1000,
                ConfidenceLevel::new(0.99).unwrap(),
            )
            .unwrap();

        let encoded =
            serde_json::to_string(&interval).unwrap();

        let decoded: ConfidenceInterval =
            serde_json::from_str(&encoded).unwrap();

        assert_eq!(interval, decoded);
    }

    #[test]
    fn method_ids_are_stable() {
        assert_eq!(
            IntervalMethod::Wilson.id(),
            "wilson"
        );

        assert_eq!(
            IntervalMethod::ClopperPearson.id(),
            "clopper_pearson"
        );

        assert_eq!(
            IntervalMethod::Wald.id(),
            "wald"
        );
    }
}