
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Toolchain — Manifest Adapter
//!
//! Path: `src/toolchain/manifest/adapter.rs`
//! Edition: Rust 2021.
//! Minimum Rust version: 1.97.
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Adapts the canonical Zamani project manifest into validated,
//! deterministic toolchain-facing project and dependency descriptions.
//!
//! The canonical manifest parser and schema remain owned by
//! `crate::zamani_project_config`.
//!
//! This module does not implement another TOML parser.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Validation at the manifest-to-toolchain boundary.
//! - Deterministic dependency projection.
//! - Toolchain-facing manifest snapshots.
//! - Conversion of a project root and explicit target intent into
//!   the canonical public BuildRequest.
//!
//! It does not own:
//!
//! - TOML parsing or serialization.
//! - The canonical manifest schema.
//! - Workspace schema or workspace discovery.
//! - Dependency resolution or version-constraint interpretation.
//! - Dependency fetching.
//! - Package identity allocation.
//! - Build graph construction.
//! - Compiler configuration or compilation.
//! - Target registration or target feasibility.
//! - Hardware discovery or allocation.
//! - Runtime execution.
//! - Deployment.
//!
//! ## Integration
//!
//! Upstream:
//!
//! - `crate::zamani_project_config::ZamaniToml`
//! - `crate::zamani_project_config::PackageSection`
//! - `crate::zamani_project_config::DependencyConfig`
//!
//! Downstream:
//!
//! - `manifest::loading` supplies parsed canonical manifests.
//! - `manifest::validation` performs broader manifest validation.
//! - `manifest::compatibility` handles manifest-version compatibility.
//! - `dependencies::model` consumes normalized dependency descriptions.
//! - `dependencies::constraints` interprets version requirements.
//! - `dependencies::sources` resolves dependency sources.
//! - `workspace::members` consumes normalized workspace-member
//!   declarations when the canonical schema supports them.
//! - `api::request` owns public build-request contracts.
//! - `build::request` and `build::plan` consume validated build intent.
//!
//! ## Domain independence
//!
//! This adapter does not enumerate classical, quantum, HDL, AI,
//! hybrid, distributed, networking, cryptographic or future domains.
//!
//! Domain-specific meaning remains owned by the appropriate semantic,
//! compiler, IR, backend, hardware and runtime authorities.
//!
//! ## POCO-REAF
//!
//! One logical project may be adapted into a build request that can
//! produce multiple compatible target-specific realizations.
//!
//! The caller supplies target-selection intent explicitly.
//!
//! An empty capability-preference list is permitted by the canonical
//! TargetSelection contract and does not imply native execution.
//!
//! This module never claims that a requested target is feasible.
//!
//! ## Determinism
//!
//! The canonical manifest currently stores dependencies in a HashMap.
//! This adapter converts them into a lexicographically ordered Vec.
//!
//! Dependency feature declarations are also normalized into deterministic
//! order after duplicate validation.
//!
//! The original canonical manifest is never modified.
//!
//! ## Scalability
//!
//! No arbitrary maximum is imposed on:
//!
//! - Dependencies.
//! - Package authors.
//! - Dependency features.
//! - Dependency path lengths.
//! - Package metadata.
//! - Targets.
//! - Computing domains.
//! - Hardware resources.
//!
//! Memory consumption is proportional to the adapted manifest data.
//! Actual execution remains subject to available resources and explicit
//! operational budgets.
//!
//! Literal infinite storage or computation is not promised.
//!
//! ## Security
//!
//! - No unsafe Rust.
//! - No shell execution.
//! - No process execution.
//! - No filesystem access.
//! - No network access.
//! - No manifest expression evaluation.
//! - No implicit dependency fetching.
//! - No credential storage.
//! - No silent data truncation.
//! - No silent conversion of invalid data into defaults.
//!
//! Local dependency paths are preserved as declarations. Their
//! canonicalization, containment, authorization and existence checks
//! belong to the dependency-source resolution subsystem.
//!
//! ## Freeze boundary
//!
//! This file depends on the canonical manifest types rather than
//! reimplementing them.
//!
//! Expanding the canonical manifest schema must not require unrelated
//! changes to this adapter. New fields should be projected only when
//! their canonical ownership and downstream contracts have been defined.

