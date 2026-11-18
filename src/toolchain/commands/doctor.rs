
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani Toolchain Doctor
//!
//! Path: src/toolchain/commands/doctor.rs
//!
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: unsafe Rust is forbidden.
//!
//! ## Purpose
//!
//! Provides read-only diagnostics for Zamani projects, workspaces,
//! manifests, build infrastructure, dependencies, external tools,
//! targets, providers, security policies and execution environments.
//!
//! ## Architectural ownership
//!
//! This module owns:
//! - Doctor requests and configuration.
//! - Structured diagnostic outcomes.
//! - Diagnostic aggregation.
//! - Read-only project and manifest checks.
//! - Extensible diagnostic provider contracts.
//! - Deterministic report generation.
//!
//! It does not own:
//! - Source parsing or semantic analysis.
//! - Compiler invocation.
//! - Dependency resolution.
//! - External process execution.
//! - Backend or target registration.
//! - Hardware discovery.
//! - Runtime scheduling.
//! - Physical resource allocation.
//! - Deployment execution.
//!
//! Those responsibilities remain with their canonical repository
//! authorities.
//!
//! ## Integration
//!
//! - `zamani_project_config` owns Zamani.toml parsing.
//! - `backend::toolchain` owns external tool discovery.
//! - `backend::process` owns external process execution.
//! - `backend::registry` owns backend registration.
//! - `compiler` owns compilation.
//! - `zhal` owns hardware integration.
//! - `zur` owns runtime execution.
//! - `distributed` owns distributed-computation infrastructure.
//!
//! Domain-specific readiness is supplied through `DoctorCheck`
//! implementations registered by the appropriate subsystem.
//!
//! ## Scalability
//!
//! There are no fixed limits on:
//! - Number of checks.
//! - Number of providers.
//! - Number of computing domains.
//! - Number of targets.
//! - Number of devices.
//! - Number of workspace members.
//! - Number of dependencies.
//! - Number of distributed participants.
//!
//! Operational resource consumption remains subject to the caller's
//! configured execution environment and policies.
//!
//! ## Safety and side effects
//!
//! The built-in checks perform read-only filesystem inspection.
//! They do not execute external commands, modify project files,
//! access networks, or acquire hardware resources.
//!
//! Provider implementations must document their side effects.
//! A provider that performs active probing must only do so when
//! explicitly authorized by its owning subsystem.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Duration, SystemTime};

use serde::{Deserialize, Serialize};

use crate::zamani_project_config::ZamaniToml;

// ============================================================================
// VERSIONED DIAGNOSTIC CONTRACT
// ============================================================================

/// Version of the Doctor report contract.
///
/// This is independent of the Zamani language version, grammar version,
/// compiler version, backend API version and package version.
pub const DOCTOR_API_VERSION: u16 = 1;

// ============================================================================
// DIAGNOSTIC IDENTITIES
// ============================================================================

/// Stable identifier for one diagnostic check.
///
/// IDs are open-world identifiers. New providers may introduce additional
/// identifiers without modifying this module.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[serde(transparent)]
pub struct CheckId(String);

impl CheckId {
    /// Construct a validated check identifier.
    pub fn new(value: impl Into<String>) -> Result<Self, DoctorError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(DoctorError::InvalidCheckId(
                "check identifier must not be empty".to_owned(),
            ));
        }

        if value.chars().any(char::is_control) {
            return Err(DoctorError::InvalidCheckId(
                "check identifier must not contain control characters"
                    .to_owned(),
            ));
        }

        Ok(Self(value))
    }

    /// Return the canonical identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for CheckId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

// ============================================================================
// DIAGNOSTIC CATEGORIES
// ============================================================================

/// Broad diagnostic classification.
///
/// Provider-defined checks use the appropriate existing category.
/// `Provider` permits future domains without requiring modifications
/// to this enumeration.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticCategory {
    Project,
    Workspace,
    Manifest,
    Grammar,
    Environment,
    Configuration,
    Dependencies,
    Build,
    ExternalTools,
    Targets,
    Capabilities,
    Backend,
    Hardware,
    Runtime,
    Distributed,
    Security,
    Deployment,
    Provider,
}

// ============================================================================
// DIAGNOSTIC STATUS
// ============================================================================

