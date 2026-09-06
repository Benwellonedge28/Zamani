//! Zamani Quantum Resilience — Safe Rollback Recovery
//!
//! Path:
//!     src/quantum/resilience/recovery/rollback.rs
//!
//! Purpose:
//!     Provides the provider-independent rollback execution contract for
//!     `quantum::resilience`.
//!
//! ----------------------------------------------------------------------------
//! Architectural position
//! ----------------------------------------------------------------------------
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
//!     RecoveryAction::Rollback
//!         |
//!         v
//!     RollbackExecutor                <-- this module
//!         |
//!         +--> checkpoint/state provider
//!         +--> execution provider
//!         +--> capability validation
//!         +--> semantic validation
//!         +--> authorization
//!         |
//!         v
//!     Verification
//!         |
//!         +--> ACCEPT
//!         +--> REJECT
//!         +--> ESCALATE
//!
//! ----------------------------------------------------------------------------
//! Responsibility
//! ----------------------------------------------------------------------------
//!
//! This module owns the EXECUTION CONTRACT for rollback.
//!
//! It does NOT own:
//!
//! - fault detection;
//! - fault diagnosis;
//! - recovery planning;
//! - policy decisions;
//! - checkpoint creation;
//! - checkpoint storage implementation;
//! - quantum-state serialization;
//! - routing;
//! - scheduling;
//! - compilation;
//! - optimization;
//! - QEC;
//! - error mitigation;
//! - hardware discovery;
//! - backend selection;
//! - semantic acceptance.
//!
//! Those responsibilities remain in their respective subsystems.
//!
//! This module coordinates them through provider-neutral traits.
//!
//! ----------------------------------------------------------------------------
//! Critical quantum invariant
//! ----------------------------------------------------------------------------
//!
//! Rollback MUST NOT imply that an arbitrary unknown quantum state can be
//! copied, serialized, transported and restored.
//!
//! A rollback target is valid only when the execution system explicitly
//! establishes that the target is restorable.
//!
//! Valid examples can include:
//!
//! - a program-start boundary;
//! - a classical execution boundary;
//! - a measurement boundary;
//! - a provider-supported resumable execution boundary;
//! - a logical/QEC boundary;
//! - a previously committed checkpoint whose semantics are defined by the
//!   execution/checkpoint subsystem.
//!
//! An arbitrary in-flight unknown quantum state is NOT automatically a valid
//! rollback target.
//!
//! ----------------------------------------------------------------------------
//! Write once / scale everywhere
//! ----------------------------------------------------------------------------
//!
//! This implementation contains:
//!
//! - no maximum qubit count;
//! - no maximum resource count;
//! - no fixed retry count;
//! - no fixed timeout;
//! - no fixed topology;
//! - no provider names;
//! - no provider-specific branches;
//! - no fixed machine size;
//! - no static array representing the quantum machine.
//!
//! Actual limits come from:
//!
//! - execution capabilities;
//! - checkpoint capabilities;
//! - resource availability;
//! - policy;
//! - security authorization;
//! - runtime configuration;
//! - operating-system/resource constraints.
//!
//! Therefore the architecture has no artificial finite quantum-machine size.
//! Real execution remains bounded by addressable representations, memory,
//! runtime resources, hardware capabilities and policy.
//!
//! ----------------------------------------------------------------------------
//! Determinism
//! ----------------------------------------------------------------------------
//!
//! This module does not generate identifiers, random values or implicit
//! decisions.
//!
//! The caller supplies:
//!
//! - rollback request identity;
//! - target checkpoint identity;
//! - execution identity;
//! - policy decision;
//! - capability snapshot;
//! - deterministic mode.
//!
//! A deterministic provider must therefore produce deterministic behavior for
//! identical inputs.
//!
//! ----------------------------------------------------------------------------
//! Concurrency
//! ----------------------------------------------------------------------------
//!
//! Implementations MUST NOT hold application locks across external provider
//! calls.
//!
//! Ownership/lease validation is delegated to the state/coordination layer.
//!
//! ----------------------------------------------------------------------------
//! Security
//! ----------------------------------------------------------------------------
//!
//! Rollback is a security-sensitive state transition.
//!
//! A rollback request must not contain:
//!
//! - credentials;
//! - private keys;
//! - API tokens;
//! - passwords;
//! - raw authorization headers;
//! - raw device pointers;
//! - memory addresses;
//! - arbitrary executable callbacks.
//!
//! Authorization is represented by an opaque authorization reference.
//!
//! Checkpoint integrity, authorization, freshness and semantic compatibility
//! must be verified before restoration.
//!
//! ----------------------------------------------------------------------------
//! Integration contract
//! ----------------------------------------------------------------------------
//!
//! planning/action.rs
//!     Provides RecoveryAction::Rollback.
//!
//! planning/plan.rs
//!     Provides immutable RecoveryPlan and rollback metadata.
//!
//! policy/*
//!     Determines whether rollback is allowed.
//!
//! checkpoint/*
//!     Owns checkpoint semantics, storage, integrity and compatibility.
//!
//! state/*
//!     Owns execution/recovery state.
//!
//! hardware/*
//!     Provides target capabilities and execution mechanisms.
//!
//! verification/*
//!     Determines whether the resulting execution is acceptable.
//!
//! telemetry/*
//!     Records lifecycle events.
//!
//! history/*
//!     Records rollback outcomes.
//!
//! coordination/*
//!     Provides ownership/lease semantics for distributed execution.
//!
//! recovery/recoverer.rs
//!     Orchestrates this operation with other recovery actions.
//!
//! recovery/resume.rs
//!     Handles continuation semantics after a valid boundary.
//!
//! recovery/checkpoint.rs
//!     Coordinates checkpoint-specific recovery.
//!
//! ----------------------------------------------------------------------------
//! Rust
//! ----------------------------------------------------------------------------
//!
//! Rust 2021
//! Rust 1.97 / 1.97.1
//! stable
//! no nightly features
//! no unsafe code
//!
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::fmt;
use std::sync::Arc;

