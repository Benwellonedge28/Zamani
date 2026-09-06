//! Zamani Quantum Resilience — Policy Engine
//!
//! Path:
//!     src/quantum/resilience/policy/policy.rs
//!
//! Purpose:
//!     Defines the provider-independent resilience policy contract and the
//!     deterministic policy evaluator used between diagnosis and planning.
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
//!     POLICY  <--- this module
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
//! This module decides WHAT the resilience system is permitted or required
//! to do. It does not implement HOW an action is executed.
//!
//! -----------------------------------------------------------------------------
//! Ownership boundaries
//! -----------------------------------------------------------------------------
//!
//! This module MUST NOT own:
//!
//! - quantum IR;
//! - quantum gates;
//! - quantum operations;
//! - quantum circuits;
//! - logical/physical qubit identity;
//! - hardware discovery;
//! - hardware calibration;
//! - hardware execution;
//! - routing;
//! - scheduling;
//! - optimization;
//! - compilation;
//! - QEC implementation;
//! - noise models;
//! - fault ontology;
//! - mitigation implementation;
//! - recovery implementation;
//! - verification implementation;
//! - provider-specific behavior;
//! - backend-specific behavior;
//! - credentials;
//! - network I/O;
//! - filesystem I/O;
//! - hidden global state;
//! - background threads;
//! - hidden retry loops.
//!
//! The authoritative quantum identities remain:
//!
//!     crate::quantum::ir::qubit::QubitId
//!     crate::quantum::ir::qubit::PhysicalQubitId
//!
//! This module does not redefine either type.
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
//! Execution remains owned by the runtime/hardware execution boundary.
//!
//! -----------------------------------------------------------------------------
//! Write once, scale everywhere
//! -----------------------------------------------------------------------------
//!
//! This module deliberately contains no machine-size assumptions.
//!
//! It MUST NOT contain:
//!
//!     MAX_QUBITS
//!     MAX_PHYSICAL_QUBITS
//!     MAX_BACKENDS
//!     MAX_INCIDENTS
//!     MAX_RECOVERY_ATTEMPTS
//!     MAX_RETRIES
//!
//! It MUST NOT contain provider branches such as:
//!
//!     if backend == "ibm" { ... }
//!     if backend == "ionq" { ... }
//!
//! It MUST NOT assume a particular topology, qubit count, gate set, timing
//! model, or execution model.
//!
//! "Infinite" scalability means that this module imposes no artificial finite
//! machine-size ceiling. Actual execution limits come from caller policy,
//! target capabilities, resource availability, deadlines, budgets, and the
//! runtime.
//!
//! -----------------------------------------------------------------------------
//! Policy philosophy
//! -----------------------------------------------------------------------------
//!
//! Policy is a safety and authorization boundary, not merely a preference
//! object.
//!
//! A policy decision MUST consider:
//!
//! 1. caller-declared requirements;
//! 2. semantic guarantees;
//! 3. permitted adaptation/recovery mechanisms;
//! 4. applicable budgets;
//! 5. diagnosis confidence;
//! 6. resource/capability availability;
//! 7. security/trust state;
//! 8. execution state;
//! 9. verification requirements;
//! 10. escalation permission.
//!
//! Availability MUST NEVER override semantic correctness.
//!
//! A policy may allow an action because it improves availability, but an action
//! must still satisfy semantic, safety, authorization, and capability
//! constraints before planning/execution.
//!
//! -----------------------------------------------------------------------------
//! Determinism
//! -----------------------------------------------------------------------------
//!
//! The evaluator is deterministic with respect to its explicit inputs.
//!
//! It does not:
//!
//! - read the clock;
//! - read environment variables;
//! - read files;
//! - perform network I/O;
//! - inspect global state;
//! - generate randomness;
//! - generate implicit identifiers.
//!
//! If stochastic strategy selection is eventually required, the randomness
//! source MUST be supplied explicitly by a higher-level deterministic/replay
//! context.
//!
//! -----------------------------------------------------------------------------
//! Security
//! -----------------------------------------------------------------------------
//!
//! Policy is fail-closed for safety-critical uncertainty.
//!
//! Unknown or untrusted information MUST NOT silently become authorization.
//!
//! The evaluator never authenticates providers and never handles credentials.
//!
//! Authentication and authorization of external observations belong to the
//! corresponding security/hardware/runtime contracts.
//!
//! -----------------------------------------------------------------------------
//! Rust contract
//! -----------------------------------------------------------------------------
//!
//! - Rust 1.97 / 1.97.1
//! - Rust 2021
//! - stable Rust only
//! - no unsafe code
//! - no unsafe operations
//! - no hidden mutable global state
//! - no fixed machine-size limits
//! - no fixed retry loop
//! - no provider-specific implementation
//!
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::fmt;
use std::num::NonZeroU64;
use std::time::Duration;

use crate::quantum::resilience::api::request::{
    AdaptationPermissions,
    MitigationPermission,
    QecAdaptationPermission,
    RecoveryPermissions,
    ResourcePreference,
    ResilienceExecutionMode,
    ResilienceRequest,
    SemanticGuarantee,
};

// =============================================================================
// Public schema
// =============================================================================

/// Stable schema identifier for the resilience policy contract.
pub const RESILIENCE_POLICY_SCHEMA_ID: &str =
    "zamani.quantum.resilience.policy";

/// Semantic version of the resilience policy contract.
///
/// This is independent from the Rust crate/package version.
pub const RESILIENCE_POLICY_SCHEMA_VERSION: u16 = 1;

// =============================================================================
// Policy action
// =============================================================================

/// A policy-level action that may be considered by the planner.
///
/// These are semantic policy actions, not execution implementations.
///
/// For example, `Reroute` means that routing MAY be requested. It does not
/// perform routing itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PolicyAction {
    /// Continue the current execution when no resilience intervention is
    /// required.
    Continue,

    /// Request another execution attempt when retry is valid.
    Retry,

    /// Restart from a valid execution boundary.
    Restart,

    /// Resume from a compatible checkpoint.
    Resume,

    /// Roll back to a valid checkpoint/state.
    Rollback,

    /// Request logical-to-physical remapping.
    Remap,

    /// Request physical rerouting.
    Reroute,

    /// Request schedule reconstruction.
    Reschedule,

    /// Request recompilation.
    Recompile,

    /// Request reoptimization.
    Reoptimize,

    /// Request a QEC configuration adaptation.
    AdaptQec,

    /// Request mitigation.
    Mitigate,

    /// Request migration to another compatible target.
    Migrate,

    /// Request a mathematically/semantically valid compensation operation.
    Compensate,

    /// Require external escalation.
    Escalate,

    /// Reject the current execution/result.
    Reject,
}

impl PolicyAction {
    /// Stable machine-readable action name.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Continue => "continue",
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
            Self::Escalate => "escalate",
            Self::Reject => "reject",
        }
    }

    /// Returns whether this action modifies the physical execution strategy.
    pub const fn is_adaptation(self) -> bool {
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

    /// Returns whether this action is a recovery operation.
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

    /// Returns whether this action invokes mitigation.
    pub const fn is_mitigation(self) -> bool {
        matches!(self, Self::Mitigate)
    }

    /// Returns whether this action requires another authority.
    pub const fn is_escalation(self) -> bool {
        matches!(self, Self::Escalate)
    }

    /// Returns whether this action explicitly rejects execution/result.
    pub const fn is_rejection(self) -> bool {
        matches!(self, Self::Reject)
    }
}

impl fmt::Display for PolicyAction {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Policy outcome
// =============================================================================

/// High-level result of policy evaluation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PolicyOutcome {
    /// Current execution may continue.
    Allow,

    /// One or more resilience actions may be planned.
    Adapt,

    /// Recovery is permitted/required.
    Recover,

    /// Mitigation is permitted/required.
    Mitigate,

    /// A result may only be accepted after the required verification contract.
    Verify,

    /// The policy requires escalation.
    Escalate,

    /// The policy rejects continuation/acceptance.
    Reject,
}

impl PolicyOutcome {
    /// Stable machine-readable name.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Allow => "allow",
            Self::Adapt => "adapt",
            Self::Recover => "recover",
            Self::Mitigate => "mitigate",
            Self::Verify => "verify",
            Self::Escalate => "escalate",
            Self::Reject => "reject",
        }
    }

    /// Returns whether this outcome permits autonomous continuation.
    pub const fn permits_autonomous_continuation(self) -> bool {
        matches!(
            self,
            Self::Allow | Self::Adapt | Self::Recover | Self::Mitigate | Self::Verify
        )
    }

    /// Returns whether this outcome requires escalation.
    pub const fn requires_escalation(self) -> bool {
        matches!(self, Self::Escalate)
    }

    /// Returns whether this outcome rejects the execution/result.
    pub const fn rejects(self) -> bool {
        matches!(self, Self::Reject)
    }
}

impl fmt::Display for PolicyOutcome {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Policy reason
// =============================================================================

/// Machine-readable reason for a policy decision.
///
/// Reasons are intentionally provider-neutral.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PolicyReason {
    /// No policy violation or resilience condition requires intervention.
    NoInterventionRequired,

    /// The current execution is already within the declared policy.
    WithinPolicy,

    /// Adaptation is explicitly permitted and required by current conditions.
    AdaptationPermitted,

    /// Recovery is explicitly permitted and required by current conditions.
    RecoveryPermitted,

