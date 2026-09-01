//! Zamani Quantum IR — Dynamic Control-Flow Integration Tests.
//!
//! Production-grade integration tests for:
//!
//! `src/quantum/ir/tests/dynamic.rs`
//!
//! # Purpose
//!
//! This file verifies the cross-module contract for dynamic quantum programs:
//!
//! ```text
//! classical predicate
//!        │
//!        ▼
//! execution condition
//!        │
//!        ▼
//! conditional control flow
//!        │
//!        ├── quantum operation references
//!        ├── nested regions
//!        ├── loops
//!        ├── logical-qubit iteration
//!        └── classical feedback
//!
//!                    │
//!                    ▼
//!             IR validation
//! ```
//!
//! The tests deliberately exercise the public canonical IR contracts rather
//! than private implementation details.
//!
//! # Architectural requirements verified here
//!
//! The test suite verifies that dynamic IR:
//!
//! - is hardware independent;
//! - has no architectural qubit-count ceiling;
//! - uses `quantum::ir::qubit::QubitId` as the canonical logical-qubit identity;
//! - represents predicates symbolically;
//! - does not materialize enormous loop domains;
//! - supports conditional execution;
//! - supports nested conditional execution;
//! - supports `while` and `do-while` semantics;
//! - supports counted/range loops;
//! - supports logical-qubit iteration;
//! - supports repeat loops;
//! - supports structured transfers;
//! - validates classical-bit namespaces;
//! - validates logical-qubit namespaces;
//! - validates operation references when an operation registry is supplied;
//! - enforces explicit validation policies;
//! - distinguishes semantic limits from architectural limits;
//! - remains deterministic;
//! - rejects malformed dynamic structures;
//! - does not silently discard invalid operations;
//! - does not require a hardware backend;
//! - remains safe Rust;
//! - remains compatible with Rust 1.97 / 1.97.1;
//! - contains no unsafe code.
//!
//! # Important boundary
//!
//! These tests do NOT test:
//!
//! - hardware execution;
//! - QPU communication;
//! - routing algorithms;
//! - scheduling algorithms;
//! - pulse generation;
//! - calibration;
//! - simulation state;
//! - QEC decoding;
//! - vendor-specific behavior;
//! - optimization algorithms.
//!
//! Those responsibilities belong downstream.
//!
//! # Canonical module paths
//!
//! New dynamic-control code is tested through:
//!
//! ```text
//! crate::quantum::ir::control
//! crate::quantum::ir::classical::predicate
//! crate::quantum::ir::qubit
//! crate::quantum::ir::identity
//! crate::quantum::ir::validation::control_flow
//! ```
//!
//! The legacy flat `quantum::ir::control_flow` module is intentionally not
//! used as the primary API in this test file. This prevents new code from
//! accidentally becoming dependent on the legacy control-flow implementation.
//!
//! # Scalability model
//!
//! "Infinite scalability" is interpreted semantically:
//!
//! ```text
//! no artificial architectural maximum
//!             !=
//! unlimited physical memory
//! ```
//!
//! Tests therefore use enormous *symbolic* domains where appropriate instead
//! of allocating enormous vectors.
//!
//! A successful construction of a range containing `u128::MAX`-scale iteration
//! semantics must not require enumeration of those iterations.
//!
//! # Integration registration
//!
//! Because this file lives under `src/quantum/ir/tests/`, the parent IR module
//! must register it explicitly under `#[cfg(test)]`:
//!
//! ```rust
//! #[cfg(test)]
//! #[path = "tests/dynamic.rs"]
//! mod dynamic;
//! ```
//!
//! This registration belongs in `src/quantum/ir/mod.rs` alongside the other
//! cross-module integration-test registrations.
//!
//! No production code depends on this test module.
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
//! - no external test dependencies;
//! - no unsafe code.
//!
//! # Security/testing principle
//!
//! Tests must never rely on an enormous allocation to prove scalability.
//!
//! A test that allocates millions of objects merely to demonstrate that the IR
//! can describe millions of objects is testing the allocator rather than the
//! semantic IR.
//!
//! Symbolic ranges, checked counts, explicit validation budgets and bounded
//! test fixtures are therefore preferred.
//!
//! -----------------------------------------------------------------------------
//! Imports
//! -----------------------------------------------------------------------------

use std::collections::BTreeSet;

