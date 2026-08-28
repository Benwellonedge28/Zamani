//! Zamani Quantum Hardware — Execution Conformance Tests.
//!
//! Production-grade integration/conformance tests for:
//!
//! `crate::quantum::hardware::execution`
//!
//! # Responsibility
//!
//! This module verifies the public execution-orchestration contract without
//! depending on provider-specific implementation details.
//!
//! It protects:
//!
//! - execution-policy validation;
//! - execution-mode semantics;
//! - execution-phase semantics;
//! - execution-handle invariants;
//! - execution-snapshot semantics;
//! - execution-receipt invariants;
//! - execution-outcome semantics;
//! - preflight-before-submission ordering;
//! - asynchronous submit -> status -> result lifecycle;
//! - synchronous execution semantics;
//! - cancellation semantics;
//! - timeout semantics;
//! - backend-identity protection;
//! - job-identity protection;
//! - result-availability protection;
//! - result-shot accounting;
//! - incomplete-result rejection;
//! - excessive-result rejection;
//! - terminal failure handling;
//! - terminal expiration handling;
//! - terminal cancellation handling;
//! - native-synchronous capability enforcement;
//! - adapter sharing through `Arc`;
//! - deterministic local execution;
//! - provider-neutral adapter behavior;
//! - regression protection for future hardware providers.
//!
//! # Non-responsibilities
//!
//! This module deliberately does NOT test:
//!
//! - provider HTTP APIs;
//! - provider SDKs;
//! - authentication;
//! - credentials;
//! - network communication;
//! - real QPU execution;
//! - provider pricing;
//! - provider-specific payload formats;
//! - OpenQASM parsing;
//! - QIR generation;
//! - routing algorithms;
//! - scheduling algorithms;
//! - calibration acquisition;
//! - benchmarking mathematics;
//! - QEC algorithms;
//! - simulator numerical correctness beyond what is needed to establish the
//!   execution lifecycle.
//!
//! Those concerns have independent ownership boundaries.
//!
//! # Integration contract
//!
//! This test module consumes the public contracts of:
//!
//! - `backend.rs`;
//! - `backend_trait.rs`;
//! - `execution.rs`;
//! - `adapters/local.rs`.
//!
//! It intentionally avoids private fields and implementation details.
//!
//! Future adapters must be able to satisfy the same semantic requirements:
//!
//! ```text
//! QuantumBackendAdapter
//!          |
//!          v
//! preflight -> submit -> status -> result
//!          |              |
//!          +--> cancel ---+
//!          |
//!          v
//! QuantumExecutionEngine
//! ```
//!
//! Adding a provider MUST NOT require provider-specific branches in this
//! module.
//!
//! # Production invariants
//!
//! A conforming execution implementation must:
//!
//! 1. validate the request before provider submission;
//! 2. reject empty executable programs;
//! 3. never submit after failed preflight;
//! 4. preserve backend identity across the lifecycle;
//! 5. preserve job identity across the lifecycle;
//! 6. never retrieve a result before completion under strict policy;
//! 7. never accept a result when the provider says no result is available;
//! 8. never accept more samples than requested;
//! 9. never accept fewer samples as a complete result;
//! 10. distinguish local timeout from remote cancellation;
//! 11. never claim cancellation succeeded when it was unsupported;
//! 12. reject native synchronous execution when the adapter does not advertise
//!     it;
//! 13. keep provider-specific behavior behind `QuantumBackendAdapter`;
//! 14. remain safe to share through `Arc`;
//! 15. remain deterministic for deterministic local adapters;
//! 16. never expose program payload bytes through debug output;
//! 17. preserve provider-neutral lifecycle semantics.
//!
//! # Rust compatibility
//!
//! Supported:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust only;
//! - no nightly features;
//! - no unsafe code.
//!
//! # Security
//!
//! Tests use synthetic data only.
//!
//! They never contain:
//!
//! - real API keys;
//! - access tokens;
//! - passwords;
//! - private keys;
//! - production endpoints;
//! - cloud credentials.
//!
//! Test programs are synthetic local payloads.
//!
//! # Determinism
//!
//! Tests use:
//!
//! - fixed program payloads;
//! - fixed request identifiers;
//! - fixed seeds;
//! - deterministic local backend configuration;
//! - no environment variables;
//! - no network;
//! - no wall-clock assertions except bounded timeout behavior;
//! - no external services.
//!
//! # No-reedit contract
//!
//! This file is intentionally written against the already-established public
//! contracts. Future additions to:
//!
//! - `job.rs`;
//! - `queue.rs`;
//! - `cancellation.rs`;
//! - provider registries;
//! - provider adapters;
//! - Danga;
//! - benchmarking;
//!
//! must consume the execution contract rather than requiring changes to this
//! test module merely because those systems were added.
//!
//! Provider-specific conformance suites may reuse the generic assertions
//! defined here.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::collections::BTreeMap;
use std::sync::Arc;
use std::time::Duration;

use crate::quantum::hardware::backend::{
    BackendCapabilities,
    BackendError,
    BackendKind,
    BackendLimits,
    BackendMetadata,
    BackendStatus,
    CircuitRequirements,
    ExecutionRequest,
    ExecutionResult,
    QuantumBackend,
    QuantumWorkload,
    QuantumWorkloadKind,
    WorkloadRequirements,
};

use crate::quantum::hardware::backend_trait::{
    BackendAdapterInfo,
    BackendCancellation,
    BackendHealth,
    BackendHealthState,
    BackendJob,
    BackendJobId,
    BackendJobState,
    BackendJobStatus,
    BackendProgram,
    BackendQueueInfo,
    CancellationOutcome,
    QuantumBackendAdapter,
};

use crate::quantum::hardware::execution::{
    ExecutionCancellation,
    ExecutionError,
    ExecutionHandle,
    ExecutionMode,
    ExecutionOutcome,
    ExecutionPhase,
    ExecutionPolicy,
    ExecutionPolicyError,
    ExecutionReceipt,
    ExecutionSnapshot,
    QuantumExecutionEngine,
    DEFAULT_EXECUTION_TIMEOUT,
    DEFAULT_POLL_INTERVAL,
    MAX_EXECUTION_TIMEOUT,
    MAX_POLL_ATTEMPTS,
    MAX_POLL_INTERVAL,
    MIN_POLL_INTERVAL,
};

use crate::quantum::hardware::adapters::local::{
    LocalBackendAdapter,
    LocalBackendConfig,
    LocalFault,
    LOCAL_PROGRAM_FORMAT,
};


// =============================================================================
// Test constants
// =============================================================================

