
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani Toolchain — Version and Compatibility Contracts
//!
//! Path: src/toolchain/version.rs
//! Edition: Rust 2021
//! Minimum supported Rust: 1.97
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Defines independently versioned contracts used by the Zamani toolchain.
//!
//! The toolchain coordinates project building, dependency management,
//! package distribution, registries, providers and developer APIs.
//!
//! Each externally consumed contract evolves independently.
//!
//! ## OWNERSHIP
//!
//! This module owns:
//!
//! - Toolchain API versions.
//! - Manifest schema versions consumed by the toolchain.
//! - Package archive format versions.
//! - Registry protocol versions.
//! - Toolchain provider API versions.
//! - Task protocol versions.
//! - Inclusive supported-version ranges.
//! - Explicit compatibility decisions.
//! - Strict parsing and canonical display.
//! - Checked version increments.
//!
//! ## DOES NOT OWN
//!
//! This module does not own:
//!
//! - Zamani language versions.
//! - Grammar versions.
//! - Lexer or token versions.
//! - AST versions.
//! - Semantic-model versions.
//! - ZUIR schema or format versions.
//! - Classical, quantum or HDL IR versions.
//! - Compiler release versions.
//! - Backend executable versions.
//! - Hardware firmware versions.
//! - Runtime implementation versions.
//! - Dependency package release semantics.
//! - Artifact serialization.
//! - Schema migration execution.
//! - Target selection or physical resource allocation.
//!
//! ## VERSION AUTHORITIES
//!
//! Canonical generic version representation:
//!     crate::zuir::version
//!
//! Language version:
//!     grammar/specification/language-version.md
//!
//! Cross-layer compatibility:
//!     grammar/compatibility/versions.md
//!
//! External executable versions:
//!     crate::backend::toolchain
//!
//! Project manifest:
//!     crate::zamani_project_config
//!
//! Compiler cache:
//!     crate::compiler::incremental
//!
//! ## IMPORTANT COMPATIBILITY RULE
//!
//! A version number alone does not prove semantic compatibility.
//!
//! Every compatibility decision must identify the contract being checked.
//!
//! Unknown future versions must fail closed.
//!
//! Historical versions requiring migration must not be interpreted before
//! their explicitly supported migration has completed.
//!
//! ## SCALABILITY
//!
//! Version components are u32 because this module reuses the canonical
//! repository version representation.
//!
//! These are metadata identifiers, not computational capacities.
//!
//! They do not impose limits on:
//! - CPUs;
//! - GPUs;
//! - FPGAs;
//! - QPUs;
//! - qubits;
//! - processors;
//! - memory;
//! - tensor rank;
//! - distributed nodes;
//! - workers;
//! - packages;
//! - projects;
//! - computational domains.
//!
//! No literal infinite computation or storage is promised.
//!
//! ## DEPENDENCY CONTRACT
//!
//! This module depends on:
//! - crate::zuir::version;
//! - serde;
//!
//! It does not depend on the compiler, backend, runtime, manifest parser,
//! package manager, registry implementation or hardware subsystem.
//!
//! ## FREEZE CRITERIA
//!
//! - Version categories have distinct Rust types.
//! - Parsing rejects ambiguous representations.
//! - Serialization is deterministic.
//! - Version increments cannot wrap.
//! - Compatibility policies are validated.
//! - Future versions are rejected.
//! - Migration requirements are explicit.
//! - Schema and format versions are independent.
//! - No language or hardware limits are introduced.
//! - All tests pass.

use core::cmp::Ordering;
use core::fmt;
use core::str::FromStr;

use serde::de;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::zuir::version::{
    Compatibility,
    CompatibilityDecision,
    CompatibilityReason,
    VersionIdentity,
    VersionTriple,
};

// ============================================================================
// VERSION PARSING
// ============================================================================

/// Errors encountered while parsing a toolchain contract version.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VersionParseError {
    /// The input is empty.
    Empty,

    /// The input does not contain exactly three components.
    InvalidComponentCount,

    /// A component is empty or contains non-decimal characters.
    InvalidComponent,

    /// A component contains a prohibited leading zero.
    LeadingZero,

    /// A numeric component exceeds u32.
    ComponentOverflow,

    /// The input contains surrounding whitespace.
    SurroundingWhitespace,
}

