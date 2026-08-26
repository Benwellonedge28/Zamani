//! Zamani Quantum Benchmarking — Cycle Benchmarking
//!
//! Production Cycle Benchmarking (CB) protocol.
//!
//! # Purpose
//!
//! This module defines the complete protocol-level contract for Cycle
//! Benchmarking without coupling the protocol to:
//!
//! - Quantum IR implementation details;
//! - a particular simulator;
//! - a particular QPU vendor;
//! - hardware networking;
//! - compiler implementation;
//! - routing;
//! - scheduling;
//! - reporting;
//! - the Zamani frontend;
//! - a particular statistical-framework implementation.
//!
//! The protocol is intentionally expressed through small, stable local
//! contracts so that the surrounding benchmarking architecture can integrate
//! with this file without requiring the protocol semantics to be rewritten.
//!
//! # What Cycle Benchmarking measures
//!
//! Cycle Benchmarking characterizes a complete quantum clock cycle:
//!
//! ```text
//!       random Pauli
//!            │
//!            ▼
//!       target cycle
//!            │
//!            ▼
//!       random Pauli
//!            │
//!            ▼
//!       target cycle
//!            │
//!           ...
//!            │
//!            ▼
//!       random Pauli
//! ```
//!
//! The target cycle may contain:
//!
//! - one-qubit gates;
//! - parallel one-qubit gates;
//! - two-qubit gates;
//! - parallel two-qubit gates;
//! - idle qubits;
//! - mixtures of the above;
//! - other operations supported by the backend's CB circuit adapter.
//!
//! # Canonical CB construction
//!
//! For a target cycle G and a Pauli preparation/measurement operator P,
//! the randomized circuit is represented as:
//!
//! ```text
//! R_m G R_(m-1) G ... R_1 G R_0
//! ```
//!
//! where there are `m + 1` random Pauli cycles.
//!
//! The executor is responsible for compiling this abstract description into
//! actual Quantum IR.
//!
//! # Statistical model
//!
//! The principal model is:
//!
//! ```text
//! f_P(m) = A_P * p_P^m
//! ```
//!
//! A diagnostic model with an explicit offset is also supported:
//!
//! ```text
//! f_P(m) = A_P * p_P^m + B_P
//! ```
//!
//! The `A * p^m` model is the preferred production model for standard CB.
//! The offset model is useful when analyzing externally supplied data or when
//! a protocol explicitly requires it.
//!
//! # Process fidelity
//!
//! For a complete Pauli basis, the process fidelity is related to the Pauli
//! fidelities by:
//!
//! ```text
//! F_process = [1 + Σ_(P != I) f_P] / d²
//! ```
//!
//! where:
//!
//! ```text
//! d = 2^n
//! ```
//!
//! For a uniformly sampled subset of non-identity Pauli operators, Zamani
//! computes the unbiased Monte-Carlo estimator:
//!
//! ```text
//! F_process_estimate
//!     = [1 + (d² - 1) * mean(f_P)] / d²
//! ```
//!
//! This estimate is NOT labelled "exact" unless the complete non-identity
//! Pauli basis was measured.
//!
//! The corresponding average gate fidelity is:
//!
//! ```text
//! F_average = (d * F_process + 1) / (d + 1)
//! ```
//!
//! and average gate infidelity is:
//!
//! ```text
//! 1 - F_average
//! ```
//!
//! # Important interpretation boundary
//!
//! Standard CB characterizes the Pauli-twirled/dressed cycle. It should not
//! silently be reported as the fidelity of the untwirled physical cycle.
//!
//! Inferring an isolated gate/cycle error by dividing against a reference
//! identity-cycle benchmark is possible, but that operation has additional
//! systematic assumptions and belongs in a higher-level interleaved/analysis
//! layer.
//!
//! # Production invariants
//!
//! This module guarantees:
//!
//! - explicit resource limits;
//! - non-zero shots;
//! - non-zero sequence lengths;
//! - strictly increasing sequence lengths;
//! - bounded randomization count;
//! - bounded Pauli count;
//! - bounded circuit count;
//! - deterministic experiment construction when supplied with the canonical
//!   benchmark RNG;
//! - explicit Pauli labels;
//! - no global mutable state;
//! - no logging;
//! - no direct printing;
//! - no unsafe code;
//! - checked dimension calculations;
//! - checked process-fidelity calculations;
//! - validation of every execution observation;
//! - rejection of NaN and infinity;
//! - rejection of impossible shot counts;
//! - rejection of non-finite fit parameters;
//! - explicit fit diagnostics;
//! - explicit protocol assumptions;
//! - distinction between sampled and exhaustive Pauli characterization.
//!
//! # Integration
//!
//! This file intentionally depends only on:
//!
//! - the Rust standard library;
//! - `generators::pauli`;
//! - `generators::random`.
//!
//! It does NOT depend directly on:
//!
//! - `quantum::ir`;
//! - `core::BenchmarkResult`;
//! - `core::Experiment`;
//! - `execution::executor`;
//! - `statistics::regression`;
//! - `metrics::fidelity`.
//!
//! Those modules can later provide adapters around the stable contracts here.
//!
//! The intended integration is:
//!
//! ```text
//! protocols::cycle_benchmarking
//!          │
//!          ├──────────────► generators::pauli
//!          ├──────────────► generators::random
//!          │
//!          ▼
//! CycleBenchmarkExperiment
//!          │
//!          ▼
//! execution::executor adapter
//!          │
//!          ▼
//! Quantum IR
//!          │
//!          ▼
//! simulator / hardware
//!          │
//!          ▼
//! CycleExecutionObservation
//!          │
//!          ▼
//! CycleBenchmarkProtocol::analyze
//!          │
//!          ▼
//! CycleBenchmarkResult
//!          │
//!          ├──────────────► core::BenchmarkResult adapter
//!          ├──────────────► metrics::fidelity adapter
//!          ├──────────────► reporting adapter
//!          └──────────────► registry adapter
//! ```
//!
//! This means later modules do not need to modify this file merely to connect
//! execution, reporting, registry, or Zamani-language integration.
//!
//! # Scientific assumptions
//!
//! Standard CB interpretation assumes, depending on the selected protocol
//! variant:
//!
//! - the target cycle is compatible with the chosen Pauli-twirling scheme;
//! - the cycle has an appropriate finite period when periodic sequence lengths
//!   are required;
//! - noise is sufficiently Markovian/stationary for a meaningful exponential
//!   decay model;
//! - the implementation of the randomizing Pauli cycles is characterized or
//!   otherwise accounted for;
//! - the selected Pauli subset represents the desired process-fidelity
//!   quantity;
//! - state-preparation and measurement basis changes are implemented
//!   consistently.
//!
//! The protocol records these assumptions instead of presenting them as
//! universally guaranteed facts.
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
//! No unsafe code is used.

#![deny(unsafe_code)]

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

use super::super::generators::pauli::{
    Pauli,
    PauliError,
};
use super::super::generators::random::{
    RandomError,
    RandomSource,
};

// =============================================================================
// Public protocol identity
// =============================================================================

/// Stable benchmark identifier.
pub const CYCLE_BENCHMARK_ID: &str = "cycle_benchmarking";

/// Semantic protocol version.
///
/// Any change to the generated circuit semantics, fitting model, Pauli
/// fidelity definition, or process-fidelity estimator requires a protocol
/// version change.
pub const CYCLE_BENCHMARK_PROTOCOL_VERSION: &str = "1.0.0";

/// Stable identifier for the canonical CB circuit construction.
pub const CYCLE_BENCHMARK_CIRCUIT_CONVENTION: &str =
    "pauli-twirled-cycle-r_m-g-r_m-1-g-r_0-v1";

/// Stable identifier for the preferred exponential model.
pub const CYCLE_BENCHMARK_FIT_MODEL: &str = "A*p^m";

/// Stable identifier for the optional diagnostic exponential model.
pub const CYCLE_BENCHMARK_OFFSET_FIT_MODEL: &str = "A*p^m+B";

/// Default number of random Pauli preparation/measurement bases.
pub const DEFAULT_PAULI_COUNT: usize = 20;

/// Default randomizations per Pauli/length point.
pub const DEFAULT_RANDOMIZATIONS_PER_LENGTH: usize = 30;

/// Default number of shots per randomized circuit.
pub const DEFAULT_SHOTS: usize = 1_000;

/// Default confidence level for reported uncertainty intervals.
pub const DEFAULT_CONFIDENCE_LEVEL: f64 = 0.95;

/// Default maximum number of qubits accepted by this protocol layer.
///
/// This is a safety ceiling, not a scientific limit. Hardware-specific
/// systems may impose a smaller limit.
pub const DEFAULT_MAX_QUBITS: usize = 4096;

/// Default maximum number of sequence lengths.
pub const DEFAULT_MAX_SEQUENCE_LENGTHS: usize = 128;

/// Default maximum number of random Pauli bases.
pub const DEFAULT_MAX_PAULIS: usize = 4096;

/// Default maximum number of randomizations per length.
pub const DEFAULT_MAX_RANDOMIZATIONS: usize = 100_000;

/// Default maximum number of shots per circuit.
pub const DEFAULT_MAX_SHOTS: usize = 10_000_000;

/// Default maximum generated circuit instances.
pub const DEFAULT_MAX_INSTANCES: usize = 10_000_000;

/// Maximum number of attempts used while constructing a unique random Pauli
/// set.
pub const DEFAULT_MAX_PAULI_SELECTION_ATTEMPTS: usize = 1_000_000;

/// Minimum number of distinct sequence lengths required for exponential fitting.
pub const MIN_FIT_SEQUENCE_LENGTHS: usize = 3;

/// Maximum number of iterations used by the deterministic one-dimensional
/// fit optimizer.
const MAX_FIT_ITERATIONS: usize = 96;

/// Numerical tolerance used for finite floating-point validation.
const FINITE_EPSILON: f64 = 1.0e-12;

// =============================================================================
// Fit model
// =============================================================================

/// Mathematical model used to fit a Pauli decay.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CycleFitModel {
    /// Standard CB model:
    ///
    /// `f(m) = A * p^m`.
    NoOffset,

    /// Diagnostic model:
    ///
    /// `f(m) = A * p^m + B`.
    ///
    /// This is not the preferred standard CB model because an unconstrained
    /// offset can increase uncertainty in the decay parameter.
    WithOffset,
}

impl CycleFitModel {
    /// Returns a stable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoOffset => CYCLE_BENCHMARK_FIT_MODEL,
            Self::WithOffset => CYCLE_BENCHMARK_OFFSET_FIT_MODEL,
        }
    }
}

impl Default for CycleFitModel {
    fn default() -> Self {
        Self::NoOffset
    }
}

// =============================================================================
// Protocol errors
// =============================================================================

