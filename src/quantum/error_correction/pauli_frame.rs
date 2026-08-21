//! Production Pauli-frame management for Zamani Quantum Error Correction.
//!
//! # Ownership
//!
//! This module owns classical Pauli-frame state.
//!
//! A Pauli frame records a pending Pauli correction without physically
//! applying that correction to a quantum device. It is therefore suitable
//! for decoder output, measurement interpretation, logical classification,
//! replay and checkpoint integration.
//!
//! This module owns:
//!
//! - Pauli-frame state;
//! - frame composition;
//! - frame accumulation;
//! - frame reset/clear;
//! - frame revision tracking;
//! - single-qubit frame access;
//! - measurement-result interpretation;
//! - immutable frame snapshots;
//! - checkpoint state representation;
//! - frame integrity validation;
//! - deterministic frame operations.
//!
//! This module does NOT own:
//!
//! - Pauli algebra itself;
//! - stabilizer algebra;
//! - logical-equivalence mathematics;
//! - decoder algorithms;
//! - surface-code topology;
//! - QPU execution;
//! - QPU credentials;
//! - network access;
//! - persistent checkpoint storage;
//! - cache storage;
//! - telemetry transport;
//! - resource-policy definition.
//!
//! Those responsibilities remain in their respective modules.
//!
//! # Architectural position
//!
//! ```text
//! Decoder
//!    │
//!    │ PauliString correction
//!    ▼
//! PauliFrame
//!    │
//!    ├───────────────┐
//!    │               │
//!    ▼               ▼
//! measurement     logical-equivalence
//! interpretation       layer
//!    │               │
//!    └───────┬───────┘
//!            ▼
//!       LogicalOutcome
//!
//! PauliFrame
//!    │
//!    ├── snapshot ──► checkpoint.rs
//!    │
//!    └── replay ────► replay.rs
//! ```
//!
//! # Representation
//!
//! The frame delegates all Pauli algebra to `stabilizer.rs`.
//!
//! A frame represents:
//!
//! ```text
//! P = P_0 ⊗ P_1 ⊗ ... ⊗ P_(n-1)
//! ```
//!
//! where every `P_i` is `I`, `X`, `Y`, or `Z`.
//!
//! Global Pauli phase is intentionally discarded because the canonical
//! `PauliString` representation in `stabilizer.rs` is binary-symplectic and
//! phase-free.
//!
//! # Critical invariants
//!
//! 1. A frame always represents a non-zero number of qubits.
//! 2. A correction may only be composed when its dimension matches the frame.
//! 3. Resource policy comes exclusively from `QecLimits`.
//! 4. Measurement interpretation never mutates the frame.
//! 5. A failed mutation does not leave partially updated state.
//! 6. Revision numbers increase only after a successful state mutation.
//! 7. Identity operations do not create artificial revisions.
//! 8. Cancellation is checked before and after expensive operations.
//! 9. Snapshots contain enough information to reconstruct the frame.
//! 10. Invalid snapshots are rejected.
//! 11. Checkpoint versions are explicitly validated.
//! 12. No physical quantum operation occurs in this module.
//!
//! # Integration contract
//!
//! ```text
//! limits.rs
//!      │
//!      ▼
//! PauliFrame
//!      │
//!      ├── decoder.rs
//!      ├── decoder_result.rs
//!      ├── logical_equivalence.rs
//!      ├── checkpoint.rs
//!      ├── replay.rs
//!      └── QPU / measurement integration
//! ```
//!
//! `stabilizer.rs` remains the sole owner of `Pauli` and `PauliString`
//! algebra.
//!
//! `limits.rs` remains the sole source of declarative resource policy.
//!
//! `errors.rs` remains the canonical public QEC error boundary.
//!
//! `checkpoint.rs` may persist `PauliFrameCheckpoint`, but persistence and
//! cryptographic integrity are not owned by this module.
//!
//! `logical_equivalence.rs` may inspect the frame's operator but must not
//! mutate it.
//!
//! # Rust compatibility
//!
//! This implementation targets Rust 1.97.1 and uses stable standard-library
//! facilities.

use core::fmt;

use super::cancellation::CancellationToken;
use super::errors::{QecError, QecResult, ResourceKind};
use super::limits::QecLimits;
use super::stabilizer::{Pauli, PauliString, QubitIndex, StabilizerError};

