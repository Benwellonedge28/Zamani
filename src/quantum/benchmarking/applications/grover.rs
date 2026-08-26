//! Zamani Quantum Benchmarking — Grover Search Application Benchmark.
//!
//! # Purpose
//!
//! Production application-level benchmark generator and mathematical analyzer
//! for Grover's quantum search algorithm.
//!
//! This file owns:
//!
//! - the benchmark identity;
//! - the Grover benchmark problem definition;
//! - marked-state validation;
//! - deterministic marked-state generation;
//! - exact Grover iteration calculation;
//! - ideal success-probability calculation;
//! - canonical Grover circuit construction;
//! - bounded multi-controlled phase-oracle construction;
//! - bounded diffusion construction;
//! - application workload construction;
//! - application metadata;
//! - canonical measurement-count analysis;
//! - result-quality metrics specific to Grover;
//! - deterministic resource accounting;
//! - unit tests for all of the above.
//!
//! This file deliberately does NOT own:
//!
//! - backend selection;
//! - backend execution;
//! - transpilation;
//! - routing;
//! - physical topology;
//! - scheduling;
//! - calibration;
//! - hardware communication;
//! - simulator implementation;
//! - vendor SDKs;
//! - error correction;
//! - persistence;
//! - report serialization;
//! - universal benchmark-result storage.
//!
//! Those responsibilities remain in the surrounding benchmarking,
//! compiler, runtime, hardware, QEC, and reporting subsystems.
//!
//! # Architectural position
//!
//! ```text
//! ApplicationGenerationRequest
//!             │
//!             ▼
//!     GroverBenchmarkGenerator
//!             │
//!       ┌─────┴─────┐
//!       ▼           ▼
//!   validation   generation
//!       │           │
//!       │           ▼
//!       │     QuantumCircuit
//!       │           │
//!       │           ▼
//!       │     ApplicationWorkload
//!       │           │
//!       └─────┬─────┘
//!             ▼
//!       BenchmarkExperiment
//!             │
//!             ▼
//!       BenchmarkExecutor
//!             │
//!             ▼
//!       normalized counts
//!             │
//!             ▼
//!       Grover analysis
//!             │
//!             ▼
//!       GroverBenchmarkResult
//! ```
//!
//! # Relationship with the existing Zamani architecture
//!
//! The canonical dependency direction is:
//!
//! ```text
//! benchmarking::applications::grover
//!             │
//!             ├────────► benchmarking::generators::application
//!             │
//!             ├────────► benchmarking::core::workload
//!             │
//!             ├────────► benchmarking::core::limits
//!             │
//!             └────────► quantum::ir
//! ```
//!
//! The direction must never be reversed:
//!
//! ```text
//! quantum::ir ─X─► benchmarking
//! ```
//!
//! The benchmark consumes Quantum IR; it does not redefine it.
//!
//! # Scientific benchmark semantics
//!
//! Grover searches an unstructured space of `N` states containing `M` marked
//! states.
//!
//! For a complete n-qubit computational basis:
//!
//! ```text
//! N = 2^n
//! ```
//!
//! The initial marked-state amplitude is:
//!
//! ```text
//! sin(theta) = sqrt(M / N)
//! ```
//!
//! After `k` Grover iterations, the ideal success probability is:
//!
//! ```text
//! P_success(k) = sin^2((2k + 1) * theta)
//! ```
//!
//! The exact near-optimal iteration count is selected by evaluating the
//! mathematical angle rather than using an unconditional approximation.
//!
//! In particular:
//!
//! ```text
//! k = floor(pi / (4 * theta) - 1/2)
//! ```
//!
//! where:
//!
//! ```text
//! theta = asin(sqrt(M / N))
//! ```
//!
//! The implementation also evaluates the neighboring integer so that the
//! returned iteration count is the best integer under the exact ideal
//! success-probability model.
//!
//! This is important because the commonly quoted:
//!
//! ```text
//! floor(pi/4 * sqrt(N/M))
//! ```
//!
//! is an asymptotic approximation, not the exact finite-size optimum.
//!
//! # Benchmark definition
//!
//! The canonical Zamani Grover benchmark uses:
//!
//! - an n-qubit search register;
//! - one or more marked computational-basis states;
//! - a phase oracle;
//! - the standard Grover diffusion operator;
//! - a configurable iteration count;
//! - computational-basis measurement of the search register.
//!
//! By default, one marked state is selected deterministically from the
//! generation seed and sequence index. This gives the benchmark the same
//! essential structure used by application-oriented Grover benchmark suites,
//! while guaranteeing reproducibility.
//!
//! Explicit `marked_state` and `marked_states` parameters are supported for
//! deterministic benchmark fixtures and controlled experiments.
//!
//! # Bit-string convention
//!
//! Zamani's benchmark representation uses:
//!
//! ```text
//! bitstring[0] == logical qubit q0 == classical bit c0
//! ```
//!
//! Therefore a marked state such as:
//!
//! ```text
//! 0101
//! ```
//!
//! means:
//!
//! ```text
//! q0 = 0
//! q1 = 1
//! q2 = 0
//! q3 = 1
//! ```
//!
//! Backend-specific bit-order conventions must be normalized by the execution
//! layer before `analyze_counts()` is called.
//!
//! This file must never silently reverse backend bitstrings.
//!
//! # Oracle construction
//!
//! The canonical phase oracle applies a phase of -1 to every marked basis
//! state.
//!
//! For one marked state `|x>`:
//!
//! ```text
//! X on every zero bit
//! MCZ on all search qubits
//! X on every zero bit
//! ```
//!
//! The multi-controlled phase operation is represented in canonical IR using:
//!
//! - Z for one qubit;
//! - CZ for two qubits;
//! - a CCX-based ancilla ladder for three or more qubits.
//!
//! Ancillas are initialized to |0> implicitly by Quantum IR's circuit-start
//! convention and are fully uncomputed after each multi-controlled phase.
//!
//! This gives a backend-independent logical circuit while allowing later
//! compiler/transpiler layers to choose hardware-specific decompositions.
//!
//! # Important benchmarking distinction
//!
//! The benchmark's logical circuit is not the same thing as the final physical
//! circuit executed by a backend.
//!
//! Therefore this file reports logical resources only:
//!
//! - search qubits;
//! - oracle ancillas;
//! - total logical qubits;
//! - logical gates;
//! - logical two-qubit gates;
//! - Grover iterations;
//! - oracle applications.
//!
//! Physical depth, routed two-qubit gates, native-gate counts, queue time,
//! execution time, calibration information, and physical fidelity belong to
//! downstream benchmarking layers.
//!
//! # Result analysis
//!
//! `analyze_counts()` consumes normalized computational-basis counts.
//!
//! It returns:
//!
//! - observed success probability;
//! - ideal success probability;
//! - absolute success-probability error;
//! - relative success-probability error;
//! - classical random-guess baseline;
//! - quantum/classical success ratio;
//! - marked-state counts;
//! - total shots;
//! - iteration count;
//! - oracle-query count.
//!
//! This is intentionally independent of `core::observation` so that the file
//! can be completed and tested before the universal observation model is
//! finalized.
//!
//! The future observation layer can convert its canonical counts into the
//! `BTreeMap<String, u64>` expected by `analyze_counts()` without changing this
//! file's mathematical contract.
//!
//! # Reproducibility
//!
//! Identical:
//!
//! ```text
//! application_id
//! instance_id
//! problem_size
//! parameters
//! seed
//! sequence_index
//! generator_revision
//! ```
//!
//! produce the same semantic Grover workload.
//!
//! No system clock, global RNG, process identifier, pointer address, thread
//! identifier, filesystem state, or network state is consulted.
//!
//! # Resource safety
//!
//! Grover is potentially expensive because its iteration count grows as:
//!
//! ```text
//! O(sqrt(N / M))
//! ```
//!
//! and the logical oracle/diffuser themselves become increasingly expensive.
//!
//! Consequently the generator:
//!
//! - bounds the number of search qubits;
//! - bounds the number of marked states;
//! - checks the exact iteration count;
//! - estimates gate resources before circuit allocation;
//! - checks those resources against `BenchmarkLimits`;
//! - uses checked arithmetic throughout;
//! - rejects impossible marked states;
//! - rejects duplicate marked states;
//! - rejects malformed textual parameters;
//! - never allocates an exponential truth table;
//! - never enumerates the complete search space;
//! - never executes caller-provided code.
//!
//! The exponential search space therefore remains mathematical rather than
//! becoming an accidental memory allocation.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021
//!
//! No nightly features are required.
//!
//! # Integration contract
//!
//! This file integrates with the existing contracts:
//!
//! ```text
//! crate::quantum::benchmarking::generators::application
//!     ├── ApplicationBenchmarkGenerator
//!     ├── ApplicationGenerationRequest
//!     ├── ApplicationGeneratorCapability
//!     └── ApplicationGeneratorDescriptor
//!
//! crate::quantum::benchmarking::core::workload
//!     ├── ApplicationParameter
//!     ├── ApplicationWorkload
//!     ├── CircuitWorkload
//!     ├── WorkloadError
//!     └── WorkloadId
//!
//! crate::quantum::benchmarking::core::limits
//!     └── BenchmarkLimits
//!
//! crate::quantum::ir
//!     ├── QuantumCircuit
//!     ├── Gate
//!     ├── GateKind
//!     ├── Measurement
//!     ├── QubitId
//!     └── ClassicalBitId
//! ```
//!
//! No modification to those contracts is required for this implementation.
//!
//! The only namespace integration required is:
//!
//! ```text
//! src/quantum/benchmarking/applications/mod.rs
//!
//! pub mod grover;
//! ```
//!
//! If `applications/mod.rs` already contains the declaration, no other module
//! change is required for this file.
//!
//! # References
//!
//! The benchmark's mathematical model follows the standard Grover/amplitude
//! amplification formulation and the exact success-probability analysis of
//! Grover search. The application-oriented benchmark role follows the QED-C
//! methodology in which Grover's Search is treated as a variable-size
//! application benchmark and result quality is evaluated against the ideal
//! output.
//!
//! See:
//!
//! - Grover, L. K., "A fast quantum mechanical algorithm for database search".
//! - Boyer, Brassard, Høyer, and Tapp, "Tight bounds on quantum searching".
//! - QED-C Application-Oriented Performance Benchmarks for Quantum Computing.
//!
//! No network access is performed by this module.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use super::super::core::errors::{BenchmarkError, BenchmarkResult};
use super::super::core::limits::BenchmarkLimits;
use super::super::core::workload::{
    ApplicationParameter,
    ApplicationWorkload,
    CircuitWorkload,
    WorkloadError,
    WorkloadId,
};
use super::super::generators::application::{
    ApplicationBenchmarkGenerator,
    ApplicationGeneratorCapability,
    ApplicationGeneratorDescriptor,
    ApplicationGenerationRequest,
};

