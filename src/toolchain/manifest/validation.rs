
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Toolchain — Manifest Validation
//!
//! Path: `src/toolchain/manifest/validation.rs`
//! Edition: Rust 2021.
//! Minimum Rust version: 1.97.
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Validates an already parsed canonical `ZamaniToml` before the toolchain
//! uses it for workspace, dependency, build, package, or other project
//! operations.
//!
//! The canonical schema and parser remain owned by
//! `crate::zamani_project_config`. This module neither parses TOML nor
//! defines a second manifest representation.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Structural validation of fields present in the current canonical
//!   manifest model.
//! - Stable validation issue codes and field paths.
//! - Validation severity and reports.
//! - Validation of relationships between current package and dependency
//!   declarations.
//!
//! This module does not own:
//!
//! - TOML parsing, serialization, or source-level TOML diagnostics.
//! - The canonical manifest schema.
//! - Manifest-version compatibility or migration.
//! - Dependency version-constraint interpretation.
//! - Dependency source resolution or filesystem access.
//! - Workspace declarations or workspace discovery.
//! - Target selection, capability checks, or feasibility.
//! - Compiler, backend, hardware, or runtime validation.
//! - Package archive validation.
//!
//! ## Canonical integration
//!
//! Input:
//!
//! - `crate::zamani_project_config::ZamaniToml`
//! - `crate::zamani_project_config::PackageSection`
//! - `crate::zamani_project_config::DependencyConfig`
//!
//! Downstream:
//!
//! - `manifest::adapter` converts validated declarations into toolchain
//!   dependency and project contracts.
//! - `manifest::loading` supplies parsed manifests.
//! - `manifest::compatibility` will own manifest-version compatibility
//!   when that module and canonical version field are implemented.
//! - `workspace` consumes workspace declarations only after the canonical
//!   manifest schema provides them.
//! - `dependencies::constraints` interprets version expressions.
//! - `dependencies::sources` resolves declared dependency paths and sources.
//! - `build` consumes validated project/build intent.
//!
//! ## Validation semantics
//!
//! A valid report means that the fields and relationships represented by
//! the current `ZamaniToml` value satisfy this module's structural rules.
//!
//! It does not certify that:
//!
//! - The original TOML used the correct type for every field.
//! - The parser preserved every source field.
//! - A version expression is supported by the dependency resolver.
//! - A dependency exists or is accessible.
//! - A local path is contained within an allowed project root.
//! - A workspace, target, provider, or execution environment is feasible.
//! - The project compiles, packages, or executes successfully.
//!
//! The current canonical parser is permissive and may discard or default
//! malformed source values. Strict TOML type checking and lossless source
//! diagnostics must be implemented by `src/zamani_project_config`.
//!
//! ## Determinism
//!
//! Validation traverses dependency names in sorted order even though the
//! canonical schema currently stores them in a `HashMap`.
//!
//! Issues are therefore emitted in deterministic order for the same
//! canonical manifest value.
//!
//! ## Scalability
//!
//! This module introduces no arbitrary maximum for:
//!
//! - Dependencies.
//! - Authors.
//! - Dependency features.
//! - Package metadata length.
//! - Path length.
//! - Computing domains.
//! - Targets or providers.
//! - Hardware resources.
//! - Distributed participants.
//!
//! Work and report storage are proportional to the fields and declarations
//! in the supplied manifest. Actual operation remains subject to available
//! memory, address-space limits, and caller-configured operational budgets.
//!
//! Literal infinite storage or computation is not promised.
//!
//! ## Security
//!
//! - No unsafe Rust.
//! - No filesystem access.
//! - No network access.
//! - No shell or process execution.
//! - No expression evaluation.
//! - No implicit dependency fetching.
//! - No path canonicalization.
//! - No credentials or secret handling.
//! - No silent correction or normalization of invalid declarations.
//!
//! A dependency path is checked only as a declaration string here.
//! Filesystem existence, symlink handling, root containment, and
//! authorization belong to dependency-source resolution.
//!
//! ## Compatibility
//!
//! Validation is based only on fields in the current canonical schema.
//! Adding a new canonical manifest field does not require this module to
//! invent or guess its meaning. The new field must receive an explicit
//! validation contract here only after its canonical owner and semantics
//! have been established.

