//! Zamani Quantum Benchmarking — Randomized Benchmarking
//!
//! Production randomized-benchmarking protocol implementation.
//!
//! # Architectural responsibility
//!
//! This module owns the *randomized benchmarking protocol*:
//!
//! - RB configuration;
//! - validation of RB-specific experimental parameters;
//! - construction of random Clifford sequences;
//! - construction of recovery/inversion Clifford operations;
//! - normalization of survival observations;
//! - aggregation of sequence observations by length;
//! - exponential-decay analysis;
//! - error-per-Clifford calculation;
//! - fit diagnostics;
//! - protocol-level warnings and scientific assumptions;
//! - deterministic/reproducible sequence generation.
//!
//! This module does NOT own:
//!
//! - Quantum IR semantics;
//! - physical gate implementation;
//! - hardware communication;
//! - backend selection;
//! - routing;
//! - scheduling;
//! - calibration;
//! - generic benchmark configuration;
//! - generic benchmark execution;
//! - generic result serialization;
//! - generic reporting;
//! - generic statistical infrastructure.
//!
//! Those responsibilities belong to their owning layers.
//!
//! # Dependency direction
//!
//! ```text
//! quantum::benchmarking::generators::clifford
//!                  │
//!                  ▼
//!       randomized_benchmarking
//!                  │
//!          ┌───────┴────────┐
//!          ▼                ▼
//!    execution layer     statistics layer
//!          │                │
//!          ▼                ▼
//!       Quantum IR      regression/CI
//!          │
//!          ▼
//! routing / scheduling / hardware / runtime
//! ```
//!
//! The dependency direction must never be reversed.
//!
//! In particular:
//!
//! ```text
//! quantum::ir
//!      X
//!      │
//!      └──> randomized_benchmarking
//! ```
//!
//! is allowed, while:
//!
//! ```text
//! randomized_benchmarking
//!      X
//!      │
//!      └──> quantum::ir
//! ```
//!
//! is intentionally NOT required by this protocol layer.
//!
//! The generated Clifford sequence is represented by the existing
//! `generators::clifford::CliffordPrimitiveSequence` abstraction. A later
//! integration layer can lower that sequence into the canonical Quantum IR.
//!
//! # Scientific model
//!
//! Standard randomized benchmarking measures survival probability as a
//! function of sequence length using a model of the form:
//!
//! ```text
//! P(m) = A * p^m + B
//! ```
//!
//! where:
//!
//! - `m` is the number of random Clifford operations;
//! - `A` is the SPAM-sensitive amplitude;
//! - `B` is the asymptotic offset;
//! - `p` is the RB decay parameter.
//!
//! For a d-dimensional system, the usual depolarizing-model relation is:
//!
//! ```text
//! r = (d - 1) / d * (1 - p)
//! ```
//!
//! where `r` is the error-per-Clifford quantity associated with the RB decay.
//!
//! For one qubit:
//!
//! ```text
//! d = 2
//! r = (1 - p) / 2
//! ```
//!
//! IMPORTANT:
//!
//! `r` is an RB decay-derived error metric. It must not automatically be
//! reported as a universal physical average gate infidelity. The relationship
//! depends on the protocol and noise assumptions.
//!
//! # Sequence semantics
//!
//! A standard sequence contains:
//!
//! ```text
//! C1, C2, ..., Cm, C_recovery
//! ```
//!
//! with:
//!
//! ```text
//! C_recovery = (Cm ... C2 C1)^-1
//! ```
//!
//! so that the ideal logical operation of the complete sequence is identity.
//!
//! The existing Clifford generator provides:
//!
//! - exact C1 sampling;
//! - Clifford composition;
//! - Clifford inversion;
//! - canonical primitive decomposition.
//!
//! This module uses those operations rather than reimplementing Clifford
//! group mathematics.
//!
//! # Current supported protocol
//!
//! The production implementation currently supports:
//!
//! - standard single-qubit Clifford RB;
//! - exact uniform sampling from C1;
//! - explicit deterministic seeds;
//! - multiple sequence lengths;
//! - multiple random sequences per length;
//! - configurable shots;
//! - pooled survival-probability observations;
//! - bounded exponential fitting;
//! - EPC calculation;
//! - fit diagnostics;
//! - reproducibility fingerprints at the sequence-definition level;
//! - offline analysis of captured observations.
//!
//! It deliberately does NOT claim full standard n-qubit Clifford RB yet.
//!
//! The repository's Clifford generator explicitly distinguishes exact C1
//! sampling from generic multi-qubit Clifford circuits. Treating the latter
//! as uniformly sampled from Cn would produce scientifically invalid RB.
//!
//! A future n-qubit Clifford-group provider can be integrated through a
//! dedicated protocol extension without changing the observation or fitting
//! contracts in this file.
//!
//! # Production safety
//!
//! The protocol rejects:
//!
//! - zero sequence lengths where no identity experiment is intended;
//! - invalid shot counts;
//! - zero sequence counts;
//! - duplicate sequence lengths;
//! - non-finite observations;
//! - successes greater than shots;
//! - resource multiplication overflow;
//! - excessively large generated workloads;
//! - invalid seeds/configuration;
//! - insufficient distinct sequence lengths for the three-parameter model;
//! - invalid fitted decay parameters.
//!
//! No process-global RNG is used.
//!
//! No library diagnostics are printed.
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
//! Existing dependency:
//!
//! - `rand = 0.8`
//!
//! No new dependency is introduced by this module.
//!
//! # Integration contract
//!
//! This file is intentionally usable before the generic benchmarking
//! foundation is completely wired.
//!
//! Future integration is one-way:
//!
//! ```text
//! core::config
//!       │
//!       ▼
//! randomized_benchmarking
//!       │
//!       ├── generators::clifford
//!       ├── statistics::regression
//!       ├── statistics::confidence
//!       ├── execution
//!       └── core::result
//! ```
//!
//! This file therefore exposes stable protocol-level types that can later be
//! adapted into `BenchmarkExperiment`, `BenchmarkObservationSet` and
//! `BenchmarkResult` without changing the RB scientific semantics.
//!
//! # References
//!
//! The implementation follows the standard randomized-benchmarking framework
//! described by Magesan, Gambetta and Emerson and subsequent RB literature.
//!
//! The protocol intentionally exposes assumptions because later work has
//! shown that the RB decay parameter and physical gate-error interpretations
//! require care under coherent, gate-dependent, leakage, non-Markovian and
//! other noise models.

use rand::rngs::StdRng;
use rand::{RngCore, SeedableRng};
use std::error::Error;
use std::fmt;

use super::super::generators::clifford::{
    CliffordError,
    CliffordOperation,
    CliffordPrimitiveSequence,
    CliffordSampler,
    SingleQubitClifford,
};

// =============================================================================
// Public constants
// =============================================================================

/// Stable protocol identifier.
pub const RANDOMIZED_BENCHMARKING_ID: &str =
    "randomized_benchmarking";

/// Protocol schema version.
///
/// Increment this whenever the externally meaningful protocol/result
/// semantics change.
pub const RANDOMIZED_BENCHMARKING_SCHEMA_VERSION: u32 = 1;

/// Current implementation algorithm identifier.
///
/// If the sequence-generation or fitting algorithm changes in a way that can
/// alter scientific results, this identifier must change.
pub const RANDOMIZED_BENCHMARKING_ALGORITHM_ID: &str =
    "zamani.rb.clifford.single_qubit.v1";

/// Minimum number of distinct sequence lengths required by the three-parameter
/// model A*p^m+B.
pub const MIN_SEQUENCE_LENGTHS_FOR_EXPONENTIAL_FIT: usize = 4;

/// Default number of independent random Clifford sequences per length.
pub const DEFAULT_SEQUENCES_PER_LENGTH: usize = 32;

/// Default shots per sequence.
pub const DEFAULT_SHOTS_PER_SEQUENCE: usize = 1_000;

/// Default confidence level used for protocol-level uncertainty diagnostics.
pub const DEFAULT_CONFIDENCE_LEVEL: f64 = 0.95;

/// Default maximum sequence length.
///
/// This is a protocol safety limit, not a physical limit of randomized
/// benchmarking.
pub const DEFAULT_MAX_SEQUENCE_LENGTH: usize = 100_000;

/// Default maximum number of sequence lengths.
pub const DEFAULT_MAX_SEQUENCE_LENGTHS: usize = 256;

/// Default maximum sequences per length.
pub const DEFAULT_MAX_SEQUENCES_PER_LENGTH: usize = 100_000;

/// Default maximum shots per sequence.
pub const DEFAULT_MAX_SHOTS_PER_SEQUENCE: usize = 10_000_000;

