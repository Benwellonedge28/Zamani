//! Zamani Quantum Hardware — Compatibility Conformance Tests.
//!
//! Production compatibility tests for:
//!
//! `crate::quantum::hardware`
//!
//! # Responsibility
//!
//! This module verifies that the canonical hardware compatibility boundary
//! correctly determines whether a provider-neutral quantum workload can be
//! accepted by a `QuantumBackend`.
//!
//! The tests cover:
//!
//! - backend availability;
//! - degraded-backend warnings;
//! - unavailable backend rejection;
//! - workload-kind compatibility;
//! - inconsistent workload declarations;
//! - physical-qubit limits;
//! - logical-qubit limits;
//! - circuit-depth limits;
//! - operation-count limits;
//! - shot limits;
//! - classical-resource limits;
//! - required stable capabilities;
//! - experimental-capability handling;
//! - native-instruction requirements;
//! - unsupported instructions;
//! - parameterized-gate capability handling;
//! - arbitrary single-qubit rotation capability handling;
//! - measurement requirements;
//! - reset requirements;
//! - mid-circuit measurement;
//! - classical feed-forward;
//! - dynamic circuits;
//! - pulse workloads;
//! - analog workloads;
//! - annealing workloads;
//! - logical workloads;
//! - fault-tolerant workloads;
//! - deterministic execution requirements;
//! - state-vector result requirements;
//! - density-matrix result requirements;
//! - expectation-value result requirements;
//! - topology availability;
//! - topology resource capacity;
//! - two-qubit connectivity;
//! - invalid qubit references;
//! - self-interactions;
//! - calibration requirements;
//! - deferred calibration-freshness verification;
//! - required native-instruction exposure;
//! - request-level validation;
//! - deterministic diagnostics;
//! - provider neutrality;
//! - compatibility with the execution preflight boundary.
//!
//! # Architectural boundary
//!
//! Compatibility analysis belongs between canonical quantum workload
//! requirements and hardware execution.
//!
//! ```text
//! Zamani source
//!      |
//!      v
//! Quantum Frontend
//!      |
//!      v
//! Zamani Quantum IR
//!      |
//!      v
//! optimization / QEC
//!      |
//!      v
//! workload requirements
//!      |
//!      v
//! hardware compatibility
//!      |
//!      +-------------------+
//!      |                   |
//!      v                   v
//! routing             scheduling
//!      |                   |
//!      +---------+---------+
//!                |
//!                v
//!        QuantumBackend
//!                |
//!                v
//!          provider adapter
//! ```
//!
//! The tests intentionally verify the hardware boundary without depending on
//! provider adapters.
//!
//! # Ownership
//!
//! This module tests compatibility. It does NOT implement compatibility
//! semantics itself.
//!
//! The authoritative implementation remains in:
//!
//! `crate::quantum::hardware::backend`
//!
//! In particular, this test module consumes:
//!
//! - `BackendCapabilities`;
//! - `BackendError`;
//! - `BackendKind`;
//! - `BackendLimits`;
//! - `BackendMetadata`;
//! - `BackendStatus`;
//! - `CircuitRequirements`;
//! - `ExecutionRequest`;
//! - `QuantumBackend`;
//! - `QuantumWorkloadKind`;
//! - `WorkloadRequirements`;
//! - `BackendValidationReport`;
//! - `ValidationDiagnostic`;
//! - `ValidationSeverity`.
//!
//! It must never duplicate those semantics.
//!
//! # Important integration rule
//!
//! This file intentionally tests the public API already exposed by
//! `backend.rs`. It does not reach into private backend implementation
//! details.
//!
//! Therefore:
//!
//! - backend implementation may be refactored;
//! - provider adapters may be added;
//! - provider registries may be added;
//! - execution/job layers may be added;
//! - routing/scheduling may evolve;
//!
//! without changing this test file, provided the public compatibility contract
//! remains valid.
//!
//! # Future compatibility module
//!
//! A future:
//!
//! `crate::quantum::hardware::compatibility`
//!
//! module may provide a richer compatibility API. When introduced, that module
//! must preserve the semantic guarantees tested here:
//!
//! 1. compatibility is deterministic;
//! 2. unsupported capabilities never silently pass;
//! 3. experimental capabilities never silently satisfy stable requirements;
//! 4. resource limits are enforced;
//! 5. topology requirements are enforced;
//! 6. invalid workload requirements are rejected;
//! 7. backend availability is enforced;
//! 8. calibration freshness is never falsely claimed;
//! 9. provider-specific details do not leak into the core compatibility model.
//!
//! The future compatibility module should therefore be tested against this
//! contract rather than changing the meaning of these tests.
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
//! # Determinism
//!
//! These tests deliberately avoid:
//!
//! - network access;
//! - provider APIs;
//! - credentials;
//! - environment variables;
//! - wall-clock dependent assertions;
//! - random values;
//! - external files;
//! - global mutable state.
//!
//! Every test uses fixed data.
//!
//! # Security
//!
//! No real authentication material is used.
//!
//! Metadata tests use synthetic secret-like field names only.
//!
//! # Test fixture policy
//!
//! The representative backend is intentionally a normal gate-model backend.
//! It is NOT declared to support every possible quantum technology.
//!
//! Advanced workload tests construct explicit capability profiles rather than
//! assuming that all hardware supports all workload classes.
//!
//! This distinction is essential for production compatibility testing.
//!
//! # No provider assumptions
//!
//! Nothing in this file assumes IBM, IonQ, AWS Braket, Rigetti, IQM,
//! Quantinuum, QuEra, or any other provider.
//!
//! Provider adapters must satisfy this generic contract independently.
//!
//! # Completion criterion
//!
//! This file is complete when every compatibility invariant exposed by the
//! current canonical backend API has a deterministic regression test here.
//!
//! Adding a provider must not require editing this file.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::collections::{BTreeMap, BTreeSet};

use crate::quantum::hardware::backend::{
    BackendCapabilities,
    BackendError,
    BackendKind,
    BackendLimits,
    BackendMetadata,
    BackendStatus,
    CircuitRequirements,
    ExecutionRequest,
    QuantumBackend,
    QuantumWorkloadKind,
    ValidationSeverity,
    WorkloadRequirements,
    BACKEND_SCHEMA_ID,
    BACKEND_SCHEMA_VERSION,
};

use crate::quantum::hardware::topology::HardwareTopology;

// =============================================================================
// Test fixtures
// =============================================================================

