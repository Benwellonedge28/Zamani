//! # Zamani Universal Intermediate Representation — Version Contracts
//!
//! Path: `src/zuir/version.rs`
//! Rust edition: 2021
//! Minimum supported Rust version: 1.97
//! Safety: Safe Rust only; unsafe code is forbidden.
//!
//! ## PURPOSE
//!
//! Define version identities and compatibility decisions for ZUIR without
//! depending on IR nodes, builders, dialect registries, serialization
//! implementations, compiler backends, or target hardware.
//!
//! ## OWNS
//!
//! - Numeric version representation.
//! - ZUIR schema and serialization format version identities.
//! - Dialect and source-language version identities.
//! - Explicit version compatibility decisions.
//! - Supported-version ranges and migration requirements.
//!
//! ## DOES NOT OWN
//!
//! - Compiler release versioning.
//! - Quantum IR versioning.
//! - Hardware, device, calibration, or runtime versions.
//! - Actual serialization or schema migration implementations.
//! - Language syntax or semantic feature acceptance.
//! - Content fingerprints or build-cache identities.
//!
//! ## DEPENDENCY CONTRACT
//!
//! This module depends only on the Rust standard library and Serde.
//! It does not depend on `ids.rs`, `errors.rs`, or any concrete IR module.
//!
//! ## COMPATIBILITY CONTRACT
//!
//! Version numbers alone do not prove semantic compatibility.
//! Callers must explicitly select a compatibility policy and check the
//! version before interpreting persisted or imported representation data.
//!
//! A version with a different major number is not automatically compatible.
//! A future version must never be silently interpreted as a supported one.
//!
//! ## SCALABILITY CONTRACT
//!
//! Version components use `u32`. This is a version-number representation,
//! not a hardware or computation-size limit. Version increments must use
//! checked arithmetic when computed dynamically.
//!
//! ## FREEZE CRITERIA
//!
//! - Version concepts have distinct Rust types.
//! - Version values are deterministic and serializable.
//! - Compatibility is an explicit decision.
//! - Unsupported future versions are not silently accepted.
//! - No hardware-specific assumptions or unsafe code are introduced.
//!
//! IMPORTANT:
//! The initial CURRENT_* constants below establish the initial ZUIR schema
//! and encoding contracts. They must be changed only through a deliberate,
//! reviewed schema or format evolution—not as a side effect of a compiler
//! release or unrelated domain extension.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::cmp::Ordering;
use core::fmt;

use serde::{Deserialize, Serialize};

/// A validated numeric version triple.
///
/// This type deliberately has no prerelease or build-metadata fields.
/// If ZUIR later needs prerelease identifiers, their grammar and ordering
/// rules must be specified explicitly rather than silently approximated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct VersionTriple {
    major: u32,
    minor: u32,
    patch: u32,
}

impl VersionTriple {
    /// Constructs a version triple.
    #[must_use]
    pub const fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Returns the major component.
    #[must_use]
    pub const fn major(self) -> u32 {
        self.major
    }

    /// Returns the minor component.
    #[must_use]
    pub const fn minor(self) -> u32 {
        self.minor
    }

    /// Returns the patch component.
    #[must_use]
    pub const fn patch(self) -> u32 {
        self.patch
    }

    /// Returns the next patch version, or `None` on overflow.
    #[must_use]
    pub const fn checked_next_patch(self) -> Option<Self> {
        match self.patch.checked_add(1) {
            Some(patch) => Some(Self::new(self.major, self.minor, patch)),
            None => None,
        }
    }

    /// Returns the next minor version, resetting patch to zero.
    ///
    /// Returns `None` if the minor component cannot be incremented.
    #[must_use]
    pub const fn checked_next_minor(self) -> Option<Self> {
        match self.minor.checked_add(1) {
            Some(minor) => Some(Self::new(self.major, minor, 0)),
            None => None,
        }
    }

    /// Returns the next major version, resetting minor and patch to zero.
    ///
    /// Returns `None` if the major component cannot be incremented.
    #[must_use]
    pub const fn checked_next_major(self) -> Option<Self> {
        match self.major.checked_add(1) {
            Some(major) => Some(Self::new(major, 0, 0)),
            None => None,
        }
    }
}

