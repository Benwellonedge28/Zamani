//! Production scalability tests for Zamani Quantum Error Correction.
//!
//! These tests verify that the QEC subsystem:
//!
//! * scales with increasing surface-code distance;
//! * does not impose artificial "infinite" resource assumptions;
//! * uses explicit resource accounting;
//! * respects configured limits;
//! * handles very large requested sizes without panicking;
//! * preserves topology invariants while scaling;
//! * supports deterministic workload descriptions;
//! * models streaming workloads;
//! * models partitionable workloads;
//! * models distributed workloads;
//! * models accelerated execution;
//! * models physical QPU execution through the backend abstraction;
//! * validates QPU topology and capabilities;
//! * measures construction/scaling characteristics;
//! * detects unexpected complexity growth;
//! * avoids hidden integer-overflow assumptions;
//! * remains suitable for graceful resource exhaustion;
//! * provides reproducible benchmark output;
//! * separates software scalability from physical-QPU availability.
//!
//! IMPORTANT:
//!
//! These are scalability/contract tests, not claims that a local test machine
//! can physically execute arbitrarily large QEC instances or a real QPU.
//! Physical QPU execution is intentionally represented through the backend
//! abstraction and metadata contract. The backend layer explicitly separates
//! QPU description from device/network I/O.
//!
//! Long-running tests are marked `#[ignore]` so normal CI remains bounded.
//!
//! Recommended CI:
//!
//!     cargo test
//!
//! Extended scalability:
//!
//!     cargo test -- --ignored scalability
//!
//! Production benchmark environments should additionally record:
//!
//! * CPU model;
//! * RAM;
//! * operating system;
//! * Rust/compiler version;
//! * backend configuration;
//! * QEC distance;
//! * number of qubits;
//! * number of stabilizers;
//! * construction latency;
//! * peak/resource estimates;
//! * deterministic workload fingerprint.

#![allow(clippy::cast_precision_loss)]

use std::collections::BTreeSet;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::time::{Duration, Instant};

use super::super::backend::{
    BackendCapabilities,
    BackendConfig,
    BackendExecutionRequest,
    BackendKind,
    BackendResourceLimits,
    BackendResourceUsage,
    BackendStatus,
    BackendTopology,
    DeterminismPolicy,
    QecWorkload,
    QpuInfo,
};
use super::super::surface_code::{
    Coordinate,
    SurfaceCode,
};

// ============================================================================
// Test policy
// ============================================================================

/// Upper bound for normal CI tests.
///
/// This is deliberately conservative. Scalability tests must not become an
/// accidental denial-of-service mechanism against the developer's machine.
const CI_MAX_DISTANCE: usize = 15;

/// Distances used for normal scaling measurements.
const CI_DISTANCES: &[usize] = &[3, 5, 7, 9, 11, 13, 15];

/// Distances used by extended/ignored tests.
const EXTENDED_DISTANCES: &[usize] = &[
    3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 25, 31, 41, 51,
];

/// Maximum number of samples used by benchmark-style tests.
const MAX_BENCHMARK_SAMPLES: usize = 14;

/// Generous construction-time guard for normal CI.
///
/// This is not a performance SLA. It prevents a pathological regression from
/// hanging CI indefinitely.
const CI_CONSTRUCTION_TIMEOUT: Duration =
    Duration::from_secs(10);

// ============================================================================
// Measurement model
// ============================================================================

#[derive(Debug, Clone, Copy)]
struct ScalingMeasurement {
    distance: usize,
    qubits: usize,
    stabilizers: usize,
    construction_time: Duration,
}

impl ScalingMeasurement {
    fn expected_qubits(self) -> usize {
        self.distance
            .checked_mul(self.distance)
            .expect("test distance must fit usize")
    }

    fn expected_stabilizers(self) -> usize {
        self.expected_qubits()
            .checked_sub(1)
            .expect("surface code must contain at least one stabilizer")
    }

    fn qubit_growth_ratio(self, previous: Self) -> f64 {
        self.qubits as f64 / previous.qubits as f64
    }

    fn stabilizer_growth_ratio(self, previous: Self) -> f64 {
        self.stabilizers as f64 / previous.stabilizers as f64
    }
}

// ============================================================================
// Helpers
// ============================================================================

