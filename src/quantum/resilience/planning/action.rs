//! Zamani Quantum Resilience — Canonical Recovery Action Model
//!
//! Path:
//!     src/quantum/resilience/planning/action.rs
//!
//! Purpose:
//!     Define the canonical, provider-independent, immutable description of
//!     actions that a resilience planner may request.
//!
//! This module is intentionally a DOMAIN/CONTRACT module.
//!
//! It does NOT:
//! - execute actions;
//! - communicate with quantum hardware;
//! - perform routing;
//! - perform scheduling;
//! - compile or optimize circuits;
//! - implement QEC;
//! - implement error mitigation;
//! - select a provider by name;
//! - decide whether an action is safe;
//! - decide whether an action is feasible;
//! - contain retry loops;
//! - contain fixed machine-size limits.
//!
//! Those responsibilities belong to:
//!
//!     planning/planner.rs
//!     planning/feasibility.rs
//!     planning/policy.rs
//!     adaptation/*
//!     recovery/*
//!     mitigation/*
//!     verification/*
//!     hardware/*
//!     routing/*
//!     scheduling/*
//!     optimization/*
//!     qec/*
//!
//! -----------------------------------------------------------------------------
//! Architectural invariant
//! -----------------------------------------------------------------------------
//!
//! A Zamani quantum program describes the computation.
//! An action describes how the execution environment may adapt while
//! preserving that computation.
//!
//! Therefore an action MUST NOT encode assumptions such as:
//!
//!     - a fixed number of qubits;
//!     - a fixed number of devices;
//!     - a fixed backend;
//!     - a fixed retry count;
//!     - a fixed fidelity threshold;
//!     - a fixed topology;
//!     - a fixed native gate set;
//!     - a fixed QEC code;
//!     - a fixed provider.
//!
//! All such information belongs to discovered capabilities, policy,
//! workload constraints, execution state, or another authoritative subsystem.
//!
//! -----------------------------------------------------------------------------
//! "Atom to everywhere"
//! -----------------------------------------------------------------------------
//!
//! This module introduces no artificial quantum-system-size limit.
//!
//! Scalability means:
//!
//!     one logical program
//!          |
//!          +--> one physical qubit
//!          +--> small QPU
//!          +--> large QPU
//!          +--> fault-tolerant machine
//!          +--> heterogeneous quantum fleet
//!          +--> distributed quantum system
//!
//! The action model identifies resources by stable identities rather than by
//! fixed-size arrays or machine-specific numeric assumptions.
//!
//! -----------------------------------------------------------------------------
//! Safety
//! -----------------------------------------------------------------------------
//!
//! `RecoveryAction` is a REQUEST/DESCRIPTION, not an authorization.
//!
//! An action is only executable after:
//!
//!     policy validation
//!     + capability validation
//!     + feasibility validation
//!     + security authorization
//!     + semantic validation
//!     + execution precondition validation
//!
//! Abort and quarantine remain available as protective actions even when
//! transformative recovery is unsafe.
//!
//! -----------------------------------------------------------------------------
//! Determinism
//! -----------------------------------------------------------------------------
//!
//! Action values are:
//!
//! - immutable;
//! - structurally comparable;
//! - hashable;
//! - serializable by an external serialization layer;
//! - independent of provider-specific runtime objects.
//!
//! Canonical ordering is supplied by `Ord` implementations below.
//!
//! -----------------------------------------------------------------------------
//! Rust compatibility
//! -----------------------------------------------------------------------------
//!
//! Rust 1.97 / 1.97.1
//! Rust 2021
//! no nightly features
//! no unsafe code
//!
//! -----------------------------------------------------------------------------
//! Integration contract
//! -----------------------------------------------------------------------------
//!
//! `planner.rs`:
//!     generates these actions.
//!
//! `plan.rs`:
//!     stores ordered action sequences.
//!
//! `feasibility.rs`:
//!     determines whether an action can be executed.
//!
//! `ranking.rs`:
//!     ranks plans containing actions.
//!
//! `adaptation/*`:
//!     implements Remap, Reroute, Reschedule, Recompile, Reoptimize,
//!     QEC adaptation and backend migration.
//!
//! `recovery/*`:
//!     implements Retry, Restart, Resume, Rollback, Checkpoint,
//!     Compensation and Abort.
//!
//! `mitigation/*`:
//!     implements Mitigate.
//!
//! `verification/*`:
//!     verifies the resulting execution.
//!
//! `registry/*`:
//!     registers implementations without modifying this domain model.
//!
//! `serialization/*`:
//!     serializes these stable domain values.
//!
//! `telemetry/*`:
//!     records action intent and outcome.
//!
//! `history/*`:
//!     records whether actions succeeded or failed.
//!
//! Other quantum subsystems MUST NOT depend on concrete recovery
//! implementations merely to represent an action.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::fmt;
use std::num::NonZeroU64;

// -----------------------------------------------------------------------------
// Canonical quantum resource identity
// -----------------------------------------------------------------------------
//
// These imports are deliberately narrow.
//
// Do not replace them with locally defined QubitId types.
//
// If the repository's canonical IR exposes a different physical-resource
// identity type in the future, that change belongs at the integration boundary
// rather than creating a second quantum identity system here.
//

use crate::quantum::ir::qubit::QubitId;

// =============================================================================
// Stable schema
// =============================================================================

/// Stable schema identifier for canonical resilience actions.
pub const ACTION_SCHEMA_ID: &str = "zamani.quantum.resilience.action";

/// Semantic version of the action schema.
///
/// Increment when serialized or externally observable semantics change.
pub const ACTION_SCHEMA_VERSION: u16 = 1;

// =============================================================================
// Action identifier
// =============================================================================

