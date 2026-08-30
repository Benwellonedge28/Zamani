//! Zamani Quantum Optimization — Fault-Tolerant Logical Cost
//!
//! Production-grade logical resource accounting for quantum circuits.
//!
//! # Architectural role
//!
//! `logical_cost.rs` answers:
//!
//! > "What does this logical quantum circuit cost in abstract logical
//! > resources?"
//!
//! It deliberately does NOT answer:
//!
//! - how the circuit is routed;
//! - how gates are scheduled in wall-clock time;
//! - how a physical QPU executes the circuit;
//! - how many physical qubits a particular code requires;
//! - how magic-state factories are physically laid out;
//! - how calibration changes execution cost;
//! - which optimization pass should be executed.
//!
//! Those concerns belong to other Zamani quantum subsystems.
//!
//! The intended architecture is:
//!
//! ```text
//!                 quantum::ir::QuantumCircuit
//!                              │
//!                              ▼
//!                    logical_cost::evaluate
//!                              │
//!          ┌───────────────────┼───────────────────┐
//!          ▼                   ▼                   ▼
//!      gate counts        logical depth       FT resources
//!          │                   │                   │
//!          └───────────────────┼───────────────────┘
//!                              ▼
//!                    LogicalCostReport
//!                              │
//!                 ┌────────────┼────────────┐
//!                 ▼            ▼            ▼
//!             optimizer    benchmarking   reporting
//! ```
//!
//! # Why this file exists
//!
//! Ordinary gate count is insufficient for fault-tolerant quantum computing.
//!
//! For example:
//!
//! - a Clifford gate may be inexpensive compared with a non-Clifford gate;
//! - T gates may consume magic-state resources;
//! - T-depth can matter independently of T-count;
//! - two-qubit logical gates can be significantly more expensive than
//!   single-qubit logical gates;
//! - logical circuit depth and total gate count measure different resources;
//! - ancilla requirements are distinct from logical-qubit count.
//!
//! Therefore this module provides a configurable, deterministic logical cost
//! model rather than a single hard-coded "number of gates" metric.
//!
//! # Fault-tolerant boundary
//!
//! This module provides *logical* fault-tolerant accounting.
//!
//! It does not model a specific code such as:
//!
//! - surface code;
//! - color code;
//! - Bacon-Shor;
//! - repetition code;
//! - LDPC code.
//!
//! Code-specific physical resource estimation belongs to the error-correction
//! and hardware layers.
//!
//! The cost model nevertheless exposes the quantities those layers need:
//!
//! - Clifford count;
//! - non-Clifford count;
//! - T count;
//! - T-dagger count;
//! - T depth;
//! - two-qubit count;
//! - logical depth;
//! - logical qubit width;
//! - ancilla accounting;
//! - weighted logical cost.
//!
//! # Integration contract
//!
//! ## Canonical Quantum IR
//!
//! The canonical representation is:
//!
//! `crate::quantum::ir::Gate`
//!
//! This file never introduces another `QuantumGate` representation.
//!
//! ## Optimization pipeline
//!
//! `pipeline.rs` may use:
//!
//! `LogicalCostEvaluator::evaluate_gates`
//!
//! before and after an optimization pass.
//!
//! The pipeline can compare reports without this file knowing anything about
//! pass scheduling.
//!
//! ## T-gate optimization
//!
//! `fault_tolerant::t_count` and `fault_tolerant::t_depth` can consume this
//! module's exact T-resource counters.
//!
//! They should not duplicate the counting rules.
//!
//! ## Cost model
//!
//! Future `cost.rs` may convert `LogicalCostReport` into a general
//! `OptimizationCostSnapshot`.
//!
//! This file intentionally does not depend on `cost.rs`, preventing a
//! dependency cycle.
//!
//! ## Verification
//!
//! Verification may compare the logical-cost reports of an original and
//! optimized circuit. This module does not perform semantic equivalence
//! checking itself.
//!
//! ## Benchmarking
//!
//! Benchmarking may consume `LogicalCostReport` as an observational metric.
//!
//! Optimization must not depend on benchmarking.
//!
//! ## Routing and hardware
//!
//! Routing/hardware may use the logical report as an input to physical
//! estimation, but this file must never import routing or hardware modules.
//!
//! # Determinism
//!
//! Evaluation is deterministic:
//!
//! - no randomness;
//! - no wall-clock dependence;
//! - no global state;
//! - no floating-point arithmetic;
//! - no hash-map iteration affecting results.
//!
//! # Scaling
//!
//! Counters use `u128` and checked arithmetic.
//!
//! The implementation is linear in the number of gates for ordinary resource
//! accounting and linear in the number of logical qubits touched for depth
//! analysis.
//!
//! It does not impose an artificial maximum circuit size. Actual resource
//! availability and the canonical Quantum IR limits remain authoritative.
//!
//! # Rust compatibility
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021
//! - stable Rust only
//! - no nightly features
//! - no `unsafe`
//!
//! # Safety
//!
//! This file contains no unsafe code.
//!
//! ```text
//! #![forbid(unsafe_code)]
//! ```
//!
//! is intentionally used so future edits cannot accidentally introduce
//! unsafe operations.

#![forbid(unsafe_code)]

use std::fmt;

use crate::quantum::ir::{Gate, GateKind, QubitId};

// =============================================================================
// Public result type aliases
// =============================================================================

/// Result type for logical-cost operations.
pub type LogicalCostResult<T> = Result<T, LogicalCostError>;

// =============================================================================
// Errors
// =============================================================================

