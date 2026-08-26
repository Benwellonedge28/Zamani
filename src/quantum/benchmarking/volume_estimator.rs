//! Zamani Quantum Benchmarking — Quantum Volume Estimator
//!
//! # Purpose
//!
//! This module contains the pure mathematical/statistical layer for Quantum
//! Volume (QV).
//!
//! It deliberately does NOT:
//!
//! - generate quantum circuits;
//! - execute circuits;
//! - communicate with hardware;
//! - select a backend;
//! - perform transpilation;
//! - perform routing;
//! - perform scheduling;
//! - depend on the Quantum IR;
//! - depend on a simulator;
//! - depend on a hardware provider;
//! - print diagnostics;
//! - maintain process-global state.
//!
//! Those responsibilities belong to the surrounding benchmarking architecture.
//!
//! The intended production dependency direction is:
//!
//! ```text
//! Zamani benchmark protocol
//!         │
//!         ▼
//! QV circuit generator
//!         │
//!         ▼
//! QV execution
//!         │
//!         ▼
//! volume_estimator
//!         │
//!         ▼
//! QuantumVolumeResult
//! ```
//!
//! In the eventual production benchmarking tree:
//!
//! ```text
//! generators/qv.rs
//!        │
//!        ▼
//! protocols/quantum_volume.rs
//!        │
//!        ▼
//! execution/
//!        │
//!        ▼
//! protocols/quantum_volume.rs
//!        │
//!        ▼
//! volume_estimator.rs
//! ```
//!
//! # Quantum Volume
//!
//! Quantum Volume is conventionally represented as:
//!
//! ```text
//! QV = 2^m
//! ```
//!
//! where `m` is the largest square circuit dimension for which the benchmark
//! succeeds. A square dimension means that the tested circuit width and depth
//! are both at least `m`.
//!
//! The conventional heavy-output threshold is:
//!
//! ```text
//! 2 / 3
//! ```
//!
//! A production benchmark should not confuse the raw measured heavy-output
//! probability with statistical success. Zamani therefore evaluates success
//! using the lower confidence bound:
//!
//! ```text
//! lower_confidence_bound > heavy_output_threshold
//! ```
//!
//! This module uses a Wilson score interval for binomial heavy-output data.
//!
//! # Statistical semantics
//!
//! `confidence_level` is the confidence level of the two-sided Wilson interval.
//! The default is the two-sided probability corresponding to exactly two
//! standard deviations under a normal model:
//!
//! ```text
//! approximately 95.4499736%
//! ```
//!
//! This corresponds to a z-score of approximately 2.0.
//!
//! If a calling protocol requires a one-sided 2σ convention (~97.72499%),
//! it should explicitly configure that confidence level rather than silently
//! relying on an ambiguous interpretation of "2σ".
//!
//! # Reproducibility
//!
//! Random circuit generation, circuit identities, backend metadata, compiler
//! metadata, and experiment provenance belong to higher-level benchmarking
//! modules. This estimator is deterministic given:
//!
//! - width;
//! - depth;
//! - threshold;
//! - confidence level;
//! - sample count;
//! - heavy-output count/probability.
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
//! This file is intentionally independent of all future benchmarking files.
//!
//! Future modules may consume this file as follows:
//!
//! ```text
//! protocols::quantum_volume
//!     -> QuantumVolumeConfig
//!     -> QuantumVolumeEstimator
//!     -> QuantumVolumeResult
//! ```
//!
//! Future `statistics/confidence.rs` may eventually replace the internal
//! Wilson implementation, but this file does not require that module to exist.
//! That allows this file to be completed and tested independently first.
//!
//! Future `core/result.rs` may wrap `QuantumVolumeResult` into the universal
//! `BenchmarkResult`, but this file remains the authoritative QV-specific
//! mathematical result until that integration is implemented.
//!
//! The estimator therefore does not need to be edited merely because those
//! future modules are added.

use std::error::Error;
use std::fmt;

// ============================================================================
// Public constants
// ============================================================================

/// Stable identifier for the Quantum Volume benchmark.
pub const QUANTUM_VOLUME_BENCHMARK_ID: &str = "quantum_volume";

/// Version of this QV mathematical result contract.
///
/// This is independent of the Zamani compiler version and independent of the
/// eventual protocol version.
pub const QUANTUM_VOLUME_RESULT_SCHEMA_VERSION: u32 = 1;

/// Conventional heavy-output probability threshold.
///
/// A QV experiment succeeds only when the statistical lower confidence bound
/// is strictly greater than this threshold.
pub const DEFAULT_HEAVY_OUTPUT_THRESHOLD: f64 = 2.0 / 3.0;

/// Two-sided normal probability corresponding to a two-standard-deviation
/// interval.
///
/// This is:
///
/// `Phi(2) - Phi(-2)`
///
/// and is approximately 95.4499736103641%.
pub const DEFAULT_CONFIDENCE_LEVEL: f64 = 0.954_499_736_103_641_6;

/// One-sided confidence level corresponding approximately to a two-sigma
/// lower-tail criterion.
///
/// This is provided explicitly so callers do not have to calculate it
/// themselves. It is NOT the default because `wilson_interval()` returns a
/// two-sided interval.
pub const TWO_SIGMA_ONE_SIDED_CONFIDENCE_LEVEL: f64 = 0.977_249_868_051_820_8;

/// Mathematical lower bound accepted for a probability.
pub const MIN_PROBABILITY: f64 = 0.0;

/// Mathematical upper bound accepted for a probability.
pub const MAX_PROBABILITY: f64 = 1.0;

/// Maximum supported confidence level.
///
/// Confidence levels arbitrarily close to 1 cause the normal quantile to grow
/// without bound and are not scientifically useful for a finite benchmark.
pub const MAX_CONFIDENCE_LEVEL: f64 = 0.999_999_999_999;

/// Minimum supported confidence level.
///
/// Values below this are statistically legal but not useful for a production
/// QV decision and can produce unstable interpretation.
pub const MIN_CONFIDENCE_LEVEL: f64 = 0.5;

// ============================================================================
// Internal numerical constants
// ============================================================================

/// Maximum exponent that can be represented by a `usize` shift.
const MAX_USIZE_EXPONENT: usize = usize::BITS as usize - 1;

