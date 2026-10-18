
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # ZUIR-to-ZCIR Canonical Representation Adapter
//!
//! Canonical path: `src/classical/zcir/lowering/from_zuir.rs`
//! Edition: Rust 2021.
//! Minimum intended Rust version: 1.97 or later.
//! Safety policy: unsafe Rust is forbidden.
//!
//! ## Purpose
//!
//! Provide the reverse integration boundary for consuming canonical ZUIR
//! entities through the Zamani Classical Intermediate Representation
//! (ZCIR) API.
//!
//! The current ZCIR IR module re-exports canonical ZUIR types and defines
//! classical-facing aliases for those same types. Consequently, this
//! adapter preserves the original entity rather than rebuilding it.
//!
//! ## Authority
//!
//! This module follows the contracts of:
//!
//! - `src/classical/zcir/ir/mod.rs`
//! - `src/classical/zcir/lowering/to_zuir.rs`
//! - `src/zuir/ir/mod.rs`
//! - `src/zuir/ids.rs`
//! - `src/zuir/verify/mod.rs`
//! - `src/zuir/lowering/mod.rs`
//! - `src/zuir/ZUIR-SPEC.md`
//! - `grammar/DESIGN.md`
//! - `grammar/Zamani.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/validation/semantic-boundaries.md`
//! - `grammar/specification/poco-reaf.md`
//! - `grammar/specification/scalability-model.md`
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - The identity adapter from canonical ZUIR entities to the ZCIR
//!   integration boundary.
//! - Borrowed read-only and mutable access helpers.
//! - Tests for identity and structural preservation.
//!
//! It does not own:
//!
//! - Classical IR graph storage or a second identity system.
//! - Type inference, source parsing, or semantic analysis.
//! - Operation legalization or dialect registration.
//! - Structural, type, dominance, effect, ownership, or resource
//!   verification algorithms.
//! - Resource allocation, scheduling, code generation, or execution.
//! - Source-map storage, diagnostics storage, or serialization.
//!
//! ## Integration contract
//!
//! 1. ZUIR remains the owner of canonical shared IR entities and IDs.
//! 2. ZCIR continues to expose the approved canonical entities through
//!    aliases and re-exports.
//! 3. Conversion preserves identity, ordering, metadata, types, effects,
//!    operands, results, regions, and control-flow information because
//!    it does not reconstruct or mutate the entity.
//! 4. Borrowed conversion does not clone or allocate.
//! 5. Conversion does not establish structural or semantic validity.
//! 6. Consumers must invoke the appropriate canonical verifier before
//!    trusting unverified IR.
//! 7. A future distinct classical representation requires a dedicated
//!    conversion with explicit losslessness and compatibility contracts.
//! 8. Unsupported or lossy conversion must never be reported as a
//!    successful identity conversion.
//!
//! ## Scalability and POCO-REAF
//!
//! This adapter adds no universal maximum for modules, functions, blocks,
//! operations, values, regions, resources, or hardware populations.
//!
//! Owned conversion performs constant adapter work. Borrowed conversion
//! does not allocate or clone. Moving a value still follows Rust ownership
//! semantics; this API does not promise zero physical copying by every
//! possible consumer or runtime.
//!
//! Actual execution remains subject to finite memory, address space,
//! identifier space, implementation capability, configured budgets, and
//! target feasibility. This adapter imposes no additional capacity ceiling.
//!
//! ## Freeze criteria
//!
//! - `to_zuir::ZuirCanonical` remains the shared identity contract.
//! - The sibling module is registered in `lowering/mod.rs`.
//! - The parent module chain is registered exactly once.
//! - Tests and repository checks pass.
//! - No competing IR graph or identifier system is introduced.

use super::to_zuir::ZuirCanonical;

/// Consume a canonical ZUIR entity through the ZCIR integration boundary.
///
/// Because the current ZCIR entities are aliases or re-exports of
/// canonical ZUIR types, this function returns the original value
/// without reconstructing it.
///
/// The generic return type is intentional: for an entity that has a
/// ZCIR-facing type alias, that alias resolves to the same Rust type.
///
/// This operation does not verify the entity, its containing graph, or
/// its suitability for any execution target.
///
/// # Complexity
///
/// O(1) adapter work, excluding work performed by downstream consumers.
///
/// # Examples
///
/// ```
/// use zamani_compiler::classical::zcir::lowering::from_zuir::from_zuir;
/// use zamani_compiler::zuir::ids::BlockId;
/// use zamani_compiler::zuir::ir::ZuirBlock;
///
/// let id = BlockId::new(1).expect("nonzero block ID");
/// let block = ZuirBlock::new(id);
/// let received: ZuirBlock = from_zuir(block);
///
/// assert_eq!(received.id(), id);
/// ```
#[must_use = "the returned canonical entity must be used"]
pub fn from_zuir<T: ZuirCanonical>(entity: T) -> T {
    entity
}

/// Borrow a canonical ZUIR entity for read-only use through ZCIR.
///
/// The original entity remains owned by the caller. The returned reference
/// has the same lifetime as the input reference.
///
/// No clone, allocation, graph traversal, or identity remapping occurs.
#[must_use]
pub fn as_zcir<T: ZuirCanonical>(entity: &T) -> &T {
    entity
}

/// Borrow a canonical ZUIR entity mutably through the ZCIR boundary.
///
/// This does not bypass the entity's public API or invariants. Mutations
/// remain subject to Rust's borrowing rules and the canonical type's
/// validation contracts.
///
/// The caller is responsible for re-verifying the affected IR wherever
/// the mutation may invalidate previously established verification results.
#[must_use]
pub fn as_zcir_mut<T: ZuirCanonical>(entity: &mut T) -> &mut T {
    entity
}

#[cfg(test)]
mod tests {
    use super::{as_zcir, as_zcir_mut, from_zuir, ZuirCanonical};
    use crate::zuir::ids::BlockId;
    use crate::zuir::ir::ZuirBlock;

    fn block_id(value: u128) -> BlockId {
        BlockId::new(value).expect("test block IDs must be nonzero")
    }

    #[test]
    fn conversion_preserves_canonical_identity() {
        let id = block_id(1);
        let block = ZuirBlock::new(id);

        let received = from_zuir(block);

        assert_eq!(received.id(), id);
    }

    #[test]
    fn borrowed_conversion_preserves_the_original() {
        let id = block_id(2);
        let block = ZuirBlock::new(id);

        let received = as_zcir(&block);

        assert_eq!(received.id(), id);
        assert_eq!(block.id(), id);
    }

    #[test]
    fn mutable_borrow_does_not_replace_identity() {
        let id = block_id(3);
        let mut block = ZuirBlock::new(id);

        {
            let received = as_zcir_mut(&mut block);
            assert_eq!(received.id(), id);
        }

        assert_eq!(block.id(), id);
    }

    #[test]
    fn canonical_entities_implement_the_shared_contract() {
        fn assert_canonical<T: ZuirCanonical>() {}

        assert_canonical::<ZuirBlock>();
    }

    #[test]
    fn conversion_preserves_block_structure() {
        let id = block_id(4);
        let mut block = ZuirBlock::new(id);

        block
            .validate()
            .expect("a newly constructed empty block should be valid");

        let received = from_zuir(block);

        assert_eq!(received.id(), id);
        assert_eq!(received.argument_count(), 0);
        assert_eq!(received.operation_count(), 0);
        assert!(received.operations().is_empty());
    }
}
