//! Production Pauli-frame management for Zamani QEC.
//!
//! A Pauli frame is classical correction state.  It records the Pauli
//! correction that should be interpreted by the decoder, logical layer,
//! measurement layer, circuit executor, or QPU adapter without requiring
//! the correction to be physically applied immediately.
//!
//! # Architectural position
//!
//! ```text
//! Decoder
//!    |
//!    v
//! Pauli correction
//!    |
//!    v
//! PauliFrame
//!    |
//!    +--------------------+
//!    |                    |
//!    v                    v
//! measurement         logical classification
//!    |                    |
//!    +---------+----------+
//!              |
//!              v
//!       QPU / circuit layer
//! ```
//!
//! This module deliberately does NOT perform physical quantum operations.
//!
//! Global Pauli phase is ignored, matching the binary-symplectic
//! representation used by `stabilizer.rs`.
//!
//! # Safety invariants
//!
//! * a frame always represents exactly one non-zero qubit count;
//! * all frame dimensions are validated;
//! * resource limits come from `QecLimits`;
//! * no independent production allocation ceiling is maintained here;
//! * corrections are composed deterministically;
//! * cancellation is checked before and during bulk operations;
//! * measurement interpretation never mutates the frame;
//! * reset preserves the physical qubit count;
//! * checkpoints contain enough state to reproduce the frame;
//! * malformed checkpoints are rejected;
//! * this module never accesses QPU credentials, devices, circuits, or
//!   network resources.

use core::fmt;

use super::cancellation::CancellationToken;
use super::errors::{
    QecError,
    QecResult,
    ResourceKind,
};
use super::limits::{
    LimitError,
    QecLimits,
};
use super::stabilizer::{
    Pauli,
    PauliString,
    QubitIndex,
    StabilizerError,
};

// ============================================================================
// Versioning
// ============================================================================

/// Current serialized Pauli-frame checkpoint schema.
pub const PAULI_FRAME_CHECKPOINT_VERSION: u32 = 1;

/// Current logical frame representation version.
pub const PAULI_FRAME_FORMAT_VERSION: u32 = 1;

// ============================================================================
// Pauli frame
// ============================================================================

/// Classical Pauli frame.
///
/// The frame represents:
///
/// ```text
/// P = P_0 ⊗ P_1 ⊗ ... ⊗ P_(n-1)
/// ```
///
/// where each `P_i` is `I`, `X`, `Y`, or `Z`.
///
/// The frame is stored as a `PauliString` so that stabilizer algebra remains
/// centralized in `stabilizer.rs`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PauliFrame {
    operator: PauliString,

    /// Monotonically increasing mutation counter.
    ///
    /// This is useful for deterministic replay, checkpoint validation and
    /// detecting whether a frame changed between two observations.
    revision: u64,
}

impl PauliFrame {
    // ------------------------------------------------------------------------
    // Construction
    // ------------------------------------------------------------------------

    /// Creates an identity frame using the supplied QEC resource policy.
    pub fn new_with_limits(
        num_qubits: usize,
        limits: &QecLimits,
    ) -> Result<Self, PauliFrameError> {
        limits
            .validate()
            .map_err(PauliFrameError::InvalidLimits)?;

        if num_qubits == 0 {
            return Err(
                PauliFrameError::ZeroQubits
            );
        }

        if num_qubits > limits.max_qubits {
            return Err(
                PauliFrameError::ResourceLimitExceeded {
                    resource: ResourceKind::Qubits,
                    requested: num_qubits as u128,
                    limit: limits.max_qubits as u128,
                },
            );
        }

        Ok(Self {
            operator: PauliString::identity(num_qubits),
            revision: 0,
        })
    }

    /// Creates an identity frame using the subsystem's configured defaults.
    ///
    /// This compatibility constructor is intentionally conservative and
    /// delegates to `QecLimits::default()`.
    pub fn new(
        num_qubits: usize,
    ) -> Result<Self, PauliFrameError> {
        Self::new_with_limits(
            num_qubits,
            &QecLimits::default(),
        )
    }