use crate::quantum::ir::{
    gate::{Gate, GateKind},
    measurement::{ClassicalBitId, Measurement},
    qubit::QubitId,
    QuantumCircuit,
};

// =============================================================================
// Stable identity and version
// =============================================================================

/// Stable benchmark identifier.
pub const GROVER_BENCHMARK_ID: &str = "grover";

/// Stable application identifier.
pub const GROVER_APPLICATION_ID: &str = "grover";

/// Stable generator version.
pub const GROVER_GENERATOR_VERSION: &str = "1.0.0";

/// Stable generator revision used in reproducibility metadata.
pub const GROVER_GENERATOR_REVISION: u32 = 1;

/// Human-readable benchmark name.
pub const GROVER_NAME: &str = "Grover Search";

/// Current result-contract version.
pub const GROVER_RESULT_SCHEMA_VERSION: u16 = 1;

/// Maximum number of search qubits represented by the `u64` marked-state
/// encoding used by this benchmark.
///
/// This is a semantic representation bound, not a statement that hardware can
/// execute 63-qubit Grover circuits.
pub const MAX_GROVER_SEARCH_QUBITS: usize = 63;

/// Maximum number of explicitly marked states accepted by one benchmark
/// instance.
///
/// This prevents a textual parameter from becoming a large in-memory oracle
/// definition while still supporting multi-solution Grover experiments.
pub const MAX_MARKED_STATES: usize = 256;

/// Maximum textual size of the marked-state parameter.
pub const MAX_MARKED_STATES_PARAMETER_BYTES: usize = 4096;

/// Maximum number of generated Grover iterations before resource estimation
/// is rejected.
///
/// The actual gate/resource limits are still checked against `BenchmarkLimits`.
pub const MAX_GROVER_ITERATIONS: u64 = 1_000_000;

// =============================================================================
// Mathematical configuration
// =============================================================================

/// Which iteration count the benchmark should use.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GroverIterationPolicy {
    /// Select the integer that maximizes the exact ideal success probability.
    Optimal,

    /// Execute an explicit number of Grover iterations.
    Explicit(u64),
}

impl Default for GroverIterationPolicy {
    fn default() -> Self {
        Self::Optimal
    }
}

impl GroverIterationPolicy {
    /// Returns a stable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Optimal => "optimal",
            Self::Explicit(_) => "explicit",
        }
    }
}

/// Typed Grover problem definition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GroverProblem {
    /// Number of search qubits.
    pub search_qubits: usize,

    /// Marked computational-basis state indexes.
    pub marked_states: Vec<u64>,
}

impl GroverProblem {
    /// Creates and validates a Grover problem.
    pub fn new(
        search_qubits: usize,
        marked_states: Vec<u64>,
    ) -> BenchmarkResult<Self> {
        if search_qubits == 0 {
            return Err(invalid_configuration(
                "problem_size",
                "Grover requires at least one search qubit",
            ));
        }

        if search_qubits > MAX_GROVER_SEARCH_QUBITS {
            return Err(invalid_configuration(
                "problem_size",
                "Grover benchmark supports at most 63 search qubits because marked-state indexes use u64",
            ));
        }

        if marked_states.is_empty() {
            return Err(invalid_configuration(
                "marked_states",
                "Grover requires at least one marked state",
            ));
        }

        if marked_states.len() > MAX_MARKED_STATES {
            return Err(invalid_configuration(
                "marked_states",
                "too many marked states",
            ));
        }

        let search_space = search_space_size(search_qubits)?;

        let mut unique = BTreeSet::new();

        for &state in &marked_states {
            if state >= search_space {
                return Err(invalid_configuration(
                    "marked_states",
                    "marked state is outside the computational search space",
                ));
            }

            if !unique.insert(state) {
                return Err(invalid_configuration(
                    "marked_states",
                    "marked_states contains a duplicate state",
                ));
            }
        }

        Ok(Self {
            search_qubits,
            marked_states,
        })
    }

    /// Number of computational-basis states in the search space.
    pub fn search_space_size(&self) -> BenchmarkResult<u64> {
        search_space_size(self.search_qubits)
    }

    /// Number of marked states.
    #[must_use]
    pub fn solution_count(&self) -> usize {
        self.marked_states.len()
    }

    /// Returns the exact Grover angle.
    pub fn theta(&self) -> BenchmarkResult<f64> {
        let n = self.search_space_size()? as f64;
        let m = self.solution_count() as f64;

        let ratio = m / n;

        if !ratio.is_finite() || ratio <= 0.0 || ratio > 1.0 {
            return Err(invalid_configuration(
                "marked_states",
                "marked-state fraction is outside (0, 1]",
            ));
        }

        let theta = ratio.sqrt().asin();

        if !theta.is_finite() || theta <= 0.0 {
            return Err(BenchmarkError::NumericalInstability {
                operation: "grover_theta".to_owned(),
                message: "Grover angle is non-finite or non-positive".to_owned(),
            });
        }

        Ok(theta)
    }

    /// Exact ideal success probability after `iterations`.
    pub fn ideal_success_probability(
        &self,
        iterations: u64,
    ) -> BenchmarkResult<f64> {
        let theta = self.theta()?;

        let multiplier = iterations
            .checked_mul(2)
            .and_then(|value| value.checked_add(1))
            .ok_or_else(|| numerical_overflow("Grover success-probability angle"))?;

        let angle = (multiplier as f64) * theta;
        let probability = angle.sin().powi(2);

        validate_probability(
            "ideal_success_probability",
            probability,
        )
    }

    /// Selects the integer that maximizes the exact ideal success probability.
    pub fn optimal_iterations(&self) -> BenchmarkResult<u64> {
        if self.solution_count() == self.search_space_size()? as usize {
            return Ok(0);
        }

        let theta = self.theta()?;

        let estimate =
            (std::f64::consts::PI / (4.0 * theta)) - 0.5;

        if !estimate.is_finite() || estimate < 0.0 {
            return Err(BenchmarkError::NumericalInstability {
                operation: "grover_optimal_iterations".to_owned(),
                message: "optimal Grover iteration estimate is invalid".to_owned(),
            });
        }

        let floor = estimate.floor() as u64;

        // Evaluate the nearest candidates rather than trusting the asymptotic
        // approximation alone.
        let mut candidates = BTreeSet::new();

        candidates.insert(floor);

        if floor > 0 {
            candidates.insert(floor - 1);
        }

        if floor < MAX_GROVER_ITERATIONS {
            candidates.insert(floor + 1);
        }

        let mut best_iteration = floor;
        let mut best_probability = -1.0_f64;

        for candidate in candidates {
            if candidate > MAX_GROVER_ITERATIONS {
                continue;
            }

            let probability =
                self.ideal_success_probability(candidate)?;

            if probability > best_probability
                || (probability == best_probability
                    && candidate < best_iteration)
            {
                best_probability = probability;
                best_iteration = candidate;
            }
        }

        if best_iteration > MAX_GROVER_ITERATIONS {
            return Err(BenchmarkError::ResourceLimitExceeded {
                resource: "grover_iterations".to_owned(),
                requested: best_iteration,
                maximum: MAX_GROVER_ITERATIONS,
            });
        }

        Ok(best_iteration)
    }

    /// Classical random-guess success probability.
    pub fn classical_baseline(&self) -> BenchmarkResult<f64> {
        let n = self.search_space_size()? as f64;
        let m = self.solution_count() as f64;

        validate_probability(
            "classical_baseline",
            m / n,
        )
    }
}

// =============================================================================
// Resource accounting
// =============================================================================

/// Logical resource accounting for one generated Grover circuit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct GroverResources {
    /// Number of search-register qubits.
    pub search_qubits: usize,

    /// Number of reusable oracle/diffusion ancillas.
    pub ancilla_qubits: usize,

    /// Total logical qubits.
    pub total_qubits: usize,

    /// Total logical gate count.
    pub logical_gate_count: usize,

    /// Total logical two-qubit gate count.
    pub logical_two_qubit_gate_count: usize,

    /// Number of Grover iterations.
    pub iterations: u64,

    /// Number of oracle invocations.
    pub oracle_calls: u64,

    /// Number of final measurements.
    pub measurement_count: usize,
}

impl GroverResources {
    /// Returns the number of ancillas required by the canonical MCZ ladder.
    #[must_use]
    pub const fn ancilla_count(search_qubits: usize) -> usize {
        if search_qubits <= 2 {
            0
        } else {
            search_qubits - 2
        }
    }

