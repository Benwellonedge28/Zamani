
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR Incremental Compilation — Stable Cache Keys
//!
//! Path: `src/zuir/incremental/cache_keys.rs`
//! Edition: Rust 2021
//! Minimum Rust baseline: Rust 1.97 or later.
//! Safety: Safe Rust only.
//!
//! PURPOSE
//!
//! Derive deterministic cache keys for ZUIR compilation artifacts from
//! explicit artifact identity, compiler identity, schema and format versions,
//! direct-input fingerprints, configuration, pass pipeline, semantic
//! environment, optional target identity, and dependency fingerprints.
//!
//! AUTHORITY
//!
//! - `src/zuir/incremental/dependencies.rs` owns logical dependency identity.
//! - `src/zuir/incremental/invalidation.rs` owns invalidation planning.
//! - `src/zuir/fingerprint/stable_hash.rs` owns canonical framed hashing.
//! - `src/zuir/version.rs` owns ZUIR schema and format versions.
//! - `src/zuir/serialization/canonical.rs` owns canonical JSON rendering.
//! - `src/zuir/ZUIR-SPEC.md` owns ZUIR representation semantics.
//! - `grammar/DESIGN.md` and `grammar/specification/` own language contracts.
//!
//! OWNS
//!
//! - The cache-key input contract.
//! - Stable cache-key derivation.
//! - Canonical dependency-fingerprint ordering.
//! - Validation of cache-key inputs.
//! - Errors for malformed or ambiguous cache-key input.
//!
//! DOES_NOT_OWN
//!
//! - Artifact storage, cache eviction, cache persistence, or cache locking.
//! - Determining which artifacts are semantically equivalent.
//! - Computing source, IR, configuration, pass, or target fingerprints.
//! - Dependency graph mutation or invalidation planning.
//! - Parsing grammar files or deciding language semantics.
//! - Target selection, hardware discovery, allocation, or execution.
//!
//! STABILITY CONTRACT
//!
//! The hash domain and field names below form a versioned cache-key schema.
//! Changing the included fields, their meanings, or their encoding requires
//! a deliberate cache-key schema change. A changed key schema naturally
//! produces different keys; callers should also invalidate or namespace
//! persisted caches when upgrading the compiler.
//!
//! A cache key is a lookup identity, not proof that an artifact is valid.
//! Cache hits must still respect the cache's integrity, trust, compatibility,
//! and artifact-validation policies.
//!
//! SCALABILITY CONTRACT
//!
//! There is no universal limit on dependency count or artifact count.
//! Dependencies are collected dynamically and sorted for deterministic
//! encoding. Work and memory grow with the supplied dependency list and
//! remain subject to actual process resources.
//!
//! POCO-REAF CONTRACT
//!
//! Target-independent artifacts should pass `target_fingerprint: None`.
//! Target-specific artifacts must include a fingerprint for every target
//! property that can affect their result. Never include an incidental device
//! identifier in a portable artifact key.
//!
//! Every semantic input that can change the produced artifact must be
//! represented by a direct-input fingerprint, a dependency fingerprint,
//! compiler identity, configuration fingerprint, pipeline fingerprint,
//! semantic-environment fingerprint, or target fingerprint.
//!
//! No cache-key design can compensate for a caller that omits a relevant
//! input. The coordinator owns the completeness of those fingerprints.
//!
//! INTEGRATION
//!
//! - `incremental/mod.rs` must declare and re-export this module.
//! - The build coordinator constructs `CacheKeyInput` after collecting all
//!   relevant fingerprints.
//! - `dependencies.rs` supplies `DependencyKey` values.
//! - `invalidation.rs` determines which artifact entries may be stale.
//! - Cache storage uses `CacheKey` as a lookup identity.
//! - Fingerprint adapters own canonical encoding of semantic content.
//!
//! Concurrency note: deriving a key is pure with respect to repository state.
//! Coordinators must still synchronize graph snapshots, cache writes, and
//! invalidation application as required by their transaction model.
//!
//! FREEZE CRITERIA
//!
//! - Equivalent inputs produce the same key regardless of dependency order.
//! - Changing any relevant input changes the derived key.
//! - Duplicate dependency identities are rejected.
//! - Missing required fingerprints cannot be silently defaulted.
//! - No process-local IDs, randomized hashers, or platform byte order affect
//!   the result.
//! - Unit and integration tests pass in the actual crate.
//!
//! This module intentionally does not use `DefaultHasher`, Rust's `Hash`
//! trait as an encoding protocol, `unsafe`, or a fixed dependency-count limit.

