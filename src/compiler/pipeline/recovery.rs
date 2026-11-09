
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Universal Meta-Compiler — Pipeline Recovery.
//!
//! Path: src/compiler/pipeline/recovery.rs
//! Edition: 2021
//! Minimum Rust version: 1.97
//!
//! # Purpose
//!
//! Evaluates whether a failed compiler-pipeline operation may be:
//!
//! - retried;
//! - replaced by an explicitly verified fallback;
//! - or stopped.
//!
//! This module produces decisions only. It never executes stages,
//! invokes backends, changes compilation lifecycle state, allocates
//! resources or modifies program semantics.
//!
//! # Canonical authorities
//!
//! - pipeline::failure: PipelineFailure and failure classification.
//! - pipeline::stage: StageId and stage execution contracts.
//! - pipeline::execution: actual stage execution.
//! - pipeline::orchestration: lifecycle and recovery coordination.
//! - compiler::context: cancellation and compilation context.
//! - compiler::configuration::budget: operational budgets.
//! - compiler::planning: feasibility and realization evidence.
//! - compiler::backends: backend compatibility.
//!
//! # Recovery invariants
//!
//! 1. Cancellation is never automatically retried.
//! 2. Internal correctness failures are never automatically retried.
//! 3. Retryability declared by a stage is not retry authorization.
//! 4. Replay safety must be explicitly established.
//! 5. Retry policy must authorize the failure category.
//! 6. Resource and external failures require additional authorization.
//! 7. A retry must identify the failed stage.
//! 8. Retry accounting must not overflow.
//! 9. Fallback must preserve semantic meaning, requirements and constraints.
//! 10. Fallback must be verified and compatible with its realization.
//! 11. Every fallback decision must retain verification provenance.
//! 12. No recovery decision may claim compilation succeeded.
//!
//! # Scalability
//!
//! This module imposes no universal hardware or program-size limits.
//!
//! Retry counts are configurable operational policy. They are not
//! language-level or hardware-capacity restrictions.
//!
//! No finite implementation can promise infinite physical resources.
//!
//! # Grammar boundary
//!
//! Grammar and semantic authorities define source-level meaning.
//! Recovery cannot weaken that meaning to make a compilation succeed.
//!
//! Relevant specifications:
//!
//! - grammar/specification/compilation-model.md
//! - grammar/specification/execution-model.md
//! - grammar/specification/poco-reaf.md
//! - grammar/validation/semantic-boundaries.md

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;
use std::num::NonZeroUsize;
use std::sync::Arc;

use crate::compiler::pipeline::failure::{
    FailureCategory,
    FailureDisposition,
    FailureSeverity,
    PipelineFailure,
};

use crate::compiler::pipeline::stage::StageId;

// ============================================================================
// API VERSION
// ============================================================================

/// Public API version of pipeline recovery.
///
/// Independent of language, grammar, AST, semantic-model, IR,
/// backend ABI and runtime versions.
pub const PIPELINE_RECOVERY_API_VERSION: u16 = 1;

// ============================================================================
// RESULT
// ============================================================================

/// Result type for recovery-policy operations.
pub type RecoveryResult<T> = Result<T, RecoveryPolicyError>;

// ============================================================================
// RETRY SAFETY
// ============================================================================

/// Evidence supplied by the execution coordinator concerning replay safety.
///
/// A stage's retryability declaration is insufficient on its own.
/// A stage may have performed externally observable work before failing.
///
/// The orchestrator must establish replay safety using the stage contract,
/// transaction state, side-effect guarantees and execution provenance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum RetrySafety {
    /// Replay safety has not been established.
    ///
    /// This is the conservative default.
    NotEstablished,

    /// The execution coordinator has established replay safety.
    ReplaySafe,
}

impl RetrySafety {
    /// Returns whether replay safety has been established.
    #[must_use]
    pub const fn is_safe(self) -> bool {
        matches!(self, Self::ReplaySafe)
    }
}

// ============================================================================
// FALLBACK EVIDENCE
// ============================================================================

