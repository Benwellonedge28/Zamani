//! Zamani Quantum Resilience — Verification Acceptance
//!
//! Path:
//!     src/quantum/resilience/verification/acceptance.rs
//!
//! # Purpose
//!
//! This module is the authoritative final acceptance-policy layer for
//! `quantum::resilience::verification`.
//!
//! Its responsibility is deliberately narrow:
//!
//! ```text
//! verification evidence
//!         │
//!         ▼
//! acceptance policy
//!         │
//!         ▼
//! VerificationDecision
//! ```
//!
//! It does NOT:
//!
//! - execute quantum programs;
//! - perform quantum-state verification;
//! - perform semantic equivalence itself;
//! - perform QEC;
//! - perform routing;
//! - perform scheduling;
//! - perform mitigation;
//! - inspect hardware directly;
//! - infer physical topology;
//! - modify canonical quantum IR;
//! - perform recovery;
//! - decide whether a recovery strategy is economically optimal.
//!
//! Those responsibilities belong to their respective subsystems.
//!
//! # Architectural rule
//!
//! Acceptance is a security and correctness boundary.
//!
//! A result MUST NOT be accepted merely because:
//!
//! - execution completed;
//! - the backend reported success;
//! - a recovery action completed;
//! - a result exists;
//! - one verification dimension passed;
//! - statistical confidence is high;
//! - a result was produced on a trusted backend.
//!
//! Acceptance requires all mandatory verification dimensions to satisfy the
//! configured policy.
//!
//! # Canonical verification architecture
//!
//! ```text
//!                       VerificationRequest
//!                               │
//!                               ▼
//!                    ┌──────────────────────┐
//!                    │ Verification stages │
//!                    └──────────┬───────────┘
//!                               │
//!          ┌────────────────────┼────────────────────┐
//!          │                    │                    │
//!          ▼                    ▼                    ▼
//!      Execution            Semantic             Result
//!          │                    │                    │
//!          ├────────────┬───────┴───────┬────────────┤
//!          ▼            ▼               ▼            ▼
//!     Invariants   Provenance      Confidence   Recovery
//!          │            │               │            │
//!          └────────────┴───────┬───────┴────────────┘
//!                                ▼
//!                       AcceptanceEvaluator
//!                                │
//!                ┌───────────────┼────────────────┐
//!                ▼               ▼                ▼
//!              ACCEPT          REPEAT          ESCALATE
//!                                │                │
//!                                └──────┬─────────┘
//!                                       ▼
//!                                     REJECT
//! ```
//!
//! # Write once, scale everywhere
//!
//! This module contains no:
//!
//! - qubit count;
//! - physical-qubit count;
//! - machine-size constant;
//! - provider name;
//! - backend-specific branch;
//! - fixed retry count;
//! - fixed fidelity threshold;
//! - fixed shot count;
//! - fixed topology;
//! - fixed execution width;
//! - fixed result size.
//!
//! All limits and thresholds are supplied through verification policy and
//! verification evidence.
//!
//! "Infinity" therefore means that this module introduces no artificial finite
//! machine-size ceiling. A concrete execution remains bounded only by the
//! resources and policies supplied by its execution environment.
//!
//! # Determinism
//!
//! Acceptance is deterministic for identical:
//!
//! - verification policy;
//! - verification request;
//! - verification stage results;
//! - aggregate confidence;
//! - violations.
//!
//! This module:
//!
//! - performs no I/O;
//! - reads no clocks;
//! - generates no randomness;
//! - reads no environment variables;
//! - uses no global mutable state;
//! - does not inspect memory addresses;
//! - does not depend on hash-map iteration order;
//! - does not contact providers;
//! - does not mutate quantum state.
//!
//! # Safety invariant
//!
//! The central invariant is:
//!
//! > Availability alone can never cause acceptance.
//!
//! A result can only be accepted when semantic, structural, provenance,
//! confidence, execution, resource and recovery requirements mandated by the
//! active policy have been satisfied.
//!
//! # Relationship with verifier.rs
//!
//! `verification/verifier.rs` owns:
//!
//! - verification orchestration;
//! - invocation of verification components;
//! - collection of evidence;
//! - aggregate verification state;
//! - construction of `VerificationReport`.
//!
//! This module owns:
//!
//! - final acceptance semantics;
//! - mandatory-stage completeness;
//! - acceptance decision classification;
//! - safety checks around contradictory evidence;
//! - deterministic mapping from verification state to decision.
//!
//! The verifier should delegate its final decision to [`AcceptanceEvaluator`].
//!
//! # Rust contract
//!
//! - Rust 1.97 / 1.97.1
//! - Rust 2021
//! - stable Rust
//! - no nightly features
//! - no unsafe code
//!
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_debug_implementations)]
#![deny(rust_2018_idioms)]

use std::fmt;
use std::sync::Arc;

use crate::quantum::resilience::errors::{
    ResilienceError,
    ResilienceErrorCode,
    ResilienceResult,
};

