//! Zamani Quantum Routing — SABRE Benchmark
//!
//! Path:
//! `src/quantum/routing/benches/sabre.rs`
//!
//! # Purpose
//!
//! Stable-Rust production benchmark for the SABRE routing implementation.
//!
//! This benchmark measures SABRE through the public routing contracts:
//!
//! ```text
//! RoutingWorkload
//!       │
//!       ├── Topology
//!       ├── QubitMapping
//!       └── RoutingConfig
//!              │
//!              ▼
//!         SabreRouter
//!              │
//!              ▼
//!        RoutingResult
//!              │
//!              ▼
//!        metrics/results
//! ```
//!
//! The benchmark deliberately does not access SABRE's private implementation
//! state. Consequently, internal changes such as:
//!
//! - replacing distance-cache implementation;
//! - changing candidate generation;
//! - changing mapping storage;
//! - changing heuristic implementation;
//! - adding LightSABRE optimizations;
//! - changing internal data structures;
//! - adding parallel candidate evaluation;
//!
//! do not require this benchmark to be rewritten as long as the public routing
//! contracts remain compatible.
//!
//! # Benchmark scope
//!
//! This file benchmarks SABRE itself, not:
//!
//! - compiler parsing;
//! - OpenQASM parsing;
//! - compiler IR construction;
//! - gate decomposition;
//! - hardware provider APIs;
//! - scheduling;
//! - pulse generation;
//! - quantum simulation;
//! - QEC decoding;
//! - benchmark-result persistence;
//! - network communication.
//!
//! Those responsibilities belong to other Zamani subsystems.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no `#[bench]`;
//! - no `test` crate;
//! - no unsafe code.
//!
//! The benchmark is a normal executable benchmark target and therefore must
//! be registered in `Cargo.toml` with:
//!
//! ```toml
//! [[bench]]
//! name = "sabre"
//! path = "src/quantum/routing/benches/sabre.rs"
//! harness = false
//! ```
//!
//! This benchmark requires no additional crate dependency.
//!
//! # Safety
//!
//! Unsafe Rust is forbidden permanently.
//!
//! # Reproducibility
//!
//! All benchmark workloads are generated deterministically.
//!
//! Every SABRE invocation uses:
//!
//! - deterministic execution;
//! - an explicit seed;
//! - deterministic topology construction;
//! - deterministic logical-qubit ordering;
//! - deterministic interaction ordering;
//! - fixed routing configuration.
//!
//! Timing itself is naturally machine-dependent and is therefore not part of
//! the workload identity.
//!
//! # Important benchmark principle
//!
//! This benchmark never treats one exact SWAP count as an immutable SABRE
//! implementation contract.
//!
//! SABRE is a heuristic algorithm. Future improvements may legitimately
//! produce a different route while remaining correct and improving another
//! objective.
//!
//! The benchmark therefore records:
//!
//! - elapsed routing time;
//! - average routing time;
//! - logical qubit count;
//! - original operation count;
//! - final operation count;
//! - inserted SWAPs;
//! - routing iterations;
//! - candidate evaluations;
//! - candidate rejections;
//! - final depth;
//! - routing overhead.
//!
//! Correctness is validated separately from performance measurement.
//!
//! # Benchmark cases
//!
//! The suite covers:
//!
//! 1. local interactions;
//! 2. nearest-neighbor interactions;
//! 3. non-local interactions;
//! 4. dense long-range interactions;
//! 5. repeated long-range interactions.
//!
//! Device sizes are bounded to:
//!
//! - 8;
//! - 16;
//! - 32;
//! - 64;
//! - 128 physical qubits.
//!
//! The largest workloads are deliberately bounded so that invoking the
//! benchmark cannot accidentally become an unbounded resource-consumption
//! test.
//!
//! # Benchmark methodology
//!
//! Each case is executed in three stages:
//!
//! ```text
//! deterministic workload construction
//!          │
//!          ▼
//! warm-up routing
//!          │
//!          ▼
//! measured routing
//! ```
//!
//! Workload construction is excluded from the measured routing interval.
//!
//! Result validation is performed for every measured invocation, but the
//! validation result is consumed with `black_box` so that the benchmark cannot
//! optimize away the result boundary.
//!
//! # Integration contract
//!
//! This file depends only on:
//!
//! ```text
//! quantum::routing::algorithms::sabre
//! quantum::routing::config
//! quantum::routing::mapping
//! quantum::routing::topology
//! quantum::routing::types
//! ```
//!
//! It intentionally does not depend on:
//!
//! ```text
//! quantum::routing::transpiler
//! compiler IR
//! OpenQASM
//! hardware providers
//! scheduler
//! simulator
//! QEC
//! ```
//!
//! This keeps SABRE performance independently measurable.
//!
//! # Production requirements represented here
//!
//! The benchmark intentionally exercises:
//!
//! - deterministic routing;
//! - seeded routing;
//! - front-layer processing;
//! - lookahead;
//! - decay heuristic;
//! - candidate generation;
//! - mapping evolution;
//! - SWAP insertion;
//! - bidirectional SABRE iterations;
//! - result metrics;
//! - final mapping validation;
//! - routing result verification;
//! - bounded search;
//! - large-but-controlled topologies.
//!
//! # Running
//!
//! After registering the target in `Cargo.toml`:
//!
//! ```text
//! cargo bench --bench sabre
//! ```
//!
//! For a release-quality measurement, use a release build and keep the
//! benchmark environment stable. Do not compare one machine's raw timing
//! against another machine's raw timing without accounting for CPU, memory,
//! operating-system, compiler, and system-load differences.
//!
//! # No unsafe
//!
//! This file intentionally contains:
//!
//! ```rust
//! #![deny(unsafe_code)]
//! #![deny(unsafe_op_in_unsafe_fn)]
//! ```
//!
//! These restrictions must remain permanent.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::hint::black_box;
use std::time::{Duration, Instant};