/// Default maximum total shots.
///
/// This protects against accidental multiplication of:
///
/// lengths × sequences × shots.
pub const DEFAULT_MAX_TOTAL_SHOTS: u128 = 1_000_000_000;

/// Numerical lower boundary for p.
pub const MIN_DECAY_PARAMETER: f64 = 0.0;

/// Numerical upper boundary for p.
pub const MAX_DECAY_PARAMETER: f64 = 1.0;

/// Default minimum decay-rate search boundary.
///
/// k = -ln(p)
pub const DEFAULT_MIN_DECAY_RATE: f64 = 0.0;

/// Default maximum decay-rate search boundary.
pub const DEFAULT_MAX_DECAY_RATE: f64 = 50.0;

/// Number of profile-search grid points.
pub const DEFAULT_FIT_GRID_POINTS: usize = 128;

/// Maximum golden-section refinement iterations.
pub const DEFAULT_FIT_REFINEMENT_ITERATIONS: usize = 256;

/// Default relative fitting tolerance.
pub const DEFAULT_FIT_RELATIVE_TOLERANCE: f64 = 1.0e-10;

/// Default absolute fitting tolerance.
pub const DEFAULT_FIT_ABSOLUTE_TOLERANCE: f64 = 1.0e-14;

/// Numerical tolerance for unit-interval values.
const UNIT_INTERVAL_EPSILON: f64 = 1.0e-12;

/// Numerical tolerance for checking a fitted parameter at a boundary.
const PARAMETER_BOUNDARY_EPSILON: f64 = 1.0e-10;

// =============================================================================
// Error type
// =============================================================================

/// Errors specific to randomized benchmarking.
#[derive(Debug, Clone, PartialEq)]
pub enum RandomizedBenchmarkingError {
    /// Invalid number of qubits.
    UnsupportedQubitCount {
        /// Requested number of qubits.
        requested: usize,
    },

    /// No sequence lengths were provided.
    EmptySequenceLengths,

    /// A sequence length was zero when zero was not permitted.
    InvalidSequenceLength {
        /// Invalid length.
        length: usize,
    },

    /// Too many distinct sequence lengths were requested.
    TooManySequenceLengths {
        /// Requested count.
        requested: usize,

        /// Maximum allowed count.
        maximum: usize,
    },

    /// Duplicate sequence length.
    DuplicateSequenceLength {
        /// Duplicated length.
        length: usize,
    },

    /// Too many sequences were requested per length.
    InvalidSequencesPerLength {
        /// Requested count.
        requested: usize,

        /// Maximum.
        maximum: usize,
    },

    /// Invalid shot count.
    InvalidShots {
        /// Requested shots.
        requested: usize,

        /// Maximum.
        maximum: usize,
    },

    /// Total-shot budget exceeded.
    TotalShotLimitExceeded {
        /// Requested total.
        requested: u128,

        /// Maximum total.
        maximum: u128,
    },

    /// Arithmetic overflow occurred during workload calculation.
    ResourceCalculationOverflow,

    /// Invalid confidence level.
    InvalidConfidenceLevel {
        /// Invalid confidence level.
        value: f64,
    },

    /// Invalid seed configuration.
    InvalidSeed,

    /// Invalid observation.
    InvalidObservation {
        /// Sequence length.
        length: usize,

        /// Number of successful outcomes.
        successes: u64,

        /// Number of shots.
        shots: u64,
    },

    /// Observation probability is not finite.
    NonFiniteObservation {
        /// Sequence length.
        length: usize,
    },

    /// Too few distinct sequence lengths exist for the requested model.
    InsufficientSequenceLengths {
        /// Number supplied.
        supplied: usize,

        /// Minimum required.
        minimum: usize,
    },

    /// Independent values cannot be used for fitting.
    InvalidFitData,

    /// Fit did not produce a finite result.
    FitFailed,

    /// Fitted decay parameter is outside the physical model domain.
    InvalidDecayParameter {
        /// Fitted p.
        value: f64,
    },

    /// Fitted amplitude is non-finite.
    InvalidAmplitude,

    /// Fitted offset is non-finite.
    InvalidOffset,

    /// Statistical model could not be identified.
    SingularModel,

    /// Numerical operation failed.
    NumericalFailure {
        /// Operation that failed.
        operation: &'static str,
    },

    /// Clifford generator error.
    Clifford(CliffordError),
}

impl fmt::Display for RandomizedBenchmarkingError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedQubitCount { requested } => {
                write!(
                    formatter,
                    "standard C1 randomized benchmarking currently \
                     supports exactly one qubit; requested {requested}"
                )
            }

            Self::EmptySequenceLengths => {
                write!(
                    formatter,
                    "randomized benchmarking requires at least one \
                     sequence length"
                )
            }

            Self::InvalidSequenceLength { length } => {
                write!(
                    formatter,
                    "invalid randomized-benchmarking sequence length {length}"
                )
            }

            Self::TooManySequenceLengths {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "requested {requested} sequence lengths; maximum is \
                     {maximum}"
                )
            }

            Self::DuplicateSequenceLength { length } => {
                write!(
                    formatter,
                    "sequence length {length} occurs more than once"
                )
            }

            Self::InvalidSequencesPerLength {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "requested {requested} sequences per length; maximum \
                     is {maximum}"
                )
            }

            Self::InvalidShots {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "requested {requested} shots per sequence; maximum \
                     is {maximum}"
                )
            }

            Self::TotalShotLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "requested {requested} total shots; maximum is {maximum}"
                )
            }

            Self::ResourceCalculationOverflow => {
                write!(
                    formatter,
                    "randomized-benchmarking resource calculation overflowed"
                )
            }

            Self::InvalidConfidenceLevel { value } => {
                write!(
                    formatter,
                    "confidence level must be finite and in (0, 1), got {value}"
                )
            }

            Self::InvalidSeed => {
                write!(
                    formatter,
                    "randomized-benchmarking seed configuration is invalid"
                )
            }

            Self::InvalidObservation {
                length,
                successes,
                shots,
            } => {
                write!(
                    formatter,
                    "invalid RB observation at length {length}: \
                     successes={successes}, shots={shots}"
                )
            }

            Self::NonFiniteObservation { length } => {
                write!(
                    formatter,
                    "non-finite survival probability at sequence length \
                     {length}"
                )
            }

            Self::InsufficientSequenceLengths {
                supplied,
                minimum,
            } => {
                write!(
                    formatter,
                    "RB exponential fitting requires at least {minimum} \
                     distinct sequence lengths; got {supplied}"
                )
            }

            Self::InvalidFitData => {
                write!(
                    formatter,
                    "RB observations cannot be fitted by the requested \
                     exponential model"
                )
            }

            Self::FitFailed => {
                write!(
                    formatter,
                    "randomized-benchmarking exponential fit failed"
                )
            }

            Self::InvalidDecayParameter { value } => {
                write!(
                    formatter,
                    "fitted RB decay parameter p={value} is outside [0, 1]"
                )
            }

            Self::InvalidAmplitude => {
                write!(
                    formatter,
                    "fitted RB amplitude is not finite"
                )
            }

            Self::InvalidOffset => {
                write!(
                    formatter,
                    "fitted RB offset is not finite"
                )
            }

            Self::SingularModel => {
                write!(
                    formatter,
                    "randomized-benchmarking exponential model is \
                     numerically singular"
                )
            }

            Self::NumericalFailure { operation } => {
                write!(
                    formatter,
                    "non-finite numerical result during {operation}"
                )
            }

            Self::Clifford(error) => {
                write!(formatter, "Clifford-generation error: {error}")
            }
        }
    }
}

impl Error for RandomizedBenchmarkingError {}

impl From<CliffordError> for RandomizedBenchmarkingError {
    fn from(error: CliffordError) -> Self {
        Self::Clifford(error)
    }
}

// =============================================================================
// Protocol assumptions
// =============================================================================

/// Explicit assumptions under which the standard RB interpretation is made.
///
/// These are metadata, not claims that the assumptions are universally true.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RandomizedBenchmarkingAssumptions {
    /// Standard C1 Clifford sampling is uniform.
    pub uniform_clifford_sampling: bool,

    /// The complete sequence is intended to implement identity ideally.
    pub recovery_is_inverse: bool,

    /// Survival probability is represented by a binomial outcome.
    pub binary_survival_measurement: bool,

    /// A single exponential is used for the primary fit.
    pub single_exponential_model: bool,

    /// SPAM effects are represented by A and B.
    pub spam_absorbed_into_amplitude_and_offset: bool,

    /// The reported EPC is the decay-derived metric.
    pub epc_is_decay_derived: bool,
}

impl RandomizedBenchmarkingAssumptions {
    /// Returns the assumptions for this implementation.
    pub const fn standard() -> Self {
        Self {
            uniform_clifford_sampling: true,
            recovery_is_inverse: true,
            binary_survival_measurement: true,
            single_exponential_model: true,
            spam_absorbed_into_amplitude_and_offset: true,
            epc_is_decay_derived: true,
        }
    }
}