/// Errors produced while evaluating logical quantum resources.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LogicalCostError {
    /// An arithmetic operation overflowed the supported `u128` accounting
    /// domain.
    ArithmeticOverflow {
        /// Name of the calculation that overflowed.
        calculation: &'static str,
    },

    /// A required qubit identifier could not be represented by the evaluator.
    InvalidQubit {
        /// Operation index containing the invalid qubit.
        operation: usize,

        /// The offending qubit.
        qubit: String,
    },

    /// The evaluator received an invalid gate representation.
    InvalidGate {
        /// Operation index.
        operation: usize,

        /// Gate kind.
        gate: GateKind,
    },

    /// A configured cost weight is invalid.
    InvalidWeight {
        /// Name of the weight.
        field: &'static str,

        /// Invalid value.
        value: u128,
    },

    /// The configured evaluator would exceed its explicit resource budget.
    ResourceLimitExceeded {
        /// Resource name.
        resource: &'static str,

        /// Requested amount.
        requested: u128,

        /// Maximum allowed amount.
        maximum: u128,
    },
}

impl fmt::Display for LogicalCostError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ArithmeticOverflow { calculation } => {
                write!(
                    formatter,
                    "logical cost arithmetic overflow while calculating {calculation}"
                )
            }

            Self::InvalidQubit { operation, qubit } => {
                write!(
                    formatter,
                    "invalid qubit `{qubit}` at logical operation {operation}"
                )
            }

            Self::InvalidGate { operation, gate } => {
                write!(
                    formatter,
                    "invalid gate {gate:?} at logical operation {operation}"
                )
            }

            Self::InvalidWeight { field, value } => {
                write!(
                    formatter,
                    "invalid logical-cost weight `{field}`: {value}"
                )
            }

            Self::ResourceLimitExceeded {
                resource,
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "logical-cost resource limit exceeded for {resource}: \
                     requested {requested}, maximum {maximum}"
                )
            }
        }
    }
}

impl std::error::Error for LogicalCostError {}

// =============================================================================
// Cost weights
// =============================================================================

/// Exact integer weights used by [`LogicalCostModel`].
///
/// The values have no physical unit. They are abstract logical cost units.
///
/// Keeping them as integers provides:
///
/// - deterministic results;
/// - no floating-point rounding;
/// - exact comparisons;
/// - reproducible compiler output;
/// - safe serialization later.
///
/// A caller can configure these weights for different logical architectures
/// without changing this file.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LogicalCostWeights {
    /// Cost of a logical identity operation.
    pub identity: u128,

    /// Cost of a single-qubit Clifford operation.
    pub single_qubit_clifford: u128,

    /// Cost of a single-qubit non-Clifford operation.
    pub single_qubit_non_clifford: u128,

    /// Cost of a two-qubit Clifford operation.
    pub two_qubit_clifford: u128,

    /// Cost of a two-qubit non-Clifford operation.
    pub two_qubit_non_clifford: u128,

    /// Cost of a three-or-more-qubit Clifford operation.
    pub multi_qubit_clifford: u128,

    /// Cost of a three-or-more-qubit non-Clifford operation.
    pub multi_qubit_non_clifford: u128,

    /// Cost of one T gate.
    pub t: u128,

    /// Cost of one T-dagger gate.
    pub t_dagger: u128,

    /// Cost of measurement.
    pub measurement: u128,

    /// Cost of reset.
    pub reset: u128,

    /// Cost of a barrier.
    pub barrier: u128,

    /// Cost assigned to parameterized rotations when they are not classified
    /// as Clifford by the canonical gate kind.
    pub parameterized_single_qubit: u128,

    /// Cost assigned to parameterized two-qubit operations.
    pub parameterized_two_qubit: u128,

    /// Cost assigned to parameterized multi-qubit operations.
    pub parameterized_multi_qubit: u128,
}

impl Default for LogicalCostWeights {
    fn default() -> Self {
        Self {
            identity: 0,
            single_qubit_clifford: 1,
            single_qubit_non_clifford: 4,
            two_qubit_clifford: 10,
            two_qubit_non_clifford: 20,
            multi_qubit_clifford: 20,
            multi_qubit_non_clifford: 40,
            t: 4,
            t_dagger: 4,
            measurement: 1,
            reset: 1,
            barrier: 0,
            parameterized_single_qubit: 4,
            parameterized_two_qubit: 20,
            parameterized_multi_qubit: 40,
        }
    }
}

impl LogicalCostWeights {
    /// Returns the default logical-resource weights.
    #[must_use]
    pub const fn standard() -> Self {
        Self {
            identity: 0,
            single_qubit_clifford: 1,
            single_qubit_non_clifford: 4,
            two_qubit_clifford: 10,
            two_qubit_non_clifford: 20,
            multi_qubit_clifford: 20,
            multi_qubit_non_clifford: 40,
            t: 4,
            t_dagger: 4,
            measurement: 1,
            reset: 1,
            barrier: 0,
            parameterized_single_qubit: 4,
            parameterized_two_qubit: 20,
            parameterized_multi_qubit: 40,
        }
    }

    /// Validates the configured weights.
    ///
    /// Zero is valid for any weight because some users intentionally want to
    /// ignore a resource category.
    pub const fn validate(&self) -> LogicalCostResult<()> {
        // `u128` weights have no invalid numerical values by themselves.
        //
        // This method exists as a stable validation boundary so future cost
        // models can add constraints without changing the evaluator API.
        Ok(())
    }
}

// =============================================================================
// Evaluation limits
// =============================================================================

/// Optional evaluator-side resource limits.
///
/// These limits are separate from Quantum IR limits.
///
/// IR limits protect the representation itself.
///
/// These limits protect an individual logical-cost calculation from consuming
/// excessive evaluator memory.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LogicalCostLimits {
    /// Maximum number of qubit-depth slots the evaluator may allocate.
    ///
    /// `None` means no evaluator-specific limit.
    pub max_depth_slots: Option<u128>,

    /// Maximum number of logical qubits tracked by the evaluator.
    ///
    /// `None` means no evaluator-specific limit.
    pub max_tracked_qubits: Option<u128>,
}

impl Default for LogicalCostLimits {
    fn default() -> Self {
        Self {
            max_depth_slots: None,
            max_tracked_qubits: None,
        }
    }
}