/// Result established by an individual diagnostic check.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum CheckStatus {
    /// The checked requirement was established as satisfied.
    Passed,

    /// A nonfatal issue was established.
    Warning,

    /// A required condition was verified to be invalid.
    Failed,

    /// Available evidence is insufficient to establish readiness.
    Unresolved,

    /// A required capability or constraint was verified as unsatisfiable.
    Infeasible,

    /// The check was intentionally not performed.
    Skipped,
}

impl CheckStatus {
    /// Whether this outcome establishes a blocking condition.
    #[must_use]
    pub const fn is_blocking(self) -> bool {
        matches!(self, Self::Failed | Self::Infeasible)
    }

    /// Whether readiness remains uncertain.
    #[must_use]
    pub const fn is_unresolved(self) -> bool {
        matches!(self, Self::Unresolved)
    }
}

// ============================================================================
// DIAGNOSTIC SEVERITY
// ============================================================================

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

// ============================================================================
// INDIVIDUAL DIAGNOSTIC
// ============================================================================

/// One structured diagnostic finding.
///
/// `code` is intended for stable machine interpretation.
/// `message` is intended for human interpretation.
///
/// Messages must not contain passwords, access tokens, private keys,
/// authentication headers or other secrets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DoctorDiagnostic {
    pub id: CheckId,
    pub category: DiagnosticCategory,
    pub status: CheckStatus,
    pub severity: DiagnosticSeverity,
    pub message: String,

    #[serde(default)]
    pub details: Vec<String>,

    #[serde(default)]
    pub recommendations: Vec<String>,

    /// Optional reference to the authority responsible for resolving
    /// this finding.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,

    /// Optional path relative to the project root.
    ///
    /// Absolute paths are deliberately not generated by built-in checks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relative_path: Option<PathBuf>,

    /// Duration of the individual check.
    ///
    /// Duration is observational metadata, not a semantic property.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<Duration>,
}

impl DoctorDiagnostic {
    /// Create a diagnostic with an explicit status.
    pub fn new(
        id: CheckId,
        category: DiagnosticCategory,
        status: CheckStatus,
        severity: DiagnosticSeverity,
        message: impl Into<String>,
    ) -> Self {
        Self {
            id,
            category,
            status,
            severity,
            message: message.into(),
            details: Vec::new(),
            recommendations: Vec::new(),
            owner: None,
            relative_path: None,
            duration: None,
        }
    }

    /// Add explanatory detail.
    #[must_use]
    pub fn with_detail(mut self, detail: impl Into<String>) -> Self {
        self.details.push(detail.into());
        self
    }

    /// Add a recommended corrective action.
    #[must_use]
    pub fn with_recommendation(
        mut self,
        recommendation: impl Into<String>,
    ) -> Self {
        self.recommendations.push(recommendation.into());
        self
    }

    /// Assign the subsystem responsible for this diagnostic.
    #[must_use]
    pub fn with_owner(mut self, owner: impl Into<String>) -> Self {
        self.owner = Some(owner.into());
        self
    }

    /// Attach a project-relative path.
    #[must_use]
    pub fn with_relative_path(mut self, path: PathBuf) -> Self {
        self.relative_path = Some(path);
        self
    }

    fn with_duration(mut self, duration: Duration) -> Self {
        self.duration = Some(duration);
        self
    }
}

// ============================================================================
// REQUEST CONFIGURATION
// ============================================================================

/// Requested Doctor operation.
#[derive(Debug, Clone)]
pub struct DoctorRequest {
    project_root: PathBuf,
    manifest_path: PathBuf,
    inspect_grammar: bool,
    inspect_environment: bool,
    inspect_providers: bool,
}

impl DoctorRequest {
    /// Construct a request for a project root.
    ///
    /// The manifest path defaults to the canonical project manifest name.
    /// The caller may supply another relative or absolute manifest path.
    #[must_use]
    pub fn new(project_root: impl Into<PathBuf>) -> Self {
        Self {
            project_root: project_root.into(),
            manifest_path: PathBuf::from("Zamani.toml"),
            inspect_grammar: true,
            inspect_environment: true,
            inspect_providers: true,
        }
    }

    /// Select a different manifest path.
    #[must_use]
    pub fn manifest_path(mut self, path: impl Into<PathBuf>) -> Self {
        self.manifest_path = path.into();
        self
    }