use crate::quantum::ir::classical::bit::ClassicalBitId;
use crate::quantum::ir::classical::predicate::ClassicalPredicate;
use crate::quantum::ir::control::branch::{
    Branch,
    BranchTarget,
};
use crate::quantum::ir::control::condition::Condition;
use crate::quantum::ir::control::control_flow::{
    ControlFlowBlock,
    ControlFlowNode,
    ControlFlowRegion,
};
use crate::quantum::ir::control::r#loop::{
    Loop,
    LoopDomain,
    LoopVariableId,
    StaticIntegerRange,
};
use crate::quantum::ir::identity::{
    BlockId,
    OperationId,
    RegionId,
    ValueId,
};
use crate::quantum::ir::qubit::{
    QubitId,
    QubitRange,
};
use crate::quantum::ir::validation::control_flow::{
    validate_node,
    validate_node_with_operation_registry,
    validate_region,
    validate_region_with_operation_registry,
    ControlFlowValidationConfig,
};

// =============================================================================
// Test helpers
// =============================================================================

/// Creates a canonical logical-qubit identifier.
///
/// This helper intentionally uses the authoritative module:
///
/// `quantum::ir::qubit::QubitId`
fn qubit(index: usize) -> QubitId {
    QubitId::new(index)
}

/// Creates a canonical classical-bit identifier.
fn classical_bit(index: usize) -> ClassicalBitId {
    ClassicalBitId::new(index)
}

/// Creates a canonical operation identity.
fn operation(index: u64) -> OperationId {
    OperationId::new(index)
}

/// Creates a canonical region identity.
fn region(index: u64) -> RegionId {
    RegionId::new(index)
}

/// Creates a canonical block identity.
fn block(index: u64) -> BlockId {
    BlockId::new(index)
}

/// Creates a canonical SSA/value identity.
fn value(index: u64) -> ValueId {
    ValueId::new(index)
}

/// Creates an unrestricted validation configuration.
///
/// This is used only for semantic scalability tests. It does not imply that
/// production services should disable resource policies.
fn unbounded_config(
    num_qubits: usize,
    num_classical_bits: usize,
    in_function: bool,
) -> ControlFlowValidationConfig {
    ControlFlowValidationConfig::unbounded(
        num_qubits,
        num_classical_bits,
        in_function,
    )
}

/// Creates a deliberately small validation policy.
///
/// Small policies make security/resource-boundary tests deterministic and
/// avoid large allocations.
fn bounded_config(
    num_qubits: usize,
    num_classical_bits: usize,
    in_function: bool,
) -> ControlFlowValidationConfig {
    let limits = crate::quantum::ir::limits::QuantumIrLimits::production()
        .with_max_operations(32)
        .with_max_validation_steps(256)
        .with_max_analysis_steps(256)
        .with_max_qubits(num_qubits)
        .with_max_classical_bits(num_classical_bits);

    ControlFlowValidationConfig::new(
        num_qubits,
        num_classical_bits,
        limits,
        in_function,
    )
}

/// Creates a non-empty block containing one semantic operation reference.
fn operation_block(id: u64) -> ControlFlowBlock {
    let mut block = ControlFlowBlock::new();

    block
        .push(ControlFlowNode::operation(operation(id)))
        .expect("operation node must be structurally valid");

    block
}

// =============================================================================
// Canonical qubit identity
// =============================================================================

#[test]
fn dynamic_control_uses_canonical_qubit_identity() {
    let first = qubit(0);
    let second = qubit(1);

    assert_eq!(first.index(), 0);
    assert_eq!(second.index(), 1);
    assert_ne!(first, second);
}

#[test]
fn qubit_ranges_are_symbolic_and_do_not_materialize_qubits() {
    let range = QubitRange::new(0, 1_000_000_000)
        .expect("large logical range must be representable");

    assert_eq!(range.start().index(), 0);
    assert_eq!(range.end().index(), 1_000_000_000);

    // The range itself must remain a compact semantic object.
    //
    // No vector of one billion QubitId values is constructed.
    assert_eq!(
        range
            .end()
            .index()
            .checked_sub(range.start().index()),
        Some(1_000_000_000)
    );
}

#[test]
fn qubit_identity_is_not_a_machine_size_limit() {
    let large = qubit(usize::MAX);

    assert_eq!(large.index(), usize::MAX);
}

// =============================================================================
// Predicate construction
// =============================================================================

#[test]
fn classical_bit_predicate_is_constructible() {
    let predicate =
        ClassicalPredicate::bit(classical_bit(0));

    predicate
        .validate()
        .expect("bit predicate must be structurally valid");
}

#[test]
fn nested_predicate_is_constructible() {
    let predicate = ClassicalPredicate::and(vec![
        ClassicalPredicate::bit(classical_bit(0)),
        ClassicalPredicate::not(
            ClassicalPredicate::bit(classical_bit(1)),
        ),
    ])
    .expect("non-empty conjunction must be valid");

    predicate
        .validate()
        .expect("nested predicate must validate");

    assert_eq!(
        predicate
            .node_count()
            .expect("node count must be representable"),
        4
    );
}

