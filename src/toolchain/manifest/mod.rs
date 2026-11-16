
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Toolchain — Manifest Integration Boundary
//!
//! Path: `src/toolchain/manifest/mod.rs`
//! Edition: Rust 2021
//! Minimum supported Rust: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Provides the toolchain-facing interface to Zamani project manifests.
//!
//! This module adapts the canonical manifest implementation owned by
//! `crate::zamani_project_config`.
//!
//! It MUST NOT introduce another TOML parser, manifest schema, package
//! identity authority, dependency model, language-version authority,
//! target registry, hardware model, or semantic-analysis implementation.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - The toolchain-facing manifest adapter.
//! - Structural validation of the canonical manifest model.
//! - A stable boundary for future workspace/build/dependency integration.
//! - Preservation of canonical manifest data.
//! - Conversion of manifest parsing and validation failures into
//!   structured toolchain errors.
//!
//! It does not own:
//!
//! - TOML parsing implementation.
//! - Canonical project configuration structures.
//! - Language grammar or syntax.
//! - Language semantic validation.
//! - Dependency resolution.
//! - Package version resolution.
//! - Compiler configuration.
//! - Target discovery.
//! - Physical hardware discovery.
//! - Backend selection.
//! - Runtime configuration execution.
//! - Deployment.
//! - Manifest migration execution.
//!
//! ## Canonical authority
//!
//! `crate::zamani_project_config` owns:
//!
//! - `ZamaniToml`
//! - `PackageSection`
//! - `DependencyConfig`
//! - Canonical manifest parsing.
//!
//! The toolchain consumes these types through this adapter.
//!
//! ## Integration
//!
//! - `workspace/` consumes project manifest metadata.
//! - `configuration/` consumes toolchain-facing configuration.
//! - `dependencies/` consumes dependency declarations.
//! - `build/` consumes validated project/build intent.
//! - `targets/` consumes target intent after the canonical schema
//!   supports it.
//! - `packages/` consumes package metadata.
//! - `registries/` consumes registry declarations after schema integration.
//! - `deployment/` consumes deployment intent after schema integration.
//!
//! The frontend and grammar remain independent of this module.
//!
//! ## Validation boundary
//!
//! Validation here covers only fields represented by the current
//! canonical `ZamaniToml` model.
//!
//! It does not claim that unrepresented TOML sections have been validated.
//!
//! Full manifest validation requires the canonical project configuration
//! subsystem to model and validate those sections.
//!
//! ## Scalability
//!
//! This module introduces no fixed limits on:
//!
//! - Project size.
//! - Dependency count.
//! - Workspace members.
//! - Computational domains.
//! - Target profiles.
//! - Providers.
//! - Hardware resources.
//! - Distributed participants.
//! - Tensor dimensions.
//! - Quantum resources.
//!
//! Memory and processing requirements are determined by actual inputs,
//! available resources, and configured operational budgets.
//!
//! Literal infinite storage or execution is not promised.
//!
//! ## Compatibility
//!
//! Manifest schema versions, language versions, compiler versions,
//! IR versions, package versions and registry protocol versions
//! are independent contracts.
//!
//! This module does not infer compatibility from a version string alone.
//!
//! ## Security
//!
//! This module does not access the filesystem, network, environment
//! variables, credentials, processes or hardware.
//!
//! File loading and external operations belong to their respective
//! toolchain services.
//!
//! All externally supplied configuration must be validated by its
//! authoritative subsystem before use.

use std::collections::BTreeMap;

use crate::zamani_project_config::{
    DependencyConfig,
    PackageSection,
    ZamaniToml,
};

use super::super::errors::ToolchainError;
use super::super::result::Result;

// ============================================================================
// CANONICAL MANIFEST RE-EXPORTS
// ============================================================================

/// Canonical Zamani project manifest representation.
///
/// The original definition and parser remain owned by
/// `crate::zamani_project_config`.
pub use crate::zamani_project_config::ZamaniToml as CanonicalManifest;

/// Canonical package section.
///
/// This is not a competing package metadata structure.
pub use crate::zamani_project_config::PackageSection as CanonicalPackageSection;

/// Canonical dependency configuration.
///
/// Dependency resolution remains owned by the dependencies subsystem.
pub use crate::zamani_project_config::DependencyConfig as CanonicalDependencyConfig;

// ============================================================================
// MANIFEST ADAPTER
// ============================================================================

