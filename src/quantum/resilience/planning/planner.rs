//! Zamani Quantum Resilience — Production Recovery Planner
//!
//! Path:
//!     src/quantum/resilience/planning/planner.rs
//!
//! Purpose:
//!     Provider-neutral, deterministic recovery-plan candidate generation.
//!
//! Architectural position:
//!
//! ```text
//!                    Quantum Execution
//!                           |
//!                           v
//!                    Detection / Diagnosis
//!                           |
//!                           v
//!                         Policy
//!                           |
//!                           v
//!                 +---------------------+
//!                 |      Planner        |
//!                 |                     |
//!                 | diagnosis           |
//!                 | policy              |
//!                 | capabilities        |
//!                 | resource state      |
//!                 | history             |
//!                 | verification        |
//!                 +----------+----------+
//!                            |
//!                            v
//!                    Recovery Plan
//!                            |
//!              +-------------+-------------+
//!              |             |             |
//!              v             v             v
//!           Adaptation    Recovery     Mitigation
//!              |             |             |
//!              +-------------+-------------+
//!                            |
//!                            v
//!                        Verification
//! ```
//!
//! # Responsibility
//!
//! This module owns the decision boundary that turns normalized resilience
//! observations into an ordered, bounded set of recovery-plan candidates.
//!
//! It does NOT execute recovery.
//!
//! It does NOT mutate Quantum IR.
//!
//! It does NOT perform routing.
//!
//! It does NOT perform scheduling.
//!
//! It does NOT implement QEC.
//!
//! It does NOT implement mitigation algorithms.
//!
//! It does NOT communicate with a QPU.
//!
//! It does NOT contain provider-specific logic.
//!
//! It does NOT assume a fixed number of qubits, devices, backends, shots,
//! retries, or machines.
//!
//! # Write once, scale everywhere
//!
//! A Zamani program is expressed against canonical logical quantum resources.
//! The planner therefore operates on resource identities and capability
//! facts supplied by the execution environment.
//!
//! No architectural quantum-machine size is encoded here.
//!
//! In particular, this file MUST NOT introduce assumptions such as:
//!
//! ```text
//! MAX_QUBITS = 127
//! MAX_QUBITS = 1000
//! retry = 3
//! fidelity < 0.95
//! qubit 7 is always bad
//! backend X always supports operation Y
//! ```
//!
//! All such values must originate from:
//!
//! - policy;
//! - discovered target capabilities;
//! - execution state;
//! - resource availability;
//! - security policy;
//! - workload constraints;
//! - verified historical evidence.
//!
//! # Determinism
//!
//! Given equivalent normalized inputs and a deterministic planning policy,
//! candidate generation and ordering are deterministic.
//!
//! External nondeterminism is not introduced by this module.
//!
//! Randomized strategies must receive their randomness from an explicit
//! caller-controlled source outside this planner.
//!
//! # Safety
//!
//! The planner follows the central resilience invariant:
//!
//! > Availability alone is never sufficient to authorize recovery.
//!
//! Every candidate carries explicit requirements describing what must be
//! checked before execution.
//!
//! Final authorization belongs to policy, feasibility and verification
//! subsystems.
//!
//! # Canonical quantum identity
//!
//! When this module needs to represent logical or physical qubit identities,
//! the authoritative types are:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! This module deliberately does not redefine those types.
//!
//! # Integration
//!
//! Upstream:
//!
//! ```text
//! detection
//! diagnosis
//! policy
//! state
//! telemetry
//! history
//! hardware capabilities
//! ```
//!
//! Downstream:
//!
//! ```text
//! planning/action.rs
//! planning/plan.rs
//! planning/cost.rs
//! planning/feasibility.rs
//! planning/ranking.rs
//! adaptation/*
//! recovery/*
//! mitigation/*
//! verification/*
//! ```
//!
//! The planner communicates with those modules through stable semantic
//! identifiers and immutable planner data. Concrete implementations remain
//! outside this file.
//!
//! # Rust
//!
//! Compatible with:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! # Resource complexity
//!
//! Planning is bounded by the number of supplied candidate strategies and
//! does not enumerate possible circuits, qubit subsets, mappings, or machines.
//!
//! A large quantum machine therefore does not cause a combinatorial planner
//! expansion merely because it contains more physical resources.
//!
//! Expensive search belongs in specialized routing, scheduling, optimization,
//! resource-estimation, or backend-selection components.
//!
//! # Important distinction
//!
//! "Scalable to infinity" means that this module introduces no artificial
//! machine-size ceiling. Every actual execution remains bounded by the
//! resources and policies available to that execution.
//!
//! =============================================================================
//! Compiler-enforced safety boundary
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::cmp::Ordering;
use std::fmt;

// =============================================================================
// Stable schema
// =============================================================================

/// Stable schema identifier for planner decisions.
pub const PLANNER_SCHEMA_ID: &str = "zamani.quantum.resilience.planner";

/// Semantic version of the planner contract.
///
/// Increment this when the externally observable planner semantics change.
pub const PLANNER_SCHEMA_VERSION: u16 = 1;

/// Stable planner implementation version.
///
/// This identifies the decision algorithm rather than serialized data.
pub const PLANNER_VERSION: u32 = 1;

// =============================================================================
// Bounded planner constants
// =============================================================================
//
// These are safety/representation bounds, NOT quantum-machine limits.
//
// They prevent malformed callers from causing unbounded planner allocation.
// They do not constrain the number of qubits, operations, machines, or
// resources that Zamani can represent elsewhere.

/// Maximum number of candidates returned by one planner invocation.
///
/// This bounds the planner's decision surface rather than the quantum system.
pub const MAX_CANDIDATES: usize = 64;

/// Maximum number of planner requirements retained per candidate.
pub const MAX_REQUIREMENTS: usize = 32;

/// Maximum number of diagnostic reasons retained per candidate.
pub const MAX_REASONS: usize = 16;

// =============================================================================
// Stable action identifiers
// =============================================================================

/// Stable identifiers for recovery/adaptation actions.
///
/// These identifiers are intentionally independent from concrete
/// implementations in `action.rs`, `recovery/*`, and `adaptation/*`.
pub mod action_id {
    /// Re-attempt the current execution.
    pub const RETRY: &str = "retry";

    /// Restart execution from a valid restart boundary.
    pub const RESTART: &str = "restart";

    /// Resume from a valid checkpoint or semantic boundary.
    pub const RESUME: &str = "resume";

    /// Roll back to a previously accepted state.
    pub const ROLLBACK: &str = "rollback";

    /// Create/use a checkpoint before continuing.
    pub const CHECKPOINT: &str = "checkpoint";

    /// Recompute logical-to-physical placement.
    pub const REMAP: &str = "remap";

    /// Recompute physical routing.
    pub const REROUTE: &str = "reroute";

    /// Rebuild execution scheduling.
    pub const RESCHEDULE: &str = "reschedule";

    /// Recompile against a changed target.
    pub const RECOMPILE: &str = "recompile";

    /// Re-run target-aware optimization.
    pub const REOPTIMIZE: &str = "reoptimize";