use crate::quantum::resilience::errors::ResilienceError;
use crate::quantum::resilience::planning::action::ResourceId;

// =============================================================================
// Schema
// =============================================================================

/// Stable schema identifier for rollback requests/results.
pub const ROLLBACK_SCHEMA_ID: &str =
    "zamani.quantum.resilience.rollback";

/// Semantic schema version.
pub const ROLLBACK_SCHEMA_VERSION: u16 = 1;

// =============================================================================
// Stable opaque identifiers
// =============================================================================

/// Stable identity of an execution.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ExecutionId(Arc<str>);

impl ExecutionId {
    /// Creates an execution identity.
    pub fn new(value: impl Into<Arc<str>>) -> Result<Self, ResilienceError> {
        let value = value.into();

        if value.is_empty() {
            return Err(ResilienceError::invalid_argument(
                "execution identity must not be empty",
            ));
        }

        Ok(Self(value))
    }

    /// Returns the identifier.
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl fmt::Display for ExecutionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Stable identity of a rollback operation.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RollbackId(Arc<str>);

impl RollbackId {
    /// Creates a rollback identity.
    pub fn new(value: impl Into<Arc<str>>) -> Result<Self, ResilienceError> {
        let value = value.into();

        if value.is_empty() {
            return Err(ResilienceError::invalid_argument(
                "rollback identity must not be empty",
            ));
        }

        Ok(Self(value))
    }

    /// Returns the identifier.
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl fmt::Display for RollbackId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Stable identity of a rollback target.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RollbackTargetId(Arc<str>);

impl RollbackTargetId {
    /// Creates a rollback-target identity.
    pub fn new(value: impl Into<Arc<str>>) -> Result<Self, ResilienceError> {
        let value = value.into();

        if value.is_empty() {
            return Err(ResilienceError::invalid_argument(
                "rollback target identity must not be empty",
            ));
        }

        Ok(Self(value))
    }

    /// Returns the identifier.
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl fmt::Display for RollbackTargetId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Opaque authorization reference.
///
/// The actual credential/token must never be stored in this object.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AuthorizationRef(Arc<str>);

impl AuthorizationRef {
    /// Creates an authorization reference.
    pub fn new(value: impl Into<Arc<str>>) -> Result<Self, ResilienceError> {
        let value = value.into();

        if value.is_empty() {
            return Err(ResilienceError::invalid_argument(
                "authorization reference must not be empty",
            ));
        }

        Ok(Self(value))
    }

    /// Returns the opaque authorization identifier.
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

// =============================================================================
// Rollback target semantics
// =============================================================================

/// Semantic kind of a rollback boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RollbackBoundaryKind {
    /// Beginning of the program/execution.
    ProgramStart,

    /// Explicit classical execution boundary.
    ClassicalBoundary,

    /// Boundary established by a measurement operation.
    MeasurementBoundary,

    /// Provider/runtime-supported resumable boundary.
    ProviderBoundary,

    /// Boundary whose state is defined by a logical/QEC subsystem.
    LogicalBoundary,

    /// Boundary explicitly defined by the checkpoint subsystem.
    CheckpointBoundary,
}

impl RollbackBoundaryKind {
    /// Stable serialized name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProgramStart => "program_start",
            Self::ClassicalBoundary => "classical_boundary",
            Self::MeasurementBoundary => "measurement_boundary",
            Self::ProviderBoundary => "provider_boundary",
            Self::LogicalBoundary => "logical_boundary",
            Self::CheckpointBoundary => "checkpoint_boundary",
        }
    }
}

impl fmt::Display for RollbackBoundaryKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Describes whether the target can legally be restored.
///
/// This is deliberately explicit. A target cannot be considered restorable
/// merely because an identifier exists.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Restorability {
    /// The target is known to be reconstructible/restorable.
    Restorable,

    /// The target requires an external provider check before restoration.
    RequiresProviderValidation,

    /// The target represents state that must not be treated as restorable.
    NotRestorable,
}

/// A validated rollback target.
///
/// This is metadata, not the quantum state itself.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RollbackTarget {
    id: RollbackTargetId,
    kind: RollbackBoundaryKind,

    /// Identity of the execution from which this target originated.
    source_execution: ExecutionId,

    /// Stable digest/identity of the target state.
    ///
    /// The actual checkpoint bytes/state are owned by the checkpoint/state
    /// subsystem and are never embedded here.
    state_reference: Arc<str>,

    /// Program/IR semantic identity expected at this boundary.
    program_identity: Arc<str>,

    /// Capability snapshot against which this target was accepted.
    capability_identity: Arc<str>,

    /// Restorability classification.
    restorability: Restorability,

    /// Whether the target has passed integrity validation.
    integrity_verified: bool,

    /// Whether the target has passed semantic compatibility validation.
    semantic_compatibility_verified: bool,

    /// Whether the target has passed freshness validation.
    freshness_verified: bool,
}

impl RollbackTarget {
    /// Creates a rollback target.
    pub fn new(
        id: RollbackTargetId,
        kind: RollbackBoundaryKind,
        source_execution: ExecutionId,
        state_reference: impl Into<Arc<str>>,
        program_identity: impl Into<Arc<str>>,
        capability_identity: impl Into<Arc<str>>,
        restorability: Restorability,
    ) -> Result<Self, ResilienceError> {
        let state_reference = state_reference.into();
        let program_identity = program_identity.into();
        let capability_identity = capability_identity.into();

        if state_reference.is_empty() {
            return Err(ResilienceError::invalid_argument(
                "rollback state reference must not be empty",
            ));
        }

        if program_identity.is_empty() {
            return Err(ResilienceError::invalid_argument(
                "rollback program identity must not be empty",
            ));
        }

        if capability_identity.is_empty() {
            return Err(ResilienceError::invalid_argument(
                "rollback capability identity must not be empty",
            ));
        }

        Ok(Self {
            id,
            kind,
            source_execution,
            state_reference,
            program_identity,
            capability_identity,
            restorability,
            integrity_verified: false,
            semantic_compatibility_verified: false,
            freshness_verified: false,
        })
    }

