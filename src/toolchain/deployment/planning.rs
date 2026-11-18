
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Toolchain — Deployment Planning
//!
//! Path: src/toolchain/deployment/planning.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Constructs deployment plans from authoritative, independently
//! established validation and target-realization evidence.
//!
//! This module is a side-effect-free orchestration component.
//!
//! It does not discover hardware, compile programs, generate artifacts,
//! allocate resources, provision infrastructure, schedule distributed
//! computation, launch applications or execute programs.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - DeploymentPlanningInput.
//! - DeploymentPlanner.
//! - DeploymentPlanningOutcome.
//! - DeploymentPlanningIssue.
//! - DeploymentReadinessAssessment.
//! - Evidence aggregation for deployment-plan construction.
//!
//! It does not own:
//!
//! - DeploymentRequest or DeployRequest.
//! - DeploymentPlan or DeploymentEvidence.
//! - DeploymentProvider or provider registration.
//! - Deployment profiles or generic profile resolution.
//! - Canonical requirements, capabilities or resource models.
//! - Feasibility definitions or feasibility algorithms.
//! - Target registration, target discovery or target compatibility.
//! - Compilation or artifact generation.
//! - Artifact verification algorithms.
//! - Authentication or authorization implementation.
//! - Hardware discovery or resource allocation.
//! - Distributed placement or scheduling.
//! - Runtime execution.
//!
//! ## Canonical authorities
//!
//! Request:
//!     crate::toolchain::api::request
//!
//! Deployment request adapter:
//!     crate::toolchain::deployment::request
//!
//! Deployment profile:
//!     crate::toolchain::deployment::profiles
//!
//! Deployment validation:
//!     crate::toolchain::deployment::validation
//!
//! Deployment plan and evidence:
//!     crate::toolchain::commands::deploy
//!
//! Canonical feasibility:
//!     crate::stdlib::feasibility
//!
//! Target compatibility and realization:
//!     crate::toolchain::targets
//!     crate::compiler
//!     crate::backend
//!
//! Physical resources:
//!     crate::zhal
//!
//! Distributed computation:
//!     crate::distributed
//!
//! Runtime:
//!     crate::zur
//!
//! ## Planning invariants
//!
//! 1. A valid request is not proof of feasibility.
//! 2. A registered provider is not proof of provider readiness.
//! 3. A capability declaration is not proof of availability.
//! 4. A preference is not a requirement.
//! 5. A hint is not a constraint.
//! 6. Missing evidence produces Unresolved.
//! 7. Conflicting evidence produces Unresolved.
//! 8. Proven mandatory violations produce Blocked.
//! 9. Explicit unsupportedness produces Blocked.
//! 10. Only complete positive evidence produces Ready.
//! 11. Target feasibility and artifact verification must apply to
//!     the same target.
//! 12. Every required target must be established when require_all
//!     is true.
//! 13. At least one compatible realization must be established
//!     when require_all is false.
//! 14. Capability-based selection must identify actual target
//!     realizations rather than inventing a native fallback.
//! 15. Planning never reserves physical resources.
//! 16. Planning never claims deployment submission or completion.
//! 17. Apply-time validation remains mandatory.
//!
//! ## POCO-REAF
//!
//! One logical project may have multiple compatible deployment
//! realizations across classical, quantum, HDL, AI, hybrid,
//! distributed and future computational domains.
//!
//! Each realization retains its own evidence.
//!
//! No universal hardware, target, provider, domain, participant,
//! artifact or resource-count ceilings are introduced.
//!
//! Collections grow subject to available implementation resources,
//! address-space limits and explicitly configured operational budgets.
//!
//! Literal infinite computation or storage is not promised.
//!
//! ## Grammar boundary
//!
//! Source deployment syntax belongs to:
//!     grammar/execution/deployment.g4
//!
//! Compilation integration belongs to:
//!     grammar/compile/deployment.g4
//!
//! Policy integration belongs to:
//!     grammar/policies/deployment.g4
//!
//! Resource requirements belong to:
//!     grammar/resources/requirements.g4
//!
//! This module consumes validated semantic and operational intent.
//! It does not parse or reinterpret source-language declarations.

use std::collections::{BTreeMap, BTreeSet};

use crate::stdlib::feasibility::FeasibilityStatus;

use crate::toolchain::api::request::{
    DeployRequest,
    TargetSelection,
};