// ============================================================================
// Versions
// ============================================================================

/// Current in-memory Pauli-frame representation version.
pub const PAULI_FRAME_FORMAT_VERSION: u32 = 2;

/// Current Pauli-frame checkpoint schema version.
pub const PAULI_FRAME_CHECKPOINT_VERSION: u32 = 2;

// ============================================================================
// PauliFrame
// ============================================================================

/// Classical Pauli correction frame.
///
/// The frame contains no physical quantum state. It is a classical
/// representation of a correction that can be interpreted by a measurement
/// or logical layer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PauliFrame {
    operator: PauliString,

    /// Number of successful state mutations.
    ///
    /// Identity operations and failed operations do not increment this value.
    revision: u64,
}

impl PauliFrame {
    // ========================================================================
    // Construction
    // ========================================================================

    /// Creates an identity Pauli frame.
    pub fn new(num_qubits: usize) -> Result<Self, PauliFrameError> {
        Self::new_with_limits(num_qubits, &QecLimits::default())
    }

    /// Creates an identity Pauli frame after resource-policy validation.
    pub fn new_with_limits(
        num_qubits: usize,
        limits: &QecLimits,
    ) -> Result<Self, PauliFrameError> {
        validate_limits(limits)?;

        validate_num_qubits(num_qubits, limits)?;

        Ok(Self {
            operator: PauliString::identity(num_qubits),
            revision: 0,
        })
    }

    /// Creates a frame from an existing Pauli operator.
    pub fn from_operator(
        operator: PauliString,
    ) -> Result<Self, PauliFrameError> {
        Self::from_operator_with_limits(operator, &QecLimits::default())
    }

    /// Creates a frame from an existing Pauli operator using explicit
    /// resource policy.
    pub fn from_operator_with_limits(
        operator: PauliString,
        limits: &QecLimits,
    ) -> Result<Self, PauliFrameError> {
        validate_limits(limits)?;

        validate_num_qubits(operator.num_qubits(), limits)?;

        Ok(Self {
            operator,
            revision: 0,
        })
    }

    // ========================================================================
    // State inspection
    // ========================================================================

    /// Returns the number of physical qubits represented by the frame.
    #[must_use]
    pub const fn num_qubits(&self) -> usize {
        self.operator.num_qubits()
    }

    /// Returns the current mutation revision.
    #[must_use]
    pub const fn revision(&self) -> u64 {
        self.revision
    }

    /// Returns the frame's Pauli operator by reference.
    #[must_use]
    pub fn operator(&self) -> &PauliString {
        &self.operator
    }

    /// Returns an owned copy of the frame operator.
    #[must_use]
    pub fn operator_owned(&self) -> PauliString {
        self.operator.clone()
    }

    /// Returns true when the frame contains no pending correction.
    #[must_use]
    pub fn is_identity(&self) -> bool {
        self.operator.is_identity()
    }

    /// Returns the number of non-identity qubits.
    #[must_use]
    pub fn weight(&self) -> usize {
        self.operator.weight()
    }

    /// Returns the deterministic support of the frame.
    #[must_use]
    pub fn support(&self) -> Vec<QubitIndex> {
        self.operator.support()
    }

    /// Returns the Pauli acting on a specific physical qubit.
    pub fn pauli_at(
        &self,
        qubit: usize,
    ) -> Result<Pauli, PauliFrameError> {
        self.check_qubit(qubit)?;

        self.operator
            .pauli_at(QubitIndex::new(qubit))
            .map_err(PauliFrameError::Stabilizer)
    }

    // ========================================================================
    // Mutation
    // ========================================================================

    /// Replaces the Pauli correction on one qubit.
    ///
    /// This is replacement semantics, not multiplication semantics.
    pub fn set(
        &mut self,
        qubit: usize,
        pauli: Pauli,
    ) -> Result<(), PauliFrameError> {
        self.check_qubit(qubit)?;

        let current = self
            .operator
            .pauli_at(QubitIndex::new(qubit))
            .map_err(PauliFrameError::Stabilizer)?;

        if current == pauli {
            return Ok(());
        }

        self.operator
            .set_pauli(QubitIndex::new(qubit), pauli)
            .map_err(PauliFrameError::Stabilizer)?;

        self.bump_revision()?;

        Ok(())
    }

