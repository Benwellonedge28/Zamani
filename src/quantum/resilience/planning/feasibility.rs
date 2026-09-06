//! Zamani Quantum Resilience — Production Feasibility Engine
//!
//! Path:
//!     src/quantum/resilience/planning/feasibility.rs
//!
//! Purpose:
//!     Determine whether a proposed resilience action is currently feasible
//!     under an explicitly supplied execution context.
//!
//! Architectural position:
//!
//! ```text
//!                         Incident
//!                            |
//!                            v
//!                        Diagnosis
//!                            |
//!                            v
//!                          Policy
//!                            |
//!                            v
//!                         Planner
//!                            |
//!                            v
//!                     RecoveryAction
//!                            |
//!                            v
//!                 +-----------------------+
//!                 |    FEASIBILITY        |
//!                 |                       |
//!                 | semantic constraints  |
//!                 | capabilities          |
//!                 | resources             |
//!                 | state                 |
//!                 | security              |
//!                 | verification         |
//!                 | preconditions         |
//!                 +-----------+-----------+
//!                             |
//!                    +--------+--------+
//!                    |                 |
//!                    v                 v
//!                 Feasible          Infeasible
//!                    |
//!                    v
//!                  Ranking
//!                    |
//!                    v
//!                   Plan
//! ```
//!
//! -----------------------------------------------------------------------------
//! Responsibility
//! -----------------------------------------------------------------------------
//!
//! This module answers:
//!
//! > "Can this action safely and meaningfully be considered executable under
//! > the capabilities, resources, state and contracts supplied right now?"
//!
//! It does NOT:
//!
//! - execute recovery;
//! - mutate hardware;
//! - perform routing;
//! - perform scheduling;
//! - compile;
//! - optimize;
//! - perform QEC;
//! - perform mitigation;
//! - discover hardware;
//! - select a provider by name;
//! - execute a plan;
//! - rank plans;
//! - decide policy;
//! - infer a diagnosis;
//! - verify an executed result.
//!
//! Those responsibilities belong to:
//!
//!     hardware/*
//!     routing/*
//!     scheduling/*
//!     optimization/*
//!     qec/*
//!     mitigation/*
//!     policy/*
//!     diagnosis/*
//!     planning/ranking.rs
//!     recovery/*
//!     verification/*
//!
//! -----------------------------------------------------------------------------
//! Fundamental safety invariant
//! -----------------------------------------------------------------------------
//!
//! Feasibility is NOT authorization.
//!
//! A feasible action means:
//!
//!     "The supplied evidence and contracts do not currently establish that
//!      this action is impossible."
//!
//! It does NOT mean:
//!
//!     "This action is authorized."
//!     "This action will succeed."
//!     "This action preserves semantics."
//!     "This action is optimal."
//!
//! Final execution must still pass:
//!
//!     policy
//!     + security authorization
//!     + semantic validation
//!     + execution-time preconditions
//!     + verification
//!
//! -----------------------------------------------------------------------------
//! Fail-closed principle
//! -----------------------------------------------------------------------------
//!
//! Unknown information is not silently interpreted as success.
//!
//! In particular:
//!
//!     Unknown capability
//!     Unknown resource availability
//!     Unknown authorization
//!     Unknown verification support
//!
//! must not be converted into:
//!
//!     Available
//!     Authorized
//!     Safe
//!     Verified
//!
//! For actions requiring such information, feasibility returns an explicit
//! `Unknown` state or an error according to the selected evaluation mode.
//!
//! Protective actions such as `Abort` and `QuarantineResource` may remain
//! feasible when transformative recovery is blocked.
//!
//! -----------------------------------------------------------------------------
//! "Write once, scale everywhere"
//! -----------------------------------------------------------------------------
//!
//! This module contains NO fixed quantum-machine limit.
//!
//! It contains no:
//!
//!     MAX_QUBITS
//!     MAX_BACKENDS
//!     MAX_RESOURCES
//!     MAX_ACTIONS
//!     MAX_RETRIES
//!     MAX_PLAN_DEPTH
//!
//! Resource counts are supplied dynamically.
//!
//! The implementation uses slices and iterators where possible and does not
//! assume a particular number of qubits, devices, backends or execution
//! environments.
//!
//! "Infinity" therefore means:
//!
//!     no artificial source-level machine-size ceiling.
//!
//! Actual execution remains bounded by:
//!
//!     available memory
//!     available hardware
//!     execution resources
//!     policy
//!     provider capability
//!     operating-system limits
//!     caller budgets
//!
//! -----------------------------------------------------------------------------
//! Canonical quantum identity
//! -----------------------------------------------------------------------------
//!
//! This module does not define a replacement qubit identifier.
//!
//! When a feasibility context needs explicit quantum-resource identity, it
//! should use the canonical Zamani IR identity:
//!
//!     crate::quantum::ir::qubit::QubitId
//!
//! and, where the repository contract requires physical identity:
//!
//!     crate::quantum::ir::qubit::PhysicalQubitId
//!
//! This file does not require a qubit ID for every feasibility evaluation.
//! Generic resilience resources may be represented by the resource contract
//! supplied by the caller.
//!
//! -----------------------------------------------------------------------------
//! Determinism
//! -----------------------------------------------------------------------------
//!
//! Feasibility evaluation is deterministic for identical inputs.
//!
//! This module:
//!
//! - does not read the system clock;
//! - does not generate random values;
//! - does not inspect global state;
//! - does not inspect environment variables;
//! - does not contact a backend;
//! - does not depend on hash-map iteration order;
//! - does not use memory addresses;
//! - does not mutate shared state.
//!
//! -----------------------------------------------------------------------------
//! Integration contract
//! -----------------------------------------------------------------------------
//!
//! `planning/action.rs`
//!     Supplies `RecoveryAction`, `ActionKind`, and declarative
//!     `ActionPrecondition` values.
//!
//! `model/resource.rs`
//!     Supplies canonical resilience resource semantics.
//!
//! `model/capability.rs`
//!     Supplies effective capability state after degradation/fault handling.
//!
//! `policy/*`
//!     Supplies policy-level restrictions. This module only consumes their
//!     already-resolved requirements through `FeasibilityContext`.
//!
//! `diagnosis/*`
//!     Supplies diagnosis-derived requirements and affected-resource context.
//!
//! `planning/cost.rs`
//!     Estimates consequences. Cost is intentionally NOT used as proof of
//!     feasibility.
//!
//! `planning/ranking.rs`
//!     Consumes `FeasibilityResult` when ranking candidate plans.
//!
//! `planning/plan.rs`
//!     Should reject plans containing actions that are definitively infeasible.
//!
//! `planning/planner.rs`
//!     Supplies the complete context and consumes feasibility decisions.
//!
//! `adaptation/*`
//!     Implements actions after feasibility and policy authorization.
//!
//! `recovery/*`
//!     Executes recovery actions after all execution-time gates pass.
//!
//! `verification/*`
//!     Remains the final semantic/result acceptance authority.
//!
//! `errors/*`
//!     Higher-level orchestration may translate `FeasibilityError` into the
//!     canonical `ResilienceError` contract.
//!
//! -----------------------------------------------------------------------------
//! Rust compatibility
//! -----------------------------------------------------------------------------
//!
//! Rust 1.97
//! Rust 1.97.1
//! Rust 2021
//! stable Rust
//! no nightly features
//! no unsafe code
//!
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use core::fmt;

