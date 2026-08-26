//! Zamani Quantum Benchmarking — Stability and Drift Metrics
//!
//! Production stability analysis for repeated quantum benchmark observations.
//!
//! # Purpose
//!
//! This module measures temporal stability and drift of quantum-computing
//! benchmark quantities such as:
//!
//! - gate error rates;
//! - randomized-benchmarking error estimates;
//! - readout fidelity;
//! - readout error;
//! - state-preparation fidelity;
//! - T1/T2/T2* measurements;
//! - leakage;
//! - logical error rate;
//! - logical fidelity;
//! - application success probability;
//! - execution latency;
//! - throughput;
//! - calibration parameters;
//! - device parameters;
//! - repeated benchmark scores.
//!
//! The module is deliberately protocol-independent.
//!
//! It does NOT:
//!
//! - execute quantum circuits;
//! - generate circuits;
//! - communicate with hardware;
//! - assume a particular quantum technology;
//! - assume a particular backend vendor;
//! - silently remove observations;
//! - infer causality from drift;
//! - declare a hardware system "stable" without an explicit decision rule;
//! - perform spectral analysis requiring an FFT dependency;
//! - write files;
//! - print to stdout/stderr;
//! - maintain global mutable state.
//!
//! # Architectural position
//!
//! ```text
//! quantum benchmark execution
//!            │
//!            ▼
//! timestamped observations
//!            │
//!            ▼
//! metrics::stability
//!            │
//!      ┌─────┼─────────────┐
//!      ▼     ▼             ▼
//!   spread  drift      time-series
//!      │     │             │
//!      └─────┼─────────────┘
//!            ▼
//!      stability result
//!            │
//!     ┌──────┴──────────┐
//!     ▼                 ▼
//! core::metric      analysis::*
//!     │                 │
//!     ▼                 ▼
//! BenchmarkResult    diagnosis
//! ```
//!
//! # Scientific interpretation
//!
//! Stability and accuracy are different concepts.
//!
//! A system can repeatedly produce the same wrong value and therefore be
//! stable without being accurate. Conversely, a system can have a correct
//! average value while exhibiting substantial temporal instability.
//!
//! This module therefore reports separate quantities for:
//!
//! - central tendency;
//! - dispersion;
//! - absolute drift;
//! - relative drift where mathematically meaningful;
//! - linear trend;
//! - local changes;
//! - autocorrelation;
//! - Allan deviation for equally spaced observations;
//! - optional control limits.
//!
//! No single quantity is treated as a universal definition of stability.
//!
//! # Quantum benchmarking interpretation
//!
//! Quantum processor errors are time-dependent. Stability analysis therefore
//! needs to operate on time-resolved benchmark observations rather than only
//! on a single aggregate benchmark result.
//!
//! This module can consequently be used both:
//!
//! 1. directly on a repeated measurement series; and
//! 2. on time-resolved results produced by other benchmark protocols such as
//!    randomized benchmarking, gate-set characterization, Ramsey experiments,
//!    readout benchmarking, or logical-error experiments.
//!
//! # Allan deviation
//!
//! Allan deviation is included because it is a standard stability statistic
//! for time-series stability analysis. It is particularly appropriate when
//! observations represent a frequency-like or fractional-frequency-like
//! quantity sampled at regular intervals.
//!
//! It MUST NOT automatically be interpreted as a generic quantum-device
//! stability score. Callers must explicitly establish that the input quantity
//! has the required interpretation.
//!
//! For equally spaced observations y_i, the non-overlapping adjacent Allan
//! deviation implemented here is:
//!
//! ```text
//!                    1       N-1
//! sigma_A = sqrt( ---------  Σ (y[i+1] - y[i])² )
//!                   2(N-1)   i=0
//! ```
//!
//! The implementation also supports a specified sampling interval `tau` in
//! the result metadata.
//!
//! # Resource safety
//!
//! Benchmark input can originate from hardware, serialized reports, remote
//! execution services, or user-defined Zamani workloads. Therefore all
//! operations enforce explicit observation limits.
//!
//! No unbounded sorting or allocation is performed.
//!
//! # Numerical safety
//!
//! The module:
//!
//! - rejects NaN;
//! - rejects positive infinity;
//! - rejects negative infinity;
//! - checks arithmetic results for finiteness;
//! - checks integer overflow;
//! - rejects invalid timestamps;
//! - rejects non-monotonic timestamps;
//! - rejects zero or negative intervals where an interval is required;
//! - rejects undefined relative metrics rather than manufacturing a value.
//!
//! # Integration contract
//!
//! This file intentionally depends only on:
//!
//! - `serde`;
//! - the Rust standard library.
//!
//! It therefore can be completed before:
//!
//! - `core/result.rs`;
//! - `core/provenance.rs`;
//! - `statistics/confidence.rs`;
//! - `statistics/regression.rs`;
//! - `analysis/drift.rs`;
//! - individual benchmark protocols.
//!
//! Later modules should consume this file rather than reimplementing temporal
//! stability calculations.
//!
//! The canonical downstream direction is:
//!
//! ```text
//! metrics::stability
//!       │
//!       ├── core::metric
//!       ├── core::result
//!       ├── analysis::drift
//!       ├── analysis::diagnosis
//!       ├── protocols::drift
//!       ├── protocols::randomized_benchmarking
//!       ├── protocols::coherence
//!       ├── protocols::crosstalk
//!       └── qec::*
//! ```
//!
//! Those modules may depend on this module.
//!
//! This module must not depend on those higher-level modules.
//!
//! # Rust compatibility
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021
//! - No nightly features
//!
//! # Serialization
//!
//! Public result structures derive `Serialize` and `Deserialize` so that
//! stability analysis can become part of the canonical machine-readable
//! benchmark result format later.
//!
//! # Versioning
//!
//! The algorithm identifier is part of the scientific reproducibility
//! contract. If the mathematical implementation changes in a result-changing
//! way, the algorithm identifier MUST be versioned.

use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;

/// Stable identifier for this implementation.
///
/// Any result-changing mathematical modification requires a new identifier.
pub const STABILITY_ALGORITHM_ID: &str = "zamani.metrics.stability.v1";

/// Default maximum number of observations accepted by one stability analysis.
pub const DEFAULT_MAX_OBSERVATIONS: usize = 1_000_000;

/// Default number of standard deviations used by control-limit helpers.
pub const DEFAULT_CONTROL_LIMIT_SIGMAS: f64 = 3.0;

/// Smallest permitted number of observations for a trend estimate.
pub const MIN_TREND_OBSERVATIONS: usize = 2;

/// Smallest permitted number of observations for sample variance.
pub const MIN_VARIANCE_OBSERVATIONS: usize = 2;

/// Smallest permitted number of observations for lag-1 autocorrelation.
pub const MIN_AUTOCORRELATION_OBSERVATIONS: usize = 3;

/// Smallest permitted number of observations for Allan deviation.
pub const MIN_ALLAN_OBSERVATIONS: usize = 2;

/// Result type used by this module.
pub type StabilityResult<T> = Result<T, StabilityError>;

/// Errors raised by stability analysis.
#[derive(Debug, Clone, PartialEq)]
pub enum StabilityError {
    /// No observations were supplied.
    EmptyObservations,

    /// Too few observations exist for the requested operation.
    InsufficientObservations {
        /// Operation that requested more observations.
        operation: &'static str,

        /// Number actually supplied.
        observations: usize,

        /// Minimum required.
        minimum: usize,
    },

    /// The configured observation limit is invalid.
    InvalidObservationLimit {
        /// Supplied limit.
        maximum: usize,
    },

    /// The input exceeded the configured observation limit.
    ObservationLimitExceeded {
        /// Number supplied.
        observations: usize,

        /// Maximum accepted.
        maximum: usize,
    },

