//! Zamani Quantum Benchmarking — Statistical Aggregation
//!
//! Production statistical aggregation primitives for quantum benchmarking.
//!
//! # Purpose
//!
//! This module provides deterministic, protocol-independent aggregation of
//! already-observed numerical benchmark samples.
//!
//! It is intended to be consumed by:
//!
//! - Quantum Volume analysis;
//! - randomized benchmarking;
//! - interleaved randomized benchmarking;
//! - simultaneous randomized benchmarking;
//! - purity/leakage randomized benchmarking;
//! - cycle benchmarking;
//! - layer-fidelity analysis;
//! - XEB;
//! - random-circuit sampling;
//! - application benchmarks;
//! - VQE;
//! - QAOA;
//! - QEC experiments;
//! - coherence measurements;
//! - crosstalk measurements;
//! - drift analysis;
//! - throughput analysis;
//! - latency analysis;
//! - volumetric benchmarking;
//! - regression/baseline analysis;
//! - future quantum benchmark protocols.
//!
//! # Architectural position
//!
//! ```text
//!                    Quantum Benchmark
//!                           │
//!                           ▼
//!                    raw observations
//!                           │
//!             ┌─────────────┴─────────────┐
//!             │                           │
//!             ▼                           ▼
//!       confidence                    aggregation
//!             │                           │
//!             └─────────────┬─────────────┘
//!                           ▼
//!                       regression
//!                           │
//!                           ▼
//!                    benchmark metrics
//!                           │
//!                           ▼
//!                    BenchmarkResult
//! ```
//!
//! This module deliberately does NOT:
//!
//! - generate quantum circuits;
//! - execute quantum circuits;
//! - understand quantum gates;
//! - understand hardware;
//! - understand backend vendors;
//! - perform protocol-specific physics;
//! - silently discard observations;
//! - silently remove outliers;
//! - perform I/O;
//! - log to stdout/stderr;
//! - maintain global mutable state.
//!
//! # Production guarantees
//!
//! - Empty input is rejected.
//! - NaN and infinite observations are rejected.
//! - NaN and infinite weights are rejected.
//! - Negative weights are rejected.
//! - Zero total weight is rejected for weighted aggregation.
//! - Arithmetic uses numerically stable algorithms where applicable.
//! - Weighted variance uses an explicitly documented unbiased frequency-weight
//!   convention.
//! - Effective sample size is reported for weighted observations.
//! - Median and quantiles use deterministic ordering.
//! - No observation is silently removed.
//! - No sorting is performed unless the selected operation requires it.
//! - Sorting allocation is bounded by an explicit aggregation limit.
//! - Integer overflow in counts is checked.
//! - Floating-point overflow/non-finite intermediate values are detected.
//! - The implementation requires no external numerical library.
//! - The implementation is compatible with Rust 1.97 and Rust 1.97.1.
//!
//! # Statistical policy
//!
//! Aggregation is not inference.
//!
//! A mean, median, variance, or weighted estimate does not establish:
//!
//! - independence of observations;
//! - stationarity of hardware;
//! - absence of drift;
//! - absence of correlated errors;
//! - causal explanations;
//! - physical correctness;
//! - benchmark validity.
//!
//! Those decisions belong to the relevant protocol, validation, confidence,
//! provenance, and analysis layers.
//!
//! # Weight semantics
//!
//! A weight represents the contribution of an observation to a weighted
//! estimate. Weights must be finite and non-negative.
//!
//! For weighted variance, Zamani uses the unbiased frequency-weight estimator:
//!
//! ```text
//!             W * Σ(wᵢ (xᵢ - μ)²)
//! variance = -----------------------
//!             W² - Σ(wᵢ²)
//! ```
//!
//! where:
//!
//! ```text
//! W = Σwᵢ
//! μ = Σ(wᵢxᵢ) / W
//! ```
//!
//! This estimator is appropriate when weights represent relative observation
//! frequencies. It must not be interpreted as a universally unbiased estimator
//! for arbitrary survey/design weights. Protocols must document their weight
//! semantics.
//!
//! # Integration contract
//!
//! This module intentionally has no dependency on other Zamani benchmarking
//! modules. It can therefore be completed before:
//!
//! - `core/metric.rs`;
//! - `core/result.rs`;
//! - `core/limits.rs`;
//! - `statistics/confidence.rs`;
//! - `statistics/bootstrap.rs`;
//! - `statistics/regression.rs`.
//!
//! Later integration should use this module rather than reimplementing means,
//! medians, weighted estimates, or variance calculations.
//!
//! Recommended downstream direction:
//!
//! ```text
//! statistics::aggregation
//!          │
//!          ├── statistics::confidence
//!          ├── statistics::bootstrap
//!          ├── statistics::regression
//!          ├── metrics::*
//!          └── protocols::*
//! ```
//!
//! The dependency direction must never be reversed.
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

use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::error::Error;
use std::fmt;
use std::mem;

/// Stable algorithm identifier.
///
/// If the mathematical behavior of this implementation changes in a way that
/// can alter benchmark results, this identifier must be versioned.
pub const AGGREGATION_ALGORITHM_ID: &str = "zamani.statistics.aggregation.v1";

/// Default maximum number of observations accepted by one aggregation request.
///
/// The limit prevents accidentally attempting extremely large sorting
/// operations through malformed or untrusted benchmark input.
///
/// Protocols requiring a different limit should configure it explicitly or
/// translate their authoritative `BenchmarkLimits` into `AggregationLimits`.
pub const DEFAULT_MAX_OBSERVATIONS: usize = 1_000_000;

/// Default maximum number of bytes that an operation may allocate for a
/// temporary sorted copy.
///
/// This is intentionally conservative and can be overridden explicitly.
pub const DEFAULT_MAX_SORT_BYTES: usize = 256 * 1024 * 1024;

/// Result type returned by aggregation operations.
pub type AggregationResult<T> = Result<T, AggregationError>;

/// Errors produced by statistical aggregation.
#[derive(Debug, Clone, PartialEq)]
pub enum AggregationError {
    /// The supplied observation set contains no observations.
    EmptyObservations,

    /// The observation set exceeds the configured production limit.
    ObservationLimitExceeded {
        /// Number of supplied observations.
        observations: usize,

        /// Maximum permitted observations.
        maximum: usize,
    },

    /// A required temporary sorting allocation would exceed the configured
    /// memory budget.
    SortAllocationLimitExceeded {
        /// Number of elements that would be copied.
        elements: usize,

        /// Size of one element.
        element_size: usize,

        /// Maximum permitted allocation.
        maximum_bytes: usize,
    },

    /// An observation is NaN or infinite.
    NonFiniteObservation {
        /// Zero-based observation index.
        index: usize,

        /// Invalid observation.
        value: f64,
    },

    /// A supplied weight is NaN or infinite.
    NonFiniteWeight {
        /// Zero-based observation index.
        index: usize,

        /// Invalid weight.
        weight: f64,
    },

    /// Negative weights are forbidden.
    NegativeWeight {
        /// Zero-based observation index.
        index: usize,

        /// Invalid weight.
        weight: f64,
    },

    /// Weighted aggregation was requested but all weights are zero.
    ZeroTotalWeight,

    /// Weighted variance cannot be calculated because its denominator is not
    /// positive.
    InsufficientWeightedDegreesOfFreedom,

    /// At least two observations are required for sample variance.
    InsufficientVarianceObservations {
        /// Number of observations supplied.
        observations: usize,
    },

    /// The calculated result became NaN or infinite.
    NonFiniteResult {
        /// Operation that produced the invalid result.
        operation: &'static str,

        /// Invalid value.
        value: f64,
    },

    /// A quantile was outside [0, 1].
    InvalidQuantile {
        /// Requested quantile.
        quantile: f64,
    },