    /// Accumulates a decoder correction into the frame.
    ///
    /// This uses Pauli multiplication modulo global phase.
    pub fn accumulate(
        &mut self,
        correction: &PauliString,
    ) -> Result<(), PauliFrameError> {
        self.compose(correction)
    }

    /// Composes a Pauli correction into the frame.
    ///
    /// The operation is transactional: the frame is only modified after the
    /// resulting PauliString has been successfully calculated.
    pub fn compose(
        &mut self,
        correction: &PauliString,
    ) -> Result<(), PauliFrameError> {
        self.check_compatible(correction)?;

        if correction.is_identity() {
            return Ok(());
        }

        let composed = self
            .operator
            .multiply(correction)
            .map_err(PauliFrameError::Stabilizer)?;

        self.operator = composed;

        self.bump_revision()?;

        Ok(())
    }

    /// Composes a correction while observing cooperative cancellation.
    ///
    /// The operation is transactional: cancellation occurring after the
    /// multiplication but before commit leaves the original frame unchanged.
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

    /// Composes multiple corrections in caller-specified deterministic order.
    pub fn compose_many(
        &mut self,
        corrections: &[PauliString],
    ) -> Result<(), PauliFrameError> {
        for correction in corrections {
            self.compose(correction)?;
        }

        Ok(())
    }

    /// Composes multiple corrections with cancellation support.
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

    /// Returns a new frame containing the composed correction.
    ///
    /// The current frame remains unchanged.
    pub fn composed(
        &self,
        correction: &PauliString,
    ) -> Result<Self, PauliFrameError> {
        let mut result = self.clone();

        result.compose(correction)?;

        Ok(result)
    }

    /// Clears all pending corrections while preserving qubit count.
    pub fn clear(&mut self) -> Result<(), PauliFrameError> {
        if self.is_identity() {
            return Ok(());
        }

        self.operator = PauliString::identity(self.num_qubits());

        self.bump_revision()?;

        Ok(())
    }

    /// Resets the frame to identity.
    ///
    /// The represented physical-qubit count never changes.
    pub fn reset(&mut self) -> Result<(), PauliFrameError> {
        self.clear()
    }

    // ========================================================================
    // Algebraic interpretation
    // ========================================================================

    /// Returns true when the frame commutes with an observable.
    pub fn commutes_with(
        &self,
        observable: &PauliString,
    ) -> Result<bool, PauliFrameError> {
        self.check_compatible(observable)?;

        self.operator
            .commutes_with(observable)
            .map_err(PauliFrameError::Stabilizer)
    }

    /// Returns true when the frame anticommutes with an observable.
    pub fn anticommutes_with(
        &self,
        observable: &PauliString,
    ) -> Result<bool, PauliFrameError> {
        self.check_compatible(observable)?;

        self.operator
            .anticommutes_with(observable)
            .map_err(PauliFrameError::Stabilizer)
    }

    /// Returns whether a Pauli measurement result is flipped by this frame.
    ///
    /// For a binary Pauli measurement, an anticommute relation flips the
    /// classical result.
    pub fn measurement_flips(
        &self,
        observable: &PauliString,
    ) -> Result<bool, PauliFrameError> {
        self.anticommutes_with(observable)
    }

    /// Interprets a raw binary measurement result.
    ///
    /// This operation is read-only and never changes the frame.
    pub fn correct_measurement(
        &self,
        raw_result: bool,
        observable: &PauliString,
    ) -> Result<bool, PauliFrameError> {
        let interpretation =
            self.interpret_measurement(
                raw_result,
                observable,
            )?;

        Ok(interpretation.corrected_result)
    }

    /// Produces a structured measurement interpretation.
    pub fn interpret_measurement(
        &self,
        raw_result: bool,
        observable: &PauliString,
    ) -> Result<FrameMeasurement, PauliFrameError> {
        let frame_flipped =
            self.measurement_flips(observable)?;

        let corrected_result =
            if frame_flipped {
                !raw_result
            } else {
                raw_result
            };

        Ok(FrameMeasurement {
            raw_result,
            frame_flipped,
            corrected_result,
            frame_revision: self.revision,
        })
    }

    // ========================================================================
    // Snapshots
    // ========================================================================

