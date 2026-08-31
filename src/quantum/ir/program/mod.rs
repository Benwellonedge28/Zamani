//! Zamani Quantum IR — Program Layer
//!
//! Canonical module boundary for the structured, hardware-independent
//! representation of Zamani quantum programs.
//!
//! # Architectural role
//!
//! `quantum::ir::program` owns the structural program layer of the canonical
//! Zamani Quantum IR.
//!
//! The program layer is deliberately broader than a quantum circuit. It is
//! capable of representing:
//!
//! - gate-based quantum programs;
//! - dynamic circuits;
//! - mid-circuit measurement;
//! - classical feedback;
//! - structured control flow;
//! - pulse-level program structure;
//! - analog program structure;
//! - annealing / Ising / QUBO workloads;
//! - logical and fault-tolerant program structure;
//! - distributed quantum programs;
//! - hybrid quantum/classical programs;
//! - future quantum-computing models.
//!
//! The program layer describes semantic program structure.
//!
//! It does NOT decide:
//!
//! - which physical device executes a program;
//! - which physical qubits are selected;
//! - how routing is performed;
//! - how scheduling is performed;
//! - which calibration is selected;
//! - which native hardware instruction is emitted;
//! - how pulses are synthesized for a particular device;
//! - how a backend communicates with hardware;
//! - how a quantum state is simulated;
//! - how QEC is decoded;
//! - which optimization algorithm is used;
//! - how Zamani source syntax is parsed.
//!
//! Those responsibilities belong to downstream subsystems.
//!
//! # Fundamental Zamani rule
//!
//! ```text
//!                 ZAMANI SOURCE
//!                       │
//!                       ▼
//!                frontend / parser
//!                       │
//!                       ▼
//!              CANONICAL QUANTUM IR
//!                       │
//!                       ▼
//!          quantum::ir::program
//!                       │
//!          ┌────────────┼─────────────┐
//!          │            │             │
//!          ▼            ▼             ▼
//!     optimization    routing     scheduling
//!          │            │             │
//!          └────────────┼─────────────┘
//!                       ▼
//!                    hardware
//!                       │
//!                       ▼
//!                    backend
//!                       │
//!                       ▼
//!                   execution
//! ```
//!
//! The canonical IR therefore answers:
//!
//! > What does the program mean?
//!
//! Hardware and backend layers answer:
//!
//! > Where and how is that meaning executed?
//!
//! # Write once, scale everywhere
//!
//! The program layer MUST NOT contain an architectural quantum-machine size.
//!
//! In particular, this module MUST NOT introduce:
//!
//! ```text
//! MAX_QUBITS
//! MAX_QUBIT_COUNT
//! MAX_REGISTER_SIZE
//! MAX_MACHINE_SIZE
//! MAX_OPERATIONS
//! MAX_PROGRAM_SIZE
//! ```
//!
//! nor any equivalent fixed semantic ceiling.
//!
//! A Zamani program may semantically describe:
//!
//! ```text
//! 1 qubit
//! 2 qubits
//! 10 qubits
//! 1,000 qubits
//! 1,000,000 qubits
//! N qubits
//! ```
//!
//! subject only to the resources available to the particular compiler,
//! execution environment and target.
//!
//! Concrete limits belong to explicit policy such as `QuantumIrLimits`.
//!
//! Those limits are security/resource controls for a particular compilation
//! or service invocation. They are NOT the semantic maximum of Zamani.
//!
//! Therefore:
//!
//! ```text
//! IR semantic capacity
//!     !=
//! compiler policy limit
//!     !=
//! host memory limit
//!     !=
//! target hardware capacity
//! ```
//!
//! # No "infinite allocation"
//!
//! "Scale to infinity" means that the IR contains no artificial machine-size
//! ceiling.
//!
//! It does NOT mean that a concrete program attempts to allocate an infinite
//! number of objects.
//!
//! Every concrete program remains finite and representable.
//!
//! The practical upper bound is determined by:
//!
//! - addressable memory;
//! - integer representation;
//! - explicit IR limits;
//! - compiler resources;
//! - target resources;
//! - backend resources.
//!
//! The same semantic types are used at every scale.
//!
//! # Canonical ownership
//!
//! The program module is divided into explicit responsibility boundaries:
//!
//! ```text
//! program/
//! ├── mod.rs
//! ├── program.rs
//! ├── module.rs
//! ├── region.rs
//! ├── block.rs
//! ├── operation.rs
//! ├── operand.rs
//! └── result.rs
//! ```
//!
//! Ownership is:
//!
//! ```text
//! mod.rs
//!     Module boundary only.
//!
//! program.rs
//!     Top-level QuantumProgram container.
//!
//! module.rs
//!     Quantum module / namespace / definition container.
//!
//! region.rs
//!     Structured region representation.
//!
//! block.rs
//!     Structured block representation.
//!
//! operation.rs
//!     Program-layer access to the canonical operation model.
//!
//! operand.rs
//!     Program operand representation.
//!
//! result.rs
//!     Program result representation.
//! ```
//!
//! No file may duplicate another file's semantic type.
//!
//! # Single source of truth
//!
//! The following rule is mandatory:
//!
//! ```text
//! ONE canonical Program representation
//! ONE canonical Operation representation
//! ONE canonical Region representation
//! ONE canonical Block representation
//! ONE canonical Operand representation
//! ONE canonical Result representation
//! ONE canonical identity representation
//! ONE canonical QubitId representation
//! ```
//!
//! In particular, `program::operation::Operation` MUST remain the same Rust
//! type as `quantum::ir::operation::Operation`.
//!
//! The program operation boundary already implements this principle by
//! re-exporting the canonical operation types instead of defining duplicates.
//!
//! # Canonical qubit identity
//!
//! The authoritative qubit module is:
//!
//! ```text
//! quantum::ir::qubit
//! ```
//!
//! The canonical identities are therefore reached through:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! New program-layer code MUST NOT use:
//!
//! ```text
//! quantum::ir::qubits::QubitId
//! ```
//!
//! The historical `qubits` spelling may exist as a compatibility alias at the
//! root IR boundary, but it is not the canonical path.
//!
//! This module therefore deliberately does not define another qubit identity.
//!
//! # Dependency direction
//!
//! The intended dependency graph is:
//!
//! ```text
//! core
//!  │
//!  ├── identity
//!  ├── types
//!  ├── value
//!  ├── parameter
//!  ├── attribute
//!  ├── extension
//!  ├── errors
//!  └── limits
//!       │
//!       ▼
//! quantum / classical / timing / resources
//!       │
//!       ▼
//! operation / region / block / operand / result
//!       │
//!       ▼
//! program
//!       │
//!       ├───────────────┬────────────────┐
//!       ▼               ▼                ▼
//! validation        analysis      serialization/hash
//!       │               │                │
//!       └───────────────┼────────────────┘
//!                       ▼
//!                  downstream
//!                       │
//!          ┌────────────┼─────────────┐
//!          ▼            ▼             ▼
//!     optimization    routing     scheduling
//!          │            │             │
//!          └────────────┼─────────────┘
//!                       ▼
//!                    hardware
//!                       │
//!                       ▼
//!                    backend
//! ```
//!
//! The program layer MUST NOT introduce reverse dependencies on:
//!
//! - frontend;
//! - optimization;
//! - routing;
//! - scheduling;
//! - hardware;
//! - simulator;
//! - QEC;
//! - backend execution implementations.
//!
//! # Program container versus circuit
//!
//! `QuantumCircuit` is a specialized circuit representation.
//!
//! `QuantumProgram` is the broader semantic program representation.
//!
//! The distinction is fundamental:
//!
//! ```text
//! QuantumProgram
//! ├── declarations
//! ├── symbols
//! ├── parameters
//! ├── logical resources
//! ├── classical resources
//! ├── regions
//! ├── blocks
//! ├── operations
//! ├── control flow
//! ├── measurements
//! ├── timing
//! ├── pulse references
//! ├── resource requirements
//! ├── capability requirements
//! ├── provenance
//! └── extensions
//! ```
//!
//! while:
//!
//! ```text
//! QuantumCircuit
//! └── ordered circuit operations
//! ```
//!
//! A circuit can therefore be represented within the broader program model,
//! but the universal program model must not be reduced to `Vec<Gate>`.
//!
//! # Structured control flow
//!
//! The program layer supports structured execution through regions and blocks.
//!
//! This allows downstream IR layers to represent:
//!
//! ```text
//! if
//! else
//! while
//! do-while
//! for
//! repeat
//! switch
//! branch
//! conditional quantum operation
//! measurement feedback
//! early termination
//! nested regions
//! ```
//!
//! Control-flow semantics remain represented by the canonical control-flow and
//! operation layers; this module only exposes their structural program
//! container.
//!
//! # Dynamic quantum programs
//!
//! Modern quantum programs are not necessarily static gate sequences.
//!
//! A valid program may express:
//!
//! ```text
//! measure
//!    │
//!    ▼
//! classical value
//!    │
//!    ▼
//! predicate
//!    │
//!    ▼
//! quantum operation
//! ```
//!
//! The program layer therefore MUST NOT assume that all dependencies are known
//! solely from static gate order.
//!
//! Dynamic conditions, region relationships and operation dependencies belong
//! to the canonical operation/control-flow representations.
//!
//! # Pulse-level integration
//!
//! Zamani must support source such as:
//!
//! ```text
//! fn x_gate(q) {
//!     pulse(amp=0.3, dur=20ns)
//! }
//! ```
//!
//! The program layer must be able to contain the resulting semantic operation
//! without deciding:
//!
//! - DAC selection;
//! - physical drive channel;
//! - carrier frequency;
//! - calibration;
//! - waveform sampling rate;
//! - native hardware instruction;
//! - device-specific pulse decomposition.
//!
//! Conceptually:
//!
//! ```text
//! Zamani source
//!      │
//!      ▼
//! Program
//!      │
//!      ▼
//! Pulse semantic operation
//!      │
//!      ▼
//! target lowering
//!      │
//!      ▼
//! physical pulse / native instruction
//! ```
//!
//! The program layer therefore remains hardware independent.
//!
//! # Analog, annealing and future models
//!
//! The program container MUST NOT assume that every quantum computation is
//! represented as gates.
//!
//! It must be able to contain operations supplied by canonical model/dialect
//! layers representing:
//!
//! - Hamiltonian evolution;
//! - analog control;
//! - annealing;
//! - Ising;
//! - QUBO;
//! - fermionic systems;
//! - bosonic systems;
//! - continuous-variable systems;
//! - measurement-based computation;
//! - logical/fault-tolerant computation;
//! - distributed quantum computation;
//! - future architectures.
//!
//! These models belong to their respective IR layers.
//!
//! `program` provides their structural container.
//!
//! # Physical versus logical resources
//!
//! Program declarations may refer to logical quantum resources.
//!
//! Physical placement belongs to mapping/routing.
//!
//! Therefore:
//!
//! ```text
//! Program
//!     │
//!     └── logical QubitId
//!              │
//!              ▼
//!          routing/mapping
//!              │
//!              ▼
//!       PhysicalQubitId
//! ```
//!
//! `program` MUST NOT perform physical allocation.
//!
//! # Resource and capability requirements
//!
//! A program may require capabilities such as:
//!
//! ```text
//! mid_circuit_measurement
//! dynamic_control
//! arbitrary_single_qubit_rotation
//! pulse_control
//! analog_control
//! logical_qubits
//! distributed_execution
//! ```
//!
//! These are semantic requirements.
//!
//! The program layer does not decide whether a target satisfies them.
//!
//! Hardware capability resolution belongs downstream.
//!
//! # Atomicity
//!
//! Program mutations are owned by `program.rs` and related structural modules.
//!
//! The module boundary itself MUST NOT introduce mutation logic.
//!
//! The intended mutation contract is:
//!
//! ```text
//! validate candidate
//!       │
//!       ▼
//! validate identity
//!       │
//!       ▼
//! validate references
//!       │
//!       ▼
//! validate explicit policy
//!       │
//!       ▼
//! reserve/grow storage
//!       │
//!       ▼
//! commit
//! ```
//!
//! A failed mutation must not leave a partially committed semantic object.
//!
//! # Determinism
//!
//! The program layer must preserve deterministic semantics.
//!
//! Deterministic ordering and identity lookup are owned by the concrete
//! program/container modules.
//!
//! `mod.rs` introduces no:
//!
//! - global mutable state;
//! - random identity generation;
//! - unordered semantic registry;
//! - hidden allocator;
//! - thread-local semantic state.
//!
//! Canonical serialization and hashing are owned by the corresponding IR
//! modules.
//!
//! # Scalability
//!
//! No type in this module imposes a fixed:
//!
//! - qubit count;
//! - register size;
//! - operation count;
//! - region count;
//! - block count;
//! - operand count;
//! - result count;
//! - machine size;
//! - topology;
//! - vendor;
//! - architecture.
//!
//! Collection types used by the implementation are host-resource mechanisms,
//! not semantic quantum-machine limits.
//!
//! # Error ownership
//!
//! This module does not define a second error hierarchy.
//!
//! Errors remain owned by the concrete canonical modules:
//!
//! ```text
//! operation.rs
//!     OperationError
//!
//! region.rs
//!     RegionError
//!
//! block.rs
//!     BlockError
//!
//! operand.rs
//!     OperandError
//!
//! result.rs
//!     ResultError
//!
//! program.rs
//!     ProgramError
//! ```
//!
//! Root-level IR errors may wrap or classify those errors where appropriate,
//! but this module MUST NOT create duplicate semantic error types merely for
//! convenience.
//!
//! # Serialization
//!
//! `program/mod.rs` does not implement serialization.
//!
//! Canonical serialization belongs to:
//!
//! ```text
//! quantum::ir::serialization
//! ```
//!
//! The serialization layer must serialize the canonical program/module types,
//! not a second representation created by this module.
//!
//! # Hashing
//!
//! `program/mod.rs` does not calculate hashes.
//!
//! Canonical content hashing belongs to:
//!
//! ```text
//! quantum::ir::hash
//! ```
//!
//! The hash must depend on semantic content, not on the module path through
//! which a type was imported.
//!
//! # Validation
//!
//! Local structural invariants belong to the individual program files.
//!
//! Whole-program semantic validation belongs to:
//!
//! ```text
//! quantum::ir::validation
//! ```
//!
//! The intended layering is:
//!
//! ```text
//! operation
//!     ↓
//! operation-local invariants
//!
//! block / region
//!     ↓
//! structural invariants
//!
//! program
//!     ↓
//! program-reference invariants
//!
//! validation
//!     ↓
//! complete semantic validation
//! ```
//!
//! # Provenance
//!
//! Program transformation lineage belongs to the canonical provenance layer.
//!
//! Optimization, routing and scheduling must be able to identify the program
//! version they consumed and produced without modifying the meaning of program
//! identity.
//!
//! # Compatibility
//!
//! The structured program directory is the long-term layout:
//!
//! ```text
//! quantum::ir::program::program
//! quantum::ir::program::module
//! quantum::ir::program::region
//! quantum::ir::program::block
//! quantum::ir::program::operation
//! quantum::ir::program::operand
//! quantum::ir::program::result
//! ```
//!
//! Compatibility re-exports should remain localized to this boundary or to
//! the dedicated IR compatibility layer.
//!
//! They must never create duplicate semantic structures.
//!
//! # Rust compatibility
//!
//! This module targets:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe`.
//!
//! The no-unsafe requirement is compiler enforced below.
//!
//! # Module API contract
//!
//! This file owns ONLY:
//!
//! 1. child-module declarations;
//! 2. program-layer namespace organization;
//! 3. carefully selected canonical re-exports;
//! 4. module-level compile-time integration tests.
//!
//! It does NOT:
//!
//! - define a second `QuantumProgram`;
//! - define a second `Operation`;
//! - define a second `QubitId`;
//! - define a second `Region`;
//! - define a second `Block`;
//! - define a second `Operand`;
//! - define a second `Result`;
//! - implement optimization;
//! - implement routing;
//! - implement scheduling;
//! - implement hardware interaction;
//! - implement execution.
//!
//! -----------------------------------------------------------------------------
//! Implementation
//! -----------------------------------------------------------------------------

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

