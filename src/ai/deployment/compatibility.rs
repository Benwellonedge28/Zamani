
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani AI Deployment Compatibility
//!
//! Path: src/ai/deployment/compatibility.rs
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # PURPOSE
//!
//! Coordinates AI contract compatibility, deployment manifest
//! integrity, target capability compatibility, and resource
//! feasibility for AI deployment requests.
//!
//! This module answers:
//!
//! "Can this AI deployment contract be accepted by the intended
//! consumer and realized against the supplied target evidence?"
//!
//! # OWNERSHIP
//!
//! This module owns:
//!
//! - AI deployment compatibility orchestration;
//! - deployment compatibility request contracts;
//! - deployment compatibility reports;
//! - deployment-specific compatibility findings;
//! - aggregation of AI and backend compatibility decisions;
//! - deployment compatibility validation.
//!
//! # DOES NOT OWN
//!
//! - AI version representation;
//! - AI feature compatibility;
//! - model semantic validation;
//! - artifact serialization;
//! - artifact storage;
//! - cryptographic verification;
//! - target discovery;
//! - hardware capability discovery;
//! - physical resource allocation;
//! - device selection;
//! - model conversion;
//! - deployment orchestration;
//! - placement;
//! - scheduling;
//! - networking;
//! - runtime execution.
//!
//! # CANONICAL AUTHORITIES
//!
//! AI compatibility:
//!     crate::ai::compatibility
//!
//! AI versions:
//!     crate::ai::version
//!
//! AI artifacts:
//!     crate::ai::deployment::artifacts
//!
//! AI deployment manifests:
//!     crate::ai::deployment::manifests
//!
//! AI deployment formats:
//!     crate::ai::deployment::formats
//!
//! Backend capability and resource compatibility:
//!     crate::backend::compatibility
//!
//! Canonical compatibility classification:
//!     crate::zuir::version
//!
//! # POCO-REAF
//!
//! Compatibility is independent of physical machine size.
//!
//! No universal limits are imposed on:
//!
//! - models;
//! - model parameters;
//! - artifacts;
//! - tensor dimensions;
//! - tensor rank;
//! - agents;
//! - deployment instances;
//! - replicas;
//! - workers;
//! - nodes;
//! - accelerators;
//! - memory;
//! - storage;
//! - processing capacity.
//!
//! Resource requirements are evaluated against supplied target
//! evidence using the canonical backend compatibility subsystem.
//!
//! A finite target may be unable to realize an otherwise valid
//! portable AI program. Such failure must not alter its semantics.
//!
//! # IMPORTANT
//!
//! Compatibility is not deployment success.
//!
//! This module performs no deployment actions.
//!
//! It evaluates declared contracts and supplied evidence only.
//!
//! Missing evidence must never be represented as successful
//! compatibility.
//!
//! # DETERMINISM
//!
//! All aggregated findings have deterministic ordering.
//!
//! The evaluator has no global mutable state.
//!
//! Identical validated inputs produce identical reports.
//!
//! # FREEZE CRITERIA
//!
//! - Existing AI compatibility implementation is reused.
//! - Existing backend compatibility implementation is reused.
//! - Deployment manifests are validated.
//! - All compatibility dimensions remain independently visible.
//! - Migration requirements are preserved.
//! - Unknown target evidence is not treated as success.
//! - Resource constraints remain separate from semantic limits.
//! - No hardware assumptions are introduced.
//! - No unsafe Rust is introduced.

use std::fmt;

use crate::ai::compatibility::{
    evaluate_compatibility,
    AiCompatibilityDescriptor,
    AiCompatibilityError,
    AiCompatibilityProfile,
    AiCompatibilityReport,
};

use crate::ai::deployment::manifests::{
    AiDeploymentManifest,
    AiDeploymentManifestError,
};

use crate::backend::compatibility::{
    CompatibilityEvaluator,
    CompatibilityReport,
    CompatibilityRequest,
    CompatibilityStatus,
    TargetCapabilityProfile,
};

use crate::zuir::version::Compatibility;

// ============================================================================
// DEPLOYMENT COMPATIBILITY STATUS
// ============================================================================

/// Overall AI deployment compatibility classification.
///
/// The classification deliberately distinguishes compatibility
/// from the ability to proceed immediately.
///
/// In particular:
///
/// - RequiresMigration is not Compatible.
/// - Indeterminate is not Compatible.
/// - Incompatible must be rejected.
///
/// A deployment coordinator must not interpret any of the
/// non-compatible states as permission to execute.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
)]
pub enum AiDeploymentCompatibilityStatus {
    /// Every required compatibility check passed.
    Compatible,

