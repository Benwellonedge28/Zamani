//! Zamani Quantum Resilience — Adaptation Adapter Contract
//!
//! Path:
//!     src/quantum/resilience/adaptation/adapter.rs
//!
//! Purpose:
//!     Defines the production-grade, provider-independent contract between
//!     the resilience planner/recovery system and concrete adaptation
//!     implementations.
//!
//! ============================================================================
//! Architectural position
//! ============================================================================
//!
//! The adaptation subsystem sits between resilience planning and the
//! authoritative quantum subsystems that actually know how to transform a
//! computation:
//!
//! ```text
//!                    quantum::resilience
//!                           |
//!                           v
//!                    RecoveryAction
//!                           |
//!                           v
//!                 AdaptationAdapterRegistry
//!                           |
//!                           v
//!                  AdaptationAdapter
//!                           |
//!          +----------------+----------------+
//!          |                |                |
//!          v                v                v
//!       routing        scheduling       compiler/IR
//!          |                |                |
//!          +----------------+----------------+
//!                           |
//!                           v
//!                         QEC
//!                           |
//!                           v
//!                       hardware
//! ```
//!
//! This file owns the CONTRACT, not the algorithms.
//!
//! It must not:
//!
//! - implement routing;
//! - implement scheduling;
//! - implement compilation;
//! - implement optimization;
//! - implement QEC;
//! - implement backend selection;
//! - communicate with hardware;
//! - access a filesystem;
//! - access a network;
//! - contain provider-specific logic;
//! - contain machine-size constants;
//! - contain retry loops;
//! - contain hidden global state;
//! - contain unsafe code.
//!
//! ============================================================================
//! Write once, scale everywhere
//! ============================================================================
//!
//! The adapter contract contains no architectural upper bound on:
//!
//! - logical qubits;
//! - physical qubits;
//! - operations;
//! - devices;
//! - execution environments;
//! - topology size;
//! - circuit depth;
//! - distributed resources.
//!
//! Concrete limits come from:
//!
//! - target capabilities;
//! - resource availability;
//! - resilience policy;
//! - execution budgets;
//! - security policy;
//! - caller/runtime configuration.
//!
//! Therefore the same logical program can be adapted from a tiny target to a
//! substantially larger target without changing this contract.
//!
//! "Infinity" means that this module imposes no artificial finite machine-size
//! ceiling. Every actual execution remains bounded by the resources available
//! to that execution.
//!
//! ============================================================================
//! Canonical semantic ownership
//! ============================================================================
//!
//! The canonical semantic IR is:
//!
//!     crate::quantum::ir
//!
//! Canonical qubit identity is:
//!
//!     crate::quantum::ir::qubit::QubitId
//!     crate::quantum::ir::qubit::PhysicalQubitId
//!
//! This module does not define another qubit identity type.
//!
//! Adaptation is allowed to carry opaque resource identities because resource
//! discovery and hardware ownership belong elsewhere.
//!
//! ============================================================================
//! Action ownership
//! ============================================================================
//!
//! The canonical action contract is:
//!
//!     crate::quantum::resilience::planning::action
//!
//! In particular:
//!
//!     RecoveryAction
//!     ActionKind
//!     ActionPayload
//!
//! An adapter MUST interpret those declarations rather than creating another
//! adaptation-action enum.
//!
//! ============================================================================
//! Error ownership
//! ============================================================================
//!
//! Fallible operations return:
//!
//!     crate::quantum::resilience::errors::ResilienceResult<T>
//!
//! and use:
//!
//!     ResilienceError
//!     ResilienceErrorCode
//!
//! No second resilience error hierarchy is created here.
//!
//! ============================================================================
//! Safety model
//! ============================================================================
//!
//! An adapter is an implementation mechanism, NOT an authorization boundary.
//!
//! Execution is permitted only after the surrounding system establishes:
//!
//!     policy validity
//!     capability validity
//!     feasibility
//!     security authorization
//!     semantic compatibility
//!     execution preconditions
//!
//! The adapter MUST NOT silently override those decisions.
//!
//! ============================================================================
//! Determinism
//! ============================================================================
//!
//! Adapters receive all decision-relevant state explicitly through
//! `AdaptationRequest`.
//!
//! They must not depend on hidden global mutable state.
//!
//! An adapter may be internally nondeterministic only when the request
//! explicitly permits nondeterminism and records the necessary provenance.
//!
//! The default contract requires deterministic behavior for equal:
//!
//!     adapter identity
//!     action
//!     request state
//!     configuration
//!
//! ============================================================================
//! Transactional semantics
//! ============================================================================
//!
//! Adaptation may produce a candidate transformation without committing it.
//!
//! The adapter therefore distinguishes:
//!
//!     Prepare -> PreparedCandidate
//!
//! from:
//!
//!     Commit -> committed AdaptationResult
//!
//! An adapter MUST NOT claim that an adaptation was committed merely because a
//! candidate was successfully prepared.
//!
//! This separation is important for:
//!
//! - semantic verification;
//! - concurrent execution;
//! - stale-state detection;
//! - rollback;
//! - deterministic replay;
//! - distributed execution.
//!
//! ============================================================================
//! Versioning
//! ============================================================================
//!
//! This file owns a stable schema for the adapter contract.
//!
//! Changes to the serialized/public meaning of these types require an explicit
//! schema-version change.
//!
//! ============================================================================
//! Rust contract
//! ============================================================================
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021
//! - stable Rust
//! - no nightly features
//! - safe Rust only
//! - no unsafe code
//!
//! ============================================================================
//! Integration contract
//! ============================================================================
//!
//! planning/action.rs
//!     |
//!     v
//! AdaptationRequest
//!     |
//!     v
//! AdaptationAdapterRegistry
//!     |
//!     +--> remapping.rs
//!     +--> rerouting.rs
//!     +--> rescheduling.rs
//!     +--> recompilation.rs
//!     +--> reoptimization.rs
//!     +--> qec_adaptation.rs
//!     +--> backend_selection.rs
//!
//! Those concrete files implement this trait.
//!
//! This file does not need to be edited when a new adapter implementation is
//! added, provided the implementation conforms to this contract and is
//! registered through the registry owned by the appropriate registry module.
//!
//! ============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_debug_implementations)]
#![deny(clippy::all)]

