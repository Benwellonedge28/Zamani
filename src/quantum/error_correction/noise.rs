//! Zamani Quantum Error Correction — Noise Model
//!
//! This module is the canonical boundary between physical-noise models and
//! the QEC execution pipeline.
//!
//! Responsibilities:
//! - represent validated physical faults;
//! - represent deterministic noise-model configuration;
//! - validate probabilities and physical identifiers;
//! - generate reproducible faults from explicit seeds;
//! - enforce QecLimits before bounded allocations;
//! - preserve deterministic ordering;
//! - distinguish Pauli, measurement, reset, leakage and erasure faults;
//! - support correlated faults;
//! - provide model-independent FaultBatch values;
//! - expose a stable integration boundary for simulation and hardware backends.
//!
//! This module deliberately does NOT:
//! - decode syndromes;
//! - perform correction;
//! - mutate quantum state;
//! - access QPU credentials;
//! - perform network I/O;
//! - emit raw fault streams to telemetry.
//!
//! Production pipeline:
//!
//! ```text
//! Noise configuration
//!        |
//!        v
//! NoiseModel
//!        |
//!        v
//! deterministic seed
//!        |
//!        v
//! resource preflight
//!        |
//!        v
//! validated FaultBatch
//!        |
//!        v
//! syndrome extraction
//!        |
//!        v
//! decoder
//! ```
//!
//! Design requirements:
//! - deterministic;
//! - bounded;
//! - cancellation-aware at expensive sampling boundaries;
//! - no hidden randomness;
//! - no panicking public constructors;
//! - checked arithmetic;
//! - explicit probability representation;
//! - explicit physical fault representation;
//! - centralized QecLimits integration;
//! - stable error conversion into QecError.

use core::fmt;

use super::errors::{
    DecoderKind,
    NumericalOperation,
    QecError,
    QecResult,
    ResourceKind,
};
use super::limits::QecLimits;

// ============================================================================
// Production constants
// ============================================================================

/// Maximum supported physical qubit identifier.
///
/// This is an API-safety boundary, not a hardware-capability statement.
pub const MAX_QUBIT_INDEX: usize = 1_000_000_000;

/// Maximum number of qubits affected by one correlated fault.
pub const MAX_CORRELATED_QUBITS: usize = 1_000;

/// Maximum number of faults accepted by the legacy/default batch constructor.
///
/// Production callers should prefer `FaultBatch::with_limits`.
pub const MAX_FAULTS_PER_BATCH: usize = 1_000_000;

/// Fixed-point probability scale.
///
/// `PROBABILITY_SCALE` represents exactly 100%.
pub const PROBABILITY_SCALE: u64 = 1_000_000_000_000;

/// One hundred percent.
pub const PROBABILITY_ONE: Probability =
    Probability(PROBABILITY_SCALE);

/// Zero percent.
pub const PROBABILITY_ZERO: Probability =
    Probability(0);

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
                NoiseError::InvalidQubitId {
                    id,
                },
            );
        }

        Ok(Self(id))
    }

    /// Returns the physical qubit index.
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
/// Fixed-point representation is used so configuration comparison,
/// ordering and hashing do not depend on floating-point behavior.
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
    /// Creates a probability from the fixed-point representation.
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

        let scaled =
            u64::from(percent)
                .checked_mul(
                    PROBABILITY_SCALE / 100,
                )
                .ok_or(
                    NoiseError::ArithmeticOverflow,
                )?;

        Self::from_scaled(
            scaled,
        )
    }

    /// Creates a probability from basis points.
    ///
    /// 10,000 basis points = 100%.
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

        let scaled =
            u64::from(basis_points)
                .checked_mul(
                    PROBABILITY_SCALE / 10_000,
                )
                .ok_or(
                    NoiseError::ArithmeticOverflow,
                )?;

        Self::from_scaled(
            scaled,
        )
    }

    /// Returns the fixed-point representation.
    pub const fn scaled(
        self,
    ) -> u64 {
        self.0
    }

    /// Converts to floating point at the API/presentation boundary.
    pub fn as_f64(
        self,
    ) -> f64 {
        self.0 as f64
            / PROBABILITY_SCALE as f64
    }

    /// Returns true if the probability is zero.
    pub const fn is_zero(
        self,
    ) -> bool {
        self.0 == 0
    }

    /// Returns true if the probability is one.
    pub const fn is_one(
        self,
    ) -> bool {
        self.0 == PROBABILITY_SCALE
    }

    /// Returns the complement.
    pub fn complement(
        self,
    ) -> Self {
        Self(
            PROBABILITY_SCALE
                - self.0,
        )
    }
}

impl Default for Probability {
    fn default() -> Self {
        PROBABILITY_ZERO
    }
}

// ============================================================================
// Pauli error
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
    /// Identity.
    I,

    /// Bit flip.
    X,

    /// Bit + phase flip.
    Y,

    /// Phase flip.
    Z,
}

impl PauliError {
    /// Returns true when this is a physical error.
    pub const fn is_non_identity(
        self,
    ) -> bool {
        !matches!(
            self,
            Self::I
        )
    }

    /// Returns true when this is identity.
    pub const fn is_identity(
        self,
    ) -> bool {
        matches!(
            self,
            Self::I
        )
    }

    /// Pauli multiplication with global phase discarded.
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

    /// Returns true when two Paulis commute.
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

    /// Returns true when two Paulis anticommute.
    pub const fn anticommutes(
        self,
        rhs: Self,
    ) -> bool {
        !self.commutes(
            rhs,
        )
    }

    /// Returns a stable integer encoding.
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

/// Physical operation associated with a fault.
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
    /// Error on stored/data state.
    Qubit,

    /// Error associated with a gate.
    Gate,

    /// Measurement/readout error.
    Measurement,

    /// State preparation/reset error.
    Reset,

    /// Error accumulated while idle.
    Idle,
}

impl NoiseOperation {
    /// Returns whether this operation can carry a Pauli fault.
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

/// Physical fault classification.
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
    /// Single-qubit Pauli fault.
    Pauli,

    /// Measurement/readout fault.
    Measurement,

    /// Reset/preparation fault.
    Reset,

    /// Correlated multi-qubit Pauli fault.
    Correlated,

    /// Leakage event.
    Leakage,

    /// Erasure event.
    Erasure,
}

impl FaultKind {
    /// Stable machine-readable identifier.
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

/// Validated immutable physical fault.
///
/// The representation is intentionally explicit. Downstream syndrome and
/// decoder layers do not need to know how the fault was generated.
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

    /// Measurement/readout fault.
    Measurement {
        qubit: QubitId,
    },

    /// Reset/preparation fault.
    Reset {
        qubit: QubitId,
        pauli: PauliError,
    },

