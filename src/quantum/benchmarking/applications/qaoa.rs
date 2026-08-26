//! Zamani Quantum Benchmarking — QAOA / MaxCut Application Benchmark.
//!
//! Production-grade, backend-independent benchmark generator and analyzer for
//! the Quantum Approximate Optimization Algorithm (QAOA) applied to weighted
//! MaxCut.
//!
//! # Responsibility
//!
//! This module owns:
//!
//! - the QAOA/MaxCut benchmark identity;
//! - MaxCut graph/problem validation;
//! - weighted positive-edge representation;
//! - deterministic graph-family generation;
//! - explicit QAOA angle schedules;
//! - deterministic Linear-Ramp QAOA (LR-QAOA) schedules;
//! - QAOA circuit generation;
//! - canonical Quantum IR construction;
//! - deterministic logical-resource accounting;
//! - bounded exact classical reference calculation;
//! - measurement-count analysis;
//! - approximation-ratio calculation;
//! - effective approximation-ratio calculation;
//! - random-baseline calculation;
//! - optimal-solution probability;
//! - best-observed solution analysis;
//! - application workload construction;
//! - application metadata;
//! - production validation;
//! - deterministic unit tests.
//!
//! This module deliberately does NOT own:
//!
//! - classical parameter optimization;
//! - backend selection;
//! - backend execution;
//! - transpilation;
//! - routing;
//! - physical topology;
//! - scheduling;
//! - pulse generation;
//! - calibration;
//! - simulator implementation;
//! - vendor SDKs;
//! - QPU communication;
//! - error correction;
//! - persistence;
//! - universal benchmark-result serialization;
//! - universal timing metrics.
//!
//! Those responsibilities belong to the surrounding algorithm,
//! benchmarking, compiler, runtime, hardware, QEC, and reporting layers.
//!
//! # Architectural position
//!
//! ```text
//! ApplicationGenerationRequest
//!             │
//!             ▼
//!     QaoaBenchmarkGenerator
//!             │
//!       ┌─────┴──────┐
//!       ▼            ▼
//!   validation    generation
//!       │            │
//!       │            ▼
//!       │      QuantumCircuit
//!       │            │
//!       │            ▼
//!       │    ApplicationWorkload
//!       │            │
//!       └──────┬─────┘
//!              ▼
//!       BenchmarkExperiment
//!              │
//!              ▼
//!       BenchmarkExecutor
//!              │
//!              ▼
//!       normalized counts
//!              │
//!              ▼
//!       analyze_counts()
//!              │
//!              ▼
//!       QaoaBenchmarkResult
//! ```
//!
//! # Relationship with quantum::algorithms::qaoa
//!
//! The existing QAOA algorithm module owns:
//!
//! - QAOA problem orchestration;
//! - optimizer integration;
//! - expectation-value objective execution;
//! - final solution measurement;
//! - algorithm-level execution metadata.
//!
//! This benchmark module must NOT reimplement those responsibilities.
//!
//! Instead:
//!
//! ```text
//! benchmarking::applications::qaoa
//!              │
//!              ├── generates benchmark workload
//!              └── analyzes benchmark observations
//!
//! quantum::algorithms::qaoa
//!              │
//!              └── performs variational optimization when requested
//! ```
//!
//! A future benchmark runner may therefore support both:
//!
//! 1. deterministic/non-variational LR-QAOA benchmarking;
//! 2. optimizer-driven variational QAOA benchmarking;
//!
//! without changing this file's workload/result contract.
//!
//! # Scientific benchmark semantics
//!
//! For a weighted undirected graph G=(V,E), the MaxCut objective is:
//!
//! ```text
//! C(x) = Σ_(i,j in E) w_ij * (1 - z_i z_j) / 2
//! ```
//!
//! where:
//!
//! ```text
//! z_i = +1 for bit 0
//! z_i = -1 for bit 1
//! ```
//!
//! Equivalently, for a computational-basis bit string:
//!
//! ```text
//! edge contributes w_ij when the endpoint bits differ;
//! edge contributes 0 when they are equal.
//! ```
//!
//! QAOA prepares:
//!
//! ```text
//! |ψ_p(γ,β)> =
//!   U_B(β_p) U_C(γ_p) ... U_B(β_1) U_C(γ_1) |+>^n
//! ```
//!
//! with the standard X mixer:
//!
//! ```text
//! H_B = Σ_i X_i
//! U_B(β) = Π_i RX(2β_i)
//! ```
//!
//! For the MaxCut cost Hamiltonian:
//!
//! ```text
//! H_C = Σ_(i,j) w_ij (I - Z_i Z_j) / 2
//! ```
//!
//! the edge-dependent phase can be implemented, up to an irrelevant global
//! phase, by:
//!
//! ```text
//! CX(i,j)
//! RZ(-γ w_ij) on j
//! CX(i,j)
//! ```
//!
//! because:
//!
//! ```text
//! CX RZ(θ) CX = exp(-i θ Z_i Z_j / 2)
//! ```
//!
//! and choosing:
//!
//! ```text
//! θ = -γ w_ij
//! ```
//!
//! produces the required ZZ component of the MaxCut cost evolution.
//!
//! The implementation records this convention explicitly in workload
//! metadata. This is essential because QAOA literature contains multiple
//! equivalent sign/constant conventions for MaxCut Hamiltonians.
//!
//! # Angle schedules
//!
//! Two schedules are supported.
//!
//! ## Explicit
//!
//! The caller supplies exactly p gamma values and p beta values.
//!
//! ## Linear-ramp QAOA
//!
//! The deterministic schedule is:
//!
//! ```text
//! beta_i  = (1 - i/p) * delta_beta
//! gamma_i = ((i + 1)/p) * delta_gamma
//! ```
//!
//! for:
//!
//! ```text
//! i = 0 .. p-1
//! ```
//!
//! The production default is:
//!
//! ```text
//! delta_beta  = 0.3
//! delta_gamma = 0.6
//! ```
//!
//! These values are a benchmark schedule, not universal QAOA-optimal
//! parameters. They are recorded explicitly in workload metadata.
//!
//! This deterministic schedule is especially useful for cross-device
//! benchmarking because it avoids introducing an optimizer-dependent
//! classical overhead into the benchmark itself.
//!
//! # Benchmark metrics
//!
//! Given normalized computational-basis counts, the analyzer computes:
//!
//! - total shots;
//! - observed mean cut value;
//! - optimal cut value when an exact reference is available;
//! - raw approximation ratio;
//! - random baseline approximation ratio;
//! - effective approximation ratio;
//! - best sampled cut value;
//! - best sampled approximation ratio;
//! - probability of sampling an optimal solution when the exact optimum is
//!   available;
//! - probability of sampling a solution at or above a requested approximation
//!   threshold;
//! - count of optimal samples;
//! - count of above-threshold samples.
//!
//! The benchmark deliberately does not invent an exact optimum for large
//! instances. When exact classical verification is outside the configured
//! bounded reference domain, the result explicitly reports that the optimum
//! is unavailable rather than silently using an approximation as ground truth.
//!
//! # Exact classical reference
//!
//! Exact MaxCut enumeration is exponential. Therefore this file has a strict
//! production reference bound:
//!
//! ```text
//! MAX_EXACT_REFERENCE_QUBITS
//! ```
//!
//! The exact solver:
//!
//! - never materializes all bit strings;
//! - never allocates a 2^n vector;
//! - streams through candidate assignments;
//! - uses checked arithmetic;
//! - is disabled automatically above the reference bound.
//!
//! This is a verification facility, not the benchmark execution path.
//!
//! # Bit-string convention
//!
//! Zamani application benchmark analysis uses:
//!
//! ```text
//! bitstring[0] == logical qubit q0 == classical bit c0
//! ```
//!
//! The execution layer must normalize backend-native bit ordering before
//! calling `analyze_counts()`.
//!
//! This file never silently reverses backend bit strings.
//!
//! # Resource safety
//!
//! Requests are untrusted input.
//!
//! The implementation:
//!
//! - validates the common generation request before allocation;
//! - validates application parameters before circuit allocation;
//! - bounds qubit count;
//! - bounds QAOA depth;
//! - bounds graph edges;
//! - bounds exact-reference enumeration;
//! - checks total logical gate count;
//! - checks total two-qubit gate count;
//! - uses checked arithmetic;
//! - rejects non-finite weights;
//! - rejects non-positive weights;
//! - rejects self-loops;
//! - rejects duplicate edges;
//! - rejects malformed graph definitions;
//! - rejects malformed angle vectors;
//! - rejects non-finite angles;
//! - validates every generated Gate;
//! - validates the complete QuantumCircuit;
//! - never performs I/O;
//! - never executes user-provided code.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021
//!
//! No nightly features.
//! No external dependencies.
//!
//! # Integration contract
//!
//! Existing contracts consumed by this file:
//!
//! ```text
//! benchmarking::generators::application
//!     ├── ApplicationBenchmarkGenerator
//!     ├── ApplicationGenerationRequest
//!     ├── ApplicationGeneratorCapability
//!     └── ApplicationGeneratorDescriptor
//!
//! benchmarking::core::workload
//!     ├── ApplicationParameter
//!     ├── ApplicationWorkload
//!     ├── CircuitWorkload
//!     ├── WorkloadError
//!     └── WorkloadId
//!
//! benchmarking::core::limits
//!     └── BenchmarkLimits
//!
//! quantum::ir
//!     ├── QuantumCircuit
//!     ├── Gate
//!     ├── GateKind
//!     ├── Parameter
//!     ├── Measurement
//!     ├── QubitId
//!     └── ClassicalBitId
//! ```
//!
//! The only namespace integration required after this file exists is:
//!
//! ```text
//! src/quantum/benchmarking/applications/mod.rs
//!
//! pub mod qaoa;
//! ```
//!
//! No QAOA algorithm file, runtime file, backend file, or IR file needs to be
//! edited to complete this generator.
//!
//! # Request parameters
//!
//! Required:
//!
//! ```text
//! none
//! ```
//!
//! Defaults:
//!
//! ```text
//! graph = ring
//! schedule = linear_ramp
//! p = 10
//! delta_beta = 0.3
//! delta_gamma = 0.6
//! mixer = x
//! approximation_threshold = 1.0
//! ```
//!
//! Optional:
//!
//! ```text
//! graph = path | ring | complete | custom
//! edges = 0-1:1.0,1-2:1.0,...
//! p = positive integer
//! schedule = linear_ramp | explicit
//! delta_beta = finite floating-point value
//! delta_gamma = finite floating-point value
//! gamma = g1,g2,...,gp
//! beta = b1,b2,...,bp
//! mixer = x
//! approximation_threshold = finite value in [0,1]
//! ```
//!
//! `edges` is required when `graph=custom`.
//!
//! For built-in graphs, `edges` must not be supplied.
//!
//! For `schedule=linear_ramp`, explicit `gamma` and `beta` are rejected.
//!
//! For `schedule=explicit`, both `gamma` and `beta` are required and each
//! vector must contain exactly p finite values.
//!
//! # Current benchmark scope
//!
//! This file intentionally implements the standard X-mixer MaxCut QAOA.
//!
//! It does not pretend that every QAOA variant is the same algorithm.
//!
//! Future files can add:
//!
//! - XY mixer;
//! - constrained mixers;
//! - warm-start QAOA;
//! - multi-angle QAOA;
//! - CVaR-QAOA;
//! - recursive QAOA;
//! - adaptive-depth QAOA;
//! - custom alternating-operator ansatzes;
//! - QRAO.
//!
//! Those variants should receive distinct benchmark identifiers or explicit
//! protocol/version identifiers rather than silently changing this benchmark.
//!
//! # References
//!
//! The benchmark design follows the standard QAOA/MaxCut formulation and
//! current application-level benchmarking practice, including deterministic
//! LR-QAOA benchmarking, approximation ratio, random baselines, and
//! problem-size/depth scaling.
//!
//! No network access is performed by this module.
//!
//! # Safety
//!
//! This module contains no unsafe code.
//!
//! -----------------------------------------------------------------------------
//! Implementation
//! -----------------------------------------------------------------------------

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
    parameter::Parameter,
    qubit::QubitId,
    QuantumCircuit,
};

