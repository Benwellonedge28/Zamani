//! Zamani Quantum Error Correction — deterministic physical noise.
//!
//! # Ownership
//!
//! `noise.rs` owns the representation, validation, configuration and
//! deterministic generation of physical noise/faults.
//!
//! It owns:
//!
//! - validated physical qubit identifiers;
//! - fixed-point probabilities;
//! - Pauli errors;
//! - fault classification;
//! - physical fault representation;
//! - bounded fault batches;
//! - deterministic noise seeds;
//! - noise-model configuration;
//! - model-independent `NoiseModel` execution;
//! - standard noise models;
//! - correlated faults;
//! - leakage and erasure events;
//! - measurement/readout faults;
//! - deterministic sampling;
//! - resource preflight;
//! - cancellation checkpoints;
//! - canonical conversion to `QecError`.
//!
//! It does NOT own:
//!
//! - syndrome decoding;
//! - logical correction;
//! - Pauli-frame evolution;
//! - QPU credentials;
//! - network I/O;
//! - backend execution;
//! - telemetry transport;
//! - checkpoint persistence;
//! - scheduler policy;
//! - distributed coordination;
//! - statistical confidence intervals.
//!
//! # Integration contract
//!
//! ```text
//! QecConfig
//!    |
//!    +---- QecLimits
//!    |
//!    +---- CancellationToken
//!    |
//!    +---- Deterministic seed
//!             |
//!             v
//!        NoiseModel
//!             |
//!             v
//!       FaultBatch
//!             |
//!      +------+-------+
//!      |              |
//!      v              v
//! syndrome.rs    simulation.rs
//!      |              |
//!      v              v
//! decoder.rs      statistics
//! ```
//!
//! `backend.rs` treats generated faults as backend-independent data.
//!
//! `syndrome_extractor.rs` converts physical faults/measurements into
//! validated syndrome and detection events.
//!
//! `simulation.rs` uses `NoiseModel::sample` with deterministic per-shot
//! seeds.
//!
//! `qpu_adapter.rs` may use the fault representation for calibrated or
//! simulated hardware models, but this module never receives QPU credentials.
//!
//! `limits.rs` remains the sole declarative resource-policy owner.
//!
//! `resources.rs` owns runtime accounting.
//!
//! `memory.rs` owns allocation/reservation enforcement.
//!
//! `cancellation.rs` owns cancellation state.
//!
//! `deterministic.rs` owns global deterministic execution policy.
//!
//! # Determinism
//!
//! Noise generation never uses a hidden global RNG.
//!
//! A caller supplies a seed. The seed is mixed with stable identifiers and
//! event indices to derive deterministic pseudo-random values.
//!
//! The same:
//!
//! ```text
//! model + configuration + qubit set + seed
//! ```
//!
//! produces the same:
//!
//! ```text
//! FaultBatch
//! ```
//!
//! regardless of process address space or thread scheduling.
//!
//! # Resource safety
//!
//! Every batch constructor that can allocate accepts `QecLimits`.
//!
//! The module never allocates an unbounded collection from an untrusted
//! requested count.
//!
//! # Rust compatibility
//!
//! Designed for Rust 1.97.1 using stable standard-library facilities only.

#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;
use std::collections::BTreeMap;

use super::cancellation::CancellationToken;
use super::errors::{
    DecoderKind,
    NumericalOperation,
    QecError,
    QecResult,
    ResourceKind,
};
use super::limits::QecLimits;

// ============================================================================
// Constants
// ============================================================================

/// Maximum supported physical qubit identifier.
///
/// This is an API-safety boundary and not a statement about QPU capacity.
pub const MAX_QUBIT_INDEX: usize = 1_000_000_000;

/// Maximum number of qubits in one correlated fault.
pub const MAX_CORRELATED_QUBITS: usize = 1_000;

/// Maximum fault count accepted by the convenience constructor.
///
/// Production code should prefer `FaultBatch::with_limits`.
pub const MAX_FAULTS_PER_BATCH: usize = 1_000_000;

/// Fixed-point probability scale.
///
/// `PROBABILITY_SCALE` represents exactly 100%.
pub const PROBABILITY_SCALE: u64 = 1_000_000_000_000;

/// Zero probability.
pub const PROBABILITY_ZERO: Probability = Probability(0);

/// One probability.
pub const PROBABILITY_ONE: Probability =
    Probability(PROBABILITY_SCALE);

// ============================================================================
// Noise error
// ============================================================================

/// Errors specific to physical-noise construction and sampling.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NoiseError {
    /// Physical qubit identifier is outside the API safety range.
    InvalidQubitId {
        id: usize,
    },

    /// Fixed-point probability exceeds 100%.
    InvalidProbability {
        scaled: u64,
    },

    /// Percentage exceeds 100.
    InvalidPercentage {
        percent: u8,
    },

    /// Basis points exceed 10,000.
    InvalidBasisPoints {
        basis_points: u16,
    },

    /// An arithmetic operation could not be completed safely.
    ArithmeticOverflow,

    /// A probability distribution does not sum to one.
    InvalidDistribution,

    /// An operation is incompatible with a fault type.
    InvalidOperation {
        operation: NoiseOperation,
        message: String,
    },

    /// An identity Pauli was supplied as a physical fault.
    IdentityFault,

    /// Correlated fault contains no qubits.
    EmptyCorrelatedFault,

    /// Correlated qubit and Pauli arrays differ in length.
    MismatchedCorrelatedLengths {
        qubits: usize,
        paulis: usize,
    },

    /// Correlated fault exceeds the explicit safety boundary.
    CorrelatedFaultTooLarge {
        requested: usize,
        maximum: usize,
    },

    /// Correlated qubits are not strictly increasing.
    NonCanonicalCorrelatedQubits,

    /// Requested fault count exceeds the permitted limit.
    FaultLimitExceeded {
        requested: usize,
        maximum: usize,
    },

    /// Model configuration is invalid.
    InvalidModel(String),

    /// A requested model operation is unsupported.
    UnsupportedModelOperation(String),

    /// Sampling was cancelled.
    Cancelled,

    /// A configured resource limit rejected the operation.
    ResourceLimitExceeded {
        resource: &'static str,
        requested: u128,
        maximum: u128,
    },
}