    /// Mitigation is explicitly permitted and required.
    MitigationPermitted,

    /// Verification is required before acceptance.
    VerificationRequired,

    /// The selected action is not permitted.
    ActionNotPermitted,

    /// A required budget is exhausted.
    BudgetExhausted,

    /// A required capability is unavailable.
    CapabilityUnavailable,

    /// Diagnosis confidence is insufficient.
    InsufficientConfidence,

    /// Conflicting evidence prevents safe autonomous action.
    ConflictingEvidence,

    /// Trusted observation requirements were not satisfied.
    UntrustedEvidence,

    /// Semantic guarantees prohibit the proposed action.
    SemanticGuaranteeViolation,

    /// Escalation is required.
    EscalationRequired,

    /// Escalation is forbidden by caller policy.
    EscalationForbidden,

    /// The execution deadline does not permit the proposed action.
    DeadlineConstraint,

    /// A caller-declared resource preference cannot be satisfied safely.
    ResourceConstraint,

    /// The policy is internally inconsistent.
    PolicyConfigurationInvalid,

    /// An action would require an implementation owned by another subsystem.
    ExternalCapabilityRequired,

    /// A recovered result must not be accepted without verification.
    UnverifiedRecovery,

    /// The supplied policy state is stale.
    StalePolicyState,
}

impl PolicyReason {
    /// Stable machine-readable reason name.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoInterventionRequired => "no_intervention_required",
            Self::WithinPolicy => "within_policy",
            Self::AdaptationPermitted => "adaptation_permitted",
            Self::RecoveryPermitted => "recovery_permitted",
            Self::MitigationPermitted => "mitigation_permitted",
            Self::VerificationRequired => "verification_required",
            Self::ActionNotPermitted => "action_not_permitted",
            Self::BudgetExhausted => "budget_exhausted",
            Self::CapabilityUnavailable => "capability_unavailable",
            Self::InsufficientConfidence => "insufficient_confidence",
            Self::ConflictingEvidence => "conflicting_evidence",
            Self::UntrustedEvidence => "untrusted_evidence",
            Self::SemanticGuaranteeViolation => "semantic_guarantee_violation",
            Self::EscalationRequired => "escalation_required",
            Self::EscalationForbidden => "escalation_forbidden",
            Self::DeadlineConstraint => "deadline_constraint",
            Self::ResourceConstraint => "resource_constraint",
            Self::PolicyConfigurationInvalid => "policy_configuration_invalid",
            Self::ExternalCapabilityRequired => "external_capability_required",
            Self::UnverifiedRecovery => "unverified_recovery",
            Self::StalePolicyState => "stale_policy_state",
        }
    }
}

impl fmt::Display for PolicyReason {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Confidence
// =============================================================================

/// Policy-level confidence represented without floating-point equality issues.
///
/// Values are normalized to the inclusive range [0, 1].
///
/// This type does not define an acceptance threshold. Thresholds are supplied
/// explicitly by `ResiliencePolicy`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PolicyConfidence(u32);

impl PolicyConfidence {
    /// Number of discrete units used by the representation.
    ///
    /// This is a representation precision, not a machine-size limit.
    const SCALE: u32 = 1_000_000;

    /// Zero confidence.
    pub const ZERO: Self = Self(0);

    /// Maximum confidence.
    pub const MAX: Self = Self(Self::SCALE);

    /// Creates confidence from a normalized floating-point value.
    ///
    /// NaN and infinite values are rejected.
    pub fn from_f64(value: f64) -> Option<Self> {
        if !value.is_finite() || !(0.0..=1.0).contains(&value) {
            return None;
        }

        let scaled = value * f64::from(Self::SCALE);
        let rounded = scaled.round();

        if !(0.0..=f64::from(Self::SCALE)).contains(&rounded) {
            return None;
        }

        Some(Self(rounded as u32))
    }

    /// Creates confidence from a numerator and denominator.
    ///
    /// `denominator == 0` is rejected.
    pub fn from_ratio(numerator: u64, denominator: u64) -> Option<Self> {
        if denominator == 0 || numerator > denominator {
            return None;
        }

        let scaled = numerator
            .saturating_mul(u64::from(Self::SCALE))
            / denominator;

        let scaled = u32::try_from(scaled).ok()?;

        Some(Self(scaled.min(Self::SCALE)))
    }

    /// Returns the normalized confidence as `f64`.
    pub fn as_f64(self) -> f64 {
        f64::from(self.0) / f64::from(Self::SCALE)
    }

    /// Returns whether confidence is exactly zero.
    pub const fn is_zero(self) -> bool {
        self.0 == 0
    }

    /// Returns whether confidence is maximal.
    pub const fn is_max(self) -> bool {
        self.0 == Self::SCALE
    }

    /// Returns the internal normalized representation.
    pub const fn units(self) -> u32 {
        self.0
    }
}

impl Default for PolicyConfidence {
    fn default() -> Self {
        Self::ZERO
    }
}

// =============================================================================
// Evidence state
// =============================================================================

/// Trust state of policy input evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum EvidenceTrust {
    /// Evidence has not yet been established.
    #[default]
    Unknown,

    /// Evidence passed the applicable trust/integrity boundary.
    Trusted,

    /// Evidence is explicitly untrusted.
    Untrusted,

    /// Evidence sources disagree.
    Conflicting,
}

impl EvidenceTrust {
    /// Stable machine-readable representation.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unknown => "unknown",
            Self::Trusted => "trusted",
            Self::Untrusted => "untrusted",
            Self::Conflicting => "conflicting",
        }
    }

    /// Returns whether the evidence is trusted.
    pub const fn is_trusted(self) -> bool {
        matches!(self, Self::Trusted)
    }

    /// Returns whether the evidence is unsafe for autonomous action.
    pub const fn is_unsafe(self) -> bool {
        matches!(
            self,
            Self::Unknown | Self::Untrusted | Self::Conflicting
        )
    }
}

impl fmt::Display for EvidenceTrust {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Diagnosis state
// =============================================================================

/// Normalized policy-relevant diagnosis state.
///
/// The actual diagnosis remains owned by `quantum::resilience::diagnosis`.
/// This type intentionally contains only the policy facts required to make a
/// decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum DiagnosisState {
    /// No diagnosis has been established.
    #[default]
    None,

    /// A condition was observed but no intervention is currently required.
    Benign,

    /// A degraded condition exists but execution may continue.
    Degraded,

    /// An adaptation may restore policy compliance.
    AdaptationRequired,

    /// Recovery is required.
    RecoveryRequired,

    /// Mitigation is required or strongly indicated.
    MitigationRequired,

    /// The condition prevents safe autonomous continuation.
    Critical,

    /// Evidence conflicts.
    Conflicting,

    /// The root cause is unknown.
    Unknown,
}

impl DiagnosisState {
    /// Stable machine-readable representation.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Benign => "benign",
            Self::Degraded => "degraded",
            Self::AdaptationRequired => "adaptation_required",
            Self::RecoveryRequired => "recovery_required",
            Self::MitigationRequired => "mitigation_required",
            Self::Critical => "critical",
            Self::Conflicting => "conflicting",
            Self::Unknown => "unknown",
        }
    }

    /// Returns whether intervention is explicitly required.
    pub const fn requires_intervention(self) -> bool {
        matches!(
            self,
            Self::AdaptationRequired
                | Self::RecoveryRequired
                | Self::MitigationRequired
                | Self::Critical
        )
    }

    /// Returns whether autonomous action is inherently unsafe without more
    /// evidence.
    pub const fn requires_additional_evidence(self) -> bool {
        matches!(self, Self::Conflicting | Self::Unknown)
    }
}

impl fmt::Display for DiagnosisState {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Verification state
// =============================================================================

/// Verification state known to policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum VerificationState {
    /// Verification has not yet occurred.
    #[default]
    NotPerformed,

    /// Verification is currently possible but not complete.
    Pending,

    /// Verification succeeded.
    Verified,

    /// Verification succeeded with an explicitly permitted degradation.
    VerifiedDegraded,

    /// Verification was inconclusive.
    Inconclusive,

    /// Verification failed.
    Failed,
}

impl VerificationState {
    /// Stable machine-readable representation.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotPerformed => "not_performed",
            Self::Pending => "pending",
            Self::Verified => "verified",
            Self::VerifiedDegraded => "verified_degraded",
            Self::Inconclusive => "inconclusive",
            Self::Failed => "failed",
        }
    }

    /// Returns whether the result is verified.
    pub const fn is_verified(self) -> bool {
        matches!(self, Self::Verified | Self::VerifiedDegraded)
    }

    /// Returns whether verification definitively failed.
    pub const fn is_failed(self) -> bool {
        matches!(self, Self::Failed)
    }
}

impl fmt::Display for VerificationState {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Capability state
// =============================================================================

/// Policy-level availability of an externally owned capability.
///
/// This does not duplicate hardware capability structures. It is only the
/// policy-facing projection required to decide whether an action may be
/// considered.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum CapabilityState {
    /// Capability information is unavailable.
    #[default]
    Unknown,

    /// Capability is available.
    Available,

    /// Capability is unavailable.
    Unavailable,

    /// Capability changed and the previous policy state may be stale.
    Changed,
}

