//! Zamani Quantum Memory — Snapshot Integration Tests
//!
//! Production integration tests for:
//!
//! `src/quantum/memory/snapshot.rs`
//!
//! # Purpose
//!
//! This file verifies the complete public snapshot contract across the memory
//! subsystem. It intentionally tests integration boundaries rather than
//! duplicating the implementation's private unit tests.
//!
//! The tests cover:
//!
//! - snapshot schema identity;
//! - format versioning;
//! - strongly typed snapshot identity;
//! - logical qubit counts;
//! - every supported state representation;
//! - backend-native state handling;
//! - extension representations;
//! - every storage-location class;
//! - precision declarations;
//! - explicit and native endianness;
//! - payload encoding;
//! - integrity metadata;
//! - metadata validation;
//! - payload-size validation;
//! - header/payload consistency;
//! - snapshot immutability/value semantics;
//! - validation policies;
//! - portable snapshot policy;
//! - restore policies;
//! - provider integration;
//! - provider-neutral QPU state handling;
//! - Serde round-tripping;
//! - corruption detection at the snapshot-envelope level;
//! - resource-limit enforcement;
//! - deterministic identifiers;
//! - no process-local resources in the snapshot model.
//!
//! # Architectural contract
//!
//! ```text
//! quantum::ir
//!      │
//!      ▼
//! execution/runtime
//!      │
//!      ▼
//! quantum::memory
//!      │
//!      ├── types.rs
//!      ├── errors.rs
//!      ├── snapshot.rs ◄── THIS TEST MODULE
//!      │       │
//!      │       └── SnapshotProvider
//!      │
//!      └── serialization.rs
//!              │
//!              ▼
//!          persistence / transport
//!
//! snapshot.rs MUST remain independent of:
//!
//! - IBM;
//! - Google;
//! - IonQ;
//! - Rigetti;
//! - Quantinuum;
//! - IQM;
//! - D-Wave;
//! - Pasqal;
//! - CUDA;
//! - HIP;
//! - Metal;
//! - Vulkan;
//! - SYCL;
//! - MPI;
//! - RDMA;
//! - any individual simulator.
//! ```
//!
//! # Important distinction
//!
//! These tests do not claim that every QPU can literally snapshot its physical
//! quantum state. Most real QPUs do not expose arbitrary live quantum-state
//! capture. Instead, the contract guarantees that Zamani can represent:
//!
//! 1. simulator-owned state;
//! 2. accelerator-owned state;
//! 3. distributed state;
//! 4. provider-native state handles;
//! 5. portable serialized state where the provider exposes such state.
//!
//! Hardware-specific capture/restore remains the responsibility of the
//! corresponding backend/provider implementation.
//!
//! # Rust compatibility
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021
//! - stable Rust
//! - no nightly features
//! - no unsafe code
//!
//! # Integration
//!
//! This module is intended to be included by:
//!
//! `src/quantum/memory/tests/mod.rs`
//!
//! with:
//!
//! ```text
//! #[cfg(test)]
//! mod snapshot;
//! ```
//!
//! It intentionally does not require changes to `snapshot.rs` merely because
//! another representation or hardware provider is added.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use crate::quantum::memory::snapshot::{
    QuantumSnapshot,
    SnapshotBuilder,
    SnapshotEndianness,
    SnapshotFormatVersion,
    SnapshotIntegrity,
    SnapshotIntegrityAlgorithm,
    SnapshotMetadata,
    SnapshotPayload,
    SnapshotPayloadEncoding,
    SnapshotPrecision,
    SnapshotProvider,
    SnapshotRestorePolicy,
    SnapshotStorageLocation,
    SnapshotValidationPolicy,
    StateRepresentation,
    MAX_DESCRIPTION_LENGTH,
    MAX_IDENTIFIER_LENGTH,
    MAX_LABEL_LENGTH,
    MAX_LABELS,
    MAX_SUPPORTED_MAJOR_VERSION,
    SNAPSHOT_FORMAT_MAJOR,
    SNAPSHOT_FORMAT_MINOR,
    SNAPSHOT_FORMAT_NAME,
    SNAPSHOT_MAGIC,
    SNAPSHOT_SCHEMA_ID,
    DEFAULT_MAX_PAYLOAD_BYTES,
};

use crate::quantum::memory::types::{
    ByteCount,
    QubitCount,
    SnapshotId,
};

use std::sync::atomic::{AtomicUsize, Ordering};

// =============================================================================
// Test helpers
// =============================================================================

fn snapshot_id(value: u64) -> SnapshotId {
    SnapshotId::new(value)
}

fn state_vector_snapshot() -> QuantumSnapshot {
    SnapshotBuilder::new(
        snapshot_id(1),
        QubitCount::new(2),
        StateRepresentation::StateVector,
    )
    .build(vec![0, 1, 2, 3])
    .expect("valid state-vector snapshot must build")
}

fn backend_snapshot() -> QuantumSnapshot {
    SnapshotBuilder::new(
        snapshot_id(2),
        QubitCount::new(4),
        StateRepresentation::BackendNative {
            provider: "test-qpu-provider".to_owned(),
        },
    )
    .storage_location(SnapshotStorageLocation::Remote)
    .precision(SnapshotPrecision::BackendDefined)
    .payload_encoding(SnapshotPayloadEncoding::ProviderDefined {
        provider: "test-qpu-provider".to_owned(),
        version: "1".to_owned(),
    })
    .build(vec![0xAA, 0x55])
    .expect("valid backend-native snapshot must build")
}

