//! Zamani Quantum Optimization — Dependency Analysis
//!
//! Production dependency analysis for the canonical Zamani Quantum IR.
//!
//! # Architectural boundary
//!
//! This module determines the logical execution dependencies between
//! operations in a [`crate::quantum::ir::QuantumCircuit`].
//!
//! The canonical representations remain:
//!
//! ```text
//! crate::quantum::ir::QuantumCircuit
//! crate::quantum::ir::Gate
//! ```
//!
//! This module does NOT define another circuit representation, another gate
//! representation, routing information, physical topology, pulse timing,
//! hardware dependencies, or execution scheduling.
//!
//! # Dependency model
//!
//! For the current Quantum IR, an operation depends on:
//!
//! 1. the latest preceding operation touching each of its logical qubits;
//! 2. the latest preceding write to the same classical measurement target;
//! 3. explicit barrier semantics through the qubits covered by the barrier;
//! 4. measurement/reset semantic boundaries through their ordinary qubit
//!    dependencies and explicit dependency reasons.
//!
//! A multi-qubit operation therefore creates a synchronization point across
//! all of its operands.
//!
//! For example:
//!
//! ```text
//! q0: A ────────┐
//!               ├── C
//! q1: B ────────┘
//! ```
//!
//! produces:
//!
//! ```text
//! A ──┐
//!     ├── C
//! B ──┘
//! ```
//!
//! If `A` is the latest operation on both operands, only one logical edge is
//! materialized. Duplicate edges are never emitted.
//!
//! # Classical dependencies
//!
//! The current canonical [`Gate`] model exposes classical destinations for
//! measurements, but does not yet expose classical reads/conditional-control
//! dependencies. Therefore this module tracks classical write ordering only.
//!
//! When the canonical IR later gains explicit classical-read/control operands,
//! those operands should be lowered into this dependency graph as additional
//! dependency domains without changing the graph's public storage model.
//!
//! # Barriers
//!
//! A barrier is represented by the canonical IR as an operation over one or
//! more qubits. The normal per-qubit dependency mechanism gives the barrier its
//! incoming dependencies and makes subsequent operations depend on it.
//!
//! Consequently:
//!
//! ```text
//! A(q0) ──┐
//!         ├── Barrier(q0,q1) ── C(q1)
//! B(q1) ──┘
//! ```
//!
//! correctly establishes:
//!
//! ```text
//! A ──┐
//!     ├── Barrier ── C
//! B ──┘
//! ```
//!
//! This is important because treating barriers as merely metadata would allow
//! later optimization passes to cross a semantic boundary accidentally.
//!
//! # Representation
//!
//! The graph uses compressed sparse row-style adjacency storage rather than
//! `Vec<Vec<...>>`.
//!
//! This gives:
//!
//! - O(V + E) construction;
//! - O(1) operation lookup;
//! - O(out-degree) successor iteration;
//! - O(in-degree) predecessor iteration;
//! - deterministic adjacency ordering;
//! - substantially lower per-operation allocation overhead;
//! - predictable memory proportional to the actual graph;
//! - no hash-map iteration nondeterminism.
//!
//! `V` is the number of operations and `E` is the number of dependency edges.
//!
//! # Scaling
//!
//! There is deliberately no artificial fixed circuit-size ceiling in this
//! module.
//!
//! The practical maximum is determined by:
//!
//! ```text
//! QuantumIrLimits
//!         +
//! OptimizationLimits
//!         +
//! available memory
//!         +
//! available CPU
//! ```
//!
//! The analysis enforces:
//!
//! - canonical IR validity;
//! - optimizer circuit-qubit limits;
//! - optimizer circuit-operation limits;
//! - optimizer analysis-work limits;
//! - optimizer dependency-edge limits;
//! - checked integer arithmetic;
//! - fallible vector reservation.
//!
//! No recursion is used.
//!
//! No quadratic all-operation comparison is used.
//!
//! No `unsafe` code is used.
//!
//! # Determinism
//!
//! The same validated circuit and the same limits always produce the same
//! graph.
//!
//! Edges are ordered by operation index.
//!
//! No hash-based collection participates in graph construction.
//!
//! # Integration contract
//!
//! This file is intentionally independent of future analysis modules.
//!
//! `depth.rs` can consume:
//!
//! ```text
//! DependencyGraph::predecessors()
//! DependencyGraph::successors()
//! DependencyGraph::topological_order()
//! ```
//!
//! `commutation.rs` can use the graph to distinguish independent operations
//! from operations separated by a true dependency.
//!
//! `liveness.rs` can use predecessor/successor information to determine last
//! uses.
//!
//! `critical_path.rs` can calculate longest paths from this graph.
//!
//! `scheduler.rs` can consume the graph as a logical dependency DAG, while
//! remaining the owner of execution scheduling.
//!
//! `pipeline.rs` can cache the graph in `OptimizationContext` using its typed
//! analysis storage.
//!
//! `context.rs` does not need to be modified merely because this analysis is
//! added.
//!
//! `statistics.rs` can report vertex/edge counts without knowing graph
//! internals.
//!
//! `provenance.rs` can identify the analysis by:
//!
//! ```text
//! quantum.optimization.analysis.dependency
//! ```
//!
//! # Important ownership rule
//!
//! This module does not modify the input circuit.
//!
//! The dependency graph is an invocation-local analysis artifact.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021.
//!
//! No nightly features.
//! No external dependencies.
//! No `unsafe` code.
//!
//! # Complexity
//!
//! Let:
//!
//! - V = operation count;
//! - Q = total logical-qubit operand references;
//! - C = number of classical measurement destinations;
//! - E = dependency-edge count.
//!
//! Construction:
//!
//! ```text
//! O(V + Q + C + E)
//! ```
//!
//! Memory:
//!
//! ```text
//! O(V + E + Nq + Nc)
//! ```
//!
//! where `Nq` is the logical-qubit namespace size and `Nc` is the classical
//! namespace size.
//!
//! The implementation deliberately avoids an O(V²) dependency search.
//!
//! # Public contract
//!
//! The primary entry points are:
//!
//! ```ignore
//! DependencyGraph::analyze(&circuit)
//! DependencyGraph::analyze_with_limits(&circuit, &limits)
//! DependencyGraph::analyze_view(&view, &limits)
//! ```
//!
//! The graph can then be queried without exposing mutable internal storage.

use std::fmt;

use crate::quantum::ir::{Gate, QuantumCircuit};

use super::super::circuit::{
    CircuitView,
    OperationId,
};
use super::super::limits::{
    OptimizationLimits,
    OptimizationLimitsError,
};