// =============================================================================
// Configuration
// =============================================================================

/// Production configuration for single-qubit Clifford randomized
/// benchmarking.
#[derive(Debug, Clone, PartialEq)]
pub struct RandomizedBenchmarkingConfig {
    /// Number of qubits.
    ///
    /// Must currently be exactly one because this protocol uses exact uniform
    /// sampling from C1.
    pub num_qubits: usize,

    /// Sequence lengths in numbers of random Clifford operations.
    ///
    /// The recovery Clifford is not included in this count.
    pub sequence_lengths: Vec<usize>,

    /// Number of independently sampled sequences for each length.
    pub sequences_per_length: usize,

    /// Number of shots for each sequence.
    pub shots_per_sequence: usize,

    /// Explicit deterministic seed.
    pub seed: u64,

    /// Confidence level for protocol uncertainty metadata.
    pub confidence_level: f64,

    /// Maximum sequence length.
    pub max_sequence_length: usize,

    /// Maximum sequence lengths.
    pub max_sequence_lengths: usize,

    /// Maximum sequences per length.
    pub max_sequences_per_length: usize,

    /// Maximum shots per sequence.
    pub max_shots_per_sequence: usize,

    /// Maximum total shots.
    pub max_total_shots: u128,

    /// Maximum number of grid points used by the local fit implementation.
    pub fit_grid_points: usize,

    /// Maximum golden-section iterations.
    pub fit_refinement_iterations: usize,

    /// Relative fitting tolerance.
    pub fit_relative_tolerance: f64,

    /// Absolute fitting tolerance.
    pub fit_absolute_tolerance: f64,
}

impl Default for RandomizedBenchmarkingConfig {
    fn default() -> Self {
        Self {
            num_qubits: 1,
            sequence_lengths: vec![1, 2, 4, 8, 16, 32],
            sequences_per_length: DEFAULT_SEQUENCES_PER_LENGTH,
            shots_per_sequence: DEFAULT_SHOTS_PER_SEQUENCE,
            seed: 0x5A4D_5242_0000_0001,
            confidence_level: DEFAULT_CONFIDENCE_LEVEL,
            max_sequence_length: DEFAULT_MAX_SEQUENCE_LENGTH,
            max_sequence_lengths: DEFAULT_MAX_SEQUENCE_LENGTHS,
            max_sequences_per_length: DEFAULT_MAX_SEQUENCES_PER_LENGTH,
            max_shots_per_sequence: DEFAULT_MAX_SHOTS_PER_SEQUENCE,
            max_total_shots: DEFAULT_MAX_TOTAL_SHOTS,
            fit_grid_points: DEFAULT_FIT_GRID_POINTS,
            fit_refinement_iterations: DEFAULT_FIT_REFINEMENT_ITERATIONS,
            fit_relative_tolerance: DEFAULT_FIT_RELATIVE_TOLERANCE,
            fit_absolute_tolerance: DEFAULT_FIT_ABSOLUTE_TOLERANCE,
        }
    }
}

impl RandomizedBenchmarkingConfig {
    /// Creates the standard default configuration.
    pub fn new() -> Result<Self, RandomizedBenchmarkingError> {
        let config = Self::default();
        config.validate()?;
        Ok(config)
    }

    /// Validates the complete configuration.
    pub fn validate(&self) -> Result<(), RandomizedBenchmarkingError> {
        if self.num_qubits != 1 {
            return Err(
                RandomizedBenchmarkingError::UnsupportedQubitCount {
                    requested: self.num_qubits,
                },
            );
        }

        if self.sequence_lengths.is_empty() {
            return Err(
                RandomizedBenchmarkingError::EmptySequenceLengths,
            );
        }

        if self.sequence_lengths.len() > self.max_sequence_lengths {
            return Err(
                RandomizedBenchmarkingError::TooManySequenceLengths {
                    requested: self.sequence_lengths.len(),
                    maximum: self.max_sequence_lengths,
                },
            );
        }

        let mut previous: Option<usize> = None;

        for &length in &self.sequence_lengths {
            if length == 0 {
                return Err(
                    RandomizedBenchmarkingError::InvalidSequenceLength {
                        length,
                    },
                );
            }

            if length > self.max_sequence_length {
                return Err(
                    RandomizedBenchmarkingError::InvalidSequenceLength {
                        length,
                    },
                );
            }

            if let Some(previous_length) = previous {
                if length == previous_length {
                    return Err(
                        RandomizedBenchmarkingError::DuplicateSequenceLength {
                            length,
                        },
                    );
                }

                if length < previous_length {
                    return Err(
                        RandomizedBenchmarkingError::InvalidSequenceLength {
                            length,
                        },
                    );
                }
            }

            previous = Some(length);
        }

        if self.sequences_per_length == 0
            || self.sequences_per_length > self.max_sequences_per_length
        {
            return Err(
                RandomizedBenchmarkingError::InvalidSequencesPerLength {
                    requested: self.sequences_per_length,
                    maximum: self.max_sequences_per_length,
                },
            );
        }

        if self.shots_per_sequence == 0
            || self.shots_per_sequence > self.max_shots_per_sequence
        {
            return Err(RandomizedBenchmarkingError::InvalidShots {
                requested: self.shots_per_sequence,
                maximum: self.max_shots_per_sequence,
            });
        }

        let total_shots = (self.sequence_lengths.len() as u128)
            .checked_mul(self.sequences_per_length as u128)
            .and_then(|value| {
                value.checked_mul(self.shots_per_sequence as u128)
            })
            .ok_or(
                RandomizedBenchmarkingError::ResourceCalculationOverflow,
            )?;

        if total_shots > self.max_total_shots {
            return Err(
                RandomizedBenchmarkingError::TotalShotLimitExceeded {
                    requested: total_shots,
                    maximum: self.max_total_shots,
                },
            );
        }

        if !self.confidence_level.is_finite()
            || self.confidence_level <= 0.0
            || self.confidence_level >= 1.0
        {
            return Err(
                RandomizedBenchmarkingError::InvalidConfidenceLevel {
                    value: self.confidence_level,
                },
            );
        }

        if self.seed == 0 {
            // Zero is technically usable by RNGs, but Zamani uses an explicit
            // non-zero seed policy so an omitted/default seed is distinguishable
            // from an intentional deterministic seed.
            return Err(RandomizedBenchmarkingError::InvalidSeed);
        }

        if self.fit_grid_points < 2 {
            return Err(
                RandomizedBenchmarkingError::NumericalFailure {
                    operation: "fit grid configuration",
                },
            );
        }

        if self.fit_refinement_iterations == 0 {
            return Err(
                RandomizedBenchmarkingError::NumericalFailure {
                    operation: "fit refinement configuration",
                },
            );
        }

        if !self.fit_relative_tolerance.is_finite()
            || self.fit_relative_tolerance <= 0.0
        {
            return Err(
                RandomizedBenchmarkingError::NumericalFailure {
                    operation: "relative fit tolerance",
                },
            );
        }

        if !self.fit_absolute_tolerance.is_finite()
            || self.fit_absolute_tolerance <= 0.0
        {
            return Err(
                RandomizedBenchmarkingError::NumericalFailure {
                    operation: "absolute fit tolerance",
                },
            );
        }

        Ok(())
    }

    /// Returns the total number of shots requested.
    pub fn total_shots(&self) -> Result<u128, RandomizedBenchmarkingError> {
        (self.sequence_lengths.len() as u128)
            .checked_mul(self.sequences_per_length as u128)
            .and_then(|value| {
                value.checked_mul(self.shots_per_sequence as u128)
            })
            .ok_or(
                RandomizedBenchmarkingError::ResourceCalculationOverflow,
            )
    }
}

// =============================================================================
// Sequence representation
// =============================================================================

/// One generated RB sequence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RandomizedBenchmarkingSequence {
    /// Stable zero-based sequence identifier within the complete experiment.
    pub sequence_id: usize,

    /// RB sequence length excluding recovery.
    pub length: usize,

    /// Random Clifford elements in application order.
    pub random_cliffords: Vec<SingleQubitClifford>,

    /// Recovery Clifford.
    pub recovery: SingleQubitClifford,

    /// Complete primitive logical sequence, including recovery.
    pub primitive_sequence: CliffordPrimitiveSequence,
}

impl RandomizedBenchmarkingSequence {
    /// Returns the number of random Clifford operations.
    pub fn random_clifford_count(&self) -> usize {
        self.random_cliffords.len()
    }

    /// Returns the total number of logical Clifford operations including
    /// recovery.
    pub fn total_clifford_count(&self) -> usize {
        self.random_cliffords.len() + 1
    }