    /// Calculates resource requirements before circuit construction.
    pub fn estimate(
        problem: &GroverProblem,
        iterations: u64,
    ) -> BenchmarkResult<Self> {
        let n = problem.search_qubits;
        let marked = problem.solution_count();

        let ancillas = Self::ancilla_count(n);

        let total_qubits = n
            .checked_add(ancillas)
            .ok_or_else(|| numerical_overflow("Grover total logical qubits"))?;

        // Each MCZ on n operands requires:
        //
        // n=1: one Z
        // n=2: one CZ
        // n>=3:
        //   n-2 CCX gates to compute the conjunction,
        //   one CZ,
        //   n-2 CCX gates to uncompute.
        //
        // Therefore:
        //
        //   gates = 1                       for n=1
        //         = 1                       for n=2
        //         = 2(n-2) + 1             for n>=3
        //
        // and two-qubit gates are:
        //
        //   0 for n=1
        //   1 for n=2
        //   2(n-2) for n>=3.
        let mcz_gates = mcz_gate_count(n)?;
        let mcz_two_qubit = mcz_two_qubit_gate_count(n)?;

        // A marked state with zero bits requires one X before and one X after
        // the MCZ for every zero bit.
        let zero_bits_per_marked_state =
            marked_state_zero_bits_upper_bound(n);

        let oracle_gates_per_marked_state = mcz_gates
            .checked_add(
                zero_bits_per_marked_state
                    .checked_mul(2)
                    .ok_or_else(|| numerical_overflow(
                        "Grover oracle X-gate count",
                    ))?,
            )
            .ok_or_else(|| numerical_overflow(
                "Grover oracle gate count",
            ))?;

        let oracle_two_qubit_per_marked_state =
            mcz_two_qubit;

        let oracle_gates = marked
            .checked_mul(oracle_gates_per_marked_state)
            .ok_or_else(|| numerical_overflow(
                "Grover oracle gate count",
            ))?;

        let oracle_two_qubit = marked
            .checked_mul(oracle_two_qubit_per_marked_state)
            .ok_or_else(|| numerical_overflow(
                "Grover oracle two-qubit gate count",
            ))?;

        // Diffusion:
        //
        // H^n X^n MCZ X^n H^n
        //
        // = 4n single-qubit gates + MCZ.
        let diffusion_gates = n
            .checked_mul(4)
            .and_then(|value| value.checked_add(mcz_gates))
            .ok_or_else(|| numerical_overflow(
                "Grover diffusion gate count",
            ))?;

        let diffusion_two_qubit =
            mcz_two_qubit;

        let iteration_gates = oracle_gates
            .checked_add(diffusion_gates)
            .ok_or_else(|| numerical_overflow(
                "Grover iteration gate count",
            ))?;

        let iteration_two_qubit =
            oracle_two_qubit
                .checked_add(diffusion_two_qubit)
                .ok_or_else(|| numerical_overflow(
                    "Grover iteration two-qubit gate count",
                ))?;

        let iterative_gate_count = iteration_gates
            .checked_mul(
                usize::try_from(iterations).map_err(|_| {
                    numerical_overflow(
                        "Grover iteration count conversion",
                    )
                })?,
            )
            .ok_or_else(|| numerical_overflow(
                "Grover total gate count",
            ))?;

        let iterative_two_qubit = iteration_two_qubit
            .checked_mul(
                usize::try_from(iterations).map_err(|_| {
                    numerical_overflow(
                        "Grover iteration count conversion",
                    )
                })?,
            )
            .ok_or_else(|| numerical_overflow(
                "Grover total two-qubit gate count",
            ))?;

        let logical_gate_count = iterative_gate_count
            .checked_add(n)
            .and_then(|value| value.checked_add(n))
            .ok_or_else(|| numerical_overflow(
                "Grover total gate count",
            ))?;

        // The first +n term is initial H preparation.
        // The second +n term is final measurement count represented as
        // operations in the canonical logical workload accounting.
        //
        // Measurement gates are intentionally counted separately in the
        // `measurement_count` field as well.
        let measurement_count = n;

        let logical_two_qubit_gate_count =
            iterative_two_qubit;

        let oracle_calls = iterations;

        Ok(Self {
            search_qubits: n,
            ancilla_qubits: ancillas,
            total_qubits,
            logical_gate_count,
            logical_two_qubit_gate_count,
            iterations,
            oracle_calls,
            measurement_count,
        })
    }
}

// =============================================================================
// Typed benchmark description
// =============================================================================

/// Strongly typed description of one Grover application benchmark instance.
#[derive(Debug, Clone, PartialEq)]
pub struct GroverWorkloadDescription {
    /// Mathematical Grover problem.
    pub problem: GroverProblem,

    /// Iteration policy.
    pub iteration_policy: GroverIterationPolicy,

    /// Resolved iteration count.
    pub iterations: u64,

    /// Exact ideal success probability at the resolved iteration count.
    pub ideal_success_probability: f64,

    /// Classical random-guess baseline.
    pub classical_baseline: f64,

    /// Logical resource estimate.
    pub resources: GroverResources,
}

impl GroverWorkloadDescription {
    /// Creates a complete validated benchmark description.
    pub fn new(
        problem: GroverProblem,
        iteration_policy: GroverIterationPolicy,
    ) -> BenchmarkResult<Self> {
        let iterations = match iteration_policy {
            GroverIterationPolicy::Optimal => {
                problem.optimal_iterations()?
            }

            GroverIterationPolicy::Explicit(value) => {
                if value > MAX_GROVER_ITERATIONS {
                    return Err(BenchmarkError::ResourceLimitExceeded {
                        resource: "grover_iterations".to_owned(),
                        requested: value,
                        maximum: MAX_GROVER_ITERATIONS,
                    });
                }

                value
            }
        };

        let ideal_success_probability =
            problem.ideal_success_probability(iterations)?;

        let classical_baseline =
            problem.classical_baseline()?;

        let resources =
            GroverResources::estimate(&problem, iterations)?;

        validate_resource_limits(&resources)?;

        Ok(Self {
            problem,
            iteration_policy,
            iterations,
            ideal_success_probability,
            classical_baseline,
            resources,
        })
    }

    /// Returns the total number of marked states.
    #[must_use]
    pub fn solution_count(&self) -> usize {
        self.problem.solution_count()
    }

    /// Returns the search-space size.
    pub fn search_space_size(&self) -> BenchmarkResult<u64> {
        self.problem.search_space_size()
    }

    /// Returns whether the ideal algorithm exceeds the random-guess baseline.
    #[must_use]
    pub fn is_quantum_amplified(&self) -> bool {
        self.ideal_success_probability > self.classical_baseline
    }
}

// =============================================================================
// Benchmark result
// =============================================================================

/// Result of analyzing normalized Grover measurement counts.
///
/// The counts must already use Zamani's canonical bit-string convention.
#[derive(Debug, Clone, PartialEq)]
pub struct GroverBenchmarkResult {
    /// Benchmark/application identifier.
    pub benchmark_id: &'static str,

    /// Result schema version.
    pub schema_version: u16,

    /// Search-qubit count.
    pub search_qubits: usize,

    /// Search-space size.
    pub search_space_size: u64,

    /// Number of marked states.
    pub solution_count: usize,

    /// Marked-state indexes.
    pub marked_states: Vec<u64>,

    /// Number of Grover iterations.
    pub iterations: u64,

    /// Number of oracle calls.
    pub oracle_calls: u64,

    /// Total measurement shots.
    pub shots: u64,

    /// Observed probability of measuring a marked state.
    pub observed_success_probability: f64,

    /// Ideal probability for the generated Grover circuit.
    pub ideal_success_probability: f64,

    /// Absolute difference between observed and ideal success probability.
    pub absolute_success_probability_error: f64,

    /// Relative difference from the ideal probability.
    ///
    /// This is zero when the ideal probability is zero.
    pub relative_success_probability_error: f64,

    /// Classical random-guess baseline.
    pub classical_baseline: f64,

    /// Observed/ideal success ratio.
    ///
    /// This is bounded by zero when the ideal probability is zero.
    pub observed_to_ideal_ratio: f64,

    /// Observed quantum/classical success ratio.
    pub observed_to_classical_ratio: f64,

    /// Number of measured marked-state shots.
    pub marked_state_shots: u64,

    /// Logical resource accounting.
    pub resources: GroverResources,
}

