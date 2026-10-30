
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani Distributed Portability Tests
//!
//! Path: src/distributed/tests/portability.rs
//!
//! Language: Rust
//! Edition: 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! These tests verify the portability and scale-independence properties
//! of the existing distributed workload-planning implementation.
//!
//! They establish that:
//!
//! - Empty workloads are represented correctly.
//! - A single logical workload can be partitioned in different ways.
//! - Increasing workload size does not introduce an application-defined
//!   universal task or partition ceiling.
//! - All logical work units survive partitioning.
//! - All dependency relationships survive partitioning.
//! - Computational work estimates are preserved.
//! - Equivalent workload graphs produce deterministic plans.
//! - Operational budget exhaustion is reported explicitly.
//! - Logical workload identity is independent of physical realization.
//!
//! ## Architectural ownership
//!
//! This test module does not introduce:
//!
//! - A new distributed workload model.
//! - A new resource model.
//! - A new capability model.
//! - A new scheduler.
//! - A new partitioning implementation.
//! - A new universal intermediate representation.
//! - A physical hardware abstraction.
//! - A network transport implementation.
//!
//! It exercises the existing:
//!
//! - distributed planning partitioner;
//! - logical workload graph;
//! - partition policy;
//! - partition budget;
//! - partition validation contract.
//!
//! ## POCO-REAF
//!
//! The fundamental invariant is:
//!
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.
//!
//! Logical program meaning must survive changes in partitioning and
//! execution realization.
//!
//! A different partition count must not silently change:
//!
//! - Work-unit identity.
//! - Dependency relationships.
//! - Total declared computational work.
//! - Logical workload membership.
//!
//! ## Scalability
//!
//! These tests deliberately do not establish a maximum supported workload.
//!
//! Test fixture sizes are representative samples, not semantic limits.
//!
//! Actual execution remains bounded by available memory, processing
//! resources, address space, representational limits, and explicitly
//! configured operational budgets.
//!
//! ## Important limitation
//!
//! Successful partitioning does not prove that a physical target can
//! execute the workload.
//!
//! Capability feasibility, resource feasibility, placement, routing,
//! scheduling, deployment, and actual execution are separate contracts.
//!
//! These tests must not be interpreted as proof of physical execution
//! portability across every hardware architecture.

use crate::distributed::planning::partition::{
    partition_workload,
    Dependency,
    PartitionBudget,
    PartitionPlan,
    PartitionPolicy,
    WorkloadGraph,
    WorkUnit,
};

/// Constructs a deterministic linear workload graph.
///
/// Every work unit has:
///
/// - A stable logical identity.
/// - A non-negative abstract work estimate.
/// - No physical target identity.
///
/// Adjacent work units have a dependency relationship.
///
/// The graph is useful for testing whether partitioning preserves
/// the original workload semantics.
///
/// # Arguments
///
/// * `size` - Number of logical work units.
///
/// # Returns
///
/// A workload graph containing the requested number of work units.
///
/// An empty graph is valid.
fn linear_workload(size: usize) -> WorkloadGraph<usize> {
    let units = (0..size)
        .map(|index| {
            WorkUnit::new(
                index,
                u128::try_from(index)
                    .expect("usize index must fit into u128")
                    + 1,
                None,
            )
        })
        .collect();

    let dependencies = (1..size)
        .map(|index| {
            Dependency::new(
                index - 1,
                index,
                1,
            )
        })
        .collect();

    WorkloadGraph::new(units, dependencies)
}

/// Calculates the expected total work of a generated workload.
///
/// Uses checked arithmetic so the test itself does not silently
/// overflow when evaluating large fixture values.
fn expected_total_work(size: usize) -> u128 {
    (0..size)
        .map(|index| {
            u128::try_from(index)
                .expect("usize index must fit into u128")
                + 1
        })
        .try_fold(0_u128, u128::checked_add)
        .expect("test workload total must be representable")
}

/// Creates a complete partition plan using the existing planner.
///
/// No artificial work-unit, partition, or dependency limits are
/// introduced by this helper.
fn create_partition_plan(
    workload: &WorkloadGraph<usize>,
    requested_partitions: Option<usize>,
) -> PartitionPlan<usize> {
    partition_workload(
        workload,
        PartitionPolicy::new()
            .with_target_partition_count(requested_partitions),
        PartitionBudget::unlimited(),
    )
    .expect("valid workload should produce a complete partition plan")
}

