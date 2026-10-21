
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Backend precondition and compatibility validation for Zamani.
//!
//! Path: `src/backend/validation.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! # Responsibility
//!
//! Validate backend compilation preconditions before lowering or emission:
//!
//! - backend context and configuration invariants;
//! - input representation verification evidence;
//! - target descriptor validity and implementation availability;
//! - mandatory capability requirements;
//! - declared artifact-format compatibility;
//! - external-tool configuration preconditions;
//! - explicit reporting of checks that remain unresolved.
//!
//! # Ownership boundaries
//!
//! This module does not parse source, define language semantics, replace an
//! IR verifier, implement target selection, execute external tools, prove
//! physical resource feasibility, allocate hardware, perform lowering, or
//! validate generated artifact bytes.
//!
//! The grammar and semantic specifications remain authoritative for source
//! meaning. Domain-specific validators remain responsible for quantum,
//! classical, HDL, tensor, distributed, and future-domain invariants.
//!
//! # Important distinction
//!
//! A successful validation report means only that the checks represented by
//! that report passed. It does not establish that a toolchain is installed,
//! that hardware resources are available, or that generated output is valid.
//! Those guarantees require their own checks.
//!
//! # Scalability
//!
//! No universal limits are imposed on hardware capacity, program size,
//! number of domains, qubits, CPUs, devices, nodes, tensor rank, or register
//! width. Consumers may enforce configured operational budgets.
//!
//! # Integration
//!
//! - `context.rs`: immutable validated compilation context.
//! - `input.rs`: verified heterogeneous backend input.
//! - `target.rs`: target descriptor, availability, and artifact formats.
//! - `capabilities.rs`: capability identifiers and matching policy.
//! - `requirements.rs`: mandatory capabilities, constraints, resources,
//!   preferences, and hints.
//! - `config.rs`: backend configuration and external-tool declarations.
//! - `errors.rs`: structured backend failures.
//! - `pipeline.rs`: call these checks before lowering or emission.
//! - `registry.rs`: independently establishes whether an emitter is
//!   registered; a target descriptor alone does not establish that fact.
//!
//! The canonical backend module root must declare `pub mod validation;`.

use std::collections::BTreeSet;

use super::capabilities::{
    CapabilityCheckResult, CapabilityId, CapabilityOutcome, CapabilityPolicy,
    CapabilityRequirement,
};
use super::config::{
    BackendConfig, ExternalToolPolicy, ToolchainKey,
};
use super::context::BackendContext;
use super::errors::{BackendError, BackendErrorKind, BackendResult};
use super::input::BackendInput;
use super::requirements::{RequirementKind, RequirementSet};
use super::target::{
    ArtifactFormat, ImplementationAvailability, TargetDescriptor,
};

/// The severity of one validation finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ValidationSeverity {
    /// A required condition has failed.
    Error,

    /// A check cannot be completed at this layer and must be completed by a
    /// downstream validator before the compilation can be considered ready.
    Deferred,

    /// An optional preference or hint has not been evaluated here.
    Informational,

    /// A non-blocking observation about the current target or configuration.
    Warning,
}

/// Stable category for a validation finding.
///
/// These categories are deliberately independent of diagnostic presentation
/// and target-specific implementation details.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum ValidationIssueKind {
    InvalidContext,
    InvalidInput,
    MissingVerificationEvidence,
    InvalidTarget,
    TargetUnavailable,
    EmitterNotRegistered,
    MissingCapability,
    UnavailableCapability,
    DisallowedCapabilityStatus,
    InsufficientCapabilityEvidence,
    UnsupportedArtifactFormat,
    ToolchainNotConfigured,
    ExternalToolsDisallowed,
    ConstraintNeedsFeasibilityCheck,
    ResourceNeedsFeasibilityCheck,
    PreferenceNotEvaluated,
    HintNotEvaluated,
    Other,
}

/// One finding emitted by backend validation.
///
/// `message` is suitable for diagnostics, but callers should use `kind` and
/// `severity` for programmatic decisions rather than parsing the message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationIssue {
    kind: ValidationIssueKind,
    severity: ValidationSeverity,
    message: String,
    target: Option<String>,
    subject: Option<String>,
}

