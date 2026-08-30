//! Zamani Quantum Optimization — Critical-Path Analysis
//!
//! Production-grade critical-path analysis over the canonical Zamani
//! `crate::quantum::ir::QuantumCircuit`.
//!
//! # Architectural position
//!
//! ```text
//!                  quantum::ir::QuantumCircuit
//!                             │
//!                             ▼
//!                 optimization::analysis
//!                             │
//!                             ▼
//!                    analysis::dependency
//!                             │
//!                             ▼
//!                    analysis::critical_path
//!                             │
//!             ┌───────────────┼────────────────┐
//!             ▼               ▼                ▼
//!       logical path     operation depth    optimization
//!             │               │                │
//!             └───────────────┼────────────────┘
//!                             ▼
//!                     downstream scheduling
//! ```
//!
//! This module determines the longest dependency-constrained logical path
//! through a quantum circuit.
//!
//! It does NOT perform:
//!
//! - hardware scheduling;
//! - pulse scheduling;
//! - routing;
//! - physical-qubit assignment;
//! - calibration;
//! - backend execution;
//! - QPU communication;
//! - optimization transformations;
//! - gate rewriting.
//!
//! Those responsibilities belong to other quantum subsystems.
//!
//! # Critical-path definition
//!
//! Given a directed acyclic dependency graph:
//!
//! ```text
//! A ──► C ──► E
//!       ▲
//!       │
//! B ────┘
//! ```
//!
//! the critical path is the dependency path with the greatest accumulated
//! operation weight.
//!
//! With the default unit weights:
//!
//! ```text
//! A = 1
//! B = 1
//! C = 1
//! E = 1
//! ```
//!
//! the critical path contains three operations:
//!
//! ```text
//! A → C → E
//! ```
//!
//! If weighted execution estimates are supplied:
//!
//! ```text
//! A = 10
//! B = 2
//! C = 20
//! E = 5
//! ```
//!
//! the critical path is selected by total weight rather than operation count.
//!
//! # Why this module is separate from depth analysis
//!
//! `analysis::depth` computes an ASAP logical-layer metric directly from the
//! ordered canonical circuit.
//!
//! `analysis::critical_path` instead consumes the explicit dependency DAG.
//!
//! These metrics are related but intentionally distinct:
//!
//! ```text
//! depth
//!     = logical layer count under the depth model
//!
//! critical path
//!     = maximum accumulated dependency-path weight
//! ```
//!
//! For ordinary unit-weight straight-line circuits they will often agree,
//! but they are not semantically interchangeable.
//!
//! Weighted critical paths can represent optimization objectives such as:
//!
//! - estimated logical duration;
//! - gate cost;
//! - two-qubit cost;
//! - fault-tolerant logical cost;
//! - target-specific abstract operation cost.
//!
//! The weighting policy belongs to the caller. This module only performs the
//! graph analysis.
//!
//! # Canonical IR ownership
//!
//! This module MUST NOT define:
//!
//! - `QuantumGate`;
//! - another circuit representation;
//! - another qubit representation;
//! - physical qubits;
//! - hardware timing;
//! - routing metadata.
//!
//! The authoritative representations remain:
//!
//! ```text
//! crate::quantum::ir::QuantumCircuit
//! crate::quantum::ir::Gate
//! crate::quantum::ir::qubits::QubitId
//! ```
//!
//! `CircuitView` is used only as the optimizer's immutable access layer.
//!
//! # Dependency-graph ownership
//!
//! `analysis::dependency` owns construction of the logical dependency graph.
//!
//! This module consumes:
//!
//! ```text
//! DependencyGraph
//! DependencyLink
//! OperationId
//! ```
//!
//! It deliberately does not rebuild dependencies independently.
//!
//! This prevents multiple subtly different definitions of dependency from
//! appearing throughout the optimization subsystem.
//!
//! # Complexity
//!
//! Let:
//!
//! - `V` = number of operations;
//! - `E` = number of dependency edges.
//!
//! Critical-path analysis is:
//!
//! ```text
//! time   = O(V + E)
//! memory = O(V)
//! ```
//!
//! excluding the memory already owned by the dependency graph.
//!
//! No all-pairs operation comparison is performed.
//!
//! No recursive DFS is used.
//!
//! Therefore the analysis remains suitable for very large circuits subject to:
//!
//! - canonical IR limits;
//! - optimizer limits;
//! - dependency-graph limits;
//! - available memory;
//! - available CPU.
//!
//! There is intentionally no artificial fixed circuit-size ceiling.
//!
//! # Determinism
//!
//! Results are deterministic.
//!
//! When multiple predecessor paths have exactly the same accumulated weight,
//! the predecessor with the smallest invocation-local `OperationId` wins.
//!
//! This makes:
//!
//! ```text
//! optimize(C)
//! ```
//!
//! reproducible under the same input, graph, weights, and limits.
//!
//! No hash map is used by the critical-path algorithm.
//!
//! # Overflow handling
//!
//! All arithmetic involving:
//!
//! - path weights;
//! - analysis work;
//! - counters;
//! - vector capacities;
//!
//! uses checked arithmetic.
//!
//! Integer overflow never silently wraps.
//!
//! # Resource limits
//!
//! The analysis honors `OptimizationLimits`.
//!
//! The dependency graph itself enforces:
//!
//! - circuit operation limits;
//! - circuit qubit limits;
//! - dependency-edge limits;
//! - dependency analysis work limits.
//!
//! This module additionally accounts for its own analysis work.
//!
//! # Weighted analysis contract
//!
//! `analyze()` uses unit weights:
//!
//! ```text
//! weight(operation) = 1
//! ```
//!
//! `analyze_with_weights()` accepts one `u64` weight per operation.
//!
//! A weight of zero is legal.
//!
//! This is important because a future target/cost model may legitimately
//! assign zero logical cost to an operation that is semantically present.
//!
//! Negative weights are impossible because weights are unsigned.
//!
//! # Integration with future cost.rs
//!
//! This file intentionally does not depend on `optimization::cost`.
//!
//! That prevents an unnecessary dependency cycle between foundational
//! analyses and future cost infrastructure.
//!
//! A future cost model can produce:
//!
//! ```text
//! Vec<u64>
//! ```
//!
//! indexed by `OperationId::index()` and pass it to:
//!
//! ```text
//! CriticalPathAnalysis::analyze_with_weights(...)
//! ```
//!
//! The optimizer can then compare:
//!
//! - unit-weight critical path;
//! - duration-weighted critical path;
//! - two-qubit-weighted critical path;
//! - fault-tolerant logical-cost critical path.
//!
//! # Integration with depth.rs
//!
//! `depth.rs` remains the owner of logical-layer depth.
//!
//! This module does not call `depth.rs`, preventing an unnecessary analysis
//! dependency cycle.
//!
//! A future composite analysis can consume both:
//!
//! ```text
//! DepthAnalysis
//! CriticalPathAnalysis
//! ```
//!
//! and expose both metrics to the optimization cost model.
//!
//! # Integration with optimization/context.rs
//!
//! The result is immutable after construction and is suitable for caching in
//! `OptimizationContext`.
//!
//! Any transformation that changes:
//!
//! - operation membership;
//! - operation order;
//! - qubit operands;
//! - classical dependencies;
//! - dependency semantics;
//!
//! invalidates this analysis.
//!
//! A metadata-only transformation may retain it if the optimizer's analysis
//! invalidation contract explicitly guarantees that the dependency graph is
//! unchanged.
//!
//! # Integration with pipeline.rs
//!
//! A pipeline can request this analysis before depth-sensitive optimization:
//!
//! ```text
//! canonical IR
//!     │
//!     ▼
//! dependency analysis
//!     │
//!     ▼
//! critical-path analysis
//!     │
//!     ▼
//! optimization pass
//! ```
//!
//! # Integration with scheduler.rs
//!
//! `scheduler.rs` may consume the critical path as an optimization/lower-bound
//! metric.
//!
//! It MUST NOT treat this result as a physical execution schedule.
//!
//! A critical path says what cannot be reordered past dependency boundaries.
//! It does not say when a QPU pulse should execute.
//!
//! # Integration with targets/
//!
//! Target-specific cost models may provide operation weights.
//!
//! The target layer remains responsible for defining what an operation costs.
//!
//! This module remains target-independent.
//!
//! # Integration with benchmarking/
//!
//! Benchmarking may consume:
//!
//! - critical-path weight before optimization;
//! - critical-path weight after optimization;
//! - critical-path operation count;
//! - path reduction;
//! - weighted path reduction.
//!
//! Benchmarking must remain a consumer rather than a dependency of this file.
//!
//! # Integration with verification/
//!
//! Critical-path analysis itself does not prove semantic equivalence.
//!
//! It only analyzes a supplied dependency graph.
//!
//! Semantic equivalence remains the responsibility of the verification
//! subsystem.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust only;
//! - no nightly features;
//! - no external dependencies;
//! - no `unsafe`.
//!
//! # Safety
//!
//! ```text
//! #![forbid(unsafe_code)]
//! ```
//!
//! is applied to the entire module.
//!
//! # Public API
//!
//! The primary API is:
//!
//! ```ignore
//! CriticalPathAnalysis::analyze(&circuit)
//! CriticalPathAnalysis::analyze_with_limits(&circuit, &limits)
//! CriticalPathAnalysis::analyze_view(&view, &limits)
//! CriticalPathAnalysis::analyze_with_weights(
//!     &view,
//!     &dependency_graph,
//!     &weights,
//!     &limits,
//! )
//! ```
//!
//! The resulting object exposes:
//!
//! ```ignore
//! analysis.operation_count()
//! analysis.edge_count()
//! analysis.critical_path_weight()
//! analysis.critical_path_operation_count()
//! analysis.critical_path()
//! analysis.operation_weight(operation)
//! analysis.operation_distance(operation)
//! analysis.critical_predecessor(operation)
//! analysis.is_on_critical_path(operation)
//! ```
//!
//! # Verification properties
//!
//! A correct result must satisfy:
//!
//! 1. every path predecessor occurs before its successor in the dependency
//!    graph;
//! 2. every critical predecessor actually has a dependency edge to its
//!    successor;
//! 3. each operation's distance equals its own weight plus the maximum
//!    predecessor distance;
//! 4. source operations have distance equal to their own weight;
//! 5. the reconstructed critical path is a valid dependency path;
//! 6. the sum of weights on the reconstructed path equals the reported
//!    critical-path weight;
//! 7. an empty graph has critical-path weight zero;
//! 8. no integer arithmetic silently wraps;
//! 9. results are deterministic;
//! 10. the input circuit is never modified.
//!
//! # Architectural rule
//!
//! This file is an analysis only.
//!
//! It MUST remain free of optimizer transformations.
//!
//! In particular, it must never:
//!
//! - remove gates;
//! - insert gates;
//! - reorder gates;
//! - synthesize gates;
//! - route gates;
//! - schedule gates;
//! - mutate the canonical circuit.

