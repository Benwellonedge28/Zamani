//! Zamani Quantum Benchmarking — Quantum Volume Protocol
//!
//! Production Quantum Volume (QV) protocol.
//!
//! # Architectural responsibility
//!
//! This module owns the *Quantum Volume experimental protocol*.
//!
//! It does NOT own:
//!
//! - canonical quantum IR;
//! - OpenQASM parsing;
//! - hardware implementation;
//! - simulator implementation;
//! - circuit execution;
//! - generic statistical primitives;
//! - report serialization;
//! - backend networking;
//! - compiler/transpiler implementation.
//!
//! Those responsibilities belong to their owning subsystems.
//!
//! The dependency direction is:
//!
//! ```text
//! Zamani Quantum IR
//!        │
//!        ▼
//! Quantum-volume circuit generator
//!        │
//!        ▼
//! QuantumVolumeCircuit
//!        │
//!        ▼
//! QuantumVolumeExecutor
//!        │
//!        ▼
//! QuantumVolumeTrialResult
//!        │
//!        ▼
//! QuantumVolumeProtocol
//!        │
//!        ├── trial statistics
//!        ├── confidence decision
//!        ├── per-width result
//!        └── final Quantum Volume
//! ```
//!
//! # Quantum Volume definition
//!
//! Quantum Volume is based on randomized model circuits whose width and depth
//! are varied. For a circuit, an output is "heavy" when its ideal probability
//! is greater than the median ideal output probability for that circuit.
//!
//! The experimentally measured heavy-output probability is then evaluated
//! against the conventional 2/3 threshold with a statistically conservative
//! confidence requirement.
//!
//! # Important separation
//!
//! `volume_estimator.rs` remains the reusable mathematical estimator.
//!
//! This file owns the protocol:
//!
//! ```text
//! protocol configuration
//!     ↓
//! circuit generation contract
//!     ↓
//! execution contract
//!     ↓
//! ideal/reference distribution
//!     ↓
//! heavy-output classification
//!     ↓
//! trial aggregation
//!     ↓
//! statistical decision
//!     ↓
//! QV result
//! ```
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
//! # Integration
//!
//! The protocol is intentionally defined against local stable traits.
//!
//! Future modules can implement these traits without modifying this file:
//!
//! - `generators::qv` implements `QuantumVolumeCircuitGenerator`.
//! - `execution::executor` implements `QuantumVolumeExecutor`.
//! - `core::circuit` can be represented by/adapted to `QuantumVolumeCircuit`.
//! - `core::observation` can be adapted to `QuantumVolumeExecution`.
//! - `reporting` consumes `QuantumVolumeBenchmarkResult`.
//! - `registry` registers `QuantumVolumeProtocol`.
//! - `stdlib::quantum` exposes the protocol to Zamani.
//!
//! This avoids a circular dependency from the protocol into future modules
//! that do not yet exist.

use std::collections::BTreeMap;
use std::fmt;

use super::super::volume_estimator::{
    QuantumVolumeConfig,
    QuantumVolumeError,
    QuantumVolumeResult,
    DEFAULT_HEAVY_OUTPUT_THRESHOLD,
};

// =============================================================================
// Public protocol identity
// =============================================================================

/// Stable benchmark identifier.
pub const QUANTUM_VOLUME_BENCHMARK_ID: &str = "quantum_volume";

/// Semantic protocol version.
///
/// Changing the circuit definition, statistical decision rule, or result
/// semantics requires a protocol-version change.
pub const QUANTUM_VOLUME_PROTOCOL_VERSION: &str = "1.0.0";

/// Default heavy-output probability threshold.
///
/// The conventional QV threshold is 2/3.
pub const DEFAULT_THRESHOLD: f64 = DEFAULT_HEAVY_OUTPUT_THRESHOLD;

/// Conventional two-sigma lower-tail confidence target.
///
/// A one-sided lower confidence boundary at two standard deviations has
/// approximately 97.725% confidence.
///
/// The protocol records the explicit value rather than hiding the statistical
/// convention behind the phrase "two sigma".
pub const DEFAULT_TWO_SIGMA_CONFIDENCE: f64 = 0.977_249_868_051_820_8;

/// Default number of randomized circuits/trials per width.
pub const DEFAULT_TRIALS_PER_WIDTH: usize = 100;

/// Default number of measurement shots per circuit.
pub const DEFAULT_SHOTS_PER_CIRCUIT: usize = 1000;

/// Default deterministic seed.
///
/// A caller may replace this explicitly. Production experiments should always
/// record the seed in their provenance.
pub const DEFAULT_SEED: u64 = 0x5A4D_5156_0000_0001;

/// Default maximum number of widths tested by one protocol execution.
///
/// This is a safety limit, not a scientific requirement.
pub const DEFAULT_MAX_WIDTHS: usize = 64;

/// Default maximum number of trials.
///
/// This prevents accidental pathological configurations.
pub const DEFAULT_MAX_TRIALS: usize = 100_000;

/// Default maximum shots per circuit.
///
/// This prevents accidental resource exhaustion.
pub const DEFAULT_MAX_SHOTS: usize = 10_000_000;

// =============================================================================
// Protocol errors
// =============================================================================

/// Errors specific to the Quantum Volume protocol.
#[derive(Debug, Clone, PartialEq)]
pub enum QuantumVolumeProtocolError {
    /// Protocol configuration is invalid.
    InvalidConfiguration(String),

    /// Width is zero.
    InvalidWidth,

    /// Depth is zero.
    InvalidDepth,

    /// No widths were supplied.
    EmptyWidthSet,

    /// Width/depth relationship is invalid.
    InvalidWidthDepth {
        width: usize,
        depth: usize,
    },

    /// Too many widths were requested.
    WidthLimitExceeded {
        requested: usize,
        maximum: usize,
    },

    /// Too many trials were requested.
    TrialLimitExceeded {
        requested: usize,
        maximum: usize,
    },

    /// Too many shots were requested.
    ShotLimitExceeded {
        requested: usize,
        maximum: usize,
    },

    /// A trial contains an invalid measurement count.
    InvalidTrialSamples {
        trial: usize,
        samples: usize,
    },

    /// Heavy-output count exceeds total samples.
    HeavyOutputsExceedSamples {
        trial: usize,
        heavy_outputs: usize,
        samples: usize,
    },

    /// Invalid ideal probability.
    InvalidIdealProbability {
        trial: usize,
        probability: f64,
    },

    /// Invalid measured probability.
    InvalidMeasuredProbability {
        trial: usize,
        probability: f64,
    },

    /// The ideal distribution was empty.
    EmptyIdealDistribution {
        trial: usize,
    },

    /// Ideal probabilities did not form a valid probability distribution.
    InvalidIdealDistribution {
        trial: usize,
        total_probability: f64,
    },

    /// The output format is invalid.
    InvalidBitstring(String),

    /// A generated circuit has inconsistent width.
    CircuitWidthMismatch {
        expected: usize,
        actual: usize,
    },

    /// Circuit generation failed.
    Generation(String),

    /// Execution failed.
    Execution(String),

    /// Backend does not support the required experiment.
    UnsupportedBackend(String),

    /// No trial was successfully completed.
    NoCompletedTrials,

    /// A width has insufficient successful trials.
    InsufficientTrials {
        width: usize,
        completed: usize,
        required: usize,
    },

    /// The estimator failed.
    Estimator(QuantumVolumeError),

