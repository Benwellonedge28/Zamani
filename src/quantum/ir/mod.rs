//! Zamani Quantum Intermediate Representation.
//!
//! Canonical, hardware-independent semantic IR for quantum computation.
//!
//! # Architectural contract
//!
//! `quantum::ir` is the stable semantic boundary between the Zamani quantum
//! frontend and every downstream quantum compiler/execution subsystem.
//!
//! The IR describes:
//!
//!     WHAT computation means
//!
//! It does not decide:
//!
//! - which physical machine executes it;
//! - which physical qubits are selected;
//! - how logical qubits are routed;
//! - how operations are scheduled;
//! - which hardware-native instruction is selected;
//! - which calibration is applied;
//! - how pulses are synthesized;
//! - how a QPU is contacted;
//! - how quantum state is simulated;
//! - how QEC syndromes are decoded;
//! - which optimization algorithm is used;
//! - how source syntax is parsed.
//!
//! Those responsibilities belong to downstream systems.
//!
//! # Write once, scale everywhere
//!
//! A Zamani quantum program is represented once at the canonical semantic
//! boundary and can subsequently be lowered to any compatible target.
//!
//! The IR therefore contains no architectural maximum for:
//!
//! - qubits;
//! - classical bits;
//! - operations;
//! - registers;
//! - regions;
//! - blocks;
//! - circuit depth;
//! - topology;
//! - gate arity;
//! - quantum architecture;
//! - vendor;
//! - execution technology.
//!
//! Concrete limits belong to explicit resource/security policies and target
//! capabilities. They are not language or IR architectural limits.
//!
//! "Infinity" in the architectural requirement means:
//!
//! > no artificial finite machine-size ceiling is encoded by the IR.
//!
//! Every concrete artifact remains finite because a compiler, process,
//! address space and execution target are finite.
//!
//! # Semantic separation
//!
//! ```text
//! Zamani source
//!       │
//!       ▼
//! quantum frontend
//!       │
//!       ▼
//! ┌─────────────────────────────┐
//! │       quantum::ir           │
//! │                             │
//! │ canonical semantic WHAT     │
//! └──────────────┬──────────────┘
//!                │
//!       ┌────────┼─────────┐
//!       ▼        ▼         ▼
//! optimization mapping scheduling
//!       │        │         │
//!       └────────┼─────────┘
//!                ▼
//!        target / hardware
//!                │
//!                ▼
//!             backend
//!                │
//!                ▼
//!            execution
//! ```
//!
//! The dependency direction must never be reversed.
//!
//! In particular, this module must not depend on:
//!
//! - `quantum::frontend`;
//! - `quantum::optimization`;
//! - `quantum::routing`;
//! - `quantum::hardware`;
//! - `quantum::simulator`;
//! - `quantum::qec`;
//! - backend implementations;
//! - vendor SDKs;
//! - credentials;
//! - network clients;
//! - filesystem execution.
//!
//! # Canonical ownership
//!
//! Each semantic concept has exactly one authoritative implementation.
//!
//! ```text
//! core/
//!     foundational IR primitives
//!
//! quantum/
//!     qubit/gate/measurement/instruction/channel semantics
//!
//! classical/
//!     classical semantic values and computation
//!
//! control/
//!     dynamic control and feedback
//!
//! program/
//!     universal structured program representation
//!
//! model/
//!     circuit/analog/Hamiltonian/annealing/QUBO/etc.
//!
//! pulse/
//!     pulse/frame/port/waveform/calibration semantics
//!
//! resources/
//!     abstract resource/capability/topology requirements
//!
//! scheduling/
//!     semantic schedule representation
//!
//! metadata/
//!     provenance/source/debug metadata
//!
//! analysis/
//!     read-only derived analysis
//!
//! validation/
//!     structural and semantic validation
//!
//! hashing/
//!     canonical content hashing
//!
//! dialect/
//!     extensible operation/model dialects
//!
//! compatibility/
//!     historical API/source compatibility
//! ```
//!
//! The parent module owns only module composition and carefully selected
//! compatibility exports. It must not implement domain logic.
//!
//! # Canonical qubit identity
//!
//! The authoritative logical/physical qubit identity implementation is:
//!
//!     quantum::ir::qubit
//!
//! New code MUST use:
//!
//!     quantum::ir::qubit::QubitId
//!     quantum::ir::qubit::PhysicalQubitId
//!
//! No second `QubitId` may be introduced by `core`, `quantum`, `model`,
//! `resources`, compatibility, or any downstream subsystem.
//!
//! The nested `quantum` facade already re-exports the canonical qubit module;
//! it does not define another qubit type.
//!
//! # Compatibility
//!
//! Historical users may still use root-level APIs such as:
//!
//!     quantum::ir::QubitId
//!     quantum::ir::Gate
//!     quantum::ir::QuantumCircuit
//!
//! Those names are deliberately retained as explicit re-exports of canonical
//! implementations.
//!
//! They are aliases/re-exports, never duplicate definitions.
//!
//! # Rust contract
//!
//! Supported:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! The no-unsafe requirement is compiler-enforced.
//!
//! # API stability
//!
//! Module paths are part of the public API.
//!
//! Adding a child module must not require changing unrelated child modules.
//! The parent should normally only need a new `pub mod` declaration.
//!
//! Root-level re-exports are deliberately selective. Glob exports are avoided
//! because they make API ownership ambiguous and allow unrelated additions to
//! create accidental name collisions.
//!
//! # Serialization
//!
//! Canonical persistence is owned by:
//!
//!     quantum::ir::serialization
//!
//! This module does not implement another serialization format.
//!
//! # Hashing
//!
//! Canonical content hashing is exposed through:
//!
//!     quantum::ir::hashing
//!
//! The historical `quantum::ir::hash` path is retained below as an explicit
//! compatibility alias to the canonical hashing subsystem.
//!
//! # Validation
//!
//! Whole-IR validation is owned by:
//!
//!     quantum::ir::validation
//!
//! Root-level validation exports are deliberately not duplicated here.
//!
//! # Models
//!
//! `QuantumCircuit` is a specialized model, not the definition of all quantum
//! programs.
//!
//! The universal structured program is represented by:
//!
//!     quantum::ir::program
//!
//! while computational paradigms are represented under:
//!
//!     quantum::ir::model
//!
//! This permits the IR to represent circuit, dynamic, analog, Hamiltonian,
//! annealing, QUBO, fermionic, bosonic, continuous-variable,
//! measurement-based, tensor-network, logical and distributed computation
//! without forcing all of them into `Vec<Gate>`.
//!
//! # Pulse semantics
//!
//! Pulse-level semantics are exposed under:
//!
//!     quantum::ir::pulse
//!
//! They remain hardware-independent. Ports, frames, waveforms, capture,
//! calibration and pulse operations describe semantic intent; target lowering
//! determines physical control implementation.
//!
//! This separation is consistent with the role of OpenQASM as an IR between
//! higher-level programs and quantum hardware and with its explicit treatment
//! of classical control, timing and pulse/calibration descriptions.
//!
//! # Resource policy
//!
//! `QuantumIrLimits` describes an explicit compilation/security policy.
//!
//! It MUST NOT be interpreted as a universal Zamani machine limit.
//!
//! Conceptually:
//!
//!     QuantumIrLimits = what this invocation permits
//!     target capability = what this target provides
//!     IR semantics = what the program means
//!
//! These three concepts must remain separate.
//!
//! # No speculative modules
//!
//! Only modules that physically exist in the current repository are declared
//! here. Future modules must be added only after their implementation and
//! contract exist.
//!
//! This prevents the parent module from becoming a permanently broken list of
//! architectural intentions.
//!
//! # No root tests.rs dependency
//!
//! Cross-module tests belong in dedicated test modules or integration tests.
//! This file does not reference a nonexistent `tests.rs`.
//!
//! =============================================================================
//! Compiler-enforced safety boundary
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