    /// A measurement value is NaN or infinite.
    NonFiniteValue {
        /// Zero-based observation index.
        index: usize,

        /// Invalid value.
        value: f64,
    },

    /// A timestamp is NaN or infinite.
    NonFiniteTimestamp {
        /// Zero-based observation index.
        index: usize,

        /// Invalid timestamp.
        timestamp: f64,
    },

    /// A timestamp is earlier than or equal to its predecessor.
    NonMonotonicTimestamp {
        /// Index of the later observation.
        index: usize,

        /// Previous timestamp.
        previous: f64,

        /// Current timestamp.
        current: f64,
    },

    /// A timestamp interval is zero.
    ZeroTimeInterval {
        /// Index of the later observation.
        index: usize,
    },

    /// A timestamp interval is negative.
    NegativeTimeInterval {
        /// Index of the later observation.
        index: usize,

        /// Interval in the supplied time unit.
        interval: f64,
    },

    /// Arithmetic generated a non-finite value.
    NonFiniteResult {
        /// Operation producing the invalid result.
        operation: &'static str,

        /// Invalid result.
        value: f64,
    },

    /// Relative drift was requested with a zero reference.
    UndefinedRelativeMetric {
        /// Operation requiring a non-zero reference.
        operation: &'static str,
    },

    /// A required variance calculation had insufficient degrees of freedom.
    InsufficientVarianceDegreesOfFreedom,

    /// A control-limit sigma value is invalid.
    InvalidSigma {
        /// Supplied sigma.
        sigma: f64,
    },

    /// A requested Allan deviation calculation requires regular sampling.
    UnequalSamplingIntervals {
        /// Interval before the mismatch.
        expected: f64,

        /// Interval encountered.
        actual: f64,

        /// Later observation index.
        index: usize,
    },

    /// The supplied Allan sampling interval is invalid.
    InvalidAllanInterval {
        /// Supplied interval.
        tau: f64,
    },

    /// A supplied baseline is invalid.
    InvalidBaseline {
        /// Supplied baseline.
        baseline: f64,
    },

    /// A threshold is invalid.
    InvalidThreshold {
        /// Supplied threshold.
        threshold: f64,
    },

    /// A probability-like value is outside [0, 1].
    InvalidProbability {
        /// Observation index.
        index: usize,

        /// Invalid value.
        value: f64,
    },

    /// The input contained a non-finite timestamp or value.
    InvalidObservation {
        /// Human-readable explanation.
        message: String,
    },
}

impl fmt::Display for StabilityError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyObservations => {
                write!(formatter, "stability analysis requires observations")
            }

            Self::InsufficientObservations {
                operation,
                observations,
                minimum,
            } => write!(
                formatter,
                "{operation} requires at least {minimum} observations, \
                 got {observations}"
            ),

            Self::InvalidObservationLimit { maximum } => write!(
                formatter,
                "stability observation limit must be greater than zero, \
                 got {maximum}"
            ),

            Self::ObservationLimitExceeded {
                observations,
                maximum,
            } => write!(
                formatter,
                "stability observation limit exceeded: \
                 observations={observations}, maximum={maximum}"
            ),

            Self::NonFiniteValue { index, value } => write!(
                formatter,
                "stability observation {index} contains a non-finite value: \
                 {value}"
            ),

            Self::NonFiniteTimestamp { index, timestamp } => write!(
                formatter,
                "stability observation {index} contains a non-finite \
                 timestamp: {timestamp}"
            ),

            Self::NonMonotonicTimestamp {
                index,
                previous,
                current,
            } => write!(
                formatter,
                "timestamp at index {index} is not strictly increasing: \
                 previous={previous}, current={current}"
            ),

            Self::ZeroTimeInterval { index } => write!(
                formatter,
                "timestamp interval before observation {index} is zero"
            ),

            Self::NegativeTimeInterval { index, interval } => write!(
                formatter,
                "timestamp interval before observation {index} is negative: \
                 {interval}"
            ),

            Self::NonFiniteResult { operation, value } => write!(
                formatter,
                "stability operation {operation} produced a non-finite \
                 result: {value}"
            ),

            Self::UndefinedRelativeMetric { operation } => write!(
                formatter,
                "stability operation {operation} has an undefined relative \
                 value because the reference is zero"
            ),

            Self::InsufficientVarianceDegreesOfFreedom => write!(
                formatter,
                "sample variance has insufficient degrees of freedom"
            ),

            Self::InvalidSigma { sigma } => write!(
                formatter,
                "control-limit sigma must be finite and greater than zero, \
                 got {sigma}"
            ),

            Self::UnequalSamplingIntervals {
                expected,
                actual,
                index,
            } => write!(
                formatter,
                "Allan deviation requires equal sampling intervals: \
                 expected={expected}, actual={actual}, index={index}"
            ),

            Self::InvalidAllanInterval { tau } => write!(
                formatter,
                "Allan deviation interval must be finite and greater than \
                 zero, got {tau}"
            ),

            Self::InvalidBaseline { baseline } => write!(
                formatter,
                "baseline must be finite, got {baseline}"
            ),

            Self::InvalidThreshold { threshold } => write!(
                formatter,
                "threshold must be finite, got {threshold}"
            ),

            Self::InvalidProbability { index, value } => write!(
                formatter,
                "probability observation {index} is outside [0, 1]: {value}"
            ),

            Self::InvalidObservation { message } => {
                write!(formatter, "invalid stability observation: {message}")
            }
        }
    }
}

impl Error for StabilityError {}

/// One timestamped benchmark observation.
///
/// `timestamp` is expressed in a caller-defined unit. For temporal analysis,
/// callers should use one consistent unit throughout the series.
///
/// Examples:
///
/// - seconds since experiment start;
/// - Unix seconds;
/// - monotonic nanoseconds converted to seconds;
/// - backend-provided elapsed time.
///
/// The module does not assume that timestamps are wall-clock timestamps.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct StabilityObservation {
    /// Observation time in a caller-defined, strictly increasing time unit.
    pub timestamp: f64,

    /// Measured benchmark value.
    pub value: f64,
}

impl StabilityObservation {
    /// Creates a validated observation.
    pub fn new(timestamp: f64, value: f64) -> StabilityResult<Self> {
        validate_finite_timestamp(timestamp, 0)?;
        validate_finite_value(value, 0)?;

        Ok(Self { timestamp, value })
    }
}

/// Configuration governing resource safety and temporal comparisons.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct StabilityConfig {
    /// Maximum observations accepted by one analysis.
    pub max_observations: usize,

    /// Absolute tolerance used when checking equal sampling intervals.
    ///
    /// This is an absolute tolerance in the same time unit as timestamps.
    pub sampling_interval_tolerance: f64,

    /// Relative tolerance used when checking equal sampling intervals.
    ///
    /// The comparison accepts:
    ///
    /// ```text
    /// |actual - expected|
    ///     <= max(absolute_tolerance,
    ///            relative_tolerance * max(|actual|, |expected|))
    /// ```
    pub sampling_interval_relative_tolerance: f64,
}

impl Default for StabilityConfig {
    fn default() -> Self {
        Self {
            max_observations: DEFAULT_MAX_OBSERVATIONS,
            sampling_interval_tolerance: 1.0e-12,
            sampling_interval_relative_tolerance: 1.0e-9,
        }
    }
}

impl StabilityConfig {
    /// Creates an explicit configuration.
    pub const fn new(
        max_observations: usize,
        sampling_interval_tolerance: f64,
        sampling_interval_relative_tolerance: f64,
    ) -> Self {
        Self {
            max_observations,
            sampling_interval_tolerance,
            sampling_interval_relative_tolerance,
        }
    }

