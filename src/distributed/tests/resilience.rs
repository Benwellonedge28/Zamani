
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! Integration tests for Zamani distributed resilience.
//!
//! Path:
//!     src/distributed/tests/resilience.rs
//!
//! Rust:
//!     1.97+
//!
//! Edition:
//!     2021
//!
//! Safety:
//!     No unsafe Rust.
//!
//! These tests exercise the existing retry, checkpoint, and recovery
//! contracts through their public interfaces.
//!
//! Test providers are deliberately in-memory implementations. They
//! establish deterministic contract behavior; they do not claim to prove
//! disk durability, distributed consensus, remote authentication,
//! provider correctness, or production fault tolerance.
//!
//! Production guarantees must additionally be established by tests
//! against the actual storage, execution, transport, and security
//! providers selected by the runtime.
//!
//! This file does not define a second resilience implementation.

use std::collections::HashMap;
use std::io::{self, Write};
use std::num::NonZeroUsize;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use std::time::Duration;

use sha2::{Digest, Sha256};

use crate::distributed::config::RetryPolicy;
use crate::distributed::error::{
    DeliveryState,
    DistributedError,
    DistributedResult,
};
use crate::distributed::ids::{
    CheckpointId,
    ExecutionContextId,
    OperationId,
    PlanId,
    TaskId,
};
use crate::distributed::resilience::checkpoint::{
    CheckpointCreateOutcome,
    CheckpointDigest,
    CheckpointError,
    CheckpointManager,
    CheckpointManifest,
    CheckpointPayload,
    CheckpointProviderError,
    CheckpointRestoreTarget,
    CheckpointSafetyEvidence,
    CheckpointStore,
    CheckpointStoreCapabilities,
};
use crate::distributed::resilience::recovery::{
    JournalWrite,
    RecoveryAction,
    RecoveryActionId,
    RecoveryActionKind,
    RecoveryActionStatus,
    RecoveryEngine,
    RecoveryExecutor,
    RecoveryIncident,
    RecoveryIncidentKind,
    RecoveryJournal,
    RecoveryPlan,
    RecoveryRecord,
    RecoveryStatus,
};
use crate::distributed::resilience::retry::{
    PreviousAttemptState,
    RetryCapacity,
    RetryDecision,
    RetryEvaluationInput,
    RetryEvaluator,
    RetryReconciliation,
    RetrySafety,
    RetrySemanticState,
    RetryStopReason,
};

// ============================================================================
// SHARED TEST HELPERS
// ============================================================================

fn retry_policy(max_attempts: usize) -> RetryPolicy {
    RetryPolicy::new(
        NonZeroUsize::new(max_attempts)
            .expect("test retry attempts must be non-zero"),
        Duration::from_millis(10),
        Duration::from_secs(5),
        2,
    )
    .expect("test retry policy must be valid")
}

fn transport_error(delivery: DeliveryState) -> DistributedError {
    DistributedError::TransportFailed {
        operation: "test-submit".to_owned(),
        delivery,
        message: "simulated transport failure".to_owned(),
    }
}

fn evaluate_retry(
    policy: &RetryPolicy,
    error: &DistributedError,
    previous_state: PreviousAttemptState,
    safety: RetrySafety,
    semantic_state: RetrySemanticState,
    retries_already_performed: usize,
    remaining_deadline: Option<Duration>,
    cancelled: bool,
    capacity: RetryCapacity,
) -> RetryDecision {
    let input = RetryEvaluationInput::new(
        error,
        previous_state,
        safety,
        semantic_state,
        retries_already_performed,
        remaining_deadline,
        cancelled,
        capacity,
    );

    RetryEvaluator::new().evaluate(policy, &input)
}

// ============================================================================
// RETRY CONTRACT TESTS
// ============================================================================

#[test]
fn retry_is_permitted_when_request_was_never_submitted() {
    let policy = retry_policy(3);
    let error = transport_error(DeliveryState::NotSent);

    let decision = evaluate_retry(
        &policy,
        &error,
        PreviousAttemptState::NotSubmitted,
        RetrySafety::Never,
        RetrySemanticState::Valid,
        0,
        None,
        false,
        RetryCapacity::available(),
    );

    assert_eq!(
        decision,
        RetryDecision::RetryPermitted {
            delay: Duration::from_millis(10),
            retry_number: 1,
        }
    );
}

#[test]
fn explicit_pre_execution_rejection_does_not_require_idempotency() {
    let policy = retry_policy(3);
    let error = transport_error(DeliveryState::RejectedBeforeExecution);

    let decision = evaluate_retry(
        &policy,
        &error,
        PreviousAttemptState::RejectedBeforeExecution,
        RetrySafety::Never,
        RetrySemanticState::Valid,
        0,
        None,
        false,
        RetryCapacity::available(),
    );

    assert!(decision.retry_permitted());
}

