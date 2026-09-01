//! Zamani Quantum Noise (ZQN)
//! Standard Randomized Benchmarking Characterization
//!
//! # Ownership
//!
//! This file owns the backend-independent scientific contract for standard
//! randomized benchmarking (RB) inside ZQN characterization.
//!
//! It owns:
//!
//! - standard-RB configuration;
//! - RB protocol metadata;
//! - deterministic RB sequence planning metadata;
//! - workload validation;
//! - RB observation aggregation;
//! - streaming accumulation by sequence length;
//! - survival-probability calculation;
//! - standard exponential RB decay fitting;
//! - EPC calculation;
//! - fit diagnostics;
//! - deterministic numerical optimization;
//! - numerical validation;
//! - RB-specific scientific warnings;
//! - RB-specific analysis results;
//! - RB-specific provenance fields required to reproduce the analysis.
//!
//! # Does not own
//!
//! This file does NOT own:
//!
//! - canonical Quantum IR;
//! - Clifford-group mathematics;
//! - Clifford synthesis;
//! - random-number generation;
//! - gate definitions;
//! - hardware APIs;
//! - simulator implementation;
//! - routing;
//! - scheduling;
//! - calibration storage;
//! - generic observation storage;
//! - generic statistical estimators;
//! - generic confidence-interval mathematics;
//! - benchmarking report generation;
//! - vendor-specific execution;
//! - QEC;
//! - source-language parsing.
//!
//! In particular, this module does not generate a Clifford circuit.
//!
//! A downstream generator consumes [`RbSequencePlan`] and converts it into
//! canonical `crate::quantum::ir` operations.
//!
//! # Architectural position
//!
//! ```text
//!                         quantum::ir
//!                             │
//!                             │ canonical semantics
//!                             ▼
//!                  ZQN characterization
//!                             │
//!                 randomized_benchmarking.rs
//!                             │
//!              ┌──────────────┼──────────────┐
//!              ▼              ▼              ▼
//!        sequence plan     execution      analysis
//!              │              │              │
//!              ▼              ▼              ▼
//!       Clifford generator  QPU/sim       RB fit
//!              │              │              │
//!              └──────────────┼──────────────┘
//!                             ▼
//!                      ZQN characterization
//!                             │
//!                  ┌──────────┼──────────┐
//!                  ▼          ▼          ▼
//!              calibration   noise   benchmarking
//! ```
//!
//! # Standard RB model
//!
//! Standard RB models the measured survival probability as:
//!
//! ```text
//! P(m) = A * p^m + B
//! ```
//!
//! where:
//!
//! - `m` is the randomized sequence length;
//! - `A` is an SPAM/amplitude parameter;
//! - `p` is the decay parameter;
//! - `B` is an offset;
//! - `P(m)` is the measured survival probability.
//!
//! Under the standard RB assumptions, the decay-derived error-per-Clifford
//! quantity is:
//!
//! ```text
//! EPC = (d - 1) / d * (1 - p)
//! ```
//!
//! where `d` is the effective Hilbert-space dimension.
//!
//! This result is an RB-derived error estimate. It is NOT automatically an
//! exact physical gate infidelity.
//!
//! The scientific interpretation depends on the RB assumptions, including
//! appropriate twirling/group properties and sufficiently controlled
//! time-dependent, gate-dependent and leakage effects.
//!
//! # Important scientific limitation
//!
//! Randomized benchmarking is an inference protocol, not a universal noise
//! tomography method.
//!
//! The resulting decay parameter can be affected by:
//!
//! - gate-dependent noise;
//! - coherent errors;
//! - non-Markovian noise;
//! - leakage;
//! - drift;
//! - imperfect state preparation;
//! - measurement error;
//! - imperfect Clifford synthesis;
//! - correlated errors;
//! - mismatch between the characterized implementation and the intended
//!   operation set.
//!
//! Therefore the result exposes diagnostics and assumptions rather than
//! claiming exact physical noise reconstruction.
//!
//! # Standard RB versus interleaved RB
//!
//! This file implements STANDARD RB.
//!
//! Interleaved RB is a different scientific protocol because it compares a
//! reference decay against a decay containing an inserted target operation.
//! It must not be silently represented as standard RB.
//!
//! The repository already contains an interleaved-RB implementation in the
//! benchmarking subsystem. ZQN characterization can consume its observations
//! through an adapter without making this file duplicate that methodology.
//!
//! # Canonical quantum identities
//!
//! When a concrete logical or physical qubit scope is supplied, this file
//! uses:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! This file never defines:
//!
//! ```text
//! ZqnQubitId
//! ZqnPhysicalQubitId
//! ```
//!
//! # Scalability
//!
//! There is NO semantic maximum for:
//!
//! - number of qubits;
//! - number of resources;
//! - number of sequence lengths;
//! - sequence length;
//! - number of random sequences;
//! - number of shots;
//! - number of experiments;
//! - number of characterization runs.
//!
//! All workload limits are explicit caller/runtime policy.
//!
//! The mathematical fitting algorithm only materializes the aggregated
//! sequence-length points. It does not require all individual shots to remain
//! in memory.
//!
//! Thus:
//!
//! ```text
//! shots
//! sequences
//! machines
//! resources
//! ```
//!
//! may grow independently, subject only to the resources available to the
//! execution and analysis environment.
//!
//! # Determinism
//!
//! Randomized experiment generation is represented by an explicit seed.
//!
//! This file:
//!
//! - never uses a global RNG;
//! - never reads the system clock;
//! - never derives identity from memory addresses;
//! - never derives identity from thread IDs;
//! - never depends on hash-map iteration order;
//! - never uses random optimization.
//!
//! The deterministic sequence identifier is derived from:
//!
//! ```text
//! seed
//! + protocol algorithm identifier
//! + sequence length
//! + sample index
//! ```
//!
//! A downstream Clifford generator must use the same deterministic identity
//! when deriving its own random substream.
//!
//! # Numerical safety
//!
//! The implementation rejects:
//!
//! - NaN;
//! - positive infinity;
//! - negative infinity;
//! - invalid probabilities;
//! - invalid dimensions;
//! - invalid decay bounds;
//! - invalid weights;
//! - arithmetic overflow.
//!
//! It does not silently clamp invalid results.
//!
//! Numerical optimization is deterministic and bounded by explicit iteration
//! configuration.
//!
//! # Resource safety
//!
//! No fixed machine-size constants are used.
//!
//! Explicit caller limits come from:
//!
//! `characterization::protocol::CharacterizationLimits`
//!
//! and the RB-specific analysis configuration.
//!
//! A large workload can therefore be accepted when the caller supplies enough
//! resources, while a constrained runtime can reject it before materializing
//! circuits.
//!
//! # Serialization
//!
//! This file defines semantic Rust structures only.
//!
//! Wire serialization belongs to `zqn::io`.
//!
//! A future serializer must preserve:
//!
//! - schema version;
//! - algorithm identifier;
//! - configuration;
//! - seed;
//! - target scope;
//! - sequence lengths;
//! - sample counts;
//! - shot counts;
//! - fit configuration;
//! - analysis results;
//! - diagnostics.
//!
//! Serialization must not depend on Rust memory layout.
//!
//! # Thread safety
//!
//! Configuration and result types contain no global mutable state.
//!
//! The accumulator is intentionally mutable only through an owned instance.
//! Independent accumulators may be used concurrently and merged in a caller-
//! controlled deterministic order.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe`.
//!
//! # Definition of done
//!
//! This file is complete when:
//!
//! 1. standard RB has an explicit protocol contract;
//! 2. workload generation is separated from execution;
//! 3. canonical quantum IDs are used;
//! 4. sequence identities are deterministic;
//! 5. no fixed machine-size limit exists;
//! 6. observations can be accumulated incrementally;
//! 7. invalid numerical input is rejected;
//! 8. fitting is deterministic;
//! 9. the RB decay model is explicit;
//! 10. EPC is explicitly identified as an RB-derived estimate;
//! 11. fit diagnostics are retained;
//! 12. no vendor dependency exists;
//! 13. no hardware dependency exists;
//! 14. no canonical IR duplication exists;
//! 15. standard RB and interleaved RB remain separate;
//! 16. the file can be consumed by protocol, observation, estimator,
//!     generator, runtime, calibration and benchmarking layers without
//!     requiring semantic changes to this file.
//!
//! # Safety
//!
//! Unsafe Rust is forbidden at compile time.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;
use std::collections::BTreeMap;
use std::error::Error;

