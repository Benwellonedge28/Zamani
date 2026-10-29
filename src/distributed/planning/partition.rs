
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Distributed workload partitioning
//!
//! Path: `src/distributed/planning/partition.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Responsibility
//!
//! Partition a validated, target-independent workload graph into logical
//! execution partitions while preserving every work unit and dependency.
//!
//! This module owns:
//!
//! - Generic workload-graph validation.
//! - Deterministic partition construction.
//! - Configurable partitioning policies.
//! - Explicit operational budgets.
//! - Cross-partition dependency and communication-boundary reporting.
//! - Structural validation of the resulting partition plan.
//!
//! This module does not own:
//!
//! - Source syntax, ASTs, name resolution, or semantic analysis.
//! - ZUIR, ZCIR, quantum::ir, HDLIR, or other domain-specific IRs.
//! - Resource discovery, capability discovery, or feasibility evaluation.
//! - Physical placement, target selection, scheduling, or network routing.
//! - Backend selection, device allocation, or runtime execution.
//!
//! ## Integration
//!
//! The intended planning sequence is:
//!
//! 1. Canonical frontend and semantic analysis.
//! 2. Canonical IR and distributed requirement extraction.
//! 3. Capability and resource feasibility analysis.
//! 4. Workload-graph construction by the owning planning adapter.
//! 5. Partitioning through [`partition_workload`].
//! 6. Placement, scheduling, routing, and execution.
//!
//! The workload graph is a generic planning input. An integration adapter
//! must derive it from the actual canonical representation and preserve
//! source provenance and domain-specific constraints.
//!
//! ## Scalability and correctness
//!
//! No universal maximum number of work units, dependencies, or partitions
//! is imposed. Optional budgets are operational policies, not language
//! semantic limits.
//!
//! All work units and dependencies must be represented in the result.
//! Exceeding a budget or encountering an invalid graph produces an error;
//! partial results are never reported as complete plans.
//!
//! This implementation uses a deterministic greedy heuristic. It does not
//! claim globally optimal graph partitioning. Optimal partitioning is
//! generally computationally expensive, and the best objective depends on
//! the workload and realization policy.
//!
//! A partition is a logical grouping, not a physical machine, worker,
//! processor, quantum device, or network endpoint.

use core::fmt;
use std::collections::{BTreeMap, BTreeSet};

/// Work estimate used by the partitioning heuristic.
///
/// The value is an abstract, non-negative cost supplied by the integration
/// layer. Its unit must be consistent within one workload. It is not
/// automatically interpreted as bytes, instructions, seconds, or FLOPs.
pub type WorkEstimate = u128;

/// Logical partition identifier.
///
/// This identifies a partition within one generated plan. It is not a
/// persistent participant identity or physical resource identifier.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PartitionId(u64);

impl PartitionId {
    /// Returns the numeric identifier within the generated plan.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }
}

impl fmt::Display for PartitionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "partition-{}", self.0)
    }
}

/// One indivisible logical work unit.
///
/// The identifier is supplied by the owning workload adapter. The
/// partitioner does not reinterpret it as an IR value, AST node, or device
/// identifier.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkUnit<Id> {
    id: Id,
    estimated_work: WorkEstimate,
    affinity: Option<String>,
}

impl<Id> WorkUnit<Id> {
    /// Creates a work unit with an optional affinity key.
    ///
    /// Work units sharing an affinity key are preferentially grouped
    /// together. Affinity is a heuristic, not a correctness constraint.
    #[must_use]
    pub fn new(
        id: Id,
        estimated_work: WorkEstimate,
        affinity: Option<String>,
    ) -> Self {
        Self {
            id,
            estimated_work,
            affinity,
        }
    }

    /// Returns the logical work-unit identifier.
    #[must_use]
    pub fn id(&self) -> &Id {
        &self.id
    }

    /// Returns the supplied abstract work estimate.
    #[must_use]
    pub const fn estimated_work(&self) -> WorkEstimate {
        self.estimated_work
    }

    /// Returns the optional grouping preference.
    #[must_use]
    pub fn affinity(&self) -> Option<&str> {
        self.affinity.as_deref()
    }
}

/// Dependency between two logical work units.
///
/// The direction is from `source` to `destination`: the destination depends
/// on the source according to the semantics supplied by the integration
/// layer.
///
/// A dependency is retained regardless of whether both endpoints belong
/// to one partition or different partitions.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Dependency<Id> {
    source: Id,
    destination: Id,
    communication_cost: WorkEstimate,
}

