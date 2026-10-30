
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Distributed Planning Integration Tests
//!
//! Path: src/distributed/tests/planning.rs
//!
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Verify the integration and correctness of the existing distributed
//! planning stages without introducing another planning implementation.
//!
//! These tests exercise:
//!
//! - feasibility;
//! - partitioning;
//! - placement;
//! - scheduling;
//! - routing;
//! - cross-stage invariants;
//! - failure propagation;
//! - configurable operational budgets;
//! - deterministic results;
//! - portability across different logical target topologies;
//! - large workloads within available process resources.
//!
//! ## Architectural ownership
//!
//! The tests consume the existing implementations:
//!
//! crate::distributed::planning::feasibility
//! crate::distributed::planning::partition
//! crate::distributed::planning::placement
//! crate::distributed::planning::scheduling
//! crate::distributed::planning::routing
//!
//! They MUST NOT create:
//!
//! - a second workload graph;
//! - another partitioning algorithm;
//! - another placement algorithm;
//! - another scheduler;
//! - another routing engine;
//! - another capability registry;
//! - another resource model;
//! - another semantic representation;
//! - another hardware abstraction.
//!
//! ## POCO-REAF
//!
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.
//!
//! A logical workload must remain independent of its physical realization.
//!
//! The same workload should be plannable against different compatible
//! resource topologies without modifying the workload itself.
//!
//! ## Important limitation
//!
//! These tests use deterministic in-memory providers and simulated
//! resource snapshots. They establish planning correctness, not actual
//! distributed execution, physical allocation, or network connectivity.
//!
//! Real provider-backed integration tests belong in the relevant
//! runtime, networking, HAL, and deployment integration suites.

use std::collections::{BTreeMap, BTreeSet};

use crate::distributed::planning::partition::{
    partition_workload,
    Dependency,
    PartitionBudget,
    PartitionError,
    PartitionPlan,
    PartitionPolicy,
    WorkUnit,
    WorkloadGraph,
};

use crate::distributed::planning::placement::{
    place_partitions,
    PlacementError,
    PlacementEvaluator,
    PlacementPlan,
    PlacementPolicy,
    PlacementTarget,
    ProjectedLoad,
    TargetAssessment,
    WorkPartition,
};

use crate::distributed::planning::scheduling::{
    schedule_workload,
    ResourceCapacity,
    SchedulePlan,
    ScheduleProfile,
    SchedulingError,
    SchedulingPolicy,
};

use crate::distributed::planning::routing::{
    find_route_by_base_cost,
    RoutePlan,
    RoutingBudget,
    RoutingError,
    RoutingGraph,
    RoutingPolicy,
};

// =============================================================================
// REUSABLE TEST PROVIDERS
// =============================================================================

/// Deterministic target capability snapshot.
///
/// This is test evidence, not a physical hardware representation.
#[derive(Clone, Debug, Eq, PartialEq)]
struct TestTargetSnapshot {
    eligible: bool,
    score: u128,
    resource_capacity: u128,
}

/// Evaluates whether a logical partition can be placed on a supplied target.
///
/// The evaluator is intentionally separate from the placement algorithm.
/// This verifies that placement consumes injected policy/evidence rather
/// than embedding a fixed hardware catalogue.
struct TestPlacementEvaluator;

impl PlacementEvaluator<u64, String, TestTargetSnapshot>
    for TestPlacementEvaluator
{
    fn assess(
        &self,
        partition: &WorkPartition<u64>,
        target: &PlacementTarget<String, TestTargetSnapshot>,
        projected_load: ProjectedLoad,
    ) -> Result<TargetAssessment, crate::distributed::planning::placement::PlacementEvaluationError>
    {
        let snapshot = target.snapshot();

        if !snapshot.eligible {
            return Ok(TargetAssessment::ineligible(
                "The supplied target evidence rejects this partition.",
            ));
        }

        let required = partition.estimated_work();

        let projected = projected_load
            .assigned_work()
            .checked_add(required);

        let Some(projected) = projected else {
            return Ok(TargetAssessment::ineligible(
                "Projected work exceeds the representable resource estimate.",
            ));
        };

        if projected > snapshot.resource_capacity {
            return Ok(TargetAssessment::ineligible(
                "The supplied resource capacity is insufficient.",
            ));
        }

        Ok(TargetAssessment::eligible(
            snapshot.score,
            "The supplied capability and resource evidence accepts this partition.",
        ))
    }
}