    /// Correlated multi-qubit Pauli fault.
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
                        "measurement faults require Fault::measurement"
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
    pub fn measurement(
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

    /// Creates a correlated Pauli fault.
    ///
    /// Qubits must be strictly increasing. This gives the representation a
    /// canonical ordering and makes deterministic hashing/replay possible.
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
                        "measurement corruption requires Fault::measurement"
                            .to_owned(),
                },
            );
        }

        if qubits.is_empty() {
            return Err(
                NoiseError::EmptyCorrelatedFault,
            );
        }

        if qubits.len()
            != paulis.len()
        {
            return Err(
                NoiseError::MismatchedCorrelatedLengths {
                    qubits: qubits.len(),
                    paulis: paulis.len(),
                },
            );
        }

        if qubits.len()
            > MAX_CORRELATED_QUBITS
        {
            return Err(
                NoiseError::CorrelatedFaultTooLarge {
                    requested: qubits.len(),
                    maximum:
                        MAX_CORRELATED_QUBITS,
                },
            );
        }

        for index in 0..paulis.len() {
            if paulis[index]
                .is_identity()
            {
                return Err(
                    NoiseError::IdentityFault,
                );
            }

            if index > 0
                && qubits[index - 1]
                    >= qubits[index]
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

    /// Creates a leakage event.
    pub fn leakage(
        operation: NoiseOperation,
        qubit: QubitId,
    ) -> Result<Self, NoiseError> {
        if matches!(
            operation,
            NoiseOperation::Measurement
        ) {
            return Err(
                NoiseError::InvalidOperation {
                    operation,
                    message:
                        "measurement leakage must use a backend-specific measurement model"
                            .to_owned(),
                },
            );
        }

        Ok(Self::Leakage {
            operation,
            qubit,
        })
    }

    /// Creates an erasure event.
    pub fn erasure(
        operation: NoiseOperation,
        qubit: QubitId,
    ) -> Result<Self, NoiseError> {
        if matches!(
            operation,
            NoiseOperation::Measurement
        ) {
            return Err(
                NoiseError::InvalidOperation {
                    operation,
                    message:
                        "measurement erasure requires a measurement/readout model"
                            .to_owned(),
                },
            );
        }

        Ok(Self::Erasure {
            operation,
            qubit,
        })
    }

    /// Returns the fault kind.
    pub const fn kind(
        &self,
    ) -> FaultKind {
        match self {
            Self::Pauli { .. } => FaultKind::Pauli,
            Self::Measurement { .. } =>
                FaultKind::Measurement,
            Self::Reset { .. } =>
                FaultKind::Reset,
            Self::Correlated { .. } =>
                FaultKind::Correlated,
            Self::Leakage { .. } =>
                FaultKind::Leakage,
            Self::Erasure { .. } =>
                FaultKind::Erasure,
        }
    }

    /// Returns the operation metadata.
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

            Self::Measurement { .. } =>
                NoiseOperation::Measurement,

            Self::Reset { .. } =>
                NoiseOperation::Reset,
        }
    }

    /// Returns the number of affected qubits.
    pub fn qubit_count(
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

    /// Returns true for a multi-qubit correlated fault.
    pub fn is_correlated(
        &self,
    ) -> bool {
        matches!(
            self,
            Self::Correlated { .. }
        )
    }

    /// Returns the first affected qubit.
    pub fn first_qubit(
        &self,
    ) -> QubitId {
        match self {
            Self::Pauli {
                qubit,
                ..
            }
            | Self::Measurement {
                qubit,
            }
            | Self::Reset {
                qubit,
                ..
            }
            | Self::Leakage {
                qubit,
                ..
            }
            | Self::Erasure {
                qubit,
                ..
            } => *qubit,

            Self::Correlated {
                qubits,
                ..
            } => qubits[0],
        }
    }

    /// Returns all affected qubits.
    ///
    /// The returned slice is borrowed and does not allocate.
    pub fn qubits(
        &self,
    ) -> FaultQubits<'_> {
        match self {
            Self::Pauli {
                qubit,
                ..
            }
            | Self::Measurement {
                qubit,
            }
            | Self::Reset {
                qubit,
                ..
            }
            | Self::Leakage {
                qubit,
                ..
            }
            | Self::Erasure {
                qubit,
                ..
            } => FaultQubits::One(
                qubit,
            ),

            Self::Correlated {
                qubits,
                ..
            } => FaultQubits::Many(
                qubits.as_slice(),
            ),
        }
    }

    /// Returns the Pauli for a single-qubit Pauli/reset fault.
    pub fn pauli(
        &self,
    ) -> Option<PauliError> {
        match self {
            Self::Pauli {
                pauli,
                ..
            }
            | Self::Reset {
                pauli,
                ..
            } => Some(*pauli),

            _ => None,
        }
    }
}

/// Borrowed fault-qubit view.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
)]
pub enum FaultQubits<'a> {
    /// One affected qubit.
    One(&'a QubitId),

    /// Multiple affected qubits.
    Many(&'a [QubitId]),
}

impl<'a> FaultQubits<'a> {
    /// Returns the number of qubits.
    pub fn len(
        self,
    ) -> usize {
        match self {
            Self::One(_) => 1,
            Self::Many(values) =>
                values.len(),
        }
    }
}

// ============================================================================
// Fault batch
// ============================================================================

/// Validated ordered batch of physical faults.
///
/// Ordering is preserved deliberately. Deterministic execution may later
/// canonicalize a copy, but the original observation order is never silently
/// destroyed.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub struct FaultBatch {
    faults: Vec<Fault>,
}

impl FaultBatch {
    /// Creates an empty fault batch.
    #[must_use]
    pub fn new() -> Self {
        Self {
            faults: Vec::new(),
        }
    }

    /// Creates an empty batch with a caller-supplied capacity.
    pub fn with_capacity(
        capacity: usize,
    ) -> Result<Self, NoiseError> {
        if capacity > MAX_FAULTS_PER_BATCH {
            return Err(
                NoiseError::FaultBatchTooLarge {
                    requested: capacity,
                    maximum:
                        MAX_FAULTS_PER_BATCH,
                },
            );
        }

        Ok(Self {
            faults: Vec::with_capacity(
                capacity,
            ),
        })
    }

    /// Creates a batch using the centralized QEC resource policy.
    ///
    /// The limit is checked before the vector is allocated.
    pub fn with_limits(
        faults: Vec<Fault>,
        limits: &QecLimits,
    ) -> QecResult<Self> {
        limits
            .validate()
            .map_err(|error| {
                QecError::ResourceLimitExceeded {
                    resource:
                        ResourceKind::SyndromeEvents,
                    requested: 0,
                    limit:
                        limits.max_syndrome_events
                            as u128,
                    message:
                        error.to_string(),
                }
            })?;

        limits
            .check_syndrome_events(
                faults.len(),
            )
            .map_err(|error| {
                QecError::ResourceLimitExceeded {
                    resource:
                        ResourceKind::SyndromeEvents,
                    requested:
                        faults.len() as u128,
                    limit:
                        limits.max_syndrome_events
                            as u128,
                    message:
                        error.to_string(),
                }
            })?;

        let memory =
            estimate_fault_batch_memory(
                faults.len(),
            )
            .map_err(
                NoiseError::from,
            )?;

        limits
            .check_memory_bytes(
                memory,
            )
            .map_err(|error| {
                QecError::MemoryLimitExceeded {
                    requested_bytes:
                        memory,
                    limit_bytes:
                        limits.max_memory_bytes,
                    message:
                        error.to_string(),
                }
            })?;

        Self::validate_faults(
            &faults,
        )
        .map_err(
            QecError::from,
        )?;

        Ok(Self {
            faults,
        })
    }

    /// Creates a batch using the legacy hard batch boundary.
    pub fn from_faults(
        faults: Vec<Fault>,
    ) -> Result<Self, NoiseError> {
        if faults.len()
            > MAX_FAULTS_PER_BATCH
        {
            return Err(
                NoiseError::FaultBatchTooLarge {
                    requested: faults.len(),
                    maximum:
                        MAX_FAULTS_PER_BATCH,
                },
            );
        }

        Self::validate_faults(
            &faults,
        )?;

        Ok(Self {
            faults,
        })
    }

