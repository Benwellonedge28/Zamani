//! Zamani Quantum Resilience — Escalation Policy
//!
//! Path:
//!     src/quantum/resilience/policy/escalation.rs
//!
//! Purpose:
//!     Defines the provider-independent, deterministic escalation policy used
//!     by the resilience policy layer when autonomous action is no longer
//!     sufficiently safe, permitted, feasible, trustworthy, or useful.
//!
//! Architectural role:
//!
//!     DETECT
//!        |
//!        v
//!     DIAGNOSE
//!        |
//!        v
//!     POLICY
//!        |
//!        +------------------+
//!        |                  |
//!        v                  v
//!     PLAN             ESCALATE  <--- this module
//!        |                  |
//!        v                  v
//!     ADAPT / RECOVER    HUMAN / EXTERNAL AUTHORITY
//!        |
//!        v
//!     VERIFY
//!
//! This module defines WHEN resilience should stop autonomous progression and
//! require escalation. It does not perform the escalation itself.
//!
//! ----------------------------------------------------------------------------
//! Ownership boundaries
//! ----------------------------------------------------------------------------
//!
//! This module MUST NOT own:
//!
//! - quantum IR;
//! - quantum gates;
//! - quantum operations;
//! - quantum circuits;
//! - logical qubit identity;
//! - physical qubit identity;
//! - hardware discovery;
//! - hardware calibration;
//! - backend/provider implementations;
//! - routing;
//! - scheduling;
//! - optimization;
//! - compilation;
//! - QEC implementation;
//! - decoder implementation;
//! - mitigation implementation;
//! - recovery implementation;
//! - execution;
//! - telemetry collection;
//! - authentication;
//! - authorization credentials;
//! - network I/O;
//! - filesystem I/O;
//! - persistence;
//! - hidden global state;
//! - background threads;
//! - random selection;
//! - retry loops.
//!
//! The authoritative quantum identity types remain:
//!
//!     crate::quantum::ir::qubit::QubitId
//!     crate::quantum::ir::qubit::PhysicalQubitId
//!
//! This module intentionally does not import or redefine them because
//! escalation decisions are resource-agnostic.
//!
//! ----------------------------------------------------------------------------
//! Relationship to the rest of resilience
//! ----------------------------------------------------------------------------
//!
//! `policy/constraints.rs`
//!     Determines whether a candidate action is permitted.
//!
//! `policy/objectives.rs`
//!     Determines which feasible candidate is preferred.
//!
//! `policy/budgets.rs`
//!     Owns budget semantics.
//!
//! `policy/safety.rs`
//!     Owns safety authorization.
//!
//! `policy/retry.rs`
//!     Owns retry-specific policy.
//!
//! `policy/escalation.rs`
//!     Determines when autonomous policy evaluation must stop and escalate.
//!
//! `policy/policy.rs`
//!     Integrates escalation with the overall policy decision.
//!
//! `diagnosis/*`
//!     Supplies diagnosis facts and confidence.
//!
//! `planning/*`
//!     Produces candidate plans. This module does not create plans.
//!
//! `recovery/*`
//!     Executes recovery after policy/planning authorization.
//!
//! `verification/*`
//!     Determines whether an execution/result is acceptable.
//!
//! `telemetry/*`
//!     Supplies observations. This module does not collect telemetry.
//!
//! ----------------------------------------------------------------------------
//! Fundamental safety invariant
//! ----------------------------------------------------------------------------
//!
//! Escalation is fail-closed.
//!
//! If the system cannot establish that autonomous continuation is safe,
//! permitted, sufficiently evidenced, and potentially useful, escalation is
//! preferred over silently continuing.
//!
//! In particular:
//!
//!     availability != authorization
//!     recoverability != correctness
//!     confidence != verification
//!     prediction != proof
//!
//! Escalation MUST NOT be bypassed merely because continuing would improve
//! availability, reduce latency, reduce cost, or avoid operator involvement.
//!
//! ----------------------------------------------------------------------------
//! Write once, scale everywhere
//! ----------------------------------------------------------------------------
//!
//! This module contains no:
//!
//!     MAX_QUBITS
//!     MAX_PHYSICAL_QUBITS
//!     MAX_BACKENDS
//!     MAX_RECOVERY_ATTEMPTS
//!     MAX_ESCALATIONS
//!     DEFAULT_RETRY_COUNT
//!     DEFAULT_FIDELITY
//!     DEFAULT_LATENCY
//!     DEFAULT_RESOURCE_LIMIT
//!
//! Any finite escalation threshold is explicitly supplied by policy.
//!
//! An optional threshold represents an unbounded/unconfigured dimension.
//! Therefore:
//!
//!     None = no finite threshold imposed by this policy dimension
//!
//! This is not an assertion that a physical machine has infinite resources.
//! Actual resource availability is supplied by the hardware/runtime/resource
//! layers.
//!
//! The same escalation policy can therefore operate on:
//!
//!     one qubit
//!     -> small QPU
//!     -> large QPU
//!     -> logical/fault-tolerant system
//!     -> multiple QPUs
//!     -> heterogeneous distributed quantum execution.
//!
//! ----------------------------------------------------------------------------
//! Determinism
//! ----------------------------------------------------------------------------
//!
//! Evaluation is a pure function of explicit policy configuration and explicit
//! evaluation context.
//!
//! This module:
//!
//! - does not read a clock;
//! - does not read environment variables;
//! - does not perform I/O;
//! - does not access global mutable state;
//! - does not spawn threads;
//! - does not generate randomness;
//! - does not use HashMap iteration order;
//! - does not create implicit identifiers.
//!
//! Given identical inputs, evaluation produces identical output.
//!
//! ----------------------------------------------------------------------------
//! Security
//! ----------------------------------------------------------------------------
//!
//! Escalation decisions must not be based solely on untrusted observations.
//!
//! The caller must establish the trust state of observations before passing
//! them into an autonomous policy decision.
//!
//! This module can nevertheless fail closed when:
//!
//! - evidence is explicitly untrusted;
//! - evidence conflicts;
//! - diagnosis is unavailable;
//! - verification failed;
//! - a safety boundary was violated;
//! - an authorization boundary was violated.
//!
//! Authentication and authorization mechanisms themselves belong elsewhere.
//!
//! ----------------------------------------------------------------------------
//! Floating-point policy
//! ----------------------------------------------------------------------------
//!
//! Confidence values are represented as `f64` only at this boundary because
//! policy configuration commonly originates from serialized configuration,
//! user input, or external systems.
//!
//! All floating-point values are validated:
//!
//!     finite
//!     0.0 <= confidence <= 1.0
//!
//! No NaN or infinity may enter the escalation evaluator.
//!
//! ----------------------------------------------------------------------------
//! Rust contract
//! ----------------------------------------------------------------------------
//!
//! Target:
//!     Rust 1.97 / Rust 1.97.1
//!
//! Edition:
//!     Rust 2021
//!
//! Safety:
//!     No unsafe code.
//!
//! Dependencies:
//!     Standard library only.
//!
//! Serialization:
//!     Serialization should be implemented by the higher-level resilience
//!     serialization adapter rather than coupling this contract to serde or
//!     another wire format.
//!
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_debug_implementations)]