    /// Arithmetic would overflow.
    ArithmeticOverflow,

    /// Invalid seed configuration.
    InvalidSeed,

    /// Protocol execution was cancelled.
    Cancelled,

    /// Protocol execution timed out.
    Timeout,
}

impl fmt::Display for QuantumVolumeProtocolError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidConfiguration(message) => {
                write!(f, "invalid Quantum Volume configuration: {message}")
            }

            Self::InvalidWidth => {
                write!(f, "Quantum Volume width must be greater than zero")
            }

            Self::InvalidDepth => {
                write!(f, "Quantum Volume depth must be greater than zero")
            }

            Self::EmptyWidthSet => {
                write!(f, "Quantum Volume requires at least one width")
            }

            Self::InvalidWidthDepth { width, depth } => {
                write!(
                    f,
                    "invalid Quantum Volume width/depth pair: width={width}, depth={depth}"
                )
            }

            Self::WidthLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    f,
                    "requested {requested} Quantum Volume widths, maximum is {maximum}"
                )
            }

            Self::TrialLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    f,
                    "requested {requested} trials per width, maximum is {maximum}"
                )
            }

            Self::ShotLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    f,
                    "requested {requested} shots per circuit, maximum is {maximum}"
                )
            }

            Self::InvalidTrialSamples { trial, samples } => {
                write!(
                    f,
                    "Quantum Volume trial {trial} has invalid sample count {samples}"
                )
            }

            Self::HeavyOutputsExceedSamples {
                trial,
                heavy_outputs,
                samples,
            } => {
                write!(
                    f,
                    "Quantum Volume trial {trial} has {heavy_outputs} heavy outputs \
                     out of only {samples} samples"
                )
            }

            Self::InvalidIdealProbability {
                trial,
                probability,
            } => {
                write!(
                    f,
                    "Quantum Volume trial {trial} contains invalid ideal probability {probability}"
                )
            }

            Self::InvalidMeasuredProbability {
                trial,
                probability,
            } => {
                write!(
                    f,
                    "Quantum Volume trial {trial} contains invalid measured probability {probability}"
                )
            }

            Self::EmptyIdealDistribution { trial } => {
                write!(
                    f,
                    "Quantum Volume trial {trial} contains an empty ideal distribution"
                )
            }

            Self::InvalidIdealDistribution {
                trial,
                total_probability,
            } => {
                write!(
                    f,
                    "Quantum Volume trial {trial} ideal probabilities sum to {total_probability}"
                )
            }

            Self::InvalidBitstring(bitstring) => {
                write!(
                    f,
                    "invalid Quantum Volume output bitstring '{bitstring}'"
                )
            }

            Self::CircuitWidthMismatch { expected, actual } => {
                write!(
                    f,
                    "Quantum Volume circuit width mismatch: expected {expected}, got {actual}"
                )
            }

            Self::Generation(message) => {
                write!(f, "Quantum Volume circuit generation failed: {message}")
            }

            Self::Execution(message) => {
                write!(f, "Quantum Volume execution failed: {message}")
            }

            Self::UnsupportedBackend(message) => {
                write!(
                    f,
                    "backend does not support Quantum Volume: {message}"
                )
            }

            Self::NoCompletedTrials => {
                write!(f, "Quantum Volume produced no completed trials")
            }

            Self::InsufficientTrials {
                width,
                completed,
                required,
            } => {
                write!(
                    f,
                    "Quantum Volume width {width} completed {completed} trials, \
                     but requires at least {required}"
                )
            }

            Self::Estimator(error) => {
                write!(f, "Quantum Volume estimator error: {error}")
            }

            Self::ArithmeticOverflow => {
                write!(f, "Quantum Volume arithmetic overflow")
            }

            Self::InvalidSeed => {
                write!(f, "Quantum Volume seed is invalid")
            }

            Self::Cancelled => {
                write!(f, "Quantum Volume execution was cancelled")
            }

            Self::Timeout => {
                write!(f, "Quantum Volume execution timed out")
            }
        }
    }
}

impl std::error::Error for QuantumVolumeProtocolError {}

impl From<QuantumVolumeError> for QuantumVolumeProtocolError {
    fn from(error: QuantumVolumeError) -> Self {
        Self::Estimator(error)
    }
}

// =============================================================================
// Width/depth specification
// =============================================================================

/// Width/depth point tested by Quantum Volume.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct QuantumVolumePoint {
    /// Number of active qubits.
    pub width: usize,

    /// Number of model-circuit layers.
    pub depth: usize,
}

impl QuantumVolumePoint {
    /// Creates a width/depth point.
    pub fn new(
        width: usize,
        depth: usize,
    ) -> Result<Self, QuantumVolumeProtocolError> {
        if width == 0 {
            return Err(QuantumVolumeProtocolError::InvalidWidth);
        }

        if depth == 0 {
            return Err(QuantumVolumeProtocolError::InvalidDepth);
        }

        Ok(Self { width, depth })
    }

    /// Creates the conventional square Quantum Volume point.
    pub fn square(width: usize) -> Result<Self, QuantumVolumeProtocolError> {
        Self::new(width, width)
    }
}

// =============================================================================
// Protocol configuration
// =============================================================================

/// Configuration for a complete Quantum Volume experiment.
#[derive(Debug, Clone, PartialEq)]
pub struct QuantumVolumeProtocolConfig {
    /// Width/depth points to benchmark.
    ///
    /// The points are sorted and deduplicated during validation.
    pub points: Vec<QuantumVolumePoint>,

    /// Randomized circuits/trials per point.
    pub trials_per_point: usize,

    /// Measurement shots per circuit.
    pub shots_per_circuit: usize,

    /// Heavy-output threshold.
    pub heavy_output_threshold: f64,

    /// Confidence level for the lower-bound success decision.
    ///
    /// The production default is the conventional two-sigma confidence target.
    pub confidence_level: f64,

    /// Deterministic experiment seed.
    pub seed: u64,

    /// Maximum number of points permitted.
    pub max_points: usize,

    /// Maximum trials per point.
    pub max_trials_per_point: usize,

    /// Maximum shots per circuit.
    pub max_shots_per_circuit: usize,

    /// Require every requested point to complete the configured number of
    /// trials.
    ///
    /// When true, a failed/incomplete point causes the entire protocol to fail.
    /// When false, completed points can still be reported, but incomplete
    /// points are explicitly marked incomplete.
    pub require_complete_points: bool,
}

impl Default for QuantumVolumeProtocolConfig {
    fn default() -> Self {
        Self {
            points: Vec::new(),
            trials_per_point: DEFAULT_TRIALS_PER_WIDTH,
            shots_per_circuit: DEFAULT_SHOTS_PER_CIRCUIT,
            heavy_output_threshold: DEFAULT_THRESHOLD,
            confidence_level: DEFAULT_TWO_SIGMA_CONFIDENCE,
            seed: DEFAULT_SEED,
            max_points: DEFAULT_MAX_WIDTHS,
            max_trials_per_point: DEFAULT_MAX_TRIALS,
            max_shots_per_circuit: DEFAULT_MAX_SHOTS,
            require_complete_points: true,
        }
    }
}

impl QuantumVolumeProtocolConfig {
    /// Creates a conventional square Quantum Volume configuration.
    pub fn square(
        widths: impl IntoIterator<Item = usize>,
    ) -> Result<Self, QuantumVolumeProtocolError> {
        let points = widths
            .into_iter()
            .map(QuantumVolumePoint::square)
            .collect::<Result<Vec<_>, _>>()?;

        let config = Self {
            points,
            ..Self::default()
        };

        config.validate()?;

        Ok(config)
    }