    /// Enable or disable grammar-contract inspection.
    #[must_use]
    pub fn inspect_grammar(mut self, enabled: bool) -> Self {
        self.inspect_grammar = enabled;
        self
    }

    /// Enable or disable host-environment inspection.
    #[must_use]
    pub fn inspect_environment(mut self, enabled: bool) -> Self {
        self.inspect_environment = enabled;
        self
    }

    /// Enable or disable registered provider checks.
    #[must_use]
    pub fn inspect_providers(mut self, enabled: bool) -> Self {
        self.inspect_providers = enabled;
        self
    }

    #[must_use]
    pub fn project_root(&self) -> &Path {
        &self.project_root
    }

    #[must_use]
    pub fn selected_manifest(&self) -> &Path {
        &self.manifest_path
    }
}

// ============================================================================
// CHECK CONTEXT
// ============================================================================

/// Immutable context shared with diagnostic providers.
///
/// Providers receive the canonicalized project root and explicit request.
/// They do not receive implicit authority to modify the project.
#[derive(Debug, Clone)]
pub struct DoctorContext {
    project_root: PathBuf,
    manifest_path: PathBuf,
    request: DoctorRequest,
}

impl DoctorContext {
    #[must_use]
    pub fn project_root(&self) -> &Path {
        &self.project_root
    }

    #[must_use]
    pub fn manifest_path(&self) -> &Path {
        &self.manifest_path
    }

    #[must_use]
    pub fn request(&self) -> &DoctorRequest {
        &self.request
    }
}

// ============================================================================
// PROVIDER CONTRACT
// ============================================================================

/// Extensible diagnostic provider contract.
///
/// Each provider owns the correctness of its own diagnostic checks.
///
/// Implementations must:
///
/// - Use stable diagnostic identifiers.
/// - Return structured outcomes.
/// - Preserve unresolved and infeasible distinctions.
/// - Avoid claiming capabilities without evidence.
/// - Avoid modifying project state.
/// - Avoid exposing secrets in diagnostic messages.
/// - Document any active probing or external side effects.
/// - Respect applicable caller authorization and budgets.
///
/// Implementations must be thread-safe because provider registrations
/// may be shared between toolchain clients.
///
/// `Send + Sync` does not authorize concurrent invocation by itself.
/// The Doctor currently invokes checks sequentially.
pub trait DoctorCheck: Send + Sync {
    /// Stable provider identifier.
    fn provider_id(&self) -> &str;

    /// Execute this provider's read-only diagnostic checks.
    fn diagnose(
        &self,
        context: &DoctorContext,
    ) -> Vec<DoctorDiagnostic>;
}

// ============================================================================
// PROVIDER REGISTRY
// ============================================================================

/// Collection of registered diagnostic providers.
///
/// Providers are stored in deterministic registration order, with
/// duplicate provider identifiers rejected.
///
/// This registry is local to Doctor diagnostics. It is not a replacement
/// for the canonical backend, hardware or runtime registries.
#[derive(Clone, Default)]
pub struct DoctorRegistry {
    providers: Vec<Arc<dyn DoctorCheck>>,
    identifiers: BTreeSet<String>,
}

impl fmt::Debug for DoctorRegistry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DoctorRegistry")
            .field("provider_count", &self.providers.len())
            .field("identifiers", &self.identifiers)
            .finish()
    }
}

impl DoctorRegistry {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a diagnostic provider.
    pub fn register(
        &mut self,
        provider: Arc<dyn DoctorCheck>,
    ) -> Result<(), DoctorError> {
        let identifier = provider.provider_id().trim();

        if identifier.is_empty() {
            return Err(DoctorError::InvalidProviderId(
                "provider identifier must not be empty".to_owned(),
            ));
        }

        if identifier.chars().any(char::is_control) {
            return Err(DoctorError::InvalidProviderId(
                "provider identifier contains control characters".to_owned(),
            ));
        }

        if !self.identifiers.insert(identifier.to_owned()) {
            return Err(DoctorError::DuplicateProvider(
                identifier.to_owned(),
            ));
        }

        self.providers.push(provider);

        Ok(())
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.providers.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.providers.is_empty()
    }
}

// ============================================================================
// REPORT
// ============================================================================