impl CapabilityState {
    /// Stable machine-readable representation.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unknown => "unknown",
            Self::Available => "available",
            Self::Unavailable => "unavailable",
            Self::Changed => "changed",
        }
    }

    /// Returns whether the capability can currently be relied upon.
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

/// Policy-facing resource state.
///
/// The concrete resource inventory remains owned by the hardware/runtime
/// resource subsystem.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ResourceState {
    /// Resource information is unavailable.
    #[default]
    Unknown,

    /// Required resources are available.
    Available,

    /// Resources are degraded but potentially usable.
    Degraded,

    /// Required resources are unavailable.
    Unavailable,

    /// Resource state changed since policy evaluation context was created.
    Changed,
}

impl ResourceState {
    /// Stable machine-readable representation.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unknown => "unknown",
            Self::Available => "available",
            Self::Degraded => "degraded",
            Self::Unavailable => "unavailable",
            Self::Changed => "changed",
        }
    }

    /// Returns whether resources are definitely available.
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

/// Remaining policy budget information.
///
/// The policy does not consume budgets itself. It evaluates the supplied
/// remaining budget state and tells the planner which classes of actions remain
/// eligible.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct PolicyBudgetState {
    /// Remaining retries, when bounded.
    retry_remaining: Option<NonZeroU64>,

    /// Remaining wall-clock budget, when bounded.
    time_remaining: Option<Duration>,

    /// Remaining execution shots, when bounded.
    shot_remaining: Option<NonZeroU64>,

    /// Whether a deadline has already been exceeded.
    deadline_exceeded: bool,
}

impl PolicyBudgetState {
    /// Creates an unbounded budget state.
    pub const fn unlimited() -> Self {
        Self {
            retry_remaining: None,
            time_remaining: None,
            shot_remaining: None,
            deadline_exceeded: false,
        }
    }

    /// Creates a budget state.
    pub const fn new(
        retry_remaining: Option<NonZeroU64>,
        time_remaining: Option<Duration>,
        shot_remaining: Option<NonZeroU64>,
        deadline_exceeded: bool,
    ) -> Self {
        Self {
            retry_remaining,
            time_remaining,
            shot_remaining,
            deadline_exceeded,
        }
    }

    /// Returns remaining retries.
    pub const fn retry_remaining(self) -> Option<NonZeroU64> {
        self.retry_remaining
    }

    /// Returns remaining time.
    pub const fn time_remaining(self) -> Option<Duration> {
        self.time_remaining
    }

    /// Returns remaining shots.
    pub const fn shot_remaining(self) -> Option<NonZeroU64> {
        self.shot_remaining
    }

    /// Returns whether the deadline has been exceeded.
    pub const fn deadline_exceeded(self) -> bool {
        self.deadline_exceeded
    }

    /// Returns whether retry budget is exhausted.
    pub const fn retry_exhausted(self) -> bool {
        matches!(self.retry_remaining, Some(value) if value.get() == 0)
    }

    /// Returns whether no bounded retry information exists.
    pub const fn retry_is_unbounded(self) -> bool {
        self.retry_remaining.is_none()
    }
}

// =============================================================================
// Policy input
// =============================================================================

/// Complete explicit input to one policy evaluation.
///
/// This structure intentionally contains only policy-facing information.
///
/// Hardware/routing/QEC/diagnosis subsystems should project their state into
/// this contract rather than making policy depend on their implementation
/// types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PolicyInput {
    /// Current normalized diagnosis.
    diagnosis: DiagnosisState,

    /// Confidence of the diagnosis.
    diagnosis_confidence: PolicyConfidence,

    /// Trust state of the observations supporting the diagnosis.
    evidence_trust: EvidenceTrust,

    /// Current verification state.
    verification: VerificationState,

    /// Target capability state relevant to the proposed policy action.
    capability: CapabilityState,

    /// Current resource state.
    resources: ResourceState,

    /// Current remaining budgets.
    budgets: PolicyBudgetState,

    /// Whether the execution state is still valid for policy decisions.
    execution_state_valid: bool,

    /// Whether the current policy context is stale.
    policy_state_stale: bool,
}

impl PolicyInput {
    /// Creates a conservative input state.
    ///
    /// Conservative means that missing information cannot silently authorize
    /// destructive or semantic-risk actions.
    pub const fn conservative() -> Self {
        Self {
            diagnosis: DiagnosisState::None,
            diagnosis_confidence: PolicyConfidence::ZERO,
            evidence_trust: EvidenceTrust::Unknown,
            verification: VerificationState::NotPerformed,
            capability: CapabilityState::Unknown,
            resources: ResourceState::Unknown,
            budgets: PolicyBudgetState::unlimited(),
            execution_state_valid: false,
            policy_state_stale: false,
        }
    }

    /// Creates an explicit policy input.
    pub const fn new(
        diagnosis: DiagnosisState,
        diagnosis_confidence: PolicyConfidence,
        evidence_trust: EvidenceTrust,
        verification: VerificationState,
        capability: CapabilityState,
        resources: ResourceState,
        budgets: PolicyBudgetState,
        execution_state_valid: bool,
        policy_state_stale: bool,
    ) -> Self {
        Self {
            diagnosis,
            diagnosis_confidence,
            evidence_trust,
            verification,
            capability,
            resources,
            budgets,
            execution_state_valid,
            policy_state_stale,
        }
    }

    /// Returns diagnosis state.
    pub const fn diagnosis(self) -> DiagnosisState {
        self.diagnosis
    }

    /// Returns diagnosis confidence.
    pub const fn diagnosis_confidence(self) -> PolicyConfidence {
        self.diagnosis_confidence
    }

    /// Returns evidence trust.
    pub const fn evidence_trust(self) -> EvidenceTrust {
        self.evidence_trust
    }

    /// Returns verification state.
    pub const fn verification(self) -> VerificationState {
        self.verification
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
    pub const fn budgets(self) -> PolicyBudgetState {
        self.budgets
    }

    /// Returns whether execution state is valid.
    pub const fn execution_state_valid(self) -> bool {
        self.execution_state_valid
    }

    /// Returns whether policy state is stale.
    pub const fn policy_state_stale(self) -> bool {
        self.policy_state_stale
    }
}

impl Default for PolicyInput {
    fn default() -> Self {
        Self::conservative()
    }
}

// =============================================================================
// Action rule
// =============================================================================

/// Explicit policy rule controlling one action class.
///
/// A rule contains no implementation details.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ActionRule {
    /// Whether the action may be considered.
    allowed: bool,

    /// Whether the action may be selected automatically.
    autonomous: bool,

    /// Whether successful verification is required after the action.
    verification_required: bool,
}

impl ActionRule {
    /// Creates an action rule.
    pub const fn new(
        allowed: bool,
        autonomous: bool,
        verification_required: bool,
    ) -> Self {
        Self {
            allowed,
            autonomous,
            verification_required,
        }
    }

    /// Creates a denied rule.
    pub const fn denied() -> Self {
        Self::new(false, false, true)
    }

    /// Creates an allowed autonomous rule.
    pub const fn autonomous() -> Self {
        Self::new(true, true, true)
    }

    /// Creates an allowed but externally-authorized rule.
    pub const fn authorized() -> Self {
        Self::new(true, false, true)
    }

    /// Returns whether the action is allowed.
    pub const fn allowed(self) -> bool {
        self.allowed
    }

    /// Returns whether the action may be selected autonomously.
    pub const fn autonomous(self) -> bool {
        self.autonomous
    }

    /// Returns whether verification is required after the action.
    pub const fn verification_required(self) -> bool {
        self.verification_required
    }

    /// Changes whether the action is allowed.
    pub const fn with_allowed(mut self, value: bool) -> Self {
        self.allowed = value;
        self
    }

    /// Changes whether the action may be selected autonomously.
    pub const fn with_autonomous(mut self, value: bool) -> Self {
        self.autonomous = value;
        self
    }

    /// Changes whether verification is required.
    pub const fn with_verification_required(mut self, value: bool) -> Self {
        self.verification_required = value;
        self
    }
}

// =============================================================================
// Policy configuration
// =============================================================================

/// Immutable resilience policy configuration.
///
/// This is deliberately independent of hardware size.
///
/// The caller can construct different policies for different workloads without
/// changing the policy implementation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResiliencePolicy {
    /// Minimum diagnosis confidence required for autonomous intervention.
    diagnosis_confidence: PolicyConfidence,

    /// Whether unknown/conflicting evidence may trigger autonomous actions.
    allow_uncertain_autonomous_action: bool,

    /// Caller resource preference.
    resource_preference: ResourcePreference,

    /// Required semantic guarantee.
    semantic_guarantee: SemanticGuarantee,

    /// Execution mode.
    execution_mode: ResilienceExecutionMode,

    /// Adaptation permissions.
    adaptation: AdaptationPermissions,

    /// Recovery permissions.
    recovery: RecoveryPermissions,

    /// Mitigation permission.
    mitigation: MitigationPermission,

    /// QEC adaptation permission.
    qec_adaptation: QecAdaptationPermission,

    /// Whether escalation is permitted.
    escalation_allowed: bool,

    /// Individual policy action rules.
    continue_rule: ActionRule,
    retry_rule: ActionRule,
    restart_rule: ActionRule,
    resume_rule: ActionRule,
    rollback_rule: ActionRule,
    remap_rule: ActionRule,
    reroute_rule: ActionRule,
    reschedule_rule: ActionRule,
    recompile_rule: ActionRule,
    reoptimize_rule: ActionRule,
    qec_rule: ActionRule,
    mitigate_rule: ActionRule,
    migrate_rule: ActionRule,
    compensate_rule: ActionRule,
    escalate_rule: ActionRule,
    reject_rule: ActionRule,
}