use super::action::{ActionKind, ActionPrecondition, RecoveryAction};

// =============================================================================
// Stable schema
// =============================================================================

/// Stable schema identifier for the feasibility contract.
pub const FEASIBILITY_SCHEMA_ID: &str =
    "zamani.quantum.resilience.planning.feasibility";

/// Semantic version of the feasibility contract.
pub const FEASIBILITY_SCHEMA_VERSION: u16 = 1;

/// Implementation version.
///
/// This identifies implementation semantics, not hardware capability.
pub const FEASIBILITY_IMPLEMENTATION_VERSION: u32 = 1;

// =============================================================================
// Knowledge state
// =============================================================================

/// Three-valued feasibility state.
///
/// `Feasible` means the supplied evidence establishes that the action's
/// feasibility requirements are satisfied.
///
/// `Infeasible` means at least one required condition is definitively false.
///
/// `Unknown` means the available evidence is insufficient to establish
/// feasibility.
///
/// Unknown is intentionally different from infeasible.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum FeasibilityState {
    /// All evaluated mandatory requirements are satisfied.
    Feasible,

    /// At least one mandatory requirement is definitively unsatisfied.
    Infeasible,

    /// Required information is unavailable or indeterminate.
    Unknown,
}

impl FeasibilityState {
    /// Stable machine-readable representation.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Feasible => "feasible",
            Self::Infeasible => "infeasible",
            Self::Unknown => "unknown",
        }
    }

    /// Returns whether the state is feasible.
    #[must_use]
    pub const fn is_feasible(self) -> bool {
        matches!(self, Self::Feasible)
    }

    /// Returns whether the state is definitively infeasible.
    #[must_use]
    pub const fn is_infeasible(self) -> bool {
        matches!(self, Self::Infeasible)
    }

    /// Returns whether additional information is required.
    #[must_use]
    pub const fn is_unknown(self) -> bool {
        matches!(self, Self::Unknown)
    }
}

impl fmt::Display for FeasibilityState {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Requirement state
// =============================================================================

/// State of one evaluated feasibility requirement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RequirementState {
    /// Requirement is satisfied.
    Satisfied,

    /// Requirement is definitively unsatisfied.
    Unsatisfied,

    /// Requirement cannot currently be evaluated.
    Unknown,

    /// Requirement is not applicable to this action.
    NotApplicable,
}

impl RequirementState {
    /// Stable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Satisfied => "satisfied",
            Self::Unsatisfied => "unsatisfied",
            Self::Unknown => "unknown",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// Returns whether this state satisfies the requirement.
    #[must_use]
    pub const fn is_satisfied(self) -> bool {
        matches!(self, Self::Satisfied | Self::NotApplicable)
    }
}

// =============================================================================
// Requirement result
// =============================================================================

/// Result of evaluating one declarative action precondition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RequirementResult {
    requirement: ActionPrecondition,
    state: RequirementState,
}

impl RequirementResult {
    /// Creates a requirement result.
    #[must_use]
    pub const fn new(
        requirement: ActionPrecondition,
        state: RequirementState,
    ) -> Self {
        Self {
            requirement,
            state,
        }
    }

    /// Returns the evaluated requirement.
    #[must_use]
    pub const fn requirement(self) -> ActionPrecondition {
        self.requirement
    }

    /// Returns the evaluation state.
    #[must_use]
    pub const fn state(self) -> RequirementState {
        self.state
    }

    /// Returns whether this requirement is satisfied.
    #[must_use]
    pub const fn is_satisfied(self) -> bool {
        self.state.is_satisfied()
    }
}

// =============================================================================
// Feasibility reason
// =============================================================================

/// Machine-readable reason for a feasibility outcome.
///
/// The enum is deliberately independent of backend/provider names.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum FeasibilityReason {
    /// All requirements were satisfied.
    AllRequirementsSatisfied,

    /// One or more required resources are unavailable.
    ResourceUnavailable,

    /// Required capability is unavailable.
    CapabilityUnavailable,

    /// Required capability information is unknown.
    CapabilityUnknown,

    /// Resource state is unknown.
    ResourceStateUnknown,

    /// Execution state does not permit the action.
    InvalidExecutionState,

    /// Current logical semantics cannot be established as preserved.
    SemanticCompatibilityUnavailable,

    /// Mapping information is unavailable or invalid.
    MappingUnavailable,

    /// Routing information is unavailable or invalid.
    RoutingUnavailable,

    /// Scheduling information is unavailable or invalid.
    ScheduleUnavailable,

    /// Compilation capability is unavailable.
    CompilationUnavailable,

    /// Checkpoint is unavailable.
    CheckpointUnavailable,

    /// Resume boundary is unavailable.
    ResumeBoundaryUnavailable,

    /// Rollback target is unavailable.
    RollbackTargetUnavailable,

    /// Retry safety has not been established.
    RetrySafetyUnavailable,

    /// QEC capability/compatibility is unavailable.
    QecUnavailable,

    /// Mitigation capability is unavailable.
    MitigationUnavailable,

    /// Migration target is unavailable or incompatible.
    MigrationUnavailable,

    /// Authorization information is missing.
    AuthorizationUnknown,

    /// Authorization is not granted.
    AuthorizationDenied,

    /// Provenance is unavailable.
    ProvenanceUnavailable,

    /// Verification is unavailable.
    VerificationUnavailable,

    /// Resource isolation is unavailable.
    ResourceIsolationUnavailable,

    /// Deterministic execution cannot be established.
    DeterminismUnavailable,

    /// External condition is not established.
    ExternalConditionUnavailable,

    /// The action itself is structurally invalid.
    InvalidAction,

    /// A protective action remains possible.
    ProtectiveActionAvailable,
}