impl fmt::Display for VersionParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => {
                f.write_str("version must not be empty")
            }
            Self::InvalidComponentCount => {
                f.write_str(
                    "version must contain exactly three components: major.minor.patch",
                )
            }
            Self::InvalidComponent => {
                f.write_str(
                    "version components must contain unsigned decimal digits",
                )
            }
            Self::LeadingZero => {
                f.write_str(
                    "version components must not contain ambiguous leading zeroes",
                )
            }
            Self::ComponentOverflow => {
                f.write_str(
                    "version component exceeds the canonical u32 representation",
                )
            }
            Self::SurroundingWhitespace => {
                f.write_str(
                    "version must not contain surrounding whitespace",
                )
            }
        }
    }
}

impl std::error::Error for VersionParseError {}

/// Parse a canonical stable MAJOR.MINOR.PATCH version.
///
/// Accepted:
///     1.0.0
///     2.14.7
///     0.1.0
///
/// Rejected:
///     1
///     1.2
///     1.2.3.4
///     01.2.3
///     1..3
///     +1.2.3
///     v1.2.3
///     1.2.3-alpha
///
/// Pre-release and build metadata are intentionally not accepted by
/// this stable contract-version representation. They require a separately
/// specified release/distribution contract rather than being silently
/// normalized into a stable version.
fn parse_version(input: &str) -> Result<VersionTriple, VersionParseError> {
    if input.is_empty() {
        return Err(VersionParseError::Empty);
    }

    if input.trim() != input {
        return Err(VersionParseError::SurroundingWhitespace);
    }

    let mut components = input.split('.');

    let major = parse_component(
        components.next().ok_or(
            VersionParseError::InvalidComponentCount,
        )?,
    )?;

    let minor = parse_component(
        components.next().ok_or(
            VersionParseError::InvalidComponentCount,
        )?,
    )?;

    let patch = parse_component(
        components.next().ok_or(
            VersionParseError::InvalidComponentCount,
        )?,
    )?;

    if components.next().is_some() {
        return Err(VersionParseError::InvalidComponentCount);
    }

    Ok(VersionTriple::new(major, minor, patch))
}

/// Parse one canonical unsigned decimal component.
fn parse_component(input: &str) -> Result<u32, VersionParseError> {
    if input.is_empty()
        || !input.bytes().all(|byte| byte.is_ascii_digit())
    {
        return Err(VersionParseError::InvalidComponent);
    }

    if input.len() > 1 && input.starts_with('0') {
        return Err(VersionParseError::LeadingZero);
    }

    input
        .parse::<u32>()
        .map_err(|_| VersionParseError::ComponentOverflow)
}

// ============================================================================
// VERSION TYPE DEFINITIONS
// ============================================================================

/// Define a strongly typed toolchain version category.
///
/// Every category uses the canonical ZUIR VersionTriple representation.
///
/// Different categories cannot be accidentally substituted for one another.
macro_rules! define_toolchain_version {
    (
        $(#[$meta:meta])*
        $name:ident
    ) => {
        $(#[$meta])*
        #[derive(
            Debug,
            Clone,
            Copy,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
        )]
        pub struct $name(VersionTriple);

        impl $name {
            /// Construct a version from numeric components.
            #[must_use]
            pub const fn new(
                major: u32,
                minor: u32,
                patch: u32,
            ) -> Self {
                Self(VersionTriple::new(major, minor, patch))
            }

            /// Construct from the canonical repository representation.
            #[must_use]
            pub const fn from_components(
                components: VersionTriple,
            ) -> Self {
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

            /// Return the canonical numeric representation.
            #[must_use]
            pub const fn components(self) -> VersionTriple {
                self.0
            }

            /// Increment the patch component without wrapping.
            #[must_use]
            pub const fn checked_next_patch(self) -> Option<Self> {
                match self.0.checked_next_patch() {
                    Some(value) => Some(Self(value)),
                    None => None,
                }
            }

            /// Increment the minor component and reset patch.
            #[must_use]
            pub const fn checked_next_minor(self) -> Option<Self> {
                match self.0.checked_next_minor() {
                    Some(value) => Some(Self(value)),
                    None => None,
                }
            }

            /// Increment the major component and reset minor and patch.
            #[must_use]
            pub const fn checked_next_major(self) -> Option<Self> {
                match self.0.checked_next_major() {
                    Some(value) => Some(Self(value)),
                    None => None,
                }
            }

            /// Return the canonical stable version representation.
            #[must_use]
            pub fn canonical(&self) -> String {
                self.to_string()
            }
        }

        impl VersionIdentity for $name {
            fn components(self) -> VersionTriple {
                self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(f)
            }
        }

        impl FromStr for $name {
            type Err = VersionParseError;

            fn from_str(input: &str) -> Result<Self, Self::Err> {
                parse_version(input).map(Self)
            }
        }

        impl Serialize for $name {
            fn serialize<S>(
                &self,
                serializer: S,
            ) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                serializer.serialize_str(&self.to_string())
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D>(
                deserializer: D,
            ) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let input = String::deserialize(deserializer)?;

                input.parse().map_err(de::Error::custom)
            }
        }
    };
}

