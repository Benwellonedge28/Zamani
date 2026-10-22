
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Hybrid Computing — Version Contracts
//!
//! Path: `src/hybrid/version.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: 1.97
//! Safety: Safe Rust only. Unsafe code is forbidden.
//!
//! ## PURPOSE
//!
//! Define the version identities and compatibility entry points owned by
//! Zamani's hybrid-composition subsystem.
//!
//! Hybrid versioning is deliberately independent of:
//! - the Zamani language specification version;
//! - the compiler package/release version;
//! - the ZUIR schema and serialization versions;
//! - the quantum IR version;
//! - classical, HDL, or other domain IR versions;
//! - backend, device, firmware, and runtime versions.
//!
//! ## OWNERSHIP
//!
//! This module owns:
//! - the hybrid logical-schema version type;
//! - the hybrid interchange-format version type;
//! - the domain-contract version type;
//! - initial/current hybrid version declarations;
//! - deterministic version parsing and formatting;
//! - explicit compatibility-check entry points.
//!
//! It does not own:
//! - serialization or deserialization;
//! - schema migration implementations;
//! - source-language feature acceptance;
//! - hardware discovery or capability negotiation;
//! - domain-specific operation semantics;
//! - compiler release versioning;
//! - target selection or execution.
//!
//! ## DEPENDENCIES
//!
//! `crate::zuir::version` is the canonical owner of generic version triples
//! and reusable version-compatibility decisions. This module introduces
//! distinct hybrid version categories without duplicating that machinery.
//!
//! Serde is already a dependency of the Zamani crate.
//!
//! ## COMPATIBILITY CONTRACT
//!
//! A version match alone does not prove semantic compatibility.
//! Schema and encoding compatibility are checked independently.
//! Older formats require an explicit migration unless a policy explicitly
//! permits reading them without migration.
//! Future versions are rejected by the conservative policy.
//!
//! ## SCALABILITY CONTRACT
//!
//! Version numbers are metadata, not hardware capacities.
//! No hardware or computation-size ceilings are introduced here.
//! Checked increments report overflow rather than wrapping.
//! Actual resource and input-processing limits belong to explicit policies.
//!
//! ## INTEGRATION
//!
//! - `src/hybrid/interchange/schema.rs` consumes the schema version.
//! - `src/hybrid/interchange/encode.rs` consumes the format version.
//! - `src/hybrid/interchange/decode.rs` checks the format before decoding
//!   and checks the schema before interpreting its contents.
//! - `src/hybrid/spec/compatibility.rs` owns hybrid-specific migration policy.
//! - `src/compiler/hybrid_pipeline.rs` must use the canonical format
//!   identifier instead of maintaining an independent format-version literal.
//!
//! This module does not perform any of those downstream operations.

use core::fmt;
use core::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::zuir::version::{
    Compatibility,
    CompatibilityDecision,
    CompatibilityPolicy,
    CompatibilityReason,
    VersionTriple,
};

/// Stable prefix for hybrid interchange-format identifiers.
///
/// The format version is appended by [`HybridFormatVersion::identifier`].
pub const HYBRID_FORMAT_PREFIX: &str = "zamani-hybrid-v";

/// Initial hybrid logical-schema version.
///
/// Change this only when a reviewed change to the hybrid representation
/// contract requires a schema-version change.
pub const CURRENT_HYBRID_SCHEMA_VERSION: HybridSchemaVersion =
    HybridSchemaVersion::new(1, 0, 0);

/// Initial hybrid serialized-format version.
///
/// Change this independently of the logical schema when the encoding
/// contract changes without an equivalent schema change.
pub const CURRENT_HYBRID_FORMAT_VERSION: HybridFormatVersion =
    HybridFormatVersion::new(1, 0, 0);

/// Initial domain-contract version.
///
/// This is a starting point, not a forced version for every domain.
/// Individual domain implementations retain ownership of their contracts.
pub const INITIAL_DOMAIN_CONTRACT_VERSION: DomainContractVersion =
    DomainContractVersion::new(1, 0, 0);

