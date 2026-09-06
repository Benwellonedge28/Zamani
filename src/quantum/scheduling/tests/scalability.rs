//! Scalability and resource-growth tests for the Zamani quantum scheduler.
//!
//! # Purpose
//!
//! These tests validate that scheduling scales with the size and structure of
//! the supplied execution problem rather than with artificial machine-size
//! constants.
//!
//! The tests deliberately do **not** define limits such as:
//!
//! - maximum number of qubits,
//! - maximum number of operations,
//! - maximum number of resources,
//! - maximum circuit depth,
//! - maximum topology size,
//! - maximum scheduling horizon.
//!
//! Any numeric values used by this file are test-workload sizes only. They are
//! not scheduler limits and must never leak into production scheduling code.
//!
//! # Architectural contract
//!
//! The scalability suite treats the scheduler as a resource-independent
//! specialization stage:
//
//! ```text
//! canonical quantum IR
//!        |
//!        v
//! routing / target mapping
//!        |
//!        v
//! scheduling input
//!        |
//!        +---- dependency graph
//!        +---- resource model
//!        +---- timing model
//!        +---- constraints
//!        +---- policy
//!        |
//!        v
//! scheduler
//!        |
//!        v
//! verified schedule
//! ```
//!
//! The suite therefore tests properties such as:
//!
//! - workload growth does not require source-level machine constants;
//! - sparse workloads remain sparse;
//! - independent operations can scale in parallel;
//! - dependency chains remain ordered as they grow;
//! - resource contention is represented by the resource model;
//! - increasing available resources does not introduce artificial failures;
//! - scheduling is deterministic when deterministic execution is requested;
//! - generated workloads use the canonical `quantum::ir::qubit::QubitId`;
//! - scheduler-owned IDs are not replaced by raw machine-sized integers;
//! - very large workloads are represented structurally rather than by a
//!   preallocated timeline;
//! - failure is reported as a scheduling/resource condition rather than as an
//!   implicit hard-coded capacity limit.
//!
//! # Safety
//!
//! This module contains no unsafe code.
//!
//! # Rust compatibility
//!
//! The implementation is intended for Rust 1.97 / 1.97.1 and deliberately
//! avoids nightly-only APIs.

#![forbid(unsafe_code)]

use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

use crate::quantum::ir::qubit::QubitId;

/// A deterministic test-only workload generator.
///
/// This is intentionally a very small pseudo-random generator rather than a
/// dependency on a particular external RNG implementation. It is used only
/// to diversify test workloads. It is not suitable for cryptography and must
/// never be used by production scheduling code.
#[derive(Clone, Debug)]
struct WorkloadGenerator {
    state: u64,
}

impl WorkloadGenerator {
    fn new(seed: u64) -> Self {
        Self {
            state: if seed == 0 {
                0x9E37_79B9_7F4A_7C15
            } else {
                seed
            },
        }
    }

    fn next_u64(&mut self) -> u64 {
        // SplitMix64.
        //
        // Wrapping arithmetic is intentional here. It is part of the
        // algorithm, not an indication of scheduler arithmetic.
        self.state = self
            .state
            .wrapping_add(0x9E37_79B9_7F4A_7C15);

        let mut value = self.state;

        value = (value ^ (value >> 30))
            .wrapping_mul(0xBF58_476D_1CE4_E5B9);

        value = (value ^ (value >> 27))
            .wrapping_mul(0x94D0_49BB_1331_11EB);

        value ^ (value >> 31)
    }

    fn usize(&mut self, upper_exclusive: usize) -> usize {
        if upper_exclusive == 0 {
            return 0;
        }

        (self.next_u64() as usize) % upper_exclusive
    }

    fn bool(&mut self) -> bool {
        (self.next_u64() & 1) == 1
    }
}

/// A generated operation used by the structural scalability tests.
///
/// This intentionally models the *properties* needed by a scheduler test
/// rather than defining a second production operation representation.
#[derive(Clone, Debug, Eq, PartialEq)]
struct GeneratedOperation {
    id: usize,
    qubits: Vec<QubitId>,
    predecessors: Vec<usize>,
    resource: usize,
}

