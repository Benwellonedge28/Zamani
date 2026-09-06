//! Zamani Quantum Scheduling — Regression Test Suite
//!
//! Path:
//!     src/quantum/scheduling/tests/regression.rs
//!
//! # Purpose
//!
//! This module permanently pins previously discovered classes of scheduling
//! failures.
//
//! Regression tests are deliberately different from property tests:
//
//! - property tests validate general mathematical laws;
//! - regression tests pin specific failure modes and architectural contracts;
//! - integration tests validate subsystem-to-subsystem behavior;
//! - scalability tests validate growth behavior;
//! - determinism tests validate reproducibility.
//
//! A regression test should answer:
//
//! > "Could a previously corrected scheduling defect silently return?"
//
//! If yes, this file is the correct place for the test.
//
//! # Architectural boundary
//!
//! ```text
//!                         Zamani quantum::ir
//!                                │
//!                                ▼
//!                     scheduling::adapters::ir
//!                                │
//!                                ▼
//!                      scheduling::ir::operation
//!                                │
//!                                ▼
//!                      scheduling::ir::graph
//!                                │
//!              ┌─────────────────┼─────────────────┐
//!              ▼                 ▼                 ▼
//!         dependency          timing           resources
//!              │                 │                 │
//!              └─────────────────┼─────────────────┘
//!                                ▼
//!                           planners
//!                                │
//!                                ▼
//!                          verification
//! ```
//!
//! This file tests stable scheduling contracts rather than implementation
//! details of a particular scheduling algorithm.
//
//! # Regression philosophy
//!
//! A regression test MUST prefer an observable contract over an implementation
//! detail.
//
//! Good:
//
//! - duplicate operation IDs are rejected;
//! - duplicate dependency IDs are rejected;
//! - self dependencies are rejected;
//! - dependencies cannot reference unknown operations;
//! - cyclic graphs cannot produce a topological schedule;
//! - failed atomic insertion does not partially mutate a graph;
//! - deterministic graph construction produces deterministic ordering;
//! - canonical qubit identities remain canonical;
//! - logical and physical qubit identities cannot silently collapse;
//! - checked identity arithmetic never wraps;
//! - checked time arithmetic never wraps.
//
//! Bad:
//
//! - asserting a particular private `Vec` layout;
//! - asserting a particular planner's internal queue;
//! - asserting a particular hash-map implementation;
//! - assuming a fixed number of qubits;
//! - assuming a fixed hardware topology;
//! - assuming a fixed gate set;
//! - assuming a fixed resource count.
//
//! # Scalability
//!
//! These tests contain no production machine limits.
//
//! In particular, this module does NOT define:
//
//! - `MAX_QUBITS`;
//! - `MAX_GATES`;
//! - `MAX_RESOURCES`;
//! - `MAX_DEPTH`;
//! - `MAX_CHANNELS`;
//! - `MAX_TIME`.
//
//! Any finite values used by an individual test are deliberately tiny
//! regression fixtures. They describe a failure shape, not a scheduler
//! capacity.
//
//! The production scheduler remains constrained only by:
//!
//! 1. available execution resources;
//! 2. explicitly supplied scheduling limits;
//! 3. target capabilities;
//! 4. compiler/runtime resource availability.
//
//! # Canonical identity rule
//!
//! Quantum qubit identity belongs to:
//!
//! ```text
//! crate::quantum::ir::qubit
//! ```
//!
//! This file therefore deliberately imports and exercises:
//
//! ```text
//! QubitId
//! PhysicalQubitId
//! QubitRef
//! ```
//!
//! It must never introduce a scheduling-local qubit identity.
//
//! # Rust contract
//!
//! Designed for:
//
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//
//! # Integration contract
//!
//! This file is intended to be included from:
//
//! ```text
//! src/quantum/scheduling/tests/mod.rs
//! ```
//!
//! using:
//
//! ```text
//! mod regression;
//! ```
//!
//! It depends only on public scheduling contracts.
//
//! Consequently, adding:
//
//! - a new planner;
//! - a new scheduling algorithm;
//! - a new hardware adapter;
//! - a new routing implementation;
//! - a new QEC implementation;
//! - a new resource type;
//! - a new optimization objective;
//
//! must not require changing this file unless the public semantic contract
//! itself intentionally changes.
//
//! # Safety
//!
//! No unsafe code is permitted.
//
//! The compiler enforces that requirement with `forbid(unsafe_code)`.