    /// Creates a frame from an existing Pauli operator under an explicit
    /// resource policy.
    pub fn from_operator_with_limits(
        operator: PauliString,
        limits: &QecLimits,
    ) -> Result<Self, PauliFrameError> {
        limits
            .validate()
            .map_err(PauliFrameError::InvalidLimits)?;

        let num_qubits =
            operator.num_qubits();

        if num_qubits == 0 {
            return Err(
                PauliFrameError::ZeroQubits
            );
        }

        if num_qubits > limits.max_qubits {
            return Err(
                PauliFrameError::ResourceLimitExceeded {
                    resource: ResourceKind::Qubits,
                    requested: num_qubits as u128,
                    limit: limits.max_qubits as u128,
                },
            );
        }

        Ok(Self {
            operator,
            revision: 0,
        })
    }

    /// Creates a frame from an existing Pauli operator.
    pub fn from_operator(
        operator: PauliString,
    ) -> Result<Self, PauliFrameError> {
        Self::from_operator_with_limits(
            operator,
            &QecLimits::default(),
        )
    }

    // ------------------------------------------------------------------------
    // Basic state
    // ------------------------------------------------------------------------

    /// Number of physical qubits represented by the frame.
    #[must_use]
    pub const fn num_qubits(
        &self,
    ) -> usize {
        self.operator.num_qubits()
    }

    /// Current deterministic mutation revision.
    #[must_use]
    pub const fn revision(
        &self,
    ) -> u64 {
        self.revision
    }

    /// Returns the complete Pauli operator represented by the frame.
    #[must_use]
    pub fn operator(
        &self,
    ) -> &PauliString {
        &self.operator
    }

    /// Returns a cloned operator.
    #[must_use]
    pub fn operator_owned(
        &self,
    ) -> PauliString {
        self.operator.clone()
    }

    /// Returns true when no correction is pending.
    #[must_use]
    pub fn is_identity(
        &self,
    ) -> bool {
        self.operator.is_identity()
    }

    /// Returns the number of qubits carrying non-identity corrections.
    #[must_use]
    pub fn weight(
        &self,
    ) -> usize {
        self.operator.weight()
    }

    /// Returns the deterministic support of the frame.
    #[must_use]
    pub fn support(
        &self,
    ) -> Vec<QubitIndex> {
        self.operator.support()
    }

    /// Returns the Pauli currently tracked on one qubit.
    pub fn pauli_at(
        &self,
        qubit: usize,
    ) -> Result<Pauli, PauliFrameError> {
        self.check_qubit(qubit)?;

        self.operator
            .pauli_at(QubitIndex::new(qubit))
            .map_err(
                PauliFrameError::Stabilizer,
            )
    }

    // ------------------------------------------------------------------------
    // Mutation
    // ------------------------------------------------------------------------

    /// Sets the correction on one qubit.
    ///
    /// This replaces the existing Pauli on that qubit.
    pub fn set(
        &mut self,
        qubit: usize,
        pauli: Pauli,
    ) -> Result<(), PauliFrameError> {
        self.check_qubit(qubit)?;

        let current =
            self.operator.pauli_at(
                QubitIndex::new(qubit)
            ).map_err(
                PauliFrameError::Stabilizer
            )?;

        if current != pauli {
            self.operator
                .set_pauli(
                    QubitIndex::new(qubit),
                    pauli,
                )
                .map_err(
                    PauliFrameError::Stabilizer
                )?;

            self.bump_revision()?;
        }

        Ok(())
    }

    /// Accumulates one correction into the frame.
    ///
    /// This is an alias for `compose` intended for decoder integration.
    pub fn accumulate(
        &mut self,
        correction: &PauliString,
    ) -> Result<(), PauliFrameError> {
        self.compose(correction)
    }

