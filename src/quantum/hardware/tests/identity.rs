//! Zamani Quantum Hardware — Identity Conformance Tests
//!
//! Production-grade conformance and integration tests for
//! `crate::quantum::hardware::identity`.
//!
//! # Responsibility
//!
//! This module verifies the public, provider-neutral identity contract used
//! throughout the Zamani quantum hardware abstraction layer.
//!
//! It verifies:
//!
//! - provider identities;
//! - hardware identities;
//! - device identities;
//! - backend identities;
//! - architecture identities;
//! - firmware versions;
//! - hardware revisions;
//! - namespace-qualified identities;
//! - complete hardware identity descriptors;
//! - backend identity descriptors;
//! - hardware identity references;
//! - validation boundaries;
//! - canonical representations;
//! - parsing;
//! - `Display`;
//! - `FromStr`;
//! - Serde serialization/deserialization;
//! - deterministic equality;
//! - deterministic ordering;
//! - deterministic hashing;
//! - `Send`/`Sync` suitability;
//! - security-related delimiter rejection;
//! - path-traversal rejection;
//! - absence of implicit normalization;
//! - documented integration examples;
//! - public API compatibility.
//!
//! # Architectural boundary
//!
//! This test file intentionally depends only on the public identity API.
//!
//! It does NOT depend on:
//!
//! - `backend.rs`;
//! - `backend_trait.rs`;
//! - `capabilities.rs`;
//! - `technology.rs`;
//! - `topology.rs`;
//! - `calibration.rs`;
//! - `execution.rs`;
//! - `job.rs`;
//! - `provider.rs`;
//! - provider adapters;
//! - benchmarking;
//! - Danga;
//! - network transports;
//! - credentials;
//! - authentication;
//! - quantum IR.
//!
//! This makes the identity contract independently testable and prevents
//! higher-level implementation changes from forcing changes here.
//!
//! # Integration contract
//!
//! `identity.rs` is a foundational module. Higher-level hardware modules
//! consume its public types.
//!
//! The expected dependency direction is:
//!
//! ```text
//! identity
//!    │
//!    ├── technology
//!    ├── capabilities
//!    ├── timing
//!    ├── instruction_set
//!    ├── topology
//!    ├── calibration
//!    ├── backend
//!    ├── provider
//!    ├── registries
//!    └── execution
//! ```
//!
//! This test suite therefore treats the public identity API as stable and
//! does not reach into private fields.
//!
//! # Mounting
//!
//! This file is designed to live at:
//!
//! `src/quantum/hardware/tests/identity.rs`
//!
//! The hardware module can mount it with:
//!
//! ```text
//! #[cfg(test)]
//! #[path = "tests/identity.rs"]
//! mod identity_tests;
//! ```
//!
//! No production identity implementation should be modified merely to make
//! this test suite compile.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - edition 2021.
//!
//! No nightly features are used.
//!
//! # Security
//!
//! Identity values are identifiers, not credential containers. Tests therefore
//! ensure that URL syntax, authority delimiters, query strings, fragments,
//! path traversal components, and other ambiguous forms cannot enter the
//! strongly typed identity APIs.
//!
//! These tests are defense-in-depth checks. They do not replace the dedicated
//! credentials or authentication subsystems.
//!
//! # Reproducibility
//!
//! Identity values are expected to be deterministic and suitable for:
//!
//! - cache keys;
//! - audit records;
//! - execution provenance;
//! - benchmark provenance;
//! - backend registries;
//! - persistence;
//! - cross-process communication.
//!
//! The tests consequently require stable string, ordering, equality, hashing,
//! and serialization semantics.

use std::collections::{BTreeSet, HashSet};
use std::hash::{Hash, Hasher};
use std::str::FromStr;

use crate::quantum::hardware::identity::{
    ArchitectureId,
    BackendId,
    BackendIdentity,
    DeviceId,
    FirmwareVersion,
    HardwareId,
    HardwareIdentity,
    HardwareIdentityRef,
    HardwareRevision,
    IdentityError,
    IdentityNamespace,
    ProviderId,
    QualifiedIdentity,
    MAX_IDENTITY_LENGTH,
    MAX_NAMESPACE_LENGTH,
    MAX_REVISION_LENGTH,
    MAX_VERSION_LENGTH,
};

// =============================================================================
// Test helpers
// =============================================================================

/// Build a complete, representative physical-hardware identity.
fn complete_hardware_identity() -> HardwareIdentity {
    HardwareIdentity::builder()
        .provider(ProviderId::new("ibm").expect("valid provider"))
        .hardware(HardwareId::new("ibm_torino").expect("valid hardware"))
        .device(DeviceId::new("ibm_torino").expect("valid device"))
        .architecture(
            ArchitectureId::new("ibm-heron-r2")
                .expect("valid architecture"),
        )
        .firmware(
            FirmwareVersion::new("1.2.3")
                .expect("valid firmware"),
        )
        .revision(
            HardwareRevision::new("A0")
                .expect("valid revision"),
        )
        .build()
        .expect("complete hardware identity")
}

/// Build a representative executable backend identity.
fn complete_backend_identity() -> BackendIdentity {
    BackendIdentity::new(
        ProviderId::new("ibm").expect("valid provider"),
        BackendId::new("ibm_torino_runtime")
            .expect("valid backend"),
        DeviceId::new("ibm_torino")
            .expect("valid device"),
        ArchitectureId::new("ibm-heron-r2")
            .expect("valid architecture"),
    )
}

