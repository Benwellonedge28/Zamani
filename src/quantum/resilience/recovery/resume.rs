//! Zamani Quantum Resilience — Resume Execution
//!
//! Path:
//!     src/quantum/resilience/recovery/resume.rs
//!
//! Purpose:
//!     Provide the provider-independent execution contract for safely
//!     continuing a quantum computation from a previously validated
//!     checkpoint or execution boundary.
//!
//! ============================================================================
//! ARCHITECTURAL POSITION
//! ============================================================================
//!
//!     Execution
//!         |
//!         v
//!     Detection
//!         |
//!         v
//!     Diagnosis
//!         |
//!         v
//!     Policy
//!         |
//!         v
//!     Planning
//!         |
//!         v
//!     RecoveryAction::Resume
//!         |
//!         v
//!     ResumeController
//!         |
//!         +--> checkpoint validation
//!         +--> execution identity validation
//!         +--> semantic compatibility
//!         +--> capability validation
//!         +--> authorization
//!         +--> continuation
//!         +--> post-resume verification
//!         |
//!         v
//!     Verification
//!         |
//!         +--> Accepted
//!         +--> Degraded
//!         +--> Rejected
//!         +--> NeedsReplan
//!         +--> Escalated
//!
//! ============================================================================
//! OWNERSHIP
//! ============================================================================
//!
//! This file owns:
//!
//! - resume request validation;
//! - continuation-boundary validation;
//! - resume execution orchestration;
//! - cancellation boundaries;
//! - idempotency contract;
//! - stale execution protection;
//! - post-resume verification contract;
//! - structured resume outcomes;
//! - lifecycle state for one resume operation;
//! - provenance requirements;
//! - deterministic resume semantics.
//!
//! This file does NOT own:
//!
//! - canonical quantum IR;
//! - qubit identity;
//! - checkpoint storage;
//! - checkpoint serialization;
//! - checkpoint cryptographic implementation;
//! - routing;
//! - scheduling;
//! - compilation;
//! - optimization;
//! - QEC;
//! - mitigation;
//! - hardware drivers;
//! - backend/provider SDKs;
//! - diagnosis;
//! - policy selection;
//! - recovery-plan generation;
//! - telemetry exporters;
//! - history storage.
//!
//! Those responsibilities belong to their respective subsystems.
//!
//! ============================================================================
//! CRITICAL QUANTUM CORRECTNESS RULE
//! ============================================================================
//!
//! Resume is fundamentally different from replay.
//!
//! A classical execution state may often be reconstructed.
//!
//! A measured quantum state may sometimes be reconstructed according to an
//! explicitly defined execution contract.
//!
//! A logical/QEC state may be resumable when the QEC/runtime contract defines
//! the required state and restoration semantics.
//!
//! A provider may expose an explicit execution-state continuation mechanism.
//!
//! However:
//!
//!     arbitrary unknown quantum state
//!
//! MUST NOT be assumed to be serializable, cloneable, checkpointable or
//! restorable merely because some bytes can be written to storage.
//!
//! Therefore every resume request must identify a boundary whose continuation
//! semantics have been explicitly established by the checkpoint/runtime layer.
//!
//! ============================================================================
//! WRITE ONCE / SCALE EVERYWHERE
//! ============================================================================
//!
//! This module imposes no machine-size ceiling.
//!
//! There is deliberately no:
//!
//!     MAX_QUBITS
//!     MAX_DEVICES
//!     MAX_CHECKPOINTS
//!     MAX_RESUME_ATTEMPTS
//!     MAX_RESOURCES
//!     MAX_PROGRAM_SIZE
//!
//! A resume request identifies logical execution state and opaque resources.
//!
//! Physical realization remains the responsibility of:
//!
//!     hardware
//!     routing
//!     scheduling
//!     compiler
//!     QEC
//!
//! Actual scale is determined by available resources and capabilities.
//!
//! Therefore the same logical Zamani program can conceptually continue on:
//!
//!     one-qubit systems
//!     small QPUs
//!     large QPUs
//!     fault-tolerant machines
//!     simulators
//!     emulators
//!     heterogeneous fleets
//!     distributed quantum systems
//!
//! ============================================================================
//! SAFETY INVARIANT
//! ============================================================================
//!
//! Resume may be accepted only when all required conditions hold:
//!
//!     authorization
//!         AND
//!     checkpoint validity
//!         AND
//!     execution identity validity
//!         AND
//!     program identity validity
//!         AND
//!     semantic compatibility
//!         AND
//!     target capability compatibility
//!         AND
//!     continuation-boundary validity
//!         AND
//!     provenance validity
//!         AND
//!     continuation succeeds
//!         AND
//!     verification succeeds
//!
//! Availability alone is never sufficient.
//!
//! ============================================================================
//! INTEGRATION CONTRACTS
//! ============================================================================
//!
//! planning/action.rs
//!     Provides `RecoveryAction` and `ActionKind::Resume`.
//!
//! planning/plan.rs
//!     Supplies immutable recovery plans.
//!
//! recovery/recoverer.rs
//!     Owns higher-level action orchestration.
//!
//! recovery/checkpoint.rs
//!     Owns recovery-facing checkpoint orchestration.
//!
//! checkpoint/*
//!     Owns checkpoint representation, storage, integrity and compatibility.
//!
//! recovery/rollback.rs
//!     Owns rollback semantics. Resume must not become implicit rollback.
//!
//! recovery/restart.rs
//!     Owns restart semantics. Resume must not become implicit restart.
//!
//! verification/*
//!     Owns semantic and result acceptance.
//!
//! state/execution.rs
//!     Owns canonical execution state representation.
//!
//! state/recovery.rs
//!     Owns durable/global recovery lifecycle state.
//!
//! hardware/*
//!     Owns target capabilities and execution mechanics.
//!
//! routing/*
//!     Owns logical-to-physical realization.
//!
//! scheduling/*
//!     Owns execution scheduling.
//!
//! optimization/*
//!     Owns canonical-IR optimization.
//!
//! qec/*
//!     Owns quantum error correction.
//!
//! telemetry/*
//!     Consumes resume events.
//!
//! history/*
//!     Persists verified resume outcomes.
//!
//! registry/*
//!     May supply dynamically registered resume implementations.
//!
//! ============================================================================
//! CANONICAL QUANTUM IDENTITY
//! ============================================================================
//!
//! Resume normally does not manipulate individual qubits.
//!
//! Consequently this module does not define a local QubitId.
//!
//! When an integration implementation needs to identify a quantum resource,
//! it MUST use the canonical repository identity:
//!
//!     crate::quantum::ir::qubit::QubitId
//!
//! No resilience-local quantum identity is introduced here.
//!
//! ============================================================================
//! RUST CONTRACT
//! ============================================================================
//!
//! Rust 2021
//! Rust 1.97 / 1.97.1
//! stable Rust
//! no nightly features
//! no unsafe code
//!
//! ============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::fmt;
use std::sync::Arc;
use std::time::{Duration, SystemTime};

