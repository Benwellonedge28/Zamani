//! Zamani Quantum Resilience — Execution Migration
//!
//! This module implements safe, backend-independent migration of a quantum
//! execution from one execution target to another.
//!
//! # Architectural role
//!
//! `recovery/migration.rs` is the recovery-layer implementation of:
//!
//! ```text
//!                         recovery plan
//!                              |
//!                              v
//!                       migration request
//!                              |
//!             +----------------+----------------+
//!             |                                 |
//!             v                                 v
//!       source validation                target validation
//!             |                                 |
//!             +----------------+----------------+
//!                              |
//!                              v
//!                    migration preparation
//!                              |
//!                              v
//!                    state transfer/rebuild
//!                              |
//!                              v
//!                         verification
//!                              |
//!                    +---------+---------+
//!                    |                   |
//!                    v                   v
//!                  accept             reject/escalate
//! ```
//!
//! Migration is deliberately different from:
//!
//! - routing;
//! - scheduling;
//! - recompilation;
//! - checkpoint creation;
//! - rollback;
//! - retry;
//! - backend discovery;
//! - hardware calibration;
//! - QEC;
//! - optimization.
//!
//! Those responsibilities remain in their respective subsystems.
//!
//! This module decides and executes the *recovery-level transfer* between
//! execution targets using injected contracts supplied by those subsystems.
//!
//! # Critical quantum-safety rule
//!
//! Migration MUST NOT assume that an arbitrary unknown quantum state can be
//! serialized and transferred.
//!
//! A migration is valid only when the execution state is one of:
//!
//! - reconstructible from a canonical program/checkpoint;
//! - represented by a valid logical/QEC checkpoint;
//! - located at a measurement/classical boundary;
//! - represented by a provider-managed state that explicitly supports migration;
//! - otherwise explicitly declared migratable by the execution provider.
//!
//! If none of these conditions hold, migration is rejected.
//!
//! # Write-once / scale-everywhere rule
//!
//! This module contains no assumptions about:
//!
//! - number of qubits;
//! - number of devices;
//! - number of providers;
//! - topology size;
//! - circuit depth;
//! - number of operations;
//! - retry count;
//! - migration count;
//! - maximum resource count;
//! - provider names;
//! - physical-qubit numbering;
//! - fixed hardware generations.
//!
//! All resource limits are supplied by the caller through policy/capability
//! contracts.
//!
//! # Separation from routing
//!
//! Migration does not decide how logical qubits map to physical qubits.
//! After migration preparation, the destination integration layer may invoke
//! routing/remapping as required.
//!
//! If a resource mapping is represented by canonical quantum IR identities,
//! callers must use:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! This file intentionally does not duplicate those types.
//!
//! # Separation from scheduling
//!
//! Migration does not implement scheduling. A destination scheduler may rebuild
//! the schedule using destination timing/resource capabilities.
//!
//! # Separation from checkpointing
//!
//! Migration consumes a migration-capable checkpoint/state descriptor. The
//! checkpoint subsystem owns persistence, storage, manifests and integrity.
//!
//! # Separation from verification
//!
//! Migration does not decide whether the resulting execution is semantically
//! correct. A verifier must explicitly accept the migrated execution before
//! the result is considered successful.
//!
//! # Determinism
//!
//! Migration planning and validation are deterministic when supplied with the
//! same request, capabilities, checkpoint metadata and policy.
//!
//! External execution itself may of course be nondeterministic if the selected
//! quantum computation or hardware is nondeterministic.
//!
//! # Transactional semantics
//!
//! The migration operation is conceptually transactional:
//!
//! ```text
//! validate source
//!       |
//!       v
//! validate destination
//!       |
//!       v
//! prepare migration
//!       |
//!       v
//! execute transfer/reconstruction
//!       |
//!       v
//! verify
//!       |
//!       +---- failure ----> rejected/escalated
//!       |
//!       v
//! accepted
//! ```
//!
//! A partially completed migration is never reported as an accepted migration.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe`.
//!
//! -----------------------------------------------------------------------------
//! Implementation
//! -----------------------------------------------------------------------------

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

// =============================================================================
// Stable identifiers
// =============================================================================

/// Opaque identifier for a migration operation.
///
/// The value is deliberately provider-independent.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct MigrationId(String);

impl MigrationId {
    /// Creates a migration identifier.
    pub fn new<S: Into<String>>(value: S) -> Result<Self, MigrationError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(MigrationError::InvalidIdentifier {
                field: "migration_id",
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

impl fmt::Display for MigrationId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Opaque execution identity.
///
/// This is intentionally not a provider-specific job ID.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ExecutionId(String);

impl ExecutionId {
    /// Creates an execution identity.
    pub fn new<S: Into<String>>(value: S) -> Result<Self, MigrationError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(MigrationError::InvalidIdentifier {
                field: "execution_id",
            });
        }

        Ok(Self(value))
    }

    /// Returns the identity.
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

/// Opaque target identity.
///
/// It can identify a device, backend, simulator, emulator, logical machine or
/// distributed execution target.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TargetId(String);

impl TargetId {
    /// Creates a target identity.
    pub fn new<S: Into<String>>(value: S) -> Result<Self, MigrationError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(MigrationError::InvalidIdentifier {
                field: "target_id",
            });
        }

        Ok(Self(value))
    }

    /// Returns the target identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for TargetId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Opaque checkpoint identity.
///
/// The checkpoint subsystem owns the actual checkpoint contents.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CheckpointId(String);

impl CheckpointId {
    /// Creates a checkpoint identity.
    pub fn new<S: Into<String>>(value: S) -> Result<Self, MigrationError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(MigrationError::InvalidIdentifier {
                field: "checkpoint_id",
            });
        }

        Ok(Self(value))
    }

    /// Returns the checkpoint identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for CheckpointId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Stable identity for the canonical program/IR representation.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ProgramIdentity(String);

impl ProgramIdentity {
    /// Creates a program identity.
    pub fn new<S: Into<String>>(value: S) -> Result<Self, MigrationError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(MigrationError::InvalidIdentifier {
                field: "program_identity",
            });
        }

        Ok(Self(value))
    }

    /// Returns the identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ProgramIdentity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// =============================================================================
// Migration state model
// =============================================================================

