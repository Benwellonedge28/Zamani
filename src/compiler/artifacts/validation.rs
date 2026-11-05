
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Compiler — Artifact Validation
//!
//! Canonical path: src/compiler/artifacts/validation.rs
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: unsafe Rust is forbidden.
//!
//! ## PURPOSE
//!
//! Provide a common validation orchestration layer for canonical compiler
//! artifacts and artifact manifests.
//!
//! This module checks:
//!
//! - Inline artifact payload integrity.
//! - Artifact metadata against caller-supplied policies.
//! - Required provenance information.
//! - External payload verification state.
//! - Manifest structural integrity.
//! - Manifest metadata.
//! - Optional portability acceptance.
//! - Explicitly incomplete validation conditions.
//!
//! It aggregates findings without replacing the authorities responsible
//! for individual validation algorithms.
//!
//! ## OWNS
//!
//! - Artifact validation policy.
//! - Manifest validation policy.
//! - Validation findings and severity.
//! - Validation reports and status.
//! - Artifact and manifest validation orchestration.
//! - Optional integration with portability assessments.
//!
//! ## DOES NOT OWN
//!
//! - Artifact representation or payload storage.
//! - Artifact metadata storage.
//! - Manifest storage or dependency graph algorithms.
//! - Portability classification.
//! - Semantic analysis.
//! - ZUIR verification.
//! - Classical IR verification.
//! - Quantum IR verification.
//! - HDL verification.
//! - AI or tensor verification.
//! - Backend code generation.
//! - Cryptographic signing or authorization.
//! - External payload retrieval.
//! - Runtime execution.
//! - Stable artifact serialization.
//!
//! ## IMPORTANT VALIDATION DISTINCTION
//!
//! A valid artifact validation report means that the checks performed by
//! this module passed under the supplied policy.
//!
//! It does not automatically mean:
//!
//! - The artifact is semantically correct.
//! - Its IR has been fully verified.
//! - Its executable behavior is safe.
//! - Its target is available.
//! - Its portability has been proven.
//! - Its signature is trusted.
//! - Its dependencies are deployable.
//!
//! Those properties require their respective authoritative checks.
//!
//! ## SCALABILITY
//!
//! This module imposes no universal limits on:
//!
//! - Artifact count.
//! - Manifest size.
//! - Dependency count.
//! - Payload size.
//! - Metadata entry count.
//! - Computing domains.
//! - Hardware count.
//! - Processor count.
//! - Device count.
//! - Quantum resource count.
//! - Tensor rank.
//! - Register width.
//!
//! Optional operational limits are supplied through validation policies.
//! They are not language or hardware ceilings.
//!
//! External payloads are not loaded into memory by this module.
//! Their verification must be performed by an external storage or
//! verification provider.
//!
//! Actual processing remains bounded by available memory, address space,
//! execution time, storage and caller-defined operational budgets.
//!
//! ## DETERMINISM
//!
//! Manifest iteration and metadata iteration use the deterministic ordering
//! provided by their canonical owners.
//!
//! Findings are emitted in deterministic traversal order.
//!
//! ## INTEGRATION
//!
//! artifact.rs:
//!     Supplies Artifact, ArtifactId, ArtifactMetadata, ArtifactDigest,
//!     ArtifactPayload and ArtifactProvenance.
//!
//! manifest.rs:
//!     Supplies ArtifactManifest and structural validation.
//!
//! metadata.rs:
//!     Supplies metadata validation policies and findings.
//!
//! portability.rs:
//!     Supplies portability assessments and acceptance policies.
//!
//! src/zuir/verify/:
//!     Remains the authority for ZUIR verification.
//!
//! Domain IR verifiers:
//!     Remain the authorities for their respective representations.
//!
//! src/backend/:
//!     Supplies backend-produced artifacts.
//!
//! linker:
//!     Must consume artifacts only after the validation requirements
//!     applicable to its operation have passed.
//!
//! serialization:
//!     Owns stable artifact and manifest encoding and decoding.
//!
//! runtime:
//!     Owns external payload resolution, resource admission and execution.
//!
//! grammar/:
//!     Defines source syntax and semantic boundaries, not artifact
//!     validation algorithms.
//!
//! ## FREEZE CONDITIONS
//!
//! - Existing artifact and manifest APIs are reused.
//! - Metadata validation delegates to metadata.rs.
//! - Portability validation delegates to portability.rs.
//! - No duplicate IR verifier is introduced.
//! - External payloads are never falsely reported as verified.
//! - Incomplete validation is distinct from successful validation.
//! - No arbitrary scalability ceilings are introduced.
//! - All public policies and reports have documented behavior.
//! - Tests cover integrity, metadata, manifests and incomplete states.

