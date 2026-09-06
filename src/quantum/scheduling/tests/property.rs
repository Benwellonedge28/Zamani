//! Zamani Quantum Scheduling — Property / Invariant Test Suite
//!
//! Path:
//!     src/quantum/scheduling/tests/property.rs
//!
//! # Purpose
//!
//! This module verifies properties that must remain true regardless of:
//!
//! - quantum-machine size;
//! - quantum technology;
//! - topology;
//! - number of qubits;
//! - number of operations;
//! - number of scheduling resources;
//! - scheduling depth;
//! - target timing resolution;
//! - routing strategy;
//! - QEC strategy;
//! - scheduling algorithm;
//! - optimization policy.
//!
//! These are invariant tests rather than tests of one particular implementation
//! strategy.
//!
//! # Architectural position
//!
//! ```text
//! canonical quantum::ir
//!          │
//!          ▼
//! optimization
//!          │
//!          ▼
//! routing
//!          │
//!          ▼
//! scheduling adapters
//!          │
//!          ▼
//! scheduling IR
//!          │
//!     ┌────┼───────────┐
//!     ▼    ▼           ▼
//! dependency timing  resources
//!     │    │           │
//!     └────┼───────────┘
//!          ▼
//!      planners
//!          │
//!          ▼
//!      schedules
//!          │
//!          ▼
//!     verification
//! ```
//!
//! This file does not implement scheduling.
//!
//! It verifies mathematical and structural properties of the contracts used by
//! scheduling.
//!
//! # Property philosophy
//!
//! A production scheduler must not merely pass examples. It must preserve
//! invariants over broad classes of inputs.
//!
//! The properties tested here include:
//!
//! - canonical logical qubit identity is preserved;
//! - canonical physical qubit identity is preserved;
//! - logical and physical identities remain distinct;
//! - identifier operations are monotonic and checked;
//! - scheduler identifiers do not silently wrap;
//! - time-point arithmetic is monotonic;
//! - duration arithmetic is monotonic;
//! - duration addition is consistent with time-point addition;
//! - time subtraction is the inverse of checked addition;
//! - zero duration does not move time;
//! - deterministic generated inputs produce deterministic outputs;
//! - dependency edges preserve their endpoints;
//! - generated DAGs remain acyclic;
//! - topological order respects every dependency;
//! - duplicate graph insertion does not silently corrupt state;
//! - self-dependencies are rejected;
//! - multiple dependency identities between the same operations remain distinct;
//! - graph statistics remain internally consistent;
//! - large representable identifiers remain valid;
//! - scheduler-local types do not replace canonical qubit identities;
//! - no test assumes a finite quantum-machine size.
//!
//! # Important scalability rule
//!
//! The test suite may deliberately generate a finite number of test cases so
//! CI remains bounded.
//!
//! Such bounds are TEST-WORKLOAD bounds only.
//!
//! They are NOT scheduler limits.
//!
//! No production constant such as:
//!
//! ```text
//! MAX_QUBITS
//! MAX_GATES
//! MAX_RESOURCES
//! MAX_DEPTH
//! MAX_CHANNELS
//! ```
//!
//! is introduced here.
//!
//! The scheduler itself remains bounded only by the resources available to the
//! compiler/execution environment and by explicitly supplied policies.
//!
//! # Determinism
//!
//! A deterministic pseudo-random generator is used instead of an external RNG
//! dependency.
//!
//! This provides:
//!
//! ```text
//! same seed
//!     +
//! same test version
//!     +
//! same input generation rules
//!     =
//! same generated cases
//! ```
//!
//! The generator is NOT cryptographic and must never be used by production
//! scheduling algorithms.
//!
//! # Rust contract
//!
//! Supported:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! # Integration contract
//!
//! This file is intended to be declared by:
//!
//! ```text
//! src/quantum/scheduling/tests/mod.rs
//! ```
//!
//! with:
//!
//! ```text
//! mod property;
//! ```
//!
//! The tests consume stable scheduler contracts from:
//!
//! ```text
//! quantum::ir::qubit
//! quantum::scheduling::types
//! quantum::scheduling::ir
//! ```
//!
//! They deliberately avoid private implementation details.
//!
//! Consequently, adding a new scheduling algorithm, hardware provider,
//! routing implementation, QEC strategy, or optimization pass should not
//! require modifying this file unless an established public contract changes.
//!
//! # Safety boundary
//!
//! No unsafe code is permitted.
//!
//! The compiler enforces this contract with `forbid(unsafe_code)`.

