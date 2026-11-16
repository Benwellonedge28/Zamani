
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Toolchain — Manifest Compatibility
//!
//! Path: src/toolchain/manifest/compatibility.rs
//! Edition: Rust 2021
//! Minimum supported Rust: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Evaluates whether a Zamani project manifest schema can be interpreted
//! by the current toolchain implementation.
//!
//! This module coordinates the existing canonical compatibility engine.
//! It does not implement another version parser or compatibility engine.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Manifest schema declaration provenance.
//! - Explicit handling of unversioned legacy manifests.
//! - Manifest-specific compatibility checking.
//! - Manifest compatibility reports.
//! - Fail-closed interpretation requirements.
//! - Manifest compatibility error conversion.
//!
//! It does not own:
//!
//! - Canonical TOML parsing.
//! - Manifest schema definitions.
//! - Language version compatibility.
//! - Compiler version compatibility.
//! - Package archive compatibility.
//! - Dependency version resolution.
//! - Schema migration execution.
//! - Hardware or target compatibility.
//! - Domain-specific semantics.
//!
//! ## Canonical authorities
//!
//! - crate::zamani_project_config owns the manifest schema and parser.
//! - crate::toolchain::version owns ManifestSchemaVersion.
//! - crate::toolchain::api::compatibility owns compatibility evaluation.
//! - crate::toolchain::errors owns structured toolchain errors.
//! - crate::toolchain::result owns the toolchain result contract.
//!
//! ## Version separation
//!
//! Manifest schema versions are independent of:
//!
//! - Zamani language versions.
//! - Grammar versions.
//! - Package release versions.
//! - Compiler versions.
//! - Toolchain API versions.
//! - IR versions.
//! - Backend versions.
//! - Runtime versions.
//! - Hardware firmware versions.
//!
//! ## Migration boundary
//!
//! This module identifies when migration is required.
//!
//! It never performs migration and never claims that an unsupported
//! historical representation has been migrated.
//!
//! Migration execution belongs to a separately specified migration service.
//!
//! ## Legacy manifests
//!
//! The existing canonical ZamaniToml representation has no manifest
//! schema-version field.
//!
//! Therefore an unversioned manifest must be identified explicitly
//! as LegacyUnversioned.
//!
//! The compatibility policy determines whether that legacy representation
//! may be interpreted as a configured historical schema version.
//!
//! The assumed version is included in the resulting report.
//!
//! ## Scalability
//!
//! This module imposes no limits on:
//!
//! - Manifest size.
//! - Dependencies.
//! - Workspace members.
//! - Targets.
//! - Providers.
//! - Computational domains.
//! - Hardware resources.
//! - Distributed participants.
//!
//! Compatibility checking is independent of physical capacity.
//!
//! Memory usage is proportional to the small compatibility report,
//! not to the size of the computational resources requested.
//!
//! Literal infinite computation or storage is not promised.
//!
//! ## Security
//!
//! - No unsafe Rust.
//! - No filesystem access.
//! - No network access.
//! - No process execution.
//! - No shell execution.
//! - No manifest expression evaluation.
//! - No automatic migration.
//! - No silent version substitution.
//! - No silent acceptance of unknown future versions.
//! - No silent conversion of incompatible versions.
//!
//! ## Freeze boundary
//!
//! This file depends on the existing version and compatibility contracts.
//!
//! Changes to the canonical manifest schema must be implemented in
//! src/zamani_project_config/.
//!
//! This file must not become a second manifest parser.

use crate::toolchain::api::compatibility::{
    CompatibilityCheck,
    CompatibilityChecker,
    CompatibilityPolicies,
    CompatibilityStatus,
};

use crate::toolchain::errors::ToolchainError;
use crate::toolchain::result::Result;

use crate::toolchain::version::{
    initial_manifest_schema_policy,
    ManifestSchemaVersion,
    VersionPolicy,
    INITIAL_MANIFEST_SCHEMA_VERSION,
};

use core::fmt;
use core::str::FromStr;

// ============================================================================
// OPERATION IDENTIFIERS
// ============================================================================

const CHECK_OPERATION: &str = "manifest.compatibility.check";

const REQUIRE_OPERATION: &str = "manifest.compatibility.require";