use super::verifier::{
    VerificationConfidence,
    VerificationDecision,
    VerificationPolicy,
    VerificationRequest,
    VerificationStage,
    VerificationStageResult,
    VerificationState,
    VerificationViolation,
    VerificationViolationKind,
};

// =============================================================================
// Schema identity
// =============================================================================

/// Stable schema identifier for the acceptance contract.
pub const ACCEPTANCE_SCHEMA_ID: &str =
    "zamani.quantum.resilience.verification.acceptance";

/// Current semantic version of the acceptance contract.
pub const ACCEPTANCE_SCHEMA_VERSION: u32 = 1;

// =============================================================================
// Stable diagnostic codes
// =============================================================================

const CODE_MISSING_STAGE: &str = "QR-VER-ACCEPT-001";
const CODE_DUPLICATE_STAGE: &str = "QR-VER-ACCEPT-002";
const CODE_FAILED_STAGE: &str = "QR-VER-ACCEPT-003";
const CODE_INDETERMINATE_STAGE: &str = "QR-VER-ACCEPT-004";
const CODE_NOT_REQUIRED_STAGE: &str = "QR-VER-ACCEPT-005";
const CODE_CONFIDENCE_INSUFFICIENT: &str = "QR-VER-ACCEPT-006";
const CODE_EXISTING_VIOLATION: &str = "QR-VER-ACCEPT-007";
const CODE_STAGE_CONTRACT: &str = "QR-VER-ACCEPT-008";

// =============================================================================
// Acceptance mode
// =============================================================================

/// Controls the strictness of final acceptance.
///
/// The default production mode is [`Self::Strict`].
///
/// `Degraded` is intentionally explicit. It must never be inferred merely
/// because verification is incomplete.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum AcceptanceMode {
    /// All mandatory verification requirements must pass.
    Strict,

    /// Explicitly permits a policy-defined degraded acceptance path.
    ///
    /// This mode does NOT automatically accept failed verification. A
    /// degraded result must still satisfy the explicit acceptance rules
    /// supplied by the caller.
    Degraded,
}

impl AcceptanceMode {
    /// Stable machine-readable representation.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Strict => "strict",
            Self::Degraded => "degraded",
        }
    }
}

impl Default for AcceptanceMode {
    fn default() -> Self {
        Self::Strict
    }
}

impl fmt::Display for AcceptanceMode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Acceptance reason
// =============================================================================

/// Machine-readable explanation for an acceptance decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum AcceptanceReason {
    /// All mandatory verification requirements passed.
    AllMandatoryRequirementsPassed,

    /// One or more verification requirements failed after recovery/adaptation.
    RecoveryCycleRequired,

    /// Verification produced insufficient evidence to decide safely.
    VerificationIndeterminate,

    /// Verification failed before a valid recovery cycle existed.
    VerificationRejected,

    /// Verification confidence is below the configured requirement.
    ConfidenceInsufficient,

    /// A structural verification contract was violated.
    VerificationContractViolation,

    /// A contradictory or invalid acceptance state was detected.
    InvalidAcceptanceState,
}

impl AcceptanceReason {
    /// Stable machine-readable representation.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AllMandatoryRequirementsPassed => {
                "all_mandatory_requirements_passed"
            }
            Self::RecoveryCycleRequired => "recovery_cycle_required",
            Self::VerificationIndeterminate => "verification_indeterminate",
            Self::VerificationRejected => "verification_rejected",
            Self::ConfidenceInsufficient => "confidence_insufficient",
            Self::VerificationContractViolation => {
                "verification_contract_violation"
            }
            Self::InvalidAcceptanceState => "invalid_acceptance_state",
        }
    }
}

impl fmt::Display for AcceptanceReason {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Acceptance outcome
// =============================================================================

/// Immutable result of the final acceptance evaluation.
///
/// This is deliberately separate from `VerificationReport`.
///
/// `VerificationReport` records the complete verification process.
/// `AcceptanceOutcome` records the final decision derived from that process.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AcceptanceOutcome {
    schema_id: &'static str,
    schema_version: u32,
    decision: VerificationDecision,
    reason: AcceptanceReason,
    confidence: VerificationConfidence,
    blocking_violations: Arc<[VerificationViolation]>,
}

impl AcceptanceOutcome {
    fn new(
        decision: VerificationDecision,
        reason: AcceptanceReason,
        confidence: VerificationConfidence,
        blocking_violations: Vec<VerificationViolation>,
    ) -> Self {
        Self {
            schema_id: ACCEPTANCE_SCHEMA_ID,
            schema_version: ACCEPTANCE_SCHEMA_VERSION,
            decision,
            reason,
            confidence,
            blocking_violations: blocking_violations.into(),
        }
    }

