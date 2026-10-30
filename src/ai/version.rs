
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani AI — Version and Compatibility Contracts
//!
//! Path: src/ai/version.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # PURPOSE
//!
//! Defines version identities and compatibility policies for the
//! Zamani AI computational domain.
//!
//! This module supports independently evolving:
//!
//! - AI public APIs;
//! - AI semantic contracts;
//! - AI model schemas;
//! - AI operation sets;
//! - AI artifact formats.
//!
//! It reuses the canonical ZUIR version representation and compatibility
//! classifications rather than introducing a competing version authority.
//!
//! # OWNERSHIP
//!
//! This module owns AI-specific version identities and AI-specific
//! compatibility-range evaluation.
//!
//! It does not own:
//!
//! - Zamani language versioning;
//! - ZUIR schema or serialization versioning;
//! - ZUIR dialect versioning;
//! - quantum IR versioning;
//! - compiler release versioning;
//! - backend versioning;
//! - runtime protocol versioning;
//! - hardware or firmware versioning;
//! - artifact serialization implementations;
//! - migration execution;
//! - deployment manifests.
//!
//! # CANONICAL VERSION AUTHORITY
//!
//! `crate::zuir::version` owns:
//!
//! - VersionTriple;
//! - VersionIdentity;
//! - LanguageVersion;
//! - ZuirSchemaVersion;
//! - ZuirFormatVersion;
//! - DialectVersion;
//! - Compatibility;
//! - CompatibilityReason.
//!
//! AI-specific version wrappers use VersionTriple.
//!
//! # INTEGRATION
//!
//! AI API:
//!     src/ai/api.rs
//!
//! AI semantic contract:
//!     src/frontend/semantic/domains/ai.rs
//!
//! AI model schema:
//!     src/ai/model/definition.rs
//!
//! AI operation set:
//!     src/ai/registry.rs
//!
//! AI artifact format:
//!     src/ai/deployment/artifacts.rs
//!
//! AI compatibility:
//!     src/ai/compatibility.rs
//!
//! ZUIR schema and format:
//!     src/zuir/version.rs
//!
//! Dialect compatibility:
//!     src/zuir/dialect/versioning.rs
//!
//! Deployment manifests:
//!     src/ai/deployment/manifests.rs
//!
//! # SCALABILITY
//!
//! Version numbers are independent of computational resources.
//!
//! No limits are imposed on:
//!
//! - AI models;
//! - model parameters;
//! - operations;
//! - tensors;
//! - datasets;
//! - agents;
//! - workers;
//! - accelerators;
//! - execution nodes;
//! - memory;
//! - hardware capacity.
//!
//! Version components use the canonical u32 representation established
//! by ZUIR. They describe version identities, not workload capacities.
//!
//! Version increments use checked arithmetic.
//!
//! # COMPATIBILITY
//!
//! Version equality alone does not establish complete semantic
//! compatibility.
//!
//! Callers must evaluate the appropriate version domain and all
//! applicable cross-domain compatibility requirements.
//!
//! Future versions must never be silently accepted.
//!
//! Older versions requiring migration must not be interpreted as
//! current representations before migration.
//!
//! # MIGRATION
//!
//! This module describes compatibility decisions.
//!
//! Actual transformations belong to the repository's migration,
//! serialization, and deployment infrastructure.
//!
//! # FREEZE CRITERIA
//!
//! - All version domains are strongly typed.
//! - Canonical ZUIR version representation is reused.
//! - Version domains cannot be accidentally interchanged.
//! - Compatibility ranges are validated.
//! - Future versions are rejected.
//! - Migration requirements are explicit.
//! - Deserialized policies can be revalidated.
//! - Version increments cannot silently overflow.
//! - Serialization round trips are tested.
//! - No unsafe Rust is introduced.

use core::fmt;

use serde::{Deserialize, Serialize};

use thiserror::Error;

// ============================================================================
// CANONICAL ZUIR VERSION INFRASTRUCTURE
// ============================================================================
//
// ZUIR is the authoritative owner of shared version representation.
//
// Re-export its version types instead of defining duplicates.
//
// This also allows downstream AI modules to use a consistent version API.