    /// Adapt QEC configuration.
    pub const ADAPT_QEC: &str = "adapt_qec";

    /// Apply an available mitigation strategy.
    pub const MITIGATE: &str = "mitigate";

    /// Move execution to another compatible backend/device.
    pub const MIGRATE: &str = "migrate";

    /// Temporarily exclude a degraded resource.
    pub const QUARANTINE_RESOURCE: &str = "quarantine_resource";

    /// Apply a mathematically defined compensation.
    pub const COMPENSATE: &str = "compensate";

    /// Escalate to an external/human/upper-layer decision boundary.
    pub const ESCALATE: &str = "escalate";

    /// Abort execution when correctness cannot be protected.
    pub const ABORT: &str = "abort";
}

// =============================================================================
// Planning phase
// =============================================================================

/// Phase at which an action belongs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PlanPhase {
    /// Preserve the current execution if possible.
    Preserve,

    /// Retry without changing semantic implementation.
    Retry,

    /// Restore from an already valid execution boundary.
    Restore,

    /// Change resource allocation.
    AdaptResources,

    /// Change physical realization.
    AdaptImplementation,

    /// Change QEC configuration.
    AdaptQec,

    /// Apply error mitigation.
    Mitigate,

    /// Move execution to another execution environment.
    Migrate,

    /// Verify the resulting execution.
    Verify,

    /// Stop automatic recovery.
    Escalate,
}

impl PlanPhase {
    /// Returns a stable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Preserve => "preserve",
            Self::Retry => "retry",
            Self::Restore => "restore",
            Self::AdaptResources => "adapt_resources",
            Self::AdaptImplementation => "adapt_implementation",
            Self::AdaptQec => "adapt_qec",
            Self::Mitigate => "mitigate",
            Self::Migrate => "migrate",
            Self::Verify => "verify",
            Self::Escalate => "escalate",
        }
    }
}

// =============================================================================
// Planning reasons
// =============================================================================

/// Stable explanation for why a candidate was generated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlanningReason {
    /// A transient failure may be safely retried.
    TransientFailure,

    /// Execution state can be restored.
    RestorableExecution,

    /// A resource has degraded.
    ResourceDegradation,

    /// The physical realization has become invalid.
    PhysicalRealizationInvalid,

    /// Target capabilities changed.
    CapabilityChange,

    /// Backend/device availability changed.
    BackendUnavailable,

    /// Timing/scheduling state became invalid.
    SchedulingInvalid,

    /// Routing/topology state became invalid.
    RoutingInvalid,

    /// Compilation target became invalid.
    CompilationInvalid,

    /// QEC state degraded.
    QecDegradation,

    /// Noise requires mitigation.
    NoiseMitigation,

    /// Current result requires stronger verification.
    VerificationRequired,

    /// No safe automatic action remains.
    NoSafeAutomaticAction,

    /// Explicit policy requested the action.
    PolicyRequested,

    /// Explicit caller request.
    CallerRequested,

    /// Multiple faults were correlated.
    CorrelatedIncident,

    /// Previous recovery attempt failed.
    PreviousRecoveryFailed,
}

impl PlanningReason {
    /// Returns a stable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TransientFailure => "transient_failure",
            Self::RestorableExecution => "restorable_execution",
            Self::ResourceDegradation => "resource_degradation",
            Self::PhysicalRealizationInvalid => "physical_realization_invalid",
            Self::CapabilityChange => "capability_change",
            Self::BackendUnavailable => "backend_unavailable",
            Self::SchedulingInvalid => "scheduling_invalid",
            Self::RoutingInvalid => "routing_invalid",
            Self::CompilationInvalid => "compilation_invalid",
            Self::QecDegradation => "qec_degradation",
            Self::NoiseMitigation => "noise_mitigation",
            Self::VerificationRequired => "verification_required",
            Self::NoSafeAutomaticAction => "no_safe_automatic_action",
            Self::PolicyRequested => "policy_requested",
            Self::CallerRequested => "caller_requested",
            Self::CorrelatedIncident => "correlated_incident",
            Self::PreviousRecoveryFailed => "previous_recovery_failed",
        }
    }
}

// =============================================================================
// Planner error
// =============================================================================

/// Errors produced while constructing a recovery plan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlannerError {
    /// The supplied planner input is internally inconsistent.
    InvalidInput {
        /// Stable explanation.
        message: String,
    },

    /// Candidate storage would exceed the planner's representation boundary.
    CandidateLimitExceeded {
        /// Number requested.
        requested: usize,

        /// Maximum retained.
        maximum: usize,
    },

    /// Requirement storage would exceed the planner's representation boundary.
    RequirementLimitExceeded {
        /// Number requested.
        requested: usize,

        /// Maximum retained.
        maximum: usize,
    },

    /// A stable action identifier is empty or malformed.
    InvalidActionIdentifier {
        /// Invalid identifier.
        value: String,
    },

    /// An action was duplicated.
    DuplicateAction {
        /// Duplicate identifier.
        action: String,
    },

    /// Arithmetic overflow occurred.
    ArithmeticOverflow {
        /// Calculation name.
        calculation: &'static str,
    },
}

impl fmt::Display for PlannerError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidInput { message } => {
                write!(formatter, "invalid resilience planner input: {message}")
            }

            Self::CandidateLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "planner candidate limit exceeded: requested {requested}, maximum {maximum}"
                )
            }

            Self::RequirementLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "planner requirement limit exceeded: requested {requested}, maximum {maximum}"
                )
            }

            Self::InvalidActionIdentifier { value } => {
                write!(
                    formatter,
                    "invalid resilience planner action identifier `{value}`"
                )
            }

            Self::DuplicateAction { action } => {
                write!(
                    formatter,
                    "duplicate resilience planner action `{action}`"
                )
            }

            Self::ArithmeticOverflow { calculation } => {
                write!(
                    formatter,
                    "arithmetic overflow while calculating {calculation}"
                )
            }
        }
    }
}

impl std::error::Error for PlannerError {}

/// Result type for planner operations.
pub type PlannerResult<T> = Result<T, PlannerError>;

// =============================================================================
// Planner input
// =============================================================================