#![cfg(test)]
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::collections::BTreeSet;

use crate::quantum::ir::core::identity::OperationId;

use crate::quantum::ir::qubit::{
    PhysicalQubitId,
    QubitId,
    QubitRef,
};

use crate::quantum::scheduling::ir::graph::{
    DependencyGraph,
    DependencyGraphError,
};

use crate::quantum::scheduling::types::{
    DependencyId,
    DependencyKind,
    DependencyRef,
    Duration,
    OperationRef,
    TimePoint,
};

// =============================================================================
// Test helpers
// =============================================================================

/// Creates a canonical scheduling operation reference.
///
/// `OperationRef` wraps the canonical quantum IR `OperationId`; regression
/// tests must therefore never construct an independent scheduler operation
/// identity.
fn operation(value: u64) -> OperationRef {
    OperationRef::new(OperationId::new(value))
}

/// Creates an explicit scheduling dependency.
fn dependency(
    value: u64,
    from: OperationRef,
    to: OperationRef,
) -> DependencyRef {
    DependencyRef::new(
        DependencyId::new(value),
        from,
        to,
        DependencyKind::Explicit,
    )
}

/// Builds a graph containing the supplied operations.
///
/// Keeping this helper intentionally small ensures that regression tests
/// exercise the public graph contract rather than bypassing it.
fn graph_with_operations(
    operations: &[OperationRef],
) -> DependencyGraph {
    DependencyGraph::from_operations(
        operations.iter().copied(),
    )
    .expect("regression fixture must construct a valid graph")
}

// =============================================================================
// Canonical identity regressions
// =============================================================================

/// Regression guard:
///
/// Scheduling must continue using the canonical `quantum::ir::qubit::QubitId`.
///
/// A scheduler-local qubit identity would allow two numerically equivalent
/// qubits to become semantically different objects at subsystem boundaries.
#[test]
fn regression_scheduler_uses_canonical_logical_qubit_identity() {
    let first = QubitId::new(7);
    let second = QubitId::new(7);

    assert_eq!(first, second);
    assert_eq!(first.index(), 7);
    assert_eq!(second.index(), 7);
}

/// Regression guard for canonical physical qubit identity.
#[test]
fn regression_scheduler_uses_canonical_physical_qubit_identity() {
    let first = PhysicalQubitId::new(11);
    let second = PhysicalQubitId::new(11);

    assert_eq!(first, second);
    assert_eq!(first.index(), 11);
    assert_eq!(second.index(), 11);
}

/// Regression guard:
///
/// A logical qubit and a physical qubit are different semantic identities even
/// when their numeric indices happen to match.
#[test]
fn regression_logical_and_physical_qubits_do_not_collapse() {
    let logical = QubitRef::Logical(QubitId::new(5));
    let physical = QubitRef::Physical(PhysicalQubitId::new(5));

    assert_ne!(logical, physical);

    assert!(logical.is_logical());
    assert!(!logical.is_physical());

    assert!(!physical.is_logical());
    assert!(physical.is_physical());
}

// =============================================================================
// Operation identity regressions
// =============================================================================

/// Regression guard:
///
/// An operation reference must preserve the canonical IR operation identity.
#[test]
fn regression_operation_reference_preserves_canonical_operation_id() {
    let canonical = OperationId::new(42);
    let reference = OperationRef::new(canonical);

    assert_eq!(reference.id(), canonical);
}

