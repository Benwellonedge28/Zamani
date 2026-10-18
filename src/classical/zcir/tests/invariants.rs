
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # ZCIR Invariant Tests
//!
//! Canonical path: `src/classical/zcir/tests/invariants.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97 or later.
//! Safety: unsafe Rust is forbidden.
//!
//! ## Purpose
//!
//! Test invariants at the classical ZCIR integration boundary while retaining
//! canonical ownership of IR entities and verification algorithms in ZUIR.
//!
//! ## Authority
//!
//! - `src/classical/zcir/ir/mod.rs`
//! - `src/classical/zcir/verify/mod.rs`
//! - `src/zuir/ids.rs`
//! - `src/zuir/ir/block.rs`
//! - `src/zuir/ir/region.rs`
//! - `src/zuir/verify/control_flow.rs`
//! - `src/zuir/verify/structural.rs`
//! - `grammar/Zamani.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/poco-reaf.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/validation/semantic-boundaries.md`
//!
//! ## Ownership
//!
//! This file owns tests only. It introduces no IR nodes, identities,
//! verification algorithms, source-language semantics, or hardware limits.
//!
//! ## Scope
//!
//! Tests cover:
//!
//! - Canonical type and identifier reuse.
//! - Local block invariants.
//! - Duplicate-reference rejection.
//! - Failure atomicity for rejected mutations.
//! - Region membership invariants.
//! - Duplicate identity detection.
//! - Missing-reference detection.
//! - Separation between local and cross-entity verification.
//!
//! These tests do not claim to establish complete SSA, type, effect,
//! ownership, dialect, resource, or target-feasibility correctness.
//! Those properties require their respective canonical verification stages.
//!
//! ## Integration contract
//!
//! The ZCIR root must register `ir` and `verify` before these tests can
//! compile. The verification module must expose the canonical ZUIR
//! control-flow verifier and its input/report types.
//!
//! ## Scalability contract
//!
//! Test inputs are finite regression workloads, not semantic limits.
//! No test introduces a universal maximum for operations, blocks, regions,
//! values, types, memory, devices, or hardware scale.

use crate::classical::zcir::{ir, verify};
use crate::zuir;

/// Construct a canonical nonzero block identity for a test.
fn block_id(value: u128) -> ir::ids::BlockId {
    ir::ids::BlockId::new(value)
        .expect("test block identity must be nonzero")
}

/// Construct a canonical nonzero region identity for a test.
fn region_id(value: u128) -> ir::ids::RegionId {
    ir::ids::RegionId::new(value)
        .expect("test region identity must be nonzero")
}

/// Construct a canonical nonzero function identity for a test.
fn function_id(value: u128) -> ir::ids::FunctionId {
    ir::ids::FunctionId::new(value)
        .expect("test function identity must be nonzero")
}

fn control_flow_region(
    id: ir::ids::RegionId,
    owner: ir::ids::FunctionId,
) -> ir::region::ZuirRegion {
    ir::region::ZuirRegion::new(
        id,
        ir::region::RegionOwner::Function(owner),
        ir::region::RegionKind::ControlFlow,
        ir::region::RegionIsolation::Isolated,
    )
}

#[test]
fn zcir_reuses_canonical_zuir_identity_types() {
    let _: Option<zuir::ids::BlockId> = None;
    let zcir_block: Option<ir::ids::BlockId> = None;

    let _: Option<zuir::ids::RegionId> = zcir_block;
    let zcir_region: Option<ir::ids::RegionId> = None;

    let _: Option<zuir::ids::FunctionId> = zcir_region;

    let zcir_function: Option<ir::ids::FunctionId> = None;
    let _: Option<zuir::ids::FunctionId> = zcir_function;

    let zcir_operation: Option<ir::ids::OperationId> = None;
    let _: Option<zuir::ids::OperationId> = zcir_operation;

    let zcir_value: Option<ir::ids::ValueId> = None;
    let _: Option<zuir::ids::ValueId> = zcir_value;
}

#[test]
fn zero_is_not_a_valid_canonical_identity() {
    assert!(ir::ids::BlockId::new(0).is_err());
    assert!(ir::ids::RegionId::new(0).is_err());
    assert!(ir::ids::FunctionId::new(0).is_err());
    assert!(ir::ids::OperationId::new(0).is_err());
    assert!(ir::ids::ValueId::new(0).is_err());
}

#[test]
fn locally_constructed_block_preserves_its_identity() {
    let id = block_id(1);
    let block = ir::block::ZuirBlock::new(id);

    assert_eq!(block.id(), id);
    assert_eq!(block.operation_count(), 0);
    assert_eq!(block.argument_count(), 0);
    assert!(block.operations().is_empty());
    assert!(!block.is_terminated());

    block
        .validate()
        .expect("a newly constructed block must satisfy local invariants");
}

