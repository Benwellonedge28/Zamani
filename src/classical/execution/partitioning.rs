
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Classical Execution Partitioning
//!
//! Path: `src/classical/execution/partitioning.rs`
//! Edition: Rust 2021
//! Minimum Rust version: Rust 1.97.
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Represent and validate a partitioning of a target-independent
//! [`ExecutionPlan`] into explicitly assigned logical partitions.
//!
//! A partition is a logical grouping of plan steps. It is not a physical
//! machine, process, thread, device, memory region, or execution schedule.
//!
//! This module derives the dependencies between partitions from the original
//! plan's step dependencies. It never discards a step or silently changes
//! its assigned partition.
//!
//! ## Authority and ownership
//!
//! - `grammar/resources/placement.g4` owns reusable resource-placement syntax.
//! - `grammar/execution/placement.g4` owns execution-placement syntax.
//! - `grammar/distributed/placement.g4` owns distributed placement composition.
//! - The normative semantic specifications govern source meaning.
//! - `src/classical/execution/plan.rs` owns the target-independent plan and
//!   plan-step dependency model.
//! - `src/classical/execution/requirements.rs` aggregates canonical resource
//!   and capability requirements.
//! - `src/classical/execution/placement.rs` represents placement intent.
//! - `src/classical/execution/capabilities.rs` assesses supplied capability
//!   evidence.
//! - The canonical semantic subsystem owns resource and capability semantics.
//! - Downstream planners own target feasibility, physical placement,
//!   scheduling, routing, allocation, and execution.
//!
//! This module owns only logical partition assignments, their validation,
//! the resulting partition dependency graph, and deterministic graph ordering.
//!
//! ## Important boundaries
//!
//! This module does not:
//!
//! - parse source or construct AST nodes;
//! - define a second classical IR or universal IR;
//! - reinterpret source-level placement intent;
//! - discover hardware or authenticate target evidence;
//! - choose devices, nodes, workers, or physical addresses;
//! - infer resource capacity or claim target feasibility;
//! - schedule operations or execute partitions;
//! - silently relax requirements or change computation semantics.
//!
//! ## Partition assignment contract
//!
//! Every step in the input plan must have exactly one assignment.
//! Assignments to unknown steps and duplicate assignments are errors.
//! The caller chooses logical partition identifiers explicitly.
//!
//! An empty plan with no assignments produces an empty partitioned plan.
//! An empty plan with assignments is invalid because the assigned steps do
//! not exist in the plan.
//!
//! ## Dependency contract
//!
//! An edge from partition `A` to partition `B` means at least one step in
//! `B` depends on a step in `A`. It does not imply that all steps within a
//! partition are mutually dependent or that the partition is an atomic
//! runtime transaction.
//!
//! Dependencies between steps assigned to the same partition remain part
//! of the original `ExecutionPlan`. They are not repeated as inter-partition
//! edges.
//!
//! A partition-level dependency cycle is rejected. Such a cycle can occur
//! even when the original step-level plan is acyclic: grouping non-contiguous
//! steps into indivisible partitions can introduce a cycle in the quotient
//! graph. The caller must choose different partitions or use a downstream
//! coordination/decomposition design that preserves the original semantics.
//!
//! ## Determinism
//!
//! The source plan's deterministic topological order determines the order of
//! steps stored inside each partition. Partition identifiers and dependency
//! edges use ordered collections. Partition topological ordering uses the
//! lexicographically smallest currently ready partition identifier.
//!
//! This guarantees deterministic results for the same plan and assignments;
//! it does not guarantee deterministic runtime execution.
//!
//! ## Scalability
//!
//! This module imposes no universal maximum on steps, partitions, or edges.
//! Memory and processing costs grow with the supplied graph and available
//! resources. Representable integer limits, allocation failure, and explicit
//! operational budgets remain real constraints.
//!
//! "Scale to infinity given resources" means no artificial semantic ceiling,
//! not literal infinite memory or computation.
//!
//! ## Integration status
//!
//! This file depends on the existing execution-plan API. The parent execution
//! module must declare `partitioning` before this module can be imported by
//! downstream code. Compilation and repository tests must pass before this
//! module can be marked verified.

use core::fmt;
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;

use super::plan::{ExecutionPlan, ExecutionPlanError, StepId};

