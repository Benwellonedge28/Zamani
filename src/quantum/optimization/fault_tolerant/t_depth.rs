#![forbid(unsafe_code)]

//! Zamani Quantum Optimization — Fault-Tolerant T-Depth Analysis.
//!
//! Production-grade, exact, backend-independent T-depth analysis over the
//! canonical Zamani Quantum IR.
//!
//! # Architectural position
//!
//! ```text
//! Zamani source
//!      │
//!      ▼
//! quantum::ir::QuantumCircuit
//!      │
//!      ▼
//! optimization::fault_tolerant::t_depth
//!      │
//!      ├── exact T-depth analysis
//!      ├── T-layer statistics
//!      └── dependency-aware resource information
//!      │
//!      ▼
//! optimization / routing / scheduling
//! ```
//!
//! # Purpose
//!
//! This module computes the logical T-depth of a canonical Quantum IR circuit.
//!
//! T-depth is a distinct resource from:
//!
//! - ordinary circuit depth;
//! - gate count;
//! - T-count;
//! - two-qubit gate count;
//! - physical execution time.
//!
//! In particular:
//!
//! ```text
//! T-count != T-depth
//! ```
//!
//! For example:
//!
//! ```text
//! T q0
//! T q1
//! ```
//!
//! has T-count 2 and logical T-depth 1 because the two operations have
//! independent logical qubit dependencies.
//!
//! Conversely:
//!
//! ```text
//! T q0
//! CX q0,q1
//! T q1
//! ```
//!
//! has T-depth 2 because the CX establishes a dependency frontier between
//! q0 and q1.
//!
//! # Important semantic boundary
//!
//! This module measures **ordered logical dependency T-depth**.
//!
//! It does not perform:
//!
//! - hardware scheduling;
//! - pulse scheduling;
//! - routing;
//! - physical-qubit allocation;
//! - backend execution;
//! - QPU communication;
//! - commutation guessing;
//! - phase-polynomial rewriting;
//! - ZX-calculus rewriting;
//! - global T-depth minimization;
//! - approximate synthesis.
//!
//! Those responsibilities belong to other optimization/compiler layers.
//!
//! A future transformation may legally reorder T operations using commutation,
//! phase-polynomial identities, Clifford synthesis, or another exact rewrite.
//! After such a transformation this analysis can be run again to measure the
//! resulting T-depth.
//!
//! Keeping analysis separate from transformation is intentional. It prevents
//! this file from making an unproved assumption that two operations commute.
//!
//! # T-depth definition
//!
//! Every logical qubit maintains a T-depth dependency frontier.
//!
//! For an operation touching:
//!
//! ```text
//! q0, q1, ..., qn
//! ```
//!
//! define:
//!
//! ```text
//! frontier = max(
//!     T_frontier[q0],
//!     T_frontier[q1],
//!     ...,
//!     T_frontier[qn]
//! )
//! ```
//!
//! For T/Tdg:
//!
//! ```text
//! T layer = frontier + 1
//! ```
//!
//! and the touched qubits receive that new frontier.
//!
//! For an ordinary non-T operation:
//!
//! ```text
//! T_frontier[q0..qn] = frontier
//! ```
//!
//! This is important for multi-qubit operations.
//!
//! For example:
//!
//! ```text
//! T q0
//! CX q0,q1
//! T q1
//! ```
//!
//! starts with:
//!
//! ```text
//! q0 = 1
//! q1 = 0
//! ```
//!
//! The CX synchronizes the two frontiers:
//!
//! ```text
//! q0 = 1
//! q1 = 1
//! ```
//!
//! Therefore the following T on q1 becomes T layer 2.
//!
//! # Measurement and reset
//!
//! Measurement and reset are semantic quantum-state boundaries.
//!
//! They do not consume T layers and do not propagate a previous quantum T
//! frontier into a newly established quantum state.
//!
//! Therefore:
//!
//! ```text
//! T q0
//! reset q0
//! T q0
//! ```
//!
//! has T-depth 1 rather than 2.
//!
//! Classical-control dependencies are not inferred from a plain Gate sequence.
//! If a future higher-level control-flow IR carries classical dependencies,
//! that control-flow layer must provide them explicitly.
//!
//! # Barriers
//!
//! A barrier synchronizes the T-depth frontier of every qubit it contains.
//!
//! A barrier itself does not consume a T layer.
//!
//! Example:
//!
//! ```text
//! T q0
//! barrier q0,q1
//! T q1
//! ```
//!
//! has T-depth 2.
//!
//! # T-family classification
//!
//! Only canonical, parameter-free:
//!
//! ```text
//! GateKind::T
//! GateKind::Tdg
//! ```
//!
//! are classified as T-family operations.
//!
//! A generic parameterized gate such as:
//!
//! ```text
//! RZ(theta)
//! ```
//!
//! is NOT treated as a T gate merely because a particular value of `theta`
//! might equal π/4.
//!
//! Value-sensitive Clifford/T classification belongs to a dedicated algebraic
//! subsystem.
//!
//! # Scaling
//!
//! Let:
//!
//! - `N` = number of circuit operations;
//! - `A` = total number of logical-qubit operands;
//! - `K` = number of distinct logical qubits actually touched;
//! - `T` = number of T/Tdg operations.
//!
//! Scalar-only analysis:
//!
//! ```text
//! Time:   expected O(N + A)
//! Memory: O(K)
//! ```
//!
//! Full report:
//!
//! ```text
//! Time:   expected O(N + A)
//! Memory: O(K + T)
//! ```
//!
//! No artificial circuit-size ceiling is introduced.
//!
//! Sparse logical namespaces are handled with a hash map rather than allocating
//! one entry for every possible logical qubit index.
//!
//! Practical limits remain those of:
//!
//! - canonical Quantum IR limits;
//! - OptimizationLimits;
//! - OptimizationContext cancellation policy;
//! - host address space;
//! - available memory;
//! - Rust allocation limits.
//!
//! The `depth()` API should be preferred when a caller only needs the scalar
//! metric for very large circuits because it does not retain per-operation or
//! per-layer report data.
//!
//! # Integer policy
//!
//! Layer identifiers use `usize` because they must remain directly usable for
//! vector indexing.
//!
//! Aggregate resource counters use `u128` to avoid unnecessary narrowing in
//! optimizer accounting.
//!
//! Circuit collection lengths originate from `usize`, so checked conversions
//! are used whenever a value crosses an accounting boundary.
//!
//! No floating-point arithmetic is used.
//!
//! # Determinism
//!
//! The result is deterministic for a deterministic canonical circuit.
//!
//! Hash-map iteration order is never exposed as semantic ordering.
//!
//! Explicit layer records preserve canonical circuit operation order.
//!
//! No:
//!
//! - random numbers;
//! - threads;
//! - global mutable state;
//! - ambient backend state;
//! - unsafe code;
//!
//! are required.
//!
//! # Mutation policy
//!
//! This module never mutates the supplied circuit.
//!
//! It implements `OptimizationPass` because the optimization framework uses a
//! common pass abstraction for both transformations and analyses.
//!
//! The pass returns `PassOutcome::unchanged(...)` after performing the analysis.
//!
//! Actual T-depth reduction must be performed by another exact transformation
//! pass, using this analysis as a cost signal.
//!
//! # Integration contract
//!
//! ## `quantum::ir`
//!
//! Uses only the canonical:
//!
//! - `QuantumCircuit`;
//! - `Gate`;
//! - `GateKind`;
//! - `QubitId`.
//!
//! No optimizer-specific quantum gate or circuit representation is created.
//!
//! ## `optimization::pass`
//!
//! Implements:
//!
//! ```text
//! OptimizationPass
//! ```
//!
//! with stable identifier:
//!
//! ```text
//! fault_tolerant.t_depth
//! ```
//!
//! ## `optimization::context`
//!
//! Uses:
//!
//! ```text
//! check_cancelled()
//! ```
//!
//! The module does not create a second resource-limit or cancellation system.
//!
//! ## `optimization::errors`
//!
//! All public pass execution failures use the canonical `OptimizationError`.
//!
//! ## `fault_tolerant::t_gate_reduction`
//!
//! Local T-power reduction should normally run before this analysis when the
//! optimization objective is T-depth-aware.
//!
//! For example:
//!
//! ```text
//! T T T T
//! ```
//!
//! can become:
//!
//! ```text
//! Z
//! ```
//!
//! eliminating T-depth entirely.
//!
//! ## `fault_tolerant::t_count`
//!
//! T-count and T-depth remain separate metrics.
//!
//! A transformation can:
//!
//! - reduce T-count without reducing T-depth;
//! - reduce T-depth without reducing T-count;
//! - reduce both;
//! - trade one against the other.
//!
//! ## `algebra::phase_polynomial`
//!
//! Phase-polynomial optimization may invoke this analysis before and after
//! transformation to measure T-depth improvement.
//!
//! This module deliberately does not depend on phase-polynomial optimization,
//! preventing a dependency cycle.
//!
//! ## `analysis::depth`
//!
//! Ordinary logical depth is a different metric.
//!
//! `analysis::depth` owns ordinary circuit-depth semantics.
//!
//! This module owns T-depth semantics.
//!
//! ## `passes::optimize_fault_tolerance`
//!
//! This composite pass may select T-depth as one of its optimization
//! objectives and consume `TDepthAnalysis` when comparing candidate circuits.
//!
//! ## `cost.rs`
//!
//! The general optimization cost model owns the `TDepth` objective.
//!
//! This file supplies the structural metric; it does not decide whether
//! T-depth is more important than T-count, gate count, error, duration, or
//! another objective.
//!
//! ## `routing`
//!
//! Routing must remain outside this module.
//!
//! Routing may introduce additional physical/logical operations, after which
//! T-depth may be recalculated if the compiler's policy requires it.
//!
//! ## `scheduling`
//!
//! Scheduling owns physical execution timing.
//!
//! Logical T-depth is not a pulse-duration metric.
//!
//! ## `verification`
//!
//! Any transformation guided by this metric must still be semantically
//! verified according to the configured equivalence policy.
//!
//! ## `registry.rs`
//!
//! Register this implementation under:
//!
//! ```text
//! fault_tolerant.t_depth
//! ```
//!
//! ## `fault_tolerant/mod.rs`
//!
//! Add:
//!
//! ```text
//! pub mod t_depth;
//! ```
//!
//! Recommended re-exports:
//!
//! ```text
//! pub use t_depth::{analyze_t_depth, TDepthAnalysis, TDepthPass};
//! ```
//!
//! No change to this file is required when those integration files are later
//! implemented because this module already exposes the stable API.
//!
//! # Rust compatibility
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust only;
//! - no nightly features;
//! - no external dependencies;
//! - no unsafe code.
//!
//! # Safety
//!
//! ```text
//! #![forbid(unsafe_code)]
//! ```
//!
//! is applied to this complete module.

