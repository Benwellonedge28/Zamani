//! Zamani Quantum IR — Circuit Analysis
//!
//! Production-grade, deterministic, read-only analysis for the canonical
//! logical quantum circuit representation.
//!
//! # Architectural role
//!
//! This module answers:
//!
//! > What structural and semantic properties can be determined from the
//! > circuit representation currently supplied to the analysis engine?
//!
//! It does NOT:
//!
//! - optimize;
//! - rewrite;
//! - route;
//! - allocate physical qubits;
//! - select a backend;
//! - select native instructions;
//! - calibrate;
//! - synthesize pulses;
//! - schedule physical execution;
//! - execute a QPU;
//! - simulate amplitudes;
//! - decode QEC syndromes;
//! - estimate vendor-specific execution cost.
//!
//! Those responsibilities belong to downstream subsystems.
//!
//! # Important architectural boundary
//!
//! `QuantumCircuit` is currently a gate-oriented circuit container.
//!
//! The universal Zamani Quantum IR is broader than `QuantumCircuit` and is
//! intended eventually to represent:
//!
//! - static circuits;
//! - dynamic circuits;
//! - classical control;
//! - pulse programs;
//! - analog/Hamiltonian programs;
//! - annealing/QUBO programs;
//! - logical/fault-tolerant programs;
//! - distributed quantum programs;
//! - future dialects/extensions.
//!
//! This file therefore intentionally analyzes the currently available
//! `QuantumCircuit` abstraction without making that abstraction the definition
//! of all quantum computation.
//!
//! Future analysis of `QuantumProgram`, regions, blocks, universal operations,
//! pulse objects, analog models, logical operations, or distributed resources
//! belongs in corresponding analysis modules under:
//!
//! ```text
//! quantum::ir::analysis
//! ```
//!
//! # Scalability
//!
//! The implementation does not allocate storage proportional to the declared
//! number of qubits.
//!
//! For example, declaring:
//!
//! ```text
//! 1_000_000 logical qubits
//! ```
//!
//! does not cause a one-million-entry analysis array to be allocated.
//!
//! Sparse structures are used instead:
//!
//! ```text
//! declared namespace
//!        │
//!        └── not materialized
//!
//! actual operands
//!        │
//!        └── sparse analysis state
//! ```
//!
//! Therefore memory usage for usage-oriented analyses is proportional to the
//! number of distinct resources actually referenced, rather than the size of
//! the declared namespace.
//!
//! There is no architectural maximum qubit count in this module.
//!
//! `QuantumIrLimits` is a resource/security policy for a particular invocation.
//! It is not the maximum size of a quantum machine supported by Zamani.
//!
//! # Determinism
//!
//! Public analysis results are deterministic:
//!
//! - logical qubit usage is ordered by `QubitId`;
//! - classical-bit usage is ordered by bit index;
//! - arbitrary-arity histograms are ordered by arity;
//! - gate-kind histograms preserve first-seen program order;
//! - no `HashMap` iteration order is exposed;
//! - no global mutable state exists.
//!
//! # Security
//!
//! A circuit may originate from an untrusted source, deserializer, generated
//! program, remote compilation service, or fuzzing system.
//!
//! Analysis therefore:
//!
//! - validates the supplied resource policy;
//! - validates circuit namespace sizes against the supplied policy;
//! - validates metadata size;
//! - validates operation count;
//! - validates every logical operand against the logical namespace;
//! - validates every classical destination against the classical namespace;
//! - checks analysis-work budgets before expensive analysis;
//! - uses checked arithmetic for counters and depth;
//! - uses fallible vector reservation where vectors are dynamically sized;
//! - never uses unchecked indexing;
//! - never uses `unsafe`.
//!
//! # Qubit identity
//!
//! The canonical logical-qubit identity is:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! ```
//!
//! This module deliberately imports:
//!
//! ```rust
//! use crate::quantum::ir::qubit::QubitId;
//! ```
//!
//! and never the historical `qubits` module.
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
//! - no unsafe code.
//!
//! `#![forbid(unsafe_code)]` makes the no-unsafe requirement compiler-enforced.
//!
//! # Integration contract
//!
//! ```text
//!                         ┌─────────────────────┐
//!                         │ QuantumCircuit      │
//!                         └──────────┬──────────┘
//!                                    │
//!                                    ▼
//!                         ┌─────────────────────┐
//!                         │ analysis::analysis  │
//!                         └──────────┬──────────┘
//!                                    │
//!              ┌─────────────────────┼────────────────────┐
//!              ▼                     ▼                    ▼
//!          statistics            usage                depth
//!              │                     │                    │
//!              └─────────────────────┼────────────────────┘
//!                                    ▼
//!                         downstream consumers
//! ```
//!
//! Downstream consumers may include:
//!
//! - optimization analysis;
//! - resource estimation;
//! - benchmarking;
//! - compiler diagnostics;
//! - visualization;
//! - reporting;
//! - scheduling;
//! - routing;
//! - hardware compatibility;
//! - compilation planning.
//!
//! They must consume these results without changing their semantic meaning.
//!
//! This module must not depend on those downstream systems.
//!
//! # File completion contract
//!
//! This file owns:
//!
//! - circuit-level analysis result types;
//! - deterministic circuit statistics;
//! - sparse qubit usage analysis;
//! - sparse classical destination usage analysis;
//! - arbitrary-arity operation statistics;
//! - gate-kind histogram analysis;
//! - logical dependency depth;
//! - analysis-work accounting;
//! - analysis-local arithmetic safety.
//!
//! This file does not own:
//!
//! - canonical qubit identity;
//! - gate definitions;
//! - circuit mutation;
//! - IR validation policy;
//! - resource-limit definitions;
//! - optimization algorithms;
//! - routing;
//! - scheduling;
//! - hardware information.
//!
//! Those concepts remain owned by their respective modules.
//!
//! # Future integration
//!
//! When the universal `Operation`/`QuantumProgram` analysis layer is complete,
//! it should be added beside this file, for example:
//!
//! ```text
//! analysis/
//! ├── mod.rs
//! ├── analysis.rs          <- this file
//! ├── program.rs           <- whole-program analysis
//! ├── operation.rs         <- universal operation analysis
//! ├── dependencies.rs      <- dependency analysis
//! ├── liveness.rs          <- liveness analysis
//! ├── resource_usage.rs    <- resource analysis
//! └── properties.rs        <- semantic properties
//! ```
//!
//! `analysis.rs` should then remain stable as the circuit-specific API.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::{BTreeMap, BTreeSet};

