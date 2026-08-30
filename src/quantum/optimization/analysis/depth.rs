//! Zamani Quantum Optimization — Circuit Depth Analysis
//!
//! Production-grade logical-circuit depth analysis over the canonical
//! `crate::quantum::ir::QuantumCircuit`.
//!
//! # Architectural position
//!
//! ```text
//!                         quantum::ir::QuantumCircuit
//!                                    │
//!                                    ▼
//!                         optimization::analysis
//!                                    │
//!                                    ▼
//!                           analysis::depth
//!                                    │
//!              ┌─────────────────────┼─────────────────────┐
//!              ▼                     ▼                     ▼
//!        depth metrics         parallelism           critical path
//!              │                     │                     │
//!              └─────────────────────┼─────────────────────┘
//!                                    ▼
//!                              optimization
//! ```
//!
//! This module computes **logical circuit depth**. It does not schedule gates
//! on physical hardware and does not model pulse timing, calibration,
//! connectivity, routing, or backend execution.
//!
//! # Core semantic definition
//!
//! For a straight-line logical circuit, every operation is assigned the
//! earliest logical layer in which it can execute while preserving the
//! canonical circuit's per-qubit ordering.
//!
//! For an operation touching qubits `q0..qn`, its layer is:
//!
//! ```text
//! 1 + max(last_layer[q0], ..., last_layer[qn])
//! ```
//!
//! where an unseen qubit has predecessor layer zero.
//!
//! All qubits touched by the operation are then advanced to the operation's
//! layer.
//!
//! This produces the canonical **ASAP logical depth** of the ordered circuit.
//!
//! # Important distinction
//!
//! This module does NOT answer:
//!
//! - how long the circuit takes on a particular QPU;
//! - which physical qubits are available;
//! - whether two gates commute algebraically;
//! - whether routing can introduce additional operations;
//! - whether pulses overlap;
//! - whether calibration permits simultaneous execution;
//! - whether a backend has dynamic scheduling constraints.
//!
//! Those concerns belong to routing, scheduling, hardware, and execution
//! subsystems.
//!
//! # Why depth must be computed here
//!
//! Optimization needs a backend-independent depth metric in order to compare
//! transformations such as:
//!
//! ```text
//! before optimization
//!        │
//!        ▼
//! logical depth = 100
//!        │
//!        ▼
//! optimization
//!        │
//!        ▼
//! logical depth = 42
//! ```
//!
//! Hardware-specific timing can then be evaluated by downstream stages.
//!
//! # Gate semantics
//!
//! Every canonical operation occupies one logical layer for each logical qubit
//! it touches.
//!
//! Therefore:
//!
//! - single-qubit gate → one qubit advances by one layer;
//! - two-qubit gate → both qubits advance to the same layer;
//! - three-qubit gate → all three qubits advance to the same layer;
//! - measurement → the measured qubit advances by one layer;
//! - reset → the reset qubit advances by one layer;
//! - barrier → every listed qubit advances to a common synchronization layer.
//!
//! A barrier is therefore a logical synchronization boundary, not a zero-cost
//! annotation for this analysis.
//!
//! # Determinism
//!
//! The result is deterministic for a deterministic canonical circuit.
//!
//! Internally, sparse qubit state is stored in a hash map because a circuit can
//! declare a very large logical namespace while touching only a small subset of
//! qubits. Public result collections are sorted by canonical logical qubit
//! identity before exposure.
//!
//! # Scaling
//!
//! Let:
//!
//! - `N` = number of operations;
//! - `A` = total number of logical qubit operands;
//! - `K` = number of distinct logical qubits actually used.
//!
//! The analysis runs in:
//!
//! - expected `O(N + A)` time;
//! - `O(K + N)` memory when operation/layer information is requested;
//! - `O(K)` mutable working memory for the core depth calculation.
//!
//! There is no artificial circuit-size limit in this module.
//!
//! The canonical IR's resource policy remains authoritative. The implementation
//! uses checked arithmetic for all counters and layer calculations.
//!
//! Memory consumption is proportional to the qubits actually encountered,
//! rather than blindly allocating one record for every declared logical qubit.
//!
//! This is important for very large sparse logical namespaces.
//!
//! # Overflow policy
//!
//! Every arithmetic operation that can overflow uses checked arithmetic.
//!
//! A depth calculation that cannot be represented by the host `usize` type
//! returns an explicit error instead of wrapping.
//!
//! # Canonical IR ownership
//!
//! This module does NOT define:
//!
//! - `QuantumGate`;
//! - another circuit type;
//! - another qubit type;
//! - physical qubits;
//! - routing data;
//! - hardware timing;
//! - optimizer transformations.
//!
//! The authoritative representations remain:
//!
//! ```text
//! crate::quantum::ir::QuantumCircuit
//! crate::quantum::ir::gate::Gate
//! crate::quantum::ir::qubits::QubitId
//! ```
//!
//! # Integration contract
//!
//! ## `analysis/mod.rs`
//!
//! Export this module with:
//!
//! ```text
//! pub mod depth;
//! ```
//!
//! The public types may additionally be re-exported.
//!
//! ## `dependency.rs`
//!
//! Dependency analysis can use `operation_layers()` and the per-operation
//! ordering represented by the canonical circuit.
//!
//! Depth analysis deliberately does not depend on dependency analysis to avoid
//! a circular module dependency. Both analyses derive ordering from the same
//! canonical IR semantics.
//!
//! ## `critical_path.rs`
//!
//! Critical-path analysis should consume:
//!
//! - `operation_layers()`;
//! - `depth()`;
//! - `layer()`;
//! - per-qubit depth information;
//!
//! and combine these with the richer dependency graph when it is available.
//!
//! ## `width.rs`
//!
//! Width analysis can consume:
//!
//! - `used_qubits()`;
//! - `max_parallelism()`;
//! - `depth()`.
//!
//! ## `gate_counts.rs`
//!
//! Gate-count analysis remains independent. Depth analysis may classify layers
//! by operation kind, but it must not become the owner of gate-count semantics.
//!
//! ## `context.rs`
//!
//! The optimization context can cache `DepthAnalysis` as an immutable analysis
//! result.
//!
//! Any transformation that changes operation ordering, operation membership,
//! or qubit operands invalidates the result.
//!
//! A metadata-only transformation may retain it.
//!
//! ## optimization passes
//!
//! Depth-sensitive passes can consume this analysis to evaluate:
//!
//! - depth reduction;
//! - parallelism exposure;
//! - critical-path changes;
//! - two-qubit depth;
//! - measurement/reset depth;
//! - optimization regressions.
//!
//! ## `targets/`
//!
//! Hardware targets may consume logical depth as an input to a cost model, but
//! `depth.rs` must never depend on target hardware.
//!
//! ## `scheduling/`
//!
//! Hardware scheduling may use this result as a logical lower-level metric.
//! It must not be implemented here.
//!
//! # Rust compatibility
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
//! is intentionally applied to this entire module.
//!
//! # Verification
//!
//! Tests cover:
//!
//! - empty circuits;
//! - one operation;
//! - independent gates;
//! - serialized gates;
//! - multi-qubit gates;
//! - measurements;
//! - resets;
//! - barriers;
//! - sparse logical qubit namespaces;
//! - repeated operations;
//! - per-qubit depth;
//! - two-qubit depth;
//! - unitary depth;
//! - maximum parallelism;
//! - deterministic output;
//! - checked arithmetic;
//! - invalid input circuits;
//! - operation-layer lookup;
//! - layer statistics.
//!
//! ```text
//! analysis::depth
//!       │
//!       ├── canonical QuantumCircuit
//!       ├── O(N + A) time
//!       ├── O(K) core state
//!       ├── deterministic results
//!       ├── checked arithmetic
//!       └── no unsafe
//! ```