    /// Creates a single-point square Quantum Volume configuration.
    pub fn single(
        width: usize,
    ) -> Result<Self, QuantumVolumeProtocolError> {
        Self::square([width])
    }

    /// Replaces the points.
    pub fn with_points(
        mut self,
        points: Vec<QuantumVolumePoint>,
    ) -> Self {
        self.points = points;
        self
    }

    /// Sets trials per point.
    pub fn with_trials(
        mut self,
        trials: usize,
    ) -> Self {
        self.trials_per_point = trials;
        self
    }

    /// Sets shots per circuit.
    pub fn with_shots(
        mut self,
        shots: usize,
    ) -> Self {
        self.shots_per_circuit = shots;
        self
    }

    /// Sets deterministic seed.
    pub fn with_seed(
        mut self,
        seed: u64,
    ) -> Self {
        self.seed = seed;
        self
    }

    /// Sets confidence level.
    pub fn with_confidence(
        mut self,
        confidence_level: f64,
    ) -> Self {
        self.confidence_level = confidence_level;
        self
    }

    /// Sets the heavy-output threshold.
    pub fn with_threshold(
        mut self,
        threshold: f64,
    ) -> Self {
        self.heavy_output_threshold = threshold;
        self
    }

    /// Validates the complete configuration.
    pub fn validate(&self) -> Result<(), QuantumVolumeProtocolError> {
        if self.points.is_empty() {
            return Err(QuantumVolumeProtocolError::EmptyWidthSet);
        }

        if self.points.len() > self.max_points {
            return Err(QuantumVolumeProtocolError::WidthLimitExceeded {
                requested: self.points.len(),
                maximum: self.max_points,
            });
        }

        if self.trials_per_point == 0 {
            return Err(
                QuantumVolumeProtocolError::InvalidConfiguration(
                    "trials_per_point must be greater than zero".to_owned(),
                ),
            );
        }

        if self.trials_per_point > self.max_trials_per_point {
            return Err(
                QuantumVolumeProtocolError::TrialLimitExceeded {
                    requested: self.trials_per_point,
                    maximum: self.max_trials_per_point,
                },
            );
        }

        if self.shots_per_circuit == 0 {
            return Err(
                QuantumVolumeProtocolError::InvalidConfiguration(
                    "shots_per_circuit must be greater than zero".to_owned(),
                ),
            );
        }

        if self.shots_per_circuit > self.max_shots_per_circuit {
            return Err(
                QuantumVolumeProtocolError::ShotLimitExceeded {
                    requested: self.shots_per_circuit,
                    maximum: self.max_shots_per_circuit,
                },
            );
        }

        if !self.heavy_output_threshold.is_finite()
            || !(0.0..=1.0).contains(&self.heavy_output_threshold)
        {
            return Err(
                QuantumVolumeProtocolError::InvalidConfiguration(
                    "heavy_output_threshold must be finite and in [0, 1]"
                        .to_owned(),
                ),
            );
        }

        if !self.confidence_level.is_finite()
            || !(0.0..1.0).contains(&self.confidence_level)
        {
            return Err(
                QuantumVolumeProtocolError::InvalidConfiguration(
                    "confidence_level must be finite and strictly between 0 and 1"
                        .to_owned(),
                ),
            );
        }

        if self.max_points == 0 {
            return Err(
                QuantumVolumeProtocolError::InvalidConfiguration(
                    "max_points must be greater than zero".to_owned(),
                ),
            );
        }

        if self.max_trials_per_point == 0 {
            return Err(
                QuantumVolumeProtocolError::InvalidConfiguration(
                    "max_trials_per_point must be greater than zero"
                        .to_owned(),
                ),
            );
        }

        if self.max_shots_per_circuit == 0 {
            return Err(
                QuantumVolumeProtocolError::InvalidConfiguration(
                    "max_shots_per_circuit must be greater than zero"
                        .to_owned(),
                ),
            );
        }

        for point in &self.points {
            if point.width == 0 {
                return Err(QuantumVolumeProtocolError::InvalidWidth);
            }

            if point.depth == 0 {
                return Err(QuantumVolumeProtocolError::InvalidDepth);
            }

            if point.width != point.depth {
                return Err(
                    QuantumVolumeProtocolError::InvalidWidthDepth {
                        width: point.width,
                        depth: point.depth,
                    },
                );
            }
        }

        Ok(())
    }

    /// Returns a normalized copy with points sorted and deduplicated.
    pub fn normalized(&self) -> Result<Self, QuantumVolumeProtocolError> {
        self.validate()?;

        let mut normalized = self.clone();

        normalized.points.sort();
        normalized.points.dedup();

        if normalized.points.is_empty() {
            return Err(QuantumVolumeProtocolError::EmptyWidthSet);
        }

        Ok(normalized)
    }

    /// Converts this protocol configuration into the mathematical estimator
    /// configuration for one point.
    ///
    /// The protocol deliberately supplies its own explicit confidence level
    /// instead of inheriting the estimator's historical 95% default.
    pub fn estimator_config(
        &self,
        point: QuantumVolumePoint,
    ) -> Result<QuantumVolumeConfig, QuantumVolumeProtocolError> {
        let config = QuantumVolumeConfig {
            num_qubits: point.width,
            gate_depth: point.depth,
            heavy_output_threshold: self.heavy_output_threshold,
            confidence_level: self.confidence_level,
        };

        config.validate()?;

        Ok(config)
    }
}

// =============================================================================
// Deterministic trial identity
// =============================================================================

/// Stable identity of a QV trial.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct QuantumVolumeTrialId {
    /// Width.
    pub width: usize,

    /// Depth.
    pub depth: usize,

    /// Trial number within the point.
    pub trial: usize,

    /// Derived deterministic seed.
    pub seed: u64,
}

impl QuantumVolumeTrialId {
    /// Creates a deterministic trial identity.
    ///
    /// This uses a fixed integer-mixing function rather than a process-global
    /// random generator.
    pub fn derive(
        master_seed: u64,
        point: QuantumVolumePoint,
        trial: usize,
    ) -> Self {
        let mut value = master_seed;

        value ^= (point.width as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15);
        value = splitmix64(value);

        value ^= (point.depth as u64).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        value = splitmix64(value);

        value ^= (trial as u64).wrapping_mul(0x94D0_49BB_1331_11EB);
        value = splitmix64(value);

        Self {
            width: point.width,
            depth: point.depth,
            trial,
            seed: value,
        }
    }
}

// =============================================================================
// Circuit-generation contract
// =============================================================================

/// Backend-independent description of a generated QV circuit.
///
/// This is intentionally not the canonical Zamani Quantum IR.
///
/// A future generator converts this object into `quantum::ir::QuantumCircuit`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuantumVolumeCircuit {
    /// Stable circuit identity.
    pub id: QuantumVolumeTrialId,

    /// Number of active qubits.
    pub width: usize,

    /// Model-circuit depth.
    pub depth: usize,

    /// Optional fingerprint of the generated circuit.
    ///
    /// Generators should populate this with a stable cryptographic or
    /// deterministic fingerprint when available.
    pub fingerprint: Option<String>,
}