    /// Validates that the recovery exactly inverts the accumulated random
    /// Clifford product.
    pub fn validate_recovery(
        &self,
    ) -> Result<(), RandomizedBenchmarkingError> {
        let mut accumulated = SingleQubitClifford::IDENTITY;

        for clifford in &self.random_cliffords {
            accumulated = clifford.compose(accumulated)?;
        }

        let expected_recovery = accumulated.inverse()?;

        if expected_recovery != self.recovery {
            return Err(
                RandomizedBenchmarkingError::NumericalFailure {
                    operation: "RB recovery validation",
                },
            );
        }

        Ok(())
    }

    /// Returns whether the complete ideal Clifford sequence is identity.
    pub fn is_ideal_identity(
        &self,
    ) -> Result<bool, RandomizedBenchmarkingError> {
        let mut accumulated = SingleQubitClifford::IDENTITY;

        for clifford in &self.random_cliffords {
            accumulated = clifford.compose(accumulated)?;
        }

        accumulated = self.recovery.compose(accumulated)?;

        Ok(accumulated == SingleQubitClifford::IDENTITY)
    }
}

// =============================================================================
// Experiment generator
// =============================================================================

/// Generator for deterministic single-qubit Clifford RB experiments.
#[derive(Debug, Clone)]
pub struct RandomizedBenchmarkingGenerator {
    config: RandomizedBenchmarkingConfig,
}

impl RandomizedBenchmarkingGenerator {
    /// Creates a validated generator.
    pub fn new(
        config: RandomizedBenchmarkingConfig,
    ) -> Result<Self, RandomizedBenchmarkingError> {
        config.validate()?;

        Ok(Self { config })
    }

    /// Returns the generator configuration.
    pub fn config(&self) -> &RandomizedBenchmarkingConfig {
        &self.config
    }

    /// Generates the complete experiment deterministically.
    ///
    /// The generation order is:
    ///
    /// 1. sequence lengths in configuration order;
    /// 2. sequence index in ascending order;
    /// 3. random Clifford position in ascending order.
    ///
    /// Consequently the same configuration and seed produce the same
    /// sequence definitions.
    pub fn generate(
        &self,
    ) -> Result<Vec<RandomizedBenchmarkingSequence>,
        RandomizedBenchmarkingError>
    {
        let mut rng = StdRng::seed_from_u64(self.config.seed);
        self.generate_with_rng(&mut rng)
    }

    /// Generates the complete experiment from an externally supplied RNG.
    ///
    /// This is the integration point for the future benchmark-wide
    /// `generators::random` abstraction.
    pub fn generate_with_rng<R>(
        &self,
        rng: &mut R,
    ) -> Result<Vec<RandomizedBenchmarkingSequence>,
        RandomizedBenchmarkingError>
    where
        R: RngCore + ?Sized,
    {
        let total_sequences = self
            .config
            .sequence_lengths
            .len()
            .checked_mul(self.config.sequences_per_length)
            .ok_or(
                RandomizedBenchmarkingError::ResourceCalculationOverflow,
            )?;

        let mut sequences = Vec::with_capacity(total_sequences);
        let sampler = CliffordSampler::new();

        let mut sequence_id = 0usize;

        for &length in &self.config.sequence_lengths {
            for _ in 0..self.config.sequences_per_length {
                let random_cliffords =
                    sampler.sample_sequence(rng, length)?;

                let mut accumulated =
                    SingleQubitClifford::IDENTITY;

                for clifford in &random_cliffords {
                    accumulated =
                        clifford.compose(accumulated)?;
                }

                let recovery = accumulated.inverse()?;

                let primitive_capacity =
                    estimate_primitive_capacity(
                        &random_cliffords,
                        recovery,
                    )?;

                let mut primitive_sequence =
                    CliffordPrimitiveSequence::with_capacity(
                        primitive_capacity,
                    );

                for clifford in &random_cliffords {
                    let decomposition = clifford.decomposition()?;

                    for primitive in decomposition {
                        primitive_sequence.push(
                            CliffordOperation::new(0, primitive),
                        );
                    }
                }

                for primitive in recovery.decomposition()? {
                    primitive_sequence.push(
                        CliffordOperation::new(0, primitive),
                    );
                }

                let sequence =
                    RandomizedBenchmarkingSequence {
                        sequence_id,
                        length,
                        random_cliffords,
                        recovery,
                        primitive_sequence,
                    };

                sequence.validate_recovery()?;

                if !sequence.is_ideal_identity()? {
                    return Err(
                        RandomizedBenchmarkingError::NumericalFailure {
                            operation: "RB identity verification",
                        },
                    );
                }

                sequences.push(sequence);
                sequence_id = sequence_id
                    .checked_add(1)
                    .ok_or(
                        RandomizedBenchmarkingError::
                            ResourceCalculationOverflow,
                    )?;
            }
        }

        Ok(sequences)
    }
}

// =============================================================================
// Observation model
// =============================================================================

/// Raw binary observation for one generated sequence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RandomizedBenchmarkingObservation {
    /// Sequence identifier.
    pub sequence_id: usize,

    /// Number of random Clifford operations.
    pub length: usize,

    /// Number of successful survival outcomes.
    pub successes: u64,

    /// Number of shots.
    pub shots: u64,
}

impl RandomizedBenchmarkingObservation {
    /// Creates and validates an observation.
    pub fn new(
        sequence_id: usize,
        length: usize,
        successes: u64,
        shots: u64,
    ) -> Result<Self, RandomizedBenchmarkingError> {
        if length == 0 || shots == 0 || successes > shots {
            return Err(
                RandomizedBenchmarkingError::InvalidObservation {
                    length,
                    successes,
                    shots,
                },
            );
        }

        Ok(Self {
            sequence_id,
            length,
            successes,
            shots,
        })
    }

    /// Returns the measured survival probability.
    pub fn survival_probability(
        &self,
    ) -> Result<f64, RandomizedBenchmarkingError> {
        let probability =
            self.successes as f64 / self.shots as f64;

        if !probability.is_finite() {
            return Err(
                RandomizedBenchmarkingError::NonFiniteObservation {
                    length: self.length,
                },
            );
        }

        Ok(probability.clamp(0.0, 1.0))
    }
}

/// Aggregated observations at one sequence length.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AggregatedObservation {
    /// Sequence length.
    pub length: usize,

    /// Number of sequences represented.
    pub sequence_count: usize,

    /// Total successful shots.
    pub successes: u64,

    /// Total shots.
    pub shots: u64,

    /// Pooled survival probability.
    pub survival_probability: f64,
}

impl AggregatedObservation {
    fn new(
        length: usize,
        sequence_count: usize,
        successes: u64,
        shots: u64,
    ) -> Result<Self, RandomizedBenchmarkingError> {
        if shots == 0 || successes > shots {
            return Err(
                RandomizedBenchmarkingError::InvalidObservation {
                    length,
                    successes,
                    shots,
                },
            );
        }

        let survival_probability =
            successes as f64 / shots as f64;

        if !survival_probability.is_finite() {
            return Err(
                RandomizedBenchmarkingError::NonFiniteObservation {
                    length,
                },
            );
        }

        Ok(Self {
            length,
            sequence_count,
            successes,
            shots,
            survival_probability,
        })
    }
}

// =============================================================================
// Fit diagnostics
// =============================================================================

/// Diagnostics describing the quality of an RB exponential fit.
#[derive(Debug, Clone, PartialEq)]
pub struct RandomizedBenchmarkingFitDiagnostics {
    /// Number of distinct sequence lengths.
    pub observations: usize,

    /// Total shots represented by the fit.
    pub total_shots: u128,

    /// Sum of squared residuals.
    pub sum_squared_error: f64,

    /// Root mean squared error.
    pub rmse: f64,

    /// Coefficient of determination.
    ///
    /// `None` when the observed data have zero variance.
    pub r_squared: Option<f64>,

    /// Akaike information criterion.
    pub aic: Option<f64>,

    /// Bayesian information criterion.
    pub bic: Option<f64>,

    /// Whether the fitted decay parameter is at a model boundary.
    pub decay_parameter_at_boundary: bool,

    /// Whether the amplitude is at a numerical boundary.
    pub amplitude_at_boundary: bool,

    /// Whether the offset is at a numerical boundary.
    pub offset_at_boundary: bool,

    /// Whether the optimizer converged according to its stopping criterion.
    pub converged: bool,

    /// Number of profile-search evaluations.
    pub objective_evaluations: usize,

    /// Number of refinement iterations.
    pub refinement_iterations: usize,
}

