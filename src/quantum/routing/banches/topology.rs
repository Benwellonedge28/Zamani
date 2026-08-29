//! Zamani Quantum Routing — Topology Benchmarks
//!
//! `src/quantum/routing/benches/topology.rs`
//!
//! Production benchmark harness for `quantum::routing::topology`.
//!
//! # Purpose
//!
//! This benchmark measures the performance and scaling characteristics of the
//! canonical physical-topology implementation used by Zamani's quantum
//! routing subsystem.
//!
//! The benchmark intentionally measures topology operations independently from
//! routing algorithms. It therefore remains valid as the routing subsystem
//! grows to include:
//!
//! - shortest-path routing;
//! - lookahead routing;
//! - SABRE / LightSABRE;
//! - noise-aware routing;
//! - dynamic routing;
//! - distributed routing;
//! - hardware-aware cost models;
//! - layout selection;
//! - routing verification.
//!
//! # What is measured
//!
//! The benchmark covers:
//!
//! 1. topology construction;
//! 2. topology validation;
//! 3. connectivity checks;
//! 4. connected-component analysis;
//! 5. directed neighbor queries;
//! 6. undirected neighbor queries;
//! 7. degree calculation;
//! 8. incoming degree calculation;
//! 9. outgoing degree calculation;
//! 10. adjacency queries;
//! 11. the built-in Heavy-Hex topology;
//! 12. scaling across increasingly large linear topologies.
//!
//! # Why a custom benchmark harness?
//!
//! Zamani targets Rust 1.97.1 stable.
//!
//! Rust's built-in `#[bench]` interface remains nightly-only, so this file does
//! not use `#[bench]` or the unstable `test` crate.
//!
//! Instead it uses:
//!
//! - `std::time::Instant`;
//! - `std::hint::black_box`;
//! - deterministic warm-up;
//! - configurable samples;
//! - configurable iterations;
//! - median and percentile statistics;
//! - explicit scaling cases.
//!
//! Cargo supports this through:
//!
//! ```toml
//! [[bench]]
//! name = "topology"
//! harness = false
//! ```
//!
//! The benchmark therefore remains fully compatible with stable Rust 1.97.1.
//!
//! # Architectural boundary
//!
//! ```text
//!                  topology.rs
//!                       │
//!                       ▼
//!          ┌──────────────────────────┐
//!          │ PhysicalTopology        │
//!          └────────────┬─────────────┘
//!                       │
//!          ┌────────────┼────────────┐
//!          ▼            ▼            ▼
//!      validation    graph queries   metadata
//!          │            │
//!          └────────────┼────────────┘
//!                       ▼
//!              topology benchmark
//! ```
//!
//! This benchmark deliberately does NOT benchmark:
//!
//! - compiler IR;
//! - transpilation;
//! - gate decomposition;
//! - scheduling;
//! - simulation;
//! - hardware execution;
//! - provider SDKs;
//! - network access;
//! - QEC;
//! - benchmark execution backends.
//!
//! Those belong to other benchmark targets.
//!
//! # Reproducibility
//!
//! Benchmark inputs are deterministic.
//!
//! No random number generator is used.
//!
//! The benchmark does not depend on:
//!
//! - wall-clock time for input generation;
//! - system topology;
//! - external hardware;
//! - network services;
//! - environment-specific device data.
//!
//! Timing itself is, of course, machine dependent.
//!
//! # Configuration
//!
//! Optional environment variables:
//!
//! `ZAMANI_BENCH_SAMPLES`
//!
//! Number of measured samples per benchmark.
//! Default: `15`.
//!
//! `ZAMANI_BENCH_WARMUP`
//!
//! Number of warm-up samples.
//! Default: `3`.
//!
//! `ZAMANI_BENCH_ITERATIONS`
//!
//! Override the default operation batch size.
//! This is useful for CI or controlled performance experiments.
//!
//! `ZAMANI_BENCH_MAX_SIZE`
//!
//! Maximum linear-topology size.
//! Default: `4096`.
//!
//! `ZAMANI_BENCH_FORMAT`
//!
//! Either:
//!
//! - `table` — human-readable output;
//! - `csv` — machine-readable output.
//!
//! Default: `table`.
//!
//! # Example
//!
//! ```text
//! cargo bench --bench topology
//! ```
//!
//! Or:
//!
//! ```text
//! ZAMANI_BENCH_SAMPLES=30 cargo bench --bench topology
//! ```
//!
//! CSV:
//!
//! ```text
//! ZAMANI_BENCH_FORMAT=csv cargo bench --bench topology
//! ```
//!
//! # Safety
//!
//! This benchmark contains no unsafe code.
//!
//! The crate-level lint is intentionally strict so an accidental unsafe block
//! cannot be introduced here.
//!
//! # Integration contract
//!
//! This file consumes only the public routing topology API:
//!
//! `crate::quantum::routing::topology::PhysicalTopology`
//!
//! and:
//!
//! `crate::quantum::routing::types::PhysicalQubitId`
//!
//! It does not access topology internals.
//!
//! Consequently, later changes to the internal representation of
//! `PhysicalTopology` do not require changes here as long as its public
//! topology contract remains stable.
//!
//! The benchmark is therefore a consumer of the same API that production
//! routing algorithms use.
//!
//! # Definition of done
//!
//! This file is complete when:
//!
//! - it builds on stable Rust 1.97.1;
//! - it contains no unsafe code;
//! - it does not require nightly features;
//! - it does not require external benchmark dependencies;
//! - every measured operation consumes the public topology API;
//! - benchmark inputs are deterministic;
//! - optimized-away work is guarded with `black_box`;
//! - warm-up and measured samples are separated;
//! - statistics are robust against a small number of timing outliers;
//! - topology sizes exercise small through large routing-device graphs;
//! - results can be emitted as human-readable text or CSV;
//! - adding routing algorithms does not require changing this file.

