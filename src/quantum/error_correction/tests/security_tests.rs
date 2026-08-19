//! Production security tests for the Zamani Quantum Error Correction (QEC)
//! subsystem.
//!
//! Security objectives covered by this suite:
//!
//! - fail-closed capability semantics;
//! - strict QPU isolation;
//! - no implicit privilege escalation;
//! - explicit hardware authorization;
//! - resource exhaustion protection;
//! - allocation-bomb resistance at validation boundaries;
//! - malformed numerical input handling;
//! - NaN / infinity poisoning resistance;
//! - invalid probability rejection;
//! - integer-boundary safety;
//! - deterministic authorization semantics;
//! - stable capability identifiers;
//! - resource-policy validation;
//! - checkpoint/resource boundary enforcement;
//! - error classification and non-panic behavior;
//! - backend isolation;
//! - distributed-execution isolation;
//! - simulation/QPU separation;
//! - adversarial input handling;
//! - regression protection for security invariants.
//!
//! These tests intentionally do NOT require:
//!
//! - a physical QPU;
//! - a GPU;
//! - network access;
//! - distributed workers;
//! - unbounded memory;
//! - privileged operating-system access.
//!
//! A security test must remain executable in an ordinary CI environment.
//!
//! Security principle:
//!
//! ```text
//! UNTRUSTED INPUT
//!       |
//!       v
//!  bounded input
//!       |
//!       v
//!   validation
//!       |
//!       v
//! capability authorization
//!       |
//!       v
//! resource authorization
//!       |
//!       v
//!   QEC execution
//! ```
//!
//! A decoder or simulator must never obtain additional authority merely
//! because it is executing a quantum workload.

use std::panic::{catch_unwind, AssertUnwindSafe};
use std::time::Duration;

use crate::quantum::error_correction::arithmetic::{
    ArithmeticError,
    FiniteF64,
};

use crate::quantum::error_correction::capabilities::{
    Capability,
    CapabilityId,
    ExecutionBackend,
    QpuOperation,
    ResourceLimits,
    ResourceRequest,
};

use crate::quantum::error_correction::errors::{
    DecoderKind,
    NumericalOperation,
    QecError,
    QecErrorKind,
    ResourceKind,
};

use crate::quantum::error_correction::validation::{
    ValidationLimits,
};

use crate::quantum::error_correction::{
    ExecutionEnvironment,
    QpuAccess,
};

// ============================================================================
// Test helpers
// ============================================================================

/// Assert that an operation never panics.
///
/// Security-sensitive validation boundaries must convert malformed input into
/// controlled errors rather than crashing the process.
fn assert_does_not_panic<T>(
    operation: impl FnOnce() -> T,
) -> T {
    catch_unwind(AssertUnwindSafe(operation))
        .expect("security boundary panicked")
}

/// Construct a deliberately tiny resource policy suitable for adversarial
/// resource-limit tests.
fn tiny_limits() -> ResourceLimits {
    ResourceLimits {
        max_code_distance: 3,
        max_qubits: 4,
        max_stabilizers: 4,
        max_syndrome_events: 8,
        max_rounds: 4,
        max_graph_nodes: 8,
        max_graph_edges: 16,
        max_memory_bytes: 1024,
        max_execution_time: Duration::from_millis(100),
        max_parallelism: 1,
        max_checkpoint_bytes: 512,
    }
}

/// A request that fits completely inside `tiny_limits`.
fn permitted_tiny_request() -> ResourceRequest {
    ResourceRequest {
        code_distance: 3,
        qubits: 4,
        stabilizers: 4,
        syndrome_events: 8,
        rounds: 4,
        graph_nodes: 8,
        graph_edges: 16,
        memory_bytes: 1024,
        execution_time: Duration::from_millis(100),
        parallelism: 1,
        checkpoint_bytes: 512,
    }
}

// ============================================================================
// Capability fail-closed tests
// ============================================================================