// =============================================================================
// Stable identity and versions
// =============================================================================

/// Stable benchmark identifier.
pub const QAOA_BENCHMARK_ID: &str = "qaoa";

/// Stable application identifier.
pub const QAOA_APPLICATION_ID: &str = "qaoa";

/// Human-readable benchmark name.
pub const QAOA_NAME: &str = "Quantum Approximate Optimization Algorithm";

/// Generator implementation version.
pub const QAOA_GENERATOR_VERSION: &str = "1.0.0";

/// Reproducibility revision.
pub const QAOA_GENERATOR_REVISION: u32 = 1;

/// Application result schema version.
pub const QAOA_RESULT_SCHEMA_VERSION: u16 = 1;

/// Maximum QAOA depth accepted by this application benchmark.
pub const MAX_QAOA_DEPTH: usize = 400;

/// Maximum logical qubits accepted by the application benchmark.
///
/// The universal benchmarking limit remains authoritative. This local bound
/// prevents pathological QAOA metadata from creating enormous angle vectors or
/// graph structures before the universal limit is consulted.
pub const MAX_QAOA_QUBITS: usize = 4_096;

/// Maximum number of graph edges represented by this generator.
pub const MAX_QAOA_EDGES: usize = 10_000_000;

/// Maximum number of bytes accepted for the custom edge-list parameter.
pub const MAX_EDGE_PARAMETER_BYTES: usize = 1_000_000;

/// Maximum number of qubits for exact classical MaxCut verification.
///
/// This is intentionally much smaller than the circuit-generation limit.
pub const MAX_EXACT_REFERENCE_QUBITS: usize = 20;

/// Default QAOA depth for the deterministic LR-QAOA benchmark.
pub const DEFAULT_QAOA_DEPTH: usize = 10;

/// Default LR-QAOA beta slope.
pub const DEFAULT_DELTA_BETA: f64 = 0.3;

/// Default LR-QAOA gamma slope.
pub const DEFAULT_DELTA_GAMMA: f64 = 0.6;

/// Default approximation threshold.
///
/// `1.0` means only an optimal solution qualifies when an exact optimum is
/// available.
pub const DEFAULT_APPROXIMATION_THRESHOLD: f64 = 1.0;

/// Maximum number of bytes accepted for any one QAOA application parameter.
pub const MAX_QAOA_PARAMETER_VALUE_BYTES: usize = 1_024;

// =============================================================================
// Graph model
// =============================================================================

/// One weighted undirected MaxCut edge.
///
/// Edge endpoints are canonicalized so:
///
/// ```text
/// u < v
/// ```
///
/// Positive finite weights are required.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct QaoaEdge {
    /// First logical vertex.
    pub u: usize,

    /// Second logical vertex.
    pub v: usize,

    /// Positive finite edge weight.
    pub weight: f64,
}

impl QaoaEdge {
    /// Creates and validates an edge.
    pub fn new(
        u: usize,
        v: usize,
        weight: f64,
    ) -> BenchmarkResult<Self> {
        if u == v {
            return Err(invalid_configuration(
                "edges",
                "MaxCut does not permit self-loop edges",
            ));
        }

        if !weight.is_finite() {
            return Err(invalid_configuration(
                "edges",
                "MaxCut edge weight must be finite",
            ));
        }

        if weight <= 0.0 {
            return Err(invalid_configuration(
                "edges",
                "this benchmark requires strictly positive edge weights",
            ));
        }

        let (u, v) = if u < v { (u, v) } else { (v, u) };

        Ok(Self { u, v, weight })
    }

    /// Returns the canonical endpoint pair.
    #[must_use]
    pub const fn endpoints(self) -> (usize, usize) {
        (self.u, self.v)
    }
}

/// Weighted undirected MaxCut graph.
#[derive(Debug, Clone, PartialEq)]
pub struct QaoaGraph {
    /// Number of vertices.
    pub qubits: usize,

    /// Canonically ordered edges.
    pub edges: Vec<QaoaEdge>,
}

impl QaoaGraph {
    /// Creates and validates a weighted graph.
    pub fn new(
        qubits: usize,
        edges: Vec<QaoaEdge>,
    ) -> BenchmarkResult<Self> {
        if qubits < 2 {
            return Err(invalid_configuration(
                "problem_size",
                "QAOA MaxCut requires at least two vertices",
            ));
        }

        if qubits > MAX_QAOA_QUBITS {
            return Err(invalid_configuration(
                "problem_size",
                "QAOA problem size exceeds the application benchmark limit",
            ));
        }

        if edges.is_empty() {
            return Err(invalid_configuration(
                "edges",
                "QAOA MaxCut requires at least one edge",
            ));
        }

        if edges.len() > MAX_QAOA_EDGES {
            return Err(BenchmarkError::ResourceLimitExceeded {
                resource: "qaoa_edges".to_owned(),
                requested: edges.len() as u64,
                maximum: MAX_QAOA_EDGES as u64,
            });
        }

        let maximum_possible_edges = qubits
            .checked_mul(qubits.saturating_sub(1))
            .ok_or_else(|| numerical_overflow("maximum graph edge count"))?
            / 2;

        if edges.len() > maximum_possible_edges {
            return Err(invalid_configuration(
                "edges",
                "graph contains more edges than a simple undirected graph permits",
            ));
        }

        let mut seen = BTreeSet::new();

        for edge in &edges {
            if edge.u >= qubits || edge.v >= qubits {
                return Err(invalid_configuration(
                    "edges",
                    "edge endpoint is outside the graph vertex range",
                ));
            }

            if edge.u >= edge.v {
                return Err(invalid_configuration(
                    "edges",
                    "edges must be canonicalized with u < v",
                ));
            }

            if !edge.weight.is_finite() || edge.weight <= 0.0 {
                return Err(invalid_configuration(
                    "edges",
                    "edge weight must be finite and strictly positive",
                ));
            }

            if !seen.insert((edge.u, edge.v)) {
                return Err(invalid_configuration(
                    "edges",
                    "graph contains duplicate edges",
                ));
            }
        }

        Ok(Self { qubits, edges })
    }

    /// Constructs an unweighted path graph.
    pub fn path(qubits: usize) -> BenchmarkResult<Self> {
        if qubits < 2 {
            return Err(invalid_configuration(
                "problem_size",
                "path graph requires at least two vertices",
            ));
        }

        let mut edges = Vec::with_capacity(qubits - 1);

        for vertex in 0..(qubits - 1) {
            edges.push(QaoaEdge::new(
                vertex,
                vertex + 1,
                1.0,
            )?);
        }

        Self::new(qubits, edges)
    }

    /// Constructs an unweighted ring graph.
    pub fn ring(qubits: usize) -> BenchmarkResult<Self> {
        if qubits < 2 {
            return Err(invalid_configuration(
                "problem_size",
                "ring graph requires at least two vertices",
            ));
        }

        let edge_count = if qubits == 2 {
            1
        } else {
            qubits
        };

        let mut edges =
            Vec::with_capacity(edge_count);

        for vertex in 0..qubits {
            let next = (vertex + 1) % qubits;

            if vertex == next {
                continue;
            }

            let (u, v) =
                if vertex < next {
                    (vertex, next)
                } else {
                    (next, vertex)
                };

            if edges
                .iter()
                .any(|edge: &QaoaEdge| {
                    edge.u == u && edge.v == v
                })
            {
                continue;
            }

            edges.push(QaoaEdge::new(
                u,
                v,
                1.0,
            )?);
        }

        Self::new(qubits, edges)
    }

    /// Constructs an unweighted complete graph.
    pub fn complete(qubits: usize) -> BenchmarkResult<Self> {
        if qubits < 2 {
            return Err(invalid_configuration(
                "problem_size",
                "complete graph requires at least two vertices",
            ));
        }

        let edge_count = qubits
            .checked_mul(qubits - 1)
            .ok_or_else(|| {
                numerical_overflow(
                    "complete graph edge count",
                )
            })?
            / 2;

        if edge_count > MAX_QAOA_EDGES {
            return Err(BenchmarkError::ResourceLimitExceeded {
                resource: "qaoa_edges".to_owned(),
                requested: edge_count as u64,
                maximum: MAX_QAOA_EDGES as u64,
            });
        }

        let mut edges =
            Vec::with_capacity(edge_count);

        for u in 0..qubits {
            for v in (u + 1)..qubits {
                edges.push(QaoaEdge::new(
                    u,
                    v,
                    1.0,
                )?);
            }
        }

        Self::new(qubits, edges)
    }

    /// Returns the total edge weight.
    pub fn total_weight(&self) -> BenchmarkResult<f64> {
        let mut total = 0.0;

        for edge in &self.edges {
            total += edge.weight;

            if !total.is_finite() {
                return Err(numerical_overflow(
                    "total graph weight",
                ));
            }
        }

        Ok(total)
    }

    /// Evaluates the MaxCut objective for one bit string.
    ///
    /// `bit i = 0` and `bit i = 1` represent the two partitions.
    pub fn cut_value(
        &self,
        bits: &str,
    ) -> BenchmarkResult<f64> {
        validate_bitstring(
            bits,
            self.qubits,
        )?;

        let bytes = bits.as_bytes();
        let mut value = 0.0;

        for edge in &self.edges {
            if bytes[edge.u] != bytes[edge.v] {
                value += edge.weight;

                if !value.is_finite() {
                    return Err(numerical_overflow(
                        "MaxCut bitstring value",
                    ));
                }
            }
        }

        Ok(value)
    }

