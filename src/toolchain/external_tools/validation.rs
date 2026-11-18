
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani External Tool Validation
//!
//! Path: `src/toolchain/external_tools/validation.rs`
//! Rust edition: 2021.
//! Minimum Rust version: 1.97.
//! Safety: unsafe Rust is forbidden.
//!
//! ## Purpose
//!
//! Provides the security and compatibility validation boundary for external
//! tools selected during Zamani toolchain resolution.
//!
//! Validation covers:
//!
//! - Executable authorization.
//! - Canonical executable path verification.
//! - Requirement identity consistency.
//! - Numeric version constraints.
//! - Provider-reported capability verification.
//! - Required versus optional tool outcomes.
//! - Preservation of capability evidence.
//! - Deterministic validation reports.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - External-tool validation policy.
//! - Pre-discovery executable authorization.
//! - Post-discovery executable authorization rechecks.
//! - Requirement-to-resolution consistency checks.
//! - Version constraint validation.
//! - Capability verification orchestration.
//! - Structured validation outcomes.
//! - Validation reports.
//!
//! This module does NOT own:
//!
//! - External-tool requirement declarations.
//! - Executable discovery.
//! - PATH lookup.
//! - Process creation or execution.
//! - Process output capture.
//! - Version probing.
//! - Backend registration.
//! - Cryptographic executable identity.
//! - Hardware discovery or allocation.
//! - Compiler or runtime execution.
//!
//! ## Integration authorities
//!
//! `requirements.rs` owns:
//! - RequirementId.
//! - CapabilityId.
//! - DomainId.
//! - NumericVersion.
//! - VersionConstraint.
//! - ExternalToolRequirement.
//! - ExternalToolRequirements.
//! - RequirementKind.
//!
//! `resolution.rs` owns:
//! - ResolutionReport.
//! - ResolvedExternalTool.
//! - Candidate selection.
//! - Backend tool discovery orchestration.
//!
//! `backend::security` owns:
//! - ToolPolicy.
//! - SecurityError.
//! - Canonical executable allowlist validation.
//!
//! `backend::toolchain` owns:
//! - ToolVersion.
//! - ToolchainDescriptor.
//! - ToolchainManager.
//! - ToolchainSpec.
//! - Version probing.
//!
//! `backend::process` owns:
//! - ProcessRunner.
//! - ProcessRequest.
//! - ProcessOutput.
//! - Process cancellation.
//!
//! ## Security ordering
//!
//! The secure resolution sequence is:
//!
//! 1. Declare requirements.
//! 2. Construct candidates from trusted configuration.
//! 3. Authorize each candidate executable.
//! 4. Discover and probe only authorized executables.
//! 5. Recheck the selected executable.
//! 6. Validate version constraints.
//! 7. Verify required capabilities.
//! 8. Produce a structured validation report.
//! 9. Admit the operation only when required tools are validated.
//!
//! A resolver must not execute an executable before authorization.
//!
//! ## Scalability
//!
//! No arbitrary maximum number of requirements, capabilities, providers,
//! candidates, or computational domains is imposed.
//!
//! Collections grow according to available memory and operational budgets.
//! Resource budgets are supplied by the caller and are not language limits.
//!
//! ## Portability
//!
//! This module is domain-neutral. It does not contain closed lists of
//! classical, quantum, HDL, AI, hybrid, distributed, networking, or future
//! computing technologies.
//!
//! New providers and capabilities are represented through open identifiers
//! and registered verification implementations.
//!
//! ## Important security limitations
//!
//! Canonical path authorization is not cryptographic executable identity.
//! It does not prevent a file from being replaced after authorization.
//!
//! Hostile concurrent filesystem modification requires stronger platform
//! isolation, executable integrity verification, or process supervision.
//!
//! A capability verifier must provide evidence from a trusted source.
//! A nonempty evidence string is retained, but is not independently proof
//! of the truth of the capability claim.

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;
use std::path::{Path, PathBuf};

use crate::backend::security::{SecurityError, ToolPolicy};
use crate::backend::toolchain::{ToolVersion, ToolchainDescriptor};

use super::requirements::{
    CapabilityId,
    ExternalToolRequirement,
    ExternalToolRequirements,
    NumericVersion,
    RequirementError,
    RequirementId,
    RequirementKind,
};