impl LogicalCostLimits {
    /// Creates an unlimited evaluator policy.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self {
            max_depth_slots: None,
            max_tracked_qubits: None,
        }
    }
}

// =============================================================================
// Resource counters
// =============================================================================

/// Exact logical operation counters.
///
/// This structure is deliberately broader than ordinary gate count because
/// fault-tolerant optimization needs multiple independent resource axes.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct LogicalGateCounts {
    /// Total number of logical operations.
    pub operations: u128,

    /// Identity operations.
    pub identity: u128,

    /// Single-qubit operations.
    pub single_qubit: u128,

    /// Two-qubit operations.
    pub two_qubit: u128,

    /// Three-or-more-qubit operations.
    pub multi_qubit: u128,

    /// Clifford operations.
    pub clifford: u128,

    /// Non-Clifford operations.
    pub non_clifford: u128,

    /// Parameterized operations.
    pub parameterized: u128,

    /// T gates.
    pub t: u128,

    /// T-dagger gates.
    pub t_dagger: u128,

    /// Measurements.
    pub measurements: u128,

    /// Resets.
    pub resets: u128,

    /// Barriers.
    pub barriers: u128,

    /// Two-qubit Clifford operations.
    pub two_qubit_clifford: u128,

    /// Two-qubit non-Clifford operations.
    pub two_qubit_non_clifford: u128,
}

impl LogicalGateCounts {
    /// Returns the total number of T-family operations.
    #[must_use]
    pub const fn t_count(&self) -> u128 {
        self.t.saturating_add(self.t_dagger)
    }

    /// Returns the number of ordinary computational quantum gates excluding
    /// measurement, reset, and barrier operations.
    #[must_use]
    pub const fn quantum_gate_count(&self) -> u128 {
        self.operations
            .saturating_sub(self.measurements)
            .saturating_sub(self.resets)
            .saturating_sub(self.barriers)
    }

    /// Returns true when this counter set represents no operations.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.operations == 0
    }
}

// =============================================================================
// Logical resource vector
// =============================================================================

/// Exact logical resource vector.
///
/// This is the principal representation for comparing an original circuit
/// against an optimized circuit.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct LogicalResourceVector {
    /// Number of logical qubits touched by the circuit.
    pub logical_qubits: u128,

    /// Logical operation count.
    pub operations: u128,

    /// Single-qubit operation count.
    pub single_qubit_operations: u128,

    /// Two-qubit operation count.
    pub two_qubit_operations: u128,

    /// Multi-qubit operation count.
    pub multi_qubit_operations: u128,

    /// Clifford operation count.
    pub clifford_operations: u128,

    /// Non-Clifford operation count.
    pub non_clifford_operations: u128,

    /// T count.
    pub t_count: u128,

    /// T depth.
    pub t_depth: u128,

    /// Logical circuit depth.
    pub logical_depth: u128,

    /// Measurement count.
    pub measurements: u128,

    /// Reset count.
    pub resets: u128,

    /// Ancilla count known to the logical-cost evaluator.
    ///
    /// Ordinary canonical gates do not necessarily carry enough information to
    /// distinguish user data qubits from ancillas. Therefore this value is
    /// zero unless the caller supplies explicit ancilla information.
    pub ancilla_count: u128,

    /// Peak simultaneously tracked logical qubits.
    pub peak_logical_width: u128,

    /// Abstract weighted logical cost.
    pub weighted_cost: u128,
}

impl LogicalResourceVector {
    /// Returns the total non-Clifford resource count represented here.
    #[must_use]
    pub const fn non_clifford_resource_count(&self) -> u128 {
        self.non_clifford_operations
    }

    /// Returns a lexicographic tuple suitable for deterministic comparisons.
    ///
    /// Lower is better.
    ///
    /// The ordering prioritizes:
    ///
    /// 1. T count;
    /// 2. two-qubit operations;
    /// 3. logical depth;
    /// 4. total operations;
    /// 5. logical width.
    #[must_use]
    pub const fn comparison_key(
        &self,
    ) -> (u128, u128, u128, u128, u128) {
        (
            self.t_count,
            self.two_qubit_operations,
            self.logical_depth,
            self.operations,
            self.peak_logical_width,
        )
    }
}

// =============================================================================
// Cost delta
// =============================================================================

/// Difference between two logical cost evaluations.
///
/// The delta is signed because an optimization may add one resource while
/// reducing another.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct LogicalCostDelta {
    /// Change in operation count.
    pub operations: i128,

    /// Change in two-qubit operations.
    pub two_qubit_operations: i128,

    /// Change in T count.
    pub t_count: i128,

    /// Change in T depth.
    pub t_depth: i128,

    /// Change in logical depth.
    pub logical_depth: i128,

    /// Change in logical width.
    pub logical_width: i128,

    /// Change in weighted cost.
    pub weighted_cost: i128,
}

impl LogicalCostDelta {
    /// Calculates the signed delta `after - before`.
    pub fn between(
        before: &LogicalResourceVector,
        after: &LogicalResourceVector,
    ) -> LogicalCostResult<Self> {
        Ok(Self {
            operations: signed_delta(
                before.operations,
                after.operations,
            )?,
            two_qubit_operations: signed_delta(
                before.two_qubit_operations,
                after.two_qubit_operations,
            )?,
            t_count: signed_delta(
                before.t_count,
                after.t_count,
            )?,
            t_depth: signed_delta(
                before.t_depth,
                after.t_depth,
            )?,
            logical_depth: signed_delta(
                before.logical_depth,
                after.logical_depth,
            )?,
            logical_width: signed_delta(
                before.peak_logical_width,
                after.peak_logical_width,
            )?,
            weighted_cost: signed_delta(
                before.weighted_cost,
                after.weighted_cost,
            )?,
        })
    }

