#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! Zamani Classical Computing — Canonical Identity Facade
//!
//! Path: `src/classical/ids.rs`
//! Rust edition: 2021
//! Minimum supported Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Exposes the canonical ZUIR identity types through the classical-computing
//! module boundary without defining competing identity types or allocators.
//!
//! # Authority
//!
//! `crate::zuir::ids` is the canonical owner of general ZUIR identities,
//! typed-ID invariants, allocation, and allocation-exhaustion behavior.
//!
//! `crate::source_map` remains the owner of source-file IDs, byte positions,
//! and source spans.
//!
//! `crate::quantum::ir` remains the owner of quantum-domain identities,
//! including logical and physical qubit identities and its ClassicalBitId.
//!
//! # Owns
//!
//! This module owns only the classical-facing re-export boundary.
//!
//! # Does not own
//!
//! This module does not define:
//!
//! - a second ProgramId, ModuleId, FunctionId, RegionId, BlockId,
//!   OperationId, ValueId, TypeId, DialectId, SymbolId, ExtensionId,
//!   or SourceId;
//! - a classical-bit, quantum-bit, device, register, memory-address,
//!   hardware, or backend identity;
//! - identity allocation, global uniqueness, or namespace membership;
//! - source spans, provenance, semantic types, IR nodes, or runtime state;
//! - persistent content hashes or cross-build identity.
//!
//! # Identity contract
//!
//! A re-export is the same Rust type as its canonical definition. It does not
//! create a wrapper, conversion, independent counter, or separate identity
//! namespace.
//!
//! A typed identifier is an opaque identity token. It is not an array index,
//! memory address, source position, execution order, or content fingerprint.
//!
//! The canonical allocator guarantees uniqueness only within its own
//! allocation sequence. Owners that merge entities from separate allocation
//! scopes must explicitly resolve collisions or establish a shared allocator.
//!
//! # POCO-REAF and scalability
//!
//! This module imposes no machine, hardware, program-size, collection-size,
//! tensor-rank, or device-count ceiling. The canonical allocator uses a finite
//! nonzero u128 identity representation and reports exhaustion rather than
//! wrapping or reusing an identifier.
//!
//! Literal infinity is not representable. Actual limits must be reported as
//! representation or resource constraints, never as arbitrary language
//! semantics.
//!
//! # Integration
//!
//! - `classical/mod.rs` declares `pub mod ids;` and may selectively re-export
//!   these types from the classical public API.
//! - Classical semantic and IR adapters use these shared IDs where the
//!   corresponding ZUIR entity is being identified.
//! - `src/zuir/ids.rs` remains the only owner of these generic identity types.
//! - `src/source_map.rs` remains authoritative for source locations.
//! - `src/quantum/ir/classical/bit.rs` remains authoritative for
//!   `ClassicalBitId`.
//! - Namespace membership, declaration validity, and cross-program ownership
//!   are checked by the owning program, registry, or IR verifier.
//!
//! # Freeze rule
//!
//! Do not add a locally defined ID type merely to make a classical API appear
//! self-contained. If a genuinely classical-only identity is required, first
//! establish that no existing canonical owner already defines it, then
//! document its scope, persistence, allocation, and compatibility contract.
//!
//! This facade is considered verified only after the module path and exports
//! compile against the repository's actual module declarations and tests.

/// Canonical identifiers and allocator contracts shared with ZUIR.
///
/// These are re-exports, not new classical-specific ID definitions.
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

    #[test]
    fn canonical_ids_are_available_from_the_classical_boundary() {
        let program = ProgramId::new(1).expect("nonzero program ID");
        let value = ValueId::new(1).expect("nonzero value ID");

        // Equal numeric payloads do not make distinct ID categories equal.
        assert_eq!(program.get(), value.get());
        assert_ne!(
            core::any::type_name::<ProgramId>(),
            core::any::type_name::<ValueId>()
        );
    }

    #[test]
    fn canonical_allocator_produces_nonzero_ids() {
        let allocator = IdAllocator::<ValueId>::new();
        let first = allocator.try_next().expect("first ID");

        assert_ne!(first.get(), 0);
    }

    #[test]
    fn independently_allocated_ids_are_not_claimed_to_be_globally_unique() {
        let first_allocator = IdAllocator::<ValueId>::new();
        let second_allocator = IdAllocator::<ValueId>::new();

        let first = first_allocator.try_next().expect("first allocator");
        let second = second_allocator.try_next().expect("second allocator");

        // Separate allocators begin independently. Namespace owners must
        // coordinate allocation when entities are combined.
        assert_eq!(first, second);
    }

    #[test]
    fn zero_is_rejected_by_the_canonical_id_contract() {
        assert!(matches!(
            ProgramId::new(0),
            Err(InvalidIdValue::Zero)
        ));
    }
}