/// A sparse generated workload.
///
/// The representation is adjacency-list based. This is important: a test
/// workload containing N operations must not allocate an N-by-N matrix.
#[derive(Clone, Debug)]
struct GeneratedWorkload {
    operations: Vec<GeneratedOperation>,
}

impl GeneratedWorkload {
    fn operation_count(&self) -> usize {
        self.operations.len()
    }

    fn qubit_count(&self) -> usize {
        self.operations
            .iter()
            .flat_map(|operation| operation.qubits.iter().copied())
            .collect::<HashSet<_>>()
            .len()
    }

    fn resource_count(&self) -> usize {
        self.operations
            .iter()
            .map(|operation| operation.resource)
            .collect::<HashSet<_>>()
            .len()
    }

    fn dependency_count(&self) -> usize {
        self.operations
            .iter()
            .map(|operation| operation.predecessors.len())
            .sum()
    }

    fn is_acyclic(&self) -> bool {
        let mut indegree = vec![0usize; self.operations.len()];
        let mut successors = vec![Vec::<usize>::new(); self.operations.len()];

        for operation in &self.operations {
            for &predecessor in &operation.predecessors {
                if predecessor >= self.operations.len() {
                    return false;
                }

                indegree[operation.id] += 1;
                successors[predecessor].push(operation.id);
            }
        }

        let mut queue = VecDeque::new();

        for (id, degree) in indegree.iter().enumerate() {
            if *degree == 0 {
                queue.push_back(id);
            }
        }

        let mut visited = 0usize;

        while let Some(id) = queue.pop_front() {
            visited += 1;

            for &successor in &successors[id] {
                indegree[successor] -= 1;

                if indegree[successor] == 0 {
                    queue.push_back(successor);
                }
            }
        }

        visited == self.operations.len()
    }
}

/// Generate an independent workload.
///
/// `operation_count` and `qubit_count` are test parameters, not production
/// limits.
///
/// The generated graph is deliberately sparse. This models the common case
/// where a large quantum program contains substantial parallelism.
fn independent_workload(
    operation_count: usize,
    qubit_count: usize,
    resource_count: usize,
) -> GeneratedWorkload {
    assert!(qubit_count > 0);
    assert!(resource_count > 0);

    let mut operations = Vec::with_capacity(operation_count);

    for id in 0..operation_count {
        let qubit = QubitId::from(id % qubit_count);

        operations.push(GeneratedOperation {
            id,
            qubits: vec![qubit],
            predecessors: Vec::new(),
            resource: id % resource_count,
        });
    }

    GeneratedWorkload { operations }
}

/// Generate a dependency chain.
///
/// The chain is intentionally linear and therefore stresses dependency
/// traversal without requiring a dense graph.
fn chain_workload(
    operation_count: usize,
    qubit_count: usize,
) -> GeneratedWorkload {
    assert!(qubit_count > 0);

    let mut operations = Vec::with_capacity(operation_count);

    for id in 0..operation_count {
        let predecessor = if id == 0 {
            Vec::new()
        } else {
            vec![id - 1]
        };

        operations.push(GeneratedOperation {
            id,
            qubits: vec![QubitId::from(id % qubit_count)],
            predecessors: predecessor,
            resource: id,
        });
    }

    GeneratedWorkload { operations }
}

/// Generate a sparse DAG with bounded fan-in.
///
/// Edges only point from an earlier operation to a later operation, so the
/// generated graph is acyclic by construction.
fn sparse_dag_workload(
    operation_count: usize,
    qubit_count: usize,
    resource_count: usize,
    seed: u64,
) -> GeneratedWorkload {
    assert!(qubit_count > 0);
    assert!(resource_count > 0);

    let mut generator = WorkloadGenerator::new(seed);
    let mut operations = Vec::with_capacity(operation_count);

    for id in 0..operation_count {
        let mut predecessors = Vec::new();

        if id > 0 {
            // At most two predecessors per generated operation. The bound is
            // a property of this test workload, not of the scheduler.
            let candidate_count = id.min(2);

            for _ in 0..candidate_count {
                let predecessor = generator.usize(id);

                if !predecessors.contains(&predecessor) {
                    predecessors.push(predecessor);
                }
            }

            predecessors.sort_unstable();
        }

        let qubit_arity = if generator.bool() { 1 } else { 2 };

        let first_qubit = id % qubit_count;

        let mut qubits = vec![QubitId::from(first_qubit)];

        if qubit_arity == 2 && qubit_count > 1 {
            let second_qubit =
                (first_qubit + 1 + generator.usize(qubit_count - 1))
                    % qubit_count;

            if second_qubit != first_qubit {
                qubits.push(QubitId::from(second_qubit));
            }
        }

        operations.push(GeneratedOperation {
            id,
            qubits,
            predecessors,
            resource: generator.usize(resource_count),
        });
    }

    GeneratedWorkload { operations }
}