impl fmt::Display for VersionTriple {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

/// Common access to version components.
///
/// Implemented only by the version categories defined in this module.
/// Consumers should prefer the concrete type where a particular version
/// category is required.
pub trait VersionIdentity:
    Copy + Eq + Ord + fmt::Debug + fmt::Display + Send + Sync + 'static
{
    /// Returns the numeric version triple.
    fn components(self) -> VersionTriple;
}

/// Defines a strongly typed version category.
///
/// The wrappers intentionally do not implement conversions between version
/// categories. A dialect version cannot accidentally be passed as a ZUIR
/// schema version.
macro_rules! define_version_type {
    ($(#[$meta:meta])* $name:ident) => {
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
            Serialize,
            Deserialize,
        )]
        #[serde(transparent)]
        pub struct $name(VersionTriple);

        impl $name {
            /// Creates a version from its major, minor, and patch components.
            #[must_use]
            pub const fn new(major: u32, minor: u32, patch: u32) -> Self {
                Self(VersionTriple::new(major, minor, patch))
            }

            /// Creates a version from an existing version triple.
            #[must_use]
            pub const fn from_components(components: VersionTriple) -> Self {
                Self(components)
            }

            /// Returns the major component.
            #[must_use]
            pub const fn major(self) -> u32 {
                self.0.major()
            }

            /// Returns the minor component.
            #[must_use]
            pub const fn minor(self) -> u32 {
                self.0.minor()
            }

            /// Returns the patch component.
            #[must_use]
            pub const fn patch(self) -> u32 {
                self.0.patch()
            }

            /// Returns the underlying numeric version triple.
            #[must_use]
            pub const fn components(self) -> VersionTriple {
                self.0
            }

            /// Returns the next patch version, or `None` on overflow.
            #[must_use]
            pub const fn checked_next_patch(self) -> Option<Self> {
                match self.0.checked_next_patch() {
                    Some(value) => Some(Self(value)),
                    None => None,
                }
            }

            /// Returns the next minor version, or `None` on overflow.
            #[must_use]
            pub const fn checked_next_minor(self) -> Option<Self> {
                match self.0.checked_next_minor() {
                    Some(value) => Some(Self(value)),
                    None => None,
                }
            }

            /// Returns the next major version, or `None` on overflow.
            #[must_use]
            pub const fn checked_next_major(self) -> Option<Self> {
                match self.0.checked_next_major() {
                    Some(value) => Some(Self(value)),
                    None => None,
                }
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
    };
}

define_version_type!(
    /// Version of the logical ZUIR schema and semantic representation contract.
    ///
    /// This is independent of the serialization encoding and compiler release.
    ZuirSchemaVersion
);

define_version_type!(
    /// Version of the serialized ZUIR encoding protocol.
    ///
    /// Two encodings may represent the same schema while using different
    /// serialization format versions.
    ZuirFormatVersion
);

define_version_type!(
    /// Version of a dialect's declared semantic contract.
    ///
    /// This is not a hardware or backend version.
    DialectVersion
);

define_version_type!(
    /// Version of the Zamani language specification.
    ///
    /// This value does not establish whether a particular syntax feature is
    /// implemented or accepted by the compiler.
    LanguageVersion
);

/// Initial ZUIR schema contract.
///
/// Increment this when the logical schema or its normative interpretation
/// changes according to the repository's compatibility policy.
pub const CURRENT_ZUIR_SCHEMA_VERSION: ZuirSchemaVersion =
    ZuirSchemaVersion::new(1, 0, 0);

/// Initial ZUIR serialization protocol.
///
/// Change this independently of the schema version when the encoding
/// contract changes without an equivalent schema change.
pub const CURRENT_ZUIR_FORMAT_VERSION: ZuirFormatVersion =
    ZuirFormatVersion::new(1, 0, 0);

/// Initial declared dialect contract version.
///
/// Individual dialects retain ownership of their actual versions. This
/// constant is a baseline, not a forced version for every dialect.
pub const INITIAL_DIALECT_VERSION: DialectVersion =
    DialectVersion::new(1, 0, 0);

/// Describes the compatibility result of comparing two versions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Compatibility {
    /// The representation can be interpreted under the selected policy
    /// without a schema migration.
    Compatible,

    /// The representation is within the supported historical range but
    /// requires an explicit migration before normal interpretation.
    RequiresMigration,

    /// The representation cannot safely be interpreted under this policy.
    Incompatible,
}

/// Why a compatibility check produced its result.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CompatibilityReason {
    /// The supplied version exactly matches the current version.
    ExactMatch,