/// Returns the canonical native instruction set used by the compatibility
/// fixtures.
///
/// The set deliberately includes both ordinary gates and measurement/reset
/// instructions so compatibility tests can distinguish instruction support from
/// generic capability support.
fn representative_native_gates() -> BTreeSet<String> {
    [
        "h",
        "x",
        "y",
        "z",
        "sx",
        "rx",
        "ry",
        "rz",
        "cx",
        "cz",
        "measure",
        "reset",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect()
}

/// Creates a representative fully featured gate-model backend.
///
/// This backend intentionally supports dynamic circuits but does not support
/// pulse, analog, annealing, or logical execution. This makes it suitable for
/// both positive and negative compatibility tests.
fn representative_capabilities() -> BackendCapabilities {
    BackendCapabilities {
        measurement: true,
        reset: true,
        mid_circuit_measurement: true,
        classical_control: true,
        dynamic_circuits: true,
        arbitrary_single_qubit_rotations: true,
        parameterized_gates: true,
        three_qubit_operations: false,
        multi_qubit_operations: false,
        parallel_operations: true,
        batch_execution: true,
        streaming_results: false,
        cancellation: true,
        queue_information: true,
        pulse_control: false,
        analog_control: false,
        annealing: false,
        logical_qubits: false,
        fault_tolerance: false,
        syndrome_measurement: false,
        decoder_execution: false,
        deterministic_seeding: true,
        state_vector_results: false,
        density_matrix_results: false,
        expectation_value_results: true,
        readout_mitigation: true,
        error_mitigation: true,
        calibration_data: true,
        timing_information: true,
        topology_information: true,
        native_instruction_set: true,
        native_gates: representative_native_gates(),
        experimental_capabilities: BTreeSet::new(),
    }
}

/// Returns deliberately conservative resource limits.
fn representative_limits() -> BackendLimits {
    BackendLimits {
        max_qubits: 4,
        max_logical_qubits: 2,
        max_circuit_depth: 100,
        max_operations: 1_000,
        max_shots: 10_000,
        max_classical_bits: 64,
        max_concurrent_jobs: 4,
        max_batch_size: 32,
    }
}

/// Creates the physical topology used by all tests.
///
/// The topology is:
///
/// ```text
/// 0 --- 1 --- 2 --- 3
/// ```
///
/// It therefore supports:
///
/// - 0 <-> 1;
/// - 1 <-> 2;
/// - 2 <-> 3;
///
/// but does not directly connect:
///
/// - 0 <-> 2;
/// - 0 <-> 3;
/// - 1 <-> 3.
fn representative_topology() -> HardwareTopology {
    HardwareTopology::linear(4)
        .expect("four-resource linear topology must be valid")
}

/// Creates deterministic provider-neutral backend metadata.
fn representative_metadata() -> BackendMetadata {
    BackendMetadata::new(
        "test.backend",
        "Zamani Compatibility Test Backend",
        "zamani.test",
        "1.0.0",
        BackendKind::Qpu,
    )
    .with_hardware_revision("test-revision-1")
    .with_firmware_version("test-firmware-1")
    .with_api_version("test-api-1")
    .with_region("test-region")
}

/// Constructs the representative backend.
fn representative_backend() -> QuantumBackend {
    QuantumBackend::new(
        representative_metadata(),
        representative_capabilities(),
        representative_limits(),
        representative_topology(),
    )
    .expect("representative backend must satisfy all backend invariants")
}

/// Returns the simplest valid gate-circuit requirements.
fn simple_circuit() -> CircuitRequirements {
    CircuitRequirements {
        qubit_count: 2,
        logical_qubit_count: 0,
        circuit_depth: 4,
        operation_count: 6,
        classical_bit_count: 2,
        shots: 1_000,
        gates: vec![
            "h".to_owned(),
            "cx".to_owned(),
            "measure".to_owned(),
        ],
        two_qubit_edges: vec![(0, 1)],
        requires_measurement: true,
        requires_reset: false,
        requires_mid_circuit_measurement: false,
        requires_classical_control: false,
        requires_dynamic_circuits: false,
        requires_pulse_control: false,
        requires_analog_control: false,
        requires_annealing: false,
        requires_logical_qubits: false,
        requires_fault_tolerance: false,
        requires_deterministic_seed: false,
        requires_state_vector: false,
        requires_density_matrix: false,
        requires_expectation_values: false,
    }
}

/// Creates generalized requirements from the simple circuit.
fn simple_workload() -> WorkloadRequirements {
    WorkloadRequirements::from_circuit(simple_circuit())
}

/// Creates a workload requiring a specific stable capability.
fn workload_requiring_capability(
    capability: &str,
) -> WorkloadRequirements {
    simple_workload().require_capability(capability)
}

/// Creates a workload requiring a specific native instruction.
fn workload_requiring_instruction(
    instruction: &str,
) -> WorkloadRequirements {
    simple_workload().require_instruction(instruction)
}

/// Creates a backend with altered capabilities.
///
/// This helper rebuilds the complete backend rather than mutating private
/// fields. That keeps tests coupled only to the public construction contract.
fn backend_with_capabilities(
    capabilities: BackendCapabilities,
) -> QuantumBackend {
    QuantumBackend::new(
        representative_metadata(),
        capabilities,
        representative_limits(),
        representative_topology(),
    )
    .expect("modified capability fixture must remain structurally valid")
}

/// Creates a backend with custom limits.
fn backend_with_limits(limits: BackendLimits) -> QuantumBackend {
    QuantumBackend::new(
        representative_metadata(),
        representative_capabilities(),
        limits,
        representative_topology(),
    )
    .expect("modified limits fixture must remain structurally valid")
}

/// Creates a backend with custom status.
fn backend_with_status(status: BackendStatus) -> QuantumBackend {
    let mut metadata = representative_metadata();
    metadata.set_status(status);

    QuantumBackend::new(
        metadata,
        representative_capabilities(),
        representative_limits(),
        representative_topology(),
    )
    .expect("modified status fixture must remain structurally valid")
}

/// Creates a backend with a custom topology.
fn backend_with_topology(
    topology: HardwareTopology,
) -> QuantumBackend {
    QuantumBackend::new(
        representative_metadata(),
        representative_capabilities(),
        representative_limits(),
        topology,
    )
    .expect("modified topology fixture must remain structurally valid")
}

/// Extracts diagnostic codes in deterministic order.
fn diagnostic_codes(
    backend: &QuantumBackend,
    workload: &WorkloadRequirements,
) -> Vec<&'static str> {
    backend
        .validation_report(workload)
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.code)
        .collect()
}

// =============================================================================
// Schema and public contract
// =============================================================================

#[test]
fn compatibility_tests_target_the_canonical_backend_schema() {
    assert_eq!(
        BACKEND_SCHEMA_ID,
        "zamani.quantum.hardware.backend"
    );

    assert!(
        BACKEND_SCHEMA_VERSION >= 1,
        "backend schema version must always be positive"
    );
}

#[test]
fn representative_backend_is_valid() {
    let backend = representative_backend();

    assert_eq!(backend.id(), "test.backend");
    assert_eq!(backend.provider(), "zamani.test");
    assert_eq!(backend.kind(), BackendKind::Qpu);
    assert_eq!(backend.status(), BackendStatus::Available);
    assert_eq!(backend.qubit_count(), 4);
    assert_eq!(backend.coupling_count(), 3);
}

// =============================================================================
// Basic compatibility
// =============================================================================

#[test]
fn simple_valid_gate_circuit_is_compatible() {
    let backend = representative_backend();
    let workload = simple_workload();

    let report = backend.validation_report(&workload);

    assert!(
        report.valid,
        "simple workload must be compatible: {:?}",
        report.diagnostics
    );
    assert!(report.errors().next().is_none());
}

#[test]
fn validate_returns_ok_for_compatible_workload() {
    let backend = representative_backend();
    let workload = simple_workload();

    assert_eq!(backend.validate(&workload), Ok(()));
}

#[test]
fn validate_circuit_accepts_compatible_gate_model_circuit() {
    let backend = representative_backend();
    let circuit = simple_circuit();

    assert_eq!(backend.validate_circuit(&circuit), Ok(()));
}