use core::fmt;
use std::error::Error;

use crate::zuir::fingerprint::stable_hash::{
    Fingerprint,
    FingerprintError,
    StableHashBuilder,
    FINGERPRINT_ALGORITHM,
    FINGERPRINT_FORMAT_VERSION,
};
use crate::zuir::incremental::dependencies::DependencyKey;
use crate::zuir::version::{ZuirFormatVersion, ZuirSchemaVersion};

/// Versioned domain for cache-key derivation.
///
/// Change this domain only as part of an explicit cache-key schema evolution.
/// It is intentionally independent of target hardware and artifact contents.
pub const CACHE_KEY_DOMAIN: &str = "zuir.incremental.cache-key.v1";

/// A deterministic cache lookup identity.
///
/// The key is derived from canonical, explicitly supplied compilation inputs.
/// It is not an artifact's content fingerprint, a dependency-graph generation,
/// or proof that a cached artifact is valid.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CacheKey(Fingerprint);

impl CacheKey {
    /// Return the underlying stable fingerprint.
    #[must_use]
    pub const fn fingerprint(self) -> Fingerprint {
        self.0
    }

    /// Return the key as lowercase hexadecimal.
    #[must_use]
    pub fn to_hex(self) -> String {
        self.0.to_hex()
    }

    /// Derive a key from a complete, explicit input description.
    ///
    /// Dependency inputs are sorted by logical key before hashing. Therefore,
    /// callers may collect dependencies in any order without changing the key.
    ///
    /// Duplicate dependency keys are rejected, even if their fingerprints
    /// happen to match. This avoids ambiguous dependency inventories and
    /// catches common dependency-collection bugs.
    pub fn derive(input: &CacheKeyInput<'_>) -> Result<Self, CacheKeyError> {
        validate_nonempty("artifact kind", input.artifact_kind)?;
        validate_nonempty("compiler identity", input.compiler_identity)?;

        let mut dependencies: Vec<&DependencyFingerprint> =
            input.dependencies.iter().collect();

        dependencies.sort_unstable_by(|left, right| {
            left.key.cmp(&right.key)
        });

        for pair in dependencies.windows(2) {
            if pair[0].key == pair[1].key {
                return Err(CacheKeyError::DuplicateDependency {
                    key: pair[0].key.clone(),
                });
            }
        }

        let dependency_count =
            u64::try_from(dependencies.len()).map_err(|_| {
                CacheKeyError::InputCountOverflow {
                    input: "dependency",
                }
            })?;

        let mut hash = StableHashBuilder::new(CACHE_KEY_DOMAIN)?;

        // Bind the key to the fingerprint protocol so an algorithm migration
        // cannot accidentally share the same cache namespace.
        hash.field_str("fingerprint_algorithm", FINGERPRINT_ALGORITHM)?;
        hash.field_u32(
            "fingerprint_format_version",
            FINGERPRINT_FORMAT_VERSION,
        )?;

        hash.field_str("artifact_kind", input.artifact_kind)?;
        hash.field_str(
            "artifact_identity",
            input.artifact_identity.as_str(),
        )?;
        hash.field_str("compiler_identity", input.compiler_identity)?;

        hash.field_u32("schema_major", input.schema_version.major())?;
        hash.field_u32("schema_minor", input.schema_version.minor())?;
        hash.field_u32("schema_patch", input.schema_version.patch())?;

        hash.field_u32("format_major", input.format_version.major())?;
        hash.field_u32("format_minor", input.format_version.minor())?;
        hash.field_u32("format_patch", input.format_version.patch())?;

        hash.field_fingerprint("direct_input", input.direct_input)?;
        hash.field_fingerprint(
            "configuration",
            input.configuration_fingerprint,
        )?;
        hash.field_fingerprint(
            "pass_pipeline",
            input.pass_pipeline_fingerprint,
        )?;
        hash.field_fingerprint(
            "semantic_environment",
            input.semantic_environment_fingerprint,
        )?;

        match input.target_fingerprint {
            Some(target) => {
                hash.field_bool("target_specific", true)?;
                hash.field_fingerprint("target", target)?;
            }
            None => {
                hash.field_bool("target_specific", false)?;
            }
        }

        hash.field_u64("dependency_count", dependency_count)?;

        for dependency in dependencies {
            // The identity and fingerprint are separately framed fields.
            // A key cannot be confused with another key or with digest bytes.
            hash.field_str(
                "dependency_identity",
                dependency.key.as_str(),
            )?;
            hash.field_fingerprint(
                "dependency_fingerprint",
                dependency.fingerprint,
            )?;
        }

        Ok(Self(hash.finish()))
    }
}