use std::fmt;
use std::sync::Arc;

use crate::quantum::resilience::errors::{
    ResilienceError,
    ResilienceErrorCode,
    ResilienceResult,
};
use crate::quantum::resilience::planning::action::{
    ActionKind,
    ActionPayload,
    ActionScope,
    RecoveryAction,
};

// ============================================================================
// Stable schema identity
// ============================================================================

/// Stable schema identifier for the adaptation adapter contract.
pub const ADAPTATION_ADAPTER_SCHEMA_ID: &str =
    "zamani.quantum.resilience.adaptation.adapter";

/// Semantic version of the adaptation adapter contract.
///
/// Increment when externally observable semantics change incompatibly.
pub const ADAPTATION_ADAPTER_SCHEMA_VERSION: u16 = 1;

// ============================================================================
// Stable adapter identity
// ============================================================================

/// Stable identifier for one adaptation implementation.
///
/// Adapter identifiers are semantic identifiers rather than Rust type names.
/// This allows the implementation behind an adapter to change without
/// invalidating persisted plans or provenance.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AdapterId(String);

impl AdapterId {
    /// Creates a validated adapter identifier.
    ///
    /// Identifiers must not be empty or contain ASCII whitespace.
    pub fn new(value: impl Into<String>) -> ResilienceResult<Self> {
        let value = value.into();

        if value.is_empty() {
            return Err(Self::invalid_identifier());
        }

        if value.chars().any(char::is_whitespace) {
            return Err(Self::invalid_identifier());
        }

        Ok(Self(value))
    }

    /// Returns the stable identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the identifier.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }

    fn invalid_identifier() -> ResilienceError {
        ResilienceError::new(ResilienceErrorCode::InvalidIdentifier)
    }
}

impl fmt::Display for AdapterId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// ============================================================================
// Adapter version
// ============================================================================

/// Semantic version of an individual adapter implementation.
///
/// The resilience contract version and adapter implementation version are
/// deliberately separate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AdapterVersion {
    major: u32,
    minor: u32,
    patch: u32,
}

impl AdapterVersion {
    /// Creates an adapter version.
    #[must_use]
    pub const fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Major version.
    #[must_use]
    pub const fn major(self) -> u32 {
        self.major
    }

    /// Minor version.
    #[must_use]
    pub const fn minor(self) -> u32 {
        self.minor
    }

    /// Patch version.
    #[must_use]
    pub const fn patch(self) -> u32 {
        self.patch
    }
}

impl Default for AdapterVersion {
    fn default() -> Self {
        Self::new(1, 0, 0)
    }
}

impl fmt::Display for AdapterVersion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}.{}.{}",
            self.major,
            self.minor,
            self.patch
        )
    }
}

// ============================================================================
// Adapter capability
// ============================================================================

/// Describes the execution properties of an adaptation adapter.
///
/// This is adapter metadata, not hardware capability metadata.
///
/// Hardware capabilities remain owned by the hardware subsystem.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AdapterCapabilities {
    /// Whether the adapter can prepare a candidate without committing it.
    prepare: bool,

    /// Whether the adapter supports transactional commit.
    commit: bool,

    /// Whether the adapter can validate a request before transformation.
    preflight: bool,

    /// Whether the adapter guarantees deterministic transformation for equal
    /// explicit inputs.
    deterministic: bool,

    /// Whether the adapter can operate on a bounded affected region rather
    /// than requiring whole-execution transformation.
    scoped: bool,

    /// Whether the adapter can preserve an existing valid unaffected region.
    partial: bool,

    /// Whether the adapter can report a reverse/rollback candidate.
    reversible: bool,
}

impl AdapterCapabilities {
    /// Creates adapter capabilities.
    #[must_use]
    pub const fn new(
        prepare: bool,
        commit: bool,
        preflight: bool,
        deterministic: bool,
        scoped: bool,
        partial: bool,
        reversible: bool,
    ) -> Self {
        Self {
            prepare,
            commit,
            preflight,
            deterministic,
            scoped,
            partial,
            reversible,
        }
    }

    /// Whether preparation is supported.
    #[must_use]
    pub const fn supports_prepare(self) -> bool {
        self.prepare
    }

    /// Whether commit is supported.
    #[must_use]
    pub const fn supports_commit(self) -> bool {
        self.commit
    }

    /// Whether preflight is supported.
    #[must_use]
    pub const fn supports_preflight(self) -> bool {
        self.preflight
    }