impl fmt::Display for NoiseError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidQubitId { id } => {
                write!(
                    formatter,
                    "invalid physical qubit id: {id}"
                )
            }

            Self::InvalidProbability { scaled } => {
                write!(
                    formatter,
                    "invalid fixed-point probability: {scaled}"
                )
            }

            Self::InvalidPercentage { percent } => {
                write!(
                    formatter,
                    "invalid probability percentage: {percent}"
                )
            }

            Self::InvalidBasisPoints { basis_points } => {
                write!(
                    formatter,
                    "invalid probability basis points: {basis_points}"
                )
            }

            Self::ArithmeticOverflow => {
                write!(
                    formatter,
                    "noise arithmetic overflow"
                )
            }

            Self::InvalidDistribution => {
                write!(
                    formatter,
                    "noise probability distribution is invalid"
                )
            }

            Self::InvalidOperation {
                operation,
                message,
            } => {
                write!(
                    formatter,
                    "invalid noise operation {operation}: {message}"
                )
            }

            Self::IdentityFault => {
                write!(
                    formatter,
                    "identity cannot be represented as a physical fault"
                )
            }

            Self::EmptyCorrelatedFault => {
                write!(
                    formatter,
                    "correlated fault must contain at least one qubit"
                )
            }

            Self::MismatchedCorrelatedLengths {
                qubits,
                paulis,
            } => {
                write!(
                    formatter,
                    "correlated fault has {qubits} qubits and {paulis} Paulis"
                )
            }

            Self::CorrelatedFaultTooLarge {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "correlated fault contains {requested} qubits; maximum is {maximum}"
                )
            }

            Self::NonCanonicalCorrelatedQubits => {
                write!(
                    formatter,
                    "correlated qubits must be strictly increasing"
                )
            }

            Self::FaultLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "fault count {requested} exceeds maximum {maximum}"
                )
            }

            Self::InvalidModel(message) => {
                write!(
                    formatter,
                    "invalid noise model: {message}"
                )
            }

            Self::UnsupportedModelOperation(message) => {
                write!(
                    formatter,
                    "unsupported noise-model operation: {message}"
                )
            }

            Self::Cancelled => {
                write!(
                    formatter,
                    "noise sampling cancelled"
                )
            }

            Self::ResourceLimitExceeded {
                resource,
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "noise {resource} request {requested} exceeds maximum {maximum}"
                )
            }
        }
    }
}

impl std::error::Error for NoiseError {}

impl From<NoiseError> for QecError {
    fn from(error: NoiseError) -> Self {
        match error {
            NoiseError::InvalidQubitId { id } => {
                QecError::invalid_input(format!(
                    "invalid physical qubit id: {id}"
                ))
            }

            NoiseError::InvalidProbability { scaled } => {
                QecError::InvalidProbability {
                    probability: scaled as f64
                        / PROBABILITY_SCALE as f64,
                    message: format!(
                        "fixed-point probability {scaled} exceeds 100%"
                    ),
                }
            }

            NoiseError::InvalidPercentage { percent } => {
                QecError::InvalidProbability {
                    probability: percent as f64 / 100.0,
                    message: format!(
                        "percentage {percent} exceeds 100%"
                    ),
                }
            }

            NoiseError::InvalidBasisPoints { basis_points } => {
                QecError::InvalidProbability {
                    probability: basis_points as f64 / 10_000.0,
                    message: format!(
                        "basis points {basis_points} exceed 100%"
                    ),
                }
            }

            NoiseError::ArithmeticOverflow => {
                QecError::numerical_failure(
                    NumericalOperation::Accumulation,
                    "noise arithmetic overflow",
                )
            }

            NoiseError::InvalidDistribution => {
                QecError::invalid_input(
                    "noise probability distribution is invalid",
                )
            }

            NoiseError::InvalidOperation {
                operation,
                message,
            } => {
                QecError::invalid_input(format!(
                    "invalid noise operation {operation}: {message}"
                ))
            }

            NoiseError::IdentityFault => {
                QecError::invalid_input(
                    "identity cannot be represented as a physical fault",
                )
            }

            NoiseError::EmptyCorrelatedFault => {
                QecError::invalid_input(
                    "correlated fault cannot be empty",
                )
            }

            NoiseError::MismatchedCorrelatedLengths {
                qubits,
                paulis,
            } => {
                QecError::invalid_input(format!(
                    "correlated fault length mismatch: {qubits} qubits, {paulis} Paulis"
                ))
            }

            NoiseError::CorrelatedFaultTooLarge {
                requested,
                maximum,
            } => {
                QecError::resource_limit(
                    ResourceKind::AllocationCount,
                    requested as u128,
                    maximum as u128,
                    "correlated noise fault exceeds configured safety boundary",
                )
            }

            NoiseError::NonCanonicalCorrelatedQubits => {
                QecError::invalid_input(
                    "correlated fault qubits must be strictly increasing",
                )
            }

            NoiseError::FaultLimitExceeded {
                requested,
                maximum,
            } => {
                QecError::resource_limit(
                    ResourceKind::AllocationCount,
                    requested as u128,
                    maximum as u128,
                    "noise fault batch exceeds permitted size",
                )
            }

            NoiseError::InvalidModel(message) => {
                QecError::invalid_input(message)
            }

            NoiseError::UnsupportedModelOperation(message) => {
                QecError::invalid_input(message)
            }

            NoiseError::Cancelled => {
                QecError::cancelled(
                    "noise sampling cancellation requested",
                )
            }

            NoiseError::ResourceLimitExceeded {
                resource,
                requested,
                maximum,
            } => {
                QecError::resource_limit(
                    ResourceKind::AllocationCount,
                    requested,
                    maximum,
                    resource,
                )
            }
        }
    }
}

// ============================================================================
// Qubit identifier
// ============================================================================

/// Validated physical qubit identifier.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
pub struct QubitId(usize);

impl QubitId {
    /// Creates a validated physical qubit identifier.
    pub const fn new(
        id: usize,
    ) -> Result<Self, NoiseError> {
        if id > MAX_QUBIT_INDEX {
            return Err(
                NoiseError::InvalidQubitId { id },
            );
        }

        Ok(Self(id))
    }

    /// Returns the underlying physical index.
    pub const fn index(
        self,
    ) -> usize {
        self.0
    }
}

impl fmt::Display for QubitId {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            formatter,
            "q{}",
            self.0
        )
    }
}

// ============================================================================
// Probability
// ============================================================================

/// Deterministic fixed-point probability.
///
/// ```text
/// 0                    = 0%
/// 500_000_000_000      = 50%
/// 1_000_000_000_000    = 100%
/// ```
///
/// Fixed-point representation is used for configuration equality,
/// deterministic hashing and reproducible model configuration.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
pub struct Probability(u64);

impl Probability {
    /// Creates a probability from fixed-point units.
    pub const fn from_scaled(
        scaled: u64,
    ) -> Result<Self, NoiseError> {
        if scaled > PROBABILITY_SCALE {
            return Err(
                NoiseError::InvalidProbability {
                    scaled,
                },
            );
        }

        Ok(Self(scaled))
    }

    /// Creates a probability from an integer percentage.
    pub fn from_percent(
        percent: u8,
    ) -> Result<Self, NoiseError> {
        if percent > 100 {
            return Err(
                NoiseError::InvalidPercentage {
                    percent,
                },
            );
        }

        let scaled = u64::from(percent)
            .checked_mul(
                PROBABILITY_SCALE / 100,
            )
            .ok_or(
                NoiseError::ArithmeticOverflow,
            )?;

        Self::from_scaled(scaled)
    }