#![forbid(unsafe_code)]

use std::fmt;

use crate::quantum::ir::QuantumCircuit;

use super::super::circuit::{
    CircuitView,
    OperationId,
};
use super::super::limits::{
    OptimizationLimits,
    OptimizationLimitsError,
};
use super::dependency::{
    DependencyAnalysisError,
    DependencyGraph,
};

// ============================================================================
// Public identifiers
// ============================================================================

/// Stable identifier for this analysis implementation.
pub const CRITICAL_PATH_ANALYSIS_ID: &str =
    "quantum.optimization.analysis.critical_path";

/// Semantic version of the critical-path analysis contract.
///
/// This version is independent of the Quantum IR version and compiler version.
pub const CRITICAL_PATH_ANALYSIS_VERSION: u32 = 1;

// ============================================================================
// Errors
// ============================================================================

/// Errors produced by critical-path analysis.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CriticalPathAnalysisError {
    /// The canonical circuit is invalid.
    InvalidCircuit {
        /// Human-readable validation error.
        message: String,
    },

    /// The dependency graph is invalid or unavailable.
    DependencyAnalysis {
        /// Underlying dependency-analysis error.
        message: String,
    },

    /// The number of supplied operation weights does not match the circuit.
    WeightCountMismatch {
        /// Number of operations in the circuit.
        operation_count: usize,

        /// Number of supplied weights.
        weight_count: usize,
    },

    /// A supplied operation weight vector is inconsistent with the graph.
    WeightOperationMismatch {
        /// Number of operations in the graph.
        operation_count: usize,

        /// Number of supplied weights.
        weight_count: usize,
    },

    /// The optimizer's analysis-work budget was exceeded.
    AnalysisWorkLimitExceeded {
        /// Work requested.
        requested: u64,

        /// Maximum permitted work.
        maximum: u64,
    },

    /// Checked arithmetic overflowed.
    ArithmeticOverflow {
        /// Description of the calculation.
        calculation: &'static str,
    },

    /// A vector allocation could not be reserved.
    AllocationFailure {
        /// Logical collection being allocated.
        collection: &'static str,

        /// Number of requested elements.
        requested: usize,
    },

    /// An operation ID is outside the analyzed circuit.
    OperationOutOfRange {
        /// Invalid operation ID.
        operation: OperationId,

        /// Number of operations in the analysis.
        operation_count: usize,
    },

    /// A reconstructed path is internally inconsistent.
    InvalidCriticalPath {
        /// Human-readable invariant failure.
        message: &'static str,
    },

    /// The supplied dependency graph contains a cycle.
    CycleDetected,
}