    /// A requested divisor is zero.
    DivisionByZero {
        /// Operation requiring the divisor.
        operation: &'static str,
    },

    /// A count conversion or arithmetic operation overflowed.
    ArithmeticOverflow {
        /// Operation that overflowed.
        operation: &'static str,
    },

    /// A weighted operation received no positive-weight observations.
    NoPositiveWeightObservations,

    /// The requested aggregation method is incompatible with the supplied
    /// input representation.
    UnsupportedOperation {
        /// Description of the incompatibility.
        message: String,
    },
}

impl fmt::Display for AggregationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyObservations => {
                write!(formatter, "aggregation requires at least one observation")
            }

            Self::ObservationLimitExceeded {
                observations,
                maximum,
            } => {
                write!(
                    formatter,
                    "aggregation observation limit exceeded: \
                     observations={observations}, maximum={maximum}"
                )
            }

            Self::SortAllocationLimitExceeded {
                elements,
                element_size,
                maximum_bytes,
            } => {
                write!(
                    formatter,
                    "aggregation sort allocation limit exceeded: \
                     elements={elements}, element_size={element_size}, \
                     maximum_bytes={maximum_bytes}"
                )
            }

            Self::NonFiniteObservation { index, value } => {
                write!(
                    formatter,
                    "aggregation observation {index} is non-finite: {value}"
                )
            }

            Self::NonFiniteWeight { index, weight } => {
                write!(
                    formatter,
                    "aggregation weight {index} is non-finite: {weight}"
                )
            }

            Self::NegativeWeight { index, weight } => {
                write!(
                    formatter,
                    "aggregation weight {index} is negative: {weight}"
                )
            }

            Self::ZeroTotalWeight => {
                write!(formatter, "weighted aggregation has zero total weight")
            }

            Self::InsufficientWeightedDegreesOfFreedom => {
                write!(
                    formatter,
                    "weighted variance has insufficient degrees of freedom"
                )
            }

            Self::InsufficientVarianceObservations { observations } => {
                write!(
                    formatter,
                    "sample variance requires at least two observations, \
                     got {observations}"
                )
            }

            Self::NonFiniteResult { operation, value } => {
                write!(
                    formatter,
                    "aggregation operation {operation} produced a non-finite \
                     result: {value}"
                )
            }

            Self::InvalidQuantile { quantile } => {
                write!(
                    formatter,
                    "quantile must be finite and within [0, 1], got {quantile}"
                )
            }

            Self::DivisionByZero { operation } => {
                write!(
                    formatter,
                    "aggregation operation {operation} attempted division \
                     by zero"
                )
            }

            Self::ArithmeticOverflow { operation } => {
                write!(
                    formatter,
                    "integer arithmetic overflow during aggregation operation \
                     {operation}"
                )
            }

            Self::NoPositiveWeightObservations => {
                write!(
                    formatter,
                    "weighted aggregation requires at least one \
                     positive-weight observation"
                )
            }

            Self::UnsupportedOperation { message } => {
                write!(formatter, "unsupported aggregation operation: {message}")
            }
        }
    }
}

impl Error for AggregationError {}

/// One numerical observation.
///
/// The optional weight is deliberately stored with the observation so that
/// weighting cannot become detached from its associated measurement.
///
/// A `None` weight means unit weight.
///
/// A `Some(0.0)` weight is valid but contributes no mass to weighted
/// calculations. It is retained rather than silently discarded.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Observation {
    /// Numerical observation.
    pub value: f64,

    /// Optional non-negative weight.
    pub weight: Option<f64>,
}

impl Observation {
    /// Creates an unweighted observation.
    pub fn new(value: f64) -> AggregationResult<Self> {
        validate_finite(value, 0)?;

        Ok(Self {
            value,
            weight: None,
        })
    }

    /// Creates a weighted observation.
    pub fn weighted(value: f64, weight: f64) -> AggregationResult<Self> {
        validate_finite(value, 0)?;
        validate_weight(weight, 0)?;

        Ok(Self {
            value,
            weight: Some(weight),
        })
    }

    /// Returns the effective weight.
    #[inline]
    pub fn effective_weight(self) -> f64 {
        self.weight.unwrap_or(1.0)
    }
}

/// Convenience conversion from a bare `f64`.
impl TryFrom<f64> for Observation {
    type Error = AggregationError;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

/// Production resource limits for one aggregation operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct AggregationLimits {
    /// Maximum number of observations accepted.
    pub max_observations: usize,

    /// Maximum temporary allocation used by sorting operations.
    pub max_sort_bytes: usize,
}

impl Default for AggregationLimits {
    fn default() -> Self {
        Self {
            max_observations: DEFAULT_MAX_OBSERVATIONS,
            max_sort_bytes: DEFAULT_MAX_SORT_BYTES,
        }
    }
}

impl AggregationLimits {
    /// Creates explicit aggregation limits.
    pub const fn new(
        max_observations: usize,
        max_sort_bytes: usize,
    ) -> Self {
        Self {
            max_observations,
            max_sort_bytes,
        }
    }

    /// Validates the limits themselves.
    pub fn validate(&self) -> AggregationResult<()> {
        if self.max_observations == 0 {
            return Err(AggregationError::UnsupportedOperation {
                message: "max_observations must be greater than zero".to_string(),
            });
        }

        if self.max_sort_bytes == 0 {
            return Err(AggregationError::UnsupportedOperation {
                message: "max_sort_bytes must be greater than zero".to_string(),
            });
        }

        Ok(())
    }

    fn check_observations(&self, observations: usize) -> AggregationResult<()> {
        self.validate()?;

        if observations == 0 {
            return Err(AggregationError::EmptyObservations);
        }

        if observations > self.max_observations {
            return Err(AggregationError::ObservationLimitExceeded {
                observations,
                maximum: self.max_observations,
            });
        }

        Ok(())
    }

    fn check_sort_allocation(&self, elements: usize) -> AggregationResult<()> {
        let element_size = mem::size_of::<Observation>();

        let bytes = elements
            .checked_mul(element_size)
            .ok_or(AggregationError::ArithmeticOverflow {
                operation: "sort allocation size",
            })?;

        if bytes > self.max_sort_bytes {
            return Err(AggregationError::SortAllocationLimitExceeded {
                elements,
                element_size,
                maximum_bytes: self.max_sort_bytes,
            });
        }

        Ok(())
    }
}

/// Statistical aggregation operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AggregationMethod {
    /// Arithmetic mean.
    Mean,

    /// Weighted arithmetic mean.
    WeightedMean,

    /// Sum of observations.
    Sum,

    /// Weighted sum.
    WeightedSum,

    /// Minimum observation.
    Minimum,

    /// Maximum observation.
    Maximum,

    /// Median.
    Median,

    /// Weighted median.
    WeightedMedian,

    /// Population variance.
    PopulationVariance,

    /// Sample variance.
    SampleVariance,

    /// Weighted population variance.
    WeightedPopulationVariance,

    /// Weighted unbiased frequency variance.
    WeightedSampleVariance,

    /// Population standard deviation.
    PopulationStandardDeviation,

    /// Sample standard deviation.
    SampleStandardDeviation,

    /// Standard error of the arithmetic mean.
    StandardError,

    /// Weighted standard error using effective sample size.
    WeightedStandardError,

    /// Geometric mean.
    GeometricMean,

    /// Weighted geometric mean.
    WeightedGeometricMean,

    /// Harmonic mean.
    HarmonicMean,

    /// Weighted harmonic mean.
    WeightedHarmonicMean,
}