use std::error::Error;
use std::fmt;

use super::artifact::{
    Artifact,
    ArtifactId,
    ArtifactMetadata,
};

use super::manifest::{
    ArtifactManifest,
};

use super::metadata::{
    validate_metadata,
    MetadataIssueSeverity,
    MetadataValidationPolicy,
};

use super::portability::{
    assess_artifact,
    assess_manifest,
    ManifestPortabilityInputs,
    PortabilityClass,
    PortabilityPolicy,
};

// ============================================================================
// Contract identity
// ============================================================================

/// Stable identifier for the artifact validation contract.
pub const ARTIFACT_VALIDATION_CONTRACT_ID: &str =
    "zamani.compiler.artifact.validation";

/// Major version of the artifact validation contract.
pub const ARTIFACT_VALIDATION_CONTRACT_MAJOR: u32 = 1;

/// Minor version of the artifact validation contract.
pub const ARTIFACT_VALIDATION_CONTRACT_MINOR: u32 = 0;

/// Patch version of the artifact validation contract.
pub const ARTIFACT_VALIDATION_CONTRACT_PATCH: u32 = 0;

// ============================================================================
// Validation status
// ============================================================================

/// Overall status of an artifact validation operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ValidationStatus {
    /// Every required check performed by this validation operation passed.
    ///
    /// This status does not imply that unrelated semantic, security,
    /// backend, or runtime checks have been performed.
    Valid,

    /// At least one required check found a known violation.
    Invalid,

    /// No known violation was established, but one or more required
    /// checks could not be completed.
    Incomplete,
}

impl ValidationStatus {
    /// Returns true only when all required checks performed by this
    /// validation operation completed successfully.
    pub const fn is_valid(self) -> bool {
        matches!(self, Self::Valid)
    }

    /// Returns true when a known validation violation was found.
    pub const fn is_invalid(self) -> bool {
        matches!(self, Self::Invalid)
    }

    /// Returns true when validation could not establish every
    /// required property.
    pub const fn is_incomplete(self) -> bool {
        matches!(self, Self::Incomplete)
    }
}

// ============================================================================
// Finding severity
// ============================================================================

/// Severity of an artifact validation finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ValidationSeverity {
    /// The finding is informative and does not invalidate the artifact
    /// under the active policy.
    Warning,

    /// A required property could not be established.
    ///
    /// An incomplete result must not be treated as successful validation.
    Deferred,

    /// A known validation invariant or policy was violated.
    Error,
}

// ============================================================================
// Validation subject
// ============================================================================

/// Identifies the object associated with a validation finding.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ValidationSubject {
    /// An individual artifact.
    Artifact(ArtifactId),

    /// The manifest itself.
    Manifest(String),

    /// A manifest-level or operation-level condition.
    Validation,
}

impl fmt::Display for ValidationSubject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Artifact(id) => {
                write!(f, "artifact '{id}'")
            }

            Self::Manifest(id) => {
                write!(f, "manifest '{id}'")
            }

            Self::Validation => {
                f.write_str("artifact validation")
            }
        }
    }
}

// ============================================================================
// Finding classification
// ============================================================================

/// Machine-readable category of a validation finding.
///
/// Codes are stable integration identifiers. Consumers must not parse
/// human-readable messages to determine validation behavior.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ValidationIssueKind {
    /// Inline payload content did not match its declared digest.
    PayloadDigestMismatch,

    /// The digest algorithm is not supported by the current verifier.
    UnsupportedDigestAlgorithm,

    /// An external payload has not been resolved and verified.
    ExternalPayloadUnverified,

    /// The active policy requires inline payload content.
    InlinePayloadRequired,

    /// Metadata violated its configured validation policy.
    MetadataPolicyViolation,

    /// A metadata issue is a warning under the active policy.
    MetadataWarning,

    /// Required source provenance is missing.
    MissingSourceProvenance,

    /// Manifest structural validation failed.
    InvalidManifestStructure,

    /// A portability declaration or assessment is inconsistent.
    InvalidPortability,

    /// The portability policy rejected an artifact.
    PortabilityPolicyRejected,

    /// An artifact or manifest could not be fully checked.
    ValidationIncomplete,

    /// An unexpected validation provider or integration failed.
    ProviderFailure,
}

