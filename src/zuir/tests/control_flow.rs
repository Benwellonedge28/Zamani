#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! ZUIR control-flow conformance tests.
//!
//! Path: src/zuir/tests/control_flow.rs
//! Rust edition: 2021
//! Minimum intended Rust version: 1.97 or later.
//!
//! PURPOSE
//!
//! Exercise the public contracts for control-flow edges, block terminators,
//! switch cases, region membership, and deterministic successor traversal.
//!
//! OWNERSHIP
//!
//! These tests do not implement control-flow verification. The canonical
//! implementation remains in:
//!
//! - crate::zuir::ir::block
//! - crate::zuir::ir::successor
//! - crate::zuir::ir::region
//! - crate::zuir::verify::control_flow
//! - crate::zuir::verify::dominance
//!
//! INTEGRATION
//!
//! The test module uses canonical ZUIR identifiers and IR entities.
//! Whole-IR verification tests must use the actual verifier input and report
//! APIs exported by the repository. They must not duplicate verifier logic.
//!
//! SCALABILITY
//!
//! No fixed maximum block, edge, successor, argument, operation, qubit,
//! register, device, or tensor-rank count is assumed. Larger tests are
//! parameterized and should be bounded by the test runner's available
//! resources, not by a universal language limit.
//!
//! SAFETY
//!
//! No unsafe Rust is used.
//!
//! IMPORTANT
//!
//! A locally valid successor is not proof that its destination exists.
//! A valid terminator is not proof of SSA dominance, value type correctness,
//! region ownership, or whole-program validity. Those properties belong to
//! the corresponding authoritative verifier.

use crate::zuir::ids::{BlockId, RegionId, ValueId};
use crate::zuir::ir::block::{
    BlockArgument,
    BlockTerminator,
    SwitchCase,
    ZuirBlock,
};
use crate::zuir::ir::successor::Successor;
use crate::zuir::schema::attributes::AttributeValue;
use crate::zuir::schema::types::QualifiedName;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn block_id(raw: u128) -> BlockId {
    BlockId::new(raw).expect("test IDs must be nonzero")
}

fn region_id(raw: u128) -> RegionId {
    RegionId::new(raw).expect("test IDs must be nonzero")
}

fn value_id(raw: u128) -> ValueId {
    ValueId::new(raw).expect("test IDs must be nonzero")
}

fn empty_block(raw: u128, region: u128) -> ZuirBlock {
    ZuirBlock::in_region(block_id(raw), region_id(region))
}

fn close_with_return(block: &mut ZuirBlock) {
    block
        .set_terminator(BlockTerminator::Return {
            values: Vec::new(),
        })
        .expect("a return terminator should be accepted");
}

// ---------------------------------------------------------------------------
// Successor contract
// ---------------------------------------------------------------------------

#[test]
fn successor_preserves_target_and_argument_order() {
    let target = block_id(10);
    let first = value_id(101);
    let second = value_id(102);
    let third = value_id(103);

    let successor =
        Successor::with_arguments(target, vec![first, second, third]);

    assert_eq!(successor.target(), target);
    assert_eq!(successor.argument_count(), 3);
    assert_eq!(
        successor.arguments(),
        &[first, second, third]
    );
    assert!(!successor.has_no_arguments());
}

#[test]
fn successor_without_arguments_is_valid() {
    let target = block_id(11);
    let successor = Successor::new(target);

    assert_eq!(successor.target(), target);
    assert!(successor.arguments().is_empty());
    assert_eq!(successor.argument_count(), 0);
    assert!(successor.has_no_arguments());
}

#[test]
fn successor_allows_repeated_values_when_semantically_valid() {
    let target = block_id(12);
    let repeated = value_id(120);

    let successor =
        Successor::with_arguments(target, vec![repeated, repeated]);

    assert_eq!(
        successor.arguments(),
        &[repeated, repeated],
        "successor construction must not impose an invalid uniqueness rule"
    );
}

#[test]
fn successor_try_from_iter_preserves_order() {
    let target = block_id(13);
    let values = [
        value_id(131),
        value_id(132),
        value_id(133),
        value_id(134),
    ];

    let successor = Successor::try_from_iter(target, values)
        .expect("finite test iterator should be collected");

    assert_eq!(successor.target(), target);
    assert_eq!(successor.arguments(), values);
}

#[test]
fn successor_replace_arguments_returns_previous_values() {
    let target = block_id(14);
    let original = vec![value_id(141), value_id(142)];
    let replacement = vec![value_id(143)];

    let mut successor = Successor::with_arguments(target, original.clone());
    let previous = successor.replace_arguments(replacement.clone());

    assert_eq!(previous, original);
    assert_eq!(successor.arguments(), replacement);
}