    /// Adds one validated fault.
    pub fn push(
        &mut self,
        fault: Fault,
    ) -> Result<(), NoiseError> {
        if self.faults.len()
            >= MAX_FAULTS_PER_BATCH
        {
            return Err(
                NoiseError::FaultBatchTooLarge {
                    requested:
                        self.faults.len()
                            .saturating_add(1),
                    maximum:
                        MAX_FAULTS_PER_BATCH,
                },
            );
        }

        self.faults.push(
            fault,
        );

        Ok(())
    }

    /// Adds a fault under a centralized QEC resource policy.
    pub fn push_with_limits(
        &mut self,
        fault: Fault,
        limits: &QecLimits,
    ) -> QecResult<()> {
        let next_len =
            self.faults
                .len()
                .checked_add(1)
                .ok_or(
                    QecError::ResourceLimitExceeded {
                        resource:
                            ResourceKind::SyndromeEvents,
                        requested:
                            usize::MAX as u128,
                        limit:
                            limits.max_syndrome_events
                                as u128,
                        message:
                            "fault-batch length overflow"
                                .to_owned(),
                    },
                )?;

        limits
            .check_syndrome_events(
                next_len,
            )
            .map_err(|error| {
                QecError::ResourceLimitExceeded {
                    resource:
                        ResourceKind::SyndromeEvents,
                    requested:
                        next_len as u128,
                    limit:
                        limits.max_syndrome_events
                            as u128,
                    message:
                        error.to_string(),
                }
            })?;

        Self::validate_fault(
            &fault,
        )
        .map_err(
            QecError::from,
        )?;

        self.faults.push(
            fault,
        );

        Ok(())
    }

    /// Number of faults.
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

    /// Returns the borrowed ordered fault slice.
    pub fn as_slice(
        &self,
    ) -> &[Fault] {
        self.faults.as_slice()
    }

    /// Returns an iterator over the faults.
    pub fn iter(
        &self,
    ) -> core::slice::Iter<'_, Fault> {
        self.faults.iter()
    }

    /// Returns the number of physical qubit incidences represented.
    pub fn total_qubit_incidences(
        &self,
    ) -> usize {
        self.faults
            .iter()
            .map(Fault::qubit_count)
            .sum()
    }

    /// Validates the entire batch.
    pub fn validate(
        &self,
    ) -> Result<(), NoiseError> {
        Self::validate_faults(
            &self.faults,
        )
    }

    fn validate_faults(
        faults: &[Fault],
    ) -> Result<(), NoiseError> {
        if faults.len()
            > MAX_FAULTS_PER_BATCH
        {
            return Err(
                NoiseError::FaultBatchTooLarge {
                    requested: faults.len(),
                    maximum:
                        MAX_FAULTS_PER_BATCH,
                },
            );
        }

        for fault in faults {
            Self::validate_fault(
                fault,
            )?;
        }

        Ok(())
    }

    fn validate_fault(
        fault: &Fault,
    ) -> Result<(), NoiseError> {
        match fault {
            Fault::Pauli {
                operation,
                qubit: _,
                pauli,
            } => {
                if !operation.supports_pauli()
                {
                    return Err(
                        NoiseError::InvalidOperation {
                            operation: *operation,
                            message:
                                "Pauli faults are not valid for measurement operations"
                                    .to_owned(),
                        },
                    );
                }

                if pauli.is_identity()
                {
                    return Err(
                        NoiseError::IdentityFault,
                    );
                }
            }

            Fault::Measurement {
                ..
            } => {}

            Fault::Reset {
                pauli,
                ..
            } => {
                if pauli.is_identity()
                {
                    return Err(
                        NoiseError::IdentityFault,
                    );
                }
            }

            Fault::Correlated {
                operation,
                qubits,
                paulis,
            } => {
                if !operation.supports_pauli()
                {
                    return Err(
                        NoiseError::InvalidOperation {
                            operation: *operation,
                            message:
                                "correlated measurement faults are unsupported"
                                    .to_owned(),
                        },
                    );
                }

                if qubits.is_empty()
                {
                    return Err(
                        NoiseError::EmptyCorrelatedFault,
                    );
                }

                if qubits.len()
                    != paulis.len()
                {
                    return Err(
                        NoiseError::MismatchedCorrelatedLengths {
                            qubits:
                                qubits.len(),
                            paulis:
                                paulis.len(),
                        },
                    );
                }

                if qubits.len()
                    > MAX_CORRELATED_QUBITS
                {
                    return Err(
                        NoiseError::CorrelatedFaultTooLarge {
                            requested:
                                qubits.len(),
                            maximum:
                                MAX_CORRELATED_QUBITS,
                        },
                    );
                }

                for index in 0..paulis.len() {
                    if paulis[index]
                        .is_identity()
                    {
                        return Err(
                            NoiseError::IdentityFault,
                        );
                    }

                    if index > 0
                        && qubits[index - 1]
                            >= qubits[index]
                    {
                        return Err(
                            NoiseError::NonCanonicalCorrelatedQubits,
                        );
                    }
                }
            }

            Fault::Leakage {
                operation,
                ..
            }
            | Fault::Erasure {
                operation,
                ..
            } => {
                if matches!(
                    operation,
                    NoiseOperation::Measurement
                ) {
                    return Err(
                        NoiseError::InvalidOperation {
                            operation: *operation,
                            message:
                                "measurement corruption requires an explicit measurement model"
                                    .to_owned(),
                        },
                    );
                }
            }
        }

        Ok(())
    }
}

impl Default for FaultBatch {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> IntoIterator
    for &'a FaultBatch
{
    type Item = &'a Fault;
    type IntoIter =
        core::slice::Iter<'a, Fault>;

    fn into_iter(
        self,
    ) -> Self::IntoIter {
        self.faults.iter()
    }
}

// ============================================================================
// Noise model abstraction
// ============================================================================

/// Context supplied to a noise model.
///
/// A model never owns the RNG. The seed is explicit so two executions with
/// the same inputs are reproducible.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
)]
pub struct NoiseContext {
    /// Number of physical qubits available.
    pub qubits: usize,

    /// Measurement/gate round.
    pub round: usize,

    /// Reproducibility seed.
    pub seed: u64,
}

impl NoiseContext {
    /// Creates a validated noise context.
    pub fn new(
        qubits: usize,
        round: usize,
        seed: u64,
        limits: &QecLimits,
    ) -> QecResult<Self> {
        limits
            .check_qubits(
                qubits,
            )
            .map_err(|error| {
                QecError::ResourceLimitExceeded {
                    resource:
                        ResourceKind::Qubits,
                    requested:
                        qubits as u128,
                    limit:
                        limits.max_qubits
                            as u128,
                    message:
                        error.to_string(),
                }
            })?;

        limits
            .check_rounds(
                round.saturating_add(1),
            )
            .map_err(|error| {
                QecError::ResourceLimitExceeded {
                    resource:
                        ResourceKind::MeasurementRounds,
                    requested:
                        round.saturating_add(1)
                            as u128,
                    limit:
                        limits.max_rounds
                            as u128,
                    message:
                        error.to_string(),
                }
            })?;

        Ok(Self {
            qubits,
            round,
            seed,
        })
    }
}

/// Deterministic physical noise model.
///
/// Implementations must not use global randomness.
pub trait NoiseModel:
    Send + Sync
{
    /// Stable model name.
    fn name(
        &self,
    ) -> &'static str;

    /// Validates the model configuration.
    fn validate(
        &self,
    ) -> Result<(), NoiseError>;

    /// Samples faults deterministically from the supplied context.
    fn sample(
        &self,
        context: NoiseContext,
        limits: &QecLimits,
    ) -> QecResult<FaultBatch>;
}