#[test]
fn canonical_conversion_preserves_the_complete_block_identity() {
    let id = block_id(2);
    let block = ir::block::ZuirBlock::new(id);

    let converted = crate::classical::zcir::lowering::to_zuir::to_zuir(block);

    assert_eq!(converted.id(), id);
    converted
        .validate()
        .expect("identity conversion must preserve local validity");
}

#[test]
fn borrowed_conversion_does_not_change_the_original_block() {
    let id = block_id(3);
    let block = ir::block::ZuirBlock::new(id);

    let borrowed =
        crate::classical::zcir::lowering::to_zuir::as_zuir(&block);

    assert_eq!(borrowed.id(), id);
    assert_eq!(block.id(), id);
}

#[test]
fn mutable_borrow_does_not_change_identity_by_itself() {
    let id = block_id(4);
    let mut block = ir::block::ZuirBlock::new(id);

    {
        let borrowed =
            crate::classical::zcir::lowering::to_zuir::as_zuir_mut(&mut block);
        assert_eq!(borrowed.id(), id);
    }

    assert_eq!(block.id(), id);
    block
        .validate()
        .expect("mutable access without mutation must preserve validity");
}

#[test]
fn duplicate_operation_reference_is_rejected() {
    let mut block = ir::block::ZuirBlock::new(block_id(5));
    let operation = ir::ids::OperationId::new(101)
        .expect("test operation identity must be nonzero");

    block
        .insert_operation(0, operation)
        .expect("first insertion must succeed");

    let result = block.insert_operation(1, operation);

    assert!(
        result.is_err(),
        "a block must reject a duplicate operation reference"
    );

    assert_eq!(block.operation_count(), 1);
    assert_eq!(block.operations(), &[operation]);
}

#[test]
fn rejected_duplicate_operation_insertion_is_atomic() {
    let mut block = ir::block::ZuirBlock::new(block_id(6));

    let first = ir::ids::OperationId::new(201)
        .expect("test operation identity must be nonzero");
    let second = ir::ids::OperationId::new(202)
        .expect("test operation identity must be nonzero");

    block
        .insert_operation(0, first)
        .expect("first insertion must succeed");
    block
        .insert_operation(1, second)
        .expect("second unique insertion must succeed");

    let before = block.operations().to_vec();

    assert!(block.insert_operation(1, first).is_err());
    assert_eq!(block.operations(), before.as_slice());
    assert_eq!(block.operation_count(), before.len());
}

#[test]
fn operation_insertion_rejects_positions_beyond_the_end() {
    let mut block = ir::block::ZuirBlock::new(block_id(7));
    let operation = ir::ids::OperationId::new(301)
        .expect("test operation identity must be nonzero");

    assert!(block.insert_operation(1, operation).is_err());
    assert!(block.operations().is_empty());
    assert_eq!(block.operation_count(), 0);
}

#[test]
fn operation_insertion_at_the_end_is_valid() {
    let mut block = ir::block::ZuirBlock::new(block_id(8));

    let first = ir::ids::OperationId::new(401)
        .expect("test operation identity must be nonzero");
    let second = ir::ids::OperationId::new(402)
        .expect("test operation identity must be nonzero");

    block
        .insert_operation(0, first)
        .expect("first insertion must succeed");
    block
        .insert_operation(block.operation_count(), second)
        .expect("insertion at the end must succeed");

    assert_eq!(block.operations(), &[first, second]);
}

#[test]
fn region_rejects_duplicate_block_membership() {
    let owner = function_id(10);
    let id = region_id(10);
    let member = block_id(10);

    let mut region = control_flow_region(id, owner);

    region
        .add_block(member)
        .expect("first block membership must succeed");

    let result = region.add_block(member);

    assert!(
        result.is_err(),
        "a region must reject duplicate block identities"
    );

    assert_eq!(region.block_count(), 1);
    assert_eq!(region.blocks(), &[member]);
}

#[test]
fn rejected_duplicate_region_membership_is_atomic() {
    let owner = function_id(11);
    let mut region = control_flow_region(region_id(11), owner);

    let first = block_id(11);
    let second = block_id(12);

    region
        .add_block(first)
        .expect("first membership must succeed");
    region
        .add_block(second)
        .expect("second unique membership must succeed");

    let before = region.blocks().to_vec();

    assert!(region.add_block(first).is_err());
    assert_eq!(region.blocks(), before.as_slice());
    assert_eq!(region.block_count(), before.len());
}

