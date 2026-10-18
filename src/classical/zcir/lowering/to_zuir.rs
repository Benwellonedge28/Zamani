
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # ZCIR-to-ZUIR Canonical Representation Adapter
//!
//! Path: `src/classical/zcir/lowering/to_zuir.rs`
//! Edition: Rust 2021
//! Minimum intended Rust version: 1.97 or later.
//! Safety: Unsafe Rust is forbidden.
//!
//! ## Purpose
//!
//! Provide the explicit ZCIR-to-ZUIR integration boundary for values that
//! already use the canonical ZUIR representation.
//!
//! The current ZCIR IR boundary re-exports canonical ZUIR entities instead
//! of defining a second classical IR graph. Consequently, converting one
//! of these entities to its ZUIR representation is an identity operation.
//! Reconstructing it would be unnecessary and could lose information.
//!
//! ## Architectural authority
//!
//! This module follows the contracts defined by:
//!
//! - `src/classical/zcir/ir/mod.rs`
//! - `src/classical/zcir/ids.rs`
//! - `src/zuir/ir/mod.rs`
//! - `src/zuir/ids.rs`
//! - `src/zuir/lowering/mod.rs`
//! - `src/zuir/lowering/api.rs`
//! - `src/zuir/lowering/context.rs`
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
//! - The marker contract for representations already backed by canonical
//!   ZUIR entities.
//! - The identity conversion for those entities.
//! - Borrowed access to those same entities without cloning.
//!
//! This module does not own:
//!
//! - A second classical IR representation.
//! - Type conversion or type inference.
//! - Operation, value, block, region, or identity allocation.
//! - Semantic analysis or source parsing.
//! - Whole-program verification.
//! - Dialect registration or operation legalization.
//! - Resource allocation, scheduling, routing, or target selection.
//! - Serialization or artifact version negotiation.
//! - A claim that an IR is semantically valid merely because it has a
//!   canonical ZUIR representation.
//!
//! ## Integration contract
//!
//! 1. ZCIR IR entities remain aliases or re-exports of canonical ZUIR types.
//! 2. `to_zuir` accepts only types explicitly implementing `ZuirCanonical`.
//! 3. Conversion preserves the complete entity because it does not
//!    reconstruct, normalize, reorder, or discard any fields.
//! 4. Borrowed conversion does not clone or allocate.
//! 5. This adapter does not validate an entity or make it executable.
//! 6. A future distinct classical representation must use a separate,
//!    explicit conversion with a documented semantic-preservation contract.
//! 7. Unsupported or lossy conversions must be reported by that future
//!    converter rather than silently routed through this identity adapter.
//!
//! ## Scalability
//!
//! This adapter introduces no fixed operation, block, value, function,
//! module, collection, processor, device, or memory limits.
//!
//! Its identity conversion performs constant work and does not traverse
//! the represented program. Borrowed access does not allocate or clone.
//!
//! The represented program remains subject to the actual storage,
//! identifier, address-space, and resource limits of its underlying
//! canonical representation.
//!
//! ## Freeze criteria
//!
//! - Every implementation below refers to an existing canonical ZUIR type.
//! - No competing IR or identity system is introduced.
//! - No unsafe Rust is used.
//! - The module is registered by `lowering/mod.rs`.
//! - Repository formatting, compilation, tests, and linting pass.
//!
//! This adapter is intentionally an identity boundary. It is not a
//! substitute for a future verified conversion from a distinct IR.

use crate::zuir::ir::{
    ZuirBlock,
    ZuirFunction,
    ZuirModule,
    ZuirOperation,
    ZuirProgram,
    ZuirRegion,
};

/// Marks a type whose representation is already owned by canonical ZUIR.
///
/// Implement this trait only for types that are the canonical ZUIR entity
/// itself or a deliberate type alias for that entity.
///
/// Implementations must not be added for an independent representation
/// merely because it contains similar data. Such a representation requires
/// an explicit conversion that preserves its defined semantics.
///
/// This trait does not assert that a value is structurally valid,
/// semantically valid, verified, portable, or executable.
pub trait ZuirCanonical {}

/// Canonical ZUIR program containers.
impl<M> ZuirCanonical for ZuirProgram<M> {}

