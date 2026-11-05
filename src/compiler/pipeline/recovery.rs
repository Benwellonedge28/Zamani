
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Universal Meta-Compiler — Pipeline Recovery.
//!
//! Path:
//!     src/compiler/pipeline/recovery.rs
//!
//! Rust:
//!     Edition 2021
//!     Minimum supported version: Rust 1.97
//!
//! Safety:
//!     Unsafe Rust is forbidden.
//!
//! ============================================================================
//! PURPOSE
//! ============================================================================
//!
//! Defines the policy and decision layer for recovering from compiler
//! pipeline failures.
//!
//! Recovery decisions include:
//!
//! - Retrying a failed stage.
//! - Selecting an explicitly authorized fallback.
//! - Stopping compilation.
//!
//! A retry or fallback must never silently change program semantics.
//!
//! This module consumes the canonical failure representation from:
//!
//!     pipeline::failure::PipelineFailure
//!
//! It does not create another failure representation.
//!
//! ============================================================================
//! ARCHITECTURAL OWNERSHIP
//! ============================================================================
//!
//! This module owns:
//!
//! - RecoveryPolicy.
//! - RecoveryPlanner.
//! - RecoveryRequest.
//! - RecoveryDecision.
//! - RecoveryAction.
//! - RetrySafety.
//! - FallbackEvidence.
//! - RecoveryReason.
//! - RecoveryPolicyError.
//! - Recovery API versioning.
//!
//! This module does NOT own:
//!
//! - StageError.
//! - PipelineFailure.
//! - StageId.
//! - Stage registration.
//! - Stage execution.
//! - Pipeline orchestration.
//! - Compilation lifecycle.
//! - CancellationToken implementation.
//! - CompilationBudget.
//! - Semantic analysis.
//! - Type checking.
//! - Ownership analysis.
//! - Canonical IR.
//! - Optimization algorithms.
//! - Hardware discovery.
//! - Physical resource allocation.
//! - Hardware scheduling.
//! - Physical routing.
//! - Backend implementations.
//! - Runtime execution.
//! - Distributed execution.
//!
//! Those responsibilities remain with their authoritative subsystems.
//!
//! ============================================================================
//! RECOVERY SAFETY
//! ============================================================================
//!
//! A retry is permitted only when:
//!
//! 1. The original stage declared the failure retryable.
//! 2. The failure is not cancellation.
//! 3. The failure is not an internal correctness invariant violation.
//! 4. The caller establishes that replay is safe.
//! 5. The failure category is authorized by policy.
//! 6. The configured retry budget has not been exhausted.
//!
//! A fallback is permitted only when the caller establishes:
//!
//! 1. Semantic equivalence.
//! 2. Preservation of required capabilities and constraints.
//! 3. Verification of the fallback implementation.
//! 4. Acceptance by the relevant realization contract.
//!
//! This module does not assume that a fallback is equivalent merely
//! because it produces an output.
//!
//! ============================================================================
//! IMPORTANT EXECUTION BOUNDARY
//! ============================================================================
//!
//! RecoveryPlanner produces decisions.
//!
//! It does not independently execute stages or invoke another pipeline.
//!
//! Actual retry execution must be coordinated by pipeline orchestration,
//! which owns lifecycle, cancellation, stage registration and execution.
//!
//! This separation prevents:
//!
//! - Recursive pipeline execution.
//! - Duplicate lifecycle tracking.
//! - Uncontrolled retry loops.
//! - Replaying non-idempotent operations.
//! - Hidden changes to compilation semantics.
//!
//! ============================================================================
//! POCO-REAF
//! ============================================================================
//!
//! No fixed hardware-capacity limits are imposed.
//!
//! No universal maximum is defined for:
//!
//! - CPUs.
//! - GPUs.
//! - FPGAs.
//! - QPUs.
//! - Accelerators.
//! - Distributed nodes.
//! - Devices.
//! - Memory.
//! - Storage.
//! - Quantum resources.
//! - Tensor dimensions.
//! - Compilation stages.
//! - Failure records.
//!
//! Retry limits are configurable operational policies.
//!
//! They are not hardware or language-level capacity restrictions.
//!
//! Physical resources, operating-system restrictions and configured
//! compilation budgets remain applicable.
//!
//! No finite implementation can promise infinite physical resources.
//!
//! ============================================================================
//! GRAMMAR BOUNDARY
//! ============================================================================
//!
//! The grammar describes source-language intent.
//!
//! Semantic analysis determines the meaning of requirements, constraints,
//! capabilities and effects.
//!
//! Recovery is an implementation-level compilation concern.
//!
//! A source-level requirement must never be silently weakened to make
//! recovery succeed.
//!
//! Relevant authorities include:
//!
//!     grammar/Zamani.g4
//!     grammar/specification/semantic-model.md
//!     grammar/validation/semantic-boundaries.md
//!
//! ============================================================================

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
/// Independent of:
///
/// - Zamani language version.
/// - Grammar version.
/// - AST version.
/// - Semantic-model version.
/// - IR version.
/// - Backend ABI version.
/// - Compiler release version.
pub const PIPELINE_RECOVERY_API_VERSION: u16 = 1;