    /// Creates a probability from basis points.
    pub fn from_basis_points(
        basis_points: u16,
    ) -> Result<Self, NoiseError> {
        if basis_points > 10_000 {
            return Err(
                NoiseError::InvalidBasisPoints {
                    basis_points,
                },
            );
        }

        let scaled = u64::from(basis_points)
            .checked_mul(
                PROBABILITY_SCALE / 10_000,
            )
            .ok_or(
                NoiseError::ArithmeticOverflow,
            )?;

        Self::from_scaled(scaled)
    }

    /// Returns the fixed-point representation.
    pub const fn scaled(
        self,
    ) -> u64 {
        self.0
    }

    /// Converts the value to `f64` for presentation/statistical APIs.
    pub fn as_f64(
        self,
    ) -> f64 {
        self.0 as f64
            / PROBABILITY_SCALE as f64
    }

    /// Returns the complement.
    pub const fn complement(
        self,
    ) -> Self {
        Self(
            PROBABILITY_SCALE
                - self.0,
        )
    }

    /// Returns true for zero.
    pub const fn is_zero(
        self,
    ) -> bool {
        self.0 == 0
    }

    /// Returns true for one.
    pub const fn is_one(
        self,
    ) -> bool {
        self.0 == PROBABILITY_SCALE
    }

    /// Returns true when the probability is strictly between zero and one.
    pub const fn is_partial(
        self,
    ) -> bool {
        self.0 != 0
            && self.0 != PROBABILITY_SCALE
    }
}

impl Default for Probability {
    fn default() -> Self {
        PROBABILITY_ZERO
    }
}

// ============================================================================
// Pauli
// ============================================================================

/// Single-qubit Pauli operator.
///
/// Global phase is intentionally omitted.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
pub enum PauliError {
    I,
    X,
    Y,
    Z,
}

impl PauliError {
    /// Returns true for identity.
    pub const fn is_identity(
        self,
    ) -> bool {
        matches!(
            self,
            Self::I
        )
    }

    /// Returns true for a physical non-identity Pauli.
    pub const fn is_non_identity(
        self,
    ) -> bool {
        !self.is_identity()
    }

    /// Multiplies two Paulis while discarding global phase.
    pub const fn multiply(
        self,
        rhs: Self,
    ) -> Self {
        use PauliError::*;

        match (
            self,
            rhs,
        ) {
            (I, value)
            | (value, I) => value,

            (X, X)
            | (Y, Y)
            | (Z, Z) => I,

            (X, Y)
            | (Y, X) => Z,

            (X, Z)
            | (Z, X) => Y,

            (Y, Z)
            | (Z, Y) => X,
        }
    }

    /// Returns true when the two Paulis commute.
    pub const fn commutes(
        self,
        rhs: Self,
    ) -> bool {
        match (
            self,
            rhs,
        ) {
            (Self::I, _)
            | (_, Self::I)
            | (Self::X, Self::X)
            | (Self::Y, Self::Y)
            | (Self::Z, Self::Z) => true,

            _ => false,
        }
    }

    /// Returns true when the two Paulis anticommute.
    pub const fn anticommutes(
        self,
        rhs: Self,
    ) -> bool {
        !self.commutes(rhs)
    }

    /// Stable numeric representation.
    pub const fn as_u8(
        self,
    ) -> u8 {
        match self {
            Self::I => 0,
            Self::X => 1,
            Self::Y => 2,
            Self::Z => 3,
        }
    }
}

// ============================================================================
// Noise operation
// ============================================================================

/// Physical operation to which a fault belongs.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
pub enum NoiseOperation {
    Qubit,
    Gate,
    Measurement,
    Reset,
    Idle,
}

impl NoiseOperation {
    /// Returns whether this operation accepts Pauli faults.
    pub const fn supports_pauli(
        self,
    ) -> bool {
        !matches!(
            self,
            Self::Measurement
        )
    }
}

// ============================================================================
// Fault classification
// ============================================================================

/// Classification of a generated physical fault.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
pub enum FaultKind {
    Pauli,
    Measurement,
    Reset,
    Correlated,
    Leakage,
    Erasure,
}

impl FaultKind {
    /// Stable identifier.
    pub const fn as_str(
        self,
    ) -> &'static str {
        match self {
            Self::Pauli => "pauli",
            Self::Measurement => "measurement",
            Self::Reset => "reset",
            Self::Correlated => "correlated",
            Self::Leakage => "leakage",
            Self::Erasure => "erasure",
        }
    }
}

// ============================================================================
// Fault
// ============================================================================

/// Explicit physical fault representation.
///
/// The enum deliberately contains no decoder-specific information.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
)]
pub enum Fault {
    /// Single-qubit Pauli fault.
    Pauli {
        operation: NoiseOperation,
        qubit: QubitId,
        pauli: PauliError,
    },

    /// Measurement/readout corruption.
    Measurement {
        qubit: QubitId,
    },

    /// Reset/preparation corruption.
    Reset {
        qubit: QubitId,
        pauli: PauliError,
    },

    /// Multi-qubit correlated Pauli fault.
    Correlated {
        operation: NoiseOperation,
        qubits: Vec<QubitId>,
        paulis: Vec<PauliError>,
    },

    /// Leakage event.
    Leakage {
        operation: NoiseOperation,
        qubit: QubitId,
    },

    /// Erasure event.
    Erasure {
        operation: NoiseOperation,
        qubit: QubitId,
    },
}

impl Fault {
    /// Creates a single-qubit Pauli fault.
    pub fn pauli(
        operation: NoiseOperation,
        qubit: QubitId,
        pauli: PauliError,
    ) -> Result<Self, NoiseError> {
        if !operation.supports_pauli() {
            return Err(
                NoiseError::InvalidOperation {
                    operation,
                    message:
                        "measurement operation requires a measurement fault"
                            .to_owned(),
                },
            );
        }

        if pauli.is_identity() {
            return Err(
                NoiseError::IdentityFault,
            );
        }

        Ok(Self::Pauli {
            operation,
            qubit,
            pauli,
        })
    }

    /// Creates a measurement fault.
    pub const fn measurement(
        qubit: QubitId,
    ) -> Self {
        Self::Measurement {
            qubit,
        }
    }

    /// Creates a reset fault.
    pub fn reset(
        qubit: QubitId,
        pauli: PauliError,
    ) -> Result<Self, NoiseError> {
        if pauli.is_identity() {
            return Err(
                NoiseError::IdentityFault,
            );
        }

        Ok(Self::Reset {
            qubit,
            pauli,
        })
    }

    /// Creates a leakage event.
    pub const fn leakage(
        operation: NoiseOperation,
        qubit: QubitId,
    ) -> Self {
        Self::Leakage {
            operation,
            qubit,
        }
    }

    /// Creates an erasure event.
    pub const fn erasure(
        operation: NoiseOperation,
        qubit: QubitId,
    ) -> Self {
        Self::Erasure {
            operation,
            qubit,
        }
    }