/// Aggregate readiness summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DoctorSummary {
    pub total: usize,
    pub passed: usize,
    pub warnings: usize,
    pub failed: usize,
    pub unresolved: usize,
    pub infeasible: usize,
    pub skipped: usize,
}

impl DoctorSummary {
    fn from_diagnostics(diagnostics: &[DoctorDiagnostic]) -> Self {
        let mut summary = Self {
            total: diagnostics.len(),
            passed: 0,
            warnings: 0,
            failed: 0,
            unresolved: 0,
            infeasible: 0,
            skipped: 0,
        };

        for diagnostic in diagnostics {
            match diagnostic.status {
                CheckStatus::Passed => summary.passed += 1,
                CheckStatus::Warning => summary.warnings += 1,
                CheckStatus::Failed => summary.failed += 1,
                CheckStatus::Unresolved => summary.unresolved += 1,
                CheckStatus::Infeasible => summary.infeasible += 1,
                CheckStatus::Skipped => summary.skipped += 1,
            }
        }

        summary
    }
}

/// Overall diagnostic readiness.
///
/// `Ready` means that every performed check passed and no check remains
/// unresolved. It does not certify unexamined hardware or providers.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum Readiness {
    Ready,
    ReadyWithWarnings,
    Blocked,
    Undetermined,
}

impl Readiness {
    fn from_summary(summary: &DoctorSummary) -> Self {
        if summary.failed > 0 || summary.infeasible > 0 {
            Self::Blocked
        } else if summary.unresolved > 0 {
            Self::Undetermined
        } else if summary.warnings > 0 || summary.skipped > 0 {
            Self::ReadyWithWarnings
        } else {
            Self::Ready
        }
    }
}

/// Complete Doctor response.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DoctorReport {
    pub api_version: u16,
    pub readiness: Readiness,
    pub summary: DoctorSummary,
    pub diagnostics: Vec<DoctorDiagnostic>,
}

impl DoctorReport {
    fn from_diagnostics(
        mut diagnostics: Vec<DoctorDiagnostic>,
    ) -> Self {
        // Deterministic output independent of provider execution order.
        diagnostics.sort_by(|left, right| {
            (
                left.category,
                &left.id,
                left.status,
            )
                .cmp(&(
                    right.category,
                    &right.id,
                    right.status,
                ))
        });

        let summary = DoctorSummary::from_diagnostics(&diagnostics);
        let readiness = Readiness::from_summary(&summary);

        Self {
            api_version: DOCTOR_API_VERSION,
            readiness,
            summary,
            diagnostics,
        }
    }

    /// Return true if no blocking conditions were established.
    #[must_use]
    pub fn has_blocking_findings(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|diagnostic| diagnostic.status.is_blocking())
    }

    /// Return true if any check could not establish readiness.
    #[must_use]
    pub fn has_unresolved_findings(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|diagnostic| diagnostic.status.is_unresolved())
    }

    /// Serialize the report to JSON.
    pub fn to_json(&self) -> Result<String, DoctorError> {
        serde_json::to_string_pretty(self)
            .map_err(DoctorError::Serialization)
    }
}

// ============================================================================
// ERRORS
// ============================================================================

#[derive(Debug)]
pub enum DoctorError {
    InvalidCheckId(String),
    InvalidProviderId(String),
    DuplicateProvider(String),
    InvalidProjectRoot(PathBuf),
    ProjectRootIo {
        path: PathBuf,
        message: String,
    },
    InvalidManifestPath(PathBuf),
    Serialization(serde_json::Error),
}

impl fmt::Display for DoctorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidCheckId(message) => {
                write!(f, "invalid diagnostic check identifier: {message}")
            }
            Self::InvalidProviderId(message) => {
                write!(f, "invalid diagnostic provider identifier: {message}")
            }
            Self::DuplicateProvider(identifier) => {
                write!(f, "diagnostic provider `{identifier}` is already registered")
            }
            Self::InvalidProjectRoot(path) => {
                write!(f, "project root is not a directory: {}", path.display())
            }
            Self::ProjectRootIo { path, message } => {
                write!(
                    f,
                    "cannot inspect project root `{}`: {message}",
                    path.display()
                )
            }
            Self::InvalidManifestPath(path) => {
                write!(
                    f,
                    "manifest path must identify a file: {}",
                    path.display()
                )
            }
            Self::Serialization(error) => {
                write!(f, "cannot serialize Doctor report: {error}")
            }
        }
    }
}