impl<Id> Dependency<Id> {
    /// Creates a dependency with an optional abstract communication cost.
    ///
    /// A zero cost is valid and means that the caller supplied no positive
    /// communication penalty for this edge. It does not mean that physical
    /// communication is necessarily free.
    #[must_use]
    pub fn new(
        source: Id,
        destination: Id,
        communication_cost: WorkEstimate,
    ) -> Self {
        Self {
            source,
            destination,
            communication_cost,
        }
    }

    /// Returns the source work-unit identifier.
    #[must_use]
    pub fn source(&self) -> &Id {
        &self.source
    }

    /// Returns the destination work-unit identifier.
    #[must_use]
    pub fn destination(&self) -> &Id {
        &self.destination
    }

    /// Returns the supplied abstract communication cost.
    #[must_use]
    pub const fn communication_cost(&self) -> WorkEstimate {
        self.communication_cost
    }
}

/// Explicit partitioning strategy.
///
/// Strategies can be extended without adding domain-specific hardware
/// enumerations to the distributed core.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[non_exhaustive]
pub enum PartitionStrategy {
    /// Prefer graph locality, then affinity, then lower accumulated work.
    ///
    /// Work units are considered in their canonical identifier order.
    #[default]
    LocalityFirst,

    /// Prefer lower accumulated work, then graph locality and affinity.
    ///
    /// This can be useful when load balance matters more than minimizing
    /// estimated cross-partition communication.
    BalanceFirst,
}

/// Optional operational budgets for one partitioning invocation.
///
/// These limits are not language-level restrictions. `None` means that
/// this budget dimension imposes no additional limit.
///
/// A budget violation returns an error instead of truncating the graph.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PartitionBudget {
    max_work_units: Option<usize>,
    max_dependencies: Option<usize>,
    max_partitions: Option<usize>,
}

impl PartitionBudget {
    /// Creates a budget without additional count limits.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self {
            max_work_units: None,
            max_dependencies: None,
            max_partitions: None,
        }
    }

    /// Sets the maximum input work-unit count.
    #[must_use]
    pub const fn with_max_work_units(mut self, limit: Option<usize>) -> Self {
        self.max_work_units = limit;
        self
    }

    /// Sets the maximum input dependency count.
    #[must_use]
    pub const fn with_max_dependencies(mut self, limit: Option<usize>) -> Self {
        self.max_dependencies = limit;
        self
    }

    /// Sets the maximum number of output partitions.
    #[must_use]
    pub const fn with_max_partitions(mut self, limit: Option<usize>) -> Self {
        self.max_partitions = limit;
        self
    }

    fn validate<Id>(
        self,
        workload: &WorkloadGraph<Id>,
    ) -> Result<(), PartitionError> {
        check_budget(
            "work units",
            workload.units.len(),
            self.max_work_units,
        )?;

        check_budget(
            "dependencies",
            workload.dependencies.len(),
            self.max_dependencies,
        )?;

        Ok(())
    }
}

impl Default for PartitionBudget {
    fn default() -> Self {
        Self::unlimited()
    }
}

/// Validated-by-construction input container.
///
/// The constructor intentionally preserves input order. The partitioning
/// algorithm canonicalizes units by identifier and dependencies by their
/// endpoints so equivalent input orderings produce equivalent plans.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkloadGraph<Id> {
    units: Vec<WorkUnit<Id>>,
    dependencies: Vec<Dependency<Id>>,
}

impl<Id> WorkloadGraph<Id> {
    /// Creates a workload graph from caller-supplied work units and edges.
    ///
    /// Structural validation occurs when the graph is partitioned. This
    /// allows graph construction to remain independent of partition policy.
    #[must_use]
    pub fn new(
        units: Vec<WorkUnit<Id>>,
        dependencies: Vec<Dependency<Id>>,
    ) -> Self {
        Self {
            units,
            dependencies,
        }
    }

    /// Returns the input work units.
    #[must_use]
    pub fn units(&self) -> &[WorkUnit<Id>] {
        &self.units
    }

    /// Returns the input dependencies.
    #[must_use]
    pub fn dependencies(&self) -> &[Dependency<Id>] {
        &self.dependencies
    }
}

/// Configurable policy for logical workload partitioning.
///
/// `target_partition_count` is a requested planning parameter, not a
/// statement that the environment has that many available workers.
///
/// `max_work_per_partition` is a caller-supplied estimate budget. It must
/// not be used to split an individual work unit; an indivisible unit that
/// exceeds the configured budget produces an explicit error.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PartitionPolicy {
    target_partition_count: Option<usize>,
    max_work_per_partition: Option<WorkEstimate>,
    max_units_per_partition: Option<usize>,
    strategy: PartitionStrategy,
}