impl GroverBenchmarkResult {
    /// Validates the result invariants.
    pub fn validate(&self) -> BenchmarkResult<()> {
        if self.benchmark_id != GROVER_BENCHMARK_ID {
            return Err(BenchmarkError::InvalidWorkload {
                workload: GROVER_APPLICATION_ID.to_owned(),
                reason: "Grover result has an invalid benchmark identifier"
                    .to_owned(),
            });
        }

        if self.schema_version != GROVER_RESULT_SCHEMA_VERSION {
            return Err(BenchmarkError::ReproducibilityFailure {
                component: "grover_result_schema".to_owned(),
                expected: GROVER_RESULT_SCHEMA_VERSION.to_string(),
                actual: self.schema_version.to_string(),
            });
        }

        if self.search_qubits == 0
            || self.search_qubits > MAX_GROVER_SEARCH_QUBITS
        {
            return Err(invalid_configuration(
                "search_qubits",
                "Grover result contains an invalid search-qubit count",
            ));
        }

        if self.marked_states.is_empty()
            || self.solution_count != self.marked_states.len()
        {
            return Err(BenchmarkError::InvalidWorkload {
                workload: GROVER_APPLICATION_ID.to_owned(),
                reason:
                    "Grover result contains inconsistent marked-state metadata"
                        .to_owned(),
            });
        }

        if self.marked_state_shots > self.shots {
            return Err(BenchmarkError::InvalidWorkload {
                workload: GROVER_APPLICATION_ID.to_owned(),
                reason:
                    "marked-state shots cannot exceed total shots"
                        .to_owned(),
            });
        }

        validate_probability(
            "observed_success_probability",
            self.observed_success_probability,
        )?;

        validate_probability(
            "ideal_success_probability",
            self.ideal_success_probability,
        )?;

        validate_probability(
            "classical_baseline",
            self.classical_baseline,
        )?;

        if !self.absolute_success_probability_error.is_finite()
            || self.absolute_success_probability_error < 0.0
        {
            return Err(BenchmarkError::NumericalInstability {
                operation: "grover_result_validation".to_owned(),
                message:
                    "absolute success-probability error is invalid"
                        .to_owned(),
            });
        }

        if !self.relative_success_probability_error.is_finite()
            || self.relative_success_probability_error < 0.0
        {
            return Err(BenchmarkError::NumericalInstability {
                operation: "grover_result_validation".to_owned(),
                message:
                    "relative success-probability error is invalid"
                        .to_owned(),
            });
        }

        if !self.observed_to_ideal_ratio.is_finite()
            || self.observed_to_ideal_ratio < 0.0
        {
            return Err(BenchmarkError::NumericalInstability {
                operation: "grover_result_validation".to_owned(),
                message:
                    "observed-to-ideal ratio is invalid".to_owned(),
            });
        }

        if !self.observed_to_classical_ratio.is_finite()
            || self.observed_to_classical_ratio < 0.0
        {
            return Err(BenchmarkError::NumericalInstability {
                operation: "grover_result_validation".to_owned(),
                message:
                    "observed-to-classical ratio is invalid".to_owned(),
            });
        }

        Ok(())
    }
}

// =============================================================================
// Generator
// =============================================================================

/// Stateless production Grover application benchmark generator.
#[derive(Debug, Clone)]
pub struct GroverBenchmarkGenerator {
    descriptor: ApplicationGeneratorDescriptor,
}

impl GroverBenchmarkGenerator {
    /// Creates the canonical Grover benchmark generator.
    pub fn new() -> BenchmarkResult<Self> {
        let descriptor = ApplicationGeneratorDescriptor::new(
            GROVER_BENCHMARK_ID,
            GROVER_APPLICATION_ID,
            GROVER_GENERATOR_VERSION,
            "Production Grover Search application benchmark generator",
        )?
        .with_capabilities([
            ApplicationGeneratorCapability::GeneratesCircuit,
            ApplicationGeneratorCapability::Deterministic,
            ApplicationGeneratorCapability::BatchGeneration,
            ApplicationGeneratorCapability::ScalableProblemSize,
            ApplicationGeneratorCapability::Parameterized,
            ApplicationGeneratorCapability::ExactSmallInstanceReference,
            ApplicationGeneratorCapability::ClassicallyVerifiable,
            ApplicationGeneratorCapability::ResourceEstimation,
            ApplicationGeneratorCapability::HardwareExecutable,
        ]);

        Ok(Self { descriptor })
    }

    /// Parses the benchmark request into a typed Grover problem.
    pub fn problem_from_request(
        &self,
        request: &ApplicationGenerationRequest,
    ) -> BenchmarkResult<GroverProblem> {
        request.validate()?;

        self.ensure_application(request)?;

        parse_problem_parameters(request)
    }

    /// Resolves the iteration policy from request parameters.
    pub fn iteration_policy_from_request(
        &self,
        request: &ApplicationGenerationRequest,
    ) -> BenchmarkResult<GroverIterationPolicy> {
        request.validate()?;
        self.ensure_application(request)?;

        let mut policy: Option<GroverIterationPolicy> = None;

        for parameter in request.parameters() {
            if parameter.name() != "iterations" {
                continue;
            }

            if policy.is_some() {
                return Err(invalid_configuration(
                    "iterations",
                    "duplicate iterations parameter",
                ));
            }

            let value = parameter.value();

            if value == "optimal" {
                policy = Some(GroverIterationPolicy::Optimal);
            } else {
                let iterations =
                    value.parse::<u64>().map_err(|_| {
                        invalid_configuration(
                            "iterations",
                            "iterations must be `optimal` or a non-negative integer",
                        )
                    })?;

                if iterations > MAX_GROVER_ITERATIONS {
                    return Err(BenchmarkError::ResourceLimitExceeded {
                        resource: "grover_iterations".to_owned(),
                        requested: iterations,
                        maximum: MAX_GROVER_ITERATIONS,
                    });
                }

                policy = Some(
                    GroverIterationPolicy::Explicit(iterations),
                );
            }
        }

        Ok(policy.unwrap_or(
            GroverIterationPolicy::Optimal,
        ))
    }

    /// Describes a request without allocating Quantum IR.
    pub fn describe(
        &self,
        request: &ApplicationGenerationRequest,
    ) -> BenchmarkResult<GroverWorkloadDescription> {
        request.validate()?;
        self.ensure_application(request)?;

        let problem =
            self.problem_from_request(request)?;

        let policy =
            self.iteration_policy_from_request(request)?;

        GroverWorkloadDescription::new(
            problem,
            policy,
        )
    }

    /// Generates the canonical logical Quantum IR circuit.
    pub fn generate_circuit(
        &self,
        request: &ApplicationGenerationRequest,
    ) -> BenchmarkResult<QuantumCircuit> {
        let description =
            self.describe(request)?;

        let mut circuit =
            QuantumCircuit::new(
                description.resources.total_qubits,
                description.problem.search_qubits,
            )
            .map_err(|error| {
                circuit_error(
                    "unable to construct Grover Quantum IR circuit",
                    error,
                )
            })?;

        circuit
            .set_name(Some(format!(
                "grover_{}",
                request.instance_id().as_str()
            )))
            .map_err(|error| {
                circuit_error(
                    "unable to assign Grover circuit name",
                    error,
                )
            })?;

        circuit
            .set_source(Some(
                "zamani.quantum.benchmarking.applications.grover"
                    .to_owned(),
            ))
            .map_err(|error| {
                circuit_error(
                    "unable to assign Grover circuit source",
                    error,
                )
            })?;

        // ---------------------------------------------------------------------
        // Uniform superposition.
        // ---------------------------------------------------------------------

        for qubit in 0..description.problem.search_qubits {
            push_single(
                &mut circuit,
                GateKind::H,
                qubit,
            )?;
        }

        // ---------------------------------------------------------------------
        // Grover iterations.
        // ---------------------------------------------------------------------

        for _ in 0..description.iterations {
            apply_phase_oracle(
                &mut circuit,
                description.problem.search_qubits,
                &description.problem.marked_states,
                description.resources.ancilla_qubits,
            )?;

            apply_diffusion(
                &mut circuit,
                description.problem.search_qubits,
                description.resources.ancilla_qubits,
            )?;
        }

        // ---------------------------------------------------------------------
        // Final measurement.
        // ---------------------------------------------------------------------

        for qubit in 0..description.problem.search_qubits {
            push_measurement(
                &mut circuit,
                qubit,
                qubit,
            )?;
        }

        circuit
            .validate()
            .map_err(|error| {
                circuit_error(
                    "generated Grover circuit failed final IR validation",
                    error,
                )
            })?;

        Ok(circuit)
    }

    /// Generates the canonical application workload.
    pub fn generate_application_workload(
        &self,
        request: &ApplicationGenerationRequest,
    ) -> BenchmarkResult<ApplicationWorkload> {
        let description =
            self.describe(request)?;

        let circuit =
            self.generate_circuit(request)?;

        let circuit_workload =
            CircuitWorkload::from_circuit(
                circuit,
                request.instance_id().clone(),
            )
            .map_err(|error| {
                workload_error(
                    "unable to create Grover circuit workload",
                    error,
                )
            })?;

        let mut workload =
            ApplicationWorkload::new(
                GROVER_APPLICATION_ID,
                request.instance_id().clone(),
                request.problem_size(),
            )
            .map_err(|error| {
                workload_error(
                    "unable to create Grover application workload",
                    error,
                )
            })?
            .with_circuit(circuit_workload);

        add_parameter(
            &mut workload,
            "search_qubits",
            &description
                .problem
                .search_qubits
                .to_string(),
        )?;

        add_parameter(
            &mut workload,
            "search_space_size",
            &description
                .search_space_size()?
                .to_string(),
        )?;

        add_parameter(
            &mut workload,
            "solution_count",
            &description
                .solution_count()
                .to_string(),
        )?;

        add_parameter(
            &mut workload,
            "marked_states",
            &format_marked_states(
                &description.problem.marked_states,
            ),
        )?;

        add_parameter(
            &mut workload,
            "iteration_policy",
            description
                .iteration_policy
                .as_str(),
        )?;

        add_parameter(
            &mut workload,
            "iterations",
            &description.iterations.to_string(),
        )?;

        add_parameter(
            &mut workload,
            "ideal_success_probability",
            &format_probability(
                description.ideal_success_probability,
            ),
        )?;

        add_parameter(
            &mut workload,
            "classical_baseline",
            &format_probability(
                description.classical_baseline,
            ),
        )?;

        add_parameter(
            &mut workload,
            "oracle_calls",
            &description
                .resources
                .oracle_calls
                .to_string(),
        )?;

        add_parameter(
            &mut workload,
            "logical_gate_count",
            &description
                .resources
                .logical_gate_count
                .to_string(),
        )?;

        add_parameter(
            &mut workload,
            "logical_two_qubit_gate_count",
            &description
                .resources
                .logical_two_qubit_gate_count
                .to_string(),
        )?;

        add_parameter(
            &mut workload,
            "search_qubits",
            &description
                .resources
                .search_qubits
                .to_string(),
        )?;

        add_parameter(
            &mut workload,
            "ancilla_qubits",
            &description
                .resources
                .ancilla_qubits
                .to_string(),
        )?;

        add_parameter(
            &mut workload,
            "total_logical_qubits",
            &description
                .resources
                .total_qubits
                .to_string(),
        )?;

        add_parameter(
            &mut workload,
            "generator_version",
            GROVER_GENERATOR_VERSION,
        )?;

        add_parameter(
            &mut workload,
            "generator_revision",
            &GROVER_GENERATOR_REVISION
                .to_string(),
        )?;

        Ok(workload)
    }

