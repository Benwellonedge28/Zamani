//! Zamani Quantum Routing — Production Routing Benchmark
//!
//! Path:
//! `src/quantum/routing/benches/routing.rs`
//!
//! # Purpose
//!
//! This benchmark measures the externally observable performance of the
//! routing subsystem without depending on private implementation details.
//!
//! It deliberately benchmarks through the stable routing contracts:
//!
//! ```text
//! QuantumOperation
//!       │
//!       ▼
//! QubitMapping
//!       │
//!       ▼
//! Topology
//!       │
//!       ▼
//! RoutingConfig
//!       │
//!       ▼
//! BasicRouter
//!       │
//!       ▼
//! RoutingResult
//! ```
//!
//! This makes the benchmark resilient to internal changes such as:
//!
//! - replacing BFS;
//! - changing candidate generation;
//! - changing mapping storage;
//! - optimizing SWAP selection;
//! - introducing cached distances;
//! - replacing the BasicRouter implementation;
//! - adding result metrics;
//! - changing internal data structures.
//!
//! The benchmark must only change when the public routing contract itself
//! changes.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Edition 2021;
//! - stable Rust;
//! - no nightly features;
//! - no `#[bench]`;
//! - no `test` crate;
//! - no `unsafe`.
//!
//! # Safety
//!
//! This file explicitly denies unsafe Rust.
//!
//! # Stable benchmark strategy
//!
//! Rust's built-in `#[bench]` attribute is nightly-only. Zamani is explicitly
//! a stable Rust project, so this benchmark is a normal executable benchmark
//! harness using:
//!
//! - `std::time::Instant`;
//! - `std::hint::black_box`;
//! - deterministic workloads;
//! - bounded warm-up;
//! - bounded measured iterations;
//! - explicit result validation.
//!
//! The Cargo target must therefore use:
//!
//! ```toml
//! [[bench]]
//! name = "routing"
//! path = "src/quantum/routing/benches/routing.rs"
//! harness = false
//! ```
//!
//! No other routing source file needs to be modified for this benchmark.
//!
//! The repository currently has a custom stable benchmark precedent in
//! `benches/compiler_bench.rs`, which also uses `Instant`, `black_box`, and a
//! custom `main` instead of nightly `#[bench]`.
//!
//! # What is measured
//!
//! The benchmark covers four distinct classes of work:
//!
//! 1. topology construction;
//! 2. mapping construction;
//! 3. already-executable routing;
//! 4. increasingly difficult non-local routing.
//!
//! Routing workloads cover:
//!
//! - 4 qubits;
//! - 8 qubits;
//! - 16 qubits;
//! - 32 qubits;
//! - 64 qubits;
//! - 128 qubits.
//!
//! For each size we measure:
//!
//! - an already-local circuit;
//! - a nearest-neighbor-heavy circuit;
//! - a deliberately non-local circuit.
//!
//! This gives the routing team a way to detect both:
//!
//! - fixed overhead regressions;
//! - scaling regressions.
//!
//! # Important benchmark rule
//!
//! This benchmark does NOT assert that BasicRouter is globally optimal.
//!
//! The BasicRouter contract is correctness/determinism-oriented. The benchmark
//! therefore records:
//!
//! - elapsed time;
//! - operations processed;
//! - inserted SWAPs;
//! - final operation count.
//!
//! It does not treat a particular SWAP count as an immutable implementation
//! contract.
//!
//! # Reproducibility
//!
//! Workloads are generated entirely deterministically from the requested
//! qubit count. No:
//!
//! - wall-clock-dependent workload generation;
//! - OS randomness;
//! - environment variables;
//! - network access;
//! - filesystem access;
//! - provider API;
//! - global mutable state
//!
//! is used.
//!
//! # Benchmark interpretation
//!
//! A single timing result is not a statistically rigorous hardware benchmark.
//! This harness is intended for:
//!
//! - regression detection;
//! - algorithm-development comparison;
//! - local performance profiling;
//! - CI performance smoke tests;
//! - release-to-release routing comparison.
//!
//! For rigorous statistical performance analysis, a future dedicated stable
//! benchmarking framework may consume these same public routing APIs.
//!
//! # Integration boundary
//!
//! This file intentionally depends on:
//!
//! ```text
//! routing::algorithms::basic
//! routing::config
//! routing::mapping
//! routing::topology
//! routing::types
//! ```
//!
//! It does NOT depend on:
//!
//! - `transpiler.rs`;
//! - compiler IR;
//! - OpenQASM;
//! - hardware providers;
//! - scheduler;
//! - pulse compiler;
//! - simulator;
//! - QEC;
//! - provider SDKs.
//!
//! This is intentional. Routing performance must be measurable independently
//! of unrelated compiler and hardware layers.
//!
//! # Future algorithm integration
//!
//! When SABRE, lookahead, noise-aware, or other routing algorithms are
//! production-ready, they should be benchmarked by additional functions that
//! consume the same stable:
//!
//! ```text
//! operations + topology + mapping + config
//! ```
//!
//! contract.
//!
//! This file does not need to be redesigned merely because another algorithm
//! is added.
//!
//! # No unsafe
//!
//! `#![deny(unsafe_code)]` is intentional and must remain permanent.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::hint::black_box;
use std::time::{Duration, Instant};