/// Result type for logical partitioning operations.
pub type PartitioningResult<T> = Result<T, PartitioningError>;

/// A logical partition identifier.
///
/// This identifier groups computation. It is not a physical resource ID,
/// device ID, worker ID, process ID, or globally unique identifier.
///
/// The accepted string is non-empty, is not whitespace-only, and contains
/// no control characters. Namespace resolution and uniqueness beyond a
/// single partitioned plan are the caller's responsibility.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PartitionId(String);

impl PartitionId {
    /// Construct a logical partition identifier.
    ///
    /// The original spelling is preserved. This constructor does not resolve
    /// namespaces, normalize case, or infer physical meaning.
    ///
    /// # Errors
    ///
    /// Returns [`PartitioningError::InvalidPartitionId`] if the supplied
    /// value is empty, whitespace-only, or contains a control character.
    pub fn new(value: impl Into<String>) -> PartitioningResult<Self> {
        let value = value.into();

        if value.trim().is_empty() || value.chars().any(char::is_control) {
            return Err(PartitioningError::InvalidPartitionId { value });
        }

        Ok(Self(value))
    }

    /// Return the original partition identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for PartitionId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// An explicit assignment of one plan step to one logical partition.
///
/// Assignments are declarative input. They do not move operations, allocate
/// resources, or establish that the partition can execute on any target.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PartitionAssignment {
    step: StepId,
    partition: PartitionId,
}

impl PartitionAssignment {
    /// Create an assignment for a plan step.
    #[must_use]
    pub const fn new(step: StepId, partition: PartitionId) -> Self {
        Self { step, partition }
    }

    /// Return the assigned plan step.
    #[must_use]
    pub const fn step(&self) -> &StepId {
        &self.step
    }

    /// Return the logical partition selected for the step.
    #[must_use]
    pub const fn partition(&self) -> &PartitionId {
        &self.partition
    }
}

/// A directed dependency between two distinct logical partitions.
///
/// `prerequisite` must be processed before `dependent` can be considered
/// ready under the partition dependency graph. This does not itself define
/// scheduling, concurrency, synchronization, or runtime completion semantics.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PartitionDependency {
    prerequisite: PartitionId,
    dependent: PartitionId,
}

impl PartitionDependency {
    /// Return the partition that must precede the dependent partition.
    #[must_use]
    pub const fn prerequisite(&self) -> &PartitionId {
        &self.prerequisite
    }

    /// Return the partition that depends on the prerequisite partition.
    #[must_use]
    pub const fn dependent(&self) -> &PartitionId {
        &self.dependent
    }
}

/// One logical partition and its member steps.
///
/// Member steps are stored in the deterministic topological order derived
/// from the source execution plan. This order is not a runtime schedule.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LogicalPartition {
    id: PartitionId,
    steps: Vec<StepId>,
}

impl LogicalPartition {
    /// Return this partition's identifier.
    #[must_use]
    pub const fn id(&self) -> &PartitionId {
        &self.id
    }

    /// Return member steps in deterministic plan topological order.
    #[must_use]
    pub fn steps(&self) -> &[StepId] {
        &self.steps
    }

    /// Return the number of member steps.
    #[must_use]
    pub fn len(&self) -> usize {
        self.steps.len()
    }

    /// Return whether the partition has no member steps.
    ///
    /// Partitions produced by [`partition_plan`] are never empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.steps.is_empty()
    }
}

/// A partitioned view of an existing execution plan.
///
/// This structure references plan-local step identifiers rather than
/// cloning or redefining operation semantics. The original [`ExecutionPlan`]
/// remains the authority for step contents and step-level dependencies.
///
/// The partitioned view records the partition membership and the dependency
/// graph induced by the explicit assignments. It is not an executable plan.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PartitionedPlan {
    source_plan_id: String,
    partitions: BTreeMap<PartitionId, LogicalPartition>,
    step_to_partition: BTreeMap<StepId, PartitionId>,
    dependencies: BTreeSet<PartitionDependency>,
    topological_order: Vec<PartitionId>,
}

impl PartitionedPlan {
    /// Return the identifier of the source execution plan.
    #[must_use]
    pub fn source_plan_id(&self) -> &str {
        &self.source_plan_id
    }

    /// Return all partitions, keyed by identifier.
    ///
    /// Iteration order is lexicographic by partition identifier.
    #[must_use]
    pub fn partitions(&self) -> &BTreeMap<PartitionId, LogicalPartition> {
        &self.partitions
    }