use crate::toolchain::errors::ToolchainError;
use crate::toolchain::result::Result;
use crate::zamani_project_config::{
    DependencyConfig,
    PackageSection,
    ZamaniToml,
};

use std::collections::BTreeSet;
use std::path::Path;

// ============================================================================
// OPERATION AND ISSUE IDENTIFIERS
// ============================================================================

const VALIDATE_OPERATION: &str = "manifest.validation.validate";

/// Stable issue code for a missing or blank required value.
pub const ISSUE_REQUIRED_VALUE: &str = "ZTMV0001";

/// Stable issue code for a value containing control characters.
pub const ISSUE_CONTROL_CHARACTER: &str = "ZTMV0002";

/// Stable issue code for surrounding whitespace in an identifier or value.
pub const ISSUE_SURROUNDING_WHITESPACE: &str = "ZTMV0003";

/// Stable issue code for a dependency without a version or path source.
pub const ISSUE_DEPENDENCY_SOURCE_MISSING: &str = "ZTMV0004";

/// Stable issue code for a dependency feature declared more than once.
pub const ISSUE_DUPLICATE_FEATURE: &str = "ZTMV0005";

/// Stable issue code for an invalid dependency path declaration.
pub const ISSUE_INVALID_DEPENDENCY_PATH: &str = "ZTMV0006";

// ============================================================================
// ISSUE SEVERITY
// ============================================================================

/// Severity assigned to a manifest validation issue.
///
/// Severity is separate from the stable issue code. Consumers should use
/// the code for machine-readable handling and severity for presentation
/// or policy decisions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ManifestValidationSeverity {
    /// The declaration violates a required structural invariant.
    Error,

    /// The declaration is accepted but deserves attention.
    ///
    /// The current validator does not emit warnings because the current
    /// canonical schema does not provide enough normative information to
    /// classify any declaration as a warning without guessing.
    Warning,
}

impl ManifestValidationSeverity {
    /// Return the stable lowercase severity name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Error => "error",
            Self::Warning => "warning",
        }
    }
}

// ============================================================================
// VALIDATION ISSUE
// ============================================================================

/// One deterministic, machine-readable manifest validation issue.
///
/// `field` is a dotted canonical field path, optionally followed by an
/// array index or dependency key. It is not a source-code span because the
/// current canonical parser does not preserve TOML source locations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManifestValidationIssue {
    code: &'static str,
    severity: ManifestValidationSeverity,
    field: String,
    message: String,
}

impl ManifestValidationIssue {
    fn new(
        code: &'static str,
        severity: ManifestValidationSeverity,
        field: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            code,
            severity,
            field: field.into(),
            message: message.into(),
        }
    }

    /// Return the stable machine-readable issue code.
    #[must_use]
    pub const fn code(&self) -> &'static str {
        self.code
    }

    /// Return the issue severity.
    #[must_use]
    pub const fn severity(&self) -> ManifestValidationSeverity {
        self.severity
    }

    /// Return the canonical manifest field path.
    #[must_use]
    pub fn field(&self) -> &str {
        &self.field
    }

    /// Return the human-readable explanation.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Return a stable, concise diagnostic representation.
    #[must_use]
    pub fn diagnostic(&self) -> String {
        format!(
            "{} {} at {}: {}",
            self.severity.as_str(),
            self.code,
            self.field,
            self.message,
        )
    }
}

// ============================================================================
// VALIDATION REPORT
// ============================================================================

