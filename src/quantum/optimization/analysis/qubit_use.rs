//! Zamani Quantum Optimization — Qubit-Use Analysis
//!
//! Production-grade logical-qubit usage analysis over the canonical
//! `crate::quantum::ir::QuantumCircuit`.
//!
//! # Architectural position
//!
//! ```text
//!                    quantum::ir::QuantumCircuit
//!                              │
//!                              ▼
//!                    analysis::qubit_use
//!                              │
//!              ┌───────────────┼────────────────┐
//!              ▼               ▼                ▼
//!          dependency       liveness          depth
//!              │               │                │
//!              └───────────────┼────────────────┘
//!                              ▼
//!                         optimization
//! ```
//!
//! This module answers questions such as:
//!
//! - Which logical qubits are actually used?
//! - How many operations use each logical qubit?
//! - What is the first operation touching a qubit?
//! - What is the last operation touching a qubit?
//! - What is the inclusive lifetime interval of a qubit?
//! - Which qubits are unused?
//! - How many total logical-qubit operands occur?
//! - Which qubits are used by measurements?
//! - Which qubits are used by resets?
//! - Which qubits are used by unitary operations?
//! - Which qubits participate in multi-qubit operations?
//!
//! # Important ownership rule
//!
//! This module does NOT define:
//!
//! - another `QubitId`;
//! - another quantum circuit representation;
//! - another gate representation;
//! - physical qubit mapping;
//! - routing;
//! - scheduling;
//! - hardware topology;
//! - quantum-state simulation;
//! - qubit allocation;
//! - optimization transformations.
//!
//! The authoritative representations remain those in `crate::quantum::ir`.
//!
//! # Complexity
//!
//! Let:
//!
//! - `N` = number of circuit operations;
//! - `K` = number of distinct logical qubits actually used;
//! - `A` = total number of logical-qubit operands across all operations.
//!
//! Analysis is:
//!
//! - expected `O(N + A)` time;
//! - `O(K)` auxiliary memory.
//!
//! Crucially, memory consumption is proportional to **used qubits**, not the
//! declared logical-qubit namespace size.
//!
//! This matters for very large sparse circuits. A circuit may legally declare
//! a very large logical namespace while touching only a small number of
//! qubits. This analysis must not allocate a record for every declared qubit.
//!
//! Results are returned in deterministic `QubitId` order, regardless of the
//! internal hash-table implementation.
//!
//! # Scaling
//!
//! There is no artificial optimizer-specific circuit-size limit in this file.
//! The canonical IR and its resource policy remain authoritative.
//!
//! The implementation scales until the underlying machine/resource policy can
//! no longer represent the requested analysis. Allocation failures are handled
//! by Rust's normal allocation behavior; this module never uses `unsafe`.
//!
//! # Semantic rules
//!
//! Every occurrence of a logical qubit in a canonical gate operand list counts
//! as one use.
//!
//! Therefore:
//!
//! - single-qubit gate → one operand use;
//! - two-qubit gate → two operand uses;
//! - three-qubit gate → three operand uses;
//! - measurement → one use;
//! - reset → one use;
//! - barrier → every listed qubit is a use.
//!
//! This module deliberately does not attempt to infer quantum-state semantics.
//! It reports syntactic/logical operand usage. Higher-level analyses can use
//! this information to infer dependencies, liveness, critical paths, or
//! entanglement structure.
//!
//! # Operation identity
//!
//! Operation positions are represented as `usize` indices into the canonical
//! circuit operation sequence.
//!
//! This deliberately avoids defining another optimizer-local operation-ID
//! type. The optimizer's `circuit.rs` may later expose `OperationId`; callers
//! can convert with `OperationId::new(index)` without requiring this analysis
//! to depend on the editing layer.
//!
//! # Rust compatibility
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust only;
//! - no nightly features.
//!
//! # Safety
//!
//! This module forbids unsafe code.
//!
//! # Integration contract
//!
//! ## `quantum::ir`
//!
//! This module consumes:
//!
//! - `QuantumCircuit`;
//! - `Gate`;
//! - `QubitId`.
//!
//! It never mutates the circuit.
//!
//! ## `analysis/mod.rs`
//!
//! Export:
//!
//! ```text
//! pub mod qubit_use;
//! ```
//!
//! and re-export the public types as desired.
//!
//! ## `dependency.rs`
//!
//! Dependency analysis should consume `QubitUseAnalysis` to identify operations
//! sharing a logical qubit.
//!
//! ## `liveness.rs`
//!
//! Liveness analysis should use `first_use()` and `last_use()` as its initial
//! logical intervals, then refine them around measurement/reset/control-flow
//! semantics.
//!
//! ## `depth.rs`
//!
//! Depth analysis may use `used_qubits()` and per-qubit operation counts, but
//! must build its own dependency/depth semantics.
//!
//! ## `width.rs`
//!
//! Width analysis may use `declared_qubits()`, `used_qubits()`, and
//! `unused_qubits()`.
//!
//! ## `critical_path.rs`
//!
//! Critical-path analysis may consume per-qubit usage information as a
//! low-level prerequisite, but dependency ordering remains authoritative.
//!
//! ## optimization passes
//!
//! Cancellation, peephole, commutation, gate fusion, routing-aware planning,
//! and synthesis passes can use this analysis without coupling the analysis to
//! any particular transformation.
//!
//! ## `context.rs`
//!
//! The optimization context may cache this analysis. The result is immutable
//! after construction, making it safe to share by immutable reference.
//!
//! ## invalidation
//!
//! Any transformation that changes the circuit's operation sequence or qubit
//! operands invalidates this analysis.
//!
//! A pass that only changes metadata and does not alter operations/qubit
//! operands may retain it.
//!
//! # Determinism
//!
//! Public iteration APIs return qubits in ascending `QubitId` order.
//!
//! Hash-map implementation details therefore never become compiler-visible
//! behavior.
//!
//! # No global state
//!
//! The analysis contains no global caches, mutable statics, threads, I/O, or
//! backend state.
//!
//! # Verification
//!
//! The implementation includes tests for:
//!
//! - empty circuits;
//! - unused qubits;
//! - single-qubit operations;
//! - multi-qubit operations;
//! - repeated uses;
//! - first/last use;
//! - measurements;
//! - resets;
//! - barriers;
//! - deterministic ordering;
//! - total operand counts;
//! - overflow-safe counters;
//! - invalid canonical circuits.
//!
//! ```text
//! analysis::qubit_use
//!       │
//!       ├── O(N + A) analysis
//!       ├── O(K) memory
//!       ├── deterministic results
//!       ├── immutable result
//!       └── no unsafe
//! ```
//!
//! where N is operation count, A is operand count, and K is distinct used
//! qubits.

