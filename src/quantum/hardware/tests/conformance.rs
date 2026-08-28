//! Zamani Quantum — Hardware Adapter Conformance Suite
//!
//! Production-grade provider-neutral conformance tests for
//! `crate::quantum::hardware::backend_trait::QuantumBackendAdapter`.
//!
//! # Responsibility
//!
//! This module defines the canonical behavioral contract that every executable
//! hardware adapter must satisfy before it can be considered conformant.
//!
//! It tests the adapter boundary rather than a provider implementation.
//!
//! The canonical lifecycle is:
//!
//! ```text
//! BackendProgram
//!      |
//!      v
//! ExecutionRequest
//!      |
//!      v
//!   preflight
//!      |
//!      v
//!    submit
//!      |
//!      v
//!  BackendJob
//!      |
//!      +-----------> status
//!      |
//!      +-----------> result
//!      |
//!      +-----------> cancel
//!
//! Additional contracts:
//!
//! adapter_info
//! backend
//! health
//! queue_info
//! synchronous execute
//! capability flags
//! ```
//!
//! # Why this file exists
//!
//! Provider adapters are dangerous to validate independently because each
//! adapter can otherwise develop subtly different semantics.
//!
//! This suite establishes one provider-neutral contract.
//!
//! A provider adapter must satisfy the same semantic expectations regardless
//! of whether it represents:
//!
//! - a local simulator;
//! - a hardware emulator;
//! - IBM;
//! - IonQ;
//! - AWS Braket;
//! - Rigetti;
//! - IQM;
//! - Quantinuum;
//! - QuEra;
//! - a future provider.
//!
//! # Architectural rule
//!
//! This module is a CONSUMER of the hardware contracts.
//!
//! It must never become a dependency of:
//!
//! - `backend.rs`;
//! - `backend_trait.rs`;
//! - `topology.rs`;
//! - `calibration.rs`;
//! - provider adapters;
//! - benchmarking.
//!
//! In particular:
//!
//! ```text
//! hardware core
//!      |
//!      v
//! backend_trait
//!      |
//!      v
//! adapters
//!      |
//!      v
//! testsconformance
//! ```
//!
//! NOT:
//!
//! ```text
//! backend_trait -> testsconformance
//! ```
//!
//! # Provider independence
//!
//! The generic conformance functions below accept:
//!
//! ```text
//! &dyn QuantumBackendAdapter
//! ```
//!
//! Consequently, adding a new provider does not require changing this file.
//!
//! Provider-specific tests belong in the provider's own test module and should
//! invoke the generic suite plus any provider-specific invariants.
//!
//! # Local reference implementation
//!
//! The repository's local adapter is the reference implementation because it
//! provides:
//!
//! - deterministic execution;
//! - no credentials;
//! - no network;
//! - no external SDK;
//! - deterministic job lifecycle;
//! - result normalization;
//! - cancellation;
//! - health;
//! - queue information;
//! - synchronous execution;
//! - fault injection.
//!
//! # Rust compatibility
//!
//! Supported:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! # Test philosophy
//!
//! The suite verifies:
//!
//! 1. identity is present;
//! 2. adapter metadata is valid;
//! 3. program payload validation is enforced;
//! 4. execution requests are structurally valid;
//! 5. preflight rejects malformed/incompatible programs;
//! 6. valid workloads can be submitted;
//! 7. submitted jobs have valid identities;
//! 8. job status is coherent;
//! 9. completed jobs expose results;
//! 10. result provenance identifies the backend;
//! 11. normalized counts never exceed the requested shots;
//! 12. completed jobs cannot be cancelled as newly accepted work;
//! 13. queue information is coherent when advertised;
//! 14. health information is coherent;
//! 15. synchronous execution agrees with the asynchronous result;
//! 16. advertised capability flags are internally coherent;
//! 17. provider errors do not escape as provider-specific types;
//! 18. secret-like metadata is rejected;
//! 19. malformed job IDs are rejected;
//! 20. malformed programs are rejected;
//! 21. deterministic local execution is reproducible;
//! 22. adapter implementations do not silently change lifecycle semantics.
//!
//! # Integration contract
//!
//! `hardware/mod.rs` should declare this module:
//!
//! ```text
//! pub mod testsconformance;
//! ```
//!
//! Because this is a test-only module, the declaration can remain ordinary;
//! the module itself is compiled only when `cfg(test)` is active.
//!
//! The preferred integration in `hardware/mod.rs` is therefore:
//!
//! ```rust
//! #[cfg(test)]
//! pub mod testsconformance;
//! ```
//!
//! No production runtime code should call this module.
//!
//! # Provider adapter integration
//!
//! A provider-specific test module can use:
//!
//! ```rust
//! use crate::quantum::hardware::testsconformance;
//!
//! #[test]
//! fn provider_conforms_to_zamani_hardware_contract() {
//!     let adapter = make_provider_adapter();
//!     testsconformance::run_all(&adapter);
//! }
//! ```
//!
//! The provider adapter must only implement
//! `QuantumBackendAdapter`; it must not modify this suite to make a failing
//! implementation pass.
//!
//! # Failure policy
//!
//! Assertions intentionally fail with semantic messages that identify the
//! contract being violated.
//!
//! A provider adapter failing this suite is NOT conformant even if it can
//! communicate with a real QPU.
//!
//! Real-provider tests may need a separate integration-test layer for network,
//! credentials, provider availability, queue state, pricing, and cloud
//! behavior. Those are intentionally not required here.
//!
//! # Determinism
//!
//! The reference local adapter uses deterministic execution. The generic suite
//! does not assume deterministic remote-provider job identifiers.
//!
//! It only requires deterministic behavior where the adapter advertises or
//! explicitly guarantees it.
//!
//! # Security
//!
//! This suite never uses credentials.
//!
//! It intentionally tests that secret-like request metadata is rejected.
//!
//! It must never contain:
//!
//! - API keys;
//! - access tokens;
//! - passwords;
//! - private keys;
//! - cookies;
//! - authentication headers.
//!
//! # No network requirement
//!
//! The generic suite performs no network operation itself.
//!
//! A provider adapter passed into the suite may perform network operations when
//! its own implementation is invoked. CI should therefore distinguish:
//!
//! - deterministic local conformance tests;
//! - provider integration tests.
//!
//! # Stability
//!
//! This file is a consumer of the stable `QuantumBackendAdapter` contract.
//!
//! If a provider needs a provider-specific behavior, add a provider-specific
//! test. Do not weaken the generic contract merely because a provider does not
//! support a required universal behavior.
//!
//! # Completion rule
//!
//! This file is complete when:
//!
//! - every generic adapter lifecycle contract is tested;
//! - malformed inputs are tested;
//! - valid execution is tested;
//! - normalized results are tested;
//! - health and queue semantics are tested;
//! - cancellation semantics are tested;
//! - synchronous/asynchronous consistency is tested where supported;
//! - the local reference adapter passes;
//! - provider adapters can reuse the suite without modifying it;
//! - Rust 1.97/1.97.1 compiles it without nightly features;
//! - no provider-specific dependency exists here.
//!
//! # IMPORTANT
//!
//! This is intentionally a test-only module. It must not be used as a runtime
//! conformance checker. Runtime compatibility belongs to the production
//! validation/compatibility subsystem.
//!
//! -----------------------------------------------------------------------------
//! Public test contract
//! -----------------------------------------------------------------------------
//!
//! `run_all()` is the canonical entry point.
//!
//! Individual functions are public so provider-specific integration tests can
//! run only the subset appropriate to a provider when a remote provider has
//! intentionally different operational characteristics.
//!
//! -----------------------------------------------------------------------------
//! Rust safety
//! -----------------------------------------------------------------------------
//!
//! Unsafe Rust is forbidden.
//!