/// Errors produced by Cycle Benchmarking.
#[derive(Debug, Clone, PartialEq)]
pub enum CycleBenchmarkError {
    /// Configuration is invalid.
    InvalidConfiguration {
        field: &'static str,
        reason: String,
    },

    /// A resource limit was exceeded.
    ResourceLimitExceeded {
        resource: &'static str,
        requested: usize,
        maximum: usize,
    },

    /// The number of qubits is invalid.
    InvalidQubitCount {
        qubits: usize,
    },

    /// A sequence length is invalid.
    InvalidSequenceLength {
        length: usize,
    },

    /// Sequence lengths were not strictly increasing.
    NonIncreasingSequenceLengths,

    /// The sequence length does not satisfy the requested cycle period.
    SequenceLengthNotMultipleOfPeriod {
        length: usize,
        period: usize,
    },

    /// A Pauli frame has the wrong number of qubits.
    PauliWidthMismatch {
        expected: usize,
        actual: usize,
    },

    /// A duplicate Pauli was supplied where uniqueness is required.
    DuplicatePauli {
        pauli: String,
    },

    /// Too many unique Pauli operators were requested.
    PauliSetExhausted {
        requested: usize,
        available: usize,
    },

    /// Random generation failed.
    Random(RandomError),

    /// Pauli generation/algebra failed.
    Pauli(PauliError),

    /// An execution observation is invalid.
    InvalidObservation {
        reason: String,
    },

    /// A fit cannot be performed.
    InsufficientFitData {
        pauli: String,
        lengths: usize,
    },

    /// The fit failed numerically.
    FitFailure {
        pauli: String,
        reason: String,
    },

    /// A fit generated an invalid parameter.
    InvalidFitParameter {
        pauli: String,
        parameter: &'static str,
        value: f64,
    },

    /// The Hilbert-space dimension overflowed.
    DimensionOverflow {
        qubits: usize,
    },

    /// A fidelity calculation produced an invalid value.
    InvalidFidelity {
        value: f64,
    },

    /// A process-fidelity estimate cannot be produced.
    ProcessFidelityUnavailable {
        reason: String,
    },

    /// The protocol received no observations.
    EmptyObservations,

    /// A requested execution item is missing.
    MissingObservation {
        instance_id: String,
    },

    /// A supplied observation appears more than once.
    DuplicateObservation {
        instance_id: String,
    },
}

impl fmt::Display for CycleBenchmarkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidConfiguration { field, reason } => {
                write!(f, "invalid Cycle Benchmarking configuration '{field}': {reason}")
            }

            Self::ResourceLimitExceeded {
                resource,
                requested,
                maximum,
            } => {
                write!(
                    f,
                    "Cycle Benchmarking resource '{resource}' exceeded: \
                     requested {requested}, maximum {maximum}"
                )
            }

            Self::InvalidQubitCount { qubits } => {
                write!(f, "invalid Cycle Benchmarking qubit count: {qubits}")
            }

            Self::InvalidSequenceLength { length } => {
                write!(f, "invalid Cycle Benchmarking sequence length: {length}")
            }

            Self::NonIncreasingSequenceLengths => {
                write!(
                    f,
                    "Cycle Benchmarking sequence lengths must be strictly increasing"
                )
            }

            Self::SequenceLengthNotMultipleOfPeriod { length, period } => {
                write!(
                    f,
                    "sequence length {length} is not a multiple of required cycle \
                     period {period}"
                )
            }

            Self::PauliWidthMismatch { expected, actual } => {
                write!(
                    f,
                    "Pauli frame width mismatch: expected {expected}, got {actual}"
                )
            }

            Self::DuplicatePauli { pauli } => {
                write!(f, "duplicate Cycle Benchmarking Pauli basis: {pauli}")
            }

            Self::PauliSetExhausted {
                requested,
                available,
            } => {
                write!(
                    f,
                    "requested {requested} unique Pauli bases but only \
                     {available} exist under the configured register size"
                )
            }

            Self::Random(error) => {
                write!(f, "Cycle Benchmarking random generation failed: {error}")
            }

            Self::Pauli(error) => {
                write!(f, "Cycle Benchmarking Pauli generation failed: {error}")
            }

            Self::InvalidObservation { reason } => {
                write!(f, "invalid Cycle Benchmarking execution observation: {reason}")
            }

            Self::InsufficientFitData { pauli, lengths } => {
                write!(
                    f,
                    "insufficient decay data for Pauli {pauli}: \
                     {lengths} distinct sequence lengths"
                )
            }

            Self::FitFailure { pauli, reason } => {
                write!(f, "Cycle Benchmarking fit failed for Pauli {pauli}: {reason}")
            }

            Self::InvalidFitParameter {
                pauli,
                parameter,
                value,
            } => {
                write!(
                    f,
                    "invalid Cycle Benchmarking fit parameter {parameter}={value} \
                     for Pauli {pauli}"
                )
            }

            Self::DimensionOverflow { qubits } => {
                write!(
                    f,
                    "Hilbert-space dimension cannot be represented safely for \
                     {qubits} qubits"
                )
            }

            Self::InvalidFidelity { value } => {
                write!(
                    f,
                    "Cycle Benchmarking produced an invalid fidelity value: {value}"
                )
            }

            Self::ProcessFidelityUnavailable { reason } => {
                write!(
                    f,
                    "Cycle Benchmarking process fidelity is unavailable: {reason}"
                )
            }

            Self::EmptyObservations => {
                write!(f, "Cycle Benchmarking received no execution observations")
            }

            Self::MissingObservation { instance_id } => {
                write!(
                    f,
                    "Cycle Benchmarking observation is missing for instance '{instance_id}'"
                )
            }

            Self::DuplicateObservation { instance_id } => {
                write!(
                    f,
                    "Cycle Benchmarking received duplicate observation for instance \
                     '{instance_id}'"
                )
            }
        }
    }
}

impl Error for CycleBenchmarkError {}

impl From<RandomError> for CycleBenchmarkError {
    fn from(error: RandomError) -> Self {
        Self::Random(error)
    }
}

impl From<PauliError> for CycleBenchmarkError {
    fn from(error: PauliError) -> Self {
        Self::Pauli(error)
    }
}

/// Result alias for this module.
pub type CycleBenchmarkResult<T> = Result<T, CycleBenchmarkError>;

// =============================================================================
// Pauli frame
// =============================================================================

/// An N-qubit tensor-product Pauli operator.
///
/// The ordering is logical-qubit order:
///
/// ```text
/// [q0, q1, q2, ...]
/// ```
///
/// The identity operator is represented by all `I` factors.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PauliFrame {
    factors: Vec<Pauli>,
}

impl PauliFrame {
    /// Constructs a Pauli frame.
    pub fn new(factors: Vec<Pauli>) -> CycleBenchmarkResult<Self> {
        if factors.is_empty() {
            return Err(CycleBenchmarkError::InvalidQubitCount { qubits: 0 });
        }

        Ok(Self { factors })
    }

    /// Constructs the all-identity frame.
    pub fn identity(qubits: usize) -> CycleBenchmarkResult<Self> {
        if qubits == 0 {
            return Err(CycleBenchmarkError::InvalidQubitCount { qubits: 0 });
        }

        Ok(Self {
            factors: vec![Pauli::I; qubits],
        })
    }

    /// Constructs a uniformly random Pauli frame using Zamani's canonical
    /// random-source contract.
    pub fn random<R: RandomSource + ?Sized>(
        qubits: usize,
        rng: &mut R,
    ) -> CycleBenchmarkResult<Self> {
        if qubits == 0 {
            return Err(CycleBenchmarkError::InvalidQubitCount { qubits: 0 });
        }

        let mut factors = Vec::with_capacity(qubits);

        for _ in 0..qubits {
            factors.push(Pauli::random(rng)?);
        }

        Ok(Self { factors })
    }

    /// Constructs a uniformly random non-identity Pauli frame.
    ///
    /// This is useful for sampling preparation/measurement bases because the
    /// identity basis does not provide a non-trivial Pauli decay.
    pub fn random_non_identity<R: RandomSource + ?Sized>(
        qubits: usize,
        rng: &mut R,
    ) -> CycleBenchmarkResult<Self> {
        if qubits == 0 {
            return Err(CycleBenchmarkError::InvalidQubitCount { qubits: 0 });
        }

        let mut frame = Self::random(qubits, rng)?;

        if frame.is_identity() {
            let index = rng.range_usize(0, qubits)?;

            let replacement = Pauli::random_non_identity(rng)?;

            frame.factors[index] = replacement;
        }

        Ok(frame)
    }

    /// Returns the number of qubits represented by this frame.
    #[inline]
    pub fn qubits(&self) -> usize {
        self.factors.len()
    }

    /// Returns the individual factors.
    #[inline]
    pub fn factors(&self) -> &[Pauli] {
        &self.factors
    }

    /// Returns whether this is the identity Pauli.
    pub fn is_identity(&self) -> bool {
        self.factors.iter().all(|p| p.is_identity())
    }

    /// Returns the number of non-identity factors.
    pub fn weight(&self) -> usize {
        self.factors.iter().map(|p| p.weight()).sum()
    }

    /// Returns a stable textual label such as `IXYZ`.
    pub fn label(&self) -> String {
        self.factors.iter().map(|p| p.symbol()).collect()
    }
}

impl fmt::Display for PauliFrame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for pauli in &self.factors {
            write!(f, "{}", pauli.symbol())?;
        }

        Ok(())
    }
}

// =============================================================================
// Cycle definition
// =============================================================================

/// Backend-independent description of the cycle being benchmarked.
///
/// This does not contain Quantum IR. The executor adapter maps this stable
/// description to the canonical IR.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CycleDefinition {
    /// Stable cycle identifier.
    pub id: String,

    /// Number of logical qubits involved in the cycle.
    pub qubits: usize,

    /// Number of operations in the cycle, if known.
    pub operation_count: Option<usize>,

    /// Number of two-qubit operations, if known.
    pub two_qubit_operation_count: Option<usize>,

    /// Number of parallel layers represented by the cycle.
    ///
    /// A normal CB clock cycle is normally one scheduling layer, but this
    /// field permits a higher-level adapter to preserve the actual cycle
    /// semantics.
    pub depth: usize,

    /// Optional finite identity period of the ideal cycle.
    ///
    /// When present, the protocol can require sequence lengths to be
    /// multiples of this value.
    pub identity_period: Option<usize>,
}

