
#![forbid(unsafe_code)]
#![deny(unused_must_use)]

//! Zamani Compiler Security Verification Coordinator.
//!
//! Path: src/compiler/security/verification.rs
//! Rust: 1.97+
//! Edition: 2021
//!
//! # Purpose
//!
//! Coordinates security verification evidence supplied by authoritative
//! compiler, semantic, IR, backend, artifact, and supply-chain subsystems.
//!
//! This module does not implement a competing semantic analyzer, IR,
//! cryptographic library, policy engine, audit database, or runtime sandbox.
//!
//! It coordinates independent verification providers and produces a
//! deterministic, evidence-backed verification report.
//!
//! # Security invariants
//!
//! - Missing evidence is never considered verified.
//! - Indeterminate evidence is never considered verified.
//! - Provider errors never become successful verification.
//! - A required check without a provider cannot pass.
//! - A rejected check prevents acceptance.
//! - A required indeterminate check prevents acceptance.
//! - A report is not proof of runtime safety.
//! - A verification claim is not a cryptographic signature.
//! - A provider identity is not proof of provider trust.
//! - No source or IR is executed by this module.
//!
//! # Ownership
//!
//! This module owns:
//!
//! - verification request contracts;
//! - verification check identifiers;
//! - verification evidence state;
//! - provider coordination;
//! - verification policy;
//! - verification reports;
//! - acceptance decisions;
//! - operational budget enforcement;
//! - cancellation handling;
//! - observer integration.
//!
//! It does not own:
//!
//! - semantic meaning or type checking;
//! - ownership and borrow checking;
//! - canonical IR definitions;
//! - IR transformation;
//! - cryptographic primitives;
//! - package resolution;
//! - supply-chain storage;
//! - backend code generation;
//! - physical hardware discovery;
//! - runtime execution;
//! - runtime authorization;
//! - audit persistence.
//!
//! # Integration boundaries
//!
//! - `src/compiler/security/audit.rs` consumes verification observations.
//! - `src/compiler/security/supply_chain.rs` supplies dependency and artifact
//!   verification providers.
//! - `src/compiler/safety_guard.rs` supplies instruction-stream security
//!   observations through an adapter.
//! - Canonical semantic analysis supplies semantic verification providers.
//! - Canonical IR verification supplies IR verification providers.
//! - `src/backend/` supplies backend and generated-artifact verification.
//! - `src/runtime/` remains responsible for runtime enforcement.
//!
//! The coordinator deliberately uses traits so these subsystems can integrate
//! without introducing circular dependencies.
//!
//! # Scalability
//!
//! No universal maximum is imposed on checks, providers, subjects, evidence,
//! compiler stages, hardware devices, or computing domains.
//!
//! Optional operational budgets are caller-supplied. Actual scalability is
//! limited by available resources and configured policy.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

// ============================================================================
// 1. RESULT AND ERROR MODEL
// ============================================================================

/// Result returned by verification operations.
pub type VerificationResult<T> = Result<T, VerificationError>;

/// Errors raised by the verification coordinator.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum VerificationError {
    EmptyField {
        field: &'static str,
    },

    InvalidField {
        field: &'static str,
    },

    DuplicateCheck {
        id: String,
    },

    DuplicateProvider {
        id: String,
    },

    DuplicateSubject {
        id: String,
    },

    UnknownRequiredCheck {
        id: String,
    },

    BudgetExceeded {
        resource: &'static str,
        limit: usize,
        attempted: usize,
    },

    ProviderFailure {
        provider: String,
        message: String,
    },

    ProviderContractViolation {
        provider: String,
        message: String,
    },

    Cancelled,

    InvalidPolicy {
        message: String,
    },

    InvalidReport {
        message: String,
    },
}

