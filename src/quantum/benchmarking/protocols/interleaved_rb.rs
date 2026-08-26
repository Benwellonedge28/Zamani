//! Zamani Quantum Benchmarking — Interleaved Randomized Benchmarking
//!
//! Production interleaved randomized benchmarking (IRB) protocol.
//!
//! # Purpose
//!
//! Interleaved randomized benchmarking estimates the error associated with a
//! selected target operation by comparing:
//!
//! 1. a reference randomized-benchmarking experiment; and
//! 2. an interleaved experiment in which the target operation is inserted
//!    between the random reference operations.
//!
//! The protocol uses the reference and interleaved decay parameters to form
//! an IRB target-error estimate.
//!
//! # Architectural boundary
//!
//! This file owns:
//!
//! - IRB configuration;
//! - validation of IRB-specific configuration;
//! - experiment/sequence planning;
//! - target-operation placement semantics;
//! - reference/interleaved observation representation;
//! - observation validation;
//! - aggregation of observations by sequence length;
//! - IRB decay analysis;
//! - target-error estimation;
//! - uncertainty propagation;
//! - systematic-bound calculation;
//! - fit-quality diagnostics;
//! - protocol-level warnings;
//! - deterministic experiment identities;
//! - offline re-analysis.
//!
//! This file does NOT own:
//!
//! - Quantum IR semantics;
//! - Clifford-group mathematics;
//! - physical gate implementation;
//! - hardware communication;
//! - backend selection;
//! - routing;
//! - scheduling;
//! - calibration;
//! - generic execution;
//! - generic result serialization;
//! - generic reporting.
//!
//! Those responsibilities belong to their owning modules.
//!
//! # Dependency direction
//!
//! ```text
//! generators::clifford
//!          │
//!          ▼
//! interleaved_rb
//!          │
//!     ┌────┴───────────┐
//!     ▼                ▼
//! execution        statistics
//!     │                │
//!     ▼                ▼
//! Quantum IR       regression
//!     │
//!     ▼
//! routing / scheduling / hardware / runtime
//! ```
//!
//! The dependency direction must never be reversed.
//!
//! In particular, this module must not make the canonical Quantum IR depend
//! on benchmarking.
//!
//! # Scientific model
//!
//! The reference and interleaved experiments are modeled independently as:
//!
//! ```text
//! P_ref(m)   = A_ref   * p_ref^m   + B_ref
//! P_int(m)   = A_int   * p_int^m   + B_int
//! ```
//!
//! where:
//!
//! - `m` is the number of reference Clifford operations;
//! - `p_ref` is the reference decay parameter;
//! - `p_int` is the interleaved decay parameter.
//!
//! Under the standard IRB assumptions, the target operation's
//! error-per-operation estimate is:
//!
//! ```text
//! r_target = (d - 1) / d * (1 - p_int / p_ref)
//! ```
//!
//! where `d` is the Hilbert-space dimension.
//!
//! For one qubit:
//!
//! ```text
//! d = 2
//!
//! r_target = 1/2 * (1 - p_int / p_ref)
//! ```
//!
//! This quantity is an IRB decay-derived error estimate. It must NOT be
//! represented as an assumption-free physical gate infidelity.
//!
//! # Important scientific limitations
//!
//! IRB is model dependent.
//!
//! In particular:
//!
//! - gate-dependent noise can affect interpretation;
//! - coherent errors can produce misleading decay relationships;
//! - non-Markovian noise can invalidate simple exponential interpretation;
//! - leakage can violate the computational-subspace model;
//! - time-dependent drift can make reference and interleaved experiments
//!   non-comparable;
//! - imperfect implementation of the interleaved operation affects the
//!   measured decay;
//! - the target operation must be inserted consistently;
//! - the reference and interleaved experiments must be sufficiently matched.
//!
//! Therefore this implementation reports diagnostics and assumptions rather
//! than presenting the target-error estimate as an exact physical quantity.
//!
//! # Sequence semantics
//!
//! For a reference sequence of random operations:
//!
//! ```text
//! C1, C2, ..., Cm, R
//! ```
//!
//! the recovery operation `R` is selected so that the ideal logical action is
//! identity.
//!
//! The corresponding interleaved sequence is:
//!
//! ```text
//! C1, G, C2, G, ..., Cm, G, R'
//! ```
//!
//! where `G` is the selected target operation.
//!
//! The recovery `R'` is computed for the complete logical sequence by the
//! Clifford/operation generator. This file does not implement group
//! multiplication or inversion.
//!
//! # Critical implementation rule
//!
//! The target operation must NOT be folded into the reference Clifford count.
//!
//! If a reference experiment has length `m`, the interleaved experiment has
//! `m` target-operation applications in addition to the `m` reference
//! operations, followed by its recovery.
//!
//! The sequence length used in the exponential model remains `m`.
//!
//! This convention is essential for interpreting `p_int / p_ref`.
//!
//! # Production safety
//!
//! The protocol rejects:
//!
//! - zero qubit dimensions;
//! - invalid Hilbert-space dimensions;
//! - empty sequence-length sets;
//! - zero sequence lengths;
//! - duplicate sequence lengths;
//! - zero repetitions;
//! - zero shots;
//! - excessive workload sizes;
//! - arithmetic overflow;
//! - invalid confidence levels;
//! - non-finite observations;
//! - invalid probabilities;
//! - invalid decay parameters;
//! - non-positive reference decay parameters when a ratio is requested;
//! - invalid target-error estimates;
//! - incompatible reference/interleaved observations.
//!
//! No process-global RNG is used.
//!
//! No I/O is performed.
//!
//! No diagnostics are printed.
//!
//! # Reproducibility
//!
//! Experiment planning is deterministic given:
//!
//! - configuration;
//! - seed;
//! - target-operation identity;
//! - random sequence identifiers.
//!
//! The actual Clifford generation remains owned by
//! `generators::clifford`.
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
//! The intended future integration is:
//!
//! ```text
//! core::config
//!       │
//!       ▼
//! interleaved_rb
//!       │
//!       ├── generators::clifford
//!       ├── execution
//!       ├── statistics::regression
//!       ├── statistics::confidence
//!       ├── metrics::gate_error
//!       └── core::result
//! ```
//!
//! This module intentionally defines an adapter-neutral observation and
//! sequence-plan contract so that it does not need to be rewritten when those
//! modules are integrated.
//!
//! # Relation to standard randomized benchmarking
//!
//! This module does not duplicate Clifford-group mathematics from
//! `randomized_benchmarking.rs` or `generators::clifford`.
//!
//! The sequence planner stores logical operation identifiers. A generator/
//! lowering adapter is responsible for converting those identifiers into the
//! canonical Quantum IR.
//!
//! # Statistical policy
//!
//! The protocol performs the following stages:
//!
//! ```text
//! observations
//!     │
//!     ▼
//! validation
//!     │
//!     ▼
//! aggregation
//!     │
//!     ▼
//! reference decay fit
//!     │
//!     ▼
//! interleaved decay fit
//!     │
//!     ▼
//! target-error estimate
//!     │
//!     ▼
//! uncertainty / bounds
//!     │
//!     ▼
//! scientific diagnostics
//! ```
//!
//! A numerical fit alone is never treated as proof that the physical IRB
//! assumptions hold.
//!
//! # No silent fallback
//!
//! If the reference decay is zero, negative, non-finite, or otherwise
//! unsuitable for the IRB ratio, this implementation returns an explicit
//! error.
//!
//! It does not clamp the reference decay to a small positive value.
//!
//! Likewise, an estimate outside the physically interpretable interval is not
//! silently clamped.
//!
//! # Versioning
//!
//! The protocol schema version, algorithm identifier, and plan fingerprint are
//! separate from the Zamani compiler version.
//!
//! A scientific algorithm change that can change results requires an algorithm
//! identifier change.
//! ============================================================================

use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;

// =============================================================================
// Public identifiers and constants
// =============================================================================

/// Stable protocol identifier.
pub const INTERLEAVED_RANDOMIZED_BENCHMARKING_ID: &str =
    "interleaved_randomized_benchmarking";

/// Stable protocol schema version.
pub const INTERLEAVED_RANDOMIZED_BENCHMARKING_SCHEMA_VERSION: u32 = 1;

/// Stable algorithm identifier.
///
/// Change this identifier whenever sequence semantics, estimator mathematics,
/// uncertainty calculation, or other scientifically meaningful behaviour
/// changes.
pub const INTERLEAVED_RANDOMIZED_BENCHMARKING_ALGORITHM_ID: &str =
    "zamani.irb.clifford.target.v1";

/// Default number of independent random sequences for each sequence length.
pub const DEFAULT_SEQUENCES_PER_LENGTH: usize = 32;

/// Default shots per sequence.
pub const DEFAULT_SHOTS_PER_SEQUENCE: u64 = 1_000;

/// Default confidence level.
pub const DEFAULT_CONFIDENCE_LEVEL: f64 = 0.95;

/// Default maximum sequence length.
pub const DEFAULT_MAX_SEQUENCE_LENGTH: usize = 100_000;

/// Default maximum number of sequence lengths.
pub const DEFAULT_MAX_SEQUENCE_LENGTHS: usize = 256;

/// Default maximum sequences per length.
pub const DEFAULT_MAX_SEQUENCES_PER_LENGTH: usize = 100_000;

/// Default maximum shots per sequence.
pub const DEFAULT_MAX_SHOTS_PER_SEQUENCE: u64 = 10_000_000;

/// Default maximum total shots.
///
/// This applies to both reference and interleaved experiments together.
pub const DEFAULT_MAX_TOTAL_SHOTS: u128 = 1_000_000_000;

/// Default minimum number of distinct sequence lengths required for an
/// exponential model with amplitude, decay and offset.
pub const MIN_SEQUENCE_LENGTHS_FOR_EXPONENTIAL_FIT: usize = 4;

/// Default minimum physically useful reference decay.
pub const MIN_REFERENCE_DECAY_PARAMETER: f64 = 1.0e-12;