    /// Returns true when the total weighted cost decreased.
    #[must_use]
    pub const fn improved_weighted_cost(&self) -> bool {
        self.weighted_cost < 0
    }

    /// Returns true when T count decreased.
    #[must_use]
    pub const fn reduced_t_count(&self) -> bool {
        self.t_count < 0
    }

    /// Returns true when two-qubit count decreased.
    #[must_use]
    pub const fn reduced_two_qubit_count(&self) -> bool {
        self.two_qubit_operations < 0
    }

    /// Returns true when logical depth decreased.
    #[must_use]
    pub const fn reduced_depth(&self) -> bool {
        self.logical_depth < 0
    }
}

// =============================================================================
// Full report
// =============================================================================

/// Complete deterministic logical-cost report.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogicalCostReport {
    /// Operation-level counts.
    pub counts: LogicalGateCounts,

    /// Resource vector.
    pub resources: LogicalResourceVector,

    /// Configured weights.
    pub weights: LogicalCostWeights,

    /// Whether depth analysis was performed.
    pub depth_analyzed: bool,

    /// Number of qubits explicitly tracked for depth.
    pub tracked_qubits: u128,
}

impl LogicalCostReport {
    /// Returns the T count.
    #[must_use]
    pub const fn t_count(&self) -> u128 {
        self.counts.t_count()
    }

    /// Returns the logical depth.
    #[must_use]
    pub const fn logical_depth(&self) -> u128 {
        self.resources.logical_depth
    }

    /// Returns the T depth.
    #[must_use]
    pub const fn t_depth(&self) -> u128 {
        self.resources.t_depth
    }

    /// Returns the two-qubit count.
    #[must_use]
    pub const fn two_qubit_count(&self) -> u128 {
        self.counts.two_qubit
    }

    /// Returns the weighted logical cost.
    #[must_use]
    pub const fn weighted_cost(&self) -> u128 {
        self.resources.weighted_cost
    }

    /// Returns a compact comparison key.
    #[must_use]
    pub const fn comparison_key(
        &self,
    ) -> (u128, u128, u128, u128, u128) {
        self.resources.comparison_key()
    }
}

// =============================================================================
// Evaluator
// =============================================================================

/// Production logical-cost evaluator.
///
/// The evaluator is immutable after construction and therefore can safely be
/// reused for multiple circuits.
///
/// No global state is maintained.
#[derive(Debug, Clone)]
pub struct LogicalCostEvaluator {
    weights: LogicalCostWeights,
    limits: LogicalCostLimits,
    analyze_depth: bool,
}

impl Default for LogicalCostEvaluator {
    fn default() -> Self {
        Self {
            weights: LogicalCostWeights::standard(),
            limits: LogicalCostLimits::unlimited(),
            analyze_depth: true,
        }
    }
}