/// Verifies that every logical work unit appears exactly once.
///
/// The existing planner provides structural validation against
/// the original workload graph.
fn assert_plan_preserves_workload(
    workload: &WorkloadGraph<usize>,
    plan: &PartitionPlan<usize>,
) {
    plan.validate_against(workload)
        .expect("partition plan must preserve workload structure");

    let mut actual_units = plan
        .partitions()
        .iter()
        .flat_map(|partition| partition.units().iter().copied())
        .collect::<Vec<_>>();

    actual_units.sort_unstable();

    let mut expected_units = workload
        .units()
        .iter()
        .map(|unit| *unit.id())
        .collect::<Vec<_>>();

    expected_units.sort_unstable();

    assert_eq!(
        actual_units,
        expected_units,
        "partitioning must preserve every logical work unit exactly once"
    );
}

/// Verifies that the original logical dependency count survives
/// partitioning.
fn assert_dependencies_preserved(
    workload: &WorkloadGraph<usize>,
    plan: &PartitionPlan<usize>,
) {
    assert_eq!(
        plan.dependency_count(),
        workload.dependencies().len(),
        "partitioning must preserve every dependency"
    );

    plan.validate_against(workload)
        .expect("dependency relationships must remain valid");
}

/// Empty distributed workloads must be valid.
///
/// An empty computation must not require a physical participant,
/// worker, machine, or execution target merely to produce a
/// structurally valid empty partition plan.
#[test]
fn empty_workload_is_portable() {
    let workload = linear_workload(0);

    let plan = create_partition_plan(&workload, None);

    assert!(plan.is_empty());

    assert_eq!(
        plan.partitions().len(),
        0
    );

    assert_eq!(
        plan.dependency_count(),
        0
    );

    assert_eq!(
        plan.total_estimated_work(),
        0
    );

    assert_plan_preserves_workload(
        &workload,
        &plan,
    );
}

/// A single logical operation must work without requiring
/// a distributed physical deployment.
///
/// Distributed semantics must not force unnecessary distribution.
#[test]
fn single_work_unit_is_portable() {
    let workload = linear_workload(1);

    let plan = create_partition_plan(
        &workload,
        None,
    );

    assert_eq!(
        plan.total_estimated_work(),
        1
    );

    assert_eq!(
        plan.dependency_count(),
        0
    );

    assert_plan_preserves_workload(
        &workload,
        &plan,
    );
}

/// A workload must remain semantically complete when its logical
/// partitioning strategy changes.
///
/// The same workload is planned using:
///
/// - One logical partition.
/// - Multiple logical partitions.
/// - A partition count corresponding to the number of work units.
///
/// The partitioning choice must not change program meaning.
#[test]
fn different_partition_counts_preserve_program_meaning() {
    let workload = linear_workload(12);

    let total_work = expected_total_work(12);

    let dependency_count = workload.dependencies().len();

    let partition_counts = [
        1,
        3,
        6,
        12,
    ];

    for requested_count in partition_counts {
        let plan = create_partition_plan(
            &workload,
            Some(requested_count),
        );

        assert_eq!(
            plan.total_estimated_work(),
            total_work,
            "partitioning must preserve total work"
        );

        assert_eq!(
            plan.dependency_count(),
            dependency_count,
            "partitioning must preserve dependency count"
        );

        assert_plan_preserves_workload(
            &workload,
            &plan,
        );

        assert_dependencies_preserved(
            &workload,
            &plan,
        );
    }
}

/// Progressively larger workloads must remain representable by the
/// existing partitioning implementation.
///
/// These sizes are test samples, not maximum supported capacities.
///
/// No source-level or universal machine-size limit is implied.
#[test]
fn progressively_larger_workloads_preserve_portability() {
    let workload_sizes = [
        1_usize,
        2,
        8,
        32,
        128,
    ];

    for size in workload_sizes {
        let workload = linear_workload(size);

        let plan = create_partition_plan(
            &workload,
            Some(size),
        );

        assert_eq!(
            plan.total_estimated_work(),
            expected_total_work(size),
            "total work must remain invariant as workload size increases"
        );

        assert_eq!(
            plan.dependency_count(),
            size.saturating_sub(1),
            "dependency count must remain correct"
        );

        assert_plan_preserves_workload(
            &workload,
            &plan,
        );
    }
}