    /// Returns the expected cut value of uniformly random bit strings.
    ///
    /// Every positive-weight edge is cut with probability 1/2.
    pub fn random_expected_cut(&self) -> BenchmarkResult<f64> {
        Ok(self.total_weight()? * 0.5)
    }

    /// Returns the number of logical cost-layer edge blocks.
    pub fn cost_edge_count(&self) -> usize {
        self.edges.len()
    }
}

// =============================================================================
// Angle schedules
// =============================================================================

/// QAOA parameter schedule.
#[derive(Debug, Clone, PartialEq)]
pub enum QaoaAngleSchedule {
    /// Explicit gamma/beta vectors.
    Explicit {
        /// Cost angles.
        gamma: Vec<f64>,

        /// Mixer angles.
        beta: Vec<f64>,
    },

    /// Deterministic linear-ramp schedule.
    LinearRamp {
        /// Final gamma slope.
        delta_gamma: f64,

        /// Initial beta slope.
        delta_beta: f64,
    },
}

impl QaoaAngleSchedule {
    /// Returns a production LR-QAOA default.
    #[must_use]
    pub const fn default_linear_ramp() -> Self {
        Self::LinearRamp {
            delta_gamma: DEFAULT_DELTA_GAMMA,
            delta_beta: DEFAULT_DELTA_BETA,
        }
    }

    /// Validates the schedule for a QAOA depth.
    pub fn validate(
        &self,
        depth: usize,
    ) -> BenchmarkResult<()> {
        if depth == 0 {
            return Err(invalid_configuration(
                "p",
                "QAOA depth must be greater than zero",
            ));
        }

        match self {
            Self::Explicit { gamma, beta } => {
                if gamma.len() != depth {
                    return Err(invalid_configuration(
                        "gamma",
                        "explicit gamma vector length must equal p",
                    ));
                }

                if beta.len() != depth {
                    return Err(invalid_configuration(
                        "beta",
                        "explicit beta vector length must equal p",
                    ));
                }

                for &value in gamma {
                    if !value.is_finite() {
                        return Err(invalid_configuration(
                            "gamma",
                            "all gamma values must be finite",
                        ));
                    }
                }

                for &value in beta {
                    if !value.is_finite() {
                        return Err(invalid_configuration(
                            "beta",
                            "all beta values must be finite",
                        ));
                    }
                }
            }

            Self::LinearRamp {
                delta_gamma,
                delta_beta,
            } => {
                if !delta_gamma.is_finite()
                    || !delta_beta.is_finite()
                {
                    return Err(invalid_configuration(
                        "delta_gamma/delta_beta",
                        "LR-QAOA slope values must be finite",
                    ));
                }
            }
        }

        Ok(())
    }

    /// Materializes exactly p gamma/beta values.
    ///
    /// This allocation is bounded by MAX_QAOA_DEPTH because callers validate
    /// depth before invoking this function.
    pub fn angles(
        &self,
        depth: usize,
    ) -> BenchmarkResult<(Vec<f64>, Vec<f64>)> {
        self.validate(depth)?;

        match self {
            Self::Explicit { gamma, beta } => {
                Ok((gamma.clone(), beta.clone()))
            }

            Self::LinearRamp {
                delta_gamma,
                delta_beta,
            } => {
                let denominator =
                    depth as f64;

                let mut gamma =
                    Vec::with_capacity(depth);
                let mut beta =
                    Vec::with_capacity(depth);

                for index in 0..depth {
                    let index_f =
                        index as f64;

                    let gamma_value =
                        ((index_f + 1.0)
                            / denominator)
                            * *delta_gamma;

                    let beta_value =
                        (1.0
                            - index_f
                                / denominator)
                            * *delta_beta;

                    if !gamma_value.is_finite()
                        || !beta_value.is_finite()
                    {
                        return Err(
                            numerical_overflow(
                                "QAOA angle schedule",
                            ),
                        );
                    }

                    gamma.push(gamma_value);
                    beta.push(beta_value);
                }

                Ok((gamma, beta))
            }
        }
    }

    /// Stable schedule identifier.
    #[must_use]
    pub const fn kind_id(&self) -> &'static str {
        match self {
            Self::Explicit { .. } => "explicit",
            Self::LinearRamp { .. } => "linear_ramp",
        }
    }
}

// =============================================================================
// Typed problem
// =============================================================================

/// Complete typed QAOA/MaxCut benchmark problem.
#[derive(Debug, Clone, PartialEq)]
pub struct QaoaProblem {
    /// Weighted MaxCut graph.
    pub graph: QaoaGraph,

    /// QAOA depth.
    pub depth: usize,

    /// Angle schedule.
    pub schedule: QaoaAngleSchedule,

    /// Approximation-ratio threshold used by result analysis.
    pub approximation_threshold: f64,
}

impl QaoaProblem {
    /// Creates and validates a QAOA benchmark problem.
    pub fn new(
        graph: QaoaGraph,
        depth: usize,
        schedule: QaoaAngleSchedule,
        approximation_threshold: f64,
    ) -> BenchmarkResult<Self> {
        if depth == 0 {
            return Err(invalid_configuration(
                "p",
                "QAOA depth must be greater than zero",
            ));
        }

        if depth > MAX_QAOA_DEPTH {
            return Err(BenchmarkError::ResourceLimitExceeded {
                resource: "qaoa_depth".to_owned(),
                requested: depth as u64,
                maximum: MAX_QAOA_DEPTH as u64,
            });
        }

        if !approximation_threshold.is_finite()
            || !(0.0..=1.0)
                .contains(&approximation_threshold)
        {
            return Err(invalid_configuration(
                "approximation_threshold",
                "approximation threshold must be finite and within [0,1]",
            ));
        }

        schedule.validate(depth)?;

        Ok(Self {
            graph,
            depth,
            schedule,
            approximation_threshold,
        })
    }

    /// Returns the materialized gamma/beta vectors.
    pub fn angles(
        &self,
    ) -> BenchmarkResult<(Vec<f64>, Vec<f64>)> {
        self.schedule.angles(self.depth)
    }

    /// Returns the logical gate count before circuit allocation.
    ///
    /// Per QAOA layer:
    ///
    /// ```text
    /// 3 * |E| + n
    /// ```
    ///
    /// because every cost edge is:
    ///
    /// ```text
    /// CX + RZ + CX
    /// ```
    ///
    /// and every qubit receives one RX mixer gate.
    ///
    /// Plus:
    ///
    /// ```text
    /// n H gates
    /// n measurement gates
    /// ```
    pub fn logical_gate_count(
        &self,
    ) -> BenchmarkResult<usize> {
        let edge_gates =
            self.graph
                .edges
                .len()
                .checked_mul(3)
                .ok_or_else(|| {
                    numerical_overflow(
                        "QAOA cost-layer gate count",
                    )
                })?;

        let layer_gates =
            edge_gates
                .checked_add(self.graph.qubits)
                .ok_or_else(|| {
                    numerical_overflow(
                        "QAOA layer gate count",
                    )
                })?;

        let repeated =
            layer_gates
                .checked_mul(self.depth)
                .ok_or_else(|| {
                    numerical_overflow(
                        "QAOA repeated layer gate count",
                    )
                })?;

        self.graph
            .qubits
            .checked_mul(2)
            .and_then(|value| {
                value.checked_add(repeated)
            })
            .ok_or_else(|| {
                numerical_overflow(
                    "QAOA total logical gate count",
                )
            })
    }

    /// Returns the logical two-qubit gate count.
    pub fn logical_two_qubit_gate_count(
        &self,
    ) -> BenchmarkResult<usize> {
        self.graph
            .edges
            .len()
            .checked_mul(2)
            .and_then(|value| {
                value.checked_mul(self.depth)
            })
            .ok_or_else(|| {
                numerical_overflow(
                    "QAOA two-qubit gate count",
                )
            })
    }

    /// Returns the expected random approximation ratio when an exact optimum
    /// exists.
    pub fn random_approximation_ratio(
        &self,
    ) -> BenchmarkResult<Option<f64>> {
        let optimum =
            exact_maxcut_if_available(&self.graph)?;

        let Some(optimum) = optimum else {
            return Ok(None);
        };

        if optimum <= 0.0 {
            return Err(invalid_configuration(
                "graph",
                "MaxCut optimum must be positive for approximation-ratio benchmarking",
            ));
        }

        let random =
            self.graph.random_expected_cut()?;

        Ok(Some(random / optimum))
    }
}

// =============================================================================
// Workload description
// =============================================================================

/// Static QAOA benchmark description.
///
/// This object is safe to construct before allocating Quantum IR.
#[derive(Debug, Clone, PartialEq)]
pub struct QaoaWorkloadDescription {
    /// Typed QAOA problem.
    pub problem: QaoaProblem,

    /// Exact optimal cut value when within the bounded classical reference
    /// domain.
    pub exact_optimum: Option<f64>,

    /// Random expected cut value.
    pub random_expected_cut: f64,

    /// Logical gate count.
    pub logical_gate_count: usize,

    /// Logical two-qubit gate count.
    pub logical_two_qubit_gate_count: usize,

    /// Number of cost-edge blocks.
    pub cost_edge_blocks: usize,

    /// Number of mixer gates per layer.
    pub mixer_gates_per_layer: usize,
}

impl QaoaWorkloadDescription {
    /// Constructs and validates a static description.
    pub fn new(
        problem: QaoaProblem,
        limits: &BenchmarkLimits,
    ) -> BenchmarkResult<Self> {
        limits
            .validate()
            .map_err(limit_error)?;

        limits
            .check_qubits(problem.graph.qubits)
            .map_err(limit_error)?;

        let logical_gate_count =
            problem.logical_gate_count()?;

        let logical_two_qubit_gate_count =
            problem.logical_two_qubit_gate_count()?;

        limits
            .check_gate_count(logical_gate_count)
            .map_err(limit_error)?;

        limits
            .check_two_qubit_gates(
                logical_two_qubit_gate_count,
            )
            .map_err(limit_error)?;

        let exact_optimum =
            exact_maxcut_if_available(
                &problem.graph,
            )?;

        let random_expected_cut =
            problem.graph.random_expected_cut()?;

        if !random_expected_cut.is_finite() {
            return Err(numerical_overflow(
                "QAOA random expected cut",
            ));
        }

        Ok(Self {
            cost_edge_blocks:
                problem.graph.edges.len(),
            mixer_gates_per_layer:
                problem.graph.qubits,
            problem,
            exact_optimum,
            random_expected_cut,
            logical_gate_count,
            logical_two_qubit_gate_count,
        })
    }
}

// =============================================================================
// Benchmark result
// =============================================================================

/// Production result of analyzing one QAOA measurement distribution.
#[derive(Debug, Clone, PartialEq)]
pub struct QaoaBenchmarkResult {
    /// Stable benchmark identifier.
    pub benchmark_id: String,

