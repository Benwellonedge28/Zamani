//! Zamani Quantum Intermediate Representation — Circuit
//!
//! Canonical container for a quantum program.
//!
//! A `QuantumCircuit` owns the ordered sequence of quantum operations together
//! with circuit-level metadata such as the number of qubits and classical
//! registers.
//!
//! Design goals:
//! - deterministic operation ordering;
//! - explicit qubit/classical-register ownership;
//! - validation at the IR boundary;
//! - safe construction and mutation;
//! - measurement/barrier awareness;
//! - compatibility with optimization passes;
//! - no hardware-specific assumptions;
//! - no dependency on a particular quantum backend.

use std::fmt;

use super::gate::{Gate, GateError};

// -----------------------------------------------------------------------------
// Errors
// -----------------------------------------------------------------------------

/// Errors that can occur while constructing or modifying a circuit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CircuitError {
    /// A qubit index is outside the circuit's declared range.
    QubitOutOfRange {
        qubit: usize,
        num_qubits: usize,
    },

    /// A classical register index is outside the circuit's declared range.
    ClassicalBitOutOfRange {
        bit: usize,
        num_classical_bits: usize,
    },

    /// A gate contains no operands when operands are required.
    MissingOperands,

    /// The gate contains the same qubit more than once.
    DuplicateQubit {
        qubit: usize,
    },

    /// A gate is incompatible with the circuit.
    InvalidGate {
        message: String,
    },

    /// The circuit cannot be modified in its current state.
    InvalidCircuit {
        message: String,
    },

    /// The requested operation index does not exist.
    OperationOutOfRange {
        index: usize,
        len: usize,
    },

    /// Classical measurement target is invalid.
    InvalidMeasurementTarget {
        bit: usize,
    },

    /// The supplied gate rejected its own construction.
    GateError(String),
}

impl fmt::Display for CircuitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
                write!(f, "qubit {qubit} appears more than once")
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

    /// Optional source language/module name.
    pub source: Option<String>,

    /// Optional compiler/runtime version.
    pub compiler_version: Option<String>,

    /// Whether the circuit is intended for fault-tolerant execution.
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
// Circuit
// -----------------------------------------------------------------------------

/// Canonical Zamani quantum circuit.
///
/// Operations are stored in execution order. Optimization passes may create
/// a new circuit or mutate a circuit through the controlled APIs provided here.
#[derive(Debug, Clone, PartialEq)]
pub struct QuantumCircuit {
    /// Number of logical qubits.
    num_qubits: usize,

    /// Number of classical bits/register slots.
    num_classical_bits: usize,

    /// Ordered quantum operations.
    operations: Vec<Gate>,

    /// Circuit metadata.
    metadata: CircuitMetadata,
}

impl QuantumCircuit {
    /// Creates an empty quantum circuit.
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

    /// Creates a circuit from an existing operation list.
    pub fn from_operations(
        num_qubits: usize,
        num_classical_bits: usize,
        operations: Vec<Gate>,
    ) -> Result<Self, CircuitError> {
        let mut circuit =
            Self::new(num_qubits, num_classical_bits);

        for gate in operations {
            circuit.push(gate)?;
        }

        Ok(circuit)
    }

    // -------------------------------------------------------------------------
    // Accessors
    // -------------------------------------------------------------------------

    /// Returns the number of logical qubits.
    pub fn num_qubits(&self) -> usize {
        self.num_qubits
    }

    /// Returns the number of classical bits.
    pub fn num_classical_bits(&self) -> usize {
        self.num_classical_bits
    }

    /// Returns the number of operations.
    pub fn len(&self) -> usize {
        self.operations.len()
    }

    /// Returns true when the circuit contains no operations.
    pub fn is_empty(&self) -> bool {
        self.operations.is_empty()
    }

    /// Returns all operations.
    pub fn operations(&self) -> &[Gate] {
        &self.operations
    }

    /// Returns mutable access to operations.
    ///
    /// Callers modifying the returned slice are responsible for preserving
    /// circuit invariants. Prefer `push`, `insert`, `replace`, and `remove`
    /// where possible.
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

    /// Returns one operation.
    pub fn get(
        &self,
        index: usize,
    ) -> Option<&Gate> {
        self.operations.get(index)
    }

    // -------------------------------------------------------------------------
    // Construction
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

    /// Inserts an operation at a specific position.
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

    /// Replaces one operation.
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

        let old =
            std::mem::replace(
                &mut self.operations[index],
                gate,
            );