    /// Creates a canonical correlated Pauli fault.
    pub fn correlated(
        operation: NoiseOperation,
        qubits: Vec<QubitId>,
        paulis: Vec<PauliError>,
    ) -> Result<Self, NoiseError> {
        if !operation.supports_pauli() {
            return Err(
                NoiseError::InvalidOperation {
                    operation,
                    message:
                        "measurement operation cannot carry correlated Pauli faults"
                            .to_owned(),
                },
            );
        }

        if qubits.is_empty() {
            return Err(
                NoiseError::EmptyCorrelatedFault,
            );
        }

        if qubits.len() != paulis.len() {
            return Err(
                NoiseError::MismatchedCorrelatedLengths {
                    qubits: qubits.len(),
                    paulis: paulis.len(),
                },
            );
        }

        if qubits.len() > MAX_CORRELATED_QUBITS {
            return Err(
                NoiseError::CorrelatedFaultTooLarge {
                    requested: qubits.len(),
                    maximum: MAX_CORRELATED_QUBITS,
                },
            );
        }

        for index in 0..qubits.len() {
            if paulis[index].is_identity() {
                return Err(
                    NoiseError::IdentityFault,
                );
            }

            if index > 0
                && qubits[index - 1] >= qubits[index]
            {
                return Err(
                    NoiseError::NonCanonicalCorrelatedQubits,
                );
            }
        }

        Ok(Self::Correlated {
            operation,
            qubits,
            paulis,
        })
    }

    /// Returns the fault classification.
    pub const fn kind(
        &self,
    ) -> FaultKind {
        match self {
            Self::Pauli { .. } => FaultKind::Pauli,
            Self::Measurement { .. } => FaultKind::Measurement,
            Self::Reset { .. } => FaultKind::Reset,
            Self::Correlated { .. } => FaultKind::Correlated,
            Self::Leakage { .. } => FaultKind::Leakage,
            Self::Erasure { .. } => FaultKind::Erasure,
        }
    }

    /// Returns the number of affected qubits.
    pub fn weight(
        &self,
    ) -> usize {
        match self {
            Self::Pauli { .. }
            | Self::Measurement { .. }
            | Self::Reset { .. }
            | Self::Leakage { .. }
            | Self::Erasure { .. } => 1,

            Self::Correlated {
                qubits,
                ..
            } => qubits.len(),
        }
    }

    /// Returns the operation associated with the fault.
    pub const fn operation(
        &self,
    ) -> NoiseOperation {
        match self {
            Self::Pauli {
                operation,
                ..
            }
            | Self::Correlated {
                operation,
                ..
            }
            | Self::Leakage {
                operation,
                ..
            }
            | Self::Erasure {
                operation,
                ..
            } => *operation,

            Self::Measurement { .. } => {
                NoiseOperation::Measurement
            }

            Self::Reset { .. } => {
                NoiseOperation::Reset
            }
        }
    }

    /// Returns the primary qubit when one exists.
    pub fn primary_qubit(
        &self,
    ) -> Option<QubitId> {
        match self {
            Self::Pauli { qubit, .. }
            | Self::Measurement { qubit }
            | Self::Reset { qubit, .. }
            | Self::Leakage { qubit, .. }
            | Self::Erasure { qubit, .. } => {
                Some(*qubit)
            }

            Self::Correlated {
                qubits,
                ..
            } => qubits.first().copied(),
        }
    }
}

// ============================================================================
// Fault batch
// ============================================================================

/// Bounded collection of physical faults.
///
/// Faults are kept in deterministic canonical order.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub struct FaultBatch {
    faults: Vec<Fault>,
    seed: u64,
    qubit_count: usize,
}

impl FaultBatch {
    /// Creates an empty batch.
    pub fn new(
        seed: u64,
    ) -> Self {
        Self {
            faults: Vec::new(),
            seed,
            qubit_count: 0,
        }
    }

    /// Creates a batch with explicit resource policy.
    pub fn with_limits(
        seed: u64,
        capacity: usize,
        limits: &QecLimits,
    ) -> QecResult<Self> {
        limits
            .validate_syndrome(capacity)
            .map_err(|error| {
                QecError::resource_limit(
                    ResourceKind::AllocationCount,
                    capacity as u128,
                    limits.max_syndrome_events as u128,
                    error.to_string(),
                )
            })?;

        Ok(Self {
            faults: Vec::with_capacity(
                capacity,
            ),
            seed,
            qubit_count: 0,
        })
    }

    /// Adds one fault while enforcing the supplied limit.
    pub fn push(
        &mut self,
        fault: Fault,
        limits: &QecLimits,
    ) -> QecResult<()> {
        let next = self
            .faults
            .len()
            .checked_add(1)
            .ok_or_else(|| {
                QecError::numerical_failure(
                    NumericalOperation::Accumulation,
                    "fault batch length overflow",
                )
            })?;

        limits
            .validate_syndrome(next)
            .map_err(|error| {
                QecError::resource_limit(
                    ResourceKind::AllocationCount,
                    next as u128,
                    limits.max_syndrome_events as u128,
                    error.to_string(),
                )
            })?;

        self.qubit_count = self
            .qubit_count
            .max(
                fault
                    .primary_qubit()
                    .map_or(0, |q| q.index().saturating_add(1)),
            );

        self.faults.push(fault);

        Ok(())
    }

    /// Adds one fault using the explicit batch safety boundary.
    pub fn push_bounded(
        &mut self,
        fault: Fault,
    ) -> Result<(), NoiseError> {
        if self.faults.len()
            >= MAX_FAULTS_PER_BATCH
        {
            return Err(
                NoiseError::FaultLimitExceeded {
                    requested: self
                        .faults
                        .len()
                        .saturating_add(1),
                    maximum:
                        MAX_FAULTS_PER_BATCH,
                },
            );
        }

        self.qubit_count = self
            .qubit_count
            .max(
                fault
                    .primary_qubit()
                    .map_or(0, |q| q.index().saturating_add(1)),
            );

        self.faults.push(fault);

        Ok(())
    }

    /// Sorts faults into deterministic canonical order.
    pub fn canonicalize(
        &mut self,
    ) {
        self.faults.sort_by(
            |left, right| {
                (
                    left.primary_qubit(),
                    left.kind(),
                    left.operation(),
                    left.weight(),
                )
                    .cmp(&(
                        right.primary_qubit(),
                        right.kind(),
                        right.operation(),
                        right.weight(),
                    ))
            },
        );
    }

    /// Returns the deterministic seed.
    pub const fn seed(
        &self,
    ) -> u64 {
        self.seed
    }

    /// Returns the number of faults.
    pub fn len(
        &self,
    ) -> usize {
        self.faults.len()
    }

    /// Returns true when no faults are present.
    pub fn is_empty(
        &self,
    ) -> bool {
        self.faults.is_empty()
    }

    /// Returns the highest referenced qubit count.
    pub fn qubit_count(
        &self,
    ) -> usize {
        self.qubit_count
    }