    /// Result schema version.
    pub schema_version: u16,

    /// Graph qubit count.
    pub qubits: usize,

    /// QAOA depth.
    pub depth: usize,

    /// Total edge count.
    pub edges: usize,

    /// Total edge weight.
    pub total_edge_weight: f64,

    /// Exact optimum, if available.
    pub exact_optimum: Option<f64>,

    /// Random expected cut value.
    pub random_expected_cut: f64,

    /// Random approximation ratio when exact optimum is available.
    pub random_approximation_ratio: Option<f64>,

    /// Total measured shots.
    pub shots: u64,

    /// Observed expected cut value.
    pub observed_expected_cut: f64,

    /// Raw approximation ratio.
    ///
    /// `None` when exact optimum is unavailable.
    pub approximation_ratio: Option<f64>,

    /// Effective approximation ratio relative to the random baseline.
    ///
    /// `None` when the exact optimum or a meaningful random denominator is
    /// unavailable.
    pub effective_approximation_ratio: Option<f64>,

    /// Best cut value actually observed.
    pub best_observed_cut: f64,

    /// Approximation ratio of the best observed solution.
    pub best_observed_approximation_ratio: Option<f64>,

    /// Probability of sampling an exact optimal solution.
    pub optimal_solution_probability: Option<f64>,

    /// Probability of sampling a solution whose approximation ratio reaches
    /// the configured threshold.
    pub threshold_success_probability: Option<f64>,

    /// Number of optimal samples.
    pub optimal_solution_shots: Option<u64>,

    /// Number of samples meeting the configured threshold.
    pub threshold_success_shots: Option<u64>,

    /// Stable angle schedule identifier.
    pub schedule: String,

    /// Logical gate count.
    pub logical_gate_count: usize,

    /// Logical two-qubit gate count.
    pub logical_two_qubit_gate_count: usize,
}

impl QaoaBenchmarkResult {
    /// Validates the result.
    pub fn validate(&self) -> BenchmarkResult<()> {
        if self.benchmark_id
            != QAOA_BENCHMARK_ID
        {
            return Err(
                BenchmarkError::InvalidWorkload {
                    workload:
                        QAOA_APPLICATION_ID
                            .to_owned(),
                    reason:
                        "QAOA result contains an invalid benchmark identifier"
                            .to_owned(),
                },
            );
        }

        if self.schema_version
            != QAOA_RESULT_SCHEMA_VERSION
        {
            return Err(
                BenchmarkError::ReproducibilityFailure {
                    component:
                        "qaoa_result_schema"
                            .to_owned(),
                    expected:
                        QAOA_RESULT_SCHEMA_VERSION
                            .to_string(),
                    actual:
                        self.schema_version
                            .to_string(),
                },
            );
        }

        if self.qubits < 2
            || self.qubits
                > MAX_QAOA_QUBITS
        {
            return Err(
                invalid_configuration(
                    "result.qubits",
                    "QAOA result contains an invalid qubit count",
                ),
            );
        }

        if self.depth == 0
            || self.depth
                > MAX_QAOA_DEPTH
        {
            return Err(
                invalid_configuration(
                    "result.depth",
                    "QAOA result contains an invalid depth",
                ),
            );
        }

        if self.edges == 0 {
            return Err(
                invalid_configuration(
                    "result.edges",
                    "QAOA result requires at least one edge",
                ),
            );
        }

        validate_non_negative_finite(
            "total_edge_weight",
            self.total_edge_weight,
        )?;

        validate_non_negative_finite(
            "random_expected_cut",
            self.random_expected_cut,
        )?;

        validate_non_negative_finite(
            "observed_expected_cut",
            self.observed_expected_cut,
        )?;

        validate_non_negative_finite(
            "best_observed_cut",
            self.best_observed_cut,
        )?;

        if let Some(value) =
            self.exact_optimum
        {
            validate_non_negative_finite(
                "exact_optimum",
                value,
            )?;

            if value <= 0.0 {
                return Err(
                    invalid_configuration(
                        "result.exact_optimum",
                        "exact MaxCut optimum must be positive",
                    ),
                );
            }
        }

        if let Some(value) =
            self.random_approximation_ratio
        {
            validate_probability_or_nonnegative(
                "random_approximation_ratio",
                value,
            )?;
        }

        if let Some(value) =
            self.approximation_ratio
        {
            validate_probability_or_nonnegative(
                "approximation_ratio",
                value,
            )?;
        }

        if let Some(value) =
            self.effective_approximation_ratio
        {
            if !value.is_finite() {
                return Err(
                    BenchmarkError::NumericalInstability {
                        operation:
                            "qaoa_result_validation"
                                .to_owned(),
                        message:
                            "effective approximation ratio is not finite"
                                .to_owned(),
                    },
                );
            }
        }

        if let Some(value) =
            self.best_observed_approximation_ratio
        {
            validate_probability_or_nonnegative(
                "best_observed_approximation_ratio",
                value,
            )?;
        }

        if let Some(value) =
            self.optimal_solution_probability
        {
            validate_probability(
                "optimal_solution_probability",
                value,
            )?;
        }

        if let Some(value) =
            self.threshold_success_probability
        {
            validate_probability(
                "threshold_success_probability",
                value,
            )?;
        }

        if let Some(optimal_shots) =
            self.optimal_solution_shots
        {
            if optimal_shots > self.shots {
                return Err(
                    invalid_configuration(
                        "result.optimal_solution_shots",
                        "optimal solution shots cannot exceed total shots",
                    ),
                );
            }
        }

        if let Some(threshold_shots) =
            self.threshold_success_shots
        {
            if threshold_shots > self.shots {
                return Err(
                    invalid_configuration(
                        "result.threshold_success_shots",
                        "threshold success shots cannot exceed total shots",
                    ),
                );
            }
        }

        Ok(())
    }
}

// =============================================================================
// Generator
// =============================================================================

/// Stateless production QAOA application benchmark generator.
#[derive(Debug, Clone)]
pub struct QaoaBenchmarkGenerator {
    descriptor:
        ApplicationGeneratorDescriptor,

    limits: BenchmarkLimits,
}

impl QaoaBenchmarkGenerator {
    /// Creates the canonical production generator using production resource
    /// limits.
    pub fn new() -> BenchmarkResult<Self> {
        Self::with_limits(
            BenchmarkLimits::production(),
        )
    }

    /// Creates a generator with an explicit resource policy.
    ///
    /// This is useful for CI, simulator profiles, embedded environments, and
    /// controlled test environments.
    pub fn with_limits(
        limits: BenchmarkLimits,
    ) -> BenchmarkResult<Self> {
        limits
            .validate()
            .map_err(limit_error)?;

        let descriptor =
            ApplicationGeneratorDescriptor::new(
                QAOA_BENCHMARK_ID,
                QAOA_APPLICATION_ID,
                QAOA_GENERATOR_VERSION,
                "Production QAOA/MaxCut application benchmark generator",
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

        Ok(Self {
            descriptor,
            limits,
        })
    }

    /// Returns the configured benchmark resource policy.
    #[must_use]
    pub const fn limits(
        &self,
    ) -> &BenchmarkLimits {
        &self.limits
    }

    /// Parses and validates a benchmark request into a typed problem.
    pub fn problem_from_request(
        &self,
        request: &ApplicationGenerationRequest,
    ) -> BenchmarkResult<QaoaProblem> {
        request.validate()?;
        self.ensure_application(request)?;

        let depth =
            parse_depth(request)?;

        let graph =
            parse_graph(request)?;

        let schedule =
            parse_schedule(
                request,
                depth,
            )?;

        let threshold =
            parse_approximation_threshold(
                request,
            )?;

        QaoaProblem::new(
            graph,
            depth,
            schedule,
            threshold,
        )
    }

    /// Describes the benchmark without allocating Quantum IR.
    pub fn describe(
        &self,
        request: &ApplicationGenerationRequest,
    ) -> BenchmarkResult<QaoaWorkloadDescription> {
        let problem =
            self.problem_from_request(
                request,
            )?;

        QaoaWorkloadDescription::new(
            problem,
            &self.limits,
        )
    }

    /// Generates the canonical logical QAOA circuit.
    ///
    /// Circuit structure:
    ///
    /// ```text
    /// H^n
    /// │
    /// ├── cost(gamma_1)
    /// ├── mixer(beta_1)
    /// ├── cost(gamma_2)
    /// ├── mixer(beta_2)
    /// │
    /// ├── ...
    /// │
    /// ├── cost(gamma_p)
    /// └── mixer(beta_p)
    /// │
    /// measure
    /// ```
    pub fn generate_circuit(
        &self,
        request: &ApplicationGenerationRequest,
    ) -> BenchmarkResult<QuantumCircuit> {
        let description =
            self.describe(request)?;

        let (
            gamma,
            beta,
        ) = description.problem.angles()?;

        let qubits =
            description.problem.graph.qubits;

        let mut circuit =
            QuantumCircuit::new(
                qubits,
                qubits,
            )
            .map_err(|error| {
                circuit_error(
                    "unable to construct QAOA Quantum IR circuit",
                    error,
                )
            })?;

        circuit
            .set_name(Some(format!(
                "qaoa_{}",
                request
                    .instance_id()
                    .as_str()
            )))
            .map_err(|error| {
                circuit_error(
                    "unable to assign QAOA circuit name",
                    error,
                )
            })?;

        circuit
            .set_source(Some(
                "zamani.quantum.benchmarking.applications.qaoa"
                    .to_owned(),
            ))
            .map_err(|error| {
                circuit_error(
                    "unable to assign QAOA circuit source",
                    error,
                )
            })?;

        // ---------------------------------------------------------------------
        // Initial |+>^n state.
        // ---------------------------------------------------------------------

        for qubit in 0..qubits {
            push_single(
                &mut circuit,
                GateKind::H,
                qubit,
            )?;
        }

        // ---------------------------------------------------------------------
        // Alternating QAOA layers.
        // ---------------------------------------------------------------------

        for layer in 0..description.problem.depth {
            append_cost_layer(
                &mut circuit,
                &description.problem.graph,
                gamma[layer],
            )?;

            append_mixer_layer(
                &mut circuit,
                qubits,
                beta[layer],
            )?;
        }

        // ---------------------------------------------------------------------
        // Final measurement.
        // ---------------------------------------------------------------------

        for qubit in 0..qubits {
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
                    "generated QAOA circuit failed final IR validation",
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
                    "unable to create QAOA circuit workload",
                    error,
                )
            })?;

        let mut workload =
            ApplicationWorkload::new(
                QAOA_APPLICATION_ID,
                request
                    .instance_id()
                    .clone(),
                request.problem_size(),
            )
            .map_err(|error| {
                workload_error(
                    "unable to create QAOA application workload",
                    error,
                )
            })?
            .with_circuit(
                circuit_workload,
            );