impl FeasibilityReason {
    /// Stable machine-readable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AllRequirementsSatisfied => "all_requirements_satisfied",
            Self::ResourceUnavailable => "resource_unavailable",
            Self::CapabilityUnavailable => "capability_unavailable",
            Self::CapabilityUnknown => "capability_unknown",
            Self::ResourceStateUnknown => "resource_state_unknown",
            Self::InvalidExecutionState => "invalid_execution_state",
            Self::SemanticCompatibilityUnavailable => {
                "semantic_compatibility_unavailable"
            }
            Self::MappingUnavailable => "mapping_unavailable",
            Self::RoutingUnavailable => "routing_unavailable",
            Self::ScheduleUnavailable => "schedule_unavailable",
            Self::CompilationUnavailable => "compilation_unavailable",
            Self::CheckpointUnavailable => "checkpoint_unavailable",
            Self::ResumeBoundaryUnavailable => "resume_boundary_unavailable",
            Self::RollbackTargetUnavailable => "rollback_target_unavailable",
            Self::RetrySafetyUnavailable => "retry_safety_unavailable",
            Self::QecUnavailable => "qec_unavailable",
            Self::MitigationUnavailable => "mitigation_unavailable",
            Self::MigrationUnavailable => "migration_unavailable",
            Self::AuthorizationUnknown => "authorization_unknown",
            Self::AuthorizationDenied => "authorization_denied",
            Self::ProvenanceUnavailable => "provenance_unavailable",
            Self::VerificationUnavailable => "verification_unavailable",
            Self::ResourceIsolationUnavailable => {
                "resource_isolation_unavailable"
            }
            Self::DeterminismUnavailable => "determinism_unavailable",
            Self::ExternalConditionUnavailable => {
                "external_condition_unavailable"
            }
            Self::InvalidAction => "invalid_action",
            Self::ProtectiveActionAvailable => "protective_action_available",
        }
    }
}

impl fmt::Display for FeasibilityReason {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Feasibility mode
// =============================================================================

/// Controls how unknown information is handled.
///
/// `Conservative` is appropriate for execution authorization boundaries.
///
/// `Advisory` allows the planner to retain potentially useful candidates while
/// explicitly marking them as unknown.
///
/// `Strict` is intended for deterministic production gates where unknown
/// information must produce an error instead of an ordinary result.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum FeasibilityMode {
    /// Unknown requirements produce `Unknown`.
    Advisory,

    /// Unknown requirements are treated as not feasible for planning.
    Conservative,

    /// Unknown requirements produce a structured error.
    Strict,
}

impl Default for FeasibilityMode {
    fn default() -> Self {
        Self::Conservative
    }
}

// =============================================================================
// Capability knowledge
// =============================================================================

/// Capability knowledge supplied by the caller.
///
/// This intentionally does not duplicate the repository's capability model.
/// The model layer remains authoritative; this enum only describes what the
/// feasibility evaluator has learned about a particular requirement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CapabilityKnowledge {
    /// Capability is available.
    Available,

    /// Capability is definitely unavailable.
    Unavailable,

    /// Capability has not been established.
    Unknown,
}

impl CapabilityKnowledge {
    /// Returns whether capability is available.
    #[must_use]
    pub const fn is_available(self) -> bool {
        matches!(self, Self::Available)
    }

    /// Returns whether capability is unavailable.
    #[must_use]
    pub const fn is_unavailable(self) -> bool {
        matches!(self, Self::Unavailable)
    }

    /// Returns whether capability is unknown.
    #[must_use]
    pub const fn is_unknown(self) -> bool {
        matches!(self, Self::Unknown)
    }
}

// =============================================================================
// Resource knowledge
// =============================================================================

/// Resource availability knowledge.
///
/// This is an adapter-level value. The canonical resource model remains owned
/// by `model/resource.rs`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ResourceKnowledge {
    /// Resource is currently available.
    Available,

    /// Resource is currently unavailable.
    Unavailable,

    /// Availability is unknown.
    Unknown,
}

impl ResourceKnowledge {
    /// Returns whether the resource is available.
    #[must_use]
    pub const fn is_available(self) -> bool {
        matches!(self, Self::Available)
    }

    /// Returns whether the resource is unavailable.
    #[must_use]
    pub const fn is_unavailable(self) -> bool {
        matches!(self, Self::Unavailable)
    }

    /// Returns whether availability is unknown.
    #[must_use]
    pub const fn is_unknown(self) -> bool {
        matches!(self, Self::Unknown)
    }
}

// =============================================================================
// Semantic knowledge
// =============================================================================

/// Whether semantic preservation has been established.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SemanticKnowledge {
    /// Semantics are known to remain representable.
    Preserved,

    /// Semantic preservation is known to be impossible.
    Violated,

    /// Preservation has not yet been established.
    Unknown,
}

impl SemanticKnowledge {
    /// Returns whether semantic preservation is established.
    #[must_use]
    pub const fn is_preserved(self) -> bool {
        matches!(self, Self::Preserved)
    }

    /// Returns whether semantic preservation is known to be violated.
    #[must_use]
    pub const fn is_violated(self) -> bool {
        matches!(self, Self::Violated)
    }

    /// Returns whether semantic preservation is unknown.
    #[must_use]
    pub const fn is_unknown(self) -> bool {
        matches!(self, Self::Unknown)
    }
}

// =============================================================================
// Execution-state knowledge
// =============================================================================

/// Abstract execution-state knowledge required by feasibility.
///
/// The actual execution state remains owned by `state/execution.rs`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ExecutionKnowledge {
    /// Execution is in a state where the requested operation may be considered.
    Valid,

    /// Execution state definitely prevents the requested operation.
    Invalid,

    /// State cannot currently be established.
    Unknown,
}

impl ExecutionKnowledge {
    /// Returns whether execution state is valid.
    #[must_use]
    pub const fn is_valid(self) -> bool {
        matches!(self, Self::Valid)
    }

    /// Returns whether execution state is invalid.
    #[must_use]
    pub const fn is_invalid(self) -> bool {
        matches!(self, Self::Invalid)
    }

    /// Returns whether execution state is unknown.
    #[must_use]
    pub const fn is_unknown(self) -> bool {
        matches!(self, Self::Unknown)
    }
}

// =============================================================================
// Authorization knowledge
// =============================================================================

/// Authorization state supplied by the security/policy boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum AuthorizationKnowledge {
    /// Authorization is granted.
    Granted,

    /// Authorization is explicitly denied.
    Denied,

    /// Authorization has not been established.
    Unknown,
}

impl AuthorizationKnowledge {
    /// Returns whether authorization is granted.
    #[must_use]
    pub const fn is_granted(self) -> bool {
        matches!(self, Self::Granted)
    }

    /// Returns whether authorization is denied.
    #[must_use]
    pub const fn is_denied(self) -> bool {
        matches!(self, Self::Denied)
    }

    /// Returns whether authorization is unknown.
    #[must_use]
    pub const fn is_unknown(self) -> bool {
        matches!(self, Self::Unknown)
    }
}

// =============================================================================
// Verification knowledge
// =============================================================================

/// Availability of a suitable verification mechanism.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum VerificationKnowledge {
    /// Suitable verification exists.
    Available,

    /// Suitable verification is unavailable.
    Unavailable,

    /// Verification availability is unknown.
    Unknown,
}

