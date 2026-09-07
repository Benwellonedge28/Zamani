//! Zamani Quantum Resilience — Checkpoint Integration Tests
//!
//! Path:
//!     src/quantum/resilience/tests/checkpoint.rs
//!
//! Purpose:
//!     Production-oriented tests for the complete checkpoint contract.
//!
//! Scope:
//!     - checkpoint identity and validation;
//!     - checkpoint creation;
//!     - authorization and lifecycle;
//!     - semantic boundaries;
//!     - restore semantics;
//!     - reconstructibility;
//!     - canonical QubitId integration;
//!     - dynamic/sparse resource scopes;
//!     - payload descriptors;
//!     - integrity metadata;
//!     - provenance;
//!     - expiration;
//!     - storage references;
//!     - checkpoint references;
//!     - deterministic serialization;
//!     - large logical identifiers;
//!     - failure isolation;
//!     - compatibility with checkpoint manifests;
//!     - production safety invariants.
//!
//! Architectural rule:
//!
//!     These tests verify contracts. They must not depend on a particular
//!     quantum provider, QPU topology, number of qubits, storage vendor,
//!     compiler implementation, scheduler implementation, routing algorithm,
//!     QEC implementation, or retry count.
//!
//! Scalability:
//!
//!     The tests deliberately avoid production-size constants such as:
//!
//!         MAX_QUBITS
//!         MAX_CHECKPOINTS
//!         MAX_ARTIFACTS
//!         MAX_DEVICES
//!
//!     Test data uses generated collections and boundary identifiers only.
//!     Any concrete allocation remains bounded by the test process resources.
//!
//! Canonical quantum identity:
//!
//!     All quantum resource tests use:
//!
//!         crate::quantum::ir::qubit::QubitId
//!
//!     The checkpoint subsystem must never introduce a competing QubitId.
//!
//! Quantum correctness:
//!
//!     These tests explicitly verify that an arbitrary unknown quantum state
//!     cannot be represented as though it were automatically restorable.
//!
//! Safety:
//!
//!     This file contains no unsafe code.
//!
//! Rust:
//!
//!     Rust 1.97 / 1.97.1
//!     Rust 2021
//!     stable Rust
//!
//! Integration:
//!
//!     `src/quantum/resilience/tests/mod.rs` must expose this module with:
//!
//!         #[cfg(test)]
//!         mod checkpoint;
//!
//!     The tests consume only public checkpoint contracts so that later
//!     implementation changes remain localized.
//!
//! Related modules:
//!
//!     quantum::resilience::checkpoint::checkpoint
//!     quantum::resilience::checkpoint::manifest
//!     quantum::resilience::checkpoint::storage
//!     quantum::resilience::checkpoint::integrity
//!     quantum::resilience::checkpoint::compatibility
//!     quantum::resilience::recovery
//!     quantum::resilience::verification
//!
//! No test in this file should become a hidden implementation dependency.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::collections::BTreeMap;
use std::time::{Duration, UNIX_EPOCH};

use crate::quantum::ir::qubit::QubitId;

use crate::quantum::resilience::checkpoint::checkpoint::{
    ArtifactId,
    Checkpoint,
    CheckpointAuthorization,
    CheckpointBoundary,
    CheckpointCreateRequest,
    CheckpointEligibility,
    CheckpointError,
    CheckpointFactory,
    CheckpointId,
    CheckpointLifecycle,
    CheckpointPayload,
    CheckpointPayloadLocation,
    CheckpointProvenance,
    CheckpointResourceScope,
    CheckpointStateKind,
    CheckpointTimestamp,
    CompressionDescriptor,
    DefaultCheckpointFactory,
    EncryptionDescriptor,
    ExecutionId,
    ExecutionPosition,
    IntegrityDescriptor,
    OperationId,
    ProgramId,
    RestoreSemantics,
    TargetId,
};

use crate::quantum::resilience::checkpoint::manifest::{
    CheckpointManifest,
    ManifestArtifact,
    ManifestArtifactRole,
    ManifestId,
    ManifestLineage,
    ManifestSchemaVersion,
    ManifestState,
};

use crate::quantum::resilience::checkpoint::storage::{
    StorageNamespace,
    StorageObjectId,
    StorageObjectReference,
    StorageVersion,
};

// ============================================================================
// Test data builders
// ============================================================================

fn checkpoint_id(value: &str) -> CheckpointId {
    CheckpointId::new(value).expect("test checkpoint identifier must be valid")
}

fn execution_id(value: &str) -> ExecutionId {
    ExecutionId::new(value).expect("test execution identifier must be valid")
}

fn program_id(value: &str) -> ProgramId {
    ProgramId::new(value).expect("test program identifier must be valid")
}

fn artifact_id(value: &str) -> ArtifactId {
    ArtifactId::new(value).expect("test artifact identifier must be valid")
}

fn operation_id(value: &str) -> OperationId {
    OperationId::new(value).expect("test operation identifier must be valid")
}

fn manifest_id(value: &str) -> ManifestId {
    ManifestId::new(value).expect("test manifest identifier must be valid")
}

fn test_timestamp() -> CheckpointTimestamp {
    CheckpointTimestamp::new(1_000, 123_456_789)
        .expect("test timestamp must be valid")
}