/// Regression guard for the maximum representable canonical operation ID.
#[test]
fn regression_operation_reference_handles_maximum_id() {
    let canonical = OperationId::new(u64::MAX);
    let reference = OperationRef::new(canonical);

    assert_eq!(reference.id(), canonical);
}

// =============================================================================
// Dependency identity regressions
// =============================================================================

/// Regression guard:
///
/// A dependency must preserve its source endpoint, destination endpoint, and
/// stable dependency identity.
#[test]
fn regression_dependency_preserves_identity_and_endpoints() {
    let from = operation(10);
    let to = operation(20);
    let dependency_id = DependencyId::new(30);

    let dependency = DependencyRef::new(
        dependency_id,
        from,
        to,
        DependencyKind::Explicit,
    );

    assert_eq!(dependency.id(), dependency_id);
    assert_eq!(dependency.from(), from);
    assert_eq!(dependency.to(), to);
    assert_eq!(dependency.kind(), DependencyKind::Explicit);
}

// =============================================================================
// Graph construction regressions
// =============================================================================

/// Regression guard:
///
/// Registering the same operation twice must fail rather than silently
/// overwriting or merging the operation.
#[test]
fn regression_duplicate_operation_is_rejected() {
    let operation = operation(1);
    let mut graph = DependencyGraph::new();

    graph
        .add_operation(operation)
        .expect("first operation insertion must succeed");

    let result = graph.add_operation(operation);

    assert!(matches!(
        result,
        Err(DependencyGraphError::DuplicateOperation {
            operation: duplicate,
        }) if duplicate == operation
    ));
}

/// Regression guard:
///
/// A rejected duplicate insertion must not corrupt the graph.
#[test]
fn regression_duplicate_operation_does_not_corrupt_graph() {
    let first = operation(1);
    let second = operation(2);

    let mut graph = graph_with_operations(&[first, second]);

    let result = graph.add_operation(first);

    assert!(matches!(
        result,
        Err(DependencyGraphError::DuplicateOperation { .. })
    ));

    let order = graph
        .topological_order()
        .expect("graph must remain a valid DAG");

    assert_eq!(
        order,
        vec![first, second],
        "rejected mutation must leave the graph unchanged"
    );
}

/// Regression guard:
///
/// Duplicate operation detection must also work inside atomic bulk insertion.
#[test]
fn regression_bulk_duplicate_operation_is_rejected() {
    let first = operation(1);
    let second = operation(2);

    let mut graph = graph_with_operations(&[first]);

    let result = graph.add_operations([second, first]);

    assert!(matches!(
        result,
        Err(DependencyGraphError::DuplicateOperation {
            operation,
        }) if operation == first
    ));

    let order = graph
        .topological_order()
        .expect("failed bulk insertion must leave graph valid");

    assert_eq!(order, vec![first]);
}

/// Regression guard:
///
/// If an atomic bulk operation insertion fails, no earlier element from that
/// attempted transaction may remain inserted.
#[test]
fn regression_bulk_operation_insertion_is_atomic() {
    let existing = operation(1);
    let new_operation = operation(2);
    let duplicate = operation(1);

    let mut graph = graph_with_operations(&[existing]);

    let result = graph.add_operations([
        new_operation,
        duplicate,
    ]);

    assert!(result.is_err());

    let order = graph
        .topological_order()
        .expect("graph must remain valid");

    assert_eq!(
        order,
        vec![existing],
        "failed atomic insertion must not partially commit"
    );

    assert!(
        !order.contains(&new_operation),
        "new operation must not leak from failed transaction"
    );
}

// =============================================================================
// Dependency insertion regressions
// =============================================================================

/// Regression guard:
///
/// Dependencies cannot refer to an operation that has not been registered.
#[test]
fn regression_unknown_dependency_source_is_rejected() {
    let known = operation(1);
    let unknown = operation(2);

    let mut graph = graph_with_operations(&[known]);

    let result = graph.add_dependency(
        dependency(1, unknown, known),
    );

    assert!(matches!(
        result,
        Err(DependencyGraphError::UnknownOperation {
            operation,
            dependency: Some(id),
        }) if operation == unknown && id == DependencyId::new(1)
    ));
}

