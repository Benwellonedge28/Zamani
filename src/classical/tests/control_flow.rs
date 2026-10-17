
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Classical Computing — Control-Flow Contract Tests
//!
//! Path: src/classical/tests/control_flow.rs
//! Edition: Rust 2021
//! Minimum supported Rust version: Rust 1.97
//! Safety: Unsafe Rust is forbidden.
//!
//! # Purpose
//!
//! Verify the public contracts of the classical control-flow planning
//! subsystem: conditional branches, loops, function control flow,
//! calls, exceptions, and termination.
//!
//! These tests exercise existing public APIs. They do not introduce a
//! second AST, semantic model, control-flow graph, canonical IR, or runtime.
//!
//! # Authority
//!
//! Source syntax:
//! - grammar/Zamani.g4
//! - grammar/classical/control.g4
//! - grammar/statements/conditionals.g4
//! - grammar/statements/loops.g4
//! - grammar/statements/control-flow.g4
//! - grammar/statements/returns.g4
//! - grammar/statements/breaks.g4
//! - grammar/statements/continues.g4
//! - grammar/statements/exceptions.g4
//! - grammar/expressions/calls.g4
//! - grammar/functions/functions.g4
//!
//! Implementation:
//! - src/classical/control_flow/branch.rs
//! - src/classical/control_flow/loops.rs
//! - src/classical/control_flow/functions.rs
//! - src/classical/control_flow/calls.rs
//! - src/classical/control_flow/exceptions.rs
//! - src/classical/control_flow/termination.rs
//! - src/classical/ids.rs
//! - src/classical/error.rs
//!
//! Canonical ownership:
//! - src/zuir/ids.rs owns general canonical identities.
//! - Canonical ZUIR owns the authoritative IR and CFG.
//! - The frontend owns parsed AST nodes and source spans.
//! - Semantic analysis owns type checking, name resolution, control-transfer
//!   legality, reachability, and effect analysis.
//! - The grammar owns source syntax, not runtime behavior.
//!
//! # Scope and limitations
//!
//! A planning object holding validly represented identifiers does not prove
//! that those entities exist in a particular IR, belong to the same
//! compilation, or form a valid CFG. Those properties must be checked by
//! semantic validation and the canonical IR verifier.
//!
//! These tests do not claim to prove end-to-end parsing, semantic analysis,
//! lowering, optimization correctness, runtime execution, or portability.
//! Those guarantees require integration tests through their owning layers.
//!
//! # Scalability
//!
//! No test below establishes a universal semantic ceiling for branch counts,
//! loop iterations, functions, return sites, call arguments, or hardware.
//! Finite test sizes are test inputs, not language limits.
//!
//! Tests remain subject to the resources available to the test process.
//! Literal infinity is not a representable or testable resource guarantee.
//!
//! # Integration
//!
//! Include this file exactly once from the existing classical module's
//! test declarations, for example:
//
//! ```ignore
//! #[cfg(test)]
//! #[path = "tests/control_flow.rs"]
//! mod control_flow_contract_tests;
//! ```
//!
//! If an equivalent declaration already exists, extend it rather than
//! declaring a duplicate. Keep this file separate from the production
//! `control_flow` module and do not add another canonical IR.
//!
//! # Completion
//!
//! Run formatting, compilation, the complete test suite, and lint checks
//! against the actual repository and supported Rust toolchain. Passing this
//! file alone is not a production-readiness certification.

use crate::classical::control_flow::branch::{
    BranchChain,
    BranchPlan,
    ConditionalArm,
};
use crate::classical::control_flow::calls::{
    CallSitePlan,
    CallTarget,
};
use crate::classical::control_flow::exceptions::{
    CatchHandlerPlan,
    ThrowPlan,
    TryExceptionPlan,
};
use crate::classical::control_flow::functions::FunctionControlFlowPlan;
use crate::classical::control_flow::loops::{
    validate_loop_plan,
    LoopPlan,
    WhileLoopPlan,
};
use crate::classical::control_flow::termination::{
    BreakPlan,
    ContinuePlan,
    ReturnPlan,
    TerminationPlan,
};
use crate::classical::error::ClassicalError;
use crate::classical::ids::{
    BlockId,
    FunctionId,
    OperationId,
    SymbolId,
    TypeId,
    ValueId,
};

// -------------------------------------------------------------------------
// Canonical test identity constructors
// -------------------------------------------------------------------------

// Test identifiers are constructed through the canonical ID API. These
// helpers do not introduce test-specific identifier types or allocators.

fn block(raw: u128) -> BlockId {
    BlockId::new(raw).expect("test block ID must be nonzero")
}