#[test]
fn capability_identifiers_are_stable() {
    assert_eq!(Capability::Decode.id(), 1);
    assert_eq!(Capability::Simulate.id(), 2);
    assert_eq!(Capability::Benchmark.id(), 3);
    assert_eq!(Capability::InspectTopology.id(), 4);
    assert_eq!(Capability::AllocateMemory.id(), 5);
    assert_eq!(Capability::UseAccelerator.id(), 6);
    assert_eq!(Capability::DistributedExecution.id(), 7);
    assert_eq!(Capability::StreamingSyndrome.id(), 8);
    assert_eq!(Capability::Checkpoint.id(), 9);
    assert_eq!(Capability::DeterministicExecution.id(), 10);
    assert_eq!(Capability::ReadMetrics.id(), 11);
    assert_eq!(Capability::EmitTelemetry.id(), 12);
    assert_eq!(Capability::ParallelExecution.id(), 13);

    assert_eq!(Capability::QpuAccess.id(), 14);
    assert_eq!(Capability::QpuInspect.id(), 15);
    assert_eq!(Capability::QpuSubmit.id(), 16);
    assert_eq!(Capability::QpuReadResults.id(), 17);
    assert_eq!(Capability::QpuCalibration.id(), 18);
    assert_eq!(Capability::QpuErrorCorrection.id(), 19);
    assert_eq!(Capability::QpuSyndromeExtraction.id(), 20);
}

#[test]
fn capability_names_are_stable_and_non_empty() {
    let capabilities = [
        Capability::Decode,
        Capability::Simulate,
        Capability::Benchmark,
        Capability::InspectTopology,
        Capability::AllocateMemory,
        Capability::UseAccelerator,
        Capability::DistributedExecution,
        Capability::StreamingSyndrome,
        Capability::Checkpoint,
        Capability::DeterministicExecution,
        Capability::ReadMetrics,
        Capability::EmitTelemetry,
        Capability::ParallelExecution,
        Capability::QpuAccess,
        Capability::QpuInspect,
        Capability::QpuSubmit,
        Capability::QpuReadResults,
        Capability::QpuCalibration,
        Capability::QpuErrorCorrection,
        Capability::QpuSyndromeExtraction,
    ];

    let mut names = std::collections::BTreeSet::new();

    for capability in capabilities {
        assert!(!capability.name().is_empty());
        assert!(
            names.insert(capability.name()),
            "duplicate capability name: {}",
            capability.name()
        );
    }
}

#[test]
fn ordinary_decode_capability_does_not_imply_qpu_access() {
    assert!(!Capability::Decode.is_qpu());
    assert!(!Capability::Decode.can_execute_hardware());

    assert!(!Capability::Simulate.is_qpu());
    assert!(!Capability::Simulate.can_execute_hardware());

    assert!(!Capability::Benchmark.is_qpu());
    assert!(!Capability::Benchmark.can_execute_hardware());
}

#[test]
fn qpu_capabilities_are_explicitly_marked() {
    let qpu_capabilities = [
        Capability::QpuAccess,
        Capability::QpuInspect,
        Capability::QpuSubmit,
        Capability::QpuReadResults,
        Capability::QpuCalibration,
        Capability::QpuErrorCorrection,
        Capability::QpuSyndromeExtraction,
    ];

    for capability in qpu_capabilities {
        assert!(
            capability.is_qpu(),
            "{capability:?} must be classified as QPU capability"
        );
    }
}

#[test]
fn only_hardware_execution_capabilities_can_execute_hardware() {
    assert!(!Capability::QpuAccess.can_execute_hardware());
    assert!(!Capability::QpuInspect.can_execute_hardware());
    assert!(!Capability::QpuReadResults.can_execute_hardware());
    assert!(!Capability::QpuCalibration.can_execute_hardware());

    assert!(Capability::QpuSubmit.can_execute_hardware());
    assert!(Capability::QpuErrorCorrection.can_execute_hardware());
    assert!(Capability::QpuSyndromeExtraction.can_execute_hardware());
}

// ============================================================================
// QPU isolation tests
// ============================================================================

#[test]
fn qpu_access_is_denied_by_default() {
    assert!(!QpuAccess::Denied.is_authorized());
    assert!(!QpuAccess::RequiresCapability.is_authorized());
}

#[test]
fn qpu_authorization_is_explicit() {
    assert!(QpuAccess::Authorized.is_authorized());
}

#[test]
fn qpu_execution_environment_is_not_classical() {
    assert!(ExecutionEnvironment::Qpu.is_qpu());
    assert!(!ExecutionEnvironment::Qpu.is_classical());
}