/// Equivalent logical workloads must produce equivalent plans
/// regardless of the order in which their input collections
/// were constructed.
///
/// This protects deterministic planning from incidental input
/// enumeration order.
#[test]
fn equivalent_input_order_produces_deterministic_plans() {
    let first = linear_workload(16);

    let mut reversed_units = first
        .units()
        .iter()
        .cloned()
        .collect::<Vec<_>>();

    reversed_units.reverse();

    let mut reversed_dependencies = first
        .dependencies()
        .iter()
        .cloned()
        .collect::<Vec<_>>();

    reversed_dependencies.reverse();

    let second = WorkloadGraph::new(
        reversed_units,
        reversed_dependencies,
    );

    let first_plan = create_partition_plan(
        &first,
        Some(4),
    );

    let second_plan = create_partition_plan(
        &second,
        Some(4),
    );

    assert_eq!(
        first_plan,
        second_plan,
        "equivalent logical workloads must produce deterministic plans"
    );
}

/// Partitioning must preserve the original abstract computational
/// work, including when dependencies cross partition boundaries.
#[test]
fn cross_partition_dependencies_preserve_semantics() {
    let workload = linear_workload(16);

    let plan = create_partition_plan(
        &workload,
        Some(8),
    );

    assert_plan_preserves_workload(
        &workload,
        &plan,
    );

    assert_dependencies_preserved(
        &workload,
        &plan,
    );

    for boundary in plan.boundaries() {
        assert_ne!(
            boundary.source_partition(),
            boundary.destination_partition(),
            "a reported boundary must connect distinct partitions"
        );

        assert!(
            workload
                .dependencies()
                .iter()
                .any(|dependency| {
                    dependency.source() == boundary.source()
                        && dependency.destination() == boundary.destination()
                        && dependency.communication_cost()
                            == boundary.communication_cost()
                }),
            "every partition boundary must originate from an actual dependency"
        );
    }
}

/// Resource budgets must be operational constraints rather than
/// hidden semantic restrictions.
///
/// When an explicitly configured budget is exceeded, planning
/// must return an error instead of silently truncating the workload.
#[test]
fn operational_budget_exhaustion_is_explicit() {
    let workload = linear_workload(4);

    let result = partition_workload(
        &workload,
        PartitionPolicy::new(),
        PartitionBudget::unlimited()
            .with_max_work_units(Some(2)),
    );

    assert!(
        result.is_err(),
        "exceeding an explicit work-unit budget must not produce success"
    );
}

/// An operational budget must not change the meaning of a workload
/// that fits within the budget.
#[test]
fn sufficient_operational_budget_preserves_workload() {
    let workload = linear_workload(4);

    let result = partition_workload(
        &workload,
        PartitionPolicy::new(),
        PartitionBudget::unlimited()
            .with_max_work_units(Some(4))
            .with_max_dependencies(Some(3)),
    );

    let plan = result.expect(
        "a workload within the configured operational budget should succeed"
    );

    assert_eq!(
        plan.total_estimated_work(),
        expected_total_work(4)
    );

    assert_plan_preserves_workload(
        &workload,
        &plan,
    );
}

/// The partitioning layer must not confuse logical partitions
/// with physical machines or workers.
///
/// Changing the number of logical partitions must preserve
/// the source workload's semantic facts.
#[test]
fn logical_partitioning_is_independent_of_physical_identity() {
    let workload = linear_workload(8);

    let local_plan = create_partition_plan(
        &workload,
        Some(1),
    );

    let distributed_plan = create_partition_plan(
        &workload,
        Some(8),
    );

    assert_eq!(
        local_plan.total_estimated_work(),
        distributed_plan.total_estimated_work()
    );

    assert_eq!(
        local_plan.dependency_count(),
        distributed_plan.dependency_count()
    );

    assert_plan_preserves_workload(
        &workload,
        &local_plan,
    );

    assert_plan_preserves_workload(
        &workload,
        &distributed_plan,
    );
}

/// Zero-length and single-element workloads must not introduce
/// arithmetic underflow or invalid dependency construction.
#[test]
fn workload_boundary_cases_are_valid() {
    for size in [0_usize, 1, 2] {
        let workload = linear_workload(size);

        let plan = create_partition_plan(
            &workload,
            None,
        );

        assert_eq!(
            plan.total_estimated_work(),
            expected_total_work(size)
        );

        assert_eq!(
            plan.dependency_count(),
            size.saturating_sub(1)
        );

        assert_plan_preserves_workload(
            &workload,
            &plan,
        );
    }
}

/// Repeated planning of an unchanged workload must not mutate
/// the original logical workload graph.
#[test]
fn planning_does_not_mutate_source_workload() {
    let workload = linear_workload(16);

    let original = workload.clone();

    let _first = create_partition_plan(
        &workload,
        Some(2),
    );

    let _second = create_partition_plan(
        &workload,
        Some(8),
    );

    assert_eq!(
        workload,
        original,
        "planning must not mutate source workload semantics"
    );
}