    /// Validates configuration values.
    pub fn validate(&self) -> StabilityResult<()> {
        if self.max_observations == 0 {
            return Err(StabilityError::InvalidObservationLimit {
                maximum: self.max_observations,
            });
        }

        if !self.sampling_interval_tolerance.is_finite()
            || self.sampling_interval_tolerance < 0.0
        {
            return Err(StabilityError::InvalidObservation {
                message:
                    "sampling_interval_tolerance must be finite and non-negative"
                        .to_string(),
            });
        }

        if !self.sampling_interval_relative_tolerance.is_finite()
            || self.sampling_interval_relative_tolerance < 0.0
        {
            return Err(StabilityError::InvalidObservation {
                message: "sampling_interval_relative_tolerance must be finite \
                          and non-negative"
                    .to_string(),
            });
        }

        Ok(())
    }
}

/// Population or sample variance selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VarianceKind {
    /// Population variance divides by N.
    Population,

    /// Sample variance divides by N-1.
    Sample,
}

/// A finite mean/variance summary.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct DistributionSummary {
    /// Number of observations.
    pub count: usize,

    /// Arithmetic mean.
    pub mean: f64,

    /// Variance according to the requested variance convention.
    pub variance: f64,

    /// Square root of variance.
    pub standard_deviation: f64,

    /// Minimum observation.
    pub minimum: f64,

    /// Maximum observation.
    pub maximum: f64,

    /// Range = maximum - minimum.
    pub range: f64,
}

impl DistributionSummary {
    /// Computes a numerically stable summary.
    pub fn from_observations(
        observations: &[StabilityObservation],
        variance_kind: VarianceKind,
        config: StabilityConfig,
    ) -> StabilityResult<Self> {
        validate_series(observations, config)?;

        let count = observations.len();

        if matches!(variance_kind, VarianceKind::Sample)
            && count < MIN_VARIANCE_OBSERVATIONS
        {
            return Err(StabilityError::InsufficientObservations {
                operation: "sample variance",
                observations: count,
                minimum: MIN_VARIANCE_OBSERVATIONS,
            });
        }

        let mut mean = 0.0_f64;
        let mut m2 = 0.0_f64;
        let mut minimum = observations[0].value;
        let mut maximum = observations[0].value;

        for (index, observation) in observations.iter().enumerate() {
            let value = observation.value;

            if value < minimum {
                minimum = value;
            }

            if value > maximum {
                maximum = value;
            }

            /*
             * Welford's online algorithm.
             *
             * This avoids accumulating the potentially much larger
             * sum(x^2) and sum(x) separately.
             */
            let n = index
                .checked_add(1)
                .ok_or(StabilityError::NonFiniteResult {
                    operation: "observation count",
                    value: index as f64,
                })?;

            let delta = value - mean;

            mean = checked_finite(
                mean + delta / n as f64,
                "online mean",
            )?;

            let delta2 = value - mean;

            m2 = checked_finite(
                m2 + delta * delta2,
                "online variance accumulator",
            )?;
        }

        let denominator = match variance_kind {
            VarianceKind::Population => count as f64,
            VarianceKind::Sample => (count - 1) as f64,
        };

        if denominator <= 0.0 {
            return Err(StabilityError::InsufficientVarianceDegreesOfFreedom);
        }

        let variance = checked_finite(
            m2 / denominator,
            "variance",
        )?;

        let standard_deviation = checked_finite(
            variance.sqrt(),
            "standard deviation",
        )?;

        let range = checked_finite(
            maximum - minimum,
            "range",
        )?;

        Ok(Self {
            count,
            mean,
            variance,
            standard_deviation,
            minimum,
            maximum,
            range,
        })
    }
}

/// Measures change between the first and last observation.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct EndpointDrift {
    /// First observed value.
    pub initial_value: f64,

    /// Final observed value.
    pub final_value: f64,

    /// Final minus initial.
    pub absolute_change: f64,

    /// Absolute change divided by the absolute initial value.
    ///
    /// `None` means the relative quantity is mathematically undefined because
    /// the initial value is zero.
    pub relative_change: Option<f64>,

    /// Absolute value of the relative change when defined.
    pub absolute_relative_change: Option<f64>,

    /// Time between first and last observation.
    pub elapsed_time: f64,

    /// Absolute change per unit time.
    pub absolute_rate: f64,

    /// Relative change per unit time when defined.
    pub relative_rate: Option<f64>,
}

impl EndpointDrift {
    /// Calculates endpoint drift.
    pub fn from_observations(
        observations: &[StabilityObservation],
        config: StabilityConfig,
    ) -> StabilityResult<Self> {
        validate_series(observations, config)?;

        if observations.len() < MIN_TREND_OBSERVATIONS {
            return Err(StabilityError::InsufficientObservations {
                operation: "endpoint drift",
                observations: observations.len(),
                minimum: MIN_TREND_OBSERVATIONS,
            });
        }

        let first = observations[0];
        let last = observations[observations.len() - 1];

        let absolute_change =
            checked_finite(last.value - first.value, "absolute endpoint change")?;

        let elapsed_time =
            checked_finite(last.timestamp - first.timestamp, "elapsed time")?;

        if elapsed_time <= 0.0 {
            return Err(StabilityError::InvalidObservation {
                message: "elapsed observation time must be greater than zero"
                    .to_string(),
            });
        }

        let absolute_rate =
            checked_finite(absolute_change / elapsed_time, "absolute drift rate")?;

        let (relative_change, absolute_relative_change, relative_rate) =
            if first.value == 0.0 {
                (None, None, None)
            } else {
                let relative_change =
                    checked_finite(
                        absolute_change / first.value,
                        "relative endpoint change",
                    )?;

                let absolute_relative_change =
                    checked_finite(
                        relative_change.abs(),
                        "absolute relative endpoint change",
                    )?;

                let relative_rate =
                    checked_finite(
                        relative_change / elapsed_time,
                        "relative drift rate",
                    )?;

                (
                    Some(relative_change),
                    Some(absolute_relative_change),
                    Some(relative_rate),
                )
            };

        Ok(Self {
            initial_value: first.value,
            final_value: last.value,
            absolute_change,
            relative_change,
            absolute_relative_change,
            elapsed_time,
            absolute_rate,
            relative_rate,
        })
    }
}

/// Linear trend estimate.
///
/// The slope is obtained using ordinary least squares against timestamp.
///
/// ```text
/// y = intercept + slope * t
/// ```
///
/// This is a descriptive trend statistic. It does not prove causality,
/// stationarity, or that the physical system follows a linear drift model.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct LinearTrend {
    /// Number of observations used.
    pub count: usize,

    /// Least-squares intercept.
    pub intercept: f64,

    /// Least-squares slope in value units per timestamp unit.
    pub slope: f64,

    /// Coefficient of determination R² when defined.
    ///
    /// A constant response has no meaningful R² and therefore returns `None`.
    pub r_squared: Option<f64>,

    /// Predicted value at the first timestamp.
    pub fitted_initial_value: f64,

    /// Predicted value at the final timestamp.
    pub fitted_final_value: f64,

    /// Fitted total change over the observed interval.
    pub fitted_change: f64,
}