impl ResiliencePolicy {
    /// Constructs a policy from a Zamani resilience request.
    ///
    /// The request remains the caller-facing contract. This method converts
    /// request options into the immutable policy used during evaluation.
    pub fn from_request(request: &ResilienceRequest) -> Self {
        Self::from_request_options(request.options())
    }

    /// Constructs a policy from request options.
    ///
    /// This method avoids making the policy evaluator depend on the entire
    /// request/program representation.
    pub fn from_request_options(
        options: &crate::quantum::resilience::api::request::ResilienceRequestOptions,
    ) -> Self {
        let adaptation = options.adaptation();
        let recovery = options.recovery();

        Self {
            diagnosis_confidence: PolicyConfidence::ZERO,
            allow_uncertain_autonomous_action: false,
            resource_preference: options.resource_preference(),
            semantic_guarantee: options.semantic_guarantee(),
            execution_mode: options.execution_mode(),
            adaptation,
            recovery,
            mitigation: options.mitigation(),
            qec_adaptation: options.qec_adaptation(),
            escalation_allowed: options.escalation_allowed(),

            continue_rule: ActionRule::autonomous(),

            retry_rule: ActionRule::new(
                recovery.retry_allowed(),
                recovery.retry_allowed(),
                true,
            ),

            restart_rule: ActionRule::new(
                recovery.restart_allowed(),
                recovery.restart_allowed(),
                true,
            ),

            resume_rule: ActionRule::new(
                recovery.resume_allowed(),
                recovery.resume_allowed(),
                true,
            ),

            rollback_rule: ActionRule::new(
                recovery.rollback_allowed(),
                recovery.rollback_allowed(),
                true,
            ),

            remap_rule: ActionRule::new(
                adaptation.remapping_allowed(),
                adaptation.remapping_allowed(),
                true,
            ),

            reroute_rule: ActionRule::new(
                adaptation.rerouting_allowed(),
                adaptation.rerouting_allowed(),
                true,
            ),

            reschedule_rule: ActionRule::new(
                adaptation.rescheduling_allowed(),
                adaptation.rescheduling_allowed(),
                true,
            ),

            recompile_rule: ActionRule::new(
                adaptation.recompilation_allowed(),
                adaptation.recompilation_allowed(),
                true,
            ),

            reoptimize_rule: ActionRule::new(
                adaptation.reoptimization_allowed(),
                adaptation.reoptimization_allowed(),
                true,
            ),

            qec_rule: ActionRule::new(
                !matches!(options.qec_adaptation(), QecAdaptationPermission::Disabled),
                !matches!(options.qec_adaptation(), QecAdaptationPermission::Disabled),
                true,
            ),

            mitigate_rule: ActionRule::new(
                !matches!(options.mitigation(), MitigationPermission::Disabled),
                !matches!(options.mitigation(), MitigationPermission::Disabled),
                true,
            ),

            migrate_rule: ActionRule::new(
                adaptation.migration_allowed() && recovery.migration_allowed(),
                adaptation.migration_allowed() && recovery.migration_allowed(),
                true,
            ),

            compensate_rule: ActionRule::new(
                recovery.compensation_allowed(),
                recovery.compensation_allowed(),
                true,
            ),

            escalate_rule: ActionRule::new(
                options.escalation_allowed(),
                options.escalation_allowed(),
                true,
            ),

            reject_rule: ActionRule::autonomous(),
        }
    }

    /// Constructs a conservative production policy.
    ///
    /// This constructor does not choose a hardware-specific threshold.
    ///
    /// The default diagnosis confidence is zero because autonomous action
    /// should not be authorized by an absent diagnosis.
    pub const fn conservative() -> Self {
        Self {
            diagnosis_confidence: PolicyConfidence::ZERO,
            allow_uncertain_autonomous_action: false,
            resource_preference: ResourcePreference::CorrectnessFirst,
            semantic_guarantee: SemanticGuarantee::Strict,
            execution_mode: ResilienceExecutionMode::BestEffort,
            adaptation: AdaptationPermissions::deny_all(),
            recovery: RecoveryPermissions::deny_all(),
            mitigation: MitigationPermission::Disabled,
            qec_adaptation: QecAdaptationPermission::Disabled,
            escalation_allowed: true,

            continue_rule: ActionRule::autonomous(),
            retry_rule: ActionRule::denied(),
            restart_rule: ActionRule::denied(),
            resume_rule: ActionRule::denied(),
            rollback_rule: ActionRule::denied(),
            remap_rule: ActionRule::denied(),
            reroute_rule: ActionRule::denied(),
            reschedule_rule: ActionRule::denied(),
            recompile_rule: ActionRule::denied(),
            reoptimize_rule: ActionRule::denied(),
            qec_rule: ActionRule::denied(),
            mitigate_rule: ActionRule::denied(),
            migrate_rule: ActionRule::denied(),
            compensate_rule: ActionRule::denied(),
            escalate_rule: ActionRule::autonomous(),
            reject_rule: ActionRule::autonomous(),
        }
    }

    /// Returns a policy suitable for request-driven production execution.
    ///
    /// All permissions originate from the request; this function does not
    /// insert a machine-specific retry count, fidelity threshold, qubit count,
    /// or provider assumption.
    pub fn production(request: &ResilienceRequest) -> Self {
        Self::from_request(request)
    }

    /// Returns the configured minimum diagnosis confidence.
    pub const fn diagnosis_confidence(&self) -> PolicyConfidence {
        self.diagnosis_confidence
    }

    /// Sets the minimum diagnosis confidence for autonomous intervention.
    ///
    /// This is a policy threshold, not a hardware/noise threshold.
    pub const fn with_diagnosis_confidence(
        mut self,
        confidence: PolicyConfidence,
    ) -> Self {
        self.diagnosis_confidence = confidence;
        self
    }

    /// Returns whether uncertain evidence may authorize autonomous action.
    pub const fn allows_uncertain_autonomous_action(&self) -> bool {
        self.allow_uncertain_autonomous_action
    }

    /// Controls whether uncertain evidence may authorize autonomous action.
    ///
    /// Enabling this should normally be reserved for carefully controlled
    /// policies. Safety-critical verification requirements remain in force.
    pub const fn with_uncertain_autonomous_action(
        mut self,
        allowed: bool,
    ) -> Self {
        self.allow_uncertain_autonomous_action = allowed;
        self
    }

    /// Returns the resource preference.
    pub const fn resource_preference(&self) -> ResourcePreference {
        self.resource_preference
    }

    /// Returns the semantic guarantee.
    pub const fn semantic_guarantee(&self) -> SemanticGuarantee {
        self.semantic_guarantee
    }

    /// Returns the execution mode.
    pub const fn execution_mode(&self) -> ResilienceExecutionMode {
        self.execution_mode
    }

    /// Returns adaptation permissions.
    pub const fn adaptation(&self) -> AdaptationPermissions {
        self.adaptation
    }

    /// Returns recovery permissions.
    pub const fn recovery(&self) -> RecoveryPermissions {
        self.recovery
    }

    /// Returns mitigation permission.
    pub const fn mitigation(&self) -> MitigationPermission {
        self.mitigation
    }

    /// Returns QEC adaptation permission.
    pub const fn qec_adaptation(&self) -> QecAdaptationPermission {
        self.qec_adaptation
    }

    /// Returns whether escalation is allowed.
    pub const fn escalation_allowed(&self) -> bool {
        self.escalation_allowed
    }

    /// Returns the rule for an action.
    pub const fn rule(&self, action: PolicyAction) -> ActionRule {
        match action {
            PolicyAction::Continue => self.continue_rule,
            PolicyAction::Retry => self.retry_rule,
            PolicyAction::Restart => self.restart_rule,
            PolicyAction::Resume => self.resume_rule,
            PolicyAction::Rollback => self.rollback_rule,
            PolicyAction::Remap => self.remap_rule,
            PolicyAction::Reroute => self.reroute_rule,
            PolicyAction::Reschedule => self.reschedule_rule,
            PolicyAction::Recompile => self.recompile_rule,
            PolicyAction::Reoptimize => self.reoptimize_rule,
            PolicyAction::AdaptQec => self.qec_rule,
            PolicyAction::Mitigate => self.mitigate_rule,
            PolicyAction::Migrate => self.migrate_rule,
            PolicyAction::Compensate => self.compensate_rule,
            PolicyAction::Escalate => self.escalate_rule,
            PolicyAction::Reject => self.reject_rule,
        }
    }