impl ValidationIssue {
    /// Construct a validation finding.
    pub fn new(
        kind: ValidationIssueKind,
        severity: ValidationSeverity,
        message: impl Into<String>,
    ) -> BackendResult<Self> {
        let message = message.into();

        if message.trim().is_empty() {
            return Err(BackendError::invalid_input(
                "validation issue message must not be empty",
            ));
        }

        Ok(Self {
            kind,
            severity,
            message,
            target: None,
            subject: None,
        })
    }

    /// Attach the target identifier associated with this finding.
    #[must_use]
    pub fn with_target(mut self, target: impl Into<String>) -> Self {
        self.target = Some(target.into());
        self
    }

    /// Attach a requirement, capability, artifact format, or toolchain key.
    #[must_use]
    pub fn with_subject(mut self, subject: impl Into<String>) -> Self {
        self.subject = Some(subject.into());
        self
    }

    pub const fn kind(&self) -> ValidationIssueKind {
        self.kind
    }

    pub const fn severity(&self) -> ValidationSeverity {
        self.severity
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn target(&self) -> Option<&str> {
        self.target.as_deref()
    }

    pub fn subject(&self) -> Option<&str> {
        self.subject.as_deref()
    }

    pub const fn is_blocking(&self) -> bool {
        matches!(
            self.severity,
            ValidationSeverity::Error | ValidationSeverity::Deferred
        )
    }
}

/// The result of a collection of validation checks.
///
/// Reports preserve issue insertion order. Check names are stored in a
/// sorted set so the completed-check summary is deterministic.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ValidationReport {
    completed_checks: BTreeSet<String>,
    issues: Vec<ValidationIssue>,
}

impl ValidationReport {
    pub fn new() -> Self {
        Self::default()
    }

    /// Record a completed check by its stable descriptive name.
    ///
    /// Empty names are rejected to avoid ambiguous or meaningless reports.
    pub fn record_check(
        &mut self,
        name: impl Into<String>,
    ) -> BackendResult<()> {
        let name = name.into();

        if name.trim().is_empty() || name.trim() != name {
            return Err(BackendError::invalid_input(
                "validation check name must be nonempty and have no surrounding whitespace",
            ));
        }

        self.completed_checks.insert(name);
        Ok(())
    }

    /// Append one validation finding.
    pub fn push_issue(&mut self, issue: ValidationIssue) {
        self.issues.push(issue);
    }

    /// Append findings from another report.
    ///
    /// Completed-check names are merged. Findings are not silently deduplicated.
    pub fn merge(&mut self, other: Self) {
        self.completed_checks.extend(other.completed_checks);
        self.issues.extend(other.issues);
    }

    pub fn completed_checks(&self) -> &BTreeSet<String> {
        &self.completed_checks
    }

    pub fn issues(&self) -> &[ValidationIssue] {
        &self.issues
    }

    pub fn errors(&self) -> impl Iterator<Item = &ValidationIssue> {
        self.issues
            .iter()
            .filter(|issue| issue.severity() == ValidationSeverity::Error)
    }

    pub fn deferred(&self) -> impl Iterator<Item = &ValidationIssue> {
        self.issues
            .iter()
            .filter(|issue| issue.severity() == ValidationSeverity::Deferred)
    }

    pub fn warnings(&self) -> impl Iterator<Item = &ValidationIssue> {
        self.issues
            .iter()
            .filter(|issue| issue.severity() == ValidationSeverity::Warning)
    }

    pub fn informational(&self) -> impl Iterator<Item = &ValidationIssue> {
        self.issues
            .iter()
            .filter(|issue| issue.severity() == ValidationSeverity::Informational)
    }

    /// Whether no validation check has reported a failed condition.
    ///
    /// Deferred checks do not make this method return false; use
    /// `is_complete()` or `ensure_ready()` when readiness is required.
    pub fn is_valid(&self) -> bool {
        self.errors().next().is_none()
    }

    /// Whether the report contains neither errors nor unresolved checks.
    pub fn is_complete(&self) -> bool {
        self.is_valid() && self.deferred().next().is_none()
    }

    pub fn has_blockers(&self) -> bool {
        self.issues.iter().any(ValidationIssue::is_blocking)
    }