use std::fmt;

// =============================================================================
// Schema
// =============================================================================

/// Stable schema identifier for the escalation policy contract.
pub const RESILIENCE_ESCALATION_SCHEMA_ID: &str =
    "zamani.quantum.resilience.policy.escalation";

/// Semantic version of this contract.
///
/// This version is independent of the Zamani compiler, IR, crate and hardware
/// schema versions.
pub const RESILIENCE_ESCALATION_SCHEMA_VERSION: u16 = 1;

// =============================================================================
// Validation
// =============================================================================

/// Errors produced when constructing an invalid escalation policy or context.
#[derive(Debug, Clone, PartialEq)]
pub enum EscalationError {
    /// A floating-point value was NaN or infinite.
    NonFiniteValue {
        /// Name of the invalid field.
        field: &'static str,

        /// Invalid value.
        value: f64,
    },

    /// A probability/confidence value was outside [0, 1].
    ProbabilityOutOfRange {
        /// Name of the invalid field.
        field: &'static str,

        /// Invalid value.
        value: f64,
    },

    /// A textual identifier was empty.
    EmptyIdentifier {
        /// Name of the invalid identifier.
        field: &'static str,
    },

    /// A textual identifier contained a control character.
    InvalidIdentifier {
        /// Name of the invalid identifier.
        field: &'static str,
    },

    /// A policy contains mutually incompatible settings.
    InvalidPolicy {
        /// Human-readable explanation.
        reason: &'static str,
    },
}

impl fmt::Display for EscalationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFiniteValue { field, value } => {
                write!(formatter, "{field} contains non-finite value {value}")
            }
            Self::ProbabilityOutOfRange { field, value } => {
                write!(
                    formatter,
                    "{field} must be within [0, 1], received {value}"
                )
            }
            Self::EmptyIdentifier { field } => {
                write!(formatter, "{field} must not be empty")
            }
            Self::InvalidIdentifier { field } => {
                write!(
                    formatter,
                    "{field} must not contain control characters"
                )
            }
            Self::InvalidPolicy { reason } => {
                write!(formatter, "invalid escalation policy: {reason}")
            }
        }
    }
}

impl std::error::Error for EscalationError {}

// =============================================================================
// Escalation trigger
// =============================================================================

/// Reason an execution may require escalation.
///
/// These are semantic reasons rather than implementation-specific failures.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum EscalationTrigger {
    /// No escalation condition was established.
    None,

    /// Diagnosis confidence is below the configured autonomous-action
    /// requirement.
    InsufficientConfidence,

    /// Evidence sources disagree.
    ConflictingEvidence,

    /// Evidence is explicitly untrusted.
    UntrustedEvidence,

    /// The diagnosis is unknown and autonomous action cannot be justified.
    UnknownCondition,

    /// The requested recovery/adaptation is not semantically safe.
    SemanticSafety,

    /// The requested action is not authorized by policy.
    Authorization,

    /// Required hardware/resource capability is unavailable.
    CapabilityUnavailable,

    /// Required resource state changed and the existing plan may be stale.
    ResourceChanged,

    /// A configured budget has been exhausted.
    BudgetExhausted,

    /// The execution deadline can no longer be satisfied.
    DeadlineExceeded,

    /// Recovery has reached its configured finite attempt boundary.
    RecoveryAttemptLimit,

    /// The policy has reached its configured finite escalation boundary.
    EscalationAttemptLimit,

    /// Recovery/adaptation failed repeatedly.
    RepeatedFailure,

    /// Recovery produced a result that cannot be verified.
    VerificationFailure,

    /// Verification is inconclusive.
    VerificationInconclusive,

    /// A safety invariant was violated.
    SafetyInvariantViolation,

    /// A required external authority is unavailable.
    ExternalAuthorityUnavailable,

    /// The current state is inconsistent or stale.
    StateInconsistency,

    /// No safe autonomous action remains.
    NoSafeAction,

    /// The caller explicitly requires escalation.
    CallerRequired,

    /// The caller explicitly requested operator/external authority.
    CallerRequested,
}

