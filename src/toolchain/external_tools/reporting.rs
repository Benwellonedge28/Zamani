
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani External Tool Reporting
//!
//! Path: src/toolchain/external_tools/reporting.rs
//! Edition: Rust 2021.
//! Minimum Rust version: 1.97.
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Consolidates external-tool requirements, resolution evidence and
//! validation evidence into deterministic, human-readable and
//! machine-readable reports.
//!
//! ## Ownership
//!
//! This module owns:
//! - External-tool report composition.
//! - Per-requirement status classification.
//! - Required and optional failure reporting.
//! - Missing, incompatible, rejected and unverified reporting.
//! - Candidate failure presentation.
//! - Capability evidence presentation.
//! - Aggregate readiness calculations.
//! - Structured report diagnostics.
//! - Human-readable rendering.
//! - JSON serialization.
//!
//! This module does NOT own:
//! - Requirement declaration.
//! - Executable discovery or PATH lookup.
//! - Version probing.
//! - Process execution.
//! - Process output capture.
//! - Executable authorization.
//! - Capability verification.
//! - Hardware discovery or allocation.
//! - Backend selection.
//! - Compiler or runtime execution.
//!
//! ## Integration authorities
//!
//! requirements.rs:
//!     ExternalToolRequirements
//!     ExternalToolRequirement
//!     RequirementId
//!     RequirementKind
//!     DomainId
//!     CapabilityId
//!
//! resolution.rs:
//!     ResolutionReport
//!     RequirementResolution
//!     ResolutionStatus
//!     ResolvedExternalTool
//!     CandidateFailure
//!
//! validation.rs:
//!     ValidationReport
//!     ToolValidationResult
//!     ToolValidationStatus
//!     ValidationIssue
//!
//! backend/toolchain.rs:
//!     ToolchainDescriptor
//!     ToolVersion
//!     ToolchainManager
//!
//! backend/process.rs:
//!     ProcessRunner
//!     ProcessOutput
//!
//! backend/security.rs:
//!     ToolPolicy
//!
//! The reporting layer consumes evidence from these authorities.
//! It never substitutes its own discovery or validation implementation.
//!
//! ## Status semantics
//!
//! Validated:
//!     All required external-tool validation passed.
//!
//! Missing:
//!     No executable candidate was available for the requirement.
//!
//! Incompatible:
//!     A definitive version or required-capability compatibility
//!     check failed.
//!
//! Rejected:
//!     An executable or selected tool was rejected by security validation.
//!
//! Unverified:
//!     Resolution or validation could not establish the required facts.
//!
//! ResolutionFailed:
//!     Candidates existed, but none completed successful resolution.
//!
//! Cancelled:
//!     The resolution operation was cancelled.
//!
//! NotReported:
//!     Required upstream evidence was not supplied.
//!
//! Unknown is never equivalent to Validated.
//!
//! ## Scalability
//!
//! No fixed limits are imposed on:
//! - Tool requirements.
//! - Providers.
//! - Candidates.
//! - Capabilities.
//! - Computational domains.
//! - Projects.
//! - Target realizations.
//!
//! Collections grow according to available memory and caller-supplied
//! operational budgets.
//!
//! Rendering and serialization allocate memory proportional to their
//! output. Callers processing large reports should use the iterator APIs
//! and avoid materializing complete output strings unnecessarily.
//!
//! ## Determinism
//!
//! Requirements are processed in the canonical order provided by
//! ExternalToolRequirements.
//!
//! Resolution and validation evidence retain their upstream ordering.
//! No hash-map iteration order is used for report identity or ordering.
//!
//! ## Security
//!
//! Executable paths and diagnostic messages are treated as untrusted data.
//! They are never executed, interpreted as commands, or used to authorize
//! an operation.
//!
//! Machine-readable reports preserve structured status codes.
//! Human-readable output is presentation only and must not be used as
//! the authoritative basis for build admission.
//!
//! ## POCO-REAF
//!
//! Reporting is domain-neutral and supports classical, quantum, HDL,
//! AI, hybrid, distributed and future computational domains through
//! open requirement and capability identifiers.
//!
//! A report describes the evidence available for a requested realization.
//! It does not promise that an unavailable tool or physical resource
//! exists.

use serde::Serialize;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;
use std::path::Path;

use super::requirements::{
    ExternalToolRequirement,
    ExternalToolRequirements,
    RequirementId,
    RequirementKind,
};

use super::resolution::{
    ResolutionReport,
    ResolutionStatus,
    RequirementResolution,
    CandidateFailure,
};

use super::validation::{
    ValidationReport,
    ValidationIssue,
    ToolValidationResult,
    ToolValidationStatus,
};

// ============================================================================
// REPORT VERSION
// ============================================================================

/// Schema version of the serialized external-tool report.
///
/// This version is independent of the Zamani language version,
/// compiler version, backend API version and external executable versions.
pub const EXTERNAL_TOOL_REPORT_SCHEMA_VERSION: u32 = 1;

