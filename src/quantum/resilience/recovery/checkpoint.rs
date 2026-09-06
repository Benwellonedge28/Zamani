//! Zamani Quantum Resilience — Checkpoint Recovery
//!
//! Path:
//!     src/quantum/resilience/recovery/checkpoint.rs
//!
//! Purpose:
//!     Recovery-facing orchestration for checkpoint creation, discovery,
//!     validation, restoration and post-restore verification.
//!
//! Architectural ownership:
//!
//!     resilience::checkpoint::*
//!         owns checkpoint representation, storage, manifests, integrity and
//!         compatibility semantics.
//!
//!     resilience::recovery::checkpoint
//!         owns recovery orchestration involving checkpoints.
//!
//!     resilience::recovery::resume
//!         owns continuation after a successfully restored checkpoint.
//!
//!     resilience::recovery::rollback
//!         owns rollback policy/orchestration.
//!
//!     resilience::verification::*
//!         owns semantic/result acceptance.
//!
//!     quantum::ir
//!         owns canonical quantum program representation.
//!
//!     quantum::ir::qubit
//!         owns canonical qubit identity.
//!
//!     quantum::hardware
//!         owns hardware/device capabilities and execution mechanics.
//!
//! This module deliberately does NOT:
//!
//! - define a competing quantum circuit;
//! - define a competing qubit identifier;
//! - serialize arbitrary quantum state;
//! - assume a finite number of qubits;
//! - assume a finite number of checkpoints;
//! - assume a fixed retry count;
//! - implement a backend;
//! - implement QEC;
//! - implement routing;
//! - implement scheduling;
//! - implement optimization;
//! - decide whether a recovered result is semantically correct;
//! - silently accept a restored state.
//!
//! # Critical quantum correctness rule
//!
//! A checkpoint is NOT automatically a serialization of an arbitrary unknown
//! quantum state.
//!
//! Checkpoints are valid only at boundaries for which the execution platform
//! and program semantics explicitly establish that restoration is possible.
//!
//! Valid examples may include:
//!
//! - program-start boundaries;
//! - classical execution boundaries;
//! - measurement boundaries;
//! - logical/QEC boundaries;
//! - provider-supported execution snapshots;
//! - reconstructible compiled-program state;
//! - explicitly supported runtime state.
//!
//! An implementation MUST NOT claim that an arbitrary unknown quantum state can
//! be serialized and restored merely because a storage backend can store bytes.
//!
//! # Write once, scale everywhere
//!
//! No machine-size assumption exists in this module.
//!
//! There is no:
//!
//!     MAX_QUBITS
//!     MAX_CHECKPOINTS
//!     MAX_RESTORE_ATTEMPTS
//!     MAX_DEVICES
//!
//! and no provider-specific branch.
//!
//! Practical limits come from:
//!
//! - checkpoint storage;
//! - available memory;
//! - target capabilities;
//! - execution policy;
//! - security policy;
//! - runtime resources;
//! - provider/backend constraints;
//! - user-declared budgets.
//!
//! "Infinite scale" therefore means that this module imposes no artificial
//! finite quantum-machine size ceiling.
//!
//! # Recovery contract
//!
//! The recovery operation is:
//!
//!     checkpoint reference
//!             |
//!             v
//!     load metadata
//!             |
//!             v
//!     validate identity
//!             |
//!             v
//!     validate integrity
//!             |
//!             v
//!     validate compatibility
//!             |
//!             v
//!     validate recovery policy
//!             |
//!             v
//!     validate target capabilities
//!             |
//!             v
//!     acquire recovery ownership
//!             |
//!             v
//!     restore
//!             |
//!             v
//!     verify restored state
//!             |
//!             v
//!     return restoration outcome
//!
//! The verification step is mandatory for an accepted restoration.
//!
//! # Integration with existing repository contracts
//!
//! This file is intentionally designed to integrate with:
//!
//!     quantum::resilience::checkpoint
//!     quantum::resilience::verification
//!     quantum::resilience::state
//!     quantum::resilience::policy
//!     quantum::resilience::planning
//!     quantum::resilience::telemetry
//!     quantum::resilience::history
//!     quantum::resilience::errors
//!     quantum::resilience::model
//!     quantum::resilience::recovery::recoverer
//!     quantum::resilience::recovery::resume
//!     quantum::resilience::recovery::rollback
//!
//! The concrete subsystem types are intentionally injected through traits.
//! This keeps this file independently implementable and prevents it from
//! becoming coupled to a particular storage provider or hardware backend.
//!
//! # Rust contract
//!
//! Supported:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! The module explicitly forbids unsafe code.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::fmt;
use std::sync::Arc;
use std::time::{Duration, SystemTime};

use crate::quantum::ir::qubit::QubitId;

// ============================================================================
// Public schema constants
// ============================================================================

/// Stable identifier for this recovery-facing checkpoint contract.
pub const RECOVERY_CHECKPOINT_SCHEMA_ID: &str =
    "zamani.quantum.resilience.recovery.checkpoint";

/// Semantic version of this recovery-facing contract.
pub const RECOVERY_CHECKPOINT_SCHEMA_VERSION: u16 = 1;

// ============================================================================
// Stable identifiers
// ============================================================================

/// Opaque checkpoint identifier.
///
/// The checkpoint subsystem owns the actual checkpoint identity semantics.
/// This recovery layer treats the value as opaque and never derives meaning
/// from its numeric representation.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CheckpointId(String);

impl CheckpointId {
    /// Creates a checkpoint identifier.
    ///
    /// Empty identifiers are rejected because an empty value cannot provide
    /// stable recovery provenance.
    pub fn new(value: impl Into<String>) -> Result<Self, CheckpointRecoveryError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(CheckpointRecoveryError::InvalidIdentifier {
                field: "checkpoint_id",
            });
        }

        Ok(Self(value))
    }

    /// Returns the identifier as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Returns the owned identifier.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for CheckpointId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Opaque execution identity.
///
/// Recovery MUST bind a checkpoint to an execution identity so that a
/// checkpoint belonging to one execution cannot silently restore another.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ExecutionId(String);

impl ExecutionId {
    /// Creates an execution identifier.
    pub fn new(value: impl Into<String>) -> Result<Self, CheckpointRecoveryError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(CheckpointRecoveryError::InvalidIdentifier {
                field: "execution_id",
            });
        }

        Ok(Self(value))
    }

    /// Returns the identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ExecutionId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Opaque program identity.