// ============================================================================
// MANIFEST SCHEMA DECLARATION
// ============================================================================

/// Identifies how a manifest schema version was obtained.
///
/// An unversioned manifest is not automatically equivalent to a
/// particular schema version. Its interpretation is controlled by
/// [`LegacyManifestPolicy`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ManifestSchemaDeclaration {
    /// The manifest explicitly declares its schema version.
    Explicit(ManifestSchemaVersion),

    /// The manifest does not declare a schema version.
    ///
    /// This variant represents a legacy manifest whose schema identity
    /// must be established by an explicit compatibility policy.
    LegacyUnversioned,
}

impl ManifestSchemaDeclaration {
    /// Construct a declaration from optional version metadata.
    ///
    /// The caller must supply the version extracted by the canonical
    /// manifest configuration subsystem.
    ///
    /// `None` means that the manifest is unversioned.
    ///
    /// An empty or malformed version is an error, not a legacy manifest.
    ///
    /// # Errors
    ///
    /// Returns a structured error when the supplied version is malformed.
    pub fn from_optional_str(
        value: Option<&str>,
    ) -> Result<Self> {
        match value {
            None => Ok(Self::LegacyUnversioned),

            Some(value) => {
                let version = ManifestSchemaVersion::from_str(value)
                    .map_err(|error| {
                        ToolchainError::invalid_input(
                            "manifest schema version is malformed",
                        )
                        .with_operation(CHECK_OPERATION)
                        .with_context(format!(
                            "declared manifest schema version: {value}"
                        ))
                        .with_context(error.to_string())
                    })?;

                Ok(Self::Explicit(version))
            }
        }
    }

    /// Return the explicitly declared version, if one exists.
    #[must_use]
    pub const fn explicit_version(
        self,
    ) -> Option<ManifestSchemaVersion> {
        match self {
            Self::Explicit(version) => Some(version),
            Self::LegacyUnversioned => None,
        }
    }

    /// Return whether the declaration is explicitly versioned.
    #[must_use]
    pub const fn is_explicit(self) -> bool {
        matches!(self, Self::Explicit(_))
    }

    /// Return whether the declaration represents a legacy manifest.
    #[must_use]
    pub const fn is_legacy(self) -> bool {
        matches!(self, Self::LegacyUnversioned)
    }
}

// ============================================================================
// LEGACY MANIFEST POLICY
// ============================================================================

/// Policy for manifests that do not declare a schema version.
///
/// Legacy interpretation must be explicit and auditable.
///
/// This policy does not infer a version from:
///
/// - Package version.
/// - Language edition.
/// - Compiler release.
/// - File modification time.
/// - Toolchain release.
/// - Operating system.
/// - Target hardware.
///
/// Only the configured manifest schema version may be used.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LegacyManifestPolicy {
    /// Reject manifests that do not explicitly declare a schema version.
    RequireExplicitVersion,

    /// Interpret an unversioned manifest using an explicitly configured
    /// historical schema version.
    ///
    /// The resulting report records that the version was assumed rather
    /// than explicitly declared.
    ///
    /// This option is appropriate only when the canonical reader is known
    /// to implement the specified historical schema.
    AssumeVersion(ManifestSchemaVersion),
}

impl LegacyManifestPolicy {
    /// Construct the compatibility bridge for the initial manifest schema.
    ///
    /// This is the explicit legacy bridge for the existing repository
    /// manifest representation, which currently has no schema-version field.
    ///
    /// It is not a general rule that every unversioned future manifest
    /// belongs to the initial schema.
    #[must_use]
    pub const fn initial_schema_legacy_bridge() -> Self {
        Self::AssumeVersion(INITIAL_MANIFEST_SCHEMA_VERSION)
    }

    /// Return the schema version used for legacy interpretation.
    #[must_use]
    pub const fn assumed_version(
        self,
    ) -> Option<ManifestSchemaVersion> {
        match self {
            Self::RequireExplicitVersion => None,
            Self::AssumeVersion(version) => Some(version),
        }
    }

    /// Return whether unversioned manifests are rejected.
    #[must_use]
    pub const fn requires_explicit_version(self) -> bool {
        matches!(self, Self::RequireExplicitVersion)
    }
}