use crate::toolchain::api::{
    BuildRequest,
    ProjectLocator,
    ProjectScope,
    TargetSelection,
};

use crate::toolchain::errors::ToolchainError;
use crate::toolchain::result::Result;

use crate::zamani_project_config::{
    DependencyConfig,
    PackageSection,
    ZamaniToml,
};

use std::collections::BTreeSet;
use std::path::{
    Path,
    PathBuf,
};

// ============================================================================
// OPERATION IDENTIFIERS
// ============================================================================

const ADAPT_OPERATION: &str = "manifest.adapter.adapt";
const BUILD_REQUEST_OPERATION: &str = "manifest.adapter.build_request";

// ============================================================================
// NORMALIZED DEPENDENCY
// ============================================================================

/// A deterministic toolchain-facing representation of one declared
/// project dependency.
///
/// This type preserves the canonical manifest's declarations without
/// interpreting version constraints or resolving dependency sources.
///
/// Version interpretation belongs to `dependencies::constraints`.
///
/// Source resolution belongs to `dependencies::sources`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManifestDependency {
    name: String,
    version_requirement: Option<String>,
    path: Option<PathBuf>,
    features: Vec<String>,
}

impl ManifestDependency {
    /// Return the declared dependency name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return the original version requirement.
    ///
    /// `None` means that the canonical manifest did not provide
    /// a nonempty version requirement.
    ///
    /// This value is not interpreted as a semantic version by
    /// the adapter.
    #[must_use]
    pub fn version_requirement(&self) -> Option<&str> {
        self.version_requirement.as_deref()
    }

    /// Return the declared local dependency path.
    ///
    /// The path is not canonicalized or accessed by this adapter.
    #[must_use]
    pub fn path(&self) -> Option<&Path> {
        self.path.as_deref()
    }

    /// Return deterministically ordered dependency features.
    #[must_use]
    pub fn features(&self) -> &[String] {
        &self.features
    }

    /// Return whether this declaration specifies a local path.
    #[must_use]
    pub fn is_path_dependency(&self) -> bool {
        self.path.is_some()
    }
}

// ============================================================================
// ADAPTED MANIFEST
// ============================================================================

/// A validated, immutable toolchain-facing snapshot of a canonical
/// Zamani project manifest.
///
/// The package metadata retains its canonical type. Dependencies are
/// represented by [`ManifestDependency`] because the canonical manifest
/// uses an unordered HashMap.
///
/// Construct snapshots through [`ManifestAdapter::adapt`].
#[derive(Debug, Clone, PartialEq)]
pub struct AdaptedManifest {
    package: PackageSection,
    dependencies: Vec<ManifestDependency>,
}

impl AdaptedManifest {
    /// Return the canonical package metadata.
    #[must_use]
    pub fn package(&self) -> &PackageSection {
        &self.package
    }

    /// Return dependencies in deterministic name order.
    #[must_use]
    pub fn dependencies(&self) -> &[ManifestDependency] {
        &self.dependencies
    }

    /// Return the number of directly declared dependencies.
    #[must_use]
    pub fn dependency_count(&self) -> usize {
        self.dependencies.len()
    }

    /// Return whether the manifest declares no direct dependencies.
    #[must_use]
    pub fn has_no_dependencies(&self) -> bool {
        self.dependencies.is_empty()
    }

    /// Find a declared dependency by its canonical manifest name.
    ///
    /// The lookup uses binary search because the adapter guarantees
    /// deterministic lexicographic dependency ordering.
    #[must_use]
    pub fn dependency(&self, name: &str) -> Option<&ManifestDependency> {
        self.dependencies
            .binary_search_by(|dependency| {
                dependency.name.as_str().cmp(name)
            })
            .ok()
            .map(|index| &self.dependencies[index])
    }

