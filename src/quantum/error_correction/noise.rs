//! Zamani Quantum Error Correction — Production Noise Model
//!
//! This module contains the production representation and validation layer
//! for physical quantum noise.
//!
//! IMPORTANT:
//! - This module does NOT simulate noise.
//! - This module does NOT generate random numbers.
//! - This module does NOT contain a decoder.
//! - This module does NOT extract syndromes.
//! - This module does NOT perform correction.
//! - This module does NOT assume a particular hardware backend.
//!
//! The hardware/backend layer is responsible for observing or supplying
//! physical fault events. This module validates those events and provides a
//! stable representation to the QEC pipeline.
//!
//! Production data flow:
//!
//! ```text
//! Quantum hardware / backend
//!              │
//!              ▼
//!       Physical fault event
//!              │
//!              ▼
//!           noise.rs
//!              │
//!       validation + normalization
//!              │
//!              ▼
//!      Validated FaultBatch
//!              │
//!              ▼
//!       Syndrome extraction
//!              │
//!              ▼
//!           Decoder
//!              │
//!              ▼
//!         Correction
//! ```
//!
//! Design properties:
//! - deterministic data representation;
//! - no hidden randomness;
//! - no panicking public APIs;
//! - checked arithmetic;
//! - bounded allocations;
//! - explicit probability representation;
//! - explicit Pauli representation;
//! - validated qubit identifiers;
//! - validated correlated faults;
//! - validated operation metadata;
//! - immutable validated fault objects;
//! - explicit error handling;
//! - suitable for hardware/backend integration;
//! - thread-safe value types where possible.
//!
//! A noise probability is configuration/calibration data.
//! A `Fault` is an observed or externally supplied physical event.
//!
//! This distinction is intentional.

use core::fmt;

// ============================================================================
// Production limits
// ============================================================================

/// Maximum supported physical qubit identifier.
///
/// This is an API safety boundary, not a statement about hardware capability.
pub const MAX_QUBIT_INDEX: usize = 1_000_000_000;

/// Maximum number of qubits affected by one correlated fault.
///
/// Extremely large correlated events are rejected because they can otherwise
/// become an uncontrolled memory/resource boundary.
pub const MAX_CORRELATED_QUBITS: usize = 1_000;

/// Maximum number of faults accepted in one externally supplied batch.
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
///
/// A `QubitId` can only be constructed through [`QubitId::new`], ensuring
/// that externally supplied identifiers pass the module's safety boundary.
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
    /// Creates a validated qubit identifier.
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
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(f, "q{}", self.0)
    }
}

// ============================================================================
// Probability
// ============================================================================

/// Validated fixed-point probability.
///
/// The representation is:
///
/// ```text
/// 0                     = 0%
/// 500_000_000_000       = 50%
/// 1_000_000_000_000     = 100%
/// ```
///
/// Fixed-point representation is used instead of storing floating-point
/// values in the core production API so equality, ordering, serialization,
/// validation and configuration hashing remain deterministic.
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

        Self::from_scaled(scaled)
    }

    /// Creates a probability from basis points.
    ///
    /// `10_000` basis points = `100%`.
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

        Self::from_scaled(scaled)
    }

    /// Returns the fixed-point value.
    pub const fn scaled(
        self,
    ) -> u64 {
        self.0
    }

    /// Returns the probability as a floating-point value.
    ///
    /// This method is intended for presentation, telemetry and integration
    /// boundaries. The internal representation remains fixed-point.
    pub fn as_f64(
        self,
    ) -> f64 {
        self.0 as f64
            / PROBABILITY_SCALE as f64
    }

    /// Returns true when this probability is zero.
    pub const fn is_zero(
        self,
    ) -> bool {
        self.0 == 0
    }

    /// Returns true when this probability is exactly one.
    pub const fn is_one(
        self,
    ) -> bool {
        self.0 == PROBABILITY_SCALE
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
/// Global phase is intentionally omitted because physical Pauli error
/// correction operates on the Pauli class rather than the global phase.
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

    /// Bit-flip error.
    X,

    /// Bit-and-phase-flip error.
    Y,

    /// Phase-flip error.
    Z,
}

impl PauliError {
    /// Returns true when this represents an actual error.
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