pub use crate::zuir::version::{
    Compatibility,
    CompatibilityReason,
    DialectVersion,
    LanguageVersion,
    VersionIdentity,
    VersionTriple,
    ZuirFormatVersion,
    ZuirSchemaVersion,
};

// ============================================================================
// AI VERSION TYPE DEFINITION
// ============================================================================
//
// Every independently evolving AI contract receives its own Rust type.
//
// Equal numeric values do not make different version domains
// interchangeable.
//
// Example:
//
// AiApiVersion(1.0.0)
// AiModelSchemaVersion(1.0.0)
//
// These are distinct version identities.

macro_rules! define_ai_version {
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
            Serialize,
            Deserialize,
        )]
        #[serde(transparent)]
        pub struct $name(VersionTriple);

        impl $name {
            /// Construct an AI version.
            #[must_use]
            pub const fn new(
                major: u32,
                minor: u32,
                patch: u32,
            ) -> Self {
                Self(
                    VersionTriple::new(
                        major,
                        minor,
                        patch,
                    )
                )
            }

            /// Construct from the canonical ZUIR representation.
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

            /// Increment the patch version.
            ///
            /// Returns None on overflow.
            #[must_use]
            pub const fn checked_next_patch(self) -> Option<Self> {
                match self.0.checked_next_patch() {
                    Some(value) => Some(Self(value)),
                    None => None,
                }
            }

            /// Increment the minor version.
            ///
            /// The patch component is reset.
            #[must_use]
            pub const fn checked_next_minor(self) -> Option<Self> {
                match self.0.checked_next_minor() {
                    Some(value) => Some(Self(value)),
                    None => None,
                }
            }

            /// Increment the major version.
            ///
            /// Minor and patch components are reset.
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
            fn fmt(
                &self,
                formatter: &mut fmt::Formatter<'_>,
            ) -> fmt::Result {
                self.0.fmt(formatter)
            }
        }

        impl From<$name> for VersionTriple {
            fn from(value: $name) -> Self {
                value.0
            }
        }
    };
}

// ============================================================================
// AI PUBLIC API VERSION
// ============================================================================

define_ai_version!(
    /// Version of the stable public AI API contract.
    ///
    /// This version changes when the externally observable AI API
    /// contract changes.
    ///
    /// It does not track compiler releases or runtime implementations.
    AiApiVersion
);

// ============================================================================
// AI SEMANTIC VERSION
// ============================================================================

define_ai_version!(
    /// Version of the normative AI computational semantic contract.
    ///
    /// This version identifies the meaning and guarantees of AI
    /// computation within Zamani.
    ///
    /// It is independent of source-language, ZUIR, and compiler versions.
    AiSemanticVersion
);

// ============================================================================
// AI MODEL SCHEMA VERSION
// ============================================================================

define_ai_version!(
    /// Version of the logical AI model schema.
    ///
    /// Covers the structural and semantic model representation contract.
    ///
    /// It does not identify the serialization encoding.
    AiModelSchemaVersion
);

// ============================================================================
// AI OPERATION SET VERSION
// ============================================================================

define_ai_version!(
    /// Version of the AI operation-set contract.
    ///
    /// Identifies the AI operation definitions and their declared
    /// semantic compatibility contract.
    ///
    /// It does not imply that every operation has an executable
    /// implementation on every target.
    AiOperationSetVersion
);

// ============================================================================
// AI ARTIFACT FORMAT VERSION
// ============================================================================

define_ai_version!(
    /// Version of the serialized AI artifact format.
    ///
    /// This version is independent of:
    ///
    /// - logical model schema;
    /// - AI semantics;
    /// - AI API;
    /// - compiler release;
    /// - backend implementation.
    ///
    /// A format reader must validate this version before interpreting
    /// an artifact payload.
    AiArtifactFormatVersion
);