/// Result of fitting:
///
/// ```text
/// P(m) = A * p^m + B
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct RandomizedBenchmarkingFit {
    /// Fitted amplitude A.
    pub amplitude: f64,

    /// Fitted decay parameter p.
    pub decay_parameter: f64,

    /// Fitted asymptotic offset B.
    pub offset: f64,

    /// Decay-derived error per Clifford.
    pub error_per_clifford: f64,

    /// Number of qubits represented by the fit.
    pub num_qubits: usize,

    /// Effective Hilbert-space dimension.
    pub dimension: u128,

    /// Fit diagnostics.
    pub diagnostics: RandomizedBenchmarkingFitDiagnostics,

    /// Confidence level requested by the protocol.
    ///
    /// This metadata does not imply that a confidence interval has been
    /// computed. This implementation deliberately avoids fabricating
    /// parameter uncertainty when the covariance model has not been
    /// independently validated.
    pub confidence_level: f64,
}

// =============================================================================
// Benchmark result
// =============================================================================

/// Complete protocol-level RB result.
#[derive(Debug, Clone, PartialEq)]
pub struct RandomizedBenchmarkingResult {
    /// Stable protocol identifier.
    pub benchmark_id: &'static str,

    /// Protocol schema version.
    pub schema_version: u32,

    /// Algorithm identifier.
    pub algorithm_id: &'static str,

    /// Number of qubits.
    pub num_qubits: usize,

    /// Configuration seed.
    pub seed: u64,

    /// Number of generated sequences.
    pub sequence_count: usize,

    /// Total shots.
    pub total_shots: u128,

    /// Aggregated observations.
    pub observations: Vec<AggregatedObservation>,

    /// Exponential fit.
    pub fit: RandomizedBenchmarkingFit,

    /// Explicit protocol assumptions.
    pub assumptions: RandomizedBenchmarkingAssumptions,

    /// Protocol warnings.
    pub warnings: Vec<RandomizedBenchmarkingWarning>,
}

/// Scientific/protocol warning.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RandomizedBenchmarkingWarning {
    /// The fitted p is close to one.
    DecayNearIdentity,

    /// The fitted p reached the lower numerical boundary.
    DecayAtLowerBoundary,

    /// The fitted p reached the upper numerical boundary.
    DecayAtUpperBoundary,

    /// The amplitude is weakly identified.
    WeakAmplitudeIdentification,

    /// The offset is weakly identified.
    WeakOffsetIdentification,

    /// Observations exhibit poor fit quality.
    PoorFitQuality,

    /// The fitted model has essentially no observable decay.
    NoObservableDecay,

    /// The fit is mathematically valid but scientifically model-dependent.
    ModelAssumptionRequired,

    /// The requested confidence level is metadata only.
    ConfidenceIntervalNotComputed,

    /// Hardware leakage may invalidate the standard interpretation.
    LeakageMayBiasStandardRb,

    /// Temporal correlations may invalidate a simple interpretation.
    TemporalCorrelationsMayBiasStandardRb,
}

impl RandomizedBenchmarkingWarning {
    /// Stable machine-readable identifier.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::DecayNearIdentity => "decay_near_identity",
            Self::DecayAtLowerBoundary => "decay_at_lower_boundary",
            Self::DecayAtUpperBoundary => "decay_at_upper_boundary",
            Self::WeakAmplitudeIdentification => {
                "weak_amplitude_identification"
            }
            Self::WeakOffsetIdentification => {
                "weak_offset_identification"
            }
            Self::PoorFitQuality => "poor_fit_quality",
            Self::NoObservableDecay => "no_observable_decay",
            Self::ModelAssumptionRequired => "model_assumption_required",
            Self::ConfidenceIntervalNotComputed => {
                "confidence_interval_not_computed"
            }
            Self::LeakageMayBiasStandardRb => {
                "leakage_may_bias_standard_rb"
            }
            Self::TemporalCorrelationsMayBiasStandardRb => {
                "temporal_correlations_may_bias_standard_rb"
            }
        }
    }
}

// =============================================================================
// Protocol analyzer
// =============================================================================

/// Analyzer for randomized-benchmarking observations.
#[derive(Debug, Clone)]
pub struct RandomizedBenchmarkingAnalyzer {
    config: RandomizedBenchmarkingConfig,
}

impl RandomizedBenchmarkingAnalyzer {
    /// Creates an analyzer from a validated configuration.
    pub fn new(
        config: RandomizedBenchmarkingConfig,
    ) -> Result<Self, RandomizedBenchmarkingError> {
        config.validate()?;

        Ok(Self { config })
    }

    /// Returns the configuration.
    pub fn config(&self) -> &RandomizedBenchmarkingConfig {
        &self.config
    }

    /// Aggregates per-sequence observations by sequence length.
    pub fn aggregate(
        &self,
        observations: &[RandomizedBenchmarkingObservation],
    ) -> Result<Vec<AggregatedObservation>,
        RandomizedBenchmarkingError>
    {
        if observations.is_empty() {
            return Err(
                RandomizedBenchmarkingError::InvalidFitData,
            );
        }

        let mut aggregated = Vec::new();

        for &configured_length in &self.config.sequence_lengths {
            let mut sequence_count = 0usize;
            let mut successes = 0u64;
            let mut shots = 0u64;

            for observation in observations
                .iter()
                .filter(|item| item.length == configured_length)
            {
                sequence_count = sequence_count
                    .checked_add(1)
                    .ok_or(
                        RandomizedBenchmarkingError::
                            ResourceCalculationOverflow,
                    )?;

                successes = successes
                    .checked_add(observation.successes)
                    .ok_or(
                        RandomizedBenchmarkingError::
                            ResourceCalculationOverflow,
                    )?;

                shots = shots
                    .checked_add(observation.shots)
                    .ok_or(
                        RandomizedBenchmarkingError::
                            ResourceCalculationOverflow,
                    )?;
            }

            if sequence_count == 0 {
                continue;
            }

            aggregated.push(
                AggregatedObservation::new(
                    configured_length,
                    sequence_count,
                    successes,
                    shots,
                )?,
            );
        }

        aggregated.sort_by_key(|item| item.length);

        Ok(aggregated)
    }

    /// Analyzes already captured observations.
    ///
    /// This is the primary offline-analysis API.
    pub fn analyze(
        &self,
        observations: &[RandomizedBenchmarkingObservation],
    ) -> Result<RandomizedBenchmarkingResult,
        RandomizedBenchmarkingError>
    {
        let aggregated = self.aggregate(observations)?;

        if aggregated.len()
            < MIN_SEQUENCE_LENGTHS_FOR_EXPONENTIAL_FIT
        {
            return Err(
                RandomizedBenchmarkingError::
                    InsufficientSequenceLengths {
                        supplied: aggregated.len(),
                        minimum:
                            MIN_SEQUENCE_LENGTHS_FOR_EXPONENTIAL_FIT,
                    },
            );
        }

        let fit = fit_exponential_decay(
            &aggregated,
            &self.config,
        )?;

        let mut warnings = Vec::new();

        if fit.diagnostics.r_squared
            .map(|value| value < 0.90)
            .unwrap_or(false)
        {
            warnings.push(
                RandomizedBenchmarkingWarning::PoorFitQuality,
            );
        }

        if fit.decay_parameter
            >= 1.0 - PARAMETER_BOUNDARY_EPSILON
        {
            warnings.push(
                RandomizedBenchmarkingWarning::DecayNearIdentity,
            );
            warnings.push(
                RandomizedBenchmarkingWarning::NoObservableDecay,
            );
        }

        if fit.decay_parameter <= PARAMETER_BOUNDARY_EPSILON {
            warnings.push(
                RandomizedBenchmarkingWarning::DecayAtLowerBoundary,
            );
        }

        if fit.decay_parameter
            >= 1.0 - PARAMETER_BOUNDARY_EPSILON
        {
            warnings.push(
                RandomizedBenchmarkingWarning::DecayAtUpperBoundary,
            );
        }

        if fit.amplitude.abs() < 1.0e-6 {
            warnings.push(
                RandomizedBenchmarkingWarning::
                    WeakAmplitudeIdentification,
            );
        }

        if fit.offset.abs() < 1.0e-6
            || (fit.offset - 1.0).abs() < 1.0e-6
        {
            warnings.push(
                RandomizedBenchmarkingWarning::
                    WeakOffsetIdentification,
            );
        }

        warnings.push(
            RandomizedBenchmarkingWarning::
                ModelAssumptionRequired,
        );

        warnings.push(
            RandomizedBenchmarkingWarning::
                ConfidenceIntervalNotComputed,
        );

        let sequence_count =
            observations.len();

        let total_shots =
            observations.iter().try_fold(
                0u128,
                |total, observation| {
                    total.checked_add(observation.shots as u128)
                        .ok_or(
                            RandomizedBenchmarkingError::
                                ResourceCalculationOverflow,
                        )
                },
            )?;

        Ok(RandomizedBenchmarkingResult {
            benchmark_id: RANDOMIZED_BENCHMARKING_ID,
            schema_version:
                RANDOMIZED_BENCHMARKING_SCHEMA_VERSION,
            algorithm_id:
                RANDOMIZED_BENCHMARKING_ALGORITHM_ID,
            num_qubits: self.config.num_qubits,
            seed: self.config.seed,
            sequence_count,
            total_shots,
            observations: aggregated,
            fit,
            assumptions:
                RandomizedBenchmarkingAssumptions::standard(),
            warnings,
        })
    }
}