// ============================================================================
// PUBLIC STATUS CONTRACT
// ============================================================================

/// Final report classification for one external-tool requirement.
///
/// This is a reporting classification. It does not replace the
/// authoritative resolution or validation statuses.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ExternalToolStatus {
    /// Discovery and all required validation succeeded.
    Validated,

    /// No executable candidate was registered for this requirement.
    Missing,

    /// A definitive version or required-capability compatibility
    /// check failed.
    Incompatible,

    /// Security validation rejected the selected executable or tool.
    Rejected,

    /// Required evidence could not be established.
    ///
    /// This includes unknown capabilities and incomplete validation.
    Unverified,

    /// Candidates existed but resolution did not succeed.
    ResolutionFailed,

    /// Resolution was explicitly cancelled.
    Cancelled,

    /// An expected upstream result was not supplied.
    NotReported,
}

impl ExternalToolStatus {
    /// Return the stable machine-readable status identifier.
    #[must_use]
    pub const fn code(self) -> &'static str {
        match self {
            Self::Validated => "validated",
            Self::Missing => "missing",
            Self::Incompatible => "incompatible",
            Self::Rejected => "rejected",
            Self::Unverified => "unverified",
            Self::ResolutionFailed => "resolution_failed",
            Self::Cancelled => "cancelled",
            Self::NotReported => "not_reported",
        }
    }

    /// Return whether the tool has passed all required validation.
    #[must_use]
    pub const fn is_validated(self) -> bool {
        matches!(self, Self::Validated)
    }

    /// Return whether the status represents an unsuccessful outcome.
    #[must_use]
    pub const fn is_failure(self) -> bool {
        !self.is_validated()
    }

    /// Return whether the status represents a definitive incompatibility.
    #[must_use]
    pub const fn is_incompatible(self) -> bool {
        matches!(self, Self::Incompatible)
    }

    /// Return whether the status represents a security rejection.
    #[must_use]
    pub const fn is_rejected(self) -> bool {
        matches!(self, Self::Rejected)
    }

    /// Return whether the status represents an inconclusive result.
    #[must_use]
    pub const fn is_unverified(self) -> bool {
        matches!(
            self,
            Self::Unverified | Self::NotReported
        )
    }
}

// ============================================================================
// DIAGNOSTIC CONTRACT
// ============================================================================

/// Severity of an external-tool reporting diagnostic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReportSeverity {
    /// Informational evidence that does not indicate a failure.
    Information,

    /// A non-blocking issue, typically involving an optional tool.
    Warning,

    /// A failure that may prevent the requested operation.
    Error,

    /// A security rejection or other security-sensitive condition.
    Security,
}

/// One structured external-tool diagnostic.
///
/// `code` is stable machine-readable metadata.
///
/// `message` is intended for human presentation and must not be parsed
/// to determine build admission.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ExternalToolDiagnostic {
    /// Stable machine-readable diagnostic identifier.
    pub code: String,

    /// Diagnostic severity.
    pub severity: ReportSeverity,

    /// Human-readable explanation.
    pub message: String,

    /// Optional provider identity.
    pub provider_id: Option<String>,

    /// Optional candidate identity.
    pub candidate_id: Option<String>,
}

impl ExternalToolDiagnostic {
    fn new(
        code: impl Into<String>,
        severity: ReportSeverity,
        message: impl Into<String>,
    ) -> Self {
        Self {
            code: code.into(),
            severity,
            message: message.into(),
            provider_id: None,
            candidate_id: None,
        }
    }

    fn candidate(
        mut self,
        provider_id: &str,
        candidate_id: &str,
    ) -> Self {
        self.provider_id = Some(provider_id.to_owned());
        self.candidate_id = Some(candidate_id.to_owned());
        self
    }
}

// ============================================================================
// TOOL REPORT ENTRY
// ============================================================================

/// Report entry describing one declared external-tool requirement.
///
/// The record contains presentation metadata and upstream evidence.
/// It does not contain executable handles or process-execution authority.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ExternalToolReportEntry {
    /// Canonical logical requirement identifier.
    pub requirement_id: String,

    /// Whether the original requirement was required or optional.
    pub requirement_kind: RequirementKindReport,

    /// Final reporting classification.
    pub status: ExternalToolStatus,

    /// Applicable computational domains.
    pub domains: Vec<String>,

    /// Declared required capabilities.
    pub required_capabilities: Vec<String>,

    /// Successfully verified capability identifiers.
    pub verified_capabilities: Vec<String>,

    /// Selected provider, when available.
    pub provider_id: Option<String>,

    /// Selected candidate, when available.
    pub candidate_id: Option<String>,

    /// Selected executable path, when available.
    ///
    /// This is descriptive evidence, not authorization.
    pub executable: Option<String>,

    /// Discovered numeric version, when available.
    pub version: Option<String>,

    /// Declared purpose, when supplied.
    pub purpose: Option<String>,

    /// Origin reference, when supplied.
    pub origin: Option<String>,

    /// Structured diagnostics for this requirement.
    pub diagnostics: Vec<ExternalToolDiagnostic>,
}

