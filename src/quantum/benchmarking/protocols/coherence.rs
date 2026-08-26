//! Zamani Quantum Benchmarking — Qubit Coherence Characterization.
//!
//! Production coherence-characterization protocol layer.
//!
//! # Purpose
//!
//! This module provides backend-independent analysis and experiment metadata
//! for:
//!
//! - T1 energy-relaxation characterization;
//! - T2 Hahn-echo coherence characterization;
//! - T2* Ramsey characterization;
//! - Tphi pure-dephasing characterization;
//! - coherence-rate derivation;
//! - uncertainty propagation;
//! - fit diagnostics;
//! - physical-consistency validation;
//! - deterministic resource validation;
//! - reproducible result metadata.
//!
//! # Scientific model
//!
//! T1 is normally extracted from an excited-state population decay:
//
//!     P1(t) = A * exp(-t / T1) + B
//!
//! T2 Hahn echo is normally extracted from a transverse-coherence decay:
//
//!     C(t) = A * exp(-t / T2) + B
//!
//! T2* Ramsey is an oscillating, damped signal:
//
//!     S(t) = B
//!            + exp(-t / T2*)
//!              * (C * cos(2π f t) + D * sin(2π f t))
//!
//! The phase/frequency component is important. A plain exponential fit must
//! not be used blindly on Ramsey fringe data because oscillations can bias the
//! estimated decay time.
//!
//! Tphi is derived from:
//
//!     1 / Tphi = 1 / T2 - 1 / (2 T1)
//!
//! When the resulting rate is zero or negative, a finite Tphi cannot be
//! physically inferred from the supplied T1/T2 pair. The implementation
//! therefore returns `None` for the finite Tphi estimate instead of producing
//! infinity, NaN, or an artificial negative coherence time.
//!
//! # Architectural boundary
//!
//! This file OWNS:
//!
//! - coherence protocol configuration;
//! - coherence observation schemas;
//! - validation of coherence observations;
//! - T1 analysis;
//! - T2 Hahn analysis;
//! - T2* Ramsey analysis;
//! - Tphi derivation;
//! - protocol-specific diagnostics;
//! - protocol-specific result representation.
//!
//! This file DOES NOT OWN:
//!
//! - quantum-circuit generation;
//! - Quantum IR;
//! - backend selection;
//! - hardware execution;
//! - routing;
//! - scheduling;
//! - calibration acquisition;
//! - generic regression implementation;
//! - generic confidence intervals;
//! - report serialization.
//!
//! Integration is therefore:
//!
//! ```text
//! generators / frontend / runtime / hardware
//!                  │
//!                  ▼
//!        coherence experiment
//!                  │
//!                  ▼
//!        coherence observations
//!                  │
//!                  ▼
//!       protocols::coherence
//!          │              │
//!          │              └──────────────► core::limits
//!          │
//!          └────────────────────────────► statistics::regression
//!
//! Result
//!   │
//!   ├────────────► core::result
//!   ├────────────► core::metric
//!   ├────────────► core::provenance
//!   └────────────► reporting
//! ```
//!
//! # Execution separation
//!
//! The protocol does not execute experiments. A future execution layer can
//! generate and execute circuits and then feed the normalized observations
//! defined here into the analyzers.
//!
//! For example:
//!
//! ```text
//! T1:
//!
//! |0> -- X -- delay(t) -- measure
//!
//! T2 Hahn:
//!
//! |0> -- X90 -- delay(t/2) -- X180 -- delay(t/2) -- X90 -- measure
//!
//! T2* Ramsey:
//!
//! |0> -- X90 -- delay(t) -- X90 -- measure
//! ```
//!
//! Exact gate implementations are backend-specific and therefore remain
//! outside this file.
//!
//! # Statistical policy
//!
//! T1 and Hahn T2 use the canonical Zamani exponential regression engine.
//!
//! T2* uses a deterministic bounded damped-cosine profile fit. The nonlinear
//! frequency and decay parameters are searched deterministically while the
//! linear coefficients are solved exactly for every candidate.
//!
//! No random initialization is used.
//!
//! A good fit is not proof that the underlying physical noise follows the
//! assumed model. Real devices can exhibit non-exponential decay, beating,
//! drift, non-Markovian noise, thermal effects, leakage, calibration changes,
//! and other systematic effects. The result therefore exposes fit diagnostics
//! and scientific warnings instead of hiding model limitations.
//!
//! # Rust compatibility
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021
//!
//! Stable Rust only. No nightly features.
//!
//! # Integration contract
//!
//! Existing dependencies:
//!
//! ```text
//! statistics::regression
//! core::limits
//! ```
//!
//! Existing repository components that can consume the results later:
//!
//! ```text
//! quantum::hardware::calibration
//! quantum::benchmarking::metrics
//! quantum::benchmarking::core::result
//! quantum::benchmarking::core::provenance
//! quantum::benchmarking::execution
//! reporting
//! ```
//!
//! The protocol intentionally does not require those future files to be
//! modified in order to complete this file.
//!
//! # Scientific references
//!
//! The distinction between T1, T2 Hahn and T2* Ramsey follows established
//! coherence-characterization practice. T2* Ramsey is sensitive to
//! inhomogeneous broadening, while Hahn echo refocuses certain slowly varying
//! frequency errors. Tphi is derived from the longitudinal and transverse
//! relaxation rates when the resulting rate is physically meaningful.
//!
//! The implementation is intentionally backend-neutral and does not assume
//! superconducting-qubit-specific physics beyond the mathematical
//! characterization models.

use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;

use crate::quantum::benchmarking::core::limits::{
    BenchmarkLimits,
    LimitError,
};
use crate::quantum::benchmarking::statistics::regression::{
    RegressionConfig,
    RegressionEngine,
    RegressionError,
    RegressionFit,
    RegressionObservation,
};

/// Stable benchmark identifier.
pub const COHERENCE_BENCHMARK_ID: &str = "coherence_characterization";

/// Stable short identifier.
pub const COHERENCE_SHORT_ID: &str = "coherence";

/// Stable protocol implementation version.
///
/// Increment whenever the experiment semantics or analysis algorithm changes
/// in a way that can change benchmark results.
pub const COHERENCE_PROTOCOL_VERSION: u32 = 1;

/// Stable result schema version.
pub const COHERENCE_RESULT_SCHEMA_VERSION: u32 = 1;

/// Stable T1 analysis identifier.
pub const T1_ANALYSIS_VERSION: &str =
    "zamani.coherence.t1.exponential.v1";

/// Stable Hahn-T2 analysis identifier.
pub const T2_HAHN_ANALYSIS_VERSION: &str =
    "zamani.coherence.t2_hahn.exponential.v1";

/// Stable Ramsey-T2* analysis identifier.
pub const T2_STAR_ANALYSIS_VERSION: &str =
    "zamani.coherence.t2_star.damped_cosine.v1";

/// Stable Tphi analysis identifier.
pub const TPHI_ANALYSIS_VERSION: &str =
    "zamani.coherence.tphi.rate_difference.v1";

/// Minimum number of observations required by the exponential model.
///
/// The shared regression engine itself requires at least four observations
/// for meaningful residual diagnostics.
pub const MIN_EXPONENTIAL_OBSERVATIONS: usize = 4;

/// Minimum number of observations for Ramsey fitting.
pub const MIN_RAMSEY_OBSERVATIONS: usize = 6;

/// Default maximum number of delay points.
pub const DEFAULT_MAX_DELAY_POINTS: usize = 10_000;

/// Default maximum shots for one delay point.
pub const DEFAULT_MAX_SHOTS_PER_DELAY: u64 = 10_000_000;

/// Default minimum shots for a statistical warning.
pub const DEFAULT_MIN_SHOTS_PER_DELAY: u64 = 100;

/// Default maximum Ramsey frequency considered by the analyzer.
///
/// The caller may lower or raise this within production resource limits.
pub const DEFAULT_MAX_RAMSEY_FREQUENCY_HZ: f64 = 1.0e9;

/// Default number of frequency-search grid points.
pub const DEFAULT_RAMSEY_FREQUENCY_GRID_POINTS: usize = 64;

/// Default number of decay-rate grid points.
pub const DEFAULT_RAMSEY_DECAY_GRID_POINTS: usize = 64;

/// Default number of local coordinate-refinement iterations.
pub const DEFAULT_RAMSEY_REFINEMENT_ITERATIONS: usize = 64;

/// Default minimum positive coherence time in seconds.
pub const MIN_COHERENCE_TIME_S: f64 = 1.0e-15;

/// Numerical tolerance for probabilities/signals.
pub const UNIT_INTERVAL_EPSILON: f64 = 1.0e-12;

/// Numerical tolerance used when comparing physically equivalent rates.
pub const RATE_EPSILON: f64 = 1.0e-15;

/// Numerical tolerance used for linear-system singularity detection.
pub const LINEAR_SYSTEM_EPSILON: f64 = 1.0e-15;