        add_parameter(
            &mut workload,
            "application",
            QAOA_APPLICATION_ID,
        )?;

        add_parameter(
            &mut workload,
            "benchmark",
            QAOA_BENCHMARK_ID,
        )?;

        add_parameter(
            &mut workload,
            "schema_version",
            &QAOA_RESULT_SCHEMA_VERSION
                .to_string(),
        )?;

        add_parameter(
            &mut workload,
            "generator_version",
            QAOA_GENERATOR_VERSION,
        )?;

        add_parameter(
            &mut workload,
            "generator_revision",
            &QAOA_GENERATOR_REVISION
                .to_string(),
        )?;

        add_parameter(
            &mut workload,
            "qubits",
            &description
                .problem
                .graph
                .qubits
                .to_string(),
        )?;

        add_parameter(
            &mut workload,
            "edges",
            &description
                .problem
                .graph
                .edges
                .len()
                .to_string(),
        )?;

        add_parameter(
            &mut workload,
            "total_edge_weight",
            &format_float(
                description
                    .problem
                    .graph
                    .total_weight()?,
            ),
        )?;

        add_parameter(
            &mut workload,
            "p",
            &description
                .problem
                .depth
                .to_string(),
        )?;

        add_parameter(
            &mut workload,
            "schedule",
            description
                .problem
                .schedule
                .kind_id(),
        )?;

        add_parameter(
            &mut workload,
            "mixer",
            "x",
        )?;

        add_parameter(
            &mut workload,
            "cost_convention",
            "maxcut_positive_cut",
        )?;

        add_parameter(
            &mut workload,
            "cost_hamiltonian",
            "sum_w_I_minus_ZZ_over_2",
        )?;

        add_parameter(
            &mut workload,
            "cost_ir_decomposition",
            "cx_rz_negative_gamma_weight_cx",
        )?;

        add_parameter(
            &mut workload,
            "mixer_ir_decomposition",
            "rx_2_beta",
        )?;

        add_parameter(
            &mut workload,
            "logical_gate_count",
            &description
                .logical_gate_count
                .to_string(),
        )?;

        add_parameter(
            &mut workload,
            "logical_two_qubit_gate_count",
            &description
                .logical_two_qubit_gate_count
                .to_string(),
        )?;

        add_parameter(
            &mut workload,
            "random_expected_cut",
            &format_float(
                description
                    .random_expected_cut,
            ),
        )?;

        if let Some(optimum) =
            description.exact_optimum
        {
            add_parameter(
                &mut workload,
                "exact_optimum",
                &format_float(optimum),
            )?;
        }

        let (
            gamma,
            beta,
        ) = description.problem.angles()?;

        add_parameter(
            &mut workload,
            "gamma",
            &format_angle_vector(
                &gamma,
            ),
        )?;

        add_parameter(
            &mut workload,
            "beta",
            &format_angle_vector(
                &beta,
            ),
        )?;

        add_parameter(
            &mut workload,
            "approximation_threshold",
            &format_float(
                description
                    .problem
                    .approximation_threshold,
            ),
        )?;

        Ok(workload)
    }

    /// Analyzes normalized computational-basis counts.
    ///
    /// The execution layer must normalize backend bit ordering before this
    /// method is called.
    pub fn analyze_counts(
        &self,
        request: &ApplicationGenerationRequest,
        counts: &BTreeMap<String, u64>,
    ) -> BenchmarkResult<QaoaBenchmarkResult> {
        let description =
            self.describe(request)?;

        analyze_counts_for_description(
            &description,
            counts,
        )
    }

    /// Returns the exact MaxCut optimum when the problem is within the bounded
    /// classical reference domain.
    pub fn exact_optimum(
        &self,
        request: &ApplicationGenerationRequest,
    ) -> BenchmarkResult<Option<f64>> {
        let problem =
            self.problem_from_request(request)?;

        exact_maxcut_if_available(
            &problem.graph,
        )
    }

    fn ensure_application(
        &self,
        request: &ApplicationGenerationRequest,
    ) -> BenchmarkResult<()> {
        if request.application_id()
            != QAOA_APPLICATION_ID
        {
            return Err(
                BenchmarkError::InconsistentConfiguration {
                    first:
                        "request.application_id"
                            .to_owned(),
                    second:
                        "qaoa.application_id"
                            .to_owned(),
                    reason:
                        "QAOA generator requires application_id `qaoa`"
                            .to_owned(),
                },
            );
        }

        Ok(())
    }
}

impl ApplicationBenchmarkGenerator
    for QaoaBenchmarkGenerator
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
            self.problem_from_request(
                request,
            )?;

        let _ =
            QaoaWorkloadDescription::new(
                problem,
                &self.limits,
            )?;

        Ok(())
    }

    fn generate_workload(
        &self,
        request: &ApplicationGenerationRequest,
    ) -> BenchmarkResult<ApplicationWorkload> {
        self.generate_application_workload(
            request,
        )
    }
}

// =============================================================================
// Request parsing
// =============================================================================

fn parse_depth(
    request: &ApplicationGenerationRequest,
) -> BenchmarkResult<usize> {
    let mut depth: Option<usize> = None;

    for parameter in request.parameters() {
        if parameter.value().len()
            > MAX_QAOA_PARAMETER_VALUE_BYTES
        {
            return Err(
                invalid_configuration(
                    "application_parameter",
                    "QAOA application parameter value is too large",
                ),
            );
        }

        if parameter.name() != "p" {
            continue;
        }

        if depth.is_some() {
            return Err(
                invalid_configuration(
                    "p",
                    "duplicate p parameter",
                ),
            );
        }

        let value =
            parameter
                .value()
                .parse::<usize>()
                .map_err(|_| {
                    invalid_configuration(
                        "p",
                        "p must be a positive integer",
                    )
                })?;

        if value == 0 {
            return Err(
                invalid_configuration(
                    "p",
                    "p must be greater than zero",
                ),
            );
        }

        if value > MAX_QAOA_DEPTH {
            return Err(
                BenchmarkError::ResourceLimitExceeded {
                    resource:
                        "qaoa_depth"
                            .to_owned(),
                    requested:
                        value as u64,
                    maximum:
                        MAX_QAOA_DEPTH
                            as u64,
                },
            );
        }

        depth = Some(value);
    }

    Ok(depth.unwrap_or(
        DEFAULT_QAOA_DEPTH,
    ))
}

fn parse_graph(
    request: &ApplicationGenerationRequest,
) -> BenchmarkResult<QaoaGraph> {
    let mut graph_kind: Option<String> =
        None;

    let mut edge_parameter: Option<String> =
        None;

    for parameter in request.parameters() {
        match parameter.name() {
            "graph" => {
                if graph_kind.is_some() {
                    return Err(
                        invalid_configuration(
                            "graph",
                            "duplicate graph parameter",
                        ),
                    );
                }

                graph_kind =
                    Some(
                        parameter
                            .value()
                            .to_owned(),
                    );
            }

            "edges" => {
                if edge_parameter.is_some() {
                    return Err(
                        invalid_configuration(
                            "edges",
                            "duplicate edges parameter",
                        ),
                    );
                }

                if parameter.value().len()
                    > MAX_EDGE_PARAMETER_BYTES
                {
                    return Err(
                        invalid_configuration(
                            "edges",
                            "custom edge list is too large",
                        ),
                    );
                }

                edge_parameter =
                    Some(
                        parameter
                            .value()
                            .to_owned(),
                    );
            }

            _ => {}
        }
    }

    let kind =
        graph_kind
            .unwrap_or_else(|| {
                "ring".to_owned()
            });

    match kind.as_str() {
        "path" => {
            if edge_parameter.is_some() {
                return Err(
                    invalid_configuration(
                        "edges",
                        "edges cannot be supplied with graph=path",
                    ),
                );
            }

            QaoaGraph::path(
                request.problem_size(),
            )
        }

        "ring" => {
            if edge_parameter.is_some() {
                return Err(
                    invalid_configuration(
                        "edges",
                        "edges cannot be supplied with graph=ring",
                    ),
                );
            }

            QaoaGraph::ring(
                request.problem_size(),
            )
        }

        "complete" => {
            if edge_parameter.is_some() {
                return Err(
                    invalid_configuration(
                        "edges",
                        "edges cannot be supplied with graph=complete",
                    ),
                );
            }

            QaoaGraph::complete(
                request.problem_size(),
            )
        }

        "custom" => {
            let encoded =
                edge_parameter
                    .ok_or_else(|| {
                        invalid_configuration(
                            "edges",
                            "graph=custom requires an edges parameter",
                        )
                    })?;

            parse_custom_edges(
                request.problem_size(),
                &encoded,
            )
        }

        _ => Err(
            invalid_configuration(
                "graph",
                "graph must be path, ring, complete, or custom",
            ),
        ),
    }
}

fn parse_schedule(
    request: &ApplicationGenerationRequest,
    depth: usize,
) -> BenchmarkResult<QaoaAngleSchedule> {
    let mut schedule: Option<String> =
        None;

    let mut delta_gamma: Option<f64> =
        None;

    let mut delta_beta: Option<f64> =
        None;

    let mut gamma: Option<Vec<f64>> =
        None;

    let mut beta: Option<Vec<f64>> =
        None;

    for parameter in request.parameters() {
        match parameter.name() {
            "schedule" => {
                if schedule.is_some() {
                    return Err(
                        invalid_configuration(
                            "schedule",
                            "duplicate schedule parameter",
                        ),
                    );
                }

                schedule =
                    Some(
                        parameter
                            .value()
                            .to_owned(),
                    );
            }

            "delta_gamma" => {
                if delta_gamma.is_some() {
                    return Err(
                        invalid_configuration(
                            "delta_gamma",
                            "duplicate delta_gamma parameter",
                        ),
                    );
                }

                delta_gamma =
                    Some(parse_finite_f64(
                        parameter.value(),
                        "delta_gamma",
                    )?);
            }

            "delta_beta" => {
                if delta_beta.is_some() {
                    return Err(
                        invalid_configuration(
                            "delta_beta",
                            "duplicate delta_beta parameter",
                        ),
                    );
                }

                delta_beta =
                    Some(parse_finite_f64(
                        parameter.value(),
                        "delta_beta",
                    )?);
            }

            "gamma" => {
                if gamma.is_some() {
                    return Err(
                        invalid_configuration(
                            "gamma",
                            "duplicate gamma parameter",
                        ),
                    );
                }

                gamma =
                    Some(parse_angle_vector(
                        parameter.value(),
                        "gamma",
                    )?);
            }

            "beta" => {
                if beta.is_some() {
                    return Err(
                        invalid_configuration(
                            "beta",
                            "duplicate beta parameter",
                        ),
                    );
                }

                beta =
                    Some(parse_angle_vector(
                        parameter.value(),
                        "beta",
                    )?);
            }

            _ => {}
        }
    }

    let schedule_kind =
        schedule.unwrap_or_else(|| {
            "linear_ramp".to_owned()
        });

    match schedule_kind.as_str() {
        "linear_ramp" => {
            if gamma.is_some()
                || beta.is_some()
            {
                return Err(
                    invalid_configuration(
                        "gamma/beta",
                        "explicit gamma and beta cannot be combined with schedule=linear_ramp",
                    ),
                );
            }

            Ok(
                QaoaAngleSchedule::LinearRamp {
                    delta_gamma:
                        delta_gamma
                            .unwrap_or(
                                DEFAULT_DELTA_GAMMA,
                            ),
                    delta_beta:
                        delta_beta
                            .unwrap_or(
                                DEFAULT_DELTA_BETA,
                            ),
                },
            )
        }

        "explicit" => {
            if delta_gamma.is_some()
                || delta_beta.is_some()
            {
                return Err(
                    invalid_configuration(
                        "delta_gamma/delta_beta",
                        "LR-QAOA slope parameters cannot be combined with schedule=explicit",
                    ),
                );
            }

            let gamma =
                gamma.ok_or_else(|| {
                    invalid_configuration(
                        "gamma",
                        "schedule=explicit requires gamma",
                    )
                })?;

            let beta =
                beta.ok_or_else(|| {
                    invalid_configuration(
                        "beta",
                        "schedule=explicit requires beta",
                    )
                })?;

            if gamma.len() != depth
                || beta.len() != depth
            {
                return Err(
                    invalid_configuration(
                        "gamma/beta",
                        "explicit angle vectors must contain exactly p values",
                    ),
                );
            }

            Ok(
                QaoaAngleSchedule::Explicit {
                    gamma,
                    beta,
                },
            )
        }

        _ => Err(
            invalid_configuration(
                "schedule",
                "schedule must be linear_ramp or explicit",
            ),
        ),
    }
}

