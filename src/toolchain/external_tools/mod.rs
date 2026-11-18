
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Toolchain — External Tools
//!
//! Path: src/toolchain/external_tools/mod.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! This module is the public composition and orchestration boundary for
//! external tools used by the Zamani toolchain.
//!
//! It coordinates:
//!
//! - Declarative external-tool requirements.
//! - Registered executable candidates.
//! - Pre-discovery executable authorization.
//! - Existing backend executable discovery.
//! - Backend-owned version probing.
//! - Post-discovery security and compatibility validation.
//! - Provider capability verification.
//! - Cancellation.
//! - Structured readiness reporting.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - External-tool subsystem composition.
//! - The public external-tool orchestration service.
//! - The complete resolve/validate/report workflow.
//! - Pre-discovery authorization ordering.
//! - Aggregate readiness outcomes.
//! - Cross-module error propagation.
//!
//! It does NOT own:
//!
//! - Source-language syntax or grammar.
//! - Semantic analysis.
//! - Compiler or IR implementation.
//! - Hardware discovery or physical resource allocation.
//! - Executable discovery implementation.
//! - Executable version parsing.
//! - Process creation or execution.
//! - Process output capture.
//! - Cryptographic executable verification.
//! - Backend registration.
//! - Runtime execution or deployment.
//!
//! ## Authoritative integration boundaries
//!
//! requirements.rs:
//!     Declarative requirements, domains, capabilities and constraints.
//!
//! resolution.rs:
//!     Candidate collections and resolution evidence.
//!
//! validation.rs:
//!     Executable authorization, capability verification and validation.
//!
//! reporting.rs:
//!     Structured readiness reports and report serialization.
//!
//! backend/toolchain.rs:
//!     ToolchainSpec, ToolVersion, ToolchainManager and ToolchainDescriptor.
//!
//! backend/process.rs:
//!     ProcessRunner, ProcessRequest, ProcessOutput and cancellation.
//!
//! backend/security.rs:
//!     ToolPolicy and executable authorization.
//!
//! No competing implementations of these authorities are introduced.
//!
//! ## Security ordering
//!
//! The orchestration sequence is:
//!
//! 1. Validate declarative requirements.
//! 2. Validate candidate references and configuration.
//! 3. Authorize every applicable executable candidate.
//! 4. Resolve candidates using the existing backend toolchain manager.
//! 5. Revalidate selected executable paths.
//! 6. Validate numeric version constraints.
//! 7. Verify declared capabilities.
//! 8. Generate a consolidated report.
//! 9. Permit dependent operations only when required tools are validated.
//!
//! An unauthorized executable MUST NOT be probed.
//!
//! Discovery MUST NOT be interpreted as authorization.
//!
//! Unknown capability evidence MUST NOT be interpreted as support.
//!
//! ## Scalability
//!
//! No arbitrary maximum is imposed on:
//!
//! - External-tool requirements.
//! - Providers.
//! - Executable candidates.
//! - Capabilities.
//! - Computational domains.
//! - Target realizations.
//! - Projects.
//! - Computational resources.
//!
//! Collections grow according to available memory and configured
//! operational budgets.
//!
//! This module does not promise literal infinite resources.
//!
//! ## Portability
//!
//! The subsystem is domain-neutral.
//!
//! It supports open identifiers for classical, quantum, HDL, AI, hybrid,
//! distributed, networking, cryptography and future computing domains.
//!
//! No vendor-specific executable names or permanent provider inventories
//! are defined here.
//!
//! ## Important limitation
//!
//! The current backend ToolchainManager discovery API does not accept
//! a cancellation token. Cancellation can therefore stop work between
//! discovery attempts but cannot interrupt an active version probe.
//!
//! Filesystem authorization also cannot eliminate every TOCTOU race.
//! Stronger executable identity and process isolation require additional
//! platform security infrastructure.


// ============================================================================
// EXTERNAL-TOOL SUBSYSTEM MODULES
// ============================================================================

/// Declarative external-tool requirements and constraints.
pub mod requirements;

/// Executable candidate management and resolution orchestration.
pub mod resolution;

/// Security, version and capability validation.
pub mod validation;

/// Consolidated external-tool reports and serialization.
pub mod reporting;


