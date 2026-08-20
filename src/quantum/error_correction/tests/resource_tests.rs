//! Resource-safety and resource-policy integration tests for Zamani QEC.
//!
//! These tests verify that the resource architecture is actually enforced at
//! runtime rather than merely declared by individual modules.
//!
//! Architectural contract:
//!
//! ```text
//!                       QecConfig
//!                           |
//!                           v
//!                       QecLimits
//!                           |
//!              +------------+------------+
//!              |                         |
//!              v                         v
//!          Preflight               ResourceManager
//!              |                         |
//!              |              +----------+----------+
//!              |              |          |          |
//!              v              v          v          v
//!          Allocation      Memory     Counters   Workers
//!              |              |          |          |
//!              +--------------+----------+----------+
//!                             |
//!                             v
//!                      ResourceSnapshot
//! ```
//!
//! The tests intentionally exercise the canonical resource infrastructure:
//!
//! - `QecLimits` is the declarative policy.
//! - `ResourceManager` is runtime accounting.
//! - `ResourceRequest` is allocation-free preflight.
//! - `ResourceQuota` can only tighten global policy.
//! - `ResourceScope` provides operation-local enforcement.
//! - memory reservations are RAII based.
//! - worker reservations are RAII based.
//! - counters are bounded.
//! - cancellation is observable.
//! - arithmetic overflow is rejected.
//! - failed reservations do not leak resources.
//!
//! These tests must remain:
//!
//! - hardware independent;
//! - network independent;
//! - deterministic;
//! - free of physical QPU access;
//! - free of process-global mutable state;
//! - safe to run in parallel with other tests.
//!
//! They should test resource contracts rather than timing-sensitive
//! implementation details.

#![allow(clippy::assertions_on_constants)]

use std::thread;
use std::time::Duration;

use crate::quantum::error_correction::{
    limits::{
        LimitKind,
        QecLimits,
    },
    resources::{
        ResourceError,
        ResourceKind,
        ResourceManager,
        ResourceQuota,
        ResourceRequest,
    },
};

// ============================================================================
// Test helpers
// ============================================================================

fn test_limits() -> QecLimits {
    let mut limits = QecLimits::default();

    // Keep tests deliberately small while remaining large enough to exercise
    // normal successful operations.
    limits.max_code_distance = 16;
    limits.max_qubits = 256;
    limits.max_stabilizers = 255;
    limits.max_syndrome_events = 1_024;
    limits.max_rounds = 128;
    limits.max_graph_nodes = 1_024;
    limits.max_graph_edges = 4_096;
    limits.max_memory_bytes = 1 << 20;
    limits.max_decoder_time_ns = 60_000_000_000;
    limits.max_parallelism = 8;
    limits.max_checkpoint_size_bytes = 1 << 18;
    limits.max_partitions = 16;
    limits.max_stream_buffer_events = 512;
    limits.max_decoder_iterations = 10_000;
    limits.max_stabilizer_weight = 16;
    limits.max_logical_operator_weight = 256;
    limits.max_qubits_per_partition = 128;
    limits.max_qpu_shots = 10_000;
    limits.max_qpu_circuits = 128;
    limits.max_verification_operations = 10_000;

    limits
}

fn manager() -> ResourceManager {
    ResourceManager::new(test_limits())
        .expect("test resource policy must be valid")
}

fn assert_limit_exceeded(
    result: Result<(), ResourceError>,
    expected: ResourceKind,
) {
    match result {
        Err(ResourceError::LimitExceeded { resource, .. }) => {
            assert_eq!(resource, expected);
        }

        other => panic!(
            "expected a resource limit error for {expected}, got {other:?}"
        ),
    }
}

// ============================================================================
// QecLimits validation
// ============================================================================

#[test]
fn canonical_qec_limits_validate() {
    let limits = test_limits();

    assert!(
        limits.validate().is_ok(),
        "canonical test resource policy must validate"
    );
}

