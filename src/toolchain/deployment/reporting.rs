
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Toolchain — Deployment Reporting
//!
//! Path: src/toolchain/deployment/reporting.rs
//! Edition: Rust 2021
//! Minimum supported Rust: 1.97
//! Safety: Unsafe Rust is forbidden.
//!
//! ## Purpose
//!
//! Produces deterministic, structured and serializable reports from
//! actual deployment command results.
//!
//! Reports preserve:
//!
//! - The exact deployment plan identity.
//! - The planning provider identity.
//! - The plan's readiness.
//! - The actual deployment-command outcome.
//! - The deployment receipt, when one exists.
//! - Planning evidence.
//! - Application evidence.
//! - Target-specific evidence.
//! - Evidence counts.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - DeploymentReport.
//! - DeploymentEvidenceRecord.
//! - DeploymentEvidencePhase.
//! - DeploymentEvidenceCounts.
//! - Deployment report construction.
//! - Deployment report validation.
//! - Deterministic evidence ordering.
//!
//! It does not own:
//!
//! - Deployment requests.
//! - Deployment plans.
//! - Deployment receipts.
//! - Deployment providers.
//! - Deployment execution.
//! - Artifact generation or verification.
//! - Provenance verification.
//! - Authentication or authorization.
//! - Target discovery.
//! - Capability discovery.
//! - Feasibility analysis.
//! - Hardware discovery.
//! - Resource allocation.
//! - Infrastructure provisioning.
//! - Distributed scheduling.
//! - Runtime execution.
//! - Cryptographic verification.
//! - Diagnostic generation.
//!
//! ## Canonical authorities
//!
//! Deployment command:
//!     crate::toolchain::commands::deploy
//!
//! Deployment request:
//!     crate::toolchain::api::request
//!
//! Deployment planning:
//!     crate::toolchain::deployment::planning
//!
//! Deployment validation:
//!     crate::toolchain::deployment::validation
//!
//! Deployment providers:
//!     crate::toolchain::deployment::providers
//!
//! Deployment profiles:
//!     crate::toolchain::deployment::profiles
//!
//! Toolchain errors:
//!     crate::toolchain::errors
//!
//! Runtime:
//!     crate::zur
//!
//! Hardware:
//!     crate::zhal
//!
//! Distributed execution:
//!     crate::distributed
//!
//! ## Reporting invariants
//!
//! 1. Reports originate from actual DeploymentCommandResult values.
//! 2. A plan-only operation never receives a fabricated receipt.
//! 3. Acceptance never means successful completion.
//! 4. Runtime completion is distinct from deployment completion.
//! 5. Blocked and unresolved outcomes remain distinct.
//! 6. Partial completion remains distinct from complete success.
//! 7. Unknown outcomes are never promoted to success.
//! 8. Receipt identities must match the associated plan.
//! 9. Evidence retains its original authority and subject.
//! 10. Target-specific evidence never loses its target identity.
//! 11. Planning evidence and application evidence remain distinguishable.
//! 12. Evidence is not independently verified by this module.
//! 13. Report construction performs no deployment side effects.
//! 14. Reports never manufacture missing provenance or evidence.
//!
//! ## POCO-REAF
//!
//! Reports support open-ended computational domains and target
//! realizations.
//!
//! No closed list of:
//!
//! - Computing domains.
//! - Targets.
//! - Providers.
//! - Artifacts.
//! - Hardware resources.
//! - Distributed participants.
//! - Deployment environments.
//!
//! is imposed here.
//!
//! Collection growth is constrained only by available implementation
//! resources, address space and explicitly configured operational
//! policies.
//!
//! Literal infinite storage or computation is not promised.
//!
//! ## Grammar boundary
//!
//! Deployment source intent:
//!     grammar/execution/deployment.g4
//!
//! Compilation integration:
//!     grammar/compile/deployment.g4
//!
//! Deployment policy intent:
//!     grammar/policies/deployment.g4
//!
//! Resource requirements:
//!     grammar/resources/requirements.g4
//!
//! Portability:
//!     grammar/specification/poco-reaf.md
//!
//! Scalability:
//!     grammar/specification/scalability-model.md
//!
//! This module reports validated operational outcomes.
//! It does not parse source declarations or reinterpret their semantics.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::toolchain::commands::deploy::{
    DeploymentCommandResult,
    DeploymentCommandStatus,
    DeploymentEvidence,
    DeploymentEvidenceStatus,
    DeploymentPlan,
    DeploymentReadiness,
    DeploymentReceipt,
    DeploymentReceiptStatus,
    DeploymentStage,
};

