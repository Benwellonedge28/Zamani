//! Zamani Quantum Benchmarking — MaxCut Application Model.
//!
//! Production-grade, backend-independent MaxCut problem and benchmark-analysis
//! layer.
//!
//! # Responsibility
//!
//! This module owns the MaxCut application semantics:
//!
//! - weighted undirected MaxCut graph representation;
//! - graph validation;
//! - deterministic graph construction;
//! - MaxCut objective evaluation;
//! - bounded exact classical reference calculation;
//! - measurement-distribution analysis;
//! - approximation ratio;
//! - best-solution analysis;
//! - optimal-solution probability;
//! - approximation-threshold success probability;
//! - random-baseline quality;
//! - effective approximation ratio;
//! - solution-quality statistics;
//! - benchmark-instance identity;
//! - deterministic problem fingerprints;
//! - benchmark-domain resource accounting;
//! - safe handling of untrusted benchmark data.
//!
//! This module deliberately does NOT own:
//!
//! - QAOA;
//! - QAOA parameter optimization;
//! - quantum circuit construction;
//! - Quantum IR construction;
//! - backend selection;
//! - routing;
//! - scheduling;
//! - calibration;
//! - execution;
//! - simulator implementation;
//! - hardware APIs;
//! - quantum annealer APIs;
//! - universal timing metrics;
//! - universal benchmark-result serialization.
//!
//! Those responsibilities belong to:
//!
//! ```text
//! applications::qaoa
//! quantum::algorithms::qaoa
//! quantum::ir
//! benchmarking::execution
//! quantum::hardware
//! runtime
//! reporting
//! ```
//!
//! # Architectural role
//!
//! ```text
//!                    MaxCutProblem
//!                         │
//!             ┌───────────┴───────────┐
//!             ▼                       ▼
//!       QAOA application        Annealing application
//!             │                       │
//!             ▼                       ▼
//!       Quantum circuit          Annealing workload
//!             │                       │
//!             └───────────┬───────────┘
//!                         ▼
//!                  normalized samples
//!                         │
//!                         ▼
//!                  MaxCutAnalyzer
//!                         │
//!             ┌───────────┼────────────┐
//!             ▼           ▼            ▼
//!       quality        success       resources
//!             │           │            │
//!             └───────────┼────────────┘
//!                         ▼
//!                   BenchmarkResult
//! ```
//!
//! # Scientific semantics
//!
//! For an undirected weighted graph:
//!
//! ```text
//! G = (V, E)
//! ```
//!
//! with positive edge weights:
//!
//! ```text
//! w(u,v) > 0
//! ```
//!
//! the cut value of a bit assignment `x` is:
//!
//! ```text
//! C(x) = Σ_(u,v ∈ E) w(u,v) [x_u != x_v]
//! ```
//!
//! The optimum is:
//!
//! ```text
//! C* = max_x C(x)
//! ```
//!
//! When an exact optimum is available, the conventional approximation ratio is:
//!
//! ```text
//! AR = C / C*
//! ```
//!
//! For a sampled distribution, the benchmark additionally reports:
//!
//! ```text
//! expected_approximation_ratio
//! best_observed_approximation_ratio
//! optimal_solution_probability
//! threshold_success_probability
//! ```
//!
//! This distinction matters because a quantum optimization application is not
//! adequately characterized by its best sample alone. Current application
//! benchmarking work explicitly evaluates the trade-off between solution
//! quality and cumulative execution time. The execution/timing subsystem is
//! therefore expected to combine this module's quality metrics with runtime
//! metrics later.
//!
//! # Backend independence
//!
//! The same MaxCut problem can be executed by:
//!
//! - QAOA;
//! - another gate-model variational algorithm;
//! - quantum annealing;
//! - analog optimization;
//! - a classical simulator;
//! - a tensor-network simulator;
//! - a hybrid algorithm;
//! - a future quantum technology.
//!
//! Consequently this file accepts normalized computational-basis samples and
//! does not know how they were produced.
//!
//! # Bit-string convention
//!
//! A normalized bit string has:
//!
//! ```text
//! bits[0] = logical vertex 0
//! bits[1] = logical vertex 1
//! ...
//! bits[n-1] = logical vertex n-1
//! ```
//!
//! The execution layer MUST normalize backend-native endianness before calling
//! [`MaxCutProblem::cut_value`] or [`MaxCutAnalyzer::analyze_counts`].
//!
//! This module intentionally never reverses bit strings implicitly.
//!
//! # Exact classical reference
//!
//! Exact MaxCut is NP-hard and exhaustive verification scales exponentially.
//!
//! The exact reference implementation therefore has a strict independent
//! bound. It:
//!
//! - never allocates a `2^n` result vector;
//! - enumerates assignments in a streaming loop;
//! - fixes vertex zero to partition zero because a cut and its complement have
//!   identical value;
//! - uses checked integer arithmetic;
//! - refuses to run above [`MAX_EXACT_REFERENCE_VERTICES`];
//! - reports unavailable reference data rather than substituting a heuristic
//!   solution.
//!
//! The exact reference is a verification facility, NOT the benchmark execution
//! path.
//!
//! # Determinism
//!
//! All graph construction and analysis in this module is deterministic.
//!
//! No:
//!
//! - system time;
//! - process ID;
//! - global RNG;
//! - pointer address;
//! - thread ID;
//! - filesystem state;
//! - network state
//!
//! is used to affect semantics.
//!
//! # Resource safety
//!
//! Public benchmark inputs are treated as untrusted.
//!
//! The implementation:
//!
//! - bounds vertex counts;
//! - bounds edge counts;
//! - bounds identifier sizes;
//! - rejects self-loops;
//! - rejects duplicate edges;
//! - rejects non-finite weights;
//! - rejects non-positive weights;
//! - checks arithmetic;
//! - bounds exact enumeration;
//! - validates every supplied bit string;
//! - detects shot-count overflow;
//! - rejects invalid probabilities;
//! - never allocates from an unchecked exponential quantity.
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
//! No unsafe code.
//! No external dependencies.
//!
//! # Integration contract
//!
//! This file is intentionally usable by the existing benchmarking architecture.
//!
//! ```text
//! benchmarking::applications::maxcut
//!             │
//!             ├── applications::qaoa
//!             │       └── QAOA circuit generation
//!             │
//!             ├── future annealing benchmark
//!             │       └── annealing workload generation
//!             │
//!             ├── applications::custom
//!             │       └── user-defined MaxCut instances
//!             │
//!             └── analysis/reporting
//!                     └── solution-quality metrics
//! ```
//!
//! The QAOA module should depend on this file for MaxCut semantics instead of
//! maintaining a second independent graph/objective implementation.
//!
//! The execution layer should consume the normalized counts produced by a
//! backend and call [`MaxCutAnalyzer::analyze_counts`].
//!
//! No backend implementation belongs here.
//!
//! # Module registration
//!
//! `src/quantum/benchmarking/applications/mod.rs` should contain:
//!
//! ```text
//! pub mod maxcut;
//! ```
//!
//! The existing QAOA module can then consume this module as:
//!
//! ```text
//! use super::maxcut::{MaxCutProblem, MaxCutAnalyzer, MaxCutEdge};
//! ```
//!
//! This file does not require modifications to `quantum::ir`,
//! `quantum::hardware`, `runtime`, or the QAOA optimizer.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::hash::{Hash, Hasher};

