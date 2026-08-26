//! Zamani Quantum Benchmarking — Quantum Hardware Drift Benchmark
//!
//! # Purpose
//!
//! This module detects, quantifies, and reports time-dependent changes in
//! quantum-device or quantum-system performance.
//!
//! Drift is different from ordinary outlier detection:
//!
//! ```text
//! outlier:
//!     one observation is unusually far from the population
//!
//! drift:
//!     the statistical behavior of the system changes over time
//! ```
//!
//! A production drift benchmark therefore preserves the complete time series
//! and reports:
//!
//! - baseline behavior;
//! - latest behavior;
//! - absolute change;
//! - relative change;
//! - linear trend;
//! - trend confidence information;
//! - exponentially weighted moving-average state;
//! - cumulative-sum change detection;
//! - largest observed excursion;
//! - direction of change;
//! - stability;
//! - data-quality information;
//! - calibration/backend provenance;
//! - warnings and detected events.
//!
//! # Architectural position
//!
//! ```text
//! quantum::hardware::calibration
//!          │
//!          │ calibration snapshots
//!          ▼
//! quantum benchmark execution
//!          │
//!          │ raw observations + timestamps
//!          ▼
//! core::observation
//!          │
//!          ▼
//! protocols::drift
//!          │
//!     ┌────┴──────────────┐
//!     ▼                   ▼
//! statistics          metrics
//!     │                   │
//!     └────────┬──────────┘
//!              ▼
//!       core::result
//!              │
//!       ┌──────┴──────┐
//!       ▼             ▼
//! analysis        reporting
//! ```
//!
//! This module does NOT:
//!
//! - execute quantum circuits;
//! - communicate with hardware;
//! - perform calibration;
//! - modify calibration data;
//! - generate quantum circuits;
//! - perform routing;
//! - perform scheduling;
//! - mutate Quantum IR;
//! - silently remove outliers;
//! - claim that correlation proves physical causation;
//! - assume a gate-model QPU;
//! - require a particular quantum technology;
//! - print diagnostics;
//! - maintain global state.
//!
//! # Supported drift targets
//!
//! A drift series may represent any scalar benchmark quantity, including:
//!
//! - gate error rate;
//! - gate fidelity;
//! - cycle error;
//! - readout error;
//! - readout fidelity;
//! - T1;
//! - T2;
//! - T2*;
//! - qubit frequency;
//! - leakage;
//! - crosstalk;
//! - randomized-benchmarking decay parameter;
//! - XEB fidelity;
//! - application success probability;
//! - logical error rate;
//! - decoder failure rate;
//! - circuit latency;
//! - queue latency;
//! - execution time;
//! - throughput;
//! - energy;
//! - calibration parameters;
//! - arbitrary user-defined benchmark metrics.
//!
//! The metric itself is identified by a stable string and its unit/direction
//! semantics are carried in the sample metadata.
//!
//! # Scientific interpretation
//!
//! Drift detection is observational.
//!
//! A detected change means:
//!
//! ```text
//! metric behavior changed over time
//! ```
//!
//! It does NOT by itself establish:
//!
//! ```text
//! temperature caused the change
//! calibration caused the change
//! TLS caused the change
//! control electronics caused the change
//! ```
//!
//! Causal attribution belongs to `analysis::attribution`.
//!
//! This distinction is intentional.
//!
//! # Statistical methods
//!
//! The implementation combines several complementary diagnostics:
//!
//! 1. Baseline comparison.
//! 2. Absolute and relative change.
//! 3. Ordinary least-squares linear trend.
//! 4. EWMA change detection.
//! 5. Two-sided CUSUM change detection.
//! 6. Largest excursion from baseline.
//!
//! No single detector is treated as universal.
//!
//! This is particularly important because quantum-device noise can be
//! time-correlated. Standard benchmark assumptions may become invalid when
//! errors drift or are temporally correlated. Drift is therefore reported
//! explicitly instead of being hidden inside a benchmark's nominal metric.
//!
//! # Outliers
//!
//! This module does not silently discard observations.
//!
//! The existing `statistics::outliers` subsystem is responsible for robust
//! outlier classification. A caller may pass samples with:
//!
//! ```text
//! quality = Valid
//! quality = Flagged
//! quality = Invalid
//! ```
//!
//! Flagged observations remain in the original series and are counted in the
//! result. They may optionally be excluded from the trend/detector calculations
//! through configuration.
//!
//! This preserves the repository's non-destructive outlier policy.
//!
//! # Timestamp semantics
//!
//! Timestamps are represented as unsigned nanoseconds from an externally
//! defined epoch. The benchmark does not assume that the epoch is Unix time.
//!
//! For hardware integration, the recommended source is the timestamp carried
//! by the calibration/backend observation.
//!
//! The timestamp must be strictly increasing within one metric series.
//!
//! This prevents ambiguous ordering and avoids incorrectly interpreting a
//! reordered distributed execution stream as physical drift.
//!
//! # Reproducibility
//!
//! Drift analysis itself is deterministic.
//!
//! Given the same:
//!
//! - metric identifier;
//! - configuration;
//! - ordered samples;
//! - baseline definition;
//! - detector parameters;
//!
//! it produces the same result.
//!
//! No random number generator is used.
//!
//! # Resource safety
//!
//! The implementation applies explicit limits before allocating result
//! structures. This prevents a malformed benchmark request from forcing
//! unbounded memory consumption.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021
//! - stable Rust only
//!
//! No nightly features are required.
//!
//! # Integration contract
//!
//! Higher-level code should use this module as:
//!
//! ```text
//! DriftConfig
//!      │
//!      ▼
//! DriftBenchmark::analyze()
//!      │
//!      ▼
//! DriftResult
//! ```
//!
//! `core::observation` should be converted into `DriftSample` by the protocol
//! adapter. The conversion belongs outside this file because this protocol
//! must not assume a particular observation payload.
//!
//! `hardware::calibration::CalibrationSnapshot` may be represented in
//! `DriftSample::metadata`, preserving backend/calibration identity without
//! coupling this module to the hardware implementation.
//!
//! `statistics::outliers` can classify samples before analysis. This module
//! intentionally accepts the classification as data rather than importing a
//! specific outlier implementation, preventing protocol/statistics coupling.
//!
//! # Public API stability
//!
//! The public structures in this file are the protocol boundary. Internal
//! helper functions are private. Future reporting, registry and core-result
//! integration should consume these types rather than requiring changes to the
//! drift mathematics.
//!
//! # No hidden side effects
//!
//! This file contains no logging, no filesystem access, no network access,
//! no process-global state, and no environment-variable reads.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};

// =============================================================================
// Schema and limits
// =============================================================================

/// Stable protocol identifier.
pub const DRIFT_BENCHMARK_ID: &str = "quantum_drift";

/// Current protocol schema version.
pub const DRIFT_SCHEMA_VERSION: u16 = 1;

/// Maximum number of samples accepted by one analysis.
pub const DEFAULT_MAX_SAMPLES: usize = 1_000_000;

/// Maximum number of metadata entries attached to one sample.
pub const DEFAULT_MAX_METADATA_FIELDS: usize = 128;

/// Minimum number of usable observations required for trend analysis.
pub const MIN_TREND_SAMPLES: usize = 3;

/// Minimum number of usable observations required for EWMA/CUSUM.
pub const MIN_CHANGE_DETECTION_SAMPLES: usize = 2;

/// Default baseline window.
pub const DEFAULT_BASELINE_WINDOW: usize = 5;

/// Default EWMA smoothing parameter.
pub const DEFAULT_EWMA_LAMBDA: f64 = 0.2;

/// Default CUSUM reference allowance expressed in baseline standard
/// deviations.
pub const DEFAULT_CUSUM_K: f64 = 0.5;

/// Default CUSUM decision threshold expressed in baseline standard deviations.
pub const DEFAULT_CUSUM_H: f64 = 5.0;

/// Default absolute drift threshold.
pub const DEFAULT_ABSOLUTE_DRIFT_THRESHOLD: f64 = 0.0;

/// Default relative drift threshold.
pub const DEFAULT_RELATIVE_DRIFT_THRESHOLD: f64 = 0.0;