use crate::quantum::ir::circuit::{CircuitError, QuantumCircuit};
use crate::quantum::ir::gate::{Gate, GateKind};
use crate::quantum::ir::limits::QuantumIrLimits;
use crate::quantum::ir::qubit::QubitId;

// =============================================================================
// Public result types
// =============================================================================

/// Complete deterministic statistics for a logical quantum circuit.
///
/// These statistics describe the representation being analyzed. They do not
/// describe a physical machine, hardware latency, calibration, routing,
/// scheduling, or backend execution cost.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CircuitStatistics {
    qubits: usize,
    classical_bits: usize,
    operation_count: usize,

    /// Logical dependency depth.
    ///
    /// This is a structural circuit metric, not physical execution latency.
    depth: usize,

    measurement_count: usize,
    barrier_count: usize,
    reset_count: usize,

    /// Number of operations with exactly one quantum operand.
    single_qubit_operations: usize,

    /// Number of operations with exactly two quantum operands.
    two_qubit_operations: usize,

    /// Number of operations with three or more quantum operands.
    ///
    /// This field exists for compatibility and convenience. The canonical
    /// arity information is available through `arity_histogram()`.
    multi_qubit_operations: usize,

    parameterized_operations: usize,
    unitary_operations: usize,
    non_unitary_operations: usize,

    qubits_used: usize,
    classical_bits_used: usize,

    /// Deterministic operation-count histogram indexed by operand arity.
    arity_histogram: Vec<ArityCount>,

    /// Deterministic gate-kind histogram in first-seen program order.
    gate_histogram: Vec<GateKindCount>,
}

impl CircuitStatistics {
    fn new(
        qubits: usize,
        classical_bits: usize,
        operation_count: usize,
        depth: usize,
        measurement_count: usize,
        barrier_count: usize,
        reset_count: usize,
        single_qubit_operations: usize,
        two_qubit_operations: usize,
        multi_qubit_operations: usize,
        parameterized_operations: usize,
        unitary_operations: usize,
        non_unitary_operations: usize,
        qubits_used: usize,
        classical_bits_used: usize,
        arity_histogram: Vec<ArityCount>,
        gate_histogram: Vec<GateKindCount>,
    ) -> Self {
        Self {
            qubits,
            classical_bits,
            operation_count,
            depth,
            measurement_count,
            barrier_count,
            reset_count,
            single_qubit_operations,
            two_qubit_operations,
            multi_qubit_operations,
            parameterized_operations,
            unitary_operations,
            non_unitary_operations,
            qubits_used,
            classical_bits_used,
            arity_histogram,
            gate_histogram,
        }
    }

    /// Returns the number of declared logical qubits.
    #[must_use]
    pub const fn qubits(&self) -> usize {
        self.qubits
    }

    /// Returns the number of declared classical bits.
    #[must_use]
    pub const fn classical_bits(&self) -> usize {
        self.classical_bits
    }

    /// Returns the number of operations.
    #[must_use]
    pub const fn operation_count(&self) -> usize {
        self.operation_count
    }

    /// Returns logical dependency depth.
    ///
    /// This is not physical execution time.
    #[must_use]
    pub const fn depth(&self) -> usize {
        self.depth
    }

    /// Returns the number of measurement operations.
    #[must_use]
    pub const fn measurement_count(&self) -> usize {
        self.measurement_count
    }

    /// Returns the number of barrier operations.
    #[must_use]
    pub const fn barrier_count(&self) -> usize {
        self.barrier_count
    }