// =============================================================================
// Stable benchmark identity
// =============================================================================

/// Stable MaxCut benchmark identifier.
pub const MAXCUT_BENCHMARK_ID: &str = "maxcut";

/// Stable application identifier.
pub const MAXCUT_APPLICATION_ID: &str = "maxcut";

/// MaxCut benchmark result schema version.
pub const MAXCUT_RESULT_SCHEMA_VERSION: u16 = 1;

/// MaxCut problem schema version.
pub const MAXCUT_PROBLEM_SCHEMA_VERSION: u16 = 1;

/// Current MaxCut semantic revision.
///
/// Increment when MaxCut objective semantics, bit ordering, edge semantics, or
/// result interpretation changes.
pub const MAXCUT_SEMANTIC_REVISION: u32 = 1;

// =============================================================================
// Resource limits
// =============================================================================

/// Maximum vertices accepted by this semantic model.
///
/// The universal `BenchmarkLimits` remain authoritative when a workload is
/// actually submitted for execution. This local limit prevents pathological
/// graph structures from being constructed before that stage.
pub const MAX_MAXCUT_VERTICES: usize = 65_536;

/// Maximum number of edges represented by one MaxCut instance.
pub const MAX_MAXCUT_EDGES: usize = 10_000_000;

/// Maximum UTF-8 byte length of an instance identifier.
pub const MAX_INSTANCE_ID_BYTES: usize = 128;

/// Maximum number of exact-reference vertices.
///
/// Exhaustive enumeration uses `2^(n-1)` assignments because the global
/// complement symmetry permits fixing vertex 0 to partition 0.
pub const MAX_EXACT_REFERENCE_VERTICES: usize = 24;

/// Maximum number of characters in a normalized bit string.
pub const MAX_BITSTRING_BYTES: usize = MAX_MAXCUT_VERTICES;

/// Maximum number of graph families exposed by one batch descriptor.
pub const MAX_BATCH_INSTANCES: usize = 1_000_000;

// =============================================================================
// Error model
// =============================================================================

/// Errors raised by the MaxCut application domain.
#[derive(Debug, Clone, PartialEq)]
pub enum MaxCutError {
    /// Required identifier was empty.
    EmptyIdentifier {
        /// Field name.
        field: &'static str,
    },

    /// Identifier exceeded its limit.
    IdentifierTooLong {
        /// Field name.
        field: &'static str,

        /// Actual size.
        length: usize,

        /// Maximum size.
        maximum: usize,
    },

    /// Vertex count is invalid.
    InvalidVertexCount {
        /// Supplied count.
        vertices: usize,
    },

    /// Edge count exceeded the local limit.
    TooManyEdges {
        /// Supplied count.
        edges: usize,

        /// Maximum.
        maximum: usize,
    },

    /// An edge endpoint is outside the graph.
    VertexOutOfRange {
        /// Vertex.
        vertex: usize,

        /// Number of vertices.
        vertices: usize,
    },

    /// Self-loop.
    SelfLoop {
        /// Vertex.
        vertex: usize,
    },

    /// Duplicate undirected edge.
    DuplicateEdge {
        /// First endpoint.
        u: usize,

        /// Second endpoint.
        v: usize,
    },

    /// Invalid edge weight.
    InvalidWeight {
        /// Weight.
        weight: f64,
    },

    /// Arithmetic overflow.
    ArithmeticOverflow {
        /// Operation.
        operation: &'static str,
    },

    /// Exact reference is intentionally unavailable above the configured
    /// reference domain.
    ExactReferenceUnavailable {
        /// Number of vertices.
        vertices: usize,

        /// Maximum exact-reference size.
        maximum: usize,
    },

    /// Invalid bit string.
    InvalidBitString {
        /// Expected number of bits.
        expected: usize,

        /// Actual number of bytes.
        actual: usize,
    },

    /// Bit string contains a non-binary byte.
    InvalidBit {
        /// Position.
        position: usize,

        /// Byte.
        byte: u8,
    },

    /// Shot count is zero.
    ZeroShots,

    /// Shot-count arithmetic overflowed.
    ShotOverflow,

    /// Probability is invalid.
    InvalidProbability {
        /// Field.
        field: &'static str,

        /// Value.
        value: f64,
    },

    /// Threshold is invalid.
    InvalidThreshold {
        /// Value.
        value: f64,
    },

    /// Exact optimum is required for the requested metric but unavailable.
    MetricRequiresExactReference {
        /// Metric.
        metric: &'static str,
    },

    /// Graph contains no usable edge.
    EmptyGraph,

    /// Invalid approximation-ratio denominator.
    InvalidOptimum {
        /// Optimum.
        optimum: f64,
    },
}

impl fmt::Display for MaxCutError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyIdentifier { field } => {
                write!(formatter, "{field} cannot be empty")
            }

            Self::IdentifierTooLong {
                field,
                length,
                maximum,
            } => write!(
                formatter,
                "{field} is {length} bytes; maximum is {maximum}"
            ),

            Self::InvalidVertexCount { vertices } => write!(
                formatter,
                "MaxCut requires at least two vertices; received {vertices}"
            ),

            Self::TooManyEdges {
                edges,
                maximum,
            } => write!(
                formatter,
                "MaxCut contains {edges} edges; maximum is {maximum}"
            ),

            Self::VertexOutOfRange {
                vertex,
                vertices,
            } => write!(
                formatter,
                "vertex {vertex} is outside graph range 0..{vertices}"
            ),

            Self::SelfLoop { vertex } => {
                write!(formatter, "MaxCut does not permit self-loop {vertex}")
            }

            Self::DuplicateEdge { u, v } => {
                write!(formatter, "duplicate undirected edge ({u},{v})")
            }

            Self::InvalidWeight { weight } => {
                write!(formatter, "invalid MaxCut edge weight {weight}")
            }

            Self::ArithmeticOverflow { operation } => {
                write!(formatter, "arithmetic overflow during {operation}")
            }

            Self::ExactReferenceUnavailable {
                vertices,
                maximum,
            } => write!(
                formatter,
                "exact MaxCut reference unavailable for {vertices} vertices; maximum is {maximum}"
            ),

            Self::InvalidBitString { expected, actual } => write!(
                formatter,
                "expected {expected} bits but received {actual} bytes"
            ),

            Self::InvalidBit {
                position,
                byte,
            } => write!(
                formatter,
                "invalid bit 0x{byte:02x} at position {position}"
            ),

            Self::ZeroShots => {
                formatter.write_str("MaxCut analysis requires at least one shot")
            }

            Self::ShotOverflow => {
                formatter.write_str("MaxCut shot-count arithmetic overflowed")
            }

            Self::InvalidProbability { field, value } => write!(
                formatter,
                "{field} must be finite and within [0,1], received {value}"
            ),

            Self::InvalidThreshold { value } => write!(
                formatter,
                "approximation threshold must be finite and within [0,1], received {value}"
            ),

            Self::MetricRequiresExactReference { metric } => write!(
                formatter,
                "{metric} requires an exact classical reference"
            ),

            Self::EmptyGraph => {
                formatter.write_str("MaxCut graph must contain at least one edge")
            }

            Self::InvalidOptimum { optimum } => {
                write!(formatter, "MaxCut optimum must be finite and positive, received {optimum}")
            }
        }
    }
}