/// Numerical tolerance for values that should lie in [0, 1].
const UNIT_INTERVAL_EPSILON: f64 = 1.0e-12;

/// Numerical tolerance for a ratio that should be close to a physical
/// probability/error interval.
const PHYSICAL_RATIO_EPSILON: f64 = 1.0e-10;

/// Maximum number of sequence identifiers generated by one plan.
const MAX_PLAN_SEQUENCE_IDS: usize = 10_000_000;

// =============================================================================
// Error type
// =============================================================================

/// Errors produced by the IRB protocol.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum InterleavedRandomizedBenchmarkingError {
    /// Zero or otherwise invalid number of qubits.
    InvalidQubitCount {
        /// Requested qubit count.
        requested: usize,
    },

    /// Hilbert-space dimension is invalid.
    InvalidDimension {
        /// Requested dimension.
        dimension: u128,
    },

    /// No sequence lengths were provided.
    EmptySequenceLengths,

    /// A sequence length is zero.
    InvalidSequenceLength {
        /// Invalid sequence length.
        length: usize,
    },

    /// Too many sequence lengths were supplied.
    TooManySequenceLengths {
        /// Requested number.
        requested: usize,

        /// Maximum allowed.
        maximum: usize,
    },

    /// Duplicate sequence length.
    DuplicateSequenceLength {
        /// Duplicated length.
        length: usize,
    },

    /// No repetitions were requested.
    InvalidSequencesPerLength {
        /// Requested repetitions.
        requested: usize,
    },

    /// Too many repetitions were requested.
    TooManySequencesPerLength {
        /// Requested repetitions.
        requested: usize,

        /// Maximum allowed.
        maximum: usize,
    },

    /// Zero shots were requested.
    InvalidShots,

    /// Shot count exceeds the configured maximum.
    TooManyShots {
        /// Requested shots.
        requested: u64,

        /// Maximum shots.
        maximum: u64,
    },

    /// Total workload exceeds configured budget.
    TotalShotLimitExceeded {
        /// Requested total.
        requested: u128,

        /// Maximum total.
        maximum: u128,
    },

    /// Resource calculation overflowed.
    ResourceCalculationOverflow,

    /// Invalid confidence level.
    InvalidConfidenceLevel {
        /// Invalid level.
        value: f64,
    },

    /// Invalid seed.
    InvalidSeed,

    /// Observation contains an invalid count.
    InvalidObservation {
        /// Sequence length.
        sequence_length: usize,

        /// Successful outcomes.
        successes: u64,

        /// Total shots.
        shots: u64,
    },

    /// Observation probability is non-finite.
    NonFiniteObservation {
        /// Sequence length.
        sequence_length: usize,
    },

    /// Observation probability is outside [0, 1].
    InvalidProbability {
        /// Sequence length.
        sequence_length: usize,

        /// Probability.
        probability: f64,
    },

    /// No observations were supplied.
    EmptyObservations,

    /// Reference and interleaved observations have different sequence-length
    /// sets.
    ObservationLengthMismatch,

    /// A sequence length is absent from one side.
    MissingSequenceLength {
        /// Missing length.
        sequence_length: usize,
    },

    /// Insufficient distinct sequence lengths exist for exponential fitting.
    InsufficientSequenceLengths {
        /// Number available.
        supplied: usize,

        /// Minimum required.
        minimum: usize,
    },

    /// A supplied decay parameter is invalid.
    InvalidDecayParameter {
        /// Parameter value.
        value: f64,

        /// Parameter name.
        parameter: &'static str,
    },

    /// The reference decay is too close to zero for a stable ratio.
    ReferenceDecayTooSmall {
        /// Reference decay.
        value: f64,
    },

    /// The target-error ratio cannot be interpreted physically.
    InvalidDecayRatio {
        /// Interleaved decay.
        interleaved: f64,

        /// Reference decay.
        reference: f64,

        /// Ratio.
        ratio: f64,
    },

    /// The target-error estimate is outside its physically interpretable
    /// interval.
    InvalidTargetErrorEstimate {
        /// Estimated target error.
        value: f64,
    },

    /// Uncertainty could not be calculated.
    InvalidUncertainty,

    /// Numerical calculation failed.
    NumericalFailure {
        /// Operation that failed.
        operation: &'static str,
    },

    /// Configuration is internally inconsistent.
    InvalidConfiguration {
        /// Human-readable reason.
        reason: &'static str,
    },
}

impl fmt::Display for InterleavedRandomizedBenchmarkingError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidQubitCount { requested } => write!(
                formatter,
                "interleaved randomized benchmarking requires at least \
                 one qubit; requested {requested}"
            ),

            Self::InvalidDimension { dimension } => write!(
                formatter,
                "IRB Hilbert-space dimension must be a positive power of two; \
                 got {dimension}"
            ),

            Self::EmptySequenceLengths => write!(
                formatter,
                "interleaved randomized benchmarking requires at least one \
                 sequence length"
            ),

            Self::InvalidSequenceLength { length } => write!(
                formatter,
                "IRB sequence length must be greater than zero; got {length}"
            ),

            Self::TooManySequenceLengths {
                requested,
                maximum,
            } => write!(
                formatter,
                "IRB requested {requested} sequence lengths; maximum is \
                 {maximum}"
            ),

            Self::DuplicateSequenceLength { length } => write!(
                formatter,
                "IRB sequence length {length} occurs more than once"
            ),

            Self::InvalidSequencesPerLength { requested } => write!(
                formatter,
                "IRB requires at least one sequence per length; got {requested}"
            ),

            Self::TooManySequencesPerLength {
                requested,
                maximum,
            } => write!(
                formatter,
                "IRB requested {requested} sequences per length; maximum is \
                 {maximum}"
            ),

            Self::InvalidShots => write!(
                formatter,
                "IRB requires at least one shot per sequence"
            ),

            Self::TooManyShots {
                requested,
                maximum,
            } => write!(
                formatter,
                "IRB requested {requested} shots per sequence; maximum is \
                 {maximum}"
            ),

            Self::TotalShotLimitExceeded {
                requested,
                maximum,
            } => write!(
                formatter,
                "IRB requested {requested} total shots; maximum is {maximum}"
            ),

            Self::ResourceCalculationOverflow => write!(
                formatter,
                "IRB workload-size calculation overflowed"
            ),

            Self::InvalidConfidenceLevel { value } => write!(
                formatter,
                "IRB confidence level must be finite and in (0, 1); got {value}"
            ),

            Self::InvalidSeed => write!(
                formatter,
                "IRB seed configuration is invalid"
            ),

            Self::InvalidObservation {
                sequence_length,
                successes,
                shots,
            } => write!(
                formatter,
                "invalid IRB observation at sequence length {sequence_length}: \
                 successes={successes}, shots={shots}"
            ),

            Self::NonFiniteObservation { sequence_length } => write!(
                formatter,
                "IRB observation at sequence length {sequence_length} is \
                 non-finite"
            ),

            Self::InvalidProbability {
                sequence_length,
                probability,
            } => write!(
                formatter,
                "IRB probability at sequence length {sequence_length} must be \
                 in [0, 1]; got {probability}"
            ),

            Self::EmptyObservations => write!(
                formatter,
                "IRB analysis requires at least one observation"
            ),

            Self::ObservationLengthMismatch => write!(
                formatter,
                "reference and interleaved IRB observations use different \
                 sequence-length sets"
            ),

            Self::MissingSequenceLength { sequence_length } => write!(
                formatter,
                "IRB observation is missing sequence length {sequence_length}"
            ),

            Self::InsufficientSequenceLengths { supplied, minimum } => write!(
                formatter,
                "IRB exponential fitting requires at least {minimum} distinct \
                 sequence lengths; got {supplied}"
            ),

            Self::InvalidDecayParameter { value, parameter } => write!(
                formatter,
                "IRB {parameter} decay parameter must be finite and in (0, 1]; \
                 got {value}"
            ),

            Self::ReferenceDecayTooSmall { value } => write!(
                formatter,
                "IRB reference decay parameter {value} is too close to zero \
                 for a stable interleaved/reference ratio"
            ),

            Self::InvalidDecayRatio {
                interleaved,
                reference,
                ratio,
            } => write!(
                formatter,
                "invalid IRB decay ratio: interleaved={interleaved}, \
                 reference={reference}, ratio={ratio}"
            ),

            Self::InvalidTargetErrorEstimate { value } => write!(
                formatter,
                "IRB target error estimate must be finite and in [0, 1]; \
                 got {value}"
            ),

            Self::InvalidUncertainty => write!(
                formatter,
                "IRB uncertainty calculation produced an invalid result"
            ),

            Self::NumericalFailure { operation } => write!(
                formatter,
                "IRB numerical calculation failed during {operation}"
            ),

            Self::InvalidConfiguration { reason } => write!(
                formatter,
                "invalid IRB configuration: {reason}"
            ),
        }
    }
}

impl Error for InterleavedRandomizedBenchmarkingError {}

/// Result type for this protocol.
pub type InterleavedRandomizedBenchmarkingResult<T> =
    Result<T, InterleavedRandomizedBenchmarkingError>;

// =============================================================================
// Target operation identity
// =============================================================================

/// Stable identity of the operation being characterized.
///
/// The actual operation is deliberately not represented here. The Clifford/
— generator and Quantum IR layers own the operation itself.
///
/// A stable identity is required so that provenance can state exactly which
/// target operation was interleaved.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TargetOperation {
    /// Stable caller-defined operation identifier.
    pub id: String,

    /// Optional semantic family, for example `clifford`, `native_gate`,
    /// `pulse`, or another backend-defined category.
    pub family: String,

    /// Optional version supplied by the operation provider.
    pub version: Option<String>,
}

