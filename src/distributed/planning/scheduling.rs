
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Distributed workload scheduling
//!
//! Path: `src/distributed/planning/scheduling.rs`
//! Rust edition: 2021; minimum Rust version: 1.97.
//!
//! Produces a deterministic, dependency-respecting execution schedule for a
//! workload described by the canonical distributed planning graph.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Dependency-aware ordering of logical work units.
//! - Deterministic list-scheduling strategies.
//! - Scheduling against caller-supplied resource capacities.
//! - Concurrent execution eligibility within one planning snapshot.
//! - Checked time and resource arithmetic.
//! - Deadline and explicit scheduling-budget validation.
//! - Validation of schedule invariants.
//!
//! This module does not own:
//!
//! - Source syntax, semantic analysis, or domain IRs.
//! - Resource discovery or capability discovery.
//! - Physical resource allocation or placement.
//! - Network routing or transport.
//! - Runtime dispatch, cancellation of running work, or recovery.
//! - Device-specific scheduling, quantum pulse timing, or HDL timing.
//!
//! ## Integration
//!
//! The scheduler consumes `WorkloadGraph` from `planning::partition`.
//! Callers supply a separate `ScheduleProfile` for each work unit and a
//! snapshot of the capacities of the resources represented by that profile.
//!
//! `WorkUnit::estimated_work` is not interpreted as execution duration.
//! `Dependency::communication_cost` is not interpreted as network latency.
//! Those values are abstract estimates owned by the partitioning contract.
//!
//! The duration and resource-demand units used here are supplied by the
//! caller and must be consistent within one scheduling request.
//!
//! Placement must establish which resource identities are valid for each
//! work unit before scheduling. A later routing stage may introduce
//! communication-delay constraints that this scheduler does not infer.
//!
//! ## Scalability
//!
//! There are no universal fixed limits on work-unit counts, resource counts,
//! or schedule length. Optional budgets are caller-selected operational
//! limits. Rust address-space, integer-representation, memory, and execution
//! time remain finite.
//!
//! The algorithm is a deterministic list scheduler, not a global optimizer.
//! It preserves dependency ordering and resource capacities, but does not
//! promise a globally optimal makespan.

use core::fmt;
use std::collections::{BTreeMap, BTreeSet};

use super::partition::WorkloadGraph;

/// Abstract, non-negative duration in caller-defined time units.
///
/// This is deliberately not `std::time::Duration`: a distributed schedule
/// may use logical ticks, domain-specific time units, or another exact unit.
/// All durations in one request must use the same unit.
pub type ScheduleTime = u128;

/// Abstract quantity of a schedulable resource.
///
/// Its interpretation belongs to the resource provider. Examples might
/// include execution slots, memory units, or another explicitly defined
/// capacity measure. This type does not imply that different resource kinds
/// have interchangeable units.
pub type ResourceAmount = u128;

/// A resource capacity snapshot used for one scheduling operation.
///
/// The snapshot is not a reservation. The runtime or resource owner must
/// revalidate availability and obtain any required allocation before
/// execution.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResourceCapacity<ResourceId> {
    id: ResourceId,
    capacity: ResourceAmount,
}

impl<ResourceId> ResourceCapacity<ResourceId> {
    /// Creates a capacity entry.
    #[must_use]
    pub const fn new(id: ResourceId, capacity: ResourceAmount) -> Self {
        Self { id, capacity }
    }

    /// Returns the resource identifier.
    #[must_use]
    pub const fn id(&self) -> &ResourceId {
        &self.id
    }

    /// Returns the available capacity reported for this snapshot.
    #[must_use]
    pub const fn capacity(&self) -> ResourceAmount {
        self.capacity
    }
}

/// Scheduling metadata for one indivisible work unit.
///
/// Resource demands are concurrent reservations for the lifetime of the
/// scheduled work unit. The profile does not select a physical device and
/// must not be treated as a replacement for canonical semantic requirements
/// or placement validation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ScheduleProfile<WorkId, ResourceId> {
    work_id: WorkId,
    duration: ScheduleTime,
    resource_demands: BTreeMap<ResourceId, ResourceAmount>,
}

impl<WorkId, ResourceId> ScheduleProfile<WorkId, ResourceId> {
    /// Creates a profile with no resource demands.
    ///
    /// Use [`ScheduleProfile::with_resource_demands`] when the work unit
    /// requires explicitly identified resources.
    #[must_use]
    pub fn new(work_id: WorkId, duration: ScheduleTime) -> Self {
        Self {
            work_id,
            duration,
            resource_demands: BTreeMap::new(),
        }
    }