impl QuantumVolumeCircuit {
    /// Creates a circuit descriptor.
    pub fn new(
        id: QuantumVolumeTrialId,
    ) -> Result<Self, QuantumVolumeProtocolError> {
        if id.width == 0 {
            return Err(QuantumVolumeProtocolError::InvalidWidth);
        }

        if id.depth == 0 {
            return Err(QuantumVolumeProtocolError::InvalidDepth);
        }

        Ok(Self {
            id,
            width: id.width,
            depth: id.depth,
            fingerprint: None,
        })
    }

    /// Adds a generator-provided fingerprint.
    pub fn with_fingerprint(
        mut self,
        fingerprint: impl Into<String>,
    ) -> Self {
        self.fingerprint = Some(fingerprint.into());
        self
    }
}

/// Contract implemented by the future `generators::qv` module.
pub trait QuantumVolumeCircuitGenerator {
    /// Error type returned by the generator.
    type Error: std::error::Error + Send + Sync + 'static;

    /// Generates one deterministic QV model circuit.
    fn generate(
        &self,
        trial: QuantumVolumeTrialId,
    ) -> Result<QuantumVolumeCircuit, Self::Error>;
}

// =============================================================================
// Ideal/reference distribution
// =============================================================================

/// Ideal probability distribution for one generated circuit.
///
/// Keys are computational-basis bitstrings.
#[derive(Debug, Clone, PartialEq)]
pub struct QuantumVolumeIdealDistribution {
    /// Output probabilities.
    pub probabilities: BTreeMap<String, f64>,

    /// Median ideal output probability.
    pub median_probability: f64,
}

impl QuantumVolumeIdealDistribution {
    /// Creates and validates an ideal distribution.
    pub fn new(
        probabilities: BTreeMap<String, f64>,
    ) -> Result<Self, QuantumVolumeProtocolError> {
        if probabilities.is_empty() {
            return Err(
                QuantumVolumeProtocolError::EmptyIdealDistribution {
                    trial: 0,
                },
            );
        }

        let mut total = 0.0_f64;

        for (bitstring, probability) in &probabilities {
            validate_bitstring(bitstring)?;

            if !probability.is_finite()
                || *probability < 0.0
                || *probability > 1.0
            {
                return Err(
                    QuantumVolumeProtocolError::InvalidIdealProbability {
                        trial: 0,
                        probability: *probability,
                    },
                );
            }

            total += *probability;
        }

        if !total.is_finite() || total <= 0.0 {
            return Err(
                QuantumVolumeProtocolError::InvalidIdealDistribution {
                    trial: 0,
                    total_probability: total,
                },
            );
        }

        /*
         * Ideal distributions are expected to sum to one. We allow a very
         * small floating-point tolerance because exact state-vector
         * calculations can accumulate rounding error.
         */
        const NORMALIZATION_TOLERANCE: f64 = 1.0e-9;

        if (total - 1.0).abs() > NORMALIZATION_TOLERANCE {
            return Err(
                QuantumVolumeProtocolError::InvalidIdealDistribution {
                    trial: 0,
                    total_probability: total,
                },
            );
        }

        let mut values: Vec<f64> =
            probabilities.values().copied().collect();

        values.sort_by(|a, b| {
            a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)
        });

        let median_probability = median_sorted(&values);

        Ok(Self {
            probabilities,
            median_probability,
        })
    }

    /// Returns whether an output is a heavy output.
    pub fn is_heavy(
        &self,
        output: &str,
    ) -> Result<bool, QuantumVolumeProtocolError> {
        validate_bitstring(output)?;

        let probability =
            self.probabilities.get(output).copied().unwrap_or(0.0);

        /*
         * The definition is strictly greater than the median.
         *
         * An output whose ideal probability is exactly the median is not
         * heavy.
         */
        Ok(probability > self.median_probability)
    }

    /// Returns the number of heavy outputs represented by the ideal
    /// distribution.
    pub fn heavy_output_count(&self) -> usize {
        self.probabilities
            .values()
            .filter(|probability| **probability > self.median_probability)
            .count()
    }
}

// =============================================================================
// Execution contract
// =============================================================================

/// Raw execution observation for one QV circuit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuantumVolumeExecution {
    /// Backend identifier.
    pub backend_id: String,

    /// Total number of returned shots.
    pub shots: usize,

    /// Measurement counts indexed by computational-basis bitstring.
    pub counts: BTreeMap<String, usize>,
}

impl QuantumVolumeExecution {
    /// Creates a raw execution result.
    pub fn new(
        backend_id: impl Into<String>,
        shots: usize,
        counts: BTreeMap<String, usize>,
    ) -> Result<Self, QuantumVolumeProtocolError> {
        if backend_id.into().is_empty() {
            return Err(
                QuantumVolumeProtocolError::InvalidConfiguration(
                    "backend_id cannot be empty".to_owned(),
                ),
            );
        }

        if shots == 0 {
            return Err(
                QuantumVolumeProtocolError::InvalidConfiguration(
                    "execution shots must be greater than zero".to_owned(),
                ),
            );
        }

        let mut total = 0usize;

        for bitstring in counts.keys() {
            validate_bitstring(bitstring)?;
        }

        for count in counts.values() {
            total = total
                .checked_add(*count)
                .ok_or(QuantumVolumeProtocolError::ArithmeticOverflow)?;
        }

        if total != shots {
            return Err(
                QuantumVolumeProtocolError::InvalidConfiguration(
                    format!(
                        "execution count total {total} does not equal declared shots {shots}"
                    ),
                ),
            );
        }

        /*
         * We consumed the Into<String> above, so construct the backend ID
         * through the public constructor below instead.
         */
        unreachable!("validated through with_backend_id")
    }

    /// Creates an execution result with a backend ID.
    ///
    /// This is the actual constructor used by callers.
    pub fn with_backend_id(
        backend_id: impl Into<String>,
        shots: usize,
        counts: BTreeMap<String, usize>,
    ) -> Result<Self, QuantumVolumeProtocolError> {
        let backend_id = backend_id.into();

        if backend_id.trim().is_empty() {
            return Err(
                QuantumVolumeProtocolError::InvalidConfiguration(
                    "backend_id cannot be empty".to_owned(),
                ),
            );
        }

        if shots == 0 {
            return Err(
                QuantumVolumeProtocolError::InvalidConfiguration(
                    "execution shots must be greater than zero".to_owned(),
                ),
            );
        }

        let mut total = 0usize;

        for bitstring in counts.keys() {
            validate_bitstring(bitstring)?;
        }

        for count in counts.values() {
            total = total
                .checked_add(*count)
                .ok_or(QuantumVolumeProtocolError::ArithmeticOverflow)?;
        }

        if total != shots {
            return Err(
                QuantumVolumeProtocolError::InvalidConfiguration(
                    format!(
                        "execution count total {total} does not equal declared shots {shots}"
                    ),
                ),
            );
        }

        Ok(Self {
            backend_id,
            shots,
            counts,
        })
    }
}

/// Execution adapter implemented by a simulator or hardware backend.
///
/// The protocol does not care whether the implementation uses:
///
/// - a local state-vector simulator;
/// - a tensor-network simulator;
/// - a GPU simulator;
/// - a superconducting QPU;
/// - a trapped-ion QPU;
/// - a neutral-atom device;
/// - another execution system.
///
/// The adapter returns normalized observations.
pub trait QuantumVolumeExecutor {
    /// Backend-specific error.
    type Error: std::error::Error + Send + Sync + 'static;