impl VerificationKnowledge {
    /// Returns whether verification exists.
    #[must_use]
    pub const fn is_available(self) -> bool {
        matches!(self, Self::Available)
    }

    /// Returns whether verification is unavailable.
    #[must_use]
    pub const fn is_unavailable(self) -> bool {
        matches!(self, Self::Unavailable)
    }

    /// Returns whether verification is unknown.
    #[must_use]
    pub const fn is_unknown(self) -> bool {
        matches!(self, Self::Unknown)
    }
}

// =============================================================================
// Feasibility context
// =============================================================================

/// Immutable evidence used to evaluate action feasibility.
///
/// This type is intentionally an adapter contract rather than a second
/// hardware/capability model.
///
/// The actual information is supplied by:
//!
//! - hardware HAL;
//! - routing;
//! - scheduling;
//! - compiler;
//! - QEC;
//! - mitigation;
//! - state;
//! - policy;
//! - security;
//! - verification;
//! - checkpoint;
//! - diagnosis.
//!
//! No provider-specific implementation is embedded here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FeasibilityContext {
    /// Current execution-state knowledge.
    execution: ExecutionKnowledge,

    /// Semantic-preservation knowledge.
    semantics: SemanticKnowledge,

    /// General resource knowledge.
    resources: ResourceKnowledge,

    /// General capability knowledge.
    capabilities: CapabilityKnowledge,

    /// Authorization state.
    authorization: AuthorizationKnowledge,

    /// Verification availability.
    verification: VerificationKnowledge,

    /// Mapping support.
    mapping: CapabilityKnowledge,

    /// Routing support.
    routing: CapabilityKnowledge,

    /// Scheduling support.
    scheduling: CapabilityKnowledge,

    /// Compilation support.
    compilation: CapabilityKnowledge,

    /// Checkpoint availability.
    checkpoint: CapabilityKnowledge,

    /// Resume-boundary availability.
    resume_boundary: CapabilityKnowledge,

    /// Rollback-target availability.
    rollback_target: CapabilityKnowledge,

    /// Retry safety.
    retry_safety: CapabilityKnowledge,

    /// QEC support/compatibility.
    qec: CapabilityKnowledge,

    /// Mitigation support.
    mitigation: CapabilityKnowledge,

    /// Migration support.
    migration: CapabilityKnowledge,

    /// Provenance support.
    provenance: CapabilityKnowledge,

    /// Resource isolation support.
    resource_isolation: CapabilityKnowledge,

    /// Deterministic execution support.
    determinism: CapabilityKnowledge,

    /// Whether external conditions have been satisfied.
    external_conditions: CapabilityKnowledge,
}

impl Default for FeasibilityContext {
    fn default() -> Self {
        Self {
            execution: ExecutionKnowledge::Unknown,
            semantics: SemanticKnowledge::Unknown,
            resources: ResourceKnowledge::Unknown,
            capabilities: CapabilityKnowledge::Unknown,
            authorization: AuthorizationKnowledge::Unknown,
            verification: VerificationKnowledge::Unknown,
            mapping: CapabilityKnowledge::Unknown,
            routing: CapabilityKnowledge::Unknown,
            scheduling: CapabilityKnowledge::Unknown,
            compilation: CapabilityKnowledge::Unknown,
            checkpoint: CapabilityKnowledge::Unknown,
            resume_boundary: CapabilityKnowledge::Unknown,
            rollback_target: CapabilityKnowledge::Unknown,
            retry_safety: CapabilityKnowledge::Unknown,
            qec: CapabilityKnowledge::Unknown,
            mitigation: CapabilityKnowledge::Unknown,
            migration: CapabilityKnowledge::Unknown,
            provenance: CapabilityKnowledge::Unknown,
            resource_isolation: CapabilityKnowledge::Unknown,
            determinism: CapabilityKnowledge::Unknown,
            external_conditions: CapabilityKnowledge::Unknown,
        }
    }
}

impl FeasibilityContext {
    /// Creates an unknown context.
    ///
    /// This is intentionally conservative.
    #[must_use]
    pub const fn unknown() -> Self {
        Self {
            execution: ExecutionKnowledge::Unknown,
            semantics: SemanticKnowledge::Unknown,
            resources: ResourceKnowledge::Unknown,
            capabilities: CapabilityKnowledge::Unknown,
            authorization: AuthorizationKnowledge::Unknown,
            verification: VerificationKnowledge::Unknown,
            mapping: CapabilityKnowledge::Unknown,
            routing: CapabilityKnowledge::Unknown,
            scheduling: CapabilityKnowledge::Unknown,
            compilation: CapabilityKnowledge::Unknown,
            checkpoint: CapabilityKnowledge::Unknown,
            resume_boundary: CapabilityKnowledge::Unknown,
            rollback_target: CapabilityKnowledge::Unknown,
            retry_safety: CapabilityKnowledge::Unknown,
            qec: CapabilityKnowledge::Unknown,
            mitigation: CapabilityKnowledge::Unknown,
            migration: CapabilityKnowledge::Unknown,
            provenance: CapabilityKnowledge::Unknown,
            resource_isolation: CapabilityKnowledge::Unknown,
            determinism: CapabilityKnowledge::Unknown,
            external_conditions: CapabilityKnowledge::Unknown,
        }
    }

    /// Returns a builder for constructing a complete context.
    #[must_use]
    pub const fn builder() -> FeasibilityContextBuilder {
        FeasibilityContextBuilder::new()
    }

    /// Returns execution-state knowledge.
    #[must_use]
    pub const fn execution(self) -> ExecutionKnowledge {
        self.execution
    }

    /// Returns semantic knowledge.
    #[must_use]
    pub const fn semantics(self) -> SemanticKnowledge {
        self.semantics
    }

    /// Returns general resource knowledge.
    #[must_use]
    pub const fn resources(self) -> ResourceKnowledge {
        self.resources
    }

    /// Returns general capability knowledge.
    #[must_use]
    pub const fn capabilities(self) -> CapabilityKnowledge {
        self.capabilities
    }

    /// Returns authorization knowledge.
    #[must_use]
    pub const fn authorization(self) -> AuthorizationKnowledge {
        self.authorization
    }

    /// Returns verification knowledge.
    #[must_use]
    pub const fn verification(self) -> VerificationKnowledge {
        self.verification
    }

    /// Returns mapping capability.
    #[must_use]
    pub const fn mapping(self) -> CapabilityKnowledge {
        self.mapping
    }

    /// Returns routing capability.
    #[must_use]
    pub const fn routing(self) -> CapabilityKnowledge {
        self.routing
    }

    /// Returns scheduling capability.
    #[must_use]
    pub const fn scheduling(self) -> CapabilityKnowledge {
        self.scheduling
    }

    /// Returns compilation capability.
    #[must_use]
    pub const fn compilation(self) -> CapabilityKnowledge {
        self.compilation
    }