    /// Analyzes canonical measurement counts against the generated problem.
    pub fn analyze_counts(
        &self,
        request: &ApplicationGenerationRequest,
        counts: &BTreeMap<String, u64>,
    ) -> BenchmarkResult<GroverBenchmarkResult> {
        let description =
            self.describe(request)?;

        analyze_counts_for_description(
            &description,
            counts,
        )
    }

    fn ensure_application(
        &self,
        request: &ApplicationGenerationRequest,
    ) -> BenchmarkResult<()> {
        if request.application_id()
            != GROVER_APPLICATION_ID
        {
            return Err(
                BenchmarkError::InconsistentConfiguration {
                    first:
                        "request.application_id"
                            .to_owned(),
                    second:
                        "grover.application_id"
                            .to_owned(),
                    reason:
                        "Grover generator requires application_id `grover`"
                            .to_owned(),
                },
            );
        }

        Ok(())
    }
}

impl ApplicationBenchmarkGenerator
    for GroverBenchmarkGenerator
{
    fn descriptor(
        &self,
    ) -> &ApplicationGeneratorDescriptor {
        &self.descriptor
    }

    fn validate(
        &self,
        request: &ApplicationGenerationRequest,
    ) -> BenchmarkResult<()> {
        request.validate()?;
        self.ensure_application(request)?;

        let problem =
            parse_problem_parameters(request)?;

        let policy =
            parse_iteration_policy(request)?;

        let description =
            GroverWorkloadDescription::new(
                problem,
                policy,
            )?;

        validate_resource_limits(
            &description.resources,
        )?;

        Ok(())
    }

    fn generate_workload(
        &self,
        request: &ApplicationGenerationRequest,
    ) -> BenchmarkResult<ApplicationWorkload> {
        self.generate_application_workload(request)
    }
}

// =============================================================================
// Request parsing
// =============================================================================

/// Parses all Grover-specific parameters.
///
/// Supported parameters:
///
/// ```text
/// marked_state = 5
/// marked_states = 1,4,7
/// solution_count = 2
/// iterations = optimal
/// iterations = 3
/// ```
///
/// `marked_state` and `marked_states` are mutually exclusive.
///
/// If neither is supplied, `solution_count` defaults to one and the marked
/// state is derived deterministically from the request seed and sequence index.
///
/// If `solution_count > 1` is supplied without explicit marked states, the
/// lowest `solution_count` states are used. This is deterministic and avoids
/// enumerating the entire search space.
fn parse_problem_parameters(
    request: &ApplicationGenerationRequest,
) -> BenchmarkResult<GroverProblem> {
    let n = request.problem_size();

    let mut marked_state: Option<&str> = None;
    let mut marked_states: Option<&str> = None;
    let mut solution_count: Option<usize> = None;

    for parameter in request.parameters() {
        match parameter.name() {
            "marked_state" => {
                if marked_state.is_some() {
                    return Err(invalid_configuration(
                        "marked_state",
                        "duplicate marked_state parameter",
                    ));
                }

                if marked_states.is_some() {
                    return Err(invalid_configuration(
                        "marked_state",
                        "marked_state cannot be combined with marked_states",
                    ));
                }

                marked_state =
                    Some(parameter.value());
            }

            "marked_states" => {
                if marked_states.is_some() {
                    return Err(invalid_configuration(
                        "marked_states",
                        "duplicate marked_states parameter",
                    ));
                }

                if marked_state.is_some() {
                    return Err(invalid_configuration(
                        "marked_states",
                        "marked_states cannot be combined with marked_state",
                    ));
                }

                if parameter.value().len()
                    > MAX_MARKED_STATES_PARAMETER_BYTES
                {
                    return Err(invalid_configuration(
                        "marked_states",
                        "marked_states parameter is too large",
                    ));
                }

                marked_states =
                    Some(parameter.value());
            }

            "solution_count" => {
                if solution_count.is_some() {
                    return Err(invalid_configuration(
                        "solution_count",
                        "duplicate solution_count parameter",
                    ));
                }

                let value =
                    parameter.value()
                        .parse::<usize>()
                        .map_err(|_| {
                            invalid_configuration(
                                "solution_count",
                                "solution_count must be a positive integer",
                            )
                        })?;

                if value == 0 {
                    return Err(invalid_configuration(
                        "solution_count",
                        "solution_count must be greater than zero",
                    ));
                }

                if value > MAX_MARKED_STATES {
                    return Err(
                        BenchmarkError::ResourceLimitExceeded {
                            resource:
                                "grover_solution_count"
                                    .to_owned(),
                            requested: value as u64,
                            maximum:
                                MAX_MARKED_STATES as u64,
                        },
                    );
                }

                solution_count = Some(value);
            }

            "iterations" => {
                // Parsed by parse_iteration_policy().
            }

            other => {
                return Err(invalid_configuration(
                    "application_parameter",
                    match other {
                        "" => {
                            "application parameter name must not be empty"
                        }
                        _ => {
                            "unknown Grover application parameter"
                        }
                    },
                ));
            }
        }
    }

    let search_space =
        search_space_size(n)?;

    let states =
        if let Some(value) = marked_states {
            let parsed =
                parse_marked_states(
                    value,
                    search_space,
                )?;

            if let Some(expected) =
                solution_count
            {
                if parsed.len() != expected {
                    return Err(
                        BenchmarkError::InconsistentConfiguration {
                            first:
                                "solution_count".to_owned(),
                            second:
                                "marked_states".to_owned(),
                            reason:
                                "solution_count must equal the number of explicitly marked states"
                                    .to_owned(),
                        },
                    );
                }
            }

            parsed
        } else if let Some(value) = marked_state {
            let state =
                parse_marked_state(
                    value,
                    search_space,
                )?;

            if let Some(expected) =
                solution_count
            {
                if expected != 1 {
                    return Err(
                        BenchmarkError::InconsistentConfiguration {
                            first:
                                "solution_count".to_owned(),
                            second:
                                "marked_state".to_owned(),
                            reason:
                                "a single marked_state requires solution_count=1"
                                    .to_owned(),
                        },
                    );
                }
            }

            vec![state]
        } else {
            let count =
                solution_count.unwrap_or(1);

            let start =
                deterministic_marked_state(
                    request,
                    search_space,
                );

            let mut derived =
                Vec::with_capacity(count);

            for offset in 0..count {
                let offset_u64 =
                    offset as u64;

                let state =
                    start
                        .checked_add(offset_u64)
                        .ok_or_else(|| {
                            numerical_overflow(
                                "deterministic Grover marked-state generation",
                            )
                        })?
                        % search_space;

                derived.push(state);
            }

            derived.sort_unstable();
            derived.dedup();

            // The requested count can only fail here when wrapping through the
            // finite search space caused duplicates.
            if derived.len() != count {
                return Err(invalid_configuration(
                    "solution_count",
                    "cannot derive the requested number of distinct marked states from the search space",
                ));
            }

            derived
        };

    GroverProblem::new(
        n,
        states,
    )
}

fn parse_iteration_policy(
    request: &ApplicationGenerationRequest,
) -> BenchmarkResult<GroverIterationPolicy> {
    let mut result: Option<
        GroverIterationPolicy,
    > = None;

    for parameter in request.parameters() {
        if parameter.name() != "iterations" {
            continue;
        }

        if result.is_some() {
            return Err(invalid_configuration(
                "iterations",
                "duplicate iterations parameter",
            ));
        }

        result = Some(
            parse_iteration_value(
                parameter.value(),
            )?,
        );
    }

    Ok(result.unwrap_or(
        GroverIterationPolicy::Optimal,
    ))
}

fn parse_iteration_value(
    value: &str,
) -> BenchmarkResult<GroverIterationPolicy> {
    if value == "optimal" {
        return Ok(
            GroverIterationPolicy::Optimal,
        );
    }

    let iterations =
        value.parse::<u64>().map_err(|_| {
            invalid_configuration(
                "iterations",
                "iterations must be `optimal` or a non-negative integer",
            )
        })?;

    if iterations > MAX_GROVER_ITERATIONS {
        return Err(
            BenchmarkError::ResourceLimitExceeded {
                resource:
                    "grover_iterations"
                        .to_owned(),
                requested: iterations,
                maximum:
                    MAX_GROVER_ITERATIONS,
            },
        );
    }

    Ok(
        GroverIterationPolicy::Explicit(
            iterations,
        ),
    )
}

fn parse_marked_state(
    value: &str,
    search_space: u64,
) -> BenchmarkResult<u64> {
    let state =
        value.parse::<u64>().map_err(|_| {
            invalid_configuration(
                "marked_state",
                "marked_state must be a non-negative integer",
            )
        })?;

    if state >= search_space {
        return Err(invalid_configuration(
            "marked_state",
            "marked_state is outside the computational search space",
        ));
    }

    Ok(state)
}