impl EscalationTrigger {
    /// Stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::InsufficientConfidence => "insufficient_confidence",
            Self::ConflictingEvidence => "conflicting_evidence",
            Self::UntrustedEvidence => "untrusted_evidence",
            Self::UnknownCondition => "unknown_condition",
            Self::SemanticSafety => "semantic_safety",
            Self::Authorization => "authorization",
            Self::CapabilityUnavailable => "capability_unavailable",
            Self::ResourceChanged => "resource_changed",
            Self::BudgetExhausted => "budget_exhausted",
            Self::DeadlineExceeded => "deadline_exceeded",
            Self::RecoveryAttemptLimit => "recovery_attempt_limit",
            Self::EscalationAttemptLimit => "escalation_attempt_limit",
            Self::RepeatedFailure => "repeated_failure",
            Self::VerificationFailure => "verification_failure",
            Self::VerificationInconclusive => "verification_inconclusive",
            Self::SafetyInvariantViolation => "safety_invariant_violation",
            Self::ExternalAuthorityUnavailable => "external_authority_unavailable",
            Self::StateInconsistency => "state_inconsistency",
            Self::NoSafeAction => "no_safe_action",
            Self::CallerRequired => "caller_required",
            Self::CallerRequested => "caller_requested",
        }
    }

    /// Returns whether the trigger represents a safety-critical condition.
    pub const fn is_safety_critical(self) -> bool {
        matches!(
            self,
            Self::SemanticSafety
                | Self::Authorization
                | Self::SafetyInvariantViolation
                | Self::VerificationFailure
                | Self::NoSafeAction
                | Self::UntrustedEvidence
                | Self::ConflictingEvidence
        )
    }

    /// Returns whether the trigger is primarily evidence-related.
    pub const fn is_evidence_related(self) -> bool {
        matches!(
            self,
            Self::InsufficientConfidence
                | Self::ConflictingEvidence
                | Self::UntrustedEvidence
                | Self::UnknownCondition
                | Self::VerificationInconclusive
        )
    }
}

impl fmt::Display for EscalationTrigger {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Escalation mode
// =============================================================================

/// Defines how aggressively a policy escalates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EscalationMode {
    /// Escalation is disabled unless explicitly required by a safety invariant
    /// or caller requirement.
    Disabled,

    /// Escalation occurs when configured conditions are met.
    Conditional,

    /// Any unresolved safety-critical condition requires escalation.
    FailClosed,

    /// Every autonomous recovery boundary requires external approval.
    ///
    /// This is useful for highly controlled deployments.
    OperatorRequired,
}

impl EscalationMode {
    /// Stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Disabled => "disabled",
            Self::Conditional => "conditional",
            Self::FailClosed => "fail_closed",
            Self::OperatorRequired => "operator_required",
        }
    }

    /// Returns whether safety-critical conditions must escalate.
    pub const fn requires_safety_escalation(self) -> bool {
        matches!(self, Self::FailClosed | Self::OperatorRequired)
    }
}

impl Default for EscalationMode {
    fn default() -> Self {
        Self::FailClosed
    }
}

impl fmt::Display for EscalationMode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Escalation destination
// =============================================================================

/// Identifies the semantic destination of an escalation.
///
/// This module does not implement communication with the destination.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum EscalationDestination {
    /// The caller/runtime owns the next decision.
    Caller,

    /// A human/operator authority.
    Operator,

    /// An external policy authority.
    ExternalAuthority,

    /// A supervisory runtime/controller.
    Supervisor,

    /// A custom deployment-defined destination.
    Custom(String),
}

impl EscalationDestination {
    /// Creates a custom destination.
    pub fn custom<S>(name: S) -> Result<Self, EscalationError>
    where
        S: Into<String>,
    {
        let name = name.into();

        if name.is_empty() {
            return Err(EscalationError::EmptyIdentifier {
                field: "escalation destination",
            });
        }

        if name.chars().any(char::is_control) {
            return Err(EscalationError::InvalidIdentifier {
                field: "escalation destination",
            });
        }

        Ok(Self::Custom(name))
    }

    /// Stable machine-readable representation.
    pub fn as_str(&self) -> &str {
        match self {
            Self::Caller => "caller",
            Self::Operator => "operator",
            Self::ExternalAuthority => "external_authority",
            Self::Supervisor => "supervisor",
            Self::Custom(name) => name.as_str(),
        }
    }
}

impl fmt::Display for EscalationDestination {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Thresholds
// =============================================================================

/// Explicit escalation thresholds.
///
/// Every field is optional so that a deployment can leave a dimension
/// unbounded/unconfigured rather than introducing a hidden finite limit.
///
/// `None` means that this specific finite threshold is not imposed here.
/// It does NOT mean that safety checks or resource constraints are ignored.
#[derive(Debug, Clone, PartialEq)]
pub struct EscalationThresholds {
    /// Minimum diagnosis confidence required for autonomous action.
    ///
    /// Inclusive range [0, 1].
    pub minimum_confidence: Option<f64>,

    /// Maximum number of recovery attempts before escalation.
    ///
    /// `None` means no finite attempt limit is imposed by this policy.
    pub maximum_recovery_attempts: Option<u64>,

    /// Maximum number of escalation cycles represented by this policy.
    ///
    /// `None` means no finite cycle limit is imposed here.
    pub maximum_escalation_cycles: Option<u64>,

    /// Maximum number of consecutive recovery failures before escalation.
    pub maximum_consecutive_failures: Option<u64>,

    /// Maximum elapsed execution time, when supplied by the caller.
    ///
    /// `None` means this policy does not impose a deadline.
    pub maximum_elapsed_nanos: Option<u128>,