fn parse_approximation_threshold(
    request: &ApplicationGenerationRequest,
) -> BenchmarkResult<f64> {
    let mut value: Option<f64> =
        None;

    for parameter in request.parameters() {
        if parameter.name()
            != "approximation_threshold"
        {
            continue;
        }

        if value.is_some() {
            return Err(
                invalid_configuration(
                    "approximation_threshold",
                    "duplicate approximation_threshold parameter",
                ),
            );
        }

        value =
            Some(parse_finite_f64(
                parameter.value(),
                "approximation_threshold",
            )?);
    }

    let threshold =
        value.unwrap_or(
            DEFAULT_APPROXIMATION_THRESHOLD,
        );

    if !(0.0..=1.0)
        .contains(&threshold)
    {
        return Err(
            invalid_configuration(
                "approximation_threshold",
                "approximation threshold must be within [0,1]",
            ),
        );
    }

    Ok(threshold)
}

fn parse_custom_edges(
    qubits: usize,
    encoded: &str,
) -> BenchmarkResult<QaoaGraph> {
    if encoded.is_empty() {
        return Err(
            invalid_configuration(
                "edges",
                "custom edge list cannot be empty",
            ),
        );
    }

    let mut edges =
        Vec::new();

    for token in
        encoded.split(',')
    {
        let token =
            token.trim();

        if token.is_empty() {
            return Err(
                invalid_configuration(
                    "edges",
                    "custom edge list contains an empty edge",
                ),
            );
        }

        let mut parts =
            token.split(':');

        let endpoints =
            parts.next().ok_or_else(
                || {
                    invalid_configuration(
                        "edges",
                        "custom edge is missing endpoints",
                    )
                },
            )?;

        let weight_text =
            parts.next().ok_or_else(
                || {
                    invalid_configuration(
                        "edges",
                        "custom edge is missing weight",
                    )
                },
            )?;

        if parts.next().is_some() {
            return Err(
                invalid_configuration(
                    "edges",
                    "custom edge contains too many ':' separators",
                ),
            );
        }

        let mut endpoint_parts =
            endpoints.split('-');

        let u =
            endpoint_parts
                .next()
                .ok_or_else(|| {
                    invalid_configuration(
                        "edges",
                        "edge is missing first endpoint",
                    )
                })?
                .trim()
                .parse::<usize>()
                .map_err(|_| {
                    invalid_configuration(
                        "edges",
                        "edge endpoint must be an unsigned integer",
                    )
                })?;

        let v =
            endpoint_parts
                .next()
                .ok_or_else(|| {
                    invalid_configuration(
                        "edges",
                        "edge is missing second endpoint",
                    )
                })?
                .trim()
                .parse::<usize>()
                .map_err(|_| {
                    invalid_configuration(
                        "edges",
                        "edge endpoint must be an unsigned integer",
                    )
                })?;

        if endpoint_parts
            .next()
            .is_some()
        {
            return Err(
                invalid_configuration(
                    "edges",
                    "edge contains too many '-' separators",
                ),
            );
        }

        let weight =
            weight_text
                .trim()
                .parse::<f64>()
                .map_err(|_| {
                    invalid_configuration(
                        "edges",
                        "edge weight must be a finite floating-point number",
                    )
                })?;

        edges.push(
            QaoaEdge::new(
                u,
                v,
                weight,
            )?,
        );

        if edges.len()
            > MAX_QAOA_EDGES
        {
            return Err(
                BenchmarkError::ResourceLimitExceeded {
                    resource:
                        "qaoa_edges"
                            .to_owned(),
                    requested:
                        edges.len()
                            as u64,
                    maximum:
                        MAX_QAOA_EDGES
                            as u64,
                },
            );
        }
    }

    edges.sort_by(
        |left, right| {
            left.u
                .cmp(&right.u)
                .then_with(
                    || left.v.cmp(&right.v),
                )
        },
    );

    QaoaGraph::new(
        qubits,
        edges,
    )
}

fn parse_angle_vector(
    value: &str,
    field: &'static str,
) -> BenchmarkResult<Vec<f64>> {
    if value.trim().is_empty() {
        return Err(
            invalid_configuration(
                field,
                "angle vector cannot be empty",
            ),
        );
    }

    let mut values =
        Vec::new();

    for token in
        value.split(',')
    {
        let token =
            token.trim();

        if token.is_empty() {
            return Err(
                invalid_configuration(
                    field,
                    "angle vector contains an empty element",
                ),
            );
        }

        let parsed =
            token.parse::<f64>().map_err(
                |_| {
                    invalid_configuration(
                        field,
                        "angle vector contains an invalid floating-point value",
                    )
                },
            )?;

        if !parsed.is_finite() {
            return Err(
                invalid_configuration(
                    field,
                    "angle vector contains a non-finite value",
                ),
            );
        }

        values.push(parsed);

        if values.len()
            > MAX_QAOA_DEPTH
        {
            return Err(
                BenchmarkError::ResourceLimitExceeded {
                    resource:
                        "qaoa_angle_count"
                            .to_owned(),
                    requested:
                        values.len()
                            as u64,
                    maximum:
                        MAX_QAOA_DEPTH
                            as u64,
                },
            );
        }
    }

    Ok(values)
}

fn parse_finite_f64(
    value: &str,
    field: &'static str,
) -> BenchmarkResult<f64> {
    let parsed =
        value.parse::<f64>().map_err(
            |_| {
                invalid_configuration(
                    field,
                    "value must be a finite floating-point number",
                )
            },
        )?;

    if !parsed.is_finite() {
        return Err(
            invalid_configuration(
                field,
                "value must be finite",
            ),
        );
    }

    Ok(parsed)
}

// =============================================================================
// Circuit generation
// =============================================================================

fn append_cost_layer(
    circuit: &mut QuantumCircuit,
    graph: &QaoaGraph,
    gamma: f64,
) -> BenchmarkResult<()> {
    if !gamma.is_finite() {
        return Err(
            invalid_configuration(
                "gamma",
                "QAOA gamma must be finite",
            ),
        );
    }

    for edge in &graph.edges {
        let angle =
            -(gamma * edge.weight);

        if !angle.is_finite() {
            return Err(
                numerical_overflow(
                    "QAOA cost rotation angle",
                ),
            );
        }

        push_two_qubit(
            circuit,
            GateKind::CX,
            edge.u,
            edge.v,
        )?;

        push_parameterized_single(
            circuit,
            GateKind::RZ,
            edge.v,
            angle,
        )?;

        push_two_qubit(
            circuit,
            GateKind::CX,
            edge.u,
            edge.v,
        )?;
    }

    Ok(())
}

fn append_mixer_layer(
    circuit: &mut QuantumCircuit,
    qubits: usize,
    beta: f64,
) -> BenchmarkResult<()> {
    if !beta.is_finite() {
        return Err(
            invalid_configuration(
                "beta",
                "QAOA beta must be finite",
            ),
        );
    }

    let angle =
        2.0 * beta;

    if !angle.is_finite() {
        return Err(
            numerical_overflow(
                "QAOA mixer rotation angle",
            ),
        );
    }

    for qubit in 0..qubits {
        push_parameterized_single(
            circuit,
            GateKind::RX,
            qubit,
            angle,
        )?;
    }

    Ok(())
}

// =============================================================================
// Circuit helpers
// =============================================================================

fn push_single(
    circuit: &mut QuantumCircuit,
    kind: GateKind,
    qubit: usize,
) -> BenchmarkResult<()> {
    let gate =
        Gate::new(
            kind,
            vec![
                QubitId::new(qubit),
            ],
            Vec::new(),
            None,
            None,
        )
        .map_err(|error| {
            invalid_workload(
                "QAOA generated invalid single-qubit gate",
                error,
            )
        })?;

    circuit
        .push(gate)
        .map_err(|error| {
            circuit_error(
                "unable to append QAOA single-qubit gate",
                error,
            )
        })
}

fn push_parameterized_single(
    circuit: &mut QuantumCircuit,
    kind: GateKind,
    qubit: usize,
    value: f64,
) -> BenchmarkResult<()> {
    if !value.is_finite() {
        return Err(
            invalid_configuration(
                "gate_parameter",
                "QAOA gate parameter must be finite",
            ),
        );
    }

    let parameter =
        Parameter::constant(value)
            .map_err(|error| {
                invalid_workload(
                    "QAOA generated invalid gate parameter",
                    error,
                )
            })?;

    let gate =
        Gate::new(
            kind,
            vec![
                QubitId::new(qubit),
            ],
            vec![parameter],
            None,
            None,
        )
        .map_err(|error| {
            invalid_workload(
                "QAOA generated invalid parameterized gate",
                error,
            )
        })?;

    circuit
        .push(gate)
        .map_err(|error| {
            circuit_error(
                "unable to append QAOA parameterized gate",
                error,
            )
        })
}

