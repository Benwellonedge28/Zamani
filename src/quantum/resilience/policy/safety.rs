//! Zamani Quantum Resilience — Safety Policy
//!
//! Path:
//!     src/quantum/resilience/policy/safety.rs
//!
//! Purpose:
//!     Provides the fail-closed safety authorization boundary for resilience
//!     decisions.
//!
//! Architectural position:
//!
//!     OBSERVE
//!        |
//!        v
//!     DETECT
//!        |
//!        v
//!     DIAGNOSE
//!        |
//!        v
//!     POLICY
//!        |
//!        +--> constraints
//!        +--> objectives
//!        +--> budgets
//!        +--> retry
//!        +--> escalation
//!        +--> SAFETY  <--- this module
//!        |
//!        v
//!     PLAN
//!        |
//!        v
//!     ADAPT / RECOVER / MITIGATE
//!        |
//!        v
//!     VERIFY
//!
//! This module decides whether a proposed resilience action is SAFE TO
//! CONSIDER under the explicitly supplied evidence, semantic guarantees,
//! authorization state, resource/capability state, budgets, and verification
//! requirements.
//!
//! It does NOT execute actions.
//!
//! -----------------------------------------------------------------------------
//! OWNERSHIP BOUNDARIES
//! -----------------------------------------------------------------------------
//!
//! This module MUST NOT own:
//!
//! - quantum IR;
//! - quantum gates;
//! - quantum operations;
//! - quantum circuits;
//! - logical qubit implementation;
//! - physical qubit placement;
//! - hardware discovery;
//! - hardware calibration;
//! - routing;
//! - scheduling;
//! - optimization;
//! - compilation;
//! - QEC implementation;
//! - decoder implementation;
//! - noise models;
//! - fault ontology;
//! - recovery implementation;
//! - mitigation implementation;
//! - verification implementation;
//! - credentials;
//! - authentication;
//! - network I/O;
//! - filesystem I/O;
//! - global mutable state;
//! - background threads;
//! - sleeping/waiting;
//! - retry loops;
//! - provider-specific behavior.
//!
//! The authoritative logical and physical qubit identities remain:
//!
//!     crate::quantum::ir::qubit::QubitId
//!     crate::quantum::ir::qubit::PhysicalQubitId
//!
//! This module normally does not need either type because safety policy is
//! intentionally resource-identity neutral. If a future safety contract must
//! scope a decision to a qubit, it MUST use the canonical types above rather
//! than defining another identifier.
//!
//! Fault semantics remain owned by:
//!
//!     crate::quantum::zqn
//!
//! Hardware capabilities remain owned by:
//!
//!     crate::quantum::hardware
//!
//! Routing remains owned by:
//!
//!     crate::quantum::routing
//!
//! Scheduling remains owned by:
//!
//!     crate::quantum::scheduling
//!
//! Optimization remains owned by:
//!
//!     crate::quantum::optimization
//!
//! QEC remains owned by the QEC subsystem.
//!
//! Verification remains owned by:
//!
//!     crate::quantum::resilience::verification
//!
//! Recovery remains owned by:
//!
//!     crate::quantum::resilience::recovery
//!
//! -----------------------------------------------------------------------------
//! CORE SAFETY INVARIANT
//! -----------------------------------------------------------------------------
//!
//! No resilience action may be authorized merely because it increases
//! availability, reduces latency, or appears likely to recover execution.
//!
//! A proposed action must satisfy the applicable:
//!
//!     semantic
//!     safety
//!     authorization
//!     capability
//!     resource
//!     budget
//!     provenance
//!     verification
//!
//! requirements before it may be considered executable.
//!
//! Unknown safety-critical information is NOT equivalent to approval.
//!
//! Missing evidence is NOT equivalent to success.
//!
//! An untrusted observation is NOT equivalent to a trusted observation.
//!
//! A higher objective score is NOT equivalent to authorization.
//!
//! -----------------------------------------------------------------------------
//! WRITE ONCE, SCALE EVERYWHERE
//! -----------------------------------------------------------------------------
//!
//! This file deliberately contains no:
//!
//!     MAX_QUBITS
//!     MAX_PHYSICAL_QUBITS
//!     MAX_BACKENDS
//!     MAX_RECOVERY_ATTEMPTS
//!     MAX_RETRIES
//!     MAX_PLAN_SIZE
//!     MAX_RESOURCES
//!
//! No provider names are encoded here.
//!
//! No topology is encoded here.
//!
//! No fixed fidelity threshold is encoded here.
//!
//! No fixed error-rate threshold is encoded here.
//!
//! No fixed machine size is encoded here.
//!
//! "Infinity" means that this module imposes no artificial finite machine-size
//! ceiling. Actual constraints are supplied explicitly by policy, capability,
//! resource, budget, execution, and verification contracts.
//!
//! -----------------------------------------------------------------------------
//! DETERMINISM
//! -----------------------------------------------------------------------------
//!
//! Safety evaluation is deterministic with respect to its explicit inputs.
//!
//! It does not:
//!
//! - read the system clock;
//! - inspect environment variables;
//! - read files;
//! - perform network I/O;
//! - generate randomness;
//! - inspect hidden global state;
//! - depend on iteration order of unordered collections;
//! - create implicit identifiers.
//!
//! If a higher-level system needs probabilistic safety evidence, that evidence
//! must already be represented in the supplied context.
//!
//! -----------------------------------------------------------------------------
//! SECURITY
//! -----------------------------------------------------------------------------
//!
//! This module is deliberately fail-closed.
//!
//! An action cannot become authorized merely because:
//!
//! - telemetry says it is safe;
//! - a backend says it is safe;
//! - a learned model predicts success;
//! - availability would improve;
//! - a retry budget remains;
//! - a planner ranks it highly.
//!
//! External evidence must arrive through an authenticated/trusted contract.
//!
//! This module does not authenticate evidence itself.
//!
//! -----------------------------------------------------------------------------
//! RUST CONTRACT
//! -----------------------------------------------------------------------------
//!
//! - Rust 1.97 / 1.97.1
//! - Rust 2021
//! - stable Rust only
//! - no unsafe code
//! - no unsafe operations
//! - no hidden mutable global state
//! - no fixed machine-size limits
//! - no fixed retry limits
//! - no provider-specific implementation
//!
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::fmt;