use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};
use crate::quantum::zqn::characterization::protocol::{
    AnalysisContract, CharacterizationLimits, CharacterizationObjective,
    CharacterizationProtocol, CharacterizationRequirements, CharacterizationScope,
    ExperimentPlan, ObservationRequirements, ProtocolDescriptor, ProtocolError,
    ProtocolId, ProtocolResult, ProtocolVersion, RandomnessContract, WorkloadQuantity,
};

// ============================================================================
// Stable protocol identifiers
// ============================================================================

/// Stable standard-RB protocol identifier.
pub const RANDOMIZED_BENCHMARKING_ID: &str =
    "randomized_benchmarking";

/// Stable semantic schema version.
pub const RANDOMIZED_BENCHMARKING_SCHEMA_VERSION: u32 = 1;

/// Stable scientific algorithm identifier.
///
/// This identifier MUST change when the mathematical fitting model or another
/// scientifically meaningful part of the analysis changes.
pub const RANDOMIZED_BENCHMARKING_ALGORITHM_ID: &str =
    "zamani.rb.standard.decay.v1";

// ============================================================================
// Error type
// ============================================================================

/// Errors specific to standard randomized benchmarking.
#[derive(Debug, Clone, PartialEq)]
pub enum RandomizedBenchmarkingError {
    /// The configuration contains an invalid value.
    InvalidConfiguration {
        /// Human-readable reason.
        reason: &'static str,
    },

    /// A sequence length is invalid.
    InvalidSequenceLength {
        /// Invalid length.
        length: u64,
    },

    /// Duplicate sequence lengths are not allowed.
    DuplicateSequenceLength {
        /// Duplicate value.
        length: u64,
    },

    /// The supplied sequence-length collection is empty.
    EmptySequenceLengths,

    /// No random sequences were requested.
    ZeroSequencesPerLength,

    /// No shots were requested.
    ZeroShotsPerSequence,

    /// A configured workload exceeds a caller-supplied resource limit.
    ResourceLimitExceeded {
        /// Workload name.
        resource: &'static str,

        /// Requested amount.
        requested: u128,

        /// Maximum allowed amount.
        maximum: u128,
    },

    /// Workload arithmetic overflowed.
    WorkloadOverflow,

    /// A dimension is invalid.
    InvalidDimension {
        /// Invalid dimension.
        dimension: u128,
    },

    /// A numerical value is non-finite.
    NonFiniteValue {
        /// Value name.
        field: &'static str,
    },

    /// A probability is outside the closed unit interval.
    InvalidProbability {
        /// Value name.
        field: &'static str,

        /// Supplied value.
        value: f64,
    },

    /// A weight is invalid.
    InvalidWeight {
        /// Weight.
        value: f64,
    },

    /// An observation contains more successes than shots.
    InvalidObservation {
        /// Sequence length.
        sequence_length: u64,

        /// Success count.
        successes: u64,

        /// Shot count.
        shots: u64,
    },

    /// No observations were supplied.
    EmptyObservations,

    /// An observation has an invalid sequence length.
    ObservationLengthNotConfigured {
        /// Length found in the observations.
        length: u64,
    },

    /// An observation set is incompatible with the configured protocol.
    ObservationConfigurationMismatch,

    /// Not enough distinct sequence lengths are available for the selected
    /// fitting model.
    InsufficientSequenceLengths {
        /// Number available.
        available: usize,

        /// Number required.
        required: usize,
    },

    /// The numerical fit failed.
    FitFailure {
        /// Operation that failed.
        operation: &'static str,
    },

    /// The reference model is numerically singular.
    SingularModel,

    /// The decay parameter is outside the configured search interval.
    InvalidDecayBounds,

    /// The reference decay is unsuitable for an RB-derived ratio.
    InvalidReferenceDecay {
        /// Decay value.
        value: f64,
    },

    /// The fit produced a non-finite result.
    NonFiniteFit,

    /// A physically interpreted result is invalid.
    InvalidPhysicalResult {
        /// Result value.
        value: f64,
    },
}

impl fmt::Display for RandomizedBenchmarkingError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidConfiguration { reason } => {
                write!(formatter, "invalid randomized benchmarking configuration: {reason}")
            }

            Self::InvalidSequenceLength { length } => {
                write!(formatter, "invalid randomized benchmarking sequence length: {length}")
            }

            Self::DuplicateSequenceLength { length } => {
                write!(formatter, "duplicate randomized benchmarking sequence length: {length}")
            }

            Self::EmptySequenceLengths => {
                formatter.write_str("randomized benchmarking requires sequence lengths")
            }

            Self::ZeroSequencesPerLength => {
                formatter.write_str("randomized benchmarking requires at least one sequence per length")
            }

            Self::ZeroShotsPerSequence => {
                formatter.write_str("randomized benchmarking requires at least one shot per sequence")
            }

            Self::ResourceLimitExceeded {
                resource,
                requested,
                maximum,
            } => write!(
                formatter,
                "randomized benchmarking {resource} exceeds resource limit: \
                 requested {requested}, maximum {maximum}"
            ),

            Self::WorkloadOverflow => {
                formatter.write_str("randomized benchmarking workload arithmetic overflowed")
            }

            Self::InvalidDimension { dimension } => {
                write!(formatter, "invalid RB Hilbert-space dimension: {dimension}")
            }

            Self::NonFiniteValue { field } => {
                write!(formatter, "{field} must be finite")
            }

            Self::InvalidProbability { field, value } => {
                write!(formatter, "{field} must be within [0,1], got {value}")
            }

            Self::InvalidWeight { value } => {
                write!(formatter, "invalid RB fit weight: {value}")
            }

            Self::InvalidObservation {
                sequence_length,
                successes,
                shots,
            } => write!(
                formatter,
                "invalid RB observation at length {sequence_length}: \
                 successes={successes}, shots={shots}"
            ),

            Self::EmptyObservations => {
                formatter.write_str("randomized benchmarking received no observations")
            }

            Self::ObservationLengthNotConfigured { length } => {
                write!(
                    formatter,
                    "observation sequence length {length} was not configured"
                )
            }

            Self::ObservationConfigurationMismatch => {
                formatter.write_str("RB observations do not match the configured experiment")
            }

            Self::InsufficientSequenceLengths {
                available,
                required,
            } => write!(
                formatter,
                "insufficient RB sequence lengths: available {available}, required {required}"
            ),

            Self::FitFailure { operation } => {
                write!(formatter, "RB numerical fit failed during {operation}")
            }

            Self::SingularModel => {
                formatter.write_str("RB regression model is numerically singular")
            }

            Self::InvalidDecayBounds => {
                formatter.write_str("invalid randomized benchmarking decay search bounds")
            }

            Self::InvalidReferenceDecay { value } => {
                write!(formatter, "invalid RB reference decay parameter: {value}")
            }

            Self::NonFiniteFit => {
                formatter.write_str("randomized benchmarking fit produced a non-finite result")
            }

            Self::InvalidPhysicalResult { value } => {
                write!(
                    formatter,
                    "randomized benchmarking physical result is invalid: {value}"
                )
            }
        }
    }
}

impl Error for RandomizedBenchmarkingError {}

/// Result type for this module.
pub type RandomizedBenchmarkingResult<T> =
    Result<T, RandomizedBenchmarkingError>;

// ============================================================================
// Scope
// ============================================================================

/// Quantum-resource scope for an RB experiment.
///
/// Logical and physical qubit IDs are the canonical Quantum IR identities.
/// `TargetDefined` and `Distributed` permit future execution models without
/// introducing new ZQN identity types.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RbScope {
    /// Explicit logical qubits.
    Logical(Vec<QubitId>),

    /// Explicit physical qubits.
    Physical(Vec<PhysicalQubitId>),

    /// Target chooses the concrete resources.
    TargetDefined,

    /// Resources span multiple execution domains.
    Distributed,
}

impl RbScope {
    /// Returns the corresponding generic characterization scope.
    pub fn protocol_scope(&self) -> CharacterizationScope {
        match self {
            Self::Logical(resources) | Self::Physical(_) if resources_len(self) == 1 => {
                CharacterizationScope::Single
            }

            Self::Logical(resources) | Self::Physical(_) if !resources.is_empty() => {
                CharacterizationScope::Explicit
            }

            Self::Logical(_) | Self::Physical(_) => CharacterizationScope::Explicit,

            Self::TargetDefined => CharacterizationScope::TargetDefined,

            Self::Distributed => CharacterizationScope::Distributed,
        }
    }