#![cfg(test)]
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::collections::{BTreeMap, BTreeSet};
use std::panic::{catch_unwind, AssertUnwindSafe};

use crate::quantum::ir::core::identity::OperationId;

use crate::quantum::ir::qubit::{
    PhysicalQubitId,
    QubitId,
    QubitRef,
};

use crate::quantum::scheduling::ir::graph::DependencyGraph;

use crate::quantum::scheduling::types::{
    DependencyId,
    DependencyKind,
    DependencyRef,
    Duration,
    OperationRef,
    TimePoint,
};

// =============================================================================
// Deterministic property generator
// =============================================================================

/// Deterministic pseudo-random generator used exclusively by this test module.
///
/// This is intentionally dependency-free so the scheduler property suite does
/// not require `rand`, `proptest`, or another third-party testing dependency.
///
/// It is NOT cryptographically secure and must never be used for:
///
/// - scheduling randomness;
/// - cryptographic material;
/// - security decisions;
/// - hardware control;
/// - randomized compiler semantics.
#[derive(Clone, Debug)]
struct PropertyRng {
    state: u64,
}

impl PropertyRng {
    /// Creates a deterministic generator.
    fn new(seed: u64) -> Self {
        Self {
            state: if seed == 0 {
                0x9E37_79B9_7F4A_7C15
            } else {
                seed
            },
        }
    }