    /// Returns an immutable fault slice.
    pub fn as_slice(
        &self,
    ) -> &[Fault] {
        &self.faults
    }

    /// Returns an iterator over faults.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = &Fault> {
        self.faults.iter()
    }

    /// Counts faults by classification.
    pub fn counts_by_kind(
        &self,
    ) -> BTreeMap<FaultKind, usize> {
        let mut counts =
            BTreeMap::new();

        for fault in &self.faults {
            let entry =
                counts
                    .entry(fault.kind())
                    .or_insert(0);

            *entry = entry.saturating_add(1);
        }

        counts
    }
}

impl Default for FaultBatch {
    fn default() -> Self {
        Self::new(0)
    }
}

// ============================================================================
// Noise seed
// ============================================================================

/// Explicit deterministic noise seed.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub struct NoiseSeed(u64);

impl NoiseSeed {
    /// Creates a seed.
    pub const fn new(
        seed: u64,
    ) -> Self {
        Self(seed)
    }

    /// Returns the raw seed.
    pub const fn value(
        self,
    ) -> u64 {
        self.0
    }

    /// Derives a stable child seed.
    pub fn derive(
        self,
        stream: u64,
    ) -> Self {
        Self(
            splitmix64(
                self.0
                    ^ stream.rotate_left(17),
            ),
        )
    }
}

impl From<u64> for NoiseSeed {
    fn from(
        value: u64,
    ) -> Self {
        Self::new(value)
    }
}

// ============================================================================
// Noise model kind
// ============================================================================

/// Standard model family.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
pub enum NoiseModelKind {
    Pauli,
    Depolarizing,
    BitFlip,
    PhaseFlip,
    MeasurementError,
    Leakage,
    Erasure,
    Correlated,
    Crosstalk,
    Thermal,
    AmplitudeDamping,
    Readout,
    HardwareCalibrated,
}

impl NoiseModelKind {
    /// Stable identifier.
    pub const fn as_str(
        self,
    ) -> &'static str {
        match self {
            Self::Pauli => "pauli",
            Self::Depolarizing => "depolarizing",
            Self::BitFlip => "bit_flip",
            Self::PhaseFlip => "phase_flip",
            Self::MeasurementError => "measurement_error",
            Self::Leakage => "leakage",
            Self::Erasure => "erasure",
            Self::Correlated => "correlated",
            Self::Crosstalk => "crosstalk",
            Self::Thermal => "thermal",
            Self::AmplitudeDamping => "amplitude_damping",
            Self::Readout => "readout",
            Self::HardwareCalibrated => "hardware_calibrated",
        }
    }
}

// ============================================================================
// Model configuration
// ============================================================================

/// Canonical configuration for noise generation.
///
/// The configuration is deliberately independent from simulation options.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub struct NoiseConfig {
    /// Model family.
    pub kind: NoiseModelKind,

    /// Base physical error probability.
    pub probability: Probability,

    /// Optional X probability.
    pub x_probability: Probability,

    /// Optional Y probability.
    pub y_probability: Probability,

    /// Optional Z probability.
    pub z_probability: Probability,

    /// Measurement/readout probability.
    pub measurement_probability: Probability,

    /// Reset/preparation probability.
    pub reset_probability: Probability,

    /// Leakage probability.
    pub leakage_probability: Probability,

    /// Erasure probability.
    pub erasure_probability: Probability,

    /// Correlated-fault probability.
    pub correlation_probability: Probability,

    /// Maximum correlation weight.
    pub correlation_weight: usize,

    /// Optional temperature parameter in kelvin.
    ///
    /// This value is metadata for models that use it; it is not interpreted
    /// as a physical calibration constant by the generic model.
    pub temperature_millikelvin: Option<u64>,

    /// Backend/device identifier for calibrated models.
    pub hardware_id: Option<String>,
}

impl Default for NoiseConfig {
    fn default() -> Self {
        Self {
            kind: NoiseModelKind::Depolarizing,
            probability: PROBABILITY_ZERO,
            x_probability: PROBABILITY_ZERO,
            y_probability: PROBABILITY_ZERO,
            z_probability: PROBABILITY_ZERO,
            measurement_probability: PROBABILITY_ZERO,
            reset_probability: PROBABILITY_ZERO,
            leakage_probability: PROBABILITY_ZERO,
            erasure_probability: PROBABILITY_ZERO,
            correlation_probability: PROBABILITY_ZERO,
            correlation_weight: 2,
            temperature_millikelvin: None,
            hardware_id: None,
        }
    }
}

impl NoiseConfig {
    /// Validates the complete model configuration.
    pub fn validate(
        &self,
        limits: &QecLimits,
    ) -> QecResult<()> {
        limits
            .validate_stabilizer_weight(
                self.correlation_weight,
            )
            .map_err(|error| {
                QecError::resource_limit(
                    ResourceKind::AllocationCount,
                    self.correlation_weight as u128,
                    limits.max_stabilizer_weight as u128,
                    error.to_string(),
                )
            })?;

        if self.correlation_weight == 0 {
            return Err(
                QecError::invalid_input(
                    "noise correlation_weight must be greater than zero",
                ),
            );
        }

        if matches!(
            self.kind,
            NoiseModelKind::HardwareCalibrated
        ) && self
            .hardware_id
            .as_ref()
            .is_none_or(String::is_empty)
        {
            return Err(
                QecError::invalid_input(
                    "hardware-calibrated noise requires a hardware_id",
                ),
            );
        }

        if self.kind
            == NoiseModelKind::Pauli
        {
            let sum = self
                .x_probability
                .scaled()
                .checked_add(
                    self.y_probability.scaled(),
                )
                .and_then(|value| {
                    value.checked_add(
                        self.z_probability.scaled(),
                    )
                })
                .ok_or_else(|| {
                    QecError::numerical_failure(
                        NumericalOperation::Accumulation,
                        "Pauli noise probability overflow",
                    )
                })?;

            if sum > PROBABILITY_SCALE {
                return Err(
                    QecError::invalid_input(
                        "Pauli X/Y/Z probabilities exceed 100%",
                    ),
                );
            }
        }

        Ok(())
    }
}

// ============================================================================
// Noise model trait
// ============================================================================

/// Model-independent deterministic physical noise interface.
///
/// Implementations must:
///
/// - use only the supplied seed;
/// - never access global random state;
/// - never allocate without a bounded request;
/// - never perform network I/O;
/// - poll cancellation during expensive generation;
/// - return faults rather than decoder results.
pub trait NoiseModel:
    Send + Sync
{
    /// Returns the model family.
    fn kind(
        &self,
    ) -> NoiseModelKind;

    /// Returns a stable model identifier.
    fn name(
        &self,
    ) -> &'static str;

    /// Validates model configuration.
    fn validate(
        &self,
        limits: &QecLimits,
    ) -> QecResult<()>;

    /// Samples a bounded fault batch.
    fn sample(
        &self,
        qubits: &[QubitId],
        operation: NoiseOperation,
        seed: NoiseSeed,
        limits: &QecLimits,
        cancellation: &CancellationToken,
    ) -> QecResult<FaultBatch>;
}