/// Evidence required before an alternative implementation may replace
/// a failed realization.
///
/// The caller must obtain this evidence from the authoritative semantic,
/// verification, feasibility and backend-compatibility systems.
///
/// This structure records evidence; it does not independently verify
/// the truth of the supplied assertions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FallbackEvidence {
    semantic_equivalence_verified: bool,
    requirements_preserved: bool,
    constraints_preserved: bool,
    fallback_verified: bool,
    realization_compatible: bool,
    provenance: Arc<str>,
}

impl FallbackEvidence {
    /// Creates fallback verification evidence.
    ///
    /// All conditions are explicit. A fallback is not considered safe
    /// merely because it can produce an output.
    ///
    /// The trusted orchestration layer must construct this value only
    /// after the corresponding authoritative checks have succeeded.
    #[must_use]
    pub fn new(
        semantic_equivalence_verified: bool,
        requirements_preserved: bool,
        constraints_preserved: bool,
        fallback_verified: bool,
        realization_compatible: bool,
        provenance: impl Into<Arc<str>>,
    ) -> Self {
        Self {
            semantic_equivalence_verified,
            requirements_preserved,
            constraints_preserved,
            fallback_verified,
            realization_compatible,
            provenance: provenance.into(),
        }
    }

    /// Returns whether semantic equivalence was verified.
    #[must_use]
    pub const fn semantic_equivalence_verified(&self) -> bool {
        self.semantic_equivalence_verified
    }

    /// Returns whether program requirements are preserved.
    #[must_use]
    pub const fn requirements_preserved(&self) -> bool {
        self.requirements_preserved
    }

    /// Returns whether mandatory constraints are preserved.
    #[must_use]
    pub const fn constraints_preserved(&self) -> bool {
        self.constraints_preserved
    }

    /// Returns whether the fallback implementation was verified.
    #[must_use]
    pub const fn fallback_verified(&self) -> bool {
        self.fallback_verified
    }

    /// Returns whether the fallback is compatible with its
    /// intended realization contract.
    #[must_use]
    pub const fn realization_compatible(&self) -> bool {
        self.realization_compatible
    }

    /// Returns verification provenance.
    #[must_use]
    pub fn provenance(&self) -> &str {
        self.provenance.as_ref()
    }

    /// Returns whether every mandatory fallback condition holds.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.semantic_equivalence_verified
            && self.requirements_preserved
            && self.constraints_preserved
            && self.fallback_verified
            && self.realization_compatible
            && !self.provenance.trim().is_empty()
    }
}

// ============================================================================
// RECOVERY POLICY
// ============================================================================

/// Immutable policy controlling permitted recovery decisions.
///
/// The default policy disables automatic retries and fallbacks.
///
/// Retry counts are additional executions after the original failed
/// execution. A configured count of zero disables retries.
///
/// Retry limits are operational safeguards, not hardware or language
/// capacity limits.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryPolicy {
    max_retries: usize,
    retryable_categories: BTreeSet<FailureCategory>,
    allow_fallback: bool,
    allow_retry_after_resource_failure: bool,
    allow_retry_after_external_failure: bool,
}

impl Default for RecoveryPolicy {
    fn default() -> Self {
        Self {
            max_retries: 0,
            retryable_categories: BTreeSet::new(),
            allow_fallback: false,
            allow_retry_after_resource_failure: false,
            allow_retry_after_external_failure: false,
        }
    }
}

impl RecoveryPolicy {
    /// Creates the conservative recovery policy.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the maximum additional retries for one failed stage.
    ///
    /// Zero disables retries.
    #[must_use]
    pub fn with_max_retries(mut self, retries: usize) -> Self {
        self.max_retries = retries;
        self
    }

    /// Sets the retry limit using a non-zero value.
    #[must_use]
    pub fn with_nonzero_max_retries(
        mut self,
        retries: NonZeroUsize,
    ) -> Self {
        self.max_retries = retries.get();
        self
    }

    /// Authorizes retry for a failure category.
    ///
    /// The stage must still declare retryability, replay safety must
    /// be established, and the retry budget must remain available.
    #[must_use]
    pub fn allow_retry_category(
        mut self,
        category: FailureCategory,
    ) -> Self {
        self.retryable_categories.insert(category);
        self
    }