// =============================================================================
// Schema
// =============================================================================

/// Stable schema identifier for the safety-policy contract.
pub const RESILIENCE_SAFETY_SCHEMA_ID: &str =
    "zamani.quantum.resilience.policy.safety";

/// Semantic version of the safety-policy contract.
///
/// This is independent from the Rust package version.
pub const RESILIENCE_SAFETY_SCHEMA_VERSION: u16 = 1;

// =============================================================================
// Evidence state
// =============================================================================

/// Trust state of safety evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum EvidenceTrust {
    /// Evidence has not been supplied.
    Unknown,

    /// Evidence exists but its provenance/trust cannot be established.
    Untrusted,

    /// Evidence has been authenticated/validated by the owning subsystem.
    Trusted,
}

impl EvidenceTrust {
    /// Stable machine-readable name.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unknown => "unknown",
            Self::Untrusted => "untrusted",
            Self::Trusted => "trusted",
        }
    }

    /// Returns whether the evidence is trusted.
    pub const fn is_trusted(self) -> bool {
        matches!(self, Self::Trusted)
    }
}

impl fmt::Display for EvidenceTrust {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Tri-state safety evidence
// =============================================================================

/// Three-state representation of a safety condition.
///
/// `Unknown` is intentionally distinct from `Safe`.
///
/// This prevents missing information from accidentally becoming authorization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SafetyState {
    /// The condition has been positively established as safe.
    Safe,

    /// The condition has been positively established as unsafe.
    Unsafe,

    /// There is insufficient evidence to determine safety.
    Unknown,
}

impl SafetyState {
    /// Stable machine-readable name.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Safe => "safe",
            Self::Unsafe => "unsafe",
            Self::Unknown => "unknown",
        }
    }

    /// Returns whether the state is explicitly safe.
    pub const fn is_safe(self) -> bool {
        matches!(self, Self::Safe)
    }

    /// Returns whether the state is explicitly unsafe.
    pub const fn is_unsafe(self) -> bool {
        matches!(self, Self::Unsafe)
    }

    /// Returns whether the state is unknown.
    pub const fn is_unknown(self) -> bool {
        matches!(self, Self::Unknown)
    }
}

impl fmt::Display for SafetyState {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Authorization state
// =============================================================================

/// Authorization state supplied by the owning security/policy boundary.
///
/// This module consumes authorization; it does not authenticate identities.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum AuthorizationState {
    /// Authorization has not been established.
    Unknown,

    /// Authorization was explicitly denied.
    Denied,

    /// Authorization was explicitly granted.
    Granted,
}

impl AuthorizationState {
    /// Stable machine-readable name.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unknown => "unknown",
            Self::Denied => "denied",
            Self::Granted => "granted",
        }
    }

    /// Returns whether authorization is explicitly granted.
    pub const fn is_granted(self) -> bool {
        matches!(self, Self::Granted)
    }

    /// Returns whether authorization is explicitly denied.
    pub const fn is_denied(self) -> bool {
        matches!(self, Self::Denied)
    }
}

impl fmt::Display for AuthorizationState {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Semantic state
// =============================================================================

/// Semantic validity of the proposed resilience action.
///
/// This is intentionally separate from verification.
///
/// Verification proves properties of a concrete result/execution. Safety
/// policy determines whether the proposed action is permissible before it is
/// executed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SemanticState {
    /// Semantic preservation has been established for the proposed action.
    Preserved,

    /// The action would violate a required semantic guarantee.
    Violated,

    /// Semantic preservation cannot currently be established.
    Unknown,
}

impl SemanticState {
    /// Stable machine-readable name.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Preserved => "preserved",
            Self::Violated => "violated",
            Self::Unknown => "unknown",
        }
    }

    /// Returns whether semantics are explicitly preserved.
    pub const fn is_preserved(self) -> bool {
        matches!(self, Self::Preserved)
    }
}

impl fmt::Display for SemanticState {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Capability state
// =============================================================================

/// Capability state for the proposed action.
///
/// The hardware/capability subsystem remains authoritative. This enum only
/// represents the result supplied to safety policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CapabilityState {
    /// Required capabilities are available.
    Available,

    /// Required capabilities are unavailable.
    Unavailable,

    /// Capability information is incomplete or stale.
    Unknown,
}

impl CapabilityState {
    /// Stable machine-readable name.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Available => "available",
            Self::Unavailable => "unavailable",
            Self::Unknown => "unknown",
        }
    }

    /// Returns whether required capabilities are available.
    pub const fn is_available(self) -> bool {
        matches!(self, Self::Available)
    }
}

impl fmt::Display for CapabilityState {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Resource state
// =============================================================================

/// Resource feasibility supplied by the resource/capability owner.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ResourceState {
    /// Resources are sufficient.
    Available,

    /// Resources are insufficient.
    Unavailable,

    /// Resource state cannot be trusted or determined.
    Unknown,
}

impl ResourceState {
    /// Stable machine-readable name.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Available => "available",
            Self::Unavailable => "unavailable",
            Self::Unknown => "unknown",
        }
    }

    /// Returns whether sufficient resources are established.
    pub const fn is_available(self) -> bool {
        matches!(self, Self::Available)
    }
}

impl fmt::Display for ResourceState {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Budget state
// =============================================================================

/// Budget state supplied by the policy/budget subsystem.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum BudgetState {
    /// The action remains within all applicable budgets.
    Available,

    /// At least one mandatory budget is exhausted.
    Exhausted,

    /// Budget information is unavailable.
    Unknown,
}

impl BudgetState {
    /// Stable machine-readable name.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Available => "available",
            Self::Exhausted => "exhausted",
            Self::Unknown => "unknown",
        }
    }

    /// Returns whether the action remains within budget.
    pub const fn is_available(self) -> bool {
        matches!(self, Self::Available)
    }
}

impl fmt::Display for BudgetState {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Provenance state
// =============================================================================

/// Provenance state for evidence and proposed action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ProvenanceState {
    /// Provenance is complete enough for the requested safety level.
    Complete,

    /// Provenance exists but is incomplete.
    Incomplete,