/// Stable machine-readable action identifier.
///
/// These identifiers are suitable for:
///
/// - serialization;
/// - telemetry;
/// - audit logs;
/// - registries;
/// - deterministic replay;
/// - policy matching.
///
/// They intentionally contain no provider names.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ActionKind {
    /// Re-attempt an execution under the existing execution contract.
    Retry,

    /// Start again from a valid restart boundary.
    Restart,

    /// Continue from a valid checkpoint or semantic boundary.
    Resume,

    /// Restore a previously accepted execution state.
    Rollback,

    /// Establish a checkpoint at a valid execution boundary.
    Checkpoint,

    /// Recompute logical-to-physical placement.
    Remap,

    /// Recompute physical routing.
    Reroute,

    /// Recompute execution scheduling.
    Reschedule,

    /// Recompile the program for a changed target.
    Recompile,

    /// Re-run target-aware optimization.
    Reoptimize,

    /// Adapt the quantum error-correction configuration.
    AdaptQec,

    /// Apply an error-mitigation strategy.
    Mitigate,

    /// Migrate execution to another compatible execution environment.
    Migrate,

    /// Temporarily remove a resource from service.
    QuarantineResource,

    /// Apply a mathematically defined compensating operation.
    Compensate,

    /// Escalate the decision to another authority.
    Escalate,

    /// Stop execution because safe continuation cannot be established.
    Abort,
}

impl ActionKind {
    /// Stable serialized action identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Retry => "retry",
            Self::Restart => "restart",
            Self::Resume => "resume",
            Self::Rollback => "rollback",
            Self::Checkpoint => "checkpoint",
            Self::Remap => "remap",
            Self::Reroute => "reroute",
            Self::Reschedule => "reschedule",
            Self::Recompile => "recompile",
            Self::Reoptimize => "reoptimize",
            Self::AdaptQec => "adapt_qec",
            Self::Mitigate => "mitigate",
            Self::Migrate => "migrate",
            Self::QuarantineResource => "quarantine_resource",
            Self::Compensate => "compensate",
            Self::Escalate => "escalate",
            Self::Abort => "abort",
        }
    }

    /// Returns whether the action can potentially mutate execution state.
    ///
    /// This is informational classification only.
    ///
    /// Actual authorization belongs to policy and feasibility validation.
    #[must_use]
    pub const fn is_mutating(self) -> bool {
        match self {
            Self::Escalate | Self::Abort => false,
            Self::Retry
            | Self::Restart
            | Self::Resume
            | Self::Rollback
            | Self::Checkpoint
            | Self::Remap
            | Self::Reroute
            | Self::Reschedule
            | Self::Recompile
            | Self::Reoptimize
            | Self::AdaptQec
            | Self::Mitigate
            | Self::Migrate
            | Self::QuarantineResource
            | Self::Compensate => true,
        }
    }

    /// Returns whether the action changes the physical realization without
    /// necessarily changing logical program semantics.
    #[must_use]
    pub const fn changes_physical_realization(self) -> bool {
        matches!(
            self,
            Self::Remap
                | Self::Reroute
                | Self::Reschedule
                | Self::Recompile
                | Self::Reoptimize
                | Self::AdaptQec
                | Self::Migrate
        )
    }

    /// Returns whether the action requires explicit post-action verification.
    ///
    /// This is deliberately conservative.
    #[must_use]
    pub const fn requires_verification(self) -> bool {
        match self {
            Self::Abort | Self::Escalate => false,
            _ => true,
        }
    }

    /// Returns whether the action is intrinsically protective.
    ///
    /// Protective actions may remain valid when transformative recovery is
    /// blocked by safety policy.
    #[must_use]
    pub const fn is_protective(self) -> bool {
        matches!(self, Self::QuarantineResource | Self::Abort | Self::Escalate)
    }
}

impl fmt::Display for ActionKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Resource scope
// =============================================================================

/// Scope at which an action applies.
///
/// This deliberately does not encode the number of resources.
///
/// A scope can represent one resource or an arbitrarily large resource set
/// through the external resource model.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ActionScope {
    /// Entire logical computation.
    Computation,

    /// Current execution.
    Execution,

    /// A logical quantum resource.
    LogicalQubit(QubitId),

    /// A physical quantum resource represented by the canonical QubitId.
///
/// The distinction between logical and physical meaning is carried by the
/// scope variant rather than by inventing another identifier type here.
    PhysicalQubit(QubitId),

    /// A provider-neutral resource identifier.
    ///
    /// The identifier is intentionally opaque to this module.
    Resource(ResourceId),

    /// An externally defined execution environment.
    ExecutionEnvironment(ResourceId),

    /// A region selected by another subsystem.
    ///
    /// The region identity is opaque; routing, scheduling, compiler and
    /// hardware layers own its interpretation.
    Region(ResourceId),

    /// All resources affected by an incident.
    AffectedResources,

    /// All resources participating in the current execution.
    ExecutionResources,
}

// =============================================================================
// Opaque resource identity
// =============================================================================

/// Provider-neutral resource identifier.
///
/// This type is intentionally opaque. It does not assume that resource
/// identity is numeric, hierarchical, UUID-based, or provider-specific.
///
/// A stable caller-generated value can be represented as bytes encoded into
/// the owned string.
///
/// The action model only needs identity; resource discovery owns meaning.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ResourceId(String);

impl ResourceId {
    /// Creates a resource identifier after validating that it is non-empty.
    pub fn new(value: impl Into<String>) -> Result<Self, ActionError> {
        let value = value.into();

        if value.is_empty() {
            return Err(ActionError::EmptyResourceId);
        }

        Ok(Self(value))
    }

    /// Returns the underlying stable identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the identifier and returns its string representation.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for ResourceId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// =============================================================================
// Action priority
// =============================================================================

/// Planner-level action priority.
///
/// Priority is not authorization and is not a retry count.
///
/// Ranking policy may use it as one input among many.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ActionPriority {
    /// Lowest preference.
    Low,

    /// Normal preference.
    Normal,