// ============================================================================
// INDEPENDENT TOOLCHAIN VERSION DOMAINS
// ============================================================================

define_toolchain_version!(
    /// Version of the public Zamani toolchain API contract.
    ///
    /// This describes externally observable toolchain service interfaces.
    ///
    /// It is not the Cargo package version or compiler release version.
    ToolchainApiVersion
);

define_toolchain_version!(
    /// Version of the toolchain-consumed project manifest schema.
    ///
    /// This identifies the manifest structure and interpretation contract.
    ///
    /// It does not replace src/zamani_project_config/.
    ManifestSchemaVersion
);

define_toolchain_version!(
    /// Version of the Zamani package archive format.
    ///
    /// This identifies package container and encoding compatibility.
    ///
    /// It is independent of package release versions and package contents.
    PackageFormatVersion
);

define_toolchain_version!(
    /// Version of the registry communication protocol.
    ///
    /// Different registries may implement different supported protocol
    /// versions through their registered provider contracts.
    RegistryProtocolVersion
);

define_toolchain_version!(
    /// Version of the third-party toolchain provider API.
    ///
    /// This is an integration contract, not a provider implementation
    /// release or executable version.
    ProviderApiVersion
);

define_toolchain_version!(
    /// Version of the toolchain task execution protocol.
    ///
    /// This identifies the externally consumed task request/result contract.
    /// It does not identify a scheduler or worker implementation release.
    TaskProtocolVersion
);

// ============================================================================
// INITIAL CONTRACT IDENTITIES
// ============================================================================
//
// These values establish initial contract identities only.
//
// They do not certify that the corresponding subsystem is implemented
// or production-ready.
//
// Change them only through reviewed compatibility evolution.

/// Initial public toolchain API contract.
pub const INITIAL_TOOLCHAIN_API_VERSION: ToolchainApiVersion =
    ToolchainApiVersion::new(1, 0, 0);

/// Initial project manifest schema contract.
pub const INITIAL_MANIFEST_SCHEMA_VERSION: ManifestSchemaVersion =
    ManifestSchemaVersion::new(1, 0, 0);

/// Initial package archive format contract.
pub const INITIAL_PACKAGE_FORMAT_VERSION: PackageFormatVersion =
    PackageFormatVersion::new(1, 0, 0);

/// Initial registry protocol contract.
pub const INITIAL_REGISTRY_PROTOCOL_VERSION: RegistryProtocolVersion =
    RegistryProtocolVersion::new(1, 0, 0);

/// Initial third-party provider API contract.
pub const INITIAL_PROVIDER_API_VERSION: ProviderApiVersion =
    ProviderApiVersion::new(1, 0, 0);

/// Initial task execution protocol contract.
pub const INITIAL_TASK_PROTOCOL_VERSION: TaskProtocolVersion =
    TaskProtocolVersion::new(1, 0, 0);

// ============================================================================
// VERSION RANGE ERRORS
// ============================================================================

/// Errors encountered while constructing or validating a version range.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VersionRangeError {
    /// The minimum version is newer than the maximum version.
    InvertedRange,
}

impl fmt::Display for VersionRangeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvertedRange => {
                f.write_str(
                    "minimum supported version is newer than maximum supported version",
                )
            }
        }
    }
}

impl std::error::Error for VersionRangeError {}

// ============================================================================
// VERSION RANGE
// ============================================================================

/// An inclusive range of supported versions.
///
/// The generic type prevents mixing unrelated version domains.
///
/// Both boundaries are inclusive.
///
/// This is a support declaration, not a migration guarantee.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub struct VersionRange<V> {
    minimum: V,
    maximum: V,
}

impl<V: VersionIdentity> VersionRange<V> {
    /// Construct a validated inclusive range.
    pub fn new(
        minimum: V,
        maximum: V,
    ) -> Result<Self, VersionRangeError> {
        if minimum > maximum {
            return Err(VersionRangeError::InvertedRange);
        }

        Ok(Self {
            minimum,
            maximum,
        })
    }

    /// Revalidate the range.
    ///
    /// This method is useful at external configuration boundaries.
    pub fn validate(&self) -> Result<(), VersionRangeError> {
        if self.minimum > self.maximum {
            return Err(VersionRangeError::InvertedRange);
        }

        Ok(())
    }

    /// Return the minimum supported version.
    #[must_use]
    pub const fn minimum(&self) -> V {
        self.minimum
    }

