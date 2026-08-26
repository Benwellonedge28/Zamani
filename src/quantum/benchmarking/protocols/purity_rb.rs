//! Zamani Quantum Benchmarking — Purity / Unitarity Randomized Benchmarking.
//!
//! Production implementation of Purity Randomized Benchmarking (Purity RB),
//! also known as Unitarity Randomized Benchmarking (URB).
//!
//! # Purpose
//!
//! Purity RB characterizes the coherence of noise in a quantum gate set by
//! measuring the decay of output-state purity under randomized Clifford
//! sequences.
//!
//! The central protocol quantity is the unitarity `u` of the effective noise
//! channel. In the idealized gate-independent, time-independent setting:
//
//!     shifted_purity(m) = A * u^m + B
//!
//! or, under the conventional RB indexing:
//
//!     shifted_purity(m) = A * u^(m - 1) + B
//!
//! depending on the selected [`DecayExponentConvention`].
//!
//! `u` lies in `[0, 1]` for a physical channel:
//!
//!     u = 1
//!
//! corresponds to purely unitary/coherent noise,
//! while smaller values indicate increasing irreversible/stochastic
//! contraction of the traceless component.
//!
//! IMPORTANT:
//!
//! `u = 1` does NOT mean that the device is accurate. A perfectly coherent
//! but badly calibrated unitary error can have unitarity one.
//!
//! # Scientific model
//!
//! For a `d`-dimensional state:
//
//!     purity(ρ) = Tr(ρ²)
//!
//! and the shifted/normalized purity used by unitarity RB is:
//
//!     shifted_purity(ρ)
//!         = (d * Tr(ρ²) - 1) / (d - 1)
//!
//! For a Pauli expansion:
//
//!     ρ = (I + Σ r_P P) / d
//!
//! the shifted purity is the normalized squared length of the traceless
//! generalized Bloch vector.
//!
//! For a one-qubit state:
//
//!     shifted_purity = <X>² + <Y>² + <Z>²
//!
//! For multiple qubits, the corresponding expression contains the
//! non-identity Pauli components.
//!
//! # Protocol
//!
//! A standard experiment is:
//
//! 1. Select sequence lengths.
//! 2. For each length, generate many independent random Clifford sequences.
//! 3. Apply the sequences to the selected qubits.
//! 4. Estimate output-state purity for every sequence.
//! 5. Average purity over random sequences at each length.
//! 6. Fit the resulting decay curve.
//! 7. Extract the decay parameter `u`.
//! 8. Report fit diagnostics and uncertainty.
//!
//! Purity RB does NOT require the sequence to be self-inverting merely to
//! estimate purity. The protocol therefore deliberately does not duplicate
//! the recovery-sequence machinery owned by `generators::clifford`.
//!
//! # Architecture
//!
//! This file owns:
//
//! - Purity RB configuration;
//! - experiment planning metadata;
//! - purity observation normalization;
//! - Pauli-based purity calculation;
//! - aggregation of per-sequence purity;
//! - exponential-decay analysis;
//! - unitarity extraction;
//! - purity-RB EPC transformation;
//! - scientific diagnostics and warnings;
//! - protocol-specific result representation.
//!
//! This file does NOT own:
//
//! - Clifford generation;
//! - random-number generation;
//! - Quantum IR;
//! - circuit lowering;
//! - routing;
//! - scheduling;
//! - backend selection;
//! - hardware execution;
//! - calibration;
//! - generic regression mathematics;
//! - generic confidence-interval mathematics;
//! - report formatting.
//!
//! Those responsibilities belong to:
//
//! ```text
//! generators/random.rs
//! generators/clifford.rs
//! core/
//! execution/
//! statistics/regression.rs
//! reporting/
//! ```
//!
//! # Integration direction
//!
//! ```text
//! generators/random.rs
//!          │
//!          ▼
//! generators/clifford.rs
//!          │
//!          ▼
//! protocols/purity_rb.rs
//!          │
//!          ├──────────────► statistics/regression.rs
//!          │
//!          ├──────────────► core limits/config/result
//!          │
//!          ▼
//! execution/executor.rs
//!          │
//!          ▼
//! quantum::ir / runtime / hardware
//! ```
//!
//! The protocol consumes generated Clifford sequences and execution
//! observations. It never constructs hardware-specific circuits itself.
//!
//! # Purity estimation
//!
//! The [`PauliPurityEstimator`] supports two-copy-independent single-copy
//! Pauli expectation measurements:
//
//!     q = Σ_{P != I} <P>² / (d² - 1)
//!
//! This is the normalized shifted purity when the expectation values are
//! represented in the standard Pauli basis.
//!
//! The implementation also supports directly supplied physical purity
//! `Tr(ρ²)` through [`shifted_purity_from_raw_purity`].
//!
//! # Statistical policy
//!
//! The protocol uses the canonical Zamani exponential regression engine:
//
//!     y = A * p^x + B
//!
//! where:
//
//!     p = u
//!
//! for Purity RB.
//!
//! The regression engine is shared with standard RB, interleaved RB, cycle
//! benchmarking and other decay protocols. This file therefore does not
//! duplicate nonlinear fitting.
//!
//! # Important statistical limitations
//!
//! A successful exponential fit does not prove that the physical noise is
//! Markovian, gate-independent or time-independent.
//!
//! Non-Markovian and time-dependent noise can produce non-exponential or
//! multi-exponential behavior. Fit quality, residuals, boundary solutions and
//! convergence diagnostics are therefore exposed in the final result.
//!
//! This follows the broader randomized-benchmarking literature, which treats
//! exponential decay as a model whose physical interpretation depends on
//! assumptions about the noise process.
//!
//! # Resource safety
//!
//! User-controlled benchmark parameters are validated before creating large
//! experiment plans.
//!
//! In particular:
//
//! - zero qubits are rejected;
//! - zero sequence lengths are rejected;
//! - duplicate sequence lengths are rejected;
//! - excessive sequence lengths are rejected;
//! - excessive sequence counts are rejected;
//! - excessive shots are rejected;
//! - arithmetic overflow is rejected;
//! - non-finite purity values are rejected;
//! - invalid Pauli expectation values are rejected;
//! - regression resource limits remain enforced by `RegressionEngine`.
//!
//! # Reproducibility
//!
//! This module does not create implicit randomness.
//!
//! A higher-level generator must provide:
//
//! - benchmark seed;
//! - generator version;
//! - Clifford representation version;
//! - sequence identity;
//! - sequence length;
//! - sequence index.
//!
//! These identifiers can be attached to [`PuritySequenceObservation`] and
//! later propagated into the universal benchmark provenance/result layer.
//!
//! # Rust compatibility
//!
//! Target:
//
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021
//!
//! Stable Rust only. No nightly features are required.
//!
//! # Integration contract
//!
//! This file is intentionally designed so that future implementation of:
//
//! - `protocols/mod.rs`;
//! - `core/result.rs`;
//! - `execution/executor.rs`;
//! - `reporting/*`;
//! - Zamani-language benchmark syntax
//!
//! does not require changing the mathematical Purity RB API.
//!
//! The protocol result can be wrapped into the universal `BenchmarkResult`
//! without changing the protocol analysis itself.

use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;

use crate::quantum::benchmarking::statistics::regression::{
    RegressionConfig,
    RegressionEngine,
    RegressionError,
    RegressionFit,
    RegressionObservation,
};

// =============================================================================
// Public protocol identity
// =============================================================================