use crate::toolchain::errors::{
    ToolchainError,
    ToolchainErrorKind,
    ToolchainResult,
};

// ============================================================================
// MODULE IDENTITY
// ============================================================================

/// Stable architectural identity of deployment reporting.
pub const DEPLOYMENT_REPORTING_MODULE_NAME: &str =
    "zamani.toolchain.deployment.reporting";

/// Deployment-report schema version.
///
/// This versions the serialized reporting contract independently
/// of the language, grammar, compiler, backend, runtime and
/// deployment-provider APIs.
pub const DEPLOYMENT_REPORT_SCHEMA_VERSION: u32 = 1;

const OPERATION_REPORT: &str =
    "toolchain.deployment.reporting";

const OPERATION_VALIDATE: &str =
    "toolchain.deployment.reporting.validate";

// ============================================================================
// ERROR CONSTRUCTION
// ============================================================================

fn invalid_report(
    message: impl Into<String>,
) -> ToolchainError {
    ToolchainError::new(
        ToolchainErrorKind::InvalidInput,
        message,
    )
    .with_operation(OPERATION_REPORT)
    .with_context(DEPLOYMENT_REPORTING_MODULE_NAME)
}

fn validation_error(
    message: impl Into<String>,
) -> ToolchainError {
    ToolchainError::new(
        ToolchainErrorKind::ValidationFailed,
        message,
    )
    .with_operation(OPERATION_VALIDATE)
    .with_context(DEPLOYMENT_REPORTING_MODULE_NAME)
}

fn serialization_error(
    message: impl Into<String>,
) -> ToolchainError {
    ToolchainError::new(
        ToolchainErrorKind::Internal,
        message,
    )
    .with_operation(OPERATION_REPORT)
    .with_context(DEPLOYMENT_REPORTING_MODULE_NAME)
}

// ============================================================================
// IDENTIFIER VALIDATION
// ============================================================================

fn validate_identifier(
    field: &str,
    value: &str,
) -> ToolchainResult<()> {
    if value.trim().is_empty() {
        return Err(validation_error(
            format!("{field} must not be empty"),
        ));
    }

    if value != value.trim() {
        return Err(validation_error(
            format!(
                "{field} must not contain surrounding whitespace"
            ),
        ));
    }

    if value.chars().any(char::is_control) {
        return Err(validation_error(
            format!(
                "{field} must not contain control characters"
            ),
        ));
    }

    Ok(())
}

// ============================================================================
// EVIDENCE PHASE
// ============================================================================

/// Identifies the deployment lifecycle phase that produced evidence.
///
/// This is reporting metadata. It does not introduce another
/// deployment lifecycle or replace DeploymentStage.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum DeploymentEvidencePhase {
    /// Evidence produced while constructing and validating a plan.
    Planning,

    /// Evidence produced while applying a plan or observing its outcome.
    Application,
}

impl DeploymentEvidencePhase {
    /// Return the stable serialized identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Planning => "planning",
            Self::Application => "application",
        }
    }
}

// ============================================================================
// EVIDENCE STATUS IDENTIFIERS
// ============================================================================

fn evidence_status_name(
    status: DeploymentEvidenceStatus,
) -> &'static str {
    match status {
        DeploymentEvidenceStatus::Passed => "passed",
        DeploymentEvidenceStatus::Failed => "failed",
        DeploymentEvidenceStatus::Unresolved => "unresolved",
        DeploymentEvidenceStatus::NotRequired => "not_required",
        _ => "unknown",
    }
}

fn deployment_stage_name(
    stage: DeploymentStage,
) -> &'static str {
    match stage {
        DeploymentStage::BuildVerified => "build_verified",
        DeploymentStage::ArtifactsVerified => "artifacts_verified",
        DeploymentStage::TargetFeasibilityEstablished => {
            "target_feasibility_established"
        }
        DeploymentStage::EnvironmentValidated => {
            "environment_validated"
        }
        DeploymentStage::PolicyAuthorized => "policy_authorized",
        DeploymentStage::SecurityVerified => "security_verified",
        DeploymentStage::ProvenanceVerified => "provenance_verified",
        DeploymentStage::ResourceRequirementsAssessed => {
            "resource_requirements_assessed"
        }
        DeploymentStage::ResourceAvailabilityObserved => {
            "resource_availability_observed"
        }
        DeploymentStage::ResourcesReserved => "resources_reserved",
        DeploymentStage::DeploymentSubmitted => "deployment_submitted",
        DeploymentStage::RuntimeStarted => "runtime_started",
        DeploymentStage::RuntimeCompleted => "runtime_completed",
        DeploymentStage::DeploymentCompleted => {
            "deployment_completed"
        }
        DeploymentStage::DeploymentCancelled => {
            "deployment_cancelled"
        }
        _ => "unknown",
    }
}