    /// Return one partition by identifier.
    #[must_use]
    pub fn partition(&self, id: &PartitionId) -> Option<&LogicalPartition> {
        self.partitions.get(id)
    }

    /// Return the partition assigned to a plan step, if present.
    #[must_use]
    pub fn partition_for_step(&self, step: &StepId) -> Option<&PartitionId> {
        self.step_to_partition.get(step)
    }

    /// Return the partition-level dependencies in deterministic order.
    #[must_use]
    pub fn dependencies(&self) -> &BTreeSet<PartitionDependency> {
        &self.dependencies
    }

    /// Return the deterministic topological ordering of the partitions.
    ///
    /// The ordering is valid for the derived partition dependency graph.
    /// It does not schedule individual operations or promise a particular
    /// runtime interleaving.
    #[must_use]
    pub fn topological_order(&self) -> &[PartitionId] {
        &self.topological_order
    }

    /// Return the number of logical partitions.
    #[must_use]
    pub fn len(&self) -> usize {
        self.partitions.len()
    }

    /// Return whether the partitioned plan contains no partitions.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.partitions.is_empty()
    }

    /// Validate internal membership and dependency invariants.
    ///
    /// This method validates only the partitioned view. It does not prove
    /// target feasibility, resource sufficiency, or runtime correctness.
    pub fn validate(&self) -> PartitioningResult<()> {
        let mut seen_steps = BTreeSet::new();

        for (id, partition) in &self.partitions {
            if id != &partition.id {
                return Err(PartitioningError::InconsistentPartitionIdentity {
                    key: id.clone(),
                    value: partition.id.clone(),
                });
            }

            if partition.steps.is_empty() {
                return Err(PartitioningError::EmptyPartition {
                    partition: id.clone(),
                });
            }

            for step in &partition.steps {
                if !seen_steps.insert(step.clone()) {
                    return Err(PartitioningError::DuplicateStepMembership {
                        step: step.clone(),
                    });
                }

                match self.step_to_partition.get(step) {
                    Some(mapped) if mapped == id => {}
                    _ => {
                        return Err(PartitioningError::InconsistentStepMapping {
                            step: step.clone(),
                            expected: id.clone(),
                        });
                    }
                }
            }
        }

        for (step, partition) in &self.step_to_partition {
            if !self.partitions.contains_key(partition) {
                return Err(PartitioningError::UnknownAssignedPartition {
                    step: step.clone(),
                    partition: partition.clone(),
                });
            }

            if !seen_steps.contains(step) {
                return Err(PartitioningError::MissingStepMembership {
                    step: step.clone(),
                });
            }
        }

        for dependency in &self.dependencies {
            if dependency.prerequisite == dependency.dependent {
                return Err(PartitioningError::SelfPartitionDependency {
                    partition: dependency.prerequisite.clone(),
                });
            }

            if !self.partitions.contains_key(&dependency.prerequisite) {
                return Err(PartitioningError::UnknownDependencyPartition {
                    partition: dependency.prerequisite.clone(),
                });
            }

            if !self.partitions.contains_key(&dependency.dependent) {
                return Err(PartitioningError::UnknownDependencyPartition {
                    partition: dependency.dependent.clone(),
                });
            }
        }

        let recomputed_order = topological_order(
            self.partitions.keys().cloned().collect(),
            &self.dependencies,
        )?;

        if recomputed_order != self.topological_order {
            return Err(PartitioningError::InconsistentTopologicalOrder);
        }

        Ok(())
    }
}