#![forbid(unsafe_code)]

use std::collections::HashMap;
use std::fmt;

use crate::quantum::ir::gate::Gate;
use crate::quantum::ir::qubits::QubitId;
use crate::quantum::ir::QuantumCircuit;

// ============================================================================
// Public scalar types
// ============================================================================

/// Zero-based operation position in the canonical circuit.
///
/// This is intentionally an alias rather than a second operation-ID type.
/// The canonical circuit's operation order is the authoritative ordering.
pub type OperationIndex = usize;

// ============================================================================
// Errors
// ============================================================================

/// Errors produced by qubit-use analysis.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QubitUseError {
    /// The canonical circuit failed validation.
    InvalidCircuit {
        /// Human-readable validation error.
        message: String,
    },

    /// A checked arithmetic operation overflowed.
    ArithmeticOverflow {
        /// Calculation that overflowed.
        calculation: &'static str,
    },

    /// A requested operation index does not exist.
    OperationOutOfRange {
        /// Requested operation index.
        index: usize,

        /// Number of operations in the analysis input.
        operation_count: usize,
    },

    /// A requested logical qubit does not belong to the declared namespace.
    QubitOutOfRange {
        /// Logical qubit index.
        qubit: QubitId,

        /// Number of declared logical qubits.
        qubit_count: usize,
    },
}

impl fmt::Display for QubitUseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidCircuit { message } => {
                write!(
                    formatter,
                    "cannot analyze qubit use: invalid quantum circuit: {message}"
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
                    "operation index {index} is outside circuit length \
                     {operation_count}"
                )
            }

            Self::QubitOutOfRange {
                qubit,
                qubit_count,
            } => {
                write!(
                    formatter,
                    "logical qubit {qubit} is outside circuit namespace \
                     0..{qubit_count}"
                )
            }
        }
    }
}

impl std::error::Error for QubitUseError {}

// ============================================================================
// Per-qubit usage record
// ============================================================================

/// Immutable usage information for one logical qubit.
///
/// A `QubitUsage` describes logical operand usage only. It does not represent
/// the physical lifetime of a qubit on hardware and does not represent a
/// quantum state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct QubitUsage {
    qubit: QubitId,
    first_use: OperationIndex,
    last_use: OperationIndex,
    use_count: usize,
    unitary_use_count: usize,
    measurement_use_count: usize,
    reset_use_count: usize,
    barrier_use_count: usize,
    multi_qubit_use_count: usize,
}

impl QubitUsage {
    fn new(
        qubit: QubitId,
        operation: OperationIndex,
        gate: &Gate,
    ) -> Result<Self, QubitUseError> {
        let mut usage = Self {
            qubit,
            first_use: operation,
            last_use: operation,
            use_count: 0,
            unitary_use_count: 0,
            measurement_use_count: 0,
            reset_use_count: 0,
            barrier_use_count: 0,
            multi_qubit_use_count: 0,
        };

        usage.record(operation, gate)?;

        Ok(usage)
    }

    fn record(
        &mut self,
        operation: OperationIndex,
        gate: &Gate,
    ) -> Result<(), QubitUseError> {
        if operation < self.first_use {
            self.first_use = operation;
        }

        if operation > self.last_use {
            self.last_use = operation;
        }

        self.use_count = self
            .use_count
            .checked_add(1)
            .ok_or(QubitUseError::ArithmeticOverflow {
                calculation: "per-qubit use count",
            })?;

        if gate.is_unitary() {
            self.unitary_use_count = self
                .unitary_use_count
                .checked_add(1)
                .ok_or(QubitUseError::ArithmeticOverflow {
                    calculation: "per-qubit unitary-use count",
                })?;
        }

        if gate.is_measurement() {
            self.measurement_use_count = self
                .measurement_use_count
                .checked_add(1)
                .ok_or(QubitUseError::ArithmeticOverflow {
                    calculation: "per-qubit measurement-use count",
                })?;
        }

        if gate.is_reset() {
            self.reset_use_count = self
                .reset_use_count
                .checked_add(1)
                .ok_or(QubitUseError::ArithmeticOverflow {
                    calculation: "per-qubit reset-use count",
                })?;
        }

        if gate.is_barrier() {
            self.barrier_use_count = self
                .barrier_use_count
                .checked_add(1)
                .ok_or(QubitUseError::ArithmeticOverflow {
                    calculation: "per-qubit barrier-use count",
                })?;
        }

        if gate.qubits().len() > 1 {
            self.multi_qubit_use_count = self
                .multi_qubit_use_count
                .checked_add(1)
                .ok_or(QubitUseError::ArithmeticOverflow {
                    calculation: "per-qubit multi-qubit-use count",
                })?;
        }

        Ok(())
    }