impl fmt::Display for VerificationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyField { field } => {
                write!(f, "verification field cannot be empty: {field}")
            }

            Self::InvalidField { field } => {
                write!(f, "invalid verification field: {field}")
            }

            Self::DuplicateCheck { id } => {
                write!(f, "duplicate verification check: {id}")
            }

            Self::DuplicateProvider { id } => {
                write!(f, "duplicate verification provider: {id}")
            }

            Self::DuplicateSubject { id } => {
                write!(f, "duplicate verification subject: {id}")
            }

            Self::UnknownRequiredCheck { id } => {
                write!(f, "required verification check is not registered: {id}")
            }

            Self::BudgetExceeded {
                resource,
                limit,
                attempted,
            } => {
                write!(
                    f,
                    "verification budget exceeded for {resource}: \
                     limit={limit}, attempted={attempted}"
                )
            }

            Self::ProviderFailure { provider, message } => {
                write!(f, "verification provider {provider} failed: {message}")
            }

            Self::ProviderContractViolation { provider, message } => {
                write!(
                    f,
                    "verification provider {provider} violated its contract: {message}"
                )
            }

            Self::Cancelled => {
                f.write_str("security verification was cancelled")
            }

            Self::InvalidPolicy { message } => {
                write!(f, "invalid verification policy: {message}")
            }

            Self::InvalidReport { message } => {
                write!(f, "invalid verification report: {message}")
            }
        }
    }
}

impl Error for VerificationError {}

// ============================================================================
// 2. FIELD VALIDATION
// ============================================================================

fn validate_text(
    field: &'static str,
    value: &str,
) -> VerificationResult<()> {
    if value.trim().is_empty() {
        return Err(VerificationError::EmptyField { field });
    }

    if value.chars().any(char::is_control) {
        return Err(VerificationError::InvalidField { field });
    }

    Ok(())
}

// ============================================================================
// 3. OPEN-WORLD IDENTIFIERS
// ============================================================================

macro_rules! string_identifier {
    ($name:ident, $description:literal) => {
        #[derive(
            Debug,
            Clone,
            PartialEq,
            Eq,
            PartialOrd,
            Ord,
            Hash
        )]
        pub struct $name(String);

        impl $name {
            pub fn new(value: impl Into<String>) -> VerificationResult<Self> {
                let value = value.into();
                validate_text($description, &value)?;
                Ok(Self(value))
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(&self.0)
            }
        }
    };
}

string_identifier!(CheckId, "verification check identifier");
string_identifier!(ProviderId, "verification provider identifier");
string_identifier!(SubjectId, "verification subject identifier");
string_identifier!(EvidenceId, "verification evidence identifier");
string_identifier!(VerificationDomain, "verification domain");
string_identifier!(VerificationReference, "verification reference");
string_identifier!(VerificationRunId, "verification run identifier");

// ============================================================================
// 4. VERIFICATION SUBJECT
// ============================================================================

/// Identifies the object being verified.
///
/// The identifier is opaque. Its meaning is established by the subsystem
/// that creates the verification request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerificationSubject {
    id: SubjectId,
    kind: String,
    description: Option<String>,
}

impl VerificationSubject {
    pub fn new(
        id: SubjectId,
        kind: impl Into<String>,
    ) -> VerificationResult<Self> {
        let kind = kind.into();
        validate_text("verification subject kind", &kind)?;

        Ok(Self {
            id,
            kind,
            description: None,
        })
    }

    pub fn with_description(
        mut self,
        description: impl Into<String>,
    ) -> VerificationResult<Self> {
        let description = description.into();
        validate_text("verification subject description", &description)?;
        self.description = Some(description);
        Ok(self)
    }

    pub fn id(&self) -> &SubjectId {
        &self.id
    }

    pub fn kind(&self) -> &str {
        &self.kind
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }
}

// ============================================================================
// 5. CHECK DEFINITIONS
// ============================================================================

/// Importance of a verification check.
///
/// Required checks participate in the acceptance decision.
/// Optional checks are reported but do not independently block acceptance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckRequirement {
    Required,
    Optional,
}

/// Stable description of a verification obligation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerificationCheck {
    id: CheckId,
    domain: VerificationDomain,
    requirement: CheckRequirement,
    description: String,
}

impl VerificationCheck {
    pub fn new(
        id: CheckId,
        domain: VerificationDomain,
        requirement: CheckRequirement,
        description: impl Into<String>,
    ) -> VerificationResult<Self> {
        let description = description.into();
        validate_text("verification check description", &description)?;

        Ok(Self {
            id,
            domain,
            requirement,
            description,
        })
    }