    /// Returns the number of reset operations.
    #[must_use]
    pub const fn reset_count(&self) -> usize {
        self.reset_count
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

    /// Returns the number of operations with three or more operands.
    #[must_use]
    pub const fn multi_qubit_operations(&self) -> usize {
        self.multi_qubit_operations
    }

    /// Returns the number of parameterized operations.
    #[must_use]
    pub const fn parameterized_operations(&self) -> usize {
        self.parameterized_operations
    }

    /// Returns the number of logically unitary operations.
    #[must_use]
    pub const fn unitary_operations(&self) -> usize {
        self.unitary_operations
    }

    /// Returns the number of logically non-unitary operations.
    #[must_use]
    pub const fn non_unitary_operations(&self) -> usize {
        self.non_unitary_operations
    }

    /// Returns the number of distinct logical qubits actually referenced.
    #[must_use]
    pub const fn qubits_used(&self) -> usize {
        self.qubits_used
    }

    /// Returns the number of distinct classical destinations actually used.
    #[must_use]
    pub const fn classical_bits_used(&self) -> usize {
        self.classical_bits_used
    }

    /// Returns deterministic operation counts grouped by operand arity.
    ///
    /// The vector is sorted by ascending arity.
    #[must_use]
    pub fn arity_histogram(&self) -> &[ArityCount] {
        &self.arity_histogram
    }

    /// Returns the number of operations with a particular quantum arity.
    #[must_use]
    pub fn operation_count_for_arity(&self, arity: usize) -> usize {
        self.arity_histogram
            .binary_search_by_key(&arity, |entry| entry.arity)
            .ok()
            .map_or(0, |index| self.arity_histogram[index].count)
    }

    /// Returns the deterministic gate-kind histogram.
    ///
    /// Entries preserve first-seen program order.
    #[must_use]
    pub fn gate_histogram(&self) -> &[GateKindCount] {
        &self.gate_histogram
    }

    /// Returns the number of occurrences of a gate kind.
    #[must_use]
    pub fn gate_count(&self, kind: GateKind) -> usize {
        self.gate_histogram
            .iter()
            .find(|entry| entry.kind == kind)
            .map_or(0, |entry| entry.count)
    }
}

/// Deterministic count for one operation arity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArityCount {
    arity: usize,
    count: usize,
}

impl ArityCount {
    fn new(arity: usize, count: usize) -> Self {
        Self { arity, count }
    }

    /// Returns the quantum operand arity.
    #[must_use]
    pub const fn arity(&self) -> usize {
        self.arity
    }

    /// Returns the number of operations with this arity.
    #[must_use]
    pub const fn count(&self) -> usize {
        self.count
    }
}

/// Deterministic count for one gate kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GateKindCount {
    kind: GateKind,
    count: usize,
}

impl GateKindCount {
    fn new(kind: GateKind, count: usize) -> Self {
        Self { kind, count }
    }

    /// Returns the gate kind.
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

/// Logical usage statistics for one logical qubit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QubitUsage {
    qubit: QubitId,
    operation_count: usize,
}

impl QubitUsage {
    /// Returns the canonical logical-qubit identity.
    #[must_use]
    pub const fn qubit(&self) -> QubitId {
        self.qubit
    }

    /// Returns the number of operations touching this qubit.
    #[must_use]
    pub const fn operation_count(&self) -> usize {
        self.operation_count
    }
}

/// Classical destination usage statistics.
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

    /// Returns the number of measurements targeting this bit.
    #[must_use]
    pub const fn measurement_count(&self) -> usize {
        self.measurement_count
    }
}

/// Compact statistics for callers that do not need usage maps or histograms.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BasicCircuitStatistics {
    operation_count: usize,
    depth: usize,
    measurement_count: usize,
    barrier_count: usize,
    reset_count: usize,
    parameterized_operations: usize,
    unitary_operations: usize,
    non_unitary_operations: usize,
}

impl BasicCircuitStatistics {
    /// Returns operation count.
    #[must_use]
    pub const fn operation_count(&self) -> usize {
        self.operation_count
    }

    /// Returns logical dependency depth.
    #[must_use]
    pub const fn depth(&self) -> usize {
        self.depth
    }

    /// Returns measurement count.
    #[must_use]
    pub const fn measurement_count(&self) -> usize {
        self.measurement_count
    }

    /// Returns barrier count.
    #[must_use]
    pub const fn barrier_count(&self) -> usize {
        self.barrier_count
    }

    /// Returns reset count.
    #[must_use]
    pub const fn reset_count(&self) -> usize {
        self.reset_count
    }

    /// Returns parameterized operation count.
    #[must_use]
    pub const fn parameterized_operations(&self) -> usize {
        self.parameterized_operations
    }

    /// Returns unitary operation count.
    #[must_use]
    pub const fn unitary_operations(&self) -> usize {
        self.unitary_operations
    }

    /// Returns non-unitary operation count.
    #[must_use]
    pub const fn non_unitary_operations(&self) -> usize {
        self.non_unitary_operations
    }
}

// =============================================================================
// Primary analysis
// =============================================================================

/// Performs complete deterministic circuit analysis using the circuit's
/// configured resource policy.
#[must_use = "analysis results should not be silently discarded"]
pub fn analyze(
    circuit: &QuantumCircuit,
) -> Result<CircuitStatistics, CircuitError> {
    analyze_with_limits(circuit, circuit.limits())
}