/// Numerical tolerance used when validating floating-point values that should
/// lie in [0, 1].
const UNIT_INTERVAL_EPSILON: f64 = 1.0e-15;

// ============================================================================
// Errors
// ============================================================================

/// Errors produced by the Quantum Volume estimator.
///
/// This error type is intentionally local to this mathematical module.
/// Future `benchmarking::core::errors` may wrap it without requiring this file
/// to know about the rest of the benchmarking framework.
#[derive(Debug, Clone, PartialEq)]
pub enum QuantumVolumeError {
    /// Number of qubits is zero.
    InvalidQubitCount,

    /// Circuit depth is zero.
    InvalidGateDepth,

    /// Heavy-output threshold is invalid.
    InvalidThreshold {
        value: f64,
    },

    /// Confidence level is invalid.
    InvalidConfidenceLevel {
        value: f64,
    },

    /// Number of samples is zero.
    InvalidSampleCount,

    /// Heavy-output count cannot be represented by the supplied sample count.
    HeavyOutputExceedsSamples {
        heavy_outputs: usize,
        samples: usize,
    },

    /// A probability was outside [0, 1] or was not finite.
    InvalidProbability {
        value: f64,
    },

    /// The QV exponent cannot be represented as a shift in `usize`.
    ExponentOverflow {
        exponent: usize,
    },

    /// The theoretical volume cannot be represented.
    VolumeOverflow {
        exponent: usize,
    },

    /// An internal statistical calculation produced a non-finite value.
    NonFiniteStatistic {
        statistic: &'static str,
    },
}

impl fmt::Display for QuantumVolumeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidQubitCount => {
                write!(formatter, "quantum volume requires at least one qubit")
            }

            Self::InvalidGateDepth => {
                write!(
                    formatter,
                    "quantum volume requires a gate depth greater than zero"
                )
            }

            Self::InvalidThreshold { value } => {
                write!(
                    formatter,
                    "heavy-output threshold must be finite and in [0, 1], got {}",
                    value
                )
            }

            Self::InvalidConfidenceLevel { value } => {
                write!(
                    formatter,
                    "confidence level must be finite, greater than 0.5, \
                     and no greater than {}, got {}",
                    MAX_CONFIDENCE_LEVEL,
                    value
                )
            }

            Self::InvalidSampleCount => {
                write!(
                    formatter,
                    "quantum volume requires at least one sample"
                )
            }

            Self::HeavyOutputExceedsSamples {
                heavy_outputs,
                samples,
            } => {
                write!(
                    formatter,
                    "heavy-output count {} exceeds sample count {}",
                    heavy_outputs,
                    samples
                )
            }

            Self::InvalidProbability { value } => {
                write!(
                    formatter,
                    "heavy-output probability must be finite and in [0, 1], got {}",
                    value
                )
            }

            Self::ExponentOverflow { exponent } => {
                write!(
                    formatter,
                    "quantum-volume exponent {} cannot be represented by usize",
                    exponent
                )
            }

            Self::VolumeOverflow { exponent } => {
                write!(
                    formatter,
                    "quantum-volume value 2^{} cannot be represented by usize",
                    exponent
                )
            }

            Self::NonFiniteStatistic { statistic } => {
                write!(
                    formatter,
                    "statistical calculation produced a non-finite {}",
                    statistic
                )
            }
        }
    }
}

impl Error for QuantumVolumeError {}

// ============================================================================
// Confidence interval method
// ============================================================================

/// Statistical confidence-interval method used by the estimator.
///
/// This enum exists now so the result explicitly records the method instead of
/// leaving statistical interpretation implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfidenceIntervalMethod {
    /// Wilson score interval for a binomial proportion.
    Wilson,
}

impl ConfidenceIntervalMethod {
    /// Stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Wilson => "wilson",
        }
    }
}

// ============================================================================
// Quantum Volume configuration
// ============================================================================

/// Configuration for one square Quantum Volume test point.
///
/// A higher-level protocol may create many configurations:
///
/// ```text
/// 1 x 1
/// 2 x 2
/// 3 x 3
/// ...
/// ```
///
/// The protocol is responsible for selecting the largest successful point.
/// This structure is deliberately limited to the mathematical properties
/// needed by the estimator.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct QuantumVolumeConfig {
    /// Number of active qubits in the tested circuit.
    pub num_qubits: usize,

    /// Circuit depth.
    pub gate_depth: usize,

    /// Heavy-output probability threshold.
    pub heavy_output_threshold: f64,

    /// Confidence level of the two-sided Wilson interval.
    pub confidence_level: f64,

    /// Statistical method.
    pub confidence_interval_method: ConfidenceIntervalMethod,
}

impl QuantumVolumeConfig {
    /// Creates a configuration using Zamani's production defaults.
    pub fn new(
        num_qubits: usize,
        gate_depth: usize,
    ) -> Result<Self, QuantumVolumeError> {
        Self {
            num_qubits,
            gate_depth,
            heavy_output_threshold: DEFAULT_HEAVY_OUTPUT_THRESHOLD,
            confidence_level: DEFAULT_CONFIDENCE_LEVEL,
            confidence_interval_method: ConfidenceIntervalMethod::Wilson,
        }
        .validate()
    }

    /// Creates a configuration with a custom threshold.
    pub fn with_threshold(
        num_qubits: usize,
        gate_depth: usize,
        heavy_output_threshold: f64,
    ) -> Result<Self, QuantumVolumeError> {
        Self {
            num_qubits,
            gate_depth,
            heavy_output_threshold,
            confidence_level: DEFAULT_CONFIDENCE_LEVEL,
            confidence_interval_method: ConfidenceIntervalMethod::Wilson,
        }
        .validate()
    }

    /// Creates a configuration with explicit threshold and confidence level.
    pub fn with_threshold_and_confidence(
        num_qubits: usize,
        gate_depth: usize,
        heavy_output_threshold: f64,
        confidence_level: f64,
    ) -> Result<Self, QuantumVolumeError> {
        Self {
            num_qubits,
            gate_depth,
            heavy_output_threshold,
            confidence_level,
            confidence_interval_method: ConfidenceIntervalMethod::Wilson,
        }
        .validate()
    }