/// Stable benchmark identifier.
pub const PURITY_RB_BENCHMARK_ID: &str = "purity_randomized_benchmarking";

/// Short stable benchmark identifier.
pub const PURITY_RB_SHORT_ID: &str = "purity_rb";

/// Protocol family identifier.
pub const PURITY_RB_FAMILY_ID: &str = "unitarity_randomized_benchmarking";

/// Protocol result schema version.
///
/// Increment when the serialized semantic meaning of the result changes.
pub const PURITY_RB_RESULT_SCHEMA_VERSION: u32 = 1;

/// Protocol implementation version.
///
/// Increment when the protocol implementation changes in a way that can
/// alter generated experiments or analyzed results.
pub const PURITY_RB_PROTOCOL_VERSION: u32 = 1;

/// Stable purity-estimation algorithm identifier.
pub const PURITY_ESTIMATOR_VERSION: &str =
    "zamani.purity.pauli_shifted.v1";

/// Stable analysis algorithm identifier.
pub const PURITY_RB_ANALYSIS_VERSION: &str =
    "zamani.purity_rb.analysis.v1";

/// Default maximum sequence length.
///
/// This is a protocol-level guard. Global benchmark limits remain authoritative
/// for complete experiments.
pub const DEFAULT_MAX_SEQUENCE_LENGTH: usize = 1_000_000;

/// Default maximum number of random sequences per length.
pub const DEFAULT_MAX_SEQUENCES_PER_LENGTH: usize = 100_000;

/// Default maximum shots per sequence.
pub const DEFAULT_MAX_SHOTS_PER_SEQUENCE: usize = 10_000_000;

/// Default minimum number of distinct sequence lengths.
pub const DEFAULT_MIN_SEQUENCE_LENGTHS: usize = 4;

/// Default minimum number of random sequences per length.
///
/// This is a statistical warning threshold, not an absolute scientific law.
pub const DEFAULT_MIN_SEQUENCES_PER_LENGTH: usize = 3;

/// Default minimum shots per sequence.
///
/// Again, this is a warning threshold rather than a universal theorem.
pub const DEFAULT_MIN_SHOTS_PER_SEQUENCE: usize = 100;

/// Numerical tolerance for values expected to lie in `[0, 1]`.
const UNIT_INTERVAL_EPSILON: f64 = 1.0e-12;

/// Numerical tolerance for normalized sequence lengths.
const INTEGER_TOLERANCE: f64 = 1.0e-12;

// =============================================================================
// Error type
// =============================================================================

/// Errors produced by the Purity RB protocol.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PurityRbError {
    /// No qubits were supplied.
    InvalidQubitCount,

    /// The requested qubit dimension could not be represented safely.
    DimensionOverflow {
        qubits: usize,
    },

    /// A sequence length was zero.
    InvalidSequenceLength {
        length: usize,
    },

    /// A sequence length exceeds the protocol safety limit.
    SequenceLengthTooLarge {
        length: usize,
        maximum: usize,
    },

    /// Sequence lengths contain duplicates.
    DuplicateSequenceLength {
        length: usize,
    },

    /// Too few distinct sequence lengths were supplied.
    InsufficientSequenceLengths {
        actual: usize,
        minimum: usize,
    },

    /// Too many sequences were requested for one length.
    TooManySequences {
        requested: usize,
        maximum: usize,
    },

    /// Zero sequences were requested.
    InvalidSequenceCount,

    /// Too many shots were requested.
    TooManyShots {
        requested: usize,
        maximum: usize,
    },

    /// Zero shots were requested.
    InvalidShotCount,

    /// Arithmetic overflow occurred while calculating total workload.
    WorkloadOverflow,

    /// A confidence level is invalid.
    InvalidConfidenceLevel {
        value: f64,
    },

    /// A decay exponent is invalid.
    InvalidDecayExponent {
        length: usize,
    },

    /// A purity value is not finite.
    NonFinitePurity {
        value: f64,
    },

    /// A shifted purity is outside its physical interval by more than the
    /// accepted numerical tolerance.
    InvalidShiftedPurity {
        value: f64,
    },

    /// A raw purity is outside the physical interval.
    InvalidRawPurity {
        value: f64,
    },

    /// A Pauli expectation is not finite.
    NonFinitePauliExpectation {
        index: usize,
        value: f64,
    },

    /// A Pauli expectation lies outside `[-1, 1]`.
    InvalidPauliExpectation {
        index: usize,
        value: f64,
    },

    /// No Pauli observables were supplied.
    EmptyPauliExpectations,

    /// A Pauli expectation variance is invalid.
    InvalidPauliVariance {
        index: usize,
        value: f64,
    },

    /// A sequence observation has invalid identity information.
    InvalidSequenceIdentity,

    /// A sequence observation has no purity measurements.
    EmptySequenceObservation,

    /// Regression failed.
    Regression(RegressionError),

    /// The fitted unitarity is outside the physical interval.
    InvalidFittedUnitarity {
        value: f64,
    },

    /// The fitted result is numerically unusable.
    InvalidFit,

    /// The supplied regression configuration is invalid.
    InvalidRegressionConfiguration,

    /// An internal numerical calculation failed.
    NumericalFailure {
        operation: &'static str,
    },
}

impl fmt::Display for PurityRbError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidQubitCount => {
                write!(formatter, "Purity RB requires at least one qubit")
            }

            Self::DimensionOverflow { qubits } => {
                write!(
                    formatter,
                    "Hilbert-space dimension for {qubits} qubits \
                     cannot be represented safely"
                )
            }

            Self::InvalidSequenceLength { length } => {
                write!(
                    formatter,
                    "Purity RB sequence length must be greater than zero, \
                     got {length}"
                )
            }

            Self::SequenceLengthTooLarge { length, maximum } => {
                write!(
                    formatter,
                    "Purity RB sequence length {length} exceeds maximum {maximum}"
                )
            }

            Self::DuplicateSequenceLength { length } => {
                write!(
                    formatter,
                    "Purity RB sequence length {length} was supplied more than once"
                )
            }

            Self::InsufficientSequenceLengths { actual, minimum } => {
                write!(
                    formatter,
                    "Purity RB requires at least {minimum} distinct sequence \
                     lengths, got {actual}"
                )
            }

            Self::TooManySequences {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "Purity RB requests {requested} sequences per length; \
                     maximum is {maximum}"
                )
            }

            Self::InvalidSequenceCount => {
                write!(
                    formatter,
                    "Purity RB requires at least one random sequence per length"
                )
            }

            Self::TooManyShots {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "Purity RB requests {requested} shots per sequence; \
                     maximum is {maximum}"
                )
            }

            Self::InvalidShotCount => {
                write!(
                    formatter,
                    "Purity RB requires at least one shot per sequence"
                )
            }

            Self::WorkloadOverflow => {
                write!(
                    formatter,
                    "Purity RB workload size calculation overflowed"
                )
            }

            Self::InvalidConfidenceLevel { value } => {
                write!(
                    formatter,
                    "invalid Purity RB confidence level: {value}"
                )
            }

            Self::InvalidDecayExponent { length } => {
                write!(
                    formatter,
                    "invalid Purity RB decay exponent for sequence length {length}"
                )
            }

            Self::NonFinitePurity { value } => {
                write!(
                    formatter,
                    "Purity RB received a non-finite purity value: {value}"
                )
            }

            Self::InvalidShiftedPurity { value } => {
                write!(
                    formatter,
                    "shifted purity {value} is outside the physical interval [0, 1]"
                )
            }

            Self::InvalidRawPurity { value } => {
                write!(
                    formatter,
                    "raw purity {value} is outside the physical interval [1/d, 1]"
                )
            }

            Self::NonFinitePauliExpectation { index, value } => {
                write!(
                    formatter,
                    "Pauli expectation at index {index} is non-finite: {value}"
                )
            }

            Self::InvalidPauliExpectation { index, value } => {
                write!(
                    formatter,
                    "Pauli expectation at index {index} must lie in [-1, 1], \
                     got {value}"
                )
            }

            Self::EmptyPauliExpectations => {
                write!(
                    formatter,
                    "at least one non-identity Pauli expectation is required"
                )
            }

            Self::InvalidPauliVariance { index, value } => {
                write!(
                    formatter,
                    "Pauli expectation variance at index {index} is invalid: {value}"
                )
            }

            Self::InvalidSequenceIdentity => {
                write!(
                    formatter,
                    "Purity RB sequence identity must not be empty"
                )
            }

            Self::EmptySequenceObservation => {
                write!(
                    formatter,
                    "Purity RB sequence observation contains no measurements"
                )
            }

            Self::Regression(error) => {
                write!(formatter, "Purity RB regression failed: {error}")
            }

            Self::InvalidFittedUnitarity { value } => {
                write!(
                    formatter,
                    "fitted Purity RB unitarity {value} is outside [0, 1]"
                )
            }

            Self::InvalidFit => {
                write!(
                    formatter,
                    "Purity RB produced a numerically unusable fit"
                )
            }

            Self::InvalidRegressionConfiguration => {
                write!(
                    formatter,
                    "Purity RB regression configuration is invalid"
                )
            }

            Self::NumericalFailure { operation } => {
                write!(
                    formatter,
                    "Purity RB numerical failure during {operation}"
                )
            }
        }
    }
}