fn value(raw: u128) -> ValueId {
    ValueId::new(raw).expect("test value ID must be nonzero")
}

fn function(raw: u128) -> FunctionId {
    FunctionId::new(raw).expect("test function ID must be nonzero")
}

fn operation(raw: u128) -> OperationId {
    OperationId::new(raw).expect("test operation ID must be nonzero")
}

fn symbol(raw: u128) -> SymbolId {
    SymbolId::new(raw).expect("test symbol ID must be nonzero")
}

fn ty(raw: u128) -> TypeId {
    TypeId::new(raw).expect("test type ID must be nonzero")
}

// -------------------------------------------------------------------------
// Conditional branches
// -------------------------------------------------------------------------

#[test]
fn branch_preserves_canonical_condition_and_destinations() {
    let condition = value(1);
    let then_block = block(2);
    let else_block = block(3);

    let plan = BranchPlan::new(
        condition,
        then_block,
        Some(else_block),
    );

    assert_eq!(plan.condition(), &condition);
    assert_eq!(plan.then_target(), &then_block);
    assert_eq!(plan.else_target(), Some(&else_block));
}

#[test]
fn branch_without_else_preserves_absent_fallback() {
    let plan = BranchPlan::new(value(1), block(2), None);

    assert!(plan.else_target().is_none());

    let (condition, then_target, else_target) = plan.into_parts();

    assert_eq!(condition, value(1));
    assert_eq!(then_target, block(2));
    assert_eq!(else_target, None);
}

#[test]
fn branch_replacement_returns_previous_identity() {
    let mut plan =
        BranchPlan::new(value(1), block(2), Some(block(3)));

    assert_eq!(plan.replace_condition(value(4)), value(1));
    assert_eq!(plan.replace_then_target(block(5)), block(2));
    assert_eq!(
        plan.replace_else_target(Some(block(6))),
        Some(block(3))
    );

    assert_eq!(plan.condition(), &value(4));
    assert_eq!(plan.then_target(), &block(5));
    assert_eq!(plan.else_target(), Some(&block(6)));
}

#[test]
fn conditional_chain_preserves_source_order() {
    let arms = vec![
        ConditionalArm::new(value(1), block(10)),
        ConditionalArm::new(value(2), block(20)),
        ConditionalArm::new(value(3), block(30)),
    ];

    let chain = BranchChain::new(arms, Some(block(40)))
        .expect("a nonempty conditional chain must be valid");

    assert_eq!(chain.arm_count(), 3);
    assert_eq!(chain.arms()[0].condition(), &value(1));
    assert_eq!(chain.arms()[1].condition(), &value(2));
    assert_eq!(chain.arms()[2].condition(), &value(3));
    assert_eq!(chain.arms()[0].target(), &block(10));
    assert_eq!(chain.arms()[1].target(), &block(20));
    assert_eq!(chain.arms()[2].target(), &block(30));
    assert_eq!(chain.fallback(), Some(&block(40)));
}

#[test]
fn empty_conditional_chain_is_rejected() {
    let result = BranchChain::new(Vec::new(), None);

    assert!(matches!(
        result,
        Err(ClassicalError::InvalidInput { .. })
    ));
}

#[test]
fn conditional_chain_without_final_else_is_valid() {
    let chain = BranchChain::new(
        vec![ConditionalArm::new(value(1), block(2))],
        None,
    )
    .expect("a final else clause is optional");

    assert_eq!(chain.arm_count(), 1);
    assert!(!chain.has_fallback());
    assert!(chain.fallback().is_none());
}

#[test]
fn conditional_chain_supports_dynamically_sized_input() {
    // This is a stress-oriented sample size, not a semantic ceiling.
    let count = 2_048_u128;

    let arms = (1..=count)
        .map(|id| ConditionalArm::new(value(id), block(id + count)))
        .collect::<Vec<_>>();

    let chain = BranchChain::new(arms, Some(block(count * 2 + 1)))
        .expect("all supplied conditional arms should be preserved");

    assert_eq!(chain.arm_count(), count as usize);
    assert_eq!(chain.first_arm().condition(), &value(1));
    assert_eq!(
        chain.arms().last().expect("last arm").condition(),
        &value(count)
    );
}

// -------------------------------------------------------------------------
// Structured loops
// -------------------------------------------------------------------------