    /// Returns the target identity.
    pub fn id(&self) -> &RollbackTargetId {
        &self.id
    }

    /// Returns the boundary kind.
    pub const fn kind(&self) -> RollbackBoundaryKind {
        self.kind
    }

    /// Returns the originating execution.
    pub fn source_execution(&self) -> &ExecutionId {
        &self.source_execution
    }

    /// Returns the opaque state reference.
    pub fn state_reference(&self) -> &str {
        self.state_reference.as_ref()
    }

    /// Returns the program identity.
    pub fn program_identity(&self) -> &str {
        self.program_identity.as_ref()
    }

    /// Returns the capability snapshot identity.
    pub fn capability_identity(&self) -> &str {
        self.capability_identity.as_ref()
    }

    /// Returns the restorability classification.
    pub const fn restorability(&self) -> Restorability {
        self.restorability
    }

    /// Returns whether integrity has been verified.
    pub const fn integrity_verified(&self) -> bool {
        self.integrity_verified
    }

    /// Returns whether semantic compatibility has been verified.
    pub const fn semantic_compatibility_verified(&self) -> bool {
        self.semantic_compatibility_verified
    }

    /// Returns whether freshness has been verified.
    pub const fn freshness_verified(&self) -> bool {
        self.freshness_verified
    }

    /// Returns whether all target-level verification gates have passed.
    #[must_use]
    pub const fn is_verified(&self) -> bool {
        self.integrity_verified
            && self.semantic_compatibility_verified
            && self.freshness_verified
    }

    /// Marks target integrity as verified.
    ///
    /// This does not establish semantic compatibility or freshness.
    pub fn with_integrity_verified(mut self, verified: bool) -> Self {
        self.integrity_verified = verified;
        self
    }

    /// Marks semantic compatibility as verified.
    pub fn with_semantic_compatibility_verified(mut self, verified: bool) -> Self {
        self.semantic_compatibility_verified = verified;
        self
    }

    /// Marks freshness as verified.
    pub fn with_freshness_verified(mut self, verified: bool) -> Self {
        self.freshness_verified = verified;
        self
    }
}

// =============================================================================
// Replay / side-effect semantics
// =============================================================================

/// Describes whether restoring an earlier execution boundary can safely
/// coexist with the program's external/classical side effects.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ReplaySafety {
    /// Replay/rollback is explicitly safe.
    Safe,

    /// Safety is established by an external idempotency/transaction contract.
    RequiresExternalGuarantee,

    /// Rollback cannot safely replay the affected computation.
    Unsafe,
}

impl ReplaySafety {
    /// Stable serialized name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Safe => "safe",
            Self::RequiresExternalGuarantee => "requires_external_guarantee",
            Self::Unsafe => "unsafe",
        }
    }
}

/// Defines how externally visible effects are handled when execution returns
/// to an earlier state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SideEffectMode {
    /// There are no non-reversible external effects in the rollback interval.
    None,

    /// External effects are protected by an idempotency/transaction boundary.
    Transactional,

    /// A separately verified compensation mechanism exists.
    Compensatable,

    /// External effects exist and cannot safely be replayed.
    NonReversible,
}

// =============================================================================
// Rollback request
// =============================================================================

/// Immutable request to perform rollback.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RollbackRequest {
    rollback_id: RollbackId,
    execution_id: ExecutionId,
    target: RollbackTarget,

    /// Identity of the canonical program/IR.
    program_identity: Arc<str>,

    /// Identity of the currently observed execution state.
    current_state_identity: Arc<str>,

    /// Authorization reference.
    authorization: Option<AuthorizationRef>,

    /// Whether the caller has established replay safety.
    replay_safety: ReplaySafety,

    /// Handling of external effects.
    side_effect_mode: SideEffectMode,

    /// Whether deterministic execution was requested.
    deterministic: bool,

    /// Whether target capability freshness must be revalidated.
    require_capability_revalidation: bool,

    /// Whether result verification is mandatory after rollback.
    require_post_rollback_verification: bool,

    /// Provider-neutral target resource.
    target_resource: Option<ResourceId>,
}

impl RollbackRequest {
    /// Creates a rollback request.
    pub fn new(
        rollback_id: RollbackId,
        execution_id: ExecutionId,
        target: RollbackTarget,
        program_identity: impl Into<Arc<str>>,
        current_state_identity: impl Into<Arc<str>>,
        replay_safety: ReplaySafety,
        side_effect_mode: SideEffectMode,
    ) -> Result<Self, ResilienceError> {
        let program_identity = program_identity.into();
        let current_state_identity = current_state_identity.into();

        if program_identity.is_empty() {
            return Err(ResilienceError::invalid_argument(
                "rollback request program identity must not be empty",
            ));
        }

        if current_state_identity.is_empty() {
            return Err(ResilienceError::invalid_argument(
                "rollback current-state identity must not be empty",
            ));
        }

        Ok(Self {
            rollback_id,
            execution_id,
            target,
            program_identity,
            current_state_identity,
            authorization: None,
            replay_safety,
            side_effect_mode,
            deterministic: false,
            require_capability_revalidation: true,
            require_post_rollback_verification: true,
            target_resource: None,
        })
    }

    /// Adds an opaque authorization reference.
    pub fn with_authorization(mut self, authorization: AuthorizationRef) -> Self {
        self.authorization = Some(authorization);
        self
    }

    /// Requests deterministic execution.
    pub const fn with_deterministic_execution(mut self, deterministic: bool) -> Self {
        self.deterministic = deterministic;
        self
    }

    /// Controls capability freshness validation.
    pub const fn with_capability_revalidation(mut self, required: bool) -> Self {
        self.require_capability_revalidation = required;
        self
    }