/// Errors produced while constructing or validating a partitioned plan.
///
/// Variants are machine-readable. Consumers must branch on variants rather
/// than parse display messages.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum PartitioningError {
    /// The source execution plan failed structural validation.
    InvalidExecutionPlan(ExecutionPlanError),

    /// A partition identifier is empty, whitespace-only, or contains a
    /// control character.
    InvalidPartitionId {
        /// The rejected identifier.
        value: String,
    },

    /// An assignment refers to a step not present in the source plan.
    UnknownStep {
        /// The unknown step.
        step: StepId,
    },

    /// A step has more than one explicit assignment.
    DuplicateAssignment {
        /// The duplicated step.
        step: StepId,
    },

    /// A source-plan step has no explicit assignment.
    MissingAssignment {
        /// The unassigned step.
        step: StepId,
    },

    /// Grouping the steps creates a cycle between logical partitions.
    ///
    /// The identifiers are the partitions left unresolved by deterministic
    /// topological ordering. This is a grouping error, not necessarily a
    /// cycle in the original step-level plan.
    PartitionDependencyCycle {
        /// Partitions that could not be topologically ordered.
        unresolved_partitions: Vec<PartitionId>,
    },

    /// A partition map key differs from the identifier stored in its value.
    InconsistentPartitionIdentity {
        /// Map key.
        key: PartitionId,
        /// Identifier stored in the partition value.
        value: PartitionId,
    },

    /// A produced partition has no member steps.
    EmptyPartition {
        /// The empty partition.
        partition: PartitionId,
    },

    /// A step occurs in more than one partition.
    DuplicateStepMembership {
        /// The duplicated step.
        step: StepId,
    },

    /// A partition's member step disagrees with the step-to-partition map.
    InconsistentStepMapping {
        /// The step with the inconsistent mapping.
        step: StepId,
        /// Partition containing the step.
        expected: PartitionId,
    },

    /// A step-to-partition mapping references a partition that does not exist.
    UnknownAssignedPartition {
        /// The mapped step.
        step: StepId,
        /// The missing partition.
        partition: PartitionId,
    },

    /// A mapped step does not occur in the partition's member list.
    MissingStepMembership {
        /// The missing step.
        step: StepId,
    },

    /// A partition depends on itself.
    SelfPartitionDependency {
        /// The partition with the invalid dependency.
        partition: PartitionId,
    },

    /// A dependency references a partition absent from the partition map.
    UnknownDependencyPartition {
        /// The missing partition.
        partition: PartitionId,
    },

    /// The stored topological order disagrees with the dependency graph.
    InconsistentTopologicalOrder,
}

impl fmt::Display for PartitioningError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidExecutionPlan(error) => {
                write!(formatter, "invalid source execution plan: {error}")
            }
            Self::InvalidPartitionId { value } => {
                write!(formatter, "invalid logical partition identifier `{value}`")
            }
            Self::UnknownStep { step } => {
                write!(formatter, "partition assignment references unknown step `{step}`")
            }
            Self::DuplicateAssignment { step } => {
                write!(formatter, "step `{step}` has more than one partition assignment")
            }
            Self::MissingAssignment { step } => {
                write!(formatter, "step `{step}` has no explicit partition assignment")
            }
            Self::PartitionDependencyCycle {
                unresolved_partitions,
            } => write!(
                formatter,
                "partition assignments create a dependency cycle involving {} partition(s): {}",
                unresolved_partitions.len(),
                unresolved_partitions
                    .iter()
                    .map(PartitionId::as_str)
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Self::InconsistentPartitionIdentity { key, value } => write!(
                formatter,
                "partition map key `{key}` differs from stored partition ID `{value}`"
            ),
            Self::EmptyPartition { partition } => {
                write!(formatter, "logical partition `{partition}` is empty")
            }
            Self::DuplicateStepMembership { step } => {
                write!(formatter, "step `{step}` occurs in more than one partition")
            }
            Self::InconsistentStepMapping { step, expected } => write!(
                formatter,
                "step `{step}` is listed in partition `{expected}` but has a different mapping"
            ),
            Self::UnknownAssignedPartition { step, partition } => write!(
                formatter,
                "step `{step}` is mapped to unknown partition `{partition}`"
            ),
            Self::MissingStepMembership { step } => {
                write!(formatter, "mapped step `{step}` is absent from partition membership")
            }
            Self::SelfPartitionDependency { partition } => {
                write!(formatter, "partition `{partition}` depends on itself")
            }
            Self::UnknownDependencyPartition { partition } => {
                write!(formatter, "dependency references unknown partition `{partition}`")
            }
            Self::InconsistentTopologicalOrder => {
                formatter.write_str("stored partition topological order is inconsistent")
            }
        }
    }
}

impl Error for PartitioningError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidExecutionPlan(error) => Some(error),
            _ => None,
        }
    }
}