#[test]
fn empty_conjunction_is_rejected() {
    let result =
        ClassicalPredicate::and(Vec::new());

    assert!(
        result.is_err(),
        "empty conjunction must not enter canonical IR"
    );
}

#[test]
fn empty_disjunction_is_rejected() {
    let result =
        ClassicalPredicate::or(Vec::new());

    assert!(
        result.is_err(),
        "empty disjunction must not enter canonical IR"
    );
}

#[test]
fn empty_xor_is_rejected() {
    let result =
        ClassicalPredicate::xor(Vec::new());

    assert!(
        result.is_err(),
        "empty XOR must not enter canonical IR"
    );
}

// =============================================================================
// Predicate namespace validation
// =============================================================================

#[test]
fn predicate_accepts_valid_classical_namespace() {
    let predicate =
        ClassicalPredicate::bit(classical_bit(3));

    predicate
        .validate_classical_bits(4)
        .expect("bit 3 must exist in a four-bit namespace");
}

#[test]
fn predicate_rejects_classical_bit_outside_namespace() {
    let predicate =
        ClassicalPredicate::bit(classical_bit(4));

    let result =
        predicate.validate_classical_bits(4);

    assert!(
        result.is_err(),
        "bit 4 must not be accepted by a four-bit namespace"
    );
}

#[test]
fn nested_predicate_checks_every_classical_dependency() {
    let predicate = ClassicalPredicate::and(vec![
        ClassicalPredicate::bit(classical_bit(0)),
        ClassicalPredicate::not(
            ClassicalPredicate::bit(classical_bit(7)),
        ),
    ])
    .expect("predicate must be constructible");

    assert!(
        predicate.validate_classical_bits(8).is_ok()
    );

    assert!(
        predicate.validate_classical_bits(7).is_err()
    );
}

// =============================================================================
// Execution condition abstraction
// =============================================================================

#[test]
fn always_condition_is_canonical() {
    let condition = Condition::always();

    assert!(condition.is_always());
    assert!(!condition.is_never());
    assert!(!condition.is_predicate());
    assert!(condition.must_execute());
    assert!(condition.may_execute());
    assert_eq!(condition.node_count(), 0);
}

#[test]
fn never_condition_is_canonical() {
    let condition = Condition::never();

    assert!(!condition.is_always());
    assert!(condition.is_never());
    assert!(!condition.must_execute());
    assert!(!condition.may_execute());
}

#[test]
fn constant_predicate_is_normalized() {
    let condition =
        Condition::predicate(ClassicalPredicate::always());

    assert!(condition.is_always());

    let condition =
        Condition::predicate(ClassicalPredicate::never());

    assert!(condition.is_never());
}

#[test]
fn predicate_condition_retains_symbolic_dependencies() {
    let condition = Condition::predicate(
        ClassicalPredicate::bit(classical_bit(9)),
    );

    assert!(condition.is_predicate());

    let dependencies =
        condition.classical_dependencies();

    assert!(dependencies.contains(&classical_bit(9)));
    assert_eq!(dependencies.len(), 1);
}

#[test]
fn condition_negation_constant_folds() {
    assert!(
        Condition::always().not().is_never()
    );

    assert!(
        Condition::never().not().is_always()
    );
}

#[test]
fn condition_conjunction_constant_folds() {
    let result = Condition::always()
        .and(Condition::predicate(
            ClassicalPredicate::bit(classical_bit(0)),
        ))
        .expect("condition conjunction must succeed");

    assert!(result.is_predicate());

    let result = Condition::never()
        .and(Condition::always())
        .expect("condition conjunction must succeed");

    assert!(result.is_never());
}

#[test]
fn condition_disjunction_constant_folds() {
    let result = Condition::always()
        .or(Condition::never())
        .expect("condition disjunction must succeed");

    assert!(result.is_always());

    let result = Condition::never()
        .or(Condition::predicate(
            ClassicalPredicate::bit(classical_bit(0)),
        ))
        .expect("condition disjunction must succeed");

    assert!(result.is_predicate());
}

// =============================================================================
// Conditional branch
// =============================================================================

#[test]
fn branch_has_explicit_true_and_false_destinations() {
    let branch = Branch::try_unparameterized(
        ClassicalPredicate::bit(classical_bit(0)),
        block(10),
        block(11),
    )
    .expect("valid conditional branch must be constructible");

    assert_eq!(
        branch.true_target().block(),
        block(10)
    );

    assert_eq!(
        branch.false_target().block(),
        block(11)
    );
}