    /// Provenance cannot be established.
    Unknown,

    /// Provenance has been explicitly identified as invalid.
    Invalid,
}

impl ProvenanceState {
    /// Stable machine-readable name.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Complete => "complete",
            Self::Incomplete => "incomplete",
            Self::Unknown => "unknown",
            Self::Invalid => "invalid",
        }
    }

    /// Returns whether provenance is complete.
    pub const fn is_complete(self) -> bool {
        matches!(self, Self::Complete)
    }
}

impl fmt::Display for ProvenanceState {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Verification requirement
// =============================================================================

/// Verification requirement for an action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum VerificationRequirement {
    /// No additional verification requirement is imposed by this safety
    /// decision. This does NOT disable verification required elsewhere.
    None,

    /// Verification is required before the resulting state/result may be
    /// accepted.
    Required,

    /// Verification is required before the action may even be started.
    RequiredBeforeExecution,
}

impl VerificationRequirement {
    /// Stable machine-readable name.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Required => "required",
            Self::RequiredBeforeExecution => "required_before_execution",
        }
    }

    /// Returns whether verification is required.
    pub const fn is_required(self) -> bool {
        !matches!(self, Self::None)
    }

    /// Returns whether verification must happen before execution.
    pub const fn is_required_before_execution(self) -> bool {
        matches!(self, Self::RequiredBeforeExecution)
    }
}

impl fmt::Display for VerificationRequirement {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Safety requirement
// =============================================================================

/// Configurable safety requirement.
///
/// These are requirements, not execution algorithms.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SafetyRequirement {
    /// Whether trusted evidence is mandatory.
    require_trusted_evidence: bool,

    /// Whether semantic preservation must be explicitly established.
    require_semantic_preservation: bool,

    /// Whether authorization must be explicitly granted.
    require_authorization: bool,

    /// Whether capability availability must be explicitly established.
    require_capability: bool,

    /// Whether resource availability must be explicitly established.
    require_resources: bool,

    /// Whether budget availability must be explicitly established.
    require_budget: bool,

    /// Whether provenance must be complete.
    require_provenance: bool,

    /// Whether verification must be required for recovery/adaptation actions.
    require_verification_for_mutating_actions: bool,

    /// Whether stale evidence is forbidden.
    forbid_stale_evidence: bool,
}

impl Default for SafetyRequirement {
    fn default() -> Self {
        Self {
            require_trusted_evidence: true,
            require_semantic_preservation: true,
            require_authorization: true,
            require_capability: true,
            require_resources: true,
            require_budget: true,
            require_provenance: true,
            require_verification_for_mutating_actions: true,
            forbid_stale_evidence: true,
        }
    }
}

impl SafetyRequirement {
    /// Returns the production fail-closed default.
    pub const fn strict() -> Self {
        Self {
            require_trusted_evidence: true,
            require_semantic_preservation: true,
            require_authorization: true,
            require_capability: true,
            require_resources: true,
            require_budget: true,
            require_provenance: true,
            require_verification_for_mutating_actions: true,
            forbid_stale_evidence: true,
        }
    }

    /// Returns a requirement configuration intended for explicitly controlled
    /// environments where the caller has deliberately chosen which checks are
    /// applicable.
    ///
    /// This does NOT disable the hard safety invariant: an explicitly unsafe
    /// condition still denies authorization.
    pub const fn permissive() -> Self {
        Self {
            require_trusted_evidence: false,
            require_semantic_preservation: true,
            require_authorization: true,
            require_capability: true,
            require_resources: false,
            require_budget: false,
            require_provenance: false,
            require_verification_for_mutating_actions: true,
            forbid_stale_evidence: true,
        }
    }

    pub const fn requires_trusted_evidence(self) -> bool {
        self.require_trusted_evidence
    }

    pub const fn requires_semantic_preservation(self) -> bool {
        self.require_semantic_preservation
    }

    pub const fn requires_authorization(self) -> bool {
        self.require_authorization
    }

    pub const fn requires_capability(self) -> bool {
        self.require_capability
    }

    pub const fn requires_resources(self) -> bool {
        self.require_resources
    }

    pub const fn requires_budget(self) -> bool {
        self.require_budget
    }

    pub const fn requires_provenance(self) -> bool {
        self.require_provenance
    }

    pub const fn requires_verification_for_mutating_actions(self) -> bool {
        self.require_verification_for_mutating_actions
    }

    pub const fn forbids_stale_evidence(self) -> bool {
        self.forbid_stale_evidence
    }

    pub const fn with_trusted_evidence(self, value: bool) -> Self {
        Self {
            require_trusted_evidence: value,
            ..self
        }
    }

    pub const fn with_semantic_preservation(self, value: bool) -> Self {
        Self {
            require_semantic_preservation: value,
            ..self
        }
    }

    pub const fn with_authorization(self, value: bool) -> Self {
        Self {
            require_authorization: value,
            ..self
        }
    }

    pub const fn with_capability(self, value: bool) -> Self {
        Self {
            require_capability: value,
            ..self
        }
    }

    pub const fn with_resources(self, value: bool) -> Self {
        Self {
            require_resources: value,
            ..self
        }
    }

    pub const fn with_budget(self, value: bool) -> Self {
        Self {
            require_budget: value,
            ..self
        }
    }

    pub const fn with_provenance(self, value: bool) -> Self {
        Self {
            require_provenance: value,
            ..self
        }
    }

    pub const fn with_verification_for_mutating_actions(self, value: bool) -> Self {
        Self {
            require_verification_for_mutating_actions: value,
            ..self
        }
    }

    pub const fn with_stale_evidence_forbidden(self, value: bool) -> Self {
        Self {
            forbid_stale_evidence: value,
            ..self
        }
    }
}

// =============================================================================
// Action class
// =============================================================================