impl PartitionPolicy {
    /// Creates a policy with no target count or per-partition restrictions.
    ///
    /// With no partition count or capacity restriction, the algorithm
    /// creates one partition for a non-empty workload. This is a valid
    /// local realization and does not imply that distribution is required.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            target_partition_count: None,
            max_work_per_partition: None,
            max_units_per_partition: None,
            strategy: PartitionStrategy::LocalityFirst,
        }
    }

    /// Requests a number of logical partitions.
    ///
    /// A value of zero is invalid when the workload is non-empty. For an
    /// empty workload, the result contains zero partitions.
    #[must_use]
    pub const fn with_target_partition_count(
        mut self,
        count: Option<usize>,
    ) -> Self {
        self.target_partition_count = count;
        self
    }

    /// Sets an optional maximum estimated work per partition.
    #[must_use]
    pub const fn with_max_work_per_partition(
        mut self,
        limit: Option<WorkEstimate>,
    ) -> Self {
        self.max_work_per_partition = limit;
        self
    }

    /// Sets an optional maximum number of work units per partition.
    #[must_use]
    pub const fn with_max_units_per_partition(
        mut self,
        limit: Option<usize>,
    ) -> Self {
        self.max_units_per_partition = limit;
        self
    }

    /// Selects the heuristic strategy.
    #[must_use]
    pub const fn with_strategy(mut self, strategy: PartitionStrategy) -> Self {
        self.strategy = strategy;
        self
    }
}

/// A dependency whose endpoints cross partition boundaries.
///
/// The dependency itself remains intact. A downstream routing or execution
/// stage determines how the cross-partition dependency is realized.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BoundaryEdge<Id> {
    source: Id,
    destination: Id,
    source_partition: PartitionId,
    destination_partition: PartitionId,
    communication_cost: WorkEstimate,
}

impl<Id> BoundaryEdge<Id> {
    /// Returns the source work-unit identifier.
    #[must_use]
    pub fn source(&self) -> &Id {
        &self.source
    }

    /// Returns the destination work-unit identifier.
    #[must_use]
    pub fn destination(&self) -> &Id {
        &self.destination
    }

    /// Returns the source partition.
    #[must_use]
    pub const fn source_partition(&self) -> PartitionId {
        self.source_partition
    }

    /// Returns the destination partition.
    #[must_use]
    pub const fn destination_partition(&self) -> PartitionId {
        self.destination_partition
    }

    /// Returns the estimated communication cost.
    #[must_use]
    pub const fn communication_cost(&self) -> WorkEstimate {
        self.communication_cost
    }
}

/// One logical partition in a generated plan.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkPartition<Id> {
    id: PartitionId,
    units: Vec<Id>,
    estimated_work: WorkEstimate,
}

impl<Id> WorkPartition<Id> {
    /// Returns this partition's identifier.
    #[must_use]
    pub const fn id(&self) -> PartitionId {
        self.id
    }

    /// Returns the member work-unit identifiers in canonical order.
    #[must_use]
    pub fn units(&self) -> &[Id] {
        &self.units
    }

    /// Returns the sum of member work estimates.
    #[must_use]
    pub const fn estimated_work(&self) -> WorkEstimate {
        self.estimated_work
    }
}

/// Complete output of the partitioning operation.
///
/// The plan retains every original dependency and separately identifies
/// those that cross partition boundaries. It does not certify resource
/// feasibility or select physical execution targets.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PartitionPlan<Id> {
    partitions: Vec<WorkPartition<Id>>,
    boundaries: Vec<BoundaryEdge<Id>>,
    dependency_count: usize,
    total_estimated_work: WorkEstimate,
}

impl<Id> PartitionPlan<Id> {
    /// Returns all partitions in stable identifier order.
    #[must_use]
    pub fn partitions(&self) -> &[WorkPartition<Id>] {
        &self.partitions
    }

    /// Returns all cross-partition dependency edges.
    #[must_use]
    pub fn boundaries(&self) -> &[BoundaryEdge<Id>] {
        &self.boundaries
    }

    /// Returns the number of input dependencies preserved by the plan.
    #[must_use]
    pub const fn dependency_count(&self) -> usize {
        self.dependency_count
    }

    /// Returns the checked sum of all work-unit estimates.
    #[must_use]
    pub const fn total_estimated_work(&self) -> WorkEstimate {
        self.total_estimated_work
    }