    /// Schema identifier.
    #[must_use]
    pub const fn schema_id(&self) -> &'static str {
        self.schema_id
    }

    /// Schema version.
    #[must_use]
    pub const fn schema_version(&self) -> u32 {
        self.schema_version
    }

    /// Final verification decision.
    #[must_use]
    pub const fn decision(&self) -> VerificationDecision {
        self.decision
    }

    /// Machine-readable reason.
    #[must_use]
    pub const fn reason(&self) -> AcceptanceReason {
        self.reason
    }

    /// Aggregate verification confidence.
    #[must_use]
    pub const fn confidence(&self) -> VerificationConfidence {
        self.confidence
    }

    /// Blocking violations.
    #[must_use]
    pub fn blocking_violations(&self) -> &[VerificationViolation] {
        &self.blocking_violations
    }

    /// Returns whether the outcome is accepted.
    #[must_use]
    pub const fn accepted(&self) -> bool {
        self.decision.is_accepted()
    }

    /// Returns whether another recovery/execution cycle is appropriate.
    #[must_use]
    pub const fn requires_repeat(&self) -> bool {
        matches!(self.decision, VerificationDecision::Repeat)
    }

    /// Returns whether automatic acceptance is impossible to establish.
    #[must_use]
    pub const fn requires_escalation(&self) -> bool {
        matches!(self.decision, VerificationDecision::Escalate)
    }
}

// =============================================================================
// Acceptance evaluator
// =============================================================================

/// Authoritative final acceptance evaluator.
///
/// The evaluator is intentionally stateless.
///
/// This makes it:
///
/// - deterministic;
/// - thread-safe;
/// - reusable;
/// - independently testable;
/// - suitable for distributed verification;
/// - independent of machine size.
///
/// The evaluator does not execute any recovery operation. It merely decides
/// what the verification result means operationally.
#[derive(Debug, Clone, Copy)]
pub struct AcceptanceEvaluator {
    mode: AcceptanceMode,
}

impl AcceptanceEvaluator {
    /// Creates a strict production evaluator.
    #[must_use]
    pub const fn strict() -> Self {
        Self {
            mode: AcceptanceMode::Strict,
        }
    }

    /// Creates an explicitly degraded-mode evaluator.
    ///
    /// Degraded mode is still fail-closed: failed mandatory verification
    /// requirements do not become accepted merely because this mode is used.
    #[must_use]
    pub const fn degraded() -> Self {
        Self {
            mode: AcceptanceMode::Degraded,
        }
    }

    /// Creates an evaluator with an explicit mode.
    #[must_use]
    pub const fn with_mode(mode: AcceptanceMode) -> Self {
        Self { mode }
    }

    /// Returns the evaluator mode.
    #[must_use]
    pub const fn mode(self) -> AcceptanceMode {
        self.mode
    }

    /// Evaluates final acceptance.
    ///
    /// `required_confidence` is deliberately supplied explicitly rather than
    /// being hard-coded in this module.
    ///
    /// The caller must provide the complete set of stage results generated by
    /// the verification orchestrator.
    ///
    /// This function is deterministic and contains no machine-specific logic.
    pub fn evaluate(
        &self,
        request: &VerificationRequest,
        policy: VerificationPolicy,
        required_confidence: VerificationConfidence,
        overall_state: VerificationState,
        overall_confidence: VerificationConfidence,
        stages: &[VerificationStageResult],
        violations: &[VerificationViolation],
    ) -> ResilienceResult<AcceptanceOutcome> {
        self.validate_inputs(
            request,
            required_confidence,
            stages,
        )?;

        let mut blocking_violations = Vec::new();

        // ---------------------------------------------------------------------
        // Existing verification violations are authoritative blockers.
        // ---------------------------------------------------------------------

        blocking_violations.extend(violations.iter().cloned());

        // ---------------------------------------------------------------------
        // Validate mandatory stage completeness.
        // ---------------------------------------------------------------------

        self.validate_stage_completeness(
            request,
            policy,
            stages,
            &mut blocking_violations,
        );

        // ---------------------------------------------------------------------
        // Validate stage states.
        // ---------------------------------------------------------------------

        self.validate_stage_states(
            request,
            policy,
            stages,
            &mut blocking_violations,
        );

        // ---------------------------------------------------------------------
        // Validate aggregate state.
        // ---------------------------------------------------------------------

        if overall_state == VerificationState::Indeterminate {
            blocking_violations.push(VerificationViolation::new(
                VerificationStage::Acceptance,
                VerificationViolationKind::Indeterminate,
                CODE_INDETERMINATE_STAGE,
                "overall verification state is indeterminate",
            ));
        }

        if overall_state == VerificationState::Failed {
            blocking_violations.push(VerificationViolation::new(
                VerificationStage::Acceptance,
                VerificationViolationKind::ResultInvalid,
                CODE_FAILED_STAGE,
                "overall verification state is failed",
            ));
        }

        // ---------------------------------------------------------------------
        // Validate confidence.
        // ---------------------------------------------------------------------

        if !overall_confidence.meets(required_confidence) {
            blocking_violations.push(VerificationViolation::new(
                VerificationStage::Confidence,
                VerificationViolationKind::ConfidenceInsufficient,
                CODE_CONFIDENCE_INSUFFICIENT,
                "aggregate verification confidence is below the required level",
            ));
        }

        // ---------------------------------------------------------------------
        // Determine final decision.
        // ---------------------------------------------------------------------

        if !blocking_violations.is_empty() {
            return Ok(self.decision_for_blocked_verification(
                request,
                overall_state,
                overall_confidence,
                blocking_violations,
            ));
        }

        // A successful acceptance path requires an explicitly passed overall
        // state. "NotRequired" is never sufficient as an overall state.
        if overall_state != VerificationState::Passed {
            return Ok(AcceptanceOutcome::new(
                VerificationDecision::Escalate,
                AcceptanceReason::InvalidAcceptanceState,
                overall_confidence,
                vec![VerificationViolation::new(
                    VerificationStage::Acceptance,
                    VerificationViolationKind::VerifierContractViolation,
                    CODE_STAGE_CONTRACT,
                    "acceptance requires an explicitly passed overall verification state",
                )],
            ));
        }

        if !overall_confidence.meets(required_confidence) {
            return Ok(AcceptanceOutcome::new(
                VerificationDecision::Escalate,
                AcceptanceReason::ConfidenceInsufficient,
                overall_confidence,
                vec![VerificationViolation::new(
                    VerificationStage::Confidence,
                    VerificationViolationKind::ConfidenceInsufficient,
                    CODE_CONFIDENCE_INSUFFICIENT,
                    "aggregate verification confidence does not satisfy the acceptance requirement",
                )],
            ));
        }

        // ---------------------------------------------------------------------
        // Successful acceptance.
        // ---------------------------------------------------------------------

        let decision = match self.mode {
            AcceptanceMode::Strict => VerificationDecision::Accept,

            // Degraded mode is intentionally conservative. Merely selecting
            // degraded mode does not manufacture degraded acceptance.
            //
            // A future policy may explicitly authorize DegradedAccept through
            // a dedicated policy field. Until such a policy exists, a fully
            // verified result remains an ordinary Accept.
            AcceptanceMode::Degraded => VerificationDecision::Accept,
        };

        Ok(AcceptanceOutcome::new(
            decision,
            AcceptanceReason::AllMandatoryRequirementsPassed,
            overall_confidence,
            Vec::new(),
        ))
    }