#[test]
fn unknown_execution_outcome_requires_reconciliation() {
    let policy = retry_policy(5);
    let error = transport_error(DeliveryState::ResponseLost);

    let decision = evaluate_retry(
        &policy,
        &error,
        PreviousAttemptState::OutcomeUnknown,
        RetrySafety::DurableDeduplicationConfirmed,
        RetrySemanticState::Valid,
        0,
        None,
        false,
        RetryCapacity::available(),
    );

    assert_eq!(
        decision.reconciliation(),
        Some(RetryReconciliation::EstablishOutcome)
    );

    assert!(!decision.retry_permitted());
}

#[test]
fn transport_error_with_unknown_delivery_requires_reconciliation() {
    let policy = retry_policy(5);
    let error = transport_error(DeliveryState::Unknown);

    let decision = evaluate_retry(
        &policy,
        &error,
        PreviousAttemptState::FailedBeforeResult,
        RetrySafety::Idempotent,
        RetrySemanticState::Valid,
        0,
        None,
        false,
        RetryCapacity::available(),
    );

    assert_eq!(
        decision.reconciliation(),
        Some(RetryReconciliation::EstablishOutcome)
    );
}

#[test]
fn partial_execution_requires_recovery_not_whole_operation_retry() {
    let policy = retry_policy(5);
    let error = DistributedError::ExecutionFailed {
        execution_id: None,
        message: "partial execution".to_owned(),
    };

    let decision = evaluate_retry(
        &policy,
        &error,
        PreviousAttemptState::PartialExecution,
        RetrySafety::Idempotent,
        RetrySemanticState::Valid,
        0,
        None,
        false,
        RetryCapacity::available(),
    );

    assert_eq!(
        decision.reconciliation(),
        Some(RetryReconciliation::RecoverPartialExecution)
    );
}

#[test]
fn completed_non_idempotent_operation_is_not_repeated() {
    let policy = retry_policy(5);
    let error = DistributedError::ExecutionFailed {
        execution_id: None,
        message: "operation failed after execution".to_owned(),
    };

    let decision = evaluate_retry(
        &policy,
        &error,
        PreviousAttemptState::CompletedFailure,
        RetrySafety::Never,
        RetrySemanticState::Valid,
        0,
        None,
        false,
        RetryCapacity::available(),
    );

    assert_eq!(
        decision.stop_reason(),
        Some(RetryStopReason::UnsafeToRepeat)
    );
}

#[test]
fn completed_idempotent_operation_may_be_retried() {
    let policy = retry_policy(3);
    let error = DistributedError::ExecutionFailed {
        execution_id: None,
        message: "idempotent operation failed".to_owned(),
    };

    let decision = evaluate_retry(
        &policy,
        &error,
        PreviousAttemptState::CompletedFailure,
        RetrySafety::Idempotent,
        RetrySemanticState::Valid,
        0,
        None,
        false,
        RetryCapacity::available(),
    );

    assert!(decision.retry_permitted());
}

#[test]
fn successful_operation_is_never_retried() {
    let policy = retry_policy(5);
    let error = transport_error(DeliveryState::NotSent);

    let decision = evaluate_retry(
        &policy,
        &error,
        PreviousAttemptState::Succeeded,
        RetrySafety::Idempotent,
        RetrySemanticState::Valid,
        0,
        None,
        false,
        RetryCapacity::available(),
    );

    assert_eq!(
        decision.stop_reason(),
        Some(RetryStopReason::AlreadySucceeded)
    );
}

#[test]
fn cancellation_prevents_another_attempt() {
    let policy = retry_policy(5);
    let error = transport_error(DeliveryState::NotSent);

    let decision = evaluate_retry(
        &policy,
        &error,
        PreviousAttemptState::NotSubmitted,
        RetrySafety::Idempotent,
        RetrySemanticState::Valid,
        0,
        None,
        true,
        RetryCapacity::available(),
    );

    assert_eq!(
        decision.stop_reason(),
        Some(RetryStopReason::Cancelled)
    );
}

#[test]
fn expired_deadline_prevents_retry() {
    let policy = retry_policy(5);
    let error = transport_error(DeliveryState::NotSent);

    let decision = evaluate_retry(
        &policy,
        &error,
        PreviousAttemptState::NotSubmitted,
        RetrySafety::Idempotent,
        RetrySemanticState::Valid,
        0,
        Some(Duration::ZERO),
        false,
        RetryCapacity::available(),
    );

    assert_eq!(
        decision.stop_reason(),
        Some(RetryStopReason::DeadlineExpired)
    );
}

#[test]
fn retry_delay_must_fit_inside_remaining_deadline() {
    let policy = retry_policy(5);
    let error = transport_error(DeliveryState::NotSent);

    let decision = evaluate_retry(
        &policy,
        &error,
        PreviousAttemptState::NotSubmitted,
        RetrySafety::Idempotent,
        RetrySemanticState::Valid,
        0,
        Some(Duration::from_millis(5)),
        false,
        RetryCapacity::available(),
    );

    assert_eq!(
        decision.stop_reason(),
        Some(RetryStopReason::InsufficientDeadline)
    );
}

#[test]
fn configured_attempt_budget_is_respected() {
    let policy = retry_policy(1);
    let error = transport_error(DeliveryState::NotSent);

    let decision = evaluate_retry(
        &policy,
        &error,
        PreviousAttemptState::NotSubmitted,
        RetrySafety::Never,
        RetrySemanticState::Valid,
        0,
        None,
        false,
        RetryCapacity::available(),
    );

    assert_eq!(
        decision.stop_reason(),
        Some(RetryStopReason::PolicyExhausted)
    );
}