#[test]
fn zero_memory_limit_is_rejected() {
    let mut limits = test_limits();
    limits.max_memory_bytes = 0;

    assert!(
        limits.validate().is_err(),
        "zero memory must never be accepted as a production limit"
    );
}

#[test]
fn zero_parallelism_limit_is_rejected() {
    let mut limits = test_limits();
    limits.max_parallelism = 0;

    assert!(
        limits.validate().is_err(),
        "zero parallelism must never be accepted"
    );
}

#[test]
fn zero_graph_node_limit_is_rejected() {
    let mut limits = test_limits();
    limits.max_graph_nodes = 0;

    assert!(
        limits.validate().is_err(),
        "zero graph-node limit must be rejected"
    );
}

#[test]
fn zero_verification_operation_limit_is_rejected() {
    let mut limits = test_limits();
    limits.max_verification_operations = 0;

    assert!(
        limits.validate().is_err(),
        "zero verification-operation limit must be rejected"
    );
}

// ============================================================================
// ResourceManager construction
// ============================================================================

#[test]
fn resource_manager_uses_canonical_qec_limits() {
    let limits = test_limits();
    let manager = ResourceManager::new(limits)
        .expect("manager must accept valid QecLimits");

    assert_eq!(
        manager.limits(),
        limits,
        "ResourceManager must retain the exact canonical policy"
    );
}

#[test]
fn resource_manager_starts_with_zero_runtime_usage() {
    let manager = manager();
    let snapshot = manager.snapshot();

    assert_eq!(snapshot.allocated_bytes, 0);
    assert_eq!(snapshot.peak_bytes, 0);
    assert_eq!(snapshot.syndrome_events, 0);
    assert_eq!(snapshot.graph_nodes, 0);
    assert_eq!(snapshot.graph_edges, 0);
    assert_eq!(snapshot.decoder_iterations, 0);
    assert_eq!(snapshot.parallel_workers, 0);
    assert_eq!(snapshot.qubits, 0);
    assert_eq!(snapshot.stabilizers, 0);
    assert_eq!(snapshot.measurement_rounds, 0);
    assert_eq!(snapshot.checkpoint_bytes, 0);
    assert_eq!(snapshot.partitions, 0);
    assert_eq!(snapshot.stream_buffer_events, 0);
    assert_eq!(snapshot.qpu_shots, 0);
    assert_eq!(snapshot.qpu_circuits, 0);
    assert_eq!(snapshot.verification_operations, 0);
}

// ============================================================================
// Allocation-free preflight
// ============================================================================

#[test]
fn surface_code_request_is_checked_before_allocation() {
    let manager = manager();

    let request = ResourceRequest::surface_code(5, 4)
        .expect("valid surface-code request");

    manager
        .preflight(&request)
        .expect("distance-5 request must fit test limits");

    let snapshot = manager.snapshot();

    assert_eq!(
        snapshot.allocated_bytes,
        0,
        "preflight must not allocate memory"
    );

    assert_eq!(
        snapshot.qubits,
        0,
        "preflight must not mutate runtime counters"
    );
}

#[test]
fn graph_request_is_checked_before_allocation() {
    let manager = manager();

    let request = ResourceRequest::graph(64, 128)
        .expect("valid graph request");

    manager
        .preflight(&request)
        .expect("graph request must fit test limits");

    assert_eq!(
        manager.snapshot().allocated_bytes,
        0,
        "graph preflight must not allocate"
    );
}

#[test]
fn oversized_surface_code_request_is_rejected_before_allocation() {
    let manager = manager();

    let request = ResourceRequest::surface_code(17, 4)
        .expect("request arithmetic should remain representable");

    let result = manager.preflight(&request);

    assert_limit_exceeded(
        result,
        ResourceKind::CodeDistance,
    );

    let snapshot = manager.snapshot();

    assert_eq!(
        snapshot.allocated_bytes,
        0,
        "failed preflight must not allocate memory"
    );

    assert_eq!(
        snapshot.qubits,
        0,
        "failed preflight must not mutate qubit accounting"
    );
}