    /// Whether deterministic behavior is guaranteed.
    #[must_use]
    pub const fn deterministic(self) -> bool {
        self.deterministic
    }

    /// Whether scoped adaptation is supported.
    #[must_use]
    pub const fn scoped(self) -> bool {
        self.scoped
    }

    /// Whether partial preservation is supported.
    #[must_use]
    pub const fn partial(self) -> bool {
        self.partial
    }

    /// Whether reversal is supported.
    #[must_use]
    pub const fn reversible(self) -> bool {
        self.reversible
    }
}

impl Default for AdapterCapabilities {
    fn default() -> Self {
        Self::new(
            true,
            true,
            true,
            true,
            true,
            true,
            false,
        )
    }
}

// ============================================================================
// Adaptation phase
// ============================================================================

/// Lifecycle phase of an adaptation operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum AdaptationPhase {
    /// Validate the request without transforming execution state.
    Preflight,

    /// Construct a candidate transformation without committing it.
    Prepare,

    /// Commit a previously prepared candidate.
    Commit,

    /// Verify the committed adaptation.
    Verify,
}

impl AdaptationPhase {
    /// Stable machine-readable representation.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Preflight => "preflight",
            Self::Prepare => "prepare",
            Self::Commit => "commit",
            Self::Verify => "verify",
        }
    }
}

impl fmt::Display for AdaptationPhase {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// Request identity
// ============================================================================

/// Stable execution-generation token.
///
/// It prevents an adapter from accidentally committing a transformation
/// against stale execution state.
///
/// The value is supplied by the execution/state subsystem and has no meaning
/// inside this module.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ExecutionGeneration(String);

impl ExecutionGeneration {
    /// Creates a validated execution-generation identifier.
    pub fn new(value: impl Into<String>) -> ResilienceResult<Self> {
        let value = value.into();

        if value.is_empty() {
            return Err(
                ResilienceError::new(ResilienceErrorCode::InvalidIdentifier)
            );
        }

        Ok(Self(value))
    }

    /// Returns the opaque generation identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ExecutionGeneration {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// ============================================================================
// Semantic revision
// ============================================================================

/// Opaque semantic revision of the computation being adapted.
///
/// The actual canonical IR owns semantic representation and hashing.
/// Resilience only carries the revision needed to detect stale requests.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SemanticRevision(String);

impl SemanticRevision {
    /// Creates a validated semantic revision identifier.
    pub fn new(value: impl Into<String>) -> ResilienceResult<Self> {
        let value = value.into();

        if value.is_empty() {
            return Err(
                ResilienceError::new(ResilienceErrorCode::InvalidIdentifier)
            );
        }

        Ok(Self(value))
    }

    /// Returns the opaque semantic revision.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for SemanticRevision {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// ============================================================================
// Adaptation request
// ============================================================================

/// Immutable request presented to an adaptation adapter.
///
/// This type intentionally contains only information required to perform a
/// safe adaptation contractually.
///
/// The actual quantum program remains owned by `quantum::ir`.
///
/// An implementation that needs canonical IR must receive it through the
/// surrounding execution/compiler integration layer rather than copying or
/// redefining IR types here.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AdaptationRequest {
    /// Canonical resilience action being requested.
    action: RecoveryAction,

    /// Current execution generation.
    generation: ExecutionGeneration,

    /// Semantic revision of the logical computation.
    semantic_revision: SemanticRevision,

    /// Optional opaque execution-environment identity.
    environment: Option<String>,

    /// Optional scope explicitly selected by the planner.
    scope: Option<ActionScope>,

    /// Whether the caller requires deterministic behavior.
    deterministic_required: bool,

    /// Whether preparation/transactional behavior is required.
    transactional_required: bool,
}

impl AdaptationRequest {
    /// Creates an adaptation request.
    pub fn new(
        action: RecoveryAction,
        generation: ExecutionGeneration,
        semantic_revision: SemanticRevision,
    ) -> Self {
        let scope = action_scope(&action);

        Self {
            action,
            generation,
            semantic_revision,
            environment: None,
            scope,
            deterministic_required: true,
            transactional_required: true,
        }
    }

    /// Sets an opaque execution-environment identity.
    pub fn with_environment(mut self, environment: impl Into<String>) -> ResilienceResult<Self> {
        let environment = environment.into();

        if environment.is_empty() {
            return Err(
                ResilienceError::new(ResilienceErrorCode::InvalidIdentifier)
            );
        }

        self.environment = Some(environment);
        Ok(self)
    }

    /// Sets whether deterministic behavior is required.
    #[must_use]
    pub const fn require_determinism(mut self, required: bool) -> Self {
        self.deterministic_required = required;
        self
    }

    /// Sets whether transactional adaptation is required.
    #[must_use]
    pub const fn require_transaction(mut self, required: bool) -> Self {
        self.transactional_required = required;
        self
    }

    /// Returns the action.
    #[must_use]
    pub fn action(&self) -> &RecoveryAction {
        &self.action
    }

    /// Returns the action kind.
    #[must_use]
    pub const fn action_kind(&self) -> ActionKind {
        self.action.kind()
    }

    /// Returns the action payload.
    #[must_use]
    pub fn action_payload(&self) -> &ActionPayload {
        self.action.payload()
    }