fn integrity_snapshot() -> QuantumSnapshot {
    SnapshotBuilder::new(
        snapshot_id(3),
        QubitCount::new(1),
        StateRepresentation::StateVector,
    )
    .integrity(
        SnapshotIntegrity::new(
            SnapshotIntegrityAlgorithm::Sha256,
            vec![0u8; 32],
        )
        .expect("32-byte SHA-256 digest must be valid"),
    )
    .build(vec![0, 1])
    .expect("snapshot with valid integrity metadata must build")
}

// =============================================================================
// Schema and version contract
// =============================================================================

#[test]
fn snapshot_schema_identity_is_stable() {
    assert_eq!(
        SNAPSHOT_SCHEMA_ID,
        "zamani.quantum.memory.snapshot"
    );

    assert_eq!(
        SNAPSHOT_FORMAT_NAME,
        "Zamani Quantum Memory Snapshot"
    );

    assert_eq!(SNAPSHOT_MAGIC, *b"ZQMS");
}

#[test]
fn current_snapshot_format_is_self_consistent() {
    let current = SnapshotFormatVersion::CURRENT;

    assert_eq!(current.major, SNAPSHOT_FORMAT_MAJOR);
    assert_eq!(current.minor, SNAPSHOT_FORMAT_MINOR);
    assert!(current.is_supported_major());
    assert!(current.is_current());
}

#[test]
fn newer_major_versions_are_not_accepted() {
    let newer = SnapshotFormatVersion::new(
        MAX_SUPPORTED_MAJOR_VERSION.saturating_add(1),
        0,
    );

    assert!(!newer.is_supported_major());
    assert!(!newer.is_current());
}

#[test]
fn older_minor_versions_do_not_change_major_identity() {
    let older_minor = SnapshotFormatVersion::new(
        SNAPSHOT_FORMAT_MAJOR,
        SNAPSHOT_FORMAT_MINOR.saturating_sub(1),
    );

    assert!(older_minor.is_supported_major());

    if SNAPSHOT_FORMAT_MINOR > 0 {
        assert!(!older_minor.is_current());
    }
}

// =============================================================================
// Representation contract
// =============================================================================

#[test]
fn every_builtin_representation_has_a_stable_identifier() {
    let representations = [
        (
            StateRepresentation::StateVector,
            "state-vector",
        ),
        (
            StateRepresentation::DensityMatrix,
            "density-matrix",
        ),
        (
            StateRepresentation::Stabilizer,
            "stabilizer",
        ),
        (
            StateRepresentation::Sparse,
            "sparse",
        ),
        (
            StateRepresentation::TensorNetwork,
            "tensor-network",
        ),
    ];

    for (representation, expected) in representations {
        assert_eq!(representation.identifier(), expected);
        assert!(!representation.is_backend_native());
    }
}

#[test]
fn backend_native_representation_is_explicitly_provider_scoped() {
    let representation = StateRepresentation::BackendNative {
        provider: "example-qpu".to_owned(),
    };

    assert_eq!(
        representation.identifier(),
        "backend-native:example-qpu"
    );

    assert!(representation.is_backend_native());
}

#[test]
fn extension_representation_is_not_mistaken_for_backend_native() {
    let representation = StateRepresentation::Extension {
        name: "zamani.photonic.dualrail.v1".to_owned(),
    };

    assert_eq!(
        representation.identifier(),
        "extension:zamani.photonic.dualrail.v1"
    );

    assert!(!representation.is_backend_native());
}

#[test]
fn backend_provider_identifier_is_bounded() {
    let representation = StateRepresentation::BackendNative {
        provider: "x".repeat(MAX_IDENTIFIER_LENGTH + 1),
    };

    let result = SnapshotBuilder::new(
        snapshot_id(10),
        QubitCount::new(1),
        representation,
    )
    .build(vec![0]);

    assert!(result.is_err());
}

// =============================================================================
// Storage-location contract
// =============================================================================

#[test]
fn all_standard_storage_locations_have_stable_identifiers() {
    let locations = [
        (SnapshotStorageLocation::Host, "host"),
        (SnapshotStorageLocation::PinnedHost, "pinned-host"),
        (SnapshotStorageLocation::Device, "device"),
        (SnapshotStorageLocation::Unified, "unified"),
        (SnapshotStorageLocation::Distributed, "distributed"),
        (SnapshotStorageLocation::Remote, "remote"),
    ];

    for (location, expected) in locations {
        assert_eq!(location.identifier(), expected);
    }
}

#[test]
fn custom_storage_locations_are_provider_neutral() {
    let location = SnapshotStorageLocation::Custom(
        "future-accelerator-memory".to_owned(),
    );

    assert_eq!(
        location.identifier(),
        "custom:future-accelerator-memory"
    );
}

#[test]
fn oversized_custom_storage_location_is_rejected() {
    let location = SnapshotStorageLocation::Custom(
        "x".repeat(MAX_IDENTIFIER_LENGTH + 1),
    );

    let result = SnapshotBuilder::new(
        snapshot_id(11),
        QubitCount::new(1),
        StateRepresentation::StateVector,
    )
    .storage_location(location)
    .build(vec![0]);

    assert!(result.is_err());
}

// =============================================================================
// Precision and endianness
// =============================================================================

#[test]
fn all_precision_classes_are_serializable() {
    let precisions = [
        SnapshotPrecision::F32,
        SnapshotPrecision::F64,
        SnapshotPrecision::Extended,
        SnapshotPrecision::BackendDefined,
    ];

    for precision in precisions {
        let snapshot = SnapshotBuilder::new(
            snapshot_id(20),
            QubitCount::new(1),
            StateRepresentation::StateVector,
        )
        .precision(precision)
        .build(vec![0])
        .expect("precision declaration must not invalidate a snapshot");

        assert_eq!(snapshot.header.precision, precision);
    }
}