    /// Replaces the resource-demand map.
    ///
    /// Repeated resource identifiers in the input are rejected rather than
    /// silently overwriting a previous demand.
    ///
    /// # Errors
    ///
    /// Returns [`SchedulingError::DuplicateResourceDemand`] if an identifier
    /// appears more than once.
    pub fn with_resource_demands(
        mut self,
        demands: impl IntoIterator<Item = (ResourceId, ResourceAmount)>,
    ) -> Result<Self, SchedulingError>
    where
        ResourceId: Ord,
    {
        let mut resource_demands = BTreeMap::new();

        for (resource_id, amount) in demands {
            if resource_demands.insert(resource_id, amount).is_some() {
                return Err(SchedulingError::DuplicateResourceDemand);
            }
        }

        self.resource_demands = resource_demands;
        Ok(self)
    }

    /// Returns the logical work-unit identifier.
    #[must_use]
    pub const fn work_id(&self) -> &WorkId {
        &self.work_id
    }

    /// Returns the estimated duration in caller-defined time units.
    #[must_use]
    pub const fn duration(&self) -> ScheduleTime {
        self.duration
    }

    /// Returns the required capacity of each resource.
    #[must_use]
    pub const fn resource_demands(
        &self,
    ) -> &BTreeMap<ResourceId, ResourceAmount> {
        &self.resource_demands
    }
}

/// Strategy for ordering work units that are ready to run.
///
/// All strategies respect dependencies and resource capacities. The strategy
/// only chooses among work units that are currently eligible to start.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[non_exhaustive]
pub enum SchedulingStrategy {
    /// Prioritize the longest remaining dependency path.
    ///
    /// This is often useful for reducing the risk of leaving a long critical
    /// path until late in the schedule.
    #[default]
    CriticalPathFirst,

    /// Prioritize longer individual work units.
    ///
    /// This can be useful when long work units would otherwise be delayed
    /// behind many short ones.
    LongestProcessingTimeFirst,

    /// Prioritize by canonical work-unit identifier.
    ///
    /// This strategy is deterministic and does not depend on estimates.
    StableIdentifierOrder,
}

/// Configurable operational policy for one scheduling request.
///
/// `max_concurrent_units`, `max_work_units`, and `deadline` are explicit
/// caller policies, not universal language limits. An omitted value imposes
/// no additional restriction for that dimension.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SchedulingPolicy {
    strategy: SchedulingStrategy,
    max_concurrent_units: Option<usize>,
    max_work_units: Option<usize>,
    deadline: Option<ScheduleTime>,
}

impl SchedulingPolicy {
    /// Creates the default scheduling policy.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            strategy: SchedulingStrategy::CriticalPathFirst,
            max_concurrent_units: None,
            max_work_units: None,
            deadline: None,
        }
    }

    /// Selects the ordering strategy.
    #[must_use]
    pub const fn with_strategy(
        mut self,
        strategy: SchedulingStrategy,
    ) -> Self {
        self.strategy = strategy;
        self
    }

    /// Sets the maximum number of simultaneously running work units.
    ///
    /// A value of zero is invalid for a non-empty workload.
    #[must_use]
    pub const fn with_max_concurrent_units(
        mut self,
        maximum: Option<usize>,
    ) -> Self {
        self.max_concurrent_units = maximum;
        self
    }

    /// Sets an explicit operational budget for the number of work units.
    #[must_use]
    pub const fn with_max_work_units(
        mut self,
        maximum: Option<usize>,
    ) -> Self {
        self.max_work_units = maximum;
        self
    }

    /// Sets an optional maximum makespan in caller-defined time units.
    ///
    /// The scheduler reports [`SchedulingError::DeadlineExceeded`] when the
    /// produced schedule finishes after this deadline. It does not silently
    /// shorten durations or discard work to satisfy the deadline.
    #[must_use]
    pub const fn with_deadline(
        mut self,
        deadline: Option<ScheduleTime>,
    ) -> Self {
        self.deadline = deadline;
        self
    }
}

impl Default for SchedulingPolicy {
    fn default() -> Self {
        Self::new()
    }
}

/// Scheduled placement in logical time for one work unit.
///
/// `start` and `finish` use the time unit chosen by the caller. This record
/// does not mean the work unit has actually started or completed.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ScheduledWork<WorkId, ResourceId> {
    work_id: WorkId,
    start: ScheduleTime,
    finish: ScheduleTime,
    resource_demands: BTreeMap<ResourceId, ResourceAmount>,
}

impl<WorkId, ResourceId> ScheduledWork<WorkId, ResourceId> {
    /// Returns the work-unit identifier.
    #[must_use]
    pub const fn work_id(&self) -> &WorkId {
        &self.work_id
    }

    /// Returns the planned start time.
    #[must_use]
    pub const fn start(&self) -> ScheduleTime {
        self.start
    }

    /// Returns the planned finish time.
    #[must_use]
    pub const fn finish(&self) -> ScheduleTime {
        self.finish
    }

    /// Returns the resources and capacities reserved by this scheduled unit.
    #[must_use]
    pub const fn resource_demands(
        &self,
    ) -> &BTreeMap<ResourceId, ResourceAmount> {
        &self.resource_demands
    }
}

