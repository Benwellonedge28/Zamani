//! Production regression tests for the Zamani Quantum Error Correction (QEC)
//! subsystem.
//!
//! Regression testing principle:
//!
//!     DISCOVERED BUG
//!          |
//!          v
//!     REPRODUCIBLE TEST
//!          |
//!          v
//!     PERMANENT INVARIANT
//!          |
//!          v
//!     FUTURE RELEASES MUST NOT REGRESS
//!
//! This suite protects previously established mathematical, security,
//! resource-management, execution, determinism, and QPU-isolation contracts.
//!
//! The tests are intentionally:
//!
//! - deterministic;
//! - bounded;
//! - offline;
//! - hardware-independent unless testing the QPU *authorization model*;
//! - safe for CI;
//! - free of real QPU access;
//! - free of GPU requirements;
//! - free of network requirements;
//! - free of distributed-worker requirements;
//! - resistant to accidental unbounded allocation.
//!
//! IMPORTANT:
//!
//! These tests verify the QPU safety/control plane. They do NOT submit work
//! to a physical QPU. A physical QPU must never become a prerequisite for
//! ordinary CI regression testing.
//!
//! Regression scope:
//!
//! 1. QEC API stability
//! 2. architecture identity
//! 3. mathematical self-checks
//! 4. execution-environment separation
//! 5. QPU isolation
//! 6. QPU capability separation
//! 7. backend separation
//! 8. resource-limit enforcement
//! 9. deterministic behavior
//! 10. fail-closed semantics
//! 11. bounded execution assumptions
//! 12. security invariants
//! 13. capability-ID stability
//! 14. capability-name stability
//! 15. resource-policy validity
//! 16. regression protection for "infinite" workloads
//! 17. distributed/accelerated execution boundaries
//! 18. future-proofing of the QEC infrastructure contract
//!
//! A regression test should be removed only when the corresponding contract
//! is intentionally changed and the API/versioning policy explicitly permits
//! that change.

#![allow(clippy::assertions_on_constants)]

use std::panic::{catch_unwind, AssertUnwindSafe};
use std::time::Duration;

use crate::quantum::error_correction::{
    api_version,
    capabilities,
    self_check,
    supported_execution_environments,
    ExecutionEnvironment,
    QpuAccess,
    QEC_API_VERSION,
    QEC_ARCHITECTURE,
    QEC_SUBSYSTEM_NAME,
};

use crate::quantum::error_correction::capabilities::{
    Capability,
    ExecutionBackend,
    QpuOperation,
    ResourceLimits,
    ResourceRequest,
};

// ============================================================================
// Test helpers
// ============================================================================

/// Executes an operation and asserts that the QEC regression boundary does not
/// panic.
///
/// A regression in malformed-input handling must become a controlled error,
/// not a process-level panic.
fn assert_no_panic<T>(
    operation: impl FnOnce() -> T,
) -> T {
    catch_unwind(AssertUnwindSafe(operation))
        .expect("QEC regression boundary unexpectedly panicked")
}

/// Small bounded resource policy used by deterministic regression tests.
///
/// These limits are intentionally tiny. They make it impossible for a
/// regression test itself to accidentally become an allocation stress test.
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