impl Error for PurityRbError {}

impl From<RegressionError> for PurityRbError {
    fn from(error: RegressionError) -> Self {
        Self::Regression(error)
    }
}

/// Result type for Purity RB operations.
pub type PurityRbResult<T> = Result<T, PurityRbError>;

// =============================================================================
// Decay convention
// =============================================================================

/// Defines which exponent is supplied to the decay model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DecayExponentConvention {
    /// Fit:
    ///
    ///     y(m) = A * u^m + B
    SequenceLength,

    /// Fit the conventional RB-style model:
    ///
    ///     y(m) = A * u^(m - 1) + B
    ///
    /// This is the default for compatibility with the conventional unitarity
    /// RB formulation.
    SequenceLengthMinusOne,
}

impl Default for DecayExponentConvention {
    fn default() -> Self {
        Self::SequenceLengthMinusOne
    }
}

impl DecayExponentConvention {
    /// Converts a sequence length into the regression exponent.
    pub fn exponent(self, length: usize) -> PurityRbResult<f64> {
        match self {
            Self::SequenceLength => {
                if length == 0 {
                    return Err(PurityRbError::InvalidDecayExponent {
                        length,
                    });
                }

                Ok(length as f64)
            }

            Self::SequenceLengthMinusOne => {
                if length == 0 {
                    return Err(PurityRbError::InvalidDecayExponent {
                        length,
                    });
                }

                Ok((length - 1) as f64)
            }
        }
    }
}

// =============================================================================
// Purity measurement model
// =============================================================================

/// Purity measurement strategy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PurityMeasurementMode {
    /// Purity reconstructed from all non-identity Pauli expectation values.
    ///
    /// This is the most direct single-copy implementation.
    CompletePauli,

    /// Purity reconstructed from a declared subset of non-identity Pauli
    /// observables.
    ///
    /// This is useful for scalable/few-observable experimental protocols but
    /// the resulting quantity must be interpreted as an estimator rather than
    /// silently treated as complete tomography.
    PauliSubset,

    /// Purity supplied directly by a two-copy or equivalent measurement
    /// protocol.
    DirectPurity,
}

impl Default for PurityMeasurementMode {
    fn default() -> Self {
        Self::CompletePauli
    }
}

// =============================================================================
// Configuration
// =============================================================================

/// Production configuration for Purity RB.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PurityRbConfig {
    /// Number of logical/physical qubits participating in the experiment.
    pub qubit_count: usize,

    /// Random Clifford sequence lengths.
    pub sequence_lengths: Vec<usize>,

    /// Number of independently randomized sequences per sequence length.
    pub sequences_per_length: usize,

    /// Number of shots per sequence.
    pub shots_per_sequence: usize,

    /// How the decay exponent is defined.
    pub decay_exponent_convention: DecayExponentConvention,

    /// Purity measurement mode.
    pub measurement_mode: PurityMeasurementMode,

    /// Regression configuration.
    pub regression: RegressionConfig,

    /// Protocol-level maximum sequence length.
    pub max_sequence_length: usize,

    /// Protocol-level maximum number of sequences per length.
    pub max_sequences_per_length: usize,

    /// Protocol-level maximum shots per sequence.
    pub max_shots_per_sequence: usize,
}

impl Default for PurityRbConfig {
    fn default() -> Self {
        Self {
            qubit_count: 1,
            sequence_lengths: vec![1, 5, 10, 20, 40, 80],
            sequences_per_length: 10,
            shots_per_sequence: 1_000,
            decay_exponent_convention:
                DecayExponentConvention::default(),
            measurement_mode:
                PurityMeasurementMode::default(),
            regression: RegressionConfig::production(),
            max_sequence_length:
                DEFAULT_MAX_SEQUENCE_LENGTH,
            max_sequences_per_length:
                DEFAULT_MAX_SEQUENCES_PER_LENGTH,
            max_shots_per_sequence:
                DEFAULT_MAX_SHOTS_PER_SEQUENCE,
        }
    }
}

impl PurityRbConfig {
    /// Creates a production one-qubit configuration.
    pub fn production() -> Self {
        Self::default()
    }

    /// Validates the configuration.
    pub fn validate(&self) -> PurityRbResult<()> {
        if self.qubit_count == 0 {
            return Err(PurityRbError::InvalidQubitCount);
        }

        checked_dimension(self.qubit_count)?;

        if self.sequence_lengths.len()
            < DEFAULT_MIN_SEQUENCE_LENGTHS
        {
            return Err(
                PurityRbError::InsufficientSequenceLengths {
                    actual: self.sequence_lengths.len(),
                    minimum: DEFAULT_MIN_SEQUENCE_LENGTHS,
                },
            );
        }

        if self.sequences_per_length == 0 {
            return Err(PurityRbError::InvalidSequenceCount);
        }

        if self.sequences_per_length
            > self.max_sequences_per_length
        {
            return Err(PurityRbError::TooManySequences {
                requested: self.sequences_per_length,
                maximum: self.max_sequences_per_length,
            });
        }

        if self.max_sequences_per_length == 0 {
            return Err(PurityRbError::InvalidSequenceCount);
        }

        if self.shots_per_sequence == 0 {
            return Err(PurityRbError::InvalidShotCount);
        }

        if self.shots_per_sequence
            > self.max_shots_per_sequence
        {
            return Err(PurityRbError::TooManyShots {
                requested: self.shots_per_sequence,
                maximum: self.max_shots_per_sequence,
            });
        }

        if self.max_sequence_length == 0 {
            return Err(PurityRbError::SequenceLengthTooLarge {
                length: 0,
                maximum: self.max_sequence_length,
            });
        }

        let mut previous = None;

        for &length in &self.sequence_lengths {
            if length == 0 {
                return Err(PurityRbError::InvalidSequenceLength {
                    length,
                });
            }

            if length > self.max_sequence_length {
                return Err(
                    PurityRbError::SequenceLengthTooLarge {
                        length,
                        maximum: self.max_sequence_length,
                    },
                );
            }

            if let Some(previous_length) = previous {
                if length <= previous_length {
                    if length == previous_length {
                        return Err(
                            PurityRbError::DuplicateSequenceLength {
                                length,
                            },
                        );
                    }

                    return Err(
                        PurityRbError::InvalidSequenceLength {
                            length,
                        },
                    );
                }
            }

            previous = Some(length);
        }

        let sequence_count =
            self.sequence_lengths.len()
                .checked_mul(self.sequences_per_length)
                .ok_or(PurityRbError::WorkloadOverflow)?;

        sequence_count
            .checked_mul(self.shots_per_sequence)
            .ok_or(PurityRbError::WorkloadOverflow)?;

        self.regression
            .validate()
            .map_err(PurityRbError::Regression)?;

        Ok(())
    }