// ============================================================================
// Validation finding
// ============================================================================

/// One structured artifact validation finding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationIssue {
    subject: ValidationSubject,
    severity: ValidationSeverity,
    kind: ValidationIssueKind,
    code: String,
    message: String,
}

impl ValidationIssue {
    fn new(
        subject: ValidationSubject,
        severity: ValidationSeverity,
        kind: ValidationIssueKind,
        code: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            subject,
            severity,
            kind,
            code: code.into(),
            message: message.into(),
        }
    }

    /// Returns the validation subject.
    pub fn subject(&self) -> &ValidationSubject {
        &self.subject
    }

    /// Returns the finding severity.
    pub const fn severity(&self) -> ValidationSeverity {
        self.severity
    }

    /// Returns the finding classification.
    pub fn kind(&self) -> &ValidationIssueKind {
        &self.kind
    }

    /// Returns the stable machine-readable code.
    pub fn code(&self) -> &str {
        &self.code
    }

    /// Returns the human-readable explanation.
    pub fn message(&self) -> &str {
        &self.message
    }
}

// ============================================================================
// Validation report
// ============================================================================

/// Aggregate result of an artifact or manifest validation operation.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ValidationReport {
    issues: Vec<ValidationIssue>,
    artifacts_checked: usize,
    payloads_verified: usize,
    external_payloads_deferred: usize,
    metadata_entries_checked: usize,
}

impl ValidationReport {
    /// Creates an empty report.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the aggregate validation status.
    ///
    /// Known errors take precedence over incomplete checks.
    #[must_use]
    pub fn status(&self) -> ValidationStatus {
        if self.has_errors() {
            ValidationStatus::Invalid
        } else if self.is_incomplete() {
            ValidationStatus::Incomplete
        } else {
            ValidationStatus::Valid
        }
    }

    /// Returns true only when every required check completed successfully.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.status().is_valid()
    }

    /// Returns true when at least one known validation error exists.
    #[must_use]
    pub fn has_errors(&self) -> bool {
        self.issues.iter().any(|issue| {
            issue.severity() == ValidationSeverity::Error
        })
    }

    /// Returns true when at least one required check was deferred.
    #[must_use]
    pub fn is_incomplete(&self) -> bool {
        self.issues.iter().any(|issue| {
            issue.severity() == ValidationSeverity::Deferred
        })
    }

    /// Returns all findings in deterministic traversal order.
    #[must_use]
    pub fn issues(&self) -> &[ValidationIssue] {
        &self.issues
    }

    /// Returns the number of errors.
    #[must_use]
    pub fn error_count(&self) -> usize {
        self.count(ValidationSeverity::Error)
    }

    /// Returns the number of warnings.
    #[must_use]
    pub fn warning_count(&self) -> usize {
        self.count(ValidationSeverity::Warning)
    }

    /// Returns the number of deferred checks.
    #[must_use]
    pub fn deferred_count(&self) -> usize {
        self.count(ValidationSeverity::Deferred)
    }

    /// Counts findings of a particular severity.
    #[must_use]
    pub fn count(
        &self,
        severity: ValidationSeverity,
    ) -> usize {
        self.issues
            .iter()
            .filter(|issue| issue.severity() == severity)
            .count()
    }

    /// Returns the number of artifacts examined.
    #[must_use]
    pub const fn artifacts_checked(&self) -> usize {
        self.artifacts_checked
    }

    /// Returns the number of inline payloads whose digests were verified.
    #[must_use]
    pub const fn payloads_verified(&self) -> usize {
        self.payloads_verified
    }

    /// Returns the number of external payloads whose content could not
    /// be checked by this module.
    #[must_use]
    pub const fn external_payloads_deferred(&self) -> usize {
        self.external_payloads_deferred
    }

    /// Returns the number of metadata entries examined.
    #[must_use]
    pub const fn metadata_entries_checked(&self) -> usize {
        self.metadata_entries_checked
    }

    fn push(
        &mut self,
        issue: ValidationIssue,
    ) {
        self.issues.push(issue);
    }

    fn record_artifact(&mut self) {
        self.artifacts_checked =
            self.artifacts_checked.saturating_add(1);
    }

    fn record_verified_payload(&mut self) {
        self.payloads_verified =
            self.payloads_verified.saturating_add(1);
    }

    fn record_external_deferred(&mut self) {
        self.external_payloads_deferred =
            self.external_payloads_deferred.saturating_add(1);
    }

    fn record_metadata_entries(
        &mut self,
        count: usize,
    ) {
        self.metadata_entries_checked =
            self.metadata_entries_checked.saturating_add(count);
    }

    fn extend(
        &mut self,
        other: ValidationReport,
    ) {
        self.issues.extend(other.issues);

        self.artifacts_checked = self
            .artifacts_checked
            .saturating_add(other.artifacts_checked);

        self.payloads_verified = self
            .payloads_verified
            .saturating_add(other.payloads_verified);

        self.external_payloads_deferred = self
            .external_payloads_deferred
            .saturating_add(other.external_payloads_deferred);

        self.metadata_entries_checked = self
            .metadata_entries_checked
            .saturating_add(other.metadata_entries_checked);
    }
}