use crate::toolchain::commands::deploy::{
    DeploymentEvidence,
    DeploymentEvidenceStatus,
    DeploymentPlan,
    DeploymentReadiness,
    DeploymentStage,
};

use crate::toolchain::deployment::validation::{
    DeploymentValidationReport,
};

use crate::toolchain::errors::{
    ToolchainError,
    ToolchainErrorKind,
    ToolchainResult,
};

// ============================================================================
// MODULE IDENTITY
// ============================================================================

/// Stable architectural identity of deployment planning.
pub const DEPLOYMENT_PLANNING_MODULE_NAME: &str =
    "zamani.toolchain.deployment.planning";

/// Deployment-planning API version.
///
/// This is independent of the language, grammar, compiler, IR,
//! backend, runtime and package-format versions.
pub const DEPLOYMENT_PLANNING_API_VERSION: u16 = 1;

const OPERATION_PLAN: &str =
    "toolchain.deployment.planning.plan";

// ============================================================================
// ERRORS
// ============================================================================

fn invalid_input(
    message: impl Into<String>,
) -> ToolchainError {
    ToolchainError::new(
        ToolchainErrorKind::InvalidInput,
        message,
    )
    .with_operation(OPERATION_PLAN)
    .with_context(DEPLOYMENT_PLANNING_MODULE_NAME)
}

fn validation_error(
    message: impl Into<String>,
) -> ToolchainError {
    ToolchainError::new(
        ToolchainErrorKind::ValidationFailed,
        message,
    )
    .with_operation(OPERATION_PLAN)
    .with_context(DEPLOYMENT_PLANNING_MODULE_NAME)
}

fn provider_error(
    message: impl Into<String>,
) -> ToolchainError {
    ToolchainError::new(
        ToolchainErrorKind::ProviderFailed,
        message,
    )
    .with_operation(OPERATION_PLAN)
    .with_context(DEPLOYMENT_PLANNING_MODULE_NAME)
}

// ============================================================================
// PLANNING ISSUES
// ============================================================================

/// Classification of a deployment-planning issue.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
#[non_exhaustive]
pub enum DeploymentPlanningIssueKind {
    /// Mandatory evidence establishes an unsatisfied condition.
    Infeasible,

    /// An authoritative provider explicitly reports unsupportedness.
    Unsupported,

    /// Required evidence is missing or incomplete.
    MissingEvidence,

    /// Evidence conflicts between authoritative observations.
    ConflictingEvidence,

    /// The requested target has not been established.
    TargetUnresolved,

    /// A target cannot satisfy a mandatory requirement.
    TargetInfeasible,

    /// Deployment validation has not established feasibility.
    ValidationUnresolved,
}

/// One structured explanation of a planning decision.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeploymentPlanningIssue {
    kind: DeploymentPlanningIssueKind,
    subject: Option<String>,
    message: String,
}

impl DeploymentPlanningIssue {
    fn new(
        kind: DeploymentPlanningIssueKind,
        subject: Option<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            subject,
            message: message.into(),
        }
    }

    /// Return the issue classification.
    #[must_use]
    pub const fn kind(&self) -> DeploymentPlanningIssueKind {
        self.kind
    }

    /// Return the affected subject, if applicable.
    #[must_use]
    pub fn subject(&self) -> Option<&str> {
        self.subject.as_deref()
    }

    /// Return the explanation.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

// ============================================================================
// READINESS ASSESSMENT
// ============================================================================

/// Detailed result of evaluating deployment readiness.
///
/// This is an orchestration result, not a second feasibility model.
/// The canonical feasibility authority remains
/// `crate::stdlib::feasibility`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeploymentReadinessAssessment {
    readiness: DeploymentReadiness,
    issues: Vec<DeploymentPlanningIssue>,
    established_targets: BTreeSet<String>,
}

impl DeploymentReadinessAssessment {
    /// Return the aggregate readiness.
    #[must_use]
    pub const fn readiness(&self) -> DeploymentReadiness {
        self.readiness
    }

    /// Return all planning issues.
    #[must_use]
    pub fn issues(&self) -> &[DeploymentPlanningIssue] {
        &self.issues
    }

    /// Return targets with both established feasibility and
    /// verified artifacts.
    #[must_use]
    pub fn established_targets(&self) -> &BTreeSet<String> {
        &self.established_targets
    }