    /// Require all reported checks to pass or be completed.
    ///
    /// This method cannot infer checks that were never requested. Callers
    /// must invoke every validator required by their compilation pipeline.
    pub fn ensure_ready(&self) -> BackendResult<()> {
        if self.is_complete() {
            return Ok(());
        }

        let details = self
            .issues
            .iter()
            .filter(|issue| issue.is_blocking())
            .map(|issue| {
                format!(
                    "{:?}: {}",
                    issue.kind(),
                    issue.message()
                )
            })
            .collect::<Vec<_>>()
            .join("; ");

        Err(BackendError::new(
            BackendErrorKind::VerificationFailed,
            if details.is_empty() {
                "backend validation is incomplete"
            } else {
                &details
            },
        )
        .with_context("requiring a complete backend validation report"))
    }
}

/// Stateless backend validator.
///
/// The validator holds no global state and can be used independently by
/// parallel compilation requests.
#[derive(Debug, Clone, Copy, Default)]
pub struct BackendValidator;

impl BackendValidator {
    pub const fn new() -> Self {
        Self
    }

    /// Validate the context's configuration and target descriptor.
    ///
    /// This does not prove that a target emitter is registered, that the
    /// target can implement a particular workload, or that its toolchain is
    /// installed.
    pub fn validate_context(
        &self,
        context: &BackendContext,
    ) -> BackendResult<ValidationReport> {
        context.validate().map_err(|error| {
            error.with_context("backend context precondition validation")
        })?;

        let mut report = ValidationReport::new();
        report.record_check("backend.context")?;
        report.record_check("backend.configuration")?;
        report.record_check("backend.target_descriptor")?;

        match context.target().availability() {
            ImplementationAvailability::Unavailable => {
                return Err(BackendError::unsupported_target(
                    context.target().id().as_str(),
                    "the target descriptor explicitly marks the legacy implementation unavailable",
                )
                .with_context("validating backend context"));
            }
            ImplementationAvailability::DescriptorOnly => {
                report.push_issue(
                    ValidationIssue::new(
                        ValidationIssueKind::EmitterNotRegistered,
                        ValidationSeverity::Deferred,
                        "target metadata is valid, but emitter availability must be established by the backend registry",
                    )?
                    .with_target(context.target().id().as_str()),
                );
            }
            ImplementationAvailability::LegacyEmitterPresent => {}
        }

        Ok(report)
    }

    /// Validate a heterogeneous input's recorded verification evidence.
    ///
    /// Representation-specific verification is performed by the verifier
    /// supplied when each `VerifiedUnit` was constructed. This method checks
    /// that the input still satisfies the backend input contract; it does not
    /// rerun an arbitrary verifier or reinterpret representation payloads.
    pub fn validate_input(
        &self,
        input: &BackendInput,
    ) -> BackendResult<ValidationReport> {
        if input.unit_count() == 0 {
            return Err(BackendError::invalid_input(
                "backend input contains no verified representation units",
            ));
        }

        if !input.has_verification_evidence() {
            return Err(BackendError::verification_failed(
                "one or more input units lack complete recorded verification evidence",
            ));
        }

        let mut report = ValidationReport::new();

        for (index, unit) in input.units().iter().enumerate() {
            let verification = unit.verification();

            if verification.verifier_id().trim().is_empty()
                || verification.verifier_version().trim().is_empty()
                || verification.completed_checks().is_empty()
            {
                return Err(BackendError::verification_failed(format!(
                    "input unit {index} has incomplete verification evidence"
                ))
                .with_context(format!(
                    "validating representation `{}`",
                    unit.descriptor().format()
                )));
            }

            report.record_check(format!("input.unit.{index}.verification_evidence"))?;
        }

        report.record_check("backend.input_nonempty")?;
        report.record_check("backend.input_verification_evidence")?;

        Ok(report)
    }