impl CycleDefinition {
    /// Creates a validated cycle definition.
    pub fn new(
        id: impl Into<String>,
        qubits: usize,
    ) -> CycleBenchmarkResult<Self> {
        let id = id.into();

        if id.trim().is_empty() {
            return Err(CycleBenchmarkError::InvalidConfiguration {
                field: "cycle.id",
                reason: "cycle identifier must not be empty".to_owned(),
            });
        }

        if qubits == 0 {
            return Err(CycleBenchmarkError::InvalidQubitCount { qubits });
        }

        Ok(Self {
            id,
            qubits,
            operation_count: None,
            two_qubit_operation_count: None,
            depth: 1,
            identity_period: None,
        })
    }

    /// Sets operation count metadata.
    #[must_use]
    pub fn with_operation_count(mut self, count: usize) -> Self {
        self.operation_count = Some(count);
        self
    }

    /// Sets two-qubit operation count metadata.
    #[must_use]
    pub fn with_two_qubit_operation_count(mut self, count: usize) -> Self {
        self.two_qubit_operation_count = Some(count);
        self
    }

    /// Sets the number of logical layers represented by the cycle.
    pub fn with_depth(
        mut self,
        depth: usize,
    ) -> CycleBenchmarkResult<Self> {
        if depth == 0 {
            return Err(CycleBenchmarkError::InvalidConfiguration {
                field: "cycle.depth",
                reason: "cycle depth must be greater than zero".to_owned(),
            });
        }

        self.depth = depth;
        Ok(self)
    }

    /// Sets the ideal cycle's finite identity period.
    pub fn with_identity_period(
        mut self,
        period: usize,
    ) -> CycleBenchmarkResult<Self> {
        if period == 0 {
            return Err(CycleBenchmarkError::InvalidConfiguration {
                field: "cycle.identity_period",
                reason: "identity period must be greater than zero".to_owned(),
            });
        }

        self.identity_period = Some(period);
        Ok(self)
    }

    /// Validates compatibility with the supplied protocol configuration.
    pub fn validate(
        &self,
        config: &CycleBenchmarkConfig,
    ) -> CycleBenchmarkResult<()> {
        if self.qubits != config.qubits {
            return Err(CycleBenchmarkError::InvalidConfiguration {
                field: "cycle.qubits",
                reason: format!(
                    "cycle has {} qubits but protocol is configured for {}",
                    self.qubits, config.qubits
                ),
            });
        }

        if let Some(period) = self.identity_period {
            if config.require_periodic_lengths {
                for &length in &config.sequence_lengths {
                    if length % period != 0 {
                        return Err(
                            CycleBenchmarkError::SequenceLengthNotMultipleOfPeriod {
                                length,
                                period,
                            },
                        );
                    }
                }
            }
        } else if config.require_periodic_lengths {
            return Err(CycleBenchmarkError::InvalidConfiguration {
                field: "cycle.identity_period",
                reason: "periodic sequence lengths were requested but the cycle \
                          definition has no verified identity period"
                    .to_owned(),
            });
        }

        Ok(())
    }
}

// =============================================================================
// Cycle benchmark configuration
// =============================================================================

/// Complete configuration for one Cycle Benchmarking experiment.
#[derive(Debug, Clone, PartialEq)]
pub struct CycleBenchmarkConfig {
    /// Number of qubits in the benchmark register.
    pub qubits: usize,

    /// Sequence lengths, expressed as number of target-cycle applications.
    pub sequence_lengths: Vec<usize>,

    /// Number of distinct Pauli preparation/measurement bases.
    pub pauli_count: usize,

    /// Number of independent randomized circuits per
    /// `(Pauli, sequence_length)` point.
    pub randomizations_per_length: usize,

    /// Shots per randomized circuit.
    pub shots: usize,

    /// Confidence level associated with reported uncertainty.
    pub confidence_level: f64,

    /// Statistical fit model.
    pub fit_model: CycleFitModel,

    /// Whether the Pauli identity is eligible for selection.
    ///
    /// Standard production CB normally excludes it because the identity
    /// channel does not provide a useful traceless Pauli decay.
    pub include_identity_pauli: bool,

    /// Whether sequence lengths must be multiples of the cycle's verified
    /// identity period.
    pub require_periodic_lengths: bool,

    /// Maximum qubit count.
    pub max_qubits: usize,

    /// Maximum number of sequence lengths.
    pub max_sequence_lengths: usize,

    /// Maximum number of Pauli bases.
    pub max_paulis: usize,

    /// Maximum randomizations per length.
    pub max_randomizations: usize,

    /// Maximum shots per circuit.
    pub max_shots: usize,

    /// Maximum total generated circuit instances.
    pub max_instances: usize,
}

impl Default for CycleBenchmarkConfig {
    fn default() -> Self {
        Self {
            qubits: 1,
            sequence_lengths: vec![2, 8, 16],
            pauli_count: DEFAULT_PAULI_COUNT,
            randomizations_per_length: DEFAULT_RANDOMIZATIONS_PER_LENGTH,
            shots: DEFAULT_SHOTS,
            confidence_level: DEFAULT_CONFIDENCE_LEVEL,
            fit_model: CycleFitModel::NoOffset,
            include_identity_pauli: false,
            require_periodic_lengths: true,
            max_qubits: DEFAULT_MAX_QUBITS,
            max_sequence_lengths: DEFAULT_MAX_SEQUENCE_LENGTHS,
            max_paulis: DEFAULT_MAX_PAULIS,
            max_randomizations: DEFAULT_MAX_RANDOMIZATIONS,
            max_shots: DEFAULT_MAX_SHOTS,
            max_instances: DEFAULT_MAX_INSTANCES,
        }
    }
}

impl CycleBenchmarkConfig {
    /// Validates the complete configuration.
    pub fn validate(&self) -> CycleBenchmarkResult<()> {
        if self.qubits == 0 {
            return Err(CycleBenchmarkError::InvalidQubitCount {
                qubits: self.qubits,
            });
        }

        if self.qubits > self.max_qubits {
            return Err(CycleBenchmarkError::ResourceLimitExceeded {
                resource: "qubits",
                requested: self.qubits,
                maximum: self.max_qubits,
            });
        }

        if self.sequence_lengths.is_empty() {
            return Err(CycleBenchmarkError::InvalidConfiguration {
                field: "sequence_lengths",
                reason: "at least one sequence length is required".to_owned(),
            });
        }

        if self.sequence_lengths.len() > self.max_sequence_lengths {
            return Err(CycleBenchmarkError::ResourceLimitExceeded {
                resource: "sequence_lengths",
                requested: self.sequence_lengths.len(),
                maximum: self.max_sequence_lengths,
            });
        }

        let mut previous = 0usize;

        for &length in &self.sequence_lengths {
            if length == 0 {
                return Err(CycleBenchmarkError::InvalidSequenceLength { length });
            }

            if length <= previous {
                return Err(CycleBenchmarkError::NonIncreasingSequenceLengths);
            }

            previous = length;
        }

        if self.pauli_count == 0 {
            return Err(CycleBenchmarkError::InvalidConfiguration {
                field: "pauli_count",
                reason: "at least one Pauli basis is required".to_owned(),
            });
        }

        if self.pauli_count > self.max_paulis {
            return Err(CycleBenchmarkError::ResourceLimitExceeded {
                resource: "pauli_count",
                requested: self.pauli_count,
                maximum: self.max_paulis,
            });
        }

        if self.randomizations_per_length == 0 {
            return Err(CycleBenchmarkError::InvalidConfiguration {
                field: "randomizations_per_length",
                reason: "at least one randomization is required".to_owned(),
            });
        }

        if self.randomizations_per_length > self.max_randomizations {
            return Err(CycleBenchmarkError::ResourceLimitExceeded {
                resource: "randomizations_per_length",
                requested: self.randomizations_per_length,
                maximum: self.max_randomizations,
            });
        }

        if self.shots == 0 {
            return Err(CycleBenchmarkError::InvalidConfiguration {
                field: "shots",
                reason: "shots must be greater than zero".to_owned(),
            });
        }

        if self.shots > self.max_shots {
            return Err(CycleBenchmarkError::ResourceLimitExceeded {
                resource: "shots",
                requested: self.shots,
                maximum: self.max_shots,
            });
        }

        if !self.confidence_level.is_finite()
            || self.confidence_level <= 0.0
            || self.confidence_level >= 1.0
        {
            return Err(CycleBenchmarkError::InvalidConfiguration {
                field: "confidence_level",
                reason: "confidence level must be finite and strictly between \
                          zero and one"
                    .to_owned(),
            });
        }

        let requested_instances = self
            .pauli_count
            .checked_mul(self.sequence_lengths.len())
            .and_then(|value| value.checked_mul(self.randomizations_per_length))
            .ok_or(CycleBenchmarkError::ResourceLimitExceeded {
                resource: "instances",
                requested: usize::MAX,
                maximum: self.max_instances,
            })?;

        if requested_instances > self.max_instances {
            return Err(CycleBenchmarkError::ResourceLimitExceeded {
                resource: "instances",
                requested: requested_instances,
                maximum: self.max_instances,
            });
        }

        Ok(())
    }

    /// Returns the expected number of generated circuit instances.
    pub fn instance_count(&self) -> CycleBenchmarkResult<usize> {
        self.validate()?;

        self.pauli_count
            .checked_mul(self.sequence_lengths.len())
            .and_then(|value| value.checked_mul(self.randomizations_per_length))
            .ok_or(CycleBenchmarkError::ResourceLimitExceeded {
                resource: "instances",
                requested: usize::MAX,
                maximum: self.max_instances,
            })
    }

    /// Returns the total requested shots.
    pub fn total_shots(&self) -> CycleBenchmarkResult<usize> {
        let instances = self.instance_count()?;

        instances.checked_mul(self.shots).ok_or(
            CycleBenchmarkError::ResourceLimitExceeded {
                resource: "total_shots",
                requested: usize::MAX,
                maximum: self.max_instances.saturating_mul(self.max_shots),
            },
        )
    }
}

// =============================================================================
// Cycle instance
// =============================================================================

/// One concrete CB randomized circuit description.
///
/// This is intentionally not a Quantum IR circuit. The execution adapter is
/// responsible for compiling this abstract description into IR.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CycleBenchmarkInstance {
    /// Stable instance identifier.
    pub id: String,

    /// Cycle identifier.
    pub cycle_id: String,

    /// Preparation/measurement Pauli.
    pub measured_pauli: PauliFrame,

    /// Number of target-cycle applications.
    pub sequence_length: usize,

    /// Random Pauli cycles.
///
/// There must be `sequence_length + 1` entries.
    pub random_pauli_cycles: Vec<PauliFrame>,

    /// Trial index for this Pauli/length pair.
    pub trial_index: usize,
}