        Ok(old)
    }

    /// Removes one operation.
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

    /// Removes all operations while preserving circuit dimensions.
    pub fn clear(&mut self) {
        self.operations.clear();
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

    /// Validates a single gate against this circuit.
    pub fn validate_gate(
        &self,
        gate: &Gate,
    ) -> Result<(), CircuitError> {
        gate.validate()?;

        for qubit in gate.qubits() {
            if *qubit >= self.num_qubits {
                return Err(
                    CircuitError::QubitOutOfRange {
                        qubit: *qubit,
                        num_qubits: self.num_qubits,
                    },
                );
            }
        }

        validate_unique_qubits(gate.qubits())?;

        if let Some(classical_bit) =
            gate.classical_target()
        {
            if classical_bit >= self.num_classical_bits {
                return Err(
                    CircuitError::ClassicalBitOutOfRange {
                        bit: classical_bit,
                        num_classical_bits:
                            self.num_classical_bits,
                    },
                );
            }
        }

        Ok(())
    }

    // -------------------------------------------------------------------------
    // Circuit analysis
    // -------------------------------------------------------------------------

    /// Returns the number of measurement operations.
    pub fn measurement_count(&self) -> usize {
        self.operations
            .iter()
            .filter(|gate| gate.is_measurement())
            .count()
    }

    /// Returns the number of barrier operations.
    pub fn barrier_count(&self) -> usize {
        self.operations
            .iter()
            .filter(|gate| gate.is_barrier())
            .count()
    }

    /// Returns the circuit depth using a simple logical-qubit model.
    ///
    /// Each operation occupies one layer after the latest layer of any qubit
    /// it touches. This is deliberately hardware-independent.
    pub fn depth(&self) -> usize {
        if self.operations.is_empty() {
            return 0;
        }

        let mut qubit_depth =
            vec![0usize; self.num_qubits];

        for gate in &self.operations {
            let latest = gate
                .qubits()
                .iter()
                .map(|q| qubit_depth[*q])
                .max()
                .unwrap_or(0);

            let next = latest + 1;

            for qubit in gate.qubits() {
                qubit_depth[*qubit] = next;
            }
        }

        qubit_depth
            .into_iter()
            .max()
            .unwrap_or(0)
    }

    /// Returns the number of gates acting on a particular qubit.
    pub fn qubit_gate_count(
        &self,
        qubit: usize,
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

    /// Returns whether the circuit contains any measurements.
    pub fn has_measurements(&self) -> bool {
        self.operations
            .iter()
            .any(|gate| gate.is_measurement())
    }

    /// Returns whether the circuit contains barriers.
    pub fn has_barriers(&self) -> bool {
        self.operations
            .iter()
            .any(|gate| gate.is_barrier())
    }

    // -------------------------------------------------------------------------
    // Qubit helpers
    // -------------------------------------------------------------------------

    /// Validates a logical qubit index.
    pub fn validate_qubit(
        &self,
        qubit: usize,
    ) -> Result<(), CircuitError> {
        if qubit >= self.num_qubits {
            return Err(
                CircuitError::QubitOutOfRange {
                    qubit,
                    num_qubits: self.num_qubits,
                },
            );
        }

        Ok(())
    }

    /// Returns all operations touching a logical qubit.
    pub fn operations_on_qubit(
        &self,
        qubit: usize,
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

    // -------------------------------------------------------------------------
    // Optimization support
    // -------------------------------------------------------------------------

    /// Removes an identity operation at an index.
    ///
    /// This is useful for optimization passes that have already established
    /// that the operation is semantically redundant.
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

    /// Returns an owned copy of the operation list.
    pub fn into_operations(self) -> Vec<Gate> {
        self.operations
    }
}

// -----------------------------------------------------------------------------
// Utility functions
// -----------------------------------------------------------------------------

fn validate_unique_qubits(
    qubits: &[usize],
) -> Result<(), CircuitError> {
    for (index, qubit) in qubits.iter().enumerate() {
        if qubits[index + 1..].contains(qubit) {
            return Err(
                CircuitError::DuplicateQubit {
                    qubit: *qubit,
                },
            );
        }
    }

    Ok(())
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn x(qubit: usize) -> Gate {
        Gate::x(qubit)
            .expect("X gate should be valid")
    }

    fn h(qubit: usize) -> Gate {
        Gate::h(qubit)
            .expect("H gate should be valid")
    }

    #[test]
    fn creates_empty_circuit() {
        let circuit =
            QuantumCircuit::new(4, 4);

        assert_eq!(circuit.num_qubits(), 4);
        assert_eq!(circuit.num_classical_bits(), 4);
        assert!(circuit.is_empty());
        assert_eq!(circuit.len(), 0);
    }

    #[test]
    fn pushes_gate() {
        let mut circuit =
            QuantumCircuit::new(2, 2);

        circuit
            .push(x(0))
            .expect("X should fit circuit");

        assert_eq!(circuit.len(), 1);
    }

    #[test]
    fn rejects_out_of_range_qubit() {
        let mut circuit =
            QuantumCircuit::new(1, 1);

        let result =
            circuit.push(x(1));

        assert!(matches!(
            result,
            Err(
                CircuitError::QubitOutOfRange {
                    qubit: 1,
                    num_qubits: 1
                }
            )
        ));
    }

    #[test]
    fn insert_and_remove_work() {
        let mut circuit =
            QuantumCircuit::new(2, 2);

        circuit.push(x(0)).unwrap();
        circuit.push(h(1)).unwrap();

        circuit
            .insert(1, x(1))
            .expect("insert should succeed");

        assert_eq!(circuit.len(), 3);

        let removed =
            circuit.remove(1)
                .expect("remove should succeed");

        assert_eq!(removed, x(1));
        assert_eq!(circuit.len(), 2);
    }

    #[test]
    fn replace_returns_old_gate() {
        let mut circuit =
            QuantumCircuit::new(1, 1);

        circuit.push(x(0)).unwrap();

        let old =
            circuit
                .replace(0, h(0))
                .expect("replace should succeed");

        assert_eq!(old, x(0));
        assert_eq!(circuit.get(0), Some(&h(0)));
    }

    #[test]
    fn clear_preserves_dimensions() {
        let mut circuit =
            QuantumCircuit::new(8, 8);

        circuit.push(x(0)).unwrap();
        circuit.clear();

        assert!(circuit.is_empty());
        assert_eq!(circuit.num_qubits(), 8);
        assert_eq!(circuit.num_classical_bits(), 8);
    }

    #[test]
    fn depth_tracks_qubits() {
        let mut circuit =
            QuantumCircuit::new(2, 2);

        circuit.push(x(0)).unwrap();
        circuit.push(x(1)).unwrap();
        circuit.push(h(0)).unwrap();

        assert_eq!(circuit.depth(), 2);
    }

    #[test]
    fn qubit_gate_count_is_correct() {
        let mut circuit =
            QuantumCircuit::new(2, 2);

        circuit.push(x(0)).unwrap();
        circuit.push(h(1)).unwrap();
        circuit.push(x(0)).unwrap();

        assert_eq!(
            circuit.qubit_gate_count(0).unwrap(),
            2
        );

        assert_eq!(
            circuit.qubit_gate_count(1).unwrap(),
            1
        );
    }

    #[test]
    fn operations_on_qubit_returns_expected_gates() {
        let mut circuit =
            QuantumCircuit::new(2, 2);

        circuit.push(x(0)).unwrap();
        circuit.push(h(1)).unwrap();
        circuit.push(x(0)).unwrap();

        let operations =
            circuit.operations_on_qubit(0)
                .unwrap();

        assert_eq!(operations.len(), 2);
        assert_eq!(operations[0], &x(0));
        assert_eq!(operations[1], &x(0));
    }

    #[test]
    fn metadata_is_preserved() {
        let metadata = CircuitMetadata {
            name: Some("bell".into()),
            source: Some("zamani".into()),
            compiler_version: Some("0.1".into()),
            fault_tolerant: false,
        };

        let circuit =
            QuantumCircuit::with_metadata(
                2,
                2,
                metadata.clone(),
            );

        assert_eq!(
            circuit.metadata(),
            &metadata
        );
    }

    #[test]
    fn from_operations_validates_every_gate() {
        let operations =
            vec![x(0), h(1)];

        let circuit =
            QuantumCircuit::from_operations(
                2,
                2,
                operations,
            )
            .expect("operations should be valid");

        assert_eq!(circuit.len(), 2);
    }

    #[test]
    fn deterministic_clone() {
        let mut circuit =
            QuantumCircuit::new(2, 2);

        circuit.push(x(0)).unwrap();
        circuit.push(h(1)).unwrap();

        assert_eq!(circuit.clone(), circuit);
    }
}