    /// Convenience function matching the current `verifier.rs` decision
    /// boundary.
    ///
    /// This function is useful when the verifier has already validated stage
    /// completeness and only needs the final decision.
    pub fn decide(
        &self,
        request: &VerificationRequest,
        overall_state: VerificationState,
        overall_confidence: VerificationConfidence,
        required_confidence: VerificationConfidence,
        violations: &[VerificationViolation],
    ) -> VerificationDecision {
        if !violations.is_empty() {
            return self
                .decision_for_blocked_verification(
                    request,
                    overall_state,
                    overall_confidence,
                    violations.to_vec(),
                )
                .decision();
        }

        if !overall_confidence.meets(required_confidence) {
            return VerificationDecision::Escalate;
        }

        match overall_state {
            VerificationState::Passed => VerificationDecision::Accept,
            VerificationState::Indeterminate => VerificationDecision::Escalate,
            VerificationState::Failed => {
                if request.recovery_or_adaptation_performed() {
                    VerificationDecision::Repeat
                } else {
                    VerificationDecision::Reject
                }
            }
            VerificationState::NotRequired => VerificationDecision::Reject,
        }
    }

    // =========================================================================
    // Input validation
    // =========================================================================

    fn validate_inputs(
        &self,
        request: &VerificationRequest,
        required_confidence: VerificationConfidence,
        stages: &[VerificationStageResult],
    ) -> ResilienceResult<()> {
        if request.execution_id().as_str().is_empty() {
            return Err(ResilienceError::new(
                ResilienceErrorCode::InvalidConfiguration,
                "acceptance requires a non-empty execution identity",
            ));
        }

        if request.expected_semantic_fingerprint().as_str().is_empty() {
            return Err(ResilienceError::new(
                ResilienceErrorCode::InvalidConfiguration,
                "acceptance requires a non-empty expected semantic fingerprint",
            ));
        }

        // The type itself guarantees the confidence range. This explicit
        // parameter is retained to make the acceptance boundary obvious and
        // prevent future callers from bypassing policy configuration.
        let _ = required_confidence;

        if stages.is_empty() {
            return Err(ResilienceError::new(
                ResilienceErrorCode::SemanticVerificationFailed,
                "acceptance requires verification stage evidence",
            ));
        }

        Ok(())
    }

    // =========================================================================
    // Stage completeness
    // =========================================================================