/// Maximum number of warnings retained in one result.
pub const MAX_WARNINGS: usize = 64;

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by coherence characterization.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CoherenceError {
    /// No qubit identifier was supplied.
    InvalidQubit,

    /// Qubit identifier exceeds the configured benchmarking limits.
    QubitLimitExceeded {
        qubit: usize,
        maximum: usize,
    },

    /// No observations were supplied.
    EmptyObservations,

    /// Too few observations were supplied.
    InsufficientObservations {
        actual: usize,
        minimum: usize,
    },

    /// Too many observations were supplied.
    TooManyObservations {
        actual: usize,
        maximum: usize,
    },

    /// Delay is invalid.
    InvalidDelay {
        index: usize,
        delay_s: f64,
    },

    /// Delays are not strictly increasing.
    NonIncreasingDelays {
        index: usize,
        previous_s: f64,
        current_s: f64,
    },

    /// Delay values contain duplicates.
    DuplicateDelay {
        delay_s: f64,
    },

    /// Signal is not finite.
    NonFiniteSignal {
        index: usize,
        value: f64,
    },

    /// Signal lies outside its physical normalized interval.
    InvalidSignal {
        index: usize,
        value: f64,
    },

    /// Shot count is zero.
    InvalidShotCount {
        index: usize,
    },

    /// Shot count exceeds the configured maximum.
    TooManyShots {
        index: usize,
        shots: u64,
        maximum: u64,
    },

    /// Frequency is invalid.
    InvalidFrequency {
        value_hz: f64,
    },

    /// Frequency exceeds the configured analysis limit.
    FrequencyTooLarge {
        value_hz: f64,
        maximum_hz: f64,
    },

    /// Search grid is invalid.
    InvalidGridSize {
        name: &'static str,
        value: usize,
    },

    /// Refinement iteration count is invalid.
    InvalidIterationCount {
        value: usize,
    },

    /// Maximum delay is invalid.
    InvalidMaximumDelay {
        value_s: f64,
    },

    /// Maximum coherence time is invalid.
    InvalidMaximumCoherenceTime {
        value_s: f64,
    },

    /// Workload calculation overflowed.
    WorkloadOverflow,

    /// Generic benchmark resource limit.
    Limit(LimitError),

    /// Generic exponential regression error.
    Regression(RegressionError),

    /// Regression produced a physically impossible coherence time.
    InvalidFittedCoherenceTime {
        value_s: f64,
    },

    /// Regression produced a non-positive decay rate.
    InvalidDecayRate {
        value: f64,
    },

    /// Ramsey fit could not find a finite solution.
    NoFiniteRamseyFit,

    /// Ramsey linear model became singular.
    SingularRamseyModel,

    /// Ramsey fit produced an invalid decay rate.
    InvalidRamseyDecayRate {
        value: f64,
    },

    /// Ramsey fit produced an invalid frequency.
    InvalidRamseyFrequency {
        value_hz: f64,
    },

    /// Tphi cannot be inferred as a finite positive time.
    InvalidTphiRate {
        rate_per_s: f64,
    },

    /// Tphi inputs are inconsistent.
    InconsistentTphiInputs {
        t1_s: f64,
        t2_s: f64,
    },

    /// Numerical operation produced a non-finite value.
    NumericalFailure {
        operation: &'static str,
    },
}

impl fmt::Display for CoherenceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidQubit => {
                write!(formatter, "coherence benchmark requires a valid qubit")
            }

            Self::QubitLimitExceeded { qubit, maximum } => {
                write!(
                    formatter,
                    "qubit {qubit} exceeds benchmarking qubit limit {maximum}"
                )
            }

            Self::EmptyObservations => {
                write!(formatter, "coherence benchmark requires observations")
            }

            Self::InsufficientObservations { actual, minimum } => {
                write!(
                    formatter,
                    "coherence benchmark requires at least {minimum} \
                     observations, got {actual}"
                )
            }

            Self::TooManyObservations { actual, maximum } => {
                write!(
                    formatter,
                    "coherence benchmark received {actual} observations; \
                     maximum is {maximum}"
                )
            }

            Self::InvalidDelay { index, delay_s } => {
                write!(
                    formatter,
                    "invalid delay at observation {index}: {delay_s} s"
                )
            }

            Self::NonIncreasingDelays {
                index,
                previous_s,
                current_s,
            } => {
                write!(
                    formatter,
                    "delays must be strictly increasing: observation {index} \
                     has {current_s} s after {previous_s} s"
                )
            }

            Self::DuplicateDelay { delay_s } => {
                write!(
                    formatter,
                    "duplicate coherence delay: {delay_s} s"
                )
            }

            Self::NonFiniteSignal { index, value } => {
                write!(
                    formatter,
                    "signal at observation {index} is non-finite: {value}"
                )
            }

            Self::InvalidSignal { index, value } => {
                write!(
                    formatter,
                    "signal at observation {index} is outside [0, 1]: {value}"
                )
            }

            Self::InvalidShotCount { index } => {
                write!(
                    formatter,
                    "observation {index} has zero shots"
                )
            }

            Self::TooManyShots {
                index,
                shots,
                maximum,
            } => {
                write!(
                    formatter,
                    "observation {index} has {shots} shots; maximum is {maximum}"
                )
            }

            Self::InvalidFrequency { value_hz } => {
                write!(
                    formatter,
                    "frequency must be finite and non-negative, got {value_hz} Hz"
                )
            }

            Self::FrequencyTooLarge {
                value_hz,
                maximum_hz,
            } => {
                write!(
                    formatter,
                    "frequency {value_hz} Hz exceeds maximum {maximum_hz} Hz"
                )
            }

            Self::InvalidGridSize { name, value } => {
                write!(
                    formatter,
                    "{name} grid size must be greater than zero, got {value}"
                )
            }

            Self::InvalidIterationCount { value } => {
                write!(
                    formatter,
                    "Ramsey refinement iterations must be greater than zero, \
                     got {value}"
                )
            }

            Self::InvalidMaximumDelay { value_s } => {
                write!(
                    formatter,
                    "maximum delay must be finite and positive, got {value_s} s"
                )
            }

            Self::InvalidMaximumCoherenceTime { value_s } => {
                write!(
                    formatter,
                    "maximum coherence time must be finite and positive, \
                     got {value_s} s"
                )
            }

            Self::WorkloadOverflow => {
                write!(
                    formatter,
                    "coherence benchmark workload calculation overflowed"
                )
            }

            Self::Limit(error) => {
                write!(formatter, "benchmark resource limit: {error}")
            }

            Self::Regression(error) => {
                write!(formatter, "coherence regression failed: {error}")
            }

            Self::InvalidFittedCoherenceTime { value_s } => {
                write!(
                    formatter,
                    "fitted coherence time is invalid: {value_s} s"
                )
            }

            Self::InvalidDecayRate { value } => {
                write!(
                    formatter,
                    "fitted decay rate is invalid: {value}"
                )
            }

            Self::NoFiniteRamseyFit => {
                write!(
                    formatter,
                    "Ramsey analysis did not produce a finite fit"
                )
            }

            Self::SingularRamseyModel => {
                write!(
                    formatter,
                    "Ramsey damped-cosine linear model is singular"
                )
            }

            Self::InvalidRamseyDecayRate { value } => {
                write!(
                    formatter,
                    "Ramsey decay rate is invalid: {value}"
                )
            }

            Self::InvalidRamseyFrequency { value_hz } => {
                write!(
                    formatter,
                    "Ramsey frequency is invalid: {value_hz} Hz"
                )
            }

            Self::InvalidTphiRate { rate_per_s } => {
                write!(
                    formatter,
                    "Tphi rate is not physically positive: {rate_per_s} s^-1"
                )
            }

            Self::InconsistentTphiInputs { t1_s, t2_s } => {
                write!(
                    formatter,
                    "Tphi inputs are inconsistent: T1={t1_s} s, T2={t2_s} s"
                )
            }

            Self::NumericalFailure { operation } => {
                write!(
                    formatter,
                    "non-finite numerical result during {operation}"
                )
            }
        }
    }
}

impl Error for CoherenceError {}

impl From<LimitError> for CoherenceError {
    fn from(error: LimitError) -> Self {
        Self::Limit(error)
    }
}

impl From<RegressionError> for CoherenceError {
    fn from(error: RegressionError) -> Self {
        Self::Regression(error)
    }
}

// =============================================================================
// Experiment type
// =============================================================================

/// Coherence experiment type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CoherenceExperiment {
    /// Energy relaxation characterization.
    T1,

    /// Hahn-echo transverse coherence.
    T2Hahn,

    /// Ramsey/inhomogeneous transverse coherence.
    T2StarRamsey,

    /// Pure dephasing derived from T1/T2.
    Tphi,
}

impl CoherenceExperiment {
    /// Stable machine-readable identifier.
    pub const fn id(self) -> &'static str {
        match self {
            Self::T1 => "t1",
            Self::T2Hahn => "t2_hahn",
            Self::T2StarRamsey => "t2_star_ramsey",
            Self::Tphi => "t_phi",
        }
    }
}

// =============================================================================
// Configuration
// =============================================================================