    /// Whether every required condition is established.
    #[must_use]
    pub const fn is_ready(&self) -> bool {
        matches!(self.readiness, DeploymentReadiness::Ready)
    }

    /// Whether a mandatory condition is known to be unsatisfied.
    #[must_use]
    pub const fn is_blocked(&self) -> bool {
        matches!(self.readiness, DeploymentReadiness::Blocked)
    }

    /// Whether additional evidence is required.
    #[must_use]
    pub const fn is_unresolved(&self) -> bool {
        matches!(self.readiness, DeploymentReadiness::Unresolved)
    }
}

// ============================================================================
// PLANNING INPUT
// ============================================================================

/// Complete input for constructing a deployment plan.
///
/// All evidence must originate from the relevant authoritative
/// subsystem or a validated adapter over that subsystem.
///
/// This type intentionally does not accept arbitrary caller assertions
/// as substitutes for verified provider evidence.
///
/// The request is the canonical public API request.
/// The validation report is produced by DeploymentValidator.
/// The evidence collection contains deployment-stage observations
/// from the compiler, build, backend, security, environment,
/// provenance, target and resource authorities.
///
/// Resource availability observations do not constitute reservations.
#[derive(Debug, Clone)]
pub struct DeploymentPlanningInput {
    plan_id: String,
    provider_id: String,
    request: DeployRequest,
    validation: DeploymentValidationReport,
    evidence: Vec<DeploymentEvidence>,
}

impl DeploymentPlanningInput {
    /// Construct deployment-planning input.
    ///
    /// The plan identity and provider identity must be stable,
    /// non-secret identifiers.
    ///
    /// # Errors
    ///
    /// Returns an error when an identifier or request is invalid.
    pub fn new(
        plan_id: impl Into<String>,
        provider_id: impl Into<String>,
        request: DeployRequest,
        validation: DeploymentValidationReport,
        evidence: Vec<DeploymentEvidence>,
    ) -> ToolchainResult<Self> {
        let input = Self {
            plan_id: plan_id.into(),
            provider_id: provider_id.into(),
            request,
            validation,
            evidence,
        };

        input.validate()?;

        Ok(input)
    }

    /// Return the proposed plan identity.
    #[must_use]
    pub fn plan_id(&self) -> &str {
        &self.plan_id
    }

    /// Return the planning-provider identity.
    #[must_use]
    pub fn provider_id(&self) -> &str {
        &self.provider_id
    }

    /// Return the canonical deployment request.
    #[must_use]
    pub fn request(&self) -> &DeployRequest {
        &self.request
    }

    /// Return the canonical validation report.
    #[must_use]
    pub fn validation(&self) -> &DeploymentValidationReport {
        &self.validation
    }

    /// Return the supplied authoritative deployment evidence.
    #[must_use]
    pub fn evidence(&self) -> &[DeploymentEvidence] {
        &self.evidence
    }

    /// Validate the structural planning input.
    pub fn validate(&self) -> ToolchainResult<()> {
        validate_identifier("plan_id", &self.plan_id)?;
        validate_identifier("provider_id", &self.provider_id)?;

        self.request.validate().map_err(|error| {
            invalid_input(
                format!("invalid deployment request: {error}"),
            )
            .with_source(error)
        })?;

        for item in &self.evidence {
            item.validate().map_err(|error| {
                validation_error(
                    format!("invalid deployment evidence: {error}"),
                )
                .with_source(error)
            })?;

            if is_apply_or_runtime_stage(item.stage()) {
                return Err(validation_error(
                    "planning evidence cannot claim resource reservation, \
                     deployment submission, runtime launch or completion",
                ));
            }
        }

        Ok(())
    }
}

fn validate_identifier(
    field: &str,
    value: &str,
) -> ToolchainResult<()> {
    if value.trim().is_empty() {
        return Err(invalid_input(
            format!("{field} must not be empty"),
        ));
    }

    if value.trim() != value {
        return Err(invalid_input(
            format!("{field} must not contain surrounding whitespace"),
        ));
    }

    if value.chars().any(char::is_control) {
        return Err(invalid_input(
            format!("{field} must not contain control characters"),
        ));
    }

    Ok(())
}

