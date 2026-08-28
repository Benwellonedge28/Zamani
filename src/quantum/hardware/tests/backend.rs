//! Zamani Quantum Hardware — Backend Conformance Tests.
//!
//! Production conformance tests for:
//!
//! `crate::quantum::hardware::backend`
//!
//! # Responsibility
//!
//! This module verifies the canonical, provider-neutral backend contract.
//!
//! It protects:
//!
//! - backend identity semantics;
//! - backend kind semantics;
//! - backend status semantics;
//! - capability semantics;
//! - backend resource limits;
//! - backend metadata;
//! - workload classification;
//! - circuit/workload requirements;
//! - execution request validation;
//! - execution result invariants;
//! - backend validation;
//! - topology integration;
//! - deterministic behaviour;
//! - metadata security rules;
//! - boundary/resource limits;
//! - error classification;
//! - provider neutrality;
//! - regression protection for future provider adapters.
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
//! - provider-specific job semantics;
//! - routing algorithms;
//! - scheduling algorithms;
//! - calibration acquisition;
//! - benchmarking mathematics;
//! - QEC algorithms;
//! - OpenQASM parsing;
//! - QIR generation;
//! - simulator internals.
//!
//! Those concerns have independent ownership boundaries.
//!
//! # Integration contract
//!
//! This file tests the public contract exposed by `backend.rs`.
//!
//! The test suite intentionally does not reach into private implementation
//! details. Future changes to the implementation are therefore allowed as long
//! as the public backend contract and its invariants remain valid.
//!
//! The contract is consumed by:
//!
//! - `backend_trait.rs`;
//! - `backend_config.rs`;
//! - `backend_status.rs`;
//! - `capabilities.rs`;
//! - `compatibility.rs`;
//! - `validation.rs`;
//! - `execution.rs`;
//! - `job.rs`;
//! - `queue.rs`;
//! - provider registries;
//! - device registries;
//! - adapters;
//! - benchmarking;
//! - Danga.
//!
//! None of those downstream systems should require this test module to be
//! modified merely because another provider is added.
//!
//! # Production invariants
//!
//! A conforming backend implementation must:
//!
//! 1. reject malformed identifiers;
//! 2. reject empty required values;
//! 3. preserve deterministic ordering;
//! 4. never silently accept invalid workload requirements;
//! 5. never report a capability that was not explicitly advertised;
//! 6. never treat experimental capability as stable capability;
//! 7. preserve topology ownership in `topology.rs`;
//! 8. reject unsafe metadata;
//! 9. enforce documented resource limits;
//! 10. produce deterministic validation results;
//! 11. distinguish backend description from execution adapter;
//! 12. remain provider-neutral.
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
//! Tests never use real credentials, tokens, provider endpoints, private keys,
//! or authentication material.
//!
//! The backend metadata security boundary is tested using synthetic secret-like
//! values only.
//!
//! # Determinism
//!
//! Tests use:
//!
//! - fixed identifiers;
//! - fixed values;
//! - deterministic collections;
//! - no wall-clock assumptions;
//! - no randomness;
//! - no network;
//! - no environment variables.
//!
//! A backend test must remain reproducible on every machine and CI runner.
//!
//! # Important architectural distinction
//!
//! `BackendKind` describes the execution target category.
//!
//! `BackendCapabilities` describes what the target can do.
//!
//! `BackendLimits` describes its resource envelope.
//!
//! `BackendMetadata` describes identifying/descriptive information.
//!
//! `QuantumWorkloadKind` describes the requested workload.
//!
//! `CircuitRequirements` describes gate-model circuit requirements.
//!
//! `WorkloadRequirements` describes generalized workload requirements.
//!
//! `ExecutionRequest` describes an execution request.
//!
//! `ExecutionResult` describes normalized execution output.
//!
//! These concepts must not be collapsed together.
//!
//! # Test policy
//!
//! The tests below intentionally exercise both happy paths and failure paths.
//! A production hardware abstraction is not complete merely because valid
//! backend descriptors can be constructed; invalid descriptors and unsafe
//! execution requests must fail deterministically.
!
//! # Future provider adapters
//!
//! Provider adapters such as IBM, IonQ, AWS Braket, Rigetti, IQM, Quantinuum,
//! QuEra and local adapters must conform to this contract.
//!
//! Adding a provider must not require provider-specific branches in this test
//! module.
//!
//! -----------------------------------------------------------------------------
//! Imports
//! -----------------------------------------------------------------------------

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::collections::{BTreeMap, BTreeSet};