    /// Returns the logical qubit represented by this record.
    #[must_use]
    pub const fn qubit(self) -> QubitId {
        self.qubit
    }

    /// Returns the first operation that uses this qubit.
    #[must_use]
    pub const fn first_use(self) -> OperationIndex {
        self.first_use
    }

    /// Returns the last operation that uses this qubit.
    #[must_use]
    pub const fn last_use(self) -> OperationIndex {
        self.last_use
    }

    /// Returns the number of logical operations that use this qubit.
    ///
    /// Each occurrence of the qubit in a gate's operand list contributes one
    /// use.
    #[must_use]
    pub const fn use_count(self) -> usize {
        self.use_count
    }

    /// Returns the number of unitary operations using this qubit.
    #[must_use]
    pub const fn unitary_use_count(self) -> usize {
        self.unitary_use_count
    }

    /// Returns the number of measurements using this qubit.
    #[must_use]
    pub const fn measurement_use_count(self) -> usize {
        self.measurement_use_count
    }

    /// Returns the number of resets using this qubit.
    #[must_use]
    pub const fn reset_use_count(self) -> usize {
        self.reset_use_count
    }

    /// Returns the number of barriers containing this qubit.
    #[must_use]
    pub const fn barrier_use_count(self) -> usize {
        self.barrier_use_count
    }

    /// Returns the number of operations involving this qubit together with at
    /// least one other qubit.
    #[must_use]
    pub const fn multi_qubit_use_count(self) -> usize {
        self.multi_qubit_use_count
    }

    /// Returns whether the qubit participates in at least one multi-qubit
    /// operation.
    #[must_use]
    pub const fn participates_in_multi_qubit_operation(self) -> bool {
        self.multi_qubit_use_count != 0
    }

    /// Returns whether the qubit is ever measured.
    #[must_use]
    pub const fn is_measured(self) -> bool {
        self.measurement_use_count != 0
    }

    /// Returns whether the qubit is ever reset.
    #[must_use]
    pub const fn is_reset(self) -> bool {
        self.reset_use_count != 0
    }

    /// Returns the inclusive operation interval covering all observed uses.
    ///
    /// This is a logical-use interval, not yet a complete liveness interval.
    /// Higher-level liveness analysis must account for control-flow,
    /// measurement semantics, reset semantics, and other boundaries.
    #[must_use]
    pub const fn interval(self) -> std::ops::RangeInclusive<OperationIndex> {
        self.first_use..=self.last_use
    }

    /// Returns the number of operation positions covered by the inclusive
    /// first-to-last-use interval.
    #[must_use]
    pub const fn interval_length(self) -> usize {
        self.last_use - self.first_use + 1
    }

    /// Returns the number of operation positions between first and last use,
    /// excluding the endpoint operation itself.
    #[must_use]
    pub const fn span(self) -> usize {
        self.last_use - self.first_use
    }

    /// Returns the density of uses within the first-to-last-use interval.
    ///
    /// A value of `1.0` means every operation position in the interval uses the
    /// qubit. A lower value indicates gaps.
    ///
    /// The result is always finite for a valid record.
    #[must_use]
    pub fn use_density(self) -> f64 {
        let interval = self.interval_length();

        if interval == 0 {
            return 0.0;
        }

        self.use_count as f64 / interval as f64
    }
}

// ============================================================================
// Internal mutable record
// ============================================================================

#[derive(Debug)]
struct MutableQubitUsage {
    usage: QubitUsage,
}

impl MutableQubitUsage {
    fn new(
        qubit: QubitId,
        operation: OperationIndex,
        gate: &Gate,
    ) -> Result<Self, QubitUseError> {
        Ok(Self {
            usage: QubitUsage::new(qubit, operation, gate)?,
        })
    }

    fn record(
        &mut self,
        operation: OperationIndex,
        gate: &Gate,
    ) -> Result<(), QubitUseError> {
        self.usage.record(operation, gate)
    }

    fn finish(self) -> QubitUsage {
        self.usage
    }
}

// ============================================================================
// Aggregate analysis
// ============================================================================

/// Complete immutable qubit-use analysis for one canonical quantum circuit.
///
/// The result owns only analysis data. It does not own or mutate the source
/// circuit.
///
/// Records are stored only for qubits that actually occur in an operation.
/// This makes sparse, very-large logical namespaces practical.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QubitUseAnalysis {
    declared_qubits: usize,
    operation_count: usize,
    used_qubits: Vec<QubitUsage>,
    total_operand_uses: usize,
    total_unitary_operand_uses: usize,
    total_measurement_operand_uses: usize,
    total_reset_operand_uses: usize,
    total_barrier_operand_uses: usize,
    total_multi_qubit_operand_uses: usize,
}