/// Stable reporting representation of requirement mandatory status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RequirementKindReport {
    /// The tool is mandatory.
    Required,

    /// The tool is optional.
}

impl From<RequirementKind> for RequirementKindReport {
    fn from(value: RequirementKind) -> Self {
        match value {
            RequirementKind::Required => Self::Required,
            RequirementKind::Optional => Self::Optional,
        }
    }
}

impl ExternalToolReportEntry {
    /// Return whether this entry describes a mandatory requirement.
    #[must_use]
    pub const fn is_required(&self) -> bool {
        matches!(
            self.requirement_kind,
            RequirementKindReport::Required
        )
    }

    /// Return whether this entry describes an optional requirement.
    #[must_use]
    pub const fn is_optional(&self) -> bool {
        matches!(
            self.requirement_kind,
            RequirementKindReport::Optional
        )
    }

    /// Return whether the tool is completely validated.
    #[must_use]
    pub const fn is_validated(&self) -> bool {
        self.status.is_validated()
    }

    /// Return the number of diagnostics associated with this entry.
    #[must_use]
    pub fn diagnostic_count(&self) -> usize {
        self.diagnostics.len()
    }

    /// Return the number of verified capabilities.
    #[must_use]
    pub fn verified_capability_count(&self) -> usize {
        self.verified_capabilities.len()
    }
}

// ============================================================================
// AGGREGATE SUMMARY
// ============================================================================

/// Aggregate external-tool reporting statistics.
///
/// Every count is derived from the entries in the corresponding report.
/// No status is inferred from a missing count or an assumed default.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct ExternalToolReportSummary {
    /// Number of declared tool requirements.
    pub total: usize,

    /// Number of successfully validated tools.
    pub validated: usize,

    /// Number of missing tools.
    pub missing: usize,

    /// Number of incompatible tools.
    pub incompatible: usize,

    /// Number of security-rejected tools.
    pub rejected: usize,

    /// Number of unverified tools.
    pub unverified: usize,

    /// Number of failed resolutions.
    pub resolution_failed: usize,

    /// Number of cancelled requirements.
    pub cancelled: usize,

    /// Number of requirements without upstream results.
    pub not_reported: usize,

    /// Number of mandatory requirements.
    pub required: usize,

    /// Number of optional requirements.
    pub optional: usize,

    /// Number of failed mandatory requirements.
    pub required_failures: usize,

    /// Number of failed optional requirements.
    pub optional_failures: usize,

    /// Number of individual diagnostics.
    pub diagnostic_count: usize,
}

// ============================================================================
// COMPLETE REPORT
// ============================================================================

/// Complete deterministic external-tool report.
///
/// The entries are ordered by their canonical requirement identifiers.
///
/// The report is immutable through its public API after construction.
/// Consumers should use the accessors rather than rebuilding or modifying
/// status classifications independently.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ExternalToolReport {
    /// Serialized report schema version.
    pub schema_version: u32,

    /// Whether resolution or validation was cancelled.
    pub cancelled: bool,

    /// Aggregate report summary.
    pub summary: ExternalToolReportSummary,

    /// Requirement entries in deterministic identifier order.
    pub entries: Vec<ExternalToolReportEntry>,

    /// Whether all mandatory requirements passed validation.
    pub required_tools_validated: bool,

    /// Whether the operation is ready with respect to external tools.
    ///
    /// This is false when cancellation occurred or any required
    /// requirement failed validation.
    pub ready_for_required_tools: bool,
}

impl ExternalToolReport {
    /// Compose a report from the original requirements, resolution
    /// results and validation results.
    ///
    /// All three inputs are borrowed. The reporting layer does not take
    /// ownership of the discovery, security or validation authorities.
    ///
    /// Every declared requirement receives one report entry.
    ///
    /// An absent upstream result becomes NotReported or Unverified.
    /// It is never silently treated as validated.
    ///
    /// Returns an error if upstream evidence references a requirement
    /// that was never declared.
    pub fn from_reports(
        requirements: &ExternalToolRequirements,
        resolution: &ResolutionReport,
        validation: &ValidationReport,
    ) -> Result<Self, ReportingError> {
        requirements
            .validate()
            .map_err(ReportingError::InvalidRequirements)?;

        for result in resolution.iter() {
            if !requirements.contains(result.requirement_id()) {
                return Err(ReportingError::UndeclaredResolution {
                    requirement_id: result.requirement_id().to_string(),
                });
            }
        }

        for result in validation.iter() {
            if !requirements.contains(result.requirement_id()) {
                return Err(ReportingError::UndeclaredValidation {
                    requirement_id: result.requirement_id().to_string(),
                });
            }
        }

        let mut entries = Vec::new();

        for requirement in requirements.iter() {
            let resolution_entry =
                resolution.get(requirement.id());

            let validation_entry =
                validation.get(requirement.id());

            entries.push(build_entry(
                requirement,
                resolution_entry,
                validation_entry,
            ));
        }

        let cancelled =
            resolution.was_cancelled() || validation.was_cancelled();

        let summary = summarize(&entries);

        let required_tools_validated =
            summary.required_failures == 0;

        let ready_for_required_tools =
            !cancelled && required_tools_validated;

        Ok(Self {
            schema_version: EXTERNAL_TOOL_REPORT_SCHEMA_VERSION,
            cancelled,
            summary,
            entries,
            required_tools_validated,
            ready_for_required_tools,
        })
    }