// =============================================================================
// Canonical subsystem modules
// =============================================================================
//
// IMPORTANT:
//
// These declarations correspond to the current production directory layout:
//
//     src/quantum/ir/
//         analysis/
//         classical/
//         compatibility/
//         control/
//         core/
//         dialect/
//         hashing/
//         metadata/
//         model/
//         program/
//         pulse/
//         quantum/
//         resources/
//         scheduling/
//         ...
//
// The parent module deliberately does not recreate the former flat-module
// architecture.

/// Read-only semantic and structural analysis.
///
/// Analysis must never mutate canonical IR or introduce target-specific
/// decisions.
pub mod analysis;

/// Classical semantic values, expressions, predicates, arrays and calls.
pub mod classical;

/// Historical source/API compatibility boundary.
///
/// Canonical IR does not depend on this module.
pub mod compatibility;

/// Dynamic quantum/classical control-flow semantics.
pub mod control;

/// Dependency-lowest canonical IR primitives.
pub mod core;

/// Extensible semantic dialect boundary.
pub mod dialect;

/// Canonical deterministic content-hashing boundary.
pub mod hashing;

/// Provenance, source-location and debugging metadata.
pub mod metadata;

/// Universal quantum computational-model taxonomy and implementations.
pub mod model;

/// Structured universal quantum-program representation.
pub mod program;

