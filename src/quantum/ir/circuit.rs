//! Zamani Quantum Intermediate Representation — Circuit
//!
//! Hardware-independent container for an ordered quantum program.
//!
//! A `QuantumCircuit` owns:
//! - logical qubit count;
//! - classical-bit count;
//! - ordered quantum operations;
//! - circuit metadata.
//!
//! Physical hardware mapping, calibration, scheduling, and backend-specific
//! constraints belong to later compiler stages.

use std::fmt;

use super::gate::{Gate, GateError};
use super::qubits::QubitId;

// -----------------------------------------------------------------------------
// Errors
// -----------------------------------------------------------------------------

/// Errors produced while constructing or modifying a quantum circuit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CircuitError {
    QubitOutOfRange {
        qubit: QubitId,
        num_qubits: usize,
    },

    ClassicalBitOutOfRange {
        bit: usize,
        num_classical_bits: usize,
    },

    MissingOperands,

    DuplicateQubit {
        qubit: QubitId,
    },

    InvalidGate {
        message: String,
    },

    InvalidCircuit {
        message: String,
    },

    OperationOutOfRange {
        index: usize,
        len: usize,
    },

    InvalidMeasurementTarget {
        bit: usize,
    },

    GateError(String),
}

impl fmt::Display for CircuitError {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::QubitOutOfRange {
                qubit,
                num_qubits,
            } => write!(
                f,
                "qubit {qubit} is outside circuit range 0..{num_qubits}"
            ),

            Self::ClassicalBitOutOfRange {
                bit,
                num_classical_bits,
            } => write!(
                f,
                "classical bit {bit} is outside circuit range 0..{num_classical_bits}"
            ),

            Self::MissingOperands => {
                write!(f, "gate has no operands")
            }

            Self::DuplicateQubit { qubit } => {
                write!(
                    f,
                    "qubit {qubit} appears more than once"
                )
            }

            Self::InvalidGate { message } => {
                write!(f, "invalid gate: {message}")
            }

            Self::InvalidCircuit { message } => {
                write!(f, "invalid circuit: {message}")
            }

            Self::OperationOutOfRange { index, len } => {
                write!(
                    f,
                    "operation index {index} is outside circuit length {len}"
                )
            }

            Self::InvalidMeasurementTarget { bit } => {
                write!(
                    f,
                    "invalid measurement target classical bit {bit}"
                )
            }

            Self::GateError(message) => {
                write!(f, "gate error: {message}")
            }
        }
    }
}

impl std::error::Error for CircuitError {}

impl From<GateError> for CircuitError {
    fn from(error: GateError) -> Self {
        Self::GateError(error.to_string())
    }
}

// -----------------------------------------------------------------------------
// Circuit metadata
// -----------------------------------------------------------------------------

/// Metadata associated with a quantum circuit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CircuitMetadata {
    /// Optional human-readable circuit name.
    pub name: Option<String>,

    /// Optional source language/module.
    pub source: Option<String>,

    /// Optional compiler version.
    pub compiler_version: Option<String>,

    /// Whether the circuit targets fault-tolerant execution.
    pub fault_tolerant: bool,
}

impl Default for CircuitMetadata {
    fn default() -> Self {
        Self {
            name: None,
            source: None,
            compiler_version: None,
            fault_tolerant: false,
        }
    }
}

// -----------------------------------------------------------------------------
// Quantum circuit
// -----------------------------------------------------------------------------

/// Canonical Zamani quantum circuit.
///
/// Operations are stored in execution order.
#[derive(Debug, Clone, PartialEq)]
pub struct QuantumCircuit {
    num_qubits: usize,
    num_classical_bits: usize,
    operations: Vec<Gate>,
    metadata: CircuitMetadata,
}

impl QuantumCircuit {
    /// Creates an empty circuit.
    pub fn new(
        num_qubits: usize,
        num_classical_bits: usize,
    ) -> Self {
        Self {
            num_qubits,
            num_classical_bits,
            operations: Vec::new(),
            metadata: CircuitMetadata::default(),
        }
    }

    /// Creates an empty circuit with metadata.
    pub fn with_metadata(
        num_qubits: usize,
        num_classical_bits: usize,
        metadata: CircuitMetadata,
    ) -> Self {
        Self {
            num_qubits,
            num_classical_bits,
            operations: Vec::new(),
            metadata,
        }
    }