use std::collections::HashMap;
use std::fmt;

use crate::quantum::ir::{
    Gate,
    GateKind,
    QubitId,
    QuantumCircuit,
};

use super::super::context::OptimizationContext;
use super::super::errors::{
    OptimizationError,
    OptimizationStage,
    PassIdentifier,
};
use super::super::pass::{
    OptimizationPass,
    PassCapability,
    PassComplexity,
    PassExecutionResult,
    PassKind,
    PassMetadata,
    PassOutcome,
    PassScope,
};

/// Stable registry/planner identifier.
pub const PASS_ID: &str = "fault_tolerant.t_depth";

/// Stable human-readable pass name.
pub const PASS_NAME: &str =
    "Fault-Tolerant T-Depth Analysis";

/// Stable analysis schema version used by provenance/reproducibility.
pub const PASS_SCHEMA_VERSION: u32 = 1;

/// A one-based T layer.
///
/// Zero means that no T operation has yet occurred on a dependency frontier.
pub type TLayer = usize;

/// Canonical operation position within the Quantum IR.
pub type OperationIndex = usize;

/// Result type returned by the T-depth analysis API.
pub type TDepthResult<T> = Result<T, OptimizationError>;

/// A single logical T layer.
///
/// Operation indices are sorted in canonical circuit order. The layer contains
/// only T/Tdg operations; non-T operations are represented implicitly by the
/// dependency frontier used to construct the layer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TLayerInfo {
    layer: TLayer,
    operations: Vec<OperationIndex>,
}