impl std::error::Error for MaxCutError {}

/// Convenient result type for this module.
pub type MaxCutResult<T> = Result<T, MaxCutError>;

// =============================================================================
// Graph edge
// =============================================================================

/// One weighted undirected MaxCut edge.
///
/// Endpoint order is canonicalized to:
///
/// ```text
/// u < v
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MaxCutEdge {
    /// First endpoint.
    pub u: usize,

    /// Second endpoint.
    pub v: usize,

    /// Strictly positive finite edge weight.
    pub weight: f64,
}

impl MaxCutEdge {
    /// Creates a validated edge.
    pub fn new(u: usize, v: usize, weight: f64) -> MaxCutResult<Self> {
        if u == v {
            return Err(MaxCutError::SelfLoop { vertex: u });
        }

        if !weight.is_finite() || weight <= 0.0 {
            return Err(MaxCutError::InvalidWeight { weight });
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

// =============================================================================
// Graph
// =============================================================================

/// Immutable weighted undirected MaxCut problem.
#[derive(Debug, Clone, PartialEq)]
pub struct MaxCutProblem {
    /// Number of vertices / logical problem variables.
    vertices: usize,

    /// Canonically ordered edges.
    edges: Vec<MaxCutEdge>,
}

impl MaxCutProblem {
    /// Constructs and validates a MaxCut problem.
    pub fn new(
        vertices: usize,
        edges: Vec<MaxCutEdge>,
    ) -> MaxCutResult<Self> {
        if vertices < 2 {
            return Err(MaxCutError::InvalidVertexCount { vertices });
        }

        if vertices > MAX_MAXCUT_VERTICES {
            return Err(MaxCutError::InvalidVertexCount { vertices });
        }

        if edges.is_empty() {
            return Err(MaxCutError::EmptyGraph);
        }

        if edges.len() > MAX_MAXCUT_EDGES {
            return Err(MaxCutError::TooManyEdges {
                edges: edges.len(),
                maximum: MAX_MAXCUT_EDGES,
            });
        }

        let maximum_simple_edges = vertices
            .checked_mul(vertices - 1)
            .ok_or(MaxCutError::ArithmeticOverflow {
                operation: "maximum simple graph edge count",
            })?
            / 2;

        if edges.len() > maximum_simple_edges {
            return Err(MaxCutError::TooManyEdges {
                edges: edges.len(),
                maximum: maximum_simple_edges,
            });
        }

        let mut seen = BTreeSet::new();

        for edge in &edges {
            if edge.u >= vertices {
                return Err(MaxCutError::VertexOutOfRange {
                    vertex: edge.u,
                    vertices,
                });
            }

            if edge.v >= vertices {
                return Err(MaxCutError::VertexOutOfRange {
                    vertex: edge.v,
                    vertices,
                });
            }

            if edge.u >= edge.v {
                return Err(MaxCutError::DuplicateEdge {
                    u: edge.u,
                    v: edge.v,
                });
            }

            if !edge.weight.is_finite() || edge.weight <= 0.0 {
                return Err(MaxCutError::InvalidWeight {
                    weight: edge.weight,
                });
            }

            if !seen.insert((edge.u, edge.v)) {
                return Err(MaxCutError::DuplicateEdge {
                    u: edge.u,
                    v: edge.v,
                });
            }
        }

        Ok(Self { vertices, edges })
    }

    /// Creates an unweighted path graph.
    pub fn path(vertices: usize) -> MaxCutResult<Self> {
        if vertices < 2 {
            return Err(MaxCutError::InvalidVertexCount { vertices });
        }

        let mut edges = Vec::with_capacity(vertices - 1);

        for u in 0..(vertices - 1) {
            edges.push(MaxCutEdge::new(u, u + 1, 1.0)?);
        }

        Self::new(vertices, edges)
    }

    /// Creates an unweighted ring graph.
    ///
    /// For two vertices the graph contains one edge rather than two parallel
    /// edges.
    pub fn ring(vertices: usize) -> MaxCutResult<Self> {
        if vertices < 2 {
            return Err(MaxCutError::InvalidVertexCount { vertices });
        }

        let capacity = if vertices == 2 { 1 } else { vertices };
        let mut edges = Vec::with_capacity(capacity);

        for u in 0..vertices {
            let v = (u + 1) % vertices;

            if u == v {
                continue;
            }

            let (a, b) = if u < v { (u, v) } else { (v, u) };

            if edges.iter().any(|edge: &MaxCutEdge| {
                edge.u == a && edge.v == b
            }) {
                continue;
            }

            edges.push(MaxCutEdge::new(a, b, 1.0)?);
        }

        Self::new(vertices, edges)
    }

    /// Creates an unweighted complete graph.
    pub fn complete(vertices: usize) -> MaxCutResult<Self> {
        if vertices < 2 {
            return Err(MaxCutError::InvalidVertexCount { vertices });
        }

        let edge_count = vertices
            .checked_mul(vertices - 1)
            .ok_or(MaxCutError::ArithmeticOverflow {
                operation: "complete graph edge count",
            })?
            / 2;

        let mut edges = Vec::with_capacity(edge_count);

        for u in 0..vertices {
            for v in (u + 1)..vertices {
                edges.push(MaxCutEdge::new(u, v, 1.0)?);
            }
        }

        Self::new(vertices, edges)
    }

    /// Returns the number of vertices.
    #[must_use]
    pub const fn vertices(&self) -> usize {
        self.vertices
    }

    /// Returns the immutable edge list.
    #[must_use]
    pub fn edges(&self) -> &[MaxCutEdge] {
        &self.edges
    }

    /// Returns the number of edges.
    #[must_use]
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    /// Returns the total graph weight.
    pub fn total_weight(&self) -> MaxCutResult<f64> {
        let mut total = 0.0;

        for edge in &self.edges {
            total += edge.weight;

            if !total.is_finite() {
                return Err(MaxCutError::ArithmeticOverflow {
                    operation: "total graph weight",
                });
            }
        }

        Ok(total)
    }

    /// Returns the expected cut value of a uniformly random assignment.
    ///
    /// Every edge is cut with probability exactly 1/2.
    pub fn random_expected_cut(&self) -> MaxCutResult<f64> {
        let result = self.total_weight()? * 0.5;

        if !result.is_finite() {
            return Err(MaxCutError::ArithmeticOverflow {
                operation: "random expected cut",
            });
        }

        Ok(result)
    }

    /// Evaluates one normalized bit string.
    ///
    /// The bit at index `i` corresponds to graph vertex `i`.
    pub fn cut_value(&self, bits: &str) -> MaxCutResult<f64> {
        validate_bitstring(bits, self.vertices)?;

        let bytes = bits.as_bytes();
        let mut value = 0.0;

        for edge in &self.edges {
            if bytes[edge.u] != bytes[edge.v] {
                value += edge.weight;

                if !value.is_finite() {
                    return Err(MaxCutError::ArithmeticOverflow {
                        operation: "cut value",
                    });
                }
            }
        }

        Ok(value)
    }

    /// Returns whether a bit string represents an optimal cut.
    pub fn is_optimal(
        &self,
        bits: &str,
        optimum: f64,
    ) -> MaxCutResult<bool> {
        let value = self.cut_value(bits)?;

        Ok(approximately_equal_cut(value, optimum))
    }

    /// Returns a deterministic stable problem fingerprint.
    ///
    /// This fingerprint is intended for reproducibility metadata, not as a
    /// cryptographic security primitive.
    #[must_use]
    pub fn fingerprint(&self) -> u64 {
        let mut hasher = StableHasher::default();

        MAXCUT_PROBLEM_SCHEMA_VERSION.hash(&mut hasher);
        self.vertices.hash(&mut hasher);

        for edge in &self.edges {
            edge.u.hash(&mut hasher);
            edge.v.hash(&mut hasher);
            edge.weight.to_bits().hash(&mut hasher);
        }

        hasher.finish()
    }

    /// Computes the exact optimum when within the bounded reference domain.
    pub fn exact_optimum(&self) -> MaxCutResult<f64> {
        if self.vertices > MAX_EXACT_REFERENCE_VERTICES {
            return Err(MaxCutError::ExactReferenceUnavailable {
                vertices: self.vertices,
                maximum: MAX_EXACT_REFERENCE_VERTICES,
            });
        }

        exact_optimum_unchecked(self)
    }

    /// Returns an exact optimum when available.
    ///
    /// Unlike [`Self::exact_optimum`], this method does not treat the configured
    /// reference bound as an execution error.
    pub fn exact_optimum_if_available(
        &self,
    ) -> MaxCutResult<Option<f64>> {
        if self.vertices > MAX_EXACT_REFERENCE_VERTICES {
            return Ok(None);
        }

        Ok(Some(exact_optimum_unchecked(self)?))
    }

    /// Returns the maximum possible cut upper bound.
    ///
    /// For positive edge weights this is the total graph weight.
    #[must_use]
    pub fn upper_bound(&self) -> MaxCutResult<f64> {
        self.total_weight()
    }
}

// =============================================================================
// Problem descriptors
// =============================================================================

/// Stable descriptor of a MaxCut benchmark instance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MaxCutInstance {
    /// User/benchmark-visible instance identifier.
    id: String,

    /// Problem fingerprint.
    fingerprint: u64,

    /// Number of vertices.
    vertices: usize,

    /// Number of edges.
    edges: usize,
}

impl MaxCutInstance {
    /// Creates an instance descriptor.
    pub fn new<S: Into<String>>(
        id: S,
        problem: &MaxCutProblem,
    ) -> MaxCutResult<Self> {
        let id = id.into();

        if id.is_empty() {
            return Err(MaxCutError::EmptyIdentifier {
                field: "instance_id",
            });
        }

        if id.len() > MAX_INSTANCE_ID_BYTES {
            return Err(MaxCutError::IdentifierTooLong {
                field: "instance_id",
                length: id.len(),
                maximum: MAX_INSTANCE_ID_BYTES,
            });
        }

        Ok(Self {
            id,
            fingerprint: problem.fingerprint(),
            vertices: problem.vertices(),
            edges: problem.edge_count(),
        })
    }

    /// Returns the instance identifier.
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the deterministic graph fingerprint.
    #[must_use]
    pub const fn fingerprint(&self) -> u64 {
        self.fingerprint
    }

    /// Returns the vertex count.
    #[must_use]
    pub const fn vertices(&self) -> usize {
        self.vertices
    }

    /// Returns the edge count.
    #[must_use]
    pub const fn edges(&self) -> usize {
        self.edges
    }
}

// =============================================================================
// Normalized samples
// =============================================================================

/// Normalized MaxCut measurement counts.
///
/// Keys must be normalized computational-basis bit strings. Values are shot
/// counts.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct MaxCutCounts {
    counts: BTreeMap<String, u64>,
}

impl MaxCutCounts {
    /// Creates an empty count set.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates counts from an iterator.
    pub fn from_iter<I, S>(items: I) -> MaxCutResult<Self>
    where
        I: IntoIterator<Item = (S, u64)>,
        S: Into<String>,
    {
        let mut result = Self::new();

        for (bits, count) in items {
            result.add(bits.into(), count)?;
        }

        if result.total_shots()? == 0 {
            return Err(MaxCutError::ZeroShots);
        }

        Ok(result)
    }