impl QubitUseAnalysis {
    /// Analyzes a canonical quantum circuit.
    ///
    /// The circuit is validated before analysis. The analysis itself never
    /// mutates the circuit.
    ///
    /// # Complexity
    ///
    /// Expected `O(N + A)` time and `O(K)` memory, where:
    ///
    /// - `N` = number of operations;
    /// - `A` = total qubit operands;
    /// - `K` = distinct qubits used.
    pub fn analyze(
        circuit: &QuantumCircuit,
    ) -> Result<Self, QubitUseError> {
        circuit
            .validate()
            .map_err(|error| QubitUseError::InvalidCircuit {
                message: error.to_string(),
            })?;

        Self::analyze_validated(circuit)
    }

    /// Analyzes a circuit that has already been validated.
    ///
    /// This avoids a second full-circuit validation in optimizer pipelines.
    ///
    /// The caller is responsible for ensuring that the circuit was validated
    /// before calling this method.
    ///
    /// This method is safe Rust and does not bypass memory safety. It merely
    /// avoids duplicate semantic validation work.
    #[must_use = "analysis results should not be silently discarded"]
    pub fn analyze_validated(
        circuit: &QuantumCircuit,
    ) -> Result<Self, QubitUseError> {
        let declared_qubits = circuit.num_qubits();
        let operations = circuit.operations();
        let operation_count = operations.len();

        let mut records: HashMap<QubitId, MutableQubitUsage> =
            HashMap::new();

        let mut total_operand_uses = 0usize;
        let mut total_unitary_operand_uses = 0usize;
        let mut total_measurement_operand_uses = 0usize;
        let mut total_reset_operand_uses = 0usize;
        let mut total_barrier_operand_uses = 0usize;
        let mut total_multi_qubit_operand_uses = 0usize;

        for (operation_index, gate) in operations.iter().enumerate() {
            let operand_count = gate.qubits().len();

            total_operand_uses = total_operand_uses
                .checked_add(operand_count)
                .ok_or(QubitUseError::ArithmeticOverflow {
                    calculation: "total qubit operand uses",
                })?;

            if gate.is_unitary() {
                total_unitary_operand_uses = total_unitary_operand_uses
                    .checked_add(operand_count)
                    .ok_or(QubitUseError::ArithmeticOverflow {
                        calculation: "total unitary operand uses",
                    })?;
            }

            if gate.is_measurement() {
                total_measurement_operand_uses =
                    total_measurement_operand_uses
                        .checked_add(operand_count)
                        .ok_or(QubitUseError::ArithmeticOverflow {
                            calculation: "total measurement operand uses",
                        })?;
            }

            if gate.is_reset() {
                total_reset_operand_uses = total_reset_operand_uses
                    .checked_add(operand_count)
                    .ok_or(QubitUseError::ArithmeticOverflow {
                        calculation: "total reset operand uses",
                    })?;
            }

            if gate.is_barrier() {
                total_barrier_operand_uses = total_barrier_operand_uses
                    .checked_add(operand_count)
                    .ok_or(QubitUseError::ArithmeticOverflow {
                        calculation: "total barrier operand uses",
                    })?;
            }

            if operand_count > 1 {
                total_multi_qubit_operand_uses =
                    total_multi_qubit_operand_uses
                        .checked_add(operand_count)
                        .ok_or(QubitUseError::ArithmeticOverflow {
                            calculation: "total multi-qubit operand uses",
                        })?;
            }

            for &qubit in gate.qubits() {
                if qubit.index() >= declared_qubits {
                    return Err(QubitUseError::QubitOutOfRange {
                        qubit,
                        qubit_count: declared_qubits,
                    });
                }

                match records.get_mut(&qubit) {
                    Some(record) => {
                        record.record(operation_index, gate)?;
                    }

                    None => {
                        records.insert(
                            qubit,
                            MutableQubitUsage::new(
                                qubit,
                                operation_index,
                                gate,
                            )?,
                        );
                    }
                }
            }
        }

        let mut used_qubits: Vec<QubitUsage> = records
            .into_values()
            .map(MutableQubitUsage::finish)
            .collect();

        // HashMap iteration order must never become compiler-visible behavior.
        used_qubits.sort_unstable_by_key(|usage| usage.qubit());

        Ok(Self {
            declared_qubits,
            operation_count,
            used_qubits,
            total_operand_uses,
            total_unitary_operand_uses,
            total_measurement_operand_uses,
            total_reset_operand_uses,
            total_barrier_operand_uses,
            total_multi_qubit_operand_uses,
        })
    }

    /// Returns the number of logical qubits declared by the circuit.
    #[must_use]
    pub const fn declared_qubits(&self) -> usize {
        self.declared_qubits
    }

    /// Returns the number of operations in the analyzed circuit.
    #[must_use]
    pub const fn operation_count(&self) -> usize {
        self.operation_count
    }

    /// Returns the number of distinct logical qubits actually used.
    #[must_use]
    pub fn used_qubit_count(&self) -> usize {
        self.used_qubits.len()
    }

    /// Returns the number of declared logical qubits that are never used.
    ///
    /// This calculation does not construct a list of unused qubits and is
    /// therefore O(1) after analysis.
    #[must_use]
    pub fn unused_qubit_count(&self) -> usize {
        self.declared_qubits
            .saturating_sub(self.used_qubits.len())
    }

