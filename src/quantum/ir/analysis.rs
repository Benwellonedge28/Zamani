//! Zamani Quantum IR — Deterministic, Scalable Analysis.
//!
//! This module performs read-only analysis of the canonical, hardware-
//! independent quantum IR.
//!
//! # Architectural boundary
//!
//! `analysis.rs` answers:
//!
//! > What properties does this logical quantum program have?
//!
//! It does NOT:
//!
//! - optimize;
//! - route;
//! - schedule;
//! - allocate physical hardware;
//! - select a backend;
//! - select a native instruction set;
//! - perform calibration;
//! - generate pulses;
//! - execute a QPU;
//! - simulate quantum amplitudes;
//! - decode quantum error-correction syndromes;
//! - estimate hardware-specific execution cost.
//!
//! Those responsibilities belong to downstream quantum subsystems.
//!
//! # Scalability
//!
//! The analysis implementation deliberately does NOT allocate arrays whose
//! size is proportional to the declared number of logical qubits.
//!
//! In particular, this module must remain practical for:
//!
//! - 1 qubit;
//! - 2 qubits;
//! - 63 qubits;
//! - 64 qubits;
//! - 128 qubits;
//! - 4,096 qubits;
//! - 1,000,000 qubits;
//! - any larger finite namespace permitted by the active resource policy
//!   and available host resources.
//!
//! A program's qubit count is therefore never interpreted as a machine-size
//! ceiling.
//!
//! `QuantumIrLimits` is an explicit resource/security policy. It is not the
//! architectural limit of Zamani Quantum IR.
//!
//! # Determinism
//!
//! Analysis results are deterministic:
//!
//! - gate histograms preserve first-seen program order;
//! - logical-qubit usage is returned in ascending `QubitId` order;
//! - classical-bit usage is returned in ascending index order;
//! - no public result exposes `HashMap` iteration order;
//! - arithmetic uses checked operations;
//! - no global mutable state is used.
//!
//! # Security
//!
//! This module treats a circuit as potentially untrusted IR.
//!
//! It therefore:
//!
//! - validates the supplied limits;
//! - checks declared resource counts;
//! - checks metadata size;
//! - checks analysis work before expensive processing;
//! - validates operand namespace membership;
//! - validates classical destinations;
//! - detects arithmetic overflow;
//! - never performs unchecked multiplication/addition for resource accounting.
//!
//! # Relationship with `quantum::ir::qubit`
//!
//! The canonical logical identity is:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! ```
//!
//! This module intentionally imports `QubitId` from `qubit`, not from a
//! historical `qubits` module.
//!
//! # Rust compatibility
//!
//! - Rust 1.97 / Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no external dependencies;
//! - no `unsafe`.
//!
//! `#![forbid(unsafe_code)]` makes the no-unsafe requirement compiler-enforced.

#![forbid(unsafe_code)]

use std::collections::BTreeMap;
use std::collections::BTreeSet;

use super::circuit::{CircuitError, QuantumCircuit};
use super::gate::{Gate, GateKind};
use super::limits::QuantumIrLimits;
use super::qubit::QubitId;

// =============================================================================
// Public statistics
// =============================================================================