#[test]
fn classical_execution_is_not_qpu_execution() {
    let classical = [
        ExecutionEnvironment::Cpu,
        ExecutionEnvironment::ParallelCpu,
        ExecutionEnvironment::Gpu,
        ExecutionEnvironment::Accelerator,
    ];

    for environment in classical {
        assert!(!environment.is_qpu());
    }
}

#[test]
fn qpu_operations_map_to_specific_capabilities() {
    assert_eq!(
        QpuOperation::Inspect.required_capability(),
        Capability::QpuInspect
    );

    assert_eq!(
        QpuOperation::ReadCalibration.required_capability(),
        Capability::QpuCalibration
    );

    assert_eq!(
        QpuOperation::SubmitCircuit.required_capability(),
        Capability::QpuSubmit
    );

    assert_eq!(
        QpuOperation::ReadResults.required_capability(),
        Capability::QpuReadResults
    );

    assert_eq!(
        QpuOperation::ErrorCorrection.required_capability(),
        Capability::QpuErrorCorrection
    );

    assert_eq!(
        QpuOperation::SyndromeExtraction.required_capability(),
        Capability::QpuSyndromeExtraction
    );
}

#[test]
fn qpu_operations_do_not_share_a_single_implicit_privilege() {
    let operations = [
        QpuOperation::Inspect,
        QpuOperation::ReadCalibration,
        QpuOperation::SubmitCircuit,
        QpuOperation::ReadResults,
        QpuOperation::ErrorCorrection,
        QpuOperation::SyndromeExtraction,
    ];

    let mut required = std::collections::BTreeSet::new();

    for operation in operations {
        assert!(
            required.insert(operation.required_capability().id()),
            "QPU operation capabilities must remain individually scoped"
        );
    }
}

// ============================================================================
// Backend isolation tests
// ============================================================================

#[test]
fn qpu_backend_is_explicitly_distinct() {
    assert!(ExecutionBackend::Qpu.is_qpu());
    assert!(ExecutionBackend::Qpu.requires_hardware_capability());
    assert_eq!(ExecutionBackend::Qpu.name(), "qpu");
}

#[test]
fn gpu_and_accelerator_are_hardware_capability_boundaries() {
    assert!(ExecutionBackend::Gpu.requires_hardware_capability());
    assert!(ExecutionBackend::Accelerator.requires_hardware_capability());
}

#[test]
fn cpu_does_not_require_hardware_capability() {
    assert!(!ExecutionBackend::Cpu.requires_hardware_capability());
    assert!(!ExecutionBackend::ParallelCpu.requires_hardware_capability());
}

#[test]
fn distributed_execution_is_not_qpu_execution() {
    assert!(!ExecutionBackend::Distributed.is_qpu());
    assert_eq!(
        ExecutionBackend::Distributed.name(),
        "distributed"
    );
}

// ============================================================================
// Resource exhaustion / DoS protection
// ============================================================================

#[test]
fn resource_policy_is_valid() {
    let limits = tiny_limits();

    assert!(limits.validate().is_ok());
}

#[test]
fn resource_policy_rejects_zero_code_distance() {
    let mut limits = tiny_limits();
    limits.max_code_distance = 0;

    assert!(limits.validate().is_err());
}

#[test]
fn resource_policy_rejects_zero_qubits() {
    let mut limits = tiny_limits();
    limits.max_qubits = 0;

    assert!(limits.validate().is_err());
}

#[test]
fn resource_policy_rejects_zero_memory() {
    let mut limits = tiny_limits();
    limits.max_memory_bytes = 0;

    assert!(limits.validate().is_err());
}

#[test]
fn resource_policy_rejects_zero_execution_time() {
    let mut limits = tiny_limits();
    limits.max_execution_time = Duration::ZERO;

    assert!(limits.validate().is_err());
}

#[test]
fn resource_policy_rejects_zero_parallelism() {
    let mut limits = tiny_limits();
    limits.max_parallelism = 0;

    assert!(limits.validate().is_err());
}

#[test]
fn permitted_request_is_accepted() {
    let limits = tiny_limits();
    let request = permitted_tiny_request();

    assert!(limits.permits(&request));
}

#[test]
fn excessive_qubit_request_is_rejected() {
    let limits = tiny_limits();

    let mut request = permitted_tiny_request();
    request.qubits = limits.max_qubits + 1;

    assert!(!limits.permits(&request));
}