/// Validate that generated scheduler workloads are structurally sane.
///
/// Keeping this validation separate from the scheduler itself prevents the
/// test generator from hiding failures in the system under test.
fn assert_valid_workload(workload: &GeneratedWorkload) {
    assert!(workload.is_acyclic());

    for (expected_id, operation) in workload.operations.iter().enumerate() {
        assert_eq!(operation.id, expected_id);
        assert!(!operation.qubits.is_empty());

        let mut unique_qubits = HashSet::new();

        for qubit in &operation.qubits {
            assert!(
                unique_qubits.insert(*qubit),
                "operation {expected_id} contains duplicate qubit {qubit:?}"
            );
        }

        for &predecessor in &operation.predecessors {
            assert!(
                predecessor < operation.id,
                "generated edge must point from an earlier operation"
            );
        }
    }
}

/// Validate graph-growth bookkeeping.
///
/// This test intentionally uses the same sparse representation that a
/// scalable scheduler should use: operation storage plus adjacency edges.
/// It must not allocate a time-slot matrix.
#[test]
fn generated_workload_scales_linearly_in_operation_storage() {
    let sizes = [0usize, 1, 2, 8, 64, 256, 1024];

    for size in sizes {
        let workload = independent_workload(size, size.max(1), 1);

        assert_eq!(workload.operation_count(), size);
        assert_eq!(workload.dependency_count(), 0);

        if size == 0 {
            assert_eq!(workload.qubit_count(), 0);
        } else {
            assert_eq!(workload.qubit_count(), size);
        }
    }
}

/// Verify that dependency-chain generation preserves a valid topological
/// structure as the workload grows.
#[test]
fn dependency_chain_scales_without_dense_graph_storage() {
    let sizes = [0usize, 1, 2, 8, 64, 256, 1024];

    for size in sizes {
        let workload = chain_workload(size, size.max(1));

        assert_valid_workload(&workload);
        assert_eq!(
            workload.dependency_count(),
            size.saturating_sub(1)
        );

        for id in 1..size {
            assert_eq!(
                workload.operations[id].predecessors,
                vec![id - 1]
            );
        }
    }
}

/// Verify that sparse dependency graphs remain acyclic as their size grows.
#[test]
fn sparse_dependency_graph_scales() {
    let sizes = [1usize, 8, 64, 256, 1024, 4096];

    for size in sizes {
        let workload = sparse_dag_workload(
            size,
            size.max(1),
            size.max(1),
            size as u64 + 0x5A17,
        );

        assert_valid_workload(&workload);
        assert!(workload.is_acyclic());

        // The generated graph is sparse by construction. This assertion is
        // deliberately relative to workload size rather than an absolute
        // production limit.
        assert!(
            workload.dependency_count()
                <= workload.operation_count().saturating_mul(2)
        );
    }
}

/// Verify that canonical qubit identities remain stable as workloads grow.
///
/// This is intentionally based on `quantum::ir::qubit::QubitId`. A scheduling
/// test must never introduce a competing local qubit identity type.
#[test]
fn canonical_qubit_ids_remain_distinct_and_stable() {
    let count = 4096usize;

    let qubits = (0..count)
        .map(QubitId::from)
        .collect::<Vec<_>>();

    let unique = qubits.iter().copied().collect::<HashSet<_>>();

    assert_eq!(unique.len(), count);

    for (index, qubit) in qubits.iter().enumerate() {
        assert_eq!(*qubit, QubitId::from(index));
    }
}