#[test]
fn compatibility_report_identifies_backend() {
    let backend = representative_backend();
    let report = backend.validation_report(&simple_workload());

    assert_eq!(report.backend_id, backend.id());
}

// =============================================================================
// Backend status
// =============================================================================

#[test]
fn available_backend_is_compatible() {
    let backend = backend_with_status(BackendStatus::Available);

    assert!(backend.validation_report(&simple_workload()).valid);
}

#[test]
fn degraded_backend_is_compatible_with_warning() {
    let backend = backend_with_status(BackendStatus::Degraded);
    let report = backend.validation_report(&simple_workload());

    assert!(
        report.valid,
        "degraded status is non-blocking by contract"
    );

    assert!(
        report.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == "BACKEND_DEGRADED"
                && diagnostic.severity == ValidationSeverity::Warning
        }),
        "degraded backend must emit a deterministic warning"
    );
}

#[test]
fn busy_backend_is_incompatible() {
    let backend = backend_with_status(BackendStatus::Busy);
    let report = backend.validation_report(&simple_workload());

    assert!(!report.valid);
    assert!(
        report
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code == "BACKEND_BUSY")
    );

    assert!(matches!(
        backend.validate(&simple_workload()),
        Err(BackendError::BackendUnavailable {
            status: BackendStatus::Busy,
            ..
        })
    ));
}

#[test]
fn unknown_backend_status_is_incompatible() {
    let backend = backend_with_status(BackendStatus::Unknown);
    let report = backend.validation_report(&simple_workload());

    assert!(!report.valid);
    assert!(
        report
            .diagnostics
            .iter()
            .any(|diagnostic| {
                diagnostic.code == "BACKEND_STATUS_UNKNOWN"
            })
    );
}

#[test]
fn maintenance_backend_is_incompatible() {
    let backend = backend_with_status(BackendStatus::Maintenance);

    assert!(!backend.validation_report(&simple_workload()).valid);
}

#[test]
fn offline_backend_is_incompatible() {
    let backend = backend_with_status(BackendStatus::Offline);

    assert!(!backend.validation_report(&simple_workload()).valid);
}

#[test]
fn retired_backend_is_incompatible() {
    let backend = backend_with_status(BackendStatus::Retired);

    assert!(!backend.validation_report(&simple_workload()).valid);
}

#[test]
fn unavailable_backend_is_incompatible() {
    let backend = backend_with_status(BackendStatus::Unavailable);

    assert!(!backend.validation_report(&simple_workload()).valid);
}

// =============================================================================
// Workload kind compatibility
// =============================================================================

#[test]
fn gate_circuit_infers_gate_circuit_kind() {
    let circuit = simple_circuit();

    assert_eq!(
        circuit.inferred_kind(),
        QuantumWorkloadKind::GateCircuit
    );
}

#[test]
fn dynamic_features_infer_dynamic_circuit_kind() {
    let mut circuit = simple_circuit();
    circuit.requires_dynamic_circuits = true;

    assert_eq!(
        circuit.inferred_kind(),
        QuantumWorkloadKind::DynamicCircuit
    );
}

#[test]
fn mid_circuit_measurement_infers_dynamic_circuit_kind() {
    let mut circuit = simple_circuit();
    circuit.requires_mid_circuit_measurement = true;

    assert_eq!(
        circuit.inferred_kind(),
        QuantumWorkloadKind::DynamicCircuit
    );
}

#[test]
fn classical_control_infers_dynamic_circuit_kind() {
    let mut circuit = simple_circuit();
    circuit.requires_classical_control = true;

    assert_eq!(
        circuit.inferred_kind(),
        QuantumWorkloadKind::DynamicCircuit
    );
}

#[test]
fn pulse_requirement_infers_pulse_workload() {
    let mut circuit = simple_circuit();
    circuit.requires_pulse_control = true;

    assert_eq!(
        circuit.inferred_kind(),
        QuantumWorkloadKind::PulseProgram
    );
}

#[test]
fn analog_requirement_infers_analog_workload() {
    let mut circuit = simple_circuit();
    circuit.requires_analog_control = true;

    assert_eq!(
        circuit.inferred_kind(),
        QuantumWorkloadKind::AnalogProgram
    );
}

#[test]
fn annealing_requirement_infers_annealing_workload() {
    let mut circuit = simple_circuit();
    circuit.requires_annealing = true;

    assert_eq!(
        circuit.inferred_kind(),
        QuantumWorkloadKind::AnnealingProblem
    );
}

#[test]
fn logical_requirement_infers_logical_workload() {
    let mut circuit = simple_circuit();
    circuit.requires_logical_qubits = true;

    assert_eq!(
        circuit.inferred_kind(),
        QuantumWorkloadKind::LogicalProgram
    );
}

#[test]
fn fault_tolerance_requirement_infers_logical_workload() {
    let mut circuit = simple_circuit();
    circuit.requires_fault_tolerance = true;

    assert_eq!(
        circuit.inferred_kind(),
        QuantumWorkloadKind::LogicalProgram
    );
}

#[test]
fn mismatched_declared_workload_kind_is_rejected() {
    let mut workload = simple_workload();
    workload.kind = QuantumWorkloadKind::PulseProgram;

    let report = representative_backend().validation_report(&workload);

    assert!(!report.valid);

    assert!(
        report
            .diagnostics
            .iter()
            .any(|diagnostic| {
                diagnostic.code == "WORKLOAD_KIND_MISMATCH"
            })
    );

    assert!(matches!(
        report.first_error(),
        Some(BackendError::InconsistentWorkloadKind {
            declared: QuantumWorkloadKind::PulseProgram,
            inferred: QuantumWorkloadKind::GateCircuit,
        })
    ));
}

#[test]
fn custom_workload_kind_is_allowed_to_defer_kind_inference() {
    let mut workload = simple_workload();
    workload.kind = QuantumWorkloadKind::Custom;

    let report = representative_backend().validation_report(&workload);

    assert!(
        report.valid,
        "Custom is explicitly allowed to bypass inferred-kind equality"
    );
}

// =============================================================================
// Physical resource compatibility
// =============================================================================

#[test]
fn zero_qubits_are_rejected() {
    let mut circuit = simple_circuit();
    circuit.qubit_count = 0;

    let workload = WorkloadRequirements::from_circuit(circuit);
    let report = representative_backend().validation_report(&workload);

    assert!(!report.valid);

    assert!(
        report
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code == "ZERO_QUBITS")
    );
}

#[test]
fn qubit_limit_is_enforced() {
    let backend = representative_backend();

    let mut circuit = simple_circuit();
    circuit.qubit_count = 5;

    let workload = WorkloadRequirements::from_circuit(circuit);
    let report = backend.validation_report(&workload);

    assert!(!report.valid);

    assert!(
        report
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code == "QUBIT_LIMIT")
    );

    assert!(matches!(
        report.first_error(),
        Some(BackendError::QubitLimitExceeded {
            requested: 5,
            maximum: 4,
        })
    ));
}

#[test]
fn logical_qubit_limit_is_enforced() {
    let backend = representative_backend();

    let mut circuit = simple_circuit();
    circuit.logical_qubit_count = 3;
    circuit.requires_logical_qubits = true;

    let workload = WorkloadRequirements::from_circuit(circuit);
    let report = backend.validation_report(&workload);

    assert!(!report.valid);

    assert!(
        report
            .diagnostics
            .iter()
            .any(|diagnostic| {
                diagnostic.code == "LOGICAL_QUBIT_LIMIT"
            })
    );
}