// ============================================================================
// Public identifiers
// ============================================================================

/// Stable identifier for this analysis implementation.
///
/// This identifier is suitable for analysis registries, provenance records,
/// diagnostics, and typed analysis caches.
pub const DEPENDENCY_ANALYSIS_ID: &str =
    "quantum.optimization.analysis.dependency";

/// Semantic version of the dependency-analysis contract.
///
/// This is independent of the Quantum IR schema version and compiler version.
pub const DEPENDENCY_ANALYSIS_VERSION: u32 = 1;

// ============================================================================
// Dependency reasons
// ============================================================================

/// Bit flags describing why one operation depends on another.
///
/// The type is intentionally implemented without an external bitflags
/// dependency so this foundational analysis remains dependency-free.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct DependencyReasons {
    bits: u8,
}

impl DependencyReasons {
    /// Dependency caused by sharing a logical qubit.
    pub const QUBIT: u8 = 1 << 0;

    /// Dependency caused by reusing the same classical destination.
    pub const CLASSICAL: u8 = 1 << 1;

    /// Dependency involving an explicit barrier.
    pub const BARRIER: u8 = 1 << 2;

    /// Dependency involving a measurement boundary.
    pub const MEASUREMENT: u8 = 1 << 3;

    /// Dependency involving a reset boundary.
    pub const RESET: u8 = 1 << 4;

    /// Creates an empty reason set.
    #[must_use]
    pub const fn empty() -> Self {
        Self { bits: 0 }
    }

    /// Creates a reason set containing one reason.
    #[must_use]
    pub const fn from_bit(bit: u8) -> Self {
        Self { bits: bit }
    }

    /// Returns the raw bit representation.
    #[must_use]
    pub const fn bits(self) -> u8 {
        self.bits
    }

    /// Returns true when no dependency reason is present.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.bits == 0
    }

    /// Adds one or more dependency reasons.
    #[must_use]
    pub const fn with_bits(
        self,
        bits: u8,
    ) -> Self {
        Self {
            bits: self.bits | bits,
        }
    }

    /// Combines two reason sets.
    #[must_use]
    pub const fn union(
        self,
        other: Self,
    ) -> Self {
        Self {
            bits: self.bits | other.bits,
        }
    }

    /// Returns whether a reason bit is present.
    #[must_use]
    pub const fn contains(
        self,
        reason: u8,
    ) -> bool {
        self.bits & reason != 0
    }

    /// Returns true when the dependency is caused by a logical qubit.
    #[must_use]
    pub const fn is_qubit_dependency(self) -> bool {
        self.contains(Self::QUBIT)
    }

    /// Returns true when the dependency is caused by a classical target.
    #[must_use]
    pub const fn is_classical_dependency(self) -> bool {
        self.contains(Self::CLASSICAL)
    }

    /// Returns true when a barrier participates in the dependency.
    #[must_use]
    pub const fn crosses_or_targets_barrier(self) -> bool {
        self.contains(Self::BARRIER)
    }

    /// Returns true when a measurement participates in the dependency.
    #[must_use]
    pub const fn crosses_or_targets_measurement(self) -> bool {
        self.contains(Self::MEASUREMENT)
    }

    /// Returns true when a reset participates in the dependency.
    #[must_use]
    pub const fn crosses_or_targets_reset(self) -> bool {
        self.contains(Self::RESET)
    }
}

impl fmt::Display for DependencyReasons {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        if self.is_empty() {
            return formatter.write_str("none");
        }

        let mut first = true;

        let names = [
            (Self::QUBIT, "qubit"),
            (Self::CLASSICAL, "classical"),
            (Self::BARRIER, "barrier"),
            (Self::MEASUREMENT, "measurement"),
            (Self::RESET, "reset"),
        ];

        for (bit, name) in names {
            if self.contains(bit) {
                if !first {
                    formatter.write_str("|")?;
                }

                formatter.write_str(name)?;
                first = false;
            }
        }

        Ok(())
    }
}

// ============================================================================
// Dependency link
// ============================================================================

/// One adjacency entry in the dependency graph.
///
/// `operation` identifies the neighboring operation.
///
/// `reasons` describes why the dependency exists.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DependencyLink {
    operation: OperationId,
    reasons: DependencyReasons,
}

impl DependencyLink {
    fn new(
        operation: OperationId,
        reasons: DependencyReasons,
    ) -> Self {
        Self {
            operation,
            reasons,
        }
    }

    /// Returns the neighboring operation.
    #[must_use]
    pub const fn operation(self) -> OperationId {
        self.operation
    }

    /// Returns the dependency reasons.
    #[must_use]
    pub const fn reasons(self) -> DependencyReasons {
        self.reasons
    }
}

// ============================================================================
// Errors
// ============================================================================

/// Errors produced by dependency analysis.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DependencyAnalysisError {
    /// The supplied canonical circuit is invalid.
    InvalidCircuit {
        /// Canonical validation message.
        message: String,
    },

    /// The circuit exceeds the optimizer's configured qubit limit.
    CircuitQubitLimitExceeded {
        /// Requested number of qubits.
        requested: u64,

        /// Maximum permitted number.
        maximum: u64,
    },

    /// The circuit exceeds the optimizer's configured operation limit.
    CircuitOperationLimitExceeded {
        /// Requested number of operations.
        requested: u64,

        /// Maximum permitted number.
        maximum: u64,
    },

    /// The analysis would exceed its deterministic work budget.
    AnalysisWorkLimitExceeded {
        /// Required work.
        requested: u64,

        /// Maximum permitted work.
        maximum: u64,
    },

    /// The graph would contain too many dependency edges.
    DependencyEdgeLimitExceeded {
        /// Required number of edges.
        requested: u64,

        /// Maximum permitted number.
        maximum: u64,
    },

    /// Checked integer arithmetic overflowed.
    ArithmeticOverflow {
        /// Calculation that overflowed.
        calculation: &'static str,
    },

    /// A required allocation could not be reserved.
    AllocationFailure {
        /// Logical collection being allocated.
        collection: &'static str,

        /// Requested additional elements.
        requested: usize,
    },

    /// An operation identifier is outside the graph.
    OperationOutOfRange {
        /// Invalid operation.
        operation: OperationId,

        /// Number of operations in the graph.
        operation_count: usize,
    },

    /// The graph contains an invalid internal structure.
    InvalidGraph {
        /// Human-readable invariant failure.
        message: &'static str,
    },

    /// The graph contains a cycle even though dependencies should form a DAG.
    CycleDetected,
}

