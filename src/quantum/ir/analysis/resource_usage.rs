//! Zamani Quantum IR — Resource Usage Analysis
//!
//! Production-grade, deterministic, read-only resource accounting for the
//! canonical logical Quantum IR.
//!
//! # Architectural role
//!
//! This module answers:
//!
//! > "What logical resources are actually represented and referenced by this
//! > IR fragment?"
//!
//! It describes the resource footprint of the IR itself.
//!
//! It does NOT estimate:
//!
//! - physical hardware cost;
//! - vendor execution cost;
//! - calibration cost;
//! - pulse/DAC cost;
//! - routing overhead;
//! - physical connectivity;
//! - QPU memory consumption;
//! - simulator state-vector memory;
//! - tensor-network memory;
//! - QEC decoder memory;
//! - execution latency;
//! - power consumption;
//! - monetary cost.
//!
//! Those concerns belong to downstream target-specific or execution-specific
//! subsystems.
//!
//! # Architectural boundary
//!
//! ```text
//! Zamani source
//!      |
//!      v
//! frontend
//!      |
//!      v
//! canonical Quantum IR
//!      |
//!      +-------------------------------+
//!      |                               |
//!      v                               v
//! analysis                       transformations
//!      |
//!      +--> statistics
//!      +--> dependencies
//!      +--> liveness
//!      +--> resource_usage  <-- this module
//!      +--> properties
//!
//! resource_usage
//!      |
//!      v
//! logical resource facts
//!      |
//!      +--> optimizer analysis
//!      +--> routing planning
//!      +--> scheduling planning
//!      +--> hardware compatibility
//!      +--> benchmarking
//!      +--> reporting
//! ```
//!
//! # Universal-program principle
//!
//! A Zamani program is written once and must be able to scale across different
//! machine sizes and quantum architectures.
//!
//! Consequently this module contains NO architectural constants such as:
//!
//! ```text
//! MAX_QUBITS = 64
//! MAX_QUBITS = 127
//! MAX_QUBITS = 4096
//! MAX_REGISTER_SIZE = 1024
//! ```
//!
//! A qubit count is data.
//!
//! The actual upper bound is determined only by:
//!
//! - the supplied IR representation;
//! - host representational capacity;
//! - available resources;
//! - explicit caller-imposed resource/security policies.
//!
//! No such policy is embedded in this module.
//!
//! # Sparse scaling
//!
//! The declared logical namespace is NOT materialized.
//!
//! For example:
//!
//! ```text
//! declared qubits = 1_000_000_000
//!
//! actual references:
//!     q7
//!     q900_000_000
//!
//! analysis storage:
//!     only q7 and q900_000_000
//! ```
//!
//! Therefore memory consumption is proportional to the number of distinct
//! resources actually observed, rather than the declared logical namespace.
//!
//! This is essential for Zamani's "atom to everywhere" scalability goal.
//!
//! # Streaming
//!
//! `ResourceUsageAccumulator` permits callers to process operations
//! incrementally:
//!
//! ```text
//! operation 1
//!     |
//!     v
//! accumulator
//!     |
//! operation 2
//!     |
//!     v
//! accumulator
//!     |
//!    ...
//!     |
//! operation N
//!     |
//!     v
//! accumulator
//!     |
//!     v
//! finish()
//! ```
//!
//! No second complete operation collection is required.
//!
//! This also provides the integration point for future whole-program,
//! region-based, block-based, distributed, and streaming analysis.
//!
//! # Logical qubit identity
//!
//! The canonical logical-qubit identity is:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! ```
//!
//! New code must NOT use the historical `qubits` module.
//!
//! # Determinism
//!
//! Results are deterministic for identical ordered input:
//!
//! - logical qubit usage is sorted by `QubitId`;
//! - arity usage is sorted by arity;
//! - no `HashMap` iteration order is exposed;
//! - no global mutable state exists;
//! - accumulation order does not affect the final sorted resource maps.
//!
//! # Arithmetic safety
//!
//! Every count is incremented using checked arithmetic.
//!
//! Overflow is reported rather than silently wrapping or saturating.
//!
//! # Memory safety
//!
//! This module:
//!
//! - contains no `unsafe`;
//! - has `#![forbid(unsafe_code)]`;
//! - never creates an array indexed by logical qubit ID;
//! - never allocates according to declared qubit count;
//! - uses sparse ordered maps;
//! - uses checked arithmetic;
//! - does not expose mutable internal collections;
//! - does not use unchecked indexing.
//!
//! # Rust compatibility
//!
//! Supported:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no external dependencies;
//! - no unsafe code.
//!
//! # Ownership contract
//!
//! This file owns:
//!
//! - resource-usage result types;
//! - sparse logical-qubit usage accounting;
//! - quantum operand-reference accounting;
//! - operation-arity accounting;
//! - maximum observed quantum arity;
//! - streaming resource accumulation;
//! - resource-analysis arithmetic errors.
//!
//! This file does NOT own:
//!
//! - `QubitId`;
//! - `Operation`;
//! - `QuantumCircuit`;
//! - gate definitions;
//! - resource definitions;
//! - resource requirements;
//! - physical topology;
//! - routing;
//! - scheduling;
//! - hardware capabilities;
//! - statistics;
//! - validation policy;
//! - execution.
//!
//! # Integration contracts
//!
//! `crate::quantum::ir::qubit`
//!     Owns canonical `QubitId`.
//!
//! `crate::quantum::ir::operation`
//!     Owns canonical `Operation` and its logical operand representation.
//!
//! `crate::quantum::ir::circuit`
//!     Supplies `QuantumCircuit` for circuit-level analysis.
//!
//! `crate::quantum::ir::analysis::statistics`
//!     Owns general statistical analysis. This module deliberately does not
//!     duplicate its gate histogram, depth, measurement, or unitary statistics.
//!
//! `crate::quantum::ir::validation`
//!     Owns whole-IR semantic validation. This module may reject malformed
//!     circuit references encountered during circuit analysis but does not
//!     replace canonical validation.
//!
//! `crate::quantum::ir::resources`
//!     Owns semantic resource definitions and requirements. This module does
//!     not reinterpret those requirements.
//!
//! `routing`
//!     May consume logical resource usage when planning physical placement.
//!
//! `scheduling`
//!     May consume resource usage when determining resource conflicts.
//!
//! `hardware`
//!     May compare logical resource requirements against target capabilities.
//!
//! `benchmarking`
//!     May consume these facts as backend-independent workload metrics.
//!
//! # Integration with analysis.rs
//!
//! Because the repository currently uses:
//!
//! ```text
//! src/quantum/ir/analysis.rs
//! ```
//!
//! as the parent analysis module, this file is declared from that module with:
//!
//! ```rust
//! pub mod resource_usage;
//! ```
//!
//! No second `analysis/mod.rs` should be created while `analysis.rs` remains
//! the parent module.
//!
//! # Future universal IR integration
//!
//! This implementation intentionally accepts `Operation` rather than a
//! gate-specific type.
//!
//! Therefore when `QuantumProgram`, regions, blocks, universal operations,
//! pulse operations, analog operations, logical operations, and distributed
//! operations become the primary analysis input, the accounting primitive
//! remains reusable.
//!
//! The future integration should be:
//!
//! ```text
//! QuantumProgram
//!     |
//!     +--> regions
//!     |      |
//!     |      +--> blocks
//!     |             |
//!     |             +--> operations
//!     |
//!     v
//! ResourceUsageAccumulator
//! ```
//!
//! The resource model itself should not need to be rewritten merely because
//! the program container becomes more expressive.
//!
//! # Important semantic distinction
//!
//! `quantum_operand_references` counts references, while
//! `distinct_logical_qubits` counts unique logical identities.
//!
//! Example:
//!
//! ```text
//! CX(q0, q1)
//! CX(q0, q1)
//! H(q0)
//! ```
//!
//! produces:
//!
//! ```text
//! quantum_operand_references = 5
//! distinct_logical_qubits    = 2
//! q0 references              = 3
//! q1 references              = 2
//! ```
//!
//! This distinction is essential for resource analysis.
//!
//! # No physical-resource inference
//!
//! This module must NEVER infer that:
//!
//! ```text
//! logical qubit q0 == physical qubit 0
//! ```
//!
//! Logical-to-physical mapping belongs to the routing/mapping layer.
//!
//! Similarly, an operation involving two logical qubits does not imply a
//! two-qubit physical gate. Decomposition, routing, ancilla usage, SWAPs, and
//! physical resource overhead belong downstream.
//!
//! # File completion contract
//!
//! Once this file is implemented, downstream files may change their internal
//! implementation without requiring this file to change, provided these
//! existing public contracts remain stable:
//!
//! - `QuantumCircuit::num_qubits()`;
//! - `QuantumCircuit::operations()`;
//! - `Operation::qubits()`;
//! - `QubitId::index()`.
//!
//! The implementation therefore depends only on stable semantic contracts,
//! rather than on concrete gate enumerations or hardware assumptions.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::BTreeMap;
use std::fmt;