impl fmt::Debug for CacheKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("CacheKey")
            .field(&self.to_hex())
            .finish()
    }
}

impl fmt::Display for CacheKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_hex())
    }
}

/// A dependency's stable identity and current content fingerprint.
///
/// The identity identifies the logical prerequisite; the fingerprint
/// identifies the relevant content/version of that prerequisite. They are
/// intentionally separate because dependency keys are not content hashes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DependencyFingerprint {
    key: DependencyKey,
    fingerprint: Fingerprint,
}

impl DependencyFingerprint {
    /// Construct a dependency fingerprint from its logical key and digest.
    #[must_use]
    pub const fn new(
        key: DependencyKey,
        fingerprint: Fingerprint,
    ) -> Self {
        Self { key, fingerprint }
    }

    /// Return the logical dependency identity.
    #[must_use]
    pub fn key(&self) -> &DependencyKey {
        &self.key
    }

    /// Return the dependency's content fingerprint.
    #[must_use]
    pub const fn fingerprint(&self) -> Fingerprint {
        self.fingerprint
    }
}

/// Complete inputs required to derive one cache key.
///
/// All fingerprints are explicit and mandatory except `target_fingerprint`,
/// which must be `None` only when the artifact is genuinely target-independent.
///
/// `direct_input` fingerprints the artifact's direct input payload, not the
/// output artifact being cached. Upstream artifacts are supplied separately
/// through `dependencies`.
///
/// The caller is responsible for ensuring that each fingerprint covers all
/// relevant fields for its domain. For example, the semantic-environment
/// fingerprint should include the dialect manifests, operation schemas, and
/// semantic extensions that can affect the artifact.
#[derive(Debug, Clone)]
pub struct CacheKeyInput<'a> {
    /// Extensible, qualified artifact category such as `zuir.module` or
    /// `analysis.effects`. This is metadata, not a closed enum.
    pub artifact_kind: &'a str,

    /// Stable logical identity of the artifact being produced.
    pub artifact_identity: &'a DependencyKey,

    /// Stable compiler build/release identity, including any compiler
    /// component versions that affect this artifact.
    pub compiler_identity: &'a str,

    /// Logical ZUIR schema version.
    pub schema_version: ZuirSchemaVersion,

    /// Serialized ZUIR format version where it affects this artifact.
    pub format_version: ZuirFormatVersion,

    /// Fingerprint of the direct input payload.
    pub direct_input: Fingerprint,

    /// Fingerprint of all relevant compilation configuration and policies.
    pub configuration_fingerprint: Fingerprint,

    /// Fingerprint of the selected ordered pass pipeline and pass versions.
    pub pass_pipeline_fingerprint: Fingerprint,

    /// Fingerprint of relevant semantic environment, including applicable
    /// dialect/extension definitions and their versions.
    pub semantic_environment_fingerprint: Fingerprint,

    /// Target identity and relevant target properties for target-specific
    /// artifacts. Use `None` for target-independent artifacts.
    pub target_fingerprint: Option<Fingerprint>,

    /// Upstream artifact identities and fingerprints.
    ///
    /// Input order does not affect the result. Each logical dependency key
    /// must appear at most once.
    pub dependencies: &'a [DependencyFingerprint],
}