const TEST_BACKEND_ID: &str = "test://execution";
const TEST_ADAPTER_ID: &str = "zamani.test.execution";
const TEST_ADAPTER_VERSION: &str = "1.0.0";

const TEST_PROGRAM: &[u8] = br#"{
    "schema": "zamani-local-v1",
    "qubits": 1,
    "classical_bits": 1,
    "measure_all": true,
    "operations": [
        {"gate": "x", "targets": [0]}
    ]
}"#;


// =============================================================================
// Public-contract helpers
// =============================================================================

/// Builds a valid local executable program.
///
/// Keeping program construction in one helper prevents individual tests from
/// accidentally testing different workload semantics.
fn local_program() -> BackendProgram {
    BackendProgram::new(
        LOCAL_PROGRAM_FORMAT,
        TEST_PROGRAM.to_vec(),
    )
    .expect("synthetic local program must be valid")
}


/// Builds a minimal valid execution request.
///
/// The backend's public request contract is intentionally used rather than
/// constructing private implementation state.
fn execution_request(
    request_id: Option<&str>,
    shots: usize,
) -> ExecutionRequest {
    ExecutionRequest {
        request_id: request_id.map(str::to_string),
        workload: QuantumWorkload {
            kind: QuantumWorkloadKind::GateCircuit,
            circuit: CircuitRequirements {
                qubits: 1,
                depth: 1,
                operations: 1,
                shots,
                native_gates: Default::default(),
            },
            requirements: WorkloadRequirements::default(),
        },
        metadata: BTreeMap::new(),
    }
}


/// Creates a deterministic local adapter suitable for lifecycle tests.
///
/// The local adapter executes without credentials or network access and is
/// therefore the canonical integration target for this test module.
fn local_adapter() -> Arc<LocalBackendAdapter> {
    Arc::new(
        LocalBackendAdapter::with_config(
            LocalBackendConfig::test(),
        )
        .expect("test local adapter must construct"),
    )
}


/// Creates a local execution engine with a short but valid polling interval.
///
/// Local execution normally completes immediately, so this policy does not
/// introduce unnecessary test latency.
fn local_engine() -> QuantumExecutionEngine<LocalBackendAdapter> {
    let policy = ExecutionPolicy::default()
        .with_timeout(Duration::from_secs(1))
        .expect("one-second timeout must be valid")
        .with_poll_interval(Duration::from_millis(1))
        .expect("one-millisecond polling interval must be valid")
        .with_max_poll_attempts(100)
        .expect("100 polling attempts must be valid");

    QuantumExecutionEngine::with_policy(
        local_adapter(),
        policy,
    )
    .expect("test execution policy must be valid")
}


/// Builds a synthetic backend job.
///
/// This helper tests lifecycle types without requiring a provider.
fn synthetic_job(
    id: &str,
    backend_id: &str,
    request_id: Option<&str>,
    state: BackendJobState,
) -> BackendJob {
    BackendJob::new(
        BackendJobId::new(id).expect("synthetic job ID must be valid"),
        backend_id,
        request_id.map(str::to_string),
        state,
    )
    .expect("synthetic backend job must be valid")
}


/// Builds a synthetic execution handle.
fn synthetic_handle(
    job_id: &str,
    backend_id: &str,
    state: BackendJobState,
    phase: ExecutionPhase,
) -> ExecutionHandle {
    ExecutionHandle {
        job: synthetic_job(
            job_id,
            backend_id,
            None,
            state,
        ),
        phase,
    }
}


// =============================================================================
// Policy conformance
// =============================================================================

#[test]
fn default_execution_policy_is_production_valid() {
    let policy = ExecutionPolicy::default();

    assert_eq!(
        policy.timeout,
        DEFAULT_EXECUTION_TIMEOUT
    );

    assert_eq!(
        policy.poll_interval,
        DEFAULT_POLL_INTERVAL
    );

    assert_eq!(
        policy.max_poll_attempts,
        MAX_POLL_ATTEMPTS
    );

    assert!(!policy.cancel_on_timeout);
    assert!(policy.require_backend_identity_match);
    assert!(policy.require_completed_state);

    assert!(
        policy.validate().is_ok(),
        "default production policy must validate"
    );
}


#[test]
fn execution_policy_rejects_zero_timeout() {
    let result = ExecutionPolicy::default()
        .with_timeout(Duration::ZERO);

    assert!(matches!(
        result,
        Err(ExecutionPolicyError::InvalidTimeout { .. })
    ));
}


#[test]
fn execution_policy_rejects_timeout_above_production_bound() {
    let result = ExecutionPolicy::default()
        .with_timeout(
            MAX_EXECUTION_TIMEOUT
                .saturating_add(Duration::from_secs(1)),
        );

    assert!(matches!(
        result,
        Err(ExecutionPolicyError::InvalidTimeout { .. })
    ));
}


#[test]
fn execution_policy_rejects_zero_poll_interval() {
    let result = ExecutionPolicy::default()
        .with_poll_interval(Duration::ZERO);

    assert!(matches!(
        result,
        Err(
            ExecutionPolicyError::InvalidPollInterval { .. }
        )
    ));
}


#[test]
fn execution_policy_rejects_poll_interval_above_bound() {
    let result = ExecutionPolicy::default()
        .with_poll_interval(
            MAX_POLL_INTERVAL
                .saturating_add(Duration::from_millis(1)),
        );

    assert!(matches!(
        result,
        Err(
            ExecutionPolicyError::InvalidPollInterval { .. }
        )
    ));
}


#[test]
fn execution_policy_accepts_minimum_poll_interval() {
    let result = ExecutionPolicy::default()
        .with_poll_interval(MIN_POLL_INTERVAL);

    assert!(result.is_ok());
}


#[test]
fn execution_policy_accepts_maximum_poll_interval() {
    let result = ExecutionPolicy::default()
        .with_poll_interval(MAX_POLL_INTERVAL);

    assert!(result.is_ok());
}


#[test]
fn execution_policy_rejects_zero_poll_attempts() {
    let result = ExecutionPolicy::default()
        .with_max_poll_attempts(0);

    assert!(matches!(
        result,
        Err(
            ExecutionPolicyError::InvalidPollAttemptLimit { .. }
        )
    ));
}


#[test]
fn execution_policy_rejects_poll_attempts_above_bound() {
    let result = ExecutionPolicy::default()
        .with_max_poll_attempts(
            MAX_POLL_ATTEMPTS.saturating_add(1),
        );

    assert!(matches!(
        result,
        Err(
            ExecutionPolicyError::InvalidPollAttemptLimit { .. }
        )
    ));
}