impl TLayerInfo {
    fn new(layer: TLayer) -> Self {
        Self {
            layer,
            operations: Vec::new(),
        }
    }

    fn push(
        &mut self,
        operation: OperationIndex,
    ) {
        self.operations.push(operation);
    }

    /// Returns the one-based T layer number.
    #[must_use]
    pub const fn layer(&self) -> TLayer {
        self.layer
    }

    /// Returns T/Tdg operation indices in canonical circuit order.
    #[must_use]
    pub fn operations(&self) -> &[OperationIndex] {
        &self.operations
    }

    /// Returns the number of T/Tdg operations in this layer.
    #[must_use]
    pub fn width(&self) -> usize {
        self.operations.len()
    }
}

/// Per-operation T-depth assignment.
///
/// `None` means the operation is not a T/Tdg operation. This vector is in
/// canonical circuit order and is therefore safe to correlate directly with
/// `QuantumCircuit::operations()`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperationTLayer {
    operation: OperationIndex,
    layer: Option<TLayer>,
}

impl OperationTLayer {
    fn new(
        operation: OperationIndex,
        layer: Option<TLayer>,
    ) -> Self {
        Self {
            operation,
            layer,
        }
    }

    /// Returns the operation index.
    #[must_use]
    pub const fn operation(&self) -> OperationIndex {
        self.operation
    }