    /// Returns the Hilbert-space dimension.
    pub fn dimension(&self) -> PurityRbResult<usize> {
        checked_dimension(self.qubit_count)
    }

    /// Returns the total number of random sequences.
    pub fn total_sequences(&self) -> PurityRbResult<usize> {
        self.sequence_lengths
            .len()
            .checked_mul(self.sequences_per_length)
            .ok_or(PurityRbError::WorkloadOverflow)
    }

    /// Returns the total number of requested shots.
    pub fn total_shots(&self) -> PurityRbResult<usize> {
        self.total_sequences()?
            .checked_mul(self.shots_per_sequence)
            .ok_or(PurityRbError::WorkloadOverflow)
    }

    /// Returns the exponent used by the regression model for a length.
    pub fn regression_exponent(
        &self,
        sequence_length: usize,
    ) -> PurityRbResult<f64> {
        self.decay_exponent_convention
            .exponent(sequence_length)
    }
}

// =============================================================================
// Experiment planning
// =============================================================================

/// One planned Purity RB sequence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PurityRbSequencePlan {
    /// Sequence length in random Clifford elements.
    pub sequence_length: usize,

    /// Zero-based sequence index within this length.
    pub sequence_index: usize,

    /// Stable sequence identifier.
    pub sequence_id: String,
}

impl PurityRbSequencePlan {
    /// Creates a deterministic sequence plan identifier.
    ///
    /// The identifier intentionally contains only benchmark-local semantic
    /// fields. A higher-level provenance layer should combine it with the
    /// benchmark seed and generator version.
    pub fn new(
        sequence_length: usize,
        sequence_index: usize,
    ) -> PurityRbResult<Self> {
        if sequence_length == 0 {
            return Err(PurityRbError::InvalidSequenceLength {
                length: sequence_length,
            });
        }

        let sequence_id =
            format!("purity-rb-m{sequence_length}-s{sequence_index}");

        Ok(Self {
            sequence_length,
            sequence_index,
            sequence_id,
        })
    }
}

/// Creates the complete deterministic experiment plan.
///
/// This function does not generate Clifford operations. The Clifford
/// generator remains the owner of that responsibility.
pub fn build_experiment_plan(
    config: &PurityRbConfig,
) -> PurityRbResult<Vec<PurityRbSequencePlan>> {
    config.validate()?;

    let total = config.total_sequences()?;

    let mut plan = Vec::with_capacity(total);

    for &length in &config.sequence_lengths {
        for sequence_index in 0..config.sequences_per_length {
            plan.push(PurityRbSequencePlan::new(
                length,
                sequence_index,
            )?);
        }
    }

    Ok(plan)
}

// =============================================================================
// Pauli purity estimator
// =============================================================================

/// Result of estimating shifted purity from Pauli expectations.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PauliPurityEstimate {
    /// Physical purity `Tr(ρ²)`.
    pub raw_purity: f64,

    /// Shifted/normalized purity in `[0, 1]`.
    pub shifted_purity: f64,

    /// Optional delta-method standard error.
    pub standard_error: Option<f64>,

    /// Number of non-identity Pauli observables included.
    pub observable_count: usize,
}

impl PauliPurityEstimate {
    /// Returns whether the estimate has uncertainty information.
    pub const fn has_uncertainty(self) -> bool {
        self.standard_error.is_some()
    }
}

/// Estimates shifted purity from non-identity Pauli expectation values.
///
/// For a complete Pauli basis:
///
///     q = Σ_{P != I} <P>² / (d² - 1)
///
/// where `d = 2^n`.
///
/// The corresponding raw purity is:
///
///     Tr(ρ²) = (1 + Σ <P>²) / d
///
/// This method does not require construction of a density matrix and is
/// therefore suitable for experimental observations.
pub fn shifted_purity_from_pauli_expectations(
    qubit_count: usize,
    expectations: &[f64],
) -> PurityRbResult<PauliPurityEstimate> {
    if expectations.is_empty() {
        return Err(PurityRbError::EmptyPauliExpectations);
    }

    let dimension = checked_dimension(qubit_count)?;

    let mut sum_squares = 0.0;

    for (index, &expectation) in expectations.iter().enumerate()
    {
        validate_pauli_expectation(index, expectation)?;

        sum_squares += expectation * expectation;

        if !sum_squares.is_finite() {
            return Err(PurityRbError::NumericalFailure {
                operation: "Pauli expectation square accumulation",
            });
        }
    }

    let pauli_dimension = dimension
        .checked_mul(dimension)
        .and_then(|value| value.checked_sub(1))
        .ok_or(PurityRbError::WorkloadOverflow)?;

    let raw_purity =
        (1.0 + sum_squares) / dimension as f64;

    let shifted_purity =
        sum_squares / pauli_dimension as f64;

    validate_probability_like(
        raw_purity,
        PurityValueKind::Raw,
        1.0 / dimension as f64,
    )?;

    validate_probability_like(
        shifted_purity,
        PurityValueKind::Shifted,
        0.0,
    )?;

    Ok(PauliPurityEstimate {
        raw_purity,
        shifted_purity,
        standard_error: None,
        observable_count: expectations.len(),
    })
}

/// Estimates shifted purity and its delta-method uncertainty from Pauli
/// expectation values.
///
/// The variance model assumes the individual expectation estimates are
/// independent. Experimental systems with correlated measurement errors should
/// provide a sequence-level uncertainty obtained from bootstrap or another
/// covariance-aware analysis instead.
pub fn shifted_purity_from_pauli_expectations_with_uncertainty(
    qubit_count: usize,
    expectations: &[f64],
    variances: &[f64],
) -> PurityRbResult<PauliPurityEstimate> {
    if expectations.is_empty() {
        return Err(PurityRbError::EmptyPauliExpectations);
    }

    if expectations.len() != variances.len() {
        return Err(PurityRbError::NumericalFailure {
            operation: "Pauli expectation/variance length validation",
        });
    }

    let dimension = checked_dimension(qubit_count)?;

    let mut sum_squares = 0.0;
    let mut variance_sum = 0.0;

    for index in 0..expectations.len() {
        let expectation = expectations[index];
        let variance = variances[index];

        validate_pauli_expectation(index, expectation)?;

        if !variance.is_finite() || variance < 0.0 {
            return Err(PurityRbError::InvalidPauliVariance {
                index,
                value: variance,
            });
        }

        sum_squares += expectation * expectation;

        // Delta method:
        //
        // q = Σ e_i² / N
        //
        // dq/de_i = 2 e_i / N
        //
        // Var(q) ≈ Σ (2 e_i / N)² Var(e_i)
        let pauli_dimension =
            (dimension as f64)
                * (dimension as f64)
                - 1.0;

        let derivative =
            2.0 * expectation / pauli_dimension;

        variance_sum += derivative * derivative * variance;

        if !sum_squares.is_finite()
            || !variance_sum.is_finite()
        {
            return Err(PurityRbError::NumericalFailure {
                operation: "Pauli purity uncertainty accumulation",
            });
        }
    }

    let raw_purity =
        (1.0 + sum_squares) / dimension as f64;

    let pauli_dimension =
        (dimension as f64)
            * (dimension as f64)
            - 1.0;

    let shifted_purity =
        sum_squares / pauli_dimension;

    validate_probability_like(
        raw_purity,
        PurityValueKind::Raw,
        1.0 / dimension as f64,
    )?;

    validate_probability_like(
        shifted_purity,
        PurityValueKind::Shifted,
        0.0,
    )?;

    let standard_error = variance_sum.sqrt();

    if !standard_error.is_finite() {
        return Err(PurityRbError::NumericalFailure {
            operation: "Pauli purity standard-error calculation",
        });
    }

    Ok(PauliPurityEstimate {
        raw_purity,
        shifted_purity,
        standard_error: Some(standard_error),
        observable_count: expectations.len(),
    })
}