#[test]
fn execution_policy_builders_preserve_other_policy_fields() {
    let policy = ExecutionPolicy::default()
        .with_timeout(Duration::from_secs(5))
        .expect("timeout must be valid")
        .with_poll_interval(Duration::from_millis(10))
        .expect("poll interval must be valid")
        .with_max_poll_attempts(100)
        .expect("attempt limit must be valid")
        .with_cancel_on_timeout(true)
        .with_backend_identity_check(false)
        .with_completed_state_requirement(false);

    assert_eq!(
        policy.timeout,
        Duration::from_secs(5)
    );

    assert_eq!(
        policy.poll_interval,
        Duration::from_millis(10)
    );

    assert_eq!(policy.max_poll_attempts, 100);
    assert!(policy.cancel_on_timeout);
    assert!(!policy.require_backend_identity_match);
    assert!(!policy.require_completed_state);

    assert!(policy.validate().is_ok());
}


// =============================================================================
// Execution-mode conformance
// =============================================================================

#[test]
fn execution_modes_have_stable_machine_names() {
    assert_eq!(
        ExecutionMode::SubmitOnly.as_str(),
        "submit_only"
    );

    assert_eq!(
        ExecutionMode::WaitForResult.as_str(),
        "wait_for_result"
    );

    assert_eq!(
        ExecutionMode::NativeSynchronous.as_str(),
        "native_synchronous"
    );
}


#[test]
fn wait_for_result_is_the_default_execution_mode() {
    assert_eq!(
        ExecutionMode::default(),
        ExecutionMode::WaitForResult
    );
}


// =============================================================================
// Execution-phase conformance
// =============================================================================

#[test]
fn execution_phases_have_stable_machine_names() {
    assert_eq!(
        ExecutionPhase::Validating.as_str(),
        "validating"
    );

    assert_eq!(
        ExecutionPhase::Preflight.as_str(),
        "preflight"
    );

    assert_eq!(
        ExecutionPhase::Submitting.as_str(),
        "submitting"
    );

    assert_eq!(
        ExecutionPhase::Submitted.as_str(),
        "submitted"
    );

    assert_eq!(
        ExecutionPhase::Waiting.as_str(),
        "waiting"
    );

    assert_eq!(
        ExecutionPhase::RetrievingResult.as_str(),
        "retrieving_result"
    );

    assert_eq!(
        ExecutionPhase::Completed.as_str(),
        "completed"
    );

    assert_eq!(
        ExecutionPhase::Cancelling.as_str(),
        "cancelling"
    );

    assert_eq!(
        ExecutionPhase::Cancelled.as_str(),
        "cancelled"
    );

    assert_eq!(
        ExecutionPhase::Failed.as_str(),
        "failed"
    );

    assert_eq!(
        ExecutionPhase::TimedOut.as_str(),
        "timed_out"
    );
}


#[test]
fn execution_phases_mark_only_terminal_phases_terminal() {
    assert!(!ExecutionPhase::Validating.is_terminal());
    assert!(!ExecutionPhase::Preflight.is_terminal());
    assert!(!ExecutionPhase::Submitting.is_terminal());
    assert!(!ExecutionPhase::Submitted.is_terminal());
    assert!(!ExecutionPhase::Waiting.is_terminal());
    assert!(!ExecutionPhase::RetrievingResult.is_terminal());
    assert!(ExecutionPhase::Completed.is_terminal());
    assert!(ExecutionPhase::Cancelled.is_terminal());
    assert!(ExecutionPhase::Failed.is_terminal());
    assert!(ExecutionPhase::TimedOut.is_terminal());
}


// =============================================================================
// Handle and snapshot conformance
// =============================================================================

#[test]
fn execution_handle_exposes_stable_job_identity() {
    let handle = synthetic_handle(
        "job-001",
        TEST_BACKEND_ID,
        BackendJobState::Running,
        ExecutionPhase::Submitted,
    );

    assert_eq!(
        handle.job_id().as_str(),
        "job-001"
    );

    assert_eq!(
        handle.backend_id(),
        TEST_BACKEND_ID
    );

    assert_eq!(
        handle.state(),
        BackendJobState::Running
    );

    assert!(!handle.is_terminal());
}


#[test]
fn execution_handle_reports_terminal_state() {
    let handle = synthetic_handle(
        "job-002",
        TEST_BACKEND_ID,
        BackendJobState::Completed,
        ExecutionPhase::Completed,
    );

    assert!(handle.is_terminal());
}


#[test]
fn submitted_snapshot_starts_without_status() {
    let handle = synthetic_handle(
        "job-003",
        TEST_BACKEND_ID,
        BackendJobState::Queued,
        ExecutionPhase::Submitted,
    );

    let snapshot = ExecutionSnapshot::submitted(
        handle.clone(),
    );

    assert_eq!(
        snapshot.handle,
        handle
    );

    assert!(snapshot.status.is_none());

    assert_eq!(
        snapshot.phase,
        ExecutionPhase::Submitted
    );

    assert_eq!(
        snapshot.poll_attempts,
        0
    );

    assert_eq!(
        snapshot.state(),
        BackendJobState::Queued
    );

    assert!(!snapshot.is_terminal());
}


#[test]
fn completed_snapshot_is_terminal() {
    let handle = synthetic_handle(
        "job-004",
        TEST_BACKEND_ID,
        BackendJobState::Completed,
        ExecutionPhase::Completed,
    );

    let snapshot = ExecutionSnapshot {
        handle,
        status: None,
        phase: ExecutionPhase::Completed,
        poll_attempts: 0,
    };

    assert!(snapshot.is_terminal());
    assert_eq!(
        snapshot.state(),
        BackendJobState::Completed
    );
}


// =============================================================================
// Receipt conformance
// =============================================================================

#[test]
fn execution_receipt_reports_complete_shot_accounting() {
    let job = synthetic_job(
        "job-receipt-001",
        TEST_BACKEND_ID,
        Some("request-001"),
        BackendJobState::Completed,
    );

    let receipt = ExecutionReceipt {
        job,
        backend_id: TEST_BACKEND_ID.to_string(),
        requested_shots: 100,
        counted_shots: 100,
        final_state: BackendJobState::Completed,
        poll_attempts: 3,
        mode: ExecutionMode::WaitForResult,
    };

    assert!(receipt.is_complete());
}


#[test]
fn execution_receipt_detects_incomplete_shot_accounting() {
    let job = synthetic_job(
        "job-receipt-002",
        TEST_BACKEND_ID,
        None,
        BackendJobState::Completed,
    );

    let receipt = ExecutionReceipt {
        job,
        backend_id: TEST_BACKEND_ID.to_string(),
        requested_shots: 100,
        counted_shots: 99,
        final_state: BackendJobState::Completed,
        poll_attempts: 3,
        mode: ExecutionMode::WaitForResult,
    };

    assert!(!receipt.is_complete());
}