/// Immutable normalized planner input.
///
/// This type deliberately contains facts rather than concrete subsystem
/// implementations. Detection, diagnosis, policy, hardware and execution
/// layers can therefore supply equivalent information without coupling the
/// planner to their concrete types.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlannerInput {
    /// Whether execution is currently active.
    execution_active: bool,

    /// Whether a valid checkpoint/restart boundary exists.
    checkpoint_available: bool,

    /// Whether the current logical computation can be resumed.
    resume_available: bool,

    /// Whether the current execution can be retried without semantic risk.
    retry_safe: bool,

    /// Whether a rollback target is available.
    rollback_available: bool,

    /// Whether the physical mapping is still valid.
    mapping_valid: bool,

    /// Whether routing remains valid.
    routing_valid: bool,

    /// Whether scheduling remains valid.
    schedule_valid: bool,

    /// Whether the current compilation remains valid for the target.
    compilation_valid: bool,

    /// Whether target capabilities changed since compilation.
    capabilities_changed: bool,

    /// Whether the selected backend remains available.
    backend_available: bool,

    /// Whether a compatible alternative backend may be selected.
    migration_possible: bool,

    /// Whether the current resource allocation remains feasible.
    resources_feasible: bool,

    /// Whether a degraded resource can be quarantined.
    quarantine_possible: bool,

    /// Whether QEC adaptation is supported.
    qec_adaptation_possible: bool,

    /// Whether mitigation is supported.
    mitigation_possible: bool,

    /// Whether compensation is explicitly available.
    compensation_possible: bool,

    /// Whether semantic verification is available.
    verification_available: bool,

    /// Whether the last recovery attempt failed.
    previous_recovery_failed: bool,

    /// Whether the incident is correlated across multiple observations.
    correlated_incident: bool,

    /// Primary reason for planning.
    primary_reason: PlanningReason,

    /// Number of previous recovery attempts.
    ///
    /// This is informational. Policy determines whether another attempt is
    /// allowed. The planner never assumes a fixed retry count.
    recovery_attempts: u128,

    /// Estimated severity on a caller-defined ordered scale.
    ///
    /// The planner treats this as an ordering value only. It does not assign
    /// provider-specific meanings to numeric values.
    severity_rank: u128,
}

impl Default for PlannerInput {
    fn default() -> Self {
        Self {
            execution_active: false,
            checkpoint_available: false,
            resume_available: false,
            retry_safe: false,
            rollback_available: false,
            mapping_valid: true,
            routing_valid: true,
            schedule_valid: true,
            compilation_valid: true,
            capabilities_changed: false,
            backend_available: true,
            migration_possible: false,
            resources_feasible: true,
            quarantine_possible: false,
            qec_adaptation_possible: false,
            mitigation_possible: false,
            compensation_possible: false,
            verification_available: false,
            previous_recovery_failed: false,
            correlated_incident: false,
            primary_reason: PlanningReason::TransientFailure,
            recovery_attempts: 0,
            severity_rank: 0,
        }
    }
}

impl PlannerInput {
    /// Creates a new normalized planner input with conservative defaults.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns whether execution is active.
    #[must_use]
    pub const fn execution_active(&self) -> bool {
        self.execution_active
    }

    /// Sets execution-active state.
    pub const fn with_execution_active(mut self, value: bool) -> Self {
        self.execution_active = value;
        self
    }

    /// Returns whether a checkpoint exists.
    #[must_use]
    pub const fn checkpoint_available(&self) -> bool {
        self.checkpoint_available
    }

    /// Sets checkpoint availability.
    pub const fn with_checkpoint_available(mut self, value: bool) -> Self {
        self.checkpoint_available = value;
        self
    }

    /// Returns whether resume is possible.
    #[must_use]
    pub const fn resume_available(&self) -> bool {
        self.resume_available
    }

    /// Sets resume availability.
    pub const fn with_resume_available(mut self, value: bool) -> Self {
        self.resume_available = value;
        self
    }

    /// Returns whether retry is safe.
    #[must_use]
    pub const fn retry_safe(&self) -> bool {
        self.retry_safe
    }

    /// Sets retry safety.
    pub const fn with_retry_safe(mut self, value: bool) -> Self {
        self.retry_safe = value;
        self
    }

    /// Returns whether rollback is possible.
    #[must_use]
    pub const fn rollback_available(&self) -> bool {
        self.rollback_available
    }

    /// Sets rollback availability.
    pub const fn with_rollback_available(mut self, value: bool) -> Self {
        self.rollback_available = value;
        self
    }

    /// Returns whether mapping remains valid.
    #[must_use]
    pub const fn mapping_valid(&self) -> bool {
        self.mapping_valid
    }

    /// Sets mapping validity.
    pub const fn with_mapping_valid(mut self, value: bool) -> Self {
        self.mapping_valid = value;
        self
    }

    /// Returns whether routing remains valid.
    #[must_use]
    pub const fn routing_valid(&self) -> bool {
        self.routing_valid
    }

    /// Sets routing validity.
    pub const fn with_routing_valid(mut self, value: bool) -> Self {
        self.routing_valid = value;
        self
    }

    /// Returns whether scheduling remains valid.
    #[must_use]
    pub const fn schedule_valid(&self) -> bool {
        self.schedule_valid
    }

    /// Sets scheduling validity.
    pub const fn with_schedule_valid(mut self, value: bool) -> Self {
        self.schedule_valid = value;
        self
    }

    /// Returns whether compilation remains valid.
    #[must_use]
    pub const fn compilation_valid(&self) -> bool {
        self.compilation_valid
    }

    /// Sets compilation validity.
    pub const fn with_compilation_valid(mut self, value: bool) -> Self {
        self.compilation_valid = value;
        self
    }

    /// Returns whether capabilities changed.
    #[must_use]
    pub const fn capabilities_changed(&self) -> bool {
        self.capabilities_changed
    }

    /// Sets capability-change state.
    pub const fn with_capabilities_changed(mut self, value: bool) -> Self {
        self.capabilities_changed = value;
        self
    }

    /// Returns whether the current backend is available.
    #[must_use]
    pub const fn backend_available(&self) -> bool {
        self.backend_available
    }

    /// Sets backend availability.
    pub const fn with_backend_available(mut self, value: bool) -> Self {
        self.backend_available = value;
        self
    }

    /// Returns whether migration is possible.
    #[must_use]
    pub const fn migration_possible(&self) -> bool {
        self.migration_possible
    }

    /// Sets migration availability.
    pub const fn with_migration_possible(mut self, value: bool) -> Self {
        self.migration_possible = value;
        self
    }

    /// Returns whether current resources are feasible.
    #[must_use]
    pub const fn resources_feasible(&self) -> bool {
        self.resources_feasible
    }

    /// Sets resource feasibility.
    pub const fn with_resources_feasible(mut self, value: bool) -> Self {
        self.resources_feasible = value;
        self
    }

    /// Returns whether resource quarantine is possible.
    #[must_use]
    pub const fn quarantine_possible(&self) -> bool {
        self.quarantine_possible
    }

    /// Sets quarantine availability.
    pub const fn with_quarantine_possible(mut self, value: bool) -> Self {
        self.quarantine_possible = value;
        self
    }

    /// Returns whether QEC adaptation is possible.
    #[must_use]
    pub const fn qec_adaptation_possible(&self) -> bool {
        self.qec_adaptation_possible
    }

    /// Sets QEC-adaptation availability.
    pub const fn with_qec_adaptation_possible(mut self, value: bool) -> Self {
        self.qec_adaptation_possible = value;
        self
    }

    /// Returns whether mitigation is possible.
    #[must_use]
    pub const fn mitigation_possible(&self) -> bool {
        self.mitigation_possible
    }

    /// Sets mitigation availability.
    pub const fn with_mitigation_possible(mut self, value: bool) -> Self {
        self.mitigation_possible = value;
        self
    }

    /// Returns whether compensation is possible.
    #[must_use]
    pub const fn compensation_possible(&self) -> bool {
        self.compensation_possible
    }

    /// Sets compensation availability.
    pub const fn with_compensation_possible(mut self, value: bool) -> Self {
        self.compensation_possible = value;
        self
    }