    /// Maximum accumulated recovery cost expressed in caller-defined units.
    ///
    /// `None` means this policy does not impose a finite recovery-cost limit.
    pub maximum_recovery_cost: Option<f64>,
}

impl Default for EscalationThresholds {
    fn default() -> Self {
        Self {
            minimum_confidence: None,
            maximum_recovery_attempts: None,
            maximum_escalation_cycles: None,
            maximum_consecutive_failures: None,
            maximum_elapsed_nanos: None,
            maximum_recovery_cost: None,
        }
    }
}

impl EscalationThresholds {
    /// Creates an unlimited threshold configuration.
    pub const fn unlimited() -> Self {
        Self {
            minimum_confidence: None,
            maximum_recovery_attempts: None,
            maximum_escalation_cycles: None,
            maximum_consecutive_failures: None,
            maximum_elapsed_nanos: None,
            maximum_recovery_cost: None,
        }
    }

    /// Validates all configured thresholds.
    pub fn validate(&self) -> Result<(), EscalationError> {
        if let Some(value) = self.minimum_confidence {
            validate_probability("minimum_confidence", value)?;
        }

        if let Some(value) = self.maximum_recovery_cost {
            if !value.is_finite() {
                return Err(EscalationError::NonFiniteValue {
                    field: "maximum_recovery_cost",
                    value,
                });
            }

            if value < 0.0 {
                return Err(EscalationError::InvalidPolicy {
                    reason: "maximum_recovery_cost cannot be negative",
                });
            }
        }

        Ok(())
    }
}

// =============================================================================
// Evaluation context
// =============================================================================

/// Explicit facts supplied to the escalation evaluator.
///
/// This is deliberately a policy-facing projection rather than a duplicate of
/// diagnosis, hardware, telemetry, verification, or execution state models.
///
/// The authoritative subsystem owns the original data and supplies these facts
/// to policy.
#[derive(Debug, Clone, PartialEq)]
pub struct EscalationContext {
    /// Whether the caller explicitly requires escalation.
    pub caller_requires_escalation: bool,

    /// Whether the caller explicitly requested escalation.
    pub caller_requested_escalation: bool,

    /// Current diagnosis confidence, if one exists.
    pub diagnosis_confidence: Option<f64>,

    /// Whether evidence sources conflict.
    pub evidence_conflicting: bool,

    /// Whether evidence is explicitly untrusted.
    pub evidence_untrusted: bool,

    /// Whether the current condition is unknown.
    pub condition_unknown: bool,

    /// Whether semantic safety would be violated by autonomous continuation.
    pub semantic_safety_violation: bool,

    /// Whether the proposed autonomous action is unauthorized.
    pub authorization_violation: bool,

    /// Whether required capabilities are unavailable.
    pub capability_unavailable: bool,

    /// Whether the relevant resource state changed since planning.
    pub resource_state_changed: bool,

    /// Whether a required budget is exhausted.
    pub budget_exhausted: bool,

    /// Whether the current deadline has already been exceeded.
    pub deadline_exceeded: bool,

    /// Number of recovery attempts already consumed.
    pub recovery_attempts: u64,

    /// Number of escalation cycles already consumed.
    pub escalation_cycles: u64,

    /// Number of consecutive recovery failures.
    pub consecutive_failures: u64,

    /// Elapsed execution time represented in nanoseconds.
    pub elapsed_nanos: u128,

    /// Accumulated recovery cost in caller-defined units.
    pub accumulated_recovery_cost: f64,

    /// Whether the most recent recovery result failed verification.
    pub verification_failed: bool,

    /// Whether verification is inconclusive.
    pub verification_inconclusive: bool,

    /// Whether a safety invariant was violated.
    pub safety_invariant_violated: bool,

    /// Whether a required external authority is unavailable.
    pub external_authority_unavailable: bool,

    /// Whether the current policy/execution state is inconsistent.
    pub state_inconsistent: bool,

    /// Whether no safe autonomous action remains.
    pub no_safe_action: bool,
}

impl Default for EscalationContext {
    fn default() -> Self {
        Self {
            caller_requires_escalation: false,
            caller_requested_escalation: false,
            diagnosis_confidence: None,
            evidence_conflicting: false,
            evidence_untrusted: false,
            condition_unknown: false,
            semantic_safety_violation: false,
            authorization_violation: false,
            capability_unavailable: false,
            resource_state_changed: false,
            budget_exhausted: false,
            deadline_exceeded: false,
            recovery_attempts: 0,
            escalation_cycles: 0,
            consecutive_failures: 0,
            elapsed_nanos: 0,
            accumulated_recovery_cost: 0.0,
            verification_failed: false,
            verification_inconclusive: false,
            safety_invariant_violated: false,
            external_authority_unavailable: false,
            state_inconsistent: false,
            no_safe_action: false,
        }
    }
}

impl EscalationContext {
    /// Validates externally supplied numeric observations.
    pub fn validate(&self) -> Result<(), EscalationError> {
        if let Some(value) = self.diagnosis_confidence {
            validate_probability("diagnosis_confidence", value)?;
        }

        if !self.accumulated_recovery_cost.is_finite() {
            return Err(EscalationError::NonFiniteValue {
                field: "accumulated_recovery_cost",
                value: self.accumulated_recovery_cost,
            });
        }

        if self.accumulated_recovery_cost < 0.0 {
            return Err(EscalationError::InvalidPolicy {
                reason: "accumulated_recovery_cost cannot be negative",
            });
        }

        Ok(())
    }
}

// =============================================================================
// Decision
// =============================================================================

/// Result of evaluating an escalation policy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EscalationDecision {
    /// Whether escalation is required.
    pub escalate: bool,

    /// Primary reason for the decision.
    pub trigger: EscalationTrigger,

    /// Semantic destination of escalation when escalation is required.
    pub destination: Option<EscalationDestination>,

    /// Whether the condition is safety-critical.
    pub safety_critical: bool,

    /// Whether autonomous continuation is permitted by this escalation layer.
    ///
    /// This is NOT a final execution authorization. Other policy, safety,
    /// capability, budget and verification gates remain authoritative.
    pub autonomous_continuation_permitted: bool,
}