// =============================================================================
// Strictness
// =============================================================================

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(unused_variables)]

// =============================================================================
// Imports
// =============================================================================

use std::env;
use std::hint::black_box;
use std::time::{Duration, Instant};

use zamani_compiler::quantum::routing::topology::PhysicalTopology;
use zamani_compiler::quantum::routing::types::PhysicalQubitId;

// =============================================================================
// Configuration
// =============================================================================

/// Runtime configuration for the benchmark harness.
#[derive(Debug, Clone, Copy)]
struct BenchmarkConfig {
    /// Number of warm-up samples.
    warmup_samples: usize,

    /// Number of measured samples.
    measured_samples: usize,

    /// Optional global iteration override.
    iteration_override: Option<usize>,

    /// Maximum topology size used by scaling benchmarks.
    max_size: usize,

    /// Output format.
    format: OutputFormat,
}

/// Supported benchmark output formats.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OutputFormat {
    /// Human-readable benchmark report.
    Table,

    /// Machine-readable CSV.
    Csv,
}

impl BenchmarkConfig {
    /// Loads benchmark configuration from the environment.
    ///
    /// Invalid environment values are rejected with a clear diagnostic rather
    /// than silently changing benchmark semantics.
    fn from_environment() -> Result<Self, String> {
        let warmup_samples =
            parse_positive_env("ZAMANI_BENCH_WARMUP", 3)?;

        let measured_samples =
            parse_positive_env("ZAMANI_BENCH_SAMPLES", 15)?;

        let iteration_override =
            parse_optional_positive_env("ZAMANI_BENCH_ITERATIONS")?;

        let max_size =
            parse_positive_env("ZAMANI_BENCH_MAX_SIZE", 4096)?;

        let format = match env::var("ZAMANI_BENCH_FORMAT")
            .unwrap_or_else(|_| "table".to_string())
            .to_ascii_lowercase()
            .as_str()
        {
            "table" => OutputFormat::Table,
            "csv" => OutputFormat::Csv,
            value => {
                return Err(format!(
                    "invalid ZAMANI_BENCH_FORMAT '{value}'; expected 'table' or 'csv'"
                ));
            }
        };

        Ok(Self {
            warmup_samples,
            measured_samples,
            iteration_override,
            max_size,
            format,
        })
    }
}