/// Canonical ZUIR module containers.
impl<F> ZuirCanonical for ZuirModule<F> {}

/// Canonical ZUIR functions.
impl ZuirCanonical for ZuirFunction {}

/// Canonical ZUIR basic blocks.
impl ZuirCanonical for ZuirBlock {}

/// Canonical ZUIR operations.
impl ZuirCanonical for ZuirOperation {}

/// Canonical ZUIR regions.
impl ZuirCanonical for ZuirRegion {}

/// Return a canonical ZUIR entity from the ZCIR integration boundary.
///
/// Because ZCIR uses the canonical ZUIR representation, this operation
/// returns the original value without reconstructing it.
///
/// # Complexity
///
/// O(1) adapter work, excluding any work associated with moving the value
/// according to Rust's ownership and type semantics.
///
/// # Validation
///
/// This function does not validate the entity or its containing graph.
/// Call the appropriate canonical validators before trusting an IR for
/// optimization, further lowering, or execution.
///
/// # Examples
///
/// ```
/// use zamani::classical::zcir::lowering::to_zuir::to_zuir;
/// use zamani::zuir::ir::ZuirBlock;
/// use zamani::zuir::ids::BlockId;
///
/// let id = BlockId::new(1).expect("nonzero block ID");
/// let block = ZuirBlock::new(id);
/// let converted: ZuirBlock = to_zuir(block);
///
/// assert_eq!(converted.id(), id);
/// ```
#[must_use = "the returned canonical ZUIR entity must be used"]
pub fn to_zuir<T: ZuirCanonical>(entity: T) -> T {
    entity
}

/// Borrow a canonical ZUIR entity without cloning or allocating.
///
/// This is useful when a consumer needs read-only access to the canonical
/// representation but must retain ownership of the original entity.
///
/// The returned reference has the same lifetime as the input reference.
#[must_use]
pub fn as_zuir<T: ZuirCanonical>(entity: &T) -> &T {
    entity
}

/// Borrow a canonical ZUIR entity mutably without replacing its identity.
///
/// Mutability is governed by the canonical entity's own API and invariants.
/// This function does not bypass private fields, validation, or ownership
/// rules.
///
/// Use this only when the caller is authorized to modify the entity.
#[must_use]
pub fn as_zuir_mut<T: ZuirCanonical>(entity: &mut T) -> &mut T {
    entity
}

#[cfg(test)]
mod tests {
    use super::{as_zuir, as_zuir_mut, to_zuir, ZuirCanonical};
    use crate::zuir::ids::BlockId;
    use crate::zuir::ir::ZuirBlock;

    #[test]
    fn canonical_block_uses_identity_conversion() {
        let id = BlockId::new(1).expect("nonzero block ID");
        let block = ZuirBlock::new(id);

        let converted = to_zuir(block);

        assert_eq!(converted.id(), id);
    }

    #[test]
    fn borrowed_conversion_preserves_the_original_entity() {
        let id = BlockId::new(2).expect("nonzero block ID");
        let block = ZuirBlock::new(id);

        let borrowed = as_zuir(&block);

        assert_eq!(borrowed.id(), id);
        assert_eq!(block.id(), id);
    }

    #[test]
    fn mutable_borrow_preserves_the_original_identity() {
        let id = BlockId::new(3).expect("nonzero block ID");
        let mut block = ZuirBlock::new(id);

        let borrowed = as_zuir_mut(&mut block);

        assert_eq!(borrowed.id(), id);
        assert_eq!(block.id(), id);
    }

    #[test]
    fn canonical_block_implements_the_marker_contract() {
        fn assert_canonical<T: ZuirCanonical>() {}

        assert_canonical::<ZuirBlock>();
    }

    #[test]
    fn identity_conversion_preserves_block_structure() {
        let id = BlockId::new(4).expect("nonzero block ID");
        let mut block = ZuirBlock::new(id);

        block
            .validate()
            .expect("a newly constructed empty block is structurally valid");

        let converted = to_zuir(block);

        assert_eq!(converted.id(), id);
        assert_eq!(converted.operation_count(), 0);
        assert_eq!(converted.argument_count(), 0);
        assert!(converted.operations().is_empty());
    }
}