use zamani_compiler::quantum::routing::algorithms::basic::BasicRouter;
use zamani_compiler::quantum::routing::config::{
    RoutingAlgorithm,
    RoutingConfig,
    RoutingObjective,
    VerificationLevel,
};
use zamani_compiler::quantum::routing::mapping::QubitMapping;
use zamani_compiler::quantum::routing::topology::Topology;
use zamani_compiler::quantum::routing::types::{
    GateIdentity,
    LogicalQubitId,
    PhysicalQubitId,
    QuantumOperation,
};

// =============================================================================
// Benchmark constants
// =============================================================================

/// Smallest benchmarked circuit/device size.
const MIN_QUBITS: usize = 4;

/// Largest benchmarked circuit/device size.
///
/// This is deliberately bounded. A benchmark must not accidentally turn into
/// an unbounded resource-consumption test merely because somebody runs it in
/// CI.
const MAX_QUBITS: usize = 128;

/// Warm-up iterations per benchmark case.
///
/// Warm-up is intentionally small because routing itself can be expensive.
const WARMUP_ITERATIONS: usize = 3;

/// Default measured iterations.
///
/// The harness automatically reduces the count for expensive large cases.
const DEFAULT_MEASURED_ITERATIONS: usize = 20;

/// Minimum measured iterations for any case.
const MIN_MEASURED_ITERATIONS: usize = 3;

/// Maximum number of generated operations per workload.
///
/// This prevents accidental benchmark explosions if workload generation is
/// changed later.
const MAX_OPERATIONS: usize = 4_096;

/// Maximum acceptable benchmark-case runtime before reducing repetitions.
///
/// This does NOT terminate a single routing operation. It only prevents a
/// benchmark suite from multiplying an unexpectedly expensive workload by a
/// large iteration count.
const TARGET_CASE_TIME: Duration = Duration::from_millis(750);

// =============================================================================
// Benchmark workload
// =============================================================================

/// A deterministic routing workload.
#[derive(Clone)]
struct Workload {
    name: &'static str,
    qubit_count: usize,
    operations: Vec<QuantumOperation>,
    topology: Topology,
    mapping: QubitMapping,
    config: RoutingConfig,
}

/// Result of one benchmark case.
#[derive(Debug, Clone, Copy)]
struct BenchmarkMeasurement {
    elapsed: Duration,
    iterations: usize,
    operations: usize,
    inserted_swaps: usize,
    final_operations: usize,
}

impl BenchmarkMeasurement {
    fn average_duration(self) -> Duration {
        if self.iterations == 0 {
            return Duration::ZERO;
        }

        let nanos = self.elapsed.as_nanos();
        let iterations = self.iterations as u128;

        let average = nanos / iterations;

        if average > u64::MAX as u128 {
            Duration::MAX
        } else {
            Duration::from_nanos(average as u64)
        }
    }

    fn average_micros(self) -> f64 {
        self.average_duration().as_secs_f64() * 1_000_000.0
    }
}

// =============================================================================
// Identifier helpers
// =============================================================================

#[inline]
fn logical(index: usize) -> LogicalQubitId {
    LogicalQubitId::new(index)
}

#[inline]
fn physical(index: usize) -> PhysicalQubitId {
    PhysicalQubitId::new(index)
}

// =============================================================================
// Configuration
// =============================================================================

/// Production benchmark configuration.
///
/// Verification is deliberately enabled for correctness-sensitive routing
/// benchmarks. A performance-only run can later be introduced as a separate
/// explicitly named benchmark if necessary; this benchmark must not silently
/// trade correctness for speed.
fn benchmark_config() -> RoutingConfig {
    RoutingConfig {
        algorithm: RoutingAlgorithm::Basic,
        objective: RoutingObjective::SwapCount,
        verify_output: true,
        verification_level: VerificationLevel::Standard,
        ..RoutingConfig::default()
    }
}