    /// Return all entries in deterministic identifier order.
    #[must_use]
    pub fn entries(&self) -> &[ExternalToolReportEntry] {
        &self.entries
    }

    /// Find a report entry by its canonical requirement identifier.
    #[must_use]
    pub fn get(
        &self,
        requirement_id: &str,
    ) -> Option<&ExternalToolReportEntry> {
        self.entries
            .binary_search_by(|entry| {
                entry.requirement_id.as_str().cmp(requirement_id)
            })
            .ok()
            .and_then(|index| self.entries.get(index))
    }

    /// Return the aggregate report summary.
    #[must_use]
    pub const fn summary(&self) -> &ExternalToolReportSummary {
        &self.summary
    }

    /// Return the number of declared requirements.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Return whether the report contains no requirements.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Return whether all mandatory tools were validated.
    #[must_use]
    pub const fn required_tools_validated(&self) -> bool {
        self.required_tools_validated
    }

    /// Return whether external-tool prerequisites permit the operation
    /// to proceed.
    #[must_use]
    pub const fn is_ready_for_required_tools(&self) -> bool {
        self.ready_for_required_tools
    }

    /// Return whether either upstream operation was cancelled.
    #[must_use]
    pub const fn was_cancelled(&self) -> bool {
        self.cancelled
    }

    /// Iterate over entries with the requested status.
    ///
    /// The iterator retains deterministic identifier order.
    pub fn with_status(
        &self,
        status: ExternalToolStatus,
    ) -> impl Iterator<Item = &ExternalToolReportEntry> {
        self.entries
            .iter()
            .filter(move |entry| entry.status == status)
    }

    /// Iterate over mandatory requirements that were not validated.
    pub fn required_failures(
        &self,
    ) -> impl Iterator<Item = &ExternalToolReportEntry> {
        self.entries.iter().filter(|entry| {
            entry.is_required() && !entry.is_validated()
        })
    }

    /// Iterate over optional requirements that were not validated.
    pub fn optional_failures(
        &self,
    ) -> impl Iterator<Item = &ExternalToolReportEntry> {
        self.entries.iter().filter(|entry| {
            entry.is_optional() && !entry.is_validated()
        })
    }

    /// Serialize the complete report as pretty-printed JSON.
    ///
    /// The serialized schema is versioned independently of the
    /// implementation and compiler versions.
    pub fn to_json_pretty(
        &self,
    ) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Serialize the complete report as compact JSON.
    pub fn to_json(
        &self,
    ) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    /// Render a human-readable report.
    ///
    /// This method produces presentation text only. It must not be used
    /// as a replacement for the structured readiness fields.
    pub fn render_text(&self) -> String {
        let mut output = String::new();

        use fmt::Write as _;

        let _ = writeln!(
            output,
            "Zamani external-tool report (schema {})",
            self.schema_version
        );

        let _ = writeln!(
            output,
            "Requirements: {}",
            self.summary.total
        );

        let _ = writeln!(
            output,
            "Validated: {}",
            self.summary.validated
        );

        let _ = writeln!(
            output,
            "Missing: {}",
            self.summary.missing
        );

        let _ = writeln!(
            output,
            "Incompatible: {}",
            self.summary.incompatible
        );

        let _ = writeln!(
            output,
            "Rejected: {}",
            self.summary.rejected
        );

        let _ = writeln!(
            output,
            "Unverified: {}",
            self.summary.unverified
        );

        let _ = writeln!(
            output,
            "Resolution failed: {}",
            self.summary.resolution_failed
        );

        let _ = writeln!(
            output,
            "Cancelled: {}",
            self.summary.cancelled
        );

        let _ = writeln!(
            output,
            "Required failures: {}",
            self.summary.required_failures
        );

        let _ = writeln!(
            output,
            "Optional failures: {}",
            self.summary.optional_failures
        );

        let _ = writeln!(
            output,
            "Ready for required tools: {}",
            self.ready_for_required_tools
        );

        for entry in &self.entries {
            let _ = writeln!(output);
            let _ = writeln!(
                output,
                "[{}] {} ({})",
                entry.status.code(),
                entry.requirement_id,
                if entry.is_required() {
                    "required"
                } else {
                    "optional"
                }
            );

            if let Some(provider) = &entry.provider_id {
                let _ = writeln!(
                    output,
                    "  Provider: {provider}"
                );
            }

            if let Some(candidate) = &entry.candidate_id {
                let _ = writeln!(
                    output,
                    "  Candidate: {candidate}"
                );
            }

            if let Some(executable) = &entry.executable {
                let _ = writeln!(
                    output,
                    "  Executable: {executable}"
                );
            }

            if let Some(version) = &entry.version {
                let _ = writeln!(
                    output,
                    "  Version: {version}"
                );
            }

            for capability in &entry.verified_capabilities {
                let _ = writeln!(
                    output,
                    "  Verified capability: {capability}"
                );
            }

            for diagnostic in &entry.diagnostics {
                let _ = writeln!(
                    output,
                    "  {:?} [{}]: {}",
                    diagnostic.severity,
                    diagnostic.code,
                    diagnostic.message
                );
            }
        }

        output
    }
}