#[test]
fn circuit_depth_limit_is_enforced() {
    let limits = representative_limits().with_max_depth(10);
    let backend = backend_with_limits(limits);

    let mut circuit = simple_circuit();
    circuit.circuit_depth = 11;

    let workload = WorkloadRequirements::from_circuit(circuit);
    let report = backend.validation_report(&workload);

    assert!(!report.valid);

    assert!(
        report
            .diagnostics
            .iter()
            .any(|diagnostic| {
                diagnostic.code == "CIRCUIT_DEPTH_LIMIT"
            })
    );
}

#[test]
fn operation_limit_is_enforced() {
    let limits = representative_limits().with_max_operations(5);
    let backend = backend_with_limits(limits);

    let mut circuit = simple_circuit();
    circuit.operation_count = 6;

    let workload = WorkloadRequirements::from_circuit(circuit);
    let report = backend.validation_report(&workload);

    assert!(!report.valid);

    assert!(
        report
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code == "OPERATION_LIMIT")
    );
}

#[test]
fn shot_limit_is_enforced() {
    let limits = representative_limits().with_max_shots(999);
    let backend = backend_with_limits(limits);

    let mut circuit = simple_circuit();
    circuit.shots = 1_000;

    let workload = WorkloadRequirements::from_circuit(circuit);
    let report = backend.validation_report(&workload);

    assert!(!report.valid);

    assert!(
        report
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code == "SHOT_LIMIT")
    );
}

#[test]
fn zero_shots_are_rejected() {
    let mut circuit = simple_circuit();
    circuit.shots = 0;

    let workload = WorkloadRequirements::from_circuit(circuit);
    let report = representative_backend().validation_report(&workload);

    assert!(!report.valid);

    assert!(
        report
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code == "INVALID_SHOTS")
    );
}

#[test]
fn classical_bit_limit_is_enforced() {
    let limits = representative_limits().with_max_classical_bits(2);
    let backend = backend_with_limits(limits);

    let mut circuit = simple_circuit();
    circuit.classical_bit_count = 3;

    let workload = WorkloadRequirements::from_circuit(circuit);
    let report = backend.validation_report(&workload);

    assert!(
        !report.valid,
        "classical resource requirements must be enforced"
    );
}

// =============================================================================
// Capability compatibility
// =============================================================================

#[test]
fn measurement_requirement_is_enforced() {
    let mut capabilities = representative_capabilities();
    capabilities.measurement = false;

    let backend = backend_with_capabilities(capabilities);

    let mut circuit = simple_circuit();
    circuit.requires_measurement = true;

    let workload = WorkloadRequirements::from_circuit(circuit);
    let report = backend.validation_report(&workload);

    assert!(!report.valid);

    assert!(
        report
            .diagnostics
            .iter()
            .any(|diagnostic| {
                diagnostic.code == "MEASUREMENT_UNSUPPORTED"
            })
    );
}

#[test]
fn reset_requirement_is_enforced() {
    let mut capabilities = representative_capabilities();
    capabilities.reset = false;

    let backend = backend_with_capabilities(capabilities);

    let mut circuit = simple_circuit();
    circuit.requires_reset = true;

    let workload = WorkloadRequirements::from_circuit(circuit);
    let report = backend.validation_report(&workload);

    assert!(!report.valid);

    assert!(
        report
            .diagnostics
            .iter()
            .any(|diagnostic| {
                diagnostic.code == "RESET_UNSUPPORTED"
            })
    );
}

#[test]
fn mid_circuit_measurement_requirement_is_enforced() {
    let mut capabilities = representative_capabilities();
    capabilities.mid_circuit_measurement = false;

    let backend = backend_with_capabilities(capabilities);

    let mut circuit = simple_circuit();
    circuit.requires_mid_circuit_measurement = true;

    let workload = WorkloadRequirements::from_circuit(circuit);
    let report = backend.validation_report(&workload);

    assert!(!report.valid);

    assert!(
        report
            .diagnostics
            .iter()
            .any(|diagnostic| {
                diagnostic.code == "MID_CIRCUIT_MEASUREMENT_UNSUPPORTED"
            })
    );
}

#[test]
fn classical_control_requirement_is_enforced() {
    let mut capabilities = representative_capabilities();
    capabilities.classical_control = false;

    let backend = backend_with_capabilities(capabilities);

    let mut circuit = simple_circuit();
    circuit.requires_classical_control = true;

    let workload = WorkloadRequirements::from_circuit(circuit);
    let report = backend.validation_report(&workload);

    assert!(!report.valid);

    assert!(
        report
            .diagnostics
            .iter()
            .any(|diagnostic| {
                diagnostic.code == "CLASSICAL_CONTROL_UNSUPPORTED"
            })
    );
}

#[test]
fn dynamic_circuit_requirement_is_enforced() {
    let mut capabilities = representative_capabilities();
    capabilities.dynamic_circuits = false;

    let backend = backend_with_capabilities(capabilities);

    let mut circuit = simple_circuit();
    circuit.requires_dynamic_circuits = true;

    let workload = WorkloadRequirements::from_circuit(circuit);
    let report = backend.validation_report(&workload);

    assert!(!report.valid);

    assert!(
        report
            .diagnostics
            .iter()
            .any(|diagnostic| {
                diagnostic.code == "DYNAMIC_CIRCUITS_UNSUPPORTED"
            })
    );
}

#[test]
fn required_stable_capability_must_be_explicitly_supported() {
    let mut capabilities = representative_capabilities();
    capabilities.batch_execution = false;

    let backend = backend_with_capabilities(capabilities);

    let workload = workload_requiring_capability("batch_execution");
    let report = backend.validation_report(&workload);

    assert!(!report.valid);

    assert!(
        report
            .diagnostics
            .iter()
            .any(|diagnostic| {
                diagnostic.code == "REQUIRED_CAPABILITY_UNSUPPORTED"
            })
    );
}

#[test]
fn supported_stable_capability_is_accepted() {
    let backend = representative_backend();

    let workload = workload_requiring_capability("batch_execution");
    let report = backend.validation_report(&workload);

    assert!(report.valid);
}

#[test]
fn unknown_capability_is_rejected() {
    let workload = workload_requiring_capability(
        "capability_that_does_not_exist",
    );

    let report =
        representative_backend().validation_report(&workload);

    assert!(!report.valid);

    assert!(
        report
            .diagnostics
            .iter()
            .any(|diagnostic| {
                diagnostic.code == "REQUIRED_CAPABILITY_UNSUPPORTED"
            })
    );
}

#[test]
fn experimental_capability_never_satisfies_stable_requirement() {
    let mut capabilities = representative_capabilities();

    capabilities
        .experimental_capabilities
        .insert("future_capability".to_owned());

    let backend = backend_with_capabilities(capabilities);

    let workload = workload_requiring_capability("future_capability");

    let report = backend.validation_report(&workload);

    assert!(!report.valid);

    assert!(
        report
            .diagnostics
            .iter()
            .any(|diagnostic| {
                diagnostic.code == "EXPERIMENTAL_CAPABILITY"
            })
    );

    assert!(matches!(
        report.first_error(),
        Some(BackendError::ExperimentalCapabilityNotAccepted {
            capability
        }) if capability == "future_capability"
    ));
}