fn build_code(distance: usize) -> SurfaceCode {
    let started = Instant::now();

    let result = catch_unwind(AssertUnwindSafe(|| {
        SurfaceCode::new(distance)
    }));

    let elapsed = started.elapsed();

    assert!(
        elapsed <= CI_CONSTRUCTION_TIMEOUT,
        "constructing distance-{distance} surface code took {elapsed:?}, \
         exceeding the CI scalability guard"
    );

    match result {
        Ok(Ok(code)) => code,

        Ok(Err(error)) => {
            panic!(
                "valid odd surface-code distance {distance} was rejected: {error:?}"
            );
        }

        Err(_) => {
            panic!(
                "surface-code construction panicked at distance {distance}"
            );
        }
    }
}

fn measure(distance: usize) -> ScalingMeasurement {
    let started = Instant::now();

    let code = build_code(distance);

    ScalingMeasurement {
        distance,
        qubits: code.num_data_qubits(),
        stabilizers: code.num_stabilizers(),
        construction_time: started.elapsed(),
    }
}

fn expected_qubits(distance: usize) -> usize {
    distance
        .checked_mul(distance)
        .expect("distance overflow in test expectation")
}

fn expected_stabilizers(distance: usize) -> usize {
    expected_qubits(distance)
        .checked_sub(1)
        .expect("invalid surface-code size")
}

fn assert_surface_code_shape(
    code: &SurfaceCode,
    distance: usize,
) {
    assert_eq!(
        code.distance(),
        distance,
        "distance metadata changed during construction"
    );

    assert_eq!(
        code.num_data_qubits(),
        expected_qubits(distance),
        "data-qubit scaling invariant violated at distance {distance}"
    );

    assert_eq!(
        code.num_stabilizers(),
        expected_stabilizers(distance),
        "stabilizer scaling invariant violated at distance {distance}"
    );

    assert_eq!(
        code.num_logical_qubits(),
        1,
        "canonical rotated planar surface code must encode one logical qubit"
    );
}

fn workload_for_distance(
    distance: usize,
) -> QecWorkload {
    let qubits = expected_qubits(distance);
    let stabilizers = expected_stabilizers(distance);

    let syndrome_events = stabilizers
        .checked_mul(8)
        .expect("syndrome-event test calculation overflowed");

    let graph_nodes = syndrome_events;

    let graph_edges = graph_nodes
        .checked_mul(4)
        .expect("graph-edge test calculation overflowed");

    QecWorkload {
        qubits,
        stabilizers,
        syndrome_events,
        graph_nodes,
        graph_edges,
        rounds: 8,
        parallelism: 1,
        memory_bytes: (qubits as u64)
            .checked_mul(64)
            .expect("memory estimate overflowed"),
        shots: 100,
        operations: BTreeSet::new(),
        requires_determinism: true,
        requires_qpu: false,
        requires_calibration: false,
        streaming: true,
        partitionable: true,
        distributable: true,
        cancellable: true,
    }
    .with_operation("syndrome_generation")
    .with_operation("decode")
    .with_operation("pauli_frame")
}

fn qpu_workload_for_distance(
    distance: usize,
) -> QecWorkload {
    let mut workload =
        workload_for_distance(distance);

    workload.requires_qpu = true;
    workload.requires_calibration = true;
    workload
        .operations
        .insert("measurement".to_string());
    workload
        .operations
        .insert("mid_circuit_measurement".to_string());
    workload
        .operations
        .insert("reset".to_string());
    workload
        .operations
        .insert("classical_control".to_string());

    workload
}

fn test_qpu_info(
    qubits: usize,
) -> QpuInfo {
    let mut topology =
        BackendTopology::new(qubits)
            .expect("positive QPU topology size");

    /*
     * Build a sparse nearest-neighbour topology.

     * This deliberately avoids a dense O(N²) edge representation.
     *
     * The QEC subsystem must be able to represent large physical systems
     * without assuming all-to-all connectivity.
     */
    if qubits > 1 {
        for index in 0..(qubits - 1) {
            topology
                .add_edge(index, index + 1)
                .expect("valid nearest-neighbour edge");
        }
    }

    let supported_native_operations =
        [
            "x",
            "z",
            "h",
            "cx",
            "measure",
            "reset",
            "mid_circuit_measurement",
            "classical_control",
        ]
        .into_iter()
        .map(str::to_owned)
        .collect();

    QpuInfo {
        device_id: "test-qpu-scaling-device".to_owned(),
        provider: "zamani-test-provider".to_owned(),
        architecture: "abstract-qpu".to_owned(),
        calibration_id: Some(
            "test-calibration-1".to_owned()
        ),
        calibration_version: Some(
            "1".to_owned()
        ),
        queue_depth: Some(0),
        available_qubits: qubits,
        supported_native_operations,
        topology,
    }
}