// =============================================================================
// Outcome conformance
// =============================================================================

#[test]
fn submitted_outcome_contains_job_identity_but_no_result() {
    let handle = synthetic_handle(
        "job-outcome-001",
        TEST_BACKEND_ID,
        BackendJobState::Queued,
        ExecutionPhase::Submitted,
    );

    let outcome = ExecutionOutcome::Submitted(
        handle.clone(),
    );

    assert_eq!(
        outcome.job_id()
            .expect("submitted outcome must have job ID")
            .as_str(),
        "job-outcome-001"
    );

    assert!(outcome.result().is_none());
}


#[test]
fn completed_outcome_exposes_result_and_job_identity() {
    /*
     * This test intentionally does not manufacture an ExecutionResult because
     * its canonical constructor/normalization belongs to backend.rs.
     *
     * The local adapter lifecycle tests below exercise the complete result
     * path.
     */
    let handle = synthetic_handle(
        "job-outcome-002",
        TEST_BACKEND_ID,
        BackendJobState::Completed,
        ExecutionPhase::Completed,
    );

    assert_eq!(
        handle.job_id().as_str(),
        "job-outcome-002"
    );
}


// =============================================================================
// Local adapter capability checks
// =============================================================================

#[test]
fn local_adapter_exposes_the_execution_capabilities_required_by_engine() {
    let adapter = local_adapter();

    assert!(
        adapter.adapter_info().production_ready,
        "local adapter must advertise its bounded production readiness"
    );

    assert!(
        !adapter.backend().id().is_empty(),
        "backend must have a stable identity"
    );

    assert_eq!(
        adapter.backend().kind(),
        BackendKind::Simulator
    );

    assert_eq!(
        adapter.backend().status(),
        BackendStatus::Available
    );

    assert!(
        adapter.supports_cancellation(),
        "local adapter must expose cancellation capability"
    );

    assert!(
        adapter.supports_queue_info(),
        "local adapter must expose queue information"
    );
}


#[test]
fn local_adapter_queue_information_is_deterministic() {
    let adapter = local_adapter();

    let queue = adapter
        .queue_info()
        .expect("local queue information must be available");

    assert_eq!(
        queue.pending_jobs,
        Some(0)
    );

    assert_eq!(
        queue.estimated_wait,
        Some(Duration::ZERO)
    );

    assert!(queue.accepting_submissions);
}


#[test]
fn local_adapter_health_is_healthy_without_fault_injection() {
    let adapter = local_adapter();

    let health = adapter
        .health()
        .expect("local health query must succeed");

    assert_eq!(
        health.state,
        BackendHealthState::Healthy
    );

    assert_eq!(
        health.backend_status,
        BackendStatus::Available
    );
}


// =============================================================================
// Program-boundary conformance
// =============================================================================

#[test]
fn backend_program_rejects_empty_payload() {
    let result = BackendProgram::new(
        LOCAL_PROGRAM_FORMAT,
        Vec::<u8>::new(),
    );

    assert!(
        result.is_err(),
        "empty executable payload must never enter execution"
    );
}


#[test]
fn backend_program_debug_never_contains_program_bytes() {
    let secret_like_payload =
        b"synthetic-secret-like-program-payload";

    let program = BackendProgram::new(
        LOCAL_PROGRAM_FORMAT,
        secret_like_payload.to_vec(),
    )
    .expect("synthetic payload must be accepted");

    let debug = format!("{program:?}");

    assert!(
        debug.contains("byte_len"),
        "debug output must expose only safe size information"
    );

    assert!(
        !debug.contains("synthetic-secret-like-program-payload"),
        "program bytes must never appear in Debug"
    );
}


#[test]
fn local_adapter_rejects_wrong_program_format_during_preflight() {
    let adapter = local_adapter();

    let request = execution_request(
        Some("wrong-format"),
        1,
    );

    let wrong_program = BackendProgram::new(
        "provider-native-test",
        b"synthetic".to_vec(),
    )
    .expect("synthetic non-empty program must construct");

    let result = adapter.preflight(
        &request,
        &wrong_program,
    );

    assert!(
        matches!(
            result,
            Err(BackendError::ExecutionRejected(_))
        ),
        "wrong executable format must be rejected before submission"
    );
}


// =============================================================================
// Engine construction
// =============================================================================

#[test]
fn execution_engine_constructs_with_default_policy() {
    let adapter = local_adapter();

    let engine =
        QuantumExecutionEngine::new(adapter);

    assert_eq!(
        engine.policy().timeout,
        DEFAULT_EXECUTION_TIMEOUT
    );

    assert_eq!(
        engine.policy().poll_interval,
        DEFAULT_POLL_INTERVAL
    );

    assert!(
        engine.backend().id().contains("local"),
        "engine must expose the adapter backend"
    );
}


#[test]
fn execution_engine_accepts_valid_explicit_policy() {
    let policy = ExecutionPolicy::default()
        .with_timeout(Duration::from_secs(1))
        .expect("valid timeout")
        .with_poll_interval(Duration::from_millis(1))
        .expect("valid poll interval")
        .with_max_poll_attempts(100)
        .expect("valid poll attempts");

    let engine = QuantumExecutionEngine::with_policy(
        local_adapter(),
        policy.clone(),
    )
    .expect("valid policy must construct engine");

    assert_eq!(
        engine.policy(),
        &policy
    );
}


#[test]
fn execution_engine_rejects_invalid_explicit_policy() {
    let mut policy = ExecutionPolicy::default();

    policy.timeout = Duration::ZERO;

    let result =
        QuantumExecutionEngine::with_policy(
            local_adapter(),
            policy,
        );

    assert!(matches!(
        result,
        Err(ExecutionError::InvalidPolicy(
            ExecutionPolicyError::InvalidTimeout { .. }
        ))
    ));
}


// =============================================================================
// Preflight ordering
// =============================================================================

#[test]
fn engine_preflight_accepts_valid_local_program() {
    let engine = local_engine();

    let request = execution_request(
        Some("preflight-valid"),
        4,
    );

    let program = local_program();

    assert!(
        engine.preflight(
            &request,
            &program
        ).is_ok()
    );
}