    /// Validates the resource identity collection.
    pub fn validate(&self) -> RandomizedBenchmarkingResult<()> {
        match self {
            Self::Logical(resources) => validate_unique_resources(resources),
            Self::Physical(resources) => validate_unique_resources(resources),
            Self::TargetDefined | Self::Distributed => Ok(()),
        }
    }

    /// Returns the explicit number of resources when known.
    pub fn explicit_resource_count(&self) -> Option<usize> {
        match self {
            Self::Logical(resources) => Some(resources.len()),
            Self::Physical(resources) => Some(resources.len()),
            Self::TargetDefined | Self::Distributed => None,
        }
    }
}

fn resources_len(scope: &RbScope) -> usize {
    scope.explicit_resource_count().unwrap_or(0)
}

fn validate_unique_resources<T>(resources: &[T]) -> RandomizedBenchmarkingResult<()>
where
    T: Ord,
{
    let mut ordered = resources.to_vec();
    ordered.sort();

    for pair in ordered.windows(2) {
        if pair[0] == pair[1] {
            return Err(RandomizedBenchmarkingError::InvalidConfiguration {
                reason: "RB scope contains duplicate quantum-resource identities",
            });
        }
    }

    Ok(())
}

// ============================================================================
// Configuration
// ============================================================================

/// Standard randomized benchmarking configuration.
///
/// This contains experiment semantics but does not contain generated circuits.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RandomizedBenchmarkingConfig {
    /// Resources being characterized.
    pub scope: RbScope,

    /// Randomized sequence lengths.
    pub sequence_lengths: Vec<u64>,

    /// Number of independently randomized sequences at each length.
    pub sequences_per_length: u64,

    /// Number of shots acquired for each sequence.
    pub shots_per_sequence: u64,

    /// Deterministic master seed supplied by the caller.
    pub seed: u128,

    /// Effective Hilbert-space dimension used for EPC interpretation.
    ///
    /// The value is deliberately not represented as `2^n` so that the
    /// characterization contract is not restricted to one future modality.
    pub dimension: u128,
}

impl RandomizedBenchmarkingConfig {
    /// Creates a configuration.
    pub fn new(
        scope: RbScope,
        sequence_lengths: Vec<u64>,
        sequences_per_length: u64,
        shots_per_sequence: u64,
        seed: u128,
        dimension: u128,
    ) -> RandomizedBenchmarkingResult<Self> {
        let configuration = Self {
            scope,
            sequence_lengths,
            sequences_per_length,
            shots_per_sequence,
            seed,
            dimension,
        };

        configuration.validate()?;

        Ok(configuration)
    }

    /// Validates intrinsic configuration invariants.
    pub fn validate(&self) -> RandomizedBenchmarkingResult<()> {
        self.scope.validate()?;

        if self.sequence_lengths.is_empty() {
            return Err(RandomizedBenchmarkingError::EmptySequenceLengths);
        }

        let mut previous = None;

        for &length in &self.sequence_lengths {
            if length == 0 {
                return Err(RandomizedBenchmarkingError::InvalidSequenceLength { length });
            }

            if let Some(previous_length) = previous {
                if previous_length == length {
                    return Err(RandomizedBenchmarkingError::DuplicateSequenceLength { length });
                }
            }

            previous = Some(length);
        }

        if self.sequences_per_length == 0 {
            return Err(RandomizedBenchmarkingError::ZeroSequencesPerLength);
        }

        if self.shots_per_sequence == 0 {
            return Err(RandomizedBenchmarkingError::ZeroShotsPerSequence);
        }

        if self.dimension < 2 {
            return Err(RandomizedBenchmarkingError::InvalidDimension {
                dimension: self.dimension,
            });
        }

        Ok(())
    }

    /// Number of randomized sequences in the experiment.
    pub fn total_sequences(&self) -> RandomizedBenchmarkingResult<u128> {
        (self.sequence_lengths.len() as u128)
            .checked_mul(self.sequences_per_length as u128)
            .ok_or(RandomizedBenchmarkingError::WorkloadOverflow)
    }

    /// Total number of shots in the reference experiment.
    pub fn total_shots(&self) -> RandomizedBenchmarkingResult<u128> {
        self.total_sequences()?
            .checked_mul(self.shots_per_sequence as u128)
            .ok_or(RandomizedBenchmarkingError::WorkloadOverflow)
    }

    /// Validates this experiment against generic ZQN resource limits.
    pub fn validate_against_limits(
        &self,
        limits: &CharacterizationLimits,
    ) -> RandomizedBenchmarkingResult<()> {
        self.validate()?;
        limits
            .validate()
            .map_err(|_| RandomizedBenchmarkingError::InvalidConfiguration {
                reason: "invalid characterization limits",
            })?;

        let total_sequences = self.total_sequences()?;
        let total_shots = self.total_shots()?;

        if let Some(maximum) = limits.max_experiments {
            if total_sequences > maximum as u128 {
                return Err(RandomizedBenchmarkingError::ResourceLimitExceeded {
                    resource: "sequences",
                    requested: total_sequences,
                    maximum: maximum as u128,
                });
            }
        }

        if let Some(maximum) = limits.max_repetitions {
            if self.shots_per_sequence > maximum {
                return Err(RandomizedBenchmarkingError::ResourceLimitExceeded {
                    resource: "shots per sequence",
                    requested: self.shots_per_sequence as u128,
                    maximum: maximum as u128,
                });
            }
        }

        if let Some(maximum) = limits.max_sequence_length {
            for &length in &self.sequence_lengths {
                if length > maximum {
                    return Err(RandomizedBenchmarkingError::ResourceLimitExceeded {
                        resource: "sequence length",
                        requested: length as u128,
                        maximum: maximum as u128,
                    });
                }
            }
        }

        if let Some(maximum) = limits.max_observations {
            if total_sequences > maximum as u128 {
                return Err(RandomizedBenchmarkingError::ResourceLimitExceeded {
                    resource: "observations",
                    requested: total_sequences,
                    maximum: maximum as u128,
                });
            }
        }

        let _ = total_shots;

        Ok(())
    }
}

// ============================================================================
// Deterministic sequence planning
// ============================================================================

/// Deterministic description of one RB sequence.
///
/// This is not a circuit.
///
/// The downstream Clifford generator uses the identity and length to produce
/// the actual canonical Quantum IR workload.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RbSequencePlan {
    /// Sequence length in randomized Clifford operations.
    pub length: u64,

    /// Independent sample index at this length.
    pub sample_index: u64,

    /// Stable deterministic sequence identity.
    pub sequence_id: u128,

    /// Whether the generated sequence requires an ideal recovery operation.
    pub requires_recovery: bool,
}

impl RbSequencePlan {
    /// Creates a deterministic sequence plan.
    pub fn new(
        seed: u128,
        length: u64,
        sample_index: u64,
    ) -> Self {
        Self {
            length,
            sample_index,
            sequence_id: derive_sequence_id(seed, length, sample_index),
            requires_recovery: true,
        }
    }
}

/// Deterministically produces an RB sequence identity.
///
/// This is a stable non-cryptographic identity function. It is NOT intended
/// to provide cryptographic randomness.
fn derive_sequence_id(
    seed: u128,
    length: u64,
    sample_index: u64,
) -> u128 {
    let mut low = seed as u64;
    let mut high = (seed >> 64) as u64;

    low ^= length.wrapping_mul(0x9E37_79B9_7F4A_7C15);
    high ^= sample_index.wrapping_mul(0xBF58_476D_1CE4_E5B9);

    low = split_mix64(low);
    high = split_mix64(high ^ low);

    ((high as u128) << 64) | low as u128
}

fn split_mix64(mut value: u64) -> u64 {
    value = value.wrapping_add(0x9E37_79B9_7F4A_7C15);

    let mut result = value;

    result = (result ^ (result >> 30))
        .wrapping_mul(0xBF58_476D_1CE4_E5B9);

    result = (result ^ (result >> 27))
        .wrapping_mul(0x94D0_49BB_1331_11EB);

    result ^ (result >> 31)
}

/// Produces sequence plans without materializing circuits.
pub fn plan_sequences(
    configuration: &RandomizedBenchmarkingConfig,
) -> RandomizedBenchmarkingResult<Vec<RbSequencePlan>> {
    configuration.validate()?;

    let total = configuration.total_sequences()?;

    let capacity = usize::try_from(total).map_err(|_| {
        RandomizedBenchmarkingError::ResourceLimitExceeded {
            resource: "materialized sequence plans",
            requested: total,
            maximum: usize::MAX as u128,
        }
    })?;

    let mut plans = Vec::with_capacity(capacity);

    for &length in &configuration.sequence_lengths {
        for sample_index in 0..configuration.sequences_per_length {
            plans.push(RbSequencePlan::new(
                configuration.seed,
                length,
                sample_index,
            ));
        }
    }

    Ok(plans)
}