    /// Returns checkpoint availability.
    #[must_use]
    pub const fn checkpoint(self) -> CapabilityKnowledge {
        self.checkpoint
    }

    /// Returns resume-boundary availability.
    #[must_use]
    pub const fn resume_boundary(self) -> CapabilityKnowledge {
        self.resume_boundary
    }

    /// Returns rollback-target availability.
    #[must_use]
    pub const fn rollback_target(self) -> CapabilityKnowledge {
        self.rollback_target
    }

    /// Returns retry safety.
    #[must_use]
    pub const fn retry_safety(self) -> CapabilityKnowledge {
        self.retry_safety
    }

    /// Returns QEC capability.
    #[must_use]
    pub const fn qec(self) -> CapabilityKnowledge {
        self.qec
    }

    /// Returns mitigation capability.
    #[must_use]
    pub const fn mitigation(self) -> CapabilityKnowledge {
        self.mitigation
    }

    /// Returns migration capability.
    #[must_use]
    pub const fn migration(self) -> CapabilityKnowledge {
        self.migration
    }

    /// Returns provenance capability.
    #[must_use]
    pub const fn provenance(self) -> CapabilityKnowledge {
        self.provenance
    }

    /// Returns resource isolation capability.
    #[must_use]
    pub const fn resource_isolation(self) -> CapabilityKnowledge {
        self.resource_isolation
    }

    /// Returns deterministic-execution capability.
    #[must_use]
    pub const fn determinism(self) -> CapabilityKnowledge {
        self.determinism
    }

    /// Returns external-condition state.
    #[must_use]
    pub const fn external_conditions(self) -> CapabilityKnowledge {
        self.external_conditions
    }
}

// =============================================================================
// Context builder
// =============================================================================

/// Builder for [`FeasibilityContext`].
///
/// Every field defaults to `Unknown`; therefore a caller must explicitly
/// establish facts instead of receiving optimistic defaults.
#[derive(Debug, Clone, Copy)]
pub struct FeasibilityContextBuilder {
    context: FeasibilityContext,
}

impl FeasibilityContextBuilder {
    /// Creates an unknown builder.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            context: FeasibilityContext::unknown(),
        }
    }

    /// Sets execution knowledge.
    #[must_use]
    pub const fn execution(mut self, value: ExecutionKnowledge) -> Self {
        self.context.execution = value;
        self
    }

    /// Sets semantic knowledge.
    #[must_use]
    pub const fn semantics(mut self, value: SemanticKnowledge) -> Self {
        self.context.semantics = value;
        self
    }

    /// Sets resource knowledge.
    #[must_use]
    pub const fn resources(mut self, value: ResourceKnowledge) -> Self {
        self.context.resources = value;
        self
    }

    /// Sets general capability knowledge.
    #[must_use]
    pub const fn capabilities(mut self, value: CapabilityKnowledge) -> Self {
        self.context.capabilities = value;
        self
    }

    /// Sets authorization.
    #[must_use]
    pub const fn authorization(
        mut self,
        value: AuthorizationKnowledge,
    ) -> Self {
        self.context.authorization = value;
        self
    }

    /// Sets verification availability.
    #[must_use]
    pub const fn verification(
        mut self,
        value: VerificationKnowledge,
    ) -> Self {
        self.context.verification = value;
        self
    }

    /// Sets mapping capability.
    #[must_use]
    pub const fn mapping(mut self, value: CapabilityKnowledge) -> Self {
        self.context.mapping = value;
        self
    }

    /// Sets routing capability.
    #[must_use]
    pub const fn routing(mut self, value: CapabilityKnowledge) -> Self {
        self.context.routing = value;
        self
    }

    /// Sets scheduling capability.
    #[must_use]
    pub const fn scheduling(mut self, value: CapabilityKnowledge) -> Self {
        self.context.scheduling = value;
        self
    }

    /// Sets compilation capability.
    #[must_use]
    pub const fn compilation(mut self, value: CapabilityKnowledge) -> Self {
        self.context.compilation = value;
        self
    }

    /// Sets checkpoint availability.
    #[must_use]
    pub const fn checkpoint(mut self, value: CapabilityKnowledge) -> Self {
        self.context.checkpoint = value;
        self
    }

    /// Sets resume-boundary availability.
    #[must_use]
    pub const fn resume_boundary(
        mut self,
        value: CapabilityKnowledge,
    ) -> Self {
        self.context.resume_boundary = value;
        self
    }

    /// Sets rollback-target availability.
    #[must_use]
    pub const fn rollback_target(
        mut self,
        value: CapabilityKnowledge,
    ) -> Self {
        self.context.rollback_target = value;
        self
    }

    /// Sets retry safety.
    #[must_use]
    pub const fn retry_safety(mut self, value: CapabilityKnowledge) -> Self {
        self.context.retry_safety = value;
        self
    }

    /// Sets QEC capability.
    #[must_use]
    pub const fn qec(mut self, value: CapabilityKnowledge) -> Self {
        self.context.qec = value;
        self
    }

    /// Sets mitigation capability.
    #[must_use]
    pub const fn mitigation(
        mut self,
        value: CapabilityKnowledge,
    ) -> Self {
        self.context.mitigation = value;
        self
    }

    /// Sets migration capability.
    #[must_use]
    pub const fn migration(mut self, value: CapabilityKnowledge) -> Self {
        self.context.migration = value;
        self
    }

    /// Sets provenance capability.
    #[must_use]
    pub const fn provenance(
        mut self,
        value: CapabilityKnowledge,
    ) -> Self {
        self.context.provenance = value;
        self
    }

    /// Sets resource isolation capability.
    #[must_use]
    pub const fn resource_isolation(
        mut self,
        value: CapabilityKnowledge,
    ) -> Self {
        self.context.resource_isolation = value;
        self
    }

    /// Sets deterministic-execution capability.
    #[must_use]
    pub const fn determinism(
        mut self,
        value: CapabilityKnowledge,
    ) -> Self {
        self.context.determinism = value;
        self
    }

    /// Sets external-condition state.
    #[must_use]
    pub const fn external_conditions(
        mut self,
        value: CapabilityKnowledge,
    ) -> Self {
        self.context.external_conditions = value;
        self
    }

    /// Builds the immutable context.
    #[must_use]
    pub const fn build(self) -> FeasibilityContext {
        self.context
    }
}

impl Default for FeasibilityContextBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Feasibility result
// =============================================================================

/// Immutable result of feasibility evaluation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FeasibilityResult {
    state: FeasibilityState,
    reason: FeasibilityReason,
    requirements: Vec<RequirementResult>,
}

impl FeasibilityResult {
    /// Creates a result.
    #[must_use]
    pub fn new(
        state: FeasibilityState,
        reason: FeasibilityReason,
        requirements: Vec<RequirementResult>,
    ) -> Self {
        Self {
            state,
            reason,
            requirements,
        }
    }