impl LinearTrend {
    /// Calculates an ordinary-least-squares linear trend.
    pub fn from_observations(
        observations: &[StabilityObservation],
        config: StabilityConfig,
    ) -> StabilityResult<Self> {
        validate_series(observations, config)?;

        let n = observations.len();

        if n < MIN_TREND_OBSERVATIONS {
            return Err(StabilityError::InsufficientObservations {
                operation: "linear trend",
                observations: n,
                minimum: MIN_TREND_OBSERVATIONS,
            });
        }

        /*
         * Shift time to the first timestamp.
         *
         * This substantially improves numerical conditioning when timestamps
         * are large absolute values such as Unix seconds or nanoseconds.
         */
        let t0 = observations[0].timestamp;

        let mut sum_t = 0.0_f64;
        let mut sum_y = 0.0_f64;

        for observation in observations {
            let t = checked_finite(
                observation.timestamp - t0,
                "relative trend time",
            )?;

            sum_t = checked_finite(sum_t + t, "trend sum time")?;
            sum_y = checked_finite(
                sum_y + observation.value,
                "trend sum value",
            )?;
        }

        let mean_t =
            checked_finite(sum_t / n as f64, "trend mean time")?;

        let mean_y =
            checked_finite(sum_y / n as f64, "trend mean value")?;

        let mut s_tt = 0.0_f64;
        let mut s_ty = 0.0_f64;
        let mut s_yy = 0.0_f64;

        for observation in observations {
            let t = observation.timestamp - t0;
            let dt = t - mean_t;
            let dy = observation.value - mean_y;

            s_tt = checked_finite(
                s_tt + dt * dt,
                "trend time sum of squares",
            )?;

            s_ty = checked_finite(
                s_ty + dt * dy,
                "trend covariance numerator",
            )?;

            s_yy = checked_finite(
                s_yy + dy * dy,
                "trend response sum of squares",
            )?;
        }

        if s_tt <= 0.0 {
            return Err(StabilityError::InvalidObservation {
                message:
                    "timestamps contain insufficient variation for a trend"
                        .to_string(),
            });
        }

        let slope =
            checked_finite(s_ty / s_tt, "linear trend slope")?;

        let intercept_relative =
            checked_finite(
                mean_y - slope * mean_t,
                "linear trend intercept",
            )?;

        /*
         * The returned intercept uses the original timestamp coordinate.
         *
         * y = intercept + slope * original_timestamp
         *
         * Computing this directly can overflow for very large timestamps even
         * though the shifted representation is perfectly stable. We therefore
         * retain the shifted intercept only when conversion is finite.
         */
        let intercept =
            checked_finite(
                intercept_relative - slope * t0,
                "linear trend original-time intercept",
            )?;

        let first_relative_time =
            observations[0].timestamp - t0;

        let final_relative_time =
            observations[n - 1].timestamp - t0;

        let fitted_initial_value =
            checked_finite(
                intercept_relative + slope * first_relative_time,
                "fitted initial value",
            )?;

        let fitted_final_value =
            checked_finite(
                intercept_relative + slope * final_relative_time,
                "fitted final value",
            )?;

        let fitted_change =
            checked_finite(
                fitted_final_value - fitted_initial_value,
                "fitted trend change",
            )?;

        let r_squared = if s_yy > 0.0 {
            let value = checked_finite(
                (s_ty * s_ty) / (s_tt * s_yy),
                "trend R-squared",
            )?;

            /*
             * Floating-point roundoff can produce values infinitesimally above
             * one. Clamp only that numerical representation error.
             */
            Some(value.clamp(0.0, 1.0))
        } else {
            None
        };

        Ok(Self {
            count: n,
            intercept,
            slope,
            r_squared,
            fitted_initial_value,
            fitted_final_value,
            fitted_change,
        })
    }
}

/// Local change statistics.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct LocalChangeSummary {
    /// Number of adjacent intervals.
    pub intervals: usize,

    /// Largest positive step.
    pub maximum_increase: f64,

    /// Largest negative step.
    pub maximum_decrease: f64,

    /// Largest absolute step.
    pub maximum_absolute_change: f64,

    /// Mean absolute adjacent change.
    pub mean_absolute_change: f64,

    /// RMS adjacent change.
    pub rms_change: f64,
}

impl LocalChangeSummary {
    /// Calculates adjacent-observation change statistics.
    pub fn from_observations(
        observations: &[StabilityObservation],
        config: StabilityConfig,
    ) -> StabilityResult<Self> {
        validate_series(observations, config)?;

        if observations.len() < MIN_TREND_OBSERVATIONS {
            return Err(StabilityError::InsufficientObservations {
                operation: "local change analysis",
                observations: observations.len(),
                minimum: MIN_TREND_OBSERVATIONS,
            });
        }

        let intervals = observations.len() - 1;

        let mut maximum_increase = f64::NEG_INFINITY;
        let mut maximum_decrease = f64::INFINITY;
        let mut maximum_absolute_change = 0.0_f64;
        let mut sum_absolute_change = 0.0_f64;
        let mut sum_squared_change = 0.0_f64;

        for index in 1..observations.len() {
            let change =
                checked_finite(
                    observations[index].value
                        - observations[index - 1].value,
                    "adjacent change",
                )?;

            if change > maximum_increase {
                maximum_increase = change;
            }

            if change < maximum_decrease {
                maximum_decrease = change;
            }

            let absolute_change = change.abs();

            if absolute_change > maximum_absolute_change {
                maximum_absolute_change = absolute_change;
            }

            sum_absolute_change = checked_finite(
                sum_absolute_change + absolute_change,
                "sum absolute adjacent change",
            )?;

            sum_squared_change = checked_finite(
                sum_squared_change + change * change,
                "sum squared adjacent change",
            )?;
        }

        let mean_absolute_change =
            checked_finite(
                sum_absolute_change / intervals as f64,
                "mean absolute adjacent change",
            )?;

        let rms_change =
            checked_finite(
                (sum_squared_change / intervals as f64).sqrt(),
                "RMS adjacent change",
            )?;

        Ok(Self {
            intervals,
            maximum_increase,
            maximum_decrease,
            maximum_absolute_change,
            mean_absolute_change,
            rms_change,
        })
    }
}

/// Lag-one autocorrelation.
///
/// This is a descriptive serial-correlation statistic. It should not be
/// interpreted as proof of a particular physical noise mechanism.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct AutocorrelationSummary {
    /// Number of observations.
    pub count: usize,

    /// Lag-one Pearson autocorrelation.
    ///
    /// `None` means the denominator is zero and the correlation is undefined.
    pub lag_one: Option<f64>,
}

impl AutocorrelationSummary {
    /// Calculates lag-one autocorrelation.
    pub fn from_observations(
        observations: &[StabilityObservation],
        config: StabilityConfig,
    ) -> StabilityResult<Self> {
        validate_series(observations, config)?;

        let n = observations.len();

        if n < MIN_AUTOCORRELATION_OBSERVATIONS {
            return Err(StabilityError::InsufficientObservations {
                operation: "lag-one autocorrelation",
                observations: n,
                minimum: MIN_AUTOCORRELATION_OBSERVATIONS,
            });
        }

        let summary = DistributionSummary::from_observations(
            observations,
            VarianceKind::Population,
            config,
        )?;

        if summary.variance == 0.0 {
            return Ok(Self {
                count: n,
                lag_one: None,
            });
        }

        let mean = summary.mean;

        let mut numerator = 0.0_f64;
        let mut denominator = 0.0_f64;

        for index in 0..n {
            let centered = observations[index].value - mean;

            denominator = checked_finite(
                denominator + centered * centered,
                "autocorrelation denominator",
            )?;

            if index > 0 {
                let previous = observations[index - 1].value - mean;

                numerator = checked_finite(
                    numerator + previous * centered,
                    "autocorrelation numerator",
                )?;
            }
        }

        if denominator == 0.0 {
            return Ok(Self {
                count: n,
                lag_one: None,
            });
        }

        let lag_one =
            checked_finite(
                numerator / denominator,
                "lag-one autocorrelation",
            )?;

        Ok(Self {
            count: n,
            lag_one: Some(lag_one.clamp(-1.0, 1.0)),
        })
    }
}

/// Allan deviation result.
///
/// This implementation assumes the supplied values represent an appropriate
/// frequency-like or fractional-frequency-like series and are sampled at
/// regular intervals.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct AllanDeviation {
    /// Number of observations.
    pub count: usize,

    /// Sampling interval.
    pub tau: f64,

    /// Allan variance.
    pub variance: f64,

    /// Allan deviation.
    pub deviation: f64,

    /// Number of adjacent differences used.
    pub difference_count: usize,
}