/// Hardware-independent pulse, frame, port, waveform and calibration
/// semantics.
pub mod pulse;

/// Canonical quantum-domain semantics.
///
/// This facade owns no duplicate quantum types; it exposes the authoritative
/// implementations such as `qubit`, `gate`, `measurement`, and channel
/// semantics.
pub mod quantum;

/// Abstract resource, capability, topology and requirement semantics.
pub mod resources;

/// Semantic scheduling representation.
///
/// Scheduling algorithms remain outside the IR.
pub mod scheduling;

// =============================================================================
// Canonical module-path compatibility aliases
// =============================================================================
//
// The aliases below are intentionally explicit.
//
// They preserve historical paths such as:
//
//     quantum::ir::circuit
//     quantum::ir::gate
//     quantum::ir::measurement
//     quantum::ir::qubit
//
// without creating duplicate implementations.
//
// The canonical implementations remain owned by their current subsystem.

// -----------------------------------------------------------------------------
// Quantum semantic aliases
// -----------------------------------------------------------------------------

/// Compatibility alias for the canonical quantum-domain namespace.
///
/// The implementation remains owned by `quantum::ir::quantum`.
///
/// New code may use either the canonical nested path or this compatibility
/// path where an older API requires it.
pub use quantum::channel;

pub use quantum::frame;

/// Canonical gate module compatibility path.
pub use quantum::gate;

/// Canonical measurement module compatibility path.
pub use quantum::measurement;

/// Canonical qubit module.
///
/// IMPORTANT: this is the sole authoritative `QubitId` implementation.
pub use quantum::qubit;

/// Canonical waveform module compatibility path.
pub use quantum::waveform;

// -----------------------------------------------------------------------------
// Circuit compatibility alias
// -----------------------------------------------------------------------------

/// Compatibility path for the canonical gate-oriented circuit model.
///
/// The implementation is owned by `model::circuit`.
pub use model::circuit as circuit;

// -----------------------------------------------------------------------------
// Hashing compatibility alias
// -----------------------------------------------------------------------------

/// Historical hashing module path.
///
/// The implementation remains owned by `hashing`.
///
/// New code should prefer `quantum::ir::hashing`.
pub use hashing as hash;

// =============================================================================
// Stable root-level type re-exports
// =============================================================================
//
// These exports preserve the existing high-value API used throughout the
// optimizer, frontend, algorithms and other downstream quantum components.
//
// Every symbol below is re-exported from its canonical owner.
// Nothing here defines a second type.

// -----------------------------------------------------------------------------
// Qubit identities
// -----------------------------------------------------------------------------