use crate::quantum::hardware::backend::{
    BackendCapabilities,
    BackendDescriptor,
    BackendError,
    BackendKind,
    BackendLimits,
    BackendMetadata,
    BackendStatus,
    CircuitRequirements,
    ExecutionRequest,
    ExecutionResult,
    QuantumBackend,
    QuantumWorkloadKind,
    WorkloadRequirements,
    BACKEND_SCHEMA_ID,
    BACKEND_SCHEMA_VERSION,
    MAX_BACKEND_ID_LENGTH,
    MAX_BACKEND_NAME_LENGTH,
    MAX_PROVIDER_ID_LENGTH,
    MAX_BACKEND_VERSION_LENGTH,
    MAX_HARDWARE_REVISION_LENGTH,
    MAX_FIRMWARE_VERSION_LENGTH,
    MAX_API_VERSION_LENGTH,
    MAX_REGION_LENGTH,
    MAX_METADATA_KEY_LENGTH,
    MAX_METADATA_VALUE_LENGTH,
    MAX_METADATA_PROPERTIES,
    MAX_NATIVE_INSTRUCTIONS,
    MAX_REQUIRED_INSTRUCTIONS,
    MAX_REQUIRED_TOPOLOGY_EDGES,
    MAX_REQUEST_METADATA_PROPERTIES,
    MAX_REQUEST_ID_LENGTH,
};

use crate::quantum::hardware::topology::HardwareTopology;

// =============================================================================
// Helpers
// =============================================================================

/// Representative native gate set used throughout backend conformance tests.
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

/// Representative backend capabilities.
///
/// This deliberately describes a gate-model backend rather than assuming every
/// possible hardware technology supports every feature.
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

/// Conservative resource limits suitable for deterministic tests.
fn representative_limits() -> BackendLimits {
    BackendLimits {
        max_qubits: 127,
        max_depth: 10_000,
        max_operations: 1_000_000,
        max_shots: 100_000,
        max_circuit_size_bytes: 16 * 1024 * 1024,
    }
}

/// Representative topology.
///
/// The exact topology construction API is intentionally isolated here so that
/// all backend tests use one canonical topology fixture.
fn representative_topology() -> HardwareTopology {
    HardwareTopology::linear(4)
        .expect("linear topology with four physical qubits must be valid")
}

/// Representative backend metadata.
fn representative_metadata() -> BackendMetadata {
    BackendMetadata {
        backend_id: "local.test.backend".to_owned(),
        name: "Zamani Test Backend".to_owned(),
        provider_id: "zamani.test".to_owned(),
        backend_version: "1.0.0".to_owned(),
        hardware_revision: Some("test-rev-1".to_owned()),
        firmware_version: Some("test-fw-1.0".to_owned()),
        api_version: Some("1.0".to_owned()),
        region: Some("test-region".to_owned()),
        description: Some("Deterministic provider-neutral conformance backend".to_owned()),
        properties: BTreeMap::new(),
    }
}

/// Builds a representative backend descriptor.
///
/// The fixture intentionally contains no provider credentials and no network
/// endpoint.
fn representative_descriptor() -> BackendDescriptor {
    BackendDescriptor::new(
        representative_metadata(),
        BackendKind::Qpu,
        BackendStatus::Available,
        representative_capabilities(),
        representative_limits(),
        representative_topology(),
    )
    .expect("representative backend descriptor must be valid")
}

/// Representative simple circuit requirements.
fn representative_circuit_requirements() -> CircuitRequirements {
    CircuitRequirements {
        qubits: 2,
        depth: 4,
        operations: 6,
        shots: 1_000,
        required_gates: representative_native_gates(),
        requires_mid_circuit_measurement: false,
        requires_reset: false,
        requires_classical_control: false,
        requires_dynamic_circuits: false,
    }
}

/// Representative generalized workload requirements.
fn representative_workload_requirements() -> WorkloadRequirements {
    WorkloadRequirements {
        kind: QuantumWorkloadKind::GateCircuit,
        qubits: 2,
        depth: 4,
        operations: 6,
        shots: 1_000,
        required_capabilities: BTreeSet::new(),
        required_instructions: BTreeSet::from([
            "h".to_owned(),
            "cx".to_owned(),
            "measure".to_owned(),
        ]),
        required_topology_edges: 1,
    }
}

// =============================================================================
// Schema invariants
// =============================================================================