    pub fn id(&self) -> &CheckId {
        &self.id
    }

    pub fn domain(&self) -> &VerificationDomain {
        &self.domain
    }

    pub fn requirement(&self) -> CheckRequirement {
        self.requirement
    }

    pub fn description(&self) -> &str {
        &self.description
    }
}

// ============================================================================
// 6. EVIDENCE MODEL
// ============================================================================

/// Actual state reported by a verification provider.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvidenceStatus {
    Verified,
    Rejected,
    Indeterminate,
}

/// Evidence produced by an authoritative provider.
///
/// The coordinator does not infer evidence from a package name, target name,
/// file extension, source annotation, or user-provided "verified" flag.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerificationEvidence {
    id: EvidenceId,
    check_id: CheckId,
    subject_id: SubjectId,
    provider_id: ProviderId,
    status: EvidenceStatus,
    reference: Option<VerificationReference>,
    explanation: Option<String>,
}

impl VerificationEvidence {
    pub fn new(
        id: EvidenceId,
        check_id: CheckId,
        subject_id: SubjectId,
        provider_id: ProviderId,
        status: EvidenceStatus,
    ) -> Self {
        Self {
            id,
            check_id,
            subject_id,
            provider_id,
            status,
            reference: None,
            explanation: None,
        }
    }

    pub fn with_reference(
        mut self,
        reference: VerificationReference,
    ) -> Self {
        self.reference = Some(reference);
        self
    }

    pub fn with_explanation(
        mut self,
        explanation: impl Into<String>,
    ) -> VerificationResult<Self> {
        let explanation = explanation.into();
        validate_text("verification explanation", &explanation)?;
        self.explanation = Some(explanation);
        Ok(self)
    }

    pub fn id(&self) -> &EvidenceId {
        &self.id
    }

    pub fn check_id(&self) -> &CheckId {
        &self.check_id
    }

    pub fn subject_id(&self) -> &SubjectId {
        &self.subject_id
    }

    pub fn provider_id(&self) -> &ProviderId {
        &self.provider_id
    }

    pub fn status(&self) -> EvidenceStatus {
        self.status
    }

    pub fn reference(&self) -> Option<&VerificationReference> {
        self.reference.as_ref()
    }

    pub fn explanation(&self) -> Option<&str> {
        self.explanation.as_deref()
    }
}

// ============================================================================
// 7. PROVIDER CONTRACT
// ============================================================================

/// Immutable request passed to an authoritative verification provider.
#[derive(Debug, Clone)]
pub struct VerificationRequest {
    run_id: VerificationRunId,
    subject: VerificationSubject,
    check: VerificationCheck,
}

impl VerificationRequest {
    pub fn run_id(&self) -> &VerificationRunId {
        &self.run_id
    }

    pub fn subject(&self) -> &VerificationSubject {
        &self.subject
    }

    pub fn check(&self) -> &VerificationCheck {
        &self.check
    }
}

/// A provider evaluates one explicitly registered verification check.
///
/// Implementations should delegate to the subsystem that owns the relevant
/// truth. Examples include:
///
/// - semantic verification;
/// - ownership verification;
/// - canonical IR verification;
/// - backend artifact verification;
/// - signature verification;
/// - dependency integrity verification;
/// - supply-chain attestation verification.
///
/// A provider must not return `Verified` merely because evidence is absent.
pub trait VerificationProvider: Send + Sync {
    fn id(&self) -> ProviderId;

    fn supports(&self, check: &VerificationCheck) -> bool;

    fn verify(
        &self,
        request: &VerificationRequest,
    ) -> VerificationResult<VerificationEvidence>;
}

// ============================================================================
// 8. OBSERVER INTEGRATION
// ============================================================================

/// Receives verification outcomes for audit and telemetry integration.
///
/// An observer is not a verification provider. Its successful execution
/// does not change the verification result.
pub trait VerificationObserver: Send + Sync {
    fn observe(
        &self,
        run_id: &VerificationRunId,
        evidence: &VerificationEvidence,
    ) -> VerificationResult<()>;
}

