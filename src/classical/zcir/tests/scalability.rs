
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # ZCIR Scalability Tests
//!
//! Canonical path: `src/classical/zcir/tests/scalability.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety policy: unsafe Rust is forbidden.
//!
//! ## Purpose
//!
//! Exercise the canonical ZUIR-backed classical IR under increasing,
//! explicitly bounded test workloads. These tests check that dynamically
//! stored operation references and control-flow graph entities grow without
//! silent truncation or loss of identity.
//!
//! ## Authority
//!
//! These tests follow the contracts in:
//!
//! - `src/classical/zcir/ir/mod.rs`
//! - `src/classical/zcir/verify/mod.rs`
//! - `src/zuir/ids.rs`
//! - `src/zuir/ir/block.rs`
//! - `src/zuir/ir/region.rs`
//! - `src/zuir/verify/control_flow.rs`
//! - `grammar/Zamani.g4`
//! - `grammar/DESIGN.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/specification/poco-reaf.md`
//! - `grammar/validation/semantic-boundaries.md`
//!
//! ## Ownership
//!
//! This file owns only test fixtures, test workload sizes, and assertions.
//! It does not define IR entities, identity allocation, language semantics,
//! resource policies, or production capacity limits.
//!
//! ## Integration contract
//!
//! ZCIR reuses canonical ZUIR IDs, blocks, regions, and verification.
//! These tests deliberately construct those canonical entities directly;
//! they do not create duplicate classical representations.
//!
//! Register this file from `src/classical/zcir/tests/mod.rs` with
//! `mod scalability;`, then register the test module from the valid
//! `src/classical/zcir/mod.rs` using `#[cfg(test)] mod tests;`.
//!
//! ## Scalability contract
//!
//! The constants below are test workload budgets only. They are not language
//! limits, IR capacity limits, runtime limits, or target capability limits.
//!
//! No finite test can demonstrate literal infinity. These tests instead
//! check representative dynamic growth and integrity properties. Production
//! code must continue to rely on available resources and explicit operational
//! policies rather than arbitrary universal ceilings.
//!
//! ## Validity boundary
//!
//! Passing these tests does not establish complete semantic correctness,
//! portability to every target, or the absence of limits in other modules.
//! Whole-IR verification, semantic-preservation, interoperability, and
//! cross-target conformance require their own tests.

use crate::zuir::ids::{BlockId, FunctionId, OperationId, RegionId};
use crate::zuir::ir::block::ZuirBlock;
use crate::zuir::ir::region::{
    RegionIsolation, RegionKind, RegionOwner, ZuirRegion,
};
use crate::zuir::verify::control_flow::{
    verify_control_flow, ControlFlowInput,
};

// These are deliberately finite test budgets. They must never be reused as
// production limits or exposed as language-wide maximums.
const OPERATION_GROWTH_BUDGET: usize = 4_096;
const REGION_BLOCK_GROWTH_BUDGET: usize = 512;

/// Construct a valid nonzero canonical block identity for a test.
fn block_id(raw: u128) -> BlockId {
    BlockId::new(raw).expect("test block identity must be nonzero")
}

/// Construct a valid nonzero canonical region identity for a test.
fn region_id(raw: u128) -> RegionId {
    RegionId::new(raw).expect("test region identity must be nonzero")
}

/// Construct a valid nonzero canonical function identity for a test.
fn function_id(raw: u128) -> FunctionId {
    FunctionId::new(raw).expect("test function identity must be nonzero")
}

/// Construct a valid nonzero canonical operation identity for a test.
fn operation_id(raw: u128) -> OperationId {
    OperationId::new(raw).expect("test operation identity must be nonzero")
}

/// Convert a test index to a nonzero ID without narrowing the integer.
fn indexed_operation_id(index: usize) -> OperationId {
    let raw = u128::try_from(index)
        .expect("a usize test index must fit in u128")
        .checked_add(1)
        .expect("test operation identity must not overflow");

    operation_id(raw)
}

#[test]
fn operation_storage_grows_without_silent_truncation() {
    let mut block = ZuirBlock::new(block_id(1));

    // Intentionally do not reserve the full workload in advance. This
    // exercises the canonical collection's ordinary dynamic-growth path.
    for index in 0..OPERATION_GROWTH_BUDGET {
        block
            .append_operation(indexed_operation_id(index))
            .expect("a unique operation reference should be accepted");
    }

    assert_eq!(block.operation_count(), OPERATION_GROWTH_BUDGET);
    assert_eq!(block.operations().len(), OPERATION_GROWTH_BUDGET);

    for (index, actual) in block.operations().iter().enumerate() {
        assert_eq!(
            *actual,
            indexed_operation_id(index),
            "operation order or identity changed at index {index}"
        );
    }

    assert!(
        block.validate().is_ok(),
        "a block containing unique operation references must remain locally valid"
    );
}