    /// Creates an immutable snapshot of the complete frame state.
    #[must_use]
    pub fn snapshot(&self) -> PauliFrameSnapshot {
        PauliFrameSnapshot {
            format_version: PAULI_FRAME_FORMAT_VERSION,
            num_qubits: self.num_qubits(),
            operator: self.operator.clone(),
            revision: self.revision,
        }
    }

    /// Restores a frame from an immutable snapshot.
    pub fn from_snapshot(
        snapshot: PauliFrameSnapshot,
        limits: &QecLimits,
    ) -> Result<Self, PauliFrameError> {
        if snapshot.format_version
            != PAULI_FRAME_FORMAT_VERSION
        {
            return Err(
                PauliFrameError::UnsupportedFormatVersion {
                    version: snapshot.format_version,
                },
            );
        }

        if snapshot.num_qubits == 0 {
            return Err(PauliFrameError::ZeroQubits);
        }

        let actual =
            snapshot.operator.num_qubits();

        if actual != snapshot.num_qubits {
            return Err(
                PauliFrameError::SnapshotDimensionMismatch {
                    declared: snapshot.num_qubits,
                    actual,
                },
            );
        }

        validate_num_qubits(
            snapshot.num_qubits,
            limits,
        )?;

        Ok(Self {
            operator: snapshot.operator,
            revision: snapshot.revision,
        })
    }

    // ========================================================================
    // Checkpoints
    // ========================================================================

    /// Produces a checkpoint payload.
    ///
    /// Persistence, authentication and storage remain the responsibility of
    /// `checkpoint.rs`.
    #[must_use]
    pub fn checkpoint(&self) -> PauliFrameCheckpoint {
        PauliFrameCheckpoint {
            schema_version:
                PAULI_FRAME_CHECKPOINT_VERSION,
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

    /// Restores a frame from checkpoint state.
    ///
    /// This validates the structural checkpoint contract but does not perform
    /// cryptographic authentication. Authenticated persistence belongs to
    /// `checkpoint.rs`.
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

        if checkpoint.format_version
            != PAULI_FRAME_FORMAT_VERSION
        {
            return Err(
                PauliFrameError::UnsupportedFormatVersion {
                    version:
                        checkpoint.format_version,
                },
            );
        }

        if checkpoint.num_qubits == 0 {
            return Err(PauliFrameError::ZeroQubits);
        }

        let actual =
            checkpoint.operator.num_qubits();

        if actual != checkpoint.num_qubits {
            return Err(
                PauliFrameError::CheckpointDimensionMismatch {
                    declared:
                        checkpoint.num_qubits,
                    actual,
                },
            );
        }

        validate_num_qubits(
            checkpoint.num_qubits,
            limits,
        )?;

        Ok(Self {
            operator:
                checkpoint.operator,
            revision:
                checkpoint.revision,
        })
    }

    // ========================================================================
    // Internal validation
    // ========================================================================

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
        correction: &PauliString,
    ) -> Result<(), PauliFrameError> {
        let expected =
            self.num_qubits();

        let actual =
            correction.num_qubits();

        if expected != actual {
            return Err(
                PauliFrameError::DimensionMismatch {
                    expected,
                    actual,
                },
            );
        }

        Ok(())
    }

    fn bump_revision(
        &mut self,
    ) -> Result<(), PauliFrameError> {
        self.revision = self
            .revision
            .checked_add(1)
            .ok_or(
                PauliFrameError::RevisionOverflow,
            )?;

        Ok(())
    }
}

// ============================================================================
// Immutable snapshot
// ============================================================================

/// Immutable representation of a Pauli-frame state.
///
/// Snapshots are intentionally separate from the mutable `PauliFrame`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PauliFrameSnapshot {
    /// In-memory frame format version.
    pub format_version: u32,

    /// Number of represented physical qubits.
    pub num_qubits: usize,

    /// Complete phase-free Pauli operator.
    pub operator: PauliString,

    /// Mutation revision at snapshot creation.
    pub revision: u64,
}