    fn validate_stage_completeness(
        &self,
        request: &VerificationRequest,
        policy: VerificationPolicy,
        stages: &[VerificationStageResult],
        violations: &mut Vec<VerificationViolation>,
    ) {
        let expected = required_stages(request, policy);

        for stage in expected {
            let count = stages
                .iter()
                .filter(|candidate| candidate.stage() == stage)
                .count();

            if count == 0 {
                violations.push(VerificationViolation::new(
                    VerificationStage::Acceptance,
                    VerificationViolationKind::MissingEvidence,
                    CODE_MISSING_STAGE,
                    format!(
                        "mandatory verification stage '{}' is missing",
                        stage.as_str()
                    ),
                ));
            } else if count > 1 {
                violations.push(VerificationViolation::new(
                    VerificationStage::Acceptance,
                    VerificationViolationKind::VerifierContractViolation,
                    CODE_DUPLICATE_STAGE,
                    format!(
                        "verification stage '{}' appears more than once",
                        stage.as_str()
                    ),
                ));
            }
        }

        // Any unknown stage cannot be silently ignored. This protects the
        // acceptance boundary if future stages are added without updating the
        // acceptance contract.
        for stage in stages {
            if !expected.contains(&stage.stage()) {
                violations.push(VerificationViolation::new(
                    VerificationStage::Acceptance,
                    VerificationViolationKind::VerifierContractViolation,
                    CODE_STAGE_CONTRACT,
                    format!(
                        "verification stage '{}' is not enabled by the active policy",
                        stage.stage().as_str()
                    ),
                ));
            }
        }
    }

    // =========================================================================
    // Stage-state validation
    // =========================================================================

    fn validate_stage_states(
        &self,
        _request: &VerificationRequest,
        policy: VerificationPolicy,
        stages: &[VerificationStageResult],
        violations: &mut Vec<VerificationViolation>,
    ) {
        for stage in stages {
            let required = match stage.stage() {
                VerificationStage::Execution => policy.execution(),
                VerificationStage::ResourceIdentity => policy.resource_identity(),
                VerificationStage::Invariants => policy.invariants(),
                VerificationStage::Semantic => policy.semantic(),
                VerificationStage::Result => policy.result(),
                VerificationStage::Provenance => policy.provenance(),
                VerificationStage::Confidence => policy.confidence(),
                VerificationStage::RecoveryIntegrity => {
                    // Recovery integrity is conditionally mandatory. The
                    // request-level condition is checked by
                    // `required_stages()`.
                    policy.recovery_integrity()
                }
                VerificationStage::Acceptance => false,
            };

            if !required {
                if stage.state() != VerificationState::NotRequired {
                    violations.push(VerificationViolation::new(
                        VerificationStage::Acceptance,
                        VerificationViolationKind::VerifierContractViolation,
                        CODE_STAGE_CONTRACT,
                        format!(
                            "disabled verification stage '{}' supplied non-NotRequired evidence",
                            stage.stage().as_str()
                        ),
                    ));
                }

                continue;
            }

            match stage.state() {
                VerificationState::Passed => {}

                VerificationState::Failed => {
                    violations.push(VerificationViolation::new(
                        stage.stage(),
                        violation_kind_for_stage(stage.stage()),
                        CODE_FAILED_STAGE,
                        format!(
                            "mandatory verification stage '{}' failed",
                            stage.stage().as_str()
                        ),
                    ));
                }

                VerificationState::Indeterminate => {
                    violations.push(VerificationViolation::new(
                        stage.stage(),
                        VerificationViolationKind::Indeterminate,
                        CODE_INDETERMINATE_STAGE,
                        format!(
                            "mandatory verification stage '{}' is indeterminate",
                            stage.stage().as_str()
                        ),
                    ));
                }

                VerificationState::NotRequired => {
                    violations.push(VerificationViolation::new(
                        stage.stage(),
                        VerificationViolationKind::MissingEvidence,
                        CODE_NOT_REQUIRED_STAGE,
                        format!(
                            "mandatory verification stage '{}' was marked NotRequired",
                            stage.stage().as_str()
                        ),
                    ));
                }
            }
        }
    }

    // =========================================================================
    // Blocked decision
    // =========================================================================

    fn decision_for_blocked_verification(
        &self,
        request: &VerificationRequest,
        state: VerificationState,
        confidence: VerificationConfidence,
        violations: Vec<VerificationViolation>,
    ) -> AcceptanceOutcome {
        let reason = if violations.iter().any(|violation| {
            violation.kind() == VerificationViolationKind::Indeterminate
                || violation.kind() == VerificationViolationKind::MissingEvidence
        }) {
            AcceptanceReason::VerificationIndeterminate
        } else if violations.iter().any(|violation| {
            violation.kind() == VerificationViolationKind::VerifierContractViolation
        }) {
            AcceptanceReason::VerificationContractViolation
        } else if !confidence.meets(request.required_confidence()) {
            AcceptanceReason::ConfidenceInsufficient
        } else if request.recovery_or_adaptation_performed()
            && state == VerificationState::Failed
        {
            AcceptanceReason::RecoveryCycleRequired
        } else {
            AcceptanceReason::VerificationRejected
        };

        let decision = match state {
            VerificationState::Indeterminate => VerificationDecision::Escalate,

            VerificationState::Failed => {
                if request.recovery_or_adaptation_performed() {
                    VerificationDecision::Repeat
                } else {
                    VerificationDecision::Reject
                }
            }

            VerificationState::Passed => {
                // Any violation while the aggregate state says "Passed" is a
                // contract contradiction. Fail closed.
                VerificationDecision::Reject
            }

            VerificationState::NotRequired => VerificationDecision::Reject,
        };

        AcceptanceOutcome::new(
            decision,
            reason,
            confidence,
            violations,
        )
    }
}

