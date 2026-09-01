//! Zamani Quantum IR — Scalable Statistics
//!
//! `statistics.rs` owns deterministic, read-only statistical aggregation for
//! the canonical logical quantum IR.
//!
//! # Architectural boundary
//!
//! This module answers:
//!
//! > "What quantitative properties can be derived from the IR objects supplied
//! > to this statistics engine?"
//!
//! It does NOT:
//!
//! - optimize the program;
//! - rewrite operations;
//! - route logical qubits;
//! - allocate physical qubits;
//! - select hardware;
//! - schedule execution;
//! - perform calibration;
//! - synthesize pulses;
//! - execute a QPU;
//! - simulate amplitudes;
//! - decode QEC syndromes;
//! - estimate vendor-specific execution cost.
//!
//! Those responsibilities belong to other IR/compiler subsystems.
//!
//! # Why this is a separate module
//!
//! The parent `analysis` module is responsible for analysis orchestration.
//! This file owns statistical accounting only.
//!
//! The separation permits:
//!
//! ```text
//! circuit/program
//!       |
//!       v
//! analysis
//!       |
//!       +----> statistics
//!       |
//!       +----> dependency analysis
//!       |
//!       +----> liveness
//!       |
//!       +----> resource usage
//!       |
//!       +----> properties
//! ```
//!
//! Statistics therefore do not become coupled to future analysis passes.
//!
//! # Scalability
//!
//! The implementation deliberately does NOT allocate storage proportional to
//! the declared number of logical qubits.
//!
//! It is sparse:
//!
//! ```text
//! declared qubits = N
//!
//! statistics memory ~= actually referenced qubits
//! ```
//!
//! Therefore a circuit declaring a very large logical namespace but touching
//! only a small subset does not force allocation for the entire namespace.
//!
//! No architectural qubit-count, register-size, gate-count, or machine-size
//! ceiling is encoded here.
//!
//! The practical upper bound is determined by:
//!
//! - the size of the supplied IR;
//! - the host's available resources;
//! - explicit `QuantumIrLimits` imposed by the caller;
//! - `usize` representational capacity on the host.
//!
//! `usize` is used only for host collection/count representation. It is NOT
//! used as a semantic qubit identity. Logical qubit identity remains the
//! canonical `quantum::ir::qubit::QubitId`.
//!
//! # Determinism
//!
//! This module guarantees deterministic results for identical ordered input:
//!
//! - qubit usage is sorted by `QubitId`;
//! - classical usage is sorted by classical-bit index;
//! - gate histogram order is first-seen order;
//! - all counters use checked arithmetic;
//! - no hash-map iteration order is exposed;
//! - no global mutable state exists.
//!
//! # Streaming / incremental operation
//!
//! `StatisticsAccumulator` allows callers to process operations incrementally:
//!
//! ```text
//! operation 1 -> accumulator
//! operation 2 -> accumulator
//! operation 3 -> accumulator
//! ...
//! operation N -> accumulator
//!
//! finalize()
//! ```
//!
//! This prevents consumers from needing to duplicate the statistical
//! classification logic and allows future streaming/program-region analysis.
//!
//! # Memory safety
//!
//! This module:
//!
//! - contains no `unsafe`;
//! - has `#![forbid(unsafe_code)]`;
//! - uses checked integer arithmetic;
//! - uses fallible collection reservation where dynamic allocation occurs;
//! - never indexes a dense qubit array by logical qubit identity;
//! - never trusts a declared qubit count as an allocation size.
//!
//! # Integration
//!
//! This module consumes the existing canonical gate API:
//!
//! - `Gate::qubits()`;
//! - `Gate::qubit_count()`;
//! - `Gate::kind()`;
//! - `Gate::is_measurement()`;
//! - `Gate::is_barrier()`;
//! - `Gate::is_reset()`;
//! - `Gate::is_parameterized()`;
//! - `Gate::is_unitary()`;
//! - `Gate::classical_target()`.
//!
//! Logical qubit identities are always taken from:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! ```
//!
//! It does not import the historical `qubits` module.
//!
//! # Rust compatibility
//!
//! Target:
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
//! - aggregate statistical structures;
//! - deterministic gate histograms;
//! - sparse logical-qubit usage;
//! - sparse classical destination usage;
//! - streaming statistical accumulation;
//! - statistical arithmetic errors.
//!
//! This file does NOT own:
//!
//! - `Gate`;
//! - `QubitId`;
//! - circuit construction;
//! - circuit validation;
//! - IR resource limits;
//! - operation semantics.
//!
//! Those concepts remain owned by their canonical modules.