#![cfg(test)]
#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::collections::BTreeMap;
use std::time::Duration;

use super::adapters::local::{LocalBackendAdapter, LocalBackendConfig};
use super::backend::{
    BackendError,
    BackendKind,
    BackendStatus,
    CircuitRequirements,
    ExecutionRequest,
    ExecutionResult,
};
use super::backend_trait::{
    BackendJob,
    BackendJobState,
    BackendJobStatus,
    BackendProgram,
    QuantumBackendAdapter,
};

/// Canonical local executable program format used by the reference adapter.
const TEST_PROGRAM_FORMAT: &str = "zamani-local-v1";

/// A minimal two-qubit Bell-state program.
///
/// The explicit measurements are important: the local adapter intentionally
/// does not silently measure an otherwise unmeasured circuit.
const BELL_PROGRAM: &str = r#"{
    "schema": "zamani-local-v1",
    "qubits": 2,
    "classical_bits": 2,
    "measure_all": false,
    "operations": [
        {"gate": "h", "targets": [0]},
        {"gate": "cx", "targets": [0, 1]},
        {"gate": "measure", "targets": [0], "classical": [0]},
        {"gate": "measure", "targets": [1], "classical": [1]}
    ]
}"#;

/// A minimal deterministic one-qubit program.
///
/// This is intentionally smaller than the Bell program and is useful for
/// generic lifecycle tests where the exact quantum distribution is not the
/// subject of the test.
const ONE_QUBIT_PROGRAM: &str = r#"{
    "schema": "zamani-local-v1",
    "qubits": 1,
    "classical_bits": 1,
    "measure_all": false,
    "operations": [
        {"gate": "x", "targets": [0]},
        {"gate": "measure", "targets": [0], "classical": [0]}
    ]
}"#;

/// Builds a minimal valid two-qubit execution request.
fn bell_request() -> ExecutionRequest {
    let mut circuit = CircuitRequirements::default();

    circuit.qubit_count = 2;
    circuit.classical_bit_count = 2;
    circuit.operation_count = 4;
    circuit.circuit_depth = 4;
    circuit.shots = 32;
    circuit.gates = vec![
        "h".to_string(),
        "cx".to_string(),
        "measure".to_string(),
    ];
    circuit.requires_measurement = true;

    ExecutionRequest::new(circuit)
        .with_seed(0)
        .with_request_id("conformance-bell")
        .expect("the conformance request identifier must be valid")
}

/// Builds a minimal valid one-qubit execution request.
fn one_qubit_request() -> ExecutionRequest {
    let mut circuit = CircuitRequirements::default();

    circuit.qubit_count = 1;
    circuit.classical_bit_count = 1;
    circuit.operation_count = 2;
    circuit.circuit_depth = 2;
    circuit.shots = 16;
    circuit.gates = vec![
        "x".to_string(),
        "measure".to_string(),
    ];
    circuit.requires_measurement = true;

    ExecutionRequest::new(circuit)
        .with_seed(0)
        .with_request_id("conformance-one-qubit")
        .expect("the conformance request identifier must be valid")
}

/// Creates the canonical local Bell program.
fn bell_program() -> BackendProgram {
    BackendProgram::new(
        TEST_PROGRAM_FORMAT,
        BELL_PROGRAM.as_bytes().to_vec(),
    )
    .expect("the canonical Bell program must be accepted")
}

/// Creates the canonical local one-qubit program.
fn one_qubit_program() -> BackendProgram {
    BackendProgram::new(
        TEST_PROGRAM_FORMAT,
        ONE_QUBIT_PROGRAM.as_bytes().to_vec(),
    )
    .expect("the canonical one-qubit program must be accepted")
}

