//! Zamani Quantum IR — Canonical Classical Subsystem
//!
//! Production-grade, hardware-independent classical semantics for the Zamani
//! Quantum Intermediate Representation.
//!
//! # Architectural role
//!
//! This module is the canonical parent module for all classical semantics
//! inside `quantum::ir`.
//!
//! It describes:
//!
//! - classical identities;
//! - classical references;
//! - classical scalar values;
//! - arbitrary-width integers;
//! - finite floating-point values;
//! - angles;
//! - booleans;
//! - classical arrays;
//! - symbolic expressions;
//! - predicates;
//! - assignments;
//! - external calls;
//! - classical resource semantics.
//!
//! It does NOT describe:
//!
//! - CPU registers;
//! - physical memory addresses;
//! - FPGA registers;
//! - GPU memory;
//! - device readout buffers;
//! - QPU execution;
//! - hardware allocation;
//! - hardware routing;
//! - scheduling;
//! - simulation state;
//! - frontend syntax;
//! - optimization algorithms;
//! - vendor APIs;
//! - credentials;
//! - operating-system resources.
//!
//! Those responsibilities belong to downstream subsystems.
//!
//! # Universal-program principle
//!
//! A Zamani program is written once at the semantic level and may be lowered
//! to any compatible execution target.
//!
//! The classical subsystem therefore contains NO architectural fixed limit
//! such as:
//!
//! ```text
//! MAX_CLASSICAL_BITS = 64
//! MAX_CLASSICAL_BITS = 4096
//! MAX_CLASSICAL_ARRAY = 1_000_000
//! MAX_ARGUMENTS = 32
//! ```
//!
//! Classical resource quantities are program data.
//!
//! Actual finite limits arise only from:
//!
//! 1. host representation limits;
//! 2. explicit compiler/security/resource policies;
//! 3. available memory;
//! 4. target capabilities;
//! 5. runtime resources;
//! 6. backend constraints.
//!
//! Those limits are not semantic limits of Zamani.
//!
//! # Canonical ownership
//!
//! The classical subsystem deliberately separates concepts that were
//! historically combined in the legacy `classical.rs` implementation.
//!
//! ```text
//! classical/
//! │
//! ├── mod.rs
//! │   canonical module boundary and public API
//! │
//! ├── bit.rs
//! │   ClassicalBitId / ClassicalBitRef
//! │
//! ├── value.rs
//! │   ClassicalValue and value-level semantics
//! │
//! ├── integer.rs
//! │   integer semantics
//! │
//! ├── float.rs
//! │   finite floating-point semantics
//! │
//! ├── boolean.rs
//! │   boolean semantics
//! │
//! ├── angle.rs
//! │   angle semantics
//! │
//! ├── array.rs
//! │   array shape/index/storage semantics
//! │
//! ├── expression.rs
//! │   symbolic classical expressions
//! │
//! ├── predicate.rs
//! │   classical predicates
//! │
//! ├── assignment.rs
//! │   classical state-update semantics
//! │
//! └── extern_call.rs
//!     declarative external-call semantics
//! ```
//!
//! Every concept has one canonical owner.
//!
//! # Canonical identity rule
//!
//! The canonical classical-bit identity is owned exclusively by:
//!
//! ```text
//! quantum::ir::classical::bit::ClassicalBitId
//! ```
//!
//! and exposed through:
//!
//! ```text
//! quantum::ir::classical::ClassicalBitId
//! ```
//!
//! No other module may define another `ClassicalBitId`.
//!
//! In particular, the following is forbidden:
//!
//! ```text
//! classical.rs        -> ClassicalBitId
//! classical/bit.rs    -> ClassicalBitId
//! measurement.rs      -> ClassicalBitId
//! ```
//!
//! There must be exactly one canonical definition.
//!
//! # Quantum integration
//!
//! Classical computation interacts with quantum computation through higher
//! level IR constructs such as measurement, conditions and operations.
//!
//! The canonical quantum identity remains:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! ```
//!
//! This module does not redefine or own `QubitId`.
//!
//! The dependency direction is:
//!
//! ```text
//! quantum::ir::qubit
//!          │
//!          │ canonical quantum identity
//!          ▼
//! measurement / operation / control-flow
//!          │
//!          ▼
//! quantum::ir::classical
//! ```
//!
//! More specifically:
//!
//! ```text
//! ClassicalBitId
//!       │
//!       ▼
//! measurement destination
//!       │
//!       ▼
//! ClassicalValue
//!       │
//!       ▼
//! ClassicalExpression
//!       │
//!       ▼
//! ClassicalPredicate
//!       │
//!       ▼
//! conditional quantum operation
//!       │
//!       ▼
//! quantum::ir::qubit::QubitId
//! ```
//!
//! The classical identity layer itself does not depend on `QubitId`. This
//! avoids unnecessary circular dependencies.
//!
//! # Semantic versus physical classical state
//!
//! This module represents:
//!
//! ```text
//! WHAT classical information means
//! ```
//!
//! It does not represent:
//!
//! ```text
//! WHERE it is physically stored
//! WHEN it is evaluated
//! HOW hardware implements it
//! ```
//!
//! For example:
//!
//! ```text
//! ClassicalBitId(10)
//! ```
//!
//! means a logical classical resource.
//!
//! It does NOT mean:
//!
//! ```text
//! CPU register 10
//! RAM byte 10
//! FPGA register 10
//! ADC buffer 10
//! QPU readout slot 10
//! ```
//!
//! Physical mapping occurs later.
//!
//! # Dynamic quantum programs
//!
//! The classical subsystem must support dynamic quantum computation.
//!
//! Example:
//!
//! ```text
//! measure(q0) -> c0
//!
//! if c0 == 1 {
//!     x(q1)
//! }
//! ```
//!
//! The classical subsystem represents:
//!
//! ```text
//! c0
//! │
//! ├── value
//! ├── expression
//! └── predicate
//! ```
//!
//! The quantum operation and control-flow modules consume those semantics.
//!
//! # Arbitrary-width integers
//!
//! A universal language cannot make `i64` or `u64` the semantic maximum of
//! classical integers.
//!
//! The integer subsystem therefore provides arbitrary-width representations.
//!
//! Their practical size is constrained by available resources rather than a
//! fixed Zamani machine width.
//!
//! # Arrays
//!
//! Array metadata and concrete storage are deliberately separated.
//!
//! A declaration such as:
//!
//! ```text
//! array<bool, 1_000_000_000>
//! ```
//!
//! must not automatically allocate one billion Rust values merely to describe
//! its shape.
//!
//! `array.rs` therefore owns:
//!
//! - shape;
//! - rank;
//! - logical size;
//! - indexing;
//! - slicing;
//! - sparse storage;
//! - optional dense storage;
//! - checked arithmetic.
//!
//! Resource policies decide whether concrete materialization is permitted.
//!
//! # Determinism
//!
//! Classical IR structures participate in deterministic compilation.
//!
//! Public classical collections must therefore prefer deterministic structures
//! where ordering is semantically observable or contributes to canonical
//! serialization/hashing.
//!
//! Examples include:
//!
//! ```text
//! BTreeMap
//! BTreeSet
//! ordered Vec
//! canonical byte representations
//! ```
//!
//! Randomized map iteration must not determine canonical IR identity.
//!
//! # Error policy
//!
//! Classical IR constructors and transformations must not silently:
//!
//! - truncate integers;
//! - wrap indexes;
//! - ignore invalid operands;
//! - discard expressions;
//! - discard unknown external-call metadata;
//! - accept invalid floating-point values;
//! - silently coerce incompatible types;
//! - ignore failed validation.
//!
//! Invalid semantic state must be represented by an explicit error.
//!
//! # No execution
//!
//! None of the modules below execute classical computation.
//!
//! They describe semantics for downstream execution.
//!
//! In particular:
//!
//! ```text
//! expression.rs
//! predicate.rs
//! assignment.rs
//! extern_call.rs
//! ```
//!
//! do not become an interpreter.
//!
//! A runtime may interpret them later.
//!
//! # Integration contracts
//!
//! ## `quantum::ir::qubit`
//!
//! Owns:
//!
//! ```text
//! QubitId
//! PhysicalQubitId
//! ```
//!
//! The classical subsystem must never duplicate those types.
//!
//! ## `quantum::ir::measurement`
//!
//! Consumes:
//!
//! ```text
//! ClassicalBitId
//! ClassicalBitRef
//! ```
//!
//! as logical measurement destinations.
//!
//! ## `quantum::ir::operation`
//!
//! Consumes classical values, expressions, predicates, assignments and
//! references when representing hybrid operations.
//!
//! ## `quantum::ir::control_flow`
//!
//! Consumes `ClassicalPredicate` and related classical expression semantics.
//!
//! ## `quantum::ir::program`
//!
//! Owns program-level declaration/lifetime and namespace relationships.
//!
//! The classical subsystem does not become a second program container.
//!
//! ## `quantum::ir::validation`
//!
//! Validates:
//!
//! - declarations;
//! - identifier membership;
//! - expression structure;
//! - type relationships;
//! - array bounds;
//! - assignment compatibility;
//! - external-call references;
//! - resource requirements.
//!
//! ## `quantum::ir::serialization`
//!
//! Owns canonical persistence and encoding.
//!
//! `classical/mod.rs` does not create a second serialization format.
//!
//! ## `quantum::ir::hash`
//!
//! Owns canonical content hashing.
//!
//! Classical semantic data must be represented deterministically so hashing can
//! remain stable.
//!
//! ## `quantum::ir::analysis`
//!
//! May inspect classical dependencies and resource usage but must not mutate
//! classical IR merely to perform analysis.
//!
//! ## `quantum::frontend`
//!
//! Lowers source-language classical constructs into these canonical types.
//!
//! The classical IR must not depend on the frontend.
//!
//! ## `quantum::hardware`
//!
//! May map logical classical resources to target-specific implementation
//! details.
//!
//! The hardware layer must not redefine canonical classical types.
//!
//! # Compatibility with the existing repository
//!
//! The repository currently contains a legacy:
//!
//! ```text
//! src/quantum/ir/classical.rs
//! ```
//!
//! and the newer directory:
//!
//! ```text
//! src/quantum/ir/classical/
//! ```
//!
//! These cannot coexist as the same Rust module namespace.
//!
//! The production architecture therefore requires:
//!
//! ```text
//! src/quantum/ir/classical.rs
//!             │
//!             └── REMOVE AFTER MIGRATION
//!
//! src/quantum/ir/classical/mod.rs
//!             │
//!             └── CANONICAL CLASSICAL MODULE
//! ```
//!
//! Before deleting the legacy file, any unique public API that is still
//! required must be migrated into the appropriate child module or explicitly
//! re-exported here.
//!
//! No duplicate type definitions may survive the migration.
//!
//! # Public API policy
//!
//! The parent module provides two levels of API.
//!
//! Explicit module paths remain available:
//!
//! ```text
//! quantum::ir::classical::bit
//! quantum::ir::classical::value
//! quantum::ir::classical::integer
//! quantum::ir::classical::float
//! quantum::ir::classical::boolean
//! quantum::ir::classical::angle
//! quantum::ir::classical::array
//! quantum::ir::classical::expression
//! quantum::ir::classical::predicate
//! quantum::ir::classical::assignment
//! quantum::ir::classical::extern_call
//! ```
//!
//! Frequently used canonical types are re-exported from this module so
//! downstream IR code does not need unnecessarily deep paths.
//!
//! # API stability
//!
//! This file owns:
//!
//! - child-module declarations;
//! - public classical API exposure;
//! - compatibility re-exports;
//! - module-level invariants;
//! - cross-module classical tests.
//!
//! It does not own the implementation of individual classical concepts.
//!
//! Therefore changes to expression internals, integer storage, array storage,
//! etc. should normally not require changes to this file unless the public
//! contract itself changes.
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
//! - no `unsafe`.
//!
//! This module explicitly forbids unsafe code.
//!
//! # Security
//!
//! Logical classical identifiers are not authority.
//!
//! Possessing:
//!
//! ```text
//! ClassicalBitId::new(1_000_000)
//! ```
//!
//! does not grant access to that resource.
//!
//! Declaration membership, capability authorization and execution permission
//! are established by higher layers.
//!
//! External calls are particularly security-sensitive. `extern_call.rs`
//! describes intent and requirements but never grants execution authority.
//!
//! # Scalability contract
//!
//! The parent module contains no fixed machine-size constants.
//!
//! It must remain valid for programs ranging from:
//!
//! ```text
//! 1 classical bit
//! 2 classical bits
//! 10^3 classical bits
//! 10^6 classical bits
//! N classical bits
//! ```
//!
//! and for future target architectures with substantially different
//! classical-control systems.
//!
//! No test value in this file is an architectural limit.
//!
//! # No unsafe
//!
//! This module and its descendants must compile with:
//!
//! ```text
//! #![forbid(unsafe_code)]
//! ```
//!
//! The canonical classical subsystem must contain no unsafe Rust.
//!
//! =============================================================================
//! Implementation
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