impl CycleBenchmarkInstance {
    /// Creates a validated CB instance.
    pub fn new(
        cycle: &CycleDefinition,
        measured_pauli: PauliFrame,
        sequence_length: usize,
        random_pauli_cycles: Vec<PauliFrame>,
        trial_index: usize,
    ) -> CycleBenchmarkResult<Self> {
        if sequence_length == 0 {
            return Err(CycleBenchmarkError::InvalidSequenceLength {
                length: sequence_length,
            });
        }

        if measured_pauli.qubits() != cycle.qubits {
            return Err(CycleBenchmarkError::PauliWidthMismatch {
                expected: cycle.qubits,
                actual: measured_pauli.qubits(),
            });
        }

        let expected_randomizations =
            sequence_length.checked_add(1).ok_or(
                CycleBenchmarkError::ResourceLimitExceeded {
                    resource: "random_pauli_cycles",
                    requested: usize::MAX,
                    maximum: usize::MAX,
                },
            )?;

        if random_pauli_cycles.len() != expected_randomizations {
            return Err(CycleBenchmarkError::InvalidConfiguration {
                field: "random_pauli_cycles",
                reason: format!(
                    "expected {} random Pauli cycles for sequence length {}, got {}",
                    expected_randomizations,
                    sequence_length,
                    random_pauli_cycles.len()
                ),
            });
        }

        for frame in &random_pauli_cycles {
            if frame.qubits() != cycle.qubits {
                return Err(CycleBenchmarkError::PauliWidthMismatch {
                    expected: cycle.qubits,
                    actual: frame.qubits(),
                });
            }
        }

        let id = format!(
            "{}::{}::m{}::trial{}",
            cycle.id,
            measured_pauli.label(),
            sequence_length,
            trial_index
        );

        Ok(Self {
            id,
            cycle_id: cycle.id.clone(),
            measured_pauli,
            sequence_length,
            random_pauli_cycles,
            trial_index,
        })
    }
}

// =============================================================================
// Execution contract
// =============================================================================

/// Backend-independent execution request.
///
/// The executor adapter converts `CycleBenchmarkInstance` into the canonical
/// Quantum IR and executes it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CycleExecutionRequest {
    /// Concrete CB circuit description.
    pub instance: CycleBenchmarkInstance,

    /// Requested shots.
    pub shots: usize,
}

impl CycleExecutionRequest {
    /// Constructs a validated execution request.
    pub fn new(
        instance: CycleBenchmarkInstance,
        shots: usize,
    ) -> CycleBenchmarkResult<Self> {
        if shots == 0 {
            return Err(CycleBenchmarkError::InvalidConfiguration {
                field: "shots",
                reason: "shots must be greater than zero".to_owned(),
            });
        }

        Ok(Self { instance, shots })
    }
}

/// Raw execution observation for one CB circuit.
///
/// `matching_outcomes` means outcomes consistent with the expected ideal
/// Pauli eigenvalue after the executor has accounted for the known ideal
/// Pauli propagation/sign.
///
/// This is deliberately more general than assuming a literal computational
/// basis `0`/`1` measurement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CycleExecutionObservation {
    /// Concrete CB instance identifier.
    pub instance_id: String,

    /// Number of successful/matching observations.
    pub matching_outcomes: usize,

    /// Total shots actually executed.
    pub shots: usize,

    /// Optional execution duration in nanoseconds.
    pub execution_time_ns: Option<u64>,
}

impl CycleExecutionObservation {
    /// Creates a validated execution observation.
    pub fn new(
        instance_id: impl Into<String>,
        matching_outcomes: usize,
        shots: usize,
    ) -> CycleBenchmarkResult<Self> {
        let instance_id = instance_id.into();

        if instance_id.trim().is_empty() {
            return Err(CycleBenchmarkError::InvalidObservation {
                reason: "instance_id must not be empty".to_owned(),
            });
        }

        if shots == 0 {
            return Err(CycleBenchmarkError::InvalidObservation {
                reason: "observation shots must be greater than zero".to_owned(),
            });
        }

        if matching_outcomes > shots {
            return Err(CycleBenchmarkError::InvalidObservation {
                reason: format!(
                    "matching outcomes {} exceed shots {}",
                    matching_outcomes, shots
                ),
            });
        }

        Ok(Self {
            instance_id,
            matching_outcomes,
            shots,
            execution_time_ns: None,
        })
    }

    /// Adds execution timing metadata.
    #[must_use]
    pub fn with_execution_time_ns(mut self, value: u64) -> Self {
        self.execution_time_ns = Some(value);
        self
    }

    /// Returns the measured matching probability.
    pub fn matching_probability(&self) -> f64 {
        self.matching_outcomes as f64 / self.shots as f64
    }

    /// Returns the measured Pauli expectation value.
    ///
    /// If matching outcomes correspond to the +1 eigenvalue and non-matching
    /// outcomes correspond to -1, then:
    ///
    /// `E = 2 p - 1`.
    pub fn expectation(&self) -> f64 {
        2.0 * self.matching_probability() - 1.0
    }
}

/// Stable executor contract for CB.
///
/// An implementation may internally use:
///
/// ```text
/// CycleBenchmarkInstance
///       ↓
/// Quantum IR adapter
///       ↓
/// compiler
///       ↓
/// routing
///       ↓
/// scheduling
///       ↓
/// simulator / hardware
/// ```
///
/// but none of those concerns are exposed by this protocol.
pub trait CycleBenchmarkExecutor {
    /// Executor-specific error.
    type Error: Error + Send + Sync + 'static;

    /// Executes one CB circuit.
    fn execute(
        &mut self,
        request: &CycleExecutionRequest,
    ) -> Result<CycleExecutionObservation, Self::Error>;
}

// =============================================================================
// Generated experiment
// =============================================================================

/// Fully generated Cycle Benchmarking experiment.
///
/// It is immutable after construction and can therefore be serialized,
/// hashed, scheduled, or passed to an execution service.
#[derive(Debug, Clone)]
pub struct CycleBenchmarkExperiment {
    /// Protocol configuration.
    pub config: CycleBenchmarkConfig,

    /// Target cycle.
    pub cycle: CycleDefinition,

    /// Selected preparation/measurement Paulis.
    pub paulis: Vec<PauliFrame>,

    /// Concrete randomized circuits.
    pub instances: Vec<CycleBenchmarkInstance>,
}

impl CycleBenchmarkExperiment {
    /// Validates the complete experiment.
    pub fn validate(&self) -> CycleBenchmarkResult<()> {
        self.config.validate()?;
        self.cycle.validate(&self.config)?;

        if self.paulis.len() != self.config.pauli_count {
            return Err(CycleBenchmarkError::InvalidConfiguration {
                field: "paulis",
                reason: format!(
                    "expected {} Pauli bases, got {}",
                    self.config.pauli_count,
                    self.paulis.len()
                ),
            });
        }

        let mut labels = BTreeMap::<String, ()>::new();

        for pauli in &self.paulis {
            if pauli.qubits() != self.config.qubits {
                return Err(CycleBenchmarkError::PauliWidthMismatch {
                    expected: self.config.qubits,
                    actual: pauli.qubits(),
                });
            }

            if !self.config.include_identity_pauli && pauli.is_identity() {
                return Err(CycleBenchmarkError::InvalidConfiguration {
                    field: "paulis",
                    reason: "identity Pauli is disabled by configuration".to_owned(),
                });
            }

            let label = pauli.label();

            if labels.insert(label.clone(), ()).is_some() {
                return Err(CycleBenchmarkError::DuplicatePauli { pauli: label });
            }
        }

        if self.instances.len() > self.config.max_instances {
            return Err(CycleBenchmarkError::ResourceLimitExceeded {
                resource: "instances",
                requested: self.instances.len(),
                maximum: self.config.max_instances,
            });
        }

        let expected_count = self.config.instance_count()?;

        if self.instances.len() != expected_count {
            return Err(CycleBenchmarkError::InvalidConfiguration {
                field: "instances",
                reason: format!(
                    "expected {} instances, got {}",
                    expected_count,
                    self.instances.len()
                ),
            });
        }

        let mut instance_ids = BTreeMap::<String, ()>::new();

        for instance in &self.instances {
            if instance_ids
                .insert(instance.id.clone(), ())
                .is_some()
            {
                return Err(CycleBenchmarkError::DuplicateObservation {
                    instance_id: instance.id.clone(),
                });
            }

            if instance.measured_pauli.qubits() != self.config.qubits {
                return Err(CycleBenchmarkError::PauliWidthMismatch {
                    expected: self.config.qubits,
                    actual: instance.measured_pauli.qubits(),
                });
            }

            if !self
                .config
                .sequence_lengths
                .contains(&instance.sequence_length)
            {
                return Err(CycleBenchmarkError::InvalidConfiguration {
                    field: "instance.sequence_length",
                    reason: format!(
                        "sequence length {} was not requested",
                        instance.sequence_length
                    ),
                });
            }

            if instance.random_pauli_cycles.len()
                != instance.sequence_length + 1
            {
                return Err(CycleBenchmarkError::InvalidConfiguration {
                    field: "instance.random_pauli_cycles",
                    reason: "random Pauli cycle count must equal m + 1".to_owned(),
                });
            }
        }

        Ok(())
    }
}

// =============================================================================
// Pauli selection
// =============================================================================

/// Selects a unique random set of Pauli preparation/measurement bases.
pub fn select_random_paulis<R: RandomSource + ?Sized>(
    qubits: usize,
    count: usize,
    include_identity: bool,
    rng: &mut R,
) -> CycleBenchmarkResult<Vec<PauliFrame>> {
    if qubits == 0 {
        return Err(CycleBenchmarkError::InvalidQubitCount { qubits });
    }

    if count == 0 {
        return Err(CycleBenchmarkError::InvalidConfiguration {
            field: "pauli_count",
            reason: "at least one Pauli is required".to_owned(),
        });
    }

    let available = checked_non_identity_pauli_count(qubits)?
        .checked_add(if include_identity { 1 } else { 0 })
        .ok_or(CycleBenchmarkError::PauliSetExhausted {
            requested: count,
            available: usize::MAX,
        })?;

    if count > available {
        return Err(CycleBenchmarkError::PauliSetExhausted {
            requested: count,
            available,
        });
    }

    let mut selected = BTreeMap::<String, PauliFrame>::new();
    let mut attempts = 0usize;

    while selected.len() < count {
        attempts = attempts.saturating_add(1);

        if attempts > DEFAULT_MAX_PAULI_SELECTION_ATTEMPTS {
            return Err(CycleBenchmarkError::InvalidConfiguration {
                field: "pauli_selection",
                reason: "unable to construct the requested unique Pauli set \
                          within the bounded selection-attempt limit"
                    .to_owned(),
            });
        }

        let candidate = if include_identity {
            PauliFrame::random(qubits, rng)?
        } else {
            PauliFrame::random_non_identity(qubits, rng)?
        };

        selected.entry(candidate.label()).or_insert(candidate);
    }

    Ok(selected.into_values().collect())
}

