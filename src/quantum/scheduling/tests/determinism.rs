//! Zamani Quantum Scheduling — Determinism Test Suite
//!
//! Path:
//!     src/quantum/scheduling/tests/determinism.rs
//!
//! # Purpose
//!
//! This module verifies that the scheduling subsystem produces reproducible
//! results when the semantic input is unchanged.
//!
//! Determinism is a production requirement for:
//!
//! - reproducible compilation;
//! - reproducible debugging;
//! - stable regression tests;
//! - schedule provenance;
//! - cache keys;
//! - distributed compilation;
//! - deterministic serialization;
//! - compiler comparison;
//! - auditability;
//! - scientific benchmarking.
//!
//! # Scope
//!
//! This file tests deterministic scheduling infrastructure rather than a
//! particular scheduling heuristic.
//!
//! It deliberately does NOT require a particular planner, resource model,
//! hardware backend, routing implementation, QEC implementation, or runtime.
//!
//! The fundamental contract tested here is:
//!
//! ```text
//! same semantic input
//! + same target snapshot
//! + same scheduling configuration
//! + same deterministic policy
//! = same observable scheduling result
//! ```
//!
//! For the dependency graph specifically:
//!
//! ```text
//! same operations
//! + same dependencies
//! = same canonical graph ordering
//! ```
//!
//! # Architectural position
//!
//! ```text
//!                    Zamani Program
//!                          │
//!                          ▼
//!                    quantum::ir
//!                          │
//!                          ▼
//!                     optimization
//!                          │
//!                          ▼
//!                       routing
//!                          │
//!                          ▼
//!                  scheduling::adapters
//!                          │
//!                          ▼
//!                  scheduling::ir
//!                          │
//!                ┌─────────┼─────────┐
//!                ▼         ▼         ▼
//!           dependency   timing   resources
//!                │         │         │
//!                └─────────┼─────────┘
//!                          ▼
//!                       planners
//!                          │
//!                          ▼
//!                    schedule result
//!                          │
//!                          ▼
//!                      verification
//! ```
//!
//! This module sits beside the other scheduling test suites. It must not
//! become a second implementation of scheduling.
//!
//! # Determinism model
//!
//! Determinism is divided into four layers:
//!
//! 1. deterministic test-data generation;
//! 2. deterministic graph construction;
//! 3. deterministic graph traversal;
//! 4. deterministic repeated observation.
//!
//! The test generator is intentionally local to this module. It is NOT a
//! production scheduler RNG and must never be reused by production code.
//!
//! # No hard-coded machine limits
//!
//! This module does NOT impose limits on:
//!
//! - qubits;
//! - operations;
//! - dependencies;
//! - graph depth;
//! - graph width;
//! - resources;
//! - channels;
//! - hardware size;
//! - topology;
//! - execution time.
//!
//! Any finite number of generated test cases is a test-run workload bound,
//! not a scheduling capacity.
//!
//! The production scheduler remains bounded only by:
//!
//! - available memory;
//! - available computation;
//! - explicit caller policy;
//! - explicit scheduler limits;
//! - target-provided resources.
//!
//! # Canonical identity boundary
//!
//! Qubit identity MUST come from:
//!
//! ```text
//! crate::quantum::ir::qubit
//! ```
//!
//! This test module therefore does not define another qubit identifier.
//!
//! Scheduling operation identity likewise comes from the existing canonical
//! scheduling/IR identity contracts.
//!
//! # Integration
//!
//! Declare this module from:
//!
//! ```text
//! src/quantum/scheduling/tests/mod.rs
//! ```
//!
//! using:
//!
//! ```rust
//! mod determinism;
//! ```
//!
//! The tests intentionally use public scheduling contracts only.
//!
//! Adding:
//!
//! - a new scheduler;
//! - a new hardware provider;
//! - a new routing algorithm;
//! - a new QEC implementation;
//! - a new optimization pass;
//! - a new resource type;
//!
//! must not require changing this file unless an established public contract
//! itself changes.
//!
//! # Rust
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
//! # Safety
//!
//! No unsafe code is permitted.
//!
//! The compiler enforces this requirement with `forbid(unsafe_code)`.
//!
//! # Important distinction
//!
//! Determinism does NOT mean every possible scheduling strategy must produce
//! the same schedule.
//!
//! Different explicit policies may legitimately produce different schedules:
//!
//! ```text
//! ASAP
//! ALAP
//! critical-path
//! resource-aware
//! fidelity-aware
//! multi-objective
//! ```
//!
//! This suite instead verifies that a given deterministic configuration does
//! not accidentally produce different results because of:
//!
//! - hash-map iteration order;
//! - insertion order;
//! - thread scheduling;
//! - hidden randomness;
//! - unstable graph traversal;
//! - clone differences;
//! - repeated execution.
//!
//! # Failure philosophy
//!
//! A determinism failure is a correctness failure for a deterministic mode.
//!
//! It must not be "fixed" by sorting the final answer after the fact if the
//! underlying scheduling decisions are already nondeterministic.
//!
//! Determinism must originate from explicit ordering and explicit policy.
//!
//! # Test independence
//!
//! Every test constructs its own state.
//!
//! No global mutable scheduler state is used.
//!
//! No test depends on another test running first.
//!
//! No wall-clock timing is used as a correctness criterion.
//!
//! No thread-count assumption is made.