#![forbid(unsafe_code)]

use std::collections::HashMap;
use std::fmt;

use crate::quantum::ir::gate::Gate;
use crate::quantum::ir::qubits::QubitId;
use crate::quantum::ir::QuantumCircuit;

// ============================================================================
// Public scalar types
// ============================================================================

/// Zero-based operation index in the canonical circuit.
pub type OperationIndex = usize;

/// One-based logical depth layer.
///
/// Layer zero means "no operation has yet executed".
///
/// The first executable logical layer is therefore `1`.
pub type DepthLayer = usize;

// ============================================================================
// Errors
// ============================================================================

/// Errors produced by logical circuit-depth analysis.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DepthAnalysisError {
    /// The canonical quantum circuit failed validation.
    InvalidCircuit {
        /// Human-readable validation error.
        message: String,
    },

    /// A checked arithmetic operation overflowed.
    ArithmeticOverflow {
        /// Calculation that overflowed.
        calculation: &'static str,
    },

    /// An operation index does not exist in the analyzed circuit.
    OperationOutOfRange {
        /// Requested operation index.
        index: usize,

        /// Number of operations in the circuit.
        operation_count: usize,
    },

    /// A logical qubit does not belong to the declared circuit namespace.
    QubitOutOfRange {
        /// Invalid logical qubit.
        qubit: QubitId,

        /// Number of declared logical qubits.
        qubit_count: usize,
    },
}

impl fmt::Display for DepthAnalysisError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidCircuit { message } => {
                write!(
                    formatter,
                    "cannot analyze circuit depth: invalid quantum circuit: {message}"
                )
            }

            Self::ArithmeticOverflow { calculation } => {
                write!(
                    formatter,
                    "arithmetic overflow while calculating {calculation}"
                )
            }

            Self::OperationOutOfRange {
                index,
                operation_count,
            } => {
                write!(
                    formatter,
                    "operation index {index} is outside circuit length {operation_count}"
                )
            }

            Self::QubitOutOfRange {
                qubit,
                qubit_count,
            } => {
                write!(
                    formatter,
                    "logical qubit {qubit:?} is outside circuit namespace 0..{qubit_count}"
                )
            }
        }
    }
}

impl std::error::Error for DepthAnalysisError {}

// ============================================================================
// Per-qubit result
// ============================================================================

/// Depth information for one logical qubit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct QubitDepth {
    qubit: QubitId,
    depth: DepthLayer,
    operation_count: usize,
    first_operation: Option<OperationIndex>,
    last_operation: Option<OperationIndex>,
}

impl QubitDepth {
    fn new(qubit: QubitId) -> Self {
        Self {
            qubit,
            depth: 0,
            operation_count: 0,
            first_operation: None,
            last_operation: None,
        }
    }