use super::resolution::{
    ResolutionReport,
    ResolvedExternalTool,
};

/// Result type for external-tool validation operations.
pub type ValidationResult<T> = Result<T, ValidationError>;

/// Assessment returned by a registered capability verifier.
///
/// A verifier must not claim that a capability is satisfied merely because
/// an executable exists or reports a version.
///
/// The verifier is responsible for checking the capability using an
/// appropriate trusted source, provider contract, executable inspection,
/// or other authoritative evidence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CapabilityAssessment {
    /// The capability was verified.
    ///
    /// Evidence must be nonempty and should identify the verification basis,
    /// such as a provider attestation, verified tool metadata, or a
    /// successful capability-specific probe.
    Satisfied {
        /// Evidence supporting the capability assessment.
        evidence: String,
    },

    /// The capability was checked and found not to be supported.
    NotSatisfied {
        /// Explanation of the failed capability assessment.
        reason: String,
    },

    /// The capability could not be established.
    ///
    /// Unknown is not equivalent to supported.
    Unknown {
        /// Explanation of why verification was inconclusive.
        reason: String,
    },
}

/// Provider-neutral interface for verifying external-tool capabilities.
///
/// Implementations may be supplied by domain providers, backend adapters,
/// trusted tool registries, or other explicitly registered components.
///
/// This trait does not execute tools itself. If verification requires
/// execution, it must use the authorized descriptor and the existing
/// backend process/toolchain APIs under the applicable security policy.
pub trait ToolCapabilityVerifier: Send + Sync {
    /// Assess one required capability for a resolved external tool.
    ///
    /// The verifier receives:
    ///
    /// - The selected resolved tool.
    /// - Its original declarative requirement.
    /// - The capability being assessed.
    ///
    /// The verifier must return Unknown when it cannot establish the claim.
    fn assess(
        &self,
        resolved: &ResolvedExternalTool,
        requirement: &ExternalToolRequirement,
        capability: &CapabilityId,
    ) -> CapabilityAssessment;
}

/// Policy applied during external-tool validation.
///
/// The policy borrows the canonical backend executable authorization policy
/// and optionally borrows a registered capability verifier.
///
/// Borrowing avoids duplicating security policy state or introducing a
/// second executable allowlist.
pub struct ExternalToolValidationPolicy<'a> {
    tool_policy: &'a ToolPolicy,
    capability_verifier: Option<&'a dyn ToolCapabilityVerifier>,
}

impl<'a> ExternalToolValidationPolicy<'a> {
    /// Construct a validation policy using the supplied executable policy.
    ///
    /// No executable is authorized by default unless the supplied
    /// `ToolPolicy` explicitly allows it.
    #[must_use]
    pub const fn new(tool_policy: &'a ToolPolicy) -> Self {
        Self {
            tool_policy,
            capability_verifier: None,
        }
    }

    /// Attach a capability verifier.
    ///
    /// The verifier is shared by reference and is not owned by this policy.
    #[must_use]
    pub const fn capability_verifier(
        mut self,
        verifier: &'a dyn ToolCapabilityVerifier,
    ) -> Self {
        self.capability_verifier = Some(verifier);
        self
    }

    /// Return the executable authorization policy.
    #[must_use]
    pub const fn tool_policy(&self) -> &'a ToolPolicy {
        self.tool_policy
    }

    /// Return the configured capability verifier, if present.
    #[must_use]
    pub const fn verifier(
        &self,
    ) -> Option<&'a dyn ToolCapabilityVerifier> {
        self.capability_verifier
    }

    /// Authorize a candidate before the resolver starts its version probe.
    ///
    /// The candidate must already use its canonical absolute executable path.
    ///
    /// This deliberately rejects:
    ///
    /// - Bare executable names.
    /// - PATH-dependent executable names.
    /// - Relative executable paths.
    /// - Symlink paths.
    /// - Non-allowlisted executables.
    ///
    /// The returned path must be the exact path supplied to the resolver's
    /// `ToolchainSpec`.
    ///
    /// The resolver must call this method before
    /// `ToolchainManager::discover()`.
    pub fn authorize_candidate_for_discovery(
        &self,
        executable: impl AsRef<Path>,
    ) -> Result<PathBuf, CandidateAuthorizationError> {
        let requested = executable.as_ref();

        let authorized = self
            .tool_policy
            .validate_executable(requested)
            .map_err(CandidateAuthorizationError::Security)?;

        if authorized.as_path() != requested {
            return Err(
                CandidateAuthorizationError::NonCanonicalPath {
                    requested: requested.to_path_buf(),
                    canonical: authorized,
                },
            );
        }

        Ok(authorized)
    }
}