impl EscalationDecision {
    /// Creates a non-escalation decision.
    pub fn continue_autonomously() -> Self {
        Self {
            escalate: false,
            trigger: EscalationTrigger::None,
            destination: None,
            safety_critical: false,
            autonomous_continuation_permitted: true,
        }
    }

    /// Creates an escalation decision.
    pub fn escalate(
        trigger: EscalationTrigger,
        destination: EscalationDestination,
    ) -> Self {
        Self {
            escalate: true,
            trigger,
            destination: Some(destination),
            safety_critical: trigger.is_safety_critical(),
            autonomous_continuation_permitted: false,
        }
    }

    /// Returns whether escalation is required.
    pub const fn requires_escalation(&self) -> bool {
        self.escalate
    }

    /// Returns whether autonomous continuation is allowed by this evaluator.
    pub const fn permits_autonomous_continuation(&self) -> bool {
        self.autonomous_continuation_permitted
    }
}

// =============================================================================
// Policy
// =============================================================================

/// Immutable, deterministic escalation policy.
///
/// This type contains only configuration. It does not execute anything and
/// owns no external state.
#[derive(Debug, Clone, PartialEq)]
pub struct EscalationPolicy {
    /// Escalation operating mode.
    pub mode: EscalationMode,

    /// Escalation destination.
    pub destination: EscalationDestination,

    /// Numeric thresholds.
    pub thresholds: EscalationThresholds,

    /// Whether verification failure must always escalate.
    pub escalate_on_verification_failure: bool,

    /// Whether inconclusive verification must escalate.
    pub escalate_on_verification_inconclusive: bool,

    /// Whether conflicting evidence must escalate.
    pub escalate_on_conflicting_evidence: bool,

    /// Whether untrusted evidence must escalate.
    pub escalate_on_untrusted_evidence: bool,

    /// Whether an unknown condition must escalate.
    pub escalate_on_unknown_condition: bool,

    /// Whether capability loss must escalate.
    pub escalate_on_capability_unavailable: bool,

    /// Whether state inconsistency must escalate.
    pub escalate_on_state_inconsistency: bool,

    /// Whether resource changes must escalate immediately rather than merely
    /// invalidating a previous plan.
    pub escalate_on_resource_change: bool,

    /// Whether an unavailable external authority itself is escalatory.
    pub escalate_on_external_authority_unavailable: bool,
}

impl Default for EscalationPolicy {
    fn default() -> Self {
        Self {
            // Fail closed is the safest default for an autonomous quantum
            // resilience system. It does not mean every ordinary degradation
            // escalates; it means safety-critical unresolved conditions do.
            mode: EscalationMode::FailClosed,

            destination: EscalationDestination::Caller,

            thresholds: EscalationThresholds::default(),

            escalate_on_verification_failure: true,
            escalate_on_verification_inconclusive: true,
            escalate_on_conflicting_evidence: true,
            escalate_on_untrusted_evidence: true,
            escalate_on_unknown_condition: true,
            escalate_on_capability_unavailable: true,
            escalate_on_state_inconsistency: true,
            escalate_on_resource_change: false,
            escalate_on_external_authority_unavailable: true,
        }
    }
}

impl EscalationPolicy {
    /// Validates the complete policy.
    pub fn validate(&self) -> Result<(), EscalationError> {
        self.thresholds.validate()?;

        if self.mode == EscalationMode::Disabled {
            // Disabled mode is legal, but safety-critical conditions remain
            // capable of being represented. The evaluator will fail closed
            // for mandatory safety boundaries rather than treating "disabled"
            // as permission to violate safety.
        }

        Ok(())
    }