/// Regression guard for an unknown destination.
#[test]
fn regression_unknown_dependency_destination_is_rejected() {
    let known = operation(1);
    let unknown = operation(2);

    let mut graph = graph_with_operations(&[known]);

    let result = graph.add_dependency(
        dependency(1, known, unknown),
    );

    assert!(matches!(
        result,
        Err(DependencyGraphError::UnknownOperation {
            operation,
            dependency: Some(id),
        }) if operation == unknown && id == DependencyId::new(1)
    ));
}

/// Regression guard:
///
/// A self dependency cannot be used to manufacture a cycle-like graph.
#[test]
fn regression_self_dependency_is_rejected() {
    let operation = operation(7);
    let mut graph = graph_with_operations(&[operation]);

    let result = graph.add_dependency(
        dependency(1, operation, operation),
    );

    assert!(matches!(
        result,
        Err(DependencyGraphError::SelfDependency {
            operation: rejected,
        }) if rejected == operation
    ));

    assert_eq!(
        graph
            .topological_order()
            .expect("rejected self dependency must not corrupt graph"),
        vec![operation]
    );
}

/// Regression guard:
///
/// Dependency identity, rather than `(from, to)`, is the authoritative edge
/// identity. Multiple distinct dependencies between the same operations are
/// therefore legal.
#[test]
fn regression_parallel_dependency_edges_remain_distinct() {
    let from = operation(1);
    let to = operation(2);

    let mut graph = graph_with_operations(&[from, to]);

    graph
        .add_dependency(dependency(10, from, to))
        .expect("first dependency must succeed");

    graph
        .add_dependency(dependency(11, from, to))
        .expect("second dependency with distinct identity must succeed");

    let order = graph
        .topological_order()
        .expect("parallel dependency edges must not create a cycle");

    assert_eq!(order, vec![from, to]);
}

/// Regression guard:
///
/// Reusing a dependency ID must be rejected even when its endpoints differ.
#[test]
fn regression_duplicate_dependency_id_is_rejected() {
    let first = operation(1);
    let second = operation(2);
    let third = operation(3);

    let mut graph = graph_with_operations(&[
        first,
        second,
        third,
    ]);

    let first_dependency = dependency(
        100,
        first,
        second,
    );

    graph
        .add_dependency(first_dependency)
        .expect("first dependency must succeed");

    let result = graph.add_dependency(
        dependency(100, second, third),
    );

    assert!(matches!(
        result,
        Err(DependencyGraphError::DuplicateDependency {
            dependency,
        }) if dependency == DependencyId::new(100)
    ));
}

/// Regression guard:
///
/// Failed dependency insertion must not partially mutate the graph.
#[test]
fn regression_rejected_dependency_does_not_corrupt_graph() {
    let first = operation(1);
    let second = operation(2);

    let mut graph = graph_with_operations(&[
        first,
        second,
    ]);

    let result = graph.add_dependency(
        dependency(1, first, operation(999)),
    );

    assert!(result.is_err());

    let order = graph
        .topological_order()
        .expect("failed dependency insertion must leave graph valid");

    assert_eq!(
        order,
        vec![first, second],
        "failed mutation must leave graph unchanged"
    );
}

// =============================================================================
// Cycle regressions
// =============================================================================

/// Regression guard:
///
/// A simple two-node cycle must never be returned as a valid topological
/// ordering.
#[test]
fn regression_two_node_cycle_is_rejected() {
    let first = operation(1);
    let second = operation(2);

    let mut graph = graph_with_operations(&[
        first,
        second,
    ]);

    graph
        .add_dependency(dependency(1, first, second))
        .expect("first edge must succeed");

    graph
        .add_dependency(dependency(2, second, first))
        .expect("second edge must be accepted into the graph");

    let result = graph.topological_order();

    assert!(matches!(
        result,
        Err(DependencyGraphError::CycleDetected { .. })
    ));
}