// =============================================================================
// Exponential fitting
// =============================================================================

#[derive(Debug, Clone, Copy)]
struct FitCandidate {
    k: f64,
    amplitude: f64,
    offset: f64,
    sse: f64,
}

impl FitCandidate {
    fn p(self) -> f64 {
        (-self.k).exp()
    }
}

/// Fits:
///
/// ```text
/// y = A * exp(-k*x) + B
/// p = exp(-k)
/// ```
///
/// The nonlinear problem has only one nonlinear parameter (`k`). For each
/// candidate k, A and B are solved exactly by a two-parameter weighted linear
/// least-squares problem.
///
/// The implementation uses:
///
/// 1. deterministic grid search;
/// 2. local golden-section refinement;
/// 3. explicit physical bounds;
/// 4. residual diagnostics.
///
/// This is intentionally deterministic and dependency-light.
///
/// A future `statistics::regression` integration can replace this internal
/// implementation without changing the protocol-level result semantics.
fn fit_exponential_decay(
    observations: &[AggregatedObservation],
    config: &RandomizedBenchmarkingConfig,
) -> Result<RandomizedBenchmarkingFit,
    RandomizedBenchmarkingError>
{
    if observations.len()
        < MIN_SEQUENCE_LENGTHS_FOR_EXPONENTIAL_FIT
    {
        return Err(
            RandomizedBenchmarkingError::
                InsufficientSequenceLengths {
                    supplied: observations.len(),
                    minimum:
                        MIN_SEQUENCE_LENGTHS_FOR_EXPONENTIAL_FIT,
                },
        );
    }

    let mut x = Vec::with_capacity(observations.len());
    let mut y = Vec::with_capacity(observations.len());
    let mut weights = Vec::with_capacity(observations.len());

    for observation in observations {
        x.push(observation.length as f64);
        y.push(observation.survival_probability);

        // For a binomial proportion, an approximate inverse-variance weight
        // is proportional to n/[p(1-p)]. We clamp the probability away from
        // zero and one to prevent infinite weights.
        let p = observation
            .survival_probability
            .clamp(1.0e-9, 1.0 - 1.0e-9);

        let variance =
            p * (1.0 - p)
                / observation.shots as f64;

        let weight =
            if variance.is_finite() && variance > 0.0 {
                1.0 / variance
            } else {
                1.0
            };

        x.shrink_to_fit();
        y.shrink_to_fit();

        if !weight.is_finite() || weight <= 0.0 {
            return Err(
                RandomizedBenchmarkingError::
                    NumericalFailure {
                        operation: "RB observation weighting",
                    },
            );
        }

        weights.push(weight);
    }

    let mut best: Option<FitCandidate> = None;
    let mut objective_evaluations = 0usize;

    let grid_points =
        config.fit_grid_points.max(2);

    for index in 0..grid_points {
        let fraction =
            index as f64 / (grid_points - 1) as f64;

        let k =
            DEFAULT_MIN_DECAY_RATE
                + fraction
                    * (DEFAULT_MAX_DECAY_RATE
                        - DEFAULT_MIN_DECAY_RATE);

        if let Some(candidate) =
            solve_linear_amplitude_offset(
                &x,
                &y,
                &weights,
                k,
            )?
        {
            objective_evaluations =
                objective_evaluations
                    .checked_add(1)
                    .ok_or(
                        RandomizedBenchmarkingError::
                            ResourceCalculationOverflow,
                    )?;

            if candidate.sse.is_finite()
                && best
                    .map(|current| {
                        candidate.sse < current.sse
                    })
                    .unwrap_or(true)
            {
                best = Some(candidate);
            }
        }
    }

    let best =
        best.ok_or(RandomizedBenchmarkingError::FitFailed)?;

    let grid_step =
        DEFAULT_MAX_DECAY_RATE
            / (grid_points - 1) as f64;

    let lower =
        (best.k - grid_step).max(
            DEFAULT_MIN_DECAY_RATE,
        );

    let upper =
        (best.k + grid_step).min(
            DEFAULT_MAX_DECAY_RATE,
        );

    let refined =
        golden_section_minimize(
            &x,
            &y,
            &weights,
            lower,
            upper,
            config.fit_refinement_iterations,
            config.fit_relative_tolerance,
            config.fit_absolute_tolerance,
            &mut objective_evaluations,
        )?;

    let final_candidate =
        if refined.sse <= best.sse {
            refined
        } else {
            best
        };

    let p = final_candidate.p();

    if !p.is_finite()
        || p < MIN_DECAY_PARAMETER - UNIT_INTERVAL_EPSILON
        || p > MAX_DECAY_PARAMETER + UNIT_INTERVAL_EPSILON
    {
        return Err(
            RandomizedBenchmarkingError::
                InvalidDecayParameter {
                    value: p,
                },
        );
    }

    let p = p.clamp(
        MIN_DECAY_PARAMETER,
        MAX_DECAY_PARAMETER,
    );

    let dimension =
        checked_dimension(config.num_qubits)?;

    let error_per_clifford =
        ((dimension - 1) as f64
            / dimension as f64)
            * (1.0 - p);

    if !error_per_clifford.is_finite()
        || error_per_clifford < -UNIT_INTERVAL_EPSILON
        || error_per_clifford
            > 1.0 + UNIT_INTERVAL_EPSILON
    {
        return Err(
            RandomizedBenchmarkingError::
                NumericalFailure {
                    operation:
                        "RB error-per-Clifford calculation",
                },
        );
    }

    let diagnostics =
        calculate_fit_diagnostics(
            &x,
            &y,
            &weights,
            final_candidate,
            objective_evaluations,
            config.fit_refinement_iterations,
        )?;

    Ok(RandomizedBenchmarkingFit {
        amplitude: final_candidate.amplitude,
        decay_parameter: p,
        offset: final_candidate.offset,
        error_per_clifford:
            error_per_clifford.clamp(0.0, 1.0),
        num_qubits: config.num_qubits,
        dimension,
        diagnostics,
        confidence_level:
            config.confidence_level,
    })
}

fn solve_linear_amplitude_offset(
    x: &[f64],
    y: &[f64],
    weights: &[f64],
    k: f64,
) -> Result<Option<FitCandidate>,
    RandomizedBenchmarkingError>
{
    let mut s_00 = 0.0f64;
    let mut s_01 = 0.0f64;
    let mut s_11 = 0.0f64;
    let mut t_0 = 0.0f64;
    let mut t_1 = 0.0f64;

    for ((&xi, &yi), &weight) in
        x.iter()
            .zip(y.iter())
            .zip(weights.iter())
    {
        let basis =
            (-k * xi).exp();

        if !basis.is_finite() {
            return Err(
                RandomizedBenchmarkingError::
                    NumericalFailure {
                        operation:
                            "RB exponential basis",
                    },
            );
        }

        s_00 += weight * basis * basis;
        s_01 += weight * basis;
        s_11 += weight;

        t_0 += weight * basis * yi;
        t_1 += weight * yi;
    }

    let determinant =
        s_00 * s_11 - s_01 * s_01;

    if !determinant.is_finite() {
        return Err(
            RandomizedBenchmarkingError::
                NumericalFailure {
                    operation:
                        "RB linear-system determinant",
                },
        );
    }

    let scale =
        (s_00.abs() + s_11.abs())
            .max(1.0);

    if determinant.abs()
        <= 1.0e-14 * scale * scale
    {
        return Ok(None);
    }

    let amplitude =
        (t_0 * s_11 - t_1 * s_01)
            / determinant;

    let offset =
        (s_00 * t_1 - s_01 * t_0)
            / determinant;

    if !amplitude.is_finite()
        || !offset.is_finite()
    {
        return Ok(None);
    }

    let mut sse = 0.0;

    for ((&xi, &yi), &weight) in
        x.iter()
            .zip(y.iter())
            .zip(weights.iter())
    {
        let predicted =
            amplitude * (-k * xi).exp()
                + offset;

        if !predicted.is_finite() {
            return Ok(None);
        }

        let residual =
            yi - predicted;

        sse += weight
            * residual
            * residual;
    }

    if !sse.is_finite() {
        return Ok(None);
    }

    Ok(Some(FitCandidate {
        k,
        amplitude,
        offset,
        sse,
    }))
}