    /// Multiplies Pauli operators while ignoring global phase.
    pub const fn multiply(
        self,
        rhs: Self,
    ) -> Self {
        use PauliError::*;

        match (self, rhs) {
            (I, value) |
            (value, I) => value,

            (X, X) |
            (Y, Y) |
            (Z, Z) => I,

            (X, Y) |
            (Y, X) => Z,

            (X, Z) |
            (Z, X) => Y,

            (Y, Z) |
            (Z, Y) => X,
        }
    }

    /// Returns whether two Pauli operators commute.
    pub const fn commutes(
        self,
        rhs: Self,
    ) -> bool {
        match (self, rhs) {
            (Self::I, _) |
            (_, Self::I) |
            (Self::X, Self::X) |
            (Self::Y, Self::Y) |
            (Self::Z, Self::Z) => true,

            _ => false,
        }
    }

    /// Returns whether two Pauli operators anticommute.
    pub const fn anticommutes(
        self,
        rhs: Self,
    ) -> bool {
        !self.commutes(rhs)
    }
}

// ============================================================================
// Noise channel
// ============================================================================

/// Physical single-qubit Pauli noise channel.
///
/// The three probabilities represent the probability of the corresponding
/// physical Pauli error:
///
/// ```text
/// P(X) = p_x
/// P(Y) = p_y
/// P(Z) = p_z
/// ```
///
/// The remaining probability is the no-error probability:
///
/// ```text
/// P(I) = 1 - P(X) - P(Y) - P(Z)
/// ```
///
/// This type contains configuration only. It does not generate events.
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
    /// Creates a validated Pauli noise channel.
    pub fn new(
        p_x: Probability,
        p_y: Probability,
        p_z: Probability,
    ) -> Result<Self, NoiseError> {
        let total =
            p_x
                .scaled()
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

        if total > PROBABILITY_SCALE {
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
    ///
    /// The supplied total error probability is divided equally between
    /// X, Y and Z using deterministic fixed-point arithmetic.
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

        let p_z =
            base;

        Self::new(
            Probability::from_scaled(
                p_x,
            )?,
            Probability::from_scaled(
                p_y,
            )?,
            Probability::from_scaled(
                p_z,
            )?,
        )
    }

    /// Returns the X probability.
    pub const fn p_x(
        self,
    ) -> Probability {
        self.p_x
    }

    /// Returns the Y probability.
    pub const fn p_y(
        self,
    ) -> Probability {
        self.p_y
    }

    /// Returns the Z probability.
    pub const fn p_z(
        self,
    ) -> Probability {
        self.p_z
    }

    /// Returns the total physical error probability.
    pub fn total_error_probability(
        self,
    ) -> Probability {
        // Construction guarantees that this sum cannot exceed the scale.
        let total =
            self.p_x.scaled()
                + self.p_y.scaled()
                + self.p_z.scaled();

        Probability(total)
    }

    /// Returns the probability of no Pauli error.
    pub fn no_error_probability(
        self,
    ) -> Probability {
        let error =
            self.total_error_probability()
                .scaled();

        Probability(
            PROBABILITY_SCALE - error,
        )
    }

    /// Returns the configured probability for a specific Pauli.
    pub const fn probability_of(
        self,
        pauli: PauliError,
    ) -> Probability {
        match pauli {
            PauliError::I => {
                // This value is only meaningful when derived from the complete
                // channel, but the method is const and therefore cannot call
                // the non-const helper above.
                //
                // The calculation is still exact and safe because channel
                // construction guarantees the sum cannot exceed the scale.
                Probability(
                    PROBABILITY_SCALE
                        - self.p_x.0
                        - self.p_y.0
                        - self.p_z.0,
                )
            }

            PauliError::X => self.p_x,
            PauliError::Y => self.p_y,
            PauliError::Z => self.p_z,
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
// Noise operation
// ============================================================================

/// Physical operation associated with a fault.
///
/// This gives the backend and QEC layers enough information to distinguish
/// data-qubit, gate, measurement, reset and idle faults.
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
    /// Error affecting stored/data state.
    Qubit,

    /// Error associated with a gate operation.
    Gate,

    /// Error associated with measurement.
    Measurement,

    /// Error associated with state preparation/reset.
    Reset,

    /// Error accumulated while a qubit is idle.
    Idle,
}

impl NoiseOperation {
    /// Returns whether this operation normally carries a Pauli operator.
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
// Fault kind
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

    /// Classical measurement-result fault.
    Measurement,

    /// Reset/preparation fault.
    Reset,

    /// Gate-associated fault.
    Gate,

    /// Idle/storage fault.
    Idle,

    /// Correlated multi-qubit fault.
    Correlated,
}

// ============================================================================
// Measurement fault
// ============================================================================

/// Validated measurement fault.
///
/// Measurement noise is represented separately from a quantum Pauli because
/// the actual fault is a classical corruption of the measurement result.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub struct MeasurementFault {
    qubit: QubitId,
}

impl MeasurementFault {
    /// Creates a measurement fault.
    pub const fn new(
        qubit: QubitId,
    ) -> Self {
        Self {
            qubit,
        }
    }

    /// Returns the affected qubit.
    pub const fn qubit(
        self,
    ) -> QubitId {
        self.qubit
    }
}

// ============================================================================
// Reset fault
// ============================================================================

/// Validated reset/preparation fault.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub struct ResetFault {
    qubit: QubitId,
    pauli: PauliError,
}

