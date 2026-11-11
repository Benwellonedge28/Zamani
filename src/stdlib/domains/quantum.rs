
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani Standard Library — Quantum Domain
//!
//! Public standard-library interface for quantum computation.
//!
//! ## Architectural ownership
//!
//! This module is a facade over the canonical Zamani quantum subsystem.
//! It does not implement another quantum language, semantic model, IR,
//! simulator, execution engine, hardware manager, or provider registry.
//!
//! Canonical authorities:
//!
//! - `crate::frontend::semantic::domains::quantum`:
//!   quantum semantic validation.
//! - `crate::quantum::ir`:
//!   canonical target-independent quantum representation.
//! - `crate::quantum::algorithms`:
//!   quantum algorithm implementations.
//! - `crate::quantum::optimization`:
//!   quantum IR transformations.
//! - `crate::quantum::routing`:
//!   logical-to-physical mapping and routing.
//! - `crate::quantum::scheduling`:
//!   quantum operation scheduling.
//! - `crate::quantum::error_correction`:
//!   quantum error-correction implementations.
//! - `crate::quantum::hardware`:
//!   quantum hardware abstractions and providers.
//! - `crate::quantum` execution subsystems:
//!   actual execution and execution lifecycle.
//!
//! This module only exposes stable standard-library access to those
//! authoritative interfaces.
//!
//! ## POCO-REAF
//!
//! Quantum programs describe computational intent independently of
//! the machine that eventually executes them.
//!
//! The same semantic program may be compiled toward compatible:
//!
//! - quantum processors;
//! - quantum simulators;
//! - hybrid classical/quantum systems;
//! - distributed quantum systems;
//! - fault-tolerant quantum systems;
//! - future quantum architectures.
//!
//! This module introduces no universal maximum for:
//!
//! - logical qubits;
//! - physical qubits;
//! - registers;
//! - operations;
//! - circuit depth;
//! - parameters;
//! - measurements;
//! - devices;
//! - processors;
//! - execution nodes;
//! - resource quantities.
//!
//! Concrete feasibility depends on the compiler, configured policies,
//! available resources, target capabilities, and execution environment.
//!
//! No finite implementation can process literally infinite input.
//! The architectural guarantee is that this facade introduces no
//! arbitrary finite semantic or hardware-capacity ceiling.
//!
//! ## Responsibility boundary
//!
//! This module does not:
//!
//! - allocate physical qubits;
//! - create quantum processor instances;
//! - choose hardware vendors;
//! - discover hardware;
//! - connect to a QPU;
//! - execute gates;
//! - generate measurement outcomes;
//! - simulate quantum states;
//! - implement noise models;
//! - implement QEC;
//! - route qubits;
//! - schedule operations;
//! - select backends;
//! - initialize the entire quantum runtime.
//!
//! Those responsibilities remain with their existing repository owners.
//!
//! ## Resource and capability semantics
//!
//! The following concepts must remain distinct:
//!
//! - Requirement: what a computation needs.
//! - Capability: what an execution target provides.
//! - Preference: what is desirable.
//! - Constraint: what must be respected.
//! - Hint: non-binding advisory information.
//! - Feasibility: whether an implementation can satisfy the requirements.
//!
//! `QuantumIrLimits`, where used, represents an explicit policy.
//! It is not a universal limit on Zamani quantum programs.
//!
//! ## Integration
//!
//! Source grammar
//!     -> canonical frontend AST
//!     -> shared semantic analysis
//!     -> quantum semantic validation
//!     -> canonical quantum IR
//!     -> compiler and target planning
//!     -> routing / scheduling / lowering
//!     -> hardware abstraction and runtime
//!     -> execution
//!
//! This module must not reverse that dependency direction.
//!
//! ## Compatibility
//!
//! All exported quantum types are re-exports of their canonical owners.
//! No duplicate standard-library quantum types are defined here.

/// Canonical quantum intermediate representation.
///
/// This is the authoritative representation of quantum computation.
/// The standard library does not define a competing representation.
pub use crate::quantum::ir as ir;

/// Canonical read-only quantum IR analysis.
pub use crate::quantum::ir::analysis;

/// Canonical classical semantic components used by quantum IR.
pub use crate::quantum::ir::classical;

/// Canonical quantum/classical dynamic-control semantics.
pub use crate::quantum::ir::control;

/// Canonical foundational IR primitives.
pub use crate::quantum::ir::core;

/// Canonical extensible quantum dialect infrastructure.
pub use crate::quantum::ir::dialect;

/// Canonical deterministic IR hashing.
pub use crate::quantum::ir::hashing;

/// Canonical provenance and source metadata.
pub use crate::quantum::ir::metadata;

/// Canonical quantum computational models.
pub use crate::quantum::ir::model;

/// Canonical structured quantum programs.
pub use crate::quantum::ir::program;

/// Canonical hardware-independent pulse semantics.
pub use crate::quantum::ir::pulse;

/// Canonical quantum-domain semantics.
pub use crate::quantum::ir::quantum;

/// Canonical abstract resource and capability semantics.
pub use crate::quantum::ir::resources;

/// Canonical semantic scheduling representation.
///
/// This is not a scheduling algorithm or runtime scheduler.
pub use crate::quantum::ir::scheduling;

// -----------------------------------------------------------------------------
// Canonical qubit identity
// -----------------------------------------------------------------------------

/// Canonical logical and physical qubit identity types.
pub use crate::quantum::ir::qubit::{
    validate_qubits,
    validate_unique_qubits,
    PhysicalQubitId,
    Qubit,
    QubitError,
    QubitId,
    QubitRange,
    QubitRangeError,
    QubitRef,
    QubitRegister,
    QubitState,
};