/// Build a representative hardware identity reference.
fn complete_identity_ref() -> HardwareIdentityRef {
    HardwareIdentityRef::new(
        ProviderId::new("ibm").expect("valid provider"),
        HardwareId::new("ibm_torino").expect("valid hardware"),
        DeviceId::new("ibm_torino").expect("valid device"),
        BackendId::new("ibm_torino_runtime")
            .expect("valid backend"),
    )
}

/// Compile-time assertion that a type is both `Send` and `Sync`.
fn assert_send_sync<T: Send + Sync>() {}

/// Verify that equal identity values produce equal hashes.
fn assert_hash_consistent<T>(left: &T, right: &T)
where
    T: Hash,
{
    let mut left_hasher =
        std::collections::hash_map::DefaultHasher::new();
    let mut right_hasher =
        std::collections::hash_map::DefaultHasher::new();

    left.hash(&mut left_hasher);
    right.hash(&mut right_hasher);

    assert_eq!(
        left_hasher.finish(),
        right_hasher.finish(),
        "equal identity values must hash identically"
    );
}

// =============================================================================
// Compile-time API and concurrency contract
// =============================================================================

#[test]
fn identity_public_types_are_send_and_sync() {
    assert_send_sync::<IdentityNamespace>();
    assert_send_sync::<QualifiedIdentity>();

    assert_send_sync::<ProviderId>();
    assert_send_sync::<HardwareId>();
    assert_send_sync::<DeviceId>();
    assert_send_sync::<BackendId>();
    assert_send_sync::<ArchitectureId>();

    assert_send_sync::<FirmwareVersion>();
    assert_send_sync::<HardwareRevision>();

    assert_send_sync::<HardwareIdentity>();
    assert_send_sync::<BackendIdentity>();
    assert_send_sync::<HardwareIdentityRef>();
}

// =============================================================================
// Namespace
// =============================================================================

#[test]
fn namespace_accepts_canonical_values() {
    for value in [
        "local",
        "provider",
        "simulator",
        "emulator",
        "custom",
        "aws-braket",
        "provider_2",
        "provider2",
    ] {
        let namespace =
            IdentityNamespace::new(value).unwrap();

        assert_eq!(namespace.as_str(), value);
        assert_eq!(namespace.to_string(), value);
    }
}

#[test]
fn namespace_convenience_values_are_canonical() {
    assert_eq!(
        IdentityNamespace::local().as_str(),
        "local"
    );

    assert_eq!(
        IdentityNamespace::provider().as_str(),
        "provider"
    );

    assert_eq!(
        IdentityNamespace::default().as_str(),
        "local"
    );
}

#[test]
fn namespace_rejects_empty_values() {
    assert!(matches!(
        IdentityNamespace::new(""),
        Err(IdentityError::InvalidNamespace { .. })
    ));
}

#[test]
fn namespace_rejects_uppercase_values() {
    assert!(
        IdentityNamespace::new("Provider").is_err()
    );
}

#[test]
fn namespace_rejects_whitespace() {
    for value in [
        " provider",
        "provider ",
        "\tprovider",
        "provider\n",
    ] {
        assert!(
            IdentityNamespace::new(value).is_err(),
            "namespace must reject {value:?}"
        );
    }
}

#[test]
fn namespace_rejects_hierarchical_delimiters() {
    for value in [
        "provider/foo",
        "provider://foo",
        "provider:foo",
        "provider.foo",
    ] {
        assert!(
            IdentityNamespace::new(value).is_err(),
            "namespace must reject {value:?}"
        );
    }
}

#[test]
fn namespace_length_limit_is_enforced() {
    let accepted =
        "n".repeat(MAX_NAMESPACE_LENGTH);

    let rejected =
        "n".repeat(MAX_NAMESPACE_LENGTH + 1);

    assert!(
        IdentityNamespace::new(accepted).is_ok()
    );

    assert!(matches!(
        IdentityNamespace::new(rejected),
        Err(IdentityError::TooLong {
            field: "namespace",
            ..
        })
    ));
}

#[test]
fn namespace_supports_from_str() {
    let constructed =
        IdentityNamespace::new("provider").unwrap();

    let parsed =
        IdentityNamespace::from_str("provider")
            .unwrap();

    assert_eq!(constructed, parsed);
}

#[test]
fn namespace_supports_serde_round_trip() {
    let namespace =
        IdentityNamespace::new("provider").unwrap();

    let encoded =
        serde_json::to_string(&namespace).unwrap();

    assert_eq!(encoded, "\"provider\"");

    let decoded: IdentityNamespace =
        serde_json::from_str(&encoded).unwrap();

    assert_eq!(namespace, decoded);
}

#[test]
fn namespace_serde_rejects_invalid_values() {
    assert!(
        serde_json::from_str::<IdentityNamespace>(
            "\"Provider\""
        )
        .is_err()
    );

    assert!(
        serde_json::from_str::<IdentityNamespace>(
            "\"provider/foo\""
        )
        .is_err()
    );

    assert!(
        serde_json::from_str::<IdentityNamespace>(
            "123"
        )
        .is_err()
    );
}

// =============================================================================
// Strongly typed identifiers
// =============================================================================

#[test]
fn all_simple_identity_types_accept_common_hardware_names() {
    assert_eq!(
        ProviderId::new("ibm").unwrap().as_str(),
        "ibm"
    );

    assert_eq!(
        HardwareId::new("ibm_torino")
            .unwrap()
            .as_str(),
        "ibm_torino"
    );

    assert_eq!(
        DeviceId::new("ibm_torino")
            .unwrap()
            .as_str(),
        "ibm_torino"
    );

    assert_eq!(
        BackendId::new("ibm_torino_runtime")
            .unwrap()
            .as_str(),
        "ibm_torino_runtime"
    );

    assert_eq!(
        ArchitectureId::new("ibm-heron-r2")
            .unwrap()
            .as_str(),
        "ibm-heron-r2"
    );
}