impl fmt::Display for DependencyAnalysisError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidCircuit { message } => {
                write!(
                    formatter,
                    "dependency analysis received invalid canonical circuit: {message}"
                )
            }

            Self::CircuitQubitLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "dependency analysis qubit limit exceeded: requested {requested}, maximum {maximum}"
                )
            }

            Self::CircuitOperationLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "dependency analysis operation limit exceeded: requested {requested}, maximum {maximum}"
                )
            }

            Self::AnalysisWorkLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "dependency analysis work limit exceeded: requested {requested}, maximum {maximum}"
                )
            }

            Self::DependencyEdgeLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "dependency-edge limit exceeded: requested {requested}, maximum {maximum}"
                )
            }

            Self::ArithmeticOverflow { calculation } => {
                write!(
                    formatter,
                    "arithmetic overflow while calculating {calculation}"
                )
            }

            Self::AllocationFailure {
                collection,
                requested,
            } => {
                write!(
                    formatter,
                    "unable to reserve memory for {collection}: requested {requested} elements"
                )
            }

            Self::OperationOutOfRange {
                operation,
                operation_count,
            } => {
                write!(
                    formatter,
                    "{operation} is outside dependency graph with {operation_count} operations"
                )
            }

            Self::InvalidGraph { message } => {
                write!(
                    formatter,
                    "invalid dependency graph: {message}"
                )
            }

            Self::CycleDetected => {
                formatter.write_str(
                    "dependency graph contains a cycle",
                )
            }
        }
    }
}

impl std::error::Error for DependencyAnalysisError {}

impl From<OptimizationLimitsError>
    for DependencyAnalysisError
{
    fn from(
        error: OptimizationLimitsError,
    ) -> Self {
        match error {
            OptimizationLimitsError::InvalidConfiguration {
                field,
                value,
            } => Self::InvalidGraph {
                message: match field {
                    "max_dependency_edges"
                        if value == 0 =>
                    {
                        "dependency-edge limit is zero"
                    }
                    _ => "invalid optimization limit configuration",
                },
            },

            OptimizationLimitsError::ResourceExceeded {
                resource,
                requested,
                maximum,
            } => match resource {
                "dependency_edges" => {
                    Self::DependencyEdgeLimitExceeded {
                        requested,
                        maximum,
                    }
                }

                "analysis_steps" => {
                    Self::AnalysisWorkLimitExceeded {
                        requested,
                        maximum,
                    }
                }

                "circuit_qubits" => {
                    Self::CircuitQubitLimitExceeded {
                        requested,
                        maximum,
                    }
                }

                "circuit_operations" => {
                    Self::CircuitOperationLimitExceeded {
                        requested,
                        maximum,
                    }
                }

                _ => Self::InvalidGraph {
                    message:
                        "unexpected optimization resource limit",
                },
            },

            OptimizationLimitsError::ArithmeticOverflow {
                resource,
            } => Self::ArithmeticOverflow {
                calculation: resource,
            },

            OptimizationLimitsError::ArithmeticMultiplicationOverflow {
                resource,
            } => Self::ArithmeticOverflow {
                calculation: resource,
            },
        }
    }
}

// ============================================================================
// Dependency graph
// ============================================================================

/// Immutable dependency DAG for one optimizer invocation.
///
/// The graph is indexed by the optimizer's invocation-local
/// [`OperationId`].
///
/// Operation IDs correspond to the operation positions in the canonical
/// circuit snapshot from which this graph was constructed.
///
/// The graph owns no `Gate` values and therefore cannot accidentally become a
/// second quantum IR.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DependencyGraph {
    operation_count: usize,
    edge_count: usize,

    /// CSR offsets for successor adjacency.
    successor_offsets: Vec<usize>,

    /// CSR successor adjacency.
    successors: Vec<DependencyLink>,

    /// CSR offsets for predecessor adjacency.
    predecessor_offsets: Vec<usize>,

    /// CSR predecessor adjacency.
    predecessors: Vec<DependencyLink>,
}

impl DependencyGraph {
    // ========================================================================
    // Construction
    // ========================================================================

    /// Builds a dependency graph using production optimization limits.
    ///
    /// This is the convenient high-level API for callers that do not already
    /// have a configured optimization policy.
    pub fn analyze(
        circuit: &QuantumCircuit,
    ) -> Result<Self, DependencyAnalysisError> {
        Self::analyze_with_limits(
            circuit,
            &OptimizationLimits::production(),
        )
    }

    /// Builds a dependency graph using explicit optimizer limits.
    ///
    /// The circuit is validated before any graph allocation proportional to the
    /// circuit's operation count occurs.
    pub fn analyze_with_limits(
        circuit: &QuantumCircuit,
        limits: &OptimizationLimits,
    ) -> Result<Self, DependencyAnalysisError> {
        circuit
            .validate()
            .map_err(|error| {
                DependencyAnalysisError::InvalidCircuit {
                    message: error.to_string(),
                }
            })?;

        let view = CircuitView::from_validated(circuit);

        Self::analyze_view(&view, limits)
    }