#[test]
fn oversized_graph_request_is_rejected_before_allocation() {
    let manager = manager();

    let request = ResourceRequest::graph(2_000, 2_000)
        .expect("request arithmetic should remain representable");

    let result = manager.preflight(&request);

    assert!(
        result.is_err(),
        "graph exceeding canonical limits must be rejected"
    );

    assert_eq!(
        manager.snapshot().allocated_bytes,
        0,
        "failed graph preflight must not allocate"
    );
}

// ============================================================================
// Memory reservations
// ============================================================================

#[test]
fn memory_reservation_updates_runtime_accounting() {
    let manager = manager();

    {
        let reservation = manager
            .reserve_memory(4_096)
            .expect("4 KiB must fit");

        assert_eq!(reservation.bytes(), 4_096);

        let snapshot = manager.snapshot();

        assert_eq!(
            snapshot.allocated_bytes,
            4_096
        );

        assert_eq!(
            snapshot.peak_bytes,
            4_096
        );
    }

    assert_eq!(
        manager.snapshot().allocated_bytes,
        0,
        "RAII memory reservation must release memory on drop"
    );
}

#[test]
fn memory_peak_is_retained_after_release() {
    let manager = manager();

    {
        let _reservation = manager
            .reserve_memory(8_192)
            .expect("reservation must fit");
    }

    let snapshot = manager.snapshot();

    assert_eq!(
        snapshot.allocated_bytes,
        0
    );

    assert!(
        snapshot.peak_bytes >= 8_192,
        "peak memory must retain the highest observed allocation"
    );
}

#[test]
fn memory_limit_is_enforced() {
    let manager = manager();

    let limit = manager.limits().max_memory_bytes;

    let result = manager.reserve_memory(
        limit + 1
    );

    match result {
        Err(ResourceError::LimitExceeded {
            resource,
            requested,
            limit: reported_limit,
            ..
        }) => {
            assert_eq!(
                resource,
                ResourceKind::MemoryBytes
            );

            assert_eq!(
                requested,
                limit + 1
            );

            assert_eq!(
                reported_limit,
                limit
            );
        }

        other => panic!(
            "expected memory limit error, got {other:?}"
        ),
    }

    assert_eq!(
        manager.snapshot().allocated_bytes,
        0,
        "failed memory reservation must not leak memory"
    );
}

#[test]
fn memory_reservation_release_is_idempotent() {
    let manager = manager();

    let reservation = manager
        .reserve_memory(2_048)
        .expect("reservation must fit");

    assert!(
        reservation.release(),
        "first release must succeed"
    );

    assert_eq!(
        manager.snapshot().allocated_bytes,
        0
    );
}

// ============================================================================
// Worker reservations
// ============================================================================

#[test]
fn worker_reservation_updates_runtime_accounting() {
    let manager = manager();

    {
        let reservation = manager
            .acquire_workers(3)
            .expect("three workers must fit");

        assert_eq!(
            reservation.workers(),
            3
        );

        assert_eq!(
            manager.snapshot().parallel_workers,
            3
        );
    }

    assert_eq!(
        manager.snapshot().parallel_workers,
        0,
        "RAII worker reservation must release workers on drop"
    );
}

#[test]
fn worker_limit_is_enforced() {
    let manager = manager();

    let limit = manager.limits().max_parallelism;

    let result = manager.acquire_workers(
        limit + 1
    );

    match result {
        Err(ResourceError::ParallelismLimitExceeded {
            requested,
            current,
            limit: reported_limit,
        }) => {
            assert_eq!(
                requested,
                limit + 1
            );

            assert_eq!(
                current,
                0
            );

            assert_eq!(
                reported_limit,
                limit
            );
        }

        other => panic!(
            "expected parallelism limit error, got {other:?}"
        ),
    }

    assert_eq!(
        manager.snapshot().parallel_workers,
        0,
        "failed worker acquisition must not leak workers"
    );
}