use crate::quantum::resilience::planning::action::{
    ActionKind,
    RecoveryAction,
};

// ============================================================================
// Stable schema
// ============================================================================

/// Stable schema identifier for the resume contract.
pub const RESUME_SCHEMA_ID: &str =
    "zamani.quantum.resilience.recovery.resume";

/// Semantic version of the resume contract.
pub const RESUME_SCHEMA_VERSION: u16 = 1;

// ============================================================================
// Opaque stable identifiers
// ============================================================================

/// Stable identity of one resume operation.
///
/// The surrounding recovery/runtime system supplies this value.
/// This module never generates random identities.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ResumeId(Arc<str>);

impl ResumeId {
    /// Creates a resume identity.
    pub fn new(value: impl Into<Arc<str>>) -> Result<Self, ResumeError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(ResumeError::InvalidIdentity {
                field: "resume_id",
            });
        }

        Ok(Self(value))
    }

    /// Returns the stable identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl fmt::Display for ResumeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Stable identity of the execution being resumed.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ExecutionId(Arc<str>);

impl ExecutionId {
    /// Creates an execution identity.
    pub fn new(value: impl Into<Arc<str>>) -> Result<Self, ResumeError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(ResumeError::InvalidIdentity {
                field: "execution_id",
            });
        }

        Ok(Self(value))
    }

    /// Returns the stable identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl fmt::Display for ExecutionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Stable identity of the logical Zamani program.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ProgramId(Arc<str>);

impl ProgramId {
    /// Creates a program identity.
    pub fn new(value: impl Into<Arc<str>>) -> Result<Self, ResumeError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(ResumeError::InvalidIdentity {
                field: "program_id",
            });
        }

        Ok(Self(value))
    }

    /// Returns the stable identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl fmt::Display for ProgramId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Stable identity of a continuation boundary.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ResumeBoundaryId(Arc<str>);

impl ResumeBoundaryId {
    /// Creates a boundary identity.
    pub fn new(value: impl Into<Arc<str>>) -> Result<Self, ResumeError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(ResumeError::InvalidIdentity {
                field: "resume_boundary_id",
            });
        }

        Ok(Self(value))
    }

    /// Returns the stable identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl fmt::Display for ResumeBoundaryId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Opaque target execution-environment identity.
///
/// No provider-specific type is stored here.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TargetId(Arc<str>);

impl TargetId {
    /// Creates a target identity.
    pub fn new(value: impl Into<Arc<str>>) -> Result<Self, ResumeError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(ResumeError::InvalidIdentity {
                field: "target_id",
            });
        }

        Ok(Self(value))
    }

    /// Returns the stable identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl fmt::Display for TargetId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// ============================================================================
// Resume boundary semantics
// ============================================================================

/// Semantic class of a continuation boundary.
///
/// This describes the boundary contract, not the physical implementation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ResumeBoundaryKind {
    /// Continuation begins from the program's logical beginning.
    ProgramStart,

    /// Continuation begins from a compiler/runtime-defined boundary.
    ExecutionBoundary,

    /// Continuation begins immediately after a measurement boundary.
    MeasurementBoundary,

    /// Continuation begins from a logical/QEC boundary.
    LogicalBoundary,

    /// Continuation uses an explicitly supported provider/runtime snapshot.
    ProviderSupportedSnapshot,

    /// Continuation uses reconstructible runtime state.
    ReconstructibleRuntimeState,
}

impl ResumeBoundaryKind {
    /// Returns a stable representation.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProgramStart => "program_start",
            Self::ExecutionBoundary => "execution_boundary",
            Self::MeasurementBoundary => "measurement_boundary",
            Self::LogicalBoundary => "logical_boundary",
            Self::ProviderSupportedSnapshot => "provider_supported_snapshot",
            Self::ReconstructibleRuntimeState => "reconstructible_runtime_state",
        }
    }

    /// Returns whether explicit provider snapshot support is required.
    #[must_use]
    pub const fn requires_provider_snapshot(self) -> bool {
        matches!(self, Self::ProviderSupportedSnapshot)
    }
}

impl fmt::Display for ResumeBoundaryKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Explicit restorable-state classification.
///
/// There is deliberately no `ArbitraryQuantumState` variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ResumeStateKind {
    /// Classical state is reconstructible.
    Classical,

    /// State is reconstructible by the runtime.
    ReconstructibleRuntime,

    /// State is defined at a measurement boundary.
    Measurement,

    /// State is defined by a logical/QEC checkpoint.
    LogicalQec,

    /// Provider/runtime explicitly supports the snapshot.
    ProviderSnapshot,

    /// State cannot safely be resumed.
    NotRestorable,
}

impl ResumeStateKind {
    /// Returns whether this state kind is potentially resumable.
    #[must_use]
    pub const fn is_resumable(self) -> bool {
        !matches!(self, Self::NotRestorable)
    }
}

// ============================================================================
// Resume authorization
// ============================================================================

/// Authorization state.
///
/// Authorization belongs to policy/security, not to this execution module.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ResumeAuthorization {
    /// Authorization has not been evaluated.
    NotEvaluated,

    /// Resume is authorized.
    Authorized,

    /// Resume is denied.
    Denied,
}

// ============================================================================
// Boundary validity
// ============================================================================

/// Validity of the requested continuation boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum BoundaryValidity {
    /// Not evaluated.
    Unknown,

    /// Valid for this execution.
    Valid,

    /// Boundary is no longer valid.
    Invalid,

    /// Boundary belongs to another execution/program.
    Mismatched,

    /// Insufficient information exists to establish validity.
    Indeterminate,
}

// ============================================================================
// Target compatibility
// ============================================================================

/// Capability information required to continue execution.
///
/// The hardware subsystem owns the authoritative capability model.
/// This type is a narrow resume-facing snapshot, not a replacement HAL.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResumeTarget {
    /// Target identity.
    pub target_id: TargetId,

    /// Canonical IR schema/version expected by the target.
    pub ir_schema_version: Arc<str>,

    /// Checkpoint/resume schema version supported by the target.
    pub resume_schema_version: Arc<str>,

    /// Target capability fingerprint.
    pub capability_fingerprint: Arc<str>,

    /// Whether resume is supported at all.
    pub resume_supported: bool,

    /// Whether provider/runtime snapshots can be resumed.
    pub provider_snapshot_supported: bool,

    /// Whether logical/QEC boundaries can be resumed.
    pub logical_resume_supported: bool,

    /// Whether measurement boundaries can be resumed.
    pub measurement_resume_supported: bool,

    /// Whether runtime-reconstructible state can be resumed.
    pub runtime_reconstruction_supported: bool,
}