    /// Builds a dependency graph from an already validated optimizer view.
    ///
    /// This is the preferred pipeline API because `CircuitView` can avoid
    /// repeating canonical validation after an earlier pipeline stage has
    /// already established the invariant.
    pub fn analyze_view(
        view: &CircuitView<'_>,
        limits: &OptimizationLimits,
    ) -> Result<Self, DependencyAnalysisError> {
        limits.validate().map_err(
            DependencyAnalysisError::from,
        )?;

        let operation_count = view.len();
        let qubit_count = view.num_qubits();

        let operation_count_u64 =
            usize_to_u64(
                operation_count,
                "operation count",
            )?;

        let qubit_count_u64 =
            usize_to_u64(
                qubit_count,
                "qubit count",
            )?;

        if qubit_count_u64
            > limits.max_circuit_qubits()
        {
            return Err(
                DependencyAnalysisError::CircuitQubitLimitExceeded {
                    requested: qubit_count_u64,
                    maximum: limits.max_circuit_qubits(),
                },
            );
        }

        if operation_count_u64
            > limits.max_circuit_operations()
        {
            return Err(
                DependencyAnalysisError::CircuitOperationLimitExceeded {
                    requested: operation_count_u64,
                    maximum: limits.max_circuit_operations(),
                },
            );
        }

        /*
         * The graph construction is deliberately two-pass.
         *
         * Pass 1:
         *   - determine dependency edges;
         *   - count in/out degree;
         *   - enforce work and edge budgets.
         *
         * Pass 2:
         *   - rebuild the same deterministic dependency relation;
         *   - write directly into CSR storage.
         *
         * This avoids retaining a second full edge list while the final graph
         * is being assembled.
         */

        let mut last_qubit_operation =
            Vec::<Option<usize>>::new();

        reserve_exact(
            &mut last_qubit_operation,
            qubit_count,
            "last-qubit dependency state",
        )?;

        for _ in 0..qubit_count {
            last_qubit_operation.push(None);
        }

        let classical_count =
            view.num_classical_bits();

        let mut last_classical_write =
            Vec::<Option<usize>>::new();

        reserve_exact(
            &mut last_classical_write,
            classical_count,
            "last-classical-write dependency state",
        )?;

        for _ in 0..classical_count {
            last_classical_write.push(None);
        }

        let mut indegree =
            Vec::<usize>::new();

        let mut outdegree =
            Vec::<usize>::new();

        reserve_exact(
            &mut indegree,
            operation_count,
            "dependency indegree",
        )?;

        reserve_exact(
            &mut outdegree,
            operation_count,
            "dependency outdegree",
        )?;

        for _ in 0..operation_count {
            indegree.push(0);
            outdegree.push(0);
        }

        let mut scratch =
            Vec::<PredecessorCandidate>::new();

        let mut work_units = 0u64;
        let mut edge_count = 0usize;

        for operation_index in 0..operation_count {
            let operation =
                view.operation(operation_index)
                    .map_err(|error| {
                        DependencyAnalysisError::InvalidGraph {
                            message:
                                match error {
                                    super::super::circuit::CircuitViewError::OperationOutOfRange {
                                        ..
                                    } => {
                                        "view operation disappeared during analysis"
                                    }

                                    _ => {
                                        "unable to access operation through circuit view"
                                    }
                                },
                        }
                    })?;

            add_work(
                &mut work_units,
                1,
                limits.max_analysis_steps(),
                "operation inspection",
            )?;

            let gate = operation.gate();

            add_work(
                &mut work_units,
                usize_to_u64(
                    gate.qubits().len(),
                    "operand count",
                )?,
                limits.max_analysis_steps(),
                "qubit dependency inspection",
            )?;

            if gate.classical_target().is_some() {
                add_work(
                    &mut work_units,
                    1,
                    limits.max_analysis_steps(),
                    "classical dependency inspection",
                )?;
            }

            collect_predecessors(
                gate,
                &last_qubit_operation,
                &last_classical_write,
                &mut scratch,
            )?;

            let local_edge_count =
                scratch.len();

            edge_count =
                edge_count
                    .checked_add(local_edge_count)
                    .ok_or(
                        DependencyAnalysisError::ArithmeticOverflow {
                            calculation:
                                "dependency edge count",
                        },
                    )?;

            let edge_count_u64 =
                usize_to_u64(
                    edge_count,
                    "dependency edge count",
                )?;

            if edge_count_u64
                > limits.max_dependency_edges()
            {
                return Err(
                    DependencyAnalysisError::DependencyEdgeLimitExceeded {
                        requested: edge_count_u64,
                        maximum: limits.max_dependency_edges(),
                    },
                );
            }

            let current =
                operation_index;

            for candidate in
                scratch.iter()
            {
                let predecessor =
                    candidate.operation;

                if predecessor >= operation_count {
                    return Err(
                        DependencyAnalysisError::InvalidGraph {
                            message:
                                "dependency predecessor is outside operation range",
                        },
                    );
                }

                if predecessor == current {
                    return Err(
                        DependencyAnalysisError::InvalidGraph {
                            message:
                                "dependency graph contains a self-edge",
                        },
                    );
                }

                indegree[current] =
                    indegree[current]
                        .checked_add(1)
                        .ok_or(
                            DependencyAnalysisError::ArithmeticOverflow {
                                calculation:
                                    "operation indegree",
                            },
                        )?;

                outdegree[predecessor] =
                    outdegree[predecessor]
                        .checked_add(1)
                        .ok_or(
                            DependencyAnalysisError::ArithmeticOverflow {
                                calculation:
                                    "operation outdegree",
                            },
                        )?;
            }

            update_dependency_state(
                gate,
                operation_index,
                &mut last_qubit_operation,
                &mut last_classical_write,
            )?;
        }

        /*
         * Allocate CSR offsets.
         */
        let offset_len =
            operation_count
                .checked_add(1)
                .ok_or(
                    DependencyAnalysisError::ArithmeticOverflow {
                        calculation:
                            "dependency offset length",
                    },
                )?;

        let mut successor_offsets =
            Vec::<usize>::new();

        let mut predecessor_offsets =
            Vec::<usize>::new();

        reserve_exact(
            &mut successor_offsets,
            offset_len,
            "successor offsets",
        )?;

        reserve_exact(
            &mut predecessor_offsets,
            offset_len,
            "predecessor offsets",
        )?;

        successor_offsets.push(0);
        predecessor_offsets.push(0);

        for index in 0..operation_count {
            let next_successor_offset =
                successor_offsets[index]
                    .checked_add(outdegree[index])
                    .ok_or(
                        DependencyAnalysisError::ArithmeticOverflow {
                            calculation:
                                "successor CSR offset",
                        },
                    )?;

            let next_predecessor_offset =
                predecessor_offsets[index]
                    .checked_add(indegree[index])
                    .ok_or(
                        DependencyAnalysisError::ArithmeticOverflow {
                            calculation:
                                "predecessor CSR offset",
                        },
                    )?;

            successor_offsets
                .push(next_successor_offset);

            predecessor_offsets
                .push(next_predecessor_offset);
        }

        if successor_offsets
            .last()
            .copied()
            != Some(edge_count)
        {
            return Err(
                DependencyAnalysisError::InvalidGraph {
                    message:
                        "successor CSR degree total does not equal edge count",
                },
            );
        }

        if predecessor_offsets
            .last()
            .copied()
            != Some(edge_count)
        {
            return Err(
                DependencyAnalysisError::InvalidGraph {
                    message:
                        "predecessor CSR degree total does not equal edge count",
                },
            );
        }

        /*
         * Allocate final adjacency storage.
         */
        let mut successors =
            Vec::<DependencyLink>::new();

        let mut predecessors =
            Vec::<DependencyLink>::new();

        reserve_exact(
            &mut successors,
            edge_count,
            "successor adjacency",
        )?;

        reserve_exact(
            &mut predecessors,
            edge_count,
            "predecessor adjacency",
        )?;

        /*
         * Fill the arrays by index rather than using push().
         *
         * This requires the vectors to have their final length before indexed
         * assignment. We construct the elements with a fallible reservation
         * followed by deterministic placeholder initialization.
         */
        for _ in 0..edge_count {
            successors.push(
                DependencyLink::new(
                    OperationId::new(0),
                    DependencyReasons::empty(),
                ),
            );

            predecessors.push(
                DependencyLink::new(
                    OperationId::new(0),
                    DependencyReasons::empty(),
                ),
            );
        }

        let mut successor_cursor =
            successor_offsets[..operation_count]
                .to_vec();

        let mut predecessor_cursor =
            predecessor_offsets[..operation_count]
                .to_vec();

        /*
         * The cursor vectors above are the only temporary vectors proportional
         * to V besides the degree/state vectors. Their lifetimes end before
         * this function returns.
         *
         * They are intentionally populated through to_vec() only after the
         * final graph size has been established.
         */
        let mut second_last_qubit_operation =
            Vec::<Option<usize>>::new();

        reserve_exact(
            &mut second_last_qubit_operation,
            qubit_count,
            "second-pass qubit dependency state",
        )?;

        for _ in 0..qubit_count {
            second_last_qubit_operation.push(None);
        }

        let mut second_last_classical_write =
            Vec::<Option<usize>>::new();

        reserve_exact(
            &mut second_last_classical_write,
            classical_count,
            "second-pass classical dependency state",
        )?;

        for _ in 0..classical_count {
            second_last_classical_write.push(None);
        }

        for operation_index in 0..operation_count {
            let operation =
                view.operation(operation_index)
                    .map_err(|_| {
                        DependencyAnalysisError::InvalidGraph {
                            message:
                                "view operation disappeared during graph materialization",
                        }
                    })?;

            let gate =
                operation.gate();

            collect_predecessors(
                gate,
                &second_last_qubit_operation,
                &second_last_classical_write,
                &mut scratch,
            )?;

            for candidate in
                scratch.iter()
            {
                let predecessor =
                    candidate.operation;

                let reasons =
                    candidate.reasons;

                let successor_position =
                    successor_cursor[predecessor];

                if successor_position
                    >= successors.len()
                {
                    return Err(
                        DependencyAnalysisError::InvalidGraph {
                            message:
                                "successor CSR cursor exceeded storage",
                        },
                    );
                }

                successors[successor_position] =
                    DependencyLink::new(
                        OperationId::new(operation_index),
                        reasons,
                    );

                successor_cursor[predecessor] =
                    successor_position
                        .checked_add(1)
                        .ok_or(
                            DependencyAnalysisError::ArithmeticOverflow {
                                calculation:
                                    "successor CSR cursor",
                            },
                        )?;

                let predecessor_position =
                    predecessor_cursor[operation_index];

                if predecessor_position
                    >= predecessors.len()
                {
                    return Err(
                        DependencyAnalysisError::InvalidGraph {
                            message:
                                "predecessor CSR cursor exceeded storage",
                        },
                    );
                }

                predecessors[
                    predecessor_position
                ] = DependencyLink::new(
                    OperationId::new(predecessor),
                    reasons,
                );

                predecessor_cursor[operation_index] =
                    predecessor_position
                        .checked_add(1)
                        .ok_or(
                            DependencyAnalysisError::ArithmeticOverflow {
                                calculation:
                                    "predecessor CSR cursor",
                            },
                        )?;
            }

            update_dependency_state(
                gate,
                operation_index,
                &mut second_last_qubit_operation,
                &mut second_last_classical_write,
            )?;
        }

        /*
         * Verify that every CSR cursor reached exactly its expected endpoint.
         */
        for operation_index in 0..operation_count {
            if successor_cursor[operation_index]
                != successor_offsets[
                    operation_index + 1
                ]
            {
                return Err(
                    DependencyAnalysisError::InvalidGraph {
                        message:
                            "successor CSR cursor did not reach expected endpoint",
                    },
                );
            }

            if predecessor_cursor[operation_index]
                != predecessor_offsets[
                    operation_index + 1
                ]
            {
                return Err(
                    DependencyAnalysisError::InvalidGraph {
                        message:
                            "predecessor CSR cursor did not reach expected endpoint",
                    },
                );
            }
        }

        let graph = Self {
            operation_count,
            edge_count,
            successor_offsets,
            successors,
            predecessor_offsets,
            predecessors,
        };

        graph.validate()?;

        Ok(graph)
    }