use zamani_compiler::quantum::routing::algorithms::sabre::{
    SabreHeuristic,
    SabreRouter,
};
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
    QubitInteraction,
    RoutingWorkload,
};

// =============================================================================
// Benchmark policy
// =============================================================================

/// Smallest benchmarked topology.
const MIN_QUBITS: usize = 8;

/// Largest benchmarked topology.
///
/// This is intentionally bounded because SABRE is a heuristic search
/// algorithm and the benchmark must remain suitable for local development and
/// CI smoke runs.
const MAX_QUBITS: usize = 128;

/// Number of warm-up invocations.
///
/// Warm-up is deliberately small because SABRE itself may be computationally
/// expensive on non-local workloads.
const WARMUP_ITERATIONS: usize = 2;

/// Default number of measured invocations.
const DEFAULT_MEASURED_ITERATIONS: usize = 8;

/// Minimum measured invocations for any benchmark case.
const MIN_MEASURED_ITERATIONS: usize = 2;

/// Maximum generated interactions in one benchmark workload.
const MAX_INTERACTIONS: usize = 2_048;

/// Maximum generated operations in one benchmark workload.
///
/// SABRE operates on the two-qubit interaction workload, so this is also a
/// protection against accidentally expanding this benchmark into an enormous
/// input generator.
const MAX_OPERATIONS: usize = 4_096;

/// Maximum number of SABRE forward/backward iterations used by this benchmark.
///
/// This is a benchmark policy limit, not a limitation of the routing API.
const BENCHMARK_SABRE_ITERATIONS: usize = 4;

/// Number of routing trials used by each SABRE invocation.
///
/// One explicit trial makes the baseline benchmark stable and keeps the
/// measured work attributable primarily to the SABRE algorithm rather than to
/// an arbitrarily large trial multiplier.
const BENCHMARK_SABRE_TRIALS: usize = 1;

/// Maximum number of candidates evaluated at one routing decision.
const BENCHMARK_CANDIDATE_LIMIT: usize = 64;

/// Lookahead depth used by the production SABRE benchmark.
const BENCHMARK_LOOKAHEAD_DEPTH: usize = 4;