// ============================================================================
// Validation policy
// ============================================================================

/// Configurable policy for validating individual artifacts.
///
/// The default policy:
///
/// - Validates inline payload digests.
/// - Applies unrestricted metadata validation.
/// - Accepts legacy unnamespaced metadata with warnings.
/// - Does not require a source fingerprint.
/// - Does not pretend external payloads have been verified.
///
/// Operational limits are caller-configured. No arbitrary global ceiling
/// is introduced.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtifactValidationPolicy {
    metadata: MetadataValidationPolicy,
    require_inline_payload: bool,
    require_source_fingerprint: bool,
}

impl ArtifactValidationPolicy {
    /// Creates a policy with no additional operational restrictions.
    pub fn unrestricted() -> Self {
        Self {
            metadata: MetadataValidationPolicy::unrestricted(),
            require_inline_payload: false,
            require_source_fingerprint: false,
        }
    }

    /// Replaces the metadata validation policy.
    #[must_use]
    pub fn with_metadata_policy(
        mut self,
        policy: MetadataValidationPolicy,
    ) -> Self {
        self.metadata = policy;
        self
    }

    /// Controls whether an artifact must have inline payload bytes.
    ///
    /// When enabled, an external payload is rejected because this module
    /// does not retrieve external content.
    ///
    /// When disabled, an external payload is reported as deferred rather
    /// than incorrectly reported as verified.
    #[must_use]
    pub fn requiring_inline_payload(
        mut self,
        required: bool,
    ) -> Self {
        self.require_inline_payload = required;
        self
    }

    /// Controls whether an artifact must contain a source fingerprint
    /// in its canonical provenance record.
    #[must_use]
    pub fn requiring_source_fingerprint(
        mut self,
        required: bool,
    ) -> Self {
        self.require_source_fingerprint = required;
        self
    }

    /// Returns the metadata validation policy.
    pub fn metadata_policy(&self) -> &MetadataValidationPolicy {
        &self.metadata
    }

    /// Returns whether inline payload bytes are required.
    pub const fn requires_inline_payload(&self) -> bool {
        self.require_inline_payload
    }

    /// Returns whether source provenance is required.
    pub const fn requires_source_fingerprint(&self) -> bool {
        self.require_source_fingerprint
    }
}

impl Default for ArtifactValidationPolicy {
    fn default() -> Self {
        Self::unrestricted()
    }
}

/// Configurable policy for validating artifact manifests.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ManifestValidationPolicy {
    artifact: ArtifactValidationPolicy,
}

impl ManifestValidationPolicy {
    /// Creates a manifest policy using the supplied artifact policy.
    pub fn new(
        artifact: ArtifactValidationPolicy,
    ) -> Self {
        Self { artifact }
    }

    /// Returns the policy applied to every artifact in the manifest.
    pub fn artifact_policy(&self) -> &ArtifactValidationPolicy {
        &self.artifact
    }
}

// ============================================================================
// Error model
// ============================================================================