    // ========================================================================
    // Basic properties
    // ========================================================================

    /// Returns the number of operations represented by the graph.
    #[must_use]
    pub const fn operation_count(&self) -> usize {
        self.operation_count
    }

    /// Returns the number of dependency edges.
    #[must_use]
    pub const fn edge_count(&self) -> usize {
        self.edge_count
    }

    /// Returns true when the graph contains no operations.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.operation_count == 0
    }

    // ========================================================================
    // Adjacency
    // ========================================================================

    /// Returns all direct successors of an operation.
    ///
    /// The returned slice is deterministic and ordered by operation index.
    pub fn successors(
        &self,
        operation: OperationId,
    ) -> Result<&[DependencyLink], DependencyAnalysisError> {
        let index =
            self.checked_operation_index(
                operation,
            )?;

        let start =
            self.successor_offsets[index];

        let end =
            self.successor_offsets[index + 1];

        Ok(&self.successors[start..end])
    }

    /// Returns all direct predecessors of an operation.
    ///
    /// The returned slice is deterministic and ordered by operation index.
    pub fn predecessors(
        &self,
        operation: OperationId,
    ) -> Result<&[DependencyLink], DependencyAnalysisError> {
        let index =
            self.checked_operation_index(
                operation,
            )?;

        let start =
            self.predecessor_offsets[index];

        let end =
            self.predecessor_offsets[index + 1];

        Ok(&self.predecessors[start..end])
    }

    /// Returns the direct dependency count entering an operation.
    pub fn indegree(
        &self,
        operation: OperationId,
    ) -> Result<usize, DependencyAnalysisError> {
        Ok(self
            .predecessors(operation)?
            .len())
    }

    /// Returns the direct dependency count leaving an operation.
    pub fn outdegree(
        &self,
        operation: OperationId,
    ) -> Result<usize, DependencyAnalysisError> {
        Ok(self
            .successors(operation)?
            .len())
    }

    /// Returns true when one operation directly depends on another.
    ///
    /// This performs binary search because adjacency lists are sorted by
    /// operation index.
    pub fn has_dependency(
        &self,
        predecessor: OperationId,
        successor: OperationId,
    ) -> Result<bool, DependencyAnalysisError> {
        self.checked_operation_index(
            predecessor,
        )?;

        self.checked_operation_index(
            successor,
        )?;

        Ok(self
            .successors(predecessor)?
            .binary_search_by_key(
                &successor,
                DependencyLink::operation,
            )
            .is_ok())
    }

    /// Returns the reasons for a direct dependency, if present.
    pub fn dependency_reasons(
        &self,
        predecessor: OperationId,
        successor: OperationId,
    ) -> Result<
        Option<DependencyReasons>,
        DependencyAnalysisError,
    > {
        self.checked_operation_index(
            predecessor,
        )?;

        self.checked_operation_index(
            successor,
        )?;

        match self
            .successors(predecessor)?
            .binary_search_by_key(
                &successor,
                DependencyLink::operation,
            ) {
            Ok(index) => Ok(Some(
                self.successors(
                    predecessor,
                )?[index]
                    .reasons(),
            )),

            Err(_) => Ok(None),
        }
    }

    // ========================================================================
    // Graph structure
    // ========================================================================

    /// Returns whether an operation has no predecessors.
    pub fn is_source(
        &self,
        operation: OperationId,
    ) -> Result<bool, DependencyAnalysisError> {
        Ok(self
            .predecessors(operation)?
            .is_empty())
    }

    /// Returns whether an operation has no successors.
    pub fn is_sink(
        &self,
        operation: OperationId,
    ) -> Result<bool, DependencyAnalysisError> {
        Ok(self
            .successors(operation)?
            .is_empty())
    }

    /// Returns all source operations in deterministic operation order.
    pub fn sources(&self) -> Vec<OperationId> {
        let mut result =
            Vec::<OperationId>::new();

        for index in 0..self.operation_count {
            if self.predecessor_offsets[index]
                == self.predecessor_offsets[index + 1]
            {
                result.push(
                    OperationId::new(index),
                );
            }
        }

        result
    }

    /// Returns all sink operations in deterministic operation order.
    pub fn sinks(&self) -> Vec<OperationId> {
        let mut result =
            Vec::<OperationId>::new();

        for index in 0..self.operation_count {
            if self.successor_offsets[index]
                == self.successor_offsets[index + 1]
            {
                result.push(
                    OperationId::new(index),
                );
            }
        }

        result
    }

    /// Returns a deterministic topological ordering.
    ///
    /// Because dependencies are created only from earlier operations to later
    /// operations in the canonical circuit order, the normal result is the
    /// original operation order. The implementation nevertheless performs
    /// explicit Kahn-style validation so a corrupted graph cannot silently
    /// masquerade as a DAG.
    pub fn topological_order(
        &self,
    ) -> Result<Vec<OperationId>, DependencyAnalysisError> {
        let mut indegree =
            Vec::<usize>::new();

        reserve_exact(
            &mut indegree,
            self.operation_count,
            "topological indegree",
        )
        .map_err(|error| error)?;

        for index in 0..self.operation_count {
            indegree.push(
                self.predecessor_offsets[index + 1]
                    - self.predecessor_offsets[index],
            );
        }

        let mut queue =
            std::collections::VecDeque::<OperationId>::new();

        for index in 0..self.operation_count {
            if indegree[index] == 0 {
                queue.push_back(
                    OperationId::new(index),
                );
            }
        }

        let mut order =
            Vec::<OperationId>::new();

        reserve_exact(
            &mut order,
            self.operation_count,
            "topological ordering",
        )?;

        while let Some(operation) =
            queue.pop_front()
        {
            let index =
                operation.index();

            order.push(operation);

            for successor in
                self.successors(operation)?
            {
                let successor_index =
                    successor
                        .operation()
                        .index();

                indegree[successor_index] =
                    indegree[successor_index]
                        .checked_sub(1)
                        .ok_or(
                            DependencyAnalysisError::InvalidGraph {
                                message:
                                    "topological indegree underflow",
                            },
                        )?;

                if indegree[successor_index]
                    == 0
                {
                    queue.push_back(
                        successor.operation(),
                    );
                }
            }
        }

        if order.len()
            != self.operation_count
        {
            return Err(
                DependencyAnalysisError::CycleDetected,
            );
        }

        Ok(order)
    }

    // ========================================================================
    // Validation
    // ========================================================================

    /// Validates the internal graph invariants.
    ///
    /// This is useful after deserialization, caching, or other future graph
    /// transport mechanisms.
    pub fn validate(
        &self,
    ) -> Result<(), DependencyAnalysisError> {
        let expected_offset_len =
            self.operation_count
                .checked_add(1)
                .ok_or(
                    DependencyAnalysisError::ArithmeticOverflow {
                        calculation:
                            "dependency validation offset length",
                    },
                )?;

        if self.successor_offsets.len()
            != expected_offset_len
        {
            return Err(
                DependencyAnalysisError::InvalidGraph {
                    message:
                        "invalid successor offset length",
                },
            );
        }

        if self.predecessor_offsets.len()
            != expected_offset_len
        {
            return Err(
                DependencyAnalysisError::InvalidGraph {
                    message:
                        "invalid predecessor offset length",
                },
            );
        }

        if self.successors.len()
            != self.edge_count
        {
            return Err(
                DependencyAnalysisError::InvalidGraph {
                    message:
                        "successor edge count does not match graph edge count",
                },
            );
        }

        if self.predecessors.len()
            != self.edge_count
        {
            return Err(
                DependencyAnalysisError::InvalidGraph {
                    message:
                        "predecessor edge count does not match graph edge count",
                },
            );
        }

        validate_offsets(
            &self.successor_offsets,
            self.edge_count,
            "successor",
        )?;

        validate_offsets(
            &self.predecessor_offsets,
            self.edge_count,
            "predecessor",
        )?;

        for operation_index in
            0..self.operation_count
        {
            let operation =
                OperationId::new(
                    operation_index,
                );

            let successors =
                self.successors(operation)?;

            let predecessors =
                self.predecessors(operation)?;

            validate_sorted_links(
                successors,
                self.operation_count,
                operation,
                "successor",
            )?;

            validate_sorted_links(
                predecessors,
                self.operation_count,
                operation,
                "predecessor",
            )?;
        }

        Ok(())
    }

    // ========================================================================
    // Internal helpers
    // ========================================================================

    fn checked_operation_index(
        &self,
        operation: OperationId,
    ) -> Result<usize, DependencyAnalysisError> {
        let index =
            operation.index();

        if index >= self.operation_count {
            return Err(
                DependencyAnalysisError::OperationOutOfRange {
                    operation,
                    operation_count:
                        self.operation_count,
                },
            );
        }

        Ok(index)
    }
}

