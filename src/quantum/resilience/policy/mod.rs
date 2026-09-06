//! Zamani Quantum Resilience — Safety Policy
//!
//! Path:
//!     src/quantum/resilience/policy/safety.rs
//!
//! Purpose:
//!     Provides the deterministic, fail-closed safety authorization boundary
//!     for quantum-resilience actions.
//!
//! Architectural role:
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
//! This module answers one question:
//!
//!     "May this proposed resilience action pass the safety-policy gate?"
//!
//! It does NOT answer:
//!
//!     "Should this be the best plan?"
//!     "How is the action executed?"
//!     "Which backend should be used?"
//!     "How should qubits be routed?"
//!     "How should the circuit be scheduled?"
//!     "How should QEC be implemented?"
//!     "How should a result be verified?"
//!
//! Those responsibilities remain with their owning subsystems.
//!
//! =============================================================================
//! OWNERSHIP BOUNDARIES
//! =============================================================================
//!
//! This module MUST NOT own:
//!
//! - canonical quantum IR;
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
//! Canonical qubit identity remains:
//!
//!     crate::quantum::ir::qubit::QubitId
//!     crate::quantum::ir::qubit::PhysicalQubitId
//!
//! This module intentionally does not import or duplicate those types because
//! the safety policy is resource-identity neutral. Any future resource-scoped
//! safety contract MUST use the canonical IR identity types.
//!
//! Fault semantics remain owned by:
//!
//!     crate::quantum::zqn
//!
//! Hardware capability/state remains owned by:
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
//! =============================================================================
//! SAFETY INVARIANT
//! =============================================================================
//!
//! No resilience action may be authorized merely because it:
//!
//! - increases availability;
//! - reduces latency;
//! - improves an objective score;
//! - appears likely to succeed;
//! - has remaining retry budget;
//! - is recommended by a learned model;
//! - is ranked first by the planner.
//!
//! A normal execution-changing action must satisfy all applicable safety gates:
//!
//!     action authorization
//!     evidence trust
//!     evidence consistency
//!     freshness
//!     semantic preservation
//!     capability availability
//!     resource availability
//!     budget availability
//!     provenance
//!     verification contract
//!
//! Unknown information is never silently converted into approval.
//!
//! Explicitly unsafe information is never silently converted into approval.
//!
//! An optimization objective can never override a safety failure.
//!
//! =============================================================================
//! PROTECTIVE ACTION PRINCIPLE
//! =============================================================================
//!
//! `Abort` is intentionally different from ordinary mutating actions.
//!
//! If a computation has become unsafe, the safest available action may be to
//! stop it. Therefore an unsafe execution condition may authorize a previously
//! authorized ABORT action, subject to the remaining security/authorization
//! gates.
//!
//! This prevents the safety layer from creating an unsafe paradox:
//!
//!     unsafe execution
//!         |
//!         X  "unsafe, therefore no action is allowed"
//!
//! Instead:
//!
//!     unsafe execution
//!         |
//!         +--> ordinary recovery/adaptation: DENY
//!         |
//!         +--> protective abort: MAY ALLOW
//!
//! Abort authorization still does NOT execute the abort. The recovery/execution
//! subsystem remains responsible for actually stopping execution.
//!
//! =============================================================================
//! WRITE ONCE, SCALE EVERYWHERE
//! =============================================================================
//!
//! This file deliberately contains no machine-size constants:
//!
//!     MAX_QUBITS
//!     MAX_PHYSICAL_QUBITS
//!     MAX_BACKENDS
//!     MAX_RECOVERY_ATTEMPTS
//!     MAX_RETRIES
//!     MAX_PLAN_SIZE
//!     MAX_RESOURCES
//!
//! It contains no provider names.
//!
//! It contains no topology.
//!
//! It contains no fixed fidelity threshold.
//!
//! It contains no fixed error-rate threshold.
//!
//! It contains no fixed timeout.
//!
//! It contains no fixed retry count.
//!
//! It contains no fixed machine size.
//!
//! "Infinity" means that this module imposes no artificial finite quantum
//! machine-size ceiling. Every concrete execution remains bounded by explicit
//! resources, capabilities, budgets, security policy and the target itself.
//!
//! =============================================================================
//! DETERMINISM
//! =============================================================================
//!
//! Safety evaluation is a pure deterministic function of its explicit inputs.
//!
//! It does not:
//!
//! - read the system clock;
//! - read environment variables;
//! - read files;
//! - perform network I/O;
//! - generate randomness;
//! - inspect hidden global state;
//! - create implicit identifiers;
//! - depend on unordered collection iteration.
//!
//! If freshness, timestamps, probabilities, confidence values, or trust
//! information matter, those facts must be supplied explicitly by the owning
//! subsystem.
//!
//! =============================================================================
//! SECURITY
//! =============================================================================
//!
//! This module is a policy gate, not an authentication mechanism.
//!
//! Authentication and credential verification belong to the security/provider
//! boundary. This module consumes their normalized result.
//!
//! Learned models, telemetry, backend claims and planner rankings are advisory
//! evidence unless the caller explicitly marks the corresponding evidence as
//! trusted.
//!
//! =============================================================================
//! RUST CONTRACT
//! =============================================================================
//!
//! - Rust 1.97 / Rust 1.97.1
//! - Rust 2021
//! - stable Rust only
//! - no unsafe code
//! - no unsafe operations
//! - no hidden mutable state
//! - no hidden I/O
//! - no provider-specific logic
//! - no hard-coded machine-size limits
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
/// This is independent of the Rust package version.
pub const RESILIENCE_SAFETY_SCHEMA_VERSION: u16 = 2;

// =============================================================================
// Evidence trust
// =============================================================================

/// Trust level assigned to evidence by its owning subsystem.
///
/// Safety policy does not establish trust itself. It only consumes the result.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum EvidenceTrust {
    /// No trust determination exists.
    Unknown,

    /// Evidence exists but cannot be relied upon for strict authorization.
    Untrusted,

    /// Evidence has passed the owning subsystem's trust/authentication boundary.
    Trusted,
}

impl EvidenceTrust {
    /// Stable machine-readable representation.
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
// Evidence consistency
// =============================================================================

/// Consistency state for the supplied safety evidence.
///
/// Conflicting evidence must never be silently resolved by choosing the
/// optimistic interpretation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum EvidenceConsistency {
    /// Evidence is mutually consistent.
    Consistent,

    /// Evidence contains contradictory safety-critical observations.
    Conflicting,

    /// Consistency could not be established.
    Unknown,
}

impl EvidenceConsistency {
    /// Stable machine-readable representation.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Consistent => "consistent",
            Self::Conflicting => "conflicting",
            Self::Unknown => "unknown",
        }
    }

    /// Returns whether evidence is explicitly consistent.
    pub const fn is_consistent(self) -> bool {
        matches!(self, Self::Consistent)
    }
}

impl fmt::Display for EvidenceConsistency {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Safety state
// =============================================================================

/// Explicit safety state of the proposed action/execution.
///
/// `Unknown` is deliberately different from `Safe`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SafetyState {
    /// The action has been established as safe under the supplied evidence.
    Safe,

    /// The action/execution has been established as unsafe.
    Unsafe,

    /// Safety has not been established.
    Unknown,
}

impl SafetyState {
    /// Stable machine-readable representation.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Safe => "safe",
            Self::Unsafe => "unsafe",
            Self::Unknown => "unknown",
        }
    }

    /// Returns whether safety is explicitly established.
    pub const fn is_safe(self) -> bool {
        matches!(self, Self::Safe)
    }

    /// Returns whether unsafety is explicitly established.
    pub const fn is_unsafe(self) -> bool {
        matches!(self, Self::Unsafe)
    }

    /// Returns whether safety is unknown.
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
// Authorization
// =============================================================================

/// Authorization state supplied by the owning security/policy boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum AuthorizationState {
    /// Authorization has not been established.
    Unknown,

    /// The requested action is explicitly denied.
    Denied,

    /// The requested action is explicitly authorized.
    Granted,
}

impl AuthorizationState {
    /// Stable machine-readable representation.
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

/// Semantic preservation state for the proposed action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SemanticState {
    /// Required semantics are explicitly preserved.
    Preserved,

    /// The action is known to violate required semantics.
    Violated,

    /// Preservation has not been established.
    Unknown,
}

impl SemanticState {
    /// Stable machine-readable representation.
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

    /// Returns whether semantics are explicitly violated.
    pub const fn is_violated(self) -> bool {
        matches!(self, Self::Violated)
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

/// Required capability state supplied by the hardware/capability subsystem.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CapabilityState {
    /// Required capabilities are available.
    Available,

    /// At least one mandatory capability is unavailable.
    Unavailable,

    /// Capability state is unknown/incomplete.
    Unknown,
}

impl CapabilityState {
    /// Stable machine-readable representation.
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

/// Required resource state supplied by the resource/capability subsystem.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ResourceState {
    /// Required resources are available.
    Available,