    /// Returns the T layer, or `None` for non-T operations.
    #[must_use]
    pub const fn layer(&self) -> Option<TLayer> {
        self.layer
    }
}

/// Exact logical T-depth report.
///
/// This report is immutable after construction and therefore safe to cache in
/// an optimizer analysis cache.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TDepthAnalysis {
    depth: TLayer,
    t_count: u128,
    layer_count: u128,
    maximum_layer_width: u128,
    layers: Vec<TLayerInfo>,
    operation_layers: Vec<OperationTLayer>,
}

impl TDepthAnalysis {
    fn empty() -> Self {
        Self {
            depth: 0,
            t_count: 0,
            layer_count: 0,
            maximum_layer_width: 0,
            layers: Vec::new(),
            operation_layers: Vec::new(),
        }
    }

    /// Returns the logical T-depth.
    #[must_use]
    pub const fn depth(&self) -> TLayer {
        self.depth
    }

    /// Returns the number of T/Tdg operations.
    #[must_use]
    pub const fn t_count(&self) -> u128 {
        self.t_count
    }

    /// Returns the number of non-empty T layers.
    #[must_use]
    pub const fn layer_count(&self) -> u128 {
        self.layer_count
    }

    /// Returns the maximum number of T/Tdg operations in one T layer.
    #[must_use]
    pub const fn maximum_layer_width(&self) -> u128 {
        self.maximum_layer_width
    }

    /// Returns the explicit T layers in ascending layer order.
    #[must_use]
    pub fn layers(&self) -> &[TLayerInfo] {
        &self.layers
    }

    /// Returns per-operation T-layer assignments in canonical operation order.
    #[must_use]
    pub fn operation_layers(&self) -> &[OperationTLayer] {
        &self.operation_layers
    }

    /// Returns whether the circuit contains no T/Tdg operations.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.t_count == 0
    }

    /// Returns the maximum number of T/Tdg operations that occur in parallel
    /// in one logical T layer.
    #[must_use]
    pub const fn max_parallel_t_gates(&self) -> u128 {
        self.maximum_layer_width
    }
}

/// Exact T-depth analysis pass.
///
/// The pass is stateless between invocations. All mutable execution state is
/// supplied by `OptimizationContext`.
#[derive(Debug, Clone)]
pub struct TDepthPass {
    metadata: PassMetadata,
}

impl TDepthPass {
    /// Constructs the production T-depth analysis pass.
    #[must_use]
    pub fn new() -> Self {
        let identifier = PassIdentifier::new(PASS_ID)
            .expect(
                "fault_tolerant.t_depth is a valid static identifier",
            );

        let metadata = PassMetadata::new(
            identifier,
            PASS_NAME,
            PassKind::Analysis,
        )
        .expect(
            "T-depth analysis metadata must be valid",
        )
        .with_description(
            "Exact logical T-depth analysis over canonical Quantum IR dependencies.",
        )
        .with_scope(PassScope::Circuit)
        .with_complexity(PassComplexity::Linear)
        .with_capability(PassCapability::AnalysisOnly)
        .with_semantic_preservation(true)
        .supports_empty_circuit(true)
        .supports_single_operation(true)
        .supports_large_circuits(true)
        .fixed_point_safe(true);

        Self { metadata }
    }