#[test]
fn local_provider_is_stable() {
    let provider = ProviderId::local();

    assert_eq!(provider.as_str(), "local");
    assert_eq!(provider.to_string(), "local");
    assert_eq!(
        provider,
        ProviderId::new("local").unwrap()
    );
}

#[test]
fn simple_identity_types_reject_empty_values() {
    assert!(matches!(
        ProviderId::new(""),
        Err(IdentityError::Empty)
    ));

    assert!(matches!(
        HardwareId::new(""),
        Err(IdentityError::Empty)
    ));

    assert!(matches!(
        DeviceId::new(""),
        Err(IdentityError::Empty)
    ));

    assert!(matches!(
        BackendId::new(""),
        Err(IdentityError::Empty)
    ));

    assert!(matches!(
        ArchitectureId::new(""),
        Err(IdentityError::Empty)
    ));
}

#[test]
fn simple_identity_types_reject_surrounding_whitespace() {
    for value in [
        " ibm",
        "ibm ",
        "\tibm",
        "ibm\n",
    ] {
        assert!(matches!(
            ProviderId::new(value),
            Err(IdentityError::SurroundingWhitespace {
                field: "provider ID"
            })
        ));
    }
}

#[test]
fn simple_identity_types_reject_internal_whitespace() {
    for value in [
        "ibm quantum",
        "ibm\tquantum",
        "ibm\nquantum",
    ] {
        assert!(
            ProviderId::new(value).is_err(),
            "identity must reject internal whitespace: {value:?}"
        );
    }
}

#[test]
fn simple_identity_types_reject_unsafe_delimiters() {
    for value in [
        "ibm/provider",
        "ibm://secret",
        "ibm:secret",
        "ibm?token=secret",
        "ibm#fragment",
        "ibm%2Fsecret",
        "ibm\\secret",
        "ibm@host",
        "ibm$secret",
    ] {
        assert!(
            ProviderId::new(value).is_err(),
            "identity must reject unsafe/delimiter value: {value:?}"
        );
    }
}

#[test]
fn simple_identity_types_reject_unicode() {
    for value in [
        "ibm-量子",
        "é",
        "π",
        "замани",
    ] {
        assert!(
            ProviderId::new(value).is_err(),
            "identity syntax must remain ASCII-safe: {value:?}"
        );
    }
}

#[test]
fn simple_identity_types_enforce_maximum_length() {
    let accepted =
        "a".repeat(MAX_IDENTITY_LENGTH);

    let rejected =
        "a".repeat(MAX_IDENTITY_LENGTH + 1);

    assert!(
        BackendId::new(accepted).is_ok()
    );

    assert!(matches!(
        BackendId::new(rejected),
        Err(IdentityError::TooLong {
            field: "backend ID",
            length,
            maximum: MAX_IDENTITY_LENGTH,
        }) if length == MAX_IDENTITY_LENGTH + 1
    ));
}

#[test]
fn simple_identity_types_report_length_and_empty_state() {
    let id =
        BackendId::new("abc-123").unwrap();

    assert!(!id.is_empty());
    assert_eq!(id.len(), 7);
}

#[test]
fn simple_identity_types_support_from_str() {
    let expected =
        BackendId::new("local-statevector")
            .unwrap();

    let parsed: BackendId =
        "local-statevector"
            .parse()
            .unwrap();

    assert_eq!(parsed, expected);
}

#[test]
fn simple_identity_types_support_display() {
    let id =
        BackendId::new("local-statevector")
            .unwrap();

    assert_eq!(
        id.to_string(),
        "local-statevector"
    );
}

#[test]
fn simple_identity_types_have_value_semantics() {
    let left =
        BackendId::new("backend-a").unwrap();

    let right =
        BackendId::new("backend-a").unwrap();

    let other =
        BackendId::new("backend-b").unwrap();

    assert_eq!(left, right);
    assert_ne!(left, other);
    assert!(left < other);

    assert_hash_consistent(&left, &right);
}

#[test]
fn simple_identity_types_work_in_ordered_sets() {
    let mut ordered = BTreeSet::new();

    ordered.insert(
        BackendId::new("backend-b").unwrap()
    );

    ordered.insert(
        BackendId::new("backend-a").unwrap()
    );

    ordered.insert(
        BackendId::new("backend-a").unwrap()
    );

    let values: Vec<&str> =
        ordered
            .iter()
            .map(BackendId::as_str)
            .collect();

    assert_eq!(
        values,
        vec!["backend-a", "backend-b"]
    );
}

#[test]
fn simple_identity_types_work_in_hashed_sets() {
    let mut hashed = HashSet::new();

    hashed.insert(
        BackendId::new("backend-a").unwrap()
    );

    hashed.insert(
        BackendId::new("backend-a").unwrap()
    );

    assert_eq!(hashed.len(), 1);
}

// =============================================================================
// Qualified identity
// =============================================================================

#[test]
fn qualified_identity_accepts_hierarchical_provider_paths() {
    let identity =
        QualifiedIdentity::with_namespace(
            "provider",
            "ibm/ibm_torino",
        )
        .unwrap();

    assert_eq!(
        identity.namespace().as_str(),
        "provider"
    );

    assert_eq!(
        identity.value(),
        "ibm/ibm_torino"
    );

    assert_eq!(
        identity.as_str(),
        "provider://ibm/ibm_torino"
    );

    assert_eq!(
        identity.to_string(),
        "provider://ibm/ibm_torino"
    );
}