    /// Composes a Pauli correction into the frame.
    ///
    /// Pauli multiplication is performed modulo global phase.
    pub fn compose(
        &mut self,
        correction: &PauliString,
    ) -> Result<(), PauliFrameError> {
        self.check_compatible(correction)?;

        if correction.is_identity() {
            return Ok(());
        }

        self.operator = self
            .operator
            .multiply(correction)
            .map_err(
                PauliFrameError::Stabilizer
            )?;

        self.bump_revision()?;

        Ok(())
    }

    /// Composes a correction while honoring cooperative cancellation.
    pub fn compose_with_cancellation(
        &mut self,
        correction: &PauliString,
        cancellation: &CancellationToken,
    ) -> QecResult<()> {
        cancellation.check()?;

        self.check_compatible(correction)
            .map_err(QecError::from)?;

        if correction.is_identity() {
            return Ok(());
        }

        /*
         * PauliString multiplication is currently a bounded linear operation.
         * Checking before and after it gives deterministic cooperative
         * cancellation semantics without exposing partially mutated state.
         */
        cancellation.check()?;

        let composed = self
            .operator
            .multiply(correction)
            .map_err(PauliFrameError::Stabilizer)
            .map_err(QecError::from)?;

        cancellation.check()?;

        self.operator = composed;

        self.bump_revision()
            .map_err(QecError::from)?;

        Ok(())
    }

    /// Composes many corrections deterministically.
    ///
    /// The caller's order is preserved. Because Pauli multiplication is
    /// represented modulo global phase, the resulting binary-symplectic
    /// operator is deterministic.
    pub fn compose_many(
        &mut self,
        corrections: &[PauliString],
    ) -> Result<(), PauliFrameError> {
        for correction in corrections {
            self.compose(correction)?;
        }

        Ok(())
    }

    /// Composes many corrections while honoring cancellation.
    pub fn compose_many_with_cancellation(
        &mut self,
        corrections: &[PauliString],
        cancellation: &CancellationToken,
    ) -> QecResult<()> {
        for correction in corrections {
            cancellation.check()?;

            self.compose_with_cancellation(
                correction,
                cancellation,
            )?;
        }

        Ok(())
    }

    /// Produces a new frame without modifying the original.
    pub fn composed(
        &self,
        correction: &PauliString,
    ) -> Result<Self, PauliFrameError> {
        let mut result =
            self.clone();

        result.compose(correction)?;

        Ok(result)
    }

    /// Clears all pending corrections.
    ///
    /// The physical-qubit count is preserved.
    pub fn clear(
        &mut self,
    ) -> Result<(), PauliFrameError> {
        if self.is_identity() {
            return Ok(());
        }

        self.operator =
            PauliString::identity(
                self.num_qubits()
            );

        self.bump_revision()?;

        Ok(())
    }

    /// Resets the frame to identity and starts a new deterministic revision.
    ///
    /// Unlike construction, this operation never changes the represented
    /// qubit count.
    pub fn reset(
        &mut self,
    ) -> Result<(), PauliFrameError> {
        self.clear()
    }

    // ------------------------------------------------------------------------
    // Logical/measurement interpretation
    // ------------------------------------------------------------------------

    /// Tests whether the frame commutes with an observable.
    pub fn commutes_with(
        &self,
        observable: &PauliString,
    ) -> Result<bool, PauliFrameError> {
        self.check_compatible(observable)?;

        self.operator
            .commutes_with(observable)
            .map_err(
                PauliFrameError::Stabilizer
            )
    }

    /// Tests whether the frame anticommutes with an observable.
    pub fn anticommutes_with(
        &self,
        observable: &PauliString,
    ) -> Result<bool, PauliFrameError> {
        self.check_compatible(observable)?;

        self.operator
            .anticommutes_with(observable)
            .map_err(
                PauliFrameError::Stabilizer
            )
    }

    /// Determines whether this frame flips the classical outcome of a
    /// measurement of `observable`.
    ///
    /// A Pauli frame changes a binary Pauli measurement result exactly when
    /// the frame anticommutes with the measured observable.
    pub fn measurement_flips(
        &self,
        observable: &PauliString,
    ) -> Result<bool, PauliFrameError> {
        self.anticommutes_with(observable)
    }