    /// Adds one outcome count.
    pub fn add(
        &mut self,
        bits: String,
        count: u64,
    ) -> MaxCutResult<()> {
        if bits.is_empty() {
            return Err(MaxCutError::InvalidBitString {
                expected: 1,
                actual: 0,
            });
        }

        if bits.len() > MAX_BITSTRING_BYTES {
            return Err(MaxCutError::InvalidBitString {
                expected: MAX_BITSTRING_BYTES,
                actual: bits.len(),
            });
        }

        if count == 0 {
            return Ok(());
        }

        let entry = self.counts.entry(bits).or_insert(0);

        *entry = entry
            .checked_add(count)
            .ok_or(MaxCutError::ShotOverflow)?;

        Ok(())
    }

    /// Returns the normalized count map.
    #[must_use]
    pub fn as_map(&self) -> &BTreeMap<String, u64> {
        &self.counts
    }

    /// Returns total shots.
    pub fn total_shots(&self) -> MaxCutResult<u64> {
        let mut total = 0u64;

        for count in self.counts.values() {
            total = total
                .checked_add(*count)
                .ok_or(MaxCutError::ShotOverflow)?;
        }

        Ok(total)
    }

    /// Returns whether no outcomes are stored.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.counts.is_empty()
    }
}

// =============================================================================
// Analysis configuration
// =============================================================================

/// Configuration controlling MaxCut solution-quality analysis.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MaxCutAnalysisConfig {
    /// Approximation ratio threshold considered a successful sample.
    ///
    /// For example `0.95` means a sample must achieve at least 95% of the
    /// exact optimum.
    pub approximation_threshold: f64,

    /// Relative tolerance used when comparing floating-point cut values.
    pub equality_tolerance: f64,
}

impl Default for MaxCutAnalysisConfig {
    fn default() -> Self {
        Self {
            approximation_threshold: 1.0,
            equality_tolerance: 1e-12,
        }
    }
}