fn test_provenance() -> CheckpointProvenance {
    CheckpointProvenance {
        program_id: program_id("program-test"),
        execution_id: execution_id("execution-test"),
        ir_schema_version: "zamani.ir.v1".to_owned(),
        program_fingerprint: "program-fingerprint".to_owned(),
        compilation_fingerprint: Some("compilation-fingerprint".to_owned()),
        routing_fingerprint: Some("routing-fingerprint".to_owned()),
        scheduling_fingerprint: Some("scheduling-fingerprint".to_owned()),
        optimization_fingerprint: Some("optimization-fingerprint".to_owned()),
        qec_fingerprint: Some("qec-fingerprint".to_owned()),
        capability_fingerprint: "capability-fingerprint".to_owned(),
    }
}

fn test_payload() -> CheckpointPayload {
    CheckpointPayload {
        artifact_id: artifact_id("artifact-test"),
        location: CheckpointPayloadLocation::Reconstructible,
        media_type: "application/zamani-checkpoint".to_owned(),
        byte_length: None,
        integrity: IntegrityDescriptor {
            algorithm: "test-integrity".to_owned(),
            digest: "test-digest".to_owned(),
            authenticity_reference: None,
        },
        compression: CompressionDescriptor::None,
        encryption: EncryptionDescriptor::None,
    }
}

fn program_start_request() -> CheckpointCreateRequest {
    CheckpointCreateRequest {
        checkpoint_id: checkpoint_id("checkpoint-test"),
        execution_id: execution_id("execution-test"),
        program_id: program_id("program-test"),
        boundary: CheckpointBoundary::ProgramStart,
        state_kind: CheckpointStateKind::ReplayableProgram,
        restore_semantics: RestoreSemantics::Replay,
        execution_position: ExecutionPosition::new("program-start")
            .expect("test position must be valid"),
        resource_scope: CheckpointResourceScope::execution(),
        provenance: test_provenance(),
        payload: test_payload(),
        expires_at: None,
        reason: Some("production checkpoint test".to_owned()),
    }
}

fn create_program_start_checkpoint() -> Checkpoint {
    DefaultCheckpointFactory
        .create_checkpoint(program_start_request())
        .expect("program-start checkpoint must be constructible")
}

// ============================================================================
// Identifier tests
// ============================================================================

#[test]
fn identifiers_reject_empty_values() {
    assert!(CheckpointId::new("").is_err());
    assert!(ExecutionId::new("   ").is_err());
    assert!(ProgramId::new("\t").is_err());
    assert!(ArtifactId::new("").is_err());
    assert!(OperationId::new(" ").is_err());
    assert!(ManifestId::new("").is_err());
    assert!(TargetId::new(" ").is_err());
}

#[test]
fn identifiers_are_opaque_and_round_trip() {
    let id = checkpoint_id("checkpoint/opaque/identity");

    assert_eq!(id.as_str(), "checkpoint/opaque/identity");
    assert_eq!(id.to_string(), "checkpoint/opaque/identity");
    assert_eq!(
        id.clone().into_string(),
        "checkpoint/opaque/identity".to_owned()
    );
}

// ============================================================================
// Timestamp tests
// ============================================================================

#[test]
fn timestamp_round_trips_after_unix_epoch() {
    let timestamp = CheckpointTimestamp::new(123, 456_789)
        .expect("timestamp must be valid");

    let system_time = timestamp
        .to_system_time()
        .expect("timestamp must convert");

    let restored = CheckpointTimestamp::from_system_time(system_time)
        .expect("timestamp must round-trip");

    assert_eq!(timestamp, restored);
}

#[test]
fn timestamp_rejects_invalid_nanoseconds() {
    assert!(CheckpointTimestamp::new(0, 1_000_000_000).is_err());
    assert!(CheckpointTimestamp::new(0, u32::MAX).is_err());
}

#[test]
fn timestamp_supports_unix_epoch() {
    let timestamp =
        CheckpointTimestamp::new(0, 0).expect("epoch timestamp is valid");

    assert_eq!(
        timestamp
            .to_system_time()
            .expect("epoch must convert"),
        UNIX_EPOCH
    );
}

#[test]
fn timestamp_supports_negative_epoch_seconds() {
    let timestamp =
        CheckpointTimestamp::new(-10, 0).expect("negative timestamp is valid");

    let system_time = timestamp
        .to_system_time()
        .expect("negative timestamp must convert");

    let restored = CheckpointTimestamp::from_system_time(system_time)
        .expect("negative timestamp must round-trip");

    assert_eq!(timestamp, restored);
}

#[test]
fn timestamp_supports_negative_seconds_with_fractional_nanoseconds() {
    // This is an important boundary case.
    //
    // -1 seconds + 500_000_000ns represents -0.5 seconds from the epoch.
    //
    // The test intentionally verifies the canonical mathematical
    // representation rather than merely testing positive timestamps.
    let timestamp =
        CheckpointTimestamp::new(-1, 500_000_000)
            .expect("fractional negative timestamp is valid");

    let system_time = timestamp
        .to_system_time()
        .expect("fractional negative timestamp must convert");

    let restored = CheckpointTimestamp::from_system_time(system_time)
        .expect("fractional negative timestamp must round-trip");

    assert_eq!(timestamp, restored);

    assert_eq!(
        system_time,
        UNIX_EPOCH
            .checked_sub(Duration::from_nanos(500_000_000))
            .expect("test time must be representable")
    );
}

// ============================================================================
// Resource scope tests
// ============================================================================

#[test]
fn execution_scope_is_unbounded_by_machine_size() {
    let scope = CheckpointResourceScope::execution();

    assert!(scope.qubits().is_none());
    assert_eq!(scope, CheckpointResourceScope::execution());
}