#[test]
fn zero_worker_acquisition_is_a_noop() {
    let manager = manager();

    let reservation = manager
        .acquire_workers(0)
        .expect("zero workers should be a no-op");

    assert_eq!(
        reservation.workers(),
        0
    );

    assert_eq!(
        manager.snapshot().parallel_workers,
        0
    );
}

#[test]
fn worker_reservation_release_is_idempotent() {
    let manager = manager();

    let reservation = manager
        .acquire_workers(2)
        .expect("worker reservation must fit");

    assert!(
        reservation.release(),
        "first release must succeed"
    );

    assert_eq!(
        manager.snapshot().parallel_workers,
        0
    );
}

// ============================================================================
// Bounded runtime counters
// ============================================================================

#[test]
fn syndrome_event_counter_is_bounded() {
    let manager = manager();

    manager
        .record_syndrome_events(100)
        .expect("100 events fit");

    assert_eq!(
        manager.snapshot().syndrome_events,
        100
    );

    let remaining =
        manager.limits().max_syndrome_events as u64 - 100;

    manager
        .record_syndrome_events(remaining)
        .expect("remaining event capacity must fit");

    assert_eq!(
        manager.snapshot().syndrome_events,
        manager.limits().max_syndrome_events as u64
    );

    let result =
        manager.record_syndrome_events(1);

    assert_limit_exceeded(
        result,
        ResourceKind::SyndromeEvents,
    );
}

#[test]
fn graph_nodes_are_bounded() {
    let manager = manager();

    manager
        .record_graph_nodes(512)
        .expect("512 graph nodes fit");

    assert_eq!(
        manager.snapshot().graph_nodes,
        512
    );

    let result = manager.record_graph_nodes(
        manager.limits().max_graph_nodes as u64
    );

    assert!(
        result.is_err(),
        "counter must account for already consumed graph nodes"
    );
}

#[test]
fn graph_edges_are_bounded() {
    let manager = manager();

    manager
        .record_graph_edges(512)
        .expect("512 graph edges fit");

    assert_eq!(
        manager.snapshot().graph_edges,
        512
    );
}

#[test]
fn decoder_iterations_are_bounded() {
    let manager = manager();

    manager
        .record_decoder_iterations(100)
        .expect("100 iterations fit");

    assert_eq!(
        manager.snapshot().decoder_iterations,
        100
    );
}

#[test]
fn qpu_shots_are_bounded_without_accessing_a_qpu() {
    let manager = manager();

    manager
        .record_qpu_shots(100)
        .expect("100 simulated/accounted shots fit");

    assert_eq!(
        manager.snapshot().qpu_shots,
        100
    );

    let result = manager.record_qpu_shots(
        manager.limits().max_qpu_shots
    );

    assert!(
        result.is_err(),
        "the second request must account for already consumed shots"
    );
}

#[test]
fn qpu_circuits_are_bounded_without_network_access() {
    let manager = manager();

    manager
        .record_qpu_circuits(4)
        .expect("four circuits fit");

    assert_eq!(
        manager.snapshot().qpu_circuits,
        4
    );
}

#[test]
fn verification_operations_have_a_dedicated_limit() {
    let manager = manager();

    manager
        .record_verification_operations(100)
        .expect("100 verification operations fit");

    assert_eq!(
        manager.snapshot().verification_operations,
        100
    );

    let result =
        manager.record_verification_operations(
            manager.limits().max_verification_operations
        );

    assert!(
        result.is_err(),
        "verification operations must not bypass their dedicated limit"
    );
}

// ============================================================================
// Runtime resource dimensions
// ============================================================================