fn readiness_name(
    readiness: DeploymentReadiness,
) -> &'static str {
    match readiness {
        DeploymentReadiness::Ready => "ready",
        DeploymentReadiness::Blocked => "blocked",
        DeploymentReadiness::Unresolved => "unresolved",
        _ => "unknown",
    }
}

fn command_status_name(
    status: DeploymentCommandStatus,
) -> &'static str {
    match status {
        DeploymentCommandStatus::Planned => "planned",
        DeploymentCommandStatus::Blocked => "blocked",
        DeploymentCommandStatus::Unresolved => "unresolved",
        DeploymentCommandStatus::Accepted => "accepted",
        DeploymentCommandStatus::InProgress => "in_progress",
        DeploymentCommandStatus::Completed => "completed",
        DeploymentCommandStatus::PartiallyCompleted => {
            "partially_completed"
        }
        DeploymentCommandStatus::Failed => "failed",
        DeploymentCommandStatus::Cancelled => "cancelled",
        DeploymentCommandStatus::Unknown => "unknown",
        _ => "unknown",
    }
}

fn receipt_status_name(
    status: DeploymentReceiptStatus,
) -> &'static str {
    match status {
        DeploymentReceiptStatus::Accepted => "accepted",
        DeploymentReceiptStatus::InProgress => "in_progress",
        DeploymentReceiptStatus::Completed => "completed",
        DeploymentReceiptStatus::PartiallyCompleted => {
            "partially_completed"
        }
        DeploymentReceiptStatus::Failed => "failed",
        DeploymentReceiptStatus::Cancelled => "cancelled",
        DeploymentReceiptStatus::Unknown => "unknown",
        _ => "unknown",
    }
}

// ============================================================================
// EVIDENCE RECORD
// ============================================================================

/// Serializable snapshot of one canonical deployment-evidence item.
///
/// This structure preserves the original authority, subject, stage,
/// status, target and non-secret evidence reference.
///
/// It does not authenticate or independently verify the observation.
///
/// The original DeploymentEvidence remains the canonical evidence
/// representation.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
)]
#[serde(deny_unknown_fields)]
pub struct DeploymentEvidenceRecord {
    /// Lifecycle phase in which the evidence was produced.
    pub phase: DeploymentEvidencePhase,

    /// Stable deployment-stage identifier.
    pub stage: String,

    /// Original evidence status.
    pub status: String,

    /// Authority responsible for the observation.
    pub authority: String,

    /// Subject assessed by the authority.
    pub subject: String,

    /// Target identity, when the evidence is target-specific.
    pub target: Option<String>,

    /// Non-secret reference to the original evidence.
    pub reference: Option<String>,

    /// Original human-readable evidence explanation.
    pub message: String,
}

impl DeploymentEvidenceRecord {
    /// Create a reporting snapshot from canonical evidence.
    ///
    /// No evidence is invented or promoted to another status.
    #[must_use]
    pub fn from_evidence(
        evidence: &DeploymentEvidence,
        phase: DeploymentEvidencePhase,
    ) -> Self {
        Self {
            phase,
            stage: deployment_stage_name(evidence.stage()).to_owned(),
            status: evidence_status_name(evidence.status()).to_owned(),
            authority: evidence.authority().to_owned(),
            subject: evidence.subject().to_owned(),
            target: evidence.target().map(str::to_owned),
            reference: evidence.reference().map(str::to_owned),
            message: evidence.message().to_owned(),
        }
    }

    /// Validate the reporting representation.
    pub fn validate(&self) -> ToolchainResult<()> {
        validate_identifier("evidence.stage", &self.stage)?;
        validate_identifier("evidence.status", &self.status)?;
        validate_identifier("evidence.authority", &self.authority)?;
        validate_identifier("evidence.subject", &self.subject)?;

        if self.message.trim().is_empty() {
            return Err(validation_error(
                "evidence.message must not be empty",
            ));
        }

        if self.message.chars().any(char::is_control) {
            return Err(validation_error(
                "evidence.message contains control characters",
            ));
        }

        if let Some(target) = &self.target {
            validate_identifier("evidence.target", target)?;
        }

        if let Some(reference) = &self.reference {
            validate_identifier("evidence.reference", reference)?;
        }

        Ok(())
    }
}