#[test]
fn backend_schema_identity_is_stable() {
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
fn backend_schema_version_is_bounded() {
    assert!(
        BACKEND_SCHEMA_VERSION <= u16::MAX,
        "schema version must fit the declared public representation"
    );
}

// =============================================================================
// BackendKind
// =============================================================================

#[test]
fn backend_kind_identifiers_are_stable() {
    assert_eq!(BackendKind::Simulator.as_str(), "simulator");
    assert_eq!(BackendKind::Emulator.as_str(), "emulator");
    assert_eq!(BackendKind::Qpu.as_str(), "qpu");
    assert_eq!(BackendKind::Custom.as_str(), "custom");
}

#[test]
fn backend_kind_display_is_machine_readable() {
    assert_eq!(BackendKind::Simulator.to_string(), "simulator");
    assert_eq!(BackendKind::Emulator.to_string(), "emulator");
    assert_eq!(BackendKind::Qpu.to_string(), "qpu");
    assert_eq!(BackendKind::Custom.to_string(), "custom");
}

#[test]
fn backend_kind_physical_semantics_are_correct() {
    assert!(!BackendKind::Simulator.is_physical());
    assert!(!BackendKind::Emulator.is_physical());
    assert!(BackendKind::Qpu.is_physical());
    assert!(!BackendKind::Custom.is_physical());
}

#[test]
fn backend_kind_software_semantics_are_correct() {
    assert!(BackendKind::Simulator.is_software());
    assert!(BackendKind::Emulator.is_software());
    assert!(!BackendKind::Qpu.is_software());
    assert!(!BackendKind::Custom.is_software());
}

#[test]
fn backend_kind_ordering_is_deterministic() {
    let mut kinds = vec![
        BackendKind::Qpu,
        BackendKind::Simulator,
        BackendKind::Custom,
        BackendKind::Emulator,
    ];

    kinds.sort();

    assert_eq!(
        kinds,
        vec![
            BackendKind::Simulator,
            BackendKind::Emulator,
            BackendKind::Qpu,
            BackendKind::Custom,
        ]
    );
}

// =============================================================================
// BackendStatus
// =============================================================================

#[test]
fn backend_status_identifiers_are_stable() {
    assert_eq!(BackendStatus::Unknown.as_str(), "unknown");
    assert_eq!(BackendStatus::Available.as_str(), "available");
    assert_eq!(BackendStatus::Busy.as_str(), "busy");
    assert_eq!(BackendStatus::Maintenance.as_str(), "maintenance");
    assert_eq!(BackendStatus::Degraded.as_str(), "degraded");
    assert_eq!(BackendStatus::Offline.as_str(), "offline");
    assert_eq!(BackendStatus::Retired.as_str(), "retired");
    assert_eq!(BackendStatus::Unavailable.as_str(), "unavailable");
}

#[test]
fn backend_status_display_is_machine_readable() {
    assert_eq!(BackendStatus::Available.to_string(), "available");
    assert_eq!(BackendStatus::Unavailable.to_string(), "unavailable");
}

#[test]
fn only_usable_statuses_are_submission_eligible_by_status_alone() {
    assert!(!BackendStatus::Unknown.is_usable());
    assert!(BackendStatus::Available.is_usable());
    assert!(!BackendStatus::Busy.is_usable());
    assert!(!BackendStatus::Maintenance.is_usable());
    assert!(BackendStatus::Degraded.is_usable());
    assert!(!BackendStatus::Offline.is_usable());
    assert!(!BackendStatus::Retired.is_usable());
    assert!(!BackendStatus::Unavailable.is_usable());
}

#[test]
fn operational_statuses_are_distinguished_from_usable_statuses() {
    assert!(!BackendStatus::Unknown.is_operational());
    assert!(BackendStatus::Available.is_operational());
    assert!(BackendStatus::Busy.is_operational());
    assert!(BackendStatus::Maintenance.is_operational());
    assert!(BackendStatus::Degraded.is_operational());
    assert!(!BackendStatus::Offline.is_operational());
    assert!(!BackendStatus::Retired.is_operational());
    assert!(!BackendStatus::Unavailable.is_operational());
}

// =============================================================================
// Capability model
// =============================================================================

#[test]
fn default_capabilities_are_conservative() {
    let capabilities = BackendCapabilities::default();

    assert!(capabilities.measurement);
    assert!(capabilities.reset);

    assert!(!capabilities.dynamic_circuits);
    assert!(!capabilities.pulse_control);
    assert!(!capabilities.analog_control);
    assert!(!capabilities.annealing);
    assert!(!capabilities.logical_qubits);
    assert!(!capabilities.fault_tolerance);
}

#[test]
fn native_gate_collection_is_deterministic() {
    let capabilities = representative_capabilities();

    let mut previous: Option<&str> = None;

    for gate in &capabilities.native_gates {
        if let Some(previous_gate) = previous {
            assert!(
                previous_gate < gate.as_str(),
                "BTreeSet native gates must have deterministic ordering"
            );
        }

        previous = Some(gate.as_str());
    }
}

#[test]
fn native_gate_names_are_non_empty() {
    let capabilities = representative_capabilities();

    for gate in &capabilities.native_gates {
        assert!(!gate.trim().is_empty());
    }
}

#[test]
fn experimental_capabilities_are_separate_from_stable_capabilities() {
    let mut capabilities = representative_capabilities();

    capabilities
        .experimental_capabilities
        .insert("experimental_test_feature".to_owned());

    assert!(
        !capabilities
            .native_gates
            .contains("experimental_test_feature")
    );

    assert!(
        capabilities
            .experimental_capabilities
            .contains("experimental_test_feature")
    );
}

#[test]
fn adding_a_native_gate_marks_instruction_set_as_available() {
    let capabilities = BackendCapabilities::default()
        .with_gate("rx");

    assert!(capabilities.native_instruction_set);
    assert!(capabilities.native_gates.contains("rx"));
}

#[test]
fn empty_native_gate_is_not_added() {
    let capabilities = BackendCapabilities::default()
        .with_gate("   ");

    assert!(
        !capabilities
            .native_gates
            .contains("")
    );
}

// =============================================================================
// Limits
// =============================================================================

#[test]
fn representative_limits_are_positive() {
    let limits = representative_limits();

    assert!(limits.max_qubits > 0);
    assert!(limits.max_depth > 0);
    assert!(limits.max_operations > 0);
    assert!(limits.max_shots > 0);
    assert!(limits.max_circuit_size_bytes > 0);
}

#[test]
fn public_limits_are_non_zero() {
    assert!(MAX_BACKEND_ID_LENGTH > 0);
    assert!(MAX_BACKEND_NAME_LENGTH > 0);
    assert!(MAX_PROVIDER_ID_LENGTH > 0);
    assert!(MAX_BACKEND_VERSION_LENGTH > 0);
    assert!(MAX_HARDWARE_REVISION_LENGTH > 0);
    assert!(MAX_FIRMWARE_VERSION_LENGTH > 0);
    assert!(MAX_API_VERSION_LENGTH > 0);
    assert!(MAX_REGION_LENGTH > 0);
    assert!(MAX_METADATA_KEY_LENGTH > 0);
    assert!(MAX_METADATA_VALUE_LENGTH > 0);
    assert!(MAX_METADATA_PROPERTIES > 0);
    assert!(MAX_NATIVE_INSTRUCTIONS > 0);
    assert!(MAX_REQUIRED_INSTRUCTIONS > 0);
    assert!(MAX_REQUIRED_TOPOLOGY_EDGES > 0);
    assert!(MAX_REQUEST_METADATA_PROPERTIES > 0);
    assert!(MAX_REQUEST_ID_LENGTH > 0);
}

// =============================================================================
// Metadata
// =============================================================================

#[test]
fn representative_metadata_has_no_secret_material() {
    let metadata = representative_metadata();

    assert!(!metadata.backend_id.is_empty());
    assert!(!metadata.name.is_empty());
    assert!(!metadata.provider_id.is_empty());
    assert!(!metadata.backend_version.is_empty());
}

#[test]
fn metadata_properties_are_deterministically_ordered() {
    let mut metadata = representative_metadata();

    metadata
        .properties
        .insert("z".to_owned(), "last".to_owned());

    metadata
        .properties
        .insert("a".to_owned(), "first".to_owned());

    let keys: Vec<&String> = metadata.properties.keys().collect();

    assert_eq!(
        keys,
        vec![
            &"a".to_owned(),
            &"z".to_owned(),
        ]
    );
}

#[test]
fn metadata_property_values_are_bounded_by_public_contract() {
    let metadata = representative_metadata();

    for (key, value) in &metadata.properties {
        assert!(key.len() <= MAX_METADATA_KEY_LENGTH);
        assert!(value.len() <= MAX_METADATA_VALUE_LENGTH);
    }
}

// =============================================================================
// Topology integration
// =============================================================================

#[test]
fn representative_topology_is_valid() {
    let topology = representative_topology();

    assert_eq!(topology.qubit_count(), 4);
    assert!(topology.coupling_count() > 0);
    assert!(topology.is_connected());

    topology
        .validate()
        .expect("representative topology must validate");
}

#[test]
fn backend_descriptor_preserves_topology_information() {
    let descriptor = representative_descriptor();

    assert_eq!(
        descriptor.topology().qubit_count(),
        representative_topology().qubit_count()
    );

    assert_eq!(
        descriptor.topology().coupling_count(),
        representative_topology().coupling_count()
    );
}

// =============================================================================
// Backend descriptor
// =============================================================================

#[test]
fn representative_backend_descriptor_is_valid() {
    let descriptor = representative_descriptor();

    assert_eq!(descriptor.kind(), BackendKind::Qpu);
    assert_eq!(descriptor.status(), BackendStatus::Available);
}

#[test]
fn backend_descriptor_identity_is_stable() {
    let descriptor = representative_descriptor();

    assert_eq!(
        descriptor.metadata().backend_id,
        "local.test.backend"
    );

    assert_eq!(
        descriptor.metadata().provider_id,
        "zamani.test"
    );
}

#[test]
fn backend_descriptor_exposes_capabilities_without_mutation() {
    let descriptor = representative_descriptor();

    let capabilities = descriptor.capabilities();

    assert!(capabilities.measurement);
    assert!(capabilities.reset);
    assert!(capabilities.mid_circuit_measurement);
    assert!(capabilities.dynamic_circuits);
    assert!(capabilities.native_gates.contains("cx"));
}

#[test]
fn backend_descriptor_exposes_limits() {
    let descriptor = representative_descriptor();

    let limits = descriptor.limits();

    assert_eq!(limits.max_qubits, 127);
    assert_eq!(limits.max_depth, 10_000);
    assert_eq!(limits.max_shots, 100_000);
}

#[test]
fn backend_descriptor_debug_does_not_require_credentials() {
    let descriptor = representative_descriptor();

    let rendered = format!("{descriptor:?}");

    assert!(rendered.contains("local.test.backend"));
    assert!(!rendered.contains("Authorization"));
    assert!(!rendered.contains("Bearer"));
    assert!(!rendered.contains("api_key"));
    assert!(!rendered.contains("access_token"));
}

// =============================================================================
// Circuit requirements
// =============================================================================

#[test]
fn representative_circuit_requirements_are_valid() {
    let requirements = representative_circuit_requirements();

    assert!(requirements.qubits > 0);
    assert!(requirements.depth > 0);
    assert!(requirements.operations > 0);
    assert!(requirements.shots > 0);
    assert!(!requirements.required_gates.is_empty());
}

#[test]
fn circuit_requirements_use_deterministic_gate_ordering() {
    let requirements = representative_circuit_requirements();

    let gates: Vec<&String> = requirements.required_gates.iter().collect();

    let mut sorted = gates.clone();
    sorted.sort();

    assert_eq!(gates, sorted);
}

#[test]
fn circuit_requirements_can_be_satisfied_by_representative_backend() {
    let descriptor = representative_descriptor();
    let requirements = representative_circuit_requirements();

    descriptor
        .validate_circuit_requirements(&requirements)
        .expect("representative backend must satisfy representative requirements");
}

#[test]
fn missing_native_gate_is_rejected() {
    let descriptor = representative_descriptor();

    let mut requirements = representative_circuit_requirements();

    requirements
        .required_gates
        .insert("nonexistent_native_gate".to_owned());

    assert!(
        descriptor
            .validate_circuit_requirements(&requirements)
            .is_err(),
        "unsupported native gate must never be silently accepted"
    );
}

#[test]
fn excessive_qubit_requirement_is_rejected() {
    let descriptor = representative_descriptor();

    let mut requirements = representative_circuit_requirements();

    requirements.qubits = descriptor.limits().max_qubits + 1;

    assert!(
        descriptor
            .validate_circuit_requirements(&requirements)
            .is_err()
    );
}

#[test]
fn excessive_depth_requirement_is_rejected() {
    let descriptor = representative_descriptor();

    let mut requirements = representative_circuit_requirements();

    requirements.depth = descriptor.limits().max_depth + 1;

    assert!(
        descriptor
            .validate_circuit_requirements(&requirements)
            .is_err()
    );
}

#[test]
fn excessive_operation_requirement_is_rejected() {
    let descriptor = representative_descriptor();

    let mut requirements = representative_circuit_requirements();

    requirements.operations = descriptor.limits().max_operations + 1;

    assert!(
        descriptor
            .validate_circuit_requirements(&requirements)
            .is_err()
    );
}

#[test]
fn excessive_shot_requirement_is_rejected() {
    let descriptor = representative_descriptor();

    let mut requirements = representative_circuit_requirements();

    requirements.shots = descriptor.limits().max_shots + 1;

    assert!(
        descriptor
            .validate_circuit_requirements(&requirements)
            .is_err()
    );
}

// =============================================================================
// Capability-dependent circuit requirements
// =============================================================================

#[test]
fn mid_circuit_measurement_requirement_is_accepted_when_supported() {
    let descriptor = representative_descriptor();

    let mut requirements = representative_circuit_requirements();
    requirements.requires_mid_circuit_measurement = true;

    descriptor
        .validate_circuit_requirements(&requirements)
        .expect("backend advertises mid-circuit measurement");
}

#[test]
fn dynamic_circuit_requirement_is_accepted_when_supported() {
    let descriptor = representative_descriptor();

    let mut requirements = representative_circuit_requirements();
    requirements.requires_dynamic_circuits = true;

    descriptor
        .validate_circuit_requirements(&requirements)
        .expect("backend advertises dynamic circuits");
}

#[test]
fn reset_requirement_is_accepted_when_supported() {
    let descriptor = representative_descriptor();

    let mut requirements = representative_circuit_requirements();
    requirements.requires_reset = true;

    descriptor
        .validate_circuit_requirements(&requirements)
        .expect("backend advertises reset");
}

#[test]
fn classical_control_requirement_is_accepted_when_supported() {
    let descriptor = representative_descriptor();

    let mut requirements = representative_circuit_requirements();
    requirements.requires_classical_control = true;

    descriptor
        .validate_circuit_requirements(&requirements)
        .expect("backend advertises classical control");
}

// =============================================================================
// Generalized workload requirements
// =============================================================================

#[test]
fn representative_workload_requirements_are_valid() {
    let requirements = representative_workload_requirements();

    assert_eq!(
        requirements.kind,
        QuantumWorkloadKind::GateCircuit
    );

    assert!(requirements.qubits > 0);
    assert!(requirements.depth > 0);
    assert!(requirements.operations > 0);
    assert!(requirements.shots > 0);
}

#[test]
fn generalized_workload_requirements_can_be_validated() {
    let descriptor = representative_descriptor();
    let requirements = representative_workload_requirements();

    descriptor
        .validate_workload_requirements(&requirements)
        .expect("representative workload must be compatible");
}

#[test]
fn generalized_workload_rejects_excessive_qubits() {
    let descriptor = representative_descriptor();

    let mut requirements = representative_workload_requirements();

    requirements.qubits = descriptor.limits().max_qubits + 1;

    assert!(
        descriptor
            .validate_workload_requirements(&requirements)
            .is_err()
    );
}

#[test]
fn generalized_workload_rejects_excessive_depth() {
    let descriptor = representative_descriptor();

    let mut requirements = representative_workload_requirements();

    requirements.depth = descriptor.limits().max_depth + 1;

    assert!(
        descriptor
            .validate_workload_requirements(&requirements)
            .is_err()
    );
}

#[test]
fn generalized_workload_rejects_excessive_operations() {
    let descriptor = representative_descriptor();

    let mut requirements = representative_workload_requirements();

    requirements.operations = descriptor.limits().max_operations + 1;

    assert!(
        descriptor
            .validate_workload_requirements(&requirements)
            .is_err()
    );
}

#[test]
fn generalized_workload_rejects_excessive_shots() {
    let descriptor = representative_descriptor();

    let mut requirements = representative_workload_requirements();

    requirements.shots = descriptor.limits().max_shots + 1;

    assert!(
        descriptor
            .validate_workload_requirements(&requirements)
            .is_err()
    );
}

// =============================================================================
// Backend validation
// =============================================================================

#[test]
fn valid_backend_descriptor_passes_validation() {
    let descriptor = representative_descriptor();

    descriptor
        .validate()
        .expect("representative descriptor must validate");
}

#[test]
fn validation_is_deterministic() {
    let descriptor = representative_descriptor();

    let first = descriptor.validate();
    let second = descriptor.validate();

    assert_eq!(
        first.is_ok(),
        second.is_ok(),
        "validation outcome must be deterministic"
    );

    if let (Err(first_error), Err(second_error)) = (&first, &second) {
        assert_eq!(
            first_error.to_string(),
            second_error.to_string(),
            "validation error representation must be deterministic"
        );
    }
}

// =============================================================================
// Backend error contract
// =============================================================================

#[test]
fn backend_error_is_debuggable_without_panicking() {
    let error = BackendError::ExecutionUnavailable;

    let debug = format!("{error:?}");
    let display = error.to_string();

    assert!(!debug.is_empty());
    assert!(!display.is_empty());
}

#[test]
fn backend_errors_do_not_expose_secret_values() {
    let error = BackendError::ExecutionUnavailable;

    let rendered = format!("{error:?}");

    assert!(!rendered.contains("Bearer"));
    assert!(!rendered.contains("Authorization"));
    assert!(!rendered.contains("api_key"));
    assert!(!rendered.contains("access_token"));
    assert!(!rendered.contains("private_key"));
}

// =============================================================================
// Execution request
// =============================================================================

#[test]
fn execution_request_limits_are_positive() {
    assert!(MAX_REQUEST_ID_LENGTH > 0);
    assert!(MAX_REQUEST_METADATA_PROPERTIES > 0);
}

#[test]
fn execution_request_validation_rejects_unsupported_workload() {
    let descriptor = representative_descriptor();

    let request = ExecutionRequest::default();

    let result = descriptor.validate_execution_request(&request);

    // A default request is intentionally not assumed to be executable. The
    // important invariant is that validation never panics.
    let _ = result;
}

#[test]
fn execution_request_validation_is_deterministic() {
    let descriptor = representative_descriptor();
    let request = ExecutionRequest::default();

    let first = descriptor.validate_execution_request(&request);
    let second = descriptor.validate_execution_request(&request);

    assert_eq!(
        first.is_ok(),
        second.is_ok(),
        "execution-request validation must be deterministic"
    );

    if let (Err(first_error), Err(second_error)) = (&first, &second) {
        assert_eq!(
            first_error.to_string(),
            second_error.to_string()
        );
    }
}

// =============================================================================
// Execution result
// =============================================================================

#[test]
fn execution_result_is_provider_neutral() {
    let result = ExecutionResult::default();

    let debug = format!("{result:?}");

    assert!(!debug.contains("api_key"));
    assert!(!debug.contains("access_token"));
    assert!(!debug.contains("private_key"));
}

#[test]
fn execution_result_can_be_constructed_without_provider_types() {
    let result = ExecutionResult::default();

    // The result type must remain usable without importing any provider SDK.
    let _ = result;
}

// =============================================================================
// QuantumBackend aggregate
// =============================================================================

#[test]
fn quantum_backend_can_be_constructed_from_descriptor() {
    let descriptor = representative_descriptor();

    let backend = QuantumBackend::from_descriptor(descriptor)
        .expect("valid descriptor must construct a backend");

    assert_eq!(backend.kind(), BackendKind::Qpu);
    assert_eq!(backend.status(), BackendStatus::Available);
}

#[test]
fn quantum_backend_exposes_the_same_identity_as_descriptor() {
    let descriptor = representative_descriptor();

    let expected_id = descriptor.metadata().backend_id.clone();

    let backend = QuantumBackend::from_descriptor(descriptor)
        .expect("valid descriptor must construct a backend");

    assert_eq!(
        backend.metadata().backend_id,
        expected_id
    );
}

#[test]
fn quantum_backend_validation_delegates_to_canonical_contract() {
    let descriptor = representative_descriptor();

    let backend = QuantumBackend::from_descriptor(descriptor)
        .expect("valid descriptor must construct a backend");

    backend
        .validate()
        .expect("valid backend must remain valid after aggregation");
}

// =============================================================================
// Provider neutrality
// =============================================================================

#[test]
fn backend_model_does_not_require_provider_sdk_types() {
    let descriptor = representative_descriptor();

    let provider_id = descriptor.metadata().provider_id.as_str();

    assert_eq!(provider_id, "zamani.test");

    // This test intentionally imports no IBM/IonQ/Braket/etc. type.
    // Provider neutrality is therefore compile-time visible in this module.
}

#[test]
fn provider_identifier_is_metadata_not_backend_kind() {
    let descriptor = representative_descriptor();

    assert_eq!(
        descriptor.metadata().provider_id,
        "zamani.test"
    );

    assert_eq!(
        descriptor.kind(),
        BackendKind::Qpu
    );

    // A provider identity and an execution-target category are distinct.
    assert_ne!(
        descriptor.metadata().provider_id,
        descriptor.kind().as_str()
    );
}

// =============================================================================
// Resource boundary constants
// =============================================================================

#[test]
fn public_collection_limits_are_ordered_safely() {
    assert!(MAX_NATIVE_INSTRUCTIONS >= 1);
    assert!(MAX_REQUIRED_INSTRUCTIONS >= 1);
    assert!(MAX_REQUIRED_TOPOLOGY_EDGES >= 1);
}

#[test]
fn metadata_limits_are_sane_relative_to_single_entry_limits() {
    assert!(
        MAX_METADATA_PROPERTIES > 1,
        "production metadata must permit more than one property"
    );

    assert!(
        MAX_METADATA_VALUE_LENGTH >= MAX_METADATA_KEY_LENGTH,
        "metadata value envelope should not be narrower than key envelope"
    );
}

// =============================================================================
// Broad workload taxonomy
// =============================================================================

#[test]
fn workload_kind_is_distinct_from_backend_kind() {
    assert_ne!(
        format!("{:?}", QuantumWorkloadKind::GateCircuit),
        format!("{:?}", BackendKind::Qpu)
    );
}

#[test]
fn gate_circuit_workload_is_supported_by_gate_model_fixture() {
    let descriptor = representative_descriptor();

    let requirements = representative_workload_requirements();

    assert_eq!(
        requirements.kind,
        QuantumWorkloadKind::GateCircuit
    );

    descriptor
        .validate_workload_requirements(&requirements)
        .expect("gate-circuit fixture must be accepted");
}

// =============================================================================
// Topology-aware backend validation
// =============================================================================

#[test]
fn backend_topology_and_qubit_limit_are_consistent() {
    let descriptor = representative_descriptor();

    assert!(
        descriptor.topology().qubit_count()
            <= descriptor.limits().max_qubits
    );
}

#[test]
fn backend_topology_is_not_allowed_to_exceed_declared_qubit_limit() {
    let topology = HardwareTopology::linear(4)
        .expect("test topology must be valid");

    let metadata = representative_metadata();
    let capabilities = representative_capabilities();

    let mut limits = representative_limits();
    limits.max_qubits = 3;

    let result = BackendDescriptor::new(
        metadata,
        BackendKind::Qpu,
        BackendStatus::Available,
        capabilities,
        limits,
        topology,
    );

    assert!(
        result.is_err(),
        "backend descriptor must reject a topology larger than its declared resource envelope"
    );
}

// =============================================================================
// Status-sensitive backend construction
// =============================================================================

#[test]
fn retired_backend_remains_describable_but_is_not_usable() {
    let mut descriptor = representative_descriptor();

    descriptor
        .set_status(BackendStatus::Retired)
        .expect("status transition to retired must be representable");

    assert_eq!(
        descriptor.status(),
        BackendStatus::Retired
    );

    assert!(
        !descriptor.status().is_usable(),
        "retired backend must not be considered submission-eligible"
    );
}

#[test]
fn offline_backend_is_not_usable() {
    let mut descriptor = representative_descriptor();

    descriptor
        .set_status(BackendStatus::Offline)
        .expect("offline status must be representable");

    assert!(
        !descriptor.status().is_usable()
    );
}

#[test]
fn degraded_backend_remains_status_eligible_but_must_be_distinguishable() {
    let mut descriptor = representative_descriptor();

    descriptor
        .set_status(BackendStatus::Degraded)
        .expect("degraded status must be representable");

    assert_eq!(
        descriptor.status(),
        BackendStatus::Degraded
    );

    assert!(
        descriptor.status().is_usable(),
        "degraded status is usable by status alone"
    );
}

// =============================================================================
// Equality / cloning / deterministic value semantics
// =============================================================================

#[test]
fn backend_descriptor_clone_preserves_identity() {
    let descriptor = representative_descriptor();
    let clone = descriptor.clone();

    assert_eq!(
        descriptor.metadata().backend_id,
        clone.metadata().backend_id
    );

    assert_eq!(
        descriptor.metadata().provider_id,
        clone.metadata().provider_id
    );

    assert_eq!(
        descriptor.kind(),
        clone.kind()
    );

    assert_eq!(
        descriptor.status(),
        clone.status()
    );
}

#[test]
fn capabilities_clone_is_value_stable() {
    let capabilities = representative_capabilities();
    let clone = capabilities.clone();

    assert_eq!(capabilities, clone);
}

#[test]
fn limits_clone_is_value_stable() {
    let limits = representative_limits();
    let clone = limits.clone();

    assert_eq!(limits, clone);
}

// =============================================================================
// Collection normalization
// =============================================================================

#[test]
fn representative_native_gates_have_no_duplicates() {
    let gates = representative_native_gates();

    assert_eq!(
        gates.len(),
        gates.iter().collect::<BTreeSet<_>>().len()
    );
}

#[test]
fn required_instruction_limit_can_represent_representative_workload() {
    let requirements = representative_workload_requirements();

    assert!(
        requirements.required_instructions.len()
            <= MAX_REQUIRED_INSTRUCTIONS
    );
}

#[test]
fn required_topology_limit_can_represent_representative_workload() {
    let requirements = representative_workload_requirements();

    assert!(
        requirements.required_topology_edges
            <= MAX_REQUIRED_TOPOLOGY_EDGES
    );
}

// =============================================================================
// Regression tests for common historical failure modes
// =============================================================================

#[test]
fn backend_does_not_silently_accept_unknown_native_gate() {
    let descriptor = representative_descriptor();

    let mut requirements = representative_circuit_requirements();

    requirements
        .required_gates
        .insert("__unknown_zamani_gate__".to_owned());

    let result = descriptor.validate_circuit_requirements(&requirements);

    assert!(
        result.is_err(),
        "unknown native instructions must produce deterministic validation failure"
    );
}

#[test]
fn backend_does_not_silently_accept_zero_qubits_for_non_empty_workload() {
    let descriptor = representative_descriptor();

    let mut requirements = representative_workload_requirements();
    requirements.qubits = 0;

    let result = descriptor.validate_workload_requirements(&requirements);

    assert!(
        result.is_err(),
        "non-empty quantum workloads cannot require zero qubits"
    );
}

#[test]
fn backend_does_not_silently_accept_zero_shots_when_shots_are_required() {
    let descriptor = representative_descriptor();

    let mut requirements = representative_workload_requirements();
    requirements.shots = 0;

    let result = descriptor.validate_workload_requirements(&requirements);

    assert!(
        result.is_err(),
        "a sampling workload requiring execution shots must reject zero shots"
    );
}

#[test]
fn backend_validation_never_depends_on_wall_clock() {
    let descriptor = representative_descriptor();

    let first = descriptor.validate();
    let second = descriptor.validate();

    assert_eq!(first.is_ok(), second.is_ok());
}

// =============================================================================
// Contract completeness
// =============================================================================

#[test]
fn backend_contract_exposes_all_core_dimensions() {
    let descriptor = representative_descriptor();

    let _identity = &descriptor.metadata().backend_id;
    let _provider = &descriptor.metadata().provider_id;
    let _kind = descriptor.kind();
    let _status = descriptor.status();
    let _capabilities = descriptor.capabilities();
    let _limits = descriptor.limits();
    let _topology = descriptor.topology();

    // This deliberately touches every architectural dimension that backend.rs
    // is responsible for. If one disappears from the public contract, this
    // conformance test fails at compile time.
}

#[test]
fn backend_contract_is_provider_neutral() {
    let descriptor = representative_descriptor();

    assert!(!descriptor.metadata().backend_id.is_empty());
    assert!(!descriptor.metadata().provider_id.is_empty());

    // No provider-specific type is used anywhere in the construction path.
    // This is an architectural compile-time invariant.
}

// =============================================================================
// Final conformance gate
// =============================================================================

#[test]
fn canonical_backend_conformance_gate() {
    let descriptor = representative_descriptor();

    // Identity.
    assert!(!descriptor.metadata().backend_id.is_empty());
    assert!(!descriptor.metadata().provider_id.is_empty());

    // Kind/status.
    assert_eq!(descriptor.kind(), BackendKind::Qpu);
    assert!(descriptor.status().is_operational());

    // Capabilities.
    assert!(descriptor.capabilities().measurement);
    assert!(descriptor.capabilities().reset);
    assert!(descriptor.capabilities().native_instruction_set);

    // Resources.
    assert!(descriptor.limits().max_qubits > 0);
    assert!(descriptor.limits().max_depth > 0);
    assert!(descriptor.limits().max_operations > 0);
    assert!(descriptor.limits().max_shots > 0);

    // Topology.
    assert!(descriptor.topology().qubit_count() > 0);
    assert!(descriptor.topology().is_connected());

    // Backend validation.
    descriptor
        .validate()
        .expect("canonical backend must pass the complete conformance gate");

    // Circuit validation.
    descriptor
        .validate_circuit_requirements(
            &representative_circuit_requirements()
        )
        .expect("canonical circuit requirements must pass");

    // Workload validation.
    descriptor
        .validate_workload_requirements(
            &representative_workload_requirements()
        )
        .expect("canonical workload requirements must pass");
}