// =============================================================================
// Instruction compatibility
// =============================================================================

#[test]
fn required_native_instruction_is_accepted() {
    let backend = representative_backend();

    let workload = workload_requiring_instruction("cx");
    let report = backend.validation_report(&workload);

    assert!(report.valid);
}

#[test]
fn missing_native_instruction_is_rejected() {
    let workload = workload_requiring_instruction("iswap");

    let report =
        representative_backend().validation_report(&workload);

    assert!(!report.valid);

    assert!(
        report
            .diagnostics
            .iter()
            .any(|diagnostic| {
                diagnostic.code == "REQUIRED_INSTRUCTION_UNSUPPORTED"
            })
    );

    assert!(matches!(
        report.first_error(),
        Some(BackendError::UnsupportedGate { gate })
            if gate == "iswap"
    ));
}

#[test]
fn required_instruction_is_rejected_when_native_instruction_set_is_not_exposed(
) {
    let mut capabilities = representative_capabilities();
    capabilities.native_instruction_set = false;
    capabilities.native_gates.clear();

    let backend = backend_with_capabilities(capabilities);

    let workload = workload_requiring_instruction("cx");
    let report = backend.validation_report(&workload);

    assert!(!report.valid);

    assert!(
        report
            .diagnostics
            .iter()
            .any(|diagnostic| {
                diagnostic.code == "NATIVE_INSTRUCTION_SET_UNAVAILABLE"
            })
    );
}

#[test]
fn arbitrary_rotation_can_satisfy_rotation_instruction_requirement() {
    let mut capabilities = representative_capabilities();
    capabilities.native_gates.remove("rx");

    assert!(capabilities.arbitrary_single_qubit_rotations);

    let backend = backend_with_capabilities(capabilities);

    let workload = workload_requiring_instruction("rx");
    let report = backend.validation_report(&workload);

    assert!(
        report.valid,
        "arbitrary rotation capability must satisfy a supported rotation family"
    );
}

#[test]
fn parameterized_gate_capability_can_satisfy_parameterized_instruction(
) {
    let mut capabilities = representative_capabilities();
    capabilities.native_gates.remove("rx");
    capabilities.arbitrary_single_qubit_rotations = false;
    capabilities.parameterized_gates = true;

    let backend = backend_with_capabilities(capabilities);

    let workload = workload_requiring_instruction("custom_parameterized_gate");
    let report = backend.validation_report(&workload);

    assert!(
        report.valid,
        "parameterized-gate capability must allow parameterized instructions"
    );
}

#[test]
fn empty_instruction_identifier_is_rejected() {
    let mut workload = simple_workload();
    workload.circuit.gates.push(String::new());

    let report =
        representative_backend().validation_report(&workload);

    assert!(!report.valid);

    assert!(
        report
            .diagnostics
            .iter()
            .any(|diagnostic| {
                diagnostic.code == "INVALID_INSTRUCTION"
            })
    );
}

// =============================================================================
// Topology compatibility
// =============================================================================

#[test]
fn adjacent_two_qubit_operation_is_compatible() {
    let backend = representative_backend();

    let workload = simple_workload();

    assert!(
        backend.validation_report(&workload).valid,
        "0 -> 1 is present in the representative topology"
    );
}

#[test]
fn non_adjacent_two_qubit_operation_is_rejected() {
    let backend = representative_backend();

    let mut circuit = simple_circuit();
    circuit.two_qubit_edges = vec![(0, 2)];

    let workload = WorkloadRequirements::from_circuit(circuit);
    let report = backend.validation_report(&workload);

    assert!(!report.valid);

    assert!(
        report
            .diagnostics
            .iter()
            .any(|diagnostic| {
                diagnostic.code == "UNSUPPORTED_CONNECTION"
            })
    );
}

#[test]
fn invalid_control_qubit_is_rejected() {
    let backend = representative_backend();

    let mut circuit = simple_circuit();
    circuit.two_qubit_edges = vec![(4, 1)];

    let workload = WorkloadRequirements::from_circuit(circuit);
    let report = backend.validation_report(&workload);

    assert!(!report.valid);

    assert!(
        report
            .diagnostics
            .iter()
            .any(|diagnostic| {
                diagnostic.code == "INVALID_CONTROL_QUBIT"
            })
    );
}

#[test]
fn invalid_target_qubit_is_rejected() {
    let backend = representative_backend();

    let mut circuit = simple_circuit();
    circuit.two_qubit_edges = vec![(0, 4)];

    let workload = WorkloadRequirements::from_circuit(circuit);
    let report = backend.validation_report(&workload);

    assert!(!report.valid);

    assert!(
        report
            .diagnostics
            .iter()
            .any(|diagnostic| {
                diagnostic.code == "INVALID_TARGET_QUBIT"
            })
    );
}

#[test]
fn self_interaction_is_rejected() {
    let backend = representative_backend();

    let mut circuit = simple_circuit();
    circuit.two_qubit_edges = vec![(1, 1)];

    let workload = WorkloadRequirements::from_circuit(circuit);
    let report = backend.validation_report(&workload);

    assert!(!report.valid);

    assert!(
        report
            .diagnostics
            .iter()
            .any(|diagnostic| {
                diagnostic.code == "SELF_INTERACTION"
            })
    );
}

#[test]
fn topology_requirement_is_enforced() {
    let mut capabilities = representative_capabilities();
    capabilities.topology_information = false;

    let backend = backend_with_capabilities(capabilities);

    let workload =
        simple_workload().with_topology_requirement(true);

    let report = backend.validation_report(&workload);

    assert!(!report.valid);

    assert!(
        report
            .diagnostics
            .iter()
            .any(|diagnostic| {
                diagnostic.code == "TOPOLOGY_INFORMATION_UNAVAILABLE"
            })
    );
}

#[test]
fn explicit_topology_requirement_is_accepted_when_exposed() {
    let workload =
        simple_workload().with_topology_requirement(true);

    let report =
        representative_backend().validation_report(&workload);

    assert!(report.valid);
}

#[test]
fn topology_resource_capacity_is_enforced_independently_of_backend_limit(
) {
    let topology = HardwareTopology::linear(2)
        .expect("two-resource topology must be valid");

    let backend = backend_with_topology(topology);

    let mut circuit = simple_circuit();
    circuit.qubit_count = 3;

    let workload = WorkloadRequirements::from_circuit(circuit);
    let report = backend.validation_report(&workload);

    assert!(!report.valid);

    assert!(
        report
            .diagnostics
            .iter()
            .any(|diagnostic| {
                diagnostic.code == "TOPOLOGY_QUBIT_LIMIT"
            })
    );
}

// =============================================================================
// Calibration compatibility
// =============================================================================

#[test]
fn calibration_requirement_is_rejected_when_unavailable() {
    let mut capabilities = representative_capabilities();
    capabilities.calibration_data = false;

    let backend = backend_with_capabilities(capabilities);

    let workload =
        simple_workload().with_calibration_requirement(true);

    let report = backend.validation_report(&workload);

    assert!(!report.valid);

    assert!(
        report
            .diagnostics
            .iter()
            .any(|diagnostic| {
                diagnostic.code == "CALIBRATION_UNAVAILABLE"
            })
    );
}

#[test]
fn calibration_requirement_is_accepted_when_exposed() {
    let workload =
        simple_workload().with_calibration_requirement(true);

    let report =
        representative_backend().validation_report(&workload);

    assert!(report.valid);
}