    /// Enables or disables retry after a resource failure.
    ///
    /// This does not establish that a resource has become available.
    /// The orchestrator must verify that retrying is meaningful.
    #[must_use]
    pub fn allow_resource_retry(mut self, allow: bool) -> Self {
        self.allow_retry_after_resource_failure = allow;
        self
    }

    /// Enables or disables retry after an external subsystem failure.
    ///
    /// Replay safety and category authorization remain mandatory.
    #[must_use]
    pub fn allow_external_retry(mut self, allow: bool) -> Self {
        self.allow_retry_after_external_failure = allow;
        self
    }

    /// Enables or disables fallback selection.
    ///
    /// Enabling fallback does not authorize an unverified fallback.
    #[must_use]
    pub fn allow_fallback(mut self, allow: bool) -> Self {
        self.allow_fallback = allow;
        self
    }

    /// Returns the configured retry limit.
    #[must_use]
    pub const fn max_retries(&self) -> usize {
        self.max_retries
    }

    /// Returns whether fallback selection is enabled.
    #[must_use]
    pub const fn fallback_allowed(&self) -> bool {
        self.allow_fallback
    }

    /// Returns whether a failure category is authorized for retry.
    #[must_use]
    pub fn retry_category_allowed(
        &self,
        category: FailureCategory,
    ) -> bool {
        self.retryable_categories.contains(&category)
    }

    /// Returns the configured retry categories.
    #[must_use]
    pub fn retryable_categories(&self) -> &BTreeSet<FailureCategory> {
        &self.retryable_categories
    }

    /// Returns whether resource-failure retry is explicitly enabled.
    #[must_use]
    pub const fn resource_retry_allowed(&self) -> bool {
        self.allow_retry_after_resource_failure
    }

    /// Returns whether external-failure retry is explicitly enabled.
    #[must_use]
    pub const fn external_retry_allowed(&self) -> bool {
        self.allow_retry_after_external_failure
    }
}

// ============================================================================
// RECOVERY REQUEST
// ============================================================================

/// Immutable information required to evaluate a recovery decision.
///
/// `retries_performed` counts additional executions already attempted
/// after the original execution.
///
/// Examples:
///
/// - Original execution failed: zero retries performed.
/// - First retry failed: one retry performed.
/// - Second retry failed: two retries performed.
#[derive(Debug)]
pub struct RecoveryRequest<'a> {
    failure: &'a PipelineFailure,
    retries_performed: usize,
    retry_safety: RetrySafety,
    fallback_evidence: Option<&'a FallbackEvidence>,
}

impl<'a> RecoveryRequest<'a> {
    /// Creates a recovery request.
    #[must_use]
    pub fn new(
        failure: &'a PipelineFailure,
        retries_performed: usize,
    ) -> Self {
        Self {
            failure,
            retries_performed,
            retry_safety: RetrySafety::NotEstablished,
            fallback_evidence: None,
        }
    }

    /// Supplies explicit replay-safety evidence.
    #[must_use]
    pub fn with_retry_safety(mut self, safety: RetrySafety) -> Self {
        self.retry_safety = safety;
        self
    }

    /// Supplies verified fallback evidence.
    #[must_use]
    pub fn with_fallback_evidence(
        mut self,
        evidence: &'a FallbackEvidence,
    ) -> Self {
        self.fallback_evidence = Some(evidence);
        self
    }

    /// Returns the original pipeline failure.
    #[must_use]
    pub fn failure(&self) -> &PipelineFailure {
        self.failure
    }

    /// Returns the number of retries already performed.
    #[must_use]
    pub const fn retries_performed(&self) -> usize {
        self.retries_performed
    }

    /// Returns the replay-safety declaration.
    #[must_use]
    pub const fn retry_safety(&self) -> RetrySafety {
        self.retry_safety
    }

    /// Returns fallback evidence, if supplied.
    #[must_use]
    pub fn fallback_evidence(&self) -> Option<&FallbackEvidence> {
        self.fallback_evidence
    }
}