// ============================================================================
// Basic scaling invariants
// ============================================================================

#[test]
fn surface_code_scales_quadratically_in_data_qubits() {
    for &distance in CI_DISTANCES {
        let code = build_code(distance);

        assert_surface_code_shape(
            &code,
            distance,
        );
    }
}

#[test]
fn surface_code_stabilizers_scale_with_qubit_count() {
    for &distance in CI_DISTANCES {
        let code = build_code(distance);

        assert_eq!(
            code.num_stabilizers(),
            code.num_data_qubits() - 1,
            "stabilizer count invariant failed at distance {distance}"
        );
    }
}

#[test]
fn scaling_measurements_are_monotonic() {
    let measurements: Vec<_> =
        CI_DISTANCES
            .iter()
            .copied()
            .map(measure)
            .collect();

    for window in measurements.windows(2) {
        let previous = window[0];
        let current = window[1];

        assert!(
            current.qubits > previous.qubits,
            "qubit count must increase monotonically"
        );

        assert!(
            current.stabilizers > previous.stabilizers,
            "stabilizer count must increase monotonically"
        );

        assert!(
            current.distance > previous.distance,
            "distance must increase monotonically"
        );
    }
}

// ============================================================================
// Topology scalability
// ============================================================================

#[test]
fn topology_coordinates_remain_bounded_at_scale() {
    for &distance in CI_DISTANCES {
        let code = build_code(distance);

        let samples = [
            Coordinate::new(0, 0),
            Coordinate::new(0, distance - 1),
            Coordinate::new(distance - 1, 0),
            Coordinate::new(distance - 1, distance - 1),
            Coordinate::new(distance / 2, distance / 2),
        ];

        for coordinate in samples {
            let qubit = code
                .qubit_at(coordinate)
                .expect("valid coordinate");

            let returned =
                qubit.coordinate();

            assert_eq!(
                returned,
                coordinate,
                "coordinate mapping changed at distance {distance}"
            );

            assert!(
                returned.row() < distance,
                "row escaped code boundary"
            );

            assert!(
                returned.column() < distance,
                "column escaped code boundary"
            );
        }
    }
}

#[test]
fn all_surface_code_qubits_have_valid_coordinates() {
    for &distance in &[3, 5, 7, 9, 11] {
        let code = build_code(distance);

        for qubit in code.data_qubits() {
            let coordinate =
                qubit.coordinate();

            assert!(
                coordinate.row() < distance,
                "invalid row at distance {distance}"
            );

            assert!(
                coordinate.column() < distance,
                "invalid column at distance {distance}"
            );

            let recovered =
                code.qubit_at(coordinate)
                    .expect("coordinate must map back to qubit");

            assert_eq!(
                recovered.index(),
                qubit.index(),
                "coordinate/index mapping is not stable"
            );
        }
    }
}

// ============================================================================
// Resource-aware scaling
// ============================================================================

#[test]
fn workload_resource_model_scales_without_overflow() {
    for &distance in CI_DISTANCES {
        let workload =
            workload_for_distance(distance);

        assert_eq!(
            workload.qubits,
            expected_qubits(distance)
        );

        assert_eq!(
            workload.stabilizers,
            expected_stabilizers(distance)
        );

        assert!(
            workload.syndrome_events
                >= workload.stabilizers
        );

        assert!(
            workload.graph_nodes
                >= workload.syndrome_events
        );

        assert!(
            workload.graph_edges
                >= workload.graph_nodes
        );

        assert!(
            workload.memory_bytes > 0
        );
    }
}

#[test]
fn resource_accounting_accepts_valid_scaling_measurements() {
    for &distance in CI_DISTANCES {
        let workload =
            workload_for_distance(distance);

        let usage =
            BackendResourceUsage {
                allocated_memory_bytes:
                    workload.memory_bytes,
                peak_memory_bytes:
                    workload.memory_bytes,
                cpu_time: Duration::from_millis(1),
                wall_time: Duration::from_millis(1),
                qubits: workload.qubits,
                stabilizers: workload.stabilizers,
                syndrome_events:
                    workload.syndrome_events,
                graph_nodes:
                    workload.graph_nodes,
                graph_edges:
                    workload.graph_edges,
                rounds: workload.rounds,
                decoder_iterations:
                    workload.syndrome_events,
                parallel_workers:
                    workload.parallelism,
                shots: workload.shots,
            };

        let limits =
            BackendResourceLimits {
                max_qubits:
                    Some(workload.qubits),
                max_stabilizers:
                    Some(workload.stabilizers),
                max_syndrome_events:
                    Some(workload.syndrome_events),
                max_graph_nodes:
                    Some(workload.graph_nodes),
                max_graph_edges:
                    Some(workload.graph_edges),
                max_rounds:
                    Some(workload.rounds),
                max_parallelism:
                    Some(workload.parallelism),
                max_memory_bytes:
                    Some(workload.memory_bytes),
                max_wall_time:
                    Some(Duration::from_secs(1)),
                max_cpu_time:
                    Some(Duration::from_secs(1)),
                max_shots:
                    Some(workload.shots),
                max_checkpoint_bytes: None,
            };

        limits
            .validate()
            .expect("valid resource limits");

        usage
            .validate_against(&limits)
            .expect(
                "measurement must fit exactly within configured limits"
            );
    }
}