// ============================================================================
// Deterministic RNG
// ============================================================================

/// Small deterministic PRNG used only at the simulation/model boundary.
///
/// This is deliberately not a cryptographic RNG. It is intended for
/// reproducible physical-noise simulation and threshold experiments.
///
/// Security-sensitive key material must never use this type.
#[derive(
    Debug,
    Clone,
    Copy,
)]
pub struct DeterministicRng {
    state: u64,
}

impl DeterministicRng {
    /// Creates a deterministic generator.
    pub const fn new(
        seed: u64,
    ) -> Self {
        Self {
            state: seed,
        }
    }

    /// Generates the next 64-bit value.
    pub fn next_u64(
        &mut self,
    ) -> u64 {
        // SplitMix64.
        self.state =
            self.state
                .wrapping_add(
                    0x9E37_79B9_7F4A_7C15,
                );

        let mut z =
            self.state;

        z = (z
            ^ (z >> 30))
            .wrapping_mul(
                0xBF58_476D_1CE4_E5B9,
            );

        z = (z
            ^ (z >> 27))
            .wrapping_mul(
                0x94D0_49BB_1331_11EB,
            );

        z ^ (z >> 31)
    }

    /// Generates a uniform fixed-point sample in `[0, PROBABILITY_SCALE)`.
    pub fn next_probability_sample(
        &mut self,
    ) -> u64 {
        // Rejection-free modulo sampling is deterministic, but slightly
        // biased. For physical threshold simulation that bias is undesirable.
        // Use rejection sampling instead.
        let bound =
            u64::MAX
                - (u64::MAX
                    % PROBABILITY_SCALE);

        loop {
            let value =
                self.next_u64();

            if value < bound {
                return value
                    % PROBABILITY_SCALE;
            }
        }
    }

    /// Returns true with the supplied probability.
    pub fn bernoulli(
        &mut self,
        probability: Probability,
    ) -> bool {
        self.next_probability_sample()
            < probability.scaled()
    }
}

// ============================================================================
// Pauli noise channel
// ============================================================================

/// Single-qubit Pauli channel.
///
/// ```text
/// P(X) = p_x
/// P(Y) = p_y
/// P(Z) = p_z
/// P(I) = 1 - p_x - p_y - p_z
/// ```
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub struct PauliNoiseChannel {
    p_x: Probability,
    p_y: Probability,
    p_z: Probability,
}

impl PauliNoiseChannel {
    /// Creates a validated Pauli channel.
    pub fn new(
        p_x: Probability,
        p_y: Probability,
        p_z: Probability,
    ) -> Result<Self, NoiseError> {
        let total =
            p_x.scaled()
                .checked_add(
                    p_y.scaled(),
                )
                .and_then(
                    |value| {
                        value.checked_add(
                            p_z.scaled(),
                        )
                    },
                )
                .ok_or(
                    NoiseError::ArithmeticOverflow,
                )?;

        if total
            > PROBABILITY_SCALE
        {
            return Err(
                NoiseError::ProbabilitySumExceedsOne {
                    p_x,
                    p_y,
                    p_z,
                },
            );
        }

        Ok(Self {
            p_x,
            p_y,
            p_z,
        })
    }

    /// Creates a depolarizing channel.
    pub fn depolarizing(
        probability: Probability,
    ) -> Result<Self, NoiseError> {
        let total =
            probability.scaled();

        let base =
            total / 3;

        let remainder =
            total % 3;

        let p_x =
            base
                .checked_add(
                    u64::from(
                        remainder >= 1,
                    ),
                )
                .ok_or(
                    NoiseError::ArithmeticOverflow,
                )?;

        let p_y =
            base
                .checked_add(
                    u64::from(
                        remainder >= 2,
                    ),
                )
                .ok_or(
                    NoiseError::ArithmeticOverflow,
                )?;

        Self::new(
            Probability::from_scaled(
                p_x,
            )?,
            Probability::from_scaled(
                p_y,
            )?,
            Probability::from_scaled(
                base,
            )?,
        )
    }

    /// X probability.
    pub const fn p_x(
        self,
    ) -> Probability {
        self.p_x
    }

    /// Y probability.
    pub const fn p_y(
        self,
    ) -> Probability {
        self.p_y
    }

    /// Z probability.
    pub const fn p_z(
        self,
    ) -> Probability {
        self.p_z
    }

    /// Total non-identity probability.
    pub fn total_error_probability(
        self,
    ) -> Probability {
        let total =
            self.p_x
                .scaled()
                .saturating_add(
                    self.p_y.scaled(),
                )
                .saturating_add(
                    self.p_z.scaled(),
                );

        Probability(total)
    }

    /// No-error probability.
    pub fn no_error_probability(
        self,
    ) -> Probability {
        self.total_error_probability()
            .complement()
    }

    /// Probability of one Pauli class.
    pub fn probability_of(
        self,
        pauli: PauliError,
    ) -> Probability {
        match pauli {
            PauliError::I =>
                self.no_error_probability(),
            PauliError::X => self.p_x,
            PauliError::Y => self.p_y,
            PauliError::Z => self.p_z,
        }
    }

    /// Samples one Pauli deterministically.
    pub fn sample(
        self,
        rng: &mut DeterministicRng,
    ) -> PauliError {
        let value =
            rng.next_probability_sample();

        let x =
            self.p_x.scaled();

        let y =
            x.saturating_add(
                self.p_y.scaled(),
            );

        let z =
            y.saturating_add(
                self.p_z.scaled(),
            );

        if value < x {
            PauliError::X
        } else if value < y {
            PauliError::Y
        } else if value < z {
            PauliError::Z
        } else {
            PauliError::I
        }
    }
}

impl Default for PauliNoiseChannel {
    fn default() -> Self {
        Self {
            p_x: PROBABILITY_ZERO,
            p_y: PROBABILITY_ZERO,
            p_z: PROBABILITY_ZERO,
        }
    }
}

// ============================================================================
// Standard physical models
// ============================================================================

/// Independent depolarizing noise.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
)]
pub struct DepolarizingNoise {
    channel: PauliNoiseChannel,
    operation: NoiseOperation,
}

impl DepolarizingNoise {
    /// Creates a depolarizing model.
    pub fn new(
        probability: Probability,
        operation: NoiseOperation,
    ) -> Result<Self, NoiseError> {
        if !operation.supports_pauli() {
            return Err(
                NoiseError::InvalidOperation {
                    operation,
                    message:
                        "depolarizing noise cannot directly model measurement faults"
                            .to_owned(),
                },
            );
        }

        Ok(Self {
            channel:
                PauliNoiseChannel::depolarizing(
                    probability,
                )?,
            operation,
        })
    }

    /// Returns the channel.
    pub const fn channel(
        self,
    ) -> PauliNoiseChannel {
        self.channel
    }
}