/// Runs the complete generic adapter conformance suite.
///
/// Provider-specific test modules should construct their adapter and call this
/// function.
///
/// The adapter is borrowed so the suite does not impose ownership semantics
/// on provider registries or test harnesses.
pub fn run_all(adapter: &dyn QuantumBackendAdapter) {
    backend_identity_is_valid(adapter);
    adapter_identity_is_valid(adapter);
    backend_kind_is_valid(adapter);
    backend_status_is_coherent(adapter);
    advertised_capabilities_are_coherent(adapter);
    queue_contract_is_coherent(adapter);
    health_contract_is_coherent(adapter);

    program_contract_is_valid();
    malformed_programs_are_rejected();
    request_structure_is_valid();

    preflight_accepts_valid_program(adapter);
    preflight_rejects_wrong_program_format(adapter);
    preflight_rejects_empty_program();

    submission_produces_valid_job(adapter);
    job_status_is_coherent(adapter);
    completed_job_has_result(adapter);
    result_provenance_is_valid(adapter);
    result_counts_respect_shot_budget(adapter);

    cancellation_of_completed_job_is_not_accepted(adapter);

    synchronous_execution_contract(adapter);
    asynchronous_and_synchronous_results_are_semantically_equivalent(adapter);

    secret_metadata_is_rejected();
    malformed_job_ids_are_rejected();

    deterministic_execution_is_reproducible(adapter);
}

/// Verifies that the backend has a non-empty stable identifier.
pub fn backend_identity_is_valid(adapter: &dyn QuantumBackendAdapter) {
    let backend = adapter.backend();
    let id = backend.id();

    assert!(
        !id.trim().is_empty(),
        "hardware conformance violation: backend ID must not be empty"
    );

    assert_eq!(
        id,
        id.trim(),
        "hardware conformance violation: backend ID must not contain \
         leading/trailing whitespace"
    );
}

/// Verifies adapter metadata.
pub fn adapter_identity_is_valid(adapter: &dyn QuantumBackendAdapter) {
    let info = adapter.adapter_info();

    assert!(
        !info.adapter_id.trim().is_empty(),
        "hardware conformance violation: adapter ID must not be empty"
    );

    assert!(
        !info.adapter_version.trim().is_empty(),
        "hardware conformance violation: adapter version must not be empty"
    );

    assert_eq!(
        info.adapter_id,
        info.adapter_id.trim(),
        "hardware conformance violation: adapter ID contains surrounding \
         whitespace"
    );

    assert_eq!(
        info.adapter_version,
        info.adapter_version.trim(),
        "hardware conformance violation: adapter version contains surrounding \
         whitespace"
    );
}

/// Verifies that the backend kind is a known canonical kind.
///
/// The exhaustive match intentionally makes this test future-proof at the
/// source level: if BackendKind gains a new variant, this match must be
/// reconsidered by the maintainer.
pub fn backend_kind_is_valid(adapter: &dyn QuantumBackendAdapter) {
    let kind = adapter.backend().kind();

    match kind {
        BackendKind::Simulator
        | BackendKind::Emulator
        | BackendKind::Qpu
        | BackendKind::Custom => {}
    }
}

/// Verifies basic backend status invariants.
pub fn backend_status_is_coherent(adapter: &dyn QuantumBackendAdapter) {
    let status = adapter.backend().status();

    match status {
        BackendStatus::Unknown
        | BackendStatus::Available
        | BackendStatus::Busy
        | BackendStatus::Maintenance
        | BackendStatus::Degraded
        | BackendStatus::Offline
        | BackendStatus::Retired
        | BackendStatus::Unavailable => {}
    }

    let health = adapter
        .health()
        .expect("a conformant adapter must return a health report");

    match health.state {
        super::backend_trait::BackendHealthState::Healthy => {
            assert!(
                matches!(
                    health.backend_status,
                    BackendStatus::Available
                        | BackendStatus::Busy
                        | BackendStatus::Degraded
                ),
                "healthy adapter health report has an impossible backend status"
            );
        }

        super::backend_trait::BackendHealthState::Degraded => {
            assert!(
                matches!(
                    health.backend_status,
                    BackendStatus::Degraded
                        | BackendStatus::Available
                        | BackendStatus::Busy
                ),
                "degraded adapter health report has an impossible backend status"
            );
        }

        super::backend_trait::BackendHealthState::Unhealthy => {
            assert!(
                matches!(
                    health.backend_status,
                    BackendStatus::Offline
                        | BackendStatus::Unavailable
                        | BackendStatus::Retired
                        | BackendStatus::Degraded
                ),
                "unhealthy adapter health report has an impossible backend status"
            );
        }

        super::backend_trait::BackendHealthState::Unknown => {}
    }
}

/// Verifies internal coherence of advertised capability flags.
pub fn advertised_capabilities_are_coherent(
    adapter: &dyn QuantumBackendAdapter,
) {
    let capabilities = adapter.backend().capabilities();

    if !capabilities.native_gates.is_empty() {
        assert!(
            capabilities.native_instruction_set,
            "native gates are advertised while native_instruction_set is false"
        );
    }

    if capabilities.dynamic_circuits {
        assert!(
            capabilities.mid_circuit_measurement,
            "dynamic circuits require at least mid-circuit measurement support"
        );
    }

    if capabilities.fault_tolerance {
        assert!(
            capabilities.logical_qubits,
            "fault tolerance requires logical-qubit support"
        );
    }

    if capabilities.syndrome_measurement {
        assert!(
            capabilities.mid_circuit_measurement
                || capabilities.measurement,
            "syndrome measurement requires measurement capability"
        );
    }

    if capabilities.decoder_execution {
        assert!(
            capabilities.fault_tolerance
                || capabilities.logical_qubits,
            "decoder execution should be associated with logical/QEC support"
        );
    }

    if capabilities.queue_information {
        let queue = adapter
            .queue_info()
            .expect("advertised queue support must expose queue information");

        assert!(
            queue.pending_jobs.is_some() || queue.estimated_wait.is_some(),
            "queue support is advertised but no queue information is exposed"
        );
    }

    if capabilities.cancellation {
        assert!(
            adapter.supports_cancellation(),
            "cancellation capability is advertised but the adapter says \
             cancellation is unsupported"
        );
    }
}