fn push_two_qubit(
    circuit: &mut QuantumCircuit,
    kind: GateKind,
    first: usize,
    second: usize,
) -> BenchmarkResult<()> {
    if first == second {
        return Err(
            invalid_configuration(
                "gate",
                "QAOA two-qubit gate cannot target the same qubit",
            ),
        );
    }

    let gate =
        Gate::new(
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
                "QAOA generated invalid two-qubit gate",
                error,
            )
        })?;

    circuit
        .push(gate)
        .map_err(|error| {
            circuit_error(
                "unable to append QAOA two-qubit gate",
                error,
            )
        })
}

fn push_measurement(
    circuit: &mut QuantumCircuit,
    qubit: usize,
    classical_bit: usize,
) -> BenchmarkResult<()> {
    let gate =
        Gate::new(
            GateKind::Measure,
            vec![
                QubitId::new(qubit),
            ],
            Vec::new(),
            Some(classical_bit),
            Some(
                Measurement::new(
                    QubitId::new(qubit),
                    ClassicalBitId::new(
                        classical_bit,
                    ),
                ),
            ),
        )
        .map_err(|error| {
            invalid_workload(
                "QAOA generated invalid measurement gate",
                error,
            )
        })?;

    circuit
        .push(gate)
        .map_err(|error| {
            circuit_error(
                "unable to append QAOA measurement",
                error,
            )
        })
}

// =============================================================================
// Classical reference calculation
// =============================================================================

/// Returns the exact MaxCut optimum when the graph is within the bounded
/// classical reference domain.
fn exact_maxcut_if_available(
    graph: &QaoaGraph,
) -> BenchmarkResult<Option<f64>> {
    if graph.qubits
        > MAX_EXACT_REFERENCE_QUBITS
    {
        return Ok(None);
    }

    let state_count =
        1usize
            .checked_shl(
                graph.qubits as u32,
            )
            .ok_or_else(|| {
                numerical_overflow(
                    "exact MaxCut state count",
                )
            })?;

    let mut best =
        0.0_f64;

    for state in 0..state_count {
        let mut value =
            0.0_f64;

        for edge in &graph.edges {
            let u_bit =
                (state >> edge.u) & 1usize;
            let v_bit =
                (state >> edge.v) & 1usize;

            if u_bit != v_bit {
                value += edge.weight;

                if !value.is_finite() {
                    return Err(
                        numerical_overflow(
                            "exact MaxCut objective",
                        ),
                    );
                }
            }
        }

        if value > best {
            best = value;
        }
    }

    if best <= 0.0 {
        return Err(
            invalid_configuration(
                "graph",
                "exact MaxCut optimum must be positive",
            ),
        );
    }

    Ok(Some(best))
}

// =============================================================================
// Count analysis
// =============================================================================

fn analyze_counts_for_description(
    description: &QaoaWorkloadDescription,
    counts: &BTreeMap<String, u64>,
) -> BenchmarkResult<QaoaBenchmarkResult> {
    if counts.is_empty() {
        return Err(
            invalid_configuration(
                "counts",
                "QAOA analysis requires at least one measurement outcome",
            ),
        );
    }

    let total_shots =
        counts.values().try_fold(
            0u64,
            |total, &count| {
                total.checked_add(count)
                    .ok_or_else(|| {
                        numerical_overflow(
                            "QAOA total shot count",
                        )
                    })
            },
        )?;

    if total_shots == 0 {
        return Err(
            invalid_configuration(
                "counts",
                "QAOA analysis requires at least one measured shot",
            ),
        );
    }

    let mut weighted_cut_sum =
        0.0_f64;

    let mut best_cut =
        0.0_f64;

    let mut optimal_shots =
        0u64;

    let mut threshold_shots =
        0u64;

    for (bitstring, &count) in counts {
        validate_bitstring(
            bitstring,
            description
                .problem
                .graph
                .qubits,
        )?;

        if count == 0 {
            continue;
        }

        let cut =
            description
                .problem
                .graph
                .cut_value(
                    bitstring,
                )?;

        weighted_cut_sum +=
            cut * count as f64;

        if !weighted_cut_sum
            .is_finite()
        {
            return Err(
                numerical_overflow(
                    "QAOA observed cut-value sum",
                ),
            );
        }

        if cut > best_cut {
            best_cut = cut;
        }

        if let Some(optimum) =
            description.exact_optimum
        {
            if approx_equal_cut(
                cut,
                optimum,
            ) {
                optimal_shots =
                    optimal_shots
                        .checked_add(
                            count,
                        )
                        .ok_or_else(|| {
                            numerical_overflow(
                                "QAOA optimal-shot count",
                            )
                        })?;
            }

            let ratio =
                cut / optimum;

            if ratio
                + f64::EPSILON
                >= description
                    .problem
                    .approximation_threshold
            {
                threshold_shots =
                    threshold_shots
                        .checked_add(
                            count,
                        )
                        .ok_or_else(|| {
                            numerical_overflow(
                                "QAOA threshold-shot count",
                            )
                        })?;
            }
        }
    }

    let observed_expected_cut =
        weighted_cut_sum
            / total_shots as f64;

    if !observed_expected_cut
        .is_finite()
    {
        return Err(
            numerical_overflow(
                "QAOA observed expected cut",
            ),
        );
    }

    let (
        approximation_ratio,
        best_observed_approximation_ratio,
        optimal_solution_probability,
        threshold_success_probability,
        effective_approximation_ratio,
        random_approximation_ratio,
    ) =
        if let Some(optimum) =
            description.exact_optimum
        {
            let ratio =
                observed_expected_cut
                    / optimum;

            let best_ratio =
                best_cut / optimum;

            let random_ratio =
                description
                    .random_expected_cut
                    / optimum;

            if random_ratio >= 1.0 {
                return Err(
                    invalid_configuration(
                        "graph",
                        "random MaxCut baseline cannot equal or exceed the exact optimum",
                    ),
                );
            }

            let effective =
                if (1.0 - random_ratio)
                    .abs()
                    <= f64::EPSILON
                {
                    None
                } else {
                    Some(
                        (ratio
                            - random_ratio)
                            / (1.0
                                - random_ratio),
                    )
                };

            let optimal_probability =
                optimal_shots as f64
                    / total_shots as f64;

            let threshold_probability =
                threshold_shots as f64
                    / total_shots as f64;

            (
                Some(ratio),
                Some(best_ratio),
                Some(optimal_probability),
                Some(threshold_probability),
                effective,
                Some(random_ratio),
            )
        } else {
            (
                None,
                None,
                None,
                None,
                None,
                None,
            )
        };

    let result =
        QaoaBenchmarkResult {
            benchmark_id:
                QAOA_BENCHMARK_ID
                    .to_owned(),
            schema_version:
                QAOA_RESULT_SCHEMA_VERSION,
            qubits:
                description
                    .problem
                    .graph
                    .qubits,
            depth:
                description
                    .problem
                    .depth,
            edges:
                description
                    .problem
                    .graph
                    .edges
                    .len(),
            total_edge_weight:
                description
                    .problem
                    .graph
                    .total_weight()?,
            exact_optimum:
                description
                    .exact_optimum,
            random_expected_cut:
                description
                    .random_expected_cut,
            random_approximation_ratio:
                random_approximation_ratio,
            shots:
                total_shots,
            observed_expected_cut,
            approximation_ratio,
            effective_approximation_ratio:
                effective_approximation_ratio,
            best_observed_cut:
                best_cut,
            best_observed_approximation_ratio,
            optimal_solution_probability,
            threshold_success_probability,
            optimal_solution_shots:
                description
                    .exact_optimum
                    .map(|_| optimal_shots),
            threshold_success_shots:
                description
                    .exact_optimum
                    .map(|_| threshold_shots),
            schedule:
                description
                    .problem
                    .schedule
                    .kind_id()
                    .to_owned(),
            logical_gate_count:
                description
                    .logical_gate_count,
            logical_two_qubit_gate_count:
                description
                    .logical_two_qubit_gate_count,
        };

    result.validate()?;

    Ok(result)
}

// =============================================================================
// Validation helpers
// =============================================================================

fn validate_bitstring(
    bits: &str,
    expected_len: usize,
) -> BenchmarkResult<()> {
    if bits.len()
        != expected_len
    {
        return Err(
            invalid_configuration(
                "counts",
                "QAOA bitstring length does not equal the logical qubit count",
            ),
        );
    }

    if !bits
        .bytes()
        .all(|byte| {
            byte == b'0'
                || byte == b'1'
        })
    {
        return Err(
            invalid_configuration(
                "counts",
                "QAOA bitstrings must contain only 0 and 1",
            ),
        );
    }

    Ok(())
}

fn validate_probability(
    field: &'static str,
    value: f64,
) -> BenchmarkResult<()> {
    if !value.is_finite()
        || !(0.0..=1.0)
            .contains(&value)
    {
        return Err(
            invalid_configuration(
                field,
                "value must be a finite probability in [0,1]",
            ),
        );
    }

    Ok(())
}

fn validate_probability_or_nonnegative(
    field: &'static str,
    value: f64,
) -> BenchmarkResult<()> {
    if !value.is_finite()
        || value < 0.0
    {
        return Err(
            invalid_configuration(
                field,
                "value must be finite and non-negative",
            ),
        );
    }

    Ok(())
}

fn validate_non_negative_finite(
    field: &'static str,
    value: f64,
) -> BenchmarkResult<()> {
    if !value.is_finite()
        || value < 0.0
    {
        return Err(
            invalid_configuration(
                field,
                "value must be finite and non-negative",
            ),
        );
    }

    Ok(())
}

fn approx_equal_cut(
    left: f64,
    right: f64,
) -> bool {
    let scale =
        left.abs().max(
            right.abs(),
        );

    let tolerance =
        1.0e-12
            * scale.max(1.0);

    (left - right).abs()
        <= tolerance
}

// =============================================================================
// Metadata helpers
// =============================================================================

fn add_parameter(
    workload: &mut ApplicationWorkload,
    name: &str,
    value: &str,
) -> BenchmarkResult<()> {
    let parameter =
        ApplicationParameter::new(
            name,
            value,
        )
        .map_err(|error| {
            workload_error(
                "unable to encode QAOA application metadata",
                error,
            )
        })?;

    workload
        .add_parameter(parameter)
        .map_err(|error| {
            workload_error(
                "unable to attach QAOA application metadata",
                error,
            )
        })
}

fn format_float(
    value: f64,
) -> String {
    format!("{value:.17}")
}

fn format_angle_vector(
    values: &[f64],
) -> String {
    let mut output =
        String::new();

    for (index, value) in
        values.iter().enumerate()
    {
        if index != 0 {
            output.push(',');
        }

        output.push_str(
            &format!("{value:.17}"),
        );
    }

    output
}