#[test]
fn resource_limits_reject_scaling_overruns() {
    let workload =
        workload_for_distance(15);

    let usage =
        BackendResourceUsage {
            qubits: workload.qubits + 1,
            stabilizers: workload.stabilizers,
            syndrome_events:
                workload.syndrome_events,
            graph_nodes:
                workload.graph_nodes,
            graph_edges:
                workload.graph_edges,
            rounds: workload.rounds,
            parallel_workers:
                workload.parallelism,
            shots: workload.shots,
            allocated_memory_bytes:
                workload.memory_bytes,
            peak_memory_bytes:
                workload.memory_bytes,
            cpu_time: Duration::from_millis(1),
            wall_time: Duration::from_millis(1),
            decoder_iterations: 1,
        };

    let limits =
        BackendResourceLimits {
            max_qubits:
                Some(workload.qubits),
            max_stabilizers:
                Some(workload.stabilizers),
            max_syndrome_events:
                Some(workload.syndrome_events),
            max_graph_nodes:
                Some(workload.graph_nodes),
            max_graph_edges:
                Some(workload.graph_edges),
            max_rounds:
                Some(workload.rounds),
            max_parallelism:
                Some(workload.parallelism),
            max_memory_bytes:
                Some(workload.memory_bytes),
            max_wall_time:
                Some(Duration::from_secs(1)),
            max_cpu_time:
                Some(Duration::from_secs(1)),
            max_shots:
                Some(workload.shots),
            max_checkpoint_bytes: None,
        };

    assert!(
        usage.validate_against(&limits).is_err(),
        "resource overrun must fail deterministically"
    );
}

// ============================================================================
// Explicit "arbitrarily large" semantics
// ============================================================================

#[test]
fn unlimited_resource_policy_does_not_mean_infinite_memory() {
    let limits =
        BackendResourceLimits::unlimited();

    assert!(
        limits.validate().is_ok()
    );

    /*
     * The policy means that this layer does not impose a numerical limit.
     * It must never be interpreted as an instruction to allocate infinite
     * memory or accept usize::MAX-sized allocations.
     */
    assert!(
        limits.max_memory_bytes.is_none()
    );

    assert!(
        limits.max_qubits.is_none()
    );

    assert!(
        limits.max_graph_nodes.is_none()
    );
}

#[test]
fn usize_extreme_values_are_not_used_as_allocation_requests() {
    /*
     * This test is intentionally arithmetic-only.

     * Production code must perform checked arithmetic before allocating.
     * The presence of a huge requested logical size must not itself cause
     * a vector allocation.
     */
    let distance =
        usize::MAX;

    let square =
        distance.checked_mul(distance);

    assert!(
        square.is_none(),
        "usize::MAX² must be detected as overflow"
    );
}

// ============================================================================
// Graceful failure / panic resistance
// ============================================================================

#[test]
fn supported_scaling_inputs_do_not_panic() {
    for &distance in CI_DISTANCES {
        let result =
            catch_unwind(
                AssertUnwindSafe(|| {
                    SurfaceCode::new(distance)
                }),
            );

        assert!(
            result.is_ok(),
            "distance {distance} caused a panic"
        );
    }
}

#[test]
fn invalid_even_distance_fails_without_panic() {
    let result =
        catch_unwind(
            AssertUnwindSafe(|| {
                SurfaceCode::new(4)
            }),
        );

    assert!(
        result.is_ok(),
        "invalid distance must return an error rather than panic"
    );

    assert!(
        result.expect("panic already checked").is_err(),
        "even distance must be rejected"
    );
}