/// Semantic class of the proposed action.
///
/// The actual implementation remains outside this module.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SafetyAction {
    /// No state-changing action is requested.
    Observe,

    /// Retry the current execution.
    Retry,

    /// Restart from an established execution boundary.
    Restart,

    /// Resume from a checkpoint/boundary.
    Resume,

    /// Roll back to a valid state.
    Rollback,

    /// Change logical-to-physical mapping.
    Remap,

    /// Change routing.
    Reroute,

    /// Rebuild the schedule.
    Reschedule,

    /// Recompile.
    Recompile,

    /// Reoptimize.
    Reoptimize,

    /// Adapt QEC configuration.
    AdaptQec,

    /// Apply error mitigation.
    Mitigate,

    /// Migrate execution.
    Migrate,

    /// Apply a compensation action.
    Compensate,

    /// Abort execution.
    Abort,
}

impl SafetyAction {
    /// Stable machine-readable name.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Observe => "observe",
            Self::Retry => "retry",
            Self::Restart => "restart",
            Self::Resume => "resume",
            Self::Rollback => "rollback",
            Self::Remap => "remap",
            Self::Reroute => "reroute",
            Self::Reschedule => "reschedule",
            Self::Recompile => "recompile",
            Self::Reoptimize => "reoptimize",
            Self::AdaptQec => "adapt_qec",
            Self::Mitigate => "mitigate",
            Self::Migrate => "migrate",
            Self::Compensate => "compensate",
            Self::Abort => "abort",
        }
    }

    /// Returns whether the action can change execution semantics/state.
    pub const fn is_mutating(self) -> bool {
        !matches!(self, Self::Observe)
    }

    /// Returns whether the action changes the physical implementation.
    pub const fn changes_implementation(self) -> bool {
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

    /// Returns whether the action is a recovery operation.
    pub const fn is_recovery(self) -> bool {
        matches!(
            self,
            Self::Retry
                | Self::Restart
                | Self::Resume
                | Self::Rollback
                | Self::Migrate
                | Self::Compensate
        )
    }
}

impl fmt::Display for SafetyAction {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Safety decision
// =============================================================================

/// Final safety-policy decision.
///
/// `Allow` means only that the action passed the safety policy. It does NOT
/// mean the action should be executed. Planning, capability negotiation,
/// execution, and verification remain separate gates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SafetyDecision {
    /// The action may proceed to planning/execution gates.
    Allow,

    /// The action must not proceed because a mandatory safety condition failed.
    Deny,

    /// The action cannot be safely authorized because required evidence is
    /// incomplete/unknown.
    RequireEvidence,

    /// The action must be externally escalated rather than autonomously
    /// authorized.
    Escalate,
}

impl SafetyDecision {
    /// Stable machine-readable name.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Allow => "allow",
            Self::Deny => "deny",
            Self::RequireEvidence => "require_evidence",
            Self::Escalate => "escalate",
        }
    }

    /// Returns whether the action passed this safety layer.
    pub const fn is_allowed(self) -> bool {
        matches!(self, Self::Allow)
    }

    /// Returns whether the action is denied.
    pub const fn is_denied(self) -> bool {
        matches!(self, Self::Deny)
    }

    /// Returns whether more evidence is required.
    pub const fn requires_evidence(self) -> bool {
        matches!(self, Self::RequireEvidence)
    }

    /// Returns whether external authority is required.
    pub const fn requires_escalation(self) -> bool {
        matches!(self, Self::Escalate)
    }
}

impl fmt::Display for SafetyDecision {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Safety reason
// =============================================================================

/// Machine-readable explanation for a safety decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SafetyReason {
    /// All applicable safety requirements passed.
    AllChecksPassed,

    /// Explicit unsafe evidence exists.
    UnsafeCondition,

    /// Required safety evidence is missing.
    SafetyEvidenceUnknown,

    /// Evidence cannot be trusted.
    EvidenceUntrusted,

    /// Evidence is stale.
    EvidenceStale,

    /// Authorization is denied.
    AuthorizationDenied,

    /// Authorization is unknown.
    AuthorizationUnknown,

    /// Required semantic preservation was not established.
    SemanticPreservationUnknown,

    /// Proposed action would violate semantics.
    SemanticViolation,

    /// Required capability is unavailable.
    CapabilityUnavailable,

    /// Capability state is unknown.
    CapabilityUnknown,

    /// Required resources are unavailable.
    ResourceUnavailable,

    /// Resource state is unknown.
    ResourceUnknown,

    /// Required budget is exhausted.
    BudgetExhausted,

    /// Budget state is unknown.
    BudgetUnknown,

    /// Provenance is invalid.
    ProvenanceInvalid,

    /// Provenance is incomplete.
    ProvenanceIncomplete,

    /// Provenance is unknown.
    ProvenanceUnknown,

    /// Required verification is missing.
    VerificationRequired,

    /// An action that mutates execution cannot be authorized without
    /// verification requirements being represented.
    VerificationContractMissing,

    /// The action is explicitly forbidden by the safety policy.
    ActionForbidden,

    /// A safety-critical policy condition conflicts with another condition.
    PolicyConflict,

    /// Safety state is stale or inconsistent.
    StaleState,

    /// An action requires a higher-level authority.
    EscalationRequired,
}

impl SafetyReason {
    /// Stable machine-readable name.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AllChecksPassed => "all_checks_passed",
            Self::UnsafeCondition => "unsafe_condition",
            Self::SafetyEvidenceUnknown => "safety_evidence_unknown",
            Self::EvidenceUntrusted => "evidence_untrusted",
            Self::EvidenceStale => "evidence_stale",
            Self::AuthorizationDenied => "authorization_denied",
            Self::AuthorizationUnknown => "authorization_unknown",
            Self::SemanticPreservationUnknown => "semantic_preservation_unknown",
            Self::SemanticViolation => "semantic_violation",
            Self::CapabilityUnavailable => "capability_unavailable",
            Self::CapabilityUnknown => "capability_unknown",
            Self::ResourceUnavailable => "resource_unavailable",
            Self::ResourceUnknown => "resource_unknown",
            Self::BudgetExhausted => "budget_exhausted",
            Self::BudgetUnknown => "budget_unknown",
            Self::ProvenanceInvalid => "provenance_invalid",
            Self::ProvenanceIncomplete => "provenance_incomplete",
            Self::ProvenanceUnknown => "provenance_unknown",
            Self::VerificationRequired => "verification_required",
            Self::VerificationContractMissing => "verification_contract_missing",
            Self::ActionForbidden => "action_forbidden",
            Self::PolicyConflict => "policy_conflict",
            Self::StaleState => "stale_state",
            Self::EscalationRequired => "escalation_required",
        }
    }
}