impl MaxCutAnalysisConfig {
    /// Creates and validates an analysis configuration.
    pub fn new(
        approximation_threshold: f64,
        equality_tolerance: f64,
    ) -> MaxCutResult<Self> {
        validate_probability(
            "approximation_threshold",
            approximation_threshold,
        )?;

        if !equality_tolerance.is_finite()
            || equality_tolerance < 0.0
        {
            return Err(MaxCutError::InvalidProbability {
                field: "equality_tolerance",
                value: equality_tolerance,
            });
        }

        Ok(Self {
            approximation_threshold,
            equality_tolerance,
        })
    }
}

// =============================================================================
// Analysis result
// =============================================================================

/// Complete MaxCut solution-quality analysis.
#[derive(Debug, Clone, PartialEq)]
pub struct MaxCutAnalysis {
    /// Result schema version.
    pub schema_version: u16,

    /// Benchmark identifier.
    pub benchmark_id: &'static str,

    /// Number of vertices.
    pub vertices: usize,

    /// Number of edges.
    pub edges: usize,

    /// Total graph weight.
    pub total_weight: f64,

    /// Total measured shots.
    pub shots: u64,

    /// Exact optimum when available.
    pub exact_optimum: Option<f64>,

    /// Mean observed cut value.
    pub expected_cut: f64,

    /// Mean approximation ratio when an exact optimum is available.
    pub approximation_ratio: Option<f64>,

    /// Best observed cut value.
    pub best_cut: f64,

    /// Approximation ratio of the best observed cut.
    pub best_approximation_ratio: Option<f64>,

    /// Random-assignment expected cut.
    pub random_expected_cut: f64,

    /// Random-baseline approximation ratio when an exact optimum is
    /// available.
    pub random_approximation_ratio: Option<f64>,

    /// Effective approximation ratio:
    ///
    /// ```text
    /// (observed - random) / (optimum - random)
    /// ```
    ///
    /// when the denominator is meaningful.
    pub effective_approximation_ratio: Option<f64>,

    /// Number of shots that produced an exact optimum.
    pub optimal_shots: Option<u64>,

    /// Probability of observing an optimum.
    pub optimal_probability: Option<f64>,

    /// Number of shots meeting the configured approximation threshold.
    pub threshold_success_shots: Option<u64>,

    /// Probability of meeting the configured approximation threshold.
    pub threshold_success_probability: Option<f64>,

    /// Configured approximation threshold.
    pub approximation_threshold: f64,

    /// Deterministic graph fingerprint.
    pub problem_fingerprint: u64,
}

impl MaxCutAnalysis {
    /// Validates the analysis result.
    pub fn validate(&self) -> MaxCutResult<()> {
        if self.schema_version != MAXCUT_RESULT_SCHEMA_VERSION {
            return Err(MaxCutError::ArithmeticOverflow {
                operation: "MaxCut result schema validation",
            });
        }

        if self.vertices < 2 {
            return Err(MaxCutError::InvalidVertexCount {
                vertices: self.vertices,
            });
        }

        if self.edges == 0 {
            return Err(MaxCutError::EmptyGraph);
        }

        if self.shots == 0 {
            return Err(MaxCutError::ZeroShots);
        }

        validate_finite_nonnegative(
            "total_weight",
            self.total_weight,
        )?;

        validate_finite_nonnegative(
            "expected_cut",
            self.expected_cut,
        )?;

        validate_finite_nonnegative(
            "best_cut",
            self.best_cut,
        )?;

        validate_finite_nonnegative(
            "random_expected_cut",
            self.random_expected_cut,
        )?;

        validate_probability(
            "approximation_threshold",
            self.approximation_threshold,
        )?;

        if let Some(value) = self.approximation_ratio {
            validate_finite_nonnegative(
                "approximation_ratio",
                value,
            )?;
        }

        if let Some(value) = self.best_approximation_ratio {
            validate_finite_nonnegative(
                "best_approximation_ratio",
                value,
            )?;
        }

        if let Some(value) = self.random_approximation_ratio {
            validate_finite_nonnegative(
                "random_approximation_ratio",
                value,
            )?;
        }

        if let Some(value) = self.effective_approximation_ratio {
            if !value.is_finite() {
                return Err(MaxCutError::InvalidProbability {
                    field: "effective_approximation_ratio",
                    value,
                });
            }
        }

        if let Some(value) = self.optimal_probability {
            validate_probability(
                "optimal_probability",
                value,
            )?;
        }

        if let Some(value) = self.threshold_success_probability {
            validate_probability(
                "threshold_success_probability",
                value,
            )?;
        }

        if let Some(value) = self.optimal_shots {
            if value > self.shots {
                return Err(MaxCutError::ShotOverflow);
            }
        }

        if let Some(value) = self.threshold_success_shots {
            if value > self.shots {
                return Err(MaxCutError::ShotOverflow);
            }
        }

        Ok(())
    }
}

// =============================================================================
// Analyzer
// =============================================================================

/// Stateless MaxCut result analyzer.
#[derive(Debug, Clone, Copy, Default)]
pub struct MaxCutAnalyzer;