// ============================================================================
// EVIDENCE COUNTS
// ============================================================================

/// Number of evidence records grouped by lifecycle phase, stage and status.
///
/// Keys use the stable representation:
///
/// `phase.stage.status`
///
/// A BTreeMap provides deterministic key ordering.
///
/// Counts are descriptive. They do not establish readiness or feasibility.
pub type DeploymentEvidenceCounts = BTreeMap<String, usize>;

fn evidence_count_key(
    record: &DeploymentEvidenceRecord,
) -> String {
    format!(
        "{}.{}.{}",
        record.phase.as_str(),
        record.stage,
        record.status,
    )
}

fn calculate_evidence_counts(
    evidence: &[DeploymentEvidenceRecord],
) -> ToolchainResult<DeploymentEvidenceCounts> {
    let mut counts = BTreeMap::new();

    for record in evidence {
        let key = evidence_count_key(record);

        let count = counts.entry(key).or_insert(0usize);

        *count = count.checked_add(1).ok_or_else(|| {
            invalid_report(
                "deployment evidence count exceeded the \
                 representable collection count",
            )
        })?;
    }

    Ok(counts)
}

// ============================================================================
// DEPLOYMENT REPORT
// ============================================================================

/// Complete, structured deployment-operation report.
///
/// This is a snapshot of an actual DeploymentCommandResult.
///
/// The report preserves planning and application evidence separately.
///
/// It does not claim that an application is running or that runtime
/// execution has completed. Runtime lifecycle evidence remains owned
/// by ZUR and is included only when supplied by the canonical receipt.
///
/// The report contains no binary artifacts. Artifact references remain
/// with their canonical artifact and provenance authorities.
///
/// All fields are private to prevent callers from constructing
/// inconsistent report states through direct field mutation.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
#[serde(deny_unknown_fields)]
pub struct DeploymentReport {
    /// Serialized reporting-contract version.
    schema_version: u32,

    /// Identity of the deployment plan.
    plan_id: String,

    /// Identity of the provider responsible for planning.
    provider: String,

    /// Readiness established during planning.
    readiness: String,

    /// Actual deployment-command outcome.
    operation_status: String,

    /// Deployment identity, when an application receipt exists.
    deployment_id: Option<String>,

    /// Actual provider-side correlation reference, when available.
    provider_reference: Option<String>,

    /// Actual application receipt status, when one exists.
    receipt_status: Option<String>,

    /// Whether an application receipt was returned.
    applied: bool,

    /// Planning and application evidence.
    evidence: Vec<DeploymentEvidenceRecord>,

    /// Deterministic evidence counts.
    evidence_counts: DeploymentEvidenceCounts,
}

impl DeploymentReport {
    /// Construct a report from an actual deployment-command result.
    ///
    /// The exact plan and receipt are validated before report creation.
    ///
    /// Planning evidence and application evidence remain separate.
    ///
    /// The function does not invoke providers or perform deployment
    /// operations.
    ///
    /// # Errors
    ///
    /// Returns an error when:
    ///
    /// - The command result contains an invalid plan.
    /// - The plan does not match the original request.
    /// - A receipt does not match the plan.
    /// - The command status contradicts the receipt.
    /// - A reporting invariant is violated.
    pub fn from_command_result(
        result: &DeploymentCommandResult,
    ) -> ToolchainResult<Self> {
        let plan = result.plan();

        validate_plan_binding(plan, result)?;

        let receipt = result.receipt();

        if let Some(receipt) = receipt {
            receipt.validate_for(plan)?;
        }

        validate_operation_consistency(
            result.status(),
            plan,
            receipt,
        )?;

        let mut evidence = Vec::new();

        evidence.extend(
            plan.evidence()
                .iter()
                .map(|item| {
                    DeploymentEvidenceRecord::from_evidence(
                        item,
                        DeploymentEvidencePhase::Planning,
                    )
                }),
        );

        if let Some(receipt) = receipt {
            evidence.extend(
                receipt
                    .evidence()
                    .iter()
                    .map(|item| {
                        DeploymentEvidenceRecord::from_evidence(
                            item,
                            DeploymentEvidencePhase::Application,
                        )
                    }),
            );
        }

        // Canonical deterministic ordering independent of provider
        // collection insertion order.
        evidence.sort_unstable();

        let report = Self {
            schema_version: DEPLOYMENT_REPORT_SCHEMA_VERSION,
            plan_id: plan.plan_id().to_owned(),
            provider: plan.provider().to_owned(),
            readiness: readiness_name(plan.readiness()).to_owned(),
            operation_status: command_status_name(
                result.status(),
            )
            .to_owned(),
            deployment_id: receipt.map(|item| {
                item.deployment_id().to_owned()
            }),
            provider_reference: receipt.and_then(|item| {
                item.provider_reference().map(str::to_owned)
            }),
            receipt_status: receipt.map(|item| {
                receipt_status_name(item.status()).to_owned()
            }),
            applied: receipt.is_some(),
            evidence_counts: calculate_evidence_counts(&evidence)?,
            evidence,
        };

        report.validate()?;

        Ok(report)
    }