fn golden_section_minimize(
    x: &[f64],
    y: &[f64],
    weights: &[f64],
    mut lower: f64,
    mut upper: f64,
    max_iterations: usize,
    relative_tolerance: f64,
    absolute_tolerance: f64,
    evaluations: &mut usize,
) -> Result<FitCandidate,
    RandomizedBenchmarkingError>
{
    if lower >= upper {
        return solve_linear_amplitude_offset(
            x,
            y,
            weights,
            lower,
        )?
        .ok_or(
            RandomizedBenchmarkingError::FitFailed,
        );
    }

    const GOLDEN_RATIO_COMPLEMENT: f64 =
        0.618_033_988_749_894_8;

    let mut x1 =
        upper
            - GOLDEN_RATIO_COMPLEMENT
                * (upper - lower);

    let mut x2 =
        lower
            + GOLDEN_RATIO_COMPLEMENT
                * (upper - lower);

    let mut f1 =
        solve_linear_amplitude_offset(
            x,
            y,
            weights,
            x1,
        )?
        .ok_or(
            RandomizedBenchmarkingError::FitFailed,
        )?;

    let mut f2 =
        solve_linear_amplitude_offset(
            x,
            y,
            weights,
            x2,
        )?
        .ok_or(
            RandomizedBenchmarkingError::FitFailed,
        )?;

    *evaluations =
        evaluations
            .checked_add(2)
            .ok_or(
                RandomizedBenchmarkingError::
                    ResourceCalculationOverflow,
            )?;

    let mut iterations = 0usize;

    while iterations < max_iterations {
        let width =
            upper - lower;

        let tolerance =
            absolute_tolerance
                + relative_tolerance
                    * x1.abs().max(x2.abs()).max(1.0);

        if width <= tolerance {
            break;
        }

        if f1.sse <= f2.sse {
            upper = x2;
            x2 = x1;
            f2 = f1;

            x1 =
                upper
                    - GOLDEN_RATIO_COMPLEMENT
                        * (upper - lower);

            f1 =
                solve_linear_amplitude_offset(
                    x,
                    y,
                    weights,
                    x1,
                )?
                .ok_or(
                    RandomizedBenchmarkingError::
                        FitFailed,
                )?;
        } else {
            lower = x1;
            x1 = x2;
            f1 = f2;

            x2 =
                lower
                    + GOLDEN_RATIO_COMPLEMENT
                        * (upper - lower);

            f2 =
                solve_linear_amplitude_offset(
                    x,
                    y,
                    weights,
                    x2,
                )?
                .ok_or(
                    RandomizedBenchmarkingError::
                        FitFailed,
                )?;
        }

        *evaluations =
            evaluations
                .checked_add(1)
                .ok_or(
                    RandomizedBenchmarkingError::
                        ResourceCalculationOverflow,
                )?;

        iterations =
            iterations
                .checked_add(1)
                .ok_or(
                    RandomizedBenchmarkingError::
                        ResourceCalculationOverflow,
                )?;
    }

    if f1.sse <= f2.sse {
        Ok(f1)
    } else {
        Ok(f2)
    }
}

fn calculate_fit_diagnostics(
    x: &[f64],
    y: &[f64],
    weights: &[f64],
    candidate: FitCandidate,
    objective_evaluations: usize,
    refinement_iterations: usize,
) -> Result<RandomizedBenchmarkingFitDiagnostics,
    RandomizedBenchmarkingError>
{
    if x.len() != y.len()
        || y.len() != weights.len()
        || x.is_empty()
    {
        return Err(
            RandomizedBenchmarkingError::
                InvalidFitData,
        );
    }

    let total_weight =
        weights.iter().copied().sum::<f64>();

    if !total_weight.is_finite()
        || total_weight <= 0.0
    {
        return Err(
            RandomizedBenchmarkingError::
                NumericalFailure {
                    operation:
                        "RB diagnostic total weight",
                },
        );
    }

    let weighted_mean =
        y.iter()
            .zip(weights.iter())
            .map(|(&value, &weight)| {
                value * weight
            })
            .sum::<f64>()
            / total_weight;

    let mut weighted_sse = 0.0;
    let mut weighted_sst = 0.0;

    for ((&xi, &yi), &weight) in
        x.iter()
            .zip(y.iter())
            .zip(weights.iter())
    {
        let predicted =
            candidate.amplitude
                * (-candidate.k * xi).exp()
                + candidate.offset;

        let residual =
            yi - predicted;

        weighted_sse +=
            weight * residual * residual;

        let centered =
            yi - weighted_mean;

        weighted_sst +=
            weight * centered * centered;
    }

    if !weighted_sse.is_finite()
        || !weighted_sst.is_finite()
    {
        return Err(
            RandomizedBenchmarkingError::
                NumericalFailure {
                    operation:
                        "RB fit diagnostics",
                },
        );
    }

    let rmse =
        (weighted_sse
            / x.len() as f64)
            .sqrt();

    let r_squared =
        if weighted_sst
            > 1.0e-24
        {
            Some(
                1.0
                    - weighted_sse
                        / weighted_sst,
            )
        } else {
            None
        };

    let n = x.len() as f64;
    let parameter_count = 3.0;

    let aic =
        if weighted_sse > 0.0 {
            Some(
                n * weighted_sse.ln()
                    + 2.0 * parameter_count,
            )
        } else {
            None
        };

    let bic =
        if weighted_sse > 0.0 {
            Some(
                n * weighted_sse.ln()
                    + parameter_count * n.ln(),
            )
        } else {
            None
        };

    let p =
        candidate.p();

    let decay_boundary =
        p <= PARAMETER_BOUNDARY_EPSILON
            || p
                >= 1.0
                    - PARAMETER_BOUNDARY_EPSILON;

    let amplitude_boundary =
        candidate.amplitude.abs()
            <= PARAMETER_BOUNDARY_EPSILON;

    let offset_boundary =
        candidate.offset.abs()
            <= PARAMETER_BOUNDARY_EPSILON
            || (candidate.offset - 1.0).abs()
                <= PARAMETER_BOUNDARY_EPSILON;

    let converged =
        refinement_iterations
            < DEFAULT_FIT_REFINEMENT_ITERATIONS
            || candidate.k
                <= DEFAULT_MIN_DECAY_RATE
                    + DEFAULT_FIT_ABSOLUTE_TOLERANCE
            || candidate.k
                >= DEFAULT_MAX_DECAY_RATE
                    - DEFAULT_FIT_ABSOLUTE_TOLERANCE;

    Ok(RandomizedBenchmarkingFitDiagnostics {
        observations: x.len(),
        total_shots: 0,
        sum_squared_error: weighted_sse,
        rmse,
        r_squared,
        aic,
        bic,
        decay_parameter_at_boundary:
            decay_boundary,
        amplitude_at_boundary:
            amplitude_boundary,
        offset_at_boundary:
            offset_boundary,
        converged,
        objective_evaluations,
        refinement_iterations,
    })
}

// =============================================================================
// Utility functions
// =============================================================================

fn checked_dimension(
    num_qubits: usize,
) -> Result<u128,
    RandomizedBenchmarkingError>
{
    if num_qubits >= 128 {
        return Err(
            RandomizedBenchmarkingError::
                ResourceCalculationOverflow,
        );
    }

    1u128
        .checked_shl(num_qubits as u32)
        .ok_or(
            RandomizedBenchmarkingError::
                ResourceCalculationOverflow,
        )
}

fn estimate_primitive_capacity(
    random_cliffords: &[SingleQubitClifford],
    recovery: SingleQubitClifford,
) -> Result<usize,
    RandomizedBenchmarkingError>
{
    let mut capacity = 0usize;

    for clifford in random_cliffords {
        capacity = capacity
            .checked_add(
                clifford.decomposition_len()?,
            )
            .ok_or(
                RandomizedBenchmarkingError::
                    ResourceCalculationOverflow,
            )?;
    }

    capacity = capacity
        .checked_add(
            recovery.decomposition_len()?,
        )
        .ok_or(
            RandomizedBenchmarkingError::
                ResourceCalculationOverflow,
        )?;

    Ok(capacity)
}

// =============================================================================
// Public convenience functions
// =============================================================================

/// Generates a deterministic standard single-qubit RB experiment.
pub fn generate(
    config: RandomizedBenchmarkingConfig,
) -> Result<Vec<RandomizedBenchmarkingSequence>,
    RandomizedBenchmarkingError>
{
    RandomizedBenchmarkingGenerator::new(config)?
        .generate()
}