impl LogicalCostEvaluator {
    /// Creates a standard logical-cost evaluator.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            weights: LogicalCostWeights::standard(),
            limits: LogicalCostLimits::unlimited(),
            analyze_depth: true,
        }
    }

    /// Creates an evaluator with custom weights.
    pub fn with_weights(
        weights: LogicalCostWeights,
    ) -> LogicalCostResult<Self> {
        weights.validate()?;

        Ok(Self {
            weights,
            limits: LogicalCostLimits::unlimited(),
            analyze_depth: true,
        })
    }

    /// Returns an evaluator configured with custom weights and limits.
    pub fn with_configuration(
        weights: LogicalCostWeights,
        limits: LogicalCostLimits,
        analyze_depth: bool,
    ) -> LogicalCostResult<Self> {
        weights.validate()?;

        Ok(Self {
            weights,
            limits,
            analyze_depth,
        })
    }

    /// Returns the evaluator's weights.
    #[must_use]
    pub const fn weights(&self) -> &LogicalCostWeights {
        &self.weights
    }

    /// Returns the evaluator's limits.
    #[must_use]
    pub const fn limits(&self) -> &LogicalCostLimits {
        &self.limits
    }

    /// Returns whether depth analysis is enabled.
    #[must_use]
    pub const fn analyzes_depth(&self) -> bool {
        self.analyze_depth
    }

    /// Enables or disables depth analysis.
    ///
    /// Disabling depth analysis can significantly reduce memory consumption
    /// for enormous circuits when only aggregate counts are required.
    #[must_use]
    pub const fn with_depth_analysis(
        mut self,
        enabled: bool,
    ) -> Self {
        self.analyze_depth = enabled;
        self
    }

    /// Replaces the evaluator limits.
    #[must_use]
    pub const fn with_limits(
        mut self,
        limits: LogicalCostLimits,
    ) -> Self {
        self.limits = limits;
        self
    }

    /// Evaluates a slice of canonical Quantum IR gates.
    ///
    /// This is the primary low-level integration API for the optimizer.
    ///
    /// The input slice is never modified.
    pub fn evaluate_gates(
        &self,
        gates: &[Gate],
    ) -> LogicalCostResult<LogicalCostReport> {
        let mut counts = LogicalGateCounts::default();

        let mut weighted_cost = 0u128;

        let mut maximum_qubit_index = None::<u128>;

        for (operation_index, gate) in gates.iter().enumerate() {
            self.validate_gate_operands(
                operation_index,
                gate,
            )?;

            let qubit_count =
                gate.qubits().len();

            self.increment_operation_count(
                &mut counts,
                gate.kind(),
            )?;

            if let Some(maximum) =
                gate.qubits()
                    .iter()
                    .map(|qubit| qubit_index(*qubit))
                    .max()
            {
                maximum_qubit_index = Some(
                    maximum_qubit_index
                        .map_or(maximum, |current| current.max(maximum)),
                );
            }

            let gate_cost =
                self.gate_cost(gate)?;

            weighted_cost = weighted_cost
                .checked_add(gate_cost)
                .ok_or(
                    LogicalCostError::ArithmeticOverflow {
                        calculation: "weighted logical cost",
                    },
                )?;

            let _ = qubit_count;
        }

        let tracked_qubits = maximum_qubit_index
            .map_or(0, |value| value.saturating_add(1));

        self.check_qubit_limit(tracked_qubits)?;

        let (
            logical_depth,
            t_depth,
            peak_width,
        ) = if self.analyze_depth {
            self.analyze_depths(gates)?
        } else {
            (0, 0, tracked_qubits)
        };

        let resources = LogicalResourceVector {
            logical_qubits: tracked_qubits,
            operations: counts.operations,
            single_qubit_operations: counts.single_qubit,
            two_qubit_operations: counts.two_qubit,
            multi_qubit_operations: counts.multi_qubit,
            clifford_operations: counts.clifford,
            non_clifford_operations: counts.non_clifford,
            t_count: counts.t_count(),
            t_depth,
            logical_depth,
            measurements: counts.measurements,
            resets: counts.resets,
            ancilla_count: 0,
            peak_logical_width: peak_width,
            weighted_cost,
        };

        Ok(LogicalCostReport {
            counts,
            resources,
            weights: self.weights,
            depth_analyzed: self.analyze_depth,
            tracked_qubits,
        })
    }

    /// Evaluates a canonical Quantum IR circuit.
    ///
    /// This method is intentionally implemented through the canonical circuit
    /// API rather than creating an optimization-specific circuit type.
    pub fn evaluate_circuit(
        &self,
        circuit: &crate::quantum::ir::QuantumCircuit,
    ) -> LogicalCostResult<LogicalCostReport> {
        let operations = circuit.operations();

        self.evaluate_gates(operations)
    }

    /// Calculates the delta between two circuit cost reports.
    pub fn delta(
        &self,
        before: &LogicalCostReport,
        after: &LogicalCostReport,
    ) -> LogicalCostResult<LogicalCostDelta> {
        LogicalCostDelta::between(
            &before.resources,
            &after.resources,
        )
    }

    // -------------------------------------------------------------------------
    // Gate classification
    // -------------------------------------------------------------------------

    fn increment_operation_count(
        &self,
        counts: &mut LogicalGateCounts,
        kind: GateKind,
    ) -> LogicalCostResult<()> {
        counts.operations =
            checked_increment(
                counts.operations,
                "operation count",
            )?;

        if kind.is_measurement() {
            counts.measurements =
                checked_increment(
                    counts.measurements,
                    "measurement count",
                )?;

            return Ok(());
        }

        if kind.is_reset() {
            counts.resets =
                checked_increment(
                    counts.resets,
                    "reset count",
                )?;

            return Ok(());
        }

        if kind.is_barrier() {
            counts.barriers =
                checked_increment(
                    counts.barriers,
                    "barrier count",
                )?;

            return Ok(());
        }

        let operand_count = kind.operand_count();

        match operand_count {
            crate::quantum::ir::gate::OperandCount::Exact(1) => {
                counts.single_qubit =
                    checked_increment(
                        counts.single_qubit,
                        "single-qubit count",
                    )?;
            }

            crate::quantum::ir::gate::OperandCount::Exact(2) => {
                counts.two_qubit =
                    checked_increment(
                        counts.two_qubit,
                        "two-qubit count",
                    )?;
            }

            crate::quantum::ir::gate::OperandCount::Exact(_) |
            crate::quantum::ir::gate::OperandCount::AtLeast(_) => {
                counts.multi_qubit =
                    checked_increment(
                        counts.multi_qubit,
                        "multi-qubit count",
                    )?;
            }
        }

        if kind.is_parameterized() {
            counts.parameterized =
                checked_increment(
                    counts.parameterized,
                    "parameterized count",
                )?;
        }

        if kind.is_clifford() {
            counts.clifford =
                checked_increment(
                    counts.clifford,
                    "Clifford count",
                )?;
        } else {
            counts.non_clifford =
                checked_increment(
                    counts.non_clifford,
                    "non-Clifford count",
                )?;
        }

        match kind {
            GateKind::I => {
                counts.identity =
                    checked_increment(
                        counts.identity,
                        "identity count",
                    )?;
            }

            GateKind::T => {
                counts.t =
                    checked_increment(
                        counts.t,
                        "T count",
                    )?;
            }

            GateKind::Tdg => {
                counts.t_dagger =
                    checked_increment(
                        counts.t_dagger,
                        "T-dagger count",
                    )?;
            }

            _ => {}
        }

        if matches!(
            kind.operand_count(),
            crate::quantum::ir::gate::OperandCount::Exact(2)
        ) {
            if kind.is_clifford() {
                counts.two_qubit_clifford =
                    checked_increment(
                        counts.two_qubit_clifford,
                        "two-qubit Clifford count",
                    )?;
            } else {
                counts.two_qubit_non_clifford =
                    checked_increment(
                        counts.two_qubit_non_clifford,
                        "two-qubit non-Clifford count",
                    )?;
            }
        }

        Ok(())
    }

    fn gate_cost(
        &self,
        gate: &Gate,
    ) -> LogicalCostResult<u128> {
        let kind = gate.kind();

        let cost = match kind {
            GateKind::I => self.weights.identity,

            GateKind::T => self.weights.t,

            GateKind::Tdg => self.weights.t_dagger,

            GateKind::Measure => self.weights.measurement,

            GateKind::Reset => self.weights.reset,

            GateKind::Barrier => self.weights.barrier,

            _ if kind.is_parameterized() => {
                match kind.operand_count() {
                    crate::quantum::ir::gate::OperandCount::Exact(1) => {
                        self.weights.parameterized_single_qubit
                    }

                    crate::quantum::ir::gate::OperandCount::Exact(2) => {
                        self.weights.parameterized_two_qubit
                    }

                    crate::quantum::ir::gate::OperandCount::Exact(_) |
                    crate::quantum::ir::gate::OperandCount::AtLeast(_) => {
                        self.weights.parameterized_multi_qubit
                    }
                }
            }

            _ => {
                match kind.operand_count() {
                    crate::quantum::ir::gate::OperandCount::Exact(1) => {
                        if kind.is_clifford() {
                            self.weights.single_qubit_clifford
                        } else {
                            self.weights.single_qubit_non_clifford
                        }
                    }

                    crate::quantum::ir::gate::OperandCount::Exact(2) => {
                        if kind.is_clifford() {
                            self.weights.two_qubit_clifford
                        } else {
                            self.weights.two_qubit_non_clifford
                        }
                    }

                    crate::quantum::ir::gate::OperandCount::Exact(_) |
                    crate::quantum::ir::gate::OperandCount::AtLeast(_) => {
                        if kind.is_clifford() {
                            self.weights.multi_qubit_clifford
                        } else {
                            self.weights.multi_qubit_non_clifford
                        }
                    }
                }
            }
        };

        Ok(cost)
    }

    // -------------------------------------------------------------------------
    // Validation
    // -------------------------------------------------------------------------

    fn validate_gate_operands(
        &self,
        operation_index: usize,
        gate: &Gate,
    ) -> LogicalCostResult<()> {
        let expected =
            gate.kind().operand_count();

        let actual =
            gate.qubits().len();

        if !expected.accepts(actual) {
            return Err(
                LogicalCostError::InvalidGate {
                    operation: operation_index,
                    gate: gate.kind(),
                },
            );
        }

        for qubit in gate.qubits() {
            let index = qubit_index(*qubit);

            // The conversion is infallible for the current QubitId API.
            // Keeping the validation in one place makes the boundary explicit
            // if the identifier representation evolves later.
            if index > usize::MAX as u128 {
                return Err(
                    LogicalCostError::InvalidQubit {
                        operation: operation_index,
                        qubit: format!("{qubit:?}"),
                    },
                );
            }
        }

        Ok(())
    }

    fn check_qubit_limit(
        &self,
        tracked_qubits: u128,
    ) -> LogicalCostResult<()> {
        if let Some(maximum) =
            self.limits.max_tracked_qubits
        {
            if tracked_qubits > maximum {
                return Err(
                    LogicalCostError::ResourceLimitExceeded {
                        resource: "tracked logical qubits",
                        requested: tracked_qubits,
                        maximum,
                    },
                );
            }
        }

        Ok(())
    }

    // -------------------------------------------------------------------------
    // Depth analysis
    // -------------------------------------------------------------------------

    fn analyze_depths(
        &self,
        gates: &[Gate],
    ) -> LogicalCostResult<(u128, u128, u128)> {
        if gates.is_empty() {
            return Ok((0, 0, 0));
        }

        let maximum_qubit = gates
            .iter()
            .flat_map(|gate| gate.qubits().iter())
            .map(|qubit| qubit_index(*qubit))
            .max()
            .unwrap_or(0);

        let tracked_qubits =
            maximum_qubit
                .checked_add(1)
                .ok_or(
                    LogicalCostError::ArithmeticOverflow {
                        calculation: "logical qubit width",
                    },
                )?;

        self.check_qubit_limit(tracked_qubits)?;

        if let Some(maximum_slots) =
            self.limits.max_depth_slots
        {
            let requested =
                tracked_qubits
                    .checked_mul(2)
                    .ok_or(
                        LogicalCostError::ArithmeticOverflow {
                            calculation: "depth-analysis storage",
                        },
                    )?;

            if requested > maximum_slots {
                return Err(
                    LogicalCostError::ResourceLimitExceeded {
                        resource: "depth-analysis slots",
                        requested,
                        maximum: maximum_slots,
                    },
                );
            }
        }

        let mut qubit_depth =
            vec![0u128; checked_usize(tracked_qubits, "depth vector")?];

        let mut qubit_t_depth =
            vec![0u128; checked_usize(tracked_qubits, "T-depth vector")?];

        let mut logical_depth = 0u128;
        let mut t_depth = 0u128;

        for gate in gates {
            if gate.qubits().is_empty() {
                continue;
            }

            let mut operation_depth = 0u128;

            for qubit in gate.qubits() {
                let index =
                    checked_usize(
                        qubit_index(*qubit),
                        "qubit index",
                    )?;

                operation_depth =
                    operation_depth.max(
                        qubit_depth[index],
                    );
            }

            let next_depth =
                operation_depth
                    .checked_add(1)
                    .ok_or(
                        LogicalCostError::ArithmeticOverflow {
                            calculation: "logical depth",
                        },
                    )?;

            for qubit in gate.qubits() {
                let index =
                    checked_usize(
                        qubit_index(*qubit),
                        "qubit index",
                    )?;

                qubit_depth[index] =
                    next_depth;
            }

            logical_depth =
                logical_depth.max(next_depth);

            if is_t_family(gate.kind()) {
                let mut operation_t_depth = 0u128;

                for qubit in gate.qubits() {
                    let index =
                        checked_usize(
                            qubit_index(*qubit),
                            "qubit index",
                        )?;

                    operation_t_depth =
                        operation_t_depth.max(
                            qubit_t_depth[index],
                        );
                }

                let next_t_depth =
                    operation_t_depth
                        .checked_add(1)
                        .ok_or(
                            LogicalCostError::ArithmeticOverflow {
                                calculation: "T depth",
                            },
                        )?;

                for qubit in gate.qubits() {
                    let index =
                        checked_usize(
                            qubit_index(*qubit),
                            "qubit index",
                        )?;

                    qubit_t_depth[index] =
                        next_t_depth;
                }

                t_depth =
                    t_depth.max(next_t_depth);
            }
        }

        Ok((
            logical_depth,
            t_depth,
            tracked_qubits,
        ))
    }
}