/// Kind of state that can be migrated.
///
/// There is intentionally no generic `ArbitraryQuantumState` variant.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum MigratableStateKind {
    /// Program can be deterministically reconstructed from the canonical
    /// program/IR representation.
    ReconstructibleProgram,

    /// State is represented by a validated classical/measurement boundary.
    MeasurementBoundary,

    /// State is represented by a valid logical/QEC checkpoint.
    LogicalCheckpoint,

    /// State is managed by a provider/runtime that explicitly guarantees
    /// migration support.
    ProviderManagedState,

    /// Only classical execution state is being transferred.
    ClassicalExecutionState,
}

impl MigratableStateKind {
    /// Returns whether this state kind is eligible for migration in principle.
    #[must_use]
    pub const fn is_migratable(self) -> bool {
        true
    }
}

/// Explicit statement about whether quantum state transfer is required.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum QuantumStateTransfer {
    /// No unknown quantum state needs to be transferred.
    NotRequired,

    /// A logical/QEC representation is transferred through an explicit
    /// checkpoint contract.
    LogicalState,

    /// The provider explicitly owns the state transfer mechanism.
    ProviderManaged,

    /// An arbitrary unknown quantum state would have to be serialized.
    ///
    /// This is always rejected by this module.
    ArbitraryUnknownState,
}

/// Replay semantics of the computation.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ReplaySafety {
    /// Replaying from the selected boundary is semantically safe.
    Safe,

    /// Replay is safe only under an externally supplied authorization/policy.
    ConditionallySafe,

    /// Replay could change program semantics or external side effects.
    Unsafe,

    /// Replay safety is not known.
    Unknown,
}

/// Whether external/classical side effects exist.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SideEffectMode {
    /// No externally visible side effects exist.
    None,

    /// Side effects are transactional and can be safely coordinated.
    Transactional,

    /// Side effects exist but are idempotent.
    Idempotent,

    /// Side effects cannot safely be replayed.
    NonReplayable,

    /// Side-effect semantics are not known.
    Unknown,
}

// =============================================================================
// Migration policy
// =============================================================================

/// Policy governing a migration.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MigrationPolicy {
    /// Whether migration is permitted at all.
    pub allow_migration: bool,

    /// Whether migration to another target is permitted.
    pub allow_cross_target: bool,

    /// Whether migration may cross provider boundaries.
    pub allow_cross_provider: bool,

    /// Whether migration may require recompilation.
    pub allow_recompilation: bool,

    /// Whether migration may require rerouting.
    pub allow_rerouting: bool,

    /// Whether migration may require rescheduling.
    pub allow_rescheduling: bool,

    /// Whether logical/QEC adaptation is permitted.
    pub allow_qec_adaptation: bool,

    /// Whether degraded execution may be accepted.
    pub allow_degraded_execution: bool,

    /// Whether downgrade/reconstruction semantics requiring explicit replay are
    /// allowed.
    pub allow_conditional_replay: bool,

    /// Maximum migration wall-clock duration, if policy chooses to impose one.
    ///
    /// This is a recovery-resource limit, not a quantum-machine size limit.
    pub deadline: Option<Duration>,
}

impl Default for MigrationPolicy {
    fn default() -> Self {
        Self {
            allow_migration: true,
            allow_cross_target: true,
            allow_cross_provider: false,
            allow_recompilation: true,
            allow_rerouting: true,
            allow_rescheduling: true,
            allow_qec_adaptation: true,
            allow_degraded_execution: false,
            allow_conditional_replay: false,
            deadline: None,
        }
    }
}

impl MigrationPolicy {
    /// Validates policy consistency.
    pub fn validate(&self) -> Result<(), MigrationError> {
        if !self.allow_migration
            && (self.allow_cross_target
                || self.allow_cross_provider
                || self.allow_recompilation
                || self.allow_rerouting
                || self.allow_rescheduling
                || self.allow_qec_adaptation)
        {
            return Err(MigrationError::InvalidPolicy(
                "migration-related capabilities cannot be enabled when migration is disabled",
            ));
        }

        if !self.allow_cross_target && self.allow_cross_provider {
            return Err(MigrationError::InvalidPolicy(
                "cross-provider migration requires cross-target migration",
            ));
        }

        Ok(())
    }
}

// =============================================================================
// Target capabilities
// =============================================================================

/// Capabilities required to determine whether migration can be performed.
///
/// This is intentionally generic. Concrete hardware capability models remain
/// owned by `quantum::hardware`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MigrationCapabilities {
    /// Whether the target can execute the workload.
    pub executable: bool,

    /// Whether the target can consume the canonical program representation.
    pub canonical_program: bool,

    /// Whether the target can consume the checkpoint representation.
    pub checkpoint_restore: bool,

    /// Whether provider-managed state migration is supported.
    pub provider_state_migration: bool,

    /// Whether logical/QEC state migration is supported.
    pub logical_state_migration: bool,

    /// Whether destination routing may be rebuilt.
    pub rerouting: bool,

    /// Whether destination scheduling may be rebuilt.
    pub rescheduling: bool,

    /// Whether destination recompilation may be performed.
    pub recompilation: bool,

    /// Whether the target can support the required execution semantics.
    pub semantic_compatibility: bool,

    /// Whether degraded operation is supported.
    pub degraded_execution: bool,
}

impl MigrationCapabilities {
    /// Validates the capability declaration.
    pub fn validate(&self) -> Result<(), MigrationError> {
        if !self.executable {
            return Err(MigrationError::CapabilityUnavailable {
                capability: "executable",
            });
        }

        if !self.semantic_compatibility {
            return Err(MigrationError::CapabilityUnavailable {
                capability: "semantic_compatibility",
            });
        }

        Ok(())
    }
}

// =============================================================================
// Migration request
// =============================================================================

/// Immutable request for migration.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MigrationRequest {
    /// Migration operation identity.
    pub migration_id: MigrationId,

    /// Currently executing workload.
    pub execution_id: ExecutionId,

    /// Canonical program identity.
    pub program: ProgramIdentity,

    /// Current execution target.
    pub source_target: TargetId,

    /// Requested destination target.
    pub destination_target: TargetId,

    /// Checkpoint from which migration can reconstruct/resume execution.
    pub checkpoint: Option<CheckpointId>,

    /// Kind of state represented by the checkpoint/boundary.
    pub state_kind: MigratableStateKind,

    /// Whether quantum state transfer is required.
    pub quantum_state_transfer: QuantumStateTransfer,

    /// Replay safety.
    pub replay_safety: ReplaySafety,

    /// Side-effect semantics.
    pub side_effects: SideEffectMode,

    /// Migration policy.
    pub policy: MigrationPolicy,

    /// Caller-supplied metadata.
    pub metadata: BTreeMap<String, String>,
}