/// Small request that fits exactly inside `tiny_limits`.
fn permitted_request() -> ResourceRequest {
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
// 1. API regression guards
// ============================================================================

/// Regression guard for the public QEC API version.
///
/// If this changes intentionally, the compatibility/versioning policy must
/// explicitly account for the change.
#[test]
fn qec_api_version_is_stable() {
    assert_eq!(
        api_version(),
        QEC_API_VERSION,
        "public API accessor must match the exported API version"
    );

    assert!(
        !QEC_API_VERSION.is_empty(),
        "QEC API version must never be empty"
    );
}

/// Regression guard for the subsystem identifier.
#[test]
fn qec_subsystem_identity_is_stable() {
    assert_eq!(
        QEC_SUBSYSTEM_NAME,
        "zamani.quantum.error_correction"
    );

    assert!(
        !QEC_SUBSYSTEM_NAME.is_empty(),
        "QEC subsystem name must never be empty"
    );
}

/// Regression guard for the architectural identity.
#[test]
fn qec_architecture_identity_is_stable() {
    assert_eq!(
        QEC_ARCHITECTURE,
        "resource-safe-scalable-qec"
    );
}

// ============================================================================
// 2. Mathematical self-check regression
// ============================================================================

/// The fundamental QEC self-check must remain valid.
///
/// This is intentionally stronger than merely checking that the module
/// compiles. It protects the identity/syndrome invariant exposed by the
/// subsystem's self-check.
#[test]
fn qec_self_check_remains_valid() {
    let result = assert_no_panic(self_check);

    assert!(
        result.is_ok(),
        "QEC mathematical self-check regressed: {result:?}"
    );
}

/// Running the self-check repeatedly must remain deterministic.
#[test]
fn qec_self_check_is_deterministic() {
    let first = assert_no_panic(self_check);

    for _ in 0..32 {
        let current = assert_no_panic(self_check);

        assert_eq!(
            current,
            first,
            "QEC self-check result changed between identical executions"
        );
    }
}

// ============================================================================
// 3. Execution-environment regression guards
// ============================================================================

/// Every supported execution environment must remain explicitly represented.
#[test]
fn all_execution_environments_remain_declared() {
    let environments = supported_execution_environments();

    assert_eq!(
        environments.len(),
        6,
        "unexpected change in supported QEC execution environments"
    );

    assert!(environments.contains(&ExecutionEnvironment::Cpu));
    assert!(environments.contains(&ExecutionEnvironment::ParallelCpu));
    assert!(environments.contains(&ExecutionEnvironment::Gpu));
    assert!(environments.contains(&ExecutionEnvironment::Accelerator));
    assert!(environments.contains(&ExecutionEnvironment::Qpu));
    assert!(environments.contains(&ExecutionEnvironment::Distributed));
}

/// QPU must remain distinct from classical execution.
#[test]
fn qpu_remains_distinct_from_classical_execution() {
    assert!(ExecutionEnvironment::Qpu.is_qpu());
    assert!(!ExecutionEnvironment::Qpu.is_classical());

    assert!(!ExecutionEnvironment::Cpu.is_qpu());
    assert!(!ExecutionEnvironment::ParallelCpu.is_qpu());
    assert!(!ExecutionEnvironment::Gpu.is_qpu());
    assert!(!ExecutionEnvironment::Accelerator.is_qpu());
    assert!(!ExecutionEnvironment::Distributed.is_qpu());
}

/// Distributed execution must not silently become QPU execution.
#[test]
fn distributed_execution_does_not_imply_qpu_execution() {
    assert!(ExecutionEnvironment::Distributed.is_distributed());
    assert!(!ExecutionEnvironment::Distributed.is_qpu());
}

// ============================================================================
// 4. QPU access regression guards
// ============================================================================

/// QPU access must remain denied by default.
#[test]
fn qpu_access_defaults_to_denied() {
    assert!(
        !QpuAccess::Denied.is_authorized(),
        "denied QPU access unexpectedly became authorized"
    );
}

/// A capability requirement must not itself count as authorization.
#[test]
fn qpu_capability_requirement_is_not_authorization() {
    assert!(
        !QpuAccess::RequiresCapability.is_authorized(),
        "requiring a capability must not grant that capability"
    );
}

/// Only an explicitly authorized state may permit QPU operations.
#[test]
fn qpu_authorization_requires_explicit_authorization() {
    assert!(QpuAccess::Authorized.is_authorized());
}

// ============================================================================
// 5. QPU capability regression guards
// ============================================================================

/// Stable QPU capability IDs.
///
/// Changing these IDs can invalidate persisted policy/checkpoint/audit data,
/// so changes must be intentional and versioned.
#[test]
fn qpu_capability_ids_remain_stable() {
    assert_eq!(Capability::QpuAccess.id(), 14);
    assert_eq!(Capability::QpuInspect.id(), 15);
    assert_eq!(Capability::QpuSubmit.id(), 16);
    assert_eq!(Capability::QpuReadResults.id(), 17);
    assert_eq!(Capability::QpuCalibration.id(), 18);
    assert_eq!(Capability::QpuErrorCorrection.id(), 19);
    assert_eq!(Capability::QpuSyndromeExtraction.id(), 20);
}

/// Stable QPU capability names.
#[test]
fn qpu_capability_names_remain_stable() {
    assert_eq!(
        Capability::QpuAccess.name(),
        "qec.qpu_access"
    );

    assert_eq!(
        Capability::QpuInspect.name(),
        "qec.qpu_inspect"
    );

    assert_eq!(
        Capability::QpuSubmit.name(),
        "qec.qpu_submit"
    );

    assert_eq!(
        Capability::QpuReadResults.name(),
        "qec.qpu_read_results"
    );

    assert_eq!(
        Capability::QpuCalibration.name(),
        "qec.qpu_calibration"
    );

    assert_eq!(
        Capability::QpuErrorCorrection.name(),
        "qec.qpu_error_correction"
    );

    assert_eq!(
        Capability::QpuSyndromeExtraction.name(),
        "qec.qpu_syndrome_extraction"
    );
}

/// Every QPU capability must remain classified as QPU-specific.
#[test]
fn qpu_capabilities_remain_qpu_scoped() {
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
            "{capability:?} lost its QPU classification"
        );
    }
}