    /// Applies the frame's interpretation to a raw binary measurement result.
    ///
    /// This does NOT perform a quantum measurement and does NOT mutate the
    /// frame.
    pub fn correct_measurement(
        &self,
        raw_result: bool,
        observable: &PauliString,
    ) -> Result<bool, PauliFrameError> {
        let flip =
            self.measurement_flips(observable)?;

        Ok(if flip {
            !raw_result
        } else {
            raw_result
        })
    }

    /// Returns a structured measurement interpretation.
    pub fn interpret_measurement(
        &self,
        raw_result: bool,
        observable: &PauliString,
    ) -> Result<FrameMeasurement, PauliFrameError> {
        let flip =
            self.measurement_flips(observable)?;

        Ok(FrameMeasurement {
            raw_result,
            frame_flipped: flip,
            corrected_result: if flip {
                !raw_result
            } else {
                raw_result
            },
            frame_revision: self.revision,
        })
    }

    // ------------------------------------------------------------------------
    // Snapshots
    // ------------------------------------------------------------------------

    /// Creates an immutable frame snapshot.
    #[must_use]
    pub fn snapshot(
        &self,
    ) -> PauliFrameSnapshot {
        PauliFrameSnapshot {
            format_version:
                PAULI_FRAME_FORMAT_VERSION,
            num_qubits:
                self.num_qubits(),
            operator:
                self.operator.clone(),
            revision:
                self.revision,
        }
    }

    /// Restores a frame from a snapshot under a resource policy.
    pub fn from_snapshot(
        snapshot: PauliFrameSnapshot,
        limits: &QecLimits,
    ) -> Result<Self, PauliFrameError> {
        if snapshot.format_version
            != PAULI_FRAME_FORMAT_VERSION
        {
            return Err(
                PauliFrameError::UnsupportedFormatVersion {
                    version:
                        snapshot.format_version,
                },
            );
        }

        if snapshot.operator.num_qubits()
            != snapshot.num_qubits
        {
            return Err(
                PauliFrameError::SnapshotDimensionMismatch {
                    declared:
                        snapshot.num_qubits,
                    actual:
                        snapshot.operator.num_qubits(),
                },
            );
        }

        let mut frame =
            Self::from_operator_with_limits(
                snapshot.operator,
                limits,
            )?;

        frame.revision =
            snapshot.revision;

        Ok(frame)
    }

    /// Creates a checkpoint containing the complete frame state.
    #[must_use]
    pub fn checkpoint(
        &self,
    ) -> PauliFrameCheckpoint {
        let snapshot =
            self.snapshot();

        let integrity =
            snapshot.integrity_hash();

        PauliFrameCheckpoint {
            schema_version:
                PAULI_FRAME_CHECKPOINT_VERSION,
            snapshot,
            integrity,
        }
    }

    /// Restores a frame from a checkpoint.
    pub fn restore_checkpoint(
        checkpoint: PauliFrameCheckpoint,
        limits: &QecLimits,
    ) -> Result<Self, PauliFrameError> {
        if checkpoint.schema_version
            != PAULI_FRAME_CHECKPOINT_VERSION
        {
            return Err(
                PauliFrameError::UnsupportedCheckpointVersion {
                    version:
                        checkpoint.schema_version,
                },
            );
        }

        let expected =
            checkpoint.snapshot.integrity_hash();

        if expected != checkpoint.integrity {
            return Err(
                PauliFrameError::CheckpointIntegrityFailure
            );
        }

        Self::from_snapshot(
            checkpoint.snapshot,
            limits,
        )
    }

    // ------------------------------------------------------------------------
    // Validation
    // ------------------------------------------------------------------------

    fn check_qubit(
        &self,
        qubit: usize,
    ) -> Result<(), PauliFrameError> {
        if qubit >= self.num_qubits() {
            return Err(
                PauliFrameError::QubitOutOfRange {
                    qubit,
                    num_qubits:
                        self.num_qubits(),
                },
            );
        }

        Ok(())
    }