// ============================================================================
// RECOVERY REASONS
// ============================================================================

/// Machine-readable explanation for a recovery decision.
///
/// The string identifiers are stable diagnostic identifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum RecoveryReason {
    RetryAuthorized,
    RetryNotDeclared,
    RetryNotSafe,
    RetryBudgetExhausted,
    RetryCategoryNotAuthorized,
    RetryStageMissing,
    RetryAttemptOverflow,
    ResourceRetryNotAuthorized,
    ExternalRetryNotAuthorized,
    CancellationCannotBeRetried,
    InternalInvariantCannotBeRetried,
    FallbackAuthorized,
    FallbackDisabled,
    FallbackEvidenceMissing,
    FallbackNotEquivalent,
    RequirementsNotPreserved,
    ConstraintsNotPreserved,
    FallbackNotVerified,
    RealizationIncompatible,
    ProvenanceMissing,
    TerminalFailure,
    IncompleteRequiredWork,
    CriticalFailure,
    NonRecoverableFailure,
}

impl RecoveryReason {
    /// Returns a stable machine-readable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RetryAuthorized => "retry_authorized",
            Self::RetryNotDeclared => "retry_not_declared",
            Self::RetryNotSafe => "retry_safety_not_established",
            Self::RetryBudgetExhausted => "retry_budget_exhausted",
            Self::RetryCategoryNotAuthorized => {
                "retry_category_not_authorized"
            }
            Self::RetryStageMissing => "retry_stage_missing",
            Self::RetryAttemptOverflow => "retry_attempt_overflow",
            Self::ResourceRetryNotAuthorized => {
                "resource_retry_not_authorized"
            }
            Self::ExternalRetryNotAuthorized => {
                "external_retry_not_authorized"
            }
            Self::CancellationCannotBeRetried => {
                "cancellation_cannot_be_retried"
            }
            Self::InternalInvariantCannotBeRetried => {
                "internal_invariant_cannot_be_retried"
            }
            Self::FallbackAuthorized => "fallback_authorized",
            Self::FallbackDisabled => "fallback_disabled",
            Self::FallbackEvidenceMissing => {
                "fallback_evidence_missing"
            }
            Self::FallbackNotEquivalent => {
                "fallback_not_semantically_equivalent"
            }
            Self::RequirementsNotPreserved => {
                "requirements_not_preserved"
            }
            Self::ConstraintsNotPreserved => {
                "constraints_not_preserved"
            }
            Self::FallbackNotVerified => "fallback_not_verified",
            Self::RealizationIncompatible => {
                "fallback_realization_incompatible"
            }
            Self::ProvenanceMissing => "fallback_provenance_missing",
            Self::TerminalFailure => "terminal_failure",
            Self::IncompleteRequiredWork => {
                "incomplete_required_work"
            }
            Self::CriticalFailure => "critical_failure",
            Self::NonRecoverableFailure => "non_recoverable_failure",
        }
    }
}

impl fmt::Display for RecoveryReason {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// RECOVERY ACTION
// ============================================================================

/// A recovery action recommended by the planner.
///
/// These variants describe decisions. They do not execute operations.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum RecoveryAction {
    /// Retry the identified failed stage.
    ///
    /// `next_attempt` is one-based and counts retries after the
    /// original execution.
    Retry {
        stage: StageId,
        next_attempt: usize,
    },

    /// Use an explicitly verified alternative realization.
    ///
    /// The appropriate planning/backend orchestration subsystem
    /// selects and invokes the actual fallback.
    UseFallback {
        stage: Option<StageId>,
        provenance: Arc<str>,
    },

    /// Stop compilation without claiming success.
    Stop,
}

// ============================================================================
// RECOVERY DECISION
// ============================================================================

/// Immutable result of recovery evaluation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryDecision {
    action: RecoveryAction,
    reason: RecoveryReason,
    explanation: Arc<str>,
}