    /// Computes a complete T-depth report without changing the circuit.
    ///
    /// This API retains per-layer and per-operation information.
    pub fn analyze(
        &self,
        circuit: &QuantumCircuit,
        context: &mut OptimizationContext,
    ) -> TDepthResult<TDepthAnalysis> {
        context
            .check_cancelled()
            .map_err(|error| {
                cancelled_error(
                    "T-depth analysis cannot start",
                    error,
                )
            })?;

        circuit.validate().map_err(|error| {
            OptimizationError::invalid_input(
                OptimizationStage::InputValidation,
                format!(
                    "T-depth analysis received invalid Quantum IR: {error}"
                ),
            )
        })?;

        let operations = circuit.operations();

        if operations.is_empty() {
            return Ok(TDepthAnalysis::empty());
        }

        let mut frontier: HashMap<QubitId, TLayer> =
            HashMap::new();

        let mut layers: Vec<TLayerInfo> = Vec::new();

        let mut operation_layers: Vec<OperationTLayer> =
            Vec::with_capacity(operations.len());

        let mut depth: TLayer = 0;
        let mut t_count: u128 = 0;
        let mut maximum_layer_width: u128 = 0;

        for (operation_index, gate) in
            operations.iter().enumerate()
        {
            if operation_index & 0x3ff == 0 {
                context
                    .check_cancelled()
                    .map_err(|error| {
                        cancelled_error(
                            "T-depth analysis was cancelled",
                            error,
                        )
                    })?;
            }

            let qubits = gate.qubits();

            if qubits.is_empty() {
                return Err(
                    OptimizationError::invalid_input(
                        OptimizationStage::FaultTolerantOptimization,
                        format!(
                            "operation {operation_index} has no logical qubit operands"
                        ),
                    ),
                );
            }

            let mut dependency_frontier: TLayer = 0;

            for &qubit in qubits {
                let value =
                    frontier.get(&qubit).copied().unwrap_or(0);

                dependency_frontier =
                    dependency_frontier.max(value);
            }

            if is_t_family_gate(gate) {
                let layer =
                    dependency_frontier
                        .checked_add(1)
                        .ok_or_else(|| {
                            OptimizationError::internal(
                                OptimizationStage::FaultTolerantOptimization,
                                "T-depth layer counter overflow",
                            )
                        })?;

                depth = depth.max(layer);

                t_count =
                    t_count.checked_add(1).ok_or_else(|| {
                        OptimizationError::internal(
                            OptimizationStage::FaultTolerantOptimization,
                            "T-gate count overflow",
                        )
                    })?;

                let layer_index =
                    layer.checked_sub(1).ok_or_else(|| {
                        OptimizationError::internal(
                            OptimizationStage::FaultTolerantOptimization,
                            "invalid one-based T-depth layer",
                        )
                    })?;

                if layer_index == layers.len() {
                    layers.push(TLayerInfo::new(layer));
                } else if layer_index > layers.len() {
                    return Err(
                        OptimizationError::internal(
                            OptimizationStage::FaultTolerantOptimization,
                            "T-depth layer sequence became non-contiguous",
                        ),
                    );
                }

                layers[layer_index]
                    .push(operation_index);

                let width =
                    u128::try_from(
                        layers[layer_index].width(),
                    )
                    .map_err(|_| {
                        OptimizationError::internal(
                            OptimizationStage::FaultTolerantOptimization,
                            "T-layer width cannot be represented by optimizer counters",
                        )
                    })?;

                maximum_layer_width =
                    maximum_layer_width.max(width);

                for &qubit in qubits {
                    frontier.insert(qubit, layer);
                }

                operation_layers.push(
                    OperationTLayer::new(
                        operation_index,
                        Some(layer),
                    ),
                );
            } else if gate.is_reset()
                || gate.is_measurement()
            {
                // Reset and measurement establish a new quantum-state
                // boundary. The previous T frontier must not leak through
                // that boundary.
                //
                // Classical dependencies, when represented by a higher-level
                // control-flow IR, must be modeled by that layer rather than
                // inferred here.
                for &qubit in qubits {
                    frontier.insert(qubit, 0);
                }

                operation_layers.push(
                    OperationTLayer::new(
                        operation_index,
                        None,
                    ),
                );
            } else {
                // Non-T operations do not consume a T layer, but a
                // multi-qubit operation synchronizes the dependency
                // frontiers of all qubits it touches.
                for &qubit in qubits {
                    frontier.insert(
                        qubit,
                        dependency_frontier,
                    );
                }

                operation_layers.push(
                    OperationTLayer::new(
                        operation_index,
                        None,
                    ),
                );
            }
        }

        let layer_count =
            u128::try_from(layers.len()).map_err(
                |_| {
                    OptimizationError::internal(
                        OptimizationStage::FaultTolerantOptimization,
                        "T-layer count cannot be represented by optimizer counters",
                    )
                },
            )?;

        Ok(TDepthAnalysis {
            depth,
            t_count,
            layer_count,
            maximum_layer_width,
            layers,
            operation_layers,
        })
    }