/// Validated output of one scheduling operation.
///
/// Work is returned in deterministic start-time and identifier order.
/// `makespan` is the maximum planned finish time, or zero for an empty
/// workload.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SchedulePlan<WorkId, ResourceId> {
    scheduled: Vec<ScheduledWork<WorkId, ResourceId>>,
    makespan: ScheduleTime,
    critical_path_length: ScheduleTime,
}

impl<WorkId, ResourceId> SchedulePlan<WorkId, ResourceId> {
    /// Returns all scheduled work units.
    #[must_use]
    pub fn scheduled(&self) -> &[ScheduledWork<WorkId, ResourceId>] {
        &self.scheduled
    }

    /// Returns the overall planned finish time.
    #[must_use]
    pub const fn makespan(&self) -> ScheduleTime {
        self.makespan
    }

    /// Returns the dependency-only critical path length.
    ///
    /// This excludes network latency, routing delays, resource allocation
    /// latency, and runtime startup costs unless the caller included those
    /// costs in the supplied work-unit durations.
    #[must_use]
    pub const fn critical_path_length(&self) -> ScheduleTime {
        self.critical_path_length
    }

    /// Returns whether the plan contains no work units.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.scheduled.is_empty()
    }

    /// Finds a scheduled unit by its logical identifier.
    #[must_use]
    pub fn work(&self, id: &WorkId) -> Option<&ScheduledWork<WorkId, ResourceId>>
    where
        WorkId: PartialEq,
    {
        self.scheduled.iter().find(|work| &work.work_id == id)
    }
}

/// Error that prevents the scheduler from producing a valid plan.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum SchedulingError {
    /// The input graph contains repeated logical work-unit identifiers.
    DuplicateWorkUnit,

    /// The input graph contains a dependency whose endpoint is unknown.
    UnknownDependencyEndpoint,

    /// A work unit has no scheduling profile.
    MissingScheduleProfile,

    /// A scheduling profile refers to a work unit not in the graph.
    UnknownScheduleProfile,

    /// Multiple scheduling profiles refer to the same work unit.
    DuplicateScheduleProfile,

    /// A resource identifier appears more than once in a capacity snapshot.
    DuplicateResourceCapacity,

    /// A resource identifier appears more than once in one work profile.
    DuplicateResourceDemand,

    /// A demanded resource has no capacity entry in the supplied snapshot.
    MissingResourceCapacity {
        /// Work-unit identifier, rendered for diagnostics.
        work_unit: String,
        /// Resource identifier, rendered for diagnostics.
        resource: String,
    },

    /// A work unit requires more of a resource than the snapshot provides.
    ResourceCapacityUnsatisfied {
        /// Work-unit identifier, rendered for diagnostics.
        work_unit: String,
        /// Resource identifier, rendered for diagnostics.
        resource: String,
        /// Required capacity.
        required: ResourceAmount,
        /// Available capacity.
        available: ResourceAmount,
    },

    /// The dependency graph contains a cycle.
    DependencyCycle,

    /// A non-empty workload requested zero concurrent work units.
    ZeroConcurrencyRequested,

    /// The input workload exceeded an explicit operational budget.
    BudgetExceeded {
        /// Name of the budget dimension.
        dimension: &'static str,
        /// Actual count.
        actual: usize,
        /// Configured maximum.
        maximum: usize,
    },

    /// A checked integer operation could not be represented.
    ArithmeticOverflow,

    /// The schedule finishes after the configured deadline.
    DeadlineExceeded {
        /// Actual planned makespan.
        makespan: ScheduleTime,
        /// Configured deadline.
        deadline: ScheduleTime,
    },

    /// No remaining work could start although no work was running.
    ///
    /// This is a defensive consistency error; valid acyclic graphs with
    /// individually satisfiable resource demands should not reach it.
    NoProgress,
}

impl fmt::Display for SchedulingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateWorkUnit => {
                f.write_str("workload contains duplicate work-unit identifiers")
            }
            Self::UnknownDependencyEndpoint => {
                f.write_str("dependency refers to an unknown work unit")
            }
            Self::MissingScheduleProfile => {
                f.write_str("a work unit has no scheduling profile")
            }
            Self::UnknownScheduleProfile => {
                f.write_str("a scheduling profile refers to an unknown work unit")
            }
            Self::DuplicateScheduleProfile => {
                f.write_str("multiple scheduling profiles refer to one work unit")
            }
            Self::DuplicateResourceCapacity => {
                f.write_str("resource capacity snapshot contains duplicate identifiers")
            }
            Self::DuplicateResourceDemand => {
                f.write_str("work profile contains duplicate resource demands")
            }
            Self::MissingResourceCapacity { work_unit, resource } => {
                write!(
                    f,
                    "work unit {work_unit} requires resource {resource}, \
                     but no capacity was supplied"
                )
            }
            Self::ResourceCapacityUnsatisfied {
                work_unit,
                resource,
                required,
                available,
            } => {
                write!(
                    f,
                    "work unit {work_unit} requires {required} of resource \
                     {resource}, but only {available} is available"
                )
            }
            Self::DependencyCycle => {
                f.write_str("workload dependency graph contains a cycle")
            }
            Self::ZeroConcurrencyRequested => {
                f.write_str("non-empty workload cannot use a zero concurrency budget")
            }
            Self::BudgetExceeded {
                dimension,
                actual,
                maximum,
            } => {
                write!(
                    f,
                    "scheduling budget for {dimension} exceeded: \
                     {actual} exceeds {maximum}"
                )
            }
            Self::ArithmeticOverflow => {
                f.write_str("scheduling arithmetic overflowed its numeric representation")
            }
            Self::DeadlineExceeded { makespan, deadline } => {
                write!(
                    f,
                    "planned makespan {makespan} exceeds deadline {deadline}"
                )
            }
            Self::NoProgress => {
                f.write_str("scheduler could not make progress on a valid workload")
            }
        }
    }
}