#[test]
fn logical_resource_scope_uses_canonical_qubit_identity() {
    let qubits = [
        QubitId::new(0),
        QubitId::new(1),
        QubitId::new(2),
    ];

    let scope = CheckpointResourceScope::logical_qubits(qubits)
        .expect("unique canonical qubits must be accepted");

    assert_eq!(
        scope.qubits().expect("logical scope must expose qubits"),
        &qubits
    );
}

#[test]
fn physical_resource_scope_uses_canonical_qubit_identity() {
    let qubits = [
        QubitId::new(7),
        QubitId::new(42),
        QubitId::new(usize::MAX),
    ];

    let scope = CheckpointResourceScope::physical_qubits(qubits)
        .expect("sparse canonical qubits must be accepted");

    assert_eq!(
        scope.qubits().expect("physical scope must expose qubits"),
        &qubits
    );
}

#[test]
fn duplicate_logical_qubits_are_rejected() {
    let duplicate = [
        QubitId::new(4),
        QubitId::new(4),
    ];

    let result = CheckpointResourceScope::logical_qubits(duplicate);

    assert!(matches!(
        result,
        Err(CheckpointError::DuplicateQubit)
    ));
}

#[test]
fn duplicate_physical_qubits_are_rejected() {
    let duplicate = [
        QubitId::new(usize::MAX),
        QubitId::new(usize::MAX),
    ];

    let result = CheckpointResourceScope::physical_qubits(duplicate);

    assert!(matches!(
        result,
        Err(CheckpointError::DuplicateQubit)
    ));
}

#[test]
fn sparse_qubit_identifiers_are_not_treated_as_machine_size() {
    let qubits = [
        QubitId::new(0),
        QubitId::new(usize::MAX / 2),
        QubitId::new(usize::MAX),
    ];

    let scope = CheckpointResourceScope::logical_qubits(qubits)
        .expect("sparse identifiers must be valid");

    assert_eq!(
        scope
            .qubits()
            .expect("explicit scope must expose qubits")
            .len(),
        3
    );
}

// ============================================================================
// Payload tests
// ============================================================================

#[test]
fn reconstructible_payload_does_not_require_byte_length() {
    let payload = CheckpointPayload {
        location: CheckpointPayloadLocation::Reconstructible,
        byte_length: None,
        ..test_payload()
    };

    assert!(payload.validate().is_ok());
}

#[test]
fn reconstructible_payload_rejects_declared_byte_length() {
    let payload = CheckpointPayload {
        location: CheckpointPayloadLocation::Reconstructible,
        byte_length: Some(1),
        ..test_payload()
    };

    assert!(matches!(
        payload.validate(),
        Err(CheckpointError::InvalidPayloadMetadata {
            field: "byte_length_for_reconstructible_payload"
        })
    ));
}

#[test]
fn external_payload_reference_must_not_be_empty() {
    assert!(CheckpointPayloadLocation::external("").is_err());
    assert!(CheckpointPayloadLocation::external("   ").is_err());
}

#[test]
fn runtime_payload_reference_must_not_be_empty() {
    assert!(CheckpointPayloadLocation::runtime("").is_err());
    assert!(CheckpointPayloadLocation::runtime("\t").is_err());
}

#[test]
fn provider_payload_reference_must_not_be_empty() {
    assert!(CheckpointPayloadLocation::provider_managed("").is_err());
    assert!(CheckpointPayloadLocation::provider_managed(" ").is_err());
}

#[test]
fn encrypted_payload_requires_algorithm_and_key_reference() {
    let mut payload = test_payload();

    payload.encryption = EncryptionDescriptor::Encrypted {
        algorithm: "".to_owned(),
        key_reference: "key".to_owned(),
    };

    assert!(payload.validate().is_err());

    payload.encryption = EncryptionDescriptor::Encrypted {
        algorithm: "test-encryption".to_owned(),
        key_reference: "".to_owned(),
    };

    assert!(payload.validate().is_err());
}

#[test]
fn compressed_payload_requires_algorithm() {
    let mut payload = test_payload();

    payload.compression = CompressionDescriptor::Algorithm {
        algorithm: "".to_owned(),
    };

    assert!(payload.validate().is_err());
}

// ============================================================================
// Provenance tests
// ============================================================================

#[test]
fn provenance_requires_ir_schema_version() {
    let mut provenance = test_provenance();
    provenance.ir_schema_version.clear();

    assert!(provenance.validate().is_err());
}

#[test]
fn provenance_requires_program_fingerprint() {
    let mut provenance = test_provenance();
    provenance.program_fingerprint.clear();

    assert!(provenance.validate().is_err());
}

#[test]
fn provenance_requires_capability_fingerprint() {
    let mut provenance = test_provenance();
    provenance.capability_fingerprint.clear();

    assert!(provenance.validate().is_err());
}

// ============================================================================
// Execution position tests
// ============================================================================

#[test]
fn execution_position_rejects_empty_values() {
    assert!(ExecutionPosition::new("").is_err());
    assert!(ExecutionPosition::new(" ").is_err());
}

#[test]
fn execution_position_can_reference_an_operation() {
    let position = ExecutionPosition::new("boundary")
        .expect("position must be valid")
        .with_operation(operation_id("operation-1"));

    assert_eq!(
        position.operation_id,
        Some(operation_id("operation-1"))
    );
}

// ============================================================================
// Checkpoint creation tests
// ============================================================================