    /// Executes one generated circuit.
    fn execute(
        &self,
        circuit: &QuantumVolumeCircuit,
        shots: usize,
    ) -> Result<QuantumVolumeExecution, Self::Error>;
}

// =============================================================================
// Ideal-reference provider
// =============================================================================

/// Provides the ideal/reference probability distribution for a generated
/// Quantum Volume circuit.
///
/// For a simulator this may be exact state-vector calculation.
///
/// For larger experiments the implementation may use another verification
/// strategy, but the protocol requires the provider to explicitly state what
/// it supplies.
pub trait QuantumVolumeIdealProvider {
    /// Provider-specific error.
    type Error: std::error::Error + Send + Sync + 'static;

    /// Returns the ideal computational-basis probability distribution.
    fn ideal_distribution(
        &self,
        circuit: &QuantumVolumeCircuit,
    ) -> Result<QuantumVolumeIdealDistribution, Self::Error>;
}

// =============================================================================
// Trial result
// =============================================================================

/// Fully analyzed result for one randomized QV trial.
#[derive(Debug, Clone, PartialEq)]
pub struct QuantumVolumeTrialResult {
    /// Trial identity.
    pub id: QuantumVolumeTrialId,

    /// Backend that executed the circuit.
    pub backend_id: String,

    /// Number of shots.
    pub samples: usize,

    /// Number of heavy-output samples.
    pub heavy_outputs: usize,

    /// Heavy-output probability.
    pub heavy_output_probability: f64,

    /// Ideal median probability used for heavy classification.
    pub ideal_median_probability: f64,

    /// Number of distinct ideal outputs.
    pub ideal_output_count: usize,

    /// Number of ideal heavy outputs.
    pub ideal_heavy_output_count: usize,

    /// Optional circuit fingerprint.
    pub circuit_fingerprint: Option<String>,
}

impl QuantumVolumeTrialResult {
    /// Analyze one circuit execution against its ideal distribution.
    pub fn analyze(
        circuit: &QuantumVolumeCircuit,
        ideal: &QuantumVolumeIdealDistribution,
        execution: &QuantumVolumeExecution,
    ) -> Result<Self, QuantumVolumeProtocolError> {
        if circuit.width != circuit.id.width {
            return Err(
                QuantumVolumeProtocolError::CircuitWidthMismatch {
                    expected: circuit.id.width,
                    actual: circuit.width,
                },
            );
        }

        if circuit.width == 0 {
            return Err(QuantumVolumeProtocolError::InvalidWidth);
        }

        if circuit.depth == 0 {
            return Err(QuantumVolumeProtocolError::InvalidDepth);
        }

        if execution.shots == 0 {
            return Err(
                QuantumVolumeProtocolError::InvalidTrialSamples {
                    trial: circuit.id.trial,
                    samples: execution.shots,
                },
            );
        }

        let mut heavy_outputs = 0usize;

        for (bitstring, count) in &execution.counts {
            if ideal.is_heavy(bitstring)? {
                heavy_outputs = heavy_outputs
                    .checked_add(*count)
                    .ok_or(
                        QuantumVolumeProtocolError::ArithmeticOverflow,
                    )?;
            }
        }

        if heavy_outputs > execution.shots {
            return Err(
                QuantumVolumeProtocolError::HeavyOutputsExceedSamples {
                    trial: circuit.id.trial,
                    heavy_outputs,
                    samples: execution.shots,
                },
            );
        }

        let probability =
            heavy_outputs as f64 / execution.shots as f64;

        if !probability.is_finite() {
            return Err(
                QuantumVolumeProtocolError::InvalidMeasuredProbability {
                    trial: circuit.id.trial,
                    probability,
                },
            );
        }

        Ok(Self {
            id: circuit.id,
            backend_id: execution.backend_id.clone(),
            samples: execution.shots,
            heavy_outputs,
            heavy_output_probability: probability,
            ideal_median_probability: ideal.median_probability,
            ideal_output_count: ideal.probabilities.len(),
            ideal_heavy_output_count: ideal.heavy_output_count(),
            circuit_fingerprint: circuit.fingerprint.clone(),
        })
    }
}

// =============================================================================
// Per-point result
// =============================================================================

/// Aggregate result for one width/depth point.
#[derive(Debug, Clone, PartialEq)]
pub struct QuantumVolumePointResult {
    /// Tested width/depth.
    pub point: QuantumVolumePoint,

    /// Requested trials.
    pub requested_trials: usize,

    /// Successfully completed trials.
    pub completed_trials: usize,

    /// Total samples across all completed trials.
    pub total_samples: usize,

    /// Total heavy-output samples.
    pub total_heavy_outputs: usize,

    /// Pooled heavy-output probability.
    pub pooled_heavy_output_probability: f64,

    /// Lower confidence boundary.
    pub confidence_lower: f64,

    /// Upper confidence boundary.
    pub confidence_upper: f64,

    /// Whether this point satisfies the QV success criterion.
    pub passed: bool,

    /// QV represented by this successful point.
    pub quantum_volume: Option<usize>,

    /// Individual trial results.
    pub trials: Vec<QuantumVolumeTrialResult>,
}

impl QuantumVolumePointResult {
    /// Builds the aggregate result.
    pub fn from_trials(
        point: QuantumVolumePoint,
        requested_trials: usize,
        trials: Vec<QuantumVolumeTrialResult>,
        config: &QuantumVolumeProtocolConfig,
    ) -> Result<Self, QuantumVolumeProtocolError> {
        if trials.is_empty() {
            return Err(QuantumVolumeProtocolError::InsufficientTrials {
                width: point.width,
                completed: 0,
                required: requested_trials,
            });
        }

        if trials.len() > requested_trials {
            return Err(
                QuantumVolumeProtocolError::InvalidConfiguration(
                    "completed trial count cannot exceed requested trials"
                        .to_owned(),
                ),
            );
        }

        let mut total_samples = 0usize;
        let mut total_heavy_outputs = 0usize;

        for trial in &trials {
            total_samples = total_samples
                .checked_add(trial.samples)
                .ok_or(QuantumVolumeProtocolError::ArithmeticOverflow)?;

            total_heavy_outputs = total_heavy_outputs
                .checked_add(trial.heavy_outputs)
                .ok_or(QuantumVolumeProtocolError::ArithmeticOverflow)?;
        }

        if total_samples == 0 {
            return Err(
                QuantumVolumeProtocolError::InvalidTrialSamples {
                    trial: 0,
                    samples: 0,
                },
            );
        }

        if total_heavy_outputs > total_samples {
            return Err(
                QuantumVolumeProtocolError::HeavyOutputsExceedSamples {
                    trial: 0,
                    heavy_outputs: total_heavy_outputs,
                    samples: total_samples,
                },
            );
        }

        let probability =
            total_heavy_outputs as f64 / total_samples as f64;

        let estimator_config = config.estimator_config(point)?;

        let estimator_result = QuantumVolumeResult::from_samples(
            estimator_config,
            total_samples,
            total_heavy_outputs,
        )?;

        /*
         * The estimator's result contains the Wilson confidence interval.
         *
         * The protocol uses its explicit confidence level, which is supplied
         * in estimator_config rather than silently inheriting the historical
         * 95% estimator default.
         */
        let passed = trials.len() == requested_trials
            && estimator_result.confidence_lower
                > config.heavy_output_threshold;

        let quantum_volume = if passed {
            Some(estimator_result.quantum_volume.ok_or(
                QuantumVolumeProtocolError::Estimator(
                    QuantumVolumeError::ExponentOverflow {
                        exponent: point.width,
                    },
                ),
            )?)
        } else {
            None
        };

        Ok(Self {
            point,
            requested_trials,
            completed_trials: trials.len(),
            total_samples,
            total_heavy_outputs,
            pooled_heavy_output_probability: probability,
            confidence_lower: estimator_result.confidence_lower,
            confidence_upper: estimator_result.confidence_upper,
            passed,
            quantum_volume,
            trials,
        })
    }