// =============================================================================
// Benchmark case
// =============================================================================

/// A single benchmark case.
///
/// The function is called `iterations` times inside one measured sample.
struct BenchmarkCase {
    /// Stable machine-readable benchmark identifier.
    name: &'static str,

    /// Topology size associated with the case.
    topology_size: usize,

    /// Number of operation repetitions per sample.
    iterations: usize,

    /// Benchmark operation.
    operation: Box<dyn FnMut() -> u64>,
}

// =============================================================================
// Benchmark result
// =============================================================================

/// Statistical result for one benchmark case.
#[derive(Debug, Clone)]
struct BenchmarkResult {
    /// Stable benchmark name.
    name: &'static str,

    /// Topology size.
    topology_size: usize,

    /// Operations executed in each sample.
    iterations: usize,

    /// Number of measured samples.
    samples: usize,

    /// Minimum sample duration.
    min: Duration,

    /// Median sample duration.
    median: Duration,

    /// 95th percentile sample duration.
    p95: Duration,

    /// Maximum sample duration.
    max: Duration,

    /// Median nanoseconds per operation.
    median_ns_per_operation: f64,
}

// =============================================================================
// Main
// =============================================================================

fn main() {
    if let Err(error) = run() {
        eprintln!("Zamani topology benchmark failed: {error}");
        std::process::exit(1);
    }
}

/// Runs the complete topology benchmark suite.
fn run() -> Result<(), String> {
    let config = BenchmarkConfig::from_environment()?;

    validate_configuration(&config)?;

    let cases = build_cases(&config)?;

    if cases.is_empty() {
        return Err(
            "benchmark configuration produced no benchmark cases"
                .to_string(),
        );
    }

    let mut results = Vec::with_capacity(cases.len());

    for case in cases {
        let result = run_case(&config, case)?;
        results.push(result);
    }

    match config.format {
        OutputFormat::Table => print_table(&results, &config),
        OutputFormat::Csv => print_csv(&results),
    }

    Ok(())
}

// =============================================================================
// Configuration validation
// =============================================================================

fn validate_configuration(
    config: &BenchmarkConfig,
) -> Result<(), String> {
    if config.warmup_samples == 0 {
        return Err(
            "warmup sample count must be greater than zero"
                .to_string(),
        );
    }

    if config.measured_samples == 0 {
        return Err(
            "measured sample count must be greater than zero"
                .to_string(),
        );
    }

    if config.max_size < 2 {
        return Err(
            "ZAMANI_BENCH_MAX_SIZE must be at least 2"
                .to_string(),
        );
    }

    if let Some(iterations) = config.iteration_override {
        if iterations == 0 {
            return Err(
                "ZAMANI_BENCH_ITERATIONS must be greater than zero"
                    .to_string(),
            );
        }
    }

    Ok(())
}

fn parse_positive_env(
    name: &str,
    default: usize,
) -> Result<usize, String> {
    match env::var(name) {
        Ok(value) => value.parse::<usize>().map_err(|_| {
            format!(
                "environment variable {name} must be a positive integer; got '{value}'"
            )
        }).and_then(|parsed| {
            if parsed == 0 {
                Err(format!(
                    "environment variable {name} must be greater than zero"
                ))
            } else {
                Ok(parsed)
            }
        }),

        Err(_) => Ok(default),
    }
}

fn parse_optional_positive_env(
    name: &str,
) -> Result<Option<usize>, String> {
    match env::var(name) {
        Ok(value) => {
            let parsed = value.parse::<usize>().map_err(|_| {
                format!(
                    "environment variable {name} must be a positive integer; got '{value}'"
                )
            })?;

            if parsed == 0 {
                return Err(format!(
                    "environment variable {name} must be greater than zero"
                ));
            }

            Ok(Some(parsed))
        }

        Err(_) => Ok(None),
    }
}

// =============================================================================
// Case construction
// =============================================================================