#[test]
fn retry_capacity_is_checked_before_another_attempt() {
    let policy = retry_policy(5);
    let error = transport_error(DeliveryState::NotSent);

    let decision = evaluate_retry(
        &policy,
        &error,
        PreviousAttemptState::NotSubmitted,
        RetrySafety::Idempotent,
        RetrySemanticState::Valid,
        0,
        None,
        false,
        RetryCapacity {
            execution_budget_available: true,
            resources_feasible: false,
            capabilities_available: true,
        },
    );

    assert_eq!(
        decision.stop_reason(),
        Some(RetryStopReason::ResourcesUnavailable)
    );
}

#[test]
fn missing_execution_budget_prevents_retry() {
    let policy = retry_policy(5);
    let error = transport_error(DeliveryState::NotSent);

    let decision = evaluate_retry(
        &policy,
        &error,
        PreviousAttemptState::NotSubmitted,
        RetrySafety::Idempotent,
        RetrySemanticState::Valid,
        0,
        None,
        false,
        RetryCapacity {
            execution_budget_available: false,
            resources_feasible: true,
            capabilities_available: true,
        },
    );

    assert_eq!(
        decision.stop_reason(),
        Some(RetryStopReason::ExecutionBudgetUnavailable)
    );
}

#[test]
fn missing_capability_prevents_retry() {
    let policy = retry_policy(5);
    let error = transport_error(DeliveryState::NotSent);

    let decision = evaluate_retry(
        &policy,
        &error,
        PreviousAttemptState::NotSubmitted,
        RetrySafety::Idempotent,
        RetrySemanticState::Valid,
        0,
        None,
        false,
        RetryCapacity {
            execution_budget_available: true,
            resources_feasible: true,
            capabilities_available: false,
        },
    );

    assert_eq!(
        decision.stop_reason(),
        Some(RetryStopReason::CapabilitiesUnavailable)
    );
}

#[test]
fn semantic_invalidity_prevents_retry() {
    let policy = retry_policy(5);
    let error = transport_error(DeliveryState::NotSent);

    let decision = evaluate_retry(
        &policy,
        &error,
        PreviousAttemptState::NotSubmitted,
        RetrySafety::Idempotent,
        RetrySemanticState::Invalid,
        0,
        None,
        false,
        RetryCapacity::available(),
    );

    assert_eq!(
        decision.stop_reason(),
        Some(RetryStopReason::SemanticInvalid)
    );
}

#[test]
fn changed_semantic_assumptions_require_revalidation() {
    let policy = retry_policy(5);
    let error = transport_error(DeliveryState::NotSent);

    let decision = evaluate_retry(
        &policy,
        &error,
        PreviousAttemptState::NotSubmitted,
        RetrySafety::Idempotent,
        RetrySemanticState::RequiresRevalidation,
        0,
        None,
        false,
        RetryCapacity::available(),
    );

    assert_eq!(
        decision.reconciliation(),
        Some(RetryReconciliation::RevalidateSemantics)
    );
}

#[test]
fn retry_decisions_are_deterministic_for_identical_evidence() {
    let policy = retry_policy(5);
    let error = transport_error(DeliveryState::NotSent);

    let first = evaluate_retry(
        &policy,
        &error,
        PreviousAttemptState::NotSubmitted,
        RetrySafety::Idempotent,
        RetrySemanticState::Valid,
        1,
        Some(Duration::from_secs(2)),
        false,
        RetryCapacity::available(),
    );

    let second = evaluate_retry(
        &policy,
        &error,
        PreviousAttemptState::NotSubmitted,
        RetrySafety::Idempotent,
        RetrySemanticState::Valid,
        1,
        Some(Duration::from_secs(2)),
        false,
        RetryCapacity::available(),
    );

    assert_eq!(first, second);
}

// ============================================================================
// CHECKPOINT TEST PROVIDERS
// ============================================================================

#[derive(Debug)]
struct TestPayload {
    bytes: Vec<u8>,
}

impl CheckpointPayload for TestPayload {
    fn write_to(
        &self,
        writer: &mut dyn Write,
    ) -> Result<(), CheckpointProviderError> {
        writer.write_all(&self.bytes)?;
        Ok(())
    }
}

#[derive(Debug)]
struct StoredCheckpoint {
    manifest: CheckpointManifest,
    payload: Vec<u8>,
}

/// Deterministic in-memory test double.
///
/// This is intentionally not a production storage implementation.
/// Its purpose is to test the manager's atomic-create, manifest,
/// integrity-verification, and restore-target contracts.
#[derive(Debug)]
struct MemoryCheckpointStore {
    capabilities: CheckpointStoreCapabilities,
    records: Mutex<HashMap<CheckpointId, StoredCheckpoint>>,
    stream_chunk_size: usize,
}