fn parse_marked_states(
    value: &str,
    search_space: u64,
) -> BenchmarkResult<Vec<u64>> {
    if value.trim().is_empty() {
        return Err(invalid_configuration(
            "marked_states",
            "marked_states must not be empty",
        ));
    }

    let mut result =
        Vec::new();

    let mut seen =
        BTreeSet::new();

    for token in value.split(',') {
        if result.len() >= MAX_MARKED_STATES {
            return Err(
                BenchmarkError::ResourceLimitExceeded {
                    resource:
                        "grover_marked_states"
                            .to_owned(),
                    requested:
                        result.len() as u64 + 1,
                    maximum:
                        MAX_MARKED_STATES as u64,
                },
            );
        }

        let token =
            token.trim();

        if token.is_empty() {
            return Err(invalid_configuration(
                "marked_states",
                "marked_states contains an empty entry",
            ));
        }

        let state =
            parse_marked_state(
                token,
                search_space,
            )?;

        if !seen.insert(state) {
            return Err(invalid_configuration(
                "marked_states",
                "marked_states contains a duplicate state",
            ));
        }

        result.push(state);
    }

    if result.is_empty() {
        return Err(invalid_configuration(
            "marked_states",
            "marked_states must contain at least one state",
        ));
    }

    result.sort_unstable();

    Ok(result)
}

// =============================================================================
// Circuit generation
// =============================================================================

fn apply_phase_oracle(
    circuit: &mut QuantumCircuit,
    search_qubits: usize,
    marked_states: &[u64],
    ancilla_count: usize,
) -> BenchmarkResult<()> {
    let ancilla_start =
        search_qubits;

    for &marked_state in marked_states {
        for qubit in 0..search_qubits {
            if ((marked_state >> qubit) & 1) == 0 {
                push_single(
                    circuit,
                    GateKind::X,
                    qubit,
                )?;
            }
        }

        apply_multi_controlled_z(
            circuit,
            search_qubits,
            ancilla_start,
        )?;

        for qubit in 0..search_qubits {
            if ((marked_state >> qubit) & 1) == 0 {
                push_single(
                    circuit,
                    GateKind::X,
                    qubit,
                )?;
            }
        }
    }

    Ok(())
}

fn apply_diffusion(
    circuit: &mut QuantumCircuit,
    search_qubits: usize,
    ancilla_count: usize,
) -> BenchmarkResult<()> {
    for qubit in 0..search_qubits {
        push_single(
            circuit,
            GateKind::H,
            qubit,
        )?;
    }

    for qubit in 0..search_qubits {
        push_single(
            circuit,
            GateKind::X,
            qubit,
        )?;
    }

    apply_multi_controlled_z(
        circuit,
        search_qubits,
        search_qubits,
    )?;

    for qubit in 0..search_qubits {
        push_single(
            circuit,
            GateKind::X,
            qubit,
        )?;
    }

    for qubit in 0..search_qubits {
        push_single(
            circuit,
            GateKind::H,
            qubit,
        )?;
    }

    // `ancilla_count` is deliberately accepted as an explicit contract
    // parameter even though the MCZ helper derives the same layout. This
    // catches future accidental changes to the circuit layout at compile/API
    // boundaries without introducing hidden global state.
    if ancilla_count != GroverResources::ancilla_count(search_qubits) {
        return Err(invalid_configuration(
            "ancilla_count",
            "Grover diffusion ancilla layout does not match the resource model",
        ));
    }

    Ok(())
}

/// Applies a multi-controlled Z over all `search_qubits`.
///
/// The operation is decomposed into the canonical IR's CCX/CZ gates using a
/// reusable ancilla ladder.
///
/// For:
///
/// ```text
/// n = 1: Z(q0)
/// n = 2: CZ(q0,q1)
/// n >= 3:
///     CCX(q0,q1,a0)
///     CCX(a0,q2,a1)
///     ...
///     CZ(a_last,q_last)
///     ...
///     CCX(a0,q2,a1)
///     CCX(q0,q1,a0)
/// ```
///
/// The ancilla register returns to |0> after the operation.
fn apply_multi_controlled_z(
    circuit: &mut QuantumCircuit,
    search_qubits: usize,
    ancilla_start: usize,
) -> BenchmarkResult<()> {
    match search_qubits {
        0 => Err(invalid_configuration(
            "search_qubits",
            "multi-controlled Z requires at least one search qubit",
        )),

        1 => {
            push_single(
                circuit,
                GateKind::Z,
                0,
            )
        }

        2 => {
            push_two(
                circuit,
                GateKind::CZ,
                0,
                1,
            )
        }

        n => {
            let ancilla_count =
                GroverResources::ancilla_count(n);

            let expected_ancilla_end =
                ancilla_start
                    .checked_add(ancilla_count)
                    .ok_or_else(|| {
                        numerical_overflow(
                            "Grover MCZ ancilla range",
                        )
                    })?;

            let required_end =
                n.checked_add(
                    ancilla_count,
                )
                .ok_or_else(|| {
                    numerical_overflow(
                        "Grover MCZ required qubit range",
                    )
                })?;

            if ancilla_start != n
                || expected_ancilla_end
                    != required_end
            {
                return Err(
                    invalid_configuration(
                        "ancilla_layout",
                        "Grover MCZ ancilla range is inconsistent with the logical circuit layout",
                    ),
                );
            }

            // Compute the conjunction of q0..q(n-2).
            push_three(
                circuit,
                GateKind::CCX,
                0,
                1,
                ancilla_start,
            )?;

            for control in 2..(n - 1) {
                let previous_ancilla =
                    ancilla_start
                        + (control - 2);

                let current_ancilla =
                    ancilla_start
                        + (control - 1);

                push_three(
                    circuit,
                    GateKind::CCX,
                    previous_ancilla,
                    control,
                    current_ancilla,
                )?;
            }

            let final_ancilla =
                ancilla_start
                    + (n - 3);

            push_two(
                circuit,
                GateKind::CZ,
                final_ancilla,
                n - 1,
            )?;

            // Uncompute the ancilla ladder in reverse.
            for control in (2..(n - 1)).rev() {
                let previous_ancilla =
                    ancilla_start
                        + (control - 2);

                let current_ancilla =
                    ancilla_start
                        + (control - 1);

                push_three(
                    circuit,
                    GateKind::CCX,
                    previous_ancilla,
                    control,
                    current_ancilla,
                )?;
            }

            push_three(
                circuit,
                GateKind::CCX,
                0,
                1,
                ancilla_start,
            )
        }
    }
}

// =============================================================================
// Gate helpers
// =============================================================================

fn push_single(
    circuit: &mut QuantumCircuit,
    kind: GateKind,
    qubit: usize,
) -> BenchmarkResult<()> {
    let gate = Gate::new(
        kind,
        vec![QubitId::new(qubit)],
        Vec::new(),
        None,
        None,
    )
    .map_err(|error| {
        invalid_workload(
            "Grover generated an invalid single-qubit gate",
            error,
        )
    })?;

    circuit
        .push(gate)
        .map_err(|error| {
            circuit_error(
                "unable to append Grover single-qubit gate",
                error,
            )
        })
}

fn push_two(
    circuit: &mut QuantumCircuit,
    kind: GateKind,
    first: usize,
    second: usize,
) -> BenchmarkResult<()> {
    if first == second {
        return Err(invalid_configuration(
            "gate_operands",
            "Grover two-qubit gate cannot target the same logical qubit",
        ));
    }

    let gate = Gate::new(
        kind,
        vec![
            QubitId::new(first),
            QubitId::new(second),
        ],
        Vec::new(),
        None,
        None,
    )
    .map_err(|error| {
        invalid_workload(
            "Grover generated an invalid two-qubit gate",
            error,
        )
    })?;

    circuit
        .push(gate)
        .map_err(|error| {
            circuit_error(
                "unable to append Grover two-qubit gate",
                error,
            )
        })
}

fn push_three(
    circuit: &mut QuantumCircuit,
    kind: GateKind,
    first: usize,
    second: usize,
    third: usize,
) -> BenchmarkResult<()> {
    if first == second
        || first == third
        || second == third
    {
        return Err(invalid_configuration(
            "gate_operands",
            "Grover three-qubit gate cannot contain duplicate logical qubits",
        ));
    }

    let gate = Gate::new(
        kind,
        vec![
            QubitId::new(first),
            QubitId::new(second),
            QubitId::new(third),
        ],
        Vec::new(),
        None,
        None,
    )
    .map_err(|error| {
        invalid_workload(
            "Grover generated an invalid three-qubit gate",
            error,
        )
    })?;

    circuit
        .push(gate)
        .map_err(|error| {
            circuit_error(
                "unable to append Grover three-qubit gate",
                error,
            )
        })
}

fn push_measurement(
    circuit: &mut QuantumCircuit,
    qubit: usize,
    classical_bit: usize,
) -> BenchmarkResult<()> {
    let gate = Gate::new(
        GateKind::Measure,
        vec![QubitId::new(qubit)],
        Vec::new(),
        Some(classical_bit),
        Some(Measurement::new(
            QubitId::new(qubit),
            ClassicalBitId::new(classical_bit),
        )),
    )
    .map_err(|error| {
        invalid_workload(
            "Grover generated an invalid measurement gate",
            error,
        )
    })?;

    circuit
        .push(gate)
        .map_err(|error| {
            circuit_error(
                "unable to append Grover measurement",
                error,
            )
        })
}

// =============================================================================
// Analysis
// =============================================================================