    /// Controls mandatory post-rollback verification.
    pub const fn with_post_rollback_verification(mut self, required: bool) -> Self {
        self.require_post_rollback_verification = required;
        self
    }

    /// Associates a provider-neutral resource identity.
    pub fn with_target_resource(mut self, resource: ResourceId) -> Self {
        self.target_resource = Some(resource);
        self
    }

    /// Returns rollback identity.
    pub fn rollback_id(&self) -> &RollbackId {
        &self.rollback_id
    }

    /// Returns execution identity.
    pub fn execution_id(&self) -> &ExecutionId {
        &self.execution_id
    }

    /// Returns rollback target.
    pub fn target(&self) -> &RollbackTarget {
        &self.target
    }

    /// Returns canonical program identity.
    pub fn program_identity(&self) -> &str {
        self.program_identity.as_ref()
    }

    /// Returns current state identity.
    pub fn current_state_identity(&self) -> &str {
        self.current_state_identity.as_ref()
    }

    /// Returns replay safety.
    pub const fn replay_safety(&self) -> ReplaySafety {
        self.replay_safety
    }

    /// Returns external side-effect mode.
    pub const fn side_effect_mode(&self) -> SideEffectMode {
        self.side_effect_mode
    }

    /// Returns whether deterministic execution is requested.
    pub const fn deterministic(&self) -> bool {
        self.deterministic
    }

    /// Returns whether capability freshness is required.
    pub const fn require_capability_revalidation(&self) -> bool {
        self.require_capability_revalidation
    }

    /// Returns whether post-rollback verification is mandatory.
    pub const fn require_post_rollback_verification(&self) -> bool {
        self.require_post_rollback_verification
    }

    /// Returns the optional target resource.
    pub fn target_resource(&self) -> Option<&ResourceId> {
        self.target_resource.as_ref()
    }

    /// Returns whether authorization was supplied.
    pub fn is_authorized_reference_present(&self) -> bool {
        self.authorization.is_some()
    }
}

// =============================================================================
// Validation contract
// =============================================================================

/// Result of pre-rollback validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RollbackValidation {
    /// Whether the request can proceed.
    valid: bool,

    /// Validation messages are stable semantic identifiers, not secrets.
    reasons: Vec<RollbackValidationReason>,
}

impl RollbackValidation {
    /// Creates a successful validation result.
    #[must_use]
    pub fn valid() -> Self {
        Self {
            valid: true,
            reasons: Vec::new(),
        }
    }

    /// Creates a failed validation result.
    #[must_use]
    pub fn invalid(reasons: Vec<RollbackValidationReason>) -> Self {
        Self {
            valid: false,
            reasons,
        }
    }

    /// Returns whether validation passed.
    #[must_use]
    pub const fn is_valid(&self) -> bool {
        self.valid
    }

    /// Returns validation reasons.
    pub fn reasons(&self) -> &[RollbackValidationReason] {
        &self.reasons
    }
}

/// Stable rollback validation failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RollbackValidationReason {
    /// Target cannot be restored.
    TargetNotRestorable,

    /// Target integrity has not been established.
    TargetIntegrityNotVerified,

    /// Target semantic compatibility has not been established.
    TargetSemanticCompatibilityNotVerified,

    /// Target freshness has not been established.
    TargetFreshnessNotVerified,

    /// Target belongs to another execution.
    ExecutionMismatch,

    /// Program/IR identities differ.
    ProgramMismatch,

    /// Replay is unsafe.
    ReplayUnsafe,

    /// External side effects cannot safely be replayed.
    NonReversibleSideEffects,

    /// Required authorization is absent.
    AuthorizationMissing,

    /// Required capability revalidation failed.
    CapabilityStale,

    /// Current execution state differs from the expected state.
    CurrentStateChanged,

    /// Target resource is unavailable.
    TargetResourceUnavailable,

    /// Request is otherwise invalid.
    InvalidRequest,
}

impl RollbackValidationReason {
    /// Stable serialized representation.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TargetNotRestorable => "target_not_restorable",
            Self::TargetIntegrityNotVerified => "target_integrity_not_verified",
            Self::TargetSemanticCompatibilityNotVerified => {
                "target_semantic_compatibility_not_verified"
            }
            Self::TargetFreshnessNotVerified => "target_freshness_not_verified",
            Self::ExecutionMismatch => "execution_mismatch",
            Self::ProgramMismatch => "program_mismatch",
            Self::ReplayUnsafe => "replay_unsafe",
            Self::NonReversibleSideEffects => "non_reversible_side_effects",
            Self::AuthorizationMissing => "authorization_missing",
            Self::CapabilityStale => "capability_stale",
            Self::CurrentStateChanged => "current_state_changed",
            Self::TargetResourceUnavailable => "target_resource_unavailable",
            Self::InvalidRequest => "invalid_request",
        }
    }
}

impl fmt::Display for RollbackValidationReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// =============================================================================
// Provider contracts
// =============================================================================

/// Execution handle returned after rollback.
///
/// This is intentionally opaque to resilience.
///
/// The hardware/runtime layer owns the meaning.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RollbackExecutionHandle(Arc<str>);

impl RollbackExecutionHandle {
    /// Creates an opaque execution handle.
    pub fn new(value: impl Into<Arc<str>>) -> Result<Self, ResilienceError> {
        let value = value.into();

        if value.is_empty() {
            return Err(ResilienceError::invalid_argument(
                "rollback execution handle must not be empty",
            ));
        }

        Ok(Self(value))
    }