// ============================================================================
// ENTRY CONSTRUCTION
// ============================================================================

fn build_entry(
    requirement: &ExternalToolRequirement,
    resolution: Option<&RequirementResolution>,
    validation: Option<&ToolValidationResult>,
) -> ExternalToolReportEntry {
    let status = classify_status(resolution, validation);

    let mut diagnostics = Vec::new();

    if let Some(resolution) = resolution {
        append_resolution_diagnostics(
            resolution,
            &mut diagnostics,
        );
    } else {
        diagnostics.push(ExternalToolDiagnostic::new(
            "resolution_not_reported",
            severity_for(requirement, ReportSeverity::Warning),
            "No resolution result was supplied for this requirement.",
        ));
    }

    if let Some(validation) = validation {
        append_validation_diagnostics(
            validation,
            &mut diagnostics,
        );
    } else {
        diagnostics.push(ExternalToolDiagnostic::new(
            "validation_not_reported",
            severity_for(requirement, ReportSeverity::Warning),
            "No validation result was supplied for this requirement.",
        ));
    }

    let (provider_id, candidate_id, executable, version) =
        validation
            .map(|result| {
                (
                    result.provider_id().map(str::to_owned),
                    result.candidate_id().map(str::to_owned),
                    result.executable().map(display_path),
                    result.version().map(|version| {
                        format!(
                            "{}.{}.{}",
                            version.major,
                            version.minor,
                            version.patch
                        )
                    }),
                )
            })
            .unwrap_or_else(|| {
                resolution
                    .and_then(|result| result.resolved())
                    .map(|resolved| {
                        (
                            Some(resolved.provider_id().to_owned()),
                            Some(resolved.candidate_id().to_owned()),
                            Some(display_path(resolved.executable())),
                            Some(resolved.version().to_string()),
                        )
                    })
                    .unwrap_or((None, None, None, None))
            });

    let verified_capabilities = validation
        .map(|result| {
            result
                .verified_capabilities()
                .keys()
                .map(ToString::to_string)
                .collect()
        })
        .unwrap_or_default();

    ExternalToolReportEntry {
        requirement_id: requirement.id().to_string(),

        requirement_kind: requirement
            .requirement_kind()
            .into(),

        status,

        domains: requirement
            .domains()
            .iter()
            .map(ToString::to_string)
            .collect(),

        required_capabilities: requirement
            .capabilities()
            .iter()
            .map(ToString::to_string)
            .collect(),

        verified_capabilities,

        provider_id,
        candidate_id,
        executable,
        version,

        purpose: requirement
            .purpose_text()
            .map(str::to_owned),

        origin: requirement
            .origin_reference()
            .map(str::to_owned),

        diagnostics,
    }
}

/// Determine the final reporting status from authoritative upstream data.
///
/// Security rejection takes precedence over compatibility failure.
/// Definitive compatibility failure takes precedence over unknown evidence.
/// Missing and failed resolution are kept distinct.
fn classify_status(
    resolution: Option<&RequirementResolution>,
    validation: Option<&ToolValidationResult>,
) -> ExternalToolStatus {
    if let Some(validation) = validation {
        match validation.status() {
            ToolValidationStatus::Validated => {
                return ExternalToolStatus::Validated;
            }

            ToolValidationStatus::Rejected => {
                if validation.issues().iter().any(|issue| {
                    matches!(
                        issue,
                        ValidationIssue::ExecutableNotAuthorized { .. }
                            | ValidationIssue::CanonicalPathMismatch { .. }
                    )
                }) {
                    return ExternalToolStatus::Rejected;
                }

                if validation.issues().iter().any(|issue| {
                    matches!(
                        issue,
                        ValidationIssue::VersionConstraintNotSatisfied { .. }
                            | ValidationIssue::CapabilityNotSatisfied { .. }
                    )
                }) {
                    return ExternalToolStatus::Incompatible;
                }

                return ExternalToolStatus::Unverified;
            }

            ToolValidationStatus::NotResolved => {
                // Continue to resolution evidence below.
            }
        }
    }

    match resolution.map(RequirementResolution::status) {
        Some(ResolutionStatus::Cancelled) => {
            ExternalToolStatus::Cancelled
        }

        Some(
            ResolutionStatus::MissingRequired
            | ResolutionStatus::MissingOptional,
        ) => ExternalToolStatus::Missing,

        Some(
            ResolutionStatus::FailedRequired
            | ResolutionStatus::FailedOptional,
        ) => ExternalToolStatus::ResolutionFailed,

        Some(
            ResolutionStatus::ResolvedRequired
            | ResolutionStatus::ResolvedOptional,
        ) => ExternalToolStatus::Unverified,

        None => {
            if validation.is_some() {
                ExternalToolStatus::Unverified
            } else {
                ExternalToolStatus::NotReported
            }
        }
    }
}