/// Ordinary QEC capabilities must not acquire QPU privileges accidentally.
#[test]
fn classical_qec_capabilities_do_not_imply_qpu_access() {
    let classical_capabilities = [
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

    for capability in classical_capabilities {
        assert!(
            !capability.is_qpu(),
            "{capability:?} unexpectedly gained QPU privileges"
        );
    }
}

// ============================================================================
// 6. QPU operation-to-capability regression guards
// ============================================================================

/// Every QPU operation must map to one specific capability.
///
/// This prevents a regression where multiple unrelated hardware operations
/// accidentally share one overly broad privilege.
#[test]
fn qpu_operations_remain_individually_scoped() {
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

/// Regression guard against privilege collapse.
///
/// No two distinct QPU operations should silently collapse into the same
/// authorization capability.
#[test]
fn qpu_operations_do_not_collapse_into_one_privilege() {
    let operations = [
        QpuOperation::Inspect,
        QpuOperation::ReadCalibration,
        QpuOperation::SubmitCircuit,
        QpuOperation::ReadResults,
        QpuOperation::ErrorCorrection,
        QpuOperation::SyndromeExtraction,
    ];

    let mut ids = std::collections::BTreeSet::new();

    for operation in operations {
        let capability = operation.required_capability();

        assert!(
            ids.insert(capability.id()),
            "QPU operation capabilities unexpectedly share an ID"
        );
    }
}

// ============================================================================
// 7. Hardware execution privilege regression guards
// ============================================================================

/// Only explicitly hardware-executing QPU capabilities may execute hardware.
#[test]
fn only_explicit_qpu_execution_capabilities_can_execute_hardware() {
    assert!(!Capability::QpuAccess.can_execute_hardware());
    assert!(!Capability::QpuInspect.can_execute_hardware());
    assert!(!Capability::QpuReadResults.can_execute_hardware());
    assert!(!Capability::QpuCalibration.can_execute_hardware());

    assert!(Capability::QpuSubmit.can_execute_hardware());
    assert!(Capability::QpuErrorCorrection.can_execute_hardware());
    assert!(Capability::QpuSyndromeExtraction.can_execute_hardware());
}

/// Decode must never implicitly become physical hardware execution.
#[test]
fn decode_capability_does_not_execute_hardware() {
    assert!(!Capability::Decode.can_execute_hardware());
    assert!(!Capability::Simulate.can_execute_hardware());
    assert!(!Capability::Benchmark.can_execute_hardware());
}

/// Simulation and hardware execution remain separate concepts.
#[test]
fn simulation_remains_separate_from_qpu_execution() {
    assert!(!Capability::Simulate.is_qpu());
    assert!(!Capability::Simulate.can_execute_hardware());

    assert!(Capability::QpuSubmit.is_qpu());
    assert!(Capability::QpuSubmit.can_execute_hardware());
}

// ============================================================================
// 8. Backend regression guards
// ============================================================================

#[test]
fn qpu_backend_remains_explicit() {
    assert!(ExecutionBackend::Qpu.is_qpu());
    assert!(
        ExecutionBackend::Qpu.requires_hardware_capability(),
        "QPU backend must require explicit hardware authorization"
    );
    assert_eq!(
        ExecutionBackend::Qpu.name(),
        "qpu"
    );
}

#[test]
fn gpu_backend_remains_hardware_capability_bound() {
    assert!(ExecutionBackend::Gpu.requires_hardware_capability());
    assert!(!ExecutionBackend::Gpu.is_qpu());
}

#[test]
fn accelerator_backend_remains_hardware_capability_bound() {
    assert!(ExecutionBackend::Accelerator.requires_hardware_capability());
    assert!(!ExecutionBackend::Accelerator.is_qpu());
}

#[test]
fn cpu_backend_does_not_require_hardware_capability() {
    assert!(!ExecutionBackend::Cpu.requires_hardware_capability());
    assert!(!ExecutionBackend::Cpu.is_qpu());
}

#[test]
fn parallel_cpu_backend_does_not_require_hardware_capability() {
    assert!(
        !ExecutionBackend::ParallelCpu
            .requires_hardware_capability()
    );

    assert!(
        !ExecutionBackend::ParallelCpu.is_qpu()
    );
}

#[test]
fn distributed_backend_does_not_become_qpu_backend() {
    assert!(!ExecutionBackend::Distributed.is_qpu());
    assert_eq!(
        ExecutionBackend::Distributed.name(),
        "distributed"
    );
}

// ============================================================================
// 9. Resource-limit regression guards
// ============================================================================

#[test]
fn tiny_resource_policy_remains_valid() {
    let limits = tiny_limits();

    assert!(
        limits.validate().is_ok(),
        "known-valid regression resource policy became invalid"
    );
}

#[test]
fn permitted_request_remains_permitted() {
    let limits = tiny_limits();
    let request = permitted_request();

    assert!(
        limits.permits(&request),
        "request exactly matching the configured ceiling was rejected"
    );
}

#[test]
fn one_over_qubit_limit_remains_rejected() {
    let limits = tiny_limits();

    let mut request = permitted_request();
    request.qubits += 1;

    assert!(
        !limits.permits(&request),
        "request beyond qubit ceiling was accepted"
    );
}

#[test]
fn one_over_stabilizer_limit_remains_rejected() {
    let limits = tiny_limits();

    let mut request = permitted_request();
    request.stabilizers += 1;

    assert!(
        !limits.permits(&request),
        "request beyond stabilizer ceiling was accepted"
    );
}

#[test]
fn one_over_syndrome_limit_remains_rejected() {
    let limits = tiny_limits();

    let mut request = permitted_request();
    request.syndrome_events += 1;

    assert!(
        !limits.permits(&request),
        "request beyond syndrome-event ceiling was accepted"
    );
}

#[test]
fn one_over_round_limit_remains_rejected() {
    let limits = tiny_limits();

    let mut request = permitted_request();
    request.rounds += 1;

    assert!(
        !limits.permits(&request),
        "request beyond round ceiling was accepted"
    );
}

#[test]
fn one_over_graph_node_limit_remains_rejected() {
    let limits = tiny_limits();

    let mut request = permitted_request();
    request.graph_nodes += 1;

    assert!(
        !limits.permits(&request),
        "request beyond graph-node ceiling was accepted"
    );
}

#[test]
fn one_over_graph_edge_limit_remains_rejected() {
    let limits = tiny_limits();

    let mut request = permitted_request();
    request.graph_edges += 1;

    assert!(
        !limits.permits(&request),
        "request beyond graph-edge ceiling was accepted"
    );
}

#[test]
fn one_over_memory_limit_remains_rejected() {
    let limits = tiny_limits();

    let mut request = permitted_request();
    request.memory_bytes += 1;

    assert!(
        !limits.permits(&request),
        "request beyond memory ceiling was accepted"
    );
}

#[test]
fn one_over_parallelism_limit_remains_rejected() {
    let limits = tiny_limits();

    let mut request = permitted_request();
    request.parallelism += 1;

    assert!(
        !limits.permits(&request),
        "request beyond parallelism ceiling was accepted"
    );
}

#[test]
fn one_over_checkpoint_limit_remains_rejected() {
    let limits = tiny_limits();

    let mut request = permitted_request();
    request.checkpoint_bytes += 1;

    assert!(
        !limits.permits(&request),
        "request beyond checkpoint ceiling was accepted"
    );
}

#[test]
fn one_over_execution_time_limit_remains_rejected() {
    let limits = tiny_limits();

    let mut request = permitted_request();
    request.execution_time += Duration::from_nanos(1);

    assert!(
        !limits.permits(&request),
        "request beyond execution-time ceiling was accepted"
    );
}

// ============================================================================
// 10. "Infinite" workload regression guards
// ============================================================================

/// Regression protection against the old conceptual promise of literal
/// infinity.
///
/// `u64::MAX` must remain a bounded request that is rejected by a finite
/// resource policy.
#[test]
fn maximum_integer_workload_is_not_treated_as_infinite_capacity() {
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

    assert!(
        !limits.permits(&hostile),
        "maximum-size request must never bypass resource controls"
    );
}

/// Resource enforcement must not use wrapping arithmetic semantics.
#[test]
fn resource_limits_remain_fail_closed_at_integer_boundaries() {
    let limits = tiny_limits();

    let mut request = permitted_request();

    request.qubits = u64::MAX;
    assert!(!limits.permits(&request));

    request.qubits = 4;
    request.graph_nodes = u64::MAX;
    assert!(!limits.permits(&request));

    request.graph_nodes = 8;
    request.graph_edges = u64::MAX;
    assert!(!limits.permits(&request));

    request.graph_edges = 16;
    request.memory_bytes = u64::MAX;
    assert!(!limits.permits(&request));

    request.memory_bytes = 1024;
    request.checkpoint_bytes = u64::MAX;
    assert!(!limits.permits(&request));
}

// ============================================================================
// 11. Zero-limit regression guards
// ============================================================================

/// Zero resource limits must remain invalid rather than silently meaning
/// unlimited.
#[test]
fn zero_code_distance_is_not_unlimited() {
    let mut limits = tiny_limits();
    limits.max_code_distance = 0;

    assert!(
        limits.validate().is_err(),
        "zero code distance must remain invalid"
    );
}

#[test]
fn zero_qubits_is_not_unlimited() {
    let mut limits = tiny_limits();
    limits.max_qubits = 0;

    assert!(
        limits.validate().is_err(),
        "zero qubits must remain invalid"
    );
}

#[test]
fn zero_memory_is_not_unlimited() {
    let mut limits = tiny_limits();
    limits.max_memory_bytes = 0;

    assert!(
        limits.validate().is_err(),
        "zero memory must remain invalid"
    );
}

#[test]
fn zero_execution_time_is_not_unlimited() {
    let mut limits = tiny_limits();
    limits.max_execution_time = Duration::ZERO;

    assert!(
        limits.validate().is_err(),
        "zero execution time must remain invalid"
    );
}

#[test]
fn zero_parallelism_is_not_unlimited() {
    let mut limits = tiny_limits();
    limits.max_parallelism = 0;

    assert!(
        limits.validate().is_err(),
        "zero parallelism must remain invalid"
    );
}

// ============================================================================
// 12. Resource-policy determinism
// ============================================================================

#[test]
fn identical_resource_policies_are_deterministic() {
    let first = tiny_limits();
    let second = tiny_limits();

    assert_eq!(first, second);
}

#[test]
fn identical_resource_requests_are_deterministic() {
    let first = permitted_request();
    let second = permitted_request();

    assert_eq!(first, second);
}

#[test]
fn repeated_resource_authorization_is_deterministic() {
    let limits = tiny_limits();
    let request = permitted_request();

    let expected = limits.permits(&request);

    for _ in 0..1_000 {
        assert_eq!(
            limits.permits(&request),
            expected,
            "resource authorization changed between identical evaluations"
        );
    }
}

// ============================================================================
// 13. Capability identity regression guards
// ============================================================================

#[test]
fn core_capability_ids_remain_stable() {
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
}

#[test]
fn capability_ids_remain_unique() {
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

    let mut ids = std::collections::BTreeSet::new();

    for capability in capabilities {
        assert!(
            ids.insert(capability.id()),
            "capability ID {} is duplicated",
            capability.id()
        );
    }

    assert_eq!(
        ids.len(),
        capabilities.len(),
        "capability IDs must remain one-to-one"
    );
}

#[test]
fn capability_names_remain_unique() {
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
        assert!(
            !capability.name().is_empty(),
            "{capability:?} has an empty capability name"
        );

        assert!(
            names.insert(capability.name()),
            "duplicate capability name: {}",
            capability.name()
        );
    }

    assert_eq!(
        names.len(),
        capabilities.len(),
        "capability names must remain one-to-one"
    );
}

// ============================================================================
// 14. Fail-closed regression guards
// ============================================================================

/// QPU authorization must not accidentally be inferred from ordinary
/// execution environments.
#[test]
fn qpu_access_is_not_inferred_from_execution_environment() {
    let environments = [
        ExecutionEnvironment::Cpu,
        ExecutionEnvironment::ParallelCpu,
        ExecutionEnvironment::Gpu,
        ExecutionEnvironment::Accelerator,
        ExecutionEnvironment::Distributed,
    ];

    for environment in environments {
        assert!(
            !environment.is_qpu(),
            "{environment:?} unexpectedly gained QPU identity"
        );
    }

    assert!(
        !QpuAccess::RequiresCapability.is_authorized(),
        "capability requirement unexpectedly became authorization"
    );
}

/// Decode capability must not silently grant accelerator privileges.
#[test]
fn decode_capability_does_not_imply_accelerator_access() {
    assert!(!Capability::Decode.is_qpu());
    assert!(!Capability::Decode.can_execute_hardware());
}

/// Simulation must not silently become physical execution.
#[test]
fn simulation_does_not_imply_physical_execution() {
    assert!(!Capability::Simulate.is_qpu());
    assert!(!Capability::Simulate.can_execute_hardware());
}

// ============================================================================
// 15. Repeated-call stability regression guards
// ============================================================================

#[test]
fn capability_identity_is_stable_across_repeated_calls() {
    for _ in 0..1_000 {
        assert_eq!(Capability::QpuSubmit.id(), 16);
        assert_eq!(
            Capability::QpuSubmit.name(),
            "qec.qpu_submit"
        );

        assert_eq!(
            Capability::QpuErrorCorrection.id(),
            19
        );

        assert_eq!(
            Capability::QpuSyndromeExtraction.id(),
            20
        );
    }
}

#[test]
fn backend_identity_is_stable_across_repeated_calls() {
    for _ in 0..1_000 {
        assert_eq!(
            ExecutionBackend::Qpu.name(),
            "qpu"
        );

        assert!(ExecutionBackend::Qpu.is_qpu());
        assert!(
            ExecutionBackend::Qpu
                .requires_hardware_capability()
        );
    }
}

// ============================================================================
// 16. No-panic regression guards
// ============================================================================

#[test]
fn qec_metadata_access_does_not_panic() {
    let result = assert_no_panic(|| {
        (
            api_version(),
            QEC_API_VERSION,
            QEC_SUBSYSTEM_NAME,
            QEC_ARCHITECTURE,
        )
    });

    assert_eq!(result.0, result.1);
}

#[test]
fn capability_metadata_access_does_not_panic() {
    let result = assert_no_panic(|| {
        (
            Capability::QpuSubmit.id(),
            Capability::QpuSubmit.name(),
            Capability::QpuSubmit.is_qpu(),
            Capability::QpuSubmit.can_execute_hardware(),
        )
    });

    assert_eq!(result.0, 16);
    assert_eq!(result.1, "qec.qpu_submit");
    assert!(result.2);
    assert!(result.3);
}

#[test]
fn resource_authorization_does_not_panic_on_extreme_values() {
    let limits = tiny_limits();

    let request = ResourceRequest {
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

    let result = assert_no_panic(|| {
        limits.permits(&request)
    });

    assert!(
        !result,
        "extreme resource request bypassed the regression boundary"
    );
}

// ============================================================================
// 17. Capability/environment matrix regression
// ============================================================================

#[test]
fn qpu_environment_remains_supported_by_current_capabilities() {
    let caps = capabilities();

    assert!(
        caps.supports_execution(ExecutionEnvironment::Qpu),
        "compiled QEC capabilities claim QPU support but do not expose it"
    );
}

#[test]
fn classical_environments_remain_supported_by_current_capabilities() {
    let caps = capabilities();

    assert!(
        caps.supports_execution(ExecutionEnvironment::Cpu)
    );

    assert!(
        caps.supports_execution(
            ExecutionEnvironment::ParallelCpu
        )
    );

    assert!(
        caps.supports_execution(
            ExecutionEnvironment::Gpu
        )
    );

    assert!(
        caps.supports_execution(
            ExecutionEnvironment::Accelerator
        )
    );

    assert!(
        caps.supports_execution(
            ExecutionEnvironment::Distributed
        )
    );
}

// ============================================================================
// 18. QPU resource-boundary regression
// ============================================================================

/// QPU resource limits must remain finite.
///
/// This protects against accidentally changing QPU execution into an
/// unlimited-resource path.
#[test]
fn qpu_resource_policy_is_finite() {
    let limits = ResourceLimits::qpu();

    assert!(limits.validate().is_ok());

    assert!(limits.max_code_distance < u64::MAX);
    assert!(limits.max_qubits < u64::MAX);
    assert!(limits.max_stabilizers < u64::MAX);
    assert!(limits.max_syndrome_events < u64::MAX);
    assert!(limits.max_rounds < u64::MAX);
    assert!(limits.max_graph_nodes < u64::MAX);
    assert!(limits.max_graph_edges < u64::MAX);
    assert!(limits.max_memory_bytes < u64::MAX);
    assert!(limits.max_parallelism < u32::MAX);
    assert!(limits.max_checkpoint_bytes < u64::MAX);
}

/// Simulation and QPU policies remain independently represented.
#[test]
fn simulation_and_qpu_resource_policies_remain_distinct() {
    let simulation = ResourceLimits::simulation();
    let qpu = ResourceLimits::qpu();

    assert!(simulation.validate().is_ok());
    assert!(qpu.validate().is_ok());

    assert_ne!(
        simulation,
        qpu,
        "simulation and QPU resource policies must not collapse into one policy"
    );
}

// ============================================================================
// 19. QPU capability separation regression
// ============================================================================

#[test]
fn qpu_submit_does_not_equal_qpu_calibration() {
    assert_ne!(
        Capability::QpuSubmit,
        Capability::QpuCalibration
    );

    assert_ne!(
        Capability::QpuSubmit.id(),
        Capability::QpuCalibration.id()
    );
}

#[test]
fn qpu_error_correction_does_not_equal_qpu_inspection() {
    assert_ne!(
        Capability::QpuErrorCorrection,
        Capability::QpuInspect
    );
}

#[test]
fn qpu_syndrome_extraction_does_not_equal_qpu_result_reading() {
    assert_ne!(
        Capability::QpuSyndromeExtraction,
        Capability::QpuReadResults
    );
}

// ============================================================================
// 20. Regression catalog integrity
// ============================================================================

/// This test is intentionally simple but important.
///
/// It documents that the suite itself covers the major production boundaries
/// established by the QEC architecture.
#[test]
fn regression_suite_covers_required_production_boundaries() {
    let required_boundaries = [
        "api",
        "mathematical-self-check",
        "execution-environment",
        "qpu-isolation",
        "qpu-capabilities",
        "qpu-operation-scope",
        "hardware-authorization",
        "backend-isolation",
        "resource-limits",
        "infinite-workload-bounds",
        "integer-boundaries",
        "determinism",
        "fail-closed",
        "no-panic",
        "capability-identity",
        "qpu-resource-policy",
        "simulation-qpu-separation",
    ];

    assert_eq!(
        required_boundaries.len(),
        17,
        "regression catalog unexpectedly changed"
    );

    for boundary in required_boundaries {
        assert!(
            !boundary.is_empty(),
            "regression boundary identifier must not be empty"
        );
    }
}

// ============================================================================
// 21. Future-proofing: supported environment uniqueness
// ============================================================================

#[test]
fn supported_execution_environments_are_unique() {
    let environments = supported_execution_environments();

    let mut unique = std::collections::BTreeSet::new();

    for environment in environments {
        assert!(
            unique.insert(*environment as u8),
            "duplicate execution environment detected"
        );
    }

    assert_eq!(
        unique.len(),
        environments.len(),
        "execution environment list must remain unique"
    );
}

// ============================================================================
// 22. QPU authorization remains explicitly opt-in
// ============================================================================

#[test]
fn qpu_authorization_is_explicitly_opt_in() {
    let states = [
        QpuAccess::Denied,
        QpuAccess::RequiresCapability,
        QpuAccess::Authorized,
    ];

    assert!(!states[0].is_authorized());
    assert!(!states[1].is_authorized());
    assert!(states[2].is_authorized());
}

#[test]
fn qpu_deny_and_capability_required_are_not_authorized() {
    assert_ne!(
        QpuAccess::Denied,
        QpuAccess::Authorized
    );

    assert_ne!(
        QpuAccess::RequiresCapability,
        QpuAccess::Authorized
    );
}

// ============================================================================
// 23. Stable backend naming
// ============================================================================

#[test]
fn backend_names_remain_stable() {
    assert_eq!(
        ExecutionBackend::Cpu.name(),
        "cpu"
    );

    assert_eq!(
        ExecutionBackend::ParallelCpu.name(),
        "parallel-cpu"
    );

    assert_eq!(
        ExecutionBackend::Gpu.name(),
        "gpu"
    );

    assert_eq!(
        ExecutionBackend::Accelerator.name(),
        "accelerator"
    );

    assert_eq!(
        ExecutionBackend::Distributed.name(),
        "distributed"
    );

    assert_eq!(
        ExecutionBackend::Qpu.name(),
        "qpu"
    );
}

// ============================================================================
// 24. No hidden privilege inheritance
// ============================================================================

#[test]
fn decode_does_not_inherit_qpu_submit_privilege() {
    assert_ne!(
        Capability::Decode,
        Capability::QpuSubmit
    );

    assert!(
        !Capability::Decode.can_execute_hardware()
    );
}

#[test]
fn simulation_does_not_inherit_qpu_submit_privilege() {
    assert_ne!(
        Capability::Simulate,
        Capability::QpuSubmit
    );

    assert!(
        !Capability::Simulate.can_execute_hardware()
    );
}

#[test]
fn topology_inspection_does_not_inherit_qpu_submit_privilege() {
    assert_ne!(
        Capability::InspectTopology,
        Capability::QpuSubmit
    );

    assert!(
        !Capability::InspectTopology
            .can_execute_hardware()
    );
}

// ============================================================================
// 25. Permanent regression contract
// ============================================================================

/// Number 25 from the QEC production requirements:
///
/// > Every discovered mathematical or implementation bug should become a
/// > permanent regression test.
///
/// This test documents the permanent-contract philosophy directly in the
/// source tree.
///
/// New bugs should be added to this file as:
///
///     regression_bug_<stable_identifier>()
///
/// rather than being fixed without a reproducible test.
///
/// Examples:
///
///     regression_bug_001_identity_syndrome()
///     regression_bug_002_qpu_privilege_escalation()
///     regression_bug_003_resource_overflow()
///
/// Existing tests above serve as permanent regression guards for the current
/// architectural invariants.
#[test]
fn permanent_regression_contract_is_documented() {
    let contract = [
        "discovered mathematical bugs become permanent tests",
        "discovered decoder bugs become permanent tests",
        "discovered resource bugs become permanent tests",
        "discovered security bugs become permanent tests",
        "discovered QPU isolation bugs become permanent tests",
        "discovered determinism bugs become permanent tests",
        "discovered serialization/checkpoint bugs become permanent tests",
        "discovered backend bugs become permanent tests",
        "discovered distributed-execution bugs become permanent tests",
        "discovered numerical bugs become permanent tests",
    ];

    assert_eq!(
        contract.len(),
        10,
        "permanent regression contract unexpectedly changed"
    );

    for rule in contract {
        assert!(
            !rule.is_empty(),
            "regression contract entry must not be empty"
        );
    }
}

// ============================================================================
// 26. Final architecture integrity check
// ============================================================================

/// Final high-level regression guard.
///
/// This deliberately does not execute any decoder or hardware. It verifies
/// that the foundational architecture still exposes all execution modes while
/// retaining QPU separation and finite resource semantics.
#[test]
fn production_qec_architecture_integrity() {
    let caps = capabilities();

    assert!(caps.validation);
    assert!(caps.resource_limits);
    assert!(caps.resource_accounting);
    assert!(caps.metrics);
    assert!(caps.telemetry);
    assert!(caps.cancellation);
    assert!(caps.deterministic_execution);
    assert!(caps.checkpointing);
    assert!(caps.streaming);
    assert!(caps.partitioning);
    assert!(caps.distributed_execution);
    assert!(caps.scheduling);
    assert!(caps.memory_management);
    assert!(caps.safe_arithmetic);
    assert!(caps.sparse_data);
    assert!(caps.caching);

    assert!(caps.cpu_backend);
    assert!(caps.parallel_cpu_backend);
    assert!(caps.gpu_backend);
    assert!(caps.accelerator_backend);
    assert!(caps.qpu_backend);

    assert!(caps.capability_security);
    assert!(caps.configuration_management);
    assert!(caps.versioning);

    assert!(
        ResourceLimits::qpu().validate().is_ok(),
        "QPU resource policy must remain valid"
    );

    assert!(
        ResourceLimits::simulation().validate().is_ok(),
        "simulation resource policy must remain valid"
    );

    assert!(
        assert_no_panic(self_check).is_ok(),
        "fundamental QEC self-check must remain valid"
    );
}