impl AllanDeviation {
    /// Calculates the non-overlapping adjacent Allan deviation for an equally
    /// spaced series.
    pub fn from_observations(
        observations: &[StabilityObservation],
        tau: f64,
        config: StabilityConfig,
    ) -> StabilityResult<Self> {
        validate_series(observations, config)?;

        if observations.len() < MIN_ALLAN_OBSERVATIONS {
            return Err(StabilityError::InsufficientObservations {
                operation: "Allan deviation",
                observations: observations.len(),
                minimum: MIN_ALLAN_OBSERVATIONS,
            });
        }

        if !tau.is_finite() || tau <= 0.0 {
            return Err(StabilityError::InvalidAllanInterval { tau });
        }

        let first_interval =
            observations[1].timestamp - observations[0].timestamp;

        validate_positive_interval(first_interval, 1)?;

        for index in 2..observations.len() {
            let interval =
                observations[index].timestamp
                    - observations[index - 1].timestamp;

            validate_positive_interval(interval, index)?;

            if !approximately_equal(
                interval,
                first_interval,
                config.sampling_interval_tolerance,
                config.sampling_interval_relative_tolerance,
            ) {
                return Err(StabilityError::UnequalSamplingIntervals {
                    expected: first_interval,
                    actual: interval,
                    index,
                });
            }
        }

        let difference_count = observations.len() - 1;

        let mut sum_squared_difference = 0.0_f64;

        for index in 1..observations.len() {
            let difference =
                checked_finite(
                    observations[index].value
                        - observations[index - 1].value,
                    "Allan adjacent difference",
                )?;

            sum_squared_difference = checked_finite(
                sum_squared_difference + difference * difference,
                "Allan squared-difference accumulator",
            )?;
        }

        let variance =
            checked_finite(
                sum_squared_difference
                    / (2.0 * difference_count as f64),
                "Allan variance",
            )?;

        let deviation =
            checked_finite(
                variance.sqrt(),
                "Allan deviation",
            )?;

        Ok(Self {
            count: observations.len(),
            tau,
            variance,
            deviation,
            difference_count,
        })
    }
}

/// Control-limit interval around a baseline.
///
/// This is deliberately a descriptive envelope. It does not itself implement
/// a complete statistical process-control decision procedure.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ControlLimits {
    /// Center/baseline.
    pub center: f64,

    /// Standard deviation used to construct the limits.
    pub standard_deviation: f64,

    /// Sigma multiplier.
    pub sigma: f64,

    /// Lower limit.
    pub lower: f64,

    /// Upper limit.
    pub upper: f64,
}

impl ControlLimits {
    /// Constructs symmetric limits around a center.
    pub fn new(
        center: f64,
        standard_deviation: f64,
        sigma: f64,
    ) -> StabilityResult<Self> {
        validate_finite_value(center, 0)?;
        validate_finite_value(standard_deviation, 0)?;

        if standard_deviation < 0.0 {
            return Err(StabilityError::InvalidObservation {
                message:
                    "standard deviation cannot be negative".to_string(),
            });
        }

        if !sigma.is_finite() || sigma <= 0.0 {
            return Err(StabilityError::InvalidSigma { sigma });
        }

        let margin =
            checked_finite(
                sigma * standard_deviation,
                "control-limit margin",
            )?;

        let lower =
            checked_finite(center - margin, "control lower limit")?;

        let upper =
            checked_finite(center + margin, "control upper limit")?;

        Ok(Self {
            center,
            standard_deviation,
            sigma,
            lower,
            upper,
        })
    }

    /// Returns whether a value lies outside the limits.
    #[must_use]
    pub fn is_outside(&self, value: f64) -> StabilityResult<bool> {
        validate_finite_value(value, 0)?;
        Ok(value < self.lower || value > self.upper)
    }

    /// Counts observations outside the limits.
    pub fn count_outside(
        &self,
        observations: &[StabilityObservation],
        config: StabilityConfig,
    ) -> StabilityResult<usize> {
        validate_series(observations, config)?;

        let mut count = 0usize;

        for observation in observations {
            if self.is_outside(observation.value)? {
                count = count.checked_add(1).ok_or(
                    StabilityError::InvalidObservation {
                        message:
                            "control-limit count overflow".to_string(),
                    },
                )?;
            }
        }

        Ok(count)
    }
}

/// Median absolute deviation summary.
///
/// MAD is a robust dispersion statistic useful for stability diagnostics when
/// isolated large excursions should be distinguished from the bulk of the
/// series without silently deleting those excursions.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct MadSummary {
    /// Median of the observations.
    pub median: f64,

    /// Median absolute deviation from the median.
    pub mad: f64,

    /// Number of observations.
    pub count: usize,
}

impl MadSummary {
    /// Calculates a deterministic MAD.
    ///
    /// The implementation allocates exactly one bounded copy of the values.
    pub fn from_observations(
        observations: &[StabilityObservation],
        config: StabilityConfig,
    ) -> StabilityResult<Self> {
        validate_series(observations, config)?;

        let mut values = Vec::with_capacity(observations.len());

        for observation in observations {
            values.push(observation.value);
        }

        values.sort_by(|a, b| a.total_cmp(b));

        let median = median_sorted(&values);

        let mut deviations = Vec::with_capacity(values.len());

        for value in &values {
            deviations.push((value - median).abs());
        }

        deviations.sort_by(|a, b| a.total_cmp(b));

        let mad = median_sorted(&deviations);

        Ok(Self {
            median,
            mad,
            count: observations.len(),
        })
    }
}

/// Complete stability-analysis result.
///
/// This is intentionally a module-level result object. Higher-level
/// `core::result::BenchmarkResult` integration can embed this structure
/// without requiring this module to depend upward on the result model.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StabilityAnalysis {
    /// Algorithm identifier used to calculate this result.
    pub algorithm_id: String,

    /// Number of observations analyzed.
    pub observation_count: usize,

    /// Distribution summary using sample variance.
    pub distribution: DistributionSummary,

    /// Endpoint drift.
    pub endpoint_drift: EndpointDrift,

    /// Linear trend.
    pub linear_trend: LinearTrend,

    /// Adjacent/local change statistics.
    pub local_change: LocalChangeSummary,

    /// Lag-one autocorrelation.
    pub autocorrelation: AutocorrelationSummary,

    /// Robust MAD summary.
    pub mad: MadSummary,

    /// Optional Allan deviation.
    ///
    /// It is `None` because Allan deviation is only valid when the caller
    /// explicitly supplies a suitable sampling interval and interpretation.
    pub allan_deviation: Option<AllanDeviation>,
}

impl StabilityAnalysis {
    /// Runs the complete generic stability analysis without Allan deviation.
    pub fn analyze(
        observations: &[StabilityObservation],
        config: StabilityConfig,
    ) -> StabilityResult<Self> {
        config.validate()?;
        validate_series(observations, config)?;

        let distribution = DistributionSummary::from_observations(
            observations,
            VarianceKind::Sample,
            config,
        )?;

        let endpoint_drift =
            EndpointDrift::from_observations(observations, config)?;

        let linear_trend =
            LinearTrend::from_observations(observations, config)?;

        let local_change =
            LocalChangeSummary::from_observations(observations, config)?;

        let autocorrelation =
            AutocorrelationSummary::from_observations(
                observations,
                config,
            )?;

        let mad =
            MadSummary::from_observations(observations, config)?;

        Ok(Self {
            algorithm_id: STABILITY_ALGORITHM_ID.to_string(),
            observation_count: observations.len(),
            distribution,
            endpoint_drift,
            linear_trend,
            local_change,
            autocorrelation,
            mad,
            allan_deviation: None,
        })
    }