    /// Returns whether this plan contains no work units.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.partitions.is_empty()
    }

    /// Validates structural invariants against the supplied workload.
    ///
    /// This method is useful at integration boundaries and after any
    /// transformation that modifies a plan.
    ///
    /// # Errors
    ///
    /// Returns an error if units are missing, duplicated, unknown, or
    /// assigned inconsistently, or if dependency-boundary information
    /// does not match the supplied graph.
    pub fn validate_against(
        &self,
        workload: &WorkloadGraph<Id>,
    ) -> Result<(), PartitionError>
    where
        Id: Clone + Ord,
    {
        let mut expected_units = BTreeSet::new();

        for unit in &workload.units {
            if !expected_units.insert(unit.id.clone()) {
                return Err(PartitionError::DuplicateWorkUnit);
            }
        }

        let mut actual_units = BTreeSet::new();
        let mut owner_by_unit = BTreeMap::new();

        for partition in &self.partitions {
            let mut partition_work = 0_u128;

            for unit_id in &partition.units {
                if !expected_units.contains(unit_id) {
                    return Err(PartitionError::UnknownWorkUnit);
                }

                if !actual_units.insert(unit_id.clone()) {
                    return Err(PartitionError::DuplicateAssignment);
                }

                owner_by_unit.insert(unit_id.clone(), partition.id);

                let work = workload
                    .units
                    .iter()
                    .find(|unit| &unit.id == unit_id)
                    .ok_or(PartitionError::UnknownWorkUnit)?
                    .estimated_work;

                partition_work = partition_work
                    .checked_add(work)
                    .ok_or(PartitionError::ArithmeticOverflow)?;
            }

            if partition_work != partition.estimated_work {
                return Err(PartitionError::InconsistentWorkEstimate);
            }
        }

        if actual_units != expected_units {
            return Err(PartitionError::MissingWorkUnit);
        }

        let mut expected_boundaries = Vec::new();

        for edge in &workload.dependencies {
            let source_partition = *owner_by_unit
                .get(&edge.source)
                .ok_or(PartitionError::UnknownWorkUnit)?;

            let destination_partition = *owner_by_unit
                .get(&edge.destination)
                .ok_or(PartitionError::UnknownWorkUnit)?;

            if source_partition != destination_partition {
                expected_boundaries.push((
                    edge.source.clone(),
                    edge.destination.clone(),
                    source_partition,
                    destination_partition,
                    edge.communication_cost,
                ));
            }
        }

        expected_boundaries.sort();

        let mut actual_boundaries = self
            .boundaries
            .iter()
            .map(|edge| {
                (
                    edge.source.clone(),
                    edge.destination.clone(),
                    edge.source_partition,
                    edge.destination_partition,
                    edge.communication_cost,
                )
            })
            .collect::<Vec<_>>();

        actual_boundaries.sort();

        if actual_boundaries != expected_boundaries {
            return Err(PartitionError::InconsistentBoundary);
        }

        if self.dependency_count != workload.dependencies.len() {
            return Err(PartitionError::MissingDependency);
        }

        Ok(())
    }
}

/// Failures that prevent construction of a valid partition plan.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum PartitionError {
    /// Two or more work units have the same logical identifier.
    DuplicateWorkUnit,

    /// A dependency refers to a work unit that does not exist.
    UnknownWorkUnit,

    /// A work unit was assigned more than once.
    DuplicateAssignment,

    /// One or more input work units were omitted.
    MissingWorkUnit,

    /// One or more input dependencies were not preserved.
    MissingDependency,

    /// The generated plan's boundary metadata is inconsistent.
    InconsistentBoundary,

    /// A partition's recorded work estimate is inconsistent.
    InconsistentWorkEstimate,

    /// A configured operational budget was exceeded.
    BudgetExceeded {
        /// Name of the budget dimension.
        dimension: &'static str,
        /// Actual input value.
        actual: usize,
        /// Configured maximum.
        maximum: usize,
    },

    /// A non-empty workload requested zero partitions.
    ZeroPartitionsRequested,

    /// A configured per-partition unit limit cannot accommodate the workload.
    UnitCapacityUnsatisfied,

    /// A work unit exceeds the configured per-partition work budget.
    WorkCapacityUnsatisfied {
        /// The offending work-unit identifier, rendered for diagnostics.
        work_unit: String,
        /// Its estimated work.
        required: WorkEstimate,
        /// The configured maximum.
        maximum: WorkEstimate,
    },

    /// Checked arithmetic detected an unrepresentable total.
    ArithmeticOverflow,

    /// A required partition identifier could not be represented.
    PartitionIdentifierOverflow,
}