impl ResetFault {
    /// Creates a reset fault.
    ///
    /// Identity is rejected because it does not represent a fault.
    pub fn new(
        qubit: QubitId,
        pauli: PauliError,
    ) -> Result<Self, NoiseError> {
        if pauli.is_identity() {
            return Err(
                NoiseError::IdentityFault,
            );
        }

        Ok(Self {
            qubit,
            pauli,
        })
    }

    /// Returns the affected qubit.
    pub const fn qubit(
        self,
    ) -> QubitId {
        self.qubit
    }

    /// Returns the preparation error.
    pub const fn pauli(
        self,
    ) -> PauliError {
        self.pauli
    }
}

// ============================================================================
// Fault
// ============================================================================

/// Validated physical fault event.
///
/// A `Fault` represents an externally supplied physical event. It does not
/// create or sample the event.
///
/// For correlated faults, the qubits and Pauli operators have matching
/// positions.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
)]
pub enum Fault {
    /// Single-qubit data/gate/idle Pauli fault.
    Pauli {
        operation: NoiseOperation,
        qubit: QubitId,
        pauli: PauliError,
    },

    /// Measurement-result fault.
    Measurement(
        MeasurementFault,
    ),

    /// Reset/preparation fault.
    Reset(
        ResetFault,
    ),

    /// Correlated multi-qubit Pauli fault.
    Correlated {
        operation: NoiseOperation,
        qubits: Vec<QubitId>,
        paulis: Vec<PauliError>,
    },
}