// ============================================================================
// Observations
// ============================================================================

/// One aggregated observation for one randomized sequence.
///
/// The observation layer may convert raw shot observations into this compact
/// representation before passing it to this module.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RbObservation {
    /// Stable sequence identity.
    pub sequence_id: u128,

    /// Randomized sequence length.
    pub sequence_length: u64,

    /// Number of shots returning the designated survival outcome.
    pub successful_shots: u64,

    /// Total shots.
    pub shots: u64,
}

impl RbObservation {
    /// Validates the observation.
    pub fn validate(&self) -> RandomizedBenchmarkingResult<()> {
        if self.sequence_length == 0 {
            return Err(RandomizedBenchmarkingError::InvalidSequenceLength {
                length: self.sequence_length,
            });
        }

        if self.shots == 0 || self.successful_shots > self.shots {
            return Err(RandomizedBenchmarkingError::InvalidObservation {
                sequence_length: self.sequence_length,
                successes: self.successful_shots,
                shots: self.shots,
            });
        }

        Ok(())
    }

    /// Returns the survival probability.
    pub fn survival_probability(&self) -> RandomizedBenchmarkingResult<f64> {
        self.validate()?;

        let probability =
            self.successful_shots as f64 / self.shots as f64;

        validate_probability("survival probability", probability)?;

        Ok(probability)
    }
}

/// Aggregated observation for one sequence length.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RbLengthAggregate {
    /// Sequence length.
    pub sequence_length: u64,

    /// Total successful shots.
    pub successful_shots: u128,

    /// Total shots.
    pub shots: u128,

    /// Number of independently randomized sequences contributing.
    pub sequences: u64,
}

impl RbLengthAggregate {
    /// Returns the aggregate survival probability.
    pub fn survival_probability(&self) -> RandomizedBenchmarkingResult<f64> {
        if self.shots == 0 || self.successful_shots > self.shots {
            return Err(RandomizedBenchmarkingError::InvalidConfiguration {
                reason: "invalid aggregated RB shot counts",
            });
        }

        let probability =
            self.successful_shots as f64 / self.shots as f64;

        validate_probability("aggregate survival probability", probability)?;

        Ok(probability)
    }
}

// ============================================================================
// Streaming accumulator
// ============================================================================

/// Streaming RB observation accumulator.
///
/// It stores only aggregate information by sequence length.
///
/// It does not retain all shots and therefore scales with the number of
/// distinct sequence lengths rather than the total number of shots.
#[derive(Debug, Clone, Default)]
pub struct RbAccumulator {
    aggregates: BTreeMap<u64, RbLengthAggregate>,
}

impl RbAccumulator {
    /// Creates an empty accumulator.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds one observation.
    pub fn push(
        &mut self,
        observation: RbObservation,
    ) -> RandomizedBenchmarkingResult<()> {
        observation.validate()?;

        let entry = self
            .aggregates
            .entry(observation.sequence_length)
            .or_insert(RbLengthAggregate {
                sequence_length: observation.sequence_length,
                successful_shots: 0,
                shots: 0,
                sequences: 0,
            });

        entry.successful_shots = entry
            .successful_shots
            .checked_add(observation.successful_shots as u128)
            .ok_or(RandomizedBenchmarkingError::WorkloadOverflow)?;

        entry.shots = entry
            .shots
            .checked_add(observation.shots as u128)
            .ok_or(RandomizedBenchmarkingError::WorkloadOverflow)?;

        entry.sequences = entry
            .sequences
            .checked_add(1)
            .ok_or(RandomizedBenchmarkingError::WorkloadOverflow)?;

        Ok(())
    }

    /// Merges another accumulator.
    ///
    /// The caller controls merge order. For reproducible parallel analysis,
    /// merge partitions in a stable deterministic order.
    pub fn merge(
        &mut self,
        other: &Self,
    ) -> RandomizedBenchmarkingResult<()> {
        for aggregate in other.aggregates.values() {
            let entry = self
                .aggregates
                .entry(aggregate.sequence_length)
                .or_insert(RbLengthAggregate {
                    sequence_length: aggregate.sequence_length,
                    successful_shots: 0,
                    shots: 0,
                    sequences: 0,
                });

            entry.successful_shots = entry
                .successful_shots
                .checked_add(aggregate.successful_shots)
                .ok_or(RandomizedBenchmarkingError::WorkloadOverflow)?;

            entry.shots = entry
                .shots
                .checked_add(aggregate.shots)
                .ok_or(RandomizedBenchmarkingError::WorkloadOverflow)?;

            entry.sequences = entry
                .sequences
                .checked_add(aggregate.sequences)
                .ok_or(RandomizedBenchmarkingError::WorkloadOverflow)?;
        }

        Ok(())
    }

    /// Returns deterministic length-sorted aggregates.
    pub fn aggregates(&self) -> impl Iterator<Item = &RbLengthAggregate> {
        self.aggregates.values()
    }

    /// Returns the number of distinct sequence lengths.
    pub fn len(&self) -> usize {
        self.aggregates.len()
    }

    /// Returns whether no sequence lengths have been accumulated.
    pub fn is_empty(&self) -> bool {
        self.aggregates.is_empty()
    }

    /// Converts the accumulator into a sorted vector.
    pub fn into_sorted(self) -> Vec<RbLengthAggregate> {
        self.aggregates.into_values().collect()
    }
}

// ============================================================================
// Fit configuration
// ============================================================================

/// Weighting policy for the exponential fit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RbWeighting {
    /// Every sequence length contributes equally.
    EqualLength,

    /// Lengths with more shots receive proportionally greater weight.
    ShotCount,
}

/// Numerical fitting configuration.
///
/// These are numerical-analysis policies, not machine-size limits.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RbFitConfig {
    /// Number of deterministic points used to locate candidate decay regions.
    ///
    /// Larger values improve global search coverage at increased analysis
    /// cost.
    pub grid_points: usize,

    /// Number of deterministic golden-section refinement iterations.
    pub refinement_iterations: u32,

    /// Lower decay parameter bound.
    ///
    /// `None` derives the physical depolarizing lower bound
    /// `-1 / (d^2 - 1)`.
    pub decay_lower: Option<f64>,

    /// Upper decay parameter bound.
    pub decay_upper: f64,

    /// Numerical pivot tolerance.
    pub pivot_tolerance: f64,

    /// Optional condition-number warning threshold.
    pub condition_warning_limit: Option<f64>,

    /// Weighting policy.
    pub weighting: RbWeighting,

    /// Reject fits whose predicted survival probabilities leave [0,1].
    pub strict_physical_predictions: bool,
}

impl Default for RbFitConfig {
    fn default() -> Self {
        Self {
            grid_points: 257,
            refinement_iterations: 64,
            decay_lower: None,
            decay_upper: 1.0,
            pivot_tolerance: 1.0e-14,
            condition_warning_limit: Some(1.0e12),
            weighting: RbWeighting::ShotCount,
            strict_physical_predictions: false,
        }
    }
}

impl RbFitConfig {
    /// Validates fitting parameters.
    pub fn validate(
        &self,
        dimension: u128,
    ) -> RandomizedBenchmarkingResult<()> {
        if self.grid_points < 8 {
            return Err(RandomizedBenchmarkingError::InvalidConfiguration {
                reason: "RB decay grid requires at least eight points",
            });
        }

        if self.refinement_iterations == 0 {
            return Err(RandomizedBenchmarkingError::InvalidConfiguration {
                reason: "RB refinement requires at least one iteration",
            });
        }

        if !self.pivot_tolerance.is_finite()
            || self.pivot_tolerance <= 0.0
        {
            return Err(RandomizedBenchmarkingError::InvalidConfiguration {
                reason: "RB pivot tolerance must be positive and finite",
            });
        }

        if !self.decay_upper.is_finite()
            || self.decay_upper > 1.0
        {
            return Err(RandomizedBenchmarkingError::InvalidDecayBounds);
        }

        let lower = self.lower_bound(dimension)?;

        if !lower.is_finite()
            || lower >= self.decay_upper
        {
            return Err(RandomizedBenchmarkingError::InvalidDecayBounds);
        }

        if let Some(limit) = self.condition_warning_limit {
            if !limit.is_finite() || limit <= 1.0 {
                return Err(RandomizedBenchmarkingError::InvalidConfiguration {
                    reason: "RB condition warning limit must be > 1",
                });
            }
        }

        Ok(())
    }