// =============================================================================
// Child modules
// =============================================================================

/// Classical angle semantics.
pub mod angle;

/// Classical array shape, indexing and storage semantics.
pub mod array;

/// Classical assignment semantics.
pub mod assignment;

/// Canonical logical classical-bit identity.
pub mod bit;

/// Boolean semantics.
pub mod boolean;

/// Classical symbolic expression semantics.
pub mod expression;

/// Declarative external-call semantics.
pub mod extern_call;

/// Finite floating-point semantics.
pub mod float;

/// Integer semantics, including arbitrary-width values.
pub mod integer;

/// Classical predicates and conditions.
pub mod predicate;

/// Canonical classical runtime/IR values.
pub mod value;

// =============================================================================
// Canonical identity exports
// =============================================================================
//
// `bit.rs` is the sole owner of ClassicalBitId.
//
// The parent re-exports it so both of these canonical paths are available:
//
//     quantum::ir::classical::bit::ClassicalBitId
//     quantum::ir::classical::ClassicalBitId
//
// There must never be another ClassicalBitId definition.

pub use bit::{
    ClassicalBitId,
    ClassicalBitMembership,
    ClassicalBitRef,
};

// =============================================================================
// Classical value exports
// =============================================================================

pub use value::{
    BigInt,
    BigUint,
    ClassicalValue,
    ClassicalValueError,
    ClassicalValueKind,
    FiniteFloat,
};