/// Production configuration for coherence analysis.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CoherenceConfig {
    /// Physical qubit being characterized.
    pub qubit: usize,

    /// Statistical confidence level.
    pub confidence_level: f64,

    /// Maximum number of delay points.
    pub max_delay_points: usize,

    /// Maximum shots at one delay.
    pub max_shots_per_delay: u64,

    /// Maximum permitted delay in seconds.
    pub max_delay_s: f64,

    /// Maximum coherence time represented by the analyzer.
    pub max_coherence_time_s: f64,

    /// Maximum Ramsey frequency considered by the fit.
    pub max_ramsey_frequency_hz: f64,

    /// Number of deterministic frequency-search grid points.
    pub ramsey_frequency_grid_points: usize,

    /// Number of deterministic decay-rate grid points.
    pub ramsey_decay_grid_points: usize,

    /// Number of local Ramsey refinement iterations.
    pub ramsey_refinement_iterations: usize,

    /// Optional expected Ramsey frequency.
    ///
    /// This is a search hint only. The analyzer still validates and searches
    /// the configured frequency interval.
    pub ramsey_frequency_hint_hz: Option<f64>,

    /// Whether to calculate inverse-variance weights from shot counts.
    pub use_shot_weights: bool,

    /// Whether to reject non-increasing delay sequences.
    pub require_strictly_increasing_delays: bool,
}

impl Default for CoherenceConfig {
    fn default() -> Self {
        Self {
            qubit: 0,
            confidence_level: 0.95,
            max_delay_points: DEFAULT_MAX_DELAY_POINTS,
            max_shots_per_delay: DEFAULT_MAX_SHOTS_PER_DELAY,
            max_delay_s: 86_400.0,
            max_coherence_time_s: 86_400.0,
            max_ramsey_frequency_hz: DEFAULT_MAX_RAMSEY_FREQUENCY_HZ,
            ramsey_frequency_grid_points:
                DEFAULT_RAMSEY_FREQUENCY_GRID_POINTS,
            ramsey_decay_grid_points:
                DEFAULT_RAMSEY_DECAY_GRID_POINTS,
            ramsey_refinement_iterations:
                DEFAULT_RAMSEY_REFINEMENT_ITERATIONS,
            ramsey_frequency_hint_hz: None,
            use_shot_weights: true,
            require_strictly_increasing_delays: true,
        }
    }
}

impl CoherenceConfig {
    /// Returns production configuration.
    pub fn production(qubit: usize) -> Self {
        Self {
            qubit,
            ..Self::default()
        }
    }

    /// Validates the configuration without executing anything.
    pub fn validate(
        &self,
        limits: &BenchmarkLimits,
    ) -> Result<(), CoherenceError> {
        limits.validate()?;

        if self.qubit >= limits.max_qubits {
            return Err(CoherenceError::QubitLimitExceeded {
                qubit: self.qubit,
                maximum: limits.max_qubits,
            });
        }

        validate_confidence_level(self.confidence_level)?;

        if self.max_delay_points == 0 {
            return Err(CoherenceError::InvalidGridSize {
                name: "max_delay_points",
                value: self.max_delay_points,
            });
        }

        if self.max_delay_points > limits.max_observations as usize {
            return Err(CoherenceError::TooManyObservations {
                actual: self.max_delay_points,
                maximum: limits.max_observations as usize,
            });
        }

        if self.max_shots_per_delay == 0
            || self.max_shots_per_delay > limits.max_shots
        {
            return Err(CoherenceError::TooManyShots {
                index: 0,
                shots: self.max_shots_per_delay,
                maximum: limits.max_shots,
            });
        }

        if !self.max_delay_s.is_finite()
            || self.max_delay_s <= 0.0
        {
            return Err(CoherenceError::InvalidMaximumDelay {
                value_s: self.max_delay_s,
            });
        }

        if !self.max_coherence_time_s.is_finite()
            || self.max_coherence_time_s <= 0.0
        {
            return Err(
                CoherenceError::InvalidMaximumCoherenceTime {
                    value_s: self.max_coherence_time_s,
                },
            );
        }

        if !self.max_ramsey_frequency_hz.is_finite()
            || self.max_ramsey_frequency_hz <= 0.0
        {
            return Err(CoherenceError::InvalidFrequency {
                value_hz: self.max_ramsey_frequency_hz,
            });
        }

        if self.ramsey_frequency_grid_points == 0 {
            return Err(CoherenceError::InvalidGridSize {
                name: "ramsey_frequency_grid_points",
                value: self.ramsey_frequency_grid_points,
            });
        }

        if self.ramsey_decay_grid_points == 0 {
            return Err(CoherenceError::InvalidGridSize {
                name: "ramsey_decay_grid_points",
                value: self.ramsey_decay_grid_points,
            });
        }

        if self.ramsey_refinement_iterations == 0 {
            return Err(CoherenceError::InvalidIterationCount {
                value: self.ramsey_refinement_iterations,
            });
        }

        if let Some(frequency) = self.ramsey_frequency_hint_hz {
            validate_frequency(
                frequency,
                self.max_ramsey_frequency_hz,
            )?;
        }

        Ok(())
    }
}

// =============================================================================
// Observation schemas
// =============================================================================

/// Generic coherence observation.
///
/// `signal` must be a normalized physical signal in `[0, 1]`.
///
/// Examples:
///
/// - T1: excited-state population P(|1>)
/// - Hahn T2: normalized coherence/survival signal
///
/// The exact physical preparation and measurement semantics belong to the
/// experiment generator/executor.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct CoherenceObservation {
    /// Delay/evolution time in seconds.
    pub delay_s: f64,

    /// Measured normalized signal.
    pub signal: f64,

    /// Number of shots represented by this observation.
    pub shots: u64,
}

impl CoherenceObservation {
    /// Creates an observation.
    pub const fn new(
        delay_s: f64,
        signal: f64,
        shots: u64,
    ) -> Self {
        Self {
            delay_s,
            signal,
            shots,
        }
    }
}

/// Ramsey coherence observation.
///
/// Ramsey signals are represented in `[-1, 1]` because the observable may be
/// a signed quadrature/expectation value rather than a probability.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct RamseyObservation {
    /// Delay/evolution time in seconds.
    pub delay_s: f64,

    /// Signed Ramsey signal.
    pub signal: f64,

    /// Number of shots.
    pub shots: u64,
}

impl RamseyObservation {
    /// Creates a Ramsey observation.
    pub const fn new(
        delay_s: f64,
        signal: f64,
        shots: u64,
    ) -> Self {
        Self {
            delay_s,
            signal,
            shots,
        }
    }
}

/// Result of a coherence-time estimate.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CoherenceTimeEstimate {
    /// Estimated coherence time in seconds.
    pub value_s: f64,

    /// Approximate standard error in seconds, if available.
    pub standard_error_s: Option<f64>,

    /// Lower uncertainty bound in seconds, if available.
    pub lower_s: Option<f64>,

    /// Upper uncertainty bound in seconds, if available.
    pub upper_s: Option<f64>,

    /// Confidence level used for the uncertainty representation.
    pub confidence_level: f64,

    /// Stable analysis algorithm identifier.
    pub analysis_version: String,
}

impl CoherenceTimeEstimate {
    /// Creates an estimate from a decay rate.
    fn from_decay_rate(
        decay_rate: f64,
        decay_rate_standard_error: Option<f64>,
        confidence_level: f64,
        analysis_version: &'static str,
    ) -> Result<Self, CoherenceError> {
        if !decay_rate.is_finite() || decay_rate <= 0.0 {
            return Err(CoherenceError::InvalidDecayRate {
                value: decay_rate,
            });
        }

        let value_s = 1.0 / decay_rate;

        if !value_s.is_finite()
            || value_s < MIN_COHERENCE_TIME_S
        {
            return Err(
                CoherenceError::InvalidFittedCoherenceTime {
                    value_s,
                },
            );
        }

        let standard_error_s =
            decay_rate_standard_error
                .filter(|value| value.is_finite() && *value >= 0.0)
                .map(|standard_error| {
                    standard_error / (decay_rate * decay_rate)
                });

        let (lower_s, upper_s) =
            match standard_error_s {
                Some(error) if error.is_finite() => {
                    let z = normal_quantile(confidence_level);
                    let lower_rate =
                        (decay_rate - z * decay_rate_standard_error.unwrap())
                            .max(MIN_COHERENCE_TIME_S);
                    let upper_rate =
                        decay_rate
                            + z
                                * decay_rate_standard_error
                                    .unwrap();

                    let lower_time =
                        if upper_rate > 0.0 {
                            Some(1.0 / upper_rate)
                        } else {
                            None
                        };

                    let upper_time =
                        if lower_rate > 0.0 {
                            Some(1.0 / lower_rate)
                        } else {
                            None
                        };

                    (lower_time, upper_time)
                }

                _ => (None, None),
            };

        Ok(Self {
            value_s,
            standard_error_s,
            lower_s,
            upper_s,
            confidence_level,
            analysis_version: analysis_version.to_string(),
        })
    }
}

/// Ramsey fit result.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RamseyFitResult {
    /// T2* estimate.
    pub t2_star: CoherenceTimeEstimate,

    /// Estimated Ramsey frequency in Hz.
    pub frequency_hz: f64,

    /// Estimated constant offset.
    pub offset: f64,

    /// Cosine coefficient.
    pub cosine_amplitude: f64,

    /// Sine coefficient.
    pub sine_amplitude: f64,

    /// Combined oscillation amplitude.
    pub fringe_amplitude: f64,

    /// Sum of squared residuals.
    pub residual_sum_squares: f64,

    /// Root mean squared residual.
    pub rmse: f64,

    /// Coefficient of determination.
    pub r_squared: Option<f64>,

    /// Number of observations.
    pub observations: usize,

    /// Total shots.
    pub total_shots: u64,

    /// Whether the fitted frequency reached the configured boundary.
    pub frequency_at_boundary: bool,

    /// Whether the fitted decay reached the configured boundary.
    pub decay_at_boundary: bool,

    /// Scientific/quality warnings.
    pub warnings: Vec<String>,
}