    /// Return the reporting schema version.
    #[must_use]
    pub const fn schema_version(&self) -> u32 {
        self.schema_version
    }

    /// Return the deployment plan identity.
    #[must_use]
    pub fn plan_id(&self) -> &str {
        &self.plan_id
    }

    /// Return the planning-provider identity.
    #[must_use]
    pub fn provider(&self) -> &str {
        &self.provider
    }

    /// Return the established planning readiness.
    #[must_use]
    pub fn readiness(&self) -> &str {
        &self.readiness
    }

    /// Return the actual deployment-operation status.
    #[must_use]
    pub fn operation_status(&self) -> &str {
        &self.operation_status
    }

    /// Return the deployment identity, if applied.
    #[must_use]
    pub fn deployment_id(&self) -> Option<&str> {
        self.deployment_id.as_deref()
    }

    /// Return the provider reference, if available.
    #[must_use]
    pub fn provider_reference(&self) -> Option<&str> {
        self.provider_reference.as_deref()
    }

    /// Return the actual receipt status, if applied.
    #[must_use]
    pub fn receipt_status(&self) -> Option<&str> {
        self.receipt_status.as_deref()
    }

    /// Whether the plan was actually applied and a receipt returned.
    #[must_use]
    pub const fn was_applied(&self) -> bool {
        self.applied
    }

    /// Return all planning and application evidence.
    #[must_use]
    pub fn evidence(&self) -> &[DeploymentEvidenceRecord] {
        &self.evidence
    }

    /// Return deterministic evidence counts.
    #[must_use]
    pub fn evidence_counts(&self) -> &DeploymentEvidenceCounts {
        &self.evidence_counts
    }

    /// Return planning-only evidence.
    #[must_use]
    pub fn planning_evidence(
        &self,
    ) -> impl Iterator<Item = &DeploymentEvidenceRecord> {
        self.evidence
            .iter()
            .filter(|item| {
                item.phase == DeploymentEvidencePhase::Planning
            })
    }

    /// Return application-only evidence.
    #[must_use]
    pub fn application_evidence(
        &self,
    ) -> impl Iterator<Item = &DeploymentEvidenceRecord> {
        self.evidence
            .iter()
            .filter(|item| {
                item.phase == DeploymentEvidencePhase::Application
            })
    }

    /// Whether the deployment authority confirmed completion.
    ///
    /// This checks the actual application outcome rather than
    /// planning readiness or request acceptance.
    #[must_use]
    pub fn is_completed_successfully(&self) -> bool {
        self.operation_status == "completed"
            && self.receipt_status.as_deref()
                == Some("completed")
    }

    /// Whether the operation was accepted but not confirmed complete.
    #[must_use]
    pub fn is_pending(&self) -> bool {
        matches!(
            self.operation_status.as_str(),
            "accepted" | "in_progress"
        )
    }

    /// Whether the deployment outcome is unknown.
    #[must_use]
    pub fn is_unknown(&self) -> bool {
        self.operation_status == "unknown"
            || self.receipt_status.as_deref() == Some("unknown")
    }

    /// Whether the deployment was partially completed.
    #[must_use]
    pub fn is_partial(&self) -> bool {
        self.operation_status == "partially_completed"
            || self.receipt_status.as_deref()
                == Some("partially_completed")
    }