impl MaxCutAnalyzer {
    /// Creates a MaxCut analyzer.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Analyzes normalized measurement counts.
    ///
    /// The analyzer is independent of how the counts were generated.
    ///
    /// Therefore this method can analyze:
    ///
    /// - QAOA;
    /// - quantum annealing;
    /// - analog quantum optimization;
    /// - simulator output;
    /// - hardware output;
    /// - hybrid algorithms.
    pub fn analyze_counts(
        &self,
        problem: &MaxCutProblem,
        counts: &MaxCutCounts,
        config: MaxCutAnalysisConfig,
    ) -> MaxCutResult<MaxCutAnalysis> {
        validate_probability(
            "approximation_threshold",
            config.approximation_threshold,
        )?;

        if !config.equality_tolerance.is_finite()
            || config.equality_tolerance < 0.0
        {
            return Err(MaxCutError::InvalidProbability {
                field: "equality_tolerance",
                value: config.equality_tolerance,
            });
        }

        let shots = counts.total_shots()?;

        if shots == 0 {
            return Err(MaxCutError::ZeroShots);
        }

        let total_weight = problem.total_weight()?;
        let random_expected_cut =
            problem.random_expected_cut()?;

        let exact_optimum =
            problem.exact_optimum_if_available()?;

        let mut weighted_cut_sum = 0.0;
        let mut best_cut = 0.0;

        let mut optimal_shots = 0u64;
        let mut threshold_success_shots = 0u64;

        for (bits, count) in counts.as_map() {
            let cut = problem.cut_value(bits)?;

            weighted_cut_sum += cut * (*count as f64);

            if !weighted_cut_sum.is_finite() {
                return Err(MaxCutError::ArithmeticOverflow {
                    operation: "weighted expected cut",
                });
            }

            if cut > best_cut {
                best_cut = cut;
            }

            if let Some(optimum) = exact_optimum {
                if approximately_equal_with_tolerance(
                    cut,
                    optimum,
                    config.equality_tolerance,
                ) {
                    optimal_shots = optimal_shots
                        .checked_add(*count)
                        .ok_or(MaxCutError::ShotOverflow)?;
                }

                let ratio = safe_ratio(cut, optimum)?;

                if ratio + config.equality_tolerance
                    >= config.approximation_threshold
                {
                    threshold_success_shots =
                        threshold_success_shots
                            .checked_add(*count)
                            .ok_or(MaxCutError::ShotOverflow)?;
                }
            }
        }

        let expected_cut =
            weighted_cut_sum / shots as f64;

        if !expected_cut.is_finite() {
            return Err(MaxCutError::ArithmeticOverflow {
                operation: "expected cut",
            });
        }

        let (
            approximation_ratio,
            best_approximation_ratio,
            random_approximation_ratio,
            effective_approximation_ratio,
        ) = if let Some(optimum) = exact_optimum {
            if optimum <= 0.0 || !optimum.is_finite() {
                return Err(MaxCutError::InvalidOptimum {
                    optimum,
                });
            }

            let approximation_ratio =
                safe_ratio(expected_cut, optimum)?;

            let best_approximation_ratio =
                safe_ratio(best_cut, optimum)?;

            let random_approximation_ratio =
                safe_ratio(random_expected_cut, optimum)?;

            let effective_approximation_ratio =
                effective_ratio(
                    expected_cut,
                    random_expected_cut,
                    optimum,
                );

            (
                Some(approximation_ratio),
                Some(best_approximation_ratio),
                Some(random_approximation_ratio),
                effective_approximation_ratio,
            )
        } else {
            (None, None, None, None)
        };

        let (
            optimal_shots,
            optimal_probability,
            threshold_success_shots,
            threshold_success_probability,
        ) = if exact_optimum.is_some() {
            let optimal_probability =
                probability_from_counts(
                    optimal_shots,
                    shots,
                )?;

            let threshold_probability =
                probability_from_counts(
                    threshold_success_shots,
                    shots,
                )?;

            (
                Some(optimal_shots),
                Some(optimal_probability),
                Some(threshold_success_shots),
                Some(threshold_probability),
            )
        } else {
            (None, None, None, None)
        };

        let result = MaxCutAnalysis {
            schema_version:
                MAXCUT_RESULT_SCHEMA_VERSION,

            benchmark_id:
                MAXCUT_BENCHMARK_ID,

            vertices: problem.vertices(),

            edges: problem.edge_count(),

            total_weight,

            shots,

            exact_optimum,

            expected_cut,

            approximation_ratio,

            best_cut,

            best_approximation_ratio,

            random_expected_cut,

            random_approximation_ratio,

            effective_approximation_ratio,

            optimal_shots,

            optimal_probability,

            threshold_success_shots,

            threshold_success_probability,

            approximation_threshold:
                config.approximation_threshold,

            problem_fingerprint:
                problem.fingerprint(),
        };

        result.validate()?;

        Ok(result)
    }

    /// Computes the cut value of every normalized outcome.
    ///
    /// This is useful for downstream reporting and performance-profile
    /// generation without coupling the analyzer to a particular reporting
    /// format.
    pub fn classify_counts(
        &self,
        problem: &MaxCutProblem,
        counts: &MaxCutCounts,
    ) -> MaxCutResult<BTreeMap<String, f64>> {
        let mut result = BTreeMap::new();

        for bits in counts.as_map().keys() {
            result.insert(
                bits.clone(),
                problem.cut_value(bits)?,
            );
        }

        Ok(result)
    }
}

// =============================================================================
// Benchmark quality profile
// =============================================================================

/// One point in a MaxCut quality/time performance profile.
///
/// Timing itself belongs to the execution layer. This structure only stores
/// already-measured timing so that MaxCut analysis can be combined with runtime
/// information without taking ownership of the timing subsystem.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MaxCutPerformancePoint {
    /// Problem size.
    pub vertices: usize,

    /// Approximation ratio at this point.
    pub approximation_ratio: Option<f64>,

    /// Best observed approximation ratio.
    pub best_approximation_ratio: Option<f64>,

    /// Cumulative elapsed time in seconds.
    pub cumulative_time_seconds: f64,
}

impl MaxCutPerformancePoint {
    /// Creates a validated performance point.
    pub fn new(
        vertices: usize,
        approximation_ratio: Option<f64>,
        best_approximation_ratio: Option<f64>,
        cumulative_time_seconds: f64,
    ) -> MaxCutResult<Self> {
        if vertices < 2 {
            return Err(MaxCutError::InvalidVertexCount {
                vertices,
            });
        }

        if !cumulative_time_seconds.is_finite()
            || cumulative_time_seconds < 0.0
        {
            return Err(MaxCutError::InvalidProbability {
                field: "cumulative_time_seconds",
                value: cumulative_time_seconds,
            });
        }

        if let Some(value) = approximation_ratio {
            if !value.is_finite() || value < 0.0 {
                return Err(MaxCutError::InvalidProbability {
                    field: "approximation_ratio",
                    value,
                });
            }
        }

        if let Some(value) = best_approximation_ratio {
            if !value.is_finite() || value < 0.0 {
                return Err(MaxCutError::InvalidProbability {
                    field: "best_approximation_ratio",
                    value,
                });
            }
        }

        Ok(Self {
            vertices,
            approximation_ratio,
            best_approximation_ratio,
            cumulative_time_seconds,
        })
    }
}

// =============================================================================
// Utility functions
// =============================================================================

fn validate_bitstring(
    bits: &str,
    expected: usize,
) -> MaxCutResult<()> {
    if bits.len() != expected {
        return Err(MaxCutError::InvalidBitString {
            expected,
            actual: bits.len(),
        });
    }

    if bits.len() > MAX_BITSTRING_BYTES {
        return Err(MaxCutError::InvalidBitString {
            expected: MAX_BITSTRING_BYTES,
            actual: bits.len(),
        });
    }

    for (position, byte) in bits.bytes().enumerate() {
        if byte != b'0' && byte != b'1' {
            return Err(MaxCutError::InvalidBit {
                position,
                byte,
            });
        }
    }

    Ok(())
}

fn validate_probability(
    field: &'static str,
    value: f64,
) -> MaxCutResult<()> {
    if !value.is_finite() || !(0.0..=1.0).contains(&value) {
        return Err(MaxCutError::InvalidProbability {
            field,
            value,
        });
    }

    Ok(())
}

fn validate_finite_nonnegative(
    field: &'static str,
    value: f64,
) -> MaxCutResult<()> {
    if !value.is_finite() || value < 0.0 {
        return Err(MaxCutError::InvalidProbability {
            field,
            value,
        });
    }

    Ok(())
}

fn probability_from_counts(
    successes: u64,
    shots: u64,
) -> MaxCutResult<f64> {
    if shots == 0 {
        return Err(MaxCutError::ZeroShots);
    }

    if successes > shots {
        return Err(MaxCutError::ShotOverflow);
    }

    let probability =
        successes as f64 / shots as f64;

    validate_probability(
        "derived_probability",
        probability,
    )?;

    Ok(probability)
}