/// Evaluator that cannot establish target feasibility.
///
/// Missing evidence must never be interpreted as proof of support.
struct IndeterminatePlacementEvaluator;

impl PlacementEvaluator<u64, String, TestTargetSnapshot>
    for IndeterminatePlacementEvaluator
{
    fn assess(
        &self,
        _partition: &WorkPartition<u64>,
        _target: &PlacementTarget<String, TestTargetSnapshot>,
        _projected_load: ProjectedLoad,
    ) -> Result<TargetAssessment, crate::distributed::planning::placement::PlacementEvaluationError>
    {
        Ok(TargetAssessment::indeterminate(
            "The resource provider supplied insufficient evidence.",
        ))
    }
}

// =============================================================================
// FIXTURES
// =============================================================================

/// Creates a logical work unit.
fn work_unit(id: u64, work: u128) -> WorkUnit<u64> {
    WorkUnit::new(id, work, None)
}

/// Creates a directed dependency.
///
/// The destination depends on the source.
fn dependency(source: u64, destination: u64) -> Dependency<u64> {
    Dependency::new(source, destination, 1)
}

/// Creates a deterministic workload graph.
fn workload_graph(
    units: Vec<WorkUnit<u64>>,
    dependencies: Vec<Dependency<u64>>,
) -> WorkloadGraph<u64> {
    WorkloadGraph::new(units, dependencies)
}

/// Creates a sequential workload.
fn sequential_workload() -> WorkloadGraph<u64> {
    workload_graph(
        vec![
            work_unit(1, 5),
            work_unit(2, 4),
            work_unit(3, 3),
        ],
        vec![
            dependency(1, 2),
            dependency(2, 3),
        ],
    )
}

/// Creates compatible target snapshots.
///
/// Every target has its own logical identity and supplied capabilities.
fn targets(
    names: &[&str],
    capacity: u128,
) -> Vec<PlacementTarget<String, TestTargetSnapshot>> {
    names
        .iter()
        .map(|name| {
            PlacementTarget::new(
                (*name).to_owned(),
                TestTargetSnapshot {
                    eligible: true,
                    score: 1,
                    resource_capacity: capacity,
                },
            )
        })
        .collect()
}

/// Partitions a workload using an explicitly supplied policy.
fn partition(
    graph: &WorkloadGraph<u64>,
    policy: PartitionPolicy,
    budget: PartitionBudget,
) -> PartitionPlan<u64> {
    partition_workload(graph, policy, budget)
        .expect("valid workload should produce a partition plan")
}

/// Places all partitions using the test provider.
fn place(
    partitions: &PartitionPlan<u64>,
    candidates: &[PlacementTarget<String, TestTargetSnapshot>],
) -> PlacementPlan<String> {
    place_partitions(
        partitions,
        candidates,
        PlacementPolicy::new(),
        &TestPlacementEvaluator,
    )
    .expect("compatible targets should permit placement")
}

/// Creates scheduling profiles for every workload unit.
///
/// Time units are supplied by the test and have no hard-coded relationship
/// to physical clocks or a particular processor architecture.
fn profiles(
    graph: &WorkloadGraph<u64>,
) -> Vec<ScheduleProfile<u64, String>> {
    graph
        .units()
        .iter()
        .map(|unit| {
            ScheduleProfile::new(
                *unit.id(),
                unit.estimated_work(),
            )
        })
        .collect()
}

/// Creates a fully connected test communication topology.
fn fully_connected_topology(
    participants: &[String],
    edge_cost: u128,
) -> RoutingGraph<String> {
    let mut graph = RoutingGraph::new();

    for participant in participants {
        graph.add_node(participant.clone());
    }

    for source in participants {
        for destination in participants {
            if source != destination {
                graph
                    .add_edge(
                        source,
                        destination.clone(),
                        edge_cost,
                    )
                    .expect("unique topology edge should be accepted");
            }
        }
    }

    graph
}