// ============================================================================
// MANIFEST COMPATIBILITY STATUS
// ============================================================================

/// Manifest-specific compatibility outcome.
///
/// These outcomes describe the ability to interpret the manifest schema.
/// They do not establish language, compiler, target or hardware compatibility.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ManifestCompatibilityStatus {
    /// The manifest schema may be interpreted without migration.
    Compatible,

    /// A supported historical schema requires migration first.
    RequiresMigration,

    /// The manifest schema cannot be interpreted safely.
    Incompatible,

    /// No compatibility policy is configured for the schema.
    PolicyUnavailable,

    /// The manifest is unversioned and policy requires an explicit version.
    LegacyVersionRequired,
}

impl ManifestCompatibilityStatus {
    /// Return whether normal manifest interpretation is permitted.
    #[must_use]
    pub const fn permits_interpretation(self) -> bool {
        matches!(self, Self::Compatible)
    }

    /// Return whether a migration is required.
    #[must_use]
    pub const fn requires_migration(self) -> bool {
        matches!(self, Self::RequiresMigration)
    }

    /// Return whether this status blocks normal interpretation.
    #[must_use]
    pub const fn blocks_interpretation(self) -> bool {
        !self.permits_interpretation()
    }

    /// Return a stable machine-readable status identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Compatible => "compatible",
            Self::RequiresMigration => "requires_migration",
            Self::Incompatible => "incompatible",
            Self::PolicyUnavailable => "policy_unavailable",
            Self::LegacyVersionRequired => "legacy_version_required",
        }
    }
}

impl fmt::Display for ManifestCompatibilityStatus {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// COMPATIBILITY REPORT
// ============================================================================

/// Immutable report for one manifest schema compatibility evaluation.
///
/// The report preserves the distinction between an explicitly declared
/// schema version and a version assigned through a legacy policy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManifestCompatibilityReport {
    declaration: ManifestSchemaDeclaration,
    effective_version: Option<ManifestSchemaVersion>,
    status: ManifestCompatibilityStatus,
    check: Option<CompatibilityCheck>,
}

impl ManifestCompatibilityReport {
    /// Construct a report from a canonical compatibility check.
    fn from_check(
        declaration: ManifestSchemaDeclaration,
        effective_version: ManifestSchemaVersion,
        check: CompatibilityCheck,
    ) -> Self {
        let status = match check.status() {
            CompatibilityStatus::Compatible => {
                ManifestCompatibilityStatus::Compatible
            }

            CompatibilityStatus::RequiresMigration => {
                ManifestCompatibilityStatus::RequiresMigration
            }

            CompatibilityStatus::Incompatible => {
                ManifestCompatibilityStatus::Incompatible
            }

            CompatibilityStatus::PolicyUnavailable => {
                ManifestCompatibilityStatus::PolicyUnavailable
            }
        };

        Self {
            declaration,
            effective_version: Some(effective_version),
            status,
            check: Some(check),
        }
    }

    /// Construct a report for a rejected unversioned manifest.
    fn legacy_version_required() -> Self {
        Self {
            declaration: ManifestSchemaDeclaration::LegacyUnversioned,
            effective_version: None,
            status: ManifestCompatibilityStatus::LegacyVersionRequired,
            check: None,
        }
    }

    /// Return the original version declaration.
    #[must_use]
    pub const fn declaration(
        &self,
    ) -> ManifestSchemaDeclaration {
        self.declaration
    }

    /// Return the version used for compatibility evaluation.
    ///
    /// This may be an explicitly declared version or a version assigned
    /// by the configured legacy policy.
    #[must_use]
    pub const fn effective_version(
        &self,
    ) -> Option<ManifestSchemaVersion> {
        self.effective_version
    }

    /// Return whether the effective version was explicitly declared.
    #[must_use]
    pub const fn version_was_explicit(self: &Self) -> bool {
        self.declaration.is_explicit()
    }

    /// Return the manifest compatibility status.
    #[must_use]
    pub const fn status(
        &self,
    ) -> ManifestCompatibilityStatus {
        self.status
    }

    /// Return the underlying canonical compatibility evidence.
    ///
    /// `None` means that the manifest was rejected before a version
    /// compatibility check could be performed.
    #[must_use]
    pub fn compatibility_check(
        &self,
    ) -> Option<&CompatibilityCheck> {
        self.check.as_ref()
    }