/// Build a deterministic logical partitioning of an execution plan.
///
/// Each step must be assigned exactly once. The function validates the source
/// plan, validates all assignments, preserves the source plan's dependency
/// order within partitions, derives cross-partition dependencies, and rejects
/// partition-level cycles.
///
/// This function does not decide which partition should contain a step.
/// Assignment policy belongs to the caller or to a future higher-level
/// partitioning strategy that can justify its decisions from explicit
/// constraints and cost/resource models.
///
/// # Errors
///
/// Returns a structured error if the plan is invalid, an assignment is
/// missing, duplicated, or references an unknown step, or the derived
/// partition dependency graph is cyclic.
pub fn partition_plan(
    plan: &ExecutionPlan,
    assignments: impl IntoIterator<Item = PartitionAssignment>,
) -> PartitioningResult<PartitionedPlan> {
    let source_order = plan
        .topological_order()
        .map_err(PartitioningError::InvalidExecutionPlan)?;

    let known_steps: BTreeSet<StepId> =
        plan.steps().iter().map(|step| step.id().clone()).collect();

    let mut step_to_partition = BTreeMap::new();

    for assignment in assignments {
        let step = assignment.step;
        let partition = assignment.partition;

        if !known_steps.contains(&step) {
            return Err(PartitioningError::UnknownStep { step });
        }

        if step_to_partition.insert(step.clone(), partition).is_some() {
            return Err(PartitioningError::DuplicateAssignment { step });
        }
    }

    for step in &source_order {
        if !step_to_partition.contains_key(step) {
            return Err(PartitioningError::MissingAssignment {
                step: step.clone(),
            });
        }
    }

    let mut partitions: BTreeMap<PartitionId, LogicalPartition> = BTreeMap::new();

    for step in &source_order {
        let partition_id = step_to_partition
            .get(step)
            .expect("every source step was checked for an assignment")
            .clone();

        partitions
            .entry(partition_id.clone())
            .or_insert_with(|| LogicalPartition {
                id: partition_id,
                steps: Vec::new(),
            })
            .steps
            .push(step.clone());
    }

    let partition_by_step = &step_to_partition;
    let mut dependencies = BTreeSet::new();

    for plan_step in plan.steps() {
        let dependent_partition = partition_by_step
            .get(plan_step.id())
            .expect("every plan step was checked for an assignment");

        for prerequisite_step in plan_step.dependencies() {
            let prerequisite_partition = partition_by_step
                .get(prerequisite_step)
                .expect("the validated plan contains every dependency");

            if prerequisite_partition != dependent_partition {
                dependencies.insert(PartitionDependency {
                    prerequisite: prerequisite_partition.clone(),
                    dependent: dependent_partition.clone(),
                });
            }
        }
    }

    let ordered_partitions = topological_order(
        partitions.keys().cloned().collect(),
        &dependencies,
    )?;

    let result = PartitionedPlan {
        source_plan_id: plan.id().to_owned(),
        partitions,
        step_to_partition,
        dependencies,
        topological_order: ordered_partitions,
    };

    result.validate()?;
    Ok(result)
}