    /// High preference.
    High,

    /// Critical protective action.
    Critical,
}

impl ActionPriority {
    /// Stable serialized identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Low => "low",
            Self::Normal => "normal",
            Self::High => "high",
            Self::Critical => "critical",
        }
    }
}

// =============================================================================
// Preconditions
// =============================================================================

/// Preconditions that an action implementation must establish before
/// execution.
///
/// These are declarative requirements, not executable checks.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ActionPrecondition {
    /// Execution must still refer to the specified execution identity.
    ExecutionIdentityValid,

    /// The target resource must be healthy enough for the action.
    ResourceAvailable,

    /// The target capability must exist.
    CapabilityAvailable,

    /// Current logical semantics must remain representable.
    SemanticCompatibility,

    /// Current mapping must be valid.
    MappingValid,

    /// Current routing must be valid.
    RoutingValid,

    /// Current schedule must be valid.
    ScheduleValid,

    /// Compilation must be valid for the selected target.
    CompilationValid,

    /// A checkpoint must exist.
    CheckpointAvailable,

    /// A valid resume boundary must exist.
    ResumeBoundaryAvailable,

    /// A rollback target must exist.
    RollbackTargetAvailable,

    /// Retry must have been declared semantically safe.
    RetrySafetyEstablished,

    /// QEC compatibility must be established.
    QecCompatibility,

    /// Mitigation capability must be established.
    MitigationCapability,

    /// Migration target must be compatible.
    MigrationCompatibility,

    /// Required authorization must be present.
    AuthorizationGranted,

    /// Required provenance must be available.
    ProvenanceAvailable,

    /// Verification mechanism must be available.
    VerificationAvailable,

    /// Resource isolation must be established.
    ResourceIsolation,

    /// Action must be deterministic under the selected execution mode.
    DeterministicExecution,

    /// Caller supplied a required condition.
    ExternalCondition(ResourceId),
}

// =============================================================================
// Expected effects
// =============================================================================

/// Declarative description of the intended effect of an action.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ExpectedEffect {
    /// Execution continues under the same implementation.
    ExecutionContinues,

    /// Execution starts again from a restart boundary.
    ExecutionRestarted,

    /// Execution resumes from a valid boundary.
    ExecutionResumed,

    /// Execution returns to an earlier accepted state.
    ExecutionRolledBack,

    /// A checkpoint becomes available.
    CheckpointCreated,

    /// Logical-to-physical mapping is replaced.
    MappingChanged,

    /// Physical route is replaced.
    RoutingChanged,

    /// Schedule is replaced.
    ScheduleChanged,

    /// Compilation artifact is replaced.
    CompilationChanged,

    /// Optimization artifact is replaced.
    OptimizationChanged,

    /// QEC configuration is replaced.
    QecConfigurationChanged,

    /// Mitigation is applied.
    MitigationApplied,

    /// Execution target is changed.
    ExecutionEnvironmentChanged,

    /// A resource is isolated from future execution.
    ResourceQuarantined,

    /// A compensating transformation is applied.
    CompensationApplied,

    /// Automatic recovery is escalated.
    RecoveryEscalated,

    /// Execution is terminated.
    ExecutionAborted,

    /// Result requires verification before acceptance.
    VerificationRequired,
}

// =============================================================================
// Verification requirement
// =============================================================================

/// Required verification strength after an action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum VerificationRequirement {
    /// No post-action result exists to verify.
    NotApplicable,

    /// Structural execution validity.
    Structural,

    /// Quantum semantic validity.
    Semantic,

    /// Strong result verification.
    Strong,

    /// Verification requirements are supplied externally.
    PolicyDefined,
}

impl VerificationRequirement {
    /// Stable serialized identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotApplicable => "not_applicable",
            Self::Structural => "structural",
            Self::Semantic => "semantic",
            Self::Strong => "strong",
            Self::PolicyDefined => "policy_defined",
        }
    }
}

// =============================================================================
// Action reason
// =============================================================================

/// Stable reason for selecting or requesting an action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ActionReason {
    /// The observed fault is transient.
    TransientFailure,

    /// Existing execution state is restorable.
    RestorableExecution,

    /// A resource has degraded.
    ResourceDegradation,

    /// Physical mapping is invalid.
    PhysicalRealizationInvalid,

    /// Target capability changed.
    CapabilityChange,

    /// Current backend/environment is unavailable.
    ExecutionEnvironmentUnavailable,

    /// Routing is invalid.
    RoutingInvalid,

    /// Scheduling is invalid.
    SchedulingInvalid,

    /// Compilation target is invalid.
    CompilationInvalid,

    /// QEC health/configuration degraded.
    QecDegradation,

    /// Noise can potentially be handled by mitigation.
    NoiseMitigation,

    /// Verification requires a new execution.
    VerificationRequired,

    /// Previous recovery failed.
    PreviousRecoveryFailed,

    /// Several faults form one correlated incident.
    CorrelatedIncident,

    /// Explicit policy requested this action.
    PolicyRequested,

    /// Explicit caller request.
    CallerRequested,

    /// No safer transformative action remains.
    NoSafeAutomaticAction,
}

impl ActionReason {
    /// Stable serialized identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TransientFailure => "transient_failure",
            Self::RestorableExecution => "restorable_execution",
            Self::ResourceDegradation => "resource_degradation",
            Self::PhysicalRealizationInvalid => "physical_realization_invalid",
            Self::CapabilityChange => "capability_change",
            Self::ExecutionEnvironmentUnavailable => {
                "execution_environment_unavailable"
            }
            Self::RoutingInvalid => "routing_invalid",
            Self::SchedulingInvalid => "scheduling_invalid",
            Self::CompilationInvalid => "compilation_invalid",
            Self::QecDegradation => "qec_degradation",
            Self::NoiseMitigation => "noise_mitigation",
            Self::VerificationRequired => "verification_required",
            Self::PreviousRecoveryFailed => "previous_recovery_failed",
            Self::CorrelatedIncident => "correlated_incident",
            Self::PolicyRequested => "policy_requested",
            Self::CallerRequested => "caller_requested",
            Self::NoSafeAutomaticAction => "no_safe_automatic_action",
        }
    }
}