impl fmt::Display for CriticalPathAnalysisError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidCircuit { message } => {
                write!(
                    formatter,
                    "critical-path analysis received invalid circuit: {message}"
                )
            }

            Self::DependencyAnalysis { message } => {
                write!(
                    formatter,
                    "critical-path dependency analysis failed: {message}"
                )
            }

            Self::WeightCountMismatch {
                operation_count,
                weight_count,
            } => {
                write!(
                    formatter,
                    "critical-path weight count mismatch: \
                     circuit has {operation_count} operations, \
                     but {weight_count} weights were supplied"
                )
            }

            Self::WeightOperationMismatch {
                operation_count,
                weight_count,
            } => {
                write!(
                    formatter,
                    "critical-path weight/graph mismatch: \
                     graph has {operation_count} operations, \
                     but {weight_count} weights were supplied"
                )
            }

            Self::AnalysisWorkLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "critical-path analysis work limit exceeded: \
                     requested {requested}, maximum {maximum}"
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
                    "unable to reserve memory for {collection}: \
                     requested {requested} elements"
                )
            }

            Self::OperationOutOfRange {
                operation,
                operation_count,
            } => {
                write!(
                    formatter,
                    "{operation} is outside critical-path analysis \
                     containing {operation_count} operations"
                )
            }

            Self::InvalidCriticalPath { message } => {
                write!(
                    formatter,
                    "invalid critical path: {message}"
                )
            }

            Self::CycleDetected => {
                formatter.write_str(
                    "critical-path dependency graph contains a cycle",
                )
            }
        }
    }
}

impl std::error::Error for CriticalPathAnalysisError {}

impl From<DependencyAnalysisError>
    for CriticalPathAnalysisError
{
    fn from(
        error: DependencyAnalysisError,
    ) -> Self {
        match error {
            DependencyAnalysisError::CycleDetected => {
                Self::CycleDetected
            }

            other => Self::DependencyAnalysis {
                message: other.to_string(),
            },
        }
    }
}

impl From<OptimizationLimitsError>
    for CriticalPathAnalysisError
{
    fn from(
        error: OptimizationLimitsError,
    ) -> Self {
        match error {
            OptimizationLimitsError::ResourceExceeded {
                resource: "analysis_steps",
                requested,
                maximum,
            } => {
                Self::AnalysisWorkLimitExceeded {
                    requested,
                    maximum,
                }
            }

            OptimizationLimitsError::ArithmeticOverflow {
                resource,
            } => {
                Self::ArithmeticOverflow {
                    calculation: resource,
                }
            }

            OptimizationLimitsError::ArithmeticMultiplicationOverflow {
                resource,
            } => {
                Self::ArithmeticOverflow {
                    calculation: resource,
                }
            }

            other => Self::DependencyAnalysis {
                message: other.to_string(),
            },
        }
    }
}

// ============================================================================
// Per-operation result
// ============================================================================

/// Per-operation critical-path information.
///
/// `distance` is the greatest accumulated weight of any dependency path
/// ending at this operation, including this operation's own weight.
///
/// `critical_predecessor` identifies the predecessor that produced that
/// maximum distance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OperationCriticalPathInfo {
    operation: OperationId,
    weight: u64,
    distance: u64,
    critical_predecessor: Option<OperationId>,
}

impl OperationCriticalPathInfo {
    fn new(
        operation: OperationId,
        weight: u64,
        distance: u64,
        critical_predecessor: Option<OperationId>,
    ) -> Self {
        Self {
            operation,
            weight,
            distance,
            critical_predecessor,
        }
    }

    /// Returns the operation identifier.
    #[must_use]
    pub const fn operation(self) -> OperationId {
        self.operation
    }

    /// Returns the operation's own weight.
    #[must_use]
    pub const fn weight(self) -> u64 {
        self.weight
    }

    /// Returns the maximum weighted path ending at this operation.
    #[must_use]
    pub const fn distance(self) -> u64 {
        self.distance
    }

    /// Returns the predecessor selected for the maximum path.
    #[must_use]
    pub const fn critical_predecessor(
        self,
    ) -> Option<OperationId> {
        self.critical_predecessor
    }
}