#![forbid(unsafe_code)]

use std::collections::BTreeMap;
use std::fmt;

use super::super::gate::{Gate, GateKind};
use super::super::qubit::QubitId;

// =============================================================================
// Public error
// =============================================================================

/// Errors produced by statistical aggregation.
///
/// Statistics never silently saturate counters. Overflow is reported
/// explicitly so corrupted or adversarial IR cannot produce misleading
/// measurements.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatisticsError {
    /// A counter could not be incremented without overflowing `usize`.
    CounterOverflow {
        /// The semantic counter that overflowed.
        counter: &'static str,
    },

    /// Dynamic storage could not be reserved.
    AllocationFailure {
        /// The collection for which reservation failed.
        collection: &'static str,

        /// Requested additional capacity.
        additional: usize,
    },

    /// A supplied operation has no logical qubit operands.
    ///
    /// This is intentionally reported by the classifier only when the caller
    /// explicitly requests operation-arity statistics that require at least
    /// one quantum operand.
    ZeroQuantumOperands,
}

impl fmt::Display for StatisticsError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CounterOverflow { counter } => {
                write!(
                    formatter,
                    "statistics counter overflow: {counter}"
                )
            }

            Self::AllocationFailure {
                collection,
                additional,
            } => {
                write!(
                    formatter,
                    "unable to reserve {additional} additional entries for statistics collection `{collection}`"
                )
            }

            Self::ZeroQuantumOperands => {
                write!(
                    formatter,
                    "operation contains zero logical quantum operands"
                )
            }
        }
    }
}

impl std::error::Error for StatisticsError {}

/// Result type used by this module.
pub type StatisticsResult<T> = Result<T, StatisticsError>;

// =============================================================================
// Gate histogram
// =============================================================================

/// Count for one semantic [`GateKind`].
///
/// Histogram entries retain first-seen order rather than enum order. This is
/// useful for deterministic human-readable reports while remaining independent
/// of any particular gate enumeration ordering.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GateKindCount {
    kind: GateKind,
    count: usize,
}

impl GateKindCount {
    fn new(kind: GateKind) -> Self {
        Self { kind, count: 1 }
    }

    fn increment(&mut self) -> StatisticsResult<()> {
        self.count = checked_increment(
            self.count,
            "gate histogram count",
        )?;

        Ok(())
    }

    /// Returns the semantic gate kind.
    #[must_use]
    pub const fn kind(&self) -> GateKind {
        self.kind
    }

    /// Returns the number of occurrences.
    #[must_use]
    pub const fn count(&self) -> usize {
        self.count
    }
}

// =============================================================================
// Logical qubit usage
// =============================================================================

/// Statistical usage information for one logical qubit.
///
/// The identity is the canonical `quantum::ir::qubit::QubitId`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QubitUsage {
    qubit: QubitId,
    operation_count: usize,
}

impl QubitUsage {
    /// Returns the logical qubit identity.
    #[must_use]
    pub const fn qubit(&self) -> QubitId {
        self.qubit
    }

    /// Returns the number of operations referencing the qubit.
    #[must_use]
    pub const fn operation_count(&self) -> usize {
        self.operation_count
    }
}

// =============================================================================
// Classical usage
// =============================================================================

/// Statistical usage information for one classical destination bit.
///
/// Classical bit identity remains represented using the existing IR's
/// classical-bit index at this layer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClassicalBitUsage {
    bit: usize,
    measurement_count: usize,
}

impl ClassicalBitUsage {
    /// Returns the classical-bit index.
    #[must_use]
    pub const fn bit(&self) -> usize {
        self.bit
    }

    /// Returns the number of measurements targeting the bit.
    #[must_use]
    pub const fn measurement_count(&self) -> usize {
        self.measurement_count
    }
}

// =============================================================================
// Complete statistics
// =============================================================================