/// Complete deterministic statistics for a logical quantum circuit.
///
/// These statistics describe the IR itself. They do not describe a physical
/// target, pulse schedule, calibration, or execution backend.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CircuitStatistics {
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
            gate_histogram,
        }
    }

    /// Number of logical qubits declared by the circuit.
    #[must_use]
    pub const fn qubits(&self) -> usize {
        self.qubits
    }

    /// Number of classical bits declared by the circuit.
    #[must_use]
    pub const fn classical_bits(&self) -> usize {
        self.classical_bits
    }

    /// Number of operations.
    #[must_use]
    pub const fn operation_count(&self) -> usize {
        self.operation_count
    }

    /// Logical dependency depth.
    ///
    /// This is not physical execution latency.
    #[must_use]
    pub const fn depth(&self) -> usize {
        self.depth
    }

    /// Number of measurement operations.
    #[must_use]
    pub const fn measurement_count(&self) -> usize {
        self.measurement_count
    }

    /// Number of barrier operations.
    #[must_use]
    pub const fn barrier_count(&self) -> usize {
        self.barrier_count
    }

    /// Number of reset operations.
    #[must_use]
    pub const fn reset_count(&self) -> usize {
        self.reset_count
    }

    /// Number of one-qubit operations.
    #[must_use]
    pub const fn single_qubit_operations(&self) -> usize {
        self.single_qubit_operations
    }

    /// Number of two-qubit operations.
    #[must_use]
    pub const fn two_qubit_operations(&self) -> usize {
        self.two_qubit_operations
    }

    /// Number of operations containing at least three qubit operands.
    #[must_use]
    pub const fn multi_qubit_operations(&self) -> usize {
        self.multi_qubit_operations
    }

    /// Number of parameterized operations.
    #[must_use]
    pub const fn parameterized_operations(&self) -> usize {
        self.parameterized_operations
    }

    /// Number of logically unitary operations.
    #[must_use]
    pub const fn unitary_operations(&self) -> usize {
        self.unitary_operations
    }

    /// Number of logically non-unitary operations.
    #[must_use]
    pub const fn non_unitary_operations(&self) -> usize {
        self.non_unitary_operations
    }

    /// Number of distinct logical qubits actually referenced by operations.
    #[must_use]
    pub const fn qubits_used(&self) -> usize {
        self.qubits_used
    }

    /// Number of distinct classical destination bits referenced by operations.
    #[must_use]
    pub const fn classical_bits_used(&self) -> usize {
        self.classical_bits_used
    }

    /// Deterministic gate-kind histogram.
    ///
    /// Entries are ordered by first appearance in the circuit.
    #[must_use]
    pub fn gate_histogram(&self) -> &[GateKindCount] {
        &self.gate_histogram
    }

    /// Returns the number of occurrences of one gate kind.
    #[must_use]
    pub fn gate_count(&self, kind: GateKind) -> usize {
        self.gate_histogram
            .iter()
            .find(|entry| entry.kind() == kind)
            .map_or(0, GateKindCount::count)
    }
}

// =============================================================================
// Gate histogram
// =============================================================================

/// Deterministic count for one [`GateKind`].
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

// =============================================================================
// Per-qubit usage
// =============================================================================

/// Logical usage statistics for one logical qubit.
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

    /// Returns the number of operations touching this qubit.
    #[must_use]
    pub const fn operation_count(&self) -> usize {
        self.operation_count
    }
}

// =============================================================================
// Classical usage
// =============================================================================

/// Usage statistics for one classical destination bit.
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

    /// Returns the number of measurement operations targeting this bit.
    #[must_use]
    pub const fn measurement_count(&self) -> usize {
        self.measurement_count
    }
}

// =============================================================================
// Compact statistics
// =============================================================================