    /// Return whether normal manifest interpretation is permitted.
    #[must_use]
    pub const fn is_compatible(&self) -> bool {
        self.status.permits_interpretation()
    }

    /// Return whether migration is required.
    #[must_use]
    pub const fn requires_migration(&self) -> bool {
        self.status.requires_migration()
    }

    /// Return whether normal interpretation must be blocked.
    #[must_use]
    pub const fn blocks_interpretation(&self) -> bool {
        self.status.blocks_interpretation()
    }

    /// Validate the internal consistency of this report.
    ///
    /// This is useful when reports are passed between toolchain services.
    ///
    /// # Errors
    ///
    /// Returns an error if the effective version, declaration, status
    /// and underlying compatibility evidence contradict one another.
    pub fn validate(&self) -> Result<()> {
        match (&self.declaration, self.effective_version, &self.check) {
            (
                ManifestSchemaDeclaration::Explicit(declared),
                Some(effective),
                Some(check),
            ) if declared == &effective => {
                validate_check_status(self.status, check)
            }

            (
                ManifestSchemaDeclaration::LegacyUnversioned,
                Some(_),
                Some(check),
            ) => {
                validate_check_status(self.status, check)
            }

            (
                ManifestSchemaDeclaration::LegacyUnversioned,
                None,
                None,
            ) if self.status
                == ManifestCompatibilityStatus::LegacyVersionRequired =>
            {
                Ok(())
            }

            _ => Err(
                ToolchainError::internal(
                    "manifest compatibility report contains inconsistent evidence",
                )
                .with_operation(CHECK_OPERATION),
            ),
        }
    }

    /// Return the report only if normal interpretation is permitted.
    ///
    /// Migration-required, incompatible, unconfigured and unversioned
    /// rejected manifests are never converted into successful results.
    ///
    /// # Errors
    ///
    /// Returns an appropriate structured toolchain error when interpretation
    /// is blocked.
    pub fn into_compatible_result(self) -> Result<Self> {
        self.validate()?;

        if self.is_compatible() {
            return Ok(self);
        }

        let mut error = ToolchainError::incompatible_version(
            format!(
                "manifest schema compatibility check failed: {}",
                self.status
            ),
        )
        .with_operation(REQUIRE_OPERATION);

        if let Some(version) = self.effective_version {
            error = error.with_context(format!(
                "effective manifest schema version: {version}"
            ));
        }

        if let Some(check) = &self.check {
            error = error.with_context(format!(
                "compatibility reason: {:?}",
                check.reason()
            ));
        }

        if self.declaration.is_legacy() {
            error = error.with_context(
                "the manifest does not explicitly declare its schema version",
            );
        }

        Err(error)
    }
}

fn validate_check_status(
    status: ManifestCompatibilityStatus,
    check: &CompatibilityCheck,
) -> Result<()> {
    check.validate()?;

    let expected = match check.status() {
        CompatibilityStatus::Compatible => {
            ManifestCompatibilityStatus::Compatible
        }

        CompatibilityStatus::RequiresMigration => {
            ManifestCompatibilityStatus::RequiresMigration
        }

        CompatibilityStatus::Incompatible => {
            ManifestCompatibilityStatus::Incompatible
        }

        CompatibilityStatus::PolicyUnavailable => {
            ManifestCompatibilityStatus::PolicyUnavailable
        }
    };

    if status != expected {
        return Err(
            ToolchainError::internal(
                "manifest status does not match canonical compatibility evidence",
            )
            .with_operation(CHECK_OPERATION),
        );
    }

    Ok(())
}

// ============================================================================
// MANIFEST COMPATIBILITY CHECKER
// ============================================================================

/// Immutable manifest compatibility service.
///
/// The checker delegates version decisions to
/// [`CompatibilityChecker`].
///
/// It does not maintain an independent compatibility algorithm.
///
/// A checker can be shared between operations because its policies
/// are immutable after construction.
#[derive(Debug, Clone)]
pub struct ManifestCompatibilityChecker {
    compatibility: CompatibilityChecker,
    legacy_policy: LegacyManifestPolicy,
}