impl NoiseModel
    for DepolarizingNoise
{
    fn name(
        &self,
    ) -> &'static str {
        "depolarizing"
    }

    fn validate(
        &self,
    ) -> Result<(), NoiseError> {
        PauliNoiseChannel::new(
            self.channel.p_x(),
            self.channel.p_y(),
            self.channel.p_z(),
        )
        .map(|_| ())
    }

    fn sample(
        &self,
        context: NoiseContext,
        limits: &QecLimits,
    ) -> QecResult<FaultBatch> {
        self.validate()
            .map_err(
                QecError::from,
            )?;

        let mut rng =
            DeterministicRng::new(
                context.seed,
            );

        let mut faults =
            Vec::new();

        for qubit_index
            in 0..context.qubits
        {
            if rng.bernoulli(
                self.channel
                    .total_error_probability(),
            ) {
                let pauli =
                    self.channel
                        .sample(
                            &mut rng,
                        );

                if pauli.is_non_identity() {
                    faults.push(
                        Fault::pauli(
                            self.operation,
                            QubitId::new(
                                qubit_index,
                            )
                            .map_err(
                                QecError::from,
                            )?,
                            pauli,
                        )
                        .map_err(
                            QecError::from,
                        )?,
                    );
                }
            }

            if faults.len()
                >= limits.max_syndrome_events
            {
                return Err(
                    QecError::ResourceLimitExceeded {
                        resource:
                            ResourceKind::SyndromeEvents,
                        requested:
                            faults.len() as u128,
                        limit:
                            limits.max_syndrome_events
                                as u128,
                        message:
                            "noise sampling exceeded the configured fault/event budget"
                                .to_owned(),
                    },
                );
            }
        }

        FaultBatch::with_limits(
            faults,
            limits,
        )
    }
}

/// Independent X-only noise.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
)]
pub struct BitFlipNoise {
    probability: Probability,
    operation: NoiseOperation,
}

impl BitFlipNoise {
    /// Creates an X-error model.
    pub fn new(
        probability: Probability,
        operation: NoiseOperation,
    ) -> Result<Self, NoiseError> {
        if !operation.supports_pauli() {
            return Err(
                NoiseError::InvalidOperation {
                    operation,
                    message:
                        "bit-flip noise cannot directly model measurement faults"
                            .to_owned(),
                },
            );
        }

        Ok(Self {
            probability,
            operation,
        })
    }
}

impl NoiseModel
    for BitFlipNoise
{
    fn name(
        &self,
    ) -> &'static str {
        "bit_flip"
    }

    fn validate(
        &self,
    ) -> Result<(), NoiseError> {
        Probability::from_scaled(
            self.probability.scaled(),
        )
        .map(|_| ())
    }

    fn sample(
        &self,
        context: NoiseContext,
        limits: &QecLimits,
    ) -> QecResult<FaultBatch> {
        self.validate()
            .map_err(
                QecError::from,
            )?;

        let mut rng =
            DeterministicRng::new(
                context.seed,
            );

        let mut faults =
            Vec::new();

        for index
            in 0..context.qubits
        {
            if rng.bernoulli(
                self.probability,
            ) {
                faults.push(
                    Fault::pauli(
                        self.operation,
                        QubitId::new(
                            index,
                        )
                        .map_err(
                            QecError::from,
                        )?,
                        PauliError::X,
                    )
                    .map_err(
                        QecError::from,
                    )?,
                );
            }

            if faults.len()
                > limits.max_syndrome_events
            {
                return Err(
                    QecError::ResourceLimitExceeded {
                        resource:
                            ResourceKind::SyndromeEvents,
                        requested:
                            faults.len() as u128,
                        limit:
                            limits.max_syndrome_events
                                as u128,
                        message:
                            "bit-flip sampling exceeded the configured fault budget"
                                .to_owned(),
                    },
                );
            }
        }

        FaultBatch::with_limits(
            faults,
            limits,
        )
    }
}

/// Independent Z-only noise.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
)]
pub struct PhaseFlipNoise {
    probability: Probability,
    operation: NoiseOperation,
}

impl PhaseFlipNoise {
    /// Creates a Z-error model.
    pub fn new(
        probability: Probability,
        operation: NoiseOperation,
    ) -> Result<Self, NoiseError> {
        if !operation.supports_pauli() {
            return Err(
                NoiseError::InvalidOperation {
                    operation,
                    message:
                        "phase-flip noise cannot directly model measurement faults"
                            .to_owned(),
                },
            );
        }

        Ok(Self {
            probability,
            operation,
        })
    }
}

impl NoiseModel
    for PhaseFlipNoise
{
    fn name(
        &self,
    ) -> &'static str {
        "phase_flip"
    }

    fn validate(
        &self,
    ) -> Result<(), NoiseError> {
        Probability::from_scaled(
            self.probability.scaled(),
        )
        .map(|_| ())
    }

    fn sample(
        &self,
        context: NoiseContext,
        limits: &QecLimits,
    ) -> QecResult<FaultBatch> {
        self.validate()
            .map_err(
                QecError::from,
            )?;

        let mut rng =
            DeterministicRng::new(
                context.seed,
            );

        let mut faults =
            Vec::new();

        for index
            in 0..context.qubits
        {
            if rng.bernoulli(
                self.probability,
            ) {
                faults.push(
                    Fault::pauli(
                        self.operation,
                        QubitId::new(
                            index,
                        )
                        .map_err(
                            QecError::from,
                        )?,
                        PauliError::Z,
                    )
                    .map_err(
                        QecError::from,
                    )?,
                );
            }

            if faults.len()
                > limits.max_syndrome_events
            {
                return Err(
                    QecError::ResourceLimitExceeded {
                        resource:
                            ResourceKind::SyndromeEvents,
                        requested:
                            faults.len() as u128,
                        limit:
                            limits.max_syndrome_events
                                as u128,
                        message:
                            "phase-flip sampling exceeded the configured fault budget"
                                .to_owned(),
                    },
                );
            }
        }

        FaultBatch::with_limits(
            faults,
            limits,
        )
    }
}

/// Independent measurement-error model.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
)]
pub struct MeasurementNoise {
    probability: Probability,
}

impl MeasurementNoise {
    /// Creates a measurement-noise model.
    pub fn new(
        probability: Probability,
    ) -> Self {
        Self {
            probability,
        }
    }

    /// Returns the configured probability.
    pub const fn probability(
        self,
    ) -> Probability {
        self.probability
    }
}

impl NoiseModel
    for MeasurementNoise
{
    fn name(
        &self,
    ) -> &'static str {
        "measurement_error"
    }

    fn validate(
        &self,
    ) -> Result<(), NoiseError> {
        Probability::from_scaled(
            self.probability.scaled(),
        )
        .map(|_| ())
    }

    fn sample(
        &self,
        context: NoiseContext,
        limits: &QecLimits,
    ) -> QecResult<FaultBatch> {
        self.validate()
            .map_err(
                QecError::from,
            )?;

        let mut rng =
            DeterministicRng::new(
                context.seed,
            );

        let mut faults =
            Vec::new();

        for index
            in 0..context.qubits
        {
            if rng.bernoulli(
                self.probability,
            ) {
                faults.push(
                    Fault::measurement(
                        QubitId::new(
                            index,
                        )
                        .map_err(
                            QecError::from,
                        )?,
                    ),
                );
            }

            if faults.len()
                > limits.max_syndrome_events
            {
                return Err(
                    QecError::ResourceLimitExceeded {
                        resource:
                            ResourceKind::SyndromeEvents,
                        requested:
                            faults.len() as u128,
                        limit:
                            limits.max_syndrome_events
                                as u128,
                        message:
                            "measurement-noise sampling exceeded the configured fault budget"
                                .to_owned(),
                    },
                );
            }
        }

        FaultBatch::with_limits(
            faults,
            limits,
        )
    }
}

/// Independent leakage model.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
)]
pub struct LeakageNoise {
    probability: Probability,
    operation: NoiseOperation,
}