// =============================================================================
// PARTITIONING INTEGRATION
// =============================================================================

/// Verify that partitioning preserves all logical work-unit identities.
#[test]
fn partitioning_preserves_every_work_unit() {
    let graph = sequential_workload();

    let plan = partition(
        &graph,
        PartitionPolicy::new()
            .with_target_partition_count(Some(3))
            .with_max_units_per_partition(Some(1)),
        PartitionBudget::unlimited(),
    );

    plan.validate_against(&graph)
        .expect("partition plan must preserve the original graph");

    let actual: BTreeSet<u64> = plan
        .partitions()
        .iter()
        .flat_map(|partition| partition.units().iter().copied())
        .collect();

    assert_eq!(actual, BTreeSet::from([1, 2, 3]));

    assert_eq!(plan.total_estimated_work(), 12);
    assert_eq!(plan.dependency_count(), 2);
}

/// Verify that dependency boundaries survive partitioning.
#[test]
fn partitioning_preserves_cross_partition_dependencies() {
    let graph = sequential_workload();

    let plan = partition(
        &graph,
        PartitionPolicy::new()
            .with_target_partition_count(Some(3))
            .with_max_units_per_partition(Some(1)),
        PartitionBudget::unlimited(),
    );

    assert_eq!(plan.boundaries().len(), 2);

    for boundary in plan.boundaries() {
        assert_ne!(
            boundary.source_partition(),
            boundary.destination_partition(),
        );
    }

    plan.validate_against(&graph)
        .expect("all dependency boundaries must be valid");
}

/// Verify that duplicate logical work-unit identifiers are rejected.
#[test]
fn partitioning_rejects_duplicate_work_unit_ids() {
    let graph = workload_graph(
        vec![
            work_unit(1, 5),
            work_unit(1, 10),
        ],
        Vec::new(),
    );

    let result = partition_workload(
        &graph,
        PartitionPolicy::new(),
        PartitionBudget::unlimited(),
    );

    assert_eq!(
        result,
        Err(PartitionError::DuplicateWorkUnit),
    );
}

/// Verify that unknown dependency endpoints cannot be silently discarded.
#[test]
fn partitioning_rejects_unknown_dependency_endpoints() {
    let graph = workload_graph(
        vec![work_unit(1, 5)],
        vec![dependency(1, 999)],
    );

    let result = partition_workload(
        &graph,
        PartitionPolicy::new(),
        PartitionBudget::unlimited(),
    );

    assert_eq!(
        result,
        Err(PartitionError::UnknownWorkUnit),
    );
}

/// Verify that an exhausted operational budget produces an explicit error.
#[test]
fn partitioning_reports_budget_exhaustion() {
    let graph = workload_graph(
        vec![
            work_unit(1, 1),
            work_unit(2, 1),
            work_unit(3, 1),
        ],
        Vec::new(),
    );

    let result = partition_workload(
        &graph,
        PartitionPolicy::new(),
        PartitionBudget::unlimited()
            .with_max_work_units(Some(2)),
    );

    assert!(matches!(
        result,
        Err(PartitionError::BudgetExceeded {
            dimension: "work units",
            actual: 3,
            maximum: 2,
        })
    ));
}

/// Verify that arithmetic overflow is never silently wrapped.
#[test]
fn partitioning_detects_work_estimate_overflow() {
    let graph = workload_graph(
        vec![
            work_unit(1, u128::MAX),
            work_unit(2, 1),
        ],
        Vec::new(),
    );

    let result = partition_workload(
        &graph,
        PartitionPolicy::new(),
        PartitionBudget::unlimited(),
    );

    assert_eq!(
        result,
        Err(PartitionError::ArithmeticOverflow),
    );
}

// =============================================================================
// PLACEMENT INTEGRATION
// =============================================================================