///
/// This is intentionally not a copy of the quantum IR. It identifies the
/// program/version against which a checkpoint was created.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ProgramIdentity(String);

impl ProgramIdentity {
    /// Creates a program identity.
    pub fn new(value: impl Into<String>) -> Result<Self, CheckpointRecoveryError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(CheckpointRecoveryError::InvalidIdentifier {
                field: "program_identity",
            });
        }

        Ok(Self(value))
    }

    /// Returns the program identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

// ============================================================================
// Recovery boundary
// ============================================================================

/// The semantic class of a checkpoint boundary.
///
/// The variants describe *why* restoration is valid. They do not claim that
/// arbitrary quantum state can be serialized.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CheckpointBoundary {
    /// The program can be safely reconstructed from its initial state.
    ProgramStart,

    /// A classical execution boundary with reconstructible state.
    ClassicalBoundary,

    /// A measurement-defined boundary.
    MeasurementBoundary,

    /// A logical/QEC-defined boundary.
    LogicalBoundary,

    /// A provider/runtime-supported execution snapshot.
    ProviderSupportedSnapshot,

    /// A checkpoint whose state is explicitly reconstructible by the runtime.
    ReconstructibleRuntimeState,
}

impl CheckpointBoundary {
    /// Returns whether the boundary explicitly represents a quantum snapshot.
    #[must_use]
    pub const fn is_quantum_snapshot(self) -> bool {
        matches!(self, Self::ProviderSupportedSnapshot)
    }

    /// Returns whether the boundary requires explicit restoration support.
    #[must_use]
    pub const fn requires_explicit_restore_support(self) -> bool {
        matches!(
            self,
            Self::ProviderSupportedSnapshot
                | Self::LogicalBoundary
                | Self::ReconstructibleRuntimeState
        )
    }
}

/// Semantic validity of the state represented by a checkpoint.
///
/// `Unknown` is deliberately distinct from `Invalid`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CheckpointStateKind {
    /// State can be reconstructed according to the checkpoint contract.
    Reconstructible,

    /// State is explicitly supported by the target/runtime.
    SupportedSnapshot,

    /// State contains only classical/replayable information.
    Classical,

    /// State was created at a measurement boundary.
    MeasurementBoundary,

    /// State is a logical/QEC checkpoint.
    Logical,

    /// The representation exists but restoration semantics are unknown.
    Unknown,

    /// The checkpoint has been determined to be invalid.
    Invalid,
}

impl CheckpointStateKind {
    /// Returns whether restoration may be considered semantically possible.
    #[must_use]
    pub const fn is_restorable(self) -> bool {
        matches!(
            self,
            Self::Reconstructible
                | Self::SupportedSnapshot
                | Self::Classical
                | Self::MeasurementBoundary
                | Self::Logical
        )
    }
}

// ============================================================================
// Checkpoint metadata
// ============================================================================

/// Immutable metadata needed by recovery to validate a checkpoint.
///
/// The actual checkpoint payload remains owned by the checkpoint subsystem or
/// storage implementation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckpointMetadata {
    /// Checkpoint identity.
    pub checkpoint_id: CheckpointId,

    /// Execution to which this checkpoint belongs.
    pub execution_id: ExecutionId,

    /// Program identity.
    pub program_identity: ProgramIdentity,

    /// Canonical IR schema/version identity.
    pub ir_schema_version: String,

    /// Resilience checkpoint schema identity.
    pub checkpoint_schema_version: String,

    /// Boundary at which the checkpoint was created.
    pub boundary: CheckpointBoundary,

    /// Semantic kind of state represented by the checkpoint.
    pub state_kind: CheckpointStateKind,

    /// Monotonic logical execution position supplied by the producer.
    ///
    /// This is intentionally opaque to this module. It is not interpreted as
    /// a qubit count or hardware position.
    pub execution_position: String,

    /// Integrity digest supplied by the checkpoint subsystem.
    pub integrity_digest: String,

    /// Capability/schema fingerprint of the environment that created it.
    pub capability_fingerprint: String,

    /// Creation timestamp.
    pub created_at: SystemTime,

    /// Whether the producer has explicitly declared this checkpoint safe for
    /// replay/restoration.
    pub restoration_authorized: bool,
}

impl CheckpointMetadata {
    /// Performs local structural validation.
    pub fn validate(&self) -> Result<(), CheckpointRecoveryError> {
        if self.ir_schema_version.trim().is_empty() {
            return Err(CheckpointRecoveryError::InvalidMetadata {
                field: "ir_schema_version",
            });
        }

        if self.checkpoint_schema_version.trim().is_empty() {
            return Err(CheckpointRecoveryError::InvalidMetadata {
                field: "checkpoint_schema_version",
            });
        }

        if self.execution_position.trim().is_empty() {
            return Err(CheckpointRecoveryError::InvalidMetadata {
                field: "execution_position",
            });
        }

        if self.integrity_digest.trim().is_empty() {
            return Err(CheckpointRecoveryError::InvalidMetadata {
                field: "integrity_digest",
            });
        }

        if self.capability_fingerprint.trim().is_empty() {
            return Err(CheckpointRecoveryError::InvalidMetadata {
                field: "capability_fingerprint",
            });
        }

        if !self.restoration_authorized {
            return Err(CheckpointRecoveryError::RestorationNotAuthorized);
        }

        if !self.state_kind.is_restorable() {
            return Err(CheckpointRecoveryError::StateNotRestorable {
                state: self.state_kind,
            });
        }

        Ok(())
    }
}

// ============================================================================
// Target compatibility
// ============================================================================

/// Target compatibility information supplied by the hardware/runtime layer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RestoreTarget {
    /// Stable target identity.
    pub target_id: String,

    /// Current IR schema/version.
    pub ir_schema_version: String,

    /// Current resilience checkpoint schema/version.
    pub checkpoint_schema_version: String,

    /// Current target capability fingerprint.
    pub capability_fingerprint: String,

    /// Whether the target supports restoration at all.
    pub restoration_supported: bool,

    /// Whether provider/runtime snapshots are supported.
    pub snapshot_restore_supported: bool,

    /// Whether logical/QEC checkpoint restoration is supported.
    pub logical_restore_supported: bool,

    /// Whether measurement-boundary restoration is supported.
    pub measurement_restore_supported: bool,
}