/// Performs complete deterministic circuit analysis using an explicit policy.
///
/// The circuit is never modified.
///
/// Analysis storage is proportional to actual referenced resources rather than
/// the declared logical namespace.
#[must_use = "analysis results should not be silently discarded"]
pub fn analyze_with_limits(
    circuit: &QuantumCircuit,
    limits: &QuantumIrLimits,
) -> Result<CircuitStatistics, CircuitError> {
    prepare_analysis(circuit, limits, AnalysisProfile::Complete)?;

    let mut measurement_count = 0usize;
    let mut barrier_count = 0usize;
    let mut reset_count = 0usize;

    let mut single_qubit_operations = 0usize;
    let mut two_qubit_operations = 0usize;
    let mut multi_qubit_operations = 0usize;

    let mut parameterized_operations = 0usize;
    let mut unitary_operations = 0usize;
    let mut non_unitary_operations = 0usize;

    let mut qubit_usage = BTreeMap::<QubitId, usize>::new();
    let mut classical_usage = BTreeMap::<usize, usize>::new();
    let mut arity_histogram = BTreeMap::<usize, usize>::new();

    let mut gate_histogram = Vec::<GateKindCount>::new();

    for gate in circuit.operations() {
        validate_gate_namespace(
            gate,
            circuit.num_qubits(),
            circuit.num_classical_bits(),
        )?;

        let arity = gate.qubit_count();

        increment_arity(&mut arity_histogram, arity)?;

        match arity {
            0 => {
                return Err(CircuitError::InvalidCircuit {
                    message: "quantum operation contains zero logical operands",
                });
            }

            1 => {
                single_qubit_operations = checked_increment(
                    single_qubit_operations,
                    "single-qubit operation count",
                )?;
            }

            2 => {
                two_qubit_operations = checked_increment(
                    two_qubit_operations,
                    "two-qubit operation count",
                )?;
            }

            _ => {
                multi_qubit_operations = checked_increment(
                    multi_qubit_operations,
                    "multi-qubit operation count",
                )?;
            }
        }

        if gate.is_measurement() {
            measurement_count =
                checked_increment(measurement_count, "measurement count")?;
        }

        if gate.is_barrier() {
            barrier_count =
                checked_increment(barrier_count, "barrier count")?;
        }

        if gate.is_reset() {
            reset_count = checked_increment(reset_count, "reset count")?;
        }

        if gate.is_parameterized() {
            parameterized_operations = checked_increment(
                parameterized_operations,
                "parameterized operation count",
            )?;
        }

        if gate.is_unitary() {
            unitary_operations =
                checked_increment(unitary_operations, "unitary operation count")?;
        } else {
            non_unitary_operations = checked_increment(
                non_unitary_operations,
                "non-unitary operation count",
            )?;
        }

        record_qubit_usage(gate, &mut qubit_usage)?;
        record_classical_usage(gate, &mut classical_usage)?;
        record_gate_kind(gate.kind(), &mut gate_histogram)?;
    }

    limits.check_measurements(measurement_count)?;
    limits.check_barriers(barrier_count)?;

    let depth = logical_depth(circuit.operations())?;

    limits.check_depth(depth)?;

    let arity_histogram = materialize_arity_histogram(
        &arity_histogram,
        limits,
    )?;

    Ok(CircuitStatistics::new(
        circuit.num_qubits(),
        circuit.num_classical_bits(),
        circuit.len(),
        depth,
        measurement_count,
        barrier_count,
        reset_count,
        single_qubit_operations,
        two_qubit_operations,
        multi_qubit_operations,
        parameterized_operations,
        unitary_operations,
        non_unitary_operations,
        qubit_usage.len(),
        classical_usage.len(),
        arity_histogram,
        gate_histogram,
    ))
}

// =============================================================================
// Compact analysis
// =============================================================================

/// Performs compact deterministic analysis.
///
/// This avoids usage maps and histograms and is therefore the preferred API
/// when only basic circuit metrics are required.
#[must_use = "analysis results should not be silently discarded"]
pub fn basic_statistics(
    circuit: &QuantumCircuit,
) -> Result<BasicCircuitStatistics, CircuitError> {
    basic_statistics_with_limits(circuit, circuit.limits())
}

/// Performs compact analysis with an explicit resource policy.
#[must_use = "analysis results should not be silently discarded"]
pub fn basic_statistics_with_limits(
    circuit: &QuantumCircuit,
    limits: &QuantumIrLimits,
) -> Result<BasicCircuitStatistics, CircuitError> {
    prepare_analysis(circuit, limits, AnalysisProfile::Basic)?;

    let mut measurement_count = 0usize;
    let mut barrier_count = 0usize;
    let mut reset_count = 0usize;
    let mut parameterized_operations = 0usize;
    let mut unitary_operations = 0usize;
    let mut non_unitary_operations = 0usize;

    for gate in circuit.operations() {
        validate_gate_namespace(
            gate,
            circuit.num_qubits(),
            circuit.num_classical_bits(),
        )?;

        if gate.qubit_count() == 0 {
            return Err(CircuitError::InvalidCircuit {
                message: "quantum operation contains zero logical operands",
            });
        }

        if gate.is_measurement() {
            measurement_count =
                checked_increment(measurement_count, "measurement count")?;
        }

        if gate.is_barrier() {
            barrier_count =
                checked_increment(barrier_count, "barrier count")?;
        }

        if gate.is_reset() {
            reset_count = checked_increment(reset_count, "reset count")?;
        }

        if gate.is_parameterized() {
            parameterized_operations = checked_increment(
                parameterized_operations,
                "parameterized operation count",
            )?;
        }

        if gate.is_unitary() {
            unitary_operations =
                checked_increment(unitary_operations, "unitary operation count")?;
        } else {
            non_unitary_operations = checked_increment(
                non_unitary_operations,
                "non-unitary operation count",
            )?;
        }
    }

    limits.check_measurements(measurement_count)?;
    limits.check_barriers(barrier_count)?;

    let depth = logical_depth(circuit.operations())?;

    limits.check_depth(depth)?;

    Ok(BasicCircuitStatistics {
        operation_count: circuit.len(),
        depth,
        measurement_count,
        barrier_count,
        reset_count,
        parameterized_operations,
        unitary_operations,
        non_unitary_operations,
    })
}