    /// Return the maximum supported version.
    #[must_use]
    pub const fn maximum(&self) -> V {
        self.maximum
    }

    /// Determine whether a version belongs to the range.
    #[must_use]
    pub fn contains(&self, version: V) -> bool {
        version >= self.minimum && version <= self.maximum
    }

    /// Determine whether the range contains one exact version.
    #[must_use]
    pub fn is_exact(&self) -> bool {
        self.minimum == self.maximum
    }
}

// Custom deserialization ensures untrusted serialized ranges are validated.
impl<'de, V> Deserialize<'de> for VersionRange<V>
where
    V: VersionIdentity + Deserialize<'de>,
{
    fn deserialize<D>(
        deserializer: D,
    ) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct WireRange<V> {
            minimum: V,
            maximum: V,
        }

        let wire = WireRange::deserialize(deserializer)?;

        Self::new(wire.minimum, wire.maximum)
            .map_err(de::Error::custom)
    }
}

// ============================================================================
// COMPATIBILITY POLICY ERRORS
// ============================================================================

/// Errors encountered while constructing or validating a compatibility policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VersionPolicyError {
    /// The supported version range is invalid.
    InvalidRange,

    /// The current implementation version is outside the supported range.
    CurrentOutsideSupportedRange,
}

impl fmt::Display for VersionPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidRange => {
                f.write_str("the supported version range is invalid")
            }
            Self::CurrentOutsideSupportedRange => {
                f.write_str(
                    "current version is outside the declared supported range",
                )
            }
        }
    }
}

impl std::error::Error for VersionPolicyError {}

// ============================================================================
// VERSION COMPATIBILITY POLICY
// ============================================================================

/// Explicit compatibility policy for one version domain.
///
/// A policy identifies:
///
/// 1. The version currently implemented.
/// 2. The inclusive historical support range.
/// 3. Whether older same-major versions can be interpreted without migration.
///
/// The third property is an explicit compatibility promise and must be
/// backed by conformance tests.
///
/// A range does not automatically make an old representation safe.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub struct VersionPolicy<V> {
    current: V,
    supported: VersionRange<V>,
    allow_older_same_major_without_migration: bool,
}

impl<V: VersionIdentity> VersionPolicy<V> {
    /// Construct a validated compatibility policy.
    pub fn new(
        current: V,
        supported: VersionRange<V>,
        allow_older_same_major_without_migration: bool,
    ) -> Result<Self, VersionPolicyError> {
        supported
            .validate()
            .map_err(|_| VersionPolicyError::InvalidRange)?;

        if !supported.contains(current) {
            return Err(
                VersionPolicyError::CurrentOutsideSupportedRange,
            );
        }

        Ok(Self {
            current,
            supported,
            allow_older_same_major_without_migration,
        })
    }

    /// Revalidate a policy obtained from an external boundary.
    pub fn validate(&self) -> Result<(), VersionPolicyError> {
        self.supported
            .validate()
            .map_err(|_| VersionPolicyError::InvalidRange)?;

        if !self.supported.contains(self.current) {
            return Err(
                VersionPolicyError::CurrentOutsideSupportedRange,
            );
        }

        Ok(())
    }

    /// Return the currently implemented version.
    #[must_use]
    pub const fn current(&self) -> V {
        self.current
    }

    /// Return the supported historical range.
    #[must_use]
    pub const fn supported(&self) -> &VersionRange<V> {
        &self.supported
    }

    /// Return the explicit older-version compatibility promise.
    #[must_use]
    pub const fn allows_older_without_migration(&self) -> bool {
        self.allow_older_same_major_without_migration
    }

    /// Evaluate one supplied version.
    ///
    /// This operation never performs a migration.
    ///
    /// Incompatible and migration-required representations must not be
    /// interpreted by a consumer as though they were current.
    #[must_use]
    pub fn check(&self, supplied: V) -> CompatibilityDecision {
        let source = supplied.components();
        let target = self.current.components();

        if supplied.major() != self.current.major() {
            return CompatibilityDecision::new(
                Compatibility::Incompatible,
                CompatibilityReason::MajorVersionMismatch,
                source,
                target,
            );
        }

        if supplied == self.current {
            return CompatibilityDecision::new(
                Compatibility::Compatible,
                CompatibilityReason::ExactMatch,
                source,
                target,
            );
        }

        if supplied > self.current {
            return CompatibilityDecision::new(
                Compatibility::Incompatible,
                CompatibilityReason::FutureVersion,
                source,
                target,
            );
        }

        if !self.supported.contains(supplied) {
            return CompatibilityDecision::new(
                Compatibility::Incompatible,
                CompatibilityReason::OutsideAllowedRange,
                source,
                target,
            );
        }

        if self.allow_older_same_major_without_migration {
            return CompatibilityDecision::new(
                Compatibility::Compatible,
                CompatibilityReason::SupportedOlderVersion,
                source,
                target,
            );
        }

        CompatibilityDecision::new(
            Compatibility::RequiresMigration,
            CompatibilityReason::MigrationRequired,
            source,
            target,
        )
    }
}