/// Numerical tolerance used when checking values close to zero.
const EPSILON: f64 = 1.0e-15;

// =============================================================================
// Error model
// =============================================================================

/// Errors produced by the drift protocol.
#[derive(Debug, Clone, PartialEq)]
pub enum DriftError {
    /// Metric identifier is empty.
    EmptyMetricId,

    /// Metric identifier contains unsupported characters.
    InvalidMetricId,

    /// Metric value is not finite.
    NonFiniteValue {
        index: usize,
    },

    /// Timestamp is invalid.
    InvalidTimestamp {
        index: usize,
    },

    /// Timestamps are not strictly increasing.
    NonMonotonicTimestamp {
        previous_index: usize,
        current_index: usize,
    },

    /// No samples were supplied.
    EmptySamples,

    /// Too many samples were supplied.
    SampleLimitExceeded {
        requested: usize,
        maximum: usize,
    },

    /// Too many metadata fields were supplied.
    MetadataLimitExceeded {
        requested: usize,
        maximum: usize,
    },

    /// Baseline window is invalid.
    InvalidBaselineWindow {
        value: usize,
    },

    /// EWMA lambda is invalid.
    InvalidEwmaLambda {
        value: f64,
    },

    /// CUSUM parameter is invalid.
    InvalidCusumParameter {
        field: &'static str,
        value: f64,
    },

    /// Drift threshold is invalid.
    InvalidThreshold {
        field: &'static str,
        value: f64,
    },

    /// Insufficient observations for requested analysis.
    InsufficientSamples {
        available: usize,
        required: usize,
        analysis: &'static str,
    },

    /// Numerical computation produced a non-finite value.
    NumericalFailure {
        statistic: &'static str,
    },

    /// Timestamp conversion overflowed.
    TimestampOverflow,

    /// Baseline has zero magnitude and therefore cannot support ordinary
    /// relative-change calculation.
    ZeroBaselineForRelativeChange,

    /// Invalid quality classification.
    InvalidSampleQuality,
}

impl fmt::Display for DriftError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyMetricId => {
                formatter.write_str("drift metric identifier cannot be empty")
            }

            Self::InvalidMetricId => {
                formatter.write_str(
                    "drift metric identifier contains unsupported characters",
                )
            }

            Self::NonFiniteValue { index } => {
                write!(
                    formatter,
                    "drift sample {index} contains a non-finite value"
                )
            }

            Self::InvalidTimestamp { index } => {
                write!(
                    formatter,
                    "drift sample {index} contains an invalid timestamp"
                )
            }

            Self::NonMonotonicTimestamp {
                previous_index,
                current_index,
            } => {
                write!(
                    formatter,
                    "drift timestamps are not strictly increasing: \
                     sample {current_index} follows sample {previous_index}"
                )
            }

            Self::EmptySamples => {
                formatter.write_str("drift analysis requires at least one sample")
            }

            Self::SampleLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "drift sample count {requested} exceeds maximum {maximum}"
                )
            }

            Self::MetadataLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "drift metadata field count {requested} exceeds maximum {maximum}"
                )
            }

            Self::InvalidBaselineWindow { value } => {
                write!(
                    formatter,
                    "drift baseline window must be greater than zero, got {value}"
                )
            }

            Self::InvalidEwmaLambda { value } => {
                write!(
                    formatter,
                    "EWMA lambda must be finite and in (0, 1], got {value}"
                )
            }

            Self::InvalidCusumParameter { field, value } => {
                write!(
                    formatter,
                    "CUSUM parameter '{field}' must be finite and \
                     greater than zero, got {value}"
                )
            }

            Self::InvalidThreshold { field, value } => {
                write!(
                    formatter,
                    "drift threshold '{field}' must be finite and \
                     non-negative, got {value}"
                )
            }

            Self::InsufficientSamples {
                available,
                required,
                analysis,
            } => {
                write!(
                    formatter,
                    "insufficient samples for {analysis}: \
                     available {available}, required {required}"
                )
            }

            Self::NumericalFailure { statistic } => {
                write!(
                    formatter,
                    "drift calculation produced a non-finite {statistic}"
                )
            }

            Self::TimestampOverflow => {
                formatter.write_str("timestamp conversion overflowed")
            }

            Self::ZeroBaselineForRelativeChange => {
                formatter.write_str(
                    "relative change cannot be calculated from a zero baseline",
                )
            }

            Self::InvalidSampleQuality => {
                formatter.write_str("invalid drift sample quality")
            }
        }
    }
}

impl Error for DriftError {}

// =============================================================================
// Metric semantics
// =============================================================================

/// Indicates whether an increase in the metric is normally better or worse.
///
/// This does not affect the raw measured values. It only affects interpretation
/// of the detected drift direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetricDirection {
    /// Higher values normally represent better performance.
    HigherIsBetter,

    /// Lower values normally represent better performance.
    LowerIsBetter,

    /// No universal quality direction exists.
    Neutral,
}

impl Default for MetricDirection {
    fn default() -> Self {
        Self::Neutral
    }
}

impl MetricDirection {
    /// Returns a stable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HigherIsBetter => "higher_is_better",
            Self::LowerIsBetter => "lower_is_better",
            Self::Neutral => "neutral",
        }
    }
}

/// Unit associated with a drift metric.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MetricUnit {
    /// Dimensionless quantity.
    Dimensionless,

    /// Probability or probability-like quantity.
    Probability,

    /// Nanoseconds.
    Nanoseconds,

    /// Microseconds.
    Microseconds,

    /// Milliseconds.
    Milliseconds,

    /// Seconds.
    Seconds,

    /// Hertz.
    Hertz,

    /// Arbitrary user-defined unit.
    Custom(String),
}

impl MetricUnit {
    /// Stable machine-readable identifier.
    pub fn as_str(&self) -> &str {
        match self {
            Self::Dimensionless => "dimensionless",
            Self::Probability => "probability",
            Self::Nanoseconds => "ns",
            Self::Microseconds => "us",
            Self::Milliseconds => "ms",
            Self::Seconds => "s",
            Self::Hertz => "hz",
            Self::Custom(value) => value.as_str(),
        }
    }
}

// =============================================================================
// Sample quality
// =============================================================================

/// Quality classification supplied by execution/statistical layers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SampleQuality {
    /// Valid observation.
    Valid,

    /// Valid numerical observation that an upstream detector flagged as
    /// potentially anomalous.
    Flagged,

    /// Observation should not participate in numerical analysis.
    Invalid,
}

impl SampleQuality {
    /// Returns whether this observation is numerically usable.
    pub const fn is_usable(self, include_flagged: bool) -> bool {
        match self {
            Self::Valid => true,
            Self::Flagged => include_flagged,
            Self::Invalid => false,
        }
    }
}

// =============================================================================
// Timestamp
// =============================================================================

/// Absolute observation timestamp.
///
/// The value is nanoseconds from an externally defined epoch.
///
/// For Unix timestamps, use [`DriftTimestamp::from_unix_nanos`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DriftTimestamp {
    unix_ns: u128,
}

impl DriftTimestamp {
    /// Creates a timestamp from nanoseconds since Unix epoch.
    pub const fn from_unix_nanos(unix_ns: u128) -> Self {
        Self { unix_ns }
    }

    /// Returns the underlying nanosecond timestamp.
    pub const fn as_unix_nanos(self) -> u128 {
        self.unix_ns
    }

    /// Captures the current Unix timestamp.
    ///
    /// This helper is intended for live acquisition adapters. Deterministic
    /// tests and replay should supply explicit timestamps.
    pub fn now() -> Result<Self, DriftError> {
        let duration = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| DriftError::TimestampOverflow)?;

        Ok(Self {
            unix_ns: duration.as_nanos(),
        })
    }

    /// Returns elapsed seconds between two timestamps.
    ///
    /// The caller must provide `later >= earlier`.
    pub fn elapsed_seconds(
        later: Self,
        earlier: Self,
    ) -> Result<f64, DriftError> {
        if later < earlier {
            return Err(DriftError::InvalidTimestamp { index: 0 });
        }

        let delta = later.unix_ns - earlier.unix_ns;
        let seconds = delta as f64 / 1_000_000_000.0;

        if !seconds.is_finite() {
            return Err(DriftError::NumericalFailure {
                statistic: "elapsed_seconds",
            });
        }

        Ok(seconds)
    }
}