// ============================================================================
// RESULT
// ============================================================================

/// Result type for recovery operations.
pub type RecoveryResult<T> = Result<T, RecoveryPolicyError>;

// ============================================================================
// RETRY SAFETY
// ============================================================================

/// Evidence supplied by the execution coordinator concerning whether
/// replaying a failed stage is safe.
///
/// Retryability declared by StageError is not sufficient by itself.
///
/// A stage may have produced externally observable side effects before
/// returning an error.
///
/// The orchestrator must establish replay safety using the stage's
/// execution contract, transaction state, provenance and side effects.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum RetrySafety {
    /// Replay safety has not been established.
    ///
    /// This is the conservative default.
    NotEstablished,

    /// The execution coordinator has established that replaying the
    /// failed operation is safe.
    ///
    /// This declaration must be based on actual stage guarantees.
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
/// The caller must obtain this evidence from authoritative verification,
/// semantic analysis and target compatibility systems.
///
/// A boolean supplied by an untrusted source is not verification.
///
/// The orchestrator must construct this value only after the relevant
/// verification procedures have succeeded.
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
    /// All conditions are explicit to prevent an accidental assumption
    /// that any alternative implementation is semantically equivalent.
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

    /// Returns the verification provenance.
    #[must_use]
    pub fn provenance(&self) -> &str {
        self.provenance.as_ref()
    }

    /// Returns whether all mandatory fallback conditions hold.
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
/// The policy does not change program semantics.
///
/// An empty retry-category set disables all retries.
///
/// The default policy performs no automatic retries and permits no
/// automatic fallback.
///
/// This conservative default prevents unexpected repeated execution.
///
/// Retry counts are configured by the application or compiler session.
///
/// They are operational limits, not universal hardware limits.
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

    /// Sets the maximum number of retries permitted for one failed stage.
    ///
    /// Zero disables retry.
    ///
    /// The number represents additional executions after the initial
    /// failed execution.
    #[must_use]
    pub fn with_max_retries(mut self, retries: usize) -> Self {
        self.max_retries = retries;
        self
    }

    /// Sets the maximum retries using a non-zero value.
    ///
    /// This is a convenience method for configurations that explicitly
    /// require retries.
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
    /// The stage must still declare retryability and replay safety
    /// must independently be established.
    #[must_use]
    pub fn allow_retry_category(
        mut self,
        category: FailureCategory,
    ) -> Self {
        self.retryable_categories.insert(category);
        self
    }

    /// Authorizes retry after a resource failure.
    ///
    /// This does not mean the resource has become available.
    ///
    /// The orchestrator must establish that a changed resource condition
    /// makes another execution meaningful.
    #[must_use]
    pub fn allow_resource_retry(mut self, allow: bool) -> Self {
        self.allow_retry_after_resource_failure = allow;
        self
    }

    /// Authorizes retry after an external subsystem failure.
    ///
    /// Replay safety and the stage retryability declaration remain
    /// mandatory.
    #[must_use]
    pub fn allow_external_retry(mut self, allow: bool) -> Self {
        self.allow_retry_after_external_failure = allow;
        self
    }

    /// Enables or disables fallback selection.
    ///
    /// Enabling this setting does not authorize an unverified fallback.
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

    /// Returns whether retry is enabled for a failure category.
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
}

