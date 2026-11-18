
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Toolchain — External Tool Command
//!
//! Path: src/toolchain/commands/tool.rs
//! Edition: Rust 2021
//! Minimum supported Rust: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Provides the developer-facing command/API boundary for inspecting,
//! resolving and validating external tools required by Zamani operations.
//!
//! This module delegates the actual work to the canonical
//! `external_tools::ExternalToolchainService`.
//!
//! ## Ownership
//!
//! This module owns:
//! - The external-tool command API.
//! - Command request and response boundaries.
//! - Command-level cancellation integration.
//! - Structured command errors.
//! - Human-readable and machine-readable report access.
//! - Command API versioning.
//!
//! ## Non-ownership
//!
//! This module does NOT own:
//! - Executable discovery.
//! - PATH resolution.
//! - Executable authorization.
//! - Tool version parsing.
//! - Process execution.
//! - Shell command construction.
//! - Package installation.
//! - Network downloading.
//! - Hardware discovery.
//! - Target selection.
//! - Compiler execution.
//! - Artifact generation.
//! - Runtime execution.
//! - Physical resource allocation.
//!
//! ## Authoritative integrations
//!
//! `external_tools/requirements.rs`:
//!     ExternalToolRequirements
//!     ExternalToolRequirement
//!     RequirementId
//!     DomainId
//!     CapabilityId
//!     VersionConstraint
//!
//! `external_tools/resolution.rs`:
//!     ToolchainCandidates
//!     ToolchainCandidate
//!     ExternalToolResolver
//!
//! `external_tools/validation.rs`:
//!     ExternalToolValidationPolicy
//!     ExternalToolValidator
//!     ToolCapabilityVerifier
//!
//! `external_tools/reporting.rs`:
//!     ExternalToolReport
//!     ExternalToolStatus
//!     Structured diagnostics
//!
//! `external_tools/mod.rs`:
//!     ExternalToolchainService
//!     ExternalToolchainOutcome
//!     ExternalToolchainError
//!
//! `backend/toolchain.rs`:
//!     ToolchainManager
//!     ToolchainSpec
//!     ToolVersion
//!     VersionRequirement
//!     ToolchainDescriptor
//!
//! `backend/process.rs`:
//!     CancellationToken
//!     ProcessRunner
//!     ProcessRequest
//!     ProcessOutput
//!
//! `backend/security.rs`:
//!     ToolPolicy
//!
//! ## Security invariants
//!
//! 1. No executable is authorized implicitly.
//! 2. No executable is probed before authorization.
//! 3. Discovery is not authorization.
//! 4. Version compatibility is not capability verification.
//! 5. Unknown capability evidence is not positive evidence.
//! 6. No command string is constructed or passed to a shell.
//! 7. No executable is invoked directly by this module.
//! 8. Human-readable reports are never used as authorization decisions.
//!
//! ## POCO-REAF
//!
//! This command is domain-neutral and supports open computational-domain
//! identifiers supplied by the canonical requirement subsystem.
//!
//! It does not impose universal limits on:
//! - Executables.
//! - Providers.
//! - Requirements.
//! - Candidates.
//! - Computational domains.
//! - Capabilities.
//! - Targets.
//! - CPUs or threads.
//! - GPUs, FPGAs or accelerators.
//! - QPUs or qubits.
//! - Distributed participants.
//! - Memory or storage.
//! - Tensor dimensions or rank.
//!
//! Operational budgets belong to the configuration and execution layers.
//!
//! No literal infinite hardware or execution capacity is promised.
//!
//! ## Integration
//!
//! The command receives already-constructed requirement and candidate
//! collections from the project configuration and provider-registration
//! layers.
//!
//! It passes these to ExternalToolchainService.
//!
//! The service performs authorization, discovery, resolution,
//! compatibility validation, capability verification and reporting.
//!
//! This command exposes the resulting evidence without replacing it.

use std::error::Error;
use std::fmt;

use crate::backend::process::CancellationToken;

use crate::toolchain::external_tools::{
    ExternalToolchainError,
    ExternalToolchainOutcome,
    ExternalToolchainService,
    ExternalToolRequirements,
    ExternalToolValidationPolicy,
    ToolchainCandidates,
};