impl PauliFrameSnapshot {
    /// Validates the snapshot against the current frame format.
    pub fn validate(&self) -> Result<(), PauliFrameError> {
        if self.format_version
            != PAULI_FRAME_FORMAT_VERSION
        {
            return Err(
                PauliFrameError::UnsupportedFormatVersion {
                    version:
                        self.format_version,
                },
            );
        }

        if self.num_qubits == 0 {
            return Err(PauliFrameError::ZeroQubits);
        }

        let actual =
            self.operator.num_qubits();

        if actual != self.num_qubits {
            return Err(
                PauliFrameError::SnapshotDimensionMismatch {
                    declared:
                        self.num_qubits,
                    actual,
                },
            );
        }

        Ok(())
    }
}

// ============================================================================
// Checkpoint
// ============================================================================

/// Serializable-independent checkpoint state for `checkpoint.rs`.
///
/// This type contains state but deliberately does not implement storage,
/// hashing, encryption or persistence. Those operations belong to the
/// checkpoint subsystem.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PauliFrameCheckpoint {
    /// Persistent checkpoint schema version.
    pub schema_version: u32,

    /// In-memory representation version.
    pub format_version: u32,

    /// Number of represented physical qubits.
    pub num_qubits: usize,

    /// Complete phase-free Pauli operator.
    pub operator: PauliString,

    /// Frame mutation revision.
    pub revision: u64,
}

impl PauliFrameCheckpoint {
    /// Validates checkpoint structure without allocating or mutating state.
    pub fn validate(
        &self,
    ) -> Result<(), PauliFrameError> {
        if self.schema_version
            != PAULI_FRAME_CHECKPOINT_VERSION
        {
            return Err(
                PauliFrameError::UnsupportedCheckpointVersion {
                    version:
                        self.schema_version,
                },
            );
        }

        if self.format_version
            != PAULI_FRAME_FORMAT_VERSION
        {
            return Err(
                PauliFrameError::UnsupportedFormatVersion {
                    version:
                        self.format_version,
                },
            );
        }

        if self.num_qubits == 0 {
            return Err(PauliFrameError::ZeroQubits);
        }

        let actual =
            self.operator.num_qubits();

        if actual != self.num_qubits {
            return Err(
                PauliFrameError::CheckpointDimensionMismatch {
                    declared:
                        self.num_qubits,
                    actual,
                },
            );
        }

        Ok(())
    }
}

// ============================================================================
// Measurement interpretation
// ============================================================================

/// Immutable interpretation of one binary Pauli measurement.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FrameMeasurement {
    /// Raw result returned by the measurement layer.
    pub raw_result: bool,

    /// Whether the frame changes the classical interpretation.
    pub frame_flipped: bool,

    /// Result after frame interpretation.
    pub corrected_result: bool,

    /// Frame revision used for the interpretation.
    pub frame_revision: u64,
}

// ============================================================================
// Errors
// ============================================================================

/// Errors specific to Pauli-frame operations.
///
/// These errors remain detailed at the local module boundary and can be
/// converted to the canonical `QecError` for higher-level APIs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PauliFrameError {
    /// The frame cannot represent zero qubits.
    ZeroQubits,

    /// A qubit index was outside the frame.
    QubitOutOfRange {
        qubit: usize,
        num_qubits: usize,
    },

    /// A correction and frame have different dimensions.
    DimensionMismatch {
        expected: usize,
        actual: usize,
    },

    /// Snapshot dimensions disagree.
    SnapshotDimensionMismatch {
        declared: usize,
        actual: usize,
    },

    /// Checkpoint dimensions disagree.
    CheckpointDimensionMismatch {
        declared: usize,
        actual: usize,
    },

    /// The supplied limits object is invalid.
    InvalidLimits {
        message: String,
    },

    /// The frame exceeded the configured qubit policy.
    ResourceLimitExceeded {
        resource: ResourceKind,
        requested: u128,
        limit: u128,
    },

    /// Stabilizer algebra rejected an operation.
    Stabilizer(StabilizerError),

    /// The frame representation version is unsupported.
    UnsupportedFormatVersion {
        version: u32,
    },

    /// The checkpoint schema version is unsupported.
    UnsupportedCheckpointVersion {
        version: u32,
    },

    /// Revision counter overflowed.
    RevisionOverflow,
}

impl fmt::Display for PauliFrameError {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::ZeroQubits => {
                write!(
                    f,
                    "Pauli frame must contain at least one qubit"
                )
            }