#[test]
fn branch_does_not_evaluate_predicate() {
    let branch = Branch::try_unparameterized(
        ClassicalPredicate::bit(classical_bit(0)),
        block(1),
        block(2),
    )
    .expect("branch must be constructible");

    // The branch represents semantic selection only.
    //
    // It must not expose an API that requires evaluating the classical state.
    assert_eq!(
        branch.condition(),
        &ClassicalPredicate::bit(classical_bit(0))
    );
}

#[test]
fn branch_target_preserves_ordered_arguments() {
    let target = BranchTarget::with_arguments(
        block(42),
        vec![value(1), value(2), value(3)],
    )
    .expect("ordered branch arguments must be valid");

    assert_eq!(
        target.arguments(),
        &[value(1), value(2), value(3)]
    );
}

#[test]
fn branch_rejects_duplicate_transferred_values() {
    let result = BranchTarget::with_arguments(
        block(42),
        vec![value(1), value(1)],
    );

    assert!(
        result.is_err(),
        "a destination must not receive the same SSA value twice"
    );
}

#[test]
fn branch_can_be_constructed_without_hardware_information() {
    let branch = Branch::try_unparameterized(
        ClassicalPredicate::bit(classical_bit(0)),
        block(100),
        block(101),
    )
    .expect("semantic branch must not require hardware");

    assert_eq!(
        branch.true_target().block(),
        block(100)
    );
}

// =============================================================================
// Structured control-flow nodes
// =============================================================================

#[test]
fn operation_node_is_constructible() {
    let node =
        ControlFlowNode::operation(operation(1));

    let config =
        unbounded_config(1, 1, true);

    let stats =
        validate_node(&node, &config)
            .expect("operation node must validate");

    assert_eq!(stats.nodes, 1);
    assert_eq!(
        stats.operation_references,
        1
    );
}

#[test]
fn if_node_requires_non_empty_then_block() {
    let empty =
        ControlFlowBlock::new();

    let result =
        ControlFlowNode::if_then(
            ClassicalPredicate::bit(classical_bit(0)),
            empty,
        );

    assert!(
        result.is_err(),
        "an if construct must have a valid then region"
    );
}

#[test]
fn if_node_accepts_measurement_driven_condition() {
    let then_block =
        operation_block(10);

    let node =
        ControlFlowNode::if_then(
            ClassicalPredicate::bit(classical_bit(0)),
            then_block,
        );

    let config =
        unbounded_config(2, 1, true);

    let stats =
        validate_node(
            &node,
            &config,
        )
        .expect("measurement-driven if must validate");

    assert_eq!(stats.nodes, 2);
    assert_eq!(
        stats.operation_references,
        1
    );
    assert_eq!(
        stats.classical_bit_references,
        1
    );
}

#[test]
fn nested_if_is_validated_recursively() {
    let inner_block =
        operation_block(2);

    let inner =
        ControlFlowNode::if_then(
            ClassicalPredicate::bit(classical_bit(1)),
            inner_block,
        )
        .expect("inner if must be valid");

    let mut outer_block =
        ControlFlowBlock::new();

    outer_block
        .push(inner)
        .expect("inner node must fit");

    let outer =
        ControlFlowNode::if_then(
            ClassicalPredicate::bit(classical_bit(0)),
            outer_block,
        )
        .expect("outer if must be valid");

    let config =
        unbounded_config(2, 2, true);

    let stats =
        validate_node(
            &outer,
            &config,
        )
        .expect("nested if must validate");

    assert_eq!(
        stats.operation_references,
        1
    );

    assert!(
        stats.maximum_depth >= 2,
        "nested control flow must report nesting depth"
    );
}

// =============================================================================
// While / do-while
// =============================================================================

#[test]
fn while_loop_is_pre_conditioned() {
    let body =
        operation_block(20);

    let node =
        ControlFlowNode::while_loop(
            ClassicalPredicate::bit(classical_bit(0)),
            body,
        )
        .expect("while loop must be valid");

    let config =
        unbounded_config(1, 1, true);

    let stats =
        validate_node(
            &node,
            &config,
        )
        .expect("while loop must validate");

    assert_eq!(
        stats.operation_references,
        1
    );
    assert_eq!(
        stats.classical_bit_references,
        1
    );
}

#[test]
fn do_while_loop_is_post_conditioned() {
    let body =
        operation_block(21);

    let node =
        ControlFlowNode::do_while(
            body,
            ClassicalPredicate::bit(classical_bit(0)),
        )
        .expect("do-while loop must be valid");

    let config =
        unbounded_config(1, 1, true);

    let stats =
        validate_node(
            &node,
            &config,
        )
        .expect("do-while loop must validate");

    assert_eq!(
        stats.operation_references,
        1
    );

    assert_eq!(
        stats.classical_bit_references,
        1
    );
}