    /// Returns the effective lower decay bound.
    pub fn lower_bound(
        &self,
        dimension: u128,
    ) -> RandomizedBenchmarkingResult<f64> {
        if dimension < 2 {
            return Err(RandomizedBenchmarkingError::InvalidDimension {
                dimension,
            });
        }

        match self.decay_lower {
            Some(value) => {
                if !value.is_finite() {
                    return Err(RandomizedBenchmarkingError::NonFiniteValue {
                        field: "decay lower bound",
                    });
                }

                Ok(value)
            }

            None => {
                let d = dimension as f64;
                let denominator = d * d - 1.0;

                if !denominator.is_finite() || denominator <= 0.0 {
                    return Err(RandomizedBenchmarkingError::InvalidDimension {
                        dimension,
                    });
                }

                Ok(-1.0 / denominator)
            }
        }
    }
}

// ============================================================================
// Fit structures
// ============================================================================

/// Parameters of the fitted RB decay model.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RbFit {
    /// SPAM/amplitude parameter.
    pub amplitude: f64,

    /// Decay parameter.
    pub decay: f64,

    /// Offset parameter.
    pub offset: f64,

    /// Weighted sum of squared residuals.
    pub weighted_sse: f64,

    /// Unweighted sum of squared residuals.
    pub sse: f64,

    /// Degrees of freedom.
    pub degrees_of_freedom: usize,

    /// Approximate rank of the local Jacobian.
    pub rank: usize,

    /// Pivot-ratio condition estimate.
    pub condition_estimate: Option<f64>,

    /// Approximate standard error of amplitude.
    pub amplitude_standard_error: Option<f64>,

    /// Approximate standard error of decay.
    pub decay_standard_error: Option<f64>,

    /// Approximate standard error of offset.
    pub offset_standard_error: Option<f64>,
}

/// RB-derived error-per-Clifford estimate.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RbErrorEstimate {
    /// Effective Hilbert-space dimension.
    pub dimension: u128,

    /// Decay parameter used.
    pub decay: f64,

    /// Error-per-Clifford estimate.
    pub error_per_clifford: f64,

    /// First-order propagated standard error.
    pub standard_error: Option<f64>,
}

/// Scientific warning emitted by RB analysis.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RbWarning {
    /// Optimum is at or extremely close to the configured decay boundary.
    DecayAtSearchBoundary,

    /// Local regression is poorly conditioned.
    PoorConditioning,

    /// The fitted model predicts survival probabilities outside [0,1].
    PhysicalPredictionViolation,

    /// There are no residual degrees of freedom.
    NoResidualDegreesOfFreedom,

    /// The numerical search is an approximation.
    NumericalOptimizationApproximation,
}

/// Complete standard-RB analysis result.
#[derive(Debug, Clone, PartialEq)]
pub struct RbAnalysisResult {
    /// Schema version.
    pub schema_version: u32,

    /// Scientific algorithm identifier.
    pub algorithm_id: &'static str,

    /// Number of distinct sequence lengths.
    pub sequence_lengths: usize,

    /// Total successful shots.
    pub successful_shots: u128,

    /// Total shots.
    pub total_shots: u128,

    /// Total independently randomized sequences.
    pub sequences: u64,

    /// Fitted decay.
    pub fit: RbFit,

    /// RB-derived EPC estimate.
    pub error_per_clifford: RbErrorEstimate,

    /// Scientific diagnostics.
    pub warnings: Vec<RbWarning>,
}

// ============================================================================
// Analysis
// ============================================================================

/// Performs standard RB analysis from streaming observations.
///
/// This function never needs individual shot observations after they have
/// been aggregated into `RbObservation`.
pub fn analyze(
    configuration: &RandomizedBenchmarkingConfig,
    accumulator: &RbAccumulator,
    fit_config: &RbFitConfig,
) -> RandomizedBenchmarkingResult<RbAnalysisResult> {
    configuration.validate()?;
    fit_config.validate(configuration.dimension)?;

    if accumulator.is_empty() {
        return Err(RandomizedBenchmarkingError::EmptyObservations);
    }

    let aggregates = accumulator.into_sorted();

    for aggregate in &aggregates {
        if !configuration
            .sequence_lengths
            .contains(&aggregate.sequence_length)
        {
            return Err(
                RandomizedBenchmarkingError::ObservationLengthNotConfigured {
                    length: aggregate.sequence_length,
                },
            );
        }
    }

    if aggregates.len() < 3 {
        return Err(
            RandomizedBenchmarkingError::InsufficientSequenceLengths {
                available: aggregates.len(),
                required: 3,
            },
        );
    }

    let mut points = Vec::with_capacity(aggregates.len());

    let mut successful_shots = 0_u128;
    let mut total_shots = 0_u128;
    let mut sequences = 0_u64;

    for aggregate in aggregates {
        let survival = aggregate.survival_probability()?;

        let weight = match fit_config.weighting {
            RbWeighting::EqualLength => 1.0,
            RbWeighting::ShotCount => aggregate.shots as f64,
        };

        validate_weight(weight)?;

        points.push(FitPoint {
            length: aggregate.sequence_length,
            survival,
            weight,
        });

        successful_shots = successful_shots
            .checked_add(aggregate.successful_shots)
            .ok_or(RandomizedBenchmarkingError::WorkloadOverflow)?;

        total_shots = total_shots
            .checked_add(aggregate.shots)
            .ok_or(RandomizedBenchmarkingError::WorkloadOverflow)?;

        sequences = sequences
            .checked_add(aggregate.sequences)
            .ok_or(RandomizedBenchmarkingError::WorkloadOverflow)?;
    }

    let fit = fit_decay(
        &points,
        configuration.dimension,
        fit_config,
    )?;

    let error_per_clifford =
        calculate_error_per_clifford(configuration.dimension, &fit)?;

    let mut warnings = Vec::new();

    if fit.condition_estimate.is_none()
        || fit.condition_estimate
            .is_some_and(|value| {
                fit_config
                    .condition_warning_limit
                    .is_some_and(|limit| value > limit)
            })
    {
        warnings.push(RbWarning::PoorConditioning);
    }

    if fit.degrees_of_freedom == 0 {
        warnings.push(RbWarning::NoResidualDegreesOfFreedom);
    }

    if fit.predictions_outside_unit_interval(&points) {
        warnings.push(RbWarning::PhysicalPredictionViolation);

        if fit_config.strict_physical_predictions {
            return Err(
                RandomizedBenchmarkingError::InvalidPhysicalResult {
                    value: fit.decay,
                },
            );
        }
    }

    let lower = fit_config.lower_bound(configuration.dimension)?;
    let upper = fit_config.decay_upper;

    if (fit.decay - lower).abs() <= fit_config.pivot_tolerance
        || (upper - fit.decay).abs() <= fit_config.pivot_tolerance
    {
        warnings.push(RbWarning::DecayAtSearchBoundary);
    }

    warnings.push(RbWarning::NumericalOptimizationApproximation);

    Ok(RbAnalysisResult {
        schema_version: RANDOMIZED_BENCHMARKING_SCHEMA_VERSION,
        algorithm_id: RANDOMIZED_BENCHMARKING_ALGORITHM_ID,
        sequence_lengths: points.len(),
        successful_shots,
        total_shots,
        sequences,
        fit,
        error_per_clifford,
        warnings,
    })
}

#[derive(Debug, Clone, Copy)]
struct FitPoint {
    length: u64,
    survival: f64,
    weight: f64,
}

impl RbFit {
    fn predictions_outside_unit_interval(
        &self,
        points: &[FitPoint],
    ) -> bool {
        points.iter().any(|point| {
            let prediction =
                self.amplitude * pow_u64(self.decay, point.length)
                    + self.offset;

            prediction < -1.0e-10 || prediction > 1.0 + 1.0e-10
        })
    }
}

// ============================================================================
// Profiled nonlinear least-squares fitting
// ============================================================================