/// Error encountered while authorizing an executable before discovery.
#[derive(Debug)]
pub enum CandidateAuthorizationError {
    /// The backend executable policy rejected the candidate.
    Security(SecurityError),

    /// The candidate did not use its canonical absolute path.
    NonCanonicalPath {
        /// Path supplied by the candidate.
        requested: PathBuf,

        /// Canonical path returned by the security policy.
        canonical: PathBuf,
    },
}

impl fmt::Display for CandidateAuthorizationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Security(source) => {
                write!(
                    formatter,
                    "candidate executable authorization failed: {source}"
                )
            }

            Self::NonCanonicalPath {
                requested,
                canonical,
            } => {
                write!(
                    formatter,
                    "candidate path `{}` is not canonical; use `{}`",
                    requested.display(),
                    canonical.display()
                )
            }
        }
    }
}

impl Error for CandidateAuthorizationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Security(source) => Some(source),
            Self::NonCanonicalPath { .. } => None,
        }
    }
}

/// Classification of an external-tool validation result.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolValidationStatus {
    /// The selected tool passed every required validation.
    Validated,

    /// The requirement has no selected tool or has inconclusive validation.
    ///
    /// This status is not sufficient to admit a required operation.
    NotResolved,

    /// A definitive security, version, or capability check failed.
    Rejected,
}

/// Structured validation issue.
///
/// Issues are retained in deterministic validation order. They are not
/// represented only as strings so callers can make machine-readable
/// admission and reporting decisions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationIssue {
    /// No resolution entry exists for the declared requirement.
    MissingResolution {
        /// Logical requirement identifier.
        requirement_id: RequirementId,
    },

    /// The requirement has a resolution entry but no selected tool.
    NoSelectedTool {
        /// Logical requirement identifier.
        requirement_id: RequirementId,
    },

    /// The selected executable is not authorized.
    ExecutableNotAuthorized {
        /// Logical requirement identifier.
        requirement_id: RequirementId,

        /// Executable path that was rejected.
        executable: PathBuf,

        /// Security policy failure.
        reason: String,
    },

    /// The selected executable path is not the exact canonical path
    /// returned by the backend security policy.
    CanonicalPathMismatch {
        /// Logical requirement identifier.
        requirement_id: RequirementId,

        /// Path held by the resolved descriptor.
        supplied: PathBuf,

        /// Canonical path returned by the security policy.
        canonical: PathBuf,
    },

    /// The selected tool does not satisfy its declared version constraint.
    VersionConstraintNotSatisfied {
        /// Logical requirement identifier.
        requirement_id: RequirementId,

        /// Actual discovered version.
        actual: NumericVersion,

        /// Human-readable constraint description.
        expected: String,
    },

    /// A required capability was definitively not satisfied.
    CapabilityNotSatisfied {
        /// Logical requirement identifier.
        requirement_id: RequirementId,

        /// Capability that failed verification.
        capability: CapabilityId,

        /// Explanation returned by the verifier.
        reason: String,
    },

    /// A required capability could not be verified.
    ///
    /// Unknown capability status must never be interpreted as support.
    CapabilityUnknown {
        /// Logical requirement identifier.
        requirement_id: RequirementId,

        /// Capability that could not be verified.
        capability: CapabilityId,

        /// Explanation of the inconclusive result.
        reason: String,
    },
}