    /// Returns execution generation.
    #[must_use]
    pub fn generation(&self) -> &ExecutionGeneration {
        &self.generation
    }

    /// Returns semantic revision.
    #[must_use]
    pub fn semantic_revision(&self) -> &SemanticRevision {
        &self.semantic_revision
    }

    /// Returns optional execution environment.
    #[must_use]
    pub fn environment(&self) -> Option<&str> {
        self.environment.as_deref()
    }

    /// Returns the selected action scope.
    #[must_use]
    pub fn scope(&self) -> Option<&ActionScope> {
        self.scope.as_ref()
    }

    /// Returns whether deterministic behavior is required.
    #[must_use]
    pub const fn deterministic_required(&self) -> bool {
        self.deterministic_required
    }

    /// Returns whether transactional behavior is required.
    #[must_use]
    pub const fn transactional_required(&self) -> bool {
        self.transactional_required
    }

    /// Validates request-level invariants.
    pub fn validate(&self) -> ResilienceResult<()> {
        let action_kind = self.action_kind();

        if action_kind.changes_physical_realization()
            && self.scope.is_none()
        {
            return Err(
                ResilienceError::new(ResilienceErrorCode::MissingInformation)
            );
        }

        if self.deterministic_required && self.action.is_mutating() {
            // The actual adapter capability is checked by the adapter.
            // This request-level validation only establishes that the caller
            // explicitly requires determinism.
        }

        if self.transactional_required && self.action.is_mutating() {
            // Transaction support is adapter-specific and is validated after
            // adapter selection.
        }

        Ok(())
    }
}

// ============================================================================
// Adaptation candidate
// ============================================================================

/// Opaque candidate produced during the prepare phase.
///
/// Concrete adaptation implementations own the meaning of the candidate.
///
/// The resilience contract intentionally does not require a universal
/// serialization format for transformed compiler/runtime objects.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AdaptationCandidate {
    /// Adapter that created the candidate.
    adapter: AdapterId,

    /// Action that produced the candidate.
    action: ActionKind,

    /// Execution generation against which the candidate was prepared.
    generation: ExecutionGeneration,

    /// Semantic revision against which the candidate was prepared.
    semantic_revision: SemanticRevision,

    /// Opaque candidate identity.
    identity: String,
}

impl AdaptationCandidate {
    /// Creates a validated candidate.
    pub fn new(
        adapter: AdapterId,
        action: ActionKind,
        generation: ExecutionGeneration,
        semantic_revision: SemanticRevision,
        identity: impl Into<String>,
    ) -> ResilienceResult<Self> {
        let identity = identity.into();

        if identity.is_empty() {
            return Err(
                ResilienceError::new(ResilienceErrorCode::InvalidIdentifier)
            );
        }

        Ok(Self {
            adapter,
            action,
            generation,
            semantic_revision,
            identity,
        })
    }

    /// Adapter identity.
    #[must_use]
    pub fn adapter(&self) -> &AdapterId {
        &self.adapter
    }

    /// Action represented by the candidate.
    #[must_use]
    pub const fn action(&self) -> ActionKind {
        self.action
    }

    /// Execution generation.
    #[must_use]
    pub fn generation(&self) -> &ExecutionGeneration {
        &self.generation
    }

    /// Semantic revision.
    #[must_use]
    pub fn semantic_revision(&self) -> &SemanticRevision {
        &self.semantic_revision
    }

    /// Opaque candidate identity.
    #[must_use]
    pub fn identity(&self) -> &str {
        &self.identity
    }
}

// ============================================================================
// Adaptation status
// ============================================================================

/// Result state of an adaptation operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum AdaptationStatus {
    /// Request was accepted for processing.
    Prepared,

    /// Candidate was committed successfully.
    Committed,

    /// Adaptation completed but requires downstream verification.
    CommittedPendingVerification,

    /// Adaptation could not be performed.
    Rejected,

    /// Adaptation could not be completed because execution state changed.
    Stale,

    /// Adapter deliberately declined the request because another adapter may
    /// be more appropriate.
    Unsupported,
}

impl AdaptationStatus {
    /// Stable machine-readable representation.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Prepared => "prepared",
            Self::Committed => "committed",
            Self::CommittedPendingVerification => {
                "committed_pending_verification"
            }
            Self::Rejected => "rejected",
            Self::Stale => "stale",
            Self::Unsupported => "unsupported",
        }
    }

    /// Returns whether the operation changed execution state.
    #[must_use]
    pub const fn changed_state(self) -> bool {
        matches!(
            self,
            Self::Committed | Self::CommittedPendingVerification
        )
    }

    /// Returns whether downstream verification is required.
    #[must_use]
    pub const fn requires_verification(self) -> bool {
        matches!(
            self,
            Self::Committed | Self::CommittedPendingVerification
        )
    }
}

impl fmt::Display for AdaptationStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// Adaptation result
// ============================================================================

/// Immutable result of an adaptation operation.
///
/// This result describes what happened; it does not perform verification.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AdaptationResult {
    /// Adapter that handled the operation.
    adapter: AdapterId,

    /// Action handled.
    action: ActionKind,

    /// Final adaptation status.
    status: AdaptationStatus,

    /// Execution generation after the operation.
    generation: ExecutionGeneration,

    /// Semantic revision after the operation.
    semantic_revision: SemanticRevision,

    /// Optional committed candidate identity.
    candidate: Option<AdaptationCandidate>,

    /// Whether semantic verification must happen before acceptance.
    verification_required: bool,

    /// Whether the resulting realization should be considered new by
    /// downstream routing/scheduling/compiler layers.
    realization_changed: bool,
}