/// Analyzes normalized computational-basis counts.
fn analyze_counts_for_description(
    description: &GroverWorkloadDescription,
    counts: &BTreeMap<String, u64>,
) -> BenchmarkResult<GroverBenchmarkResult> {
    if counts.is_empty() {
        return Err(invalid_configuration(
            "counts",
            "Grover analysis requires at least one measurement outcome",
        ));
    }

    let n =
        description.problem.search_qubits;

    let mut total_shots =
        0_u64;

    let mut marked_shots =
        0_u64;

    for (bitstring, count) in counts {
        validate_bitstring(
            bitstring,
            n,
        )?;

        total_shots =
            total_shots
                .checked_add(*count)
                .ok_or_else(|| {
                    numerical_overflow(
                        "Grover total measurement shots",
                    )
                })?;

        if bitstring_is_marked(
            bitstring,
            &description.problem.marked_states,
            n,
        ) {
            marked_shots =
                marked_shots
                    .checked_add(*count)
                    .ok_or_else(|| {
                        numerical_overflow(
                            "Grover marked-state measurement shots",
                        )
                    })?;
        }
    }

    if total_shots == 0 {
        return Err(invalid_configuration(
            "counts",
            "Grover analysis requires a positive number of measurement shots",
        ));
    }

    let observed =
        marked_shots as f64
            / total_shots as f64;

    let ideal =
        description.ideal_success_probability;

    let absolute_error =
        (observed - ideal).abs();

    let relative_error =
        if ideal > 0.0 {
            absolute_error / ideal
        } else {
            0.0
        };

    let observed_to_ideal_ratio =
        if ideal > 0.0 {
            observed / ideal
        } else {
            0.0
        };

    let classical =
        description.classical_baseline;

    let observed_to_classical_ratio =
        if classical > 0.0 {
            observed / classical
        } else {
            0.0
        };

    let result =
        GroverBenchmarkResult {
            benchmark_id:
                GROVER_BENCHMARK_ID,
            schema_version:
                GROVER_RESULT_SCHEMA_VERSION,
            search_qubits:
                n,
            search_space_size:
                description
                    .search_space_size()?,
            solution_count:
                description
                    .solution_count(),
            marked_states:
                description
                    .problem
                    .marked_states
                    .clone(),
            iterations:
                description
                    .iterations,
            oracle_calls:
                description
                    .resources
                    .oracle_calls,
            shots:
                total_shots,
            observed_success_probability:
                observed,
            ideal_success_probability:
                ideal,
            absolute_success_probability_error:
                absolute_error,
            relative_success_probability_error:
                relative_error,
            classical_baseline:
                classical,
            observed_to_ideal_ratio,
            observed_to_classical_ratio,
            marked_state_shots:
                marked_shots,
            resources:
                description.resources,
        };

    result.validate()?;

    Ok(result)
}

fn validate_bitstring(
    bitstring: &str,
    search_qubits: usize,
) -> BenchmarkResult<()> {
    if bitstring.len() != search_qubits {
        return Err(invalid_configuration(
            "counts",
            "Grover measurement bitstring has an invalid length",
        ));
    }

    if !bitstring
        .bytes()
        .all(|byte| byte == b'0' || byte == b'1')
    {
        return Err(invalid_configuration(
            "counts",
            "Grover measurement bitstrings must contain only 0 and 1",
        ));
    }

    Ok(())
}

fn bitstring_is_marked(
    bitstring: &str,
    marked_states: &[u64],
    search_qubits: usize,
) -> bool {
    let value =
        bitstring_to_state(
            bitstring,
            search_qubits,
        );

    marked_states.binary_search(&value).is_ok()
}

fn bitstring_to_state(
    bitstring: &str,
    search_qubits: usize,
) -> u64 {
    let mut value = 0_u64;

    for (index, byte) in
        bitstring.bytes().enumerate()
    {
        if byte == b'1' {
            let qubit =
                index;

            // The canonical Zamani representation is q0-first, therefore
            // bitstring position i maps directly to bit i.
            value |=
                1_u64 << qubit;
        }
    }

    debug_assert!(
        search_qubits <= MAX_GROVER_SEARCH_QUBITS
    );

    value
}

// =============================================================================
// Resource estimation
// =============================================================================

fn validate_resource_limits(
    resources: &GroverResources,
) -> BenchmarkResult<()> {
    let limits =
        BenchmarkLimits::production();

    limits
        .check_qubits(
            resources.total_qubits,
        )
        .map_err(|error| {
            resource_limit_error(
                "qubits",
                error,
            )
        })?;

    limits
        .check_gate_count(
            resources.logical_gate_count,
        )
        .map_err(|error| {
            resource_limit_error(
                "gate_count",
                error,
            )
        })?;

    limits
        .check_two_qubit_gates(
            resources
                .logical_two_qubit_gate_count,
        )
        .map_err(|error| {
            resource_limit_error(
                "two_qubit_gates",
                error,
            )
        })?;

    Ok(())
}

fn mcz_gate_count(
    search_qubits: usize,
) -> BenchmarkResult<usize> {
    match search_qubits {
        0 => Err(invalid_configuration(
            "search_qubits",
            "MCZ requires at least one qubit",
        )),

        1 | 2 => Ok(1),

        n => n.checked_mul(2)
            .and_then(|value| {
                value.checked_sub(3)
            })
            .ok_or_else(|| {
                numerical_overflow(
                    "Grover MCZ gate count",
                )
            }),
    }
}

fn mcz_two_qubit_gate_count(
    search_qubits: usize,
) -> BenchmarkResult<usize> {
    match search_qubits {
        0 | 1 => Ok(0),
        2 => Ok(1),
        n => n.checked_mul(2)
            .and_then(|value| {
                value.checked_sub(4)
            })
            .ok_or_else(|| {
                numerical_overflow(
                    "Grover MCZ two-qubit gate count",
                )
            }),
    }
}

/// Conservative upper bound on zero bits in one marked state.
fn marked_state_zero_bits_upper_bound(
    search_qubits: usize,
) -> usize {
    search_qubits
}

// =============================================================================
// Deterministic state generation
// =============================================================================

fn deterministic_marked_state(
    request: &ApplicationGenerationRequest,
    search_space: u64,
) -> u64 {
    let seed =
        request.metadata().seed();

    let sequence =
        request
            .metadata()
            .sequence_index();

    // SplitMix64-style deterministic mixing. This is used only to select a
    // benchmark instance, not as a cryptographic RNG.
    let mut z =
        seed.wrapping_add(
            0x9E37_79B9_7F4A_7C15_u64
                .wrapping_mul(
                    sequence.wrapping_add(1),
                ),
        );

    z = (z ^ (z >> 30))
        .wrapping_mul(
            0xBF58_476D_1CE4_E5B9_u64,
        );

    z = (z ^ (z >> 27))
        .wrapping_mul(
            0x94D0_49BB_1331_11EB_u64,
        );

    z ^ (z >> 31) % search_space
}

// =============================================================================
// Mathematical helpers
// =============================================================================

fn search_space_size(
    search_qubits: usize,
) -> BenchmarkResult<u64> {
    if search_qubits == 0 {
        return Err(invalid_configuration(
            "problem_size",
            "Grover requires at least one search qubit",
        ));
    }

    if search_qubits > MAX_GROVER_SEARCH_QUBITS {
        return Err(invalid_configuration(
            "problem_size",
            "Grover supports at most 63 search qubits in the current benchmark representation",
        ));
    }

    1_u64
        .checked_shl(
            search_qubits as u32,
        )
        .ok_or_else(|| {
            numerical_overflow(
                "Grover search-space size",
            )
        })
}

fn validate_probability(
    field: &'static str,
    value: f64,
) -> BenchmarkResult<f64> {
    if !value.is_finite()
        || value < 0.0
        || value > 1.0
    {
        return Err(
            BenchmarkError::NumericalInstability {
                operation:
                    "grover_probability_validation"
                        .to_owned(),
                message:
                    format!(
                        "{field} must be finite and in [0,1], got {value}"
                    ),
            },
        );
    }

    Ok(value)
}

fn format_probability(
    value: f64,
) -> String {
    format!("{value:.17}")
}

fn format_marked_states(
    states: &[u64],
) -> String {
    let mut result =
        String::new();

    for (index, state) in
        states.iter().enumerate()
    {
        if index != 0 {
            result.push(',');
        }

        result.push_str(
            &state.to_string(),
        );
    }

    result
}

// =============================================================================
// Benchmark error helpers
// =============================================================================

fn invalid_configuration(
    field: &'static str,
    reason: &'static str,
) -> BenchmarkError {
    BenchmarkError::InvalidConfiguration {
        field: field.to_owned(),
        reason: reason.to_owned(),
    }
}

fn invalid_workload(
    reason: &'static str,
    error: impl fmt::Display,
) -> BenchmarkError {
    BenchmarkError::InvalidWorkload {
        workload:
            GROVER_APPLICATION_ID.to_owned(),
        reason:
            format!("{reason}: {error}"),
    }
}

fn workload_error(
    reason: &'static str,
    error: WorkloadError,
) -> BenchmarkError {
    BenchmarkError::InvalidWorkload {
        workload:
            GROVER_APPLICATION_ID.to_owned(),
        reason:
            format!("{reason}: {error}"),
    }
}

fn circuit_error(
    reason: &'static str,
    error: impl fmt::Display,
) -> BenchmarkError {
    BenchmarkError::InvalidWorkload {
        workload:
            GROVER_APPLICATION_ID.to_owned(),
        reason:
            format!("{reason}: {error}"),
    }
}

fn numerical_overflow(
    operation: &'static str,
) -> BenchmarkError {
    BenchmarkError::NumericalOverflow {
        operation:
            operation.to_owned(),
        value: None,
    }
}