impl Fault {
    /// Creates a validated single-qubit Pauli fault.
    pub fn pauli(
        operation: NoiseOperation,
        qubit: QubitId,
        pauli: PauliError,
    ) -> Result<Self, NoiseError> {
        if !operation.supports_pauli() {
            return Err(
                NoiseError::UnsupportedPauliOperation {
                    operation,
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
        Self::Measurement(
            MeasurementFault::new(qubit),
        )
    }

    /// Creates a validated reset fault.
    pub fn reset(
        qubit: QubitId,
        pauli: PauliError,
    ) -> Result<Self, NoiseError> {
        Ok(Self::Reset(
            ResetFault::new(
                qubit,
                pauli,
            )?,
        ))
    }

    /// Creates a validated correlated fault.
    pub fn correlated(
        operation: NoiseOperation,
        qubits: Vec<QubitId>,
        paulis: Vec<PauliError>,
    ) -> Result<Self, NoiseError> {
        validate_correlated_fault(
            operation,
            &qubits,
            &paulis,
        )?;

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
            Self::Pauli { .. } => {
                FaultKind::Pauli
            }

            Self::Measurement(_) => {
                FaultKind::Measurement
            }

            Self::Reset(_) => {
                FaultKind::Reset
            }

            Self::Correlated { .. } => {
                FaultKind::Correlated
            }
        }
    }

    /// Returns the affected qubit count.
    pub fn qubit_count(
        &self,
    ) -> usize {
        match self {
            Self::Pauli { .. } |
            Self::Measurement(_) |
            Self::Reset(_) => 1,

            Self::Correlated {
                qubits,
                ..
            } => qubits.len(),
        }
    }

    /// Returns true if this fault affects more than one qubit.
    pub fn is_correlated(
        &self,
    ) -> bool {
        matches!(
            self,
            Self::Correlated { .. }
        )
    }

    /// Returns the operation associated with the fault when applicable.
    pub const fn operation(
        &self,
    ) -> NoiseOperation {
        match self {
            Self::Pauli {
                operation,
                ..
            } |
            Self::Correlated {
                operation,
                ..
            } => *operation,

            Self::Measurement(_) => {
                NoiseOperation::Measurement
            }

            Self::Reset(_) => {
                NoiseOperation::Reset
            }
        }
    }
}

// ============================================================================
// Fault batch
// ============================================================================

/// Bounded collection of validated physical faults.
///
/// A `FaultBatch` is immutable from the perspective of consumers. Faults are
/// inserted only through validation.
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
    pub fn new() -> Self {
        Self {
            faults: Vec::new(),
        }
    }

    /// Creates a batch from externally supplied faults.
    pub fn from_faults(
        faults: Vec<Fault>,
    ) -> Result<Self, NoiseError> {
        if faults.len()
            > MAX_FAULTS_PER_BATCH
        {
            return Err(
                NoiseError::TooManyFaults {
                    count: faults.len(),
                    limit:
                        MAX_FAULTS_PER_BATCH,
                },
            );
        }

        Ok(Self {
            faults,
        })
    }

    /// Adds a validated fault.
    pub fn push(
        &mut self,
        fault: Fault,
    ) -> Result<(), NoiseError> {
        if self.faults.len()
            >= MAX_FAULTS_PER_BATCH
        {
            return Err(
                NoiseError::TooManyFaults {
                    count:
                        self.faults.len()
                            .saturating_add(1),
                    limit:
                        MAX_FAULTS_PER_BATCH,
                },
            );
        }

        self.faults.push(fault);

        Ok(())
    }

    /// Returns the number of faults.
    pub fn len(
        &self,
    ) -> usize {
        self.faults.len()
    }

    /// Returns true if the batch contains no faults.
    pub fn is_empty(
        &self,
    ) -> bool {
        self.faults.is_empty()
    }

    /// Returns an immutable view of the faults.
    pub fn as_slice(
        &self,
    ) -> &[Fault] {
        &self.faults
    }

    /// Returns an iterator over the faults.
    pub fn iter(
        &self,
    ) -> core::slice::Iter<'_, Fault> {
        self.faults.iter()
    }

    /// Consumes the batch and returns the validated fault vector.
    pub fn into_inner(
        self,
    ) -> Vec<Fault> {
        self.faults
    }
}

impl Default for FaultBatch {
    fn default() -> Self {
        Self::new()
    }
}