impl ValidationIssue {
    /// Return the logical requirement identifier associated with this issue.
    #[must_use]
    pub fn requirement_id(&self) -> &RequirementId {
        match self {
            Self::MissingResolution { requirement_id }
            | Self::NoSelectedTool { requirement_id }
            | Self::ExecutableNotAuthorized {
                requirement_id,
                ..
            }
            | Self::CanonicalPathMismatch {
                requirement_id,
                ..
            }
            | Self::VersionConstraintNotSatisfied {
                requirement_id,
                ..
            }
            | Self::CapabilityNotSatisfied {
                requirement_id,
                ..
            }
            | Self::CapabilityUnknown {
                requirement_id,
                ..
            } => requirement_id,
        }
    }

    /// Return a stable machine-readable issue code.
    #[must_use]
    pub const fn code(&self) -> &'static str {
        match self {
            Self::MissingResolution { .. } => "missing_resolution",
            Self::NoSelectedTool { .. } => "no_selected_tool",
            Self::ExecutableNotAuthorized { .. } => {
                "executable_not_authorized"
            }
            Self::CanonicalPathMismatch { .. } => {
                "canonical_path_mismatch"
            }
            Self::VersionConstraintNotSatisfied { .. } => {
                "version_constraint_not_satisfied"
            }
            Self::CapabilityNotSatisfied { .. } => {
                "capability_not_satisfied"
            }
            Self::CapabilityUnknown { .. } => "capability_unknown",
        }
    }

    /// Return whether this issue represents a definitive rejection.
    #[must_use]
    pub const fn is_rejection(&self) -> bool {
        matches!(
            self,
            Self::ExecutableNotAuthorized { .. }
                | Self::CanonicalPathMismatch { .. }
                | Self::VersionConstraintNotSatisfied { .. }
                | Self::CapabilityNotSatisfied { .. }
        )
    }
}

/// Validation result for one declared external-tool requirement.
#[derive(Debug, Clone)]
pub struct ToolValidationResult {
    requirement_id: RequirementId,
    requirement_kind: RequirementKind,
    status: ToolValidationStatus,
    provider_id: Option<String>,
    candidate_id: Option<String>,
    executable: Option<PathBuf>,
    version: Option<NumericVersion>,
    verified_capabilities: BTreeMap<CapabilityId, String>,
    issues: Vec<ValidationIssue>,
}

impl ToolValidationResult {
    fn unresolved(
        requirement: &ExternalToolRequirement,
        issue: ValidationIssue,
    ) -> Self {
        Self {
            requirement_id: requirement.id().clone(),
            requirement_kind: requirement.requirement_kind(),
            status: ToolValidationStatus::NotResolved,
            provider_id: None,
            candidate_id: None,
            executable: None,
            version: None,
            verified_capabilities: BTreeMap::new(),
            issues: vec![issue],
        }
    }

    fn from_resolved(
        resolved: &ResolvedExternalTool,
        requirement: &ExternalToolRequirement,
    ) -> Self {
        Self {
            requirement_id: requirement.id().clone(),
            requirement_kind: requirement.requirement_kind(),
            status: ToolValidationStatus::Validated,
            provider_id: Some(resolved.provider_id().to_owned()),
            candidate_id: Some(resolved.candidate_id().to_owned()),
            executable: Some(resolved.executable().to_path_buf()),
            version: Some(numeric_version(resolved.version())),
            verified_capabilities: BTreeMap::new(),
            issues: Vec::new(),
        }
    }

    fn reject(&mut self, issue: ValidationIssue) {
        self.status = ToolValidationStatus::Rejected;
        self.issues.push(issue);
    }

    fn mark_unresolved(&mut self, issue: ValidationIssue) {
        if self.status != ToolValidationStatus::Rejected {
            self.status = ToolValidationStatus::NotResolved;
        }

        self.issues.push(issue);
    }

    /// Return the logical requirement identifier.
    #[must_use]
    pub fn requirement_id(&self) -> &RequirementId {
        &self.requirement_id
    }

    /// Return whether this requirement is mandatory or optional.
    #[must_use]
    pub const fn requirement_kind(&self) -> RequirementKind {
        self.requirement_kind
    }

    /// Return the validation status.
    #[must_use]
    pub const fn status(&self) -> ToolValidationStatus {
        self.status
    }

    /// Return whether the selected tool passed validation.
    #[must_use]
    pub const fn is_validated(&self) -> bool {
        matches!(self.status, ToolValidationStatus::Validated)
    }