impl AggregationMethod {
    /// Stable machine-readable identifier.
    pub const fn id(self) -> &'static str {
        match self {
            Self::Mean => "mean",
            Self::WeightedMean => "weighted_mean",
            Self::Sum => "sum",
            Self::WeightedSum => "weighted_sum",
            Self::Minimum => "minimum",
            Self::Maximum => "maximum",
            Self::Median => "median",
            Self::WeightedMedian => "weighted_median",
            Self::PopulationVariance => "population_variance",
            Self::SampleVariance => "sample_variance",
            Self::WeightedPopulationVariance => "weighted_population_variance",
            Self::WeightedSampleVariance => "weighted_sample_variance",
            Self::PopulationStandardDeviation => "population_standard_deviation",
            Self::SampleStandardDeviation => "sample_standard_deviation",
            Self::StandardError => "standard_error",
            Self::WeightedStandardError => "weighted_standard_error",
            Self::GeometricMean => "geometric_mean",
            Self::WeightedGeometricMean => "weighted_geometric_mean",
            Self::HarmonicMean => "harmonic_mean",
            Self::WeightedHarmonicMean => "weighted_harmonic_mean",
        }
    }

    /// Returns whether this operation requires sorting.
    pub const fn requires_sorting(self) -> bool {
        matches!(
            self,
            Self::Median | Self::WeightedMedian
        )
    }

    /// Returns whether this operation requires positive observations.
    pub const fn requires_positive_values(self) -> bool {
        matches!(
            self,
            Self::GeometricMean
                | Self::WeightedGeometricMean
                | Self::HarmonicMean
                | Self::WeightedHarmonicMean
        )
    }

    /// Returns whether this operation uses weights.
    pub const fn is_weighted(self) -> bool {
        matches!(
            self,
            Self::WeightedMean
                | Self::WeightedSum
                | Self::WeightedMedian
                | Self::WeightedPopulationVariance
                | Self::WeightedSampleVariance
                | Self::WeightedStandardError
                | Self::WeightedGeometricMean
                | Self::WeightedHarmonicMean
        )
    }
}

/// Configuration for aggregation.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct AggregationConfig {
    /// Selected aggregation operation.
    pub method: AggregationMethod,

    /// Resource limits.
    pub limits: AggregationLimits,
}

impl AggregationConfig {
    /// Creates a configuration using production limits.
    pub fn new(method: AggregationMethod) -> Self {
        Self {
            method,
            limits: AggregationLimits::default(),
        }
    }

    /// Creates a configuration with explicit limits.
    pub fn with_limits(
        method: AggregationMethod,
        limits: AggregationLimits,
    ) -> AggregationResult<Self> {
        limits.validate()?;

        Ok(Self {
            method,
            limits,
        })
    }

    /// Validates the configuration.
    pub fn validate(&self) -> AggregationResult<()> {
        self.limits.validate()
    }
}

/// Complete result of an aggregation operation.
///
/// The result deliberately contains more than the final estimate. Benchmark
/// reports need enough information to audit how an aggregate was obtained.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AggregationSummary {
    /// Stable aggregation algorithm identifier.
    pub algorithm: String,

    /// Aggregation operation.
    pub method: AggregationMethod,

    /// Final estimate.
    pub estimate: f64,

    /// Number of supplied observations.
    pub observations: usize,

    /// Number of observations with positive effective weight.
    pub positive_weight_observations: usize,

    /// Sum of weights.
    pub total_weight: f64,

    /// Sum of squared weights.
    pub sum_squared_weights: f64,

    /// Effective sample size:
    ///
    /// `W² / Σw²`
    pub effective_sample_size: f64,

    /// Sum of values.
    pub sum: f64,

    /// Minimum value.
    pub minimum: f64,

    /// Maximum value.
    pub maximum: f64,

    /// Population variance when it is defined by the supplied observations.
    ///
    /// For weighted observations this is the weighted population variance.
    pub population_variance: f64,

    /// Population standard deviation.
    pub population_standard_deviation: f64,

    /// Sample variance, when at least two observations exist.
    ///
    /// For weighted data this is the unbiased frequency-weight variance when
    /// the denominator is positive.
    pub sample_variance: Option<f64>,

    /// Sample standard deviation.
    pub sample_standard_deviation: Option<f64>,

    /// Standard error of the mean when it can be calculated.
    pub standard_error: Option<f64>,
}

impl AggregationSummary {
    /// Returns the confidence-independent standard error if available.
    pub fn standard_error(&self) -> Option<f64> {
        self.standard_error
    }

    /// Returns the interval-independent width of the observed range.
    pub fn range(&self) -> f64 {
        self.maximum - self.minimum
    }

    /// Returns whether all supplied observations had unit weight.
    pub fn is_unweighted(&self) -> bool {
        self.total_weight == self.observations as f64
            && self.sum_squared_weights == self.observations as f64
    }
}

/// Aggregate a slice of observations using production limits.
pub fn aggregate(
    observations: &[Observation],
    method: AggregationMethod,
) -> AggregationResult<AggregationSummary> {
    aggregate_with_limits(
        observations,
        AggregationConfig::new(method).limits,
    )
}

/// Aggregate a slice of observations with explicit production limits.
pub fn aggregate_with_limits(
    observations: &[Observation],
    limits: AggregationLimits,
) -> AggregationResult<AggregationSummary> {
    let config = AggregationConfig::with_limits(
        AggregationMethod::Mean,
        limits,
    )?;

    aggregate_with_config(observations, config)
}

/// Aggregate observations using an explicit configuration.
pub fn aggregate_with_config(
    observations: &[Observation],
    config: AggregationConfig,
) -> AggregationResult<AggregationSummary> {
    config.validate()?;
    validate_observations(observations, &config.limits)?;

    let statistics = calculate_common_statistics(observations)?;

    let estimate = match config.method {
        AggregationMethod::Mean => arithmetic_mean(&statistics),

        AggregationMethod::WeightedMean => weighted_mean(&statistics)?,

        AggregationMethod::Sum => statistics.sum,

        AggregationMethod::WeightedSum => {
            weighted_sum(&statistics)?
        }

        AggregationMethod::Minimum => statistics.minimum,

        AggregationMethod::Maximum => statistics.maximum,

        AggregationMethod::Median => {
            median(observations, &config.limits)?
        }

        AggregationMethod::WeightedMedian => {
            weighted_median(observations, &config.limits)?
        }

        AggregationMethod::PopulationVariance => {
            statistics.population_variance
        }

        AggregationMethod::SampleVariance => {
            statistics
                .sample_variance
                .ok_or(
                    AggregationError::InsufficientVarianceObservations {
                        observations: statistics.count,
                    },
                )?
        }

        AggregationMethod::WeightedPopulationVariance => {
            statistics.weighted_population_variance
        }

        AggregationMethod::WeightedSampleVariance => {
            statistics
                .weighted_sample_variance
                .ok_or(
                    AggregationError::InsufficientWeightedDegreesOfFreedom,
                )?
        }

        AggregationMethod::PopulationStandardDeviation => {
            statistics.population_standard_deviation
        }

        AggregationMethod::SampleStandardDeviation => {
            statistics
                .sample_standard_deviation
                .ok_or(
                    AggregationError::InsufficientVarianceObservations {
                        observations: statistics.count,
                    },
                )?
        }

        AggregationMethod::StandardError => {
            statistics
                .standard_error
                .ok_or(
                    AggregationError::InsufficientVarianceObservations {
                        observations: statistics.count,
                    },
                )?
        }

        AggregationMethod::WeightedStandardError => {
            weighted_standard_error(&statistics)?
        }

        AggregationMethod::GeometricMean => {
            geometric_mean(observations)?
        }

        AggregationMethod::WeightedGeometricMean => {
            weighted_geometric_mean(observations)?
        }

        AggregationMethod::HarmonicMean => {
            harmonic_mean(observations)?
        }

        AggregationMethod::WeightedHarmonicMean => {
            weighted_harmonic_mean(observations)?
        }
    };

    ensure_finite(estimate, config.method.id())?;

    Ok(AggregationSummary {
        algorithm: AGGREGATION_ALGORITHM_ID.to_string(),
        method: config.method,
        estimate,
        observations: statistics.count,
        positive_weight_observations: statistics.positive_weight_count,
        total_weight: statistics.total_weight,
        sum_squared_weights: statistics.sum_squared_weights,
        effective_sample_size: statistics.effective_sample_size,
        sum: statistics.sum,
        minimum: statistics.minimum,
        maximum: statistics.maximum,
        population_variance: statistics.population_variance,
        population_standard_deviation: statistics.population_standard_deviation,
        sample_variance: statistics.sample_variance,
        sample_standard_deviation: statistics.sample_standard_deviation,
        standard_error: statistics.standard_error,
    })
}