    /// Returns whether verification is available.
    #[must_use]
    pub const fn verification_available(&self) -> bool {
        self.verification_available
    }

    /// Sets verification availability.
    pub const fn with_verification_available(mut self, value: bool) -> Self {
        self.verification_available = value;
        self
    }

    /// Returns whether the previous recovery failed.
    #[must_use]
    pub const fn previous_recovery_failed(&self) -> bool {
        self.previous_recovery_failed
    }

    /// Sets previous-recovery failure state.
    pub const fn with_previous_recovery_failed(mut self, value: bool) -> Self {
        self.previous_recovery_failed = value;
        self
    }

    /// Returns whether this is a correlated incident.
    #[must_use]
    pub const fn correlated_incident(&self) -> bool {
        self.correlated_incident
    }

    /// Sets correlated-incident state.
    pub const fn with_correlated_incident(mut self, value: bool) -> Self {
        self.correlated_incident = value;
        self
    }

    /// Returns the primary reason.
    #[must_use]
    pub const fn primary_reason(&self) -> PlanningReason {
        self.primary_reason
    }

    /// Sets the primary reason.
    pub const fn with_primary_reason(mut self, value: PlanningReason) -> Self {
        self.primary_reason = value;
        self
    }

    /// Returns previous recovery-attempt count.
    #[must_use]
    pub const fn recovery_attempts(&self) -> u128 {
        self.recovery_attempts
    }

    /// Sets previous recovery-attempt count.
    pub const fn with_recovery_attempts(mut self, value: u128) -> Self {
        self.recovery_attempts = value;
        self
    }

    /// Returns severity ordering rank.
    #[must_use]
    pub const fn severity_rank(&self) -> u128 {
        self.severity_rank
    }

    /// Sets severity ordering rank.
    pub const fn with_severity_rank(mut self, value: u128) -> Self {
        self.severity_rank = value;
        self
    }

    /// Validates the normalized planner input.
    pub fn validate(&self) -> PlannerResult<()> {
        if self.execution_active
            && !self.backend_available
            && !self.migration_possible
            && !self.checkpoint_available
            && !self.retry_safe
            && !self.rollback_available
            && self.mapping_valid
            && self.routing_valid
            && self.schedule_valid
            && self.compilation_valid
        {
            return Err(PlannerError::InvalidInput {
                message: String::from(
                    "active execution has no available execution-preservation path",
                ),
            });
        }

        Ok(())
    }
}

// =============================================================================
// Candidate requirements
// =============================================================================

/// A condition that must be satisfied before a candidate may execute.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Requirement {
    /// Policy must authorize the action.
    PolicyAuthorization,

    /// Target capabilities must still satisfy the action.
    CapabilityValidation,

    /// Current resource state must be revalidated.
    ResourceValidation,

    /// Current execution generation/state must still match.
    ExecutionStateValidation,

    /// Semantic correctness must be verified after the action.
    SemanticVerification,

    /// Result correctness must be verified.
    ResultVerification,

    /// Provenance must be retained.
    ProvenanceRetention,

    /// Security authorization is required.
    SecurityAuthorization,

    /// Checkpoint integrity must be verified.
    CheckpointIntegrity,

    /// A valid logical-to-physical mapping is required.
    ValidMapping,

    /// A valid route is required.
    ValidRouting,

    /// A valid schedule is required.
    ValidSchedule,

    /// A compatible compilation target is required.
    CompatibleCompilation,

    /// A compatible backend is required.
    CompatibleBackend,

    /// A valid QEC configuration is required.
    ValidQecConfiguration,
}

impl Requirement {
    /// Returns a stable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PolicyAuthorization => "policy_authorization",
            Self::CapabilityValidation => "capability_validation",
            Self::ResourceValidation => "resource_validation",
            Self::ExecutionStateValidation => "execution_state_validation",
            Self::SemanticVerification => "semantic_verification",
            Self::ResultVerification => "result_verification",
            Self::ProvenanceRetention => "provenance_retention",
            Self::SecurityAuthorization => "security_authorization",
            Self::CheckpointIntegrity => "checkpoint_integrity",
            Self::ValidMapping => "valid_mapping",
            Self::ValidRouting => "valid_routing",
            Self::ValidSchedule => "valid_schedule",
            Self::CompatibleCompilation => "compatible_compilation",
            Self::CompatibleBackend => "compatible_backend",
            Self::ValidQecConfiguration => "valid_qec_configuration",
        }
    }
}

// =============================================================================
// Candidate
// =============================================================================

/// One immutable recovery-plan candidate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlanCandidate {
    action: String,
    phase: PlanPhase,
    reason: PlanningReason,
    priority: u32,
    requirements: Vec<Requirement>,
}

impl PlanCandidate {
    fn new(
        action: &'static str,
        phase: PlanPhase,
        reason: PlanningReason,
        priority: u32,
    ) -> PlannerResult<Self> {
        validate_action_id(action)?;

        Ok(Self {
            action: String::from(action),
            phase,
            reason,
            priority,
            requirements: Vec::new(),
        })
    }

    fn with_requirements(
        mut self,
        requirements: &[Requirement],
    ) -> PlannerResult<Self> {
        if requirements.len() > MAX_REQUIREMENTS {
            return Err(PlannerError::RequirementLimitExceeded {
                requested: requirements.len(),
                maximum: MAX_REQUIREMENTS,
            });
        }

        self.requirements.extend_from_slice(requirements);

        Ok(self)
    }

    /// Returns the stable action identifier.
    #[must_use]
    pub fn action(&self) -> &str {
        &self.action
    }

    /// Returns the planning phase.
    #[must_use]
    pub const fn phase(&self) -> PlanPhase {
        self.phase
    }

    /// Returns the reason for selection.
    #[must_use]
    pub const fn reason(&self) -> PlanningReason {
        self.reason
    }

    /// Returns the deterministic priority.
    #[must_use]
    pub const fn priority(&self) -> u32 {
        self.priority
    }

    /// Returns all execution requirements.
    #[must_use]
    pub fn requirements(&self) -> &[Requirement] {
        &self.requirements
    }
}

// =============================================================================
// Planner configuration
// =============================================================================

/// Configuration controlling planner candidate generation.
///
/// This is intentionally a small decision-policy boundary. Detailed policy
/// constraints remain owned by `policy/*`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlannerConfig {
    /// Whether automatic recovery candidates may be generated.
    automatic_recovery: bool,

    /// Whether adaptation candidates may be generated.
    adaptation_enabled: bool,

    /// Whether migration candidates may be generated.
    migration_enabled: bool,

    /// Whether mitigation candidates may be generated.
    mitigation_enabled: bool,

    /// Whether QEC adaptation candidates may be generated.
    qec_adaptation_enabled: bool,

    /// Whether escalation/abort candidates may be generated.
    escalation_enabled: bool,

    /// Whether preserve-current-execution is preferred.
    prefer_preservation: bool,
}