/// Regression guard for a longer cycle.
///
/// This catches implementations that only detect direct two-node cycles.
#[test]
fn regression_multi_node_cycle_is_rejected() {
    let first = operation(1);
    let second = operation(2);
    let third = operation(3);
    let fourth = operation(4);

    let mut graph = graph_with_operations(&[
        first,
        second,
        third,
        fourth,
    ]);

    graph
        .add_dependency(dependency(1, first, second))
        .expect("edge 1 must succeed");

    graph
        .add_dependency(dependency(2, second, third))
        .expect("edge 2 must succeed");

    graph
        .add_dependency(dependency(3, third, fourth))
        .expect("edge 3 must succeed");

    graph
        .add_dependency(dependency(4, fourth, first))
        .expect("edge 4 must succeed");

    let result = graph.topological_order();

    assert!(matches!(
        result,
        Err(DependencyGraphError::CycleDetected { .. })
    ));
}

// =============================================================================
// Topological-order regressions
// =============================================================================

/// Regression guard:
///
/// A dependency `A -> B` must always appear in that order.
#[test]
fn regression_topological_order_preserves_dependency_direction() {
    let first = operation(1);
    let second = operation(2);
    let third = operation(3);

    let mut graph = graph_with_operations(&[
        first,
        second,
        third,
    ]);

    graph
        .add_dependency(dependency(1, first, second))
        .expect("edge must succeed");

    graph
        .add_dependency(dependency(2, second, third))
        .expect("edge must succeed");

    let order = graph
        .topological_order()
        .expect("acyclic graph must have topological order");

    let first_position = order
        .iter()
        .position(|operation| *operation == first)
        .expect("first operation must be present");

    let second_position = order
        .iter()
        .position(|operation| *operation == second)
        .expect("second operation must be present");

    let third_position = order
        .iter()
        .position(|operation| *operation == third)
        .expect("third operation must be present");

    assert!(first_position < second_position);
    assert!(second_position < third_position);
}

/// Regression guard:
///
/// An unconstrained operation must remain schedulable and must not fabricate a
/// dependency with another operation merely because both exist.
#[test]
fn regression_unconstrained_operations_remain_independent() {
    let first = operation(1);
    let second = operation(2);
    let third = operation(3);

    let graph = graph_with_operations(&[
        first,
        second,
        third,
    ]);

    let order = graph
        .topological_order()
        .expect("independent operations form a DAG");

    assert_eq!(
        order,
        vec![first, second, third],
        "deterministic graph ordering must not invent dependencies"
    );
}

/// Regression guard:
///
/// Deterministic graph ordering must not depend on insertion order.
#[test]
fn regression_topological_order_is_insertion_order_independent() {
    let first = operation(1);
    let second = operation(2);
    let third = operation(3);
    let fourth = operation(4);

    let mut forward = DependencyGraph::new();

    forward
        .add_operations([
            first,
            second,
            third,
            fourth,
        ])
        .expect("forward insertion must succeed");

    let mut reverse = DependencyGraph::new();

    reverse
        .add_operations([
            fourth,
            third,
            second,
            first,
        ])
        .expect("reverse insertion must succeed");

    let forward_order = forward
        .topological_order()
        .expect("forward graph must be acyclic");

    let reverse_order = reverse
        .topological_order()
        .expect("reverse graph must be acyclic");

    assert_eq!(
        forward_order,
        reverse_order,
        "canonical deterministic graph ordering must not depend on insertion order"
    );
}

// =============================================================================
// Atomic dependency insertion regressions
// =============================================================================