/// Calculate an arithmetic mean.
///
/// This uses incremental mean updates instead of summing all values directly,
/// reducing loss of precision for large collections with values of different
/// magnitudes.
pub fn mean(observations: &[Observation]) -> AggregationResult<f64> {
    let statistics = calculate_common_statistics_with_default_limits(observations)?;
    Ok(arithmetic_mean(&statistics))
}

/// Calculate a weighted arithmetic mean.
pub fn weighted_mean(
    observations: &[Observation],
) -> AggregationResult<f64> {
    let statistics = calculate_common_statistics_with_default_limits(observations)?;
    weighted_mean(&statistics)
}

/// Calculate a sum.
pub fn sum(observations: &[Observation]) -> AggregationResult<f64> {
    let statistics = calculate_common_statistics_with_default_limits(observations)?;
    ensure_finite(statistics.sum, "sum")?;
    Ok(statistics.sum)
}

/// Calculate a weighted sum.
pub fn weighted_sum(
    observations: &[Observation],
) -> AggregationResult<f64> {
    let statistics = calculate_common_statistics_with_default_limits(observations)?;
    weighted_sum(&statistics)
}

/// Calculate the population variance.
pub fn population_variance(
    observations: &[Observation],
) -> AggregationResult<f64> {
    let statistics = calculate_common_statistics_with_default_limits(observations)?;
    Ok(statistics.population_variance)
}

/// Calculate the sample variance.
///
/// Returns an error when fewer than two observations are supplied.
pub fn sample_variance(
    observations: &[Observation],
) -> AggregationResult<f64> {
    let statistics = calculate_common_statistics_with_default_limits(observations)?;

    statistics
        .sample_variance
        .ok_or(AggregationError::InsufficientVarianceObservations {
            observations: statistics.count,
        })
}

/// Calculate the population standard deviation.
pub fn population_standard_deviation(
    observations: &[Observation],
) -> AggregationResult<f64> {
    let statistics = calculate_common_statistics_with_default_limits(observations)?;
    Ok(statistics.population_standard_deviation)
}

/// Calculate the sample standard deviation.
pub fn sample_standard_deviation(
    observations: &[Observation],
) -> AggregationResult<f64> {
    let statistics = calculate_common_statistics_with_default_limits(observations)?;

    statistics
        .sample_standard_deviation
        .ok_or(AggregationError::InsufficientVarianceObservations {
            observations: statistics.count,
        })
}

/// Calculate the standard error of the arithmetic mean.
pub fn standard_error(
    observations: &[Observation],
) -> AggregationResult<f64> {
    let statistics = calculate_common_statistics_with_default_limits(observations)?;

    statistics
        .standard_error
        .ok_or(AggregationError::InsufficientVarianceObservations {
            observations: statistics.count,
        })
}

/// Calculate a median.
pub fn median(observations: &[Observation]) -> AggregationResult<f64> {
    median(
        observations,
        &AggregationLimits::default(),
    )
}

/// Calculate a weighted median.
pub fn weighted_median(
    observations: &[Observation],
) -> AggregationResult<f64> {
    weighted_median(
        observations,
        &AggregationLimits::default(),
    )
}

/// Calculate a quantile using linear interpolation.
///
/// The quantile is defined over the sorted empirical observations using the
/// standard `(n - 1) * q` position convention.
///
/// This function is unweighted.
pub fn quantile(
    observations: &[Observation],
    quantile: f64,
) -> AggregationResult<f64> {
    validate_quantile(quantile)?;

    let limits = AggregationLimits::default();

    validate_observations(observations, &limits)?;
    limits.check_sort_allocation(observations.len())?;

    let mut values: Vec<f64> =
        Vec::with_capacity(observations.len());

    for observation in observations {
        values.push(observation.value);
    }

    values.sort_by(total_order_f64);

    empirical_quantile_sorted(&values, quantile)
}

/// Calculate a weighted quantile.
///
/// Weighted quantiles are defined by cumulative normalized weight. When the
/// requested probability falls exactly on a cumulative boundary, the upper
/// observation is selected. This deterministic convention avoids ambiguous
/// interpolation when weights represent discrete frequencies.
pub fn weighted_quantile(
    observations: &[Observation],
    quantile: f64,
) -> AggregationResult<f64> {
    validate_quantile(quantile)?;

    let limits = AggregationLimits::default();

    validate_observations(observations, &limits)?;
    limits.check_sort_allocation(observations.len())?;

    let mut weighted: Vec<(f64, f64)> =
        Vec::with_capacity(observations.len());

    let mut total_weight = 0.0;

    for observation in observations {
        let weight = observation.effective_weight();

        if weight > 0.0 {
            total_weight = checked_add_f64(
                total_weight,
                weight,
                "weighted quantile total weight",
            )?;

            weighted.push((observation.value, weight));
        }
    }

    if weighted.is_empty() {
        return Err(AggregationError::NoPositiveWeightObservations);
    }

    ensure_finite(total_weight, "weighted quantile total weight")?;

    weighted.sort_by(|left, right| {
        total_order_f64(&left.0, &right.0)
    });

    let target = quantile * total_weight;
    let mut cumulative = 0.0;

    for (value, weight) in weighted {
        cumulative = checked_add_f64(
            cumulative,
            weight,
            "weighted quantile cumulative weight",
        )?;

        if cumulative >= target {
            return Ok(value);
        }
    }

    Err(AggregationError::NonFiniteResult {
        operation: "weighted quantile",
        value: f64::NAN,
    })
}

/// Calculate the median with explicit resource limits.
pub fn median_with_limits(
    observations: &[Observation],
    limits: AggregationLimits,
) -> AggregationResult<f64> {
    validate_observations(observations, &limits)?;
    limits.check_sort_allocation(observations.len())?;

    let mut values: Vec<f64> =
        Vec::with_capacity(observations.len());

    for observation in observations {
        values.push(observation.value);
    }

    values.sort_by(total_order_f64);

    empirical_quantile_sorted(&values, 0.5)
}

/// Calculate the weighted median with explicit resource limits.
pub fn weighted_median_with_limits(
    observations: &[Observation],
    limits: AggregationLimits,
) -> AggregationResult<f64> {
    validate_observations(observations, &limits)?;
    limits.check_sort_allocation(observations.len())?;

    weighted_quantile_with_limits(
        observations,
        0.5,
        limits,
    )
}

/// Calculate a weighted quantile with explicit limits.
pub fn weighted_quantile_with_limits(
    observations: &[Observation],
    quantile: f64,
    limits: AggregationLimits,
) -> AggregationResult<f64> {
    validate_quantile(quantile)?;
    validate_observations(observations, &limits)?;
    limits.check_sort_allocation(observations.len())?;

    let mut weighted: Vec<(f64, f64)> =
        Vec::with_capacity(observations.len());

    let mut total_weight = 0.0;

    for observation in observations {
        let weight = observation.effective_weight();

        if weight > 0.0 {
            total_weight = checked_add_f64(
                total_weight,
                weight,
                "weighted quantile total weight",
            )?;

            weighted.push((observation.value, weight));
        }
    }

    if weighted.is_empty() {
        return Err(AggregationError::NoPositiveWeightObservations);
    }

    weighted.sort_by(|left, right| {
        total_order_f64(&left.0, &right.0)
    });

    let target = quantile * total_weight;
    let mut cumulative = 0.0;

    for (value, weight) in weighted {
        cumulative = checked_add_f64(
            cumulative,
            weight,
            "weighted quantile cumulative weight",
        )?;

        if cumulative >= target {
            return Ok(value);
        }
    }

    Err(AggregationError::NonFiniteResult {
        operation: "weighted quantile",
        value: f64::NAN,
    })
}