    /// Return the selected provider identifier, if resolved.
    #[must_use]
    pub fn provider_id(&self) -> Option<&str> {
        self.provider_id.as_deref()
    }

    /// Return the selected candidate identifier, if resolved.
    #[must_use]
    pub fn candidate_id(&self) -> Option<&str> {
        self.candidate_id.as_deref()
    }

    /// Return the selected canonical executable path, if resolved.
    #[must_use]
    pub fn executable(&self) -> Option<&Path> {
        self.executable.as_deref()
    }

    /// Return the selected numeric version, if resolved.
    #[must_use]
    pub const fn version(&self) -> Option<NumericVersion> {
        self.version
    }

    /// Return evidence recorded for successfully verified capabilities.
    ///
    /// Evidence is indexed by the original capability identifier.
    #[must_use]
    pub fn verified_capabilities(
        &self,
    ) -> &BTreeMap<CapabilityId, String> {
        &self.verified_capabilities
    }

    /// Return all validation issues for this requirement.
    #[must_use]
    pub fn issues(&self) -> &[ValidationIssue] {
        &self.issues
    }

    /// Return the number of validation issues.
    #[must_use]
    pub fn issue_count(&self) -> usize {
        self.issues.len()
    }

    /// Return whether this requirement has any validation issues.
    #[must_use]
    pub fn has_issues(&self) -> bool {
        !self.issues.is_empty()
    }
}

/// Deterministic validation report for a complete requirement collection.
#[derive(Debug, Clone, Default)]
pub struct ValidationReport {
    results: BTreeMap<RequirementId, ToolValidationResult>,
    cancelled: bool,
}

impl ValidationReport {
    /// Construct an empty validation report.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Look up a validation result by logical requirement identifier.
    #[must_use]
    pub fn get(
        &self,
        id: &RequirementId,
    ) -> Option<&ToolValidationResult> {
        self.results.get(id)
    }

    /// Iterate over results in deterministic requirement-ID order.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = &ToolValidationResult> {
        self.results.values()
    }

    /// Return the number of validation results.
    #[must_use]
    pub fn len(&self) -> usize {
        self.results.len()
    }

    /// Return whether the report contains no results.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.results.is_empty()
    }

    /// Return the number of successfully validated tools.
    #[must_use]
    pub fn validated_count(&self) -> usize {
        self.results
            .values()
            .filter(|result| result.is_validated())
            .count()
    }

    /// Return the number of mandatory requirements that were not validated.
    #[must_use]
    pub fn required_failure_count(&self) -> usize {
        self.results
            .values()
            .filter(|result| {
                result.requirement_kind == RequirementKind::Required
                    && !result.is_validated()
            })
            .count()
    }

    /// Return the number of optional requirements that were not validated.
    #[must_use]
    pub fn optional_failure_count(&self) -> usize {
        self.results
            .values()
            .filter(|result| {
                result.requirement_kind == RequirementKind::Optional
                    && !result.is_validated()
            })
            .count()
    }

    /// Return whether every mandatory requirement was validated.
    #[must_use]
    pub fn required_requirements_validated(&self) -> bool {
        self.required_failure_count() == 0
    }

    /// Return whether every requirement, including optional requirements,
    /// was validated.
    #[must_use]
    pub fn all_requirements_validated(&self) -> bool {
        self.results
            .values()
            .all(ToolValidationResult::is_validated)
    }

    /// Return whether the associated resolution was cancelled.
    #[must_use]
    pub const fn was_cancelled(&self) -> bool {
        self.cancelled
    }

    /// Return whether the operation may proceed with respect to required
    /// external-tool validation.
    ///
    /// Optional failures do not block readiness.
    ///
    /// Cancellation always prevents admission.
    #[must_use]
    pub fn is_ready_for_required_tools(&self) -> bool {
        !self.cancelled && self.required_requirements_validated()
    }

    /// Return the total number of recorded validation issues.
    #[must_use]
    pub fn issue_count(&self) -> usize {
        self.results
            .values()
            .map(ToolValidationResult::issue_count)
            .sum()
    }
}

/// Errors encountered while validating an external-tool resolution.
#[derive(Debug)]
pub enum ValidationError {
    /// The declarative requirement collection is invalid.
    InvalidRequirements(RequirementError),