// Custom deserialization prevents invalid policy construction through Serde.
impl<'de, V> Deserialize<'de> for VersionPolicy<V>
where
    V: VersionIdentity + Deserialize<'de>,
{
    fn deserialize<D>(
        deserializer: D,
    ) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct WirePolicy<V> {
            current: V,
            supported: VersionRange<V>,
            allow_older_same_major_without_migration: bool,
        }

        let wire = WirePolicy::deserialize(deserializer)?;

        Self::new(
            wire.current,
            wire.supported,
            wire.allow_older_same_major_without_migration,
        )
        .map_err(de::Error::custom)
    }
}

// ============================================================================
// COMPATIBILITY HELPERS
// ============================================================================

/// Determine whether a compatibility decision permits interpretation
/// without migration.
#[must_use]
pub const fn is_compatible(
    decision: CompatibilityDecision,
) -> bool {
    matches!(
        decision.compatibility(),
        Compatibility::Compatible
    )
}

/// Determine whether an explicit migration is required.
#[must_use]
pub const fn requires_migration(
    decision: CompatibilityDecision,
) -> bool {
    matches!(
        decision.compatibility(),
        Compatibility::RequiresMigration
    )
}

/// Determine whether a version must be rejected.
#[must_use]
pub const fn is_incompatible(
    decision: CompatibilityDecision,
) -> bool {
    matches!(
        decision.compatibility(),
        Compatibility::Incompatible
    )
}

/// Compare two version identities numerically.
///
/// The caller is responsible for ensuring both identities represent
/// the same version category before using the result for policy decisions.
#[must_use]
pub fn compare_versions<A, B>(
    left: A,
    right: B,
) -> Ordering
where
    A: VersionIdentity,
    B: VersionIdentity,
{
    left.components().cmp(&right.components())
}

// ============================================================================
// INITIAL DEFAULT POLICIES
// ============================================================================
//
// These policies accept the initial version only.
//
// Historical compatibility must be introduced only after the relevant
// readers and migration paths have been implemented and tested.
//
// A subsystem must not advertise support merely because its version is
// numerically within a range.

/// Conservative initial toolchain API policy.
pub fn initial_toolchain_api_policy(
) -> VersionPolicy<ToolchainApiVersion> {
    VersionPolicy::new(
        INITIAL_TOOLCHAIN_API_VERSION,
        VersionRange::new(
            INITIAL_TOOLCHAIN_API_VERSION,
            INITIAL_TOOLCHAIN_API_VERSION,
        )
        .expect("an exact version range is valid"),
        false,
    )
    .expect("the current version belongs to its exact range")
}

/// Conservative initial manifest schema policy.
pub fn initial_manifest_schema_policy(
) -> VersionPolicy<ManifestSchemaVersion> {
    VersionPolicy::new(
        INITIAL_MANIFEST_SCHEMA_VERSION,
        VersionRange::new(
            INITIAL_MANIFEST_SCHEMA_VERSION,
            INITIAL_MANIFEST_SCHEMA_VERSION,
        )
        .expect("an exact version range is valid"),
        false,
    )
    .expect("the current version belongs to its exact range")
}

/// Conservative initial package format policy.
pub fn initial_package_format_policy(
) -> VersionPolicy<PackageFormatVersion> {
    VersionPolicy::new(
        INITIAL_PACKAGE_FORMAT_VERSION,
        VersionRange::new(
            INITIAL_PACKAGE_FORMAT_VERSION,
            INITIAL_PACKAGE_FORMAT_VERSION,
        )
        .expect("an exact version range is valid"),
        false,
    )
    .expect("the current version belongs to its exact range")
}

/// Conservative initial registry protocol policy.
pub fn initial_registry_protocol_policy(
) -> VersionPolicy<RegistryProtocolVersion> {
    VersionPolicy::new(
        INITIAL_REGISTRY_PROTOCOL_VERSION,
        VersionRange::new(
            INITIAL_REGISTRY_PROTOCOL_VERSION,
            INITIAL_REGISTRY_PROTOCOL_VERSION,
        )
        .expect("an exact version range is valid"),
        false,
    )
    .expect("the current version belongs to its exact range")
}

