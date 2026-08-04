//! Zenith Standard Library: Quantum Computing APIs
//!
//! This module provides high-level abstractions and APIs for working with
//! quantum computing concepts within Zenith programs. It simplifies interaction
//! with the underlying quantum runtime and hardware.

use crate::runtime::quantum::{
    get_quantum_processor, init_quantum_runtime, QuantumProcessor, QubitState,
};
use std::fmt::{self, Debug};
use std::sync::{Arc, Mutex};

static mut QUANTUM_PROCESSOR_ARC: Option<Arc<Mutex<QuantumProcessor>>> = None;

/// Initializes the quantum standard library components.
pub fn init_quantum_lib() {
    println!("  - Initializing StdLib Quantum APIs...");
    // The actual runtime state is initialized by runtime::quantum::init_quantum_runtime()
    // and is stored in a static variable for access by stdlib functions.
    unsafe {
        QUANTUM_PROCESSOR_ARC = Some(crate::runtime::quantum::init_quantum_runtime());
    }
}

/// Shuts down the quantum standard library components.
pub fn shutdown_quantum_lib() {
    println!("  - Shutting down StdLib Quantum APIs...");
    unsafe {
        QUANTUM_PROCESSOR_ARC = None;
    }
}

/// A conceptual qubit.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub struct Qubit(usize); // Represents an ID from the quantum runtime

impl Qubit {
    /// Allocates a new qubit in the |0> state.
    pub fn new() -> Self {
        println!("[StdLib::quantum] Allocating a new Qubit.");
        if let Some(processor_arc) = unsafe { QUANTUM_PROCESSOR_ARC.as_ref() } {
            let mut processor = processor_arc.lock().unwrap();
            Qubit(processor.allocate_qubit())
        } else {
            println!("  Warning: Quantum Runtime not initialized, returning dummy Qubit.");
            Qubit(0)
        }
    }

    /// Deallocates the qubit.
    pub fn deallocate(&self) {
        println!("[StdLib::quantum] Deallocating Qubit {}.", self.0);
        if let Some(processor_arc) = unsafe { QUANTUM_PROCESSOR_ARC.as_ref() } {
            let mut processor = processor_arc.lock().unwrap();
            processor.deallocate_qubit(self.0);
        }
    }

    // --- Single-Qubit Gates ---
    pub fn h(&mut self) {
        println!(
            "[StdLib::quantum] Applying Hadamard gate to Qubit {}.",
            self.0
        );
        if let Some(processor_arc) = unsafe { QUANTUM_PROCESSOR_ARC.as_ref() } {
            let mut processor = processor_arc.lock().unwrap();
            processor.apply_single_qubit_gate(self.0, "H");
        }
    }
    pub fn x(&mut self) {
        println!(
            "[StdLib::quantum] Applying X (Pauli-X) gate to Qubit {}.",
            self.0
        );
        if let Some(processor_arc) = unsafe { QUANTUM_PROCESSOR_ARC.as_ref() } {
            let mut processor = processor_arc.lock().unwrap();
            processor.apply_single_qubit_gate(self.0, "X");
        }
    }
    // ... other single-qubit gates (Y, Z, S, T, etc.) would be here ...

    // --- Multi-Qubit Gates ---
    /// Applies a CNOT gate to this control qubit and a target qubit.
    pub fn cnot(&mut self, target: &mut Qubit) {
        println!(
            "[StdLib::quantum] Applying CNOT gate (Control: {}, Target: {}).",
            self.0, target.0
        );
        if let Some(processor_arc) = unsafe { QUANTUM_PROCESSOR_ARC.as_ref() } {
            let mut processor = processor_arc.lock().unwrap();
            processor.apply_cnot_gate(self.0, target.0);
        }
    }
    // ... other multi-qubit gates (CCNOT, SWAP, etc.) would be here ...

    /// Measures the qubit, collapsing its superposition.
    pub fn measure(&mut self) -> bool {
        println!("[StdLib::quantum] Measuring Qubit {}.", self.0);
        if let Some(processor_arc) = unsafe { QUANTUM_PROCESSOR_ARC.as_ref() } {
            let mut processor = processor_arc.lock().unwrap();
            processor.measure_qubit(self.0)
        } else {
            false // Default for uninitialized runtime
        }
    }

    /// Get the conceptual state of the qubit.
    pub fn get_state(&self) -> QubitState {
        if let Some(processor_arc) = unsafe { QUANTUM_PROCESSOR_ARC.as_ref() } {
            let processor = processor_arc.lock().unwrap();
            processor
                .allocated_qubits
                .get(&self.0)
                .map(|q| q.state.clone())
                .unwrap_or(QubitState::Mixed)
        } else {
            QubitState::Mixed
        }
    }

    /// Checks if this qubit is conceptually entangled with another.
    pub fn is_entangled_with(&self, other: &Qubit) -> bool {
        if let Some(processor_arc) = unsafe { QUANTUM_PROCESSOR_ARC.as_ref() } {
            let processor = processor_arc.lock().unwrap();
            processor
                .allocated_qubits
                .get(&self.0)
                .map_or(false, |q| q.entangled_with.contains(&other.0))
        } else {
            false
        }
    }
}

/// A conceptual quantum register (array of qubits).
#[derive(Debug, Clone)]
pub struct QReg {
    qubits: Vec<Qubit>,
}

impl QReg {
    /// Allocates a quantum register with `size` qubits.
    pub fn new(size: usize) -> Self {
        println!("[StdLib::quantum] Allocating a QReg with {} qubits.", size);
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

    /// Deallocates all qubits in the register.
    pub fn deallocate_all(&self) {
        for qubit in &self.qubits {
            qubit.deallocate();
        }
    }
}

// Helper for conceptual random function
fn rand() -> f64 {
    {
        use std::time::{SystemTime, UNIX_EPOCH};
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.subsec_nanos() as f64)
            .unwrap_or(0.0);
        (nanos / 1_000_000_000.0).fract()
    }
}