// =============================================================================
// Topology construction
// =============================================================================

/// Builds a deterministic linear topology.
///
/// ```text
/// p0 -- p1 -- p2 -- ... -- pN
/// ```
fn line_topology(qubit_count: usize) -> Topology {
    assert!(
        (MIN_QUBITS..=MAX_QUBITS).contains(&qubit_count),
        "benchmark qubit count must be between {MIN_QUBITS} and {MAX_QUBITS}"
    );

    Topology::line(qubit_count)
        .expect("benchmark line topology must be valid")
}

// =============================================================================
// Mapping construction
// =============================================================================

/// Creates the identity logical-to-physical mapping.
///
/// ```text
/// q0 -> p0
/// q1 -> p1
/// q2 -> p2
/// ...
/// ```
fn identity_mapping(qubit_count: usize) -> QubitMapping {
    let mut mapping = QubitMapping::new();

    for index in 0..qubit_count {
        mapping
            .assign(logical(index), physical(index))
            .expect("identity benchmark mapping must be valid");
    }

    mapping
}

// =============================================================================
// Operation construction
// =============================================================================

/// Creates a generic single-qubit gate.
#[inline]
fn single_qubit_gate(qubit: usize) -> QuantumOperation {
    QuantumOperation::new(
        GateIdentity::H,
        vec![logical(qubit)],
    )
}

/// Creates a two-qubit CX operation.
#[inline]
fn cx(first: usize, second: usize) -> QuantumOperation {
    QuantumOperation::new(
        GateIdentity::Cx,
        vec![logical(first), logical(second)],
    )
}

// =============================================================================
// Deterministic workloads
// =============================================================================

/// Generates an already-executable workload.
///
/// This isolates routing overhead when no movement is required.
fn build_local_workload(qubit_count: usize) -> Workload {
    let topology = line_topology(qubit_count);
    let mapping = identity_mapping(qubit_count);

    let mut operations = Vec::new();

    // A deterministic mixture of single-qubit and adjacent two-qubit gates.
    //
    // Every CX is physically adjacent under the identity mapping.
    for index in 0..qubit_count {
        operations.push(single_qubit_gate(index));

        if index + 1 < qubit_count {
            operations.push(cx(index, index + 1));
        }
    }

    assert!(
        operations.len() <= MAX_OPERATIONS,
        "local benchmark exceeded operation bound"
    );

    Workload {
        name: "local",
        qubit_count,
        operations,
        topology,
        mapping,
        config: benchmark_config(),
    }
}

/// Generates a nearest-neighbor-heavy workload.
///
/// This exercises repeated routing decisions while keeping most interactions
/// local. It is useful for detecting regressions in the common case.
fn build_nearest_neighbor_workload(
    qubit_count: usize,
) -> Workload {
    let topology = line_topology(qubit_count);
    let mapping = identity_mapping(qubit_count);

    let mut operations = Vec::new();

    for round in 0..4 {
        for index in 0..qubit_count {
            if (index + round) % 3 == 0 {
                operations.push(single_qubit_gate(index));
            }

            if index + 1 < qubit_count
                && (index + round) % 2 == 0
            {
                operations.push(cx(index, index + 1));
            }
        }
    }

    assert!(
        operations.len() <= MAX_OPERATIONS,
        "nearest-neighbor benchmark exceeded operation bound"
    );

    Workload {
        name: "nearest_neighbor",
        qubit_count,
        operations,
        topology,
        mapping,
        config: benchmark_config(),
    }
}

/// Generates a deliberately non-local workload.
///
/// The logical interactions are intentionally between distant positions in a
/// linear topology. This stresses candidate generation, shortest paths,
/// mapping evolution, and SWAP insertion.
fn build_nonlocal_workload(
    qubit_count: usize,
) -> Workload {
    let topology = line_topology(qubit_count);
    let mapping = identity_mapping(qubit_count);

    let mut operations = Vec::new();

    let half = qubit_count / 2;

    for offset in 0..half {
        let left = offset;
        let right = qubit_count - 1 - offset;

        if left != right {
            operations.push(cx(left, right));
        }
    }

    // Add a deterministic second wave in the opposite direction. This tests
    // mapping evolution rather than only one isolated route.
    for offset in 0..half {
        let left = offset;
        let right = qubit_count - 1 - offset;

        if left != right {
            operations.push(cx(right, left));
        }
    }

    assert!(
        operations.len() <= MAX_OPERATIONS,
        "non-local benchmark exceeded operation bound"
    );

    Workload {
        name: "nonlocal",
        qubit_count,
        operations,
        topology,
        mapping,
        config: benchmark_config(),
    }
}