/// Calculate a geometric mean.
pub fn geometric_mean(
    observations: &[Observation],
) -> AggregationResult<f64> {
    validate_observations(
        observations,
        &AggregationLimits::default(),
    )?;

    let mut mean_log = 0.0;
    let mut count = 0usize;

    for observation in observations {
        if observation.value <= 0.0 {
            return Err(AggregationError::UnsupportedOperation {
                message:
                    "geometric mean requires strictly positive observations"
                        .to_string(),
            });
        }

        count = count
            .checked_add(1)
            .ok_or(AggregationError::ArithmeticOverflow {
                operation: "geometric mean observation count",
            })?;

        let log_value = observation.value.ln();

        ensure_finite(
            log_value,
            "geometric mean logarithm",
        )?;

        let delta = log_value - mean_log;

        mean_log += delta / count as f64;

        ensure_finite(
            mean_log,
            "geometric mean log accumulator",
        )?;
    }

    let result = mean_log.exp();

    ensure_finite(result, "geometric mean")?;

    Ok(result)
}

/// Calculate a weighted geometric mean.
pub fn weighted_geometric_mean(
    observations: &[Observation],
) -> AggregationResult<f64> {
    validate_observations(
        observations,
        &AggregationLimits::default(),
    )?;

    let mut total_weight = 0.0;
    let mut weighted_log_sum = 0.0;

    for (index, observation) in observations.iter().enumerate() {
        let weight = observation.effective_weight();

        if weight == 0.0 {
            continue;
        }

        if observation.value <= 0.0 {
            return Err(AggregationError::UnsupportedOperation {
                message: format!(
                    "weighted geometric mean requires strictly positive \
                     observations; observation {index} is {}",
                    observation.value
                ),
            });
        }

        total_weight = checked_add_f64(
            total_weight,
            weight,
            "weighted geometric mean total weight",
        )?;

        let contribution =
            weight * observation.value.ln();

        ensure_finite(
            contribution,
            "weighted geometric mean contribution",
        )?;

        weighted_log_sum = checked_add_f64(
            weighted_log_sum,
            contribution,
            "weighted geometric mean accumulator",
        )?;
    }

    if total_weight <= 0.0 {
        return Err(AggregationError::ZeroTotalWeight);
    }

    let mean_log = weighted_log_sum / total_weight;

    ensure_finite(
        mean_log,
        "weighted geometric mean logarithm",
    )?;

    let result = mean_log.exp();

    ensure_finite(result, "weighted geometric mean")?;

    Ok(result)
}

/// Calculate a harmonic mean.
pub fn harmonic_mean(
    observations: &[Observation],
) -> AggregationResult<f64> {
    validate_observations(
        observations,
        &AggregationLimits::default(),
    )?;

    let count = observations.len();

    let mut reciprocal_sum = 0.0;

    for observation in observations {
        if observation.value <= 0.0 {
            return Err(AggregationError::UnsupportedOperation {
                message:
                    "harmonic mean requires strictly positive observations"
                        .to_string(),
            });
        }

        reciprocal_sum = checked_add_f64(
            reciprocal_sum,
            1.0 / observation.value,
            "harmonic mean reciprocal sum",
        )?;
    }

    if reciprocal_sum <= 0.0 {
        return Err(AggregationError::DivisionByZero {
            operation: "harmonic mean",
        });
    }

    let result = count as f64 / reciprocal_sum;

    ensure_finite(result, "harmonic mean")?;

    Ok(result)
}

/// Calculate a weighted harmonic mean.
pub fn weighted_harmonic_mean(
    observations: &[Observation],
) -> AggregationResult<f64> {
    validate_observations(
        observations,
        &AggregationLimits::default(),
    )?;

    let mut total_weight = 0.0;
    let mut weighted_reciprocal_sum = 0.0;

    for (index, observation) in observations.iter().enumerate() {
        let weight = observation.effective_weight();

        if weight == 0.0 {
            continue;
        }

        if observation.value <= 0.0 {
            return Err(AggregationError::UnsupportedOperation {
                message: format!(
                    "weighted harmonic mean requires strictly positive \
                     observations; observation {index} is {}",
                    observation.value
                ),
            });
        }

        total_weight = checked_add_f64(
            total_weight,
            weight,
            "weighted harmonic mean total weight",
        )?;

        let contribution =
            weight / observation.value;

        ensure_finite(
            contribution,
            "weighted harmonic mean contribution",
        )?;

        weighted_reciprocal_sum = checked_add_f64(
            weighted_reciprocal_sum,
            contribution,
            "weighted harmonic mean accumulator",
        )?;
    }

    if total_weight <= 0.0 {
        return Err(AggregationError::ZeroTotalWeight);
    }

    if weighted_reciprocal_sum <= 0.0 {
        return Err(AggregationError::DivisionByZero {
            operation: "weighted harmonic mean",
        });
    }

    let result =
        total_weight / weighted_reciprocal_sum;

    ensure_finite(
        result,
        "weighted harmonic mean",
    )?;

    Ok(result)
}

/// Internal common statistics accumulator.
#[derive(Debug, Clone, Copy)]
struct CommonStatistics {
    count: usize,
    positive_weight_count: usize,

    sum: f64,

    minimum: f64,
    maximum: f64,

    total_weight: f64,
    sum_squared_weights: f64,

    effective_sample_size: f64,

    weighted_mean: f64,

    population_variance: f64,
    sample_variance: Option<f64>,

    weighted_population_variance: f64,
    weighted_sample_variance: Option<f64>,

    population_standard_deviation: f64,
    sample_standard_deviation: Option<f64>,

    standard_error: Option<f64>,
}

fn calculate_common_statistics_with_default_limits(
    observations: &[Observation],
) -> AggregationResult<CommonStatistics> {
    calculate_common_statistics_with_limits(
        observations,
        AggregationLimits::default(),
    )
}

fn calculate_common_statistics(
    observations: &[Observation],
) -> AggregationResult<CommonStatistics> {
    calculate_common_statistics_with_limits(
        observations,
        AggregationLimits::default(),
    )
}

