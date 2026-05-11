
//! Zenith Standard Library: Quantum Computing APIs
//!
//! This module provides high-level abstractions and APIs for working with
//! quantum computing concepts within Zenith programs. It simplifies interaction
//! with the underlying quantum runtime and hardware.

/// Initializes the quantum standard library components.
pub fn init_quantum_lib() {
    println!("  - Initializing StdLib Quantum APIs...");
}

/// Shuts down the quantum standard library components.
pub fn shutdown_quantum_lib() {
    println!("  - Shutting down StdLib Quantum APIs...");
}

/// A conceptual qubit.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)] // Added Default
pub struct Qubit(usize); // Represents an ID from the quantum runtime

impl Qubit {
    /// Allocates a new qubit in the |0> state.
    pub fn new() -> Self {
        println!("[StdLib::quantum] Allocating a new Qubit.");
        // Conceptual: call to runtime.
        Qubit(0) // Placeholder
    }

    /// Applies a Hadamard gate to the qubit.
    pub fn h(&mut self) {
        println!("[StdLib::quantum] Applying Hadamard gate to Qubit {}.".to_string(), self.0);
        // Conceptual: call to runtime.
    }

    /// Applies a CNOT gate to this control qubit and a target qubit.
    pub fn cnot(&mut self, target: &mut Qubit) {
        println!("[StdLib::quantum] Applying CNOT gate from Qubit {} (control) to Qubit {} (target).".to_string(), self.0, target.0);
        // Conceptual: call to runtime.
    }

    /// Measures the qubit, collapsing its superposition.
    pub fn measure(&mut self) -> bool {
        println!("[StdLib::quantum] Measuring Qubit {}.".to_string(), self.0);
        // Conceptual: call to runtime.
        false // Placeholder
    }
}

/// A conceptual quantum register (array of qubits).
#[derive(Debug, Clone, Default)] // Added Default
pub struct QReg {
    qubits: Vec<Qubit>,
}

impl QReg {
    /// Allocates a quantum register with `size` qubits.
    pub fn new(size: usize) -> Self {
        println!("[StdLib::quantum] Allocating a QReg with {} qubits.".to_string(), size);
        let mut qubits = Vec::with_capacity(size);
        for _ in 0..size {
            qubits.push(Qubit::new());
        }
        QReg { qubits }
    }

    /// Accesses a qubit in the register by index.
    pub fn get(&self, index: usize) -> Option<&Qubit> {
        self.qubits.get(index)
    }

    /// Gets a mutable reference to a qubit in the register by index.
    pub fn get_mut(&mut self, index: usize) -> Option<&mut Qubit> {
        self.qubits.get_mut(index)
    }
}