/// Error returned when a textual version cannot be parsed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VersionParseError {
    /// A component is missing.
    MissingComponent,

    /// The input contains more than three numeric components.
    UnexpectedComponent,

    /// A component is empty or contains invalid numeric characters.
    InvalidComponent,

    /// A numeric component exceeds the supported representation.
    ComponentOverflow,
}

impl fmt::Display for VersionParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingComponent => {
                f.write_str("version must contain major, minor, and patch")
            }
            Self::UnexpectedComponent => {
                f.write_str("version contains more than three components")
            }
            Self::InvalidComponent => {
                f.write_str("version component is not an unsigned decimal integer")
            }
            Self::ComponentOverflow => {
                f.write_str("version component exceeds the supported numeric range")
            }
        }
    }
}

impl std::error::Error for VersionParseError {}

/// Parse exactly three unsigned decimal components.
///
/// Prerelease and build-metadata syntax is intentionally not accepted:
/// this module defines numeric schema/format identities, not full SemVer.
fn parse_components(input: &str) -> Result<VersionTriple, VersionParseError> {
    let mut parts = input.split('.');

    let major = parse_component(parts.next().ok_or(
        VersionParseError::MissingComponent,
    )?)?;

    let minor = parse_component(parts.next().ok_or(
        VersionParseError::MissingComponent,
    )?)?;

    let patch = parse_component(parts.next().ok_or(
        VersionParseError::MissingComponent,
    )?)?;

    if parts.next().is_some() {
        return Err(VersionParseError::UnexpectedComponent);
    }

    Ok(VersionTriple::new(major, minor, patch))
}

fn parse_component(input: &str) -> Result<u32, VersionParseError> {
    if input.is_empty() || !input.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(VersionParseError::InvalidComponent);
    }

    input
        .parse::<u32>()
        .map_err(|_| VersionParseError::ComponentOverflow)
}

/// Defines a strongly typed hybrid version category.
///
/// These wrappers deliberately do not implement conversions between
/// categories. A schema version cannot accidentally be used as a format
/// version or domain-contract version.
macro_rules! define_hybrid_version {
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
            /// Creates a version from numeric components.
            #[must_use]
            pub const fn new(major: u32, minor: u32, patch: u32) -> Self {
                Self(VersionTriple::new(major, minor, patch))
            }

            /// Wraps the canonical version-triple type.
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

            /// Returns the canonical numeric version triple.
            #[must_use]
            pub const fn components(self) -> VersionTriple {
                self.0
            }

            /// Returns the next patch version, or `None` on overflow.
            #[must_use]
            pub const fn checked_next_patch(self) -> Option<Self> {
                match self.0.checked_next_patch() {
                    Some(next) => Some(Self(next)),
                    None => None,
                }
            }

            /// Returns the next minor version, or `None` on overflow.
            #[must_use]
            pub const fn checked_next_minor(self) -> Option<Self> {
                match self.0.checked_next_minor() {
                    Some(next) => Some(Self(next)),
                    None => None,
                }
            }

            /// Returns the next major version, or `None` on overflow.
            #[must_use]
            pub const fn checked_next_major(self) -> Option<Self> {
                match self.0.checked_next_major() {
                    Some(next) => Some(Self(next)),
                    None => None,
                }
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
                parse_components(input).map(Self)
            }
        }
    };
}

define_hybrid_version!(
    /// Version of the logical hybrid representation contract.
    ///
    /// This version describes the meaning and structure of hybrid composition.
    /// It is independent of the serialized encoding.
    HybridSchemaVersion
);

define_hybrid_version!(
    /// Version of the serialized hybrid interchange format.
    ///
    /// This version describes the encoding protocol, not the logical schema.
    HybridFormatVersion
);

define_hybrid_version!(
    /// Version of a participating domain's declared contract.
    ///
    /// Each domain owns its own contract version. Equal major numbers are
    /// only a coarse compatibility signal, not proof of semantic equivalence.
    DomainContractVersion
);