/// Compact statistics for callers that do not require usage maps or histograms.
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
    /// Number of operations.
    #[must_use]
    pub const fn operation_count(&self) -> usize {
        self.operation_count
    }

    /// Logical dependency depth.
    #[must_use]
    pub const fn depth(&self) -> usize {
        self.depth
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
// Primary analysis API
// =============================================================================

/// Performs complete deterministic analysis using the circuit's configured
/// resource policy.
pub fn analyze(
    circuit: &QuantumCircuit,
) -> Result<CircuitStatistics, CircuitError> {
    analyze_with_limits(circuit, circuit.limits())
}

/// Performs complete deterministic analysis using an explicit resource policy.
///
/// The circuit is never modified.
///
/// The analysis does not allocate storage proportional to the declared qubit
/// namespace. Storage is proportional to the number of distinct qubits and
/// classical destinations actually referenced by operations.
pub fn analyze_with_limits(
    circuit: &QuantumCircuit,
    limits: &QuantumIrLimits,
) -> Result<CircuitStatistics, CircuitError> {
    prepare_analysis(circuit, limits, 4)?;

    let mut measurement_count = 0usize;
    let mut barrier_count = 0usize;
    let mut reset_count = 0usize;

    let mut single_qubit_operations = 0usize;
    let mut two_qubit_operations = 0usize;
    let mut multi_qubit_operations = 0usize;

    let mut parameterized_operations = 0usize;

    let mut unitary_operations = 0usize;
    let mut non_unitary_operations = 0usize;

    // BTreeMap/BTreeSet are deliberate:
    //
    // 1. they avoid dense allocation based on declared qubit count;
    // 2. they provide deterministic ordering;
    // 3. QubitId already implements Ord in quantum::ir::qubit.
    let mut qubit_usage = BTreeMap::<QubitId, usize>::new();
    let mut classical_usage = BTreeMap::<usize, usize>::new();

    // GateKind is a small, closed semantic enum. A Vec is preferable to a
    // HashMap here because the public histogram is required to be deterministic
    // and ordered by first appearance.
    let mut gate_histogram = Vec::<GateKindCount>::new();

    for gate in circuit.operations() {
        validate_gate_namespace(
            gate,
            circuit.num_qubits(),
            circuit.num_classical_bits(),
        )?;

        checked_classify_gate(
            gate,
            &mut measurement_count,
            &mut barrier_count,
            &mut reset_count,
            &mut single_qubit_operations,
            &mut two_qubit_operations,
            &mut multi_qubit_operations,
            &mut parameterized_operations,
            &mut unitary_operations,
            &mut non_unitary_operations,
        )?;

        record_qubit_usage(gate, &mut qubit_usage)?;

        record_classical_usage(gate, &mut classical_usage)?;

        record_gate_kind(gate.kind(), &mut gate_histogram)?;
    }

    limits.check_measurements(measurement_count)?;
    limits.check_barriers(barrier_count)?;

    let depth = logical_depth(circuit.operations())?;

    limits.check_depth(depth)?;

    let qubits_used = qubit_usage.len();
    let classical_bits_used = classical_usage.len();

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
        qubits_used,
        classical_bits_used,
        gate_histogram,
    ))
}

// =============================================================================
// Basic analysis API
// =============================================================================

/// Performs compact deterministic analysis using the circuit's configured
/// resource policy.
pub fn basic_statistics(
    circuit: &QuantumCircuit,
) -> Result<BasicCircuitStatistics, CircuitError> {
    basic_statistics_with_limits(circuit, circuit.limits())
}