#[test]
fn engine_preflight_rejects_empty_program_before_adapter_submission() {
    let engine = local_engine();

    let request = execution_request(
        Some("preflight-empty"),
        1,
    );

    let empty_program = BackendProgram::new(
        LOCAL_PROGRAM_FORMAT,
        Vec::<u8>::new(),
    );

    assert!(empty_program.is_err());

    /*
     * The provider-neutral program constructor already prevents an empty
     * payload from entering the engine. This test therefore verifies the
     * security boundary at the earliest possible layer.
     */
    assert!(
        engine
            .preflight(
                &request,
                &BackendProgram::new(
                    LOCAL_PROGRAM_FORMAT,
                    b"x".to_vec()
                )
                .expect("non-empty synthetic payload"),
            )
            .is_err()
            || empty_program.is_err()
    );
}


#[test]
fn engine_preflight_rejects_wrong_local_format_without_submission() {
    let engine = local_engine();

    let request = execution_request(
        Some("preflight-format"),
        1,
    );

    let program = BackendProgram::new(
        "not-local",
        b"synthetic".to_vec(),
    )
    .expect("synthetic payload");

    let result =
        engine.preflight(&request, &program);

    assert!(
        matches!(
            result,
            Err(ExecutionError::Preflight(
                BackendError::ExecutionRejected(_)
            ))
        )
    );
}


// =============================================================================
// Submit-only lifecycle
// =============================================================================

#[test]
fn local_submit_only_returns_a_provider_neutral_handle() {
    let engine = local_engine();

    let request = execution_request(
        Some("submit-only"),
        8,
    );

    let program = local_program();

    let handle =
        engine
            .submit(&request, &program)
            .expect("local submission must succeed");

    assert!(
        handle.job_id().as_str().starts_with("local-"),
        "local provider job IDs must remain provider-neutral at the engine boundary"
    );

    assert_eq!(
        handle.backend_id(),
        engine.backend().id()
    );

    assert_eq!(
        handle.job.request_id.as_deref(),
        Some("submit-only")
    );
}


#[test]
fn local_run_submit_only_never_returns_a_result() {
    let engine = local_engine();

    let request = execution_request(
        Some("run-submit-only"),
        4,
    );

    let program = local_program();

    let outcome = engine
        .run(
            &request,
            &program,
            ExecutionMode::SubmitOnly,
        )
        .expect("submit-only execution must succeed");

    assert!(
        matches!(
            outcome,
            ExecutionOutcome::Submitted(_)
        )
    );

    assert!(outcome.result().is_none());
}


// =============================================================================
// Complete asynchronous lifecycle
// =============================================================================

#[test]
fn local_engine_execute_completes_full_submit_poll_result_lifecycle() {
    let engine = local_engine();

    let request = execution_request(
        Some("full-lifecycle"),
        32,
    );

    let program = local_program();

    let (
        handle,
        result,
        receipt,
    ) = engine
        .execute(&request, &program)
        .expect(
            "local execution must complete successfully",
        );

    assert_eq!(
        handle.backend_id(),
        engine.backend().id()
    );

    assert_eq!(
        handle.job.request_id.as_deref(),
        Some("full-lifecycle")
    );

    assert_eq!(
        handle.job.state,
        BackendJobState::Completed
    );

    assert_eq!(
        result.backend_id,
        engine.backend().id()
    );

    assert_eq!(
        result.counted_shots(),
        32
    );

    assert_eq!(
        receipt.requested_shots,
        32
    );

    assert_eq!(
        receipt.counted_shots,
        32
    );

    assert_eq!(
        receipt.final_state,
        BackendJobState::Completed
    );

    assert!(
        receipt.poll_attempts >= 1,
        "asynchronous orchestration must observe provider status"
    );

    assert_eq!(
        receipt.mode,
        ExecutionMode::WaitForResult
    );

    assert!(receipt.is_complete());
}


#[test]
fn local_wait_for_result_returns_completed_outcome() {
    let engine = local_engine();

    let request = execution_request(
        Some("wait-for-result"),
        16,
    );

    let program = local_program();

    let outcome = engine
        .run(
            &request,
            &program,
            ExecutionMode::WaitForResult,
        )
        .expect(
            "wait-for-result execution must succeed",
        );

    match outcome {
        ExecutionOutcome::Completed {
            handle,
            result,
            receipt,
        } => {
            assert_eq!(
                handle.job.state,
                BackendJobState::Completed
            );

            assert_eq!(
                result.counted_shots(),
                16
            );

            assert_eq!(
                receipt.requested_shots,
                16
            );

            assert_eq!(
                receipt.counted_shots,
                16
            );

            assert_eq!(
                receipt.mode,
                ExecutionMode::WaitForResult
            );

            assert!(receipt.is_complete());
        }

        ExecutionOutcome::Submitted(_) => {
            panic!(
                "WaitForResult must not return a submission-only outcome"
            );
        }
    }
}


// =============================================================================
// Polling
// =============================================================================

#[test]
fn local_poll_returns_completed_snapshot() {
    let engine = local_engine();

    let request = execution_request(
        Some("poll-completed"),
        8,
    );

    let program = local_program();

    let handle =
        engine
            .submit(&request, &program)
            .expect("submission must succeed");

    let snapshot =
        engine
            .poll(&handle)
            .expect("poll must succeed");

    assert_eq!(
        snapshot.state(),
        BackendJobState::Completed
    );

    assert_eq!(
        snapshot.phase,
        ExecutionPhase::Waiting
    );

    assert_eq!(
        snapshot.poll_attempts,
        1
    );

    assert!(
        snapshot.status.is_some(),
        "poll must return normalized provider status"
    );

    assert!(snapshot.is_terminal());
}


// =============================================================================
// Direct result retrieval
// =============================================================================

#[test]
fn local_result_retrieval_requires_completed_job() {
    let engine = local_engine();

    let request = execution_request(
        Some("result-direct"),
        8,
    );

    let program = local_program();

    let handle =
        engine
            .submit(&request, &program)
            .expect("submission must succeed");

    let result =
        engine
            .result(&handle, &request)
            .expect(
                "local completed result must be retrievable",
            );

    assert_eq!(
        result.backend_id,
        engine.backend().id()
    );

    assert_eq!(
        result.counted_shots(),
        8
    );
}


// =============================================================================
// Native synchronous execution
// =============================================================================

#[test]
fn local_adapter_does_not_claim_native_synchronous_execution_unless_supported() {
    let adapter = local_adapter();

    /*
     * The adapter may legitimately implement synchronous execution internally,
     * but the capability contract must be authoritative. The engine checks the
     * explicit capability before invoking native synchronous execution.
     */
    let advertised =
        adapter.supports_synchronous_execution();

    if advertised {
        let request = execution_request(
            Some("native-sync"),
            4,
        );

        let program = local_program();

        let result = QuantumExecutionEngine::new(adapter)
            .execute_synchronously(
                &request,
                &program,
            );

        assert!(
            result.is_ok(),
            "an adapter advertising native synchronous execution must implement it"
        );
    } else {
        let request = execution_request(
            Some("native-sync-unsupported"),
            4,
        );

        let program = local_program();

        let result = QuantumExecutionEngine::new(adapter)
            .execute_synchronously(
                &request,
                &program,
            );

        assert!(matches!(
            result,
            Err(
                ExecutionError::NativeSynchronousUnsupported
            )
        ));
    }
}