/// Result of T1 analysis.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct T1Result {
    /// T1 estimate.
    pub t1: CoherenceTimeEstimate,

    /// Underlying exponential regression.
    pub regression: RegressionFit,

    /// Number of observations.
    pub observations: usize,

    /// Total shots.
    pub total_shots: u64,

    /// Scientific warnings.
    pub warnings: Vec<String>,
}

/// Result of Hahn T2 analysis.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct T2HahnResult {
    /// T2 estimate.
    pub t2: CoherenceTimeEstimate,

    /// Underlying exponential regression.
    pub regression: RegressionFit,

    /// Number of observations.
    pub observations: usize,

    /// Total shots.
    pub total_shots: u64,

    /// Scientific warnings.
    pub warnings: Vec<String>,
}

/// Result of pure-dephasing derivation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TphiResult {
    /// Pure-dephasing rate in inverse seconds.
    pub rate_per_s: f64,

    /// Finite Tphi when the inferred rate is strictly positive.
    ///
    /// `None` means that the supplied T1/T2 pair does not support a finite
    /// positive Tphi under the standard rate relation.
    pub time_s: Option<f64>,

    /// Approximate propagated standard error of the rate.
    pub rate_standard_error_per_s: Option<f64>,

    /// Approximate propagated standard error of Tphi.
    pub time_standard_error_s: Option<f64>,

    /// Confidence level used for the uncertainty.
    pub confidence_level: f64,

    /// Whether the inferred rate is physically positive.
    pub physically_positive: bool,

    /// Scientific warnings.
    pub warnings: Vec<String>,
}

/// Complete coherence characterization result.
///
/// This is intentionally protocol-specific. A future universal
/// `BenchmarkResult` can wrap this value without changing the coherence
/// mathematics.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CoherenceResult {
    /// Stable benchmark identifier.
    pub benchmark_id: String,

    /// Stable protocol version.
    pub protocol_version: u32,

    /// Result schema version.
    pub result_schema_version: u32,

    /// Physical qubit.
    pub qubit: usize,

    /// Optional T1 result.
    pub t1: Option<T1Result>,

    /// Optional Hahn T2 result.
    pub t2_hahn: Option<T2HahnResult>,

    /// Optional Ramsey T2* result.
    pub t2_star: Option<RamseyFitResult>,

    /// Optional Tphi result.
    pub tphi: Option<TphiResult>,

    /// Protocol-wide warnings.
    pub warnings: Vec<String>,
}

impl CoherenceResult {
    /// Creates an empty result envelope.
    pub fn new(qubit: usize) -> Self {
        Self {
            benchmark_id: COHERENCE_BENCHMARK_ID.to_string(),
            protocol_version: COHERENCE_PROTOCOL_VERSION,
            result_schema_version:
                COHERENCE_RESULT_SCHEMA_VERSION,
            qubit,
            t1: None,
            t2_hahn: None,
            t2_star: None,
            tphi: None,
            warnings: Vec::new(),
        }
    }
}

// =============================================================================
// Analyzer
// =============================================================================

/// Production coherence analyzer.
#[derive(Debug, Clone)]
pub struct CoherenceAnalyzer {
    config: CoherenceConfig,
    limits: BenchmarkLimits,
}

impl CoherenceAnalyzer {
    /// Creates an analyzer with production resource limits.
    pub fn new(
        config: CoherenceConfig,
    ) -> Result<Self, CoherenceError> {
        let limits = BenchmarkLimits::production();
        config.validate(&limits)?;

        Ok(Self { config, limits })
    }

    /// Creates an analyzer with an explicit resource policy.
    pub fn with_limits(
        config: CoherenceConfig,
        limits: BenchmarkLimits,
    ) -> Result<Self, CoherenceError> {
        config.validate(&limits)?;
        Ok(Self { config, limits })
    }

    /// Returns the analyzer configuration.
    pub fn config(&self) -> &CoherenceConfig {
        &self.config
    }

    /// Returns the active resource limits.
    pub fn limits(&self) -> &BenchmarkLimits {
        &self.limits
    }

    /// Validates generic probability-like coherence observations.
    pub fn validate_observations(
        &self,
        observations: &[CoherenceObservation],
    ) -> Result<(), CoherenceError> {
        self.validate_observation_count(observations.len())?;

        let mut previous_delay = None;

        for (index, observation) in observations.iter().enumerate() {
            validate_delay(
                index,
                observation.delay_s,
                self.config.max_delay_s,
            )?;

            validate_unit_interval_signal(
                index,
                observation.signal,
            )?;

            self.validate_shots(index, observation.shots)?;

            if self.config.require_strictly_increasing_delays {
                if let Some(previous) = previous_delay {
                    if observation.delay_s < previous {
                        return Err(
                            CoherenceError::NonIncreasingDelays {
                                index,
                                previous_s: previous,
                                current_s: observation.delay_s,
                            },
                        );
                    }

                    if observation.delay_s == previous {
                        return Err(
                            CoherenceError::DuplicateDelay {
                                delay_s: observation.delay_s,
                            },
                        );
                    }
                }
            }

            previous_delay = Some(observation.delay_s);
        }

        Ok(())
    }

    /// Validates signed Ramsey observations.
    pub fn validate_ramsey_observations(
        &self,
        observations: &[RamseyObservation],
    ) -> Result<(), CoherenceError> {
        self.validate_observation_count(observations.len())?;

        let mut previous_delay = None;

        for (index, observation) in observations.iter().enumerate() {
            validate_delay(
                index,
                observation.delay_s,
                self.config.max_delay_s,
            )?;

            if !observation.signal.is_finite() {
                return Err(CoherenceError::NonFiniteSignal {
                    index,
                    value: observation.signal,
                });
            }

            if observation.signal < -1.0 - UNIT_INTERVAL_EPSILON
                || observation.signal > 1.0 + UNIT_INTERVAL_EPSILON
            {
                return Err(CoherenceError::InvalidSignal {
                    index,
                    value: observation.signal,
                });
            }

            self.validate_shots(index, observation.shots)?;

            if self.config.require_strictly_increasing_delays {
                if let Some(previous) = previous_delay {
                    if observation.delay_s < previous {
                        return Err(
                            CoherenceError::NonIncreasingDelays {
                                index,
                                previous_s: previous,
                                current_s: observation.delay_s,
                            },
                        );
                    }

                    if observation.delay_s == previous {
                        return Err(
                            CoherenceError::DuplicateDelay {
                                delay_s: observation.delay_s,
                            },
                        );
                    }
                }
            }

            previous_delay = Some(observation.delay_s);
        }

        Ok(())
    }

    /// Analyzes T1 observations.
    ///
    /// The signal is expected to represent the excited-state population.
    pub fn analyze_t1(
        &self,
        observations: &[CoherenceObservation],
    ) -> Result<T1Result, CoherenceError> {
        self.validate_observations(observations)?;

        let regression =
            self.fit_exponential(observations)?;

        let t1 =
            CoherenceTimeEstimate::from_decay_rate(
                regression.decay_rate.value,
                regression.decay_rate.standard_error,
                self.config.confidence_level,
                T1_ANALYSIS_VERSION,
            )?;

        let mut warnings = Vec::new();

        self.append_exponential_warnings(
            &regression,
            "T1",
            &mut warnings,
        );

        if observations.len() < 8 {
            push_warning(
                &mut warnings,
                "T1 uses fewer than 8 delay points; uncertainty and \
                 model diagnostics may be weak.",
            );
        }

        if total_shots(observations) < 1_000 {
            push_warning(
                &mut warnings,
                "T1 uses fewer than 1000 total shots.",
            );
        }

        Ok(T1Result {
            t1,
            regression,
            observations: observations.len(),
            total_shots: total_shots(observations),
            warnings,
        })
    }

    /// Analyzes Hahn-echo T2 observations.
    ///
    /// The supplied signal should be a normalized transverse-coherence or
    /// survival observable that follows an approximately exponential decay.
    pub fn analyze_t2_hahn(
        &self,
        observations: &[CoherenceObservation],
    ) -> Result<T2HahnResult, CoherenceError> {
        self.validate_observations(observations)?;

        let regression =
            self.fit_exponential(observations)?;

        let t2 =
            CoherenceTimeEstimate::from_decay_rate(
                regression.decay_rate.value,
                regression.decay_rate.standard_error,
                self.config.confidence_level,
                T2_HAHN_ANALYSIS_VERSION,
            )?;

        let mut warnings = Vec::new();

        self.append_exponential_warnings(
            &regression,
            "T2 Hahn",
            &mut warnings,
        );

        if observations.len() < 8 {
            push_warning(
                &mut warnings,
                "T2 Hahn uses fewer than 8 delay points; uncertainty and \
                 model diagnostics may be weak.",
            );
        }

        Ok(T2HahnResult {
            t2,
            regression,
            observations: observations.len(),
            total_shots: total_shots(observations),
            warnings,
        })
    }