    /// Runs complete stability analysis and adds Allan deviation.
    pub fn analyze_with_allan(
        observations: &[StabilityObservation],
        tau: f64,
        config: StabilityConfig,
    ) -> StabilityResult<Self> {
        let mut result = Self::analyze(observations, config)?;

        result.allan_deviation = Some(
            AllanDeviation::from_observations(
                observations,
                tau,
                config,
            )?,
        );

        Ok(result)
    }

    /// Returns a dimensionless coefficient of variation when the mean is
    /// non-zero.
    ///
    /// This is:
    ///
    /// ```text
    /// sample_standard_deviation / |mean|
    /// ```
    ///
    /// It is intentionally undefined for a zero mean because manufacturing a
    /// large or infinite value would be misleading.
    pub fn coefficient_of_variation(&self) -> Option<f64> {
        if self.distribution.mean == 0.0 {
            return None;
        }

        let value =
            self.distribution.standard_deviation
                / self.distribution.mean.abs();

        if value.is_finite() {
            Some(value)
        } else {
            None
        }
    }

    /// Constructs conventional three-sigma limits around the sample mean.
    pub fn three_sigma_limits(&self) -> StabilityResult<ControlLimits> {
        ControlLimits::new(
            self.distribution.mean,
            self.distribution.standard_deviation,
            DEFAULT_CONTROL_LIMIT_SIGMAS,
        )
    }

    /// Constructs custom sigma limits around the sample mean.
    pub fn control_limits(
        &self,
        sigma: f64,
    ) -> StabilityResult<ControlLimits> {
        ControlLimits::new(
            self.distribution.mean,
            self.distribution.standard_deviation,
            sigma,
        )
    }

    /// Returns the fraction of observations outside supplied control limits.
    pub fn outside_limit_fraction(
        &self,
        observations: &[StabilityObservation],
        limits: &ControlLimits,
        config: StabilityConfig,
    ) -> StabilityResult<f64> {
        validate_series(observations, config)?;

        let outside = limits.count_outside(observations, config)?;

        checked_finite(
            outside as f64 / observations.len() as f64,
            "outside-limit fraction",
        )
    }
}

/// Validates a complete observation series.
fn validate_series(
    observations: &[StabilityObservation],
    config: StabilityConfig,
) -> StabilityResult<()> {
    config.validate()?;

    if observations.is_empty() {
        return Err(StabilityError::EmptyObservations);
    }

    if observations.len() > config.max_observations {
        return Err(StabilityError::ObservationLimitExceeded {
            observations: observations.len(),
            maximum: config.max_observations,
        });
    }

    for (index, observation) in observations.iter().enumerate() {
        validate_finite_timestamp(observation.timestamp, index)?;
        validate_finite_value(observation.value, index)?;

        if index > 0 {
            let previous = observations[index - 1].timestamp;
            let current = observation.timestamp;

            if current <= previous {
                return Err(StabilityError::NonMonotonicTimestamp {
                    index,
                    previous,
                    current,
                });
            }
        }
    }

    Ok(())
}

/// Validates a finite observation value.
fn validate_finite_value(
    value: f64,
    index: usize,
) -> StabilityResult<()> {
    if !value.is_finite() {
        return Err(StabilityError::NonFiniteValue { index, value });
    }

    Ok(())
}

/// Validates a finite timestamp.
fn validate_finite_timestamp(
    timestamp: f64,
    index: usize,
) -> StabilityResult<()> {
    if !timestamp.is_finite() {
        return Err(StabilityError::NonFiniteTimestamp {
            index,
            timestamp,
        });
    }

    Ok(())
}

/// Validates a positive time interval.
fn validate_positive_interval(
    interval: f64,
    index: usize,
) -> StabilityResult<()> {
    if !interval.is_finite() {
        return Err(StabilityError::NonFiniteResult {
            operation: "time interval",
            value: interval,
        });
    }

    if interval == 0.0 {
        return Err(StabilityError::ZeroTimeInterval { index });
    }

    if interval < 0.0 {
        return Err(StabilityError::NegativeTimeInterval {
            index,
            interval,
        });
    }

    Ok(())
}

/// Checks that a floating-point result is finite.
fn checked_finite(
    value: f64,
    operation: &'static str,
) -> StabilityResult<f64> {
    if value.is_finite() {
        Ok(value)
    } else {
        Err(StabilityError::NonFiniteResult { operation, value })
    }
}

/// Tests approximate equality for sampling intervals.
fn approximately_equal(
    a: f64,
    b: f64,
    absolute_tolerance: f64,
    relative_tolerance: f64,
) -> bool {
    let difference = (a - b).abs();

    if difference <= absolute_tolerance {
        return true;
    }

    difference
        <= relative_tolerance * a.abs().max(b.abs())
}

/// Returns the median of a non-empty sorted slice.
fn median_sorted(values: &[f64]) -> f64 {
    debug_assert!(!values.is_empty());

    let middle = values.len() / 2;

    if values.len() % 2 == 0 {
        (values[middle - 1] + values[middle]) / 2.0
    } else {
        values[middle]
    }
}

/// Validates a probability-like benchmark series.
///
/// This helper is intentionally separate from generic stability analysis:
/// generic stability values may legitimately be outside [0, 1], while
/// probabilities, fidelities, success probabilities and many error rates may
/// not be.
pub fn validate_probability_series(
    observations: &[StabilityObservation],
    config: StabilityConfig,
) -> StabilityResult<()> {
    validate_series(observations, config)?;

    for (index, observation) in observations.iter().enumerate() {
        if !(0.0..=1.0).contains(&observation.value) {
            return Err(StabilityError::InvalidProbability {
                index,
                value: observation.value,
            });
        }
    }

    Ok(())
}

/// Calculates a stability index from the coefficient of variation.
///
/// This function deliberately names the returned quantity `stability_index`
/// rather than pretending it is a universal physical stability definition.
///
/// Lower values mean less relative dispersion.
///
/// It requires a non-zero mean.
pub fn stability_index(
    observations: &[StabilityObservation],
    config: StabilityConfig,
) -> StabilityResult<f64> {
    let summary = DistributionSummary::from_observations(
        observations,
        VarianceKind::Sample,
        config,
    )?;

    if summary.mean == 0.0 {
        return Err(StabilityError::UndefinedRelativeMetric {
            operation: "stability index",
        });
    }

    checked_finite(
        summary.standard_deviation / summary.mean.abs(),
        "stability index",
    )
}

/// Calculates the absolute endpoint drift relative to a caller-supplied
/// non-zero baseline.
///
/// This is useful when the benchmark has an externally defined target or
/// calibration baseline rather than using the first observation as the
/// reference.
pub fn relative_change_from_baseline(
    final_value: f64,
    baseline: f64,
) -> StabilityResult<f64> {
    if !final_value.is_finite() {
        return Err(StabilityError::NonFiniteValue {
            index: 0,
            value: final_value,
        });
    }

    if !baseline.is_finite() {
        return Err(StabilityError::InvalidBaseline { baseline });
    }

    if baseline == 0.0 {
        return Err(StabilityError::UndefinedRelativeMetric {
            operation: "relative change from baseline",
        });
    }

    checked_finite(
        (final_value - baseline) / baseline,
        "relative change from baseline",
    )
}