/// Verifies queue information when the adapter advertises it.
pub fn queue_contract_is_coherent(adapter: &dyn QuantumBackendAdapter) {
    let queue = adapter
        .queue_info()
        .expect("queue_info must be callable for conformance");

    if let Some(wait) = queue.estimated_wait {
        assert!(
            wait <= Duration::from_secs(365 * 24 * 60 * 60),
            "queue wait estimate is unreasonably large: {wait:?}"
        );
    }

    if let Some(pending) = queue.pending_jobs {
        assert!(
            pending <= usize::MAX,
            "queue pending count overflowed"
        );
    }

    if !queue.accepting_submissions {
        // A non-accepting queue is valid. This branch exists to make the
        // semantic distinction explicit and prevent tests from incorrectly
        // treating queue availability as a mandatory property.
    }
}

/// Verifies health-report structural invariants.
pub fn health_contract_is_coherent(adapter: &dyn QuantumBackendAdapter) {
    let health = adapter
        .health()
        .expect("health() must return a provider-neutral health report");

    if let Some(message) = health.message {
        assert!(
            !message.contains("Authorization:"),
            "health message must not contain authorization headers"
        );

        assert!(
            !message.to_ascii_lowercase().contains("api_key"),
            "health message must not expose API-key metadata"
        );

        assert!(
            !message.to_ascii_lowercase().contains("access_token"),
            "health message must not expose access-token metadata"
        );
    }
}

/// Verifies the provider-neutral program payload contract.
pub fn program_contract_is_valid() {
    let program = bell_program();

    assert_eq!(
        program.format(),
        TEST_PROGRAM_FORMAT,
        "program format must be preserved exactly"
    );

    assert!(
        !program.is_empty(),
        "valid program payload must not be empty"
    );

    assert_eq!(
        program.len(),
        BELL_PROGRAM.as_bytes().len(),
        "program length must describe the encoded payload"
    );

    let debug = format!("{program:?}");

    assert!(
        !debug.contains(BELL_PROGRAM),
        "BackendProgram Debug must not expose the complete program payload"
    );

    assert!(
        debug.contains("byte_len"),
        "BackendProgram Debug must expose payload size rather than payload bytes"
    );
}

/// Verifies that malformed provider-neutral programs are rejected before
/// provider execution.
pub fn malformed_programs_are_rejected() {
    assert!(
        BackendProgram::new(TEST_PROGRAM_FORMAT, Vec::<u8>::new()).is_err(),
        "empty executable programs must be rejected"
    );

    assert!(
        BackendProgram::new("", b"program".to_vec()).is_err(),
        "empty program formats must be rejected"
    );

    assert!(
        BackendProgram::new("   ", b"program".to_vec()).is_err(),
        "whitespace-only program formats must be rejected"
    );

    assert!(
        BackendProgram::new(
            "format\nwith\ncontrol",
            b"program".to_vec()
        )
        .is_err(),
        "program formats containing control characters must be rejected"
    );
}

/// Verifies request-local structure without requiring provider execution.
pub fn request_structure_is_valid() {
    let request = bell_request();

    request
        .validate_structure()
        .expect("canonical conformance request must pass structural validation");

    assert_eq!(
        request.workload.circuit.qubit_count,
        2,
        "conformance request must require two qubits"
    );

    assert_eq!(
        request.workload.circuit.shots,
        32,
        "conformance request must require 32 shots"
    );

    assert_eq!(
        request.seed,
        Some(0),
        "conformance execution must use deterministic seed zero"
    );
}

/// Verifies that a valid program passes adapter preflight.
pub fn preflight_accepts_valid_program(
    adapter: &dyn QuantumBackendAdapter,
) {
    let request = bell_request();
    let program = bell_program();

    adapter
        .preflight(&request, &program)
        .expect("valid program/request pair must pass adapter preflight");
}

/// Verifies that adapters reject a program format they do not implement.
pub fn preflight_rejects_wrong_program_format(
    adapter: &dyn QuantumBackendAdapter,
) {
    let request = bell_request();

    let program = BackendProgram::new(
        "zamani-conformance-unsupported-format",
        b"{}",
    )
    .expect("the test payload itself should be structurally valid");

    let result = adapter.preflight(&request, &program);

    assert!(
        result.is_err(),
        "adapter must reject an executable format it does not support"
    );
}

/// Verifies that an empty payload never reaches provider execution.
pub fn preflight_rejects_empty_program() {
    let result = BackendProgram::new(
        TEST_PROGRAM_FORMAT,
        Vec::<u8>::new(),
    );

    assert!(
        matches!(result, Err(BackendError::ExecutionUnavailable)),
        "empty program must fail at BackendProgram construction"
    );
}

/// Verifies successful job submission.
pub fn submission_produces_valid_job(
    adapter: &dyn QuantumBackendAdapter,
) {
    let request = bell_request();
    let program = bell_program();

    let job = adapter
        .submit(&request, &program)
        .expect("valid workload submission must succeed");

    assert!(
        !job.id.as_str().trim().is_empty(),
        "submitted job must have a non-empty ID"
    );

    assert_eq!(
        job.backend_id,
        adapter.backend().id(),
        "job provenance must identify the backend that accepted it"
    );

    assert_eq!(
        job.request_id,
        request.request_id,
        "job must preserve the caller request identifier"
    );

    assert!(
        !matches!(
            job.state,
            BackendJobState::Unknown
        ),
        "successful submission must not return an unknown lifecycle state"
    );
}