// =============================================================================
// Experiment generation
// =============================================================================

/// Generates a complete randomized CB experiment.
///
/// Randomness MUST be supplied explicitly through Zamani's benchmark
/// `RandomSource`. This guarantees that the caller controls the root seed and
/// can record it in benchmark provenance.
pub fn generate_experiment<R: RandomSource + ?Sized>(
    cycle: CycleDefinition,
    config: CycleBenchmarkConfig,
    rng: &mut R,
) -> CycleBenchmarkResult<CycleBenchmarkExperiment> {
    config.validate()?;
    cycle.validate(&config)?;

    let paulis = select_random_paulis(
        config.qubits,
        config.pauli_count,
        config.include_identity_pauli,
        rng,
    )?;

    let expected_instances = config.instance_count()?;
    let mut instances = Vec::with_capacity(expected_instances);

    for pauli in &paulis {
        for &sequence_length in &config.sequence_lengths {
            for trial_index in 0..config.randomizations_per_length {
                let mut random_pauli_cycles =
                    Vec::with_capacity(sequence_length + 1);

                for _ in 0..=sequence_length {
                    random_pauli_cycles.push(
                        PauliFrame::random(config.qubits, rng)?,
                    );
                }

                instances.push(CycleBenchmarkInstance::new(
                    &cycle,
                    pauli.clone(),
                    sequence_length,
                    random_pauli_cycles,
                    trial_index,
                )?);
            }
        }
    }

    let experiment = CycleBenchmarkExperiment {
        config,
        cycle,
        paulis,
        instances,
    };

    experiment.validate()?;

    Ok(experiment)
}

// =============================================================================
// Fit data
// =============================================================================

/// One aggregated decay point.
///
/// Individual randomized circuits remain available through the raw
/// observations; this structure contains their statistically aggregated
/// expectation value.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CycleDecayPoint {
    /// Sequence length m.
    pub sequence_length: usize,

    /// Mean Pauli expectation value.
    pub mean_expectation: f64,

    /// Standard error of the mean.
    pub standard_error: f64,

    /// Number of randomized circuits contributing to this point.
    pub randomizations: usize,

    /// Total shots contributing to this point.
    pub total_shots: usize,
}

impl CycleDecayPoint {
    /// Creates a decay point from raw observations.
    pub fn from_observations(
        sequence_length: usize,
        observations: &[CycleExecutionObservation],
    ) -> CycleBenchmarkResult<Self> {
        if observations.is_empty() {
            return Err(CycleBenchmarkError::InvalidObservation {
                reason: "cannot construct a decay point from zero observations"
                    .to_owned(),
            });
        }

        let mut total_shots = 0usize;
        let mut values = Vec::with_capacity(observations.len());

        for observation in observations {
            total_shots = total_shots.checked_add(observation.shots).ok_or(
                CycleBenchmarkError::InvalidObservation {
                    reason: "total shot count overflowed".to_owned(),
                },
            )?;

            values.push(observation.expectation());
        }

        let mean = values.iter().sum::<f64>() / values.len() as f64;

        if !mean.is_finite() {
            return Err(CycleBenchmarkError::InvalidObservation {
                reason: "mean expectation is not finite".to_owned(),
            });
        }

        let variance = if values.len() > 1 {
            let sum_squared = values
                .iter()
                .map(|value| {
                    let delta = *value - mean;
                    delta * delta
                })
                .sum::<f64>();

            sum_squared / (values.len() - 1) as f64
        } else {
            0.0
        };

        let standard_error = if values.len() > 1 {
            (variance / values.len() as f64).sqrt()
        } else {
            0.0
        };

        if !standard_error.is_finite() {
            return Err(CycleBenchmarkError::InvalidObservation {
                reason: "standard error is not finite".to_owned(),
            });
        }

        Ok(Self {
            sequence_length,
            mean_expectation: mean,
            standard_error,
            randomizations: values.len(),
            total_shots,
        })
    }
}

// =============================================================================
// Fit result
// =============================================================================

/// Diagnostics produced by fitting one Pauli decay.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CycleDecayFit {
    /// Fitted SPAM amplitude A.
    pub amplitude: f64,

    /// Fitted decay parameter p.
    pub decay_parameter: f64,

    /// Fitted offset B.
///
/// For `NoOffset`, this is exactly zero.
    pub offset: f64,

    /// Sum of squared residuals.
    pub sum_squared_error: f64,

    /// Root mean squared error.
    pub root_mean_squared_error: f64,

    /// Coefficient of determination.
    ///
    /// This is diagnostic only and must not be used as the sole criterion for
    /// accepting a physical benchmark result.
    pub r_squared: f64,

    /// Number of distinct sequence lengths.
    pub sequence_points: usize,

    /// Number of fitted parameters.
    pub fitted_parameters: usize,

    /// Approximate standard error of the fitted decay parameter.
    ///
    /// This is a local curvature estimate, not a guaranteed coverage interval.
    pub decay_standard_error: Option<f64>,

    /// Fit model.
    pub model: CycleFitModel,
}

impl CycleDecayFit {
    /// Returns the Pauli infidelity `1 - p`.
    pub fn pauli_infidelity(&self) -> f64 {
        1.0 - self.decay_parameter
    }

    /// Predicts the expectation value at sequence length m.
    pub fn predict(&self, sequence_length: usize) -> f64 {
        let power = safe_power(self.decay_parameter, sequence_length);

        self.amplitude * power + self.offset
    }

    /// Returns whether the fitted parameter is inside the mathematically
    /// possible Pauli-transfer interval.
    pub fn decay_parameter_is_physical(&self) -> bool {
        self.decay_parameter.is_finite()
            && self.decay_parameter >= -1.0 - FINITE_EPSILON
            && self.decay_parameter <= 1.0 + FINITE_EPSILON
    }
}

// =============================================================================
// Pauli benchmark result
// =============================================================================

/// Complete result for one Pauli decay.
#[derive(Debug, Clone)]
pub struct PauliCycleResult {
    /// Measured Pauli basis.
    pub pauli: PauliFrame,

    /// Aggregated decay points.
    pub decay_points: Vec<CycleDecayPoint>,

    /// Exponential fit.
    pub fit: CycleDecayFit,
}

impl PauliCycleResult {
    /// Returns the fitted Pauli fidelity.
    pub fn pauli_fidelity(&self) -> f64 {
        self.fit.decay_parameter
    }

    /// Returns the fitted Pauli infidelity.
    pub fn pauli_infidelity(&self) -> f64 {
        1.0 - self.fit.decay_parameter
    }
}

// =============================================================================
// Composite process result
// =============================================================================

/// Estimate of the process fidelity for the complete cycle.
///
/// The estimate explicitly records whether the complete Pauli basis was
/// measured or whether this is a sampled estimator.
#[derive(Debug, Clone)]
pub struct CompositeCycleFidelity {
    /// Estimated process fidelity.
    pub process_fidelity: f64,

    /// Estimated process infidelity.
    pub process_infidelity: f64,

    /// Estimated average gate fidelity.
    pub average_gate_fidelity: f64,

    /// Estimated average gate infidelity.
    pub average_gate_infidelity: f64,

    /// Number of non-identity Pauli terms used.
    pub pauli_terms: usize,

    /// Number of non-identity Pauli terms in the complete n-qubit basis.
    pub complete_non_identity_terms: usize,

    /// Whether every non-identity Pauli term was measured.
    pub exhaustive: bool,

    /// Mean sampled non-identity Pauli fidelity.
    pub mean_pauli_fidelity: f64,

    /// Standard deviation of the sampled Pauli fidelities.
    pub pauli_fidelity_standard_deviation: f64,
}

impl CompositeCycleFidelity {
    /// Computes a composite process-fidelity estimate from Pauli decay fits.
    pub fn from_pauli_results(
        qubits: usize,
        results: &[PauliCycleResult],
    ) -> CycleBenchmarkResult<Self> {
        if results.is_empty() {
            return Err(CycleBenchmarkError::ProcessFidelityUnavailable {
                reason: "no Pauli decay results are available".to_owned(),
            });
        }

        let complete_non_identity_terms =
            checked_non_identity_pauli_count(qubits)?;

        let mut values = Vec::with_capacity(results.len());

        for result in results {
            let fidelity = result.pauli_fidelity();

            if !fidelity.is_finite() {
                return Err(CycleBenchmarkError::InvalidFidelity {
                    value: fidelity,
                });
            }

            values.push(fidelity);
        }

        let mean = values.iter().sum::<f64>() / values.len() as f64;

        if !mean.is_finite() {
            return Err(CycleBenchmarkError::InvalidFidelity { value: mean });
        }

        let variance = if values.len() > 1 {
            values
                .iter()
                .map(|value| {
                    let delta = *value - mean;
                    delta * delta
                })
                .sum::<f64>()
                / (values.len() - 1) as f64
        } else {
            0.0
        };

        let standard_deviation = variance.sqrt();

        let d = hilbert_dimension(qubits)?;

        let d_squared = d
            .checked_mul(d)
            .ok_or(CycleBenchmarkError::DimensionOverflow { qubits })?;

        let process_fidelity = if results.len() == complete_non_identity_terms {
            // Exact complete-Pauli-basis estimator:
            //
            // F = [1 + sum(nonidentity f_P)] / d²
            let sum = values.iter().sum::<f64>();
            (1.0 + sum) / d_squared as f64
        } else {
            // Unbiased estimator when nonidentity Paulis are sampled uniformly:
            //
            // F = [1 + (d² - 1) * mean(f_P)] / d²
            (1.0
                + complete_non_identity_terms as f64 * mean)
                / d_squared as f64
        };

        validate_fidelity(process_fidelity)?;

        let process_infidelity = 1.0 - process_fidelity;

        let average_gate_fidelity =
            (d as f64 * process_fidelity + 1.0)
                / (d as f64 + 1.0);

        validate_fidelity(average_gate_fidelity)?;

        let average_gate_infidelity = 1.0 - average_gate_fidelity;

        Ok(Self {
            process_fidelity,
            process_infidelity,
            average_gate_fidelity,
            average_gate_infidelity,
            pauli_terms: results.len(),
            complete_non_identity_terms,
            exhaustive: results.len() == complete_non_identity_terms,
            mean_pauli_fidelity: mean,
            pauli_fidelity_standard_deviation: standard_deviation,
        })
    }
}

// =============================================================================
// Full benchmark result
// =============================================================================

/// Final protocol result.
///
/// This is intentionally protocol-specific. `core::BenchmarkResult` should
/// later wrap this structure rather than replace its semantics.
#[derive(Debug, Clone)]
pub struct CycleBenchmarkProtocolResult {
    /// Protocol identifier.
    pub benchmark_id: &'static str,