            Self::QubitOutOfRange {
                qubit,
                num_qubits,
            } => {
                write!(
                    f,
                    "qubit index {qubit} is outside frame of {num_qubits} qubits"
                )
            }

            Self::DimensionMismatch {
                expected,
                actual,
            } => {
                write!(
                    f,
                    "Pauli-frame dimension mismatch: expected {expected} qubits, got {actual}"
                )
            }

            Self::SnapshotDimensionMismatch {
                declared,
                actual,
            } => {
                write!(
                    f,
                    "Pauli-frame snapshot dimension mismatch: declared {declared}, actual {actual}"
                )
            }

            Self::CheckpointDimensionMismatch {
                declared,
                actual,
            } => {
                write!(
                    f,
                    "Pauli-frame checkpoint dimension mismatch: declared {declared}, actual {actual}"
                )
            }

            Self::InvalidLimits { message } => {
                write!(
                    f,
                    "invalid QEC limits: {message}"
                )
            }

            Self::ResourceLimitExceeded {
                resource,
                requested,
                limit,
            } => {
                write!(
                    f,
                    "Pauli-frame resource limit exceeded for {resource}: requested {requested}, limit {limit}"
                )
            }

            Self::Stabilizer(error) => {
                write!(
                    f,
                    "stabilizer operation failed: {error}"
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
                    "unsupported Pauli-frame checkpoint schema version {version}"
                )
            }

            Self::RevisionOverflow => {
                write!(
                    f,
                    "Pauli-frame revision counter overflow"
                )
            }
        }
    }
}

impl std::error::Error for PauliFrameError {}

impl From<PauliFrameError> for QecError {
    fn from(error: PauliFrameError) -> Self {
        match error {
            PauliFrameError::ZeroQubits => {
                QecError::invalid_input(
                    "Pauli frame must contain at least one qubit",
                )
            }

            PauliFrameError::QubitOutOfRange {
                qubit,
                num_qubits,
            } => {
                QecError::invalid_input(format!(
                    "qubit index {qubit} is outside frame of {num_qubits} qubits",
                ))
            }

            PauliFrameError::DimensionMismatch {
                expected,
                actual,
            } => {
                QecError::invalid_input(format!(
                    "Pauli-frame dimension mismatch: expected {expected}, got {actual}",
                ))
            }

            PauliFrameError::SnapshotDimensionMismatch {
                declared,
                actual,
            } => {
                QecError::CheckpointInvalid {
                    message: format!(
                        "snapshot dimension mismatch: declared {declared}, actual {actual}",
                    ),
                }
            }

            PauliFrameError::CheckpointDimensionMismatch {
                declared,
                actual,
            } => {
                QecError::CheckpointInvalid {
                    message: format!(
                        "checkpoint dimension mismatch: declared {declared}, actual {actual}",
                    ),
                }
            }

            PauliFrameError::InvalidLimits {
                message,
            } => {
                QecError::invalid_input(format!(
                    "invalid QEC limits: {message}",
                ))
            }

            PauliFrameError::ResourceLimitExceeded {
                resource,
                requested,
                limit,
            } => {
                QecError::ResourceLimitExceeded {
                    resource,
                    requested,
                    current: 0,
                    limit,
                    message: format!(
                        "Pauli frame requested {requested} units of {resource}, exceeding limit {limit}",
                    ),
                }
            }

            PauliFrameError::Stabilizer(error) => {
                QecError::invalid_stabilizer(
                    error.to_string(),
                )
            }

            PauliFrameError::UnsupportedFormatVersion {
                version,
            } => {
                QecError::VersionMismatch {
                    component:
                        "pauli_frame".to_string(),
                    expected:
                        PAULI_FRAME_FORMAT_VERSION.to_string(),
                    actual:
                        version.to_string(),
                    message:
                        "unsupported Pauli-frame representation version"
                            .to_string(),
                }
            }

            PauliFrameError::UnsupportedCheckpointVersion {
                version,
            } => {
                QecError::VersionMismatch {
                    component:
                        "pauli_frame_checkpoint".to_string(),
                    expected:
                        PAULI_FRAME_CHECKPOINT_VERSION.to_string(),
                    actual:
                        version.to_string(),
                    message:
                        "unsupported Pauli-frame checkpoint schema version"
                            .to_string(),
                }
            }

            PauliFrameError::RevisionOverflow => {
                QecError::InternalInvariantViolation {
                    invariant:
                        "pauli_frame_revision".to_string(),
                    message:
                        "Pauli-frame revision counter overflowed"
                            .to_string(),
                }
            }
        }
    }
}