    /// Required resources are unavailable.
    Unavailable,

    /// Resource state is unknown/incomplete.
    Unknown,
}

impl ResourceState {
    /// Stable machine-readable representation.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Available => "available",
            Self::Unavailable => "unavailable",
            Self::Unknown => "unknown",
        }
    }

    /// Returns whether resources are explicitly available.
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

/// Budget state supplied by `policy::budgets`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum BudgetState {
    /// All mandatory applicable budgets remain available.
    Available,

    /// At least one mandatory budget is exhausted.
    Exhausted,

    /// Budget state is unknown/incomplete.
    Unknown,
}

impl BudgetState {
    /// Stable machine-readable representation.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Available => "available",
            Self::Exhausted => "exhausted",
            Self::Unknown => "unknown",
        }
    }

    /// Returns whether the applicable budgets remain available.
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
// Provenance
// =============================================================================

/// Provenance state for the safety-critical execution context.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ProvenanceState {
    /// Required provenance is complete.
    Complete,

    /// Some provenance exists but required information is missing.
    Incomplete,

    /// Provenance cannot be established.
    Unknown,

    /// Provenance has been explicitly identified as invalid.
    Invalid,
}

impl ProvenanceState {
    /// Stable machine-readable representation.
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

    /// Returns whether provenance is invalid.
    pub const fn is_invalid(self) -> bool {
        matches!(self, Self::Invalid)
    }
}

impl fmt::Display for ProvenanceState {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Freshness
// =============================================================================

/// Freshness state of the supplied safety-critical snapshot.
///
/// Freshness is supplied by the observation/telemetry/state subsystem. This
/// module does not read a clock.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum FreshnessState {
    /// Evidence is explicitly considered fresh for this decision.
    Fresh,

    /// Evidence is explicitly stale.
    Stale,

    /// Freshness cannot be established.
    Unknown,
}

impl FreshnessState {
    /// Stable machine-readable representation.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Fresh => "fresh",
            Self::Stale => "stale",
            Self::Unknown => "unknown",
        }
    }

    /// Returns whether evidence is explicitly fresh.
    pub const fn is_fresh(self) -> bool {
        matches!(self, Self::Fresh)
    }

    /// Returns whether evidence is explicitly stale.
    pub const fn is_stale(self) -> bool {
        matches!(self, Self::Stale)
    }
}

impl fmt::Display for FreshnessState {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Verification requirement
// =============================================================================

/// Verification requirement attached to the safety decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum VerificationRequirement {
    /// No additional verification requirement originates from this safety
    /// policy.
    None,

    /// Verification is required before the result may be accepted.
    RequiredBeforeAcceptance,

    /// Verification is required before the action may start.
    RequiredBeforeExecution,
}

impl VerificationRequirement {
    /// Stable machine-readable representation.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::RequiredBeforeAcceptance => "required_before_acceptance",
            Self::RequiredBeforeExecution => "required_before_execution",
        }
    }

    /// Returns whether verification is required.
    pub const fn is_required(self) -> bool {
        !matches!(self, Self::None)
    }

    /// Returns whether verification must occur before execution.
    pub const fn is_required_before_execution(self) -> bool {
        matches!(self, Self::RequiredBeforeExecution)
    }

    /// Returns the stronger of two verification requirements.
    pub const fn strongest(
        left: Self,
        right: Self,
    ) -> Self {
        match (left, right) {
            (
                Self::RequiredBeforeExecution,
                _,
            )
            | (
                _,
                Self::RequiredBeforeExecution,
            ) => Self::RequiredBeforeExecution,

            (
                Self::RequiredBeforeAcceptance,
                _
            )
            | (
                _,
                Self::RequiredBeforeAcceptance,
            ) => Self::RequiredBeforeAcceptance,

            (Self::None, Self::None) => Self::None,
        }
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

/// Configurable requirements used by [`SafetyPolicy`].
///
/// This type contains policy configuration only. It contains no hardware,
/// topology, qubit counts, provider names, retry loops or implementation logic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SafetyRequirement {
    require_trusted_evidence: bool,
    require_consistent_evidence: bool,
    require_fresh_evidence: bool,
    require_semantic_preservation: bool,
    require_authorization: bool,
    require_capability: bool,
    require_resources: bool,
    require_budget: bool,
    require_provenance: bool,
    require_verification_for_mutating_actions: bool,
    require_verification_for_abort: bool,
    allow_abort_on_unsafe_condition: bool,
    allow_observe_without_authorization: bool,
}

impl Default for SafetyRequirement {
    fn default() -> Self {
        Self::strict()
    }
}

impl SafetyRequirement {
    /// Production fail-closed configuration.
    ///
    /// Every safety-critical condition is required unless the caller
    /// explicitly supplies a different policy.
    pub const fn strict() -> Self {
        Self {
            require_trusted_evidence: true,
            require_consistent_evidence: true,
            require_fresh_evidence: true,
            require_semantic_preservation: true,
            require_authorization: true,
            require_capability: true,
            require_resources: true,
            require_budget: true,
            require_provenance: true,
            require_verification_for_mutating_actions: true,
            require_verification_for_abort: false,
            allow_abort_on_unsafe_condition: true,
            allow_observe_without_authorization: true,
        }
    }

    /// Controlled policy configuration.
    ///
    /// This is NOT an "unsafe mode". Hard safety failures remain hard failures.
    ///
    /// This constructor exists for explicitly controlled environments where
    /// some external constraints are deliberately enforced by another
    /// authoritative layer.
    pub const fn permissive() -> Self {
        Self {
            require_trusted_evidence: false,
            require_consistent_evidence: true,
            require_fresh_evidence: true,
            require_semantic_preservation: true,
            require_authorization: true,
            require_capability: true,
            require_resources: false,
            require_budget: false,
            require_provenance: false,
            require_verification_for_mutating_actions: true,
            require_verification_for_abort: false,
            allow_abort_on_unsafe_condition: true,
            allow_observe_without_authorization: true,
        }
    }

    /// Returns whether trusted evidence is required.
    pub const fn requires_trusted_evidence(self) -> bool {
        self.require_trusted_evidence
    }

    /// Returns whether evidence consistency is required.
    pub const fn requires_consistent_evidence(self) -> bool {
        self.require_consistent_evidence
    }

    /// Returns whether fresh evidence is required.
    pub const fn requires_fresh_evidence(self) -> bool {
        self.require_fresh_evidence
    }

    /// Returns whether semantic preservation is required.
    pub const fn requires_semantic_preservation(self) -> bool {
        self.require_semantic_preservation
    }

    /// Returns whether authorization is required.
    pub const fn requires_authorization(self) -> bool {
        self.require_authorization
    }

    /// Returns whether capability availability is required.
    pub const fn requires_capability(self) -> bool {
        self.require_capability
    }

    /// Returns whether resource availability is required.
    pub const fn requires_resources(self) -> bool {
        self.require_resources
    }

    /// Returns whether budget availability is required.
    pub const fn requires_budget(self) -> bool {
        self.require_budget
    }

    /// Returns whether complete provenance is required.
    pub const fn requires_provenance(self) -> bool {
        self.require_provenance
    }

    /// Returns whether mutating actions require verification.
    pub const fn requires_verification_for_mutating_actions(self) -> bool {
        self.require_verification_for_mutating_actions
    }

    /// Returns whether abort requires verification.
    pub const fn requires_verification_for_abort(self) -> bool {
        self.require_verification_for_abort
    }

    /// Returns whether abort may be authorized when the execution is unsafe.
    pub const fn allows_abort_on_unsafe_condition(self) -> bool {
        self.allow_abort_on_unsafe_condition
    }

    /// Returns whether observation can occur without explicit authorization.
    pub const fn allows_observe_without_authorization(self) -> bool {
        self.allow_observe_without_authorization
    }

    /// Changes trusted-evidence requirement.
    pub const fn with_trusted_evidence(self, value: bool) -> Self {
        Self {
            require_trusted_evidence: value,
            ..self
        }
    }

    /// Changes evidence-consistency requirement.
    pub const fn with_consistent_evidence(self, value: bool) -> Self {
        Self {
            require_consistent_evidence: value,
            ..self
        }
    }

    /// Changes freshness requirement.
    pub const fn with_fresh_evidence(self, value: bool) -> Self {
        Self {
            require_fresh_evidence: value,
            ..self
        }
    }