    /// Computes only the scalar T-depth.
    ///
    /// This is the preferred API for very large circuits when the caller does
    /// not need explicit layer membership or per-operation assignments.
    ///
    /// Memory is O(K), rather than O(K + T).
    pub fn depth(
        &self,
        circuit: &QuantumCircuit,
        context: &mut OptimizationContext,
    ) -> TDepthResult<TLayer> {
        context
            .check_cancelled()
            .map_err(|error| {
                cancelled_error(
                    "T-depth calculation cannot start",
                    error,
                )
            })?;

        circuit.validate().map_err(|error| {
            OptimizationError::invalid_input(
                OptimizationStage::InputValidation,
                format!(
                    "T-depth calculation received invalid Quantum IR: {error}"
                ),
            )
        })?;

        let mut frontier: HashMap<QubitId, TLayer> =
            HashMap::new();

        let mut depth: TLayer = 0;

        for (operation_index, gate) in
            circuit.operations().iter().enumerate()
        {
            if operation_index & 0x3ff == 0 {
                context
                    .check_cancelled()
                    .map_err(|error| {
                        cancelled_error(
                            "T-depth calculation was cancelled",
                            error,
                        )
                    })?;
            }

            let qubits = gate.qubits();

            if qubits.is_empty() {
                return Err(
                    OptimizationError::invalid_input(
                        OptimizationStage::FaultTolerantOptimization,
                        format!(
                            "operation {operation_index} has no logical qubit operands"
                        ),
                    ),
                );
            }

            let dependency_frontier =
                qubits
                    .iter()
                    .map(|qubit| {
                        frontier
                            .get(qubit)
                            .copied()
                            .unwrap_or(0)
                    })
                    .max()
                    .unwrap_or(0);

            if is_t_family_gate(gate) {
                let layer =
                    dependency_frontier
                        .checked_add(1)
                        .ok_or_else(|| {
                            OptimizationError::internal(
                                OptimizationStage::FaultTolerantOptimization,
                                "T-depth layer counter overflow",
                            )
                        })?;

                depth = depth.max(layer);

                for &qubit in qubits {
                    frontier.insert(qubit, layer);
                }
            } else if gate.is_reset()
                || gate.is_measurement()
            {
                for &qubit in qubits {
                    frontier.insert(qubit, 0);
                }
            } else {
                for &qubit in qubits {
                    frontier.insert(
                        qubit,
                        dependency_frontier,
                    );
                }
            }
        }

        Ok(depth)
    }

    /// Returns whether a canonical gate is a parameter-free T/Tdg operation.
    #[must_use]
    pub fn is_t_gate(gate: &Gate) -> bool {
        is_t_family_gate(gate)
    }

    /// Returns the stable pass identifier.
    #[must_use]
    pub fn pass_id() -> &'static str {
        PASS_ID
    }

    /// Returns the analysis schema version.
    #[must_use]
    pub const fn schema_version() -> u32 {
        PASS_SCHEMA_VERSION
    }
}