#[test]
fn code_distance_is_checked_before_being_recorded() {
    let manager = manager();

    manager
        .record_code_distance(8)
        .expect("distance 8 fits");

    assert_eq!(
        manager.snapshot().code_distance,
        8
    );

    let result = manager.record_code_distance(
        manager.limits().max_code_distance + 1
    );

    assert_limit_exceeded(
        result,
        ResourceKind::CodeDistance,
    );

    assert_eq!(
        manager.snapshot().code_distance,
        8,
        "failed distance recording must not overwrite valid state"
    );
}

#[test]
fn qubit_count_is_bounded() {
    let manager = manager();

    manager
        .record_qubits(64)
        .expect("64 qubits fit");

    assert_eq!(
        manager.snapshot().qubits,
        64
    );

    let result = manager.record_qubits(
        manager.limits().max_qubits
    );

    assert!(
        result.is_err(),
        "recording must account for already consumed qubits"
    );
}

#[test]
fn stabilizer_count_is_bounded() {
    let manager = manager();

    manager
        .record_stabilizers(32)
        .expect("32 stabilizers fit");

    assert_eq!(
        manager.snapshot().stabilizers,
        32
    );
}

#[test]
fn measurement_rounds_are_bounded() {
    let manager = manager();

    manager
        .record_measurement_rounds(16)
        .expect("16 rounds fit");

    assert_eq!(
        manager.snapshot().measurement_rounds,
        16
    );
}

#[test]
fn checkpoint_bytes_are_bounded() {
    let manager = manager();

    manager
        .record_checkpoint_bytes(4_096)
        .expect("checkpoint fits");

    assert_eq!(
        manager.snapshot().checkpoint_bytes,
        4_096
    );
}

#[test]
fn partition_count_is_bounded() {
    let manager = manager();

    manager
        .record_partitions(4)
        .expect("four partitions fit");

    assert_eq!(
        manager.snapshot().partitions,
        4
    );
}

#[test]
fn stream_buffer_is_bounded() {
    let manager = manager();

    manager
        .record_stream_buffer_events(128)
        .expect("128 buffered events fit");

    assert_eq!(
        manager.snapshot().stream_buffer_events,
        128
    );
}

// ============================================================================
// ResourceQuota
// ============================================================================

#[test]
fn_empty_resource_quota_is_valid() {
    let quota = ResourceQuota::default();

    assert!(
        quota.validate().is_ok(),
        "default quota must impose no additional restriction"
    );
}

#[test]
fn_zero_operation_quota_is_rejected() {
    let quota = ResourceQuota {
        max_memory_bytes: Some(0),
        ..ResourceQuota::default()
    };

    assert!(
        quota.validate().is_err(),
        "zero operation quota must be rejected"
    );
}

#[test]
fn_operation_quota_can_only_tighten_memory() {
    let manager = manager();

    let quota = ResourceQuota {
        max_memory_bytes: Some(4_096),
        ..ResourceQuota::default()
    };

    let scope = manager
        .scope("memory-quota-test", quota)
        .expect("quota must be valid");

    let reservation = scope
        .reserve_memory(4_096)
        .expect("request equal to quota must fit");

    assert_eq!(
        reservation.bytes(),
        4_096
    );

    drop(reservation);

    let result = scope.reserve_memory(4_097);

    match result {
        Err(ResourceError::QuotaExceeded {
            resource,
            requested,
            limit,
            ..
        }) => {
            assert_eq!(
                resource,
                ResourceKind::MemoryBytes
            );

            assert_eq!(
                requested,
                4_097
            );

            assert_eq!(
                limit,
                4_096
            );
        }

        other => panic!(
            "expected quota error, got {other:?}"
        ),
    }

    assert_eq!(
        manager.snapshot().allocated_bytes,
        0,
        "failed quota reservation must not leak memory"
    );
}