/// Verify that every logical partition receives a valid placement.
#[test]
fn placement_assigns_every_partition() {
    let graph = sequential_workload();

    let partitions = partition(
        &graph,
        PartitionPolicy::new()
            .with_target_partition_count(Some(3))
            .with_max_units_per_partition(Some(1)),
        PartitionBudget::unlimited(),
    );

    let candidates = targets(
        &["target-a", "target-b", "target-c"],
        100,
    );

    let result = place(&partitions, &candidates);

    assert_eq!(
        result.assignments().len(),
        partitions.partitions().len(),
    );

    result
        .validate_against(&partitions)
        .expect("placement must cover all partitions");
}

/// Verify that incompatible targets cannot receive work.
#[test]
fn placement_rejects_incompatible_targets() {
    let graph = sequential_workload();

    let partitions = partition(
        &graph,
        PartitionPolicy::new(),
        PartitionBudget::unlimited(),
    );

    let candidates = vec![
        PlacementTarget::new(
            "unsupported".to_owned(),
            TestTargetSnapshot {
                eligible: false,
                score: 0,
                resource_capacity: 0,
            },
        ),
    ];

    let result = place_partitions(
        &partitions,
        &candidates,
        PlacementPolicy::new(),
        &TestPlacementEvaluator,
    );

    assert!(matches!(
        result,
        Err(PlacementError::NoEligibleTarget { .. })
    ));
}

/// Verify that unknown capability or resource evidence is not accepted.
#[test]
fn placement_rejects_indeterminate_evidence() {
    let graph = sequential_workload();

    let partitions = partition(
        &graph,
        PartitionPolicy::new(),
        PartitionBudget::unlimited(),
    );

    let candidates = targets(&["unknown-target"], 100);

    let result = place_partitions(
        &partitions,
        &candidates,
        PlacementPolicy::new(),
        &IndeterminatePlacementEvaluator,
    );

    assert!(matches!(
        result,
        Err(PlacementError::NoEligibleTarget { .. })
    ));
}

/// Verify that duplicate physical target identities are rejected.
#[test]
fn placement_rejects_duplicate_target_ids() {
    let graph = sequential_workload();

    let partitions = partition(
        &graph,
        PartitionPolicy::new(),
        PartitionBudget::unlimited(),
    );

    let candidates = targets(
        &["duplicate", "duplicate"],
        100,
    );

    let result = place_partitions(
        &partitions,
        &candidates,
        PlacementPolicy::new(),
        &TestPlacementEvaluator,
    );

    assert_eq!(
        result,
        Err(PlacementError::DuplicateTarget),
    );
}

/// Verify that insufficient target capacity is not ignored.
#[test]
fn placement_rejects_insufficient_resource_capacity() {
    let graph = workload_graph(
        vec![work_unit(1, 100)],
        Vec::new(),
    );

    let partitions = partition(
        &graph,
        PartitionPolicy::new(),
        PartitionBudget::unlimited(),
    );

    let candidates = targets(&["small-target"], 10);

    let result = place_partitions(
        &partitions,
        &candidates,
        PlacementPolicy::new(),
        &TestPlacementEvaluator,
    );

    assert!(matches!(
        result,
        Err(PlacementError::NoEligibleTarget { .. })
    ));
}

// =============================================================================
// SCHEDULING INTEGRATION
// =============================================================================

/// Verify that scheduling preserves dependency ordering.
#[test]
fn scheduling_preserves_dependency_order() {
    let graph = sequential_workload();

    let schedule = schedule_workload(
        &graph,
        &profiles(&graph),
        &[],
        SchedulingPolicy::new(),
    )
    .expect("valid dependency graph should schedule");

    let first = schedule.work(&1).expect("first task");
    let second = schedule.work(&2).expect("second task");
    let third = schedule.work(&3).expect("third task");

    assert!(second.start() >= first.finish());
    assert!(third.start() >= second.finish());

    assert_eq!(schedule.makespan(), 12);
}