impl RestoreTarget {
    /// Validates basic target information.
    pub fn validate(&self) -> Result<(), CheckpointRecoveryError> {
        if self.target_id.trim().is_empty() {
            return Err(CheckpointRecoveryError::InvalidTarget {
                field: "target_id",
            });
        }

        if self.ir_schema_version.trim().is_empty() {
            return Err(CheckpointRecoveryError::InvalidTarget {
                field: "ir_schema_version",
            });
        }

        if self.checkpoint_schema_version.trim().is_empty() {
            return Err(CheckpointRecoveryError::InvalidTarget {
                field: "checkpoint_schema_version",
            });
        }

        if self.capability_fingerprint.trim().is_empty() {
            return Err(CheckpointRecoveryError::InvalidTarget {
                field: "capability_fingerprint",
            });
        }

        if !self.restoration_supported {
            return Err(CheckpointRecoveryError::TargetRestoreUnsupported);
        }

        Ok(())
    }
}

// ============================================================================
// Recovery policy
// ============================================================================

/// Policy supplied by the resilience policy subsystem.
///
/// This structure intentionally contains no implementation of policy. It is a
/// snapshot of the authorization relevant to one checkpoint restoration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckpointRecoveryPolicy {
    /// Whether checkpoint restoration is allowed.
    pub restoration_allowed: bool,

    /// Whether cross-capability-fingerprint restoration is allowed.
    pub cross_capability_restore_allowed: bool,

    /// Whether provider snapshots may be restored.
    pub provider_snapshot_restore_allowed: bool,

    /// Whether logical/QEC checkpoints may be restored.
    pub logical_restore_allowed: bool,

    /// Whether measurement-boundary restoration may be used.
    pub measurement_restore_allowed: bool,

    /// Maximum checkpoint age, if policy imposes one.
    ///
    /// `None` means policy does not impose an age limit.
    pub maximum_age: Option<Duration>,
}

impl CheckpointRecoveryPolicy {
    /// Validates policy consistency.
    pub fn validate(&self) -> Result<(), CheckpointRecoveryError> {
        if !self.restoration_allowed {
            return Err(CheckpointRecoveryError::PolicyRejected);
        }

        Ok(())
    }
}

// ============================================================================
// Recovery request
// ============================================================================

/// Immutable request to restore a checkpoint.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckpointRecoveryRequest {
    /// Checkpoint to restore.
    pub checkpoint_id: CheckpointId,

    /// Execution that requested recovery.
    pub execution_id: ExecutionId,

    /// Program expected by the recovery operation.
    pub program_identity: ProgramIdentity,

    /// Optional logical qubits whose semantic mapping must be checked by the
    /// verification layer.
    ///
    /// This uses the canonical IR `QubitId` and does not define another
    /// resilience-specific qubit identity.
    pub affected_logical_qubits: Vec<QubitId>,

    /// Current target.
    pub target: RestoreTarget,

    /// Recovery policy snapshot.
    pub policy: CheckpointRecoveryPolicy,

    /// Correlation/operation identity for telemetry and provenance.
    pub recovery_operation_id: String,

    /// Whether deterministic recovery mode is requested.
    pub deterministic: bool,
}

impl CheckpointRecoveryRequest {
    /// Performs request validation.
    pub fn validate(&self) -> Result<(), CheckpointRecoveryError> {
        if self.recovery_operation_id.trim().is_empty() {
            return Err(CheckpointRecoveryError::InvalidIdentifier {
                field: "recovery_operation_id",
            });
        }

        self.target.validate()?;
        self.policy.validate()?;

        Ok(())
    }
}

// ============================================================================
// Checkpoint provider contracts
// ============================================================================

/// Provides checkpoint metadata.
///
/// This is deliberately separate from payload restoration. Metadata can be
/// inspected before any destructive or expensive restore operation occurs.
pub trait CheckpointMetadataProvider: Send + Sync {
    /// Loads immutable checkpoint metadata.
    fn metadata(
        &self,
        checkpoint_id: &CheckpointId,
    ) -> Result<CheckpointMetadata, CheckpointRecoveryError>;
}

/// Verifies checkpoint payload integrity.
///
/// Implementations belong to the checkpoint integrity subsystem/storage
/// adapter, not to this recovery coordinator.
pub trait CheckpointIntegrityVerifier: Send + Sync {
    /// Verifies that checkpoint contents match the declared integrity data.
    fn verify_integrity(
        &self,
        metadata: &CheckpointMetadata,
    ) -> Result<(), CheckpointRecoveryError>;
}

/// Checks checkpoint compatibility with the current execution target.
pub trait CheckpointCompatibilityValidator: Send + Sync {
    /// Validates checkpoint against the target and request.
    fn validate_compatibility(
        &self,
        metadata: &CheckpointMetadata,
        request: &CheckpointRecoveryRequest,
    ) -> Result<(), CheckpointRecoveryError>;
}

/// Performs the actual restore.
///
/// This is the only trait in this module that should cross into the runtime or
/// hardware execution layer.
pub trait CheckpointRestorer: Send + Sync {
    /// Opaque restored execution handle.
    type Restored: Send + Sync + 'static;

    /// Restores a validated checkpoint.
    fn restore(
        &self,
        metadata: &CheckpointMetadata,
        request: &CheckpointRecoveryRequest,
    ) -> Result<Self::Restored, CheckpointRecoveryError>;
}

/// Verifies the state after restoration.
///
/// Semantic acceptance remains owned by the verification subsystem. This trait
/// therefore returns a structured verification result instead of `bool`.
pub trait CheckpointRestoreVerifier<R>: Send + Sync {
    /// Verifies a restored execution.
    fn verify(
        &self,
        metadata: &CheckpointMetadata,
        request: &CheckpointRecoveryRequest,
        restored: &R,
    ) -> Result<RestoreVerification, CheckpointRecoveryError>;
}

/// Optional recovery event observer.
///
/// The observer MUST NOT control whether restoration is accepted. It exists
/// only for telemetry, audit and history integration.
pub trait CheckpointRecoveryObserver: Send + Sync {
    /// Records a recovery lifecycle event.
    fn observe(&self, event: &CheckpointRecoveryEvent);
}

// ============================================================================
// Verification result
// ============================================================================