#[test]
fn while_loop_preserves_condition_and_control_blocks() {
    let condition = value(1);
    let header = block(2);
    let body = block(3);
    let exit = block(4);

    let plan = WhileLoopPlan::new(condition, header, body, exit);

    assert_eq!(plan.condition(), &condition);
    assert_eq!(plan.header(), &header);
    assert_eq!(plan.body(), &body);
    assert_eq!(plan.exit(), &exit);

    // This planning API models continuation at the header. Lowering may
    // introduce a separate latch when required by the canonical IR.
    assert_eq!(plan.continue_target(), &header);

    let (actual_condition, actual_header, actual_body, actual_exit) =
        plan.into_parts();

    assert_eq!(actual_condition, condition);
    assert_eq!(actual_header, header);
    assert_eq!(actual_body, body);
    assert_eq!(actual_exit, exit);
}

#[test]
fn loop_enum_preserves_while_loop_kind() {
    let plan = LoopPlan::while_loop(
        value(1),
        block(2),
        block(3),
        block(4),
    );

    assert!(plan.is_while());
    assert!(!plan.is_for());
    assert!(plan.as_while().is_some());
    assert!(plan.as_for().is_none());

    assert_eq!(plan.header(), &block(2));
    assert_eq!(plan.body(), &block(3));
    assert_eq!(plan.exit(), &block(4));
    assert_eq!(plan.continue_target(), &block(2));

    validate_loop_plan(&plan)
        .expect("a structurally valid while-loop plan should validate");
}

#[test]
fn loop_enum_preserves_for_loop_kind_and_binding() {
    let iterable = value(1);
    let binding = symbol(2);

    let plan = LoopPlan::for_loop(
        iterable,
        binding,
        block(3),
        block(4),
        block(5),
    );

    assert!(plan.is_for());
    assert!(!plan.is_while());
    assert!(plan.as_for().is_some());
    assert!(plan.as_while().is_none());

    let for_loop = plan.as_for().expect("for-loop variant");

    assert_eq!(for_loop.iterable(), &iterable);
    assert_eq!(for_loop.binding(), &binding);
    assert_eq!(for_loop.header(), &block(3));
    assert_eq!(for_loop.body(), &block(4));
    assert_eq!(for_loop.exit(), &block(5));
    assert_eq!(for_loop.continue_target(), &block(3));

    validate_loop_plan(&plan)
        .expect("a structurally valid for-loop plan should validate");
}

// -------------------------------------------------------------------------
// Function control flow
// -------------------------------------------------------------------------

#[test]
fn function_plan_preserves_function_and_entry_identity() {
    let function_id = function(1);
    let entry = block(2);

    let plan = FunctionControlFlowPlan::new(function_id, entry);

    assert_eq!(plan.function(), function_id);
    assert_eq!(plan.entry(), entry);
    assert_eq!(plan.return_block_count(), 0);
    assert_eq!(plan.return_blocks().count(), 0);
}

#[test]
fn function_plan_registers_and_removes_return_blocks() {
    let mut plan =
        FunctionControlFlowPlan::new(function(1), block(2));

    let first = block(3);
    let second = block(4);

    plan.add_return_block(first)
        .expect("first return block should register");
    plan.add_return_block(second)
        .expect("second return block should register");

    assert_eq!(plan.return_block_count(), 2);
    assert!(plan.contains_return_block(&first));
    assert!(plan.contains_return_block(&second));

    assert!(plan.remove_return_block(&first));
    assert!(!plan.contains_return_block(&first));
    assert_eq!(plan.return_block_count(), 1);

    assert!(!plan.remove_return_block(&first));
}

#[test]
fn duplicate_return_block_registration_is_rejected() {
    let mut plan =
        FunctionControlFlowPlan::new(function(1), block(2));
    let return_block = block(3);

    plan.add_return_block(return_block)
        .expect("initial registration should succeed");

    let result = plan.add_return_block(return_block);

    assert!(matches!(
        result,
        Err(ClassicalError::InvalidInput { .. })
    ));

    assert_eq!(plan.return_block_count(), 1);
}

#[test]
fn bulk_return_block_registration_is_deterministic() {
    let plan = FunctionControlFlowPlan::with_return_blocks(
        function(1),
        block(2),
        [block(30), block(10), block(20)],
    )
    .expect("distinct return blocks should register");

    let actual = plan.return_blocks().copied().collect::<Vec<_>>();

    assert_eq!(actual, vec![block(10), block(20), block(30)]);
}

#[test]
fn bulk_return_block_registration_rejects_duplicates() {
    let result = FunctionControlFlowPlan::with_return_blocks(
        function(1),
        block(2),
        [block(3), block(4), block(3)],
    );

    assert!(matches!(
        result,
        Err(ClassicalError::InvalidInput { .. })
    ));
}