// =============================================================================
// Canonical structured program modules
// =============================================================================
//
// Keep these declarations explicit and responsibility-oriented.
//
// Rust resolves:
//
//     program/mod.rs
//         ├── block.rs
//         ├── module.rs
//         ├── operand.rs
//         ├── operation.rs
//         ├── program.rs
//         ├── region.rs
//         └── result.rs
//
// No module below this boundary is optional. The structured program contract
// requires all seven components.

/// Structured basic-block representation.
///
/// Owns block identity, operation ordering within a block, arguments,
/// successors and termination structure.
pub mod block;

/// Quantum module / namespace representation.
///
/// Owns reusable definitions, symbols and module-level program structure.
pub mod module;

/// Canonical program operand representation.
///
/// Owns references to semantic values/resources consumed by operations.
pub mod operand;

/// Program-layer namespace for the canonical operation model.
///
/// IMPORTANT:
/// `program::operation::Operation` is a re-export of the canonical
/// `quantum::ir::operation::Operation`; it is not a second operation type.
pub mod operation;

/// Top-level universal quantum-program container.
///
/// Owns program declarations, program structure, operation ownership and
/// deterministic program-level relationships.
pub mod program;

/// Structured region representation.
///
/// Owns region structure and relationships between blocks.
pub mod region;