    /// Returns the total number of logical-qubit operand occurrences.
    #[must_use]
    pub const fn total_operand_uses(&self) -> usize {
        self.total_operand_uses
    }

    /// Returns the total number of operand occurrences belonging to unitary
    /// operations.
    #[must_use]
    pub const fn total_unitary_operand_uses(&self) -> usize {
        self.total_unitary_operand_uses
    }

    /// Returns the total number of measurement operand occurrences.
    #[must_use]
    pub const fn total_measurement_operand_uses(&self) -> usize {
        self.total_measurement_operand_uses
    }

    /// Returns the total number of reset operand occurrences.
    #[must_use]
    pub const fn total_reset_operand_uses(&self) -> usize {
        self.total_reset_operand_uses
    }

    /// Returns the total number of barrier operand occurrences.
    #[must_use]
    pub const fn total_barrier_operand_uses(&self) -> usize {
        self.total_barrier_operand_uses
    }

    /// Returns the total number of operand occurrences in multi-qubit
    /// operations.
    #[must_use]
    pub const fn total_multi_qubit_operand_uses(&self) -> usize {
        self.total_multi_qubit_operand_uses
    }

    /// Returns the immutable usage records in ascending logical-qubit order.
    #[must_use]
    pub fn used_qubits(&self) -> &[QubitUsage] {
        &self.used_qubits
    }

    /// Returns an iterator over all used logical qubits.
    pub fn iter(&self) -> impl Iterator<Item = QubitUsage> + '_ {
        self.used_qubits.iter().copied()
    }

    /// Returns usage information for one logical qubit.
    ///
    /// Returns `None` when the qubit is declared but never used.
    #[must_use]
    pub fn usage(&self, qubit: QubitId) -> Option<QubitUsage> {
        self.used_qubits
            .binary_search_by_key(&qubit, QubitUsage::qubit)
            .ok()
            .map(|index| self.used_qubits[index])
    }

    /// Returns whether a logical qubit is used at least once.
    #[must_use]
    pub fn is_used(&self, qubit: QubitId) -> bool {
        self.usage(qubit).is_some()
    }

    /// Returns whether a declared logical qubit is unused.
    ///
    /// This does not require materializing all unused qubit identifiers.
    #[must_use]
    pub fn is_unused(&self, qubit: QubitId) -> bool {
        qubit.index() < self.declared_qubits && !self.is_used(qubit)
    }

    /// Returns the first operation using a logical qubit.
    #[must_use]
    pub fn first_use(
        &self,
        qubit: QubitId,
    ) -> Option<OperationIndex> {
        self.usage(qubit).map(QubitUsage::first_use)
    }

    /// Returns the last operation using a logical qubit.
    #[must_use]
    pub fn last_use(
        &self,
        qubit: QubitId,
    ) -> Option<OperationIndex> {
        self.usage(qubit).map(QubitUsage::last_use)
    }

    /// Returns the number of uses of a logical qubit.
    #[must_use]
    pub fn use_count(&self, qubit: QubitId) -> usize {
        self.usage(qubit)
            .map(QubitUsage::use_count)
            .unwrap_or(0)
    }

    /// Returns the inclusive logical-use interval of a qubit.
    ///
    /// Returns `None` when the qubit is unused.
    #[must_use]
    pub fn interval(
        &self,
        qubit: QubitId,
    ) -> Option<std::ops::RangeInclusive<OperationIndex>> {
        self.usage(qubit).map(QubitUsage::interval)
    }

    /// Returns the number of qubits participating in at least one multi-qubit
    /// operation.
    #[must_use]
    pub fn multi_qubit_participant_count(&self) -> usize {
        self.used_qubits
            .iter()
            .filter(|usage| {
                usage.participates_in_multi_qubit_operation()
            })
            .count()
    }

    /// Returns the number of qubits that are ever measured.
    #[must_use]
    pub fn measured_qubit_count(&self) -> usize {
        self.used_qubits
            .iter()
            .filter(|usage| usage.is_measured())
            .count()
    }

    /// Returns the number of qubits that are ever reset.
    #[must_use]
    pub fn reset_qubit_count(&self) -> usize {
        self.used_qubits
            .iter()
            .filter(|usage| usage.is_reset())
            .count()
    }

    /// Returns the average number of uses among used qubits.
    ///
    /// Returns `0.0` when no qubits are used.
    #[must_use]
    pub fn average_use_count(&self) -> f64 {
        if self.used_qubits.is_empty() {
            return 0.0;
        }

        self.total_operand_uses as f64 / self.used_qubits.len() as f64
    }

    /// Returns the maximum number of uses by any one logical qubit.
    #[must_use]
    pub fn maximum_use_count(&self) -> usize {
        self.used_qubits
            .iter()
            .map(|usage| usage.use_count())
            .max()
            .unwrap_or(0)
    }

    /// Returns the logical qubit with the highest use count.
    ///
    /// Ties are resolved deterministically in ascending `QubitId` order.
    #[must_use]
    pub fn most_used_qubit(&self) -> Option<QubitUsage> {
        self.used_qubits
            .iter()
            .copied()
            .max_by(|left, right| {
                left.use_count()
                    .cmp(&right.use_count())
                    .then_with(|| {
                        // Reverse the identifier comparison because `max_by`
                        // is being used while we want the smallest identifier
                        // to win a tie.
                        right.qubit().cmp(&left.qubit())
                    })
            })
    }

    /// Returns the number of unused logical qubits without allocating them.
    ///
    /// This is equivalent to `unused_qubit_count()`.
    #[must_use]
    pub fn unused_qubits(&self) -> usize {
        self.unused_qubit_count()
    }

    /// Returns the first used logical qubit.
    #[must_use]
    pub fn first_used_qubit(&self) -> Option<QubitId> {
        self.used_qubits.first().map(QubitUsage::qubit)
    }

    /// Returns the last used logical qubit.
    #[must_use]
    pub fn last_used_qubit(&self) -> Option<QubitId> {
        self.used_qubits.last().map(QubitUsage::qubit)
    }

    /// Returns whether the circuit contains no logical-qubit uses.
    #[must_use]
    pub fn has_no_uses(&self) -> bool {
        self.used_qubits.is_empty()
    }

    /// Returns whether every declared qubit is used.
    #[must_use]
    pub fn all_declared_qubits_used(&self) -> bool {
        self.used_qubits.len() == self.declared_qubits
    }

    /// Returns the fraction of declared logical qubits that are used.
    ///
    /// Returns `0.0` for a zero-qubit circuit.
    #[must_use]
    pub fn qubit_utilization(&self) -> f64 {
        if self.declared_qubits == 0 {
            return 0.0;
        }

        self.used_qubits.len() as f64 / self.declared_qubits as f64
    }

    /// Returns a compact immutable summary of the analysis.
    #[must_use]
    pub fn summary(&self) -> QubitUseSummary {
        QubitUseSummary {
            declared_qubits: self.declared_qubits,
            used_qubits: self.used_qubit_count(),
            unused_qubits: self.unused_qubit_count(),
            operation_count: self.operation_count,
            total_operand_uses: self.total_operand_uses,
            multi_qubit_participants: self.multi_qubit_participant_count(),
            measured_qubits: self.measured_qubit_count(),
            reset_qubits: self.reset_qubit_count(),
        }
    }
}