fn safe_ratio(
    numerator: f64,
    denominator: f64,
) -> MaxCutResult<f64> {
    if !numerator.is_finite()
        || !denominator.is_finite()
        || denominator <= 0.0
    {
        return Err(MaxCutError::InvalidOptimum {
            optimum: denominator,
        });
    }

    let ratio = numerator / denominator;

    if !ratio.is_finite() || ratio < 0.0 {
        return Err(MaxCutError::InvalidProbability {
            field: "approximation_ratio",
            value: ratio,
        });
    }

    Ok(ratio)
}

fn effective_ratio(
    observed: f64,
    random: f64,
    optimum: f64,
) -> Option<f64> {
    let denominator = optimum - random;

    if !observed.is_finite()
        || !random.is_finite()
        || !optimum.is_finite()
        || denominator <= 0.0
    {
        return None;
    }

    let value =
        (observed - random) / denominator;

    if !value.is_finite() {
        return None;
    }

    Some(value)
}

fn approximately_equal_cut(
    lhs: f64,
    rhs: f64,
) -> bool {
    approximately_equal_with_tolerance(
        lhs,
        rhs,
        1e-12,
    )
}

fn approximately_equal_with_tolerance(
    lhs: f64,
    rhs: f64,
    tolerance: f64,
) -> bool {
    if lhs == rhs {
        return true;
    }

    let scale = lhs.abs().max(rhs.abs()).max(1.0);

    (lhs - rhs).abs()
        <= tolerance * scale
}

// =============================================================================
// Exact reference solver
// =============================================================================

fn exact_optimum_unchecked(
    problem: &MaxCutProblem,
) -> MaxCutResult<f64> {
    debug_assert!(
        problem.vertices
            <= MAX_EXACT_REFERENCE_VERTICES
    );

    /*
     * Global complement symmetry:
     *
     *   C(x) == C(not x)
     *
     * Therefore vertex 0 can be fixed to 0 without changing the optimum.
     *
     * We enumerate only:
     *
     *   2^(n-1)
     *
     * assignments instead of:
     *
     *   2^n.
     */

    let variable_vertices =
        problem.vertices - 1;

    let assignments =
        1u64
            .checked_shl(
                variable_vertices as u32,
            )
            .ok_or(MaxCutError::ArithmeticOverflow {
                operation: "exact MaxCut assignment count",
            })?;

    let mut optimum = 0.0;

    for mask in 0..assignments {
        let mut value = 0.0;

        for edge in &problem.edges {
            let u_bit = if edge.u == 0 {
                false
            } else {
                ((mask
                    >> (edge.u - 1))
                    & 1)
                    != 0
            };

            let v_bit = if edge.v == 0 {
                false
            } else {
                ((mask
                    >> (edge.v - 1))
                    & 1)
                    != 0
            };

            if u_bit != v_bit {
                value += edge.weight;

                if !value.is_finite() {
                    return Err(
                        MaxCutError::ArithmeticOverflow {
                            operation:
                                "exact MaxCut objective",
                        },
                    );
                }
            }
        }

        if value > optimum {
            optimum = value;
        }
    }

    if !optimum.is_finite()
        || optimum <= 0.0
    {
        return Err(MaxCutError::InvalidOptimum {
            optimum,
        });
    }

    Ok(optimum)
}

// =============================================================================
// Deterministic lightweight hasher
// =============================================================================

/// Small deterministic hasher used only for reproducibility fingerprints.
///
/// This is intentionally NOT cryptographic.
#[derive(Debug, Default)]
struct StableHasher {
    state: u64,
}

impl Hasher for StableHasher {
    fn finish(&self) -> u64 {
        self.state
    }