// =============================================================================
// Classical array exports
// =============================================================================
//
// Keep the complete array module publicly accessible through:
//
//     quantum::ir::classical::array
//
// The most fundamental structural types are also exposed here.

pub use array::{
    ClassicalArray,
    ClassicalArrayError,
    ClassicalArrayIndex,
    ClassicalArrayShape,
    ClassicalArraySlice,
};

// =============================================================================
// Integer exports
// =============================================================================
//
// The integer module owns integer-specific semantics. The parent exposes the
// canonical integer types where they exist without redefining them.

pub use integer::{
    BigInt as IntegerBigInt,
    BigUint as IntegerBigUint,
};

// =============================================================================
// Float exports
// =============================================================================

pub use float::{
    ClassicalFloat,
    ClassicalFloatError,
};

// =============================================================================
// Boolean exports
// =============================================================================

pub use boolean::{
    ClassicalBoolean,
    ClassicalBooleanError,
};

// =============================================================================
// Angle exports
// =============================================================================

pub use angle::{
    ClassicalAngle,
    ClassicalAngleError,
};

// =============================================================================
// Expression exports
// =============================================================================
//
// Expressions are intentionally exported from their canonical child module.
// The exact expression vocabulary belongs to expression.rs.

pub use expression::{
    ClassicalExpression,
    ClassicalExpressionError,
};