    /// The version is an older supported version in the same major line.
    SupportedOlderVersion,

    /// The version is older than the current version and needs migration.
    MigrationRequired,

    /// The supplied version is newer than the current version.
    FutureVersion,

    /// The major component differs from the current version.
    MajorVersionMismatch,

    /// The version is older than the oldest version the reader supports.
    BelowMinimumSupported,

    /// The version is outside a caller-selected allowed range.
    OutsideAllowedRange,
}

/// A complete, explicit compatibility decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CompatibilityDecision {
    compatibility: Compatibility,
    reason: CompatibilityReason,
    source: VersionTriple,
    target: VersionTriple,
}

impl CompatibilityDecision {
    /// Creates a compatibility decision.
    #[must_use]
    pub const fn new(
        compatibility: Compatibility,
        reason: CompatibilityReason,
        source: VersionTriple,
        target: VersionTriple,
    ) -> Self {
        Self {
            compatibility,
            reason,
            source,
            target,
        }
    }

    /// Returns the decision category.
    #[must_use]
    pub const fn compatibility(self) -> Compatibility {
        self.compatibility
    }

    /// Returns the reason for the decision.
    #[must_use]
    pub const fn reason(self) -> CompatibilityReason {
        self.reason
    }

    /// Returns the version being examined.
    #[must_use]
    pub const fn source(self) -> VersionTriple {
        self.source
    }

    /// Returns the version against which it was examined.
    #[must_use]
    pub const fn target(self) -> VersionTriple {
        self.target
    }

    /// Returns `true` only when no migration is required.
    #[must_use]
    pub const fn is_compatible_without_migration(self) -> bool {
        matches!(self.compatibility, Compatibility::Compatible)
    }

    /// Returns `true` when interpretation is unsafe without migration.
    #[must_use]
    pub const fn must_reject_without_migration(self) -> bool {
        !matches!(self.compatibility, Compatibility::Compatible)
    }
}

/// Compatibility policy for versioned schemas and serialized formats.
///
/// `current` is the version this reader implements.
/// `minimum_supported` is the oldest version it can recognize under this
/// policy. Versions between these boundaries are not automatically safe:
/// callers must still use the decision to select a migration or reject input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CompatibilityPolicy {
    current: VersionTriple,
    minimum_supported: VersionTriple,
    allow_same_major_older_versions_without_migration: bool,
}

impl CompatibilityPolicy {
    /// Creates a compatibility policy.
    ///
    /// Returns `None` if the minimum supported version is newer than current.
    #[must_use]
    pub const fn new(
        current: VersionTriple,
        minimum_supported: VersionTriple,
        allow_same_major_older_versions_without_migration: bool,
    ) -> Option<Self> {
        if minimum_supported.major > current.major
            || (minimum_supported.major == current.major
                && minimum_supported.minor > current.minor)
            || (minimum_supported.major == current.major
                && minimum_supported.minor == current.minor
                && minimum_supported.patch > current.patch)
        {
            return None;
        }

        Some(Self {
            current,
            minimum_supported,
            allow_same_major_older_versions_without_migration,
        })
    }

    /// Returns a conservative policy that accepts only the current version
    /// without migration.
    #[must_use]
    pub const fn exact(current: VersionTriple) -> Self {
        Self {
            current,
            minimum_supported: current,
            allow_same_major_older_versions_without_migration: false,
        }
    }

    /// Returns the current version.
    #[must_use]
    pub const fn current(self) -> VersionTriple {
        self.current
    }

    /// Returns the oldest version supported by this policy.
    #[must_use]
    pub const fn minimum_supported(self) -> VersionTriple {
        self.minimum_supported
    }