/// Complete result of validating one canonical manifest snapshot.
///
/// A report is valid when it contains no error-severity issues. Warnings,
/// if introduced by a future explicitly specified validation rule, do not
/// make a report invalid.
///
/// The report retains every issue found during the validation pass rather
/// than stopping at the first malformed field.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ManifestValidationReport {
    issues: Vec<ManifestValidationIssue>,
}

impl ManifestValidationReport {
    /// Construct an empty successful report.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Return all issues in deterministic validation order.
    #[must_use]
    pub fn issues(&self) -> &[ManifestValidationIssue] {
        &self.issues
    }

    /// Return whether the manifest passed all error-level checks.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        !self
            .issues
            .iter()
            .any(|issue| {
                issue.severity() == ManifestValidationSeverity::Error
            })
    }

    /// Return whether the report contains any issues.
    #[must_use]
    pub fn has_issues(&self) -> bool {
        !self.issues.is_empty()
    }

    /// Return the number of error-severity issues.
    #[must_use]
    pub fn error_count(&self) -> usize {
        self.issues
            .iter()
            .filter(|issue| {
                issue.severity() == ManifestValidationSeverity::Error
            })
            .count()
    }

    /// Return the number of warning-severity issues.
    #[must_use]
    pub fn warning_count(&self) -> usize {
        self.issues
            .iter()
            .filter(|issue| {
                issue.severity() == ManifestValidationSeverity::Warning
            })
            .count()
    }

    /// Consume the report and return it as a successful toolchain result
    /// only when no error-severity issue exists.
    ///
    /// The report remains available to the caller when this method is
    /// used by reference through [`ManifestValidator::validate`]. This
    /// consuming conversion is intended for orchestration boundaries that
    /// require a simple success/failure result.
    ///
    /// # Errors
    ///
    /// Returns a structured `ValidationFailed` toolchain error when one
    /// or more error-severity issues exist. The error includes the total
    /// error count and the first issue's stable code and field path.
    /// Callers that need every diagnostic should retain and inspect the
    /// report instead of relying only on this conversion.
    pub fn into_result(self) -> Result<Self> {
        if self.is_valid() {
            return Ok(self);
        }

        let first_error = self
            .issues
            .iter()
            .find(|issue| {
                issue.severity() == ManifestValidationSeverity::Error
            })
            .expect("an invalid report must contain an error");

        Err(
            ToolchainError::validation_failed(format!(
                "manifest validation failed with {} error(s)",
                self.error_count(),
            ))
            .with_operation(VALIDATE_OPERATION)
            .with_context(format!(
                "first issue: {} at {}",
                first_error.code(),
                first_error.field(),
            ))
            .with_context(first_error.message().to_owned()),
        )
    }
}

// ============================================================================
// VALIDATION POLICY
// ============================================================================

/// Options controlling structural manifest validation.
///
/// The current canonical schema does not define optional project policies
/// such as requiring authors or a description. The validator therefore
/// does not invent such requirements.
///
/// Future policy options may be added when their semantics are specified
/// by the canonical manifest and toolchain configuration contracts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ManifestValidationOptions {
    // Reserved for future explicitly specified policy switches.
    //
    // Do not add arbitrary limits or target/domain-specific switches here.
}

impl ManifestValidationOptions {
    /// Construct the default structural validation policy.
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}

// ============================================================================
// MANIFEST VALIDATOR
// ============================================================================

/// Stateless validator for the current canonical Zamani manifest schema.
///
/// The validator accepts an already parsed `ZamaniToml`. It never reads a
/// file, reparses TOML, resolves a dependency, or changes the manifest.
///
/// The same validator can be used by CLI, IDE, CI, automation, and agent
/// clients so they receive identical structural validation behavior.
#[derive(Debug, Default, Clone, Copy)]
pub struct ManifestValidator;

impl ManifestValidator {
    /// Construct a stateless manifest validator.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Validate a canonical manifest using the default policy.
    ///
    /// This method collects all detectable structural issues in one pass.
    /// It returns a report even when the manifest is invalid.
    #[must_use]
    pub fn validate(
        &self,
        manifest: &ZamaniToml,
    ) -> ManifestValidationReport {
        self.validate_with_options(
            manifest,
            ManifestValidationOptions::new(),
        )
    }