/// A validated toolchain-facing view of a canonical Zamani manifest.
///
/// This wrapper preserves the original manifest model rather than
/// translating it into a competing schema.
///
/// Construction validates the fields represented by the current
/// canonical model.
///
/// Important:
///
/// The current `ZamaniToml` implementation represents package metadata
/// and dependencies. It does not yet represent every section in the
/// repository's root `Zamani.toml`.
///
/// Consequently, this adapter does not claim that unrepresented sections
/// have been parsed or validated.
///
/// Once the canonical project configuration model expands, this adapter
/// can expose the additional fields without changing the ownership model.
#[derive(Debug, Clone, PartialEq)]
pub struct ToolchainManifest {
    canonical: ZamaniToml,
}

impl ToolchainManifest {
    /// Parse a manifest using the canonical project configuration parser.
    ///
    /// This function deliberately delegates TOML parsing to
    /// `ZamaniToml::parse_from_str`.
    ///
    /// It does not implement another TOML parser.
    ///
    /// # Errors
    ///
    /// Returns a structured toolchain error if parsing or structural
    /// validation fails.
    pub fn parse(source: &str) -> Result<Self> {
        if source.trim().is_empty() {
            return Err(
                ToolchainError::invalid_input(
                    "manifest source must not be empty",
                )
                .with_operation("manifest.parse"),
            );
        }

        let canonical = ZamaniToml::parse_from_str(source)
            .map_err(|error| {
                ToolchainError::invalid_input(
                    format!("canonical manifest parser rejected input: {error}"),
                )
                .with_operation("manifest.parse")
            })?;

        Self::from_canonical(canonical)
    }

    /// Construct an adapter from an existing canonical manifest.
    ///
    /// This is the preferred integration path when a workspace or project
    /// configuration service has already parsed the manifest.
    ///
    /// The supplied canonical model is preserved without reconstructing it.
    ///
    /// # Errors
    ///
    /// Returns an error if the represented manifest fields are invalid.
    pub fn from_canonical(canonical: ZamaniToml) -> Result<Self> {
        let manifest = Self { canonical };

        manifest.validate()?;

        Ok(manifest)
    }

    /// Borrow the original canonical manifest.
    ///
    /// The returned reference is the original model, not a reconstructed
    /// representation.
    #[must_use]
    pub fn canonical(&self) -> &ZamaniToml {
        &self.canonical
    }

    /// Consume the adapter and return the original canonical manifest.
    #[must_use]
    pub fn into_canonical(self) -> ZamaniToml {
        self.canonical
    }

    /// Borrow canonical package metadata.
    #[must_use]
    pub fn package(&self) -> &PackageSection {
        &self.canonical.package
    }

    /// Borrow canonical dependency declarations.
    ///
    /// Dependency resolution and feature unification do not belong here.
    #[must_use]
    pub fn dependencies(
        &self,
    ) -> &std::collections::HashMap<String, DependencyConfig> {
        &self.canonical.dependencies
    }

    /// Return the canonical project name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.canonical.package.name
    }

    /// Return the canonical project version string.
    ///
    /// Package version parsing and release compatibility are owned by
    /// the package/dependency versioning contracts.
    #[must_use]
    pub fn version(&self) -> &str {
        &self.canonical.package.version
    }

    /// Return the declared project type.
    ///
    /// This is project metadata, not a closed computational-domain enum.
    #[must_use]
    pub fn package_type(&self) -> &str {
        &self.canonical.package.package_type
    }

    /// Return whether the canonical manifest declares any dependencies.
    #[must_use]
    pub fn has_dependencies(&self) -> bool {
        !self.canonical.dependencies.is_empty()
    }

    /// Return dependency declarations in deterministic name order.
    ///
    /// The canonical configuration currently uses a `HashMap`.
    /// This method provides deterministic iteration for downstream
    /// reporting, planning and fingerprint preparation.
    ///
    /// It does not resolve versions or establish dependency order.
    #[must_use]
    pub fn ordered_dependencies(
        &self,
    ) -> BTreeMap<&str, &DependencyConfig> {
        self.canonical
            .dependencies
            .iter()
            .map(|(name, dependency)| {
                (name.as_str(), dependency)
            })
            .collect()
    }

    /// Validate all fields represented by the canonical manifest model.
    ///
    /// This validation is intentionally limited to the actual fields
    /// represented by `ZamaniToml`.
    ///
    /// It does not claim that build, feature, registry, target or
    /// deployment sections have been validated.
    ///
    /// # Errors
    ///
    /// Returns a structured validation error for malformed represented
    /// package or dependency metadata.
    pub fn validate(&self) -> Result<()> {
        validate_package(&self.canonical.package)?;
        validate_dependencies(&self.canonical.dependencies)?;

        Ok(())
    }
}