    /// Evaluates the explicit context against this policy.
    ///
    /// Evaluation is deterministic and side-effect free.
    pub fn evaluate(
        &self,
        context: &EscalationContext,
    ) -> Result<EscalationDecision, EscalationError> {
        self.validate()?;
        context.validate()?;

        // ---------------------------------------------------------------------
        // Absolute safety boundaries.
        // ---------------------------------------------------------------------
        //
        // These are intentionally checked before all ordinary configuration.
        // Availability, cost, latency or policy mode cannot override them.
        if context.safety_invariant_violated {
            return Ok(EscalationDecision::escalate(
                EscalationTrigger::SafetyInvariantViolation,
                self.destination.clone(),
            ));
        }

        if context.semantic_safety_violation {
            return Ok(EscalationDecision::escalate(
                EscalationTrigger::SemanticSafety,
                self.destination.clone(),
            ));
        }

        if context.authorization_violation {
            return Ok(EscalationDecision::escalate(
                EscalationTrigger::Authorization,
                self.destination.clone(),
            ));
        }

        if context.verification_failed && self.escalate_on_verification_failure {
            return Ok(EscalationDecision::escalate(
                EscalationTrigger::VerificationFailure,
                self.destination.clone(),
            ));
        }

        // ---------------------------------------------------------------------
        // Explicit caller requirements.
        // ---------------------------------------------------------------------

        if context.caller_requires_escalation {
            return Ok(EscalationDecision::escalate(
                EscalationTrigger::CallerRequired,
                self.destination.clone(),
            ));
        }

        if context.caller_requested_escalation {
            return Ok(EscalationDecision::escalate(
                EscalationTrigger::CallerRequested,
                self.destination.clone(),
            ));
        }

        // ---------------------------------------------------------------------
        // Evidence integrity.
        // ---------------------------------------------------------------------

        if context.evidence_conflicting && self.escalate_on_conflicting_evidence {
            return Ok(EscalationDecision::escalate(
                EscalationTrigger::ConflictingEvidence,
                self.destination.clone(),
            ));
        }

        if context.evidence_untrusted && self.escalate_on_untrusted_evidence {
            return Ok(EscalationDecision::escalate(
                EscalationTrigger::UntrustedEvidence,
                self.destination.clone(),
            ));
        }

        // ---------------------------------------------------------------------
        // Unknown diagnosis.
        // ---------------------------------------------------------------------

        if context.condition_unknown && self.escalate_on_unknown_condition {
            return Ok(EscalationDecision::escalate(
                EscalationTrigger::UnknownCondition,
                self.destination.clone(),
            ));
        }

        // ---------------------------------------------------------------------
        // Confidence.
        // ---------------------------------------------------------------------

        if let Some(minimum) = self.thresholds.minimum_confidence {
            if let Some(confidence) = context.diagnosis_confidence {
                if confidence < minimum {
                    return Ok(EscalationDecision::escalate(
                        EscalationTrigger::InsufficientConfidence,
                        self.destination.clone(),
                    ));
                }
            } else {
                // A configured minimum confidence cannot be satisfied when
                // no confidence exists. Fail closed.
                return Ok(EscalationDecision::escalate(
                    EscalationTrigger::InsufficientConfidence,
                    self.destination.clone(),
                ));
            }
        }

        // ---------------------------------------------------------------------
        // Verification uncertainty.
        // ---------------------------------------------------------------------

        if context.verification_inconclusive
            && self.escalate_on_verification_inconclusive
        {
            return Ok(EscalationDecision::escalate(
                EscalationTrigger::VerificationInconclusive,
                self.destination.clone(),
            ));
        }

        // ---------------------------------------------------------------------
        // Capabilities/resources.
        // ---------------------------------------------------------------------

        if context.capability_unavailable
            && self.escalate_on_capability_unavailable
        {
            return Ok(EscalationDecision::escalate(
                EscalationTrigger::CapabilityUnavailable,
                self.destination.clone(),
            ));
        }

        if context.resource_state_changed && self.escalate_on_resource_change {
            return Ok(EscalationDecision::escalate(
                EscalationTrigger::ResourceChanged,
                self.destination.clone(),
            ));
        }

        // ---------------------------------------------------------------------
        // Budget/deadline boundaries.
        // ---------------------------------------------------------------------

        if context.budget_exhausted {
            return Ok(EscalationDecision::escalate(
                EscalationTrigger::BudgetExhausted,
                self.destination.clone(),
            ));
        }

        if context.deadline_exceeded {
            return Ok(EscalationDecision::escalate(
                EscalationTrigger::DeadlineExceeded,
                self.destination.clone(),
            ));
        }

        // ---------------------------------------------------------------------
        // Explicit configured attempt boundaries.
        // ---------------------------------------------------------------------

        if let Some(limit) = self.thresholds.maximum_recovery_attempts {
            if context.recovery_attempts >= limit {
                return Ok(EscalationDecision::escalate(
                    EscalationTrigger::RecoveryAttemptLimit,
                    self.destination.clone(),
                ));
            }
        }

        if let Some(limit) = self.thresholds.maximum_escalation_cycles {
            if context.escalation_cycles >= limit {
                return Ok(EscalationDecision::escalate(
                    EscalationTrigger::EscalationAttemptLimit,
                    self.destination.clone(),
                ));
            }
        }

        if let Some(limit) = self.thresholds.maximum_consecutive_failures {
            if context.consecutive_failures >= limit {
                return Ok(EscalationDecision::escalate(
                    EscalationTrigger::RepeatedFailure,
                    self.destination.clone(),
                ));
            }
        }

        if let Some(limit) = self.thresholds.maximum_elapsed_nanos {
            if context.elapsed_nanos >= limit {
                return Ok(EscalationDecision::escalate(
                    EscalationTrigger::DeadlineExceeded,
                    self.destination.clone(),
                ));
            }
        }

        if let Some(limit) = self.thresholds.maximum_recovery_cost {
            if context.accumulated_recovery_cost >= limit {
                return Ok(EscalationDecision::escalate(
                    EscalationTrigger::BudgetExhausted,
                    self.destination.clone(),
                ));
            }
        }

        // ---------------------------------------------------------------------
        // External authority availability.
        // ---------------------------------------------------------------------

        if context.external_authority_unavailable
            && self.escalate_on_external_authority_unavailable
        {
            return Ok(EscalationDecision::escalate(
                EscalationTrigger::ExternalAuthorityUnavailable,
                self.destination.clone(),
            ));
        }

        // ---------------------------------------------------------------------
        // State consistency.
        // ---------------------------------------------------------------------

        if context.state_inconsistent && self.escalate_on_state_inconsistency {
            return Ok(EscalationDecision::escalate(
                EscalationTrigger::StateInconsistency,
                self.destination.clone(),
            ));
        }

        // ---------------------------------------------------------------------
        // No safe action.
        // ---------------------------------------------------------------------

        if context.no_safe_action {
            return Ok(EscalationDecision::escalate(
                EscalationTrigger::NoSafeAction,
                self.destination.clone(),
            ));
        }

        // ---------------------------------------------------------------------
        // Mode-specific behavior.
        // ---------------------------------------------------------------------

        match self.mode {
            EscalationMode::Disabled | EscalationMode::Conditional => {
                Ok(EscalationDecision::continue_autonomously())
            }

            EscalationMode::FailClosed => {
                // FailClosed has already escalated all known safety-critical
                // conditions above. Ordinary conditions may continue through
                // normal policy/planning evaluation.
                Ok(EscalationDecision::continue_autonomously())
            }

            EscalationMode::OperatorRequired => Ok(EscalationDecision::escalate(
                EscalationTrigger::CallerRequired,
                self.destination.clone(),
            )),
        }
    }

    /// Returns whether this policy would escalate the supplied context.
    pub fn would_escalate(
        &self,
        context: &EscalationContext,
    ) -> Result<bool, EscalationError> {
        Ok(self.evaluate(context)?.requires_escalation())
    }
}