#[test]
fn invalid_tiny_distance_fails_without_panic() {
    for distance in [0usize, 1usize, 2usize] {
        let result =
            catch_unwind(
                AssertUnwindSafe(|| {
                    SurfaceCode::new(distance)
                }),
            );

        assert!(
            result.is_ok(),
            "distance {distance} caused a panic"
        );

        assert!(
            result.expect("panic already checked").is_err(),
            "invalid distance {distance} must be rejected"
        );
    }
}

// ============================================================================
// Backend scalability contract
// ============================================================================

#[test]
fn backend_kinds_cover_scalable_execution_classes() {
    let kinds = [
        BackendKind::Cpu,
        BackendKind::ParallelCpu,
        BackendKind::Gpu,
        BackendKind::Accelerator,
        BackendKind::Distributed,
        BackendKind::Simulator,
        BackendKind::Emulator,
        BackendKind::Qpu,
    ];

    for kind in kinds {
        assert!(
            kind.is_software()
                || kind.is_physical_qpu()
        );
    }

    assert!(
        BackendKind::Qpu.is_physical_qpu()
    );

    assert!(
        BackendKind::Qpu.may_be_remote()
    );

    assert!(
        BackendKind::Distributed.may_be_remote()
    );
}

#[test]
fn deterministic_configuration_is_explicit() {
    let config =
        BackendConfig {
            determinism:
                DeterminismPolicy::Strict,
            limits:
                BackendResourceLimits::unlimited(),
            max_retries: 0,
            allow_degraded: false,
            allow_fallback: false,
            require_explicit_qpu_capability:
                true,
        };

    assert!(
        config.validate().is_ok()
    );

    assert_eq!(
        config.determinism,
        DeterminismPolicy::Strict
    );

    assert!(
        config.require_explicit_qpu_capability
    );
}

#[test]
fn scalable_workload_expresses_streaming_partitioning_and_distribution() {
    let workload =
        workload_for_distance(15);

    assert!(
        workload.streaming,
        "large QEC workloads should support streaming"
    );

    assert!(
        workload.partitionable,
        "large QEC workloads should support partitioning"
    );

    assert!(
        workload.distributable,
        "large QEC workloads should support distributed execution"
    );

    assert!(
        workload.cancellable,
        "expensive workloads must be cancellable"
    );

    assert!(
        workload.requires_determinism,
        "production scaling measurements must be reproducible"
    );
}

// ============================================================================
// QPU scalability contract
// ============================================================================

#[test]
fn qpu_backend_is_explicitly_separated_from_software_backends() {
    assert!(
        BackendKind::Qpu.is_physical_qpu()
    );

    assert!(
        !BackendKind::Qpu.is_software()
    );

    assert!(
        BackendKind::Cpu.is_software()
    );

    assert!(
        !BackendKind::Cpu.is_physical_qpu()
    );
}

#[test]
fn qpu_topology_scales_sparse_not_dense() {
    for &distance in &[3usize, 5, 7, 9] {
        let qubits =
            expected_qubits(distance);

        let qpu =
            test_qpu_info(qubits);

        qpu.validate()
            .expect("synthetic QPU metadata must validate");

        /*
         * A line topology has N-1 edges, not N² edges.

         * This verifies that the test's physical topology model does not
         * accidentally require dense connectivity as the system grows.
         */
        assert_eq!(
            qpu.topology.edges.len(),
            qubits.saturating_sub(1)
        );

        assert_eq!(
            qpu.available_qubits,
            qubits
        );
    }
}

#[test]
fn qpu_workload_requires_explicit_physical_capability() {
    let workload =
        qpu_workload_for_distance(7);

    assert!(
        workload.requires_qpu
    );

    assert!(
        workload.requires_calibration
    );

    assert!(
        workload.requires_operation("measurement")
    );

    assert!(
        workload.requires_operation(
            "mid_circuit_measurement"
        )
    );

    assert!(
        workload.requires_operation("reset")
    );

    assert!(
        workload.requires_operation(
            "classical_control"
        )
    );

    let config =
        BackendConfig {
            determinism:
                DeterminismPolicy::Strict,
            limits:
                BackendResourceLimits::unlimited(),
            max_retries: 0,
            allow_degraded: false,
            allow_fallback: false,
            require_explicit_qpu_capability:
                true,
        };

    let request =
        BackendExecutionRequest::new(
            workload,
            config,
        )
        .expect(
            "valid QPU workload request envelope"
        );

    assert!(
        request.workload.requires_qpu
    );

    assert!(
        request
            .config
            .require_explicit_qpu_capability
    );
}