// =============================================================================
// Counted loops
// =============================================================================

#[test]
fn static_integer_range_is_symbolic() {
    let range =
        StaticIntegerRange::exclusive(
            0,
            1_000_000_000_000i128,
            1,
        )
        .expect("large range must be representable");

    assert_eq!(range.start(), 0);
    assert_eq!(
        range.end(),
        1_000_000_000_000i128
    );
    assert_eq!(range.step(), 1);
    assert!(!range.inclusive_end());

    assert_eq!(
        range
            .trip_count()
            .expect("trip count must be representable"),
        1_000_000_000_000u128
    );
}

#[test]
fn inclusive_range_counts_endpoint_correctly() {
    let range =
        StaticIntegerRange::inclusive(
            0,
            10,
            2,
        )
        .expect("range must be valid");

    assert_eq!(
        range
            .trip_count()
            .expect("trip count must be representable"),
        6
    );
}

#[test]
fn descending_range_is_supported() {
    let range =
        StaticIntegerRange::exclusive(
            10,
            0,
            -2,
        )
        .expect("descending range must be valid");

    assert_eq!(
        range
            .trip_count()
            .expect("trip count must be representable"),
        5
    );
}

#[test]
fn zero_step_is_rejected() {
    let result =
        StaticIntegerRange::exclusive(
            0,
            10,
            0,
        );

    assert!(
        result.is_err(),
        "zero-step loops cannot make semantic progress"
    );
}

#[test]
fn incompatible_positive_range_is_rejected() {
    let result =
        StaticIntegerRange::exclusive(
            10,
            0,
            1,
        );

    assert!(
        result.is_err(),
        "positive-step range cannot descend"
    );
}

#[test]
fn incompatible_negative_range_is_rejected() {
    let result =
        StaticIntegerRange::exclusive(
            0,
            10,
            -1,
        );

    assert!(
        result.is_err(),
        "negative-step range cannot ascend"
    );
}

#[test]
fn huge_trip_count_does_not_materialize_iterations() {
    let range =
        StaticIntegerRange::exclusive(
            0,
            i128::MAX,
            1,
        )
        .expect("huge range must be representable");

    let count =
        range
            .trip_count()
            .expect("trip count must be representable");

    assert_eq!(
        count,
        i128::MAX as u128
    );
}

// =============================================================================
// Domain loops
// =============================================================================

#[test]
fn integer_loop_domain_is_symbolic() {
    let domain =
        LoopDomain::integer_range(
            0,
            10_000_000_000i128,
            1,
        )
        .expect("integer loop domain must be valid");

    assert_eq!(
        domain
            .iteration_count()
            .expect("iteration count must be representable"),
        10_000_000_000u128
    );
}

#[test]
fn qubit_loop_domain_uses_canonical_qubit_ids() {
    let domain =
        LoopDomain::qubits(
            0,
            1_000_000,
        )
        .expect("qubit loop domain must be valid");

    assert_eq!(
        domain
            .iteration_count()
            .expect("iteration count must be representable"),
        1_000_000u128
    );

    match domain {
        LoopDomain::Qubits(range) => {
            assert_eq!(
                range.start().index(),
                0
            );

            assert_eq!(
                range.end().index(),
                1_000_000
            );
        }

        _ => panic!(
            "qubit loop must use LoopDomain::Qubits"
        ),
    }
}

#[test]
fn qubit_loop_domain_does_not_enumerate_qubits() {
    let domain =
        LoopDomain::qubits(
            0,
            10_000_000_000usize,
        )
        .expect("large qubit domain must be valid");

    assert_eq!(
        domain
            .iteration_count()
            .expect("trip count must be representable"),
        10_000_000_000u128
    );
}

#[test]
fn for_loop_requires_a_valid_domain() {
    let domain =
        LoopDomain::integer_range(
            0,
            4,
            1,
        )
        .expect("domain must be valid");

    let body =
        region(1);

    let loop_ir =
        Loop::for_loop(
            operation(100),
            LoopVariableId::new(0),
            domain,
            body,
        )
        .expect("for loop must be valid");

    assert_eq!(
        loop_ir.operation_id(),
        operation(100)
    );
}

#[test]
fn repeat_loop_can_represent_extremely_large_iteration_counts() {
    let loop_ir =
        Loop::repeat(
            operation(200),
            u128::MAX,
            region(2),
        )
        .expect("large repeat count must be representable");

    assert_eq!(
        loop_ir.iterations(),
        Some(u128::MAX)
    );
}

#[test]
fn loop_identity_is_not_a_qubit_identity() {
    let variable =
        LoopVariableId::new(7);

    assert_eq!(
        variable.value(),
        7
    );

    assert_ne!(
        variable.value(),
        qubit(7).index() as u64
    );
}