#[test]
fn fresh_calibration_requires_calibration_data() {
    let mut capabilities = representative_capabilities();
    capabilities.calibration_data = false;

    let backend = backend_with_capabilities(capabilities);

    let workload =
        simple_workload().with_fresh_calibration_requirement(true);

    let report = backend.validation_report(&workload);

    assert!(!report.valid);

    assert!(
        report
            .diagnostics
            .iter()
            .any(|diagnostic| {
                diagnostic.code == "FRESH_CALIBRATION_UNAVAILABLE"
            })
    );
}

#[test]
fn fresh_calibration_is_explicitly_deferred_to_calibration_snapshot_check(
) {
    let workload =
        simple_workload().with_fresh_calibration_requirement(true);

    let report =
        representative_backend().validation_report(&workload);

    assert!(
        report.valid,
        "backend capability compatibility should not pretend to prove freshness"
    );

    assert!(
        report
            .diagnostics
            .iter()
            .any(|diagnostic| {
                diagnostic.code == "CALIBRATION_FRESHNESS_DEFERRED"
                    && diagnostic.severity == ValidationSeverity::Warning
            })
    );
}

// =============================================================================
// Advanced workload compatibility
// =============================================================================

#[test]
fn pulse_workload_is_rejected_by_gate_only_backend() {
    let mut circuit = simple_circuit();
    circuit.requires_pulse_control = true;

    let workload = WorkloadRequirements::from_circuit(circuit);
    let report =
        representative_backend().validation_report(&workload);

    assert!(!report.valid);

    assert!(
        report
            .diagnostics
            .iter()
            .any(|diagnostic| {
                diagnostic.code == "PULSE_CONTROL_UNSUPPORTED"
            })
    );
}

#[test]
fn analog_workload_is_rejected_by_gate_only_backend() {
    let mut circuit = simple_circuit();
    circuit.requires_analog_control = true;

    let workload = WorkloadRequirements::from_circuit(circuit);
    let report =
        representative_backend().validation_report(&workload);

    assert!(!report.valid);

    assert!(
        report
            .diagnostics
            .iter()
            .any(|diagnostic| {
                diagnostic.code == "ANALOG_CONTROL_UNSUPPORTED"
            })
    );
}

#[test]
fn annealing_workload_is_rejected_by_gate_only_backend() {
    let mut circuit = simple_circuit();
    circuit.requires_annealing = true;

    let workload = WorkloadRequirements::from_circuit(circuit);
    let report =
        representative_backend().validation_report(&workload);

    assert!(!report.valid);

    assert!(
        report
            .diagnostics
            .iter()
            .any(|diagnostic| {
                diagnostic.code == "ANNEALING_UNSUPPORTED"
            })
    );
}

#[test]
fn logical_workload_is_rejected_without_logical_qubit_support() {
    let mut circuit = simple_circuit();
    circuit.requires_logical_qubits = true;
    circuit.logical_qubit_count = 1;

    let workload = WorkloadRequirements::from_circuit(circuit);
    let report =
        representative_backend().validation_report(&workload);

    assert!(!report.valid);

    assert!(
        report
            .diagnostics
            .iter()
            .any(|diagnostic| {
                diagnostic.code == "LOGICAL_QUBITS_UNSUPPORTED"
            })
    );
}

#[test]
fn fault_tolerant_workload_is_rejected_without_fault_tolerance_support() {
    let mut circuit = simple_circuit();
    circuit.requires_fault_tolerance = true;

    let workload = WorkloadRequirements::from_circuit(circuit);
    let report =
        representative_backend().validation_report(&workload);

    assert!(!report.valid);

    assert!(
        report
            .diagnostics
            .iter()
            .any(|diagnostic| {
                diagnostic.code == "FAULT_TOLERANCE_UNSUPPORTED"
            })
    );
}

#[test]
fn deterministic_execution_requirement_is_enforced() {
    let mut capabilities = representative_capabilities();
    capabilities.deterministic_seeding = false;

    let backend = backend_with_capabilities(capabilities);

    let workload = simple_workload();

    let request = ExecutionRequest::from_workload(workload)
        .with_seed(42);

    assert!(matches!(
        backend.validate_request(&request),
        Err(BackendError::DeterministicSeedingUnsupported)
    ));
}

#[test]
fn deterministic_execution_is_accepted_when_supported() {
    let backend = representative_backend();

    let request = ExecutionRequest::from_workload(simple_workload())
        .with_seed(42);

    assert_eq!(backend.validate_request(&request), Ok(()));
}

#[test]
fn state_vector_result_requirement_is_enforced() {
    let mut capabilities = representative_capabilities();
    capabilities.state_vector_results = false;

    let backend = backend_with_capabilities(capabilities);

    let mut circuit = simple_circuit();
    circuit.requires_state_vector = true;

    let workload = WorkloadRequirements::from_circuit(circuit);
    let report =
        backend.validation_report(&workload);

    assert!(!report.valid);

    assert!(
        report
            .diagnostics
            .iter()
            .any(|diagnostic| {
                diagnostic.code == "STATE_VECTOR_UNSUPPORTED"
            })
    );
}

#[test]
fn density_matrix_result_requirement_is_enforced() {
    let mut capabilities = representative_capabilities();
    capabilities.density_matrix_results = false;

    let backend = backend_with_capabilities(capabilities);

    let mut circuit = simple_circuit();
    circuit.requires_density_matrix = true;

    let workload = WorkloadRequirements::from_circuit(circuit);
    let report =
        backend.validation_report(&workload);

    assert!(!report.valid);

    assert!(
        report
            .diagnostics
            .iter()
            .any(|diagnostic| {
                diagnostic.code == "DENSITY_MATRIX_UNSUPPORTED"
            })
    );
}

#[test]
fn expectation_value_requirement_is_accepted_when_supported() {
    let mut circuit = simple_circuit();
    circuit.requires_expectation_values = true;

    let workload = WorkloadRequirements::from_circuit(circuit);
    let report =
        representative_backend().validation_report(&workload);

    assert!(report.valid);
}

#[test]
fn expectation_value_requirement_is_rejected_when_unsupported() {
    let mut capabilities = representative_capabilities();
    capabilities.expectation_value_results = false;

    let backend = backend_with_capabilities(capabilities);

    let mut circuit = simple_circuit();
    circuit.requires_expectation_values = true;

    let workload = WorkloadRequirements::from_circuit(circuit);
    let report =
        backend.validation_report(&workload);

    assert!(!report.valid);

    assert!(
        report
            .diagnostics
            .iter()
            .any(|diagnostic| {
                diagnostic.code == "EXPECTATION_VALUES_UNSUPPORTED"
            })
    );
}

// =============================================================================
// Request-level compatibility
// =============================================================================

#[test]
fn execution_request_preflight_accepts_compatible_request() {
    let backend = representative_backend();

    let request =
        ExecutionRequest::from_workload(simple_workload());

    assert_eq!(backend.preflight(&request), Ok(()));
}

#[test]
fn synchronous_execution_request_is_still_compatible() {
    let backend = representative_backend();

    let request =
        ExecutionRequest::from_workload(simple_workload())
            .synchronous();

    assert_eq!(backend.preflight(&request), Ok(()));
}