/// Complete deterministic statistics for an ordered logical quantum
/// operation sequence.
///
/// These statistics describe the supplied IR sequence, not any physical
/// hardware target.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Statistics {
    operation_count: usize,
    logical_depth: usize,

    quantum_operations: usize,
    single_qubit_operations: usize,
    two_qubit_operations: usize,
    multi_qubit_operations: usize,

    measurement_count: usize,
    barrier_count: usize,
    reset_count: usize,

    parameterized_operations: usize,
    unitary_operations: usize,
    non_unitary_operations: usize,

    distinct_qubits: usize,
    distinct_classical_bits: usize,

    gate_histogram: Vec<GateKindCount>,
}

impl Statistics {
    fn from_accumulator(accumulator: StatisticsAccumulator) -> Self {
        let distinct_qubits = accumulator.qubit_usage.len();
        let distinct_classical_bits = accumulator.classical_usage.len();

        Self {
            operation_count: accumulator.operation_count,
            logical_depth: accumulator.logical_depth,
            quantum_operations: accumulator.quantum_operations,
            single_qubit_operations: accumulator.single_qubit_operations,
            two_qubit_operations: accumulator.two_qubit_operations,
            multi_qubit_operations: accumulator.multi_qubit_operations,
            measurement_count: accumulator.measurement_count,
            barrier_count: accumulator.barrier_count,
            reset_count: accumulator.reset_count,
            parameterized_operations: accumulator.parameterized_operations,
            unitary_operations: accumulator.unitary_operations,
            non_unitary_operations: accumulator.non_unitary_operations,
            distinct_qubits,
            distinct_classical_bits,
            gate_histogram: accumulator.gate_histogram,
        }
    }

    /// Number of operations processed.
    #[must_use]
    pub const fn operation_count(&self) -> usize {
        self.operation_count
    }

    /// Logical dependency depth.
    ///
    /// This is IR depth, not physical wall-clock execution latency.
    #[must_use]
    pub const fn logical_depth(&self) -> usize {
        self.logical_depth
    }

    /// Number of operations containing at least one logical quantum operand.
    #[must_use]
    pub const fn quantum_operations(&self) -> usize {
        self.quantum_operations
    }

    /// Number of single-qubit operations.
    #[must_use]
    pub const fn single_qubit_operations(&self) -> usize {
        self.single_qubit_operations
    }

    /// Number of two-qubit operations.
    #[must_use]
    pub const fn two_qubit_operations(&self) -> usize {
        self.two_qubit_operations
    }

    /// Number of operations involving three or more logical qubits.
    #[must_use]
    pub const fn multi_qubit_operations(&self) -> usize {
        self.multi_qubit_operations
    }

    /// Number of measurements.
    #[must_use]
    pub const fn measurement_count(&self) -> usize {
        self.measurement_count
    }

    /// Number of barriers.
    #[must_use]
    pub const fn barrier_count(&self) -> usize {
        self.barrier_count
    }

    /// Number of resets.
    #[must_use]
    pub const fn reset_count(&self) -> usize {
        self.reset_count
    }

    /// Number of parameterized operations.
    #[must_use]
    pub const fn parameterized_operations(&self) -> usize {
        self.parameterized_operations
    }

    /// Number of unitary operations.
    #[must_use]
    pub const fn unitary_operations(&self) -> usize {
        self.unitary_operations
    }

    /// Number of non-unitary operations.
    #[must_use]
    pub const fn non_unitary_operations(&self) -> usize {
        self.non_unitary_operations
    }

    /// Number of distinct logical qubits actually referenced.
    #[must_use]
    pub const fn distinct_qubits(&self) -> usize {
        self.distinct_qubits
    }

    /// Number of distinct classical destination bits actually referenced.
    #[must_use]
    pub const fn distinct_classical_bits(&self) -> usize {
        self.distinct_classical_bits
    }

    /// Returns the deterministic gate histogram.
    #[must_use]
    pub fn gate_histogram(&self) -> &[GateKindCount] {
        &self.gate_histogram
    }

    /// Returns the count for one gate kind.
    #[must_use]
    pub fn gate_count(&self, kind: GateKind) -> usize {
        self.gate_histogram
            .iter()
            .find(|entry| entry.kind == kind)
            .map_or(0, GateKindCount::count)
    }