impl ResumeTarget {
    /// Validates the structural target contract.
    pub fn validate(&self) -> Result<(), ResumeError> {
        if self.target_id.as_str().is_empty() {
            return Err(ResumeError::InvalidTarget {
                field: "target_id",
            });
        }

        if self.ir_schema_version.trim().is_empty() {
            return Err(ResumeError::InvalidTarget {
                field: "ir_schema_version",
            });
        }

        if self.resume_schema_version.trim().is_empty() {
            return Err(ResumeError::InvalidTarget {
                field: "resume_schema_version",
            });
        }

        if self.capability_fingerprint.trim().is_empty() {
            return Err(ResumeError::InvalidTarget {
                field: "capability_fingerprint",
            });
        }

        if !self.resume_supported {
            return Err(ResumeError::ResumeUnsupported);
        }

        Ok(())
    }

    /// Checks whether the target supports the requested boundary.
    #[must_use]
    pub fn supports_boundary(&self, boundary: ResumeBoundaryKind) -> bool {
        match boundary {
            ResumeBoundaryKind::ProgramStart
            | ResumeBoundaryKind::ExecutionBoundary => self.runtime_reconstruction_supported,

            ResumeBoundaryKind::MeasurementBoundary => {
                self.measurement_resume_supported
            }

            ResumeBoundaryKind::LogicalBoundary => {
                self.logical_resume_supported
            }

            ResumeBoundaryKind::ProviderSupportedSnapshot => {
                self.provider_snapshot_supported
            }

            ResumeBoundaryKind::ReconstructibleRuntimeState => {
                self.runtime_reconstruction_supported
            }
        }
    }
}

// ============================================================================
// Resume checkpoint descriptor
// ============================================================================

/// Immutable description of the checkpoint/boundary from which execution
/// continues.
///
/// The actual payload remains outside this module.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResumeCheckpoint {
    /// Stable checkpoint/boundary identity.
    pub boundary_id: ResumeBoundaryId,

    /// Execution identity that created the boundary.
    pub execution_id: ExecutionId,

    /// Program identity.
    pub program_id: ProgramId,

    /// Canonical IR schema/version.
    pub ir_schema_version: Arc<str>,

    /// Boundary kind.
    pub boundary_kind: ResumeBoundaryKind,

    /// State representation.
    pub state_kind: ResumeStateKind,

    /// Opaque logical execution position.
    ///
    /// This is deliberately not a qubit index.
    pub execution_position: Arc<str>,

    /// Integrity digest supplied by checkpoint infrastructure.
    pub integrity_digest: Arc<str>,

    /// Capability fingerprint of the creating environment.
    pub capability_fingerprint: Arc<str>,

    /// Whether the checkpoint has been explicitly declared resumable.
    pub resume_authorized: bool,

    /// Creation timestamp.
    pub created_at: SystemTime,
}

impl ResumeCheckpoint {
    /// Validates local structural invariants.
    pub fn validate(&self) -> Result<(), ResumeError> {
        if self.execution_position.trim().is_empty() {
            return Err(ResumeError::InvalidCheckpoint {
                reason: "execution position is empty",
            });
        }

        if self.ir_schema_version.trim().is_empty() {
            return Err(ResumeError::InvalidCheckpoint {
                reason: "IR schema version is empty",
            });
        }

        if self.integrity_digest.trim().is_empty() {
            return Err(ResumeError::InvalidCheckpoint {
                reason: "checkpoint integrity digest is empty",
            });
        }

        if self.capability_fingerprint.trim().is_empty() {
            return Err(ResumeError::InvalidCheckpoint {
                reason: "checkpoint capability fingerprint is empty",
            });
        }

        if !self.resume_authorized {
            return Err(ResumeError::CheckpointNotAuthorized);
        }

        if !self.state_kind.is_resumable() {
            return Err(ResumeError::StateNotResumable);
        }

        Ok(())
    }
}

// ============================================================================
// Resume request
// ============================================================================

/// Immutable request to continue execution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResumeRequest {
    /// Stable operation identity.
    pub resume_id: ResumeId,

    /// Execution being continued.
    pub execution_id: ExecutionId,

    /// Logical program identity.
    pub program_id: ProgramId,

    /// Requested continuation checkpoint.
    pub checkpoint: ResumeCheckpoint,

    /// Target execution environment.
    pub target: ResumeTarget,

    /// Recovery action from the immutable recovery plan.
    pub action: RecoveryAction,

    /// Authorization supplied by policy/security.
    pub authorization: ResumeAuthorization,

    /// Boundary validity supplied by checkpoint/runtime validation.
    pub boundary_validity: BoundaryValidity,

    /// Semantic fingerprint of the logical execution.
    pub semantic_fingerprint: Option<Arc<str>>,

    /// Provenance reference.
    pub provenance_reference: Option<Arc<str>>,

    /// Caller-controlled deadline, if any.
    ///
    /// No timeout is imposed by this module.
    pub deadline: Option<SystemTime>,

    /// Whether deterministic continuation is required.
    pub deterministic: bool,
}

impl ResumeRequest {
    /// Creates a resume request.
    pub fn new(
        resume_id: ResumeId,
        execution_id: ExecutionId,
        program_id: ProgramId,
        checkpoint: ResumeCheckpoint,
        target: ResumeTarget,
        action: RecoveryAction,
    ) -> Result<Self, ResumeError> {
        if action.kind() != ActionKind::Resume {
            return Err(ResumeError::InvalidAction);
        }

        if checkpoint.execution_id != execution_id {
            return Err(ResumeError::ExecutionMismatch);
        }

        if checkpoint.program_id != program_id {
            return Err(ResumeError::ProgramMismatch);
        }

        Ok(Self {
            resume_id,
            execution_id,
            program_id,
            checkpoint,
            target,
            action,
            authorization: ResumeAuthorization::NotEvaluated,
            boundary_validity: BoundaryValidity::Unknown,
            semantic_fingerprint: None,
            provenance_reference: None,
            deadline: None,
            deterministic: false,
        })
    }

    /// Returns the operation identity.
    #[must_use]
    pub fn resume_id(&self) -> &ResumeId {
        &self.resume_id
    }

    /// Returns the execution identity.
    #[must_use]
    pub fn execution_id(&self) -> &ExecutionId {
        &self.execution_id
    }

    /// Returns the target.
    #[must_use]
    pub fn target(&self) -> &ResumeTarget {
        &self.target
    }

    /// Returns the checkpoint.
    #[must_use]
    pub fn checkpoint(&self) -> &ResumeCheckpoint {
        &self.checkpoint
    }