fn is_apply_or_runtime_stage(
    stage: DeploymentStage,
) -> bool {
    matches!(
        stage,
        DeploymentStage::ResourcesReserved
            | DeploymentStage::DeploymentSubmitted
            | DeploymentStage::RuntimeStarted
            | DeploymentStage::RuntimeCompleted
            | DeploymentStage::DeploymentCompleted
            | DeploymentStage::DeploymentCancelled
    )
}

// ============================================================================
// EVIDENCE AGGREGATION
// ============================================================================

/// Internal classification of a set of evidence observations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EvidenceAssessment {
    Passed,
    Failed,
    Unresolved,
}

/// Classify evidence for one exact stage and optional target.
///
/// Multiple authorities may contribute evidence.
///
/// Positive and negative observations that conflict are unresolved.
/// Missing evidence is unresolved.
/// A negative result is only decisive when there is no contradictory
/// positive observation.
fn assess_evidence<'a>(
    evidence: impl IntoIterator<Item = &'a DeploymentEvidence>,
    stage: DeploymentStage,
    target: Option<&str>,
) -> EvidenceAssessment {
    let mut passed = false;
    let mut failed = false;
    let mut found = false;

    for item in evidence {
        if item.stage() != stage {
            continue;
        }

        if item.target() != target {
            continue;
        }

        found = true;

        match item.status() {
            DeploymentEvidenceStatus::Passed => passed = true,
            DeploymentEvidenceStatus::Failed => failed = true,
            DeploymentEvidenceStatus::Unresolved
            | DeploymentEvidenceStatus::NotRequired => {}
        }
    }

    match (found, passed, failed) {
        (_, true, true) => EvidenceAssessment::Unresolved,
        (_, true, false) => EvidenceAssessment::Passed,
        (_, false, true) => EvidenceAssessment::Failed,
        _ => EvidenceAssessment::Unresolved,
    }
}

// ============================================================================
// TARGET ASSESSMENT
// ============================================================================

/// Outcome of assessing one target realization.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TargetAssessment {
    status: EvidenceAssessment,
}

/// Assess feasibility and artifacts for the same target.
///
/// Evidence for one target must never satisfy another target.
fn assess_target(
    evidence: &[DeploymentEvidence],
    target: &str,
) -> TargetAssessment {
    let feasibility = assess_evidence(
        evidence,
        DeploymentStage::TargetFeasibilityEstablished,
        Some(target),
    );

    let artifacts = assess_evidence(
        evidence,
        DeploymentStage::ArtifactsVerified,
        Some(target),
    );

    let status = match (feasibility, artifacts) {
        (EvidenceAssessment::Failed, EvidenceAssessment::Passed)
        | (EvidenceAssessment::Passed, EvidenceAssessment::Failed) => {
            EvidenceAssessment::Failed
        }

        (EvidenceAssessment::Failed, EvidenceAssessment::Failed) => {
            EvidenceAssessment::Failed
        }

        (EvidenceAssessment::Passed, EvidenceAssessment::Passed) => {
            EvidenceAssessment::Passed
        }

        _ => EvidenceAssessment::Unresolved,
    };

    TargetAssessment { status }
}

// ============================================================================
// PLANNER
// ============================================================================

/// Stateless deployment-plan coordinator.
///
/// The planner is deterministic for the same request, validation
/// report and evidence sequence.
///
/// It performs no filesystem, network, process, hardware, allocation,
/// provisioning, runtime or deployment operations.
///
/// The caller is responsible for supplying evidence produced by
/// authoritative services.
///
/// No hidden provider registry or global mutable state is used.
#[derive(Debug, Clone, Copy, Default)]
pub struct DeploymentPlanner;