/// Errors raised while constructing or using validation policies.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ValidationError {
    /// A portability assessment could not be completed.
    Portability {
        message: String,
    },

    /// The requested portability policy rejected the assessment.
    PortabilityRejected {
        message: String,
    },
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Portability { message } => {
                write!(f, "portability assessment failed: {message}")
            }

            Self::PortabilityRejected { message } => {
                write!(f, "portability policy rejected artifact: {message}")
            }
        }
    }
}

impl Error for ValidationError {}

/// Result type for validation operations that can fail to construct
/// an assessment.
pub type ValidationResult<T> = Result<T, ValidationError>;

// ============================================================================
// Metadata validation integration
// ============================================================================

fn validate_artifact_metadata(
    subject: ValidationSubject,
    metadata: &ArtifactMetadata,
    policy: &ArtifactValidationPolicy,
    report: &mut ValidationReport,
) {
    report.record_metadata_entries(metadata.len());

    let metadata_report = validate_metadata(
        metadata,
        policy.metadata_policy(),
    );

    for issue in metadata_report.issues() {
        let (severity, kind, code) = match issue.severity() {
            MetadataIssueSeverity::Warning => (
                ValidationSeverity::Warning,
                ValidationIssueKind::MetadataWarning,
                "artifact.metadata.warning",
            ),

            MetadataIssueSeverity::Error => (
                ValidationSeverity::Error,
                ValidationIssueKind::MetadataPolicyViolation,
                "artifact.metadata.policy_violation",
            ),
        };

        let message = match issue.key() {
            Some(key) => {
                format!("{key}: {}", issue.message())
            }

            None => issue.message().to_owned(),
        };

        report.push(ValidationIssue::new(
            subject.clone(),
            severity,
            kind,
            code,
            message,
        ));
    }
}

// ============================================================================
// Individual artifact validation
// ============================================================================

/// Validates one canonical artifact.
///
/// Checks:
///
/// 1. Inline payload digest and size integrity.
/// 2. Metadata against the active metadata policy.
/// 3. Required source provenance.
/// 4. External payload verification state.
///
/// This function does not retrieve external payloads and does not claim
/// that artifact bytes are semantically valid IR or executable code.
pub fn validate_artifact(
    artifact: &Artifact,
    policy: &ArtifactValidationPolicy,
) -> ValidationReport {
    let mut report = ValidationReport::new();

    report.record_artifact();

    let subject = ValidationSubject::Artifact(
        artifact.id().clone(),
    );

    // 1. Validate canonical metadata.
    validate_artifact_metadata(
        subject.clone(),
        artifact.descriptor().metadata(),
        policy,
        &mut report,
    );

    // 2. Validate required provenance.
    if policy.requires_source_fingerprint()
        && artifact
            .descriptor()
            .provenance()
            .source_fingerprint()
            .is_none()
    {
        report.push(ValidationIssue::new(
            subject.clone(),
            ValidationSeverity::Error,
            ValidationIssueKind::MissingSourceProvenance,
            "artifact.provenance.source_fingerprint_missing",
            "the active validation policy requires a source fingerprint",
        ));
    }

    // 3. Validate payload integrity.
    if artifact.has_inline_payload() {
        match artifact.verify_inline_payload() {
            Ok(()) => {
                report.record_verified_payload();
            }

            Err(error) => {
                let message = error.to_string();

                let kind = if message.contains("digest algorithm") {
                    ValidationIssueKind::UnsupportedDigestAlgorithm
                } else {
                    ValidationIssueKind::PayloadDigestMismatch
                };

                let code = match kind {
                    ValidationIssueKind::UnsupportedDigestAlgorithm => {
                        "artifact.payload.unsupported_digest_algorithm"
                    }

                    _ => "artifact.payload.integrity_failure",
                };

                report.push(ValidationIssue::new(
                    subject.clone(),
                    ValidationSeverity::Error,
                    kind,
                    code,
                    message,
                ));
            }
        }
    } else if artifact.has_external_payload() {
        if policy.requires_inline_payload() {
            report.push(ValidationIssue::new(
                subject.clone(),
                ValidationSeverity::Error,
                ValidationIssueKind::InlinePayloadRequired,
                "artifact.payload.inline_required",
                "the active policy requires inline payload bytes, \
                 but the artifact refers to external content",
            ));
        } else {
            report.record_external_deferred();

            report.push(ValidationIssue::new(
                subject,
                ValidationSeverity::Deferred,
                ValidationIssueKind::ExternalPayloadUnverified,
                "artifact.payload.external_unverified",
                "external payload content has not been retrieved and \
                 verified by an artifact storage provider",
            ));
        }
    }

    report
}