/// Conservative initial provider API policy.
pub fn initial_provider_api_policy(
) -> VersionPolicy<ProviderApiVersion> {
    VersionPolicy::new(
        INITIAL_PROVIDER_API_VERSION,
        VersionRange::new(
            INITIAL_PROVIDER_API_VERSION,
            INITIAL_PROVIDER_API_VERSION,
        )
        .expect("an exact version range is valid"),
        false,
    )
    .expect("the current version belongs to its exact range")
}

/// Conservative initial task protocol policy.
pub fn initial_task_protocol_policy(
) -> VersionPolicy<TaskProtocolVersion> {
    VersionPolicy::new(
        INITIAL_TASK_PROTOCOL_VERSION,
        VersionRange::new(
            INITIAL_TASK_PROTOCOL_VERSION,
            INITIAL_TASK_PROTOCOL_VERSION,
        )
        .expect("an exact version range is valid"),
        false,
    )
    .expect("the current version belongs to its exact range")
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ------------------------------------------------------------------------
    // Parsing
    // ------------------------------------------------------------------------

    #[test]
    fn parses_canonical_versions() {
        let version: ToolchainApiVersion =
            "1.2.3".parse().expect("valid version");

        assert_eq!(version.major(), 1);
        assert_eq!(version.minor(), 2);
        assert_eq!(version.patch(), 3);
        assert_eq!(version.to_string(), "1.2.3");
    }

    #[test]
    fn rejects_empty_versions() {
        assert_eq!(
            "".parse::<ToolchainApiVersion>(),
            Err(VersionParseError::Empty)
        );
    }

    #[test]
    fn rejects_incomplete_versions() {
        assert_eq!(
            "1.2".parse::<ToolchainApiVersion>(),
            Err(VersionParseError::InvalidComponentCount)
        );
    }

    #[test]
    fn rejects_extra_components() {
        assert_eq!(
            "1.2.3.4".parse::<ToolchainApiVersion>(),
            Err(VersionParseError::InvalidComponentCount)
        );
    }

    #[test]
    fn rejects_leading_zeroes() {
        assert_eq!(
            "01.2.3".parse::<ToolchainApiVersion>(),
            Err(VersionParseError::LeadingZero)
        );
    }

    #[test]
    fn rejects_invalid_numeric_components() {
        assert_eq!(
            "1.x.3".parse::<ToolchainApiVersion>(),
            Err(VersionParseError::InvalidComponent)
        );
    }

    #[test]
    fn rejects_version_component_overflow() {
        assert_eq!(
            "4294967296.0.0".parse::<ToolchainApiVersion>(),
            Err(VersionParseError::ComponentOverflow)
        );
    }

    #[test]
    fn rejects_prerelease_and_build_metadata() {
        assert!(
            "1.0.0-alpha"
                .parse::<ToolchainApiVersion>()
                .is_err()
        );

        assert!(
            "1.0.0+build"
                .parse::<ToolchainApiVersion>()
                .is_err()
        );
    }

    #[test]
    fn rejects_surrounding_whitespace() {
        assert_eq!(
            " 1.0.0".parse::<ToolchainApiVersion>(),
            Err(VersionParseError::SurroundingWhitespace)
        );

        assert_eq!(
            "1.0.0 ".parse::<ToolchainApiVersion>(),
            Err(VersionParseError::SurroundingWhitespace)
        );
    }

    // ------------------------------------------------------------------------
    // Independent version identities
    // ------------------------------------------------------------------------

    #[test]
    fn version_domains_are_independent_types() {
        let api = ToolchainApiVersion::new(1, 0, 0);
        let package = PackageFormatVersion::new(1, 0, 0);

        assert_eq!(api.to_string(), package.to_string());
        assert_eq!(api.components(), package.components());

        // The types remain distinct despite equal numeric components.
        assert_eq!(api.major(), package.major());
    }

    #[test]
    fn version_domains_parse_independently() {
        let api: ToolchainApiVersion =
            "1.0.0".parse().expect("valid API version");

        let package: PackageFormatVersion =
            "1.0.0".parse().expect("valid package version");

        assert_eq!(api.to_string(), "1.0.0");
        assert_eq!(package.to_string(), "1.0.0");
    }

    // ------------------------------------------------------------------------
    // Version arithmetic
    // ------------------------------------------------------------------------

    #[test]
    fn checked_patch_increment() {
        let version = ToolchainApiVersion::new(1, 2, 3);

        assert_eq!(
            version.checked_next_patch(),
            Some(ToolchainApiVersion::new(1, 2, 4))
        );
    }

    #[test]
    fn checked_minor_increment_resets_patch() {
        let version = ToolchainApiVersion::new(1, 2, 9);

        assert_eq!(
            version.checked_next_minor(),
            Some(ToolchainApiVersion::new(1, 3, 0))
        );
    }

    #[test]
    fn checked_major_increment_resets_minor_and_patch() {
        let version = ToolchainApiVersion::new(1, 9, 9);

        assert_eq!(
            version.checked_next_major(),
            Some(ToolchainApiVersion::new(2, 0, 0))
        );
    }

    #[test]
    fn checked_increment_does_not_wrap() {
        let version =
            ToolchainApiVersion::new(1, 2, u32::MAX);

        assert_eq!(version.checked_next_patch(), None);
    }

    // ------------------------------------------------------------------------
    // Version ranges
    // ------------------------------------------------------------------------

    #[test]
    fn creates_valid_version_range() {
        let range = VersionRange::new(
            ToolchainApiVersion::new(1, 0, 0),
            ToolchainApiVersion::new(2, 0, 0),
        )
        .expect("valid range");

        assert!(range.contains(ToolchainApiVersion::new(1, 0, 0)));
        assert!(range.contains(ToolchainApiVersion::new(1, 5, 0)));
        assert!(range.contains(ToolchainApiVersion::new(2, 0, 0)));

        assert!(!range.contains(ToolchainApiVersion::new(2, 0, 1)));
        assert!(!range.contains(ToolchainApiVersion::new(0, 9, 9)));
    }

    #[test]
    fn rejects_inverted_version_range() {
        let result = VersionRange::new(
            ToolchainApiVersion::new(2, 0, 0),
            ToolchainApiVersion::new(1, 0, 0),
        );

        assert_eq!(
            result,
            Err(VersionRangeError::InvertedRange)
        );
    }

    #[test]
    fn exact_range_contains_one_version() {
        let version = ToolchainApiVersion::new(1, 0, 0);

        let range = VersionRange::new(version, version)
            .expect("valid exact range");

        assert!(range.is_exact());
        assert!(range.contains(version));
    }

    // ------------------------------------------------------------------------
    // Compatibility
    // ------------------------------------------------------------------------

    fn historical_policy() -> VersionPolicy<ToolchainApiVersion> {
        let current = ToolchainApiVersion::new(2, 0, 0);

        let supported = VersionRange::new(
            ToolchainApiVersion::new(1, 0, 0),
            current,
        )
        .expect("valid range");

        VersionPolicy::new(current, supported, false)
            .expect("valid policy")
    }

    #[test]
    fn exact_version_is_compatible() {
        let policy = historical_policy();

        let decision = policy.check(
            ToolchainApiVersion::new(2, 0, 0),
        );

        assert!(is_compatible(decision));
    }

    #[test]
    fn historical_version_requires_migration_by_default() {
        let policy = historical_policy();

        let decision = policy.check(
            ToolchainApiVersion::new(1, 5, 0),
        );

        assert!(requires_migration(decision));
    }

    #[test]
    fn explicitly_supported_historical_version_can_be_compatible() {
        let current = ToolchainApiVersion::new(2, 0, 0);

        let supported = VersionRange::new(
            ToolchainApiVersion::new(1, 0, 0),
            current,
        )
        .expect("valid range");

        let policy = VersionPolicy::new(
            current,
            supported,
            true,
        )
        .expect("valid policy");

        let decision = policy.check(
            ToolchainApiVersion::new(1, 5, 0),
        );

        assert!(is_compatible(decision));
    }

    #[test]
    fn future_versions_are_rejected() {
        let policy = historical_policy();

        let decision = policy.check(
            ToolchainApiVersion::new(3, 0, 0),
        );

        assert!(is_incompatible(decision));

        assert_eq!(
            decision.reason(),
            CompatibilityReason::MajorVersionMismatch
        );
    }

    #[test]
    fn future_same_major_versions_are_rejected() {
        let policy = historical_policy();

        let decision = policy.check(
            ToolchainApiVersion::new(2, 1, 0),
        );

        assert!(is_incompatible(decision));

        assert_eq!(
            decision.reason(),
            CompatibilityReason::FutureVersion
        );
    }

    #[test]
    fn versions_outside_historical_range_are_rejected() {
        let policy = historical_policy();

        let decision = policy.check(
            ToolchainApiVersion::new(0, 9, 0),
        );

        assert!(is_incompatible(decision));
    }

    #[test]
    fn invalid_policy_current_version_is_rejected() {
        let supported = VersionRange::new(
            ToolchainApiVersion::new(1, 0, 0),
            ToolchainApiVersion::new(1, 5, 0),
        )
        .expect("valid range");

        assert_eq!(
            VersionPolicy::new(
                ToolchainApiVersion::new(2, 0, 0),
                supported,
                false,
            ),
            Err(VersionPolicyError::CurrentOutsideSupportedRange)
        );
    }

    // ------------------------------------------------------------------------
    // Serialization
    // ------------------------------------------------------------------------

    #[test]
    fn versions_serialize_as_canonical_strings() {
        let version = ToolchainApiVersion::new(1, 2, 3);

        let encoded =
            serde_json::to_string(&version).expect("serialize");

        assert_eq!(encoded, "\"1.2.3\"");
    }

    #[test]
    fn versions_deserialize_from_canonical_strings() {
        let version: ToolchainApiVersion =
            serde_json::from_str("\"1.2.3\"")
                .expect("deserialize");

        assert_eq!(version, ToolchainApiVersion::new(1, 2, 3));
    }

    #[test]
    fn deserialization_rejects_noncanonical_versions() {
        assert!(
            serde_json::from_str::<ToolchainApiVersion>(
                "\"01.2.3\""
            )
            .is_err()
        );

        assert!(
            serde_json::from_str::<ToolchainApiVersion>(
                "\"1.2\""
            )
            .is_err()
        );

        assert!(
            serde_json::from_str::<ToolchainApiVersion>(
                "\"1.2.3-alpha\""
            )
            .is_err()
        );
    }

    #[test]
    fn version_range_serialization_round_trips() {
        let original = VersionRange::new(
            ToolchainApiVersion::new(1, 0, 0),
            ToolchainApiVersion::new(2, 0, 0),
        )
        .expect("valid range");

        let encoded =
            serde_json::to_string(&original).expect("serialize");

        let decoded: VersionRange<ToolchainApiVersion> =
            serde_json::from_str(&encoded)
                .expect("deserialize");

        assert_eq!(original, decoded);
    }

    #[test]
    fn version_range_deserialization_rejects_inverted_range() {
        let encoded = r#"{
            "minimum": "2.0.0",
            "maximum": "1.0.0"
        }"#;

        assert!(
            serde_json::from_str::<VersionRange<ToolchainApiVersion>>(
                encoded
            )
            .is_err()
        );
    }

    #[test]
    fn version_policy_serialization_round_trips() {
        let original = historical_policy();

        let encoded =
            serde_json::to_string(&original).expect("serialize");

        let decoded: VersionPolicy<ToolchainApiVersion> =
            serde_json::from_str(&encoded)
                .expect("deserialize");

        assert_eq!(original, decoded);
    }

    #[test]
    fn version_policy_deserialization_rejects_invalid_current() {
        let encoded = r#"{
            "current": "3.0.0",
            "supported": {
                "minimum": "1.0.0",
                "maximum": "2.0.0"
            },
            "allow_older_same_major_without_migration": false
        }"#;

        assert!(
            serde_json::from_str::<VersionPolicy<ToolchainApiVersion>>(
                encoded
            )
            .is_err()
        );
    }

    // ------------------------------------------------------------------------
    // Initial contract declarations
    // ------------------------------------------------------------------------

    #[test]
    fn initial_contracts_have_independent_identities() {
        assert_eq!(
            INITIAL_TOOLCHAIN_API_VERSION.to_string(),
            "1.0.0"
        );

        assert_eq!(
            INITIAL_MANIFEST_SCHEMA_VERSION.to_string(),
            "1.0.0"
        );

        assert_eq!(
            INITIAL_PACKAGE_FORMAT_VERSION.to_string(),
            "1.0.0"
        );

        assert_eq!(
            INITIAL_REGISTRY_PROTOCOL_VERSION.to_string(),
            "1.0.0"
        );

        assert_eq!(
            INITIAL_PROVIDER_API_VERSION.to_string(),
            "1.0.0"
        );

        assert_eq!(
            INITIAL_TASK_PROTOCOL_VERSION.to_string(),
            "1.0.0"
        );
    }

    #[test]
    fn initial_policies_are_conservative() {
        assert_eq!(
            initial_toolchain_api_policy()
                .current(),
            INITIAL_TOOLCHAIN_API_VERSION
        );

        assert!(
            !initial_toolchain_api_policy()
                .allows_older_without_migration()
        );

        assert!(
            !initial_manifest_schema_policy()
                .allows_older_without_migration()
        );

        assert!(
            !initial_package_format_policy()
                .allows_older_without_migration()
        );

        assert!(
            !initial_registry_protocol_policy()
                .allows_older_without_migration()
        );

        assert!(
            !initial_provider_api_policy()
                .allows_older_without_migration()
        );

        assert!(
            !initial_task_protocol_policy()
                .allows_older_without_migration()
        );
    }
}