impl HybridFormatVersion {
    /// Returns the stable format identifier.
    ///
    /// For version `1.0.0`, this returns `zamani-hybrid-v1`.
    /// The major component identifies the format family.
    #[must_use]
    pub fn identifier(self) -> String {
        format!("{HYBRID_FORMAT_PREFIX}{}", self.major())
    }
}

/// Returns the conservative policy for hybrid schema versions.
///
/// Only the current schema is accepted without migration because this module
/// does not claim that historical schema migrations already exist.
#[must_use]
pub const fn hybrid_schema_compatibility_policy() -> CompatibilityPolicy {
    CompatibilityPolicy::exact(
        CURRENT_HYBRID_SCHEMA_VERSION.components(),
    )
}

/// Returns the conservative policy for hybrid format versions.
///
/// Schema compatibility does not imply encoding compatibility. The format
/// must be checked independently before attempting to decode an artifact.
#[must_use]
pub const fn hybrid_format_compatibility_policy() -> CompatibilityPolicy {
    CompatibilityPolicy::exact(
        CURRENT_HYBRID_FORMAT_VERSION.components(),
    )
}

/// Checks a supplied schema version against the current hybrid schema policy.
///
/// A historical version is not accepted merely because it is numerically
/// older. A migration must be explicitly supported by the migration layer.
#[must_use]
pub const fn check_hybrid_schema_compatibility(
    supplied: HybridSchemaVersion,
) -> CompatibilityDecision {
    hybrid_schema_compatibility_policy().check(supplied.components())
}

/// Checks a supplied format version against the current hybrid format policy.
///
/// Call this before decoding the artifact payload.
#[must_use]
pub const fn check_hybrid_format_compatibility(
    supplied: HybridFormatVersion,
) -> CompatibilityDecision {
    hybrid_format_compatibility_policy().check(supplied.components())
}

/// Checks a domain contract version against a caller-supplied policy.
///
/// The hybrid layer does not impose one universal current version on all
/// domains. The domain registry or owning adapter supplies the policy.
#[must_use]
pub const fn check_domain_contract_compatibility(
    supplied: DomainContractVersion,
    policy: CompatibilityPolicy,
) -> CompatibilityDecision {
    policy.check(supplied.components())
}

/// Returns `true` when a compatibility decision permits direct interpretation
/// without a migration.
#[must_use]
pub const fn is_compatible_without_migration(
    decision: CompatibilityDecision,
) -> bool {
    matches!(decision.compatibility(), Compatibility::Compatible)
}