pub use quantum::qubit::{
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
// Gate semantics
// -----------------------------------------------------------------------------

pub use quantum::gate::{
    Gate,
    GateError,
    GateKind,
    GateParameter,
    GateResult,
    OperandCount,
};

// -----------------------------------------------------------------------------
// Measurement semantics
// -----------------------------------------------------------------------------

pub use quantum::measurement::{
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
// Core identity/version types
// -----------------------------------------------------------------------------

pub use core::identity::{
    BlockId,
    CircuitId,
    IrVersion,
    ModuleId,
    NamespaceId,
    OperationId,
    ParameterId,
    ProgramId,
    RegionId,
    ResourceId,
    SymbolId,
    ValueId,
};

// -----------------------------------------------------------------------------
// Core parameter API
// -----------------------------------------------------------------------------

pub use core::parameter::Parameter;

// -----------------------------------------------------------------------------
// Core limits
// -----------------------------------------------------------------------------

pub use core::limits::{
    LimitsError,
    QuantumIrLimits,
};

// -----------------------------------------------------------------------------
// Canonical IR errors
// -----------------------------------------------------------------------------
//
// The error module is owned by core. Root-level aliases are kept narrow and
// intentionally avoid importing every internal diagnostic type.

pub use core::errors::{
    IrError,
    IrResult,
};

// -----------------------------------------------------------------------------
// Canonical circuit model
// -----------------------------------------------------------------------------

pub use model::circuit::{
    CircuitError,
    CircuitMetadata,
    QuantumCircuit,
};

// -----------------------------------------------------------------------------
// Universal program model
// -----------------------------------------------------------------------------

pub use program::QuantumProgram;

// -----------------------------------------------------------------------------
// Model classification
// -----------------------------------------------------------------------------

pub use model::ModelKind;

// -----------------------------------------------------------------------------
// Canonical analysis API
// -----------------------------------------------------------------------------
//
// These are the stable circuit-analysis APIs already exposed by the analysis
// subsystem. Specialized analysis remains under `quantum::ir::analysis`.

pub use analysis::{
    analyze,
    analyze_with_limits,
    basic_statistics,
    basic_statistics_with_limits,
    BasicCircuitStatistics,
    CircuitStatistics,
};

// =============================================================================
// Stable common prelude
// =============================================================================
//
// The prelude is intentionally small.
//
// It is not a replacement for canonical module paths and does not attempt to
// flatten the entire IR.
//
// Downstream code requiring specialized APIs should import them from their
// owning module.

/// Stable common-import surface for the most frequently consumed Quantum IR
/// semantic types.
///
/// Specialized functionality remains available through the canonical module
/// hierarchy.
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

// =============================================================================
// Architectural compile-time checks
// =============================================================================
//
// These tests do not allocate a quantum state, create hardware, assume a
// particular topology, or establish a machine-size limit.
//
// Their purpose is solely to ensure that the public root API continues to
// refer to the canonical implementations.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_qubit_identity_is_preserved() {
        let logical = QubitId::new(7);

        let canonical: crate::quantum::ir::qubit::QubitId = logical;

        assert_eq!(canonical, logical);
        assert_eq!(canonical.index(), 7);
    }

    #[test]
    fn canonical_physical_qubit_identity_is_preserved() {
        let physical = PhysicalQubitId::new(11);

        let canonical: crate::quantum::ir::qubit::PhysicalQubitId = physical;

        assert_eq!(canonical, physical);
        assert_eq!(canonical.index(), 11);
    }

    #[test]
    fn logical_and_physical_qubit_types_remain_distinct() {
        let logical = QubitId::new(1);
        let physical = PhysicalQubitId::new(1);

        let logical_ref = QubitRef::Logical(logical);
        let physical_ref = QubitRef::Physical(physical);

        assert!(logical_ref.is_logical());
        assert!(!logical_ref.is_physical());

        assert!(physical_ref.is_physical());
        assert!(!physical_ref.is_logical());
    }

    #[test]
    fn canonical_gate_identity_is_preserved() {
        fn accepts_canonical_gate(_: &crate::quantum::ir::quantum::gate::Gate) {}

        let _ = accepts_canonical_gate;
    }

    #[test]
    fn canonical_measurement_identity_is_preserved() {
        fn accepts_canonical_measurement(
            _: &crate::quantum::ir::quantum::measurement::Measurement,
        ) {
        }

        let _ = accepts_canonical_measurement;
    }

    #[test]
    fn canonical_circuit_identity_is_preserved() {
        fn accepts_canonical_circuit(
            _: &crate::quantum::ir::model::circuit::QuantumCircuit,
        ) {
        }

        let _ = accepts_canonical_circuit;
    }

    #[test]
    fn compatibility_circuit_path_is_the_same_type() {
        fn accepts_canonical_circuit(
            _: &crate::quantum::ir::model::circuit::QuantumCircuit,
        ) {
        }

        let _: fn(&crate::quantum::ir::circuit::QuantumCircuit) =
            accepts_canonical_circuit;
    }

    #[test]
    fn compatibility_hash_path_is_the_same_namespace() {
        let _: fn(&[u8]) -> crate::quantum::ir::hashing::IrHash =
            crate::quantum::ir::hashing::hash_canonical_bytes;

        let _: fn(&[u8]) -> crate::quantum::ir::hashing::IrHash =
            crate::quantum::ir::hash::hash_canonical_bytes;
    }

    #[test]
    fn root_parameter_is_the_canonical_parameter_type() {
        fn accepts_canonical_parameter(
            _: &crate::quantum::ir::core::parameter::Parameter,
        ) {
        }

        let _ = accepts_canonical_parameter;
    }

    #[test]
    fn root_operation_identity_is_the_canonical_identity_type() {
        fn accepts_canonical_operation_id(
            _: crate::quantum::ir::core::identity::OperationId,
        ) {
        }

        let _ = accepts_canonical_operation_id;
    }

    #[test]
    fn no_machine_size_is_encoded_by_the_root_module() {
        // This test is deliberately structural rather than numerical.
        //
        // Qubit cardinality belongs to program/model/resource data and is
        // never encoded by this module as a machine-wide constant.
        let _small = QubitId::new(0);
        let _large = QubitId::new(usize::MAX);

        assert_ne!(_small, _large);
    }
}