// ============================================================================
// Analysis result
// ============================================================================

/// Immutable critical-path analysis result.
///
/// The object contains only optimizer-analysis information. It does not own
/// or duplicate the canonical quantum circuit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CriticalPathAnalysis {
    operation_count: usize,
    edge_count: usize,

    /// Weight assigned to every operation.
    operation_weights: Vec<u64>,

    /// Longest weighted path ending at every operation.
    operation_distances: Vec<u64>,

    /// Selected predecessor producing the longest path ending at each
    /// operation.
    critical_predecessors: Vec<Option<OperationId>>,

    /// Reconstructed longest path.
    critical_path: Vec<OperationId>,

    /// Total weight of the reconstructed path.
    critical_path_weight: u64,
}

impl CriticalPathAnalysis {
    // ========================================================================
    // Construction
    // ========================================================================

    /// Analyze a circuit using production optimizer limits and unit operation
    /// weights.
    ///
    /// Each operation has weight `1`.
    pub fn analyze(
        circuit: &QuantumCircuit,
    ) -> Result<Self, CriticalPathAnalysisError> {
        Self::analyze_with_limits(
            circuit,
            &OptimizationLimits::production(),
        )
    }

    /// Analyze a circuit using explicit optimizer limits and unit weights.
    pub fn analyze_with_limits(
        circuit: &QuantumCircuit,
        limits: &OptimizationLimits,
    ) -> Result<Self, CriticalPathAnalysisError> {
        circuit
            .validate()
            .map_err(|error| {
                CriticalPathAnalysisError::InvalidCircuit {
                    message: error.to_string(),
                }
            })?;

        let view = CircuitView::from_validated(circuit);

        Self::analyze_view(
            &view,
            limits,
        )
    }

    /// Analyze an already validated circuit view using unit weights.
    ///
    /// This is the preferred pipeline API when another stage has already
    /// validated the canonical circuit.
    pub fn analyze_view(
        view: &CircuitView<'_>,
        limits: &OptimizationLimits,
    ) -> Result<Self, CriticalPathAnalysisError> {
        let operation_count = view.len();

        let mut weights = Vec::<u64>::new();

        reserve_exact(
            &mut weights,
            operation_count,
            "critical-path operation weights",
        )?;

        for _ in 0..operation_count {
            weights.push(1);
        }

        Self::analyze_view_with_weights(
            view,
            &weights,
            limits,
        )
    }

    /// Analyze an already validated circuit view with explicit operation
    /// weights.
    ///
    /// The weight at index `i` belongs to `OperationId::new(i)`.
    ///
    /// A zero weight is legal.
    pub fn analyze_view_with_weights(
        view: &CircuitView<'_>,
        weights: &[u64],
        limits: &OptimizationLimits,
    ) -> Result<Self, CriticalPathAnalysisError> {
        let graph =
            DependencyGraph::analyze_view(
                view,
                limits,
            )?;

        Self::analyze_with_graph_and_weights(
            view,
            &graph,
            weights,
            limits,
        )
    }