impl std::error::Error for SchedulingError {}

/// Produces a deterministic, dependency-respecting schedule.
///
/// `graph` is the workload graph already used by the partitioning stage.
/// `profiles` provide the duration and resource demand for every work unit.
/// `capacities` describe one resource snapshot and must be consistent with
/// the resource identities used in the profiles.
///
/// Each work unit is scheduled as an indivisible item. This function does
/// not subdivide work, choose physical targets, route communication, or
/// execute the resulting plan.
///
/// Duplicate dependency edges are treated as redundant ordering constraints
/// for scheduling purposes. The original graph is not modified.
///
/// # Errors
///
/// Returns an error if identifiers or profiles are invalid, a dependency
/// endpoint is unknown, the graph contains a cycle, a resource demand cannot
/// be satisfied, a configured budget is exceeded, or checked arithmetic
/// fails.
pub fn schedule_workload<WorkId, ResourceId>(
    graph: &WorkloadGraph<WorkId>,
    profiles: &[ScheduleProfile<WorkId, ResourceId>],
    capacities: &[ResourceCapacity<ResourceId>],
    policy: SchedulingPolicy,
) -> Result<SchedulePlan<WorkId, ResourceId>, SchedulingError>
where
    WorkId: Clone + Ord + fmt::Debug,
    ResourceId: Clone + Ord + fmt::Debug,
{
    let units = graph.units();
    let dependencies = graph.dependencies();

    if let Some(maximum) = policy.max_work_units {
        if units.len() > maximum {
            return Err(SchedulingError::BudgetExceeded {
                dimension: "work units",
                actual: units.len(),
                maximum,
            });
        }
    }

    if !units.is_empty() && policy.max_concurrent_units == Some(0) {
        return Err(SchedulingError::ZeroConcurrencyRequested);
    }

    let mut unit_ids = BTreeSet::new();
    for unit in units {
        if !unit_ids.insert(unit.id().clone()) {
            return Err(SchedulingError::DuplicateWorkUnit);
        }
    }

    let mut profile_by_id = BTreeMap::new();
    for profile in profiles {
        if !unit_ids.contains(profile.work_id()) {
            return Err(SchedulingError::UnknownScheduleProfile);
        }

        if profile_by_id
            .insert(profile.work_id().clone(), profile)
            .is_some()
        {
            return Err(SchedulingError::DuplicateScheduleProfile);
        }
    }

    if profile_by_id.len() != unit_ids.len() {
        return Err(SchedulingError::MissingScheduleProfile);
    }

    let mut capacity_by_id = BTreeMap::new();
    for capacity in capacities {
        if capacity_by_id
            .insert(capacity.id().clone(), capacity.capacity())
            .is_some()
        {
            return Err(SchedulingError::DuplicateResourceCapacity);
        }
    }

    // Validate each individual task against the supplied snapshot before
    // scheduling. A task that can never fit must not be left waiting forever.
    for (work_id, profile) in &profile_by_id {
        for (resource_id, required) in profile.resource_demands() {
            let available = capacity_by_id.get(resource_id).copied().ok_or_else(|| {
                SchedulingError::MissingResourceCapacity {
                    work_unit: format!("{work_id:?}"),
                    resource: format!("{resource_id:?}"),
                }
            })?;

            if *required > available {
                return Err(SchedulingError::ResourceCapacityUnsatisfied {
                    work_unit: format!("{work_id:?}"),
                    resource: format!("{resource_id:?}"),
                    required: *required,
                    available,
                });
            }
        }
    }

    if units.is_empty() {
        return Ok(SchedulePlan {
            scheduled: Vec::new(),
            makespan: 0,
            critical_path_length: 0,
        });
    }

    // Normalize dependencies to unique predecessor/successor relationships.
    // The original graph can retain parallel edges; repeated edges do not
    // change precedence eligibility.
    let mut predecessors: BTreeMap<WorkId, BTreeSet<WorkId>> = unit_ids
        .iter()
        .cloned()
        .map(|id| (id, BTreeSet::new()))
        .collect();

    let mut successors: BTreeMap<WorkId, BTreeSet<WorkId>> = unit_ids
        .iter()
        .cloned()
        .map(|id| (id, BTreeSet::new()))
        .collect();

    for dependency in dependencies {
        let source = dependency.source();
        let destination = dependency.destination();

        if !unit_ids.contains(source) || !unit_ids.contains(destination) {
            return Err(SchedulingError::UnknownDependencyEndpoint);
        }

        if successors
            .get_mut(source)
            .ok_or(SchedulingError::UnknownDependencyEndpoint)?
            .insert(destination.clone())
        {
            predecessors
                .get_mut(destination)
                .ok_or(SchedulingError::UnknownDependencyEndpoint)?
                .insert(source.clone());
        }
    }

    let topological = topological_order(&unit_ids, &predecessors, &successors)?;
    let critical_path = compute_critical_path(&topological, &successors, &profile_by_id)?;

    let critical_path_length = critical_path
        .values()
        .copied()
        .max()
        .unwrap_or(0);

    let mut remaining = unit_ids.clone();
    let mut completed = BTreeSet::new();

    // Running work is tracked by logical work-unit ID and planned finish time.
    let mut running: BTreeMap<WorkId, (ScheduleTime, BTreeMap<ResourceId, ResourceAmount>)> =
        BTreeMap::new();

    let mut used: BTreeMap<ResourceId, ResourceAmount> = capacity_by_id
        .keys()
        .cloned()
        .map(|id| (id, 0))
        .collect();

    let mut scheduled = Vec::with_capacity(units.len());
    let mut now: ScheduleTime = 0;

    while !remaining.is_empty() {
        // Complete all work that has finished at the current logical time.
        let finished_ids: Vec<WorkId> = running
            .iter()
            .filter(|(_, (finish, _))| *finish <= now)
            .map(|(id, _)| id.clone())
            .collect();

        for id in finished_ids {
            let (_, demands) = running
                .remove(&id)
                .ok_or(SchedulingError::NoProgress)?;

            for (resource_id, amount) in demands {
                let current = used
                    .get_mut(&resource_id)
                    .ok_or(SchedulingError::NoProgress)?;

                *current = current
                    .checked_sub(amount)
                    .ok_or(SchedulingError::ArithmeticOverflow)?;
            }

            completed.insert(id);
        }

        let mut ready: Vec<WorkId> = remaining
            .iter()
            .filter(|id| {
                predecessors
                    .get(*id)
                    .is_some_and(|required| required.iter().all(|p| completed.contains(p)))
            })
            .cloned()
            .collect();

        order_ready_work(
            &mut ready,
            policy.strategy,
            &profile_by_id,
            &critical_path,
        );

        let mut started_any = false;

        for id in ready {
            if policy
                .max_concurrent_units
                .is_some_and(|maximum| running.len() >= maximum)
            {
                break;
            }

            let profile = profile_by_id
                .get(&id)
                .copied()
                .ok_or(SchedulingError::MissingScheduleProfile)?;

            if !resources_fit(profile, &used, &capacity_by_id)? {
                continue;
            }

            let finish = now
                .checked_add(profile.duration())
                .ok_or(SchedulingError::ArithmeticOverflow)?;

            // Reserve only for positive-duration work. Zero-duration work
            // completes immediately, so it cannot deadlock the event loop.
            if profile.duration() == 0 {
                completed.insert(id.clone());
            } else {
                for (resource_id, amount) in profile.resource_demands() {
                    let current = used
                        .get_mut(resource_id)
                        .ok_or(SchedulingError::MissingResourceCapacity {
                            work_unit: format!("{id:?}"),
                            resource: format!("{resource_id:?}"),
                        })?;

                    *current = current
                        .checked_add(*amount)
                        .ok_or(SchedulingError::ArithmeticOverflow)?;
                }

                running.insert(
                    id.clone(),
                    (finish, profile.resource_demands().clone()),
                );
            }

            scheduled.push(ScheduledWork {
                work_id: id.clone(),
                start: now,
                finish,
                resource_demands: profile.resource_demands().clone(),
            });

            remaining.remove(&id);
            started_any = true;
        }

        if remaining.is_empty() {
            break;
        }

        // Re-evaluate at the same time after starting or completing work.
        // This permits zero-duration dependency chains to advance without
        // artificially adding time.
        if started_any {
            continue;
        }

        // If nothing is running, no ready task can start despite every
        // individual demand fitting its capacity. That indicates an
        // inconsistent scheduling state rather than a reason to spin.
        let next_finish = running
            .values()
            .map(|(finish, _)| *finish)
            .min();

        match next_finish {
            Some(finish) if finish > now => now = finish,
            _ => return Err(SchedulingError::NoProgress),
        }
    }

    scheduled.sort_by(|left, right| {
        left.start
            .cmp(&right.start)
            .then_with(|| left.work_id.cmp(&right.work_id))
    });

    let makespan = scheduled
        .iter()
        .map(|work| work.finish)
        .max()
        .unwrap_or(0);

    if let Some(deadline) = policy.deadline {
        if makespan > deadline {
            return Err(SchedulingError::DeadlineExceeded {
                makespan,
                deadline,
            });
        }
    }

    let plan = SchedulePlan {
        scheduled,
        makespan,
        critical_path_length,
    };

    validate_schedule(graph, profiles, &plan)?;

    Ok(plan)
}