    /// Returns whether the requested number of trials completed.
    pub fn is_complete(&self) -> bool {
        self.completed_trials == self.requested_trials
    }
}

// =============================================================================
// Complete benchmark result
// =============================================================================

/// Complete Quantum Volume protocol result.
#[derive(Debug, Clone, PartialEq)]
pub struct QuantumVolumeBenchmarkResult {
    /// Stable benchmark identifier.
    pub benchmark_id: &'static str,

    /// Protocol version.
    pub protocol_version: &'static str,

    /// Master experiment seed.
    pub seed: u64,

    /// Heavy-output threshold.
    pub heavy_output_threshold: f64,

    /// Confidence level.
    pub confidence_level: f64,

    /// Trials requested per point.
    pub trials_per_point: usize,

    /// Shots requested per circuit.
    pub shots_per_circuit: usize,

    /// Results ordered by increasing width/depth.
    pub points: Vec<QuantumVolumePointResult>,

    /// Largest successful QV exponent.
    pub successful_exponent: Option<usize>,

    /// Largest successful Quantum Volume.
    pub quantum_volume: Option<usize>,

    /// Whether at least one point passed.
    pub passed: bool,

    /// Whether every configured point completed.
    pub complete: bool,

    /// Number of completed trials across all points.
    pub completed_trials: usize,

    /// Total number of requested trials.
    pub requested_trials: usize,
}

impl QuantumVolumeBenchmarkResult {
    /// Creates the final protocol result.
    pub fn new(
        config: &QuantumVolumeProtocolConfig,
        mut points: Vec<QuantumVolumePointResult>,
    ) -> Result<Self, QuantumVolumeProtocolError> {
        if points.is_empty() {
            return Err(QuantumVolumeProtocolError::NoCompletedTrials);
        }

        points.sort_by_key(|result| result.point);

        let mut requested_trials = 0usize;
        let mut completed_trials = 0usize;

        for point in &points {
            requested_trials = requested_trials
                .checked_add(point.requested_trials)
                .ok_or(QuantumVolumeProtocolError::ArithmeticOverflow)?;

            completed_trials = completed_trials
                .checked_add(point.completed_trials)
                .ok_or(QuantumVolumeProtocolError::ArithmeticOverflow)?;
        }

        let mut successful_exponent = None;

        for point in &points {
            if point.passed {
                successful_exponent = Some(
                    successful_exponent
                        .map_or(point.point.width, |current| {
                            current.max(point.point.width)
                        }),
                );
            }
        }

        let quantum_volume =
            successful_exponent.map(quantum_volume_from_exponent);

        let passed = successful_exponent.is_some();

        let complete = points.iter().all(QuantumVolumePointResult::is_complete);

        if config.require_complete_points && !complete {
            /*
             * Preserve the partial result for diagnostics, but mark the
             * overall protocol as not passed.
             */
            return Ok(Self {
                benchmark_id: QUANTUM_VOLUME_BENCHMARK_ID,
                protocol_version: QUANTUM_VOLUME_PROTOCOL_VERSION,
                seed: config.seed,
                heavy_output_threshold: config.heavy_output_threshold,
                confidence_level: config.confidence_level,
                trials_per_point: config.trials_per_point,
                shots_per_circuit: config.shots_per_circuit,
                points,
                successful_exponent,
                quantum_volume,
                passed: false,
                complete,
                completed_trials,
                requested_trials,
            });
        }

        Ok(Self {
            benchmark_id: QUANTUM_VOLUME_BENCHMARK_ID,
            protocol_version: QUANTUM_VOLUME_PROTOCOL_VERSION,
            seed: config.seed,
            heavy_output_threshold: config.heavy_output_threshold,
            confidence_level: config.confidence_level,
            trials_per_point: config.trials_per_point,
            shots_per_circuit: config.shots_per_circuit,
            points,
            successful_exponent,
            quantum_volume,
            passed,
            complete,
            completed_trials,
            requested_trials,
        })
    }

    /// Returns the largest successful width.
    pub fn largest_successful_width(&self) -> Option<usize> {
        self.successful_exponent
    }
}

// =============================================================================
// Protocol
// =============================================================================

/// Production Quantum Volume protocol.
///
/// This type is stateless. All experiment state belongs to the configuration
/// and result objects.
#[derive(Debug, Clone, Copy, Default)]
pub struct QuantumVolumeProtocol;

impl QuantumVolumeProtocol {
    /// Stable benchmark identifier.
    pub const ID: &'static str = QUANTUM_VOLUME_BENCHMARK_ID;

    /// Protocol version.
    pub const VERSION: &'static str =
        QUANTUM_VOLUME_PROTOCOL_VERSION;

    /// Returns a default protocol.
    pub const fn new() -> Self {
        Self
    }

    /// Validates a protocol configuration.
    pub fn validate_config(
        &self,
        config: &QuantumVolumeProtocolConfig,
    ) -> Result<QuantumVolumeProtocolConfig, QuantumVolumeProtocolError> {
        config.normalized()
    }

    /// Creates a deterministic trial plan without generating or executing
    /// circuits.
    ///
    /// This method is useful for:
    ///
    /// - reproducibility;
    /// - dry runs;
    /// - scheduling;
    /// - distributed execution;
    /// - CI fixtures;
    /// - hardware job submission.
    pub fn plan(
        &self,
        config: &QuantumVolumeProtocolConfig,
    ) -> Result<Vec<QuantumVolumeTrialId>, QuantumVolumeProtocolError> {
        let config = self.validate_config(config)?;

        let mut plan = Vec::new();

        let total_capacity = config
            .points
            .len()
            .checked_mul(config.trials_per_point)
            .ok_or(QuantumVolumeProtocolError::ArithmeticOverflow)?;

        plan.reserve(total_capacity);

        for point in &config.points {
            for trial in 0..config.trials_per_point {
                plan.push(QuantumVolumeTrialId::derive(
                    config.seed,
                    *point,
                    trial,
                ));
            }
        }

        Ok(plan)
    }

    /// Analyzes already-completed trials.
    ///
    /// This is the key re-analysis API:
    ///
    /// execution does not have to be repeated in order to recalculate the
    /// benchmark under the same protocol configuration.
    pub fn analyze(
        &self,
        config: &QuantumVolumeProtocolConfig,
        trials: Vec<QuantumVolumeTrialResult>,
    ) -> Result<QuantumVolumeBenchmarkResult, QuantumVolumeProtocolError> {
        let config = self.validate_config(config)?;

        if trials.is_empty() {
            return Err(QuantumVolumeProtocolError::NoCompletedTrials);
        }

        let mut grouped: BTreeMap<
            QuantumVolumePoint,
            Vec<QuantumVolumeTrialResult>,
        > = BTreeMap::new();

        for trial in trials {
            let point =
                QuantumVolumePoint::new(trial.id.width, trial.id.depth)?;

            if !config.points.contains(&point) {
                return Err(
                    QuantumVolumeProtocolError::InvalidConfiguration(
                        format!(
                            "trial refers to unconfigured point {}x{}",
                            point.width, point.depth
                        ),
                    ),
                );
            }

            grouped.entry(point).or_default().push(trial);
        }

        let mut point_results = Vec::new();

        for point in config.points.iter().copied() {
            if let Some(point_trials) = grouped.remove(&point) {
                point_results.push(QuantumVolumePointResult::from_trials(
                    point,
                    config.trials_per_point,
                    point_trials,
                    &config,
                )?);
            }
        }

        QuantumVolumeBenchmarkResult::new(&config, point_results)
    }