#[test]
fn_explicit_endianness_is_portable() {
    assert!(SnapshotEndianness::Little.is_explicit());
    assert!(SnapshotEndianness::Big.is_explicit());
    assert!(!SnapshotEndianness::Native.is_explicit());
}

#[test]
fn host_endianness_is_always_explicit() {
    let host = SnapshotEndianness::host();

    assert!(matches!(
        host,
        SnapshotEndianness::Little | SnapshotEndianness::Big
    ));

    assert!(host.is_explicit());
}

// =============================================================================
// Payload contract
// =============================================================================

#[test]
fn payload_size_matches_actual_payload() {
    let payload = SnapshotPayload::new(
        vec![1, 2, 3, 4],
        ByteCount::new(4),
    )
    .expect("payload should fit");

    assert_eq!(payload.size(), ByteCount::new(4));
    assert_eq!(payload.as_bytes(), &[1, 2, 3, 4]);
    assert!(!payload.is_empty());
}

#[test]
fn empty_payload_is_representable() {
    let payload = SnapshotPayload::new(
        Vec::new(),
        ByteCount::ZERO,
    )
    .expect("zero-byte payload must be representable");

    assert_eq!(payload.size(), ByteCount::ZERO);
    assert!(payload.is_empty());
}

#[test]
fn payload_limit_is_enforced_before_snapshot_construction() {
    let result = SnapshotPayload::new(
        vec![1, 2, 3, 4],
        ByteCount::new(3),
    );

    assert!(result.is_err());
}

#[test]
fn snapshot_payload_size_is_derived_from_payload_not_user_metadata() {
    let snapshot = state_vector_snapshot();

    assert_eq!(
        snapshot.payload_size(),
        ByteCount::new(4)
    );

    assert_eq!(
        snapshot.header.payload_size,
        ByteCount::new(4)
    );
}

// =============================================================================
// Integrity contract
// =============================================================================

#[test]
fn no_integrity_requires_an_empty_digest() {
    let result = SnapshotIntegrity::new(
        SnapshotIntegrityAlgorithm::None,
        Vec::new(),
    );

    assert!(result.is_ok());
}

#[test]
fn no_integrity_rejects_nonempty_digest() {
    let result = SnapshotIntegrity::new(
        SnapshotIntegrityAlgorithm::None,
        vec![0],
    );

    assert!(result.is_err());
}

#[test]
fn sha256_requires_exactly_32_bytes() {
    let valid = SnapshotIntegrity::new(
        SnapshotIntegrityAlgorithm::Sha256,
        vec![0; 32],
    );

    assert!(valid.is_ok());

    let too_short = SnapshotIntegrity::new(
        SnapshotIntegrityAlgorithm::Sha256,
        vec![0; 31],
    );

    assert!(too_short.is_err());

    let too_long = SnapshotIntegrity::new(
        SnapshotIntegrityAlgorithm::Sha256,
        vec![0; 33],
    );

    assert!(too_long.is_err());
}

#[test]
fn sha512_requires_exactly_64_bytes() {
    let valid = SnapshotIntegrity::new(
        SnapshotIntegrityAlgorithm::Sha512,
        vec![0; 64],
    );

    assert!(valid.is_ok());

    let too_short = SnapshotIntegrity::new(
        SnapshotIntegrityAlgorithm::Sha512,
        vec![0; 63],
    );

    assert!(too_short.is_err());

    let too_long = SnapshotIntegrity::new(
        SnapshotIntegrityAlgorithm::Sha512,
        vec![0; 65],
    );

    assert!(too_long.is_err());
}

#[test]
fn provider_defined_integrity_can_be_carried_without_vendor_types() {
    let integrity = SnapshotIntegrity::new(
        SnapshotIntegrityAlgorithm::ProviderDefined {
            provider: "future-qpu-provider".to_owned(),
            algorithm: "provider-integrity-v1".to_owned(),
        },
        vec![1, 2, 3],
    )
    .expect("provider-defined integrity should be structurally valid");

    assert!(integrity.validate().is_ok());
}

// =============================================================================
// Metadata contract
// =============================================================================

#[test]
fn default_metadata_is_valid() {
    let metadata = SnapshotMetadata::default();

    assert!(metadata.validate().is_ok());
}

#[test]
fn metadata_description_limit_is_enforced() {
    let mut metadata = SnapshotMetadata::default();

    metadata.description =
        Some("x".repeat(MAX_DESCRIPTION_LENGTH + 1));

    assert!(metadata.validate().is_err());
}

#[test]
fn metadata_label_count_limit_is_enforced() {
    let mut metadata = SnapshotMetadata::default();

    metadata.labels = vec![
        "label".to_owned();
        MAX_LABELS + 1
    ];

    assert!(metadata.validate().is_err());
}

#[test]
fn metadata_label_length_limit_is_enforced() {
    let mut metadata = SnapshotMetadata::default();

    metadata.labels = vec![
        "x".repeat(MAX_LABEL_LENGTH + 1)
    ];

    assert!(metadata.validate().is_err());
}

#[test]
fn metadata_rejects_nul_characters() {
    let mut metadata = SnapshotMetadata::default();

    metadata.description = Some("valid\0invalid".to_owned());

    assert!(metadata.validate().is_err());
}

// =============================================================================
// Builder contract
// =============================================================================