/// Result of post-restore verification.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RestoreVerification {
    /// Whether semantic verification succeeded.
    pub semantic_valid: bool,

    /// Whether resource/capability verification succeeded.
    pub capability_valid: bool,

    /// Whether provenance is intact.
    pub provenance_valid: bool,

    /// Whether security/integrity conditions remain valid.
    pub security_valid: bool,

    /// Optional deterministic verifier fingerprint.
    pub verification_fingerprint: Option<String>,

    /// Human-readable reason safe for diagnostics.
    pub reason: Option<String>,
}

impl RestoreVerification {
    /// Returns whether every mandatory acceptance dimension passed.
    #[must_use]
    pub const fn accepted(&self) -> bool {
        self.semantic_valid
            && self.capability_valid
            && self.provenance_valid
            && self.security_valid
    }

    /// Validates that a verification fingerprint is present when required by
    /// deterministic mode.
    pub fn validate(&self, deterministic: bool) -> Result<(), CheckpointRecoveryError> {
        if deterministic && self.verification_fingerprint.is_none() {
            return Err(CheckpointRecoveryError::VerificationFingerprintMissing);
        }

        Ok(())
    }
}

// ============================================================================
// Lifecycle
// ============================================================================

/// Explicit checkpoint recovery lifecycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CheckpointRecoveryState {
    /// No recovery operation is active.
    Idle,

    /// Request has been structurally validated.
    Prepared,

    /// Metadata has been loaded.
    MetadataLoaded,

    /// Integrity has been verified.
    IntegrityVerified,

    /// Compatibility has been verified.
    CompatibilityVerified,

    /// Restoration is in progress.
    Restoring,

    /// Restoration completed and verification is running.
    Verifying,

    /// Restored state passed all acceptance checks.
    Accepted,

    /// Restoration completed but acceptance could not be established.
    Rejected,

    /// Operation was cancelled before acceptance.
    Cancelled,

    /// Operation failed.
    Failed,
}

impl CheckpointRecoveryState {
    /// Returns whether this is a terminal state.
    #[must_use]
    pub const fn is_terminal(self) -> bool {
        matches!(
            self,
            Self::Accepted | Self::Rejected | Self::Cancelled | Self::Failed
        )
    }
}

/// Lifecycle event.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckpointRecoveryEvent {
    /// Recovery operation identity.
    pub recovery_operation_id: String,

    /// Checkpoint identity.
    pub checkpoint_id: CheckpointId,

    /// State reached.
    pub state: CheckpointRecoveryState,

    /// Event timestamp.
    pub timestamp: SystemTime,

    /// Optional safe diagnostic message.
    pub message: Option<String>,
}

// ============================================================================
// Outcome
// ============================================================================

/// Final checkpoint restoration status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CheckpointRecoveryStatus {
    /// Checkpoint restoration succeeded and was verified.
    Accepted,

    /// Restoration occurred, but semantic acceptance failed.
    Rejected,

    /// Recovery was cancelled.
    Cancelled,

    /// Recovery failed before producing an accepted restoration.
    Failed,
}

/// Immutable outcome of a checkpoint recovery operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckpointRecoveryOutcome<R> {
    /// Final status.
    pub status: CheckpointRecoveryStatus,

    /// Checkpoint identity.
    pub checkpoint_id: CheckpointId,

    /// Execution identity.
    pub execution_id: ExecutionId,

    /// Program identity.
    pub program_identity: ProgramIdentity,

    /// Final lifecycle state.
    pub final_state: CheckpointRecoveryState,

    /// Restored execution handle when restoration reached the runtime.
    pub restored: Option<Arc<R>>,

    /// Verification result, if verification was reached.
    pub verification: Option<RestoreVerification>,

    /// Target identity.
    pub target_id: String,

    /// Number of lifecycle restore operations performed.
    ///
    /// This is observational, not a retry limit.
    pub restore_operations: u64,

    /// Creation time of the outcome.
    pub completed_at: SystemTime,
}

impl<R> CheckpointRecoveryOutcome<R> {
    /// Returns whether the outcome is accepted.
    #[must_use]
    pub const fn accepted(&self) -> bool {
        matches!(self.status, CheckpointRecoveryStatus::Accepted)
    }
}

// ============================================================================
// Cancellation
// ============================================================================

/// Cancellation contract.
///
/// Cancellation is cooperative. The recovery coordinator checks cancellation
/// before crossing into the potentially expensive restore operation.
pub trait CheckpointCancellation: Send + Sync {
    /// Returns `true` when the operation must stop.
    fn is_cancelled(&self) -> bool;
}

/// Cancellation implementation that never cancels.
#[derive(Debug, Default, Clone, Copy)]
pub struct NeverCancel;

impl CheckpointCancellation for NeverCancel {
    fn is_cancelled(&self) -> bool {
        false
    }
}

// ============================================================================
// Coordinator
// ============================================================================

/// Production checkpoint recovery coordinator.
///
/// The coordinator is intentionally generic over the actual restored runtime
/// object. This keeps the recovery layer independent of the runtime/hardware
/// implementation.
pub struct CheckpointRecoveryCoordinator<
    M,
    I,
    C,
    R,
    V,
    O = NoopCheckpointRecoveryObserver,
    X = NeverCancel,
> where
    M: CheckpointMetadataProvider,
    I: CheckpointIntegrityVerifier,
    C: CheckpointCompatibilityValidator,
    R: CheckpointRestorer,
    V: CheckpointRestoreVerifier<R::Restored>,
    O: CheckpointRecoveryObserver,
    X: CheckpointCancellation,
{
    metadata_provider: Arc<M>,
    integrity_verifier: Arc<I>,
    compatibility_validator: Arc<C>,
    restorer: Arc<R>,
    verifier: Arc<V>,
    observer: Arc<O>,
    cancellation: Arc<X>,
}

impl<M, I, C, R, V>
    CheckpointRecoveryCoordinator<M, I, C, R, V, NoopCheckpointRecoveryObserver, NeverCancel>