impl MemoryCheckpointStore {
    fn new() -> Self {
        Self {
            capabilities: CheckpointStoreCapabilities {
                atomic_create: true,
                durable_commit: true,
                verified_restore: true,
            },
            records: Mutex::new(HashMap::new()),
            stream_chunk_size: 4093,
        }
    }

    fn with_capabilities(
        capabilities: CheckpointStoreCapabilities,
    ) -> Self {
        Self {
            capabilities,
            records: Mutex::new(HashMap::new()),
            stream_chunk_size: 4093,
        }
    }

    fn corrupt_payload(
        &self,
        checkpoint_id: CheckpointId,
    ) {
        let mut records = self
            .records
            .lock()
            .expect("test checkpoint store lock must not be poisoned");

        let stored = records
            .get_mut(&checkpoint_id)
            .expect("checkpoint must exist before corruption");

        if let Some(first) = stored.payload.first_mut() {
            *first ^= 0xff;
        } else {
            stored.payload.push(0xff);
        }
    }
}

fn provider_error(message: &'static str) -> CheckpointProviderError {
    Box::new(io::Error::other(message))
}

impl CheckpointStore for MemoryCheckpointStore {
    fn capabilities(&self) -> CheckpointStoreCapabilities {
        self.capabilities
    }

    fn create_atomic(
        &self,
        manifest: &CheckpointManifest,
        payload: &dyn CheckpointPayload,
    ) -> Result<CheckpointCreateOutcome, CheckpointProviderError> {
        let mut records = self
            .records
            .lock()
            .map_err(|_| provider_error("checkpoint store lock poisoned"))?;

        if records.contains_key(&manifest.checkpoint_id()) {
            return Ok(CheckpointCreateOutcome::AlreadyExists);
        }

        let mut bytes = Vec::new();
        payload.write_to(&mut bytes)?;

        let actual_digest: [u8; 32] = Sha256::digest(&bytes).into();

        if &actual_digest != manifest.payload_digest().as_bytes() {
            return Err(provider_error("payload digest does not match manifest"));
        }

        if let Some(expected_length) = manifest.payload_length() {
            if expected_length != bytes.len() as u128 {
                return Err(provider_error("payload length does not match manifest"));
            }
        }

        records.insert(
            manifest.checkpoint_id(),
            StoredCheckpoint {
                manifest: manifest.clone(),
                payload: bytes,
            },
        );

        Ok(CheckpointCreateOutcome::Created)
    }

    fn load_manifest(
        &self,
        checkpoint_id: CheckpointId,
    ) -> Result<Option<CheckpointManifest>, CheckpointProviderError> {
        let records = self
            .records
            .lock()
            .map_err(|_| provider_error("checkpoint store lock poisoned"))?;

        Ok(records
            .get(&checkpoint_id)
            .map(|stored| stored.manifest.clone()))
    }

    fn stream_payload(
        &self,
        checkpoint_id: CheckpointId,
        writer: &mut dyn Write,
    ) -> Result<(), CheckpointProviderError> {
        let bytes = {
            let records = self
                .records
                .lock()
                .map_err(|_| provider_error("checkpoint store lock poisoned"))?;

            records
                .get(&checkpoint_id)
                .map(|stored| stored.payload.clone())
                .ok_or_else(|| provider_error("checkpoint payload not found"))?
        };

        for chunk in bytes.chunks(self.stream_chunk_size) {
            writer.write_all(chunk)?;
        }

        Ok(())
    }

    fn delete(
        &self,
        checkpoint_id: CheckpointId,
    ) -> Result<bool, CheckpointProviderError> {
        let mut records = self
            .records
            .lock()
            .map_err(|_| provider_error("checkpoint store lock poisoned"))?;

        Ok(records.remove(&checkpoint_id).is_some())
    }
}

#[derive(Debug, Default)]
struct MemoryRestoreTarget {
    staged: Vec<u8>,
    committed: Option<Vec<u8>>,
    aborted: bool,
}

impl CheckpointRestoreTarget for MemoryRestoreTarget {
    fn write_chunk(
        &mut self,
        bytes: &[u8],
    ) -> Result<(), CheckpointProviderError> {
        self.staged.extend_from_slice(bytes);
        Ok(())
    }

    fn commit(&mut self) -> Result<(), CheckpointProviderError> {
        self.committed = Some(std::mem::take(&mut self.staged));
        Ok(())
    }

    fn abort(&mut self) -> Result<(), CheckpointProviderError> {
        self.staged.clear();
        self.aborted = true;
        Ok(())
    }
}

fn checkpoint_manifest(
    checkpoint_sequence: u128,
    bytes: &[u8],
) -> CheckpointManifest {
    let digest: [u8; 32] = Sha256::digest(bytes).into();

    let checkpoint_id = CheckpointId::new(0xCAFE, checkpoint_sequence)
        .expect("checkpoint identity must be valid");

    let task_id = TaskId::new(0xCAFE, 1)
        .expect("task identity must be valid");

    let execution_context_id = ExecutionContextId::new(0xCAFE, 1)
        .expect("execution context identity must be valid");

    let safety = CheckpointSafetyEvidence::new(
        "classical",
        "task_boundary",
        "test-provider",
        true,
        true,
        true,
    )
    .expect("test checkpoint safety metadata must be valid");

    CheckpointManifest::new(
        checkpoint_id,
        task_id,
        execution_context_id,
        &safety,
        "test.payload",
        1,
        Vec::new(),
        CheckpointDigest::sha256(digest),
        Some(bytes.len() as u128),
    )
    .expect("test checkpoint manifest must be valid")
}