    /// Protocol version.
    pub protocol_version: &'static str,

    /// Cycle being benchmarked.
    pub cycle: CycleDefinition,

    /// Configuration used.
    pub config: CycleBenchmarkConfig,

    /// Pauli decay results.
    pub pauli_results: Vec<PauliCycleResult>,

    /// Composite process fidelity.
    pub composite_fidelity: CompositeCycleFidelity,

    /// Total number of executed circuits.
    pub executed_circuits: usize,

    /// Total number of executed shots.
    pub executed_shots: usize,

    /// Optional total execution time in nanoseconds.
    pub total_execution_time_ns: Option<u64>,

    /// Scientific assumptions attached to the result.
    pub assumptions: CycleBenchmarkAssumptions,
}

impl CycleBenchmarkProtocolResult {
    /// Returns the process fidelity.
    pub fn process_fidelity(&self) -> f64 {
        self.composite_fidelity.process_fidelity
    }

    /// Returns the process infidelity.
    pub fn process_infidelity(&self) -> f64 {
        self.composite_fidelity.process_infidelity
    }
}

// =============================================================================
// Scientific assumptions
// =============================================================================

/// Assumptions and interpretation flags attached to every CB result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CycleBenchmarkAssumptions {
    /// Whether the target cycle is asserted to have the required finite
    /// identity period.
    pub periodic_cycle_verified: bool,

    /// Whether the protocol assumes Markovian/stationary noise.
    pub markovian_noise_assumed: bool,

    /// Whether the Pauli twirling implementation is assumed sufficiently
    /// characterized.
    pub pauli_twirling_implementation_assumed_characterized: bool,

    /// Whether preparation/measurement basis operations are assumed to be
    /// consistent across sequence lengths.
    pub consistent_spam_basis_assumed: bool,

    /// Whether the reported fidelity is explicitly the dressed/twirled cycle.
    pub reports_dressed_cycle: bool,

    /// Whether the Pauli sample is exhaustive.
    pub exhaustive_pauli_basis: bool,
}

impl Default for CycleBenchmarkAssumptions {
    fn default() -> Self {
        Self {
            periodic_cycle_verified: false,
            markovian_noise_assumed: true,
            pauli_twirling_implementation_assumed_characterized: true,
            consistent_spam_basis_assumed: true,
            reports_dressed_cycle: true,
            exhaustive_pauli_basis: false,
        }
    }
}

// =============================================================================
// Protocol object
// =============================================================================

/// Production Cycle Benchmarking protocol object.
#[derive(Debug, Clone)]
pub struct CycleBenchmarkProtocol {
    /// Protocol configuration.
    pub config: CycleBenchmarkConfig,
}

impl CycleBenchmarkProtocol {
    /// Creates a validated protocol.
    pub fn new(
        config: CycleBenchmarkConfig,
    ) -> CycleBenchmarkResult<Self> {
        config.validate()?;

        Ok(Self { config })
    }

    /// Generates a reproducible CB experiment using the supplied RNG.
    pub fn generate<R: RandomSource + ?Sized>(
        &self,
        cycle: CycleDefinition,
        rng: &mut R,
    ) -> CycleBenchmarkResult<CycleBenchmarkExperiment> {
        generate_experiment(cycle, self.config.clone(), rng)
    }

    /// Executes all generated instances using the supplied executor.
    ///
    /// Execution remains backend-independent.
    pub fn execute<E: CycleBenchmarkExecutor>(
        &self,
        experiment: &CycleBenchmarkExperiment,
        executor: &mut E,
    ) -> CycleBenchmarkResult<Vec<CycleExecutionObservation>> {
        experiment.validate()?;

        let mut observations =
            Vec::with_capacity(experiment.instances.len());

        for instance in &experiment.instances {
            let request =
                CycleExecutionRequest::new(instance.clone(), self.config.shots)?;

            let observation = executor
                .execute(&request)
                .map_err(|error| CycleBenchmarkError::InvalidObservation {
                    reason: format!(
                        "executor failed for '{}': {}",
                        instance.id, error
                    ),
                })?;

            if observation.instance_id != instance.id {
                return Err(CycleBenchmarkError::InvalidObservation {
                    reason: format!(
                        "executor returned observation ID '{}' for requested \
                         instance '{}'",
                        observation.instance_id, instance.id
                    ),
                });
            }

            if observation.shots != self.config.shots {
                return Err(CycleBenchmarkError::InvalidObservation {
                    reason: format!(
                        "executor returned {} shots for '{}', expected {}",
                        observation.shots,
                        instance.id,
                        self.config.shots
                    ),
                });
            }

            observations.push(observation);
        }

        Ok(observations)
    }

    /// Analyzes previously captured observations.
    ///
    /// This is intentionally independent of execution and therefore allows:
    ///
    /// - offline analysis;
    /// - re-analysis with newer reporting;
    /// - regression tests;
    /// - archived hardware-result analysis;
    /// - simulator-vs-hardware comparison.
    pub fn analyze(
        &self,
        experiment: &CycleBenchmarkExperiment,
        observations: &[CycleExecutionObservation],
    ) -> CycleBenchmarkResult<CycleBenchmarkProtocolResult> {
        experiment.validate()?;

        if observations.is_empty() {
            return Err(CycleBenchmarkError::EmptyObservations);
        }

        let observation_map = index_observations(observations)?;

        let mut pauli_results = Vec::with_capacity(experiment.paulis.len());

        let mut executed_shots = 0usize;
        let mut executed_circuits = 0usize;
        let mut total_execution_time_ns = 0u64;
        let mut has_timing = false;

        for pauli in &experiment.paulis {
            let mut decay_points = Vec::with_capacity(
                experiment.config.sequence_lengths.len(),
            );

            for &sequence_length in
                &experiment.config.sequence_lengths
            {
                let mut point_observations = Vec::with_capacity(
                    experiment.config.randomizations_per_length,
                );

                for trial_index in
                    0..experiment.config.randomizations_per_length
                {
                    let instance_id = format!(
                        "{}::{}::m{}::trial{}",
                        experiment.cycle.id,
                        pauli.label(),
                        sequence_length,
                        trial_index
                    );

                    let observation = observation_map
                        .get(&instance_id)
                        .ok_or_else(|| {
                            CycleBenchmarkError::MissingObservation {
                                instance_id: instance_id.clone(),
                            }
                        })?;

                    executed_circuits =
                        executed_circuits.saturating_add(1);

                    executed_shots = executed_shots
                        .checked_add(observation.shots)
                        .ok_or(CycleBenchmarkError::InvalidObservation {
                            reason: "executed shot count overflowed".to_owned(),
                        })?;

                    if let Some(duration) =
                        observation.execution_time_ns
                    {
                        has_timing = true;
                        total_execution_time_ns =
                            total_execution_time_ns
                                .checked_add(duration)
                                .ok_or(
                                    CycleBenchmarkError::InvalidObservation {
                                        reason: "execution time overflowed"
                                            .to_owned(),
                                    },
                                )?;
                    }

                    point_observations.push(observation.clone());
                }

                decay_points.push(CycleDecayPoint::from_observations(
                    sequence_length,
                    &point_observations,
                )?);
            }

            let fit = fit_decay(
                pauli,
                &decay_points,
                self.config.fit_model,
            )?;

            pauli_results.push(PauliCycleResult {
                pauli: pauli.clone(),
                decay_points,
                fit,
            });
        }

        let composite_fidelity =
            CompositeCycleFidelity::from_pauli_results(
                experiment.config.qubits,
                &pauli_results,
            )?;

        let mut assumptions = CycleBenchmarkAssumptions::default();

        assumptions.periodic_cycle_verified =
            experiment.cycle.identity_period.is_some()
                && experiment.config.require_periodic_lengths;

        assumptions.exhaustive_pauli_basis =
            composite_fidelity.exhaustive;

        Ok(CycleBenchmarkProtocolResult {
            benchmark_id: CYCLE_BENCHMARK_ID,
            protocol_version: CYCLE_BENCHMARK_PROTOCOL_VERSION,
            cycle: experiment.cycle.clone(),
            config: experiment.config.clone(),
            pauli_results,
            composite_fidelity,
            executed_circuits,
            executed_shots,
            total_execution_time_ns: if has_timing {
                Some(total_execution_time_ns)
            } else {
                None
            },
            assumptions,
        })
    }

    /// Convenience method that generates and executes a complete experiment.
    pub fn run<E, R>(
        &self,
        cycle: CycleDefinition,
        rng: &mut R,
        executor: &mut E,
    ) -> CycleBenchmarkResult<CycleBenchmarkProtocolResult>
    where
        E: CycleBenchmarkExecutor,
        R: RandomSource + ?Sized,
    {
        let experiment = self.generate(cycle, rng)?;
        let observations = self.execute(&experiment, executor)?;
        self.analyze(&experiment, &observations)
    }
}

// =============================================================================
// Observation indexing
// =============================================================================

fn index_observations(
    observations: &[CycleExecutionObservation],
) -> CycleBenchmarkResult<BTreeMap<String, CycleExecutionObservation>> {
    let mut indexed = BTreeMap::new();

    for observation in observations {
        if indexed
            .insert(
                observation.instance_id.clone(),
                observation.clone(),
            )
            .is_some()
        {
            return Err(CycleBenchmarkError::DuplicateObservation {
                instance_id: observation.instance_id.clone(),
            });
        }
    }

    Ok(indexed)
}

// =============================================================================
// Decay fitting
// =============================================================================