    /// Returns the logical-qubit utilization ratio.
    ///
    /// `None` is returned when `declared_qubits == 0`.
    ///
    /// This method does not store the declared qubit count because statistics
    /// can also be accumulated over a standalone operation stream.
    #[must_use]
    pub fn qubit_utilization(
        &self,
        declared_qubits: usize,
    ) -> Option<f64> {
        if declared_qubits == 0 {
            None
        } else {
            Some(
                self.distinct_qubits as f64
                    / declared_qubits as f64,
            )
        }
    }
}

// =============================================================================
// Streaming accumulator
// =============================================================================

/// Incremental statistics accumulator.
///
/// This is the preferred primitive for large programs because callers can
/// feed operations incrementally without constructing another intermediate
/// operation collection.
///
/// The accumulator is deterministic for a deterministic input order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatisticsAccumulator {
    operation_count: usize,
    logical_depth: usize,

    quantum_operations: usize,
    single_qubit_operations: usize,
    two_qubit_operations: usize,
    multi_qubit_operations: usize,

    measurement_count: usize,
    barrier_count: usize,
    reset_count: usize,

    parameterized_operations: usize,
    unitary_operations: usize,
    non_unitary_operations: usize,

    qubit_usage: BTreeMap<QubitId, usize>,
    classical_usage: BTreeMap<usize, usize>,

    gate_histogram: Vec<GateKindCount>,

    /// Last logical depth associated with every actually referenced qubit.
    ///
    /// This is sparse and therefore does not allocate according to the
    /// declared quantum namespace.
    qubit_depth: BTreeMap<QubitId, usize>,
}

impl StatisticsAccumulator {
    /// Creates an empty statistics accumulator.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            operation_count: 0,
            logical_depth: 0,

            quantum_operations: 0,
            single_qubit_operations: 0,
            two_qubit_operations: 0,
            multi_qubit_operations: 0,

            measurement_count: 0,
            barrier_count: 0,
            reset_count: 0,

            parameterized_operations: 0,
            unitary_operations: 0,
            non_unitary_operations: 0,

            qubit_usage: BTreeMap::new(),
            classical_usage: BTreeMap::new(),

            gate_histogram: Vec::new(),