use crate::quantum::ir::circuit::QuantumCircuit;
use crate::quantum::ir::operation::Operation;
use crate::quantum::ir::qubit::QubitId;

// =============================================================================
// Result
// =============================================================================

/// Result returned by resource-usage analysis.
pub type ResourceUsageResult<T> = Result<T, ResourceUsageError>;

// =============================================================================
// Errors
// =============================================================================

/// Errors produced while calculating logical resource usage.
///
/// Resource analysis never silently wraps counters. An overflow is reported
/// because silently changing a resource count would make downstream planning
/// and benchmarking incorrect.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceUsageError {
    /// A resource counter could not be incremented safely.
    CounterOverflow {
        /// Name of the counter that overflowed.
        counter: &'static str,
    },

    /// A logical qubit reference is outside the circuit's declared namespace.
    ///
    /// This indicates malformed or externally constructed IR. A normally
    /// constructed `QuantumCircuit` should already enforce this invariant.
    QubitOutOfRange {
        /// Referenced logical qubit.
        qubit: QubitId,

        /// Declared logical namespace size.
        num_qubits: usize,
    },
}

impl fmt::Display for ResourceUsageError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CounterOverflow { counter } => {
                write!(
                    formatter,
                    "resource-usage counter overflow: {counter}"
                )
            }

            Self::QubitOutOfRange {
                qubit,
                num_qubits,
            } => {
                write!(
                    formatter,
                    "logical qubit {qubit} is outside circuit namespace 0..{num_qubits}"
                )
            }
        }
    }
}