// =============================================================================
// Required-stage calculation
// =============================================================================

/// Returns the exact verification dimensions that must be present.
///
/// The function is deliberately policy-driven and contains no hardware
/// assumptions.
///
/// Recovery integrity is conditional because ordinary executions may not have
/// undergone recovery/adaptation.
fn required_stages(
    request: &VerificationRequest,
    policy: VerificationPolicy,
) -> Vec<VerificationStage> {
    let mut stages = Vec::new();

    if policy.execution() {
        stages.push(VerificationStage::Execution);
    }

    if policy.resource_identity() {
        stages.push(VerificationStage::ResourceIdentity);
    }

    if policy.invariants() {
        stages.push(VerificationStage::Invariants);
    }

    if policy.semantic() {
        stages.push(VerificationStage::Semantic);
    }

    if policy.result() {
        stages.push(VerificationStage::Result);
    }

    if policy.provenance() {
        stages.push(VerificationStage::Provenance);
    }

    if policy.confidence() {
        stages.push(VerificationStage::Confidence);
    }

    if policy.recovery_integrity()
        && request.recovery_or_adaptation_performed()
    {
        stages.push(VerificationStage::RecoveryIntegrity);
    }

    stages
}

// =============================================================================
// Violation mapping
// =============================================================================

fn violation_kind_for_stage(
    stage: VerificationStage,
) -> VerificationViolationKind {
    match stage {
        VerificationStage::Execution => {
            VerificationViolationKind::ExecutionFailure
        }

        VerificationStage::ResourceIdentity => {
            VerificationViolationKind::ResourceIdentityMismatch
        }

        VerificationStage::Invariants => {
            VerificationViolationKind::InvariantFailure
        }

        VerificationStage::Semantic => {
            VerificationViolationKind::SemanticMismatch
        }

        VerificationStage::Result => {
            VerificationViolationKind::ResultInvalid
        }

        VerificationStage::Provenance => {
            VerificationViolationKind::ProvenanceInvalid
        }

        VerificationStage::Confidence => {
            VerificationViolationKind::ConfidenceInsufficient
        }

        VerificationStage::RecoveryIntegrity => {
            VerificationViolationKind::RecoveryIntegrityFailure
        }

        VerificationStage::Acceptance => {
            VerificationViolationKind::VerifierContractViolation
        }
    }
}

// =============================================================================
// Default implementation
// =============================================================================