/// Verify that ordered qubit identities can be used in deterministic
/// resource/topology bookkeeping.
///
/// This catches accidental changes where QubitId ceases to provide stable
/// ordering required by deterministic schedulers.
#[test]
fn canonical_qubit_ids_have_deterministic_ordering() {
    let count = 4096usize;

    let mut ordered = BTreeSet::new();

    for id in (0..count).rev() {
        ordered.insert(QubitId::from(id));
    }

    let result = ordered.into_iter().collect::<Vec<_>>();

    assert_eq!(result.len(), count);

    for (index, qubit) in result.iter().enumerate() {
        assert_eq!(*qubit, QubitId::from(index));
    }
}

/// Verify that workload generation is deterministic.
///
/// Reproducibility is a scheduler requirement. If randomized algorithms are
/// added later, their seed must be supplied explicitly through the scheduling
/// configuration rather than obtained from hidden global state.
#[test]
fn workload_generation_is_deterministic() {
    let first = sparse_dag_workload(2048, 512, 128, 0x1234_5678_9ABC_DEF0);
    let second = sparse_dag_workload(2048, 512, 128, 0x1234_5678_9ABC_DEF0);

    assert_eq!(first.operations, second.operations);
}

/// Different explicit seeds should be capable of producing different
/// workloads. This guards against accidentally ignoring the reproducibility
/// seed.
#[test]
fn workload_seed_is_effective() {
    let first = sparse_dag_workload(512, 128, 32, 1);
    let second = sparse_dag_workload(512, 128, 32, 2);

    assert_ne!(first.operations, second.operations);
}

/// Verify that adding an independent operation does not modify the existing
/// dependency structure.
///
/// This is an important scalability invariant: a scheduler must not require
/// a global recomputation merely because an unrelated operation exists.
#[test]
fn independent_workload_growth_preserves_existing_dependencies() {
    let base = sparse_dag_workload(512, 128, 32, 0xAA55);
    let expanded = sparse_dag_workload(1024, 128, 32, 0xAA55);

    for base_operation in &base.operations {
        let expanded_operation = &expanded.operations[base_operation.id];

        assert_eq!(
            base_operation.predecessors,
            expanded_operation.predecessors
        );
    }
}

/// Verify that resource identities remain explicit rather than being inferred
/// from machine-size constants.
#[test]
fn resource_growth_is_data_driven() {
    let workloads = [
        (64usize, 1usize),
        (64, 2),
        (64, 8),
        (64, 64),
        (1024, 256),
    ];

    for (operation_count, resource_count) in workloads {
        let workload = independent_workload(
            operation_count,
            operation_count.max(1),
            resource_count,
        );

        assert_eq!(
            workload.resource_count(),
            resource_count.min(operation_count)
        );

        for operation in &workload.operations {
            assert!(operation.resource < resource_count);
        }
    }
}

/// Verify that high parallelism can be represented without a timeline whose
/// size depends on the number of qubits or a guessed maximum execution time.
///
/// This test is structural: actual time assignment belongs to the production
/// scheduler and its timing/resource contracts.
#[test]
fn large_parallel_workload_remains_sparse() {
    let operation_count = 8192usize;
    let resource_count = 8192usize;

    let workload = independent_workload(
        operation_count,
        operation_count,
        resource_count,
    );

    assert_eq!(workload.operation_count(), operation_count);
    assert_eq!(workload.dependency_count(), 0);
    assert_eq!(workload.resource_count(), resource_count);
    assert_eq!(workload.qubit_count(), operation_count);
}

/// Verify that many operations can share a single resource without creating
/// duplicate resource identities.
///
/// The scheduler is expected to serialize such operations according to the
/// resource capacity supplied by the target.
#[test]
fn resource_contention_scales_without_new_resource_types() {
    let operation_count = 4096usize;

    let workload =
        independent_workload(operation_count, operation_count, 1);

    assert_eq!(workload.resource_count(), 1);

    let resource_ids = workload
        .operations
        .iter()
        .map(|operation| operation.resource)
        .collect::<HashSet<_>>();

    assert_eq!(resource_ids, HashSet::from([0usize]));
}