    /// Creates a configuration with an explicit statistical method.
    ///
    /// Wilson is currently the only implementation provided by this
    /// independent module.
    pub fn with_statistical_method(
        num_qubits: usize,
        gate_depth: usize,
        heavy_output_threshold: f64,
        confidence_level: f64,
        confidence_interval_method: ConfidenceIntervalMethod,
    ) -> Result<Self, QuantumVolumeError> {
        Self {
            num_qubits,
            gate_depth,
            heavy_output_threshold,
            confidence_level,
            confidence_interval_method,
        }
        .validate()
    }

    /// Validate all configuration values.
    pub fn validate(&self) -> Result<Self, QuantumVolumeError> {
        if self.num_qubits == 0 {
            return Err(QuantumVolumeError::InvalidQubitCount);
        }

        if self.gate_depth == 0 {
            return Err(QuantumVolumeError::InvalidGateDepth);
        }

        validate_unit_interval(
            self.heavy_output_threshold,
            QuantumVolumeError::InvalidThreshold,
        )?;

        if !self.confidence_level.is_finite()
            || self.confidence_level < MIN_CONFIDENCE_LEVEL
            || self.confidence_level > MAX_CONFIDENCE_LEVEL
        {
            return Err(QuantumVolumeError::InvalidConfidenceLevel {
                value: self.confidence_level,
            });
        }

        Ok(*self)
    }

    /// Returns the square dimension represented by this test point.
    ///
    /// QV uses the largest square dimension, therefore:
    ///
    /// `m = min(width, depth)`.
    pub const fn exponent(&self) -> usize {
        if self.num_qubits < self.gate_depth {
            self.num_qubits
        } else {
            self.gate_depth
        }
    }

    /// Returns the theoretical QV associated with this square dimension.
    ///
    /// This is the mathematical `2^m`, not an experimental result.
    pub fn theoretical_volume(&self) -> Result<usize, QuantumVolumeError> {
        checked_quantum_volume(self.exponent())
    }

    /// Returns the statistical method identifier.
    pub const fn confidence_interval_method(&self) -> &'static str {
        self.confidence_interval_method.as_str()
    }
}

// ============================================================================
// Confidence interval
// ============================================================================

/// Confidence interval for a measured heavy-output probability.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ConfidenceInterval {
    /// Lower confidence bound.
    pub lower: f64,

    /// Upper confidence bound.
    pub upper: f64,

    /// Confidence level represented by this interval.
    pub confidence_level: f64,

    /// Statistical method.
    pub method: ConfidenceIntervalMethod,
}

impl ConfidenceInterval {
    /// Creates a validated confidence interval.
    pub fn new(
        lower: f64,
        upper: f64,
        confidence_level: f64,
        method: ConfidenceIntervalMethod,
    ) -> Result<Self, QuantumVolumeError> {
        if !lower.is_finite() {
            return Err(QuantumVolumeError::NonFiniteStatistic {
                statistic: "confidence lower bound",
            });
        }

        if !upper.is_finite() {
            return Err(QuantumVolumeError::NonFiniteStatistic {
                statistic: "confidence upper bound",
            });
        }

        if lower < 0.0
            || upper > 1.0
            || lower > upper
        {
            return Err(QuantumVolumeError::InvalidProbability {
                value: if lower < 0.0 || lower > 1.0 {
                    lower
                } else {
                    upper
                },
            });
        }

        if !confidence_level.is_finite()
            || confidence_level < MIN_CONFIDENCE_LEVEL
            || confidence_level > MAX_CONFIDENCE_LEVEL
        {
            return Err(QuantumVolumeError::InvalidConfidenceLevel {
                value: confidence_level,
            });
        }

        Ok(Self {
            lower,
            upper,
            confidence_level,
            method,
        })
    }

    /// Returns the interval width.
    pub fn width(&self) -> f64 {
        self.upper - self.lower
    }

    /// Returns whether the entire interval is above the supplied threshold.
    pub fn strictly_above(&self, threshold: f64) -> bool {
        threshold.is_finite() && self.lower > threshold
    }

    /// Returns whether the interval contains a supplied probability.
    pub fn contains(&self, probability: f64) -> bool {
        probability.is_finite()
            && probability >= self.lower
            && probability <= self.upper
    }
}

// ============================================================================
// Quantum Volume result
// ============================================================================

/// Complete mathematical result for one Quantum Volume test point.
///
/// This structure intentionally contains enough information to independently
/// audit the decision without rerunning the experiment.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct QuantumVolumeResult {
    /// Result schema version.
    pub schema_version: u32,

    /// Benchmark identifier.
    pub benchmark_id: &'static str,

    /// Tested number of qubits.
    pub num_qubits: usize,

    /// Tested circuit depth.
    pub gate_depth: usize,

    /// Square QV exponent.
    pub exponent: usize,

    /// Number of measured shots/samples.
    pub samples: usize,

    /// Exact heavy-output count when raw counts were supplied.
    ///
    /// When the result was constructed from a probability only, this is the
    /// nearest representable count and `heavy_outputs_are_exact` is false.
    pub heavy_outputs: usize,

    /// Whether `heavy_outputs` is an exact observed count.
    pub heavy_outputs_are_exact: bool,

    /// Measured heavy-output probability.
    pub heavy_output_probability: f64,

    /// Heavy-output threshold.
    pub heavy_output_threshold: f64,

    /// Confidence interval.
    pub confidence_interval: ConfidenceInterval,

    /// Whether the lower confidence bound strictly exceeds the threshold.
    pub passed: bool,

    /// QV value if the test passed.
    pub quantum_volume: Option<usize>,
}

impl QuantumVolumeResult {
    /// Evaluate raw heavy-output observations.
    ///
    /// This is the preferred production entry point when the execution layer
    /// has raw counts.
    pub fn from_samples(
        config: QuantumVolumeConfig,
        samples: usize,
        heavy_outputs: usize,
    ) -> Result<Self, QuantumVolumeError> {
        config.validate()?;

        if samples == 0 {
            return Err(QuantumVolumeError::InvalidSampleCount);
        }

        if heavy_outputs > samples {
            return Err(
                QuantumVolumeError::HeavyOutputExceedsSamples {
                    heavy_outputs,
                    samples,
                },
            );
        }

        let probability =
            heavy_outputs as f64 / samples as f64;

        Self::from_probability_internal(
            config,
            samples,
            probability,
            heavy_outputs,
            true,
        )
    }

