
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # ZCIR Basic-Block Integration
//!
//! Canonical path: `src/classical/zcir/ir/block.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97 or later.
//! Safety: `unsafe` Rust is forbidden.
//!
//! ## PURPOSE
//!
//! Expose the canonical ZUIR basic-block implementation through the ZCIR
//! namespace so classical computation can use the shared, target-independent
//! control-flow representation.
//!
//! ## AUTHORITY
//!
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/ir/block.rs`
//! - `src/zuir/ir/successor.rs`
//! - `src/zuir/ir/region.rs`
//! - `src/zuir/ids.rs`
//! - `src/zuir/schema/attributes.rs`
//! - `src/zuir/schema/types.rs`
//! - `grammar/DESIGN.md`
//! - `grammar/Zamani.g4`
//! - `grammar/functions/functions.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/specification/poco-reaf.md`
//!
//! ## OWNS
//!
//! This adapter owns no independent IR entity or invariant.
//!
//! ZUIR remains the sole owner of:
//!
//! - Basic-block representation and identity.
//! - Block arguments and their canonical value/type references.
//! - Operation-reference ordering.
//! - Control-flow terminators.
//! - Successor edges and their ordered arguments.
//! - Block-local validation and errors.
//!
//! This file owns only the ZCIR-facing names and compatibility boundary.
//!
//! ## DOES NOT OWN
//!
//! This file does not own:
//!
//! - ID allocation or global identity uniqueness.
//! - Operation definitions or operation payloads.
//! - Region storage or region ownership.
//! - Type definitions or type inference.
//! - Source syntax, AST construction, or semantic analysis.
//! - SSA dominance, whole-CFG verification, or whole-program verification.
//! - Hardware selection, scheduling, memory allocation, or runtime execution.
//! - A separate classical control-flow representation.
//!
//! ## INTEGRATION CONTRACT
//!
//! `ZcirBlock` is exactly `ZuirBlock`; it is not a wrapper containing a second
//! block. The same rule applies to block arguments, terminators, switch cases,
//! errors, results, and successor edges.
//!
//! `src/classical/zcir/ir/mod.rs` must declare this module and re-export the
//! intended public names. The ZCIR root must then expose the IR module through
//! its actual, valid Rust module tree.
//!
//! `src/zuir/ir/region.rs` remains responsible for ordered region membership
//! and region ownership. The full verifier must establish that a block's
//! parent region and the region's block list agree.
//!
//! `src/zuir/ir/operation.rs` remains responsible for operation definitions.
//! Blocks store canonical `OperationId` references, not duplicate operations.
//!
//! `src/zuir/ir/successor.rs` remains responsible for successor edges. The
//! full verifier resolves target blocks, checks edge arguments and types, and
//! establishes control-flow and SSA correctness.
//!
//! `src/zuir/ids.rs` remains responsible for typed IDs. ZCIR must not create
//! alternative block, region, operation, value, or type identifiers.
//!
//! The frontend grammar and semantic model establish source meaning before
//! the compiler builds or transforms this representation.
//!
//! ## SCALABILITY AND POCO-REAF
//!
//! The adapter introduces no fixed maximum for block arguments, operations,
//! successor arguments, blocks, functions, processors, devices, or other
//! hardware resources.
//!
//! Dynamic collection storage and fallible reservation remain the
//! responsibility of the canonical implementation. The canonical block API
//! exposes fallible reservation for its argument and operation-reference
//! collections.
//!
//! No finite implementation can promise literal infinite memory or execution
//! time. POCO-REAF requires avoiding arbitrary language-wide capacity ceilings,
//! preserving specified semantics, and reporting invalid or infeasible
//! realizations explicitly.
//!
//! ## VALIDITY AND TRUST
//!
//! A block that passes local validation is not necessarily valid in its
//! enclosing IR. Local validation does not establish that referenced IDs
//! exist, that operations belong to the correct block, that types agree, or
//! that SSA and control-flow rules hold.
//!
//! Decoded or transformed blocks must pass canonical local validation and the
//! applicable whole-IR verifier before trusted optimization, lowering, or
//! execution.
//!
//! ## FREEZE CRITERIA
//!
//! - No competing ZCIR block representation is introduced.
//! - Canonical ZUIR IDs, terminators, and successor edges are reused.
//! - The adapter is registered only through real Rust module declarations.
//! - Tests establish type identity and canonical behavior.
//! - Repository formatting, compilation, tests, and lint checks pass.
//! - Downstream APIs do not require changes when later ZCIR files are added,
//!   provided this public adapter contract remains unchanged.

/// Canonical block arguments, errors, results, switch cases, terminators,
/// and basic-block implementation.
pub use crate::zuir::ir::block::{
    BlockArgument,
    BlockError,
    BlockResult,
    BlockTerminator,
    SwitchCase,
    ZuirBlock,
};

/// Canonical successor-edge representation.
pub use crate::zuir::ir::successor::Successor;

/// ZCIR-facing name for the canonical ZUIR basic block.
///
/// This is a type alias, not a second block implementation.
pub type ZcirBlock = ZuirBlock;

/// ZCIR-facing name for the canonical block argument.
pub type ZcirBlockArgument = BlockArgument;

/// ZCIR-facing name for the canonical block terminator.
pub type ZcirBlockTerminator = BlockTerminator;

/// ZCIR-facing name for the canonical switch case.
pub type ZcirSwitchCase = SwitchCase;

/// ZCIR-facing name for the canonical successor edge.
pub type ZcirSuccessor = Successor;

/// ZCIR-facing name for canonical block-local errors.
pub type ZcirBlockError = BlockError;

/// ZCIR-facing name for canonical block operation results.
pub type ZcirBlockResult<T> = BlockResult<T>;

#[cfg(test)]
mod tests {
    use super::*;

    use crate::zuir::ids::{
        BlockId,
        OperationId,
        RegionId,
        TypeId,
        ValueId,
    };
    use crate::zuir::ir::block::ZuirBlock;

    fn block_id(raw: u128) -> BlockId {
        BlockId::new(raw).expect("test block ID must be nonzero")
    }

    fn region_id(raw: u128) -> RegionId {
        RegionId::new(raw).expect("test region ID must be nonzero")
    }

    fn operation_id(raw: u128) -> OperationId {
        OperationId::new(raw).expect("test operation ID must be nonzero")
    }

    fn type_id(raw: u128) -> TypeId {
        TypeId::new(raw).expect("test type ID must be nonzero")
    }

    fn value_id(raw: u128) -> ValueId {
        ValueId::new(raw).expect("test value ID must be nonzero")
    }

    #[test]
    fn zcir_block_is_the_canonical_zuir_block_type() {
        fn accepts_zcir(_: ZcirBlock) {}
        fn accepts_zuir(_: ZuirBlock) {}

        let block = ZcirBlock::new(block_id(1));

        accepts_zcir(block.clone());
        accepts_zuir(block.clone());

        assert_eq!(
            core::any::type_name::<ZcirBlock>(),
            core::any::type_name::<ZuirBlock>(),
        );

        assert_eq!(block.id(), block_id(1));
    }

    #[test]
    fn region_association_uses_the_canonical_region_id() {
        let block = ZcirBlock::in_region(block_id(1), region_id(2));

        assert_eq!(block.parent_region(), Some(region_id(2)));
        assert!(block.validate().is_ok());
    }

    #[test]
    fn block_arguments_preserve_canonical_value_and_type_ids() {
        let mut block = ZcirBlock::new(block_id(1));

        let argument =
            ZcirBlockArgument::new(value_id(3), type_id(4));

        block.add_argument(argument).expect("argument is locally valid");

        assert_eq!(block.argument_count(), 1);
        assert_eq!(block.arguments()[0].value(), value_id(3));
        assert_eq!(block.arguments()[0].ty(), type_id(4));
        assert!(block.validate().is_ok());
    }

    #[test]
    fn duplicate_arguments_are_rejected_by_the_canonical_implementation() {
        let mut block = ZcirBlock::new(block_id(1));

        block
            .add_argument(ZcirBlockArgument::new(
                value_id(3),
                type_id(4),
            ))
            .expect("first argument should be accepted");

        let result = block.add_argument(ZcirBlockArgument::new(
            value_id(3),
            type_id(5),
        ));

        assert_eq!(
            result,
            Err(ZcirBlockError::DuplicateArgument(value_id(3))),
        );

        // A failed mutation must leave the original block unchanged.
        assert_eq!(block.argument_count(), 1);
        assert_eq!(block.arguments()[0].ty(), type_id(4));
    }

    #[test]
    fn operation_order_is_preserved() {
        let mut block = ZcirBlock::new(block_id(1));

        block
            .append_operation(operation_id(10))
            .expect("first operation should be accepted");

        block
            .append_operation(operation_id(20))
            .expect("second operation should be accepted");

        assert_eq!(
            block.operations(),
            &[operation_id(10), operation_id(20)],
        );
    }

    #[test]
    fn duplicate_operation_references_are_rejected() {
        let mut block = ZcirBlock::new(block_id(1));

        block
            .append_operation(operation_id(10))
            .expect("first operation should be accepted");

        assert_eq!(
            block.append_operation(operation_id(10)),
            Err(ZcirBlockError::DuplicateOperation(operation_id(10))),
        );

        assert_eq!(block.operation_count(), 1);
    }

    #[test]
    fn closed_block_rejects_appending_more_operations() {
        let mut block = ZcirBlock::in_region(block_id(1), region_id(2));

        block
            .set_terminator(ZcirBlockTerminator::Unreachable)
            .expect("unreachable is a valid closed terminator");

        assert_eq!(
            block.append_operation(operation_id(10)),
            Err(ZcirBlockError::BlockAlreadyTerminated),
        );

        assert!(block.validate_complete().is_ok());
    }

    #[test]
    fn complete_block_requires_region_and_closed_terminator() {
        let mut detached = ZcirBlock::new(block_id(1));

        assert_eq!(
            detached.validate_complete(),
            Err(ZcirBlockError::MissingParentRegion),
        );

        let mut attached =
            ZcirBlock::in_region(block_id(2), region_id(3));

        assert!(attached.validate_complete().is_err());

        attached
            .set_terminator(ZcirBlockTerminator::Unreachable)
            .expect("valid closed terminator");

        assert!(attached.validate_complete().is_ok());
    }

    #[test]
    fn zcir_successor_is_the_canonical_successor_type() {
        fn accepts_zcir(_: ZcirSuccessor) {}
        fn accepts_zuir(_: Successor) {}

        let edge = ZcirSuccessor::new(block_id(2));

        accepts_zcir(edge.clone());
        accepts_zuir(edge.clone());

        assert_eq!(
            core::any::type_name::<ZcirSuccessor>(),
            core::any::type_name::<Successor>(),
        );

        assert_eq!(edge.target(), block_id(2));
    }

    #[test]
    fn canonical_validation_is_exposed_without_duplicating_it() {
        let mut block = ZcirBlock::in_region(block_id(1), region_id(2));

        block
            .set_terminator(ZcirBlockTerminator::Unreachable)
            .expect("valid closed terminator");

        assert!(block.validate().is_ok());
        assert!(block.validate_complete().is_ok());
    }
}