/// Stable architectural identity of this command.
pub const TOOL_COMMAND_MODULE_NAME: &str =
    "zamani.toolchain.commands.tool";

/// Public API version of the external-tool command.
///
/// This version is independent of the Zamani language version,
/// grammar version, compiler version, backend version and external
/// executable versions.
pub const TOOL_COMMAND_API_VERSION: u16 = 1;

/// Stable schema identifier for command-level report consumers.
pub const TOOL_COMMAND_REPORT_SCHEMA: &str =
    "zamani.toolchain.tool-report.v1";

/// Stateless external-tool command service.
///
/// This service owns no executable registry, provider inventory,
/// filesystem state, credentials, process runner or mutable global state.
///
/// All authoritative operations are delegated to the existing
/// external-tool orchestration subsystem.
///
/// Independent instances can safely be created for different
/// projects, workspaces and build requests.
#[derive(Debug, Default, Clone, Copy)]
pub struct ToolCommandService;

impl ToolCommandService {
    /// Construct the stateless external-tool command service.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Resolve and validate all declared external-tool requirements.
    ///
    /// The request supplies:
    ///
    /// - Canonical external-tool requirements.
    /// - Explicitly registered executable candidates.
    /// - An explicit executable authorization and validation policy.
    /// - A cooperative cancellation token.
    ///
    /// The command does not invent executable candidates or assume
    /// that an executable is available merely because its logical
    /// requirement has been declared.
    ///
    /// Security authorization is performed by the authoritative
    /// external-tool service before executable discovery.
    ///
    /// Required tools must pass the complete validation process before
    /// the returned result can report readiness.
    ///
    /// Optional tool failures are preserved in the report but do not
    /// independently prevent required-tool readiness.
    ///
    /// A ready result establishes only that the requested external-tool
    /// prerequisites have been validated. It does not establish:
    ///
    /// - Target feasibility.
    /// - Hardware availability.
    /// - Physical resource allocation.
    /// - Successful compilation.
    /// - Artifact validity.
    /// - Successful execution.
    ///
    /// # Errors
    ///
    /// Returns a structured error if:
    ///
    /// - Requirements are invalid.
    /// - Candidate configuration is invalid.
    /// - Executable authorization fails.
    /// - Backend discovery fails.
    /// - Resolution cannot be completed.
    /// - Validation encounters a fatal error.
    /// - Report construction fails.
    ///
    /// Cancellation is represented according to the authoritative
    /// external-tool orchestration contract.
    pub fn execute(
        &self,
        request: &ToolCommandRequest<'_>,
    ) -> ToolCommandResult<ToolCommandOutcome> {
        let service = ExternalToolchainService::new();

        let outcome = service
            .resolve_and_validate_with_cancellation(
                request.requirements,
                request.candidates,
                request.policy,
                request.cancellation,
            )
            .map_err(ToolCommandError::Orchestration)?;

        Ok(ToolCommandOutcome::new(outcome))
    }
}

/// Complete input to one external-tool command invocation.
///
/// All referenced objects are borrowed. The command does not take
/// ownership of project configuration, provider registrations,
/// executable policies or cancellation state.
pub struct ToolCommandRequest<'a> {
    requirements: &'a ExternalToolRequirements,
    candidates: &'a ToolchainCandidates,
    policy: &'a ExternalToolValidationPolicy<'a>,
    cancellation: &'a CancellationToken,
}

impl<'a> ToolCommandRequest<'a> {
    /// Construct an external-tool command request.
    ///
    /// The caller must provide the canonical requirements, candidate
    /// collection, explicit validation policy and cancellation token.
    #[must_use]
    pub const fn new(
        requirements: &'a ExternalToolRequirements,
        candidates: &'a ToolchainCandidates,
        policy: &'a ExternalToolValidationPolicy<'a>,
        cancellation: &'a CancellationToken,
    ) -> Self {
        Self {
            requirements,
            candidates,
            policy,
            cancellation,
        }
    }