    fn check_compatible(
        &self,
        other: &PauliString,
    ) -> Result<(), PauliFrameError> {
        if other.num_qubits()
            != self.num_qubits()
        {
            return Err(
                PauliFrameError::QubitCountMismatch {
                    expected:
                        self.num_qubits(),
                    actual:
                        other.num_qubits(),
                },
            );
        }

        Ok(())
    }

    fn bump_revision(
        &mut self,
    ) -> Result<(), PauliFrameError> {
        self.revision =
            self.revision
                .checked_add(1)
                .ok_or(
                    PauliFrameError::RevisionOverflow
                )?;

        Ok(())
    }
}

// ============================================================================
// Measurement result
// ============================================================================

/// Result of interpreting a classical measurement through a Pauli frame.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FrameMeasurement {
    /// Raw result returned by the measurement backend.
    pub raw_result: bool,

    /// Whether the Pauli frame changes the classical result.
    pub frame_flipped: bool,

    /// Result after frame interpretation.
    pub corrected_result: bool,

    /// Frame revision used for the interpretation.
    pub frame_revision: u64,
}

// ============================================================================
// Snapshot
// ============================================================================

/// Immutable, deterministic representation of a Pauli frame.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PauliFrameSnapshot {
    pub format_version: u32,
    pub num_qubits: usize,
    pub operator: PauliString,
    pub revision: u64,
}

impl PauliFrameSnapshot {
    /// Computes a deterministic integrity value.
    ///
    /// This is intended to detect accidental/corrupt checkpoint state.
    /// Cryptographic checkpoint authentication remains the responsibility of
    /// the higher-level checkpoint subsystem.
    #[must_use]
    pub fn integrity_hash(
        &self,
    ) -> u64 {
        let mut hash =
            0xcbf29ce484222325u64;

        fn mix(
            hash: &mut u64,
            byte: u8,
        ) {
            *hash ^= u64::from(byte);
            *hash =
                hash.wrapping_mul(
                    0x100000001b3
                );
        }

        for byte in
            self.format_version
                .to_le_bytes()
        {
            mix(&mut hash, byte);
        }

        for byte in
            (self.num_qubits as u64)
                .to_le_bytes()
        {
            mix(&mut hash, byte);
        }

        for byte in
            self.revision.to_le_bytes()
        {
            mix(&mut hash, byte);
        }

        for pauli in
            self.operator.to_paulis()
        {
            mix(
                &mut hash,
                match pauli {
                    Pauli::I => 0,
                    Pauli::X => 1,
                    Pauli::Y => 2,
                    Pauli::Z => 3,
                },
            );
        }

        hash
    }
}

// ============================================================================
// Checkpoint
// ============================================================================

/// Serializable-independent checkpoint representation for the frame.
///
/// The higher-level checkpoint module can embed this structure into its
/// authenticated checkpoint envelope.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PauliFrameCheckpoint {
    pub schema_version: u32,
    pub snapshot: PauliFrameSnapshot,
    pub integrity: u64,
}

// ============================================================================
// Errors
// ============================================================================

/// Errors produced by Pauli-frame operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PauliFrameError {
    ZeroQubits,

    ResourceLimitExceeded {
        resource: ResourceKind,
        requested: u128,
        limit: u128,
    },

    InvalidLimits(LimitError),

    QubitOutOfRange {
        qubit: usize,
        num_qubits: usize,
    },

    QubitCountMismatch {
        expected: usize,
        actual: usize,
    },

    Stabilizer(StabilizerError),

    RevisionOverflow,

    UnsupportedFormatVersion {
        version: u32,
    },

    UnsupportedCheckpointVersion {
        version: u32,
    },

    SnapshotDimensionMismatch {
        declared: usize,
        actual: usize,
    },

    CheckpointIntegrityFailure,
}