// ============================================================================
// PACKAGE METADATA VALIDATION
// ============================================================================

/// Validate the package metadata represented by the canonical model.
///
/// This function checks required structural information without imposing
/// a fixed package naming convention, project-type enumeration or
/// computational-domain restriction.
///
/// Package-name policy, release-version policy and package compatibility
/// remain the responsibility of their authoritative subsystems.
fn validate_package(package: &PackageSection) -> Result<()> {
    if package.name.trim().is_empty() {
        return Err(
            ToolchainError::validation_failed(
                "manifest package name must not be empty",
            )
            .with_operation("manifest.validate.package"),
        );
    }

    if package.version.trim().is_empty() {
        return Err(
            ToolchainError::validation_failed(
                "manifest package version must not be empty",
            )
            .with_operation("manifest.validate.package"),
        );
    }

    if package.package_type.trim().is_empty() {
        return Err(
            ToolchainError::validation_failed(
                "manifest package type must not be empty",
            )
            .with_operation("manifest.validate.package"),
        );
    }

    Ok(())
}

// ============================================================================
// DEPENDENCY DECLARATION VALIDATION
// ============================================================================

/// Validate dependency declarations represented by the canonical model.
///
/// A dependency must declare at least one usable source specification:
///
/// - A non-empty version requirement; or
/// - A non-empty local path.
///
/// If both are supplied, both remain available to the dependency resolver.
///
/// This function does not:
///
/// - Resolve versions.
/// - Check whether a path exists.
/// - Access the filesystem.
/// - Contact registries.
/// - Verify package signatures.
/// - Select a compatible dependency release.
///
/// Those operations belong to the dependency and package subsystems.
fn validate_dependencies(
    dependencies: &std::collections::HashMap<String, DependencyConfig>,
) -> Result<()> {
    for (name, dependency) in dependencies {
        if name.trim().is_empty() {
            return Err(
                ToolchainError::validation_failed(
                    "dependency name must not be empty",
                )
                .with_operation("manifest.validate.dependencies"),
            );
        }

        let has_version = !dependency.version.trim().is_empty();

        let has_path = dependency
            .path
            .as_deref()
            .is_some_and(|path| !path.trim().is_empty());

        if !has_version && !has_path {
            return Err(
                ToolchainError::validation_failed(
                    format!(
                        "dependency '{name}' must declare a non-empty \
                         version requirement or local path"
                    ),
                )
                .with_operation("manifest.validate.dependencies")
                .with_subject(name.clone()),
            );
        }

        if let Some(features) = &dependency.features {
            for feature in features {
                if feature.trim().is_empty() {
                    return Err(
                        ToolchainError::validation_failed(
                            format!(
                                "dependency '{name}' contains an empty \
                                 feature declaration"
                            ),
                        )
                        .with_operation("manifest.validate.dependencies")
                        .with_subject(name.clone()),
                    );
                }
            }
        }
    }

    Ok(())
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::{
        ToolchainManifest,
    };

    use crate::zamani_project_config::{
        DependencyConfig,
        PackageSection,
        ZamaniToml,
    };

    use std::collections::HashMap;

    fn valid_manifest() -> ZamaniToml {
        let mut dependencies = HashMap::new();

        dependencies.insert(
            "zamani_std".to_owned(),
            DependencyConfig {
                version: "0.1.0".to_owned(),
                path: None,
                features: None,
            },
        );

        ZamaniToml {
            package: PackageSection {
                name: "example_project".to_owned(),
                version: "0.1.0".to_owned(),
                authors: Vec::new(),
                edition: "2026".to_owned(),
                description: "Example project".to_owned(),
                homepage: None,
                repository: None,
                license: None,
                package_type: "application".to_owned(),
            },
            dependencies,
        }
    }

    #[test]
    fn canonical_manifest_is_preserved() {
        let canonical = valid_manifest();

        let adapter = ToolchainManifest::from_canonical(
            canonical.clone(),
        )
        .expect("valid canonical manifest");

        assert_eq!(adapter.canonical(), &canonical);
        assert_eq!(adapter.into_canonical(), canonical);
    }

    #[test]
    fn package_metadata_is_exposed_without_duplication() {
        let adapter = ToolchainManifest::from_canonical(
            valid_manifest(),
        )
        .expect("valid manifest");

        assert_eq!(adapter.name(), "example_project");
        assert_eq!(adapter.version(), "0.1.0");
        assert_eq!(adapter.package_type(), "application");
    }

    #[test]
    fn dependencies_are_exposed() {
        let adapter = ToolchainManifest::from_canonical(
            valid_manifest(),
        )
        .expect("valid manifest");

        assert!(adapter.has_dependencies());

        assert!(
            adapter.dependencies().contains_key("zamani_std")
        );
    }

    #[test]
    fn dependency_iteration_is_deterministic() {
        let mut canonical = valid_manifest();

        canonical.dependencies.insert(
            "alpha".to_owned(),
            DependencyConfig {
                version: "1.0.0".to_owned(),
                path: None,
                features: None,
            },
        );

        canonical.dependencies.insert(
            "zeta".to_owned(),
            DependencyConfig {
                version: "2.0.0".to_owned(),
                path: None,
                features: None,
            },
        );

        let adapter = ToolchainManifest::from_canonical(
            canonical,
        )
        .expect("valid manifest");

        let names: Vec<&str> = adapter
            .ordered_dependencies()
            .keys()
            .copied()
            .collect();

        assert_eq!(
            names,
            vec!["alpha", "zamani_std", "zeta"]
        );
    }

    #[test]
    fn empty_source_is_rejected() {
        let result = ToolchainManifest::parse("");

        assert!(result.is_err());
    }

    #[test]
    fn whitespace_source_is_rejected() {
        let result = ToolchainManifest::parse("   \n\t ");

        assert!(result.is_err());
    }

    #[test]
    fn missing_package_name_is_rejected() {
        let mut canonical = valid_manifest();

        canonical.package.name.clear();

        assert!(
            ToolchainManifest::from_canonical(canonical).is_err()
        );
    }

    #[test]
    fn missing_package_version_is_rejected() {
        let mut canonical = valid_manifest();

        canonical.package.version.clear();

        assert!(
            ToolchainManifest::from_canonical(canonical).is_err()
        );
    }

    #[test]
    fn missing_package_type_is_rejected() {
        let mut canonical = valid_manifest();

        canonical.package.package_type.clear();

        assert!(
            ToolchainManifest::from_canonical(canonical).is_err()
        );
    }

    #[test]
    fn dependency_without_version_or_path_is_rejected() {
        let mut canonical = valid_manifest();

        canonical.dependencies.insert(
            "invalid_dependency".to_owned(),
            DependencyConfig {
                version: String::new(),
                path: None,
                features: None,
            },
        );

        assert!(
            ToolchainManifest::from_canonical(canonical).is_err()
        );
    }

    #[test]
    fn local_path_dependency_is_accepted() {
        let mut canonical = valid_manifest();

        canonical.dependencies.insert(
            "local_library".to_owned(),
            DependencyConfig {
                version: String::new(),
                path: Some(
                    "./modules/local_library".to_owned(),
                ),
                features: None,
            },
        );

        assert!(
            ToolchainManifest::from_canonical(canonical).is_ok()
        );
    }

    #[test]
    fn empty_dependency_feature_is_rejected() {
        let mut canonical = valid_manifest();

        canonical.dependencies.insert(
            "feature_library".to_owned(),
            DependencyConfig {
                version: "1.0.0".to_owned(),
                path: None,
                features: Some(vec![
                    "valid_feature".to_owned(),
                    String::new(),
                ]),
            },
        );

        assert!(
            ToolchainManifest::from_canonical(canonical).is_err()
        );
    }

    #[test]
    fn no_dependency_is_required() {
        let mut canonical = valid_manifest();

        canonical.dependencies.clear();

        let adapter = ToolchainManifest::from_canonical(
            canonical,
        )
        .expect("dependency-free project is valid");

        assert!(!adapter.has_dependencies());
    }

    #[test]
    fn unknown_project_types_are_not_restricted_here() {
        let mut canonical = valid_manifest();

        canonical.package.package_type =
            "future_computational_domain".to_owned();

        assert!(
            ToolchainManifest::from_canonical(canonical).is_ok()
        );
    }

    #[test]
    fn all_supported_metadata_remains_available() {
        let mut canonical = valid_manifest();

        canonical.package.authors =
            vec!["Example Author".to_owned()];

        canonical.package.description =
            "Cross-domain project".to_owned();

        canonical.package.homepage =
            Some("https://example.invalid".to_owned());

        canonical.package.repository =
            Some("https://example.invalid/repository".to_owned());

        canonical.package.license =
            Some("Apache-2.0".to_owned());

        let adapter = ToolchainManifest::from_canonical(
            canonical.clone(),
        )
        .expect("valid manifest");

        assert_eq!(
            adapter.canonical(),
            &canonical
        );
    }
}