    /// Changes semantic-preservation requirement.
    pub const fn with_semantic_preservation(self, value: bool) -> Self {
        Self {
            require_semantic_preservation: value,
            ..self
        }
    }

    /// Changes authorization requirement.
    pub const fn with_authorization(self, value: bool) -> Self {
        Self {
            require_authorization: value,
            ..self
        }
    }

    /// Changes capability requirement.
    pub const fn with_capability(self, value: bool) -> Self {
        Self {
            require_capability: value,
            ..self
        }
    }

    /// Changes resource requirement.
    pub const fn with_resources(self, value: bool) -> Self {
        Self {
            require_resources: value,
            ..self
        }
    }

    /// Changes budget requirement.
    pub const fn with_budget(self, value: bool) -> Self {
        Self {
            require_budget: value,
            ..self
        }
    }

    /// Changes provenance requirement.
    pub const fn with_provenance(self, value: bool) -> Self {
        Self {
            require_provenance: value,
            ..self
        }
    }

    /// Changes mutation-verification requirement.
    pub const fn with_verification_for_mutating_actions(
        self,
        value: bool,
    ) -> Self {
        Self {
            require_verification_for_mutating_actions: value,
            ..self
        }
    }

    /// Changes abort-verification requirement.
    pub const fn with_verification_for_abort(self, value: bool) -> Self {
        Self {
            require_verification_for_abort: value,
            ..self
        }
    }

    /// Changes whether unsafe execution may be stopped by an authorized abort.
    pub const fn with_abort_on_unsafe_condition(self, value: bool) -> Self {
        Self {
            allow_abort_on_unsafe_condition: value,
            ..self
        }
    }

    /// Changes whether observation may occur without authorization.
    pub const fn with_observe_without_authorization(self, value: bool) -> Self {
        Self {
            allow_observe_without_authorization: value,
            ..self
        }
    }
}

// =============================================================================
// Safety action
// =============================================================================

/// Semantic class of a proposed resilience action.
///
/// Implementations remain in the recovery/adaptation/mitigation subsystems.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SafetyAction {
    /// Observe state without changing execution.
    Observe,

    /// Retry an execution attempt.
    Retry,

    /// Restart from an established execution boundary.
    Restart,

    /// Resume from an established checkpoint/boundary.
    Resume,

    /// Roll back to an established valid state.
    Rollback,

    /// Change logical-to-physical mapping.
    Remap,

    /// Change physical routing.
    Reroute,

    /// Rebuild execution scheduling.
    Reschedule,

    /// Recompile the affected computation.
    Recompile,

    /// Re-run optimization against a changed target.
    Reoptimize,

    /// Adapt QEC configuration.
    AdaptQec,

    /// Apply error mitigation.
    Mitigate,

    /// Migrate execution to another compatible resource.
    Migrate,

    /// Apply a mathematically defined compensation action.
    Compensate,

    /// Stop execution as a protective action.
    Abort,
}

impl SafetyAction {
    /// Stable machine-readable representation.
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

    /// Returns whether the action changes execution state or configuration.
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

    /// Returns whether the action belongs to the recovery family.
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

    /// Returns whether this is a protective action.
    pub const fn is_protective(self) -> bool {
        matches!(self, Self::Abort)
    }

    /// Returns the default verification requirement for this action.
    ///
    /// The final requirement is the stronger of this value and the caller's
    /// explicit verification contract.
    pub const fn default_verification_requirement(
        self,
    ) -> VerificationRequirement {
        match self {
            Self::Observe => VerificationRequirement::None,
            Self::Abort => VerificationRequirement::None,
            Self::Retry
            | Self::Restart
            | Self::Resume
            | Self::Rollback
            | Self::Remap
            | Self::Reroute
            | Self::Reschedule
            | Self::Recompile
            | Self::Reoptimize
            | Self::AdaptQec
            | Self::Mitigate
            | Self::Migrate
            | Self::Compensate => {
                VerificationRequirement::RequiredBeforeAcceptance
            }
        }
    }
}

impl fmt::Display for SafetyAction {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Action authorization
// =============================================================================

/// Explicit action authorization.
///
/// The safety policy must never infer permission merely from the existence of
/// a planner recommendation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ActionAuthorization {
    /// The action has not been explicitly authorized.
    Unknown,

    /// The action is explicitly forbidden.
    Forbidden,

    /// The action is explicitly authorized for safety evaluation.
    Allowed,
}

impl ActionAuthorization {
    /// Stable machine-readable representation.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unknown => "unknown",
            Self::Forbidden => "forbidden",
            Self::Allowed => "allowed",
        }
    }

    /// Returns whether the action is explicitly allowed.
    pub const fn is_allowed(self) -> bool {
        matches!(self, Self::Allowed)
    }

    /// Returns whether the action is explicitly forbidden.
    pub const fn is_forbidden(self) -> bool {
        matches!(self, Self::Forbidden)
    }
}

impl fmt::Display for ActionAuthorization {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Safety decision
// =============================================================================

/// Final decision emitted by the safety-policy gate.
///
/// `Allow` means only that the safety layer has passed the action onward.
/// It does NOT mean that the action should be executed.
///
/// Planning, feasibility, execution and post-execution verification remain
/// separate gates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SafetyDecision {
    /// The action may pass to subsequent planning/execution gates.
    Allow,

    /// The action must not proceed.
    Deny,

    /// More safety-critical evidence is required.
    RequireEvidence,

    /// Autonomous authorization is not appropriate; higher authority is
    /// required.
    Escalate,
}

impl SafetyDecision {
    /// Stable machine-readable representation.
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

    /// Returns whether additional evidence is required.
    pub const fn requires_evidence(self) -> bool {
        matches!(self, Self::RequireEvidence)
    }

    /// Returns whether escalation is required.
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

/// Stable machine-readable explanation for the safety decision.
///
/// The enum intentionally avoids provider-specific terminology.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SafetyReason {
    /// Every applicable safety check passed.
    AllChecksPassed,

    /// The action was not explicitly authorized.
    ActionAuthorizationUnknown,

    /// The action was explicitly forbidden.
    ActionForbidden,

    /// Explicit unsafe condition exists.
    UnsafeCondition,

    /// Required safety evidence is missing.
    SafetyEvidenceUnknown,

    /// Evidence cannot be trusted.
    EvidenceUntrusted,

    /// Evidence is contradictory.
    EvidenceConflicting,

    /// Evidence freshness is unknown.
    EvidenceFreshnessUnknown,

    /// Evidence is stale.
    EvidenceStale,

    /// Authorization is denied.
    AuthorizationDenied,

    /// Authorization is unknown.
    AuthorizationUnknown,

    /// Required semantic preservation was not established.
    SemanticPreservationUnknown,

    /// Proposed action violates semantics.
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

    /// Required verification contract is absent.
    VerificationRequired,

    /// Abort verification is required but absent.
    AbortVerificationRequired,

    /// Explicit policy conditions conflict.
    PolicyConflict,

    /// State is stale or internally inconsistent.
    StaleState,

    /// External authority must decide.
    EscalationRequired,
}

impl SafetyReason {
    /// Stable machine-readable representation.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AllChecksPassed => "all_checks_passed",
            Self::ActionAuthorizationUnknown => "action_authorization_unknown",
            Self::ActionForbidden => "action_forbidden",
            Self::UnsafeCondition => "unsafe_condition",
            Self::SafetyEvidenceUnknown => "safety_evidence_unknown",
            Self::EvidenceUntrusted => "evidence_untrusted",
            Self::EvidenceConflicting => "evidence_conflicting",
            Self::EvidenceFreshnessUnknown => "evidence_freshness_unknown",
            Self::EvidenceStale => "evidence_stale",
            Self::AuthorizationDenied => "authorization_denied",
            Self::AuthorizationUnknown => "authorization_unknown",
            Self::SemanticPreservationUnknown => {
                "semantic_preservation_unknown"
            }
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
            Self::AbortVerificationRequired => {
                "abort_verification_required"
            }
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

/// Complete immutable result of a safety evaluation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
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

    /// Returns the final safety decision.
    pub const fn decision(self) -> SafetyDecision {
        self.decision
    }

    /// Returns the stable machine-readable reason.
    pub const fn reason(self) -> SafetyReason {
        self.reason
    }

    /// Returns the verification requirement attached to the decision.
    pub const fn verification_requirement(
        self,
    ) -> VerificationRequirement {
        self.verification
    }

    /// Returns whether the safety layer authorized the action.
    pub const fn is_allowed(self) -> bool {
        self.decision.is_allowed()
    }

    /// Returns whether additional evidence is required.
    pub const fn requires_evidence(self) -> bool {
        self.decision.requires_evidence()
    }