impl std::error::Error for ResourceUsageError {}

// =============================================================================
// Per-qubit usage
// =============================================================================

/// Resource usage for one logical qubit.
///
/// The logical identity is the canonical
/// `quantum::ir::qubit::QubitId`.
///
/// This type contains no physical placement information.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LogicalQubitResourceUsage {
    qubit: QubitId,
    references: usize,
}

impl LogicalQubitResourceUsage {
    /// Creates a usage record.
    #[must_use]
    const fn new(qubit: QubitId, references: usize) -> Self {
        Self { qubit, references }
    }

    /// Returns the canonical logical qubit identity.
    #[must_use]
    pub const fn qubit(&self) -> QubitId {
        self.qubit
    }

    /// Returns how many operation operands reference this qubit.
    #[must_use]
    pub const fn references(&self) -> usize {
        self.references
    }
}

// =============================================================================
// Arity usage
// =============================================================================

/// Resource usage for one quantum operand arity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QuantumArityUsage {
    arity: usize,
    operation_count: usize,
}

impl QuantumArityUsage {
    /// Creates an arity-usage record.
    #[must_use]
    const fn new(arity: usize, operation_count: usize) -> Self {
        Self {
            arity,
            operation_count,
        }
    }

    /// Returns the number of logical quantum operands.
    #[must_use]
    pub const fn arity(&self) -> usize {
        self.arity
    }

    /// Returns the number of operations having this arity.
    #[must_use]
    pub const fn operation_count(&self) -> usize {
        self.operation_count
    }
}