            qubit_depth: BTreeMap::new(),
        }
    }

    /// Returns the number of operations already accumulated.
    #[must_use]
    pub const fn operation_count(&self) -> usize {
        self.operation_count
    }

    /// Adds one operation to the statistics.
    ///
    /// The operation is borrowed and is never modified.
    ///
    /// All counters and depth calculations are checked.
    pub fn observe(
        &mut self,
        gate: &Gate,
    ) -> StatisticsResult<()> {
        self.classify_arity(gate)?;
        self.classify_semantics(gate)?;
        self.observe_qubits(gate)?;
        self.observe_classical_destination(gate)?;
        self.observe_gate_kind(gate)?;
        self.observe_depth(gate)?;

        self.operation_count = checked_increment(
            self.operation_count,
            "operation count",
        )?;

        Ok(())
    }

    /// Adds all operations from an iterator.
    ///
    /// This is intentionally iterator-based so callers can provide slices,
    /// vectors, streaming adapters, or other operation sources without this
    /// module owning the source collection.
    pub fn observe_iter<'a, I>(
        &mut self,
        operations: I,
    ) -> StatisticsResult<()>
    where
        I: IntoIterator<Item = &'a Gate>,
    {
        for gate in operations {
            self.observe(gate)?;
        }

        Ok(())
    }

    /// Finalizes the accumulator into immutable statistics.
    #[must_use]
    pub fn finish(self) -> Statistics {
        Statistics::from_accumulator(self)
    }

    /// Clears all accumulated state.
    ///
    /// This allows one accumulator instance to be reused for independent
    /// analysis jobs.
    pub fn clear(&mut self) {
        *self = Self::new();
    }

    fn classify_arity(
        &mut self,
        gate: &Gate,
    ) -> StatisticsResult<()> {
        let operand_count = gate.qubit_count();

        if operand_count == 0 {
            return Err(StatisticsError::ZeroQuantumOperands);
        }

        self.quantum_operations = checked_increment(
            self.quantum_operations,
            "quantum operation count",
        )?;

        match operand_count {
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

        Ok(())
    }

    fn classify_semantics(
        &mut self,
        gate: &Gate,
    ) -> StatisticsResult<()> {
        if gate.is_measurement() {
            self.measurement_count = checked_increment(
                self.measurement_count,
                "measurement count",
            )?;
        }

        if gate.is_barrier() {
            self.barrier_count = checked_increment(
                self.barrier_count,
                "barrier count",
            )?;
        }

        if gate.is_reset() {
            self.reset_count = checked_increment(
                self.reset_count,
                "reset count",
            )?;
        }

        if gate.is_parameterized() {
            self.parameterized_operations =
                checked_increment(
                    self.parameterized_operations,
                    "parameterized operation count",
                )?;
        }

        if gate.is_unitary() {
            self.unitary_operations =
                checked_increment(
                    self.unitary_operations,
                    "unitary operation count",
                )?;
        } else {
            self.non_unitary_operations =
                checked_increment(
                    self.non_unitary_operations,
                    "non-unitary operation count",
                )?;
        }

        Ok(())
    }

    fn observe_qubits(
        &mut self,
        gate: &Gate,
    ) -> StatisticsResult<()> {
        for &qubit in gate.qubits() {
            let entry = self
                .qubit_usage
                .entry(qubit)
                .or_insert(0);

            *entry = checked_increment(
                *entry,
                "per-qubit operation count",
            )?;
        }

        Ok(())
    }

    fn observe_classical_destination(
        &mut self,
        gate: &Gate,
    ) -> StatisticsResult<()> {
        if let Some(bit) = gate.classical_target() {
            let entry = self
                .classical_usage
                .entry(bit)
                .or_insert(0);

            *entry = checked_increment(
                *entry,
                "per-classical-bit measurement count",
            )?;
        }

        Ok(())
    }

    fn observe_gate_kind(
        &mut self,
        kind: GateKind,
    ) -> StatisticsResult<()> {
        if let Some(entry) = self
            .gate_histogram
            .iter_mut()
            .find(|entry| entry.kind == kind)
        {
            return entry.increment();
        }

        self.gate_histogram
            .try_reserve(1)
            .map_err(|_| StatisticsError::AllocationFailure {
                collection: "gate histogram",
                additional: 1,
            })?;

        self.gate_histogram
            .push(GateKindCount::new(kind));

        Ok(())
    }

    fn observe_depth(
        &mut self,
        gate: &Gate,
    ) -> StatisticsResult<()> {
        let mut predecessor_depth = 0usize;

        for &qubit in gate.qubits() {
            if let Some(&depth) =
                self.qubit_depth.get(&qubit)
            {
                predecessor_depth =
                    predecessor_depth.max(depth);
            }
        }

        let operation_depth = checked_increment(
            predecessor_depth,
            "logical circuit depth",
        )?;

        for &qubit in gate.qubits() {
            self.qubit_depth
                .insert(qubit, operation_depth);
        }

        self.logical_depth =
            self.logical_depth.max(operation_depth);

        Ok(())
    }

    /// Produces deterministic logical-qubit usage statistics.
    ///
    /// The result is sorted by ascending `QubitId`.
    pub fn qubit_usage(
        &self,
    ) -> StatisticsResult<Vec<QubitUsage>> {
        let mut result = Vec::new();

        result
            .try_reserve(self.qubit_usage.len())
            .map_err(|_| StatisticsError::AllocationFailure {
                collection: "qubit usage result",
                additional: self.qubit_usage.len(),
            })?;

        for (&qubit, &operation_count) in
            &self.qubit_usage
        {
            result.push(QubitUsage {
                qubit,
                operation_count,
            });
        }

        Ok(result)
    }

    /// Produces deterministic classical-bit usage statistics.
    ///
    /// The result is sorted by ascending classical-bit index.
    pub fn classical_bit_usage(
        &self,
    ) -> StatisticsResult<Vec<ClassicalBitUsage>> {
        let mut result = Vec::new();

        result
            .try_reserve(self.classical_usage.len())
            .map_err(|_| StatisticsError::AllocationFailure {
                collection: "classical usage result",
                additional: self.classical_usage.len(),
            })?;

        for (&bit, &measurement_count) in
            &self.classical_usage
        {
            result.push(ClassicalBitUsage {
                bit,
                measurement_count,
            });
        }

        Ok(result)
    }
}