// ============================================================================
// INITIAL CONTRACT VERSIONS
// ============================================================================
//
// These values establish initial version identities for the AI subsystem.
//
// They are not hardware limits.
//
// They must be changed only through reviewed contract evolution.
//
// The word INITIAL is deliberate: the existence of an initial version
// does not certify that the corresponding subsystem is production-ready.

/// Initial public AI API contract.
pub const INITIAL_AI_API_VERSION: AiApiVersion =
    AiApiVersion::new(1, 0, 0);

/// Initial AI semantic contract.
pub const INITIAL_AI_SEMANTIC_VERSION: AiSemanticVersion =
    AiSemanticVersion::new(1, 0, 0);

/// Initial AI model schema.
pub const INITIAL_AI_MODEL_SCHEMA_VERSION: AiModelSchemaVersion =
    AiModelSchemaVersion::new(1, 0, 0);

/// Initial AI operation set.
pub const INITIAL_AI_OPERATION_SET_VERSION: AiOperationSetVersion =
    AiOperationSetVersion::new(1, 0, 0);

/// Initial AI artifact encoding.
pub const INITIAL_AI_ARTIFACT_FORMAT_VERSION: AiArtifactFormatVersion =
    AiArtifactFormatVersion::new(1, 0, 0);

// ============================================================================
// VERSION RANGE ERRORS
// ============================================================================

/// Errors encountered while defining a supported version range.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Error,
)]
pub enum VersionRangeError {
    /// The minimum version is newer than the maximum version.
    #[error("minimum supported version is newer than maximum supported version")]
    InvertedRange,
}

// ============================================================================
// VERSION RANGE
// ============================================================================
//
// A version range represents explicitly supported versions.
//
// Both boundaries are inclusive.
//
// It is a compatibility declaration, not an automatic migration guarantee.
//
// No arbitrary history-window or version-count ceiling is introduced.

/// An inclusive supported version range.
///
/// The type is generic so a range for AI model schemas cannot accidentally
/// be used as a range for AI artifact formats.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
)]
pub struct VersionRange<V> {
    minimum: V,
    maximum: V,
}

impl<V: VersionIdentity> VersionRange<V> {
    /// Construct a validated inclusive version range.
    ///
    /// Returns an error when minimum is greater than maximum.
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

    /// Validate a range, including one obtained through deserialization.
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

    /// Determine whether a version belongs to this range.
    #[must_use]
    pub fn contains(&self, version: V) -> bool {
        version >= self.minimum && version <= self.maximum
    }

    /// Determine whether the range contains exactly one version.
    #[must_use]
    pub fn is_exact(&self) -> bool {
        self.minimum == self.maximum
    }
}

// ============================================================================
// COMPATIBILITY POLICY ERRORS
// ============================================================================

/// Errors encountered while constructing an AI compatibility policy.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Error,
)]
pub enum VersionPolicyError {
    /// The supplied supported range is invalid.
    #[error("the supported version range is invalid")]
    InvalidRange,

    /// The current implementation version is outside its supported range.
    #[error("current version is outside the declared supported range")]
    CurrentOutsideSupportedRange,
}

// ============================================================================
// AI VERSION COMPATIBILITY POLICY
// ============================================================================
//
// This policy deliberately separates:
//
// 1. What version the reader currently implements.
// 2. Which historical versions it recognizes.
// 3. Whether older same-major versions can be consumed without migration.
//
// Recognizing a version does not imply that its representation is safe
// to interpret without migration.

/// Explicit compatibility policy for an AI version domain.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
)]
pub struct AiVersionPolicy<V> {
    current: V,
    supported: VersionRange<V>,
    allow_older_same_major_without_migration: bool,
}