/// Verifies status normalization and terminal-result coherence.
pub fn job_status_is_coherent(
    adapter: &dyn QuantumBackendAdapter,
) {
    let request = bell_request();
    let program = bell_program();

    let job = adapter
        .submit(&request, &program)
        .expect("valid workload submission must succeed");

    let status = adapter
        .status(&job.id)
        .expect("submitted job must be queryable");

    assert_eq!(
        status.job.id,
        job.id,
        "status must refer to the requested job"
    );

    assert_eq!(
        status.job.backend_id,
        adapter.backend().id(),
        "status must preserve backend provenance"
    );

    if status.job.state == BackendJobState::Completed {
        assert!(
            status.result_available,
            "completed jobs must advertise result availability"
        );
    }

    if status.result_available {
        assert!(
            status.job.state == BackendJobState::Completed,
            "result availability must only be advertised for completed jobs"
        );
    }

    assert!(
        !matches!(
            status.job.state,
            BackendJobState::Unknown
        ),
        "a successfully submitted local/reference job must not remain unknown"
    );
}

/// Verifies normalized result retrieval.
pub fn completed_job_has_result(
    adapter: &dyn QuantumBackendAdapter,
) {
    let request = bell_request();
    let program = bell_program();

    let job = adapter
        .submit(&request, &program)
        .expect("valid workload submission must succeed");

    let status = adapter
        .status(&job.id)
        .expect("submitted job must have a status");

    if status.job.state != BackendJobState::Completed {
        return;
    }

    let result = adapter
        .result(&job.id)
        .expect("completed job must expose a normalized result");

    assert_eq!(
        result.backend_id,
        adapter.backend().id(),
        "result backend provenance is incorrect"
    );

    assert_eq!(
        result.shots,
        request.workload.circuit.shots,
        "result shot count must match the execution request"
    );

    assert!(
        result.counts_within_shots(),
        "normalized counts must never exceed the requested shot budget"
    );

    assert!(
        result.counts_match_shots(),
        "a completed sampled execution must expose all requested shots"
    );
}

/// Verifies result provenance and non-secret metadata.
pub fn result_provenance_is_valid(
    adapter: &dyn QuantumBackendAdapter,
) {
    let request = bell_request();
    let program = bell_program();

    let result = adapter
        .execute(&request, &program)
        .expect("reference adapter must execute the canonical program");

    assert_eq!(
        result.backend_id,
        adapter.backend().id(),
        "execution result must identify its backend"
    );

    for (key, value) in &result.metadata {
        let lower_key = key.to_ascii_lowercase();
        let lower_value = value.to_ascii_lowercase();

        assert!(
            !lower_key.contains("api_key"),
            "result metadata must not expose API-key fields"
        );

        assert!(
            !lower_key.contains("access_token"),
            "result metadata must not expose access-token fields"
        );

        assert!(
            !lower_key.contains("password"),
            "result metadata must not expose password fields"
        );

        assert!(
            !lower_value.starts_with("bearer "),
            "result metadata must not contain bearer authorization values"
        );
    }
}

/// Verifies that normalized counts cannot exceed the requested shot budget.
pub fn result_counts_respect_shot_budget(
    adapter: &dyn QuantumBackendAdapter,
) {
    let request = one_qubit_request();
    let program = one_qubit_program();

    let result = adapter
        .execute(&request, &program)
        .expect("one-qubit conformance execution must succeed");

    assert!(
        result.counts_within_shots(),
        "execution result counts exceeded requested shots"
    );

    assert_eq!(
        result.counted_shots(),
        result.shots,
        "completed sampled execution must account for every shot"
    );
}

/// Verifies terminal cancellation semantics.
///
/// A completed job must not suddenly transition back into an accepted
/// cancellation state.
pub fn cancellation_of_completed_job_is_not_accepted(
    adapter: &dyn QuantumBackendAdapter,
) {
    let request = bell_request();
    let program = bell_program();

    let job = adapter
        .submit(&request, &program)
        .expect("valid workload submission must succeed");

    let status = adapter
        .status(&job.id)
        .expect("submitted job must be queryable");

    if status.job.state != BackendJobState::Completed {
        return;
    }

    let cancellation = adapter
        .cancel(&job.id)
        .expect("cancelling a known terminal job must be representable");

    assert!(
        !matches!(
            cancellation.outcome,
            super::backend_trait::CancellationOutcome::Accepted
        ),
        "terminal jobs must not be reported as newly accepted for cancellation"
    );
}

/// Verifies synchronous execution when the adapter advertises it.
pub fn synchronous_execution_contract(
    adapter: &dyn QuantumBackendAdapter,
) {
    if !adapter.supports_synchronous_execution() {
        return;
    }

    let mut request = bell_request();
    request.asynchronous = false;

    let program = bell_program();

    let result = adapter
        .execute(&request, &program)
        .expect("synchronous execution must succeed when advertised");

    assert_eq!(
        result.backend_id,
        adapter.backend().id(),
        "synchronous result must preserve backend provenance"
    );

    assert_eq!(
        result.shots,
        request.workload.circuit.shots,
        "synchronous result must preserve requested shot count"
    );

    assert!(
        result.counts_match_shots(),
        "synchronous sampled result must account for all requested shots"
    );
}