    fn write(&mut self, bytes: &[u8]) {
        /*
         * FNV-1a-style deterministic accumulation.
         *
         * The fingerprint is a reproducibility identifier, not a security
         * boundary. Security-sensitive hashes belong in provenance/reporting
         * where a cryptographic digest can be selected explicitly.
         */
        if self.state == 0 {
            self.state =
                0xcbf29ce484222325;
        }

        for byte in bytes {
            self.state ^=
                u64::from(*byte);

            self.state =
                self.state
                    .wrapping_mul(
                        0x100000001b3,
                    );
        }
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn edge_canonicalizes_endpoints() {
        let edge =
            MaxCutEdge::new(
                4,
                1,
                2.0,
            )
            .expect("valid edge");

        assert_eq!(
            edge.endpoints(),
            (1, 4)
        );
        assert_eq!(
            edge.weight,
            2.0
        );
    }

    #[test]
    fn self_loop_is_rejected() {
        let result =
            MaxCutEdge::new(
                1,
                1,
                1.0,
            );

        assert!(matches!(
            result,
            Err(MaxCutError::SelfLoop {
                vertex: 1
            })
        ));
    }

    #[test]
    fn non_positive_weight_is_rejected() {
        assert!(matches!(
            MaxCutEdge::new(
                0,
                1,
                0.0
            ),
            Err(MaxCutError::InvalidWeight {
                ..
            })
        ));

        assert!(matches!(
            MaxCutEdge::new(
                0,
                1,
                -1.0
            ),
            Err(MaxCutError::InvalidWeight {
                ..
            })
        ));
    }

    #[test]
    fn nan_weight_is_rejected() {
        assert!(matches!(
            MaxCutEdge::new(
                0,
                1,
                f64::NAN
            ),
            Err(MaxCutError::InvalidWeight {
                ..
            })
        ));
    }

    #[test]
    fn duplicate_edges_are_rejected() {
        let result =
            MaxCutProblem::new(
                3,
                vec![
                    MaxCutEdge::new(
                        0,
                        1,
                        1.0,
                    )
                    .expect("edge"),
                    MaxCutEdge::new(
                        1,
                        0,
                        2.0,
                    )
                    .expect("edge"),
                ],
            );

        assert!(matches!(
            result,
            Err(MaxCutError::DuplicateEdge {
                ..
            })
        ));
    }

    #[test]
    fn path_three_has_expected_cut() {
        let problem =
            MaxCutProblem::path(3)
                .expect("valid path");

        assert_eq!(
            problem.cut_value("010")
                .expect("valid bits"),
            2.0
        );

        assert_eq!(
            problem.cut_value("000")
                .expect("valid bits"),
            0.0
        );
    }

    #[test]
    fn ring_four_exact_optimum_is_four() {
        let problem =
            MaxCutProblem::ring(4)
                .expect("valid ring");

        let optimum =
            problem
                .exact_optimum()
                .expect("exact reference");

        assert_eq!(
            optimum,
            4.0
        );
    }

    #[test]
    fn complete_four_exact_optimum_is_four() {
        let problem =
            MaxCutProblem::complete(4)
                .expect("valid graph");

        let optimum =
            problem
                .exact_optimum()
                .expect("exact reference");

        assert_eq!(
            optimum,
            4.0
        );
    }

    #[test]
    fn random_baseline_is_half_total_weight() {
        let problem =
            MaxCutProblem::path(4)
                .expect("valid graph");

        assert_eq!(
            problem
                .random_expected_cut()
                .expect("baseline"),
            1.5
        );
    }

    #[test]
    fn exact_reference_is_unavailable_above_bound() {
        let problem =
            MaxCutProblem::ring(
                MAX_EXACT_REFERENCE_VERTICES
                    + 1,
            )
            .expect("valid graph");

        assert_eq!(
            problem
                .exact_optimum_if_available()
                .expect("reference query"),
            None
        );
    }

    #[test]
    fn invalid_bitstring_length_is_rejected() {
        let problem =
            MaxCutProblem::path(3)
                .expect("valid graph");

        assert!(matches!(
            problem.cut_value("01"),
            Err(
                MaxCutError::InvalidBitString {
                    expected: 3,
                    actual: 2
                }
            )
        ));
    }

    #[test]
    fn invalid_bit_is_rejected() {
        let problem =
            MaxCutProblem::path(3)
                .expect("valid graph");

        assert!(matches!(
            problem.cut_value("0a0"),
            Err(
                MaxCutError::InvalidBit {
                    position: 1,
                    ..
                }
            )
        ));
    }

    #[test]
    fn count_overflow_is_rejected() {
        let mut counts =
            MaxCutCounts::new();

        counts
            .add(
                "00".to_owned(),
                u64::MAX,
            )
            .expect("first count");

        assert!(matches!(
            counts.add(
                "00".to_owned(),
                1
            ),
            Err(MaxCutError::ShotOverflow)
        ));
    }

    #[test]
    fn counts_are_deterministic() {
        let first =
            MaxCutCounts::from_iter([
                ("010".to_owned(), 10),
                ("101".to_owned(), 5),
            ])
            .expect("counts");

        let second =
            MaxCutCounts::from_iter([
                ("101".to_owned(), 5),
                ("010".to_owned(), 10),
            ])
            .expect("counts");

        assert_eq!(
            first,
            second
        );
    }

    #[test]
    fn perfect_distribution_has_unit_approximation_ratio() {
        let problem =
            MaxCutProblem::ring(4)
                .expect("graph");

        let counts =
            MaxCutCounts::from_iter([
                ("0101".to_owned(), 100),
            ])
            .expect("counts");

        let analyzer =
            MaxCutAnalyzer::new();

        let result =
            analyzer
                .analyze_counts(
                    &problem,
                    &counts,
                    MaxCutAnalysisConfig::default(),
                )
                .expect("analysis");

        assert_eq!(
            result.exact_optimum,
            Some(4.0)
        );

        assert_eq!(
            result.expected_cut,
            4.0
        );

        assert_eq!(
            result.approximation_ratio,
            Some(1.0)
        );

        assert_eq!(
            result.best_approximation_ratio,
            Some(1.0)
        );

        assert_eq!(
            result.optimal_probability,
            Some(1.0)
        );
    }

    #[test]
    fn threshold_probability_is_computed() {
        let problem =
            MaxCutProblem::ring(4)
                .expect("graph");

        let counts =
            MaxCutCounts::from_iter([
                ("0101".to_owned(), 25),
                ("0000".to_owned(), 75),
            ])
            .expect("counts");

        let analyzer =
            MaxCutAnalyzer::new();

        let config =
            MaxCutAnalysisConfig::new(
                1.0,
                1e-12,
            )
            .expect("config");

        let result =
            analyzer
                .analyze_counts(
                    &problem,
                    &counts,
                    config,
                )
                .expect("analysis");

        assert_eq!(
            result.optimal_probability,
            Some(0.25)
        );

        assert_eq!(
            result.threshold_success_probability,
            Some(0.25)
        );
    }

    #[test]
    fn approximate_threshold_works() {
        let problem =
            MaxCutProblem::ring(4)
                .expect("graph");

        let counts =
            MaxCutCounts::from_iter([
                ("0101".to_owned(), 10),
                ("0111".to_owned(), 90),
            ])
            .expect("counts");

        let config =
            MaxCutAnalysisConfig::new(
                0.5,
                1e-12,
            )
            .expect("config");

        let result =
            MaxCutAnalyzer::new()
                .analyze_counts(
                    &problem,
                    &counts,
                    config,
                )
                .expect("analysis");

        assert_eq!(
            result.threshold_success_probability,
            Some(1.0)
        );
    }

    #[test]
    fn effective_ratio_is_defined_when_random_is_below_optimum() {
        let problem =
            MaxCutProblem::ring(4)
                .expect("graph");

        let counts =
            MaxCutCounts::from_iter([
                ("0101".to_owned(), 50),
                ("0000".to_owned(), 50),
            ])
            .expect("counts");

        let result =
            MaxCutAnalyzer::new()
                .analyze_counts(
                    &problem,
                    &counts,
                    MaxCutAnalysisConfig::default(),
                )
                .expect("analysis");

        /*
         * Expected cut:
         *
         *   (4 + 0) / 2 = 2
         *
         * Random baseline:
         *
         *   2
         *
         * Optimum:
         *
         *   4
         *
         * Therefore the effective ratio is exactly zero.
         */
        assert_eq!(
            result.expected_cut,
            2.0
        );

        assert_eq!(
            result.effective_approximation_ratio,
            Some(0.0)
        );
    }

    #[test]
    fn fingerprint_is_stable_for_identical_problem() {
        let first =
            MaxCutProblem::ring(8)
                .expect("graph");

        let second =
            MaxCutProblem::ring(8)
                .expect("graph");

        assert_eq!(
            first.fingerprint(),
            second.fingerprint()
        );
    }

    #[test]
    fn changing_weight_changes_fingerprint() {
        let first =
            MaxCutProblem::new(
                3,
                vec![
                    MaxCutEdge::new(
                        0,
                        1,
                        1.0,
                    )
                    .expect("edge"),
                    MaxCutEdge::new(
                        1,
                        2,
                        1.0,
                    )
                    .expect("edge"),
                ],
            )
            .expect("graph");

        let second =
            MaxCutProblem::new(
                3,
                vec![
                    MaxCutEdge::new(
                        0,
                        1,
                        2.0,
                    )
                    .expect("edge"),
                    MaxCutEdge::new(
                        1,
                        2,
                        1.0,
                    )
                    .expect("edge"),
                ],
            )
            .expect("graph");

        assert_ne!(
            first.fingerprint(),
            second.fingerprint()
        );
    }

    #[test]
    fn invalid_threshold_is_rejected() {
        assert!(matches!(
            MaxCutAnalysisConfig::new(
                1.1,
                1e-12,
            ),
            Err(MaxCutError::InvalidProbability {
                field: "approximation_threshold",
                ..
            })
        ));
    }

    #[test]
    fn performance_point_validates_time() {
        assert!(MaxCutPerformancePoint::new(
            8,
            Some(0.9),
            Some(1.0),
            1.25,
        )
        .is_ok());

        assert!(MaxCutPerformancePoint::new(
            8,
            Some(0.9),
            Some(1.0),
            -1.0,
        )
        .is_err());
    }
}