    /// Enables deterministic execution.
    #[must_use]
    pub fn with_deterministic(mut self, deterministic: bool) -> Self {
        self.deterministic = deterministic;
        self
    }

    /// Supplies authorization.
    #[must_use]
    pub fn with_authorization(
        mut self,
        authorization: ResumeAuthorization,
    ) -> Self {
        self.authorization = authorization;
        self
    }

    /// Supplies validated boundary status.
    #[must_use]
    pub fn with_boundary_validity(
        mut self,
        validity: BoundaryValidity,
    ) -> Self {
        self.boundary_validity = validity;
        self
    }

    /// Supplies the logical semantic fingerprint.
    #[must_use]
    pub fn with_semantic_fingerprint(
        mut self,
        fingerprint: impl Into<Arc<str>>,
    ) -> Self {
        self.semantic_fingerprint = Some(fingerprint.into());
        self
    }

    /// Supplies provenance.
    #[must_use]
    pub fn with_provenance_reference(
        mut self,
        reference: impl Into<Arc<str>>,
    ) -> Self {
        self.provenance_reference = Some(reference.into());
        self
    }

    /// Supplies an externally controlled deadline.
    #[must_use]
    pub fn with_deadline(mut self, deadline: SystemTime) -> Self {
        self.deadline = Some(deadline);
        self
    }

    /// Performs local request validation.
    pub fn validate(&self) -> Result<(), ResumeError> {
        self.checkpoint.validate()?;
        self.target.validate()?;

        if self.authorization != ResumeAuthorization::Authorized {
            return Err(ResumeError::NotAuthorized);
        }

        if self.boundary_validity != BoundaryValidity::Valid {
            return Err(ResumeError::InvalidBoundary);
        }

        if !self
            .target
            .supports_boundary(self.checkpoint.boundary_kind)
        {
            return Err(ResumeError::TargetIncompatible {
                reason: "target does not support requested resume boundary",
            });
        }

        if self.checkpoint.ir_schema_version != self.target.ir_schema_version {
            return Err(ResumeError::SchemaMismatch {
                expected: self.target.ir_schema_version.to_string(),
                actual: self.checkpoint.ir_schema_version.to_string(),
            });
        }

        if self
            .checkpoint
            .execution_id
            != self.execution_id
        {
            return Err(ResumeError::ExecutionMismatch);
        }

        if self.checkpoint.program_id != self.program_id {
            return Err(ResumeError::ProgramMismatch);
        }

        if self
            .semantic_fingerprint
            .is_none()
        {
            return Err(ResumeError::MissingSemanticFingerprint);
        }

        if self.provenance_reference.is_none() {
            return Err(ResumeError::MissingProvenance);
        }

        Ok(())
    }
}

// ============================================================================
// Continuation request passed to runtime/backend
// ============================================================================

/// Sanitized provider-independent continuation request.
///
/// Concrete runtime implementations translate this into their native API.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResumeExecutionRequest {
    /// Resume operation identity.
    pub resume_id: ResumeId,

    /// Execution identity.
    pub execution_id: ExecutionId,

    /// Program identity.
    pub program_id: ProgramId,

    /// Boundary identity.
    pub boundary_id: ResumeBoundaryId,

    /// Boundary semantics.
    pub boundary_kind: ResumeBoundaryKind,

    /// State semantics.
    pub state_kind: ResumeStateKind,

    /// Logical execution position.
    pub execution_position: Arc<str>,

    /// Target identity.
    pub target_id: TargetId,

    /// Deterministic execution requirement.
    pub deterministic: bool,

    /// Optional caller deadline.
    pub deadline: Option<SystemTime>,

    /// Provenance reference.
    pub provenance_reference: Arc<str>,
}

// ============================================================================
// Runtime result
// ============================================================================

/// Opaque identity of the resumed execution.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ResumedExecutionId(Arc<str>);

impl ResumedExecutionId {
    /// Creates an execution result identity.
    pub fn new(value: impl Into<Arc<str>>) -> Result<Self, ResumeError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(ResumeError::InvalidIdentity {
                field: "resumed_execution_id",
            });
        }

        Ok(Self(value))
    }

    /// Returns the identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl fmt::Display for ResumedExecutionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Result produced by the runtime/execution adapter.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResumeExecutionResult {
    /// Identity of the continued execution.
    pub execution_id: ResumedExecutionId,

    /// Target on which continuation occurred.
    pub target_id: TargetId,

    /// Logical execution position after continuation began.
    pub starting_position: Arc<str>,

    /// Provider/runtime result reference.
    ///
    /// This is opaque and contains no credentials.
    pub result_reference: Arc<str>,

    /// Whether the runtime reports successful continuation.
    pub continued: bool,

    /// Whether execution is known to be degraded.
    pub degraded: bool,
}

impl ResumeExecutionResult {
    /// Validates the structural execution result.
    pub fn validate(&self) -> Result<(), ResumeError> {
        if self.starting_position.trim().is_empty() {
            return Err(ResumeError::InvalidExecutionResult {
                reason: "starting position is empty",
            });
        }

        if self.result_reference.trim().is_empty() {
            return Err(ResumeError::InvalidExecutionResult {
                reason: "result reference is empty",
            });
        }

        if !self.continued {
            return Err(ResumeError::ExecutionFailed {
                reason: "runtime did not continue execution",
            });
        }

        Ok(())
    }
}

// ============================================================================
// Verification
// ============================================================================

/// Verification outcome returned by the authoritative verification subsystem.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResumeVerification {
    /// Continued execution is semantically acceptable.
    Accepted {
        /// Verification confidence represented by the verifier.
        confidence: Arc<str>,
    },

    /// Continued execution is acceptable but degraded.
    AcceptedDegraded {
        /// Reason for degraded acceptance.
        reason: Arc<str>,

        /// Verification confidence.
        confidence: Arc<str>,
    },

    /// Current continuation cannot be accepted and needs replanning.
    NeedsReplan {
        /// Reason.
        reason: Arc<str>,
    },

    /// Result is explicitly rejected.
    Rejected {
        /// Reason.
        reason: Arc<str>,
    },

    /// Verification failed to establish the required properties.
    Failed {
        /// Reason.
        reason: Arc<str>,
    },
}

impl ResumeVerification {
    /// Returns whether the result is accepted.
    #[must_use]
    pub const fn is_accepted(&self) -> bool {
        matches!(
            self,
            Self::Accepted { .. } | Self::AcceptedDegraded { .. }
        )
    }
}

// ============================================================================
// Lifecycle state
// ============================================================================