#[test]
fn qualified_identity_round_trips_through_from_str() {
    let original =
        QualifiedIdentity::with_namespace(
            "provider",
            "aws-braket/device-01",
        )
        .unwrap();

    let encoded =
        original.to_string();

    let parsed: QualifiedIdentity =
        encoded.parse().unwrap();

    assert_eq!(original, parsed);
    assert_hash_consistent(&original, &parsed);
}

#[test]
fn qualified_identity_requires_scheme_separator() {
    for value in [
        "provider/ibm",
        "provider:ibm",
        "provider",
    ] {
        assert!(matches!(
            QualifiedIdentity::parse(value),
            Err(IdentityError::InvalidQualifiedIdentity {
                ..
            })
        ));
    }
}

#[test]
fn qualified_identity_rejects_empty_input() {
    assert!(matches!(
        QualifiedIdentity::parse(""),
        Err(IdentityError::InvalidQualifiedIdentity {
            ..
        })
    ));
}

#[test]
fn qualified_identity_rejects_empty_components() {
    for value in [
        "provider://",
        "provider:///ibm",
        "provider://ibm//torino",
        "provider://ibm/",
    ] {
        assert!(
            QualifiedIdentity::parse(value).is_err(),
            "qualified identity must reject {value:?}"
        );
    }
}

#[test]
fn qualified_identity_rejects_path_traversal() {
    for value in [
        "provider://../secret",
        "provider://ibm/../secret",
        "provider://ibm/./secret",
        "provider://./secret",
    ] {
        assert!(
            QualifiedIdentity::parse(value).is_err(),
            "path traversal must be rejected: {value:?}"
        );
    }
}

#[test]
fn qualified_identity_rejects_credential_like_syntax() {
    for value in [
        "provider://ibm:secret/torino",
        "provider://ibm?api_key=secret",
        "provider://ibm#secret/torino",
        "provider://ibm\\secret/torino",
    ] {
        assert!(
            QualifiedIdentity::parse(value).is_err(),
            "credential/URL-like identity must be rejected: {value:?}"
        );
    }
}

#[test]
fn qualified_identity_rejects_invalid_namespaces() {
    assert!(
        QualifiedIdentity::with_namespace(
            "Provider",
            "ibm/torino"
        )
        .is_err()
    );

    assert!(
        QualifiedIdentity::with_namespace(
            "provider/foo",
            "ibm/torino"
        )
        .is_err()
    );

    assert!(
        QualifiedIdentity::with_namespace(
            "provider",
            " ibm/torino"
        )
        .is_err()
    );
}

#[test]
fn qualified_identity_is_orderable_and_hashable() {
    let first =
        QualifiedIdentity::with_namespace(
            "provider",
            "ibm/a",
        )
        .unwrap();

    let second =
        QualifiedIdentity::with_namespace(
            "provider",
            "ibm/b",
        )
        .unwrap();

    assert!(first < second);
    assert_ne!(first, second);
}

#[test]
fn qualified_identity_serializes_canonically() {
    let identity =
        QualifiedIdentity::with_namespace(
            "provider",
            "ibm/ibm_torino",
        )
        .unwrap();

    let encoded =
        serde_json::to_string(&identity)
            .unwrap();

    assert_eq!(
        encoded,
        "\"provider://ibm/ibm_torino\""
    );
}

#[test]
fn qualified_identity_serde_round_trip_preserves_value() {
    let identity =
        QualifiedIdentity::with_namespace(
            "provider",
            "ibm/ibm_torino",
        )
        .unwrap();

    let encoded =
        serde_json::to_string(&identity)
            .unwrap();

    let decoded: QualifiedIdentity =
        serde_json::from_str(&encoded)
            .unwrap();

    assert_eq!(identity, decoded);
}

#[test]
fn qualified_identity_serde_rejects_invalid_value() {
    assert!(
        serde_json::from_str::<QualifiedIdentity>(
            "\"provider://ibm/../torino\""
        )
        .is_err()
    );
}

#[test]
fn qualified_identity_serde_rejects_non_string_values() {
    assert!(
        serde_json::from_str::<QualifiedIdentity>(
            "123"
        )
        .is_err()
    );

    assert!(
        serde_json::from_str::<QualifiedIdentity>(
            "null"
        )
        .is_err()
    );

    assert!(
        serde_json::from_str::<QualifiedIdentity>(
            "[]"
        )
        .is_err()
    );
}

// =============================================================================
// Firmware versions
// =============================================================================

#[test]
fn firmware_version_accepts_provider_defined_forms() {
    for value in [
        "1.0.0",
        "v1.2.3",
        "2026.08",
        "1.0.0-rc1",
        "2026.08.27-build7",
        "firmware_2026-08",
        "build+2026.08",
    ] {
        assert!(
            FirmwareVersion::new(value).is_ok(),
            "firmware version should be accepted: {value:?}"
        );
    }
}

#[test]
fn firmware_version_rejects_invalid_values() {
    for value in [
        "",
        " 1.0.0",
        "1.0.0 ",
        "1/0/0",
        "1:0:0",
        "1?token",
        "1#fragment",
        "1\\0\\0",
    ] {
        assert!(
            FirmwareVersion::new(value).is_err(),
            "firmware version must reject: {value:?}"
        );
    }
}

#[test]
fn firmware_version_length_limit_is_enforced() {
    let accepted =
        "a".repeat(MAX_VERSION_LENGTH);

    let rejected =
        "a".repeat(MAX_VERSION_LENGTH + 1);

    assert!(
        FirmwareVersion::new(accepted).is_ok()
    );

    assert!(matches!(
        FirmwareVersion::new(rejected),
        Err(IdentityError::TooLong {
            field: "version",
            ..
        })
    ));
}