// ============================================================================
// CHECKPOINT CONTRACT TESTS
// ============================================================================

#[test]
fn checkpoint_manager_rejects_storage_without_atomic_creation() {
    let store = MemoryCheckpointStore::with_capabilities(
        CheckpointStoreCapabilities {
            atomic_create: false,
            durable_commit: true,
            verified_restore: true,
        },
    );

    let result = CheckpointManager::new(store);

    assert!(matches!(
        result,
        Err(CheckpointError::InsufficientStoreGuarantees {
            capability: "atomic_create"
        })
    ));
}

#[test]
fn checkpoint_manager_rejects_storage_without_durable_commit() {
    let store = MemoryCheckpointStore::with_capabilities(
        CheckpointStoreCapabilities {
            atomic_create: true,
            durable_commit: false,
            verified_restore: true,
        },
    );

    let result = CheckpointManager::new(store);

    assert!(matches!(
        result,
        Err(CheckpointError::InsufficientStoreGuarantees {
            capability: "durable_commit"
        })
    ));
}

#[test]
fn checkpoint_manager_rejects_storage_without_verified_restore() {
    let store = MemoryCheckpointStore::with_capabilities(
        CheckpointStoreCapabilities {
            atomic_create: true,
            durable_commit: true,
            verified_restore: false,
        },
    );

    let result = CheckpointManager::new(store);

    assert!(matches!(
        result,
        Err(CheckpointError::InsufficientStoreGuarantees {
            capability: "verified_restore"
        })
    ));
}

#[test]
fn checkpoint_create_and_restore_preserve_payload_and_manifest() {
    let bytes = b"portable distributed checkpoint payload".to_vec();
    let manifest = checkpoint_manifest(1, &bytes);
    let payload = TestPayload {
        bytes: bytes.clone(),
    };

    let manager = CheckpointManager::new(MemoryCheckpointStore::new())
        .expect("test store must satisfy checkpoint guarantees");

    let created = manager
        .create(&manifest, &payload)
        .expect("checkpoint creation must succeed");

    assert_eq!(created, CheckpointCreateOutcome::Created);

    let mut target = MemoryRestoreTarget::default();

    let restored = manager
        .restore(manifest.checkpoint_id(), &mut target)
        .expect("valid checkpoint must restore");

    assert_eq!(restored, manifest);
    assert_eq!(target.committed.as_deref(), Some(bytes.as_slice()));
    assert!(!target.aborted);
}

#[test]
fn checkpoint_creation_is_idempotent_for_the_same_manifest() {
    let bytes = b"same logical checkpoint".to_vec();
    let manifest = checkpoint_manifest(2, &bytes);
    let payload = TestPayload {
        bytes: bytes.clone(),
    };

    let manager = CheckpointManager::new(MemoryCheckpointStore::new())
        .expect("test store must satisfy checkpoint guarantees");

    assert_eq!(
        manager.create(&manifest, &payload).unwrap(),
        CheckpointCreateOutcome::Created
    );

    assert_eq!(
        manager.create(&manifest, &payload).unwrap(),
        CheckpointCreateOutcome::AlreadyExists
    );
}

#[test]
fn checkpoint_identity_cannot_be_reused_for_different_metadata() {
    let original_bytes = b"original checkpoint".to_vec();
    let replacement_bytes = b"different checkpoint".to_vec();

    let original = checkpoint_manifest(3, &original_bytes);
    let replacement = checkpoint_manifest(3, &replacement_bytes);

    let manager = CheckpointManager::new(MemoryCheckpointStore::new())
        .expect("test store must satisfy checkpoint guarantees");

    manager
        .create(
            &original,
            &TestPayload {
                bytes: original_bytes,
            },
        )
        .expect("initial checkpoint creation must succeed");

    let result = manager.create(
        &replacement,
        &TestPayload {
            bytes: replacement_bytes,
        },
    );

    assert!(matches!(
        result,
        Err(CheckpointError::IdentityConflict { .. })
    ));
}

#[test]
fn restore_rejects_corrupted_payload_and_aborts_staging() {
    let bytes = b"integrity protected checkpoint".to_vec();
    let manifest = checkpoint_manifest(4, &bytes);

    let manager = CheckpointManager::new(MemoryCheckpointStore::new())
        .expect("test store must satisfy checkpoint guarantees");

    manager
        .create(
            &manifest,
            &TestPayload {
                bytes: bytes.clone(),
            },
        )
        .expect("checkpoint creation must succeed");

    manager
        .store()
        .corrupt_payload(manifest.checkpoint_id());

    let mut target = MemoryRestoreTarget::default();

    let result = manager.restore(manifest.checkpoint_id(), &mut target);

    assert!(matches!(
        result,
        Err(CheckpointError::DigestMismatch)
    ));

    assert!(target.committed.is_none());
    assert!(target.staged.is_empty());
    assert!(target.aborted);
}