    /// Returns the opaque handle.
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

/// Provider-neutral capabilities required for rollback validation.
pub trait RollbackCapabilityProvider: Send + Sync {
    /// Validates whether the current target can restore the supplied rollback
    /// target.
    fn validate_target(
        &self,
        request: &RollbackRequest,
    ) -> Result<(), ResilienceError>;
}

/// Current execution-state provider.
///
/// This prevents a stale rollback request from overwriting newer state.
pub trait RollbackStateProvider: Send + Sync {
    /// Returns whether the execution still has the expected state identity.
    fn validate_current_state(
        &self,
        request: &RollbackRequest,
    ) -> Result<(), ResilienceError>;
}

/// Authorization provider.
///
/// The actual credential material remains outside the rollback module.
pub trait RollbackAuthorizationProvider: Send + Sync {
    /// Validates authorization for the requested rollback.
    fn authorize(&self, request: &RollbackRequest) -> Result<(), ResilienceError>;
}

/// Provider-neutral state restoration executor.
pub trait RollbackStateExecutor: Send + Sync {
    /// Provider performs the actual restoration.
    ///
    /// The implementation may restore a checkpoint, recreate a classical
    /// execution state, request a provider-supported resumable boundary, or
    /// perform another explicitly supported operation.
    ///
    /// It MUST NOT pretend that arbitrary unknown quantum state is
    /// serializable/restorable unless the underlying execution technology
    /// explicitly guarantees that capability.
    fn rollback(
        &self,
        request: &RollbackRequest,
    ) -> Result<RollbackExecutionHandle, ResilienceError>;
}

/// Post-rollback verification contract.
///
/// The verification subsystem remains authoritative for acceptance.
pub trait RollbackVerifier: Send + Sync {
    /// Verifies the result of the rollback operation.
    fn verify(
        &self,
        request: &RollbackRequest,
        execution: &RollbackExecutionHandle,
    ) -> Result<RollbackVerification, ResilienceError>;
}

/// Optional lifecycle observer.
///
/// Implementations may connect this to telemetry/history without making this
/// module depend on concrete observability implementations.
pub trait RollbackObserver: Send + Sync {
    /// Called before execution.
    fn prepared(&self, request: &RollbackRequest);

    /// Called after the state provider reports successful restoration.
    fn restored(
        &self,
        request: &RollbackRequest,
        execution: &RollbackExecutionHandle,
    );

    /// Called after verification.
    fn verified(
        &self,
        request: &RollbackRequest,
        verification: &RollbackVerification,
    );
}

// =============================================================================
// Verification
// =============================================================================

/// Verification status returned by the authoritative verification subsystem.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RollbackVerificationStatus {
    /// Rollback restored an acceptable execution state.
    Accepted,

    /// Rollback succeeded but acceptance requires a higher-level decision.
    Degraded,

    /// Rollback result is not acceptable.
    Rejected,

    /// Verification could not establish acceptance.
    Inconclusive,
}

impl RollbackVerificationStatus {
    /// Stable serialized name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Accepted => "accepted",
            Self::Degraded => "degraded",
            Self::Rejected => "rejected",
            Self::Inconclusive => "inconclusive",
        }
    }

    /// Whether this status permits direct acceptance.
    #[must_use]
    pub const fn is_accepted(self) -> bool {
        matches!(self, Self::Accepted)
    }
}

/// Result of post-rollback verification.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RollbackVerification {
    status: RollbackVerificationStatus,

    /// Stable semantic identity of the verified program/result.
    verified_program_identity: Arc<str>,

    /// Stable identity of the verified execution state.
    verified_state_identity: Arc<str>,

    /// Verification provenance identity.
    verification_reference: Arc<str>,
}

impl RollbackVerification {
    /// Creates a verification result.
    pub fn new(
        status: RollbackVerificationStatus,
        verified_program_identity: impl Into<Arc<str>>,
        verified_state_identity: impl Into<Arc<str>>,
        verification_reference: impl Into<Arc<str>>,
    ) -> Result<Self, ResilienceError> {
        let verified_program_identity = verified_program_identity.into();
        let verified_state_identity = verified_state_identity.into();
        let verification_reference = verification_reference.into();

        if verified_program_identity.is_empty()
            || verified_state_identity.is_empty()
            || verification_reference.is_empty()
        {
            return Err(ResilienceError::invalid_argument(
                "rollback verification identities must not be empty",
            ));
        }

        Ok(Self {
            status,
            verified_program_identity,
            verified_state_identity,
            verification_reference,
        })
    }

    /// Returns verification status.
    pub const fn status(&self) -> RollbackVerificationStatus {
        self.status
    }

    /// Returns verified program identity.
    pub fn verified_program_identity(&self) -> &str {
        self.verified_program_identity.as_ref()
    }

    /// Returns verified state identity.
    pub fn verified_state_identity(&self) -> &str {
        self.verified_state_identity.as_ref()
    }

    /// Returns verification reference.
    pub fn verification_reference(&self) -> &str {
        self.verification_reference.as_ref()
    }
}

// =============================================================================
// Lifecycle
// =============================================================================

/// Rollback lifecycle state.
///
/// The state machine is intentionally explicit so that callers cannot confuse
/// "restoration requested" with "rollback accepted".
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RollbackState {
    /// Request received.
    Requested,

    /// Request has passed local structural validation.
    Validating,

    /// External authorization/capability/state validation is occurring.
    PreconditionsChecking,

    /// Ready to invoke the state executor.
    Prepared,

    /// State restoration is in progress.
    Restoring,

    /// Restoration completed; result has not yet been accepted.
    Restored,

    /// Verification is in progress.
    Verifying,

    /// Result accepted.
    Accepted,

    /// Result is degraded and requires higher-level policy handling.
    Degraded,

    /// Result rejected.
    Rejected,

    /// Operation was cancelled before completion.
    Cancelled,

    /// Rollback failed.
    Failed,
}

impl RollbackState {
    /// Stable serialized name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Requested => "requested",
            Self::Validating => "validating",
            Self::PreconditionsChecking => "preconditions_checking",
            Self::Prepared => "prepared",
            Self::Restoring => "restoring",
            Self::Restored => "restored",
            Self::Verifying => "verifying",
            Self::Accepted => "accepted",
            Self::Degraded => "degraded",
            Self::Rejected => "rejected",
            Self::Cancelled => "cancelled",
            Self::Failed => "failed",
        }
    }
}