// ============================================================================
// Deterministic RNG
// ============================================================================

/// Small deterministic generator used exclusively by this module.
///
/// It is not intended to be cryptographic randomness.
#[derive(
    Debug,
    Clone,
    Copy,
)]
struct DeterministicRng {
    state: u64,
}

impl DeterministicRng {
    fn new(
        seed: NoiseSeed,
    ) -> Self {
        Self {
            state: splitmix64(
                seed.value(),
            ),
        }
    }

    fn next_u64(
        &mut self,
    ) -> u64 {
        self.state = self
            .state
            .wrapping_add(
                0x9E37_79B9_7F4A_7C15,
            );

        splitmix64(
            self.state,
        )
    }

    fn sample_probability(
        &mut self,
        probability: Probability,
    ) -> bool {
        if probability.is_zero() {
            return false;
        }

        if probability.is_one() {
            return true;
        }

        self.next_u64()
            % PROBABILITY_SCALE
            < probability.scaled()
    }

    fn sample_pauli(
        &mut self,
        x: Probability,
        y: Probability,
        z: Probability,
    ) -> Option<PauliError> {
        let roll =
            self.next_u64()
                % PROBABILITY_SCALE;

        let x_end =
            x.scaled();

        let y_end =
            x_end
                .checked_add(
                    y.scaled(),
                )?;

        let z_end =
            y_end
                .checked_add(
                    z.scaled(),
                )?;

        if roll < x_end {
            Some(PauliError::X)
        } else if roll < y_end {
            Some(PauliError::Y)
        } else if roll < z_end {
            Some(PauliError::Z)
        } else {
            None
        }
    }
}

// ============================================================================
// Standard model
// ============================================================================

/// Generic configurable standard noise model.
///
/// This is the model implementation used when callers need explicit
/// probabilities without introducing a separate model type.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub struct StandardNoiseModel {
    config: NoiseConfig,
}

impl StandardNoiseModel {
    /// Creates a validated model configuration.
    pub fn new(
        config: NoiseConfig,
        limits: &QecLimits,
    ) -> QecResult<Self> {
        config.validate(limits)?;

        Ok(Self {
            config,
        })
    }

    /// Returns the immutable configuration.
    pub fn config(
        &self,
    ) -> &NoiseConfig {
        &self.config
    }

    fn sample_pauli_model(
        &self,
        qubits: &[QubitId],
        operation: NoiseOperation,
        seed: NoiseSeed,
        limits: &QecLimits,
        cancellation: &CancellationToken,
    ) -> QecResult<FaultBatch> {
        let mut batch =
            FaultBatch::with_limits(
                seed.value(),
                qubits.len(),
                limits,
            )?;

        let mut rng =
            DeterministicRng::new(seed);

        for (index, qubit) in
            qubits.iter().copied().enumerate()
        {
            if cancellation.is_cancelled() {
                return Err(
                    NoiseError::Cancelled.into(),
                );
            }

            let pauli =
                match self.config.kind {
                    NoiseModelKind::BitFlip => {
                        if rng.sample_probability(
                            self.config.probability,
                        ) {
                            Some(PauliError::X)
                        } else {
                            None
                        }
                    }

                    NoiseModelKind::PhaseFlip => {
                        if rng.sample_probability(
                            self.config.probability,
                        ) {
                            Some(PauliError::Z)
                        } else {
                            None
                        }
                    }

                    NoiseModelKind::Depolarizing => {
                        let p =
                            self.config.probability;

                        let one_third =
                            p.scaled()
                                / 3;

                        let remainder =
                            p.scaled()
                                % 3;

                        let x =
                            Probability::from_scaled(
                                one_third
                                    .saturating_add(
                                        u64::from(
                                            remainder > 0,
                                        ),
                                    ),
                            )
                            .map_err(QecError::from)?;

                        let y =
                            Probability::from_scaled(
                                one_third
                                    .saturating_add(
                                        u64::from(
                                            remainder > 1,
                                        ),
                                    ),
                            )
                            .map_err(QecError::from)?;

                        let z =
                            Probability::from_scaled(
                                one_third,
                            )
                            .map_err(QecError::from)?;

                        rng.sample_pauli(
                            x,
                            y,
                            z,
                        )
                    }

                    NoiseModelKind::Pauli
                    | NoiseModelKind::HardwareCalibrated => {
                        rng.sample_pauli(
                            self.config.x_probability,
                            self.config.y_probability,
                            self.config.z_probability,
                        )
                    }

                    _ => {
                        return Err(
                            NoiseError::UnsupportedModelOperation(
                                format!(
                                    "model {} does not generate Pauli faults",
                                    self.config.kind.as_str(),
                                ),
                            )
                            .into(),
                        );
                    }
                };

            if let Some(pauli) = pauli {
                let fault =
                    Fault::pauli(
                        operation,
                        qubit,
                        pauli,
                    )
                    .map_err(QecError::from)?;

                batch.push(
                    fault,
                    limits,
                )?;
            }

            // Periodic cancellation boundary for very large workloads.
            if index & 0xFF == 0
                && cancellation.is_cancelled()
            {
                return Err(
                    NoiseError::Cancelled.into(),
                );
            }
        }

        batch.canonicalize();

        Ok(batch)
    }

    fn sample_measurement(
        &self,
        qubits: &[QubitId],
        seed: NoiseSeed,
        limits: &QecLimits,
        cancellation: &CancellationToken,
    ) -> QecResult<FaultBatch> {
        let mut batch =
            FaultBatch::with_limits(
                seed.value(),
                qubits.len(),
                limits,
            )?;

        let mut rng =
            DeterministicRng::new(seed);

        for (index, qubit) in
            qubits.iter().copied().enumerate()
        {
            if cancellation.is_cancelled() {
                return Err(
                    NoiseError::Cancelled.into(),
                );
            }

            if rng.sample_probability(
                self.config.measurement_probability,
            ) {
                batch.push(
                    Fault::measurement(qubit),
                    limits,
                )?;
            }

            if index & 0xFF == 0
                && cancellation.is_cancelled()
            {
                return Err(
                    NoiseError::Cancelled.into(),
                );
            }
        }

        batch.canonicalize();

        Ok(batch)
    }