impl Error for DoctorError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Serialization(error) => Some(error),
            _ => None,
        }
    }
}

// ============================================================================
// DOCTOR SERVICE
// ============================================================================

/// Read-only project diagnostic service.
#[derive(Debug, Clone, Default)]
pub struct Doctor;

impl Doctor {
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Inspect a project using built-in checks and registered providers.
    ///
    /// This operation does not compile, execute, install, publish,
    /// deploy, allocate hardware or modify the project.
    pub fn inspect(
        &self,
        request: DoctorRequest,
        registry: &DoctorRegistry,
    ) -> Result<DoctorReport, DoctorError> {
        let root = fs::canonicalize(request.project_root())
            .map_err(|error| DoctorError::ProjectRootIo {
                path: request.project_root().to_path_buf(),
                message: error.to_string(),
            })?;

        if !root.is_dir() {
            return Err(DoctorError::InvalidProjectRoot(root));
        }

        let manifest_path =
            resolve_manifest_path(&root, request.selected_manifest())?;

        let context = DoctorContext {
            project_root: root.clone(),
            manifest_path,
            request: request.clone(),
        };

        let mut diagnostics = Vec::new();

        diagnostics.push(check_project_root(&context));

        diagnostics.push(check_manifest(&context));

        if request.inspect_grammar {
            diagnostics.extend(check_grammar_contracts(&context));
        }

        if request.inspect_environment {
            diagnostics.push(check_environment(&context));
        }

        if request.inspect_providers {
            for provider in &registry.providers {
                let provider_diagnostics = provider.diagnose(&context);

                for diagnostic in provider_diagnostics {
                    diagnostics.push(diagnostic);
                }
            }
        }

        Ok(DoctorReport::from_diagnostics(diagnostics))
    }
}

// ============================================================================
// MANIFEST RESOLUTION
// ============================================================================

fn resolve_manifest_path(
    root: &Path,
    requested: &Path,
) -> Result<PathBuf, DoctorError> {
    let candidate = if requested.is_absolute() {
        requested.to_path_buf()
    } else {
        root.join(requested)
    };

    if !candidate.is_file() {
        return Ok(candidate);
    }

    let canonical = fs::canonicalize(&candidate)
        .map_err(|error| DoctorError::ProjectRootIo {
            path: candidate.clone(),
            message: error.to_string(),
        })?;

    // Prevent a caller-supplied relative manifest path or symlink from
    // silently escaping the selected project root.
    if !canonical.starts_with(root) {
        return Err(DoctorError::InvalidManifestPath(canonical));
    }

    Ok(canonical)
}

// ============================================================================
// BUILT-IN PROJECT CHECKS
// ============================================================================

fn check_project_root(context: &DoctorContext) -> DoctorDiagnostic {
    let result = CheckId::new("project.root")
        .expect("static diagnostic identifier is valid");

    let root = context.project_root();

    let status = if root.is_dir() {
        CheckStatus::Passed
    } else {
        CheckStatus::Failed
    };

    DoctorDiagnostic::new(
        result,
        DiagnosticCategory::Project,
        status,
        if status == CheckStatus::Passed {
            DiagnosticSeverity::Info
        } else {
            DiagnosticSeverity::Error
        },
        if status == CheckStatus::Passed {
            "Project root is accessible"
        } else {
            "Project root is unavailable"
        },
    )
    .with_owner("toolchain::workspace")
}