#![cfg(test)]
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::collections::{BTreeMap, BTreeSet};

use crate::quantum::ir::core::identity::OperationId;

use crate::quantum::ir::qubit::QubitId;

use crate::quantum::scheduling::ir::graph::DependencyGraph;

use crate::quantum::scheduling::types::{
    DependencyId,
    DependencyKind,
    DependencyRef,
    OperationRef,
};

// =============================================================================
// Deterministic test generator
// =============================================================================

/// Dependency-free deterministic generator used exclusively by this test
/// module.
///
/// This generator is deliberately not part of the scheduling implementation.
/// Its only purpose is to create repeatable test workloads.
///
/// The generator is not cryptographically secure and must never be used for:
///
/// - scheduling decisions;
/// - security;
/// - cryptographic material;
/// - hardware control;
/// - production randomness.
#[derive(Clone, Debug, PartialEq, Eq)]
struct DeterministicGenerator {
    state: u64,
}

impl DeterministicGenerator {
    /// Creates a generator from an explicit seed.
    ///
    /// A zero seed is mapped to a non-zero internal state so that the
    /// generator never becomes permanently stuck at zero.
    #[must_use]
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
    ///
    /// This is xorshift64*-style test-only generation.
    fn next_u64(&mut self) -> u64 {
        let mut value = self.state;

        value ^= value >> 12;
        value ^= value << 25;
        value ^= value >> 27;

        self.state = value;

        value.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    /// Produces a deterministic boolean.
    fn next_bool(&mut self) -> bool {
        self.next_u64() & 1 == 1
    }

    /// Produces a deterministic value in `[0, upper)`.
    fn range(&mut self, upper: usize) -> usize {
        if upper == 0 {
            0
        } else {
            (self.next_u64() as usize) % upper
        }
    }

    /// Produces a deterministic value in the inclusive range
    /// `[lower, upper]`.
    fn inclusive_range(
        &mut self,
        lower: usize,
        upper: usize,
    ) -> usize {
        if lower >= upper {
            lower
        } else {
            lower.saturating_add(
                self.range(
                    upper
                        .saturating_sub(lower)
                        .saturating_add(1),
                ),
            )
        }
    }
}

// =============================================================================
// Test workload configuration
// =============================================================================
//
// These are test execution parameters only.
//
// They are NOT scheduler limits.
//
// In particular, nothing here establishes:
//
// MAX_QUBITS
// MAX_GATES
// MAX_RESOURCES
// MAX_DEPTH
// MAX_CHANNELS
//
// Production scalability is determined by the scheduler's explicit context,
// policies, target resources, and available machine resources.

/// Number of repeated deterministic cases.
///
/// This controls CI/test effort only.
const DETERMINISM_CASES: usize = 256;

/// Maximum number of operations generated in one ordinary determinism case.
///
/// This is deliberately a test workload size, not a scheduler capacity.
const GENERATED_OPERATIONS: usize = 64;

// =============================================================================
// Identity helpers
// =============================================================================

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

fn canonical_qubit(value: usize) -> QubitId {
    QubitId::new(value)
}

// =============================================================================
// Deterministic graph fixture
// =============================================================================

/// A complete deterministic description of a scheduling dependency graph.
///
/// Keeping the semantic fixture separate from `DependencyGraph` is important:
/// it allows the same semantic program to be inserted into the graph through
/// different orders and verifies that insertion order cannot affect canonical
/// traversal.
#[derive(Clone, Debug, PartialEq, Eq)]
struct GraphFixture {
    operations: Vec<OperationRef>,
    dependencies: Vec<DependencyRef>,
}

impl GraphFixture {
    /// Builds a deterministic acyclic fixture.
    ///
    /// Edges are generated only from a lower operation index to a higher
    /// operation index. Therefore the generated graph is structurally a DAG.
    #[must_use]
    fn generate(
        generator: &mut DeterministicGenerator,
        operation_count: usize,
    ) -> Self {
        let operations: Vec<OperationRef> = (0..operation_count)
            .map(|index| operation(index as u64))
            .collect();

        let mut dependencies = Vec::new();
        let mut dependency_id = 0u64;

        for from_index in 0..operation_count {
            for to_index in
                from_index.saturating_add(1)..operation_count
            {
                if !generator.next_bool() {
                    continue;
                }

                let edge = dependency(
                    dependency_id,
                    operations[from_index],
                    operations[to_index],
                );

                dependencies.push(edge);

                dependency_id = dependency_id
                    .checked_add(1)
                    .expect(
                        "test dependency identity space exhausted",
                    );
            }
        }

        Self {
            operations,
            dependencies,
        }
    }