impl fmt::Display for PartitionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateWorkUnit => {
                f.write_str("workload contains duplicate work-unit identifiers")
            }
            Self::UnknownWorkUnit => {
                f.write_str("dependency or partition refers to an unknown work unit")
            }
            Self::DuplicateAssignment => {
                f.write_str("a work unit is assigned to more than one partition")
            }
            Self::MissingWorkUnit => {
                f.write_str("partition plan omits one or more input work units")
            }
            Self::MissingDependency => {
                f.write_str("partition plan does not preserve every input dependency")
            }
            Self::InconsistentBoundary => {
                f.write_str("partition boundary metadata is inconsistent")
            }
            Self::InconsistentWorkEstimate => {
                f.write_str("partition work estimate does not match its members")
            }
            Self::BudgetExceeded {
                dimension,
                actual,
                maximum,
            } => write!(
                f,
                "partitioning budget exceeded for {dimension}: {actual} exceeds {maximum}"
            ),
            Self::ZeroPartitionsRequested => {
                f.write_str("a non-empty workload cannot request zero partitions")
            }
            Self::UnitCapacityUnsatisfied => {
                f.write_str("configured per-partition unit capacity cannot accommodate the workload")
            }
            Self::WorkCapacityUnsatisfied {
                work_unit,
                required,
                maximum,
            } => write!(
                f,
                "work unit {work_unit:?} requires estimated work {required}, exceeding partition capacity {maximum}"
            ),
            Self::ArithmeticOverflow => {
                f.write_str("partitioning arithmetic overflowed the supported representation")
            }
            Self::PartitionIdentifierOverflow => {
                f.write_str("partition identifier space was exhausted")
            }
        }
    }
}

impl std::error::Error for PartitionError {}

fn check_budget(
    dimension: &'static str,
    actual: usize,
    maximum: Option<usize>,
) -> Result<(), PartitionError> {
    if let Some(maximum) = maximum {
        if actual > maximum {
            return Err(PartitionError::BudgetExceeded {
                dimension,
                actual,
                maximum,
            });
        }
    }

    Ok(())
}