impl Default for PlannerConfig {
    fn default() -> Self {
        Self {
            automatic_recovery: true,
            adaptation_enabled: true,
            migration_enabled: true,
            mitigation_enabled: true,
            qec_adaptation_enabled: true,
            escalation_enabled: true,
            prefer_preservation: true,
        }
    }
}

impl PlannerConfig {
    /// Creates the default production configuration.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns whether automatic recovery is enabled.
    #[must_use]
    pub const fn automatic_recovery(&self) -> bool {
        self.automatic_recovery
    }

    /// Enables/disables automatic recovery.
    pub const fn with_automatic_recovery(mut self, value: bool) -> Self {
        self.automatic_recovery = value;
        self
    }

    /// Returns whether adaptation is enabled.
    #[must_use]
    pub const fn adaptation_enabled(&self) -> bool {
        self.adaptation_enabled
    }

    /// Enables/disables adaptation.
    pub const fn with_adaptation_enabled(mut self, value: bool) -> Self {
        self.adaptation_enabled = value;
        self
    }

    /// Returns whether migration is enabled.
    #[must_use]
    pub const fn migration_enabled(&self) -> bool {
        self.migration_enabled
    }

    /// Enables/disables migration.
    pub const fn with_migration_enabled(mut self, value: bool) -> Self {
        self.migration_enabled = value;
        self
    }

    /// Returns whether mitigation is enabled.
    #[must_use]
    pub const fn mitigation_enabled(&self) -> bool {
        self.mitigation_enabled
    }

    /// Enables/disables mitigation.
    pub const fn with_mitigation_enabled(mut self, value: bool) -> Self {
        self.mitigation_enabled = value;
        self
    }

    /// Returns whether QEC adaptation is enabled.
    #[must_use]
    pub const fn qec_adaptation_enabled(&self) -> bool {
        self.qec_adaptation_enabled
    }

    /// Enables/disables QEC adaptation.
    pub const fn with_qec_adaptation_enabled(mut self, value: bool) -> Self {
        self.qec_adaptation_enabled = value;
        self
    }

    /// Returns whether escalation is enabled.
    #[must_use]
    pub const fn escalation_enabled(&self) -> bool {
        self.escalation_enabled
    }

    /// Enables/disables escalation.
    pub const fn with_escalation_enabled(mut self, value: bool) -> Self {
        self.escalation_enabled = value;
        self
    }

    /// Returns whether preservation is preferred.
    #[must_use]
    pub const fn prefer_preservation(&self) -> bool {
        self.prefer_preservation
    }

    /// Enables/disables preservation preference.
    pub const fn with_prefer_preservation(mut self, value: bool) -> Self {
        self.prefer_preservation = value;
        self
    }

    /// Validates the planner configuration.
    pub fn validate(&self) -> PlannerResult<()> {
        if !self.automatic_recovery
            && !self.adaptation_enabled
            && !self.migration_enabled
            && !self.mitigation_enabled
            && !self.escalation_enabled
        {
            return Err(PlannerError::InvalidInput {
                message: String::from(
                    "planner has no enabled decision path",
                ),
            });
        }

        Ok(())
    }
}

// =============================================================================
// Plan
// =============================================================================

/// Immutable ordered recovery plan candidate set.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryPlan {
    schema_id: &'static str,
    schema_version: u16,
    planner_version: u32,
    candidates: Vec<PlanCandidate>,
    requires_verification: bool,
}

impl RecoveryPlan {
    fn new(
        candidates: Vec<PlanCandidate>,
        requires_verification: bool,
    ) -> PlannerResult<Self> {
        if candidates.len() > MAX_CANDIDATES {
            return Err(PlannerError::CandidateLimitExceeded {
                requested: candidates.len(),
                maximum: MAX_CANDIDATES,
            });
        }

        Ok(Self {
            schema_id: PLANNER_SCHEMA_ID,
            schema_version: PLANNER_SCHEMA_VERSION,
            planner_version: PLANNER_VERSION,
            candidates,
            requires_verification,
        })
    }

    /// Returns the planner schema identifier.
    #[must_use]
    pub const fn schema_id(&self) -> &'static str {
        self.schema_id
    }

    /// Returns the planner schema version.
    #[must_use]
    pub const fn schema_version(&self) -> u16 {
        self.schema_version
    }

    /// Returns the planner implementation version.
    #[must_use]
    pub const fn planner_version(&self) -> u32 {
        self.planner_version
    }

    /// Returns ordered candidates.
    #[must_use]
    pub fn candidates(&self) -> &[PlanCandidate] {
        &self.candidates
    }

    /// Returns whether downstream verification is mandatory.
    #[must_use]
    pub const fn requires_verification(&self) -> bool {
        self.requires_verification
    }

    /// Returns the highest-priority candidate.
    #[must_use]
    pub fn preferred(&self) -> Option<&PlanCandidate> {
        self.candidates.first()
    }

    /// Returns whether no candidate was generated.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.candidates.is_empty()
    }

    /// Number of candidates.
    #[must_use]
    pub fn len(&self) -> usize {
        self.candidates.len()
    }
}

// =============================================================================
// Planner
// =============================================================================

/// Production recovery planner.
///
/// The planner is intentionally stateless between calls. Durable state belongs
/// to `planning/planner_state.rs` and `state/*`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Planner {
    config: PlannerConfig,
}

impl Default for Planner {
    fn default() -> Self {
        Self::new()
    }
}

impl Planner {
    /// Creates a production planner with the default configuration.
    #[must_use]
    pub fn new() -> Self {
        Self {
            config: PlannerConfig::default(),
        }
    }

    /// Creates a planner with explicit configuration.
    pub fn with_config(config: PlannerConfig) -> PlannerResult<Self> {
        config.validate()?;

        Ok(Self { config })
    }

    /// Returns the planner configuration.
    #[must_use]
    pub const fn config(&self) -> &PlannerConfig {
        &self.config
    }

    /// Generates an immutable recovery plan.
    ///
    /// This method:
    ///
    /// 1. validates normalized input;
    /// 2. generates only semantically plausible candidates;
    /// 3. attaches mandatory downstream requirements;
    /// 4. deduplicates candidates;
    /// 5. orders candidates deterministically;
    /// 6. returns an immutable candidate set.
    ///
    /// It does not execute any candidate.
    pub fn plan(&self, input: &PlannerInput) -> PlannerResult<RecoveryPlan> {
        self.config.validate()?;
        input.validate()?;

        let mut candidates = Vec::new();

        self.generate_preservation(input, &mut candidates)?;
        self.generate_retry(input, &mut candidates)?;
        self.generate_restore(input, &mut candidates)?;
        self.generate_resource_adaptation(input, &mut candidates)?;
        self.generate_implementation_adaptation(input, &mut candidates)?;
        self.generate_qec_adaptation(input, &mut candidates)?;
        self.generate_mitigation(input, &mut candidates)?;
        self.generate_migration(input, &mut candidates)?;
        self.generate_compensation(input, &mut candidates)?;
        self.generate_escalation(input, &mut candidates)?;

        deduplicate_candidates(&mut candidates);

        candidates.sort_by(compare_candidates);

        if candidates.len() > MAX_CANDIDATES {
            return Err(PlannerError::CandidateLimitExceeded {
                requested: candidates.len(),
                maximum: MAX_CANDIDATES,
            });
        }

        Ok(RecoveryPlan::new(
            candidates,
            input.verification_available(),
        )?)
    }