// =============================================================================
// Free evaluation API
// =============================================================================

/// Evaluates logical cost using the standard model.
///
/// This is the simplest public entry point.
pub fn evaluate_logical_cost(
    gates: &[Gate],
) -> LogicalCostResult<LogicalCostReport> {
    LogicalCostEvaluator::new()
        .evaluate_gates(gates)
}

/// Evaluates logical cost for a canonical QuantumCircuit.
pub fn evaluate_circuit_logical_cost(
    circuit: &crate::quantum::ir::QuantumCircuit,
) -> LogicalCostResult<LogicalCostReport> {
    LogicalCostEvaluator::new()
        .evaluate_circuit(circuit)
}

/// Evaluates logical cost with caller-supplied weights.
pub fn evaluate_with_weights(
    gates: &[Gate],
    weights: LogicalCostWeights,
) -> LogicalCostResult<LogicalCostReport> {
    LogicalCostEvaluator::with_weights(weights)?
        .evaluate_gates(gates)
}

// =============================================================================
// Comparison helpers
// =============================================================================

/// Returns true if `after` is lexicographically better than `before`.
///
/// This is intentionally conservative and deterministic.
///
/// Ordering:
///
/// 1. lower T count;
/// 2. lower two-qubit count;
/// 3. lower logical depth;
/// 4. lower total operation count;
/// 5. lower peak width.
#[must_use]
pub fn is_logically_better(
    before: &LogicalCostReport,
    after: &LogicalCostReport,
) -> bool {
    after.comparison_key()
        < before.comparison_key()
}