// ============================================================================
// Artifact portability validation integration
// ============================================================================

/// Validates an artifact and optionally evaluates its portability.
///
/// Portability declarations and evidence are supplied explicitly.
///
/// The portability subsystem remains the authority for classification.
/// This function only coordinates its result with artifact validation.
///
/// If a portability policy is supplied, the assessment must satisfy it.
pub fn validate_artifact_with_portability(
    artifact: &Artifact,
    policy: &ArtifactValidationPolicy,
    declaration: Option<&super::portability::PortabilityDeclaration>,
    evidence: Option<&super::portability::PortabilityEvidence>,
    portability_policy: &PortabilityPolicy,
) -> ValidationResult<ValidationReport> {
    let mut report = validate_artifact(artifact, policy);

    let subject = ValidationSubject::Artifact(
        artifact.id().clone(),
    );

    let assessment = assess_artifact(
        artifact,
        declaration,
        evidence,
    )
    .map_err(|error| ValidationError::Portability {
        message: error.to_string(),
    })?;

    if let Err(error) = portability_policy.evaluate_artifact(&assessment) {
        report.push(ValidationIssue::new(
            subject,
            ValidationSeverity::Error,
            ValidationIssueKind::PortabilityPolicyRejected,
            "artifact.portability.policy_rejected",
            error.to_string(),
        ));
    }

    Ok(report)
}

// ============================================================================
// Manifest validation
// ============================================================================

/// Validates a canonical artifact manifest.
///
/// The manifest's own structural validator is invoked first.
///
/// Every contained artifact is then validated using the same artifact
/// policy. Manifest metadata is checked using the same metadata policy.
///
/// Structural failure does not prevent independent artifact diagnostics
/// from being collected.
pub fn validate_manifest(
    manifest: &ArtifactManifest,
    policy: &ManifestValidationPolicy,
) -> ValidationReport {
    let mut report = ValidationReport::new();

    let manifest_subject = ValidationSubject::Manifest(
        manifest.id().to_string(),
    );

    // 1. Validate manifest structure.
    if let Err(error) = manifest.validate() {
        report.push(ValidationIssue::new(
            manifest_subject.clone(),
            ValidationSeverity::Error,
            ValidationIssueKind::InvalidManifestStructure,
            "artifact.manifest.structure_invalid",
            error.to_string(),
        ));
    }

    // 2. Validate manifest-level metadata.
    validate_artifact_metadata(
        manifest_subject,
        manifest.metadata(),
        policy.artifact_policy(),
        &mut report,
    );

    // 3. Validate every contained artifact in canonical manifest order.
    for (_, artifact) in manifest.artifacts() {
        let artifact_report = validate_artifact(
            artifact,
            policy.artifact_policy(),
        );

        report.extend(artifact_report);
    }

    report
}

// ============================================================================
// Manifest portability validation integration
// ============================================================================