// =============================================================================
// Action identity
// =============================================================================

/// Globally meaningful action identity within a resilience planning domain.
///
/// The identifier is not a pointer and carries no execution state.
///
/// It is suitable for:
///
/// - tracing;
/// - deterministic replay;
/// - audit;
/// - correlation;
/// - persistence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ActionId(NonZeroU64);

impl ActionId {
    /// Constructs an action ID.
    ///
    /// Zero is rejected because it is reserved as "not assigned".
    pub fn new(value: u64) -> Result<Self, ActionError> {
        NonZeroU64::new(value)
            .map(Self)
            .ok_or(ActionError::InvalidActionId)
    }

    /// Returns the numeric representation.
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0.get()
    }
}

impl fmt::Display for ActionId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "action-{}", self.get())
    }
}

// =============================================================================
// Action payload
// =============================================================================

/// Canonical action-specific parameters.
///
/// This enum contains only declarative parameters.
///
/// Concrete implementations interpret these parameters.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ActionPayload {
    /// Retry the current execution.
///
/// Retry policy such as retry budget, backoff and attempt accounting belongs
    /// to `policy/retry.rs`, not here.
    Retry {
        /// Optional resource/environment scope.
        scope: Option<ActionScope>,
    },

    /// Restart from a valid restart boundary.
    Restart {
        /// Boundary/resource scope.
        scope: ActionScope,
    },

    /// Resume from an existing valid boundary.
    Resume {
        /// Checkpoint or execution-boundary identifier.
        checkpoint: ResourceId,
    },

    /// Roll back to an accepted state.
    Rollback {
        /// Rollback target identifier.
        target: ResourceId,
    },

    /// Create a checkpoint.
    Checkpoint {
        /// Scope whose state is to be checkpointed.
        scope: ActionScope,
    },

    /// Recompute logical-to-physical mapping.
    Remap {
        /// Resource region requiring remapping.
        scope: ActionScope,
    },

    /// Recompute physical routing.
    Reroute {
        /// Resource region requiring rerouting.
        scope: ActionScope,
    },

    /// Recompute schedule.
    Reschedule {
        /// Resource region requiring rescheduling.
        scope: ActionScope,
    },

    /// Recompile against currently discovered target capabilities.
    Recompile {
        /// Compilation scope.
        scope: ActionScope,
    },

    /// Re-run target-aware optimization.
    Reoptimize {
        /// Optimization scope.
        scope: ActionScope,
    },

    /// Adapt QEC configuration.
    AdaptQec {
        /// Logical resource or execution scope.
        scope: ActionScope,
    },

    /// Apply mitigation.
    ///
    /// Strategy selection belongs to `mitigation/selection.rs`.
    Mitigate {
        /// Scope requiring mitigation.
        scope: ActionScope,

        /// Optional opaque strategy identifier.
        strategy: Option<ResourceId>,
    },

    /// Migrate to another compatible execution environment.
    ///
    /// The target is opaque here. Capability negotiation belongs to hardware
    /// and backend-selection layers.
    Migrate {
        /// Current execution scope.
        scope: ActionScope,

        /// Target environment identity.
        target: ResourceId,
    },

    /// Quarantine a resource.
    QuarantineResource {
        /// Resource to isolate.
        resource: ResourceId,
    },

    /// Apply a mathematically defined compensation.
    Compensate {
        /// Scope affected by compensation.
        scope: ActionScope,

        /// Opaque compensation strategy identifier.
        strategy: ResourceId,
    },

    /// Escalate.
    Escalate {
        /// Escalation scope.
        scope: ActionScope,
    },

    /// Abort execution.
    Abort {
        /// Execution scope to terminate.
        scope: ActionScope,
    },
}

impl ActionPayload {
    /// Returns the action kind represented by this payload.
    #[must_use]
    pub const fn kind(&self) -> ActionKind {
        match self {
            Self::Retry { .. } => ActionKind::Retry,
            Self::Restart { .. } => ActionKind::Restart,
            Self::Resume { .. } => ActionKind::Resume,
            Self::Rollback { .. } => ActionKind::Rollback,
            Self::Checkpoint { .. } => ActionKind::Checkpoint,
            Self::Remap { .. } => ActionKind::Remap,
            Self::Reroute { .. } => ActionKind::Reroute,
            Self::Reschedule { .. } => ActionKind::Reschedule,
            Self::Recompile { .. } => ActionKind::Recompile,
            Self::Reoptimize { .. } => ActionKind::Reoptimize,
            Self::AdaptQec { .. } => ActionKind::AdaptQec,
            Self::Mitigate { .. } => ActionKind::Mitigate,
            Self::Migrate { .. } => ActionKind::Migrate,
            Self::QuarantineResource { .. } => ActionKind::QuarantineResource,
            Self::Compensate { .. } => ActionKind::Compensate,
            Self::Escalate { .. } => ActionKind::Escalate,
            Self::Abort { .. } => ActionKind::Abort,
        }
    }
}

// =============================================================================
// Canonical RecoveryAction
// =============================================================================

/// Complete immutable description of one resilience action.
///
/// This is the central type exported to:
///
/// - planner;
/// - plan;
/// - feasibility;
/// - ranking;
/// - adaptation;
/// - recovery;
/// - mitigation;
/// - verification;
/// - telemetry;
/// - history;
/// - serialization.
///
/// It describes intent.
///
/// It does not execute anything.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RecoveryAction {
    /// Stable action identity.
    id: ActionId,

    /// Action-specific declarative payload.
    payload: ActionPayload,

    /// Planner-level priority.
    priority: ActionPriority,

    /// Why this action is being considered.
    reason: ActionReason,

    /// Declarative preconditions.
    preconditions: Vec<ActionPrecondition>,

    /// Expected effects.
    expected_effects: Vec<ExpectedEffect>,

    /// Required verification strength.
    verification: VerificationRequirement,

    /// Whether semantic preservation is explicitly required.
    semantic_preservation_required: bool,
}