impl RecoveryDecision {
    /// Creates a recovery decision.
    #[must_use]
    pub fn new(
        action: RecoveryAction,
        reason: RecoveryReason,
        explanation: impl Into<Arc<str>>,
    ) -> Self {
        Self {
            action,
            reason,
            explanation: explanation.into(),
        }
    }

    /// Returns the selected action.
    #[must_use]
    pub fn action(&self) -> &RecoveryAction {
        &self.action
    }

    /// Returns the machine-readable reason.
    #[must_use]
    pub const fn reason(&self) -> RecoveryReason {
        self.reason
    }

    /// Returns the human-readable explanation.
    #[must_use]
    pub fn explanation(&self) -> &str {
        self.explanation.as_ref()
    }

    /// Returns whether retry was selected.
    #[must_use]
    pub fn should_retry(&self) -> bool {
        matches!(self.action, RecoveryAction::Retry { .. })
    }

    /// Returns whether fallback was selected.
    #[must_use]
    pub fn should_use_fallback(&self) -> bool {
        matches!(self.action, RecoveryAction::UseFallback { .. })
    }

    /// Returns whether compilation must stop.
    #[must_use]
    pub fn must_stop(&self) -> bool {
        matches!(self.action, RecoveryAction::Stop)
    }
}

// ============================================================================
// POLICY ERROR
// ============================================================================

/// Errors associated with recovery-policy construction or validation.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum RecoveryPolicyError {
    /// The recovery policy is internally inconsistent.
    InvalidPolicy {
        message: Arc<str>,
    },
}

impl fmt::Display for RecoveryPolicyError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidPolicy { message } => {
                write!(formatter, "invalid recovery policy: {message}")
            }
        }
    }
}

impl Error for RecoveryPolicyError {}

// ============================================================================
// RECOVERY PLANNER
// ============================================================================

/// Stateless recovery decision engine.
///
/// The planner evaluates a PipelineFailure against explicit policy
/// and safety evidence.
///
/// It does not mutate the pipeline, execute stages, change lifecycle
/// state, allocate resources or invoke fallback implementations.
#[derive(Debug, Default, Clone, Copy)]
pub struct RecoveryPlanner;

impl RecoveryPlanner {
    /// Creates a recovery planner.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Evaluates a recovery request.
    ///
    /// Decision precedence:
    ///
    /// 1. Cancellation and internal correctness failures stop immediately.
    /// 2. Critical failures stop immediately.
    /// 3. Authorized, replay-safe retries are preferred.
    /// 4. A verified fallback may be selected if retry is unavailable.
    /// 5. Otherwise compilation stops.
    ///
    /// The planner never treats a retry declaration as sufficient
    /// authorization.
    #[must_use]
    pub fn plan(
        &self,
        request: &RecoveryRequest<'_>,
        policy: &RecoveryPolicy,
    ) -> RecoveryDecision {
        let failure = request.failure();

        if Self::is_cancellation(failure) {
            return Self::stop(
                RecoveryReason::CancellationCannotBeRetried,
                "Compilation was cancelled. Recovery cannot override \
                 the cancellation request.",
            );
        }

        if Self::is_internal_failure(failure) {
            return Self::stop(
                RecoveryReason::InternalInvariantCannotBeRetried,
                "An internal correctness invariant failed. The pipeline \
                 must stop rather than replay potentially corrupted work.",
            );
        }

        if failure.severity() == FailureSeverity::Critical {
            return Self::stop(
                RecoveryReason::CriticalFailure,
                "A critical pipeline failure requires explicit \
                 investigation and cannot be automatically recovered.",
            );
        }

        if let Some(decision) = Self::retry_decision(request, policy) {
            return decision;
        }

        if let Some(decision) = Self::fallback_decision(request, policy) {
            return decision;
        }

        Self::stop(
            Self::final_stop_reason(request, policy),
            "No recovery action satisfied the configured policy and \
             required safety conditions. Compilation remains unsuccessful.",
        )
    }

    fn is_cancellation(failure: &PipelineFailure) -> bool {
        failure.is_cancelled()
            || failure.disposition() == FailureDisposition::Cancelled
            || failure.category() == FailureCategory::Cancellation
    }