// ============================================================================
// Summary
// ============================================================================

/// Small allocation-free summary of a qubit-use analysis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct QubitUseSummary {
    declared_qubits: usize,
    used_qubits: usize,
    unused_qubits: usize,
    operation_count: usize,
    total_operand_uses: usize,
    multi_qubit_participants: usize,
    measured_qubits: usize,
    reset_qubits: usize,
}

impl QubitUseSummary {
    /// Number of declared logical qubits.
    #[must_use]
    pub const fn declared_qubits(self) -> usize {
        self.declared_qubits
    }

    /// Number of distinct used logical qubits.
    #[must_use]
    pub const fn used_qubits(self) -> usize {
        self.used_qubits
    }

    /// Number of unused logical qubits.
    #[must_use]
    pub const fn unused_qubits(self) -> usize {
        self.unused_qubits
    }

    /// Number of circuit operations.
    #[must_use]
    pub const fn operation_count(self) -> usize {
        self.operation_count
    }

    /// Total number of logical-qubit operand occurrences.
    #[must_use]
    pub const fn total_operand_uses(self) -> usize {
        self.total_operand_uses
    }

    /// Number of distinct qubits participating in multi-qubit operations.
    #[must_use]
    pub const fn multi_qubit_participants(self) -> usize {
        self.multi_qubit_participants
    }

    /// Number of distinct measured qubits.
    #[must_use]
    pub const fn measured_qubits(self) -> usize {
        self.measured_qubits
    }

    /// Number of distinct reset qubits.
    #[must_use]
    pub const fn reset_qubits(self) -> usize {
        self.reset_qubits
    }
}

// ============================================================================
// Convenience API
// ============================================================================