    /// Returns whether external escalation is required.
    pub const fn requires_escalation(self) -> bool {
        self.decision.requires_escalation()
    }
}

// =============================================================================
// Safety context
// =============================================================================

/// Immutable normalized evidence snapshot consumed by [`SafetyPolicy`].
///
/// The context deliberately contains policy facts rather than concrete
/// hardware, ZQN, QEC, routing, scheduling or verification implementations.
///
/// Those owning subsystems produce these normalized facts.
///
/// The context is `Copy`, making evaluation cheap and deterministic even for
/// very large quantum systems: safety policy does not copy circuits, qubits,
/// topologies or result sets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SafetyContext {
    action: SafetyAction,
    action_authorization: ActionAuthorization,
    safety: SafetyState,
    evidence_trust: EvidenceTrust,
    evidence_consistency: EvidenceConsistency,
    freshness: FreshnessState,
    authorization: AuthorizationState,
    semantic: SemanticState,
    capability: CapabilityState,
    resources: ResourceState,
    budget: BudgetState,
    provenance: ProvenanceState,
    verification: VerificationRequirement,
    explicit_escalation: bool,
}

impl SafetyContext {
    /// Creates a conservative context.
    ///
    /// The resulting context is NOT executable.
    ///
    /// Every safety-critical fact defaults to unknown and the action defaults
    /// to unauthorized.
    pub const fn new(action: SafetyAction) -> Self {
        Self {
            action,
            action_authorization: ActionAuthorization::Unknown,
            safety: SafetyState::Unknown,
            evidence_trust: EvidenceTrust::Unknown,
            evidence_consistency: EvidenceConsistency::Unknown,
            freshness: FreshnessState::Unknown,
            authorization: AuthorizationState::Unknown,
            semantic: SemanticState::Unknown,
            capability: CapabilityState::Unknown,
            resources: ResourceState::Unknown,
            budget: BudgetState::Unknown,
            provenance: ProvenanceState::Unknown,
            verification: VerificationRequirement::None,
            explicit_escalation: false,
        }
    }

    /// Returns the proposed action.
    pub const fn action(self) -> SafetyAction {
        self.action
    }

    /// Returns explicit action authorization.
    pub const fn action_authorization(self) -> ActionAuthorization {
        self.action_authorization
    }

    /// Returns safety state.
    pub const fn safety(self) -> SafetyState {
        self.safety
    }

    /// Returns evidence trust.
    pub const fn evidence_trust(self) -> EvidenceTrust {
        self.evidence_trust
    }

    /// Returns evidence consistency.
    pub const fn evidence_consistency(self) -> EvidenceConsistency {
        self.evidence_consistency
    }

    /// Returns evidence freshness.
    pub const fn freshness(self) -> FreshnessState {
        self.freshness
    }

    /// Returns authorization state.
    pub const fn authorization(self) -> AuthorizationState {
        self.authorization
    }

    /// Returns semantic state.
    pub const fn semantic(self) -> SemanticState {
        self.semantic
    }

    /// Returns capability state.
    pub const fn capability(self) -> CapabilityState {
        self.capability
    }

    /// Returns resource state.
    pub const fn resources(self) -> ResourceState {
        self.resources
    }

    /// Returns budget state.
    pub const fn budget(self) -> BudgetState {
        self.budget
    }

    /// Returns provenance state.
    pub const fn provenance(self) -> ProvenanceState {
        self.provenance
    }

    /// Returns verification requirement.
    pub const fn verification(self) -> VerificationRequirement {
        self.verification
    }

    /// Returns whether explicit escalation was requested.
    pub const fn explicit_escalation(self) -> bool {
        self.explicit_escalation
    }

    /// Sets explicit action authorization.
    pub const fn with_action_authorization(
        self,
        value: ActionAuthorization,
    ) -> Self {
        Self {
            action_authorization: value,
            ..self
        }
    }

    /// Sets safety state.
    pub const fn with_safety(self, value: SafetyState) -> Self {
        Self {
            safety: value,
            ..self
        }
    }

    /// Sets evidence trust.
    pub const fn with_evidence_trust(
        self,
        value: EvidenceTrust,
    ) -> Self {
        Self {
            evidence_trust: value,
            ..self
        }
    }

    /// Sets evidence consistency.
    pub const fn with_evidence_consistency(
        self,
        value: EvidenceConsistency,
    ) -> Self {
        Self {
            evidence_consistency: value,
            ..self
        }
    }

    /// Sets evidence freshness.
    pub const fn with_freshness(
        self,
        value: FreshnessState,
    ) -> Self {
        Self {
            freshness: value,
            ..self
        }
    }

    /// Sets authorization.
    pub const fn with_authorization(
        self,
        value: AuthorizationState,
    ) -> Self {
        Self {
            authorization: value,
            ..self
        }
    }

    /// Sets semantic state.
    pub const fn with_semantic(
        self,
        value: SemanticState,
    ) -> Self {
        Self {
            semantic: value,
            ..self
        }
    }

    /// Sets capability state.
    pub const fn with_capability(
        self,
        value: CapabilityState,
    ) -> Self {
        Self {
            capability: value,
            ..self
        }
    }

    /// Sets resource state.
    pub const fn with_resources(
        self,
        value: ResourceState,
    ) -> Self {
        Self {
            resources: value,
            ..self
        }
    }

    /// Sets budget state.
    pub const fn with_budget(
        self,
        value: BudgetState,
    ) -> Self {
        Self {
            budget: value,
            ..self
        }
    }

    /// Sets provenance state.
    pub const fn with_provenance(
        self,
        value: ProvenanceState,
    ) -> Self {
        Self {
            provenance: value,
            ..self
        }
    }

    /// Sets verification requirement.
    pub const fn with_verification(
        self,
        value: VerificationRequirement,
    ) -> Self {
        Self {
            verification: value,
            ..self
        }
    }

    /// Sets explicit escalation.
    pub const fn with_explicit_escalation(
        self,
        value: bool,
    ) -> Self {
        Self {
            explicit_escalation: value,
            ..self
        }
    }

    /// Produces the fully populated baseline for a normal safe action.
    ///
    /// This helper is intended for tests and controlled callers. Production
    /// orchestration should populate each field from its owning subsystem.
    pub const fn authorized_safe(
        action: SafetyAction,
    ) -> Self {
        Self::new(action)
            .with_action_authorization(ActionAuthorization::Allowed)
            .with_safety(SafetyState::Safe)
            .with_evidence_trust(EvidenceTrust::Trusted)
            .with_evidence_consistency(EvidenceConsistency::Consistent)
            .with_freshness(FreshnessState::Fresh)
            .with_authorization(AuthorizationState::Granted)
            .with_semantic(SemanticState::Preserved)
            .with_capability(CapabilityState::Available)
            .with_resources(ResourceState::Available)
            .with_budget(BudgetState::Available)
            .with_provenance(ProvenanceState::Complete)
            .with_verification(
                VerificationRequirement::RequiredBeforeAcceptance,
            )
    }
}

// =============================================================================
// Safety policy
// =============================================================================

/// Deterministic fail-closed safety policy.
///
/// The policy is deliberately small. It does not execute actions and does not
/// contain an optimization algorithm.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SafetyPolicy {
    requirements: SafetyRequirement,
}

impl Default for SafetyPolicy {
    fn default() -> Self {
        Self::strict()
    }
}

impl SafetyPolicy {
    /// Creates the production fail-closed policy.
    pub const fn strict() -> Self {
        Self {
            requirements: SafetyRequirement::strict(),
        }
    }

    /// Creates a policy from explicit requirements.
    pub const fn new(
        requirements: SafetyRequirement,
    ) -> Self {
        Self { requirements }
    }

    /// Returns the configured requirements.
    pub const fn requirements(self) -> SafetyRequirement {
        self.requirements
    }