#[test]
fn request_with_valid_identifier_is_accepted() {
    let backend = representative_backend();

    let request =
        ExecutionRequest::from_workload(simple_workload())
            .with_request_id("compatibility-test-001")
            .expect("valid request ID must be accepted");

    assert_eq!(backend.preflight(&request), Ok(()));
}

#[test]
fn malformed_request_identifier_is_rejected() {
    let backend = representative_backend();

    let request =
        ExecutionRequest::from_workload(simple_workload())
            .with_request_id(" \t\n ");

    assert!(
        matches!(
            request,
            Err(BackendError::InvalidIdentifier {
                field: "request_id"
            })
        ),
        "whitespace-only request IDs must be rejected"
    );
}

#[test]
fn request_metadata_is_part_of_compatibility_boundary() {
    let backend = representative_backend();

    let mut request =
        ExecutionRequest::from_workload(simple_workload());

    request
        .insert_metadata("experiment", "compatibility-test")
        .expect("ordinary metadata must be accepted");

    assert_eq!(backend.preflight(&request), Ok(()));
}

#[test]
fn secret_like_request_metadata_is_rejected() {
    let backend = representative_backend();

    let mut request =
        ExecutionRequest::from_workload(simple_workload());

    let result = request.insert_metadata(
        "api_key",
        "synthetic-test-secret",
    );

    assert!(matches!(
        result,
        Err(BackendError::SecretLikeMetadata { .. })
    ));
}

// =============================================================================
// Requirement-set invariants
// =============================================================================

#[test]
fn malformed_required_capability_is_rejected_before_backend_matching() {
    let mut workload = simple_workload();
    workload
        .required_capabilities
        .insert("bad\ncapability".to_owned());

    let report =
        representative_backend().validation_report(&workload);

    assert!(!report.valid);

    assert!(
        report
            .diagnostics
            .iter()
            .any(|diagnostic| {
                diagnostic.code == "INVALID_WORKLOAD"
            })
    );

    assert!(matches!(
        report.first_error(),
        Some(BackendError::InvalidIdentifier {
            field: "required_capability"
        })
    ));
}

#[test]
fn malformed_required_instruction_is_rejected_before_backend_matching() {
    let mut workload = simple_workload();
    workload
        .required_instructions
        .insert("cx\ninvalid".to_owned());

    let report =
        representative_backend().validation_report(&workload);

    assert!(!report.valid);

    assert!(
        report
            .diagnostics
            .iter()
            .any(|diagnostic| {
                diagnostic.code == "INVALID_WORKLOAD"
            })
    );
}

#[test]
fn malformed_circuit_gate_is_rejected_before_backend_matching() {
    let mut workload = simple_workload();
    workload.circuit.gates.push("cx\ninvalid".to_owned());

    let report =
        representative_backend().validation_report(&workload);

    assert!(!report.valid);

    assert!(
        report
            .diagnostics
            .iter()
            .any(|diagnostic| {
                diagnostic.code == "INVALID_WORKLOAD"
            })
    );
}

// =============================================================================
// Determinism
// =============================================================================

#[test]
fn compatibility_diagnostics_are_deterministically_ordered() {
    let backend = representative_backend();

    let mut workload = simple_workload();

    workload.circuit.qubit_count = 99;
    workload.circuit.circuit_depth = 101;
    workload.circuit.operation_count = 10_000;
    workload.circuit.shots = 20_000;

    workload
        .required_capabilities
        .insert("unsupported_capability_b".to_owned());

    workload
        .required_capabilities
        .insert("unsupported_capability_a".to_owned());

    workload
        .required_instructions
        .insert("unsupported_gate_b".to_owned());

    workload
        .required_instructions
        .insert("unsupported_gate_a".to_owned());

    let first = diagnostic_codes(&backend, &workload);
    let second = diagnostic_codes(&backend, &workload);
    let third = diagnostic_codes(&backend, &workload);

    assert_eq!(first, second);
    assert_eq!(second, third);
}

#[test]
fn compatibility_report_contains_no_duplicate_diagnostic_identity() {
    let backend = representative_backend();

    let mut workload = simple_workload();

    workload
        .required_capabilities
        .insert("batch_execution".to_owned());

    workload
        .required_capabilities
        .insert("unsupported_capability".to_owned());

    let report = backend.validation_report(&workload);

    let mut identities = BTreeSet::new();

    for diagnostic in &report.diagnostics {
        let identity = (
            diagnostic.code,
            diagnostic.severity,
            diagnostic.requirement.clone(),
            diagnostic.message.clone(),
        );

        assert!(
            identities.insert(identity),
            "duplicate diagnostic identity detected"
        );
    }
}

#[test]
fn compatibility_result_does_not_depend_on_collection_insertion_order() {
    let backend = representative_backend();

    let mut first = simple_workload();
    first
        .required_capabilities
        .insert("unsupported_a".to_owned());
    first
        .required_capabilities
        .insert("unsupported_b".to_owned());

    let mut second = simple_workload();
    second
        .required_capabilities
        .insert("unsupported_b".to_owned());
    second
        .required_capabilities
        .insert("unsupported_a".to_owned());

    let first_report = backend.validation_report(&first);
    let second_report = backend.validation_report(&second);

    assert_eq!(
        first_report.diagnostics,
        second_report.diagnostics
    );
}

// =============================================================================
// Multiple simultaneous incompatibilities
// =============================================================================

#[test]
fn compatibility_report_preserves_multiple_independent_failures() {
    let mut capabilities = representative_capabilities();
    capabilities.measurement = false;
    capabilities.reset = false;
    capabilities.dynamic_circuits = false;

    let backend = backend_with_capabilities(capabilities);

    let mut circuit = simple_circuit();
    circuit.qubit_count = 5;
    circuit.requires_measurement = true;
    circuit.requires_reset = true;
    circuit.requires_dynamic_circuits = true;
    circuit.two_qubit_edges = vec![(0, 3)];

    let workload = WorkloadRequirements::from_circuit(circuit);

    let report = backend.validation_report(&workload);

    assert!(!report.valid);

    let codes: BTreeSet<&'static str> = report
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.code)
        .collect();

    assert!(codes.contains("QUBIT_LIMIT"));
    assert!(codes.contains("MEASUREMENT_UNSUPPORTED"));
    assert!(codes.contains("RESET_UNSUPPORTED"));
    assert!(codes.contains("DYNAMIC_CIRCUITS_UNSUPPORTED"));
    assert!(codes.contains("UNSUPPORTED_CONNECTION"));
}

// =============================================================================
// Provider neutrality
// =============================================================================

#[test]
fn compatibility_does_not_require_provider_specific_types() {
    let backend = representative_backend();

    let workload = simple_workload();

    let report = backend.validation_report(&workload);

    assert!(
        report.valid,
        "provider-neutral compatibility must work without a provider adapter"
    );

    assert_eq!(report.backend_id, "test.backend");
}

#[test]
fn provider_identifier_does_not_change_workload_semantics() {
    let metadata_a = BackendMetadata::new(
        "backend.a",
        "Backend A",
        "provider.a",
        "1.0.0",
        BackendKind::Qpu,
    );

    let metadata_b = BackendMetadata::new(
        "backend.b",
        "Backend B",
        "provider.b",
        "1.0.0",
        BackendKind::Qpu,
    );

    let backend_a = QuantumBackend::new(
        metadata_a,
        representative_capabilities(),
        representative_limits(),
        representative_topology(),
    )
    .expect("backend A fixture must be valid");

    let backend_b = QuantumBackend::new(
        metadata_b,
        representative_capabilities(),
        representative_limits(),
        representative_topology(),
    )
    .expect("backend B fixture must be valid");

    let workload = simple_workload();

    assert_eq!(
        backend_a.validation_report(&workload).valid,
        backend_b.validation_report(&workload).valid
    );
}