    /// Validate a target descriptor independently of a compilation request.
    ///
    /// `DescriptorOnly` is not treated as an invalid descriptor: it means
    /// that a separate registry/emitter check is still required.
    pub fn validate_target_descriptor(
        &self,
        target: &TargetDescriptor,
    ) -> BackendResult<ValidationReport> {
        target.validate().map_err(|error| {
            error
                .with_target(target.id().as_str())
                .with_context("validating target descriptor")
        })?;

        let mut report = ValidationReport::new();
        report.record_check("backend.target_descriptor")?;

        match target.availability() {
            ImplementationAvailability::Unavailable => {
                return Err(BackendError::unsupported_target(
                    target.id().as_str(),
                    "target implementation is explicitly unavailable",
                ));
            }
            ImplementationAvailability::DescriptorOnly => {
                report.push_issue(
                    ValidationIssue::new(
                        ValidationIssueKind::EmitterNotRegistered,
                        ValidationSeverity::Deferred,
                        "target descriptor does not establish emitter availability",
                    )?
                    .with_target(target.id().as_str()),
                );
            }
            ImplementationAvailability::LegacyEmitterPresent => {}
        }

        Ok(report)
    }

    /// Validate that an emitter is registered for a target before codegen.
    ///
    /// `emitter_registered` must be supplied by the authoritative backend
    /// registry. This method does not create or discover emitters.
    pub fn validate_codegen_target(
        &self,
        target: &TargetDescriptor,
        emitter_registered: bool,
    ) -> BackendResult<ValidationReport> {
        let mut report = self.validate_target_descriptor(target)?;

        if !emitter_registered {
            return Err(BackendError::unsupported_target(
                target.id().as_str(),
                "no registered emitter is available for the requested target",
            )
            .with_context("validating code-generation preconditions"));
        }

        if target.availability() == ImplementationAvailability::Unavailable {
            return Err(BackendError::unsupported_target(
                target.id().as_str(),
                "the target descriptor explicitly marks the implementation unavailable",
            ));
        }

        report.record_check("backend.emitter_registered")?;

        // Registry evidence supersedes the descriptor-only observation.
        report.issues.retain(|issue| {
            issue.kind() != ValidationIssueKind::EmitterNotRegistered
        });

        Ok(report)
    }

    /// Validate a list of mandatory capabilities against the selected target.
    ///
    /// Preferences, constraints, hints, and resource feasibility are not
    /// inferred from this list. They have separate contracts.
    pub fn validate_capabilities(
        &self,
        target: &TargetDescriptor,
        requirements: &[CapabilityRequirement],
        policy: CapabilityPolicy,
    ) -> BackendResult<ValidationReport> {
        target.validate()?;

        let result = target
            .capability_profile()
            .check(requirements, policy);

        self.capability_result_report(target, &result)
    }

    fn capability_result_report(
        &self,
        target: &TargetDescriptor,
        result: &CapabilityCheckResult,
    ) -> BackendResult<ValidationReport> {
        let mut report = ValidationReport::new();
        report.record_check("backend.required_capabilities")?;

        for finding in result.failures() {
            let (kind, message) = match finding.outcome() {
                CapabilityOutcome::Missing => (
                    ValidationIssueKind::MissingCapability,
                    format!(
                        "required capability `{}` is not declared by the target",
                        finding.id()
                    ),
                ),
                CapabilityOutcome::Unavailable => (
                    ValidationIssueKind::UnavailableCapability,
                    format!(
                        "required capability `{}` is explicitly unavailable",
                        finding.id()
                    ),
                ),
                CapabilityOutcome::DisallowedStatus(status) => (
                    ValidationIssueKind::DisallowedCapabilityStatus,
                    format!(
                        "required capability `{}` has status `{status}` disallowed by policy",
                        finding.id()
                    ),
                ),
                CapabilityOutcome::InsufficientEvidence => (
                    ValidationIssueKind::InsufficientCapabilityEvidence,
                    format!(
                        "required capability `{}` lacks the direct evidence required by policy",
                        finding.id()
                    ),
                ),
                CapabilityOutcome::Satisfied => continue,
                _ => (
                    ValidationIssueKind::Other,
                    format!(
                        "required capability `{}` has an unrecognized matching outcome",
                        finding.id()
                    ),
                ),
            };

            report.push_issue(
                ValidationIssue::new(
                    kind,
                    ValidationSeverity::Error,
                    message,
                )?
                .with_target(target.id().as_str())
                .with_subject(finding.id().as_str()),
            );
        }

        Ok(report)
    }