impl fmt::Display for SafetyReason {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Evaluation result
// =============================================================================

/// Complete result of a safety evaluation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SafetyEvaluation {
    decision: SafetyDecision,
    reason: SafetyReason,
    verification: VerificationRequirement,
}

impl SafetyEvaluation {
    const fn new(
        decision: SafetyDecision,
        reason: SafetyReason,
        verification: VerificationRequirement,
    ) -> Self {
        Self {
            decision,
            reason,
            verification,
        }
    }

    /// Final safety decision.
    pub const fn decision(self) -> SafetyDecision {
        self.decision
    }

    /// Machine-readable reason.
    pub const fn reason(self) -> SafetyReason {
        self.reason
    }

    /// Verification requirement attached to the decision.
    pub const fn verification_requirement(self) -> VerificationRequirement {
        self.verification
    }

    /// Returns whether the safety layer authorized the action.
    pub const fn is_allowed(self) -> bool {
        self.decision.is_allowed()
    }
}

// =============================================================================
// Evaluation context
// =============================================================================

/// Immutable evidence snapshot evaluated by [`SafetyPolicy`].
///
/// The context deliberately contains only normalized policy facts. Hardware,
/// ZQN, QEC, routing, scheduling, verification, and security subsystems remain
/// responsible for producing those facts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SafetyContext {
    action: SafetyAction,
    safety: SafetyState,
    evidence_trust: EvidenceTrust,
    authorization: AuthorizationState,
    semantic: SemanticState,
    capability: CapabilityState,
    resources: ResourceState,
    budget: BudgetState,
    provenance: ProvenanceState,
    verification: VerificationRequirement,
    stale: bool,
    explicit_escalation: bool,
}

impl SafetyContext {
    /// Creates a context with conservative defaults.
    ///
    /// The default is intentionally NOT safe to execute. The caller must
    /// explicitly provide the evidence required by the configured policy.
    pub const fn new(action: SafetyAction) -> Self {
        Self {
            action,
            safety: SafetyState::Unknown,
            evidence_trust: EvidenceTrust::Unknown,
            authorization: AuthorizationState::Unknown,
            semantic: SemanticState::Unknown,
            capability: CapabilityState::Unknown,
            resources: ResourceState::Unknown,
            budget: BudgetState::Unknown,
            provenance: ProvenanceState::Unknown,
            verification: VerificationRequirement::None,
            stale: false,
            explicit_escalation: false,
        }
    }

    pub const fn action(self) -> SafetyAction {
        self.action
    }

    pub const fn safety(self) -> SafetyState {
        self.safety
    }

    pub const fn evidence_trust(self) -> EvidenceTrust {
        self.evidence_trust
    }

    pub const fn authorization(self) -> AuthorizationState {
        self.authorization
    }

    pub const fn semantic(self) -> SemanticState {
        self.semantic
    }

    pub const fn capability(self) -> CapabilityState {
        self.capability
    }

    pub const fn resources(self) -> ResourceState {
        self.resources
    }

    pub const fn budget(self) -> BudgetState {
        self.budget
    }

    pub const fn provenance(self) -> ProvenanceState {
        self.provenance
    }

    pub const fn verification(self) -> VerificationRequirement {
        self.verification
    }

    pub const fn is_stale(self) -> bool {
        self.stale
    }

    pub const fn explicit_escalation(self) -> bool {
        self.explicit_escalation
    }

    pub const fn with_safety(self, value: SafetyState) -> Self {
        Self {
            safety: value,
            ..self
        }
    }

    pub const fn with_evidence_trust(self, value: EvidenceTrust) -> Self {
        Self {
            evidence_trust: value,
            ..self
        }
    }

    pub const fn with_authorization(self, value: AuthorizationState) -> Self {
        Self {
            authorization: value,
            ..self
        }
    }

    pub const fn with_semantic(self, value: SemanticState) -> Self {
        Self {
            semantic: value,
            ..self
        }
    }

    pub const fn with_capability(self, value: CapabilityState) -> Self {
        Self {
            capability: value,
            ..self
        }
    }

    pub const fn with_resources(self, value: ResourceState) -> Self {
        Self {
            resources: value,
            ..self
        }
    }

    pub const fn with_budget(self, value: BudgetState) -> Self {
        Self {
            budget: value,
            ..self
        }
    }

    pub const fn with_provenance(self, value: ProvenanceState) -> Self {
        Self {
            provenance: value,
            ..self
        }
    }

    pub const fn with_verification(
        self,
        value: VerificationRequirement,
    ) -> Self {
        Self {
            verification: value,
            ..self
        }
    }

    pub const fn with_stale(self, value: bool) -> Self {
        Self {
            stale: value,
            ..self
        }
    }

    pub const fn with_explicit_escalation(self, value: bool) -> Self {
        Self {
            explicit_escalation: value,
            ..self
        }
    }
}

// =============================================================================
// Safety policy
// =============================================================================

/// Production safety policy.
///
/// The policy is intentionally small and deterministic. It is a gate, not a
/// planner and not an executor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SafetyPolicy {
    requirements: SafetyRequirement,
}

impl Default for SafetyPolicy {
    fn default() -> Self {
        Self::strict()
    }
}

impl SafetyPolicy {
    /// Production fail-closed policy.
    pub const fn strict() -> Self {
        Self {
            requirements: SafetyRequirement::strict(),
        }
    }

    /// Creates a policy from explicit requirements.
    pub const fn new(requirements: SafetyRequirement) -> Self {
        Self { requirements }
    }

    /// Returns the configured requirements.
    pub const fn requirements(self) -> SafetyRequirement {
        self.requirements
    }