// =============================================================================
// Qubit usage
// =============================================================================

/// Returns deterministic logical-qubit usage statistics.
///
/// Results are ordered by ascending [`QubitId`].
///
/// No storage proportional to the declared qubit namespace is allocated.
#[must_use = "qubit analysis results should not be silently discarded"]
pub fn qubit_usage(
    circuit: &QuantumCircuit,
) -> Result<Vec<QubitUsage>, CircuitError> {
    qubit_usage_with_limits(circuit, circuit.limits())
}

/// Returns deterministic logical-qubit usage statistics under an explicit
/// policy.
#[must_use = "qubit analysis results should not be silently discarded"]
pub fn qubit_usage_with_limits(
    circuit: &QuantumCircuit,
    limits: &QuantumIrLimits,
) -> Result<Vec<QubitUsage>, CircuitError> {
    prepare_analysis(circuit, limits, AnalysisProfile::Usage)?;

    let mut usage = BTreeMap::<QubitId, usize>::new();

    for gate in circuit.operations() {
        validate_gate_namespace(
            gate,
            circuit.num_qubits(),
            circuit.num_classical_bits(),
        )?;

        record_qubit_usage(gate, &mut usage)?;
    }

    let result_len = usage.len();

    if result_len > limits.max_analysis_steps() {
        return Err(CircuitError::InvalidCircuit {
            message: "qubit usage result exceeds analysis work budget",
        });
    }

    let mut result = Vec::new();

    result
        .try_reserve(result_len)
        .map_err(|_| CircuitError::InvalidCircuit {
            message: "unable to reserve memory for qubit usage analysis",
        })?;

    for (qubit, operation_count) in usage {
        result.push(QubitUsage {
            qubit,
            operation_count,
        });
    }

    Ok(result)
}

// =============================================================================
// Classical usage
// =============================================================================

/// Returns deterministic classical destination usage.
///
/// Results are ordered by ascending classical-bit index.
#[must_use = "classical usage analysis results should not be silently discarded"]
pub fn classical_bit_usage(
    circuit: &QuantumCircuit,
) -> Result<Vec<ClassicalBitUsage>, CircuitError> {
    classical_bit_usage_with_limits(circuit, circuit.limits())
}

/// Returns deterministic classical destination usage under an explicit policy.
#[must_use = "classical usage analysis results should not be silently discarded"]
pub fn classical_bit_usage_with_limits(
    circuit: &QuantumCircuit,
    limits: &QuantumIrLimits,
) -> Result<Vec<ClassicalBitUsage>, CircuitError> {
    prepare_analysis(circuit, limits, AnalysisProfile::Usage)?;

    let mut usage = BTreeMap::<usize, usize>::new();

    for gate in circuit.operations() {
        validate_gate_namespace(
            gate,
            circuit.num_qubits(),
            circuit.num_classical_bits(),
        )?;

        record_classical_usage(gate, &mut usage)?;
    }

    let result_len = usage.len();

    if result_len > limits.max_analysis_steps() {
        return Err(CircuitError::InvalidCircuit {
            message: "classical usage result exceeds analysis work budget",
        });
    }

    let mut result = Vec::new();

    result
        .try_reserve(result_len)
        .map_err(|_| CircuitError::InvalidCircuit {
            message: "unable to reserve memory for classical usage analysis",
        })?;

    for (bit, measurement_count) in usage {
        result.push(ClassicalBitUsage {
            bit,
            measurement_count,
        });
    }

    Ok(result)
}

// =============================================================================
// Analysis profiles
// =============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AnalysisProfile {
    Basic,
    Usage,
    Complete,
}

impl AnalysisProfile {
    const fn work_multiplier(self) -> usize {
        match self {
            Self::Basic => 3,
            Self::Usage => 2,
            Self::Complete => 5,
        }
    }
}

// =============================================================================
// Preparation and safety checks
// =============================================================================