impl Default for StatisticsAccumulator {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Slice-level APIs
// =============================================================================

/// Calculates complete statistics for a borrowed operation slice.
///
/// No operation is cloned.
pub fn calculate(
    operations: &[Gate],
) -> StatisticsResult<Statistics> {
    let mut accumulator = StatisticsAccumulator::new();

    accumulator.observe_iter(operations)?;

    Ok(accumulator.finish())
}

/// Calculates complete statistics from any borrowed gate iterator.
///
/// This is useful when the caller does not own the operations as a contiguous
/// slice.
pub fn calculate_iter<'a, I>(
    operations: I,
) -> StatisticsResult<Statistics>
where
    I: IntoIterator<Item = &'a Gate>,
{
    let mut accumulator = StatisticsAccumulator::new();

    accumulator.observe_iter(operations)?;

    Ok(accumulator.finish())
}

/// Calculates compact statistics without retaining usage maps or a histogram.
///
/// This function is optimized for callers that only need aggregate counters.
pub fn calculate_basic(
    operations: &[Gate],
) -> StatisticsResult<BasicStatistics> {
    let mut operation_count = 0usize;
    let mut logical_depth = 0usize;

    let mut quantum_operations = 0usize;
    let mut single_qubit_operations = 0usize;
    let mut two_qubit_operations = 0usize;
    let mut multi_qubit_operations = 0usize;

    let mut measurement_count = 0usize;
    let mut barrier_count = 0usize;
    let mut reset_count = 0usize;

    let mut parameterized_operations = 0usize;
    let mut unitary_operations = 0usize;
    let mut non_unitary_operations = 0usize;

    let mut qubit_depth = BTreeMap::<QubitId, usize>::new();

    for gate in operations {
        let operand_count = gate.qubit_count();

        if operand_count == 0 {
            return Err(StatisticsError::ZeroQuantumOperands);
        }

        quantum_operations = checked_increment(
            quantum_operations,
            "quantum operation count",
        )?;

        match operand_count {
            1 => {
                single_qubit_operations =
                    checked_increment(
                        single_qubit_operations,
                        "single-qubit operation count",
                    )?;
            }

            2 => {
                two_qubit_operations =
                    checked_increment(
                        two_qubit_operations,
                        "two-qubit operation count",
                    )?;
            }

            _ => {
                multi_qubit_operations =
                    checked_increment(
                        multi_qubit_operations,
                        "multi-qubit operation count",
                    )?;
            }
        }

        if gate.is_measurement() {
            measurement_count = checked_increment(
                measurement_count,
                "measurement count",
            )?;
        }

        if gate.is_barrier() {
            barrier_count = checked_increment(
                barrier_count,
                "barrier count",
            )?;
        }

        if gate.is_reset() {
            reset_count = checked_increment(
                reset_count,
                "reset count",
            )?;
        }

        if gate.is_parameterized() {
            parameterized_operations =
                checked_increment(
                    parameterized_operations,
                    "parameterized operation count",
                )?;
        }

        if gate.is_unitary() {
            unitary_operations =
                checked_increment(
                    unitary_operations,
                    "unitary operation count",
                )?;
        } else {
            non_unitary_operations =
                checked_increment(
                    non_unitary_operations,
                    "non-unitary operation count",
                )?;
        }

        let mut predecessor_depth = 0usize;

        for &qubit in gate.qubits() {
            if let Some(&depth) =
                qubit_depth.get(&qubit)
            {
                predecessor_depth =
                    predecessor_depth.max(depth);
            }
        }

        let operation_depth = checked_increment(
            predecessor_depth,
            "logical circuit depth",
        )?;

        for &qubit in gate.qubits() {
            qubit_depth
                .insert(qubit, operation_depth);
        }

        logical_depth =
            logical_depth.max(operation_depth);

        operation_count = checked_increment(
            operation_count,
            "operation count",
        )?;
    }

    Ok(BasicStatistics {
        operation_count,
        logical_depth,
        quantum_operations,
        single_qubit_operations,
        two_qubit_operations,
        multi_qubit_operations,
        measurement_count,
        barrier_count,
        reset_count,
        parameterized_operations,
        unitary_operations,
        non_unitary_operations,
    })
}

// =============================================================================
// Compact statistics
// =============================================================================