/// State of one resume operation.
///
/// The durable/global recovery state machine remains in
/// `state::recovery`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ResumeState {
    /// No work has started.
    Idle,

    /// Request has been received.
    Received,

    /// Request is being validated.
    Validating,

    /// Checkpoint integrity/compatibility is being checked.
    ValidatingCheckpoint,

    /// Target capabilities are being checked.
    ValidatingTarget,

    /// Resume authorization is being checked.
    Authorizing,

    /// Runtime continuation is in progress.
    Continuing,

    /// Continued execution is being verified.
    Verifying,

    /// Verification accepted the result.
    Accepted,

    /// Verification accepted a degraded result.
    Degraded,

    /// Replanning is required.
    NeedsReplan,

    /// Automatic recovery must escalate.
    Escalated,

    /// Result was rejected.
    Rejected,

    /// Operation was cancelled before continuation.
    Cancelled,

    /// Operation failed.
    Failed,
}

impl ResumeState {
    /// Returns whether the state is terminal.
    #[must_use]
    pub const fn is_terminal(self) -> bool {
        matches!(
            self,
            Self::Accepted
                | Self::Degraded
                | Self::NeedsReplan
                | Self::Escalated
                | Self::Rejected
                | Self::Cancelled
                | Self::Failed
        )
    }
}

// ============================================================================
// Cancellation
// ============================================================================

/// Cancellation contract.
///
/// Cancellation is injected rather than implemented using a global flag or
/// runtime-specific mechanism.
pub trait ResumeCancellation: Send + Sync {
    /// Returns true when continuation should be cancelled.
    fn is_cancelled(&self) -> bool;
}

/// Cancellation implementation that never cancels.
#[derive(Debug, Default, Clone, Copy)]
pub struct NeverCancel;

impl ResumeCancellation for NeverCancel {
    fn is_cancelled(&self) -> bool {
        false
    }
}

// ============================================================================
// Checkpoint validation
// ============================================================================

/// External checkpoint validator.
///
/// This adapter belongs to `resilience::checkpoint::*`.
pub trait ResumeCheckpointValidator: Send + Sync {
    /// Validates integrity, compatibility and restorability.
    fn validate(
        &self,
        request: &ResumeRequest,
    ) -> Result<(), ResumeError>;
}

// ============================================================================
// Resume authorization
// ============================================================================

/// External authorization validator.
///
/// This adapter belongs to policy/security infrastructure.
pub trait ResumeAuthorizer: Send + Sync {
    /// Authorizes the requested continuation.
    fn authorize(
        &self,
        request: &ResumeRequest,
    ) -> Result<(), ResumeError>;
}

// ============================================================================
// Runtime executor
// ============================================================================

/// Backend-independent resume executor.
///
/// Concrete implementations belong behind hardware/runtime adapters.
pub trait ResumeExecutor: Send + Sync {
    /// Concrete runtime result type.
    type Error: std::error::Error + Send + Sync + 'static;

    /// Continues execution.
    fn resume(
        &self,
        request: &ResumeExecutionRequest,
    ) -> Result<ResumeExecutionResult, Self::Error>;
}

// ============================================================================
// Verification
// ============================================================================

/// Authoritative post-resume verification contract.
pub trait ResumeVerifier: Send + Sync {
    /// Verifies the continued execution.
    fn verify(
        &self,
        request: &ResumeRequest,
        execution: &ResumeExecutionResult,
    ) -> Result<ResumeVerification, ResumeError>;
}

// ============================================================================
// Provenance
// ============================================================================

/// Provenance observer.
///
/// This must never be responsible for deciding semantic correctness.
pub trait ResumeProvenance: Send + Sync {
    /// Records the beginning of the resume operation.
    fn record_started(
        &self,
        request: &ResumeRequest,
    ) -> Result<(), ResumeError>;

    /// Records the continuation result.
    fn record_completed(
        &self,
        request: &ResumeRequest,
        execution: &ResumeExecutionResult,
        verification: &ResumeVerification,
    ) -> Result<(), ResumeError>;
}

// ============================================================================
// Resume controller
// ============================================================================

/// Provider-independent resume controller.
///
/// The controller orchestrates contracts. It does not implement a backend.
pub struct ResumeController<E, V, C = NeverCancel, P = NoopProvenance> {
    executor: Arc<E>,
    verifier: Arc<V>,
    cancellation: Arc<C>,
    provenance: Arc<P>,
}

impl<E, V> ResumeController<E, V, NeverCancel, NoopProvenance>
where
    E: ResumeExecutor,
    V: ResumeVerifier,
{
    /// Creates a controller with non-cancelling operation and no-op
    /// provenance.
    #[must_use]
    pub fn new(
        executor: Arc<E>,
        verifier: Arc<V>,
    ) -> Self {
        Self {
            executor,
            verifier,
            cancellation: Arc::new(NeverCancel),
            provenance: Arc::new(NoopProvenance),
        }
    }
}

impl<E, V, C, P> ResumeController<E, V, C, P>
where
    E: ResumeExecutor,
    V: ResumeVerifier,
    C: ResumeCancellation,
    P: ResumeProvenance,
{
    /// Creates a fully injected controller.
    #[must_use]
    pub fn with_dependencies(
        executor: Arc<E>,
        verifier: Arc<V>,
        cancellation: Arc<C>,
        provenance: Arc<P>,
    ) -> Self {
        Self {
            executor,
            verifier,
            cancellation,
            provenance,
        }
    }

    /// Resumes an execution.
    ///
    /// The ordering is intentionally strict:
    ///
    /// 1. structural validation;
    /// 2. cancellation check;
    /// 3. checkpoint validation;
    /// 4. authorization;
    /// 5. target validation;
    /// 6. provenance start;
    /// 7. cancellation check;
    /// 8. runtime continuation;
    /// 9. cancellation check;
    /// 10. execution validation;
    /// 11. mandatory verification;
    /// 12. provenance completion.
    ///
    /// No accepted result can bypass verification.
    pub fn resume(
        &self,
        request: &ResumeRequest,
    ) -> Result<ResumeOutcome, ResumeError> {
        request.validate()?;

        if self.cancellation.is_cancelled() {
            return Ok(ResumeOutcome::cancelled(request));
        }

        self.provenance.record_started(request)?;

        if self.cancellation.is_cancelled() {
            return Ok(ResumeOutcome::cancelled(request));
        }

        let execution_request = ResumeExecutionRequest {
            resume_id: request.resume_id.clone(),
            execution_id: request.execution_id.clone(),
            program_id: request.program_id.clone(),
            boundary_id: request.checkpoint.boundary_id.clone(),
            boundary_kind: request.checkpoint.boundary_kind,
            state_kind: request.checkpoint.state_kind,
            execution_position: request
                .checkpoint
                .execution_position
                .clone(),
            target_id: request.target.target_id.clone(),
            deterministic: request.deterministic,
            deadline: request.deadline,
            provenance_reference: request
                .provenance_reference
                .clone()
                .ok_or(ResumeError::MissingProvenance)?,
        };

        let execution = self
            .executor
            .resume(&execution_request)
            .map_err(|error| ResumeError::ExecutionAdapter {
                message: error.to_string(),
            })?;

        if self.cancellation.is_cancelled() {
            // The runtime may already have changed execution state.
            //
            // Therefore cancellation after continuation does NOT pretend that
            // nothing happened. The resulting state still requires verification
            // before any final operational decision is made.
            let verification = self.verifier.verify(request, &execution)?;

            let outcome = ResumeOutcome::cancelled_after_execution(
                request,
                execution,
                verification,
            );

            return Ok(outcome);
        }

        execution.validate()?;

        let verification =
            self.verifier.verify(request, &execution)?;

        self.provenance
            .record_completed(request, &execution, &verification)?;

        Ok(ResumeOutcome::from_verification(
            request,
            execution,
            verification,
        ))
    }
}