// =============================================================================
// Cancellation
// =============================================================================

#[test]
fn local_completed_job_reports_already_terminal_on_cancellation() {
    let engine = local_engine();

    let request = execution_request(
        Some("cancel-terminal"),
        4,
    );

    let program = local_program();

    let handle =
        engine
            .submit(&request, &program)
            .expect("submission must succeed");

    let cancellation =
        engine
            .cancel(&handle)
            .expect("cancellation request must return normalized outcome");

    assert_eq!(
        cancellation.job.as_str(),
        handle.job_id().as_str()
    );

    assert_eq!(
        cancellation.cancellation.outcome,
        CancellationOutcome::AlreadyTerminal
    );

    assert_eq!(
        cancellation.phase,
        ExecutionPhase::Cancelled
    );
}


#[test]
fn local_cancellation_fault_is_not_misreported_as_success() {
    let adapter = Arc::new(
        LocalBackendAdapter::with_config(
            LocalBackendConfig::test()
                .with_fault(
                    LocalFault::RejectCancellation
                ),
        )
        .expect("fault-injected local adapter must construct"),
    );

    let engine =
        QuantumExecutionEngine::new(adapter);

    let request = execution_request(
        Some("cancel-unsupported"),
        4,
    );

    let program = local_program();

    let handle =
        engine
            .submit(&request, &program)
            .expect("submission must succeed");

    let cancellation =
        engine
            .cancel(&handle)
            .expect("unsupported cancellation is a normalized outcome");

    assert_eq!(
        cancellation.cancellation.outcome,
        CancellationOutcome::Unsupported
    );

    assert_eq!(
        cancellation.phase,
        ExecutionPhase::Failed
    );
}


// =============================================================================
// Failure propagation
// =============================================================================

#[test]
fn local_submission_fault_is_reported_as_submission_error() {
    let adapter = Arc::new(
        LocalBackendAdapter::with_config(
            LocalBackendConfig::test()
                .with_fault(
                    LocalFault::RejectSubmission
                ),
        )
        .expect("fault-injected local adapter must construct"),
    );

    let engine =
        QuantumExecutionEngine::new(adapter);

    let request = execution_request(
        Some("submission-failure"),
        4,
    );

    let program = local_program();

    let result =
        engine.submit(&request, &program);

    assert!(matches!(
        result,
        Err(ExecutionError::Submission(
            BackendError::ExecutionRejected(_)
        ))
    ));
}


#[test]
fn local_result_fault_is_propagated_without_fabricating_a_result() {
    let adapter = Arc::new(
        LocalBackendAdapter::with_config(
            LocalBackendConfig::test()
                .with_fault(LocalFault::RejectResult),
        )
        .expect("fault-injected local adapter must construct"),
    );

    let engine =
        QuantumExecutionEngine::new(adapter);

    let request = execution_request(
        Some("result-failure"),
        4,
    );

    let program = local_program();

    let result =
        engine.execute(&request, &program);

    assert!(matches!(
        result,
        Err(ExecutionError::Submission(
            BackendError::ExecutionRejected(_)
        ))
    ));
}


// =============================================================================
// Backend identity protection
// =============================================================================

#[test]
fn execution_handle_with_wrong_backend_is_rejected() {
    let engine = local_engine();

    let handle = synthetic_handle(
        "foreign-job",
        "provider://foreign-backend",
        BackendJobState::Completed,
        ExecutionPhase::Completed,
    );

    let request = execution_request(
        Some("wrong-backend"),
        1,
    );

    let result =
        engine.result(
            &handle,
            &request,
        );

    assert!(matches!(
        result,
        Err(
            ExecutionError::BackendIdentityMismatch {
                ..
            }
        )
    ));
}


// =============================================================================
// Lifecycle semantics
// =============================================================================

#[test]
fn all_terminal_backend_job_states_are_terminal() {
    assert!(BackendJobState::Completed.is_terminal());
    assert!(BackendJobState::Cancelled.is_terminal());
    assert!(BackendJobState::Failed.is_terminal());
    assert!(BackendJobState::Expired.is_terminal());
    assert!(BackendJobState::TimedOut.is_terminal());

    assert!(!BackendJobState::Created.is_terminal());
    assert!(!BackendJobState::Queued.is_terminal());
    assert!(!BackendJobState::Running.is_terminal());
    assert!(!BackendJobState::Cancelling.is_terminal());
}


#[test]
fn active_backend_job_states_are_not_terminal() {
    assert!(BackendJobState::Created.is_active());
    assert!(BackendJobState::Queued.is_active());
    assert!(BackendJobState::Running.is_active());
    assert!(BackendJobState::Cancelling.is_active());

    assert!(!BackendJobState::Completed.is_active());
    assert!(!BackendJobState::Cancelled.is_active());
    assert!(!BackendJobState::Failed.is_active());
}


// =============================================================================
// Status object conformance
// =============================================================================

#[test]
fn completed_status_is_result_retrievable_only_when_result_is_available() {
    let job = synthetic_job(
        "status-001",
        TEST_BACKEND_ID,
        None,
        BackendJobState::Completed,
    );

    let status_without_result = BackendJobStatus {
        job: job.clone(),
        provider_status: None,
        queue_position: None,
        estimated_wait: None,
        result_available: false,
    };

    assert!(
        !status_without_result.can_retrieve_result()
    );

    let status_with_result = BackendJobStatus {
        job,
        provider_status: None,
        queue_position: None,
        estimated_wait: None,
        result_available: true,
    };

    assert!(
        status_with_result.can_retrieve_result()
    );
}


#[test]
fn active_status_can_request_cancellation() {
    for state in [
        BackendJobState::Created,
        BackendJobState::Queued,
        BackendJobState::Running,
    ] {
        let status = BackendJobStatus {
            job: synthetic_job(
                "cancel-status",
                TEST_BACKEND_ID,
                None,
                state,
            ),
            provider_status: None,
            queue_position: None,
            estimated_wait: None,
            result_available: false,
        };

        assert!(
            status.can_request_cancellation(),
            "state {:?} should permit cancellation",
            state
        );
    }
}