fn fit_decay(
    points: &[FitPoint],
    dimension: u128,
    configuration: &RbFitConfig,
) -> RandomizedBenchmarkingResult<RbFit> {
    if points.len() < 3 {
        return Err(
            RandomizedBenchmarkingError::InsufficientSequenceLengths {
                available: points.len(),
                required: 3,
            },
        );
    }

    let lower = configuration.lower_bound(dimension)?;
    let upper = configuration.decay_upper;

    let span = upper - lower;

    if !span.is_finite() || span <= 0.0 {
        return Err(RandomizedBenchmarkingError::InvalidDecayBounds);
    }

    let grid_count = configuration.grid_points;

    let mut best: Option<CandidateFit> = None;
    let mut best_grid_index = 0_usize;

    for index in 0..grid_count {
        let fraction =
            index as f64 / (grid_count - 1) as f64;

        let decay = lower + span * fraction;

        if let Some(candidate) =
            evaluate_candidate(points, decay, configuration)?
        {
            if best
                .as_ref()
                .is_none_or(|current| {
                    candidate.weighted_sse < current.weighted_sse
                })
            {
                best = Some(candidate);
                best_grid_index = index;
            }
        }
    }

    let best = best.ok_or(RandomizedBenchmarkingError::FitFailure {
        operation: "global decay search",
    })?;

    let mut refined = best;

    if best_grid_index > 0
        && best_grid_index + 1 < grid_count
    {
        let left =
            lower + span * ((best_grid_index - 1) as f64)
                / ((grid_count - 1) as f64);

        let right =
            lower + span * ((best_grid_index + 1) as f64)
                / ((grid_count - 1) as f64);

        refined = golden_section_search(
            points,
            left,
            right,
            configuration,
            best,
        )?;
    }

    let covariance =
        estimate_parameter_uncertainty(points, &refined, configuration)?;

    let degrees_of_freedom =
        points.len().saturating_sub(3);

    let condition_estimate =
        covariance.as_ref().and_then(|value| value.condition_estimate);

    let (amplitude_standard_error, decay_standard_error, offset_standard_error) =
        covariance
            .map(|value| {
                (
                    value.standard_errors[0],
                    value.standard_errors[1],
                    value.standard_errors[2],
                )
            })
            .unwrap_or((None, None, None));

    Ok(RbFit {
        amplitude: refined.amplitude,
        decay: refined.decay,
        offset: refined.offset,
        weighted_sse: refined.weighted_sse,
        sse: refined.sse,
        degrees_of_freedom,
        rank: refined.rank,
        condition_estimate,
        amplitude_standard_error,
        decay_standard_error,
        offset_standard_error,
    })
}

#[derive(Debug, Clone, Copy)]
struct CandidateFit {
    amplitude: f64,
    decay: f64,
    offset: f64,
    weighted_sse: f64,
    sse: f64,
    rank: usize,
    condition_estimate: Option<f64>,
}

fn evaluate_candidate(
    points: &[FitPoint],
    decay: f64,
    configuration: &RbFitConfig,
) -> RandomizedBenchmarkingResult<Option<CandidateFit>> {
    if !decay.is_finite() {
        return Ok(None);
    }

    let mut s_xx = 0.0;
    let mut s_x1 = 0.0;
    let mut s_11 = 0.0;
    let mut s_xy = 0.0;
    let mut s_1y = 0.0;

    for point in points {
        let x = pow_u64(decay, point.length);

        if !x.is_finite() {
            return Ok(None);
        }

        s_xx += point.weight * x * x;
        s_x1 += point.weight * x;
        s_11 += point.weight;
        s_xy += point.weight * x * point.survival;
        s_1y += point.weight * point.survival;
    }

    let determinant =
        s_xx * s_11 - s_x1 * s_x1;

    let scale =
        s_xx.abs().max(s_11.abs()).max(1.0);

    if !determinant.is_finite()
        || determinant.abs()
            <= configuration.pivot_tolerance * scale
    {
        return Ok(None);
    }

    let amplitude =
        (s_xy * s_11 - s_1y * s_x1)
            / determinant;

    let offset =
        (s_xx * s_1y - s_x1 * s_xy)
            / determinant;

    if !amplitude.is_finite() || !offset.is_finite() {
        return Ok(None);
    }

    let mut weighted_sse = 0.0;
    let mut sse = 0.0;

    for point in points {
        let prediction =
            amplitude * pow_u64(decay, point.length)
                + offset;

        let residual =
            point.survival - prediction;

        weighted_sse += point.weight * residual * residual;
        sse += residual * residual;
    }

    if !weighted_sse.is_finite()
        || !sse.is_finite()
    {
        return Ok(None);
    }

    let rank = 3;

    Ok(Some(CandidateFit {
        amplitude,
        decay,
        offset,
        weighted_sse,
        sse,
        rank,
        condition_estimate: None,
    }))
}

fn golden_section_search(
    points: &[FitPoint],
    mut left: f64,
    mut right: f64,
    configuration: &RbFitConfig,
    initial: CandidateFit,
) -> RandomizedBenchmarkingResult<CandidateFit> {
    let golden_ratio = 0.618_033_988_749_894_9_f64;

    let mut x1 =
        right - golden_ratio * (right - left);

    let mut x2 =
        left + golden_ratio * (right - left);

    let mut f1 =
        evaluate_candidate(points, x1, configuration)?;

    let mut f2 =
        evaluate_candidate(points, x2, configuration)?;

    let mut best = initial;

    for _ in 0..configuration.refinement_iterations {
        if let Some(candidate) = f1 {
            if candidate.weighted_sse < best.weighted_sse {
                best = candidate;
            }
        }

        if let Some(candidate) = f2 {
            if candidate.weighted_sse < best.weighted_sse {
                best = candidate;
            }
        }

        if (right - left).abs()
            <= configuration.pivot_tolerance
                * (1.0 + left.abs().max(right.abs()))
        {
            break;
        }

        let score1 =
            f1.as_ref().map(|candidate| candidate.weighted_sse);

        let score2 =
            f2.as_ref().map(|candidate| candidate.weighted_sse);

        match (score1, score2) {
            (Some(first), Some(second)) if first <= second => {
                right = x2;
                x2 = x1;
                f2 = f1;
                x1 =
                    right - golden_ratio * (right - left);

                f1 =
                    evaluate_candidate(points, x1, configuration)?;
            }

            (Some(_), Some(_)) => {
                left = x1;
                x1 = x2;
                f1 = f2;
                x2 =
                    left + golden_ratio * (right - left);

                f2 =
                    evaluate_candidate(points, x2, configuration)?;
            }

            (Some(_), None) => {
                right = x2;
                x2 = x1;
                f2 = f1;
                x1 =
                    right - golden_ratio * (right - left);

                f1 =
                    evaluate_candidate(points, x1, configuration)?;
            }

            (None, Some(_)) => {
                left = x1;
                x1 = x2;
                f1 = f2;
                x2 =
                    left + golden_ratio * (right - left);

                f2 =
                    evaluate_candidate(points, x2, configuration)?;
            }

            (None, None) => break,
        }
    }

    Ok(best)
}

// ============================================================================
// Parameter uncertainty
// ============================================================================

#[derive(Debug, Clone, Copy)]
struct CovarianceEstimate {
    standard_errors: [Option<f64>; 3],
    condition_estimate: Option<f64>,
}

fn estimate_parameter_uncertainty(
    points: &[FitPoint],
    candidate: &CandidateFit,
    configuration: &RbFitConfig,
) -> RandomizedBenchmarkingResult<Option<CovarianceEstimate>> {
    if points.len() <= 3 {
        return Ok(None);
    }

    let mut normal = [[0.0_f64; 3]; 3];

    for point in points {
        let x =
            pow_u64(candidate.decay, point.length);

        let derivative_decay =
            derivative_decay(
                candidate.amplitude,
                candidate.decay,
                point.length,
            )?;

        let jacobian = [x, derivative_decay, 1.0];

        for row in 0..3 {
            for column in 0..3 {
                normal[row][column] +=
                    point.weight
                        * jacobian[row]
                        * jacobian[column];
            }
        }
    }

    let mut inverse_diagonal = [None; 3];
    let mut minimum_pivot = f64::INFINITY;
    let mut maximum_pivot = 0.0_f64;

    for column in 0..3 {
        let mut rhs = [0.0_f64; 3];
        rhs[column] = 1.0;

        let solved =
            solve_3x3(
                normal,
                rhs,
                configuration.pivot_tolerance,
            );

        let (solution, pivots) = match solved {
            Some(value) => value,
            None => return Ok(None),
        };

        for pivot in pivots {
            minimum_pivot =
                minimum_pivot.min(pivot.abs());

            maximum_pivot =
                maximum_pivot.max(pivot.abs());
        }

        inverse_diagonal[column] =
            Some(solution[column]);
    }

    let condition_estimate =
        if minimum_pivot.is_finite()
            && minimum_pivot > 0.0
            && maximum_pivot.is_finite()
        {
            Some(maximum_pivot / minimum_pivot)
        } else {
            None
        };

    let variance =
        candidate.weighted_sse
            / (points.len() - 3) as f64;

    if !variance.is_finite()
        || variance < 0.0
    {
        return Ok(None);
    }

    let standard_errors = inverse_diagonal.map(|diagonal| {
        diagonal
            .and_then(|value| {
                let scaled = variance * value;

                if scaled.is_finite() && scaled >= 0.0 {
                    Some(scaled.sqrt())
                } else {
                    None
                }
            })
    });

    Ok(Some(CovarianceEstimate {
        standard_errors,
        condition_estimate,
    }))
}