// =============================================================================
// Drift sample
// =============================================================================

/// One time-stamped measurement in a drift series.
#[derive(Debug, Clone, PartialEq)]
pub struct DriftSample {
    /// Observation timestamp.
    pub timestamp: DriftTimestamp,

    /// Measured metric value.
    pub value: f64,

    /// Upstream quality classification.
    pub quality: SampleQuality,

    /// Optional uncertainty associated with the observation.
    ///
    /// This value is informational at the drift layer. It is not silently
    /// treated as a weighting factor.
    pub uncertainty: Option<f64>,

    /// Optional calibration identifier.
    pub calibration_id: Option<String>,

    /// Optional backend identifier.
    pub backend_id: Option<String>,

    /// Optional backend version.
    pub backend_version: Option<String>,

    /// Additional deterministic metadata.
    pub metadata: BTreeMap<String, String>,
}

impl DriftSample {
    /// Creates a valid sample.
    pub fn new(timestamp: DriftTimestamp, value: f64) -> Result<Self, DriftError> {
        if !value.is_finite() {
            return Err(DriftError::NonFiniteValue { index: 0 });
        }

        Ok(Self {
            timestamp,
            value,
            quality: SampleQuality::Valid,
            uncertainty: None,
            calibration_id: None,
            backend_id: None,
            backend_version: None,
            metadata: BTreeMap::new(),
        })
    }

    /// Changes the quality classification.
    #[must_use]
    pub const fn with_quality(mut self, quality: SampleQuality) -> Self {
        self.quality = quality;
        self
    }

    /// Attaches uncertainty.
    pub fn with_uncertainty(
        mut self,
        uncertainty: f64,
    ) -> Result<Self, DriftError> {
        if !uncertainty.is_finite() || uncertainty < 0.0 {
            return Err(DriftError::NonFiniteValue { index: 0 });
        }

        self.uncertainty = Some(uncertainty);
        Ok(self)
    }

    /// Attaches calibration identity.
    #[must_use]
    pub fn with_calibration_id(mut self, value: impl Into<String>) -> Self {
        self.calibration_id = Some(value.into());
        self
    }

    /// Attaches backend identity.
    #[must_use]
    pub fn with_backend_id(mut self, value: impl Into<String>) -> Self {
        self.backend_id = Some(value.into());
        self
    }

    /// Attaches backend version.
    #[must_use]
    pub fn with_backend_version(mut self, value: impl Into<String>) -> Self {
        self.backend_version = Some(value.into());
        self
    }

    /// Adds deterministic metadata.
    pub fn with_metadata(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Self, DriftError> {
        self.metadata.insert(key.into(), value.into());

        if self.metadata.len() > DEFAULT_MAX_METADATA_FIELDS {
            return Err(DriftError::MetadataLimitExceeded {
                requested: self.metadata.len(),
                maximum: DEFAULT_MAX_METADATA_FIELDS,
            });
        }

        Ok(self)
    }
}

// =============================================================================
// Baseline
// =============================================================================

/// Strategy used to establish the reference behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BaselineStrategy {
    /// Use the first valid/eligible observation.
    FirstSample,

    /// Use the arithmetic mean of the first N eligible observations.
    InitialMean,

    /// Use the median of the first N eligible observations.
    InitialMedian,
}

impl Default for BaselineStrategy {
    fn default() -> Self {
        Self::InitialMean
    }
}

// =============================================================================
// Drift configuration
// =============================================================================

/// Production configuration for drift analysis.
#[derive(Debug, Clone, PartialEq)]
pub struct DriftConfig {
    /// Stable metric identifier.
    pub metric_id: String,

    /// Human-readable unit.
    pub unit: MetricUnit,

    /// Interpretation of increasing/decreasing metric values.
    pub direction: MetricDirection,

    /// Baseline construction strategy.
    pub baseline_strategy: BaselineStrategy,

    /// Number of initial samples used by initial mean/median baselines.
    pub baseline_window: usize,

    /// Whether upstream-flagged samples participate in calculations.
    ///
    /// They are always retained in the result.
    pub include_flagged_samples: bool,

    /// Maximum number of samples accepted.
    pub max_samples: usize,

    /// EWMA smoothing parameter.
    pub ewma_lambda: f64,

    /// CUSUM reference allowance in baseline standard deviations.
    pub cusum_k: f64,

    /// CUSUM decision threshold in baseline standard deviations.
    pub cusum_h: f64,

    /// Absolute change threshold.
    ///
    /// Zero disables threshold-based absolute alerts.
    pub absolute_drift_threshold: f64,

    /// Relative change threshold.
    ///
    /// Zero disables threshold-based relative alerts.
    pub relative_drift_threshold: f64,

    /// Minimum absolute slope required for a trend alert.
    ///
    /// Zero disables slope thresholding.
    pub slope_threshold: f64,
}

impl Default for DriftConfig {
    fn default() -> Self {
        Self {
            metric_id: "unknown".to_string(),
            unit: MetricUnit::Dimensionless,
            direction: MetricDirection::Neutral,
            baseline_strategy: BaselineStrategy::InitialMean,
            baseline_window: DEFAULT_BASELINE_WINDOW,
            include_flagged_samples: false,
            max_samples: DEFAULT_MAX_SAMPLES,
            ewma_lambda: DEFAULT_EWMA_LAMBDA,
            cusum_k: DEFAULT_CUSUM_K,
            cusum_h: DEFAULT_CUSUM_H,
            absolute_drift_threshold: DEFAULT_ABSOLUTE_DRIFT_THRESHOLD,
            relative_drift_threshold: DEFAULT_RELATIVE_DRIFT_THRESHOLD,
            slope_threshold: 0.0,
        }
    }
}

impl DriftConfig {
    /// Creates a production configuration for one metric.
    pub fn production(
        metric_id: impl Into<String>,
        unit: MetricUnit,
        direction: MetricDirection,
    ) -> Result<Self, DriftError> {
        let config = Self {
            metric_id: metric_id.into(),
            unit,
            direction,
            ..Self::default()
        };

        config.validate()?;

        Ok(config)
    }

    /// Validates the complete configuration.
    pub fn validate(&self) -> Result<(), DriftError> {
        validate_metric_id(&self.metric_id)?;

        if self.baseline_window == 0 {
            return Err(DriftError::InvalidBaselineWindow {
                value: self.baseline_window,
            });
        }

        if self.max_samples == 0 {
            return Err(DriftError::SampleLimitExceeded {
                requested: 0,
                maximum: 0,
            });
        }

        if !self.ewma_lambda.is_finite()
            || self.ewma_lambda <= 0.0
            || self.ewma_lambda > 1.0
        {
            return Err(DriftError::InvalidEwmaLambda {
                value: self.ewma_lambda,
            });
        }

        if !self.cusum_k.is_finite() || self.cusum_k <= 0.0 {
            return Err(DriftError::InvalidCusumParameter {
                field: "cusum_k",
                value: self.cusum_k,
            });
        }

        if !self.cusum_h.is_finite() || self.cusum_h <= 0.0 {
            return Err(DriftError::InvalidCusumParameter {
                field: "cusum_h",
                value: self.cusum_h,
            });
        }

        if !self.absolute_drift_threshold.is_finite()
            || self.absolute_drift_threshold < 0.0
        {
            return Err(DriftError::InvalidThreshold {
                field: "absolute_drift_threshold",
                value: self.absolute_drift_threshold,
            });
        }

        if !self.relative_drift_threshold.is_finite()
            || self.relative_drift_threshold < 0.0
        {
            return Err(DriftError::InvalidThreshold {
                field: "relative_drift_threshold",
                value: self.relative_drift_threshold,
            });
        }

        if !self.slope_threshold.is_finite() || self.slope_threshold < 0.0 {
            return Err(DriftError::InvalidThreshold {
                field: "slope_threshold",
                value: self.slope_threshold,
            });
        }

        Ok(())
    }
}