impl LeakageNoise {
    /// Creates a leakage model.
    pub fn new(
        probability: Probability,
        operation: NoiseOperation,
    ) -> Result<Self, NoiseError> {
        if matches!(
            operation,
            NoiseOperation::Measurement
        ) {
            return Err(
                NoiseError::InvalidOperation {
                    operation,
                    message:
                        "leakage must be attached to a physical state/gate/idle operation"
                            .to_owned(),
                },
            );
        }

        Ok(Self {
            probability,
            operation,
        })
    }
}

impl NoiseModel
    for LeakageNoise
{
    fn name(
        &self,
    ) -> &'static str {
        "leakage"
    }

    fn validate(
        &self,
    ) -> Result<(), NoiseError> {
        Probability::from_scaled(
            self.probability.scaled(),
        )
        .map(|_| ())
    }

    fn sample(
        &self,
        context: NoiseContext,
        limits: &QecLimits,
    ) -> QecResult<FaultBatch> {
        self.validate()
            .map_err(
                QecError::from,
            )?;

        let mut rng =
            DeterministicRng::new(
                context.seed,
            );

        let mut faults =
            Vec::new();

        for index
            in 0..context.qubits
        {
            if rng.bernoulli(
                self.probability,
            ) {
                faults.push(
                    Fault::leakage(
                        self.operation,
                        QubitId::new(
                            index,
                        )
                        .map_err(
                            QecError::from,
                        )?,
                    )
                    .map_err(
                        QecError::from,
                    )?,
                );
            }

            if faults.len()
                > limits.max_syndrome_events
            {
                return Err(
                    QecError::ResourceLimitExceeded {
                        resource:
                            ResourceKind::SyndromeEvents,
                        requested:
                            faults.len() as u128,
                        limit:
                            limits.max_syndrome_events
                                as u128,
                        message:
                            "leakage sampling exceeded the configured fault budget"
                                .to_owned(),
                    },
                );
            }
        }

        FaultBatch::with_limits(
            faults,
            limits,
        )
    }
}

/// Independent erasure model.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
)]
pub struct ErasureNoise {
    probability: Probability,
    operation: NoiseOperation,
}

impl ErasureNoise {
    /// Creates an erasure model.
    pub fn new(
        probability: Probability,
        operation: NoiseOperation,
    ) -> Result<Self, NoiseError> {
        if matches!(
            operation,
            NoiseOperation::Measurement
        ) {
            return Err(
                NoiseError::InvalidOperation {
                    operation,
                    message:
                        "measurement erasure should use an explicit readout model"
                            .to_owned(),
                },
            );
        }

        Ok(Self {
            probability,
            operation,
        })
    }
}

impl NoiseModel
    for ErasureNoise
{
    fn name(
        &self,
    ) -> &'static str {
        "erasure"
    }

    fn validate(
        &self,
    ) -> Result<(), NoiseError> {
        Probability::from_scaled(
            self.probability.scaled(),
        )
        .map(|_| ())
    }

    fn sample(
        &self,
        context: NoiseContext,
        limits: &QecLimits,
    ) -> QecResult<FaultBatch> {
        self.validate()
            .map_err(
                QecError::from,
            )?;

        let mut rng =
            DeterministicRng::new(
                context.seed,
            );

        let mut faults =
            Vec::new();

        for index
            in 0..context.qubits
        {
            if rng.bernoulli(
                self.probability,
            ) {
                faults.push(
                    Fault::erasure(
                        self.operation,
                        QubitId::new(
                            index,
                        )
                        .map_err(
                            QecError::from,
                        )?,
                    )
                    .map_err(
                        QecError::from,
                    )?,
                );
            }

            if faults.len()
                > limits.max_syndrome_events
            {
                return Err(
                    QecError::ResourceLimitExceeded {
                        resource:
                            ResourceKind::SyndromeEvents,
                        requested:
                            faults.len() as u128,
                        limit:
                            limits.max_syndrome_events
                                as u128,
                        message:
                            "erasure sampling exceeded the configured fault budget"
                                .to_owned(),
                    },
                );
            }
        }

        FaultBatch::with_limits(
            faults,
            limits,
        )
    }
}

// ============================================================================
// Correlated noise model
// ============================================================================

/// Deterministic correlated fault specification.
///
/// This model represents an explicit list of correlated Pauli events rather
/// than inventing topology or hardware connectivity.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub struct CorrelatedNoise {
    operation: NoiseOperation,
    groups: Vec<CorrelatedNoiseGroup>,
}

impl CorrelatedNoise {
    /// Creates an empty correlated model.
    pub fn new(
        operation: NoiseOperation,
    ) -> Result<Self, NoiseError> {
        if !operation.supports_pauli() {
            return Err(
                NoiseError::InvalidOperation {
                    operation,
                    message:
                        "correlated Pauli noise cannot use measurement operation"
                            .to_owned(),
                },
            );
        }

        Ok(Self {
            operation,
            groups: Vec::new(),
        })
    }

    /// Adds a deterministic correlated group.
    pub fn add_group(
        &mut self,
        qubits: Vec<QubitId>,
        paulis: Vec<PauliError>,
    ) -> Result<(), NoiseError> {
        let fault =
            Fault::correlated(
                self.operation,
                qubits,
                paulis,
            )?;

        let group =
            match fault {
                Fault::Correlated {
                    qubits,
                    paulis,
                    ..
                } => {
                    CorrelatedNoiseGroup {
                        qubits,
                        paulis,
                    }
                }

                _ => {
                    return Err(
                        NoiseError::InternalInvariant(
                            "correlated constructor returned non-correlated fault"
                                .to_owned(),
                        ),
                    );
                }
            };

        self.groups.push(
            group,
        );

        Ok(())
    }

    /// Number of configured correlated groups.
    pub fn group_count(
        &self,
    ) -> usize {
        self.groups.len()
    }
}

impl NoiseModel
    for CorrelatedNoise
{
    fn name(
        &self,
    ) -> &'static str {
        "correlated"
    }

    fn validate(
        &self,
    ) -> Result<(), NoiseError> {
        for group in
            &self.groups
        {
            Fault::correlated(
                self.operation,
                group.qubits.clone(),
                group.paulis.clone(),
            )?;
        }

        Ok(())
    }

    fn sample(
        &self,
        _context: NoiseContext,
        limits: &QecLimits,
    ) -> QecResult<FaultBatch> {
        self.validate()
            .map_err(
                QecError::from,
            )?;

        let mut faults =
            Vec::with_capacity(
                self.groups.len(),
            );

        for group in
            &self.groups
        {
            faults.push(
                Fault::correlated(
                    self.operation,
                    group.qubits.clone(),
                    group.paulis.clone(),
                )
                .map_err(
                    QecError::from,
                )?,
            );

            if faults.len()
                > limits.max_syndrome_events
            {
                return Err(
                    QecError::ResourceLimitExceeded {
                        resource:
                            ResourceKind::SyndromeEvents,
                        requested:
                            faults.len() as u128,
                        limit:
                            limits.max_syndrome_events
                                as u128,
                        message:
                            "correlated-noise sampling exceeded the configured fault budget"
                                .to_owned(),
                    },
                );
            }
        }

        FaultBatch::with_limits(
            faults,
            limits,
        )
    }
}

/// Internal immutable correlated group.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
struct CorrelatedNoiseGroup {
    qubits: Vec<QubitId>,
    paulis: Vec<PauliError>,
}

// ============================================================================
// Noise composition
// ============================================================================

/// A deterministic collection of independent noise models.
///
/// Models are evaluated in declared order and their resulting fault batches
/// are concatenated. No model may silently reorder another model's events.
pub struct CompositeNoise {
    models:
        Vec<Box<dyn NoiseModel>>,
}