#[test]
fn program_start_checkpoint_is_valid_and_recoverable() {
    let checkpoint = create_program_start_checkpoint();

    assert!(checkpoint.validate().is_ok());
    assert!(checkpoint.locally_recoverable());

    assert_eq!(
        checkpoint.eligibility(),
        CheckpointEligibility::Eligible
    );

    assert_eq!(
        checkpoint.lifecycle,
        CheckpointLifecycle::Committed
    );

    assert_eq!(
        checkpoint.authorization,
        CheckpointAuthorization::Authorized
    );
}

#[test]
fn checkpoint_factory_is_stateless() {
    let factory = DefaultCheckpointFactory;

    let first = factory
        .create_checkpoint(program_start_request())
        .expect("first checkpoint must be created");

    let second = factory
        .create_checkpoint(program_start_request())
        .expect("second checkpoint must be created");

    assert_eq!(first.checkpoint_id, second.checkpoint_id);
    assert_eq!(first.program_id, second.program_id);
    assert_eq!(first.execution_id, second.execution_id);
    assert_eq!(first.boundary, second.boundary);
    assert_eq!(first.state_kind, second.state_kind);
}

// ============================================================================
// Boundary/state semantic tests
// ============================================================================

#[test]
fn unknown_state_is_not_restorable() {
    let mut request = program_start_request();

    request.state_kind = CheckpointStateKind::Unknown;

    assert!(request.validate().is_err());
}

#[test]
fn invalid_state_is_not_restorable() {
    let mut request = program_start_request();

    request.state_kind = CheckpointStateKind::Invalid;

    assert!(request.validate().is_err());
}

#[test]
fn all_supported_checkpoint_semantics_are_explicit() {
    let cases = [
        (
            CheckpointBoundary::ProgramStart,
            CheckpointStateKind::ReplayableProgram,
            RestoreSemantics::Replay,
        ),
        (
            CheckpointBoundary::ClassicalExecution,
            CheckpointStateKind::Classical,
            RestoreSemantics::ClassicalState,
        ),
        (
            CheckpointBoundary::Measurement,
            CheckpointStateKind::MeasurementBoundary,
            RestoreSemantics::Replay,
        ),
        (
            CheckpointBoundary::LogicalQec,
            CheckpointStateKind::LogicalQec,
            RestoreSemantics::LogicalQecState,
        ),
        (
            CheckpointBoundary::ProviderSupportedSnapshot,
            CheckpointStateKind::SupportedQuantumSnapshot,
            RestoreSemantics::ProviderSnapshot,
        ),
        (
            CheckpointBoundary::ReconstructibleRuntime,
            CheckpointStateKind::ReconstructibleRuntime,
            RestoreSemantics::RuntimeSnapshot,
        ),
        (
            CheckpointBoundary::CompiledExecution,
            CheckpointStateKind::CompiledExecution,
            RestoreSemantics::CompiledState,
        ),
    ];

    for (boundary, state_kind, restore_semantics) in cases {
        let mut request = program_start_request();

        request.boundary = boundary;
        request.state_kind = state_kind;
        request.restore_semantics = restore_semantics;

        assert!(
            request.validate().is_ok(),
            "supported boundary/state/restore combination must validate"
        );
    }
}

#[test]
fn mismatched_boundary_state_semantics_are_rejected() {
    let mut request = program_start_request();

    request.boundary = CheckpointBoundary::Measurement;

    assert!(request.validate().is_err());
}

#[test]
fn provider_snapshot_is_not_assumed_to_be_generic_replay() {
    let mut request = program_start_request();

    request.boundary = CheckpointBoundary::ProviderSupportedSnapshot;
    request.state_kind = CheckpointStateKind::SupportedQuantumSnapshot;
    request.restore_semantics = RestoreSemantics::ProviderSnapshot;
    request.payload = CheckpointPayload {
        location: CheckpointPayloadLocation::provider_managed(
            "provider-snapshot",
        )
        .expect("provider reference must be valid"),
        ..test_payload()
    };

    let checkpoint = DefaultCheckpointFactory
        .create_checkpoint(request)
        .expect("provider checkpoint must be constructible");

    assert_eq!(
        checkpoint.eligibility(),
        CheckpointEligibility::RequiresTargetSupport
    );

    assert!(
        checkpoint.boundary.is_provider_snapshot()
    );
}

// ============================================================================
// Identity consistency tests
// ============================================================================

#[test]
fn mismatched_execution_provenance_is_rejected() {
    let mut request = program_start_request();

    request.execution_id = execution_id("different-execution");

    assert!(matches!(
        request.validate(),
        Err(CheckpointError::ExecutionIdentityMismatch)
    ));
}

#[test]
fn mismatched_program_provenance_is_rejected() {
    let mut request = program_start_request();

    request.program_id = program_id("different-program");

    assert!(matches!(
        request.validate(),
        Err(CheckpointError::ProgramIdentityMismatch)
    ));
}

// ============================================================================
// Expiration tests
// ============================================================================