/// Explicit deterministic seed.
///
/// Changing this seed intentionally changes the benchmark workload's SABRE
/// search path. It should therefore be changed only as a deliberate benchmark
/// revision.
const BENCHMARK_SEED: u64 = 0x5A_B4_E2_02_60_00_00_01;

/// Maximum number of routing iterations allowed for one benchmark invocation.
///
/// This prevents an algorithm regression from turning `cargo bench` into an
/// effectively unbounded process.
const BENCHMARK_MAX_ITERATIONS: usize = 100_000;

/// Maximum number of inserted SWAPs permitted for one benchmark invocation.
///
/// A benchmark failure caused by this limit is preferable to silently running
/// indefinitely on a pathological route.
const BENCHMARK_MAX_SWAPS: usize = 1_000_000;

/// Maximum wall-clock duration permitted for one individual routing invocation.
///
/// This is a safety boundary, not a timing target. It protects developer/CI
/// environments from pathological cases while normal benchmark measurements
/// remain comparable for invocations that complete inside the bound.
const BENCHMARK_TIMEOUT: Duration = Duration::from_secs(30);

/// Time budget used to decide whether to reduce measured repetitions.
///
/// The routing operation itself still has its explicit timeout above.
const TARGET_CASE_TIME: Duration = Duration::from_millis(750);

// =============================================================================
// Benchmark workload
// =============================================================================

/// A completely prepared deterministic SABRE workload.
///
/// Construction happens outside the measured routing interval.
#[derive(Clone)]
struct Workload {
    /// Stable benchmark case name.
    name: &'static str,

    /// Number of logical/physical qubits.
    qubit_count: usize,

    /// Canonical routing workload.
    routing_workload: RoutingWorkload,

    /// Physical topology.
    topology: Topology,

    /// Initial logical-to-physical mapping.
    mapping: QubitMapping,

    /// Routing configuration.
    config: RoutingConfig,
}

/// Measured information from one routing invocation.
#[derive(Debug, Clone, Copy)]
struct Measurement {
    elapsed: Duration,
    original_operations: usize,
    final_operations: usize,
    inserted_swaps: usize,
    routing_iterations: usize,
    candidate_evaluations: usize,
    candidate_rejections: usize,
    final_depth: usize,
    routing_depth: usize,
}

/// Aggregate result for a benchmark case.
#[derive(Debug, Clone, Copy)]
struct CaseMeasurement {
    total_elapsed: Duration,
    iterations: usize,
    last: Measurement,
}

impl CaseMeasurement {
    /// Returns the average invocation duration.
    #[must_use]
    fn average_duration(self) -> Duration {
        if self.iterations == 0 {
            return Duration::ZERO;
        }

        let total_nanos = self.total_elapsed.as_nanos();
        let iterations = self.iterations as u128;

        let average_nanos = total_nanos / iterations;

        if average_nanos > u64::MAX as u128 {
            Duration::MAX
        } else {
            Duration::from_nanos(average_nanos as u64)
        }
    }