#[test]
fn builder_creates_a_complete_valid_snapshot() {
    let snapshot = SnapshotBuilder::new(
        snapshot_id(100),
        QubitCount::new(3),
        StateRepresentation::StateVector,
    )
    .storage_location(SnapshotStorageLocation::Host)
    .precision(SnapshotPrecision::F64)
    .endianness(SnapshotEndianness::Little)
    .payload_encoding(SnapshotPayloadEncoding::Named {
        name: "zamani-state-vector".to_owned(),
        version: "1".to_owned(),
    })
    .metadata(SnapshotMetadata {
        description: Some("Bell/GHZ test state".to_owned()),
        labels: vec![
            "test".to_owned(),
            "state-vector".to_owned(),
        ],
        zamani_version: Some("1.0".to_owned()),
        program_identity: Some("program:test".to_owned()),
        execution_identity: Some("execution:test".to_owned()),
        provider: None,
        provider_version: None,
    })
    .build(vec![1, 2, 3, 4, 5])
    .expect("complete snapshot must build");

    assert_eq!(snapshot.id(), snapshot_id(100));
    assert_eq!(snapshot.qubit_count(), QubitCount::new(3));
    assert_eq!(
        snapshot.representation(),
        &StateRepresentation::StateVector
    );
    assert_eq!(
        snapshot.payload_bytes(),
        &[1, 2, 3, 4, 5]
    );

    assert!(snapshot.validate().is_ok());
}

#[test]
fn builder_enforces_custom_payload_limit() {
    let result = SnapshotBuilder::new(
        snapshot_id(101),
        QubitCount::new(2),
        StateRepresentation::StateVector,
    )
    .max_payload_bytes(ByteCount::new(2))
    .build(vec![1, 2, 3]);

    assert!(result.is_err());
}

#[test]
fn builder_default_limit_is_consistent_with_snapshot_contract() {
    assert_eq!(
        DEFAULT_MAX_PAYLOAD_BYTES,
        16 * 1024 * 1024 * 1024
    );

    let snapshot = SnapshotBuilder::new(
        snapshot_id(102),
        QubitCount::new(1),
        StateRepresentation::StateVector,
    )
    .build(vec![0; 8])
    .expect("small payload must fit default limit");

    assert!(snapshot.validate().is_ok());
}

// =============================================================================
// Header/payload consistency
// =============================================================================

#[test]
fn payload_length_mismatch_is_rejected() {
    let mut snapshot = state_vector_snapshot();

    snapshot.header.payload_size =
        ByteCount::new(snapshot.payload_size().get() + 1);

    assert!(snapshot.validate().is_err());
}

#[test]
fn invalid_magic_is_rejected() {
    let mut snapshot = state_vector_snapshot();

    snapshot.header.magic = *b"BAD!";

    assert!(snapshot.validate().is_err());
}

#[test]
fn invalid_schema_id_is_rejected() {
    let mut snapshot = state_vector_snapshot();

    snapshot.header.schema_id =
        "zamani.quantum.memory.invalid".to_owned();

    assert!(snapshot.validate().is_err());
}

#[test]
fn unsupported_major_format_is_rejected() {
    let mut snapshot = state_vector_snapshot();

    snapshot.header.format_version =
        SnapshotFormatVersion::new(
            SNAPSHOT_FORMAT_MAJOR.saturating_add(1),
            0,
        );

    assert!(snapshot.validate().is_err());
}

#[test]
fn invalid_representation_identifier_is_rejected() {
    let mut snapshot = state_vector_snapshot();

    snapshot.header.representation =
        StateRepresentation::BackendNative {
            provider: "\0invalid".to_owned(),
        };

    assert!(snapshot.validate().is_err());
}

// =============================================================================
// Validation-policy contract
// =============================================================================

#[test]
fn default_validation_policy_accepts_normal_snapshot() {
    let snapshot = state_vector_snapshot();

    assert!(
        snapshot
            .validate_with(&SnapshotValidationPolicy::default())
            .is_ok()
    );
}

#[test]
fn validation_policy_rejects_excessive_qubit_count() {
    let snapshot = state_vector_snapshot();

    let policy = SnapshotValidationPolicy {
        max_qubits: QubitCount::new(1),
        ..SnapshotValidationPolicy::default()
    };

    assert!(
        snapshot.validate_with(&policy).is_err()
    );
}

#[test]
fn validation_policy_rejects_excessive_payload() {
    let snapshot = state_vector_snapshot();

    let policy = SnapshotValidationPolicy {
        max_payload_bytes: ByteCount::new(3),
        ..SnapshotValidationPolicy::default()
    };

    assert!(
        snapshot.validate_with(&policy).is_err()
    );
}

#[test]
fn portable_policy_rejects_backend_native_state() {
    let snapshot = backend_snapshot();

    assert!(
        snapshot
            .validate_with(&SnapshotValidationPolicy::portable())
            .is_err()
    );
}

#[test]
fn portable_policy_rejects_provider_defined_encoding() {
    let snapshot = SnapshotBuilder::new(
        snapshot_id(200),
        QubitCount::new(1),
        StateRepresentation::StateVector,
    )
    .payload_encoding(SnapshotPayloadEncoding::ProviderDefined {
        provider: "future-provider".to_owned(),
        version: "1".to_owned(),
    })
    .integrity(
        SnapshotIntegrity::new(
            SnapshotIntegrityAlgorithm::Sha256,
            vec![0; 32],
        )
        .expect("valid SHA-256 descriptor"),
    )
    .endianness(SnapshotEndianness::Little)
    .build(vec![1])
    .expect("snapshot itself is structurally valid");

    assert!(
        snapshot
            .validate_with(&SnapshotValidationPolicy::portable())
            .is_err()
    );
}