    /// Construct a canonical public build request for this project.
    ///
    /// `project_root` is supplied by the caller because the manifest
    /// model does not own filesystem location.
    ///
    /// `targets` must be supplied explicitly. The adapter does not
    /// assume a native target, a particular architecture, or a
    /// particular computational domain.
    ///
    /// This function constructs request intent only. It does not:
    ///
    /// - Resolve dependencies.
    /// - Discover targets.
    /// - Validate hardware capabilities.
    /// - Determine physical feasibility.
    /// - Invoke the compiler.
    /// - Execute the program.
    ///
    /// # Errors
    ///
    /// Returns a structured toolchain error when the supplied project
    /// location or target-selection request is structurally invalid.
    pub fn to_build_request(
        &self,
        project_root: impl Into<PathBuf>,
        targets: TargetSelection,
    ) -> Result<BuildRequest> {
        let project_root = project_root.into();

        if project_root.as_os_str().is_empty() {
            return Err(
                ToolchainError::invalid_input(
                    "project root must not be empty",
                )
                .with_operation(BUILD_REQUEST_OPERATION),
            );
        }

        let locator = ProjectLocator::from_path(project_root)
            .map_err(|error| {
                ToolchainError::invalid_input(error.to_string())
                    .with_operation(BUILD_REQUEST_OPERATION)
                    .with_context(
                        format!("request field: {}", error.field()),
                    )
            })?;

        let scope = ProjectScope::new(locator)
            .map_err(|error| {
                ToolchainError::invalid_input(error.to_string())
                    .with_operation(BUILD_REQUEST_OPERATION)
                    .with_context(
                        format!("request field: {}", error.field()),
                    )
            })?;

        BuildRequest::new(scope, targets)
            .map_err(|error| {
                ToolchainError::invalid_input(error.to_string())
                    .with_operation(BUILD_REQUEST_OPERATION)
                    .with_context(
                        format!("request field: {}", error.field()),
                    )
            })
    }
}

// ============================================================================
// MANIFEST ADAPTER
// ============================================================================

/// Stateless adapter from the canonical Zamani project configuration
/// into toolchain-facing project and dependency contracts.
///
/// The adapter does not own the canonical manifest parser or schema.
///
/// It can therefore be reused across CLI, IDE, automation, CI and
/// AI-agent entry points without creating separate manifest semantics.
#[derive(Debug, Default, Clone, Copy)]
pub struct ManifestAdapter;

impl ManifestAdapter {
    /// Construct a stateless manifest adapter.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Adapt and validate an already parsed canonical manifest.
    ///
    /// This method never reparses TOML.
    ///
    /// It preserves package metadata and converts the dependency map
    /// into deterministic toolchain-facing records.
    ///
    /// # Errors
    ///
    /// Returns a structured error when required package metadata,
    /// dependency identities, dependency requirements, or feature
    /// declarations are invalid.
    pub fn adapt(
        &self,
        manifest: &ZamaniToml,
    ) -> Result<AdaptedManifest> {
        validate_package(&manifest.package)?;

        let mut dependencies = Vec::with_capacity(
            manifest.dependencies.len(),
        );

        // The canonical manifest currently uses HashMap. Sorting keys
        // before projection prevents nondeterministic build inputs.
        let mut entries: Vec<_> = manifest.dependencies.iter().collect();

        entries.sort_unstable_by(|left, right| {
            left.0.cmp(right.0)
        });

        for (name, declaration) in entries {
            dependencies.push(
                adapt_dependency(name, declaration)?,
            );
        }

        Ok(AdaptedManifest {
            package: manifest.package.clone(),
            dependencies,
        })
    }