// =============================================================================
// Helper validation
// =============================================================================

fn validate_probability(field: &'static str, value: f64) -> Result<(), EscalationError> {
    if !value.is_finite() {
        return Err(EscalationError::NonFiniteValue { field, value });
    }

    if !(0.0..=1.0).contains(&value) {
        return Err(EscalationError::ProbabilityOutOfRange { field, value });
    }

    Ok(())
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_policy_is_fail_closed() {
        let policy = EscalationPolicy::default();

        assert_eq!(policy.mode, EscalationMode::FailClosed);
        assert!(policy.validate().is_ok());
    }

    #[test]
    fn clean_context_does_not_escalate() {
        let policy = EscalationPolicy::default();
        let context = EscalationContext::default();

        let decision = policy
            .evaluate(&context)
            .expect("default context must be valid");

        assert!(!decision.requires_escalation());
        assert!(decision.permits_autonomous_continuation());
        assert_eq!(decision.trigger, EscalationTrigger::None);
    }

    #[test]
    fn safety_violation_always_escalates() {
        let policy = EscalationPolicy {
            mode: EscalationMode::Disabled,
            ..EscalationPolicy::default()
        };

        let context = EscalationContext {
            safety_invariant_violated: true,
            ..EscalationContext::default()
        };

        let decision = policy
            .evaluate(&context)
            .expect("safety context must be valid");

        assert!(decision.requires_escalation());
        assert_eq!(
            decision.trigger,
            EscalationTrigger::SafetyInvariantViolation
        );
        assert!(decision.safety_critical);
        assert!(!decision.permits_autonomous_continuation());
    }

    #[test]
    fn semantic_safety_cannot_be_overridden_by_disabled_mode() {
        let policy = EscalationPolicy {
            mode: EscalationMode::Disabled,
            ..EscalationPolicy::default()
        };

        let context = EscalationContext {
            semantic_safety_violation: true,
            ..EscalationContext::default()
        };

        let decision = policy
            .evaluate(&context)
            .expect("context must be valid");

        assert!(decision.requires_escalation());
        assert_eq!(decision.trigger, EscalationTrigger::SemanticSafety);
    }

    #[test]
    fn insufficient_confidence_escalates_when_configured() {
        let policy = EscalationPolicy {
            thresholds: EscalationThresholds {
                minimum_confidence: Some(0.8),
                ..EscalationThresholds::default()
            },
            ..EscalationPolicy::default()
        };

        let context = EscalationContext {
            diagnosis_confidence: Some(0.5),
            ..EscalationContext::default()
        };

        let decision = policy
            .evaluate(&context)
            .expect("context must be valid");

        assert!(decision.requires_escalation());
        assert_eq!(
            decision.trigger,
            EscalationTrigger::InsufficientConfidence
        );
    }

    #[test]
    fn missing_confidence_fails_closed_when_required() {
        let policy = EscalationPolicy {
            thresholds: EscalationThresholds {
                minimum_confidence: Some(0.5),
                ..EscalationThresholds::default()
            },
            ..EscalationPolicy::default()
        };

        let context = EscalationContext::default();

        let decision = policy
            .evaluate(&context)
            .expect("context must be valid");

        assert!(decision.requires_escalation());
        assert_eq!(
            decision.trigger,
            EscalationTrigger::InsufficientConfidence
        );
    }

    #[test]
    fn confidence_at_threshold_is_allowed() {
        let policy = EscalationPolicy {
            thresholds: EscalationThresholds {
                minimum_confidence: Some(0.8),
                ..EscalationThresholds::default()
            },
            ..EscalationPolicy::default()
        };

        let context = EscalationContext {
            diagnosis_confidence: Some(0.8),
            ..EscalationContext::default()
        };

        let decision = policy
            .evaluate(&context)
            .expect("context must be valid");

        assert!(!decision.requires_escalation());
    }

    #[test]
    fn recovery_attempt_limit_is_configurable() {
        let policy = EscalationPolicy {
            thresholds: EscalationThresholds {
                maximum_recovery_attempts: Some(5),
                ..EscalationThresholds::default()
            },
            ..EscalationPolicy::default()
        };

        let context = EscalationContext {
            recovery_attempts: 5,
            ..EscalationContext::default()
        };

        let decision = policy
            .evaluate(&context)
            .expect("context must be valid");

        assert!(decision.requires_escalation());
        assert_eq!(
            decision.trigger,
            EscalationTrigger::RecoveryAttemptLimit
        );
    }

    #[test]
    fn no_finite_attempt_limit_means_no_artificial_limit() {
        let policy = EscalationPolicy::default();

        let context = EscalationContext {
            recovery_attempts: u64::MAX,
            ..EscalationContext::default()
        };

        let decision = policy
            .evaluate(&context)
            .expect("context must be valid");

        assert!(!decision.requires_escalation());
    }

    #[test]
    fn conflicting_evidence_escalates() {
        let policy = EscalationPolicy::default();

        let context = EscalationContext {
            evidence_conflicting: true,
            ..EscalationContext::default()
        };

        let decision = policy
            .evaluate(&context)
            .expect("context must be valid");

        assert!(decision.requires_escalation());
        assert_eq!(
            decision.trigger,
            EscalationTrigger::ConflictingEvidence
        );
    }

    #[test]
    fn untrusted_evidence_escalates() {
        let policy = EscalationPolicy::default();

        let context = EscalationContext {
            evidence_untrusted: true,
            ..EscalationContext::default()
        };

        let decision = policy
            .evaluate(&context)
            .expect("context must be valid");

        assert!(decision.requires_escalation());
        assert_eq!(
            decision.trigger,
            EscalationTrigger::UntrustedEvidence
        );
    }

    #[test]
    fn verification_failure_escalates() {
        let policy = EscalationPolicy::default();

        let context = EscalationContext {
            verification_failed: true,
            ..EscalationContext::default()
        };

        let decision = policy
            .evaluate(&context)
            .expect("context must be valid");

        assert!(decision.requires_escalation());
        assert_eq!(
            decision.trigger,
            EscalationTrigger::VerificationFailure
        );
    }

    #[test]
    fn inconclusive_verification_escalates_by_default() {
        let policy = EscalationPolicy::default();

        let context = EscalationContext {
            verification_inconclusive: true,
            ..EscalationContext::default()
        };

        let decision = policy
            .evaluate(&context)
            .expect("context must be valid");

        assert!(decision.requires_escalation());
        assert_eq!(
            decision.trigger,
            EscalationTrigger::VerificationInconclusive
        );
    }

    #[test]
    fn resource_change_can_be_policy_controlled() {
        let policy = EscalationPolicy {
            escalate_on_resource_change: true,
            ..EscalationPolicy::default()
        };

        let context = EscalationContext {
            resource_state_changed: true,
            ..EscalationContext::default()
        };

        let decision = policy
            .evaluate(&context)
            .expect("context must be valid");

        assert!(decision.requires_escalation());
        assert_eq!(decision.trigger, EscalationTrigger::ResourceChanged);
    }

    #[test]
    fn resource_change_does_not_have_to_escalate() {
        let policy = EscalationPolicy::default();

        let context = EscalationContext {
            resource_state_changed: true,
            ..EscalationContext::default()
        };

        let decision = policy
            .evaluate(&context)
            .expect("context must be valid");

        assert!(!decision.requires_escalation());
    }

    #[test]
    fn operator_required_always_escalates() {
        let policy = EscalationPolicy {
            mode: EscalationMode::OperatorRequired,
            ..EscalationPolicy::default()
        };

        let context = EscalationContext::default();

        let decision = policy
            .evaluate(&context)
            .expect("context must be valid");

        assert!(decision.requires_escalation());
        assert_eq!(decision.trigger, EscalationTrigger::CallerRequired);
    }

    #[test]
    fn disabled_mode_can_disable_ordinary_escalation() {
        let policy = EscalationPolicy {
            mode: EscalationMode::Disabled,
            thresholds: EscalationThresholds {
                minimum_confidence: None,
                ..EscalationThresholds::default()
            },
            escalate_on_conflicting_evidence: false,
            ..EscalationPolicy::default()
        };

        let context = EscalationContext {
            evidence_conflicting: true,
            ..EscalationContext::default()
        };

        let decision = policy
            .evaluate(&context)
            .expect("context must be valid");

        assert!(!decision.requires_escalation());
    }

    #[test]
    fn disabled_mode_does_not_disable_safety() {
        let policy = EscalationPolicy {
            mode: EscalationMode::Disabled,
            ..EscalationPolicy::default()
        };

        let context = EscalationContext {
            verification_failed: true,
            ..EscalationContext::default()
        };

        let decision = policy
            .evaluate(&context)
            .expect("context must be valid");

        assert!(decision.requires_escalation());
    }

    #[test]
    fn invalid_confidence_is_rejected() {
        let policy = EscalationPolicy {
            thresholds: EscalationThresholds {
                minimum_confidence: Some(1.5),
                ..EscalationThresholds::default()
            },
            ..EscalationPolicy::default()
        };

        assert!(policy.validate().is_err());
    }

    #[test]
    fn nan_confidence_is_rejected() {
        let policy = EscalationPolicy {
            thresholds: EscalationThresholds {
                minimum_confidence: Some(f64::NAN),
                ..EscalationThresholds::default()
            },
            ..EscalationPolicy::default()
        };

        assert!(policy.validate().is_err());
    }

    #[test]
    fn infinite_cost_is_rejected() {
        let policy = EscalationPolicy {
            thresholds: EscalationThresholds {
                maximum_recovery_cost: Some(f64::INFINITY),
                ..EscalationThresholds::default()
            },
            ..EscalationPolicy::default()
        };

        assert!(policy.validate().is_err());
    }

    #[test]
    fn negative_cost_is_rejected() {
        let policy = EscalationPolicy {
            thresholds: EscalationThresholds {
                maximum_recovery_cost: Some(-1.0),
                ..EscalationThresholds::default()
            },
            ..EscalationPolicy::default()
        };

        assert!(policy.validate().is_err());
    }

    #[test]
    fn non_finite_context_cost_is_rejected() {
        let policy = EscalationPolicy::default();

        let context = EscalationContext {
            accumulated_recovery_cost: f64::NAN,
            ..EscalationContext::default()
        };

        assert!(policy.evaluate(&context).is_err());
    }

    #[test]
    fn custom_destination_is_validated() {
        assert!(EscalationDestination::custom("operator").is_ok());
        assert!(EscalationDestination::custom("").is_err());
        assert!(EscalationDestination::custom("\noperator").is_err());
    }

    #[test]
    fn decision_is_deterministic() {
        let policy = EscalationPolicy {
            thresholds: EscalationThresholds {
                minimum_confidence: Some(0.75),
                maximum_recovery_attempts: Some(10),
                maximum_consecutive_failures: Some(4),
                ..EscalationThresholds::default()
            },
            ..EscalationPolicy::default()
        };

        let context = EscalationContext {
            diagnosis_confidence: Some(0.5),
            recovery_attempts: 3,
            consecutive_failures: 2,
            ..EscalationContext::default()
        };

        let first = policy
            .evaluate(&context)
            .expect("context must be valid");

        let second = policy
            .evaluate(&context)
            .expect("context must be valid");

        assert_eq!(first, second);
    }
}