    /// A resolved tool references a requirement that was not declared.
    UnexpectedResolvedRequirement {
        /// Unexpected logical requirement identifier.
        requirement_id: RequirementId,
    },

    /// Multiple resolved tools were supplied for one logical requirement.
    DuplicateResolvedRequirement {
        /// Duplicate logical requirement identifier.
        requirement_id: RequirementId,
    },

    /// The resolution report contains an internally inconsistent result.
    InconsistentResolution {
        /// Logical requirement identifier.
        requirement_id: RequirementId,

        /// Explanation of the inconsistency.
        reason: String,
    },
}

impl fmt::Display for ValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidRequirements(source) => {
                write!(
                    formatter,
                    "invalid external-tool requirements: {source}"
                )
            }

            Self::UnexpectedResolvedRequirement {
                requirement_id,
            } => {
                write!(
                    formatter,
                    "resolution contains undeclared requirement `{requirement_id}`"
                )
            }

            Self::DuplicateResolvedRequirement {
                requirement_id,
            } => {
                write!(
                    formatter,
                    "resolution contains multiple selected tools for requirement `{requirement_id}`"
                )
            }

            Self::InconsistentResolution {
                requirement_id,
                reason,
            } => {
                write!(
                    formatter,
                    "inconsistent resolution for `{requirement_id}`: {reason}"
                )
            }
        }
    }
}

impl Error for ValidationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidRequirements(source) => Some(source),
            _ => None,
        }
    }
}

/// Stateless external-tool validator.
///
/// The validator consumes already-resolved tools. It does not discover
/// executables, probe versions, or execute processes.
#[derive(Debug, Default, Clone, Copy)]
pub struct ExternalToolValidator;

impl ExternalToolValidator {
    /// Construct an external-tool validator.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Validate a resolution report against the original requirements.
    ///
    /// The validator:
    ///
    /// 1. Validates the requirement collection.
    /// 2. Rejects resolution entries for undeclared requirements.
    /// 3. Rejects duplicate selected tools for one requirement.
    /// 4. Rechecks executable authorization.
    /// 5. Checks that the resolved executable path is canonical.
    /// 6. Validates numeric version constraints.
    /// 7. Verifies every declared capability.
    /// 8. Records all outcomes without silently treating unknown as success.
    ///
    /// This method does not execute external tools.
    pub fn validate(
        &self,
        requirements: &ExternalToolRequirements,
        resolution: &ResolutionReport,
        policy: &ExternalToolValidationPolicy<'_>,
    ) -> ValidationResult<ValidationReport> {
        requirements
            .validate()
            .map_err(ValidationError::InvalidRequirements)?;

        let mut resolved_by_requirement =
            BTreeMap::<RequirementId, &ResolvedExternalTool>::new();

        for resolved in resolution.resolved_tools() {
            let requirement_id = resolved.requirement_id();

            if !requirements.contains(requirement_id) {
                return Err(
                    ValidationError::UnexpectedResolvedRequirement {
                        requirement_id: requirement_id.clone(),
                    },
                );
            }

            if resolved_by_requirement
                .insert(requirement_id.clone(), resolved)
                .is_some()
            {
                return Err(
                    ValidationError::DuplicateResolvedRequirement {
                        requirement_id: requirement_id.clone(),
                    },
                );
            }
        }

        let mut report = ValidationReport::new();
        report.cancelled = resolution.was_cancelled();

        for requirement in requirements.iter() {
            let requirement_id = requirement.id();

            let resolution_entry = resolution.get(requirement_id);

            if resolution_entry.is_none() {
                report.results.insert(
                    requirement_id.clone(),
                    ToolValidationResult::unresolved(
                        requirement,
                        ValidationIssue::MissingResolution {
                            requirement_id: requirement_id.clone(),
                        },
                    ),
                );

                continue;
            }

            let Some(resolved) =
                resolved_by_requirement.get(requirement_id).copied()
            else {
                report.results.insert(
                    requirement_id.clone(),
                    ToolValidationResult::unresolved(
                        requirement,
                        ValidationIssue::NoSelectedTool {
                            requirement_id: requirement_id.clone(),
                        },
                    ),
                );

                continue;
            };

            let result = self.validate_selected_tool(
                requirement,
                resolved,
                policy,
            );

            report.results.insert(requirement_id.clone(), result);
        }

        Ok(report)
    }