/// Computes an exact T-depth report using the production analysis
/// implementation.
///
/// This free function is the preferred integration point for other optimizer
/// modules that do not need to retain a `TDepthPass` instance.
pub fn analyze_t_depth(
    circuit: &QuantumCircuit,
    context: &mut OptimizationContext,
) -> TDepthResult<TDepthAnalysis> {
    TDepthPass::new().analyze(circuit, context)
}

impl Default for TDepthPass {
    fn default() -> Self {
        Self::new()
    }
}

impl OptimizationPass for TDepthPass {
    fn metadata(&self) -> &PassMetadata {
        &self.metadata
    }

    fn run(
        &self,
        circuit: &mut QuantumCircuit,
        context: &mut OptimizationContext,
    ) -> PassExecutionResult {
        let operations =
            u64::try_from(circuit.len()).map_err(
                |_| {
                    OptimizationError::internal(
                        OptimizationStage::FaultTolerantOptimization,
                        "operation count cannot be represented by optimizer counters",
                    )
                },
            )?;

        let _report =
            self.analyze(circuit, context)?;

        // Analysis-only by design. No circuit mutation occurs.
        Ok(PassOutcome::unchanged(
            operations,
            operations,
        ))
    }
}

/// Returns whether a gate is exactly a parameter-free T/Tdg operation.
#[must_use]
fn is_t_family_gate(gate: &Gate) -> bool {
    matches!(
        gate.kind(),
        GateKind::T | GateKind::Tdg
    ) && gate.parameters().is_empty()
        && gate.qubits().len() == 1
}