    /// Evaluates one explicit policy input.
    ///
    /// This function is pure with respect to the outside world.
    ///
    /// It never:
    ///
    /// - executes an action;
    /// - performs routing;
    /// - changes schedules;
    /// - invokes QEC;
    /// - invokes mitigation;
    /// - invokes hardware;
    /// - retries execution.
    pub fn evaluate(&self, input: PolicyInput) -> PolicyDecision {
        if input.policy_state_stale() {
            return PolicyDecision::reject(
                PolicyReason::StalePolicyState,
                PolicyAction::Escalate,
            );
        }

        if !input.execution_state_valid() {
            return self.fail_closed(PolicyReason::ResourceConstraint);
        }

        if input.evidence_trust() != EvidenceTrust::Trusted
            && !self.allow_uncertain_autonomous_action
        {
            return self.handle_untrusted_evidence(input);
        }

        if input.diagnosis_confidence() < self.diagnosis_confidence
            && input.diagnosis().requires_intervention()
        {
            return self.handle_insufficient_confidence();
        }

        if input.diagnosis() == DiagnosisState::Conflicting {
            return self.handle_conflict();
        }

        if input.diagnosis() == DiagnosisState::Unknown {
            return self.handle_unknown();
        }

        if input.budgets().deadline_exceeded() {
            return self.handle_deadline();
        }

        if input.verification().is_failed() {
            return self.handle_verification_failure();
        }

        match input.diagnosis() {
            DiagnosisState::None | DiagnosisState::Benign => {
                self.evaluate_no_intervention(input)
            }

            DiagnosisState::Degraded => {
                self.evaluate_degraded(input)
            }

            DiagnosisState::AdaptationRequired => {
                self.evaluate_adaptation(input)
            }

            DiagnosisState::RecoveryRequired => {
                self.evaluate_recovery(input)
            }

            DiagnosisState::MitigationRequired => {
                self.evaluate_mitigation(input)
            }

            DiagnosisState::Critical => {
                self.evaluate_critical(input)
            }

            DiagnosisState::Conflicting | DiagnosisState::Unknown => {
                self.handle_unknown()
            }
        }
    }

    fn evaluate_no_intervention(
        &self,
        input: PolicyInput,
    ) -> PolicyDecision {
        if input.verification() == VerificationState::NotPerformed
            && self.semantic_guarantee == SemanticGuarantee::Strict
        {
            return PolicyDecision::new(
                PolicyOutcome::Verify,
                PolicyAction::Continue,
                PolicyReason::VerificationRequired,
                true,
            );
        }

        PolicyDecision::new(
            PolicyOutcome::Allow,
            PolicyAction::Continue,
            PolicyReason::NoInterventionRequired,
            false,
        )
    }

    fn evaluate_degraded(&self, input: PolicyInput) -> PolicyDecision {
        match self.resource_preference {
            ResourcePreference::CorrectnessFirst
            | ResourcePreference::PolicyDriven => {
                if self.semantic_guarantee
                    == SemanticGuarantee::AllowVerifiedDegradation
                    && input.verification() == VerificationState::VerifiedDegraded
                {
                    PolicyDecision::new(
                        PolicyOutcome::Verify,
                        PolicyAction::Continue,
                        PolicyReason::WithinPolicy,
                        false,
                    )
                } else {
                    self.find_first_allowed_adaptation()
                        .unwrap_or_else(|| {
                            self.escalate_or_reject(
                                PolicyReason::ResourceConstraint,
                            )
                        })
                }
            }

            ResourcePreference::AvailabilityFirst => {
                self.find_first_allowed_adaptation()
                    .or_else(|| {
                        if self.mitigation_rule.allowed() {
                            Some(PolicyDecision::new(
                                PolicyOutcome::Mitigate,
                                PolicyAction::Mitigate,
                                PolicyReason::MitigationPermitted,
                                true,
                            ))
                        } else {
                            None
                        }
                    })
                    .unwrap_or_else(|| {
                        self.escalate_or_reject(
                            PolicyReason::ResourceConstraint,
                        )
                    })
            }

            ResourcePreference::ResourceEfficient => {
                if input.verification().is_verified() {
                    PolicyDecision::new(
                        PolicyOutcome::Verify,
                        PolicyAction::Continue,
                        PolicyReason::WithinPolicy,
                        false,
                    )
                } else {
                    self.find_first_low_overhead_adaptation()
                        .unwrap_or_else(|| {
                            self.escalate_or_reject(
                                PolicyReason::ResourceConstraint,
                            )
                        })
                }
            }
        }
    }

    fn evaluate_adaptation(&self, input: PolicyInput) -> PolicyDecision {
        if !input.capability().is_available() {
            return self.escalate_or_reject(
                PolicyReason::CapabilityUnavailable,
            );
        }

        if !input.resources().is_available()
            && !matches!(input.resources(), ResourceState::Degraded)
        {
            return self.escalate_or_reject(
                PolicyReason::ResourceConstraint,
            );
        }

        self.find_first_allowed_adaptation()
            .unwrap_or_else(|| {
                self.escalate_or_reject(
                    PolicyReason::ActionNotPermitted,
                )
            })
    }

    fn evaluate_recovery(&self, input: PolicyInput) -> PolicyDecision {
        if input.budgets().retry_exhausted()
            && self.retry_rule.allowed()
        {
            return self.find_first_non_retry_recovery()
                .unwrap_or_else(|| {
                    self.escalate_or_reject(
                        PolicyReason::BudgetExhausted,
                    )
                });
        }

        if input.capability() == CapabilityState::Unavailable
            || input.resources() == ResourceState::Unavailable
        {
            if self.migrate_rule.allowed() {
                return PolicyDecision::new(
                    PolicyOutcome::Recover,
                    PolicyAction::Migrate,
                    PolicyReason::RecoveryPermitted,
                    true,
                );
            }
        }

        self.find_first_recovery()
            .unwrap_or_else(|| {
                self.escalate_or_reject(
                    PolicyReason::ActionNotPermitted,
                )
            })
    }

    fn evaluate_mitigation(&self, input: PolicyInput) -> PolicyDecision {
        match self.mitigation {
            MitigationPermission::Disabled => self.escalate_or_reject(
                PolicyReason::ActionNotPermitted,
            ),

            MitigationPermission::Allowed
            | MitigationPermission::Required => {
                if input.capability() == CapabilityState::Unavailable {
                    return self.escalate_or_reject(
                        PolicyReason::CapabilityUnavailable,
                    );
                }

                if self.mitigation_rule.allowed() {
                    PolicyDecision::new(
                        PolicyOutcome::Mitigate,
                        PolicyAction::Mitigate,
                        PolicyReason::MitigationPermitted,
                        self.mitigation_rule.verification_required(),
                    )
                } else {
                    self.escalate_or_reject(
                        PolicyReason::ActionNotPermitted,
                    )
                }
            }
        }
    }

    fn evaluate_critical(&self, input: PolicyInput) -> PolicyDecision {
        if input.evidence_trust() != EvidenceTrust::Trusted {
            return self.fail_closed(PolicyReason::UntrustedEvidence);
        }

        if self.escalation_allowed && self.escalate_rule.allowed() {
            return PolicyDecision::new(
                PolicyOutcome::Escalate,
                PolicyAction::Escalate,
                PolicyReason::EscalationRequired,
                true,
            );
        }

        PolicyDecision::reject(
            PolicyReason::EscalationForbidden,
            PolicyAction::Reject,
        )
    }

    fn handle_untrusted_evidence(
        &self,
        input: PolicyInput,
    ) -> PolicyDecision {
        if input.diagnosis().requires_intervention() {
            self.fail_closed(PolicyReason::UntrustedEvidence)
        } else {
            PolicyDecision::new(
                PolicyOutcome::Verify,
                PolicyAction::Continue,
                PolicyReason::VerificationRequired,
                true,
            )
        }
    }

    fn handle_insufficient_confidence(&self) -> PolicyDecision {
        if self.escalation_allowed && self.escalate_rule.allowed() {
            PolicyDecision::new(
                PolicyOutcome::Escalate,
                PolicyAction::Escalate,
                PolicyReason::InsufficientConfidence,
                true,
            )
        } else {
            PolicyDecision::reject(
                PolicyReason::InsufficientConfidence,
                PolicyAction::Reject,
            )
        }
    }

    fn handle_conflict(&self) -> PolicyDecision {
        if self.escalation_allowed && self.escalate_rule.allowed() {
            PolicyDecision::new(
                PolicyOutcome::Escalate,
                PolicyAction::Escalate,
                PolicyReason::ConflictingEvidence,
                true,
            )
        } else {
            PolicyDecision::reject(
                PolicyReason::ConflictingEvidence,
                PolicyAction::Reject,
            )
        }
    }

    fn handle_unknown(&self) -> PolicyDecision {
        if self.escalation_allowed && self.escalate_rule.allowed() {
            PolicyDecision::new(
                PolicyOutcome::Escalate,
                PolicyAction::Escalate,
                PolicyReason::InsufficientConfidence,
                true,
            )
        } else {
            PolicyDecision::reject(
                PolicyReason::InsufficientConfidence,
                PolicyAction::Reject,
            )
        }
    }

    fn handle_deadline(&self) -> PolicyDecision {
        if self.escalation_allowed && self.escalate_rule.allowed() {
            PolicyDecision::new(
                PolicyOutcome::Escalate,
                PolicyAction::Escalate,
                PolicyReason::DeadlineConstraint,
                true,
            )
        } else {
            PolicyDecision::reject(
                PolicyReason::DeadlineConstraint,
                PolicyAction::Reject,
            )
        }
    }

    fn handle_verification_failure(&self) -> PolicyDecision {
        if self.escalation_allowed && self.escalate_rule.allowed() {
            PolicyDecision::new(
                PolicyOutcome::Escalate,
                PolicyAction::Escalate,
                PolicyReason::UnverifiedRecovery,
                true,
            )
        } else {
            PolicyDecision::reject(
                PolicyReason::UnverifiedRecovery,
                PolicyAction::Reject,
            )
        }
    }