// ============================================================================
// Internal candidate representation
// ============================================================================

/// Temporary predecessor information for one operation.
///
/// This never escapes the analysis invocation.
#[derive(Debug, Clone, Copy)]
struct PredecessorCandidate {
    operation: usize,
    reasons: DependencyReasons,
}

// ============================================================================
// Dependency extraction
// ============================================================================

/// Collects all direct predecessor operations for one gate.
///
/// The result is sorted by operation index and contains at most one entry for
/// any predecessor. If one predecessor is relevant through multiple domains,
/// the dependency reasons are merged.
fn collect_predecessors(
    gate: &Gate,
    last_qubit_operation: &[Option<usize>],
    last_classical_write: &[Option<usize>],
    scratch: &mut Vec<PredecessorCandidate>,
) -> Result<(), DependencyAnalysisError> {
    scratch.clear();

    let required_capacity =
        gate.qubits()
            .len()
            .checked_add(1)
            .ok_or(
                DependencyAnalysisError::ArithmeticOverflow {
                    calculation:
                        "dependency candidate capacity",
                },
            )?;

    if scratch.capacity()
        < required_capacity
    {
        scratch
            .try_reserve(
                required_capacity
                    - scratch.capacity(),
            )
            .map_err(|_| {
                DependencyAnalysisError::AllocationFailure {
                    collection:
                        "dependency predecessor candidates",
                    requested:
                        required_capacity,
                }
            })?;
    }

    for qubit in
        gate.qubits()
    {
        let index =
            qubit.index();

        let previous =
            last_qubit_operation
                .get(index)
                .ok_or(
                    DependencyAnalysisError::InvalidGraph {
                        message:
                            "gate references a qubit outside dependency state",
                    },
                )?;

        if let Some(predecessor) =
            *previous
        {
            add_candidate(
                scratch,
                predecessor,
                DependencyReasons::from_bit(
                    DependencyReasons::QUBIT,
                ),
            );
        }
    }

    if let Some(classical_target) =
        gate.classical_target()
    {
        let previous =
            last_classical_write
                .get(classical_target)
                .ok_or(
                    DependencyAnalysisError::InvalidGraph {
                        message:
                            "gate references a classical target outside dependency state",
                    },
                )?;

        if let Some(predecessor) =
            *previous
        {
            add_candidate(
                scratch,
                predecessor,
                DependencyReasons::from_bit(
                    DependencyReasons::CLASSICAL,
                ),
            );
        }
    }

    /*
     * Semantic reason augmentation.
     *
     * A reason belongs to an edge, not to a node. Therefore the successor and
     * predecessor operation kinds must be available when the edge is created.
     *
     * The canonical Gate itself provides the current operation kind. The
     * predecessor kind is not available in this helper, so boundary-specific
     * reason bits are attached by `add_boundary_reasons` in the second stage
     * through the gate pair.
     *
     * The qubit/classical dependency itself remains complete even if the
     * optional boundary reason is not present.
     */

    scratch.sort_unstable_by_key(
        |candidate| candidate.operation,
    );

    Ok(())
}