impl<V: VersionIdentity> AiVersionPolicy<V> {
    /// Construct an AI version compatibility policy.
    ///
    /// The current version must be contained in the supported range.
    ///
    /// `allow_older_same_major_without_migration` is an explicit
    /// compatibility promise. It must only be enabled when conformance
    /// evidence establishes that promise.
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
                VersionPolicyError::CurrentOutsideSupportedRange
            );
        }

        Ok(Self {
            current,
            supported,
            allow_older_same_major_without_migration,
        })
    }

    /// Revalidate a policy, including a deserialized policy.
    pub fn validate(&self) -> Result<(), VersionPolicyError> {
        self.supported
            .validate()
            .map_err(|_| VersionPolicyError::InvalidRange)?;

        if !self.supported.contains(self.current) {
            return Err(
                VersionPolicyError::CurrentOutsideSupportedRange
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

    /// Check whether older same-major versions can be used without
    /// migration under this policy.
    #[must_use]
    pub const fn allows_older_without_migration(&self) -> bool {
        self.allow_older_same_major_without_migration
    }

    /// Evaluate a supplied version.
    ///
    /// Compatibility is evaluated conservatively:
    ///
    /// - Exact version: compatible.
    /// - Future version: incompatible.
    /// - Version outside the supported range: incompatible.
    /// - Older major version: migration required.
    /// - Older same-major version: compatible only when explicitly allowed.
    /// - Other supported historical versions: migration required.
    ///
    /// No migration is performed by this function.
    #[must_use]
    pub fn check(
        &self,
        supplied: V,
    ) -> AiVersionDecision<V> {
        let current = self.current;

        if supplied == current {
            return AiVersionDecision::new(
                supplied,
                current,
                Compatibility::Compatible,
                CompatibilityReason::ExactMatch,
            );
        }

        if supplied > current {
            return AiVersionDecision::new(
                supplied,
                current,
                Compatibility::Incompatible,
                CompatibilityReason::FutureVersion,
            );
        }

        if !self.supported.contains(supplied) {
            return AiVersionDecision::new(
                supplied,
                current,
                Compatibility::Incompatible,
                CompatibilityReason::OutsideAllowedRange,
            );
        }

        if supplied.major() != current.major() {
            return AiVersionDecision::new(
                supplied,
                current,
                Compatibility::RequiresMigration,
                CompatibilityReason::MajorVersionMismatch,
            );
        }

        if self.allow_older_same_major_without_migration {
            return AiVersionDecision::new(
                supplied,
                current,
                Compatibility::Compatible,
                CompatibilityReason::SupportedOlderVersion,
            );
        }

        AiVersionDecision::new(
            supplied,
            current,
            Compatibility::RequiresMigration,
            CompatibilityReason::MigrationRequired,
        )
    }
}

// ============================================================================
// AI VERSION COMPATIBILITY DECISION
// ============================================================================

/// Complete typed compatibility decision for an AI version domain.
///
/// The supplied and current versions retain their concrete Rust type.
///
/// This prevents accidentally treating a model schema decision as an
/// artifact-format decision.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
)]
pub struct AiVersionDecision<V> {
    supplied: V,
    current: V,
    compatibility: Compatibility,
    reason: CompatibilityReason,
}

impl<V: VersionIdentity> AiVersionDecision<V> {
    /// Construct a compatibility decision.
    #[must_use]
    pub const fn new(
        supplied: V,
        current: V,
        compatibility: Compatibility,
        reason: CompatibilityReason,
    ) -> Self {
        Self {
            supplied,
            current,
            compatibility,
            reason,
        }
    }

    /// Return the supplied version.
    #[must_use]
    pub const fn supplied(&self) -> V {
        self.supplied
    }

    /// Return the current version.
    #[must_use]
    pub const fn current(&self) -> V {
        self.current
    }

    /// Return the compatibility classification.
    #[must_use]
    pub const fn compatibility(&self) -> Compatibility {
        self.compatibility
    }

    /// Return the reason for the decision.
    #[must_use]
    pub const fn reason(&self) -> CompatibilityReason {
        self.reason
    }

    /// Whether the version can be used without migration.
    #[must_use]
    pub const fn is_compatible_without_migration(&self) -> bool {
        matches!(
            self.compatibility,
            Compatibility::Compatible
        )
    }

    /// Whether explicit migration is required.
    #[must_use]
    pub const fn requires_migration(&self) -> bool {
        matches!(
            self.compatibility,
            Compatibility::RequiresMigration
        )
    }