// =============================================================================
// Cancellation
// =============================================================================

/// Cancellation abstraction.
///
/// This keeps rollback independent from any particular async/runtime
/// cancellation mechanism.
pub trait RollbackCancellation: Send + Sync {
    /// Returns true when rollback should stop before the next irreversible
    /// operation.
    fn is_cancelled(&self) -> bool;
}

/// Never-cancelled implementation.
#[derive(Debug, Default, Clone, Copy)]
pub struct NeverCancelled;

impl RollbackCancellation for NeverCancelled {
    fn is_cancelled(&self) -> bool {
        false
    }
}

// =============================================================================
// Rollback result
// =============================================================================

/// Final outcome of a rollback operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RollbackOutcome {
    rollback_id: RollbackId,
    execution_id: ExecutionId,
    target: RollbackTargetId,
    state: RollbackState,
    execution: Option<RollbackExecutionHandle>,
    verification: Option<RollbackVerification>,
}

impl RollbackOutcome {
    /// Returns rollback identity.
    pub fn rollback_id(&self) -> &RollbackId {
        &self.rollback_id
    }

    /// Returns execution identity.
    pub fn execution_id(&self) -> &ExecutionId {
        &self.execution_id
    }

    /// Returns target identity.
    pub fn target(&self) -> &RollbackTargetId {
        &self.target
    }

    /// Returns final lifecycle state.
    pub const fn state(&self) -> RollbackState {
        self.state
    }

    /// Returns execution handle when restoration actually occurred.
    pub fn execution(&self) -> Option<&RollbackExecutionHandle> {
        self.execution.as_ref()
    }

    /// Returns verification when verification occurred.
    pub fn verification(&self) -> Option<&RollbackVerification> {
        self.verification.as_ref()
    }

    /// Returns whether rollback was accepted by verification.
    #[must_use]
    pub fn is_accepted(&self) -> bool {
        self.state == RollbackState::Accepted
    }
}

// =============================================================================
// Rollback executor
// =============================================================================

/// Coordinates safe rollback without owning the underlying quantum execution
/// technology.
pub struct RollbackExecutor<
    C,
    S,
    A,
    E,
    V,
    O = NoopRollbackObserver,
> where
    C: RollbackCapabilityProvider,
    S: RollbackStateProvider,
    A: RollbackAuthorizationProvider,
    E: RollbackStateExecutor,
    V: RollbackVerifier,
    O: RollbackObserver,
{
    capabilities: Arc<C>,
    state: Arc<S>,
    authorization: Arc<A>,
    executor: Arc<E>,
    verifier: Arc<V>,
    observer: Arc<O>,
}

impl<C, S, A, E, V> RollbackExecutor<C, S, A, E, V, NoopRollbackObserver>
where
    C: RollbackCapabilityProvider,
    S: RollbackStateProvider,
    A: RollbackAuthorizationProvider,
    E: RollbackStateExecutor,
    V: RollbackVerifier,
{
    /// Creates a rollback executor without an observer.
    pub fn new(
        capabilities: Arc<C>,
        state: Arc<S>,
        authorization: Arc<A>,
        executor: Arc<E>,
        verifier: Arc<V>,
    ) -> Self {
        Self {
            capabilities,
            state,
            authorization,
            executor,
            verifier,
            observer: Arc::new(NoopRollbackObserver),
        }
    }
}