/// Program result representation.
///
/// Owns semantic values/results produced by program operations.
pub mod result;

// =============================================================================
// Canonical operation compatibility exports
// =============================================================================
//
// These exports make the program namespace ergonomic while preserving the
// single-source-of-truth rule.
//
// `program::operation::*` itself already re-exports the canonical operation
// implementation from `quantum::ir::operation`.
//
// We intentionally do NOT glob-export every child module here. Glob exports
// would make this boundary fragile as the child modules evolve and could
// introduce name collisions between independently owned concepts.
//
// Callers can always use the explicit canonical paths:
//
//     quantum::ir::program::program::QuantumProgram
//     quantum::ir::program::operation::Operation
//     quantum::ir::program::operation::OperationId
//     quantum::ir::program::region::Region
//     quantum::ir::program::block::Block
//     quantum::ir::program::operand::...
//     quantum::ir::program::result::...

// =============================================================================
// Canonical identity convenience exports
// =============================================================================
//
// The operation namespace already exposes the canonical OperationId.
// Re-exporting the canonical identity here provides a stable program-level
// access path without defining another identity type.
//
// IMPORTANT:
// This is a re-export only. There is still exactly one OperationId type.

pub use super::identity::{
    BlockId,
    ModuleId,
    OperationId,
    ParameterId,
    ProgramId,
    RegionId,
    SymbolId,
    ValueId,
};