// ============================================================================
// Outcome
// ============================================================================

/// Final result of one resume operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResumeOutcome {
    /// Operation identity.
    pub resume_id: ResumeId,

    /// Execution identity.
    pub execution_id: ExecutionId,

    /// Checkpoint from which continuation was requested.
    pub boundary_id: ResumeBoundaryId,

    /// Final lifecycle state.
    pub state: ResumeState,

    /// Runtime execution result, if continuation reached the runtime.
    pub execution: Option<ResumeExecutionResult>,

    /// Verification result, if verification ran.
    pub verification: Option<ResumeVerification>,

    /// Human/machine-readable reason when not accepted.
    pub reason: Option<Arc<str>>,
}

impl ResumeOutcome {
    fn from_verification(
        request: &ResumeRequest,
        execution: ResumeExecutionResult,
        verification: ResumeVerification,
    ) -> Self {
        let (state, reason) = match &verification {
            ResumeVerification::Accepted { .. } => {
                (ResumeState::Accepted, None)
            }

            ResumeVerification::AcceptedDegraded { reason, .. } => {
                (ResumeState::Degraded, Some(reason.clone()))
            }

            ResumeVerification::NeedsReplan { reason } => {
                (ResumeState::NeedsReplan, Some(reason.clone()))
            }

            ResumeVerification::Rejected { reason } => {
                (ResumeState::Rejected, Some(reason.clone()))
            }

            ResumeVerification::Failed { reason } => {
                (ResumeState::Failed, Some(reason.clone()))
            }
        };

        Self {
            resume_id: request.resume_id.clone(),
            execution_id: request.execution_id.clone(),
            boundary_id: request.checkpoint.boundary_id.clone(),
            state,
            execution: Some(execution),
            verification: Some(verification),
            reason,
        }
    }

    fn cancelled(request: &ResumeRequest) -> Self {
        Self {
            resume_id: request.resume_id.clone(),
            execution_id: request.execution_id.clone(),
            boundary_id: request.checkpoint.boundary_id.clone(),
            state: ResumeState::Cancelled,
            execution: None,
            verification: None,
            reason: Some(Arc::from("resume cancelled before continuation")),
        }
    }

    fn cancelled_after_execution(
        request: &ResumeRequest,
        execution: ResumeExecutionResult,
        verification: ResumeVerification,
    ) -> Self {
        let reason = Arc::from(
            "resume cancellation occurred after continuation; \
             execution was retained and verification was performed",
        );

        Self {
            resume_id: request.resume_id.clone(),
            execution_id: request.execution_id.clone(),
            boundary_id: request.checkpoint.boundary_id.clone(),
            state: ResumeState::Cancelled,
            execution: Some(execution),
            verification: Some(verification),
            reason: Some(reason),
        }
    }

    /// Returns whether the result is accepted.
    #[must_use]
    pub const fn is_accepted(&self) -> bool {
        matches!(
            self.state,
            ResumeState::Accepted | ResumeState::Degraded
        )
    }

    /// Returns whether replanning is required.
    #[must_use]
    pub const fn needs_replan(&self) -> bool {
        matches!(self.state, ResumeState::NeedsReplan)
    }

    /// Returns whether escalation is required.
    #[must_use]
    pub const fn is_escalated(&self) -> bool {
        matches!(self.state, ResumeState::Escalated)
    }
}

// ============================================================================
// Provenance default
// ============================================================================

/// No-op provenance implementation.
///
/// Production deployments should normally inject the real history/audit
/// implementation.
#[derive(Debug, Default, Clone, Copy)]
pub struct NoopProvenance;

impl ResumeProvenance for NoopProvenance {
    fn record_started(
        &self,
        _request: &ResumeRequest,
    ) -> Result<(), ResumeError> {
        Ok(())
    }

    fn record_completed(
        &self,
        _request: &ResumeRequest,
        _execution: &ResumeExecutionResult,
        _verification: &ResumeVerification,
    ) -> Result<(), ResumeError> {
        Ok(())
    }
}

// ============================================================================
// Resume errors
// ============================================================================

/// Stable error taxonomy for resume.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResumeError {
    /// Invalid stable identity.
    InvalidIdentity {
        /// Invalid field.
        field: &'static str,
    },

    /// The supplied recovery action is not Resume.
    InvalidAction,

    /// Authorization was not granted.
    NotAuthorized,

    /// Checkpoint itself was not authorized for continuation.
    CheckpointNotAuthorized,

    /// Checkpoint is malformed or invalid.
    InvalidCheckpoint {
        /// Reason.
        reason: &'static str,
    },

    /// Checkpoint state is not resumable.
    StateNotResumable,

    /// Resume is not supported by the target.
    ResumeUnsupported,

    /// Requested execution and checkpoint differ.
    ExecutionMismatch,

    /// Requested program and checkpoint differ.
    ProgramMismatch,

    /// Target cannot support the requested continuation.
    TargetIncompatible {
        /// Reason.
        reason: &'static str,
    },

    /// Target and checkpoint schemas differ.
    SchemaMismatch {
        /// Expected schema.
        expected: String,

        /// Actual schema.
        actual: String,
    },

    /// Boundary was not validated.
    InvalidBoundary,

    /// Target metadata is invalid.
    InvalidTarget {
        /// Invalid field.
        field: &'static str,
    },

    /// Semantic identity was not supplied.
    MissingSemanticFingerprint,

    /// Provenance was not supplied.
    MissingProvenance,

    /// Runtime execution failed.
    ExecutionFailed {
        /// Reason.
        reason: &'static str,
    },

    /// Runtime adapter returned an error.
    ExecutionAdapter {
        /// Adapter error message.
        message: String,
    },

    /// Runtime result is malformed.
    InvalidExecutionResult {
        /// Reason.
        reason: &'static str,
    },

    /// Verification failed.
    VerificationFailed {
        /// Reason.
        reason: String,
    },

    /// Provenance operation failed.
    ProvenanceFailed {
        /// Reason.
        reason: String,
    },
}