/// Analyzes captured RB observations.
pub fn analyze(
    config: RandomizedBenchmarkingConfig,
    observations: &[RandomizedBenchmarkingObservation],
) -> Result<RandomizedBenchmarkingResult,
    RandomizedBenchmarkingError>
{
    RandomizedBenchmarkingAnalyzer::new(config)?
        .analyze(observations)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config() -> RandomizedBenchmarkingConfig {
        RandomizedBenchmarkingConfig {
            sequence_lengths: vec![1, 2, 4, 8, 16, 32],
            sequences_per_length: 2,
            shots_per_sequence: 100,
            ..RandomizedBenchmarkingConfig::default()
        }
    }

    #[test]
    fn default_configuration_is_valid() {
        let config =
            RandomizedBenchmarkingConfig::default();

        assert!(config.validate().is_ok());
    }

    #[test]
    fn zero_qubits_are_rejected() {
        let mut config =
            RandomizedBenchmarkingConfig::default();

        config.num_qubits = 0;

        assert!(matches!(
            config.validate(),
            Err(
                RandomizedBenchmarkingError::
                    UnsupportedQubitCount {
                        requested: 0
                    }
            )
        ));
    }

    #[test]
    fn multiqubit_standard_rb_is_not_falsely_supported() {
        let mut config =
            RandomizedBenchmarkingConfig::default();

        config.num_qubits = 2;

        assert!(matches!(
            config.validate(),
            Err(
                RandomizedBenchmarkingError::
                    UnsupportedQubitCount {
                        requested: 2
                    }
            )
        ));
    }

    #[test]
    fn duplicate_lengths_are_rejected() {
        let mut config =
            RandomizedBenchmarkingConfig::default();

        config.sequence_lengths =
            vec![1, 2, 2, 4];

        assert!(matches!(
            config.validate(),
            Err(
                RandomizedBenchmarkingError::
                    DuplicateSequenceLength {
                        length: 2
                    }
            )
        ));
    }

    #[test]
    fn unsorted_lengths_are_rejected() {
        let mut config =
            RandomizedBenchmarkingConfig::default();

        config.sequence_lengths =
            vec![1, 8, 4];

        assert!(matches!(
            config.validate(),
            Err(
                RandomizedBenchmarkingError::
                    InvalidSequenceLength {
                        length: 4
                    }
            )
        ));
    }

    #[test]
    fn total_shots_are_checked() {
        let mut config =
            RandomizedBenchmarkingConfig::default();

        config.sequence_lengths =
            vec![1, 2, 4];

        config.sequences_per_length = 10;
        config.shots_per_sequence = 100;

        assert_eq!(
            config.total_shots().unwrap(),
            3_000
        );
    }

    #[test]
    fn deterministic_generation_is_reproducible() {
        let config = test_config();

        let first =
            RandomizedBenchmarkingGenerator::new(
                config.clone(),
            )
            .unwrap()
            .generate()
            .unwrap();

        let second =
            RandomizedBenchmarkingGenerator::new(config)
                .unwrap()
                .generate()
                .unwrap();

        assert_eq!(first, second);
    }

    #[test]
    fn generated_sequences_have_identity_recovery() {
        let config = test_config();

        let sequences =
            RandomizedBenchmarkingGenerator::new(config)
                .unwrap()
                .generate()
                .unwrap();

        assert!(!sequences.is_empty());

        for sequence in sequences {
            assert!(
                sequence.validate_recovery().is_ok()
            );

            assert!(
                sequence
                    .is_ideal_identity()
                    .unwrap()
            );
        }
    }

    #[test]
    fn recovery_is_not_counted_as_random_length() {
        let mut config =
            RandomizedBenchmarkingConfig::default();

        config.sequence_lengths =
            vec![5];

        config.sequences_per_length = 1;

        let sequence =
            RandomizedBenchmarkingGenerator::new(config)
                .unwrap()
                .generate()
                .unwrap()
                .remove(0);

        assert_eq!(
            sequence.random_clifford_count(),
            5
        );

        assert_eq!(
            sequence.total_clifford_count(),
            6
        );
    }

    #[test]
    fn observation_probability_is_correct() {
        let observation =
            RandomizedBenchmarkingObservation::new(
                0,
                10,
                75,
                100,
            )
            .unwrap();

        assert!(
            (observation
                .survival_probability()
                .unwrap()
                - 0.75)
                .abs()
                < 1.0e-12
        );
    }

    #[test]
    fn invalid_observation_is_rejected() {
        assert!(matches!(
            RandomizedBenchmarkingObservation::new(
                0,
                10,
                101,
                100,
            ),
            Err(
                RandomizedBenchmarkingError::
                    InvalidObservation { .. }
            )
        ));
    }

    #[test]
    fn aggregate_observations_by_length() {
        let config = test_config();

        let analyzer =
            RandomizedBenchmarkingAnalyzer::new(
                config,
            )
            .unwrap();

        let observations = vec![
            RandomizedBenchmarkingObservation::new(
                0, 1, 90, 100,
            )
            .unwrap(),
            RandomizedBenchmarkingObservation::new(
                1, 1, 80, 100,
            )
            .unwrap(),
            RandomizedBenchmarkingObservation::new(
                2, 2, 70, 100,
            )
            .unwrap(),
        ];

        let aggregated =
            analyzer
                .aggregate(&observations)
                .unwrap();

        assert_eq!(aggregated.len(), 2);

        assert_eq!(
            aggregated[0].length,
            1
        );

        assert_eq!(
            aggregated[0].successes,
            170
        );

        assert_eq!(
            aggregated[0].shots,
            200
        );

        assert!(
            (aggregated[0]
                .survival_probability
                - 0.85)
                .abs()
                < 1.0e-12
        );
    }

    #[test]
    fn synthetic_decay_can_be_fitted() {
        let config =
            RandomizedBenchmarkingConfig {
                sequence_lengths:
                    vec![1, 2, 4, 8, 16, 32],
                sequences_per_length: 1,
                shots_per_sequence:
                    1_000_000,
                ..RandomizedBenchmarkingConfig::default()
            };

        let p = 0.98;
        let a = 0.48;
        let b = 0.50;

        let observations =
            config
                .sequence_lengths
                .iter()
                .enumerate()
                .map(|(index, &length)| {
                    let probability =
                        a * p.powi(length as i32)
                            + b;

                    let successes =
                        (probability
                            * config.shots_per_sequence
                                as f64)
                            .round()
                            as u64;

                    RandomizedBenchmarkingObservation::new(
                        index,
                        length,
                        successes,
                        config.shots_per_sequence
                            as u64,
                    )
                    .unwrap()
                })
                .collect::<Vec<_>>();

        let result =
            analyze(
                config,
                &observations,
            )
            .unwrap();

        assert!(
            (result.fit.decay_parameter - p)
                .abs()
                < 0.01,
            "fitted p={} expected approximately {}",
            result.fit.decay_parameter,
            p
        );

        assert!(
            result.fit.error_per_clifford
                >= 0.0
        );

        assert!(
            result.fit.error_per_clifford
                <= 1.0
        );
    }

    #[test]
    fn ideal_no_decay_is_detected() {
        let config =
            RandomizedBenchmarkingConfig {
                sequence_lengths:
                    vec![1, 2, 4, 8, 16, 32],
                sequences_per_length: 1,
                shots_per_sequence: 10_000,
                ..RandomizedBenchmarkingConfig::default()
            };

        let observations =
            config
                .sequence_lengths
                .iter()
                .enumerate()
                .map(|(index, &length)| {
                    RandomizedBenchmarkingObservation::new(
                        index,
                        length,
                        10_000,
                        10_000,
                    )
                    .unwrap()
                })
                .collect::<Vec<_>>();

        let result =
            analyze(
                config,
                &observations,
            )
            .unwrap();

        assert!(
            result.fit.decay_parameter
                > 0.99
        );

        assert!(
            result
                .warnings
                .iter()
                .any(|warning| matches!(
                    warning,
                    RandomizedBenchmarkingWarning::
                        DecayNearIdentity
                ))
        );
    }

    #[test]
    fn dimension_for_one_qubit_is_two() {
        assert_eq!(
            checked_dimension(1).unwrap(),
            2
        );
    }

    #[test]
    fn epc_formula_for_single_qubit_is_half_one_minus_p() {
        let p = 0.98;

        let epc =
            (2.0 - 1.0)
                / 2.0
                * (1.0 - p);

        assert!(
            (epc - 0.01).abs()
                < 1.0e-12
        );
    }

    #[test]
    fn warning_identifiers_are_stable() {
        assert_eq!(
            RandomizedBenchmarkingWarning::
                ModelAssumptionRequired
                .as_str(),
            "model_assumption_required"
        );

        assert_eq!(
            RandomizedBenchmarkingWarning::
                PoorFitQuality
                .as_str(),
            "poor_fit_quality"
        );
    }
}