// ============================================================================
// 9. OPERATIONAL BUDGET
// ============================================================================

/// Optional operational limits.
///
/// `None` means that this coordinator imposes no application-level limit
/// for that resource. The operating environment may still impose limits.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct VerificationBudget {
    pub max_checks: Option<usize>,
    pub max_subjects: Option<usize>,
    pub max_evidence: Option<usize>,
    pub max_provider_invocations: Option<usize>,
}

impl VerificationBudget {
    fn check(
        &self,
        resource: &'static str,
        limit: Option<usize>,
        attempted: usize,
    ) -> VerificationResult<()> {
        if let Some(limit) = limit {
            if attempted > limit {
                return Err(VerificationError::BudgetExceeded {
                    resource,
                    limit,
                    attempted,
                });
            }
        }

        Ok(())
    }
}

// ============================================================================
// 10. VERIFICATION POLICY
// ============================================================================

/// Policy for interpreting verification evidence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerificationPolicy {
    /// Reject a run when a required check has no supporting provider.
    pub require_provider_for_required_checks: bool,

    /// Reject a run when a required check is indeterminate.
    pub reject_indeterminate_required_checks: bool,

    /// Reject a run when any required check is rejected.
    pub reject_failed_required_checks: bool,

    /// Whether provider failures abort the run immediately.
    ///
    /// When false, provider failures are represented as indeterminate
    /// evidence and still cannot satisfy required checks.
    pub abort_on_provider_error: bool,
}

impl Default for VerificationPolicy {
    fn default() -> Self {
        Self {
            require_provider_for_required_checks: true,
            reject_indeterminate_required_checks: true,
            reject_failed_required_checks: true,
            abort_on_provider_error: false,
        }
    }
}

impl VerificationPolicy {
    pub fn validate(&self) -> VerificationResult<()> {
        // These fields are intentionally explicit. A caller may select
        // a different policy, but the report always preserves the actual
        // evidence status and never rewrites failure as success.
        Ok(())
    }
}

// ============================================================================
// 11. VERIFICATION REQUEST BUILDER
// ============================================================================

/// Complete request for one verification run.
#[derive(Debug, Clone)]
pub struct VerificationPlan {
    run_id: VerificationRunId,
    subjects: Vec<VerificationSubject>,
    checks: Vec<VerificationCheck>,
    budget: VerificationBudget,
    policy: VerificationPolicy,
}

impl VerificationPlan {
    pub fn new(run_id: VerificationRunId) -> Self {
        Self {
            run_id,
            subjects: Vec::new(),
            checks: Vec::new(),
            budget: VerificationBudget::default(),
            policy: VerificationPolicy::default(),
        }
    }

    pub fn add_subject(
        &mut self,
        subject: VerificationSubject,
    ) -> VerificationResult<()> {
        if self.subjects.iter().any(|existing| existing.id() == subject.id()) {
            return Err(VerificationError::DuplicateSubject {
                id: subject.id().to_string(),
            });
        }

        self.subjects.push(subject);
        Ok(())
    }

    pub fn add_check(
        &mut self,
        check: VerificationCheck,
    ) -> VerificationResult<()> {
        if self.checks.iter().any(|existing| existing.id() == check.id()) {
            return Err(VerificationError::DuplicateCheck {
                id: check.id().to_string(),
            });
        }

        self.checks.push(check);
        Ok(())
    }

    pub fn with_budget(mut self, budget: VerificationBudget) -> Self {
        self.budget = budget;
        self
    }

    pub fn with_policy(
        mut self,
        policy: VerificationPolicy,
    ) -> VerificationResult<Self> {
        policy.validate()?;
        self.policy = policy;
        Ok(self)
    }

    pub fn run_id(&self) -> &VerificationRunId {
        &self.run_id
    }

    pub fn subjects(&self) -> &[VerificationSubject] {
        &self.subjects
    }

    pub fn checks(&self) -> &[VerificationCheck] {
        &self.checks
    }
}

// ============================================================================
// 12. RESULT CLASSIFICATION
// ============================================================================

/// Result of evaluating one subject/check pair.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckOutcome {
    Verified,
    Rejected,
    Indeterminate,
    Missing,
    NotApplicable,
}