    /// Creates a circuit from an existing sequence of operations.
    pub fn from_operations(
        num_qubits: usize,
        num_classical_bits: usize,
        operations: Vec<Gate>,
    ) -> Result<Self, CircuitError> {
        let mut circuit =
            Self::new(
                num_qubits,
                num_classical_bits,
            );

        for gate in operations {
            circuit.push(gate)?;
        }

        Ok(circuit)
    }

    // -------------------------------------------------------------------------
    // Accessors
    // -------------------------------------------------------------------------

    /// Number of logical qubits.
    pub const fn num_qubits(&self) -> usize {
        self.num_qubits
    }

    /// Number of classical bits.
    pub const fn num_classical_bits(&self) -> usize {
        self.num_classical_bits
    }

    /// Number of operations.
    pub fn len(&self) -> usize {
        self.operations.len()
    }

    /// Whether the circuit has no operations.
    pub fn is_empty(&self) -> bool {
        self.operations.is_empty()
    }

    /// Returns all operations.
    pub fn operations(&self) -> &[Gate] {
        &self.operations
    }

    /// Returns mutable access to the operation list.
    ///
    /// Callers are responsible for preserving circuit invariants.
    pub fn operations_mut(&mut self) -> &mut [Gate] {
        &mut self.operations
    }

    /// Returns circuit metadata.
    pub fn metadata(&self) -> &CircuitMetadata {
        &self.metadata
    }

    /// Returns mutable circuit metadata.
    pub fn metadata_mut(&mut self) -> &mut CircuitMetadata {
        &mut self.metadata
    }

    /// Returns an operation by index.
    pub fn get(
        &self,
        index: usize,
    ) -> Option<&Gate> {
        self.operations.get(index)
    }

    // -------------------------------------------------------------------------
    // Construction / mutation
    // -------------------------------------------------------------------------

    /// Appends an operation.
    pub fn push(
        &mut self,
        gate: Gate,
    ) -> Result<(), CircuitError> {
        self.validate_gate(&gate)?;
        self.operations.push(gate);
        Ok(())
    }

    /// Inserts an operation.
    pub fn insert(
        &mut self,
        index: usize,
        gate: Gate,
    ) -> Result<(), CircuitError> {
        if index > self.operations.len() {
            return Err(
                CircuitError::OperationOutOfRange {
                    index,
                    len: self.operations.len(),
                },
            );
        }

        self.validate_gate(&gate)?;
        self.operations.insert(index, gate);

        Ok(())
    }

    /// Replaces an existing operation.
    pub fn replace(
        &mut self,
        index: usize,
        gate: Gate,
    ) -> Result<Gate, CircuitError> {
        if index >= self.operations.len() {
            return Err(
                CircuitError::OperationOutOfRange {
                    index,
                    len: self.operations.len(),
                },
            );
        }

        self.validate_gate(&gate)?;

        Ok(std::mem::replace(
            &mut self.operations[index],
            gate,
        ))
    }

    /// Removes an operation.
    pub fn remove(
        &mut self,
        index: usize,
    ) -> Result<Gate, CircuitError> {
        if index >= self.operations.len() {
            return Err(
                CircuitError::OperationOutOfRange {
                    index,
                    len: self.operations.len(),
                },
            );
        }

        Ok(self.operations.remove(index))
    }

    /// Removes all operations.
    pub fn clear(&mut self) {
        self.operations.clear();
    }

    /// Consumes the circuit and returns its operations.
    pub fn into_operations(self) -> Vec<Gate> {
        self.operations
    }

    // -------------------------------------------------------------------------
    // Validation
    // -------------------------------------------------------------------------

    /// Validates the complete circuit.
    pub fn validate(&self) -> Result<(), CircuitError> {
        for gate in &self.operations {
            self.validate_gate(gate)?;
        }

        Ok(())
    }

    /// Validates a gate against this circuit.
    pub fn validate_gate(
        &self,
        gate: &Gate,
    ) -> Result<(), CircuitError> {
        gate.validate()?;

        if gate.qubits().is_empty() {
            return Err(
                CircuitError::MissingOperands
            );
        }

        for qubit in gate.qubits() {
            self.validate_qubit(*qubit)?;
        }

        if let Some(classical_bit) =
            gate.classical_target()
        {
            self.validate_classical_bit(
                classical_bit.index(),
            )?;
        }

        Ok(())
    }

    /// Validates a logical qubit.
    pub fn validate_qubit(
        &self,
        qubit: QubitId,
    ) -> Result<(), CircuitError> {
        if qubit.index() >= self.num_qubits {
            return Err(
                CircuitError::QubitOutOfRange {
                    qubit,
                    num_qubits: self.num_qubits,
                },
            );
        }

        Ok(())
    }