impl CompositeNoise {
    /// Creates an empty composite model.
    #[must_use]
    pub fn new() -> Self {
        Self {
            models: Vec::new(),
        }
    }

    /// Adds a model.
    pub fn push<M>(
        &mut self,
        model: M,
    )
    where
        M: NoiseModel + 'static,
    {
        self.models.push(
            Box::new(model),
        );
    }

    /// Returns the number of child models.
    pub fn len(
        &self,
    ) -> usize {
        self.models.len()
    }

    /// Returns true when no child models exist.
    pub fn is_empty(
        &self,
    ) -> bool {
        self.models.is_empty()
    }
}

impl Default for CompositeNoise {
    fn default() -> Self {
        Self::new()
    }
}

impl NoiseModel
    for CompositeNoise
{
    fn name(
        &self,
    ) -> &'static str {
        "composite"
    }

    fn validate(
        &self,
    ) -> Result<(), NoiseError> {
        for model in
            &self.models
        {
            model.validate()?;
        }

        Ok(())
    }

    fn sample(
        &self,
        context: NoiseContext,
        limits: &QecLimits,
    ) -> QecResult<FaultBatch> {
        self.validate()
            .map_err(
                QecError::from,
            )?;

        let mut result =
            FaultBatch::new();

        for (
            model_index,
            model,
        ) in self.models.iter().enumerate()
        {
            // Derive an independent deterministic stream for every model.
            let model_seed =
                derive_seed(
                    context.seed,
                    model_index as u64,
                );

            let model_context =
                NoiseContext {
                    qubits:
                        context.qubits,
                    round:
                        context.round,
                    seed:
                        model_seed,
                };

            let batch =
                model.sample(
                    model_context,
                    limits,
                )?;

            for fault in
                batch.as_slice()
            {
                result.push_with_limits(
                    fault.clone(),
                    limits,
                )?;
            }
        }

        Ok(result)
    }
}

// ============================================================================
// Resource estimation
// ============================================================================

/// Conservative estimate of memory needed for a fault batch.
///
/// This is intentionally an upper-bound estimate rather than an allocator
/// measurement. The actual ResourceManager remains responsible for runtime
/// accounting.
pub fn estimate_fault_batch_memory(
    fault_count: usize,
) -> Result<u64, NoiseError> {
    const BATCH_OVERHEAD: u64 =
        24;

    const FAULT_BASE: u64 =
        64;

    const QUBIT_ID_BYTES: u64 =
        core::mem::size_of::<QubitId>()
            as u64;

    const PAULI_BYTES: u64 =
        core::mem::size_of::<PauliError>()
            as u64;

    let fault_count =
        u64::try_from(
            fault_count,
        )
        .map_err(
            |_| NoiseError::ArithmeticOverflow,
        )?;

    let per_fault =
        FAULT_BASE
            .checked_add(
                QUBIT_ID_BYTES
                    .saturating_mul(
                        MAX_CORRELATED_QUBITS
                            as u64,
                    ),
            )
            .and_then(
                |value| {
                    value.checked_add(
                        PAULI_BYTES
                            .saturating_mul(
                                MAX_CORRELATED_QUBITS
                                    as u64,
                            ),
                    )
                },
            )
            .ok_or(
                NoiseError::ArithmeticOverflow,
            )?;

    BATCH_OVERHEAD
        .checked_add(
            fault_count
                .checked_mul(
                    per_fault,
                )
                .ok_or(
                    NoiseError::ArithmeticOverflow,
                )?,
        )
        .ok_or(
            NoiseError::ArithmeticOverflow,
        )
}

// ============================================================================
// Deterministic helpers
// ============================================================================

fn derive_seed(
    seed: u64,
    stream: u64,
) -> u64 {
    seed
        .wrapping_add(
            stream.wrapping_mul(
                0x9E37_79B9_7F4A_7C15,
            ),
        )
        .rotate_left(
            17,
        )
        ^ 0xD1B5_4A32_D192_ED03
}

// ============================================================================
// Noise errors
// ============================================================================

/// Errors produced by the noise representation/model layer.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub enum NoiseError {
    /// Physical qubit identifier is outside the supported API range.
    InvalidQubitId {
        id: usize,
    },

    /// Probability fixed-point value is outside [0, 1].
    InvalidProbability {
        scaled: u64,
    },

    /// Integer percentage is outside [0, 100].
    InvalidPercentage {
        percent: u8,
    },

    /// Basis-point value is outside [0, 10,000].
    InvalidBasisPoints {
        basis_points: u16,
    },

    /// Probability components sum to more than one.
    ProbabilitySumExceedsOne {
        p_x: Probability,
        p_y: Probability,
        p_z: Probability,
    },

    /// Arithmetic overflow occurred.
    ArithmeticOverflow,

    /// Identity is not a physical fault.
    IdentityFault,

    /// Operation cannot represent the requested fault type.
    InvalidOperation {
        operation: NoiseOperation,
        message: String,
    },

    /// Correlated fault contains no qubits.
    EmptyCorrelatedFault,

    /// Correlated qubit and Pauli arrays differ in length.
    MismatchedCorrelatedLengths {
        qubits: usize,
        paulis: usize,
    },

    /// Correlated fault exceeds the local safety boundary.
    CorrelatedFaultTooLarge {
        requested: usize,
        maximum: usize,
    },

    /// Correlated qubits are not strictly increasing.
    NonCanonicalCorrelatedQubits,

    /// Fault batch exceeds the legacy/default hard boundary.
    FaultBatchTooLarge {
        requested: usize,
        maximum: usize,
    },

    /// Internal invariant violation.
    InternalInvariant(
        String,
    ),
}

impl fmt::Display
    for NoiseError
{
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidQubitId {
                id,
            } => write!(
                formatter,
                "invalid physical qubit identifier: {}",
                id
            ),

            Self::InvalidProbability {
                scaled,
            } => write!(
                formatter,
                "invalid fixed-point probability: {}",
                scaled
            ),

            Self::InvalidPercentage {
                percent,
            } => write!(
                formatter,
                "invalid percentage: {}",
                percent
            ),

            Self::InvalidBasisPoints {
                basis_points,
            } => write!(
                formatter,
                "invalid basis points: {}",
                basis_points
            ),

            Self::ProbabilitySumExceedsOne {
                ..
            } => write!(
                formatter,
                "Pauli probability sum exceeds one"
            ),

            Self::ArithmeticOverflow =>
                write!(
                    formatter,
                    "arithmetic overflow in noise calculation"
                ),

            Self::IdentityFault =>
                write!(
                    formatter,
                    "identity is not a physical fault"
                ),

            Self::InvalidOperation {
                operation,
                message,
            } => write!(
                formatter,
                "invalid noise operation {:?}: {}",
                operation,
                message
            ),

            Self::EmptyCorrelatedFault =>
                write!(
                    formatter,
                    "correlated fault cannot be empty"
                ),

            Self::MismatchedCorrelatedLengths {
                qubits,
                paulis,
            } => write!(
                formatter,
                "correlated fault has {} qubits but {} Pauli values",
                qubits,
                paulis
            ),

            Self::CorrelatedFaultTooLarge {
                requested,
                maximum,
            } => write!(
                formatter,
                "correlated fault size {} exceeds maximum {}",
                requested,
                maximum
            ),

            Self::NonCanonicalCorrelatedQubits =>
                write!(
                    formatter,
                    "correlated qubits must be strictly increasing"
                ),

            Self::FaultBatchTooLarge {
                requested,
                maximum,
            } => write!(
                formatter,
                "fault batch size {} exceeds maximum {}",
                requested,
                maximum
            ),

            Self::InternalInvariant(
                message,
            ) => write!(
                formatter,
                "noise internal invariant violation: {}",
                message
            ),
        }
    }
}