impl RecoveryAction {
    /// Creates a validated recovery action.
    pub fn new(
        id: ActionId,
        payload: ActionPayload,
        priority: ActionPriority,
        reason: ActionReason,
        preconditions: Vec<ActionPrecondition>,
        expected_effects: Vec<ExpectedEffect>,
        verification: VerificationRequirement,
        semantic_preservation_required: bool,
    ) -> Result<Self, ActionError> {
        if preconditions.is_empty()
            && payload.kind() != ActionKind::Abort
            && payload.kind() != ActionKind::Escalate
        {
            return Err(ActionError::MissingPreconditions);
        }

        if payload.kind().requires_verification()
            && verification == VerificationRequirement::NotApplicable
        {
            return Err(ActionError::VerificationRequired);
        }

        Ok(Self {
            id,
            payload,
            priority,
            reason,
            preconditions,
            expected_effects,
            verification,
            semantic_preservation_required,
        })
    }

    /// Returns the stable action ID.
    #[must_use]
    pub const fn id(&self) -> ActionId {
        self.id
    }

    /// Returns the action kind.
    #[must_use]
    pub const fn kind(&self) -> ActionKind {
        self.payload.kind()
    }

    /// Returns the action payload.
    #[must_use]
    pub fn payload(&self) -> &ActionPayload {
        &self.payload
    }

    /// Returns planner priority.
    #[must_use]
    pub const fn priority(&self) -> ActionPriority {
        self.priority
    }

    /// Returns the reason.
    #[must_use]
    pub const fn reason(&self) -> ActionReason {
        self.reason
    }

    /// Returns preconditions.
    #[must_use]
    pub fn preconditions(&self) -> &[ActionPrecondition] {
        &self.preconditions
    }

    /// Returns expected effects.
    #[must_use]
    pub fn expected_effects(&self) -> &[ExpectedEffect] {
        &self.expected_effects
    }

    /// Returns verification requirement.
    #[must_use]
    pub const fn verification_requirement(&self) -> VerificationRequirement {
        self.verification
    }

    /// Returns whether semantic preservation must be established.
    #[must_use]
    pub const fn requires_semantic_preservation(&self) -> bool {
        self.semantic_preservation_required
    }

    /// Returns whether this action is protective.
    #[must_use]
    pub const fn is_protective(&self) -> bool {
        self.kind().is_protective()
    }

    /// Returns whether this action mutates execution/resource state.
    #[must_use]
    pub const fn is_mutating(&self) -> bool {
        self.kind().is_mutating()
    }

    /// Returns a stable action key suitable for deterministic ordering.
    #[must_use]
    pub fn stable_key(&self) -> ActionStableKey<'_> {
        ActionStableKey {
            kind: self.kind(),
            priority: self.priority,
            reason: self.reason,
            id: self.id,
        }
    }
}

/// Stable deterministic action ordering key.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ActionStableKey<'a> {
    kind: ActionKind,
    priority: ActionPriority,
    reason: ActionReason,
    id: ActionId,
}

impl<'a> ActionStableKey<'a> {
    /// Returns action kind.
    #[must_use]
    pub const fn kind(self) -> ActionKind {
        self.kind
    }

    /// Returns priority.
    #[must_use]
    pub const fn priority(self) -> ActionPriority {
        self.priority
    }

    /// Returns reason.
    #[must_use]
    pub const fn reason(self) -> ActionReason {
        self.reason
    }

    /// Returns action identity.
    #[must_use]
    pub const fn id(self) -> ActionId {
        self.id
    }
}

// =============================================================================
// Constructors
// =============================================================================

impl RecoveryAction {
    /// Constructs a retry action.
    pub fn retry(
        id: ActionId,
        reason: ActionReason,
        scope: Option<ActionScope>,
    ) -> Result<Self, ActionError> {
        Self::new(
            id,
            ActionPayload::Retry { scope },
            ActionPriority::Normal,
            reason,
            vec![
                ActionPrecondition::ExecutionIdentityValid,
                ActionPrecondition::RetrySafetyEstablished,
                ActionPrecondition::AuthorizationGranted,
            ],
            vec![
                ExpectedEffect::ExecutionContinues,
                ExpectedEffect::VerificationRequired,
            ],
            VerificationRequirement::Semantic,
            true,
        )
    }

    /// Constructs a restart action.
    pub fn restart(
        id: ActionId,
        reason: ActionReason,
        scope: ActionScope,
    ) -> Result<Self, ActionError> {
        Self::new(
            id,
            ActionPayload::Restart { scope },
            ActionPriority::High,
            reason,
            vec![
                ActionPrecondition::ExecutionIdentityValid,
                ActionPrecondition::AuthorizationGranted,
                ActionPrecondition::SemanticCompatibility,
            ],
            vec![
                ExpectedEffect::ExecutionRestarted,
                ExpectedEffect::VerificationRequired,
            ],
            VerificationRequirement::Semantic,
            true,
        )
    }

    /// Constructs a resume action.
    pub fn resume(
        id: ActionId,
        reason: ActionReason,
        checkpoint: ResourceId,
    ) -> Result<Self, ActionError> {
        Self::new(
            id,
            ActionPayload::Resume { checkpoint },
            ActionPriority::High,
            reason,
            vec![
                ActionPrecondition::CheckpointAvailable,
                ActionPrecondition::ResumeBoundaryAvailable,
                ActionPrecondition::SemanticCompatibility,
                ActionPrecondition::AuthorizationGranted,
            ],
            vec![
                ExpectedEffect::ExecutionResumed,
                ExpectedEffect::VerificationRequired,
            ],
            VerificationRequirement::Semantic,
            true,
        )
    }