// =============================================================================
// Regression guards for architectural boundaries
// =============================================================================

#[test]
fn compatibility_uses_authoritative_topology() {
    let topology = HardwareTopology::linear(4)
        .expect("topology must be valid");

    let backend = backend_with_topology(topology);

    let mut circuit = simple_circuit();
    circuit.two_qubit_edges = vec![(0, 2)];

    let workload = WorkloadRequirements::from_circuit(circuit);

    let report = backend.validation_report(&workload);

    assert!(
        !report.valid,
        "compatibility must consult the authoritative hardware topology"
    );
}

#[test]
fn compatibility_does_not_assume_full_connectivity() {
    let backend = representative_backend();

    let mut circuit = simple_circuit();
    circuit.two_qubit_edges = vec![(0, 3)];

    let workload = WorkloadRequirements::from_circuit(circuit);

    assert!(
        !backend.validation_report(&workload).valid,
        "compatibility must never assume an arbitrary two-qubit pair is connected"
    );
}

#[test]
fn compatibility_does_not_treat_degraded_status_as_hard_failure() {
    let backend = backend_with_status(BackendStatus::Degraded);

    let report = backend.validation_report(&simple_workload());

    assert!(report.valid);
    assert!(
        report
            .diagnostics
            .iter()
            .all(|diagnostic| {
                diagnostic.severity != ValidationSeverity::Error
                    || diagnostic.code != "BACKEND_DEGRADED"
            })
    );
}

#[test]
fn compatibility_never_claims_freshness_without_calibration_evidence() {
    let workload =
        simple_workload().with_fresh_calibration_requirement(true);

    let report =
        representative_backend().validation_report(&workload);

    assert!(
        report
            .diagnostics
            .iter()
            .any(|diagnostic| {
                diagnostic.code == "CALIBRATION_FRESHNESS_DEFERRED"
            }),
        "freshness must be verified against a calibration snapshot rather than inferred from capability metadata"
    );
}

// =============================================================================
// End-to-end compatibility contract
// =============================================================================

#[test]
fn complete_preflight_contract_accepts_valid_request() {
    let backend = representative_backend();

    let mut workload = simple_workload();

    workload
        .required_capabilities
        .insert("measurement".to_owned());

    workload
        .required_capabilities
        .insert("reset".to_owned());

    workload
        .required_instructions
        .insert("h".to_owned());

    workload
        .required_instructions
        .insert("cx".to_owned());

    workload
        .required_instructions
        .insert("measure".to_owned());

    let request =
        ExecutionRequest::from_workload(workload)
            .with_seed(12345)
            .with_priority(10)
            .with_request_id("compatibility-e2e-001")
            .expect("request identifier must be valid");

    assert_eq!(backend.preflight(&request), Ok(()));
}

#[test]
fn complete_preflight_contract_rejects_incompatible_request() {
    let mut capabilities = representative_capabilities();
    capabilities.dynamic_circuits = false;
    capabilities.measurement = false;

    let backend = backend_with_capabilities(capabilities);

    let mut circuit = simple_circuit();
    circuit.requires_dynamic_circuits = true;
    circuit.requires_measurement = true;

    let workload = WorkloadRequirements::from_circuit(circuit);

    let request = ExecutionRequest::from_workload(workload);

    let result = backend.preflight(&request);

    assert!(result.is_err());

    assert!(matches!(
        result,
        Err(BackendError::DynamicCircuitUnsupported)
            | Err(BackendError::DynamicCircuitsUnsupported)
            | Err(BackendError::MeasurementUnsupported)
            | Err(BackendError::ExecutionRejected(_))
    ));
}

// =============================================================================
// Compatibility semantics that must remain stable
// =============================================================================

#[test]
fn workload_advanced_classification_is_provider_independent() {
    let simple = simple_workload();

    assert!(!simple.circuit.is_advanced());

    let mut dynamic = simple.circuit.clone();
    dynamic.requires_dynamic_circuits = true;

    assert!(
        WorkloadRequirements::from_circuit(dynamic)
            .circuit
            .is_advanced()
    );
}

#[test]
fn native_gate_listing_is_deterministic() {
    let backend = representative_backend();

    let gates = backend.native_gates();

    let mut sorted = gates.clone();
    sorted.sort();

    assert_eq!(gates, sorted);
}

#[test]
fn capability_listing_is_deterministic() {
    let backend = representative_backend();

    let capabilities = backend.capability_names();

    let mut sorted = capabilities.clone();
    sorted.sort();

    assert_eq!(capabilities, sorted);
}

// =============================================================================
// Compatibility matrix smoke test
// =============================================================================

#[test]
fn compatibility_matrix_has_expected_gate_model_results() {
    let backend = representative_backend();

    let cases = [
        (
            "ordinary gate circuit",
            simple_workload(),
            true,
        ),
        (
            "dynamic circuit",
            {
                let mut circuit = simple_circuit();
                circuit.requires_dynamic_circuits = true;
                WorkloadRequirements::from_circuit(circuit)
            },
            true,
        ),
        (
            "pulse workload",
            {
                let mut circuit = simple_circuit();
                circuit.requires_pulse_control = true;
                WorkloadRequirements::from_circuit(circuit)
            },
            false,
        ),
        (
            "analog workload",
            {
                let mut circuit = simple_circuit();
                circuit.requires_analog_control = true;
                WorkloadRequirements::from_circuit(circuit)
            },
            false,
        ),
        (
            "annealing workload",
            {
                let mut circuit = simple_circuit();
                circuit.requires_annealing = true;
                WorkloadRequirements::from_circuit(circuit)
            },
            false,
        ),
        (
            "logical workload",
            {
                let mut circuit = simple_circuit();
                circuit.requires_logical_qubits = true;
                circuit.logical_qubit_count = 1;
                WorkloadRequirements::from_circuit(circuit)
            },
            false,
        ),
    ];

    for (name, workload, expected) in cases {
        let report = backend.validation_report(&workload);

        assert_eq!(
            report.valid, expected,
            "unexpected compatibility result for {name}: {:?}",
            report.diagnostics
        );
    }
}

// =============================================================================
// Explicit integration contract documentation test
// =============================================================================

#[test]
fn compatibility_boundary_has_the_expected_downstream_contract() {
    let backend = representative_backend();

    // The following calls intentionally exercise the public methods that
    // downstream hardware components are expected to consume.
    //
    // Future modules:
    //
    // - routing.rs
    // - scheduling.rs
    // - execution.rs
    // - job.rs
    // - provider adapters
    // - benchmarking
    // - Danga
    //
    // must consume these provider-neutral semantics rather than bypassing
    // compatibility validation.
    let workload = simple_workload();

    let report = backend.validation_report(&workload);
    assert!(report.valid);

    assert_eq!(backend.validate(&workload), Ok(()));

    let request = ExecutionRequest::from_workload(workload);

    assert_eq!(backend.validate_request(&request), Ok(()));
    assert_eq!(backend.preflight(&request), Ok(()));
}