impl TargetOperation {
    /// Creates a target-operation identity.
    pub fn new(
        id: impl Into<String>,
        family: impl Into<String>,
    ) -> InterleavedRandomizedBenchmarkingResult<Self> {
        let id = id.into();
        let family = family.into();

        if id.trim().is_empty() {
            return Err(
                InterleavedRandomizedBenchmarkingError::InvalidConfiguration {
                    reason: "target operation id must not be empty",
                },
            );
        }

        if family.trim().is_empty() {
            return Err(
                InterleavedRandomizedBenchmarkingError::InvalidConfiguration {
                    reason: "target operation family must not be empty",
                },
            );
        }

        Ok(Self {
            id,
            family,
            version: None,
        })
    }

    /// Adds an operation-provider version.
    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }
}

// =============================================================================
// Configuration
// =============================================================================

/// Complete IRB protocol configuration.
///
/// This structure intentionally contains protocol-level settings only.
/// Generic benchmark settings can later be mapped into this structure.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InterleavedRandomizedBenchmarkingConfig {
    /// Number of qubits in the characterized system.
    pub num_qubits: usize,

    /// Sequence lengths measured in reference Clifford operations.
    pub sequence_lengths: Vec<usize>,

    /// Number of independently randomized sequences per length.
    pub sequences_per_length: usize,

    /// Number of measurement shots per sequence.
    pub shots_per_sequence: u64,

    /// Target operation being characterized.
    pub target_operation: TargetOperation,

    /// Deterministic experiment seed.
    pub seed: u64,

    /// Confidence level for uncertainty reporting.
    pub confidence_level: f64,

    /// Maximum allowed sequence length.
    pub max_sequence_length: usize,

    /// Maximum allowed number of sequence lengths.
    pub max_sequence_lengths: usize,

    /// Maximum allowed sequences per length.
    pub max_sequences_per_length: usize,

    /// Maximum allowed shots per sequence.
    pub max_shots_per_sequence: u64,

    /// Maximum total shots for reference plus interleaved experiments.
    pub max_total_shots: u128,
}

impl Default for InterleavedRandomizedBenchmarkingConfig {
    fn default() -> Self {
        Self {
            num_qubits: 1,
            sequence_lengths: vec![1, 2, 4, 8, 16, 32],
            sequences_per_length: DEFAULT_SEQUENCES_PER_LENGTH,
            shots_per_sequence: DEFAULT_SHOTS_PER_SEQUENCE,
            target_operation: TargetOperation {
                id: "target".to_owned(),
                family: "operation".to_owned(),
                version: None,
            },
            seed: 0,
            confidence_level: DEFAULT_CONFIDENCE_LEVEL,
            max_sequence_length: DEFAULT_MAX_SEQUENCE_LENGTH,
            max_sequence_lengths: DEFAULT_MAX_SEQUENCE_LENGTHS,
            max_sequences_per_length: DEFAULT_MAX_SEQUENCES_PER_LENGTH,
            max_shots_per_sequence: DEFAULT_MAX_SHOTS_PER_SEQUENCE,
            max_total_shots: DEFAULT_MAX_TOTAL_SHOTS,
        }
    }
}

impl InterleavedRandomizedBenchmarkingConfig {
    /// Creates a configuration using production defaults.
    pub fn new(
        num_qubits: usize,
        sequence_lengths: Vec<usize>,
        target_operation: TargetOperation,
        seed: u64,
    ) -> InterleavedRandomizedBenchmarkingResult<Self> {
        let configuration = Self {
            num_qubits,
            sequence_lengths,
            target_operation,
            seed,
            ..Self::default()
        };

        configuration.validate()
    }

    /// Validates the complete configuration.
    pub fn validate(
        &self,
    ) -> InterleavedRandomizedBenchmarkingResult<Self> {
        if self.num_qubits == 0 {
            return Err(
                InterleavedRandomizedBenchmarkingError::InvalidQubitCount {
                    requested: self.num_qubits,
                },
            );
        }

        if self.sequence_lengths.is_empty() {
            return Err(
                InterleavedRandomizedBenchmarkingError::EmptySequenceLengths,
            );
        }

        if self.sequence_lengths.len() > self.max_sequence_lengths {
            return Err(
                InterleavedRandomBenchmarkingError::TooManySequenceLengths {
                    requested: self.sequence_lengths.len(),
                    maximum: self.max_sequence_lengths,
                },
            );
        }

        for &length in &self.sequence_lengths {
            if length == 0 {
                return Err(
                    InterleavedRandomizedBenchmarkingError::InvalidSequenceLength {
                        length,
                    },
                );
            }

            if length > self.max_sequence_length {
                return Err(
                    InterleavedRandomizedBenchmarkingError::TooManySequenceLengths {
                        requested: length,
                        maximum: self.max_sequence_length,
                    },
                );
            }
        }

        for index in 0..self.sequence_lengths.len() {
            for other in (index + 1)..self.sequence_lengths.len() {
                if self.sequence_lengths[index] == self.sequence_lengths[other] {
                    return Err(
                        InterleavedRandomizedBenchmarkingError::DuplicateSequenceLength {
                            length: self.sequence_lengths[index],
                        },
                    );
                }
            }
        }

        if self.sequences_per_length == 0 {
            return Err(
                InterleavedRandomizedBenchmarkingError::InvalidSequencesPerLength {
                    requested: 0,
                },
            );
        }

        if self.sequences_per_length > self.max_sequences_per_length {
            return Err(
                InterleavedRandomizedBenchmarkingError::TooManySequencesPerLength {
                    requested: self.sequences_per_length,
                    maximum: self.max_sequences_per_length,
                },
            );
        }

        if self.shots_per_sequence == 0 {
            return Err(
                InterleavedRandomizedBenchmarkingError::InvalidShots,
            );
        }

        if self.shots_per_sequence > self.max_shots_per_sequence {
            return Err(
                InterleavedRandomizedBenchmarkingError::TooManyShots {
                    requested: self.shots_per_sequence,
                    maximum: self.max_shots_per_sequence,
                },
            );
        }

        if !self.confidence_level.is_finite()
            || self.confidence_level <= 0.0
            || self.confidence_level >= 1.0
        {
            return Err(
                InterleavedRandomizedBenchmarkingError::InvalidConfidenceLevel {
                    value: self.confidence_level,
                },
            );
        }

        if self.target_operation.id.trim().is_empty() {
            return Err(
                InterleavedRandomizedBenchmarkingError::InvalidConfiguration {
                    reason: "target operation id must not be empty",
                },
            );
        }

        if self.target_operation.family.trim().is_empty() {
            return Err(
                InterleavedRandomizedBenchmarkingError::InvalidConfiguration {
                    reason: "target operation family must not be empty",
                },
            );
        }

        let total_per_side = checked_mul_u128(
            self.sequence_lengths.len() as u128,
            self.sequences_per_length as u128,
        )?;

        let total_per_side = checked_mul_u128(
            total_per_side,
            self.shots_per_sequence as u128,
        )?;

        let total_both = checked_mul_u128(total_per_side, 2)?;

        if total_both > self.max_total_shots {
            return Err(
                InterleavedRandomizedBenchmarkingError::TotalShotLimitExceeded {
                    requested: total_both,
                    maximum: self.max_total_shots,
                },
            );
        }

        Ok(self.clone())
    }

    /// Returns the Hilbert-space dimension `2^n`.
    pub fn dimension(
        &self,
    ) -> InterleavedRandomizedBenchmarkingResult<u128> {
        let shift = self.num_qubits;

        if shift >= 128 {
            return Err(
                InterleavedRandomizedBenchmarkingError::InvalidDimension {
                    dimension: u128::MAX,
                },
            );
        }

        let dimension = 1u128
            .checked_shl(shift as u32)
            .ok_or(
                InterleavedRandomBenchmarkingError::InvalidDimension {
                    dimension: u128::MAX,
                },
            )?;

        if dimension == 0 {
            return Err(
                InterleavedRandomBenchmarkingError::InvalidDimension {
                    dimension,
                },
            );
        }

        Ok(dimension)
    }

    /// Returns the total number of shots across both reference and
    /// interleaved experiments.
    pub fn total_shots(
        &self,
    ) -> InterleavedRandomizedBenchmarkingResult<u128> {
        self.validate()?;

        let sequence_count = checked_mul_u128(
            self.sequence_lengths.len() as u128,
            self.sequences_per_length as u128,
        )?;

        let shots_per_side = checked_mul_u128(
            sequence_count,
            self.shots_per_sequence as u128,
        )?;

        checked_mul_u128(shots_per_side, 2)
    }
}

// =============================================================================
// Sequence planning
// =============================================================================

/// Kind of IRB experiment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExperimentKind {
    /// Standard reference RB sequence.
    Reference,

    /// Target-interleaved RB sequence.
    Interleaved,
}

impl ExperimentKind {
    /// Stable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Reference => "reference",
            Self::Interleaved => "interleaved",
        }
    }
}

/// A deterministic identifier for a random sequence.
///
/// The actual Clifford sequence is generated by the Clifford generator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SequenceId {
    /// Sequence length.
    pub length: usize,

    /// Repetition index at that length.
    pub repetition: usize,

    /// Experiment kind.
    pub kind: ExperimentKind,

    /// Deterministic derived seed.
    pub seed: u64,
}

/// Logical IRB sequence plan.
///
/// It deliberately contains no Quantum IR and no physical gate representation.
///
/// A generator adapter consumes this plan and constructs the actual:
///
/// ```text
/// C1, G, C2, G, ..., Cm, G, recovery
/// ```
///
/// or:
///
/// ```text
/// C1, C2, ..., Cm, recovery
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InterleavedSequencePlan {
    /// Stable sequence identity.
    pub id: SequenceId,

    /// Number of reference Clifford operations.
    pub reference_length: usize,

    /// Whether the target operation is inserted.
    pub experiment_kind: ExperimentKind,

    /// Target-operation identity.
    pub target_operation: TargetOperation,
}