    /// Evaluate a previously calculated heavy-output probability.
    ///
    /// This is useful when an external execution system has already performed
    /// sampling and returned a probability rather than raw counts.
    ///
    /// Because a probability alone does not identify the exact observed count,
    /// `heavy_outputs_are_exact` is false in the resulting object.
    pub fn from_probability(
        config: QuantumVolumeConfig,
        samples: usize,
        probability: f64,
    ) -> Result<Self, QuantumVolumeError> {
        config.validate()?;

        if samples == 0 {
            return Err(QuantumVolumeError::InvalidSampleCount);
        }

        validate_probability(probability)?;

        let estimated_count =
            probability_to_nearest_count(probability, samples)?;

        Self::from_probability_internal(
            config,
            samples,
            probability,
            estimated_count,
            false,
        )
    }

    fn from_probability_internal(
        config: QuantumVolumeConfig,
        samples: usize,
        probability: f64,
        heavy_outputs: usize,
        heavy_outputs_are_exact: bool,
    ) -> Result<Self, QuantumVolumeError> {
        validate_probability(probability)?;

        let confidence_interval = match config.confidence_interval_method {
            ConfidenceIntervalMethod::Wilson => wilson_interval(
                probability,
                samples,
                config.confidence_level,
            )?,
        };

        let passed =
            confidence_interval.strictly_above(
                config.heavy_output_threshold,
            );

        let quantum_volume = if passed {
            Some(config.theoretical_volume()?)
        } else {
            None
        };

        Ok(Self {
            schema_version: QUANTUM_VOLUME_RESULT_SCHEMA_VERSION,
            benchmark_id: QUANTUM_VOLUME_BENCHMARK_ID,
            num_qubits: config.num_qubits,
            gate_depth: config.gate_depth,
            exponent: config.exponent(),
            samples,
            heavy_outputs,
            heavy_outputs_are_exact,
            heavy_output_probability: probability,
            heavy_output_threshold: config.heavy_output_threshold,
            confidence_interval,
            passed,
            quantum_volume,
        })
    }

    /// Returns the confidence lower bound.
    pub fn confidence_lower(&self) -> f64 {
        self.confidence_interval.lower
    }

    /// Returns the confidence upper bound.
    pub fn confidence_upper(&self) -> f64 {
        self.confidence_interval.upper
    }

    /// Returns the confidence interval width.
    pub fn confidence_width(&self) -> f64 {
        self.confidence_interval.width()
    }

    /// Returns the statistical method identifier.
    pub fn confidence_method(&self) -> &'static str {
        self.confidence_interval.method.as_str()
    }

    /// Returns the theoretical volume for this test point.
    ///
    /// Unlike `quantum_volume`, this returns the mathematical volume even if
    /// the experimental test failed.
    pub fn theoretical_volume(&self) -> Result<usize, QuantumVolumeError> {
        checked_quantum_volume(self.exponent)
    }

    /// Returns a concise stable decision classification.
    pub const fn decision(&self) -> QuantumVolumeDecision {
        if self.passed {
            QuantumVolumeDecision::Passed
        } else {
            QuantumVolumeDecision::Failed
        }
    }
}

// ============================================================================
// Decision
// ============================================================================

/// Quantum Volume test-point decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuantumVolumeDecision {
    /// Lower confidence bound is strictly above the heavy-output threshold.
    Passed,

    /// Lower confidence bound does not exceed the threshold.
    Failed,
}

impl QuantumVolumeDecision {
    /// Stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Passed => "passed",
            Self::Failed => "failed",
        }
    }
}

// ============================================================================
// Quantum Volume estimator
// ============================================================================

/// Production Quantum Volume estimator.
///
/// This preserves the original public API while adding fallible constructors
/// and complete statistical evaluation.
///
/// The estimator contains no execution state.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct QuantumVolumeEstimator {
    /// Number of qubits.
    pub num_qubits: usize,

    /// Circuit depth.
    pub gate_depth: usize,

    /// Heavy-output threshold.
    pub heavy_output_threshold: f64,

    /// Confidence level.
    pub confidence_level: f64,
}

impl QuantumVolumeEstimator {
    /// Creates an estimator using Zamani's production defaults.
    ///
    /// This method preserves the historical infallible API.
    ///
    /// For new production code where invalid user input is possible, prefer
    /// `try_new()`.
    pub fn new(
        num_qubits: usize,
        gate_depth: usize,
    ) -> Self {
        match Self::try_new(num_qubits, gate_depth) {
            Ok(estimator) => estimator,
            Err(error) => {
                panic!(
                    "invalid quantum volume estimator configuration: {}",
                    error
                )
            }
        }
    }

    /// Fallible constructor.
    pub fn try_new(
        num_qubits: usize,
        gate_depth: usize,
    ) -> Result<Self, QuantumVolumeError> {
        let config =
            QuantumVolumeConfig::new(num_qubits, gate_depth)?;

        Ok(Self::from_config(config))
    }

    /// Constructor with custom threshold.
    pub fn with_threshold(
        num_qubits: usize,
        gate_depth: usize,
        threshold: f64,
    ) -> Result<Self, QuantumVolumeError> {
        let config = QuantumVolumeConfig::with_threshold(
            num_qubits,
            gate_depth,
            threshold,
        )?;

        Ok(Self::from_config(config))
    }

    /// Constructor with explicit threshold and confidence level.
    pub fn with_threshold_and_confidence(
        num_qubits: usize,
        gate_depth: usize,
        threshold: f64,
        confidence_level: f64,
    ) -> Result<Self, QuantumVolumeError> {
        let config =
            QuantumVolumeConfig::with_threshold_and_confidence(
                num_qubits,
                gate_depth,
                threshold,
                confidence_level,
            )?;

        Ok(Self::from_config(config))
    }

    /// Construct from a validated configuration.
    pub fn from_config(
        config: QuantumVolumeConfig,
    ) -> Self {
        Self {
            num_qubits: config.num_qubits,
            gate_depth: config.gate_depth,
            heavy_output_threshold:
                config.heavy_output_threshold,
            confidence_level: config.confidence_level,
        }
    }