impl AdaptationResult {
    /// Creates an adaptation result.
    pub fn new(
        adapter: AdapterId,
        action: ActionKind,
        status: AdaptationStatus,
        generation: ExecutionGeneration,
        semantic_revision: SemanticRevision,
    ) -> Self {
        let verification_required = status.requires_verification();
        let realization_changed = action.changes_physical_realization()
            && status.changed_state();

        Self {
            adapter,
            action,
            status,
            generation,
            semantic_revision,
            candidate: None,
            verification_required,
            realization_changed,
        }
    }

    /// Associates a prepared/committed candidate.
    #[must_use]
    pub fn with_candidate(mut self, candidate: AdaptationCandidate) -> Self {
        self.candidate = Some(candidate);
        self
    }

    /// Adapter identity.
    #[must_use]
    pub fn adapter(&self) -> &AdapterId {
        &self.adapter
    }

    /// Action kind.
    #[must_use]
    pub const fn action(&self) -> ActionKind {
        self.action
    }

    /// Status.
    #[must_use]
    pub const fn status(&self) -> AdaptationStatus {
        self.status
    }

    /// Current execution generation.
    #[must_use]
    pub fn generation(&self) -> &ExecutionGeneration {
        &self.generation
    }

    /// Current semantic revision.
    #[must_use]
    pub fn semantic_revision(&self) -> &SemanticRevision {
        &self.semantic_revision
    }

    /// Optional candidate.
    #[must_use]
    pub fn candidate(&self) -> Option<&AdaptationCandidate> {
        self.candidate.as_ref()
    }

    /// Whether verification is required.
    #[must_use]
    pub const fn verification_required(&self) -> bool {
        self.verification_required
    }

    /// Whether the physical realization changed.
    #[must_use]
    pub const fn realization_changed(&self) -> bool {
        self.realization_changed
    }
}

// ============================================================================
// Adapter operation
// ============================================================================

/// Operation requested from an adapter.
///
/// The explicit phase prevents a prepare-only adapter from accidentally being
/// treated as a committed transformation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AdapterOperation {
    /// Validate the request without changing state.
    Preflight,

    /// Prepare a candidate without committing execution state.
    Prepare,

    /// Commit a prepared candidate.
    Commit {
        /// Candidate returned by a previous prepare operation.
        candidate: AdaptationCandidate,
    },

    /// Verify that the adapter's committed transformation remains internally
    /// consistent before the external verification subsystem performs
    /// semantic acceptance.
    Verify {
        /// Candidate associated with the adaptation.
        candidate: AdaptationCandidate,
    },
}

impl AdapterOperation {
    /// Returns the lifecycle phase.
    #[must_use]
    pub const fn phase(&self) -> AdaptationPhase {
        match self {
            Self::Preflight => AdaptationPhase::Preflight,
            Self::Prepare => AdaptationPhase::Prepare,
            Self::Commit { .. } => AdaptationPhase::Commit,
            Self::Verify { .. } => AdaptationPhase::Verify,
        }
    }
}

// ============================================================================
// Adapter trait
// ============================================================================

/// Stable contract implemented by every concrete adaptation strategy.
///
/// Concrete implementations belong in:
///
/// - `remapping.rs`;
/// - `rerouting.rs`;
/// - `rescheduling.rs`;
/// - `recompilation.rs`;
/// - `reoptimization.rs`;
/// - `qec_adaptation.rs`;
/// - `backend_selection.rs`.
///
/// The trait deliberately contains no hardware/provider types.
///
/// Implementations may internally use those authoritative subsystems through
/// dependency injection, but this contract remains provider-neutral.
pub trait AdaptationAdapter: Send + Sync + fmt::Debug {
    /// Returns the stable adapter identifier.
    fn id(&self) -> &AdapterId;

    /// Returns the adapter implementation version.
    fn version(&self) -> AdapterVersion;

    /// Returns adapter capabilities.
    fn capabilities(&self) -> AdapterCapabilities;

    /// Returns the action kinds this adapter can handle.
    ///
    /// The returned slice must be stable for the lifetime of the adapter.
    fn supported_actions(&self) -> &[ActionKind];

    /// Returns whether this adapter can handle the supplied request.
    ///
    /// This method must be side-effect free.
    fn supports(&self, request: &AdaptationRequest) -> bool {
        self.supported_actions()
            .iter()
            .any(|kind| *kind == request.action_kind())
    }

    /// Performs an adaptation operation.
    ///
    /// The default implementation rejects the operation. Concrete adapters
    /// must explicitly implement operations they support.
    fn execute(
        &self,
        operation: &AdapterOperation,
        request: &AdaptationRequest,
    ) -> ResilienceResult<AdaptationResult>;

    /// Performs side-effect-free validation.
    fn preflight(
        &self,
        request: &AdaptationRequest,
    ) -> ResilienceResult<()> {
        request.validate()?;

        if !self.supports(request) {
            return Err(
                ResilienceError::new(
                    ResilienceErrorCode::CapabilityUnavailable
                )
            );
        }

        if request.deterministic_required()
            && !self.capabilities().deterministic()
        {
            return Err(
                ResilienceError::new(
                    ResilienceErrorCode::CompatibilityFailure
                )
            );
        }

        if request.transactional_required()
            && !self.capabilities().supports_prepare()
        {
            return Err(
                ResilienceError::new(
                    ResilienceErrorCode::CompatibilityFailure
                )
            );
        }

        Ok(())
    }