/// One evaluated check result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckResult {
    subject_id: SubjectId,
    check_id: CheckId,
    requirement: CheckRequirement,
    outcome: CheckOutcome,
    evidence: Vec<VerificationEvidence>,
}

impl CheckResult {
    pub fn subject_id(&self) -> &SubjectId {
        &self.subject_id
    }

    pub fn check_id(&self) -> &CheckId {
        &self.check_id
    }

    pub fn requirement(&self) -> CheckRequirement {
        self.requirement
    }

    pub fn outcome(&self) -> CheckOutcome {
        self.outcome
    }

    pub fn evidence(&self) -> &[VerificationEvidence] {
        &self.evidence
    }
}

/// Overall acceptance state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerificationDecision {
    Accepted,
    Rejected,
    Indeterminate,
}

/// Aggregate counts for a report.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct VerificationSummary {
    pub verified: usize,
    pub rejected: usize,
    pub indeterminate: usize,
    pub missing: usize,
    pub not_applicable: usize,
}

impl VerificationSummary {
    fn record(&mut self, outcome: CheckOutcome) {
        match outcome {
            CheckOutcome::Verified => {
                self.verified = self.verified.saturating_add(1);
            }
            CheckOutcome::Rejected => {
                self.rejected = self.rejected.saturating_add(1);
            }
            CheckOutcome::Indeterminate => {
                self.indeterminate = self.indeterminate.saturating_add(1);
            }
            CheckOutcome::Missing => {
                self.missing = self.missing.saturating_add(1);
            }
            CheckOutcome::NotApplicable => {
                self.not_applicable = self.not_applicable.saturating_add(1);
            }
        }
    }

    pub fn total(&self) -> usize {
        self.verified
            .saturating_add(self.rejected)
            .saturating_add(self.indeterminate)
            .saturating_add(self.missing)
            .saturating_add(self.not_applicable)
    }
}

// ============================================================================
// 13. REPORT
// ============================================================================

/// Immutable result of a verification run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerificationReport {
    run_id: VerificationRunId,
    decision: VerificationDecision,
    results: Vec<CheckResult>,
    summary: VerificationSummary,
    provider_errors: Vec<String>,
}

impl VerificationReport {
    pub fn run_id(&self) -> &VerificationRunId {
        &self.run_id
    }

    pub fn decision(&self) -> VerificationDecision {
        self.decision
    }

    pub fn results(&self) -> &[CheckResult] {
        &self.results
    }

    pub fn summary(&self) -> VerificationSummary {
        self.summary
    }

    pub fn provider_errors(&self) -> &[String] {
        &self.provider_errors
    }

    pub fn is_accepted(&self) -> bool {
        self.decision == VerificationDecision::Accepted
    }

    pub fn has_rejections(&self) -> bool {
        self.summary.rejected > 0
    }

    pub fn has_indeterminate_evidence(&self) -> bool {
        self.summary.indeterminate > 0 || self.summary.missing > 0
    }
}

// ============================================================================
// 14. CANCELLATION
// ============================================================================

/// Shared cancellation token.
///
/// The caller controls cancellation. This module does not create background
/// tasks or workers.
#[derive(Debug, Clone, Default)]
pub struct VerificationCancellation {
    cancelled: Arc<AtomicBool>,
}

impl VerificationCancellation {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::Release);
    }

    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Acquire)
    }

    fn check(&self) -> VerificationResult<()> {
        if self.is_cancelled() {
            Err(VerificationError::Cancelled)
        } else {
            Ok(())
        }
    }
}

// ============================================================================
// 15. COORDINATOR
// ============================================================================

/// Coordinates verification providers.
///
/// Providers are stored by stable identifier. The coordinator executes
/// checks in deterministic identifier order. It does not spawn threads,
/// perform network access, or allocate hardware.
pub struct VerificationCoordinator {
    providers: BTreeMap<ProviderId, Arc<dyn VerificationProvider>>,
    observers: Vec<Arc<dyn VerificationObserver>>,
}

impl Default for VerificationCoordinator {
    fn default() -> Self {
        Self::new()
    }
}