fn derivative_decay(
    amplitude: f64,
    decay: f64,
    length: u64,
) -> RandomizedBenchmarkingResult<f64> {
    if length == 0 {
        return Err(RandomizedBenchmarkingError::InvalidSequenceLength {
            length,
        });
    }

    let power = if length == 1 {
        1.0
    } else {
        pow_u64(decay, length - 1)
    };

    let derivative =
        amplitude * length as f64 * power;

    if !derivative.is_finite() {
        return Err(RandomizedBenchmarkingError::NonFiniteValue {
            field: "RB decay derivative",
        });
    }

    Ok(derivative)
}

// ============================================================================
// EPC
// ============================================================================

fn calculate_error_per_clifford(
    dimension: u128,
    fit: &RbFit,
) -> RandomizedBenchmarkingResult<RbErrorEstimate> {
    if dimension < 2 {
        return Err(RandomizedBenchmarkingError::InvalidDimension {
            dimension,
        });
    }

    if !fit.decay.is_finite() {
        return Err(RandomizedBenchmarkingError::NonFiniteFit);
    }

    let d = dimension as f64;

    let factor = (d - 1.0) / d;

    let error_per_clifford =
        factor * (1.0 - fit.decay);

    if !error_per_clifford.is_finite() {
        return Err(RandomizedBenchmarkingError::NonFiniteFit);
    }

    let standard_error =
        fit.decay_standard_error.map(|value| {
            factor * value
        });

    Ok(RbErrorEstimate {
        dimension,
        decay: fit.decay,
        error_per_clifford,
        standard_error,
    })
}

// ============================================================================
// Stable power
// ============================================================================

/// Computes `base^exponent` without converting a potentially large exponent
/// into a narrower integer type.
///
/// The implementation uses exponentiation by squaring.
///
/// For RB decay values in the configured physical interval this is bounded
/// numerically by the underlying floating-point representation.
fn pow_u64(
    mut base: f64,
    mut exponent: u64,
) -> f64 {
    let mut result = 1.0_f64;

    while exponent != 0 {
        if exponent & 1 == 1 {
            result *= base;
        }

        exponent >>= 1;

        if exponent != 0 {
            base *= base;
        }
    }

    result
}

// ============================================================================
// Small deterministic linear algebra
// ============================================================================

fn solve_3x3(
    mut matrix: [[f64; 3]; 3],
    mut rhs: [f64; 3],
    tolerance: f64,
) -> Option<([f64; 3], [f64; 3])> {
    let mut pivots = [0.0_f64; 3];

    for column in 0..3 {
        let mut pivot_row = column;
        let mut pivot_value =
            matrix[column][column].abs();

        for row in (column + 1)..3 {
            let candidate =
                matrix[row][column].abs();

            if candidate > pivot_value {
                pivot_value = candidate;
                pivot_row = row;
            }
        }

        if !pivot_value.is_finite()
            || pivot_value <= tolerance
        {
            return None;
        }

        if pivot_row != column {
            matrix.swap(column, pivot_row);
            rhs.swap(column, pivot_row);
        }

        let pivot =
            matrix[column][column];

        pivots[column] = pivot;

        for row in (column + 1)..3 {
            let factor =
                matrix[row][column] / pivot;

            if !factor.is_finite() {
                return None;
            }

            matrix[row][column] = 0.0;

            for next_column in (column + 1)..3 {
                matrix[row][next_column] -=
                    factor * matrix[column][next_column];
            }

            rhs[row] -= factor * rhs[column];
        }
    }

    let mut solution = [0.0_f64; 3];

    for row in (0..3).rev() {
        let mut value = rhs[row];

        for column in (row + 1)..3 {
            value -=
                matrix[row][column]
                    * solution[column];
        }

        let diagonal =
            matrix[row][row];

        if !diagonal.is_finite()
            || diagonal.abs() <= tolerance
        {
            return None;
        }

        solution[row] =
            value / diagonal;

        if !solution[row].is_finite() {
            return None;
        }
    }

    Some((solution, pivots))
}

// ============================================================================
// Validation helpers
// ============================================================================

fn validate_probability(
    field: &'static str,
    value: f64,
) -> RandomizedBenchmarkingResult<()> {
    if !value.is_finite() {
        return Err(RandomizedBenchmarkingError::NonFiniteValue {
            field,
        });
    }

    if value < 0.0 || value > 1.0 {
        return Err(RandomizedBenchmarkingError::InvalidProbability {
            field,
            value,
        });
    }

    Ok(())
}

fn validate_weight(
    value: f64,
) -> RandomizedBenchmarkingResult<()> {
    if !value.is_finite() || value <= 0.0 {
        return Err(RandomizedBenchmarkingError::InvalidWeight {
            value,
        });
    }

    Ok(())
}

// ============================================================================
// CharacterizationProtocol integration
// ============================================================================

/// Concrete ZQN standard randomized-benchmarking protocol.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RandomizedBenchmarkingProtocol {
    /// Protocol configuration.
    pub configuration: RandomizedBenchmarkingConfig,
}

impl RandomizedBenchmarkingProtocol {
    /// Creates the protocol.
    pub fn new(
        configuration: RandomizedBenchmarkingConfig,
    ) -> RandomizedBenchmarkingResult<Self> {
        configuration.validate()?;

        Ok(Self { configuration })
    }

    /// Validates against caller-provided resource policy.
    pub fn validate_with_limits(
        &self,
        limits: &CharacterizationLimits,
    ) -> RandomizedBenchmarkingResult<()> {
        self.configuration
            .validate_against_limits(limits)
    }

    /// Returns deterministic sequence metadata.
    pub fn sequence_plans(
        &self,
    ) -> RandomizedBenchmarkingResult<Vec<RbSequencePlan>> {
        plan_sequences(&self.configuration)
    }

    /// Analyzes an accumulated observation set.
    pub fn analyze(
        &self,
        accumulator: &RbAccumulator,
        fit_config: &RbFitConfig,
    ) -> RandomizedBenchmarkingResult<RbAnalysisResult> {
        analyze(
            &self.configuration,
            accumulator,
            fit_config,
        )
    }
}

impl CharacterizationProtocol for RandomizedBenchmarkingProtocol {
    fn descriptor(&self) -> ProtocolDescriptor {
        let id = ProtocolId::new(RANDOMIZED_BENCHMARKING_ID)
            .expect("the built-in RB protocol identifier is valid");

        ProtocolDescriptor {
            id,
            version: ProtocolVersion::new(
                1,
                0,
                0,
            ),
            objective: CharacterizationObjective::Process,
            scope: self.configuration.scope.protocol_scope(),
            name: String::from(
                "Standard Randomized Benchmarking",
            ),
            description: String::from(
                "Backend-independent randomized benchmarking \
                 for estimating exponential decay and \
                 error per randomized Clifford.",
            ),
        }
    }

    fn requirements(&self) -> CharacterizationRequirements {
        CharacterizationRequirements {
            resource_addressing: true,
            state_preparation: true,
            measurement: true,
            repeated_execution: true,
            mid_circuit_measurement: false,
            reset: false,
            timing: false,
            calibration: false,
            simultaneous_execution: false,
            reference_probabilities: false,
            process_access: false,
            dynamic_circuits: false,
        }
    }

    fn observation_requirements(&self) -> ObservationRequirements {
        ObservationRequirements {
            raw_measurements: true,
            per_shot_observations: false,
            timing: false,
            resource_identity: true,
            experiment_identity: true,
            calibration_identity: true,
            target_identity: true,
            randomness_provenance: true,
        }
    }