impl fmt::Display
    for PauliFrameError
{
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::ZeroQubits => {
                write!(
                    f,
                    "a Pauli frame requires at least one qubit"
                )
            }

            Self::ResourceLimitExceeded {
                resource,
                requested,
                limit,
            } => {
                write!(
                    f,
                    "Pauli frame resource limit exceeded: \
                     {} requested {}, limit {}",
                    resource.as_str(),
                    requested,
                    limit
                )
            }

            Self::InvalidLimits(error) => {
                write!(
                    f,
                    "invalid QEC resource limits: {error}"
                )
            }

            Self::QubitOutOfRange {
                qubit,
                num_qubits,
            } => {
                write!(
                    f,
                    "qubit {qubit} is outside a \
                     {num_qubits}-qubit Pauli frame"
                )
            }

            Self::QubitCountMismatch {
                expected,
                actual,
            } => {
                write!(
                    f,
                    "Pauli frame requires {expected} qubits, \
                     got {actual}"
                )
            }

            Self::Stabilizer(error) => {
                write!(
                    f,
                    "stabilizer error: {error}"
                )
            }

            Self::RevisionOverflow => {
                write!(
                    f,
                    "Pauli-frame revision counter overflowed"
                )
            }

            Self::UnsupportedFormatVersion {
                version,
            } => {
                write!(
                    f,
                    "unsupported Pauli-frame format version {version}"
                )
            }

            Self::UnsupportedCheckpointVersion {
                version,
            } => {
                write!(
                    f,
                    "unsupported Pauli-frame checkpoint version {version}"
                )
            }

            Self::SnapshotDimensionMismatch {
                declared,
                actual,
            } => {
                write!(
                    f,
                    "Pauli-frame snapshot declares {declared} qubits \
                     but contains {actual}"
                )
            }

            Self::CheckpointIntegrityFailure => {
                write!(
                    f,
                    "Pauli-frame checkpoint integrity verification failed"
                )
            }
        }
    }
}

impl std::error::Error
    for PauliFrameError
{
}

// ============================================================================
// Canonical QecError integration
// ============================================================================

impl From<PauliFrameError>
    for QecError
{
    fn from(
        error: PauliFrameError,
    ) -> Self {
        match error {
            PauliFrameError::ZeroQubits
            | PauliFrameError::QubitOutOfRange { .. }
            | PauliFrameError::QubitCountMismatch { .. }
            | PauliFrameError::SnapshotDimensionMismatch { .. } => {
                QecError::InvalidInput {
                    message:
                        error.to_string(),
                }
            }

            PauliFrameError::ResourceLimitExceeded {
                resource,
                requested,
                limit,
            } => {
                QecError::ResourceLimitExceeded {
                    resource,
                    requested,
                    limit,
                    message:
                        error.to_string(),
                }
            }

            PauliFrameError::InvalidLimits(
                limit_error,
            ) => {
                QecError::InvalidInput {
                    message:
                        limit_error.to_string(),
                }
            }

            PauliFrameError::Stabilizer(
                stabilizer_error,
            ) => {
                QecError::InvalidStabilizer {
                    message:
                        stabilizer_error.to_string(),
                }
            }

            PauliFrameError::RevisionOverflow => {
                QecError::InternalInvariantViolation {
                    invariant:
                        "PauliFrame revision must not overflow",
                    message:
                        error.to_string(),
                }
            }

            PauliFrameError::UnsupportedFormatVersion {
                version,
            } => {
                QecError::UnsupportedConfiguration {
                    feature:
                        "pauli_frame_format".to_owned(),
                    message:
                        format!(
                            "unsupported Pauli-frame format version {version}"
                        ),
                }
            }

            PauliFrameError::UnsupportedCheckpointVersion {
                version,
            } => {
                QecError::UnsupportedConfiguration {
                    feature:
                        "pauli_frame_checkpoint".to_owned(),
                    message:
                        format!(
                            "unsupported Pauli-frame checkpoint version {version}"
                        ),
                }
            }

            PauliFrameError::CheckpointIntegrityFailure => {
                QecError::InvalidInput {
                    message:
                        "Pauli-frame checkpoint integrity verification failed"
                            .to_owned(),
                }
            }
        }
    }
}