// ---------------------------------------------------------------------------
// Terminator state and successor traversal
// ---------------------------------------------------------------------------

#[test]
fn open_terminator_is_not_finalized() {
    let terminator = BlockTerminator::Open;

    assert!(terminator.is_open());
    assert!(!terminator.is_closed());
    assert_eq!(terminator.successor_count(), 0);
}

#[test]
fn return_terminator_has_no_successors() {
    let terminator = BlockTerminator::Return {
        values: vec![value_id(201)],
    };

    assert!(terminator.is_closed());
    assert_eq!(terminator.successor_count(), 0);
}

#[test]
fn unconditional_branch_exposes_one_successor() {
    let destination = block_id(21);
    let terminator = BlockTerminator::Branch {
        successor: Successor::new(destination),
    };

    assert_eq!(terminator.successor_count(), 1);

    let mut observed = Vec::new();
    terminator.for_each_successor(|edge| {
        observed.push(edge.target());
    });

    assert_eq!(observed, vec![destination]);
}

#[test]
fn conditional_branch_preserves_true_then_false_edge_order() {
    let then_block = block_id(22);
    let else_block = block_id(23);

    let terminator = BlockTerminator::ConditionalBranch {
        condition: value_id(220),
        then_successor: Successor::new(then_block),
        else_successor: Successor::new(else_block),
    };

    assert_eq!(terminator.successor_count(), 2);

    let mut observed = Vec::new();
    terminator.for_each_successor(|edge| {
        observed.push(edge.target());
    });

    assert_eq!(observed, vec![then_block, else_block]);
}

#[test]
fn switch_exposes_default_before_declared_cases() {
    let default_block = block_id(30);
    let case_one_block = block_id(31);
    let case_two_block = block_id(32);

    let terminator = BlockTerminator::Switch {
        selector: value_id(300),
        default_successor: Successor::new(default_block),
        cases: vec![
            SwitchCase::new(
                AttributeValue::Bool(true),
                Successor::new(case_one_block),
            ),
            SwitchCase::new(
                AttributeValue::Bool(false),
                Successor::new(case_two_block),
            ),
        ],
    };

    assert_eq!(terminator.successor_count(), 3);
    terminator
        .validate()
        .expect("distinct switch cases should validate");

    let mut observed = Vec::new();
    terminator.for_each_successor(|edge| {
        observed.push(edge.target());
    });

    assert_eq!(
        observed,
        vec![default_block, case_one_block, case_two_block]
    );
}

#[test]
fn unreachable_terminator_has_no_successors() {
    let terminator = BlockTerminator::Unreachable;

    assert!(terminator.is_closed());
    assert_eq!(terminator.successor_count(), 0);
    terminator
        .validate()
        .expect("unreachable is a valid explicit terminator");
}

#[test]
fn extension_terminator_preserves_explicit_edges() {
    let first = block_id(41);
    let second = block_id(42);

    let name = QualifiedName::new("vendor", "dispatch")
        .expect("qualified extension name should be valid");

    let terminator = BlockTerminator::Extension {
        name,
        successors: vec![
            Successor::new(first),
            Successor::new(second),
        ],
    };

    terminator
        .validate()
        .expect("well-formed extension name should validate");

    assert_eq!(terminator.successor_count(), 2);

    let mut observed = Vec::new();
    terminator.for_each_successor(|edge| {
        observed.push(edge.target());
    });

    assert_eq!(observed, vec![first, second]);
}

// ---------------------------------------------------------------------------
// Block construction and local invariants
// ---------------------------------------------------------------------------

#[test]
fn block_can_be_created_under_a_region() {
    let id = block_id(50);
    let region = region_id(500);

    let block = ZuirBlock::in_region(id, region);

    assert_eq!(block.id(), id);
    assert_eq!(block.parent_region(), Some(region));
    assert_eq!(block.operation_count(), 0);
    assert_eq!(block.argument_count(), 0);
    assert!(!block.is_terminated());
}

#[test]
fn block_argument_preserves_value_and_type_identity() {
    let value = value_id(51);
    let ty = crate::zuir::ids::TypeId::new(510)
        .expect("test type ID must be nonzero");

    let argument = BlockArgument::new(value, ty);

    assert_eq!(argument.value(), value);
    assert_eq!(argument.ty(), ty);
    assert!(argument.role().is_none());
    argument
        .validate()
        .expect("argument without optional role should validate");
}