    /// Analyzes T2* Ramsey observations.
    ///
    /// The analysis uses:
    ///
    ///     y(t) = B + exp(-k t)
    ///              * [C cos(2π f t) + D sin(2π f t)]
    ///
    /// and returns T2* = 1/k.
    pub fn analyze_t2_star(
        &self,
        observations: &[RamseyObservation],
    ) -> Result<RamseyFitResult, CoherenceError> {
        self.validate_ramsey_observations(observations)?;

        if observations.len() < MIN_RAMSEY_OBSERVATIONS {
            return Err(
                CoherenceError::InsufficientObservations {
                    actual: observations.len(),
                    minimum: MIN_RAMSEY_OBSERVATIONS,
                },
            );
        }

        let total_shots =
            observations.iter().try_fold(
                0_u64,
                |sum, observation| {
                    sum.checked_add(observation.shots)
                        .ok_or(CoherenceError::WorkloadOverflow)
                },
            )?;

        let fit = self.fit_ramsey(observations)?;

        let t2_star =
            CoherenceTimeEstimate::from_decay_rate(
                fit.decay_rate,
                None,
                self.config.confidence_level,
                T2_STAR_ANALYSIS_VERSION,
            )?;

        let mut warnings = Vec::new();

        if observations.len() < 12 {
            push_warning(
                &mut warnings,
                "Ramsey analysis uses fewer than 12 delay points; \
                 frequency/decay identifiability may be weak.",
            );
        }

        if total_shots < 1_000 {
            push_warning(
                &mut warnings,
                "Ramsey analysis uses fewer than 1000 total shots.",
            );
        }

        if fit.frequency_at_boundary {
            push_warning(
                &mut warnings,
                "Ramsey frequency reached the configured search boundary; \
                 the true frequency may lie outside the configured range.",
            );
        }

        if fit.decay_at_boundary {
            push_warning(
                &mut warnings,
                "Ramsey decay rate reached the configured search boundary; \
                 T2* may not be identifiable within the configured range.",
            );
        }

        if let Some(r_squared) = fit.r_squared {
            if r_squared < 0.8 {
                push_warning(
                    &mut warnings,
                    "Ramsey damped-cosine fit has R² below 0.8; inspect \
                     residuals and consider non-exponential or multi-frequency \
                     behaviour.",
                );
            }
        }

        Ok(RamseyFitResult {
            t2_star,
            frequency_hz: fit.frequency_hz,
            offset: fit.offset,
            cosine_amplitude: fit.cosine_amplitude,
            sine_amplitude: fit.sine_amplitude,
            fringe_amplitude: fit.fringe_amplitude,
            residual_sum_squares: fit.residual_sum_squares,
            rmse: fit.rmse,
            r_squared: fit.r_squared,
            observations: observations.len(),
            total_shots,
            frequency_at_boundary: fit.frequency_at_boundary,
            decay_at_boundary: fit.decay_at_boundary,
            warnings,
        })
    }

    /// Derives Tphi from T1 and T2.
    ///
    ///     gamma_phi = gamma_2 - gamma_1 / 2
    ///
    /// A non-positive gamma_phi is not converted to infinity. The result
    /// contains `time_s = None` and a diagnostic warning.
    pub fn derive_tphi(
        &self,
        t1: &CoherenceTimeEstimate,
        t2: &CoherenceTimeEstimate,
    ) -> Result<TphiResult, CoherenceError> {
        if !t1.value_s.is_finite()
            || !t2.value_s.is_finite()
            || t1.value_s <= 0.0
            || t2.value_s <= 0.0
        {
            return Err(
                CoherenceError::InconsistentTphiInputs {
                    t1_s: t1.value_s,
                    t2_s: t2.value_s,
                },
            );
        }

        let gamma_1 = 1.0 / t1.value_s;
        let gamma_2 = 1.0 / t2.value_s;

        let rate = gamma_2 - 0.5 * gamma_1;

        if !rate.is_finite() {
            return Err(CoherenceError::NumericalFailure {
                operation: "Tphi rate derivation",
            });
        }

        let sigma_t1 =
            t1.standard_error_s.unwrap_or(0.0);

        let sigma_t2 =
            t2.standard_error_s.unwrap_or(0.0);

        let sigma_gamma_1 =
            if sigma_t1 > 0.0 {
                sigma_t1 / (t1.value_s * t1.value_s)
            } else {
                0.0
            };

        let sigma_gamma_2 =
            if sigma_t2 > 0.0 {
                sigma_t2 / (t2.value_s * t2.value_s)
            } else {
                0.0
            };

        let rate_standard_error =
            (sigma_gamma_2 * sigma_gamma_2
                + 0.25 * sigma_gamma_1 * sigma_gamma_1)
                .sqrt();

        if rate <= RATE_EPSILON {
            return Ok(TphiResult {
                rate_per_s: rate,
                time_s: None,
                rate_standard_error_per_s: Some(
                    rate_standard_error,
                ),
                time_standard_error_s: None,
                confidence_level: self.config.confidence_level,
                physically_positive: false,
                warnings: vec![
                    "The inferred pure-dephasing rate is zero or negative. \
                     No finite positive Tphi is reported. This may indicate \
                     that the measured T2 is relaxation-limited, that the \
                     uncertainties overlap zero, or that the simple rate \
                     model is not adequate."
                        .to_string(),
                ],
            });
        }

        let time_s = 1.0 / rate;

        if !time_s.is_finite() || time_s <= 0.0 {
            return Err(CoherenceError::InvalidTphiRate {
                rate_per_s: rate,
            });
        }

        let time_standard_error =
            rate_standard_error / (rate * rate);

        Ok(TphiResult {
            rate_per_s: rate,
            time_s: Some(time_s),
            rate_standard_error_per_s: Some(
                rate_standard_error,
            ),
            time_standard_error_s: Some(
                time_standard_error,
            ),
            confidence_level: self.config.confidence_level,
            physically_positive: true,
            warnings: Vec::new(),
        })
    }

    /// Performs a combined T1/T2/Tphi characterization.
    pub fn analyze_t1_t2_tphi(
        &self,
        t1_observations: &[CoherenceObservation],
        t2_observations: &[CoherenceObservation],
    ) -> Result<CoherenceResult, CoherenceError> {
        let t1 = self.analyze_t1(t1_observations)?;
        let t2 = self.analyze_t2_hahn(t2_observations)?;
        let tphi = self.derive_tphi(&t1.t1, &t2.t2)?;

        let mut result =
            CoherenceResult::new(self.config.qubit);

        result.t1 = Some(t1);
        result.t2_hahn = Some(t2);
        result.tphi = Some(tphi);

        self.validate_result(&result)?;

        Ok(result)
    }

    /// Performs T1/T2/T2*/Tphi analysis when all input datasets are
    /// available.
    pub fn analyze_all(
        &self,
        t1_observations: &[CoherenceObservation],
        t2_observations: &[CoherenceObservation],
        t2_star_observations: &[RamseyObservation],
    ) -> Result<CoherenceResult, CoherenceError> {
        let t1 = self.analyze_t1(t1_observations)?;
        let t2 = self.analyze_t2_hahn(t2_observations)?;
        let t2_star =
            self.analyze_t2_star(t2_star_observations)?;

        let tphi = self.derive_tphi(&t1.t1, &t2.t2)?;

        let mut result =
            CoherenceResult::new(self.config.qubit);

        result.t1 = Some(t1);
        result.t2_hahn = Some(t2);
        result.t2_star = Some(t2_star);
        result.tphi = Some(tphi);

        self.validate_result(&result)?;

        Ok(result)
    }

    /// Validates a completed coherence result.
    pub fn validate_result(
        &self,
        result: &CoherenceResult,
    ) -> Result<(), CoherenceError> {
        if result.qubit != self.config.qubit {
            return Err(CoherenceError::InvalidQubit);
        }

        if let Some(t1) = &result.t1 {
            validate_coherence_time(
                t1.t1.value_s,
                self.config.max_coherence_time_s,
            )?;
        }

        if let Some(t2) = &result.t2_hahn {
            validate_coherence_time(
                t2.t2.value_s,
                self.config.max_coherence_time_s,
            )?;
        }

        if let Some(t2_star) = &result.t2_star {
            validate_coherence_time(
                t2_star.t2_star.value_s,
                self.config.max_coherence_time_s,
            )?;

            validate_frequency(
                t2_star.frequency_hz,
                self.config.max_ramsey_frequency_hz,
            )?;
        }

        if let Some(tphi) = &result.tphi {
            if !tphi.rate_per_s.is_finite() {
                return Err(CoherenceError::NumericalFailure {
                    operation: "Tphi result validation",
                });
            }

            if let Some(time_s) = tphi.time_s {
                if !time_s.is_finite() || time_s <= 0.0 {
                    return Err(
                        CoherenceError::InvalidTphiRate {
                            rate_per_s: tphi.rate_per_s,
                        },
                    );
                }
            }
        }

        Ok(())
    }

    // -------------------------------------------------------------------------
    // Exponential analysis
    // -------------------------------------------------------------------------