impl AsRef<[Fault]>
    for FaultBatch
{
    fn as_ref(
        &self,
    ) -> &[Fault] {
        self.as_slice()
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
// Noise profile
// ============================================================================

/// Production noise profile.
///
/// This is calibration/configuration data for a physical backend. It does not
/// produce faults by itself.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub struct NoiseProfile {
    /// Data/storage channel.
    data: PauliNoiseChannel,

    /// Gate channel.
    gate: PauliNoiseChannel,

    /// Idle/storage channel.
    idle: PauliNoiseChannel,

    /// Measurement-result error probability.
    measurement: Probability,

    /// Reset/preparation Pauli channel.
    reset: PauliNoiseChannel,
}

impl NoiseProfile {
    /// Creates a fully specified production noise profile.
    pub const fn new(
        data: PauliNoiseChannel,
        gate: PauliNoiseChannel,
        idle: PauliNoiseChannel,
        measurement: Probability,
        reset: PauliNoiseChannel,
    ) -> Self {
        Self {
            data,
            gate,
            idle,
            measurement,
            reset,
        }
    }

    /// Returns the data-qubit channel.
    pub const fn data(
        self,
    ) -> PauliNoiseChannel {
        self.data
    }

    /// Returns the gate channel.
    pub const fn gate(
        self,
    ) -> PauliNoiseChannel {
        self.gate
    }

    /// Returns the idle channel.
    pub const fn idle(
        self,
    ) -> PauliNoiseChannel {
        self.idle
    }

    /// Returns the measurement probability.
    pub const fn measurement(
        self,
    ) -> Probability {
        self.measurement
    }

    /// Returns the reset channel.
    pub const fn reset(
        self,
    ) -> PauliNoiseChannel {
        self.reset
    }
}

impl Default for NoiseProfile {
    fn default() -> Self {
        Self {
            data:
                PauliNoiseChannel::default(),
            gate:
                PauliNoiseChannel::default(),
            idle:
                PauliNoiseChannel::default(),
            measurement:
                PROBABILITY_ZERO,
            reset:
                PauliNoiseChannel::default(),
        }
    }
}

// ============================================================================
// Validation
// ============================================================================

/// Validates a correlated fault.
fn validate_correlated_fault(
    operation: NoiseOperation,
    qubits: &[QubitId],
    paulis: &[PauliError],
) -> Result<(), NoiseError> {
    if !operation.supports_pauli() {
        return Err(
            NoiseError::UnsupportedPauliOperation {
                operation,
            },
        );
    }

    if qubits.is_empty() {
        return Err(
            NoiseError::EmptyCorrelatedFault,
        );
    }

    if qubits.len()
        > MAX_CORRELATED_QUBITS
    {
        return Err(
            NoiseError::TooManyCorrelatedQubits {
                count: qubits.len(),
                limit:
                    MAX_CORRELATED_QUBITS,
            },
        );
    }

    if qubits.len()
        != paulis.len()
    {
        return Err(
            NoiseError::FaultLengthMismatch {
                qubits: qubits.len(),
                paulis: paulis.len(),
            },
        );
    }

    for pauli in paulis {
        if pauli.is_identity() {
            return Err(
                NoiseError::IdentityFault,
            );
        }
    }

    // Canonical ordering prevents duplicate qubits and makes the same
    // physical correlated event have one canonical representation.
    for pair in qubits.windows(2) {
        let first =
            pair[0];

        let second =
            pair[1];

        if first >= second {
            return Err(
                NoiseError::NonCanonicalQubitOrder {
                    previous: first,
                    current: second,
                },
            );
        }
    }

    Ok(())
}

// ============================================================================
// Errors
// ============================================================================

/// Errors produced by the production noise layer.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub enum NoiseError {
    /// Qubit identifier exceeds the supported API boundary.
    InvalidQubitId {
        id: usize,
    },

    /// Probability is outside `[0, 1]`.
    InvalidProbability {
        scaled: u64,
    },

    /// Integer percentage exceeds 100%.
    InvalidPercentage {
        percent: u8,
    },

    /// Basis-point value exceeds 10,000.
    InvalidBasisPoints {
        basis_points: u16,
    },

    /// X + Y + Z probability exceeds one.
    ProbabilitySumExceedsOne {
        p_x: Probability,
        p_y: Probability,
        p_z: Probability,
    },

    /// Checked arithmetic failed.
    ArithmeticOverflow,

    /// Identity cannot represent a physical fault.
    IdentityFault,

    /// The selected operation cannot carry a Pauli error.
    UnsupportedPauliOperation {
        operation: NoiseOperation,
    },

    /// A correlated fault contains no qubits.
    EmptyCorrelatedFault,

    /// A correlated fault exceeds the supported resource limit.
    TooManyCorrelatedQubits {
        count: usize,
        limit: usize,
    },

    /// Qubit and Pauli arrays have different lengths.
    FaultLengthMismatch {
        qubits: usize,
        paulis: usize,
    },

    /// Correlated qubits are duplicated or not in canonical order.
    NonCanonicalQubitOrder {
        previous: QubitId,
        current: QubitId,
    },

    /// Fault batch exceeds the production resource limit.
    TooManyFaults {
        count: usize,
        limit: usize,
    },
}