#[test]
fn portable_policy_requires_integrity() {
    let snapshot = SnapshotBuilder::new(
        snapshot_id(201),
        QubitCount::new(1),
        StateRepresentation::StateVector,
    )
    .endianness(SnapshotEndianness::Little)
    .build(vec![1])
    .expect("snapshot should build");

    let policy = SnapshotValidationPolicy::portable();

    assert!(snapshot.validate_with(&policy).is_err());
}

#[test]
fn portable_policy_accepts_explicit_endianness_with_integrity() {
    let snapshot = SnapshotBuilder::new(
        snapshot_id(202),
        QubitCount::new(1),
        StateRepresentation::StateVector,
    )
    .endianness(SnapshotEndianness::Little)
    .integrity(
        SnapshotIntegrity::new(
            SnapshotIntegrityAlgorithm::Sha256,
            vec![0; 32],
        )
        .expect("valid SHA-256 descriptor"),
    )
    .build(vec![1])
    .expect("snapshot should build");

    let policy = SnapshotValidationPolicy::portable();

    assert!(snapshot.validate_with(&policy).is_ok());
}

#[test]
fn native_endianness_is_rejected_by_portable_policy() {
    let snapshot = SnapshotBuilder::new(
        snapshot_id(203),
        QubitCount::new(1),
        StateRepresentation::StateVector,
    )
    .endianness(SnapshotEndianness::Native)
    .integrity(
        SnapshotIntegrity::new(
            SnapshotIntegrityAlgorithm::Sha256,
            vec![0; 32],
        )
        .expect("valid SHA-256 descriptor"),
    )
    .build(vec![1])
    .expect("snapshot should build");

    assert!(
        snapshot
            .validate_with(&SnapshotValidationPolicy::portable())
            .is_err()
    );
}

#[test]
fn custom_storage_can_be_disabled_by_policy() {
    let snapshot = SnapshotBuilder::new(
        snapshot_id(204),
        QubitCount::new(1),
        StateRepresentation::StateVector,
    )
    .storage_location(
        SnapshotStorageLocation::Custom(
            "special-memory".to_owned(),
        ),
    )
    .build(vec![1])
    .expect("snapshot should build");

    let policy = SnapshotValidationPolicy {
        allow_custom_storage_location: false,
        ..SnapshotValidationPolicy::default()
    };

    assert!(snapshot.validate_with(&policy).is_err());
}

// =============================================================================
// Restore-policy contract
// =============================================================================

#[test]
fn default_restore_policy_is_conservative() {
    let policy = SnapshotRestorePolicy::default();

    assert!(!policy.allow_representation_conversion);
    assert!(policy.allow_storage_migration);
    assert!(policy.require_matching_backend_provider);
    assert!(!policy.allow_precision_conversion);
    assert!(policy.require_exact_payload);
}

#[test]
fn strict_restore_policy_disables_migration_and_conversion() {
    let policy = SnapshotRestorePolicy::strict();

    assert!(!policy.allow_representation_conversion);
    assert!(!policy.allow_storage_migration);
    assert!(policy.require_matching_backend_provider);
    assert!(!policy.allow_precision_conversion);
    assert!(policy.require_exact_payload);
}

#[test]
fn strict_restore_policy_does_not_permit_representation_conversion() {
    assert!(
        !SnapshotRestorePolicy::strict()
            .permits_representation_conversion()
    );
}

// =============================================================================
// Provider integration
// =============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
struct MockProviderState {
    payload: Vec<u8>,
    qubits: QubitCount,
}

#[derive(Debug, Default)]
struct MockProvider {
    captures: AtomicUsize,
    restores: AtomicUsize,
}

impl SnapshotProvider for MockProvider {
    type State = MockProviderState;

    fn snapshot(
        &self,
        state: &Self::State,
    ) -> Result<QuantumSnapshot, crate::quantum::memory::errors::MemoryError>
    {
        self.captures.fetch_add(1, Ordering::SeqCst);

        SnapshotBuilder::new(
            snapshot_id(300),
            state.qubits,
            StateRepresentation::StateVector,
        )
        .storage_location(SnapshotStorageLocation::Host)
        .build(state.payload.clone())
    }

    fn can_restore(
        &self,
        snapshot: &QuantumSnapshot,
        _policy: &SnapshotRestorePolicy,
    ) -> Result<(), crate::quantum::memory::errors::MemoryError>
    {
        snapshot.validate()
    }

    fn restore(
        &self,
        snapshot: &QuantumSnapshot,
        _policy: &SnapshotRestorePolicy,
    ) -> Result<Self::State, crate::quantum::memory::errors::MemoryError>
    {
        self.restores.fetch_add(1, Ordering::SeqCst);

        snapshot.validate()?;

        Ok(MockProviderState {
            payload: snapshot.payload_bytes().to_vec(),
            qubits: snapshot.qubit_count(),
        })
    }
}

#[test]
fn snapshot_provider_can_capture_and_restore_without_vendor_types() {
    let provider = MockProvider::default();

    let state = MockProviderState {
        payload: vec![0, 1, 1, 0],
        qubits: QubitCount::new(2),
    };

    let snapshot = provider
        .snapshot(&state)
        .expect("provider capture should succeed");

    assert_eq!(provider.captures.load(Ordering::SeqCst), 1);

    provider
        .can_restore(
            &snapshot,
            &SnapshotRestorePolicy::strict(),
        )
        .expect("provider should accept its own snapshot");

    let restored = provider
        .restore(
            &snapshot,
            &SnapshotRestorePolicy::strict(),
        )
        .expect("provider restore should succeed");

    assert_eq!(provider.restores.load(Ordering::SeqCst), 1);
    assert_eq!(restored, state);
}