impl InterleavedSequencePlan {
    /// Returns the number of target-operation applications required.
    pub const fn target_operation_count(&self) -> usize {
        match self.experiment_kind {
            ExperimentKind::Reference => 0,
            ExperimentKind::Interleaved => self.reference_length,
        }
    }

    /// Returns the logical number of reference Clifford operations.
    pub const fn reference_operation_count(&self) -> usize {
        self.reference_length
    }

    /// Returns whether the sequence is interleaved.
    pub const fn is_interleaved(&self) -> bool {
        matches!(self.experiment_kind, ExperimentKind::Interleaved)
    }
}

/// Complete deterministic IRB experiment plan.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InterleavedExperimentPlan {
    /// Protocol identifier.
    pub protocol_id: String,

    /// Protocol schema version.
    pub schema_version: u32,

    /// Algorithm identifier.
    pub algorithm_id: String,

    /// Target operation.
    pub target_operation: TargetOperation,

    /// Configuration seed.
    pub seed: u64,

    /// Reference plans.
    pub reference: Vec<InterleavedSequencePlan>,

    /// Interleaved plans.
    pub interleaved: Vec<InterleavedSequencePlan>,
}

impl InterleavedExperimentPlan {
    /// Creates a deterministic plan from a validated configuration.
    pub fn from_config(
        config: &InterleavedRandomizedBenchmarkingConfig,
    ) -> InterleavedRandomizedBenchmarkingResult<Self> {
        config.validate()?;

        let sequence_count_per_side = checked_mul_usize(
            config.sequence_lengths.len(),
            config.sequences_per_length,
        )?;

        if sequence_count_per_side > MAX_PLAN_SEQUENCE_IDS {
            return Err(
                InterleavedRandomizedBenchmarkingError::TooManySequencesPerLength {
                    requested: sequence_count_per_side,
                    maximum: MAX_PLAN_SEQUENCE_IDS,
                },
            );
        }

        let mut reference =
            Vec::with_capacity(sequence_count_per_side);

        let mut interleaved =
            Vec::with_capacity(sequence_count_per_side);

        for &length in &config.sequence_lengths {
            for repetition in 0..config.sequences_per_length {
                let derived_seed = derive_sequence_seed(
                    config.seed,
                    length,
                    repetition,
                );

                let sequence_id = SequenceId {
                    length,
                    repetition,
                    kind: ExperimentKind::Reference,
                    seed: derived_seed,
                };

                reference.push(InterleavedSequencePlan {
                    id: sequence_id,
                    reference_length: length,
                    experiment_kind: ExperimentKind::Reference,
                    target_operation: config.target_operation.clone(),
                });

                let interleaved_id = SequenceId {
                    length,
                    repetition,
                    kind: ExperimentKind::Interleaved,
                    seed: derive_interleaved_seed(derived_seed),
                };

                interleaved.push(InterleavedSequencePlan {
                    id: interleaved_id,
                    reference_length: length,
                    experiment_kind: ExperimentKind::Interleaved,
                    target_operation: config.target_operation.clone(),
                });
            }
        }

        Ok(Self {
            protocol_id:
                INTERLEAVED_RANDOMIZED_BENCHMARKING_ID.to_owned(),
            schema_version:
                INTERLEAVED_RANDOMIZED_BENCHMARKING_SCHEMA_VERSION,
            algorithm_id:
                INTERLEAVED_RANDOMIZED_BENCHMARKING_ALGORITHM_ID.to_owned(),
            target_operation: config.target_operation.clone(),
            seed: config.seed,
            reference,
            interleaved,
        })
    }

    /// Returns all sequence plans in deterministic order.
    pub fn all_sequences(&self) -> Vec<&InterleavedSequencePlan> {
        let mut sequences =
            Vec::with_capacity(self.reference.len() + self.interleaved.len());

        sequences.extend(self.reference.iter());
        sequences.extend(self.interleaved.iter());

        sequences
    }

    /// Returns the number of reference sequences.
    pub fn reference_sequence_count(&self) -> usize {
        self.reference.len()
    }

    /// Returns the number of interleaved sequences.
    pub fn interleaved_sequence_count(&self) -> usize {
        self.interleaved.len()
    }
}

// =============================================================================
// Observations
// =============================================================================

/// One raw binomial survival observation.
///
/// The preferred source is an execution adapter that converts backend-specific
/// measurement results into this representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurvivalObservation {
    /// Reference Clifford sequence length.
    pub sequence_length: usize,

    /// Number of successful/surviving outcomes.
    pub successes: u64,

    /// Number of measurement shots.
    pub shots: u64,
}

impl SurvivalObservation {
    /// Creates a validated observation.
    pub fn new(
        sequence_length: usize,
        successes: u64,
        shots: u64,
    ) -> InterleavedRandomizedBenchmarkingResult<Self> {
        if sequence_length == 0 {
            return Err(
                InterleavedRandomizedBenchmarkingError::InvalidSequenceLength {
                    length: sequence_length,
                },
            );
        }

        if shots == 0 || successes > shots {
            return Err(
                InterleavedRandomizedBenchmarkingError::InvalidObservation {
                    sequence_length,
                    successes,
                    shots,
                },
            );
        }

        Ok(Self {
            sequence_length,
            successes,
            shots,
        })
    }

    /// Returns the empirical survival probability.
    pub fn probability(
        self,
    ) -> InterleavedRandomizedBenchmarkingResult<f64> {
        let probability = self.successes as f64 / self.shots as f64;

        if !probability.is_finite() {
            return Err(
                InterleavedRandomizedBenchmarkingError::NonFiniteObservation {
                    sequence_length: self.sequence_length,
                },
            );
        }

        if probability < -UNIT_INTERVAL_EPSILON
            || probability > 1.0 + UNIT_INTERVAL_EPSILON
        {
            return Err(
                InterleavedRandomizedBenchmarkingError::InvalidProbability {
                    sequence_length: self.sequence_length,
                    probability,
                },
            );
        }

        Ok(probability.clamp(0.0, 1.0))
    }
}

/// Aggregated observations at one sequence length.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct AggregatedObservation {
    /// Sequence length.
    pub sequence_length: usize,

    /// Pooled successful outcomes.
    pub successes: u64,

    /// Pooled shots.
    pub shots: u64,

    /// Empirical survival probability.
    pub probability: f64,
}

impl AggregatedObservation {
    fn from_observations(
        observations: &[SurvivalObservation],
        sequence_length: usize,
    ) -> InterleavedRandomizedBenchmarkingResult<Self> {
        let mut successes = 0u64;
        let mut shots = 0u64;
        let mut found = false;

        for observation in observations {
            if observation.sequence_length != sequence_length {
                continue;
            }

            found = true;

            successes = successes.checked_add(observation.successes).ok_or(
                InterleavedRandomizedBenchmarkingError::NumericalFailure {
                    operation: "aggregating survival successes",
                },
            )?;

            shots = shots.checked_add(observation.shots).ok_or(
                InterleavedRandomizedBenchmarkingError::NumericalFailure {
                    operation: "aggregating survival shots",
                },
            )?;
        }

        if !found || shots == 0 || successes > shots {
            return Err(
                InterleavedRandomizedBenchmarkingError::InvalidObservation {
                    sequence_length,
                    successes,
                    shots,
                },
            );
        }

        let probability = successes as f64 / shots as f64;

        if !probability.is_finite() {
            return Err(
                InterleavedRandomizedBenchmarkingError::NonFiniteObservation {
                    sequence_length,
                },
            );
        }

        Ok(Self {
            sequence_length,
            successes,
            shots,
            probability: probability.clamp(0.0, 1.0),
        })
    }
}

// =============================================================================
// Decay fit
// =============================================================================

/// Declared fit quality classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FitQuality {
    /// Fit passed numerical and residual checks.
    Good,

    /// Fit is usable but diagnostics indicate caution.
    Acceptable,

    /// Fit should not be interpreted as a reliable IRB decay.
    Poor,
}

impl FitQuality {
    /// Stable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Good => "good",
            Self::Acceptable => "acceptable",
            Self::Poor => "poor",
        }
    }
}

/// Exponential-decay fit.
///
/// The fit model is:
///
/// ```text
/// y(m) = amplitude * p^m + offset
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct DecayFit {
    /// Fitted amplitude.
    pub amplitude: f64,

    /// Fitted decay parameter.
    pub decay_parameter: f64,

    /// Fitted asymptotic offset.
    pub offset: f64,

    /// Root mean square error.
    pub rmse: f64,

    /// Coefficient of determination.
    pub r_squared: f64,

    /// Number of observations used.
    pub observations: usize,

    /// Quality classification.
    pub quality: FitQuality,
}

impl DecayFit {
    /// Constructs a validated fit.
    pub fn new(
        amplitude: f64,
        decay_parameter: f64,
        offset: f64,
        rmse: f64,
        r_squared: f64,
        observations: usize,
    ) -> InterleavedRandomizedBenchmarkingResult<Self> {
        if !amplitude.is_finite() {
            return Err(
                InterleavedRandomizedBenchmarkingError::NumericalFailure {
                    operation: "validating decay amplitude",
                },
            );
        }

        if !decay_parameter.is_finite()
            || decay_parameter <= 0.0
            || decay_parameter > 1.0 + PHYSICAL_RATIO_EPSILON
        {
            return Err(
                InterleavedRandomizedBenchmarkingError::InvalidDecayParameter {
                    value: decay_parameter,
                    parameter: "decay",
                },
            );
        }

        if !offset.is_finite() {
            return Err(
                InterleavedRandomizedBenchmarkingError::NumericalFailure {
                    operation: "validating decay offset",
                },
            );
        }

        if !rmse.is_finite() || rmse < 0.0 {
            return Err(
                InterleavedRandomizedBenchmarkingError::NumericalFailure {
                    operation: "validating decay RMSE",
                },
            );
        }

        if !r_squared.is_finite() {
            return Err(
                InterleavedRandomizedBenchmarkingError::NumericalFailure {
                    operation: "validating decay R-squared",
                },
            );
        }

        if observations < MIN_SEQUENCE_LENGTHS_FOR_EXPONENTIAL_FIT {
            return Err(
                InterleavedRandomizedBenchmarkingError::InsufficientSequenceLengths {
                    supplied: observations,
                    minimum: MIN_SEQUENCE_LENGTHS_FOR_EXPONENTIAL_FIT,
                },
            );
        }

        let quality = if r_squared >= 0.99 && rmse <= 0.05 {
            FitQuality::Good
        } else if r_squared >= 0.90 && rmse <= 0.10 {
            FitQuality::Acceptable
        } else {
            FitQuality::Poor
        };

        Ok(Self {
            amplitude,
            decay_parameter: decay_parameter.clamp(0.0, 1.0),
            offset,
            rmse,
            r_squared,
            observations,
            quality,
        })
    }
}

