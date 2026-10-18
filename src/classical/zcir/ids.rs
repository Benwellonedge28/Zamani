
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! ZCIR — Canonical Identity Integration
//!
//! Path: `src/classical/zcir/ids.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Expose canonical ZUIR identities through the ZCIR module boundary without
//! creating duplicate identifier types, allocation counters, or namespaces.
//!
//! # Authority
//!
//! `crate::zuir::ids` exclusively owns general-purpose ZUIR identifiers,
//! their nonzero-value invariant, allocation, and exhaustion behavior.
//!
//! `crate::source_map` owns the legacy source-location representation.
//! `crate::zuir::source_map` owns ZUIR source provenance and its associations.
//! `crate::quantum::ir` owns quantum-domain identities.
//! `crate::quantum::ir::classical::bit::ClassicalBitId` remains the canonical
//! identity for logical classical bits in the quantum IR.
//!
//! `crate::classical::ids` is the existing classical-facing identity facade.
//! This ZCIR facade deliberately re-exports the same ZUIR definitions.
//!
//! # Owns
//!
//! This file owns only the ZCIR-facing public import boundary.
//!
//! # Does not own
//!
//! This file does not define:
//!
//! - New ID types or numeric representations.
//! - ID allocation, allocation scope, or global uniqueness.
//! - AST node identities or semantic-model identities.
//! - Source spans, source files, or provenance records.
//! - Logical or physical qubit identities.
//! - Classical-bit identities.
//! - Device, memory-address, register, or hardware identities.
//! - Content fingerprints or persistent program identities.
//! - IR storage, graph validation, or namespace membership.
//!
//! # Identity invariants
//!
//! 1. Every exported name refers to its canonical ZUIR definition.
//! 2. Re-exporting an ID does not create a new Rust type.
//! 3. IDs from different categories remain type-distinct.
//! 4. Raw ID values are opaque identities, not array indexes or addresses.
//! 5. Uniqueness is guaranteed only within the relevant coordinated
//!    allocation scope.
//! 6. Allocation exhaustion is handled by the canonical allocator.
//! 7. Deterministic artifact identity must use the specified fingerprint or
//!    persistence mechanism, not an allocation counter.
//!
//! # POCO-REAF and scalability
//!
//! This module imposes no artificial machine-size, operation-count,
//! collection-size, register-width, or device-count ceiling.
//!
//! The canonical ZUIR implementation currently represents IDs using
//! nonzero `u128` values. This is a finite representational space, not
//! literal infinity. Exhaustion must be reported rather than wrapping or
//! reusing an ID.
//!
//! # Dependency direction
//!
//! ```text
//! crate::zuir::ids
//!       |
//!       +----> crate::classical::ids
//!       |
//!       +----> crate::classical::zcir::ids
//!       |
//!       +----> crate::zuir::ir
//!       |
//!       +----> ZCIR builders, verifiers, and adapters
//! ```
//!
//! This module must remain independent of IR nodes, builders, verifiers,
//! resource planning, serialization implementations, and backends.
//!
//! # Integration contract
//!
//! - `zcir/ir/*` imports the canonical IDs for its corresponding entities.
//! - `zcir/builder/*` obtains IDs through the appropriate canonical allocator
//!   supplied by the owning program or construction context.
//! - `zcir/verify/*` validates reference integrity and uniqueness within the
//!   relevant namespace; this facade performs no graph validation.
//! - `zcir/lowering/*` preserves identity where the conversion contract
//!   requires it and records explicit mappings where identities differ.
//! - `zcir/serialization/*` uses the canonical identity representation and
//!   the applicable artifact-versioning rules.
//! - Source provenance is integrated through the existing source-map
//!   contracts rather than by defining a second `SourceId`.
//! - Quantum and classical-bit identities are imported from their canonical
//!   domain modules, never redefined here.
//!
//! # Freeze criteria
//!
//! - The repository root registers the existing `zuir` and `classical`
//!   modules.
//! - The ZCIR module root is valid Rust and declares this file.
//! - Every re-export resolves to the actual canonical ZUIR API.
//! - No competing identifier definition or allocator is introduced.
//! - The identity facade compiles and its integration tests pass.
//!
//! This file is intentionally small. Identity behavior belongs to the
//! canonical implementation, not to a second ZCIR implementation.

/// Canonical general-purpose ZUIR identities and allocation contracts.
///
/// Every item below is re-exported directly from its authoritative
/// implementation. These are the same Rust types, not ZCIR-specific
/// wrappers or aliases with independent identity semantics.
pub use crate::zuir::ids::{
    BlockId,
    DialectId,
    ExtensionId,
    FunctionId,
    IdAllocator,
    IdSpaceExhausted,
    InvalidIdValue,
    ModuleId,
    OperationId,
    ProgramId,
    RegionId,
    SourceId,
    SymbolId,
    TypeId,
    TypedId,
    ValueId,
};

#[cfg(test)]
mod tests {
    use super::*;

    use crate::classical::ids as classical_ids;

    #[test]
    fn zcir_and_classical_facades_expose_the_same_canonical_types() {
        fn accepts_program_id(_: ProgramId) {}
        fn accepts_classical_program_id(_: classical_ids::ProgramId) {}

        fn accepts_operation_id(_: OperationId) {}
        fn accepts_classical_operation_id(_: classical_ids::OperationId) {}

        let program = ProgramId::new(1).expect("nonzero program ID");
        let operation = OperationId::new(2).expect("nonzero operation ID");

        accepts_program_id(program);
        accepts_classical_program_id(program);
        accepts_operation_id(operation);
        accepts_classical_operation_id(operation);

        assert_eq!(
            core::any::type_name::<ProgramId>(),
            core::any::type_name::<classical_ids::ProgramId>()
        );

        assert_eq!(
            core::any::type_name::<OperationId>(),
            core::any::type_name::<classical_ids::OperationId>()
        );
    }

    #[test]
    fn canonical_ids_preserve_their_type_distinction() {
        let program = ProgramId::new(7).expect("nonzero program ID");
        let operation = OperationId::new(7).expect("nonzero operation ID");

        assert_eq!(program.get(), operation.get());

        assert_ne!(
            core::any::type_name::<ProgramId>(),
            core::any::type_name::<OperationId>()
        );
    }

    #[test]
    fn canonical_allocator_produces_nonzero_ids() {
        let allocator = IdAllocator::<OperationId>::new();

        let first = allocator.try_next().expect("first operation ID");
        let second = allocator.try_next().expect("second operation ID");

        assert_ne!(first.get(), 0);
        assert_ne!(second.get(), 0);
        assert_ne!(first, second);
    }

    #[test]
    fn canonical_exhaustion_and_validation_errors_are_reused() {
        let zero = ProgramId::new(0);

        assert!(matches!(zero, Err(InvalidIdValue::Zero)));

        let exhausted = IdSpaceExhausted;
        assert!(!exhausted.to_string().is_empty());
    }

    #[test]
    fn source_identity_is_not_a_source_span_or_byte_offset() {
        let source = SourceId::new(1).expect("nonzero source ID");

        assert_eq!(source.get(), 1);

        // SourceId identifies a source artifact. Source spans and byte
        // positions remain owned by their respective source-map modules.
        assert_eq!(
            core::any::type_name::<SourceId>(),
            core::any::type_name::<crate::zuir::ids::SourceId>()
        );
    }
}