/// Returns whether two reports have exactly equal logical resources.
#[must_use]
pub fn equivalent_logical_cost(
    left: &LogicalCostReport,
    right: &LogicalCostReport,
) -> bool {
    left.resources == right.resources
        && left.counts == right.counts
}

// =============================================================================
// Internal arithmetic helpers
// =============================================================================

fn checked_increment(
    value: u128,
    calculation: &'static str,
) -> LogicalCostResult<u128> {
    value
        .checked_add(1)
        .ok_or(
            LogicalCostError::ArithmeticOverflow {
                calculation,
            },
        )
}

fn checked_usize(
    value: u128,
    calculation: &'static str,
) -> LogicalCostResult<usize> {
    usize::try_from(value)
        .map_err(|_| LogicalCostError::ArithmeticOverflow {
            calculation,
        })
}

fn signed_delta(
    before: u128,
    after: u128,
) -> LogicalCostResult<i128> {
    if after >= before {
        let difference =
            after
                .checked_sub(before)
                .ok_or(
                    LogicalCostError::ArithmeticOverflow {
                        calculation: "positive resource delta",
                    },
                )?;

        i128::try_from(difference)
            .map_err(|_| {
                LogicalCostError::ArithmeticOverflow {
                    calculation: "positive signed resource delta",
                }
            })
    } else {
        let difference =
            before
                .checked_sub(after)
                .ok_or(
                    LogicalCostError::ArithmeticOverflow {
                        calculation: "negative resource delta",
                    },
                )?;

        let signed =
            i128::try_from(difference)
                .map_err(|_| {
                    LogicalCostError::ArithmeticOverflow {
                        calculation: "negative signed resource delta",
                    }
                })?;

        Ok(-signed)
    }
}

/// Extracts the numeric index from the canonical `QubitId`.
///
/// `QubitId` is intentionally opaque at the optimization boundary. Its
/// canonical implementation exposes `index()`.
fn qubit_index(qubit: QubitId) -> u128 {
    qubit.index() as u128
}