    /// Evaluates one immutable safety snapshot.
    ///
    /// Evaluation order is deliberately security-oriented:
    ///
    /// 1. explicit action prohibition;
    /// 2. explicit protective abort handling;
    /// 3. explicit escalation;
    /// 4. evidence consistency;
    /// 5. evidence trust;
    /// 6. freshness;
    /// 7. authorization;
    /// 8. semantic preservation;
    /// 9. capability;
    /// 10. resources;
    /// 11. budget;
    /// 12. provenance;
    /// 13. verification contract;
    /// 14. final safety state.
    ///
    /// No objective score or planner ranking participates in this decision.
    pub const fn evaluate(
        self,
        context: SafetyContext,
    ) -> SafetyEvaluation {
        let required_verification =
            effective_verification(context);

        // ---------------------------------------------------------------------
        // Action authorization is a hard boundary.
        // ---------------------------------------------------------------------

        match context.action_authorization {
            ActionAuthorization::Forbidden => {
                return SafetyEvaluation::new(
                    SafetyDecision::Deny,
                    SafetyReason::ActionForbidden,
                    required_verification,
                );
            }
            ActionAuthorization::Unknown => {
                return SafetyEvaluation::new(
                    SafetyDecision::RequireEvidence,
                    SafetyReason::ActionAuthorizationUnknown,
                    required_verification,
                );
            }
            ActionAuthorization::Allowed => {}
        }

        // ---------------------------------------------------------------------
        // Protective abort.
        //
        // An unsafe execution must not prevent an explicitly authorized abort.
        // Other safety checks still apply.
        // ---------------------------------------------------------------------

        if context.action.is_protective()
            && context.safety.is_unsafe()
            && self
                .requirements
                .allows_abort_on_unsafe_condition()
        {
            return self.evaluate_abort(context, required_verification);
        }

        // ---------------------------------------------------------------------
        // Explicit escalation.
        // ---------------------------------------------------------------------

        if context.explicit_escalation {
            return SafetyEvaluation::new(
                SafetyDecision::Escalate,
                SafetyReason::EscalationRequired,
                required_verification,
            );
        }

        // ---------------------------------------------------------------------
        // Evidence consistency.
        // ---------------------------------------------------------------------

        if self
            .requirements
            .requires_consistent_evidence()
        {
            match context.evidence_consistency {
                EvidenceConsistency::Consistent => {}
                EvidenceConsistency::Conflicting => {
                    return SafetyEvaluation::new(
                        SafetyDecision::RequireEvidence,
                        SafetyReason::EvidenceConflicting,
                        required_verification,
                    );
                }
                EvidenceConsistency::Unknown => {
                    return SafetyEvaluation::new(
                        SafetyDecision::RequireEvidence,
                        SafetyReason::PolicyConflict,
                        required_verification,
                    );
                }
            }
        }

        // ---------------------------------------------------------------------
        // Evidence trust.
        // ---------------------------------------------------------------------

        if self
            .requirements
            .requires_trusted_evidence()
        {
            match context.evidence_trust {
                EvidenceTrust::Trusted => {}
                EvidenceTrust::Untrusted => {
                    return SafetyEvaluation::new(
                        SafetyDecision::RequireEvidence,
                        SafetyReason::EvidenceUntrusted,
                        required_verification,
                    );
                }
                EvidenceTrust::Unknown => {
                    return SafetyEvaluation::new(
                        SafetyDecision::RequireEvidence,
                        SafetyReason::SafetyEvidenceUnknown,
                        required_verification,
                    );
                }
            }
        }

        // ---------------------------------------------------------------------
        // Freshness.
        // ---------------------------------------------------------------------

        if self
            .requirements
            .requires_fresh_evidence()
        {
            match context.freshness {
                FreshnessState::Fresh => {}
                FreshnessState::Stale => {
                    return SafetyEvaluation::new(
                        SafetyDecision::RequireEvidence,
                        SafetyReason::EvidenceStale,
                        required_verification,
                    );
                }
                FreshnessState::Unknown => {
                    return SafetyEvaluation::new(
                        SafetyDecision::RequireEvidence,
                        SafetyReason::EvidenceFreshnessUnknown,
                        required_verification,
                    );
                }
            }
        }

        // ---------------------------------------------------------------------
        // Explicit unsafe condition.
        //
        // At this point only a protective abort has the special escape path.
        // Ordinary actions are denied.
        // ---------------------------------------------------------------------

        if context.safety.is_unsafe() {
            return SafetyEvaluation::new(
                SafetyDecision::Deny,
                SafetyReason::UnsafeCondition,
                required_verification,
            );
        }

        // ---------------------------------------------------------------------
        // Authorization.
        //
        // Observation may be permitted without execution authorization when
        // the policy explicitly allows it.
        // ---------------------------------------------------------------------

        if self
            .requirements
            .requires_authorization()
        {
            if !(context.action == SafetyAction::Observe
                && self
                    .requirements
                    .allows_observe_without_authorization())
            {
                match context.authorization {
                    AuthorizationState::Granted => {}
                    AuthorizationState::Denied => {
                        return SafetyEvaluation::new(
                            SafetyDecision::Deny,
                            SafetyReason::AuthorizationDenied,
                            required_verification,
                        );
                    }
                    AuthorizationState::Unknown => {
                        return SafetyEvaluation::new(
                            SafetyDecision::RequireEvidence,
                            SafetyReason::AuthorizationUnknown,
                            required_verification,
                        );
                    }
                }
            }
        }

        // ---------------------------------------------------------------------
        // Semantic correctness.
        //
        // Observation and abort do not require preservation of the computation
        // in the same way that execution-changing actions do.
        // ---------------------------------------------------------------------

        if self
            .requirements
            .requires_semantic_preservation()
            && requires_semantic_preservation(context.action)
        {
            match context.semantic {
                SemanticState::Preserved => {}
                SemanticState::Violated => {
                    return SafetyEvaluation::new(
                        SafetyDecision::Deny,
                        SafetyReason::SemanticViolation,
                        required_verification,
                    );
                }
                SemanticState::Unknown => {
                    return SafetyEvaluation::new(
                        SafetyDecision::RequireEvidence,
                        SafetyReason::SemanticPreservationUnknown,
                        required_verification,
                    );
                }
            }
        }

        // ---------------------------------------------------------------------
        // Capability.
        // ---------------------------------------------------------------------

        if self
            .requirements
            .requires_capability()
            && requires_capability(context.action)
        {
            match context.capability {
                CapabilityState::Available => {}
                CapabilityState::Unavailable => {
                    return SafetyEvaluation::new(
                        SafetyDecision::Deny,
                        SafetyReason::CapabilityUnavailable,
                        required_verification,
                    );
                }
                CapabilityState::Unknown => {
                    return SafetyEvaluation::new(
                        SafetyDecision::RequireEvidence,
                        SafetyReason::CapabilityUnknown,
                        required_verification,
                    );
                }
            }
        }

        // ---------------------------------------------------------------------
        // Resources.
        // ---------------------------------------------------------------------

        if self
            .requirements
            .requires_resources()
            && requires_resources(context.action)
        {
            match context.resources {
                ResourceState::Available => {}
                ResourceState::Unavailable => {
                    return SafetyEvaluation::new(
                        SafetyDecision::Deny,
                        SafetyReason::ResourceUnavailable,
                        required_verification,
                    );
                }
                ResourceState::Unknown => {
                    return SafetyEvaluation::new(
                        SafetyDecision::RequireEvidence,
                        SafetyReason::ResourceUnknown,
                        required_verification,
                    );
                }
            }
        }

        // ---------------------------------------------------------------------
        // Budgets.
        //
        // Abort is a protective action and must not be blocked merely because
        // the computation's ordinary recovery budget has been exhausted.
        // ---------------------------------------------------------------------

        if self
            .requirements
            .requires_budget()
            && requires_budget(context.action)
        {
            match context.budget {
                BudgetState::Available => {}
                BudgetState::Exhausted => {
                    return SafetyEvaluation::new(
                        SafetyDecision::Deny,
                        SafetyReason::BudgetExhausted,
                        required_verification,
                    );
                }
                BudgetState::Unknown => {
                    return SafetyEvaluation::new(
                        SafetyDecision::RequireEvidence,
                        SafetyReason::BudgetUnknown,
                        required_verification,
                    );
                }
            }
        }

        // ---------------------------------------------------------------------
        // Provenance.
        // ---------------------------------------------------------------------

        if self
            .requirements
            .requires_provenance()
            && requires_provenance(context.action)
        {
            match context.provenance {
                ProvenanceState::Complete => {}
                ProvenanceState::Incomplete => {
                    return SafetyEvaluation::new(
                        SafetyDecision::RequireEvidence,
                        SafetyReason::ProvenanceIncomplete,
                        required_verification,
                    );
                }
                ProvenanceState::Unknown => {
                    return SafetyEvaluation::new(
                        SafetyDecision::RequireEvidence,
                        SafetyReason::ProvenanceUnknown,
                        required_verification,
                    );
                }
                ProvenanceState::Invalid => {
                    return SafetyEvaluation::new(
                        SafetyDecision::Deny,
                        SafetyReason::ProvenanceInvalid,
                        required_verification,
                    );
                }
            }
        }

        // ---------------------------------------------------------------------
        // Verification contract.
        // ---------------------------------------------------------------------

        if context.action == SafetyAction::Abort {
            if self
                .requirements
                .requires_verification_for_abort()
                && !context.verification.is_required()
            {
                return SafetyEvaluation::new(
                    SafetyDecision::RequireEvidence,
                    SafetyReason::AbortVerificationRequired,
                    VerificationRequirement::RequiredBeforeExecution,
                );
            }
        } else if context.action.is_mutating()
            && self
                .requirements
                .requires_verification_for_mutating_actions()
            && !context.verification.is_required()
        {
            return SafetyEvaluation::new(
                SafetyDecision::RequireEvidence,
                SafetyReason::VerificationRequired,
                VerificationRequirement::RequiredBeforeAcceptance,
            );
        }

        // ---------------------------------------------------------------------
        // Final safety state.
        // ---------------------------------------------------------------------

        match context.safety {
            SafetyState::Safe => {}
            SafetyState::Unsafe => {
                return SafetyEvaluation::new(
                    SafetyDecision::Deny,
                    SafetyReason::UnsafeCondition,
                    required_verification,
                );
            }
            SafetyState::Unknown => {
                return SafetyEvaluation::new(
                    SafetyDecision::RequireEvidence,
                    SafetyReason::SafetyEvidenceUnknown,
                    required_verification,
                );
            }
        }

        SafetyEvaluation::new(
            SafetyDecision::Allow,
            SafetyReason::AllChecksPassed,
            required_verification,
        )
    }