#[test]
fn checkpoint_metadata_rejects_uncheckpointable_domain_state() {
    let evidence = CheckpointSafetyEvidence::new(
        "quantum",
        "mid_circuit",
        "quantum-provider",
        false,
        true,
        true,
    )
    .expect("the evidence metadata itself is valid");

    let digest = CheckpointDigest::sha256([0; 32]);

    let result = CheckpointManifest::new(
        CheckpointId::new(0xCAFE, 5).unwrap(),
        TaskId::new(0xCAFE, 1).unwrap(),
        ExecutionContextId::new(0xCAFE, 1).unwrap(),
        &evidence,
        "quantum.provider.snapshot",
        1,
        Vec::new(),
        digest,
        None,
    );

    assert!(matches!(
        result,
        Err(CheckpointError::NotCheckpointable { .. })
    ));
}

// ============================================================================
// RECOVERY TEST JOURNAL
// ============================================================================

/// In-memory journal used only to test recovery-engine state transitions.
///
/// It implements revision checking and atomic replacement within this
/// process. It is not a durable production journal.
#[derive(Debug, Default)]
struct MemoryRecoveryJournal {
    records: Mutex<HashMap<OperationId, RecoveryRecord>>,
}

impl RecoveryJournal for MemoryRecoveryJournal {
    fn load(
        &self,
        operation_id: OperationId,
    ) -> DistributedResult<Option<RecoveryRecord>> {
        let records = self.records.lock().map_err(|_| {
            DistributedError::InvalidInput {
                message: "test recovery journal lock poisoned".to_owned(),
            }
        })?;

        Ok(records.get(&operation_id).cloned())
    }

    fn compare_and_swap(
        &self,
        operation_id: OperationId,
        expected_revision: u64,
        record: RecoveryRecord,
    ) -> DistributedResult<JournalWrite> {
        let mut records = self.records.lock().map_err(|_| {
            DistributedError::InvalidInput {
                message: "test recovery journal lock poisoned".to_owned(),
            }
        })?;

        let next_revision = expected_revision.checked_add(1);

        match records.get(&operation_id) {
            None if expected_revision == 0 && record.revision() == 0 => {
                records.insert(operation_id, record);
                Ok(JournalWrite::Committed)
            }

            Some(current)
                if current.revision() == expected_revision
                    && next_revision == Some(record.revision()) =>
            {
                records.insert(operation_id, record);
                Ok(JournalWrite::Committed)
            }

            _ => Ok(JournalWrite::RevisionConflict),
        }
    }
}

// ============================================================================
// RECOVERY TEST EXECUTOR
// ============================================================================

#[derive(Debug)]
struct TestRecoveryExecutor {
    fail_first_submission: AtomicBool,
    fail_verification: AtomicBool,
    submitted_action_ids: Mutex<Vec<RecoveryActionId>>,
    reconciled_action_ids: Mutex<Vec<RecoveryActionId>>,
    verified: AtomicBool,
}

impl TestRecoveryExecutor {
    fn new(
        fail_first_submission: bool,
        fail_verification: bool,
    ) -> Self {
        Self {
            fail_first_submission: AtomicBool::new(fail_first_submission),
            fail_verification: AtomicBool::new(fail_verification),
            submitted_action_ids: Mutex::new(Vec::new()),
            reconciled_action_ids: Mutex::new(Vec::new()),
            verified: AtomicBool::new(false),
        }
    }
}

impl RecoveryExecutor for TestRecoveryExecutor {
    fn validate_plan(
        &self,
        _incident: &RecoveryIncident,
        _plan: &RecoveryPlan,
    ) -> DistributedResult<()> {
        Ok(())
    }

    fn reconcile_action(
        &self,
        action_id: RecoveryActionId,
        _action: &RecoveryAction,
    ) -> DistributedResult<RecoveryActionStatus> {
        self.reconciled_action_ids
            .lock()
            .map_err(|_| DistributedError::InvalidInput {
                message: "test executor reconciliation lock poisoned".to_owned(),
            })?
            .push(action_id);

        // The test provider establishes that the interrupted action
        // completed before its response was lost.
        Ok(RecoveryActionStatus::Completed)
    }

    fn execute_action(
        &self,
        action_id: RecoveryActionId,
        _action: &RecoveryAction,
    ) -> DistributedResult<RecoveryActionStatus> {
        self.submitted_action_ids
            .lock()
            .map_err(|_| DistributedError::InvalidInput {
                message: "test executor submission lock poisoned".to_owned(),
            })?
            .push(action_id);

        if self
            .fail_first_submission
            .swap(false, Ordering::SeqCst)
        {
            return Err(DistributedError::TransportFailed {
                operation: "recovery-action".to_owned(),
                delivery: DeliveryState::MayHaveExecuted,
                message: "simulated process interruption after submission".to_owned(),
            });
        }

        Ok(RecoveryActionStatus::Completed)
    }