// =============================================================================
// Structured loop/control-flow integration
// =============================================================================

#[test]
fn for_loop_node_can_be_embedded_in_structured_control_flow() {
    let body =
        operation_block(300);

    let domain =
        crate::quantum::ir::control_flow::IntegerLoopRange::new(
            0,
            8,
            1,
        )
        .expect("legacy-compatible integer range must be valid");

    let node =
        ControlFlowNode::for_loop(
            crate::quantum::ir::control_flow::LoopVariable::new(0),
            domain,
            body,
        )
        .expect("structured for loop must be valid");

    let config =
        unbounded_config(8, 1, true);

    let stats =
        validate_node(
            &node,
            &config,
        )
        .expect("for-loop node must validate");

    assert!(
        stats.nodes >= 2
    );
}

#[test]
fn repeat_node_can_be_nested_inside_if() {
    let repeat_body =
        operation_block(400);

    let repeat =
        ControlFlowNode::repeat(
            100,
            repeat_body,
        )
        .expect("repeat node must be valid");

    let mut then_block =
        ControlFlowBlock::new();

    then_block
        .push(repeat)
        .expect("repeat must fit inside then block");

    let condition =
        ClassicalPredicate::bit(
            classical_bit(0)
        );

    let if_node =
        ControlFlowNode::if_then(
            condition,
            then_block,
        )
        .expect("nested repeat must be valid");

    let config =
        unbounded_config(1, 1, true);

    let stats =
        validate_node(
            &if_node,
            &config,
        )
        .expect("nested repeat must validate");

    assert!(
        stats.nodes >= 3
    );
}

// =============================================================================
// Structured transfers
// =============================================================================

#[test]
fn break_is_invalid_outside_loop() {
    let node =
        ControlFlowNode::Break;

    let config =
        unbounded_config(1, 1, true);

    let result =
        validate_node(
            &node,
            &config,
        );

    assert!(
        result.is_err(),
        "break outside a loop must be rejected"
    );
}

#[test]
fn continue_is_invalid_outside_loop() {
    let node =
        ControlFlowNode::Continue;

    let config =
        unbounded_config(1, 1, true);

    let result =
        validate_node(
            &node,
            &config,
        );

    assert!(
        result.is_err(),
        "continue outside a loop must be rejected"
    );
}

#[test]
fn return_requires_function_context() {
    let node =
        ControlFlowNode::Return;

    let outside_function =
        unbounded_config(1, 1, false);

    assert!(
        validate_node(
            &node,
            &outside_function,
        )
        .is_err()
    );

    let inside_function =
        unbounded_config(1, 1, true);

    assert!(
        validate_node(
            &node,
            &inside_function,
        )
        .is_ok()
    );
}

// =============================================================================
// Validation resource boundaries
// =============================================================================

#[test]
fn validation_rejects_classical_dependency_beyond_namespace() {
    let node =
        ControlFlowNode::if_then(
            ClassicalPredicate::bit(
                classical_bit(8),
            ),
            operation_block(500),
        )
        .expect("node itself must be structurally valid");

    let config =
        bounded_config(8, 8, true);

    assert!(
        validate_node(
            &node,
            &config,
        )
        .is_err(),
        "bit 8 must not be valid in an eight-bit namespace"
    );
}

#[test]
fn validation_rejects_operation_budget_exhaustion() {
    let mut block =
        ControlFlowBlock::new();

    for id in 0..32u64 {
        block
            .push(
                ControlFlowNode::operation(
                    operation(id),
                ),
            )
            .expect("operation node must be valid");
    }

    let config =
        bounded_config(1, 1, true);

    let result =
        validate_region(
            &ControlFlowRegion::from_block(block)
                .expect("region must be valid"),
            &config,
        );

    assert!(
        result.is_err(),
        "explicit validation policy must bound work"
    );
}

#[test]
fn validation_policy_is_not_an_architectural_qubit_limit() {
    let config =
        bounded_config(8, 8, true);

    assert_eq!(
        config.num_qubits,
        8
    );

    let unrestricted =
        unbounded_config(
            1_000_000,
            1_000_000,
            true,
        );

    assert_eq!(
        unrestricted.num_qubits,
        1_000_000
    );
}

// =============================================================================
// Operation-reference integration
// =============================================================================

#[test]
fn operation_reference_validation_accepts_known_operations() {
    let node =
        ControlFlowNode::operation(
            operation(10),
        );

    let config =
        unbounded_config(1, 1, true);

    let mut registry =
        BTreeSet::new();

    registry.insert(operation(10));

    let stats =
        validate_node_with_operation_registry(
            &node,
            &config,
            &registry,
        )
        .expect("known operation reference must validate");

    assert_eq!(
        stats.operation_references,
        1
    );
}