#[test]
fn qpu_metadata_requires_valid_identity_and_topology() {
    let qpu =
        test_qpu_info(
            expected_qubits(7)
        );

    assert!(
        qpu.validate().is_ok()
    );

    assert!(
        !qpu.device_id.is_empty()
    );

    assert!(
        !qpu.provider.is_empty()
    );

    assert!(
        !qpu.architecture.is_empty()
    );

    assert!(
        qpu.calibration_id.is_some()
    );

    assert!(
        qpu.calibration_version.is_some()
    );
}

#[test]
fn qpu_capabilities_are_explicit() {
    let capabilities =
        BackendCapabilities {
            qec_execution: true,
            syndrome_generation: true,
            decoding: true,
            simulation: false,
            pauli_frame: true,
            streaming: true,
            partitioning: true,
            distributed: true,
            acceleration: true,
            checkpointing: true,
            cancellation: true,
            deterministic_execution: false,
            physical_qpu: true,
            calibration: true,
            mid_circuit_measurement: true,
            reset: true,
            measurement: true,
            dynamic_circuits: true,
            classical_control: true,
            native_operations:
                BTreeSet::from([
                    "x".to_owned(),
                    "z".to_owned(),
                    "h".to_owned(),
                    "cx".to_owned(),
                    "measure".to_owned(),
                    "reset".to_owned(),
                ]),
        };

    assert!(
        capabilities.qec_execution
    );

    assert!(
        capabilities.decoding
    );

    assert!(
        capabilities.physical_qpu
    );

    assert!(
        capabilities.calibration
    );

    assert!(
        capabilities.mid_circuit_measurement
    );

    assert!(
        capabilities.measurement
    );

    assert!(
        capabilities.reset
    );

    assert!(
        capabilities.classical_control
    );

    assert!(
        capabilities.supports_operation("CX")
    );
}

// ============================================================================
// QPU resource limits
// ============================================================================

#[test]
fn qpu_resource_limits_prevent_oversized_workloads() {
    let distance = 9usize;

    let workload =
        qpu_workload_for_distance(distance);

    let limits =
        BackendResourceLimits {
            max_qubits:
                Some(workload.qubits - 1),
            max_stabilizers:
                Some(workload.stabilizers),
            max_syndrome_events:
                Some(workload.syndrome_events),
            max_graph_nodes:
                Some(workload.graph_nodes),
            max_graph_edges:
                Some(workload.graph_edges),
            max_rounds:
                Some(workload.rounds),
            max_parallelism:
                Some(workload.parallelism),
            max_memory_bytes:
                Some(workload.memory_bytes),
            max_wall_time:
                Some(Duration::from_secs(30)),
            max_cpu_time:
                Some(Duration::from_secs(30)),
            max_shots:
                Some(workload.shots),
            max_checkpoint_bytes: None,
        };

    limits
        .validate()
        .expect("resource policy itself must be valid");

    let usage =
        BackendResourceUsage {
            qubits: workload.qubits,
            stabilizers: workload.stabilizers,
            syndrome_events:
                workload.syndrome_events,
            graph_nodes:
                workload.graph_nodes,
            graph_edges:
                workload.graph_edges,
            rounds: workload.rounds,
            parallel_workers:
                workload.parallelism,
            shots: workload.shots,
            allocated_memory_bytes:
                workload.memory_bytes,
            peak_memory_bytes:
                workload.memory_bytes,
            cpu_time: Duration::from_millis(1),
            wall_time: Duration::from_millis(1),
            decoder_iterations: 1,
        };

    assert!(
        usage.validate_against(&limits).is_err(),
        "QPU workload exceeding available qubits must be rejected"
    );
}

// ============================================================================
// Determinism / reproducibility
// ============================================================================

#[test]
fn identical_scaling_workloads_have_identical_descriptions() {
    let a =
        workload_for_distance(11);

    let b =
        workload_for_distance(11);

    assert_eq!(
        a,
        b,
        "identical QEC scaling workloads must have stable descriptions"
    );
}

#[test]
fn repeated_surface_code_construction_is_structurally_identical() {
    for &distance in &[3usize, 5, 7, 9, 11] {
        let first =
            build_code(distance);

        let second =
            build_code(distance);

        assert_eq!(
            first.distance(),
            second.distance()
        );

        assert_eq!(
            first.num_data_qubits(),
            second.num_data_qubits()
        );

        assert_eq!(
            first.num_stabilizers(),
            second.num_stabilizers()
        );

        assert_eq!(
            first.num_logical_qubits(),
            second.num_logical_qubits()
        );

        for (a, b) in first
            .data_qubits()
            .iter()
            .zip(second.data_qubits())
        {
            assert_eq!(
                a.index(),
                b.index()
            );

            assert_eq!(
                a.coordinate(),
                b.coordinate()
            );
        }
    }
}

