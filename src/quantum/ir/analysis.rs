//! Zamani Quantum IR — Deterministic Circuit Analysis.
//!
//! This module provides read-only analysis of the canonical, hardware-
//! independent quantum IR.
//!
//! # Architectural boundary
//!
//! `analysis.rs` describes properties of a logical quantum program.
//!
//! It does NOT perform:
//!
//! - optimization;
//! - routing;
//! - scheduling;
//! - calibration;
//! - hardware execution;
//! - QPU communication;
//! - error-correction decoding;
//! - backend-specific cost estimation.
//!
//! Those concerns belong to downstream quantum compiler/backend stages.
//!
//! The canonical representation remains [`QuantumCircuit`]. Analysis consumes
//! that representation and never mutates it.
//!
//! # Design goals
//!
//! - deterministic results;
//! - read-only operation;
//! - explicit analysis work budgets;
//! - overflow-safe arithmetic;
//! - hardware-independent semantics;
//! - stable logical-qubit accounting;
//! - stable gate-kind accounting;
//! - no hash-map iteration nondeterminism;
//! - no mutation of the circuit;
//! - compatibility with untrusted IR;
//! - no external dependencies.
//!
//! # Rust compatibility
//!
//! Rust 1.97.1.
//!
//! No nightly features are required.

use super::circuit::{CircuitError, QuantumCircuit};
use super::gate::{Gate, GateKind};
use super::limits::QuantumIrLimits;
use super::qubits::QubitId;

// =============================================================================
// Circuit statistics
// =============================================================================

/// Complete deterministic statistics for one logical quantum circuit.
///
/// All counters describe the logical IR itself.
///
/// They do NOT represent:
///
/// - physical gate counts;
/// - pulse counts;
/// - execution latency;
/// - calibration cost;
/// - hardware-specific resource usage.
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

    /// Number of logical classical bits declared by the circuit.
    #[must_use]
    pub const fn classical_bits(&self) -> usize {
        self.classical_bits
    }

    /// Number of ordered operations.
    #[must_use]
    pub const fn operation_count(&self) -> usize {
        self.operation_count
    }

    /// Logical circuit depth.
    ///
    /// This is a logical dependency depth, not a hardware execution latency.
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

    /// Number of operations containing three or more qubit operands.
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

    /// Number of distinct logical qubits touched by operations.
    #[must_use]
    pub const fn qubits_used(&self) -> usize {
        self.qubits_used
    }

    /// Number of distinct classical destination bits referenced by operations.
    #[must_use]
    pub const fn classical_bits_used(&self) -> usize {
        self.classical_bits_used
    }

    /// Returns the deterministic gate-kind histogram.
    ///
    /// Entries are ordered by first appearance in the circuit.
    #[must_use]
    pub fn gate_histogram(&self) -> &[GateKindCount] {
        &self.gate_histogram
    }

    /// Returns the number of occurrences of a gate kind.
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

/// Logical usage statistics for one qubit.
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
// Complete analysis
// =============================================================================

/// Performs complete deterministic analysis using the circuit's own limits.
///
/// This is the primary analysis API.
pub fn analyze(
    circuit: &QuantumCircuit,
) -> Result<CircuitStatistics, CircuitError> {
    analyze_with_limits(circuit, circuit.limits())
}