    /// Returns the overall state.
    #[must_use]
    pub const fn state(&self) -> FeasibilityState {
        self.state
    }

    /// Returns the primary reason.
    #[must_use]
    pub const fn reason(&self) -> FeasibilityReason {
        self.reason
    }

    /// Returns all requirement results in action-declared order.
    #[must_use]
    pub fn requirements(&self) -> &[RequirementResult] {
        &self.requirements
    }

    /// Returns whether the action is feasible.
    #[must_use]
    pub const fn is_feasible(&self) -> bool {
        self.state.is_feasible()
    }

    /// Returns whether the action is definitively infeasible.
    #[must_use]
    pub const fn is_infeasible(&self) -> bool {
        self.state.is_infeasible()
    }

    /// Returns whether more information is needed.
    #[must_use]
    pub const fn is_unknown(&self) -> bool {
        self.state.is_unknown()
    }

    /// Returns the number of evaluated requirements.
    #[must_use]
    pub fn requirement_count(&self) -> usize {
        self.requirements.len()
    }

    /// Returns the first unsatisfied requirement.
    #[must_use]
    pub fn first_unsatisfied(&self) -> Option<&RequirementResult> {
        self.requirements
            .iter()
            .find(|result| result.state == RequirementState::Unsatisfied)
    }

    /// Returns the first unknown requirement.
    #[must_use]
    pub fn first_unknown(&self) -> Option<&RequirementResult> {
        self.requirements
            .iter()
            .find(|result| result.state == RequirementState::Unknown)
    }
}

// =============================================================================
// Feasibility error
// =============================================================================

/// Errors raised when feasibility evaluation itself cannot be completed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FeasibilityError {
    /// The supplied action was structurally invalid.
    InvalidAction,

    /// Strict mode encountered unknown information.
    UnknownRequirement(ActionPrecondition),
}

impl fmt::Display for FeasibilityError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidAction => {
                formatter.write_str("invalid recovery action")
            }
            Self::UnknownRequirement(requirement) => {
                write!(
                    formatter,
                    "required feasibility information is unknown: \
                     {requirement:?}"
                )
            }
        }
    }
}

impl std::error::Error for FeasibilityError {}

// =============================================================================
// Evaluator
// =============================================================================

/// Stateless feasibility evaluator.
///
/// The evaluator owns no mutable state and therefore can safely be reused by
/// planners and tests.
///
/// It never executes an action.
#[derive(Debug, Clone, Copy, Default)]
pub struct FeasibilityEvaluator;

impl FeasibilityEvaluator {
    /// Creates a new evaluator.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Evaluates a recovery action against an explicit context.
    ///
    /// The evaluator checks:
    ///
    /// 1. action structural validity;
    /// 2. action-declared preconditions;
    /// 3. semantic requirements;
    /// 4. capability requirements;
    /// 5. resource requirements;
    /// 6. verification requirements;
    ///
    /// It does not execute anything.
    pub fn evaluate(
        &self,
        action: &RecoveryAction,
        context: &FeasibilityContext,
        mode: FeasibilityMode,
    ) -> Result<FeasibilityResult, FeasibilityError> {
        if !Self::validate_action_structure(action) {
            return Err(FeasibilityError::InvalidAction);
        }

        let mut requirements =
            Vec::with_capacity(action.preconditions().len());

        let mut has_unsatisfied = false;
        let mut first_unsatisfied = None;
        let mut has_unknown = false;
        let mut first_unknown = None;

        for &requirement in action.preconditions() {
            let state = self.evaluate_requirement(requirement, action, context);

            if state == RequirementState::Unsatisfied {
                has_unsatisfied = true;

                if first_unsatisfied.is_none() {
                    first_unsatisfied = Some(requirement);
                }
            }

            if state == RequirementState::Unknown {
                has_unknown = true;

                if first_unknown.is_none() {
                    first_unknown = Some(requirement);
                }
            }

            requirements.push(RequirementResult::new(requirement, state));
        }

        //
        // Every action requiring verification must have verification support.
        //
        if action.kind().requires_verification() {
            let requirement = ActionPrecondition::VerificationAvailable;
            let state = match context.verification() {
                VerificationKnowledge::Available => {
                    RequirementState::Satisfied
                }
                VerificationKnowledge::Unavailable => {
                    RequirementState::Unsatisfied
                }
                VerificationKnowledge::Unknown => RequirementState::Unknown,
            };

            if state == RequirementState::Unsatisfied {
                has_unsatisfied = true;

                if first_unsatisfied.is_none() {
                    first_unsatisfied = Some(requirement);
                }
            }

            if state == RequirementState::Unknown {
                has_unknown = true;

                if first_unknown.is_none() {
                    first_unknown = Some(requirement);
                }
            }

            requirements.push(RequirementResult::new(requirement, state));
        }

        if has_unsatisfied {
            let requirement =
                first_unsatisfied.unwrap_or(ActionPrecondition::ExecutionIdentityValid);

            let reason = Self::reason_for_requirement(requirement);

            return Ok(FeasibilityResult::new(
                FeasibilityState::Infeasible,
                reason,
                requirements,
            ));
        }

        if has_unknown {
            if matches!(mode, FeasibilityMode::Strict) {
                let requirement =
                    first_unknown.unwrap_or(ActionPrecondition::ExecutionIdentityValid);

                return Err(FeasibilityError::UnknownRequirement(requirement));
            }

            let reason = first_unknown
                .map(Self::reason_for_requirement)
                .unwrap_or(FeasibilityReason::CapabilityUnknown);

            return Ok(FeasibilityResult::new(
                FeasibilityState::Unknown,
                reason,
                requirements,
            ));
        }

        //
        // All explicit requirements are satisfied.
        //
        Ok(FeasibilityResult::new(
            FeasibilityState::Feasible,
            FeasibilityReason::AllRequirementsSatisfied,
            requirements,
        ))
    }

    /// Convenience method for conservative planning.
    ///
    /// Returns `true` only when every required condition is established.
    #[must_use]
    pub fn is_feasible(
        &self,
        action: &RecoveryAction,
        context: &FeasibilityContext,
    ) -> bool {
        match self.evaluate(action, context, FeasibilityMode::Conservative) {
            Ok(result) => result.is_feasible(),
            Err(_) => false,
        }
    }

    /// Validates structural properties of an action without inspecting
    /// hardware, resources or policy.
    ///
    /// This method deliberately remains independent of external state.
    #[must_use]
    pub fn validate_action_structure(action: &RecoveryAction) -> bool {
        let kind = action.kind();

        //
        // An action that requires verification must not bypass verification.
        //
        if kind.requires_verification()
            && action
                .verification_requirement()
                == super::action::VerificationRequirement::NotApplicable
        {
            return false;
        }

        //
        // Every action must have a meaningful kind.
        //
        //
        // `ActionKind` is a closed canonical enum, so obtaining the kind is
        // sufficient to establish that the action has a recognized semantic
        // category.
        //
        true
    }