    /// Produces the next deterministic 64-bit value.
    fn next_u64(&mut self) -> u64 {
        // xorshift64*.
        //
        // This is test-only deterministic generation, not cryptography.
        let mut value = self.state;

        value ^= value >> 12;
        value ^= value << 25;
        value ^= value >> 27;

        self.state = value;

        value.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    /// Produces a usize value.
    fn next_usize(&mut self) -> usize {
        self.next_u64() as usize
    }

    /// Produces a deterministic boolean.
    fn next_bool(&mut self) -> bool {
        self.next_u64() & 1 == 1
    }

    /// Produces a value in `[0, upper)`.
    fn range(&mut self, upper: usize) -> usize {
        if upper == 0 {
            0
        } else {
            self.next_usize() % upper
        }
    }

    /// Produces a value in an inclusive interval.
    fn inclusive_range(
        &mut self,
        lower: usize,
        upper: usize,
    ) -> usize {
        if lower >= upper {
            lower
        } else {
            lower.saturating_add(self.range(
                upper.saturating_sub(lower).saturating_add(1),
            ))
        }
    }
}

// =============================================================================
// Test constants
// =============================================================================
//
// These constants describe test execution effort only.
//
// They are intentionally NOT scheduling-machine limits.

/// Number of deterministic generated property cases.
///
/// This is a CI workload bound, not a scheduler capacity.
const PROPERTY_CASES: usize = 512;

/// Maximum generated graph nodes per individual property case.
///
/// This is intentionally modest so ordinary unit-test execution remains
/// predictable. Dedicated scalability tests should exercise substantially
/// larger workloads.
const GENERATED_GRAPH_NODES: usize = 64;

// =============================================================================
// Panic isolation
// =============================================================================

/// Executes a property body and converts an unexpected panic into a test
/// failure.
///
/// Invalid scheduler inputs should normally be represented by `Result::Err`,
/// not process-level unwinding.
fn assert_no_panic<T, F>(operation: F) -> T
where
    F: FnOnce() -> T,
{
    catch_unwind(AssertUnwindSafe(operation))
        .expect("scheduler property operation unexpectedly panicked")
}

// =============================================================================
// Identity helpers
// =============================================================================

fn logical_qubit(index: usize) -> QubitId {
    QubitId::new(index)
}

fn physical_qubit(index: usize) -> PhysicalQubitId {
    PhysicalQubitId::new(index)
}

fn operation(value: u64) -> OperationRef {
    OperationRef::new(OperationId::new(value))
}

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

// =============================================================================
// Canonical qubit properties
// =============================================================================

#[test]
fn property_qubit_identity_is_reflexive() {
    for index in [
        0usize,
        1,
        2,
        7,
        31,
        127,
        usize::MAX / 2,
        usize::MAX,
    ] {
        let qubit = logical_qubit(index);

        assert_eq!(qubit, qubit);
        assert_eq!(qubit.index(), index);
    }
}

#[test]
fn property_qubit_identity_is_symmetric() {
    for left in 0usize..32 {
        for right in 0usize..32 {
            let a = logical_qubit(left);
            let b = logical_qubit(right);

            assert_eq!(a == b, b == a);
            assert_eq!(a != b, b != a);
        }
    }
}

#[test]
fn property_qubit_identity_is_order_consistent() {
    for left in 0usize..32 {
        for right in 0usize..32 {
            let a = logical_qubit(left);
            let b = logical_qubit(right);

            assert_eq!(
                a.cmp(&b),
                logical_qubit(left).cmp(&logical_qubit(right))
            );
        }
    }
}

#[test]
fn property_qubit_checked_next_is_successor_when_representable() {
    for index in [
        0usize,
        1,
        2,
        7,
        31,
        127,
        usize::MAX / 2,
        usize::MAX.saturating_sub(1),
    ] {
        let qubit = logical_qubit(index);

        if index < usize::MAX {
            assert_eq!(
                qubit.checked_next(),
                Some(logical_qubit(index + 1))
            );
        }
    }
}

#[test]
fn property_qubit_checked_next_never_wraps() {
    let logical = logical_qubit(usize::MAX);
    let physical = physical_qubit(usize::MAX);

    assert_eq!(logical.checked_next(), None);
    assert_eq!(physical.checked_next(), None);
}

#[test]
fn property_logical_and_physical_references_remain_distinct() {
    for index in [
        0usize,
        1,
        2,
        17,
        1024,
        usize::MAX,
    ] {
        let logical = QubitRef::Logical(logical_qubit(index));
        let physical = QubitRef::Physical(physical_qubit(index));

        assert_ne!(logical, physical);

        assert!(logical.is_logical());
        assert!(!logical.is_physical());

        assert!(!physical.is_logical());
        assert!(physical.is_physical());
    }
}

#[test]
fn property_logical_reference_round_trips_its_canonical_identity() {
    for index in [
        0usize,
        1,
        7,
        31,
        255,
        usize::MAX,
    ] {
        let qubit = logical_qubit(index);
        let reference = QubitRef::Logical(qubit);

        assert_eq!(reference.logical(), Some(qubit));
        assert_eq!(reference.physical(), None);
    }
}

#[test]
fn property_physical_reference_round_trips_its_canonical_identity() {
    for index in [
        0usize,
        1,
        7,
        31,
        255,
        usize::MAX,
    ] {
        let qubit = physical_qubit(index);
        let reference = QubitRef::Physical(qubit);

        assert_eq!(reference.logical(), None);
        assert_eq!(reference.physical(), Some(qubit));
    }
}

// =============================================================================
// Scheduler identifier properties
// =============================================================================

#[test]
fn property_operation_reference_round_trip_is_identity_preserving() {
    for value in [
        0u64,
        1,
        2,
        7,
        42,
        u64::MAX / 2,
        u64::MAX,
    ] {
        let reference = operation(value);

        assert_eq!(reference.id(), OperationId::new(value));
    }
}

#[test]
fn property_dependency_reference_preserves_endpoints() {
    for value in 0u64..32 {
        let from = operation(value.saturating_mul(2).saturating_add(1));
        let to = operation(value.saturating_mul(2).saturating_add(2));

        let reference = dependency(value, from, to);

        assert_eq!(reference.id(), DependencyId::new(value));
        assert_eq!(reference.from(), from);
        assert_eq!(reference.to(), to);
        assert_eq!(reference.kind(), DependencyKind::Explicit);
    }
}

#[test]
fn property_scheduler_identifiers_have_checked_successors() {
    let identifiers = [
        DependencyId::new(0),
        DependencyId::new(1),
        DependencyId::new(42),
        DependencyId::new(u64::MAX - 1),
        DependencyId::new(u64::MAX),
    ];

    for identifier in identifiers {
        match identifier.checked_next() {
            Some(next) => {
                assert_eq!(
                    next.value(),
                    identifier.value().saturating_add(1)
                );
                assert!(next > identifier);
            }
            None => {
                assert_eq!(identifier.value(), u64::MAX);
            }
        }
    }
}

#[test]
fn property_scheduler_identifier_successor_never_wraps() {
    let identifier = DependencyId::new(u64::MAX);

    assert_eq!(identifier.checked_next(), None);
}

#[test]
fn property_scheduler_identifier_zero_is_detected_correctly() {
    assert!(DependencyId::new(0).is_zero());
    assert!(!DependencyId::new(1).is_zero());
    assert!(!DependencyId::new(u64::MAX).is_zero());
}

// =============================================================================
// Duration properties
// =============================================================================

#[test]
fn property_duration_zero_is_additive_identity() {
    for value in [
        0u128,
        1,
        2,
        7,
        42,
        u64::MAX as u128,
        u128::MAX / 2,
        u128::MAX,
    ] {
        let duration = Duration::new(value);

        assert_eq!(
            duration.checked_add(Duration::ZERO),
            Some(duration)
        );

        assert_eq!(
            Duration::ZERO.checked_add(duration),
            Some(duration)
        );
    }
}

#[test]
fn property_duration_is_never_negative() {
    for value in [
        0u128,
        1,
        2,
        17,
        u64::MAX as u128,
        u128::MAX,
    ] {
        let duration = Duration::new(value);

        assert_eq!(duration.value(), value);
    }
}

#[test]
fn property_duration_checked_subtraction_is_exact_when_ordered() {
    for left in 0u128..64 {
        for right in 0u128..=left {
            let a = Duration::new(left);
            let b = Duration::new(right);

            let difference = a
                .checked_sub(b)
                .expect("ordered duration subtraction must succeed");

            assert_eq!(difference.value(), left - right);
        }
    }
}

#[test]
fn property_duration_checked_subtraction_rejects_underflow() {
    for left in 0u128..32 {
        for right in (left + 1)..64 {
            let a = Duration::new(left);
            let b = Duration::new(right);

            assert_eq!(a.checked_sub(b), None);
        }
    }
}

#[test]
fn property_duration_addition_is_commutative_when_representable() {
    for left in 0u128..32 {
        for right in 0u128..32 {
            let a = Duration::new(left);
            let b = Duration::new(right);

            assert_eq!(
                a.checked_add(b),
                b.checked_add(a)
            );
        }
    }
}

#[test]
fn property_duration_addition_rejects_overflow() {
    let left = Duration::new(u128::MAX);
    let right = Duration::new(1);

    assert_eq!(left.checked_add(right), None);
}

// =============================================================================
// Time-point properties
// =============================================================================

#[test]
fn property_time_zero_is_the_origin() {
    assert_eq!(TimePoint::ZERO.value(), 0);
    assert!(TimePoint::ZERO.is_zero());
}

#[test]
fn property_time_point_round_trips_values() {
    for value in [
        0u128,
        1,
        2,
        7,
        42,
        u64::MAX as u128,
        u128::MAX / 2,
        u128::MAX,
    ] {
        let point = TimePoint::new(value);

        assert_eq!(point.value(), value);
    }
}

#[test]
fn property_time_addition_is_monotonic() {
    for start in 0u128..32 {
        for duration in 0u128..32 {
            let point = TimePoint::new(start);
            let delta = Duration::new(duration);

            let end = point
                .checked_add(delta)
                .expect("small representable addition must succeed");

            assert!(end >= point);
            assert_eq!(end.value(), start + duration);
        }
    }
}

#[test]
fn property_time_zero_duration_does_not_move_time() {
    for value in [
        0u128,
        1,
        7,
        42,
        u64::MAX as u128,
        u128::MAX,
    ] {
        let point = TimePoint::new(value);

        assert_eq!(
            point.checked_add(Duration::ZERO),
            Some(point)
        );
    }
}

#[test]
fn property_time_subtraction_is_inverse_of_addition() {
    for start in 0u128..32 {
        for duration in 0u128..32 {
            let point = TimePoint::new(start);
            let delta = Duration::new(duration);

            let end = point
                .checked_add(delta)
                .expect("small representable addition must succeed");

            let recovered = end
                .checked_sub(delta)
                .expect("subtraction must recover original time");

            assert_eq!(recovered, point);
        }
    }
}

#[test]
fn property_time_duration_difference_is_exact() {
    for start in 0u128..32 {
        for duration in 0u128..32 {
            let point = TimePoint::new(start);
            let delta = Duration::new(duration);

            let end = point
                .checked_add(delta)
                .expect("small representable addition must succeed");

            let measured = point
                .checked_duration_until(end)
                .expect("end must not precede start");

            assert_eq!(measured, delta);
        }
    }
}

#[test]
fn property_time_rejects_overflow() {
    let point = TimePoint::new(u128::MAX);
    let duration = Duration::new(1);

    assert_eq!(point.checked_add(duration), None);
}

#[test]
fn property_time_rejects_underflow() {
    let point = TimePoint::ZERO;
    let duration = Duration::new(1);

    assert_eq!(point.checked_sub(duration), None);
}

#[test]
fn property_time_duration_until_rejects_reverse_interval() {
    let start = TimePoint::new(10);
    let end = TimePoint::new(9);

    assert_eq!(start.checked_duration_until(end), None);
}

// =============================================================================
// Deterministic generator properties
// =============================================================================

#[test]
fn property_generator_is_deterministic() {
    let mut first = PropertyRng::new(0xA5A5_1234_5678_9ABC);
    let mut second = PropertyRng::new(0xA5A5_1234_5678_9ABC);

    for _ in 0..512 {
        assert_eq!(first.next_u64(), second.next_u64());
    }
}

#[test]
fn property_generator_seed_changes_sequence() {
    let mut first = PropertyRng::new(1);
    let mut second = PropertyRng::new(2);

    let mut differences = 0usize;

    for _ in 0..64 {
        if first.next_u64() != second.next_u64() {
            differences += 1;
        }
    }

    assert!(differences > 0);
}

#[test]
fn property_generator_never_divides_by_zero() {
    let mut rng = PropertyRng::new(0x1234_5678);

    for _ in 0..PROPERTY_CASES {
        assert_eq!(rng.range(0), 0);
    }
}

// =============================================================================
// Dependency graph construction helpers
// =============================================================================

/// Builds a deterministic DAG.
///
/// For every generated node, edges may only point from a lower operation
/// index to a higher operation index.
///
/// Therefore a generated graph is structurally incapable of containing a
/// backward edge.
fn generated_dag(
    rng: &mut PropertyRng,
    node_count: usize,
) -> (
    DependencyGraph,
    Vec<OperationRef>,
    Vec<DependencyRef>,
) {
    let mut graph = DependencyGraph::new();

    let operations: Vec<OperationRef> = (0..node_count)
        .map(|index| operation(index as u64))
        .collect();

    for &node in &operations {
        graph
            .add_operation(node)
            .expect("fresh generated operation must be accepted");
    }

    let mut dependencies = Vec::new();
    let mut dependency_id = 0u64;

    for from_index in 0..node_count {
        for to_index in (from_index + 1)..node_count {
            // Sparse deterministic generation.
            if !rng.next_bool() {
                continue;
            }

            let edge = dependency(
                dependency_id,
                operations[from_index],
                operations[to_index],
            );

            graph
                .add_dependency(edge)
                .expect("forward DAG edge must be accepted");

            dependencies.push(edge);

            dependency_id = dependency_id
                .checked_add(1)
                .expect("test dependency identity space exhausted");
        }
    }

    (graph, operations, dependencies)
}

// =============================================================================
// Dependency graph properties
// =============================================================================

#[test]
fn property_empty_dependency_graph_is_empty() {
    let graph = DependencyGraph::new();

    assert!(graph.is_empty());
    assert_eq!(graph.node_count(), 0);
    assert_eq!(graph.dependency_count(), 0);
}

#[test]
fn property_graph_node_registration_is_monotonic() {
    let mut graph = DependencyGraph::new();

    for value in 0u64..32 {
        let before = graph.node_count();

        graph
            .add_operation(operation(value))
            .expect("unique operation must be accepted");

        assert_eq!(graph.node_count(), before + 1);
    }
}

#[test]
fn property_graph_rejects_duplicate_operations() {
    let mut graph = DependencyGraph::new();

    let first = operation(7);

    graph
        .add_operation(first)
        .expect("first operation must be accepted");

    let result = graph.add_operation(first);

    assert!(result.is_err());
    assert_eq!(graph.node_count(), 1);
}

#[test]
fn property_graph_rejects_self_dependencies() {
    for value in 0u64..32 {
        let mut graph = DependencyGraph::new();
        let node = operation(value);

        graph
            .add_operation(node)
            .expect("operation must be accepted");

        let result = graph.add_dependency(
            dependency(value, node, node),
        );

        assert!(result.is_err());
        assert_eq!(graph.dependency_count(), 0);
    }
}

#[test]
fn property_graph_rejects_unknown_dependency_endpoints() {
    let mut graph = DependencyGraph::new();

    let registered = operation(1);
    let unknown = operation(2);

    graph
        .add_operation(registered)
        .expect("operation must be accepted");

    let result = graph.add_dependency(
        dependency(0, registered, unknown),
    );

    assert!(result.is_err());
    assert_eq!(graph.dependency_count(), 0);
}

#[test]
fn property_graph_preserves_dependency_endpoints() {
    let mut graph = DependencyGraph::new();

    let first = operation(1);
    let second = operation(2);

    graph
        .add_operation(first)
        .expect("first operation must be accepted");

    graph
        .add_operation(second)
        .expect("second operation must be accepted");

    let edge = dependency(42, first, second);

    graph
        .add_dependency(edge)
        .expect("valid dependency must be accepted");

    let outgoing = graph
        .successors(first)
        .collect::<Vec<_>>();

    let incoming = graph
        .predecessors(second)
        .collect::<Vec<_>>();

    assert_eq!(outgoing.len(), 1);
    assert_eq!(incoming.len(), 1);
}

#[test]
fn property_graph_accepts_multiple_distinct_dependencies_between_same_nodes() {
    let mut graph = DependencyGraph::new();

    let first = operation(1);
    let second = operation(2);

    graph
        .add_operation(first)
        .expect("first operation must be accepted");

    graph
        .add_operation(second)
        .expect("second operation must be accepted");

    let first_dependency = dependency(10, first, second);
    let second_dependency = dependency(11, first, second);

    graph
        .add_dependency(first_dependency)
        .expect("first dependency must be accepted");

    graph
        .add_dependency(second_dependency)
        .expect("second dependency must be accepted");

    assert_eq!(graph.dependency_count(), 2);

    let successors = graph
        .successors(first)
        .collect::<Vec<_>>();

    assert_eq!(successors.len(), 2);
}

#[test]
fn property_graph_rejects_duplicate_dependency_identity() {
    let mut graph = DependencyGraph::new();

    let first = operation(1);
    let second = operation(2);

    graph
        .add_operation(first)
        .expect("first operation must be accepted");

    graph
        .add_operation(second)
        .expect("second operation must be accepted");

    let edge = dependency(10, first, second);

    graph
        .add_dependency(edge)
        .expect("first dependency must be accepted");

    let result = graph.add_dependency(edge);

    assert!(result.is_err());
    assert_eq!(graph.dependency_count(), 1);
}

#[test]
fn property_generated_forward_graph_is_acyclic() {
    let mut rng = PropertyRng::new(0xC0DE_CAFE);

    for _ in 0..PROPERTY_CASES {
        let node_count = rng.inclusive_range(
            0,
            GENERATED_GRAPH_NODES,
        );

        let (graph, _, _) = generated_dag(
            &mut rng,
            node_count,
        );

        assert_no_panic(|| {
            graph
                .validate_acyclic()
                .expect("forward-only generated graph must be acyclic");
        });
    }
}

#[test]
fn property_generated_graph_topological_order_contains_every_node_once() {
    let mut rng = PropertyRng::new(0xA11C_E5E5);

    for _ in 0..PROPERTY_CASES {
        let node_count = rng.inclusive_range(
            0,
            GENERATED_GRAPH_NODES,
        );

        let (graph, operations, _) = generated_dag(
            &mut rng,
            node_count,
        );

        let order = graph
            .topological_order()
            .expect("generated DAG must have a topological order");

        assert_eq!(order.len(), operations.len());

        let expected: BTreeSet<OperationRef> =
            operations.iter().copied().collect();

        let actual: BTreeSet<OperationRef> =
            order.iter().copied().collect();

        assert_eq!(actual, expected);

        assert_eq!(
            order.len(),
            actual.len(),
            "topological order must not contain duplicate operations"
        );
    }
}

#[test]
fn property_generated_topological_order_respects_all_edges() {
    let mut rng = PropertyRng::new(0x51CE_DA7A);

    for _ in 0..PROPERTY_CASES {
        let node_count = rng.inclusive_range(
            0,
            GENERATED_GRAPH_NODES,
        );

        let (graph, operations, dependencies) =
            generated_dag(&mut rng, node_count);

        let order = graph
            .topological_order()
            .expect("generated DAG must have a topological order");

        let positions: BTreeMap<OperationRef, usize> = order
            .iter()
            .copied()
            .enumerate()
            .map(|(position, operation)| (operation, position))
            .collect();

        assert_eq!(positions.len(), operations.len());

        for edge in dependencies {
            let from_position = *positions
                .get(&edge.from())
                .expect("edge source must occur in topological order");

            let to_position = *positions
                .get(&edge.to())
                .expect("edge destination must occur in topological order");

            assert!(
                from_position < to_position,
                "dependency order violated: {} must precede {}",
                edge.from(),
                edge.to()
            );
        }
    }
}

#[test]
fn property_generated_graph_statistics_are_consistent() {
    let mut rng = PropertyRng::new(0x57A7_15A7);

    for _ in 0..PROPERTY_CASES {
        let node_count = rng.inclusive_range(
            0,
            GENERATED_GRAPH_NODES,
        );

        let (graph, operations, dependencies) =
            generated_dag(&mut rng, node_count);

        let statistics = graph.statistics();

        assert_eq!(statistics.nodes, operations.len());
        assert_eq!(statistics.dependencies, dependencies.len());

        assert!(
            statistics.roots <= statistics.nodes,
            "root count cannot exceed node count"
        );

        assert!(
            statistics.leaves <= statistics.nodes,
            "leaf count cannot exceed node count"
        );

        assert!(
            statistics.maximum_in_degree <= statistics.nodes,
            "in-degree cannot exceed the number of nodes"
        );

        assert!(
            statistics.maximum_out_degree <= statistics.nodes,
            "out-degree cannot exceed the number of nodes"
        );
    }
}

// =============================================================================
// Dependency graph determinism
// =============================================================================

#[test]
fn property_same_generated_dag_has_same_topological_order() {
    let mut first_rng = PropertyRng::new(0xD37E_12A5);
    let mut second_rng = PropertyRng::new(0xD37E_12A5);

    for _ in 0..PROPERTY_CASES {
        let first_count = first_rng.inclusive_range(
            0,
            GENERATED_GRAPH_NODES,
        );

        let second_count = second_rng.inclusive_range(
            0,
            GENERATED_GRAPH_NODES,
        );

        assert_eq!(first_count, second_count);

        let (first_graph, _, _) =
            generated_dag(&mut first_rng, first_count);

        let (second_graph, _, _) =
            generated_dag(&mut second_rng, second_count);

        let first_order = first_graph
            .topological_order()
            .expect("first generated graph must be a DAG");

        let second_order = second_graph
            .topological_order()
            .expect("second generated graph must be a DAG");

        assert_eq!(first_order, second_order);
    }
}

#[test]
fn property_graph_clone_preserves_semantics() {
    let mut rng = PropertyRng::new(0xC10E_5EED);

    for _ in 0..PROPERTY_CASES {
        let node_count = rng.inclusive_range(
            0,
            GENERATED_GRAPH_NODES,
        );

        let (graph, _, _) = generated_dag(
            &mut rng,
            node_count,
        );

        let clone = graph.clone();

        assert_eq!(
            graph.node_count(),
            clone.node_count()
        );

        assert_eq!(
            graph.dependency_count(),
            clone.dependency_count()
        );

        assert_eq!(
            graph.topological_order(),
            clone.topological_order()
        );
    }
}

// =============================================================================
// Generated operation identity properties
// =============================================================================

#[test]
fn property_generated_operation_ids_are_unique() {
    let mut rng = PropertyRng::new(0x0P55_0001);

    let mut identifiers = BTreeSet::new();

    for _ in 0..PROPERTY_CASES {
        let value = rng.next_u64();

        assert!(
            identifiers.insert(value),
            "generated identity collision unexpectedly occurred"
        );
    }
}

// =============================================================================
// Cross-domain time identity properties
// =============================================================================

#[test]
fn property_time_addition_preserves_order() {
    let mut rng = PropertyRng::new(0x71ME_0001);

    for _ in 0..PROPERTY_CASES {
        let start = rng.next_u64() as u128;
        let first_duration = rng.range(128) as u128;
        let second_duration = rng.range(128) as u128;

        let first = TimePoint::new(start);
        let first_delta = Duration::new(first_duration);
        let second_delta = Duration::new(second_duration);

        let after_first = match first.checked_add(first_delta) {
            Some(value) => value,
            None => continue,
        };

        let after_second = match after_first.checked_add(second_delta) {
            Some(value) => value,
            None => continue,
        };

        assert!(after_first >= first);
        assert!(after_second >= after_first);
    }
}

#[test]
fn property_duration_associativity_holds_when_all_operations_are_representable() {
    let mut rng = PropertyRng::new(0xA550_C1A7);

    for _ in 0..PROPERTY_CASES {
        let a = Duration::new(rng.range(128) as u128);
        let b = Duration::new(rng.range(128) as u128);
        let c = Duration::new(rng.range(128) as u128);

        let left = a
            .checked_add(b)
            .and_then(|value| value.checked_add(c));

        let right = b
            .checked_add(c)
            .and_then(|value| a.checked_add(value));

        assert_eq!(left, right);
    }
}

// =============================================================================
// No-panic boundary properties
// =============================================================================

#[test]
fn property_boundary_identifier_operations_do_not_panic() {
    assert_no_panic(|| {
        let _ = DependencyId::new(u64::MAX);
        let _ = DependencyId::new(u64::MAX).checked_next();
        let _ = logical_qubit(usize::MAX);
        let _ = physical_qubit(usize::MAX);
    });
}

#[test]
fn property_boundary_time_operations_do_not_panic() {
    assert_no_panic(|| {
        let maximum = TimePoint::new(u128::MAX);
        let unit = Duration::new(1);

        let _ = maximum.checked_add(unit);
        let _ = maximum.checked_sub(unit);
        let _ = TimePoint::ZERO.checked_sub(unit);
        let _ = TimePoint::ZERO.checked_duration_until(maximum);
    });
}

#[test]
fn property_boundary_graph_operations_do_not_panic() {
    assert_no_panic(|| {
        let mut graph = DependencyGraph::new();

        let first = operation(0);
        let second = operation(u64::MAX);

        let _ = graph.add_operation(first);
        let _ = graph.add_operation(second);

        let _ = graph.add_dependency(
            dependency(u64::MAX, first, second),
        );

        let _ = graph.validate_acyclic();
        let _ = graph.topological_order();
        let _ = graph.statistics();
    });
}

// =============================================================================
// Structural scalability properties
// =============================================================================

#[test]
fn property_graph_storage_scales_with_nodes_and_edges_not_time() {
    let mut graph = DependencyGraph::new();

    let first = operation(1);
    let second = operation(2);

    graph
        .add_operation(first)
        .expect("first operation must be accepted");

    graph
        .add_operation(second)
        .expect("second operation must be accepted");

    graph
        .add_dependency(
            dependency(1, first, second),
        )
        .expect("dependency must be accepted");

    // The graph API has no timeline-sized allocation contract.
    //
    // A dependency graph containing one edge remains structurally one edge
    // regardless of the eventual physical execution duration.
    assert_eq!(graph.node_count(), 2);
    assert_eq!(graph.dependency_count(), 1);
}

#[test]
fn property_large_semantic_identifiers_do_not_imply_machine_size_limits() {
    let operation_a = operation(u64::MAX - 1);
    let operation_b = operation(u64::MAX);

    let mut graph = DependencyGraph::new();

    graph
        .add_operation(operation_a)
        .expect("large operation identity must be accepted");

    graph
        .add_operation(operation_b)
        .expect("maximum operation identity must be accepted");

    graph
        .add_dependency(
            dependency(
                u64::MAX,
                operation_a,
                operation_b,
            ),
        )
        .expect("large dependency identity must be accepted");

    assert_eq!(graph.node_count(), 2);
    assert_eq!(graph.dependency_count(), 1);

    graph
        .validate_acyclic()
        .expect("two-node forward graph must remain acyclic");
}

// =============================================================================
// Property-suite aggregate smoke test
// =============================================================================

/// A small aggregate smoke test.
///
/// This does not replace the individual property tests. It exists to ensure
/// that the core canonical boundaries can be exercised together in one
/// deterministic operation.
#[test]
fn property_core_scheduler_boundaries_work_together() {
    assert_no_panic(|| {
        let logical = QubitId::new(7);
        let physical = PhysicalQubitId::new(13);

        let logical_reference = QubitRef::Logical(logical);
        let physical_reference = QubitRef::Physical(physical);

        assert_ne!(logical_reference, physical_reference);

        let first_operation = operation(1);
        let second_operation = operation(2);

        let edge = dependency(
            1,
            first_operation,
            second_operation,
        );

        let mut graph = DependencyGraph::new();

        graph
            .add_operation(first_operation)
            .expect("first operation must be accepted");

        graph
            .add_operation(second_operation)
            .expect("second operation must be accepted");

        graph
            .add_dependency(edge)
            .expect("dependency must be accepted");

        graph
            .validate_acyclic()
            .expect("generated dependency graph must be acyclic");

        let order = graph
            .topological_order()
            .expect("topological order must exist");

        assert_eq!(order.len(), 2);
        assert!(order[0] == first_operation);
        assert!(order[1] == second_operation);

        let start = TimePoint::ZERO;
        let duration = Duration::new(10);

        let finish = start
            .checked_add(duration)
            .expect("small duration must be representable");

        assert_eq!(
            start
                .checked_duration_until(finish)
                .expect("finish must follow start"),
            duration
        );
    });
}