/// Fits a Pauli decay.
///
/// The implementation deliberately performs a bounded deterministic search
/// over the decay parameter and analytically solves the linear amplitude/offset
/// terms for every candidate parameter.
///
/// This avoids:
///
/// - unconstrained nonlinear optimizer dependencies;
/// - hidden random initialization;
/// - backend-specific numerical libraries;
/// - local minima caused by arbitrary optimizer seeds.
///
/// It is not a replacement for a future statistically weighted regression
/// module; its result is a stable protocol-level baseline that can later be
/// cross-validated against `statistics::regression`.
fn fit_decay(
    pauli: &PauliFrame,
    points: &[CycleDecayPoint],
    model: CycleFitModel,
) -> CycleBenchmarkResult<CycleDecayFit> {
    if points.len() < MIN_FIT_SEQUENCE_LENGTHS {
        return Err(CycleBenchmarkError::InsufficientFitData {
            pauli: pauli.label(),
            lengths: points.len(),
        });
    }

    for point in points {
        if !point.mean_expectation.is_finite() {
            return Err(CycleBenchmarkError::FitFailure {
                pauli: pauli.label(),
                reason: "non-finite expectation value".to_owned(),
            });
        }

        if point.mean_expectation < -1.0 - FINITE_EPSILON
            || point.mean_expectation > 1.0 + FINITE_EPSILON
        {
            return Err(CycleBenchmarkError::FitFailure {
                pauli: pauli.label(),
                reason: format!(
                    "expectation {} is outside [-1, 1]",
                    point.mean_expectation
                ),
            });
        }
    }

    let (best_p, amplitude, offset, sse) =
        optimize_decay_parameter(points, model)?;

    let fitted_parameters = match model {
        CycleFitModel::NoOffset => 2,
        CycleFitModel::WithOffset => 3,
    };

    let mean_y =
        points.iter().map(|point| point.mean_expectation).sum::<f64>()
            / points.len() as f64;

    let total_sum_squares = points
        .iter()
        .map(|point| {
            let delta = point.mean_expectation - mean_y;
            delta * delta
        })
        .sum::<f64>();

    let r_squared = if total_sum_squares <= FINITE_EPSILON {
        if sse <= FINITE_EPSILON {
            1.0
        } else {
            0.0
        }
    } else {
        1.0 - sse / total_sum_squares
    };

    let rmse = (sse / points.len() as f64).sqrt();

    let decay_standard_error =
        estimate_decay_standard_error(points, best_p, model);

    let fit = CycleDecayFit {
        amplitude,
        decay_parameter: best_p,
        offset,
        sum_squared_error: sse,
        root_mean_squared_error: rmse,
        r_squared,
        sequence_points: points.len(),
        fitted_parameters,
        decay_standard_error,
        model,
    };

    if !fit.amplitude.is_finite()
        || !fit.decay_parameter.is_finite()
        || !fit.offset.is_finite()
        || !fit.sum_squared_error.is_finite()
        || !fit.root_mean_squared_error.is_finite()
        || !fit.r_squared.is_finite()
    {
        return Err(CycleBenchmarkError::FitFailure {
            pauli: pauli.label(),
            reason: "fit generated a non-finite parameter".to_owned(),
        });
    }

    if !fit.decay_parameter_is_physical() {
        return Err(CycleBenchmarkError::InvalidFitParameter {
            pauli: pauli.label(),
            parameter: "decay_parameter",
            value: fit.decay_parameter,
        });
    }

    Ok(fit)
}

/// Finds the decay parameter by deterministic golden-section search.
///
/// The parameter is constrained to [-1, 1], the physically allowed interval
/// for a Pauli-transfer eigenvalue. For standard CB data, the meaningful
/// solution is normally in [0, 1].
fn optimize_decay_parameter(
    points: &[CycleDecayPoint],
    model: CycleFitModel,
) -> CycleBenchmarkResult<(f64, f64, f64, f64)> {
    let mut lower = -1.0f64;
    let mut upper = 1.0f64;

    let golden_ratio = 0.618_033_988_749_894_8_f64;

    let mut x1 = upper - golden_ratio * (upper - lower);
    let mut x2 = lower + golden_ratio * (upper - lower);

    let mut f1 = linear_parameters_for_decay(points, x1, model)?;
    let mut f2 = linear_parameters_for_decay(points, x2, model)?;

    for _ in 0..MAX_FIT_ITERATIONS {
        if f1.3 <= f2.3 {
            upper = x2;
            x2 = x1;
            f2 = f1;

            x1 = upper - golden_ratio * (upper - lower);
            f1 = linear_parameters_for_decay(points, x1, model)?;
        } else {
            lower = x1;
            x1 = x2;
            f1 = f2;

            x2 = lower + golden_ratio * (upper - lower);
            f2 = linear_parameters_for_decay(points, x2, model)?;
        }
    }

    let midpoint = 0.5 * (lower + upper);
    let final_fit =
        linear_parameters_for_decay(points, midpoint, model)?;

    Ok(final_fit)
}

/// Solves the linear amplitude/offset parameters for a fixed decay parameter.
///
/// Returns:
///
/// ```text
/// (p, A, B, SSE)
/// ```
fn linear_parameters_for_decay(
    points: &[CycleDecayPoint],
    p: f64,
    model: CycleFitModel,
) -> CycleBenchmarkResult<(f64, f64, f64, f64)> {
    if !p.is_finite() {
        return Err(CycleBenchmarkError::FitFailure {
            pauli: "<unknown>".to_owned(),
            reason: "candidate decay parameter is non-finite".to_owned(),
        });
    }

    match model {
        CycleFitModel::NoOffset => {
            let mut xx = 0.0;
            let mut xy = 0.0;

            for point in points {
                let x = safe_power(p, point.sequence_length);

                xx += x * x;
                xy += x * point.mean_expectation;
            }

            if xx <= FINITE_EPSILON {
                return Err(CycleBenchmarkError::FitFailure {
                    pauli: "<unknown>".to_owned(),
                    reason: "degenerate exponential basis".to_owned(),
                });
            }

            let amplitude = xy / xx;

            if !amplitude.is_finite() {
                return Err(CycleBenchmarkError::FitFailure {
                    pauli: "<unknown>".to_owned(),
                    reason: "amplitude is non-finite".to_owned(),
                });
            }

            let mut sse = 0.0;

            for point in points {
                let prediction =
                    amplitude * safe_power(p, point.sequence_length);

                let residual =
                    point.mean_expectation - prediction;

                sse += residual * residual;
            }

            Ok((p, amplitude, 0.0, sse))
        }

        CycleFitModel::WithOffset => {
            let mut sx = 0.0;
            let mut sy = 0.0;
            let mut sxx = 0.0;
            let mut sxy = 0.0;

            for point in points {
                let x = safe_power(p, point.sequence_length);
                let y = point.mean_expectation;

                sx += x;
                sy += y;
                sxx += x * x;
                sxy += x * y;
            }

            let n = points.len() as f64;

            let determinant = n * sxx - sx * sx;

            if determinant.abs() <= FINITE_EPSILON {
                return Err(CycleBenchmarkError::FitFailure {
                    pauli: "<unknown>".to_owned(),
                    reason: "amplitude/offset linear system is singular"
                        .to_owned(),
                });
            }

            let amplitude =
                (n * sxy - sx * sy) / determinant;

            let offset =
                (sy - amplitude * sx) / n;

            if !amplitude.is_finite() || !offset.is_finite() {
                return Err(CycleBenchmarkError::FitFailure {
                    pauli: "<unknown>".to_owned(),
                    reason: "linear fit produced a non-finite parameter"
                        .to_owned(),
                });
            }

            let mut sse = 0.0;

            for point in points {
                let prediction = amplitude
                    * safe_power(p, point.sequence_length)
                    + offset;

                let residual =
                    point.mean_expectation - prediction;

                sse += residual * residual;
            }

            Ok((p, amplitude, offset, sse))
        }
    }
}

/// Estimates local uncertainty in p from the curvature of the SSE surface.
///
/// This is deliberately labelled as an approximate standard error. It is not
/// a bootstrap confidence interval and must not be represented as one by
/// reporting layers.
fn estimate_decay_standard_error(
    points: &[CycleDecayPoint],
    best_p: f64,
    model: CycleFitModel,
) -> Option<f64> {
    let h = 1.0e-4;

    let left_p = (best_p - h).max(-1.0);
    let right_p = (best_p + h).min(1.0);

    if (right_p - left_p).abs() <= FINITE_EPSILON {
        return None;
    }

    let center =
        linear_parameters_for_decay(points, best_p, model).ok()?;
    let left =
        linear_parameters_for_decay(points, left_p, model).ok()?;
    let right =
        linear_parameters_for_decay(points, right_p, model).ok()?;

    let step_left = best_p - left_p;
    let step_right = right_p - best_p;

    let curvature = if (step_left - step_right).abs() <= 1.0e-10 {
        (right.3 - 2.0 * center.3 + left.3)
            / (step_left * step_left)
    } else {
        let slope_left =
            (center.3 - left.3) / step_left;

        let slope_right =
            (right.3 - center.3) / step_right;

        2.0 * (slope_right - slope_left)
            / (step_left + step_right)
    };

    if !curvature.is_finite() || curvature <= FINITE_EPSILON {
        return None;
    }

    let residual_dof =
        points.len().saturating_sub(match model {
            CycleFitModel::NoOffset => 2,
            CycleFitModel::WithOffset => 3,
        });

    if residual_dof == 0 {
        return None;
    }

    let variance =
        center.3 / residual_dof as f64;

    if !variance.is_finite() || variance < 0.0 {
        return None;
    }

    // For SSE curvature:
    //
    // SSE ≈ SSE_min + 1/2 * H * (p-p_hat)^2
    //
    // so the local variance estimate is approximately:
    //
    // sigma² / (H/2)
    let parameter_variance =
        2.0 * variance / curvature;

    if !parameter_variance.is_finite()
        || parameter_variance < 0.0
    {
        return None;
    }

    Some(parameter_variance.sqrt())
}

// =============================================================================
// Numerical helpers
// =============================================================================

/// Computes `base^exponent` while avoiding avoidable overflow/NaN creation.
///
/// For the CB decay parameter domain `[-1, 1]`, this is numerically safe for
/// the normal sequence lengths used by the protocol.
fn safe_power(base: f64, exponent: usize) -> f64 {
    if exponent == 0 {
        return 1.0;
    }

    if base == 0.0 {
        return 0.0;
    }

    base.powi(exponent as i32)
}

/// Calculates the Hilbert-space dimension `2^n` using checked integer
/// arithmetic.
///
/// CB's process-fidelity formula needs `d²`, so callers additionally perform
/// a checked multiplication.
fn hilbert_dimension(
    qubits: usize,
) -> CycleBenchmarkResult<usize> {
    if qubits >= usize::BITS as usize {
        return Err(CycleBenchmarkError::DimensionOverflow {
            qubits,
        });
    }

    1usize
        .checked_shl(qubits as u32)
        .ok_or(CycleBenchmarkError::DimensionOverflow {
            qubits,
        })
}

/// Calculates the number of non-identity n-qubit Pauli operators.
///
/// ```text
/// 4^n - 1
/// ```
fn checked_non_identity_pauli_count(
    qubits: usize,
) -> CycleBenchmarkResult<usize> {
    if qubits == 0 {
        return Err(CycleBenchmarkError::InvalidQubitCount {
            qubits,
        });
    }

    let exponent = qubits
        .checked_mul(2)
        .ok_or(CycleBenchmarkError::DimensionOverflow {
            qubits,
        })?;

    if exponent >= usize::BITS as usize {
        return Err(CycleBenchmarkError::DimensionOverflow {
            qubits,
        });
    }

    let total = 1usize
        .checked_shl(exponent as u32)
        .ok_or(CycleBenchmarkError::DimensionOverflow {
            qubits,
        })?;

    total
        .checked_sub(1)
        .ok_or(CycleBenchmarkError::DimensionOverflow {
            qubits,
        })
}