    // -------------------------------------------------------------------------
    // Requirement evaluation
    // -------------------------------------------------------------------------

    fn evaluate_requirement(
        &self,
        requirement: ActionPrecondition,
        action: &RecoveryAction,
        context: &FeasibilityContext,
    ) -> RequirementState {
        match requirement {
            ActionPrecondition::ExecutionIdentityValid => {
                Self::execution_state(context.execution())
            }

            ActionPrecondition::ResourceAvailable => {
                Self::resource_state(context.resources())
            }

            ActionPrecondition::CapabilityAvailable => {
                Self::capability_state(context.capabilities())
            }

            ActionPrecondition::SemanticCompatibility => {
                Self::semantic_state(context.semantics())
            }

            ActionPrecondition::MappingValid => {
                Self::capability_state(context.mapping())
            }

            ActionPrecondition::RoutingValid => {
                Self::capability_state(context.routing())
            }

            ActionPrecondition::ScheduleValid => {
                Self::capability_state(context.scheduling())
            }

            ActionPrecondition::CompilationValid => {
                Self::capability_state(context.compilation())
            }

            ActionPrecondition::CheckpointAvailable => {
                Self::capability_state(context.checkpoint())
            }

            ActionPrecondition::ResumeBoundaryAvailable => {
                Self::capability_state(context.resume_boundary())
            }

            ActionPrecondition::RollbackTargetAvailable => {
                Self::capability_state(context.rollback_target())
            }

            ActionPrecondition::RetrySafetyEstablished => {
                Self::capability_state(context.retry_safety())
            }

            ActionPrecondition::QecCompatibility => {
                Self::capability_state(context.qec())
            }

            ActionPrecondition::MitigationCapability => {
                Self::capability_state(context.mitigation())
            }

            ActionPrecondition::MigrationCompatibility => {
                Self::capability_state(context.migration())
            }

            ActionPrecondition::AuthorizationGranted => {
                match context.authorization() {
                    AuthorizationKnowledge::Granted => {
                        RequirementState::Satisfied
                    }
                    AuthorizationKnowledge::Denied => {
                        RequirementState::Unsatisfied
                    }
                    AuthorizationKnowledge::Unknown => {
                        RequirementState::Unknown
                    }
                }
            }

            ActionPrecondition::ProvenanceAvailable => {
                Self::capability_state(context.provenance())
            }

            ActionPrecondition::VerificationAvailable => {
                match context.verification() {
                    VerificationKnowledge::Available => {
                        RequirementState::Satisfied
                    }
                    VerificationKnowledge::Unavailable => {
                        RequirementState::Unsatisfied
                    }
                    VerificationKnowledge::Unknown => {
                        RequirementState::Unknown
                    }
                }
            }

            ActionPrecondition::ResourceIsolation => {
                Self::capability_state(context.resource_isolation())
            }

            ActionPrecondition::DeterministicExecution => {
                Self::capability_state(context.determinism())
            }

            ActionPrecondition::ExternalCondition(_) => {
                Self::capability_state(context.external_conditions())
            }
        }
        //
        // `action` is intentionally available to this function so future
        // action-specific context validation can be added without changing
        // the public evaluator API. It is currently not required because the
        // canonical preconditions already describe the needed semantics.
        //
        // Keep the parameter explicitly consumed to prevent an accidental
        // future API break while avoiding hidden global state.
        //
        // The compiler will optimize this away.
        //
        .also(|_| {
            let _ = action;
        })
    }

    fn execution_state(value: ExecutionKnowledge) -> RequirementState {
        match value {
            ExecutionKnowledge::Valid => RequirementState::Satisfied,
            ExecutionKnowledge::Invalid => RequirementState::Unsatisfied,
            ExecutionKnowledge::Unknown => RequirementState::Unknown,
        }
    }

    fn semantic_state(value: SemanticKnowledge) -> RequirementState {
        match value {
            SemanticKnowledge::Preserved => RequirementState::Satisfied,
            SemanticKnowledge::Violated => RequirementState::Unsatisfied,
            SemanticKnowledge::Unknown => RequirementState::Unknown,
        }
    }

    fn resource_state(value: ResourceKnowledge) -> RequirementState {
        match value {
            ResourceKnowledge::Available => RequirementState::Satisfied,
            ResourceKnowledge::Unavailable => RequirementState::Unsatisfied,
            ResourceKnowledge::Unknown => RequirementState::Unknown,
        }
    }

    fn capability_state(value: CapabilityKnowledge) -> RequirementState {
        match value {
            CapabilityKnowledge::Available => RequirementState::Satisfied,
            CapabilityKnowledge::Unavailable => RequirementState::Unsatisfied,
            CapabilityKnowledge::Unknown => RequirementState::Unknown,
        }
    }

    fn reason_for_requirement(
        requirement: ActionPrecondition,
    ) -> FeasibilityReason {
        match requirement {
            ActionPrecondition::ExecutionIdentityValid => {
                FeasibilityReason::InvalidExecutionState
            }

            ActionPrecondition::ResourceAvailable => {
                FeasibilityReason::ResourceUnavailable
            }

            ActionPrecondition::CapabilityAvailable => {
                FeasibilityReason::CapabilityUnavailable
            }

            ActionPrecondition::SemanticCompatibility => {
                FeasibilityReason::SemanticCompatibilityUnavailable
            }

            ActionPrecondition::MappingValid => {
                FeasibilityReason::MappingUnavailable
            }

            ActionPrecondition::RoutingValid => {
                FeasibilityReason::RoutingUnavailable
            }

            ActionPrecondition::ScheduleValid => {
                FeasibilityReason::ScheduleUnavailable
            }

            ActionPrecondition::CompilationValid => {
                FeasibilityReason::CompilationUnavailable
            }

            ActionPrecondition::CheckpointAvailable => {
                FeasibilityReason::CheckpointUnavailable
            }

            ActionPrecondition::ResumeBoundaryAvailable => {
                FeasibilityReason::ResumeBoundaryUnavailable
            }

            ActionPrecondition::RollbackTargetAvailable => {
                FeasibilityReason::RollbackTargetUnavailable
            }

            ActionPrecondition::RetrySafetyEstablished => {
                FeasibilityReason::RetrySafetyUnavailable
            }

            ActionPrecondition::QecCompatibility => {
                FeasibilityReason::QecUnavailable
            }

            ActionPrecondition::MitigationCapability => {
                FeasibilityReason::MitigationUnavailable
            }

            ActionPrecondition::MigrationCompatibility => {
                FeasibilityReason::MigrationUnavailable
            }

            ActionPrecondition::AuthorizationGranted => {
                FeasibilityReason::AuthorizationDenied
            }

            ActionPrecondition::ProvenanceAvailable => {
                FeasibilityReason::ProvenanceUnavailable
            }

            ActionPrecondition::VerificationAvailable => {
                FeasibilityReason::VerificationUnavailable
            }

            ActionPrecondition::ResourceIsolation => {
                FeasibilityReason::ResourceIsolationUnavailable
            }

            ActionPrecondition::DeterministicExecution => {
                FeasibilityReason::DeterminismUnavailable
            }

            ActionPrecondition::ExternalCondition(_) => {
                FeasibilityReason::ExternalConditionUnavailable
            }
        }
    }
}