#[test]
fn expiration_before_creation_is_rejected() {
    let created =
        CheckpointTimestamp::new(100, 0).expect("timestamp must be valid");

    let expired =
        CheckpointTimestamp::new(99, 0).expect("timestamp must be valid");

    let mut request = program_start_request();
    request.expires_at = Some(expired);

    assert!(request.validate().is_err());

    let checkpoint = Checkpoint {
        schema_id:
            crate::quantum::resilience::checkpoint::checkpoint::CHECKPOINT_SCHEMA_ID
                .to_owned(),
        schema_version:
            crate::quantum::resilience::checkpoint::checkpoint::CHECKPOINT_SCHEMA_VERSION,
        checkpoint_id: request.checkpoint_id,
        execution_id: request.execution_id,
        program_id: request.program_id,
        boundary: request.boundary,
        state_kind: request.state_kind,
        restore_semantics: request.restore_semantics,
        execution_position: request.execution_position,
        resource_scope: request.resource_scope,
        provenance: request.provenance,
        payload: request.payload,
        created_at: created,
        expires_at: Some(expired),
        authorization: CheckpointAuthorization::Authorized,
        lifecycle: CheckpointLifecycle::Committed,
        reason: None,
    };

    assert!(checkpoint.validate().is_err());
}

#[test]
fn non_expiring_checkpoint_is_never_expired() {
    let checkpoint = create_program_start_checkpoint();

    let now =
        CheckpointTimestamp::new(i64::MAX, 999_999_999)
            .expect("maximum representable test timestamp must be valid");

    assert!(
        !checkpoint
            .is_expired_at(now)
            .expect("expiration check must succeed")
    );
}

#[test]
fn expiration_boundary_is_not_expired_at_exact_expiration() {
    let expiration =
        CheckpointTimestamp::new(100, 0).expect("timestamp must be valid");

    let mut request = program_start_request();
    request.expires_at = Some(expiration);

    let checkpoint = request
        .commit(test_timestamp())
        .expect("checkpoint must be created");

    assert!(
        !checkpoint
            .is_expired_at(expiration)
            .expect("expiration check must succeed")
    );

    let later =
        CheckpointTimestamp::new(101, 0).expect("timestamp must be valid");

    assert!(
        checkpoint
            .is_expired_at(later)
            .expect("expiration check must succeed")
    );
}

// ============================================================================
// Lifecycle tests
// ============================================================================

#[test]
fn committed_checkpoint_is_locally_recoverable() {
    let checkpoint = create_program_start_checkpoint();

    assert!(checkpoint.locally_recoverable());
    assert!(checkpoint.lifecycle.is_recoverable());
}

#[test]
fn revoked_checkpoint_is_not_locally_recoverable() {
    let mut checkpoint = create_program_start_checkpoint();

    checkpoint.lifecycle = CheckpointLifecycle::Revoked;
    checkpoint.authorization = CheckpointAuthorization::Revoked;

    assert!(!checkpoint.locally_recoverable());
    assert_eq!(
        checkpoint.eligibility(),
        CheckpointEligibility::Ineligible
    );
}

#[test]
fn retired_checkpoint_is_not_locally_recoverable() {
    let mut checkpoint = create_program_start_checkpoint();

    checkpoint.lifecycle = CheckpointLifecycle::Retired;

    assert!(!checkpoint.locally_recoverable());
    assert_eq!(
        checkpoint.eligibility(),
        CheckpointEligibility::Ineligible
    );
}

#[test]
fn lifecycle_transition_rejects_invalid_transition() {
    let checkpoint = create_program_start_checkpoint();

    let result = crate::quantum::resilience::checkpoint::checkpoint::transition(
        &checkpoint,
        crate::quantum::resilience::checkpoint::checkpoint::CheckpointTransition::Commit,
    );

    assert!(matches!(
        result,
        Err(CheckpointError::InvalidLifecycleTransition)
    ));
}

#[test]
fn lifecycle_transition_revoke_changes_authorization() {
    let checkpoint = create_program_start_checkpoint();

    let revoked =
        crate::quantum::resilience::checkpoint::checkpoint::transition(
            &checkpoint,
            crate::quantum::resilience::checkpoint::checkpoint::CheckpointTransition::Revoke,
        )
        .expect("committed checkpoint must be revocable");

    assert_eq!(revoked.lifecycle, CheckpointLifecycle::Revoked);
    assert_eq!(
        revoked.authorization,
        CheckpointAuthorization::Revoked
    );
    assert!(!revoked.locally_recoverable());
}

#[test]
fn lifecycle_transition_retire_is_only_valid_after_revoke() {
    let checkpoint = create_program_start_checkpoint();

    let revoked =
        crate::quantum::resilience::checkpoint::checkpoint::transition(
            &checkpoint,
            crate::quantum::resilience::checkpoint::checkpoint::CheckpointTransition::Revoke,
        )
        .expect("checkpoint must be revocable");

    let retired =
        crate::quantum::resilience::checkpoint::checkpoint::transition(
            &revoked,
            crate::quantum::resilience::checkpoint::checkpoint::CheckpointTransition::Retire,
        )
        .expect("revoked checkpoint must be retireable");

    assert_eq!(retired.lifecycle, CheckpointLifecycle::Retired);
}

// ============================================================================
// Schema validation tests
// ============================================================================

#[test]
fn checkpoint_rejects_wrong_schema_id() {
    let mut checkpoint = create_program_start_checkpoint();

    checkpoint.schema_id = "foreign.schema".to_owned();

    assert!(matches!(
        checkpoint.validate(),
        Err(CheckpointError::SchemaMismatch { .. })
    ));
}

#[test]
fn checkpoint_rejects_zero_schema_version() {
    let mut checkpoint = create_program_start_checkpoint();

    checkpoint.schema_version = 0;

    assert!(matches!(
        checkpoint.validate(),
        Err(CheckpointError::InvalidSchemaVersion)
    ));
}