    /// Hard requirements passed, but non-blocking warnings exist.
    CompatibleWithWarnings,

    /// One or more representations require explicit migration.
    RequiresMigration,

    /// Required evidence is insufficient to establish compatibility.
    Indeterminate,

    /// At least one hard compatibility requirement failed.
    Incompatible,
}

impl AiDeploymentCompatibilityStatus {
    /// Whether the deployment is compatible without migration.
    #[must_use]
    pub const fn is_compatible(self) -> bool {
        matches!(
            self,
            Self::Compatible
                | Self::CompatibleWithWarnings
        )
    }

    /// Whether migration must occur before deployment.
    #[must_use]
    pub const fn requires_migration(self) -> bool {
        matches!(
            self,
            Self::RequiresMigration
        )
    }

    /// Whether the result must be rejected.
    #[must_use]
    pub const fn is_incompatible(self) -> bool {
        matches!(
            self,
            Self::Incompatible
        )
    }

    /// Whether additional evidence is necessary.
    #[must_use]
    pub const fn is_indeterminate(self) -> bool {
        matches!(
            self,
            Self::Indeterminate
        )
    }
}

impl fmt::Display for AiDeploymentCompatibilityStatus {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let value = match self {
            Self::Compatible => "compatible",
            Self::CompatibleWithWarnings => {
                "compatible_with_warnings"
            }
            Self::RequiresMigration => "requires_migration",
            Self::Indeterminate => "indeterminate",
            Self::Incompatible => "incompatible",
        };

        formatter.write_str(value)
    }
}

// ============================================================================
// DEPLOYMENT COMPATIBILITY FINDINGS
// ============================================================================

/// Category of a deployment compatibility finding.
///
/// These categories describe the boundary at which compatibility
/// was evaluated.
///
/// They do not replace the canonical error or diagnostic systems.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
)]
pub enum AiDeploymentCompatibilityDomain {
    /// AI public API contract.
    Api,

    /// AI computational semantics.
    Semantics,

    /// AI model schema.
    ModelSchema,

    /// AI operation set.
    Operations,

    /// AI artifact serialization contract.
    ArtifactFormat,

    /// AI feature support.
    Features,

    /// AI deployment manifest.
    Manifest,

    /// Target capabilities and resources.
    Target,

    /// Deployment evidence.
    Evidence,
}

/// Stable deployment compatibility finding code.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
)]
pub enum AiDeploymentCompatibilityCode {
    /// A deployment representation needs migration.
    MigrationRequired,

    /// An AI contract version is incompatible.
    VersionIncompatible,

    /// A required AI feature is unsupported.
    FeatureUnsupported,

    /// The deployment manifest is inconsistent.
    ManifestInconsistent,

    /// A required target capability is unavailable.
    TargetCapabilityUnavailable,

    /// A required target resource is unavailable.
    TargetResourceUnavailable,

    /// Target capability evidence is insufficient.
    TargetEvidenceInsufficient,

    /// An optional capability or resource is unavailable.
    OptionalRequirementUnavailable,

    /// Experimental support was accepted by explicit policy.
    ExperimentalSupportAccepted,
}

/// Severity of a deployment compatibility finding.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
)]
pub enum AiDeploymentCompatibilitySeverity {
    /// Informational finding.
    Info,

    /// Non-blocking finding.
    Warning,

    /// Blocking finding.
    Error,
}

/// One deployment compatibility finding.
///
/// `subject` contains a stable domain identifier or reference.
///
/// `message` is explanatory text. Consumers should use `code`,
/// `domain`, and `severity` for programmatic decisions.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub struct AiDeploymentCompatibilityIssue {
    /// Finding domain.
    pub domain: AiDeploymentCompatibilityDomain,

    /// Stable finding code.
    pub code: AiDeploymentCompatibilityCode,

    /// Finding severity.
    pub severity: AiDeploymentCompatibilitySeverity,

    /// Subject associated with the finding.
    pub subject: String,

    /// Human-readable explanation.
    pub message: String,
}

impl AiDeploymentCompatibilityIssue {
    fn new(
        domain: AiDeploymentCompatibilityDomain,
        code: AiDeploymentCompatibilityCode,
        severity: AiDeploymentCompatibilitySeverity,
        subject: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            domain,
            code,
            severity,
            subject: subject.into(),
            message: message.into(),
        }
    }
}