    /// Validate a canonical manifest using explicit validation options.
    ///
    /// Validation is deterministic: dependency declarations are visited
    /// in sorted key order, and feature declarations are visited in their
    /// source order.
    ///
    /// # Scope
    ///
    /// This validates only data represented by the current canonical
    /// `ZamaniToml` structure. It cannot detect malformed source values
    /// that `ZamaniToml::parse_from_str` has already discarded or replaced.
    #[must_use]
    pub fn validate_with_options(
        &self,
        manifest: &ZamaniToml,
        _options: ManifestValidationOptions,
    ) -> ManifestValidationReport {
        let mut report = ManifestValidationReport::new();

        validate_package(&manifest.package, &mut report);
        validate_dependencies(&manifest.dependencies, &mut report);

        report
    }

    /// Validate a canonical manifest and convert an invalid report into
    /// the shared structured toolchain error contract.
    ///
    /// Use [`Self::validate`] when the caller needs the complete list of
    /// issues. Use this method at a boundary where an invalid manifest
    /// must immediately stop the requested operation.
    ///
    /// # Errors
    ///
    /// Returns `ToolchainErrorKind::ValidationFailed` when one or more
    /// error-level issues are present.
    pub fn validate_or_error(
        &self,
        manifest: &ZamaniToml,
    ) -> Result<ManifestValidationReport> {
        self.validate(manifest).into_result()
    }
}

// ============================================================================
// PACKAGE VALIDATION
// ============================================================================

fn validate_package(
    package: &PackageSection,
    report: &mut ManifestValidationReport,
) {
    validate_required_text(
        "package.name",
        &package.name,
        ISSUE_REQUIRED_VALUE,
        report,
    );

    validate_required_text(
        "package.version",
        &package.version,
        ISSUE_REQUIRED_VALUE,
        report,
    );

    validate_required_text(
        "package.edition",
        &package.edition,
        ISSUE_REQUIRED_VALUE,
        report,
    );

    validate_required_text(
        "package.type",
        &package.package_type,
        ISSUE_REQUIRED_VALUE,
        report,
    );

    // The canonical schema currently represents description as String,
    // not Option<String>. An empty description is permitted; a supplied
    // nonempty value must still be well-formed text.
    validate_optional_text(
        "package.description",
        Some(&package.description),
        report,
    );

    validate_optional_text(
        "package.homepage",
        package.homepage.as_deref(),
        report,
    );

    validate_optional_text(
        "package.repository",
        package.repository.as_deref(),
        report,
    );

    validate_optional_text(
        "package.license",
        package.license.as_deref(),
        report,
    );

    for (index, author) in package.authors.iter().enumerate() {
        let field = format!("package.authors[{index}]");

        validate_required_text(
            &field,
            author,
            ISSUE_REQUIRED_VALUE,
            report,
        );
    }
}

// ============================================================================
// DEPENDENCY VALIDATION
// ============================================================================

fn validate_dependencies(
    dependencies: &std::collections::HashMap<String, DependencyConfig>,
    report: &mut ManifestValidationReport,
) {
    // The canonical schema uses HashMap. Sorting ensures stable issue order
    // and stable diagnostics regardless of hash iteration order.
    let mut entries: Vec<_> = dependencies.iter().collect();

    entries.sort_unstable_by(|left, right| left.0.cmp(right.0));

    for (name, declaration) in entries {
        validate_dependency(name, declaration, report);
    }
}