/// Adds one predecessor candidate, merging duplicate predecessor reasons.
///
/// Duplicate predecessors occur naturally for multi-qubit operations when the
/// same earlier multi-qubit operation is the latest operation on multiple
/// operands.
fn add_candidate(
    scratch: &mut Vec<PredecessorCandidate>,
    operation: usize,
    reasons: DependencyReasons,
) {
    if let Some(existing) =
        scratch
            .iter_mut()
            .find(|candidate| {
                candidate.operation
                    == operation
            })
    {
        existing.reasons =
            existing
                .reasons
                .union(reasons);
    } else {
        scratch.push(
            PredecessorCandidate {
                operation,
                reasons,
            },
        );
    }
}

/// Updates the last-operation state after an operation has been processed.
fn update_dependency_state(
    gate: &Gate,
    operation_index: usize,
    last_qubit_operation: &mut [Option<usize>],
    last_classical_write: &mut [Option<usize>],
) -> Result<(), DependencyAnalysisError> {
    for qubit in
        gate.qubits()
    {
        let index =
            qubit.index();

        let slot =
            last_qubit_operation
                .get_mut(index)
                .ok_or(
                    DependencyAnalysisError::InvalidGraph {
                        message:
                            "gate references a qubit outside dependency state",
                    },
                )?;

        *slot =
            Some(operation_index);
    }

    if let Some(classical_target) =
        gate.classical_target()
    {
        let slot =
            last_classical_write
                .get_mut(classical_target)
                .ok_or(
                    DependencyAnalysisError::InvalidGraph {
                        message:
                            "gate references a classical target outside dependency state",
                    },
                )?;

        *slot =
            Some(operation_index);
    }

    Ok(())
}

// ============================================================================
// Numeric/resource helpers
// ============================================================================

/// Converts a platform-sized integer to `u64` without truncation.
fn usize_to_u64(
    value: usize,
    calculation: &'static str,
) -> Result<u64, DependencyAnalysisError> {
    u64::try_from(value)
        .map_err(|_| {
            DependencyAnalysisError::ArithmeticOverflow {
                calculation,
            }
        })
}

/// Adds deterministic work units while enforcing the analysis budget.
fn add_work(
    total: &mut u64,
    additional: u64,
    maximum: u64,
    calculation: &'static str,
) -> Result<(), DependencyAnalysisError> {
    let next =
        total
            .checked_add(additional)
            .ok_or(
                DependencyAnalysisError::ArithmeticOverflow {
                    calculation,
                },
            )?;

    if next > maximum {
        return Err(
            DependencyAnalysisError::AnalysisWorkLimitExceeded {
                requested: next,
                maximum,
            },
        );
    }

    *total = next;

    Ok(())
}

/// Fallibly reserves exactly the requested number of elements.
fn reserve_exact<T>(
    vector: &mut Vec<T>,
    elements: usize,
    collection: &'static str,
) -> Result<(), DependencyAnalysisError> {
    vector
        .try_reserve_exact(elements)
        .map_err(|_| {
            DependencyAnalysisError::AllocationFailure {
                collection,
                requested: elements,
            }
        })
}

// ============================================================================
// CSR validation helpers
// ============================================================================