// =============================================================================
// Complete result
// =============================================================================

/// Deterministic logical resource-usage report.
///
/// This report describes the semantic footprint of the supplied IR.
///
/// It does NOT describe physical hardware resources.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourceUsage {
    /// Number of logical qubits declared by the analyzed circuit.
    declared_logical_qubits: usize,

    /// Number of operations analyzed.
    operation_count: usize,

    /// Total number of logical quantum operand references.
    ///
    /// A qubit referenced by three operations contributes three references.
    quantum_operand_references: usize,

    /// Number of distinct logical qubits actually referenced.
    distinct_logical_qubits: usize,

    /// Maximum number of quantum operands on any single operation.
    maximum_quantum_arity: usize,

    /// Number of operations with zero quantum operands.
    zero_quantum_operand_operations: usize,

    /// Number of operations with exactly one quantum operand.
    single_qubit_operations: usize,

    /// Number of operations with exactly two quantum operands.
    two_qubit_operations: usize,

    /// Number of operations with three or more quantum operands.
    multi_qubit_operations: usize,

    /// Deterministic sparse usage for each referenced logical qubit.
    logical_qubits: Vec<LogicalQubitResourceUsage>,

    /// Deterministic quantum-arity histogram.
    arity: Vec<QuantumArityUsage>,
}

impl ResourceUsage {
    /// Returns the number of logical qubits declared by the circuit.
    #[must_use]
    pub const fn declared_logical_qubits(&self) -> usize {
        self.declared_logical_qubits
    }

    /// Returns the number of analyzed operations.
    #[must_use]
    pub const fn operation_count(&self) -> usize {
        self.operation_count
    }

    /// Returns the total number of logical quantum operand references.
    #[must_use]
    pub const fn quantum_operand_references(&self) -> usize {
        self.quantum_operand_references
    }

    /// Returns the number of distinct logical qubits actually referenced.
    #[must_use]
    pub const fn distinct_logical_qubits(&self) -> usize {
        self.distinct_logical_qubits
    }

    /// Returns the maximum quantum operand arity observed.
    #[must_use]
    pub const fn maximum_quantum_arity(&self) -> usize {
        self.maximum_quantum_arity
    }

    /// Returns the number of operations with no logical quantum operands.
    #[must_use]
    pub const fn zero_quantum_operand_operations(&self) -> usize {
        self.zero_quantum_operand_operations
    }

    /// Returns the number of one-qubit operations.
    #[must_use]
    pub const fn single_qubit_operations(&self) -> usize {
        self.single_qubit_operations
    }

    /// Returns the number of two-qubit operations.
    #[must_use]
    pub const fn two_qubit_operations(&self) -> usize {
        self.two_qubit_operations
    }

    /// Returns the number of operations involving three or more logical
    /// quantum operands.
    #[must_use]
    pub const fn multi_qubit_operations(&self) -> usize {
        self.multi_qubit_operations
    }

    /// Returns sparse logical-qubit usage in ascending `QubitId` order.
    #[must_use]
    pub fn logical_qubits(&self) -> &[LogicalQubitResourceUsage] {
        &self.logical_qubits
    }

    /// Returns the quantum-arity histogram in ascending arity order.
    #[must_use]
    pub fn arity(&self) -> &[QuantumArityUsage] {
        &self.arity
    }

    /// Returns the number of operations having the supplied quantum arity.
    #[must_use]
    pub fn operation_count_for_arity(&self, arity: usize) -> usize {
        match self
            .arity
            .binary_search_by_key(&arity, QuantumArityUsage::arity)
        {
            Ok(index) => self.arity[index].operation_count(),
            Err(_) => 0,
        }
    }

    /// Returns usage information for one logical qubit.
    #[must_use]
    pub fn usage_for_qubit(
        &self,
        qubit: QubitId,
    ) -> Option<LogicalQubitResourceUsage> {
        match self
            .logical_qubits
            .binary_search_by_key(&qubit, LogicalQubitResourceUsage::qubit)
        {
            Ok(index) => self.logical_qubits.get(index).copied(),
            Err(_) => None,
        }
    }