#[test]
fn checkpoint_rejects_future_schema_version() {
    let mut checkpoint = create_program_start_checkpoint();

    checkpoint.schema_version =
        crate::quantum::resilience::checkpoint::checkpoint::CHECKPOINT_SCHEMA_VERSION
            + 1;

    assert!(matches!(
        checkpoint.validate(),
        Err(CheckpointError::UnsupportedSchemaVersion { .. })
    ));
}

// ============================================================================
// Storage reference tests
// ============================================================================

#[test]
fn storage_namespace_rejects_empty_value() {
    assert!(StorageNamespace::new("").is_err());
    assert!(StorageNamespace::new(" ").is_err());
}

#[test]
fn storage_object_id_rejects_empty_value() {
    assert!(StorageObjectId::new("").is_err());
    assert!(StorageObjectId::new("\t").is_err());
}

#[test]
fn storage_version_rejects_empty_value() {
    assert!(StorageVersion::new("").is_err());
    assert!(StorageVersion::new(" ").is_err());
}

#[test]
fn storage_object_reference_preserves_opaque_identity() {
    let namespace =
        StorageNamespace::new("checkpoint-test")
            .expect("namespace must be valid");

    let object =
        StorageObjectId::new("object-opaque-id")
            .expect("object ID must be valid");

    let version =
        StorageVersion::new("generation-opaque-id")
            .expect("version must be valid");

    let reference =
        StorageObjectReference::new(namespace.clone(), object.clone())
            .with_version(version.clone());

    assert_eq!(reference.namespace, namespace);
    assert_eq!(reference.object_id, object);
    assert_eq!(reference.version, Some(version));
}

// ============================================================================
// Manifest integration tests
// ============================================================================

#[test]
fn manifest_can_reference_checkpoint_artifacts_without_payload_bytes() {
    let checkpoint = create_program_start_checkpoint();

    let artifact = ManifestArtifact::required(
        checkpoint.payload.artifact_id.clone(),
        ManifestArtifactRole::ProgramRepresentation,
        0,
    )
    .with_media_type("application/zamani-checkpoint")
    .with_logical_byte_length(0);

    let manifest = CheckpointManifest::new(
        manifest_id("manifest-test"),
        checkpoint.checkpoint_id.clone(),
        checkpoint.created_at,
        [artifact],
    )
    .expect("manifest must be constructible");

    assert_eq!(manifest.artifact_count(), 1);
    assert!(!manifest.is_empty());
    assert_eq!(
        manifest.artifact(&checkpoint.payload.artifact_id)
            .expect("artifact must be discoverable")
            .role,
        ManifestArtifactRole::ProgramRepresentation
    );
}

#[test]
fn manifest_preserves_explicit_artifact_order() {
    let checkpoint = create_program_start_checkpoint();

    let artifacts = [
        ManifestArtifact::required(
            artifact_id("artifact-a"),
            ManifestArtifactRole::Metadata,
            10,
        ),
        ManifestArtifact::required(
            artifact_id("artifact-b"),
            ManifestArtifactRole::ExecutionState,
            20,
        ),
        ManifestArtifact::optional(
            artifact_id("artifact-c"),
            ManifestArtifactRole::Provenance,
            30,
        ),
    ];

    let manifest = CheckpointManifest::new(
        manifest_id("manifest-order"),
        checkpoint.checkpoint_id,
        checkpoint.created_at,
        artifacts,
    )
    .expect("manifest must be valid");

    assert_eq!(
        manifest
            .artifacts()
            .iter()
            .map(|artifact| artifact.sequence)
            .collect::<Vec<_>>(),
        vec![10, 20, 30]
    );
}

#[test]
fn manifest_can_use_sparse_large_sequence_numbers() {
    let checkpoint = create_program_start_checkpoint();

    let artifacts = [
        ManifestArtifact::required(
            artifact_id("artifact-low"),
            ManifestArtifactRole::Metadata,
            0,
        ),
        ManifestArtifact::required(
            artifact_id("artifact-high"),
            ManifestArtifactRole::ExecutionState,
            u64::MAX,
        ),
    ];

    let manifest = CheckpointManifest::new(
        manifest_id("manifest-sparse"),
        checkpoint.checkpoint_id,
        checkpoint.created_at,
        artifacts,
    )
    .expect("manifest must support sparse sequence values");

    assert_eq!(
        manifest.artifacts()[1].sequence,
        u64::MAX
    );
}

#[test]
fn manifest_lineage_rejects_self_reference() {
    let checkpoint = create_program_start_checkpoint();

    let mut manifest = CheckpointManifest::new(
        manifest_id("manifest-lineage"),
        checkpoint.checkpoint_id,
        checkpoint.created_at,
        [],
    )
    .expect("empty manifest must be constructible if contract permits it");

    let own_id = manifest.manifest_id.clone();

    let lineage = ManifestLineage {
        parent_manifest_id: Some(own_id),
        supersedes_manifest_id: None,
        reason: Some("invalid self reference".to_owned()),
    };

    assert!(manifest.set_lineage(lineage).is_err());
}

#[test]
fn manifest_schema_version_is_explicit() {
    assert!(
        ManifestSchemaVersion::CURRENT.compatible_major(
            ManifestSchemaVersion::CURRENT
        )
    );

    assert_eq!(
        ManifestSchemaVersion::CURRENT.major,
        crate::quantum::resilience::checkpoint::manifest::CHECKPOINT_MANIFEST_SCHEMA_MAJOR
    );
}