    /// Validate the mandatory capability requirements in a `RequirementSet`.
    ///
    /// Constraint and resource requirements are reported as deferred because
    /// this module has no authoritative target-resource or feasibility model.
    /// Preferences and hints are informational and cannot block a valid
    /// compilation merely because they were not evaluated here.
    pub fn validate_requirements(
        &self,
        target: &TargetDescriptor,
        requirements: &RequirementSet,
        policy: CapabilityPolicy,
    ) -> BackendResult<ValidationReport> {
        target.validate()?;

        let mut capabilities = Vec::new();
        for requirement in requirements {
            if requirement.kind() == RequirementKind::RequiredCapability {
                let id = CapabilityId::new(requirement.key().as_str()).map_err(|error| {
                    BackendError::new(
                        BackendErrorKind::InvalidInput,
                        format!(
                            "invalid capability requirement `{}`: {error}",
                            requirement.key()
                        ),
                    )
                })?;
                capabilities.push(CapabilityRequirement::new(id));
            }
        }

        let mut report =
            self.capability_result_report(
                target,
                &target.capability_profile().check(&capabilities, policy),
            )?;

        for requirement in requirements {
            match requirement.kind() {
                RequirementKind::RequiredCapability => {
                    // Already checked against the target capability profile.
                }
                RequirementKind::Constraint => {
                    report.push_issue(
                        ValidationIssue::new(
                            ValidationIssueKind::ConstraintNeedsFeasibilityCheck,
                            ValidationSeverity::Deferred,
                            format!(
                                "constraint `{}` requires evaluation by the target compatibility or feasibility layer",
                                requirement.key()
                            ),
                        )?
                        .with_target(target.id().as_str())
                        .with_subject(requirement.key().as_str()),
                    );
                }
                RequirementKind::Resource => {
                    report.push_issue(
                        ValidationIssue::new(
                            ValidationIssueKind::ResourceNeedsFeasibilityCheck,
                            ValidationSeverity::Deferred,
                            format!(
                                "resource requirement `{}` needs authoritative resource-feasibility evidence",
                                requirement.key()
                            ),
                        )?
                        .with_target(target.id().as_str())
                        .with_subject(requirement.key().as_str()),
                    );
                }
                RequirementKind::Preference => {
                    report.push_issue(
                        ValidationIssue::new(
                            ValidationIssueKind::PreferenceNotEvaluated,
                            ValidationSeverity::Informational,
                            format!(
                                "preference `{}` has not been ranked by target selection",
                                requirement.key()
                            ),
                        )?
                        .with_target(target.id().as_str())
                        .with_subject(requirement.key().as_str()),
                    );
                }
                RequirementKind::Hint => {
                    report.push_issue(
                        ValidationIssue::new(
                            ValidationIssueKind::HintNotEvaluated,
                            ValidationSeverity::Informational,
                            format!(
                                "hint `{}` has not been evaluated by a target-specific optimization stage",
                                requirement.key()
                            ),
                        )?
                        .with_target(target.id().as_str())
                        .with_subject(requirement.key().as_str()),
                    );
                }
            }
        }

        report.record_check("backend.requirement_categories")?;
        Ok(report)
    }

    /// Check that a target declares support for an artifact format.
    ///
    /// This checks the descriptor contract only. It does not verify generated
    /// artifact contents or claim that the corresponding emitter works.
    pub fn validate_artifact_format(
        &self,
        target: &TargetDescriptor,
        format: &ArtifactFormat,
    ) -> BackendResult<ValidationReport> {
        target.validate()?;

        if !target.supports_artifact_format(format) {
            return Err(BackendError::new(
                BackendErrorKind::TargetMismatch,
                format!(
                    "target `{}` does not declare artifact format `{}`",
                    target.id(),
                    format
                ),
            )
            .with_target(target.id().as_str())
            .with_context("validating requested artifact format"));
        }

        let mut report = ValidationReport::new();
        report.record_check("backend.artifact_format_declared")?;
        Ok(report)
    }