    fn validate_selected_tool(
        &self,
        requirement: &ExternalToolRequirement,
        resolved: &ResolvedExternalTool,
        policy: &ExternalToolValidationPolicy<'_>,
    ) -> ToolValidationResult {
        let mut result =
            ToolValidationResult::from_resolved(resolved, requirement);

        let requirement_id = requirement.id().clone();
        let descriptor: &ToolchainDescriptor = resolved.descriptor();

        // Recheck authorization after discovery. This does not eliminate
        // filesystem TOCTOU races, but detects many path changes and policy
        // inconsistencies before the tool is admitted.
        let authorized = match policy
            .tool_policy()
            .validate_executable(descriptor.executable())
        {
            Ok(path) => path,

            Err(source) => {
                result.reject(
                    ValidationIssue::ExecutableNotAuthorized {
                        requirement_id,
                        executable: descriptor.executable().to_path_buf(),
                        reason: source.to_string(),
                    },
                );

                return result;
            }
        };

        if authorized.as_path() != descriptor.executable() {
            result.reject(
                ValidationIssue::CanonicalPathMismatch {
                    requirement_id,
                    supplied: descriptor.executable().to_path_buf(),
                    canonical: authorized,
                },
            );

            return result;
        }

        // The version constraint is defined by requirements.rs.
        // ToolVersion is converted without guessing vendor-specific suffixes.
        let actual_version = numeric_version(descriptor.version());

        match requirement
            .version_constraint()
            .matches(actual_version)
        {
            Ok(true) => {}

            Ok(false) => {
                result.reject(
                    ValidationIssue::VersionConstraintNotSatisfied {
                        requirement_id,
                        actual: actual_version,
                        expected: describe_version_constraint(
                            requirement.version_constraint(),
                        ),
                    },
                );

                return result;
            }

            Err(error) => {
                result.reject(
                    ValidationIssue::VersionConstraintNotSatisfied {
                        requirement_id,
                        actual: actual_version,
                        expected: format!(
                            "invalid configured version constraint: {error}"
                        ),
                    },
                );

                return result;
            }
        }

        // Every declared capability must be independently assessed.
        // No verifier means capability support remains unknown.
        for capability in requirement.capabilities() {
            let assessment = match policy.verifier() {
                Some(verifier) => {
                    verifier.assess(resolved, requirement, capability)
                }

                None => CapabilityAssessment::Unknown {
                    reason: "no capability verifier is registered".to_owned(),
                },
            };

            match assessment {
                CapabilityAssessment::Satisfied { evidence } => {
                    let evidence = evidence.trim();

                    if evidence.is_empty() {
                        result.mark_unresolved(
                            ValidationIssue::CapabilityUnknown {
                                requirement_id: requirement_id.clone(),
                                capability: capability.clone(),
                                reason: "the verifier returned empty evidence"
                                    .to_owned(),
                            },
                        );
                    } else {
                        result.verified_capabilities.insert(
                            capability.clone(),
                            evidence.to_owned(),
                        );
                    }
                }

                CapabilityAssessment::NotSatisfied { reason } => {
                    result.reject(
                        ValidationIssue::CapabilityNotSatisfied {
                            requirement_id: requirement_id.clone(),
                            capability: capability.clone(),
                            reason,
                        },
                    );
                }

                CapabilityAssessment::Unknown { reason } => {
                    result.mark_unresolved(
                        ValidationIssue::CapabilityUnknown {
                            requirement_id: requirement_id.clone(),
                            capability: capability.clone(),
                            reason,
                        },
                    );
                }
            }
        }

        result
    }
}

/// Convert the backend's numeric tool version into the provider-neutral
/// declarative numeric version.
#[must_use]
pub const fn numeric_version(version: ToolVersion) -> NumericVersion {
    NumericVersion::new(version.major, version.minor, version.patch)
}