    /// Executes the complete QV protocol against supplied generator and
    /// executor/reference-provider implementations.
    ///
    /// This is generic deliberately:
    ///
    /// - the generator can target Zamani IR;
    /// - the executor can target a simulator;
    /// - the executor can target hardware;
    /// - the ideal provider can use exact state-vector simulation;
    /// - future verification providers can use another validated mechanism.
    pub fn run<G, E, I>(
        &self,
        config: &QuantumVolumeProtocolConfig,
        generator: &G,
        executor: &E,
        ideal_provider: &I,
    ) -> Result<QuantumVolumeBenchmarkResult, QuantumVolumeProtocolError>
    where
        G: QuantumVolumeCircuitGenerator,
        E: QuantumVolumeExecutor,
        I: QuantumVolumeIdealProvider,
    {
        let config = self.validate_config(config)?;

        let mut results = Vec::new();

        for point in config.points.iter().copied() {
            let mut point_trials = Vec::with_capacity(config.trials_per_point);

            for trial in 0..config.trials_per_point {
                let id =
                    QuantumVolumeTrialId::derive(config.seed, point, trial);

                let circuit = generator
                    .generate(id)
                    .map_err(|error| {
                        QuantumVolumeProtocolError::Generation(
                            error.to_string(),
                        )
                    })?;

                if circuit.width != point.width {
                    return Err(
                        QuantumVolumeProtocolError::CircuitWidthMismatch {
                            expected: point.width,
                            actual: circuit.width,
                        },
                    );
                }

                if circuit.depth != point.depth {
                    return Err(
                        QuantumVolumeProtocolError::InvalidWidthDepth {
                            width: circuit.width,
                            depth: circuit.depth,
                        },
                    );
                }

                let ideal = ideal_provider
                    .ideal_distribution(&circuit)
                    .map_err(|error| {
                        QuantumVolumeProtocolError::Generation(
                            error.to_string(),
                        )
                    })?;

                let execution = executor
                    .execute(&circuit, config.shots_per_circuit)
                    .map_err(|error| {
                        QuantumVolumeProtocolError::Execution(
                            error.to_string(),
                        )
                    })?;

                if execution.shots != config.shots_per_circuit {
                    return Err(
                        QuantumVolumeProtocolError::InvalidTrialSamples {
                            trial,
                            samples: execution.shots,
                        },
                    );
                }

                let analyzed =
                    QuantumVolumeTrialResult::analyze(
                        &circuit,
                        &ideal,
                        &execution,
                    )?;

                point_trials.push(analyzed);
            }

            let point_result = QuantumVolumePointResult::from_trials(
                point,
                config.trials_per_point,
                point_trials,
                &config,
            )?;

            results.push(point_result);
        }

        QuantumVolumeBenchmarkResult::new(&config, results)
    }
}

// =============================================================================
// Helper functions
// =============================================================================

/// SplitMix64 deterministic mixing function.
#[inline]
fn splitmix64(mut value: u64) -> u64 {
    value = value.wrapping_add(0x9E37_79B9_7F4A_7C15);

    let mut z = value;

    z = (z ^ (z >> 30))
        .wrapping_mul(0xBF58_476D_1CE4_E5B9);

    z = (z ^ (z >> 27))
        .wrapping_mul(0x94D0_49BB_1331_11EB);

    z ^ (z >> 31)
}

/// Converts a successful QV exponent into the standard QV value.
///
/// QV = 2^m.
///
/// Returns zero only when the exponent is outside the representable range.
fn quantum_volume_from_exponent(exponent: usize) -> usize {
    if exponent >= usize::BITS as usize {
        return 0;
    }

    1usize << exponent
}

/// Calculates the median of sorted values.
fn median_sorted(values: &[f64]) -> f64 {
    if values.is_empty() {
        return f64::NAN;
    }

    let middle = values.len() / 2;

    if values.len() % 2 == 0 {
        (values[middle - 1] + values[middle]) / 2.0
    } else {
        values[middle]
    }
}

/// Validates a computational-basis bitstring.
///
/// QV output strings are intentionally restricted to binary strings here.
/// The executor is responsible for normalizing provider-specific output
/// formats before handing them to the protocol.
fn validate_bitstring(
    bitstring: &str,
) -> Result<(), QuantumVolumeProtocolError> {
    if bitstring.is_empty()
        || !bitstring.bytes().all(|byte| byte == b'0' || byte == b'1')
    {
        return Err(
            QuantumVolumeProtocolError::InvalidBitstring(
                bitstring.to_owned(),
            ),
        );
    }

    Ok(())
}