// =============================================================================
// Target-error uncertainty
// =============================================================================

/// Target-error estimate and its uncertainty.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct TargetErrorEstimate {
    /// Point estimate of target error per operation.
    pub value: f64,

    /// Lower uncertainty bound.
    pub lower: f64,

    /// Upper uncertainty bound.
    pub upper: f64,

    /// Confidence level.
    pub confidence_level: f64,

    /// Reference decay parameter.
    pub reference_decay: f64,

    /// Interleaved decay parameter.
    pub interleaved_decay: f64,

    /// Decay ratio `p_int / p_ref`.
    pub decay_ratio: f64,
}

impl TargetErrorEstimate {
    /// Calculates the target-error point estimate.
    ///
    /// ```text
    /// r = (d - 1) / d * (1 - p_int / p_ref)
    /// ```
    pub fn from_decay_parameters(
        reference_decay: f64,
        interleaved_decay: f64,
        dimension: u128,
        confidence_level: f64,
    ) -> InterleavedRandomizedBenchmarkingResult<Self> {
        validate_decay_parameter(reference_decay, "reference")?;
        validate_decay_parameter(interleaved_decay, "interleaved")?;

        if dimension < 2 {
            return Err(
                InterleavedRandomizedBenchmarkingError::InvalidDimension {
                    dimension,
                },
            );
        }

        if reference_decay < MIN_REFERENCE_DECAY_PARAMETER {
            return Err(
                InterleavedRandomizedBenchmarkingError::ReferenceDecayTooSmall {
                    value: reference_decay,
                },
            );
        }

        if !confidence_level.is_finite()
            || confidence_level <= 0.0
            || confidence_level >= 1.0
        {
            return Err(
                InterleavedRandomizedBenchmarkingError::InvalidConfidenceLevel {
                    value: confidence_level,
                },
            );
        }

        let ratio = interleaved_decay / reference_decay;

        if !ratio.is_finite()
            || ratio < -PHYSICAL_RATIO_EPSILON
            || ratio > 1.0 + PHYSICAL_RATIO_EPSILON
        {
            return Err(
                InterleavedRandomizedBenchmarkingError::InvalidDecayRatio {
                    interleaved: interleaved_decay,
                    reference: reference_decay,
                    ratio,
                },
            );
        }

        let d = dimension as f64;
        let prefactor = (d - 1.0) / d;

        let value = prefactor * (1.0 - ratio);

        if !value.is_finite()
            || value < -PHYSICAL_RATIO_EPSILON
            || value > 1.0 + PHYSICAL_RATIO_EPSILON
        {
            return Err(
                InterleavedRandomizedBenchmarkingError::InvalidTargetErrorEstimate {
                    value,
                },
            );
        }

        let value = value.clamp(0.0, 1.0);

        Ok(Self {
            value,
            lower: value,
            upper: value,
            confidence_level,
            reference_decay,
            interleaved_decay,
            decay_ratio: ratio.clamp(0.0, 1.0),
        })
    }

    /// Adds a conservative uncertainty envelope based on decay-parameter
    /// uncertainties.
    ///
    /// The caller supplies lower/upper uncertainty bounds for both decay
    /// parameters. The extrema are evaluated directly rather than relying on
    /// a first-order linear approximation.
    pub fn with_decay_bounds(
        self,
        reference_lower: f64,
        reference_upper: f64,
        interleaved_lower: f64,
        interleaved_upper: f64,
        dimension: u128,
    ) -> InterleavedRandomizedBenchmarkingResult<Self> {
        validate_decay_parameter(reference_lower, "reference lower")?;
        validate_decay_parameter(reference_upper, "reference upper")?;
        validate_decay_parameter(interleaved_lower, "interleaved lower")?;
        validate_decay_parameter(interleaved_upper, "interleaved upper")?;

        if reference_lower > reference_upper
            || interleaved_lower > interleaved_upper
        {
            return Err(
                InterleavedRandomizedBenchmarkingError::InvalidUncertainty,
            );
        }

        let candidates = [
            target_error_from_ratio(
                reference_lower,
                interleaved_lower,
                dimension,
            )?,
            target_error_from_ratio(
                reference_lower,
                interleaved_upper,
                dimension,
            )?,
            target_error_from_ratio(
                reference_upper,
                interleaved_lower,
                dimension,
            )?,
            target_error_from_ratio(
                reference_upper,
                interleaved_upper,
                dimension,
            )?,
        ];

        let mut lower = f64::INFINITY;
        let mut upper = f64::NEG_INFINITY;

        for value in candidates {
            lower = lower.min(value);
            upper = upper.max(value);
        }

        if !lower.is_finite()
            || !upper.is_finite()
            || lower > upper
            || lower < -PHYSICAL_RATIO_EPSILON
            || upper > 1.0 + PHYSICAL_RATIO_EPSILON
        {
            return Err(
                InterleavedRandomizedBenchmarkingError::InvalidUncertainty,
            );
        }

        Ok(Self {
            lower: lower.clamp(0.0, 1.0),
            upper: upper.clamp(0.0, 1.0),
            ..self
        })
    }

    /// Returns the uncertainty half-width.
    pub fn margin(self) -> f64 {
        (self.upper - self.lower) / 2.0
    }

    /// Returns whether zero target error is compatible with the reported
    /// interval.
    pub fn includes_zero(self) -> bool {
        self.lower <= 0.0 && self.upper >= 0.0
    }
}

// =============================================================================
// Complete analysis result
// =============================================================================

/// Protocol-level diagnostics.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticCode {
    /// Reference and interleaved fits have different quality.
    UnequalFitQuality,

    /// Interleaved decay is greater than reference decay.
    InterleavedDecayGreaterThanReference,

    /// Reference decay is near zero.
    ReferenceDecayNearZero,

    /// Fit quality is poor.
    PoorFit,

    /// Fit quality is acceptable rather than good.
    AcceptableFit,

    /// Target error is compatible with zero.
    TargetErrorIncludesZero,

    /// The target error estimate is strongly model dependent.
    ModelDependentEstimate,

    /// Sequence counts are low.
    LowSequenceCount,

    /// Sequence lengths are narrowly distributed.
    NarrowSequenceRange,

    /// Uncertainty envelope is wide.
    WideUncertainty,

    /// Reference and interleaved experiments may be temporally unmatched.
    TemporalMismatchRisk,
}

/// A protocol diagnostic.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Diagnostic {
    /// Stable diagnostic code.
    pub code: DiagnosticCode,

    /// Human-readable explanation.
    pub message: String,
}

impl Diagnostic {
    fn new(code: DiagnosticCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}

/// Complete IRB analysis result.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InterleavedRandomizedBenchmarkingResult {
    /// Protocol identifier.
    pub protocol_id: String,

    /// Protocol schema version.
    pub schema_version: u32,

    /// Algorithm identifier.
    pub algorithm_id: String,

    /// Target operation.
    pub target_operation: TargetOperation,

    /// Number of qubits.
    pub num_qubits: usize,

    /// Hilbert-space dimension.
    pub dimension: u128,

    /// Reference decay fit.
    pub reference_fit: DecayFit,

    /// Interleaved decay fit.
    pub interleaved_fit: DecayFit,

    /// Target error estimate.
    pub target_error: TargetErrorEstimate,

    /// Number of reference sequences.
    pub reference_sequence_count: usize,

    /// Number of interleaved sequences.
    pub interleaved_sequence_count: usize,

    /// Total reference shots.
    pub reference_shots: u64,

    /// Total interleaved shots.
    pub interleaved_shots: u64,

    /// Protocol confidence level.
    pub confidence_level: f64,

    /// Deterministic experiment seed.
    pub seed: u64,

    /// Protocol diagnostics.
    pub diagnostics: Vec<Diagnostic>,
}

impl InterleavedRandomizedBenchmarkingResult {
    /// Returns true if the result has no diagnostics classified as poor fit.
    pub fn scientifically_clean(&self) -> bool {
        !self
            .diagnostics
            .iter()
            .any(|diagnostic| {
                matches!(
                    diagnostic.code,
                    DiagnosticCode::PoorFit
                        | DiagnosticCode::ReferenceDecayNearZero
                )
            })
    }

    /// Returns the point estimate of target error.
    pub fn target_error_value(&self) -> f64 {
        self.target_error.value
    }

    /// Returns the confidence interval for target error.
    pub fn target_error_interval(&self) -> (f64, f64) {
        (self.target_error.lower, self.target_error.upper)
    }
}

// =============================================================================
// Analysis input
// =============================================================================

/// Offline analysis input.
///
/// Keeping this separate from execution makes captured hardware data
/// independently re-analyzable.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InterleavedAnalysisInput {
    /// Reference observations.
    pub reference: Vec<SurvivalObservation>,

    /// Interleaved observations.
    pub interleaved: Vec<SurvivalObservation>,
}