fn validate_dependency(
    name: &str,
    declaration: &DependencyConfig,
    report: &mut ManifestValidationReport,
) {
    let dependency_field = format!("dependencies[{name:?}]");

    validate_required_text(
        &format!("{dependency_field}.name"),
        name,
        ISSUE_REQUIRED_VALUE,
        report,
    );

    let has_version = !declaration.version.trim().is_empty();
    let has_path = declaration
        .path
        .as_deref()
        .is_some_and(|path| !path.trim().is_empty());

    if !has_version && !has_path {
        report.issues.push(ManifestValidationIssue::new(
            ISSUE_DEPENDENCY_SOURCE_MISSING,
            ManifestValidationSeverity::Error,
            dependency_field.clone(),
            "dependency must declare a nonempty version requirement or local path",
        ));
    }

    if has_version {
        validate_required_text(
            &format!("{dependency_field}.version"),
            &declaration.version,
            ISSUE_REQUIRED_VALUE,
            report,
        );
    }

    if let Some(path) = declaration.path.as_deref() {
        validate_dependency_path(
            &format!("{dependency_field}.path"),
            path,
            report,
        );
    }

    validate_dependency_features(
        name,
        declaration.features.as_deref(),
        report,
    );
}

fn validate_dependency_path(
    field: &str,
    value: &str,
    report: &mut ManifestValidationReport,
) {
    if value.trim().is_empty() {
        // An empty path is not a usable source declaration. The
        // dependency-level source check above reports the missing source
        // relationship; this issue identifies the malformed path itself.
        report.issues.push(ManifestValidationIssue::new(
            ISSUE_INVALID_DEPENDENCY_PATH,
            ManifestValidationSeverity::Error,
            field,
            "dependency path must not be empty or whitespace-only",
        ));
        return;
    }

    if value.chars().any(char::is_control) {
        report.issues.push(ManifestValidationIssue::new(
            ISSUE_CONTROL_CHARACTER,
            ManifestValidationSeverity::Error,
            field,
            "dependency path must not contain control characters",
        ));
        return;
    }

    if value != value.trim() {
        report.issues.push(ManifestValidationIssue::new(
            ISSUE_SURROUNDING_WHITESPACE,
            ManifestValidationSeverity::Error,
            field,
            "dependency path must not contain surrounding whitespace",
        ));
        return;
    }

    // Constructing a Path performs no filesystem access. In particular,
    // do not reject parent components here: whether a relative dependency
    // path is allowed to leave a project root is a source-resolution and
    // security-policy decision owned by dependencies::sources.
    let path = Path::new(value);

    if path.as_os_str().is_empty() {
        report.issues.push(ManifestValidationIssue::new(
            ISSUE_INVALID_DEPENDENCY_PATH,
            ManifestValidationSeverity::Error,
            field,
            "dependency path does not identify a path",
        ));
    }
}

fn validate_dependency_features(
    dependency_name: &str,
    features: Option<&[String]>,
    report: &mut ManifestValidationReport,
) {
    let Some(features) = features else {
        return;
    };

    let mut seen = BTreeSet::new();

    for (index, feature) in features.iter().enumerate() {
        let field = format!(
            "dependencies[{dependency_name:?}].features[{index}]"
        );

        validate_required_text(
            &field,
            feature,
            ISSUE_REQUIRED_VALUE,
            report,
        );

        // Duplicate detection is exact and case-sensitive. The canonical
        // schema does not define case folding or feature-name aliases.
        if !seen.insert(feature.as_str()) {
            report.issues.push(ManifestValidationIssue::new(
                ISSUE_DUPLICATE_FEATURE,
                ManifestValidationSeverity::Error,
                field,
                format!("feature {feature:?} is declared more than once"),
            ));
        }
    }
}

// ============================================================================
// SHARED TEXT VALIDATION
// ============================================================================

fn validate_required_text(
    field: &str,
    value: &str,
    required_code: &'static str,
    report: &mut ManifestValidationReport,
) {
    if value.is_empty() || value.trim().is_empty() {
        report.issues.push(ManifestValidationIssue::new(
            required_code,
            ManifestValidationSeverity::Error,
            field,
            "required value must not be empty or whitespace-only",
        ));
        return;
    }

    validate_nonempty_text(field, value, report);
}