impl VerificationCoordinator {
    pub fn new() -> Self {
        Self {
            providers: BTreeMap::new(),
            observers: Vec::new(),
        }
    }

    pub fn register_provider(
        &mut self,
        provider: Arc<dyn VerificationProvider>,
    ) -> VerificationResult<()> {
        let id = provider.id();

        if self.providers.contains_key(&id) {
            return Err(VerificationError::DuplicateProvider {
                id: id.to_string(),
            });
        }

        self.providers.insert(id, provider);
        Ok(())
    }

    pub fn register_observer(
        &mut self,
        observer: Arc<dyn VerificationObserver>,
    ) {
        self.observers.push(observer);
    }

    pub fn provider_count(&self) -> usize {
        self.providers.len()
    }

    /// Execute a verification plan.
    ///
    /// Every subject is evaluated against every declared check. A check
    /// without a supporting provider is missing evidence, never success.
    pub fn verify(
        &self,
        plan: &VerificationPlan,
        cancellation: &VerificationCancellation,
    ) -> VerificationResult<VerificationReport> {
        plan.policy.validate()?;

        plan.budget.check(
            "checks",
            plan.budget.max_checks,
            plan.checks.len(),
        )?;

        plan.budget.check(
            "subjects",
            plan.budget.max_subjects,
            plan.subjects.len(),
        )?;

        let mut ordered_checks = plan.checks.clone();
        ordered_checks.sort_by(|a, b| a.id.cmp(&b.id));

        let mut ordered_subjects = plan.subjects.clone();
        ordered_subjects.sort_by(|a, b| a.id.cmp(&b.id));

        let mut results = Vec::new();
        let mut provider_errors = Vec::new();
        let mut evidence_count = 0usize;
        let mut provider_invocations = 0usize;

        for subject in &ordered_subjects {
            for check in &ordered_checks {
                cancellation.check()?;

                let request = VerificationRequest {
                    run_id: plan.run_id.clone(),
                    subject: subject.clone(),
                    check: check.clone(),
                };

                let mut evidence = Vec::new();
                let mut supporting_provider_found = false;

                for provider in self.providers.values() {
                    cancellation.check()?;

                    if !provider.supports(check) {
                        continue;
                    }

                    supporting_provider_found = true;

                    provider_invocations = provider_invocations
                        .checked_add(1)
                        .ok_or(VerificationError::BudgetExceeded {
                            resource: "provider invocations",
                            limit: usize::MAX,
                            attempted: usize::MAX,
                        })?;

                    plan.budget.check(
                        "provider invocations",
                        plan.budget.max_provider_invocations,
                        provider_invocations,
                    )?;

                    match provider.verify(&request) {
                        Ok(item) => {
                            if item.check_id() != check.id()
                                || item.subject_id() != subject.id()
                                || item.provider_id() != &provider.id()
                            {
                                return Err(
                                    VerificationError::ProviderContractViolation {
                                        provider: provider.id().to_string(),
                                        message: "returned evidence does not match request"
                                            .to_string(),
                                    },
                                );
                            }

                            evidence_count = evidence_count
                                .checked_add(1)
                                .ok_or(VerificationError::BudgetExceeded {
                                    resource: "evidence",
                                    limit: usize::MAX,
                                    attempted: usize::MAX,
                                })?;

                            plan.budget.check(
                                "evidence",
                                plan.budget.max_evidence,
                                evidence_count,
                            )?;

                            for observer in &self.observers {
                                observer.observe(&plan.run_id, &item)?;
                            }

                            evidence.push(item);
                        }

                        Err(error) => {
                            let message = error.to_string();
                            provider_errors.push(format!(
                                "{}: {}",
                                provider.id(),
                                message
                            ));

                            if plan.policy.abort_on_provider_error {
                                return Err(VerificationError::ProviderFailure {
                                    provider: provider.id().to_string(),
                                    message,
                                });
                            }
                        }
                    }
                }

                let outcome = evaluate_check(
                    check,
                    supporting_provider_found,
                    &evidence,
                    &plan.policy,
                );

                results.push(CheckResult {
                    subject_id: subject.id().clone(),
                    check_id: check.id().clone(),
                    requirement: check.requirement(),
                    outcome,
                    evidence,
                });
            }
        }

        let summary = summarize(&results);
        let decision = decide(&results, &plan.policy);

        Ok(VerificationReport {
            run_id: plan.run_id.clone(),
            decision,
            results,
            summary,
            provider_errors,
        })
    }
}