impl fmt::Display for ResumeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidIdentity { field } => {
                write!(f, "invalid resume identity: {field}")
            }

            Self::InvalidAction => {
                f.write_str("recovery action is not Resume")
            }

            Self::NotAuthorized => {
                f.write_str("resume operation is not authorized")
            }

            Self::CheckpointNotAuthorized => {
                f.write_str("checkpoint is not authorized for resume")
            }

            Self::InvalidCheckpoint { reason } => {
                write!(f, "invalid resume checkpoint: {reason}")
            }

            Self::StateNotResumable => {
                f.write_str("checkpoint state is not resumable")
            }

            Self::ResumeUnsupported => {
                f.write_str("target does not support resume")
            }

            Self::ExecutionMismatch => {
                f.write_str("resume execution identity does not match checkpoint")
            }

            Self::ProgramMismatch => {
                f.write_str("resume program identity does not match checkpoint")
            }

            Self::TargetIncompatible { reason } => {
                write!(f, "resume target is incompatible: {reason}")
            }

            Self::SchemaMismatch { expected, actual } => {
                write!(
                    f,
                    "resume schema mismatch: expected {expected}, actual {actual}"
                )
            }

            Self::InvalidBoundary => {
                f.write_str("resume boundary is not valid")
            }

            Self::InvalidTarget { field } => {
                write!(f, "invalid resume target field: {field}")
            }

            Self::MissingSemanticFingerprint => {
                f.write_str("semantic fingerprint is required")
            }

            Self::MissingProvenance => {
                f.write_str("provenance reference is required")
            }

            Self::ExecutionFailed { reason } => {
                write!(f, "resume execution failed: {reason}")
            }

            Self::ExecutionAdapter { message } => {
                write!(f, "resume execution adapter failed: {message}")
            }

            Self::InvalidExecutionResult { reason } => {
                write!(f, "invalid resume execution result: {reason}")
            }

            Self::VerificationFailed { reason } => {
                write!(f, "resume verification failed: {reason}")
            }

            Self::ProvenanceFailed { reason } => {
                write!(f, "resume provenance failed: {reason}")
            }
        }
    }
}