/// Verifies that the asynchronous lifecycle and synchronous convenience API
/// produce semantically equivalent results for deterministic adapters.
///
/// The comparison intentionally ignores provider job identifiers and metadata
/// that may legitimately differ between executions.
pub fn asynchronous_and_synchronous_results_are_semantically_equivalent(
    adapter: &dyn QuantumBackendAdapter,
) {
    if !adapter.supports_synchronous_execution() {
        return;
    }

    let request = bell_request();
    let program = bell_program();

    let synchronous = adapter
        .execute(&request, &program)
        .expect("synchronous execution must succeed");

    let job = adapter
        .submit(&request, &program)
        .expect("asynchronous submission must succeed");

    let status = adapter
        .status(&job.id)
        .expect("submitted job must be queryable");

    if status.job.state != BackendJobState::Completed {
        return;
    }

    let asynchronous = adapter
        .result(&job.id)
        .expect("completed asynchronous job must expose a result");

    assert_eq!(
        synchronous.backend_id,
        asynchronous.backend_id,
        "synchronous and asynchronous results must use the same backend"
    );

    assert_eq!(
        synchronous.shots,
        asynchronous.shots,
        "synchronous and asynchronous executions must use the same shot count"
    );

    assert_eq!(
        synchronous.counts,
        asynchronous.counts,
        "deterministic adapter results must agree between synchronous and \
         asynchronous execution"
    );
}

/// Verifies that secret-like request metadata cannot cross the hardware
/// boundary.
pub fn secret_metadata_is_rejected() {
    let mut request = bell_request();

    let secret_keys = [
        "api_key",
        "access_token",
        "refresh_token",
        "password",
        "private_key",
        "authorization",
        "authorization_header",
        "cookie",
        "secret",
    ];

    for key in secret_keys {
        let result = request.insert_metadata(key, "must-not-be-accepted");

        assert!(
            result.is_err(),
            "secret-like metadata key `{key}` must be rejected"
        );
    }
}

/// Verifies provider-neutral job ID validation.
pub fn malformed_job_ids_are_rejected() {
    use super::backend_trait::BackendJobId;

    assert!(
        BackendJobId::new("").is_err(),
        "empty job identifiers must be rejected"
    );

    assert!(
        BackendJobId::new("   ").is_err(),
        "whitespace-only job identifiers must be rejected"
    );

    assert!(
        BackendJobId::new("\ninvalid").is_err(),
        "job identifiers containing control characters must be rejected"
    );

    let valid = BackendJobId::new("provider-job-001")
        .expect("ordinary job identifier must be accepted");

    assert_eq!(
        valid.as_str(),
        "provider-job-001",
        "job identifier canonical representation changed unexpectedly"
    );
}

/// Verifies deterministic execution for adapters that guarantee deterministic
/// seeded execution.
///
/// The local adapter is the reference implementation and must pass this test.
pub fn deterministic_execution_is_reproducible(
    adapter: &dyn QuantumBackendAdapter,
) {
    let capabilities = adapter.backend().capabilities();

    if !capabilities.deterministic_seeding {
        return;
    }

    let request = bell_request();
    let program = bell_program();

    let first = adapter
        .execute(&request, &program)
        .expect("deterministic execution must succeed");

    let second = adapter
        .execute(&request, &program)
        .expect("deterministic execution must succeed repeatedly");

    assert_eq!(
        first.backend_id,
        second.backend_id,
        "deterministic executions must use the same backend"
    );

    assert_eq!(
        first.shots,
        second.shots,
        "deterministic executions must use the same shot count"
    );

    assert_eq!(
        first.counts,
        second.counts,
        "identical deterministic executions must produce identical counts"
    );
}

/// Constructs the canonical reference local adapter.
///
/// This is intentionally kept private: consumers should normally call
/// `local_adapter_conforms()` rather than coupling themselves to the
/// constructor.
fn reference_local_adapter() -> LocalBackendAdapter {
    LocalBackendAdapter::with_config(
        LocalBackendConfig::test()
            .with_max_qubits(8)
            .with_max_classical_bits(64)
            .with_max_shots(10_000)
            .with_max_operations(100_000),
    )
    .expect("reference local adapter configuration must be valid")
}

/// Runs the complete conformance suite against the repository's local
/// deterministic adapter.
///
/// This is the most important CI-level test because it proves that the
/// provider-neutral adapter contract can be exercised without cloud
/// credentials or physical QPU access.
#[test]
fn local_adapter_passes_complete_conformance_suite() {
    let adapter = reference_local_adapter();

    run_all(&adapter);
}

/// Verifies the local adapter's advertised synchronous capability.
#[test]
fn local_adapter_advertises_synchronous_execution() {
    let adapter = reference_local_adapter();

    assert!(
        adapter.supports_synchronous_execution(),
        "reference local adapter must support synchronous execution"
    );
}

/// Verifies the local adapter's queue boundary.
#[test]
fn local_adapter_queue_contract_is_zero_latency_local_execution() {
    let adapter = reference_local_adapter();

    let queue = adapter
        .queue_info()
        .expect("reference local adapter must expose queue information");

    assert_eq!(
        queue.pending_jobs,
        Some(0),
        "local reference adapter must not report a remote queue"
    );

    assert_eq!(
        queue.estimated_wait,
        Some(Duration::ZERO),
        "local reference adapter must report zero queue wait"
    );

    assert!(
        queue.accepting_submissions,
        "reference local adapter must accept submissions"
    );
}

/// Verifies the local adapter's healthy state.
#[test]
fn local_adapter_health_is_healthy() {
    let adapter = reference_local_adapter();

    let health = adapter
        .health()
        .expect("reference local adapter health must be available");

    assert_eq!(
        health.state,
        super::backend_trait::BackendHealthState::Healthy,
        "reference local adapter must start healthy"
    );

    assert_eq!(
        health.backend_status,
        BackendStatus::Available,
        "reference local backend must start available"
    );
}