    /// Return the configuration represented by this estimator.
    pub fn config(
        &self,
    ) -> Result<QuantumVolumeConfig, QuantumVolumeError> {
        QuantumVolumeConfig {
            num_qubits: self.num_qubits,
            gate_depth: self.gate_depth,
            heavy_output_threshold:
                self.heavy_output_threshold,
            confidence_level: self.confidence_level,
            confidence_interval_method:
                ConfidenceIntervalMethod::Wilson,
        }
        .validate()
    }

    /// Return the QV exponent.
    pub fn exponent(&self) -> usize {
        if self.num_qubits < self.gate_depth {
            self.num_qubits
        } else {
            self.gate_depth
        }
    }

    /// Return the theoretical QV.
    ///
    /// This method is retained for compatibility with the original estimator.
    ///
    /// Invalid internal state cannot normally occur because public constructors
    /// validate their configuration. If the public fields are manually mutated,
    /// this method returns `0` rather than emitting output or panicking.
    ///
    /// New code should prefer `try_estimate_quantum_volume()`.
    pub fn estimate_quantum_volume(&self) -> usize {
        match self.try_estimate_quantum_volume() {
            Ok(volume) => volume,
            Err(_) => 0,
        }
    }

    /// Fallible theoretical QV calculation.
    pub fn try_estimate_quantum_volume(
        &self,
    ) -> Result<usize, QuantumVolumeError> {
        let config = self.config()?;
        config.theoretical_volume()
    }

    /// Evaluate raw heavy-output observations.
    pub fn evaluate(
        &self,
        samples: usize,
        heavy_outputs: usize,
    ) -> Result<QuantumVolumeResult, QuantumVolumeError> {
        let config = self.config()?;

        QuantumVolumeResult::from_samples(
            config,
            samples,
            heavy_outputs,
        )
    }

    /// Evaluate an externally calculated heavy-output probability.
    pub fn evaluate_probability(
        &self,
        samples: usize,
        probability: f64,
    ) -> Result<QuantumVolumeResult, QuantumVolumeError> {
        let config = self.config()?;

        QuantumVolumeResult::from_probability(
            config,
            samples,
            probability,
        )
    }

    /// Determine whether a probability is strictly above the configured
    /// threshold.
    ///
    /// This is intentionally a raw-threshold check only. It does NOT constitute
    /// a statistically valid QV pass decision.
    ///
    /// Use `evaluate()` for the production benchmark decision.
    pub fn passes_threshold(
        &self,
        probability: f64,
    ) -> bool {
        probability.is_finite()
            && probability > self.heavy_output_threshold
    }

    /// Calculate a Wilson interval for an externally supplied probability.
    pub fn confidence_interval(
        &self,
        samples: usize,
        probability: f64,
    ) -> Result<ConfidenceInterval, QuantumVolumeError> {
        let config = self.config()?;

        if samples == 0 {
            return Err(QuantumVolumeError::InvalidSampleCount);
        }

        validate_probability(probability)?;

        wilson_interval(
            probability,
            samples,
            config.confidence_level,
        )
    }
}

// ============================================================================
// Mathematical helpers
// ============================================================================

/// Validate a probability in the closed unit interval.
fn validate_probability(
    probability: f64,
) -> Result<(), QuantumVolumeError> {
    if !probability.is_finite()
        || probability < MIN_PROBABILITY
        || probability > MAX_PROBABILITY
    {
        return Err(QuantumVolumeError::InvalidProbability {
            value: probability,
        });
    }

    Ok(())
}

/// Validate a unit-interval value and map it into a caller-specific error.
fn validate_unit_interval<F>(
    value: f64,
    error: F,
) -> Result<(), QuantumVolumeError>
where
    F: FnOnce(f64) -> QuantumVolumeError,
{
    if !value.is_finite()
        || value < -UNIT_INTERVAL_EPSILON
        || value > 1.0 + UNIT_INTERVAL_EPSILON
    {
        return Err(error(value));
    }

    Ok(())
}

/// Checked calculation of `2^exponent` in `usize`.
fn checked_quantum_volume(
    exponent: usize,
) -> Result<usize, QuantumVolumeError> {
    if exponent > MAX_USIZE_EXPONENT {
        return Err(QuantumVolumeError::ExponentOverflow {
            exponent,
        });
    }

    1usize
        .checked_shl(exponent as u32)
        .ok_or(QuantumVolumeError::VolumeOverflow {
            exponent,
        })
}

/// Convert a probability into the nearest representable count.
///
/// This conversion is only metadata when the original backend supplied a
/// probability. It must never be represented as an exact observed count.
fn probability_to_nearest_count(
    probability: f64,
    samples: usize,
) -> Result<usize, QuantumVolumeError> {
    validate_probability(probability)?;

    let count =
        probability * samples as f64;

    if !count.is_finite() {
        return Err(QuantumVolumeError::NonFiniteStatistic {
            statistic: "estimated heavy-output count",
        });
    }

    let rounded = count.round();

    if rounded < 0.0
        || rounded > samples as f64
    {
        return Err(QuantumVolumeError::NonFiniteStatistic {
            statistic: "estimated heavy-output count",
        });
    }

    Ok(rounded as usize)
}

// ============================================================================
// Wilson confidence interval
// ============================================================================