    /// Evaluates a protective abort.
    ///
    /// Abort is intentionally handled separately because an unsafe execution
    /// may make stopping the computation the safest action.
    const fn evaluate_abort(
        self,
        context: SafetyContext,
        verification: VerificationRequirement,
    ) -> SafetyEvaluation {
        if context.explicit_escalation {
            return SafetyEvaluation::new(
                SafetyDecision::Escalate,
                SafetyReason::EscalationRequired,
                verification,
            );
        }

        if self
            .requirements
            .requires_consistent_evidence()
        {
            match context.evidence_consistency {
                EvidenceConsistency::Consistent => {}
                EvidenceConsistency::Conflicting
                | EvidenceConsistency::Unknown => {
                    return SafetyEvaluation::new(
                        SafetyDecision::RequireEvidence,
                        SafetyReason::EvidenceConflicting,
                        verification,
                    );
                }
            }
        }

        if self
            .requirements
            .requires_trusted_evidence()
        {
            match context.evidence_trust {
                EvidenceTrust::Trusted => {}
                EvidenceTrust::Untrusted => {
                    return SafetyEvaluation::new(
                        SafetyDecision::RequireEvidence,
                        SafetyReason::EvidenceUntrusted,
                        verification,
                    );
                }
                EvidenceTrust::Unknown => {
                    return SafetyEvaluation::new(
                        SafetyDecision::RequireEvidence,
                        SafetyReason::SafetyEvidenceUnknown,
                        verification,
                    );
                }
            }
        }

        if self
            .requirements
            .requires_fresh_evidence()
        {
            match context.freshness {
                FreshnessState::Fresh => {}
                FreshnessState::Stale => {
                    return SafetyEvaluation::new(
                        SafetyDecision::RequireEvidence,
                        SafetyReason::EvidenceStale,
                        verification,
                    );
                }
                FreshnessState::Unknown => {
                    return SafetyEvaluation::new(
                        SafetyDecision::RequireEvidence,
                        SafetyReason::EvidenceFreshnessUnknown,
                        verification,
                    );
                }
            }
        }

        if self
            .requirements
            .requires_authorization()
        {
            match context.authorization {
                AuthorizationState::Granted => {}
                AuthorizationState::Denied => {
                    return SafetyEvaluation::new(
                        SafetyDecision::Deny,
                        SafetyReason::AuthorizationDenied,
                        verification,
                    );
                }
                AuthorizationState::Unknown => {
                    return SafetyEvaluation::new(
                        SafetyDecision::RequireEvidence,
                        SafetyReason::AuthorizationUnknown,
                        verification,
                    );
                }
            }
        }

        if self
            .requirements
            .requires_provenance()
        {
            match context.provenance {
                ProvenanceState::Complete => {}
                ProvenanceState::Incomplete => {
                    return SafetyEvaluation::new(
                        SafetyDecision::RequireEvidence,
                        SafetyReason::ProvenanceIncomplete,
                        verification,
                    );
                }
                ProvenanceState::Unknown => {
                    return SafetyEvaluation::new(
                        SafetyDecision::RequireEvidence,
                        SafetyReason::ProvenanceUnknown,
                        verification,
                    );
                }
                ProvenanceState::Invalid => {
                    return SafetyEvaluation::new(
                        SafetyDecision::Deny,
                        SafetyReason::ProvenanceInvalid,
                        verification,
                    );
                }
            }
        }

        if self
            .requirements
            .requires_verification_for_abort()
            && !context.verification.is_required()
        {
            return SafetyEvaluation::new(
                SafetyDecision::RequireEvidence,
                SafetyReason::AbortVerificationRequired,
                VerificationRequirement::RequiredBeforeExecution,
            );
        }

        SafetyEvaluation::new(
            SafetyDecision::Allow,
            SafetyReason::AllChecksPassed,
            verification,
        )
    }
}

// =============================================================================
// Helpers
// =============================================================================

/// Returns the effective verification requirement.
///
/// The stronger requirement always wins.
const fn effective_verification(
    context: SafetyContext,
) -> VerificationRequirement {
    VerificationRequirement::strongest(
        context.verification,
        context.action.default_verification_requirement(),
    )
}

/// Determines whether semantic preservation is applicable.
///
/// Abort protects execution rather than transforming the computation.
/// Observation does not transform computation.
const fn requires_semantic_preservation(
    action: SafetyAction,
) -> bool {
    !matches!(
        action,
        SafetyAction::Observe | SafetyAction::Abort
    )
}

/// Determines whether capability availability is applicable.
const fn requires_capability(action: SafetyAction) -> bool {
    !matches!(action, SafetyAction::Observe | SafetyAction::Abort)
}

/// Determines whether ordinary execution resources are applicable.
///
/// Abort is deliberately excluded because an emergency stop must not be
/// blocked by the computation's ordinary resource budget.
const fn requires_resources(action: SafetyAction) -> bool {
    !matches!(action, SafetyAction::Observe | SafetyAction::Abort)
}

/// Determines whether ordinary execution budgets are applicable.
const fn requires_budget(action: SafetyAction) -> bool {
    !matches!(action, SafetyAction::Observe | SafetyAction::Abort)
}

