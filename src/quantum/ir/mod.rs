//! Zamani Quantum Intermediate Representation.
//!
//! This module defines the hardware-independent representation of quantum
//! programs. The IR is intentionally divided into:
//!
//! - `qubits`       — logical and physical qubit identities;
//! - `gate`         — quantum operations;
//! - `measurement` — quantum-to-classical measurement semantics;
//! - `circuit`      — complete quantum circuit/program containers.
//!
//! Higher compiler stages should operate on these abstractions rather than
//! directly on backend-specific hardware representations.

// -----------------------------------------------------------------------------
// Modules
// -----------------------------------------------------------------------------

pub mod circuit;
pub mod gate;
pub mod measurement;
pub mod qubits;

// -----------------------------------------------------------------------------
// Core IR exports
// -----------------------------------------------------------------------------

pub use circuit::QuantumCircuit;

pub use gate::{
    Gate,
    GateError,
    GateKind,
    GateParameter,
};

pub use measurement::{
    measure,
    measure_x,
    measure_y,
    ClassicalBitId,
    ClassicalRegister,
    Measurement,
    MeasurementBasis,
    MeasurementError,
    MeasurementGroup,
    MeasurementMode,
};

pub use qubits::{
    validate_qubits,
    validate_unique_qubits,
    PhysicalQubitId,
    Qubit,
    QubitError,
    QubitId,
    QubitRegister,
    QubitState,
};

// -----------------------------------------------------------------------------
// Prelude
// -----------------------------------------------------------------------------

/// Common quantum IR types.
///
/// Compiler passes can import this prelude when they need the primary IR
/// abstractions without depending on individual implementation modules.
pub mod prelude {
    pub use super::{
        measure,
        measure_x,
        measure_y,
        ClassicalBitId,
        ClassicalRegister,
        Gate,
        GateError,
        GateKind,
        GateParameter,
        Measurement,
        MeasurementBasis,
        MeasurementError,
        MeasurementGroup,
        MeasurementMode,
        PhysicalQubitId,
        Qubit,
        QubitError,
        QubitId,
        QubitRegister,
        QubitState,
        QuantumCircuit,
    };
}