/// Performs inexpensive checks before analysis begins.
///
/// The important property is that resource policy validation happens before
/// analysis starts consuming potentially expensive structures.
///
/// Work is conservatively estimated as:
///
/// ```text
/// operation_count * profile_multiplier
///     + total quantum operands
/// ```
///
/// All arithmetic is checked.
fn prepare_analysis(
    circuit: &QuantumCircuit,
    limits: &QuantumIrLimits,
    profile: AnalysisProfile,
) -> Result<(), CircuitError> {
    limits.validate()?;

    limits.check_qubits(circuit.num_qubits())?;
    limits.check_classical_bits(circuit.num_classical_bits())?;
    limits.check_operations(circuit.len())?;

    let metadata_size = circuit.metadata().byte_size()?;
    limits.check_metadata_bytes(metadata_size)?;

    let operation_work = circuit
        .len()
        .checked_mul(profile.work_multiplier())
        .ok_or(CircuitError::ArithmeticOverflow {
            calculation: "quantum circuit analysis operation work",
        })?;

    let operand_work = circuit
        .operations()
        .iter()
        .try_fold(0usize, |total, gate| {
            total.checked_add(gate.qubit_count())
        })
        .ok_or(CircuitError::ArithmeticOverflow {
            calculation: "quantum circuit analysis operand work",
        })?;

    let required_steps = operation_work
        .checked_add(operand_work)
        .ok_or(CircuitError::ArithmeticOverflow {
            calculation: "quantum circuit analysis work",
        })?;

    if required_steps > limits.max_analysis_steps() {
        return Err(CircuitError::InvalidCircuit {
            message: "quantum circuit analysis exceeds maximum analysis work budget",
        });
    }

    Ok(())
}

// =============================================================================
// Namespace validation
// =============================================================================

/// Validates all logical and classical references contained in a gate.
///
/// This check is intentionally performed during analysis even though normal
/// circuit mutation already performs local validation. Analysis may receive
/// circuits reconstructed through deserialization, replay, migration, or
/// future external interfaces.
fn validate_gate_namespace(
    gate: &Gate,
    num_qubits: usize,
    num_classical_bits: usize,
) -> Result<(), CircuitError> {
    for &qubit in gate.qubits() {
        if qubit.index() >= num_qubits {
            return Err(CircuitError::QubitOutOfRange {
                qubit,
                num_qubits,
            });
        }
    }

    if let Some(bit) = gate.classical_target() {
        if bit >= num_classical_bits {
            return Err(CircuitError::ClassicalBitOutOfRange {
                bit,
                num_classical_bits,
            });
        }
    }

    Ok(())
}

// =============================================================================
// Qubit accounting
// =============================================================================

fn record_qubit_usage(
    gate: &Gate,
    usage: &mut BTreeMap<QubitId, usize>,
) -> Result<(), CircuitError> {
    for &qubit in gate.qubits() {
        let current = usage.get(&qubit).copied().unwrap_or(0);

        let next = checked_increment(
            current,
            "per-qubit operation count",
        )?;

        usage.insert(qubit, next);
    }

    Ok(())
}

// =============================================================================
// Classical accounting
// =============================================================================

fn record_classical_usage(
    gate: &Gate,
    usage: &mut BTreeMap<usize, usize>,
) -> Result<(), CircuitError> {
    if let Some(bit) = gate.classical_target() {
        let current = usage.get(&bit).copied().unwrap_or(0);

        let next = checked_increment(
            current,
            "per-classical-bit measurement count",
        )?;

        usage.insert(bit, next);
    }

    Ok(())
}

// =============================================================================
// Arity accounting
// =============================================================================

fn increment_arity(
    histogram: &mut BTreeMap<usize, usize>,
    arity: usize,
) -> Result<(), CircuitError> {
    let current = histogram.get(&arity).copied().unwrap_or(0);

    let next = checked_increment(
        current,
        "operation arity histogram count",
    )?;

    histogram.insert(arity, next);

    Ok(())
}

fn materialize_arity_histogram(
    histogram: &BTreeMap<usize, usize>,
    limits: &QuantumIrLimits,
) -> Result<Vec<ArityCount>, CircuitError> {
    if histogram.len() > limits.max_analysis_steps() {
        return Err(CircuitError::InvalidCircuit {
            message: "arity histogram exceeds analysis work budget",
        });
    }

    let mut result = Vec::new();

    result
        .try_reserve(histogram.len())
        .map_err(|_| CircuitError::InvalidCircuit {
            message: "unable to reserve memory for arity histogram",
        })?;

    for (&arity, &count) in histogram {
        result.push(ArityCount::new(arity, count));
    }

    Ok(result)
}

// =============================================================================
// Gate-kind accounting
// =============================================================================

/// Records gate kinds in first-seen program order.
///
/// A vector is intentionally used rather than a hash table because:
///
/// 1. the public result has deterministic ordering;
/// 2. `GateKind` is currently a compact closed standard dialect;
/// 3. the vector avoids exposing hash iteration order;
/// 4. it preserves source/program order naturally.
///
/// If the canonical gate system becomes an open-ended dialect registry, the
/// future universal-operation analysis layer should use stable operation/dialect
/// identifiers rather than extending this finite `GateKind` mechanism.
fn record_gate_kind(
    kind: GateKind,
    histogram: &mut Vec<GateKindCount>,
) -> Result<(), CircuitError> {
    if let Some(entry) = histogram
        .iter_mut()
        .find(|entry| entry.kind == kind)
    {
        entry.count =
            checked_increment(entry.count, "gate histogram count")?;

        return Ok(());
    }

    histogram
        .try_reserve(1)
        .map_err(|_| CircuitError::InvalidCircuit {
            message: "unable to reserve memory for gate histogram",
        })?;

    histogram.push(GateKindCount::new(kind, 1));

    Ok(())
}