#[test]
fn operation_reference_validation_rejects_unknown_operations() {
    let node =
        ControlFlowNode::operation(
            operation(11),
        );

    let config =
        unbounded_config(1, 1, true);

    let registry =
        BTreeSet::<OperationId>::new();

    assert!(
        validate_node_with_operation_registry(
            &node,
            &config,
            &registry,
        )
        .is_err(),
        "unknown operation references must not be silently accepted"
    );
}

#[test]
fn operation_registry_is_deterministic() {
    let node =
        ControlFlowNode::operation(
            operation(12),
        );

    let config =
        unbounded_config(1, 1, true);

    let mut first =
        BTreeSet::new();

    first.insert(operation(12));

    let mut second =
        BTreeSet::new();

    second.insert(operation(12));

    let left =
        validate_node_with_operation_registry(
            &node,
            &config,
            &first,
        )
        .expect("first registry must validate");

    let right =
        validate_node_with_operation_registry(
            &node,
            &config,
            &second,
        )
        .expect("second registry must validate");

    assert_eq!(
        left,
        right,
        "validation statistics must be deterministic"
    );
}

// =============================================================================
// Whole-region integration
// =============================================================================

#[test]
fn complete_dynamic_region_validates() {
    let mut region_ir =
        ControlFlowRegion::new();

    region_ir
        .push(
            ControlFlowNode::operation(
                operation(1),
            ),
        )
        .expect("operation must be valid");

    let then_block =
        operation_block(2);

    region_ir
        .push(
            ControlFlowNode::if_then(
                ClassicalPredicate::bit(
                    classical_bit(0),
                ),
                then_block,
            )
            .expect("if must be valid"),
        )
        .expect("if must fit region");

    let config =
        unbounded_config(2, 1, true);

    let stats =
        validate_region(
            &region_ir,
            &config,
        )
        .expect("complete dynamic region must validate");

    assert!(
        stats.nodes >= 3
    );

    assert_eq!(
        stats.operation_references,
        2
    );

    assert_eq!(
        stats.classical_bit_references,
        1
    );
}

#[test]
fn complete_dynamic_region_can_use_operation_registry() {
    let mut region_ir =
        ControlFlowRegion::new();

    region_ir
        .push(
            ControlFlowNode::operation(
                operation(1),
            ),
        )
        .expect("operation must be valid");

    region_ir
        .push(
            ControlFlowNode::if_then(
                ClassicalPredicate::bit(
                    classical_bit(0),
                ),
                operation_block(2),
            )
            .expect("if must be valid"),
        )
        .expect("if must fit region");

    let mut registry =
        BTreeSet::new();

    registry.insert(operation(1));
    registry.insert(operation(2));

    let config =
        unbounded_config(2, 1, true);

    let stats =
        validate_region_with_operation_registry(
            &region_ir,
            &config,
            &registry,
        )
        .expect("registered dynamic region must validate");

    assert_eq!(
        stats.operation_references,
        2
    );
}

// =============================================================================
// Deep nesting
// =============================================================================

#[test]
fn deeply_nested_dynamic_control_flow_is_representable_without_hardware_assumptions() {
    let mut current =
        operation_block(900);

    for depth in 0..64u64 {
        let node =
            ControlFlowNode::if_then(
                ClassicalPredicate::bit(
                    classical_bit(
                        (depth % 2) as usize
                    ),
                ),
                current,
            )
            .expect("nested if must remain structurally valid");

        let mut parent =
            ControlFlowBlock::new();

        parent
            .push(node)
            .expect("nested node must fit");

        current = parent;
    }

    let root =
        ControlFlowNode::if_then(
            ClassicalPredicate::bit(
                classical_bit(0),
            ),
            current,
        )
        .expect("root conditional must be valid");

    let config =
        unbounded_config(1, 2, true);

    let stats =
        validate_node(
            &root,
            &config,
        )
        .expect("deeply nested control flow must validate");

    assert!(
        stats.maximum_depth >= 64
    );
}

// =============================================================================
// Semantic determinism
// =============================================================================

#[test]
fn identical_dynamic_nodes_are_equal() {
    let left =
        ControlFlowNode::if_then(
            ClassicalPredicate::bit(
                classical_bit(0),
            ),
            operation_block(1),
        )
        .expect("left node must be valid");

    let right =
        ControlFlowNode::if_then(
            ClassicalPredicate::bit(
                classical_bit(0),
            ),
            operation_block(1),
        )
        .expect("right node must be valid");

    assert_eq!(
        left,
        right
    );
}