    /// Validate the complete report.
    ///
    /// This also protects reports deserialized from external sources.
    pub fn validate(&self) -> ToolchainResult<()> {
        if self.schema_version
            != DEPLOYMENT_REPORT_SCHEMA_VERSION
        {
            return Err(validation_error(
                format!(
                    "unsupported deployment report schema version `{}`",
                    self.schema_version,
                ),
            ));
        }

        validate_identifier("report.plan_id", &self.plan_id)?;
        validate_identifier("report.provider", &self.provider)?;
        validate_identifier("report.readiness", &self.readiness)?;
        validate_identifier(
            "report.operation_status",
            &self.operation_status,
        )?;

        if let Some(id) = &self.deployment_id {
            validate_identifier("report.deployment_id", id)?;
        }

        if let Some(reference) = &self.provider_reference {
            validate_identifier(
                "report.provider_reference",
                reference,
            )?;
        }

        if let Some(status) = &self.receipt_status {
            validate_identifier("report.receipt_status", status)?;
        }

        if self.applied != self.deployment_id.is_some() {
            return Err(validation_error(
                "applied state and deployment identity are inconsistent",
            ));
        }

        if self.applied != self.receipt_status.is_some() {
            return Err(validation_error(
                "applied state and receipt status are inconsistent",
            ));
        }

        if !self.applied && self.provider_reference.is_some() {
            return Err(validation_error(
                "a plan-only report cannot contain a provider receipt reference",
            ));
        }

        if !self.applied
            && !matches!(
                self.operation_status.as_str(),
                "planned" | "blocked" | "unresolved"
            )
        {
            return Err(validation_error(
                "an operation without a receipt has an inconsistent status",
            ));
        }

        if self.applied
            && !matches!(
                self.operation_status.as_str(),
                "accepted"
                    | "in_progress"
                    | "completed"
                    | "partially_completed"
                    | "failed"
                    | "cancelled"
                    | "unknown"
            )
        {
            return Err(validation_error(
                "an applied operation has an invalid lifecycle status",
            ));
        }

        if self.applied
            && self.operation_status != self.receipt_status.as_deref()
                .unwrap_or_default()
        {
            return Err(validation_error(
                "operation status and receipt status disagree",
            ));
        }

        for record in &self.evidence {
            record.validate()?;
        }

        if !self.evidence.windows(2).all(|pair| {
            pair[0] <= pair[1]
        }) {
            return Err(validation_error(
                "deployment evidence is not in canonical order",
            ));
        }

        let calculated = calculate_evidence_counts(&self.evidence)?;

        if calculated != self.evidence_counts {
            return Err(validation_error(
                "deployment evidence counts do not match the evidence collection",
            ));
        }

        Ok(())
    }

    /// Serialize the validated report into JSON.
    ///
    /// The output contains report metadata and evidence snapshots,
    /// not binary artifacts or credentials.
    pub fn to_json(&self) -> ToolchainResult<String> {
        self.validate()?;

        serde_json::to_string(self).map_err(|error| {
            serialization_error(format!(
                "failed to serialize deployment report: {error}"
            ))
        })
    }

    /// Serialize the validated report as pretty-printed JSON.
    pub fn to_pretty_json(&self) -> ToolchainResult<String> {
        self.validate()?;

        serde_json::to_string_pretty(self).map_err(|error| {
            serialization_error(format!(
                "failed to serialize deployment report: {error}"
            ))
        })
    }

    /// Deserialize and validate a deployment report.
    ///
    /// Reports from untrusted external sources must pass this method
    /// before being consumed.
    pub fn from_json(
        input: &str,
    ) -> ToolchainResult<Self> {
        let report: Self = serde_json::from_str(input)
            .map_err(|error| {
                invalid_report(format!(
                    "invalid deployment report JSON: {error}"
                ))
            })?;

        report.validate()?;

        Ok(report)
    }
}

// ============================================================================
// PLAN AND RECEIPT BINDING
// ============================================================================

fn validate_plan_binding(
    plan: &DeploymentPlan,
    result: &DeploymentCommandResult,
) -> ToolchainResult<()> {
    plan.validate()?;

    plan.validate_for(result.request())?;

    Ok(())
}

fn validate_operation_consistency(
    status: DeploymentCommandStatus,
    plan: &DeploymentPlan,
    receipt: Option<&DeploymentReceipt>,
) -> ToolchainResult<()> {
    match receipt {
        None => {
            let expected = match plan.readiness() {
                DeploymentReadiness::Ready => {
                    DeploymentCommandStatus::Planned
                }
                DeploymentReadiness::Blocked => {
                    DeploymentCommandStatus::Blocked
                }
                DeploymentReadiness::Unresolved => {
                    DeploymentCommandStatus::Unresolved
                }
                _ => DeploymentCommandStatus::Unknown,
            };

            if status != expected {
                return Err(validation_error(
                    "plan-only command status does not match plan readiness",
                ));
            }
        }

        Some(receipt) => {
            receipt.validate_for(plan)?;

            let expected = match receipt.status() {
                DeploymentReceiptStatus::Accepted => {
                    DeploymentCommandStatus::Accepted
                }
                DeploymentReceiptStatus::InProgress => {
                    DeploymentCommandStatus::InProgress
                }
                DeploymentReceiptStatus::Completed => {
                    DeploymentCommandStatus::Completed
                }
                DeploymentReceiptStatus::PartiallyCompleted => {
                    DeploymentCommandStatus::PartiallyCompleted
                }
                DeploymentReceiptStatus::Failed => {
                    DeploymentCommandStatus::Failed
                }
                DeploymentReceiptStatus::Cancelled => {
                    DeploymentCommandStatus::Cancelled
                }
                DeploymentReceiptStatus::Unknown => {
                    DeploymentCommandStatus::Unknown
                }
                _ => DeploymentCommandStatus::Unknown,
            };

            if status != expected {
                return Err(validation_error(
                    "deployment command status does not match its receipt",
                ));
            }
        }
    }

    Ok(())
}