    /// Prepares an adaptation candidate.
    fn prepare(
        &self,
        request: &AdaptationRequest,
    ) -> ResilienceResult<AdaptationResult> {
        if !self.capabilities().supports_prepare() {
            return Err(
                ResilienceError::new(
                    ResilienceErrorCode::CapabilityUnavailable
                )
            );
        }

        self.preflight(request)?;

        self.execute(&AdapterOperation::Prepare, request)
    }

    /// Commits a prepared adaptation candidate.
    fn commit(
        &self,
        request: &AdaptationRequest,
        candidate: AdaptationCandidate,
    ) -> ResilienceResult<AdaptationResult> {
        if !self.capabilities().supports_commit() {
            return Err(
                ResilienceError::new(
                    ResilienceErrorCode::CapabilityUnavailable
                )
            );
        }

        validate_candidate(request, &candidate)?;

        self.execute(
            &AdapterOperation::Commit { candidate },
            request,
        )
    }

    /// Performs adapter-local verification.
    fn verify(
        &self,
        request: &AdaptationRequest,
        candidate: AdaptationCandidate,
    ) -> ResilienceResult<AdaptationResult> {
        validate_candidate(request, &candidate)?;

        self.execute(
            &AdapterOperation::Verify { candidate },
            request,
        )
    }
}

// ============================================================================
// Shared adapter handle
// ============================================================================

/// Thread-safe owned adapter handle.
///
/// The resilience registry can store these handles without knowing the
/// concrete adapter type.
pub type AdaptationAdapterHandle = Arc<dyn AdaptationAdapter>;

// ============================================================================
// Adapter collection
// ============================================================================

/// Immutable collection of adapter handles.
///
/// This is intentionally not a registry implementation. The registry module
/// owns registration, replacement and lifecycle policy.
///
/// This type only provides deterministic read-only selection over an already
/// assembled adapter collection.
#[derive(Clone, Default)]
pub struct AdaptationAdapterSet {
    adapters: Arc<[AdaptationAdapterHandle]>,
}

impl fmt::Debug for AdaptationAdapterSet {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AdaptationAdapterSet")
            .field("count", &self.adapters.len())
            .finish()
    }
}

impl AdaptationAdapterSet {
    /// Creates an adapter set.
    ///
    /// Duplicate adapter IDs are rejected because ambiguous identity would
    /// make deterministic replay impossible.
    pub fn new(
        adapters: impl IntoIterator<Item = AdaptationAdapterHandle>,
    ) -> ResilienceResult<Self> {
        let mut values: Vec<AdaptationAdapterHandle> =
            adapters.into_iter().collect();

        values.sort_by(|left, right| left.id().cmp(right.id()));

        for pair in values.windows(2) {
            if pair[0].id() == pair[1].id() {
                return Err(
                    ResilienceError::new(
                        ResilienceErrorCode::InvalidConfiguration
                    )
                );
            }
        }

        Ok(Self {
            adapters: Arc::from(values),
        })
    }

    /// Returns the number of registered adapters.
    #[must_use]
    pub fn len(&self) -> usize {
        self.adapters.len()
    }

    /// Returns whether no adapters are available.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.adapters.is_empty()
    }

    /// Returns all adapters in deterministic identity order.
    #[must_use]
    pub fn adapters(&self) -> &[AdaptationAdapterHandle] {
        self.adapters.as_ref()
    }

    /// Selects the unique adapter that supports a request.
    ///
    /// Selection is deterministic because adapter identities are sorted.
    ///
    /// If multiple adapters support the same request, this method rejects the
    /// ambiguity instead of silently selecting one based on registration
    /// order.
    pub fn select(
        &self,
        request: &AdaptationRequest,
    ) -> ResilienceResult<AdaptationAdapterHandle> {
        request.validate()?;

        let mut selected: Option<AdaptationAdapterHandle> = None;

        for adapter in self.adapters.iter() {
            if !adapter.supports(request) {
                continue;
            }

            if selected.is_some() {
                return Err(
                    ResilienceError::new(
                        ResilienceErrorCode::PlanSelectionFailed
                    )
                );
            }

            selected = Some(Arc::clone(adapter));
        }

        selected.ok_or_else(|| {
            ResilienceError::new(
                ResilienceErrorCode::CapabilityUnavailable
            )
        })
    }

    /// Selects all compatible adapters in deterministic order.
    ///
    /// This is useful when the planner deliberately wants multiple candidate
    /// implementations evaluated and ranked elsewhere.
    pub fn compatible(
        &self,
        request: &AdaptationRequest,
    ) -> ResilienceResult<Vec<AdaptationAdapterHandle>> {
        request.validate()?;

        Ok(self
            .adapters
            .iter()
            .filter(|adapter| adapter.supports(request))
            .cloned()
            .collect())
    }
}

// ============================================================================
// Validation helpers
// ============================================================================