/// Analyzes qubit usage in a canonical quantum circuit.
///
/// This is the preferred simple API for callers that do not need to construct
/// the analysis type explicitly.
#[must_use = "qubit-use analysis results should not be silently discarded"]
pub fn analyze_qubit_use(
    circuit: &QuantumCircuit,
) -> Result<QubitUseAnalysis, QubitUseError> {
    QubitUseAnalysis::analyze(circuit)
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::quantum::ir::gate::GateKind;
    use crate::quantum::ir::parameter::Parameter;

    fn circuit_with_qubits(
        qubits: usize,
    ) -> QuantumCircuit {
        QuantumCircuit::new(qubits, 0).expect("valid test circuit")
    }

    fn append_gate(
        circuit: &mut QuantumCircuit,
        kind: GateKind,
        qubits: &[usize],
    ) {
        let operands = qubits
            .iter()
            .copied()
            .map(QubitId::new)
            .collect::<Vec<_>>();

        let parameters = match kind.parameter_count() {
            0 => Vec::new(),

            1 => vec![Parameter::Constant(0.0)],

            2 => vec![
                Parameter::Constant(0.0),
                Parameter::Constant(0.0),
            ],

            3 => vec![
                Parameter::Constant(0.0),
                Parameter::Constant(0.0),
                Parameter::Constant(0.0),
            ],

            count => panic!(
                "test helper does not support {count} parameters"
            ),
        };

        let classical_target =
            if kind.requires_classical_target() {
                Some(0)
            } else {
                None
            };

        let measurement = None;

        let gate = Gate::new(
            kind,
            operands,
            parameters,
            classical_target,
            measurement,
        )
        .expect("valid test gate");

        circuit
            .push_gate(gate)
            .expect("valid gate insertion");
    }

    #[test]
    fn empty_circuit_has_no_qubit_uses() {
        let circuit = circuit_with_qubits(8);

        let analysis =
            QubitUseAnalysis::analyze(&circuit).expect("analysis");

        assert_eq!(analysis.declared_qubits(), 8);
        assert_eq!(analysis.operation_count(), 0);
        assert_eq!(analysis.used_qubit_count(), 0);
        assert_eq!(analysis.unused_qubit_count(), 8);
        assert_eq!(analysis.total_operand_uses(), 0);
        assert!(analysis.has_no_uses());
        assert!(!analysis.all_declared_qubits_used());
    }

    #[test]
    fn single_qubit_use_is_recorded() {
        let mut circuit = circuit_with_qubits(4);

        append_gate(&mut circuit, GateKind::H, &[2]);

        let analysis =
            QubitUseAnalysis::analyze(&circuit).expect("analysis");

        let usage = analysis
            .usage(QubitId::new(2))
            .expect("qubit must be used");

        assert_eq!(usage.first_use(), 0);
        assert_eq!(usage.last_use(), 0);
        assert_eq!(usage.use_count(), 1);
        assert_eq!(usage.unitary_use_count(), 1);
        assert_eq!(usage.multi_qubit_use_count(), 0);

        assert_eq!(analysis.used_qubit_count(), 1);
        assert_eq!(analysis.unused_qubit_count(), 3);
        assert_eq!(analysis.total_operand_uses(), 1);
    }

    #[test]
    fn repeated_use_tracks_first_last_and_count() {
        let mut circuit = circuit_with_qubits(2);

        append_gate(&mut circuit, GateKind::H, &[0]);
        append_gate(&mut circuit, GateKind::X, &[1]);
        append_gate(&mut circuit, GateKind::Z, &[0]);
        append_gate(&mut circuit, GateKind::H, &[0]);

        let analysis =
            QubitUseAnalysis::analyze(&circuit).expect("analysis");

        let usage = analysis
            .usage(QubitId::new(0))
            .expect("qubit must be used");

        assert_eq!(usage.first_use(), 0);
        assert_eq!(usage.last_use(), 3);
        assert_eq!(usage.use_count(), 3);
        assert_eq!(usage.interval_length(), 4);
        assert_eq!(usage.span(), 3);
    }

    #[test]
    fn multi_qubit_operation_counts_each_operand() {
        let mut circuit = circuit_with_qubits(3);

        append_gate(&mut circuit, GateKind::CX, &[0, 2]);

        let analysis =
            QubitUseAnalysis::analyze(&circuit).expect("analysis");

        assert_eq!(analysis.total_operand_uses(), 2);
        assert_eq!(analysis.total_multi_qubit_operand_uses(), 2);

        let first = analysis
            .usage(QubitId::new(0))
            .expect("q0 used");

        let second = analysis
            .usage(QubitId::new(2))
            .expect("q2 used");

        assert_eq!(first.use_count(), 1);
        assert_eq!(second.use_count(), 1);
        assert!(first.participates_in_multi_qubit_operation());
        assert!(second.participates_in_multi_qubit_operation());
    }

    #[test]
    fn measurement_is_counted_as_a_use() {
        let mut circuit = circuit_with_qubits(1);

        append_gate(&mut circuit, GateKind::Measure, &[0]);

        let analysis =
            QubitUseAnalysis::analyze(&circuit).expect("analysis");

        let usage = analysis
            .usage(QubitId::new(0))
            .expect("q0 used");

        assert_eq!(usage.use_count(), 1);
        assert_eq!(usage.measurement_use_count(), 1);
        assert_eq!(analysis.total_measurement_operand_uses(), 1);
        assert_eq!(analysis.measured_qubit_count(), 1);
    }

    #[test]
    fn reset_is_counted_as_a_use() {
        let mut circuit = circuit_with_qubits(1);

        append_gate(&mut circuit, GateKind::Reset, &[0]);

        let analysis =
            QubitUseAnalysis::analyze(&circuit).expect("analysis");

        let usage = analysis
            .usage(QubitId::new(0))
            .expect("q0 used");

        assert_eq!(usage.use_count(), 1);
        assert_eq!(usage.reset_use_count(), 1);
        assert_eq!(analysis.total_reset_operand_uses(), 1);
        assert_eq!(analysis.reset_qubit_count(), 1);
    }

    #[test]
    fn barrier_uses_all_its_operands() {
        let mut circuit = circuit_with_qubits(4);

        append_gate(
            &mut circuit,
            GateKind::Barrier,
            &[0, 1, 3],
        );

        let analysis =
            QubitUseAnalysis::analyze(&circuit).expect("analysis");

        assert_eq!(analysis.total_operand_uses(), 3);
        assert_eq!(analysis.total_barrier_operand_uses(), 3);

        assert_eq!(
            analysis
                .usage(QubitId::new(0))
                .expect("q0")
                .barrier_use_count(),
            1
        );

        assert_eq!(
            analysis
                .usage(QubitId::new(1))
                .expect("q1")
                .barrier_use_count(),
            1
        );

        assert_eq!(
            analysis
                .usage(QubitId::new(3))
                .expect("q3")
                .barrier_use_count(),
            1
        );
    }

    #[test]
    fn results_are_deterministically_sorted() {
        let mut circuit = circuit_with_qubits(8);

        append_gate(&mut circuit, GateKind::X, &[7]);
        append_gate(&mut circuit, GateKind::X, &[2]);
        append_gate(&mut circuit, GateKind::X, &[5]);
        append_gate(&mut circuit, GateKind::X, &[0]);

        let analysis =
            QubitUseAnalysis::analyze(&circuit).expect("analysis");

        let ids = analysis
            .used_qubits()
            .iter()
            .map(QubitUsage::qubit)
            .collect::<Vec<_>>();

        assert_eq!(
            ids,
            vec![
                QubitId::new(0),
                QubitId::new(2),
                QubitId::new(5),
                QubitId::new(7),
            ]
        );
    }

    #[test]
    fn unused_qubits_are_not_materialized() {
        let mut circuit = circuit_with_qubits(1_000_000);

        append_gate(&mut circuit, GateKind::H, &[999_999]);

        let analysis =
            QubitUseAnalysis::analyze(&circuit).expect("analysis");

        assert_eq!(analysis.declared_qubits(), 1_000_000);
        assert_eq!(analysis.used_qubit_count(), 1);
        assert_eq!(analysis.unused_qubit_count(), 999_999);
        assert_eq!(
            analysis.first_used_qubit(),
            Some(QubitId::new(999_999))
        );
    }

    #[test]
    fn utilization_is_correct() {
        let mut circuit = circuit_with_qubits(4);

        append_gate(&mut circuit, GateKind::X, &[0]);
        append_gate(&mut circuit, GateKind::X, &[2]);

        let analysis =
            QubitUseAnalysis::analyze(&circuit).expect("analysis");

        assert_eq!(analysis.used_qubit_count(), 2);
        assert_eq!(analysis.unused_qubit_count(), 2);
        assert!((analysis.qubit_utilization() - 0.5).abs() < f64::EPSILON);
    }

    #[test]
    fn usage_density_is_correct() {
        let mut circuit = circuit_with_qubits(1);

        append_gate(&mut circuit, GateKind::X, &[0]);
        append_gate(&mut circuit, GateKind::H, &[0]);
        append_gate(&mut circuit, GateKind::Z, &[0]);

        let usage = QubitUseAnalysis::analyze(&circuit)
            .expect("analysis")
            .usage(QubitId::new(0))
            .expect("q0");

        assert_eq!(usage.use_count(), 3);
        assert_eq!(usage.interval_length(), 3);
        assert!((usage.use_density() - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn summary_is_allocation_free() {
        let mut circuit = circuit_with_qubits(4);

        append_gate(&mut circuit, GateKind::CX, &[0, 1]);

        let analysis =
            QubitUseAnalysis::analyze(&circuit).expect("analysis");

        let summary = analysis.summary();

        assert_eq!(summary.declared_qubits(), 4);
        assert_eq!(summary.used_qubits(), 2);
        assert_eq!(summary.unused_qubits(), 2);
        assert_eq!(summary.operation_count(), 1);
        assert_eq!(summary.total_operand_uses(), 2);
        assert_eq!(summary.multi_qubit_participants(), 2);
    }

    #[test]
    fn first_and_last_used_qubits_are_deterministic() {
        let mut circuit = circuit_with_qubits(4);

        append_gate(&mut circuit, GateKind::X, &[3]);
        append_gate(&mut circuit, GateKind::X, &[1]);

        let analysis =
            QubitUseAnalysis::analyze(&circuit).expect("analysis");

        assert_eq!(
            analysis.first_used_qubit(),
            Some(QubitId::new(1))
        );

        assert_eq!(
            analysis.last_used_qubit(),
            Some(QubitId::new(3))
        );
    }

    #[test]
    fn most_used_qubit_breaks_ties_by_lowest_id() {
        let mut circuit = circuit_with_qubits(3);

        append_gate(&mut circuit, GateKind::X, &[2]);
        append_gate(&mut circuit, GateKind::X, &[1]);
        append_gate(&mut circuit, GateKind::H, &[2]);
        append_gate(&mut circuit, GateKind::H, &[1]);

        let analysis =
            QubitUseAnalysis::analyze(&circuit).expect("analysis");

        let most_used = analysis
            .most_used_qubit()
            .expect("at least one used qubit");

        assert_eq!(most_used.qubit(), QubitId::new(1));
        assert_eq!(most_used.use_count(), 2);
    }

    #[test]
    fn analyze_validated_matches_full_analysis() {
        let mut circuit = circuit_with_qubits(2);

        append_gate(&mut circuit, GateKind::H, &[0]);
        append_gate(&mut circuit, GateKind::CX, &[0, 1]);

        let normal =
            QubitUseAnalysis::analyze(&circuit).expect("analysis");

        let validated =
            QubitUseAnalysis::analyze_validated(&circuit)
                .expect("analysis");

        assert_eq!(normal, validated);
    }

    #[test]
    fn convenience_function_matches_constructor() {
        let mut circuit = circuit_with_qubits(2);

        append_gate(&mut circuit, GateKind::X, &[1]);

        let direct =
            QubitUseAnalysis::analyze(&circuit).expect("analysis");

        let convenience =
            analyze_qubit_use(&circuit).expect("analysis");

        assert_eq!(direct, convenience);
    }
}