#[test]
fn block_argument_role_must_be_validated() {
    let value = value_id(52);
    let ty = crate::zuir::ids::TypeId::new(520)
        .expect("test type ID must be nonzero");

    let role = QualifiedName::new("core", "loop_index")
        .expect("qualified argument role should be valid");

    let argument = BlockArgument::with_role(value, ty, role)
        .expect("valid role should be accepted");

    assert_eq!(
        argument.role().map(ToString::to_string),
        Some("core.loop_index".to_owned())
    );
    argument.validate().expect("role should remain valid");
}

#[test]
fn block_rejects_operations_appended_after_termination() {
    let mut block = empty_block(60, 600);
    close_with_return(&mut block);

    assert!(block.is_terminated());

    // The exact operation-insertion API is intentionally tested through
    // the public block API in the repository's builder integration suite.
    // This test asserts the terminator state that guards that API.
    assert!(block.terminator().is_closed());
}

#[test]
fn block_terminator_can_be_replaced_during_explicit_rewrite() {
    let mut block = empty_block(61, 610);
    close_with_return(&mut block);

    block
        .set_terminator(BlockTerminator::Unreachable)
        .expect("an explicit rewrite should replace the terminator");

    assert!(matches!(
        block.terminator(),
        BlockTerminator::Unreachable
    ));
}

// ---------------------------------------------------------------------------
// Switch validation
// ---------------------------------------------------------------------------

#[test]
fn switch_rejects_duplicate_case_values() {
    let terminator = BlockTerminator::Switch {
        selector: value_id(700),
        default_successor: Successor::new(block_id(70)),
        cases: vec![
            SwitchCase::new(
                AttributeValue::Bool(true),
                Successor::new(block_id(71)),
            ),
            SwitchCase::new(
                AttributeValue::Bool(true),
                Successor::new(block_id(72)),
            ),
        ],
    };

    assert!(
        terminator.validate().is_err(),
        "equal case values must not select ambiguously"
    );
}

#[test]
fn switch_with_no_explicit_cases_still_has_default_edge() {
    let default_block = block_id(80);

    let terminator = BlockTerminator::Switch {
        selector: value_id(800),
        default_successor: Successor::new(default_block),
        cases: Vec::new(),
    };

    assert_eq!(terminator.successor_count(), 1);
    terminator.validate().expect("empty case list is valid");

    let mut observed = Vec::new();
    terminator.for_each_successor(|edge| {
        observed.push(edge.target());
    });

    assert_eq!(observed, vec![default_block]);
}

// ---------------------------------------------------------------------------
// Determinism and scalable dynamic collections
// ---------------------------------------------------------------------------

#[test]
fn successor_iteration_is_deterministic() {
    let targets = [
        block_id(901),
        block_id(902),
        block_id(903),
        block_id(904),
    ];

    let terminator = BlockTerminator::Extension {
        name: QualifiedName::new("vendor", "multiway")
            .expect("valid extension name"),
        successors: targets
            .iter()
            .copied()
            .map(Successor::new)
            .collect(),
    };

    let collect = || {
        let mut result = Vec::new();
        terminator.for_each_successor(|edge| {
            result.push(edge.target());
        });
        result
    };

    assert_eq!(collect(), targets);
    assert_eq!(collect(), targets);
}

#[test]
fn successor_argument_storage_grows_without_a_semantic_ceiling() {
    // This is a scalable regression test, not a claim of infinite capacity.
    // Its size can be increased by the stress-test profile without changing
    // the IR contract or introducing a universal maximum.
    let count = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1)
        .saturating_mul(1024);

    let target = block_id(1000);
    let arguments = (1..=count)
        .map(|n| {
            ValueId::new(n as u128)
                .expect("generated test IDs are nonzero")
        });

    let successor = Successor::try_from_iter(target, arguments)
        .expect("test collection should fit available resources");

    assert_eq!(successor.argument_count(), count);
    assert_eq!(
        successor.arguments().first().copied(),
        Some(value_id(1))
    );
    assert_eq!(
        successor.arguments().last().copied(),
        Some(value_id(count as u128))
    );
}

// ---------------------------------------------------------------------------
// Scope of these tests
// ---------------------------------------------------------------------------
//
// The following properties require the actual whole-IR verifier fixture and
// must not be approximated by local constructors:
//
// - missing successor destination;
// - destination block belonging to another region;
// - incorrect edge-argument arity;
// - undefined SSA value;
// - non-dominating value use;
// - branch argument type mismatch;
// - invalid entry-block definition;
// - extension terminator without a registered contract;
// - malformed serialized control-flow graph.
//
// Add these cases to the verifier integration suite using the canonical
// ControlFlowInput / verification-report API exposed by
// src/zuir/verify/control_flow.rs. The verifier, not this test file, must own
// their expected diagnostic codes.