fn resource_limit_error(
    resource: &'static str,
    error: impl fmt::Display,
) -> BenchmarkError {
    BenchmarkError::ResourceLimitExceeded {
        resource:
            resource.to_owned(),
        requested:
            0,
        maximum:
            0,
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn request(
        problem_size: usize,
    ) -> ApplicationGenerationRequest {
        ApplicationGenerationRequest::new(
            GROVER_APPLICATION_ID,
            WorkloadId::new(
                "instance_0",
            )
            .expect(
                "test workload ID must be valid",
            ),
            problem_size,
            42,
        )
        .expect(
            "test request must be valid",
        )
        .with_generator_revision(
            GROVER_GENERATOR_REVISION,
        )
    }

    fn request_with_parameters(
        problem_size: usize,
        parameters: &[(&str, &str)],
    ) -> ApplicationGenerationRequest {
        let mut result =
            request(problem_size);

        for &(name, value) in
            parameters
        {
            result = result
                .with_parameter(
                    ApplicationParameter::new(
                        name,
                        value,
                    )
                    .expect(
                        "test parameter must be valid",
                    ),
                )
                .expect(
                    "test parameter must be accepted",
                );
        }

        result
    }

    #[test]
    fn problem_validates_single_marked_state() {
        let problem =
            GroverProblem::new(
                2,
                vec![1],
            )
            .expect(
                "two-qubit Grover problem should be valid",
            );

        assert_eq!(
            problem.search_space_size()
                .expect("search-space size"),
            4
        );

        assert_eq!(
            problem.solution_count(),
            1
        );
    }

    #[test]
    fn problem_rejects_zero_qubits() {
        assert!(
            GroverProblem::new(
                0,
                vec![0],
            )
            .is_err()
        );
    }

    #[test]
    fn problem_rejects_out_of_range_state() {
        assert!(
            GroverProblem::new(
                3,
                vec![8],
            )
            .is_err()
        );
    }

    #[test]
    fn problem_rejects_duplicate_states() {
        assert!(
            GroverProblem::new(
                3,
                vec![2, 2],
            )
            .is_err()
        );
    }

    #[test]
    fn all_states_require_zero_iterations() {
        let problem =
            GroverProblem::new(
                2,
                vec![0, 1, 2, 3],
            )
            .expect(
                "all states should be valid",
            );

        assert_eq!(
            problem
                .optimal_iterations()
                .expect(
                    "all-state Grover optimum"
                ),
            0
        );

        assert!(
            (
                problem
                    .ideal_success_probability(
                        0,
                    )
                    .expect(
                        "ideal probability"
                    )
                    - 1.0
            )
            .abs()
                < 1.0e-12
        );
    }

    #[test]
    fn two_qubit_single_solution_has_one_optimal_iteration() {
        let problem =
            GroverProblem::new(
                2,
                vec![1],
            )
            .expect(
                "two-qubit Grover problem",
            );

        assert_eq!(
            problem
                .optimal_iterations()
                .expect(
                    "optimal iterations"
                ),
            1
        );

        assert!(
            (
                problem
                    .ideal_success_probability(
                        1,
                    )
                    .expect(
                        "ideal probability"
                    )
                    - 1.0
            )
            .abs()
                < 1.0e-12
        );
    }

    #[test]
    fn classical_baseline_is_inverse_search_space() {
        let problem =
            GroverProblem::new(
                3,
                vec![5],
            )
            .expect(
                "three-qubit Grover problem",
            );

        let baseline =
            problem
                .classical_baseline()
                .expect(
                    "classical baseline"
                );

        assert!(
            (
                baseline
                    - 0.125
            )
            .abs()
                < 1.0e-12
        );
    }

    #[test]
    fn explicit_marked_state_is_parsed() {
        let generator =
            GroverBenchmarkGenerator::new()
                .expect(
                    "generator should construct"
                );

        let request =
            request_with_parameters(
                3,
                &[("marked_state", "5")],
            );

        let problem =
            generator
                .problem_from_request(
                    &request,
                )
                .expect(
                    "marked state should parse"
                );

        assert_eq!(
            problem.marked_states,
            vec![5]
        );
    }

    #[test]
    fn marked_state_default_is_deterministic() {
        let generator =
            GroverBenchmarkGenerator::new()
                .expect(
                    "generator should construct"
                );

        let first =
            generator
                .problem_from_request(
                    &request(5),
                )
                .expect(
                    "first problem"
                );

        let second =
            generator
                .problem_from_request(
                    &request(5),
                )
                .expect(
                    "second problem"
                );

        assert_eq!(
            first,
            second
        );
    }

    #[test]
    fn multiple_marked_states_are_supported() {
        let generator =
            GroverBenchmarkGenerator::new()
                .expect(
                    "generator should construct"
                );

        let request =
            request_with_parameters(
                4,
                &[
                    ("marked_states", "1,4,9"),
                ],
            );

        let problem =
            generator
                .problem_from_request(
                    &request,
                )
                .expect(
                    "marked states"
                );

        assert_eq!(
            problem.marked_states,
            vec![1, 4, 9]
        );
    }

    #[test]
    fn invalid_parameter_is_rejected() {
        let generator =
            GroverBenchmarkGenerator::new()
                .expect(
                    "generator should construct"
                );

        let request =
            request_with_parameters(
                4,
                &[("unknown", "1")],
            );

        assert!(
            generator
                .validate(
                    &request
                )
                .is_err()
        );
    }

    #[test]
    fn zero_iterations_is_allowed() {
        let generator =
            GroverBenchmarkGenerator::new()
                .expect(
                    "generator should construct"
                );

        let request =
            request_with_parameters(
                2,
                &[("iterations", "0")],
            );

        let description =
            generator
                .describe(
                    &request,
                )
                .expect(
                    "explicit zero iterations"
                );

        assert_eq!(
            description
                .iterations,
            0
        );
    }

    #[test]
    fn circuit_generation_produces_valid_ir() {
        let generator =
            GroverBenchmarkGenerator::new()
                .expect(
                    "generator should construct"
                );

        let request =
            request_with_parameters(
                2,
                &[("marked_state", "1")],
            );

        let circuit =
            generator
                .generate_circuit(
                    &request,
                )
                .expect(
                    "Grover circuit should generate"
                );

        circuit
            .validate()
            .expect(
                "generated circuit must validate"
            );
    }

    #[test]
    fn three_qubit_circuit_uses_reusable_ancilla() {
        let generator =
            GroverBenchmarkGenerator::new()
                .expect(
                    "generator should construct"
                );

        let request =
            request_with_parameters(
                3,
                &[("marked_state", "5")],
            );

        let description =
            generator
                .describe(
                    &request,
                )
                .expect(
                    "description"
                );

        assert_eq!(
            description
                .resources
                .ancilla_qubits,
            1
        );

        let circuit =
            generator
                .generate_circuit(
                    &request,
                )
                .expect(
                    "three-qubit circuit"
                );

        circuit
            .validate()
            .expect(
                "three-qubit circuit must validate"
            );
    }

    #[test]
    fn ideal_probability_matches_counts_analysis() {
        let generator =
            GroverBenchmarkGenerator::new()
                .expect(
                    "generator should construct"
                );

        let request =
            request_with_parameters(
                2,
                &[("marked_state", "1")],
            );

        let mut counts =
            BTreeMap::new();

        counts.insert(
            "01".to_owned(),
            100,
        );

        let result =
            generator
                .analyze_counts(
                    &request,
                    &counts,
                )
                .expect(
                    "counts should analyze"
                );

        assert_eq!(
            result.shots,
            100
        );

        assert!(
            (
                result
                    .observed_success_probability
                    - 1.0
            )
            .abs()
                < 1.0e-12
        );

        assert!(
            (
                result
                    .ideal_success_probability
                    - 1.0
            )
            .abs()
                < 1.0e-12
        );
    }

    #[test]
    fn analysis_rejects_invalid_bitstring() {
        let generator =
            GroverBenchmarkGenerator::new()
                .expect(
                    "generator should construct"
                );

        let request =
            request(3);

        let mut counts =
            BTreeMap::new();

        counts.insert(
            "0101".to_owned(),
            1,
        );

        assert!(
            generator
                .analyze_counts(
                    &request,
                    &counts,
                )
                .is_err()
        );
    }

    #[test]
    fn analysis_counts_multiple_marked_states() {
        let generator =
            GroverBenchmarkGenerator::new()
                .expect(
                    "generator should construct"
                );

        let request =
            request_with_parameters(
                2,
                &[(
                    "marked_states",
                    "1,2",
                )],
            );

        let mut counts =
            BTreeMap::new();

        counts.insert(
            "01".to_owned(),
            60,
        );

        counts.insert(
            "10".to_owned(),
            30,
        );

        counts.insert(
            "00".to_owned(),
            10,
        );

        let result =
            generator
                .analyze_counts(
                    &request,
                    &counts,
                )
                .expect(
                    "multi-solution analysis"
                );

        assert!(
            (
                result
                    .observed_success_probability
                    - 0.9
            )
            .abs()
                < 1.0e-12
        );

        assert_eq!(
            result.marked_state_shots,
            90
        );
    }

    #[test]
    fn generator_is_idempotent_for_same_request() {
        let generator =
            GroverBenchmarkGenerator::new()
                .expect(
                    "generator should construct"
                );

        let request =
            request_with_parameters(
                2,
                &[("marked_state", "3")],
            );

        let first =
            generator
                .generate_application_workload(
                    &request,
                )
                .expect(
                    "first workload"
                );

        let second =
            generator
                .generate_application_workload(
                    &request,
                )
                .expect(
                    "second workload"
                );

        assert_eq!(
            first.application_id(),
            second.application_id()
        );

        assert_eq!(
            first.problem_size(),
            second.problem_size()
        );

        assert_eq!(
            first.parameters(),
            second.parameters()
        );

        assert!(
            first.circuit().is_some()
        );

        assert!(
            second.circuit().is_some()
        );
    }
}