// ============================================================================
// PUBLIC REQUIREMENT API
// ============================================================================

pub use requirements::{
    CapabilityId,
    DomainId,
    ExternalToolRequirement,
    ExternalToolRequirements,
    NumericVersion,
    RequirementError,
    RequirementId,
    RequirementKind,
    VersionConstraint,
};


// ============================================================================
// PUBLIC RESOLUTION API
// ============================================================================

pub use resolution::{
    CandidateFailure,
    ExternalToolResolver,
    RequirementResolution,
    ResolutionError,
    ResolutionReport,
    ResolutionStatus,
    ResolvedExternalTool,
    ToolchainCandidate,
    ToolchainCandidates,
};


// ============================================================================
// PUBLIC VALIDATION API
// ============================================================================

pub use validation::{
    CandidateAuthorizationError,
    CapabilityAssessment,
    ExternalToolValidationPolicy,
    ExternalToolValidator,
    ToolCapabilityVerifier,
    ToolValidationResult,
    ToolValidationStatus,
    ValidationError,
    ValidationIssue,
    ValidationReport,
    ValidationResult,
};


// ============================================================================
// PUBLIC REPORTING API
// ============================================================================

pub use reporting::{
    ExternalToolDiagnostic,
    ExternalToolReport,
    ExternalToolReportEntry,
    ExternalToolReportSummary,
    ExternalToolStatus,
    ReportSeverity,
    ReportingError,
    EXTERNAL_TOOL_REPORT_SCHEMA_VERSION,
};


// ============================================================================
// CANONICAL BACKEND INTEGRATION
// ============================================================================

use crate::backend::process::CancellationToken;
use crate::backend::security::ToolPolicy;

use std::error::Error;
use std::fmt;


// ============================================================================
// SERVICE RESULT
// ============================================================================

/// Result type for external-tool orchestration.
pub type ExternalToolchainResult<T> =
    Result<T, ExternalToolchainError>;


// ============================================================================
// ORCHESTRATION OUTCOME
// ============================================================================

/// Complete result of one external-tool orchestration operation.
///
/// This object preserves the original resolution evidence, validation
/// evidence and consolidated report.
///
/// It does not discard optional failures or replace upstream evidence
/// with a simplified success flag.
pub struct ExternalToolchainOutcome {
    resolution: ResolutionReport,
    validation: ValidationReport,
    report: ExternalToolReport,
}

impl ExternalToolchainOutcome {
    /// Construct an orchestration outcome.
    fn new(
        resolution: ResolutionReport,
        validation: ValidationReport,
        report: ExternalToolReport,
    ) -> Self {
        Self {
            resolution,
            validation,
            report,
        }
    }

    /// Return the original resolution evidence.
    #[must_use]
    pub const fn resolution(&self) -> &ResolutionReport {
        &self.resolution
    }

    /// Return the original validation evidence.
    #[must_use]
    pub const fn validation(&self) -> &ValidationReport {
        &self.validation
    }

    /// Return the consolidated external-tool report.
    #[must_use]
    pub const fn report(&self) -> &ExternalToolReport {
        &self.report
    }

    /// Return whether all required external tools were validated.
    ///
    /// This is a prerequisite result, not proof that the entire build
    /// or target realization is feasible.
    #[must_use]
    pub const fn required_tools_validated(&self) -> bool {
        self.report.required_tools_validated()
    }

    /// Return whether dependent operations may proceed with respect
    /// to external-tool prerequisites.
    ///
    /// Optional tool failures do not independently prevent readiness.
    #[must_use]
    pub const fn is_ready(&self) -> bool {
        self.report.is_ready_for_required_tools()
    }

    /// Return whether the operation was cancelled.
    #[must_use]
    pub const fn was_cancelled(&self) -> bool {
        self.report.was_cancelled()
    }

    /// Return the number of declared external-tool requirements.
    #[must_use]
    pub fn requirement_count(&self) -> usize {
        self.report.len()
    }

    /// Return the number of successfully resolved executables.
    #[must_use]
    pub fn resolved_count(&self) -> usize {
        self.resolution.resolved_count()
    }

    /// Return the number of successfully validated executables.
    #[must_use]
    pub fn validated_count(&self) -> usize {
        self.validation.validated_count()
    }