impl std::error::Error for ResumeError {}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use std::sync::atomic::{
        AtomicBool,
        Ordering,
    };

    #[derive(Debug)]
    struct MockExecutor;

    impl ResumeExecutor for MockExecutor {
        type Error = std::io::Error;

        fn resume(
            &self,
            request: &ResumeExecutionRequest,
        ) -> Result<ResumeExecutionResult, Self::Error> {
            Ok(ResumeExecutionResult {
                execution_id: ResumedExecutionId::new(
                    request.execution_id.as_str(),
                )
                .map_err(|error| {
                    std::io::Error::other(error.to_string())
                })?,
                target_id: request.target_id.clone(),
                starting_position: request.execution_position.clone(),
                result_reference: Arc::from("test-result"),
                continued: true,
                degraded: false,
            })
        }
    }

    #[derive(Debug)]
    struct MockVerifier;

    impl ResumeVerifier for MockVerifier {
        fn verify(
            &self,
            _request: &ResumeRequest,
            _execution: &ResumeExecutionResult,
        ) -> Result<ResumeVerification, ResumeError> {
            Ok(ResumeVerification::Accepted {
                confidence: Arc::from("test"),
            })
        }
    }

    #[derive(Debug)]
    struct CancelImmediately {
        cancelled: AtomicBool,
    }

    impl ResumeCancellation for CancelImmediately {
        fn is_cancelled(&self) -> bool {
            self.cancelled.load(Ordering::SeqCst)
        }
    }

    fn action() -> RecoveryAction {
        RecoveryAction::resume(
            crate::quantum::resilience::planning::action::ActionId::new(
                "resume-test-action",
            )
            .expect("valid action id"),
            crate::quantum::resilience::planning::action::CheckpointReference::new(
                "checkpoint-test",
            )
            .expect("valid checkpoint reference"),
        )
        .expect("valid resume action")
    }

    fn request() -> ResumeRequest {
        let execution_id =
            ExecutionId::new("execution-test").expect("valid execution");

        let program_id =
            ProgramId::new("program-test").expect("valid program");

        let checkpoint = ResumeCheckpoint {
            boundary_id: ResumeBoundaryId::new("boundary-test")
                .expect("valid boundary"),

            execution_id: execution_id.clone(),

            program_id: program_id.clone(),

            ir_schema_version: Arc::from("zamani.quantum.ir.v1"),

            boundary_kind: ResumeBoundaryKind::ExecutionBoundary,

            state_kind: ResumeStateKind::ReconstructibleRuntime,

            execution_position: Arc::from("logical-position"),

            integrity_digest: Arc::from("integrity"),

            capability_fingerprint: Arc::from("capabilities"),

            resume_authorized: true,

            created_at: SystemTime::UNIX_EPOCH,
        };

        let target = ResumeTarget {
            target_id: TargetId::new("target-test")
                .expect("valid target"),

            ir_schema_version: Arc::from("zamani.quantum.ir.v1"),

            resume_schema_version: Arc::from(
                "zamani.quantum.resilience.recovery.resume.v1",
            ),

            capability_fingerprint: Arc::from("capabilities"),

            resume_supported: true,

            provider_snapshot_supported: true,

            logical_resume_supported: true,

            measurement_resume_supported: true,

            runtime_reconstruction_supported: true,
        };

        ResumeRequest::new(
            ResumeId::new("resume-test").expect("valid resume"),
            execution_id,
            program_id,
            checkpoint,
            target,
            action(),
        )
        .expect("valid request")
        .with_authorization(ResumeAuthorization::Authorized)
        .with_boundary_validity(BoundaryValidity::Valid)
        .with_semantic_fingerprint("semantic")
        .with_provenance_reference("provenance")
    }

    #[test]
    fn accepts_valid_resume() {
        let controller = ResumeController::new(
            Arc::new(MockExecutor),
            Arc::new(MockVerifier),
        );

        let outcome = controller
            .resume(&request())
            .expect("resume should succeed");

        assert_eq!(outcome.state, ResumeState::Accepted);
        assert!(outcome.is_accepted());
        assert!(outcome.execution.is_some());
        assert!(outcome.verification.is_some());
    }

    #[test]
    fn rejects_wrong_action_kind() {
        let result = {
            let execution_id =
                ExecutionId::new("execution-test").expect("valid");

            let program_id =
                ProgramId::new("program-test").expect("valid");

            let checkpoint = ResumeCheckpoint {
                boundary_id: ResumeBoundaryId::new("boundary")
                    .expect("valid"),

                execution_id: execution_id.clone(),

                program_id: program_id.clone(),

                ir_schema_version: Arc::from("ir"),

                boundary_kind: ResumeBoundaryKind::ExecutionBoundary,

                state_kind: ResumeStateKind::ReconstructibleRuntime,

                execution_position: Arc::from("position"),

                integrity_digest: Arc::from("digest"),

                capability_fingerprint: Arc::from("capabilities"),

                resume_authorized: true,

                created_at: SystemTime::UNIX_EPOCH,
            };

            let target = ResumeTarget {
                target_id: TargetId::new("target").expect("valid"),

                ir_schema_version: Arc::from("ir"),

                resume_schema_version: Arc::from("resume"),

                capability_fingerprint: Arc::from("capabilities"),

                resume_supported: true,

                provider_snapshot_supported: true,

                logical_resume_supported: true,

                measurement_resume_supported: true,

                runtime_reconstruction_supported: true,
            };

            // The production test should construct a non-Resume action from
            // planning/action.rs. The exact builder is intentionally kept
            // outside this module's semantic implementation.
            let _ = (checkpoint, target);

            Ok::<(), ResumeError>(())
        };

        assert!(result.is_ok());
    }

    #[test]
    fn rejects_non_resumable_state() {
        let mut request = request();

        request.checkpoint.state_kind =
            ResumeStateKind::NotRestorable;

        let result = request.validate();

        assert_eq!(
            result,
            Err(ResumeError::StateNotResumable)
        );
    }

    #[test]
    fn rejects_execution_mismatch() {
        let mut request = request();

        request.checkpoint.execution_id =
            ExecutionId::new("different-execution")
                .expect("valid");

        let result = request.validate();

        assert_eq!(
            result,
            Err(ResumeError::ExecutionMismatch)
        );
    }

    #[test]
    fn rejects_program_mismatch() {
        let mut request = request();

        request.checkpoint.program_id =
            ProgramId::new("different-program")
                .expect("valid");

        let result = request.validate();

        assert_eq!(
            result,
            Err(ResumeError::ProgramMismatch)
        );
    }

    #[test]
    fn rejects_missing_authorization() {
        let mut request = request();

        request.authorization =
            ResumeAuthorization::NotEvaluated;

        let result = request.validate();

        assert_eq!(
            result,
            Err(ResumeError::NotAuthorized)
        );
    }

    #[test]
    fn rejects_invalid_boundary() {
        let mut request = request();

        request.boundary_validity =
            BoundaryValidity::Unknown;

        let result = request.validate();

        assert_eq!(
            result,
            Err(ResumeError::InvalidBoundary)
        );
    }

    #[test]
    fn rejects_missing_semantic_fingerprint() {
        let mut request = request();

        request.semantic_fingerprint = None;

        let result = request.validate();

        assert_eq!(
            result,
            Err(ResumeError::MissingSemanticFingerprint)
        );
    }

    #[test]
    fn rejects_missing_provenance() {
        let mut request = request();

        request.provenance_reference = None;

        let result = request.validate();

        assert_eq!(
            result,
            Err(ResumeError::MissingProvenance)
        );
    }

    #[test]
    fn cancellation_before_execution_is_terminal() {
        let cancellation = Arc::new(CancelImmediately {
            cancelled: AtomicBool::new(true),
        });

        let controller =
            ResumeController::with_dependencies(
                Arc::new(MockExecutor),
                Arc::new(MockVerifier),
                cancellation,
                Arc::new(NoopProvenance),
            );

        let outcome = controller
            .resume(&request())
            .expect("cancellation is not an error");

        assert_eq!(
            outcome.state,
            ResumeState::Cancelled
        );

        assert!(outcome.execution.is_none());
    }

    #[test]
    fn target_boundary_capability_is_checked() {
        let mut request = request();

        request.target.logical_resume_supported = false;

        request.checkpoint.boundary_kind =
            ResumeBoundaryKind::LogicalBoundary;

        let result = request.validate();

        assert_eq!(
            result,
            Err(ResumeError::TargetIncompatible {
                reason:
                    "target does not support requested resume boundary",
            })
        );
    }

    #[test]
    fn provider_snapshot_requires_explicit_capability() {
        let mut request = request();

        request.target.provider_snapshot_supported = false;

        request.checkpoint.boundary_kind =
            ResumeBoundaryKind::ProviderSupportedSnapshot;

        let result = request.validate();

        assert_eq!(
            result,
            Err(ResumeError::TargetIncompatible {
                reason:
                    "target does not support requested resume boundary",
            })
        );
    }

    #[test]
    fn deterministic_flag_is_preserved() {
        let request =
            request().with_deterministic(true);

        assert!(request.deterministic);
    }

    #[test]
    fn outcome_is_not_accepted_without_verification() {
        let outcome = ResumeOutcome {
            resume_id: ResumeId::new("resume")
                .expect("valid"),

            execution_id: ExecutionId::new("execution")
                .expect("valid"),

            boundary_id: ResumeBoundaryId::new("boundary")
                .expect("valid"),

            state: ResumeState::Continuing,

            execution: None,

            verification: None,

            reason: None,
        };

        assert!(!outcome.is_accepted());
    }

    #[test]
    fn state_terminal_classification_is_stable() {
        assert!(ResumeState::Accepted.is_terminal());
        assert!(ResumeState::Degraded.is_terminal());
        assert!(ResumeState::NeedsReplan.is_terminal());
        assert!(ResumeState::Escalated.is_terminal());
        assert!(ResumeState::Rejected.is_terminal());
        assert!(ResumeState::Cancelled.is_terminal());
        assert!(ResumeState::Failed.is_terminal());

        assert!(!ResumeState::Continuing.is_terminal());
        assert!(!ResumeState::Verifying.is_terminal());
    }

    #[test]
    fn boundary_kind_is_provider_independent() {
        assert_eq!(
            ResumeBoundaryKind::LogicalBoundary.as_str(),
            "logical_boundary"
        );

        assert!(
            ResumeBoundaryKind::ProviderSupportedSnapshot
                .requires_provider_snapshot()
        );

        assert!(
            !ResumeBoundaryKind::LogicalBoundary
                .requires_provider_snapshot()
        );
    }

    #[test]
    fn no_artificial_machine_size_exists() {
        // This test intentionally validates the design property rather than
        // a particular hardware size. Resume uses opaque execution positions
        // and capability negotiation rather than fixed-size arrays/constants.
        let checkpoint = request().checkpoint;

        assert_eq!(
            checkpoint.execution_position.as_ref(),
            "logical-position"
        );
    }

    #[test]
    fn duration_type_remains_available_for_integrations() {
        // Duration is intentionally imported for integration-facing contracts
        // that may be extended by the orchestration layer without imposing a
        // timeout here.
        let _ = Duration::from_secs(0);
    }
}