/// Converts raw purity into shifted purity.
///
/// The transformation is:
///
///     q = (d * P - 1) / (d - 1)
///
/// where `P = Tr(ρ²)`.
pub fn shifted_purity_from_raw_purity(
    qubit_count: usize,
    raw_purity: f64,
) -> PurityRbResult<f64> {
    let dimension = checked_dimension(qubit_count)?;

    validate_probability_like(
        raw_purity,
        PurityValueKind::Raw,
        1.0 / dimension as f64,
    )?;

    if dimension <= 1 {
        return Err(PurityRbError::DimensionOverflow {
            qubits: qubit_count,
        });
    }

    let shifted =
        (dimension as f64 * raw_purity - 1.0)
            / (dimension as f64 - 1.0);

    validate_probability_like(
        shifted,
        PurityValueKind::Shifted,
        0.0,
    )?;

    Ok(shifted)
}

/// Converts shifted purity into raw purity.
///
///     P = (1 + (d - 1) q) / d
pub fn raw_purity_from_shifted_purity(
    qubit_count: usize,
    shifted_purity: f64,
) -> PurityRbResult<f64> {
    let dimension = checked_dimension(qubit_count)?;

    validate_probability_like(
        shifted_purity,
        PurityValueKind::Shifted,
        0.0,
    )?;

    let raw =
        (1.0
            + (dimension as f64 - 1.0)
                * shifted_purity)
            / dimension as f64;

    validate_probability_like(
        raw,
        PurityValueKind::Raw,
        1.0 / dimension as f64,
    )?;

    Ok(raw)
}

// =============================================================================
// Sequence observations
// =============================================================================

/// Purity result for one randomized Clifford sequence.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PuritySequenceObservation {
    /// Stable sequence identifier.
    pub sequence_id: String,

    /// Number of random Clifford elements.
    pub sequence_length: usize,

    /// Estimated shifted purity.
    pub shifted_purity: f64,

    /// Optional standard error for the sequence purity estimate.
    pub standard_error: Option<f64>,

    /// Number of shots used by the sequence.
    pub shots: usize,

    /// Number of Pauli observables used to estimate purity.
    pub observable_count: Option<usize>,
}

impl PuritySequenceObservation {
    /// Creates an observation with no externally supplied uncertainty.
    pub fn new(
        sequence_id: impl Into<String>,
        sequence_length: usize,
        shifted_purity: f64,
        shots: usize,
    ) -> PurityRbResult<Self> {
        let sequence_id = sequence_id.into();

        if sequence_id.trim().is_empty() {
            return Err(PurityRbError::InvalidSequenceIdentity);
        }

        if sequence_length == 0 {
            return Err(PurityRbError::InvalidSequenceLength {
                length: sequence_length,
            });
        }

        if shots == 0 {
            return Err(PurityRbError::InvalidShotCount);
        }

        validate_probability_like(
            shifted_purity,
            PurityValueKind::Shifted,
            0.0,
        )?;

        Ok(Self {
            sequence_id,
            sequence_length,
            shifted_purity,
            standard_error: None,
            shots,
            observable_count: None,
        })
    }

    /// Adds an externally calculated standard error.
    pub fn with_standard_error(
        mut self,
        standard_error: f64,
    ) -> PurityRbResult<Self> {
        if !standard_error.is_finite()
            || standard_error < 0.0
        {
            return Err(PurityRbError::NumericalFailure {
                operation: "sequence purity standard-error validation",
            });
        }

        self.standard_error = Some(standard_error);
        Ok(self)
    }

    /// Adds the number of Pauli observables used.
    pub fn with_observable_count(
        mut self,
        count: usize,
    ) -> PurityRbResult<Self> {
        if count == 0 {
            return Err(PurityRbError::EmptyPauliExpectations);
        }

        self.observable_count = Some(count);
        Ok(self)
    }
}

// =============================================================================
// Per-length aggregation
// =============================================================================

/// Aggregated purity result for one sequence length.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PurityLengthObservation {
    /// Sequence length in random Clifford gates.
    pub sequence_length: usize,

    /// Regression exponent.
    pub decay_exponent: f64,

    /// Number of randomized sequences contributing to the point.
    pub sequence_count: usize,

    /// Mean shifted purity.
    pub mean_shifted_purity: f64,

    /// Standard deviation across randomized sequences.
    pub sample_standard_deviation: Option<f64>,

    /// Standard error of the mean across randomized sequences.
    pub standard_error_of_mean: Option<f64>,

    /// Whether the point contains at least the recommended number of
    /// randomized sequences.
    pub statistically_well_sampled: bool,
}

/// Aggregates individual sequence observations at one sequence length.
pub fn aggregate_length(
    config: &PurityRbConfig,
    observations: &[PuritySequenceObservation],
) -> PurityRbResult<PurityLengthObservation> {
    if observations.is_empty() {
        return Err(PurityRbError::EmptySequenceObservation);
    }

    let sequence_length =
        observations[0].sequence_length;

    let expected_exponent =
        config.regression_exponent(sequence_length)?;

    let mut sum = 0.0;

    for observation in observations {
        if observation.sequence_length != sequence_length {
            return Err(PurityRbError::NumericalFailure {
                operation: "Purity RB length grouping validation",
            });
        }

        validate_probability_like(
            observation.shifted_purity,
            PurityValueKind::Shifted,
            0.0,
        )?;

        sum += observation.shifted_purity;

        if !sum.is_finite() {
            return Err(PurityRbError::NumericalFailure {
                operation: "Purity RB length mean accumulation",
            });
        }
    }

    let count = observations.len();

    let mean = sum / count as f64;

    if !mean.is_finite() {
        return Err(PurityRbError::NumericalFailure {
            operation: "Purity RB length mean",
        });
    }

    let standard_deviation =
        if count >= 2 {
            let mut squared_sum = 0.0;

            for observation in observations {
                let difference =
                    observation.shifted_purity - mean;

                squared_sum += difference * difference;
            }

            let variance =
                squared_sum / (count - 1) as f64;

            let deviation = variance.sqrt();

            if !deviation.is_finite() {
                return Err(PurityRbError::NumericalFailure {
                    operation:
                        "Purity RB length standard deviation",
                });
            }

            Some(deviation)
        } else {
            None
        };

    let standard_error_of_mean =
        standard_deviation.map(|value| {
            value / (count as f64).sqrt()
        });

    Ok(PurityLengthObservation {
        sequence_length,
        decay_exponent: expected_exponent,
        sequence_count: count,
        mean_shifted_purity: mean,
        sample_standard_deviation: standard_deviation,
        standard_error_of_mean,
        statistically_well_sampled:
            count >= DEFAULT_MIN_SEQUENCES_PER_LENGTH,
    })
}