// -----------------------------------------------------------------------------
// Canonical gate semantics
// -----------------------------------------------------------------------------

/// Canonical standard gate representation and validation types.
pub use crate::quantum::ir::gate::{
    Gate,
    GateError,
    GateKind,
    GateParameter,
    GateResult,
    OperandCount,
};

// -----------------------------------------------------------------------------
// Canonical measurement semantics
// -----------------------------------------------------------------------------

/// Canonical measurement operations and semantic types.
pub use crate::quantum::ir::measurement::{
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

// -----------------------------------------------------------------------------
// Canonical circuit and program representations
// -----------------------------------------------------------------------------

/// Canonical circuit representation.
pub use crate::quantum::ir::model::circuit::{
    CircuitError,
    CircuitMetadata,
    QuantumCircuit,
};

/// Universal structured quantum program representation.
pub use crate::quantum::ir::program::QuantumProgram;

// -----------------------------------------------------------------------------
// Canonical model classification
// -----------------------------------------------------------------------------

/// Canonical quantum computational-model classification.
pub use crate::quantum::ir::model::ModelKind;

// -----------------------------------------------------------------------------
// Canonical IR policy and errors
// -----------------------------------------------------------------------------

/// Explicit quantum IR policy limits.
///
/// These are invocation or compilation policies, not universal
/// machine-size or language limits.
pub use crate::quantum::ir::core::limits::{
    LimitsError,
    QuantumIrLimits,
};

/// Canonical quantum IR error types.
pub use crate::quantum::ir::core::errors::{
    IrError,
    IrResult,
};

// -----------------------------------------------------------------------------
// Canonical analysis API
// -----------------------------------------------------------------------------

/// Canonical circuit and IR analysis functions and result types.
pub use crate::quantum::ir::analysis::{
    analyze,
    analyze_with_limits,
    basic_statistics,
    basic_statistics_with_limits,
    BasicCircuitStatistics,
    CircuitStatistics,
};

// -----------------------------------------------------------------------------
// Common import surface
// -----------------------------------------------------------------------------

/// Commonly used quantum standard-library imports.
///
/// Specialized functionality remains available through the canonical
/// module hierarchy and is not flattened into this prelude.
pub mod prelude {
    pub use super::{
        analyze,
        analyze_with_limits,
        basic_statistics,
        basic_statistics_with_limits,
        BasicCircuitStatistics,
        CircuitError,
        CircuitId,
        CircuitMetadata,
        CircuitStatistics,
        ClassicalBitId,
        ClassicalRegister,
        Gate,
        GateError,
        GateKind,
        GateParameter,
        GateResult,
        IrError,
        IrResult,
        IrVersion,
        LimitsError,
        Measurement,
        MeasurementBasis,
        MeasurementError,
        MeasurementGroup,
        MeasurementMode,
        ModelKind,
        OperandCount,
        OperationId,
        Parameter,
        ParameterId,
        PhysicalQubitId,
        Qubit,
        QubitError,
        QubitId,
        QubitRange,
        QubitRangeError,
        QubitRef,
        QubitRegister,
        QubitState,
        QuantumCircuit,
        QuantumIrLimits,
        QuantumProgram,
        RegionId,
        SymbolId,
        ValueId,
    };
}

// -----------------------------------------------------------------------------
// Architectural identity tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    /// Ensures that the standard library uses the canonical logical ID.
    #[test]
    fn logical_qubit_identity_is_canonical() {
        let value = QubitId::new(17);

        let canonical: crate::quantum::ir::qubit::QubitId = value;

        assert_eq!(canonical, value);
        assert_eq!(canonical.index(), 17);
    }

    /// Ensures that the standard library uses the canonical physical ID.
    #[test]
    fn physical_qubit_identity_is_canonical() {
        let value = PhysicalQubitId::new(23);

        let canonical: crate::quantum::ir::qubit::PhysicalQubitId = value;

        assert_eq!(canonical, value);
        assert_eq!(canonical.index(), 23);
    }

    /// Logical and physical identities must never become interchangeable.
    #[test]
    fn logical_and_physical_identities_are_distinct() {
        let logical = QubitRef::Logical(QubitId::new(1));
        let physical = QubitRef::Physical(PhysicalQubitId::new(1));

        assert!(logical.is_logical());
        assert!(!logical.is_physical());

        assert!(physical.is_physical());
        assert!(!physical.is_logical());
    }

    /// The standard-library IR path must resolve to the canonical IR.
    #[test]
    fn standard_library_ir_is_canonical() {
        let value = QubitId::new(7);

        let canonical: crate::quantum::ir::QubitId = value;
        let standard_library: ir::QubitId = value;

        assert_eq!(canonical, standard_library);
    }

    /// The identity abstraction must not encode a machine capacity.
    #[test]
    fn qubit_identity_does_not_encode_a_fixed_capacity() {
        let small = QubitId::new(0);
        let large = QubitId::new(usize::MAX);

        assert_ne!(small, large);
        assert_eq!(large.index(), usize::MAX);
    }

    /// The standard-library circuit type must be the canonical circuit type.
    #[test]
    fn circuit_type_is_canonical() {
        fn accepts_canonical(_: &crate::quantum::ir::model::circuit::QuantumCircuit) {}

        let _ = accepts_canonical;
    }

    /// The standard-library program type must be the canonical program type.
    #[test]
    fn program_type_is_canonical() {
        fn accepts_canonical(_: &crate::quantum::ir::program::QuantumProgram) {}

        let _ = accepts_canonical;
    }
}