fn topological_order<WorkId>(
    unit_ids: &BTreeSet<WorkId>,
    predecessors: &BTreeMap<WorkId, BTreeSet<WorkId>>,
    successors: &BTreeMap<WorkId, BTreeSet<WorkId>>,
) -> Result<Vec<WorkId>, SchedulingError>
where
    WorkId: Clone + Ord,
{
    let mut indegree: BTreeMap<WorkId, usize> = predecessors
        .iter()
        .map(|(id, parents)| (id.clone(), parents.len()))
        .collect();

    let mut ready: BTreeSet<WorkId> = indegree
        .iter()
        .filter(|(_, count)| **count == 0)
        .map(|(id, _)| id.clone())
        .collect();

    let mut order = Vec::with_capacity(unit_ids.len());

    while let Some(id) = ready.pop_first() {
        order.push(id.clone());

        if let Some(children) = successors.get(&id) {
            for child in children {
                let count = indegree
                    .get_mut(child)
                    .ok_or(SchedulingError::UnknownDependencyEndpoint)?;

                *count = count
                    .checked_sub(1)
                    .ok_or(SchedulingError::ArithmeticOverflow)?;

                if *count == 0 {
                    ready.insert(child.clone());
                }
            }
        }
    }

    if order.len() != unit_ids.len() {
        return Err(SchedulingError::DependencyCycle);
    }

    Ok(order)
}