// =============================================================================
// Trend result
// =============================================================================

/// Linear trend calculated against elapsed time.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LinearTrend {
    /// Number of samples used.
    pub sample_count: usize,

    /// Slope in metric-units per second.
    pub slope_per_second: f64,

    /// Intercept relative to the normalized time origin.
    pub intercept: f64,

    /// Coefficient of determination.
    ///
    /// `None` when variance in either the independent or dependent variable
    /// is zero.
    pub r_squared: Option<f64>,

    /// Total time span in seconds.
    pub duration_seconds: f64,
}

impl LinearTrend {
    /// Returns whether the trend is increasing.
    pub fn is_increasing(self) -> bool {
        self.slope_per_second > EPSILON
    }

    /// Returns whether the trend is decreasing.
    pub fn is_decreasing(self) -> bool {
        self.slope_per_second < -EPSILON
    }

    /// Returns whether the trend is effectively flat.
    pub fn is_flat(self) -> bool {
        self.slope_per_second.abs() <= EPSILON
    }
}

// =============================================================================
// EWMA result
// =============================================================================

/// EWMA diagnostic result.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EwmaResult {
    /// Smoothing factor.
    pub lambda: f64,

    /// Initial value.
    pub initial_value: f64,

    /// Final EWMA value.
    pub final_value: f64,

    /// Maximum positive excursion from the initial value.
    pub maximum_positive_excursion: f64,

    /// Maximum negative excursion from the initial value.
    pub maximum_negative_excursion: f64,
}

// =============================================================================
// CUSUM result
// =============================================================================

/// CUSUM diagnostic result.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CusumResult {
    /// Reference allowance.
    pub k: f64,

    /// Decision threshold.
    pub h: f64,

    /// Maximum positive cumulative statistic.
    pub maximum_positive: f64,

    /// Maximum negative cumulative statistic.
    pub maximum_negative: f64,

    /// First sample index producing a positive alarm.
    pub first_positive_alarm: Option<usize>,

    /// First sample index producing a negative alarm.
    pub first_negative_alarm: Option<usize>,
}

impl CusumResult {
    /// Returns whether any CUSUM alarm was triggered.
    pub fn alarmed(self) -> bool {
        self.first_positive_alarm.is_some()
            || self.first_negative_alarm.is_some()
    }
}

// =============================================================================
// Drift direction
// =============================================================================

/// Direction of the observed metric change.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriftDirection {
    /// No meaningful direction detected.
    Stable,

    /// Metric increased.
    Increasing,

    /// Metric decreased.
    Decreasing,

    /// Increase and decrease are both substantial or inconsistent.
    Mixed,
}

impl DriftDirection {
    /// Returns a stable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Increasing => "increasing",
            Self::Decreasing => "decreasing",
            Self::Mixed => "mixed",
        }
    }
}

// =============================================================================
// Drift severity
// =============================================================================

/// Severity of detected drift.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DriftSeverity {
    /// No drift criterion triggered.
    None,

    /// Small measurable movement.
    Informational,

    /// Configured drift threshold triggered.
    Warning,

    /// Multiple or strong detectors triggered.
    Critical,
}

impl DriftSeverity {
    /// Stable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Informational => "informational",
            Self::Warning => "warning",
            Self::Critical => "critical",
        }
    }
}

// =============================================================================
// Drift event
// =============================================================================

/// A detected change event.
#[derive(Debug, Clone, PartialEq)]
pub struct DriftEvent {
    /// First sample index associated with the event.
    pub sample_index: usize,

    /// Event timestamp.
    pub timestamp: DriftTimestamp,

    /// Detector that triggered.
    pub detector: String,

    /// Direction.
    pub direction: DriftDirection,

    /// Severity.
    pub severity: DriftSeverity,

    /// Value at the event.
    pub value: f64,

    /// Baseline value.
    pub baseline: f64,

    /// Absolute difference from baseline.
    pub absolute_change: f64,

    /// Relative difference from baseline, when defined.
    pub relative_change: Option<f64>,
}

// =============================================================================
// Drift result
// =============================================================================

/// Complete result of one drift analysis.
#[derive(Debug, Clone, PartialEq)]
pub struct DriftResult {
    /// Protocol schema version.
    pub schema_version: u16,

    /// Benchmark identifier.
    pub benchmark_id: &'static str,

    /// Metric identifier.
    pub metric_id: String,

    /// Metric unit.
    pub unit: MetricUnit,

    /// Metric quality direction.
    pub metric_direction: MetricDirection,

    /// Number of input samples.
    pub total_samples: usize,

    /// Number of valid samples used.
    pub valid_samples: usize,

    /// Number of flagged samples.
    pub flagged_samples: usize,

    /// Number of invalid samples.
    pub invalid_samples: usize,

    /// Baseline value.
    pub baseline: f64,

    /// Latest usable value.
    pub latest_value: f64,

    /// Absolute change from baseline to latest value.
    pub absolute_change: f64,

    /// Relative change from baseline to latest value.
    ///
    /// `None` when baseline is zero.
    pub relative_change: Option<f64>,

    /// Linear trend.
    pub trend: Option<LinearTrend>,

    /// EWMA analysis.
    pub ewma: Option<EwmaResult>,

    /// CUSUM analysis.
    pub cusum: Option<CusumResult>,

    /// Largest absolute excursion from baseline.
    pub maximum_absolute_excursion: f64,

    /// Largest relative excursion from baseline.
    pub maximum_relative_excursion: Option<f64>,

    /// Overall direction.
    pub direction: DriftDirection,

    /// Overall severity.
    pub severity: DriftSeverity,

    /// Whether configured drift criteria were triggered.
    pub drift_detected: bool,

    /// Detected events.
    pub events: Vec<DriftEvent>,

    /// Backend identities observed in the series.
    pub backend_ids: Vec<String>,

    /// Calibration identities observed in the series.
    pub calibration_ids: Vec<String>,

    /// Warnings that do not invalidate the analysis.
    pub warnings: Vec<String>,
}

impl DriftResult {
    /// Returns true when the metric is trending upward.
    pub fn is_increasing(&self) -> bool {
        self.direction == DriftDirection::Increasing
    }

    /// Returns true when the metric is trending downward.
    pub fn is_decreasing(&self) -> bool {
        self.direction == DriftDirection::Decreasing
    }

    /// Returns true when at least one detector generated an event.
    pub fn has_events(&self) -> bool {
        !self.events.is_empty()
    }

    /// Returns whether the observed drift represents degradation according to
    /// the metric's direction semantics.
    pub fn is_degradation(&self) -> bool {
        match self.metric_direction {
            MetricDirection::HigherIsBetter => {
                self.direction == DriftDirection::Decreasing
            }

            MetricDirection::LowerIsBetter => {
                self.direction == DriftDirection::Increasing
            }

            MetricDirection::Neutral => false,

        }
    }

    /// Returns whether the observed drift represents improvement according to
    /// the metric's direction semantics.
    pub fn is_improvement(&self) -> bool {
        match self.metric_direction {
            MetricDirection::HigherIsBetter => {
                self.direction == DriftDirection::Increasing
            }

            MetricDirection::LowerIsBetter => {
                self.direction == DriftDirection::Decreasing
            }

            MetricDirection::Neutral => false,
        }
    }
}

// =============================================================================
// Benchmark
// =============================================================================

/// Stateless production drift benchmark.
#[derive(Debug, Clone, PartialEq)]
pub struct DriftBenchmark {
    /// Immutable benchmark configuration.
    pub config: DriftConfig,
}

impl DriftBenchmark {
    /// Creates a validated drift benchmark.
    pub fn new(config: DriftConfig) -> Result<Self, DriftError> {
        config.validate()?;

        Ok(Self { config })
    }