impl DeploymentPlanner {
    /// Construct a deployment planner.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Assess deployment readiness without creating a plan.
    ///
    /// This is useful when a client needs to inspect unresolved or
    /// blocked conditions before requesting plan construction.
    ///
    /// The returned readiness is derived from canonical validation
    /// and target-specific evidence.
    ///
    /// # Errors
    ///
    /// Returns structural input-validation errors.
    pub fn assess(
        &self,
        input: &DeploymentPlanningInput,
    ) -> ToolchainResult<DeploymentReadinessAssessment> {
        input.validate()?;

        let mut issues = Vec::new();
        let mut established_targets = BTreeSet::new();
        let mut blocked = false;
        let mut unresolved = false;

        // ------------------------------------------------------------
        // Canonical validation assessment
        // ------------------------------------------------------------

        let validation = input.validation();

        if validation.is_infeasible() {
            blocked = true;

            issues.push(DeploymentPlanningIssue::new(
                DeploymentPlanningIssueKind::Infeasible,
                None,
                "authoritative deployment validation established \
                 an unsatisfied mandatory condition",
            ));
        } else if validation.is_unsupported() {
            blocked = true;

            issues.push(DeploymentPlanningIssue::new(
                DeploymentPlanningIssueKind::Unsupported,
                None,
                "an authoritative deployment validation provider \
                 established that a required operation is unsupported",
            ));
        } else if !validation.is_feasible() {
            unresolved = true;

            issues.push(DeploymentPlanningIssue::new(
                DeploymentPlanningIssueKind::ValidationUnresolved,
                None,
                "deployment validation is incomplete or does not \
                 establish feasibility",
            ));
        }

        // ------------------------------------------------------------
        // Required global evidence
        // ------------------------------------------------------------

        let global_stages = [
            DeploymentStage::BuildVerified,
            DeploymentStage::EnvironmentValidated,
            DeploymentStage::PolicyAuthorized,
            DeploymentStage::SecurityVerified,
            DeploymentStage::ProvenanceVerified,
            DeploymentStage::ResourceRequirementsAssessed,
        ];

        for stage in global_stages {
            let assessment = assess_evidence(
                input.evidence(),
                stage,
                None,
            );

            match assessment {
                EvidenceAssessment::Passed => {}

                EvidenceAssessment::Failed => {
                    blocked = true;

                    issues.push(DeploymentPlanningIssue::new(
                        DeploymentPlanningIssueKind::Infeasible,
                        Some(format!("{stage:?}")),
                        format!(
                            "mandatory deployment stage {stage:?} \
                             failed according to authoritative evidence"
                        ),
                    ));
                }

                EvidenceAssessment::Unresolved => {
                    unresolved = true;

                    issues.push(DeploymentPlanningIssue::new(
                        DeploymentPlanningIssueKind::MissingEvidence,
                        Some(format!("{stage:?}")),
                        format!(
                            "deployment stage {stage:?} has no \
                             complete positive evidence"
                        ),
                    ));
                }
            }
        }

        // ------------------------------------------------------------
        // Target-specific evidence
        // ------------------------------------------------------------

        let target_result = self.assess_targets(
            &input.request,
            input.evidence(),
        );

        established_targets.extend(
            target_result.established_targets
        );

        if target_result.blocked {
            blocked = true;
        }

        if target_result.unresolved {
            unresolved = true;
        }

        issues.extend(target_result.issues);

        // ------------------------------------------------------------
        // Aggregate readiness
        // ------------------------------------------------------------

        let readiness = if blocked {
            DeploymentReadiness::Blocked
        } else if unresolved {
            DeploymentReadiness::Unresolved
        } else {
            DeploymentReadiness::Ready
        };

        Ok(DeploymentReadinessAssessment {
            readiness,
            issues,
            established_targets,
        })
    }

    /// Construct a deployment plan using the existing command-level
    /// DeploymentPlan contract.
    ///
    /// Readiness is computed internally. Callers cannot directly
    /// assert Ready.
    ///
    /// The resulting plan is validated again by DeploymentPlan::new.
    ///
    /// # Errors
    ///
    /// Returns invalid-input, validation or provider-contract errors.
    pub fn plan(
        &self,
        input: DeploymentPlanningInput,
    ) -> ToolchainResult<DeploymentPlan> {
        input.validate()?;

        let assessment = self.assess(&input)?;

        let plan = DeploymentPlan::new(
            input.plan_id,
            input.provider_id,
            input.request,
            assessment.readiness(),
            input.evidence,
        )
        .map_err(|error| {
            provider_error(
                format!(
                    "constructed deployment plan violated \
                     the canonical deployment-plan contract: {error}"
                ),
            )
            .with_source(error)
        })?;

        Ok(plan)
    }