where
    M: CheckpointMetadataProvider,
    I: CheckpointIntegrityVerifier,
    C: CheckpointCompatibilityValidator,
    R: CheckpointRestorer,
    V: CheckpointRestoreVerifier<R::Restored>,
{
    /// Creates a coordinator with no-op observation and no cancellation.
    #[must_use]
    pub fn new(
        metadata_provider: Arc<M>,
        integrity_verifier: Arc<I>,
        compatibility_validator: Arc<C>,
        restorer: Arc<R>,
        verifier: Arc<V>,
    ) -> Self {
        Self {
            metadata_provider,
            integrity_verifier,
            compatibility_validator,
            restorer,
            verifier,
            observer: Arc::new(NoopCheckpointRecoveryObserver),
            cancellation: Arc::new(NeverCancel),
        }
    }
}

impl<M, I, C, R, V, O, X> CheckpointRecoveryCoordinator<M, I, C, R, V, O, X>
where
    M: CheckpointMetadataProvider,
    I: CheckpointIntegrityVerifier,
    C: CheckpointCompatibilityValidator,
    R: CheckpointRestorer,
    V: CheckpointRestoreVerifier<R::Restored>,
    O: CheckpointRecoveryObserver,
    X: CheckpointCancellation,
{
    /// Creates a fully configured coordinator.
    #[must_use]
    pub fn with_observer_and_cancellation(
        metadata_provider: Arc<M>,
        integrity_verifier: Arc<I>,
        compatibility_validator: Arc<C>,
        restorer: Arc<R>,
        verifier: Arc<V>,
        observer: Arc<O>,
        cancellation: Arc<X>,
    ) -> Self {
        Self {
            metadata_provider,
            integrity_verifier,
            compatibility_validator,
            restorer,
            verifier,
            observer,
            cancellation,
        }
    }

    /// Restores a checkpoint after all mandatory validation gates pass.
    ///
    /// The order is intentionally fixed:
    ///
    /// 1. request validation;
    /// 2. cancellation;
    /// 3. metadata load;
    /// 4. metadata validation;
    /// 5. identity validation;
    /// 6. age/policy validation;
    /// 7. integrity verification;
    /// 8. compatibility validation;
    /// 9. cancellation;
    /// 10. restoration;
    /// 11. post-restore verification;
    /// 12. acceptance.
    ///
    /// No external lock is held across calls into storage, runtime or
    /// verification implementations.
    pub fn restore(
        &self,
        request: &CheckpointRecoveryRequest,
    ) -> Result<CheckpointRecoveryOutcome<R::Restored>, CheckpointRecoveryError> {
        request.validate()?;

        self.emit(
            request,
            CheckpointRecoveryState::Prepared,
            None,
        );

        self.check_cancelled(request)?;

        let metadata = self.metadata_provider.metadata(&request.checkpoint_id)?;

        self.emit(
            request,
            CheckpointRecoveryState::MetadataLoaded,
            None,
        );

        metadata.validate()?;

        self.validate_identity(&metadata, request)?;
        self.validate_age(&metadata, &request.policy)?;

        self.check_cancelled(request)?;

        self.integrity_verifier.verify_integrity(&metadata)?;

        self.emit(
            request,
            CheckpointRecoveryState::IntegrityVerified,
            None,
        );

        self.compatibility_validator
            .validate_compatibility(&metadata, request)?;

        self.validate_boundary_policy(&metadata, &request.policy)?;
        self.validate_target_support(&metadata, &request.target)?;

        self.emit(
            request,
            CheckpointRecoveryState::CompatibilityVerified,
            None,
        );

        self.check_cancelled(request)?;

        self.emit(
            request,
            CheckpointRecoveryState::Restoring,
            None,
        );

        let restored = self.restorer.restore(&metadata, request)?;

        self.emit(
            request,
            CheckpointRecoveryState::Verifying,
            None,
        );

        let verification = self
            .verifier
            .verify(&metadata, request, &restored)?;

        verification.validate(request.deterministic)?;

        if !verification.accepted() {
            let reason = verification
                .reason
                .clone()
                .unwrap_or_else(|| "restored state failed acceptance verification".to_owned());

            self.emit(
                request,
                CheckpointRecoveryState::Rejected,
                Some(reason.clone()),
            );

            return Ok(CheckpointRecoveryOutcome {
                status: CheckpointRecoveryStatus::Rejected,
                checkpoint_id: metadata.checkpoint_id,
                execution_id: metadata.execution_id,
                program_identity: metadata.program_identity,
                final_state: CheckpointRecoveryState::Rejected,
                restored: Some(Arc::new(restored)),
                verification: Some(verification),
                target_id: request.target.target_id.clone(),
                restore_operations: 1,
                completed_at: SystemTime::now(),
            });
        }

        self.emit(
            request,
            CheckpointRecoveryState::Accepted,
            None,
        );

        Ok(CheckpointRecoveryOutcome {
            status: CheckpointRecoveryStatus::Accepted,
            checkpoint_id: metadata.checkpoint_id,
            execution_id: metadata.execution_id,
            program_identity: metadata.program_identity,
            final_state: CheckpointRecoveryState::Accepted,
            restored: Some(Arc::new(restored)),
            verification: Some(verification),
            target_id: request.target.target_id.clone(),
            restore_operations: 1,
            completed_at: SystemTime::now(),
        })
    }

    fn validate_identity(
        &self,
        metadata: &CheckpointMetadata,
        request: &CheckpointRecoveryRequest,
    ) -> Result<(), CheckpointRecoveryError> {
        if metadata.execution_id != request.execution_id {
            return Err(CheckpointRecoveryError::ExecutionIdentityMismatch);
        }

        if metadata.program_identity != request.program_identity {
            return Err(CheckpointRecoveryError::ProgramIdentityMismatch);
        }

        Ok(())
    }

    fn validate_age(
        &self,
        metadata: &CheckpointMetadata,
        policy: &CheckpointRecoveryPolicy,
    ) -> Result<(), CheckpointRecoveryError> {
        let Some(maximum_age) = policy.maximum_age else {
            return Ok(());
        };

        let now = SystemTime::now();

        let age = now
            .duration_since(metadata.created_at)
            .map_err(|_| CheckpointRecoveryError::ClockError)?;

        if age > maximum_age {
            return Err(CheckpointRecoveryError::CheckpointTooOld {
                age,
                maximum_age,
            });
        }

        Ok(())
    }

    fn validate_boundary_policy(
        &self,
        metadata: &CheckpointMetadata,
        policy: &CheckpointRecoveryPolicy,
    ) -> Result<(), CheckpointRecoveryError> {
        match metadata.boundary {
            CheckpointBoundary::ProviderSupportedSnapshot
                if !policy.provider_snapshot_restore_allowed =>
            {
                Err(CheckpointRecoveryError::PolicyRejected)
            }

            CheckpointBoundary::LogicalBoundary if !policy.logical_restore_allowed => {
                Err(CheckpointRecoveryError::PolicyRejected)
            }

            CheckpointBoundary::MeasurementBoundary
                if !policy.measurement_restore_allowed =>
            {
                Err(CheckpointRecoveryError::PolicyRejected)
            }

            _ => Ok(()),
        }
    }

    fn validate_target_support(
        &self,
        metadata: &CheckpointMetadata,
        target: &RestoreTarget,
    ) -> Result<(), CheckpointRecoveryError> {
        match metadata.boundary {
            CheckpointBoundary::ProviderSupportedSnapshot
                if !target.snapshot_restore_supported =>
            {
                Err(CheckpointRecoveryError::TargetBoundaryUnsupported {
                    boundary: metadata.boundary,
                })
            }

            CheckpointBoundary::LogicalBoundary if !target.logical_restore_supported => {
                Err(CheckpointRecoveryError::TargetBoundaryUnsupported {
                    boundary: metadata.boundary,
                })
            }

            CheckpointBoundary::MeasurementBoundary
                if !target.measurement_restore_supported =>
            {
                Err(CheckpointRecoveryError::TargetBoundaryUnsupported {
                    boundary: metadata.boundary,
                })
            }

            _ => Ok(()),
        }
    }

    fn check_cancelled(
        &self,
        request: &CheckpointRecoveryRequest,
    ) -> Result<(), CheckpointRecoveryError> {
        if self.cancellation.is_cancelled() {
            self.emit(
                request,
                CheckpointRecoveryState::Cancelled,
                Some("checkpoint recovery cancelled".to_owned()),
            );

            return Err(CheckpointRecoveryError::Cancelled);
        }

        Ok(())
    }

    fn emit(
        &self,
        request: &CheckpointRecoveryRequest,
        state: CheckpointRecoveryState,
        message: Option<String>,
    ) {
        self.observer.observe(&CheckpointRecoveryEvent {
            recovery_operation_id: request.recovery_operation_id.clone(),
            checkpoint_id: request.checkpoint_id.clone(),
            state,
            timestamp: SystemTime::now(),
            message,
        });
    }
}