    fn fit_exponential(
        &self,
        observations: &[CoherenceObservation],
    ) -> Result<RegressionFit, CoherenceError> {
        let mut config =
            RegressionConfig::production();

        config.confidence_level =
            crate::quantum::benchmarking::statistics::confidence::ConfidenceLevel::new(
                self.config.confidence_level,
            )
            .map_err(|_| {
                CoherenceError::NumericalFailure {
                    operation:
                        "confidence level construction",
                }
            })?;

        config.max_decay_rate =
            1.0 / MIN_COHERENCE_TIME_S;

        let engine =
            RegressionEngine::with_limits(
                config,
                self.limits,
            )?;

        let regression_observations =
            observations
                .iter()
                .map(|observation| {
                    if self.config.use_shot_weights {
                        let probability =
                            observation.signal
                                .clamp(0.0, 1.0);

                        let variance =
                            (probability
                                * (1.0 - probability))
                                / observation.shots
                                    as f64;

                        if variance > 0.0
                            && variance.is_finite()
                        {
                            RegressionObservation::weighted(
                                observation.delay_s,
                                probability,
                                1.0 / variance,
                            )
                        } else {
                            RegressionObservation::new(
                                observation.delay_s,
                                probability,
                            )
                        }
                    } else {
                        RegressionObservation::new(
                            observation.delay_s,
                            observation.signal,
                        )
                    }
                })
                .collect::<Vec<_>>();

        Ok(engine.fit(&regression_observations)?)
    }

    fn append_exponential_warnings(
        &self,
        regression: &RegressionFit,
        name: &str,
        warnings: &mut Vec<String>,
    ) {
        if !regression.diagnostics.converged {
            push_warning(
                warnings,
                &format!(
                    "{name} exponential regression did not \
                     report convergence."
                ),
            );
        }

        if regression.diagnostics.r_squared.is_finite()
            && regression.diagnostics.r_squared < 0.8
        {
            push_warning(
                warnings,
                &format!(
                    "{name} exponential fit has R² below 0.8; \
                     inspect residuals for non-exponential behaviour."
                ),
            );
        }

        if regression.decay_rate.value <= 0.0
            || !regression.decay_rate.value.is_finite()
        {
            push_warning(
                warnings,
                &format!(
                    "{name} produced an invalid positive decay rate."
                ),
            );
        }
    }

    // -------------------------------------------------------------------------
    // Ramsey analysis
    // -------------------------------------------------------------------------

    fn fit_ramsey(
        &self,
        observations: &[RamseyObservation],
    ) -> Result<RamseyInternalFit, CoherenceError> {
        let max_frequency =
            self.config.max_ramsey_frequency_hz;

        let min_decay =
            1.0 / self.config.max_coherence_time_s;

        let max_decay =
            1.0 / MIN_COHERENCE_TIME_S;

        if !min_decay.is_finite()
            || !max_decay.is_finite()
            || min_decay <= 0.0
            || max_decay <= min_decay
        {
            return Err(CoherenceError::NumericalFailure {
                operation: "Ramsey decay search bounds",
            });
        }

        let frequency_points =
            self.config.ramsey_frequency_grid_points;

        let decay_points =
            self.config.ramsey_decay_grid_points;

        let mut best: Option<RamseyCandidate> = None;

        for frequency_index in 0..frequency_points {
            let frequency =
                grid_value(
                    0.0,
                    max_frequency,
                    frequency_index,
                    frequency_points,
                );

            for decay_index in 0..decay_points {
                let decay =
                    log_grid_value(
                        min_decay,
                        max_decay,
                        decay_index,
                        decay_points,
                    );

                if let Some(candidate) =
                    self.evaluate_ramsey_candidate(
                        observations,
                        frequency,
                        decay,
                    )?
                {
                    if best
                        .as_ref()
                        .map(|current| {
                            candidate.sse < current.sse
                        })
                        .unwrap_or(true)
                    {
                        best = Some(candidate);
                    }
                }
            }
        }

        let mut candidate =
            best.ok_or(
                CoherenceError::NoFiniteRamseyFit,
            )?;

        let mut frequency_step =
            max_frequency
                / frequency_points.max(1) as f64;

        let mut decay_log_step =
            (max_decay.ln() - min_decay.ln())
                / decay_points.max(1) as f64;

        for _ in 0..self.config.ramsey_refinement_iterations {
            let frequency_low =
                (candidate.frequency
                    - frequency_step)
                    .max(0.0);

            let frequency_high =
                (candidate.frequency
                    + frequency_step)
                    .min(max_frequency);

            let decay_log =
                candidate.decay.max(min_decay).ln();

            let decay_low =
                (decay_log - decay_log_step)
                    .max(min_decay.ln());

            let decay_high =
                (decay_log + decay_log_step)
                    .min(max_decay.ln());

            let frequency_candidates = [
                frequency_low,
                candidate.frequency,
                frequency_high,
            ];

            let decay_candidates = [
                decay_low.exp(),
                candidate.decay,
                decay_high.exp(),
            ];

            let mut improved = false;

            for frequency in frequency_candidates {
                for decay in decay_candidates {
                    if let Some(next) =
                        self.evaluate_ramsey_candidate(
                            observations,
                            frequency,
                            decay,
                        )?
                    {
                        if next.sse + 1.0e-18
                            < candidate.sse
                        {
                            candidate = next;
                            improved = true;
                        }
                    }
                }
            }

            frequency_step *= 0.5;
            decay_log_step *= 0.5;

            if !improved
                && frequency_step
                    <= self.config.max_ramsey_frequency_hz
                        * 1.0e-12
                && decay_log_step <= 1.0e-12
            {
                break;
            }
        }

        let frequency_at_boundary =
            candidate.frequency <= frequency_step * 2.0
                || candidate.frequency
                    >= max_frequency - frequency_step * 2.0;

        let decay_at_boundary =
            candidate.decay <= min_decay * (1.0 + 1.0e-9)
                || candidate.decay
                    >= max_decay * (1.0 - 1.0e-9);

        let r_squared =
            calculate_r_squared(
                observations
                    .iter()
                    .map(|observation| observation.signal),
                candidate.sse,
            );

        Ok(RamseyInternalFit {
            decay_rate: candidate.decay,
            frequency_hz: candidate.frequency,
            offset: candidate.offset,
            cosine_amplitude: candidate.cosine_amplitude,
            sine_amplitude: candidate.sine_amplitude,
            fringe_amplitude: (
                candidate.cosine_amplitude
                    * candidate.cosine_amplitude
                    + candidate.sine_amplitude
                        * candidate.sine_amplitude
            )
            .sqrt(),
            residual_sum_squares: candidate.sse,
            rmse: (candidate.sse
                / observations.len() as f64)
                .sqrt(),
            r_squared,
            frequency_at_boundary,
            decay_at_boundary,
        })
    }

    fn evaluate_ramsey_candidate(
        &self,
        observations: &[RamseyObservation],
        frequency_hz: f64,
        decay_rate: f64,
    ) -> Result<Option<RamseyCandidate>, CoherenceError> {
        if !frequency_hz.is_finite()
            || frequency_hz < 0.0
            || frequency_hz
                > self.config.max_ramsey_frequency_hz
        {
            return Ok(None);
        }

        if !decay_rate.is_finite()
            || decay_rate <= 0.0
        {
            return Ok(None);
        }

        let omega =
            2.0 * std::f64::consts::PI
                * frequency_hz;

        let mut s00 = 0.0;
        let mut s01 = 0.0;
        let mut s02 = 0.0;
        let mut s11 = 0.0;
        let mut s12 = 0.0;
        let mut s22 = 0.0;

        let mut b0 = 0.0;
        let mut b1 = 0.0;
        let mut b2 = 0.0;

        for observation in observations {
            let weight =
                if self.config.use_shot_weights {
                    observation.shots as f64
                } else {
                    1.0
                };

            let envelope =
                (-decay_rate * observation.delay_s)
                    .exp();

            let phase =
                omega * observation.delay_s;

            let x0 = 1.0;
            let x1 =
                envelope * phase.cos();
            let x2 =
                envelope * phase.sin();

            let y = observation.signal;

            s00 += weight * x0 * x0;
            s01 += weight * x0 * x1;
            s02 += weight * x0 * x2;
            s11 += weight * x1 * x1;
            s12 += weight * x1 * x2;
            s22 += weight * x2 * x2;

            b0 += weight * x0 * y;
            b1 += weight * x1 * y;
            b2 += weight * x2 * y;
        }

        let determinant =
            determinant_3x3(
                s00,
                s01,
                s02,
                s01,
                s11,
                s12,
                s02,
                s12,
                s22,
            );

        if !determinant.is_finite()
            || determinant.abs()
                <= LINEAR_SYSTEM_EPSILON
        {
            return Ok(None);
        }

        let offset =
            determinant_3x3(
                b0,
                s01,
                s02,
                b1,
                s11,
                s12,
                b2,
                s12,
                s22,
            ) / determinant;

        let cosine_amplitude =
            determinant_3x3(
                s00,
                b0,
                s02,
                s01,
                b1,
                s12,
                s02,
                b2,
                s22,
            ) / determinant;

        let sine_amplitude =
            determinant_3x3(
                s00,
                s01,
                b0,
                s01,
                s11,
                b1,
                s02,
                s12,
                b2,
            ) / determinant;

        if !offset.is_finite()
            || !cosine_amplitude.is_finite()
            || !sine_amplitude.is_finite()
        {
            return Err(CoherenceError::NumericalFailure {
                operation: "Ramsey linear least-squares solution",
            });
        }

        let mut sse = 0.0;

        for observation in observations {
            let envelope =
                (-decay_rate * observation.delay_s)
                    .exp();

            let phase =
                omega * observation.delay_s;

            let predicted =
                offset
                    + envelope
                        * (cosine_amplitude
                            * phase.cos()
                            + sine_amplitude
                                * phase.sin());

            let residual =
                observation.signal - predicted;

            sse += residual * residual;
        }

        if !sse.is_finite() {
            return Err(CoherenceError::NumericalFailure {
                operation: "Ramsey residual calculation",
            });
        }

        Ok(Some(RamseyCandidate {
            frequency: frequency_hz,
            decay: decay_rate,
            offset,
            cosine_amplitude,
            sine_amplitude,
            sse,
        }))
    }