/// Regression guard:
///
/// Bulk dependency insertion must be transactional. If one dependency is
/// invalid, none of the dependencies in that transaction may be committed.
#[test]
fn regression_bulk_dependency_insertion_is_atomic() {
    let first = operation(1);
    let second = operation(2);
    let third = operation(3);

    let mut graph = graph_with_operations(&[
        first,
        second,
        third,
    ]);

    let valid = dependency(
        1,
        first,
        second,
    );

    let invalid = dependency(
        2,
        second,
        operation(999),
    );

    let result = graph.add_dependencies([
        valid,
        invalid,
    ]);

    assert!(result.is_err());

    let order = graph
        .topological_order()
        .expect("failed bulk dependency insertion must preserve DAG validity");

    assert_eq!(
        order,
        vec![first, second, third],
        "valid dependency must not leak from failed transaction"
    );
}

// =============================================================================
// Large representable identity regressions
// =============================================================================

/// Regression guard:
///
/// Scheduler identity arithmetic must not silently wrap at `u64::MAX`.
#[test]
fn regression_dependency_id_does_not_wrap() {
    let last = DependencyId::new(u64::MAX);

    assert_eq!(last.checked_next(), None);
}

/// Regression guard:
///
/// The largest representable dependency identity remains a valid identity.
#[test]
fn regression_maximum_dependency_id_remains_valid() {
    let dependency_id = DependencyId::new(u64::MAX);

    assert_eq!(dependency_id.value(), u64::MAX);
    assert!(!dependency_id.is_zero());
}

/// Regression guard:
///
/// A large identity immediately below the maximum can still advance safely.
#[test]
fn regression_dependency_id_near_maximum_advances_once() {
    let dependency_id = DependencyId::new(u64::MAX - 1);

    let next = dependency_id
        .checked_next()
        .expect("u64::MAX - 1 must have a representable successor");

    assert_eq!(
        next,
        DependencyId::new(u64::MAX)
    );

    assert!(
        next > dependency_id,
        "checked successor must remain strictly increasing"
    );

    assert_eq!(next.checked_next(), None);
}

// =============================================================================
// Time arithmetic regressions
// =============================================================================

/// Regression guard:
///
/// Zero duration must be an additive identity for scheduling time.
#[test]
fn regression_zero_duration_does_not_move_time() {
    let points = [
        TimePoint::new(0),
        TimePoint::new(1),
        TimePoint::new(42),
        TimePoint::new(u64::MAX as u128),
        TimePoint::new(u128::MAX),
    ];

    for point in points {
        assert_eq!(
            point.checked_add(Duration::ZERO),
            Some(point)
        );
    }
}

/// Regression guard:
///
/// Adding a representable duration and then subtracting the same duration must
/// recover the original time point.
#[test]
fn regression_time_addition_and_subtraction_are_inverse() {
    let cases = [
        (0u128, 0u128),
        (0, 1),
        (1, 1),
        (10, 20),
        (u64::MAX as u128, 17),
        (u128::MAX - 100, 100),
    ];

    for (start_value, duration_value) in cases {
        let start = TimePoint::new(start_value);
        let duration = Duration::new(duration_value);

        let Some(end) = start.checked_add(duration) else {
            continue;
        };

        assert_eq!(
            end.checked_sub(duration),
            Some(start),
            "checked time arithmetic must be reversible"
        );
    }
}

/// Regression guard:
///
/// Time arithmetic must reject overflow rather than wrapping to an earlier
/// point.
#[test]
fn regression_time_addition_never_wraps() {
    let start = TimePoint::new(u128::MAX);
    let duration = Duration::new(1);

    assert_eq!(
        start.checked_add(duration),
        None,
        "time overflow must be rejected"
    );
}

/// Regression guard:
///
/// Duration arithmetic must reject overflow rather than wrapping.
#[test]
fn regression_duration_addition_never_wraps() {
    let duration = Duration::new(u128::MAX);

    assert_eq!(
        duration.checked_add(Duration::new(1)),
        None,
        "duration overflow must be rejected"
    );
}

/// Regression guard:
///
/// Time subtraction must reject underflow rather than producing a wrapped
/// timestamp.
#[test]
fn regression_time_subtraction_never_wraps() {
    let start = TimePoint::new(0);
    let duration = Duration::new(1);

    assert_eq!(
        start.checked_sub(duration),
        None,
        "time underflow must be rejected"
    );
}