    /// Validate and adapt a canonical manifest using the default
    /// stateless adapter.
    ///
    /// This convenience method is equivalent to
    /// `ManifestAdapter::new().adapt(manifest)`.
    pub fn adapt_manifest(
        manifest: &ZamaniToml,
    ) -> Result<AdaptedManifest> {
        Self::new().adapt(manifest)
    }
}

// ============================================================================
// PACKAGE VALIDATION
// ============================================================================

fn validate_package(package: &PackageSection) -> Result<()> {
    validate_required_text(
        "package.name",
        &package.name,
    )?;

    validate_required_text(
        "package.version",
        &package.version,
    )?;

    validate_required_text(
        "package.edition",
        &package.edition,
    )?;

    validate_required_text(
        "package.type",
        &package.package_type,
    )?;

    validate_optional_text(
        "package.description",
        Some(&package.description),
    )?;

    validate_optional_text(
        "package.homepage",
        package.homepage.as_deref(),
    )?;

    validate_optional_text(
        "package.repository",
        package.repository.as_deref(),
    )?;

    validate_optional_text(
        "package.license",
        package.license.as_deref(),
    )?;

    for (index, author) in package.authors.iter().enumerate() {
        validate_required_text(
            &format!("package.authors[{index}]"),
            author,
        )?;
    }

    Ok(())
}

// ============================================================================
// DEPENDENCY ADAPTATION
// ============================================================================

fn adapt_dependency(
    name: &str,
    declaration: &DependencyConfig,
) -> Result<ManifestDependency> {
    validate_required_text(
        "dependencies.name",
        name,
    )
    .map_err(|error| {
        error
            .with_context("invalid dependency identity")
    })?;

    // An empty version is permitted only when another source declaration,
    // such as a local path, is present.
    let version_requirement = if declaration.version.is_empty() {
        None
    } else {
        validate_required_text(
            "dependencies.version",
            &declaration.version,
        )?;

        Some(declaration.version.clone())
    };

    let path = match declaration.path.as_deref() {
        Some(value) => {
            validate_required_text(
                "dependencies.path",
                value,
            )?;

            Some(PathBuf::from(value))
        }

        None => None,
    };

    if version_requirement.is_none() && path.is_none() {
        return Err(
            ToolchainError::validation_failed(
                "dependency must declare a version requirement or a local path",
            )
            .with_operation(ADAPT_OPERATION)
            .with_subject(name),
        );
    }

    let features = normalize_features(
        name,
        declaration.features.as_deref(),
    )?;

    Ok(ManifestDependency {
        name: name.to_owned(),
        version_requirement,
        path,
        features,
    })
}

// ============================================================================
// FEATURE NORMALIZATION
// ============================================================================

fn normalize_features(
    dependency_name: &str,
    declared_features: Option<&[String]>,
) -> Result<Vec<String>> {
    let Some(declared_features) = declared_features else {
        return Ok(Vec::new());
    };

    let mut unique = BTreeSet::new();

    for feature in declared_features {
        validate_required_text(
            "dependencies.features",
            feature,
        )
        .map_err(|error| {
            error
                .with_subject(dependency_name)
                .with_context("invalid dependency feature")
        })?;

        if !unique.insert(feature.as_str()) {
            return Err(
                ToolchainError::validation_failed(
                    "dependency declares the same feature more than once",
                )
                .with_operation(ADAPT_OPERATION)
                .with_subject(dependency_name)
                .with_context(format!("duplicate feature: {feature}")),
            );
        }
    }

    Ok(unique
        .into_iter()
        .map(str::to_owned)
        .collect())
}

// ============================================================================
// COMMON TEXT VALIDATION
// ============================================================================