    fn validate_observation_count(
        &self,
        count: usize,
    ) -> Result<(), CoherenceError> {
        if count == 0 {
            return Err(CoherenceError::EmptyObservations);
        }

        if count < MIN_EXPONENTIAL_OBSERVATIONS {
            return Err(
                CoherenceError::InsufficientObservations {
                    actual: count,
                    minimum: MIN_EXPONENTIAL_OBSERVATIONS,
                },
            );
        }

        if count > self.config.max_delay_points {
            return Err(
                CoherenceError::TooManyObservations {
                    actual: count,
                    maximum: self.config.max_delay_points,
                },
            );
        }

        self.limits
            .check_observations(count as u64)?;

        Ok(())
    }

    fn validate_shots(
        &self,
        index: usize,
        shots: u64,
    ) -> Result<(), CoherenceError> {
        if shots == 0 {
            return Err(
                CoherenceError::InvalidShotCount { index },
            );
        }

        if shots > self.config.max_shots_per_delay {
            return Err(CoherenceError::TooManyShots {
                index,
                shots,
                maximum: self.config.max_shots_per_delay,
            });
        }

        self.limits.check_shots(shots)?;

        Ok(())
    }
}

// =============================================================================
// Internal fit types
// =============================================================================

#[derive(Debug, Clone, Copy)]
struct RamseyCandidate {
    frequency: f64,
    decay: f64,
    offset: f64,
    cosine_amplitude: f64,
    sine_amplitude: f64,
    sse: f64,
}

#[derive(Debug, Clone, Copy)]
struct RamseyInternalFit {
    decay_rate: f64,
    frequency_hz: f64,
    offset: f64,
    cosine_amplitude: f64,
    sine_amplitude: f64,
    fringe_amplitude: f64,
    residual_sum_squares: f64,
    rmse: f64,
    r_squared: Option<f64>,
    frequency_at_boundary: bool,
    decay_at_boundary: bool,
}

// =============================================================================
// Validation helpers
// =============================================================================

fn validate_confidence_level(
    value: f64,
) -> Result<(), CoherenceError> {
    if !value.is_finite()
        || !(0.0 < value && value < 1.0)
    {
        return Err(CoherenceError::NumericalFailure {
            operation: "confidence level validation",
        });
    }

    Ok(())
}

fn validate_delay(
    index: usize,
    delay_s: f64,
    maximum_s: f64,
) -> Result<(), CoherenceError> {
    if !delay_s.is_finite()
        || delay_s < 0.0
        || delay_s > maximum_s
    {
        return Err(CoherenceError::InvalidDelay {
            index,
            delay_s,
        });
    }

    Ok(())
}

fn validate_unit_interval_signal(
    index: usize,
    value: f64,
) -> Result<(), CoherenceError> {
    if !value.is_finite() {
        return Err(CoherenceError::NonFiniteSignal {
            index,
            value,
        });
    }

    if value < -UNIT_INTERVAL_EPSILON
        || value > 1.0 + UNIT_INTERVAL_EPSILON
    {
        return Err(CoherenceError::InvalidSignal {
            index,
            value,
        });
    }

    Ok(())
}

fn validate_frequency(
    value_hz: f64,
    maximum_hz: f64,
) -> Result<(), CoherenceError> {
    if !value_hz.is_finite()
        || value_hz < 0.0
    {
        return Err(CoherenceError::InvalidFrequency {
            value_hz,
        });
    }

    if value_hz > maximum_hz {
        return Err(
            CoherenceError::FrequencyTooLarge {
                value_hz,
                maximum_hz,
            },
        );
    }

    Ok(())
}

fn validate_coherence_time(
    value_s: f64,
    maximum_s: f64,
) -> Result<(), CoherenceError> {
    if !value_s.is_finite()
        || value_s < MIN_COHERENCE_TIME_S
        || value_s > maximum_s
    {
        return Err(
            CoherenceError::InvalidFittedCoherenceTime {
                value_s,
            },
        );
    }

    Ok(())
}

// =============================================================================
// Numerical helpers
// =============================================================================

fn total_shots(
    observations: &[CoherenceObservation],
) -> u64 {
    observations
        .iter()
        .map(|observation| observation.shots)
        .fold(0_u64, u64::saturating_add)
}

/// Linear grid including both endpoints when there is more than one point.
fn grid_value(
    minimum: f64,
    maximum: f64,
    index: usize,
    points: usize,
) -> f64 {
    if points <= 1 {
        return minimum;
    }

    minimum
        + (maximum - minimum)
            * index as f64
            / (points - 1) as f64
}

/// Logarithmic grid for strictly positive values.
fn log_grid_value(
    minimum: f64,
    maximum: f64,
    index: usize,
    points: usize,
) -> f64 {
    if points <= 1 {
        return minimum;
    }

    let log_min = minimum.ln();
    let log_max = maximum.ln();

    (log_min
        + (log_max - log_min)
            * index as f64
            / (points - 1) as f64)
        .exp()
}

fn determinant_3x3(
    a00: f64,
    a01: f64,
    a02: f64,
    a10: f64,
    a11: f64,
    a12: f64,
    a20: f64,
    a21: f64,
    a22: f64,
) -> f64 {
    a00 * (a11 * a22 - a12 * a21)
        - a01 * (a10 * a22 - a12 * a20)
        + a02 * (a10 * a21 - a11 * a20)
}

fn calculate_r_squared<I>(
    values: I,
    residual_sum_squares: f64,
) -> Option<f64>
where
    I: Iterator<Item = f64>,
{
    let values: Vec<f64> = values.collect();

    if values.is_empty()
        || !residual_sum_squares.is_finite()
    {
        return None;
    }

    let mean =
        values.iter().copied().sum::<f64>()
            / values.len() as f64;

    let total_sum_squares =
        values
            .iter()
            .map(|value| {
                let difference = *value - mean;
                difference * difference
            })
            .sum::<f64>();

    if !total_sum_squares.is_finite() {
        return None;
    }

    if total_sum_squares <= 0.0 {
        return None;
    }

    let r_squared =
        1.0 - residual_sum_squares
            / total_sum_squares;

    if r_squared.is_finite() {
        Some(r_squared.clamp(-1.0, 1.0))
    } else {
        None
    }
}

fn push_warning(
    warnings: &mut Vec<String>,
    warning: &str,
) {
    if warnings.len() < MAX_WARNINGS
        && !warnings.iter().any(|item| item == warning)
    {
        warnings.push(warning.to_string());
    }
}

/// Approximate two-sided normal quantile.
///
/// This is the Acklam-style rational approximation and is used only for
/// transforming regression standard errors into approximate confidence bounds.
///
/// It deliberately avoids a statistical dependency for this protocol-level
/// transformation. Exact confidence-interval machinery remains owned by
/// `statistics::confidence`.
fn normal_quantile(
    confidence_level: f64,
) -> f64 {
    let p =
        0.5 + confidence_level / 2.0;

    inverse_standard_normal(p)
}