/// Builds deterministic benchmark cases.
///
/// The sizes intentionally cover:
///
/// - tiny compiler/device topologies;
/// - normal NISQ-scale devices;
/// - hundreds of qubits;
/// - thousand-qubit-scale graphs;
/// - several-thousand-qubit stress cases.
///
/// The final size is always bounded by the configured maximum.
fn build_cases(
    config: &BenchmarkConfig,
) -> Result<Vec<BenchmarkCase>, String> {
    let mut sizes = vec![
        4usize,
        16usize,
        64usize,
        256usize,
        1024usize,
        4096usize,
    ];

    sizes.retain(|size| *size <= config.max_size);

    if sizes.is_empty() {
        sizes.push(config.max_size);
    }

    let mut cases = Vec::new();

    // -------------------------------------------------------------------------
    // Construction
    // -------------------------------------------------------------------------

    for size in sizes.iter().copied() {
        let iterations = config
            .iteration_override
            .unwrap_or_else(|| construction_iterations(size));

        cases.push(BenchmarkCase {
            name: "topology.construct.line",
            topology_size: size,
            iterations,
            operation: Box::new(move || {
                let topology = PhysicalTopology::line(size)
                    .expect("benchmark topology construction must succeed");

                black_box(topology.qubit_count() as u64)
            }),
        });
    }

    // -------------------------------------------------------------------------
    // Validation
    // -------------------------------------------------------------------------

    for size in sizes.iter().copied() {
        let topology = PhysicalTopology::line(size)
            .map_err(|error| {
                format!(
                    "failed to prepare validation topology of size {size}: {error}"
                )
            })?;

        let iterations = config
            .iteration_override
            .unwrap_or_else(|| validation_iterations(size));

        cases.push(BenchmarkCase {
            name: "topology.validate.line",
            topology_size: size,
            iterations,
            operation: Box::new(move || {
                let result = topology.validate();

                black_box(result.is_ok() as u64)
            }),
        });
    }

    // -------------------------------------------------------------------------
    // Structural queries
    // -------------------------------------------------------------------------

    for size in sizes.iter().copied() {
        let topology = PhysicalTopology::line(size)
            .map_err(|error| {
                format!(
                    "failed to prepare query topology of size {size}: {error}"
                )
            })?;

        let first = PhysicalQubitId::new(0);
        let middle =
            PhysicalQubitId::new(size.saturating_sub(1) / 2);
        let last =
            PhysicalQubitId::new(size.saturating_sub(1));

        let iterations = config
            .iteration_override
            .unwrap_or_else(|| query_iterations(size));

        {
            let topology = topology.clone();

            cases.push(BenchmarkCase {
                name: "topology.is_connected",
                topology_size: size,
                iterations,
                operation: Box::new(move || {
                    black_box(topology.is_connected() as u64)
                }),
            });
        }

        {
            let topology = topology.clone();

            cases.push(BenchmarkCase {
                name: "topology.connected_components",
                topology_size: size,
                iterations: component_iterations(size, config),
                operation: Box::new(move || {
                    let components =
                        topology.connected_components();

                    black_box(components.len() as u64)
                }),
            });
        }

        {
            let topology = topology.clone();

            cases.push(BenchmarkCase {
                name: "topology.neighbors.middle",
                topology_size: size,
                iterations,
                operation: Box::new(move || {
                    let neighbors =
                        topology.neighbors(middle);

                    black_box(neighbors.len() as u64)
                }),
            });
        }

        {
            let topology = topology.clone();

            cases.push(BenchmarkCase {
                name: "topology.undirected_neighbors.middle",
                topology_size: size,
                iterations,
                operation: Box::new(move || {
                    let neighbors =
                        topology.undirected_neighbors(middle);

                    black_box(neighbors.len() as u64)
                }),
            });
        }

        {
            let topology = topology.clone();

            cases.push(BenchmarkCase {
                name: "topology.degree.middle",
                topology_size: size,
                iterations,
                operation: Box::new(move || {
                    black_box(topology.degree(middle) as u64)
                }),
            });
        }

        {
            let topology = topology.clone();

            cases.push(BenchmarkCase {
                name: "topology.outgoing_degree.middle",
                topology_size: size,
                iterations,
                operation: Box::new(move || {
                    black_box(
                        topology.outgoing_degree(middle) as u64,
                    )
                }),
            });
        }

        {
            let topology = topology.clone();

            cases.push(BenchmarkCase {
                name: "topology.incoming_degree.middle",
                topology_size: size,
                iterations,
                operation: Box::new(move || {
                    black_box(
                        topology.incoming_degree(middle) as u64,
                    )
                }),
            });
        }

        {
            let topology = topology.clone();

            cases.push(BenchmarkCase {
                name: "topology.is_adjacent.local",
                topology_size: size,
                iterations,
                operation: Box::new(move || {
                    black_box(
                        topology.is_adjacent(first, middle)
                            as u64,
                    )
                }),
            });
        }

        {
            let topology = topology.clone();

            cases.push(BenchmarkCase {
                name: "topology.is_adjacent.nearby",
                topology_size: size,
                iterations,
                operation: Box::new(move || {
                    let adjacent_to_middle =
                        if size > 1 {
                            PhysicalQubitId::new(
                                (size - 1) / 2 + 1,
                            )
                        } else {
                            middle
                        };

                    black_box(
                        topology.is_adjacent(
                            middle,
                            adjacent_to_middle,
                        ) as u64,
                    )
                }),
            });
        }

        {
            let topology = topology.clone();

            cases.push(BenchmarkCase {
                name: "topology.is_adjacent.distant",
                topology_size: size,
                iterations,
                operation: Box::new(move || {
                    black_box(
                        topology.is_adjacent(first, last)
                            as u64,
                    )
                }),
            });
        }
    }

    // -------------------------------------------------------------------------
    // Heavy-Hex regression benchmark
    // -------------------------------------------------------------------------

    cases.push(BenchmarkCase {
        name: "topology.construct.heavy_hex",
        topology_size: 6,
        iterations: config
            .iteration_override
            .unwrap_or(10_000),
        operation: Box::new(|| {
            let topology = PhysicalTopology::heavy_hex();

            black_box(topology.is_connected() as u64)
        }),
    });

    cases.push(BenchmarkCase {
        name: "topology.validate.heavy_hex",
        topology_size: 6,
        iterations: config
            .iteration_override
            .unwrap_or(10_000),
        operation: Box::new(|| {
            let topology = PhysicalTopology::heavy_hex();

            black_box(topology.validate().is_ok() as u64)
        }),
    });

    Ok(cases)
}