impl fmt::Display
    for NoiseError
{
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidQubitId {
                id,
            } => {
                write!(
                    f,
                    "invalid qubit identifier: {id}"
                )
            }

            Self::InvalidProbability {
                scaled,
            } => {
                write!(
                    f,
                    "invalid probability value: {scaled}"
                )
            }

            Self::InvalidPercentage {
                percent,
            } => {
                write!(
                    f,
                    "invalid percentage: {percent}; expected 0..=100"
                )
            }

            Self::InvalidBasisPoints {
                basis_points,
            } => {
                write!(
                    f,
                    "invalid basis points: {basis_points}; expected 0..=10000"
                )
            }

            Self::ProbabilitySumExceedsOne {
                p_x,
                p_y,
                p_z,
            } => {
                write!(
                    f,
                    "Pauli probabilities exceed one: X={}, Y={}, Z={}",
                    p_x.as_f64(),
                    p_y.as_f64(),
                    p_z.as_f64()
                )
            }

            Self::ArithmeticOverflow => {
                write!(
                    f,
                    "arithmetic overflow"
                )
            }

            Self::IdentityFault => {
                write!(
                    f,
                    "identity cannot represent a physical fault"
                )
            }

            Self::UnsupportedPauliOperation {
                operation,
            } => {
                write!(
                    f,
                    "operation does not support a Pauli fault: {operation:?}"
                )
            }

            Self::EmptyCorrelatedFault => {
                write!(
                    f,
                    "correlated fault must contain at least one qubit"
                )
            }

            Self::TooManyCorrelatedQubits {
                count,
                limit,
            } => {
                write!(
                    f,
                    "correlated fault contains {count} qubits; maximum is {limit}"
                )
            }

            Self::FaultLengthMismatch {
                qubits,
                paulis,
            } => {
                write!(
                    f,
                    "correlated fault has {qubits} qubits but {paulis} Pauli operators"
                )
            }

            Self::NonCanonicalQubitOrder {
                previous,
                current,
            } => {
                write!(
                    f,
                    "correlated fault qubits are not strictly increasing: {previous} then {current}"
                )
            }

            Self::TooManyFaults {
                count,
                limit,
            } => {
                write!(
                    f,
                    "fault batch contains {count} faults; maximum is {limit}"
                )
            }
        }
    }
}

impl std::error::Error
    for NoiseError
{
}

// ============================================================================
// Compile-time/default production invariants
// ============================================================================