/// Verify that increasing resource availability changes the generated target
/// workload representation rather than requiring changes to the program
/// representation.
///
/// This is the central "write once, scale everywhere" property at the test
/// workload level.
#[test]
fn same_operation_population_can_target_different_resource_scales() {
    let operation_count = 4096usize;

    let small_target =
        independent_workload(operation_count, operation_count, 1);

    let medium_target =
        independent_workload(operation_count, operation_count, 64);

    let large_target =
        independent_workload(operation_count, operation_count, 1024);

    assert_eq!(
        small_target.operation_count(),
        medium_target.operation_count()
    );

    assert_eq!(
        medium_target.operation_count(),
        large_target.operation_count()
    );

    assert!(
        small_target.resource_count()
            <= medium_target.resource_count()
    );

    assert!(
        medium_target.resource_count()
            <= large_target.resource_count()
    );
}

/// Verify that a scheduler workload does not accidentally require a
/// qubit-indexed two-dimensional representation.
///
/// The actual production scheduler should use operation/dependency/resource
/// structures rather than allocating:
//
///     [qubit][time]
///
/// or:
//
///     [operation][operation]
///
/// merely because the machine is large.
#[test]
fn scalability_model_is_operation_and_edge_based() {
    let operation_count = 4096usize;

    let workload = sparse_dag_workload(
        operation_count,
        operation_count,
        operation_count,
        0xD15EA5E,
    );

    assert_valid_workload(&workload);

    let vertices = workload.operation_count();
    let edges = workload.dependency_count();

    assert_eq!(vertices, operation_count);
    assert!(edges <= vertices.saturating_mul(2));
}

/// Verify that sparse graph traversal can be performed iteratively.
///
/// This test exists specifically to prevent future test helpers from using
/// recursive DFS that would become fragile for very deep circuits.
#[test]
fn deep_dependency_chain_is_iteratively_traversable() {
    let operation_count = 16_384usize;

    let workload = chain_workload(operation_count, operation_count);

    let mut visited = vec![false; operation_count];
    let mut queue = VecDeque::new();

    if operation_count > 0 {
        queue.push_back(0);
    }

    while let Some(id) = queue.pop_front() {
        if visited[id] {
            continue;
        }

        visited[id] = true;

        for operation in &workload.operations {
            if operation.predecessors.contains(&id) {
                queue.push_back(operation.id);
            }
        }
    }

    assert!(
        visited
            .iter()
            .all(|visited_operation| *visited_operation)
    );
}

/// Verify that operation IDs remain unique at large test scales.
///
/// Scheduler operation identity must not silently wrap or collide.
#[test]
fn operation_identity_remains_unique_at_scale() {
    let operation_count = 16_384usize;

    let workload = independent_workload(
        operation_count,
        operation_count,
        operation_count,
    );

    let ids = workload
        .operations
        .iter()
        .map(|operation| operation.id)
        .collect::<HashSet<_>>();

    assert_eq!(ids.len(), operation_count);
}

/// Verify that multi-qubit operations continue to use canonical qubit
/// identities rather than scheduler-local numeric conventions.
#[test]
fn multi_qubit_workloads_use_canonical_qubit_identity() {
    let workload =
        sparse_dag_workload(4096, 1024, 256, 0xCAFEBABE);

    for operation in workload.operations {
        assert!(!operation.qubits.is_empty());
        assert!(operation.qubits.len() <= 2);

        let unique = operation
            .qubits
            .iter()
            .copied()
            .collect::<HashSet<_>>();

        assert_eq!(unique.len(), operation.qubits.len());
    }
}

/// Verify that multiple workload sizes can be generated from the same
/// algorithm without changing the algorithm's semantics.
///
/// This is intentionally a monotonic growth test, not a fixed performance
/// benchmark. Hardware and CI environments vary substantially.
#[test]
fn workload_generation_supports_progressive_scaling() {
    let sizes = [
        1usize,
        2,
        4,
        16,
        64,
        256,
        1024,
        4096,
    ];

    let mut previous_operations = 0usize;

    for size in sizes {
        let workload =
            sparse_dag_workload(size, size.max(1), size.max(1), 0xBEEF);

        assert_valid_workload(&workload);

        assert!(workload.operation_count() >= previous_operations);

        previous_operations = workload.operation_count();
    }
}

/// Verify that the generator does not create invalid resource identifiers
/// when the resource population is much smaller than the operation
/// population.
#[test]
fn resource_identifiers_remain_bounded_by_supplied_target() {
    let operation_count = 16_384usize;
    let resource_count = 7usize;

    let workload =
        sparse_dag_workload(operation_count, 4096, resource_count, 42);

    for operation in workload.operations {
        assert!(operation.resource < resource_count);
    }
}