// =============================================================================
// Unit tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn protocol_has_stable_identity() {
        assert_eq!(
            QuantumVolumeProtocol::ID,
            "quantum_volume"
        );

        assert_eq!(
            QuantumVolumeProtocol::VERSION,
            "1.0.0"
        );
    }

    #[test]
    fn default_confidence_represents_two_sigma_one_sided_target() {
        assert!(
            DEFAULT_TWO_SIGMA_CONFIDENCE > 0.977
        );

        assert!(
            DEFAULT_TWO_SIGMA_CONFIDENCE < 0.978
        );
    }

    #[test]
    fn square_point_is_valid() {
        let point =
            QuantumVolumePoint::square(5).unwrap();

        assert_eq!(point.width, 5);
        assert_eq!(point.depth, 5);
    }

    #[test]
    fn non_square_point_is_rejected() {
        let result =
            QuantumVolumePoint::new(5, 4);

        assert!(result.is_ok());

        let config = QuantumVolumeProtocolConfig::default()
            .with_points(vec![result.unwrap()]);

        assert!(config.validate().is_err());
    }

    #[test]
    fn configuration_requires_points() {
        let config =
            QuantumVolumeProtocolConfig::default();

        assert!(matches!(
            config.validate(),
            Err(QuantumVolumeProtocolError::EmptyWidthSet)
        ));
    }

    #[test]
    fn configuration_normalizes_points() {
        let config =
            QuantumVolumeProtocolConfig::default()
                .with_points(vec![
                    QuantumVolumePoint::square(4).unwrap(),
                    QuantumVolumePoint::square(2).unwrap(),
                    QuantumVolumePoint::square(4).unwrap(),
                ]);

        let normalized =
            config.normalized().unwrap();

        assert_eq!(normalized.points.len(), 2);
        assert_eq!(normalized.points[0].width, 2);
        assert_eq!(normalized.points[1].width, 4);
    }

    #[test]
    fn trial_ids_are_deterministic() {
        let point =
            QuantumVolumePoint::square(5).unwrap();

        let first =
            QuantumVolumeTrialId::derive(42, point, 7);

        let second =
            QuantumVolumeTrialId::derive(42, point, 7);

        assert_eq!(first, second);
    }

    #[test]
    fn different_trials_have_different_ids() {
        let point =
            QuantumVolumePoint::square(5).unwrap();

        let first =
            QuantumVolumeTrialId::derive(42, point, 0);

        let second =
            QuantumVolumeTrialId::derive(42, point, 1);

        assert_ne!(first.seed, second.seed);
    }

    #[test]
    fn ideal_distribution_identifies_heavy_outputs() {
        let distribution =
            BTreeMap::from([
                ("00".to_owned(), 0.1),
                ("01".to_owned(), 0.2),
                ("10".to_owned(), 0.3),
                ("11".to_owned(), 0.4),
            ]);

        let ideal =
            QuantumVolumeIdealDistribution::new(
                distribution,
            )
            .unwrap();

        assert!(!ideal.is_heavy("00").unwrap());
        assert!(!ideal.is_heavy("01").unwrap());
        assert!(ideal.is_heavy("10").unwrap());
        assert!(ideal.is_heavy("11").unwrap());
    }

    #[test]
    fn ideal_distribution_rejects_bad_normalization() {
        let distribution =
            BTreeMap::from([
                ("00".to_owned(), 0.1),
                ("01".to_owned(), 0.1),
            ]);

        assert!(
            QuantumVolumeIdealDistribution::new(
                distribution
            )
            .is_err()
        );
    }

    #[test]
    fn execution_requires_exact_shot_count() {
        let counts =
            BTreeMap::from([
                ("00".to_owned(), 5usize),
                ("01".to_owned(), 5usize),
            ]);

        let execution =
            QuantumVolumeExecution::with_backend_id(
                "test",
                10,
                counts,
            )
            .unwrap();

        assert_eq!(execution.shots, 10);
    }

    #[test]
    fn execution_rejects_count_mismatch() {
        let counts =
            BTreeMap::from([
                ("00".to_owned(), 5usize),
            ]);

        let execution =
            QuantumVolumeExecution::with_backend_id(
                "test",
                10,
                counts,
            );

        assert!(execution.is_err());
    }

    #[test]
    fn execution_rejects_invalid_bitstrings() {
        let counts =
            BTreeMap::from([
                ("xyz".to_owned(), 10usize),
            ]);

        let execution =
            QuantumVolumeExecution::with_backend_id(
                "test",
                10,
                counts,
            );

        assert!(matches!(
            execution,
            Err(
                QuantumVolumeProtocolError::InvalidBitstring(_)
            )
        ));
    }

    #[test]
    fn trial_analysis_counts_only_ideal_heavy_outputs() {
        let point =
            QuantumVolumePoint::square(2).unwrap();

        let id =
            QuantumVolumeTrialId::derive(
                42,
                point,
                0,
            );

        let circuit =
            QuantumVolumeCircuit::new(id)
                .unwrap();

        let ideal =
            QuantumVolumeIdealDistribution::new(
                BTreeMap::from([
                    ("00".to_owned(), 0.1),
                    ("01".to_owned(), 0.2),
                    ("10".to_owned(), 0.3),
                    ("11".to_owned(), 0.4),
                ]),
            )
            .unwrap();

        let execution =
            QuantumVolumeExecution::with_backend_id(
                "simulator",
                100,
                BTreeMap::from([
                    ("00".to_owned(), 20usize),
                    ("01".to_owned(), 20usize),
                    ("10".to_owned(), 30usize),
                    ("11".to_owned(), 30usize),
                ]),
            )
            .unwrap();

        let result =
            QuantumVolumeTrialResult::analyze(
                &circuit,
                &ideal,
                &execution,
            )
            .unwrap();

        assert_eq!(
            result.heavy_outputs,
            60
        );

        assert_eq!(
            result.samples,
            100
        );

        assert!(
            (result.heavy_output_probability - 0.60).abs()
                < 1.0e-12
        );
    }

    #[test]
    fn point_result_requires_trials() {
        let config =
            QuantumVolumeProtocolConfig::single(2)
                .unwrap();

        let point =
            QuantumVolumePoint::square(2)
                .unwrap();

        let result =
            QuantumVolumePointResult::from_trials(
                point,
                1,
                Vec::new(),
                &config,
            );

        assert!(result.is_err());
    }

    #[test]
    fn point_result_can_pass_with_strong_data() {
        let config =
            QuantumVolumeProtocolConfig::single(2)
                .unwrap()
                .with_trials(10)
                .with_shots(1000);

        let point =
            QuantumVolumePoint::square(2)
                .unwrap();

        let mut trials = Vec::new();

        for trial in 0..10 {
            let id =
                QuantumVolumeTrialId::derive(
                    config.seed,
                    point,
                    trial,
                );

            trials.push(
                QuantumVolumeTrialResult {
                    id,
                    backend_id:
                        "simulator".to_owned(),
                    samples: 1000,
                    heavy_outputs: 900,
                    heavy_output_probability:
                        0.9,
                    ideal_median_probability:
                        0.01,
                    ideal_output_count: 4,
                    ideal_heavy_output_count: 2,
                    circuit_fingerprint:
                        None,
                },
            );
        }

        let result =
            QuantumVolumePointResult::from_trials(
                point,
                10,
                trials,
                &config,
            )
            .unwrap();

        assert!(result.passed);
        assert_eq!(
            result.quantum_volume,
            Some(4)
        );
    }

    #[test]
    fn benchmark_result_selects_largest_successful_width() {
        let config =
            QuantumVolumeProtocolConfig {
                points: vec![
                    QuantumVolumePoint::square(2)
                        .unwrap(),
                    QuantumVolumePoint::square(4)
                        .unwrap(),
                ],
                trials_per_point: 1,
                shots_per_circuit: 1000,
                ..QuantumVolumeProtocolConfig::default()
            };

        let mut point_results = Vec::new();

        for width in [2usize, 4usize] {
            let point =
                QuantumVolumePoint::square(width)
                    .unwrap();

            let id =
                QuantumVolumeTrialId::derive(
                    config.seed,
                    point,
                    0,
                );

            let trial =
                QuantumVolumeTrialResult {
                    id,
                    backend_id:
                        "simulator".to_owned(),
                    samples: 1000,
                    heavy_outputs:
                        if width == 2 {
                            900
                        } else {
                            100
                        },
                    heavy_output_probability:
                        if width == 2 {
                            0.9
                        } else {
                            0.1
                        },
                    ideal_median_probability:
                        0.01,
                    ideal_output_count: 4,
                    ideal_heavy_output_count: 2,
                    circuit_fingerprint:
                        None,
                };

            point_results.push(
                QuantumVolumePointResult::from_trials(
                    point,
                    1,
                    vec![trial],
                    &config,
                )
                .unwrap(),
            );
        }

        let result =
            QuantumVolumeBenchmarkResult::new(
                &config,
                point_results,
            )
            .unwrap();

        assert_eq!(
            result.successful_exponent,
            Some(2)
        );

        assert_eq!(
            result.quantum_volume,
            Some(4)
        );
    }

    #[test]
    fn protocol_plan_is_deterministic() {
        let config =
            QuantumVolumeProtocolConfig::square(
                [2, 3],
            )
            .unwrap()
            .with_trials(3);

        let protocol =
            QuantumVolumeProtocol::new();

        let first =
            protocol.plan(&config).unwrap();

        let second =
            protocol.plan(&config).unwrap();

        assert_eq!(first, second);
        assert_eq!(first.len(), 6);
    }
}