impl InterleavedAnalysisInput {
    /// Creates analysis input.
    pub fn new(
        reference: Vec<SurvivalObservation>,
        interleaved: Vec<SurvivalObservation>,
    ) -> InterleavedRandomizedBenchmarkingResult<Self> {
        if reference.is_empty() || interleaved.is_empty() {
            return Err(
                InterleavedRandomizedBenchmarkingError::EmptyObservations,
            );
        }

        validate_observation_vector(&reference)?;
        validate_observation_vector(&interleaved)?;

        ensure_matching_sequence_lengths(&reference, &interleaved)?;

        Ok(Self {
            reference,
            interleaved,
        })
    }
}

// =============================================================================
// Public analyzer
// =============================================================================

/// Production IRB analyzer.
///
/// This analyzer is deterministic and performs no I/O.
#[derive(Debug, Clone, Copy)]
pub struct InterleavedRandomizedBenchmarkingAnalyzer;

impl InterleavedRandomizedBenchmarkingAnalyzer {
    /// Creates the analyzer.
    pub const fn new() -> Self {
        Self
    }

    /// Analyzes a complete IRB experiment.
    ///
    /// The decay fitting implementation here is intentionally deterministic
    /// and bounded. In the fully integrated benchmarking tree, the protocol
    /// may instead delegate the actual fit to
    /// `statistics::regression`; the resulting protocol semantics remain the
    /// same.
    pub fn analyze(
        &self,
        config: &InterleavedRandomizedBenchmarkingConfig,
        input: &InterleavedAnalysisInput,
    ) -> InterleavedRandomizedBenchmarkingResult {
        config.validate()?;

        let dimension = config.dimension()?;

        if input.reference.is_empty() || input.interleaved.is_empty() {
            return Err(
                InterleavedRandomizedBenchmarkingError::EmptyObservations,
            );
        }

        ensure_matching_sequence_lengths(
            &input.reference,
            &input.interleaved,
        )?;

        let reference_aggregated =
            aggregate_observations(&input.reference)?;

        let interleaved_aggregated =
            aggregate_observations(&input.interleaved)?;

        let reference_fit =
            fit_decay(&reference_aggregated)?;

        let interleaved_fit =
            fit_decay(&interleaved_aggregated)?;

        let target_error = TargetErrorEstimate::from_decay_parameters(
            reference_fit.decay_parameter,
            interleaved_fit.decay_parameter,
            dimension,
            config.confidence_level,
        )?;

        let diagnostics = build_diagnostics(
            config,
            &reference_fit,
            &interleaved_fit,
            &target_error,
        );

        let reference_sequence_count =
            input.reference.len();

        let interleaved_sequence_count =
            input.interleaved.len();

        let reference_shots =
            checked_sum_shots(&input.reference)?;

        let interleaved_shots =
            checked_sum_shots(&input.interleaved)?;

        Ok(InterleavedRandomizedBenchmarkingResult {
            protocol_id:
                INTERLEAVED_RANDOMIZED_BENCHMARKING_ID.to_owned(),
            schema_version:
                INTERLEAVED_RANDOMIZED_BENCHMARKING_SCHEMA_VERSION,
            algorithm_id:
                INTERLEAVED_RANDOMIZED_BENCHMARKING_ALGORITHM_ID.to_owned(),
            target_operation: config.target_operation.clone(),
            num_qubits: config.num_qubits,
            dimension,
            reference_fit,
            interleaved_fit,
            target_error,
            reference_sequence_count,
            interleaved_sequence_count,
            reference_shots,
            interleaved_shots,
            confidence_level: config.confidence_level,
            seed: config.seed,
            diagnostics,
        })
    }
}

impl Default for InterleavedRandomizedBenchmarkingAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Observation aggregation
// =============================================================================

fn aggregate_observations(
    observations: &[SurvivalObservation],
) -> InterleavedRandomizedBenchmarkingResult<Vec<AggregatedObservation>> {
    if observations.is_empty() {
        return Err(
            InterleavedRandomizedBenchmarkingError::EmptyObservations,
        );
    }

    let mut lengths = Vec::<usize>::new();

    for observation in observations {
        if !lengths.contains(&observation.sequence_length) {
            lengths.push(observation.sequence_length);
        }
    }

    lengths.sort_unstable();

    if lengths.len() < MIN_SEQUENCE_LENGTHS_FOR_EXPONENTIAL_FIT {
        return Err(
            InterleavedRandomizedBenchmarkingError::InsufficientSequenceLengths {
                supplied: lengths.len(),
                minimum: MIN_SEQUENCE_LENGTHS_FOR_EXPONENTIAL_FIT,
            },
        );
    }

    let mut result = Vec::with_capacity(lengths.len());

    for length in lengths {
        result.push(
            AggregatedObservation::from_observations(
                observations,
                length,
            )?,
        );
    }

    Ok(result)
}

// =============================================================================
// Deterministic exponential fit
// =============================================================================

/// Fits:
///
/// ```text
/// y = A * p^m + B
/// ```
///
/// using a deterministic bounded profile search.
///
/// The implementation intentionally has no unconstrained optimizer and no
/// process-global numerical state.
///
/// In the integrated benchmarking tree this mathematical work may be delegated
/// to `statistics::regression`; this implementation remains available as the
/// protocol's stable fallback and test oracle.
fn fit_decay(
    observations: &[AggregatedObservation],
) -> InterleavedRandomizedBenchmarkingResult<DecayFit> {
    if observations.len() < MIN_SEQUENCE_LENGTHS_FOR_EXPONENTIAL_FIT {
        return Err(
            InterleavedRandomizedBenchmarkingError::InsufficientSequenceLengths {
                supplied: observations.len(),
                minimum: MIN_SEQUENCE_LENGTHS_FOR_EXPONENTIAL_FIT,
            },
        );
    }

    let mut best_error = f64::INFINITY;
    let mut best_amplitude = 0.0;
    let mut best_decay = 1.0;
    let mut best_offset = 0.0;

    // Search p directly. The grid is intentionally bounded.
    //
    // Dense enough for a robust initial basin while remaining deterministic
    // and inexpensive for benchmark-scale datasets.
    const GRID_POINTS: usize = 512;

    for index in 0..GRID_POINTS {
        let fraction =
            index as f64 / (GRID_POINTS - 1) as f64;

        // Avoid exactly zero because p^m becomes degenerate for large m.
        let p = 1.0e-6 + fraction * (1.0 - 1.0e-6);

        let candidate =
            solve_amplitude_offset(observations, p)?;

        if candidate.error < best_error {
            best_error = candidate.error;
            best_amplitude = candidate.amplitude;
            best_decay = p;
            best_offset = candidate.offset;
        }
    }

    // Refine the best grid point with deterministic ternary/golden-style
    // interval search.
    let grid_spacing =
        1.0 / (GRID_POINTS - 1) as f64;

    let mut lower =
        (best_decay - grid_spacing * 2.0).max(1.0e-10);

    let mut upper =
        (best_decay + grid_spacing * 2.0).min(1.0);

    for _ in 0..128 {
        let left =
            lower + (upper - lower) / 3.0;

        let right =
            upper - (upper - lower) / 3.0;

        let left_fit =
            solve_amplitude_offset(observations, left)?;

        let right_fit =
            solve_amplitude_offset(observations, right)?;

        if left_fit.error < right_fit.error {
            upper = right;
        } else {
            lower = left;
        }

        if (upper - lower) <= 1.0e-12 {
            break;
        }
    }

    let refined_p = (lower + upper) / 2.0;

    let refined =
        solve_amplitude_offset(observations, refined_p)?;

    if !refined.error.is_finite() {
        return Err(
            InterleavedRandomizedBenchmarkingError::NumericalFailure {
                operation: "IRB exponential fit",
            },
        );
    }

    best_amplitude = refined.amplitude;
    best_decay = refined_p;
    best_offset = refined.offset;
    best_error = refined.error;

    let mean = observations
        .iter()
        .map(|observation| observation.probability)
        .sum::<f64>()
        / observations.len() as f64;

    let mut total_sum_squares = 0.0;

    for observation in observations {
        let delta =
            observation.probability - mean;

        total_sum_squares += delta * delta;
    }

    let rmse =
        (best_error / observations.len() as f64).sqrt();

    let r_squared = if total_sum_squares <= f64::EPSILON {
        if best_error <= f64::EPSILON {
            1.0
        } else {
            0.0
        }
    } else {
        1.0 - best_error / total_sum_squares
    };

    DecayFit::new(
        best_amplitude,
        best_decay,
        best_offset,
        rmse,
        r_squared,
        observations.len(),
    )
}

/// Linear least-squares result for fixed p.
#[derive(Debug, Clone, Copy)]
struct FixedDecayFit {
    amplitude: f64,
    offset: f64,
    error: f64,
}

/// Solves the linear A/B problem for a fixed decay p.
fn solve_amplitude_offset(
    observations: &[AggregatedObservation],
    p: f64,
) -> InterleavedRandomizedBenchmarkingResult<FixedDecayFit> {
    if !p.is_finite() || p <= 0.0 || p > 1.0 {
        return Err(
            InterleavedRandomizedBenchmarkingError::InvalidDecayParameter {
                value: p,
                parameter: "candidate",
            },
        );
    }

    let mut sum_x = 0.0;
    let mut sum_y = 0.0;
    let mut sum_xx = 0.0;
    let mut sum_xy = 0.0;

    for observation in observations {
        let x =
            p.powi(observation.sequence_length as i32);

        if !x.is_finite() {
            return Err(
                InterleavedRandomizedBenchmarkingError::NumericalFailure {
                    operation: "computing exponential basis",
                },
            );
        }

        let y = observation.probability;

        sum_x += x;
        sum_y += y;
        sum_xx += x * x;
        sum_xy += x * y;
    }

    let n = observations.len() as f64;

    let determinant =
        n * sum_xx - sum_x * sum_x;

    if !determinant.is_finite()
        || determinant.abs() <= 1.0e-18
    {
        return Err(
            InterleavedRandomizedBenchmarkingError::NumericalFailure {
                operation: "solving amplitude-offset system",
            },
        );
    }

    let amplitude =
        (n * sum_xy - sum_x * sum_y) / determinant;

    let offset =
        (sum_y - amplitude * sum_x) / n;

    if !amplitude.is_finite() || !offset.is_finite() {
        return Err(
            InterleavedRandomizedBenchmarkingError::NumericalFailure {
                operation: "validating amplitude-offset solution",
            },
        );
    }

    let mut error = 0.0;

    for observation in observations {
        let x =
            p.powi(observation.sequence_length as i32);

        let predicted =
            amplitude * x + offset;

        let residual =
            observation.probability - predicted;

        error += residual * residual;
    }

    if !error.is_finite() {
        return Err(
            InterleavedRandomizedBenchmarkingError::NumericalFailure {
                operation: "calculating regression residuals",
            },
        );
    }

    Ok(FixedDecayFit {
        amplitude,
        offset,
        error,
    })
}