#[test]
fn manifest_metadata_is_deterministically_ordered() {
    let checkpoint = create_program_start_checkpoint();

    let mut metadata = BTreeMap::new();
    metadata.insert("z".to_owned(), "last".to_owned());
    metadata.insert("a".to_owned(), "first".to_owned());

    let mut manifest = CheckpointManifest::new(
        manifest_id("manifest-metadata"),
        checkpoint.checkpoint_id,
        checkpoint.created_at,
        [],
    )
    .expect("manifest must be constructible");

    for (key, value) in metadata {
        manifest
            .insert_metadata(key, value)
            .expect("draft manifest must accept metadata");
    }

    let keys = manifest.metadata.keys().cloned().collect::<Vec<_>>();

    assert_eq!(
        keys,
        vec!["a".to_owned(), "z".to_owned()]
    );
}

// ============================================================================
// Serialization tests
// ============================================================================

#[test]
fn checkpoint_is_json_serializable() {
    let checkpoint = create_program_start_checkpoint();

    let encoded =
        serde_json::to_string(&checkpoint)
            .expect("checkpoint must serialize");

    assert!(!encoded.is_empty());

    let decoded: Checkpoint =
        serde_json::from_str(&encoded)
            .expect("checkpoint must deserialize");

    assert_eq!(checkpoint, decoded);
}

#[test]
fn checkpoint_serialization_is_deterministic_for_identical_values() {
    let checkpoint = create_program_start_checkpoint();

    let first =
        serde_json::to_string(&checkpoint)
            .expect("checkpoint must serialize");

    let second =
        serde_json::to_string(&checkpoint)
            .expect("checkpoint must serialize");

    assert_eq!(first, second);
}

#[test]
fn manifest_is_json_serializable() {
    let checkpoint = create_program_start_checkpoint();

    let artifact = ManifestArtifact::required(
        checkpoint.payload.artifact_id.clone(),
        ManifestArtifactRole::Metadata,
        0,
    );

    let manifest = CheckpointManifest::new(
        manifest_id("manifest-serialization"),
        checkpoint.checkpoint_id,
        checkpoint.created_at,
        [artifact],
    )
    .expect("manifest must be valid");

    let encoded =
        serde_json::to_string(&manifest)
            .expect("manifest must serialize");

    let decoded: CheckpointManifest =
        serde_json::from_str(&encoded)
            .expect("manifest must deserialize");

    assert_eq!(manifest, decoded);
}

// ============================================================================
// Quantum-state safety tests
// ============================================================================

#[test]
fn unknown_quantum_state_cannot_be_accepted_as_checkpoint() {
    let mut request = program_start_request();

    request.state_kind = CheckpointStateKind::Unknown;
    request.boundary =
        CheckpointBoundary::ProviderSupportedSnapshot;
    request.restore_semantics =
        RestoreSemantics::ProviderSnapshot;

    request.payload = CheckpointPayload {
        location: CheckpointPayloadLocation::provider_managed(
            "opaque-provider-state",
        )
        .expect("reference must be valid"),
        ..test_payload()
    };

    let result =
        DefaultCheckpointFactory.create_checkpoint(request);

    assert!(result.is_err());
}

#[test]
fn supported_quantum_snapshot_requires_explicit_restore_semantics() {
    let mut request = program_start_request();

    request.boundary =
        CheckpointBoundary::ProviderSupportedSnapshot;
    request.state_kind =
        CheckpointStateKind::SupportedQuantumSnapshot;
    request.restore_semantics =
        RestoreSemantics::Replay;

    request.payload = CheckpointPayload {
        location: CheckpointPayloadLocation::provider_managed(
            "provider-snapshot",
        )
        .expect("reference must be valid"),
        ..test_payload()
    };

    assert!(request.validate().is_err());
}

// ============================================================================
// Resource-scale tests
// ============================================================================

#[test]
fn generated_qubit_scope_has_no_fixed_machine_width_assumption() {
    // The collection size is generated from the test input rather than
    // represented by a production constant.
    //
    // This intentionally exercises ordinary contiguous IDs plus a sparse
    // identifier at the architectural boundary.
    let generated = (0_usize..64_usize)
        .map(QubitId::new)
        .chain([QubitId::new(usize::MAX)])
        .collect::<Vec<_>>();

    let scope = CheckpointResourceScope::logical_qubits(generated.clone())
        .expect("generated unique qubits must be accepted");

    assert_eq!(
        scope
            .qubits()
            .expect("explicit scope must expose qubits")
            .len(),
        generated.len()
    );
}

#[test]
fn empty_explicit_resource_scope_is_supported_without_inventing_qubits() {
    let logical =
        CheckpointResourceScope::logical_qubits([])
            .expect("empty logical scope must be representable");

    let physical =
        CheckpointResourceScope::physical_qubits([])
            .expect("empty physical scope must be representable");

    assert_eq!(
        logical.qubits().expect("logical scope must expose collection"),
        &[]
    );

    assert_eq!(
        physical.qubits().expect("physical scope must expose collection"),
        &[]
    );
}

// ============================================================================
// Boundary helpers
// ============================================================================

#[test]
fn provider_snapshot_boundary_is_target_dependent() {
    assert!(
        CheckpointBoundary::ProviderSupportedSnapshot
            .requires_explicit_restore_support()
    );

    assert!(
        CheckpointBoundary::LogicalQec
            .requires_explicit_restore_support()
    );

    assert!(
        !CheckpointBoundary::ProgramStart
            .requires_explicit_restore_support()
    );
}