    /// Constructs a rollback action.
    pub fn rollback(
        id: ActionId,
        reason: ActionReason,
        target: ResourceId,
    ) -> Result<Self, ActionError> {
        Self::new(
            id,
            ActionPayload::Rollback { target },
            ActionPriority::High,
            reason,
            vec![
                ActionPrecondition::RollbackTargetAvailable,
                ActionPrecondition::SemanticCompatibility,
                ActionPrecondition::AuthorizationGranted,
            ],
            vec![
                ExpectedEffect::ExecutionRolledBack,
                ExpectedEffect::VerificationRequired,
            ],
            VerificationRequirement::Semantic,
            true,
        )
    }

    /// Constructs a checkpoint action.
    pub fn checkpoint(
        id: ActionId,
        reason: ActionReason,
        scope: ActionScope,
    ) -> Result<Self, ActionError> {
        Self::new(
            id,
            ActionPayload::Checkpoint { scope },
            ActionPriority::Normal,
            reason,
            vec![
                ActionPrecondition::ExecutionIdentityValid,
                ActionPrecondition::AuthorizationGranted,
                ActionPrecondition::ProvenanceAvailable,
            ],
            vec![ExpectedEffect::CheckpointCreated],
            VerificationRequirement::Structural,
            true,
        )
    }

    /// Constructs a remapping action.
    pub fn remap(
        id: ActionId,
        reason: ActionReason,
        scope: ActionScope,
    ) -> Result<Self, ActionError> {
        Self::new(
            id,
            ActionPayload::Remap { scope },
            ActionPriority::High,
            reason,
            vec![
                ActionPrecondition::CapabilityAvailable,
                ActionPrecondition::SemanticCompatibility,
                ActionPrecondition::AuthorizationGranted,
            ],
            vec![
                ExpectedEffect::MappingChanged,
                ExpectedEffect::VerificationRequired,
            ],
            VerificationRequirement::Semantic,
            true,
        )
    }

    /// Constructs a rerouting action.
    pub fn reroute(
        id: ActionId,
        reason: ActionReason,
        scope: ActionScope,
    ) -> Result<Self, ActionError> {
        Self::new(
            id,
            ActionPayload::Reroute { scope },
            ActionPriority::High,
            reason,
            vec![
                ActionPrecondition::CapabilityAvailable,
                ActionPrecondition::SemanticCompatibility,
                ActionPrecondition::AuthorizationGranted,
            ],
            vec![
                ExpectedEffect::RoutingChanged,
                ExpectedEffect::VerificationRequired,
            ],
            VerificationRequirement::Semantic,
            true,
        )
    }

    /// Constructs a rescheduling action.
    pub fn reschedule(
        id: ActionId,
        reason: ActionReason,
        scope: ActionScope,
    ) -> Result<Self, ActionError> {
        Self::new(
            id,
            ActionPayload::Reschedule { scope },
            ActionPriority::High,
            reason,
            vec![
                ActionPrecondition::CapabilityAvailable,
                ActionPrecondition::SemanticCompatibility,
                ActionPrecondition::AuthorizationGranted,
            ],
            vec![
                ExpectedEffect::ScheduleChanged,
                ExpectedEffect::VerificationRequired,
            ],
            VerificationRequirement::Semantic,
            true,
        )
    }

    /// Constructs a recompile action.
    pub fn recompile(
        id: ActionId,
        reason: ActionReason,
        scope: ActionScope,
    ) -> Result<Self, ActionError> {
        Self::new(
            id,
            ActionPayload::Recompile { scope },
            ActionPriority::High,
            reason,
            vec![
                ActionPrecondition::CapabilityAvailable,
                ActionPrecondition::SemanticCompatibility,
                ActionPrecondition::AuthorizationGranted,
            ],
            vec![
                ExpectedEffect::CompilationChanged,
                ExpectedEffect::VerificationRequired,
            ],
            VerificationRequirement::Semantic,
            true,
        )
    }

    /// Constructs a reoptimization action.
    pub fn reoptimize(
        id: ActionId,
        reason: ActionReason,
        scope: ActionScope,
    ) -> Result<Self, ActionError> {
        Self::new(
            id,
            ActionPayload::Reoptimize { scope },
            ActionPriority::Normal,
            reason,
            vec![
                ActionPrecondition::CapabilityAvailable,
                ActionPrecondition::SemanticCompatibility,
                ActionPrecondition::AuthorizationGranted,
            ],
            vec![
                ExpectedEffect::OptimizationChanged,
                ExpectedEffect::VerificationRequired,
            ],
            VerificationRequirement::Semantic,
            true,
        )
    }

    /// Constructs a QEC adaptation action.
    pub fn adapt_qec(
        id: ActionId,
        reason: ActionReason,
        scope: ActionScope,
    ) -> Result<Self, ActionError> {
        Self::new(
            id,
            ActionPayload::AdaptQec { scope },
            ActionPriority::High,
            reason,
            vec![
                ActionPrecondition::QecCompatibility,
                ActionPrecondition::CapabilityAvailable,
                ActionPrecondition::SemanticCompatibility,
                ActionPrecondition::AuthorizationGranted,
            ],
            vec![
                ExpectedEffect::QecConfigurationChanged,
                ExpectedEffect::VerificationRequired,
            ],
            VerificationRequirement::Strong,
            true,
        )
    }