// =============================================================================
// Correctness validation
// =============================================================================

/// Validates a benchmark result using only the public routing contract.
///
/// Benchmark code must not silently accept a failed or malformed route.
fn validate_result(
    workload: &Workload,
    result: &zamani_compiler::quantum::routing::result::RoutingResult,
) {
    result
        .final_mapping()
        .validate(&workload.topology)
        .expect("benchmark routing must return a valid final mapping");

    let metrics = result.metrics();

    assert_eq!(
        metrics.original_operations,
        workload.operations.len(),
        "routing must preserve original operation count"
    );

    assert_eq!(
        metrics.final_operations,
        result.operations().len(),
        "routing metrics must agree with returned operation count"
    );

    assert!(
        metrics.final_operations >= metrics.original_operations,
        "routing cannot remove operations"
    );

    assert_eq!(
        metrics.final_operations - metrics.original_operations,
        metrics.inserted_swaps,
        "benchmark currently uses semantic SWAP-only routing overhead"
    );
}

// =============================================================================
// Single routing invocation
// =============================================================================

/// Executes one routing invocation and validates the result.
///
/// `black_box` is applied to both the input workload and result boundary so
/// compiler optimization cannot trivially eliminate the benchmarked call.
#[inline(never)]
fn route_once(
    workload: &Workload,
    router: &BasicRouter,
) -> usize {
    let operations = black_box(&workload.operations);
    let topology = black_box(&workload.topology);
    let mapping = black_box(&workload.mapping);
    let config = black_box(&workload.config);

    let result = router
        .route_with_mapping(
            operations,
            topology,
            mapping,
            config,
        )
        .expect("production benchmark routing must succeed");

    validate_result(workload, &result);

    let final_operations = result.operations().len();

    black_box(result);
    black_box(final_operations)
}

// =============================================================================
// Measurement
// =============================================================================

/// Performs warm-up iterations.
fn warm_up(
    workload: &Workload,
    router: &BasicRouter,
) {
    for _ in 0..WARMUP_ITERATIONS {
        black_box(route_once(workload, router));
    }
}

/// Measures a workload for a bounded number of iterations.
fn measure(
    workload: &Workload,
    router: &BasicRouter,
) -> BenchmarkMeasurement {
    warm_up(workload, router);

    let mut iterations = DEFAULT_MEASURED_ITERATIONS;

    // Start with a bounded number of iterations. If the workload is already
    // expensive, reduce repetition after measuring a small probe.
    let probe_iterations = MIN_MEASURED_ITERATIONS;

    let probe_start = Instant::now();

    let mut probe_swaps = 0usize;
    let mut probe_final_operations = 0usize;

    for _ in 0..probe_iterations {
        let operations = black_box(&workload.operations);
        let topology = black_box(&workload.topology);
        let mapping = black_box(&workload.mapping);
        let config = black_box(&workload.config);

        let result = router
            .route_with_mapping(
                operations,
                topology,
                mapping,
                config,
            )
            .expect("benchmark probe routing must succeed");

        validate_result(workload, &result);

        probe_swaps = probe_swaps
            .checked_add(result.metrics().inserted_swaps)
            .expect("benchmark SWAP counter overflow");

        probe_final_operations = result.operations().len();

        black_box(result);
    }

    let probe_elapsed = probe_start.elapsed();

    if probe_elapsed >= TARGET_CASE_TIME {
        iterations = MIN_MEASURED_ITERATIONS;
    } else {
        // Keep the full benchmark bounded even on extremely fast machines.
        iterations = iterations.max(MIN_MEASURED_ITERATIONS);
    }

    let start = Instant::now();

    let mut inserted_swaps = 0usize;
    let mut final_operations = 0usize;

    for _ in 0..iterations {
        let operations = black_box(&workload.operations);
        let topology = black_box(&workload.topology);
        let mapping = black_box(&workload.mapping);
        let config = black_box(&workload.config);

        let result = router
            .route_with_mapping(
                operations,
                topology,
                mapping,
                config,
            )
            .expect("benchmark routing must succeed");

        validate_result(workload, &result);

        inserted_swaps = inserted_swaps
            .checked_add(result.metrics().inserted_swaps)
            .expect("benchmark SWAP counter overflow");

        final_operations = result.operations().len();

        black_box(result);
    }

    let elapsed = start.elapsed();

    // Keep the probe variables live. They are useful when debugging an
    // unexpectedly changing workload and prevent accidental "unused" removal
    // if the benchmark implementation is later modified.
    black_box(probe_swaps);
    black_box(probe_final_operations);
    black_box(probe_elapsed);

    let representative_swaps = inserted_swaps
        .checked_div(iterations)
        .expect("benchmark iteration count must be non-zero");

    // `representative_swaps` is used as the displayed workload metric. It is
    // expected to be identical for deterministic routing.
    let final_swaps = representative_swaps;

    BenchmarkMeasurement {
        elapsed,
        iterations,
        operations: workload.operations.len(),
        inserted_swaps: final_swaps,
        final_operations,
    }
}