// =============================================================================
// Diagnostics
// =============================================================================

fn build_diagnostics(
    config: &InterleavedRandomizedBenchmarkingConfig,
    reference: &DecayFit,
    interleaved: &DecayFit,
    target_error: &TargetErrorEstimate,
) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();

    if reference.quality != interleaved.quality {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::UnequalFitQuality,
            "reference and interleaved exponential fits have different \
             quality classifications",
        ));
    }

    if reference.quality == FitQuality::Poor
        || interleaved.quality == FitQuality::Poor
    {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::PoorFit,
            "at least one IRB decay fit is classified as poor; the target \
             error should not be interpreted without additional validation",
        ));
    } else if reference.quality == FitQuality::Acceptable
        || interleaved.quality == FitQuality::Acceptable
    {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::AcceptableFit,
            "at least one IRB decay fit is acceptable rather than good",
        ));
    }

    if reference.decay_parameter
        < MIN_REFERENCE_DECAY_PARAMETER * 100.0
    {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::ReferenceDecayNearZero,
            "reference decay is close to the numerical stability boundary",
        ));
    }

    if interleaved.decay_parameter
        > reference.decay_parameter
    {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::InterleavedDecayGreaterThanReference,
            "interleaved decay exceeds reference decay; the resulting \
             target-error interpretation is inconsistent with the standard \
             monotonic IRB model",
        ));
    }

    if target_error.includes_zero() {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::TargetErrorIncludesZero,
            "the target-error uncertainty interval includes zero",
        ));
    }

    diagnostics.push(Diagnostic::new(
        DiagnosticCode::ModelDependentEstimate,
        "the target-error estimate is a decay-derived IRB quantity and \
         depends on the assumptions of the interleaved randomized-benchmarking \
         model",
    ));

    if config.sequences_per_length < 8 {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::LowSequenceCount,
            "fewer than eight random sequences were used per sequence length; \
             sampling uncertainty may be substantial",
        ));
    }

    if config.sequence_lengths.len() < 6 {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::NarrowSequenceRange,
            "the experiment contains fewer than six sequence lengths; decay \
             diagnostics may be less robust",
        ));
    }

    if target_error.margin() > 0.10 {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::WideUncertainty,
            "the target-error uncertainty interval is wide relative to the \
             unit error scale",
        ));
    }

    diagnostics.push(Diagnostic::new(
        DiagnosticCode::TemporalMismatchRisk,
        "reference and interleaved experiments should be randomized or \
         scheduled so that temporal drift does not become confounded with \
         experiment type",
    ));

    diagnostics
}

// =============================================================================
// Validation helpers
// =============================================================================

fn validate_observation_vector(
    observations: &[SurvivalObservation],
) -> InterleavedRandomizedBenchmarkingResult<()> {
    for observation in observations {
        if observation.sequence_length == 0
            || observation.shots == 0
            || observation.successes > observation.shots
        {
            return Err(
                InterleavedRandomizedBenchmarkingError::InvalidObservation {
                    sequence_length: observation.sequence_length,
                    successes: observation.successes,
                    shots: observation.shots,
                },
            );
        }

        observation.probability()?;
    }

    Ok(())
}

fn ensure_matching_sequence_lengths(
    reference: &[SurvivalObservation],
    interleaved: &[SurvivalObservation],
) -> InterleavedRandomizedBenchmarkingResult<()> {
    let mut reference_lengths = Vec::<usize>::new();

    for observation in reference {
        if !reference_lengths.contains(&observation.sequence_length) {
            reference_lengths.push(observation.sequence_length);
        }
    }

    let mut interleaved_lengths = Vec::<usize>::new();

    for observation in interleaved {
        if !interleaved_lengths.contains(&observation.sequence_length) {
            interleaved_lengths.push(observation.sequence_length);
        }
    }

    reference_lengths.sort_unstable();
    interleaved_lengths.sort_unstable();

    if reference_lengths != interleaved_lengths {
        return Err(
            InterleavedRandomizedBenchmarkingError::ObservationLengthMismatch,
        );
    }

    for length in reference_lengths {
        if !interleaved_lengths.contains(&length) {
            return Err(
                InterleavedRandomizedBenchmarkingError::MissingSequenceLength {
                    sequence_length: length,
                },
            );
        }
    }

    Ok(())
}

fn checked_sum_shots(
    observations: &[SurvivalObservation],
) -> InterleavedRandomizedBenchmarkingResult<u64> {
    let mut total = 0u64;

    for observation in observations {
        total = total.checked_add(observation.shots).ok_or(
            InterleavedRandomizedBenchmarkingError::ResourceCalculationOverflow,
        )?;
    }

    Ok(total)
}

fn validate_decay_parameter(
    value: f64,
    parameter: &'static str,
) -> InterleavedRandomizedBenchmarkingResult<()> {
    if !value.is_finite() || value <= 0.0 || value > 1.0 {
        return Err(
            InterleavedRandomizedBenchmarkingError::InvalidDecayParameter {
                value,
                parameter,
            },
        );
    }

    Ok(())
}

fn target_error_from_ratio(
    reference_decay: f64,
    interleaved_decay: f64,
    dimension: u128,
) -> InterleavedRandomizedBenchmarkingResult<f64> {
    validate_decay_parameter(reference_decay, "reference")?;
    validate_decay_parameter(interleaved_decay, "interleaved")?;

    if dimension < 2 {
        return Err(
            InterleavedRandomizedBenchmarkingError::InvalidDimension {
                dimension,
            },
        );
    }

    if reference_decay < MIN_REFERENCE_DECAY_PARAMETER {
        return Err(
            InterleavedRandomizedBenchmarkingError::ReferenceDecayTooSmall {
                value: reference_decay,
            },
        );
    }

    let ratio =
        interleaved_decay / reference_decay;

    if !ratio.is_finite()
        || ratio < -PHYSICAL_RATIO_EPSILON
        || ratio > 1.0 + PHYSICAL_RATIO_EPSILON
    {
        return Err(
            InterleavedRandomizedBenchmarkingError::InvalidDecayRatio {
                interleaved: interleaved_decay,
                reference: reference_decay,
                ratio,
            },
        );
    }

    let d = dimension as f64;

    let result =
        ((d - 1.0) / d) * (1.0 - ratio);

    if !result.is_finite()
        || result < -PHYSICAL_RATIO_EPSILON
        || result > 1.0 + PHYSICAL_RATIO_EPSILON
    {
        return Err(
            InterleavedRandomizedBenchmarkingError::InvalidTargetErrorEstimate {
                value: result,
            },
        );
    }

    Ok(result.clamp(0.0, 1.0))
}

// =============================================================================
// Deterministic seed derivation
// =============================================================================

/// Derives a deterministic seed for one reference sequence.
///
/// This is deliberately not a cryptographic hash. It is a reproducible
/// experiment-stream derivation function.
///
/// Security-sensitive identity hashing must use the provenance subsystem
/// instead.
fn derive_sequence_seed(
    seed: u64,
    sequence_length: usize,
    repetition: usize,
) -> u64 {
    let mut x =
        seed ^ 0x9E37_79B9_7F4A_7C15;

    x ^= sequence_length as u64;
    x = splitmix64(x);

    x ^= repetition as u64;
    x = splitmix64(x);

    x
}

/// Derives a separate stream for the interleaved experiment.
fn derive_interleaved_seed(seed: u64) -> u64 {
    splitmix64(seed ^ 0xD1B5_4A32_D192_ED03)
}

/// Deterministic SplitMix64 step.
fn splitmix64(mut value: u64) -> u64 {
    value = value.wrapping_add(0x9E37_79B9_7F4A_7C15);

    let mut z = value;

    z = (z ^ (z >> 30))
        .wrapping_mul(0xBF58_476D_1CE4_E5B9);

    z = (z ^ (z >> 27))
        .wrapping_mul(0x94D0_49BB_1331_11EB);

    z ^ (z >> 31)
}

// =============================================================================
// Arithmetic helpers
// =============================================================================

fn checked_mul_u128(
    left: u128,
    right: u128,
) -> InterleavedRandomizedBenchmarkingResult<u128> {
    left.checked_mul(right).ok_or(
        InterleavedRandomizedBenchmarkingError::ResourceCalculationOverflow,
    )
}