// ============================================================================
// COMPATIBILITY REQUEST
// ============================================================================

/// Complete AI deployment compatibility request.
///
/// The caller supplies authoritative compatibility evidence.
///
/// This request deliberately borrows its inputs:
///
/// - no artifact payload is copied;
/// - no manifest is mutated;
/// - no target profile is modified;
/// - no global state is created.
///
/// The AI descriptor and consumer profile are evaluated by
/// `crate::ai::compatibility`.
///
/// The target request and profile are evaluated by
/// `crate::backend::compatibility`.
#[derive(Debug)]
pub struct AiDeploymentCompatibilityRequest<'a> {
    /// AI contract and required feature descriptor.
    pub ai_descriptor: &'a AiCompatibilityDescriptor,

    /// AI consumer compatibility profile.
    pub ai_consumer: &'a AiCompatibilityProfile,

    /// Optional complete deployment manifest.
    ///
    /// When supplied, its structural and referential integrity
    /// is validated before compatibility is accepted.
    pub manifest: Option<&'a AiDeploymentManifest>,

    /// Backend target requirements for deployment.
    ///
    /// These include capability and resource requirements.
    pub target_requirements: &'a CompatibilityRequest,

    /// Actual target capability and resource evidence.
    pub target_profile: &'a TargetCapabilityProfile,
}

impl<'a> AiDeploymentCompatibilityRequest<'a> {
    /// Construct a deployment compatibility request.
    #[must_use]
    pub const fn new(
        ai_descriptor: &'a AiCompatibilityDescriptor,
        ai_consumer: &'a AiCompatibilityProfile,
        target_requirements: &'a CompatibilityRequest,
        target_profile: &'a TargetCapabilityProfile,
    ) -> Self {
        Self {
            ai_descriptor,
            ai_consumer,
            manifest: None,
            target_requirements,
            target_profile,
        }
    }

    /// Attach a deployment manifest for validation.
    #[must_use]
    pub const fn with_manifest(
        mut self,
        manifest: &'a AiDeploymentManifest,
    ) -> Self {
        self.manifest = Some(manifest);
        self
    }
}

// ============================================================================
// COMPLETE DEPLOYMENT COMPATIBILITY REPORT
// ============================================================================

/// Complete deployment compatibility assessment.
///
/// The underlying AI and backend reports remain independently
/// accessible.
///
/// This prevents loss of information when several compatibility
/// domains are evaluated together.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub struct AiDeploymentCompatibilityReport {
    /// Final aggregated status.
    pub status: AiDeploymentCompatibilityStatus,

    /// AI contract compatibility.
    pub ai: AiCompatibilityReport,

    /// Backend target compatibility.
    pub target: CompatibilityReport,

    /// Deterministically ordered deployment findings.
    pub issues: Vec<AiDeploymentCompatibilityIssue>,
}

impl AiDeploymentCompatibilityReport {
    /// Whether the deployment can proceed without migration.
    ///
    /// This does not mean that deployment has been performed.
    #[must_use]
    pub const fn is_compatible(&self) -> bool {
        self.status.is_compatible()
    }

    /// Whether explicit migration is required.
    #[must_use]
    pub const fn requires_migration(&self) -> bool {
        self.status.requires_migration()
    }

    /// Whether deployment must be rejected.
    #[must_use]
    pub const fn must_reject(&self) -> bool {
        self.status.is_incompatible()
    }

    /// Whether additional target evidence is necessary.
    #[must_use]
    pub const fn is_indeterminate(&self) -> bool {
        self.status.is_indeterminate()
    }

    /// Return blocking findings.
    pub fn errors(
        &self,
    ) -> impl Iterator<Item = &AiDeploymentCompatibilityIssue> {
        self.issues.iter().filter(|issue| {
            issue.severity
                == AiDeploymentCompatibilitySeverity::Error
        })
    }

    /// Return non-blocking findings.
    pub fn warnings(
        &self,
    ) -> impl Iterator<Item = &AiDeploymentCompatibilityIssue> {
        self.issues.iter().filter(|issue| {
            issue.severity
                == AiDeploymentCompatibilitySeverity::Warning
        })
    }