// =============================================================================
// Topology benchmark
// =============================================================================

/// Benchmarks topology construction independently from routing.
///
/// This detects regressions in the topology builder without mixing graph
/// construction time into the routing measurement.
fn measure_topology_construction(
    qubit_count: usize,
    iterations: usize,
) -> Duration {
    let start = Instant::now();

    for _ in 0..iterations {
        let topology = Topology::line(qubit_count)
            .expect("benchmark topology must be valid");

        black_box(topology);
    }

    start.elapsed()
}

// =============================================================================
// Mapping benchmark
// =============================================================================

/// Benchmarks identity mapping construction independently from routing.
fn measure_mapping_construction(
    qubit_count: usize,
    iterations: usize,
) -> Duration {
    let start = Instant::now();

    for _ in 0..iterations {
        let mapping = identity_mapping(qubit_count);

        black_box(mapping);
    }

    start.elapsed()
}

// =============================================================================
// Output
// =============================================================================

fn print_header() {
    println!();
    println!("Zamani Quantum Routing Benchmarks");
    println!("=================================");
    println!("Rust: 1.97.1-compatible stable benchmark harness");
    println!("Unsafe: disabled");
    println!("Verification: standard");
    println!("Algorithm: basic");
    println!();
}

fn print_separator() {
    println!(
        "{:<8} {:<20} {:>12} {:>12} {:>12} {:>12}",
        "Qubits",
        "Workload",
        "Avg µs",
        "Ops",
        "SWAPs",
        "Final Ops"
    );
    println!(
        "{:-<8} {:-<20} {:->12} {:->12} {:->12} {:->12}",
        "",
        "",
        "",
        "",
        "",
        ""
    );
}

fn print_measurement(
    measurement: BenchmarkMeasurement,
    workload_name: &str,
    qubit_count: usize,
) {
    println!(
        "{:<8} {:<20} {:>12.3} {:>12} {:>12} {:>12}",
        qubit_count,
        workload_name,
        measurement.average_micros(),
        measurement.operations,
        measurement.inserted_swaps,
        measurement.final_operations
    );
}

// =============================================================================
// Main benchmark
// =============================================================================

fn main() {
    print_header();

    let router = BasicRouter::new();

    print_separator();

    let benchmark_sizes = [
        4usize,
        8,
        16,
        32,
        64,
        128,
    ];

    for qubit_count in benchmark_sizes {
        let workloads = [
            build_local_workload(qubit_count),
            build_nearest_neighbor_workload(qubit_count),
            build_nonlocal_workload(qubit_count),
        ];

        for workload in &workloads {
            let measurement = measure(
                workload,
                &router,
            );

            print_measurement(
                measurement,
                workload.name,
                workload.qubit_count,
            );
        }
    }

    println!();
    println!("Topology construction");
    println!("=====================");

    println!(
        "{:<8} {:>16}",
        "Qubits",
        "Total µs"
    );

    for qubit_count in benchmark_sizes {
        let iterations = 100usize;

        let elapsed = measure_topology_construction(
            qubit_count,
            iterations,
        );

        println!(
            "{:<8} {:>16.3}",
            qubit_count,
            elapsed.as_secs_f64() * 1_000_000.0
        );
    }

    println!();
    println!("Mapping construction");
    println!("====================");

    println!(
        "{:<8} {:>16}",
        "Qubits",
        "Total µs"
    );

    for qubit_count in benchmark_sizes {
        let iterations = 100usize;

        let elapsed = measure_mapping_construction(
            qubit_count,
            iterations,
        );

        println!(
            "{:<8} {:>16.3}",
            qubit_count,
            elapsed.as_secs_f64() * 1_000_000.0
        );
    }

    println!();
    println!("Benchmark completed successfully.");
}