// =============================================================================
// Full analysis input
// =============================================================================

/// Complete Purity RB data set for one experiment.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PurityRbData {
    /// Configuration used for the experiment.
    pub config: PurityRbConfig,

    /// Per-sequence observations.
    pub sequences: Vec<PuritySequenceObservation>,
}

impl PurityRbData {
    /// Creates a validated data set.
    pub fn new(
        config: PurityRbConfig,
        sequences: Vec<PuritySequenceObservation>,
    ) -> PurityRbResult<Self> {
        config.validate()?;

        if sequences.is_empty() {
            return Err(PurityRbError::EmptySequenceObservation);
        }

        for sequence in &sequences {
            if sequence.shots == 0 {
                return Err(PurityRbError::InvalidShotCount);
            }

            validate_probability_like(
                sequence.shifted_purity,
                PurityValueKind::Shifted,
                0.0,
            )?;
        }

        Ok(Self {
            config,
            sequences,
        })
    }

    /// Groups all sequence observations by configured sequence length.
    pub fn aggregate(
        &self,
    ) -> PurityRbResult<Vec<PurityLengthObservation>> {
        let mut aggregated = Vec::new();

        for &length in &self.config.sequence_lengths {
            let group: Vec<PuritySequenceObservation> =
                self.sequences
                    .iter()
                    .filter(|observation| {
                        observation.sequence_length == length
                    })
                    .cloned()
                    .collect();

            if group.is_empty() {
                return Err(
                    PurityRbError::EmptySequenceObservation,
                );
            }

            aggregated.push(aggregate_length(
                &self.config,
                &group,
            )?);
        }

        Ok(aggregated)
    }
}

// =============================================================================
// Analysis result
// =============================================================================

/// Scientific interpretation warning.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PurityRbWarning {
    /// Fewer than the recommended number of random sequences were supplied
    /// for at least one sequence length.
    LowSequenceReplication,

    /// Fewer than the recommended number of shots were used.
    LowShotCount,

    /// Regression landed on a physical/search boundary.
    BoundaryFit,

    /// Regression did not report numerical convergence.
    FitNotConverged,

    /// Regression covariance was unavailable.
    UncertaintyUnavailable,

    /// Regression conditioning was poor.
    PoorConditioning,

    /// Fit quality was insufficient for strong physical interpretation.
    PoorFitQuality,

    /// One or more observations were reconstructed from a subset of Pauli
    /// observables rather than a complete Pauli basis.
    PauliSubsetUsed,

    /// The protocol is being interpreted outside its usual small/few-qubit
    /// experimental regime.
    LargeQubitCount,

    /// Unitarity is a noise-coherence quantity and must not be treated as a
    /// standalone device-quality score.
    UnitarityIsNotFidelity,
}

/// Canonical Purity RB result.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PurityRbResult {
    /// Stable benchmark identifier.
    pub benchmark_id: String,

    /// Short benchmark identifier.
    pub protocol_id: String,

    /// Result schema version.
    pub result_schema_version: u32,

    /// Protocol implementation version.
    pub protocol_version: u32,

    /// Purity-estimator algorithm identifier.
    pub purity_estimator_version: String,

    /// Analysis algorithm identifier.
    pub analysis_version: String,

    /// Number of qubits.
    pub qubit_count: usize,

    /// Hilbert-space dimension.
    pub dimension: usize,

    /// Measurement mode.
    pub measurement_mode: PurityMeasurementMode,

    /// Decay convention.
    pub decay_exponent_convention:
        DecayExponentConvention,

    /// Per-length observations.
    pub length_observations:
        Vec<PurityLengthObservation>,

    /// Exponential regression.
    pub regression: RegressionFit,

    /// Extracted unitarity.
    pub unitarity: f64,

    /// Standard error of unitarity, when available.
    pub unitarity_standard_error: Option<f64>,

    /// Lower confidence bound of unitarity, when available.
    pub unitarity_lower: Option<f64>,

    /// Upper confidence bound of unitarity, when available.
    pub unitarity_upper: Option<f64>,

    /// Purity-RB error-per-Clifford transformation.
    ///
    /// For a `d`-dimensional system:
    ///
    ///     EPC_pur = (d - 1) / d * (1 - sqrt(u))
    ///
    /// This quantity should be interpreted as the purity-RB EPC-style
    /// transformation, not as a universal physical gate infidelity.
    pub purity_error_per_clifford: Option<f64>,

    /// Number of randomized sequences.
    pub total_sequences: usize,

    /// Number of shots.
    pub total_shots: usize,

    /// Scientific and numerical warnings.
    pub warnings: Vec<PurityRbWarning>,
}

impl PurityRbResult {
    /// Returns whether the fit is numerically usable.
    pub fn is_valid(&self) -> bool {
        self.unitarity.is_finite()
            && self.unitarity >= 0.0
            && self.unitarity <= 1.0
            && self.regression.diagnostics.is_converged()
    }

    /// Returns the fitted unitarity.
    pub const fn unitarity(&self) -> f64 {
        self.unitarity
    }

    /// Returns the unitarity deficit.
    ///
    /// This is:
    ///
    ///     1 - u
    ///
    /// It is a coherence/noise-dynamics indicator and is NOT an average
    /// gate infidelity.
    pub fn unitarity_deficit(&self) -> f64 {
        1.0 - self.unitarity
    }

    /// Returns the square-root unitarity.
    pub fn sqrt_unitarity(&self) -> f64 {
        self.unitarity.sqrt()
    }
}

// =============================================================================
// Main protocol analysis
// =============================================================================

/// Production Purity RB analyzer.
#[derive(Debug, Clone)]
pub struct PurityRbAnalyzer {
    config: PurityRbConfig,
    regression_engine: RegressionEngine,
}

impl PurityRbAnalyzer {
    /// Creates a production analyzer.
    pub fn new(
        config: PurityRbConfig,
    ) -> PurityRbResult<Self> {
        config.validate()?;

        let regression_engine =
            RegressionEngine::new(config.regression)
                .map_err(PurityRbError::Regression)?;

        Ok(Self {
            config,
            regression_engine,
        })
    }

    /// Returns the immutable analyzer configuration.
    pub fn config(&self) -> &PurityRbConfig {
        &self.config
    }

    /// Returns the configured regression engine.
    pub fn regression_engine(&self) -> &RegressionEngine {
        &self.regression_engine
    }