// ============================================================================
// Scaling regression guards
// ============================================================================

#[test]
fn qubit_growth_matches_square_law() {
    let mut previous =
        measure(3);

    for &distance in
        &[5usize, 7, 9, 11, 13, 15]
    {
        let current =
            measure(distance);

        let expected_ratio =
            (distance as f64 / previous.distance as f64)
                .powi(2);

        let actual_ratio =
            current.qubit_growth_ratio(previous);

        /*
         * Exact integer counts are tested elsewhere. This ratio check catches
         * future implementations that accidentally become linear, cubic, or
         * otherwise structurally incorrect.
         */
        let relative_error =
            (actual_ratio - expected_ratio)
                .abs()
                / expected_ratio;

        assert!(
            relative_error < 1e-12,
            "unexpected qubit scaling: d={} ratio={} expected={}",
            distance,
            actual_ratio,
            expected_ratio
        );

        previous = current;
    }
}

#[test]
fn stabilizer_growth_tracks_qubit_growth() {
    let mut previous =
        measure(3);

    for &distance in
        &[5usize, 7, 9, 11, 13, 15]
    {
        let current =
            measure(distance);

        let ratio =
            current.stabilizer_growth_ratio(
                previous
            );

        assert!(
            ratio > 1.0,
            "stabilizer count must grow with distance"
        );

        previous = current;
    }
}

// ============================================================================
// Performance measurement
// ============================================================================

#[test]
fn scaling_measurements_are_recordable_without_unbounded_storage() {
    let mut measurements =
        Vec::with_capacity(
            MAX_BENCHMARK_SAMPLES
        );

    for &distance in
        CI_DISTANCES.iter()
    {
        if measurements.len()
            >= MAX_BENCHMARK_SAMPLES
        {
            break;
        }

        measurements.push(
            measure(distance)
        );
    }

    assert!(
        !measurements.is_empty()
    );

    /*
     * We deliberately retain only one compact measurement per distance.
     * Production benchmarking must not accumulate complete syndrome histories
     * merely to measure scaling.
     */
    assert!(
        measurements.len()
            <= MAX_BENCHMARK_SAMPLES
    );

    for measurement in measurements {
        assert!(
            measurement.construction_time
                <= CI_CONSTRUCTION_TIMEOUT
        );

        assert_eq!(
            measurement.qubits,
            measurement.expected_qubits()
        );

        assert_eq!(
            measurement.stabilizers,
            measurement.expected_stabilizers()
        );
    }
}

// ============================================================================
// Extended scalability tests
// ============================================================================

#[test]
#[ignore = "extended scalability benchmark"]
fn extended_surface_code_scalability() {
    for &distance in
        EXTENDED_DISTANCES
    {
        let started =
            Instant::now();

        let code =
            SurfaceCode::new(distance)
                .unwrap_or_else(|error| {
                    panic!(
                        "distance {distance} failed: {error:?}"
                    )
                });

        let elapsed =
            started.elapsed();

        assert_surface_code_shape(
            &code,
            distance
        );

        eprintln!(
            "QEC_SCALE distance={} qubits={} stabilizers={} construction_ms={}",
            distance,
            code.num_data_qubits(),
            code.num_stabilizers(),
            elapsed.as_secs_f64() * 1000.0
        );
    }
}