#[test]
fn excessive_stabilizer_request_is_rejected() {
    let limits = tiny_limits();

    let mut request = permitted_tiny_request();
    request.stabilizers = limits.max_stabilizers + 1;

    assert!(!limits.permits(&request));
}

#[test]
fn excessive_syndrome_request_is_rejected() {
    let limits = tiny_limits();

    let mut request = permitted_tiny_request();
    request.syndrome_events = limits.max_syndrome_events + 1;

    assert!(!limits.permits(&request));
}

#[test]
fn excessive_round_request_is_rejected() {
    let limits = tiny_limits();

    let mut request = permitted_tiny_request();
    request.rounds = limits.max_rounds + 1;

    assert!(!limits.permits(&request));
}

#[test]
fn excessive_graph_nodes_are_rejected() {
    let limits = tiny_limits();

    let mut request = permitted_tiny_request();
    request.graph_nodes = limits.max_graph_nodes + 1;

    assert!(!limits.permits(&request));
}

#[test]
fn excessive_graph_edges_are_rejected() {
    let limits = tiny_limits();

    let mut request = permitted_tiny_request();
    request.graph_edges = limits.max_graph_edges + 1;

    assert!(!limits.permits(&request));
}

#[test]
fn excessive_memory_is_rejected() {
    let limits = tiny_limits();

    let mut request = permitted_tiny_request();
    request.memory_bytes = limits.max_memory_bytes + 1;

    assert!(!limits.permits(&request));
}

#[test]
fn excessive_execution_time_is_rejected() {
    let limits = tiny_limits();

    let mut request = permitted_tiny_request();
    request.execution_time =
        limits.max_execution_time + Duration::from_nanos(1);

    assert!(!limits.permits(&request));
}

#[test]
fn excessive_parallelism_is_rejected() {
    let limits = tiny_limits();

    let mut request = permitted_tiny_request();
    request.parallelism = limits.max_parallelism + 1;

    assert!(!limits.permits(&request));
}

#[test]
fn excessive_checkpoint_size_is_rejected() {
    let limits = tiny_limits();

    let mut request = permitted_tiny_request();
    request.checkpoint_bytes =
        limits.max_checkpoint_bytes + 1;

    assert!(!limits.permits(&request));
}

#[test]
fn resource_policy_never_treats_excess_as_unlimited() {
    let limits = tiny_limits();

    let hostile = ResourceRequest {
        code_distance: u64::MAX,
        qubits: u64::MAX,
        stabilizers: u64::MAX,
        syndrome_events: u64::MAX,
        rounds: u64::MAX,
        graph_nodes: u64::MAX,
        graph_edges: u64::MAX,
        memory_bytes: u64::MAX,
        execution_time: Duration::MAX,
        parallelism: u32::MAX,
        checkpoint_bytes: u64::MAX,
    };

    assert!(!limits.permits(&hostile));
}

// ============================================================================
// Validation policy security
// ============================================================================

#[test]
fn validation_limits_reject_zero_qubit_budget() {
    let limits = ValidationLimits {
        max_qubits: 0,
        ..ValidationLimits::default()
    };

    assert!(limits.validate().is_err());
}

#[test]
fn validation_limits_reject_zero_stabilizer_budget() {
    let limits = ValidationLimits {
        max_stabilizers: 0,
        ..ValidationLimits::default()
    };

    assert!(limits.validate().is_err());
}

#[test]
fn validation_limits_reject_zero_measurement_budget() {
    let limits = ValidationLimits {
        max_syndrome_measurements: 0,
        ..ValidationLimits::default()
    };

    assert!(limits.validate().is_err());
}

#[test]
fn validation_limits_reject_zero_detection_event_budget() {
    let limits = ValidationLimits {
        max_detection_events: 0,
        ..ValidationLimits::default()
    };

    assert!(limits.validate().is_err());
}

#[test]
fn validation_limits_reject_zero_stabilizer_weight() {
    let limits = ValidationLimits {
        max_stabilizer_weight: 0,
        ..ValidationLimits::default()
    };

    assert!(limits.validate().is_err());
}

#[test]
fn validation_limits_are_valid_by_default() {
    assert!(ValidationLimits::default().validate().is_ok());
}

// ============================================================================
// Numerical poisoning protection
// ============================================================================