    fn generate_preservation(
        &self,
        input: &PlannerInput,
        candidates: &mut Vec<PlanCandidate>,
    ) -> PlannerResult<()> {
        if !self.config.prefer_preservation {
            return Ok(());
        }

        if input.backend_available()
            && input.resources_feasible()
            && input.mapping_valid()
            && input.routing_valid()
            && input.schedule_valid()
            && input.compilation_valid()
        {
            let candidate = PlanCandidate::new(
                action_id::RETRY,
                PlanPhase::Preserve,
                PlanningReason::TransientFailure,
                10,
            )?
            .with_requirements(&[
                Requirement::PolicyAuthorization,
                Requirement::ExecutionStateValidation,
                Requirement::CapabilityValidation,
                Requirement::ResourceValidation,
                Requirement::SecurityAuthorization,
                Requirement::ProvenanceRetention,
                Requirement::SemanticVerification,
            ])?;

            push_unique(candidates, candidate)?;
        }

        Ok(())
    }

    fn generate_retry(
        &self,
        input: &PlannerInput,
        candidates: &mut Vec<PlanCandidate>,
    ) -> PlannerResult<()> {
        if !self.config.automatic_recovery || !input.retry_safe() {
            return Ok(());
        }

        if !input.backend_available() {
            return Ok(());
        }

        let candidate = PlanCandidate::new(
            action_id::RETRY,
            PlanPhase::Retry,
            if input.previous_recovery_failed() {
                PlanningReason::PreviousRecoveryFailed
            } else {
                PlanningReason::TransientFailure
            },
            20,
        )?
        .with_requirements(&[
            Requirement::PolicyAuthorization,
            Requirement::ExecutionStateValidation,
            Requirement::CapabilityValidation,
            Requirement::ResourceValidation,
            Requirement::SecurityAuthorization,
            Requirement::ProvenanceRetention,
            Requirement::SemanticVerification,
        ])?;

        push_unique(candidates, candidate)?;

        Ok(())
    }

    fn generate_restore(
        &self,
        input: &PlannerInput,
        candidates: &mut Vec<PlanCandidate>,
    ) -> PlannerResult<()> {
        if !self.config.automatic_recovery {
            return Ok(());
        }

        if input.resume_available() && input.checkpoint_available() {
            let candidate = PlanCandidate::new(
                action_id::RESUME,
                PlanPhase::Restore,
                PlanningReason::RestorableExecution,
                30,
            )?
            .with_requirements(&[
                Requirement::PolicyAuthorization,
                Requirement::CheckpointIntegrity,
                Requirement::ExecutionStateValidation,
                Requirement::CapabilityValidation,
                Requirement::ResourceValidation,
                Requirement::SecurityAuthorization,
                Requirement::ProvenanceRetention,
                Requirement::SemanticVerification,
            ])?;

            push_unique(candidates, candidate)?;
        }

        if input.rollback_available() {
            let candidate = PlanCandidate::new(
                action_id::ROLLBACK,
                PlanPhase::Restore,
                PlanningReason::RestorableExecution,
                35,
            )?
            .with_requirements(&[
                Requirement::PolicyAuthorization,
                Requirement::ExecutionStateValidation,
                Requirement::ResourceValidation,
                Requirement::SecurityAuthorization,
                Requirement::ProvenanceRetention,
                Requirement::SemanticVerification,
            ])?;

            push_unique(candidates, candidate)?;
        }

        Ok(())
    }

    fn generate_resource_adaptation(
        &self,
        input: &PlannerInput,
        candidates: &mut Vec<PlanCandidate>,
    ) -> PlannerResult<()> {
        if !self.config.adaptation_enabled {
            return Ok(());
        }

        if !input.resources_feasible() && input.quarantine_possible() {
            let candidate = PlanCandidate::new(
                action_id::QUARANTINE_RESOURCE,
                PlanPhase::AdaptResources,
                PlanningReason::ResourceDegradation,
                40,
            )?
            .with_requirements(&[
                Requirement::PolicyAuthorization,
                Requirement::CapabilityValidation,
                Requirement::ResourceValidation,
                Requirement::SecurityAuthorization,
                Requirement::ProvenanceRetention,
                Requirement::SemanticVerification,
            ])?;

            push_unique(candidates, candidate)?;
        }

        Ok(())
    }

    fn generate_implementation_adaptation(
        &self,
        input: &PlannerInput,
        candidates: &mut Vec<PlanCandidate>,
    ) -> PlannerResult<()> {
        if !self.config.adaptation_enabled {
            return Ok(());
        }

        if !input.mapping_valid() {
            let candidate = PlanCandidate::new(
                action_id::REMAP,
                PlanPhase::AdaptImplementation,
                PlanningReason::PhysicalRealizationInvalid,
                45,
            )?
            .with_requirements(&[
                Requirement::PolicyAuthorization,
                Requirement::CapabilityValidation,
                Requirement::ResourceValidation,
                Requirement::SecurityAuthorization,
                Requirement::ValidMapping,
                Requirement::ProvenanceRetention,
                Requirement::SemanticVerification,
            ])?;

            push_unique(candidates, candidate)?;
        }

        if !input.routing_valid() {
            let candidate = PlanCandidate::new(
                action_id::REROUTE,
                PlanPhase::AdaptImplementation,
                PlanningReason::RoutingInvalid,
                50,
            )?
            .with_requirements(&[
                Requirement::PolicyAuthorization,
                Requirement::CapabilityValidation,
                Requirement::ResourceValidation,
                Requirement::SecurityAuthorization,
                Requirement::ValidRouting,
                Requirement::ProvenanceRetention,
                Requirement::SemanticVerification,
            ])?;

            push_unique(candidates, candidate)?;
        }

        if !input.schedule_valid() {
            let candidate = PlanCandidate::new(
                action_id::RESCHEDULE,
                PlanPhase::AdaptImplementation,
                PlanningReason::SchedulingInvalid,
                55,
            )?
            .with_requirements(&[
                Requirement::PolicyAuthorization,
                Requirement::CapabilityValidation,
                Requirement::ResourceValidation,
                Requirement::SecurityAuthorization,
                Requirement::ValidSchedule,
                Requirement::ProvenanceRetention,
                Requirement::SemanticVerification,
            ])?;

            push_unique(candidates, candidate)?;
        }

        if !input.compilation_valid() || input.capabilities_changed() {
            let candidate = PlanCandidate::new(
                action_id::RECOMPILE,
                PlanPhase::AdaptImplementation,
                if input.capabilities_changed() {
                    PlanningReason::CapabilityChange
                } else {
                    PlanningReason::CompilationInvalid
                },
                60,
            )?
            .with_requirements(&[
                Requirement::PolicyAuthorization,
                Requirement::CapabilityValidation,
                Requirement::ResourceValidation,
                Requirement::SecurityAuthorization,
                Requirement::CompatibleCompilation,
                Requirement::ProvenanceRetention,
                Requirement::SemanticVerification,
            ])?;

            push_unique(candidates, candidate)?;

            let reoptimize = PlanCandidate::new(
                action_id::REOPTIMIZE,
                PlanPhase::AdaptImplementation,
                PlanningReason::CapabilityChange,
                65,
            )?
            .with_requirements(&[
                Requirement::PolicyAuthorization,
                Requirement::CapabilityValidation,
                Requirement::ResourceValidation,
                Requirement::SecurityAuthorization,
                Requirement::CompatibleCompilation,
                Requirement::ProvenanceRetention,
                Requirement::SemanticVerification,
            ])?;

            push_unique(candidates, reoptimize)?;
        }

        Ok(())
    }