impl MigrationRequest {
    /// Validates migration invariants that do not require external services.
    pub fn validate(&self) -> Result<(), MigrationError> {
        self.policy.validate()?;

        if self.source_target == self.destination_target {
            return Err(MigrationError::SameSourceAndDestination);
        }

        if !self.policy.allow_migration {
            return Err(MigrationError::MigrationNotAllowed);
        }

        if self.quantum_state_transfer
            == QuantumStateTransfer::ArbitraryUnknownState
        {
            return Err(MigrationError::ArbitraryQuantumStateTransfer);
        }

        if !self.state_kind.is_migratable() {
            return Err(MigrationError::StateNotMigratable {
                state: self.state_kind,
            });
        }

        match self.replay_safety {
            ReplaySafety::Unsafe => {
                return Err(MigrationError::ReplayUnsafe);
            }
            ReplaySafety::Unknown => {
                if self.state_kind == MigratableStateKind::ReconstructibleProgram {
                    return Err(MigrationError::ReplaySafetyUnknown);
                }
            }
            ReplaySafety::ConditionallySafe => {
                if !self.policy.allow_conditional_replay {
                    return Err(MigrationError::ConditionalReplayNotAllowed);
                }
            }
            ReplaySafety::Safe => {}
        }

        if self.side_effects == SideEffectMode::NonReplayable
            && self.state_kind == MigratableStateKind::ReconstructibleProgram
        {
            return Err(MigrationError::NonReplayableSideEffects);
        }

        if self.side_effects == SideEffectMode::Unknown
            && self.state_kind == MigratableStateKind::ReconstructibleProgram
        {
            return Err(MigrationError::SideEffectSemanticsUnknown);
        }

        match self.quantum_state_transfer {
            QuantumStateTransfer::LogicalState => {
                if self.checkpoint.is_none() {
                    return Err(MigrationError::CheckpointRequired);
                }
            }
            QuantumStateTransfer::ProviderManaged => {
                if self.checkpoint.is_none() {
                    return Err(MigrationError::CheckpointRequired);
                }
            }
            QuantumStateTransfer::NotRequired
            | QuantumStateTransfer::ArbitraryUnknownState => {}
        }

        Ok(())
    }
}

// =============================================================================
// Prepared migration
// =============================================================================

/// Validated migration prepared for execution.
///
/// This object exists so external migration execution cannot accidentally skip
/// preflight validation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreparedMigration {
    /// Migration identity.
    pub migration_id: MigrationId,

    /// Source target.
    pub source_target: TargetId,

    /// Destination target.
    pub destination_target: TargetId,

    /// Execution identity.
    pub execution_id: ExecutionId,

    /// Program identity.
    pub program: ProgramIdentity,

    /// Checkpoint selected for migration.
    pub checkpoint: Option<CheckpointId>,

    /// State representation being migrated.
    pub state_kind: MigratableStateKind,

    /// Whether rerouting may be required.
    pub rerouting_required: bool,

    /// Whether rescheduling may be required.
    pub rescheduling_required: bool,

    /// Whether recompilation may be required.
    pub recompilation_required: bool,

    /// Whether QEC adaptation may be required.
    pub qec_adaptation_allowed: bool,

    /// Destination capabilities used during preparation.
    pub destination_capabilities: MigrationCapabilities,
}

impl PreparedMigration {
    /// Creates a prepared migration after all static checks.
    pub fn prepare(
        request: &MigrationRequest,
        destination_capabilities: MigrationCapabilities,
    ) -> Result<Self, MigrationError> {
        request.validate()?;
        destination_capabilities.validate()?;

        if !request.policy.allow_cross_target {
            return Err(MigrationError::CrossTargetMigrationNotAllowed);
        }

        if request.quantum_state_transfer
            == QuantumStateTransfer::ProviderManaged
            && !destination_capabilities.provider_state_migration
        {
            return Err(MigrationError::CapabilityUnavailable {
                capability: "provider_state_migration",
            });
        }

        if request.quantum_state_transfer
            == QuantumStateTransfer::LogicalState
            && !destination_capabilities.logical_state_migration
        {
            return Err(MigrationError::CapabilityUnavailable {
                capability: "logical_state_migration",
            });
        }

        let requires_recompilation = !destination_capabilities.canonical_program;

        if requires_recompilation && !request.policy.allow_recompilation {
            return Err(MigrationError::RecompilationNotAllowed);
        }

        Ok(Self {
            migration_id: request.migration_id.clone(),
            source_target: request.source_target.clone(),
            destination_target: request.destination_target.clone(),
            execution_id: request.execution_id.clone(),
            program: request.program.clone(),
            checkpoint: request.checkpoint.clone(),
            state_kind: request.state_kind,
            rerouting_required: request.policy.allow_rerouting,
            rescheduling_required: request.policy.allow_rescheduling,
            recompilation_required: requires_recompilation,
            qec_adaptation_allowed: request.policy.allow_qec_adaptation,
            destination_capabilities,
        })
    }
}

// =============================================================================
// Migration execution request
// =============================================================================

/// Provider/runtime-neutral command supplied to the migration executor.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MigrationExecutionRequest {
    /// Prepared migration.
    pub migration: PreparedMigration,

    /// Correlation metadata.
    pub metadata: BTreeMap<String, String>,
}

// =============================================================================
// Migration result
// =============================================================================

/// Execution handle returned by the destination runtime.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MigratedExecution {
    /// Destination execution identity.
    pub execution_id: ExecutionId,

    /// Destination target.
    pub target: TargetId,

    /// Whether the execution has actually resumed.
    pub resumed: bool,

    /// Whether the result is provisional until verification.
    pub provisional: bool,

    /// Optional checkpoint/boundary used.
    pub checkpoint: Option<CheckpointId>,
}

/// Result of semantic verification after migration.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MigrationVerification {
    /// Whether the migrated execution is semantically equivalent.
    pub semantically_equivalent: bool,

    /// Whether destination capability requirements were satisfied.
    pub capability_compatible: bool,

    /// Whether the migrated state is valid.
    pub state_valid: bool,

    /// Whether provenance is complete enough for acceptance.
    pub provenance_complete: bool,

    /// Whether degraded acceptance is explicitly permitted.
    pub degraded: bool,

    /// Verifier confidence represented as a caller-defined scalar.
    ///
    /// This module does not assign a universal statistical meaning to the
    /// value.
    pub confidence: Option<f64>,

    /// Human/machine-readable diagnostics.
    pub diagnostics: Vec<String>,
}