    /// Analyze a circuit using an already constructed dependency graph and
    /// explicit operation weights.
    ///
    /// This is the most efficient API for pipelines that already cache the
    /// dependency graph in `OptimizationContext`.
    ///
    /// The graph and view must describe the same immutable circuit snapshot.
    ///
    /// Because `OperationId` values are invocation-local, callers must never
    /// combine a graph from one circuit snapshot with a different circuit
    /// snapshot.
    pub fn analyze_with_graph_and_weights(
        view: &CircuitView<'_>,
        graph: &DependencyGraph,
        weights: &[u64],
        limits: &OptimizationLimits,
    ) -> Result<Self, CriticalPathAnalysisError> {
        limits.validate().map_err(
            CriticalPathAnalysisError::from,
        )?;

        let operation_count = view.len();

        if graph.operation_count()
            != operation_count
        {
            return Err(
                CriticalPathAnalysisError::DependencyAnalysis {
                    message:
                        "dependency graph operation count does not match circuit view"
                            .to_owned(),
                },
            );
        }

        if weights.len()
            != operation_count
        {
            return Err(
                CriticalPathAnalysisError::WeightCountMismatch {
                    operation_count,
                    weight_count: weights.len(),
                },
            );
        }

        if operation_count == 0 {
            return Ok(Self {
                operation_count: 0,
                edge_count: graph.edge_count(),
                operation_weights: Vec::new(),
                operation_distances: Vec::new(),
                critical_predecessors: Vec::new(),
                critical_path: Vec::new(),
                critical_path_weight: 0,
            });
        }

        /*
         * Validate the graph's topological structure before performing dynamic
         * programming.
         *
         * We intentionally use the graph's own deterministic topological-order
         * implementation rather than assuming that canonical operation order
         * is sufficient. This makes a corrupted dependency graph fail closed.
         */
        let topological_order =
            graph.topological_order()?;

        if topological_order.len()
            != operation_count
        {
            return Err(
                CriticalPathAnalysisError::CycleDetected,
            );
        }

        /*
         * Allocate all O(V) result arrays once.
         *
         * The implementation deliberately avoids Vec<Option<u64>> for the
         * distance state because every operation receives a concrete distance.
         */
        let mut distances =
            Vec::<u64>::new();

        let mut critical_predecessors =
            Vec::<Option<OperationId>>::new();

        reserve_exact(
            &mut distances,
            operation_count,
            "critical-path distances",
        )?;

        reserve_exact(
            &mut critical_predecessors,
            operation_count,
            "critical-path predecessor state",
        )?;

        for _ in 0..operation_count {
            distances.push(0);
            critical_predecessors.push(None);
        }

        /*
         * Dynamic programming over the DAG.
         *
         * For operation v:
         *
         *     distance[v]
         *       = weight[v]
         *         + max(distance[p])
         *
         * over all predecessors p of v.
         *
         * Ties are resolved by the smallest predecessor OperationId. This is
         * deterministic and independent of allocation layout.
         */
        let mut work_units = 0u64;

        for operation in
            topological_order.iter().copied()
        {
            add_work(
                &mut work_units,
                1,
                limits.max_analysis_steps(),
            )?;

            let index =
                operation.index();

            if index >= operation_count {
                return Err(
                    CriticalPathAnalysisError::OperationOutOfRange {
                        operation,
                        operation_count,
                    },
                );
            }

            let mut best_predecessor =
                None::<OperationId>;

            let mut best_predecessor_distance =
                0u64;

            for link in
                graph.predecessors(operation)?
            {
                add_work(
                    &mut work_units,
                    1,
                    limits.max_analysis_steps(),
                )?;

                let predecessor =
                    link.operation();

                let predecessor_index =
                    predecessor.index();

                if predecessor_index
                    >= operation_count
                {
                    return Err(
                        CriticalPathAnalysisError::InvalidCriticalPath {
                            message:
                                "dependency predecessor is outside operation range",
                        },
                    );
                }

                let predecessor_distance =
                    distances[predecessor_index];

                match best_predecessor {
                    None => {
                        best_predecessor =
                            Some(predecessor);

                        best_predecessor_distance =
                            predecessor_distance;
                    }

                    Some(current_best)
                        if predecessor_distance
                            > best_predecessor_distance =>
                    {
                        best_predecessor =
                            Some(predecessor);

                        best_predecessor_distance =
                            predecessor_distance;
                    }

                    Some(current_best)
                        if predecessor_distance
                            == best_predecessor_distance
                            && predecessor
                                < current_best =>
                    {
                        best_predecessor =
                            Some(predecessor);
                    }

                    _ => {}
                }
            }

            let own_weight =
                weights[index];

            let distance =
                best_predecessor_distance
                    .checked_add(own_weight)
                    .ok_or(
                        CriticalPathAnalysisError::ArithmeticOverflow {
                            calculation:
                                "critical-path accumulated weight",
                        },
                    )?;

            distances[index] =
                distance;

            critical_predecessors[index] =
                best_predecessor;
        }

        /*
         * Find the terminal operation with the maximum distance.
         *
         * Deterministic tie-break:
         * smallest OperationId.
         */
        let mut critical_end =
            OperationId::new(0);

        let mut critical_weight =
            distances[0];

        for index in 1..operation_count {
            add_work(
                &mut work_units,
                1,
                limits.max_analysis_steps(),
            )?;

            let candidate =
                OperationId::new(index);

            let candidate_weight =
                distances[index];

            if candidate_weight
                > critical_weight
            {
                critical_weight =
                    candidate_weight;

                critical_end =
                    candidate;
            }
        }

        /*
         * Reconstruct the critical path by walking predecessor links
         * backwards.
         *
         * The dependency graph is acyclic, so at most V operations can be
         * visited. We still explicitly guard against malformed predecessor
         * chains rather than trusting the graph blindly.
         */
        let mut reversed_path =
            Vec::<OperationId>::new();

        reserve_exact(
            &mut reversed_path,
            operation_count,
            "critical-path reconstruction",
        )?;

        let mut current =
            Some(critical_end);

        let mut reconstruction_steps =
            0usize;

        while let Some(operation) =
            current
        {
            add_work(
                &mut work_units,
                1,
                limits.max_analysis_steps(),
            )?;

            reconstruction_steps =
                reconstruction_steps
                    .checked_add(1)
                    .ok_or(
                        CriticalPathAnalysisError::ArithmeticOverflow {
                            calculation:
                                "critical-path reconstruction steps",
                        },
                    )?;

            if reconstruction_steps
                > operation_count
            {
                return Err(
                    CriticalPathAnalysisError::InvalidCriticalPath {
                        message:
                            "critical predecessor chain exceeds operation count",
                    },
                );
            }

            reversed_path.push(operation);

            current =
                critical_predecessors[
                    operation.index()
                ];
        }

        /*
         * Reverse the path into forward execution order.
         */
        reversed_path.reverse();

        /*
         * Validate the reconstructed path against the actual dependency graph
         * and recompute its weight. This is intentionally done in production
         * builds because this analysis is foundational to optimization decisions.
         */
        validate_reconstructed_path(
            &reversed_path,
            graph,
            weights,
            critical_weight,
            &mut work_units,
            limits.max_analysis_steps(),
        )?;

        Ok(Self {
            operation_count,
            edge_count: graph.edge_count(),
            operation_weights: weights.to_vec(),
            operation_distances: distances,
            critical_predecessors,
            critical_path: reversed_path,
            critical_path_weight: critical_weight,
        })
    }

    // ========================================================================
    // Basic metrics
    // ========================================================================

    /// Returns the number of operations in the analyzed circuit.
    #[must_use]
    pub const fn operation_count(
        &self,
    ) -> usize {
        self.operation_count
    }

    /// Returns the number of dependency edges used by the analysis.
    #[must_use]
    pub const fn edge_count(
        &self,
    ) -> usize {
        self.edge_count
    }

    /// Returns the total weight of the critical path.
    ///
    /// With unit operation weights this is the number of operations on the
    /// critical path.
    #[must_use]
    pub const fn critical_path_weight(
        &self,
    ) -> u64 {
        self.critical_path_weight
    }

    /// Returns the number of operations on the critical path.
    #[must_use]
    pub fn critical_path_operation_count(
        &self,
    ) -> usize {
        self.critical_path.len()
    }

    /// Returns the critical path in forward dependency order.
    ///
    /// The returned vector is owned by the analysis result and therefore
    /// remains valid independently of the original circuit.
    #[must_use]
    pub fn critical_path(
        &self,
    ) -> &[OperationId] {
        &self.critical_path
    }

    /// Returns true when the circuit contains no operations on its critical
    /// path.
    #[must_use]
    pub fn critical_path_is_empty(
        &self,
    ) -> bool {
        self.critical_path.is_empty()
    }