impl ManifestCompatibilityChecker {
    /// Construct a checker from explicit compatibility policies.
    ///
    /// The supplied policies are validated by the canonical compatibility
    /// subsystem before this checker can be constructed.
    ///
    /// # Errors
    ///
    /// Returns an error if any supplied version policy is invalid.
    pub fn new(
        policies: CompatibilityPolicies,
        legacy_policy: LegacyManifestPolicy,
    ) -> Result<Self> {
        let compatibility = CompatibilityChecker::new(policies)?;

        Ok(Self {
            compatibility,
            legacy_policy,
        })
    }

    /// Construct a checker using the repository's conservative initial
    /// compatibility policies and the explicit initial-schema legacy bridge.
    ///
    /// Only manifest schema 1.0.0 is initially accepted by the existing
    /// conservative manifest policy.
    ///
    /// This does not certify support for historical versions that have
    /// not been independently implemented and tested.
    #[must_use]
    pub fn conservative_initial() -> Self {
        Self {
            compatibility: CompatibilityChecker::conservative_initial(),
            legacy_policy:
                LegacyManifestPolicy::initial_schema_legacy_bridge(),
        }
    }

    /// Construct a checker that requires explicit manifest schema versions.
    ///
    /// This is appropriate for strict new-project and registry boundaries.
    #[must_use]
    pub fn strict() -> Self {
        Self {
            compatibility: CompatibilityChecker::conservative_initial(),
            legacy_policy: LegacyManifestPolicy::RequireExplicitVersion,
        }
    }

    /// Construct a checker using a supplied canonical compatibility checker.
    ///
    /// This permits the application to share one authoritative compatibility
    /// policy configuration across the complete toolchain.
    #[must_use]
    pub const fn with_checker(
        compatibility: CompatibilityChecker,
        legacy_policy: LegacyManifestPolicy,
    ) -> Self {
        Self {
            compatibility,
            legacy_policy,
        }
    }

    /// Return the configured legacy manifest policy.
    #[must_use]
    pub const fn legacy_policy(&self) -> LegacyManifestPolicy {
        self.legacy_policy
    }

    /// Return the canonical compatibility checker.
    #[must_use]
    pub const fn compatibility_checker(
        &self,
    ) -> &CompatibilityChecker {
        &self.compatibility
    }

    /// Evaluate one manifest schema declaration.
    ///
    /// An explicit declaration is evaluated directly.
    ///
    /// An unversioned declaration follows the configured legacy policy.
    ///
    /// This operation never performs migration.
    ///
    /// # Errors
    ///
    /// Returns an error if the resulting compatibility evidence is
    /// internally inconsistent.
    pub fn check(
        &self,
        declaration: ManifestSchemaDeclaration,
    ) -> Result<ManifestCompatibilityReport> {
        match declaration {
            ManifestSchemaDeclaration::Explicit(version) => {
                self.check_version(declaration, version)
            }

            ManifestSchemaDeclaration::LegacyUnversioned => {
                match self.legacy_policy {
                    LegacyManifestPolicy::RequireExplicitVersion => {
                        Ok(
                            ManifestCompatibilityReport::legacy_version_required(),
                        )
                    }

                    LegacyManifestPolicy::AssumeVersion(version) => {
                        self.check_version(declaration, version)
                    }
                }
            }
        }
    }

    /// Evaluate an explicitly supplied manifest schema version.
    ///
    /// This method is useful when the canonical manifest configuration
    /// subsystem has already extracted and validated the version.
    ///
    /// # Errors
    ///
    /// Returns an error if compatibility evidence is internally inconsistent.
    pub fn check_version(
        &self,
        declaration: ManifestSchemaDeclaration,
        version: ManifestSchemaVersion,
    ) -> Result<ManifestCompatibilityReport> {
        if let ManifestSchemaDeclaration::Explicit(declared) = declaration {
            if declared != version {
                return Err(
                    ToolchainError::invalid_input(
                        "explicit manifest version does not match the supplied version",
                    )
                    .with_operation(CHECK_OPERATION)
                    .with_context(format!(
                        "declared version: {declared}"
                    ))
                    .with_context(format!(
                        "supplied version: {version}"
                    )),
                );
            }
        } else if self.legacy_policy.assumed_version() != Some(version) {
            return Err(
                ToolchainError::invalid_input(
                    "legacy manifest version does not match the configured legacy policy",
                )
                .with_operation(CHECK_OPERATION),
            );
        }

        let check = self.compatibility.check_manifest_schema(version);

        let report = ManifestCompatibilityReport::from_check(
            declaration,
            version,
            check,
        );

        report.validate()?;

        Ok(report)
    }