/// Extracts the scope carried by an action.
///
/// The action model remains authoritative; this helper merely provides a
/// uniform read-only view to adaptation.
fn action_scope(action: &RecoveryAction) -> Option<ActionScope> {
    match action.payload() {
        ActionPayload::Retry { scope } => scope.clone(),

        ActionPayload::Restart { scope }
        | ActionPayload::Checkpoint { scope }
        | ActionPayload::Remap { scope }
        | ActionPayload::Reroute { scope }
        | ActionPayload::Reschedule { scope }
        | ActionPayload::Recompile { scope }
        | ActionPayload::Reoptimize { scope }
        | ActionPayload::AdaptQec { scope }
        | ActionPayload::Escalate { scope }
        | ActionPayload::Abort { scope } => Some(scope.clone()),

        ActionPayload::Resume { .. }
        | ActionPayload::Rollback { .. }
        | ActionPayload::Mitigate { .. }
        | ActionPayload::Migrate { .. }
        | ActionPayload::QuarantineResource { .. }
        | ActionPayload::Compensate { .. } => None,
    }
}

/// Validates that a candidate belongs to the current request.
///
/// This prevents stale candidates from being committed after the execution
/// state or semantic program revision has changed.
fn validate_candidate(
    request: &AdaptationRequest,
    candidate: &AdaptationCandidate,
) -> ResilienceResult<()> {
    if candidate.action() != request.action_kind() {
        return Err(
            ResilienceError::new(
                ResilienceErrorCode::InvalidArgument
            )
        );
    }

    if candidate.generation() != request.generation() {
        return Err(
            ResilienceError::new(
                ResilienceErrorCode::PlanStale
            )
        );
    }

    if candidate.semantic_revision()
        != request.semantic_revision()
    {
        return Err(
            ResilienceError::new(
                ResilienceErrorCode::SemanticAdaptationViolation
            )
        );
    }

    Ok(())
}

// ============================================================================
// Standard unsupported-operation implementation
// ============================================================================

/// Creates the canonical error for an adapter that cannot perform a requested
/// operation.
///
/// Concrete adapters may use this helper when they intentionally expose only
/// part of the lifecycle.
#[must_use]
pub fn unsupported_operation() -> ResilienceError {
    ResilienceError::new(ResilienceErrorCode::CapabilityUnavailable)
}

// ============================================================================
// Standard adapter-local failure helpers
// ============================================================================

/// Creates an adaptation failure for an adapter implementation.
#[must_use]
pub fn adaptation_failed() -> ResilienceError {
    ResilienceError::new(ResilienceErrorCode::AdaptationFailed)
}

/// Creates a stale-plan error for an adapter implementation.
#[must_use]
pub fn stale_request() -> ResilienceError {
    ResilienceError::new(ResilienceErrorCode::PlanStale)
}