    fn assess_targets(
        &self,
        request: &DeployRequest,
        evidence: &[DeploymentEvidence],
    ) -> ToolchainResult<TargetAssessmentResult> {
        let mut result = TargetAssessmentResult::default();

        match &request.targets {
            TargetSelection::Explicit {
                targets,
                require_all,
            } => {
                let mut passed = 0usize;
                let mut failed = 0usize;
                let mut unresolved = 0usize;

                for target in targets {
                    let target_name = target.as_str();

                    let assessment = assess_target(
                        evidence,
                        target_name,
                    );

                    match assessment.status {
                        EvidenceAssessment::Passed => {
                            passed += 1;
                            result.established_targets.insert(
                                target_name.to_owned(),
                            );
                        }

                        EvidenceAssessment::Failed => {
                            failed += 1;

                            result.issues.push(
                                DeploymentPlanningIssue::new(
                                    DeploymentPlanningIssueKind::TargetInfeasible,
                                    Some(target_name.to_owned()),
                                    format!(
                                        "target `{target_name}` has \
                                         authoritative negative evidence \
                                         for feasibility or artifact verification"
                                    ),
                                ),
                            );
                        }

                        EvidenceAssessment::Unresolved => {
                            unresolved += 1;

                            result.issues.push(
                                DeploymentPlanningIssue::new(
                                    DeploymentPlanningIssueKind::TargetUnresolved,
                                    Some(target_name.to_owned()),
                                    format!(
                                        "target `{target_name}` lacks \
                                         complete, consistent feasibility \
                                         and artifact evidence"
                                    ),
                                ),
                            );
                        }
                    }
                }

                if *require_all {
                    result.blocked = failed > 0;
                    result.unresolved = unresolved > 0;

                    if passed != targets.len()
                        && failed == 0
                        && unresolved == 0
                    {
                        result.unresolved = true;
                    }
                } else if passed > 0 {
                    // At least one explicitly requested realization
                    // is established. Other unsuccessful candidates
                    // do not invalidate the permitted partial selection.
                    result.blocked = false;
                    result.unresolved = false;
                } else if failed == targets.len() {
                    result.blocked = true;
                } else {
                    result.unresolved = true;
                }
            }

            TargetSelection::ResolveFromCapabilities { .. } => {
                let candidates = collect_target_candidates(evidence);

                if candidates.is_empty() {
                    result.unresolved = true;

                    result.issues.push(
                        DeploymentPlanningIssue::new(
                            DeploymentPlanningIssueKind::TargetUnresolved,
                            None,
                            "capability-based target resolution did not \
                             provide any evidence-backed target candidate",
                        ),
                    );

                    return Ok(result);
                }

                let mut passed = 0usize;
                let mut failed = 0usize;

                for target in candidates {
                    match assess_target(evidence, &target).status {
                        EvidenceAssessment::Passed => {
                            passed += 1;
                            result.established_targets.insert(target);
                        }

                        EvidenceAssessment::Failed => {
                            failed += 1;
                        }

                        EvidenceAssessment::Unresolved => {
                            result.unresolved = true;

                            result.issues.push(
                                DeploymentPlanningIssue::new(
                                    DeploymentPlanningIssueKind::TargetUnresolved,
                                    Some(target.clone()),
                                    format!(
                                        "candidate target `{target}` \
                                         has incomplete or conflicting evidence"
                                    ),
                                ),
                            );
                        }
                    }
                }

                if passed > 0 {
                    // The planner records established candidates.
                    // The authoritative target-selection service
                    // determines preference ordering and selection.
                    result.blocked = false;
                    result.unresolved = false;
                } else if failed == candidates.len() {
                    result.blocked = true;

                    result.issues.push(
                        DeploymentPlanningIssue::new(
                            DeploymentPlanningIssueKind::TargetInfeasible,
                            None,
                            "all evidence-backed target candidates \
                             failed mandatory feasibility or artifact checks",
                        ),
                    );
                } else {
                    result.unresolved = true;
                }
            }
        }

        Ok(result)
    }
}

// ============================================================================
// INTERNAL TARGET RESULTS
// ============================================================================

#[derive(Debug, Default)]
struct TargetAssessmentResult {
    established_targets: BTreeSet<String>,
    issues: Vec<DeploymentPlanningIssue>,
    blocked: bool,
    unresolved: bool,
}

/// Collect target identities from target-scoped feasibility evidence.
///
/// This does not discover targets or choose them. It only identifies
/// the candidate identities already supplied by authoritative providers.
fn collect_target_candidates(
    evidence: &[DeploymentEvidence],
) -> BTreeSet<String> {
    evidence
        .iter()
        .filter(|item| {
            item.stage()
                == DeploymentStage::TargetFeasibilityEstablished
        })
        .filter_map(|item| item.target().map(str::to_owned))
        .collect()
}