    /// Validates a classical-bit index.
    pub fn validate_classical_bit(
        &self,
        bit: usize,
    ) -> Result<(), CircuitError> {
        if bit >= self.num_classical_bits {
            return Err(
                CircuitError::ClassicalBitOutOfRange {
                    bit,
                    num_classical_bits:
                        self.num_classical_bits,
                },
            );
        }

        Ok(())
    }

    // -------------------------------------------------------------------------
    // Analysis
    // -------------------------------------------------------------------------

    /// Number of measurement operations.
    pub fn measurement_count(&self) -> usize {
        self.operations
            .iter()
            .filter(|gate| gate.is_measurement())
            .count()
    }

    /// Number of barriers.
    pub fn barrier_count(&self) -> usize {
        self.operations
            .iter()
            .filter(|gate| gate.is_barrier())
            .count()
    }

    /// Whether the circuit contains measurements.
    pub fn has_measurements(&self) -> bool {
        self.operations
            .iter()
            .any(|gate| gate.is_measurement())
    }

    /// Whether the circuit contains barriers.
    pub fn has_barriers(&self) -> bool {
        self.operations
            .iter()
            .any(|gate| gate.is_barrier())
    }

    /// Number of operations touching a logical qubit.
    pub fn qubit_gate_count(
        &self,
        qubit: QubitId,
    ) -> Result<usize, CircuitError> {
        self.validate_qubit(qubit)?;

        Ok(self
            .operations
            .iter()
            .filter(|gate| {
                gate.qubits().contains(&qubit)
            })
            .count())
    }

    /// Returns operations acting on a logical qubit.
    pub fn operations_on_qubit(
        &self,
        qubit: QubitId,
    ) -> Result<Vec<&Gate>, CircuitError> {
        self.validate_qubit(qubit)?;

        Ok(self
            .operations
            .iter()
            .filter(|gate| {
                gate.qubits().contains(&qubit)
            })
            .collect())
    }

    /// Calculates hardware-independent logical circuit depth.
    pub fn depth(&self) -> usize {
        if self.operations.is_empty() {
            return 0;
        }

        let mut depths =
            vec![0usize; self.num_qubits];

        for gate in &self.operations {
            let latest = gate
                .qubits()
                .iter()
                .map(|qubit| {
                    depths[qubit.index()]
                })
                .max()
                .unwrap_or(0);

            let next = latest + 1;

            for qubit in gate.qubits() {
                depths[qubit.index()] = next;
            }
        }

        depths.into_iter().max().unwrap_or(0)
    }

    // -------------------------------------------------------------------------
    // Optimization support
    // -------------------------------------------------------------------------

    /// Removes an operation if it is an identity.
    pub fn remove_if_identity(
        &mut self,
        index: usize,
    ) -> Result<bool, CircuitError> {
        if index >= self.operations.len() {
            return Err(
                CircuitError::OperationOutOfRange {
                    index,
                    len: self.operations.len(),
                },
            );
        }

        if self.operations[index].is_identity() {
            self.operations.remove(index);
            return Ok(true);
        }

        Ok(false)
    }
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn q(index: usize) -> QubitId {
        QubitId::new(index)
    }

    fn c(index: usize) -> super::super::measurement::ClassicalBitId {
        super::super::measurement::ClassicalBitId::new(index)
    }

    #[test]
    fn creates_empty_circuit() {
        let circuit =
            QuantumCircuit::new(4, 4);

        assert_eq!(
            circuit.num_qubits(),
            4
        );

        assert_eq!(
            circuit.num_classical_bits(),
            4
        );

        assert!(circuit.is_empty());
    }

    #[test]
    fn pushes_gate() {
        let mut circuit =
            QuantumCircuit::new(2, 2);

        circuit
            .push(
                Gate::x(q(0)).unwrap()
            )
            .unwrap();

        assert_eq!(circuit.len(), 1);
        assert_eq!(
            circuit.get(0).unwrap().qubits(),
            &[q(0)]
        );
    }

    #[test]
    fn rejects_out_of_range_qubit() {
        let mut circuit =
            QuantumCircuit::new(2, 2);

        let result =
            circuit.push(
                Gate::x(q(2)).unwrap()
            );

        assert!(matches!(
            result,
            Err(
                CircuitError::QubitOutOfRange {
                    ..
                }
            )
        ));
    }