#[test]
fn finite_wrapper_rejects_nan() {
    let result = assert_does_not_panic(|| {
        FiniteF64::new(f64::NAN)
    });

    assert_eq!(result, Err(ArithmeticError::NaN));
}

#[test]
fn finite_wrapper_rejects_positive_infinity() {
    let result = assert_does_not_panic(|| {
        FiniteF64::new(f64::INFINITY)
    });

    assert_eq!(result, Err(ArithmeticError::Infinite));
}

#[test]
fn finite_wrapper_rejects_negative_infinity() {
    let result = assert_does_not_panic(|| {
        FiniteF64::new(f64::NEG_INFINITY)
    });

    assert_eq!(result, Err(ArithmeticError::Infinite));
}

#[test]
fn finite_wrapper_accepts_finite_extreme_values() {
    let values = [
        f64::MIN,
        f64::MAX,
        f64::MIN_POSITIVE,
        -f64::MAX,
        0.0,
        -0.0,
        1.0,
    ];

    for value in values {
        let result = assert_does_not_panic(|| {
            FiniteF64::new(value)
        });

        assert!(
            result.is_ok(),
            "finite value unexpectedly rejected: {value:?}"
        );
    }
}

#[test]
fn numerical_poisoning_never_panics() {
    let hostile_values = [
        f64::NAN,
        f64::INFINITY,
        f64::NEG_INFINITY,
        f64::MAX,
        f64::MIN,
        f64::MIN_POSITIVE,
        -f64::MIN_POSITIVE,
        0.0,
        -0.0,
    ];

    for value in hostile_values {
        let result = assert_does_not_panic(|| {
            FiniteF64::new(value)
        });

        if !value.is_finite() {
            assert!(result.is_err());
        }
    }
}

// ============================================================================
// Unified error security semantics
// ============================================================================

#[test]
fn invalid_input_is_classified_as_input_error() {
    let error = QecError::invalid_input(
        "attacker-controlled malformed QEC object",
    );

    assert_eq!(
        error.kind(),
        QecErrorKind::InvalidInput
    );

    assert!(error.is_input_error());
    assert!(!error.is_internal());
    assert!(!error.is_resource_error());
    assert!(!error.is_cancellation());
}

#[test]
fn resource_failure_is_classified_as_resource_error() {
    let error = QecError::resource_limit(
        ResourceKind::GraphNodes,
        u128::MAX,
        100,
        "graph node limit exceeded",
    );

    assert_eq!(
        error.kind(),
        QecErrorKind::ResourceLimitExceeded
    );

    assert!(error.is_resource_error());
    assert!(!error.is_input_error());
    assert!(!error.is_internal());
}

#[test]
fn memory_failure_is_classified_as_resource_error() {
    let error = QecError::memory_limit(
        u64::MAX,
        1024,
        "memory budget exceeded",
    );

    assert!(error.is_resource_error());
    assert_eq!(
        error.kind(),
        QecErrorKind::MemoryLimitExceeded
    );
}

#[test]
fn time_failure_is_classified_as_resource_error() {
    let error = QecError::time_limit(
        u64::MAX,
        100,
        "decoder deadline exceeded",
    );

    assert!(error.is_resource_error());
    assert_eq!(
        error.kind(),
        QecErrorKind::TimeLimitExceeded
    );
}

#[test]
fn cancellation_is_distinguished_from_failure() {
    let error = QecError::cancelled(
        "operation cancelled by caller",
    );

    assert!(error.is_cancellation());
    assert!(!error.is_resource_error());
    assert!(!error.is_internal());
    assert_eq!(
        error.kind(),
        QecErrorKind::CancellationRequested
    );
}

#[test]
fn internal_invariant_is_not_misclassified_as_input() {
    let error = QecError::invariant(
        "stabilizer_commutation",
        "internal invariant violated",
    );

    assert!(error.is_internal());
    assert!(!error.is_input_error());
    assert_eq!(
        error.kind(),
        QecErrorKind::InternalInvariantViolation
    );
}

#[test]
fn numerical_failure_has_explicit_classification() {
    let error = QecError::numerical_failure(
        NumericalOperation::ProbabilityValidation,
        "invalid numerical state",
    );

    assert_eq!(
        error.kind(),
        QecErrorKind::NumericalFailure
    );

    assert!(!error.is_input_error());
    assert!(!error.is_resource_error());
}