    /// Return the declared external-tool requirements.
    #[must_use]
    pub const fn requirements(&self) -> &'a ExternalToolRequirements {
        self.requirements
    }

    /// Return the registered executable candidates.
    #[must_use]
    pub const fn candidates(&self) -> &'a ToolchainCandidates {
        self.candidates
    }

    /// Return the configured validation policy.
    #[must_use]
    pub const fn policy(
        &self,
    ) -> &'a ExternalToolValidationPolicy<'a> {
        self.policy
    }

    /// Return the cancellation token.
    #[must_use]
    pub const fn cancellation(&self) -> &'a CancellationToken {
        self.cancellation
    }
}

/// Standard result type for external-tool command operations.
pub type ToolCommandResult<T> = Result<T, ToolCommandError>;

/// Structured errors originating from the external-tool command.
///
/// Original orchestration and serialization errors are retained as
/// causal sources.
#[derive(Debug)]
pub enum ToolCommandError {
    /// The authoritative external-tool orchestration failed.
    Orchestration(ExternalToolchainError),

    /// Machine-readable report serialization failed.
    Serialization(serde_json::Error),
}

impl fmt::Display for ToolCommandError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Orchestration(source) => {
                write!(
                    formatter,
                    "external-tool command failed: {source}"
                )
            }

            Self::Serialization(source) => {
                write!(
                    formatter,
                    "external-tool report serialization failed: {source}"
                )
            }
        }
    }
}

impl Error for ToolCommandError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Orchestration(source) => Some(source),
            Self::Serialization(source) => Some(source),
        }
    }
}

impl From<ExternalToolchainError> for ToolCommandError {
    fn from(source: ExternalToolchainError) -> Self {
        Self::Orchestration(source)
    }
}

impl From<serde_json::Error> for ToolCommandError {
    fn from(source: serde_json::Error) -> Self {
        Self::Serialization(source)
    }
}

/// Successful completion of the command orchestration stage.
///
/// This object preserves the complete authoritative outcome rather
/// than reducing it to a boolean or discarding diagnostic evidence.
pub struct ToolCommandOutcome {
    outcome: ExternalToolchainOutcome,
}

impl ToolCommandOutcome {
    /// Construct a command outcome from the canonical service result.
    fn new(outcome: ExternalToolchainOutcome) -> Self {
        Self { outcome }
    }

    /// Return the original external-tool orchestration outcome.
    #[must_use]
    pub const fn orchestration(&self) -> &ExternalToolchainOutcome {
        &self.outcome
    }

    /// Return the authoritative external-tool readiness report.
    ///
    /// This report contains per-requirement statuses, diagnostic
    /// evidence and aggregate readiness information.
    #[must_use]
    pub fn report(
        &self,
    ) -> &crate::toolchain::external_tools::ExternalToolReport {
        self.outcome.report()
    }

    /// Return the original resolution evidence.
    #[must_use]
    pub const fn resolution(
        &self,
    ) -> &crate::toolchain::external_tools::ResolutionReport {
        self.outcome.resolution()
    }

    /// Return the original validation evidence.
    #[must_use]
    pub const fn validation(
        &self,
    ) -> &crate::toolchain::external_tools::ValidationReport {
        self.outcome.validation()
    }

    /// Return whether all mandatory external-tool requirements passed.
    ///
    /// Optional tool failures do not independently prevent this result.
    #[must_use]
    pub const fn required_tools_validated(&self) -> bool {
        self.outcome.required_tools_validated()
    }

    /// Return whether dependent operations may proceed with respect
    /// to external-tool prerequisites.
    ///
    /// This is not a target-feasibility or execution-readiness result.
    #[must_use]
    pub const fn is_ready(&self) -> bool {
        self.outcome.is_ready()
    }

    /// Return whether the operation was cancelled.
    #[must_use]
    pub const fn was_cancelled(&self) -> bool {
        self.outcome.was_cancelled()
    }

    /// Return the declared requirement count.
    #[must_use]
    pub fn requirement_count(&self) -> usize {
        self.outcome.requirement_count()
    }