impl Default for AcceptanceEvaluator {
    fn default() -> Self {
        Self::strict()
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn execution_id() -> super::super::verifier::ExecutionIdentity {
        super::super::verifier::ExecutionIdentity::new("acceptance-test")
            .expect("valid execution identity")
    }

    fn fingerprint(
        value: &str,
    ) -> super::super::verifier::SemanticFingerprint {
        super::super::verifier::SemanticFingerprint::new(value)
            .expect("valid semantic fingerprint")
    }

    fn request() -> VerificationRequest {
        VerificationRequest::new(
            execution_id(),
            fingerprint("semantic"),
            super::super::verifier::VerificationResourceScope::default(),
        )
        .with_candidate_semantic_fingerprint(fingerprint("semantic"))
        .with_execution_success()
    }

    fn passing_stage(
        stage: VerificationStage,
    ) -> VerificationStageResult {
        VerificationStageResult::from_evidence_for_tests(
            stage,
            VerificationState::Passed,
            VerificationConfidence::certain(),
        )
    }

    fn strict_stages() -> Vec<VerificationStageResult> {
        vec![
            passing_stage(VerificationStage::Execution),
            passing_stage(VerificationStage::ResourceIdentity),
            passing_stage(VerificationStage::Invariants),
            passing_stage(VerificationStage::Semantic),
            passing_stage(VerificationStage::Result),
            passing_stage(VerificationStage::Provenance),
            passing_stage(VerificationStage::Confidence),
        ]
    }

    #[test]
    fn strict_acceptance_requires_all_mandatory_stages() {
        let evaluator = AcceptanceEvaluator::strict();
        let request = request();
        let stages = strict_stages();

        let outcome = evaluator
            .evaluate(
                &request,
                VerificationPolicy::strict(),
                VerificationConfidence::certain(),
                VerificationState::Passed,
                VerificationConfidence::certain(),
                &stages,
                &[],
            )
            .expect("acceptance evaluation should succeed");

        assert_eq!(
            outcome.decision(),
            VerificationDecision::Accept
        );

        assert!(outcome.accepted());
        assert!(outcome.blocking_violations().is_empty());
    }

    #[test]
    fn missing_mandatory_stage_cannot_be_accepted() {
        let evaluator = AcceptanceEvaluator::strict();
        let request = request();

        let mut stages = strict_stages();

        stages.retain(|stage| {
            stage.stage() != VerificationStage::Semantic
        });

        let outcome = evaluator
            .evaluate(
                &request,
                VerificationPolicy::strict(),
                VerificationConfidence::certain(),
                VerificationState::Passed,
                VerificationConfidence::certain(),
                &stages,
                &[],
            )
            .expect("acceptance evaluation should produce a decision");

        assert!(!outcome.accepted());
        assert_eq!(
            outcome.decision(),
            VerificationDecision::Reject
        );

        assert!(outcome.blocking_violations().iter().any(|violation| {
            violation.code() == CODE_MISSING_STAGE
        }));
    }

    #[test]
    fn failed_stage_blocks_acceptance() {
        let evaluator = AcceptanceEvaluator::strict();
        let request = request();

        let mut stages = strict_stages();

        stages.retain(|stage| {
            stage.stage() != VerificationStage::Result
        });

        stages.push(
            VerificationStageResult::from_evidence_for_tests(
                VerificationStage::Result,
                VerificationState::Failed,
                VerificationConfidence::certain(),
            ),
        );

        let outcome = evaluator
            .evaluate(
                &request,
                VerificationPolicy::strict(),
                VerificationConfidence::certain(),
                VerificationState::Failed,
                VerificationConfidence::certain(),
                &stages,
                &[],
            )
            .expect("acceptance evaluation should succeed");

        assert!(!outcome.accepted());
        assert_eq!(
            outcome.decision(),
            VerificationDecision::Reject
        );
    }

    #[test]
    fn failed_verification_after_recovery_requests_repeat() {
        let evaluator = AcceptanceEvaluator::strict();

        let request = request()
            .with_recovery_or_adaptation();

        let mut stages = strict_stages();

        stages.retain(|stage| {
            stage.stage() != VerificationStage::Result
        });

        stages.push(
            VerificationStageResult::from_evidence_for_tests(
                VerificationStage::Result,
                VerificationState::Failed,
                VerificationConfidence::certain(),
            ),
        );

        let outcome = evaluator
            .evaluate(
                &request,
                VerificationPolicy::strict(),
                VerificationConfidence::certain(),
                VerificationState::Failed,
                VerificationConfidence::certain(),
                &stages,
                &[],
            )
            .expect("acceptance evaluation should succeed");

        assert_eq!(
            outcome.decision(),
            VerificationDecision::Repeat
        );

        assert!(outcome.requires_repeat());
        assert!(!outcome.accepted());
    }

    #[test]
    fn indeterminate_verification_escalates() {
        let evaluator = AcceptanceEvaluator::strict();
        let request = request();

        let mut stages = strict_stages();

        stages.retain(|stage| {
            stage.stage() != VerificationStage::Semantic
        });

        stages.push(
            VerificationStageResult::from_evidence_for_tests(
                VerificationStage::Semantic,
                VerificationState::Indeterminate,
                VerificationConfidence::none(),
            ),
        );

        let outcome = evaluator
            .evaluate(
                &request,
                VerificationPolicy::strict(),
                VerificationConfidence::certain(),
                VerificationState::Indeterminate,
                VerificationConfidence::none(),
                &stages,
                &[],
            )
            .expect("acceptance evaluation should succeed");

        assert_eq!(
            outcome.decision(),
            VerificationDecision::Escalate
        );

        assert!(outcome.requires_escalation());
        assert!(!outcome.accepted());
    }

    #[test]
    fn insufficient_confidence_cannot_be_accepted() {
        let evaluator = AcceptanceEvaluator::strict();
        let request = request();

        let stages = strict_stages();

        let required = VerificationConfidence::from_basis_points(9_000)
            .expect("valid confidence");

        let actual = VerificationConfidence::from_basis_points(8_999)
            .expect("valid confidence");

        let outcome = evaluator
            .evaluate(
                &request,
                VerificationPolicy::strict(),
                required,
                VerificationState::Passed,
                actual,
                &stages,
                &[],
            )
            .expect("acceptance evaluation should succeed");

        assert!(!outcome.accepted());
        assert_eq!(
            outcome.decision(),
            VerificationDecision::Escalate
        );
    }

    #[test]
    fn preexisting_violation_blocks_acceptance() {
        let evaluator = AcceptanceEvaluator::strict();
        let request = request();
        let stages = strict_stages();

        let violation = VerificationViolation::new(
            VerificationStage::Semantic,
            VerificationViolationKind::SemanticMismatch,
            "TEST-SEMANTIC-001",
            "test semantic mismatch",
        );

        let outcome = evaluator
            .evaluate(
                &request,
                VerificationPolicy::strict(),
                VerificationConfidence::certain(),
                VerificationState::Passed,
                VerificationConfidence::certain(),
                &stages,
                &[violation],
            )
            .expect("acceptance evaluation should succeed");

        assert!(!outcome.accepted());
        assert_eq!(
            outcome.decision(),
            VerificationDecision::Reject
        );
    }

    #[test]
    fn acceptance_is_deterministic() {
        let evaluator = AcceptanceEvaluator::strict();
        let request = request();
        let stages = strict_stages();

        let first = evaluator
            .evaluate(
                &request,
                VerificationPolicy::strict(),
                VerificationConfidence::certain(),
                VerificationState::Passed,
                VerificationConfidence::certain(),
                &stages,
                &[],
            )
            .expect("first evaluation");

        let second = evaluator
            .evaluate(
                &request,
                VerificationPolicy::strict(),
                VerificationConfidence::certain(),
                VerificationState::Passed,
                VerificationConfidence::certain(),
                &stages,
                &[],
            )
            .expect("second evaluation");

        assert_eq!(first, second);
    }

    #[test]
    fn not_required_overall_state_never_accepts() {
        let evaluator = AcceptanceEvaluator::strict();
        let request = request();
        let stages = strict_stages();

        let outcome = evaluator
            .evaluate(
                &request,
                VerificationPolicy::strict(),
                VerificationConfidence::certain(),
                VerificationState::NotRequired,
                VerificationConfidence::certain(),
                &stages,
                &[],
            )
            .expect("acceptance evaluation should succeed");

        assert!(!outcome.accepted());
        assert_eq!(
            outcome.decision(),
            VerificationDecision::Reject
        );
    }

    #[test]
    fn disabled_policy_stages_must_be_not_required() {
        let evaluator = AcceptanceEvaluator::strict();
        let request = request();

        let policy = VerificationPolicy::permissive();

        let stages = vec![
            passing_stage(VerificationStage::Execution),
            passing_stage(VerificationStage::ResourceIdentity),
            passing_stage(VerificationStage::Invariants),
            passing_stage(VerificationStage::Semantic),
            passing_stage(VerificationStage::Result),
            VerificationStageResult::from_evidence_for_tests(
                VerificationStage::Provenance,
                VerificationState::NotRequired,
                VerificationConfidence::certain(),
            ),
            VerificationStageResult::from_evidence_for_tests(
                VerificationStage::Confidence,
                VerificationState::NotRequired,
                VerificationConfidence::certain(),
            ),
        ];

        let outcome = evaluator
            .evaluate(
                &request,
                policy,
                VerificationConfidence::certain(),
                VerificationState::Passed,
                VerificationConfidence::certain(),
                &stages,
                &[],
            )
            .expect("acceptance evaluation should succeed");

        assert!(outcome.accepted());
    }

    #[test]
    fn recovery_integrity_is_required_only_after_recovery() {
        let evaluator = AcceptanceEvaluator::strict();

        let request = request()
            .with_recovery_or_adaptation();

        let stages = strict_stages();

        let outcome = evaluator
            .evaluate(
                &request,
                VerificationPolicy::strict(),
                VerificationConfidence::certain(),
                VerificationState::Passed,
                VerificationConfidence::certain(),
                &stages,
                &[],
            )
            .expect("acceptance evaluation should succeed");

        assert!(!outcome.accepted());

        assert!(outcome.blocking_violations().iter().any(|violation| {
            violation.code() == CODE_MISSING_STAGE
                && violation.message().contains("recovery_integrity")
        }));
    }
}

// =============================================================================
// Test-only construction support
// =============================================================================
//
// `VerificationStageResult` intentionally keeps its constructor private in
// verifier.rs. Tests in this module need deterministic stage fixtures without
// exposing an unsafe/public construction path.
//
// The production implementation below is compiled only for tests.

#[cfg(test)]
trait VerificationStageResultTestExt {
    fn from_evidence_for_tests(
        stage: VerificationStage,
        state: VerificationState,
        confidence: VerificationConfidence,
    ) -> Self;
}

#[cfg(test)]
impl VerificationStageResultTestExt for VerificationStageResult {
    fn from_evidence_for_tests(
        stage: VerificationStage,
        state: VerificationState,
        confidence: VerificationConfidence,
    ) -> Self {
        // This extension point intentionally cannot construct the private
        // fields of verifier.rs from another module. The actual repository
        // integration should therefore expose a crate-visible constructor
        // such as:
        //
        // pub(crate) fn from_evidence_for_acceptance(...)
        //
        // in verifier.rs.
        //
        // Keeping this explicit is preferable to using unsafe reflection or
        // duplicating VerificationStageResult.
        //
        // The implementation is supplied through the verifier module's
        // crate-level test constructor in the integration patch described
        // below.
        VerificationStageResult::from_evidence_for_acceptance(
            stage,
            "test.acceptance",
            VerificationStateEvidence::new(state, confidence),
        )
    }
}

/// Test-only representation used by the crate-visible verifier constructor.
#[cfg(test)]
#[derive(Debug, Clone, Copy)]
struct VerificationStateEvidence {
    state: VerificationState,
    confidence: VerificationConfidence,
}

#[cfg(test)]
impl VerificationStateEvidence {
    const fn new(
        state: VerificationState,
        confidence: VerificationConfidence,
    ) -> Self {
        Self { state, confidence }
    }
}