    /// Analyzes a complete ordered time series.
    pub fn analyze(
        &self,
        samples: &[DriftSample],
    ) -> Result<DriftResult, DriftError> {
        validate_samples(samples, self.config.max_samples)?;

        let usable = collect_usable_samples(
            samples,
            self.config.include_flagged_samples,
        );

        if usable.is_empty() {
            return Err(DriftError::InsufficientSamples {
                available: 0,
                required: 1,
                analysis: "drift analysis",
            });
        }

        let baseline = calculate_baseline(
            &usable,
            self.config.baseline_strategy,
            self.config.baseline_window,
        )?;

        let latest = usable
            .last()
            .ok_or(DriftError::EmptySamples)?;

        let absolute_change = latest.value - baseline;

        let relative_change =
            calculate_relative_change(baseline, latest.value);

        let maximum_absolute_excursion =
            maximum_absolute_excursion(&usable, baseline);

        let maximum_relative_excursion =
            maximum_relative_excursion(&usable, baseline);

        let trend = if usable.len() >= MIN_TREND_SAMPLES {
            Some(calculate_linear_trend(&usable)?)
        } else {
            None
        };

        let ewma = if usable.len() >= MIN_CHANGE_DETECTION_SAMPLES {
            Some(calculate_ewma(
                &usable,
                self.config.ewma_lambda,
            )?)
        } else {
            None
        };

        let baseline_stddev = calculate_standard_deviation(
            &usable
                .iter()
                .take(self.config.baseline_window)
                .map(|sample| sample.value)
                .collect::<Vec<_>>(),
        )?;

        let cusum = if usable.len() >= MIN_CHANGE_DETECTION_SAMPLES {
            Some(calculate_cusum(
                &usable,
                baseline,
                baseline_stddev,
                self.config.cusum_k,
                self.config.cusum_h,
            )?)
        } else {
            None
        };

        let direction = determine_direction(
            absolute_change,
            trend.as_ref(),
            maximum_absolute_excursion,
        );

        let mut events = Vec::new();

        append_threshold_events(
            &usable,
            baseline,
            self.config.absolute_drift_threshold,
            self.config.relative_drift_threshold,
            &mut events,
        );

        if let Some(cusum_result) = cusum {
            append_cusum_events(
                &usable,
                baseline,
                cusum_result,
                &mut events,
            );
        }

        if let Some(trend_result) = trend {
            if self.config.slope_threshold > 0.0
                && trend_result.slope_per_second.abs()
                    >= self.config.slope_threshold
            {
                let trend_direction = if trend_result.slope_per_second > 0.0 {
                    DriftDirection::Increasing
                } else {
                    DriftDirection::Decreasing
                };

                events.push(DriftEvent {
                    sample_index: samples.len().saturating_sub(1),
                    timestamp: latest.timestamp,
                    detector: "linear_trend".to_string(),
                    direction: trend_direction,
                    severity: DriftSeverity::Warning,
                    value: latest.value,
                    baseline,
                    absolute_change,
                    relative_change,
                });
            }
        }

        let drift_detected = !events.is_empty();

        let severity = calculate_severity(
            &events,
            maximum_absolute_excursion,
            self.config.absolute_drift_threshold,
            self.config.relative_drift_threshold,
        );

        let mut warnings = Vec::new();

        if samples.len() < MIN_TREND_SAMPLES {
            warnings.push(
                "fewer than three usable samples: linear trend \
                 analysis is unavailable"
                    .to_string(),
            );
        }

        if baseline.abs() <= EPSILON {
            warnings.push(
                "baseline is approximately zero; relative drift \
                 cannot be interpreted"
                    .to_string(),
            );
        }

        if baseline_stddev <= EPSILON {
            warnings.push(
                "baseline dispersion is approximately zero; CUSUM \
                 interpretation is limited"
                    .to_string(),
            );
        }

        if flagged_sample_count(samples) > 0
            && !self.config.include_flagged_samples
        {
            warnings.push(
                "upstream-flagged observations were retained but \
                 excluded from numerical analysis"
                    .to_string(),
            );
        }

        let backend_ids = collect_backend_ids(samples);
        let calibration_ids = collect_calibration_ids(samples);

        Ok(DriftResult {
            schema_version: DRIFT_SCHEMA_VERSION,
            benchmark_id: DRIFT_BENCHMARK_ID,
            metric_id: self.config.metric_id.clone(),
            unit: self.config.unit.clone(),
            metric_direction: self.config.direction,
            total_samples: samples.len(),
            valid_samples: samples
                .iter()
                .filter(|sample| {
                    sample.quality == SampleQuality::Valid
                        || (sample.quality == SampleQuality::Flagged
                            && self.config.include_flagged_samples)
                })
                .count(),
            flagged_samples: flagged_sample_count(samples),
            invalid_samples: samples
                .iter()
                .filter(|sample| sample.quality == SampleQuality::Invalid)
                .count(),
            baseline,
            latest_value: latest.value,
            absolute_change,
            relative_change,
            trend,
            ewma,
            cusum,
            maximum_absolute_excursion,
            maximum_relative_excursion,
            direction,
            severity,
            drift_detected,
            events,
            backend_ids,
            calibration_ids,
            warnings,
        })
    }
}

/// Convenience function for production-default drift analysis.
pub fn analyze_drift(
    config: DriftConfig,
    samples: &[DriftSample],
) -> Result<DriftResult, DriftError> {
    DriftBenchmark::new(config)?.analyze(samples)
}

// =============================================================================
// Validation
// =============================================================================

fn validate_metric_id(metric_id: &str) -> Result<(), DriftError> {
    if metric_id.trim().is_empty() {
        return Err(DriftError::EmptyMetricId);
    }

    for character in metric_id.chars() {
        if !(character.is_ascii_alphanumeric()
            || character == '_'
            || character == '-'
            || character == '.')
        {
            return Err(DriftError::InvalidMetricId);
        }
    }

    Ok(())
}

fn validate_samples(
    samples: &[DriftSample],
    maximum: usize,
) -> Result<(), DriftError> {
    if samples.is_empty() {
        return Err(DriftError::EmptySamples);
    }

    if samples.len() > maximum {
        return Err(DriftError::SampleLimitExceeded {
            requested: samples.len(),
            maximum,
        });
    }

    for (index, sample) in samples.iter().enumerate() {
        if !sample.value.is_finite() {
            return Err(DriftError::NonFiniteValue { index });
        }

        if let Some(uncertainty) = sample.uncertainty {
            if !uncertainty.is_finite() || uncertainty < 0.0 {
                return Err(DriftError::NonFiniteValue { index });
            }
        }

        if sample.metadata.len() > DEFAULT_MAX_METADATA_FIELDS {
            return Err(DriftError::MetadataLimitExceeded {
                requested: sample.metadata.len(),
                maximum: DEFAULT_MAX_METADATA_FIELDS,
            });
        }

        if index > 0 {
            let previous = samples[index - 1].timestamp;
            let current = sample.timestamp;

            if current <= previous {
                return Err(DriftError::NonMonotonicTimestamp {
                    previous_index: index - 1,
                    current_index: index,
                });
            }
        }
    }

    Ok(())
}

fn collect_usable_samples<'a>(
    samples: &'a [DriftSample],
    include_flagged: bool,
) -> Vec<&'a DriftSample> {
    samples
        .iter()
        .filter(|sample| sample.quality.is_usable(include_flagged))
        .collect()
}

fn flagged_sample_count(samples: &[DriftSample]) -> usize {
    samples
        .iter()
        .filter(|sample| sample.quality == SampleQuality::Flagged)
        .count()
}

// =============================================================================
// Baseline calculations
// =============================================================================

fn calculate_baseline(
    samples: &[&DriftSample],
    strategy: BaselineStrategy,
    window: usize,
) -> Result<f64, DriftError> {
    if samples.is_empty() {
        return Err(DriftError::EmptySamples);
    }

    let count = match strategy {
        BaselineStrategy::FirstSample => 1,
        BaselineStrategy::InitialMean
        | BaselineStrategy::InitialMedian => samples.len().min(window),
    };

    if count == 0 {
        return Err(DriftError::InvalidBaselineWindow {
            value: window,
        });
    }

    let values: Vec<f64> = samples
        .iter()
        .take(count)
        .map(|sample| sample.value)
        .collect();

    let result = match strategy {
        BaselineStrategy::FirstSample => values[0],

        BaselineStrategy::InitialMean => {
            let sum = values.iter().copied().sum::<f64>();
            sum / values.len() as f64
        }

        BaselineStrategy::InitialMedian => {
            median(&values)?
        }
    };

    if !result.is_finite() {
        return Err(DriftError::NumericalFailure {
            statistic: "baseline",
        });
    }

    Ok(result)
}