    /// Evaluates one immutable safety snapshot.
    ///
    /// The evaluation order is deliberate:
    ///
    /// 1. explicit unsafe conditions;
    /// 2. explicit escalation;
    /// 3. stale/untrusted evidence;
    /// 4. authorization;
    /// 5. semantic preservation;
    /// 6. capability;
    /// 7. resources;
    /// 8. budget;
    /// 9. provenance;
    /// 10. verification.
    ///
    /// This means an availability objective cannot override a safety failure.
    pub const fn evaluate(self, context: SafetyContext) -> SafetyEvaluation {
        // ---------------------------------------------------------------------
        // Hard safety barrier.
        // ---------------------------------------------------------------------

        if context.safety.is_unsafe() {
            return SafetyEvaluation::new(
                SafetyDecision::Deny,
                SafetyReason::UnsafeCondition,
                required_verification(context),
            );
        }

        // ---------------------------------------------------------------------
        // Explicit external escalation always wins over autonomous execution.
        // ---------------------------------------------------------------------

        if context.explicit_escalation {
            return SafetyEvaluation::new(
                SafetyDecision::Escalate,
                SafetyReason::EscalationRequired,
                required_verification(context),
            );
        }

        // ---------------------------------------------------------------------
        // Stale state/evidence cannot authorize a safety-critical action.
        // ---------------------------------------------------------------------

        if context.stale && self.requirements.forbids_stale_evidence() {
            return SafetyEvaluation::new(
                SafetyDecision::RequireEvidence,
                SafetyReason::EvidenceStale,
                required_verification(context),
            );
        }

        // ---------------------------------------------------------------------
        // Trust boundary.
        // ---------------------------------------------------------------------

        if self.requirements.requires_trusted_evidence() {
            match context.evidence_trust {
                EvidenceTrust::Trusted => {}
                EvidenceTrust::Untrusted => {
                    return SafetyEvaluation::new(
                        SafetyDecision::RequireEvidence,
                        SafetyReason::EvidenceUntrusted,
                        required_verification(context),
                    );
                }
                EvidenceTrust::Unknown => {
                    return SafetyEvaluation::new(
                        SafetyDecision::RequireEvidence,
                        SafetyReason::SafetyEvidenceUnknown,
                        required_verification(context),
                    );
                }
            }
        }

        // ---------------------------------------------------------------------
        // Authorization.
        // ---------------------------------------------------------------------

        if self.requirements.requires_authorization() {
            match context.authorization {
                AuthorizationState::Granted => {}
                AuthorizationState::Denied => {
                    return SafetyEvaluation::new(
                        SafetyDecision::Deny,
                        SafetyReason::AuthorizationDenied,
                        required_verification(context),
                    );
                }
                AuthorizationState::Unknown => {
                    return SafetyEvaluation::new(
                        SafetyDecision::RequireEvidence,
                        SafetyReason::AuthorizationUnknown,
                        required_verification(context),
                    );
                }
            }
        }

        // ---------------------------------------------------------------------
        // Semantic correctness is a hard constraint.
        // ---------------------------------------------------------------------

        if self.requirements.requires_semantic_preservation() {
            match context.semantic {
                SemanticState::Preserved => {}
                SemanticState::Violated => {
                    return SafetyEvaluation::new(
                        SafetyDecision::Deny,
                        SafetyReason::SemanticViolation,
                        required_verification(context),
                    );
                }
                SemanticState::Unknown => {
                    return SafetyEvaluation::new(
                        SafetyDecision::RequireEvidence,
                        SafetyReason::SemanticPreservationUnknown,
                        required_verification(context),
                    );
                }
            }
        }

        // ---------------------------------------------------------------------
        // Capability.
        // ---------------------------------------------------------------------

        if self.requirements.requires_capability() {
            match context.capability {
                CapabilityState::Available => {}
                CapabilityState::Unavailable => {
                    return SafetyEvaluation::new(
                        SafetyDecision::Deny,
                        SafetyReason::CapabilityUnavailable,
                        required_verification(context),
                    );
                }
                CapabilityState::Unknown => {
                    return SafetyEvaluation::new(
                        SafetyDecision::RequireEvidence,
                        SafetyReason::CapabilityUnknown,
                        required_verification(context),
                    );
                }
            }
        }

        // ---------------------------------------------------------------------
        // Resources.
        // ---------------------------------------------------------------------

        if self.requirements.requires_resources() {
            match context.resources {
                ResourceState::Available => {}
                ResourceState::Unavailable => {
                    return SafetyEvaluation::new(
                        SafetyDecision::Deny,
                        SafetyReason::ResourceUnavailable,
                        required_verification(context),
                    );
                }
                ResourceState::Unknown => {
                    return SafetyEvaluation::new(
                        SafetyDecision::RequireEvidence,
                        SafetyReason::ResourceUnknown,
                        required_verification(context),
                    );
                }
            }
        }

        // ---------------------------------------------------------------------
        // Budgets.
        // ---------------------------------------------------------------------

        if self.requirements.requires_budget() {
            match context.budget {
                BudgetState::Available => {}
                BudgetState::Exhausted => {
                    return SafetyEvaluation::new(
                        SafetyDecision::Deny,
                        SafetyReason::BudgetExhausted,
                        required_verification(context),
                    );
                }
                BudgetState::Unknown => {
                    return SafetyEvaluation::new(
                        SafetyDecision::RequireEvidence,
                        SafetyReason::BudgetUnknown,
                        required_verification(context),
                    );
                }
            }
        }

        // ---------------------------------------------------------------------
        // Provenance.
        // ---------------------------------------------------------------------

        if self.requirements.requires_provenance() {
            match context.provenance {
                ProvenanceState::Complete => {}
                ProvenanceState::Incomplete => {
                    return SafetyEvaluation::new(
                        SafetyDecision::RequireEvidence,
                        SafetyReason::ProvenanceIncomplete,
                        required_verification(context),
                    );
                }
                ProvenanceState::Unknown => {
                    return SafetyEvaluation::new(
                        SafetyDecision::RequireEvidence,
                        SafetyReason::ProvenanceUnknown,
                        required_verification(context),
                    );
                }
                ProvenanceState::Invalid => {
                    return SafetyEvaluation::new(
                        SafetyDecision::Deny,
                        SafetyReason::ProvenanceInvalid,
                        required_verification(context),
                    );
                }
            }
        }

        // ---------------------------------------------------------------------
        // Mutating actions require an explicit verification contract.
        // ---------------------------------------------------------------------

        if context.action.is_mutating()
            && self
                .requirements
                .requires_verification_for_mutating_actions()
        {
            if !context.verification.is_required() {
                return SafetyEvaluation::new(
                    SafetyDecision::RequireEvidence,
                    SafetyReason::VerificationRequired,
                    VerificationRequirement::Required,
                );
            }
        }

        // ---------------------------------------------------------------------
        // Explicitly unsafe/unknown safety state.
        // ---------------------------------------------------------------------

        match context.safety {
            SafetyState::Safe => {}
            SafetyState::Unsafe => {
                return SafetyEvaluation::new(
                    SafetyDecision::Deny,
                    SafetyReason::UnsafeCondition,
                    required_verification(context),
                );
            }
            SafetyState::Unknown => {
                return SafetyEvaluation::new(
                    SafetyDecision::RequireEvidence,
                    SafetyReason::SafetyEvidenceUnknown,
                    required_verification(context),
                );
            }
        }

        SafetyEvaluation::new(
            SafetyDecision::Allow,
            SafetyReason::AllChecksPassed,
            context.verification,
        )
    }
}