#[test]
fn reserved_operation_storage_preserves_every_inserted_reference() {
    let mut block = ZuirBlock::new(block_id(2));

    // Reserve in increments rather than assuming one particular capacity
    // growth strategy or allocator implementation.
    let chunk_size = 64_usize;
    let mut inserted = 0_usize;

    while inserted < OPERATION_GROWTH_BUDGET {
        let remaining = OPERATION_GROWTH_BUDGET - inserted;
        let additional = remaining.min(chunk_size);

        block
            .try_reserve_operations(additional)
            .expect("the bounded test reservation should be feasible");

        let end = inserted
            .checked_add(additional)
            .expect("bounded test index must not overflow");

        while inserted < end {
            block
                .append_operation(indexed_operation_id(inserted))
                .expect("each operation identity must be unique");

            inserted = inserted
                .checked_add(1)
                .expect("bounded test index must not overflow");
        }
    }

    assert_eq!(inserted, OPERATION_GROWTH_BUDGET);
    assert_eq!(block.operation_count(), OPERATION_GROWTH_BUDGET);
    assert_eq!(block.operations().len(), OPERATION_GROWTH_BUDGET);

    assert!(
        block.validate().is_ok(),
        "reserved growth must preserve block-local invariants"
    );
}

#[test]
fn impossible_reservation_is_reported_without_losing_existing_operations() {
    let mut block = ZuirBlock::new(block_id(3));

    for index in 0..16 {
        block
            .append_operation(indexed_operation_id(index))
            .expect("fixture operation should be accepted");
    }

    let before = block.operations().to_vec();
    let count_before = block.operation_count();

    // Vec::try_reserve reports capacity overflow for this request instead
    // of attempting to create a collection of this size. This is a
    // robustness test, not a claim that real memory exhaustion is always
    // recoverable throughout the whole process.
    let result = block.try_reserve_operations(usize::MAX);

    assert!(
        result.is_err(),
        "an unrepresentable reservation must return an error"
    );
    assert_eq!(
        block.operation_count(),
        count_before,
        "a failed reservation must not change the operation count"
    );
    assert_eq!(
        block.operations(),
        before.as_slice(),
        "a failed reservation must not discard or reorder existing references"
    );
    assert!(
        block.validate().is_ok(),
        "the block must remain locally valid after reservation failure"
    );
}

#[test]
fn region_block_membership_grows_and_preserves_insertion_order() {
    let region_identity = region_id(1);
    let owner = function_id(1);

    let mut region = ZuirRegion::new(
        region_identity,
        RegionOwner::Function(owner),
        RegionKind::ControlFlow,
        RegionIsolation::Isolated,
    );

    // Keep the corresponding blocks alive so the test can verify both the
    // region's ordered membership and the block's declared parent region.
    let mut blocks = Vec::with_capacity(REGION_BLOCK_GROWTH_BUDGET);

    for index in 0..REGION_BLOCK_GROWTH_BUDGET {
        let raw = u128::try_from(index)
            .expect("a usize test index must fit in u128")
            .checked_add(1)
            .expect("test block identity must not overflow");

        let identity = block_id(raw);
        let block = ZuirBlock::in_region(identity, region_identity);

        region
            .add_block(identity)
            .expect("a unique block identity should be accepted");

        blocks.push(block);
    }

    assert_eq!(region.block_count(), REGION_BLOCK_GROWTH_BUDGET);
    assert_eq!(region.blocks().len(), REGION_BLOCK_GROWTH_BUDGET);
    assert_eq!(blocks.len(), REGION_BLOCK_GROWTH_BUDGET);

    for (index, identity) in region.blocks().iter().enumerate() {
        let raw = u128::try_from(index)
            .expect("a usize test index must fit in u128")
            .checked_add(1)
            .expect("test block identity must not overflow");

        assert_eq!(
            *identity,
            block_id(raw),
            "region membership order changed at index {index}"
        );

        assert_eq!(
            blocks[index].id(),
            *identity,
            "the block collection and region membership disagree"
        );

        assert_eq!(
            blocks[index].parent_region(),
            Some(region_identity),
            "the block's parent region was not preserved"
        );
    }

    assert!(
        region.validate().is_ok(),
        "a nonempty region with unique block references must remain locally valid"
    );

    for block in &blocks {
        assert!(
            block.validate().is_ok(),
            "each block must retain its local invariants"
        );
    }
}

#[test]
fn control_flow_verifier_processes_a_growing_graph_without_losing_entities() {
    let region_identity = region_id(2);

    let mut region = ZuirRegion::new(
        region_identity,
        RegionOwner::Function(function_id(2)),
        RegionKind::ControlFlow,
        RegionIsolation::Isolated,
    );

    let mut blocks = Vec::with_capacity(REGION_BLOCK_GROWTH_BUDGET);

    for index in 0..REGION_BLOCK_GROWTH_BUDGET {
        let raw = u128::try_from(index)
            .expect("a usize test index must fit in u128")
            .checked_add(1)
            .expect("test block identity must not overflow");

        let identity = block_id(raw);

        region
            .add_block(identity)
            .expect("block membership must be unique");

        blocks.push(ZuirBlock::in_region(identity, region_identity));
    }

    let regions = [region];

    let report = verify_control_flow(&ControlFlowInput {
        regions: &regions,
        blocks: &blocks,
    });

    // Open blocks are construction states, not finalized executable control
    // flow. The verifier may correctly report them as invalid. This test
    // checks that the verifier processes and accounts for the complete
    // supplied graph; it does not mistake "no panic" for graph validity.
    assert_eq!(
        report.regions_checked(),
        1,
        "the verifier must account for the supplied region"
    );
    assert_eq!(
        report.blocks_checked(),
        REGION_BLOCK_GROWTH_BUDGET,
        "the verifier must account for every supplied block"
    );
}