/// Regression guard:
///
/// A duration cannot become negative through checked subtraction.
#[test]
fn regression_duration_subtraction_never_wraps() {
    let smaller = Duration::new(1);
    let larger = Duration::new(2);

    assert_eq!(
        smaller.checked_sub(larger),
        None,
        "duration underflow must be rejected"
    );
}

/// Regression guard:
///
/// The duration between two ordered time points must equal their coordinate
/// difference.
#[test]
fn regression_time_interval_round_trip_is_exact() {
    let start = TimePoint::new(100);
    let end = TimePoint::new(175);

    let duration = start
        .checked_duration_until(end)
        .expect("end after start must have a representable duration");

    assert_eq!(duration, Duration::new(75));

    assert_eq!(
        start.checked_add(duration),
        Some(end)
    );
}

/// Regression guard:
///
/// Asking for the duration from a later point to an earlier point must fail
/// rather than wrap.
#[test]
fn regression_reverse_time_interval_is_rejected() {
    let start = TimePoint::new(100);
    let end = TimePoint::new(99);

    assert_eq!(
        start.checked_duration_until(end),
        None
    );
}

// =============================================================================
// Deterministic graph regression
// =============================================================================

/// Regression guard:
///
/// Canonical graph traversal must be reproducible across independent graph
/// instances.
#[test]
fn regression_equivalent_graphs_have_identical_topological_order() {
    let operations = [
        operation(1),
        operation(2),
        operation(3),
        operation(4),
        operation(5),
        operation(6),
    ];

    let mut first = graph_with_operations(&operations);
    let mut second = graph_with_operations(&operations);

    first
        .add_dependencies([
            dependency(10, operations[0], operations[2]),
            dependency(11, operations[1], operations[2]),
            dependency(12, operations[2], operations[4]),
            dependency(13, operations[3], operations[4]),
            dependency(14, operations[4], operations[5]),
        ])
        .expect("first graph dependencies must succeed");

    second
        .add_dependencies([
            dependency(14, operations[4], operations[5]),
            dependency(12, operations[2], operations[4]),
            dependency(10, operations[0], operations[2]),
            dependency(13, operations[3], operations[4]),
            dependency(11, operations[1], operations[2]),
        ])
        .expect("second graph dependencies must succeed");

    let first_order = first
        .topological_order()
        .expect("first graph must be acyclic");

    let second_order = second
        .topological_order()
        .expect("second graph must be acyclic");

    assert_eq!(
        first_order,
        second_order,
        "equivalent dependency graphs must have reproducible ordering"
    );
}

// =============================================================================
// Sparse / scalable graph regression
// =============================================================================

/// Regression guard:
///
/// The scheduler dependency representation must work for sparse graphs without
/// requiring a dense matrix or time-slot array.
///
/// This fixture intentionally grows linearly and only creates a chain of
/// dependencies. It therefore exercises the representation used for very
/// sparse programs without encoding any production capacity limit.
#[test]
fn regression_sparse_dependency_chain_remains_schedulable() {
    const NODE_COUNT: usize = 128;

    let operations: Vec<OperationRef> = (0..NODE_COUNT)
        .map(|index| operation(index as u64))
        .collect();

    let mut graph = graph_with_operations(&operations);

    for index in 0..NODE_COUNT.saturating_sub(1) {
        graph
            .add_dependency(
                dependency(
                    index as u64,
                    operations[index],
                    operations[index + 1],
                ),
            )
            .expect("sparse chain dependency must succeed");
    }

    let order = graph
        .topological_order()
        .expect("sparse acyclic chain must be schedulable");

    assert_eq!(
        order.len(),
        NODE_COUNT,
        "every registered operation must appear exactly once"
    );

    for index in 0..NODE_COUNT {
        assert_eq!(
            order[index],
            operations[index],
            "chain ordering must preserve every dependency"
        );
    }
}