// =============================================================================
// Predicate exports
// =============================================================================

pub use predicate::{
    ClassicalPredicate,
    ClassicalPredicateError,
};

// =============================================================================
// Assignment exports
// =============================================================================

pub use assignment::{
    ClassicalAssignment,
    ClassicalAssignmentError,
};

// =============================================================================
// External-call exports
// =============================================================================
//
// External calls remain declarative. They never execute from the IR.

pub use extern_call::{
    ExternalAbi,
    ExternalCall,
    ExternalCallArgument,
    ExternalCallError,
    ExternalCallResult,
    ExternalCallResultValue,
    ExternalEffect,
    ExternalLinkage,
    ExternalNamespace,
    ExternalSymbol,
};

// =============================================================================
// Canonical type aliases
// =============================================================================
//
// These aliases are semantic conveniences only. They do not create additional
// resource identity types.

/// Canonical classical-bit resource reference.
pub type ClassicalResourceRef = ClassicalBitRef;

// =============================================================================
// Module-level integration tests
// =============================================================================
//
// These tests verify invariants owned by the parent module itself.
//
// They deliberately avoid testing implementation details belonging to child
// modules. Child-specific behavior belongs in the child files.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_classical_bit_identity_is_stable() {
        let id = ClassicalBitId::new(0);

        assert_eq!(id.index(), 0);
        assert!(id.is_zero());
        assert_eq!(id.to_string(), "c0");
    }

    #[test]
    fn canonical_classical_bit_reference_is_identity_based() {
        let id = ClassicalBitId::new(42);
        let reference = ClassicalBitRef::new(id);

        assert_eq!(reference.id(), id);
        assert_eq!(reference.index(), 42);
    }

    #[test]
    fn classical_bit_identity_does_not_imply_value() {
        let zero = ClassicalBitId::new(0);
        let one = ClassicalBitId::new(1);

        assert_ne!(zero, one);

        // Identity and value are intentionally different semantic domains.
        let false_value = ClassicalValue::Bool(false);
        let true_value = ClassicalValue::Bool(true);

        assert_ne!(false_value, true_value);
    }

    #[test]
    fn canonical_module_has_no_fixed_classical_machine_limit() {
        let first = ClassicalBitId::new(0);
        let large = ClassicalBitId::new(usize::MAX);

        assert_eq!(first.index(), 0);
        assert_eq!(large.index(), usize::MAX);
    }

    #[test]
    fn classical_bit_checked_arithmetic_does_not_wrap() {
        let maximum = ClassicalBitId::new(usize::MAX);

        assert!(maximum.checked_next().is_none());
        assert!(maximum.checked_add(1).is_none());
        assert_eq!(
            maximum.checked_sub(usize::MAX),
            Some(ClassicalBitId::new(0))
        );
    }

    #[test]
    fn classical_bit_reference_is_copyable_and_deterministic() {
        let id = ClassicalBitId::new(7);

        let first = ClassicalBitRef::from(id);
        let second = first;

        assert_eq!(first, second);
        assert_eq!(first.id(), id);
    }
}