/// Errors produced while validating or deriving a cache key.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CacheKeyError {
    /// A required descriptive field was empty or contained control characters.
    InvalidField {
        /// Name of the invalid field.
        field: &'static str,
    },

    /// A logical dependency appears more than once in one input inventory.
    DuplicateDependency {
        /// Repeated dependency identity.
        key: DependencyKey,
    },

    /// The dependency inventory could not be represented by the canonical
    /// count encoding.
    InputCountOverflow {
        /// Name of the count that overflowed.
        input: &'static str,
    },

    /// Stable fingerprint construction failed.
    Fingerprint(FingerprintError),
}

impl fmt::Display for CacheKeyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidField { field } => {
                write!(f, "invalid cache-key field: {field}")
            }
            Self::DuplicateDependency { key } => {
                write!(f, "duplicate cache-key dependency: {key}")
            }
            Self::InputCountOverflow { input } => {
                write!(f, "cache-key {input} count cannot be represented")
            }
            Self::Fingerprint(error) => {
                write!(f, "cache-key fingerprint failed: {error}")
            }
        }
    }
}

impl Error for CacheKeyError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Fingerprint(error) => Some(error),
            Self::InvalidField { .. }
            | Self::DuplicateDependency { .. }
            | Self::InputCountOverflow { .. } => None,
        }
    }
}

impl From<FingerprintError> for CacheKeyError {
    fn from(error: FingerprintError) -> Self {
        Self::Fingerprint(error)
    }
}