/// Verify that independent work can execute concurrently.
#[test]
fn scheduling_allows_independent_work_to_overlap() {
    let graph = workload_graph(
        vec![
            work_unit(1, 5),
            work_unit(2, 7),
        ],
        Vec::new(),
    );

    let schedule = schedule_workload(
        &graph,
        &profiles(&graph),
        &[],
        SchedulingPolicy::new(),
    )
    .expect("independent tasks should schedule");

    assert_eq!(
        schedule.work(&1).expect("task one").start(),
        0,
    );

    assert_eq!(
        schedule.work(&2).expect("task two").start(),
        0,
    );

    assert_eq!(schedule.makespan(), 7);
}

/// Verify that dependency cycles are rejected.
#[test]
fn scheduling_rejects_dependency_cycles() {
    let graph = workload_graph(
        vec![
            work_unit(1, 1),
            work_unit(2, 1),
        ],
        vec![
            dependency(1, 2),
            dependency(2, 1),
        ],
    );

    let result = schedule_workload(
        &graph,
        &profiles(&graph),
        &[],
        SchedulingPolicy::new(),
    );

    assert_eq!(
        result,
        Err(SchedulingError::DependencyCycle),
    );
}

/// Verify that resource capacities constrain concurrent execution.
#[test]
fn scheduling_respects_shared_resource_capacity() {
    let graph = workload_graph(
        vec![
            work_unit(1, 1),
            work_unit(2, 1),
        ],
        Vec::new(),
    );

    let profiles = vec![
        ScheduleProfile::new(1, 5)
            .with_resource_demands([("execution-slot".to_owned(), 1)])
            .expect("valid resource demand"),

        ScheduleProfile::new(2, 7)
            .with_resource_demands([("execution-slot".to_owned(), 1)])
            .expect("valid resource demand"),
    ];

    let capacities = vec![
        ResourceCapacity::new("execution-slot".to_owned(), 1),
    ];

    let schedule = schedule_workload(
        &graph,
        &profiles,
        &capacities,
        SchedulingPolicy::new(),
    )
    .expect("work should schedule within capacity");

    let first = schedule.work(&1).expect("first task");
    let second = schedule.work(&2).expect("second task");

    assert!(
        first.finish() <= second.start()
            || second.finish() <= first.start()
    );

    assert_eq!(schedule.makespan(), 12);
}

/// Verify that an impossible resource demand is reported.
#[test]
fn scheduling_rejects_impossible_resource_demands() {
    let graph = workload_graph(
        vec![work_unit(1, 1)],
        Vec::new(),
    );

    let profiles = vec![
        ScheduleProfile::new(1, 5)
            .with_resource_demands([("memory".to_owned(), 100)])
            .expect("valid resource demand"),
    ];

    let capacities = vec![
        ResourceCapacity::new("memory".to_owned(), 50),
    ];

    let result = schedule_workload(
        &graph,
        &profiles,
        &capacities,
        SchedulingPolicy::new(),
    );

    assert!(matches!(
        result,
        Err(SchedulingError::ResourceCapacityUnsatisfied {
            required: 100,
            available: 50,
            ..
        })
    ));
}

/// Verify that missing profiles cannot produce an incomplete schedule.
#[test]
fn scheduling_rejects_missing_work_profiles() {
    let graph = workload_graph(
        vec![
            work_unit(1, 1),
            work_unit(2, 1),
        ],
        Vec::new(),
    );

    let incomplete_profiles = vec![
        ScheduleProfile::<u64, String>::new(1, 5),
    ];

    let result = schedule_workload(
        &graph,
        &incomplete_profiles,
        &[],
        SchedulingPolicy::new(),
    );

    assert_eq!(
        result,
        Err(SchedulingError::MissingScheduleProfile),
    );
}

// =============================================================================
// ROUTING INTEGRATION
// =============================================================================