    /// Return informational findings.
    pub fn information(
        &self,
    ) -> impl Iterator<Item = &AiDeploymentCompatibilityIssue> {
        self.issues.iter().filter(|issue| {
            issue.severity
                == AiDeploymentCompatibilitySeverity::Info
        })
    }
}

// ============================================================================
// EVALUATOR
// ============================================================================

/// Stateless AI deployment compatibility evaluator.
///
/// Evaluation is pure with respect to the supplied contracts and
/// evidence.
///
/// It performs no:
///
/// - filesystem access;
/// - network access;
/// - hardware discovery;
/// - device selection;
/// - resource allocation;
/// - artifact conversion;
/// - model loading;
/// - deployment;
/// - runtime execution.
#[derive(Debug, Clone, Copy, Default)]
pub struct AiDeploymentCompatibilityEvaluator;

impl AiDeploymentCompatibilityEvaluator {
    /// Create an evaluator.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Evaluate a complete deployment compatibility request.
    ///
    /// An invalid descriptor, invalid consumer profile, or invalid
    /// deployment manifest returns an error.
    ///
    /// A valid request with incompatible target evidence returns
    /// a compatibility report instead.
    pub fn evaluate(
        &self,
        request: &AiDeploymentCompatibilityRequest<'_>,
    ) -> Result<
        AiDeploymentCompatibilityReport,
        AiDeploymentCompatibilityError,
    > {
        // ------------------------------------------------------------
        // 1. Validate the optional deployment manifest.
        // ------------------------------------------------------------

        if let Some(manifest) = request.manifest {
            manifest.validate().map_err(
                AiDeploymentCompatibilityError::Manifest,
            )?;

            self.validate_manifest_versions(
                manifest,
                request.ai_descriptor,
            )?;
        }

        // ------------------------------------------------------------
        // 2. Evaluate AI contract compatibility.
        // ------------------------------------------------------------

        let ai_report = evaluate_compatibility(
            request.ai_descriptor,
            request.ai_consumer,
        )
        .map_err(AiDeploymentCompatibilityError::Ai)?;

        // ------------------------------------------------------------
        // 3. Evaluate target compatibility.
        // ------------------------------------------------------------

        let target_report = CompatibilityEvaluator.evaluate(
            request.target_requirements,
            request.target_profile,
        );

        // ------------------------------------------------------------
        // 4. Aggregate domain-specific findings.
        // ------------------------------------------------------------

        let mut issues = Vec::new();

        self.collect_ai_issues(
            &ai_report,
            &mut issues,
        );

        self.collect_target_issues(
            &target_report,
            &mut issues,
        );

        // ------------------------------------------------------------
        // 5. Establish deterministic ordering.
        // ------------------------------------------------------------

        issues.sort_by(|left, right| {
            left.domain
                .cmp(&right.domain)
                .then_with(|| left.subject.cmp(&right.subject))
                .then_with(|| left.code.cmp(&right.code))
                .then_with(|| left.severity.cmp(&right.severity))
                .then_with(|| left.message.cmp(&right.message))
        });

        // ------------------------------------------------------------
        // 6. Determine the final status.
        // ------------------------------------------------------------

        let status = classify_deployment(
            &ai_report,
            &target_report,
            &issues,
        );

        Ok(AiDeploymentCompatibilityReport {
            status,
            ai: ai_report,
            target: target_report,
            issues,
        })
    }

    fn validate_manifest_versions(
        &self,
        manifest: &AiDeploymentManifest,
        descriptor: &AiCompatibilityDescriptor,
    ) -> Result<(), AiDeploymentCompatibilityError> {
        if manifest.api_version != descriptor.api_version {
            return Err(
                AiDeploymentCompatibilityError::ManifestVersionMismatch {
                    domain: "AI API",
                },
            );
        }

        if manifest.semantic_version != descriptor.semantic_version {
            return Err(
                AiDeploymentCompatibilityError::ManifestVersionMismatch {
                    domain: "AI semantics",
                },
            );
        }

        if manifest.artifact_format_version
            != descriptor.artifact_format_version
        {
            return Err(
                AiDeploymentCompatibilityError::ManifestVersionMismatch {
                    domain: "AI artifact format",
                },
            );
        }

        Ok(())
    }