    fn record(
        &mut self,
        operation: OperationIndex,
        layer: DepthLayer,
    ) -> Result<(), DepthAnalysisError> {
        if self.first_operation.is_none() {
            self.first_operation = Some(operation);
        }

        self.last_operation = Some(operation);

        self.operation_count = self
            .operation_count
            .checked_add(1)
            .ok_or(DepthAnalysisError::ArithmeticOverflow {
                calculation: "per-qubit operation count",
            })?;

        if layer > self.depth {
            self.depth = layer;
        }

        Ok(())
    }

    /// Returns the logical qubit.
    #[must_use]
    pub const fn qubit(self) -> QubitId {
        self.qubit
    }

    /// Returns the last logical layer occupied by this qubit.
    #[must_use]
    pub const fn depth(self) -> DepthLayer {
        self.depth
    }

    /// Returns the number of operations touching this qubit.
    #[must_use]
    pub const fn operation_count(self) -> usize {
        self.operation_count
    }

    /// Returns the first operation touching this qubit.
    #[must_use]
    pub const fn first_operation(self) -> Option<OperationIndex> {
        self.first_operation
    }

    /// Returns the last operation touching this qubit.
    #[must_use]
    pub const fn last_operation(self) -> Option<OperationIndex> {
        self.last_operation
    }

    /// Returns whether this qubit participates in the circuit.
    #[must_use]
    pub const fn is_used(self) -> bool {
        self.operation_count != 0
    }
}

// ============================================================================
// Layer information
// ============================================================================

/// Summary information about one logical depth layer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LayerInfo {
    layer: DepthLayer,
    operation_count: usize,
    qubit_operand_count: usize,
    single_qubit_operation_count: usize,
    multi_qubit_operation_count: usize,
    unitary_operation_count: usize,
    measurement_count: usize,
    reset_count: usize,
    barrier_count: usize,
}

impl LayerInfo {
    fn new(layer: DepthLayer) -> Self {
        Self {
            layer,
            operation_count: 0,
            qubit_operand_count: 0,
            single_qubit_operation_count: 0,
            multi_qubit_operation_count: 0,
            unitary_operation_count: 0,
            measurement_count: 0,
            reset_count: 0,
            barrier_count: 0,
        }
    }

    fn record(&mut self, gate: &Gate) -> Result<(), DepthAnalysisError> {
        self.operation_count = self
            .operation_count
            .checked_add(1)
            .ok_or(DepthAnalysisError::ArithmeticOverflow {
                calculation: "layer operation count",
            })?;

        self.qubit_operand_count = self
            .qubit_operand_count
            .checked_add(gate.qubits().len())
            .ok_or(DepthAnalysisError::ArithmeticOverflow {
                calculation: "layer qubit operand count",
            })?;

        if gate.qubits().len() == 1 {
            self.single_qubit_operation_count = self
                .single_qubit_operation_count
                .checked_add(1)
                .ok_or(DepthAnalysisError::ArithmeticOverflow {
                    calculation: "layer single-qubit operation count",
                })?;
        } else if gate.qubits().len() > 1 {
            self.multi_qubit_operation_count = self
                .multi_qubit_operation_count
                .checked_add(1)
                .ok_or(DepthAnalysisError::ArithmeticOverflow {
                    calculation: "layer multi-qubit operation count",
                })?;
        }

        if gate.is_unitary() {
            self.unitary_operation_count = self
                .unitary_operation_count
                .checked_add(1)
                .ok_or(DepthAnalysisError::ArithmeticOverflow {
                    calculation: "layer unitary operation count",
                })?;
        }

        if gate.is_measurement() {
            self.measurement_count = self
                .measurement_count
                .checked_add(1)
                .ok_or(DepthAnalysisError::ArithmeticOverflow {
                    calculation: "layer measurement count",
                })?;
        }

        if gate.is_reset() {
            self.reset_count = self
                .reset_count
                .checked_add(1)
                .ok_or(DepthAnalysisError::ArithmeticOverflow {
                    calculation: "layer reset count",
                })?;
        }

        if gate.is_barrier() {
            self.barrier_count = self
                .barrier_count
                .checked_add(1)
                .ok_or(DepthAnalysisError::ArithmeticOverflow {
                    calculation: "layer barrier count",
                })?;
        }

        Ok(())
    }

    /// Returns the one-based layer number.
    #[must_use]
    pub const fn layer(self) -> DepthLayer {
        self.layer
    }

    /// Returns the number of operations in this layer.
    #[must_use]
    pub const fn operation_count(self) -> usize {
        self.operation_count
    }

    /// Returns the total number of logical qubit operands in this layer.
    #[must_use]
    pub const fn qubit_operand_count(self) -> usize {
        self.qubit_operand_count
    }

    /// Returns the number of single-qubit operations.
    #[must_use]
    pub const fn single_qubit_operation_count(self) -> usize {
        self.single_qubit_operation_count
    }

    /// Returns the number of operations touching two or more qubits.
    #[must_use]
    pub const fn multi_qubit_operation_count(self) -> usize {
        self.multi_qubit_operation_count
    }

    /// Returns the number of unitary operations.
    #[must_use]
    pub const fn unitary_operation_count(self) -> usize {
        self.unitary_operation_count
    }

    /// Returns the number of measurements.
    #[must_use]
    pub const fn measurement_count(self) -> usize {
        self.measurement_count
    }

    /// Returns the number of resets.
    #[must_use]
    pub const fn reset_count(self) -> usize {
        self.reset_count
    }