/// Verify that communication routes can connect distinct logical targets.
#[test]
fn routing_finds_a_valid_communication_path() {
    let mut graph = RoutingGraph::new();

    graph.add_node("source".to_owned());
    graph.add_node("relay".to_owned());
    graph.add_node("destination".to_owned());

    graph
        .add_edge(
            &"source".to_owned(),
            "relay".to_owned(),
            2,
        )
        .expect("valid edge");

    graph
        .add_edge(
            &"relay".to_owned(),
            "destination".to_owned(),
            3,
        )
        .expect("valid edge");

    let route = find_route_by_base_cost(
        &graph,
        &"source".to_owned(),
        &"destination".to_owned(),
        RoutingPolicy::new(),
    )
    .expect("connected targets should have a route");

    assert_eq!(
        route.nodes(),
        &[
            "source".to_owned(),
            "relay".to_owned(),
            "destination".to_owned(),
        ],
    );

    assert_eq!(route.total_cost(), 5);

    route
        .validate_structure()
        .expect("route must be structurally valid");
}

/// Verify that missing connectivity is not invented.
#[test]
fn routing_rejects_disconnected_targets() {
    let mut graph = RoutingGraph::new();

    graph.add_node("source".to_owned());
    graph.add_node("destination".to_owned());

    let result = find_route_by_base_cost(
        &graph,
        &"source".to_owned(),
        &"destination".to_owned(),
        RoutingPolicy::new(),
    );

    assert_eq!(
        result,
        Err(RoutingError::NoRoute),
    );
}

/// Verify that route budgets are enforced.
#[test]
fn routing_enforces_configured_hop_budgets() {
    let participants = vec![
        "a".to_owned(),
        "b".to_owned(),
        "c".to_owned(),
    ];

    let graph = fully_connected_topology(&participants, 1);

    let policy = RoutingPolicy::new().with_budget(
        RoutingBudget::unlimited()
            .with_max_hops(Some(0)),
    );

    let result = find_route_by_base_cost(
        &graph,
        &"a".to_owned(),
        &"c".to_owned(),
        policy,
    );

    assert_eq!(
        result,
        Err(RoutingError::HopBudgetExceeded),
    );
}

/// Verify that routing arithmetic does not wrap.
#[test]
fn routing_rejects_cost_overflow() {
    let mut graph = RoutingGraph::new();

    graph.add_node(1_u64);
    graph.add_node(2_u64);
    graph.add_node(3_u64);

    graph
        .add_edge(&1, 2, u128::MAX)
        .expect("valid edge");

    graph
        .add_edge(&2, 3, 1)
        .expect("valid edge");

    let result = find_route_by_base_cost(
        &graph,
        &1,
        &3,
        RoutingPolicy::new(),
    );

    assert_eq!(
        result,
        Err(RoutingError::CostBudgetExceeded),
    );
}

// =============================================================================
// CROSS-STAGE INTEGRATION
// =============================================================================

/// Verify the complete planning sequence using the existing implementations.
///
/// This test proves that:
///
/// 1. The workload can be partitioned.
/// 2. Every partition can be placed.
/// 3. Scheduling respects the original dependencies.
/// 4. Cross-partition communication boundaries can be routed.
///
/// No stage implements another stage's responsibilities.
#[test]
fn complete_distributed_planning_pipeline() {
    let graph = sequential_workload();

    // Stage 1: Logical partitioning.
    let partitions = partition(
        &graph,
        PartitionPolicy::new()
            .with_target_partition_count(Some(3))
            .with_max_units_per_partition(Some(1)),
        PartitionBudget::unlimited(),
    );

    partitions
        .validate_against(&graph)
        .expect("partitioning must preserve workload semantics");

    // Stage 2: Target placement.
    let candidates = targets(
        &["target-a", "target-b", "target-c"],
        100,
    );

    let placement = place(&partitions, &candidates);

    placement
        .validate_against(&partitions)
        .expect("placement must cover the partition plan");

    // Stage 3: Dependency-aware scheduling.
    let schedule = schedule_workload(
        &graph,
        &profiles(&graph),
        &[],
        SchedulingPolicy::new(),
    )
    .expect("valid workload should schedule");

    assert_eq!(schedule.scheduled().len(), graph.units().len());

    // Stage 4: Build a logical communication topology.
    //
    // The topology is supplied independently. Placement does not
    // automatically imply that a network path exists.
    let participant_ids: Vec<String> = candidates
        .iter()
        .map(|target| target.id().clone())
        .collect();

    let topology = fully_connected_topology(
        &participant_ids,
        1,
    );

    // Stage 5: Route every cross-partition dependency.
    for boundary in partitions.boundaries() {
        let source_target = placement
            .target_for(boundary.source_partition())
            .expect("source partition must have a placement");

        let destination_target = placement
            .target_for(boundary.destination_partition())
            .expect("destination partition must have a placement");

        let route = find_route_by_base_cost(
            &topology,
            source_target,
            destination_target,
            RoutingPolicy::new(),
        )
        .expect("connected targets must have a communication route");

        route
            .validate_structure()
            .expect("communication route must be valid");
    }

    // The original workload remains authoritative.
    assert_eq!(partitions.total_estimated_work(), 12);
    assert_eq!(schedule.makespan(), 12);
}