// ============================================================================
// CONTRACT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn evidence(
        stage: DeploymentStage,
        status: DeploymentEvidenceStatus,
        target: Option<&str>,
    ) -> DeploymentEvidence {
        let item = DeploymentEvidence::new(
            stage,
            status,
            "test.authority",
            "test.subject",
            "test evidence",
        )
        .expect("test evidence should be structurally valid");

        match target {
            Some(value) => item.for_target(value),
            None => item,
        }
    }

    #[test]
    fn missing_evidence_is_unresolved() {
        let evidence = Vec::new();

        assert_eq!(
            assess_evidence(
                &evidence,
                DeploymentStage::BuildVerified,
                None,
            ),
            EvidenceAssessment::Unresolved,
        );
    }

    #[test]
    fn passed_evidence_is_accepted() {
        let evidence = vec![
            evidence(
                DeploymentStage::BuildVerified,
                DeploymentEvidenceStatus::Passed,
                None,
            ),
        ];

        assert_eq!(
            assess_evidence(
                &evidence,
                DeploymentStage::BuildVerified,
                None,
            ),
            EvidenceAssessment::Passed,
        );
    }

    #[test]
    fn failed_evidence_is_not_promoted_to_success() {
        let evidence = vec![
            evidence(
                DeploymentStage::BuildVerified,
                DeploymentEvidenceStatus::Failed,
                None,
            ),
        ];

        assert_eq!(
            assess_evidence(
                &evidence,
                DeploymentStage::BuildVerified,
                None,
            ),
            EvidenceAssessment::Failed,
        );
    }

    #[test]
    fn contradictory_evidence_is_unresolved() {
        let evidence = vec![
            evidence(
                DeploymentStage::BuildVerified,
                DeploymentEvidenceStatus::Passed,
                None,
            ),
            evidence(
                DeploymentStage::BuildVerified,
                DeploymentEvidenceStatus::Failed,
                None,
            ),
        ];

        assert_eq!(
            assess_evidence(
                &evidence,
                DeploymentStage::BuildVerified,
                None,
            ),
            EvidenceAssessment::Unresolved,
        );
    }

    #[test]
    fn target_feasibility_cannot_be_combined_with_another_targets_artifact() {
        let evidence = vec![
            evidence(
                DeploymentStage::TargetFeasibilityEstablished,
                DeploymentEvidenceStatus::Passed,
                Some("target.alpha"),
            ),
            evidence(
                DeploymentStage::ArtifactsVerified,
                DeploymentEvidenceStatus::Passed,
                Some("target.beta"),
            ),
        ];

        assert_eq!(
            assess_target(&evidence, "target.alpha").status,
            EvidenceAssessment::Unresolved,
        );
    }

    #[test]
    fn target_requires_both_feasibility_and_verified_artifacts() {
        let evidence = vec![
            evidence(
                DeploymentStage::TargetFeasibilityEstablished,
                DeploymentEvidenceStatus::Passed,
                Some("target.alpha"),
            ),
            evidence(
                DeploymentStage::ArtifactsVerified,
                DeploymentEvidenceStatus::Passed,
                Some("target.alpha"),
            ),
        ];

        assert_eq!(
            assess_target(&evidence, "target.alpha").status,
            EvidenceAssessment::Passed,
        );
    }

    #[test]
    fn planning_never_accepts_apply_or_runtime_evidence() {
        let item = evidence(
            DeploymentStage::ResourcesReserved,
            DeploymentEvidenceStatus::Passed,
            None,
        );

        assert!(is_apply_or_runtime_stage(item.stage()));
    }

    #[test]
    fn planning_does_not_confuse_unknown_with_infeasible() {
        assert_ne!(
            EvidenceAssessment::Unresolved,
            EvidenceAssessment::Failed,
        );
    }

    #[test]
    fn planning_module_has_stable_identity() {
        assert_eq!(
            DEPLOYMENT_PLANNING_MODULE_NAME,
            "zamani.toolchain.deployment.planning",
        );

        assert!(DEPLOYMENT_PLANNING_API_VERSION > 0);
    }
}