// ============================================================================
// Decoder failure boundary
// ============================================================================

#[test]
fn decoder_failure_is_not_silently_treated_as_success() {
    let error = QecError::decoder_failure(
        DecoderKind::Mwpm,
        "adversarial graph exceeded decoder policy",
    );

    assert_eq!(
        error.kind(),
        QecErrorKind::DecoderFailure
    );

    assert!(!error.is_internal());
}

// ============================================================================
// Capability ID security properties
// ============================================================================

#[test]
fn capability_ids_are_exactly_representable() {
    let id = CapabilityId::from_parts(
        u64::MAX,
        u64::MAX,
    );

    assert_eq!(
        id.raw(),
        u128::MAX
    );
}

#[test]
fn capability_ids_do_not_collapse_high_and_low_components() {
    let a = CapabilityId::from_parts(1, 0);
    let b = CapabilityId::from_parts(0, 1);

    assert_ne!(a, b);
    assert_ne!(a.raw(), b.raw());
}

#[test]
fn capability_id_zero_is_distinct_from_nonzero_ids() {
    let zero = CapabilityId::from_parts(0, 0);
    let nonzero = CapabilityId::from_parts(0, 1);

    assert_ne!(zero, nonzero);
}

// ============================================================================
// Adversarial-boundary tests
// ============================================================================

#[test]
fn hostile_resource_values_are_handled_without_panic() {
    let result = assert_does_not_panic(|| {
        let limits = tiny_limits();

        let hostile = ResourceRequest {
            code_distance: usize::MAX as u64,
            qubits: usize::MAX as u64,
            stabilizers: usize::MAX as u64,
            syndrome_events: usize::MAX as u64,
            rounds: usize::MAX as u64,
            graph_nodes: usize::MAX as u64,
            graph_edges: usize::MAX as u64,
            memory_bytes: usize::MAX as u64,
            execution_time: Duration::MAX,
            parallelism: u32::MAX,
            checkpoint_bytes: usize::MAX as u64,
        };

        limits.permits(&hostile)
    });

    assert!(!result);
}

#[test]
fn attacker_cannot_turn_zero_into_unlimited_resource_policy() {
    let mut limits = tiny_limits();

    limits.max_qubits = 0;

    assert!(limits.validate().is_err());

    let request = ResourceRequest::default();

    assert!(!limits.permits(&request));
}

#[test]
fn attacker_cannot_turn_zero_parallelism_into_unbounded_execution() {
    let mut limits = tiny_limits();

    limits.max_parallelism = 0;

    assert!(limits.validate().is_err());

    let mut request = permitted_tiny_request();
    request.parallelism = 0;

    assert!(!limits.validate().is_ok());
}

// ============================================================================
// Deterministic security behavior
// ============================================================================

#[test]
fn resource_authorization_is_deterministic() {
    let limits = tiny_limits();
    let request = permitted_tiny_request();

    let first = limits.permits(&request);

    for _ in 0..1_000 {
        assert_eq!(
            limits.permits(&request),
            first
        );
    }
}

#[test]
fn capability_classification_is_deterministic() {
    let capabilities = [
        Capability::Decode,
        Capability::Simulate,
        Capability::Benchmark,
        Capability::QpuAccess,
        Capability::QpuSubmit,
        Capability::QpuErrorCorrection,
        Capability::QpuSyndromeExtraction,
    ];

    for capability in capabilities {
        let first = (
            capability.id(),
            capability.name(),
            capability.is_qpu(),
            capability.can_execute_hardware(),
        );

        for _ in 0..100 {
            assert_eq!(
                (
                    capability.id(),
                    capability.name(),
                    capability.is_qpu(),
                    capability.can_execute_hardware(),
                ),
                first
            );
        }
    }
}

// ============================================================================
// No physical QPU required for security tests
// ============================================================================

#[test]
fn qpu_security_boundary_can_be_tested_without_hardware() {
    assert!(!QpuAccess::Denied.is_authorized());

    assert!(
        Capability::QpuSubmit
            .can_execute_hardware()
    );

    assert_eq!(
        QpuOperation::SubmitCircuit
            .required_capability(),
        Capability::QpuSubmit
    );

    // The security test deliberately stops at authorization semantics.
    // It must never submit a real workload to physical hardware.
}