    /// Checks a version against this policy.
    ///
    /// This function makes no assumptions about whether a migration exists.
    /// A `RequiresMigration` result means the caller must find and run an
    /// explicit migration before interpreting the representation.
    #[must_use]
    pub const fn check(self, source: VersionTriple) -> CompatibilityDecision {
        let target = self.current;

        if source.major != target.major {
            return CompatibilityDecision::new(
                Compatibility::Incompatible,
                CompatibilityReason::MajorVersionMismatch,
                source,
                target,
            );
        }

        if source.major == target.major
            && source.minor == target.minor
            && source.patch == target.patch
        {
            return CompatibilityDecision::new(
                Compatibility::Compatible,
                CompatibilityReason::ExactMatch,
                source,
                target,
            );
        }

        if source > target {
            return CompatibilityDecision::new(
                Compatibility::Incompatible,
                CompatibilityReason::FutureVersion,
                source,
                target,
            );
        }

        if source < self.minimum_supported {
            return CompatibilityDecision::new(
                Compatibility::Incompatible,
                CompatibilityReason::BelowMinimumSupported,
                source,
                target,
            );
        }

        if self.allow_same_major_older_versions_without_migration {
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

/// Returns a compatibility decision for a ZUIR schema version.
///
/// This helper uses the caller's policy; it does not assume every historical
/// ZUIR schema can be migrated automatically.
#[must_use]
pub const fn check_schema_compatibility(
    source: ZuirSchemaVersion,
    policy: CompatibilityPolicy,
) -> CompatibilityDecision {
    policy.check(source.components())
}

/// Returns a compatibility decision for a ZUIR serialization format.
///
/// Schema compatibility and encoding compatibility are separate decisions.
/// Call this function for the format version before decoding the payload.
#[must_use]
pub const fn check_format_compatibility(
    source: ZuirFormatVersion,
    policy: CompatibilityPolicy,
) -> CompatibilityDecision {
    policy.check(source.components())
}

/// Checks whether a dialect version matches the expected major contract.
///
/// This is only a coarse compatibility check. A dialect registry must still
/// validate the dialect's declared operations, types, dependencies, and
/// semantic contracts.
#[must_use]
pub const fn same_dialect_major(
    supplied: DialectVersion,
    expected: DialectVersion,
) -> bool {
    supplied.major() == expected.major()
}

/// Compares any two version identities by their numeric components.
///
/// The caller remains responsible for ensuring both values represent the
/// same version category before using the result for policy decisions.
#[must_use]
pub fn compare_versions<A: VersionIdentity, B: VersionIdentity>(
    left: A,
    right: B,
) -> Ordering {
    left.components().cmp(&right.components())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_components_and_display_are_stable() {
        let version = ZuirSchemaVersion::new(2, 7, 11);

        assert_eq!(version.major(), 2);
        assert_eq!(version.minor(), 7);
        assert_eq!(version.patch(), 11);
        assert_eq!(version.to_string(), "2.7.11");
    }

    #[test]
    fn version_categories_are_distinct_types() {
        let schema = ZuirSchemaVersion::new(1, 0, 0);
        let format = ZuirFormatVersion::new(1, 0, 0);

        assert_eq!(schema.components(), format.components());
        // The types intentionally have no implicit conversion between them.
    }

    #[test]
    fn checked_version_increments_do_not_overflow() {
        let version = VersionTriple::new(4, 8, 12);

        assert_eq!(
            version.checked_next_patch(),
            Some(VersionTriple::new(4, 8, 13))
        );
        assert_eq!(
            version.checked_next_minor(),
            Some(VersionTriple::new(4, 9, 0))
        );
        assert_eq!(
            version.checked_next_major(),
            Some(VersionTriple::new(5, 0, 0))
        );
        assert_eq!(
            VersionTriple::new(1, 2, u32::MAX).checked_next_patch(),
            None
        );
        assert_eq!(
            VersionTriple::new(1, u32::MAX, 0).checked_next_minor(),
            None
        );
        assert_eq!(
            VersionTriple::new(u32::MAX, 0, 0).checked_next_major(),
            None
        );
    }

    #[test]
    fn exact_policy_accepts_only_the_current_version() {
        let current = VersionTriple::new(1, 0, 0);
        let policy = CompatibilityPolicy::exact(current);

        assert_eq!(
            policy.check(current).compatibility(),
            Compatibility::Compatible
        );
        assert_eq!(
            policy
                .check(VersionTriple::new(1, 0, 1))
                .compatibility(),
            Compatibility::Incompatible
        );
        assert_eq!(
            policy
                .check(VersionTriple::new(1, 0, 0))
                .reason(),
            CompatibilityReason::ExactMatch
        );
    }

    #[test]
    fn older_versions_require_explicit_migration_by_default() {
        let current = VersionTriple::new(2, 1, 0);
        let minimum = VersionTriple::new(2, 0, 0);

        let policy = CompatibilityPolicy::new(current, minimum, false)
            .expect("valid compatibility range");

        let decision = policy.check(VersionTriple::new(2, 0, 3));

        assert_eq!(
            decision.compatibility(),
            Compatibility::RequiresMigration
        );
        assert_eq!(
            decision.reason(),
            CompatibilityReason::MigrationRequired
        );
        assert!(decision.must_reject_without_migration());
    }

    #[test]
    fn explicitly_supported_older_versions_can_be_accepted() {
        let current = VersionTriple::new(3, 2, 0);
        let minimum = VersionTriple::new(3, 0, 0);

        let policy = CompatibilityPolicy::new(current, minimum, true)
            .expect("valid compatibility range");

        let decision = policy.check(VersionTriple::new(3, 1, 7));

        assert_eq!(decision.compatibility(), Compatibility::Compatible);
        assert_eq!(
            decision.reason(),
            CompatibilityReason::SupportedOlderVersion
        );
    }

    #[test]
    fn future_and_different_major_versions_are_rejected() {
        let current = VersionTriple::new(1, 2, 0);
        let policy = CompatibilityPolicy::exact(current);

        assert_eq!(
            policy
                .check(VersionTriple::new(1, 3, 0))
                .reason(),
            CompatibilityReason::FutureVersion
        );
        assert_eq!(
            policy
                .check(VersionTriple::new(2, 0, 0))
                .reason(),
            CompatibilityReason::MajorVersionMismatch
        );
    }

    #[test]
    fn versions_below_the_supported_range_are_rejected() {
        let current = VersionTriple::new(2, 0, 0);
        let minimum = VersionTriple::new(1, 8, 0);

        let policy = CompatibilityPolicy::new(current, minimum, false)
            .expect("valid compatibility range");

        let decision = policy.check(VersionTriple::new(1, 7, 9));

        assert_eq!(decision.compatibility(), Compatibility::Incompatible);
        assert_eq!(
            decision.reason(),
            CompatibilityReason::BelowMinimumSupported
        );
    }

    #[test]
    fn invalid_compatibility_ranges_are_rejected() {
        assert!(
            CompatibilityPolicy::new(
                VersionTriple::new(1, 0, 0),
                VersionTriple::new(1, 1, 0),
                false,
            )
            .is_none()
        );
    }

    #[test]
    fn schema_and_format_checks_use_their_own_types() {
        let policy = CompatibilityPolicy::exact(VersionTriple::new(1, 0, 0));

        assert_eq!(
            check_schema_compatibility(
                CURRENT_ZUIR_SCHEMA_VERSION,
                policy
            )
            .compatibility(),
            Compatibility::Compatible
        );

        assert_eq!(
            check_format_compatibility(
                CURRENT_ZUIR_FORMAT_VERSION,
                policy
            )
            .compatibility(),
            Compatibility::Compatible
        );
    }

    #[test]
    fn dialect_major_compatibility_is_explicit() {
        assert!(same_dialect_major(
            DialectVersion::new(2, 1, 0),
            DialectVersion::new(2, 9, 4),
        ));

        assert!(!same_dialect_major(
            DialectVersion::new(2, 1, 0),
            DialectVersion::new(3, 0, 0),
        ));
    }

    #[test]
    fn version_serialization_round_trips() {
        let version = ZuirSchemaVersion::new(1, 4, 2);
        let encoded = serde_json::to_string(&version)
            .expect("version serialization should succeed");
        let decoded: ZuirSchemaVersion = serde_json::from_str(&encoded)
            .expect("version deserialization should succeed");

        assert_eq!(decoded, version);
    }
}