    /// Returns average routing time in microseconds.
    #[must_use]
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

/// Creates the fixed production SABRE benchmark configuration.
///
/// The configuration intentionally goes through the public builder API rather
/// than constructing internal routing state.
fn benchmark_config() -> RoutingConfig {
    RoutingConfig::default()
        .with_algorithm(RoutingAlgorithm::Sabre)
        .with_objective(RoutingObjective::SwapCount)
        .with_verification(VerificationLevel::Standard)
        .with_deterministic(true)
        .with_seed(BENCHMARK_SEED)
        .with_sabre_iterations(BENCHMARK_SABRE_ITERATIONS)
        .with_sabre_trials(BENCHMARK_SABRE_TRIALS)
        .with_lookahead_depth(BENCHMARK_LOOKAHEAD_DEPTH)
        .with_candidate_limit(BENCHMARK_CANDIDATE_LIMIT)
        .with_max_iterations(BENCHMARK_MAX_ITERATIONS)
        .with_max_swaps(BENCHMARK_MAX_SWAPS)
        .with_timeout(BENCHMARK_TIMEOUT)
        .with_swap(true)
        .with_bridge(false)
        .with_direction_reversal(false)
}

// =============================================================================
// Topology
// =============================================================================

/// Creates a deterministic line topology.
///
/// ```text
/// p0 -- p1 -- p2 -- p3 -- ... -- pN
/// ```
fn line_topology(qubit_count: usize) -> Topology {
    assert!(
        (MIN_QUBITS..=MAX_QUBITS).contains(&qubit_count),
        "benchmark qubit count {qubit_count} is outside supported range"
    );

    Topology::line(qubit_count)
        .expect("deterministic line topology must be valid")
}

// =============================================================================
// Mapping
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
// Interaction construction
// =============================================================================

/// Creates a two-qubit CX interaction.
#[inline]
fn cx(first: usize, second: usize) -> QubitInteraction {
    QubitInteraction::new(
        vec![logical(first), logical(second)],
        GateIdentity::Cx,
    )
}

/// Creates a deterministic single-qubit-free interaction workload.
///
/// SABRE is intentionally benchmarked with two-qubit interactions because
/// routing movement is determined by two-qubit connectivity.
fn make_workload(
    name: &'static str,
    qubit_count: usize,
    interactions: Vec<QubitInteraction>,
) -> Workload {
    assert!(
        (MIN_QUBITS..=MAX_QUBITS).contains(&qubit_count),
        "benchmark qubit count must be between {MIN_QUBITS} and {MAX_QUBITS}"
    );

    assert!(
        !interactions.is_empty(),
        "SABRE benchmark workload must contain at least one interaction"
    );

    assert!(
        interactions.len() <= MAX_INTERACTIONS,
        "benchmark workload exceeds maximum interaction limit"
    );

    let routing_workload = RoutingWorkload::new(
        (0..qubit_count).map(logical).collect(),
        interactions,
    );

    let topology = line_topology(qubit_count);
    let mapping = identity_mapping(qubit_count);
    let config = benchmark_config();

    Workload {
        name,
        qubit_count,
        routing_workload,
        topology,
        mapping,
        config,
    }
}

// =============================================================================
// Workload generators
// =============================================================================

/// Generates adjacent interactions.
///
/// These interactions should require no routing movement under the identity
/// mapping. This measures SABRE's fast/common path where the front layer is
/// already executable.
fn build_local_workload(qubit_count: usize) -> Workload {
    let mut interactions = Vec::with_capacity(qubit_count - 1);

    for index in 0..qubit_count.saturating_sub(1) {
        interactions.push(cx(index, index + 1));
    }

    make_workload("local", qubit_count, interactions)
}

/// Generates repeated nearest-neighbor interactions.
///
/// This stresses repeated front-layer processing and mapping checks without
/// making every interaction long-range.
fn build_nearest_neighbor_workload(
    qubit_count: usize,
) -> Workload {
    let rounds = 8usize;

    let mut interactions = Vec::with_capacity(
        (qubit_count.saturating_sub(1)) * rounds,
    );

    for round in 0..rounds {
        let forward = round % 2 == 0;

        for index in 0..qubit_count.saturating_sub(1) {
            if (index + round) % 2 != 0 {
                continue;
            }

            if forward {
                interactions.push(cx(index, index + 1));
            } else {
                interactions.push(cx(index + 1, index));
            }
        }
    }

    make_workload(
        "nearest_neighbor",
        qubit_count,
        interactions,
    )
}

/// Generates paired long-range interactions.
///
/// For a line topology, the logical endpoints begin far apart and therefore
/// force SABRE to perform substantial mapping movement.
fn build_nonlocal_workload(
    qubit_count: usize,
) -> Workload {
    let pair_count = qubit_count / 2;

    let mut interactions = Vec::with_capacity(pair_count * 2);

    for offset in 0..pair_count {
        let left = offset;
        let right = qubit_count - 1 - offset;

        if left != right {
            interactions.push(cx(left, right));
        }
    }

    for offset in 0..pair_count {
        let left = offset;
        let right = qubit_count - 1 - offset;

        if left != right {
            interactions.push(cx(right, left));
        }
    }

    make_workload("nonlocal", qubit_count, interactions)
}

/// Generates a dense long-range interaction workload.
///
/// Every logical qubit interacts with several distant logical qubits. This
/// gives SABRE a larger front/extended-set search problem than the paired
/// non-local workload.
fn build_dense_nonlocal_workload(
    qubit_count: usize,
) -> Workload {
    let mut interactions = Vec::new();

    let stride = (qubit_count / 3).max(2);

    for first in 0..qubit_count {
        let second = (first + stride) % qubit_count;

        if first != second {
            interactions.push(cx(first, second));
        }

        let third = (first + (stride * 2)) % qubit_count;

        if first != third && second != third {
            interactions.push(cx(first, third));
        }
    }

    make_workload(
        "dense_nonlocal",
        qubit_count,
        interactions,
    )
}

/// Generates a workload that repeatedly changes which logical qubits interact.
///
/// This is intended to exercise mapping evolution rather than only solving one
/// isolated long-range gate.
fn build_repeated_nonlocal_workload(
    qubit_count: usize,
) -> Workload {
    let rounds = 6usize;

    let mut interactions = Vec::new();

    for round in 0..rounds {
        let shift = (round * 3) % qubit_count;

        for index in 0..(qubit_count / 2) {
            let first = (index + shift) % qubit_count;
            let second =
                (qubit_count - 1 - index + shift) % qubit_count;

            if first != second {
                interactions.push(cx(first, second));
            }
        }
    }

    make_workload(
        "repeated_nonlocal",
        qubit_count,
        interactions,
    )
}

// =============================================================================
// Workload validation
// =============================================================================

/// Validates a prepared workload before benchmark execution.
///
/// This is deliberately performed outside the measured interval.
fn validate_workload(workload: &Workload) {
    assert_eq!(
        workload.mapping.len(),
        workload.qubit_count,
        "{} benchmark mapping must cover every logical qubit",
        workload.name
    );

    workload
        .mapping
        .validate()
        .expect("benchmark mapping must be structurally valid");

    workload
        .topology
        .validate()
        .expect("benchmark topology must be valid");

    assert_eq!(
        workload.topology.qubit_count(),
        workload.qubit_count,
        "{} benchmark topology/mapping sizes must agree",
        workload.name
    );

    assert_eq!(
        workload.routing_workload.logical_qubits().len(),
        workload.qubit_count,
        "{} benchmark workload must contain every logical qubit",
        workload.name
    );

    assert!(
        workload.routing_workload.interaction_count() <= MAX_INTERACTIONS,
        "{} benchmark exceeds interaction limit",
        workload.name
    );

    assert!(
        workload.routing_workload.interaction_count() > 0,
        "{} benchmark cannot be empty",
        workload.name
    );

    assert!(
        workload.routing_workload
            .interactions()
            .iter()
            .all(QubitInteraction::is_two_qubit),
        "{} benchmark must contain only two-qubit interactions",
        workload.name
    );

    assert!(
        workload.routing_workload
            .interactions()
            .iter()
            .all(|interaction| {
                interaction
                    .operands()
                    .iter()
                    .all(|qubit| {
                        workload
                            .mapping
                            .contains_logical(*qubit)
                    })
            }),
        "{} benchmark contains an unmapped logical qubit",
        workload.name
    );
}

// =============================================================================
// One routing invocation
// =============================================================================

/// Routes one workload once and validates the returned result.
///
/// This function contains no benchmark timing itself so it can also be used
/// for warm-up.
fn route_once(workload: &Workload) -> Measurement {
    let router = SabreRouter::with_heuristic(
        SabreHeuristic::Decay,
    );

    let started = Instant::now();

    let result = router
        .route(
            &workload.routing_workload,
            &workload.topology,
            &workload.mapping,
            &workload.config,
        )
        .unwrap_or_else(|error| {
            panic!(
                "SABRE benchmark case '{}' failed for {} qubits: {}",
                workload.name,
                workload.qubit_count,
                error
            );
        });

    let elapsed = started.elapsed();

    validate_result(workload, &result);

    let metrics = result.metrics;

    assert_eq!(
        metrics.logical_qubits,
        workload.qubit_count,
        "{} benchmark returned unexpected logical-qubit count",
        workload.name
    );

    assert_eq!(
        metrics.physical_qubits,
        workload.qubit_count,
        "{} benchmark returned unexpected physical-qubit count",
        workload.name
    );

    assert_eq!(
        metrics.original_two_qubit_operations,
        workload
            .routing_workload
            .interaction_count(),
        "{} benchmark must report all input interactions",
        workload.name
    );

    assert_eq!(
        metrics.inserted_moves,
        metrics.inserted_swaps
            + metrics.inserted_bridges
            + metrics.inserted_permutations,
        "{} benchmark returned inconsistent movement metrics",
        workload.name
    );

    assert!(
        metrics.final_operations >= metrics.original_operations,
        "{} benchmark cannot return fewer operations after routing",
        workload.name
    );

    assert_eq!(
        metrics.final_operations,
        result.operations.len(),
        "{} benchmark result metrics and operation stream disagree",
        workload.name
    );

    assert!(
        metrics.floating_point_values_are_finite(),
        "{} benchmark returned a non-finite floating-point metric",
        workload.name
    );

    assert!(
        result.verification.status.passed()
            || result.verification.status.not_requested(),
        "{} benchmark returned an invalid verification state",
        workload.name
    );

    result
        .layout
        .final_mapping
        .validate()
        .expect("SABRE final mapping must be valid");

    Measurement {
        elapsed,
        original_operations: metrics.original_operations,
        final_operations: metrics.final_operations,
        inserted_swaps: metrics.inserted_swaps,
        routing_iterations: metrics.routing_iterations,
        candidate_evaluations: metrics.candidate_evaluations,
        candidate_rejections: metrics.candidate_rejections,
        final_depth: metrics.final_depth,
        routing_depth: metrics.routing_depth,
    }
}

// =============================================================================
// Adaptive repetition count
// =============================================================================

/// Performs a small pilot measurement to avoid multiplying a very expensive
/// SABRE workload by the default repetition count.
///
/// The pilot result is not included in the reported measurement.
fn choose_measured_iterations(
    workload: &Workload,
) -> usize {
    let pilot = route_once(workload);

    if pilot.elapsed.is_zero() {
        return DEFAULT_MEASURED_ITERATIONS;
    }

    if pilot.elapsed >= TARGET_CASE_TIME {
        return MIN_MEASURED_ITERATIONS;
    }

    let target_nanos = TARGET_CASE_TIME.as_nanos();
    let pilot_nanos = pilot.elapsed.as_nanos();

    let estimated =
        target_nanos
            .checked_div(pilot_nanos)
            .unwrap_or(1);

    let estimated = estimated as usize;

    estimated
        .clamp(
            MIN_MEASURED_ITERATIONS,
            DEFAULT_MEASURED_ITERATIONS,
        )
}

// =============================================================================
// Measurement
// =============================================================================

/// Executes a workload for the requested number of measured iterations.
fn measure_workload(
    workload: &Workload,
    iterations: usize,
) -> CaseMeasurement {
    assert!(
        iterations >= MIN_MEASURED_ITERATIONS,
        "measured iteration count must not be below the benchmark minimum"
    );

    for _ in 0..WARMUP_ITERATIONS {
        let measurement = route_once(workload);

        black_box(measurement);
    }

    let mut total_elapsed = Duration::ZERO;
    let mut last = Measurement {
        elapsed: Duration::ZERO,
        original_operations: 0,
        final_operations: 0,
        inserted_swaps: 0,
        routing_iterations: 0,
        candidate_evaluations: 0,
        candidate_rejections: 0,
        final_depth: 0,
        routing_depth: 0,
    };

    for _ in 0..iterations {
        let measurement = route_once(workload);

        total_elapsed = total_elapsed
            .checked_add(measurement.elapsed)
            .expect("benchmark duration accumulator overflow");

        last = measurement;

        black_box(last);
    }

    CaseMeasurement {
        total_elapsed,
        iterations,
        last,
    }
}

// =============================================================================
// Benchmark output
// =============================================================================

/// Prints one stable human-readable benchmark record.
///
/// The exact timing numbers are intentionally not parsed by the routing
/// implementation. External tooling may parse the labels if required, but
/// machine-readable benchmark storage belongs to the future benchmarking
/// subsystem rather than this lightweight executable harness.
fn print_measurement(
    workload: &Workload,
    measurement: CaseMeasurement,
) {
    let average = measurement.average_duration();

    println!(
        concat!(
            "sabre",
            " case={}",
            " qubits={}",
            " iterations={}",
            " avg_us={:.3}",
            " original_ops={}",
            " final_ops={}",
            " swaps={}",
            " routing_iterations={}",
            " candidate_evaluations={}",
            " candidate_rejections={}",
            " final_depth={}",
            " routing_depth={}"
        ),
        workload.name,
        workload.qubit_count,
        measurement.iterations,
        measurement.average_micros(),
        measurement.last.original_operations,
        measurement.last.final_operations,
        measurement.last.inserted_swaps,
        measurement.last.routing_iterations,
        measurement.last.candidate_evaluations,
        measurement.last.candidate_rejections,
        measurement.last.final_depth,
        measurement.last.routing_depth,
    );

    println!(
        "  total_ms={:.3} last_invocation_us={:.3}",
        measurement.total_elapsed.as_secs_f64()
            * 1_000.0,
        measurement.last.elapsed.as_secs_f64()
            * 1_000_000.0,
    );

    assert!(
        average <= BENCHMARK_TIMEOUT,
        "average SABRE benchmark invocation exceeded the configured safety timeout"
    );
}

// =============================================================================
// Benchmark matrix
// =============================================================================

/// Runs all SABRE benchmark cases for one topology size.
fn benchmark_size(qubit_count: usize) {
    let workloads = [
        build_local_workload(qubit_count),
        build_nearest_neighbor_workload(qubit_count),
        build_nonlocal_workload(qubit_count),
        build_dense_nonlocal_workload(qubit_count),
        build_repeated_nonlocal_workload(qubit_count),
    ];

    for workload in workloads {
        validate_workload(&workload);

        let iterations =
            choose_measured_iterations(&workload);

        let measurement =
            measure_workload(&workload, iterations);

        print_measurement(
            &workload,
            measurement,
        );

        black_box(measurement);
    }
}

// =============================================================================
// Main
// =============================================================================

fn main() {
    println!("Zamani SABRE routing benchmark");
    println!(
        "algorithm_version={}",
        zamani_compiler::quantum::routing::algorithms::sabre::
            SABRE_ALGORITHM_VERSION
    );
    println!(
        "routing_version={}",
        zamani_compiler::quantum::routing::algorithms::sabre::
            SABRE_ROUTING_VERSION
    );
    println!(
        "seed={:#018x}",
        BENCHMARK_SEED
    );
    println!(
        "sabre_iterations={}",
        BENCHMARK_SABRE_ITERATIONS
    );
    println!(
        "sabre_trials={}",
        BENCHMARK_SABRE_TRIALS
    );
    println!(
        "lookahead_depth={}",
        BENCHMARK_LOOKAHEAD_DEPTH
    );
    println!(
        "candidate_limit={}",
        BENCHMARK_CANDIDATE_LIMIT
    );
    println!(
        "verification=standard"
    );
    println!();

    for qubit_count in [
        8usize,
        16usize,
        32usize,
        64usize,
        128usize,
    ] {
        println!(
            "=== SABRE {} qubits ===",
            qubit_count
        );

        benchmark_size(qubit_count);

        println!();
    }

    black_box((
        MIN_QUBITS,
        MAX_QUBITS,
        MAX_OPERATIONS,
        MAX_INTERACTIONS,
    ));
}