    /// Serialize the complete outcome report to JSON.
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        self.report.to_json()
    }

    /// Serialize the complete outcome report as pretty JSON.
    pub fn to_json_pretty(
        &self,
    ) -> Result<String, serde_json::Error> {
        self.report.to_json_pretty()
    }

    /// Render a human-readable report.
    ///
    /// The returned text is presentation only.
    /// Call is_ready() for the authoritative readiness decision.
    pub fn render_text(&self) -> String {
        self.report.render_text()
    }
}


// ============================================================================
// EXTERNAL-TOOL ORCHESTRATION SERVICE
// ============================================================================

/// Unified external-tool orchestration service.
///
/// This service composes the existing requirement, resolution,
/// validation and reporting implementations.
///
/// It does not maintain global mutable registries, perform implicit
/// hardware discovery or create another process execution subsystem.
///
/// Instances are inexpensive and can be created independently for
/// different workspaces, build requests or execution environments.
#[derive(Debug, Default, Clone, Copy)]
pub struct ExternalToolchainService {
    resolver: ExternalToolResolver,
    validator: ExternalToolValidator,
}

impl ExternalToolchainService {
    /// Construct the external-tool orchestration service.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            resolver: ExternalToolResolver::new(),
            validator: ExternalToolValidator::new(),
        }
    }

    /// Resolve, validate and report external-tool requirements.
    ///
    /// The supplied security policy is mandatory.
    ///
    /// An empty executable allowlist authorizes no executable.
    ///
    /// A new cancellation token is used for this operation.
    ///
    /// For cancellable operations, use
    /// resolve_and_validate_with_cancellation().
    pub fn resolve_and_validate(
        &self,
        requirements: &ExternalToolRequirements,
        candidates: &ToolchainCandidates,
        policy: &ExternalToolValidationPolicy<'_>,
    ) -> ExternalToolchainResult<ExternalToolchainOutcome> {
        let cancellation = CancellationToken::new();

        self.resolve_and_validate_with_cancellation(
            requirements,
            candidates,
            policy,
            &cancellation,
        )
    }

    /// Resolve, validate and report external-tool requirements with
    /// cooperative cancellation.
    ///
    /// SECURITY ORDER:
    ///
    /// Every candidate associated with a declared requirement is
    /// authorized before the resolver is permitted to probe any
    /// executable.
    ///
    /// If candidate authorization fails, this operation returns an
    /// error and no candidate discovery is started.
    ///
    /// Resolution, validation and reporting retain their existing
    /// authoritative implementations.
    ///
    /// Cancellation behavior:
    ///
    /// - Cancellation before authorization completes stops the operation.
    /// - Cancellation between candidate probes stops further resolution.
    /// - An active backend version probe cannot currently be interrupted.
    /// - Partial resolution evidence is preserved where available.
    pub fn resolve_and_validate_with_cancellation(
        &self,
        requirements: &ExternalToolRequirements,
        candidates: &ToolchainCandidates,
        policy: &ExternalToolValidationPolicy<'_>,
        cancellation: &CancellationToken,
    ) -> ExternalToolchainResult<ExternalToolchainOutcome> {

        // ------------------------------------------------------------
        // STAGE 1: VALIDATE REQUIREMENTS
        // ------------------------------------------------------------

        requirements
            .validate()
            .map_err(ExternalToolchainError::InvalidRequirements)?;

        if cancellation.is_cancelled() {
            return Err(
                ExternalToolchainError::CancelledBeforeDiscovery,
            );
        }

        // ------------------------------------------------------------
        // STAGE 2: PRE-DISCOVERY SECURITY AUTHORIZATION
        // ------------------------------------------------------------
        //
        // This stage deliberately precedes ExternalToolResolver.
        //
        // The existing resolver invokes ToolchainManager::discover().
        // Therefore authorization must be established before invoking
        // that resolver.
        //
        // Candidate identifiers are never interpreted as executable
        // names or shell commands.

        for requirement in requirements.iter() {
            for candidate in
                candidates.for_requirement(requirement.id())
            {
                if cancellation.is_cancelled() {
                    return Err(
                        ExternalToolchainError::CancelledBeforeDiscovery,
                    );
                }

                policy
                    .authorize_candidate_for_discovery(
                        candidate.requested_executable(),
                    )
                    .map_err(|source| {
                        ExternalToolchainError::CandidateAuthorization {
                            requirement_id:
                                requirement.id().to_string(),
                            provider_id:
                                candidate.provider_id().to_owned(),
                            candidate_id:
                                candidate.candidate_id().to_owned(),
                            source,
                        }
                    })?;
            }
        }

        // ------------------------------------------------------------
        // STAGE 3: BACKEND-OWNED DISCOVERY AND VERSION VALIDATION
        // ------------------------------------------------------------
        //
        // No duplicate executable resolver or version parser is
        // introduced here.
        //
        // backend::toolchain remains authoritative.

        let resolution = self
            .resolver
            .resolve_with_cancellation(
                requirements,
                candidates,
                cancellation,
            )
            .map_err(ExternalToolchainError::Resolution)?;

        // ------------------------------------------------------------
        // STAGE 4: POST-DISCOVERY VALIDATION
        // ------------------------------------------------------------
        //
        // Recheck executable authorization.
        //
        // Validate requirement identity, version constraints and
        // registered capability verification evidence.
        //
        // Unknown capability evidence remains unresolved.

        let validation = self
            .validator
            .validate(
                requirements,
                &resolution,
                policy,
            )
            .map_err(ExternalToolchainError::Validation)?;

        // ------------------------------------------------------------
        // STAGE 5: CONSOLIDATED REPORT
        // ------------------------------------------------------------
        //
        // The reporting subsystem is the sole authority for the
        // consolidated external-tool report schema.

        let report = ExternalToolReport::from_reports(
            requirements,
            &resolution,
            &validation,
        )
        .map_err(ExternalToolchainError::Reporting)?;

        Ok(ExternalToolchainOutcome::new(
            resolution,
            validation,
            report,
        ))
    }
}