fn validate_required_text(
    field: &str,
    value: &str,
) -> Result<()> {
    if value.is_empty() || value.trim().is_empty() {
        return Err(
            ToolchainError::validation_failed(
                format!("{field} must not be empty"),
            )
            .with_operation(ADAPT_OPERATION)
            .with_context(format!("field: {field}")),
        );
    }

    if value.chars().any(char::is_control) {
        return Err(
            ToolchainError::validation_failed(
                format!("{field} must not contain control characters"),
            )
            .with_operation(ADAPT_OPERATION)
            .with_context(format!("field: {field}")),
        );
    }

    if value != value.trim() {
        return Err(
            ToolchainError::validation_failed(
                format!("{field} must not contain surrounding whitespace"),
            )
            .with_operation(ADAPT_OPERATION)
            .with_context(format!("field: {field}")),
        );
    }

    Ok(())
}

fn validate_optional_text(
    field: &str,
    value: Option<&str>,
) -> Result<()> {
    if let Some(value) = value {
        if !value.is_empty() {
            validate_required_text(field, value)?;
        }
    }

    Ok(())
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::zamani_project_config::{
        DependencyConfig,
        PackageSection,
        ZamaniToml,
    };

    use std::collections::HashMap;

    fn package() -> PackageSection {
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
        dependencies: HashMap<String, DependencyConfig>,
    ) -> ZamaniToml {
        ZamaniToml {
            package: package(),
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
            features: features.map(|values| {
                values.into_iter().map(str::to_owned).collect()
            }),
        }
    }

    #[test]
    fn adapts_empty_manifest() {
        let input = manifest(HashMap::new());

        let result = ManifestAdapter::new()
            .adapt(&input)
            .expect("valid manifest should adapt");

        assert_eq!(result.package().name, "example");
        assert!(result.has_no_dependencies());
    }

    #[test]
    fn preserves_package_metadata() {
        let input = manifest(HashMap::new());

        let result = ManifestAdapter::adapt_manifest(&input)
            .expect("valid manifest should adapt");

        assert_eq!(result.package(), &input.package);
    }

    #[test]
    fn adapts_version_dependency() {
        let mut dependencies = HashMap::new();

        dependencies.insert(
            "core.math".to_owned(),
            dependency(">=1,<2", None, None),
        );

        let result = ManifestAdapter::adapt_manifest(
            &manifest(dependencies),
        )
        .expect("valid dependency should adapt");

        let adapted = result
            .dependency("core.math")
            .expect("dependency should exist");

        assert_eq!(
            adapted.version_requirement(),
            Some(">=1,<2"),
        );

        assert_eq!(adapted.path(), None);
    }

    #[test]
    fn adapts_local_path_dependency() {
        let mut dependencies = HashMap::new();

        dependencies.insert(
            "local.library".to_owned(),
            dependency("", Some("../library"), None),
        );

        let result = ManifestAdapter::adapt_manifest(
            &manifest(dependencies),
        )
        .expect("local dependency should adapt");

        let adapted = result
            .dependency("local.library")
            .expect("dependency should exist");

        assert!(adapted.is_path_dependency());

        assert_eq!(
            adapted.path(),
            Some(Path::new("../library")),
        );

        assert_eq!(adapted.version_requirement(), None);
    }

    #[test]
    fn supports_version_and_local_path_together() {
        let mut dependencies = HashMap::new();

        dependencies.insert(
            "local.library".to_owned(),
            dependency(">=2", Some("../library"), None),
        );

        let result = ManifestAdapter::adapt_manifest(
            &manifest(dependencies),
        )
        .expect("combined declaration should adapt");

        let adapted = result
            .dependency("local.library")
            .expect("dependency should exist");

        assert_eq!(
            adapted.version_requirement(),
            Some(">=2"),
        );

        assert_eq!(
            adapted.path(),
            Some(Path::new("../library")),
        );
    }

    #[test]
    fn dependency_order_is_deterministic() {
        let mut dependencies = HashMap::new();

        dependencies.insert(
            "zeta".to_owned(),
            dependency("1", None, None),
        );

        dependencies.insert(
            "alpha".to_owned(),
            dependency("2", None, None),
        );

        let result = ManifestAdapter::adapt_manifest(
            &manifest(dependencies),
        )
        .expect("valid dependencies should adapt");

        let names: Vec<&str> = result
            .dependencies()
            .iter()
            .map(ManifestDependency::name)
            .collect();

        assert_eq!(names, vec!["alpha", "zeta"]);
    }

    #[test]
    fn feature_order_is_deterministic() {
        let mut dependencies = HashMap::new();

        dependencies.insert(
            "example".to_owned(),
            dependency(
                "1",
                None,
                Some(vec!["zeta", "alpha", "middle"]),
            ),
        );

        let result = ManifestAdapter::adapt_manifest(
            &manifest(dependencies),
        )
        .expect("valid features should adapt");

        assert_eq!(
            result.dependencies()[0].features(),
            &["alpha", "middle", "zeta"],
        );
    }

    #[test]
    fn rejects_duplicate_features() {
        let mut dependencies = HashMap::new();

        dependencies.insert(
            "example".to_owned(),
            dependency(
                "1",
                None,
                Some(vec!["compute", "compute"]),
            ),
        );

        let result = ManifestAdapter::adapt_manifest(
            &manifest(dependencies),
        );

        assert!(result.is_err());
    }

    #[test]
    fn rejects_missing_dependency_source_and_version() {
        let mut dependencies = HashMap::new();

        dependencies.insert(
            "example".to_owned(),
            dependency("", None, None),
        );

        assert!(
            ManifestAdapter::adapt_manifest(
                &manifest(dependencies),
            )
            .is_err()
        );
    }

    #[test]
    fn rejects_empty_package_name() {
        let mut input = manifest(HashMap::new());

        input.package.name.clear();

        assert!(
            ManifestAdapter::adapt_manifest(&input).is_err()
        );
    }

    #[test]
    fn rejects_empty_package_version() {
        let mut input = manifest(HashMap::new());

        input.package.version.clear();

        assert!(
            ManifestAdapter::adapt_manifest(&input).is_err()
        );
    }

    #[test]
    fn rejects_control_characters_in_package_metadata() {
        let mut input = manifest(HashMap::new());

        input.package.name = "example\npackage".to_owned();

        assert!(
            ManifestAdapter::adapt_manifest(&input).is_err()
        );
    }

    #[test]
    fn rejects_empty_dependency_path() {
        let mut dependencies = HashMap::new();

        dependencies.insert(
            "example".to_owned(),
            dependency("1", Some(""), None),
        );

        assert!(
            ManifestAdapter::adapt_manifest(
                &manifest(dependencies),
            )
            .is_err()
        );
    }

    #[test]
    fn does_not_modify_the_canonical_manifest() {
        let mut dependencies = HashMap::new();

        dependencies.insert(
            "example".to_owned(),
            dependency("1", None, Some(vec!["zeta", "alpha"])),
        );

        let input = manifest(dependencies.clone());
        let original = input.clone();

        let _ = ManifestAdapter::adapt_manifest(&input)
            .expect("valid manifest should adapt");

        assert_eq!(input, original);
    }

    #[test]
    fn adapts_a_real_toml_manifest_through_canonical_parser() {
        let source = r#"
            [package]
            name = "scientific"
            version = "1.0.0"
            edition = "2021"
            description = "Scientific computing"
            type = "application"

            [dependencies]
            math = ">=1"

            [dependencies.local]
            version = "2"
            path = "../local"
            features = ["compute", "parallel"]
        "#;

        let parsed = ZamaniToml::parse_from_str(source)
            .expect("canonical parser should parse the manifest");

        let result = ManifestAdapter::adapt_manifest(&parsed)
            .expect("adapter should accept the parsed manifest");

        assert_eq!(result.package().name, "scientific");
        assert_eq!(result.dependency_count(), 2);

        assert!(
            result.dependency("math").is_some()
        );

        assert!(
            result.dependency("local").is_some()
        );
    }
}