#[test]
fn provider_snapshot_is_owned_and_does_not_borrow_provider_state() {
    let provider = MockProvider::default();

    let state = MockProviderState {
        payload: vec![1, 2, 3],
        qubits: QubitCount::new(1),
    };

    let snapshot = provider
        .snapshot(&state)
        .expect("capture should succeed");

    drop(state);

    assert_eq!(
        snapshot.payload_bytes(),
        &[1, 2, 3]
    );
}

// =============================================================================
// Backend/QPU neutrality
// =============================================================================

#[test]
fn remote_backend_snapshot_can_be_described_without_vendor_dependencies() {
    let snapshot = backend_snapshot();

    assert_eq!(
        snapshot.header.storage_location,
        SnapshotStorageLocation::Remote
    );

    assert_eq!(
        snapshot.header.representation,
        StateRepresentation::BackendNative {
            provider: "test-qpu-provider".to_owned(),
        }
    );

    assert_eq!(
        snapshot.header.precision,
        SnapshotPrecision::BackendDefined
    );

    assert!(
        snapshot.header.representation.is_backend_native()
    );
}

#[test]
fn distributed_snapshot_location_is_representation_independent() {
    let snapshot = SnapshotBuilder::new(
        snapshot_id(301),
        QubitCount::new(64),
        StateRepresentation::TensorNetwork,
    )
    .storage_location(SnapshotStorageLocation::Distributed)
    .precision(SnapshotPrecision::F64)
    .build(vec![1, 2, 3])
    .expect("distributed snapshot should build");

    assert_eq!(
        snapshot.header.storage_location,
        SnapshotStorageLocation::Distributed
    );

    assert_eq!(
        snapshot.header.representation,
        StateRepresentation::TensorNetwork
    );
}

#[test]
fn accelerator_snapshot_location_does_not_expose_device_pointers() {
    let snapshot = SnapshotBuilder::new(
        snapshot_id(302),
        QubitCount::new(16),
        StateRepresentation::StateVector,
    )
    .storage_location(SnapshotStorageLocation::Device)
    .build(vec![0xAA])
    .expect("device snapshot should build");

    assert_eq!(
        snapshot.header.storage_location,
        SnapshotStorageLocation::Device
    );

    // The public snapshot payload is an owned byte vector. There is no public
    // raw device pointer, allocator address, file descriptor, thread handle,
    // or other process-local resource in the snapshot contract.
    assert_eq!(snapshot.payload_bytes(), &[0xAA]);
}

// =============================================================================
// Serde/value semantics
// =============================================================================

#[test]
fn snapshot_is_value_semantic() {
    let snapshot = state_vector_snapshot();
    let cloned = snapshot.clone();

    assert_eq!(snapshot, cloned);
}

#[test]
fn snapshot_serde_round_trip_preserves_semantics() {
    let snapshot = state_vector_snapshot();

    let encoded = serde_json::to_vec(&snapshot)
        .expect("snapshot serialization should succeed");

    let restored: QuantumSnapshot =
        serde_json::from_slice(&encoded)
            .expect("snapshot deserialization should succeed");

    assert_eq!(restored, snapshot);
    assert!(restored.validate().is_ok());
}

#[test]
fn backend_snapshot_serde_round_trip_preserves_provider_identity() {
    let snapshot = backend_snapshot();

    let encoded = serde_json::to_vec(&snapshot)
        .expect("backend snapshot serialization should succeed");

    let restored: QuantumSnapshot =
        serde_json::from_slice(&encoded)
            .expect("backend snapshot deserialization should succeed");

    assert_eq!(
        restored.header.representation,
        StateRepresentation::BackendNative {
            provider: "test-qpu-provider".to_owned(),
        }
    );

    assert_eq!(restored, snapshot);
}

#[test]
fn all_state_representation_variants_serde_round_trip() {
    let representations = vec![
        StateRepresentation::StateVector,
        StateRepresentation::DensityMatrix,
        StateRepresentation::Stabilizer,
        StateRepresentation::Sparse,
        StateRepresentation::TensorNetwork,
        StateRepresentation::BackendNative {
            provider: "provider-a".to_owned(),
        },
        StateRepresentation::Extension {
            name: "extension-a".to_owned(),
        },
    ];

    for representation in representations {
        let encoded = serde_json::to_vec(&representation)
            .expect("representation must serialize");

        let restored: StateRepresentation =
            serde_json::from_slice(&encoded)
                .expect("representation must deserialize");

        assert_eq!(restored, representation);
    }
}

// =============================================================================
// Corruption and defensive validation
// =============================================================================

#[test]
fn corrupted_snapshot_magic_cannot_be_accepted() {
    let mut snapshot = integrity_snapshot();

    snapshot.header.magic = [0, 0, 0, 0];

    assert!(snapshot.validate().is_err());
}

#[test]
fn corrupted_snapshot_schema_cannot_be_accepted() {
    let mut snapshot = integrity_snapshot();

    snapshot.header.schema_id =
        "malicious.schema".to_owned();

    assert!(snapshot.validate().is_err());
}

#[test]
fn corrupted_payload_size_cannot_be_accepted() {
    let mut snapshot = integrity_snapshot();

    snapshot.header.payload_size =
        ByteCount::new(0);

    assert!(snapshot.validate().is_err());
}