    /// Whether the supplied version must be rejected.
    #[must_use]
    pub const fn must_reject(&self) -> bool {
        matches!(
            self.compatibility,
            Compatibility::Incompatible
        )
    }
}

// ============================================================================
// INITIAL AI COMPATIBILITY POLICIES
// ============================================================================
//
// These helpers create conservative initial policies.
//
// Historical compatibility must be expanded only when actual readers,
// migrations, and compatibility tests establish support.
//
// The default policy does not silently accept historical representations.

/// Create an exact-version AI compatibility policy.
pub fn exact_ai_policy<V: VersionIdentity>(
    current: V,
) -> AiVersionPolicy<V> {
    let supported = VersionRange::new(current, current)
        .expect("an exact version range is always valid");

    AiVersionPolicy::new(
        current,
        supported,
        false,
    )
    .expect("an exact policy contains its current version")
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ------------------------------------------------------------------------
    // Version identity
    // ------------------------------------------------------------------------

    #[test]
    fn ai_versions_have_distinct_types() {
        assert_ne!(
            core::any::type_name::<AiApiVersion>(),
            core::any::type_name::<AiSemanticVersion>(),
        );

        assert_ne!(
            core::any::type_name::<AiModelSchemaVersion>(),
            core::any::type_name::<AiArtifactFormatVersion>(),
        );

        assert_ne!(
            core::any::type_name::<AiOperationSetVersion>(),
            core::any::type_name::<AiApiVersion>(),
        );
    }

    #[test]
    fn version_components_are_preserved() {
        let version = AiApiVersion::new(2, 7, 11);

        assert_eq!(version.major(), 2);
        assert_eq!(version.minor(), 7);
        assert_eq!(version.patch(), 11);
        assert_eq!(version.to_string(), "2.7.11");
    }

    #[test]
    fn canonical_version_components_are_reused() {
        let ai = AiSemanticVersion::new(1, 2, 3);

        let canonical: VersionTriple = ai.into();

        assert_eq!(canonical.major(), 1);
        assert_eq!(canonical.minor(), 2);
        assert_eq!(canonical.patch(), 3);
    }

    // ------------------------------------------------------------------------
    // Version evolution
    // ------------------------------------------------------------------------

    #[test]
    fn patch_increment_is_checked() {
        let version = AiApiVersion::new(1, 2, 3);

        assert_eq!(
            version.checked_next_patch(),
            Some(AiApiVersion::new(1, 2, 4))
        );
    }

    #[test]
    fn minor_increment_resets_patch() {
        let version = AiApiVersion::new(1, 2, 9);

        assert_eq!(
            version.checked_next_minor(),
            Some(AiApiVersion::new(1, 3, 0))
        );
    }

    #[test]
    fn major_increment_resets_minor_and_patch() {
        let version = AiApiVersion::new(1, 8, 9);

        assert_eq!(
            version.checked_next_major(),
            Some(AiApiVersion::new(2, 0, 0))
        );
    }

    #[test]
    fn version_increment_never_silently_overflows() {
        assert_eq!(
            AiApiVersion::new(1, 0, u32::MAX)
                .checked_next_patch(),
            None
        );

        assert_eq!(
            AiApiVersion::new(1, u32::MAX, 0)
                .checked_next_minor(),
            None
        );

        assert_eq!(
            AiApiVersion::new(u32::MAX, 0, 0)
                .checked_next_major(),
            None
        );
    }

    // ------------------------------------------------------------------------
    // Version ranges
    // ------------------------------------------------------------------------

    #[test]
    fn version_ranges_include_both_boundaries() {
        let range = VersionRange::new(
            AiModelSchemaVersion::new(1, 0, 0),
            AiModelSchemaVersion::new(2, 0, 0),
        )
        .expect("valid range");

        assert!(range.contains(
            AiModelSchemaVersion::new(1, 0, 0)
        ));

        assert!(range.contains(
            AiModelSchemaVersion::new(2, 0, 0)
        ));

        assert!(range.contains(
            AiModelSchemaVersion::new(1, 5, 7)
        ));
    }

    #[test]
    fn version_ranges_reject_versions_outside_boundaries() {
        let range = VersionRange::new(
            AiModelSchemaVersion::new(1, 0, 0),
            AiModelSchemaVersion::new(2, 0, 0),
        )
        .expect("valid range");

        assert!(!range.contains(
            AiModelSchemaVersion::new(0, 9, 9)
        ));

        assert!(!range.contains(
            AiModelSchemaVersion::new(2, 0, 1)
        ));
    }

    #[test]
    fn inverted_version_ranges_are_rejected() {
        let result = VersionRange::new(
            AiApiVersion::new(2, 0, 0),
            AiApiVersion::new(1, 0, 0),
        );

        assert_eq!(
            result,
            Err(VersionRangeError::InvertedRange)
        );
    }

    #[test]
    fn exact_ranges_are_recognized() {
        let version = AiApiVersion::new(1, 0, 0);

        let range = VersionRange::new(version, version)
            .expect("valid range");

        assert!(range.is_exact());
    }

    // ------------------------------------------------------------------------
    // Compatibility policy
    // ------------------------------------------------------------------------

    #[test]
    fn exact_current_version_is_compatible() {
        let current = AiModelSchemaVersion::new(1, 0, 0);

        let policy = exact_ai_policy(current);

        let decision = policy.check(current);

        assert_eq!(
            decision.compatibility(),
            Compatibility::Compatible
        );

        assert_eq!(
            decision.reason(),
            CompatibilityReason::ExactMatch
        );

        assert!(decision.is_compatible_without_migration());
    }

    #[test]
    fn future_versions_are_rejected() {
        let current = AiModelSchemaVersion::new(1, 0, 0);

        let policy = exact_ai_policy(current);

        let decision = policy.check(
            AiModelSchemaVersion::new(1, 1, 0)
        );

        assert_eq!(
            decision.compatibility(),
            Compatibility::Incompatible
        );

        assert_eq!(
            decision.reason(),
            CompatibilityReason::FutureVersion
        );

        assert!(decision.must_reject());
    }

    #[test]
    fn unsupported_historical_versions_are_rejected() {
        let current = AiArtifactFormatVersion::new(2, 0, 0);

        let range = VersionRange::new(
            AiArtifactFormatVersion::new(1, 5, 0),
            current,
        )
        .expect("valid range");

        let policy = AiVersionPolicy::new(
            current,
            range,
            false,
        )
        .expect("valid policy");

        let decision = policy.check(
            AiArtifactFormatVersion::new(1, 4, 9)
        );

        assert_eq!(
            decision.compatibility(),
            Compatibility::Incompatible
        );

        assert_eq!(
            decision.reason(),
            CompatibilityReason::OutsideAllowedRange
        );
    }

    #[test]
    fn supported_historical_versions_require_migration_by_default() {
        let current = AiModelSchemaVersion::new(2, 0, 0);

        let range = VersionRange::new(
            AiModelSchemaVersion::new(1, 0, 0),
            current,
        )
        .expect("valid range");

        let policy = AiVersionPolicy::new(
            current,
            range,
            false,
        )
        .expect("valid policy");

        let decision = policy.check(
            AiModelSchemaVersion::new(1, 5, 0)
        );

        assert_eq!(
            decision.compatibility(),
            Compatibility::RequiresMigration
        );

        assert_eq!(
            decision.reason(),
            CompatibilityReason::MajorVersionMismatch
        );

        assert!(decision.requires_migration());
    }

    #[test]
    fn older_same_major_versions_require_migration_by_default() {
        let current = AiOperationSetVersion::new(1, 3, 0);

        let range = VersionRange::new(
            AiOperationSetVersion::new(1, 0, 0),
            current,
        )
        .expect("valid range");

        let policy = AiVersionPolicy::new(
            current,
            range,
            false,
        )
        .expect("valid policy");

        let decision = policy.check(
            AiOperationSetVersion::new(1, 2, 0)
        );

        assert_eq!(
            decision.compatibility(),
            Compatibility::RequiresMigration
        );

        assert_eq!(
            decision.reason(),
            CompatibilityReason::MigrationRequired
        );
    }

    #[test]
    fn older_same_major_versions_can_be_explicitly_supported() {
        let current = AiApiVersion::new(1, 4, 0);

        let range = VersionRange::new(
            AiApiVersion::new(1, 0, 0),
            current,
        )
        .expect("valid range");

        let policy = AiVersionPolicy::new(
            current,
            range,
            true,
        )
        .expect("valid policy");

        let decision = policy.check(
            AiApiVersion::new(1, 2, 0)
        );

        assert_eq!(
            decision.compatibility(),
            Compatibility::Compatible
        );

        assert_eq!(
            decision.reason(),
            CompatibilityReason::SupportedOlderVersion
        );
    }

    #[test]
    fn current_version_must_belong_to_supported_range() {
        let range = VersionRange::new(
            AiApiVersion::new(1, 0, 0),
            AiApiVersion::new(1, 2, 0),
        )
        .expect("valid range");

        let result = AiVersionPolicy::new(
            AiApiVersion::new(1, 3, 0),
            range,
            false,
        );

        assert_eq!(
            result,
            Err(
                VersionPolicyError::CurrentOutsideSupportedRange
            )
        );
    }

    // ------------------------------------------------------------------------
    // Serialization
    // ------------------------------------------------------------------------

    #[test]
    fn ai_versions_serialize_and_deserialize() {
        let original = AiModelSchemaVersion::new(3, 4, 5);

        let encoded = serde_json::to_string(&original)
            .expect("serialization succeeds");

        let decoded: AiModelSchemaVersion =
            serde_json::from_str(&encoded)
                .expect("deserialization succeeds");

        assert_eq!(original, decoded);
    }

    #[test]
    fn version_ranges_round_trip() {
        let original = VersionRange::new(
            AiArtifactFormatVersion::new(1, 0, 0),
            AiArtifactFormatVersion::new(2, 0, 0),
        )
        .expect("valid range");

        let encoded = serde_json::to_string(&original)
            .expect("serialization succeeds");

        let decoded: VersionRange<AiArtifactFormatVersion> =
            serde_json::from_str(&encoded)
                .expect("deserialization succeeds");

        assert_eq!(original, decoded);
        assert!(decoded.validate().is_ok());
    }

    #[test]
    fn compatibility_policy_round_trip() {
        let current = AiSemanticVersion::new(1, 2, 0);

        let range = VersionRange::new(
            AiSemanticVersion::new(1, 0, 0),
            current,
        )
        .expect("valid range");

        let original = AiVersionPolicy::new(
            current,
            range,
            false,
        )
        .expect("valid policy");

        let encoded = serde_json::to_string(&original)
            .expect("serialization succeeds");

        let decoded: AiVersionPolicy<AiSemanticVersion> =
            serde_json::from_str(&encoded)
                .expect("deserialization succeeds");

        assert!(decoded.validate().is_ok());

        assert_eq!(decoded.current(), current);
    }

    // ------------------------------------------------------------------------
    // Initial contract versions
    // ------------------------------------------------------------------------

    #[test]
    fn all_initial_ai_contract_versions_are_defined() {
        assert_eq!(
            INITIAL_AI_API_VERSION,
            AiApiVersion::new(1, 0, 0)
        );

        assert_eq!(
            INITIAL_AI_SEMANTIC_VERSION,
            AiSemanticVersion::new(1, 0, 0)
        );

        assert_eq!(
            INITIAL_AI_MODEL_SCHEMA_VERSION,
            AiModelSchemaVersion::new(1, 0, 0)
        );

        assert_eq!(
            INITIAL_AI_OPERATION_SET_VERSION,
            AiOperationSetVersion::new(1, 0, 0)
        );

        assert_eq!(
            INITIAL_AI_ARTIFACT_FORMAT_VERSION,
            AiArtifactFormatVersion::new(1, 0, 0)
        );
    }
}