fn calculate_common_statistics_with_limits(
    observations: &[Observation],
    limits: AggregationLimits,
) -> AggregationResult<CommonStatistics> {
    validate_observations(observations, &limits)?;

    let count = observations.len();

    let mut minimum = observations[0].value;
    let mut maximum = observations[0].value;

    let mut sum = 0.0;

    // Ordinary Welford state.
    let mut ordinary_mean = 0.0;
    let mut ordinary_m2 = 0.0;

    // Weighted Welford state.
    let mut total_weight = 0.0;
    let mut sum_squared_weights = 0.0;
    let mut weighted_mean_value = 0.0;
    let mut weighted_m2 = 0.0;

    let mut positive_weight_count = 0usize;

    for (index, observation) in observations.iter().enumerate() {
        let value = observation.value;
        let weight = observation.effective_weight();

        if value < minimum {
            minimum = value;
        }

        if value > maximum {
            maximum = value;
        }

        sum = checked_add_f64(
            sum,
            value,
            "ordinary sum",
        )?;

        // ------------------------------------------------------------------
        // Ordinary Welford update.
        // ------------------------------------------------------------------

        let ordinary_count = index
            .checked_add(1)
            .ok_or(AggregationError::ArithmeticOverflow {
                operation: "ordinary observation count",
            })?;

        let delta = value - ordinary_mean;

        ordinary_mean +=
            delta / ordinary_count as f64;

        let delta_after = value - ordinary_mean;

        let ordinary_increment =
            delta * delta_after;

        ordinary_m2 = checked_add_f64(
            ordinary_m2,
            ordinary_increment,
            "ordinary Welford M2",
        )?;

        // ------------------------------------------------------------------
        // Weighted Welford update.
        // ------------------------------------------------------------------

        if weight > 0.0 {
            positive_weight_count =
                positive_weight_count
                    .checked_add(1)
                    .ok_or(
                        AggregationError::ArithmeticOverflow {
                            operation:
                                "positive-weight observation count",
                        },
                    )?;

            let previous_weight = total_weight;

            total_weight = checked_add_f64(
                total_weight,
                weight,
                "total weight",
            )?;

            sum_squared_weights = checked_add_f64(
                sum_squared_weights,
                weight * weight,
                "sum squared weights",
            )?;

            ensure_finite(
                total_weight,
                "total weight",
            )?;

            ensure_finite(
                sum_squared_weights,
                "sum squared weights",
            )?;

            if previous_weight == 0.0 {
                weighted_mean_value = value;
            } else {
                let delta =
                    value - weighted_mean_value;

                let ratio =
                    weight / total_weight;

                weighted_mean_value +=
                    ratio * delta;

                let delta_after =
                    value - weighted_mean_value;

                let increment =
                    weight * delta * delta_after;

                weighted_m2 = checked_add_f64(
                    weighted_m2,
                    increment,
                    "weighted Welford M2",
                )?;
            }
        }
    }

    ensure_finite(sum, "ordinary sum")?;
    ensure_finite(
        ordinary_mean,
        "ordinary mean",
    )?;
    ensure_finite(
        ordinary_m2,
        "ordinary Welford M2",
    )?;

    // Population variance.
    let population_variance =
        ordinary_m2 / count as f64;

    ensure_finite(
        population_variance,
        "population variance",
    )?;

    // Sample variance.
    let sample_variance = if count >= 2 {
        let value =
            ordinary_m2 / (count - 1) as f64;

        ensure_finite(
            value,
            "sample variance",
        )?;

        Some(value)
    } else {
        None
    };

    // Standard deviations.
    let population_standard_deviation =
        nonnegative_sqrt(
            population_variance,
            "population standard deviation",
        )?;

    let sample_standard_deviation =
        match sample_variance {
            Some(value) => Some(
                nonnegative_sqrt(
                    value,
                    "sample standard deviation",
                )?,
            ),

            None => None,
        };

    // Standard error.
    let standard_error =
        match sample_standard_deviation {
            Some(stddev) => {
                let value =
                    stddev / (count as f64).sqrt();

                ensure_finite(
                    value,
                    "standard error",
                )?;

                Some(value)
            }

            None => None,
        };

    // Weighted statistics.
    let weighted_mean =
        if total_weight > 0.0 {
            weighted_mean_value
        } else {
            0.0
        };

    let weighted_population_variance =
        if total_weight > 0.0 {
            weighted_m2 / total_weight
        } else {
            0.0
        };

    ensure_finite(
        weighted_population_variance,
        "weighted population variance",
    )?;

    let weighted_sample_variance =
        if positive_weight_count >= 2 {
            let denominator =
                total_weight * total_weight
                    - sum_squared_weights;

            if denominator > 0.0 {
                let numerator =
                    total_weight * weighted_m2;

                let value =
                    numerator / denominator;

                ensure_finite(
                    value,
                    "weighted sample variance",
                )?;

                Some(value)
            } else {
                None
            }
        } else {
            None
        };

    let effective_sample_size =
        if sum_squared_weights > 0.0 {
            let numerator =
                total_weight * total_weight;

            let value =
                numerator / sum_squared_weights;

            ensure_finite(
                value,
                "effective sample size",
            )?;

            value
        } else {
            0.0
        };

    Ok(CommonStatistics {
        count,
        positive_weight_count,
        sum,
        minimum,
        maximum,
        total_weight,
        sum_squared_weights,
        effective_sample_size,
        weighted_mean,
        population_variance,
        sample_variance,
        weighted_population_variance,
        weighted_sample_variance,
        population_standard_deviation,
        sample_standard_deviation,
        standard_error,
    })
}

fn arithmetic_mean(
    statistics: &CommonStatistics,
) -> f64 {
    statistics
        .sum
        / statistics.count as f64
}

fn weighted_mean(
    statistics: &CommonStatistics,
) -> AggregationResult<f64> {
    if statistics.total_weight <= 0.0 {
        return Err(AggregationError::ZeroTotalWeight);
    }

    ensure_finite(
        statistics.weighted_mean,
        "weighted mean",
    )?;

    Ok(statistics.weighted_mean)
}

fn weighted_sum(
    statistics: &CommonStatistics,
) -> AggregationResult<f64> {
    if statistics.total_weight <= 0.0 {
        return Err(AggregationError::ZeroTotalWeight);
    }

    let weighted =
        statistics.weighted_mean
            * statistics.total_weight;

    ensure_finite(
        weighted,
        "weighted sum",
    )?;

    Ok(weighted)
}

fn weighted_standard_error(
    statistics: &CommonStatistics,
) -> AggregationResult<f64> {
    if statistics.total_weight <= 0.0 {
        return Err(AggregationError::ZeroTotalWeight);
    }

    if statistics.effective_sample_size <= 1.0 {
        return Err(
            AggregationError::InsufficientWeightedDegreesOfFreedom,
        );
    }

    let variance =
        statistics.weighted_sample_variance.ok_or(
            AggregationError::InsufficientWeightedDegreesOfFreedom,
        )?;

    let value =
        (variance / statistics.effective_sample_size).sqrt();

    ensure_finite(
        value,
        "weighted standard error",
    )?;

    Ok(value)
}

fn validate_observations(
    observations: &[Observation],
    limits: &AggregationLimits,
) -> AggregationResult<()> {
    limits.check_observations(observations.len())?;

    for (index, observation) in observations.iter().enumerate() {
        validate_finite(
            observation.value,
            index,
        )?;

        if let Some(weight) = observation.weight {
            validate_weight(weight, index)?;
        }
    }

    Ok(())
}

fn validate_finite(
    value: f64,
    index: usize,
) -> AggregationResult<()> {
    if !value.is_finite() {
        return Err(
            AggregationError::NonFiniteObservation {
                index,
                value,
            },
        );
    }

    Ok(())
}

fn validate_weight(
    weight: f64,
    index: usize,
) -> AggregationResult<()> {
    if !weight.is_finite() {
        return Err(
            AggregationError::NonFiniteWeight {
                index,
                weight,
            },
        );
    }

    if weight < 0.0 {
        return Err(
            AggregationError::NegativeWeight {
                index,
                weight,
            },
        );
    }

    Ok(())
}

fn validate_quantile(
    quantile: f64,
) -> AggregationResult<()> {
    if !quantile.is_finite()
        || !(0.0..=1.0).contains(&quantile)
    {
        return Err(AggregationError::InvalidQuantile {
            quantile,
        });
    }

    Ok(())
}

fn ensure_finite(
    value: f64,
    operation: &'static str,
) -> AggregationResult<()> {
    if !value.is_finite() {
        return Err(
            AggregationError::NonFiniteResult {
                operation,
                value,
            },
        );
    }

    Ok(())
}

fn checked_add_f64(
    left: f64,
    right: f64,
    operation: &'static str,
) -> AggregationResult<f64> {
    let result = left + right;

    ensure_finite(result, operation)?;

    Ok(result)
}