    fn generate_qec_adaptation(
        &self,
        input: &PlannerInput,
        candidates: &mut Vec<PlanCandidate>,
    ) -> PlannerResult<()> {
        if !self.config.qec_adaptation_enabled
            || !input.qec_adaptation_possible()
        {
            return Ok(());
        }

        let candidate = PlanCandidate::new(
            action_id::ADAPT_QEC,
            PlanPhase::AdaptQec,
            PlanningReason::QecDegradation,
            70,
        )?
        .with_requirements(&[
            Requirement::PolicyAuthorization,
            Requirement::CapabilityValidation,
            Requirement::ResourceValidation,
            Requirement::SecurityAuthorization,
            Requirement::ValidQecConfiguration,
            Requirement::ProvenanceRetention,
            Requirement::SemanticVerification,
        ])?;

        push_unique(candidates, candidate)?;

        Ok(())
    }

    fn generate_mitigation(
        &self,
        input: &PlannerInput,
        candidates: &mut Vec<PlanCandidate>,
    ) -> PlannerResult<()> {
        if !self.config.mitigation_enabled || !input.mitigation_possible() {
            return Ok(());
        }

        let candidate = PlanCandidate::new(
            action_id::MITIGATE,
            PlanPhase::Mitigate,
            PlanningReason::NoiseMitigation,
            80,
        )?
        .with_requirements(&[
            Requirement::PolicyAuthorization,
            Requirement::CapabilityValidation,
            Requirement::ResourceValidation,
            Requirement::SecurityAuthorization,
            Requirement::ProvenanceRetention,
            Requirement::ResultVerification,
            Requirement::SemanticVerification,
        ])?;

        push_unique(candidates, candidate)?;

        Ok(())
    }

    fn generate_migration(
        &self,
        input: &PlannerInput,
        candidates: &mut Vec<PlanCandidate>,
    ) -> PlannerResult<()> {
        if !self.config.migration_enabled
            || !input.migration_possible()
        {
            return Ok(());
        }

        let reason = if input.capabilities_changed() {
            PlanningReason::CapabilityChange
        } else if !input.backend_available() {
            PlanningReason::BackendUnavailable
        } else {
            PlanningReason::ResourceDegradation
        };

        let candidate = PlanCandidate::new(
            action_id::MIGRATE,
            PlanPhase::Migrate,
            reason,
            90,
        )?
        .with_requirements(&[
            Requirement::PolicyAuthorization,
            Requirement::CapabilityValidation,
            Requirement::ResourceValidation,
            Requirement::CompatibleBackend,
            Requirement::SecurityAuthorization,
            Requirement::ProvenanceRetention,
            Requirement::SemanticVerification,
        ])?;

        push_unique(candidates, candidate)?;

        Ok(())
    }

    fn generate_compensation(
        &self,
        input: &PlannerInput,
        candidates: &mut Vec<PlanCandidate>,
    ) -> PlannerResult<()> {
        if !self.config.automatic_recovery
            || !input.compensation_possible()
        {
            return Ok(());
        }

        let candidate = PlanCandidate::new(
            action_id::COMPENSATE,
            PlanPhase::Restore,
            PlanningReason::PreviousRecoveryFailed,
            100,
        )?
        .with_requirements(&[
            Requirement::PolicyAuthorization,
            Requirement::CapabilityValidation,
            Requirement::ExecutionStateValidation,
            Requirement::ResourceValidation,
            Requirement::SecurityAuthorization,
            Requirement::ProvenanceRetention,
            Requirement::SemanticVerification,
            Requirement::ResultVerification,
        ])?;

        push_unique(candidates, candidate)?;

        Ok(())
    }

    fn generate_escalation(
        &self,
        input: &PlannerInput,
        candidates: &mut Vec<PlanCandidate>,
    ) -> PlannerResult<()> {
        if !self.config.escalation_enabled {
            return Ok(());
        }

        // Escalation is always available as a final safety boundary when
        // automatic recovery is exhausted or unsafe.
        let reason = if input.previous_recovery_failed() {
            PlanningReason::PreviousRecoveryFailed
        } else {
            PlanningReason::NoSafeAutomaticAction
        };

        let candidate = PlanCandidate::new(
            action_id::ESCALATE,
            PlanPhase::Escalate,
            reason,
            1_000,
        )?
        .with_requirements(&[
            Requirement::PolicyAuthorization,
            Requirement::SecurityAuthorization,
            Requirement::ProvenanceRetention,
        ])?;

        push_unique(candidates, candidate)?;

        // Abort is deliberately ordered after escalation.
        let abort = PlanCandidate::new(
            action_id::ABORT,
            PlanPhase::Escalate,
            PlanningReason::NoSafeAutomaticAction,
            1_100,
        )?
        .with_requirements(&[
            Requirement::PolicyAuthorization,
            Requirement::SecurityAuthorization,
            Requirement::ProvenanceRetention,
        ])?;

        push_unique(candidates, abort)?;

        Ok(())
    }
}

// =============================================================================
// Candidate helpers
// =============================================================================

fn validate_action_id(action: &str) -> PlannerResult<()> {
    if action.is_empty() {
        return Err(PlannerError::InvalidActionIdentifier {
            value: String::new(),
        });
    }

    let valid = action
        .bytes()
        .all(|byte| byte.is_ascii_lowercase()
            || byte.is_ascii_digit()
            || byte == b'_');

    if !valid {
        return Err(PlannerError::InvalidActionIdentifier {
            value: String::from(action),
        });
    }

    Ok(())
}

fn push_unique(
    candidates: &mut Vec<PlanCandidate>,
    candidate: PlanCandidate,
) -> PlannerResult<()> {
    if candidates.len() >= MAX_CANDIDATES {
        return Err(PlannerError::CandidateLimitExceeded {
            requested: candidates.len().saturating_add(1),
            maximum: MAX_CANDIDATES,
        });
    }

    if candidates
        .iter()
        .any(|existing| existing.action() == candidate.action())
    {
        return Ok(());
    }

    candidates.push(candidate);

    Ok(())
}

fn deduplicate_candidates(candidates: &mut Vec<PlanCandidate>) {
    let mut retained = Vec::with_capacity(candidates.len());

    for candidate in candidates.drain(..) {
        if !retained
            .iter()
            .any(|existing: &PlanCandidate| {
                existing.action() == candidate.action()
            })
        {
            retained.push(candidate);
        }
    }

    *candidates = retained;
}