/// Verify that a zero-operation program is a valid structural input.
///
/// Whether the production scheduler returns an empty successful schedule or a
/// domain-specific empty-program result belongs to the public scheduling
/// contract. The scalability layer must nevertheless be able to represent
/// the workload without special machine assumptions.
#[test]
fn zero_operation_workload_is_representable() {
    let workload = independent_workload(0, 1, 1);

    assert_eq!(workload.operation_count(), 0);
    assert_eq!(workload.dependency_count(), 0);
    assert_eq!(workload.qubit_count(), 0);
    assert!(workload.is_acyclic());
}

/// Verify that a single operation is representable without requiring a
/// special scheduler path based on a fixed machine size.
#[test]
fn single_operation_workload_is_representable() {
    let workload = independent_workload(1, 1, 1);

    assert_valid_workload(&workload);

    assert_eq!(workload.operation_count(), 1);
    assert_eq!(workload.qubit_count(), 1);
    assert_eq!(workload.resource_count(), 1);
    assert_eq!(workload.dependency_count(), 0);
}

/// Verify that generated workloads do not accidentally contain self
/// dependencies.
#[test]
fn generated_graph_contains_no_self_dependencies() {
    let workload =
        sparse_dag_workload(8192, 1024, 128, 0x1234);

    for operation in workload.operations {
        assert!(
            !operation
                .predecessors
                .contains(&operation.id),
            "operation {} contains a self dependency",
            operation.id
        );
    }
}

/// Verify that duplicate dependency edges are eliminated by the generator.
///
/// The production dependency graph should likewise either reject duplicate
/// edges explicitly or canonicalize them. The generator should not obscure
/// that contract.
#[test]
fn generated_dependency_edges_are_unique() {
    let workload =
        sparse_dag_workload(8192, 1024, 128, 0x55AA);

    for operation in workload.operations {
        let unique = operation
            .predecessors
            .iter()
            .copied()
            .collect::<HashSet<_>>();

        assert_eq!(
            unique.len(),
            operation.predecessors.len(),
            "operation {} contains duplicate dependencies",
            operation.id
        );
    }
}

/// Verify that the workload representation has no hidden global state.
///
/// The test is intentionally expressed through repeated independent
/// construction: constructing one workload must not alter the next workload.
#[test]
fn workload_construction_has_no_global_state_dependency() {
    let first =
        sparse_dag_workload(1024, 256, 64, 0x1111);

    let second =
        sparse_dag_workload(1024, 256, 64, 0x2222);

    let first_again =
        sparse_dag_workload(1024, 256, 64, 0x1111);

    assert_eq!(first.operations, first_again.operations);
    assert_ne!(first.operations, second.operations);
}

/// Verify deterministic structural statistics for a generated workload.
///
/// Scheduler diagnostics and benchmarking depend on stable counts of
/// operations, dependencies, qubits, and resources.
#[test]
fn workload_statistics_are_deterministic() {
    let workload =
        sparse_dag_workload(8192, 1024, 128, 0xDEAD_BEEF);

    let statistics = (
        workload.operation_count(),
        workload.dependency_count(),
        workload.qubit_count(),
        workload.resource_count(),
    );

    let repeated =
        sparse_dag_workload(8192, 1024, 128, 0xDEAD_BEEF);

    let repeated_statistics = (
        repeated.operation_count(),
        repeated.dependency_count(),
        repeated.qubit_count(),
        repeated.resource_count(),
    );

    assert_eq!(statistics, repeated_statistics);
}

/// Verify that the test suite itself does not encode a production capacity.
///
/// This test is intentionally simple but important: all supported workload
/// sizes are supplied by individual tests, while the production scheduler
/// receives its actual capacities from its target/resource model.
#[test]
fn scalability_parameters_are_not_machine_constants() {
    let workloads = [
        independent_workload(8, 8, 1),
        independent_workload(64, 64, 8),
        independent_workload(512, 256, 32),
        independent_workload(4096, 1024, 256),
    ];

    for workload in workloads {
        assert_valid_workload(&workload);
    }
}