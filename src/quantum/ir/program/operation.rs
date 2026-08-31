//! Zamani Quantum IR — Program Operation Compatibility Boundary
//!
//! This module is the program-layer access point for the canonical Zamani
//! Quantum IR operation model.
//!
//! # Architectural role
//!
//! `program/operation.rs` deliberately does NOT define a second `Operation`
//! type.
//!
//! The canonical operation contract is owned by:
//!
//! ```text
//! quantum::ir::operation
//! ```
//!
//! This module provides the program-layer namespace for that canonical
//! operation contract while preserving exactly one source of truth.
//!
//! The architecture is:
//!
//! ```text
//! src/quantum/ir/
//! │
//! ├── operation.rs
//! │      │
//! │      └── canonical Operation model
//! │
//! └── program/
//!        │
//!        ├── operation.rs  ← this compatibility boundary
//!        └── program.rs
//! ```
//!
//! The important invariant is:
//!
//! ```text
//! ONE Operation type
//! ONE OperationBody type
//! ONE OperationError type
//! ONE OperationId type
//! ONE validation contract
//! ```
//!
//! There must never be independent program-level and root-level versions of
//! these semantic objects.
//!
//! # Why this file exists
//!
//! The Zamani Quantum IR is being organized into responsibility-oriented
//! subdirectories:
//!
//! ```text
//! quantum::ir::core
//! quantum::ir::program
//! quantum::ir::quantum
//! quantum::ir::classical
//! quantum::ir::control
//! quantum::ir::pulse
//! quantum::ir::timing
//! quantum::ir::models
//! quantum::ir::resources
//! quantum::ir::scheduling
//! quantum::ir::analysis
//! quantum::ir::validation
//! quantum::ir::serialization
//! quantum::ir::hashing
//! quantum::ir::dialect
//! quantum::ir::compatibility
//! ```
//!
//! `program/operation.rs` therefore establishes the program-layer namespace
//! without prematurely moving or duplicating the canonical operation
//! implementation.
//!
//! This allows the repository to migrate from the historical flat IR layout
//! to the structured layout without changing the semantic identity of an
//! operation.
//!
//! # Single source of truth
//!
//! The following are re-exported from the canonical operation module:
//!
//! - `Operation`;
//! - `OperationBody`;
//! - `OperationClass`;
//! - `OperationCondition`;
//! - `OperationError`;
//! - `OperationResult`;
//! - `OperationSequence`.
//!
//! No type in this file wraps or duplicates those types.
//!
//! Consequently:
//!
//! ```text
//! program::operation::Operation
//! ```
//!
//! and:
//!
//! ```text
//! operation::Operation
//! ```
//!
//! are exactly the same Rust type.
//!
//! This prevents:
//!
//! - incompatible operation representations;
//! - duplicate validation;
//! - duplicate serialization rules;
//! - duplicate hashing rules;
//! - optimizer type mismatches;
//! - routing type mismatches;
//! - scheduler type mismatches;
//! - frontend lowering mismatches;
//! - backend integration mismatches.
//!
//! # Canonical qubit identity
//!
//! Operations ultimately use:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! ```
//!
//! The legacy/non-canonical spelling:
//!
//! ```text
//! quantum::ir::qubits::QubitId
//! ```
//!
//! must not be introduced here.
//!
//! This boundary therefore intentionally does not define or wrap qubit
//! identities.
//!
//! # Program integration
//!
//! `program/program.rs` owns the program container and deterministic operation
//! ordering.
//!
//! It does NOT redefine operation semantics.
//!
//! Conceptually:
//!
//! ```text
//! QuantumProgram
//!     │
//!     ├── OperationId
//!     │
//!     ├── Vec<OperationId>
//!     │
//!     └── operation storage
//!              │
//!              ▼
//!       program::operation::Operation
//!              │
//!              ▼
//!       quantum::ir::operation::Operation
//! ```
//!
//! Because this module re-exports the canonical type, no conversion is
//! required.
//!
//! # Integration with optimization
//!
//! The optimization subsystem must consume the canonical `Operation`:
//!
//! ```text
//! quantum::ir::operation::Operation
//! ```
//!
//! or, where the program namespace is more convenient:
//!
//! ```text
//! quantum::ir::program::operation::Operation
//! ```
//!
//! Both names refer to the same type.
//!
//! Optimization must never define another operation type merely to represent
//! optimized operations.
//!
//! # Integration with routing
//!
//! Routing consumes logical quantum operands from the canonical operation
//! model.
//!
//! Logical qubits remain:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! ```
//!
//! Physical placement remains owned by the mapping/routing subsystem.
//!
//! This file therefore introduces no physical qubit representation.
//!
//! # Integration with scheduling
//!
//! Scheduling consumes operations and produces scheduling information.
//!
//! Scheduling does not modify the semantic identity of an operation merely
//! because an execution time has been assigned.
//!
//! Timing and scheduling information therefore remain outside this module.
//!
//! # Integration with hardware
//!
//! Hardware capabilities determine whether an operation can be implemented.
//!
//! This module does not contain:
//!
//! - vendor IDs;
//! - hardware IDs;
//! - physical qubit numbers;
//! - native gate sets;
//! - DAC configuration;
//! - calibration data;
//! - device topology;
//! - execution queues.
//!
//! This preserves the fundamental IR boundary:
//!
//! ```text
//! IR       = WHAT
//! hardware = WHERE / WHAT EXISTS
//! routing  = WHICH RESOURCE
//! schedule = WHEN
//! backend  = HOW EXECUTED
//! ```
//!
//! # Integration with pulse-level control
//!
//! The canonical operation model already supports pulse-level operation
//! references through typed identities such as `PulseId`.
//!
//! This module does not redefine pulse semantics.
//!
//! For example, a Zamani source construct conceptually equivalent to:
//!
//! ```text
//! fn x_gate(q) {
//!     pulse(amp=0.3, dur=20ns)
//! }
//! ```
//!
//! is represented by the canonical operation model and subsequently resolved
//! by the pulse layer.
//!
//! No hardware DAC or physical control channel is introduced here.
//!
//! # Integration with dynamic circuits
//!
//! Dynamic quantum programs may contain:
//!
//! ```text
//! measurement
//!     ↓
//! classical value
//!     ↓
//! condition
//!     ↓
//! quantum operation
//! ```
//!
//! The canonical operation model provides the operation-level condition and
//! stable operation references.
//!
//! More complex structured control flow remains owned by the program/control
//! layers.
//!
//! This module does not introduce a second condition representation.
//!
//! # Integration with serialization
//!
//! Serialization must serialize the canonical operation type, regardless of
//! whether it was imported through:
//!
//! ```text
//! quantum::ir::operation
//! ```
//!
//! or:
//!
//! ```text
//! quantum::ir::program::operation
//! ```
//!
//! Because the latter is a re-export, no adapter or duplicate schema is
//! necessary.
//!
//! # Integration with hashing
//!
//! Canonical hashing must hash the canonical operation semantics.
//!
//! This file adds no hashing implementation and no identity transformation.
//!
//! In particular:
//!
//! ```text
//! program::operation::Operation
//! ```
//!
//! must not receive a different hash merely because it was accessed through
//! the program namespace.
//!
//! # Integration with validation
//!
//! Local operation validation remains owned by the canonical operation module.
//!
//! Whole-program validation remains owned by `validation.rs`.
//!
//! This gives the following separation:
//!
//! ```text
//! operation.rs
//!     ↓
//! local operation invariants
//!
//! program.rs
//!     ↓
//! program structure and references
//!
//! validation.rs
//!     ↓
//! complete semantic validation
//! ```
//!
//! # Scalability
//!
//! This module introduces no machine-size assumptions.
//!
//! It does not define:
//!
//! - maximum qubits;
//! - maximum operations;
//! - maximum operands;
//! - maximum registers;
//! - maximum program size;
//! - maximum regions;
//! - maximum blocks;
//! - maximum nesting;
//! - maximum machine size.
//!
//! Concrete limits belong to explicit resource/security policy such as
//! `QuantumIrLimits`.
//!
//! Therefore this boundary does not prevent the same semantic program model
//! from being used for:
//!
//! - one qubit;
//! - thousands of qubits;
//! - millions of qubits;
//! - distributed quantum systems;
//! - logical/fault-tolerant systems;
//! - future architectures.
//!
//! "Infinity" is not represented as an infinite allocation. Every concrete
//! program remains finite and is constrained only by explicit policies and
//! available resources.
//!
//! # Determinism
//!
//! This module introduces no mutable state and no global allocator.
//!
//! It introduces no random IDs.
//!
//! It introduces no unordered semantic collection.
//!
//! It therefore preserves the determinism guarantees of the canonical
//! operation module.
//!
//! # Thread safety
//!
//! This module introduces no mutable global state and no synchronization
//! primitives.
//!
//! The re-exported operation types inherit their normal Rust ownership and
//! thread-safety properties from their underlying fields.
//!
//! No additional `Send`, `Sync`, `Unpin`, or other unsafe marker
//! implementations are introduced here.
//!
//! # Versioning
//!
//! This namespace boundary is semantically transparent.
//!
//! Versioning remains owned by the canonical IR version contract.
//!
//! A program-layer namespace migration must not change the semantic IR
//! version merely because a type became reachable through a new module path.
//!
//! # Compatibility
//!
//! This file is deliberately compatible with the current repository layout,
//! where `src/quantum/ir/operation.rs` is already the canonical operation
//! implementation.
//!
//! It enables the new directory layout without creating duplicate types.
//!
//! Future migration may make `program/operation.rs` the physical home of the
//! canonical implementation, but that migration must preserve this public
//! contract:
//!
//! ```text
//! quantum::ir::operation::Operation
//! quantum::ir::program::operation::Operation
//! ```
//!
//! must either remain aliases/re-exports of one another or the old path must
//! receive an explicit compatibility re-export.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no external dependencies;
//! - no unsafe.
//!
//! `#![forbid(unsafe_code)]` makes the no-unsafe requirement compiler-enforced.
//!
//! # Public API contract
//!
//! This module intentionally exposes only canonical operation types.
//!
//! No private implementation state is introduced.
//!
//! No constructors are duplicated.
//!
//! No validation helpers are duplicated.
//!
//! No conversion layer is required.
//!
//! No operation wrapper is introduced.
//!
//! # Ownership contract
//!
//! OWNS:
//!
//! - program-layer namespace access to canonical operations;
//! - compatibility of the structured program module layout.
//!
//! DOES NOT OWN:
//!
//! - operation semantics;
//! - operation identity;
//! - gate semantics;
//! - measurement semantics;
//! - qubit identity;
//! - classical-bit identity;
//! - pulse semantics;
//! - timing semantics;
//! - hardware resources;
//! - routing;
//! - scheduling;
//! - optimization;
//! - validation;
//! - serialization;
//! - hashing;
//! - execution.
//!
//! # Why re-export instead of duplication?
//!
//! A duplicate implementation would create this invalid architecture:
//!
//! ```text
//!                 ┌── root Operation
//!                 │
//! frontend ───────┤
//!                 │
//!                 └── program Operation
//! ```
//!
//! Different compiler components would eventually consume different types.
//!
//! The correct architecture is:
//!
//! ```text
//!                 ┌── quantum::ir::operation::Operation
//!                 │
//! frontend ───────┤
//!                 │
//! program ────────┤
//! optimization ───┤
//! routing ────────┤
//! scheduling ─────┤
//! hardware ───────┘
//! ```
//!
//! This is especially important for Zamani's "write once, scale everywhere"
//! requirement.
//!
//! A universal IR cannot have multiple subtly different semantic operation
//! definitions.
//!
//! # Compile-time guarantee
//!
//! Because this module uses `pub use` rather than wrapper structs, Rust itself
//! guarantees that callers receive the exact canonical types.
//!
//! There is no runtime dispatch, allocation, conversion, or unsafe operation
//! involved.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