fn check_manifest(context: &DoctorContext) -> DoctorDiagnostic {
    let id = CheckId::new("manifest.parse")
        .expect("static diagnostic identifier is valid");

    let relative_path = context
        .manifest_path()
        .strip_prefix(context.project_root())
        .unwrap_or(context.manifest_path())
        .to_path_buf();

    let started = std::time::Instant::now();

    let diagnostic = if !context.manifest_path().is_file() {
        DoctorDiagnostic::new(
            id,
            DiagnosticCategory::Manifest,
            CheckStatus::Failed,
            DiagnosticSeverity::Error,
            "Project manifest is missing",
        )
        .with_detail("The selected manifest file could not be found.")
        .with_recommendation(
            "Create a valid Zamani.toml or select an existing manifest.",
        )
        .with_owner("zamani_project_config")
    } else {
        match fs::read_to_string(context.manifest_path()) {
            Err(error) => DoctorDiagnostic::new(
                id,
                DiagnosticCategory::Manifest,
                CheckStatus::Failed,
                DiagnosticSeverity::Error,
                "Project manifest cannot be read",
            )
            .with_detail(error.to_string())
            .with_recommendation(
                "Check the manifest file's encoding and read permissions.",
            )
            .with_owner("zamani_project_config"),

            Ok(contents) => {
                match ZamaniToml::parse_from_str(&contents) {
                    Err(error) => DoctorDiagnostic::new(
                        id,
                        DiagnosticCategory::Manifest,
                        CheckStatus::Failed,
                        DiagnosticSeverity::Error,
                        "Project manifest could not be parsed",
                    )
                    .with_detail(error)
                    .with_recommendation(
                        "Correct the TOML syntax and required package section.",
                    )
                    .with_owner("zamani_project_config"),

                    Ok(manifest) => {
                        let mut diagnostic = DoctorDiagnostic::new(
                            id,
                            DiagnosticCategory::Manifest,
                            CheckStatus::Passed,
                            DiagnosticSeverity::Info,
                            "Project manifest was parsed successfully",
                        )
                        .with_owner("zamani_project_config");

                        if manifest.package.name.trim().is_empty() {
                            diagnostic.status = CheckStatus::Warning;
                            diagnostic.severity = DiagnosticSeverity::Warning;
                            diagnostic.details.push(
                                "Package name is empty in the parsed manifest."
                                    .to_owned(),
                            );
                        }

                        if manifest.package.version.trim().is_empty() {
                            diagnostic.status = CheckStatus::Warning;
                            diagnostic.severity = DiagnosticSeverity::Warning;
                            diagnostic.details.push(
                                "Package version is empty in the parsed manifest."
                                    .to_owned(),
                            );
                        }

                        diagnostic
                    }
                }
            }
        }
    };

    diagnostic
        .with_relative_path(relative_path)
        .with_duration(started.elapsed())
}

// ============================================================================
// GRAMMAR CONTRACT CHECKS
// ============================================================================

/// Normative grammar and semantic documents that should be available
/// to the repository's language-development infrastructure.
///
/// These paths identify architecture authorities, not a closed list
/// of supported computing domains.
const GRAMMAR_CONTRACT_FILES: &[&str] = &[
    "grammar/DESIGN.md",
    "grammar/Zamani.g4",
    "grammar/antlr/ZamaniLexer.g4",
    "grammar/antlr/ZamaniParser.g4",
    "grammar/specification/semantic-model.md",
    "grammar/validation/semantic-boundaries.md",
];

fn check_grammar_contracts(
    context: &DoctorContext,
) -> Vec<DoctorDiagnostic> {
    let mut diagnostics = Vec::new();

    for relative in GRAMMAR_CONTRACT_FILES {
        let path = context.project_root().join(relative);

        let id = CheckId::new(format!("grammar.contract.{relative}"))
            .expect("static grammar check identifier is valid");

        let diagnostic = if path.is_file() {
            DoctorDiagnostic::new(
                id,
                DiagnosticCategory::Grammar,
                CheckStatus::Passed,
                DiagnosticSeverity::Info,
                format!("Grammar contract file is present: {relative}"),
            )
        } else {
            DoctorDiagnostic::new(
                id,
                DiagnosticCategory::Grammar,
                CheckStatus::Unresolved,
                DiagnosticSeverity::Warning,
                format!("Grammar contract file is unavailable: {relative}"),
            )
            .with_recommendation(
                "Restore the authoritative grammar or semantic specification \
                 before relying on grammar-dependent validation.",
            )
        };

        diagnostics.push(
            diagnostic
                .with_owner("grammar")
                .with_relative_path(PathBuf::from(relative)),
        );
    }

    diagnostics
}

// ============================================================================
// ENVIRONMENT CHECK
// ============================================================================