#[test]
fn duplicate_block_identity_is_detected_by_control_flow_verification() {
    let id = block_id(20);

    // These are two entities in one verification scope claiming the same
    // canonical identity. They must not be treated as distinct blocks.
    let blocks = [
        ir::block::ZuirBlock::new(id),
        ir::block::ZuirBlock::new(id),
    ];
    let regions: [ir::region::ZuirRegion; 0] = [];

    let report = verify::verify_control_flow(&verify::ControlFlowInput {
        regions: &regions,
        blocks: &blocks,
    });

    assert!(
        !report.is_valid(),
        "duplicate block identities must invalidate control-flow verification"
    );
    assert!(
        report.issue_count() > 0,
        "the verifier must explain the duplicate identity"
    );
}

#[test]
fn missing_region_block_reference_is_detected() {
    let owner = function_id(21);
    let region = region_id(21);
    let missing_block = block_id(21);

    let mut region_entity = control_flow_region(region, owner);
    region_entity
        .add_block(missing_block)
        .expect("region should accept a unique reference locally");

    // The region is locally well-formed with respect to duplicate
    // membership, but the referenced block is absent from the supplied
    // verification scope. Cross-entity verification must reject this.
    let regions = [region_entity];
    let blocks: [ir::block::ZuirBlock; 0] = [];

    let report = verify::verify_control_flow(&verify::ControlFlowInput {
        regions: &regions,
        blocks: &blocks,
    });

    assert!(
        !report.is_valid(),
        "a missing referenced block must be reported"
    );
    assert!(
        report.issue_count() > 0,
        "the report must retain at least one actionable finding"
    );
}

#[test]
fn local_block_validation_does_not_claim_cross_entity_validity() {
    let mut block = ir::block::ZuirBlock::new(block_id(22));
    let missing_operation = ir::ids::OperationId::new(501)
        .expect("test operation identity must be nonzero");

    block
        .insert_operation(0, missing_operation)
        .expect("the block can store a unique operation reference");

    // The block owns the ordered references, not the operation payloads.
    // Its local validator cannot establish that the referenced operation
    // exists in the surrounding IR.
    block
        .validate()
        .expect("local invariants do not resolve cross-entity references");

    assert_eq!(block.operations(), &[missing_operation]);
}

#[test]
fn distinct_identity_domains_remain_distinct_types() {
    fn accepts_block_id(_: ir::ids::BlockId) {}
    fn accepts_operation_id(_: ir::ids::OperationId) {}
    fn accepts_region_id(_: ir::ids::RegionId) {}

    accepts_block_id(block_id(30));
    accepts_operation_id(
        ir::ids::OperationId::new(30)
            .expect("test operation identity must be nonzero"),
    );
    accepts_region_id(region_id(30));
}

#[test]
fn dynamic_operation_storage_grows_for_a_bounded_test_workload() {
    // A finite test budget keeps routine test runs practical. It is not a
    // maximum operation count or a limit on Zamani programs.
    const TEST_OPERATION_COUNT: usize = 1_024;

    let mut block = ir::block::ZuirBlock::new(block_id(40));

    block
        .try_reserve_operations(TEST_OPERATION_COUNT)
        .expect("the bounded test workload should be reservable");

    for index in 0..TEST_OPERATION_COUNT {
        let raw = u128::try_from(index)
            .expect("usize must fit the test identity representation")
            + 1;

        let operation = ir::ids::OperationId::new(raw)
            .expect("generated test identity must be nonzero");

        block
            .insert_operation(block.operation_count(), operation)
            .expect("unique operation reference must be accepted");
    }

    assert_eq!(block.operation_count(), TEST_OPERATION_COUNT);

    let mut seen = std::collections::BTreeSet::new();

    for operation in block.operations() {
        assert!(
            seen.insert(*operation),
            "the constructed block must contain unique operation identities"
        );
    }

    assert_eq!(seen.len(), TEST_OPERATION_COUNT);

    block
        .validate()
        .expect("the completed block must preserve local invariants");
}

#[test]
fn empty_control_flow_scope_is_handled_without_panicking() {
    let regions: [ir::region::ZuirRegion; 0] = [];
    let blocks: [ir::block::ZuirBlock; 0] = [];

    let report = verify::verify_control_flow(&verify::ControlFlowInput {
        regions: &regions,
        blocks: &blocks,
    });

    // An empty scope may be structurally vacuous at this verification
    // stage. Whether an empty program is legal is a separate program-level
    // contract; this test only requires safe, deterministic processing.
    assert_eq!(report.issue_count(), 0);
    assert!(report.is_valid());
}