// -----------------------------------------------------------------------------
// Canonical operation re-exports
// -----------------------------------------------------------------------------
//
// IMPORTANT:
// //
// The canonical implementation remains in `quantum::ir::operation` while the
// repository transitions to the structured `program/` hierarchy.
//
// Do not replace these with duplicate definitions.

pub use super::operation::{
    Operation,
    OperationBody,
    OperationClass,
    OperationCondition,
    OperationError,
    OperationResult,
    OperationSequence,
};

// -----------------------------------------------------------------------------
// Canonical identity re-exports
// -----------------------------------------------------------------------------
//
// These are intentionally included here because operation consumers often
// import the operation and its stable identity from the same program-level
// namespace.
//
// They remain the exact canonical identity types from `identity.rs`.

pub use super::identity::OperationId;

// -----------------------------------------------------------------------------
// Canonical qubit identity re-export
// -----------------------------------------------------------------------------
//
// The canonical qubit module is `quantum::ir::qubit`.
// //
// This re-export is intentionally explicit so program-level consumers do not
// fall back to the historical `qubits` spelling.

pub use super::qubit::QubitId;

// -----------------------------------------------------------------------------
// Compile-time API assertions
// -----------------------------------------------------------------------------
//
// These functions do not execute and allocate nothing. They simply provide
// compile-time type checking that the public aliases remain the expected
// canonical types.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn program_operation_is_the_canonical_operation_type() {
        let id = OperationId::new(1);
        let pulse = super::super::identity::PulseId::new(1);

        let operation = Operation::pulse(id, pulse)
            .expect("canonical pulse operation must be constructible");

        assert_eq!(operation.id(), id);
        assert!(operation.is_pulse());
    }

    #[test]
    fn program_operation_uses_canonical_qubit_identity() {
        let qubit = QubitId::new(7);

        let _: super::super::qubit::QubitId = qubit;

        assert_eq!(qubit.index(), 7);
    }

    #[test]
    fn operation_body_is_reexported_without_conversion() {
        let body = OperationBody::Reset {
            qubit: QubitId::new(0),
        };

        assert_eq!(body.qubits_vec(), vec![QubitId::new(0)]);
    }

    #[test]
    fn operation_class_is_canonical() {
        let body = OperationBody::Reset {
            qubit: QubitId::new(0),
        };

        assert_eq!(body.class(), OperationClass::Reset);
    }

    #[test]
    fn operation_condition_is_canonical() {
        let condition = OperationCondition::when_true(
            super::super::classical::ClassicalBitId::new(0),
        );

        assert!(condition.value());
    }

    #[test]
    fn operation_sequence_is_canonical() {
        let sequence = OperationSequence::new();

        assert!(sequence.is_empty());
        assert_eq!(sequence.len(), 0);
    }
}