/// Partitions a workload graph deterministically.
///
/// # Algorithm
///
/// 1. Validate uniqueness of work-unit identifiers.
/// 2. Validate all dependency endpoints.
/// 3. Check explicit operational budgets.
/// 4. Canonicalize units and dependencies.
/// 5. Greedily assign each work unit to a permitted partition.
/// 6. Prefer graph locality and affinity, or balance, according to policy.
/// 7. Preserve all dependencies and identify cross-partition edges.
/// 8. Validate the complete output before returning it.
///
/// A dependency cycle is preserved rather than silently broken. The
/// integration layer is responsible for deciding whether cycles are
/// permitted by the source execution model.
///
/// # Errors
///
/// Returns an error for invalid graphs, unsatisfied explicit partition
/// capacities, budget exhaustion, or arithmetic overflow. No partial plan
/// is returned.
///
/// # Determinism
///
/// For the same graph, identifiers, estimates, affinity keys, and policy,
/// this function returns the same logical partition assignment regardless
/// of input vector ordering.
///
/// # Complexity
///
/// The greedy assignment currently scans candidate partitions for each
/// work unit. Its time complexity can therefore grow quadratically with
/// the number of generated partitions. For very large graphs, the planning
/// coordinator should use an alternative strategy or implementation through
/// the same contract rather than impose a universal graph-size ceiling.
pub fn partition_workload<Id>(
    workload: &WorkloadGraph<Id>,
    policy: PartitionPolicy,
    budget: PartitionBudget,
) -> Result<PartitionPlan<Id>, PartitionError>
where
    Id: Clone + Ord + fmt::Debug,
{
    budget.validate(workload)?;

    if workload.units.is_empty() {
        if !workload.dependencies.is_empty() {
            return Err(PartitionError::UnknownWorkUnit);
        }

        return Ok(PartitionPlan {
            partitions: Vec::new(),
            boundaries: Vec::new(),
            dependency_count: 0,
            total_estimated_work: 0,
        });
    }

    if policy.target_partition_count == Some(0) {
        return Err(PartitionError::ZeroPartitionsRequested);
    }

    if policy.max_units_per_partition == Some(0) {
        return Err(PartitionError::UnitCapacityUnsatisfied);
    }

    if policy.max_work_per_partition == Some(0)
        && workload.units.iter().any(|unit| unit.estimated_work > 0)
    {
        let unit = workload
            .units
            .iter()
            .find(|unit| unit.estimated_work > 0)
            .ok_or(PartitionError::ArithmeticOverflow)?;

        return Err(PartitionError::WorkCapacityUnsatisfied {
            work_unit: format!("{:?}", unit.id),
            required: unit.estimated_work,
            maximum: 0,
        });
    }

    let mut units_by_id = BTreeMap::new();

    for unit in &workload.units {
        if units_by_id.insert(unit.id.clone(), unit).is_some() {
            return Err(PartitionError::DuplicateWorkUnit);
        }
    }

    let mut sorted_dependencies = workload.dependencies.iter().collect::<Vec<_>>();
    sorted_dependencies.sort_by(|left, right| {
        (&left.source, &left.destination, left.communication_cost).cmp(&(
            &right.source,
            &right.destination,
            right.communication_cost,
        ))
    });

    let mut adjacency: BTreeMap<Id, Vec<(Id, WorkEstimate)>> = BTreeMap::new();

    for edge in &sorted_dependencies {
        if !units_by_id.contains_key(&edge.source)
            || !units_by_id.contains_key(&edge.destination)
        {
            return Err(PartitionError::UnknownWorkUnit);
        }

        adjacency
            .entry(edge.source.clone())
            .or_default()
            .push((edge.destination.clone(), edge.communication_cost));

        adjacency
            .entry(edge.destination.clone())
            .or_default()
            .push((edge.source.clone(), edge.communication_cost));
    }

    let mut total_estimated_work = 0_u128;

    for unit in units_by_id.values() {
        total_estimated_work = total_estimated_work
            .checked_add(unit.estimated_work)
            .ok_or(PartitionError::ArithmeticOverflow)?;

        if let Some(maximum) = policy.max_work_per_partition {
            if unit.estimated_work > maximum {
                return Err(PartitionError::WorkCapacityUnsatisfied {
                    work_unit: format!("{:?}", unit.id),
                    required: unit.estimated_work,
                    maximum,
                });
            }
        }
    }

    let mut partitions: Vec<WorkPartition<Id>> = Vec::new();
    let mut owner_by_unit: BTreeMap<Id, PartitionId> = BTreeMap::new();

    for unit in units_by_id.values() {
        let mut candidates = Vec::new();

        for partition in &partitions {
            if !partition_can_accept(partition, unit, policy) {
                continue;
            }

            let locality = adjacency
                .get(&unit.id)
                .into_iter()
                .flatten()
                .filter(|(neighbor, _)| {
                    owner_by_unit.get(neighbor) == Some(&partition.id)
                })
                .try_fold(0_u128, |sum, (_, cost)| sum.checked_add(*cost))
                .ok_or(PartitionError::ArithmeticOverflow)?;

            let affinity_match = unit.affinity.as_ref().is_some_and(|affinity| {
                partition.units.iter().any(|member| {
                    units_by_id
                        .get(member)
                        .is_some_and(|other| other.affinity.as_ref() == Some(affinity))
                })
            });

            let projected_work = partition
                .estimated_work
                .checked_add(unit.estimated_work)
                .ok_or(PartitionError::ArithmeticOverflow)?;

            candidates.push((
                partition.id,
                locality,
                affinity_match,
                projected_work,
                partition.units.len(),
            ));
        }

        candidates.sort_by(|left, right| {
            let (left_id, left_locality, left_affinity, left_work, left_count) = *left;
            let (right_id, right_locality, right_affinity, right_work, right_count) = *right;

            match policy.strategy {
                PartitionStrategy::LocalityFirst => right_locality
                    .cmp(&left_locality)
                    .then_with(|| right_affinity.cmp(&left_affinity))
                    .then_with(|| left_work.cmp(&right_work))
                    .then_with(|| left_count.cmp(&right_count))
                    .then_with(|| left_id.cmp(&right_id)),
                PartitionStrategy::BalanceFirst => left_work
                    .cmp(&right_work)
                    .then_with(|| right_locality.cmp(&left_locality))
                    .then_with(|| right_affinity.cmp(&left_affinity))
                    .then_with(|| left_count.cmp(&right_count))
                    .then_with(|| left_id.cmp(&right_id)),
            }
        });

        let selected = candidates.first().map(|candidate| candidate.0);

        let partition_id = match selected {
            Some(id) => id,
            None => {
                let id = u64::try_from(partitions.len())
                    .map(PartitionId)
                    .map_err(|_| PartitionError::PartitionIdentifierOverflow)?;

                if let Some(maximum) = budget.max_partitions {
                    let next_count = partitions
                        .len()
                        .checked_add(1)
                        .ok_or(PartitionError::ArithmeticOverflow)?;

                    if next_count > maximum {
                        return Err(PartitionError::BudgetExceeded {
                            dimension: "output partitions",
                            actual: next_count,
                            maximum,
                        });
                    }
                }

                if let Some(target) = policy.target_partition_count {
                    if partitions.len() >= target {
                        if policy.max_work_per_partition.is_some()
                            || policy.max_units_per_partition.is_some()
                        {
                            return Err(PartitionError::UnitCapacityUnsatisfied);
                        }

                        // A requested count is a target, not a semantic
                        // guarantee that every target slot must be used.
                        // If all existing partitions are eligible, this
                        // branch is not reached.
                    }
                }

                partitions.push(WorkPartition {
                    id,
                    units: Vec::new(),
                    estimated_work: 0,
                });

                id
            }
        };

        let partition = partitions
            .iter_mut()
            .find(|partition| partition.id == partition_id)
            .ok_or(PartitionError::PartitionIdentifierOverflow)?;

        partition.units.push(unit.id.clone());
        partition.estimated_work = partition
            .estimated_work
            .checked_add(unit.estimated_work)
            .ok_or(PartitionError::ArithmeticOverflow)?;

        owner_by_unit.insert(unit.id.clone(), partition_id);
    }

    let mut boundaries = Vec::new();

    for edge in &sorted_dependencies {
        let source_partition = *owner_by_unit
            .get(&edge.source)
            .ok_or(PartitionError::MissingWorkUnit)?;

        let destination_partition = *owner_by_unit
            .get(&edge.destination)
            .ok_or(PartitionError::MissingWorkUnit)?;

        if source_partition != destination_partition {
            boundaries.push(BoundaryEdge {
                source: edge.source.clone(),
                destination: edge.destination.clone(),
                source_partition,
                destination_partition,
                communication_cost: edge.communication_cost,
            });
        }
    }

    let plan = PartitionPlan {
        partitions,
        boundaries,
        dependency_count: workload.dependencies.len(),
        total_estimated_work,
    };

    plan.validate_against(workload)?;

    Ok(plan)
}