impl MigrationVerification {
    /// Returns whether the migration is acceptable under strict semantics.
    #[must_use]
    pub fn accepted(&self) -> bool {
        self.semantically_equivalent
            && self.capability_compatible
            && self.state_valid
            && self.provenance_complete
    }

    /// Validates the confidence value if present.
    pub fn validate(&self) -> Result<(), MigrationError> {
        if let Some(value) = self.confidence {
            if !value.is_finite() || !(0.0..=1.0).contains(&value) {
                return Err(MigrationError::InvalidConfidence);
            }
        }

        Ok(())
    }
}

// =============================================================================
// Lifecycle
// =============================================================================

/// Migration lifecycle state.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum MigrationStatus {
    /// Request received but not validated.
    Requested,

    /// Static and capability validation completed.
    Prepared,

    /// External migration is running.
    Migrating,

    /// Destination execution exists and is awaiting verification.
    Verifying,

    /// Migration passed all required acceptance checks.
    Accepted,

    /// Migration succeeded but the verifier explicitly classified it as
    /// degraded and policy permits that outcome.
    Degraded,

    /// Migration was rejected before an accepted destination execution.
    Rejected,

    /// Migration was cancelled before completion.
    Cancelled,

    /// Migration failed.
    Failed,

    /// Migration could not safely complete and must be handled by the higher
    /// resilience controller.
    Escalated,
}

impl MigrationStatus {
    /// Returns whether the state is terminal.
    #[must_use]
    pub const fn is_terminal(self) -> bool {
        matches!(
            self,
            Self::Accepted
                | Self::Degraded
                | Self::Rejected
                | Self::Cancelled
                | Self::Failed
                | Self::Escalated
        )
    }
}

/// Valid lifecycle transition.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct MigrationTransition {
    /// Previous state.
    pub from: MigrationStatus,

    /// New state.
    pub to: MigrationStatus,
}

impl MigrationTransition {
    /// Checks whether a lifecycle transition is legal.
    #[must_use]
    pub const fn is_valid(self) -> bool {
        match (self.from, self.to) {
            (MigrationStatus::Requested, MigrationStatus::Prepared)
            | (MigrationStatus::Requested, MigrationStatus::Rejected)
            | (MigrationStatus::Requested, MigrationStatus::Cancelled)
            | (MigrationStatus::Prepared, MigrationStatus::Migrating)
            | (MigrationStatus::Prepared, MigrationStatus::Cancelled)
            | (MigrationStatus::Prepared, MigrationStatus::Rejected)
            | (MigrationStatus::Migrating, MigrationStatus::Verifying)
            | (MigrationStatus::Migrating, MigrationStatus::Failed)
            | (MigrationStatus::Migrating, MigrationStatus::Cancelled)
            | (MigrationStatus::Migrating, MigrationStatus::Escalated)
            | (MigrationStatus::Verifying, MigrationStatus::Accepted)
            | (MigrationStatus::Verifying, MigrationStatus::Degraded)
            | (MigrationStatus::Verifying, MigrationStatus::Rejected)
            | (MigrationStatus::Verifying, MigrationStatus::Failed)
            | (MigrationStatus::Verifying, MigrationStatus::Escalated) => true,

            _ => false,
        }
    }
}

// =============================================================================
// Outcome
// =============================================================================

/// Immutable migration outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MigrationOutcome {
    /// Migration identity.
    pub migration_id: MigrationId,

    /// Final status.
    pub status: MigrationStatus,

    /// Original execution.
    pub source_execution: ExecutionId,

    /// Destination execution if one was created.
    pub destination_execution: Option<ExecutionId>,

    /// Source target.
    pub source_target: TargetId,

    /// Destination target.
    pub destination_target: TargetId,

    /// Checkpoint used.
    pub checkpoint: Option<CheckpointId>,

    /// Verification result.
    pub verification: Option<MigrationVerification>,

    /// Provenance metadata.
    pub provenance: MigrationProvenance,

    /// Migration duration if available.
    pub elapsed: Option<Duration>,

    /// Structured diagnostics.
    pub diagnostics: Vec<String>,
}

impl MigrationOutcome {
    /// Returns whether migration is fully accepted.
    #[must_use]
    pub fn accepted(&self) -> bool {
        matches!(
            self.status,
            MigrationStatus::Accepted | MigrationStatus::Degraded
        )
    }
}

/// Immutable provenance record.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MigrationProvenance {
    /// Migration implementation identifier.
    pub implementation: String,

    /// Migration operation ID.
    pub migration_id: MigrationId,

    /// Source target.
    pub source_target: TargetId,

    /// Destination target.
    pub destination_target: TargetId,

    /// Source execution.
    pub source_execution: ExecutionId,

    /// Destination execution.
    pub destination_execution: Option<ExecutionId>,

    /// Program identity.
    pub program: ProgramIdentity,

    /// Checkpoint used.
    pub checkpoint: Option<CheckpointId>,

    /// Migration state representation.
    pub state_kind: MigratableStateKind,

    /// Arbitrary provenance metadata.
    pub metadata: BTreeMap<String, String>,
}

impl MigrationProvenance {
    /// Creates provenance for a prepared migration.
    #[must_use]
    pub fn from_prepared(
        migration: &PreparedMigration,
        source_execution: ExecutionId,
        metadata: BTreeMap<String, String>,
    ) -> Self {
        Self {
            implementation: MIGRATION_IMPLEMENTATION_ID.to_owned(),
            migration_id: migration.migration_id.clone(),
            source_target: migration.source_target.clone(),
            destination_target: migration.destination_target.clone(),
            source_execution,
            destination_execution: None,
            program: migration.program.clone(),
            checkpoint: migration.checkpoint.clone(),
            state_kind: migration.state_kind,
            metadata,
        }
    }
}

/// Stable implementation identity.
pub const MIGRATION_IMPLEMENTATION_ID: &str =
    "zamani.quantum.resilience.recovery.migration";

/// Current contract version.
pub const MIGRATION_CONTRACT_VERSION: u16 = 1;

// =============================================================================
// External contracts
// =============================================================================

/// Resolves destination capabilities.
///
/// The hardware subsystem should implement this contract using its canonical
/// capability model. Migration does not discover hardware itself.
pub trait MigrationCapabilityProvider: Send + Sync {
    /// Error generated while resolving capabilities.
    type Error: Error + Send + Sync + 'static;