/// Validates a process or average gate fidelity.
fn validate_fidelity(value: f64) -> CycleBenchmarkResult<()> {
    if !value.is_finite()
        || value < -FINITE_EPSILON
        || value > 1.0 + FINITE_EPSILON
    {
        return Err(CycleBenchmarkError::InvalidFidelity {
            value,
        });
    }

    Ok(())
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::quantum::benchmarking::generators::random::{
        BenchmarkSeed,
        RandomStream,
    };

    fn rng() -> RandomStream {
        RandomStream::from_seed(
            BenchmarkSeed::from_u64(0xCB_2026_08_26),
        )
    }

    #[test]
    fn identity_pauli_frame_has_correct_width() {
        let frame = PauliFrame::identity(4)
            .expect("identity Pauli frame should be valid");

        assert_eq!(frame.qubits(), 4);
        assert!(frame.is_identity());
        assert_eq!(frame.weight(), 0);
        assert_eq!(frame.label(), "IIII");
    }

    #[test]
    fn random_non_identity_frame_is_not_identity() {
        let mut random = rng();

        for _ in 0..100 {
            let frame =
                PauliFrame::random_non_identity(5, &mut random)
                    .expect("random Pauli generation should succeed");

            assert_eq!(frame.qubits(), 5);
            assert!(!frame.is_identity());
        }
    }

    #[test]
    fn cycle_definition_rejects_zero_qubits() {
        let result = CycleDefinition::new("cx", 0);

        assert!(matches!(
            result,
            Err(CycleBenchmarkError::InvalidQubitCount {
                qubits: 0
            })
        ));
    }

    #[test]
    fn configuration_rejects_unsorted_lengths() {
        let config = CycleBenchmarkConfig {
            sequence_lengths: vec![2, 8, 8],
            ..CycleBenchmarkConfig::default()
        };

        assert!(matches!(
            config.validate(),
            Err(CycleBenchmarkError::NonIncreasingSequenceLengths)
        ));
    }

    #[test]
    fn configuration_counts_instances() {
        let config = CycleBenchmarkConfig {
            qubits: 2,
            sequence_lengths: vec![2, 4, 8],
            pauli_count: 5,
            randomizations_per_length: 7,
            shots: 100,
            ..CycleBenchmarkConfig::default()
        };

        assert_eq!(
            config
                .instance_count()
                .expect("instance count should fit"),
            5 * 3 * 7
        );
    }

    #[test]
    fn random_pauli_selection_is_unique() {
        let mut random = rng();

        let paulis =
            select_random_paulis(3, 10, false, &mut random)
                .expect("Pauli selection should succeed");

        assert_eq!(paulis.len(), 10);

        let labels = paulis
            .iter()
            .map(PauliFrame::label)
            .collect::<std::collections::BTreeSet<_>>();

        assert_eq!(labels.len(), 10);

        assert!(paulis.iter().all(|pauli| !pauli.is_identity()));
    }

    #[test]
    fn random_pauli_selection_rejects_exhaustion() {
        let mut random = rng();

        // One qubit has exactly 3 non-identity Paulis.
        let result =
            select_random_paulis(1, 4, false, &mut random);

        assert!(matches!(
            result,
            Err(CycleBenchmarkError::PauliSetExhausted {
                requested: 4,
                available: 3
            })
        ));
    }

    #[test]
    fn instance_contains_m_plus_one_random_paulis() {
        let cycle =
            CycleDefinition::new("test-cycle", 2)
                .expect("cycle should be valid");

        let pauli = PauliFrame::identity(2)
            .expect("Pauli should be valid");

        let mut random = rng();

        let mut random_paulis = Vec::new();

        for _ in 0..=8 {
            random_paulis.push(
                PauliFrame::random(2, &mut random)
                    .expect("random Pauli should succeed"),
            );
        }

        let instance = CycleBenchmarkInstance::new(
            &cycle,
            pauli,
            8,
            random_paulis,
            0,
        )
        .expect("instance should be valid");

        assert_eq!(instance.random_pauli_cycles.len(), 9);
    }

    #[test]
    fn observation_expectation_is_correct() {
        let observation =
            CycleExecutionObservation::new("x", 750, 1000)
                .expect("observation should be valid");

        assert!((observation.matching_probability() - 0.75).abs() < 1e-12);
        assert!((observation.expectation() - 0.5).abs() < 1e-12);
    }

    #[test]
    fn observation_rejects_impossible_counts() {
        let result =
            CycleExecutionObservation::new("x", 1001, 1000);

        assert!(result.is_err());
    }

    #[test]
    fn no_offset_fit_recovers_exact_decay() {
        let pauli =
            PauliFrame::new(vec![Pauli::X])
                .expect("Pauli should be valid");

        let true_p = 0.92;
        let true_a = 0.83;

        let points = vec![
            CycleDecayPoint {
                sequence_length: 1,
                mean_expectation: true_a * true_p.powi(1),
                standard_error: 0.001,
                randomizations: 30,
                total_shots: 30_000,
            },
            CycleDecayPoint {
                sequence_length: 2,
                mean_expectation: true_a * true_p.powi(2),
                standard_error: 0.001,
                randomizations: 30,
                total_shots: 30_000,
            },
            CycleDecayPoint {
                sequence_length: 4,
                mean_expectation: true_a * true_p.powi(4),
                standard_error: 0.001,
                randomizations: 30,
                total_shots: 30_000,
            },
            CycleDecayPoint {
                sequence_length: 8,
                mean_expectation: true_a * true_p.powi(8),
                standard_error: 0.001,
                randomizations: 30,
                total_shots: 30_000,
            },
        ];

        let fit =
            fit_decay(&pauli, &points, CycleFitModel::NoOffset)
                .expect("fit should succeed");

        assert!(
            (fit.decay_parameter - true_p).abs() < 1e-4,
            "p={} expected {}",
            fit.decay_parameter,
            true_p
        );

        assert!(
            (fit.amplitude - true_a).abs() < 1e-4,
            "A={} expected {}",
            fit.amplitude,
            true_a
        );

        assert!(fit.r_squared > 0.9999);
    }

    #[test]
    fn offset_fit_recovers_exact_decay() {
        let pauli =
            PauliFrame::new(vec![Pauli::Y])
                .expect("Pauli should be valid");

        let true_p = 0.88;
        let true_a = 0.71;
        let true_b = 0.08;

        let points = vec![
            CycleDecayPoint {
                sequence_length: 1,
                mean_expectation:
                    true_a * true_p.powi(1) + true_b,
                standard_error: 0.001,
                randomizations: 30,
                total_shots: 30_000,
            },
            CycleDecayPoint {
                sequence_length: 2,
                mean_expectation:
                    true_a * true_p.powi(2) + true_b,
                standard_error: 0.001,
                randomizations: 30,
                total_shots: 30_000,
            },
            CycleDecayPoint {
                sequence_length: 4,
                mean_expectation:
                    true_a * true_p.powi(4) + true_b,
                standard_error: 0.001,
                randomizations: 30,
                total_shots: 30_000,
            },
            CycleDecayPoint {
                sequence_length: 8,
                mean_expectation:
                    true_a * true_p.powi(8) + true_b,
                standard_error: 0.001,
                randomizations: 30,
                total_shots: 30_000,
            },
        ];

        let fit =
            fit_decay(&pauli, &points, CycleFitModel::WithOffset)
                .expect("fit should succeed");

        assert!(
            (fit.decay_parameter - true_p).abs() < 1e-4,
            "p={} expected {}",
            fit.decay_parameter,
            true_p
        );

        assert!(
            (fit.amplitude - true_a).abs() < 1e-4,
            "A={} expected {}",
            fit.amplitude,
            true_a
        );

        assert!(
            (fit.offset - true_b).abs() < 1e-4,
            "B={} expected {}",
            fit.offset,
            true_b
        );
    }

    #[test]
    fn composite_process_fidelity_for_perfect_cycle_is_one() {
        let pauli =
            PauliFrame::new(vec![Pauli::X])
                .expect("Pauli should be valid");

        let points = vec![
            CycleDecayPoint {
                sequence_length: 1,
                mean_expectation: 1.0,
                standard_error: 0.0,
                randomizations: 30,
                total_shots: 30_000,
            },
            CycleDecayPoint {
                sequence_length: 2,
                mean_expectation: 1.0,
                standard_error: 0.0,
                randomizations: 30,
                total_shots: 30_000,
            },
            CycleDecayPoint {
                sequence_length: 4,
                mean_expectation: 1.0,
                standard_error: 0.0,
                randomizations: 30,
                total_shots: 30_000,
            },
        ];

        let fit =
            fit_decay(&pauli, &points, CycleFitModel::NoOffset)
                .expect("perfect fit should succeed");

        let result = PauliCycleResult {
            pauli,
            decay_points: points,
            fit,
        };

        let fidelity =
            CompositeCycleFidelity::from_pauli_results(1, &[result])
                .expect("fidelity should be computable");

        assert!((fidelity.process_fidelity - 1.0).abs() < 1e-12);
        assert!((fidelity.average_gate_fidelity - 1.0).abs() < 1e-12);
    }

    #[test]
    fn hilbert_dimension_is_checked() {
        assert_eq!(
            hilbert_dimension(3).expect("dimension should fit"),
            8
        );
    }

    #[test]
    fn process_fidelity_marks_sampled_pauli_set() {
        let make_result = |label: Pauli| {
            let pauli =
                PauliFrame::new(vec![label])
                    .expect("Pauli should be valid");

            let points = vec![
                CycleDecayPoint {
                    sequence_length: 1,
                    mean_expectation: 0.99,
                    standard_error: 0.01,
                    randomizations: 20,
                    total_shots: 20_000,
                },
                CycleDecayPoint {
                    sequence_length: 2,
                    mean_expectation: 0.98,
                    standard_error: 0.01,
                    randomizations: 20,
                    total_shots: 20_000,
                },
                CycleDecayPoint {
                    sequence_length: 4,
                    mean_expectation: 0.96,
                    standard_error: 0.01,
                    randomizations: 20,
                    total_shots: 20_000,
                },
            ];

            let fit =
                fit_decay(&pauli, &points, CycleFitModel::NoOffset)
                    .expect("fit should succeed");

            PauliCycleResult {
                pauli,
                decay_points: points,
                fit,
            }
        };

        let result =
            CompositeCycleFidelity::from_pauli_results(
                1,
                &[make_result(Pauli::X)],
            )
            .expect("sampled fidelity should be computable");

        assert!(!result.exhaustive);
        assert_eq!(result.pauli_terms, 1);
        assert_eq!(result.complete_non_identity_terms, 3);
    }
}