    /// Returns the fraction of declared logical qubits actually referenced.
    ///
    /// `None` is returned for a circuit with zero declared logical qubits.
    ///
    /// The value is informational only and must not be interpreted as hardware
    /// utilization.
    #[must_use]
    pub fn logical_qubit_utilization(&self) -> Option<f64> {
        if self.declared_logical_qubits == 0 {
            None
        } else {
            Some(
                self.distinct_logical_qubits as f64
                    / self.declared_logical_qubits as f64,
            )
        }
    }
}

// =============================================================================
// Streaming accumulator
// =============================================================================

/// Streaming sparse resource-usage accumulator.
///
/// This is the primary primitive for large IR workloads.
///
/// Memory grows with the number of distinct logical resources observed, not
/// with the declared logical namespace.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourceUsageAccumulator {
    operation_count: usize,
    quantum_operand_references: usize,
    maximum_quantum_arity: usize,

    zero_quantum_operand_operations: usize,
    single_qubit_operations: usize,
    two_qubit_operations: usize,
    multi_qubit_operations: usize,

    logical_qubits: BTreeMap<QubitId, usize>,
    arity: BTreeMap<usize, usize>,
}

impl ResourceUsageAccumulator {
    /// Creates an empty resource-usage accumulator.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            operation_count: 0,
            quantum_operand_references: 0,
            maximum_quantum_arity: 0,

            zero_quantum_operand_operations: 0,
            single_qubit_operations: 0,
            two_qubit_operations: 0,
            multi_qubit_operations: 0,

            logical_qubits: BTreeMap::new(),
            arity: BTreeMap::new(),
        }
    }

    /// Adds one canonical operation to the resource analysis.
    ///
    /// The operation's logical qubit operands are obtained exclusively through
    /// `Operation::qubits()`.
    ///
    /// No gate enumeration is required, so this method remains valid when new
    /// operation kinds are added to the canonical IR.
    pub fn observe_operation(
        &mut self,
        operation: &Operation,
    ) -> ResourceUsageResult<()> {
        self.operation_count =
            checked_increment(self.operation_count, "operation count")?;

        let qubits = operation.qubits();
        let arity = qubits.len();

        if arity > self.maximum_quantum_arity {
            self.maximum_quantum_arity = arity;
        }

        match arity {
            0 => {
                self.zero_quantum_operand_operations =
                    checked_increment(
                        self.zero_quantum_operand_operations,
                        "zero-quantum-operand operation count",
                    )?;
            }

            1 => {
                self.single_qubit_operations =
                    checked_increment(
                        self.single_qubit_operations,
                        "single-qubit operation count",
                    )?;
            }

            2 => {
                self.two_qubit_operations =
                    checked_increment(
                        self.two_qubit_operations,
                        "two-qubit operation count",
                    )?;
            }

            _ => {
                self.multi_qubit_operations =
                    checked_increment(
                        self.multi_qubit_operations,
                        "multi-qubit operation count",
                    )?;
            }
        }

        self.quantum_operand_references =
            self.quantum_operand_references
                .checked_add(arity)
                .ok_or(ResourceUsageError::CounterOverflow {
                    counter: "quantum operand references",
                })?;

        increment_map_count(
            &mut self.arity,
            arity,
            "quantum arity histogram",
        )?;

        for qubit in qubits {
            increment_map_count(
                &mut self.logical_qubits,
                qubit,
                "logical qubit usage",
            )?;
        }

        Ok(())
    }

    /// Finalizes the accumulator into an immutable deterministic report.
    #[must_use]
    pub fn finish(self, declared_logical_qubits: usize) -> ResourceUsage {
        let logical_qubits = self
            .logical_qubits
            .into_iter()
            .map(|(qubit, references)| {
                LogicalQubitResourceUsage::new(qubit, references)
            })
            .collect();

        let arity = self
            .arity
            .into_iter()
            .map(|(arity, operation_count)| {
                QuantumArityUsage::new(arity, operation_count)
            })
            .collect();

        let distinct_logical_qubits = logical_qubits.len();

        ResourceUsage {
            declared_logical_qubits,
            operation_count: self.operation_count,
            quantum_operand_references: self.quantum_operand_references,
            distinct_logical_qubits,
            maximum_quantum_arity: self.maximum_quantum_arity,
            zero_quantum_operand_operations:
                self.zero_quantum_operand_operations,
            single_qubit_operations: self.single_qubit_operations,
            two_qubit_operations: self.two_qubit_operations,
            multi_qubit_operations: self.multi_qubit_operations,
            logical_qubits,
            arity,
        }
    }

    /// Returns the number of operations observed so far.
    #[must_use]
    pub const fn operation_count(&self) -> usize {
        self.operation_count
    }

    /// Returns the number of distinct logical qubits observed so far.
    #[must_use]
    pub fn distinct_logical_qubits(&self) -> usize {
        self.logical_qubits.len()
    }

    /// Returns the total number of quantum operand references observed so far.
    #[must_use]
    pub const fn quantum_operand_references(&self) -> usize {
        self.quantum_operand_references
    }

    /// Returns the maximum quantum arity observed so far.
    #[must_use]
    pub const fn maximum_quantum_arity(&self) -> usize {
        self.maximum_quantum_arity
    }

    /// Clears all accumulated state.
    ///
    /// The accumulator remains reusable after this operation.
    pub fn clear(&mut self) {
        self.operation_count = 0;
        self.quantum_operand_references = 0;
        self.maximum_quantum_arity = 0;

        self.zero_quantum_operand_operations = 0;
        self.single_qubit_operations = 0;
        self.two_qubit_operations = 0;
        self.multi_qubit_operations = 0;

        self.logical_qubits.clear();
        self.arity.clear();
    }
}