// =============================================================================
// Iteration policy
// =============================================================================

/// Construction is intentionally given fewer repetitions as topology size
/// increases because construction allocates and populates the complete graph.
fn construction_iterations(size: usize) -> usize {
    match size {
        0..=16 => 1_000,
        17..=256 => 250,
        257..=1_024 => 50,
        1_025..=4_096 => 10,
        _ => 3,
    }
}

/// Validation walks the entire topology and therefore scales with topology
/// size.
fn validation_iterations(size: usize) -> usize {
    match size {
        0..=16 => 1_000,
        17..=256 => 250,
        257..=1_024 => 50,
        1_025..=4_096 => 10,
        _ => 3,
    }
}

/// Local graph queries are cheap and can use larger batches.
fn query_iterations(size: usize) -> usize {
    match size {
        0..=256 => 10_000,
        257..=1_024 => 5_000,
        1_025..=4_096 => 2_000,
        _ => 1_000,
    }
}

/// Connected-component analysis traverses the complete graph and allocates
/// component vectors, so it gets a smaller batch at larger sizes.
fn component_iterations(
    size: usize,
    config: &BenchmarkConfig,
) -> usize {
    config
        .iteration_override
        .unwrap_or_else(|| match size {
            0..=16 => 1_000,
            17..=256 => 250,
            257..=1_024 => 50,
            1_025..=4_096 => 10,
            _ => 3,
        })
}

// =============================================================================
// Benchmark execution
// =============================================================================