    /// Returns the number of barriers.
    #[must_use]
    pub const fn barrier_count(self) -> usize {
        self.barrier_count
    }

    /// Returns whether this layer contains at least one multi-qubit operation.
    #[must_use]
    pub const fn contains_multi_qubit_operation(self) -> bool {
        self.multi_qubit_operation_count != 0
    }

    /// Returns whether this layer contains at least one measurement.
    #[must_use]
    pub const fn contains_measurement(self) -> bool {
        self.measurement_count != 0
    }

    /// Returns whether this layer contains at least one reset.
    #[must_use]
    pub const fn contains_reset(self) -> bool {
        self.reset_count != 0
    }

    /// Returns whether this layer contains a barrier.
    #[must_use]
    pub const fn contains_barrier(self) -> bool {
        self.barrier_count != 0
    }
}

// ============================================================================
// Main result
// ============================================================================

/// Immutable logical depth-analysis result.
///
/// The result is independent of the input circuit after construction. This
/// makes it suitable for caching inside `optimization::context`.
///
/// The result does not contain a reference to the circuit and therefore cannot
/// accidentally observe mutations performed after the analysis.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DepthAnalysis {
    declared_qubits: usize,
    operation_count: usize,
    depth: DepthLayer,
    unitary_depth: DepthLayer,
    multi_qubit_depth: DepthLayer,
    measurement_depth: DepthLayer,
    reset_depth: DepthLayer,
    barrier_depth: DepthLayer,
    max_parallelism: usize,
    max_multi_qubit_parallelism: usize,
    total_qubit_operands: usize,
    used_qubits: Vec<QubitDepth>,
    operation_layers: Vec<DepthLayer>,
    layers: Vec<LayerInfo>,
}

impl DepthAnalysis {
    /// Analyzes a canonical quantum circuit.
    ///
    /// The input circuit is validated before analysis. This is intentionally
    /// conservative because optimizer analyses may eventually receive IR from
    /// deserialization, generated code, external tools, or other compiler
    /// stages.
    pub fn analyze(
        circuit: &QuantumCircuit,
    ) -> Result<Self, DepthAnalysisError> {
        circuit
            .validate()
            .map_err(|error| DepthAnalysisError::InvalidCircuit {
                message: error.to_string(),
            })?;

        Self::analyze_validated(circuit)
    }

    /// Analyzes a circuit that has already been validated by the current
    /// compiler stage.
    ///
    /// This avoids repeating O(N) canonical validation when several analyses
    /// are run over the same immutable circuit.
    ///
    /// The method is safe Rust. The caller is responsible for maintaining the
    /// documented validation precondition.
    pub fn analyze_validated(
        circuit: &QuantumCircuit,
    ) -> Result<Self, DepthAnalysisError> {
        let operations = circuit.operations();

        let operation_count = operations.len();
        let declared_qubits = circuit.num_qubits();

        if operation_count == 0 {
            return Ok(Self {
                declared_qubits,
                operation_count: 0,
                depth: 0,
                unitary_depth: 0,
                multi_qubit_depth: 0,
                measurement_depth: 0,
                reset_depth: 0,
                barrier_depth: 0,
                max_parallelism: 0,
                max_multi_qubit_parallelism: 0,
                total_qubit_operands: 0,
                used_qubits: Vec::new(),
                operation_layers: Vec::new(),
                layers: Vec::new(),
            });
        }

        // Only allocate state for qubits actually encountered.
        //
        // This is intentionally sparse. A circuit may declare a very large
        // logical namespace while using only a small subset of it.
        let mut last_layer: HashMap<QubitId, DepthLayer> = HashMap::new();

        let mut qubit_depths: HashMap<QubitId, QubitDepth> = HashMap::new();

        let mut operation_layers =
            Vec::with_capacity(operation_count);

        let mut layers: Vec<LayerInfo> = Vec::new();

        let mut total_qubit_operands = 0usize;

        let mut unitary_depth = 0usize;
        let mut multi_qubit_depth = 0usize;
        let mut measurement_depth = 0usize;
        let mut reset_depth = 0usize;
        let mut barrier_depth = 0usize;

        let mut max_parallelism = 0usize;
        let mut max_multi_qubit_parallelism = 0usize;

        for (operation_index, gate) in operations.iter().enumerate() {
            let qubits = gate.qubits();

            if qubits.is_empty() {
                return Err(DepthAnalysisError::InvalidCircuit {
                    message: format!(
                        "operation {operation_index} contains no logical qubit operands"
                    ),
                });
            }

            let mut required_layer = 0usize;

            for &qubit in qubits {
                if qubit.index() >= declared_qubits {
                    return Err(
                        DepthAnalysisError::QubitOutOfRange {
                            qubit,
                            qubit_count: declared_qubits,
                        },
                    );
                }

                let predecessor_layer =
                    last_layer.get(&qubit).copied().unwrap_or(0);

                if predecessor_layer > required_layer {
                    required_layer = predecessor_layer;
                }
            }

            let layer = required_layer
                .checked_add(1)
                .ok_or(
                    DepthAnalysisError::ArithmeticOverflow {
                        calculation: "logical circuit depth layer",
                    },
                )?;

            operation_layers.push(layer);

            total_qubit_operands = total_qubit_operands
                .checked_add(qubits.len())
                .ok_or(
                    DepthAnalysisError::ArithmeticOverflow {
                        calculation: "total qubit operand count",
                    },
                )?;

            // Record per-qubit depth and advance each qubit's frontier.
            for &qubit in qubits {
                let entry = qubit_depths
                    .entry(qubit)
                    .or_insert_with(|| QubitDepth::new(qubit));

                entry.record(operation_index, layer)?;

                last_layer.insert(qubit, layer);
            }

            // Ensure the layer vector contains the current layer.
            let zero_based_layer = layer - 1;

            if zero_based_layer >= layers.len() {
                let new_len = zero_based_layer
                    .checked_add(1)
                    .ok_or(
                        DepthAnalysisError::ArithmeticOverflow {
                            calculation: "layer vector length",
                        },
                    )?;

                layers.resize_with(new_len, || {
                    LayerInfo::new(0)
                });

                // Fill the layer numbers only for newly created entries.
                //
                // The resize above creates placeholder values. Set all
                // uninitialized placeholders deterministically.
                for index in 0..layers.len() {
                    if layers[index].layer == 0 {
                        layers[index] =
                            LayerInfo::new(index + 1);
                    }
                }
            }

            layers[zero_based_layer].record(gate)?;

            // Aggregate logical depth categories.
            if gate.is_unitary() {
                unitary_depth = unitary_depth.max(layer);
            }

            if qubits.len() > 1 {
                multi_qubit_depth =
                    multi_qubit_depth.max(layer);
            }

            if gate.is_measurement() {
                measurement_depth =
                    measurement_depth.max(layer);
            }

            if gate.is_reset() {
                reset_depth = reset_depth.max(layer);
            }

            if gate.is_barrier() {
                barrier_depth = barrier_depth.max(layer);
            }
        }

        let depth = operation_layers
            .iter()
            .copied()
            .max()
            .unwrap_or(0);

        for layer in &layers {
            if layer.operation_count() > max_parallelism {
                max_parallelism =
                    layer.operation_count();
            }

            if layer.multi_qubit_operation_count()
                > max_multi_qubit_parallelism
            {
                max_multi_qubit_parallelism =
                    layer.multi_qubit_operation_count();
            }
        }

        // HashMap iteration order must never become compiler-visible behavior.
        let mut used_qubits: Vec<QubitDepth> =
            qubit_depths.into_values().collect();

        used_qubits.sort_by_key(|usage| usage.qubit());

        Ok(Self {
            declared_qubits,
            operation_count,
            depth,
            unitary_depth,
            multi_qubit_depth,
            measurement_depth,
            reset_depth,
            barrier_depth,
            max_parallelism,
            max_multi_qubit_parallelism,
            total_qubit_operands,
            used_qubits,
            operation_layers,
            layers,
        })
    }