#[test]
fn restore_semantics_report_target_dependencies() {
    assert!(
        RestoreSemantics::ProviderSnapshot
            .requires_target_support()
    );

    assert!(
        RestoreSemantics::LogicalQecState
            .requires_target_support()
    );

    assert!(
        RestoreSemantics::RuntimeSnapshot
            .requires_target_support()
    );

    assert!(
        !RestoreSemantics::Replay
            .requires_target_support()
    );
}

// ============================================================================
// Validation-context tests
// ============================================================================

#[test]
fn validation_context_requires_ir_schema() {
    let context =
        crate::quantum::resilience::checkpoint::checkpoint::CheckpointValidationContext {
            execution_id: execution_id("execution"),
            program_id: program_id("program"),
            ir_schema_version: String::new(),
            target_id: TargetId::new("target").expect("target must be valid"),
            capability_fingerprint: "capability".to_owned(),
            restoration_supported: true,
            provider_snapshot_supported: false,
            logical_qec_supported: false,
            runtime_reconstruction_supported: true,
        };

    assert!(context.validate().is_err());
}

#[test]
fn validation_context_requires_capability_fingerprint() {
    let context =
        crate::quantum::resilience::checkpoint::checkpoint::CheckpointValidationContext {
            execution_id: execution_id("execution"),
            program_id: program_id("program"),
            ir_schema_version: "zamani.ir.v1".to_owned(),
            target_id: TargetId::new("target").expect("target must be valid"),
            capability_fingerprint: String::new(),
            restoration_supported: true,
            provider_snapshot_supported: false,
            logical_qec_supported: false,
            runtime_reconstruction_supported: true,
        };

    assert!(context.validate().is_err());
}

#[test]
fn validation_context_rejects_unsupported_restoration() {
    let context =
        crate::quantum::resilience::checkpoint::checkpoint::CheckpointValidationContext {
            execution_id: execution_id("execution"),
            program_id: program_id("program"),
            ir_schema_version: "zamani.ir.v1".to_owned(),
            target_id: TargetId::new("target").expect("target must be valid"),
            capability_fingerprint: "capability".to_owned(),
            restoration_supported: false,
            provider_snapshot_supported: false,
            logical_qec_supported: false,
            runtime_reconstruction_supported: false,
        };

    assert!(matches!(
        context.validate(),
        Err(CheckpointError::TargetRestoreUnsupported)
    ));
}

// ============================================================================
// Immutability / API ownership tests
// ============================================================================

#[test]
fn committed_manifest_is_authoritative_only_when_committed() {
    assert!(!ManifestState::Draft.is_authoritative());
    assert!(!ManifestState::Prepared.is_authoritative());
    assert!(ManifestState::Committed.is_authoritative());
    assert!(!ManifestState::Invalidated.is_authoritative());
    assert!(!ManifestState::Superseded.is_authoritative());
}

#[test]
fn only_draft_manifest_is_mutable() {
    assert!(ManifestState::Draft.is_mutable());
    assert!(!ManifestState::Prepared.is_mutable());
    assert!(!ManifestState::Committed.is_mutable());
    assert!(!ManifestState::Invalidated.is_mutable());
    assert!(!ManifestState::Superseded.is_mutable());
}

// ============================================================================
// Production invariant tests
// ============================================================================

#[test]
fn checkpoint_does_not_encode_provider_identity() {
    let checkpoint = create_program_start_checkpoint();

    let encoded =
        serde_json::to_string(&checkpoint)
            .expect("checkpoint must serialize");

    assert!(!encoded.contains("IBM"));
    assert!(!encoded.contains("AWS"));
    assert!(!encoded.contains("Azure"));
    assert!(!encoded.contains("Google"));
}

#[test]
fn checkpoint_does_not_require_physical_qubit_enumeration_for_execution_scope() {
    let mut request = program_start_request();

    request.resource_scope = CheckpointResourceScope::execution();

    let checkpoint = DefaultCheckpointFactory
        .create_checkpoint(request)
        .expect("execution-wide checkpoint must be constructible");

    assert!(checkpoint.resource_scope.qubits().is_none());
}

#[test]
fn checkpoint_preserves_full_provenance_chain_fields() {
    let checkpoint = create_program_start_checkpoint();

    assert_eq!(
        checkpoint.provenance.ir_schema_version,
        "zamani.ir.v1"
    );

    assert_eq!(
        checkpoint.provenance.program_fingerprint,
        "program-fingerprint"
    );

    assert_eq!(
        checkpoint
            .provenance
            .compilation_fingerprint
            .as_deref(),
        Some("compilation-fingerprint")
    );

    assert_eq!(
        checkpoint
            .provenance
            .routing_fingerprint
            .as_deref(),
        Some("routing-fingerprint")
    );

    assert_eq!(
        checkpoint
            .provenance
            .scheduling_fingerprint
            .as_deref(),
        Some("scheduling-fingerprint")
    );

    assert_eq!(
        checkpoint
            .provenance
            .optimization_fingerprint
            .as_deref(),
        Some("optimization-fingerprint")
    );

    assert_eq!(
        checkpoint
            .provenance
            .qec_fingerprint
            .as_deref(),
        Some("qec-fingerprint")
    );
}

#[test]
fn checkpoint_payload_integrity_metadata_is_not_storage_implementation() {
    let checkpoint = create_program_start_checkpoint();

    assert_eq!(
        checkpoint.payload.integrity.algorithm,
        "test-integrity"
    );

    assert_eq!(
        checkpoint.payload.integrity.digest,
        "test-digest"
    );

    assert!(
        checkpoint.payload.location
            == CheckpointPayloadLocation::Reconstructible
    );
}