/// Creates a semantic compatibility error.
#[must_use]
pub fn semantic_incompatibility() -> ResilienceError {
    ResilienceError::new(
        ResilienceErrorCode::SemanticAdaptationViolation
    )
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct TestAdapter {
        id: AdapterId,
        version: AdapterVersion,
        capabilities: AdapterCapabilities,
        supported: &'static [ActionKind],
    }

    impl TestAdapter {
        fn new(
            id: &'static str,
            supported: &'static [ActionKind],
        ) -> Self {
            Self {
                id: AdapterId::new(id)
                    .expect("static test adapter ID must be valid"),
                version: AdapterVersion::new(1, 0, 0),
                capabilities: AdapterCapabilities::default(),
                supported,
            }
        }
    }

    impl AdaptationAdapter for TestAdapter {
        fn id(&self) -> &AdapterId {
            &self.id
        }

        fn version(&self) -> AdapterVersion {
            self.version
        }

        fn capabilities(&self) -> AdapterCapabilities {
            self.capabilities
        }

        fn supported_actions(&self) -> &[ActionKind] {
            self.supported
        }

        fn execute(
            &self,
            operation: &AdapterOperation,
            request: &AdaptationRequest,
        ) -> ResilienceResult<AdaptationResult> {
            let status = match operation {
                AdapterOperation::Preflight => {
                    AdaptationStatus::Prepared
                }
                AdapterOperation::Prepare => {
                    AdaptationStatus::Prepared
                }
                AdapterOperation::Commit { .. } => {
                    AdaptationStatus::CommittedPendingVerification
                }
                AdapterOperation::Verify { .. } => {
                    AdaptationStatus::Committed
                }
            };

            Ok(AdaptationResult::new(
                self.id.clone(),
                request.action_kind(),
                status,
                request.generation().clone(),
                request.semantic_revision().clone(),
            ))
        }
    }

    fn sample_request() -> AdaptationRequest {
        let action_id =
            crate::quantum::resilience::planning::action::ActionId::new(1)
                .expect("non-zero action ID");

        let action = RecoveryAction::remap(
            action_id,
            crate::quantum::resilience::planning::action::ActionReason::
                PhysicalRealizationInvalid,
            ActionScope::Computation,
        )
        .expect("valid remap action");

        AdaptationRequest::new(
            action,
            ExecutionGeneration::new("generation-1")
                .expect("valid generation"),
            SemanticRevision::new("semantic-1")
                .expect("valid semantic revision"),
        )
    }

    #[test]
    fn adapter_id_rejects_empty_values() {
        assert!(AdapterId::new("").is_err());
    }

    #[test]
    fn adapter_id_rejects_whitespace() {
        assert!(AdapterId::new("adapter name").is_err());
    }

    #[test]
    fn adapter_version_is_stable() {
        let version = AdapterVersion::new(2, 3, 4);

        assert_eq!(version.major(), 2);
        assert_eq!(version.minor(), 3);
        assert_eq!(version.patch(), 4);
        assert_eq!(version.to_string(), "2.3.4");
    }

    #[test]
    fn request_extracts_action_kind() {
        let request = sample_request();

        assert_eq!(request.action_kind(), ActionKind::Remap);
        assert!(request.scope().is_some());
    }

    #[test]
    fn adapter_support_is_action_based() {
        let adapter = TestAdapter::new(
            "test.remap",
            &[ActionKind::Remap],
        );

        let request = sample_request();

        assert!(adapter.supports(&request));
    }

    #[test]
    fn unsupported_action_is_not_supported() {
        let adapter = TestAdapter::new(
            "test.remap",
            &[ActionKind::Remap],
        );

        let request = sample_request();

        assert!(adapter.supports(&request));

        let supported =
            adapter.supported_actions();

        assert_eq!(supported, &[ActionKind::Remap]);
    }

    #[test]
    fn adapter_set_is_deterministically_sorted() {
        let first = Arc::new(TestAdapter::new(
            "z.remap",
            &[ActionKind::Remap],
        ));

        let second = Arc::new(TestAdapter::new(
            "a.remap",
            &[ActionKind::Remap],
        ));

        let set = AdaptationAdapterSet::new([
            first as AdaptationAdapterHandle,
            second as AdaptationAdapterHandle,
        ])
        .expect("unique IDs");

        assert_eq!(set.adapters()[0].id().as_str(), "a.remap");
        assert_eq!(set.adapters()[1].id().as_str(), "z.remap");
    }

    #[test]
    fn duplicate_adapter_ids_are_rejected() {
        let first = Arc::new(TestAdapter::new(
            "same",
            &[ActionKind::Remap],
        ));

        let second = Arc::new(TestAdapter::new(
            "same",
            &[ActionKind::Reroute],
        ));

        let result = AdaptationAdapterSet::new([
            first as AdaptationAdapterHandle,
            second as AdaptationAdapterHandle,
        ]);

        assert!(result.is_err());
    }

    #[test]
    fn ambiguous_selection_is_rejected() {
        let first = Arc::new(TestAdapter::new(
            "a.remap",
            &[ActionKind::Remap],
        ));

        let second = Arc::new(TestAdapter::new(
            "b.remap",
            &[ActionKind::Remap],
        ));

        let set = AdaptationAdapterSet::new([
            first as AdaptationAdapterHandle,
            second as AdaptationAdapterHandle,
        ])
        .expect("unique IDs");

        let result = set.select(&sample_request());

        assert!(result.is_err());
    }

    #[test]
    fn unique_selection_succeeds() {
        let adapter = Arc::new(TestAdapter::new(
            "a.remap",
            &[ActionKind::Remap],
        ));

        let set = AdaptationAdapterSet::new([
            adapter as AdaptationAdapterHandle,
        ])
        .expect("unique adapter");

        let selected = set
            .select(&sample_request())
            .expect("one compatible adapter");

        assert_eq!(selected.id().as_str(), "a.remap");
    }

    #[test]
    fn prepare_commit_and_verify_preserve_identity() {
        let adapter = TestAdapter::new(
            "a.remap",
            &[ActionKind::Remap],
        );

        let request = sample_request();

        let prepared = adapter
            .prepare(&request)
            .expect("prepare succeeds");

        assert_eq!(prepared.status(), AdaptationStatus::Prepared);

        let candidate = AdaptationCandidate::new(
            adapter.id().clone(),
            ActionKind::Remap,
            request.generation().clone(),
            request.semantic_revision().clone(),
            "candidate-1",
        )
        .expect("valid candidate");

        let committed = adapter
            .commit(&request, candidate.clone())
            .expect("commit succeeds");

        assert!(
            committed.status()
                == AdaptationStatus::CommittedPendingVerification
        );

        let verified = adapter
            .verify(&request, candidate)
            .expect("verify succeeds");

        assert_eq!(
            verified.status(),
            AdaptationStatus::Committed
        );
    }

    #[test]
    fn stale_generation_is_rejected() {
        let adapter = TestAdapter::new(
            "a.remap",
            &[ActionKind::Remap],
        );

        let request = sample_request();

        let candidate = AdaptationCandidate::new(
            adapter.id().clone(),
            ActionKind::Remap,
            ExecutionGeneration::new("different-generation")
                .expect("valid generation"),
            request.semantic_revision().clone(),
            "candidate-1",
        )
        .expect("valid candidate");

        let result = adapter.commit(&request, candidate);

        assert!(result.is_err());
    }

    #[test]
    fn stale_semantic_revision_is_rejected() {
        let adapter = TestAdapter::new(
            "a.remap",
            &[ActionKind::Remap],
        );

        let request = sample_request();

        let candidate = AdaptationCandidate::new(
            adapter.id().clone(),
            ActionKind::Remap,
            request.generation().clone(),
            SemanticRevision::new("different-semantic")
                .expect("valid semantic revision"),
            "candidate-1",
        )
        .expect("valid candidate");

        let result = adapter.commit(&request, candidate);

        assert!(result.is_err());
    }
}