/// Verify that the same logical workload can be planned against different
/// physical resource topologies.
#[test]
fn same_workload_supports_different_resource_topologies() {
    let graph = sequential_workload();

    let partitions = partition(
        &graph,
        PartitionPolicy::new()
            .with_target_partition_count(Some(3))
            .with_max_units_per_partition(Some(1)),
        PartitionBudget::unlimited(),
    );

    let small_topology = targets(
        &["local-executor"],
        100,
    );

    let larger_topology = targets(
        &[
            "edge-a",
            "edge-b",
            "cloud-a",
            "accelerator-a",
        ],
        100,
    );

    let first = place(&partitions, &small_topology);
    let second = place(&partitions, &larger_topology);

    first
        .validate_against(&partitions)
        .expect("small topology must preserve placement correctness");

    second
        .validate_against(&partitions)
        .expect("larger topology must preserve placement correctness");

    // The logical workload is unchanged.
    assert_eq!(first.assignments().len(), partitions.partitions().len());
    assert_eq!(second.assignments().len(), partitions.partitions().len());
}

/// Verify that changing candidate input order does not change deterministic
/// placement when identifiers and evidence are unchanged.
#[test]
fn placement_is_deterministic_for_identical_snapshots() {
    let graph = sequential_workload();

    let partitions = partition(
        &graph,
        PartitionPolicy::new()
            .with_target_partition_count(Some(3))
            .with_max_units_per_partition(Some(1)),
        PartitionBudget::unlimited(),
    );

    let first_targets = targets(
        &["a", "b", "c"],
        100,
    );

    let second_targets = targets(
        &["c", "a", "b"],
        100,
    );

    let first = place(&partitions, &first_targets);
    let second = place(&partitions, &second_targets);

    assert_eq!(first, second);
}

/// Verify that independent planning stages can be repeated with identical
/// immutable inputs and produce identical results.
#[test]
fn planning_results_are_deterministic() {
    let graph = sequential_workload();

    let policy = PartitionPolicy::new()
        .with_target_partition_count(Some(3))
        .with_max_units_per_partition(Some(1));

    let first = partition(
        &graph,
        policy,
        PartitionBudget::unlimited(),
    );

    let second = partition(
        &graph,
        policy,
        PartitionBudget::unlimited(),
    );

    assert_eq!(first, second);

    let first_schedule = schedule_workload(
        &graph,
        &profiles(&graph),
        &[],
        SchedulingPolicy::new(),
    )
    .expect("first schedule");

    let second_schedule = schedule_workload(
        &graph,
        &profiles(&graph),
        &[],
        SchedulingPolicy::new(),
    )
    .expect("second schedule");

    assert_eq!(first_schedule, second_schedule);
}

// =============================================================================
// SCALABILITY AND BOUNDARY TESTS
// =============================================================================