    #[test]
    fn accepts_two_qubit_gate() {
        let mut circuit =
            QuantumCircuit::new(2, 2);

        circuit
            .push(
                Gate::cx(
                    q(0),
                    q(1),
                )
                .unwrap(),
            )
            .unwrap();

        assert_eq!(circuit.len(), 1);
    }

    #[test]
    fn rejects_out_of_range_measurement_target() {
        let mut circuit =
            QuantumCircuit::new(2, 2);

        let gate =
            Gate::measurement(
                q(0),
                c(2),
            )
            .unwrap();

        let result =
            circuit.push(gate);

        assert!(matches!(
            result,
            Err(
                CircuitError::ClassicalBitOutOfRange {
                    ..
                }
            )
        ));
    }

    #[test]
    fn counts_measurements() {
        let mut circuit =
            QuantumCircuit::new(2, 2);

        circuit
            .push(
                Gate::measurement(
                    q(0),
                    c(0),
                )
                .unwrap(),
            )
            .unwrap();

        circuit
            .push(
                Gate::measurement(
                    q(1),
                    c(1),
                )
                .unwrap(),
            )
            .unwrap();

        assert_eq!(
            circuit.measurement_count(),
            2
        );

        assert!(circuit.has_measurements());
    }

    #[test]
    fn counts_barriers() {
        let mut circuit =
            QuantumCircuit::new(3, 3);

        circuit
            .push(
                Gate::barrier(vec![
                    q(0),
                    q(1),
                    q(2),
                ])
                .unwrap(),
            )
            .unwrap();

        assert_eq!(
            circuit.barrier_count(),
            1
        );

        assert!(circuit.has_barriers());
    }

    #[test]
    fn calculates_depth() {
        let mut circuit =
            QuantumCircuit::new(2, 2);

        circuit
            .push(
                Gate::x(q(0)).unwrap()
            )
            .unwrap();

        circuit
            .push(
                Gate::x(q(1)).unwrap()
            )
            .unwrap();

        circuit
            .push(
                Gate::cx(
                    q(0),
                    q(1),
                )
                .unwrap(),
            )
            .unwrap();

        assert_eq!(
            circuit.depth(),
            2
        );
    }

    #[test]
    fn counts_operations_on_qubit() {
        let mut circuit =
            QuantumCircuit::new(2, 2);

        circuit
            .push(
                Gate::x(q(0)).unwrap()
            )
            .unwrap();

        circuit
            .push(
                Gate::h(q(1)).unwrap()
            )
            .unwrap();

        circuit
            .push(
                Gate::cx(
                    q(0),
                    q(1),
                )
                .unwrap(),
            )
            .unwrap();

        assert_eq!(
            circuit
                .qubit_gate_count(q(0))
                .unwrap(),
            2
        );

        assert_eq!(
            circuit
                .qubit_gate_count(q(1))
                .unwrap(),
            2
        );
    }

    #[test]
    fn replaces_operation() {
        let mut circuit =
            QuantumCircuit::new(1, 1);

        circuit
            .push(
                Gate::x(q(0)).unwrap()
            )
            .unwrap();

        let old =
            circuit
                .replace(
                    0,
                    Gate::h(q(0)).unwrap(),
                )
                .unwrap();

        assert_eq!(
            old.kind(),
            super::super::gate::GateKind::X
        );

        assert_eq!(
            circuit
                .get(0)
                .unwrap()
                .kind(),
            super::super::gate::GateKind::H
        );
    }

    #[test]
    fn removes_operation() {
        let mut circuit =
            QuantumCircuit::new(1, 1);

        circuit
            .push(
                Gate::x(q(0)).unwrap()
            )
            .unwrap();

        let removed =
            circuit.remove(0).unwrap();

        assert_eq!(
            removed.kind(),
            super::super::gate::GateKind::X
        );

        assert!(circuit.is_empty());
    }

    #[test]
    fn removes_identity() {
        let mut circuit =
            QuantumCircuit::new(1, 1);

        circuit
            .push(
                Gate::id(q(0)).unwrap()
            )
            .unwrap();

        assert!(
            circuit
                .remove_if_identity(0)
                .unwrap()
        );

        assert!(circuit.is_empty());
    }

    #[test]
    fn metadata_is_preserved() {
        let mut metadata =
            CircuitMetadata::default();

        metadata.name =
            Some("Bell State".into());

        let circuit =
            QuantumCircuit::with_metadata(
                2,
                2,
                metadata,
            );

        assert_eq!(
            circuit.metadata().name.as_deref(),
            Some("Bell State")
        );
    }
}