fn format_edge_list(
    graph: &QaoaGraph,
) -> String {
    let mut output =
        String::new();

    for (index, edge) in
        graph.edges.iter().enumerate()
    {
        if index != 0 {
            output.push(',');
        }

        output.push_str(
            &format!(
                "{}-{}:{:.17}",
                edge.u,
                edge.v,
                edge.weight
            ),
        );
    }

    output
}

// =============================================================================
// Error helpers
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
            QAOA_APPLICATION_ID
                .to_owned(),
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
            QAOA_APPLICATION_ID
                .to_owned(),
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
            QAOA_APPLICATION_ID
                .to_owned(),
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

fn limit_error(
    error: impl fmt::Display,
) -> BenchmarkError {
    BenchmarkError::ResourceLimitExceeded {
        resource:
            "benchmark_limits"
                .to_owned(),
        requested: 1,
        maximum: 0,
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use super::super::super::core::workload::WorkloadId;

    fn request(
        problem_size: usize,
    ) -> ApplicationGenerationRequest {
        ApplicationGenerationRequest::new(
            QAOA_APPLICATION_ID,
            WorkloadId::new(
                "qaoa_instance_0",
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
            QAOA_GENERATOR_REVISION,
        )
    }

    fn request_with_parameters(
        problem_size: usize,
        parameters:
            &[(&str, &str)],
    ) -> ApplicationGenerationRequest {
        let mut request =
            request(problem_size);

        for &(name, value) in
            parameters
        {
            request =
                request.with_parameter(
                    ApplicationParameter::new(
                        name,
                        value,
                    )
                    .expect(
                        "test parameter must be valid",
                    ),
                );
        }

        request
    }

    #[test]
    fn ring_graph_has_expected_edges() {
        let graph =
            QaoaGraph::ring(4)
                .expect(
                    "ring graph must build",
                );

        assert_eq!(
            graph.edges.len(),
            4
        );

        assert_eq!(
            graph.total_weight()
                .expect(
                    "weight must calculate",
                ),
            4.0
        );
    }

    #[test]
    fn two_vertex_ring_has_one_edge() {
        let graph =
            QaoaGraph::ring(2)
                .expect(
                    "two-vertex ring must build",
                );

        assert_eq!(
            graph.edges.len(),
            1
        );
    }

    #[test]
    fn duplicate_edges_are_rejected() {
        let result =
            QaoaGraph::new(
                3,
                vec![
                    QaoaEdge::new(
                        0,
                        1,
                        1.0,
                    )
                    .expect("edge"),
                    QaoaEdge::new(
                        1,
                        0,
                        1.0,
                    )
                    .expect("edge"),
                ],
            );

        assert!(
            result.is_err()
        );
    }

    #[test]
    fn self_loop_is_rejected() {
        assert!(
            QaoaEdge::new(
                0,
                0,
                1.0,
            )
            .is_err()
        );
    }

    #[test]
    fn negative_weight_is_rejected() {
        assert!(
            QaoaEdge::new(
                0,
                1,
                -1.0,
            )
            .is_err()
        );
    }

    #[test]
    fn linear_ramp_is_deterministic() {
        let schedule =
            QaoaAngleSchedule::LinearRamp {
                delta_gamma: 0.6,
                delta_beta: 0.3,
            };

        let (
            gamma,
            beta,
        ) = schedule
            .angles(4)
            .expect(
                "angles must generate",
            );

        assert_eq!(
            gamma.len(),
            4
        );

        assert_eq!(
            beta.len(),
            4
        );

        assert!(
            (gamma[0] - 0.15)
                .abs()
                < 1.0e-12
        );

        assert!(
            (gamma[3] - 0.6)
                .abs()
                < 1.0e-12
        );

        assert!(
            (beta[0] - 0.3)
                .abs()
                < 1.0e-12
        );

        assert!(
            (beta[3] - 0.075)
                .abs()
                < 1.0e-12
        );
    }

    #[test]
    fn explicit_angles_require_exact_depth() {
        let schedule =
            QaoaAngleSchedule::Explicit {
                gamma: vec![
                    0.1,
                    0.2,
                ],
                beta: vec![
                    0.3,
                    0.4,
                ],
            };

        assert!(
            schedule
                .validate(2)
                .is_ok()
        );

        assert!(
            schedule
                .validate(1)
                .is_err()
        );
    }

    #[test]
    fn exact_maxcut_triangle_is_two() {
        let graph =
            QaoaGraph::complete(3)
                .expect(
                    "triangle must build",
                );

        let optimum =
            exact_maxcut_if_available(
                &graph,
            )
            .expect(
                "exact calculation must succeed",
            )
            .expect(
                "triangle must be exactly verifiable",
            );

        assert!(
            (optimum - 2.0)
                .abs()
                < 1.0e-12
        );
    }

    #[test]
    fn exact_maxcut_path_is_two_for_four_vertices() {
        let graph =
            QaoaGraph::path(4)
                .expect(
                    "path must build",
                );

        let optimum =
            exact_maxcut_if_available(
                &graph,
            )
            .expect(
                "exact calculation must succeed",
            )
            .expect(
                "path must be exactly verifiable",
            );

        assert!(
            (optimum - 3.0)
                .abs()
                < 1.0e-12
        );
    }

    #[test]
    fn random_expected_cut_is_half_total_weight() {
        let graph =
            QaoaGraph::path(4)
                .expect(
                    "path must build",
                );

        assert!(
            (
                graph
                    .random_expected_cut()
                    .expect(
                        "random baseline must calculate",
                    )
                    - 1.5
            )
            .abs()
                < 1.0e-12
        );
    }

    #[test]
    fn custom_weighted_graph_parses() {
        let request =
            request_with_parameters(
                3,
                &[
                    (
                        "graph",
                        "custom",
                    ),
                    (
                        "edges",
                        "0-1:2.0,1-2:3.0,0-2:1.5",
                    ),
                ],
            );

        let generator =
            QaoaBenchmarkGenerator::new()
                .expect(
                    "generator must construct",
                );

        let graph =
            generator
                .problem_from_request(
                    &request,
                )
                .expect(
                    "custom graph must parse",
                )
                .graph;

        assert_eq!(
            graph.edges.len(),
            3
        );

        assert!(
            (
                graph
                    .total_weight()
                    .expect(
                        "weight must calculate",
                    )
                    - 6.5
            )
            .abs()
                < 1.0e-12
        );
    }

    #[test]
    fn default_request_uses_lr_qaoa() {
        let generator =
            QaoaBenchmarkGenerator::new()
                .expect(
                    "generator must construct",
                );

        let description =
            generator
                .describe(
                    &request(4),
                )
                .expect(
                    "default description must build",
                );

        assert_eq!(
            description
                .problem
                .depth,
            DEFAULT_QAOA_DEPTH
        );

        assert_eq!(
            description
                .problem
                .schedule
                .kind_id(),
            "linear_ramp"
        );
    }

    #[test]
    fn generated_circuit_is_valid() {
        let generator =
            QaoaBenchmarkGenerator::new()
                .expect(
                    "generator must construct",
                );

        let circuit =
            generator
                .generate_circuit(
                    &request(3),
                )
                .expect(
                    "QAOA circuit must generate",
                );

        assert_eq!(
            circuit.qubit_count(),
            3
        );

        assert_eq!(
            circuit.classical_bit_count(),
            3
        );

        assert!(
            circuit
                .validate()
                .is_ok()
        );
    }

    #[test]
    fn generated_workload_contains_metadata() {
        let generator =
            QaoaBenchmarkGenerator::new()
                .expect(
                    "generator must construct",
                );

        let workload =
            generator
                .generate_application_workload(
                    &request(3),
                )
                .expect(
                    "workload must generate",
                );

        assert_eq!(
            workload.application_id(),
            QAOA_APPLICATION_ID
        );

        assert!(
            workload
                .circuit()
                .is_some()
        );
    }

    #[test]
    fn exact_optimal_counts_produce_perfect_metrics() {
        let generator =
            QaoaBenchmarkGenerator::new()
                .expect(
                    "generator must construct",
                );

        let request =
            request_with_parameters(
                3,
                &[
                    (
                        "graph",
                        "complete",
                    ),
                    (
                        "p",
                        "1",
                    ),
                    (
                        "schedule",
                        "explicit",
                    ),
                    (
                        "gamma",
                        "1.0",
                    ),
                    (
                        "beta",
                        "0.1",
                    ),
                ],
            );

        let mut counts =
            BTreeMap::new();

        counts.insert(
            "001".to_owned(),
            500,
        );

        counts.insert(
            "010".to_owned(),
            500,
        );

        let result =
            generator
                .analyze_counts(
                    &request,
                    &counts,
                )
                .expect(
                    "analysis must succeed",
                );

        assert_eq!(
            result.shots,
            1_000
        );

        assert!(
            (
                result
                    .approximation_ratio
                    .expect(
                        "ratio must exist",
                    )
                    - 1.0
            )
            .abs()
                < 1.0e-12
        );

        assert!(
            (
                result
                    .optimal_solution_probability
                    .expect(
                        "optimal probability must exist",
                    )
                    - 1.0
            )
            .abs()
                < 1.0e-12
        );
    }

    #[test]
    fn malformed_bitstring_is_rejected() {
        let generator =
            QaoaBenchmarkGenerator::new()
                .expect(
                    "generator must construct",
                );

        let request =
            request(3);

        let mut counts =
            BTreeMap::new();

        counts.insert(
            "01x".to_owned(),
            10,
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
    fn zero_shots_are_rejected() {
        let generator =
            QaoaBenchmarkGenerator::new()
                .expect(
                    "generator must construct",
                );

        let request =
            request(3);

        let mut counts =
            BTreeMap::new();

        counts.insert(
            "000".to_owned(),
            0,
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
    fn large_problem_does_not_attempt_exact_enumeration() {
        let graph =
            QaoaGraph::path(
                MAX_EXACT_REFERENCE_QUBITS
                    + 1,
            )
            .expect(
                "graph must build",
            );

        let result =
            exact_maxcut_if_available(
                &graph,
            )
            .expect(
                "reference calculation must return",
            );

        assert!(
            result.is_none()
        );
    }

    #[test]
    fn resource_count_matches_formula() {
        let graph =
            QaoaGraph::ring(4)
                .expect(
                    "graph must build",
                );

        let problem =
            QaoaProblem::new(
                graph,
                2,
                QaoaAngleSchedule::default_linear_ramp(),
                1.0,
            )
            .expect(
                "problem must build",
            );

        // n initial H + p * (3E + n) + n measurements
        //
        // 4 + 2 * (3*4 + 4) + 4 = 36.
        assert_eq!(
            problem
                .logical_gate_count()
                .expect(
                    "resource count must calculate",
                ),
            36
        );

        // p * E * 2 CX.
        assert_eq!(
            problem
                .logical_two_qubit_gate_count()
                .expect(
                    "two-qubit resource count must calculate",
                ),
            16
        );
    }
}