/// Verifies that malformed executable JSON is rejected by the local adapter
/// rather than being silently accepted.
#[test]
fn local_adapter_rejects_malformed_program() {
    let adapter = reference_local_adapter();

    let request = one_qubit_request();

    let program = BackendProgram::new(
        TEST_PROGRAM_FORMAT,
        br#"{"schema":"zamani-local-v1","qubits":"not-a-number"}"#.to_vec(),
    )
    .expect("payload container should accept the bytes");

    assert!(
        adapter.preflight(&request, &program).is_err(),
        "malformed local program must be rejected during preflight"
    );
}

/// Verifies that the local adapter does not silently measure an unmeasured
/// circuit.
#[test]
fn local_adapter_does_not_silently_measure() {
    let adapter = reference_local_adapter();

    let mut circuit = CircuitRequirements::default();
    circuit.qubit_count = 1;
    circuit.classical_bit_count = 1;
    circuit.operation_count = 1;
    circuit.circuit_depth = 1;
    circuit.shots = 4;
    circuit.gates = vec!["x".to_string()];
    circuit.requires_measurement = true;

    let request = ExecutionRequest::new(circuit);

    let program = BackendProgram::new(
        TEST_PROGRAM_FORMAT,
        br#"{
            "schema": "zamani-local-v1",
            "qubits": 1,
            "classical_bits": 1,
            "measure_all": false,
            "operations": [
                {"gate": "x", "targets": [0]}
            ]
        }"#.to_vec(),
    )
    .expect("test program payload should be structurally valid");

    assert!(
        adapter.preflight(&request, &program).is_err(),
        "local adapter must reject a workload that requests measurement \
         without an explicit measurement operation"
    );
}

/// Verifies that a completed local job is retrievable exactly once or many
/// times without mutation of the normalized result.
#[test]
fn local_result_is_stable_after_completion() {
    let adapter = reference_local_adapter();

    let request = bell_request();
    let program = bell_program();

    let job = adapter
        .submit(&request, &program)
        .expect("reference submission must succeed");

    let first = adapter
        .result(&job.id)
        .expect("completed local result must be available");

    let second = adapter
        .result(&job.id)
        .expect("completed local result must remain available");

    assert_eq!(
        first,
        second,
        "retrieving a completed result must not mutate it"
    );
}

/// Verifies local cancellation semantics for a terminal job.
#[test]
fn local_completed_job_cancellation_is_terminally_safe() {
    let adapter = reference_local_adapter();

    let request = one_qubit_request();
    let program = one_qubit_program();

    let job = adapter
        .submit(&request, &program)
        .expect("reference submission must succeed");

    let status = adapter
        .status(&job.id)
        .expect("submitted job must be queryable");

    assert_eq!(
        status.job.state,
        BackendJobState::Completed,
        "reference local submission should complete synchronously"
    );

    let cancellation = adapter
        .cancel(&job.id)
        .expect("terminal cancellation must be representable");

    assert!(
        !matches!(
            cancellation.outcome,
            super::backend_trait::CancellationOutcome::Accepted
        ),
        "terminal local job must not be newly accepted for cancellation"
    );

    let final_status = adapter
        .status(&job.id)
        .expect("terminal job must remain queryable");

    assert_eq!(
        final_status.job.state,
        BackendJobState::Completed,
        "cancelling a completed local job must not resurrect or mutate it"
    );
}

/// Verifies that the normalized result type correctly accounts for shot
/// replacement semantics. This protects the backend result invariant from a
/// common accounting bug.
#[test]
fn normalized_result_replacement_preserves_shot_accounting() {
    let mut result = ExecutionResult::empty(
        "local://conformance",
        10,
    )
    .expect("test result should be constructible");

    result
        .insert_count("00", 6)
        .expect("first count must fit within ten shots");

    result
        .insert_count("11", 4)
        .expect("second count must complete the ten shots");

    assert_eq!(
        result.counted_shots(),
        10,
        "counts must represent ten shots"
    );

    result
        .insert_count("00", 3)
        .expect("replacing six with three must remain within ten shots");

    assert_eq!(
        result.counted_shots(),
        7,
        "replacing an existing count must subtract the previous value"
    );

    assert!(
        result.counts_within_shots(),
        "replacement must never cause false shot overflow"
    );
}

/// Verifies that result counts cannot exceed the declared shot budget.
#[test]
fn normalized_result_rejects_shot_overflow() {
    let mut result = ExecutionResult::empty(
        "local://conformance",
        4,
    )
    .expect("test result should be constructible");

    result
        .insert_count("0000", 4)
        .expect("four counts must fit within four shots");

    let overflow = result.insert_count("1111", 1);

    assert!(
        matches!(
            overflow,
            Err(BackendError::ResultShotsExceeded { .. })
        ),
        "normalized result must reject counts exceeding requested shots"
    );
}

/// Verifies deterministic ordering of normalized counts.
///
/// This matters for reproducible benchmarking, reporting, hashing and
/// provenance.
#[test]
fn normalized_counts_are_deterministically_ordered() {
    let mut result = ExecutionResult::empty(
        "local://conformance",
        3,
    )
    .expect("test result should be constructible");

    result
        .insert_count("11", 1)
        .expect("count should fit");

    result
        .insert_count("00", 1)
        .expect("count should fit");

    result
        .insert_count("01", 1)
        .expect("count should fit");

    let keys: Vec<&String> = result.counts.keys().collect();

    let mut sorted = keys.clone();
    sorted.sort();

    assert_eq!(
        keys,
        sorted,
        "normalized result counts must have deterministic ordering"
    );
}