    fn fail_closed(&self, reason: PolicyReason) -> PolicyDecision {
        if self.escalation_allowed && self.escalate_rule.allowed() {
            PolicyDecision::new(
                PolicyOutcome::Escalate,
                PolicyAction::Escalate,
                reason,
                true,
            )
        } else {
            PolicyDecision::reject(reason, PolicyAction::Reject)
        }
    }

    fn escalate_or_reject(&self, reason: PolicyReason) -> PolicyDecision {
        if self.escalation_allowed && self.escalate_rule.allowed() {
            PolicyDecision::new(
                PolicyOutcome::Escalate,
                PolicyAction::Escalate,
                reason,
                true,
            )
        } else {
            PolicyDecision::reject(reason, PolicyAction::Reject)
        }
    }

    fn find_first_allowed_adaptation(&self) -> Option<PolicyDecision> {
        let candidates = [
            PolicyAction::Remap,
            PolicyAction::Reroute,
            PolicyAction::Reschedule,
            PolicyAction::Recompile,
            PolicyAction::Reoptimize,
            PolicyAction::AdaptQec,
            PolicyAction::Migrate,
        ];

        for action in candidates {
            let rule = self.rule(action);

            if !rule.allowed() || !rule.autonomous() {
                continue;
            }

            if action == PolicyAction::AdaptQec
                && self.qec_adaptation == QecAdaptationPermission::Disabled
            {
                continue;
            }

            if action == PolicyAction::Migrate
                && !self.adaptation.migration_allowed()
            {
                continue;
            }

            return Some(PolicyDecision::new(
                PolicyOutcome::Adapt,
                action,
                PolicyReason::AdaptationPermitted,
                rule.verification_required(),
            ));
        }

        None
    }

    fn find_first_low_overhead_adaptation(
        &self,
    ) -> Option<PolicyDecision> {
        let candidates = [
            PolicyAction::Reschedule,
            PolicyAction::Reroute,
            PolicyAction::Remap,
            PolicyAction::Reoptimize,
            PolicyAction::Recompile,
            PolicyAction::AdaptQec,
            PolicyAction::Migrate,
        ];

        for action in candidates {
            let rule = self.rule(action);

            if rule.allowed() && rule.autonomous() {
                if action == PolicyAction::AdaptQec
                    && self.qec_adaptation
                        == QecAdaptationPermission::Disabled
                {
                    continue;
                }

                return Some(PolicyDecision::new(
                    PolicyOutcome::Adapt,
                    action,
                    PolicyReason::AdaptationPermitted,
                    rule.verification_required(),
                ));
            }
        }

        None
    }

    fn find_first_recovery(&self) -> Option<PolicyDecision> {
        let candidates = [
            PolicyAction::Resume,
            PolicyAction::Rollback,
            PolicyAction::Retry,
            PolicyAction::Restart,
            PolicyAction::Migrate,
            PolicyAction::Compensate,
        ];

        for action in candidates {
            let rule = self.rule(action);

            if !rule.allowed() || !rule.autonomous() {
                continue;
            }

            if action == PolicyAction::Retry {
                // Actual retry-budget accounting belongs to the execution
                // planner/runtime. Policy only checks the explicit remaining
                // budget supplied in PolicyInput.
                continue;
            }

            return Some(PolicyDecision::new(
                PolicyOutcome::Recover,
                action,
                PolicyReason::RecoveryPermitted,
                rule.verification_required(),
            ));
        }

        if self.retry_rule.allowed() && self.retry_rule.autonomous() {
            return Some(PolicyDecision::new(
                PolicyOutcome::Recover,
                PolicyAction::Retry,
                PolicyReason::RecoveryPermitted,
                self.retry_rule.verification_required(),
            ));
        }

        None
    }

    fn find_first_non_retry_recovery(&self) -> Option<PolicyDecision> {
        let candidates = [
            PolicyAction::Resume,
            PolicyAction::Rollback,
            PolicyAction::Restart,
            PolicyAction::Migrate,
            PolicyAction::Compensate,
        ];

        for action in candidates {
            let rule = self.rule(action);

            if rule.allowed() && rule.autonomous() {
                return Some(PolicyDecision::new(
                    PolicyOutcome::Recover,
                    action,
                    PolicyReason::RecoveryPermitted,
                    rule.verification_required(),
                ));
            }
        }

        None
    }
}

// =============================================================================
// Policy decision
// =============================================================================

/// Immutable result of one policy evaluation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PolicyDecision {
    outcome: PolicyOutcome,
    action: PolicyAction,
    reason: PolicyReason,
    verification_required: bool,
}

impl PolicyDecision {
    /// Creates a policy decision.
    pub const fn new(
        outcome: PolicyOutcome,
        action: PolicyAction,
        reason: PolicyReason,
        verification_required: bool,
    ) -> Self {
        Self {
            outcome,
            action,
            reason,
            verification_required,
        }
    }

    /// Creates a rejection decision.
    pub const fn reject(
        reason: PolicyReason,
        action: PolicyAction,
    ) -> Self {
        Self {
            outcome: PolicyOutcome::Reject,
            action,
            reason,
            verification_required: true,
        }
    }

    /// Returns the policy outcome.
    pub const fn outcome(self) -> PolicyOutcome {
        self.outcome
    }

    /// Returns the selected policy action.
    pub const fn action(self) -> PolicyAction {
        self.action
    }

    /// Returns the decision reason.
    pub const fn reason(self) -> PolicyReason {
        self.reason
    }

    /// Returns whether verification is required.
    pub const fn verification_required(self) -> bool {
        self.verification_required
    }

    /// Returns whether the decision permits autonomous continuation.
    pub const fn permits_continuation(self) -> bool {
        self.outcome.permits_autonomous_continuation()
    }

    /// Returns whether the decision requires escalation.
    pub const fn requires_escalation(self) -> bool {
        self.outcome.requires_escalation()
    }

    /// Returns whether the decision rejects the operation/result.
    pub const fn rejects(self) -> bool {
        self.outcome.rejects()
    }
}

// =============================================================================
// Policy validation
// =============================================================================

/// Policy configuration validation error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PolicyValidationError {
    /// The minimum confidence configuration is inconsistent.
    InvalidConfidence,

    /// An escalation rule is enabled while escalation is globally disabled.
    EscalationConfigurationConflict,

    /// QEC adaptation is marked required while the QEC action is denied.
    RequiredQecActionDenied,

    /// Mitigation is marked required while the mitigation action is denied.
    RequiredMitigationActionDenied,

    /// A recovery rule is autonomous without being allowed.
    AutonomousRuleDenied(PolicyAction),
}

impl fmt::Display for PolicyValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidConfidence => {
                formatter.write_str(
                    "resilience policy contains invalid confidence configuration",
                )
            }

            Self::EscalationConfigurationConflict => {
                formatter.write_str(
                    "resilience policy enables escalation behavior while escalation is globally disabled",
                )
            }

            Self::RequiredQecActionDenied => {
                formatter.write_str(
                    "resilience policy requires QEC adaptation while the QEC action is denied",
                )
            }

            Self::RequiredMitigationActionDenied => {
                formatter.write_str(
                    "resilience policy requires mitigation while the mitigation action is denied",
                )
            }

            Self::AutonomousRuleDenied(action) => {
                write!(
                    formatter,
                    "resilience policy marks action `{action}` autonomous while it is denied"
                )
            }
        }
    }
}

impl std::error::Error for PolicyValidationError {}

impl ResiliencePolicy {
    /// Validates the internal policy configuration.
    ///
    /// This function performs only local validation. Cross-subsystem
    /// capability validation belongs to the planner/controller.
    pub fn validate(&self) -> Result<(), PolicyValidationError> {
        if self.diagnosis_confidence.units() > PolicyConfidence::SCALE {
            return Err(PolicyValidationError::InvalidConfidence);
        }

        if self.escalate_rule.autonomous() && !self.escalation_allowed {
            return Err(
                PolicyValidationError::EscalationConfigurationConflict,
            );
        }

        if self.qec_adaptation == QecAdaptationPermission::Required
            && !self.qec_rule.allowed()
        {
            return Err(PolicyValidationError::RequiredQecActionDenied);
        }

        if self.mitigation == MitigationPermission::Required
            && !self.mitigate_rule.allowed()
        {
            return Err(
                PolicyValidationError::RequiredMitigationActionDenied,
            );
        }

        let actions = [
            PolicyAction::Continue,
            PolicyAction::Retry,
            PolicyAction::Restart,
            PolicyAction::Resume,
            PolicyAction::Rollback,
            PolicyAction::Remap,
            PolicyAction::Reroute,
            PolicyAction::Reschedule,
            PolicyAction::Recompile,
            PolicyAction::Reoptimize,
            PolicyAction::AdaptQec,
            PolicyAction::Mitigate,
            PolicyAction::Migrate,
            PolicyAction::Compensate,
            PolicyAction::Escalate,
            PolicyAction::Reject,
        ];

        for action in actions {
            let rule = self.rule(action);

            if rule.autonomous() && !rule.allowed() {
                return Err(
                    PolicyValidationError::AutonomousRuleDenied(action),
                );
            }
        }

        Ok(())
    }
}

// =============================================================================
// Policy builder
// =============================================================================