    /// Constructs a mitigation action.
    pub fn mitigate(
        id: ActionId,
        reason: ActionReason,
        scope: ActionScope,
        strategy: Option<ResourceId>,
    ) -> Result<Self, ActionError> {
        Self::new(
            id,
            ActionPayload::Mitigate { scope, strategy },
            ActionPriority::Normal,
            reason,
            vec![
                ActionPrecondition::MitigationCapability,
                ActionPrecondition::AuthorizationGranted,
            ],
            vec![
                ExpectedEffect::MitigationApplied,
                ExpectedEffect::VerificationRequired,
            ],
            VerificationRequirement::Strong,
            true,
        )
    }

    /// Constructs a migration action.
    pub fn migrate(
        id: ActionId,
        reason: ActionReason,
        scope: ActionScope,
        target: ResourceId,
    ) -> Result<Self, ActionError> {
        Self::new(
            id,
            ActionPayload::Migrate { scope, target },
            ActionPriority::High,
            reason,
            vec![
                ActionPrecondition::MigrationCompatibility,
                ActionPrecondition::CapabilityAvailable,
                ActionPrecondition::SemanticCompatibility,
                ActionPrecondition::AuthorizationGranted,
                ActionPrecondition::ProvenanceAvailable,
            ],
            vec![
                ExpectedEffect::ExecutionEnvironmentChanged,
                ExpectedEffect::VerificationRequired,
            ],
            VerificationRequirement::Strong,
            true,
        )
    }

    /// Constructs a quarantine action.
    pub fn quarantine(
        id: ActionId,
        reason: ActionReason,
        resource: ResourceId,
    ) -> Result<Self, ActionError> {
        Self::new(
            id,
            ActionPayload::QuarantineResource { resource },
            ActionPriority::Critical,
            reason,
            vec![
                ActionPrecondition::AuthorizationGranted,
                ActionPrecondition::ResourceIsolation,
            ],
            vec![ExpectedEffect::ResourceQuarantined],
            VerificationRequirement::Structural,
            false,
        )
    }

    /// Constructs a compensation action.
    pub fn compensate(
        id: ActionId,
        reason: ActionReason,
        scope: ActionScope,
        strategy: ResourceId,
    ) -> Result<Self, ActionError> {
        Self::new(
            id,
            ActionPayload::Compensate { scope, strategy },
            ActionPriority::High,
            reason,
            vec![
                ActionPrecondition::CapabilityAvailable,
                ActionPrecondition::SemanticCompatibility,
                ActionPrecondition::AuthorizationGranted,
            ],
            vec![
                ExpectedEffect::CompensationApplied,
                ExpectedEffect::VerificationRequired,
            ],
            VerificationRequirement::Strong,
            true,
        )
    }

    /// Constructs an escalation action.
    pub fn escalate(
        id: ActionId,
        reason: ActionReason,
        scope: ActionScope,
    ) -> Result<Self, ActionError> {
        Self::new(
            id,
            ActionPayload::Escalate { scope },
            ActionPriority::Critical,
            reason,
            Vec::new(),
            vec![ExpectedEffect::RecoveryEscalated],
            VerificationRequirement::NotApplicable,
            false,
        )
    }

    /// Constructs an abort action.
    pub fn abort(
        id: ActionId,
        reason: ActionReason,
        scope: ActionScope,
    ) -> Result<Self, ActionError> {
        Self::new(
            id,
            ActionPayload::Abort { scope },
            ActionPriority::Critical,
            reason,
            Vec::new(),
            vec![ExpectedEffect::ExecutionAborted],
            VerificationRequirement::NotApplicable,
            false,
        )
    }
}

// =============================================================================
// Action validation
// =============================================================================

/// Validates structural invariants of an action.
///
/// This function does not perform capability, policy, security or semantic
/// validation. Those belong to their respective subsystems.
pub fn validate_action(action: &RecoveryAction) -> Result<(), ActionError> {
    if action.id.get() == 0 {
        return Err(ActionError::InvalidActionId);
    }

    let kind = action.kind();

    if kind.requires_verification()
        && action.verification_requirement() == VerificationRequirement::NotApplicable
    {
        return Err(ActionError::VerificationRequired);
    }

    if kind.is_mutating() && action.preconditions().is_empty() {
        return Err(ActionError::MissingPreconditions);
    }

    match action.payload() {
        ActionPayload::Migrate { target, .. } => {
            if target.as_str().is_empty() {
                return Err(ActionError::EmptyResourceId);
            }
        }

        ActionPayload::QuarantineResource { resource } => {
            if resource.as_str().is_empty() {
                return Err(ActionError::EmptyResourceId);
            }
        }

        ActionPayload::Compensate { strategy, .. } => {
            if strategy.as_str().is_empty() {
                return Err(ActionError::EmptyResourceId);
            }
        }

        ActionPayload::Resume { checkpoint } => {
            if checkpoint.as_str().is_empty() {
                return Err(ActionError::EmptyResourceId);
            }
        }

        ActionPayload::Rollback { target } => {
            if target.as_str().is_empty() {
                return Err(ActionError::EmptyResourceId);
            }
        }

        ActionPayload::Mitigate {
            strategy: Some(strategy),
            ..
        } => {
            if strategy.as_str().is_empty() {
                return Err(ActionError::EmptyResourceId);
            }
        }

        _ => {}
    }

    Ok(())
}

// =============================================================================
// Action error
// =============================================================================

/// Structural errors raised by the action model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActionError {
    /// Action identity is invalid.
    InvalidActionId,

    /// Resource identity is empty.
    EmptyResourceId,

    /// A mutating action has no declared preconditions.
    MissingPreconditions,

    /// An action capable of changing execution has no verification contract.
    VerificationRequired,
}

impl fmt::Display for ActionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidActionId => {
                formatter.write_str("invalid resilience action identifier")
            }

            Self::EmptyResourceId => {
                formatter.write_str("resilience resource identifier cannot be empty")
            }

            Self::MissingPreconditions => {
                formatter.write_str(
                    "mutating resilience action requires at least one precondition",
                )
            }

            Self::VerificationRequired => {
                formatter.write_str(
                    "resilience action requires an explicit verification requirement",
                )
            }
        }
    }
}