    fn collect_ai_issues(
        &self,
        report: &AiCompatibilityReport,
        issues: &mut Vec<AiDeploymentCompatibilityIssue>,
    ) {
        let versions = &report.versions;

        self.collect_version_issue(
            "api",
            Compatibility::from_version_decision(&versions.api),
            AiDeploymentCompatibilityDomain::Api,
            issues,
        );

        self.collect_version_issue(
            "semantics",
            Compatibility::from_version_decision(&versions.semantics),
            AiDeploymentCompatibilityDomain::Semantics,
            issues,
        );

        self.collect_version_issue(
            "model_schema",
            Compatibility::from_version_decision(&versions.model_schema),
            AiDeploymentCompatibilityDomain::ModelSchema,
            issues,
        );

        self.collect_version_issue(
            "operations",
            Compatibility::from_version_decision(&versions.operations),
            AiDeploymentCompatibilityDomain::Operations,
            issues,
        );

        self.collect_version_issue(
            "artifact_format",
            Compatibility::from_version_decision(
                &versions.artifact_format,
            ),
            AiDeploymentCompatibilityDomain::ArtifactFormat,
            issues,
        );

        for feature in &report.features.unsupported {
            issues.push(
                AiDeploymentCompatibilityIssue::new(
                    AiDeploymentCompatibilityDomain::Features,
                    AiDeploymentCompatibilityCode::FeatureUnsupported,
                    AiDeploymentCompatibilitySeverity::Error,
                    feature.qualified_name(),
                    format!(
                        "Required AI feature `{feature}` is not supported \
                         by consumer `{}`",
                        report.consumer_identity
                    ),
                ),
            );
        }
    }

    fn collect_version_issue<T>(
        &self,
        subject: &'static str,
        decision: Compatibility,
        domain: AiDeploymentCompatibilityDomain,
        issues: &mut Vec<AiDeploymentCompatibilityIssue>,
    )
    where
        T: ?Sized,
    {
        match decision {
            Compatibility::Compatible => {}

            Compatibility::RequiresMigration => {
                issues.push(
                    AiDeploymentCompatibilityIssue::new(
                        domain,
                        AiDeploymentCompatibilityCode::MigrationRequired,
                        AiDeploymentCompatibilitySeverity::Warning,
                        subject,
                        format!(
                            "{subject} requires an explicit migration \
                             before deployment"
                        ),
                    ),
                );
            }

            Compatibility::Incompatible => {
                issues.push(
                    AiDeploymentCompatibilityIssue::new(
                        domain,
                        AiDeploymentCompatibilityCode::VersionIncompatible,
                        AiDeploymentCompatibilitySeverity::Error,
                        subject,
                        format!(
                            "{subject} version is incompatible"
                        ),
                    ),
                );
            }
        }
    }

    fn collect_target_issues(
        &self,
        report: &CompatibilityReport,
        issues: &mut Vec<AiDeploymentCompatibilityIssue>,
    ) {
        use crate::backend::compatibility::{
            CompatibilityIssueCode,
            CompatibilitySeverity,
        };

        for finding in &report.issues {
            let (code, severity) = match finding.code {
                CompatibilityIssueCode::CapabilityUnsupported
                | CompatibilityIssueCode::CapabilityPartiallySupported => (
                    AiDeploymentCompatibilityCode::TargetCapabilityUnavailable,
                    map_target_severity(finding.severity),
                ),

                CompatibilityIssueCode::CapabilityExperimental => (
                    AiDeploymentCompatibilityCode::ExperimentalSupportAccepted,
                    map_target_severity(finding.severity),
                ),

                CompatibilityIssueCode::CapabilityUnknown
                | CompatibilityIssueCode::ResourceUnknown => (
                    AiDeploymentCompatibilityCode::TargetEvidenceInsufficient,
                    map_target_severity(finding.severity),
                ),

                CompatibilityIssueCode::ResourceInsufficient
                | CompatibilityIssueCode::ResourceUnavailable => (
                    AiDeploymentCompatibilityCode::TargetResourceUnavailable,
                    map_target_severity(finding.severity),
                ),

                CompatibilityIssueCode::CapabilityPreferenceUnavailable
                | CompatibilityIssueCode::ResourcePreferenceUnavailable => (
                    AiDeploymentCompatibilityCode::OptionalRequirementUnavailable,
                    map_target_severity(finding.severity),
                ),
            };

            let domain = if matches!(
                finding.code,
                CompatibilityIssueCode::ResourceInsufficient
                    | CompatibilityIssueCode::ResourceUnavailable
                    | CompatibilityIssueCode::ResourceUnknown
                    | CompatibilityIssueCode::ResourcePreferenceUnavailable
            ) {
                AiDeploymentCompatibilityDomain::Target
            } else {
                AiDeploymentCompatibilityDomain::Target
            };

            let severity = match finding.severity {
                CompatibilitySeverity::Info =>
                    AiDeploymentCompatibilitySeverity::Info,
                CompatibilitySeverity::Warning =>
                    AiDeploymentCompatibilitySeverity::Warning,
                CompatibilitySeverity::Error =>
                    AiDeploymentCompatibilitySeverity::Error,
            };

            issues.push(
                AiDeploymentCompatibilityIssue::new(
                    domain,
                    code,
                    severity,
                    finding.subject.clone(),
                    finding.message.clone(),
                ),
            );
        }
    }
}