// =============================================================================
// Logical depth
// =============================================================================

/// Computes logical dependency depth.
///
/// For every logical qubit, the analysis records the depth at which that qubit
/// was last touched.
///
/// For a new operation:
///
/// ```text
/// operation_depth = 1 + max(depth(q) for q in operands)
/// ```
///
/// Independent operations on different logical qubits may therefore occupy the
/// same logical depth.
///
/// This is a structural dependency metric. It is NOT:
///
/// - physical latency;
/// - wall-clock execution time;
/// - pulse duration;
/// - hardware schedule length.
///
/// Hardware scheduling belongs downstream.
///
/// # Sparse scalability
///
/// No array is allocated using `num_qubits`.
///
/// Only actually referenced `QubitId` values are stored.
fn logical_depth(
    operations: &[Gate],
) -> Result<usize, CircuitError> {
    let mut qubit_depth = BTreeMap::<QubitId, usize>::new();
    let mut maximum_depth = 0usize;

    for gate in operations {
        if gate.qubit_count() == 0 {
            return Err(CircuitError::InvalidCircuit {
                message: "cannot compute logical depth for a zero-operand quantum operation",
            });
        }

        let mut start_depth = 0usize;

        for &qubit in gate.qubits() {
            if let Some(&depth) = qubit_depth.get(&qubit) {
                start_depth = start_depth.max(depth);
            }
        }

        let operation_depth =
            checked_increment(start_depth, "logical circuit depth")?;

        for &qubit in gate.qubits() {
            qubit_depth.insert(qubit, operation_depth);
        }

        maximum_depth = maximum_depth.max(operation_depth);
    }

    Ok(maximum_depth)
}

// =============================================================================
// Arithmetic
// =============================================================================