#[test]
fn firmware_version_semver_helper_is_non_destructive() {
    let version =
        FirmwareVersion::new("v1.2.3")
            .unwrap();

    assert_eq!(
        version.as_str(),
        "v1.2.3"
    );

    assert_eq!(
        version.semver_components(),
        Some((1, 2, 3))
    );
}

#[test]
fn firmware_version_semver_helper_is_optional() {
    assert_eq!(
        FirmwareVersion::new("2026.08")
            .unwrap()
            .semver_components(),
        None
    );

    assert_eq!(
        FirmwareVersion::new("build-2026")
            .unwrap()
            .semver_components(),
        None
    );
}

#[test]
fn firmware_version_supports_from_str() {
    let expected =
        FirmwareVersion::new("1.2.3")
            .unwrap();

    let parsed: FirmwareVersion =
        "1.2.3".parse().unwrap();

    assert_eq!(expected, parsed);
}

#[test]
fn firmware_version_supports_display() {
    let version =
        FirmwareVersion::new("1.2.3")
            .unwrap();

    assert_eq!(
        version.to_string(),
        "1.2.3"
    );
}

#[test]
fn firmware_version_round_trips_through_serde() {
    let version =
        FirmwareVersion::new(
            "2026.08.27-build7",
        )
        .unwrap();

    let encoded =
        serde_json::to_string(&version)
            .unwrap();

    let decoded: FirmwareVersion =
        serde_json::from_str(&encoded)
            .unwrap();

    assert_eq!(version, decoded);
    assert_eq!(
        encoded,
        "\"2026.08.27-build7\""
    );
}

// =============================================================================
// Hardware revisions
// =============================================================================

#[test]
fn hardware_revision_accepts_common_provider_forms() {
    for value in [
        "A0",
        "B1",
        "rev-2",
        "v3",
        "gen4",
        "chip-2026-01",
    ] {
        assert!(
            HardwareRevision::new(value).is_ok(),
            "hardware revision should be accepted: {value:?}"
        );
    }
}

#[test]
fn hardware_revision_rejects_invalid_values() {
    for value in [
        "",
        " A0",
        "A0 ",
        "A/0",
        "A:0",
        "A?0",
        "A\\0",
    ] {
        assert!(
            HardwareRevision::new(value).is_err(),
            "hardware revision must reject: {value:?}"
        );
    }
}

#[test]
fn hardware_revision_length_limit_is_enforced() {
    let accepted =
        "r".repeat(MAX_REVISION_LENGTH);

    let rejected =
        "r".repeat(MAX_REVISION_LENGTH + 1);

    assert!(
        HardwareRevision::new(accepted).is_ok()
    );

    assert!(matches!(
        HardwareRevision::new(rejected),
        Err(IdentityError::TooLong {
            field: "hardware revision",
            ..
        })
    ));
}

#[test]
fn hardware_revision_supports_serde_round_trip() {
    let revision =
        HardwareRevision::new("chip-2026-01")
            .unwrap();

    let encoded =
        serde_json::to_string(&revision)
            .unwrap();

    let decoded: HardwareRevision =
        serde_json::from_str(&encoded)
            .unwrap();

    assert_eq!(revision, decoded);
}

// =============================================================================
// Complete hardware identity
// =============================================================================

#[test]
fn complete_hardware_identity_exposes_every_component() {
    let identity =
        complete_hardware_identity();

    assert_eq!(
        identity.provider().as_str(),
        "ibm"
    );

    assert_eq!(
        identity.hardware().as_str(),
        "ibm_torino"
    );

    assert_eq!(
        identity.device().as_str(),
        "ibm_torino"
    );

    assert_eq!(
        identity.architecture().as_str(),
        "ibm-heron-r2"
    );

    assert_eq!(
        identity.firmware().as_str(),
        "1.2.3"
    );

    assert_eq!(
        identity.revision().as_str(),
        "A0"
    );
}

#[test]
fn complete_hardware_identity_requires_provider() {
    let result =
        HardwareIdentity::builder()
            .hardware(
                HardwareId::new("ibm_torino")
                    .unwrap(),
            )
            .device(
                DeviceId::new("ibm_torino")
                    .unwrap(),
            )
            .architecture(
                ArchitectureId::new(
                    "ibm-heron-r2",
                )
                .unwrap(),
            )
            .firmware(
                FirmwareVersion::new("1.2.3")
                    .unwrap(),
            )
            .revision(
                HardwareRevision::new("A0")
                    .unwrap(),
            )
            .build();

    assert_eq!(
        result,
        Err(IdentityError::Empty)
    );
}

#[test]
fn complete_hardware_identity_requires_hardware() {
    let result =
        HardwareIdentity::builder()
            .provider(
                ProviderId::new("ibm")
                    .unwrap(),
            )
            .device(
                DeviceId::new("ibm_torino")
                    .unwrap(),
            )
            .architecture(
                ArchitectureId::new(
                    "ibm-heron-r2",
                )
                .unwrap(),
            )
            .firmware(
                FirmwareVersion::new("1.2.3")
                    .unwrap(),
            )
            .revision(
                HardwareRevision::new("A0")
                    .unwrap(),
            )
            .build();

    assert_eq!(
        result,
        Err(IdentityError::Empty)
    );
}

#[test]
fn complete_hardware_identity_requires_device() {
    let result =
        HardwareIdentity::builder()
            .provider(
                ProviderId::new("ibm")
                    .unwrap(),
            )
            .hardware(
                HardwareId::new("ibm_torino")
                    .unwrap(),
            )
            .architecture(
                ArchitectureId::new(
                    "ibm-heron-r2",
                )
                .unwrap(),
            )
            .firmware(
                FirmwareVersion::new("1.2.3")
                    .unwrap(),
            )
            .revision(
                HardwareRevision::new("A0")
                    .unwrap(),
            )
            .build();

    assert_eq!(
        result,
        Err(IdentityError::Empty)
    );
}