#[test]
fn_operation_quota_cannot_expand_global_memory_limit() {
    let manager = manager();

    let global_limit =
        manager.limits().max_memory_bytes;

    let quota = ResourceQuota {
        max_memory_bytes: Some(
            global_limit.saturating_add(1)
        ),
        ..ResourceQuota::default()
    };

    let scope = manager
        .scope("global-bound-test", quota)
        .expect("quota itself may be valid");

    let result =
        scope.reserve_memory(global_limit + 1);

    match result {
        Err(ResourceError::LimitExceeded {
            resource,
            ..
        }) => {
            assert_eq!(
                resource,
                ResourceKind::MemoryBytes
            );
        }

        other => panic!(
            "operation quota must never bypass global policy: {other:?}"
        ),
    }
}

#[test]
fn operation_scope_applies_worker_quota() {
    let manager = manager();

    let quota = ResourceQuota {
        max_parallelism: Some(2),
        ..ResourceQuota::default()
    };

    let scope = manager
        .scope("worker-quota-test", quota)
        .expect("quota must be valid");

    let _workers = scope
        .acquire_workers(2)
        .expect("two workers fit quota");

    let result = scope.acquire_workers(1);

    match result {
        Err(ResourceError::ParallelismQuotaExceeded {
            limit,
            ..
        }) => {
            assert_eq!(
                limit,
                2
            );
        }

        other => panic!(
            "expected worker quota error, got {other:?}"
        ),
    }
}

// ============================================================================
// ResourceScope preflight
// ============================================================================

#[test]
fn resource_scope_preflight_enforces_global_policy() {
    let manager = manager();

    let quota = ResourceQuota::default();

    let scope = manager
        .scope("preflight-test", quota)
        .expect("scope must be valid");

    let request = ResourceRequest::graph(
        32,
        64,
    )
    .expect("graph request must be representable");

    scope
        .preflight(&request)
        .expect("request must fit global limits");

    assert_eq!(
        manager.snapshot().allocated_bytes,
        0,
        "scope preflight must remain allocation free"
    );
}

#[test]
fn resource_scope_preflight_enforces_operation_quota() {
    let manager = manager();

    let quota = ResourceQuota {
        max_graph_nodes: Some(8),
        ..ResourceQuota::default()
    };

    let scope = manager
        .scope("graph-quota-test", quota)
        .expect("scope must be valid");

    let request = ResourceRequest::graph(
        16,
        16,
    )
    .expect("graph request must be representable");

    let result = scope.preflight(&request);

    match result {
        Err(ResourceError::QuotaExceeded {
            resource,
            requested,
            limit,
            ..
        }) => {
            assert_eq!(
                resource,
                ResourceKind::GraphNodes
            );

            assert_eq!(
                requested,
                16
            );

            assert_eq!(
                limit,
                8
            );
        }

        other => panic!(
            "expected graph quota error, got {other:?}"
        ),
    }

    assert_eq!(
        manager.snapshot().allocated_bytes,
        0,
        "failed scoped preflight must not allocate"
    );
}

// ============================================================================
// Cancellation
// ============================================================================

#[test]
fn cancellation_is_observable_by_resource_manager() {
    let manager = manager();

    assert!(
        !manager.is_cancelled(),
        "new manager must not be cancelled"
    );

    manager.cancel();

    assert!(
        manager.is_cancelled(),
        "cancel must become observable"
    );

    match manager.check() {
        Err(ResourceError::Cancelled) => {}

        other => panic!(
            "expected cancellation error, got {other:?}"
        ),
    }
}

#[test]
fn cancellation_prevents_new_memory_reservations() {
    let manager = manager();

    manager.cancel();

    let result = manager.reserve_memory(1_024);

    assert!(
        matches!(result, Err(ResourceError::Cancelled)),
        "cancelled execution must not acquire new memory"
    );

    assert_eq!(
        manager.snapshot().allocated_bytes,
        0
    );
}