// ============================================================================
// ORCHESTRATION ERRORS
// ============================================================================

/// Errors encountered while coordinating external-tool operations.
///
/// Underlying errors are retained as sources rather than being reduced
/// to unstructured success/failure messages.
#[derive(Debug)]
pub enum ExternalToolchainError {
    /// Declarative requirements are invalid.
    InvalidRequirements(RequirementError),

    /// A candidate executable failed pre-discovery authorization.
    CandidateAuthorization {
        /// Logical requirement identifier.
        requirement_id: String,

        /// Provider identifier.
        provider_id: String,

        /// Candidate identifier.
        candidate_id: String,

        /// Original authorization failure.
        source: CandidateAuthorizationError,
    },

    /// Resolution failed.
    Resolution(ResolutionError),

    /// Post-discovery validation failed.
    Validation(ValidationError),

    /// Consolidated reporting failed.
    Reporting(ReportingError),

    /// Cancellation occurred before discovery could safely begin.
    CancelledBeforeDiscovery,
}

impl fmt::Display for ExternalToolchainError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidRequirements(source) => {
                write!(
                    formatter,
                    "external-tool requirements are invalid: {source}"
                )
            }

            Self::CandidateAuthorization {
                requirement_id,
                provider_id,
                candidate_id,
                source,
            } => {
                write!(
                    formatter,
                    "external-tool candidate authorization failed \
                     for requirement `{requirement_id}`, \
                     provider `{provider_id}`, \
                     candidate `{candidate_id}`: {source}"
                )
            }

            Self::Resolution(source) => {
                write!(
                    formatter,
                    "external-tool resolution failed: {source}"
                )
            }

            Self::Validation(source) => {
                write!(
                    formatter,
                    "external-tool validation failed: {source}"
                )
            }

            Self::Reporting(source) => {
                write!(
                    formatter,
                    "external-tool reporting failed: {source}"
                )
            }

            Self::CancelledBeforeDiscovery => {
                formatter.write_str(
                    "external-tool operation was cancelled before discovery",
                )
            }
        }
    }
}

impl Error for ExternalToolchainError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidRequirements(source) => Some(source),

            Self::CandidateAuthorization {
                source,
                ..
            } => Some(source),

            Self::Resolution(source) => Some(source),

            Self::Validation(source) => Some(source),

            Self::Reporting(source) => Some(source),

            Self::CancelledBeforeDiscovery => None,
        }
    }
}