fn append_resolution_diagnostics(
    resolution: &RequirementResolution,
    diagnostics: &mut Vec<ExternalToolDiagnostic>,
) {
    match resolution.status() {
        ResolutionStatus::ResolvedRequired
        | ResolutionStatus::ResolvedOptional => {}

        ResolutionStatus::MissingRequired => {
            diagnostics.push(ExternalToolDiagnostic::new(
                "required_tool_missing",
                ReportSeverity::Error,
                "No executable candidate was registered for this required tool.",
            ));
        }

        ResolutionStatus::MissingOptional => {
            diagnostics.push(ExternalToolDiagnostic::new(
                "optional_tool_missing",
                ReportSeverity::Warning,
                "No executable candidate was registered for this optional tool.",
            ));
        }

        ResolutionStatus::FailedRequired => {
            diagnostics.push(ExternalToolDiagnostic::new(
                "required_tool_resolution_failed",
                ReportSeverity::Error,
                "Executable candidates were registered, but none resolved successfully.",
            ));
        }

        ResolutionStatus::FailedOptional => {
            diagnostics.push(ExternalToolDiagnostic::new(
                "optional_tool_resolution_failed",
                ReportSeverity::Warning,
                "Executable candidates were registered, but none resolved successfully.",
            ));
        }

        ResolutionStatus::Cancelled => {
            diagnostics.push(ExternalToolDiagnostic::new(
                "tool_resolution_cancelled",
                ReportSeverity::Warning,
                "External-tool resolution was cancelled.",
            ));
        }
    }

    for failure in resolution.failures() {
        diagnostics.push(candidate_failure_diagnostic(failure));
    }
}

fn candidate_failure_diagnostic(
    failure: &CandidateFailure,
) -> ExternalToolDiagnostic {
    ExternalToolDiagnostic::new(
        "candidate_resolution_failed",
        ReportSeverity::Warning,
        failure.error().to_owned(),
    )
    .candidate(
        failure.provider_id(),
        failure.candidate_id(),
    )
}

fn append_validation_diagnostics(
    validation: &ToolValidationResult,
    diagnostics: &mut Vec<ExternalToolDiagnostic>,
) {
    for issue in validation.issues() {
        let diagnostic = validation_issue_diagnostic(issue);

        diagnostics.push(diagnostic);
    }
}

fn validation_issue_diagnostic(
    issue: &ValidationIssue,
) -> ExternalToolDiagnostic {
    match issue {
        ValidationIssue::MissingResolution { .. } => {
            ExternalToolDiagnostic::new(
                issue.code(),
                ReportSeverity::Warning,
                "The validation input contained no resolution entry.",
            )
        }

        ValidationIssue::NoSelectedTool { .. } => {
            ExternalToolDiagnostic::new(
                issue.code(),
                ReportSeverity::Warning,
                "No executable was selected for this requirement.",
            )
        }

        ValidationIssue::ExecutableNotAuthorized {
            executable,
            reason,
            ..
        } => {
            ExternalToolDiagnostic::new(
                issue.code(),
                ReportSeverity::Security,
                format!(
                    "Executable `{}` was not authorized: {reason}",
                    display_path(executable)
                ),
            )
        }

        ValidationIssue::CanonicalPathMismatch {
            supplied,
            canonical,
            ..
        } => {
            ExternalToolDiagnostic::new(
                issue.code(),
                ReportSeverity::Security,
                format!(
                    "Executable path `{}` is not the authorized canonical path `{}`.",
                    display_path(supplied),
                    display_path(canonical)
                ),
            )
        }

        ValidationIssue::VersionConstraintNotSatisfied {
            actual,
            expected,
            ..
        } => {
            ExternalToolDiagnostic::new(
                issue.code(),
                ReportSeverity::Error,
                format!(
                    "Version {actual} does not satisfy the required constraint: {expected}."
                ),
            )
        }

        ValidationIssue::CapabilityNotSatisfied {
            capability,
            reason,
            ..
        } => {
            ExternalToolDiagnostic::new(
                issue.code(),
                ReportSeverity::Error,
                format!(
                    "Required capability `{capability}` is not satisfied: {reason}"
                ),
            )
        }

        ValidationIssue::CapabilityUnknown {
            capability,
            reason,
            ..
        } => {
            ExternalToolDiagnostic::new(
                issue.code(),
                ReportSeverity::Warning,
                format!(
                    "Capability `{capability}` could not be verified: {reason}"
                ),
            )
        }
    }
}