    /// Builds a graph using canonical insertion order.
    fn build_canonical(&self) -> DependencyGraph {
        let mut graph = DependencyGraph::new();

        for &operation in &self.operations {
            graph
                .add_operation(operation)
                .expect("fixture operation must be unique");
        }

        for &dependency in &self.dependencies {
            graph
                .add_dependency(dependency)
                .expect("fixture dependency must be valid");
        }

        graph
    }

    /// Builds a graph with operations and dependencies inserted in reverse
    /// order.
    ///
    /// A deterministic graph must not depend on insertion order.
    fn build_reverse(&self) -> DependencyGraph {
        let mut graph = DependencyGraph::new();

        for &operation in self.operations.iter().rev() {
            graph
                .add_operation(operation)
                .expect("fixture operation must be unique");
        }

        for &dependency in self.dependencies.iter().rev() {
            graph
                .add_dependency(dependency)
                .expect("fixture dependency must be valid");
        }

        graph
    }

    /// Builds a graph with operations and dependencies inserted according to
    /// two independent deterministic permutations.
    fn build_permuted(
        &self,
        operation_order: &[usize],
        dependency_order: &[usize],
    ) -> DependencyGraph {
        assert_eq!(
            operation_order.len(),
            self.operations.len()
        );

        assert_eq!(
            dependency_order.len(),
            self.dependencies.len()
        );

        let mut graph = DependencyGraph::new();

        for &index in operation_order {
            let operation = self.operations[index];

            graph
                .add_operation(operation)
                .expect("permuted operation must be unique");
        }

        for &index in dependency_order {
            let dependency = self.dependencies[index];

            graph
                .add_dependency(dependency)
                .expect("permuted dependency must be valid");
        }

        graph
    }
}

// =============================================================================
// Canonical graph observation
// =============================================================================

/// Captures the externally observable deterministic properties of a graph.
///
/// This deliberately avoids private implementation details.
#[derive(Clone, Debug, PartialEq, Eq)]
struct GraphObservation {
    node_count: usize,
    dependency_count: usize,
    topological_order: Vec<OperationRef>,
    statistics: (
        usize,
        usize,
        usize,
        usize,
        usize,
    ),
}

fn observe(graph: &DependencyGraph) -> GraphObservation {
    let statistics = graph.statistics();

    GraphObservation {
        node_count: graph.node_count(),
        dependency_count: graph.dependency_count(),
        topological_order: graph
            .topological_order()
            .expect("deterministic fixture must be acyclic"),
        statistics: (
            statistics.nodes,
            statistics.dependencies,
            statistics.roots,
            statistics.leaves,
            statistics.maximum_in_degree
                .saturating_add(statistics.maximum_out_degree),
        ),
    }
}

// =============================================================================
// Permutation helpers
// =============================================================================

/// Produces a deterministic permutation of indices.
///
/// The implementation is deliberately independent of graph storage.
fn deterministic_permutation(
    length: usize,
    generator: &mut DeterministicGenerator,
) -> Vec<usize> {
    let mut values: Vec<usize> = (0..length).collect();

    if length < 2 {
        return values;
    }

    for index in (1..length).rev() {
        let selected = generator.range(index.saturating_add(1));

        values.swap(index, selected);
    }

    values
}

// =============================================================================
// Deterministic generator tests
// =============================================================================

#[test]
fn determinism_generator_same_seed_produces_same_sequence() {
    let mut first =
        DeterministicGenerator::new(0xD37E_12A5_1234_5678);

    let mut second =
        DeterministicGenerator::new(0xD37E_12A5_1234_5678);

    for _ in 0..1024 {
        assert_eq!(
            first.next_u64(),
            second.next_u64()
        );
    }
}

#[test]
fn determinism_generator_clone_preserves_future_sequence() {
    let mut original =
        DeterministicGenerator::new(0xA11C_E5E5_57A7_15A7);

    for _ in 0..64 {
        let _ = original.next_u64();
    }

    let mut clone = original.clone();

    for _ in 0..1024 {
        assert_eq!(
            original.next_u64(),
            clone.next_u64()
        );
    }
}

#[test]
fn determinism_generator_different_seeds_are_not_equivalent() {
    let mut first =
        DeterministicGenerator::new(0x0000_0000_0000_0001);

    let mut second =
        DeterministicGenerator::new(0x0000_0000_0000_0002);

    let mut equal_values = 0usize;

    for _ in 0..128 {
        if first.next_u64() == second.next_u64() {
            equal_values = equal_values.saturating_add(1);
        }
    }

    assert!(
        equal_values < 128,
        "independent seeds unexpectedly produced identical sequences"
    );
}

#[test]
fn determinism_zero_seed_is_stable() {
    let mut first = DeterministicGenerator::new(0);
    let mut second = DeterministicGenerator::new(0);

    for _ in 0..256 {
        assert_eq!(
            first.next_u64(),
            second.next_u64()
        );
    }
}

#[test]
fn determinism_range_zero_is_stable() {
    let mut generator =
        DeterministicGenerator::new(0x1234_5678);

    for _ in 0..DETERMINISM_CASES {
        assert_eq!(
            generator.range(0),
            0
        );
    }
}

// =============================================================================
// Canonical qubit determinism
// =============================================================================

#[test]
fn determinism_uses_canonical_qubit_identity() {
    let first = canonical_qubit(7);
    let second = canonical_qubit(7);

    assert_eq!(first, second);
    assert_eq!(first.index(), second.index());
}

#[test]
fn determinism_canonical_qubit_order_is_stable() {
    let first: Vec<QubitId> =
        (0usize..64).map(canonical_qubit).collect();

    let second: Vec<QubitId> =
        (0usize..64).map(canonical_qubit).collect();

    assert_eq!(first, second);
}

#[test]
fn determinism_large_canonical_qubit_identity_does_not_change() {
    let values = [
        0usize,
        1,
        7,
        31,
        255,
        4_096,
        usize::MAX / 2,
        usize::MAX,
    ];

    let first: Vec<QubitId> =
        values.iter().copied().map(canonical_qubit).collect();

    let second: Vec<QubitId> =
        values.iter().copied().map(canonical_qubit).collect();

    assert_eq!(first, second);

    for (expected, qubit) in values.iter().copied().zip(first) {
        assert_eq!(qubit.index(), expected);
    }
}

// =============================================================================
// Fixture determinism
// =============================================================================

#[test]
fn determinism_same_seed_generates_identical_fixtures() {
    let mut first =
        DeterministicGenerator::new(0xCAFE_BABE_1234_5678);

    let mut second =
        DeterministicGenerator::new(0xCAFE_BABE_1234_5678);

    for _ in 0..DETERMINISM_CASES {
        let first_count =
            first.inclusive_range(0, GENERATED_OPERATIONS);

        let second_count =
            second.inclusive_range(0, GENERATED_OPERATIONS);

        assert_eq!(first_count, second_count);

        let first_fixture =
            GraphFixture::generate(&mut first, first_count);

        let second_fixture =
            GraphFixture::generate(&mut second, second_count);

        assert_eq!(first_fixture, second_fixture);
    }
}

#[test]
fn determinism_same_fixture_produces_identical_graph_observation() {
    let mut generator =
        DeterministicGenerator::new(0xF1F1_7E57);

    for _ in 0..DETERMINISM_CASES {
        let count =
            generator.inclusive_range(0, GENERATED_OPERATIONS);

        let fixture =
            GraphFixture::generate(&mut generator, count);

        let first = fixture.build_canonical();
        let second = fixture.build_canonical();

        assert_eq!(
            observe(&first),
            observe(&second)
        );
    }
}

// =============================================================================
// Insertion-order determinism
// =============================================================================

#[test]
fn determinism_insertion_order_does_not_change_graph_observation() {
    let mut generator =
        DeterministicGenerator::new(0x1A2B_3C4D_5E6F_7788);

    for _ in 0..DETERMINISM_CASES {
        let count =
            generator.inclusive_range(0, GENERATED_OPERATIONS);

        let fixture =
            GraphFixture::generate(&mut generator, count);

        let canonical = fixture.build_canonical();
        let reverse = fixture.build_reverse();

        assert_eq!(
            observe(&canonical),
            observe(&reverse)
        );
    }
}

#[test]
fn determinism_permuted_insertion_does_not_change_topological_order() {
    let mut generator =
        DeterministicGenerator::new(0xD15C_0DE5_2026_0001);

    for _ in 0..DETERMINISM_CASES {
        let count =
            generator.inclusive_range(0, GENERATED_OPERATIONS);

        let fixture =
            GraphFixture::generate(&mut generator, count);

        let operation_order =
            deterministic_permutation(
                fixture.operations.len(),
                &mut generator,
            );

        let dependency_order =
            deterministic_permutation(
                fixture.dependencies.len(),
                &mut generator,
            );

        let canonical =
            fixture.build_canonical();

        let permuted =
            fixture.build_permuted(
                &operation_order,
                &dependency_order,
            );

        assert_eq!(
            canonical.topological_order(),
            permuted.topological_order()
        );
    }
}

#[test]
fn determinism_reversed_dependency_insertion_preserves_statistics() {
    let mut generator =
        DeterministicGenerator::new(0xABCD_EF01_2345_6789);

    for _ in 0..DETERMINISM_CASES {
        let count =
            generator.inclusive_range(0, GENERATED_OPERATIONS);

        let fixture =
            GraphFixture::generate(&mut generator, count);

        let canonical =
            fixture.build_canonical();

        let reverse =
            fixture.build_reverse();

        let first =
            canonical.statistics();

        let second =
            reverse.statistics();

        assert_eq!(first.nodes, second.nodes);
        assert_eq!(
            first.dependencies,
            second.dependencies
        );
        assert_eq!(first.roots, second.roots);
        assert_eq!(first.leaves, second.leaves);
        assert_eq!(
            first.maximum_in_degree,
            second.maximum_in_degree
        );
        assert_eq!(
            first.maximum_out_degree,
            second.maximum_out_degree
        );
    }
}

// =============================================================================
// Canonical ordering tests
// =============================================================================

#[test]
fn determinism_topological_order_is_repeatable() {
    let mut generator =
        DeterministicGenerator::new(0x70P0_0001);

    for _ in 0..DETERMINISM_CASES {
        let count =
            generator.inclusive_range(0, GENERATED_OPERATIONS);

        let fixture =
            GraphFixture::generate(&mut generator, count);

        let graph =
            fixture.build_canonical();

        let first =
            graph.topological_order()
                .expect("fixture must be acyclic");

        let second =
            graph.topological_order()
                .expect("fixture must be acyclic");

        assert_eq!(first, second);
    }
}

#[test]
fn determinism_topological_order_is_a_set_preserving_operation_identity() {
    let mut generator =
        DeterministicGenerator::new(0x7A7A_0001);

    for _ in 0..DETERMINISM_CASES {
        let count =
            generator.inclusive_range(0, GENERATED_OPERATIONS);

        let fixture =
            GraphFixture::generate(&mut generator, count);

        let graph =
            fixture.build_canonical();

        let order =
            graph.topological_order()
                .expect("fixture must be acyclic");

        let expected: BTreeSet<OperationRef> =
            fixture.operations.iter().copied().collect();

        let actual: BTreeSet<OperationRef> =
            order.iter().copied().collect();

        assert_eq!(expected, actual);
        assert_eq!(order.len(), actual.len());
    }
}

#[test]
fn determinism_topological_order_has_stable_positions() {
    let mut generator =
        DeterministicGenerator::new(0x5151_0001);

    for _ in 0..DETERMINISM_CASES {
        let count =
            generator.inclusive_range(0, GENERATED_OPERATIONS);

        let fixture =
            GraphFixture::generate(&mut generator, count);

        let graph =
            fixture.build_canonical();

        let first =
            graph.topological_order()
                .expect("fixture must be acyclic");

        let second =
            graph.topological_order()
                .expect("fixture must be acyclic");

        let first_positions: BTreeMap<
            OperationRef,
            usize,
        > = first
            .iter()
            .copied()
            .enumerate()
            .map(|(position, operation)| {
                (operation, position)
            })
            .collect();

        let second_positions: BTreeMap<
            OperationRef,
            usize,
        > = second
            .iter()
            .copied()
            .enumerate()
            .map(|(position, operation)| {
                (operation, position)
            })
            .collect();

        assert_eq!(
            first_positions,
            second_positions
        );
    }
}

// =============================================================================
// Graph validation determinism
// =============================================================================

#[test]
fn determinism_structure_validation_is_repeatable() {
    let mut generator =
        DeterministicGenerator::new(0x57A7_15A7);

    for _ in 0..DETERMINISM_CASES {
        let count =
            generator.inclusive_range(0, GENERATED_OPERATIONS);

        let fixture =
            GraphFixture::generate(&mut generator, count);

        let graph =
            fixture.build_canonical();

        let first =
            graph.validate_structure();

        let second =
            graph.validate_structure();

        assert_eq!(first, second);
        assert!(first.is_ok());
    }
}

#[test]
fn determinism_acyclic_validation_is_repeatable() {
    let mut generator =
        DeterministicGenerator::new(0xAC1C_1C00);

    for _ in 0..DETERMINISM_CASES {
        let count =
            generator.inclusive_range(0, GENERATED_OPERATIONS);

        let fixture =
            GraphFixture::generate(&mut generator, count);

        let graph =
            fixture.build_canonical();

        let first =
            graph.validate_acyclic();

        let second =
            graph.validate_acyclic();

        assert_eq!(first, second);
        assert!(first.is_ok());
    }
}

// =============================================================================
// Clone determinism
// =============================================================================

#[test]
fn determinism_graph_clone_preserves_observable_semantics() {
    let mut generator =
        DeterministicGenerator::new(0xC10E_5EED);

    for _ in 0..DETERMINISM_CASES {
        let count =
            generator.inclusive_range(0, GENERATED_OPERATIONS);

        let fixture =
            GraphFixture::generate(&mut generator, count);

        let original =
            fixture.build_canonical();

        let clone =
            original.clone();

        assert_eq!(
            observe(&original),
            observe(&clone)
        );

        assert_eq!(
            original.validate_structure(),
            clone.validate_structure()
        );

        assert_eq!(
            original.validate_acyclic(),
            clone.validate_acyclic()
        );
    }
}

// =============================================================================
// Empty and singleton determinism
// =============================================================================

#[test]
fn determinism_empty_graph_is_stable() {
    let first = DependencyGraph::new();
    let second = DependencyGraph::new();

    assert_eq!(
        observe(&first),
        observe(&second)
    );

    assert_eq!(
        first.validate_structure(),
        second.validate_structure()
    );

    assert_eq!(
        first.validate_acyclic(),
        second.validate_acyclic()
    );
}

#[test]
fn determinism_single_operation_graph_is_stable() {
    let operation = operation(0);

    let mut first =
        DependencyGraph::new();

    let mut second =
        DependencyGraph::new();

    first
        .add_operation(operation)
        .expect("operation must be accepted");

    second
        .add_operation(operation)
        .expect("operation must be accepted");

    assert_eq!(
        observe(&first),
        observe(&second)
    );

    assert_eq!(
        first.topological_order(),
        second.topological_order()
    );
}

// =============================================================================
// Dependency identity determinism
// =============================================================================

#[test]
fn determinism_dependency_identity_is_stable() {
    let from = operation(1);
    let to = operation(2);

    let first =
        dependency(42, from, to);

    let second =
        dependency(42, from, to);

    assert_eq!(first, second);
    assert_eq!(first.id(), second.id());
    assert_eq!(first.from(), second.from());
    assert_eq!(first.to(), second.to());
    assert_eq!(first.kind(), second.kind());
}

#[test]
fn determinism_multiple_dependencies_keep_distinct_identity() {
    let from = operation(1);
    let to = operation(2);

    let first =
        dependency(100, from, to);

    let second =
        dependency(101, from, to);

    assert_ne!(first, second);
    assert_ne!(first.id(), second.id());

    let mut graph =
        DependencyGraph::new();

    graph
        .add_operation(from)
        .expect("source operation must be accepted");

    graph
        .add_operation(to)
        .expect("destination operation must be accepted");

    graph
        .add_dependency(first)
        .expect("first dependency must be accepted");

    graph
        .add_dependency(second)
        .expect("second dependency must be accepted");

    assert_eq!(
        graph.dependency_count(),
        2
    );
}

// =============================================================================
// Cycle diagnostic determinism
// =============================================================================

#[test]
fn determinism_cycle_detection_is_repeatable() {
    let first_error = {
        let one = operation(1);
        let two = operation(2);
        let three = operation(3);

        let mut graph =
            DependencyGraph::new();

        graph
            .add_operation(one)
            .expect("operation must be accepted");

        graph
            .add_operation(two)
            .expect("operation must be accepted");

        graph
            .add_operation(three)
            .expect("operation must be accepted");

        graph
            .add_dependency(
                dependency(1, one, two),
            )
            .expect("edge must be accepted");

        graph
            .add_dependency(
                dependency(2, two, three),
            )
            .expect("edge must be accepted");

        graph
            .add_dependency(
                dependency(3, three, one),
            )
            .expect("edge must be accepted");

        graph
            .validate_acyclic()
            .expect_err("cycle must be rejected")
    };

    let second_error = {
        let one = operation(1);
        let two = operation(2);
        let three = operation(3);

        let mut graph =
            DependencyGraph::new();

        graph
            .add_operation(one)
            .expect("operation must be accepted");

        graph
            .add_operation(two)
            .expect("operation must be accepted");

        graph
            .add_operation(three)
            .expect("operation must be accepted");

        graph
            .add_dependency(
                dependency(1, one, two),
            )
            .expect("edge must be accepted");

        graph
            .add_dependency(
                dependency(2, two, three),
            )
            .expect("edge must be accepted");

        graph
            .add_dependency(
                dependency(3, three, one),
            )
            .expect("edge must be accepted");

        graph
            .validate_acyclic()
            .expect_err("cycle must be rejected")
    };

    assert_eq!(first_error, second_error);
}

// =============================================================================
// Cross-construction determinism
// =============================================================================

#[test]
fn determinism_canonical_and_permuted_graphs_are_semantically_equal() {
    let mut generator =
        DeterministicGenerator::new(0xD3D3_2026);

    for _ in 0..DETERMINISM_CASES {
        let count =
            generator.inclusive_range(0, GENERATED_OPERATIONS);

        let fixture =
            GraphFixture::generate(&mut generator, count);

        let operation_order =
            deterministic_permutation(
                fixture.operations.len(),
                &mut generator,
            );

        let dependency_order =
            deterministic_permutation(
                fixture.dependencies.len(),
                &mut generator,
            );

        let canonical =
            fixture.build_canonical();

        let permuted =
            fixture.build_permuted(
                &operation_order,
                &dependency_order,
            );

        assert_eq!(
            canonical.node_count(),
            permuted.node_count()
        );

        assert_eq!(
            canonical.dependency_count(),
            permuted.dependency_count()
        );

        assert_eq!(
            canonical.topological_order(),
            permuted.topological_order()
        );

        assert_eq!(
            canonical.statistics().nodes,
            permuted.statistics().nodes
        );

        assert_eq!(
            canonical.statistics().dependencies,
            permuted.statistics().dependencies
        );

        assert_eq!(
            canonical.statistics().roots,
            permuted.statistics().roots
        );

        assert_eq!(
            canonical.statistics().leaves,
            permuted.statistics().leaves
        );

        assert_eq!(
            canonical.statistics().maximum_in_degree,
            permuted.statistics().maximum_in_degree
        );

        assert_eq!(
            canonical.statistics().maximum_out_degree,
            permuted.statistics().maximum_out_degree
        );
    }
}

// =============================================================================
// Repeated-observation determinism
// =============================================================================

#[test]
fn determinism_repeated_observation_is_stable() {
    let mut generator =
        DeterministicGenerator::new(0x4242_2026);

    for _ in 0..DETERMINISM_CASES {
        let count =
            generator.inclusive_range(0, GENERATED_OPERATIONS);

        let fixture =
            GraphFixture::generate(&mut generator, count);

        let graph =
            fixture.build_canonical();

        let baseline =
            observe(&graph);

        for _ in 0..8 {
            assert_eq!(
                baseline,
                observe(&graph)
            );
        }
    }
}

// =============================================================================
// Deterministic graph growth
// =============================================================================

#[test]
fn determinism_graph_prefixes_have_stable_semantics() {
    let mut generator =
        DeterministicGenerator::new(0x5052_4546_4958);

    let maximum =
        GENERATED_OPERATIONS;

    let mut previous_order =
        Vec::<OperationRef>::new();

    for count in 0..=maximum {
        let fixture =
            GraphFixture::generate(
                &mut generator,
                count,
            );

        let graph =
            fixture.build_canonical();

        let order =
            graph.topological_order()
                .expect("forward-generated graph must be acyclic");

        assert_eq!(
            order.len(),
            count
        );

        for operation in &previous_order {
            assert!(
                order.contains(operation),
                "previously present operation disappeared while growing test graph"
            );
        }

        previous_order = order;
    }
}

// =============================================================================
// Deterministic resource-independent graph identity
// =============================================================================

#[test]
fn determinism_graph_identity_does_not_depend_on_test_container_order() {
    let mut generator =
        DeterministicGenerator::new(0x1D3N_71TY);

    for _ in 0..DETERMINISM_CASES {
        let count =
            generator.inclusive_range(0, GENERATED_OPERATIONS);

        let fixture =
            GraphFixture::generate(&mut generator, count);

        let canonical =
            fixture.build_canonical();

        let mut operations_sorted =
            fixture.operations.clone();

        operations_sorted.sort();

        let mut dependencies_sorted =
            fixture.dependencies.clone();

        dependencies_sorted.sort_by_key(
            |dependency| dependency.id()
        );

        let mut sorted_graph =
            DependencyGraph::new();

        for operation in operations_sorted {
            sorted_graph
                .add_operation(operation)
                .expect("sorted operation must be accepted");
        }

        for dependency in dependencies_sorted {
            sorted_graph
                .add_dependency(dependency)
                .expect("sorted dependency must be accepted");
        }

        assert_eq!(
            canonical.topological_order(),
            sorted_graph.topological_order()
        );

        assert_eq!(
            canonical.statistics(),
            sorted_graph.statistics()
        );
    }
}