    /// Returns capabilities for a target.
    fn capabilities(
        &self,
        target: &TargetId,
    ) -> Result<MigrationCapabilities, Self::Error>;
}

/// Validates the source checkpoint/state.
///
/// The checkpoint subsystem should implement this contract.
pub trait MigrationCheckpointProvider: Send + Sync {
    /// Error generated by checkpoint validation.
    type Error: Error + Send + Sync + 'static;

    /// Validates that a checkpoint can be used for this migration.
    fn validate(
        &self,
        request: &MigrationRequest,
    ) -> Result<CheckpointValidation, Self::Error>;
}

/// Result of checkpoint validation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CheckpointValidation {
    /// Checkpoint is intact.
    pub integrity_valid: bool,

    /// Checkpoint belongs to the requested program.
    pub program_compatible: bool,

    /// Checkpoint can be reconstructed on the destination.
    pub destination_compatible: bool,

    /// Quantum state representation is valid for migration.
    pub state_compatible: bool,

    /// Diagnostics.
    pub diagnostics: Vec<String>,
}

impl CheckpointValidation {
    /// Returns whether the checkpoint is usable.
    #[must_use]
    pub fn usable(&self) -> bool {
        self.integrity_valid
            && self.program_compatible
            && self.destination_compatible
            && self.state_compatible
    }
}

/// Executes migration using a backend/runtime-specific implementation.
///
/// This is the only contract that is allowed to perform provider-specific
/// execution work.
pub trait MigrationExecutor: Send + Sync {
    /// Runtime execution result.
    type Execution: Clone + Send + Sync + 'static;

    /// Backend/runtime error.
    type Error: Error + Send + Sync + 'static;

    /// Executes the prepared migration.
    fn migrate(
        &self,
        request: &MigrationExecutionRequest,
    ) -> Result<Self::Execution, Self::Error>;
}

/// Converts a runtime-specific execution result into the normalized migration
/// result.
pub trait MigrationExecutionAdapter<E>: Send + Sync {
    /// Runtime adapter error.
    type Error: Error + Send + Sync + 'static;

    /// Normalizes the destination execution.
    fn normalize(
        &self,
        request: &PreparedMigration,
        execution: &E,
    ) -> Result<MigratedExecution, Self::Error>;
}

/// Performs semantic verification.
///
/// This contract should normally be implemented by the resilience verification
/// subsystem and should ultimately reason against canonical quantum IR.
pub trait MigrationVerifier<E>: Send + Sync {
    /// Verification error.
    type Error: Error + Send + Sync + 'static;

    /// Verifies the migrated execution.
    fn verify(
        &self,
        request: &MigrationRequest,
        prepared: &PreparedMigration,
        execution: &E,
    ) -> Result<MigrationVerification, Self::Error>;
}

/// Cancellation contract.
///
/// Implementations may connect to runtime cancellation, user cancellation,
/// scheduler cancellation or a distributed coordinator.
pub trait MigrationCancellation: Send + Sync {
    /// Returns whether migration has been cancelled.
    fn is_cancelled(&self) -> bool;
}

/// Optional authorization contract.
///
/// Migration can cross trust boundaries, so an enclosing security layer may
/// require explicit authorization.
pub trait MigrationAuthorizer: Send + Sync {
    /// Authorization error.
    type Error: Error + Send + Sync + 'static;

    /// Authorizes the requested migration.
    fn authorize(
        &self,
        request: &MigrationRequest,
    ) -> Result<(), Self::Error>;
}

/// Optional provenance/audit observer.
pub trait MigrationObserver: Send + Sync {
    /// Called when migration begins.
    fn started(
        &self,
        request: &MigrationRequest,
    );

    /// Called after migration reaches a terminal state.
    fn completed(
        &self,
        outcome: &MigrationOutcome,
    );
}

/// Clock abstraction.
///
/// This avoids coupling deterministic decision logic to wall-clock time.
pub trait MigrationClock: Send + Sync {
    /// Returns current time for observability only.
    fn now(&self) -> SystemTime;
}

/// System clock implementation.
#[derive(Clone, Copy, Debug, Default)]
pub struct SystemMigrationClock;

impl MigrationClock for SystemMigrationClock {
    fn now(&self) -> SystemTime {
        SystemTime::now()
    }
}

// =============================================================================
// Migration service
// =============================================================================

/// Production migration service.
///
/// Dependencies are injected so the resilience layer remains backend-neutral.
pub struct MigrationService<C, K, E, A, V>
where
    C: MigrationCapabilityProvider,
    K: MigrationCheckpointProvider,
    E: MigrationExecutor,
    A: MigrationExecutionAdapter<E::Execution>,
    V: MigrationVerifier<E::Execution>,
{
    capabilities: Arc<C>,
    checkpoints: Arc<K>,
    executor: Arc<E>,
    adapter: Arc<A>,
    verifier: Arc<V>,
    authorizer: Option<Arc<dyn MigrationAuthorizer<Error = MigrationAuthorizationError>>>,
    cancellation: Option<Arc<dyn MigrationCancellation>>,
    observer: Option<Arc<dyn MigrationObserver>>,
    clock: Arc<dyn MigrationClock>,
}