// ============================================================================
// RECOVERY REQUEST
// ============================================================================

/// Immutable information needed to evaluate a recovery decision.
///
/// `retries_performed` counts retries already performed after the
/// initial execution.
///
/// For example:
///
/// - Initial execution fails: 0 retries performed.
/// - First retry fails: 1 retry performed.
/// - Second retry fails: 2 retries performed.
///
/// The failure object remains owned by the caller.
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
    pub fn with_retry_safety(
        mut self,
        safety: RetrySafety,
    ) -> Self {
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

    /// Returns the fallback evidence, if provided.
    #[must_use]
    pub fn fallback_evidence(&self) -> Option<&FallbackEvidence> {
        self.fallback_evidence
    }
}

// ============================================================================
// RECOVERY REASONS
// ============================================================================

/// Machine-readable explanation for a recovery decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum RecoveryReason {
    RetryAuthorized,

    RetryNotDeclared,

    RetryNotSafe,

    RetryBudgetExhausted,

    RetryCategoryNotAuthorized,

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

/// The recovery action recommended by the recovery planner.
///
/// These are decisions, not executed operations.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum RecoveryAction {
    /// Retry the failed stage.
    ///
    /// The orchestration subsystem must perform the actual retry.
    Retry {
        stage: StageId,
        next_attempt: usize,
    },

    /// Use an explicitly verified alternative realization.
    ///
    /// The actual fallback implementation is selected and invoked
    /// by the appropriate planning/backend orchestration subsystem.
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
        &self.explanation
    }

    /// Returns whether a retry was selected.
    #[must_use]
    pub fn should_retry(&self) -> bool {
        matches!(self.action, RecoveryAction::Retry { .. })
    }

    /// Returns whether a fallback was selected.
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
// RECOVERY POLICY ERROR
// ============================================================================