#[test]
fn cancellation_prevents_new_worker_acquisition() {
    let manager = manager();

    manager.cancel();

    let result = manager.acquire_workers(1);

    assert!(
        matches!(result, Err(ResourceError::Cancelled)),
        "cancelled execution must not acquire workers"
    );

    assert_eq!(
        manager.snapshot().parallel_workers,
        0
    );
}

#[test]
fn cancellation_prevents_new_counter_updates() {
    let manager = manager();

    manager.cancel();

    let result =
        manager.record_syndrome_events(1);

    assert!(
        matches!(result, Err(ResourceError::Cancelled)),
        "cancelled execution must not continue consuming resources"
    );

    assert_eq!(
        manager.snapshot().syndrome_events,
        0
    );
}

#[test]
fn cancellation_can_be_reset_between_logical_operations() {
    let manager = manager();

    manager.cancel();

    assert!(
        manager.check().is_err()
    );

    manager.reset_cancellation();

    assert!(
        manager.check().is_ok(),
        "reset must allow a new logical operation"
    );
}

// ============================================================================
// Arithmetic / overflow safety
// ============================================================================

#[test]
fn absurd_surface_code_distance_is_rejected_without_allocation() {
    let manager = manager();

    let result =
        ResourceRequest::surface_code(
            usize::MAX,
            4,
        );

    /*
     * The request constructor itself must fail safely if the derived
     * distance^2 calculation cannot be represented.
     */
    assert!(
        result.is_err(),
        "surface-code preflight must reject overflowing dimensions"
    );

    assert_eq!(
        manager.snapshot().allocated_bytes,
        0
    );
}

#[test]
fn absurd_graph_dimensions_are_rejected_without_allocation() {
    let manager = manager();

    let result =
        ResourceRequest::graph(
            usize::MAX,
            usize::MAX,
        );

    assert!(
        result.is_err(),
        "graph memory estimation must reject arithmetic overflow"
    );

    assert_eq!(
        manager.snapshot().allocated_bytes,
        0
    );
}

// ============================================================================
// Runtime accounting consistency
// ============================================================================

#[test]
fn failed_memory_reservation_does_not_change_peak_or_current_usage() {
    let manager = manager();

    let before = manager.snapshot();

    let result = manager.reserve_memory(
        manager.limits().max_memory_bytes + 1
    );

    assert!(result.is_err());

    let after = manager.snapshot();

    assert_eq!(
        after.allocated_bytes,
        before.allocated_bytes
    );

    assert_eq!(
        after.peak_bytes,
        before.peak_bytes
    );
}

#[test]
fn failed_worker_acquisition_does_not_change_worker_count() {
    let manager = manager();

    let before =
        manager.snapshot().parallel_workers;

    let result = manager.acquire_workers(
        manager.limits().max_parallelism + 1
    );

    assert!(result.is_err());

    assert_eq!(
        manager.snapshot().parallel_workers,
        before
    );
}

#[test]
fn resource_snapshot_reports_compute_time_without_requiring_wall_clock_assertions() {
    let manager = manager();

    manager
        .record_compute_time(
            Duration::from_nanos(1)
        )
        .expect("small compute duration must fit");

    let snapshot = manager.snapshot();

    assert!(
        snapshot.compute_time >= Duration::from_nanos(1),
        "recorded compute time must be represented in the snapshot"
    );
}

// ============================================================================
// Shared manager / concurrent accounting
// ============================================================================