// ============================================================================
// PUBLIC CONVENIENCE FUNCTION
// ============================================================================

/// Resolve and validate external tools using the canonical backend
/// executable security policy.
///
/// This is a convenience wrapper around ExternalToolchainService.
///
/// It does not provide a default permissive security policy.
///
/// The caller must supply an explicitly configured validation policy.
pub fn resolve_external_tools(
    requirements: &ExternalToolRequirements,
    candidates: &ToolchainCandidates,
    policy: &ExternalToolValidationPolicy<'_>,
) -> ExternalToolchainResult<ExternalToolchainOutcome> {
    ExternalToolchainService::new().resolve_and_validate(
        requirements,
        candidates,
        policy,
    )
}


// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use std::path::PathBuf;

    fn empty_security_policy() -> ToolPolicy {
        ToolPolicy::new(
            std::iter::empty::<PathBuf>(),
        )
        .expect("an empty executable allowlist is valid")
    }

    #[test]
    fn external_tool_service_can_be_constructed() {
        let service = ExternalToolchainService::new();

        assert_eq!(
            std::mem::size_of_val(&service),
            std::mem::size_of::<ExternalToolchainService>()
        );
    }

    #[test]
    fn empty_requirements_produce_a_ready_report() {
        let requirements = ExternalToolRequirements::new();
        let candidates = ToolchainCandidates::new();

        let security = empty_security_policy();
        let policy = ExternalToolValidationPolicy::new(&security);

        let outcome = ExternalToolchainService::new()
            .resolve_and_validate(
                &requirements,
                &candidates,
                &policy,
            )
            .expect("empty requirements must be valid");

        assert!(outcome.is_ready());
        assert!(outcome.required_tools_validated());
        assert_eq!(outcome.requirement_count(), 0);
        assert_eq!(outcome.resolved_count(), 0);
        assert_eq!(outcome.validated_count(), 0);
    }

    #[test]
    fn missing_required_tool_does_not_report_readiness() {
        let mut requirements = ExternalToolRequirements::new();

        requirements
            .insert(
                ExternalToolRequirement::new(
                    "compiler.required",
                )
                .expect("requirement identifier must be valid"),
            )
            .expect("requirement must be inserted");

        let candidates = ToolchainCandidates::new();

        let security = empty_security_policy();
        let policy = ExternalToolValidationPolicy::new(&security);

        let outcome = ExternalToolchainService::new()
            .resolve_and_validate(
                &requirements,
                &candidates,
                &policy,
            )
            .expect("missing candidates are reported, not fabricated");

        assert!(!outcome.is_ready());
        assert!(!outcome.required_tools_validated());
        assert_eq!(outcome.requirement_count(), 1);
        assert_eq!(outcome.resolved_count(), 0);
        assert_eq!(outcome.validated_count(), 0);
    }

    #[test]
    fn pre_cancelled_operation_does_not_start_discovery() {
        let requirements = ExternalToolRequirements::new();
        let candidates = ToolchainCandidates::new();

        let security = empty_security_policy();
        let policy = ExternalToolValidationPolicy::new(&security);

        let cancellation = CancellationToken::new();
        cancellation.cancel();

        let result = ExternalToolchainService::new()
            .resolve_and_validate_with_cancellation(
                &requirements,
                &candidates,
                &policy,
                &cancellation,
            );

        assert!(matches!(
            result,
            Err(
                ExternalToolchainError::CancelledBeforeDiscovery
            )
        ));
    }

    #[test]
    fn report_schema_version_is_nonzero() {
        assert_ne!(
            EXTERNAL_TOOL_REPORT_SCHEMA_VERSION,
            0
        );
    }

    #[test]
    fn open_domain_identifiers_are_supported() {
        let domain = DomainId::new("future_computing_domain")
            .expect("domain identifiers must remain extensible");

        assert_eq!(
            domain.as_str(),
            "future_computing_domain"
        );
    }

    #[test]
    fn open_capability_identifiers_are_supported() {
        let capability = CapabilityId::new(
            "future.accelerated_computation",
        )
        .expect("capability identifiers must remain extensible");

        assert_eq!(
            capability.as_str(),
            "future.accelerated_computation"
        );
    }
}