fn compute_critical_path<WorkId, ResourceId>(
    topological: &[WorkId],
    successors: &BTreeMap<WorkId, BTreeSet<WorkId>>,
    profiles: &BTreeMap<WorkId, &ScheduleProfile<WorkId, ResourceId>>,
) -> Result<BTreeMap<WorkId, ScheduleTime>, SchedulingError>
where
    WorkId: Clone + Ord,
{
    let mut critical = BTreeMap::new();

    for id in topological.iter().rev() {
        let duration = profiles
            .get(id)
            .ok_or(SchedulingError::MissingScheduleProfile)?
            .duration();

        let longest_child = successors
            .get(id)
            .into_iter()
            .flat_map(|children| children.iter())
            .map(|child| {
                critical
                    .get(child)
                    .copied()
                    .ok_or(SchedulingError::NoProgress)
            })
            .collect::<Result<Vec<ScheduleTime>, SchedulingError>>()?
            .into_iter()
            .max()
            .unwrap_or(0);

        let length = duration
            .checked_add(longest_child)
            .ok_or(SchedulingError::ArithmeticOverflow)?;

        critical.insert(id.clone(), length);
    }

    Ok(critical)
}

fn order_ready_work<WorkId, ResourceId>(
    ready: &mut [WorkId],
    strategy: SchedulingStrategy,
    profiles: &BTreeMap<WorkId, &ScheduleProfile<WorkId, ResourceId>>,
    critical_path: &BTreeMap<WorkId, ScheduleTime>,
) where
    WorkId: Clone + Ord,
{
    ready.sort_by(|left, right| {
        let left_profile = profiles.get(left).copied();
        let right_profile = profiles.get(right).copied();

        match strategy {
            SchedulingStrategy::CriticalPathFirst => {
                critical_path
                    .get(right)
                    .cmp(&critical_path.get(left))
                    .then_with(|| left.cmp(right))
            }
            SchedulingStrategy::LongestProcessingTimeFirst => {
                right_profile
                    .map(|profile| profile.duration())
                    .cmp(&left_profile.map(|profile| profile.duration()))
                    .then_with(|| left.cmp(right))
            }
            SchedulingStrategy::StableIdentifierOrder => left.cmp(right),
        }
    });
}

fn resources_fit<WorkId, ResourceId>(
    profile: &ScheduleProfile<WorkId, ResourceId>,
    used: &BTreeMap<ResourceId, ResourceAmount>,
    capacities: &BTreeMap<ResourceId, ResourceAmount>,
) -> Result<bool, SchedulingError>
where
    WorkId: fmt::Debug,
    ResourceId: Clone + Ord + fmt::Debug,
{
    for (resource_id, required) in profile.resource_demands() {
        let available = capacities.get(resource_id).copied().ok_or_else(|| {
            SchedulingError::MissingResourceCapacity {
                work_unit: format!("{:?}", profile.work_id()),
                resource: format!("{resource_id:?}"),
            }
        })?;

        let current = used.get(resource_id).copied().unwrap_or(0);
        let remaining = available
            .checked_sub(current)
            .ok_or(SchedulingError::ArithmeticOverflow)?;

        if *required > remaining {
            return Ok(false);
        }
    }

    Ok(true)
}