// =============================================================================
// Change calculations
// =============================================================================

fn calculate_relative_change(
    baseline: f64,
    value: f64,
) -> Option<f64> {
    if baseline.abs() <= EPSILON {
        None
    } else {
        Some((value - baseline) / baseline)
    }
}

fn maximum_absolute_excursion(
    samples: &[&DriftSample],
    baseline: f64,
) -> f64 {
    samples
        .iter()
        .map(|sample| (sample.value - baseline).abs())
        .fold(0.0, f64::max)
}

fn maximum_relative_excursion(
    samples: &[&DriftSample],
    baseline: f64,
) -> Option<f64> {
    if baseline.abs() <= EPSILON {
        return None;
    }

    Some(
        samples
            .iter()
            .map(|sample| {
                ((sample.value - baseline) / baseline).abs()
            })
            .fold(0.0, f64::max),
    )
}

// =============================================================================
// Linear regression
// =============================================================================

fn calculate_linear_trend(
    samples: &[&DriftSample],
) -> Result<LinearTrend, DriftError> {
    if samples.len() < MIN_TREND_SAMPLES {
        return Err(DriftError::InsufficientSamples {
            available: samples.len(),
            required: MIN_TREND_SAMPLES,
            analysis: "linear trend",
        });
    }

    let origin = samples[0].timestamp;

    let points: Vec<(f64, f64)> = samples
        .iter()
        .map(|sample| {
            Ok((
                DriftTimestamp::elapsed_seconds(
                    sample.timestamp,
                    origin,
                )?,
                sample.value,
            ))
        })
        .collect::<Result<Vec<_>, DriftError>>()?;

    let n = points.len() as f64;

    let sum_x = points.iter().map(|point| point.0).sum::<f64>();
    let sum_y = points.iter().map(|point| point.1).sum::<f64>();

    let mean_x = sum_x / n;
    let mean_y = sum_y / n;

    let mut numerator = 0.0;
    let mut denominator = 0.0;

    for (x, y) in &points {
        let dx = *x - mean_x;
        let dy = *y - mean_y;

        numerator += dx * dy;
        denominator += dx * dx;
    }

    let slope = if denominator <= EPSILON {
        0.0
    } else {
        numerator / denominator
    };

    let intercept = mean_y - slope * mean_x;

    let mut ss_total = 0.0;
    let mut ss_residual = 0.0;

    for (x, y) in &points {
        let predicted = intercept + slope * *x;

        ss_total += (*y - mean_y).powi(2);
        ss_residual += (*y - predicted).powi(2);
    }

    let r_squared = if ss_total <= EPSILON {
        None
    } else {
        Some((1.0 - ss_residual / ss_total).clamp(0.0, 1.0))
    };

    let duration_seconds = points
        .last()
        .map(|point| point.0)
        .unwrap_or(0.0);

    if !slope.is_finite()
        || !intercept.is_finite()
        || !duration_seconds.is_finite()
    {
        return Err(DriftError::NumericalFailure {
            statistic: "linear_trend",
        });
    }

    Ok(LinearTrend {
        sample_count: samples.len(),
        slope_per_second: slope,
        intercept,
        r_squared,
        duration_seconds,
    })
}

// =============================================================================
// Standard deviation
// =============================================================================

fn calculate_standard_deviation(
    values: &[f64],
) -> Result<f64, DriftError> {
    if values.is_empty() {
        return Err(DriftError::InsufficientSamples {
            available: 0,
            required: 1,
            analysis: "standard deviation",
        });
    }

    let mean =
        values.iter().copied().sum::<f64>() / values.len() as f64;

    let variance = values
        .iter()
        .map(|value| (*value - mean).powi(2))
        .sum::<f64>()
        / values.len() as f64;

    if !variance.is_finite() || variance < 0.0 {
        return Err(DriftError::NumericalFailure {
            statistic: "variance",
        });
    }

    let result = variance.sqrt();

    if !result.is_finite() {
        return Err(DriftError::NumericalFailure {
            statistic: "standard_deviation",
        });
    }

    Ok(result)
}

// =============================================================================
// EWMA
// =============================================================================

fn calculate_ewma(
    samples: &[&DriftSample],
    lambda: f64,
) -> Result<EwmaResult, DriftError> {
    if samples.len() < MIN_CHANGE_DETECTION_SAMPLES {
        return Err(DriftError::InsufficientSamples {
            available: samples.len(),
            required: MIN_CHANGE_DETECTION_SAMPLES,
            analysis: "EWMA",
        });
    }

    let mut ewma = samples[0].value;
    let initial = ewma;

    let mut max_positive = 0.0;
    let mut max_negative = 0.0;

    for sample in samples.iter().skip(1) {
        ewma =
            lambda * sample.value + (1.0 - lambda) * ewma;

        let excursion = ewma - initial;

        if excursion > max_positive {
            max_positive = excursion;
        }

        if excursion < max_negative {
            max_negative = excursion;
        }
    }

    if !ewma.is_finite() {
        return Err(DriftError::NumericalFailure {
            statistic: "EWMA",
        });
    }

    Ok(EwmaResult {
        lambda,
        initial_value: initial,
        final_value: ewma,
        maximum_positive_excursion: max_positive,
        maximum_negative_excursion: max_negative,
    })
}

// =============================================================================
// CUSUM
// =============================================================================

fn calculate_cusum(
    samples: &[&DriftSample],
    baseline: f64,
    baseline_stddev: f64,
    k: f64,
    h: f64,
) -> Result<CusumResult, DriftError> {
    if samples.len() < MIN_CHANGE_DETECTION_SAMPLES {
        return Err(DriftError::InsufficientSamples {
            available: samples.len(),
            required: MIN_CHANGE_DETECTION_SAMPLES,
            analysis: "CUSUM",
        });
    }

    // If baseline variance is zero, a standardized CUSUM cannot be
    // interpreted safely. Return a non-alarming diagnostic result instead of
    // dividing by zero.
    if baseline_stddev <= EPSILON {
        return Ok(CusumResult {
            k,
            h,
            maximum_positive: 0.0,
            maximum_negative: 0.0,
            first_positive_alarm: None,
            first_negative_alarm: None,
        });
    }

    let scale = baseline_stddev;

    let mut positive = 0.0;
    let mut negative = 0.0;

    let mut maximum_positive = 0.0;
    let mut maximum_negative = 0.0;

    let mut first_positive_alarm = None;
    let mut first_negative_alarm = None;

    for (index, sample) in samples.iter().enumerate().skip(1) {
        let standardized =
            (sample.value - baseline) / scale;

        positive = (positive + standardized - k).max(0.0);
        negative = (negative - standardized - k).max(0.0);

        maximum_positive =
            maximum_positive.max(positive);

        maximum_negative =
            maximum_negative.max(negative);

        if positive >= h && first_positive_alarm.is_none() {
            first_positive_alarm = Some(index);
        }

        if negative >= h && first_negative_alarm.is_none() {
            first_negative_alarm = Some(index);
        }
    }

    if !maximum_positive.is_finite()
        || !maximum_negative.is_finite()
    {
        return Err(DriftError::NumericalFailure {
            statistic: "CUSUM",
        });
    }

    Ok(CusumResult {
        k,
        h,
        maximum_positive,
        maximum_negative,
        first_positive_alarm,
        first_negative_alarm,
    })
}

// =============================================================================
// Direction
// =============================================================================