/// Compact statistics for high-throughput callers.
///
/// This structure deliberately contains no per-qubit maps and no histogram.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BasicStatistics {
    operation_count: usize,
    logical_depth: usize,

    quantum_operations: usize,
    single_qubit_operations: usize,
    two_qubit_operations: usize,
    multi_qubit_operations: usize,

    measurement_count: usize,
    barrier_count: usize,
    reset_count: usize,

    parameterized_operations: usize,
    unitary_operations: usize,
    non_unitary_operations: usize,
}

impl BasicStatistics {
    /// Number of operations.
    #[must_use]
    pub const fn operation_count(&self) -> usize {
        self.operation_count
    }

    /// Logical dependency depth.
    #[must_use]
    pub const fn logical_depth(&self) -> usize {
        self.logical_depth
    }

    /// Number of quantum operations.
    #[must_use]
    pub const fn quantum_operations(&self) -> usize {
        self.quantum_operations
    }

    /// Number of single-qubit operations.
    #[must_use]
    pub const fn single_qubit_operations(&self) -> usize {
        self.single_qubit_operations
    }

    /// Number of two-qubit operations.
    #[must_use]
    pub const fn two_qubit_operations(&self) -> usize {
        self.two_qubit_operations
    }

    /// Number of multi-qubit operations.
    #[must_use]
    pub const fn multi_qubit_operations(&self) -> usize {
        self.multi_qubit_operations
    }

    /// Number of measurements.
    #[must_use]
    pub const fn measurement_count(&self) -> usize {
        self.measurement_count
    }

    /// Number of barriers.
    #[must_use]
    pub const fn barrier_count(&self) -> usize {
        self.barrier_count
    }

    /// Number of resets.
    #[must_use]
    pub const fn reset_count(&self) -> usize {
        self.reset_count
    }

    /// Number of parameterized operations.
    #[must_use]
    pub const fn parameterized_operations(&self) -> usize {
        self.parameterized_operations
    }

    /// Number of unitary operations.
    #[must_use]
    pub const fn unitary_operations(&self) -> usize {
        self.unitary_operations
    }

    /// Number of non-unitary operations.
    #[must_use]
    pub const fn non_unitary_operations(&self) -> usize {
        self.non_unitary_operations
    }
}

// =============================================================================
// Utility functions
// =============================================================================