#[test]
fn complete_hardware_identity_requires_architecture() {
    let result =
        HardwareIdentity::builder()
            .provider(
                ProviderId::new("ibm")
                    .unwrap(),
            )
            .hardware(
                HardwareId::new("ibm_torino")
                    .unwrap(),
            )
            .device(
                DeviceId::new("ibm_torino")
                    .unwrap(),
            )
            .firmware(
                FirmwareVersion::new("1.2.3")
                    .unwrap(),
            )
            .revision(
                HardwareRevision::new("A0")
                    .unwrap(),
            )
            .build();

    assert_eq!(
        result,
        Err(IdentityError::Empty)
    );
}

#[test]
fn complete_hardware_identity_requires_firmware() {
    let result =
        HardwareIdentity::builder()
            .provider(
                ProviderId::new("ibm")
                    .unwrap(),
            )
            .hardware(
                HardwareId::new("ibm_torino")
                    .unwrap(),
            )
            .device(
                DeviceId::new("ibm_torino")
                    .unwrap(),
            )
            .architecture(
                ArchitectureId::new(
                    "ibm-heron-r2",
                )
                .unwrap(),
            )
            .revision(
                HardwareRevision::new("A0")
                    .unwrap(),
            )
            .build();

    assert_eq!(
        result,
        Err(IdentityError::Empty)
    );
}

#[test]
fn complete_hardware_identity_requires_revision() {
    let result =
        HardwareIdentity::builder()
            .provider(
                ProviderId::new("ibm")
                    .unwrap(),
            )
            .hardware(
                HardwareId::new("ibm_torino")
                    .unwrap(),
            )
            .device(
                DeviceId::new("ibm_torino")
                    .unwrap(),
            )
            .architecture(
                ArchitectureId::new(
                    "ibm-heron-r2",
                )
                .unwrap(),
            )
            .firmware(
                FirmwareVersion::new("1.2.3")
                    .unwrap(),
            )
            .build();

    assert_eq!(
        result,
        Err(IdentityError::Empty)
    );
}

#[test]
fn complete_hardware_identity_qualified_id_is_deterministic() {
    let identity =
        complete_hardware_identity();

    assert_eq!(
        identity
            .qualified_hardware_id()
            .to_string(),
        "provider://ibm/ibm_torino"
    );
}

#[test]
fn complete_hardware_identity_provenance_key_is_deterministic() {
    let identity =
        complete_hardware_identity();

    assert_eq!(
        identity.provenance_key(),
        "provider=ibm;hardware=ibm_torino;device=ibm_torino;architecture=ibm-heron-r2;firmware=1.2.3;revision=A0"
    );
}

#[test]
fn complete_hardware_identity_has_value_semantics() {
    let first =
        complete_hardware_identity();

    let second =
        complete_hardware_identity();

    assert_eq!(first, second);
    assert_hash_consistent(&first, &second);
}

#[test]
fn complete_hardware_identity_round_trips_through_serde() {
    let identity =
        complete_hardware_identity();

    let encoded =
        serde_json::to_string(&identity)
            .unwrap();

    let decoded: HardwareIdentity =
        serde_json::from_str(&encoded)
            .unwrap();

    assert_eq!(identity, decoded);

    assert!(
        encoded.contains(
            "\"provider\":\"ibm\""
        )
    );

    assert!(
        encoded.contains(
            "\"hardware\":\"ibm_torino\""
        )
    );

    assert!(
        encoded.contains(
            "\"device\":\"ibm_torino\""
        )
    );

    assert!(
        encoded.contains(
            "\"architecture\":\"ibm-heron-r2\""
        )
    );

    assert!(
        encoded.contains(
            "\"firmware\":\"1.2.3\""
        )
    );

    assert!(
        encoded.contains(
            "\"revision\":\"A0\""
        )
    );
}

// =============================================================================
// Backend identity
// =============================================================================

#[test]
fn backend_identity_separates_backend_from_device() {
    let identity =
        complete_backend_identity();

    assert_eq!(
        identity.provider().as_str(),
        "ibm"
    );

    assert_eq!(
        identity.backend().as_str(),
        "ibm_torino_runtime"
    );

    assert_eq!(
        identity.device().as_str(),
        "ibm_torino"
    );

    assert_eq!(
        identity.architecture().as_str(),
        "ibm-heron-r2"
    );
}

#[test]
fn backend_identity_qualified_id_uses_backend_identity() {
    let identity =
        complete_backend_identity();

    assert_eq!(
        identity.qualified().to_string(),
        "provider://ibm/ibm_torino_runtime"
    );
}

#[test]
fn backend_identity_provenance_key_is_deterministic() {
    let identity =
        complete_backend_identity();

    assert_eq!(
        identity.provenance_key(),
        "provider=ibm;backend=ibm_torino_runtime;device=ibm_torino;architecture=ibm-heron-r2"
    );
}

#[test]
fn backend_identity_round_trips_through_serde() {
    let identity =
        complete_backend_identity();

    let encoded =
        serde_json::to_string(&identity)
            .unwrap();

    let decoded: BackendIdentity =
        serde_json::from_str(&encoded)
            .unwrap();

    assert_eq!(identity, decoded);
}

#[test]
fn backend_identity_can_have_same_text_as_hardware_identity_without_type_aliasing() {
    let backend =
        BackendId::new("ibm_torino")
            .unwrap();

    let hardware =
        HardwareId::new("ibm_torino")
            .unwrap();

    assert_eq!(
        backend.as_str(),
        hardware.as_str()
    );

    // The distinct semantic Rust types intentionally prevent accidental
    // interchange at API boundaries.
    assert_send_sync::<BackendId>();
    assert_send_sync::<HardwareId>();
}