fn validate_nonempty(
    field: &'static str,
    value: &str,
) -> Result<(), CacheKeyError> {
    if value.trim().is_empty() || value.chars().any(char::is_control) {
        return Err(CacheKeyError::InvalidField { field });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::zuir::fingerprint::stable_hash::fingerprint;
    use crate::zuir::incremental::dependencies::DependencyKey;
    use crate::zuir::version::{
        CURRENT_ZUIR_FORMAT_VERSION,
        CURRENT_ZUIR_SCHEMA_VERSION,
    };

    fn digest(value: &str) -> Fingerprint {
        fingerprint("zuir.cache-key.test-input", value)
            .expect("test fingerprint")
    }

    fn key(value: &str) -> DependencyKey {
        DependencyKey::new(value).expect("valid dependency key")
    }

    fn input<'a>(
        identity: &'a DependencyKey,
        dependencies: &'a [DependencyFingerprint],
    ) -> CacheKeyInput<'a> {
        CacheKeyInput {
            artifact_kind: "zuir.test-artifact",
            artifact_identity: identity,
            compiler_identity: "zamani-test-build",
            schema_version: CURRENT_ZUIR_SCHEMA_VERSION,
            format_version: CURRENT_ZUIR_FORMAT_VERSION,
            direct_input: digest("direct-input"),
            configuration_fingerprint: digest("configuration"),
            pass_pipeline_fingerprint: digest("pass-pipeline"),
            semantic_environment_fingerprint: digest("semantic-environment"),
            target_fingerprint: None,
            dependencies,
        }
    }

    #[test]
    fn identical_inputs_produce_identical_keys() {
        let identity = key("zuir:module:main");
        let dependencies = vec![
            DependencyFingerprint::new(key("source:main.zm"), digest("source")),
            DependencyFingerprint::new(key("semantic:module:math"), digest("math")),
        ];

        let first = CacheKey::derive(&input(&identity, &dependencies))
            .expect("derive first");
        let second = CacheKey::derive(&input(&identity, &dependencies))
            .expect("derive second");

        assert_eq!(first, second);
        assert_eq!(first.to_hex().len(), 64);
    }

    #[test]
    fn dependency_order_does_not_change_key() {
        let identity = key("zuir:module:main");
        let first_dependencies = vec![
            DependencyFingerprint::new(key("source:b.zm"), digest("b")),
            DependencyFingerprint::new(key("source:a.zm"), digest("a")),
        ];
        let second_dependencies = vec![
            DependencyFingerprint::new(key("source:a.zm"), digest("a")),
            DependencyFingerprint::new(key("source:b.zm"), digest("b")),
        ];

        let first = CacheKey::derive(&input(&identity, &first_dependencies))
            .expect("derive first");
        let second = CacheKey::derive(&input(&identity, &second_dependencies))
            .expect("derive second");

        assert_eq!(first, second);
    }

    #[test]
    fn changed_dependency_fingerprint_changes_key() {
        let identity = key("zuir:module:main");
        let first_dependencies = vec![DependencyFingerprint::new(
            key("source:main.zm"),
            digest("old-source"),
        )];
        let second_dependencies = vec![DependencyFingerprint::new(
            key("source:main.zm"),
            digest("new-source"),
        )];

        let first = CacheKey::derive(&input(&identity, &first_dependencies))
            .expect("derive first");
        let second = CacheKey::derive(&input(&identity, &second_dependencies))
            .expect("derive second");

        assert_ne!(first, second);
    }

    #[test]
    fn duplicate_dependency_identity_is_rejected() {
        let identity = key("zuir:module:main");
        let dependencies = vec![
            DependencyFingerprint::new(key("source:main.zm"), digest("one")),
            DependencyFingerprint::new(key("source:main.zm"), digest("two")),
        ];

        assert!(matches!(
            CacheKey::derive(&input(&identity, &dependencies)),
            Err(CacheKeyError::DuplicateDependency { .. })
        ));
    }

    #[test]
    fn configuration_changes_key() {
        let identity = key("zuir:module:main");
        let dependencies = Vec::new();
        let base = input(&identity, &dependencies);
        let original =
            CacheKey::derive(&base).expect("derive original");

        let changed = CacheKeyInput {
            configuration_fingerprint: digest("different-configuration"),
            ..base
        };
        let changed_key =
            CacheKey::derive(&changed).expect("derive changed");

        assert_ne!(original, changed_key);
    }

    #[test]
    fn target_specific_and_portable_keys_are_distinct() {
        let identity = key("zuir:module:main");
        let dependencies = Vec::new();
        let portable_input = input(&identity, &dependencies);
        let portable =
            CacheKey::derive(&portable_input).expect("portable key");

        let target_input = CacheKeyInput {
            target_fingerprint: Some(digest("target:example")),
            ..portable_input
        };
        let target =
            CacheKey::derive(&target_input).expect("target key");

        assert_ne!(portable, target);
    }

    #[test]
    fn empty_artifact_kind_is_rejected() {
        let identity = key("zuir:module:main");
        let dependencies = Vec::new();
        let invalid = CacheKeyInput {
            artifact_kind: " ",
            ..input(&identity, &dependencies)
        };

        assert!(matches!(
            CacheKey::derive(&invalid),
            Err(CacheKeyError::InvalidField {
                field: "artifact kind"
            })
        ));
    }

    #[test]
    fn artifact_identity_is_part_of_the_key() {
        let first_identity = key("zuir:module:first");
        let second_identity = key("zuir:module:second");
        let dependencies = Vec::new();

        let first =
            CacheKey::derive(&input(&first_identity, &dependencies))
                .expect("first");
        let second =
            CacheKey::derive(&input(&second_identity, &dependencies))
                .expect("second");

        assert_ne!(first, second);
    }
}