/// Regression guard:
///
/// A wide graph must not accidentally acquire dependencies merely because
/// operations share the same scheduling context.
///
/// This is particularly important for large parallel quantum programs.
#[test]
fn regression_wide_independent_graph_remains_parallelizable() {
    const NODE_COUNT: usize = 128;

    let operations: Vec<OperationRef> = (0..NODE_COUNT)
        .map(|index| operation(index as u64))
        .collect();

    let graph = graph_with_operations(&operations);

    let order = graph
        .topological_order()
        .expect("independent graph must be acyclic");

    assert_eq!(order.len(), NODE_COUNT);

    let unique: BTreeSet<OperationRef> =
        order.iter().copied().collect();

    assert_eq!(
        unique.len(),
        NODE_COUNT,
        "topological order must not duplicate operations"
    );
}

// =============================================================================
// Graph semantic consistency regressions
// =============================================================================

/// Regression guard:
///
/// Every operation returned by topological ordering must be an operation that
/// was actually registered.
#[test]
fn regression_topological_order_contains_only_registered_operations() {
    let operations = [
        operation(10),
        operation(20),
        operation(30),
        operation(40),
    ];

    let graph = graph_with_operations(&operations);

    let order = graph
        .topological_order()
        .expect("graph must be acyclic");

    let registered: BTreeSet<_> =
        operations.into_iter().collect();

    for operation in order {
        assert!(
            registered.contains(&operation),
            "topological traversal returned an unregistered operation"
        );
    }
}

/// Regression guard:
///
/// Topological order must contain every registered operation exactly once.
#[test]
fn regression_topological_order_is_a_permutation_of_nodes() {
    let operations = [
        operation(7),
        operation(3),
        operation(100),
        operation(2),
        operation(55),
    ];

    let graph = graph_with_operations(&operations);

    let order = graph
        .topological_order()
        .expect("graph must be acyclic");

    let expected: BTreeSet<_> =
        operations.into_iter().collect();

    let actual: BTreeSet<_> =
        order.iter().copied().collect();

    assert_eq!(actual, expected);
    assert_eq!(
        order.len(),
        expected.len(),
        "an operation must occur exactly once in topological order"
    );
}

// =============================================================================
// Transactional mutation regression
// =============================================================================

/// Regression guard:
///
/// If a bulk operation mutation contains a duplicate that already exists,
/// the graph must remain byte-for-byte equivalent at the semantic level:
/// same node set and same topological order.
#[test]
fn regression_failed_bulk_mutation_preserves_semantic_graph_state() {
    let first = operation(1);
    let second = operation(2);
    let third = operation(3);

    let mut graph = graph_with_operations(&[
        first,
        second,
        third,
    ]);

    graph
        .add_dependency(dependency(1, first, second))
        .expect("initial dependency must succeed");

    let before = graph
        .topological_order()
        .expect("initial graph must be valid");

    let result = graph.add_dependencies([
        dependency(2, second, third),
        dependency(3, third, operation(999)),
    ]);

    assert!(result.is_err());

    let after = graph
        .topological_order()
        .expect("failed transaction must preserve validity");

    assert_eq!(
        before,
        after,
        "failed bulk mutation must preserve semantic graph state"
    );
}

// =============================================================================
// Regression contract summary
// =============================================================================

/// This test intentionally documents the invariants pinned by this module.
///
/// It has no production behavior. Its purpose is to make the regression
/// contract explicit for maintainers and reviewers.
#[test]
fn regression_contract_is_explicit() {
    let required_invariants = [
        "canonical qubit identity",
        "canonical operation identity",
        "unique operation identity",
        "unique dependency identity",
        "registered dependency endpoints",
        "no self dependencies",
        "cycle detection",
        "deterministic topological ordering",
        "atomic graph mutation",
        "checked scheduler identifier arithmetic",
        "checked time arithmetic",
        "zero-duration identity",
        "sparse graph scalability",
        "wide graph independence",
    ];

    assert!(
        !required_invariants.is_empty(),
        "regression contract must remain non-empty"
    );
}