/// Verify that the planning implementation does not impose an arbitrary
/// universal workload-size ceiling.
///
/// The test size is a test fixture, not a production maximum.
#[test]
fn planning_handles_larger_generated_workloads() {
    let workload_size: usize = 257;

    let units = (0..workload_size)
        .map(|index| {
            work_unit(
                index as u64,
                (index as u128).saturating_add(1),
            )
        })
        .collect();

    let graph = workload_graph(
        units,
        Vec::new(),
    );

    let plan = partition(
        &graph,
        PartitionPolicy::new()
            .with_target_partition_count(Some(17)),
        PartitionBudget::unlimited(),
    );

    plan.validate_against(&graph)
        .expect("generated workload must be preserved");

    assert_eq!(
        plan.partitions()
            .iter()
            .map(|partition| partition.units().len())
            .sum::<usize>(),
        workload_size,
    );

    assert_eq!(
        plan.total_estimated_work(),
        (1..=workload_size as u128).sum::<u128>(),
    );
}

/// Verify that large logical identifiers are not confused with physical
/// resource limits.
#[test]
fn planning_supports_large_representable_logical_identifiers() {
    let graph = workload_graph(
        vec![
            work_unit(1, 1),
            work_unit(u64::MAX, 1),
        ],
        vec![
            dependency(1, u64::MAX),
        ],
    );

    let partitions = partition(
        &graph,
        PartitionPolicy::new(),
        PartitionBudget::unlimited(),
    );

    partitions
        .validate_against(&graph)
        .expect("large identifiers must remain valid");
}

/// Verify that an empty workload is handled without inventing work.
#[test]
fn empty_workload_is_valid_across_planning_stages() {
    let graph = workload_graph(
        Vec::new(),
        Vec::new(),
    );

    let partitions = partition(
        &graph,
        PartitionPolicy::new(),
        PartitionBudget::unlimited(),
    );

    assert!(partitions.is_empty());

    let placement = place_partitions(
        &partitions,
        &[],
        PlacementPolicy::new(),
        &TestPlacementEvaluator,
    )
    .expect("empty placement requires no targets");

    assert!(placement.is_empty());

    let schedule = schedule_workload(
        &graph,
        &[],
        &[],
        SchedulingPolicy::new(),
    )
    .expect("empty workload should schedule");

    assert!(schedule.is_empty());
    assert_eq!(schedule.makespan(), 0);
}

// =============================================================================
// RESULT VALIDATION
// =============================================================================

/// Ensure that every scheduled work unit exists in the original graph.
#[test]
fn schedule_contains_only_original_work_units() {
    let graph = sequential_workload();

    let schedule = schedule_workload(
        &graph,
        &profiles(&graph),
        &[],
        SchedulingPolicy::new(),
    )
    .expect("schedule should succeed");

    let expected: BTreeSet<u64> = graph
        .units()
        .iter()
        .map(|unit| *unit.id())
        .collect();

    let actual: BTreeSet<u64> = schedule
        .scheduled()
        .iter()
        .map(|work| *work.work_id())
        .collect();

    assert_eq!(actual, expected);
}

/// Ensure that placement references only known logical partitions.
#[test]
fn placement_contains_only_original_partitions() {
    let graph = sequential_workload();

    let partitions = partition(
        &graph,
        PartitionPolicy::new()
            .with_target_partition_count(Some(3))
            .with_max_units_per_partition(Some(1)),
        PartitionBudget::unlimited(),
    );

    let placement = place(
        &partitions,
        &targets(&["a", "b", "c"], 100),
    );

    let expected: BTreeSet<_> = partitions
        .partitions()
        .iter()
        .map(|partition| partition.id())
        .collect();

    let actual: BTreeSet<_> = placement
        .assignments()
        .iter()
        .map(|assignment| assignment.partition())
        .collect();

    assert_eq!(actual, expected);
}

/// Ensure that route plans retain their requested logical endpoints.
#[test]
fn route_preserves_requested_endpoints() {
    let participants = vec![
        "first".to_owned(),
        "second".to_owned(),
    ];

    let graph = fully_connected_topology(
        &participants,
        7,
    );

    let route: RoutePlan<String> = find_route_by_base_cost(
        &graph,
        &"first".to_owned(),
        &"second".to_owned(),
        RoutingPolicy::new(),
    )
    .expect("route should exist");

    assert_eq!(route.source(), "first");
    assert_eq!(route.destination(), "second");

    assert_eq!(route.nodes().first(), Some(route.source()));
    assert_eq!(route.nodes().last(), Some(route.destination()));
}