    /// Check a manifest and require compatibility without migration.
    ///
    /// This is the preferred interpretation gate before downstream
    /// manifest validation, adaptation or build planning.
    ///
    /// A migration-required result is not considered success.
    ///
    /// # Errors
    ///
    /// Returns an error when:
    ///
    /// - The manifest is unversioned and explicit versioning is required.
    /// - Its schema is incompatible.
    /// - Its schema requires migration.
    /// - No compatibility policy is configured.
    pub fn require_compatible(
        &self,
        declaration: ManifestSchemaDeclaration,
    ) -> Result<ManifestCompatibilityReport> {
        self.check(declaration)?
            .into_compatible_result()
    }

    /// Parse optional schema metadata and evaluate its compatibility.
    ///
    /// The supplied value must already have been extracted by the canonical
    /// manifest parser or its version-metadata adapter.
    ///
    /// This method does not parse TOML.
    ///
    /// # Errors
    ///
    /// Returns an error for malformed version metadata or inconsistent
    /// compatibility evidence.
    pub fn check_optional_version(
        &self,
        value: Option<&str>,
    ) -> Result<ManifestCompatibilityReport> {
        let declaration =
            ManifestSchemaDeclaration::from_optional_str(value)?;

        self.check(declaration)
    }

    /// Parse optional schema metadata and require compatible interpretation.
    ///
    /// # Errors
    ///
    /// Returns an error for malformed, incompatible, migration-required,
    /// unconfigured or rejected legacy schema versions.
    pub fn require_optional_version(
        &self,
        value: Option<&str>,
    ) -> Result<ManifestCompatibilityReport> {
        let declaration =
            ManifestSchemaDeclaration::from_optional_str(value)?;

        self.require_compatible(declaration)
    }
}

// ============================================================================
// POLICY CONSTRUCTION HELPERS
// ============================================================================