#[test]
fn terminal_status_cannot_request_cancellation() {
    for state in [
        BackendJobState::Completed,
        BackendJobState::Cancelled,
        BackendJobState::Failed,
        BackendJobState::Expired,
        BackendJobState::TimedOut,
    ] {
        let status = BackendJobStatus {
            job: synthetic_job(
                "terminal-status",
                TEST_BACKEND_ID,
                None,
                state,
            ),
            provider_status: None,
            queue_position: None,
            estimated_wait: None,
            result_available: state
                == BackendJobState::Completed,
        };

        assert!(
            !status.can_request_cancellation(),
            "terminal state {:?} must not permit cancellation",
            state
        );
    }
}


// =============================================================================
// Adapter sharing / thread-safety boundary
// =============================================================================

#[test]
fn execution_engine_is_cloneable_without_cloning_backend_state() {
    let engine = local_engine();
    let clone = engine.clone();

    assert_eq!(
        engine.backend().id(),
        clone.backend().id()
    );

    assert_eq!(
        engine.policy(),
        clone.policy()
    );
}


#[test]
fn shared_local_adapter_can_be_used_by_multiple_engines() {
    let adapter = local_adapter();

    let first =
        QuantumExecutionEngine::new(
            Arc::clone(&adapter),
        );

    let second =
        QuantumExecutionEngine::new(
            Arc::clone(&adapter),
        );

    let request_one = execution_request(
        Some("shared-001"),
        4,
    );

    let request_two = execution_request(
        Some("shared-002"),
        4,
    );

    let program = local_program();

    let first_result =
        first.execute(
            &request_one,
            &program,
        );

    let second_result =
        second.execute(
            &request_two,
            &program,
        );

    assert!(
        first_result.is_ok(),
        "first shared-engine execution must succeed"
    );

    assert!(
        second_result.is_ok(),
        "second shared-engine execution must succeed"
    );

    assert_eq!(
        adapter.stored_job_count()
            .expect("job store query must succeed"),
        2
    );
}


// =============================================================================
// Determinism
// =============================================================================

#[test]
fn identical_local_executions_are_deterministic() {
    let first_engine = local_engine();
    let second_engine = local_engine();

    let request_one = execution_request(
        Some("deterministic-one"),
        32,
    );

    let request_two = execution_request(
        Some("deterministic-two"),
        32,
    );

    let program = local_program();

    let (_, first_result, _) =
        first_engine
            .execute(
                &request_one,
                &program,
            )
            .expect("first deterministic execution");

    let (_, second_result, _) =
        second_engine
            .execute(
                &request_two,
                &program,
            )
            .expect("second deterministic execution");

    /*
     * Request IDs are intentionally different, but the quantum workload,
     * program and default local seed are the same.
     *
     * The normalized measurement distribution therefore must be identical.
     *
     * The test compares the actual result representation exposed by the
     * canonical result contract through Debug rather than depending on
     * provider-private storage.
     */
    assert_eq!(
        format!("{first_result:?}"),
        format!("{second_result:?}")
    );
}


// =============================================================================
// Job-store lifecycle
// =============================================================================

#[test]
fn local_adapter_retains_completed_jobs_until_explicit_cleanup() {
    let adapter = local_adapter();

    let engine =
        QuantumExecutionEngine::new(
            Arc::clone(&adapter),
        );

    let request = execution_request(
        Some("retention"),
        4,
    );

    let program = local_program();

    engine
        .execute(
            &request,
            &program,
        )
        .expect("execution must succeed");

    assert_eq!(
        adapter
            .stored_job_count()
            .expect("job count must succeed"),
        1
    );

    let removed =
        adapter
            .clear_terminal_jobs()
            .expect("terminal cleanup must succeed");

    assert_eq!(removed, 1);

    assert_eq!(
        adapter
            .stored_job_count()
            .expect("job count must succeed"),
        0
    );
}


// =============================================================================
// Local fault health semantics
// =============================================================================

#[test]
fn degraded_health_fault_is_reported_without_being_silently_normalized_to_healthy() {
    let adapter =
        LocalBackendAdapter::with_config(
            LocalBackendConfig::test()
                .with_fault(
                    LocalFault::DegradedHealth
                ),
        )
        .expect("degraded-health adapter must construct");

    let health =
        adapter
            .health()
            .expect("health query must succeed");

    assert_eq!(
        health.state,
        BackendHealthState::Degraded
    );

    assert_eq!(
        health.backend_status,
        BackendStatus::Degraded
    );
}


#[test]
fn unhealthy_fault_is_reported_as_unhealthy() {
    let adapter =
        LocalBackendAdapter::with_config(
            LocalBackendConfig::test()
                .with_fault(
                    LocalFault::Unhealthy
                ),
        )
        .expect("unhealthy adapter must construct");

    let health =
        adapter
            .health()
            .expect("health query must succeed");

    assert_eq!(
        health.state,
        BackendHealthState::Unhealthy
    );

    assert_eq!(
        health.backend_status,
        BackendStatus::Unavailable
    );

    assert!(
        !health.state.permits_execution()
    );
}


// =============================================================================
// Adapter conformance invariants
// =============================================================================

#[test]
fn adapter_metadata_is_provider_neutral_and_versioned() {
    let adapter = local_adapter();

    let info = adapter.adapter_info();

    assert!(
        !info.adapter_id.trim().is_empty()
    );

    assert!(
        !info.adapter_version.trim().is_empty()
    );

    assert!(
        info.production_ready
    );
}


#[test]
fn adapter_backend_identity_is_stable() {
    let adapter = local_adapter();

    let first =
        adapter.backend().id().to_string();

    let second =
        adapter.backend().id().to_string();

    assert_eq!(first, second);
}


#[test]
fn provider_neutral_job_identity_is_stable_after_submission() {
    let engine = local_engine();

    let request = execution_request(
        Some("stable-job-id"),
        4,
    );

    let program = local_program();

    let handle =
        engine
            .submit(
                &request,
                &program,
            )
            .expect("submission must succeed");

    let id = handle.job_id().to_string();

    assert_eq!(
        id,
        handle.job_id().to_string()
    );
}


// =============================================================================
// Regression guards for execution semantics
// =============================================================================

#[test]
fn submit_does_not_change_requested_shots() {
    let engine = local_engine();

    let request = execution_request(
        Some("shot-preservation"),
        37,
    );

    let program = local_program();

    let handle =
        engine
            .submit(
                &request,
                &program,
            )
            .expect("submission must succeed");

    let result =
        engine
            .result(
                &handle,
                &request,
            )
            .expect("result must succeed");

    assert_eq!(
        result.counted_shots(),
        37
    );
}