#[test]
fn function_plan_accepts_entry_as_a_return_block() {
    let entry = block(2);

    let plan = FunctionControlFlowPlan::with_return_blocks(
        function(1),
        entry,
        [entry],
    )
    .expect("immediate return from entry is representable");

    assert_eq!(plan.entry(), entry);
    assert!(plan.contains_return_block(&entry));
}

#[test]
fn function_plan_has_no_small_fixed_return_block_ceiling() {
    let blocks = (1_u128..=2_048)
        .map(block)
        .collect::<Vec<_>>();

    let plan = FunctionControlFlowPlan::with_return_blocks(
        function(1),
        block(10_000),
        blocks,
    )
    .expect("all distinct supplied return blocks should be retained");

    assert_eq!(plan.return_block_count(), 2_048);
}

// -------------------------------------------------------------------------
// Calls
// -------------------------------------------------------------------------

#[test]
fn direct_call_preserves_target_arguments_and_results() {
    let caller = function(1);
    let callee = function(2);
    let operation_id = operation(3);
    let arguments = vec![value(4), value(5)];
    let results = vec![value(6)];

    let plan = CallSitePlan::new(
        operation_id,
        caller,
        CallTarget::Direct(callee),
        arguments.clone(),
        results.clone(),
    )
    .expect("distinct call results should be accepted");

    assert_eq!(plan.operation(), operation_id);
    assert_eq!(plan.caller(), caller);
    assert_eq!(plan.target().as_direct(), Some(callee));
    assert!(plan.target().is_direct());
    assert!(!plan.target().is_indirect());
    assert_eq!(plan.arguments(), arguments.as_slice());
    assert_eq!(plan.results(), results.as_slice());
    assert_eq!(plan.argument_count(), 2);
    assert_eq!(plan.result_count(), 1);
    assert!(!plan.has_no_arguments());
    assert!(!plan.has_no_results());
}

#[test]
fn indirect_call_preserves_callable_value() {
    let callable = value(10);

    let target = CallTarget::Indirect(callable);

    assert_eq!(target.as_indirect(), Some(callable));
    assert_eq!(target.as_direct(), None);
    assert!(target.is_indirect());
    assert!(!target.is_direct());
}

#[test]
fn call_without_arguments_or_results_is_representable() {
    let plan = CallSitePlan::new(
        operation(1),
        function(2),
        CallTarget::Direct(function(3)),
        Vec::new(),
        Vec::new(),
    )
    .expect("a zero-argument, zero-result call is representable");

    assert!(plan.has_no_arguments());
    assert!(plan.has_no_results());
    assert_eq!(plan.argument_count(), 0);
    assert_eq!(plan.result_count(), 0);
}

#[test]
fn duplicate_call_result_identity_is_rejected() {
    let repeated_result = value(5);

    let result = CallSitePlan::new(
        operation(1),
        function(2),
        CallTarget::Direct(function(3)),
        Vec::new(),
        vec![repeated_result, repeated_result],
    );

    assert!(matches!(
        result,
        Err(ClassicalError::InvalidInput { .. })
    ));
}

// -------------------------------------------------------------------------
// Exceptions
// -------------------------------------------------------------------------

#[test]
fn throw_plan_preserves_exception_value_identity() {
    let exception_value = value(1);
    let plan = ThrowPlan::new(exception_value);

    assert_eq!(plan.value(), exception_value);
    assert_eq!(plan.into_parts(), exception_value);
}

#[test]
fn catch_handler_preserves_binding_type_and_entry() {
    let binding = symbol(1);
    let exception_type = ty(2);
    let entry = block(3);

    let handler =
        CatchHandlerPlan::new(binding, Some(exception_type), entry);

    assert_eq!(handler.binding(), binding);
    assert_eq!(handler.exception_type(), Some(exception_type));
    assert_eq!(handler.entry(), entry);

    assert_eq!(
        handler.into_parts(),
        (binding, Some(exception_type), entry)
    );
}

#[test]
fn untyped_catch_handler_is_representable() {
    let handler = CatchHandlerPlan::new(symbol(1), None, block(2));

    assert_eq!(handler.exception_type(), None);
}

#[test]
fn try_exception_plan_requires_a_handler_or_finally_block() {
    let result = TryExceptionPlan::new(block(1), Vec::new(), None);

    assert!(matches!(
        result,
        Err(ClassicalError::InvalidInput { .. })
    ));
}

#[test]
fn try_exception_plan_accepts_finally_without_catch_handlers() {
    let body = block(1);
    let finally = block(2);

    let plan = TryExceptionPlan::new(body, Vec::new(), Some(finally))
        .expect("finally-only exception flow is representable");

    assert_eq!(plan.body_entry(), body);
    assert_eq!(plan.handler_count(), 0);
    assert!(!plan.has_handlers());
    assert!(plan.has_finally());
    assert_eq!(plan.finally_entry(), Some(finally));
}