fn compare_candidates(
    left: &PlanCandidate,
    right: &PlanCandidate,
) -> Ordering {
    left.priority()
        .cmp(&right.priority())
        .then_with(|| left.phase().cmp(&right.phase()))
        .then_with(|| left.action().cmp(right.action()))
        .then_with(|| left.reason().as_str().cmp(right.reason().as_str()))
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_input_is_conservative() {
        let input = PlannerInput::new();

        assert!(!input.retry_safe());
        assert!(!input.migration_possible());
        assert!(!input.mitigation_possible());
        assert!(!input.qec_adaptation_possible());
    }

    #[test]
    fn default_planner_produces_safe_fallback() {
        let planner = Planner::new();
        let input = PlannerInput::new();

        let plan = planner.plan(&input).expect("planning must succeed");

        assert!(!plan.is_empty());
        assert_eq!(plan.schema_id(), PLANNER_SCHEMA_ID);
        assert_eq!(plan.schema_version(), PLANNER_SCHEMA_VERSION);
        assert_eq!(plan.planner_version(), PLANNER_VERSION);

        assert_eq!(
            plan.candidates()
                .last()
                .map(PlanCandidate::action),
            Some(action_id::ABORT)
        );
    }

    #[test]
    fn safe_retry_is_generated() {
        let planner = Planner::new();

        let input = PlannerInput::new()
            .with_execution_active(true)
            .with_retry_safe(true);

        let plan = planner.plan(&input).expect("planning must succeed");

        assert!(
            plan.candidates()
                .iter()
                .any(|candidate| candidate.action() == action_id::RETRY)
        );
    }

    #[test]
    fn checkpoint_resume_is_generated() {
        let planner = Planner::new();

        let input = PlannerInput::new()
            .with_execution_active(true)
            .with_checkpoint_available(true)
            .with_resume_available(true);

        let plan = planner.plan(&input).expect("planning must succeed");

        assert!(
            plan.candidates()
                .iter()
                .any(|candidate| candidate.action() == action_id::RESUME)
        );
    }

    #[test]
    fn invalid_mapping_generates_remap() {
        let planner = Planner::new();

        let input = PlannerInput::new()
            .with_mapping_valid(false);

        let plan = planner.plan(&input).expect("planning must succeed");

        assert!(
            plan.candidates()
                .iter()
                .any(|candidate| candidate.action() == action_id::REMAP)
        );
    }

    #[test]
    fn invalid_routing_generates_reroute() {
        let planner = Planner::new();

        let input = PlannerInput::new()
            .with_routing_valid(false);

        let plan = planner.plan(&input).expect("planning must succeed");

        assert!(
            plan.candidates()
                .iter()
                .any(|candidate| candidate.action() == action_id::REROUTE)
        );
    }

    #[test]
    fn invalid_schedule_generates_reschedule() {
        let planner = Planner::new();

        let input = PlannerInput::new()
            .with_schedule_valid(false);

        let plan = planner.plan(&input).expect("planning must succeed");

        assert!(
            plan.candidates()
                .iter()
                .any(|candidate| candidate.action() == action_id::RESCHEDULE)
        );
    }

    #[test]
    fn capability_change_generates_recompilation() {
        let planner = Planner::new();

        let input = PlannerInput::new()
            .with_capabilities_changed(true);

        let plan = planner.plan(&input).expect("planning must succeed");

        assert!(
            plan.candidates()
                .iter()
                .any(|candidate| candidate.action() == action_id::RECOMPILE)
        );
    }

    #[test]
    fn migration_is_generated_when_available() {
        let planner = Planner::new();

        let input = PlannerInput::new()
            .with_backend_available(false)
            .with_migration_possible(true);

        let plan = planner.plan(&input).expect("planning must succeed");

        assert!(
            plan.candidates()
                .iter()
                .any(|candidate| candidate.action() == action_id::MIGRATE)
        );
    }

    #[test]
    fn mitigation_is_generated_when_available() {
        let planner = Planner::new();

        let input = PlannerInput::new()
            .with_mitigation_possible(true);

        let plan = planner.plan(&input).expect("planning must succeed");

        assert!(
            plan.candidates()
                .iter()
                .any(|candidate| candidate.action() == action_id::MITIGATE)
        );
    }

    #[test]
    fn qec_adaptation_is_generated_when_available() {
        let planner = Planner::new();

        let input = PlannerInput::new()
            .with_qec_adaptation_possible(true);

        let plan = planner.plan(&input).expect("planning must succeed");

        assert!(
            plan.candidates()
                .iter()
                .any(|candidate| candidate.action() == action_id::ADAPT_QEC)
        );
    }

    #[test]
    fn candidates_are_deterministically_ordered() {
        let planner = Planner::new();

        let input = PlannerInput::new()
            .with_retry_safe(true)
            .with_mapping_valid(false)
            .with_routing_valid(false)
            .with_schedule_valid(false)
            .with_capabilities_changed(true)
            .with_migration_possible(true)
            .with_mitigation_possible(true)
            .with_qec_adaptation_possible(true);

        let first = planner
            .plan(&input)
            .expect("first planning pass must succeed");

        let second = planner
            .plan(&input)
            .expect("second planning pass must succeed");

        assert_eq!(first, second);
    }

    #[test]
    fn candidate_actions_are_unique() {
        let planner = Planner::new();

        let input = PlannerInput::new()
            .with_retry_safe(true)
            .with_backend_available(false)
            .with_migration_possible(true);

        let plan = planner.plan(&input).expect("planning must succeed");

        for (index, candidate) in plan.candidates().iter().enumerate() {
            assert!(
                !plan.candidates()[index + 1..]
                    .iter()
                    .any(|other| other.action() == candidate.action()),
                "duplicate action found: {}",
                candidate.action()
            );
        }
    }

    #[test]
    fn no_unsafe_code_is_required() {
        // This test exists as documentation of the module's compiler-level
        // safety boundary. The actual guarantee is supplied by:
        //
        //     #![forbid(unsafe_code)]
        //
        // above.
        assert_eq!(PLANNER_SCHEMA_VERSION, 1);
    }

    #[test]
    fn configuration_with_no_paths_is_rejected() {
        let config = PlannerConfig::new()
            .with_automatic_recovery(false)
            .with_adaptation_enabled(false)
            .with_migration_enabled(false)
            .with_mitigation_enabled(false)
            .with_escalation_enabled(false);

        assert!(Planner::with_config(config).is_err());
    }

    #[test]
    fn escalation_is_always_after_automatic_candidates() {
        let planner = Planner::new();

        let input = PlannerInput::new()
            .with_retry_safe(true)
            .with_mitigation_possible(true);

        let plan = planner.plan(&input).expect("planning must succeed");

        let escalation_index = plan
            .candidates()
            .iter()
            .position(|candidate| candidate.action() == action_id::ESCALATE)
            .expect("escalation must exist");

        let retry_index = plan
            .candidates()
            .iter()
            .position(|candidate| candidate.action() == action_id::RETRY)
            .expect("retry must exist");

        assert!(retry_index < escalation_index);
    }
}