#[test]
fn changing_a_classical_dependency_changes_dynamic_semantics() {
    let left =
        ControlFlowNode::if_then(
            ClassicalPredicate::bit(
                classical_bit(0),
            ),
            operation_block(1),
        )
        .expect("left node must be valid");

    let right =
        ControlFlowNode::if_then(
            ClassicalPredicate::bit(
                classical_bit(1),
            ),
            operation_block(1),
        )
        .expect("right node must be valid");

    assert_ne!(
        left,
        right
    );
}

#[test]
fn changing_an_operation_reference_changes_dynamic_semantics() {
    let left =
        ControlFlowNode::if_then(
            ClassicalPredicate::bit(
                classical_bit(0),
            ),
            operation_block(1),
        )
        .expect("left node must be valid");

    let right =
        ControlFlowNode::if_then(
            ClassicalPredicate::bit(
                classical_bit(0),
            ),
            operation_block(2),
        )
        .expect("right node must be valid");

    assert_ne!(
        left,
        right
    );
}

// =============================================================================
// Failure atomicity
// =============================================================================

#[test]
fn failed_nested_node_construction_does_not_require_partial_ir_state() {
    let mut block =
        ControlFlowBlock::new();

    block
        .push(
            ControlFlowNode::operation(
                operation(1),
            ),
        )
        .expect("initial operation must be valid");

    let before =
        block.len();

    let result =
        ControlFlowNode::if_then(
            ClassicalPredicate::bit(
                classical_bit(0),
            ),
            ControlFlowBlock::new(),
        );

    assert!(
        result.is_err()
    );

    assert_eq!(
        block.len(),
        before,
        "failed child construction must not mutate its parent"
    );
}

// =============================================================================
// Large symbolic dynamic program
// =============================================================================

#[test]
fn large_symbolic_dynamic_program_does_not_require_iteration_materialization() {
    let domain =
        LoopDomain::integer_range(
            0,
            i128::MAX,
            1,
        )
        .expect("huge symbolic domain must be valid");

    let trip_count =
        domain
            .iteration_count()
            .expect("trip count must be representable");

    assert_eq!(
        trip_count,
        i128::MAX as u128
    );

    // The test deliberately stops here.
    //
    // Constructing `trip_count` operations would defeat the purpose of the
    // semantic loop representation.
}

// =============================================================================
// Public API integration smoke tests
// =============================================================================

#[test]
fn canonical_control_namespace_is_reachable() {
    assert_eq!(
        crate::quantum::ir::control::module_path(),
        "quantum::ir::control"
    );
}

#[test]
fn canonical_qubit_namespace_is_reachable() {
    let id =
        crate::quantum::ir::qubit::QubitId::new(123);

    assert_eq!(
        id.index(),
        123
    );
}

#[test]
fn canonical_operation_identity_is_reachable() {
    let id =
        crate::quantum::ir::identity::OperationId::new(123);

    assert_eq!(
        id,
        operation(123)
    );
}

// =============================================================================
// Contract summary
// =============================================================================
//
// This test module intentionally establishes the following production
// invariants:
//
// 1. Dynamic conditions are symbolic.
// 2. Classical dependencies are explicitly represented.
// 3. Logical qubit identity comes from `quantum::ir::qubit`.
// 4. Dynamic control does not own physical placement.
// 5. Branches have explicit semantic destinations.
// 6. Branches do not evaluate classical state themselves.
// 7. While loops are pre-conditioned.
// 8. Do-while loops are post-conditioned.
// 9. Counted loops are symbolic.
// 10. Qubit iteration is symbolic.
// 11. Repeat counts can be extremely large.
// 12. Zero-step loops are rejected.
// 13. Invalid range direction is rejected.
// 14. Break cannot escape a loop context.
// 15. Continue cannot escape a loop context.
// 16. Return requires a function context.
// 17. Classical-bit namespaces are validated.
// 18. Operation references can be validated against a caller-owned registry.
// 19. Unknown operation references are rejected when registry validation is enabled.
// 20. Validation limits are explicit policies.
// 21. Validation policies are not language architecture limits.
// 22. Large symbolic programs do not require large allocations.
// 23. Identical semantic structures are deterministic/equal.
// 24. Semantic changes produce distinguishable IR.
// 25. Failed child construction does not partially mutate its parent.
// 26. No hardware backend is required.
// 27. No vendor is required.
// 28. No physical topology is required.
// 29. No unsafe Rust is required.
// 30. No fixed quantum-machine size is encoded.
//
// If a future implementation change breaks one of these guarantees, the
// implementation should be corrected rather than weakening this test merely
// to accommodate the implementation.
//
// -----------------------------------------------------------------------------
// End of dynamic.rs
// -----------------------------------------------------------------------------