    /// Returns the number of declared logical qubits.
    #[must_use]
    pub const fn declared_qubits(&self) -> usize {
        self.declared_qubits
    }

    /// Returns the number of circuit operations analyzed.
    #[must_use]
    pub const fn operation_count(&self) -> usize {
        self.operation_count
    }

    /// Returns total logical circuit depth.
    ///
    /// Zero means the circuit contains no operations.
    #[must_use]
    pub const fn depth(&self) -> DepthLayer {
        self.depth
    }

    /// Returns the depth considering only unitary operations.
    ///
    /// Non-unitary operations such as measurement, reset, and barrier are
    /// excluded from the category's maximum, although their dependencies can
    /// still affect the layer assigned to later unitary operations.
    #[must_use]
    pub const fn unitary_depth(&self) -> DepthLayer {
        self.unitary_depth
    }

    /// Returns the maximum layer containing an operation acting on two or more
    /// logical qubits.
    ///
    /// This is a logical layer metric, not a count of physical two-qubit
    /// execution slots.
    #[must_use]
    pub const fn multi_qubit_depth(&self) -> DepthLayer {
        self.multi_qubit_depth
    }

    /// Returns the maximum layer containing a measurement.
    #[must_use]
    pub const fn measurement_depth(&self) -> DepthLayer {
        self.measurement_depth
    }

    /// Returns the maximum layer containing a reset.
    #[must_use]
    pub const fn reset_depth(&self) -> DepthLayer {
        self.reset_depth
    }

    /// Returns the maximum layer containing a barrier.
    #[must_use]
    pub const fn barrier_depth(&self) -> DepthLayer {
        self.barrier_depth
    }

    /// Returns the maximum number of operations assigned to the same logical
    /// layer.
    #[must_use]
    pub const fn max_parallelism(&self) -> usize {
        self.max_parallelism
    }

    /// Returns the maximum number of multi-qubit operations assigned to one
    /// logical layer.
    #[must_use]
    pub const fn max_multi_qubit_parallelism(&self) -> usize {
        self.max_multi_qubit_parallelism
    }

    /// Returns the total number of logical qubit operands encountered.
    #[must_use]
    pub const fn total_qubit_operands(&self) -> usize {
        self.total_qubit_operands
    }

    /// Returns the number of distinct logical qubits actually used.
    #[must_use]
    pub fn used_qubit_count(&self) -> usize {
        self.used_qubits.len()
    }

    /// Returns immutable per-qubit depth records in ascending logical-qubit
    /// order.
    #[must_use]
    pub fn qubits(&self) -> &[QubitDepth] {
        &self.used_qubits
    }