// =============================================================================
// Hardware identity reference
// =============================================================================

#[test]
fn hardware_identity_reference_preserves_all_relationships() {
    let reference =
        complete_identity_ref();

    assert_eq!(
        reference.provider().as_str(),
        "ibm"
    );

    assert_eq!(
        reference.hardware().as_str(),
        "ibm_torino"
    );

    assert_eq!(
        reference.device().as_str(),
        "ibm_torino"
    );

    assert_eq!(
        reference.backend().as_str(),
        "ibm_torino_runtime"
    );
}

#[test]
fn hardware_identity_reference_qualified_backend_is_deterministic() {
    let reference =
        complete_identity_ref();

    assert_eq!(
        reference
            .qualified_backend()
            .to_string(),
        "provider://ibm/ibm_torino_runtime"
    );
}

#[test]
fn hardware_identity_reference_round_trips_through_serde() {
    let reference =
        complete_identity_ref();

    let encoded =
        serde_json::to_string(&reference)
            .unwrap();

    let decoded: HardwareIdentityRef =
        serde_json::from_str(&encoded)
            .unwrap();

    assert_eq!(reference, decoded);
}

// =============================================================================
// Serde validation boundary
// =============================================================================

#[test]
fn all_simple_identity_types_serialize_as_strings() {
    assert_eq!(
        serde_json::to_string(
            &ProviderId::new("ibm").unwrap()
        )
        .unwrap(),
        "\"ibm\""
    );

    assert_eq!(
        serde_json::to_string(
            &HardwareId::new("ibm_torino").unwrap()
        )
        .unwrap(),
        "\"ibm_torino\""
    );

    assert_eq!(
        serde_json::to_string(
            &DeviceId::new("ibm_torino").unwrap()
        )
        .unwrap(),
        "\"ibm_torino\""
    );

    assert_eq!(
        serde_json::to_string(
            &BackendId::new(
                "local-statevector"
            )
            .unwrap()
        )
        .unwrap(),
        "\"local-statevector\""
    );

    assert_eq!(
        serde_json::to_string(
            &ArchitectureId::new(
                "heron-r2"
            )
            .unwrap()
        )
        .unwrap(),
        "\"heron-r2\""
    );
}

#[test]
fn serde_rejects_invalid_simple_identity_values() {
    assert!(
        serde_json::from_str::<ProviderId>(
            "\"ibm/provider\""
        )
        .is_err()
    );

    assert!(
        serde_json::from_str::<HardwareId>(
            "\"ibm torino\""
        )
        .is_err()
    );

    assert!(
        serde_json::from_str::<DeviceId>(
            "\"\""
        )
        .is_err()
    );

    assert!(
        serde_json::from_str::<BackendId>(
            "\"backend:secret\""
        )
        .is_err()
    );

    assert!(
        serde_json::from_str::<ArchitectureId>(
            "\"../architecture\""
        )
        .is_err()
    );
}

#[test]
fn serde_rejects_non_string_simple_identity_values() {
    assert!(
        serde_json::from_str::<ProviderId>(
            "123"
        )
        .is_err()
    );

    assert!(
        serde_json::from_str::<BackendId>(
            "null"
        )
        .is_err()
    );

    assert!(
        serde_json::from_str::<HardwareId>(
            "[]"
        )
        .is_err()
    );

    assert!(
        serde_json::from_str::<DeviceId>(
            "{}"
        )
        .is_err()
    );
}

#[test]
fn complete_identity_serde_preserves_all_semantics() {
    let identity =
        complete_hardware_identity();

    let encoded =
        serde_json::to_string(&identity)
            .unwrap();

    let decoded: HardwareIdentity =
        serde_json::from_str(&encoded)
            .unwrap();

    assert_eq!(
        identity.provider(),
        decoded.provider()
    );

    assert_eq!(
        identity.hardware(),
        decoded.hardware()
    );

    assert_eq!(
        identity.device(),
        decoded.device()
    );

    assert_eq!(
        identity.architecture(),
        decoded.architecture()
    );

    assert_eq!(
        identity.firmware(),
        decoded.firmware()
    );

    assert_eq!(
        identity.revision(),
        decoded.revision()
    );
}

// =============================================================================
// Security and canonicalization regression tests
// =============================================================================

#[test]
fn simple_identity_rejects_url_and_credential_syntax() {
    for value in [
        "ibm://user:password@host",
        "https://api.example/token",
        "provider://ibm?api_key=secret",
        "ibm:secret",
    ] {
        assert!(
            ProviderId::new(value).is_err(),
            "provider identity must reject URL/credential syntax: {value:?}"
        );
    }
}

#[test]
fn simple_identity_does_not_normalize_input() {
    assert!(
        ProviderId::new(" ibm").is_err()
    );

    assert!(
        ProviderId::new("ibm ").is_err()
    );

    assert!(
        FirmwareVersion::new(" 1.0.0").is_err()
    );

    assert!(
        HardwareRevision::new(" A0").is_err()
    );
}

#[test]
fn identity_case_is_semantically_significant() {
    let lower =
        ProviderId::new("ibm").unwrap();

    let upper =
        ProviderId::new("IBM").unwrap();

    assert_ne!(lower, upper);

    assert_eq!(
        lower.as_str(),
        "ibm"
    );

    assert_eq!(
        upper.as_str(),
        "IBM"
    );
}