/// Topologically order a directed acyclic partition graph.
///
/// Among ready nodes, the lexicographically smallest identifier is selected.
/// This uses an iterative algorithm rather than recursive graph traversal,
/// so long dependency chains do not consume call-stack depth.
fn topological_order(
    partition_ids: Vec<PartitionId>,
    dependencies: &BTreeSet<PartitionDependency>,
) -> PartitioningResult<Vec<PartitionId>> {
    let mut outgoing: BTreeMap<PartitionId, BTreeSet<PartitionId>> = partition_ids
        .iter()
        .cloned()
        .map(|id| (id, BTreeSet::new()))
        .collect();

    let mut indegree: BTreeMap<PartitionId, usize> = partition_ids
        .into_iter()
        .map(|id| (id, 0))
        .collect();

    for dependency in dependencies {
        if dependency.prerequisite == dependency.dependent {
            return Err(PartitioningError::SelfPartitionDependency {
                partition: dependency.prerequisite.clone(),
            });
        }

        let successors = outgoing
            .get_mut(&dependency.prerequisite)
            .ok_or_else(|| PartitioningError::UnknownDependencyPartition {
                partition: dependency.prerequisite.clone(),
            })?;

        if !indegree.contains_key(&dependency.dependent) {
            return Err(PartitioningError::UnknownDependencyPartition {
                partition: dependency.dependent.clone(),
            });
        }

        if successors.insert(dependency.dependent.clone()) {
            let degree = indegree
                .get_mut(&dependency.dependent)
                .expect("dependent partition existence was checked");

            *degree = degree.checked_add(1).ok_or_else(|| {
                PartitioningError::PartitionDependencyCycle {
                    unresolved_partitions: vec![dependency.dependent.clone()],
                }
            })?;
        }
    }

    let mut ready: BTreeSet<PartitionId> = indegree
        .iter()
        .filter_map(|(id, degree)| (*degree == 0).then_some(id.clone()))
        .collect();

    let mut ordered = Vec::with_capacity(indegree.len());

    while let Some(next) = ready.pop_first() {
        ordered.push(next.clone());

        if let Some(successors) = outgoing.get(&next) {
            for successor in successors {
                let degree = indegree
                    .get_mut(successor)
                    .expect("all successors have an indegree entry");

                *degree -= 1;

                if *degree == 0 {
                    ready.insert(successor.clone());
                }
            }
        }
    }

    if ordered.len() != indegree.len() {
        let unresolved_partitions = indegree
            .into_iter()
            .filter_map(|(id, degree)| (degree > 0).then_some(id))
            .collect();

        return Err(PartitioningError::PartitionDependencyCycle {
            unresolved_partitions,
        });
    }

    Ok(ordered)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::classical::execution::plan::{PlanStep, StepId};

    fn step_id(value: &str) -> StepId {
        StepId::new(value).expect("test step ID should be valid")
    }

    fn partition_id(value: &str) -> PartitionId {
        PartitionId::new(value).expect("test partition ID should be valid")
    }

    fn step(id: &str) -> PlanStep {
        PlanStep::new(step_id(id), "classical.test")
            .expect("test operation kind should be valid")
    }

    fn assignment(step: &str, partition: &str) -> PartitionAssignment {
        PartitionAssignment::new(step_id(step), partition_id(partition))
    }

    fn simple_plan() -> ExecutionPlan {
        let mut plan =
            ExecutionPlan::new("partition-test").expect("test plan ID should be valid");

        let first = step("a");
        let mut second = step("b");
        second.depends_on(step_id("a"));

        let mut third = step("c");
        third.depends_on(step_id("b"));

        plan.add_step(first);
        plan.add_step(second);
        plan.add_step(third);
        plan
    }

    #[test]
    fn partitions_a_linear_plan_and_preserves_dependencies() {
        let plan = simple_plan();
        let result = partition_plan(
            &plan,
            [
                assignment("a", "left"),
                assignment("b", "middle"),
                assignment("c", "right"),
            ],
        )
        .expect("valid partition assignment should succeed");

        assert_eq!(result.len(), 3);
        assert_eq!(
            result
                .topological_order()
                .iter()
                .map(PartitionId::as_str)
                .collect::<Vec<_>>(),
            vec!["left", "middle", "right"]
        );

        assert_eq!(result.dependencies().len(), 2);
        result.validate().expect("result invariants should hold");
    }

    #[test]
    fn dependencies_inside_one_partition_are_not_duplicated() {
        let plan = simple_plan();
        let result = partition_plan(
            &plan,
            [
                assignment("a", "whole"),
                assignment("b", "whole"),
                assignment("c", "whole"),
            ],
        )
        .expect("one logical partition should be valid");

        assert_eq!(result.len(), 1);
        assert!(result.dependencies().is_empty());
        assert_eq!(
            result.partitions().values().next().unwrap().steps().len(),
            3
        );
    }

    #[test]
    fn missing_assignment_is_an_error() {
        let error = partition_plan(
            &simple_plan(),
            [assignment("a", "one"), assignment("b", "two")],
        )
        .expect_err("missing assignment must not be inferred");

        assert!(matches!(
            error,
            PartitioningError::MissingAssignment { .. }
        ));
    }

    #[test]
    fn duplicate_assignment_is_an_error() {
        let error = partition_plan(
            &simple_plan(),
            [
                assignment("a", "one"),
                assignment("a", "two"),
                assignment("b", "two"),
                assignment("c", "three"),
            ],
        )
        .expect_err("a step must not have two assignments");

        assert!(matches!(
            error,
            PartitioningError::DuplicateAssignment { .. }
        ));
    }

    #[test]
    fn unknown_step_is_an_error() {
        let error = partition_plan(
            &simple_plan(),
            [
                assignment("a", "one"),
                assignment("b", "two"),
                assignment("c", "three"),
                assignment("not-in-plan", "four"),
            ],
        )
        .expect_err("unknown steps must be rejected");

        assert!(matches!(error, PartitioningError::UnknownStep { .. }));
    }

    #[test]
    fn grouping_can_introduce_a_partition_cycle() {
        let mut plan =
            ExecutionPlan::new("cycle-test").expect("test plan ID should be valid");

        plan.add_step(step("a"));

        let mut second = step("b");
        second.depends_on(step_id("a"));
        plan.add_step(second);

        let mut third = step("c");
        third.depends_on(step_id("b"));
        plan.add_step(third);

        let error = partition_plan(
            &plan,
            [
                assignment("a", "alpha"),
                assignment("b", "beta"),
                assignment("c", "alpha"),
            ],
        )
        .expect_err("non-contiguous grouping should expose the induced cycle");

        assert!(matches!(
            error,
            PartitioningError::PartitionDependencyCycle { .. }
        ));
    }

    #[test]
    fn empty_plan_and_empty_assignments_are_valid() {
        let plan =
            ExecutionPlan::new("empty-plan").expect("test plan ID should be valid");

        let result = partition_plan(&plan, [])
            .expect("an empty plan should produce an empty partitioning");

        assert!(result.is_empty());
        assert!(result.topological_order().is_empty());
        result.validate().expect("empty result is consistent");
    }

    #[test]
    fn partition_identifiers_are_validated_without_fixed_catalogues() {
        assert!(PartitionId::new("").is_err());
        assert!(PartitionId::new("   ").is_err());
        assert!(PartitionId::new("partition\nname").is_err());

        assert_eq!(
            PartitionId::new("future::logical/partition-α")
                .expect("extensible identifier should be accepted")
                .as_str(),
            "future::logical/partition-α"
        );
    }

    #[test]
    fn member_order_follows_source_plan_topological_order() {
        let mut plan =
            ExecutionPlan::new("order-test").expect("test plan ID should be valid");

        let mut z = step("z");
        z.depends_on(step_id("a"));

        plan.add_step(z);
        plan.add_step(step("a"));

        let result = partition_plan(
            &plan,
            [assignment("z", "group"), assignment("a", "group")],
        )
        .expect("valid grouping should succeed");

        let member_ids: Vec<_> = result
            .partitions()
            .values()
            .next()
            .unwrap()
            .steps()
            .iter()
            .map(StepId::as_str)
            .collect();

        assert_eq!(member_ids, vec!["a", "z"]);
    }

    #[test]
    fn independent_partitions_have_stable_lexicographic_order() {
        let mut plan =
            ExecutionPlan::new("independent-test").expect("test plan ID should be valid");

        plan.add_step(step("a"));
        plan.add_step(step("b"));
        plan.add_step(step("c"));

        let first = partition_plan(
            &plan,
            [
                assignment("a", "zeta"),
                assignment("b", "alpha"),
                assignment("c", "middle"),
            ],
        )
        .expect("partitioning should succeed");

        let second = partition_plan(
            &plan,
            [
                assignment("c", "middle"),
                assignment("a", "zeta"),
                assignment("b", "alpha"),
            ],
        )
        .expect("assignment input order must not change graph ordering");

        assert_eq!(first.topological_order(), second.topological_order());
        assert_eq!(
            first
                .topological_order()
                .iter()
                .map(PartitionId::as_str)
                .collect::<Vec<_>>(),
            vec!["alpha", "middle", "zeta"]
        );
    }

    #[test]
    fn supports_many_logical_partitions_without_a_semantic_ceiling() {
        let mut plan =
            ExecutionPlan::new("growth-test").expect("test plan ID should be valid");

        let mut assignments = Vec::new();

        for index in 0..4096 {
            let name = format!("step-{index}");
            let partition = format!("partition-{index}");

            plan.add_step(step(&name));
            assignments.push(PartitionAssignment::new(
                step_id(&name),
                partition_id(&partition),
            ));
        }

        let result = partition_plan(&plan, assignments)
            .expect("partition count should be limited by actual resources");

        assert_eq!(result.len(), 4096);
        assert_eq!(result.topological_order().len(), 4096);
        result.validate().expect("large result should remain consistent");
    }
}