#[test]
fn try_exception_plan_preserves_handler_order() {
    let first = CatchHandlerPlan::new(symbol(1), None, block(10));
    let second =
        CatchHandlerPlan::new(symbol(2), Some(ty(3)), block(20));

    let plan = TryExceptionPlan::new(
        block(4),
        vec![first, second],
        Some(block(30)),
    )
    .expect("at least one handler makes the try plan valid");

    assert_eq!(plan.handler_count(), 2);
    assert!(plan.has_handlers());
    assert_eq!(plan.handlers()[0].binding(), symbol(1));
    assert_eq!(plan.handlers()[0].entry(), block(10));
    assert_eq!(plan.handlers()[1].binding(), symbol(2));
    assert_eq!(plan.handlers()[1].entry(), block(20));
}

// -------------------------------------------------------------------------
// Return, break, and continue
// -------------------------------------------------------------------------

#[test]
fn return_plan_supports_value_and_value_less_returns() {
    let with_value = ReturnPlan::new(function(1), Some(value(2)));

    assert_eq!(with_value.function(), function(1));
    assert_eq!(with_value.value(), Some(value(2)));
    assert!(!with_value.is_value_less());

    let without_value = ReturnPlan::new(function(3), None);

    assert_eq!(without_value.function(), function(3));
    assert_eq!(without_value.value(), None);
    assert!(without_value.is_value_less());
}

#[test]
fn break_plan_preserves_destination_and_optional_label() {
    let destination = block(1);
    let label = symbol(2);

    let unlabelled = BreakPlan::new(destination, None);

    assert_eq!(unlabelled.destination(), destination);
    assert_eq!(unlabelled.label(), None);
    assert!(!unlabelled.is_labelled());

    let labelled = BreakPlan::new(destination, Some(label));

    assert_eq!(labelled.destination(), destination);
    assert_eq!(labelled.label(), Some(label));
    assert!(labelled.is_labelled());
}

#[test]
fn continue_plan_preserves_destination_and_optional_label() {
    let destination = block(1);
    let label = symbol(2);

    let unlabelled = ContinuePlan::new(destination, None);

    assert_eq!(unlabelled.destination(), destination);
    assert_eq!(unlabelled.label(), None);
    assert!(!unlabelled.is_labelled());

    let labelled = ContinuePlan::new(destination, Some(label));

    assert_eq!(labelled.destination(), destination);
    assert_eq!(labelled.label(), Some(label));
    assert!(labelled.is_labelled());
}

#[test]
fn termination_enum_identifies_each_control_transfer() {
    let return_plan =
        TerminationPlan::Return(ReturnPlan::new(function(1), Some(value(2))));

    assert!(return_plan.is_return());
    assert!(!return_plan.is_break());
    assert!(!return_plan.is_continue());
    assert!(return_plan.as_return().is_some());
    assert!(return_plan.as_break().is_none());
    assert!(return_plan.as_continue().is_none());

    let break_plan =
        TerminationPlan::Break(BreakPlan::new(block(3), None));

    assert!(!break_plan.is_return());
    assert!(break_plan.is_break());
    assert!(!break_plan.is_continue());
    assert!(break_plan.as_break().is_some());

    let continue_plan =
        TerminationPlan::Continue(ContinuePlan::new(block(4), None));

    assert!(!continue_plan.is_return());
    assert!(!continue_plan.is_break());
    assert!(continue_plan.is_continue());
    assert!(continue_plan.as_continue().is_some());
}

// -------------------------------------------------------------------------
// Identity representation boundaries
// -------------------------------------------------------------------------

#[test]
fn large_canonical_identifiers_are_preserved_without_index_assumptions() {
    let large = u128::MAX;

    let large_block = block(large);
    let large_value = value(large);
    let large_function = function(large);
    let large_operation = operation(large);
    let large_symbol = symbol(large);

    assert_eq!(large_block.get(), large);
    assert_eq!(large_value.get(), large);
    assert_eq!(large_function.get(), large);
    assert_eq!(large_operation.get(), large);
    assert_eq!(large_symbol.get(), large);

    // Typed IDs retain their distinct type identities even when their
    // numeric payloads happen to be equal.
    assert_ne!(
        core::any::type_name::<BlockId>(),
        core::any::type_name::<ValueId>()
    );
    assert_ne!(
        core::any::type_name::<FunctionId>(),
        core::any::type_name::<OperationId>()
    );
}