    fn verify_recovered_state(
        &self,
        _incident: &RecoveryIncident,
        _plan: &RecoveryPlan,
    ) -> DistributedResult<()> {
        if self.fail_verification.load(Ordering::SeqCst) {
            return Err(DistributedError::RecoveryFailed {
                subject: "test-verification".to_owned(),
                message: "simulated recovered-state verification failure".to_owned(),
            });
        }

        self.verified.store(true, Ordering::SeqCst);
        Ok(())
    }
}

// ============================================================================
// RECOVERY TEST HELPERS
// ============================================================================

fn recovery_operation_id() -> OperationId {
    OperationId::new(0xD157, 1)
        .expect("test operation identity must be valid")
}

fn recovery_incident(operation_id: OperationId) -> RecoveryIncident {
    RecoveryIncident::new(
        operation_id,
        RecoveryIncidentKind::CommunicationFailure,
        "test communication interruption",
        0,
        17,
    )
}

fn recovery_plan(
    operation_id: OperationId,
    expected_revision: u64,
) -> RecoveryPlan {
    RecoveryPlan::new(
        operation_id,
        PlanId::new(0xD157, 1)
            .expect("test plan identity must be valid"),
        1,
        expected_revision,
        17,
        23,
        vec![
            RecoveryAction::new(
                RecoveryActionKind::Resume,
                None,
                "resume from an authoritative recoverable boundary",
            ),
        ],
    )
    .expect("test recovery plan must be valid")
}

fn register_recovery(
    journal: &MemoryRecoveryJournal,
    engine: &RecoveryEngine,
    operation_id: OperationId,
) -> RecoveryRecord {
    engine
        .register_incident(
            journal,
            recovery_incident(operation_id),
        )
        .expect("recovery incident registration must succeed")
}

// ============================================================================
// RECOVERY CONTRACT TESTS
// ============================================================================

#[test]
fn recovery_requires_a_validated_plan_before_execution() {
    let journal = MemoryRecoveryJournal::default();
    let engine = RecoveryEngine::new();
    let executor = TestRecoveryExecutor::new(false, false);
    let operation_id = recovery_operation_id();

    register_recovery(&journal, &engine, operation_id);

    let result = engine.execute_recovery(
        &journal,
        &executor,
        operation_id,
    );

    assert!(result.is_err());
    assert!(executor.submitted_action_ids.lock().unwrap().is_empty());
}

#[test]
fn recovery_rejects_a_stale_journal_revision() {
    let journal = MemoryRecoveryJournal::default();
    let engine = RecoveryEngine::new();
    let executor = TestRecoveryExecutor::new(false, false);
    let operation_id = recovery_operation_id();

    register_recovery(&journal, &engine, operation_id);

    let stale_plan = recovery_plan(operation_id, 99);

    let result = engine.begin_recovery(
        &journal,
        &executor,
        stale_plan,
    );

    assert!(matches!(
        result,
        Err(DistributedError::Conflict { .. })
    ));

    let record = journal
        .load(operation_id)
        .unwrap()
        .expect("incident must remain recorded");

    assert_eq!(record.status(), RecoveryStatus::Incident);
}

#[test]
fn recovery_rejects_an_empty_action_plan() {
    let operation_id = recovery_operation_id();

    let result = RecoveryPlan::new(
        operation_id,
        PlanId::new(0xD157, 2).unwrap(),
        1,
        0,
        17,
        23,
        Vec::new(),
    );

    assert!(matches!(
        result,
        Err(DistributedError::InvalidInput { .. })
    ));
}

#[test]
fn recovery_rejects_duplicate_incident_registration() {
    let journal = MemoryRecoveryJournal::default();
    let engine = RecoveryEngine::new();
    let operation_id = recovery_operation_id();

    register_recovery(&journal, &engine, operation_id);

    let duplicate = engine.register_incident(
        &journal,
        recovery_incident(operation_id),
    );

    assert!(matches!(
        duplicate,
        Err(DistributedError::Conflict { .. })
    ));
}

#[test]
fn recovery_reconciles_interrupted_action_before_resubmission() {
    let journal = MemoryRecoveryJournal::default();
    let engine = RecoveryEngine::new();
    let executor = TestRecoveryExecutor::new(true, false);
    let operation_id = recovery_operation_id();

    let incident_record =
        register_recovery(&journal, &engine, operation_id);

    let plan = recovery_plan(
        operation_id,
        incident_record.revision(),
    );

    let planned = engine
        .begin_recovery(&journal, &executor, plan)
        .expect("recovery plan validation must succeed");

    assert_eq!(planned.status(), RecoveryStatus::PlanValidated);

    // The first submission returns an error after it may have reached
    // the execution boundary. The journal must retain the Started state.
    let first_attempt = engine.execute_recovery(
        &journal,
        &executor,
        operation_id,
    );

    assert!(first_attempt.is_err());

    let interrupted = journal
        .load(operation_id)
        .unwrap()
        .expect("interrupted recovery record must remain durable in the test journal");

    assert_eq!(interrupted.status(), RecoveryStatus::Recovering);
    assert_eq!(
        interrupted.action_statuses(),
        &[RecoveryActionStatus::Started]
    );

    // The next execution must consult the provider's authoritative
    // reconciliation result before it can treat the action as complete.
    let resumed = engine
        .execute_recovery(&journal, &executor, operation_id)
        .expect("recovery must reconcile the interrupted action");

    assert_eq!(resumed.status(), RecoveryStatus::Verifying);
    assert_eq!(
        resumed.action_statuses(),
        &[RecoveryActionStatus::Completed]
    );

    let submitted = executor
        .submitted_action_ids
        .lock()
        .unwrap()
        .clone();

    let reconciled = executor
        .reconciled_action_ids
        .lock()
        .unwrap()
        .clone();

    assert_eq!(submitted.len(), 1);
    assert_eq!(reconciled.len(), 1);

    // The action identity is stable across the interrupted operation
    // and its reconciliation.
    assert_eq!(submitted[0], reconciled[0]);

    let verified = engine
        .verify_recovery(&journal, &executor, operation_id)
        .expect("recovered state verification must succeed");

    assert_eq!(verified.status(), RecoveryStatus::Succeeded);
    assert!(executor.verified.load(Ordering::SeqCst));
}