impl<C, K, E, A, V> MigrationService<C, K, E, A, V>
where
    C: MigrationCapabilityProvider,
    K: MigrationCheckpointProvider,
    E: MigrationExecutor,
    A: MigrationExecutionAdapter<E::Execution>,
    V: MigrationVerifier<E::Execution>,
{
    /// Creates a migration service with required dependencies.
    #[must_use]
    pub fn new(
        capabilities: Arc<C>,
        checkpoints: Arc<K>,
        executor: Arc<E>,
        adapter: Arc<A>,
        verifier: Arc<V>,
    ) -> Self {
        Self {
            capabilities,
            checkpoints,
            executor,
            adapter,
            verifier,
            authorizer: None,
            cancellation: None,
            observer: None,
            clock: Arc::new(SystemMigrationClock),
        }
    }

    /// Adds an authorization provider.
    #[must_use]
    pub fn with_authorizer(
        mut self,
        authorizer: Arc<
            dyn MigrationAuthorizer<Error = MigrationAuthorizationError>,
        >,
    ) -> Self {
        self.authorizer = Some(authorizer);
        self
    }

    /// Adds a cancellation provider.
    #[must_use]
    pub fn with_cancellation(
        mut self,
        cancellation: Arc<dyn MigrationCancellation>,
    ) -> Self {
        self.cancellation = Some(cancellation);
        self
    }

    /// Adds an observer.
    #[must_use]
    pub fn with_observer(
        mut self,
        observer: Arc<dyn MigrationObserver>,
    ) -> Self {
        self.observer = Some(observer);
        self
    }

    /// Replaces the clock.
    #[must_use]
    pub fn with_clock(
        mut self,
        clock: Arc<dyn MigrationClock>,
    ) -> Self {
        self.clock = clock;
        self
    }

    /// Performs migration.
    ///
    /// No internal fixed retry loop exists. Retry/replan/escalation belongs to
    /// `recovery/recoverer.rs` and `planning/*`.
    pub fn migrate(
        &self,
        request: MigrationRequest,
    ) -> MigrationOutcome {
        let started = self.clock.now();

        if let Some(observer) = &self.observer {
            observer.started(&request);
        }

        let outcome = self.migrate_inner(&request, started);

        if let Some(observer) = &self.observer {
            observer.completed(&outcome);
        }

        outcome
    }

    fn migrate_inner(
        &self,
        request: &MigrationRequest,
        started: SystemTime,
    ) -> MigrationOutcome {
        if let Err(error) = request.validate() {
            return self.failure_outcome(
                request,
                MigrationStatus::Rejected,
                None,
                None,
                error.to_string(),
                started,
            );
        }

        if self.is_cancelled() {
            return self.failure_outcome(
                request,
                MigrationStatus::Cancelled,
                None,
                None,
                "migration cancelled before preparation".to_owned(),
                started,
            );
        }

        if let Some(authorizer) = &self.authorizer {
            if let Err(error) = authorizer.authorize(request) {
                return self.failure_outcome(
                    request,
                    MigrationStatus::Rejected,
                    None,
                    None,
                    error.to_string(),
                    started,
                );
            }
        }

        let checkpoint_validation = match self.checkpoints.validate(request) {
            Ok(value) => value,
            Err(error) => {
                return self.failure_outcome(
                    request,
                    MigrationStatus::Rejected,
                    None,
                    None,
                    format!("checkpoint validation failed: {error}"),
                    started,
                );
            }
        };

        if !checkpoint_validation.usable() {
            return self.failure_outcome(
                request,
                MigrationStatus::Rejected,
                None,
                None,
                format!(
                    "checkpoint is not usable: {}",
                    checkpoint_validation.diagnostics.join("; ")
                ),
                started,
            );
        }

        let capabilities = match self
            .capabilities
            .capabilities(&request.destination_target)
        {
            Ok(value) => value,
            Err(error) => {
                return self.failure_outcome(
                    request,
                    MigrationStatus::Rejected,
                    None,
                    None,
                    format!("destination capability resolution failed: {error}"),
                    started,
                );
            }
        };

        let prepared =
            match PreparedMigration::prepare(request, capabilities) {
                Ok(value) => value,
                Err(error) => {
                    return self.failure_outcome(
                        request,
                        MigrationStatus::Rejected,
                        None,
                        None,
                        error.to_string(),
                        started,
                    );
                }
            };

        if self.is_cancelled() {
            return self.failure_outcome(
                request,
                MigrationStatus::Cancelled,
                Some(prepared),
                None,
                "migration cancelled after preparation".to_owned(),
                started,
            );
        }

        let execution_request = MigrationExecutionRequest {
            migration: prepared.clone(),
            metadata: request.metadata.clone(),
        };

        let migrated_execution =
            match self.executor.migrate(&execution_request) {
                Ok(value) => value,
                Err(error) => {
                    return self.failure_outcome(
                        request,
                        MigrationStatus::Failed,
                        Some(prepared),
                        None,
                        format!("migration execution failed: {error}"),
                        started,
                    );
                }
            };

        if self.is_cancelled() {
            // A destination execution may already exist. We therefore do not
            // pretend cancellation rewound the external side effect.
            return self.failure_outcome(
                request,
                MigrationStatus::Escalated,
                Some(prepared),
                None,
                "migration was cancelled after destination execution began; external cleanup/verification is required"
                    .to_owned(),
                started,
            );
        }

        let normalized =
            match self.adapter.normalize(&prepared, &migrated_execution) {
                Ok(value) => value,
                Err(error) => {
                    return self.failure_outcome(
                        request,
                        MigrationStatus::Failed,
                        Some(prepared),
                        None,
                        format!(
                            "destination execution normalization failed: {error}"
                        ),
                        started,
                    );
                }
            };

        let verification = match self.verifier.verify(
            request,
            &prepared,
            &migrated_execution,
        ) {
            Ok(value) => value,
            Err(error) => {
                return self.failure_outcome(
                    request,
                    MigrationStatus::Escalated,
                    Some(prepared),
                    Some(normalized),
                    format!("migration verification failed: {error}"),
                    started,
                );
            }
        };

        if let Err(error) = verification.validate() {
            return self.failure_outcome(
                request,
                MigrationStatus::Rejected,
                Some(prepared),
                Some(normalized),
                error.to_string(),
                started,
            );
        }

        if !verification.accepted() {
            if verification.degraded
                && request.policy.allow_degraded_execution
            {
                return self.success_outcome(
                    request,
                    MigrationStatus::Degraded,
                    &prepared,
                    &normalized,
                    verification,
                    started,
                );
            }

            return self.failure_outcome(
                request,
                MigrationStatus::Escalated,
                Some(prepared),
                Some(normalized),
                format!(
                    "migrated execution failed acceptance verification: {}",
                    verification.diagnostics.join("; ")
                ),
                started,
            );
        }

        self.success_outcome(
            request,
            MigrationStatus::Accepted,
            &prepared,
            &normalized,
            verification,
            started,
        )
    }

    fn is_cancelled(&self) -> bool {
        self.cancellation
            .as_ref()
            .is_some_and(|value| value.is_cancelled())
    }

    fn success_outcome(
        &self,
        request: &MigrationRequest,
        status: MigrationStatus,
        prepared: &PreparedMigration,
        execution: &MigratedExecution,
        verification: MigrationVerification,
        started: SystemTime,
    ) -> MigrationOutcome {
        let mut provenance = MigrationProvenance::from_prepared(
            prepared,
            request.execution_id.clone(),
            request.metadata.clone(),
        );

        provenance.destination_execution =
            Some(execution.execution_id.clone());

        MigrationOutcome {
            migration_id: request.migration_id.clone(),
            status,
            source_execution: request.execution_id.clone(),
            destination_execution: Some(execution.execution_id.clone()),
            source_target: request.source_target.clone(),
            destination_target: request.destination_target.clone(),
            checkpoint: request.checkpoint.clone(),
            verification: Some(verification),
            provenance,
            elapsed: elapsed_since(started, self.clock.now()),
            diagnostics: Vec::new(),
        }
    }

    fn failure_outcome(
        &self,
        request: &MigrationRequest,
        status: MigrationStatus,
        prepared: Option<PreparedMigration>,
        execution: Option<MigratedExecution>,
        diagnostic: String,
        started: SystemTime,
    ) -> MigrationOutcome {
        let provenance = match prepared {
            Some(ref value) => MigrationProvenance::from_prepared(
                value,
                request.execution_id.clone(),
                request.metadata.clone(),
            ),
            None => MigrationProvenance {
                implementation: MIGRATION_IMPLEMENTATION_ID.to_owned(),
                migration_id: request.migration_id.clone(),
                source_target: request.source_target.clone(),
                destination_target: request.destination_target.clone(),
                source_execution: request.execution_id.clone(),
                destination_execution: execution
                    .as_ref()
                    .map(|value| value.execution_id.clone()),
                program: request.program.clone(),
                checkpoint: request.checkpoint.clone(),
                state_kind: request.state_kind,
                metadata: request.metadata.clone(),
            },
        };

        MigrationOutcome {
            migration_id: request.migration_id.clone(),
            status,
            source_execution: request.execution_id.clone(),
            destination_execution: execution
                .as_ref()
                .map(|value| value.execution_id.clone()),
            source_target: request.source_target.clone(),
            destination_target: request.destination_target.clone(),
            checkpoint: request.checkpoint.clone(),
            verification: None,
            provenance,
            elapsed: elapsed_since(started, self.clock.now()),
            diagnostics: vec![diagnostic],
        }
    }
}