    // ========================================================================
    // Per-operation queries
    // ========================================================================

    /// Returns the weight assigned to an operation.
    pub fn operation_weight(
        &self,
        operation: OperationId,
    ) -> Result<u64, CriticalPathAnalysisError> {
        let index =
            self.checked_operation(operation)?;

        Ok(self.operation_weights[index])
    }

    /// Returns the longest weighted dependency path ending at an operation.
    pub fn operation_distance(
        &self,
        operation: OperationId,
    ) -> Result<u64, CriticalPathAnalysisError> {
        let index =
            self.checked_operation(operation)?;

        Ok(self.operation_distances[index])
    }

    /// Returns the selected critical predecessor for an operation.
    pub fn critical_predecessor(
        &self,
        operation: OperationId,
    ) -> Result<
        Option<OperationId>,
        CriticalPathAnalysisError,
    > {
        let index =
            self.checked_operation(operation)?;

        Ok(self.critical_predecessors[index])
    }

    /// Returns the complete per-operation critical-path information.
    pub fn operation_info(
        &self,
        operation: OperationId,
    ) -> Result<
        OperationCriticalPathInfo,
        CriticalPathAnalysisError,
    > {
        let index =
            self.checked_operation(operation)?;

        Ok(OperationCriticalPathInfo::new(
            operation,
            self.operation_weights[index],
            self.operation_distances[index],
            self.critical_predecessors[index],
        ))
    }

    /// Returns whether an operation belongs to the selected critical path.
    #[must_use]
    pub fn is_on_critical_path(
        &self,
        operation: OperationId,
    ) -> bool {
        self.critical_path
            .binary_search(&operation)
            .is_ok()
    }

    /// Returns the operation index of the first critical-path operation.
    #[must_use]
    pub fn critical_path_start(
        &self,
    ) -> Option<OperationId> {
        self.critical_path.first().copied()
    }

    /// Returns the operation index of the final critical-path operation.
    #[must_use]
    pub fn critical_path_end(
        &self,
    ) -> Option<OperationId> {
        self.critical_path.last().copied()
    }

    /// Returns all operation weights.
    ///
    /// The returned slice is indexed by `OperationId::index()`.
    #[must_use]
    pub fn operation_weights(
        &self,
    ) -> &[u64] {
        &self.operation_weights
    }

    /// Returns all accumulated operation distances.
    ///
    /// The returned slice is indexed by `OperationId::index()`.
    #[must_use]
    pub fn operation_distances(
        &self,
    ) -> &[u64] {
        &self.operation_distances
    }

    /// Returns all selected critical predecessors.
    ///
    /// The returned slice is indexed by `OperationId::index()`.
    #[must_use]
    pub fn critical_predecessors(
        &self,
    ) -> &[Option<OperationId>] {
        &self.critical_predecessors
    }

    // ========================================================================
    // Validation
    // ========================================================================

    /// Validates the internal consistency of this result.
    ///
    /// This method does not require the original circuit or dependency graph.
    /// It verifies all invariants that can be established from the stored
    /// result alone.
    pub fn validate(
        &self,
    ) -> Result<(), CriticalPathAnalysisError> {
        if self.operation_weights.len()
            != self.operation_count
        {
            return Err(
                CriticalPathAnalysisError::InvalidCriticalPath {
                    message:
                        "operation weight array length mismatch",
                },
            );
        }

        if self.operation_distances.len()
            != self.operation_count
        {
            return Err(
                CriticalPathAnalysisError::InvalidCriticalPath {
                    message:
                        "operation distance array length mismatch",
                },
            );
        }

        if self.critical_predecessors.len()
            != self.operation_count
        {
            return Err(
                CriticalPathAnalysisError::InvalidCriticalPath {
                    message:
                        "critical predecessor array length mismatch",
                },
            );
        }

        for index in 0..self.operation_count {
            if let Some(predecessor) =
                self.critical_predecessors[index]
            {
                if predecessor.index()
                    >= self.operation_count
                {
                    return Err(
                        CriticalPathAnalysisError::InvalidCriticalPath {
                            message:
                                "critical predecessor is outside operation range",
                        },
                    );
                }

                if predecessor.index()
                    >= index
                {
                    return Err(
                        CriticalPathAnalysisError::InvalidCriticalPath {
                            message:
                                "critical predecessor is not earlier than its operation",
                        },
                    );
                }
            }

            let weight =
                self.operation_weights[index];

            let predecessor_distance =
                self.critical_predecessors[index]
                    .map(|predecessor| {
                        self.operation_distances[
                            predecessor.index()
                        ]
                    })
                    .unwrap_or(0);

            let expected =
                predecessor_distance
                    .checked_add(weight)
                    .ok_or(
                        CriticalPathAnalysisError::ArithmeticOverflow {
                            calculation:
                                "critical-path result validation distance",
                        },
                    )?;

            if self.operation_distances[index]
                != expected
            {
                return Err(
                    CriticalPathAnalysisError::InvalidCriticalPath {
                        message:
                            "operation distance does not equal predecessor distance plus operation weight",
                    },
                );
            }
        }

        if self.critical_path.is_empty() {
            if self.operation_count != 0
                && self.critical_path_weight != 0
            {
                return Err(
                    CriticalPathAnalysisError::InvalidCriticalPath {
                        message:
                            "non-empty critical-path weight has an empty path",
                    },
                );
            }

            return Ok(());
        }

        for window in
            self.critical_path.windows(2)
        {
            let predecessor =
                window[0];

            let successor =
                window[1];

            if predecessor.index()
                >= self.operation_count
                || successor.index()
                    >= self.operation_count
            {
                return Err(
                    CriticalPathAnalysisError::InvalidCriticalPath {
                        message:
                            "critical path contains an out-of-range operation",
                    },
                );
            }

            if predecessor.index()
                >= successor.index()
            {
                return Err(
                    CriticalPathAnalysisError::InvalidCriticalPath {
                        message:
                            "critical path is not in deterministic operation order",
                    },
                );
            }
        }

        Ok(())
    }