impl<C, S, A, E, V, O> RollbackExecutor<C, S, A, E, V, O>
where
    C: RollbackCapabilityProvider,
    S: RollbackStateProvider,
    A: RollbackAuthorizationProvider,
    E: RollbackStateExecutor,
    V: RollbackVerifier,
    O: RollbackObserver,
{
    /// Creates a rollback executor with an observer.
    pub fn with_observer(
        capabilities: Arc<C>,
        state: Arc<S>,
        authorization: Arc<A>,
        executor: Arc<E>,
        verifier: Arc<V>,
        observer: Arc<O>,
    ) -> Self {
        Self {
            capabilities,
            state,
            authorization,
            executor,
            verifier,
            observer,
        }
    }

    /// Performs rollback.
    ///
    /// The operation is deliberately staged:
    ///
    /// 1. structural validation;
    /// 2. cancellation check;
    /// 3. authorization;
    /// 4. current-state validation;
    /// 5. capability validation;
    /// 6. provider restoration;
    /// 7. verification;
    /// 8. final outcome.
    ///
    /// No retry loop exists here. Retry policy belongs to
    /// `policy/retry.rs` and `recovery/retry.rs`.
    pub fn rollback(
        &self,
        request: &RollbackRequest,
        cancellation: &dyn RollbackCancellation,
    ) -> Result<RollbackOutcome, ResilienceError> {
        self.validate_request(request)?;

        if cancellation.is_cancelled() {
            return Ok(self.cancelled_outcome(request));
        }

        self.observer.prepared(request);

        if cancellation.is_cancelled() {
            return Ok(self.cancelled_outcome(request));
        }

        self.authorization.authorize(request)?;

        if cancellation.is_cancelled() {
            return Ok(self.cancelled_outcome(request));
        }

        self.state.validate_current_state(request)?;

        if request.require_capability_revalidation() {
            self.capabilities.validate_target(request)?;
        }

        if cancellation.is_cancelled() {
            return Ok(self.cancelled_outcome(request));
        }

        let execution = self.executor.rollback(request)?;

        self.observer.restored(request, &execution);

        if cancellation.is_cancelled() {
            return Ok(RollbackOutcome {
                rollback_id: request.rollback_id().clone(),
                execution_id: request.execution_id().clone(),
                target: request.target().id().clone(),
                state: RollbackState::Cancelled,
                execution: Some(execution),
                verification: None,
            });
        }

        if request.require_post_rollback_verification() {
            let verification = self.verifier.verify(request, &execution)?;

            self.observer.verified(request, &verification);

            let state = match verification.status() {
                RollbackVerificationStatus::Accepted => RollbackState::Accepted,
                RollbackVerificationStatus::Degraded => RollbackState::Degraded,
                RollbackVerificationStatus::Rejected
                | RollbackVerificationStatus::Inconclusive => {
                    RollbackState::Rejected
                }
            };

            return Ok(RollbackOutcome {
                rollback_id: request.rollback_id().clone(),
                execution_id: request.execution_id().clone(),
                target: request.target().id().clone(),
                state,
                execution: Some(execution),
                verification: Some(verification),
            });
        }

        // Production safety rule:
        //
        // A successful restoration without mandatory verification is NOT
        // automatically considered accepted.
        //
        // The caller explicitly disabled verification, so the result remains
        // degraded and must be handled by the higher-level resilience policy.
        Ok(RollbackOutcome {
            rollback_id: request.rollback_id().clone(),
            execution_id: request.execution_id().clone(),
            target: request.target().id().clone(),
            state: RollbackState::Degraded,
            execution: Some(execution),
            verification: None,
        })
    }

    /// Performs structural validation before external calls.
    pub fn validate_request(
        &self,
        request: &RollbackRequest,
    ) -> Result<(), ResilienceError> {
        let target = request.target();

        if target.source_execution() != request.execution_id() {
            return Err(ResilienceError::invalid_state(
                "rollback target belongs to a different execution",
            ));
        }

        if target.program_identity() != request.program_identity() {
            return Err(ResilienceError::invalid_state(
                "rollback target is incompatible with the requested program identity",
            ));
        }

        match target.restorability() {
            Restorability::Restorable => {}
            Restorability::RequiresProviderValidation => {}
            Restorability::NotRestorable => {
                return Err(ResilienceError::invalid_state(
                    "rollback target is not restorable",
                ));
            }
        }

        if !target.integrity_verified() {
            return Err(ResilienceError::invalid_state(
                "rollback target integrity has not been verified",
            ));
        }

        if !target.semantic_compatibility_verified() {
            return Err(ResilienceError::invalid_state(
                "rollback target semantic compatibility has not been verified",
            ));
        }

        if !target.freshness_verified() {
            return Err(ResilienceError::invalid_state(
                "rollback target freshness has not been verified",
            ));
        }

        if matches!(request.replay_safety(), ReplaySafety::Unsafe) {
            return Err(ResilienceError::invalid_state(
                "rollback is not semantically safe for this execution",
            ));
        }

        if matches!(
            request.side_effect_mode(),
            SideEffectMode::NonReversible
        ) {
            return Err(ResilienceError::invalid_state(
                "rollback would replay non-reversible external side effects",
            ));
        }

        if request.is_authorized_reference_present() {
            // Authorization material itself remains outside this module.
        } else {
            return Err(ResilienceError::invalid_state(
                "rollback authorization reference is missing",
            ));
        }

        Ok(())
    }

    fn cancelled_outcome(&self, request: &RollbackRequest) -> RollbackOutcome {
        RollbackOutcome {
            rollback_id: request.rollback_id().clone(),
            execution_id: request.execution_id().clone(),
            target: request.target().id().clone(),
            state: RollbackState::Cancelled,
            execution: None,
            verification: None,
        }
    }
}

// =============================================================================
// No-op observer
// =============================================================================

/// Observer used when telemetry/history integration is not required by the
/// caller.
#[derive(Debug, Default, Clone, Copy)]
pub struct NoopRollbackObserver;

impl RollbackObserver for NoopRollbackObserver {
    fn prepared(&self, _request: &RollbackRequest) {}

    fn restored(
        &self,
        _request: &RollbackRequest,
        _execution: &RollbackExecutionHandle,
    ) {
    }

    fn verified(
        &self,
        _request: &RollbackRequest,
        _verification: &RollbackVerification,
    ) {
    }
}