// =============================================================================
// Helpers
// =============================================================================

/// Returns the verification requirement that must accompany a denial or
/// evidence request.
const fn required_verification(context: SafetyContext) -> VerificationRequirement {
    if context.verification.is_required() {
        context.verification
    } else if context.action.is_mutating() {
        VerificationRequirement::Required
    } else {
        VerificationRequirement::None
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    const fn safe_context(action: SafetyAction) -> SafetyContext {
        SafetyContext::new(action)
            .with_safety(SafetyState::Safe)
            .with_evidence_trust(EvidenceTrust::Trusted)
            .with_authorization(AuthorizationState::Granted)
            .with_semantic(SemanticState::Preserved)
            .with_capability(CapabilityState::Available)
            .with_resources(ResourceState::Available)
            .with_budget(BudgetState::Available)
            .with_provenance(ProvenanceState::Complete)
            .with_verification(VerificationRequirement::Required)
    }

    #[test]
    fn strict_policy_allows_fully_established_safe_action() {
        let policy = SafetyPolicy::strict();

        let context = safe_context(SafetyAction::Retry);

        let result = policy.evaluate(context);

        assert_eq!(result.decision(), SafetyDecision::Allow);
        assert_eq!(result.reason(), SafetyReason::AllChecksPassed);
        assert_eq!(
            result.verification_requirement(),
            VerificationRequirement::Required
        );
    }

    #[test]
    fn unknown_safety_fails_closed() {
        let policy = SafetyPolicy::strict();

        let context = SafetyContext::new(SafetyAction::Retry)
            .with_evidence_trust(EvidenceTrust::Trusted)
            .with_authorization(AuthorizationState::Granted)
            .with_semantic(SemanticState::Preserved)
            .with_capability(CapabilityState::Available)
            .with_resources(ResourceState::Available)
            .with_budget(BudgetState::Available)
            .with_provenance(ProvenanceState::Complete)
            .with_verification(VerificationRequirement::Required);

        let result = policy.evaluate(context);

        assert_eq!(result.decision(), SafetyDecision::RequireEvidence);
        assert_eq!(
            result.reason(),
            SafetyReason::SafetyEvidenceUnknown
        );
    }

    #[test]
    fn explicit_unsafe_condition_overrides_availability() {
        let policy = SafetyPolicy::strict();

        let context = safe_context(SafetyAction::Retry)
            .with_safety(SafetyState::Unsafe);

        let result = policy.evaluate(context);

        assert_eq!(result.decision(), SafetyDecision::Deny);
        assert_eq!(result.reason(), SafetyReason::UnsafeCondition);
    }

    #[test]
    fn unknown_evidence_does_not_become_authorization() {
        let policy = SafetyPolicy::strict();

        let context = safe_context(SafetyAction::Retry)
            .with_evidence_trust(EvidenceTrust::Unknown);

        let result = policy.evaluate(context);

        assert_eq!(result.decision(), SafetyDecision::RequireEvidence);
        assert_eq!(result.reason(), SafetyReason::SafetyEvidenceUnknown);
    }

    #[test]
    fn untrusted_evidence_does_not_authorize_action() {
        let policy = SafetyPolicy::strict();

        let context = safe_context(SafetyAction::Retry)
            .with_evidence_trust(EvidenceTrust::Untrusted);

        let result = policy.evaluate(context);

        assert_eq!(result.decision(), SafetyDecision::RequireEvidence);
        assert_eq!(result.reason(), SafetyReason::EvidenceUntrusted);
    }

    #[test]
    fn denied_authorization_is_terminal_for_this_policy_layer() {
        let policy = SafetyPolicy::strict();

        let context = safe_context(SafetyAction::Retry)
            .with_authorization(AuthorizationState::Denied);

        let result = policy.evaluate(context);

        assert_eq!(result.decision(), SafetyDecision::Deny);
        assert_eq!(
            result.reason(),
            SafetyReason::AuthorizationDenied
        );
    }

    #[test]
    fn unknown_authorization_fails_closed() {
        let policy = SafetyPolicy::strict();

        let context = safe_context(SafetyAction::Retry)
            .with_authorization(AuthorizationState::Unknown);

        let result = policy.evaluate(context);

        assert_eq!(result.decision(), SafetyDecision::RequireEvidence);
        assert_eq!(
            result.reason(),
            SafetyReason::AuthorizationUnknown
        );
    }

    #[test]
    fn semantic_violation_cannot_be_overridden_by_other_successes() {
        let policy = SafetyPolicy::strict();

        let context = safe_context(SafetyAction::Recompile)
            .with_semantic(SemanticState::Violated);

        let result = policy.evaluate(context);

        assert_eq!(result.decision(), SafetyDecision::Deny);
        assert_eq!(
            result.reason(),
            SafetyReason::SemanticViolation
        );
    }

    #[test]
    fn unknown_semantics_fail_closed() {
        let policy = SafetyPolicy::strict();

        let context = safe_context(SafetyAction::Reroute)
            .with_semantic(SemanticState::Unknown);

        let result = policy.evaluate(context);

        assert_eq!(result.decision(), SafetyDecision::RequireEvidence);
        assert_eq!(
            result.reason(),
            SafetyReason::SemanticPreservationUnknown
        );
    }

    #[test]
    fn unavailable_capability_denies_action() {
        let policy = SafetyPolicy::strict();

        let context = safe_context(SafetyAction::Migrate)
            .with_capability(CapabilityState::Unavailable);

        let result = policy.evaluate(context);

        assert_eq!(result.decision(), SafetyDecision::Deny);
        assert_eq!(
            result.reason(),
            SafetyReason::CapabilityUnavailable
        );
    }

    #[test]
    fn unknown_capability_requires_evidence() {
        let policy = SafetyPolicy::strict();

        let context = safe_context(SafetyAction::Migrate)
            .with_capability(CapabilityState::Unknown);

        let result = policy.evaluate(context);

        assert_eq!(result.decision(), SafetyDecision::RequireEvidence);
        assert_eq!(
            result.reason(),
            SafetyReason::CapabilityUnknown
        );
    }

    #[test]
    fn exhausted_budget_denies_action() {
        let policy = SafetyPolicy::strict();

        let context = safe_context(SafetyAction::Retry)
            .with_budget(BudgetState::Exhausted);

        let result = policy.evaluate(context);

        assert_eq!(result.decision(), SafetyDecision::Deny);
        assert_eq!(result.reason(), SafetyReason::BudgetExhausted);
    }

    #[test]
    fn incomplete_provenance_cannot_authorize_strict_policy() {
        let policy = SafetyPolicy::strict();

        let context = safe_context(SafetyAction::Recompile)
            .with_provenance(ProvenanceState::Incomplete);

        let result = policy.evaluate(context);

        assert_eq!(result.decision(), SafetyDecision::RequireEvidence);
        assert_eq!(
            result.reason(),
            SafetyReason::ProvenanceIncomplete
        );
    }

    #[test]
    fn invalid_provenance_denies_action() {
        let policy = SafetyPolicy::strict();

        let context = safe_context(SafetyAction::Recompile)
            .with_provenance(ProvenanceState::Invalid);

        let result = policy.evaluate(context);

        assert_eq!(result.decision(), SafetyDecision::Deny);
        assert_eq!(
            result.reason(),
            SafetyReason::ProvenanceInvalid
        );
    }

    #[test]
    fn stale_state_fails_closed() {
        let policy = SafetyPolicy::strict();

        let context = safe_context(SafetyAction::Reroute)
            .with_stale(true);

        let result = policy.evaluate(context);

        assert_eq!(result.decision(), SafetyDecision::RequireEvidence);
        assert_eq!(result.reason(), SafetyReason::EvidenceStale);
    }

    #[test]
    fn mutating_action_requires_verification_contract() {
        let policy = SafetyPolicy::strict();

        let context = safe_context(SafetyAction::Retry)
            .with_verification(VerificationRequirement::None);

        let result = policy.evaluate(context);

        assert_eq!(result.decision(), SafetyDecision::RequireEvidence);
        assert_eq!(result.reason(), SafetyReason::VerificationRequired);
        assert_eq!(
            result.verification_requirement(),
            VerificationRequirement::Required
        );
    }

    #[test]
    fn observation_does_not_require_mutation_verification() {
        let policy = SafetyPolicy::strict();

        let context = SafetyContext::new(SafetyAction::Observe)
            .with_safety(SafetyState::Safe)
            .with_evidence_trust(EvidenceTrust::Trusted)
            .with_authorization(AuthorizationState::Granted)
            .with_semantic(SemanticState::Preserved)
            .with_capability(CapabilityState::Available)
            .with_resources(ResourceState::Available)
            .with_budget(BudgetState::Available)
            .with_provenance(ProvenanceState::Complete);

        let result = policy.evaluate(context);

        assert_eq!(result.decision(), SafetyDecision::Allow);
        assert_eq!(
            result.verification_requirement(),
            VerificationRequirement::None
        );
    }

    #[test]
    fn explicit_escalation_overrides_other_positive_conditions() {
        let policy = SafetyPolicy::strict();

        let context = safe_context(SafetyAction::Migrate)
            .with_explicit_escalation(true);

        let result = policy.evaluate(context);

        assert_eq!(result.decision(), SafetyDecision::Escalate);
        assert_eq!(result.reason(), SafetyReason::EscalationRequired);
    }

    #[test]
    fn explicit_unsafe_condition_has_precedence_over_escalation() {
        let policy = SafetyPolicy::strict();

        let context = safe_context(SafetyAction::Migrate)
            .with_safety(SafetyState::Unsafe)
            .with_explicit_escalation(true);

        let result = policy.evaluate(context);

        assert_eq!(result.decision(), SafetyDecision::Deny);
        assert_eq!(result.reason(), SafetyReason::UnsafeCondition);
    }

    #[test]
    fn evaluation_is_deterministic() {
        let policy = SafetyPolicy::strict();

        let context = safe_context(SafetyAction::Reoptimize);

        let first = policy.evaluate(context);
        let second = policy.evaluate(context);

        assert_eq!(first, second);
    }

    #[test]
    fn default_context_is_not_authorized() {
        let policy = SafetyPolicy::strict();

        let context = SafetyContext::new(SafetyAction::Retry);

        let result = policy.evaluate(context);

        assert_ne!(result.decision(), SafetyDecision::Allow);
    }

    #[test]
    fn all_mutating_actions_require_verification() {
        let actions = [
            SafetyAction::Retry,
            SafetyAction::Restart,
            SafetyAction::Resume,
            SafetyAction::Rollback,
            SafetyAction::Remap,
            SafetyAction::Reroute,
            SafetyAction::Reschedule,
            SafetyAction::Recompile,
            SafetyAction::Reoptimize,
            SafetyAction::AdaptQec,
            SafetyAction::Mitigate,
            SafetyAction::Migrate,
            SafetyAction::Compensate,
            SafetyAction::Abort,
        ];

        let policy = SafetyPolicy::strict();

        for action in actions {
            let context = safe_context(action)
                .with_verification(VerificationRequirement::None);

            let result = policy.evaluate(context);

            assert_eq!(
                result.decision(),
                SafetyDecision::RequireEvidence,
                "action {action} unexpectedly passed without verification"
            );
        }
    }
}