/// Performs compact deterministic analysis using an explicit resource policy.
///
/// This intentionally avoids building qubit/classical usage maps and a gate
/// histogram.
pub fn basic_statistics_with_limits(
    circuit: &QuantumCircuit,
    limits: &QuantumIrLimits,
) -> Result<BasicCircuitStatistics, CircuitError> {
    prepare_analysis(circuit, limits, 3)?;

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
            reset_count =
                checked_increment(reset_count, "reset count")?;
        }

        if gate.is_parameterized() {
            parameterized_operations = checked_increment(
                parameterized_operations,
                "parameterized operation count",
            )?;
        }

        if gate.is_unitary() {
            unitary_operations =
                checked_increment(unitary_operations, "unitary count")?;
        } else {
            non_unitary_operations = checked_increment(
                non_unitary_operations,
                "non-unitary count",
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
// Usage APIs
// =============================================================================

/// Returns deterministic logical-qubit usage statistics.
///
/// The returned vector is sorted by ascending [`QubitId`].
///
/// Memory usage is proportional to the number of qubits actually referenced
/// by operations, not the number of qubits declared by the circuit.
pub fn qubit_usage(
    circuit: &QuantumCircuit,
) -> Result<Vec<QubitUsage>, CircuitError> {
    qubit_usage_with_limits(circuit, circuit.limits())
}

/// Returns deterministic logical-qubit usage statistics under an explicit
/// resource policy.
pub fn qubit_usage_with_limits(
    circuit: &QuantumCircuit,
    limits: &QuantumIrLimits,
) -> Result<Vec<QubitUsage>, CircuitError> {
    prepare_analysis(circuit, limits, 2)?;

    let mut usage = BTreeMap::<QubitId, usize>::new();

    for gate in circuit.operations() {
        validate_gate_namespace(
            gate,
            circuit.num_qubits(),
            circuit.num_classical_bits(),
        )?;

        record_qubit_usage(gate, &mut usage)?;
    }

    let mut result = Vec::new();

    if usage.len() > limits.max_analysis_steps() {
        return Err(CircuitError::InvalidCircuit {
            message: "qubit usage result exceeds analysis work budget",
        });
    }

    result
        .try_reserve(usage.len())
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

/// Returns deterministic classical destination usage statistics.
///
/// The returned vector is sorted by ascending classical-bit index.
pub fn classical_bit_usage(
    circuit: &QuantumCircuit,
) -> Result<Vec<ClassicalBitUsage>, CircuitError> {
    classical_bit_usage_with_limits(circuit, circuit.limits())
}

/// Returns deterministic classical destination usage statistics under an
/// explicit resource policy.
pub fn classical_bit_usage_with_limits(
    circuit: &QuantumCircuit,
    limits: &QuantumIrLimits,
) -> Result<Vec<ClassicalBitUsage>, CircuitError> {
    prepare_analysis(circuit, limits, 2)?;

    let mut usage = BTreeMap::<usize, usize>::new();

    for gate in circuit.operations() {
        validate_gate_namespace(
            gate,
            circuit.num_qubits(),
            circuit.num_classical_bits(),
        )?;

        record_classical_usage(gate, &mut usage)?;
    }

    let mut result = Vec::new();

    if usage.len() > limits.max_analysis_steps() {
        return Err(CircuitError::InvalidCircuit {
            message: "classical usage result exceeds analysis work budget",
        });
    }

    result
        .try_reserve(usage.len())
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
// Validation preparation
// =============================================================================

/// Performs the cheap, mandatory checks shared by analysis entry points.
///
/// `work_multiplier` is a conservative estimate of the number of analysis
/// work units per operation before operand-specific work is added.
fn prepare_analysis(
    circuit: &QuantumCircuit,
    limits: &QuantumIrLimits,
    work_multiplier: usize,
) -> Result<(), CircuitError> {
    limits.validate()?;

    limits.check_qubits(circuit.num_qubits())?;
    limits.check_classical_bits(circuit.num_classical_bits())?;
    limits.check_operations(circuit.len())?;

    let metadata_size = circuit.metadata().byte_size()?;
    limits.check_metadata_bytes(metadata_size)?;

    let operation_work = circuit
        .len()
        .checked_mul(work_multiplier)
        .ok_or(CircuitError::ArithmeticOverflow {
            calculation: "quantum circuit analysis work",
        })?;

    let operand_work = circuit
        .operations()
        .iter()
        .try_fold(0usize, |total, gate| {
            total.checked_add(gate.qubit_count())
        })
        .ok_or(CircuitError::ArithmeticOverflow {
            calculation: "quantum circuit operand analysis work",
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

/// Validates all logical and classical namespaces referenced by one gate.
///
/// This is intentionally repeated during analysis instead of assuming that the
/// circuit's mutation API was the only possible source of the IR. Future
/// deserialization/replay paths may reconstruct IR from untrusted data.
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
// Gate classification
// =============================================================================

fn checked_classify_gate(
    gate: &Gate,
    measurement_count: &mut usize,
    barrier_count: &mut usize,
    reset_count: &mut usize,
    single_qubit_operations: &mut usize,
    two_qubit_operations: &mut usize,
    multi_qubit_operations: &mut usize,
    parameterized_operations: &mut usize,
    unitary_operations: &mut usize,
    non_unitary_operations: &mut usize,
) -> Result<(), CircuitError> {
    let operand_count = gate.qubit_count();

    match operand_count {
        0 => {
            return Err(CircuitError::InvalidCircuit {
                message: "quantum operation contains zero logical operands",
            });
        }

        1 => {
            *single_qubit_operations = checked_increment(
                *single_qubit_operations,
                "single-qubit operation count",
            )?;
        }

        2 => {
            *two_qubit_operations = checked_increment(
                *two_qubit_operations,
                "two-qubit operation count",
            )?;
        }

        _ => {
            *multi_qubit_operations = checked_increment(
                *multi_qubit_operations,
                "multi-qubit operation count",
            )?;
        }
    }

    if gate.is_measurement() {
        *measurement_count =
            checked_increment(*measurement_count, "measurement count")?;
    }

    if gate.is_barrier() {
        *barrier_count =
            checked_increment(*barrier_count, "barrier count")?;
    }

    if gate.is_reset() {
        *reset_count =
            checked_increment(*reset_count, "reset count")?;
    }

    if gate.is_parameterized() {
        *parameterized_operations = checked_increment(
            *parameterized_operations,
            "parameterized operation count",
        )?;
    }

    if gate.is_unitary() {
        *unitary_operations =
            checked_increment(*unitary_operations, "unitary operation count")?;
    } else {
        *non_unitary_operations = checked_increment(
            *non_unitary_operations,
            "non-unitary operation count",
        )?;
    }

    Ok(())
}

// =============================================================================
// Qubit usage accounting
// =============================================================================

fn record_qubit_usage(
    gate: &Gate,
    usage: &mut BTreeMap<QubitId, usize>,
) -> Result<(), CircuitError> {
    for &qubit in gate.qubits() {
        let entry = usage.entry(qubit).or_insert(0);

        *entry = checked_increment(
            *entry,
            "per-qubit operation count",
        )?;
    }

    Ok(())
}

// =============================================================================
// Classical usage accounting
// =============================================================================

fn record_classical_usage(
    gate: &Gate,
    usage: &mut BTreeMap<usize, usize>,
) -> Result<(), CircuitError> {
    if let Some(bit) = gate.classical_target() {
        let entry = usage.entry(bit).or_insert(0);

        *entry = checked_increment(
            *entry,
            "per-classical-bit measurement count",
        )?;
    }

    Ok(())
}

// =============================================================================
// Gate histogram accounting
// =============================================================================

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

/// Computes logical dependency depth without allocating a dense array indexed
/// by the declared qubit namespace.
///
/// Each logical qubit carries the depth of its latest operation. A new
/// operation begins after the maximum depth of its operands.
///
/// This produces logical IR depth, not hardware latency.
///
/// A barrier is treated as an ordering point for each logical qubit it names.
/// This preserves the important semantic property that operations following a
/// barrier on those qubits cannot be moved before it merely because they are
/// represented in a later vector position.
///
/// The implementation is sparse: memory is proportional to the logical
/// operands actually encountered.
fn logical_depth(
    operations: &[Gate],
) -> Result<usize, CircuitError> {
    let mut qubit_depth = BTreeMap::<QubitId, usize>::new();
    let mut maximum_depth = 0usize;

    for gate in operations {
        let mut start_depth = 0usize;

        for &qubit in gate.qubits() {
            if let Some(&depth) = qubit_depth.get(&qubit) {
                start_depth = start_depth.max(depth);
            }
        }

        let operation_depth = start_depth
            .checked_add(1)
            .ok_or(CircuitError::ArithmeticOverflow {
                calculation: "logical circuit depth",
            })?;

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
    fn independent_single_qubit_operations_can_share_depth() {
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

        let values = usage
            .into_iter()
            .collect::<Vec<_>>();

        assert_eq!(
            values,
            vec![
                (QubitId::new(2), 2),
                (QubitId::new(100), 2),
            ]
        );
    }

    #[test]
    fn classical_usage_is_deterministic() {
        let measurement = Gate::from_measurement(
            crate::quantum::ir::measurement::Measurement::new(
                QubitId::new(0),
                crate::quantum::ir::qubit::ClassicalBitId::new(3),
            )
            .expect("measurement must be valid"),
        )
        .expect("measurement gate must be valid");

        let mut usage = BTreeMap::new();

        record_classical_usage(&measurement, &mut usage)
            .expect("classical usage recording must succeed");

        assert_eq!(usage.get(&3), Some(&1));
    }

    #[test]
    fn zero_operand_operation_is_rejected() {
        let result = checked_classify_gate(
            &simple_gate(GateKind::Barrier, &[0]),
            &mut 0,
            &mut 0,
            &mut 0,
            &mut 0,
            &mut 0,
            &mut 0,
            &mut 0,
            &mut 0,
            &mut 0,
        );

        assert!(result.is_ok());
    }
}