/// Calculates a baseline-relative drift rate.
///
/// The returned value is relative change per timestamp unit.
pub fn relative_drift_rate_from_baseline(
    initial_value: f64,
    final_value: f64,
    elapsed_time: f64,
    baseline: f64,
) -> StabilityResult<f64> {
    if !initial_value.is_finite()
        || !final_value.is_finite()
    {
        return Err(StabilityError::NonFiniteValue {
            index: 0,
            value: if !initial_value.is_finite() {
                initial_value
            } else {
                final_value
            },
        });
    }

    if !baseline.is_finite() {
        return Err(StabilityError::InvalidBaseline { baseline });
    }

    if !elapsed_time.is_finite() || elapsed_time <= 0.0 {
        return Err(StabilityError::InvalidObservation {
            message:
                "elapsed_time must be finite and greater than zero"
                    .to_string(),
        });
    }

    let relative_change =
        relative_change_from_baseline(final_value, baseline)?;

    checked_finite(
        relative_change / elapsed_time,
        "baseline-relative drift rate",
    )
}

/// Calculates a simple absolute threshold exceedance count.
///
/// This is useful for protocol-level drift detection while keeping the actual
/// policy outside this low-level metric module.
pub fn count_absolute_exceedances(
    observations: &[StabilityObservation],
    baseline: f64,
    threshold: f64,
    config: StabilityConfig,
) -> StabilityResult<usize> {
    validate_series(observations, config)?;

    if !baseline.is_finite() {
        return Err(StabilityError::InvalidBaseline { baseline });
    }

    if !threshold.is_finite() || threshold < 0.0 {
        return Err(StabilityError::InvalidThreshold { threshold });
    }

    let mut count = 0usize;

    for observation in observations {
        if (observation.value - baseline).abs() > threshold {
            count = count.checked_add(1).ok_or(
                StabilityError::InvalidObservation {
                    message:
                        "absolute exceedance count overflow".to_string(),
                },
            )?;
        }
    }

    Ok(count)
}