    /// Returns the depth information for a logical qubit.
    #[must_use]
    pub fn qubit(&self, qubit: QubitId) -> Option<QubitDepth> {
        self.used_qubits
            .binary_search_by_key(&qubit, |entry| entry.qubit())
            .ok()
            .map(|index| self.used_qubits[index])
    }

    /// Returns the logical layer assigned to an operation.
    ///
    /// Operation indices are zero-based.
    pub fn operation_layer(
        &self,
        operation: OperationIndex,
    ) -> Result<DepthLayer, DepthAnalysisError> {
        self.operation_layers
            .get(operation)
            .copied()
            .ok_or(
                DepthAnalysisError::OperationOutOfRange {
                    index: operation,
                    operation_count: self.operation_count,
                },
            )
    }

    /// Returns the operation-to-layer mapping.
    ///
    /// Entry `i` corresponds to operation `i`.
    #[must_use]
    pub fn operation_layers(&self) -> &[DepthLayer] {
        &self.operation_layers
    }

    /// Returns the number of logical layers.
    #[must_use]
    pub fn layer_count(&self) -> usize {
        self.layers.len()
    }

    /// Returns immutable layer information in ascending layer order.
    #[must_use]
    pub fn layers(&self) -> &[LayerInfo] {
        &self.layers
    }

    /// Returns information about a logical layer.
    #[must_use]
    pub fn layer(&self, layer: DepthLayer) -> Option<LayerInfo> {
        if layer == 0 {
            return None;
        }

        self.layers.get(layer - 1).copied()
    }

    /// Returns whether the circuit has no operations.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.operation_count == 0
    }

    /// Returns the average logical parallelism.
    ///
    /// Returns `0.0` for an empty circuit.
    #[must_use]
    pub fn average_parallelism(&self) -> f64 {
        if self.depth == 0 {
            return 0.0;
        }

        self.operation_count as f64 / self.depth as f64
    }

    /// Returns the average number of qubit operands per operation.
    ///
    /// Returns `0.0` for an empty circuit.
    #[must_use]
    pub fn average_qubit_arity(&self) -> f64 {
        if self.operation_count == 0 {
            return 0.0;
        }

        self.total_qubit_operands as f64
            / self.operation_count as f64
    }

    /// Returns the fraction of logical layers containing at least one
    /// multi-qubit operation.
    ///
    /// Returns `0.0` for an empty circuit.
    #[must_use]
    pub fn multi_qubit_layer_fraction(&self) -> f64 {
        if self.depth == 0 {
            return 0.0;
        }

        let layers_with_multi_qubit = self
            .layers
            .iter()
            .filter(|layer| {
                layer.contains_multi_qubit_operation()
            })
            .count();

        layers_with_multi_qubit as f64
            / self.depth as f64
    }

    /// Returns the number of operations in the deepest logical layer.
    #[must_use]
    pub fn deepest_layer_operation_count(&self) -> usize {
        self.layers
            .last()
            .map(LayerInfo::operation_count)
            .unwrap_or(0)
    }

    /// Returns whether at least one operation executes in parallel with another
    /// operation.
    #[must_use]
    pub fn has_parallelism(&self) -> bool {
        self.max_parallelism > 1
    }
}

// ============================================================================
// Convenience API
// ============================================================================

/// Computes logical circuit depth.
///
/// This is the recommended one-shot API for callers that do not need the
/// complete `DepthAnalysis` result.
pub fn analyze_depth(
    circuit: &QuantumCircuit,
) -> Result<DepthAnalysis, DepthAnalysisError> {
    DepthAnalysis::analyze(circuit)
}

/// Computes logical depth for a previously validated circuit.
///
/// This is intended for compiler pipelines where canonical validation has
/// already been performed once and several analyses are being computed.
pub fn analyze_depth_validated(
    circuit: &QuantumCircuit,
) -> Result<DepthAnalysis, DepthAnalysisError> {
    DepthAnalysis::analyze_validated(circuit)
}