fn determine_direction(
    latest_change: f64,
    trend: Option<&LinearTrend>,
    maximum_excursion: f64,
) -> DriftDirection {
    let mut increasing = latest_change > EPSILON;
    let mut decreasing = latest_change < -EPSILON;

    if let Some(trend) = trend {
        increasing |= trend.is_increasing();
        decreasing |= trend.is_decreasing();
    }

    if maximum_excursion <= EPSILON {
        return DriftDirection::Stable;
    }

    match (increasing, decreasing) {
        (true, false) => DriftDirection::Increasing,
        (false, true) => DriftDirection::Decreasing,
        (true, true) => DriftDirection::Mixed,
        (false, false) => DriftDirection::Stable,
    }
}

// =============================================================================
// Threshold events
// =============================================================================

fn append_threshold_events(
    samples: &[&DriftSample],
    baseline: f64,
    absolute_threshold: f64,
    relative_threshold: f64,
    events: &mut Vec<DriftEvent>,
) {
    for (index, sample) in samples.iter().enumerate() {
        let absolute_change =
            sample.value - baseline;

        let absolute_trigger =
            absolute_threshold > 0.0
                && absolute_change.abs() >= absolute_threshold;

        let relative_change =
            calculate_relative_change(baseline, sample.value);

        let relative_trigger =
            relative_threshold > 0.0
                && relative_change
                    .map(|value| value.abs() >= relative_threshold)
                    .unwrap_or(false);

        if !absolute_trigger && !relative_trigger {
            continue;
        }

        let direction =
            if absolute_change > EPSILON {
                DriftDirection::Increasing
            } else if absolute_change < -EPSILON {
                DriftDirection::Decreasing
            } else {
                DriftDirection::Stable
            };

        let detector = match (absolute_trigger, relative_trigger) {
            (true, true) => "absolute_and_relative_threshold",
            (true, false) => "absolute_threshold",
            (false, true) => "relative_threshold",
            (false, false) => "threshold",
        };

        events.push(DriftEvent {
            sample_index: index,
            timestamp: sample.timestamp,
            detector: detector.to_string(),
            direction,
            severity: DriftSeverity::Warning,
            value: sample.value,
            baseline,
            absolute_change,
            relative_change,
        });
    }
}

// =============================================================================
// CUSUM events
// =============================================================================

fn append_cusum_events(
    samples: &[&DriftSample],
    baseline: f64,
    cusum: CusumResult,
    events: &mut Vec<DriftEvent>,
) {
    if let Some(index) = cusum.first_positive_alarm {
        if let Some(sample) = samples.get(index) {
            let absolute_change =
                sample.value - baseline;

            events.push(DriftEvent {
                sample_index: index,
                timestamp: sample.timestamp,
                detector: "cusum_positive".to_string(),
                direction: DriftDirection::Increasing,
                severity: DriftSeverity::Critical,
                value: sample.value,
                baseline,
                absolute_change,
                relative_change: calculate_relative_change(
                    baseline,
                    sample.value,
                ),
            });
        }
    }

    if let Some(index) = cusum.first_negative_alarm {
        if let Some(sample) = samples.get(index) {
            let absolute_change =
                sample.value - baseline;

            events.push(DriftEvent {
                sample_index: index,
                timestamp: sample.timestamp,
                detector: "cusum_negative".to_string(),
                direction: DriftDirection::Decreasing,
                severity: DriftSeverity::Critical,
                value: sample.value,
                baseline,
                absolute_change,
                relative_change: calculate_relative_change(
                    baseline,
                    sample.value,
                ),
            });
        }
    }
}

// =============================================================================
// Severity
// =============================================================================

fn calculate_severity(
    events: &[DriftEvent],
    maximum_absolute_excursion: f64,
    absolute_threshold: f64,
    relative_threshold: f64,
) -> DriftSeverity {
    if events.is_empty() {
        if maximum_absolute_excursion > EPSILON {
            return DriftSeverity::Informational;
        }

        return DriftSeverity::None;
    }

    if events.iter().any(|event| {
        event.severity == DriftSeverity::Critical
    }) {
        return DriftSeverity::Critical;
    }

    let strong_absolute = absolute_threshold > 0.0
        && maximum_absolute_excursion
            >= 2.0 * absolute_threshold;

    let strong_relative = if relative_threshold > 0.0 {
        events.iter().any(|event| {
            event.relative_change
                .map(|change| change.abs() >= 2.0 * relative_threshold)
                .unwrap_or(false)
        })
    } else {
        false
    };

    if strong_absolute || strong_relative {
        DriftSeverity::Critical
    } else {
        DriftSeverity::Warning
    }
}

// =============================================================================
// Metadata extraction
// =============================================================================

fn collect_backend_ids(
    samples: &[DriftSample],
) -> Vec<String> {
    let mut values = BTreeMap::<String, ()>::new();

    for sample in samples {
        if let Some(value) = &sample.backend_id {
            values.insert(value.clone(), ());
        }
    }

    values.into_keys().collect()
}

fn collect_calibration_ids(
    samples: &[DriftSample],
) -> Vec<String> {
    let mut values = BTreeMap::<String, ()>::new();

    for sample in samples {
        if let Some(value) = &sample.calibration_id {
            values.insert(value.clone(), ());
        }
    }

    values.into_keys().collect()
}

// =============================================================================
// Median
// =============================================================================