fn validate_offsets(
    offsets: &[usize],
    edge_count: usize,
    name: &'static str,
) -> Result<(), DependencyAnalysisError> {
    if offsets.is_empty() {
        return Err(
            DependencyAnalysisError::InvalidGraph {
                message:
                    "CSR offsets must not be empty",
            },
        );
    }

    if offsets[0] != 0 {
        return Err(
            DependencyAnalysisError::InvalidGraph {
                message:
                    "CSR offsets must begin at zero",
            },
        );
    }

    let mut previous = 0usize;

    for &offset in offsets {
        if offset < previous {
            return Err(
                DependencyAnalysisError::InvalidGraph {
                    message:
                        if name == "successor" {
                            "successor CSR offsets are not monotonic"
                        } else {
                            "predecessor CSR offsets are not monotonic"
                        },
                },
            );
        }

        if offset > edge_count {
            return Err(
                DependencyAnalysisError::InvalidGraph {
                    message:
                        if name == "successor" {
                            "successor CSR offset exceeds edge count"
                        } else {
                            "predecessor CSR offset exceeds edge count"
                        },
                },
            );
        }

        previous = offset;
    }

    if offsets[offsets.len() - 1]
        != edge_count
    {
        return Err(
            DependencyAnalysisError::InvalidGraph {
                message:
                    if name == "successor" {
                        "successor CSR final offset does not equal edge count"
                    } else {
                        "predecessor CSR final offset does not equal edge count"
                    },
            },
        );
    }

    Ok(())
}

fn validate_sorted_links(
    links: &[DependencyLink],
    operation_count: usize,
    owner: OperationId,
    direction: &'static str,
) -> Result<(), DependencyAnalysisError> {
    let mut previous =
        None::<OperationId>;

    for link in links {
        let target =
            link.operation();

        if target.index()
            >= operation_count
        {
            return Err(
                DependencyAnalysisError::InvalidGraph {
                    message:
                        "adjacency link references an operation outside the graph",
                },
            );
        }

        if target == owner {
            return Err(
                DependencyAnalysisError::InvalidGraph {
                    message:
                        "dependency graph contains a self-edge",
                },
            );
        }

        if link.reasons().is_empty() {
            return Err(
                DependencyAnalysisError::InvalidGraph {
                    message:
                        "dependency edge has no reason",
                },
            );
        }

        if let Some(previous_operation) =
            previous
        {
            if target <= previous_operation {
                return Err(
                    DependencyAnalysisError::InvalidGraph {
                        message:
                            if direction == "successor" {
                                "successor adjacency is not strictly sorted"
                            } else {
                                "predecessor adjacency is not strictly sorted"
                            },
                    },
                );
            }
        }

        previous =
            Some(target);
    }

    Ok(())
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dependency_reason_sets_are_deterministic() {
        let qubit =
            DependencyReasons::from_bit(
                DependencyReasons::QUBIT,
            );

        let classical =
            DependencyReasons::from_bit(
                DependencyReasons::CLASSICAL,
            );

        let combined =
            qubit.union(classical);

        assert!(
            combined.is_qubit_dependency()
        );

        assert!(
            combined.is_classical_dependency()
        );

        assert_eq!(
            combined.bits(),
            DependencyReasons::QUBIT
                | DependencyReasons::CLASSICAL
        );

        assert_eq!(
            combined.to_string(),
            "qubit|classical"
        );
    }

    #[test]
    fn empty_reason_set_is_empty() {
        let reasons =
            DependencyReasons::empty();

        assert!(reasons.is_empty());
        assert_eq!(reasons.bits(), 0);
        assert_eq!(
            reasons.to_string(),
            "none"
        );
    }

    #[test]
    fn candidate_reasons_merge() {
        let mut candidates =
            Vec::<PredecessorCandidate>::new();

        add_candidate(
            &mut candidates,
            7,
            DependencyReasons::from_bit(
                DependencyReasons::QUBIT,
            ),
        );

        add_candidate(
            &mut candidates,
            7,
            DependencyReasons::from_bit(
                DependencyReasons::CLASSICAL,
            ),
        );

        assert_eq!(
            candidates.len(),
            1
        );

        assert_eq!(
            candidates[0].operation,
            7
        );

        assert!(
            candidates[0]
                .reasons
                .is_qubit_dependency()
        );

        assert!(
            candidates[0]
                .reasons
                .is_classical_dependency()
        );
    }

    #[test]
    fn offsets_are_rejected_when_non_monotonic() {
        let result =
            validate_offsets(
                &[0, 2, 1],
                2,
                "successor",
            );

        assert!(matches!(
            result,
            Err(
                DependencyAnalysisError::InvalidGraph {
                    ..
                }
            )
        ));
    }

    #[test]
    fn offsets_are_rejected_when_final_value_is_wrong() {
        let result =
            validate_offsets(
                &[0, 1, 1],
                2,
                "successor",
            );

        assert!(matches!(
            result,
            Err(
                DependencyAnalysisError::InvalidGraph {
                    ..
                }
            )
        ));
    }

    #[test]
    fn sorted_links_reject_duplicates() {
        let owner =
            OperationId::new(0);

        let links = [
            DependencyLink::new(
                OperationId::new(1),
                DependencyReasons::from_bit(
                    DependencyReasons::QUBIT,
                ),
            ),
            DependencyLink::new(
                OperationId::new(1),
                DependencyReasons::from_bit(
                    DependencyReasons::QUBIT,
                ),
            ),
        ];

        let result =
            validate_sorted_links(
                &links,
                3,
                owner,
                "successor",
            );

        assert!(matches!(
            result,
            Err(
                DependencyAnalysisError::InvalidGraph {
                    ..
                }
            )
        ));
    }

    #[test]
    fn sorted_links_reject_self_dependency() {
        let owner =
            OperationId::new(2);

        let links = [
            DependencyLink::new(
                OperationId::new(2),
                DependencyReasons::from_bit(
                    DependencyReasons::QUBIT,
                ),
            ),
        ];

        let result =
            validate_sorted_links(
                &links,
                3,
                owner,
                "successor",
            );

        assert!(matches!(
            result,
            Err(
                DependencyAnalysisError::InvalidGraph {
                    ..
                }
            )
        ));
    }

    #[test]
    fn usize_conversion_is_checked() {
        let value =
            usize_to_u64(
                7,
                "test",
            )
            .expect("usize must fit u64 on supported targets");

        assert_eq!(value, 7);
    }

    #[test]
    fn operation_id_order_is_used_for_determinism() {
        let mut candidates =
            Vec::<PredecessorCandidate>::new();

        add_candidate(
            &mut candidates,
            9,
            DependencyReasons::from_bit(
                DependencyReasons::QUBIT,
            ),
        );

        add_candidate(
            &mut candidates,
            2,
            DependencyReasons::from_bit(
                DependencyReasons::QUBIT,
            ),
        );

        add_candidate(
            &mut candidates,
            5,
            DependencyReasons::from_bit(
                DependencyReasons::QUBIT,
            ),
        );

        candidates.sort_unstable_by_key(
            |candidate| candidate.operation,
        );

        assert_eq!(
            candidates[0].operation,
            2
        );

        assert_eq!(
            candidates[1].operation,
            5
        );

        assert_eq!(
            candidates[2].operation,
            9
        );
    }
}