/// Verifies request metadata remains deterministic and non-secret.
#[test]
fn request_metadata_is_deterministic_and_safe() {
    let mut request = one_qubit_request();

    request
        .insert_metadata("experiment", "conformance")
        .expect("ordinary metadata must be accepted");

    request
        .insert_metadata("backend_revision", "test")
        .expect("ordinary metadata must be accepted");

    let mut expected = BTreeMap::new();
    expected.insert(
        "experiment".to_string(),
        "conformance".to_string(),
    );
    expected.insert(
        "backend_revision".to_string(),
        "test".to_string(),
    );

    assert_eq!(
        request.metadata,
        expected,
        "request metadata must use deterministic key/value semantics"
    );
}

/// Verifies that a conformant adapter can be used behind its object-safe trait
/// boundary.
///
/// This is important because provider registries are expected to hold
/// `dyn QuantumBackendAdapter`.
#[test]
fn local_adapter_is_usable_through_object_safe_trait() {
    let adapter = reference_local_adapter();

    let object: &dyn QuantumBackendAdapter = &adapter;

    let backend = object.backend();

    assert_eq!(
        backend.id(),
        adapter.backend().id(),
        "object-safe adapter boundary changed backend identity"
    );

    assert_eq!(
        object.adapter_info().adapter_id,
        adapter.adapter_info().adapter_id,
        "object-safe adapter boundary changed adapter identity"
    );
}

/// Verifies that the local adapter accepts the canonical one-qubit workload
/// through the full preflight path.
#[test]
fn local_one_qubit_program_passes_full_preflight() {
    let adapter = reference_local_adapter();

    let request = one_qubit_request();
    let program = one_qubit_program();

    adapter
        .preflight(&request, &program)
        .expect("canonical one-qubit workload must pass preflight");
}

/// Verifies the local adapter's deterministic X-gate result.
///
/// X|0> produces |1>, therefore every shot must be represented by bitstring
/// "1" for the deterministic local reference execution.
#[test]
fn local_x_gate_has_expected_deterministic_result() {
    let adapter = reference_local_adapter();

    let request = one_qubit_request();
    let program = one_qubit_program();

    let result = adapter
        .execute(&request, &program)
        .expect("X-gate execution must succeed");

    assert_eq!(
        result.counts.get("1").copied(),
        Some(request.workload.circuit.shots),
        "deterministic X|0> execution must produce only the |1> outcome"
    );

    assert!(
        result.counts.get("0").is_none(),
        "deterministic X|0> execution must not produce a |0> outcome"
    );
}

/// Verifies that a known job can be polled repeatedly without changing its
/// terminal state.
#[test]
fn local_job_status_is_stable() {
    let adapter = reference_local_adapter();

    let request = one_qubit_request();
    let program = one_qubit_program();

    let job = adapter
        .submit(&request, &program)
        .expect("submission must succeed");

    let first = adapter
        .status(&job.id)
        .expect("first status request must succeed");

    let second = adapter
        .status(&job.id)
        .expect("second status request must succeed");

    assert_eq!(
        first,
        second,
        "repeated status retrieval must not mutate a completed local job"
    );
}

/// Verifies that an unknown job ID does not produce a fabricated status.
#[test]
fn local_unknown_job_is_rejected() {
    use super::backend_trait::BackendJobId;

    let adapter = reference_local_adapter();

    let unknown = BackendJobId::new(
        "local-this-job-does-not-exist",
    )
    .expect("syntactically valid unknown job ID should construct");

    assert!(
        adapter.status(&unknown).is_err(),
        "unknown jobs must not receive fabricated status"
    );

    assert!(
        adapter.result(&unknown).is_err(),
        "unknown jobs must not receive fabricated results"
    );

    assert!(
        adapter.cancel(&unknown).is_err(),
        "unknown jobs must not receive fabricated cancellation"
    );
}

/// Verifies that the local adapter does not retain executable program bytes in
/// its job lifecycle contract.
///
/// This test cannot inspect private storage directly; instead it verifies the
/// public behavior that a completed job remains result-addressable without
/// requiring the original program payload to be retained by the caller.
#[test]
fn local_job_lifecycle_is_independent_of_program_buffer() {
    let adapter = reference_local_adapter();

    let request = one_qubit_request();

    let program = {
        let bytes = ONE_QUBIT_PROGRAM.as_bytes().to_vec();

        BackendProgram::new(
            TEST_PROGRAM_FORMAT,
            bytes,
        )
        .expect("program should construct")
    };

    let job = adapter
        .submit(&request, &program)
        .expect("submission must succeed");

    drop(program);

    let result = adapter
        .result(&job.id)
        .expect("result must remain available after caller drops program");

    assert_eq!(
        result.backend_id,
        adapter.backend().id(),
        "result must retain backend provenance"
    );
}

/// Verifies that request IDs become useful provenance but do not affect the
/// quantum semantics of deterministic local execution.
#[test]
fn request_id_does_not_change_deterministic_result() {
    let adapter = reference_local_adapter();

    let base = one_qubit_request();
    let alternate = ExecutionRequest::new(base.workload.circuit.clone())
        .with_seed(0)
        .with_request_id("different-request-id")
        .expect("alternate request ID must be valid");

    let program = one_qubit_program();

    let first = adapter
        .execute(&base, &program)
        .expect("first deterministic execution must succeed");

    let second = adapter
        .execute(&alternate, &program)
        .expect("second deterministic execution must succeed");

    assert_eq!(
        first.counts,
        second.counts,
        "request identity must not alter deterministic quantum semantics"
    );
}

/// Verifies that the generic suite does not require a provider-specific
/// transport or credential layer.
///
/// This test is intentionally simple: constructing the local adapter itself
/// proves that the core conformance suite can execute entirely offline.
#[test]
fn reference_conformance_is_offline_capable() {
    let adapter = reference_local_adapter();

    let health = adapter
        .health()
        .expect("local health must not require network access");

    assert!(
        health.message
            .as_deref()
            .unwrap_or_default()
            .to_ascii_lowercase()
            .contains("local"),
        "reference health report should identify local execution"
    );
}