/// Returns whether a gate belongs to the Clifford+T non-Clifford family.
#[must_use]
fn is_t_family(kind: GateKind) -> bool {
    matches!(
        kind,
        GateKind::T | GateKind::Tdg
    )
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::quantum::ir::{
        Gate,
        GateKind,
        QubitId,
    };

    fn q(index: usize) -> QubitId {
        QubitId::new(index)
            .expect("test qubit should be valid")
    }

    fn gate(
        kind: GateKind,
        qubits: &[usize],
    ) -> Gate {
        let operands =
            qubits
                .iter()
                .copied()
                .map(q)
                .collect();

        Gate::new(
            kind,
            operands,
            Vec::new(),
            None,
            None,
        )
        .expect("test gate should be valid")
    }

    #[test]
    fn empty_circuit_has_zero_cost() {
        let report =
            evaluate_logical_cost(&[])
                .expect("empty circuit should evaluate");

        assert_eq!(report.counts.operations, 0);
        assert_eq!(report.resources.logical_depth, 0);
        assert_eq!(report.resources.t_count, 0);
        assert_eq!(report.resources.weighted_cost, 0);
    }

    #[test]
    fn counts_single_qubit_clifford() {
        let gates = vec![
            gate(GateKind::H, &[0]),
            gate(GateKind::X, &[0]),
            gate(GateKind::Z, &[0]),
        ];

        let report =
            evaluate_logical_cost(&gates)
                .expect("evaluation should succeed");

        assert_eq!(report.counts.operations, 3);
        assert_eq!(report.counts.single_qubit, 3);
        assert_eq!(report.counts.clifford, 3);
        assert_eq!(report.counts.non_clifford, 0);
        assert_eq!(report.resources.logical_depth, 3);
    }

    #[test]
    fn disjoint_gates_can_share_depth() {
        let gates = vec![
            gate(GateKind::H, &[0]),
            gate(GateKind::H, &[1]),
            gate(GateKind::H, &[2]),
        ];

        let report =
            evaluate_logical_cost(&gates)
                .expect("evaluation should succeed");

        assert_eq!(report.resources.logical_depth, 1);
        assert_eq!(report.resources.peak_logical_width, 3);
    }

    #[test]
    fn dependent_gates_increase_depth() {
        let gates = vec![
            gate(GateKind::H, &[0]),
            gate(GateKind::H, &[0]),
            gate(GateKind::H, &[0]),
        ];

        let report =
            evaluate_logical_cost(&gates)
                .expect("evaluation should succeed");

        assert_eq!(report.resources.logical_depth, 3);
    }

    #[test]
    fn two_qubit_gate_is_counted_separately() {
        let gates = vec![
            gate(GateKind::CX, &[0, 1]),
        ];

        let report =
            evaluate_logical_cost(&gates)
                .expect("evaluation should succeed");

        assert_eq!(report.counts.operations, 1);
        assert_eq!(report.counts.two_qubit, 1);
        assert_eq!(report.counts.two_qubit_clifford, 1);
        assert_eq!(report.resources.logical_depth, 1);
        assert_eq!(report.resources.logical_qubits, 2);
    }

    #[test]
    fn t_count_and_t_depth_are_distinct() {
        let gates = vec![
            gate(GateKind::T, &[0]),
            gate(GateKind::T, &[1]),
            gate(GateKind::T, &[0]),
        ];

        let report =
            evaluate_logical_cost(&gates)
                .expect("evaluation should succeed");

        assert_eq!(report.counts.t, 3);
        assert_eq!(report.counts.t_dagger, 0);
        assert_eq!(report.resources.t_count, 3);

        // q0: T layer 1, T layer 2
        // q1: T layer 1
        // Therefore total T depth is 2.
        assert_eq!(report.resources.t_depth, 2);
    }

    #[test]
    fn t_and_t_dagger_are_both_counted() {
        let gates = vec![
            gate(GateKind::T, &[0]),
            gate(GateKind::Tdg, &[0]),
        ];

        let report =
            evaluate_logical_cost(&gates)
                .expect("evaluation should succeed");

        assert_eq!(report.counts.t, 1);
        assert_eq!(report.counts.t_dagger, 1);
        assert_eq!(report.resources.t_count, 2);
        assert_eq!(report.resources.t_depth, 2);
    }

    #[test]
    fn measurement_and_reset_are_not_clifford_gates() {
        let measurement =
            Gate::new(
                GateKind::Measure,
                vec![q(0)],
                Vec::new(),
                Some(0),
                None,
            )
            .expect("measurement should be valid");

        let reset =
            gate(GateKind::Reset, &[0]);

        let report =
            evaluate_logical_cost(&[
                measurement,
                reset,
            ])
            .expect("evaluation should succeed");

        assert_eq!(report.counts.operations, 2);
        assert_eq!(report.counts.measurements, 1);
        assert_eq!(report.counts.resets, 1);
        assert_eq!(report.counts.clifford, 0);
        assert_eq!(report.counts.non_clifford, 0);
    }

    #[test]
    fn barrier_has_zero_default_cost() {
        let barrier =
            gate(GateKind::Barrier, &[0]);

        let report =
            evaluate_logical_cost(&[barrier])
                .expect("evaluation should succeed");

        assert_eq!(report.counts.barriers, 1);
        assert_eq!(report.resources.weighted_cost, 0);
    }

    #[test]
    fn custom_weights_are_applied() {
        let mut weights =
            LogicalCostWeights::standard();

        weights.t = 100;
        weights.two_qubit_clifford = 50;

        let gates = vec![
            gate(GateKind::T, &[0]),
            gate(GateKind::CX, &[0, 1]),
        ];

        let report =
            evaluate_with_weights(
                &gates,
                weights,
            )
            .expect("custom weighting should succeed");

        assert_eq!(
            report.resources.weighted_cost,
            150
        );
    }

    #[test]
    fn cost_delta_detects_improvement() {
        let before = evaluate_logical_cost(&[
            gate(GateKind::T, &[0]),
            gate(GateKind::T, &[0]),
            gate(GateKind::CX, &[0, 1]),
        ])
        .expect("before evaluation should succeed");

        let after = evaluate_logical_cost(&[
            gate(GateKind::S, &[0]),
            gate(GateKind::CX, &[0, 1]),
        ])
        .expect("after evaluation should succeed");

        let delta =
            LogicalCostDelta::between(
                &before.resources,
                &after.resources,
            )
            .expect("delta should succeed");

        assert!(delta.reduced_t_count());
        assert!(delta.reduced_two_qubit_count() == false);
    }

    #[test]
    fn logical_comparison_is_deterministic() {
        let before = evaluate_logical_cost(&[
            gate(GateKind::T, &[0]),
            gate(GateKind::CX, &[0, 1]),
        ])
        .expect("before evaluation should succeed");

        let after = evaluate_logical_cost(&[
            gate(GateKind::CX, &[0, 1]),
        ])
        .expect("after evaluation should succeed");

        assert!(
            is_logically_better(
                &before,
                &after
            )
        );
    }

    #[test]
    fn depth_analysis_can_be_disabled() {
        let evaluator =
            LogicalCostEvaluator::new()
                .with_depth_analysis(false);

        let report =
            evaluator
                .evaluate_gates(&[
                    gate(GateKind::H, &[0]),
                    gate(GateKind::H, &[0]),
                ])
                .expect("evaluation should succeed");

        assert!(!report.depth_analyzed);
        assert_eq!(
            report.resources.logical_depth,
            0
        );
        assert_eq!(
            report.counts.operations,
            2
        );
    }

    #[test]
    fn unlimited_mode_is_default() {
        let evaluator =
            LogicalCostEvaluator::new();

        assert_eq!(
            evaluator.limits().max_tracked_qubits,
            None
        );

        assert_eq!(
            evaluator.limits().max_depth_slots,
            None
        );
    }

    #[test]
    fn logical_cost_does_not_modify_input() {
        let gates = vec![
            gate(GateKind::H, &[0]),
            gate(GateKind::T, &[0]),
            gate(GateKind::CX, &[0, 1]),
        ];

        let original =
            gates.clone();

        let _ =
            evaluate_logical_cost(&gates)
                .expect("evaluation should succeed");

        assert_eq!(gates, original);
    }
}