fn checked_increment(
    value: usize,
    counter: &'static str,
) -> StatisticsResult<usize> {
    value
        .checked_add(1)
        .ok_or(StatisticsError::CounterOverflow {
            counter,
        })
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::quantum::ir::gate::Gate;
    use crate::quantum::ir::qubit::QubitId;

    fn gate(
        kind: GateKind,
        qubits: &[usize],
    ) -> Gate {
        let operands = qubits
            .iter()
            .copied()
            .map(QubitId::new)
            .collect::<Vec<_>>();

        Gate::simple(kind, operands)
            .expect("test gate must be valid")
    }

    #[test]
    fn empty_input_has_zero_statistics() {
        let statistics =
            calculate(&[]).expect("empty input must succeed");

        assert_eq!(statistics.operation_count(), 0);
        assert_eq!(statistics.logical_depth(), 0);
        assert_eq!(statistics.quantum_operations(), 0);
        assert_eq!(statistics.distinct_qubits(), 0);
        assert_eq!(statistics.gate_histogram().len(), 0);
    }

    #[test]
    fn independent_qubits_share_logical_depth() {
        let operations = vec![
            gate(GateKind::H, &[0]),
            gate(GateKind::H, &[1]),
        ];

        let statistics =
            calculate(&operations)
                .expect("statistics must succeed");

        assert_eq!(statistics.logical_depth(), 1);
        assert_eq!(statistics.operation_count(), 2);
    }

    #[test]
    fn dependent_operations_increase_depth() {
        let operations = vec![
            gate(GateKind::H, &[0]),
            gate(GateKind::X, &[0]),
        ];

        let statistics =
            calculate(&operations)
                .expect("statistics must succeed");

        assert_eq!(statistics.logical_depth(), 2);
    }

    #[test]
    fn two_qubit_operation_depends_on_both_operands() {
        let operations = vec![
            gate(GateKind::H, &[0]),
            gate(GateKind::H, &[1]),
            gate(GateKind::CX, &[0, 1]),
        ];

        let statistics =
            calculate(&operations)
                .expect("statistics must succeed");

        assert_eq!(statistics.logical_depth(), 2);
        assert_eq!(statistics.two_qubit_operations(), 1);
    }

    #[test]
    fn multi_qubit_operations_are_not_capped_at_three() {
        let operations = vec![
            gate(
                GateKind::CCX,
                &[0, 1, 2],
            ),
        ];

        let statistics =
            calculate(&operations)
                .expect("statistics must succeed");

        assert_eq!(statistics.multi_qubit_operations(), 1);
    }

    #[test]
    fn histogram_preserves_first_seen_order() {
        let operations = vec![
            gate(GateKind::X, &[0]),
            gate(GateKind::H, &[1]),
            gate(GateKind::X, &[2]),
            gate(GateKind::H, &[3]),
        ];

        let statistics =
            calculate(&operations)
                .expect("statistics must succeed");

        let histogram =
            statistics.gate_histogram();

        assert_eq!(histogram.len(), 2);

        assert_eq!(histogram[0].kind(), GateKind::X);
        assert_eq!(histogram[0].count(), 2);

        assert_eq!(histogram[1].kind(), GateKind::H);
        assert_eq!(histogram[1].count(), 2);
    }

    #[test]
    fn sparse_qubit_usage_is_sorted() {
        let operations = vec![
            gate(GateKind::X, &[100]),
            gate(GateKind::H, &[2]),
            gate(GateKind::CX, &[100, 2]),
        ];

        let mut accumulator =
            StatisticsAccumulator::new();

        accumulator
            .observe_iter(&operations)
            .expect("statistics must succeed");

        let usage = accumulator
            .qubit_usage()
            .expect("usage extraction must succeed");

        assert_eq!(
            usage,
            vec![
                QubitUsage {
                    qubit: QubitId::new(2),
                    operation_count: 2,
                },
                QubitUsage {
                    qubit: QubitId::new(100),
                    operation_count: 2,
                },
            ]
        );
    }

    #[test]
    fn qubit_namespace_is_not_dense() {
        let operations = vec![
            gate(GateKind::X, &[1_000_000]),
        ];

        let statistics =
            calculate(&operations)
                .expect("statistics must succeed");

        assert_eq!(statistics.distinct_qubits(), 1);
    }

    #[test]
    fn accumulator_can_be_reused() {
        let mut accumulator =
            StatisticsAccumulator::new();

        let first = vec![
            gate(GateKind::H, &[0]),
        ];

        accumulator
            .observe_iter(&first)
            .expect("first accumulation must succeed");

        assert_eq!(accumulator.operation_count(), 1);

        accumulator.clear();

        assert_eq!(accumulator.operation_count(), 0);

        let second = vec![
            gate(GateKind::X, &[1]),
        ];

        accumulator
            .observe_iter(&second)
            .expect("second accumulation must succeed");

        assert_eq!(accumulator.operation_count(), 1);

        let result = accumulator.finish();

        assert_eq!(result.gate_count(GateKind::X), 1);
        assert_eq!(result.gate_count(GateKind::H), 0);
    }

    #[test]
    fn basic_statistics_avoid_histogram_storage() {
        let operations = vec![
            gate(GateKind::H, &[0]),
            gate(GateKind::X, &[0]),
        ];

        let statistics =
            calculate_basic(&operations)
                .expect("basic statistics must succeed");

        assert_eq!(statistics.operation_count(), 2);
        assert_eq!(statistics.logical_depth(), 2);
        assert_eq!(statistics.single_qubit_operations(), 2);
    }

    #[test]
    fn gate_count_returns_zero_for_missing_kind() {
        let operations = vec![
            gate(GateKind::H, &[0]),
        ];

        let statistics =
            calculate(&operations)
                .expect("statistics must succeed");

        assert_eq!(statistics.gate_count(GateKind::CX), 0);
    }

    #[test]
    fn zero_operand_operation_is_rejected() {
        let result =
            Gate::simple(GateKind::Barrier, Vec::new());

        if let Ok(operation) = result {
            let statistics =
                calculate(std::slice::from_ref(&operation));

            assert_eq!(
                statistics,
                Err(StatisticsError::ZeroQuantumOperands)
            );
        }
    }
}