/// Validates a manifest and evaluates its portability policy.
///
/// The manifest's portability inputs are supplied by the caller and are
/// indexed by canonical ArtifactId.
///
/// Missing declarations or evidence are not interpreted as proof of
/// portability.
///
/// Portability classification and acceptance remain owned by
/// portability.rs.
pub fn validate_manifest_with_portability(
    manifest: &ArtifactManifest,
    policy: &ManifestValidationPolicy,
    inputs: &ManifestPortabilityInputs,
    portability_policy: &PortabilityPolicy,
) -> ValidationResult<ValidationReport> {
    let mut report = validate_manifest(manifest, policy);

    let subject = ValidationSubject::Manifest(
        manifest.id().to_string(),
    );

    let portability_report = assess_manifest(
        manifest,
        inputs,
    )
    .map_err(|error| ValidationError::Portability {
        message: error.to_string(),
    })?;

    for assessment in portability_report.assessments() {
        if let Err(error) =
            portability_policy.evaluate_artifact(assessment)
        {
            report.push(ValidationIssue::new(
                ValidationSubject::Artifact(
                    assessment.artifact().clone(),
                ),
                ValidationSeverity::Error,
                ValidationIssueKind::PortabilityPolicyRejected,
                "artifact.manifest.artifact_portability_rejected",
                error.to_string(),
            ));
        }
    }

    // Preserve an explicit manifest-level finding when the aggregate
    // classification is mixed or unresolved.
    if matches!(
        portability_report.class(),
        PortabilityClass::Inconsistent | PortabilityClass::Mixed
    ) {
        report.push(ValidationIssue::new(
            subject,
            ValidationSeverity::Error,
            ValidationIssueKind::InvalidPortability,
            "artifact.manifest.portability_inconsistent",
            format!(
                "manifest portability classification is '{}'",
                portability_report.class()
            ),
        ));
    }

    Ok(report)
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::compiler::artifacts::artifact::{
        ArtifactDescriptor,
        ArtifactFormatId,
        ArtifactId,
        ArtifactKindId,
        ArtifactProducerId,
        ArtifactProvenance,
        ArtifactTargetRef,
        ArtifactPayloadRef,
        ArtifactStoreId,
        ArtifactContentId,
        ArtifactDigest,
    };

    use crate::compiler::manifest::{
        ArtifactManifest,
        ManifestId,
    };

    fn make_artifact(
        id: &str,
        bytes: Vec<u8>,
    ) -> Artifact {
        let provenance = ArtifactProvenance::new(
            ArtifactProducerId::new("zamani.test").expect("producer"),
        );

        let descriptor = ArtifactDescriptor::new(
            ArtifactId::new(id).expect("artifact ID"),
            ArtifactKindId::new("portable-ir").expect("artifact kind"),
            ArtifactFormatId::new("application/x-zamani-test")
                .expect("artifact format"),
            provenance,
        );

        Artifact::new_inline(descriptor, bytes)
    }

    fn make_manifest() -> ArtifactManifest {
        ArtifactManifest::new(
            ManifestId::new("manifest.test").expect("manifest ID"),
            ArtifactProvenance::new(
                ArtifactProducerId::new("zamani.test")
                    .expect("producer"),
            ),
        )
    }

    #[test]
    fn valid_inline_artifact_passes_integrity_validation() {
        let artifact = make_artifact(
            "artifact.valid",
            b"valid payload".to_vec(),
        );

        let report = validate_artifact(
            &artifact,
            &ArtifactValidationPolicy::default(),
        );

        assert_eq!(report.status(), ValidationStatus::Valid);
        assert_eq!(report.artifacts_checked(), 1);
        assert_eq!(report.payloads_verified(), 1);
        assert!(!report.has_errors());
    }

    #[test]
    fn external_payload_is_not_reported_as_verified() {
        let provenance = ArtifactProvenance::new(
            ArtifactProducerId::new("zamani.test").expect("producer"),
        );

        let descriptor = ArtifactDescriptor::new(
            ArtifactId::new("artifact.external").expect("artifact ID"),
            ArtifactKindId::new("portable-ir").expect("artifact kind"),
            ArtifactFormatId::new("application/x-zamani-test")
                .expect("artifact format"),
            provenance,
        );

        let reference = ArtifactPayloadRef::new(
            ArtifactStoreId::new("test.store").expect("store"),
            ArtifactContentId::new("content.1").expect("content"),
            Some(128),
        );

        let artifact = Artifact::new_external(
            descriptor,
            reference,
            ArtifactDigest::sha256(b"expected external bytes"),
        )
        .expect("external artifact");

        let report = validate_artifact(
            &artifact,
            &ArtifactValidationPolicy::default(),
        );

        assert_eq!(report.status(), ValidationStatus::Incomplete);
        assert_eq!(report.payloads_verified(), 0);
        assert_eq!(report.external_payloads_deferred(), 1);
        assert_eq!(report.deferred_count(), 1);
    }

    #[test]
    fn policy_can_require_inline_payloads() {
        let provenance = ArtifactProvenance::new(
            ArtifactProducerId::new("zamani.test").expect("producer"),
        );

        let descriptor = ArtifactDescriptor::new(
            ArtifactId::new("artifact.external.required")
                .expect("artifact ID"),
            ArtifactKindId::new("portable-ir").expect("artifact kind"),
            ArtifactFormatId::new("application/x-zamani-test")
                .expect("artifact format"),
            provenance,
        );

        let reference = ArtifactPayloadRef::new(
            ArtifactStoreId::new("test.store").expect("store"),
            ArtifactContentId::new("content.2").expect("content"),
            None,
        );

        let artifact = Artifact::new_external(
            descriptor,
            reference,
            ArtifactDigest::sha256(b"content"),
        )
        .expect("external artifact");

        let policy = ArtifactValidationPolicy::default()
            .requiring_inline_payload(true);

        let report = validate_artifact(&artifact, &policy);

        assert_eq!(report.status(), ValidationStatus::Invalid);
        assert_eq!(report.error_count(), 1);
    }

    #[test]
    fn required_source_fingerprint_is_enforced() {
        let artifact = make_artifact(
            "artifact.no-source-fingerprint",
            b"payload".to_vec(),
        );

        let policy = ArtifactValidationPolicy::default()
            .requiring_source_fingerprint(true);

        let report = validate_artifact(&artifact, &policy);

        assert_eq!(report.status(), ValidationStatus::Invalid);
        assert!(report.issues().iter().any(|issue| {
            issue.kind()
                == &ValidationIssueKind::MissingSourceProvenance
        }));
    }

    #[test]
    fn manifest_validation_checks_contained_artifacts() {
        let mut manifest = make_manifest();

        manifest
            .insert_artifact(make_artifact(
                "artifact.one",
                b"one".to_vec(),
            ))
            .expect("insert first artifact");

        manifest
            .insert_artifact(make_artifact(
                "artifact.two",
                b"two".to_vec(),
            ))
            .expect("insert second artifact");

        let report = validate_manifest(
            &manifest,
            &ManifestValidationPolicy::default(),
        );

        assert_eq!(report.status(), ValidationStatus::Valid);
        assert_eq!(report.artifacts_checked(), 2);
        assert_eq!(report.payloads_verified(), 2);
    }

    #[test]
    fn empty_manifest_is_structurally_valid() {
        let manifest = make_manifest();

        let report = validate_manifest(
            &manifest,
            &ManifestValidationPolicy::default(),
        );

        assert_eq!(report.status(), ValidationStatus::Valid);
        assert_eq!(report.artifacts_checked(), 0);
    }

    #[test]
    fn metadata_policy_is_applied_to_artifact_metadata() {
        let artifact = make_artifact(
            "artifact.metadata",
            b"payload".to_vec(),
        );

        let metadata_policy = MetadataValidationPolicy::unrestricted()
            .requiring_namespaced_keys(true);

        let policy = ArtifactValidationPolicy::default()
            .with_metadata_policy(metadata_policy);

        let report = validate_artifact(&artifact, &policy);

        assert_eq!(report.status(), ValidationStatus::Valid);
    }

    #[test]
    fn metadata_policy_is_applied_to_manifest_metadata() {
        let mut manifest = make_manifest();

        manifest
            .metadata_mut()
            .insert("legacy-key", "legacy-value")
            .expect("metadata insertion");

        let metadata_policy = MetadataValidationPolicy::unrestricted()
            .requiring_namespaced_keys(true);

        let policy = ManifestValidationPolicy::new(
            ArtifactValidationPolicy::default()
                .with_metadata_policy(metadata_policy),
        );

        let report = validate_manifest(&manifest, &policy);

        assert_eq!(report.status(), ValidationStatus::Invalid);
        assert!(report.error_count() > 0);
    }

    #[test]
    fn validation_does_not_claim_portability_without_evidence() {
        let artifact = make_artifact(
            "artifact.unknown-portability",
            b"payload".to_vec(),
        );

        let portability_policy = PortabilityPolicy::default();

        let result = validate_artifact_with_portability(
            &artifact,
            &ArtifactValidationPolicy::default(),
            None,
            None,
            &portability_policy,
        );

        let report = result.expect("assessment operation succeeds");

        assert_eq!(report.status(), ValidationStatus::Invalid);
        assert!(report.issues().iter().any(|issue| {
            issue.kind()
                == &ValidationIssueKind::PortabilityPolicyRejected
        }));
    }

    #[test]
    fn no_fixed_payload_or_artifact_count_ceiling_is_introduced() {
        let payload = vec![0_u8; 1024 * 1024];

        let artifact = make_artifact(
            "artifact.large",
            payload,
        );

        let report = validate_artifact(
            &artifact,
            &ArtifactValidationPolicy::default(),
        );

        assert_eq!(report.status(), ValidationStatus::Valid);
        assert_eq!(report.payloads_verified(), 1);
    }
}