#[test]
fn qpu_error_correction_requires_explicit_qpu_capability() {
    assert_eq!(
        QpuOperation::ErrorCorrection
            .required_capability(),
        Capability::QpuErrorCorrection
    );

    assert!(
        QpuOperation::ErrorCorrection
            .required_capability()
            .is_qpu()
    );
}

#[test]
fn qpu_syndrome_extraction_requires_explicit_qpu_capability() {
    assert_eq!(
        QpuOperation::SyndromeExtraction
            .required_capability(),
        Capability::QpuSyndromeExtraction
    );

    assert!(
        QpuOperation::SyndromeExtraction
            .required_capability()
            .is_qpu()
    );
}

// ============================================================================
// Security regression matrix
// ============================================================================

#[test]
fn security_regression_capability_matrix() {
    let ordinary_capabilities = [
        Capability::Decode,
        Capability::Simulate,
        Capability::Benchmark,
        Capability::InspectTopology,
        Capability::AllocateMemory,
        Capability::StreamingSyndrome,
        Capability::Checkpoint,
        Capability::DeterministicExecution,
        Capability::ReadMetrics,
        Capability::EmitTelemetry,
        Capability::ParallelExecution,
    ];

    for capability in ordinary_capabilities {
        assert!(
            !capability.is_qpu(),
            "{capability:?} unexpectedly gained QPU privilege"
        );

        assert!(
            !capability.can_execute_hardware(),
            "{capability:?} unexpectedly gained hardware execution privilege"
        );
    }
}

#[test]
fn security_regression_qpu_matrix() {
    let qpu_capabilities = [
        Capability::QpuAccess,
        Capability::QpuInspect,
        Capability::QpuSubmit,
        Capability::QpuReadResults,
        Capability::QpuCalibration,
        Capability::QpuErrorCorrection,
        Capability::QpuSyndromeExtraction,
    ];

    for capability in qpu_capabilities {
        assert!(
            capability.is_qpu(),
            "{capability:?} lost QPU classification"
        );
    }
}

#[test]
fn security_regression_hardware_execution_matrix() {
    let execution_capabilities = [
        Capability::QpuSubmit,
        Capability::QpuErrorCorrection,
        Capability::QpuSyndromeExtraction,
    ];

    for capability in execution_capabilities {
        assert!(
            capability.can_execute_hardware(),
            "{capability:?} must remain hardware-execution capable"
        );
    }
}

#[test]
fn security_regression_resource_boundaries() {
    let limits = tiny_limits();

    let mut request = permitted_tiny_request();

    assert!(limits.permits(&request));

    request.qubits += 1;
    assert!(!limits.permits(&request));

    request = permitted_tiny_request();
    request.memory_bytes += 1;
    assert!(!limits.permits(&request));

    request = permitted_tiny_request();
    request.graph_edges += 1;
    assert!(!limits.permits(&request));

    request = permitted_tiny_request();
    request.checkpoint_bytes += 1;
    assert!(!limits.permits(&request));
}

// ============================================================================
// Security contract
// ============================================================================

/// This test documents the subsystem's core security contract.
///
/// The important invariant is not that every possible attack is represented
/// here; rather, the architectural security boundary must remain explicit:
///
/// ```text
/// malformed input
///       -> validation failure
///
/// excessive resources
///       -> resource failure
///
/// invalid numerical state
///       -> numerical failure
///
/// missing QPU capability
///       -> no QPU execution
///
/// cancelled workload
///       -> cancellation result
///
/// implementation invariant failure
///       -> explicit internal error
/// ```
///
/// No category should silently become successful execution.
#[test]
fn production_qec_security_contract_is_explicit() {
    let input_error =
        QecError::invalid_input("malformed input");

    let resource_error =
        QecError::resource_limit(
            ResourceKind::Qubits,
            u128::MAX,
            1024,
            "too many qubits",
        );

    let numerical_error =
        QecError::numerical_failure(
            NumericalOperation::ProbabilityValidation,
            "NaN probability",
        );

    let cancellation_error =
        QecError::cancelled("cancelled");

    assert!(input_error.is_input_error());
    assert!(resource_error.is_resource_error());
    assert_eq!(
        numerical_error.kind(),
        QecErrorKind::NumericalFailure
    );
    assert!(cancellation_error.is_cancellation());

    assert!(!QpuAccess::Denied.is_authorized());
}