/// Errors associated with invalid recovery-policy construction.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum RecoveryPolicyError {
    /// A recovery policy is internally inconsistent.
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
/// It does not mutate the pipeline, execute stages or change lifecycle.
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
    /// 1. Cancellation and correctness-invariant failures stop immediately.
    /// 2. Authorized safe retries are preferred.
    /// 3. Verified fallback may be selected if retry is unavailable.
    /// 4. Otherwise compilation stops.
    ///
    /// A retry never occurs solely because the stage declared retryability.
    #[must_use]
    pub fn plan(
        &self,
        request: &RecoveryRequest<'_>,
        policy: &RecoveryPolicy,
    ) -> RecoveryDecision {
        let failure = request.failure();

        // Cancellation must not be converted into a retry.
        if failure.is_cancelled() {
            return Self::stop(
                RecoveryReason::CancellationCannotBeRetried,
                "Compilation was cancelled. Recovery cannot silently \
                 override the cancellation request.",
            );
        }

        // Internal invariant violations indicate a correctness problem.
        if failure.disposition()
            == FailureDisposition::InternalFailure
        {
            return Self::stop(
                RecoveryReason::InternalInvariantCannotBeRetried,
                "An internal correctness invariant failed. The pipeline \
                 must stop rather than retry potentially corrupted work.",
            );
        }

        // Critical failures require explicit investigation.
        if failure.severity() == FailureSeverity::Critical {
            return Self::stop(
                RecoveryReason::TerminalFailure,
                "A critical pipeline failure cannot be automatically \
                 recovered.",
            );
        }

        // Evaluate retry eligibility.
        if let Some(decision) = Self::retry_decision(request, policy) {
            return decision;
        }

        // A fallback is a separate realization, not an implicit retry.
        if let Some(decision) = Self::fallback_decision(request, policy) {
            return decision;
        }

        Self::stop(
            Self::final_stop_reason(request, policy),
            "No recovery action satisfied the configured policy and \
             required safety conditions. Compilation remains unsuccessful.",
        )
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

        if request.retries_performed() >= policy.max_retries() {
            return None;
        }

        let category = failure.category();

        if !policy.retry_category_allowed(category) {
            return None;
        }

        if category == FailureCategory::Resource
            && !policy.allow_retry_after_resource_failure
        {
            return None;
        }

        if category == FailureCategory::External
            && !policy.allow_retry_after_external_failure
        {
            return None;
        }

        let stage = failure.stage()?.clone();

        let next_attempt = request
            .retries_performed()
            .checked_add(1)?;

        Some(RecoveryDecision::new(
            RecoveryAction::Retry {
                stage,
                next_attempt,
            },
            RecoveryReason::RetryAuthorized,
            "The original stage declared retryability, replay safety \
             was established, the failure category is authorized, \
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

        if !evidence.semantic_equivalence_verified() {
            return None;
        }

        if !evidence.requirements_preserved() {
            return None;
        }

        if !evidence.constraints_preserved() {
            return None;
        }

        if !evidence.fallback_verified() {
            return None;
        }

        if !evidence.realization_compatible() {
            return None;
        }

        if evidence.provenance().trim().is_empty() {
            return None;
        }

        Some(RecoveryDecision::new(
            RecoveryAction::UseFallback {
                stage: request.failure().stage().cloned(),
                provenance: Arc::from(evidence.provenance()),
            },
            RecoveryReason::FallbackAuthorized,
            "The fallback has verified semantic equivalence, preserves \
             required capabilities and constraints, and satisfies \
             the realization compatibility contract.",
        ))
    }

    fn final_stop_reason(
        request: &RecoveryRequest<'_>,
        policy: &RecoveryPolicy,
    ) -> RecoveryReason {
        let failure = request.failure();

        if failure.is_cancelled() {
            return RecoveryReason::CancellationCannotBeRetried;
        }

        if failure.disposition()
            == FailureDisposition::Incomplete
        {
            return RecoveryReason::IncompleteRequiredWork;
        }

        if !failure.is_declared_retryable()
            && request.fallback_evidence().is_none()
        {
            return RecoveryReason::RetryNotDeclared;
        }

        if failure.is_declared_retryable()
            && !request.retry_safety().is_safe()
        {
            return RecoveryReason::RetryNotSafe;
        }

        if request.retries_performed() >= policy.max_retries()
            && failure.is_declared_retryable()
        {
            return RecoveryReason::RetryBudgetExhausted;
        }

        if !policy.retry_category_allowed(failure.category())
            && failure.is_declared_retryable()
        {
            return RecoveryReason::RetryCategoryNotAuthorized;
        }

        if policy.fallback_allowed()
            && request.fallback_evidence().is_none()
        {
            return RecoveryReason::FallbackEvidenceMissing;
        }

        if !policy.fallback_allowed()
            && request.fallback_evidence().is_some()
        {
            return RecoveryReason::FallbackDisabled;
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
    }

    #[test]
    fn retry_limit_is_configurable() {
        let policy = RecoveryPolicy::new()
            .with_max_retries(7);

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
    fn custom_failure_categories_can_be_authorized() {
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

        assert!(!policy.allow_retry_after_resource_failure);
    }

    #[test]
    fn external_retry_requires_explicit_authorization() {
        let policy = RecoveryPolicy::new()
            .allow_retry_category(FailureCategory::External);

        assert!(!policy.allow_retry_after_external_failure);
    }

    #[test]
    fn fallback_is_disabled_by_default() {
        let policy = RecoveryPolicy::default();

        assert!(!policy.fallback_allowed());
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
        // StageId construction and actual failure integration are tested
        // in the pipeline integration test suite.
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
    fn retry_policy_does_not_impose_hardware_limits() {
        let policy = RecoveryPolicy::new()
            .with_max_retries(usize::MAX);

        assert_eq!(policy.max_retries(), usize::MAX);
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
}