    /// Verify that an external tool is permitted and explicitly configured.
    ///
    /// This function deliberately does not test executable existence, version,
    /// permissions, or behavior. Those checks belong to `toolchain.rs` and
    /// the process execution layer.
    pub fn validate_toolchain_configured(
        &self,
        config: &BackendConfig,
        key: &ToolchainKey,
    ) -> BackendResult<ValidationReport> {
        config.validate().map_err(|error| {
            BackendError::invalid_configuration(error.to_string())
                .with_context("validating toolchain configuration")
        })?;

        if config.external_tools == ExternalToolPolicy::Disallow {
            return Err(BackendError::new(
                BackendErrorKind::ToolchainUnavailable,
                format!(
                    "external tool execution is disallowed; toolchain `{key}` cannot be used"
                ),
            )
            .with_context("validating external tool preconditions"));
        }

        let Some(toolchain) = config.toolchain(key) else {
            return Err(BackendError::toolchain_unavailable(
                key.as_str(),
                format!("toolchain `{key}` is not explicitly configured"),
            ));
        };

        toolchain.validate().map_err(|error| {
            BackendError::invalid_configuration(error.to_string())
                .with_context(format!("validating toolchain `{key}`"))
        })?;

        let mut report = ValidationReport::new();
        report.record_check("backend.external_tool_policy")?;
        report.record_check("backend.toolchain_declared")?;
        Ok(report)
    }

    /// Run the representation and context preconditions as one stage.
    ///
    /// The caller must still validate workload requirements, emitter
    /// registration, toolchain availability, and output artifacts as required
    /// by the selected pipeline.
    pub fn validate_request(
        &self,
        context: &BackendContext,
        input: &BackendInput,
    ) -> BackendResult<ValidationReport> {
        let mut report = self.validate_context(context)?;
        report.merge(self.validate_input(input)?);
        report.record_check("backend.request_preconditions")?;
        Ok(report)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ValidationIssue, ValidationIssueKind, ValidationReport, ValidationSeverity,
    };

    #[test]
    fn report_is_valid_when_only_informational_findings_exist() {
        let mut report = ValidationReport::new();
        report
            .push_issue(
                ValidationIssue::new(
                    ValidationIssueKind::HintNotEvaluated,
                    ValidationSeverity::Informational,
                    "optional optimization hint was not evaluated",
                )
                .expect("valid issue"),
            );

        assert!(report.is_valid());
        assert!(report.is_complete());
        assert!(!report.has_blockers());
        assert!(report.ensure_ready().is_ok());
    }

    #[test]
    fn deferred_findings_prevent_readiness_without_becoming_errors() {
        let mut report = ValidationReport::new();
        report
            .push_issue(
                ValidationIssue::new(
                    ValidationIssueKind::ResourceNeedsFeasibilityCheck,
                    ValidationSeverity::Deferred,
                    "resource feasibility has not been checked",
                )
                .expect("valid issue"),
            );

        assert!(report.is_valid());
        assert!(!report.is_complete());
        assert!(report.has_blockers());
        assert!(report.ensure_ready().is_err());
    }

    #[test]
    fn errors_prevent_readiness() {
        let mut report = ValidationReport::new();
        report
            .push_issue(
                ValidationIssue::new(
                    ValidationIssueKind::MissingCapability,
                    ValidationSeverity::Error,
                    "required capability is missing",
                )
                .expect("valid issue"),
            );

        assert!(!report.is_valid());
        assert!(!report.is_complete());
        assert!(report.ensure_ready().is_err());
    }

    #[test]
    fn completed_checks_are_deterministically_ordered() {
        let mut report = ValidationReport::new();
        report.record_check("z-last").expect("valid check");
        report.record_check("a-first").expect("valid check");
        report.record_check("z-last").expect("duplicate check is idempotent");

        assert_eq!(
            report.completed_checks().iter().map(String::as_str).collect::<Vec<_>>(),
            vec!["a-first", "z-last"]
        );
    }

    #[test]
    fn empty_check_names_are_rejected() {
        let mut report = ValidationReport::new();

        assert!(report.record_check("").is_err());
        assert!(report.record_check("  ").is_err());
        assert!(report.record_check(" padded ").is_err());
    }

    #[test]
    fn issue_subject_and_target_are_preserved() {
        let issue = ValidationIssue::new(
            ValidationIssueKind::MissingCapability,
            ValidationSeverity::Error,
            "missing capability",
        )
        .expect("valid issue")
        .with_target("quantum-target")
        .with_subject("quantum.measurement");

        assert_eq!(issue.target(), Some("quantum-target"));
        assert_eq!(issue.subject(), Some("quantum.measurement"));
    }
}