#[test]
fn corrupted_integrity_metadata_is_rejected() {
    let mut snapshot = integrity_snapshot();

    snapshot.header.integrity =
        SnapshotIntegrity {
            algorithm: SnapshotIntegrityAlgorithm::Sha256,
            digest: vec![0; 31],
        };

    assert!(snapshot.validate().is_err());
}

// =============================================================================
// Resource safety
// =============================================================================

#[test]
fn extremely_large_qubit_counts_are_policy_checked_before_restore() {
    let snapshot = SnapshotBuilder::new(
        snapshot_id(400),
        QubitCount::new(usize::MAX),
        StateRepresentation::BackendNative {
            provider: "remote-qpu".to_owned(),
        },
    )
    .storage_location(SnapshotStorageLocation::Remote)
    .build(Vec::new())
    .expect("snapshot envelope itself does not allocate state memory");

    let policy = SnapshotValidationPolicy {
        max_qubits: QubitCount::new(1_000_000),
        ..SnapshotValidationPolicy::default()
    };

    assert!(snapshot.validate_with(&policy).is_err());
}

#[test]
fn snapshot_layer_does_not_implicitly_allocate_exponential_state_memory() {
    // A 1,000,000-qubit backend-native snapshot can carry an empty provider
    // payload because the snapshot envelope is not a simulator.
    //
    // The actual provider must decide whether such a state can be restored.
    let snapshot = SnapshotBuilder::new(
        snapshot_id(401),
        QubitCount::new(1_000_000),
        StateRepresentation::BackendNative {
            provider: "remote-provider".to_owned(),
        },
    )
    .storage_location(SnapshotStorageLocation::Remote)
    .build(Vec::new())
    .expect("snapshot construction must not allocate 2^n amplitudes");

    assert_eq!(
        snapshot.qubit_count(),
        QubitCount::new(1_000_000)
    );

    assert!(snapshot.payload_bytes().is_empty());
}

#[test]
fn default_snapshot_payload_ceiling_is_not_used_as_an_exponential_state_allocator() {
    assert_eq!(
        DEFAULT_MAX_PAYLOAD_BYTES,
        16 * 1024 * 1024 * 1024
    );

    // This is deliberately only a small payload. The test exists to ensure
    // that constructing a snapshot never interprets qubit_count as a request
    // to allocate a dense 2^n state vector.
    let snapshot = SnapshotBuilder::new(
        snapshot_id(402),
        QubitCount::new(1024),
        StateRepresentation::BackendNative {
            provider: "provider".to_owned(),
        },
    )
    .storage_location(SnapshotStorageLocation::Remote)
    .build(vec![1])
    .expect("snapshot envelope must remain allocation-independent");

    assert_eq!(
        snapshot.payload_size(),
        ByteCount::new(1)
    );
}

// =============================================================================
// Determinism
// =============================================================================

#[test]
fn identical_snapshot_builds_are_equal() {
    let first = SnapshotBuilder::new(
        snapshot_id(500),
        QubitCount::new(2),
        StateRepresentation::StateVector,
    )
    .build(vec![1, 2, 3])
    .expect("first snapshot");

    let second = SnapshotBuilder::new(
        snapshot_id(500),
        QubitCount::new(2),
        StateRepresentation::StateVector,
    )
    .build(vec![1, 2, 3])
    .expect("second snapshot");

    assert_eq!(first, second);
}

#[test]
fn snapshot_identifiers_remain_distinct() {
    let first = SnapshotBuilder::new(
        snapshot_id(501),
        QubitCount::new(1),
        StateRepresentation::StateVector,
    )
    .build(vec![0])
    .expect("first snapshot");

    let second = SnapshotBuilder::new(
        snapshot_id(502),
        QubitCount::new(1),
        StateRepresentation::StateVector,
    )
    .build(vec![0])
    .expect("second snapshot");

    assert_ne!(first.id(), second.id());
}

// =============================================================================
// No hidden process-local resources
// =============================================================================

#[test]
fn snapshot_public_model_contains_only_portable_semantic_resources() {
    let snapshot = backend_snapshot();

    let serialized = serde_json::to_string(&snapshot)
        .expect("snapshot should serialize");

    // The snapshot contract must not serialize raw/process-local resource
    // concepts. These are deliberately broad negative checks.
    assert!(!serialized.contains("0x"));
    assert!(!serialized.contains("pointer"));
    assert!(!serialized.contains("device_ptr"));
    assert!(!serialized.contains("file_descriptor"));
    assert!(!serialized.contains("thread_handle"));
    assert!(!serialized.contains("mutex"));
}

// =============================================================================
// Provider failure behaviour
// =============================================================================

#[derive(Debug, Default)]
struct RejectingProvider;

impl SnapshotProvider for RejectingProvider {
    type State = Vec<u8>;

    fn snapshot(
        &self,
        _state: &Self::State,
    ) -> Result<
        QuantumSnapshot,
        crate::quantum::memory::errors::MemoryError,
    > {
        Err(
            crate::quantum::memory::errors::MemoryError::unsupported(
                crate::quantum::memory::errors::MemoryErrorCode::UnsupportedOperation,
                "test provider intentionally does not expose snapshot capture",
            ),
        )
    }

    fn can_restore(
        &self,
        _snapshot: &QuantumSnapshot,
        _policy: &SnapshotRestorePolicy,
    ) -> Result<
        (),
        crate::quantum::memory::errors::MemoryError,
    > {
        Err(
            crate::quantum::memory::errors::MemoryError::unsupported(
                crate::quantum::memory::errors::MemoryErrorCode::UnsupportedOperation,
                "test provider intentionally does not expose snapshot restore",
            ),
        )
    }