/// Construct a manifest-only compatibility policy.
///
/// This helper allows callers to configure manifest compatibility without
/// changing unrelated package, registry, provider or API policies.
///
/// # Errors
///
/// Returns an error if the current version is outside the supported range.
pub fn manifest_schema_policy(
    current: ManifestSchemaVersion,
    minimum_supported: ManifestSchemaVersion,
    maximum_supported: ManifestSchemaVersion,
    allow_older_without_migration: bool,
) -> Result<VersionPolicy<ManifestSchemaVersion>> {
    let range = crate::toolchain::version::VersionRange::new(
        minimum_supported,
        maximum_supported,
    )
    .map_err(|error| {
        ToolchainError::invalid_configuration(
            "invalid manifest schema compatibility range",
        )
        .with_operation(CHECK_OPERATION)
        .with_context(error.to_string())
    })?;

    VersionPolicy::new(
        current,
        range,
        allow_older_without_migration,
    )
    .map_err(|error| {
        ToolchainError::invalid_configuration(
            "invalid manifest schema compatibility policy",
        )
        .with_operation(CHECK_OPERATION)
        .with_context(error.to_string())
    })
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::toolchain::version::{
        VersionRange,
        VersionPolicy,
    };

    fn version(
        major: u32,
        minor: u32,
        patch: u32,
    ) -> ManifestSchemaVersion {
        ManifestSchemaVersion::new(major, minor, patch)
    }

    fn policies(
        minimum: ManifestSchemaVersion,
        current: ManifestSchemaVersion,
        allow_older_without_migration: bool,
    ) -> CompatibilityPolicies {
        let mut policies = CompatibilityPolicies::conservative_initial();

        let range = VersionRange::new(minimum, current)
            .expect("valid version range");

        policies.manifest_schema = Some(
            VersionPolicy::new(
                current,
                range,
                allow_older_without_migration,
            )
            .expect("valid manifest schema policy"),
        );

        policies
    }

    #[test]
    fn parses_explicit_manifest_version() {
        let declaration =
            ManifestSchemaDeclaration::from_optional_str(
                Some("1.0.0"),
            )
            .expect("valid schema version");

        assert_eq!(
            declaration.explicit_version(),
            Some(version(1, 0, 0))
        );

        assert!(declaration.is_explicit());
        assert!(!declaration.is_legacy());
    }

    #[test]
    fn identifies_unversioned_legacy_manifest() {
        let declaration =
            ManifestSchemaDeclaration::from_optional_str(None)
                .expect("legacy declaration");

        assert!(declaration.is_legacy());
        assert!(!declaration.is_explicit());
        assert_eq!(declaration.explicit_version(), None);
    }

    #[test]
    fn rejects_malformed_version() {
        for value in [
            "",
            "1",
            "1.0",
            "1..0",
            "01.0.0",
            "1.0.0.1",
            " 1.0.0",
            "1.0.0 ",
            "future",
        ] {
            assert!(
                ManifestSchemaDeclaration::from_optional_str(
                    Some(value)
                )
                .is_err(),
                "accepted malformed version: {value}"
            );
        }
    }

    #[test]
    fn accepts_exact_initial_schema() {
        let checker =
            ManifestCompatibilityChecker::conservative_initial();

        let report = checker
            .check(ManifestSchemaDeclaration::Explicit(
                INITIAL_MANIFEST_SCHEMA_VERSION,
            ))
            .expect("compatibility check");

        assert_eq!(
            report.status(),
            ManifestCompatibilityStatus::Compatible
        );

        assert!(report.is_compatible());
        assert!(!report.requires_migration());
        assert!(report.validate().is_ok());
    }

    #[test]
    fn future_schema_is_rejected() {
        let checker =
            ManifestCompatibilityChecker::conservative_initial();

        let report = checker
            .check(ManifestSchemaDeclaration::Explicit(
                version(2, 0, 0),
            ))
            .expect("compatibility evidence");

        assert_eq!(
            report.status(),
            ManifestCompatibilityStatus::Incompatible
        );

        assert!(report.blocks_interpretation());
        assert!(report.into_compatible_result().is_err());
    }

    #[test]
    fn older_schema_requires_migration_by_default() {
        let current = version(2, 0, 0);
        let older = version(1, 0, 0);

        let checker = ManifestCompatibilityChecker::new(
            policies(older, current, false),
            LegacyManifestPolicy::RequireExplicitVersion,
        )
        .expect("valid checker");

        let report = checker
            .check(ManifestSchemaDeclaration::Explicit(older))
            .expect("compatibility evidence");

        assert_eq!(
            report.status(),
            ManifestCompatibilityStatus::RequiresMigration
        );

        assert!(report.requires_migration());
        assert!(report.into_compatible_result().is_err());
    }

    #[test]
    fn explicitly_supported_older_schema_can_be_compatible() {
        let current = version(2, 0, 0);
        let older = version(1, 0, 0);

        let checker = ManifestCompatibilityChecker::new(
            policies(older, current, true),
            LegacyManifestPolicy::RequireExplicitVersion,
        )
        .expect("valid checker");

        let report = checker
            .check(ManifestSchemaDeclaration::Explicit(older))
            .expect("compatibility evidence");

        assert_eq!(
            report.status(),
            ManifestCompatibilityStatus::Compatible
        );

        assert!(report.is_compatible());
    }

    #[test]
    fn major_version_mismatch_is_incompatible() {
        let checker =
            ManifestCompatibilityChecker::conservative_initial();

        let report = checker
            .check(ManifestSchemaDeclaration::Explicit(
                version(2, 0, 0),
            ))
            .expect("compatibility evidence");

        assert_eq!(
            report.status(),
            ManifestCompatibilityStatus::Incompatible
        );
    }

    #[test]
    fn legacy_bridge_records_assumed_version() {
        let checker =
            ManifestCompatibilityChecker::conservative_initial();

        let report = checker
            .check(ManifestSchemaDeclaration::LegacyUnversioned)
            .expect("legacy compatibility evidence");

        assert_eq!(
            report.status(),
            ManifestCompatibilityStatus::Compatible
        );

        assert!(report.declaration().is_legacy());
        assert!(!report.version_was_explicit());

        assert_eq!(
            report.effective_version(),
            Some(INITIAL_MANIFEST_SCHEMA_VERSION)
        );

        assert!(report.validate().is_ok());
    }

    #[test]
    fn strict_policy_rejects_unversioned_manifest() {
        let checker = ManifestCompatibilityChecker::strict();

        let report = checker
            .check(ManifestSchemaDeclaration::LegacyUnversioned)
            .expect("legacy rejection report");

        assert_eq!(
            report.status(),
            ManifestCompatibilityStatus::LegacyVersionRequired
        );

        assert!(report.blocks_interpretation());
        assert!(report.compatibility_check().is_none());
        assert!(report.into_compatible_result().is_err());
    }

    #[test]
    fn legacy_version_must_match_configured_assumption() {
        let checker =
            ManifestCompatibilityChecker::conservative_initial();

        assert!(
            checker
                .check_version(
                    ManifestSchemaDeclaration::LegacyUnversioned,
                    version(2, 0, 0),
                )
                .is_err()
        );
    }

    #[test]
    fn explicit_declaration_must_match_supplied_version() {
        let checker =
            ManifestCompatibilityChecker::conservative_initial();

        assert!(
            checker
                .check_version(
                    ManifestSchemaDeclaration::Explicit(
                        version(1, 0, 0),
                    ),
                    version(2, 0, 0),
                )
                .is_err()
        );
    }

    #[test]
    fn absent_policy_fails_closed() {
        let mut configuration =
            CompatibilityPolicies::conservative_initial();

        configuration.manifest_schema = None;

        let checker = ManifestCompatibilityChecker::new(
            configuration,
            LegacyManifestPolicy::RequireExplicitVersion,
        )
        .expect("valid checker without manifest policy");

        let report = checker
            .check(ManifestSchemaDeclaration::Explicit(
                version(1, 0, 0),
            ))
            .expect("policy-unavailable report");

        assert_eq!(
            report.status(),
            ManifestCompatibilityStatus::PolicyUnavailable
        );

        assert!(report.blocks_interpretation());
    }

    #[test]
    fn rejects_invalid_version_ranges() {
        let result = manifest_schema_policy(
            version(2, 0, 0),
            version(3, 0, 0),
            version(1, 0, 0),
            false,
        );

        assert!(result.is_err());
    }

    #[test]
    fn rejects_current_version_outside_supported_range() {
        let result = manifest_schema_policy(
            version(3, 0, 0),
            version(1, 0, 0),
            version(2, 0, 0),
            false,
        );

        assert!(result.is_err());
    }

    #[test]
    fn report_status_identifiers_are_stable() {
        assert_eq!(
            ManifestCompatibilityStatus::Compatible.as_str(),
            "compatible"
        );

        assert_eq!(
            ManifestCompatibilityStatus::RequiresMigration.as_str(),
            "requires_migration"
        );

        assert_eq!(
            ManifestCompatibilityStatus::Incompatible.as_str(),
            "incompatible"
        );

        assert_eq!(
            ManifestCompatibilityStatus::PolicyUnavailable.as_str(),
            "policy_unavailable"
        );

        assert_eq!(
            ManifestCompatibilityStatus::LegacyVersionRequired.as_str(),
            "legacy_version_required"
        );
    }

    #[test]
    fn does_not_conflate_manifest_and_language_versions() {
        let manifest_version = version(1, 0, 0);

        assert_eq!(
            manifest_version,
            INITIAL_MANIFEST_SCHEMA_VERSION
        );

        // The manifest version is a distinct Rust type.
        // Language version decisions remain with the language authority.
    }

    #[test]
    fn initial_policy_is_exact_and_conservative() {
        let policy = initial_manifest_schema_policy();

        assert_eq!(
            policy.current(),
            INITIAL_MANIFEST_SCHEMA_VERSION
        );

        assert_eq!(
            policy.supported().minimum(),
            INITIAL_MANIFEST_SCHEMA_VERSION
        );

        assert_eq!(
            policy.supported().maximum(),
            INITIAL_MANIFEST_SCHEMA_VERSION
        );

        assert!(!policy.allows_older_without_migration());
    }
}