    /// Return the number of successfully resolved executables.
    #[must_use]
    pub fn resolved_count(&self) -> usize {
        self.outcome.resolved_count()
    }

    /// Return the number of successfully validated executables.
    #[must_use]
    pub fn validated_count(&self) -> usize {
        self.outcome.validated_count()
    }

    /// Render the authoritative report for human consumption.
    ///
    /// This is presentation only.
    ///
    /// Consumers must use is_ready() or the structured report
    /// for programmatic admission decisions.
    #[must_use]
    pub fn render_text(&self) -> String {
        self.outcome.render_text()
    }

    /// Serialize the authoritative report into compact JSON.
    ///
    /// JSON serialization does not alter validation status or
    /// authorization decisions.
    pub fn to_json(&self) -> ToolCommandResult<String> {
        self.outcome
            .to_json()
            .map_err(ToolCommandError::Serialization)
    }

    /// Serialize the authoritative report into pretty JSON.
    ///
    /// For very large reports, callers should avoid unnecessary
    /// repeated serialization and should prefer the structured
    /// report accessors when direct inspection is sufficient.
    pub fn to_json_pretty(&self) -> ToolCommandResult<String> {
        self.outcome
            .to_json_pretty()
            .map_err(ToolCommandError::Serialization)
    }

    /// Consume this result and return the complete orchestration outcome.
    #[must_use]
    pub fn into_orchestration(self) -> ExternalToolchainOutcome {
        self.outcome
    }
}

/// Select the presentation format for a command report.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ToolReportFormat {
    /// Human-readable terminal presentation.
    #[default]
    Text,

    /// Compact machine-readable JSON.
    Json,

    /// Pretty-printed machine-readable JSON.
    JsonPretty,
}

impl ToolReportFormat {
    /// Return the stable format identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Text => "text",
            Self::Json => "json",
            Self::JsonPretty => "json_pretty",
        }
    }
}

/// Render a completed tool command result in the requested format.
///
/// The underlying structured result remains available independently
/// of its rendering.
///
/// # Errors
///
/// Returns a serialization error if JSON generation fails.
pub fn render_tool_report(
    outcome: &ToolCommandOutcome,
    format: ToolReportFormat,
) -> ToolCommandResult<String> {
    match format {
        ToolReportFormat::Text => Ok(outcome.render_text()),
        ToolReportFormat::Json => outcome.to_json(),
        ToolReportFormat::JsonPretty => outcome.to_json_pretty(),
    }
}

/// Return the external-tool command API version.
#[must_use]
pub const fn api_version() -> u16 {
    TOOL_COMMAND_API_VERSION
}

/// Return the stable command report schema identifier.
#[must_use]
pub const fn report_schema() -> &'static str {
    TOOL_COMMAND_REPORT_SCHEMA
}

/// Return the stable architectural module identity.
#[must_use]
pub const fn module_name() -> &'static str {
    TOOL_COMMAND_MODULE_NAME
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_api_is_versioned() {
        assert_eq!(api_version(), 1);
    }

    #[test]
    fn command_report_schema_is_stable() {
        assert_eq!(
            report_schema(),
            "zamani.toolchain.tool-report.v1"
        );
    }

    #[test]
    fn command_module_identity_is_stable() {
        assert_eq!(
            module_name(),
            "zamani.toolchain.commands.tool"
        );
    }

    #[test]
    fn report_formats_have_stable_identifiers() {
        assert_eq!(ToolReportFormat::Text.as_str(), "text");
        assert_eq!(ToolReportFormat::Json.as_str(), "json");
        assert_eq!(
            ToolReportFormat::JsonPretty.as_str(),
            "json_pretty"
        );
    }

    #[test]
    fn default_report_format_is_human_readable() {
        assert_eq!(
            ToolReportFormat::default(),
            ToolReportFormat::Text
        );
    }

    #[test]
    fn report_formats_are_distinct() {
        assert_ne!(
            ToolReportFormat::Text,
            ToolReportFormat::Json
        );

        assert_ne!(
            ToolReportFormat::Json,
            ToolReportFormat::JsonPretty
        );
    }
}