    fn analysis_contract(&self) -> AnalysisContract {
        AnalysisContract {
            quantity: String::from(
                "randomized-benchmarking decay and error-per-Clifford",
            ),
            requires_uncertainty: true,
            requires_raw_observations: true,
            scalar_result: false,
        }
    }

    fn randomness_contract(&self) -> RandomnessContract {
        RandomnessContract::deterministic(
            RANDOMIZED_BENCHMARKING_ALGORITHM_ID,
        )
        .expect("the built-in RB randomness domain is valid")
    }

    fn validate(
        &self,
        limits: &CharacterizationLimits,
    ) -> ProtocolResult<()> {
        self.validate_with_limits(limits)
            .map_err(map_rb_error_to_protocol_error)
    }

    fn plan(
        &self,
        limits: &CharacterizationLimits,
    ) -> ProtocolResult<ExperimentPlan> {
        self.validate(limits)?;

        let total_sequences =
            self.configuration
                .total_sequences()
                .map_err(map_rb_error_to_protocol_error)?;

        let quantities = vec![
            WorkloadQuantity::new(
                u64::try_from(total_sequences).map_err(|_| {
                    ProtocolError::InvalidResourceCount
                })?,
                "randomized_sequence",
            )?,
            WorkloadQuantity::new(
                self.configuration
                    .shots_per_sequence,
                "shot_per_sequence",
            )?,
        ];

        let plan = ExperimentPlan {
            protocol: self.descriptor(),
            scope: self.configuration.scope.protocol_scope(),
            quantities,
            requirements: self.requirements(),
            limits: limits.clone(),
            randomness: self.randomness_contract(),
        };

        plan.validate()?;

        Ok(plan)
    }
}

fn map_rb_error_to_protocol_error(
    error: RandomizedBenchmarkingError,
) -> ProtocolError {
    match error {
        RandomizedBenchmarkingError::ResourceLimitExceeded { .. } => {
            ProtocolError::ResourceLimitExceeded
        }

        RandomizedBenchmarkingError::InvalidConfiguration { .. }
        | RandomizedBenchmarkingError::EmptySequenceLengths
        | RandomizedBenchmarkingError::ZeroSequencesPerLength
        | RandomizedBenchmarkingError::ZeroShotsPerSequence
        | RandomizedBenchmarkingError::InvalidSequenceLength { .. }
        | RandomizedBenchmarkingError::DuplicateSequenceLength { .. }
        | RandomizedBenchmarkingError::InvalidDimension { .. }
        | RandomizedBenchmarkingError::WorkloadOverflow => {
            ProtocolError::InvalidConfiguration
        }

        _ => ProtocolError::InconsistentContract,
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn configuration() -> RandomizedBenchmarkingConfig {
        RandomizedBenchmarkingConfig::new(
            RbScope::TargetDefined,
            vec![1, 2, 4, 8, 16],
            8,
            100,
            42,
            2,
        )
        .expect("test configuration must be valid")
    }

    #[test]
    fn configuration_rejects_duplicate_lengths() {
        let result =
            RandomizedBenchmarkingConfig::new(
                RbScope::TargetDefined,
                vec![1, 2, 2],
                1,
                10,
                0,
                2,
            );

        assert!(matches!(
            result,
            Err(
                RandomizedBenchmarkingError::DuplicateSequenceLength {
                    length: 2
                }
            )
        ));
    }

    #[test]
    fn sequence_identity_is_deterministic() {
        let first =
            RbSequencePlan::new(42, 100, 7);

        let second =
            RbSequencePlan::new(42, 100, 7);

        assert_eq!(first, second);
    }

    #[test]
    fn changing_sequence_inputs_changes_identity() {
        let first =
            RbSequencePlan::new(42, 100, 7);

        let second =
            RbSequencePlan::new(42, 101, 7);

        assert_ne!(
            first.sequence_id,
            second.sequence_id
        );
    }

    #[test]
    fn accumulator_is_order_independent_for_exact_counts() {
        let mut first = RbAccumulator::new();

        first
            .push(RbObservation {
                sequence_id: 1,
                sequence_length: 1,
                successful_shots: 50,
                shots: 100,
            })
            .unwrap();

        first
            .push(RbObservation {
                sequence_id: 2,
                sequence_length: 1,
                successful_shots: 25,
                shots: 50,
            })
            .unwrap();

        let mut second = RbAccumulator::new();

        second
            .push(RbObservation {
                sequence_id: 2,
                sequence_length: 1,
                successful_shots: 25,
                shots: 50,
            })
            .unwrap();

        second
            .push(RbObservation {
                sequence_id: 1,
                sequence_length: 1,
                successful_shots: 50,
                shots: 100,
            })
            .unwrap();

        assert_eq!(
            first
                .aggregates()
                .next()
                .unwrap()
                .survival_probability()
                .unwrap(),
            second
                .aggregates()
                .next()
                .unwrap()
                .survival_probability()
                .unwrap()
        );
    }

    #[test]
    fn invalid_probability_is_rejected() {
        let result =
            validate_probability(
                "test",
                f64::NAN,
            );

        assert!(result.is_err());
    }

    #[test]
    fn zero_success_probability_is_valid() {
        let observation =
            RbObservation {
                sequence_id: 1,
                sequence_length: 1,
                successful_shots: 0,
                shots: 100,
            };

        assert_eq!(
            observation
                .survival_probability()
                .unwrap(),
            0.0
        );
    }

    #[test]
    fn perfect_success_probability_is_valid() {
        let observation =
            RbObservation {
                sequence_id: 1,
                sequence_length: 1,
                successful_shots: 100,
                shots: 100,
            };

        assert_eq!(
            observation
                .survival_probability()
                .unwrap(),
            1.0
        );
    }

    #[test]
    fn plan_is_resource_validated_before_materialization() {
        let protocol =
            RandomizedBenchmarkingProtocol::new(
                configuration(),
            )
            .unwrap();

        let mut limits =
            CharacterizationLimits::default();

        limits.max_experiments = Some(1);

        assert!(
            protocol
                .validate_with_limits(&limits)
                .is_err()
        );
    }

    #[test]
    fn standard_rb_can_recover_a_synthetic_decay() {
        let configuration =
            configuration();

        let true_amplitude = 0.5_f64;
        let true_decay = 0.93_f64;
        let true_offset = 0.5_f64;

        let mut accumulator =
            RbAccumulator::new();

        for &length
            in &configuration.sequence_lengths
        {
            let probability =
                true_amplitude
                    * pow_u64(
                        true_decay,
                        length,
                    )
                    + true_offset;

            let successes =
                (probability * 100_000.0)
                    as u64;

            accumulator
                .push(RbObservation {
                    sequence_id:
                        derive_sequence_id(
                            configuration.seed,
                            length,
                            0,
                        ),
                    sequence_length:
                        length,
                    successful_shots:
                        successes,
                    shots: 100_000,
                })
                .unwrap();
        }

        let result =
            analyze(
                &configuration,
                &accumulator,
                &RbFitConfig::default(),
            )
            .unwrap();

        assert!(
            (result.fit.decay - true_decay).abs()
                < 0.01,
            "fitted decay={}",
            result.fit.decay
        );
    }

    #[test]
    fn epc_formula_is_dimension_aware() {
        let fit =
            RbFit {
                amplitude: 0.5,
                decay: 0.9,
                offset: 0.5,
                weighted_sse: 0.0,
                sse: 0.0,
                degrees_of_freedom: 1,
                rank: 3,
                condition_estimate: None,
                amplitude_standard_error: None,
                decay_standard_error: None,
                offset_standard_error: None,
            };

        let result =
            calculate_error_per_clifford(
                2,
                &fit,
            )
            .unwrap();

        assert!(
            (result.error_per_clifford - 0.05).abs()
                < 1.0e-12
        );
    }

    #[test]
    fn power_function_handles_large_exponents_without_narrowing() {
        let value =
            pow_u64(0.999, u64::MAX);

        assert!(value.is_finite());
        assert!(value >= 0.0);
    }

    #[test]
    fn protocol_contract_is_valid() {
        let protocol =
            RandomizedBenchmarkingProtocol::new(
                configuration(),
            )
            .unwrap();

        let descriptor =
            protocol.descriptor();

        assert_eq!(
            descriptor.id.as_str(),
            RANDOMIZED_BENCHMARKING_ID
        );

        assert!(
            protocol
                .requirements()
                .measurement
        );

        assert!(
            protocol
                .randomness_contract()
                .replayable
        );
    }
}