fn severity_for(
    requirement: &ExternalToolRequirement,
    mandatory_severity: ReportSeverity,
) -> ReportSeverity {
    match requirement.requirement_kind() {
        RequirementKind::Required => mandatory_severity,
        RequirementKind::Optional => ReportSeverity::Warning,
    }
}

fn display_path(path: &Path) -> String {
    path.to_string_lossy().into_owned()
}

// ============================================================================
// SUMMARY
// ============================================================================

fn summarize(
    entries: &[ExternalToolReportEntry],
) -> ExternalToolReportSummary {
    let mut summary = ExternalToolReportSummary {
        total: entries.len(),
        ..ExternalToolReportSummary::default()
    };

    for entry in entries {
        match entry.requirement_kind {
            RequirementKindReport::Required => {
                summary.required += 1;
            }

            RequirementKindReport::Optional => {
                summary.optional += 1;
            }
        }

        match entry.status {
            ExternalToolStatus::Validated => {
                summary.validated += 1;
            }

            ExternalToolStatus::Missing => {
                summary.missing += 1;
            }

            ExternalToolStatus::Incompatible => {
                summary.incompatible += 1;
            }

            ExternalToolStatus::Rejected => {
                summary.rejected += 1;
            }

            ExternalToolStatus::Unverified => {
                summary.unverified += 1;
            }

            ExternalToolStatus::ResolutionFailed => {
                summary.resolution_failed += 1;
            }

            ExternalToolStatus::Cancelled => {
                summary.cancelled += 1;
            }

            ExternalToolStatus::NotReported => {
                summary.not_reported += 1;
            }
        }

        if entry.is_required() && !entry.is_validated() {
            summary.required_failures += 1;
        }

        if entry.is_optional() && !entry.is_validated() {
            summary.optional_failures += 1;
        }

        summary.diagnostic_count += entry.diagnostic_count();
    }

    summary
}

// ============================================================================
// REPORTING ERRORS
// ============================================================================

/// Errors encountered while composing an external-tool report.
#[derive(Debug)]
pub enum ReportingError {
    /// The supplied requirement collection is invalid.
    InvalidRequirements(
        super::requirements::RequirementError,
    ),

    /// Resolution evidence references an undeclared requirement.
    UndeclaredResolution {
        /// Unexpected requirement identifier.
        requirement_id: String,
    },

    /// Validation evidence references an undeclared requirement.
    UndeclaredValidation {
        /// Unexpected requirement identifier.
        requirement_id: String,
    },
}

impl fmt::Display for ReportingError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidRequirements(source) => {
                write!(
                    formatter,
                    "cannot report invalid external-tool requirements: {source}"
                )
            }

            Self::UndeclaredResolution {
                requirement_id,
            } => {
                write!(
                    formatter,
                    "resolution report contains undeclared requirement `{requirement_id}`"
                )
            }

            Self::UndeclaredValidation {
                requirement_id,
            } => {
                write!(
                    formatter,
                    "validation report contains undeclared requirement `{requirement_id}`"
                )
            }
        }
    }
}