fn inverse_standard_normal(
    p: f64,
) -> f64 {
    if p <= 0.0 {
        return f64::NEG_INFINITY;
    }

    if p >= 1.0 {
        return f64::INFINITY;
    }

    // Coefficients for the inverse normal CDF approximation.
    const A1: f64 = -3.969683028665376e1;
    const A2: f64 = 2.209460984245205e2;
    const A3: f64 = -2.759285104469687e2;
    const A4: f64 = 1.383577518672690e2;
    const A5: f64 = -3.066479806614716e1;
    const A6: f64 = 2.506628277459239e0;

    const B1: f64 = -5.447609879822406e1;
    const B2: f64 = 1.615858368580409e2;
    const B3: f64 = -1.556989798598866e2;
    const B4: f64 = 6.680131188771972e1;
    const B5: f64 = -1.328068155288572e1;

    const C1: f64 = -7.784894002430293e-3;
    const C2: f64 = -3.223964580411365e-1;
    const C3: f64 = -2.400758277161838e0;
    const C4: f64 = -2.549732539343734e0;
    const C5: f64 = 4.374664141464968e0;
    const C6: f64 = 2.938163982698783e0;

    const D1: f64 = 7.784695709041462e-3;
    const D2: f64 = 3.224671290700398e-1;
    const D3: f64 = 2.445134137142996e0;
    const D4: f64 = 3.754408661907416e0;

    const P_LOW: f64 = 0.02425;
    const P_HIGH: f64 = 1.0 - P_LOW;

    if p < P_LOW {
        let q =
            (-2.0 * p.ln()).sqrt();

        return (((((C1 * q + C2) * q + C3)
            * q
            + C4)
            * q
            + C5)
            * q
            + C6)
            / ((((D1 * q + D2) * q + D3)
                * q
                + D4)
                * q
                + 1.0);
    }

    if p > P_HIGH {
        let q =
            (-2.0 * (1.0 - p).ln()).sqrt();

        return -(((((C1 * q + C2) * q + C3)
            * q
            + C4)
            * q
            + C5)
            * q
            + C6)
            / ((((D1 * q + D2) * q + D3)
                * q
                + D4)
                * q
                + 1.0));
    }

    let q = p - 0.5;
    let r = q * q;

    (((((A1 * r + A2) * r + A3)
        * r
        + A4)
        * r
        + A5)
        * r
        + A6)
        * q
        / (((((B1 * r + B2) * r + B3)
            * r
            + B4)
            * r
            + B5)
            * r
            + 1.0)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn analyzer() -> CoherenceAnalyzer {
        CoherenceAnalyzer::new(
            CoherenceConfig::production(0),
        )
        .expect("production configuration must be valid")
    }

    fn exponential_data(
        tau_s: f64,
    ) -> Vec<CoherenceObservation> {
        let delays = [
            0.0,
            tau_s * 0.25,
            tau_s * 0.5,
            tau_s,
            tau_s * 1.5,
            tau_s * 2.0,
            tau_s * 3.0,
            tau_s * 4.0,
        ];

        delays
            .iter()
            .map(|delay| {
                CoherenceObservation::new(
                    *delay,
                    (-delay / tau_s).exp(),
                    100_000,
                )
            })
            .collect()
    }

    #[test]
    fn t1_recovers_known_exponential_time() {
        let analyzer = analyzer();

        let observations =
            exponential_data(20.0e-6);

        let result =
            analyzer
                .analyze_t1(&observations)
                .expect("T1 fit should succeed");

        let relative_error =
            (result.t1.value_s - 20.0e-6).abs()
                / 20.0e-6;

        assert!(
            relative_error < 0.02,
            "relative error was {relative_error}"
        );
    }

    #[test]
    fn t2_hahn_recovers_known_exponential_time() {
        let analyzer = analyzer();

        let observations =
            exponential_data(30.0e-6);

        let result =
            analyzer
                .analyze_t2_hahn(&observations)
                .expect("T2 fit should succeed");

        let relative_error =
            (result.t2.value_s - 30.0e-6).abs()
                / 30.0e-6;

        assert!(
            relative_error < 0.02,
            "relative error was {relative_error}"
        );
    }

    #[test]
    fn tphi_rate_is_derived_correctly() {
        let analyzer = analyzer();

        let t1 =
            CoherenceTimeEstimate {
                value_s: 100.0,
                standard_error_s: None,
                lower_s: None,
                upper_s: None,
                confidence_level: 0.95,
                analysis_version:
                    T1_ANALYSIS_VERSION.to_string(),
            };

        let t2 =
            CoherenceTimeEstimate {
                value_s: 80.0,
                standard_error_s: None,
                lower_s: None,
                upper_s: None,
                confidence_level: 0.95,
                analysis_version:
                    T2_HAHN_ANALYSIS_VERSION
                        .to_string(),
            };

        let result =
            analyzer
                .derive_tphi(&t1, &t2)
                .expect("Tphi derivation should succeed");

        let expected =
            1.0 / 80.0 - 1.0 / (2.0 * 100.0);

        assert!(
            (result.rate_per_s - expected).abs()
                < 1.0e-12
        );

        assert!(
            result.time_s.is_some()
        );
    }

    #[test]
    fn non_positive_tphi_rate_is_not_reported_as_infinity() {
        let analyzer = analyzer();

        let t1 =
            CoherenceTimeEstimate {
                value_s: 10.0,
                standard_error_s: None,
                lower_s: None,
                upper_s: None,
                confidence_level: 0.95,
                analysis_version:
                    T1_ANALYSIS_VERSION.to_string(),
            };

        let t2 =
            CoherenceTimeEstimate {
                value_s: 20.0,
                standard_error_s: None,
                lower_s: None,
                upper_s: None,
                confidence_level: 0.95,
                analysis_version:
                    T2_HAHN_ANALYSIS_VERSION
                        .to_string(),
            };

        let result =
            analyzer
                .derive_tphi(&t1, &t2)
                .expect("Tphi derivation should not fail");

        assert!(!result.physically_positive);
        assert!(result.time_s.is_none());
        assert!(result.rate_per_s <= RATE_EPSILON);
    }

    #[test]
    fn invalid_probability_is_rejected() {
        let analyzer = analyzer();

        let observations = [
            CoherenceObservation::new(
                0.0,
                1.1,
                100,
            ),
            CoherenceObservation::new(
                1.0,
                0.8,
                100,
            ),
            CoherenceObservation::new(
                2.0,
                0.6,
                100,
            ),
            CoherenceObservation::new(
                3.0,
                0.4,
                100,
            ),
        ];

        assert!(
            analyzer
                .validate_observations(&observations)
                .is_err()
        );
    }

    #[test]
    fn duplicate_delays_are_rejected() {
        let analyzer = analyzer();

        let observations = [
            CoherenceObservation::new(
                0.0,
                1.0,
                100,
            ),
            CoherenceObservation::new(
                1.0,
                0.8,
                100,
            ),
            CoherenceObservation::new(
                1.0,
                0.7,
                100,
            ),
            CoherenceObservation::new(
                2.0,
                0.6,
                100,
            ),
        ];

        assert!(
            analyzer
                .validate_observations(&observations)
                .is_err()
        );
    }

    #[test]
    fn zero_shots_are_rejected() {
        let analyzer = analyzer();

        let observations = [
            CoherenceObservation::new(
                0.0,
                1.0,
                100,
            ),
            CoherenceObservation::new(
                1.0,
                0.8,
                100,
            ),
            CoherenceObservation::new(
                2.0,
                0.6,
                0,
            ),
            CoherenceObservation::new(
                3.0,
                0.4,
                100,
            ),
        ];

        assert!(
            analyzer
                .validate_observations(&observations)
                .is_err()
        );
    }

    #[test]
    fn ramsey_fit_recovers_known_frequency_and_decay() {
        let mut config =
            CoherenceConfig::production(0);

        config.max_ramsey_frequency_hz =
            2.0e6;

        config.ramsey_frequency_grid_points =
            48;

        config.ramsey_decay_grid_points =
            48;

        config.ramsey_refinement_iterations =
            32;

        let analyzer =
            CoherenceAnalyzer::new(config)
                .expect("configuration must be valid");

        let t2_star = 40.0e-6;
        let frequency = 250_000.0;

        let mut observations =
            Vec::new();

        for index in 0..40 {
            let delay =
                index as f64 * 2.0e-6;

            let signal =
                (-delay / t2_star).exp()
                    * (2.0
                        * std::f64::consts::PI
                        * frequency
                        * delay)
                        .cos();

            observations.push(
                RamseyObservation::new(
                    delay,
                    signal,
                    100_000,
                ),
            );
        }

        let result =
            analyzer
                .analyze_t2_star(&observations)
                .expect("Ramsey fit should succeed");

        let frequency_error =
            (result.frequency_hz - frequency)
                .abs()
                / frequency;

        let time_error =
            (result.t2_star.value_s - t2_star)
                .abs()
                / t2_star;

        assert!(
            frequency_error < 0.05,
            "frequency error was {frequency_error}"
        );

        assert!(
            time_error < 0.10,
            "T2* error was {time_error}"
        );
    }

    #[test]
    fn tphi_uncertainty_propagates_from_t1_and_t2() {
        let analyzer = analyzer();

        let t1 =
            CoherenceTimeEstimate {
                value_s: 100.0,
                standard_error_s: Some(1.0),
                lower_s: None,
                upper_s: None,
                confidence_level: 0.95,
                analysis_version:
                    T1_ANALYSIS_VERSION.to_string(),
            };

        let t2 =
            CoherenceTimeEstimate {
                value_s: 80.0,
                standard_error_s: Some(1.0),
                lower_s: None,
                upper_s: None,
                confidence_level: 0.95,
                analysis_version:
                    T2_HAHN_ANALYSIS_VERSION
                        .to_string(),
            };

        let result =
            analyzer
                .derive_tphi(&t1, &t2)
                .expect("Tphi should succeed");

        assert!(
            result
                .rate_standard_error_per_s
                .is_some()
        );

        assert!(
            result
                .time_standard_error_s
                .is_some()
        );
    }

    #[test]
    fn configuration_rejects_invalid_confidence() {
        let mut config =
            CoherenceConfig::production(0);

        config.confidence_level = 1.0;

        assert!(
            CoherenceAnalyzer::new(config)
                .is_err()
        );
    }

    #[test]
    fn frequency_grid_is_deterministic() {
        let a =
            grid_value(
                0.0,
                100.0,
                5,
                11,
            );

        let b =
            grid_value(
                0.0,
                100.0,
                5,
                11,
            );

        assert_eq!(a, b);
    }

    #[test]
    fn logarithmic_grid_is_positive() {
        for index in 0..20 {
            let value =
                log_grid_value(
                    1.0e-6,
                    1.0e6,
                    index,
                    20,
                );

            assert!(
                value.is_finite()
                    && value > 0.0
            );
        }
    }

    #[test]
    fn result_schema_is_stable() {
        let result =
            CoherenceResult::new(3);

        assert_eq!(
            result.benchmark_id,
            COHERENCE_BENCHMARK_ID
        );

        assert_eq!(
            result.protocol_version,
            COHERENCE_PROTOCOL_VERSION
        );

        assert_eq!(
            result.result_schema_version,
            COHERENCE_RESULT_SCHEMA_VERSION
        );

        assert_eq!(
            result.qubit,
            3
        );
    }
}