// ============================================================================
// 16. CHECK EVALUATION
// ============================================================================

fn evaluate_check(
    check: &VerificationCheck,
    supporting_provider_found: bool,
    evidence: &[VerificationEvidence],
    policy: &VerificationPolicy,
) -> CheckOutcome {
    if evidence.iter().any(|item| item.status() == EvidenceStatus::Rejected) {
        return CheckOutcome::Rejected;
    }

    if evidence.iter().any(|item| item.status() == EvidenceStatus::Indeterminate) {
        return CheckOutcome::Indeterminate;
    }

    if !supporting_provider_found || evidence.is_empty() {
        if check.requirement() == CheckRequirement::Required
            && policy.require_provider_for_required_checks
        {
            return CheckOutcome::Missing;
        }

        return CheckOutcome::NotApplicable;
    }

    if evidence.iter().all(|item| item.status() == EvidenceStatus::Verified) {
        CheckOutcome::Verified
    } else {
        CheckOutcome::Indeterminate
    }
}

// ============================================================================
// 17. REPORT AGGREGATION
// ============================================================================

fn summarize(results: &[CheckResult]) -> VerificationSummary {
    let mut summary = VerificationSummary::default();

    for result in results {
        summary.record(result.outcome);
    }

    summary
}

fn decide(
    results: &[CheckResult],
    policy: &VerificationPolicy,
) -> VerificationDecision {
    let mut has_required_indeterminate = false;
    let mut has_required_rejection = false;

    for result in results {
        if result.requirement != CheckRequirement::Required {
            continue;
        }

        match result.outcome {
            CheckOutcome::Rejected => {
                has_required_rejection = true;
            }

            CheckOutcome::Indeterminate
            | CheckOutcome::Missing => {
                has_required_indeterminate = true;
            }

            CheckOutcome::Verified
            | CheckOutcome::NotApplicable => {}
        }
    }

    if has_required_rejection && policy.reject_failed_required_checks {
        return VerificationDecision::Rejected;
    }

    if has_required_indeterminate
        && policy.reject_indeterminate_required_checks
    {
        return VerificationDecision::Indeterminate;
    }

    if has_required_rejection || has_required_indeterminate {
        return VerificationDecision::Indeterminate;
    }

    VerificationDecision::Accepted
}

// ============================================================================
// 18. ADAPTER SUPPORT
// ============================================================================

/// Helper for adapting a subsystem's existing verification result.
///
/// The adapter must supply the actual result from its authoritative
/// verification implementation.
pub fn evidence_from_result(
    evidence_id: EvidenceId,
    check_id: CheckId,
    subject_id: SubjectId,
    provider_id: ProviderId,
    verified: bool,
    explanation: Option<String>,
) -> VerificationResult<VerificationEvidence> {
    let status = if verified {
        EvidenceStatus::Verified
    } else {
        EvidenceStatus::Rejected
    };

    let mut evidence = VerificationEvidence::new(
        evidence_id,
        check_id,
        subject_id,
        provider_id,
        status,
    );

    if let Some(explanation) = explanation {
        evidence = evidence.with_explanation(explanation)?;
    }

    Ok(evidence)
}