fn median(values: &[f64]) -> Result<f64, DriftError> {
    if values.is_empty() {
        return Err(DriftError::InsufficientSamples {
            available: 0,
            required: 1,
            analysis: "median",
        });
    }

    let mut sorted = values.to_vec();

    sorted.sort_by(|left, right| {
        left.partial_cmp(right)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    let middle = sorted.len() / 2;

    let result = if sorted.len() % 2 == 0 {
        (sorted[middle - 1] + sorted[middle]) / 2.0
    } else {
        sorted[middle]
    };

    if !result.is_finite() {
        return Err(DriftError::NumericalFailure {
            statistic: "median",
        });
    }

    Ok(result)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn timestamp(seconds: u64) -> DriftTimestamp {
        DriftTimestamp::from_unix_nanos(
            seconds as u128 * 1_000_000_000,
        )
    }

    fn sample(
        seconds: u64,
        value: f64,
    ) -> DriftSample {
        DriftSample::new(timestamp(seconds), value)
            .expect("test sample must be valid")
    }

    fn production_config() -> DriftConfig {
        DriftConfig {
            metric_id: "gate_error_rate".to_string(),
            unit: MetricUnit::Probability,
            direction: MetricDirection::LowerIsBetter,
            baseline_strategy: BaselineStrategy::InitialMean,
            baseline_window: 3,
            include_flagged_samples: false,
            max_samples: 100,
            ewma_lambda: 0.2,
            cusum_k: 0.5,
            cusum_h: 5.0,
            absolute_drift_threshold: 0.01,
            relative_drift_threshold: 0.10,
            slope_threshold: 0.0,
        }
    }

    #[test]
    fn configuration_validates() {
        let config = production_config();

        assert!(config.validate().is_ok());
    }

    #[test]
    fn empty_metric_id_is_rejected() {
        let mut config = production_config();
        config.metric_id = String::new();

        assert_eq!(
            config.validate(),
            Err(DriftError::EmptyMetricId)
        );
    }

    #[test]
    fn non_monotonic_timestamps_are_rejected() {
        let config = production_config();

        let samples = vec![
            sample(2, 0.1),
            sample(1, 0.2),
        ];

        let result =
            DriftBenchmark::new(config)
                .expect("configuration must be valid")
                .analyze(&samples);

        assert!(matches!(
            result,
            Err(DriftError::NonMonotonicTimestamp { .. })
        ));
    }

    #[test]
    fn stable_series_has_no_threshold_drift() {
        let mut config = production_config();

        config.absolute_drift_threshold = 0.02;
        config.relative_drift_threshold = 0.20;

        let samples = vec![
            sample(1, 0.01),
            sample(2, 0.0101),
            sample(3, 0.0099),
            sample(4, 0.0100),
            sample(5, 0.0101),
        ];

        let result =
            DriftBenchmark::new(config)
                .expect("configuration must be valid")
                .analyze(&samples)
                .expect("analysis must succeed");

        assert!(!result.drift_detected);
        assert_eq!(result.severity, DriftSeverity::None);
    }

    #[test]
    fn increasing_series_is_detected() {
        let mut config = production_config();

        config.absolute_drift_threshold = 0.01;
        config.relative_drift_threshold = 0.10;

        let samples = vec![
            sample(1, 0.010),
            sample(2, 0.011),
            sample(3, 0.012),
            sample(4, 0.020),
            sample(5, 0.025),
        ];

        let result =
            DriftBenchmark::new(config)
                .expect("configuration must be valid")
                .analyze(&samples)
                .expect("analysis must succeed");

        assert!(result.drift_detected);
        assert_eq!(
            result.direction,
            DriftDirection::Increasing
        );
        assert!(result.is_degradation());
    }

    #[test]
    fn decreasing_series_is_detected() {
        let mut config = production_config();

        config.absolute_drift_threshold = 0.005;

        let samples = vec![
            sample(1, 0.020),
            sample(2, 0.019),
            sample(3, 0.018),
            sample(4, 0.010),
            sample(5, 0.008),
        ];

        let result =
            DriftBenchmark::new(config)
                .expect("configuration must be valid")
                .analyze(&samples)
                .expect("analysis must succeed");

        assert!(result.drift_detected);
        assert_eq!(
            result.direction,
            DriftDirection::Decreasing
        );
        assert!(result.is_improvement());
    }

    #[test]
    fn flagged_samples_are_retained_but_can_be_excluded() {
        let config = production_config();

        let flagged = sample(2, 100.0)
            .with_quality(SampleQuality::Flagged);

        let samples = vec![
            sample(1, 0.01),
            flagged,
            sample(3, 0.011),
        ];

        let result =
            DriftBenchmark::new(config)
                .expect("configuration must be valid")
                .analyze(&samples)
                .expect("analysis must succeed");

        assert_eq!(result.total_samples, 3);
        assert_eq!(result.flagged_samples, 1);
        assert_eq!(result.valid_samples, 2);
        assert_eq!(result.invalid_samples, 0);
        assert!(
            result
                .warnings
                .iter()
                .any(|warning| warning.contains("flagged"))
        );
    }

    #[test]
    fn invalid_samples_are_excluded_without_being_silently_deleted() {
        let config = production_config();

        let invalid =
            sample(2, 0.5)
                .with_quality(SampleQuality::Invalid);

        let samples = vec![
            sample(1, 0.01),
            invalid,
            sample(3, 0.011),
        ];

        let result =
            DriftBenchmark::new(config)
                .expect("configuration must be valid")
                .analyze(&samples)
                .expect("analysis must succeed");

        assert_eq!(result.total_samples, 3);
        assert_eq!(result.invalid_samples, 1);
        assert_eq!(result.valid_samples, 2);
    }

    #[test]
    fn linear_trend_has_positive_slope() {
        let samples = vec![
            sample(1, 1.0),
            sample(2, 2.0),
            sample(3, 3.0),
            sample(4, 4.0),
        ];

        let references: Vec<&DriftSample> =
            samples.iter().collect();

        let trend =
            calculate_linear_trend(&references)
                .expect("trend must succeed");

        assert!(trend.slope_per_second > 0.0);
        assert!(
            trend.r_squared
                .expect("r squared must exist")
                > 0.99
        );
    }

    #[test]
    fn median_baseline_is_robust_to_initial_outlier() {
        let mut config = production_config();

        config.baseline_strategy =
            BaselineStrategy::InitialMedian;
        config.baseline_window = 3;

        let samples = vec![
            sample(1, 100.0),
            sample(2, 1.0),
            sample(3, 1.0),
            sample(4, 1.0),
        ];

        let result =
            DriftBenchmark::new(config)
                .expect("configuration must be valid")
                .analyze(&samples)
                .expect("analysis must succeed");

        assert!((result.baseline - 1.0).abs() < EPSILON);
    }

    #[test]
    fn relative_change_is_none_for_zero_baseline() {
        assert_eq!(
            calculate_relative_change(0.0, 1.0),
            None
        );
    }

    #[test]
    fn relative_change_is_correct() {
        let change =
            calculate_relative_change(10.0, 12.0)
                .expect("baseline is non-zero");

        assert!((change - 0.2).abs() < EPSILON);
    }

    #[test]
    fn ewma_is_deterministic() {
        let samples = vec![
            sample(1, 1.0),
            sample(2, 2.0),
            sample(3, 3.0),
        ];

        let references: Vec<&DriftSample> =
            samples.iter().collect();

        let first =
            calculate_ewma(&references, 0.2)
                .expect("EWMA must succeed");

        let second =
            calculate_ewma(&references, 0.2)
                .expect("EWMA must succeed");

        assert_eq!(first, second);
    }

    #[test]
    fn cusum_detects_strong_increase() {
        let samples = vec![
            sample(1, 0.0),
            sample(2, 0.0),
            sample(3, 0.0),
            sample(4, 10.0),
            sample(5, 10.0),
            sample(6, 10.0),
        ];

        let references: Vec<&DriftSample> =
            samples.iter().collect();

        let result =
            calculate_cusum(
                &references,
                0.0,
                1.0,
                0.5,
                5.0,
            )
            .expect("CUSUM must succeed");

        assert!(result.first_positive_alarm.is_some());
        assert!(result.alarmed());
    }

    #[test]
    fn zero_baseline_produces_warning_not_failure() {
        let mut config = production_config();

        config.baseline_strategy =
            BaselineStrategy::FirstSample;

        let samples = vec![
            sample(1, 0.0),
            sample(2, 0.1),
            sample(3, 0.2),
        ];

        let result =
            DriftBenchmark::new(config)
                .expect("configuration must be valid")
                .analyze(&samples)
                .expect("analysis must succeed");

        assert!(result.relative_change.is_none());

        assert!(
            result
                .warnings
                .iter()
                .any(|warning| warning.contains("zero"))
        );
    }

    #[test]
    fn backend_and_calibration_provenance_is_preserved() {
        let first =
            sample(1, 0.01)
                .with_backend_id("simulator")
                .with_calibration_id("cal-001");

        let second =
            sample(2, 0.02)
                .with_backend_id("qpu-1")
                .with_calibration_id("cal-002");

        let samples = vec![first, second];

        let config = production_config();

        let result =
            DriftBenchmark::new(config)
                .expect("configuration must be valid")
                .analyze(&samples)
                .expect("analysis must succeed");

        assert_eq!(
            result.backend_ids,
            vec![
                "qpu-1".to_string(),
                "simulator".to_string()
            ]
        );

        assert_eq!(
            result.calibration_ids,
            vec![
                "cal-001".to_string(),
                "cal-002".to_string()
            ]
        );
    }

    #[test]
    fn sample_limit_is_enforced() {
        let mut config = production_config();
        config.max_samples = 2;

        let samples = vec![
            sample(1, 1.0),
            sample(2, 1.0),
            sample(3, 1.0),
        ];

        let result =
            DriftBenchmark::new(config)
                .expect("configuration must be valid")
                .analyze(&samples);

        assert!(matches!(
            result,
            Err(DriftError::SampleLimitExceeded { .. })
        ));
    }

    #[test]
    fn custom_metric_identifier_is_supported() {
        let config = DriftConfig {
            metric_id: "qpu.gate_error.2q".to_string(),
            ..production_config()
        };

        assert!(config.validate().is_ok());
    }

    #[test]
    fn invalid_metric_identifier_is_rejected() {
        let config = DriftConfig {
            metric_id: "gate error".to_string(),
            ..production_config()
        };

        assert!(matches!(
            config.validate(),
            Err(DriftError::InvalidMetricId)
        ));
    }

    #[test]
    fn direction_semantics_are_correct() {
        let mut config = production_config();

        config.direction =
            MetricDirection::HigherIsBetter;
        config.absolute_drift_threshold = 0.001;

        let samples = vec![
            sample(1, 1.0),
            sample(2, 0.9),
            sample(3, 0.8),
        ];

        let result =
            DriftBenchmark::new(config)
                .expect("configuration must be valid")
                .analyze(&samples)
                .expect("analysis must succeed");

        assert!(result.is_degradation());
        assert!(!result.is_improvement());
    }
}