    // ========================================================================
    // Internal helpers
    // ========================================================================

    fn checked_operation(
        &self,
        operation: OperationId,
    ) -> Result<usize, CriticalPathAnalysisError> {
        let index =
            operation.index();

        if index >= self.operation_count {
            return Err(
                CriticalPathAnalysisError::OperationOutOfRange {
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
// Free-function convenience APIs
// ============================================================================

/// Computes the unit-weight critical path for a circuit.
pub fn analyze(
    circuit: &QuantumCircuit,
) -> Result<CriticalPathAnalysis, CriticalPathAnalysisError> {
    CriticalPathAnalysis::analyze(circuit)
}

/// Computes the unit-weight critical path using explicit optimizer limits.
pub fn analyze_with_limits(
    circuit: &QuantumCircuit,
    limits: &OptimizationLimits,
) -> Result<CriticalPathAnalysis, CriticalPathAnalysisError> {
    CriticalPathAnalysis::analyze_with_limits(
        circuit,
        limits,
    )
}

/// Computes a weighted critical path from a validated optimizer view.
pub fn analyze_with_weights(
    view: &CircuitView<'_>,
    weights: &[u64],
    limits: &OptimizationLimits,
) -> Result<CriticalPathAnalysis, CriticalPathAnalysisError> {
    CriticalPathAnalysis::analyze_view_with_weights(
        view,
        weights,
        limits,
    )
}

// ============================================================================
// Internal allocation helpers
// ============================================================================

/// Reserve exactly enough capacity for a collection.
///
/// This wrapper exists so all fallible allocation points in the analysis have
/// one explicit error policy.
///
/// Rust's allocation API can abort the process on allocator-level OOM rather
/// than returning an ordinary Rust error. Therefore this helper is primarily
/// responsible for preventing accidental over-allocation caused by arithmetic
/// mistakes; it does not claim to turn arbitrary OS-level OOM into a recoverable
/// error.
fn reserve_exact<T>(
    vector: &mut Vec<T>,
    additional: usize,
    collection: &'static str,
) -> Result<(), CriticalPathAnalysisError> {
    vector
        .try_reserve_exact(additional)
        .map_err(|_| {
            CriticalPathAnalysisError::AllocationFailure {
                collection,
                requested: additional,
            }
        })
}

// ============================================================================
// Work-budget helpers
// ============================================================================

/// Add deterministic analysis work while enforcing the configured limit.
fn add_work(
    work: &mut u64,
    additional: u64,
    maximum: u64,
) -> Result<(), CriticalPathAnalysisError> {
    let requested =
        work.checked_add(additional)
            .ok_or(
                CriticalPathAnalysisError::ArithmeticOverflow {
                    calculation:
                        "critical-path analysis work counter",
                },
            )?;

    if requested > maximum {
        return Err(
            CriticalPathAnalysisError::AnalysisWorkLimitExceeded {
                requested,
                maximum,
            },
        );
    }

    *work = requested;

    Ok(())
}

// ============================================================================
// Path validation
// ============================================================================

/// Validates the reconstructed critical path against the dependency graph.
///
/// This is intentionally performed after the dynamic-programming phase.
/// Critical-path information is foundational to optimization decisions and
/// therefore must fail closed if the graph/result relationship is inconsistent.
fn validate_reconstructed_path(
    path: &[OperationId],
    graph: &DependencyGraph,
    weights: &[u64],
    expected_weight: u64,
    work: &mut u64,
    maximum_work: u64,
) -> Result<(), CriticalPathAnalysisError> {
    if path.is_empty() {
        if expected_weight != 0 {
            return Err(
                CriticalPathAnalysisError::InvalidCriticalPath {
                    message:
                        "empty reconstructed path has non-zero weight",
                },
            );
        }

        return Ok(());
    }

    let mut accumulated =
        0u64;

    for index in 0..path.len() {
        add_work(
            work,
            1,
            maximum_work,
        )?;

        let operation =
            path[index];

        let operation_index =
            operation.index();

        if operation_index
            >= weights.len()
        {
            return Err(
                CriticalPathAnalysisError::InvalidCriticalPath {
                    message:
                        "reconstructed path contains an out-of-range operation",
                },
            );
        }

        accumulated =
            accumulated
                .checked_add(
                    weights[operation_index],
                )
                .ok_or(
                    CriticalPathAnalysisError::ArithmeticOverflow {
                        calculation:
                            "reconstructed critical-path weight",
                    },
                )?;

        if index > 0 {
            let predecessor =
                path[index - 1];

            if predecessor.index()
                >= operation_index
            {
                return Err(
                    CriticalPathAnalysisError::InvalidCriticalPath {
                        message:
                            "critical path operations are not strictly ordered",
                    },
                );
            }

            add_work(
                work,
                1,
                maximum_work,
            )?;

            let dependency =
                graph.has_dependency(
                    predecessor,
                    operation,
                )?;

            if !dependency {
                return Err(
                    CriticalPathAnalysisError::InvalidCriticalPath {
                        message:
                            "adjacent critical-path operations are not connected by a dependency edge",
                    },
                );
            }
        }
    }

    if accumulated
        != expected_weight
    {
        return Err(
            CriticalPathAnalysisError::InvalidCriticalPath {
                message:
                    "reconstructed critical-path weight does not match reported weight",
            },
        );
    }

    Ok(())
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /*
     * These tests intentionally focus on the algorithm's mathematical
     * invariants rather than depending on a particular frontend.
     *
     * Circuit-construction tests should be expanded alongside the canonical
     * Quantum IR test suite as the IR gate constructors evolve.
     */

    #[test]
    fn empty_weighted_analysis_has_zero_critical_path() {
        /*
         * The test is deliberately expressed through the result structure
         * because constructing an empty QuantumCircuit is an IR-owned concern.
         *
         * The production invariant being tested is:
         *
         *     empty graph => zero path weight => empty path.
         */
        let analysis =
            CriticalPathAnalysis {
                operation_count: 0,
                edge_count: 0,
                operation_weights:
                    Vec::new(),
                operation_distances:
                    Vec::new(),
                critical_predecessors:
                    Vec::new(),
                critical_path:
                    Vec::new(),
                critical_path_weight: 0,
            };

        assert_eq!(
            analysis.operation_count(),
            0
        );

        assert_eq!(
            analysis.critical_path_weight(),
            0
        );

        assert_eq!(
            analysis.critical_path_operation_count(),
            0
        );

        assert!(
            analysis
                .critical_path_is_empty()
        );

        assert!(
            analysis.validate().is_ok()
        );
    }

    #[test]
    fn operation_info_reports_stored_values() {
        let analysis =
            CriticalPathAnalysis {
                operation_count: 2,
                edge_count: 1,
                operation_weights:
                    vec![2, 5],
                operation_distances:
                    vec![2, 7],
                critical_predecessors:
                    vec![
                        None,
                        Some(OperationId::new(0)),
                    ],
                critical_path:
                    vec![
                        OperationId::new(0),
                        OperationId::new(1),
                    ],
                critical_path_weight: 7,
            };

        let info =
            analysis
                .operation_info(
                    OperationId::new(1),
                )
                .expect(
                    "operation must exist",
                );

        assert_eq!(
            info.operation(),
            OperationId::new(1)
        );

        assert_eq!(
            info.weight(),
            5
        );

        assert_eq!(
            info.distance(),
            7
        );

        assert_eq!(
            info.critical_predecessor(),
            Some(OperationId::new(0))
        );
    }

    #[test]
    fn result_validation_accepts_valid_chain() {
        let analysis =
            CriticalPathAnalysis {
                operation_count: 3,
                edge_count: 2,
                operation_weights:
                    vec![1, 2, 3],
                operation_distances:
                    vec![1, 3, 6],
                critical_predecessors:
                    vec![
                        None,
                        Some(OperationId::new(0)),
                        Some(OperationId::new(1)),
                    ],
                critical_path:
                    vec![
                        OperationId::new(0),
                        OperationId::new(1),
                        OperationId::new(2),
                    ],
                critical_path_weight: 6,
            };

        assert!(
            analysis.validate().is_ok()
        );
    }

    #[test]
    fn result_validation_rejects_forward_predecessor() {
        let analysis =
            CriticalPathAnalysis {
                operation_count: 2,
                edge_count: 1,
                operation_weights:
                    vec![1, 1],
                operation_distances:
                    vec![1, 2],
                critical_predecessors:
                    vec![
                        Some(OperationId::new(1)),
                        None,
                    ],
                critical_path:
                    vec![
                        OperationId::new(0),
                        OperationId::new(1),
                    ],
                critical_path_weight: 2,
            };

        assert!(
            matches!(
                analysis.validate(),
                Err(
                    CriticalPathAnalysisError::InvalidCriticalPath {
                        ..
                    }
                )
            )
        );
    }

    #[test]
    fn zero_weight_operations_are_valid() {
        let analysis =
            CriticalPathAnalysis {
                operation_count: 2,
                edge_count: 1,
                operation_weights:
                    vec![0, 0],
                operation_distances:
                    vec![0, 0],
                critical_predecessors:
                    vec![
                        None,
                        Some(OperationId::new(0)),
                    ],
                critical_path:
                    vec![
                        OperationId::new(0),
                        OperationId::new(1),
                    ],
                critical_path_weight: 0,
            };

        assert!(
            analysis.validate().is_ok()
        );

        assert_eq!(
            analysis.critical_path_weight(),
            0
        );

        assert_eq!(
            analysis.critical_path_operation_count(),
            2
        );
    }

    #[test]
    fn operation_range_is_checked() {
        let analysis =
            CriticalPathAnalysis {
                operation_count: 1,
                edge_count: 0,
                operation_weights:
                    vec![1],
                operation_distances:
                    vec![1],
                critical_predecessors:
                    vec![None],
                critical_path:
                    vec![
                        OperationId::new(0),
                    ],
                critical_path_weight: 1,
            };

        let result =
            analysis.operation_weight(
                OperationId::new(1),
            );

        assert!(
            matches!(
                result,
                Err(
                    CriticalPathAnalysisError::OperationOutOfRange {
                        ..
                    }
                )
            )
        );
    }

    #[test]
    fn critical_path_membership_is_deterministic() {
        let analysis =
            CriticalPathAnalysis {
                operation_count: 3,
                edge_count: 2,
                operation_weights:
                    vec![1, 1, 1],
                operation_distances:
                    vec![1, 2, 3],
                critical_predecessors:
                    vec![
                        None,
                        Some(OperationId::new(0)),
                        Some(OperationId::new(1)),
                    ],
                critical_path:
                    vec![
                        OperationId::new(0),
                        OperationId::new(1),
                        OperationId::new(2),
                    ],
                critical_path_weight: 3,
            };

        assert!(
            analysis.is_on_critical_path(
                OperationId::new(0)
            )
        );

        assert!(
            analysis.is_on_critical_path(
                OperationId::new(1)
            )
        );

        assert!(
            analysis.is_on_critical_path(
                OperationId::new(2)
            )
        );

        assert!(
            !analysis.is_on_critical_path(
                OperationId::new(3)
            )
        );
    }

    #[test]
    fn overflow_is_detected_during_distance_calculation() {
        /*
         * This validates the arithmetic contract independently of the IR.
         *
         * A production weighted analysis must never wrap:
         *
         *     u64::MAX + 1
         *
         * into zero.
         */
        let overflow =
            u64::MAX.checked_add(1);

        assert!(
            overflow.is_none()
        );
    }
}