fn checked_mul_usize(
    left: usize,
    right: usize,
) -> InterleavedRandomizedBenchmarkingResult<usize> {
    left.checked_mul(right).ok_or(
        InterleavedRandomizedBenchmarkingError::ResourceCalculationOverflow,
    )
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn target() -> TargetOperation {
        TargetOperation::new("x", "clifford").unwrap()
    }

    #[test]
    fn default_configuration_is_valid() {
        let configuration =
            InterleavedRandomizedBenchmarkingConfig::default();

        assert!(configuration.validate().is_ok());
        assert_eq!(configuration.num_qubits, 1);
    }

    #[test]
    fn dimension_for_one_qubit_is_two() {
        let configuration =
            InterleavedRandomizedBenchmarkingConfig::default();

        assert_eq!(configuration.dimension().unwrap(), 2);
    }

    #[test]
    fn dimension_for_two_qubits_is_four() {
        let mut configuration =
            InterleavedRandomizedBenchmarkingConfig::default();

        configuration.num_qubits = 2;

        assert_eq!(configuration.dimension().unwrap(), 4);
    }

    #[test]
    fn zero_qubits_are_rejected() {
        let mut configuration =
            InterleavedRandomizedBenchmarkingConfig::default();

        configuration.num_qubits = 0;

        assert!(matches!(
            configuration.validate(),
            Err(
                InterleavedRandomizedBenchmarkingError::InvalidQubitCount {
                    ..
                }
            )
        ));
    }

    #[test]
    fn zero_sequence_length_is_rejected() {
        let mut configuration =
            InterleavedRandomizedBenchmarkingConfig::default();

        configuration.sequence_lengths = vec![0, 1, 2, 4];

        assert!(matches!(
            configuration.validate(),
            Err(
                InterleavedRandomizedBenchmarkingError::InvalidSequenceLength {
                    length: 0
                }
            )
        ));
    }

    #[test]
    fn duplicate_sequence_length_is_rejected() {
        let mut configuration =
            InterleavedRandomizedBenchmarkingConfig::default();

        configuration.sequence_lengths = vec![1, 2, 2, 4];

        assert!(matches!(
            configuration.validate(),
            Err(
                InterleavedRandomizedBenchmarkingError::DuplicateSequenceLength {
                    length: 2
                }
            )
        ));
    }

    #[test]
    fn invalid_observation_is_rejected() {
        let result =
            SurvivalObservation::new(1, 11, 10);

        assert!(matches!(
            result,
            Err(
                InterleavedRandomizedBenchmarkingError::InvalidObservation {
                    ..
                }
            )
        ));
    }

    #[test]
    fn survival_probability_is_correct() {
        let observation =
            SurvivalObservation::new(4, 750, 1_000)
                .unwrap();

        assert!((observation.probability().unwrap() - 0.75).abs() < 1.0e-12);
    }

    #[test]
    fn target_error_formula_for_one_qubit_is_correct() {
        let result =
            TargetErrorEstimate::from_decay_parameters(
                0.98,
                0.97,
                2,
                0.95,
            )
            .unwrap();

        let expected =
            0.5 * (1.0 - 0.97 / 0.98);

        assert!((result.value - expected).abs() < 1.0e-12);
    }

    #[test]
    fn target_error_zero_when_decays_are_equal() {
        let result =
            TargetErrorEstimate::from_decay_parameters(
                0.98,
                0.98,
                2,
                0.95,
            )
            .unwrap();

        assert!(result.value.abs() < 1.0e-12);
        assert!(result.includes_zero());
    }

    #[test]
    fn interleaved_decay_greater_than_reference_is_rejected() {
        let result =
            TargetErrorEstimate::from_decay_parameters(
                0.90,
                0.91,
                2,
                0.95,
            );

        assert!(matches!(
            result,
            Err(
                InterleavedRandomizedBenchmarkingError::InvalidDecayRatio {
                    ..
                }
            )
        ));
    }

    #[test]
    fn reference_decay_near_zero_is_rejected() {
        let result =
            TargetErrorEstimate::from_decay_parameters(
                1.0e-15,
                1.0e-15,
                2,
                0.95,
            );

        assert!(matches!(
            result,
            Err(
                InterleavedRandomizedBenchmarkingError::ReferenceDecayTooSmall {
                    ..
                }
            )
        ));
    }

    #[test]
    fn deterministic_plan_is_reproducible() {
        let configuration =
            InterleavedRandomizedBenchmarkingConfig::new(
                1,
                vec![1, 2, 4, 8],
                target(),
                42,
            )
            .unwrap();

        let first =
            InterleavedExperimentPlan::from_config(&configuration)
                .unwrap();

        let second =
            InterleavedExperimentPlan::from_config(&configuration)
                .unwrap();

        assert_eq!(first, second);
    }

    #[test]
    fn reference_and_interleaved_plans_are_distinct() {
        let configuration =
            InterleavedRandomizedBenchmarkingConfig::new(
                1,
                vec![1, 2, 4, 8],
                target(),
                42,
            )
            .unwrap();

        let plan =
            InterleavedExperimentPlan::from_config(&configuration)
                .unwrap();

        assert_eq!(
            plan.reference_sequence_count(),
            plan.interleaved_sequence_count()
        );

        assert_eq!(
            plan.reference[0].target_operation_count(),
            0
        );

        assert_eq!(
            plan.interleaved[0].target_operation_count(),
            plan.interleaved[0].reference_length
        );
    }

    #[test]
    fn fixed_decay_fit_recovers_synthetic_data() {
        let observations = vec![
            AggregatedObservation {
                sequence_length: 1,
                successes: 900,
                shots: 1_000,
                probability: 0.900,
            },
            AggregatedObservation {
                sequence_length: 2,
                successes: 850,
                shots: 1_000,
                probability: 0.850,
            },
            AggregatedObservation {
                sequence_length: 4,
                successes: 770,
                shots: 1_000,
                probability: 0.770,
            },
            AggregatedObservation {
                sequence_length: 8,
                successes: 650,
                shots: 1_000,
                probability: 0.650,
            },
            AggregatedObservation {
                sequence_length: 16,
                successes: 500,
                shots: 1_000,
                probability: 0.500,
            },
        ];

        let fit =
            fit_decay(&observations).unwrap();

        assert!(fit.decay_parameter > 0.0);
        assert!(fit.decay_parameter <= 1.0);
        assert!(fit.rmse.is_finite());
        assert!(fit.r_squared.is_finite());
    }

    #[test]
    fn analysis_requires_matching_lengths() {
        let reference = vec![
            SurvivalObservation::new(1, 900, 1_000).unwrap(),
            SurvivalObservation::new(2, 850, 1_000).unwrap(),
            SurvivalObservation::new(4, 800, 1_000).unwrap(),
            SurvivalObservation::new(8, 700, 1_000).unwrap(),
        ];

        let interleaved = vec![
            SurvivalObservation::new(1, 890, 1_000).unwrap(),
            SurvivalObservation::new(2, 830, 1_000).unwrap(),
            SurvivalObservation::new(4, 770, 1_000).unwrap(),
            SurvivalObservation::new(16, 600, 1_000).unwrap(),
        ];

        let result =
            InterleavedAnalysisInput::new(
                reference,
                interleaved,
            );

        assert!(matches!(
            result,
            Err(
                InterleavedRandomizedBenchmarkingError::ObservationLengthMismatch
            )
        ));
    }

    #[test]
    fn analysis_is_deterministic() {
        let configuration =
            InterleavedRandomizedBenchmarkingConfig::new(
                1,
                vec![1, 2, 4, 8, 16],
                target(),
                123,
            )
            .unwrap();

        let reference = vec![
            SurvivalObservation::new(1, 950, 1_000).unwrap(),
            SurvivalObservation::new(2, 930, 1_000).unwrap(),
            SurvivalObservation::new(4, 900, 1_000).unwrap(),
            SurvivalObservation::new(8, 850, 1_000).unwrap(),
            SurvivalObservation::new(16, 760, 1_000).unwrap(),
        ];

        let interleaved = vec![
            SurvivalObservation::new(1, 940, 1_000).unwrap(),
            SurvivalObservation::new(2, 910, 1_000).unwrap(),
            SurvivalObservation::new(4, 860, 1_000).unwrap(),
            SurvivalObservation::new(8, 760, 1_000).unwrap(),
            SurvivalObservation::new(16, 600, 1_000).unwrap(),
        ];

        let input =
            InterleavedAnalysisInput::new(
                reference,
                interleaved,
            )
            .unwrap();

        let analyzer =
            InterleavedRandomizedBenchmarkingAnalyzer::new();

        let first =
            analyzer.analyze(&configuration, &input)
                .unwrap();

        let second =
            analyzer.analyze(&configuration, &input)
                .unwrap();

        assert_eq!(first, second);
    }

    #[test]
    fn uncertainty_envelope_is_ordered() {
        let estimate =
            TargetErrorEstimate::from_decay_parameters(
                0.98,
                0.95,
                2,
                0.95,
            )
            .unwrap();

        let bounded =
            estimate
                .with_decay_bounds(
                    0.97,
                    0.99,
                    0.94,
                    0.96,
                    2,
                )
                .unwrap();

        assert!(bounded.lower <= bounded.value);
        assert!(bounded.value <= bounded.upper);
        assert!(bounded.lower >= 0.0);
        assert!(bounded.upper <= 1.0);
    }

    #[test]
    fn sequence_seed_changes_with_repetition() {
        let first =
            derive_sequence_seed(42, 8, 0);

        let second =
            derive_sequence_seed(42, 8, 1);

        assert_ne!(first, second);
    }

    #[test]
    fn sequence_seed_changes_with_length() {
        let first =
            derive_sequence_seed(42, 8, 0);

        let second =
            derive_sequence_seed(42, 16, 0);

        assert_ne!(first, second);
    }

    #[test]
    fn interleaved_seed_differs_from_reference_seed() {
        let reference =
            derive_sequence_seed(42, 8, 0);

        let interleaved =
            derive_interleaved_seed(reference);

        assert_ne!(reference, interleaved);
    }

    #[test]
    fn total_shots_counts_both_experiments() {
        let configuration =
            InterleavedRandomizedBenchmarkingConfig {
                sequence_lengths: vec![1, 2, 4, 8],
                sequences_per_length: 10,
                shots_per_sequence: 100,
                ..InterleavedRandomizedBenchmarkingConfig::default()
            };

        assert_eq!(
            configuration.total_shots().unwrap(),
            8_000
        );
    }

    #[test]
    fn target_operation_identity_requires_id() {
        let result =
            TargetOperation::new("", "gate");

        assert!(result.is_err());
    }

    #[test]
    fn target_operation_identity_requires_family() {
        let result =
            TargetOperation::new("x", "");

        assert!(result.is_err());
    }
}