// ============================================================================
// STATUS AGGREGATION
// ============================================================================

fn classify_deployment(
    ai: &AiCompatibilityReport,
    target: &CompatibilityReport,
    issues: &[AiDeploymentCompatibilityIssue],
) -> AiDeploymentCompatibilityStatus {
    // Hard failures always take precedence.
    if ai.must_reject()
        || target.status == CompatibilityStatus::Incompatible
        || issues.iter().any(|issue| {
            issue.severity
                == AiDeploymentCompatibilitySeverity::Error
        })
    {
        return AiDeploymentCompatibilityStatus::Incompatible;
    }

    // Missing required target evidence is not successful compatibility.
    if target.status == CompatibilityStatus::Indeterminate {
        return AiDeploymentCompatibilityStatus::Indeterminate;
    }

    // Migration is an explicit prerequisite.
    if ai.requires_migration() {
        return AiDeploymentCompatibilityStatus::RequiresMigration;
    }

    if issues.iter().any(|issue| {
        issue.code == AiDeploymentCompatibilityCode::MigrationRequired
    }) {
        return AiDeploymentCompatibilityStatus::RequiresMigration;
    }

    if target.status == CompatibilityStatus::CompatibleWithWarnings
        || issues.iter().any(|issue| {
            issue.severity
                == AiDeploymentCompatibilitySeverity::Warning
        })
    {
        return AiDeploymentCompatibilityStatus::CompatibleWithWarnings;
    }

    AiDeploymentCompatibilityStatus::Compatible
}

fn map_target_severity(
    severity: crate::backend::compatibility::CompatibilitySeverity,
) -> AiDeploymentCompatibilitySeverity {
    use crate::backend::compatibility::CompatibilitySeverity;

    match severity {
        CompatibilitySeverity::Info =>
            AiDeploymentCompatibilitySeverity::Info,

        CompatibilitySeverity::Warning =>
            AiDeploymentCompatibilitySeverity::Warning,

        CompatibilitySeverity::Error =>
            AiDeploymentCompatibilitySeverity::Error,
    }
}

// ============================================================================
// ERRORS
// ============================================================================