// ============================================================================
// Display
// ============================================================================

impl fmt::Display
    for PauliFrame
{
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            f,
            "{}",
            self.operator
        )
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn limits() -> QecLimits {
        QecLimits::default()
    }

    #[test]
    fn identity_frame_is_empty() {
        let frame =
            PauliFrame::new(3)
                .unwrap();

        assert!(
            frame.is_identity()
        );

        assert_eq!(
            frame.weight(),
            0
        );

        assert_eq!(
            frame.revision(),
            0
        );
    }

    #[test]
    fn set_tracks_pauli() {
        let mut frame =
            PauliFrame::new(3)
                .unwrap();

        frame
            .set(1, Pauli::X)
            .unwrap();

        assert_eq!(
            frame.pauli_at(1)
                .unwrap(),
            Pauli::X
        );

        assert_eq!(
            frame.weight(),
            1
        );

        assert_eq!(
            frame.revision(),
            1
        );
    }

    #[test]
    fn setting_same_pauli_is_idempotent() {
        let mut frame =
            PauliFrame::new(2)
                .unwrap();

        frame
            .set(0, Pauli::X)
            .unwrap();

        let revision =
            frame.revision();

        frame
            .set(0, Pauli::X)
            .unwrap();

        assert_eq!(
            frame.revision(),
            revision
        );
    }

    #[test]
    fn setting_identity_clears_qubit() {
        let mut frame =
            PauliFrame::new(2)
                .unwrap();

        frame
            .set(0, Pauli::Y)
            .unwrap();

        frame
            .set(0, Pauli::I)
            .unwrap();

        assert_eq!(
            frame.pauli_at(0)
                .unwrap(),
            Pauli::I
        );

        assert!(
            frame.is_identity()
        );
    }

    #[test]
    fn composition_is_modulo_global_phase() {
        let mut frame =
            PauliFrame::new(2)
                .unwrap();

        frame
            .set(0, Pauli::X)
            .unwrap();

        frame
            .set(1, Pauli::Z)
            .unwrap();

        let correction =
            PauliString::from_paulis(
                &[Pauli::X, Pauli::Z],
            );

        frame
            .compose(&correction)
            .unwrap();

        assert!(
            frame.is_identity()
        );
    }

    #[test]
    fn x_composed_with_z_becomes_y() {
        let mut frame =
            PauliFrame::new(1)
                .unwrap();

        frame
            .set(0, Pauli::X)
            .unwrap();

        let correction =
            PauliString::from_paulis(
                &[Pauli::Z],
            );

        frame
            .compose(&correction)
            .unwrap();

        assert_eq!(
            frame.pauli_at(0)
                .unwrap(),
            Pauli::Y
        );
    }

    #[test]
    fn composed_does_not_mutate_original() {
        let mut frame =
            PauliFrame::new(1)
                .unwrap();

        frame
            .set(0, Pauli::X)
            .unwrap();

        let correction =
            PauliString::from_paulis(
                &[Pauli::Z],
            );

        let result =
            frame
                .composed(&correction)
                .unwrap();

        assert_eq!(
            frame.pauli_at(0)
                .unwrap(),
            Pauli::X
        );

        assert_eq!(
            result.pauli_at(0)
                .unwrap(),
            Pauli::Y
        );
    }

    #[test]
    fn clear_preserves_qubit_count() {
        let mut frame =
            PauliFrame::new(4)
                .unwrap();

        frame
            .set(0, Pauli::X)
            .unwrap();

        frame
            .set(2, Pauli::Y)
            .unwrap();

        frame
            .clear()
            .unwrap();

        assert!(
            frame.is_identity()
        );

        assert_eq!(
            frame.num_qubits(),
            4
        );
    }

    #[test]
    fn support_is_deterministic() {
        let mut frame =
            PauliFrame::new(4)
                .unwrap();

        frame
            .set(3, Pauli::Y)
            .unwrap();

        frame
            .set(1, Pauli::X)
            .unwrap();

        assert_eq!(
            frame.support(),
            vec![
                QubitIndex::new(1),
                QubitIndex::new(3),
            ]
        );
    }

    #[test]
    fn measurement_flips_when_frame_anticommutes() {
        let mut frame =
            PauliFrame::new(1)
                .unwrap();

        frame
            .set(0, Pauli::X)
            .unwrap();

        let observable =
            PauliString::from_paulis(
                &[Pauli::Z],
            );

        assert!(
            frame
                .measurement_flips(
                    &observable
                )
                .unwrap()
        );

        assert!(
            frame
                .correct_measurement(
                    false,
                    &observable,
                )
                .unwrap()
        );
    }

    #[test]
    fn measurement_does_not_flip_when_frame_commutes() {
        let mut frame =
            PauliFrame::new(1)
                .unwrap();

        frame
            .set(0, Pauli::X)
            .unwrap();

        let observable =
            PauliString::from_paulis(
                &[Pauli::X],
            );

        assert!(
            !frame
                .measurement_flips(
                    &observable
                )
                .unwrap()
        );

        assert!(
            !frame
                .correct_measurement(
                    false,
                    &observable,
                )
                .unwrap()
        );
    }

    #[test]
    fn snapshot_round_trip() {
        let mut frame =
            PauliFrame::new(3)
                .unwrap();

        frame
            .set(0, Pauli::X)
            .unwrap();

        frame
            .set(2, Pauli::Z)
            .unwrap();

        let snapshot =
            frame.snapshot();

        let restored =
            PauliFrame::from_snapshot(
                snapshot,
                &limits(),
            )
            .unwrap();

        assert_eq!(
            restored,
            frame
        );
    }

    #[test]
    fn checkpoint_round_trip() {
        let mut frame =
            PauliFrame::new(3)
                .unwrap();

        frame
            .set(0, Pauli::Y)
            .unwrap();

        let checkpoint =
            frame.checkpoint();

        let restored =
            PauliFrame::restore_checkpoint(
                checkpoint,
                &limits(),
            )
            .unwrap();

        assert_eq!(
            restored,
            frame
        );
    }

    #[test]
    fn corrupted_checkpoint_is_rejected() {
        let frame =
            PauliFrame::new(2)
                .unwrap();

        let mut checkpoint =
            frame.checkpoint();

        checkpoint.integrity ^=
            1;

        assert!(matches!(
            PauliFrame::restore_checkpoint(
                checkpoint,
                &limits(),
            ),
            Err(
                PauliFrameError::
                    CheckpointIntegrityFailure
            )
        ));
    }

    #[test]
    fn dimension_mismatch_is_rejected() {
        let mut frame =
            PauliFrame::new(2)
                .unwrap();

        let correction =
            PauliString::identity(3);

        assert!(matches!(
            frame.compose(
                &correction
            ),
            Err(
                PauliFrameError::
                    QubitCountMismatch {
                        expected: 2,
                        actual: 3,
                    }
            )
        ));
    }

    #[test]
    fn out_of_range_is_rejected() {
        let mut frame =
            PauliFrame::new(2)
                .unwrap();

        assert!(matches!(
            frame.set(
                2,
                Pauli::X,
            ),
            Err(
                PauliFrameError::
                    QubitOutOfRange { .. }
            )
        ));
    }

    #[test]
    fn explicit_limits_are_used() {
        let mut configured =
            QecLimits::default();

        configured.max_qubits = 4;

        assert!(
            PauliFrame::new_with_limits(
                4,
                &configured,
            )
            .is_ok()
        );

        assert!(matches!(
            PauliFrame::new_with_limits(
                5,
                &configured,
            ),
            Err(
                PauliFrameError::
                    ResourceLimitExceeded {
                        resource:
                            ResourceKind::Qubits,
                        ..
                    }
            )
        ));
    }
}