    fn is_internal_failure(failure: &PipelineFailure) -> bool {
        failure.disposition() == FailureDisposition::InternalFailure
            || failure.category() == FailureCategory::Internal
    }

    fn retry_decision(
        request: &RecoveryRequest<'_>,
        policy: &RecoveryPolicy,
    ) -> Option<RecoveryDecision> {
        let failure = request.failure();

        if !failure.is_declared_retryable() {
            return None;
        }

        if !request.retry_safety().is_safe() {
            return None;
        }

        if failure.stage().is_none() {
            return None;
        }

        if request.retries_performed() >= policy.max_retries() {
            return None;
        }

        let category = failure.category();

        if !policy.retry_category_allowed(category) {
            return None;
        }

        if category == FailureCategory::Resource
            && !policy.resource_retry_allowed()
        {
            return None;
        }

        if category == FailureCategory::External
            && !policy.external_retry_allowed()
        {
            return None;
        }

        let next_attempt = request
            .retries_performed()
            .checked_add(1)?;

        let stage = failure.stage()?.clone();

        Some(RecoveryDecision::new(
            RecoveryAction::Retry {
                stage,
                next_attempt,
            },
            RecoveryReason::RetryAuthorized,
            "The stage declared retryability, replay safety was \
             established, the failure category is authorized, \
             and retry budget remains.",
        ))
    }

    fn fallback_decision(
        request: &RecoveryRequest<'_>,
        policy: &RecoveryPolicy,
    ) -> Option<RecoveryDecision> {
        if !policy.fallback_allowed() {
            return None;
        }

        let evidence = request.fallback_evidence()?;

        if !evidence.is_valid() {
            return None;
        }

        Some(RecoveryDecision::new(
            RecoveryAction::UseFallback {
                stage: request.failure().stage().cloned(),
                provenance: Arc::from(evidence.provenance()),
            },
            RecoveryReason::FallbackAuthorized,
            "The fallback has verified semantic equivalence, preserves \
             required capabilities and constraints, has passed \
             verification, and satisfies the realization contract.",
        ))
    }

    fn final_stop_reason(
        request: &RecoveryRequest<'_>,
        policy: &RecoveryPolicy,
    ) -> RecoveryReason {
        let failure = request.failure();

        if Self::is_cancellation(failure) {
            return RecoveryReason::CancellationCannotBeRetried;
        }

        if Self::is_internal_failure(failure) {
            return RecoveryReason::InternalInvariantCannotBeRetried;
        }

        if failure.severity() == FailureSeverity::Critical {
            return RecoveryReason::CriticalFailure;
        }

        if failure.disposition() == FailureDisposition::Incomplete {
            return RecoveryReason::IncompleteRequiredWork;
        }

        if failure.prevents_success()
            && !failure.is_declared_retryable()
            && request.fallback_evidence().is_none()
        {
            return RecoveryReason::TerminalFailure;
        }

        if failure.is_declared_retryable() {
            if request.retry_safety() != RetrySafety::ReplaySafe {
                return RecoveryReason::RetryNotSafe;
            }

            if failure.stage().is_none() {
                return RecoveryReason::RetryStageMissing;
            }

            if request.retries_performed() >= policy.max_retries() {
                return RecoveryReason::RetryBudgetExhausted;
            }

            let category = failure.category();

            if !policy.retry_category_allowed(category) {
                return RecoveryReason::RetryCategoryNotAuthorized;
            }

            if category == FailureCategory::Resource
                && !policy.resource_retry_allowed()
            {
                return RecoveryReason::ResourceRetryNotAuthorized;
            }

            if category == FailureCategory::External
                && !policy.external_retry_allowed()
            {
                return RecoveryReason::ExternalRetryNotAuthorized;
            }

            if request.retries_performed().checked_add(1).is_none() {
                return RecoveryReason::RetryAttemptOverflow;
            }
        } else if request.fallback_evidence().is_none()
            && policy.fallback_allowed()
        {
            return RecoveryReason::RetryNotDeclared;
        }

        if !policy.fallback_allowed()
            && request.fallback_evidence().is_some()
        {
            return RecoveryReason::FallbackDisabled;
        }

        if policy.fallback_allowed()
            && request.fallback_evidence().is_none()
        {
            return RecoveryReason::FallbackEvidenceMissing;
        }

        if let Some(evidence) = request.fallback_evidence() {
            if !evidence.semantic_equivalence_verified() {
                return RecoveryReason::FallbackNotEquivalent;
            }

            if !evidence.requirements_preserved() {
                return RecoveryReason::RequirementsNotPreserved;
            }

            if !evidence.constraints_preserved() {
                return RecoveryReason::ConstraintsNotPreserved;
            }

            if !evidence.fallback_verified() {
                return RecoveryReason::FallbackNotVerified;
            }

            if !evidence.realization_compatible() {
                return RecoveryReason::RealizationIncompatible;
            }

            if evidence.provenance().trim().is_empty() {
                return RecoveryReason::ProvenanceMissing;
            }
        }

        if failure.prevents_success() {
            return RecoveryReason::TerminalFailure;
        }

        RecoveryReason::NonRecoverableFailure
    }