fn validate_optional_text(
    field: &str,
    value: Option<&str>,
    report: &mut ManifestValidationReport,
) {
    let Some(value) = value else {
        return;
    };

    // Empty optional metadata is equivalent to an omitted optional value
    // in the current canonical schema. Whitespace-only metadata is not.
    if value.is_empty() {
        return;
    }

    if value.trim().is_empty() {
        report.issues.push(ManifestValidationIssue::new(
            ISSUE_REQUIRED_VALUE,
            ManifestValidationSeverity::Error,
            field,
            "optional value must be omitted or contain non-whitespace text",
        ));
        return;
    }

    validate_nonempty_text(field, value, report);
}

fn validate_nonempty_text(
    field: &str,
    value: &str,
    report: &mut ManifestValidationReport,
) {
    if value.chars().any(char::is_control) {
        report.issues.push(ManifestValidationIssue::new(
            ISSUE_CONTROL_CHARACTER,
            ManifestValidationSeverity::Error,
            field,
            "value must not contain control characters",
        ));
        return;
    }

    if value != value.trim() {
        report.issues.push(ManifestValidationIssue::new(
            ISSUE_SURROUNDING_WHITESPACE,
            ManifestValidationSeverity::Error,
            field,
            "value must not contain surrounding whitespace",
        ));
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::{
        ManifestValidationSeverity,
        ManifestValidator,
        ISSUE_DEPENDENCY_SOURCE_MISSING,
        ISSUE_DUPLICATE_FEATURE,
        ISSUE_INVALID_DEPENDENCY_PATH,
        ISSUE_REQUIRED_VALUE,
        ISSUE_SURROUNDING_WHITESPACE,
    };

    use crate::toolchain::errors::ToolchainErrorKind;
    use crate::zamani_project_config::{
        DependencyConfig,
        PackageSection,
        ZamaniToml,
    };

    use std::collections::HashMap;

    fn valid_package() -> PackageSection {
        PackageSection {
            name: "example".to_owned(),
            version: "1.0.0".to_owned(),
            authors: vec!["Zamani Developer".to_owned()],
            edition: "2021".to_owned(),
            description: "Example project".to_owned(),
            homepage: None,
            repository: None,
            license: Some("MIT".to_owned()),
            package_type: "application".to_owned(),
        }
    }

    fn manifest(
        package: PackageSection,
        dependencies: HashMap<String, DependencyConfig>,
    ) -> ZamaniToml {
        ZamaniToml {
            package,
            dependencies,
        }
    }

    fn dependency(
        version: &str,
        path: Option<&str>,
        features: Option<Vec<&str>>,
    ) -> DependencyConfig {
        DependencyConfig {
            version: version.to_owned(),
            path: path.map(str::to_owned),
            features: features.map(|items| {
                items.into_iter().map(str::to_owned).collect()
            }),
        }
    }

    fn codes(
        report: &super::ManifestValidationReport,
    ) -> Vec<&'static str> {
        report
            .issues()
            .iter()
            .map(|issue| issue.code())
            .collect()
    }

    #[test]
    fn accepts_minimal_valid_package_without_dependencies() {
        let input = manifest(valid_package(), HashMap::new());

        let report = ManifestValidator::new().validate(&input);

        assert!(report.is_valid());
        assert!(!report.has_issues());
        assert_eq!(report.error_count(), 0);
        assert_eq!(report.warning_count(), 0);
    }

    #[test]
    fn accepts_version_dependency() {
        let mut dependencies = HashMap::new();
        dependencies.insert(
            "core.math".to_owned(),
            dependency(">=1,<2", None, None),
        );

        let report = ManifestValidator::new().validate(
            &manifest(valid_package(), dependencies),
        );

        assert!(report.is_valid());
    }

    #[test]
    fn accepts_local_path_dependency_without_version() {
        let mut dependencies = HashMap::new();
        dependencies.insert(
            "local.library".to_owned(),
            dependency("", Some("../library"), None),
        );

        let report = ManifestValidator::new().validate(
            &manifest(valid_package(), dependencies),
        );

        assert!(report.is_valid());
    }

    #[test]
    fn accepts_dependency_with_both_version_and_path() {
        let mut dependencies = HashMap::new();
        dependencies.insert(
            "local.library".to_owned(),
            dependency("^1.2", Some("../library"), None),
        );

        let report = ManifestValidator::new().validate(
            &manifest(valid_package(), dependencies),
        );

        assert!(report.is_valid());
    }

    #[test]
    fn rejects_missing_required_package_values() {
        let mut package = valid_package();
        package.name.clear();
        package.version = "   ".to_owned();
        package.edition.clear();
        package.package_type.clear();

        let report = ManifestValidator::new().validate(
            &manifest(package, HashMap::new()),
        );

        assert!(!report.is_valid());
        assert_eq!(
            report
                .issues()
                .iter()
                .filter(|issue| issue.code() == ISSUE_REQUIRED_VALUE)
                .count(),
            4
        );
    }

    #[test]
    fn rejects_blank_author_entries() {
        let mut package = valid_package();
        package.authors = vec![
            "Valid Author".to_owned(),
            "   ".to_owned(),
        ];

        let report = ManifestValidator::new().validate(
            &manifest(package, HashMap::new()),
        );

        assert!(report.issues().iter().any(|issue| {
            issue.field() == "package.authors[1]"
                && issue.code() == ISSUE_REQUIRED_VALUE
        }));
    }

    #[test]
    fn rejects_surrounding_whitespace_in_required_values() {
        let mut package = valid_package();
        package.name = " example ".to_owned();

        let report = ManifestValidator::new().validate(
            &manifest(package, HashMap::new()),
        );

        assert!(report.issues().iter().any(|issue| {
            issue.field() == "package.name"
                && issue.code() == ISSUE_SURROUNDING_WHITESPACE
        }));
    }

    #[test]
    fn rejects_dependency_without_version_or_path() {
        let mut dependencies = HashMap::new();
        dependencies.insert(
            "missing.source".to_owned(),
            dependency("", None, None),
        );

        let report = ManifestValidator::new().validate(
            &manifest(valid_package(), dependencies),
        );

        assert!(report.issues().iter().any(|issue| {
            issue.code() == ISSUE_DEPENDENCY_SOURCE_MISSING
        }));
    }

    #[test]
    fn rejects_empty_dependency_path() {
        let mut dependencies = HashMap::new();
        dependencies.insert(
            "empty.path".to_owned(),
            dependency("", Some("   "), None),
        );

        let report = ManifestValidator::new().validate(
            &manifest(valid_package(), dependencies),
        );

        assert!(report.issues().iter().any(|issue| {
            issue.code() == ISSUE_INVALID_DEPENDENCY_PATH
        }));
    }

    #[test]
    fn rejects_duplicate_dependency_features() {
        let mut dependencies = HashMap::new();
        dependencies.insert(
            "feature.package".to_owned(),
            dependency(
                "^1",
                None,
                Some(vec!["classical", "quantum", "classical"]),
            ),
        );

        let report = ManifestValidator::new().validate(
            &manifest(valid_package(), dependencies),
        );

        assert!(report.issues().iter().any(|issue| {
            issue.code() == ISSUE_DUPLICATE_FEATURE
                && issue.field()
                    == "dependencies[\"feature.package\"].features[2]"
        }));
    }

    #[test]
    fn feature_names_are_case_sensitive() {
        let mut dependencies = HashMap::new();
        dependencies.insert(
            "feature.package".to_owned(),
            dependency(
                "^1",
                None,
                Some(vec!["Quantum", "quantum"]),
            ),
        );

        let report = ManifestValidator::new().validate(
            &manifest(valid_package(), dependencies),
        );

        assert!(report.is_valid());
    }

    #[test]
    fn reports_all_detectable_issues_in_one_pass() {
        let mut package = valid_package();
        package.name.clear();
        package.edition = " ".to_owned();

        let mut dependencies = HashMap::new();
        dependencies.insert(
            "missing.source".to_owned(),
            dependency("", None, None),
        );

        let report = ManifestValidator::new().validate(
            &manifest(package, dependencies),
        );

        assert!(!report.is_valid());
        assert!(report.error_count() >= 3);
    }

    #[test]
    fn dependency_issue_order_is_deterministic() {
        let mut first = HashMap::new();
        first.insert(
            "zeta".to_owned(),
            dependency("", None, None),
        );
        first.insert(
            "alpha".to_owned(),
            dependency("", None, None),
        );

        let mut second = HashMap::new();
        second.insert(
            "alpha".to_owned(),
            dependency("", None, None),
        );
        second.insert(
            "zeta".to_owned(),
            dependency("", None, None),
        );

        let validator = ManifestValidator::new();

        let first_report = validator.validate(
            &manifest(valid_package(), first),
        );
        let second_report = validator.validate(
            &manifest(valid_package(), second),
        );

        let first_fields: Vec<_> = first_report
            .issues()
            .iter()
            .map(|issue| issue.field())
            .collect();

        let second_fields: Vec<_> = second_report
            .issues()
            .iter()
            .map(|issue| issue.field())
            .collect();

        assert_eq!(first_fields, second_fields);
    }

    #[test]
    fn validation_error_conversion_uses_shared_toolchain_error() {
        let mut package = valid_package();
        package.name.clear();

        let input = manifest(package, HashMap::new());

        let error = ManifestValidator::new()
            .validate_or_error(&input)
            .expect_err("invalid manifest must fail");

        assert_eq!(
            error.kind(),
            ToolchainErrorKind::ValidationFailed
        );
        assert_eq!(
            error.operation(),
            Some("manifest.validation.validate")
        );
    }

    #[test]
    fn valid_report_can_be_converted_to_success() {
        let input = manifest(valid_package(), HashMap::new());

        let report = ManifestValidator::new()
            .validate(&input)
            .into_result()
            .expect("valid manifest report");

        assert!(report.is_valid());
    }

    #[test]
    fn empty_optional_metadata_is_permitted() {
        let mut package = valid_package();
        package.description.clear();
        package.homepage = Some(String::new());
        package.repository = None;
        package.license = Some(String::new());

        let report = ManifestValidator::new().validate(
            &manifest(package, HashMap::new()),
        );

        assert!(report.is_valid());
    }

    #[test]
    fn whitespace_only_optional_metadata_is_rejected() {
        let mut package = valid_package();
        package.description = "   ".to_owned();

        let report = ManifestValidator::new().validate(
            &manifest(package, HashMap::new()),
        );

        assert!(!report.is_valid());
    }

    #[test]
    fn parent_relative_dependency_path_is_not_resolved_or_rejected_here() {
        let mut dependencies = HashMap::new();
        dependencies.insert(
            "sibling.library".to_owned(),
            dependency("", Some("../library"), None),
        );

        let report = ManifestValidator::new().validate(
            &manifest(valid_package(), dependencies),
        );

        // Containment and authorization are checked by dependency-source
        // resolution, not by structural manifest validation.
        assert!(report.is_valid());
    }

    #[test]
    fn issue_severity_is_explicit() {
        let mut package = valid_package();
        package.name.clear();

        let report = ManifestValidator::new().validate(
            &manifest(package, HashMap::new()),
        );

        assert_eq!(
            report.issues()[0].severity(),
            ManifestValidationSeverity::Error
        );
    }

    #[test]
    fn report_codes_are_stable_for_required_value_failures() {
        let mut package = valid_package();
        package.name.clear();

        let report = ManifestValidator::new().validate(
            &manifest(package, HashMap::new()),
        );

        assert_eq!(
            codes(&report),
            vec![ISSUE_REQUIRED_VALUE]
        );
    }
}