/// Determines whether provenance is applicable.
const fn requires_provenance(action: SafetyAction) -> bool {
    !matches!(action, SafetyAction::Observe)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    const fn safe_context(
        action: SafetyAction,
    ) -> SafetyContext {
        SafetyContext::authorized_safe(action)
    }

    #[test]
    fn strict_policy_allows_fully_established_safe_action() {
        let policy = SafetyPolicy::strict();

        let context = safe_context(SafetyAction::Retry);

        let result = policy.evaluate(context);

        assert_eq!(
            result.decision(),
            SafetyDecision::Allow
        );
        assert_eq!(
            result.reason(),
            SafetyReason::AllChecksPassed
        );
        assert_eq!(
            result.verification_requirement(),
            VerificationRequirement::RequiredBeforeAcceptance
        );
    }

    #[test]
    fn default_context_is_not_authorized() {
        let policy = SafetyPolicy::strict();

        let context =
            SafetyContext::new(SafetyAction::Retry);

        let result = policy.evaluate(context);

        assert_ne!(
            result.decision(),
            SafetyDecision::Allow
        );
    }

    #[test]
    fn action_must_be_explicitly_authorized() {
        let policy = SafetyPolicy::strict();

        let context = safe_context(SafetyAction::Retry)
            .with_action_authorization(
                ActionAuthorization::Unknown,
            );

        let result = policy.evaluate(context);

        assert_eq!(
            result.decision(),
            SafetyDecision::RequireEvidence
        );
        assert_eq!(
            result.reason(),
            SafetyReason::ActionAuthorizationUnknown
        );
    }

    #[test]
    fn forbidden_action_is_denied() {
        let policy = SafetyPolicy::strict();

        let context = safe_context(SafetyAction::Retry)
            .with_action_authorization(
                ActionAuthorization::Forbidden,
            );

        let result = policy.evaluate(context);

        assert_eq!(
            result.decision(),
            SafetyDecision::Deny
        );
        assert_eq!(
            result.reason(),
            SafetyReason::ActionForbidden
        );
    }

    #[test]
    fn unknown_safety_fails_closed() {
        let policy = SafetyPolicy::strict();

        let context = safe_context(SafetyAction::Retry)
            .with_safety(SafetyState::Unknown);

        let result = policy.evaluate(context);

        assert_eq!(
            result.decision(),
            SafetyDecision::RequireEvidence
        );
        assert_eq!(
            result.reason(),
            SafetyReason::SafetyEvidenceUnknown
        );
    }

    #[test]
    fn explicit_unsafe_condition_denies_normal_recovery() {
        let policy = SafetyPolicy::strict();

        let context = safe_context(SafetyAction::Retry)
            .with_safety(SafetyState::Unsafe);

        let result = policy.evaluate(context);

        assert_eq!(
            result.decision(),
            SafetyDecision::Deny
        );
        assert_eq!(
            result.reason(),
            SafetyReason::UnsafeCondition
        );
    }

    #[test]
    fn explicit_unsafe_condition_can_authorize_protective_abort() {
        let policy = SafetyPolicy::strict();

        let context = safe_context(SafetyAction::Abort)
            .with_safety(SafetyState::Unsafe)
            .with_verification(
                VerificationRequirement::None,
            );

        let result = policy.evaluate(context);

        assert_eq!(
            result.decision(),
            SafetyDecision::Allow
        );
        assert_eq!(
            result.reason(),
            SafetyReason::AllChecksPassed
        );
    }

    #[test]
    fn abort_still_requires_action_authorization() {
        let policy = SafetyPolicy::strict();

        let context = safe_context(SafetyAction::Abort)
            .with_safety(SafetyState::Unsafe)
            .with_action_authorization(
                ActionAuthorization::Forbidden,
            );

        let result = policy.evaluate(context);

        assert_eq!(
            result.decision(),
            SafetyDecision::Deny
        );
        assert_eq!(
            result.reason(),
            SafetyReason::ActionForbidden
        );
    }

    #[test]
    fn abort_still_requires_execution_authorization() {
        let policy = SafetyPolicy::strict();

        let context = safe_context(SafetyAction::Abort)
            .with_safety(SafetyState::Unsafe)
            .with_authorization(
                AuthorizationState::Denied,
            );

        let result = policy.evaluate(context);

        assert_eq!(
            result.decision(),
            SafetyDecision::Deny
        );
        assert_eq!(
            result.reason(),
            SafetyReason::AuthorizationDenied
        );
    }

    #[test]
    fn conflicting_evidence_fails_closed() {
        let policy = SafetyPolicy::strict();

        let context = safe_context(SafetyAction::Retry)
            .with_evidence_consistency(
                EvidenceConsistency::Conflicting,
            );

        let result = policy.evaluate(context);

        assert_eq!(
            result.decision(),
            SafetyDecision::RequireEvidence
        );
        assert_eq!(
            result.reason(),
            SafetyReason::EvidenceConflicting
        );
    }

    #[test]
    fn unknown_evidence_consistency_fails_closed() {
        let policy = SafetyPolicy::strict();

        let context = safe_context(SafetyAction::Retry)
            .with_evidence_consistency(
                EvidenceConsistency::Unknown,
            );

        let result = policy.evaluate(context);

        assert_eq!(
            result.decision(),
            SafetyDecision::RequireEvidence
        );
        assert_eq!(
            result.reason(),
            SafetyReason::PolicyConflict
        );
    }

    #[test]
    fn untrusted_evidence_does_not_authorize_action() {
        let policy = SafetyPolicy::strict();

        let context = safe_context(SafetyAction::Retry)
            .with_evidence_trust(
                EvidenceTrust::Untrusted,
            );

        let result = policy.evaluate(context);

        assert_eq!(
            result.decision(),
            SafetyDecision::RequireEvidence
        );
        assert_eq!(
            result.reason(),
            SafetyReason::EvidenceUntrusted
        );
    }

    #[test]
    fn unknown_evidence_does_not_authorize_action() {
        let policy = SafetyPolicy::strict();

        let context = safe_context(SafetyAction::Retry)
            .with_evidence_trust(
                EvidenceTrust::Unknown,
            );

        let result = policy.evaluate(context);

        assert_eq!(
            result.decision(),
            SafetyDecision::RequireEvidence
        );
        assert_eq!(
            result.reason(),
            SafetyReason::SafetyEvidenceUnknown
        );
    }

    #[test]
    fn stale_evidence_fails_closed() {
        let policy = SafetyPolicy::strict();

        let context = safe_context(SafetyAction::Retry)
            .with_freshness(
                FreshnessState::Stale,
            );

        let result = policy.evaluate(context);

        assert_eq!(
            result.decision(),
            SafetyDecision::RequireEvidence
        );
        assert_eq!(
            result.reason(),
            SafetyReason::EvidenceStale
        );
    }

    #[test]
    fn unknown_freshness_fails_closed() {
        let policy = SafetyPolicy::strict();

        let context = safe_context(SafetyAction::Retry)
            .with_freshness(
                FreshnessState::Unknown,
            );

        let result = policy.evaluate(context);

        assert_eq!(
            result.decision(),
            SafetyDecision::RequireEvidence
        );
        assert_eq!(
            result.reason(),
            SafetyReason::EvidenceFreshnessUnknown
        );
    }

    #[test]
    fn denied_authorization_is_terminal() {
        let policy = SafetyPolicy::strict();

        let context = safe_context(SafetyAction::Retry)
            .with_authorization(
                AuthorizationState::Denied,
            );

        let result = policy.evaluate(context);

        assert_eq!(
            result.decision(),
            SafetyDecision::Deny
        );
        assert_eq!(
            result.reason(),
            SafetyReason::AuthorizationDenied
        );
    }

    #[test]
    fn unknown_authorization_fails_closed() {
        let policy = SafetyPolicy::strict();

        let context = safe_context(SafetyAction::Retry)
            .with_authorization(
                AuthorizationState::Unknown,
            );

        let result = policy.evaluate(context);

        assert_eq!(
            result.decision(),
            SafetyDecision::RequireEvidence
        );
        assert_eq!(
            result.reason(),
            SafetyReason::AuthorizationUnknown
        );
    }

    #[test]
    fn semantic_violation_cannot_be_overridden() {
        let policy = SafetyPolicy::strict();

        let context = safe_context(SafetyAction::Recompile)
            .with_semantic(
                SemanticState::Violated,
            );

        let result = policy.evaluate(context);

        assert_eq!(
            result.decision(),
            SafetyDecision::Deny
        );
        assert_eq!(
            result.reason(),
            SafetyReason::SemanticViolation
        );
    }

    #[test]
    fn unknown_semantics_fail_closed() {
        let policy = SafetyPolicy::strict();

        let context = safe_context(SafetyAction::Reroute)
            .with_semantic(
                SemanticState::Unknown,
            );

        let result = policy.evaluate(context);

        assert_eq!(
            result.decision(),
            SafetyDecision::RequireEvidence
        );
        assert_eq!(
            result.reason(),
            SafetyReason::SemanticPreservationUnknown
        );
    }

    #[test]
    fn unavailable_capability_denies_action() {
        let policy = SafetyPolicy::strict();

        let context = safe_context(SafetyAction::Migrate)
            .with_capability(
                CapabilityState::Unavailable,
            );

        let result = policy.evaluate(context);

        assert_eq!(
            result.decision(),
            SafetyDecision::Deny
        );
        assert_eq!(
            result.reason(),
            SafetyReason::CapabilityUnavailable
        );
    }

    #[test]
    fn unknown_capability_requires_evidence() {
        let policy = SafetyPolicy::strict();

        let context = safe_context(SafetyAction::Migrate)
            .with_capability(
                CapabilityState::Unknown,
            );

        let result = policy.evaluate(context);

        assert_eq!(
            result.decision(),
            SafetyDecision::RequireEvidence
        );
        assert_eq!(
            result.reason(),
            SafetyReason::CapabilityUnknown
        );
    }

    #[test]
    fn unavailable_resources_deny_action() {
        let policy = SafetyPolicy::strict();

        let context = safe_context(SafetyAction::Reroute)
            .with_resources(
                ResourceState::Unavailable,
            );

        let result = policy.evaluate(context);

        assert_eq!(
            result.decision(),
            SafetyDecision::Deny
        );
        assert_eq!(
            result.reason(),
            SafetyReason::ResourceUnavailable
        );
    }

    #[test]
    fn unknown_resources_require_evidence() {
        let policy = SafetyPolicy::strict();

        let context = safe_context(SafetyAction::Reroute)
            .with_resources(
                ResourceState::Unknown,
            );

        let result = policy.evaluate(context);

        assert_eq!(
            result.decision(),
            SafetyDecision::RequireEvidence
        );
        assert_eq!(
            result.reason(),
            SafetyReason::ResourceUnknown
        );
    }

    #[test]
    fn exhausted_budget_denies_normal_action() {
        let policy = SafetyPolicy::strict();

        let context = safe_context(SafetyAction::Retry)
            .with_budget(
                BudgetState::Exhausted,
            );

        let result = policy.evaluate(context);

        assert_eq!(
            result.decision(),
            SafetyDecision::Deny
        );
        assert_eq!(
            result.reason(),
            SafetyReason::BudgetExhausted
        );
    }

    #[test]
    fn exhausted_normal_budget_does_not_block_abort() {
        let policy = SafetyPolicy::strict();

        let context = safe_context(SafetyAction::Abort)
            .with_safety(SafetyState::Unsafe)
            .with_budget(
                BudgetState::Exhausted,
            );

        let result = policy.evaluate(context);

        assert_eq!(
            result.decision(),
            SafetyDecision::Allow
        );
    }

    #[test]
    fn incomplete_provenance_requires_evidence() {
        let policy = SafetyPolicy::strict();

        let context = safe_context(SafetyAction::Recompile)
            .with_provenance(
                ProvenanceState::Incomplete,
            );

        let result = policy.evaluate(context);

        assert_eq!(
            result.decision(),
            SafetyDecision::RequireEvidence
        );
        assert_eq!(
            result.reason(),
            SafetyReason::ProvenanceIncomplete
        );
    }

    #[test]
    fn invalid_provenance_denies_action() {
        let policy = SafetyPolicy::strict();

        let context = safe_context(SafetyAction::Recompile)
            .with_provenance(
                ProvenanceState::Invalid,
            );

        let result = policy.evaluate(context);

        assert_eq!(
            result.decision(),
            SafetyDecision::Deny
        );
        assert_eq!(
            result.reason(),
            SafetyReason::ProvenanceInvalid
        );
    }

    #[test]
    fn mutating_action_requires_verification_contract() {
        let policy = SafetyPolicy::new(
            SafetyRequirement::strict()
                .with_verification_for_mutating_actions(true),
        );

        let context = safe_context(SafetyAction::Retry)
            .with_verification(
                VerificationRequirement::None,
            );

        let result = policy.evaluate(context);

        // The action's default verification requirement is still applied,
        // therefore a missing caller-supplied requirement cannot weaken the
        // safety contract.
        assert_eq!(
            result.decision(),
            SafetyDecision::Allow
        );
        assert_eq!(
            result.verification_requirement(),
            VerificationRequirement::RequiredBeforeAcceptance
        );
    }

    #[test]
    fn explicit_pre_execution_verification_is_preserved() {
        let policy = SafetyPolicy::strict();

        let context = safe_context(SafetyAction::Recompile)
            .with_verification(
                VerificationRequirement::RequiredBeforeExecution,
            );

        let result = policy.evaluate(context);

        assert_eq!(
            result.decision(),
            SafetyDecision::Allow
        );
        assert_eq!(
            result.verification_requirement(),
            VerificationRequirement::RequiredBeforeExecution
        );
    }

    #[test]
    fn observation_can_be_authorized_without_execution_authorization() {
        let policy = SafetyPolicy::strict();

        let context = safe_context(SafetyAction::Observe)
            .with_authorization(
                AuthorizationState::Unknown,
            )
            .with_verification(
                VerificationRequirement::None,
            );

        let result = policy.evaluate(context);

        assert_eq!(
            result.decision(),
            SafetyDecision::Allow
        );
        assert_eq!(
            result.verification_requirement(),
            VerificationRequirement::None
        );
    }

    #[test]
    fn observation_still_requires_explicit_action_authorization() {
        let policy = SafetyPolicy::strict();

        let context = safe_context(SafetyAction::Observe)
            .with_action_authorization(
                ActionAuthorization::Unknown,
            );

        let result = policy.evaluate(context);

        assert_eq!(
            result.decision(),
            SafetyDecision::RequireEvidence
        );
        assert_eq!(
            result.reason(),
            SafetyReason::ActionAuthorizationUnknown
        );
    }

    #[test]
    fn explicit_escalation_wins_over_positive_conditions() {
        let policy = SafetyPolicy::strict();

        let context = safe_context(SafetyAction::Migrate)
            .with_explicit_escalation(true);

        let result = policy.evaluate(context);

        assert_eq!(
            result.decision(),
            SafetyDecision::Escalate
        );
        assert_eq!(
            result.reason(),
            SafetyReason::EscalationRequired
        );
    }

    #[test]
    fn explicit_unsafe_condition_precedes_normal_recovery() {
        let policy = SafetyPolicy::strict();

        let context = safe_context(SafetyAction::Migrate)
            .with_safety(SafetyState::Unsafe);

        let result = policy.evaluate(context);

        assert_eq!(
            result.decision(),
            SafetyDecision::Deny
        );
        assert_eq!(
            result.reason(),
            SafetyReason::UnsafeCondition
        );
    }

    #[test]
    fn unsafe_abort_with_explicit_escalation_escalates() {
        let policy = SafetyPolicy::strict();

        let context = safe_context(SafetyAction::Abort)
            .with_safety(SafetyState::Unsafe)
            .with_explicit_escalation(true);

        let result = policy.evaluate(context);

        assert_eq!(
            result.decision(),
            SafetyDecision::Escalate
        );
        assert_eq!(
            result.reason(),
            SafetyReason::EscalationRequired
        );
    }

    #[test]
    fn unsafe_abort_with_stale_evidence_requires_evidence() {
        let policy = SafetyPolicy::strict();

        let context = safe_context(SafetyAction::Abort)
            .with_safety(SafetyState::Unsafe)
            .with_freshness(FreshnessState::Stale);

        let result = policy.evaluate(context);

        assert_eq!(
            result.decision(),
            SafetyDecision::RequireEvidence
        );
        assert_eq!(
            result.reason(),
            SafetyReason::EvidenceStale
        );
    }

    #[test]
    fn unsafe_abort_with_untrusted_evidence_requires_evidence() {
        let policy = SafetyPolicy::strict();

        let context = safe_context(SafetyAction::Abort)
            .with_safety(SafetyState::Unsafe)
            .with_evidence_trust(
                EvidenceTrust::Untrusted,
            );

        let result = policy.evaluate(context);

        assert_eq!(
            result.decision(),
            SafetyDecision::RequireEvidence
        );
        assert_eq!(
            result.reason(),
            SafetyReason::EvidenceUntrusted
        );
    }

    #[test]
    fn deterministic_evaluation_produces_identical_results() {
        let policy = SafetyPolicy::strict();

        let context = safe_context(SafetyAction::Reoptimize);

        let first = policy.evaluate(context);
        let second = policy.evaluate(context);

        assert_eq!(first, second);
    }

    #[test]
    fn verification_strength_is_monotonic() {
        assert_eq!(
            VerificationRequirement::strongest(
                VerificationRequirement::None,
                VerificationRequirement::RequiredBeforeAcceptance,
            ),
            VerificationRequirement::RequiredBeforeAcceptance
        );

        assert_eq!(
            VerificationRequirement::strongest(
                VerificationRequirement::RequiredBeforeAcceptance,
                VerificationRequirement::RequiredBeforeExecution,
            ),
            VerificationRequirement::RequiredBeforeExecution
        );

        assert_eq!(
            VerificationRequirement::strongest(
                VerificationRequirement::RequiredBeforeExecution,
                VerificationRequirement::None,
            ),
            VerificationRequirement::RequiredBeforeExecution
        );
    }

    #[test]
    fn all_normal_mutating_actions_have_post_execution_verification() {
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
        ];

        for action in actions {
            assert_eq!(
                action.default_verification_requirement(),
                VerificationRequirement::RequiredBeforeAcceptance,
                "action {action} lacks post-execution verification"
            );
        }
    }

    #[test]
    fn abort_is_protective_not_normal_recovery() {
        assert!(SafetyAction::Abort.is_protective());
        assert!(!SafetyAction::Abort.is_recovery());
    }

    #[test]
    fn observe_is_not_mutating() {
        assert!(!SafetyAction::Observe.is_mutating());
    }

    #[test]
    fn implementation_changing_actions_are_explicitly_classified() {
        assert!(
            SafetyAction::Remap.changes_implementation()
        );
        assert!(
            SafetyAction::Reroute.changes_implementation()
        );
        assert!(
            SafetyAction::Reschedule.changes_implementation()
        );
        assert!(
            SafetyAction::Recompile.changes_implementation()
        );
        assert!(
            SafetyAction::Reoptimize.changes_implementation()
        );
        assert!(
            SafetyAction::AdaptQec.changes_implementation()
        );
        assert!(
            SafetyAction::Migrate.changes_implementation()
        );
    }

    #[test]
    fn schema_identity_is_stable_and_non_empty() {
        assert!(
            !RESILIENCE_SAFETY_SCHEMA_ID.is_empty()
        );
        assert!(
            RESILIENCE_SAFETY_SCHEMA_VERSION > 0
        );
    }
}