fn run_case(
    config: &BenchmarkConfig,
    mut case: BenchmarkCase,
) -> Result<BenchmarkResult, String> {
    // Warm-up is intentionally excluded from reported measurements.
    for _ in 0..config.warmup_samples {
        let mut accumulator = 0u64;

        for _ in 0..case.iterations {
            accumulator =
                accumulator.wrapping_add((case.operation)());
        }

        black_box(accumulator);
    }

    let mut samples =
        Vec::with_capacity(config.measured_samples);

    for _ in 0..config.measured_samples {
        let start = Instant::now();

        let mut accumulator = 0u64;

        for _ in 0..case.iterations {
            accumulator =
                accumulator.wrapping_add((case.operation)());
        }

        let elapsed = start.elapsed();

        // Prevent the optimizer from proving the measured work irrelevant.
        black_box(accumulator);

        samples.push(elapsed);
    }

    if samples.is_empty() {
        return Err(format!(
            "benchmark '{}' produced no samples",
            case.name
        ));
    }

    samples.sort_unstable();

    let min = samples[0];
    let max = samples[samples.len() - 1];
    let median = percentile(&samples, 0.50);
    let p95 = percentile(&samples, 0.95);

    let median_ns =
        median.as_secs_f64() * 1_000_000_000.0;

    let median_ns_per_operation =
        median_ns / case.iterations as f64;

    Ok(BenchmarkResult {
        name: case.name,
        topology_size: case.topology_size,
        iterations: case.iterations,
        samples: samples.len(),
        min,
        median,
        p95,
        max,
        median_ns_per_operation,
    })
}

// =============================================================================
// Statistics
// =============================================================================

/// Returns a nearest-rank percentile.
///
/// The benchmark deliberately avoids floating-point interpolation between
/// samples. The returned value is therefore always one of the actual observed
/// sample durations.
fn percentile(
    sorted_samples: &[Duration],
    percentile: f64,
) -> Duration {
    debug_assert!(!sorted_samples.is_empty());
    debug_assert!((0.0..=1.0).contains(&percentile));

    let last = sorted_samples.len() - 1;

    let index =
        ((last as f64) * percentile).round() as usize;

    sorted_samples[index.min(last)]
}

// =============================================================================
// Output
// =============================================================================

fn print_table(
    results: &[BenchmarkResult],
    config: &BenchmarkConfig,
) {
    println!();
    println!("Zamani Quantum Routing — Topology Benchmarks");
    println!("Rust stable / custom benchmark harness");
    println!();
    println!(
        "warmup={} samples={} max_size={}",
        config.warmup_samples,
        config.measured_samples,
        config.max_size
    );
    println!();

    println!(
        "{:<42} {:>8} {:>10} {:>12} {:>12} {:>12} {:>12}",
        "benchmark",
        "qubits",
        "iterations",
        "median/op",
        "min",
        "p95",
        "max"
    );

    println!(
        "{}",
        "-".repeat(112)
    );

    for result in results {
        println!(
            "{:<42} {:>8} {:>10} {:>10.2} ns {:>10.2} ns {:>10.2} ns {:>10.2} ns",
            result.name,
            result.topology_size,
            result.iterations,
            result.median_ns_per_operation,
            duration_ns(result.min),
            duration_ns(result.p95),
            duration_ns(result.max),
        );
    }

    println!();
    println!(
        "Measured sample median is reported per operation; min/p95/max are per sample."
    );
    println!(
        "Use repeated runs on the same machine for regression comparisons."
    );
}

fn print_csv(results: &[BenchmarkResult]) {
    println!(
        "benchmark,topology_size,iterations,samples,min_ns,median_ns,p95_ns,max_ns,median_ns_per_operation"
    );

    for result in results {
        println!(
            "{},{},{},{},{:.3},{:.3},{:.3},{:.3},{:.6}",
            result.name,
            result.topology_size,
            result.iterations,
            result.samples,
            duration_ns(result.min),
            duration_ns(result.median),
            duration_ns(result.p95),
            duration_ns(result.max),
            result.median_ns_per_operation,
        );
    }
}

fn duration_ns(duration: Duration) -> f64 {
    duration.as_secs_f64() * 1_000_000_000.0
}