    fn sample_reset(
        &self,
        qubits: &[QubitId],
        seed: NoiseSeed,
        limits: &QecLimits,
        cancellation: &CancellationToken,
    ) -> QecResult<FaultBatch> {
        let mut batch =
            FaultBatch::with_limits(
                seed.value(),
                qubits.len(),
                limits,
            )?;

        let mut rng =
            DeterministicRng::new(seed);

        for qubit in qubits {
            if cancellation.is_cancelled() {
                return Err(
                    NoiseError::Cancelled.into(),
                );
            }

            if rng.sample_probability(
                self.config.reset_probability,
            ) {
                let pauli =
                    rng.sample_pauli(
                        Probability::from_scaled(
                            self.config.reset_probability
                                .scaled()
                                / 3,
                        )
                        .map_err(QecError::from)?,
                        Probability::from_scaled(
                            self.config.reset_probability
                                .scaled()
                                / 3,
                        )
                        .map_err(QecError::from)?,
                        Probability::from_scaled(
                            self.config.reset_probability
                                .scaled()
                                / 3,
                        )
                        .map_err(QecError::from)?,
                    )
                    .unwrap_or(
                        PauliError::X,
                    );

                batch.push(
                    Fault::reset(
                        *qubit,
                        pauli,
                    )
                    .map_err(QecError::from)?,
                    limits,
                )?;
            }
        }

        batch.canonicalize();

        Ok(batch)
    }

    fn sample_special(
        &self,
        qubits: &[QubitId],
        operation: NoiseOperation,
        seed: NoiseSeed,
        limits: &QecLimits,
        cancellation: &CancellationToken,
    ) -> QecResult<FaultBatch> {
        let mut batch =
            FaultBatch::with_limits(
                seed.value(),
                qubits.len(),
                limits,
            )?;

        let mut rng =
            DeterministicRng::new(seed);

        for qubit in qubits {
            if cancellation.is_cancelled() {
                return Err(
                    NoiseError::Cancelled.into(),
                );
            }

            if rng.sample_probability(
                self.config.leakage_probability,
            ) {
                batch.push(
                    Fault::leakage(
                        operation,
                        *qubit,
                    ),
                    limits,
                )?;
            }

            if rng.sample_probability(
                self.config.erasure_probability,
            ) {
                batch.push(
                    Fault::erasure(
                        operation,
                        *qubit,
                    ),
                    limits,
                )?;
            }
        }

        batch.canonicalize();

        Ok(batch)
    }
}

impl NoiseModel for StandardNoiseModel {
    fn kind(
        &self,
    ) -> NoiseModelKind {
        self.config.kind
    }

    fn name(
        &self,
    ) -> &'static str {
        self.config.kind.as_str()
    }

    fn validate(
        &self,
        limits: &QecLimits,
    ) -> QecResult<()> {
        self.config.validate(limits)
    }

    fn sample(
        &self,
        qubits: &[QubitId],
        operation: NoiseOperation,
        seed: NoiseSeed,
        limits: &QecLimits,
        cancellation: &CancellationToken,
    ) -> QecResult<FaultBatch> {
        self.validate(limits)?;

        if qubits.len()
            > limits.max_qubits
        {
            return Err(
                NoiseError::ResourceLimitExceeded {
                    resource: "qubits",
                    requested: qubits.len() as u128,
                    maximum: limits.max_qubits as u128,
                }
                .into(),
            );
        }

        match operation {
            NoiseOperation::Measurement
                => self.sample_measurement(
                    qubits,
                    seed,
                    limits,
                    cancellation,
                ),

            NoiseOperation::Reset
                => self.sample_reset(
                    qubits,
                    seed,
                    limits,
                    cancellation,
                ),

            NoiseOperation::Qubit
            | NoiseOperation::Gate
            | NoiseOperation::Idle => {
                match self.config.kind {
                    NoiseModelKind::MeasurementError
                    | NoiseModelKind::Readout => {
                        self.sample_measurement(
                            qubits,
                            seed,
                            limits,
                            cancellation,
                        )
                    }

                    NoiseModelKind::Leakage
                    | NoiseModelKind::Erasure => {
                        self.sample_special(
                            qubits,
                            operation,
                            seed,
                            limits,
                            cancellation,
                        )
                    }

                    _ => self.sample_pauli_model(
                        qubits,
                        operation,
                        seed,
                        limits,
                        cancellation,
                    ),
                }
            }
        }
    }
}

// ============================================================================
// Model constructors
// ============================================================================

/// Creates a depolarizing noise model.
pub fn depolarizing(
    probability: Probability,
    limits: &QecLimits,
) -> QecResult<StandardNoiseModel> {
    StandardNoiseModel::new(
        NoiseConfig {
            kind: NoiseModelKind::Depolarizing,
            probability,
            ..NoiseConfig::default()
        },
        limits,
    )
}

/// Creates a bit-flip noise model.
pub fn bit_flip(
    probability: Probability,
    limits: &QecLimits,
) -> QecResult<StandardNoiseModel> {
    StandardNoiseModel::new(
        NoiseConfig {
            kind: NoiseModelKind::BitFlip,
            probability,
            ..NoiseConfig::default()
        },
        limits,
    )
}

/// Creates a phase-flip noise model.
pub fn phase_flip(
    probability: Probability,
    limits: &QecLimits,
) -> QecResult<StandardNoiseModel> {
    StandardNoiseModel::new(
        NoiseConfig {
            kind: NoiseModelKind::PhaseFlip,
            probability,
            ..NoiseConfig::default()
        },
        limits,
    )
}

/// Creates an explicit Pauli noise model.
pub fn pauli(
    x: Probability,
    y: Probability,
    z: Probability,
    limits: &QecLimits,
) -> QecResult<StandardNoiseModel> {
    StandardNoiseModel::new(
        NoiseConfig {
            kind: NoiseModelKind::Pauli,
            x_probability: x,
            y_probability: y,
            z_probability: z,
            probability: PROBABILITY_ZERO,
            ..NoiseConfig::default()
        },
        limits,
    )
}

/// Creates a measurement/readout noise model.
pub fn measurement_error(
    probability: Probability,
    limits: &QecLimits,
) -> QecResult<StandardNoiseModel> {
    StandardNoiseModel::new(
        NoiseConfig {
            kind: NoiseModelKind::MeasurementError,
            measurement_probability: probability,
            probability,
            ..NoiseConfig::default()
        },
        limits,
    )
}

/// Creates a leakage model.
pub fn leakage(
    probability: Probability,
    limits: &QecLimits,
) -> QecResult<StandardNoiseModel> {
    StandardNoiseModel::new(
        NoiseConfig {
            kind: NoiseModelKind::Leakage,
            leakage_probability: probability,
            probability,
            ..NoiseConfig::default()
        },
        limits,
    )
}

/// Creates an erasure model.
pub fn erasure(
    probability: Probability,
    limits: &QecLimits,
) -> QecResult<StandardNoiseModel> {
    StandardNoiseModel::new(
        NoiseConfig {
            kind: NoiseModelKind::Erasure,
            erasure_probability: probability,
            probability,
            ..NoiseConfig::default()
        },
        limits,
    )
}

// ============================================================================
// Deterministic seed derivation
// ============================================================================

/// Derives a stable per-shot seed.
///
/// This function intentionally has no global state.
pub fn derive_shot_seed(
    base_seed: u64,
    shot_index: u64,
) -> NoiseSeed {
    NoiseSeed::new(
        splitmix64(
            base_seed
                .wrapping_add(
                    shot_index.wrapping_mul(
                        0x9E37_79B9_7F4A_7C15,
                    ),
                ),
        ),
    )
}