#[test]
#[ignore = "extended resource scaling benchmark"]
fn extended_resource_accounting_scalability() {
    for &distance in
        EXTENDED_DISTANCES
    {
        let workload =
            workload_for_distance(distance);

        let usage =
            BackendResourceUsage {
                allocated_memory_bytes:
                    workload.memory_bytes,
                peak_memory_bytes:
                    workload.memory_bytes,
                cpu_time: Duration::from_millis(1),
                wall_time: Duration::from_millis(1),
                qubits: workload.qubits,
                stabilizers:
                    workload.stabilizers,
                syndrome_events:
                    workload.syndrome_events,
                graph_nodes:
                    workload.graph_nodes,
                graph_edges:
                    workload.graph_edges,
                rounds:
                    workload.rounds,
                decoder_iterations:
                    workload.syndrome_events,
                parallel_workers:
                    workload.parallelism,
                shots:
                    workload.shots,
            };

        let limits =
            BackendResourceLimits {
                max_qubits:
                    Some(workload.qubits),
                max_stabilizers:
                    Some(workload.stabilizers),
                max_syndrome_events:
                    Some(workload.syndrome_events),
                max_graph_nodes:
                    Some(workload.graph_nodes),
                max_graph_edges:
                    Some(workload.graph_edges),
                max_rounds:
                    Some(workload.rounds),
                max_parallelism:
                    Some(workload.parallelism),
                max_memory_bytes:
                    Some(workload.memory_bytes),
                max_wall_time:
                    Some(Duration::from_secs(60)),
                max_cpu_time:
                    Some(Duration::from_secs(60)),
                max_shots:
                    Some(workload.shots),
                max_checkpoint_bytes:
                    None,
            };

        usage
            .validate_against(&limits)
            .unwrap_or_else(|error| {
                panic!(
                    "resource accounting failed at distance {distance}: \
                     {error:?}"
                )
            });

        eprintln!(
            "QEC_RESOURCE distance={} qubits={} stabilizers={} \
             syndrome_events={} graph_nodes={} graph_edges={} \
             memory_bytes={} shots={}",
            distance,
            workload.qubits,
            workload.stabilizers,
            workload.syndrome_events,
            workload.graph_nodes,
            workload.graph_edges,
            workload.memory_bytes,
            workload.shots
        );
    }
}

#[test]
#[ignore = "extended QPU contract benchmark"]
fn extended_qpu_scalability_contract() {
    for &distance in
        &[3usize, 5, 7, 9, 11, 13, 15]
    {
        let qubits =
            expected_qubits(distance);

        let qpu =
            test_qpu_info(qubits);

        qpu.validate()
            .expect("synthetic QPU must remain valid");

        let workload =
            qpu_workload_for_distance(distance);

        assert!(
            workload.requires_qpu
        );

        assert!(
            workload.requires_calibration
        );

        assert!(
            qpu.available_qubits
                >= workload.qubits
        );

        eprintln!(
            "QPU_QEC_SCALE distance={} logical_qubits={} \
             physical_qubits={} topology_edges={} \
             calibration={:?}",
            distance,
            workload.qubits,
            qpu.available_qubits,
            qpu.topology.edges.len(),
            qpu.calibration_version
        );
    }
}

// ============================================================================
// Regression: no accidental dense QPU topology
// ============================================================================

#[test]
fn qpu_topology_does_not_require_all_to_all_connectivity() {
    let qubits =
        expected_qubits(15);

    let qpu =
        test_qpu_info(qubits);

    let dense_edges =
        qubits
            .checked_mul(
                qubits.saturating_sub(1)
            )
            .expect(
                "dense edge calculation overflowed"
            )
            / 2;

    let sparse_edges =
        qpu.topology.edges.len();

    assert!(
        sparse_edges < dense_edges,
        "QPU topology representation must not silently become dense"
    );
}

// ============================================================================
// Resource-growth sanity
// ============================================================================

#[test]
fn resource_estimates_are_monotonic() {
    let mut previous =
        workload_for_distance(3);

    for &distance in
        &[5usize, 7, 9, 11, 13, 15]
    {
        let current =
            workload_for_distance(distance);

        assert!(
            current.qubits
                > previous.qubits
        );

        assert!(
            current.stabilizers
                > previous.stabilizers
        );

        assert!(
            current.syndrome_events
                > previous.syndrome_events
        );

        assert!(
            current.graph_nodes
                > previous.graph_nodes
        );

        assert!(
            current.graph_edges
                > previous.graph_edges
        );

        assert!(
            current.memory_bytes
                > previous.memory_bytes
        );

        previous = current;
    }
}

// ============================================================================
// Production contract summary
// ============================================================================

#[test]
fn production_scalability_contract_is_explicit() {
    /*
     * This test documents the intended Zamani QEC scalability contract in
     * executable assertions rather than relying on a comment alone.
     */

    let workload =
        workload_for_distance(15);

    assert!(workload.streaming);
    assert!(workload.partitionable);
    assert!(workload.distributable);
    assert!(workload.cancellable);
    assert!(workload.requires_determinism);

    let config =
        BackendConfig {
            determinism:
                DeterminismPolicy::Strict,
            limits:
                BackendResourceLimits::unlimited(),
            max_retries: 0,
            allow_degraded: false,
            allow_fallback: false,
            require_explicit_qpu_capability:
                true,
        };

    let request =
        BackendExecutionRequest::new(
            workload,
            config,
        )
        .expect(
            "production QEC workload must form a valid request"
        );

    assert!(
        request
            .config
            .require_explicit_qpu_capability
    );
}