/// Errors encountered while validating deployment compatibility inputs.
///
/// Incompatibility itself is not an error returned by this type.
/// It is represented by `AiDeploymentCompatibilityReport`.
#[derive(Debug, thiserror::Error)]
pub enum AiDeploymentCompatibilityError {
    /// AI compatibility descriptor or consumer profile is invalid.
    #[error("AI compatibility validation failed: {0}")]
    Ai(#[source] AiCompatibilityError),

    /// Deployment manifest is invalid.
    #[error("AI deployment manifest validation failed: {0}")]
    Manifest(#[source] AiDeploymentManifestError),

    /// Manifest and descriptor version identities disagree.
    #[error("deployment manifest version mismatch in {domain}")]
    ManifestVersionMismatch {
        /// Version domain.
        domain: &'static str,
    },
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::ai::compatibility::{
        AiCompatibilityDescriptor,
        AiCompatibilityPolicies,
        AiCompatibilityProfile,
    };

    use crate::ai::version::{
        exact_ai_policy,
        AiApiVersion,
        AiArtifactFormatVersion,
        AiModelSchemaVersion,
        AiOperationSetVersion,
        AiSemanticVersion,
    };

    use crate::backend::compatibility::{
        CompatibilityRequest,
        TargetCapabilityProfile,
    };

    fn descriptor() -> AiCompatibilityDescriptor {
        AiCompatibilityDescriptor::new(
            AiApiVersion::new(1, 0, 0),
            AiSemanticVersion::new(1, 0, 0),
            AiModelSchemaVersion::new(1, 0, 0),
            AiOperationSetVersion::new(1, 0, 0),
            AiArtifactFormatVersion::new(1, 0, 0),
        )
    }

    fn profile() -> AiCompatibilityProfile {
        let policies = AiCompatibilityPolicies::new(
            exact_ai_policy(AiApiVersion::new(1, 0, 0)),
            exact_ai_policy(AiSemanticVersion::new(1, 0, 0)),
            exact_ai_policy(AiModelSchemaVersion::new(1, 0, 0)),
            exact_ai_policy(AiOperationSetVersion::new(1, 0, 0)),
            exact_ai_policy(AiArtifactFormatVersion::new(1, 0, 0)),
        );

        AiCompatibilityProfile::new(
            "zamani-ai-deployment-consumer",
            policies,
        )
        .expect("valid profile")
    }

    fn target() -> TargetCapabilityProfile {
        TargetCapabilityProfile::new("test-target")
            .expect("valid target")
    }

    #[test]
    fn compatible_contract_and_partial_empty_target_inventory_are_not_confused() {
        let descriptor = descriptor();
        let profile = profile();
        let requirements = CompatibilityRequest::new();
        let target = target();

        let request = AiDeploymentCompatibilityRequest::new(
            &descriptor,
            &profile,
            &requirements,
            &target,
        );

        let report = AiDeploymentCompatibilityEvaluator::new()
            .evaluate(&request)
            .expect("valid compatibility request");

        assert_eq!(
            report.status,
            AiDeploymentCompatibilityStatus::Compatible
        );
    }

    #[test]
    fn unknown_required_target_capability_is_not_success() {
        use crate::backend::compatibility::{
            CapabilityId,
            CapabilityRequirement,
        };

        let descriptor = descriptor();
        let profile = profile();

        let mut requirements = CompatibilityRequest::new();

        requirements.require_capability(
            CapabilityRequirement::required(
                CapabilityId::new("ai.inference")
                    .expect("valid capability"),
            ),
        );

        let target = target();

        let request = AiDeploymentCompatibilityRequest::new(
            &descriptor,
            &profile,
            &requirements,
            &target,
        );

        let report = AiDeploymentCompatibilityEvaluator::new()
            .evaluate(&request)
            .expect("valid request");

        assert_eq!(
            report.status,
            AiDeploymentCompatibilityStatus::Incompatible
        );
    }

    #[test]
    fn compatible_ai_contract_does_not_imply_resource_feasibility() {
        use crate::backend::compatibility::{
            DecimalInteger,
            ResourceId,
            ResourceRequirement,
        };

        let descriptor = descriptor();
        let profile = profile();

        let mut requirements = CompatibilityRequest::new();

        requirements.require_resource(
            ResourceRequirement::required(
                ResourceId::new("memory.bytes")
                    .expect("valid resource"),
                DecimalInteger::new("1024")
                    .expect("valid quantity"),
            ),
        );

        let target = target();

        let request = AiDeploymentCompatibilityRequest::new(
            &descriptor,
            &profile,
            &requirements,
            &target,
        );

        let report = AiDeploymentCompatibilityEvaluator::new()
            .evaluate(&request)
            .expect("valid request");

        assert_eq!(
            report.status,
            AiDeploymentCompatibilityStatus::Incompatible
        );

        assert!(report.ai.is_compatible_without_migration());
    }

    #[test]
    fn report_findings_are_deterministically_ordered() {
        let descriptor = descriptor();
        let profile = profile();
        let requirements = CompatibilityRequest::new();
        let target = target();

        let request = AiDeploymentCompatibilityRequest::new(
            &descriptor,
            &profile,
            &requirements,
            &target,
        );

        let evaluator = AiDeploymentCompatibilityEvaluator::new();

        let first = evaluator.evaluate(&request).unwrap();
        let second = evaluator.evaluate(&request).unwrap();

        assert_eq!(first, second);
    }

    #[test]
    fn status_helpers_are_consistent() {
        assert!(
            AiDeploymentCompatibilityStatus::Compatible.is_compatible()
        );

        assert!(
            AiDeploymentCompatibilityStatus::RequiresMigration
                .requires_migration()
        );

        assert!(
            AiDeploymentCompatibilityStatus::Incompatible
                .is_incompatible()
        );

        assert!(
            AiDeploymentCompatibilityStatus::Indeterminate
                .is_indeterminate()
        );
    }
}