    /// Analyzes complete Purity RB observations.
    pub fn analyze(
        &self,
        data: &PurityRbData,
    ) -> PurityRbResult<PurityRbResult> {
        if data.config != self.config {
            return Err(
                PurityRbError::InvalidRegressionConfiguration,
            );
        }

        let length_observations =
            data.aggregate()?;

        let mut regression_observations =
            Vec::with_capacity(length_observations.len());

        for observation in &length_observations {
            let weight =
                observation
                    .standard_error_of_mean
                    .filter(|error| {
                        error.is_finite() && *error > 0.0
                    })
                    .map(|error| 1.0 / (error * error));

            regression_observations.push(
                RegressionObservation {
                    x: observation.decay_exponent,
                    y: observation.mean_shifted_purity,
                    weight,
                },
            );
        }

        let regression =
            self.regression_engine
                .fit(&regression_observations)?;

        let unitarity =
            regression.decay_parameter.value;

        if !unitarity.is_finite()
            || unitarity < -UNIT_INTERVAL_EPSILON
            || unitarity > 1.0 + UNIT_INTERVAL_EPSILON
        {
            return Err(
                PurityRbError::InvalidFittedUnitarity {
                    value: unitarity,
                },
            );
        }

        let unitarity =
            clamp_unit_interval(unitarity);

        let purity_error_per_clifford =
            purity_error_per_clifford(
                self.config.qubit_count,
                unitarity,
            )?;

        let mut warnings = Vec::new();

        if length_observations.iter().any(
            |observation| {
                !observation.statistically_well_sampled
            },
        ) {
            warnings.push(
                PurityRbWarning::LowSequenceReplication,
            );
        }

        if data.sequences.iter().any(|sequence| {
            sequence.shots < DEFAULT_MIN_SHOTS_PER_SEQUENCE
        }) {
            warnings.push(PurityRbWarning::LowShotCount);
        }

        if regression.is_boundary_solution() {
            warnings.push(PurityRbWarning::BoundaryFit);
        }

        if !regression.diagnostics.is_converged() {
            warnings.push(PurityRbWarning::FitNotConverged);
        }

        if !regression
            .diagnostics
            .covariance_available
        {
            warnings.push(
                PurityRbWarning::UncertaintyUnavailable,
            );
        }

        if regression
            .diagnostics
            .conditioning_reciprocal
            .map(|value| value < 1.0e-10)
            .unwrap_or(true)
        {
            warnings.push(
                PurityRbWarning::PoorConditioning,
            );
        }

        if regression
            .diagnostics
            .r_squared
            .map(|value| value < 0.90)
            .unwrap_or(true)
        {
            warnings.push(
                PurityRbWarning::PoorFitQuality,
            );
        }

        if self.config.measurement_mode
            == PurityMeasurementMode::PauliSubset
        {
            warnings.push(
                PurityRbWarning::PauliSubsetUsed,
            );
        }

        if self.config.qubit_count > 2 {
            warnings.push(
                PurityRbWarning::LargeQubitCount,
            );
        }

        warnings.push(
            PurityRbWarning::UnitarityIsNotFidelity,
        );

        let total_sequences =
            data.sequences.len();

        let total_shots = data
            .sequences
            .iter()
            .try_fold(0usize, |acc, observation| {
                acc.checked_add(observation.shots)
                    .ok_or(PurityRbError::WorkloadOverflow)
            })?;

        Ok(PurityRbResult {
            benchmark_id:
                PURITY_RB_BENCHMARK_ID.to_owned(),
            protocol_id:
                PURITY_RB_SHORT_ID.to_owned(),
            result_schema_version:
                PURITY_RB_RESULT_SCHEMA_VERSION,
            protocol_version:
                PURITY_RB_PROTOCOL_VERSION,
            purity_estimator_version:
                PURITY_ESTIMATOR_VERSION.to_owned(),
            analysis_version:
                PURITY_RB_ANALYSIS_VERSION.to_owned(),
            qubit_count:
                self.config.qubit_count,
            dimension:
                self.config.dimension()?,
            measurement_mode:
                self.config.measurement_mode,
            decay_exponent_convention:
                self.config.decay_exponent_convention,
            length_observations,
            unitarity,
            unitarity_standard_error:
                regression
                    .decay_parameter
                    .standard_error,
            unitarity_lower:
                regression.decay_parameter.lower,
            unitarity_upper:
                regression.decay_parameter.upper,
            purity_error_per_clifford,
            regression,
            total_sequences,
            total_shots,
            warnings,
        })
    }
}

// =============================================================================
// Purity-RB EPC transformation
// =============================================================================

/// Calculates the Purity-RB EPC-style transformation.
///
/// For dimension `d`:
///
///     EPC_pur = (d - 1) / d * (1 - sqrt(u))
///
/// This transformation is useful when comparing the purity decay parameter
/// with standard RB under compatible assumptions.
///
/// It must NOT be labelled as an unconditional physical average gate
/// infidelity.
///
/// The relationship follows the fact that for depolarizing noise the purity
/// decay parameter is the square of the ordinary RB decay parameter.
pub fn purity_error_per_clifford(
    qubit_count: usize,
    unitarity: f64,
) -> PurityRbResult<f64> {
    let dimension = checked_dimension(qubit_count)?;

    validate_probability_like(
        unitarity,
        PurityValueKind::Shifted,
        0.0,
    )?;

    let result =
        (dimension as f64 - 1.0)
            / dimension as f64
            * (1.0 - unitarity.sqrt());

    if !result.is_finite()
        || result < -UNIT_INTERVAL_EPSILON
        || result > 1.0 + UNIT_INTERVAL_EPSILON
    {
        return Err(PurityRbError::NumericalFailure {
            operation:
                "Purity RB error-per-Clifford transformation",
        });
    }

    Ok(clamp_unit_interval(result))
}

// =============================================================================
// Helper functions
// =============================================================================

/// Safe `2^qubits` calculation.
fn checked_dimension(qubits: usize) -> PurityRbResult<usize> {
    if qubits == 0 {
        return Err(PurityRbError::InvalidQubitCount);
    }

    let bits =
        usize::BITS as usize;

    if qubits >= bits {
        return Err(PurityRbError::DimensionOverflow {
            qubits,
        });
    }

    1usize
        .checked_shl(qubits as u32)
        .ok_or(PurityRbError::DimensionOverflow {
            qubits,
        })
}

/// Validates a Pauli expectation.
fn validate_pauli_expectation(
    index: usize,
    value: f64,
) -> PurityRbResult<()> {
    if !value.is_finite() {
        return Err(
            PurityRbError::NonFinitePauliExpectation {
                index,
                value,
            },
        );
    }

    if value < -1.0 - UNIT_INTERVAL_EPSILON
        || value > 1.0 + UNIT_INTERVAL_EPSILON
    {
        return Err(
            PurityRbError::InvalidPauliExpectation {
                index,
                value,
            },
        );
    }

    Ok(())
}

/// Categories of purity-like quantities.
#[derive(Debug, Clone, Copy)]
enum PurityValueKind {
    Raw,
    Shifted,
}

/// Validates a purity-like value.
fn validate_probability_like(
    value: f64,
    kind: PurityValueKind,
    lower_bound: f64,
) -> PurityRbResult<()> {
    if !value.is_finite() {
        return Err(PurityRbError::NonFinitePurity {
            value,
        });
    }

    let upper_bound = 1.0;

    let valid =
        value >= lower_bound - UNIT_INTERVAL_EPSILON
            && value
                <= upper_bound + UNIT_INTERVAL_EPSILON;

    if !valid {
        return Err(match kind {
            PurityValueKind::Raw => {
                PurityRbError::InvalidRawPurity {
                    value,
                }
            }

            PurityValueKind::Shifted => {
                PurityRbError::InvalidShiftedPurity {
                    value,
                }
            }
        });
    }

    Ok(())
}