/// Validates schedule coverage, dependency order, duration, and resource
/// capacity constraints against the original graph and supplied profiles.
///
/// This validation does not certify that the resource snapshot remains
/// current, that placement is authorized, or that runtime dispatch succeeds.
pub fn validate_schedule<WorkId, ResourceId>(
    graph: &WorkloadGraph<WorkId>,
    profiles: &[ScheduleProfile<WorkId, ResourceId>],
    plan: &SchedulePlan<WorkId, ResourceId>,
) -> Result<(), SchedulingError>
where
    WorkId: Clone + Ord + fmt::Debug,
    ResourceId: Clone + Ord + fmt::Debug,
{
    let mut expected = BTreeSet::new();
    for unit in graph.units() {
        if !expected.insert(unit.id().clone()) {
            return Err(SchedulingError::DuplicateWorkUnit);
        }
    }

    let mut profile_by_id = BTreeMap::new();
    for profile in profiles {
        if !expected.contains(profile.work_id()) {
            return Err(SchedulingError::UnknownScheduleProfile);
        }
        if profile_by_id
            .insert(profile.work_id().clone(), profile)
            .is_some()
        {
            return Err(SchedulingError::DuplicateScheduleProfile);
        }
    }

    if profile_by_id.len() != expected.len() {
        return Err(SchedulingError::MissingScheduleProfile);
    }

    let mut by_id = BTreeMap::new();
    for work in plan.scheduled() {
        if !expected.contains(work.work_id()) {
            return Err(SchedulingError::UnknownScheduleProfile);
        }
        if by_id.insert(work.work_id().clone(), work).is_some() {
            return Err(SchedulingError::DuplicateScheduleProfile);
        }
    }

    if by_id.len() != expected.len() {
        return Err(SchedulingError::MissingScheduleProfile);
    }

    for dependency in graph.dependencies() {
        let source = by_id
            .get(dependency.source())
            .ok_or(SchedulingError::UnknownDependencyEndpoint)?;
        let destination = by_id
            .get(dependency.destination())
            .ok_or(SchedulingError::UnknownDependencyEndpoint)?;

        if destination.start() < source.finish() {
            return Err(SchedulingError::DependencyCycle);
        }
    }

    for (id, work) in &by_id {
        let profile = profile_by_id
            .get(id)
            .copied()
            .ok_or(SchedulingError::MissingScheduleProfile)?;

        let expected_finish = work
            .start()
            .checked_add(profile.duration())
            .ok_or(SchedulingError::ArithmeticOverflow)?;

        if work.finish() != expected_finish {
            return Err(SchedulingError::ArithmeticOverflow);
        }

        if work.resource_demands() != profile.resource_demands() {
            return Err(SchedulingError::DuplicateResourceDemand);
        }
    }

    let makespan = plan
        .scheduled()
        .iter()
        .map(|work| work.finish())
        .max()
        .unwrap_or(0);

    if makespan != plan.makespan() {
        return Err(SchedulingError::ArithmeticOverflow);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::distributed::planning::partition::{
        Dependency, WorkUnit,
    };

    fn unit(id: u64) -> WorkUnit<u64> {
        WorkUnit::new(id, 1, None)
    }

    fn graph(
        ids: &[u64],
        edges: &[(u64, u64)],
    ) -> WorkloadGraph<u64> {
        WorkloadGraph::new(
            ids.iter().copied().map(unit).collect(),
            edges
                .iter()
                .map(|(source, destination)| {
                    Dependency::new(*source, *destination, 0)
                })
                .collect(),
        )
    }

    fn profile(id: u64, duration: u128) -> ScheduleProfile<u64, &'static str> {
        ScheduleProfile::new(id, duration)
    }

    #[test]
    fn independent_work_can_run_concurrently() {
        let workload = graph(&[1, 2], &[]);
        let profiles = vec![profile(1, 5), profile(2, 7)];

        let plan = schedule_workload(
            &workload,
            &profiles,
            &[],
            SchedulingPolicy::new(),
        )
        .expect("independent work should schedule");

        assert_eq!(plan.makespan(), 7);
        assert_eq!(plan.work(&1).expect("task 1").start(), 0);
        assert_eq!(plan.work(&2).expect("task 2").start(), 0);
    }

    #[test]
    fn dependencies_are_respected() {
        let workload = graph(&[1, 2, 3], &[(1, 2), (2, 3)]);
        let profiles = vec![
            profile(1, 4),
            profile(2, 3),
            profile(3, 2),
        ];

        let plan = schedule_workload(
            &workload,
            &profiles,
            &[],
            SchedulingPolicy::new(),
        )
        .expect("acyclic work should schedule");

        assert_eq!(plan.work(&1).expect("task 1").start(), 0);
        assert_eq!(plan.work(&2).expect("task 2").start(), 4);
        assert_eq!(plan.work(&3).expect("task 3").start(), 7);
        assert_eq!(plan.makespan(), 9);
    }

    #[test]
    fn shared_capacity_serializes_conflicting_work() {
        let workload = graph(&[1, 2], &[]);
        let profiles = vec![
            ScheduleProfile::new(1, 4)
                .with_resource_demands([("worker-slot", 1)])
                .expect("unique demand"),
            ScheduleProfile::new(2, 3)
                .with_resource_demands([("worker-slot", 1)])
                .expect("unique demand"),
        ];
        let capacities = vec![ResourceCapacity::new("worker-slot", 1)];

        let plan = schedule_workload(
            &workload,
            &profiles,
            &capacities,
            SchedulingPolicy::new(),
        )
        .expect("tasks should serialize within capacity");

        assert_eq!(plan.makespan(), 7);
        assert_ne!(
            plan.work(&1).expect("task 1").start(),
            plan.work(&2).expect("task 2").start()
        );
    }

    #[test]
    fn detects_cycles() {
        let workload = graph(&[1, 2], &[(1, 2), (2, 1)]);
        let profiles = vec![profile(1, 1), profile(2, 1)];

        assert_eq!(
            schedule_workload(
                &workload,
                &profiles,
                &[],
                SchedulingPolicy::new(),
            ),
            Err(SchedulingError::DependencyCycle)
        );
    }

    #[test]
    fn rejects_missing_profile() {
        let workload = graph(&[1, 2], &[]);
        let profiles = vec![profile(1, 1)];

        assert_eq!(
            schedule_workload(
                &workload,
                &profiles,
                &[],
                SchedulingPolicy::new(),
            ),
            Err(SchedulingError::MissingScheduleProfile)
        );
    }

    #[test]
    fn rejects_resource_demand_exceeding_capacity() {
        let workload = graph(&[1], &[]);
        let profiles = vec![
            ScheduleProfile::new(1, 1)
                .with_resource_demands([("memory-unit", 8)])
                .expect("unique demand"),
        ];
        let capacities = vec![ResourceCapacity::new("memory-unit", 4)];

        assert!(matches!(
            schedule_workload(
                &workload,
                &profiles,
                &capacities,
                SchedulingPolicy::new(),
            ),
            Err(SchedulingError::ResourceCapacityUnsatisfied {
                required: 8,
                available: 4,
                ..
            })
        ));
    }

    #[test]
    fn zero_duration_dependency_chain_completes_without_time_inflation() {
        let workload = graph(&[1, 2, 3], &[(1, 2), (2, 3)]);
        let profiles = vec![
            profile(1, 0),
            profile(2, 0),
            profile(3, 2),
        ];

        let plan = schedule_workload(
            &workload,
            &profiles,
            &[],
            SchedulingPolicy::new(),
        )
        .expect("zero-duration tasks should complete");

        assert_eq!(plan.work(&1).expect("task 1").finish(), 0);
        assert_eq!(plan.work(&2).expect("task 2").start(), 0);
        assert_eq!(plan.work(&3).expect("task 3").start(), 0);
        assert_eq!(plan.makespan(), 2);
    }

    #[test]
    fn deadline_is_enforced_without_truncating_work() {
        let workload = graph(&[1], &[]);
        let profiles = vec![profile(1, 9)];

        assert_eq!(
            schedule_workload(
                &workload,
                &profiles,
                &[],
                SchedulingPolicy::new().with_deadline(Some(8)),
            ),
            Err(SchedulingError::DeadlineExceeded {
                makespan: 9,
                deadline: 8,
            })
        );
    }

    #[test]
    fn empty_workload_has_empty_schedule() {
        let workload = graph(&[], &[]);

        let plan = schedule_workload(
            &workload,
            &[],
            &[],
            SchedulingPolicy::new(),
        )
        .expect("empty workload is valid");

        assert!(plan.is_empty());
        assert_eq!(plan.makespan(), 0);
        assert_eq!(plan.critical_path_length(), 0);
    }

    #[test]
    fn output_is_deterministic_for_reordered_inputs() {
        let first = WorkloadGraph::new(
            vec![unit(1), unit(2), unit(3)],
            vec![
                Dependency::new(1, 3, 0),
                Dependency::new(2, 3, 0),
            ],
        );
        let second = WorkloadGraph::new(
            vec![unit(3), unit(2), unit(1)],
            vec![
                Dependency::new(2, 3, 0),
                Dependency::new(1, 3, 0),
            ],
        );

        let profiles = vec![
            profile(3, 1),
            profile(2, 4),
            profile(1, 4),
        ];

        let left = schedule_workload(
            &first,
            &profiles,
            &[],
            SchedulingPolicy::new(),
        )
        .expect("first graph should schedule");

        let right = schedule_workload(
            &second,
            &profiles,
            &[],
            SchedulingPolicy::new(),
        )
        .expect("second graph should schedule");

        assert_eq!(left, right);
    }
}