fn nonnegative_sqrt(
    value: f64,
    operation: &'static str,
) -> AggregationResult<f64> {
    if value < 0.0 {
        return Err(AggregationError::NonFiniteResult {
            operation,
            value,
        });
    }

    let result = value.sqrt();

    ensure_finite(result, operation)?;

    Ok(result)
}

fn median(
    observations: &[Observation],
    limits: &AggregationLimits,
) -> AggregationResult<f64> {
    median_with_limits(observations, *limits)
}

fn weighted_median(
    observations: &[Observation],
    limits: &AggregationLimits,
) -> AggregationResult<f64> {
    weighted_median_with_limits(observations, *limits)
}

fn weighted_quantile_with_limits(
    observations: &[Observation],
    quantile: f64,
    limits: AggregationLimits,
) -> AggregationResult<f64> {
    validate_quantile(quantile)?;
    validate_observations(observations, limits)?;
    limits.check_sort_allocation(observations.len())?;

    let mut weighted: Vec<(f64, f64)> =
        Vec::with_capacity(observations.len());

    let mut total_weight = 0.0;

    for observation in observations {
        let weight = observation.effective_weight();

        if weight > 0.0 {
            total_weight = checked_add_f64(
                total_weight,
                weight,
                "weighted quantile total weight",
            )?;

            weighted.push((observation.value, weight));
        }
    }

    if weighted.is_empty() {
        return Err(
            AggregationError::NoPositiveWeightObservations,
        );
    }

    weighted.sort_by(|left, right| {
        total_order_f64(&left.0, &right.0)
    });

    if quantile == 0.0 {
        return Ok(weighted[0].0);
    }

    let target =
        quantile * total_weight;

    let mut cumulative = 0.0;

    for (value, weight) in weighted {
        cumulative = checked_add_f64(
            cumulative,
            weight,
            "weighted quantile cumulative weight",
        )?;

        if cumulative >= target {
            return Ok(value);
        }
    }

    Err(AggregationError::NonFiniteResult {
        operation: "weighted quantile",
        value: f64::NAN,
    })
}

fn empirical_quantile_sorted(
    values: &[f64],
    quantile: f64,
) -> AggregationResult<f64> {
    if values.is_empty() {
        return Err(AggregationError::EmptyObservations);
    }

    validate_quantile(quantile)?;

    if values.len() == 1 {
        return Ok(values[0]);
    }

    let position =
        quantile * (values.len() - 1) as f64;

    let lower_index =
        position.floor() as usize;

    let upper_index =
        position.ceil() as usize;

    let lower =
        values[lower_index];

    let upper =
        values[upper_index];

    if lower_index == upper_index {
        return Ok(lower);
    }

    let fraction =
        position - lower_index as f64;

    let result =
        lower + fraction * (upper - lower);

    ensure_finite(
        result,
        "empirical quantile",
    )?;

    Ok(result)
}