// ============================================================================
// Validation helpers
// ============================================================================

fn validate_limits(
    limits: &QecLimits,
) -> Result<(), PauliFrameError> {
    limits
        .validate()
        .map_err(|error| {
            PauliFrameError::InvalidLimits {
                message: format!("{error:?}"),
            }
        })
}

fn validate_num_qubits(
    num_qubits: usize,
    limits: &QecLimits,
) -> Result<(), PauliFrameError> {
    if num_qubits == 0 {
        return Err(PauliFrameError::ZeroQubits);
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

    Ok(())
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn frame() -> PauliFrame {
        PauliFrame::new(3).expect("valid frame")
    }

    fn x_on(
        num_qubits: usize,
        qubit: usize,
    ) -> PauliString {
        let mut operator =
            PauliString::identity(num_qubits);

        operator
            .set_pauli(
                QubitIndex::new(qubit),
                Pauli::X,
            )
            .expect("valid qubit");

        operator
    }

    fn z_on(
        num_qubits: usize,
        qubit: usize,
    ) -> PauliString {
        let mut operator =
            PauliString::identity(num_qubits);

        operator
            .set_pauli(
                QubitIndex::new(qubit),
                Pauli::Z,
            )
            .expect("valid qubit");

        operator
    }

    #[test]
    fn identity_frame_is_valid() {
        let value = frame();

        assert_eq!(value.num_qubits(), 3);
        assert!(value.is_identity());
        assert_eq!(value.weight(), 0);
        assert_eq!(value.revision(), 0);
    }

    #[test]
    fn set_replaces_single_qubit_pauli() {
        let mut value = frame();

        value
            .set(1, Pauli::X)
            .expect("valid set");

        assert_eq!(
            value.pauli_at(1).unwrap(),
            Pauli::X
        );

        value
            .set(1, Pauli::Z)
            .expect("valid replacement");

        assert_eq!(
            value.pauli_at(1).unwrap(),
            Pauli::Z
        );
    }

    #[test]
    fn setting_same_value_does_not_change_revision() {
        let mut value = frame();

        value
            .set(1, Pauli::X)
            .expect("valid set");

        let revision = value.revision();

        value
            .set(1, Pauli::X)
            .expect("same value is valid");

        assert_eq!(
            value.revision(),
            revision
        );
    }

    #[test]
    fn composition_is_modulo_phase() {
        let mut value = frame();

        value
            .compose(&x_on(3, 0))
            .expect("valid composition");

        value
            .compose(&x_on(3, 0))
            .expect("valid composition");

        assert!(value.is_identity());
        assert_eq!(value.revision(), 2);
    }

    #[test]
    fn accumulation_is_composition() {
        let mut value = frame();

        value
            .accumulate(&x_on(3, 2))
            .expect("valid accumulation");

        assert_eq!(
            value.pauli_at(2).unwrap(),
            Pauli::X
        );
    }

    #[test]
    fn incompatible_dimensions_are_rejected() {
        let mut value = frame();

        let correction =
            PauliString::identity(4);

        let result =
            value.compose(&correction);

        assert!(matches!(
            result,
            Err(
                PauliFrameError::DimensionMismatch {
                    expected: 3,
                    actual: 4
                }
            )
        ));
    }

    #[test]
    fn out_of_range_qubit_is_rejected() {
        let value = frame();

        let result = value.pauli_at(3);

        assert!(matches!(
            result,
            Err(
                PauliFrameError::QubitOutOfRange {
                    qubit: 3,
                    num_qubits: 3
                }
            )
        ));
    }

    #[test]
    fn clear_preserves_dimension() {
        let mut value = frame();

        value
            .compose(&x_on(3, 0))
            .expect("valid composition");

        value.clear().expect("clear succeeds");

        assert_eq!(value.num_qubits(), 3);
        assert!(value.is_identity());
    }

    #[test]
    fn reset_preserves_dimension() {
        let mut value = frame();

        value
            .compose(&x_on(3, 0))
            .expect("valid composition");

        value.reset().expect("reset succeeds");

        assert_eq!(value.num_qubits(), 3);
        assert!(value.is_identity());
    }

    #[test]
    fn measurement_flips_when_frame_anticommutes() {
        let mut value = frame();

        value
            .compose(&x_on(3, 0))
            .expect("valid composition");

        let observable =
            z_on(3, 0);

        assert!(
            value
                .measurement_flips(
                    &observable
                )
                .unwrap()
        );

        assert!(
            value
                .correct_measurement(
                    false,
                    &observable
                )
                .unwrap()
        );
    }

    #[test]
    fn measurement_does_not_mutate_frame() {
        let mut value = frame();

        value
            .compose(&x_on(3, 0))
            .expect("valid composition");

        let before = value.clone();

        let observable =
            z_on(3, 0);

        let _ = value
            .interpret_measurement(
                false,
                &observable,
            )
            .unwrap();

        assert_eq!(value, before);
    }

    #[test]
    fn snapshot_round_trip() {
        let mut value = frame();

        value
            .compose(&x_on(3, 0))
            .expect("valid composition");

        let snapshot =
            value.snapshot();

        let restored =
            PauliFrame::from_snapshot(
                snapshot,
                &QecLimits::default(),
            )
            .expect("snapshot restores");

        assert_eq!(restored, value);
    }

    #[test]
    fn checkpoint_round_trip() {
        let mut value = frame();

        value
            .compose(&x_on(3, 1))
            .expect("valid composition");

        let checkpoint =
            value.checkpoint();

        let restored =
            PauliFrame::restore_checkpoint(
                checkpoint,
                &QecLimits::default(),
            )
            .expect("checkpoint restores");

        assert_eq!(restored, value);
    }

    #[test]
    fn invalid_checkpoint_version_is_rejected() {
        let mut checkpoint =
            frame().checkpoint();

        checkpoint.schema_version += 1;

        assert!(matches!(
            PauliFrame::restore_checkpoint(
                checkpoint,
                &QecLimits::default(),
            ),
            Err(
                PauliFrameError::UnsupportedCheckpointVersion {
                    ..
                }
            )
        ));
    }

    #[test]
    fn invalid_snapshot_dimension_is_rejected() {
        let mut snapshot =
            frame().snapshot();

        snapshot.num_qubits = 4;

        assert!(matches!(
            PauliFrame::from_snapshot(
                snapshot,
                &QecLimits::default(),
            ),
            Err(
                PauliFrameError::SnapshotDimensionMismatch {
                    declared: 4,
                    actual: 3
                }
            )
        ));
    }

    #[test]
    fn compose_is_transactional_on_dimension_failure() {
        let mut value = frame();

        let before = value.clone();

        let result =
            value.compose(
                &PauliString::identity(4)
            );

        assert!(result.is_err());
        assert_eq!(value, before);
    }

    #[test]
    fn compose_many_preserves_order() {
        let mut value = frame();

        let first =
            x_on(3, 0);

        let second =
            z_on(3, 0);

        value
            .compose_many(&[
                first,
                second,
            ])
            .expect("composition succeeds");

        assert_eq!(
            value.pauli_at(0).unwrap(),
            Pauli::Y
        );
    }

    #[test]
    fn support_is_deterministic() {
        let mut value = frame();

        value
            .compose(&x_on(3, 2))
            .expect("valid");

        value
            .compose(&z_on(3, 0))
            .expect("valid");

        let support =
            value.support();

        assert_eq!(
            support,
            vec![
                QubitIndex::new(0),
                QubitIndex::new(2),
            ]
        );
    }

    #[test]
    fn canonical_error_conversion_works() {
        let error =
            PauliFrameError::DimensionMismatch {
                expected: 3,
                actual: 4,
            };

        let qec_error =
            QecError::from(error);

        assert_eq!(
            qec_error.kind(),
            super::super::errors::QecErrorKind::InvalidInput
        );
    }

    #[test]
    fn revision_overflow_is_detected() {
        let mut value = frame();

        value.revision =
            u64::MAX;

        let result =
            value.set(0, Pauli::X);

        assert!(matches!(
            result,
            Err(
                PauliFrameError::RevisionOverflow
            )
        ));
    }
}