// =============================================================================
// Errors
// =============================================================================

/// Migration error taxonomy.
///
/// The error remains provider-independent. Provider errors are wrapped by the
/// service rather than exposed as part of the migration model.
#[derive(Debug)]
pub enum MigrationError {
    /// Invalid identifier.
    InvalidIdentifier {
        /// Identifier field.
        field: &'static str,
    },

    /// Invalid policy.
    InvalidPolicy(&'static str),

    /// Source and destination are equal.
    SameSourceAndDestination,

    /// Migration disabled.
    MigrationNotAllowed,

    /// Cross-target migration disabled.
    CrossTargetMigrationNotAllowed,

    /// Cross-provider migration disabled.
    CrossProviderMigrationNotAllowed,

    /// Arbitrary quantum state transfer attempted.
    ArbitraryQuantumStateTransfer,

    /// State cannot be migrated.
    StateNotMigratable {
        /// State kind.
        state: MigratableStateKind,
    },

    /// Replay safety is unknown.
    ReplaySafetyUnknown,

    /// Replay is unsafe.
    ReplayUnsafe,

    /// Conditional replay requires explicit policy.
    ConditionalReplayNotAllowed,

    /// Non-replayable side effects.
    NonReplayableSideEffects,

    /// Side-effect semantics unknown.
    SideEffectSemanticsUnknown,

    /// A checkpoint is required.
    CheckpointRequired,

    /// Required capability unavailable.
    CapabilityUnavailable {
        /// Capability name.
        capability: &'static str,
    },

    /// Recompilation forbidden.
    RecompilationNotAllowed,

    /// Verification confidence invalid.
    InvalidConfidence,

    /// External authorization failed.
    AuthorizationFailed(String),

    /// External migration failed.
    ExecutionFailed(String),

    /// Verification failed.
    VerificationFailed(String),
}

impl fmt::Display for MigrationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidIdentifier { field } => {
                write!(formatter, "invalid empty {field}")
            }
            Self::InvalidPolicy(reason) => {
                write!(formatter, "invalid migration policy: {reason}")
            }
            Self::SameSourceAndDestination => {
                formatter.write_str(
                    "source and destination targets must differ",
                )
            }
            Self::MigrationNotAllowed => {
                formatter.write_str("migration is not allowed by policy")
            }
            Self::CrossTargetMigrationNotAllowed => {
                formatter.write_str(
                    "cross-target migration is not allowed by policy",
                )
            }
            Self::CrossProviderMigrationNotAllowed => {
                formatter.write_str(
                    "cross-provider migration is not allowed by policy",
                )
            }
            Self::ArbitraryQuantumStateTransfer => {
                formatter.write_str(
                    "arbitrary unknown quantum-state transfer is forbidden",
                )
            }
            Self::StateNotMigratable { state } => {
                write!(formatter, "state kind {state:?} is not migratable")
            }
            Self::ReplaySafetyUnknown => {
                formatter.write_str("replay safety is unknown")
            }
            Self::ReplayUnsafe => {
                formatter.write_str("replay is not semantically safe")
            }
            Self::ConditionalReplayNotAllowed => {
                formatter.write_str(
                    "conditional replay requires explicit policy permission",
                )
            }
            Self::NonReplayableSideEffects => {
                formatter.write_str(
                    "execution contains non-replayable external side effects",
                )
            }
            Self::SideEffectSemanticsUnknown => {
                formatter.write_str(
                    "external side-effect semantics are unknown",
                )
            }
            Self::CheckpointRequired => {
                formatter.write_str(
                    "a valid checkpoint is required for this migration mode",
                )
            }
            Self::CapabilityUnavailable { capability } => {
                write!(
                    formatter,
                    "destination capability unavailable: {capability}"
                )
            }
            Self::RecompilationNotAllowed => {
                formatter.write_str(
                    "destination requires recompilation but policy forbids it",
                )
            }
            Self::InvalidConfidence => {
                formatter.write_str(
                    "verification confidence must be finite and between zero and one",
                )
            }
            Self::AuthorizationFailed(reason) => {
                write!(formatter, "migration authorization failed: {reason}")
            }
            Self::ExecutionFailed(reason) => {
                write!(formatter, "migration execution failed: {reason}")
            }
            Self::VerificationFailed(reason) => {
                write!(formatter, "migration verification failed: {reason}")
            }
        }
    }
}

impl Error for MigrationError {}

/// Canonical authorization error adapter.
///
/// Concrete security systems can map their own error into this stable contract.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MigrationAuthorizationError {
    /// Explanation.
    pub message: String,
}