impl std::error::Error
    for NoiseError
{
}

impl From<NoiseError>
    for QecError
{
    fn from(
        error: NoiseError,
    ) -> Self {
        match error {
            NoiseError::InvalidQubitId {
                id,
            } => QecError::InvalidInput {
                message:
                    format!(
                        "invalid physical qubit identifier: {}",
                        id
                    ),
            },

            NoiseError::InvalidProbability {
                scaled,
            } => QecError::InvalidProbability {
                probability:
                    scaled as f64
                        / PROBABILITY_SCALE as f64,
                message:
                    "probability is outside the valid [0,1] domain"
                        .to_owned(),
            },

            NoiseError::InvalidPercentage {
                percent,
            } => QecError::InvalidProbability {
                probability:
                    percent as f64
                        / 100.0,
                message:
                    "percentage is outside the valid [0,100] domain"
                        .to_owned(),
            },

            NoiseError::InvalidBasisPoints {
                basis_points,
            } => QecError::InvalidProbability {
                probability:
                    basis_points as f64
                        / 10_000.0,
                message:
                    "basis-point probability is invalid"
                        .to_owned(),
            },

            NoiseError::ProbabilitySumExceedsOne {
                ..
            } => QecError::InvalidProbability {
                probability: 1.0,
                message:
                    "Pauli probability components exceed total probability one"
                        .to_owned(),
            },

            NoiseError::ArithmeticOverflow =>
                QecError::NumericalFailure {
                    operation:
                        NumericalOperation::Accumulation,
                    message:
                        "checked noise arithmetic overflowed"
                            .to_owned(),
                },

            NoiseError::IdentityFault =>
                QecError::InvalidInput {
                    message:
                        "identity is not a physical fault"
                            .to_owned(),
                },

            NoiseError::InvalidOperation {
                operation,
                message,
            } => QecError::InvalidInput {
                message:
                    format!(
                        "invalid {:?} noise operation: {}",
                        operation,
                        message
                    ),
            },

            NoiseError::EmptyCorrelatedFault =>
                QecError::InvalidInput {
                    message:
                        "correlated fault cannot be empty"
                            .to_owned(),
                },

            NoiseError::MismatchedCorrelatedLengths {
                qubits,
                paulis,
            } => QecError::InvalidInput {
                message:
                    format!(
                        "correlated fault has {} qubits and {} Pauli values",
                        qubits,
                        paulis
                    ),
            },

            NoiseError::CorrelatedFaultTooLarge {
                requested,
                maximum,
            } => QecError::ResourceLimitExceeded {
                resource:
                    ResourceKind::Qubits,
                requested:
                    requested as u128,
                limit:
                    maximum as u128,
                message:
                    "correlated fault exceeds the configured physical-event boundary"
                        .to_owned(),
            },

            NoiseError::NonCanonicalCorrelatedQubits =>
                QecError::InvalidInput {
                    message:
                        "correlated qubits must be strictly increasing"
                            .to_owned(),
                },

            NoiseError::FaultBatchTooLarge {
                requested,
                maximum,
            } => QecError::ResourceLimitExceeded {
                resource:
                    ResourceKind::SyndromeEvents,
                requested:
                    requested as u128,
                limit:
                    maximum as u128,
                message:
                    "fault batch exceeds the production event boundary"
                        .to_owned(),
            },

            NoiseError::InternalInvariant(
                message,
            ) => QecError::InternalInvariantViolation {
                invariant:
                    "noise representation invariant"
                        .to_owned(),
                message,
            },
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn probability_rejects_values_above_one() {
        assert!(
            Probability::from_scaled(
                PROBABILITY_SCALE
                    .saturating_add(1),
            )
            .is_err()
        );
    }

    #[test]
    fn probability_percent_is_deterministic() {
        let probability =
            Probability::from_percent(
                50,
            )
            .expect(
                "50 percent must be valid",
            );

        assert_eq!(
            probability.scaled(),
            500_000_000_000
        );
    }

    #[test]
    fn depolarizing_channel_is_normalized() {
        let channel =
            PauliNoiseChannel::depolarizing(
                Probability::from_percent(
                    30,
                )
                .expect(
                    "30 percent must be valid",
                ),
            )
            .expect(
                "valid depolarizing channel",
            );

        assert!(
            channel
                .total_error_probability()
                .scaled()
                <= PROBABILITY_SCALE
        );
    }

    #[test]
    fn identity_fault_is_rejected() {
        assert!(
            Fault::pauli(
                NoiseOperation::Qubit,
                QubitId::new(0)
                    .expect(
                        "q0 must be valid",
                    ),
                PauliError::I,
            )
            .is_err()
        );
    }

    #[test]
    fn correlated_fault_is_canonical() {
        let fault =
            Fault::correlated(
                NoiseOperation::Gate,
                vec![
                    QubitId::new(1)
                        .expect(
                            "q1",
                        ),
                    QubitId::new(2)
                        .expect(
                            "q2",
                        ),
                ],
                vec![
                    PauliError::X,
                    PauliError::Z,
                ],
            )
            .expect(
                "canonical correlated fault",
            );

        assert_eq!(
            fault.qubit_count(),
            2
        );
    }

    #[test]
    fn noncanonical_correlated_fault_is_rejected() {
        assert!(
            Fault::correlated(
                NoiseOperation::Gate,
                vec![
                    QubitId::new(2)
                        .expect(
                            "q2",
                        ),
                    QubitId::new(1)
                        .expect(
                            "q1",
                        ),
                ],
                vec![
                    PauliError::X,
                    PauliError::Z,
                ],
            )
            .is_err()
        );
    }

    #[test]
    fn same_seed_produces_same_noise() {
        let limits =
            QecLimits::new();

        let context =
            NoiseContext::new(
                64,
                0,
                12345,
                &limits,
            )
            .expect(
                "context must be valid",
            );

        let model =
            BitFlipNoise::new(
                Probability::from_percent(
                    10,
                )
                .expect(
                    "10 percent",
                ),
                NoiseOperation::Qubit,
            )
            .expect(
                "valid model",
            );

        let first =
            model
                .sample(
                    context,
                    &limits,
                )
                .expect(
                    "sampling must succeed",
                );

        let second =
            model
                .sample(
                    context,
                    &limits,
                )
                .expect(
                    "sampling must succeed",
                );

        assert_eq!(
            first,
            second
        );
    }

    #[test]
    fn different_seed_can_produce_different_noise() {
        let limits =
            QecLimits::new();

        let first_context =
            NoiseContext::new(
                128,
                0,
                1,
                &limits,
            )
            .expect(
                "valid context",
            );

        let second_context =
            NoiseContext::new(
                128,
                0,
                2,
                &limits,
            )
            .expect(
                "valid context",
            );

        let model =
            BitFlipNoise::new(
                Probability::from_percent(
                    50,
                )
                .expect(
                    "50 percent",
                ),
                NoiseOperation::Qubit,
            )
            .expect(
                "valid model",
            );

        let first =
            model
                .sample(
                    first_context,
                    &limits,
                )
                .expect(
                    "sampling",
                );

        let second =
            model
                .sample(
                    second_context,
                    &limits,
                )
                .expect(
                    "sampling",
                );

        assert_ne!(
            first,
            second
        );
    }
}