/// Derives a stable seed for one operation.
pub fn derive_operation_seed(
    shot_seed: NoiseSeed,
    operation: NoiseOperation,
    operation_index: u64,
) -> NoiseSeed {
    let operation_tag =
        match operation {
            NoiseOperation::Qubit => 0x01_u64,
            NoiseOperation::Gate => 0x02_u64,
            NoiseOperation::Measurement => 0x03_u64,
            NoiseOperation::Reset => 0x04_u64,
            NoiseOperation::Idle => 0x05_u64,
        };

    NoiseSeed::new(
        splitmix64(
            shot_seed
                .value()
                ^ operation_tag.rotate_left(11)
                ^ operation_index.rotate_left(23),
        ),
    )
}

// ============================================================================
// Helpers
// ============================================================================

/// Deterministic SplitMix64 mixing function.
fn splitmix64(
    mut value: u64,
) -> u64 {
    value =
        value.wrapping_add(
            0x9E37_79B9_7F4A_7C15,
        );

    let mut result =
        value;

    result =
        (result
            ^ (result >> 30))
            .wrapping_mul(
                0xBF58_476D_1CE4_E5B9,
            );

    result =
        (result
            ^ (result >> 27))
            .wrapping_mul(
                0x94D0_49BB_1331_11EB,
            );

    result
        ^ (result >> 31)
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn limits() -> QecLimits {
        QecLimits::new()
    }

    fn token() -> CancellationToken {
        CancellationToken::new()
    }

    #[test]
    fn probability_rejects_values_above_one() {
        assert!(
            Probability::from_scaled(
                PROBABILITY_SCALE + 1
            )
            .is_err()
        );
    }

    #[test]
    fn probability_percentage_is_exact() {
        let probability =
            Probability::from_percent(
                50,
            )
            .expect("50% must be valid");

        assert_eq!(
            probability.scaled(),
            PROBABILITY_SCALE / 2
        );
    }

    #[test]
    fn probability_complement_is_exact() {
        let probability =
            Probability::from_percent(
                25,
            )
            .expect("25% must be valid");

        assert_eq!(
            probability
                .complement()
                .scaled(),
            750_000_000_000
        );
    }

    #[test]
    fn pauli_multiplication_is_phase_free() {
        assert_eq!(
            PauliError::X.multiply(
                PauliError::X
            ),
            PauliError::I
        );

        assert_eq!(
            PauliError::X.multiply(
                PauliError::Y
            ),
            PauliError::Z
        );
    }

    #[test]
    fn correlated_fault_requires_canonical_qubits() {
        let q0 =
            QubitId::new(0)
                .expect("q0 valid");

        let q1 =
            QubitId::new(1)
                .expect("q1 valid");

        assert!(
            Fault::correlated(
                NoiseOperation::Gate,
                vec![q1, q0],
                vec![
                    PauliError::X,
                    PauliError::Z,
                ],
            )
            .is_err()
        );
    }

    #[test]
    fn deterministic_seeds_repeat() {
        let first =
            derive_shot_seed(
                1234,
                99,
            );

        let second =
            derive_shot_seed(
                1234,
                99,
            );

        assert_eq!(
            first,
            second
        );
    }

    #[test]
    fn different_shots_get_different_seeds() {
        let first =
            derive_shot_seed(
                1234,
                0,
            );

        let second =
            derive_shot_seed(
                1234,
                1,
            );

        assert_ne!(
            first,
            second
        );
    }

    #[test]
    fn depolarizing_model_is_reproducible() {
        let limits =
            limits();

        let model =
            depolarizing(
                Probability::from_percent(
                    10,
                )
                .expect("valid probability"),
                &limits,
            )
            .expect("valid model");

        let qubits: Vec<QubitId> =
            (0..32)
                .map(|index| {
                    QubitId::new(index)
                        .expect("valid qubit")
                })
                .collect();

        let first =
            model
                .sample(
                    &qubits,
                    NoiseOperation::Qubit,
                    NoiseSeed::new(42),
                    &limits,
                    &token(),
                )
                .expect("sampling succeeds");

        let second =
            model
                .sample(
                    &qubits,
                    NoiseOperation::Qubit,
                    NoiseSeed::new(42),
                    &limits,
                    &token(),
                )
                .expect("sampling succeeds");

        assert_eq!(
            first,
            second
        );
    }

    #[test]
    fn zero_probability_produces_no_pauli_faults() {
        let limits =
            limits();

        let model =
            depolarizing(
                PROBABILITY_ZERO,
                &limits,
            )
            .expect("valid model");

        let qubits: Vec<QubitId> =
            (0..32)
                .map(|index| {
                    QubitId::new(index)
                        .expect("valid qubit")
                })
                .collect();

        let batch =
            model
                .sample(
                    &qubits,
                    NoiseOperation::Qubit,
                    NoiseSeed::new(7),
                    &limits,
                    &token(),
                )
                .expect("sampling succeeds");

        assert!(
            batch.is_empty()
        );
    }

    #[test]
    fn one_probability_produces_faults_for_bit_flip() {
        let limits =
            limits();

        let model =
            bit_flip(
                PROBABILITY_ONE,
                &limits,
            )
            .expect("valid model");

        let qubits: Vec<QubitId> =
            (0..16)
                .map(|index| {
                    QubitId::new(index)
                        .expect("valid qubit")
                })
                .collect();

        let batch =
            model
                .sample(
                    &qubits,
                    NoiseOperation::Qubit,
                    NoiseSeed::new(1),
                    &limits,
                    &token(),
                )
                .expect("sampling succeeds");

        assert_eq!(
            batch.len(),
            qubits.len()
        );
    }

    #[test]
    fn cancellation_is_observed() {
        let limits =
            limits();

        let source =
            super::super::cancellation::CancellationSource::new();

        let token =
            source.token();

        source.cancel();

        let model =
            depolarizing(
                Probability::from_percent(
                    50,
                )
                .expect("valid probability"),
                &limits,
            )
            .expect("valid model");

        let qubits: Vec<QubitId> =
            (0..64)
                .map(|index| {
                    QubitId::new(index)
                        .expect("valid qubit")
                })
                .collect();

        let result =
            model.sample(
                &qubits,
                NoiseOperation::Qubit,
                NoiseSeed::new(1),
                &limits,
                &token,
            );

        assert!(
            result.is_err()
        );
    }

    #[test]
    fn fault_batch_preserves_seed() {
        let batch =
            FaultBatch::new(123);

        assert_eq!(
            batch.seed(),
            123
        );
    }

    #[test]
    fn fault_weight_is_correct() {
        let q0 =
            QubitId::new(0)
                .expect("valid qubit");

        let fault =
            Fault::pauli(
                NoiseOperation::Qubit,
                q0,
                PauliError::X,
            )
            .expect("valid fault");

        assert_eq!(
            fault.weight(),
            1
        );
    }
}