/// Describe a numeric version constraint for diagnostics.
#[must_use]
pub fn describe_version_constraint(
    constraint: &super::requirements::VersionConstraint,
) -> String {
    if let Some(exact) = constraint.exact() {
        return format!("exactly {exact}");
    }

    match (constraint.minimum(), constraint.maximum()) {
        (Some(minimum), Some(maximum)) => {
            format!("between {minimum} and {maximum}, inclusive")
        }

        (Some(minimum), None) => {
            format!("at least {minimum}")
        }

        (None, Some(maximum)) => {
            format!("at most {maximum}")
        }

        (None, None) => {
            "any identifiable numeric version".to_owned()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn backend_version_conversion_preserves_all_components() {
        let backend = ToolVersion::new(17, 2, 9);

        assert_eq!(
            numeric_version(backend),
            NumericVersion::new(17, 2, 9)
        );
    }

    #[test]
    fn version_constraint_description_handles_unbounded_constraint() {
        let constraint = super::super::requirements::VersionConstraint::any();

        assert_eq!(
            describe_version_constraint(&constraint),
            "any identifiable numeric version"
        );
    }

    #[test]
    fn version_constraint_description_handles_exact_version() {
        let exact = NumericVersion::new(1, 2, 3);

        let constraint = super::super::requirements::VersionConstraint::any()
            .exactly(exact);

        assert_eq!(
            describe_version_constraint(&constraint),
            "exactly 1.2.3"
        );
    }

    #[test]
    fn version_constraint_description_handles_range() {
        let constraint = super::super::requirements::VersionConstraint::any()
            .at_least(NumericVersion::new(1, 0, 0))
            .at_most(NumericVersion::new(4, 0, 0));

        assert_eq!(
            describe_version_constraint(&constraint),
            "between 1.0.0 and 4.0.0, inclusive"
        );
    }

    #[test]
    fn unknown_capability_is_not_a_successful_assessment() {
        let assessment = CapabilityAssessment::Unknown {
            reason: "no verifier".to_owned(),
        };

        assert!(matches!(
            assessment,
            CapabilityAssessment::Unknown { .. }
        ));
    }

    #[test]
    fn empty_capability_evidence_is_not_valid_evidence() {
        let evidence = "   ";

        assert!(evidence.trim().is_empty());
    }

    #[test]
    fn rejection_issue_codes_are_stable() {
        let id = RequirementId::new("llvm.clang")
            .expect("valid requirement identifier");

        let issue = ValidationIssue::MissingResolution {
            requirement_id: id,
        };

        assert_eq!(issue.code(), "missing_resolution");
        assert!(!issue.is_rejection());
    }

    #[test]
    fn validation_report_distinguishes_required_and_optional_failures() {
        let required = ExternalToolRequirement::new("compiler.required")
            .expect("valid requirement");

        let optional = ExternalToolRequirement::new("compiler.optional")
            .expect("valid requirement")
            .kind(RequirementKind::Optional);

        let mut report = ValidationReport::new();

        report.results.insert(
            required.id().clone(),
            ToolValidationResult::unresolved(
                &required,
                ValidationIssue::NoSelectedTool {
                    requirement_id: required.id().clone(),
                },
            ),
        );

        report.results.insert(
            optional.id().clone(),
            ToolValidationResult::unresolved(
                &optional,
                ValidationIssue::NoSelectedTool {
                    requirement_id: optional.id().clone(),
                },
            ),
        );

        assert_eq!(report.required_failure_count(), 1);
        assert_eq!(report.optional_failure_count(), 1);
        assert!(!report.is_ready_for_required_tools());
    }

    #[test]
    fn optional_failure_does_not_count_as_required_failure() {
        let optional = ExternalToolRequirement::new("compiler.optional")
            .expect("valid requirement")
            .kind(RequirementKind::Optional);

        let mut report = ValidationReport::new();

        report.results.insert(
            optional.id().clone(),
            ToolValidationResult::unresolved(
                &optional,
                ValidationIssue::NoSelectedTool {
                    requirement_id: optional.id().clone(),
                },
            ),
        );

        assert_eq!(report.required_failure_count(), 0);
        assert_eq!(report.optional_failure_count(), 1);
        assert!(report.is_ready_for_required_tools());
    }

    #[test]
    fn cancellation_prevents_admission() {
        let mut report = ValidationReport::new();
        report.cancelled = true;

        assert!(report.was_cancelled());
        assert!(!report.is_ready_for_required_tools());
    }
}