/// Calculate a Wilson score confidence interval for a binomial proportion.
///
/// This is deliberately implemented without an external numerical dependency
/// so the foundational QV estimator remains lightweight and independently
/// buildable.
///
/// The formula is:
///
/// ```text
/// center = (p + z²/(2n)) / (1 + z²/n)
///
/// margin =
///     z / (1 + z²/n)
///     * sqrt(p(1-p)/n + z²/(4n²))
/// ```
///
/// The result is clamped to the mathematical probability interval [0, 1].
pub fn wilson_interval(
    probability: f64,
    samples: usize,
    confidence_level: f64,
) -> Result<ConfidenceInterval, QuantumVolumeError> {
    validate_probability(probability)?;

    if samples == 0 {
        return Err(QuantumVolumeError::InvalidSampleCount);
    }

    if !confidence_level.is_finite()
        || confidence_level < MIN_CONFIDENCE_LEVEL
        || confidence_level > MAX_CONFIDENCE_LEVEL
    {
        return Err(
            QuantumVolumeError::InvalidConfidenceLevel {
                value: confidence_level,
            },
        );
    }

    let tail_probability =
        0.5 + confidence_level / 2.0;

    let z =
        inverse_normal_cdf(tail_probability)?;

    if !z.is_finite() || z <= 0.0 {
        return Err(QuantumVolumeError::NonFiniteStatistic {
            statistic: "normal quantile",
        });
    }

    let n = samples as f64;

    let z_squared = z * z;

    let denominator =
        1.0 + z_squared / n;

    if !denominator.is_finite()
        || denominator <= 0.0
    {
        return Err(QuantumVolumeError::NonFiniteStatistic {
            statistic: "Wilson denominator",
        });
    }

    let center =
        (probability + z_squared / (2.0 * n))
            / denominator;

    let variance_component =
        probability * (1.0 - probability) / n
            + z_squared / (4.0 * n * n);

    if !variance_component.is_finite()
        || variance_component < 0.0
    {
        return Err(QuantumVolumeError::NonFiniteStatistic {
            statistic: "Wilson variance",
        });
    }

    let margin =
        z * variance_component.sqrt()
            / denominator;

    if !margin.is_finite() {
        return Err(QuantumVolumeError::NonFiniteStatistic {
            statistic: "Wilson margin",
        });
    }

    let lower =
        (center - margin).clamp(0.0, 1.0);

    let upper =
        (center + margin).clamp(0.0, 1.0);

    ConfidenceInterval::new(
        lower,
        upper,
        confidence_level,
        ConfidenceIntervalMethod::Wilson,
    )
}

// ============================================================================
// Inverse normal CDF
// ============================================================================