/// Clamps only tiny floating-point excursions around `[0, 1]`.
fn clamp_unit_interval(value: f64) -> f64 {
    if value < 0.0
        && value >= -UNIT_INTERVAL_EPSILON
    {
        0.0
    } else if value > 1.0
        && value <= 1.0 + UNIT_INTERVAL_EPSILON
    {
        1.0
    } else {
        value
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dimension_is_checked_without_overflow() {
        assert_eq!(
            checked_dimension(1).unwrap(),
            2
        );

        assert_eq!(
            checked_dimension(2).unwrap(),
            4
        );

        assert!(checked_dimension(0).is_err());
    }

    #[test]
    fn raw_and_shifted_purity_are_inverse_transformations() {
        let raw = 0.75;

        let shifted =
            shifted_purity_from_raw_purity(
                2,
                raw,
            )
            .unwrap();

        let recovered =
            raw_purity_from_shifted_purity(
                2,
                shifted,
            )
            .unwrap();

        assert!(
            (raw - recovered).abs() < 1.0e-12
        );
    }

    #[test]
    fn pure_single_qubit_state_has_shifted_purity_one() {
        let estimate =
            shifted_purity_from_pauli_expectations(
                1,
                &[0.0, 0.0, 1.0],
            )
            .unwrap();

        assert!(
            (estimate.raw_purity - 1.0).abs()
                < 1.0e-12
        );

        assert!(
            (estimate.shifted_purity - 1.0).abs()
                < 1.0e-12
        );
    }

    #[test]
    fn maximally_mixed_state_has_zero_shifted_purity() {
        let estimate =
            shifted_purity_from_pauli_expectations(
                1,
                &[0.0, 0.0, 0.0],
            )
            .unwrap();

        assert!(
            estimate.raw_purity
                .abs()
                < 1.0e-12
                + 0.5
        );

        assert!(
            estimate.shifted_purity.abs()
                < 1.0e-12
        );
    }

    #[test]
    fn two_qubit_maximally_mixed_state_is_valid() {
        let estimate =
            shifted_purity_from_pauli_expectations(
                2,
                &[0.0],
            )
            .unwrap();

        assert!(
            estimate.shifted_purity.abs()
                < 1.0e-12
        );
    }

    #[test]
    fn invalid_pauli_expectation_is_rejected() {
        let result =
            shifted_purity_from_pauli_expectations(
                1,
                &[1.1],
            );

        assert!(matches!(
            result,
            Err(
                PurityRbError::InvalidPauliExpectation {
                    ..
                }
            )
        ));
    }

    #[test]
    fn plan_is_deterministic() {
        let config = PurityRbConfig {
            sequence_lengths:
                vec![1, 5, 10, 20],
            sequences_per_length: 2,
            ..PurityRbConfig::default()
        };

        let first =
            build_experiment_plan(&config)
                .unwrap();

        let second =
            build_experiment_plan(&config)
                .unwrap();

        assert_eq!(first, second);
        assert_eq!(first.len(), 8);
        assert_eq!(
            first[0].sequence_id,
            "purity-rb-m1-s0"
        );
    }

    #[test]
    fn sequence_length_minus_one_is_correct() {
        let convention =
            DecayExponentConvention::
                SequenceLengthMinusOne;

        assert_eq!(
            convention.exponent(1).unwrap(),
            0.0
        );

        assert_eq!(
            convention.exponent(10).unwrap(),
            9.0
        );
    }

    #[test]
    fn sequence_length_convention_is_correct() {
        let convention =
            DecayExponentConvention::SequenceLength;

        assert_eq!(
            convention.exponent(10).unwrap(),
            10.0
        );
    }

    #[test]
    fn configuration_rejects_duplicate_lengths() {
        let config = PurityRbConfig {
            sequence_lengths:
                vec![1, 5, 5, 10],
            ..PurityRbConfig::default()
        };

        assert!(matches!(
            config.validate(),
            Err(
                PurityRbError::DuplicateSequenceLength {
                    length: 5
                }
            )
        ));
    }

    #[test]
    fn configuration_rejects_unsorted_lengths() {
        let config = PurityRbConfig {
            sequence_lengths:
                vec![1, 10, 5, 20],
            ..PurityRbConfig::default()
        };

        assert!(config.validate().is_err());
    }

    #[test]
    fn configuration_rejects_zero_shots() {
        let config = PurityRbConfig {
            shots_per_sequence: 0,
            ..PurityRbConfig::default()
        };

        assert!(matches!(
            config.validate(),
            Err(PurityRbError::InvalidShotCount)
        ));
    }

    #[test]
    fn configuration_rejects_zero_sequences() {
        let config = PurityRbConfig {
            sequences_per_length: 0,
            ..PurityRbConfig::default()
        };

        assert!(matches!(
            config.validate(),
            Err(
                PurityRbError::InvalidSequenceCount
            )
        ));
    }

    #[test]
    fn epc_is_zero_for_unitarity_one() {
        let epc =
            purity_error_per_clifford(
                1,
                1.0,
            )
            .unwrap();

        assert!(epc.abs() < 1.0e-12);
    }

    #[test]
    fn epc_is_one_half_for_zero_unitarity_on_one_qubit() {
        let epc =
            purity_error_per_clifford(
                1,
                0.0,
            )
            .unwrap();

        assert!(
            (epc - 0.5).abs()
                < 1.0e-12
        );
    }

    #[test]
    fn direct_sequence_observation_is_validated() {
        let observation =
            PuritySequenceObservation::new(
                "sequence-0",
                10,
                0.8,
                1_000,
            )
            .unwrap();

        assert_eq!(
            observation.sequence_length,
            10
        );

        assert!(
            (observation.shifted_purity - 0.8)
                .abs()
                < 1.0e-12
        );
    }

    #[test]
    fn empty_sequence_identity_is_rejected() {
        let result =
            PuritySequenceObservation::new(
                "",
                10,
                0.8,
                1_000,
            );

        assert!(matches!(
            result,
            Err(
                PurityRbError::InvalidSequenceIdentity
            )
        ));
    }

    #[test]
    fn aggregation_calculates_mean() {
        let config = PurityRbConfig {
            sequence_lengths:
                vec![1, 5, 10, 20],
            ..PurityRbConfig::default()
        };

        let observations = vec![
            PuritySequenceObservation::new(
                "a",
                5,
                0.8,
                1_000,
            )
            .unwrap(),
            PuritySequenceObservation::new(
                "b",
                5,
                0.6,
                1_000,
            )
            .unwrap(),
        ];

        let aggregate =
            aggregate_length(
                &config,
                &observations,
            )
            .unwrap();

        assert!(
            (aggregate.mean_shifted_purity - 0.7)
                .abs()
                < 1.0e-12
        );

        assert_eq!(
            aggregate.sequence_count,
            2
        );
    }

    #[test]
    fn pauli_uncertainty_is_nonnegative() {
        let estimate =
            shifted_purity_from_pauli_expectations_with_uncertainty(
                1,
                &[0.2, 0.3, 0.4],
                &[0.01, 0.01, 0.01],
            )
            .unwrap();

        assert!(
            estimate.standard_error.unwrap()
                >= 0.0
        );
    }

    #[test]
    fn default_configuration_is_valid() {
        PurityRbConfig::production()
            .validate()
            .unwrap();
    }

    #[test]
    fn result_identity_constants_are_stable() {
        assert_eq!(
            PURITY_RB_BENCHMARK_ID,
            "purity_randomized_benchmarking"
        );

        assert_eq!(
            PURITY_RB_SHORT_ID,
            "purity_rb"
        );

        assert_eq!(
            PURITY_RB_FAMILY_ID,
            "unitarity_randomized_benchmarking"
        );
    }
}