#[test]
fn recovery_does_not_mark_state_successful_when_verification_fails() {
    let journal = MemoryRecoveryJournal::default();
    let engine = RecoveryEngine::new();
    let executor = TestRecoveryExecutor::new(false, true);
    let operation_id = recovery_operation_id();

    let incident_record =
        register_recovery(&journal, &engine, operation_id);

    let plan = recovery_plan(
        operation_id,
        incident_record.revision(),
    );

    engine
        .begin_recovery(&journal, &executor, plan)
        .expect("recovery plan validation must succeed");

    let executing = engine
        .execute_recovery(&journal, &executor, operation_id)
        .expect("recovery action must complete");

    assert_eq!(executing.status(), RecoveryStatus::Verifying);

    let verification = engine.verify_recovery(
        &journal,
        &executor,
        operation_id,
    );

    assert!(matches!(
        verification,
        Err(DistributedError::RecoveryFailed { .. })
    ));

    let record = journal
        .load(operation_id)
        .unwrap()
        .expect("recovery record must remain available");

    assert_eq!(record.status(), RecoveryStatus::Verifying);
    assert!(!executor.verified.load(Ordering::SeqCst));
}

#[test]
fn recovery_journal_rejects_a_stale_compare_and_swap() {
    let journal = MemoryRecoveryJournal::default();
    let engine = RecoveryEngine::new();
    let operation_id = recovery_operation_id();

    let record = register_recovery(&journal, &engine, operation_id);

    let result = journal
        .compare_and_swap(
            operation_id,
            1,
            record,
        )
        .expect("test journal operation must complete");

    assert_eq!(result, JournalWrite::RevisionConflict);
}

#[test]
fn recovery_does_not_claim_success_when_an_action_outcome_is_unknown() {
    // The production engine requires the execution provider to establish
    // an action outcome. An unresolved outcome must never be interpreted
    // as a completed recovery action.
    //
    // This test is intentionally expressed through the recovery lifecycle:
    // the provider's reconciliation result is the authority, not the
    // caller's assumption that a previous submission probably succeeded.

    let journal = MemoryRecoveryJournal::default();
    let engine = RecoveryEngine::new();
    let executor = TestRecoveryExecutor::new(true, false);
    let operation_id = recovery_operation_id();

    let incident_record =
        register_recovery(&journal, &engine, operation_id);

    engine
        .begin_recovery(
            &journal,
            &executor,
            recovery_plan(operation_id, incident_record.revision()),
        )
        .expect("recovery plan validation must succeed");

    assert!(
        engine
            .execute_recovery(&journal, &executor, operation_id)
            .is_err()
    );

    let interrupted = journal
        .load(operation_id)
        .unwrap()
        .expect("recovery record must remain available");

    assert_eq!(interrupted.status(), RecoveryStatus::Recovering);
    assert_eq!(
        interrupted.action_statuses(),
        &[RecoveryActionStatus::Started]
    );

    // No verification or success transition has occurred.
    assert!(!executor.verified.load(Ordering::SeqCst));
}

// ============================================================================
// CROSS-CUTTING PORTABILITY / REPRESENTATION TESTS
// ============================================================================

#[test]
fn checkpoint_payload_size_is_not_a_language_level_limit() {
    // This is a test fixture size, not a supported maximum.
    // The production API streams payload data and leaves operational
    // storage limits to the selected provider and its configured budgets.
    let bytes = vec![0x5a; 1024 * 1024];
    let manifest = checkpoint_manifest(99, &bytes);

    let manager = CheckpointManager::new(MemoryCheckpointStore::new())
        .expect("test store must satisfy checkpoint guarantees");

    manager
        .create(
            &manifest,
            &TestPayload {
                bytes: bytes.clone(),
            },
        )
        .expect("test checkpoint must be created");

    let mut target = MemoryRestoreTarget::default();

    manager
        .restore(manifest.checkpoint_id(), &mut target)
        .expect("test checkpoint must restore");

    assert_eq!(target.committed.as_deref(), Some(bytes.as_slice()));
}