// =============================================================================
// Test-only implementations
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct TestCapabilities;

    impl RollbackCapabilityProvider for TestCapabilities {
        fn validate_target(
            &self,
            _request: &RollbackRequest,
        ) -> Result<(), ResilienceError> {
            Ok(())
        }
    }

    #[derive(Debug)]
    struct TestState;

    impl RollbackStateProvider for TestState {
        fn validate_current_state(
            &self,
            _request: &RollbackRequest,
        ) -> Result<(), ResilienceError> {
            Ok(())
        }
    }

    #[derive(Debug)]
    struct TestAuthorization;

    impl RollbackAuthorizationProvider for TestAuthorization {
        fn authorize(
            &self,
            _request: &RollbackRequest,
        ) -> Result<(), ResilienceError> {
            Ok(())
        }
    }

    #[derive(Debug)]
    struct TestExecutor;

    impl RollbackStateExecutor for TestExecutor {
        fn rollback(
            &self,
            _request: &RollbackRequest,
        ) -> Result<RollbackExecutionHandle, ResilienceError> {
            RollbackExecutionHandle::new("test-execution")
        }
    }

    #[derive(Debug)]
    struct TestVerifier;

    impl RollbackVerifier for TestVerifier {
        fn verify(
            &self,
            request: &RollbackRequest,
            _execution: &RollbackExecutionHandle,
        ) -> Result<RollbackVerification, ResilienceError> {
            RollbackVerification::new(
                RollbackVerificationStatus::Accepted,
                request.program_identity(),
                "restored-state",
                "verification-1",
            )
        }
    }

    fn valid_request() -> RollbackRequest {
        let execution =
            ExecutionId::new("execution-1").expect("valid execution identity");

        let target_id =
            RollbackTargetId::new("checkpoint-1")
                .expect("valid target identity");

        let target = RollbackTarget::new(
            target_id,
            RollbackBoundaryKind::CheckpointBoundary,
            execution.clone(),
            "state-hash-1",
            "program-hash-1",
            "capability-hash-1",
            Restorability::Restorable,
        )
        .expect("valid target")
        .with_integrity_verified(true)
        .with_semantic_compatibility_verified(true)
        .with_freshness_verified(true);

        RollbackRequest::new(
            RollbackId::new("rollback-1")
                .expect("valid rollback identity"),
            execution,
            target,
            "program-hash-1",
            "current-state-1",
            ReplaySafety::Safe,
            SideEffectMode::None,
        )
        .expect("valid rollback request")
        .with_authorization(
            AuthorizationRef::new("authorization-reference")
                .expect("valid authorization"),
        )
    }

    fn executor(
    ) -> RollbackExecutor<
        TestCapabilities,
        TestState,
        TestAuthorization,
        TestExecutor,
        TestVerifier,
    > {
        RollbackExecutor::new(
            Arc::new(TestCapabilities),
            Arc::new(TestState),
            Arc::new(TestAuthorization),
            Arc::new(TestExecutor),
            Arc::new(TestVerifier),
        )
    }

    #[test]
    fn valid_rollback_is_verified_and_accepted() {
        let request = valid_request();

        let result = executor()
            .rollback(&request, &NeverCancelled)
            .expect("rollback should execute");

        assert_eq!(result.state(), RollbackState::Accepted);
        assert!(result.execution().is_some());
        assert!(result.verification().is_some());
        assert!(result.is_accepted());
    }

    #[test]
    fn arbitrary_non_restorable_target_is_rejected() {
        let execution =
            ExecutionId::new("execution-1").expect("valid execution");

        let target = RollbackTarget::new(
            RollbackTargetId::new("unsafe-target")
                .expect("valid target"),
            RollbackBoundaryKind::ProviderBoundary,
            execution.clone(),
            "state-hash",
            "program-hash",
            "capability-hash",
            Restorability::NotRestorable,
        )
        .expect("target construction")
        .with_integrity_verified(true)
        .with_semantic_compatibility_verified(true)
        .with_freshness_verified(true);

        let request = RollbackRequest::new(
            RollbackId::new("rollback-1")
                .expect("valid rollback"),
            execution,
            target,
            "program-hash",
            "current-state",
            ReplaySafety::Safe,
            SideEffectMode::None,
        )
        .expect("request")
        .with_authorization(
            AuthorizationRef::new("auth")
                .expect("authorization"),
        );

        let result = executor().rollback(&request, &NeverCancelled);

        assert!(result.is_err());
    }

    #[test]
    fn unsafe_replay_is_rejected() {
        let request = {
            let mut request = valid_request();

            request.replay_safety = ReplaySafety::Unsafe;

            request
        };

        let result = executor().rollback(&request, &NeverCancelled);

        assert!(result.is_err());
    }

    #[test]
    fn non_reversible_side_effects_are_rejected() {
        let request = {
            let mut request = valid_request();

            request.side_effect_mode = SideEffectMode::NonReversible;

            request
        };

        let result = executor().rollback(&request, &NeverCancelled);

        assert!(result.is_err());
    }

    #[test]
    fn missing_authorization_is_rejected() {
        let request = {
            let mut request = valid_request();

            request.authorization = None;

            request
        };

        let result = executor().rollback(&request, &NeverCancelled);

        assert!(result.is_err());
    }

    #[test]
    fn cancellation_prevents_restoration() {
        #[derive(Debug)]
        struct Cancelled;

        impl RollbackCancellation for Cancelled {
            fn is_cancelled(&self) -> bool {
                true
            }
        }

        let request = valid_request();

        let result = executor()
            .rollback(&request, &Cancelled)
            .expect("cancellation is an outcome");

        assert_eq!(result.state(), RollbackState::Cancelled);
        assert!(result.execution().is_none());
    }

    #[test]
    fn verification_rejection_is_not_reported_as_success() {
        #[derive(Debug)]
        struct RejectingVerifier;

        impl RollbackVerifier for RejectingVerifier {
            fn verify(
                &self,
                request: &RollbackRequest,
                _execution: &RollbackExecutionHandle,
            ) -> Result<RollbackVerification, ResilienceError> {
                RollbackVerification::new(
                    RollbackVerificationStatus::Rejected,
                    request.program_identity(),
                    "state",
                    "verification",
                )
            }
        }

        let rollback_executor = RollbackExecutor::new(
            Arc::new(TestCapabilities),
            Arc::new(TestState),
            Arc::new(TestAuthorization),
            Arc::new(TestExecutor),
            Arc::new(RejectingVerifier),
        );

        let request = valid_request();

        let result = rollback_executor
            .rollback(&request, &NeverCancelled)
            .expect("verification rejection is a normal outcome");

        assert_eq!(result.state(), RollbackState::Rejected);
        assert!(!result.is_accepted());
    }

    #[test]
    fn disabling_verification_produces_degraded_not_accepted() {
        let request = valid_request()
            .with_post_rollback_verification(false);

        let result = executor()
            .rollback(&request, &NeverCancelled)
            .expect("rollback restoration should succeed");

        assert_eq!(result.state(), RollbackState::Degraded);
        assert!(!result.is_accepted());
    }

    #[test]
    fn program_mismatch_is_rejected_before_provider_execution() {
        let request = {
            let mut request = valid_request();

            request.program_identity = Arc::from("different-program");

            request
        };

        let result = executor().validate_request(&request);

        assert!(result.is_err());
    }

    #[test]
    fn target_execution_mismatch_is_rejected() {
        let request = {
            let mut request = valid_request();

            request.target = RollbackTarget::new(
                RollbackTargetId::new("target")
                    .expect("target"),
                RollbackBoundaryKind::CheckpointBoundary,
                ExecutionId::new("different-execution")
                    .expect("execution"),
                "state",
                "program-hash-1",
                "capability",
                Restorability::Restorable,
            )
            .expect("target")
            .with_integrity_verified(true)
            .with_semantic_compatibility_verified(true)
            .with_freshness_verified(true);

            request
        };

        let result = executor().validate_request(&request);

        assert!(result.is_err());
    }
}