#[test]
fn shared_resource_manager_supports_concurrent_accounting() {
    let manager = ResourceManager::shared(test_limits())
        .expect("shared manager must be constructible");

    let mut handles = Vec::new();

    for _ in 0..4 {
        let manager = manager.clone();

        handles.push(thread::spawn(move || {
            manager
                .record_syndrome_events(10)
                .expect("10 events must fit");

            let reservation = manager
                .reserve_memory(1_024)
                .expect("1 KiB must fit");

            /*
             * Keep the reservation alive until the worker finishes its
             * accounting operation.
             */
            assert_eq!(
                reservation.bytes(),
                1_024
            );
        }));
    }

    for handle in handles {
        handle
            .join()
            .expect("resource worker must not panic");
    }

    let snapshot = manager.snapshot();

    assert_eq!(
        snapshot.syndrome_events,
        40,
        "concurrent event accounting must not lose updates"
    );

    assert_eq!(
        snapshot.allocated_bytes,
        0,
        "all worker memory reservations must be released"
    );

    assert!(
        snapshot.peak_bytes >= 1_024,
        "peak memory must account for concurrent reservations"
    );
}

// ============================================================================
// Resource policy is single-source-of-truth
// ============================================================================

#[test]
fn resource_manager_does_not_use_a_second_independent_policy() {
    let mut limits = test_limits();

    limits.max_memory_bytes = 32_768;
    limits.max_parallelism = 3;
    limits.max_syndrome_events = 77;

    let manager = ResourceManager::new(limits)
        .expect("custom policy must validate");

    assert_eq!(
        manager.limits().max_memory_bytes,
        32_768
    );

    assert_eq!(
        manager.limits().max_parallelism,
        3
    );

    assert_eq!(
        manager.limits().max_syndrome_events,
        77
    );

    /*
     * Verify actual runtime behavior follows those exact values.
     */
    let memory =
        manager.reserve_memory(32_768)
            .expect("exact memory limit must fit");

    assert!(
        manager.reserve_memory(1).is_err(),
        "runtime memory accounting must use the canonical QecLimits value"
    );

    drop(memory);

    let workers =
        manager.acquire_workers(3)
            .expect("exact worker limit must fit");

    assert!(
        manager.acquire_workers(1).is_err(),
        "runtime worker accounting must use canonical QecLimits"
    );

    drop(workers);

    manager
        .record_syndrome_events(77)
        .expect("exact event limit must fit");

    assert!(
        manager.record_syndrome_events(1).is_err(),
        "runtime syndrome accounting must use canonical QecLimits"
    );
}

// ============================================================================
// Resource dimensions map to explicit policy kinds
// ============================================================================

#[test]
fn canonical_limit_kinds_have_stable_identifiers() {
    assert_eq!(
        LimitKind::CodeDistance.as_str(),
        "code_distance"
    );

    assert_eq!(
        LimitKind::Qubits.as_str(),
        "qubits"
    );

    assert_eq!(
        LimitKind::MemoryBytes.as_str(),
        "memory_bytes"
    );

    assert_eq!(
        LimitKind::GraphNodes.as_str(),
        "graph_nodes"
    );

    assert_eq!(
        LimitKind::GraphEdges.as_str(),
        "graph_edges"
    );

    assert_eq!(
        LimitKind::QpuShots.as_str(),
        "qpu_shots"
    );

    assert_eq!(
        LimitKind::VerificationOperations.as_str(),
        "verification_operations"
    );
}

// ============================================================================
// Final resource invariant
// ============================================================================

#[test]
fn resource_manager_never_reports_negative_like_state() {
    let manager = manager();

    {
        let _memory = manager
            .reserve_memory(4_096)
            .expect("memory must fit");

        let _workers = manager
            .acquire_workers(2)
            .expect("workers must fit");

        manager
            .record_syndrome_events(32)
            .expect("events must fit");
    }

    let snapshot = manager.snapshot();

    /*
     * Unsigned counters plus RAII release must leave the manager in a valid
     * zero-allocation state after the operation.
     */
    assert_eq!(
        snapshot.allocated_bytes,
        0
    );

    assert_eq!(
        snapshot.parallel_workers,
        0
    );

    assert_eq!(
        snapshot.syndrome_events,
        32
    );

    assert!(
        snapshot.peak_bytes >= 4_096
    );
}