/// Performs complete deterministic analysis using an explicit resource policy.
///
/// The circuit is never modified.
///
/// The supplied limits are checked before analysis begins.
pub fn analyze_with_limits(
    circuit: &QuantumCircuit,
    limits: &QuantumIrLimits,
) -> Result<CircuitStatistics, CircuitError> {
    limits.validate()?;

    limits.check_qubits(circuit.num_qubits())?;
    limits.check_classical_bits(circuit.num_classical_bits())?;
    limits.check_operations(circuit.len())?;

    let metadata_size = circuit.metadata().byte_size()?;

    limits.check_metadata_bytes(metadata_size)?;

    /*
     * Analysis work is deliberately bounded.
     *
     * The estimate is conservative:
     *
     *     4 × operations
     *   + operand visits
     *   + qubit namespace scan
     *   + classical namespace scan
     *
     * The estimate itself is overflow-safe.
     */
    let operation_work = circuit
        .len()
        .checked_mul(4)
        .ok_or(CircuitError::ArithmeticOverflow {
            calculation: "circuit analysis work",
        })?;

    let operand_work = circuit
        .operations()
        .iter()
        .try_fold(0usize, |total, gate| {
            total.checked_add(gate.qubits().len())
        })
        .ok_or(CircuitError::ArithmeticOverflow {
            calculation: "circuit operand analysis work",
        })?;

    let required_steps = operation_work
        .checked_add(operand_work)
        .and_then(|value| value.checked_add(circuit.num_qubits()))
        .and_then(|value| value.checked_add(circuit.num_classical_bits()))
        .ok_or(CircuitError::ArithmeticOverflow {
            calculation: "circuit analysis work",
        })?;

    if required_steps > limits.max_analysis_steps() {
        return Err(CircuitError::InvalidCircuit {
            message: "circuit analysis exceeds maximum analysis work budget",
        });
    }

    let mut measurement_count = 0usize;
    let mut barrier_count = 0usize;
    let mut reset_count = 0usize;

    let mut single_qubit_operations = 0usize;
    let mut two_qubit_operations = 0usize;
    let mut multi_qubit_operations = 0usize;

    let mut parameterized_operations = 0usize;

    let mut unitary_operations = 0usize;
    let mut non_unitary_operations = 0usize;

    let mut qubit_used = vec![false; circuit.num_qubits()];
    let mut classical_used = vec![false; circuit.num_classical_bits()];

    /*
     * A Vec is intentionally used instead of HashMap.
     *
     * This guarantees deterministic iteration and serialization without
     * depending on hash-map ordering.
     */
    let mut gate_histogram = Vec::<GateKindCount>::new();

    for gate in circuit.operations() {
        classify_gate(
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
            &mut qubit_used,
            &mut classical_used,
            &mut gate_histogram,
        )?;
    }

    limits.check_measurements(measurement_count)?;
    limits.check_barriers(barrier_count)?;

    let depth = circuit.depth()?;

    limits.check_depth(depth)?;

    let qubits_used = qubit_used
        .iter()
        .filter(|used| **used)
        .count();

    let classical_bits_used = classical_used
        .iter()
        .filter(|used| **used)
        .count();

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
// Basic analysis
// =============================================================================

/// Compact deterministic circuit statistics.
///
/// Use this when the caller does not need:
///
/// - per-qubit usage;
/// - classical-bit usage;
/// - gate histogram.
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

    /// Logical depth.
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

/// Performs compact deterministic analysis using the circuit's own limits.
pub fn basic_statistics(
    circuit: &QuantumCircuit,
) -> Result<BasicCircuitStatistics, CircuitError> {
    basic_statistics_with_limits(circuit, circuit.limits())
}

/// Performs compact deterministic analysis using explicit limits.
pub fn basic_statistics_with_limits(
    circuit: &QuantumCircuit,
    limits: &QuantumIrLimits,
) -> Result<BasicCircuitStatistics, CircuitError> {
    limits.validate()?;

    limits.check_qubits(circuit.num_qubits())?;
    limits.check_classical_bits(circuit.num_classical_bits())?;
    limits.check_operations(circuit.len())?;

    let required_steps = circuit
        .len()
        .checked_mul(3)
        .ok_or(CircuitError::ArithmeticOverflow {
            calculation: "basic circuit analysis work",
        })?;

    if required_steps > limits.max_analysis_steps() {
        return Err(CircuitError::InvalidCircuit {
            message: "basic circuit analysis exceeds maximum analysis work budget",
        });
    }

    let mut measurement_count = 0usize;
    let mut barrier_count = 0usize;
    let mut reset_count = 0usize;
    let mut parameterized_operations = 0usize;
    let mut unitary_operations = 0usize;
    let mut non_unitary_operations = 0usize;

    for gate in circuit.operations() {
        if gate.is_measurement() {
            measurement_count = measurement_count
                .checked_add(1)
                .ok_or(CircuitError::ArithmeticOverflow {
                    calculation: "measurement count",
                })?;
        }

        if gate.is_barrier() {
            barrier_count = barrier_count
                .checked_add(1)
                .ok_or(CircuitError::ArithmeticOverflow {
                    calculation: "barrier count",
                })?;
        }

        if gate.is_reset() {
            reset_count = reset_count
                .checked_add(1)
                .ok_or(CircuitError::ArithmeticOverflow {
                    calculation: "reset count",
                })?;
        }

        if gate.is_parameterized() {
            parameterized_operations = parameterized_operations
                .checked_add(1)
                .ok_or(CircuitError::ArithmeticOverflow {
                    calculation: "parameterized operation count",
                })?;
        }

        if gate.is_unitary() {
            unitary_operations = unitary_operations
                .checked_add(1)
                .ok_or(CircuitError::ArithmeticOverflow {
                    calculation: "unitary operation count",
                })?;
        } else {
            non_unitary_operations = non_unitary_operations
                .checked_add(1)
                .ok_or(CircuitError::ArithmeticOverflow {
                    calculation: "non-unitary operation count",
                })?;
        }
    }

    limits.check_measurements(measurement_count)?;
    limits.check_barriers(barrier_count)?;

    let depth = circuit.depth()?;

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

/// Calculates per-logical-qubit operation usage using the circuit's own limits.
///
/// The returned entries are ordered by logical qubit identity.
pub fn qubit_usage(
    circuit: &QuantumCircuit,
) -> Result<Vec<QubitUsage>, CircuitError> {
    qubit_usage_with_limits(circuit, circuit.limits())
}

/// Calculates per-logical-qubit operation usage under explicit limits.
///
/// The result is ordered by ascending logical qubit identity.
pub fn qubit_usage_with_limits(
    circuit: &QuantumCircuit,
    limits: &QuantumIrLimits,
) -> Result<Vec<QubitUsage>, CircuitError> {
    limits.validate()?;

    limits.check_qubits(circuit.num_qubits())?;
    limits.check_operations(circuit.len())?;

    let required_steps = circuit
        .len()
        .checked_mul(2)
        .and_then(|value| {
            value.checked_add(circuit.num_qubits())
        })
        .ok_or(CircuitError::ArithmeticOverflow {
            calculation: "qubit usage analysis work",
        })?;

    if required_steps > limits.max_analysis_steps() {
        return Err(CircuitError::InvalidCircuit {
            message: "qubit usage analysis exceeds maximum analysis work budget",
        });
    }

    let mut counts = vec![0usize; circuit.num_qubits()];

    for gate in circuit.operations() {
        for qubit in gate.qubits() {
            let index = qubit.index();

            if index >= counts.len() {
                return Err(CircuitError::QubitOutOfRange {
                    qubit: *qubit,
                    num_qubits: counts.len(),
                });
            }

            counts[index] = counts[index]
                .checked_add(1)
                .ok_or(CircuitError::ArithmeticOverflow {
                    calculation: "per-qubit operation count",
                })?;
        }
    }

    let mut result = Vec::new();

    for (index, count) in counts.into_iter().enumerate() {
        if count == 0 {
            continue;
        }

        result.push(QubitUsage {
            qubit: QubitId::new(index),
            operation_count: count,
        });
    }

    Ok(result)
}

// =============================================================================
// Internal helpers
// =============================================================================

#[allow(clippy::too_many_arguments)]
fn classify_gate(
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
    qubit_used: &mut [bool],
    classical_used: &mut [bool],
    gate_histogram: &mut Vec<GateKindCount>,
) -> Result<(), CircuitError> {
    let kind = gate.kind();

    increment_if(
        measurement_count,
        gate.is_measurement(),
        "measurement count",
    )?;

    increment_if(
        barrier_count,
        gate.is_barrier(),
        "barrier count",
    )?;

    increment_if(
        reset_count,
        gate.is_reset(),
        "reset count",
    )?;

    match gate.qubits().len() {
        0 => {
            return Err(CircuitError::InvalidCircuit {
                message:
                    "analysis encountered an operation without logical operands",
            });
        }

        1 => {
            increment_if(
                single_qubit_operations,
                true,
                "single-qubit operation count",
            )?;
        }

        2 => {
            increment_if(
                two_qubit_operations,
                true,
                "two-qubit operation count",
            )?;
        }

        _ => {
            increment_if(
                multi_qubit_operations,
                true,
                "multi-qubit operation count",
            )?;
        }
    }

    increment_if(
        parameterized_operations,
        gate.is_parameterized(),
        "parameterized operation count",
    )?;

    if gate.is_unitary() {
        increment_if(
            unitary_operations,
            true,
            "unitary operation count",
        )?;
    } else {
        increment_if(
            non_unitary_operations,
            true,
            "non-unitary operation count",
        )?;
    }

    for qubit in gate.qubits() {
        let index = qubit.index();

        if index >= qubit_used.len() {
            return Err(CircuitError::QubitOutOfRange {
                qubit: *qubit,
                num_qubits: qubit_used.len(),
            });
        }

        qubit_used[index] = true;
    }

    if let Some(bit) = gate.classical_target() {
        if bit >= classical_used.len() {
            return Err(CircuitError::ClassicalBitOutOfRange {
                bit,
                num_classical_bits: classical_used.len(),
            });
        }

        classical_used[bit] = true;
    }

    add_histogram_entry(gate_histogram, kind)?;

    Ok(())
}

fn increment_if(
    counter: &mut usize,
    enabled: bool,
    calculation: &'static str,
) -> Result<(), CircuitError> {
    if !enabled {
        return Ok(());
    }

    *counter = counter
        .checked_add(1)
        .ok_or(CircuitError::ArithmeticOverflow {
            calculation,
        })?;

    Ok(())
}

fn add_histogram_entry(
    histogram: &mut Vec<GateKindCount>,
    kind: GateKind,
) -> Result<(), CircuitError> {
    if let Some(entry) = histogram
        .iter_mut()
        .find(|entry| entry.kind == kind)
    {
        entry.count = entry
            .count
            .checked_add(1)
            .ok_or(CircuitError::ArithmeticOverflow {
                calculation:
                    "gate-kind histogram count",
            })?;

        return Ok(());
    }

    histogram.push(GateKindCount::new(kind, 1));

    Ok(())
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::quantum::ir::gate::Gate;
    use crate::quantum::ir::limits::QuantumIrLimits;

    #[test]
    fn empty_circuit_has_zero_statistics() {
        let circuit =
            QuantumCircuit::new(4, 4)
                .expect("valid circuit");

        let stats =
            analyze(&circuit)
                .expect("analysis must succeed");

        assert_eq!(stats.qubits(), 4);
        assert_eq!(stats.classical_bits(), 4);
        assert_eq!(stats.operation_count(), 0);
        assert_eq!(stats.depth(), 0);
        assert_eq!(stats.measurement_count(), 0);
        assert_eq!(stats.barrier_count(), 0);
        assert_eq!(stats.reset_count(), 0);
        assert_eq!(stats.qubits_used(), 0);
        assert_eq!(stats.classical_bits_used(), 0);
        assert!(stats.gate_histogram().is_empty());
    }

    #[test]
    fn analysis_is_deterministic() {
        let mut first =
            QuantumCircuit::new(2, 2)
                .expect("valid circuit");

        let mut second =
            QuantumCircuit::new(2, 2)
                .expect("valid circuit");

        first
            .push(
                Gate::x(QubitId::new(0))
                    .expect("valid gate"),
            )
            .expect("push must succeed");

        first
            .push(
                Gate::h(QubitId::new(1))
                    .expect("valid gate"),
            )
            .expect("push must succeed");

        second
            .push(
                Gate::x(QubitId::new(0))
                    .expect("valid gate"),
            )
            .expect("push must succeed");

        second
            .push(
                Gate::h(QubitId::new(1))
                    .expect("valid gate"),
            )
            .expect("push must succeed");

        assert_eq!(
            analyze(&first),
            analyze(&second)
        );
    }

    #[test]
    fn gate_histogram_is_deterministic() {
        let mut circuit =
            QuantumCircuit::new(2, 2)
                .expect("valid circuit");

        circuit
            .push(
                Gate::x(QubitId::new(0))
                    .expect("valid gate"),
            )
            .expect("push must succeed");

        circuit
            .push(
                Gate::h(QubitId::new(0))
                    .expect("valid gate"),
            )
            .expect("push must succeed");

        circuit
            .push(
                Gate::x(QubitId::new(1))
                    .expect("valid gate"),
            )
            .expect("push must succeed");

        let stats =
            analyze(&circuit)
                .expect("analysis must succeed");

        let histogram =
            stats.gate_histogram();

        assert_eq!(histogram.len(), 2);

        assert_eq!(
            histogram[0].kind(),
            GateKind::X
        );

        assert_eq!(
            histogram[0].count(),
            2
        );

        assert_eq!(
            histogram[1].kind(),
            GateKind::H
        );

        assert_eq!(
            histogram[1].count(),
            1
        );
    }

    #[test]
    fn qubit_usage_is_sorted_by_identity() {
        let mut circuit =
            QuantumCircuit::new(3, 0)
                .expect("valid circuit");

        circuit
            .push(
                Gate::x(QubitId::new(2))
                    .expect("valid gate"),
            )
            .expect("push must succeed");

        circuit
            .push(
                Gate::x(QubitId::new(0))
                    .expect("valid gate"),
            )
            .expect("push must succeed");

        circuit
            .push(
                Gate::x(QubitId::new(2))
                    .expect("valid gate"),
            )
            .expect("push must succeed");

        let usage =
            qubit_usage(&circuit)
                .expect("usage analysis must succeed");

        assert_eq!(usage.len(), 2);

        assert_eq!(
            usage[0].qubit(),
            QubitId::new(0)
        );

        assert_eq!(
            usage[0].operation_count(),
            1
        );

        assert_eq!(
            usage[1].qubit(),
            QubitId::new(2)
        );

        assert_eq!(
            usage[1].operation_count(),
            2
        );
    }

    #[test]
    fn analysis_respects_work_budget() {
        let limits =
            QuantumIrLimits::production()
                .with_max_analysis_steps(1);

        let circuit =
            QuantumCircuit::try_new_with_limits(
                1,
                0,
                limits,
            )
            .expect("empty circuit is valid");

        assert!(
            analyze(&circuit).is_err()
        );
    }

    #[test]
    fn basic_and_full_analysis_agree() {
        let mut circuit =
            QuantumCircuit::new(2, 2)
                .expect("valid circuit");

        circuit
            .push(
                Gate::x(QubitId::new(0))
                    .expect("valid gate"),
            )
            .expect("push must succeed");

        let full =
            analyze(&circuit)
                .expect("full analysis must succeed");

        let basic =
            basic_statistics(&circuit)
                .expect("basic analysis must succeed");

        assert_eq!(
            full.operation_count(),
            basic.operation_count()
        );

        assert_eq!(
            full.depth(),
            basic.depth()
        );

        assert_eq!(
            full.measurement_count(),
            basic.measurement_count()
        );

        assert_eq!(
            full.barrier_count(),
            basic.barrier_count()
        );

        assert_eq!(
            full.reset_count(),
            basic.reset_count()
        );

        assert_eq!(
            full.parameterized_operations(),
            basic.parameterized_operations()
        );

        assert_eq!(
            full.unitary_operations(),
            basic.unitary_operations()
        );

        assert_eq!(
            full.non_unitary_operations(),
            basic.non_unitary_operations()
        );
    }
}