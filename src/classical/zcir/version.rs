
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani Classical Intermediate Representation — Version Contracts
//!
//! Path: `src/classical/zcir/version.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Unsafe Rust is forbidden.
//!
//! ## Purpose
//!
//! Define version identities for ZCIR's logical schema, serialized format,
//! and classical dialect contract. Provide explicit compatibility checks
//! that reuse ZUIR's canonical version compatibility machinery.
//!
//! ## Architectural authority
//!
//! This module conforms to:
//!
//! - `DESIGN.md`
//! - `grammar/DESIGN.md`
//! - `grammar/Zamani.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/specification/poco-reaf.md`
//! - `grammar/validation/semantic-boundaries.md`
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/version.rs`
//!
//! The language specification defines source meaning. ZUIR owns the shared
//! representation and version-compatibility foundations. This module owns
//! only ZCIR-specific version identities and their application.
//!
//! ## Owns
//!
//! - ZCIR schema, encoding-format, and classical-dialect version identities.
//! - Initial ZCIR version constants.
//! - A version manifest for identifying the contracts used by an artifact.
//! - ZCIR-specific entry points for canonical compatibility decisions.
//!
//! ## Does not own
//!
//! - Zamani language-version definitions.
//! - ZUIR schema, format, or dialect version definitions.
//! - Compiler release, build, target, hardware, or runtime versioning.
//! - Serialization, decoding, schema migration, or cache fingerprints.
//! - Language feature availability or semantic validation.
//! - The existence or implementation of a migration path.
//!
//! ## Version separation
//!
//! `ZcirSchemaVersion` identifies the logical ZCIR representation contract.
//! `ZcirFormatVersion` identifies its serialized encoding contract.
//! `DialectVersion` is reused from ZUIR for the classical dialect contract.
//! `LanguageVersion` is reused from ZUIR for source-language version identity.
//!
//! These identities are not interchangeable. A format change need not imply
//! a schema change, and a compiler release does not automatically change
//! either contract.
//!
//! ## Compatibility
//!
//! Version equality alone does not prove that an artifact is valid.
//! Compatibility decisions must be checked before interpreting external data.
//! Historical representations requiring migration must be migrated by an
//! explicitly implemented and tested migration before normal interpretation.
//! Unknown future versions must not be silently accepted.
//!
//! ## Scalability and determinism
//!
//! Version components are finite numeric identifiers, not limits on program
//! size, operations, functions, data, memory, hardware, or execution scale.
//! Version arithmetic uses checked operations. This module has no global
//! mutable state, I/O, hardware discovery, or environment-dependent behavior.
//!
//! ## Integration contract
//!
//! - `zcir/mod.rs`: declare `pub mod version;`.
//! - `zcir/config.rs`: `CONFIG_SCHEMA_VERSION` remains the configuration
//!   schema version and is independent of the ZCIR IR schema.
//! - `zcir/serialization/`: persist schema and format identities separately.
//! - `zcir/compatibility.rs`: apply these decisions and resolve migrations.
//! - `zcir/api.rs`: reject unsupported versions before consuming artifacts.
//! - `zcir/lowering/`: validate version compatibility and semantic preservation.
//! - Incremental compilation: include every result-affecting version identity
//!   in the cache key.
//! - ZUIR integration: reuse its `VersionTriple`, `DialectVersion`,
//!   `LanguageVersion`, `CompatibilityPolicy`, and compatibility decisions.
//!
//! This module intentionally does not depend on ZCIR errors, IR nodes,
//! serializers, builders, or any unfinished ZCIR child module.
//!
//! ## Freeze rule
//!
//! The version constants are initial contract declarations. Changing them
//! requires a reviewed schema, format, or dialect evolution decision and
//! appropriate conformance and compatibility tests.

use core::fmt;

use serde::{Deserialize, Serialize};

use crate::zuir::version::{
    Compatibility, CompatibilityDecision, CompatibilityPolicy,
    CompatibilityReason, DialectVersion, LanguageVersion, VersionTriple,
};

/// Current ZCIR logical-schema version.
///
/// Increment this only when the reviewed logical representation contract
/// changes according to the ZCIR compatibility policy.
///
/// This is not the ZUIR schema version, configuration schema version,
/// serialized-format version, or Zamani language version.
pub const CURRENT_ZCIR_SCHEMA_VERSION: ZcirSchemaVersion =
    ZcirSchemaVersion::new(1, 0, 0);

/// Current ZCIR serialization-format version.
///
/// Evolve this independently from the logical schema when the encoding
/// contract changes without an equivalent logical-schema change.
pub const CURRENT_ZCIR_FORMAT_VERSION: ZcirFormatVersion =
    ZcirFormatVersion::new(1, 0, 0);

/// Initial classical ZCIR dialect contract version.
///
/// This reuses ZUIR's canonical dialect-version identity. It does not assert
/// that every operation or feature in the planned classical dialect has
/// already been implemented.
pub const INITIAL_ZCIR_DIALECT_VERSION: DialectVersion =
    DialectVersion::new(1, 0, 0);

/// A strongly typed ZCIR logical-schema version.
///
/// The wrapper prevents accidental substitution of a ZCIR schema version
/// for a serialized-format version or an unrelated ZUIR version.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize,
)]
#[serde(transparent)]
pub struct ZcirSchemaVersion(VersionTriple);

impl ZcirSchemaVersion {
    /// Construct a schema version from numeric components.
    #[must_use]
    pub const fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self(VersionTriple::new(major, minor, patch))
    }

    /// Construct a schema version from canonical version components.
    #[must_use]
    pub const fn from_components(components: VersionTriple) -> Self {
        Self(components)
    }

    /// Return the major component.
    #[must_use]
    pub const fn major(self) -> u32 {
        self.0.major()
    }

    /// Return the minor component.
    #[must_use]
    pub const fn minor(self) -> u32 {
        self.0.minor()
    }

    /// Return the patch component.
    #[must_use]
    pub const fn patch(self) -> u32 {
        self.0.patch()
    }

    /// Return the canonical version components.
    #[must_use]
    pub const fn components(self) -> VersionTriple {
        self.0
    }

    /// Return the next patch version, or `None` on overflow.
    #[must_use]
    pub const fn checked_next_patch(self) -> Option<Self> {
        match self.0.checked_next_patch() {
            Some(next) => Some(Self(next)),
            None => None,
        }
    }

    /// Return the next minor version with patch reset to zero.
    #[must_use]
    pub const fn checked_next_minor(self) -> Option<Self> {
        match self.0.checked_next_minor() {
            Some(next) => Some(Self(next)),
            None => None,
        }
    }

    /// Return the next major version with minor and patch reset to zero.
    #[must_use]
    pub const fn checked_next_major(self) -> Option<Self> {
        match self.0.checked_next_major() {
            Some(next) => Some(Self(next)),
            None => None,
        }
    }
}

impl fmt::Display for ZcirSchemaVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// A strongly typed ZCIR serialized-format version.
///
/// A schema describes logical representation meaning. A format describes
/// how an artifact is encoded. Both must be checked before decoding and
/// interpreting an external artifact.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize,
)]
#[serde(transparent)]
pub struct ZcirFormatVersion(VersionTriple);

impl ZcirFormatVersion {
    /// Construct a format version from numeric components.
    #[must_use]
    pub const fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self(VersionTriple::new(major, minor, patch))
    }

    /// Construct a format version from canonical version components.
    #[must_use]
    pub const fn from_components(components: VersionTriple) -> Self {
        Self(components)
    }

    /// Return the major component.
    #[must_use]
    pub const fn major(self) -> u32 {
        self.0.major()
    }

    /// Return the minor component.
    #[must_use]
    pub const fn minor(self) -> u32 {
        self.0.minor()
    }

    /// Return the patch component.
    #[must_use]
    pub const fn patch(self) -> u32 {
        self.0.patch()
    }

    /// Return the canonical version components.
    #[must_use]
    pub const fn components(self) -> VersionTriple {
        self.0
    }

    /// Return the next patch version, or `None` on overflow.
    #[must_use]
    pub const fn checked_next_patch(self) -> Option<Self> {
        match self.0.checked_next_patch() {
            Some(next) => Some(Self(next)),
            None => None,
        }
    }

    /// Return the next minor version with patch reset to zero.
    #[must_use]
    pub const fn checked_next_minor(self) -> Option<Self> {
        match self.0.checked_next_minor() {
            Some(next) => Some(Self(next)),
            None => None,
        }
    }

    /// Return the next major version with minor and patch reset to zero.
    #[must_use]
    pub const fn checked_next_major(self) -> Option<Self> {
        match self.0.checked_next_major() {
            Some(next) => Some(Self(next)),
            None => None,
        }
    }
}

impl fmt::Display for ZcirFormatVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// Version identities needed to identify a ZCIR artifact's contracts.
///
/// This is metadata, not the artifact itself. It does not establish that the
/// artifact is structurally valid, that its dialect features are implemented,
/// or that a migration exists.
///
/// The language version is reused from ZUIR to prevent a duplicate
/// Zamani-language version definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ZcirVersionManifest {
    schema: ZcirSchemaVersion,
    format: ZcirFormatVersion,
    dialect: DialectVersion,
    language: LanguageVersion,
}

impl ZcirVersionManifest {
    /// Construct a manifest from explicit version identities.
    #[must_use]
    pub const fn new(
        schema: ZcirSchemaVersion,
        format: ZcirFormatVersion,
        dialect: DialectVersion,
        language: LanguageVersion,
    ) -> Self {
        Self {
            schema,
            format,
            dialect,
            language,
        }
    }

    /// Return the initial ZCIR contract manifest.
    ///
    /// The language version is deliberately supplied by the caller because
    /// this module must not invent the current Zamani language version.
    #[must_use]
    pub const fn initial(language: LanguageVersion) -> Self {
        Self::new(
            CURRENT_ZCIR_SCHEMA_VERSION,
            CURRENT_ZCIR_FORMAT_VERSION,
            INITIAL_ZCIR_DIALECT_VERSION,
            language,
        )
    }

    /// Return the ZCIR schema version.
    #[must_use]
    pub const fn schema(self) -> ZcirSchemaVersion {
        self.schema
    }

    /// Return the serialized-format version.
    #[must_use]
    pub const fn format(self) -> ZcirFormatVersion {
        self.format
    }

    /// Return the classical dialect version.
    #[must_use]
    pub const fn dialect(self) -> DialectVersion {
        self.dialect
    }

    /// Return the Zamani language version.
    #[must_use]
    pub const fn language(self) -> LanguageVersion {
        self.language
    }
}

/// Compatibility results for a ZCIR artifact.
///
/// Schema and format decisions are separate because a reader can support
/// a logical schema but not its encoding, or support an encoding protocol
/// that carries an unsupported logical schema.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ZcirCompatibility {
    schema: CompatibilityDecision,
    format: CompatibilityDecision,
}

impl ZcirCompatibility {
    /// Construct a combined compatibility result.
    #[must_use]
    pub const fn new(
        schema: CompatibilityDecision,
        format: CompatibilityDecision,
    ) -> Self {
        Self { schema, format }
    }

    /// Return the schema compatibility decision.
    #[must_use]
    pub const fn schema(self) -> CompatibilityDecision {
        self.schema
    }

    /// Return the format compatibility decision.
    #[must_use]
    pub const fn format(self) -> CompatibilityDecision {
        self.format
    }

    /// Whether both schema and format are compatible without migration.
    #[must_use]
    pub const fn is_compatible_without_migration(self) -> bool {
        matches!(self.schema.compatibility(), Compatibility::Compatible)
            && matches!(self.format.compatibility(), Compatibility::Compatible)
    }

    /// Whether either version requires migration or must be rejected.
    #[must_use]
    pub const fn requires_migration_or_rejection(self) -> bool {
        !self.is_compatible_without_migration()
    }
}

/// Check a ZCIR schema version against an explicit canonical policy.
///
/// This delegates the decision to ZUIR's canonical compatibility machinery.
/// It does not perform a migration.
#[must_use]
pub const fn check_schema_compatibility(
    source: ZcirSchemaVersion,
    policy: CompatibilityPolicy,
) -> CompatibilityDecision {
    policy.check(source.components())
}

/// Check a ZCIR format version against an explicit canonical policy.
///
/// Call this before decoding an artifact whose format is not already trusted.
/// This function does not decode bytes or establish that the encoding is valid.
#[must_use]
pub const fn check_format_compatibility(
    source: ZcirFormatVersion,
    policy: CompatibilityPolicy,
) -> CompatibilityDecision {
    policy.check(source.components())
}

/// Check schema and format versions independently.
///
/// A successful result means only that both versions are accepted under the
/// supplied policies. The artifact still requires decoding, structural
/// verification, and any other applicable validation.
#[must_use]
pub const fn check_artifact_compatibility(
    manifest: ZcirVersionManifest,
    schema_policy: CompatibilityPolicy,
    format_policy: CompatibilityPolicy,
) -> ZcirCompatibility {
    ZcirCompatibility::new(
        check_schema_compatibility(manifest.schema(), schema_policy),
        check_format_compatibility(manifest.format(), format_policy),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const LANGUAGE: LanguageVersion = LanguageVersion::new(1, 0, 0);

    #[test]
    fn initial_versions_are_explicit_and_independent() {
        assert_eq!(CURRENT_ZCIR_SCHEMA_VERSION.to_string(), "1.0.0");
        assert_eq!(CURRENT_ZCIR_FORMAT_VERSION.to_string(), "1.0.0");
        assert_eq!(INITIAL_ZCIR_DIALECT_VERSION.to_string(), "1.0.0");

        let manifest = ZcirVersionManifest::initial(LANGUAGE);

        assert_eq!(manifest.schema(), CURRENT_ZCIR_SCHEMA_VERSION);
        assert_eq!(manifest.format(), CURRENT_ZCIR_FORMAT_VERSION);
        assert_eq!(manifest.dialect(), INITIAL_ZCIR_DIALECT_VERSION);
        assert_eq!(manifest.language(), LANGUAGE);
    }

    #[test]
    fn schema_and_format_are_distinct_types() {
        let schema = ZcirSchemaVersion::new(1, 2, 3);
        let format = ZcirFormatVersion::new(1, 2, 3);

        assert_eq!(schema.to_string(), format.to_string());
        assert_eq!(schema.components(), format.components());

        // The types deliberately have no implicit conversion between them.
        // Their equal numeric representation does not make their contracts
        // interchangeable.
    }

    #[test]
    fn checked_version_increments_handle_overflow() {
        let schema = ZcirSchemaVersion::new(1, 2, 3);

        assert_eq!(
            schema.checked_next_patch(),
            Some(ZcirSchemaVersion::new(1, 2, 4))
        );
        assert_eq!(
            schema.checked_next_minor(),
            Some(ZcirSchemaVersion::new(1, 3, 0))
        );
        assert_eq!(
            schema.checked_next_major(),
            Some(ZcirSchemaVersion::new(2, 0, 0))
        );

        assert_eq!(
            ZcirSchemaVersion::new(1, 2, u32::MAX).checked_next_patch(),
            None
        );
        assert_eq!(
            ZcirSchemaVersion::new(1, u32::MAX, 7).checked_next_minor(),
            None
        );
        assert_eq!(
            ZcirSchemaVersion::new(u32::MAX, 2, 7).checked_next_major(),
            None
        );
    }

    #[test]
    fn exact_schema_and_format_versions_are_compatible() {
        let schema_policy =
            CompatibilityPolicy::exact(CURRENT_ZCIR_SCHEMA_VERSION.components());
        let format_policy =
            CompatibilityPolicy::exact(CURRENT_ZCIR_FORMAT_VERSION.components());

        let manifest = ZcirVersionManifest::initial(LANGUAGE);
        let result = check_artifact_compatibility(
            manifest,
            schema_policy,
            format_policy,
        );

        assert!(result.is_compatible_without_migration());
        assert!(!result.requires_migration_or_rejection());
        assert_eq!(
            result.schema().reason(),
            CompatibilityReason::ExactMatch
        );
        assert_eq!(
            result.format().reason(),
            CompatibilityReason::ExactMatch
        );
    }

    #[test]
    fn future_schema_is_not_silently_accepted() {
        let future = ZcirSchemaVersion::new(
            CURRENT_ZCIR_SCHEMA_VERSION.major(),
            CURRENT_ZCIR_SCHEMA_VERSION.minor() + 1,
            0,
        );

        let policy =
            CompatibilityPolicy::exact(CURRENT_ZCIR_SCHEMA_VERSION.components());
        let result = check_schema_compatibility(future, policy);

        assert_eq!(result.compatibility(), Compatibility::Incompatible);
        assert_eq!(result.reason(), CompatibilityReason::FutureVersion);
        assert!(result.must_reject_without_migration());
    }

    #[test]
    fn incompatible_major_versions_are_rejected() {
        let source = ZcirFormatVersion::new(2, 0, 0);
        let policy =
            CompatibilityPolicy::exact(CURRENT_ZCIR_FORMAT_VERSION.components());

        let result = check_format_compatibility(source, policy);

        assert_eq!(result.compatibility(), Compatibility::Incompatible);
        assert_eq!(
            result.reason(),
            CompatibilityReason::MajorVersionMismatch
        );
    }

    #[test]
    fn older_versions_require_explicit_migration_under_exact_policy() {
        let source = ZcirSchemaVersion::new(1, 0, 0);
        let current = ZcirSchemaVersion::new(1, 1, 0);
        let policy = CompatibilityPolicy::exact(current.components());

        let result = check_schema_compatibility(source, policy);

        assert_eq!(
            result.compatibility(),
            Compatibility::RequiresMigration
        );
        assert_eq!(
            result.reason(),
            CompatibilityReason::MigrationRequired
        );
        assert!(result.must_reject_without_migration());
    }

    #[test]
    fn separately_supported_older_versions_can_be_accepted_explicitly() {
        let source = ZcirSchemaVersion::new(1, 0, 0);
        let current = ZcirSchemaVersion::new(1, 2, 0);

        let policy = CompatibilityPolicy::new(
            current.components(),
            source.components(),
            true,
        )
        .expect("minimum supported version must not exceed current");

        let result = check_schema_compatibility(source, policy);

        assert_eq!(result.compatibility(), Compatibility::Compatible);
        assert_eq!(
            result.reason(),
            CompatibilityReason::SupportedOlderVersion
        );
    }

    #[test]
    fn invalid_compatibility_range_is_rejected_by_canonical_policy() {
        let current = VersionTriple::new(1, 0, 0);
        let newer_minimum = VersionTriple::new(1, 1, 0);

        assert!(CompatibilityPolicy::new(current, newer_minimum, true).is_none());
    }

    #[test]
    fn schema_compatibility_does_not_hide_format_incompatibility() {
        let manifest = ZcirVersionManifest::new(
            CURRENT_ZCIR_SCHEMA_VERSION,
            ZcirFormatVersion::new(2, 0, 0),
            INITIAL_ZCIR_DIALECT_VERSION,
            LANGUAGE,
        );

        let schema_policy =
            CompatibilityPolicy::exact(CURRENT_ZCIR_SCHEMA_VERSION.components());
        let format_policy =
            CompatibilityPolicy::exact(CURRENT_ZCIR_FORMAT_VERSION.components());

        let result = check_artifact_compatibility(
            manifest,
            schema_policy,
            format_policy,
        );

        assert_eq!(result.schema().compatibility(), Compatibility::Compatible);
        assert_eq!(result.format().compatibility(), Compatibility::Incompatible);
        assert!(!result.is_compatible_without_migration());
    }

    #[test]
    fn manifest_is_deterministic_and_copyable() {
        let first = ZcirVersionManifest::initial(LANGUAGE);
        let second = first;

        assert_eq!(first, second);
        assert_eq!(first.schema(), second.schema());
        assert_eq!(first.format(), second.format());
        assert_eq!(first.dialect(), second.dialect());
        assert_eq!(first.language(), second.language());
    }
}