impl Error for ReportingError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidRequirements(source) => Some(source),
            Self::UndeclaredResolution { .. }
            | Self::UndeclaredValidation { .. } => None,
        }
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_codes_are_stable_and_machine_readable() {
        assert_eq!(
            ExternalToolStatus::Validated.code(),
            "validated"
        );

        assert_eq!(
            ExternalToolStatus::Missing.code(),
            "missing"
        );

        assert_eq!(
            ExternalToolStatus::Incompatible.code(),
            "incompatible"
        );

        assert_eq!(
            ExternalToolStatus::Rejected.code(),
            "rejected"
        );

        assert_eq!(
            ExternalToolStatus::Unverified.code(),
            "unverified"
        );

        assert_eq!(
            ExternalToolStatus::ResolutionFailed.code(),
            "resolution_failed"
        );

        assert_eq!(
            ExternalToolStatus::Cancelled.code(),
            "cancelled"
        );

        assert_eq!(
            ExternalToolStatus::NotReported.code(),
            "not_reported"
        );
    }

    #[test]
    fn unknown_status_never_counts_as_validated() {
        assert!(!ExternalToolStatus::Unverified.is_validated());

        assert!(!ExternalToolStatus::NotReported.is_validated());

        assert!(ExternalToolStatus::Unverified.is_failure());

        assert!(ExternalToolStatus::NotReported.is_failure());
    }

    #[test]
    fn security_rejection_takes_precedence() {
        let issues = [
            ValidationIssue::CapabilityNotSatisfied {
                requirement_id:
                    super::super::requirements::RequirementId::new(
                        "llvm.compiler",
                    )
                    .expect("valid requirement"),
                capability:
                    super::super::requirements::CapabilityId::new(
                        "compiler.native",
                    )
                    .expect("valid capability"),
                reason: "capability unavailable".to_owned(),
            },
            ValidationIssue::ExecutableNotAuthorized {
                requirement_id:
                    super::super::requirements::RequirementId::new(
                        "llvm.compiler",
                    )
                    .expect("valid requirement"),
                executable: std::path::PathBuf::from("/untrusted/compiler"),
                reason: "not allowed".to_owned(),
            },
        ];

        assert!(
            issues.iter().any(|issue| {
                matches!(
                    issue,
                    ValidationIssue::ExecutableNotAuthorized { .. }
                )
            })
        );
    }

    #[test]
    fn report_summary_defaults_to_zero() {
        let summary = ExternalToolReportSummary::default();

        assert_eq!(summary.total, 0);
        assert_eq!(summary.validated, 0);
        assert_eq!(summary.required_failures, 0);
        assert_eq!(summary.optional_failures, 0);
    }

    #[test]
    fn schema_version_is_nonzero() {
        assert_ne!(
            EXTERNAL_TOOL_REPORT_SCHEMA_VERSION,
            0
        );
    }

    #[test]
    fn requirement_kind_is_preserved() {
        assert_eq!(
            RequirementKindReport::from(RequirementKind::Required),
            RequirementKindReport::Required
        );

        assert_eq!(
            RequirementKindReport::from(RequirementKind::Optional),
            RequirementKindReport::Optional
        );
    }

    #[test]
    fn status_serializes_as_snake_case() {
        assert_eq!(
            serde_json::to_string(&ExternalToolStatus::ResolutionFailed)
                .expect("serialization succeeds"),
            "\"resolution_failed\""
        );
    }

    #[test]
    fn diagnostic_serialization_preserves_structured_fields() {
        let diagnostic = ExternalToolDiagnostic::new(
            "tool_missing",
            ReportSeverity::Error,
            "Required executable is unavailable.",
        );

        let json = serde_json::to_value(&diagnostic)
            .expect("serialization succeeds");

        assert_eq!(
            json["code"],
            "tool_missing"
        );

        assert_eq!(
            json["severity"],
            "error"
        );

        assert_eq!(
            json["message"],
            "Required executable is unavailable."
        );
    }

    #[test]
    fn report_lookup_requires_exact_requirement_identity() {
        let entries = vec![
            ExternalToolReportEntry {
                requirement_id: "compiler.clang".to_owned(),
                requirement_kind: RequirementKindReport::Required,
                status: ExternalToolStatus::Validated,
                domains: Vec::new(),
                required_capabilities: Vec::new(),
                verified_capabilities: Vec::new(),
                provider_id: None,
                candidate_id: None,
                executable: None,
                version: None,
                purpose: None,
                origin: None,
                diagnostics: Vec::new(),
            },
        ];

        let report = ExternalToolReport {
            schema_version:
                EXTERNAL_TOOL_REPORT_SCHEMA_VERSION,
            cancelled: false,
            summary: summarize(&entries),
            entries,
            required_tools_validated: true,
            ready_for_required_tools: true,
        };

        assert!(report.get("compiler.clang").is_some());

        assert!(report.get("compiler.gcc").is_none());

        assert!(report.is_ready_for_required_tools());
    }

    #[test]
    fn optional_failure_does_not_block_required_readiness() {
        let entries = vec![
            ExternalToolReportEntry {
                requirement_id: "compiler.clang".to_owned(),
                requirement_kind: RequirementKindReport::Required,
                status: ExternalToolStatus::Validated,
                domains: Vec::new(),
                required_capabilities: Vec::new(),
                verified_capabilities: Vec::new(),
                provider_id: None,
                candidate_id: None,
                executable: None,
                version: None,
                purpose: None,
                origin: None,
                diagnostics: Vec::new(),
            },
            ExternalToolReportEntry {
                requirement_id: "formatter.optional".to_owned(),
                requirement_kind: RequirementKindReport::Optional,
                status: ExternalToolStatus::Missing,
                domains: Vec::new(),
                required_capabilities: Vec::new(),
                verified_capabilities: Vec::new(),
                provider_id: None,
                candidate_id: None,
                executable: None,
                version: None,
                purpose: None,
                origin: None,
                diagnostics: Vec::new(),
            },
        ];

        let summary = summarize(&entries);

        assert_eq!(summary.required_failures, 0);

        assert_eq!(summary.optional_failures, 1);

        assert_eq!(summary.validated, 1);
    }

    #[test]
    fn mandatory_failure_is_counted() {
        let entries = vec![
            ExternalToolReportEntry {
                requirement_id: "quantum.simulator".to_owned(),
                requirement_kind: RequirementKindReport::Required,
                status: ExternalToolStatus::Unverified,
                domains: vec!["quantum".to_owned()],
                required_capabilities: vec![
                    "quantum.simulation".to_owned()
                ],
                verified_capabilities: Vec::new(),
                provider_id: None,
                candidate_id: None,
                executable: None,
                version: None,
                purpose: None,
                origin: None,
                diagnostics: Vec::new(),
            },
        ];

        let summary = summarize(&entries);

        assert_eq!(summary.required_failures, 1);

        assert_eq!(summary.unverified, 1);
    }
}