/// Builder for explicit policy customization.
///
/// The builder does not contain hardware-specific defaults.
#[derive(Debug, Clone)]
pub struct ResiliencePolicyBuilder {
    policy: ResiliencePolicy,
}

impl ResiliencePolicyBuilder {
    /// Creates a builder from a conservative policy.
    pub const fn new() -> Self {
        Self {
            policy: ResiliencePolicy::conservative(),
        }
    }

    /// Creates a builder from request options.
    pub fn from_request(
        request: &ResilienceRequest,
    ) -> Self {
        Self {
            policy: ResiliencePolicy::from_request(request),
        }
    }

    /// Sets diagnosis confidence.
    pub const fn with_diagnosis_confidence(
        mut self,
        confidence: PolicyConfidence,
    ) -> Self {
        self.policy.diagnosis_confidence = confidence;
        self
    }

    /// Controls uncertain autonomous actions.
    pub const fn with_uncertain_autonomous_action(
        mut self,
        allowed: bool,
    ) -> Self {
        self.policy.allow_uncertain_autonomous_action = allowed;
        self
    }

    /// Sets resource preference.
    pub const fn with_resource_preference(
        mut self,
        preference: ResourcePreference,
    ) -> Self {
        self.policy.resource_preference = preference;
        self
    }

    /// Sets semantic guarantee.
    pub const fn with_semantic_guarantee(
        mut self,
        guarantee: SemanticGuarantee,
    ) -> Self {
        self.policy.semantic_guarantee = guarantee;
        self
    }

    /// Sets execution mode.
    pub const fn with_execution_mode(
        mut self,
        mode: ResilienceExecutionMode,
    ) -> Self {
        self.policy.execution_mode = mode;
        self
    }

    /// Sets adaptation permissions.
    pub const fn with_adaptation(
        mut self,
        permissions: AdaptationPermissions,
    ) -> Self {
        self.policy.adaptation = permissions;

        self.policy.remap_rule =
            self.policy.remap_rule.with_allowed(
                permissions.remapping_allowed(),
            );

        self.policy.reroute_rule =
            self.policy.reroute_rule.with_allowed(
                permissions.rerouting_allowed(),
            );

        self.policy.reschedule_rule =
            self.policy.reschedule_rule.with_allowed(
                permissions.rescheduling_allowed(),
            );

        self.policy.recompile_rule =
            self.policy.recompile_rule.with_allowed(
                permissions.recompilation_allowed(),
            );

        self.policy.reoptimize_rule =
            self.policy.reoptimize_rule.with_allowed(
                permissions.reoptimization_allowed(),
            );

        self.policy.migrate_rule =
            self.policy.migrate_rule.with_allowed(
                permissions.migration_allowed(),
            );

        self
    }

    /// Sets recovery permissions.
    pub const fn with_recovery(
        mut self,
        permissions: RecoveryPermissions,
    ) -> Self {
        self.policy.recovery = permissions;

        self.policy.retry_rule =
            self.policy.retry_rule.with_allowed(
                permissions.retry_allowed(),
            );

        self.policy.restart_rule =
            self.policy.restart_rule.with_allowed(
                permissions.restart_allowed(),
            );

        self.policy.resume_rule =
            self.policy.resume_rule.with_allowed(
                permissions.resume_allowed(),
            );

        self.policy.rollback_rule =
            self.policy.rollback_rule.with_allowed(
                permissions.rollback_allowed(),
            );

        self.policy.compensate_rule =
            self.policy.compensate_rule.with_allowed(
                permissions.compensation_allowed(),
            );

        self.policy.migrate_rule =
            self.policy.migrate_rule.with_allowed(
                permissions.migration_allowed(),
            );

        self
    }

    /// Sets mitigation permission.
    pub const fn with_mitigation(
        mut self,
        permission: MitigationPermission,
    ) -> Self {
        self.policy.mitigation = permission;

        self.policy.mitigate_rule =
            self.policy.mitigate_rule.with_allowed(
                !matches!(permission, MitigationPermission::Disabled),
            );

        self
    }

    /// Sets QEC adaptation permission.
    pub const fn with_qec_adaptation(
        mut self,
        permission: QecAdaptationPermission,
    ) -> Self {
        self.policy.qec_adaptation = permission;

        self.policy.qec_rule =
            self.policy.qec_rule.with_allowed(
                !matches!(permission, QecAdaptationPermission::Disabled),
            );

        self
    }

    /// Controls escalation.
    pub const fn with_escalation(
        mut self,
        allowed: bool,
    ) -> Self {
        self.policy.escalation_allowed = allowed;

        self.policy.escalate_rule =
            self.policy.escalate_rule.with_allowed(allowed);

        self
    }

    /// Overrides an individual action rule.
    pub const fn with_action_rule(
        mut self,
        action: PolicyAction,
        rule: ActionRule,
    ) -> Self {
        match action {
            PolicyAction::Continue => self.policy.continue_rule = rule,
            PolicyAction::Retry => self.policy.retry_rule = rule,
            PolicyAction::Restart => self.policy.restart_rule = rule,
            PolicyAction::Resume => self.policy.resume_rule = rule,
            PolicyAction::Rollback => self.policy.rollback_rule = rule,
            PolicyAction::Remap => self.policy.remap_rule = rule,
            PolicyAction::Reroute => self.policy.reroute_rule = rule,
            PolicyAction::Reschedule => {
                self.policy.reschedule_rule = rule
            }
            PolicyAction::Recompile => {
                self.policy.recompile_rule = rule
            }
            PolicyAction::Reoptimize => {
                self.policy.reoptimize_rule = rule
            }
            PolicyAction::AdaptQec => self.policy.qec_rule = rule,
            PolicyAction::Mitigate => {
                self.policy.mitigate_rule = rule
            }
            PolicyAction::Migrate => self.policy.migrate_rule = rule,
            PolicyAction::Compensate => {
                self.policy.compensate_rule = rule
            }
            PolicyAction::Escalate => {
                self.policy.escalate_rule = rule
            }
            PolicyAction::Reject => self.policy.reject_rule = rule,
        }

        self
    }

    /// Finalizes and validates the policy.
    pub fn build(
        self,
    ) -> Result<ResiliencePolicy, PolicyValidationError> {
        self.policy.validate()?;
        Ok(self.policy)
    }
}

impl Default for ResiliencePolicyBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Policy evaluation helper
// =============================================================================

/// Stateless policy evaluator.
///
/// Keeping the evaluator separate from `ResiliencePolicy` makes it possible
/// for the planner/controller to inject a policy without introducing global
/// state.
#[derive(Debug, Clone, Copy, Default)]
pub struct PolicyEvaluator;

impl PolicyEvaluator {
    /// Creates an evaluator.
    pub const fn new() -> Self {
        Self
    }

    /// Evaluates an explicit policy/input pair.
    pub fn evaluate(
        &self,
        policy: &ResiliencePolicy,
        input: PolicyInput,
    ) -> PolicyDecision {
        policy.evaluate(input)
    }