/// Calculates a relative threshold exceedance count.
///
/// The baseline must be non-zero.
pub fn count_relative_exceedances(
    observations: &[StabilityObservation],
    baseline: f64,
    relative_threshold: f64,
    config: StabilityConfig,
) -> StabilityResult<usize> {
    validate_series(observations, config)?;

    if !baseline.is_finite() {
        return Err(StabilityError::InvalidBaseline { baseline });
    }

    if baseline == 0.0 {
        return Err(StabilityError::UndefinedRelativeMetric {
            operation: "relative threshold exceedance",
        });
    }

    if !relative_threshold.is_finite()
        || relative_threshold < 0.0
    {
        return Err(StabilityError::InvalidThreshold {
            threshold: relative_threshold,
        });
    }

    let mut count = 0usize;

    for observation in observations {
        let relative =
            (observation.value - baseline).abs()
                / baseline.abs();

        if relative > relative_threshold {
            count = count.checked_add(1).ok_or(
                StabilityError::InvalidObservation {
                    message:
                        "relative exceedance count overflow".to_string(),
                },
            )?;
        }
    }

    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn series(values: &[f64]) -> Vec<StabilityObservation> {
        values
            .iter()
            .enumerate()
            .map(|(index, value)| {
                StabilityObservation::new(index as f64, *value)
                    .unwrap()
            })
            .collect()
    }

    #[test]
    fn stable_constant_series_has_zero_dispersion() {
        let observations = series(&[10.0, 10.0, 10.0, 10.0]);

        let summary = DistributionSummary::from_observations(
            &observations,
            VarianceKind::Sample,
            StabilityConfig::default(),
        )
        .unwrap();

        assert_eq!(summary.mean, 10.0);
        assert_eq!(summary.variance, 0.0);
        assert_eq!(summary.standard_deviation, 0.0);
        assert_eq!(summary.range, 0.0);
    }

    #[test]
    fn endpoint_drift_is_correct() {
        let observations = series(&[10.0, 12.0, 15.0]);

        let result = EndpointDrift::from_observations(
            &observations,
            StabilityConfig::default(),
        )
        .unwrap();

        assert_eq!(result.initial_value, 10.0);
        assert_eq!(result.final_value, 15.0);
        assert_eq!(result.absolute_change, 5.0);
        assert_eq!(result.relative_change, Some(0.5));
        assert_eq!(result.elapsed_time, 2.0);
        assert_eq!(result.absolute_rate, 2.5);
        assert_eq!(result.relative_rate, Some(0.25));
    }

    #[test]
    fn zero_baseline_has_undefined_relative_change() {
        let observations = series(&[0.0, 1.0]);

        let result = EndpointDrift::from_observations(
            &observations,
            StabilityConfig::default(),
        )
        .unwrap();

        assert_eq!(result.relative_change, None);
        assert_eq!(result.absolute_relative_change, None);
        assert_eq!(result.relative_rate, None);
    }

    #[test]
    fn linear_trend_recovers_exact_line() {
        let observations = vec![
            StabilityObservation::new(0.0, 2.0).unwrap(),
            StabilityObservation::new(1.0, 5.0).unwrap(),
            StabilityObservation::new(2.0, 8.0).unwrap(),
            StabilityObservation::new(3.0, 11.0).unwrap(),
        ];

        let trend = LinearTrend::from_observations(
            &observations,
            StabilityConfig::default(),
        )
        .unwrap();

        assert!((trend.slope - 3.0).abs() < 1.0e-12);
        assert!(trend.r_squared.unwrap() > 1.0 - 1.0e-12);
        assert!((trend.fitted_change - 9.0).abs() < 1.0e-12);
    }

    #[test]
    fn local_changes_are_correct() {
        let observations = series(&[1.0, 4.0, 2.0, 5.0]);

        let result = LocalChangeSummary::from_observations(
            &observations,
            StabilityConfig::default(),
        )
        .unwrap();

        assert_eq!(result.maximum_increase, 3.0);
        assert_eq!(result.maximum_decrease, -2.0);
        assert_eq!(result.maximum_absolute_change, 3.0);
        assert!((result.mean_absolute_change - 8.0 / 3.0).abs() < 1.0e-12);
    }

    #[test]
    fn autocorrelation_is_defined_for_nonconstant_series() {
        let observations = series(&[1.0, 2.0, 3.0, 4.0]);

        let result = AutocorrelationSummary::from_observations(
            &observations,
            StabilityConfig::default(),
        )
        .unwrap();

        assert!(result.lag_one.is_some());
        assert!(result.lag_one.unwrap() > 0.0);
    }

    #[test]
    fn autocorrelation_is_undefined_for_constant_series() {
        let observations = series(&[5.0, 5.0, 5.0]);

        let result = AutocorrelationSummary::from_observations(
            &observations,
            StabilityConfig::default(),
        )
        .unwrap();

        assert_eq!(result.lag_one, None);
    }

    #[test]
    fn allan_deviation_is_zero_for_constant_series() {
        let observations = series(&[1.0, 1.0, 1.0, 1.0]);

        let result = AllanDeviation::from_observations(
            &observations,
            1.0,
            StabilityConfig::default(),
        )
        .unwrap();

        assert_eq!(result.variance, 0.0);
        assert_eq!(result.deviation, 0.0);
        assert_eq!(result.tau, 1.0);
    }

    #[test]
    fn allan_deviation_matches_definition() {
        let observations = series(&[0.0, 1.0, 0.0]);

        let result = AllanDeviation::from_observations(
            &observations,
            1.0,
            StabilityConfig::default(),
        )
        .unwrap();

        /*
         * Differences: +1, -1
         *
         * Allan variance:
         *
         * (1 / (2 * 2)) * (1² + (-1)²) = 0.5
         */
        assert!((result.variance - 0.5).abs() < 1.0e-12);
        assert!((result.deviation - 0.5_f64.sqrt()).abs() < 1.0e-12);
    }

    #[test]
    fn allan_rejects_unequal_sampling() {
        let observations = vec![
            StabilityObservation::new(0.0, 1.0).unwrap(),
            StabilityObservation::new(1.0, 2.0).unwrap(),
            StabilityObservation::new(3.5, 3.0).unwrap(),
        ];

        let result = AllanDeviation::from_observations(
            &observations,
            1.0,
            StabilityConfig::default(),
        );

        assert!(matches!(
            result,
            Err(StabilityError::UnequalSamplingIntervals { .. })
        ));
    }

    #[test]
    fn control_limits_detect_outlier_without_removing_it() {
        let observations = series(&[
            10.0,
            10.0,
            10.0,
            10.0,
            20.0,
        ]);

        let analysis = StabilityAnalysis::analyze(
            &observations,
            StabilityConfig::default(),
        )
        .unwrap();

        let limits = analysis.three_sigma_limits().unwrap();

        let outside = limits
            .count_outside(
                &observations,
                StabilityConfig::default(),
            )
            .unwrap();

        assert!(outside >= 1);
        assert_eq!(analysis.observation_count, 5);
    }

    #[test]
    fn mad_handles_large_excursion() {
        let observations = series(&[
            1.0,
            1.0,
            1.0,
            1.0,
            100.0,
        ]);

        let result = MadSummary::from_observations(
            &observations,
            StabilityConfig::default(),
        )
        .unwrap();

        assert_eq!(result.median, 1.0);
        assert_eq!(result.mad, 0.0);
    }

    #[test]
    fn probability_validation_accepts_valid_values() {
        let observations = series(&[
            0.0,
            0.25,
            0.5,
            0.75,
            1.0,
        ]);

        assert!(
            validate_probability_series(
                &observations,
                StabilityConfig::default(),
            )
            .is_ok()
        );
    }

    #[test]
    fn probability_validation_rejects_invalid_values() {
        let observations = series(&[
            0.0,
            0.5,
            1.2,
        ]);

        assert!(matches!(
            validate_probability_series(
                &observations,
                StabilityConfig::default(),
            ),
            Err(StabilityError::InvalidProbability { .. })
        ));
    }

    #[test]
    fn stability_index_is_dimensionless() {
        let observations = series(&[
            9.0,
            10.0,
            11.0,
        ]);

        let index = stability_index(
            &observations,
            StabilityConfig::default(),
        )
        .unwrap();

        assert!(index >= 0.0);
        assert!(index.is_finite());
    }

    #[test]
    fn stability_index_rejects_zero_mean() {
        let observations = series(&[
            -1.0,
            0.0,
            1.0,
        ]);

        let result = stability_index(
            &observations,
            StabilityConfig::default(),
        );

        assert!(matches!(
            result,
            Err(StabilityError::UndefinedRelativeMetric { .. })
        ));
    }

    #[test]
    fn absolute_threshold_count_is_correct() {
        let observations = series(&[
            10.0,
            10.5,
            11.0,
            13.0,
        ]);

        let count = count_absolute_exceedances(
            &observations,
            10.0,
            1.0,
            StabilityConfig::default(),
        )
        .unwrap();

        assert_eq!(count, 1);
    }

    #[test]
    fn relative_threshold_count_is_correct() {
        let observations = series(&[
            100.0,
            101.0,
            105.0,
            110.0,
        ]);

        let count = count_relative_exceedances(
            &observations,
            100.0,
            0.05,
            StabilityConfig::default(),
        )
        .unwrap();

        assert_eq!(count, 1);
    }

    #[test]
    fn relative_baseline_change_is_correct() {
        let result =
            relative_change_from_baseline(110.0, 100.0)
                .unwrap();

        assert!((result - 0.10).abs() < 1.0e-12);
    }

    #[test]
    fn relative_baseline_change_rejects_zero_baseline() {
        let result =
            relative_change_from_baseline(10.0, 0.0);

        assert!(matches!(
            result,
            Err(StabilityError::UndefinedRelativeMetric { .. })
        ));
    }

    #[test]
    fn non_monotonic_timestamps_are_rejected() {
        let observations = vec![
            StabilityObservation::new(0.0, 1.0).unwrap(),
            StabilityObservation::new(2.0, 2.0).unwrap(),
            StabilityObservation::new(1.0, 3.0).unwrap(),
        ];

        let result = StabilityAnalysis::analyze(
            &observations,
            StabilityConfig::default(),
        );

        assert!(matches!(
            result,
            Err(StabilityError::NonMonotonicTimestamp { .. })
        ));
    }

    #[test]
    fn non_finite_values_are_rejected() {
        let result =
            StabilityObservation::new(0.0, f64::NAN);

        assert!(matches!(
            result,
            Err(StabilityError::NonFiniteValue { .. })
        ));

        let result =
            StabilityObservation::new(0.0, f64::INFINITY);

        assert!(matches!(
            result,
            Err(StabilityError::NonFiniteValue { .. })
        ));
    }

    #[test]
    fn observation_limit_is_enforced() {
        let observations = series(&[
            1.0,
            2.0,
            3.0,
        ]);

        let config = StabilityConfig::new(
            2,
            1.0e-12,
            1.0e-9,
        );

        let result = StabilityAnalysis::analyze(
            &observations,
            config,
        );

        assert!(matches!(
            result,
            Err(StabilityError::ObservationLimitExceeded { .. })
        ));
    }

    #[test]
    fn complete_analysis_contains_all_core_components() {
        let observations = series(&[
            1.0,
            1.1,
            1.05,
            1.2,
            1.15,
        ]);

        let result = StabilityAnalysis::analyze(
            &observations,
            StabilityConfig::default(),
        )
        .unwrap();

        assert_eq!(
            result.algorithm_id,
            STABILITY_ALGORITHM_ID
        );

        assert_eq!(result.observation_count, 5);
        assert!(result.distribution.mean.is_finite());
        assert!(result.endpoint_drift.absolute_change.is_finite());
        assert!(result.linear_trend.slope.is_finite());
        assert!(result.local_change.rms_change.is_finite());
        assert!(result.mad.mad.is_finite());
        assert!(result.allan_deviation.is_none());
    }

    #[test]
    fn complete_analysis_can_include_allan_deviation() {
        let observations = series(&[
            1.0,
            1.01,
            1.00,
            1.02,
            1.01,
        ]);

        let result = StabilityAnalysis::analyze_with_allan(
            &observations,
            1.0,
            StabilityConfig::default(),
        )
        .unwrap();

        assert!(result.allan_deviation.is_some());
        assert!(
            result
                .allan_deviation
                .unwrap()
                .deviation
                .is_finite()
        );
    }

    #[test]
    fn sampling_tolerance_allows_small_rounding_error() {
        let observations = vec![
            StabilityObservation::new(0.0, 1.0).unwrap(),
            StabilityObservation::new(1.0, 2.0).unwrap(),
            StabilityObservation::new(2.0000000001, 3.0).unwrap(),
        ];

        let config = StabilityConfig::new(
            DEFAULT_MAX_OBSERVATIONS,
            1.0e-9,
            1.0e-9,
        );

        let result = AllanDeviation::from_observations(
            &observations,
            1.0,
            config,
        );

        assert!(result.is_ok());
    }

    #[test]
    fn control_limit_sigma_must_be_positive() {
        let result =
            ControlLimits::new(1.0, 0.1, 0.0);

        assert!(matches!(
            result,
            Err(StabilityError::InvalidSigma { .. })
        ));
    }

    #[test]
    fn empty_series_is_rejected() {
        let result = DistributionSummary::from_observations(
            &[],
            VarianceKind::Sample,
            StabilityConfig::default(),
        );

        assert!(matches!(
            result,
            Err(StabilityError::EmptyObservations)
        ));
    }

    #[test]
    fn one_observation_cannot_produce_sample_variance() {
        let observations = series(&[1.0]);

        let result = DistributionSummary::from_observations(
            &observations,
            VarianceKind::Sample,
            StabilityConfig::default(),
        );

        assert!(matches!(
            result,
            Err(StabilityError::InsufficientObservations { .. })
        ));
    }
}