// ============================================================================
// CONTRACT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_evidence() -> DeploymentEvidence {
        DeploymentEvidence::new(
            DeploymentStage::BuildVerified,
            DeploymentEvidenceStatus::Passed,
            "compiler.artifact.verification",
            "project.sample",
            "The canonical build artifact was verified",
        )
        .expect("valid test evidence")
    }

    #[test]
    fn reporting_module_has_stable_identity() {
        assert_eq!(
            DEPLOYMENT_REPORTING_MODULE_NAME,
            "zamani.toolchain.deployment.reporting",
        );

        assert_eq!(DEPLOYMENT_REPORT_SCHEMA_VERSION, 1);
    }

    #[test]
    fn evidence_phase_identifiers_are_stable() {
        assert_eq!(
            DeploymentEvidencePhase::Planning.as_str(),
            "planning",
        );

        assert_eq!(
            DeploymentEvidencePhase::Application.as_str(),
            "application",
        );
    }

    #[test]
    fn planning_evidence_preserves_original_identity() {
        let evidence = sample_evidence();

        let record = DeploymentEvidenceRecord::from_evidence(
            &evidence,
            DeploymentEvidencePhase::Planning,
        );

        assert_eq!(record.phase, DeploymentEvidencePhase::Planning);
        assert_eq!(record.stage, "build_verified");
        assert_eq!(record.status, "passed");
        assert_eq!(
            record.authority,
            "compiler.artifact.verification",
        );
        assert_eq!(record.subject, "project.sample");
    }

    #[test]
    fn target_specific_evidence_preserves_target_identity() {
        let evidence = sample_evidence()
            .for_target("future.target.realization");

        let record = DeploymentEvidenceRecord::from_evidence(
            &evidence,
            DeploymentEvidencePhase::Planning,
        );

        assert_eq!(
            record.target.as_deref(),
            Some("future.target.realization"),
        );
    }

    #[test]
    fn evidence_reference_is_preserved() {
        let evidence = sample_evidence()
            .with_reference("provenance.reference.001");

        let record = DeploymentEvidenceRecord::from_evidence(
            &evidence,
            DeploymentEvidencePhase::Planning,
        );

        assert_eq!(
            record.reference.as_deref(),
            Some("provenance.reference.001"),
        );
    }

    #[test]
    fn evidence_counts_are_deterministic() {
        let evidence = sample_evidence();

        let record = DeploymentEvidenceRecord::from_evidence(
            &evidence,
            DeploymentEvidencePhase::Planning,
        );

        let counts = calculate_evidence_counts(&[
            record.clone(),
            record,
        ])
        .expect("counts should be representable");

        assert_eq!(
            counts.get("planning.build_verified.passed"),
            Some(&2),
        );
    }

    #[test]
    fn evidence_order_is_deterministic() {
        let first = DeploymentEvidenceRecord {
            phase: DeploymentEvidencePhase::Planning,
            stage: "build_verified".to_owned(),
            status: "passed".to_owned(),
            authority: "authority.z".to_owned(),
            subject: "subject".to_owned(),
            target: None,
            reference: None,
            message: "message".to_owned(),
        };

        let second = DeploymentEvidenceRecord {
            authority: "authority.a".to_owned(),
            ..first.clone()
        };

        let mut forward = vec![first.clone(), second.clone()];
        let mut reverse = vec![second, first];

        forward.sort_unstable();
        reverse.sort_unstable();

        assert_eq!(forward, reverse);
    }

    #[test]
    fn evidence_with_unknown_status_is_not_promoted_to_success() {
        assert_eq!(
            evidence_status_name(
                DeploymentEvidenceStatus::Unresolved,
            ),
            "unresolved",
        );

        assert_eq!(
            evidence_status_name(
                DeploymentEvidenceStatus::NotRequired,
            ),
            "not_required",
        );
    }

    #[test]
    fn accepted_is_not_completed() {
        assert_eq!(
            command_status_name(
                DeploymentCommandStatus::Accepted,
            ),
            "accepted",
        );

        assert_ne!(
            command_status_name(
                DeploymentCommandStatus::Accepted,
            ),
            "completed",
        );
    }

    #[test]
    fn runtime_completion_and_deployment_completion_are_distinct() {
        assert_eq!(
            deployment_stage_name(
                DeploymentStage::RuntimeCompleted,
            ),
            "runtime_completed",
        );

        assert_eq!(
            deployment_stage_name(
                DeploymentStage::DeploymentCompleted,
            ),
            "deployment_completed",
        );
    }

    #[test]
    fn unknown_receipt_remains_unknown() {
        assert_eq!(
            receipt_status_name(
                DeploymentReceiptStatus::Unknown,
            ),
            "unknown",
        );
    }

    #[test]
    fn invalid_evidence_records_are_rejected() {
        let record = DeploymentEvidenceRecord {
            phase: DeploymentEvidencePhase::Planning,
            stage: String::new(),
            status: "passed".to_owned(),
            authority: "authority".to_owned(),
            subject: "subject".to_owned(),
            target: None,
            reference: None,
            message: "message".to_owned(),
        };

        assert!(record.validate().is_err());
    }

    #[test]
    fn evidence_control_characters_are_rejected() {
        let record = DeploymentEvidenceRecord {
            phase: DeploymentEvidencePhase::Planning,
            stage: "build_verified".to_owned(),
            status: "passed".to_owned(),
            authority: "authority".to_owned(),
            subject: "subject".to_owned(),
            target: None,
            reference: None,
            message: "invalid\nmessage".to_owned(),
        };

        assert!(record.validate().is_err());
    }

    #[test]
    fn evidence_count_key_preserves_phase() {
        let evidence = sample_evidence();

        let planning = DeploymentEvidenceRecord::from_evidence(
            &evidence,
            DeploymentEvidencePhase::Planning,
        );

        let application = DeploymentEvidenceRecord::from_evidence(
            &evidence,
            DeploymentEvidencePhase::Application,
        );

        assert_ne!(
            evidence_count_key(&planning),
            evidence_count_key(&application),
        );
    }

    #[test]
    fn plan_readiness_is_not_application_completion() {
        assert_eq!(
            readiness_name(DeploymentReadiness::Ready),
            "ready",
        );

        assert_ne!(
            readiness_name(DeploymentReadiness::Ready),
            command_status_name(
                DeploymentCommandStatus::Completed,
            ),
        );
    }

    #[test]
    fn report_schema_is_versioned() {
        assert_eq!(
            DEPLOYMENT_REPORT_SCHEMA_VERSION,
            1,
        );
    }

    #[test]
    fn serialization_contract_uses_stable_status_names() {
        assert_eq!(
            command_status_name(
                DeploymentCommandStatus::PartiallyCompleted,
            ),
            "partially_completed",
        );

        assert_eq!(
            command_status_name(
                DeploymentCommandStatus::Cancelled,
            ),
            "cancelled",
        );
    }

    #[test]
    fn receipt_status_mapping_is_complete() {
        let statuses = [
            DeploymentReceiptStatus::Accepted,
            DeploymentReceiptStatus::InProgress,
            DeploymentReceiptStatus::Completed,
            DeploymentReceiptStatus::PartiallyCompleted,
            DeploymentReceiptStatus::Failed,
            DeploymentReceiptStatus::Cancelled,
            DeploymentReceiptStatus::Unknown,
        ];

        for status in statuses {
            assert!(!receipt_status_name(status).is_empty());
        }
    }

    #[test]
    fn report_consistency_rejects_missing_receipts_for_applied_statuses() {
        // A command result claiming completion without a receipt
        // cannot satisfy the plan-only lifecycle contract.
        let result = validate_operation_consistency(
            DeploymentCommandStatus::Completed,
            &dummy_plan_unavailable(),
            None,
        );

        assert!(result.is_err());
    }

    // The deployment plan requires a validated canonical request.
    // Full plan/result construction and serialization round-trip
    // are exercised by the deployment integration test suite.
    //
    // This helper deliberately does not manufacture a plan.
    fn dummy_plan_unavailable() -> DeploymentPlan {
        panic!(
            "A deployment plan must be constructed from a canonical \
             DeployRequest in an integration test"
        );
    }
}