#[test]
fn qualified_identity_cannot_escape_its_hierarchy() {
    for value in [
        "provider://../secret",
        "provider://ibm/../secret",
        "provider://ibm/./secret",
        "provider://./secret",
    ] {
        assert!(
            QualifiedIdentity::parse(value).is_err(),
            "qualified identity must reject traversal: {value:?}"
        );
    }
}

// =============================================================================
// Documented examples
// =============================================================================

#[test]
fn documented_provider_examples_are_valid() {
    for provider in [
        "ibm",
        "ionq",
        "quantinuum",
        "rigetti",
        "iqm",
        "aws-braket",
        "quera",
        "local",
    ] {
        assert!(
            ProviderId::new(provider).is_ok(),
            "invalid provider example: {provider}"
        );
    }
}

#[test]
fn documented_hardware_examples_are_valid() {
    for hardware in [
        "ibm_torino",
        "ionq_forte",
        "chip_a",
        "logical_qpu_01",
    ] {
        assert!(
            HardwareId::new(hardware).is_ok(),
            "invalid hardware example: {hardware}"
        );
    }
}

#[test]
fn documented_architecture_examples_are_valid() {
    for architecture in [
        "ibm-heron-r2",
        "ionq-forte",
        "neutral-atom-a",
        "zamani-statevector-v1",
    ] {
        assert!(
            ArchitectureId::new(architecture).is_ok(),
            "invalid architecture example: {architecture}"
        );
    }
}

#[test]
fn documented_qualified_identity_examples_are_valid() {
    for value in [
        "provider://ibm/ibm_torino",
        "provider://ionq/forte",
        "local://simulator/statevector",
    ] {
        let parsed =
            QualifiedIdentity::parse(value)
                .unwrap();

        assert_eq!(
            parsed.to_string(),
            value
        );
    }
}

// =============================================================================
// Cross-object determinism
// =============================================================================

#[test]
fn complete_identity_descriptors_are_deterministic() {
    let first =
        complete_hardware_identity();

    let second =
        complete_hardware_identity();

    assert_eq!(
        first.provenance_key(),
        second.provenance_key()
    );

    assert_eq!(
        first.qualified_hardware_id(),
        second.qualified_hardware_id()
    );
}

#[test]
fn backend_identity_descriptors_are_deterministic() {
    let first =
        complete_backend_identity();

    let second =
        complete_backend_identity();

    assert_eq!(
        first.provenance_key(),
        second.provenance_key()
    );

    assert_eq!(
        first.qualified(),
        second.qualified()
    );
}

#[test]
fn identity_reference_is_deterministic() {
    let first =
        complete_identity_ref();

    let second =
        complete_identity_ref();

    assert_eq!(first, second);

    assert_eq!(
        first.qualified_backend(),
        second.qualified_backend()
    );
}

// =============================================================================
// Public API compatibility smoke test
// =============================================================================

#[test]
fn identity_api_matches_hardware_hal_contract() {
    let hardware =
        complete_hardware_identity();

    let backend =
        complete_backend_identity();

    let reference =
        complete_identity_ref();

    let _provider: &ProviderId =
        hardware.provider();

    let _hardware: &HardwareId =
        hardware.hardware();

    let _device: &DeviceId =
        hardware.device();

    let _architecture: &ArchitectureId =
        hardware.architecture();

    let _firmware: &FirmwareVersion =
        hardware.firmware();

    let _revision: &HardwareRevision =
        hardware.revision();

    let _backend_provider: &ProviderId =
        backend.provider();

    let _backend: &BackendId =
        backend.backend();

    let _backend_device: &DeviceId =
        backend.device();

    let _backend_architecture:
        &ArchitectureId =
        backend.architecture();

    let _reference_provider:
        &ProviderId =
        reference.provider();

    let _reference_hardware:
        &HardwareId =
        reference.hardware();

    let _reference_device:
        &DeviceId =
        reference.device();

    let _reference_backend:
        &BackendId =
        reference.backend();

    assert!(
        !hardware
            .provenance_key()
            .is_empty()
    );

    assert!(
        !backend
            .provenance_key()
            .is_empty()
    );

    assert!(
        !reference
            .qualified_backend()
            .to_string()
            .is_empty()
    );
}

// =============================================================================
// Final conformance invariant
// =============================================================================

#[test]
fn identity_layer_has_no_implicit_runtime_dependencies() {
    // This test intentionally exercises only pure construction, parsing,
    // serialization and deterministic value semantics.
    //
    // If this test remains independent of backend/provider/network state, the
    // identity layer retains its required foundational architecture.

    let provider =
        ProviderId::new("local").unwrap();

    let hardware =
        HardwareId::new("simulator").unwrap();

    let device =
        DeviceId::new("statevector").unwrap();

    let backend =
        BackendId::new("local-statevector")
            .unwrap();

    let architecture =
        ArchitectureId::new(
            "zamani-statevector-v1",
        )
        .unwrap();

    let firmware =
        FirmwareVersion::new("1.0.0")
            .unwrap();

    let revision =
        HardwareRevision::new("v1")
            .unwrap();

    let identity =
        HardwareIdentity::builder()
            .provider(provider)
            .hardware(hardware)
            .device(device)
            .architecture(architecture)
            .firmware(firmware)
            .revision(revision)
            .build()
            .unwrap();

    assert_eq!(
        identity
            .qualified_hardware_id()
            .to_string(),
        "provider://local/simulator"
    );

    let backend_identity =
        BackendIdentity::new(
            ProviderId::local(),
            backend,
            DeviceId::new("statevector")
                .unwrap(),
            ArchitectureId::new(
                "zamani-statevector-v1",
            )
            .unwrap(),
        );

    assert_eq!(
        backend_identity
            .qualified()
            .to_string(),
        "provider://local/local-statevector"
    );
}