/// Total ordering for validated finite floating-point values.
///
/// Since NaN is rejected before sorting, this ordering is deterministic and
/// free of partial-order ambiguity.
fn total_order_f64(
    left: &f64,
    right: &f64,
) -> Ordering {
    left.partial_cmp(right)
        .unwrap_or(Ordering::Equal)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn observations(
        values: &[f64],
    ) -> Vec<Observation> {
        values
            .iter()
            .map(|value| {
                Observation::new(*value)
                    .expect("test observation must be valid")
            })
            .collect()
    }

    fn weighted_observations(
        values: &[(f64, f64)],
    ) -> Vec<Observation> {
        values
            .iter()
            .map(|(value, weight)| {
                Observation::weighted(
                    *value,
                    *weight,
                )
                .expect("test observation must be valid")
            })
            .collect()
    }

    #[test]
    fn mean_is_correct() {
        let values =
            observations(&[1.0, 2.0, 3.0, 4.0]);

        let result =
            mean(&values).expect("mean must succeed");

        assert!((result - 2.5).abs() < 1e-12);
    }

    #[test]
    fn weighted_mean_is_correct() {
        let values =
            weighted_observations(&[
                (1.0, 1.0),
                (3.0, 3.0),
            ]);

        let result =
            weighted_mean(&values)
                .expect("weighted mean must succeed");

        assert!((result - 2.5).abs() < 1e-12);
    }

    #[test]
    fn sum_is_correct() {
        let values =
            observations(&[1.0, 2.0, 3.0]);

        let result =
            sum(&values).expect("sum must succeed");

        assert_eq!(result, 6.0);
    }

    #[test]
    fn weighted_sum_is_correct() {
        let values =
            weighted_observations(&[
                (2.0, 2.0),
                (4.0, 3.0),
            ]);

        let result =
            weighted_sum(&values)
                .expect("weighted sum must succeed");

        assert!((result - 16.0).abs() < 1e-12);
    }

    #[test]
    fn minimum_and_maximum_are_correct() {
        let values =
            observations(&[3.0, -1.0, 8.0, 2.0]);

        let minimum =
            aggregate(
                &values,
                AggregationMethod::Minimum,
            )
            .expect("minimum must succeed");

        let maximum =
            aggregate(
                &values,
                AggregationMethod::Maximum,
            )
            .expect("maximum must succeed");

        assert_eq!(minimum.estimate, -1.0);
        assert_eq!(maximum.estimate, 8.0);
    }

    #[test]
    fn median_is_correct_for_odd_sample() {
        let values =
            observations(&[5.0, 1.0, 3.0]);

        let result =
            median(&values)
                .expect("median must succeed");

        assert_eq!(result, 3.0);
    }

    #[test]
    fn median_is_correct_for_even_sample() {
        let values =
            observations(&[4.0, 1.0, 3.0, 2.0]);

        let result =
            median(&values)
                .expect("median must succeed");

        assert_eq!(result, 2.5);
    }

    #[test]
    fn quantile_interpolates() {
        let values =
            observations(&[
                1.0, 2.0, 3.0, 4.0,
            ]);

        let result =
            quantile(&values, 0.25)
                .expect("quantile must succeed");

        assert!((result - 1.75).abs() < 1e-12);
    }

    #[test]
    fn weighted_median_is_deterministic() {
        let values =
            weighted_observations(&[
                (1.0, 1.0),
                (2.0, 1.0),
                (10.0, 8.0),
            ]);

        let result =
            weighted_median(&values)
                .expect("weighted median must succeed");

        assert_eq!(result, 10.0);
    }

    #[test]
    fn population_variance_is_correct() {
        let values =
            observations(&[
                1.0, 2.0, 3.0,
            ]);

        let result =
            population_variance(&values)
                .expect("population variance must succeed");

        assert!((result - 2.0 / 3.0).abs() < 1e-12);
    }

    #[test]
    fn sample_variance_is_correct() {
        let values =
            observations(&[
                1.0, 2.0, 3.0,
            ]);

        let result =
            sample_variance(&values)
                .expect("sample variance must succeed");

        assert_eq!(result, 1.0);
    }

    #[test]
    fn population_standard_deviation_is_correct() {
        let values =
            observations(&[
                1.0, 2.0, 3.0,
            ]);

        let result =
            population_standard_deviation(&values)
                .expect("standard deviation must succeed");

        assert!(
            (result - (2.0_f64 / 3.0).sqrt()).abs()
                < 1e-12
        );
    }

    #[test]
    fn standard_error_is_correct() {
        let values =
            observations(&[
                1.0, 2.0, 3.0,
            ]);

        let result =
            standard_error(&values)
                .expect("standard error must succeed");

        assert!((result - 1.0 / 3.0_f64.sqrt()).abs() < 1e-12);
    }

    #[test]
    fn weighted_variance_handles_frequency_weights() {
        let values =
            weighted_observations(&[
                (1.0, 1.0),
                (2.0, 2.0),
                (3.0, 1.0),
            ]);

        let result =
            aggregate(
                &values,
                AggregationMethod::WeightedSampleVariance,
            )
            .expect("weighted variance must succeed");

        assert!((result.estimate - 2.0 / 3.0).abs() < 1e-12);
    }

    #[test]
    fn effective_sample_size_is_reported() {
        let values =
            weighted_observations(&[
                (1.0, 1.0),
                (2.0, 3.0),
            ]);

        let result =
            aggregate(
                &values,
                AggregationMethod::WeightedMean,
            )
            .expect("aggregation must succeed");

        let expected =
            16.0 / 10.0;

        assert!(
            (result.effective_sample_size - expected).abs()
                < 1e-12
        );
    }

    #[test]
    fn geometric_mean_is_correct() {
        let values =
            observations(&[
                1.0, 4.0, 16.0,
            ]);

        let result =
            geometric_mean(&values)
                .expect("geometric mean must succeed");

        assert!(
            (result - 4.0).abs() < 1e-12
        );
    }

    #[test]
    fn harmonic_mean_is_correct() {
        let values =
            observations(&[
                1.0, 2.0,
            ]);

        let result =
            harmonic_mean(&values)
                .expect("harmonic mean must succeed");

        assert!(
            (result - 4.0 / 3.0).abs() < 1e-12
        );
    }

    #[test]
    fn empty_observations_are_rejected() {
        let values: Vec<Observation> =
            Vec::new();

        let result =
            aggregate(
                &values,
                AggregationMethod::Mean,
            );

        assert_eq!(
            result,
            Err(AggregationError::EmptyObservations)
        );
    }

    #[test]
    fn nan_is_rejected() {
        let result =
            Observation::new(f64::NAN);

        assert!(matches!(
            result,
            Err(
                AggregationError::NonFiniteObservation {
                    ..
                }
            )
        ));
    }

    #[test]
    fn infinity_is_rejected() {
        let result =
            Observation::new(f64::INFINITY);

        assert!(matches!(
            result,
            Err(
                AggregationError::NonFiniteObservation {
                    ..
                }
            )
        ));
    }

    #[test]
    fn negative_weight_is_rejected() {
        let result =
            Observation::weighted(
                1.0,
                -1.0,
            );

        assert!(matches!(
            result,
            Err(
                AggregationError::NegativeWeight {
                    ..
                }
            )
        ));
    }

    #[test]
    fn nan_weight_is_rejected() {
        let result =
            Observation::weighted(
                1.0,
                f64::NAN,
            );

        assert!(matches!(
            result,
            Err(
                AggregationError::NonFiniteWeight {
                    ..
                }
            )
        ));
    }

    #[test]
    fn zero_total_weight_is_rejected() {
        let values =
            weighted_observations(&[
                (1.0, 0.0),
                (2.0, 0.0),
            ]);

        let result =
            aggregate(
                &values,
                AggregationMethod::WeightedMean,
            );

        assert_eq!(
            result,
            Err(AggregationError::ZeroTotalWeight)
        );
    }

    #[test]
    fn weighted_zero_observation_does_not_contribute() {
        let values =
            weighted_observations(&[
                (100.0, 0.0),
                (10.0, 1.0),
                (20.0, 1.0),
            ]);

        let result =
            weighted_mean(&values)
                .expect("weighted mean must succeed");

        assert_eq!(result, 15.0);
    }

    #[test]
    fn invalid_quantile_is_rejected() {
        let values =
            observations(&[
                1.0, 2.0, 3.0,
            ]);

        let result =
            quantile(&values, 1.1);

        assert!(matches!(
            result,
            Err(
                AggregationError::InvalidQuantile {
                    ..
                }
            )
        ));
    }

    #[test]
    fn resource_limit_is_enforced() {
        let values =
            observations(&[
                1.0, 2.0, 3.0,
            ]);

        let limits =
            AggregationLimits::new(
                2,
                DEFAULT_MAX_SORT_BYTES,
            );

        let result =
            aggregate_with_limits(
                &values,
                limits,
            );

        assert!(matches!(
            result,
            Err(
                AggregationError::ObservationLimitExceeded {
                    ..
                }
            )
        ));
    }

    #[test]
    fn sort_memory_limit_is_enforced() {
        let values =
            observations(&[
                1.0, 2.0, 3.0,
            ]);

        let limits =
            AggregationLimits::new(
                10,
                1,
            );

        let result =
            aggregate_with_config(
                &values,
                AggregationConfig::with_limits(
                    AggregationMethod::Median,
                    limits,
                )
                .expect("configuration must be valid"),
            );

        assert!(matches!(
            result,
            Err(
                AggregationError::SortAllocationLimitExceeded {
                    ..
                }
            )
        ));
    }

    #[test]
    fn aggregation_result_contains_audit_metadata() {
        let values =
            observations(&[
                1.0, 2.0, 3.0,
            ]);

        let result =
            aggregate(
                &values,
                AggregationMethod::Mean,
            )
            .expect("aggregation must succeed");

        assert_eq!(
            result.algorithm,
            AGGREGATION_ALGORITHM_ID
        );

        assert_eq!(
            result.observations,
            3
        );

        assert_eq!(
            result.positive_weight_observations,
            3
        );

        assert_eq!(
            result.total_weight,
            3.0
        );

        assert_eq!(
            result.effective_sample_size,
            3.0
        );
    }

    #[test]
    fn deterministic_results_are_reproducible() {
        let values =
            observations(&[
                9.0,
                1.0,
                4.0,
                7.0,
                3.0,
            ]);

        let first =
            aggregate(
                &values,
                AggregationMethod::Median,
            )
            .expect("first aggregation must succeed");

        let second =
            aggregate(
                &values,
                AggregationMethod::Median,
            )
            .expect("second aggregation must succeed");

        assert_eq!(
            first,
            second
        );
    }

    #[test]
    fn negative_values_are_valid_for_arithmetic_statistics() {
        let values =
            observations(&[
                -4.0,
                -2.0,
                2.0,
                4.0,
            ]);

        let result =
            mean(&values)
                .expect("mean must support negative values");

        assert_eq!(result, 0.0);
    }

    #[test]
    fn geometric_mean_rejects_zero() {
        let values =
            observations(&[
                0.0,
                2.0,
            ]);

        let result =
            geometric_mean(&values);

        assert!(matches!(
            result,
            Err(
                AggregationError::UnsupportedOperation {
                    ..
                }
            )
        ));
    }

    #[test]
    fn harmonic_mean_rejects_negative_values() {
        let values =
            observations(&[
                1.0,
                -2.0,
            ]);

        let result =
            harmonic_mean(&values);

        assert!(matches!(
            result,
            Err(
                AggregationError::UnsupportedOperation {
                    ..
                }
            )
        ));
    }

    #[test]
    fn common_summary_is_available_for_all_methods() {
        let values =
            observations(&[
                1.0,
                2.0,
                3.0,
                4.0,
            ]);

        for method in [
            AggregationMethod::Mean,
            AggregationMethod::WeightedMean,
            AggregationMethod::Sum,
            AggregationMethod::WeightedSum,
            AggregationMethod::Minimum,
            AggregationMethod::Maximum,
            AggregationMethod::Median,
            AggregationMethod::WeightedMedian,
            AggregationMethod::PopulationVariance,
            AggregationMethod::SampleVariance,
            AggregationMethod::WeightedPopulationVariance,
            AggregationMethod::WeightedSampleVariance,
            AggregationMethod::PopulationStandardDeviation,
            AggregationMethod::SampleStandardDeviation,
            AggregationMethod::StandardError,
            AggregationMethod::WeightedStandardError,
        ] {
            let result =
                aggregate(
                    &values,
                    method,
                )
                .expect(
                    "common aggregation method must succeed",
                );

            assert!(
                result.estimate.is_finite()
            );
        }
    }
}