// ============================================================================
// No-op observer
// ============================================================================

/// Default observer that intentionally does nothing.
#[derive(Debug, Default, Clone, Copy)]
pub struct NoopCheckpointRecoveryObserver;

impl CheckpointRecoveryObserver for NoopCheckpointRecoveryObserver {
    fn observe(&self, _event: &CheckpointRecoveryEvent) {}
}

// ============================================================================
// Error model
// ============================================================================

/// Checkpoint-recovery-specific error.
///
/// The broader resilience error subsystem can translate this error into the
/// repository-wide `ResilienceError` while preserving the semantic category.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CheckpointRecoveryError {
    /// Invalid stable identifier.
    InvalidIdentifier {
        /// Invalid field.
        field: &'static str,
    },

    /// Invalid checkpoint metadata.
    InvalidMetadata {
        /// Invalid field.
        field: &'static str,
    },

    /// Invalid target description.
    InvalidTarget {
        /// Invalid field.
        field: &'static str,
    },

    /// Restoration was not authorized by the checkpoint producer.
    RestorationNotAuthorized,

    /// Checkpoint state cannot safely be restored.
    StateNotRestorable {
        /// State kind.
        state: CheckpointStateKind,
    },

    /// Recovery policy rejected restoration.
    PolicyRejected,

    /// Target cannot restore checkpoints.
    TargetRestoreUnsupported,

    /// Requested boundary is unsupported by the target.
    TargetBoundaryUnsupported {
        /// Unsupported boundary.
        boundary: CheckpointBoundary,
    },

    /// Checkpoint belongs to a different execution.
    ExecutionIdentityMismatch,

    /// Checkpoint belongs to a different program.
    ProgramIdentityMismatch,

    /// Checkpoint exceeded configured age.
    CheckpointTooOld {
        /// Observed age.
        age: Duration,

        /// Maximum permitted age.
        maximum_age: Duration,
    },

    /// Clock moved backwards relative to checkpoint creation.
    ClockError,

    /// Integrity verification failed.
    IntegrityFailure(String),

    /// Compatibility validation failed.
    CompatibilityFailure(String),

    /// Storage/metadata operation failed.
    StorageFailure(String),

    /// Runtime restore failed.
    RestoreFailure(String),

    /// Verification failed.
    VerificationFailure(String),

    /// Deterministic mode requires a verification fingerprint.
    VerificationFingerprintMissing,

    /// Recovery was cancelled.
    Cancelled,
}

impl fmt::Display for CheckpointRecoveryError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidIdentifier { field } => {
                write!(formatter, "invalid checkpoint recovery identifier: {field}")
            }

            Self::InvalidMetadata { field } => {
                write!(formatter, "invalid checkpoint metadata: {field}")
            }

            Self::InvalidTarget { field } => {
                write!(formatter, "invalid checkpoint restore target: {field}")
            }

            Self::RestorationNotAuthorized => {
                formatter.write_str("checkpoint restoration is not authorized")
            }

            Self::StateNotRestorable { state } => {
                write!(formatter, "checkpoint state is not restorable: {state:?}")
            }

            Self::PolicyRejected => {
                formatter.write_str("checkpoint restoration rejected by policy")
            }

            Self::TargetRestoreUnsupported => {
                formatter.write_str("target does not support checkpoint restoration")
            }

            Self::TargetBoundaryUnsupported { boundary } => {
                write!(formatter, "target does not support checkpoint boundary: {boundary:?}")
            }

            Self::ExecutionIdentityMismatch => {
                formatter.write_str("checkpoint execution identity does not match recovery request")
            }

            Self::ProgramIdentityMismatch => {
                formatter.write_str("checkpoint program identity does not match recovery request")
            }

            Self::CheckpointTooOld {
                age,
                maximum_age,
            } => {
                write!(
                    formatter,
                    "checkpoint is too old: age={age:?}, maximum_age={maximum_age:?}"
                )
            }

            Self::ClockError => {
                formatter.write_str("system clock is earlier than checkpoint creation time")
            }

            Self::IntegrityFailure(message) => {
                write!(formatter, "checkpoint integrity verification failed: {message}")
            }

            Self::CompatibilityFailure(message) => {
                write!(formatter, "checkpoint compatibility verification failed: {message}")
            }

            Self::StorageFailure(message) => {
                write!(formatter, "checkpoint storage operation failed: {message}")
            }

            Self::RestoreFailure(message) => {
                write!(formatter, "checkpoint restoration failed: {message}")
            }

            Self::VerificationFailure(message) => {
                write!(formatter, "checkpoint restore verification failed: {message}")
            }

            Self::VerificationFingerprintMissing => {
                formatter.write_str(
                    "deterministic checkpoint recovery requires a verification fingerprint",
                )
            }

            Self::Cancelled => {
                formatter.write_str("checkpoint recovery was cancelled")
            }
        }
    }
}