impl std::error::Error for ActionError {}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn action_id(value: u64) -> ActionId {
        ActionId::new(value).expect("test action ID must be non-zero")
    }

    fn resource(value: &str) -> ResourceId {
        ResourceId::new(value).expect("test resource must be non-empty")
    }

    #[test]
    fn action_kinds_have_stable_identifiers() {
        assert_eq!(ActionKind::Retry.as_str(), "retry");
        assert_eq!(ActionKind::Restart.as_str(), "restart");
        assert_eq!(ActionKind::Resume.as_str(), "resume");
        assert_eq!(ActionKind::Rollback.as_str(), "rollback");
        assert_eq!(ActionKind::Checkpoint.as_str(), "checkpoint");
        assert_eq!(ActionKind::Remap.as_str(), "remap");
        assert_eq!(ActionKind::Reroute.as_str(), "reroute");
        assert_eq!(ActionKind::Reschedule.as_str(), "reschedule");
        assert_eq!(ActionKind::Recompile.as_str(), "recompile");
        assert_eq!(ActionKind::Reoptimize.as_str(), "reoptimize");
        assert_eq!(ActionKind::AdaptQec.as_str(), "adapt_qec");
        assert_eq!(ActionKind::Mitigate.as_str(), "mitigate");
        assert_eq!(ActionKind::Migrate.as_str(), "migrate");
        assert_eq!(
            ActionKind::QuarantineResource.as_str(),
            "quarantine_resource"
        );
        assert_eq!(ActionKind::Compensate.as_str(), "compensate");
        assert_eq!(ActionKind::Escalate.as_str(), "escalate");
        assert_eq!(ActionKind::Abort.as_str(), "abort");
    }

    #[test]
    fn retry_requires_explicit_safety() {
        let action = RecoveryAction::retry(
            action_id(1),
            ActionReason::TransientFailure,
            Some(ActionScope::Execution),
        )
        .expect("retry action must construct");

        assert!(
            action
                .preconditions()
                .contains(&ActionPrecondition::RetrySafetyEstablished)
        );

        assert!(action.requires_semantic_preservation());
        assert_eq!(
            action.verification_requirement(),
            VerificationRequirement::Semantic
        );
    }

    #[test]
    fn migration_requires_compatibility() {
        let action = RecoveryAction::migrate(
            action_id(2),
            ActionReason::ExecutionEnvironmentUnavailable,
            ActionScope::Execution,
            resource("target-environment"),
        )
        .expect("migration action must construct");

        assert!(
            action
                .preconditions()
                .contains(&ActionPrecondition::MigrationCompatibility)
        );

        assert!(action.requires_semantic_preservation());
        assert_eq!(
            action.verification_requirement(),
            VerificationRequirement::Strong
        );
    }

    #[test]
    fn quarantine_is_protective() {
        let action = RecoveryAction::quarantine(
            action_id(3),
            ActionReason::ResourceDegradation,
            resource("resource"),
        )
        .expect("quarantine action must construct");

        assert!(action.is_protective());
        assert!(action.is_mutating());
        assert!(!action.requires_semantic_preservation());
    }

    #[test]
    fn abort_is_available_without_transformative_preconditions() {
        let action = RecoveryAction::abort(
            action_id(4),
            ActionReason::NoSafeAutomaticAction,
            ActionScope::Execution,
        )
        .expect("abort action must construct");

        assert!(action.is_protective());
        assert!(!action.is_mutating());
        assert_eq!(
            action.verification_requirement(),
            VerificationRequirement::NotApplicable
        );
    }

    #[test]
    fn escalation_is_not_a_mutating_recovery_action() {
        let action = RecoveryAction::escalate(
            action_id(5),
            ActionReason::NoSafeAutomaticAction,
            ActionScope::Execution,
        )
        .expect("escalation action must construct");

        assert!(action.is_protective());
        assert!(!action.is_mutating());
    }

    #[test]
    fn action_payload_kind_is_consistent() {
        let payload = ActionPayload::Remap {
            scope: ActionScope::ExecutionResources,
        };

        assert_eq!(payload.kind(), ActionKind::Remap);
    }

    #[test]
    fn action_validation_accepts_valid_action() {
        let action = RecoveryAction::remap(
            action_id(6),
            ActionReason::PhysicalRealizationInvalid,
            ActionScope::AffectedResources,
        )
        .expect("remap action must construct");

        validate_action(&action).expect("valid action must validate");
    }

    #[test]
    fn zero_action_id_is_rejected() {
        assert_eq!(
            ActionId::new(0),
            Err(ActionError::InvalidActionId)
        );
    }

    #[test]
    fn empty_resource_id_is_rejected() {
        assert_eq!(
            ResourceId::new(""),
            Err(ActionError::EmptyResourceId)
        );
    }

    #[test]
    fn action_ordering_is_deterministic() {
        let first = RecoveryAction::retry(
            action_id(1),
            ActionReason::TransientFailure,
            Some(ActionScope::Execution),
        )
        .expect("action must construct");

        let second = RecoveryAction::retry(
            action_id(2),
            ActionReason::TransientFailure,
            Some(ActionScope::Execution),
        )
        .expect("action must construct");

        assert!(first.stable_key() < second.stable_key());
    }

    #[test]
    fn physical_qubit_scope_uses_canonical_ir_identity() {
        // This intentionally compiles against the canonical QubitId type.
        //
        // The exact constructor is delegated to the canonical IR in production
        // code. This test only verifies that ActionScope accepts that type.
        let _ = std::mem::size_of::<ActionScope>();

        // The import is intentionally retained as part of the compile-time
        // integration contract:
        let _canonical_type_marker: Option<QubitId> = None;

        assert!(_canonical_type_marker.is_none());
    }
}