/// Computes only the total logical depth.
///
/// This still validates the circuit because it is a public safety boundary.
pub fn logical_depth(
    circuit: &QuantumCircuit,
) -> Result<DepthLayer, DepthAnalysisError> {
    Ok(DepthAnalysis::analyze(circuit)?.depth())
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::quantum::ir::gate::GateKind;
    use crate::quantum::ir::parameter::Parameter;
    use crate::quantum::ir::qubits::QubitId;
    use crate::quantum::ir::QuantumCircuit;

    fn q(index: usize) -> QubitId {
        QubitId::new(index)
    }

    fn single_qubit_gate(
        kind: GateKind,
        qubit: usize,
    ) -> Gate {
        Gate::new(
            kind,
            vec![q(qubit)],
            Vec::new(),
            None,
            None,
        )
        .expect("test gate must be valid")
    }

    fn two_qubit_gate(
        kind: GateKind,
        first: usize,
        second: usize,
    ) -> Gate {
        Gate::new(
            kind,
            vec![q(first), q(second)],
            Vec::new(),
            None,
            None,
        )
        .expect("test gate must be valid")
    }

    fn rotation_gate(
        kind: GateKind,
        qubit: usize,
    ) -> Gate {
        Gate::new(
            kind,
            vec![q(qubit)],
            vec![Parameter::Constant(0.5)],
            None,
            None,
        )
        .expect("test rotation gate must be valid")
    }

    fn circuit_with(
        qubits: usize,
        gates: Vec<Gate>,
    ) -> QuantumCircuit {
        let mut circuit =
            QuantumCircuit::new(qubits, 0, Default::default())
                .expect("test circuit must be constructible");

        for gate in gates {
            circuit
                .append_gate(gate)
                .expect("test gate insertion must succeed");
        }

        circuit
    }

    #[test]
    fn empty_circuit_has_zero_depth() {
        let circuit =
            circuit_with(4, Vec::new());

        let analysis =
            DepthAnalysis::analyze(&circuit)
                .expect("analysis must succeed");

        assert_eq!(analysis.depth(), 0);
        assert_eq!(analysis.unitary_depth(), 0);
        assert_eq!(analysis.operation_count(), 0);
        assert_eq!(analysis.layer_count(), 0);
        assert_eq!(analysis.max_parallelism(), 0);
    }

    #[test]
    fn_independent_single_qubit_operations_are_parallel() {
        let circuit = circuit_with(
            3,
            vec![
                single_qubit_gate(GateKind::H, 0),
                single_qubit_gate(GateKind::X, 1),
                single_qubit_gate(GateKind::Z, 2),
            ],
        );

        let analysis =
            DepthAnalysis::analyze(&circuit)
                .expect("analysis must succeed");

        assert_eq!(analysis.depth(), 1);
        assert_eq!(analysis.operation_count(), 3);
        assert_eq!(analysis.max_parallelism(), 3);

        assert_eq!(
            analysis.operation_layers(),
            &[1, 1, 1]
        );
    }

    #[test]
    fn same_qubit_operations_are_serialized() {
        let circuit = circuit_with(
            1,
            vec![
                single_qubit_gate(GateKind::H, 0),
                single_qubit_gate(GateKind::X, 0),
                single_qubit_gate(GateKind::Z, 0),
            ],
        );

        let analysis =
            DepthAnalysis::analyze(&circuit)
                .expect("analysis must succeed");

        assert_eq!(analysis.depth(), 3);
        assert_eq!(
            analysis.operation_layers(),
            &[1, 2, 3]
        );
        assert_eq!(analysis.max_parallelism(), 1);
    }

    #[test]
    fn two_qubit_gate_synchronizes_both_qubits() {
        let circuit = circuit_with(
            2,
            vec![
                single_qubit_gate(GateKind::H, 0),
                single_qubit_gate(GateKind::X, 1),
                two_qubit_gate(GateKind::CX, 0, 1),
                single_qubit_gate(GateKind::Z, 0),
            ],
        );

        let analysis =
            DepthAnalysis::analyze(&circuit)
                .expect("analysis must succeed");

        assert_eq!(
            analysis.operation_layers(),
            &[1, 1, 2, 3]
        );
        assert_eq!(analysis.depth(), 3);
        assert_eq!(
            analysis.multi_qubit_depth(),
            2
        );

        assert_eq!(
            analysis.qubit(q(0))
                .expect("q0 must exist")
                .depth(),
            3
        );

        assert_eq!(
            analysis.qubit(q(1))
                .expect("q1 must exist")
                .depth(),
            2
        );
    }

    #[test]
    fn measurement_is_a_logical_layer() {
        let measure = Gate::new(
            GateKind::Measure,
            vec![q(0)],
            Vec::new(),
            Some(0),
            None,
        )
        .expect("measurement must be valid");

        let circuit = circuit_with(
            1,
            vec![
                single_qubit_gate(GateKind::H, 0),
                measure,
            ],
        );

        let analysis =
            DepthAnalysis::analyze(&circuit)
                .expect("analysis must succeed");

        assert_eq!(
            analysis.operation_layers(),
            &[1, 2]
        );
        assert_eq!(analysis.depth(), 2);
        assert_eq!(analysis.measurement_depth(), 2);
    }

    #[test]
    fn reset_is_a_logical_layer() {
        let reset = Gate::new(
            GateKind::Reset,
            vec![q(0)],
            Vec::new(),
            None,
            None,
        )
        .expect("reset must be valid");

        let circuit = circuit_with(
            1,
            vec![
                single_qubit_gate(GateKind::X, 0),
                reset,
                single_qubit_gate(GateKind::H, 0),
            ],
        );

        let analysis =
            DepthAnalysis::analyze(&circuit)
                .expect("analysis must succeed");

        assert_eq!(
            analysis.operation_layers(),
            &[1, 2, 3]
        );
        assert_eq!(analysis.reset_depth(), 2);
        assert_eq!(analysis.depth(), 3);
    }

    #[test]
    fn barrier_synchronizes_all_its_operands() {
        let barrier = Gate::new(
            GateKind::Barrier,
            vec![q(0), q(1)],
            Vec::new(),
            None,
            None,
        )
        .expect("barrier must be valid");

        let circuit = circuit_with(
            2,
            vec![
                single_qubit_gate(GateKind::H, 0),
                single_qubit_gate(GateKind::X, 1),
                barrier,
                single_qubit_gate(GateKind::Z, 0),
                single_qubit_gate(GateKind::Y, 1),
            ],
        );

        let analysis =
            DepthAnalysis::analyze(&circuit)
                .expect("analysis must succeed");

        assert_eq!(
            analysis.operation_layers(),
            &[1, 1, 2, 3, 3]
        );
        assert_eq!(analysis.barrier_depth(), 2);
        assert_eq!(analysis.depth(), 3);
    }

    #[test]
    fn rotation_gate_is_analyzed_without_special_casing() {
        let circuit = circuit_with(
            1,
            vec![
                rotation_gate(GateKind::RZ, 0),
                rotation_gate(GateKind::RX, 0),
            ],
        );

        let analysis =
            DepthAnalysis::analyze(&circuit)
                .expect("analysis must succeed");

        assert_eq!(analysis.depth(), 2);
        assert_eq!(analysis.unitary_depth(), 2);
    }

    #[test]
    fn qubit_results_are_deterministically_sorted() {
        let circuit = circuit_with(
            4,
            vec![
                single_qubit_gate(GateKind::X, 3),
                single_qubit_gate(GateKind::X, 1),
                single_qubit_gate(GateKind::X, 2),
            ],
        );

        let analysis =
            DepthAnalysis::analyze(&circuit)
                .expect("analysis must succeed");

        let qubits: Vec<QubitId> = analysis
            .qubits()
            .iter()
            .map(|entry| entry.qubit())
            .collect();

        assert_eq!(
            qubits,
            vec![q(1), q(2), q(3)]
        );
    }

    #[test]
    fn unused_declared_qubits_are_not_materialized() {
        let circuit = circuit_with(
            1_000_000,
            vec![
                single_qubit_gate(GateKind::H, 999_999),
            ],
        );

        let analysis =
            DepthAnalysis::analyze(&circuit)
                .expect("analysis must succeed");

        assert_eq!(analysis.declared_qubits(), 1_000_000);
        assert_eq!(analysis.used_qubit_count(), 1);
        assert_eq!(
            analysis.qubits()[0].qubit(),
            q(999_999)
        );
    }

    #[test]
    fn operation_layer_lookup_is_checked() {
        let circuit = circuit_with(
            1,
            vec![
                single_qubit_gate(GateKind::H, 0),
            ],
        );

        let analysis =
            DepthAnalysis::analyze(&circuit)
                .expect("analysis must succeed");

        assert_eq!(
            analysis
                .operation_layer(0)
                .expect("operation must exist"),
            1
        );

        assert!(matches!(
            analysis.operation_layer(1),
            Err(
                DepthAnalysisError::OperationOutOfRange {
                    index: 1,
                    operation_count: 1
                }
            )
        ));
    }

    #[test]
    fn layer_statistics_are_consistent() {
        let circuit = circuit_with(
            3,
            vec![
                single_qubit_gate(GateKind::H, 0),
                single_qubit_gate(GateKind::X, 1),
                two_qubit_gate(GateKind::CX, 1, 2),
            ],
        );

        let analysis =
            DepthAnalysis::analyze(&circuit)
                .expect("analysis must succeed");

        assert_eq!(analysis.depth(), 2);

        let layer_one =
            analysis.layer(1).expect("layer 1 exists");

        assert_eq!(
            layer_one.operation_count(),
            2
        );
        assert_eq!(
            layer_one.single_qubit_operation_count(),
            2
        );
        assert_eq!(
            layer_one.multi_qubit_operation_count(),
            0
        );

        let layer_two =
            analysis.layer(2).expect("layer 2 exists");

        assert_eq!(
            layer_two.operation_count(),
            1
        );
        assert_eq!(
            layer_two.multi_qubit_operation_count(),
            1
        );
        assert!(layer_two.contains_multi_qubit_operation());
    }

    #[test]
    fn average_parallelism_is_well_defined() {
        let circuit = circuit_with(
            2,
            vec![
                single_qubit_gate(GateKind::H, 0),
                single_qubit_gate(GateKind::X, 1),
            ],
        );

        let analysis =
            DepthAnalysis::analyze(&circuit)
                .expect("analysis must succeed");

        assert_eq!(
            analysis.average_parallelism(),
            2.0
        );
    }

    #[test]
    fn average_qubit_arity_is_correct() {
        let circuit = circuit_with(
            2,
            vec![
                single_qubit_gate(GateKind::H, 0),
                two_qubit_gate(GateKind::CX, 0, 1),
            ],
        );

        let analysis =
            DepthAnalysis::analyze(&circuit)
                .expect("analysis must succeed");

        assert_eq!(
            analysis.average_qubit_arity(),
            1.5
        );
    }

    #[test]
    fn multi_qubit_layer_fraction_is_correct() {
        let circuit = circuit_with(
            2,
            vec![
                single_qubit_gate(GateKind::H, 0),
                single_qubit_gate(GateKind::X, 1),
                two_qubit_gate(GateKind::CX, 0, 1),
                single_qubit_gate(GateKind::Z, 0),
            ],
        );

        let analysis =
            DepthAnalysis::analyze(&circuit)
                .expect("analysis must succeed");

        // Layers:
        // 1: H/X
        // 2: CX
        // 3: Z
        assert_eq!(
            analysis.multi_qubit_layer_fraction(),
            1.0 / 3.0
        );
    }

    #[test]
    fn depth_is_idempotent_when_reanalyzed() {
        let circuit = circuit_with(
            2,
            vec![
                single_qubit_gate(GateKind::H, 0),
                single_qubit_gate(GateKind::X, 1),
                two_qubit_gate(GateKind::CX, 0, 1),
            ],
        );

        let first =
            DepthAnalysis::analyze(&circuit)
                .expect("analysis must succeed");

        let second =
            DepthAnalysis::analyze(&circuit)
                .expect("analysis must succeed");

        assert_eq!(first, second);
    }
}