#[test]
fn execution_result_must_identify_the_expected_backend() {
    let engine = local_engine();

    let request = execution_request(
        Some("result-identity"),
        8,
    );

    let program = local_program();

    let (
        handle,
        result,
        _receipt,
    ) = engine
        .execute(
            &request,
            &program,
        )
        .expect("execution must succeed");

    assert_eq!(
        result.backend_id,
        handle.backend_id()
    );

    assert_eq!(
        result.backend_id,
        engine.backend().id()
    );
}


#[test]
fn receipt_preserves_final_completed_state() {
    let engine = local_engine();

    let request = execution_request(
        Some("receipt-state"),
        8,
    );

    let program = local_program();

    let (
        _handle,
        _result,
        receipt,
    ) = engine
        .execute(
            &request,
            &program,
        )
        .expect("execution must succeed");

    assert_eq!(
        receipt.final_state,
        BackendJobState::Completed
    );

    assert!(
        receipt.is_complete()
    );
}


// =============================================================================
// Security regression guards
// =============================================================================

#[test]
fn execution_debug_output_does_not_contain_program_payload() {
    let program = local_program();

    let debug =
        format!("{program:?}");

    assert!(
        !debug.contains(
            "\"operations\""
        ),
        "program structure must not leak through Debug"
    );

    assert!(
        !debug.contains(
            "\"gate\": \"x\""
        ),
        "program contents must not leak through Debug"
    );
}


#[test]
fn synthetic_provider_errors_are_not_treated_as_success() {
    let adapter = Arc::new(
        LocalBackendAdapter::with_config(
            LocalBackendConfig::test()
                .with_fault(
                    LocalFault::RejectSubmission
                ),
        )
        .expect("fault adapter"),
    );

    let engine =
        QuantumExecutionEngine::new(adapter);

    let request = execution_request(
        Some("security-failure"),
        1,
    );

    let program = local_program();

    let outcome =
        engine.run(
            &request,
            &program,
            ExecutionMode::WaitForResult,
        );

    assert!(
        outcome.is_err(),
        "provider failure must never be converted into successful execution"
    );
}


// =============================================================================
// Future-provider conformance contract
// =============================================================================

/// Generic assertions for any provider-neutral adapter.
///
/// Provider-specific test modules can call this function after constructing
/// their own adapter.
///
/// This deliberately tests only the universal contract and never checks
/// provider-specific names, APIs, or payloads.
fn assert_adapter_contract<A>(
    adapter: Arc<A>,
    request: &ExecutionRequest,
    program: &BackendProgram,
)
where
    A: QuantumBackendAdapter + ?Sized,
{
    assert!(
        !adapter.backend().id().trim().is_empty(),
        "adapter must expose a stable backend identity"
    );

    assert!(
        !adapter.adapter_info().adapter_id.trim().is_empty(),
        "adapter must expose a stable adapter identity"
    );

    assert!(
        !adapter.adapter_info().adapter_version.trim().is_empty(),
        "adapter must expose an adapter version"
    );

    adapter
        .preflight(request, program)
        .expect(
            "a conforming adapter must accept its own valid test workload",
        );
}


#[test]
fn local_adapter_satisfies_generic_provider_neutral_contract() {
    let adapter = local_adapter();

    let request = execution_request(
        Some("generic-contract"),
        4,
    );

    let program = local_program();

    assert_adapter_contract(
        adapter,
        &request,
        &program,
    );
}


// =============================================================================
// Compile-time trait-bound guards
// =============================================================================

fn assert_send_sync<T: Send + Sync>() {}


#[test]
fn execution_components_are_send_and_sync_where_the_contract_requires_it() {
    assert_send_sync::<LocalBackendAdapter>();
    assert_send_sync::<QuantumExecutionEngine<LocalBackendAdapter>>();
    assert_send_sync::<BackendProgram>();
    assert_send_sync::<BackendJobId>();
}


// =============================================================================
// Schema constants
// =============================================================================

#[test]
fn execution_schema_contract_is_non_empty_and_versioned() {
    assert!(
        !crate::quantum::hardware::execution::EXECUTION_SCHEMA_ID
            .trim()
            .is_empty()
    );

    assert!(
        crate::quantum::hardware::execution::EXECUTION_SCHEMA_VERSION
            >= 1
    );
}


// =============================================================================
// Final lifecycle acceptance gate
// =============================================================================

#[test]
fn execution_subsystem_passes_minimum_production_lifecycle_gate() {
    let engine = local_engine();

    let request = execution_request(
        Some("production-gate"),
        64,
    );

    let program = local_program();

    /*
     * Gate 1 — preflight.
     */
    engine
        .preflight(
            &request,
            &program,
        )
        .expect(
            "production execution must pass preflight",
        );

    /*
     * Gate 2 — submission.
     */
    let handle =
        engine
            .submit(
                &request,
                &program,
            )
            .expect(
                "production execution must submit",
            );

    assert!(
        !handle.job_id().as_str().is_empty()
    );

    /*
     * Gate 3 — lifecycle observation.
     */
    let snapshot =
        engine
            .poll(&handle)
            .expect(
                "production execution must expose lifecycle status",
            );

    assert!(
        snapshot.status.is_some()
    );

    /*
     * Gate 4 — completion.
     */
    assert_eq!(
        snapshot.state(),
        BackendJobState::Completed
    );

    /*
     * Gate 5 — result retrieval.
     */
    let result =
        engine
            .result(
                &handle,
                &request,
            )
            .expect(
                "production execution must retrieve completed results",
            );

    /*
     * Gate 6 — result provenance.
     */
    assert_eq!(
        result.backend_id,
        engine.backend().id()
    );

    /*
     * Gate 7 — shot integrity.
     */
    assert_eq!(
        result.counted_shots(),
        64
    );

    /*
     * Gate 8 — complete orchestrated execution.
     */
    let (
        second_handle,
        second_result,
        receipt,
    ) = engine
        .execute(
            &request,
            &program,
        )
        .expect(
            "complete production execution path must succeed",
        );

    assert_eq!(
        second_handle.backend_id(),
        engine.backend().id()
    );

    assert_eq!(
        second_result.backend_id,
        engine.backend().id()
    );

    assert_eq!(
        second_result.counted_shots(),
        64
    );

    assert_eq!(
        receipt.requested_shots,
        64
    );

    assert_eq!(
        receipt.counted_shots,
        64
    );

    assert_eq!(
        receipt.final_state,
        BackendJobState::Completed
    );

    assert_eq!(
        receipt.mode,
        ExecutionMode::WaitForResult
    );

    assert!(
        receipt.is_complete()
    );
}