impl fmt::Display for MigrationAuthorizationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl Error for MigrationAuthorizationError {}

// =============================================================================
// Utility functions
// =============================================================================

fn elapsed_since(
    start: SystemTime,
    end: SystemTime,
) -> Option<Duration> {
    end.duration_since(start).ok()
}

/// Returns Unix epoch milliseconds for observability.
///
/// This is deliberately not used for migration decisions.
#[must_use]
pub fn unix_timestamp_millis(time: SystemTime) -> Option<u128> {
    time.duration_since(UNIX_EPOCH)
        .ok()
        .map(|duration| duration.as_millis())
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn request() -> MigrationRequest {
        MigrationRequest {
            migration_id: MigrationId::new("migration-1").unwrap(),
            execution_id: ExecutionId::new("execution-1").unwrap(),
            program: ProgramIdentity::new("program-hash").unwrap(),
            source_target: TargetId::new("source").unwrap(),
            destination_target: TargetId::new("destination").unwrap(),
            checkpoint: Some(CheckpointId::new("checkpoint-1").unwrap()),
            state_kind: MigratableStateKind::LogicalCheckpoint,
            quantum_state_transfer: QuantumStateTransfer::LogicalState,
            replay_safety: ReplaySafety::Safe,
            side_effects: SideEffectMode::None,
            policy: MigrationPolicy::default(),
            metadata: BTreeMap::new(),
        }
    }

    #[test]
    fn valid_request_passes_static_validation() {
        assert!(request().validate().is_ok());
    }

    #[test]
    fn same_target_is_rejected() {
        let mut value = request();

        value.destination_target = value.source_target.clone();

        assert!(matches!(
            value.validate(),
            Err(MigrationError::SameSourceAndDestination)
        ));
    }

    #[test]
    fn arbitrary_quantum_state_transfer_is_rejected() {
        let mut value = request();

        value.quantum_state_transfer =
            QuantumStateTransfer::ArbitraryUnknownState;

        assert!(matches!(
            value.validate(),
            Err(MigrationError::ArbitraryQuantumStateTransfer)
        ));
    }

    #[test]
    fn unsafe_replay_is_rejected() {
        let mut value = request();

        value.replay_safety = ReplaySafety::Unsafe;

        assert!(matches!(
            value.validate(),
            Err(MigrationError::ReplayUnsafe)
        ));
    }

    #[test]
    fn unknown_replay_safety_is_rejected_for_reconstruction() {
        let mut value = request();

        value.state_kind =
            MigratableStateKind::ReconstructibleProgram;
        value.quantum_state_transfer =
            QuantumStateTransfer::NotRequired;
        value.checkpoint = None;
        value.replay_safety = ReplaySafety::Unknown;

        assert!(matches!(
            value.validate(),
            Err(MigrationError::ReplaySafetyUnknown)
        ));
    }

    #[test]
    fn non_replayable_side_effects_are_rejected() {
        let mut value = request();

        value.state_kind =
            MigratableStateKind::ReconstructibleProgram;
        value.quantum_state_transfer =
            QuantumStateTransfer::NotRequired;
        value.checkpoint = None;
        value.side_effects = SideEffectMode::NonReplayable;

        assert!(matches!(
            value.validate(),
            Err(MigrationError::NonReplayableSideEffects)
        ));
    }

    #[test]
    fn logical_state_requires_checkpoint() {
        let mut value = request();

        value.checkpoint = None;

        assert!(matches!(
            value.validate(),
            Err(MigrationError::CheckpointRequired)
        ));
    }

    #[test]
    fn verification_acceptance_requires_all_invariants() {
        let verification = MigrationVerification {
            semantically_equivalent: true,
            capability_compatible: true,
            state_valid: true,
            provenance_complete: true,
            degraded: false,
            confidence: Some(1.0),
            diagnostics: Vec::new(),
        };

        assert!(verification.accepted());
    }

    #[test]
    fn verification_rejects_semantic_failure() {
        let verification = MigrationVerification {
            semantically_equivalent: false,
            capability_compatible: true,
            state_valid: true,
            provenance_complete: true,
            degraded: false,
            confidence: Some(1.0),
            diagnostics: vec![
                "semantic equivalence failed".to_owned()
            ],
        };

        assert!(!verification.accepted());
    }

    #[test]
    fn invalid_confidence_is_rejected() {
        let verification = MigrationVerification {
            semantically_equivalent: true,
            capability_compatible: true,
            state_valid: true,
            provenance_complete: true,
            degraded: false,
            confidence: Some(f64::NAN),
            diagnostics: Vec::new(),
        };

        assert!(matches!(
            verification.validate(),
            Err(MigrationError::InvalidConfidence)
        ));
    }

    #[test]
    fn lifecycle_transitions_are_explicit() {
        assert!(
            MigrationTransition {
                from: MigrationStatus::Requested,
                to: MigrationStatus::Prepared,
            }
            .is_valid()
        );

        assert!(
            MigrationTransition {
                from: MigrationStatus::Prepared,
                to: MigrationStatus::Migrating,
            }
            .is_valid()
        );

        assert!(
            MigrationTransition {
                from: MigrationStatus::Migrating,
                to: MigrationStatus::Verifying,
            }
            .is_valid()
        );

        assert!(
            MigrationTransition {
                from: MigrationStatus::Verifying,
                to: MigrationStatus::Accepted,
            }
            .is_valid()
        );

        assert!(
            !MigrationTransition {
                from: MigrationStatus::Accepted,
                to: MigrationStatus::Migrating,
            }
            .is_valid()
        );
    }

    #[test]
    fn policy_rejects_inconsistent_cross_provider_configuration() {
        let mut policy = MigrationPolicy::default();

        policy.allow_cross_target = false;
        policy.allow_cross_provider = true;

        assert!(matches!(
            policy.validate(),
            Err(MigrationError::InvalidPolicy(_))
        ));
    }

    #[test]
    fn capabilities_require_execution_and_semantic_compatibility() {
        let capabilities = MigrationCapabilities {
            executable: true,
            canonical_program: true,
            checkpoint_restore: true,
            provider_state_migration: true,
            logical_state_migration: true,
            rerouting: true,
            rescheduling: true,
            recompilation: true,
            semantic_compatibility: true,
            degraded_execution: true,
        };

        assert!(capabilities.validate().is_ok());
    }

    #[test]
    fn timestamp_helper_is_observability_only() {
        assert!(
            unix_timestamp_millis(UNIX_EPOCH).is_some()
        );
    }
}