/// Returns `true` when the supplied version is a future version relative to
/// the target version used by the compatibility decision.
#[must_use]
pub const fn is_future_version(
    decision: CompatibilityDecision,
) -> bool {
    matches!(decision.reason(), CompatibilityReason::FutureVersion)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_categories_are_distinct_types() {
        let schema = HybridSchemaVersion::new(1, 0, 0);
        let format = HybridFormatVersion::new(1, 0, 0);
        let domain = DomainContractVersion::new(1, 0, 0);

        assert_eq!(schema.components(), format.components());
        assert_eq!(format.components(), domain.components());

        // Deliberately no implicit conversions between these types.
    }

    #[test]
    fn numeric_version_formatting_is_stable() {
        let version = HybridSchemaVersion::new(3, 12, 7);

        assert_eq!(version.to_string(), "3.12.7");
        assert_eq!(version.major(), 3);
        assert_eq!(version.minor(), 12);
        assert_eq!(version.patch(), 7);
    }

    #[test]
    fn parsing_accepts_exactly_three_decimal_components() {
        assert_eq!(
            "1.0.0".parse::<HybridSchemaVersion>(),
            Ok(HybridSchemaVersion::new(1, 0, 0))
        );

        assert_eq!(
            "12.34.56".parse::<HybridFormatVersion>(),
            Ok(HybridFormatVersion::new(12, 34, 56))
        );

        assert_eq!(
            "1.2".parse::<HybridSchemaVersion>(),
            Err(VersionParseError::MissingComponent)
        );

        assert_eq!(
            "1.2.3.4".parse::<HybridSchemaVersion>(),
            Err(VersionParseError::UnexpectedComponent)
        );

        assert_eq!(
            "1.x.3".parse::<HybridSchemaVersion>(),
            Err(VersionParseError::InvalidComponent)
        );

        assert_eq!(
            "1.2.4294967296".parse::<HybridSchemaVersion>(),
            Err(VersionParseError::ComponentOverflow)
        );
    }

    #[test]
    fn checked_increments_never_wrap() {
        let version = HybridSchemaVersion::new(2, 4, 8);

        assert_eq!(
            version.checked_next_patch(),
            Some(HybridSchemaVersion::new(2, 4, 9))
        );

        assert_eq!(
            version.checked_next_minor(),
            Some(HybridSchemaVersion::new(2, 5, 0))
        );

        assert_eq!(
            version.checked_next_major(),
            Some(HybridSchemaVersion::new(3, 0, 0))
        );

        assert_eq!(
            HybridSchemaVersion::new(1, 0, u32::MAX)
                .checked_next_patch(),
            None
        );

        assert_eq!(
            HybridSchemaVersion::new(1, u32::MAX, 0)
                .checked_next_minor(),
            None
        );

        assert_eq!(
            HybridSchemaVersion::new(u32::MAX, 0, 0)
                .checked_next_major(),
            None
        );
    }

    #[test]
    fn format_identifier_is_stable() {
        assert_eq!(
            HybridFormatVersion::new(1, 0, 0).identifier(),
            "zamani-hybrid-v1"
        );

        assert_eq!(
            HybridFormatVersion::new(3, 2, 1).identifier(),
            "zamani-hybrid-v3"
        );
    }

    #[test]
    fn current_schema_is_accepted_without_migration() {
        let decision = check_hybrid_schema_compatibility(
            CURRENT_HYBRID_SCHEMA_VERSION,
        );

        assert!(is_compatible_without_migration(decision));
        assert_eq!(decision.reason(), CompatibilityReason::ExactMatch);
    }

    #[test]
    fn future_schema_is_rejected() {
        let future = HybridSchemaVersion::new(
            CURRENT_HYBRID_SCHEMA_VERSION.major() + 1,
            0,
            0,
        );

        let decision = check_hybrid_schema_compatibility(future);

        assert_eq!(decision.compatibility(), Compatibility::Incompatible);
        assert!(is_future_version(decision));
    }

    #[test]
    fn historical_schema_requires_an_explicit_migration() {
        let historical = HybridSchemaVersion::new(0, 9, 0);

        let decision = check_hybrid_schema_compatibility(historical);

        assert_eq!(
            decision.compatibility(),
            Compatibility::Incompatible
        );

        assert_eq!(
            decision.reason(),
            CompatibilityReason::MajorVersionMismatch
        );
    }

    #[test]
    fn schema_and_format_policies_are_independent() {
        let schema_decision = check_hybrid_schema_compatibility(
            CURRENT_HYBRID_SCHEMA_VERSION,
        );

        let format_decision = check_hybrid_format_compatibility(
            HybridFormatVersion::new(2, 0, 0),
        );

        assert_eq!(
            schema_decision.compatibility(),
            Compatibility::Compatible
        );

        assert_eq!(
            format_decision.compatibility(),
            Compatibility::Incompatible
        );
    }

    #[test]
    fn domain_versions_use_the_callers_policy() {
        let current = DomainContractVersion::new(2, 1, 0);

        let policy = CompatibilityPolicy::exact(
            current.components(),
        );

        let decision =
            check_domain_contract_compatibility(current, policy);

        assert_eq!(
            decision.compatibility(),
            Compatibility::Compatible
        );
    }

    #[test]
    fn format_version_serialization_is_transparent() {
        let version = HybridFormatVersion::new(1, 2, 3);

        let encoded = serde_json::to_string(&version)
            .expect("version serialization should succeed");

        assert_eq!(encoded, r#""1.2.3""#);

        let decoded: HybridFormatVersion =
            serde_json::from_str(&encoded)
                .expect("version deserialization should succeed");

        assert_eq!(decoded, version);
    }
}