    fn stop(
        reason: RecoveryReason,
        explanation: &'static str,
    ) -> RecoveryDecision {
        RecoveryDecision::new(
            RecoveryAction::Stop,
            reason,
            explanation,
        )
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_policy_disables_automatic_recovery() {
        let policy = RecoveryPolicy::default();

        assert_eq!(policy.max_retries(), 0);
        assert!(!policy.fallback_allowed());
        assert!(policy.retryable_categories().is_empty());
        assert!(!policy.resource_retry_allowed());
        assert!(!policy.external_retry_allowed());
    }

    #[test]
    fn retry_limit_is_configurable() {
        let policy = RecoveryPolicy::new().with_max_retries(7);

        assert_eq!(policy.max_retries(), 7);
    }

    #[test]
    fn nonzero_retry_limit_is_supported() {
        let retries = NonZeroUsize::new(3)
            .expect("three is non-zero");

        let policy = RecoveryPolicy::new()
            .with_nonzero_max_retries(retries);

        assert_eq!(policy.max_retries(), 3);
    }

    #[test]
    fn maximum_retry_limit_is_not_a_hardware_limit() {
        let policy = RecoveryPolicy::new()
            .with_max_retries(usize::MAX);

        assert_eq!(policy.max_retries(), usize::MAX);
    }

    #[test]
    fn retry_categories_are_extensible() {
        let policy = RecoveryPolicy::new()
            .allow_retry_category(FailureCategory::StageExecution)
            .allow_retry_category(FailureCategory::Resource);

        assert!(
            policy.retry_category_allowed(
                FailureCategory::StageExecution
            )
        );

        assert!(
            policy.retry_category_allowed(
                FailureCategory::Resource
            )
        );

        assert!(
            !policy.retry_category_allowed(
                FailureCategory::Cancellation
            )
        );
    }

    #[test]
    fn resource_retry_requires_explicit_authorization() {
        let policy = RecoveryPolicy::new()
            .allow_retry_category(FailureCategory::Resource);

        assert!(!policy.resource_retry_allowed());

        let authorized = policy.allow_resource_retry(true);

        assert!(authorized.resource_retry_allowed());
    }

    #[test]
    fn external_retry_requires_explicit_authorization() {
        let policy = RecoveryPolicy::new()
            .allow_retry_category(FailureCategory::External);

        assert!(!policy.external_retry_allowed());

        let authorized = policy.allow_external_retry(true);

        assert!(authorized.external_retry_allowed());
    }

    #[test]
    fn fallback_is_disabled_by_default() {
        assert!(!RecoveryPolicy::default().fallback_allowed());
    }

    #[test]
    fn fallback_requires_all_safety_conditions() {
        let evidence = FallbackEvidence::new(
            true,
            true,
            true,
            true,
            true,
            "verified-realization-contract",
        );

        assert!(evidence.is_valid());
    }

    #[test]
    fn fallback_rejects_semantic_mismatch() {
        let evidence = FallbackEvidence::new(
            false,
            true,
            true,
            true,
            true,
            "verification-record",
        );

        assert!(!evidence.is_valid());
    }

    #[test]
    fn fallback_rejects_missing_requirements() {
        let evidence = FallbackEvidence::new(
            true,
            false,
            true,
            true,
            true,
            "verification-record",
        );

        assert!(!evidence.is_valid());
    }

    #[test]
    fn fallback_rejects_missing_constraints() {
        let evidence = FallbackEvidence::new(
            true,
            true,
            false,
            true,
            true,
            "verification-record",
        );

        assert!(!evidence.is_valid());
    }

    #[test]
    fn fallback_rejects_unverified_implementation() {
        let evidence = FallbackEvidence::new(
            true,
            true,
            true,
            false,
            true,
            "verification-record",
        );

        assert!(!evidence.is_valid());
    }

    #[test]
    fn fallback_rejects_incompatible_realization() {
        let evidence = FallbackEvidence::new(
            true,
            true,
            true,
            true,
            false,
            "verification-record",
        );

        assert!(!evidence.is_valid());
    }

    #[test]
    fn fallback_rejects_missing_provenance() {
        let evidence = FallbackEvidence::new(
            true,
            true,
            true,
            true,
            true,
            " ",
        );

        assert!(!evidence.is_valid());
    }

    #[test]
    fn retry_safety_is_conservative_by_default() {
        assert!(!RetrySafety::NotEstablished.is_safe());
        assert!(RetrySafety::ReplaySafe.is_safe());
    }

    #[test]
    fn recovery_reasons_have_stable_identifiers() {
        assert_eq!(
            RecoveryReason::RetryAuthorized.as_str(),
            "retry_authorized"
        );

        assert_eq!(
            RecoveryReason::FallbackAuthorized.as_str(),
            "fallback_authorized"
        );

        assert_eq!(
            RecoveryReason::RetryBudgetExhausted.as_str(),
            "retry_budget_exhausted"
        );

        assert_eq!(
            RecoveryReason::CancellationCannotBeRetried.as_str(),
            "cancellation_cannot_be_retried"
        );
    }

    #[test]
    fn recovery_decision_reports_stop_correctly() {
        let decision = RecoveryDecision::new(
            RecoveryAction::Stop,
            RecoveryReason::NonRecoverableFailure,
            "No safe recovery available.",
        );

        assert!(decision.must_stop());
        assert!(!decision.should_retry());
        assert!(!decision.should_use_fallback());
    }

    #[test]
    fn recovery_decision_reports_retry_correctly() {
        let decision = RecoveryDecision::new(
            RecoveryAction::Stop,
            RecoveryReason::RetryNotSafe,
            "Replay safety has not been established.",
        );

        assert!(!decision.should_retry());
        assert!(decision.must_stop());
    }

    #[test]
    fn fallback_provenance_is_preserved() {
        let evidence = FallbackEvidence::new(
            true,
            true,
            true,
            true,
            true,
            "semantic-verifier/v1",
        );

        assert_eq!(
            evidence.provenance(),
            "semantic-verifier/v1"
        );
    }

    #[test]
    fn retry_policy_is_independent_of_fallback_policy() {
        let retry_only = RecoveryPolicy::new()
            .with_max_retries(4)
            .allow_retry_category(FailureCategory::StageExecution);

        assert_eq!(retry_only.max_retries(), 4);
        assert!(!retry_only.fallback_allowed());

        let fallback_only = RecoveryPolicy::new()
            .allow_fallback(true);

        assert_eq!(fallback_only.max_retries(), 0);
        assert!(fallback_only.fallback_allowed());
    }

    #[test]
    fn fallback_evidence_is_not_valid_without_provenance() {
        let evidence = FallbackEvidence::new(
            true,
            true,
            true,
            true,
            true,
            "",
        );

        assert!(!evidence.is_valid());
    }

    #[test]
    fn fallback_policy_can_be_disabled_after_configuration() {
        let policy = RecoveryPolicy::new()
            .allow_fallback(true)
            .allow_fallback(false);

        assert!(!policy.fallback_allowed());
    }
}