fn check_environment(context: &DoctorContext) -> DoctorDiagnostic {
    let id = CheckId::new("environment.host")
        .expect("static diagnostic identifier is valid");

    let current_dir = std::env::current_dir();

    let mut diagnostic = match current_dir {
        Ok(directory) => DoctorDiagnostic::new(
            id,
            DiagnosticCategory::Environment,
            CheckStatus::Passed,
            DiagnosticSeverity::Info,
            "Host environment is accessible",
        )
        .with_detail(format!(
            "Current working directory is accessible: {}",
            directory.display()
        )),

        Err(error) => DoctorDiagnostic::new(
            id,
            DiagnosticCategory::Environment,
            CheckStatus::Warning,
            DiagnosticSeverity::Warning,
            "Current working directory could not be inspected",
        )
        .with_detail(error.to_string()),
    };

    diagnostic.owner = Some("toolchain::environments".to_owned());

    // These are observational host facts, not target capabilities.
    diagnostic.details.push(format!(
        "Operating system: {}",
        std::env::consts::OS
    ));

    diagnostic.details.push(format!(
        "Host architecture: {}",
        std::env::consts::ARCH
    ));

    diagnostic.details.push(format!(
        "Project root: {}",
        context.project_root().display()
    ));

    diagnostic
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use std::sync::atomic::{AtomicU64, Ordering};

    static NEXT_TEST_ID: AtomicU64 = AtomicU64::new(1);

    struct TestProject {
        path: PathBuf,
    }

    impl TestProject {
        fn new() -> Self {
            let id = NEXT_TEST_ID.fetch_add(1, Ordering::Relaxed);

            let path = std::env::temp_dir().join(format!(
                "zamani-doctor-{}-{id}",
                std::process::id()
            ));

            fs::create_dir_all(&path).expect("create test project");

            Self { path }
        }

        fn write_manifest(&self, contents: &str) {
            fs::write(self.path.join("Zamani.toml"), contents)
                .expect("write test manifest");
        }

        fn root(&self) -> &Path {
            &self.path
        }
    }

    impl Drop for TestProject {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }

    fn valid_manifest() -> &'static str {
        r#"
[package]
name = "doctor-test"
version = "1.0.0"
edition = "2021"
authors = []
description = "Doctor test project"
type = "application"

[dependencies]
"#
    }

    struct StaticProvider {
        identifier: String,
        diagnostics: Vec<DoctorDiagnostic>,
    }

    impl DoctorCheck for StaticProvider {
        fn provider_id(&self) -> &str {
            &self.identifier
        }

        fn diagnose(
            &self,
            _context: &DoctorContext,
        ) -> Vec<DoctorDiagnostic> {
            self.diagnostics.clone()
        }
    }

    fn registry_with(
        diagnostics: Vec<DoctorDiagnostic>,
    ) -> DoctorRegistry {
        let mut registry = DoctorRegistry::new();

        registry
            .register(Arc::new(StaticProvider {
                identifier: "test.provider".to_owned(),
                diagnostics,
            }))
            .expect("register test provider");

        registry
    }

    #[test]
    fn doctor_api_version_is_nonzero() {
        assert_ne!(DOCTOR_API_VERSION, 0);
    }

    #[test]
    fn valid_project_manifest_is_recognized() {
        let project = TestProject::new();
        project.write_manifest(valid_manifest());

        let report = Doctor::new()
            .inspect(
                DoctorRequest::new(project.root())
                    .inspect_grammar(false)
                    .inspect_environment(false),
                &DoctorRegistry::new(),
            )
            .expect("doctor inspection");

        assert!(report.diagnostics.iter().any(|diagnostic| {
            diagnostic.id.as_str() == "manifest.parse"
                && diagnostic.status == CheckStatus::Passed
        }));
    }

    #[test]
    fn missing_manifest_is_reported_as_failure() {
        let project = TestProject::new();

        let report = Doctor::new()
            .inspect(
                DoctorRequest::new(project.root())
                    .inspect_grammar(false)
                    .inspect_environment(false),
                &DoctorRegistry::new(),
            )
            .expect("doctor inspection");

        assert!(report.diagnostics.iter().any(|diagnostic| {
            diagnostic.id.as_str() == "manifest.parse"
                && diagnostic.status == CheckStatus::Failed
        }));
    }

    #[test]
    fn malformed_manifest_is_not_reported_as_success() {
        let project = TestProject::new();
        project.write_manifest("[package\ninvalid");

        let report = Doctor::new()
            .inspect(
                DoctorRequest::new(project.root())
                    .inspect_grammar(false)
                    .inspect_environment(false),
                &DoctorRegistry::new(),
            )
            .expect("doctor inspection");

        assert_eq!(report.readiness, Readiness::Blocked);
    }

    #[test]
    fn unresolved_capability_does_not_become_available() {
        let project = TestProject::new();
        project.write_manifest(valid_manifest());

        let diagnostic = DoctorDiagnostic::new(
            CheckId::new("quantum.provider.readiness").expect("valid ID"),
            DiagnosticCategory::Capabilities,
            CheckStatus::Unresolved,
            DiagnosticSeverity::Warning,
            "Quantum capability has not been established",
        );

        let registry = registry_with(vec![diagnostic]);

        let report = Doctor::new()
            .inspect(
                DoctorRequest::new(project.root())
                    .inspect_grammar(false)
                    .inspect_environment(false),
                &registry,
            )
            .expect("doctor inspection");

        assert_eq!(report.readiness, Readiness::Undetermined);
        assert!(report.has_unresolved_findings());
    }

    #[test]
    fn infeasible_requirements_block_readiness() {
        let project = TestProject::new();
        project.write_manifest(valid_manifest());

        let diagnostic = DoctorDiagnostic::new(
            CheckId::new("target.feasibility").expect("valid ID"),
            DiagnosticCategory::Targets,
            CheckStatus::Infeasible,
            DiagnosticSeverity::Error,
            "Required target capability cannot be satisfied",
        );

        let report = Doctor::new()
            .inspect(
                DoctorRequest::new(project.root())
                    .inspect_grammar(false)
                    .inspect_environment(false),
                &registry_with(vec![diagnostic]),
            )
            .expect("doctor inspection");

        assert_eq!(report.readiness, Readiness::Blocked);
        assert!(report.has_blocking_findings());
    }

    #[test]
    fn duplicate_provider_registration_is_rejected() {
        let mut registry = DoctorRegistry::new();

        let first = Arc::new(StaticProvider {
            identifier: "provider.one".to_owned(),
            diagnostics: Vec::new(),
        });

        let second = Arc::new(StaticProvider {
            identifier: "provider.one".to_owned(),
            diagnostics: Vec::new(),
        });

        registry.register(first).expect("first registration");

        assert!(matches!(
            registry.register(second),
            Err(DoctorError::DuplicateProvider(_))
        ));

        assert_eq!(registry.len(), 1);
    }

    #[test]
    fn diagnostic_order_is_deterministic() {
        let first = DoctorDiagnostic::new(
            CheckId::new("z.check").expect("valid ID"),
            DiagnosticCategory::Project,
            CheckStatus::Passed,
            DiagnosticSeverity::Info,
            "Second",
        );

        let second = DoctorDiagnostic::new(
            CheckId::new("a.check").expect("valid ID"),
            DiagnosticCategory::Project,
            CheckStatus::Passed,
            DiagnosticSeverity::Info,
            "First",
        );

        let report = DoctorReport::from_diagnostics(vec![
            first,
            second,
        ]);

        assert_eq!(
            report.diagnostics[0].id.as_str(),
            "a.check"
        );

        assert_eq!(
            report.diagnostics[1].id.as_str(),
            "z.check"
        );
    }

    #[test]
    fn report_serializes_to_machine_readable_json() {
        let report = DoctorReport::from_diagnostics(Vec::new());

        let json = report.to_json().expect("serialize report");

        assert!(json.contains("api_version"));
        assert!(json.contains("readiness"));
        assert!(json.contains("summary"));
        assert!(json.contains("diagnostics"));
    }

    #[test]
    fn empty_report_is_ready_without_claiming_hardware_availability() {
        let report = DoctorReport::from_diagnostics(Vec::new());

        assert_eq!(report.readiness, Readiness::Ready);
        assert_eq!(report.summary.total, 0);
        assert_eq!(report.summary.passed, 0);
    }

    #[test]
    fn missing_grammar_contract_is_unresolved() {
        let project = TestProject::new();
        project.write_manifest(valid_manifest());

        let report = Doctor::new()
            .inspect(
                DoctorRequest::new(project.root())
                    .inspect_environment(false)
                    .inspect_providers(false),
                &DoctorRegistry::new(),
            )
            .expect("doctor inspection");

        assert!(report.diagnostics.iter().any(|diagnostic| {
            diagnostic.category == DiagnosticCategory::Grammar
                && diagnostic.status == CheckStatus::Unresolved
        }));
    }
}