fn cancelled_error(
    prefix: &str,
    error: impl fmt::Display,
) -> OptimizationError {
    OptimizationError::internal(
        OptimizationStage::FaultTolerantOptimization,
        format!("{prefix}: {error}"),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::quantum::ir::Gate;
    use crate::quantum::ir::QubitId;
    use crate::quantum::ir::QuantumCircuit;
    use crate::quantum::optimization::config::OptimizationConfig;
    use crate::quantum::optimization::context::OptimizationContext;
    use crate::quantum::optimization::limits::OptimizationLimits;

    fn gate(
        kind: GateKind,
        qubits: &[usize],
    ) -> Gate {
        Gate::new(
            kind,
            qubits
                .iter()
                .copied()
                .map(|index| {
                    QubitId::new(index)
                        .expect(
                            "test qubit identifier must be valid",
                        )
                })
                .collect(),
            Vec::new(),
            None,
            None,
        )
        .expect("test gate must be valid")
    }

    fn circuit(
        gates: Vec<Gate>,
        qubits: usize,
    ) -> QuantumCircuit {
        let mut circuit =
            QuantumCircuit::new(qubits, 8)
                .expect(
                    "test circuit must be constructible",
                );

        for gate in gates {
            circuit
                .push(gate)
                .expect(
                    "test gate must be insertable",
                );
        }

        circuit
    }

    fn context() -> OptimizationContext {
        OptimizationContext::new(
            OptimizationConfig::default(),
            OptimizationLimits::production(),
        )
        .expect(
            "production optimization context should construct",
        )
    }

    #[test]
    fn metadata_has_stable_identifier() {
        let pass = TDepthPass::new();

        assert_eq!(
            pass.metadata().id().as_str(),
            PASS_ID
        );
    }

    #[test]
    fn empty_circuit_has_zero_t_depth() {
        let circuit =
            circuit(Vec::new(), 0);

        let pass = TDepthPass::new();
        let mut context = context();

        assert_eq!(
            pass.depth(
                &circuit,
                &mut context
            )
            .unwrap(),
            0
        );
    }

    #[test]
    fn independent_t_gates_share_one_layer() {
        let circuit = circuit(
            vec![
                gate(GateKind::T, &[0]),
                gate(GateKind::T, &[1]),
            ],
            2,
        );

        let pass = TDepthPass::new();
        let mut context = context();

        let report = pass
            .analyze(
                &circuit,
                &mut context,
            )
            .unwrap();

        assert_eq!(
            report.depth(),
            1
        );

        assert_eq!(
            report.t_count(),
            2
        );

        assert_eq!(
            report.maximum_layer_width(),
            2
        );

        assert_eq!(
            report.layers()[0]
                .operations(),
            &[0, 1]
        );
    }

    #[test]
    fn same_qubit_t_gates_have_serial_t_depth() {
        let circuit = circuit(
            vec![
                gate(GateKind::T, &[0]),
                gate(GateKind::T, &[0]),
            ],
            1,
        );

        let pass = TDepthPass::new();
        let mut context = context();

        let report = pass
            .analyze(
                &circuit,
                &mut context,
            )
            .unwrap();

        assert_eq!(
            report.depth(),
            2
        );

        assert_eq!(
            report.t_count(),
            2
        );

        assert_eq!(
            report.layers()[0]
                .operations(),
            &[0]
        );

        assert_eq!(
            report.layers()[1]
                .operations(),
            &[1]
        );
    }

    #[test]
    fn multi_qubit_gate_synchronizes_t_frontiers() {
        let circuit = circuit(
            vec![
                gate(GateKind::T, &[0]),
                gate(GateKind::CX, &[0, 1]),
                gate(GateKind::T, &[1]),
            ],
            2,
        );

        let pass = TDepthPass::new();
        let mut context = context();

        assert_eq!(
            pass.depth(
                &circuit,
                &mut context
            )
            .unwrap(),
            2
        );
    }

    #[test]
    fn non_t_operations_do_not_consume_t_layers() {
        let circuit = circuit(
            vec![
                gate(GateKind::H, &[0]),
                gate(GateKind::T, &[0]),
                gate(GateKind::X, &[0]),
                gate(GateKind::T, &[1]),
            ],
            2,
        );

        let pass = TDepthPass::new();
        let mut context = context();

        let report = pass
            .analyze(
                &circuit,
                &mut context,
            )
            .unwrap();

        assert_eq!(
            report.depth(),
            1
        );

        assert_eq!(
            report.t_count(),
            2
        );
    }

    #[test]
    fn barrier_synchronizes_frontiers() {
        let circuit = circuit(
            vec![
                gate(GateKind::T, &[0]),
                gate(
                    GateKind::Barrier,
                    &[0, 1],
                ),
                gate(GateKind::T, &[1]),
            ],
            2,
        );

        let pass = TDepthPass::new();
        let mut context = context();

        assert_eq!(
            pass.depth(
                &circuit,
                &mut context
            )
            .unwrap(),
            2
        );
    }

    #[test]
    fn reset_breaks_the_quantum_t_frontier() {
        let circuit = circuit(
            vec![
                gate(GateKind::T, &[0]),
                gate(GateKind::Reset, &[0]),
                gate(GateKind::T, &[0]),
            ],
            1,
        );

        let pass = TDepthPass::new();
        let mut context = context();

        assert_eq!(
            pass.depth(
                &circuit,
                &mut context
            )
            .unwrap(),
            1
        );
    }

    #[test]
    fn t_and_tdg_are_both_t_family_operations() {
        let circuit = circuit(
            vec![
                gate(GateKind::T, &[0]),
                gate(GateKind::Tdg, &[1]),
            ],
            2,
        );

        let pass = TDepthPass::new();
        let mut context = context();

        let report = pass
            .analyze(
                &circuit,
                &mut context,
            )
            .unwrap();

        assert_eq!(
            report.depth(),
            1
        );

        assert_eq!(
            report.t_count(),
            2
        );
    }

    #[test]
    fn t_power_pair_is_still_two_layers_before_rewriting() {
        let circuit = circuit(
            vec![
                gate(GateKind::T, &[0]),
                gate(GateKind::Tdg, &[0]),
            ],
            1,
        );

        let pass = TDepthPass::new();
        let mut context = context();

        assert_eq!(
            pass.depth(
                &circuit,
                &mut context
            )
            .unwrap(),
            2
        );
    }

    #[test]
    fn run_is_analysis_only() {
        let original = circuit(
            vec![
                gate(GateKind::T, &[0]),
                gate(GateKind::T, &[1]),
            ],
            2,
        );

        let mut circuit =
            original.clone();

        let pass = TDepthPass::new();
        let mut context = context();

        let outcome = pass
            .run(
                &mut circuit,
                &mut context,
            )
            .unwrap();

        assert!(!outcome.changed());

        assert_eq!(
            circuit.operations(),
            original.operations()
        );
    }
}