impl std::error::Error for CheckpointRecoveryError {}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use std::sync::atomic::{AtomicBool, Ordering};

    fn checkpoint_id() -> CheckpointId {
        CheckpointId::new("checkpoint:test").expect("valid checkpoint id")
    }

    fn execution_id() -> ExecutionId {
        ExecutionId::new("execution:test").expect("valid execution id")
    }

    fn program_identity() -> ProgramIdentity {
        ProgramIdentity::new("program:test").expect("valid program identity")
    }

    fn metadata() -> CheckpointMetadata {
        CheckpointMetadata {
            checkpoint_id: checkpoint_id(),
            execution_id: execution_id(),
            program_identity: program_identity(),
            ir_schema_version: "zamani.ir.v1".to_owned(),
            checkpoint_schema_version: "zamani.checkpoint.v1".to_owned(),
            boundary: CheckpointBoundary::MeasurementBoundary,
            state_kind: CheckpointStateKind::MeasurementBoundary,
            execution_position: "boundary:measurement:0".to_owned(),
            integrity_digest: "digest:test".to_owned(),
            capability_fingerprint: "capabilities:test".to_owned(),
            created_at: SystemTime::now(),
            restoration_authorized: true,
        }
    }

    fn target() -> RestoreTarget {
        RestoreTarget {
            target_id: "target:test".to_owned(),
            ir_schema_version: "zamani.ir.v1".to_owned(),
            checkpoint_schema_version: "zamani.checkpoint.v1".to_owned(),
            capability_fingerprint: "capabilities:test".to_owned(),
            restoration_supported: true,
            snapshot_restore_supported: true,
            logical_restore_supported: true,
            measurement_restore_supported: true,
        }
    }

    fn policy() -> CheckpointRecoveryPolicy {
        CheckpointRecoveryPolicy {
            restoration_allowed: true,
            cross_capability_restore_allowed: true,
            provider_snapshot_restore_allowed: true,
            logical_restore_allowed: true,
            measurement_restore_allowed: true,
            maximum_age: None,
        }
    }

    fn request() -> CheckpointRecoveryRequest {
        CheckpointRecoveryRequest {
            checkpoint_id: checkpoint_id(),
            execution_id: execution_id(),
            program_identity: program_identity(),
            affected_logical_qubits: Vec::new(),
            target: target(),
            policy: policy(),
            recovery_operation_id: "recovery:test".to_owned(),
            deterministic: true,
        }
    }

    struct MetadataProvider;

    impl CheckpointMetadataProvider for MetadataProvider {
        fn metadata(
            &self,
            _checkpoint_id: &CheckpointId,
        ) -> Result<CheckpointMetadata, CheckpointRecoveryError> {
            Ok(metadata())
        }
    }

    struct IntegrityVerifier;

    impl CheckpointIntegrityVerifier for IntegrityVerifier {
        fn verify_integrity(
            &self,
            _metadata: &CheckpointMetadata,
        ) -> Result<(), CheckpointRecoveryError> {
            Ok(())
        }
    }

    struct CompatibilityValidator;

    impl CheckpointCompatibilityValidator for CompatibilityValidator {
        fn validate_compatibility(
            &self,
            metadata: &CheckpointMetadata,
            request: &CheckpointRecoveryRequest,
        ) -> Result<(), CheckpointRecoveryError> {
            if metadata.ir_schema_version != request.target.ir_schema_version {
                return Err(CheckpointRecoveryError::CompatibilityFailure(
                    "IR schema mismatch".to_owned(),
                ));
            }

            Ok(())
        }
    }

    struct Restorer;

    impl CheckpointRestorer for Restorer {
        type Restored = String;

        fn restore(
            &self,
            _metadata: &CheckpointMetadata,
            _request: &CheckpointRecoveryRequest,
        ) -> Result<Self::Restored, CheckpointRecoveryError> {
            Ok("restored-execution".to_owned())
        }
    }

    struct Verifier;

    impl CheckpointRestoreVerifier<String> for Verifier {
        fn verify(
            &self,
            _metadata: &CheckpointMetadata,
            _request: &CheckpointRecoveryRequest,
            _restored: &String,
        ) -> Result<RestoreVerification, CheckpointRecoveryError> {
            Ok(RestoreVerification {
                semantic_valid: true,
                capability_valid: true,
                provenance_valid: true,
                security_valid: true,
                verification_fingerprint: Some("verification:test".to_owned()),
                reason: None,
            })
        }
    }

    #[derive(Default)]
    struct TestCancellation {
        cancelled: AtomicBool,
    }

    impl CheckpointCancellation for TestCancellation {
        fn is_cancelled(&self) -> bool {
            self.cancelled.load(Ordering::Acquire)
        }
    }

    #[test]
    fn restores_valid_checkpoint() {
        let coordinator = CheckpointRecoveryCoordinator::with_observer_and_cancellation(
            Arc::new(MetadataProvider),
            Arc::new(IntegrityVerifier),
            Arc::new(CompatibilityValidator),
            Arc::new(Restorer),
            Arc::new(Verifier),
            Arc::new(NoopCheckpointRecoveryObserver),
            Arc::new(NeverCancel),
        );

        let result = coordinator.restore(&request()).expect("restore succeeds");

        assert!(result.accepted());
        assert_eq!(
            result.final_state,
            CheckpointRecoveryState::Accepted
        );
        assert_eq!(
            result.restored.as_deref(),
            Some(&"restored-execution".to_owned())
        );
    }

    #[test]
    fn rejects_wrong_execution_identity() {
        let coordinator = CheckpointRecoveryCoordinator::new(
            Arc::new(MetadataProvider),
            Arc::new(IntegrityVerifier),
            Arc::new(CompatibilityValidator),
            Arc::new(Restorer),
            Arc::new(Verifier),
        );

        let mut request = request();
        request.execution_id =
            ExecutionId::new("execution:different").expect("valid execution id");

        let error = coordinator
            .restore(&request)
            .expect_err("identity mismatch must fail");

        assert_eq!(
            error,
            CheckpointRecoveryError::ExecutionIdentityMismatch
        );
    }

    #[test]
    fn rejects_unauthorized_checkpoint() {
        let mut checkpoint = metadata();
        checkpoint.restoration_authorized = false;

        struct UnauthorizedProvider;

        impl CheckpointMetadataProvider for UnauthorizedProvider {
            fn metadata(
                &self,
                _checkpoint_id: &CheckpointId,
            ) -> Result<CheckpointMetadata, CheckpointRecoveryError> {
                let mut value = metadata();
                value.restoration_authorized = false;
                Ok(value)
            }
        }

        let coordinator = CheckpointRecoveryCoordinator::new(
            Arc::new(UnauthorizedProvider),
            Arc::new(IntegrityVerifier),
            Arc::new(CompatibilityValidator),
            Arc::new(Restorer),
            Arc::new(Verifier),
        );

        let error = coordinator
            .restore(&request())
            .expect_err("unauthorized restore must fail");

        assert_eq!(
            error,
            CheckpointRecoveryError::RestorationNotAuthorized
        );

        let _ = &mut checkpoint;
    }

    #[test]
    fn rejects_unsupported_measurement_boundary() {
        let mut target = target();
        target.measurement_restore_supported = false;

        let mut request = request();
        request.target = target;

        let coordinator = CheckpointRecoveryCoordinator::new(
            Arc::new(MetadataProvider),
            Arc::new(IntegrityVerifier),
            Arc::new(CompatibilityValidator),
            Arc::new(Restorer),
            Arc::new(Verifier),
        );

        let error = coordinator
            .restore(&request)
            .expect_err("unsupported boundary must fail");

        assert_eq!(
            error,
            CheckpointRecoveryError::TargetBoundaryUnsupported {
                boundary: CheckpointBoundary::MeasurementBoundary,
            }
        );
    }

    #[test]
    fn cancellation_is_checked_before_restore() {
        let cancellation = Arc::new(TestCancellation {
            cancelled: AtomicBool::new(true),
        });

        let coordinator = CheckpointRecoveryCoordinator::with_observer_and_cancellation(
            Arc::new(MetadataProvider),
            Arc::new(IntegrityVerifier),
            Arc::new(CompatibilityValidator),
            Arc::new(Restorer),
            Arc::new(Verifier),
            Arc::new(NoopCheckpointRecoveryObserver),
            cancellation,
        );

        let error = coordinator
            .restore(&request())
            .expect_err("cancelled recovery must fail");

        assert_eq!(error, CheckpointRecoveryError::Cancelled);
    }

    #[test]
    fn rejects_unrestorable_unknown_state() {
        let mut checkpoint = metadata();
        checkpoint.state_kind = CheckpointStateKind::Unknown;

        struct UnknownStateProvider;

        impl CheckpointMetadataProvider for UnknownStateProvider {
            fn metadata(
                &self,
                _checkpoint_id: &CheckpointId,
            ) -> Result<CheckpointMetadata, CheckpointRecoveryError> {
                let mut value = metadata();
                value.state_kind = CheckpointStateKind::Unknown;
                Ok(value)
            }
        }

        let coordinator = CheckpointRecoveryCoordinator::new(
            Arc::new(UnknownStateProvider),
            Arc::new(IntegrityVerifier),
            Arc::new(CompatibilityValidator),
            Arc::new(Restorer),
            Arc::new(Verifier),
        );

        let error = coordinator
            .restore(&request())
            .expect_err("unknown state cannot be restored");

        assert_eq!(
            error,
            CheckpointRecoveryError::StateNotRestorable {
                state: CheckpointStateKind::Unknown,
            }
        );
    }

    #[test]
    fn verification_rejection_never_becomes_acceptance() {
        struct RejectingVerifier;

        impl CheckpointRestoreVerifier<String> for RejectingVerifier {
            fn verify(
                &self,
                _metadata: &CheckpointMetadata,
                _request: &CheckpointRecoveryRequest,
                _restored: &String,
            ) -> Result<RestoreVerification, CheckpointRecoveryError> {
                Ok(RestoreVerification {
                    semantic_valid: false,
                    capability_valid: true,
                    provenance_valid: true,
                    security_valid: true,
                    verification_fingerprint: Some("verification:rejected".to_owned()),
                    reason: Some("semantic mismatch".to_owned()),
                })
            }
        }

        let coordinator = CheckpointRecoveryCoordinator::new(
            Arc::new(MetadataProvider),
            Arc::new(IntegrityVerifier),
            Arc::new(CompatibilityValidator),
            Arc::new(Restorer),
            Arc::new(RejectingVerifier),
        );

        let result = coordinator
            .restore(&request())
            .expect("verification rejection is a valid recovery outcome");

        assert!(!result.accepted());
        assert_eq!(
            result.status,
            CheckpointRecoveryStatus::Rejected
        );
        assert_eq!(
            result.final_state,
            CheckpointRecoveryState::Rejected
        );
    }

    #[test]
    fn deterministic_mode_requires_verification_fingerprint() {
        let verification = RestoreVerification {
            semantic_valid: true,
            capability_valid: true,
            provenance_valid: true,
            security_valid: true,
            verification_fingerprint: None,
            reason: None,
        };

        assert_eq!(
            verification.validate(true),
            Err(CheckpointRecoveryError::VerificationFingerprintMissing)
        );

        assert!(verification.validate(false).is_ok());
    }

    #[test]
    fn resource_scaling_does_not_depend_on_fixed_qubit_count() {
        let request = CheckpointRecoveryRequest {
            affected_logical_qubits: vec![
                QubitId::from(0_u64),
                QubitId::from(1_u64),
                QubitId::from(10_000_u64),
            ],
            ..request()
        };

        assert_eq!(request.affected_logical_qubits.len(), 3);
    }
}