    fn restore(
        &self,
        _snapshot: &QuantumSnapshot,
        _policy: &SnapshotRestorePolicy,
    ) -> Result<
        Self::State,
        crate::quantum::memory::errors::MemoryError,
    > {
        Err(
            crate::quantum::memory::errors::MemoryError::unsupported(
                crate::quantum::memory::errors::MemoryErrorCode::UnsupportedOperation,
                "test provider intentionally does not expose snapshot restore",
            ),
        )
    }
}

#[test]
fn providers_may_explicitly_reject_snapshot_capture() {
    let provider = RejectingProvider;

    let result = provider.snapshot(&vec![1, 2, 3]);

    assert!(result.is_err());
}

#[test]
fn providers_may_explicitly_reject_snapshot_restore() {
    let provider = RejectingProvider;
    let snapshot = state_vector_snapshot();

    let result = provider.can_restore(
        &snapshot,
        &SnapshotRestorePolicy::strict(),
    );

    assert!(result.is_err());

    let restore_result = provider.restore(
        &snapshot,
        &SnapshotRestorePolicy::strict(),
    );

    assert!(restore_result.is_err());
}

// =============================================================================
// Cross-representation envelope compatibility
// =============================================================================

#[test]
fn every_representation_can_use_the_same_snapshot_envelope_contract() {
    let representations = [
        StateRepresentation::StateVector,
        StateRepresentation::DensityMatrix,
        StateRepresentation::Stabilizer,
        StateRepresentation::Sparse,
        StateRepresentation::TensorNetwork,
        StateRepresentation::BackendNative {
            provider: "provider".to_owned(),
        },
        StateRepresentation::Extension {
            name: "future-extension".to_owned(),
        },
    ];

    for (index, representation) in
        representations.into_iter().enumerate()
    {
        let snapshot = SnapshotBuilder::new(
            snapshot_id(600 + index as u64),
            QubitCount::new(2),
            representation,
        )
        .build(vec![1, 2, 3])
        .expect("all representations must fit the common envelope");

        assert!(snapshot.validate().is_ok());
        assert_eq!(
            snapshot.payload_size(),
            ByteCount::new(3)
        );
    }
}

// =============================================================================
// Final production contract
// =============================================================================

#[test]
fn production_snapshot_contract_is_complete() {
    let snapshot = SnapshotBuilder::new(
        snapshot_id(700),
        QubitCount::new(2),
        StateRepresentation::StateVector,
    )
    .storage_location(SnapshotStorageLocation::Host)
    .precision(SnapshotPrecision::F64)
    .endianness(SnapshotEndianness::Little)
    .payload_encoding(SnapshotPayloadEncoding::Raw)
    .integrity(
        SnapshotIntegrity::new(
            SnapshotIntegrityAlgorithm::Sha256,
            vec![0; 32],
        )
        .expect("valid integrity"),
    )
    .metadata(SnapshotMetadata {
        description: Some("production-contract".to_owned()),
        labels: vec![
            "production".to_owned(),
            "integration".to_owned(),
        ],
        zamani_version: Some("1.0".to_owned()),
        program_identity: Some("zamani-program".to_owned()),
        execution_identity: Some("execution-700".to_owned()),
        provider: Some("local".to_owned()),
        provider_version: Some("1".to_owned()),
    })
    .build(vec![0, 1, 2, 3])
    .expect("complete production snapshot must build");

    assert_eq!(
        snapshot.header.magic,
        SNAPSHOT_MAGIC
    );

    assert_eq!(
        snapshot.header.schema_id,
        SNAPSHOT_SCHEMA_ID
    );

    assert_eq!(
        snapshot.header.format_version,
        SnapshotFormatVersion::CURRENT
    );

    assert_eq!(
        snapshot.header.qubit_count,
        QubitCount::new(2)
    );

    assert_eq!(
        snapshot.header.representation,
        StateRepresentation::StateVector
    );

    assert_eq!(
        snapshot.header.storage_location,
        SnapshotStorageLocation::Host
    );

    assert_eq!(
        snapshot.header.precision,
        SnapshotPrecision::F64
    );

    assert_eq!(
        snapshot.header.endianness,
        SnapshotEndianness::Little
    );

    assert_eq!(
        snapshot.header.payload_encoding,
        SnapshotPayloadEncoding::Raw
    );

    assert_eq!(
        snapshot.payload_size(),
        ByteCount::new(4)
    );

    assert!(snapshot.validate().is_ok());
}

// =============================================================================
// Compile-time-facing documentation contract
// =============================================================================
//
// This test deliberately does nothing at runtime. Its purpose is to keep the
// integration file explicit about the API boundary that future memory modules
// must satisfy.
//
// The snapshot subsystem must remain usable by:
//!
//! - state_vector.rs
//! - density_matrix.rs
//! - stabilizer.rs
//! - sparse.rs
//! - tensor_network.rs
//! - backend_state.rs
//! - gpu.rs
//! - distributed.rs
//! - migration.rs
//! - checkpoint.rs
//! - serialization.rs
//!
//! without modifying this test contract merely because a new provider or state
//! representation is introduced.
//
// The concrete provider-neutral abstraction is `SnapshotProvider`.

#[test]
fn snapshot_provider_trait_remains_the_provider_boundary() {
    fn assert_provider<P: SnapshotProvider>() {}

    assert_provider::<MockProvider>();
    assert_provider::<RejectingProvider>();
}