impl Default for ResourceUsageAccumulator {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Public analysis functions
// =============================================================================

/// Analyzes the logical resource usage of a complete canonical circuit.
///
/// The circuit's declared namespace is used only for validating references
/// and reporting the declared logical-qubit count.
///
/// No allocation proportional to `circuit.num_qubits()` occurs.
pub fn analyze(
    circuit: &QuantumCircuit,
) -> ResourceUsageResult<ResourceUsage> {
    let declared_logical_qubits = circuit.num_qubits();

    let mut accumulator = ResourceUsageAccumulator::new();

    for operation in circuit.operations() {
        validate_qubit_namespace(
            operation,
            declared_logical_qubits,
        )?;

        accumulator.observe_operation(operation)?;
    }

    Ok(accumulator.finish(declared_logical_qubits))
}

/// Alias with an explicit circuit-oriented name.
///
/// This is useful at call sites where several kinds of resource analysis
/// coexist.
pub fn analyze_circuit(
    circuit: &QuantumCircuit,
) -> ResourceUsageResult<ResourceUsage> {
    analyze(circuit)
}

/// Creates an empty accumulator for streaming analysis.
///
/// This function exists as a stable factory for callers that prefer a
/// functional-style API.
#[must_use]
pub const fn accumulator() -> ResourceUsageAccumulator {
    ResourceUsageAccumulator::new()
}

// =============================================================================
// Internal helpers
// =============================================================================

/// Safely increments a `usize` counter.
fn checked_increment(
    value: usize,
    counter: &'static str,
) -> ResourceUsageResult<usize> {
    value
        .checked_add(1)
        .ok_or(ResourceUsageError::CounterOverflow {
            counter,
        })
}

/// Increments one sparse map counter without exposing mutable collection
/// details to callers.
///
/// `BTreeMap` provides deterministic key ordering for finalization.
fn increment_map_count<K>(
    map: &mut BTreeMap<K, usize>,
    key: K,
    collection: &'static str,
) -> ResourceUsageResult<()>
where
    K: Ord,
{
    match map.get_mut(&key) {
        Some(count) => {
            *count = checked_increment(*count, collection)?;
        }

        None => {
            map.insert(key, 1);
        }
    }

    Ok(())
}

/// Validates every logical operand of an operation against the circuit's
/// declared logical namespace.
///
/// This is deliberately local defensive validation. Full semantic validation
/// remains owned by the canonical validation subsystem.
fn validate_qubit_namespace(
    operation: &Operation,
    num_qubits: usize,
) -> ResourceUsageResult<()> {
    for qubit in operation.qubits() {
        if qubit.index() >= num_qubits {
            return Err(ResourceUsageError::QubitOutOfRange {
                qubit,
                num_qubits,
            });
        }
    }

    Ok(())
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_accumulator_is_zero() {
        let accumulator = ResourceUsageAccumulator::new();

        assert_eq!(accumulator.operation_count(), 0);
        assert_eq!(accumulator.distinct_logical_qubits(), 0);
        assert_eq!(accumulator.quantum_operand_references(), 0);
        assert_eq!(accumulator.maximum_quantum_arity(), 0);
    }

    #[test]
    fn empty_result_preserves_declared_namespace_without_materializing_it() {
        let result =
            ResourceUsageAccumulator::new().finish(1_000_000);

        assert_eq!(result.declared_logical_qubits(), 1_000_000);
        assert_eq!(result.distinct_logical_qubits(), 0);
        assert!(result.logical_qubits().is_empty());
        assert_eq!(
            result.quantum_operand_references(),
            0
        );
        assert_eq!(
            result.logical_qubit_utilization(),
            Some(0.0)
        );
    }

    #[test]
    fn sparse_map_counts_repeated_qubits() {
        let mut accumulator =
            ResourceUsageAccumulator::new();

        accumulator
            .logical_qubits
            .insert(QubitId::new(7), 3);

        let result = accumulator.finish(100);

        assert_eq!(result.distinct_logical_qubits(), 1);

        let usage = result
            .usage_for_qubit(QubitId::new(7))
            .expect("q7 must exist");

        assert_eq!(usage.qubit(), QubitId::new(7));
        assert_eq!(usage.references(), 3);
    }

    #[test]
    fn arity_histogram_is_sorted() {
        let mut accumulator =
            ResourceUsageAccumulator::new();

        accumulator.arity.insert(4, 2);
        accumulator.arity.insert(1, 5);
        accumulator.arity.insert(2, 7);

        let result = accumulator.finish(10);

        assert_eq!(result.arity()[0].arity(), 1);
        assert_eq!(result.arity()[1].arity(), 2);
        assert_eq!(result.arity()[2].arity(), 4);
    }

    #[test]
    fn utilization_is_none_for_zero_declared_qubits() {
        let result =
            ResourceUsageAccumulator::new().finish(0);

        assert_eq!(
            result.logical_qubit_utilization(),
            None
        );
    }

    #[test]
    fn utilization_uses_distinct_references() {
        let mut accumulator =
            ResourceUsageAccumulator::new();

        accumulator
            .logical_qubits
            .insert(QubitId::new(1), 100);
        accumulator
            .logical_qubits
            .insert(QubitId::new(9), 1);

        let result = accumulator.finish(10);

        assert_eq!(
            result.logical_qubit_utilization(),
            Some(0.2)
        );
    }

    #[test]
    fn checked_increment_reports_overflow() {
        let result =
            checked_increment(usize::MAX, "test counter");

        assert_eq!(
            result,
            Err(ResourceUsageError::CounterOverflow {
                counter: "test counter",
            })
        );
    }

    #[test]
    fn clear_reuses_accumulator() {
        let mut accumulator =
            ResourceUsageAccumulator::new();

        accumulator.operation_count = 42;
        accumulator.quantum_operand_references = 99;
        accumulator.maximum_quantum_arity = 8;
        accumulator
            .logical_qubits
            .insert(QubitId::new(3), 4);
        accumulator.arity.insert(2, 1);

        accumulator.clear();

        assert_eq!(accumulator.operation_count(), 0);
        assert_eq!(
            accumulator.quantum_operand_references(),
            0
        );
        assert_eq!(
            accumulator.maximum_quantum_arity(),
            0
        );
        assert_eq!(
            accumulator.distinct_logical_qubits(),
            0
        );
    }
}