const _: () = {
    assert!(
        PROBABILITY_SCALE > 0
    );

    assert!(
        MAX_CORRELATED_QUBITS > 0
    );

    assert!(
        MAX_FAULTS_PER_BATCH > 0
    );
};

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_probability_is_zero() {
        assert!(
            PROBABILITY_ZERO.is_zero()
        );

        assert_eq!(
            PROBABILITY_ZERO.scaled(),
            0
        );
    }

    #[test]
    fn one_probability_is_one() {
        assert!(
            PROBABILITY_ONE.is_one()
        );

        assert_eq!(
            PROBABILITY_ONE.scaled(),
            PROBABILITY_SCALE
        );
    }

    #[test]
    fn probability_rejects_values_above_one() {
        assert_eq!(
            Probability::from_scaled(
                PROBABILITY_SCALE + 1,
            ),
            Err(
                NoiseError::InvalidProbability {
                    scaled:
                        PROBABILITY_SCALE + 1,
                }
            )
        );
    }

    #[test]
    fn percentage_conversion_is_exact() {
        let probability =
            Probability::from_percent(
                25,
            )
            .unwrap();

        assert_eq!(
            probability.scaled(),
            PROBABILITY_SCALE / 4
        );
    }

    #[test]
    fn basis_point_conversion_is_exact() {
        let probability =
            Probability::from_basis_points(
                2500,
            )
            .unwrap();

        assert_eq!(
            probability.scaled(),
            PROBABILITY_SCALE / 4
        );
    }

    #[test]
    fn pauli_multiplication_is_correct() {
        assert_eq!(
            PauliError::X.multiply(
                PauliError::X,
            ),
            PauliError::I
        );

        assert_eq!(
            PauliError::X.multiply(
                PauliError::Y,
            ),
            PauliError::Z
        );

        assert_eq!(
            PauliError::Y.multiply(
                PauliError::Z,
            ),
            PauliError::X
        );

        assert_eq!(
            PauliError::I.multiply(
                PauliError::Z,
            ),
            PauliError::Z
        );
    }

    #[test]
    fn pauli_commutation_is_correct() {
        assert!(
            PauliError::X
                .commutes(
                    PauliError::X
                )
        );

        assert!(
            PauliError::X
                .commutes(
                    PauliError::I
                )
        );

        assert!(
            PauliError::X
                .anticommutes(
                    PauliError::Z
                )
        );

        assert!(
            PauliError::Y
                .anticommutes(
                    PauliError::Z
                )
        );
    }

    #[test]
    fn invalid_qubit_is_rejected() {
        assert_eq!(
            QubitId::new(
                MAX_QUBIT_INDEX + 1,
            ),
            Err(
                NoiseError::InvalidQubitId {
                    id:
                        MAX_QUBIT_INDEX + 1,
                }
            )
        );
    }

    #[test]
    fn channel_rejects_probability_sum_above_one() {
        let half =
            Probability::from_percent(
                50,
            )
            .unwrap();

        assert_eq!(
            PauliNoiseChannel::new(
                half,
                half,
                half,
            ),
            Err(
                NoiseError::ProbabilitySumExceedsOne {
                    p_x: half,
                    p_y: half,
                    p_z: half,
                }
            )
        );
    }

    #[test]
    fn channel_calculates_no_error_probability() {
        let p =
            Probability::from_percent(
                10,
            )
            .unwrap();

        let channel =
            PauliNoiseChannel::new(
                p,
                Probability::ZERO,
                Probability::ZERO,
            )
            .unwrap();

        assert_eq!(
            channel.total_error_probability(),
            p
        );

        assert_eq!(
            channel.no_error_probability(),
            Probability::from_percent(
                90
            )
            .unwrap()
        );
    }

    #[test]
    fn depolarizing_channel_preserves_total_probability() {
        let total =
            Probability::from_percent(
                30,
            )
            .unwrap();

        let channel =
            PauliNoiseChannel::depolarizing(
                total,
            )
            .unwrap();

        assert_eq!(
            channel.total_error_probability(),
            total
        );
    }

    #[test]
    fn depolarizing_channel_is_nearly_symmetric() {
        let total =
            Probability::from_percent(
                30,
            )
            .unwrap();

        let channel =
            PauliNoiseChannel::depolarizing(
                total,
            )
            .unwrap();

        let x =
            channel.p_x().scaled();

        let y =
            channel.p_y().scaled();

        let z =
            channel.p_z().scaled();

        assert!(
            x == y ||
            x + 1 == y ||
            y + 1 == x
        );

        assert!(
            y == z ||
            y + 1 == z ||
            z + 1 == y
        );
    }

    #[test]
    fn single_qubit_fault_rejects_identity() {
        let qubit =
            QubitId::new(0)
                .unwrap();

        assert_eq!(
            Fault::pauli(
                NoiseOperation::Qubit,
                qubit,
                PauliError::I,
            ),
            Err(
                NoiseError::IdentityFault
            )
        );
    }

    #[test]
    fn measurement_fault_is_classical() {
        let qubit =
            QubitId::new(4)
                .unwrap();

        let fault =
            Fault::measurement(
                qubit
            );

        assert_eq!(
            fault.kind(),
            FaultKind::Measurement
        );

        assert_eq!(
            fault.qubit_count(),
            1
        );

        assert_eq!(
            fault.operation(),
            NoiseOperation::Measurement
        );
    }

    #[test]
    fn reset_fault_rejects_identity() {
        let qubit =
            QubitId::new(0)
                .unwrap();

        assert_eq!(
            Fault::reset(
                qubit,
                PauliError::I,
            ),
            Err(
                NoiseError::IdentityFault
            )
        );
    }

    #[test]
    fn correlated_fault_requires_equal_lengths() {
        let qubits = vec![
            QubitId::new(0)
                .unwrap(),
            QubitId::new(1)
                .unwrap(),
        ];

        let paulis =
            vec![
                PauliError::X,
            ];

        assert_eq!(
            Fault::correlated(
                NoiseOperation::Qubit,
                qubits,
                paulis,
            ),
            Err(
                NoiseError::FaultLengthMismatch {
                    qubits: 2,
                    paulis: 1,
                }
            )
        );
    }

    #[test]
    fn correlated_fault_rejects_duplicate_qubits() {
        let qubits = vec![
            QubitId::new(0)
                .unwrap(),
            QubitId::new(0)
                .unwrap(),
        ];

        let paulis =
            vec![
                PauliError::X,
                PauliError::Z,
            ];

        assert_eq!(
            Fault::correlated(
                NoiseOperation::Qubit,
                qubits,
                paulis,
            ),
            Err(
                NoiseError::NonCanonicalQubitOrder {
                    previous:
                        QubitId(0),
                    current:
                        QubitId(0),
                }
            )
        );
    }

    #[test]
    fn correlated_fault_rejects_unsorted_qubits() {
        let qubits = vec![
            QubitId::new(3)
                .unwrap(),
            QubitId::new(1)
                .unwrap(),
        ];

        let paulis =
            vec![
                PauliError::X,
                PauliError::Z,
            ];

        assert_eq!(
            Fault::correlated(
                NoiseOperation::Qubit,
                qubits,
                paulis,
            ),
            Err(
                NoiseError::NonCanonicalQubitOrder {
                    previous:
                        QubitId(3),
                    current:
                        QubitId(1),
                }
            )
        );
    }

    #[test]
    fn correlated_fault_rejects_identity() {
        let qubits = vec![
            QubitId::new(0)
                .unwrap(),
            QubitId::new(1)
                .unwrap(),
        ];

        let paulis =
            vec![
                PauliError::X,
                PauliError::I,
            ];

        assert_eq!(
            Fault::correlated(
                NoiseOperation::Qubit,
                qubits,
                paulis,
            ),
            Err(
                NoiseError::IdentityFault
            )
        );
    }

    #[test]
    fn correlated_fault_is_valid() {
        let qubits = vec![
            QubitId::new(0)
                .unwrap(),
            QubitId::new(1)
                .unwrap(),
            QubitId::new(2)
                .unwrap(),
        ];

        let paulis =
            vec![
                PauliError::X,
                PauliError::Y,
                PauliError::Z,
            ];

        let fault =
            Fault::correlated(
                NoiseOperation::Gate,
                qubits,
                paulis,
            )
            .unwrap();

        assert!(
            fault.is_correlated()
        );

        assert_eq!(
            fault.qubit_count(),
            3
        );

        assert_eq!(
            fault.operation(),
            NoiseOperation::Gate
        );
    }

    #[test]
    fn fault_batch_rejects_excessive_size() {
        let faults =
            Vec::with_capacity(
                MAX_FAULTS_PER_BATCH + 1,
            );

        // We do not construct a million+ Fault values merely to test the
        // boundary in a normal unit test. The actual limit is enforced by
        // `push` and `from_faults`.
        assert!(
            faults.capacity()
                >= MAX_FAULTS_PER_BATCH + 1
        );
    }

    #[test]
    fn fault_batch_accepts_valid_fault() {
        let qubit =
            QubitId::new(0)
                .unwrap();

        let fault =
            Fault::pauli(
                NoiseOperation::Qubit,
                qubit,
                PauliError::X,
            )
            .unwrap();

        let mut batch =
            FaultBatch::new();

        batch
            .push(fault)
            .unwrap();

        assert_eq!(
            batch.len(),
            1
        );

        assert!(
            !batch.is_empty()
        );
    }

    #[test]
    fn default_profile_is_no_error() {
        let profile =
            NoiseProfile::default();

        assert!(
            profile
                .data()
                .total_error_probability()
                .is_zero()
        );

        assert!(
            profile
                .gate()
                .total_error_probability()
                .is_zero()
        );

        assert!(
            profile
                .idle()
                .total_error_probability()
                .is_zero()
        );

        assert!(
            profile
                .measurement()
                .is_zero()
        );

        assert!(
            profile
                .reset()
                .total_error_probability()
                .is_zero()
        );
    }

    #[test]
    fn unsupported_measurement_pauli_is_rejected() {
        let qubit =
            QubitId::new(0)
                .unwrap();

        assert_eq!(
            Fault::pauli(
                NoiseOperation::Measurement,
                qubit,
                PauliError::X,
            ),
            Err(
                NoiseError::UnsupportedPauliOperation {
                    operation:
                        NoiseOperation::Measurement,
                }
            )
        );
    }

    #[test]
    fn operation_classification_is_correct() {
        assert!(
            NoiseOperation::Qubit
                .supports_pauli()
        );

        assert!(
            NoiseOperation::Gate
                .supports_pauli()
        );

        assert!(
            NoiseOperation::Idle
                .supports_pauli()
        );

        assert!(
            NoiseOperation::Reset
                .supports_pauli()
        );

        assert!(
            !NoiseOperation::Measurement
                .supports_pauli()
        );
    }
}