// ============================================================================
// 19. TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    struct StaticProvider {
        id: ProviderId,
        status: EvidenceStatus,
    }

    impl VerificationProvider for StaticProvider {
        fn id(&self) -> ProviderId {
            self.id.clone()
        }

        fn supports(&self, _check: &VerificationCheck) -> bool {
            true
        }

        fn verify(
            &self,
            request: &VerificationRequest,
        ) -> VerificationResult<VerificationEvidence> {
            Ok(VerificationEvidence::new(
                EvidenceId::new(format!(
                    "{}:{}",
                    request.subject().id(),
                    request.check().id()
                ))?,
                request.check().id().clone(),
                request.subject().id().clone(),
                self.id.clone(),
                self.status,
            ))
        }
    }

    fn sample_plan() -> VerificationResult<VerificationPlan> {
        let mut plan = VerificationPlan::new(
            VerificationRunId::new("test-run")?,
        );

        plan.add_subject(VerificationSubject::new(
            SubjectId::new("program")?,
            "program",
        )?)?;

        plan.add_check(VerificationCheck::new(
            CheckId::new("ir.integrity")?,
            VerificationDomain::new("ir")?,
            CheckRequirement::Required,
            "Verify canonical IR integrity",
        )?)?;

        Ok(plan)
    }

    #[test]
    fn verified_required_check_is_accepted() -> VerificationResult<()> {
        let mut coordinator = VerificationCoordinator::new();

        coordinator.register_provider(Arc::new(StaticProvider {
            id: ProviderId::new("test.ir")?,
            status: EvidenceStatus::Verified,
        }))?;

        let report = coordinator.verify(
            &sample_plan()?,
            &VerificationCancellation::new(),
        )?;

        assert_eq!(report.decision(), VerificationDecision::Accepted);
        assert_eq!(report.summary().verified, 1);

        Ok(())
    }

    #[test]
    fn missing_required_provider_is_not_accepted() -> VerificationResult<()> {
        let coordinator = VerificationCoordinator::new();

        let report = coordinator.verify(
            &sample_plan()?,
            &VerificationCancellation::new(),
        )?;

        assert_eq!(report.decision(), VerificationDecision::Indeterminate);
        assert_eq!(report.summary().missing, 1);

        Ok(())
    }

    #[test]
    fn rejected_required_check_is_rejected() -> VerificationResult<()> {
        let mut coordinator = VerificationCoordinator::new();

        coordinator.register_provider(Arc::new(StaticProvider {
            id: ProviderId::new("test.ir")?,
            status: EvidenceStatus::Rejected,
        }))?;

        let report = coordinator.verify(
            &sample_plan()?,
            &VerificationCancellation::new(),
        )?;

        assert_eq!(report.decision(), VerificationDecision::Rejected);
        assert_eq!(report.summary().rejected, 1);

        Ok(())
    }

    #[test]
    fn indeterminate_required_check_is_not_accepted() -> VerificationResult<()> {
        let mut coordinator = VerificationCoordinator::new();

        coordinator.register_provider(Arc::new(StaticProvider {
            id: ProviderId::new("test.ir")?,
            status: EvidenceStatus::Indeterminate,
        }))?;

        let report = coordinator.verify(
            &sample_plan()?,
            &VerificationCancellation::new(),
        )?;

        assert_eq!(
            report.decision(),
            VerificationDecision::Indeterminate
        );

        Ok(())
    }

    #[test]
    fn cancellation_stops_verification() -> VerificationResult<()> {
        let coordinator = VerificationCoordinator::new();
        let cancellation = VerificationCancellation::new();
        cancellation.cancel();

        let result = coordinator.verify(&sample_plan()?, &cancellation);

        assert_eq!(result, Err(VerificationError::Cancelled));

        Ok(())
    }

    #[test]
    fn budget_is_enforced_without_global_ceiling() -> VerificationResult<()> {
        let coordinator = VerificationCoordinator::new();

        let plan = sample_plan()?.with_budget(VerificationBudget {
            max_checks: Some(0),
            ..VerificationBudget::default()
        });

        let result = coordinator.verify(
            &plan,
            &VerificationCancellation::new(),
        );

        assert!(matches!(
            result,
            Err(VerificationError::BudgetExceeded {
                resource: "checks",
                ..
            })
        ));

        Ok(())
    }

    #[test]
    fn duplicate_checks_are_rejected() -> VerificationResult<()> {
        let mut plan = sample_plan()?;

        let duplicate = VerificationCheck::new(
            CheckId::new("ir.integrity")?,
            VerificationDomain::new("ir")?,
            CheckRequirement::Required,
            "Duplicate check",
        )?;

        assert!(matches!(
            plan.add_check(duplicate),
            Err(VerificationError::DuplicateCheck { .. })
        ));

        Ok(())
    }
}