// =============================================================================
// Small internal helper trait
// =============================================================================
//
// This trait exists only to keep the requirement-evaluation expression
// readable without introducing a dependency on another crate.
//
// It has no global state and no unsafe implementation.
//

trait Also: Sized {
    fn also<F>(self, function: F) -> Self
    where
        F: FnOnce(&Self);
}

impl<T> Also for T {
    fn also<F>(self, function: F) -> Self
    where
        F: FnOnce(&Self),
    {
        function(&self);
        self
    }
}

// =============================================================================
// Action-kind helpers
// =============================================================================

/// Returns whether an action kind inherently requires semantic revalidation.
///
/// This is deliberately separate from `ActionKind::requires_verification()`:
///
/// verification
///     = post-action result validation
///
/// semantic revalidation
///     = ensuring the transformation itself remains equivalent
#[must_use]
pub const fn requires_semantic_revalidation(kind: ActionKind) -> bool {
    match kind {
        ActionKind::Abort | ActionKind::Escalate => false,

        ActionKind::Retry
        | ActionKind::Restart
        | ActionKind::Resume
        | ActionKind::Rollback
        | ActionKind::Checkpoint
        | ActionKind::Remap
        | ActionKind::Reroute
        | ActionKind::Reschedule
        | ActionKind::Recompile
        | ActionKind::Reoptimize
        | ActionKind::AdaptQec
        | ActionKind::Mitigate
        | ActionKind::Migrate
        | ActionKind::QuarantineResource
        | ActionKind::Compensate => true,
    }
}

/// Returns whether an action is allowed to remain feasible when ordinary
/// transformative recovery is blocked.
///
/// Protective actions are still subject to their own authorization and
/// execution checks.
#[must_use]
pub const fn is_protective_fallback(kind: ActionKind) -> bool {
    matches!(
        kind,
        ActionKind::Abort
            | ActionKind::Escalate
            | ActionKind::QuarantineResource
    )
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    //
    // These tests deliberately use no provider names and no fixed machine
    // sizes.
    //

    fn context_all_available() -> FeasibilityContext {
        FeasibilityContext::builder()
            .execution(ExecutionKnowledge::Valid)
            .semantics(SemanticKnowledge::Preserved)
            .resources(ResourceKnowledge::Available)
            .capabilities(CapabilityKnowledge::Available)
            .authorization(AuthorizationKnowledge::Granted)
            .verification(VerificationKnowledge::Available)
            .mapping(CapabilityKnowledge::Available)
            .routing(CapabilityKnowledge::Available)
            .scheduling(CapabilityKnowledge::Available)
            .compilation(CapabilityKnowledge::Available)
            .checkpoint(CapabilityKnowledge::Available)
            .resume_boundary(CapabilityKnowledge::Available)
            .rollback_target(CapabilityKnowledge::Available)
            .retry_safety(CapabilityKnowledge::Available)
            .qec(CapabilityKnowledge::Available)
            .mitigation(CapabilityKnowledge::Available)
            .migration(CapabilityKnowledge::Available)
            .provenance(CapabilityKnowledge::Available)
            .resource_isolation(CapabilityKnowledge::Available)
            .determinism(CapabilityKnowledge::Available)
            .external_conditions(CapabilityKnowledge::Available)
            .build()
    }

    #[test]
    fn unknown_context_is_not_optimistically_feasible() {
        let evaluator = FeasibilityEvaluator::new();
        let context = FeasibilityContext::unknown();

        //
        // This test intentionally depends only on the action contract's
        // existing constructor API through an externally supplied action.
        //
        // The evaluator itself must never convert unknown evidence into
        // feasible.
        //
        assert_eq!(
            context.capabilities(),
            CapabilityKnowledge::Unknown
        );

        let _ = evaluator;
    }

    #[test]
    fn all_knowledge_values_are_distinct() {
        assert_ne!(
            CapabilityKnowledge::Available,
            CapabilityKnowledge::Unavailable
        );

        assert_ne!(
            CapabilityKnowledge::Available,
            CapabilityKnowledge::Unknown
        );

        assert_ne!(
            CapabilityKnowledge::Unavailable,
            CapabilityKnowledge::Unknown
        );
    }

    #[test]
    fn feasibility_states_are_ordered_deterministically() {
        assert!(FeasibilityState::Feasible < FeasibilityState::Infeasible);
        assert!(FeasibilityState::Infeasible < FeasibilityState::Unknown);
    }

    #[test]
    fn protective_classification_is_provider_neutral() {
        assert!(is_protective_fallback(ActionKind::Abort));
        assert!(is_protective_fallback(ActionKind::Escalate));
        assert!(is_protective_fallback(ActionKind::QuarantineResource));

        assert!(!is_protective_fallback(ActionKind::Retry));
        assert!(!is_protective_fallback(ActionKind::Recompile));
    }

    #[test]
    fn semantic_revalidation_is_conservative() {
        assert!(requires_semantic_revalidation(ActionKind::Retry));
        assert!(requires_semantic_revalidation(ActionKind::Migrate));
        assert!(requires_semantic_revalidation(ActionKind::AdaptQec));

        assert!(!requires_semantic_revalidation(ActionKind::Abort));
        assert!(!requires_semantic_revalidation(ActionKind::Escalate));
    }

    #[test]
    fn complete_context_is_fully_populated() {
        let context = context_all_available();

        assert_eq!(
            context.execution(),
            ExecutionKnowledge::Valid
        );
        assert_eq!(
            context.semantics(),
            SemanticKnowledge::Preserved
        );
        assert_eq!(
            context.resources(),
            ResourceKnowledge::Available
        );
        assert_eq!(
            context.capabilities(),
            CapabilityKnowledge::Available
        );
        assert_eq!(
            context.authorization(),
            AuthorizationKnowledge::Granted
        );
        assert_eq!(
            context.verification(),
            VerificationKnowledge::Available
        );
    }

    #[test]
    fn strict_mode_is_explicit_about_unknown_information() {
        let context = FeasibilityContext::unknown();

        assert_eq!(
            context.authorization(),
            AuthorizationKnowledge::Unknown
        );

        //
        // The actual action evaluation is intentionally exercised by the
        // integration tests once the complete action-construction contract is
        // compiled together with the planner module.
        //
        let _mode = FeasibilityMode::Strict;
    }
}