    /// Evaluates using a policy constructed from a request.
    pub fn evaluate_request(
        &self,
        request: &ResilienceRequest,
        input: PolicyInput,
    ) -> PolicyDecision {
        let policy = ResiliencePolicy::from_request(request);
        policy.evaluate(input)
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn trusted_valid_input() -> PolicyInput {
        PolicyInput::new(
            DiagnosisState::Benign,
            PolicyConfidence::MAX,
            EvidenceTrust::Trusted,
            VerificationState::Verified,
            CapabilityState::Available,
            ResourceState::Available,
            PolicyBudgetState::unlimited(),
            true,
            false,
        )
    }

    #[test]
    fn policy_action_names_are_stable() {
        assert_eq!(PolicyAction::Continue.as_str(), "continue");
        assert_eq!(PolicyAction::Retry.as_str(), "retry");
        assert_eq!(PolicyAction::Reroute.as_str(), "reroute");
        assert_eq!(PolicyAction::AdaptQec.as_str(), "adapt_qec");
        assert_eq!(PolicyAction::Mitigate.as_str(), "mitigate");
        assert_eq!(PolicyAction::Migrate.as_str(), "migrate");
        assert_eq!(PolicyAction::Escalate.as_str(), "escalate");
        assert_eq!(PolicyAction::Reject.as_str(), "reject");
    }

    #[test]
    fn policy_outcome_names_are_stable() {
        assert_eq!(PolicyOutcome::Allow.as_str(), "allow");
        assert_eq!(PolicyOutcome::Adapt.as_str(), "adapt");
        assert_eq!(PolicyOutcome::Recover.as_str(), "recover");
        assert_eq!(PolicyOutcome::Mitigate.as_str(), "mitigate");
        assert_eq!(PolicyOutcome::Verify.as_str(), "verify");
        assert_eq!(PolicyOutcome::Escalate.as_str(), "escalate");
        assert_eq!(PolicyOutcome::Reject.as_str(), "reject");
    }

    #[test]
    fn confidence_rejects_invalid_values() {
        assert!(PolicyConfidence::from_f64(-1.0).is_none());
        assert!(PolicyConfidence::from_f64(2.0).is_none());
        assert!(PolicyConfidence::from_f64(f64::NAN).is_none());
        assert!(PolicyConfidence::from_f64(f64::INFINITY).is_none());

        assert!(PolicyConfidence::from_f64(0.0).is_some());
        assert!(PolicyConfidence::from_f64(0.5).is_some());
        assert!(PolicyConfidence::from_f64(1.0).is_some());
    }

    #[test]
    fn confidence_ratio_rejects_invalid_ratio() {
        assert!(PolicyConfidence::from_ratio(0, 0).is_none());
        assert!(PolicyConfidence::from_ratio(2, 1).is_none());
        assert_eq!(
            PolicyConfidence::from_ratio(1, 2)
                .expect("valid ratio")
                .as_f64(),
            0.5
        );
    }

    #[test]
    fn conservative_policy_is_fail_closed() {
        let policy = ResiliencePolicy::conservative();

        assert!(policy.validate().is_ok());
        assert!(!policy.adaptation.remapping_allowed());
        assert!(!policy.recovery.retry_allowed());
        assert_eq!(
            policy.mitigation,
            MitigationPermission::Disabled
        );
        assert_eq!(
            policy.qec_adaptation,
            QecAdaptationPermission::Disabled
        );
        assert!(policy.escalation_allowed);
    }

    #[test]
    fn trusted_benign_execution_can_continue() {
        let policy = ResiliencePolicy::conservative();
        let input = trusted_valid_input();

        let decision = policy.evaluate(input);

        assert_eq!(decision.action(), PolicyAction::Continue);
        assert_eq!(decision.outcome(), PolicyOutcome::Allow);
    }

    #[test]
    fn stale_policy_state_is_not_accepted() {
        let policy = ResiliencePolicy::conservative();

        let input = PolicyInput::new(
            DiagnosisState::Benign,
            PolicyConfidence::MAX,
            EvidenceTrust::Trusted,
            VerificationState::Verified,
            CapabilityState::Available,
            ResourceState::Available,
            PolicyBudgetState::unlimited(),
            true,
            true,
        );

        let decision = policy.evaluate(input);

        assert_eq!(decision.outcome(), PolicyOutcome::Reject);
        assert_eq!(decision.action(), PolicyAction::Escalate);
        assert_eq!(
            decision.reason(),
            PolicyReason::StalePolicyState
        );
    }

    #[test]
    fn invalid_execution_state_fails_closed() {
        let policy = ResiliencePolicy::conservative();

        let input = PolicyInput::new(
            DiagnosisState::Benign,
            PolicyConfidence::MAX,
            EvidenceTrust::Trusted,
            VerificationState::Verified,
            CapabilityState::Available,
            ResourceState::Available,
            PolicyBudgetState::unlimited(),
            false,
            false,
        );

        let decision = policy.evaluate(input);

        assert_eq!(decision.action(), PolicyAction::Escalate);
        assert_eq!(
            decision.reason(),
            PolicyReason::ResourceConstraint
        );
    }

    #[test]
    fn untrusted_intervention_does_not_authorize_autonomous_action() {
        let policy = ResiliencePolicy::conservative();

        let input = PolicyInput::new(
            DiagnosisState::RecoveryRequired,
            PolicyConfidence::MAX,
            EvidenceTrust::Untrusted,
            VerificationState::NotPerformed,
            CapabilityState::Available,
            ResourceState::Available,
            PolicyBudgetState::unlimited(),
            true,
            false,
        );

        let decision = policy.evaluate(input);

        assert_eq!(decision.action(), PolicyAction::Escalate);
        assert_eq!(
            decision.reason(),
            PolicyReason::UntrustedEvidence
        );
    }

    #[test]
    fn conflicting_diagnosis_escalates() {
        let policy = ResiliencePolicy::conservative();

        let input = PolicyInput::new(
            DiagnosisState::Conflicting,
            PolicyConfidence::MAX,
            EvidenceTrust::Trusted,
            VerificationState::NotPerformed,
            CapabilityState::Available,
            ResourceState::Available,
            PolicyBudgetState::unlimited(),
            true,
            false,
        );

        let decision = policy.evaluate(input);

        assert_eq!(decision.action(), PolicyAction::Escalate);
        assert_eq!(
            decision.reason(),
            PolicyReason::ConflictingEvidence
        );
    }

    #[test]
    fn critical_condition_escalates() {
        let policy = ResiliencePolicy::conservative();

        let input = PolicyInput::new(
            DiagnosisState::Critical,
            PolicyConfidence::MAX,
            EvidenceTrust::Trusted,
            VerificationState::NotPerformed,
            CapabilityState::Available,
            ResourceState::Available,
            PolicyBudgetState::unlimited(),
            true,
            false,
        );

        let decision = policy.evaluate(input);

        assert_eq!(decision.outcome(), PolicyOutcome::Escalate);
        assert_eq!(decision.action(), PolicyAction::Escalate);
    }

    #[test]
    fn adaptation_requires_explicit_permission() {
        let policy = ResiliencePolicy::conservative();

        let input = PolicyInput::new(
            DiagnosisState::AdaptationRequired,
            PolicyConfidence::MAX,
            EvidenceTrust::Trusted,
            VerificationState::NotPerformed,
            CapabilityState::Available,
            ResourceState::Available,
            PolicyBudgetState::unlimited(),
            true,
            false,
        );

        let decision = policy.evaluate(input);

        assert_eq!(decision.action(), PolicyAction::Escalate);
    }

    #[test]
    fn adaptation_can_be_enabled_without_hardware_assumptions() {
        let permissions = AdaptationPermissions::deny_all()
            .with_rerouting(true);

        let policy = ResiliencePolicyBuilder::new()
            .with_adaptation(permissions)
            .build()
            .expect("valid policy");

        let input = PolicyInput::new(
            DiagnosisState::AdaptationRequired,
            PolicyConfidence::MAX,
            EvidenceTrust::Trusted,
            VerificationState::NotPerformed,
            CapabilityState::Available,
            ResourceState::Available,
            PolicyBudgetState::unlimited(),
            true,
            false,
        );

        let decision = policy.evaluate(input);

        assert_eq!(decision.action(), PolicyAction::Reroute);
        assert_eq!(decision.outcome(), PolicyOutcome::Adapt);
    }

    #[test]
    fn retry_budget_is_not_hard_coded() {
        let permissions = RecoveryPermissions::deny_all()
            .with_retry(true)
            .with_restart(true);

        let policy = ResiliencePolicyBuilder::new()
            .with_recovery(permissions)
            .build()
            .expect("valid policy");

        let input = PolicyInput::new(
            DiagnosisState::RecoveryRequired,
            PolicyConfidence::MAX,
            EvidenceTrust::Trusted,
            VerificationState::NotPerformed,
            CapabilityState::Available,
            ResourceState::Available,
            PolicyBudgetState::new(
                NonZeroU64::new(1),
                None,
                None,
                false,
            ),
            true,
            false,
        );

        let decision = policy.evaluate(input);

        assert_eq!(decision.action(), PolicyAction::Restart);
    }

    #[test]
    fn unavailable_target_can_trigger_migration_when_allowed() {
        let adaptation = AdaptationPermissions::deny_all()
            .with_migration(true);

        let recovery = RecoveryPermissions::deny_all()
            .with_migration(true);

        let policy = ResiliencePolicyBuilder::new()
            .with_adaptation(adaptation)
            .with_recovery(recovery)
            .build()
            .expect("valid policy");

        let input = PolicyInput::new(
            DiagnosisState::RecoveryRequired,
            PolicyConfidence::MAX,
            EvidenceTrust::Trusted,
            VerificationState::NotPerformed,
            CapabilityState::Unavailable,
            ResourceState::Unavailable,
            PolicyBudgetState::unlimited(),
            true,
            false,
        );

        let decision = policy.evaluate(input);

        assert_eq!(decision.action(), PolicyAction::Migrate);
        assert_eq!(decision.outcome(), PolicyOutcome::Recover);
    }

    #[test]
    fn mitigation_permission_controls_mitigation() {
        let policy = ResiliencePolicyBuilder::new()
            .with_mitigation(MitigationPermission::Allowed)
            .build()
            .expect("valid policy");

        let input = PolicyInput::new(
            DiagnosisState::MitigationRequired,
            PolicyConfidence::MAX,
            EvidenceTrust::Trusted,
            VerificationState::NotPerformed,
            CapabilityState::Available,
            ResourceState::Available,
            PolicyBudgetState::unlimited(),
            true,
            false,
        );

        let decision = policy.evaluate(input);

        assert_eq!(decision.action(), PolicyAction::Mitigate);
        assert_eq!(decision.outcome(), PolicyOutcome::Mitigate);
        assert!(decision.verification_required());
    }

    #[test]
    fn failed_verification_never_becomes_success() {
        let policy = ResiliencePolicy::conservative();

        let input = PolicyInput::new(
            DiagnosisState::Benign,
            PolicyConfidence::MAX,
            EvidenceTrust::Trusted,
            VerificationState::Failed,
            CapabilityState::Available,
            ResourceState::Available,
            PolicyBudgetState::unlimited(),
            true,
            false,
        );

        let decision = policy.evaluate(input);

        assert_ne!(decision.outcome(), PolicyOutcome::Allow);
        assert_ne!(decision.outcome(), PolicyOutcome::Recover);
    }

    #[test]
    fn schema_identity_is_stable() {
        assert_eq!(
            RESILIENCE_POLICY_SCHEMA_ID,
            "zamani.quantum.resilience.policy"
        );

        assert_eq!(RESILIENCE_POLICY_SCHEMA_VERSION, 1);
    }
}