fn partition_can_accept<Id>(
    partition: &WorkPartition<Id>,
    unit: &WorkUnit<Id>,
    policy: PartitionPolicy,
) -> bool {
    if let Some(maximum) = policy.max_units_per_partition {
        if partition.units.len() >= maximum {
            return false;
        }
    }

    if let Some(maximum) = policy.max_work_per_partition {
        if partition
            .estimated_work
            .checked_add(unit.estimated_work)
            .is_none_or(|work| work > maximum)
        {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    fn unit(id: u64, work: u128) -> WorkUnit<u64> {
        WorkUnit::new(id, work, None)
    }

    #[test]
    fn empty_workload_produces_empty_plan() {
        let graph = WorkloadGraph::<u64>::new(Vec::new(), Vec::new());

        let plan = partition_workload(
            &graph,
            PartitionPolicy::new(),
            PartitionBudget::unlimited(),
        )
        .expect("empty workload should be valid");

        assert!(plan.is_empty());
        assert_eq!(plan.dependency_count(), 0);
        assert_eq!(plan.total_estimated_work(), 0);
    }

    #[test]
    fn every_work_unit_is_preserved() {
        let graph = WorkloadGraph::new(
            vec![unit(3, 7), unit(1, 2), unit(2, 5)],
            Vec::new(),
        );

        let plan = partition_workload(
            &graph,
            PartitionPolicy::new().with_target_partition_count(Some(2)),
            PartitionBudget::unlimited(),
        )
        .expect("graph should partition");

        plan.validate_against(&graph)
            .expect("plan should preserve all work units");

        assert_eq!(
            plan.partitions()
                .iter()
                .flat_map(|partition| partition.units().iter().copied())
                .collect::<BTreeSet<_>>(),
            BTreeSet::from([1, 2, 3])
        );
    }

    #[test]
    fn cross_partition_dependencies_are_reported() {
        let graph = WorkloadGraph::new(
            vec![unit(1, 4), unit(2, 4)],
            vec![Dependency::new(1, 2, 9)],
        );

        let plan = partition_workload(
            &graph,
            PartitionPolicy::new()
                .with_target_partition_count(Some(2))
                .with_max_units_per_partition(Some(1)),
            PartitionBudget::unlimited(),
        )
        .expect("graph should partition");

        assert_eq!(plan.dependency_count(), 1);
        assert_eq!(plan.boundaries().len(), 1);
        assert_eq!(plan.boundaries()[0].source(), &1);
        assert_eq!(plan.boundaries()[0].destination(), &2);
        assert_ne!(
            plan.boundaries()[0].source_partition(),
            plan.boundaries()[0].destination_partition()
        );
    }

    #[test]
    fn dependencies_with_unknown_endpoints_are_rejected() {
        let graph = WorkloadGraph::new(
            vec![unit(1, 1)],
            vec![Dependency::new(1, 99, 1)],
        );

        let result = partition_workload(
            &graph,
            PartitionPolicy::new(),
            PartitionBudget::unlimited(),
        );

        assert_eq!(result, Err(PartitionError::UnknownWorkUnit));
    }

    #[test]
    fn duplicate_work_unit_identifiers_are_rejected() {
        let graph = WorkloadGraph::new(
            vec![unit(1, 1), unit(1, 2)],
            Vec::new(),
        );

        let result = partition_workload(
            &graph,
            PartitionPolicy::new(),
            PartitionBudget::unlimited(),
        );

        assert_eq!(result, Err(PartitionError::DuplicateWorkUnit));
    }

    #[test]
    fn budget_exhaustion_is_not_reported_as_success() {
        let graph = WorkloadGraph::new(
            vec![unit(1, 1), unit(2, 1)],
            Vec::new(),
        );

        let result = partition_workload(
            &graph,
            PartitionPolicy::new(),
            PartitionBudget::unlimited().with_max_work_units(Some(1)),
        );

        assert!(matches!(
            result,
            Err(PartitionError::BudgetExceeded {
                dimension: "work units",
                actual: 2,
                maximum: 1,
            })
        ));
    }

    #[test]
    fn indivisible_work_unit_over_capacity_is_rejected() {
        let graph = WorkloadGraph::new(vec![unit(1, 11)], Vec::new());

        let result = partition_workload(
            &graph,
            PartitionPolicy::new().with_max_work_per_partition(Some(10)),
            PartitionBudget::unlimited(),
        );

        assert!(matches!(
            result,
            Err(PartitionError::WorkCapacityUnsatisfied {
                required: 11,
                maximum: 10,
                ..
            })
        ));
    }

    #[test]
    fn arithmetic_overflow_is_reported() {
        let graph = WorkloadGraph::new(
            vec![unit(1, u128::MAX), unit(2, 1)],
            Vec::new(),
        );

        let result = partition_workload(
            &graph,
            PartitionPolicy::new(),
            PartitionBudget::unlimited(),
        );

        assert_eq!(result, Err(PartitionError::ArithmeticOverflow));
    }

    #[test]
    fn result_is_independent_of_input_order() {
        let first = WorkloadGraph::new(
            vec![unit(1, 3), unit(2, 5), unit(3, 7)],
            vec![
                Dependency::new(1, 2, 4),
                Dependency::new(2, 3, 6),
            ],
        );

        let second = WorkloadGraph::new(
            vec![unit(3, 7), unit(1, 3), unit(2, 5)],
            vec![
                Dependency::new(2, 3, 6),
                Dependency::new(1, 2, 4),
            ],
        );

        let policy = PartitionPolicy::new()
            .with_target_partition_count(Some(2))
            .with_strategy(PartitionStrategy::LocalityFirst);

        let left = partition_workload(
            &first,
            policy,
            PartitionBudget::unlimited(),
        )
        .expect("first graph should partition");

        let right = partition_workload(
            &second,
            policy,
            PartitionBudget::unlimited(),
        )
        .expect("second graph should partition");

        assert_eq!(left, right);
    }

    #[test]
    fn explicit_partition_capacity_is_enforced() {
        let graph = WorkloadGraph::new(
            vec![unit(1, 1), unit(2, 1), unit(3, 1)],
            Vec::new(),
        );

        let result = partition_workload(
            &graph,
            PartitionPolicy::new()
                .with_max_units_per_partition(Some(1)),
            PartitionBudget::unlimited().with_max_partitions(Some(2)),
        );

        assert!(matches!(
            result,
            Err(PartitionError::BudgetExceeded {
                dimension: "output partitions",
                actual: 3,
                maximum: 2,
            })
        ));
    }

    #[test]
    fn no_unsafe_code_or_fixed_workload_ceiling_is_required() {
        let graph = WorkloadGraph::new(
            vec![unit(0, 0), unit(u64::MAX, 0)],
            Vec::new(),
        );

        let plan = partition_workload(
            &graph,
            PartitionPolicy::new(),
            PartitionBudget::unlimited(),
        )
        .expect("large representable identifiers should work");

        assert_eq!(plan.partitions().len(), 1);
        assert_eq!(plan.total_estimated_work(), 0);
    }
}