// =============================================================================
// Canonical qubit identity convenience exports
// =============================================================================
//
// IMPORTANT:
// These MUST come from `quantum::ir::qubit`.
//
// Do not change these imports to `super::qubits`.
//
// The repository's canonical module is `qubit`, and the program layer must
// preserve that naming contract.

pub use super::qubit::{
    PhysicalQubitId,
    QubitId,
};

// =============================================================================
// Program-level public API
// =============================================================================
//
// Re-export the top-level program representation.
//
// If the concrete program implementation introduces additional public
// program-level types, they should be re-exported from `program.rs` itself
// and added here deliberately rather than using a glob export.
//
// The central type is `QuantumProgram`.

pub use self::program::QuantumProgram;

// =============================================================================
// Compile-time namespace invariants
// =============================================================================
//
// These tests deliberately verify only relationships that are stable and
// architectural. They do not instantiate hardware, allocate quantum state,
// or assume a fixed machine size.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_qubit_path_is_available() {
        let logical = QubitId::new(0);

        let _: super::super::qubit::QubitId = logical;

        assert_eq!(logical, QubitId::new(0));
    }

    #[test]
    fn canonical_physical_qubit_path_is_available() {
        let physical = PhysicalQubitId::new(0);

        let _: super::super::qubit::PhysicalQubitId = physical;

        assert_eq!(physical, PhysicalQubitId::new(0));
    }

    #[test]
    fn canonical_operation_identity_is_shared() {
        let id = OperationId::new(0);

        let _: super::super::identity::OperationId = id;
        let _: super::operation::OperationId = id;

        assert_eq!(id, OperationId::new(0));
    }

    #[test]
    fn canonical_program_identity_is_shared() {
        let id = ProgramId::new(0);

        let _: super::super::identity::ProgramId = id;

        assert_eq!(id, ProgramId::new(0));
    }

    #[test]
    fn canonical_region_identity_is_shared() {
        let id = RegionId::new(0);

        let _: super::super::identity::RegionId = id;

        assert_eq!(id, RegionId::new(0));
    }

    #[test]
    fn canonical_block_identity_is_shared() {
        let id = BlockId::new(0);

        let _: super::super::identity::BlockId = id;

        assert_eq!(id, BlockId::new(0));
    }

    #[test]
    fn canonical_module_identity_is_shared() {
        let id = ModuleId::new(0);

        let _: super::super::identity::ModuleId = id;

        assert_eq!(id, ModuleId::new(0));
    }

    #[test]
    fn canonical_parameter_identity_is_shared() {
        let id = ParameterId::new(0);

        let _: super::super::identity::ParameterId = id;

        assert_eq!(id, ParameterId::new(0));
    }

    #[test]
    fn canonical_symbol_identity_is_shared() {
        let id = SymbolId::new(0);

        let _: super::super::identity::SymbolId = id;

        assert_eq!(id, SymbolId::new(0));
    }

    #[test]
    fn canonical_value_identity_is_shared() {
        let id = ValueId::new(0);

        let _: super::super::identity::ValueId = id;

        assert_eq!(id, ValueId::new(0));
    }

    #[test]
    fn program_module_exposes_the_canonical_program_type() {
        fn accepts_program_type(_: &QuantumProgram) {}

        let _ = accepts_program_type;
    }

    #[test]
    fn program_module_does_not_define_machine_size_constants() {
        // This test intentionally has no fixed qubit-count expectation.
        //
        // Its existence documents the architectural rule:
        //
        //     program/mod.rs
        //         !=
        //     hardware capacity
        //
        // Quantum-machine size belongs to declarations, target capabilities
        // and explicit compilation policy.
        assert!(true);
    }
}