fn checked_increment(
    value: usize,
    calculation: &'static str,
) -> Result<usize, CircuitError> {
    value
        .checked_add(1)
        .ok_or(CircuitError::ArithmeticOverflow {
            calculation,
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

    fn simple_gate(
        kind: GateKind,
        qubits: &[usize],
    ) -> Gate {
        let operands = qubits
            .iter()
            .copied()
            .map(QubitId::new)
            .collect::<Vec<_>>();

        Gate::simple(kind, operands)
            .expect("test gate must be locally valid")
    }

    // -------------------------------------------------------------------------
    // Depth
    // -------------------------------------------------------------------------

    #[test]
    fn empty_operation_list_has_zero_depth() {
        let operations: Vec<Gate> = Vec::new();

        assert_eq!(
            logical_depth(&operations)
                .expect("empty depth analysis must succeed"),
            0
        );
    }

    #[test]
    fn independent_single_qubit_operations_share_depth() {
        let operations = vec![
            simple_gate(GateKind::H, &[0]),
            simple_gate(GateKind::H, &[1]),
        ];

        assert_eq!(
            logical_depth(&operations)
                .expect("depth analysis must succeed"),
            1
        );
    }

    #[test]
    fn dependent_operations_increase_depth() {
        let operations = vec![
            simple_gate(GateKind::H, &[0]),
            simple_gate(GateKind::X, &[0]),
        ];

        assert_eq!(
            logical_depth(&operations)
                .expect("depth analysis must succeed"),
            2
        );
    }

    #[test]
    fn two_qubit_operation_depends_on_both_operands() {
        let operations = vec![
            simple_gate(GateKind::H, &[0]),
            simple_gate(GateKind::H, &[1]),
            simple_gate(GateKind::CX, &[0, 1]),
        ];

        assert_eq!(
            logical_depth(&operations)
                .expect("depth analysis must succeed"),
            2
        );
    }

    #[test]
    fn later_independent_operation_does_not_depend_on_unrelated_qubit() {
        let operations = vec![
            simple_gate(GateKind::H, &[0]),
            simple_gate(GateKind::H, &[1]),
            simple_gate(GateKind::X, &[0]),
        ];

        assert_eq!(
            logical_depth(&operations)
                .expect("depth analysis must succeed"),
            2
        );
    }

    #[test]
    fn depth_uses_sparse_qubit_identity() {
        let operations = vec![
            simple_gate(GateKind::H, &[1_000_000]),
            simple_gate(GateKind::X, &[1_000_000]),
        ];

        assert_eq!(
            logical_depth(&operations)
                .expect("sparse qubit depth must succeed"),
            2
        );
    }

    // -------------------------------------------------------------------------
    // Histogram
    // -------------------------------------------------------------------------

    #[test]
    fn gate_histogram_preserves_first_seen_order() {
        let mut histogram = Vec::new();

        record_gate_kind(GateKind::X, &mut histogram)
            .expect("recording must succeed");

        record_gate_kind(GateKind::H, &mut histogram)
            .expect("recording must succeed");

        record_gate_kind(GateKind::X, &mut histogram)
            .expect("recording must succeed");

        assert_eq!(histogram.len(), 2);
        assert_eq!(histogram[0].kind(), GateKind::X);
        assert_eq!(histogram[0].count(), 2);
        assert_eq!(histogram[1].kind(), GateKind::H);
        assert_eq!(histogram[1].count(), 1);
    }

    #[test]
    fn arity_histogram_is_sorted() {
        let mut histogram = BTreeMap::new();

        increment_arity(&mut histogram, 3)
            .expect("recording must succeed");

        increment_arity(&mut histogram, 1)
            .expect("recording must succeed");

        increment_arity(&mut histogram, 2)
            .expect("recording must succeed");

        let result = materialize_arity_histogram(
            &histogram,
            &test_limits(),
        )
        .expect("materialization must succeed");

        assert_eq!(
            result.iter().map(ArityCount::arity).collect::<Vec<_>>(),
            vec![1, 2, 3]
        );
    }

    // -------------------------------------------------------------------------
    // Qubit usage
    // -------------------------------------------------------------------------

    #[test]
    fn qubit_usage_is_sparse_and_sorted() {
        let mut usage = BTreeMap::new();

        let gates = vec![
            simple_gate(GateKind::X, &[100]),
            simple_gate(GateKind::H, &[2]),
            simple_gate(GateKind::CX, &[100, 2]),
        ];

        for gate in &gates {
            record_qubit_usage(gate, &mut usage)
                .expect("usage recording must succeed");
        }

        let values = usage.into_iter().collect::<Vec<_>>();

        assert_eq!(
            values,
            vec![
                (QubitId::new(2), 2),
                (QubitId::new(100), 2),
            ]
        );
    }

    #[test]
    fn qubit_usage_does_not_materialize_namespace() {
        let mut usage = BTreeMap::new();

        let gate = simple_gate(GateKind::X, &[usize::MAX]);

        record_qubit_usage(&gate, &mut usage)
            .expect("sparse usage recording must succeed");

        assert_eq!(
            usage.get(&QubitId::new(usize::MAX)),
            Some(&1)
        );
        assert_eq!(usage.len(), 1);
    }

    // -------------------------------------------------------------------------
    // Namespace validation
    // -------------------------------------------------------------------------

    #[test]
    fn namespace_validation_accepts_valid_qubits() {
        let gate = simple_gate(GateKind::CX, &[0, 1]);

        validate_gate_namespace(&gate, 2, 2)
            .expect("valid namespace must be accepted");
    }

    #[test]
    fn namespace_validation_rejects_invalid_qubit() {
        let gate = simple_gate(GateKind::X, &[2]);

        let error = validate_gate_namespace(&gate, 2, 2)
            .expect_err("out-of-range qubit must be rejected");

        assert_eq!(
            error,
            CircuitError::QubitOutOfRange {
                qubit: QubitId::new(2),
                num_qubits: 2,
            }
        );
    }

    // -------------------------------------------------------------------------
    // Arithmetic
    // -------------------------------------------------------------------------

    #[test]
    fn checked_increment_succeeds_normally() {
        assert_eq!(
            checked_increment(41, "test")
                .expect("increment must succeed"),
            42
        );
    }

    #[test]
    fn checked_increment_rejects_overflow() {
        let error = checked_increment(usize::MAX, "test")
            .expect_err("overflow must be rejected");

        assert_eq!(
            error,
            CircuitError::ArithmeticOverflow {
                calculation: "test",
            }
        );
    }

    // -------------------------------------------------------------------------
    // Gate classification
    // -------------------------------------------------------------------------

    #[test]
    fn measurement_is_non_unitary() {
        let gate = simple_gate(GateKind::Measure, &[0]);

        assert!(gate.is_measurement());
        assert!(!gate.is_unitary());
    }

    #[test]
    fn reset_is_non_unitary() {
        let gate = simple_gate(GateKind::Reset, &[0]);

        assert!(gate.is_reset());
        assert!(!gate.is_unitary());
    }

    #[test]
    fn barrier_is_unitary_compatible_for_structural_statistics() {
        let gate = simple_gate(GateKind::Barrier, &[0]);

        assert!(gate.is_barrier());
        assert!(gate.is_unitary());
    }

    // -------------------------------------------------------------------------
    // Basic statistics object
    // -------------------------------------------------------------------------

    #[test]
    fn basic_statistics_accessors_are_stable() {
        let statistics = BasicCircuitStatistics {
            operation_count: 10,
            depth: 4,
            measurement_count: 2,
            barrier_count: 1,
            reset_count: 1,
            parameterized_operations: 5,
            unitary_operations: 8,
            non_unitary_operations: 2,
        };

        assert_eq!(statistics.operation_count(), 10);
        assert_eq!(statistics.depth(), 4);
        assert_eq!(statistics.measurement_count(), 2);
        assert_eq!(statistics.barrier_count(), 1);
        assert_eq!(statistics.reset_count(), 1);
        assert_eq!(statistics.parameterized_operations(), 5);
        assert_eq!(statistics.unitary_operations(), 8);
        assert_eq!(statistics.non_unitary_operations(), 2);
    }

    // -------------------------------------------------------------------------
    // Resource-policy helper
    // -------------------------------------------------------------------------

    fn test_limits() -> QuantumIrLimits {
        QuantumIrLimits::unbounded()
    }
}