/// Calculate the inverse standard-normal cumulative distribution function.
///
/// Uses the Peter J. Acklam rational approximation.
///
/// The function is kept private because it is an implementation detail of the
/// Wilson interval. If Zamani later introduces a shared statistics module, that
/// implementation can replace this function without changing the public QV
/// result contract.
fn inverse_normal_cdf(
    probability: f64,
) -> Result<f64, QuantumVolumeError> {
    if !probability.is_finite()
        || probability <= 0.0
        || probability >= 1.0
    {
        return Err(QuantumVolumeError::InvalidProbability {
            value: probability,
        });
    }

    const A: [f64; 6] = [
        -39.696_830_286_653_76,
        220.946_098_424_520_5,
        -275.928_510_446_968_7,
        138.357_751_867_269,
        -30.664_798_066_147_16,
        2.506_628_277_459_239,
    ];

    const B: [f64; 5] = [
        -54.476_098_798_224_06,
        161.585_836_858_040_9,
        -155.698_979_859_886_6,
        66.801_311_887_719_72,
        -13.280_681_552_885_72,
    ];

    const C: [f64; 6] = [
        -0.007_784_894_002_430_293,
        -0.322_396_458_041_136_5,
        -2.400_758_277_161_838,
        -2.549_732_539_343_734,
        4.374_664_141_464_968,
        2.938_163_982_698_783,
    ];

    const D: [f64; 4] = [
        0.007_784_695_709_041_462,
        0.322_467_129_070_039_8,
        2.445_134_137_142_996,
        3.754_408_661_907_416,
    ];

    const LOW: f64 = 0.024_25;
    const HIGH: f64 = 1.0 - LOW;

    let result = if probability < LOW {
        let q =
            (-2.0 * probability.ln()).sqrt();

        let numerator =
            (((((C[0] * q + C[1]) * q + C[2]) * q
                + C[3])
                * q
                + C[4])
                * q)
                + C[5];

        let denominator =
            ((((D[0] * q + D[1]) * q + D[2]) * q
                + D[3])
                * q)
                + 1.0;

        -numerator / denominator
    } else if probability > HIGH {
        let q =
            (-2.0 * (1.0 - probability).ln()).sqrt();

        let numerator =
            (((((C[0] * q + C[1]) * q + C[2]) * q
                + C[3])
                * q
                + C[4])
                * q)
                + C[5];

        let denominator =
            ((((D[0] * q + D[1]) * q + D[2]) * q
                + D[3])
                * q)
                + 1.0;

        numerator / denominator
    } else {
        let q =
            probability - 0.5;

        let r = q * q;

        (((((A[0] * r + A[1]) * r + A[2]) * r
            + A[3])
            * r
            + A[4])
            * r
            + A[5])
            * q
            / (((((B[0] * r + B[1]) * r + B[2]) * r
                + B[3])
                * r
                + B[4])
                * r)
                + 1.0)
    };

    if !result.is_finite() {
        return Err(QuantumVolumeError::NonFiniteStatistic {
            statistic: "inverse normal CDF",
        });
    }

    Ok(result)
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_configuration_is_valid() {
        let config =
            QuantumVolumeConfig::new(8, 8)
                .expect("8x8 QV configuration must be valid");

        assert_eq!(config.num_qubits, 8);
        assert_eq!(config.gate_depth, 8);
        assert_eq!(
            config.heavy_output_threshold,
            DEFAULT_HEAVY_OUTPUT_THRESHOLD
        );
        assert_eq!(
            config.confidence_level,
            DEFAULT_CONFIDENCE_LEVEL
        );
        assert_eq!(config.exponent(), 8);
    }

    #[test]
    fn exponent_uses_smaller_dimension() {
        let config =
            QuantumVolumeConfig::new(8, 12)
                .expect("configuration must be valid");

        assert_eq!(config.exponent(), 8);
    }

    #[test]
    fn theoretical_volume_is_checked() {
        let config =
            QuantumVolumeConfig::new(8, 8)
                .expect("configuration must be valid");

        assert_eq!(
            config.theoretical_volume()
                .expect("QV must fit"),
            256
        );
    }

    #[test]
    fn one_qubit_volume_is_two() {
        let config =
            QuantumVolumeConfig::new(1, 1)
                .expect("configuration must be valid");

        assert_eq!(
            config.theoretical_volume()
                .expect("QV must fit"),
            2
        );
    }

    #[test]
    fn zero_qubits_are_rejected() {
        let result =
            QuantumVolumeConfig::new(0, 1);

        assert_eq!(
            result,
            Err(QuantumVolumeError::InvalidQubitCount)
        );
    }

    #[test]
    fn zero_depth_is_rejected() {
        let result =
            QuantumVolumeConfig::new(1, 0);

        assert_eq!(
            result,
            Err(QuantumVolumeError::InvalidGateDepth)
        );
    }

    #[test]
    fn invalid_thresholds_are_rejected() {
        assert!(matches!(
            QuantumVolumeConfig::with_threshold(
                4,
                4,
                f64::NAN
            ),
            Err(QuantumVolumeError::InvalidThreshold { .. })
        ));

        assert!(matches!(
            QuantumVolumeConfig::with_threshold(
                4,
                4,
                -0.1
            ),
            Err(QuantumVolumeError::InvalidThreshold { .. })
        ));

        assert!(matches!(
            QuantumVolumeConfig::with_threshold(
                4,
                4,
                1.1
            ),
            Err(QuantumVolumeError::InvalidThreshold { .. })
        ));
    }

    #[test]
    fn invalid_confidence_is_rejected() {
        assert!(matches!(
            QuantumVolumeConfig::with_threshold_and_confidence(
                4,
                4,
                2.0 / 3.0,
                0.1
            ),
            Err(QuantumVolumeError::InvalidConfidenceLevel { .. })
        ));

        assert!(matches!(
            QuantumVolumeConfig::with_threshold_and_confidence(
                4,
                4,
                2.0 / 3.0,
                1.0
            ),
            Err(QuantumVolumeError::InvalidConfidenceLevel { .. })
        ));
    }

    #[test]
    fn zero_samples_are_rejected() {
        let config =
            QuantumVolumeConfig::new(4, 4)
                .expect("configuration must be valid");

        let result =
            QuantumVolumeResult::from_samples(
                config,
                0,
                0,
            );

        assert_eq!(
            result,
            Err(QuantumVolumeError::InvalidSampleCount)
        );
    }

    #[test]
    fn_heavy_outputs_cannot_exceed_samples() {
        let config =
            QuantumVolumeConfig::new(4, 4)
                .expect("configuration must be valid");

        let result =
            QuantumVolumeResult::from_samples(
                config,
                100,
                101,
            );

        assert_eq!(
            result,
            Err(
                QuantumVolumeError::HeavyOutputExceedsSamples {
                    heavy_outputs: 101,
                    samples: 100,
                }
            )
        );
    }

    #[test]
    fn perfect_heavy_output_probability_is_valid() {
        let config =
            QuantumVolumeConfig::new(4, 4)
                .expect("configuration must be valid");

        let result =
            QuantumVolumeResult::from_samples(
                config,
                1_000,
                1_000,
            )
            .expect("perfect result must be valid");

        assert_eq!(
            result.heavy_output_probability,
            1.0
        );

        assert!(result.confidence_lower() > 0.0);
        assert_eq!(
            result.confidence_upper(),
            1.0
        );

        assert!(result.passed);
        assert_eq!(
            result.quantum_volume,
            Some(16)
        );
    }

    #[test]
    fn zero_heavy_outputs_fail() {
        let config =
            QuantumVolumeConfig::new(4, 4)
                .expect("configuration must be valid");

        let result =
            QuantumVolumeResult::from_samples(
                config,
                1_000,
                0,
            )
            .expect("zero-heavy result is valid");

        assert_eq!(
            result.heavy_output_probability,
            0.0
        );

        assert!(!result.passed);
        assert_eq!(
            result.quantum_volume,
            None
        );
    }

    #[test]
    fn raw_count_result_preserves_exact_count() {
        let config =
            QuantumVolumeConfig::new(8, 8)
                .expect("configuration must be valid");

        let result =
            QuantumVolumeResult::from_samples(
                config,
                1_000,
                700,
            )
            .expect("result must be valid");

        assert_eq!(
            result.heavy_outputs,
            700
        );

        assert!(
            result.heavy_outputs_are_exact
        );

        assert_eq!(
            result.heavy_output_probability,
            0.7
        );
    }

    #[test]
    fn probability_result_does_not_claim_exact_count() {
        let config =
            QuantumVolumeConfig::new(8, 8)
                .expect("configuration must be valid");

        let result =
            QuantumVolumeResult::from_probability(
                config,
                1_000,
                0.7,
            )
            .expect("result must be valid");

        assert_eq!(
            result.heavy_outputs,
            700
        );

        assert!(
            !result.heavy_outputs_are_exact
        );
    }

    #[test]
    fn invalid_probability_is_rejected() {
        let config =
            QuantumVolumeConfig::new(4, 4)
                .expect("configuration must be valid");

        assert!(matches!(
            QuantumVolumeResult::from_probability(
                config,
                100,
                f64::NAN
            ),
            Err(QuantumVolumeError::InvalidProbability { .. })
        ));

        assert!(matches!(
            QuantumVolumeResult::from_probability(
                config,
                100,
                -0.1
            ),
            Err(QuantumVolumeError::InvalidProbability { .. })
        ));

        assert!(matches!(
            QuantumVolumeResult::from_probability(
                config,
                100,
                1.1
            ),
            Err(QuantumVolumeError::InvalidProbability { .. })
        ));
    }

    #[test]
    fn wilson_interval_is_inside_probability_domain() {
        let interval =
            wilson_interval(
                0.5,
                1_000,
                DEFAULT_CONFIDENCE_LEVEL,
            )
            .expect("Wilson interval must succeed");

        assert!(
            interval.lower >= 0.0
        );

        assert!(
            interval.upper <= 1.0
        );

        assert!(
            interval.lower < 0.5
        );

        assert!(
            interval.upper > 0.5
        );
    }

    #[test]
    fn wilson_interval_is_ordered() {
        for probability in [
            0.0,
            0.01,
            0.25,
            0.5,
            2.0 / 3.0,
            0.9,
            0.99,
            1.0,
        ] {
            let interval =
                wilson_interval(
                    probability,
                    10_000,
                    DEFAULT_CONFIDENCE_LEVEL,
                )
                .expect("interval must succeed");

            assert!(
                interval.lower <= interval.upper
            );

            assert!(
                interval.lower >= 0.0
            );

            assert!(
                interval.upper <= 1.0
            );

            assert!(
                interval.contains(probability),
                "probability {} should be inside its Wilson interval",
                probability
            );
        }
    }

    #[test]
    fn confidence_interval_is_reproducible() {
        let first =
            wilson_interval(
                0.685,
                10_000,
                DEFAULT_CONFIDENCE_LEVEL,
            )
            .expect("first calculation must succeed");

        let second =
            wilson_interval(
                0.685,
                10_000,
                DEFAULT_CONFIDENCE_LEVEL,
            )
            .expect("second calculation must succeed");

        assert_eq!(first, second);
    }

    #[test]
    fn two_sigma_configuration_is_explicit() {
        let config =
            QuantumVolumeConfig::with_threshold_and_confidence(
                8,
                8,
                DEFAULT_HEAVY_OUTPUT_THRESHOLD,
                DEFAULT_CONFIDENCE_LEVEL,
            )
            .expect("2σ configuration must be valid");

        assert_eq!(
            config.confidence_level,
            DEFAULT_CONFIDENCE_LEVEL
        );

        let one_sided =
            QuantumVolumeConfig::with_threshold_and_confidence(
                8,
                8,
                DEFAULT_HEAVY_OUTPUT_THRESHOLD,
                TWO_SIGMA_ONE_SIDED_CONFIDENCE_LEVEL,
            )
            .expect("one-sided 2σ configuration must be valid");

        assert_eq!(
            one_sided.confidence_level,
            TWO_SIGMA_ONE_SIDED_CONFIDENCE_LEVEL
        );
    }

    #[test]
    fn result_contains_complete_decision_information() {
        let config =
            QuantumVolumeConfig::new(8, 8)
                .expect("configuration must be valid");

        let result =
            QuantumVolumeResult::from_samples(
                config,
                10_000,
                7_000,
            )
            .expect("result must be valid");

        assert_eq!(
            result.schema_version,
            QUANTUM_VOLUME_RESULT_SCHEMA_VERSION
        );

        assert_eq!(
            result.benchmark_id,
            QUANTUM_VOLUME_BENCHMARK_ID
        );

        assert_eq!(
            result.num_qubits,
            8
        );

        assert_eq!(
            result.gate_depth,
            8
        );

        assert_eq!(
            result.exponent,
            8
        );

        assert_eq!(
            result.samples,
            10_000
        );

        assert_eq!(
            result.heavy_outputs,
            7_000
        );

        assert_eq!(
            result.heavy_output_probability,
            0.7
        );

        assert!(
            result.confidence_lower()
                > 0.0
        );

        assert!(
            result.confidence_upper()
                <= 1.0
        );

        assert_eq!(
            result.confidence_method(),
            "wilson"
        );
    }

    #[test]
    fn threshold_check_is_not_statistical_pass_decision() {
        let estimator =
            QuantumVolumeEstimator::new(8, 8);

        assert!(
            estimator.passes_threshold(0.68)
        );

        assert!(
            !estimator.passes_threshold(2.0 / 3.0)
        );

        assert!(
            !estimator.passes_threshold(0.60)
        );

        // This test deliberately documents the API distinction:
        // passes_threshold() is a raw comparison; evaluate() performs the
        // confidence-bound decision.
    }

    #[test]
    fn estimator_evaluate_uses_statistical_decision() {
        let estimator =
            QuantumVolumeEstimator::new(8, 8);

        let result =
            estimator
                .evaluate(10_000, 7_000)
                .expect("evaluation must succeed");

        assert!(result.passed);
        assert_eq!(
            result.quantum_volume,
            Some(256)
        );
    }

    #[test]
    fn estimator_probability_path_is_available() {
        let estimator =
            QuantumVolumeEstimator::new(8, 8);

        let result =
            estimator
                .evaluate_probability(
                    10_000,
                    0.70,
                )
                .expect("evaluation must succeed");

        assert_eq!(
            result.heavy_output_probability,
            0.70
        );
    }

    #[test]
    fn custom_threshold_is_respected() {
        let estimator =
            QuantumVolumeEstimator::with_threshold(
                8,
                8,
                0.90,
            )
            .expect("custom threshold must be valid");

        let result =
            estimator
                .evaluate(10_000, 7_000)
                .expect("evaluation must succeed");

        assert!(
            !result.passed
        );

        assert_eq!(
            result.quantum_volume,
            None
        );
    }

    #[test]
    fn checked_volume_rejects_impossible_shift() {
        let result =
            checked_quantum_volume(
                MAX_USIZE_EXPONENT + 1
            );

        assert_eq!(
            result,
            Err(
                QuantumVolumeError::ExponentOverflow {
                    exponent: MAX_USIZE_EXPONENT + 1,
                }
            )
        );
    }

    #[test]
    fn estimator_does_not_print_on_invalid_mutated_state() {
        let mut estimator =
            QuantumVolumeEstimator::new(4, 4);

        estimator.num_qubits = 0;

        assert_eq!(
            estimator.estimate_quantum_volume(),
            0
        );

        assert!(
            estimator
                .try_estimate_quantum_volume()
                .is_err()
        );
    }

    #[test]
    fn decision_identifier_is_stable() {
        assert_eq!(
            QuantumVolumeDecision::Passed.as_str(),
            "passed"
        );

        assert_eq!(
            QuantumVolumeDecision::Failed.as_str(),
            "failed"
        );
    }

    #[test]
    fn confidence_method_identifier_is_stable() {
        assert_eq!(
            ConfidenceIntervalMethod::Wilson.as_str(),
            "wilson"
        );
    }

    #[test]
    fn probability_to_count_is_bounded() {
        assert_eq!(
            probability_to_nearest_count(
                0.0,
                1_000
            )
            .expect("conversion must succeed"),
            0
        );

        assert_eq!(
            probability_to_nearest_count(
                1.0,
                1_000
            )
            .expect("conversion must succeed"),
            1_000
        );

        assert_eq!(
            probability_to_nearest_count(
                0.5,
                1_001
            )
            .expect("conversion must succeed"),
            501
        );
    }
}