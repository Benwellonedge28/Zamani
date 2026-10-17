
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Classical Execution Scheduling
//!
//! Path: `src/classical/execution/scheduling.rs`
//! Edition: Rust 2021
//! Minimum Rust version: Rust 1.97.
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Produce a deterministic, dependency-respecting schedule for an existing
//! target-independent [`ExecutionPlan`], given explicit per-step durations,
//! per-step resource usage, and resource-capacity evidence.
//!
//! ## Authority and ownership
//!
//! - `grammar/execution/scheduling.g4` owns scheduling-intent syntax.
//! - `grammar/specification/execution-model.md` defines execution boundaries.
//! - `src/classical/execution/plan.rs` owns the execution plan and step
//!   dependency semantics.
//! - `src/frontend/semantic/resources/model.rs` owns canonical semantic
//!   resource requirements and constraints.
//! - `src/frontend/semantic/capabilities/model.rs` owns semantic capability
//!   requirements.
//! - `src/zuir/` owns universal computation IR.
//! - `src/quantum/ir/` remains the canonical quantum IR.
//! - Target adapters own interpretation of target timing and capacity data.
//! - Runtime infrastructure owns actual dispatch, execution, and observation.
//!
//! This module owns a deterministic list-scheduling algorithm and its
//! schedule artifact. It does not own source-language semantics, canonical
//! IR identities, resource discovery, placement, routing, or runtime work.
//!
//! ## Scheduling contract
//!
//! Every source-plan step must have exactly one explicit [`StepScheduleSpec`].
//! A duration is expressed in caller-defined abstract time units; it is never
//! inferred from the operation name.
//!
//! Resource capacities are supplied explicitly. They are not guessed from
//! the host machine. A resource demand without corresponding capacity
//! evidence is rejected rather than treated as unlimited.
//!
//! The algorithm schedules steps in deterministic topological order and
//! chooses the earliest start time compatible with already scheduled steps,
//! dependency completion, durations, and resource capacities.
//!
//! This is a deterministic greedy heuristic, not a globally optimal
//! makespan optimizer. A different valid schedule may have a shorter total
//! duration. Scheduling does not change the meaning of the source program.
//!
//! ## Time and resource semantics
//!
//! Time is represented by non-negative `u128` ticks. The meaning and scale
//! of a tick are defined by the caller's timing adapter. Arithmetic overflow
//! is reported; time never silently wraps.
//!
//! Resource reservations occupy half-open intervals `[start, finish)`.
//! Consequently, a reservation finishing at tick `t` does not conflict with
//! another reservation starting at `t`.
//!
//! Zero-duration operations occupy no interval. Their dependencies still
//! constrain their start and finish times.
//!
//! Resource quantities are abstract units of named resource classes. They
//! do not identify a physical processor, device, memory address, node, or
//! hardware resource.
//!
//! ## Scalability and determinism
//!
//! No universal maximum is imposed on the number of steps, resource classes,
//! or resource reservations. Collections grow according to available memory.
//! Explicit operational budgets belong to the compilation/execution context.
//!
//! Ordered collections and stable tie-breaking make the result deterministic
//! for the same plan, specifications, capacities, and algorithm version.
//! This does not imply deterministic runtime behavior or literal infinite
//! capacity.
//!
//! ## Non-responsibilities
//!
//! This module does not:
//!
//! - parse scheduling grammar or interpret AST nodes;
//! - infer operation durations or resource usage;
//! - invent missing resource capacities;
//! - prove that a capability requirement is satisfied;
//! - select physical devices or execution locations;
//! - perform placement, routing, allocation, or runtime dispatch;
//! - add synchronization or alter dependency semantics;
//! - guarantee an optimal schedule;
//! - define a competing IR or canonical resource model.
//!
//! ## Integration status
//!
//! The implementation depends on the public API in `plan.rs`. The parent
//! execution module must register this module. Frontend and target adapters
//! must explicitly construct the duration, usage, and capacity inputs.
//! Compilation and repository tests are required before this module can be
//! considered verified.

use core::fmt;
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;

use super::plan::{ExecutionPlan, ExecutionPlanError, StepId};

/// The result of a scheduling operation.
pub type SchedulingResult<T> = Result<T, SchedulingError>;

/// Abstract non-negative scheduling time in caller-defined ticks.
///
/// This is not a wall-clock timestamp, hardware cycle count, or universal
/// physical time unit. Its interpretation is defined by the caller.
pub type ScheduleTime = u128;

/// A validated, extensible resource-class identifier.
///
/// The identifier describes a logical resource category. It does not identify
/// a physical device or imply that all resources of this class are fungible.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ResourceClass(String);

impl ResourceClass {
    /// Construct a resource-class identifier.
    ///
    /// Identifiers use the same open, ASCII namespace spelling accepted by
    /// the execution-plan identifier contract: letters, digits, `.`, `_`,
    /// `-`, `:`, and `/`; at least one letter or digit is required.
    ///
    /// # Errors
    ///
    /// Returns [`SchedulingError::InvalidResourceClass`] for an invalid name.
    pub fn new(value: impl Into<String>) -> SchedulingResult<Self> {
        let value = value.into();

        if !is_valid_identifier(&value) {
            return Err(SchedulingError::InvalidResourceClass { value });
        }

        Ok(Self(value))
    }

    /// Return the identifier's original spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ResourceClass {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// An abstract amount of a named resource required by one step.
///
/// The amount is measured in the unit defined by the resource adapter.
/// Zero is valid and means that the step reserves none of that resource.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResourceUse {
    resource_class: ResourceClass,
    amount: u128,
}

impl ResourceUse {
    /// Construct a resource usage declaration.
    #[must_use]
    pub const fn new(resource_class: ResourceClass, amount: u128) -> Self {
        Self {
            resource_class,
            amount,
        }
    }

    /// Return the resource class.
    #[must_use]
    pub fn resource_class(&self) -> &ResourceClass {
        &self.resource_class
    }

    /// Return the requested amount.
    #[must_use]
    pub const fn amount(&self) -> u128 {
        self.amount
    }
}

/// Explicit timing and resource requirements for one plan step.
///
/// Exactly one specification must be supplied for every step in the source
/// execution plan. Durations and resource usage are supplied by an upstream
/// adapter or an explicitly selected execution policy; this module does not
/// infer them from operation names.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StepScheduleSpec {
    step: StepId,
    duration: ScheduleTime,
    resource_usage: BTreeMap<ResourceClass, u128>,
}

impl StepScheduleSpec {
    /// Construct an explicit specification for a plan step.
    ///
    /// Duplicate resource classes are rejected rather than silently added.
    /// Resource aggregation belongs to the authoritative resource adapter,
    /// which must provide one already-aggregated quantity per class.
    pub fn new(
        step: StepId,
        duration: ScheduleTime,
        resource_usage: impl IntoIterator<Item = ResourceUse>,
    ) -> SchedulingResult<Self> {
        let mut usage = BTreeMap::new();

        for item in resource_usage {
            let class = item.resource_class;

            if usage.insert(class.clone(), item.amount).is_some() {
                return Err(SchedulingError::DuplicateStepResource {
                    step,
                    resource_class: class,
                });
            }
        }

        Ok(Self {
            step,
            duration,
            resource_usage: usage,
        })
    }

    /// Return the plan step described by this specification.
    #[must_use]
    pub fn step(&self) -> &StepId {
        &self.step
    }

    /// Return the declared duration.
    #[must_use]
    pub const fn duration(&self) -> ScheduleTime {
        self.duration
    }

    /// Return the per-step resource usage, ordered by resource class.
    #[must_use]
    pub fn resource_usage(&self) -> &BTreeMap<ResourceClass, u128> {
        &self.resource_usage
    }
}

/// Supplied capacity evidence for a logical resource class.
///
/// The capacity is the maximum aggregate amount that may be reserved
/// concurrently in that class. It is not an allocation and does not identify
/// a physical resource.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResourceCapacity {
    resource_class: ResourceClass,
    capacity: u128,
}

impl ResourceCapacity {
    /// Construct a capacity declaration.
    #[must_use]
    pub const fn new(resource_class: ResourceClass, capacity: u128) -> Self {
        Self {
            resource_class,
            capacity,
        }
    }

    /// Return the resource class.
    #[must_use]
    pub fn resource_class(&self) -> &ResourceClass {
        &self.resource_class
    }

    /// Return the declared capacity.
    #[must_use]
    pub const fn capacity(&self) -> u128 {
        self.capacity
    }
}

/// One scheduled step and its half-open resource reservation interval.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ScheduledStep {
    step: StepId,
    start: ScheduleTime,
    finish: ScheduleTime,
    resource_usage: BTreeMap<ResourceClass, u128>,
}

impl ScheduledStep {
    /// Return the plan-local step identifier.
    #[must_use]
    pub fn step(&self) -> &StepId {
        &self.step
    }

    /// Return the scheduled start time.
    #[must_use]
    pub const fn start(&self) -> ScheduleTime {
        self.start
    }

    /// Return the scheduled finish time.
    #[must_use]
    pub const fn finish(&self) -> ScheduleTime {
        self.finish
    }

    /// Return the scheduled duration.
    #[must_use]
    pub const fn duration(&self) -> ScheduleTime {
        self.finish - self.start
    }

    /// Return this step's resource usage.
    #[must_use]
    pub fn resource_usage(&self) -> &BTreeMap<ResourceClass, u128> {
        &self.resource_usage
    }
}

/// A deterministic, validated schedule for an execution plan.
///
/// This is a planning artifact, not a runtime executor. Entries are stored
/// by step identifier, while [`Schedule::order`] retains the deterministic
/// dependency order used by the scheduler.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Schedule {
    plan_id: String,
    entries: BTreeMap<StepId, ScheduledStep>,
    order: Vec<StepId>,
    makespan: ScheduleTime,
}

impl Schedule {
    /// Return the source execution-plan identifier.
    #[must_use]
    pub fn plan_id(&self) -> &str {
        &self.plan_id
    }

    /// Return all scheduled entries keyed by step identifier.
    #[must_use]
    pub fn entries(&self) -> &BTreeMap<StepId, ScheduledStep> {
        &self.entries
    }

    /// Look up a scheduled step.
    #[must_use]
    pub fn step(&self, id: &StepId) -> Option<&ScheduledStep> {
        self.entries.get(id)
    }

    /// Return the deterministic topological order used during scheduling.
    #[must_use]
    pub fn order(&self) -> &[StepId] {
        &self.order
    }

    /// Return the schedule's makespan.
    ///
    /// For an empty schedule, the makespan is zero.
    #[must_use]
    pub const fn makespan(&self) -> ScheduleTime {
        self.makespan
    }

    /// Return the number of scheduled steps.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Return whether the schedule contains no steps.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Validate basic schedule consistency against its source plan.
    ///
    /// This verifies complete membership, dependency ordering, duration
    /// consistency, and the makespan. It does not revalidate resource
    /// capacities; use [`schedule_plan`] to construct a capacity-checked
    /// schedule.
    pub fn validate_against(&self, plan: &ExecutionPlan) -> SchedulingResult<()> {
        if self.plan_id != plan.id() {
            return Err(SchedulingError::PlanIdentityMismatch {
                expected: plan.id().to_owned(),
                actual: self.plan_id.clone(),
            });
        }

        let topological_order = plan
            .topological_order()
            .map_err(SchedulingError::InvalidExecutionPlan)?;

        if self.entries.len() != plan.steps().len()
            || self.order.len() != plan.steps().len()
        {
            return Err(SchedulingError::IncompleteSchedule {
                expected: plan.steps().len(),
                actual: self.entries.len(),
            });
        }

        let mut seen = BTreeSet::new();

        for id in &self.order {
            if !seen.insert(id.clone()) {
                return Err(SchedulingError::DuplicateScheduledStep {
                    step: id.clone(),
                });
            }

            if !self.entries.contains_key(id) {
                return Err(SchedulingError::MissingScheduledStep {
                    step: id.clone(),
                });
            }
        }

        if seen.len() != self.entries.len() || topological_order.len() != self.order.len() {
            return Err(SchedulingError::IncompleteSchedule {
                expected: topological_order.len(),
                actual: seen.len(),
            });
        }

        let plan_steps: BTreeMap<_, _> = plan
            .steps()
            .iter()
            .map(|step| (step.id().clone(), step))
            .collect();

        for (id, entry) in &self.entries {
            if id != &entry.step {
                return Err(SchedulingError::InconsistentScheduledIdentity {
                    key: id.clone(),
                    value: entry.step.clone(),
                });
            }

            if entry.finish < entry.start {
                return Err(SchedulingError::InvalidTimeInterval {
                    step: id.clone(),
                    start: entry.start,
                    finish: entry.finish,
                });
            }

            let source_step = plan_steps
                .get(id)
                .ok_or_else(|| SchedulingError::UnknownScheduledStep {
                    step: id.clone(),
                })?;

            for dependency in source_step.dependencies() {
                let predecessor = self.entries.get(dependency).ok_or_else(|| {
                    SchedulingError::MissingScheduledStep {
                        step: dependency.clone(),
                    }
                })?;

                if predecessor.finish > entry.start {
                    return Err(SchedulingError::DependencyOrderViolation {
                        step: id.clone(),
                        dependency: dependency.clone(),
                    });
                }
            }
        }

        let actual_makespan = self
            .entries
            .values()
            .map(|entry| entry.finish)
            .max()
            .unwrap_or(0);

        if actual_makespan != self.makespan {
            return Err(SchedulingError::InconsistentMakespan {
                expected: actual_makespan,
                actual: self.makespan,
            });
        }

        Ok(())
    }
}

/// Structured scheduling errors.
///
/// Error variants are machine-readable. Consumers should match variants,
/// not parse their display messages.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum SchedulingError {
    /// The source execution plan is invalid.
    InvalidExecutionPlan(ExecutionPlanError),

    /// A resource-class identifier is invalid.
    InvalidResourceClass {
        /// Rejected identifier.
        value: String,
    },

    /// More than one usage entry was supplied for the same step and class.
    DuplicateStepResource {
        /// Step containing the duplicate.
        step: StepId,
        /// Duplicated resource class.
        resource_class: ResourceClass,
    },

    /// A step specification references a step absent from the plan.
    UnknownStepSpecification {
        /// Unknown step.
        step: StepId,
    },

    /// A plan step has more than one schedule specification.
    DuplicateStepSpecification {
        /// Duplicated step.
        step: StepId,
    },

    /// A plan step has no schedule specification.
    MissingStepSpecification {
        /// Unspecified step.
        step: StepId,
    },

    /// A resource capacity was supplied more than once.
    DuplicateResourceCapacity {
        /// Duplicated class.
        resource_class: ResourceClass,
    },

    /// A required resource class has no supplied capacity evidence.
    MissingResourceCapacity {
        /// Step requiring the resource.
        step: StepId,
        /// Resource class without capacity evidence.
        resource_class: ResourceClass,
    },

    /// One step's individual demand exceeds the supplied capacity.
    ResourceDemandExceedsCapacity {
        /// Step that cannot fit the declared capacity.
        step: StepId,
        /// Resource class that is insufficient.
        resource_class: ResourceClass,
        /// Requested amount.
        requested: u128,
        /// Supplied capacity.
        capacity: u128,
    },

    /// The end time cannot be represented by `u128`.
    TimeOverflow {
        /// Step whose finish time overflowed.
        step: StepId,
        /// Candidate start time.
        start: ScheduleTime,
        /// Declared duration.
        duration: ScheduleTime,
    },

    /// A resource conflict could not identify a strictly later release time.
    InvalidResourceRelease {
        /// Step being scheduled.
        step: StepId,
        /// Candidate start time.
        candidate: ScheduleTime,
    },

    /// The schedule refers to a different source plan.
    PlanIdentityMismatch {
        /// Expected source plan identifier.
        expected: String,
        /// Actual schedule plan identifier.
        actual: String,
    },

    /// A schedule does not contain exactly one entry for each plan step.
    IncompleteSchedule {
        /// Expected number of steps.
        expected: usize,
        /// Actual number of entries or unique steps.
        actual: usize,
    },

    /// The schedule's order contains a duplicate step.
    DuplicateScheduledStep {
        /// Duplicated step.
        step: StepId,
    },

    /// A required scheduled step is missing.
    MissingScheduledStep {
        /// Missing step.
        step: StepId,
    },

    /// A schedule contains a step absent from the plan.
    UnknownScheduledStep {
        /// Unknown step.
        step: StepId,
    },

    /// A map key disagrees with the scheduled entry's identity.
    InconsistentScheduledIdentity {
        /// Map key.
        key: StepId,
        /// Stored step identity.
        value: StepId,
    },

    /// A scheduled interval ends before it starts.
    InvalidTimeInterval {
        /// Step with an invalid interval.
        step: StepId,
        /// Start time.
        start: ScheduleTime,
        /// Finish time.
        finish: ScheduleTime,
    },

    /// A step starts before one of its dependencies finishes.
    DependencyOrderViolation {
        /// Dependent step.
        step: StepId,
        /// Required predecessor.
        dependency: StepId,
    },

    /// The stored makespan does not equal the latest finish time.
    InconsistentMakespan {
        /// Recomputed makespan.
        expected: ScheduleTime,
        /// Stored makespan.
        actual: ScheduleTime,
    },
}

impl fmt::Display for SchedulingError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidExecutionPlan(error) => {
                write!(formatter, "invalid execution plan: {error}")
            }
            Self::InvalidResourceClass { value } => {
                write!(formatter, "invalid resource-class identifier `{value}`")
            }
            Self::DuplicateStepResource {
                step,
                resource_class,
            } => write!(
                formatter,
                "step `{step}` specifies resource class `{resource_class}` more than once"
            ),
            Self::UnknownStepSpecification { step } => {
                write!(formatter, "schedule specification references unknown step `{step}`")
            }
            Self::DuplicateStepSpecification { step } => {
                write!(formatter, "step `{step}` has more than one schedule specification")
            }
            Self::MissingStepSpecification { step } => {
                write!(formatter, "step `{step}` has no schedule specification")
            }
            Self::DuplicateResourceCapacity { resource_class } => {
                write!(formatter, "resource class `{resource_class}` has duplicate capacity data")
            }
            Self::MissingResourceCapacity {
                step,
                resource_class,
            } => write!(
                formatter,
                "step `{step}` requires resource class `{resource_class}` without capacity evidence"
            ),
            Self::ResourceDemandExceedsCapacity {
                step,
                resource_class,
                requested,
                capacity,
            } => write!(
                formatter,
                "step `{step}` requests {requested} units of `{resource_class}`, \
                 exceeding capacity {capacity}"
            ),
            Self::TimeOverflow {
                step,
                start,
                duration,
            } => write!(
                formatter,
                "schedule time overflow for step `{step}`: {start} + {duration}"
            ),
            Self::InvalidResourceRelease { step, candidate } => write!(
                formatter,
                "resource conflict for step `{step}` at {candidate} has no later release time"
            ),
            Self::PlanIdentityMismatch { expected, actual } => write!(
                formatter,
                "schedule belongs to plan `{actual}`, not expected plan `{expected}`"
            ),
            Self::IncompleteSchedule { expected, actual } => write!(
                formatter,
                "schedule contains {actual} entries; expected {expected}"
            ),
            Self::DuplicateScheduledStep { step } => {
                write!(formatter, "step `{step}` occurs more than once in schedule order")
            }
            Self::MissingScheduledStep { step } => {
                write!(formatter, "scheduled step `{step}` is missing")
            }
            Self::UnknownScheduledStep { step } => {
                write!(formatter, "schedule contains unknown step `{step}`")
            }
            Self::InconsistentScheduledIdentity { key, value } => write!(
                formatter,
                "schedule map key `{key}` differs from stored step `{value}`"
            ),
            Self::InvalidTimeInterval {
                step,
                start,
                finish,
            } => write!(
                formatter,
                "step `{step}` has invalid interval [{start}, {finish})"
            ),
            Self::DependencyOrderViolation { step, dependency } => write!(
                formatter,
                "step `{step}` starts before dependency `{dependency}` finishes"
            ),
            Self::InconsistentMakespan { expected, actual } => write!(
                formatter,
                "schedule makespan is {actual}; recomputed makespan is {expected}"
            ),
        }
    }
}

impl Error for SchedulingError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidExecutionPlan(error) => Some(error),
            _ => None,
        }
    }
}

/// Build a deterministic schedule for an existing execution plan.
///
/// Every step requires an explicit specification. Resource capacities must
/// be supplied for every resource class with positive usage. Unused capacity
/// declarations are permitted.
///
/// The scheduler uses a deterministic greedy list algorithm:
///
/// 1. Obtain the source plan's deterministic topological order.
/// 2. Set each step's earliest start to the maximum finish of its dependencies.
/// 3. Find the earliest interval that satisfies all supplied resource
///    capacities against already scheduled intervals.
/// 4. Record the interval and continue in topological order.
///
/// The algorithm is deterministic but is not guaranteed to minimize makespan.
/// It performs no target discovery, placement, allocation, or execution.
///
/// # Errors
///
/// Returns an error for an invalid plan, missing/duplicate/unknown step
/// specifications, duplicate capacities, missing capacity evidence,
/// individually infeasible resource demands, or time overflow.
pub fn schedule_plan(
    plan: &ExecutionPlan,
    specifications: impl IntoIterator<Item = StepScheduleSpec>,
    capacities: impl IntoIterator<Item = ResourceCapacity>,
) -> SchedulingResult<Schedule> {
    let topological_order = plan
        .topological_order()
        .map_err(SchedulingError::InvalidExecutionPlan)?;

    let plan_steps: BTreeMap<_, _> = plan
        .steps()
        .iter()
        .map(|step| (step.id().clone(), step))
        .collect();

    let mut specs = BTreeMap::new();

    for spec in specifications {
        if !plan_steps.contains_key(&spec.step) {
            return Err(SchedulingError::UnknownStepSpecification {
                step: spec.step,
            });
        }

        let id = spec.step.clone();

        if specs.insert(id.clone(), spec).is_some() {
            return Err(SchedulingError::DuplicateStepSpecification { step: id });
        }
    }

    for id in &topological_order {
        if !specs.contains_key(id) {
            return Err(SchedulingError::MissingStepSpecification {
                step: id.clone(),
            });
        }
    }

    let mut capacity_by_class = BTreeMap::new();

    for capacity in capacities {
        let class = capacity.resource_class;

        if capacity_by_class
            .insert(class.clone(), capacity.capacity)
            .is_some()
        {
            return Err(SchedulingError::DuplicateResourceCapacity {
                resource_class: class,
            });
        }
    }

    for spec in specs.values() {
        for (class, amount) in &spec.resource_usage {
            if *amount == 0 {
                continue;
            }

            let capacity = capacity_by_class.get(class).copied().ok_or_else(|| {
                SchedulingError::MissingResourceCapacity {
                    step: spec.step.clone(),
                    resource_class: class.clone(),
                }
            })?;

            if *amount > capacity {
                return Err(SchedulingError::ResourceDemandExceedsCapacity {
                    step: spec.step.clone(),
                    resource_class: class.clone(),
                    requested: *amount,
                    capacity,
                });
            }
        }
    }

    let mut entries: BTreeMap<StepId, ScheduledStep> = BTreeMap::new();
    let mut makespan = 0;

    for id in &topological_order {
        let source_step = plan_steps
            .get(id)
            .expect("topological order contains only validated plan steps");

        let spec = specs
            .get(id)
            .expect("every plan step was checked for a specification");

        let mut earliest_start = 0;

        for dependency in source_step.dependencies() {
            let predecessor = entries
                .get(dependency)
                .expect("topological order places dependencies first");

            earliest_start = earliest_start.max(predecessor.finish);
        }

        let start = earliest_feasible_start(
            id,
            earliest_start,
            spec.duration,
            &spec.resource_usage,
            &capacity_by_class,
            &entries,
        )?;

        let finish = start
            .checked_add(spec.duration)
            .ok_or_else(|| SchedulingError::TimeOverflow {
                step: id.clone(),
                start,
                duration: spec.duration,
            })?;

        makespan = makespan.max(finish);

        entries.insert(
            id.clone(),
            ScheduledStep {
                step: id.clone(),
                start,
                finish,
                resource_usage: spec.resource_usage.clone(),
            },
        );
    }

    let schedule = Schedule {
        plan_id: plan.id().to_owned(),
        entries,
        order: topological_order,
        makespan,
    };

    schedule.validate_against(plan)?;
    Ok(schedule)
}

/// Find the earliest start time that satisfies all resource capacities.
///
/// Reservations use half-open intervals. When an overlap violates a capacity,
/// the candidate advances to a reservation-release boundary and is checked
/// again. This method is a feasibility search for the chosen list order, not
/// an optimal global scheduling algorithm.
fn earliest_feasible_start(
    step: &StepId,
    mut candidate: ScheduleTime,
    duration: ScheduleTime,
    requested: &BTreeMap<ResourceClass, u128>,
    capacities: &BTreeMap<ResourceClass, u128>,
    scheduled: &BTreeMap<StepId, ScheduledStep>,
) -> SchedulingResult<ScheduleTime> {
    if duration == 0 || requested.values().all(|amount| *amount == 0) {
        candidate
            .checked_add(duration)
            .ok_or_else(|| SchedulingError::TimeOverflow {
                step: step.clone(),
                start: candidate,
                duration,
            })?;
        return Ok(candidate);
    }

    loop {
        let candidate_finish = candidate
            .checked_add(duration)
            .ok_or_else(|| SchedulingError::TimeOverflow {
                step: step.clone(),
                start: candidate,
                duration,
            })?;

        let mut earliest_release: Option<ScheduleTime> = None;

        for (class, amount) in requested {
            if *amount == 0 {
                continue;
            }

            let capacity = *capacities
                .get(class)
                .expect("positive demands were checked for capacity evidence");

            if let Some(release) = resource_conflict_release(
                class,
                *amount,
                capacity,
                candidate,
                candidate_finish,
                scheduled,
            ) {
                earliest_release = Some(
                    earliest_release.map_or(release, |current| current.min(release)),
                );
            }
        }

        match earliest_release {
            None => return Ok(candidate),
            Some(release) if release > candidate => candidate = release,
            Some(_) => {
                return Err(SchedulingError::InvalidResourceRelease {
                    step: step.clone(),
                    candidate,
                });
            }
        }
    }
}

/// Return the earliest release boundary that resolves a capacity conflict
/// for one resource class, if the candidate interval conflicts.
///
/// Existing reservation start/end events are swept in deterministic time
/// order. At each interval, the active usage plus the candidate's usage must
/// fit within the declared capacity. A conflict yields a release boundary
/// strictly later than the candidate start.
fn resource_conflict_release(
    class: &ResourceClass,
    requested: u128,
    capacity: u128,
    candidate_start: ScheduleTime,
    candidate_finish: ScheduleTime,
    scheduled: &BTreeMap<StepId, ScheduledStep>,
) -> Option<ScheduleTime> {
    let mut events: BTreeMap<
        ScheduleTime,
        (Vec<(StepId, u128)>, Vec<StepId>),
    > = BTreeMap::new();

    events.entry(candidate_start).or_default();
    events.entry(candidate_finish).or_default();

    for entry in scheduled.values() {
        let amount = entry.resource_usage.get(class).copied().unwrap_or(0);

        if amount == 0
            || entry.start >= candidate_finish
            || candidate_start >= entry.finish
        {
            continue;
        }

        let overlap_start = entry.start.max(candidate_start);
        let overlap_finish = entry.finish.min(candidate_finish);

        if overlap_start >= overlap_finish {
            continue;
        }

        events
            .entry(overlap_start)
            .or_default()
            .0
            .push((entry.step.clone(), amount));

        events
            .entry(overlap_finish)
            .or_default()
            .1
            .push(entry.step.clone());
    }

    let mut active: BTreeMap<StepId, u128> = BTreeMap::new();
    let mut active_total = 0_u128;
    let mut active_total_overflowed = false;
    let boundaries: Vec<ScheduleTime> = events.keys().copied().collect();

    for (index, time) in boundaries.iter().copied().enumerate() {
        let (starts, ends) = events
            .get(&time)
            .expect("each boundary has an event record");

        // Half-open intervals release resources before new reservations start
        // at the same time boundary.
        for ended in ends {
            if let Some(amount) = active.remove(ended) {
                if !active_total_overflowed {
                    active_total -= amount;
                }
            }
        }

        for (started, amount) in starts {
            active.insert(started.clone(), *amount);

            if !active_total_overflowed {
                match active_total.checked_add(*amount) {
                    Some(total) => active_total = total,
                    None => {
                        active_total_overflowed = true;
                        active_total = u128::MAX;
                    }
                }
            }
        }

        if active_total_overflowed {
            match sum_active_usage(&active) {
                Some(total) => {
                    active_total = total;
                    active_total_overflowed = false;
                }
                None => active_total = u128::MAX,
            }
        }

        let Some(next_time) = boundaries.get(index + 1).copied() else {
            break;
        };

        if next_time <= time {
            continue;
        }

        let combined = active_total.checked_add(requested);
        let exceeds_capacity = active_total_overflowed
            || combined.is_none()
            || combined.is_some_and(|usage| usage > capacity);

        if exceeds_capacity {
            // At least one existing reservation must be active: the candidate
            // demand itself was prevalidated against the capacity.
            let release = active
                .keys()
                .filter_map(|id| scheduled.get(id).map(|entry| entry.finish))
                .min();

            if let Some(release) = release {
                return Some(release);
            }
        }
    }

    None
}

/// Sum active resource reservations, returning `None` if the sum cannot be
/// represented. An unrepresentable sum necessarily exceeds any representable
/// capacity and is treated as a conflict by the scheduling algorithm.
fn sum_active_usage(active: &BTreeMap<StepId, u128>) -> Option<u128> {
    active
        .values()
        .try_fold(0_u128, |total, amount| total.checked_add(*amount))
}

/// Validate an extensible ASCII identifier without using a closed registry.
fn is_valid_identifier(value: &str) -> bool {
    !value.is_empty()
        && value.bytes().any(|byte| byte.is_ascii_alphanumeric())
        && value.bytes().all(|byte| {
            byte.is_ascii_alphanumeric()
                || matches!(byte, b'.' | b'_' | b'-' | b':' | b'/')
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::classical::execution::plan::{PlanStep, StepId};

    fn step_id(value: &str) -> StepId {
        StepId::new(value).expect("test step identifier should be valid")
    }

    fn resource_class(value: &str) -> ResourceClass {
        ResourceClass::new(value).expect("test resource class should be valid")
    }

    fn step(value: &str) -> PlanStep {
        PlanStep::new(step_id(value), "classical.test")
            .expect("test operation kind should be valid")
    }

    fn spec(value: &str, duration: u128) -> StepScheduleSpec {
        StepScheduleSpec::new(step_id(value), duration, [])
            .expect("test schedule spec should be valid")
    }

    fn use_of(class: &str, amount: u128) -> ResourceUse {
        ResourceUse::new(resource_class(class), amount)
    }

    fn capacity(class: &str, amount: u128) -> ResourceCapacity {
        ResourceCapacity::new(resource_class(class), amount)
    }

    fn linear_plan() -> ExecutionPlan {
        let mut plan =
            ExecutionPlan::new("linear").expect("test plan identifier should be valid");

        plan.add_step(step("a"));

        let mut b = step("b");
        b.depends_on(step_id("a"));
        plan.add_step(b);

        let mut c = step("c");
        c.depends_on(step_id("b"));
        plan.add_step(c);

        plan
    }

    #[test]
    fn schedules_a_dependency_chain() {
        let plan = linear_plan();

        let result = schedule_plan(
            &plan,
            [spec("a", 2), spec("b", 3), spec("c", 1)],
            [],
        )
        .expect("valid dependency chain should schedule");

        assert_eq!(result.step(&step_id("a")).unwrap().start(), 0);
        assert_eq!(result.step(&step_id("a")).unwrap().finish(), 2);
        assert_eq!(result.step(&step_id("b")).unwrap().start(), 2);
        assert_eq!(result.step(&step_id("b")).unwrap().finish(), 5);
        assert_eq!(result.step(&step_id("c")).unwrap().start(), 5);
        assert_eq!(result.makespan(), 6);
        result.validate_against(&plan).expect("schedule should validate");
    }

    #[test]
    fn independent_steps_can_overlap_without_shared_resources() {
        let mut plan =
            ExecutionPlan::new("parallel").expect("test plan identifier should be valid");

        plan.add_step(step("a"));
        plan.add_step(step("b"));

        let result = schedule_plan(
            &plan,
            [spec("a", 7), spec("b", 4)],
            [],
        )
        .expect("independent steps without resource constraints can overlap");

        assert_eq!(result.step(&step_id("a")).unwrap().start(), 0);
        assert_eq!(result.step(&step_id("b")).unwrap().start(), 0);
        assert_eq!(result.makespan(), 7);
    }

    #[test]
    fn shared_capacity_serializes_conflicting_steps() {
        let mut plan =
            ExecutionPlan::new("shared").expect("test plan identifier should be valid");

        plan.add_step(step("a"));
        plan.add_step(step("b"));

        let a = StepScheduleSpec::new(step_id("a"), 5, [use_of("worker", 1)])
            .expect("test spec should be valid");
        let b = StepScheduleSpec::new(step_id("b"), 3, [use_of("worker", 1)])
            .expect("test spec should be valid");

        let result = schedule_plan(&plan, [a, b], [capacity("worker", 1)])
            .expect("shared capacity should serialize reservations");

        assert_eq!(result.step(&step_id("a")).unwrap().start(), 0);
        assert_eq!(result.step(&step_id("b")).unwrap().start(), 5);
        assert_eq!(result.makespan(), 8);
    }

    #[test]
    fn capacity_allows_parallel_steps_when_aggregate_usage_fits() {
        let mut plan =
            ExecutionPlan::new("capacity-two").expect("test plan identifier should be valid");

        plan.add_step(step("a"));
        plan.add_step(step("b"));

        let a = StepScheduleSpec::new(step_id("a"), 5, [use_of("worker", 1)])
            .expect("test spec should be valid");
        let b = StepScheduleSpec::new(step_id("b"), 3, [use_of("worker", 1)])
            .expect("test spec should be valid");

        let result = schedule_plan(&plan, [a, b], [capacity("worker", 2)])
            .expect("capacity should allow overlap");

        assert_eq!(result.step(&step_id("a")).unwrap().start(), 0);
        assert_eq!(result.step(&step_id("b")).unwrap().start(), 0);
        assert_eq!(result.makespan(), 5);
    }

    #[test]
    fn rejects_missing_step_specification() {
        let error = schedule_plan(&linear_plan(), [spec("a", 1)], [])
            .expect_err("every step must have a specification");

        assert!(matches!(
            error,
            SchedulingError::MissingStepSpecification { .. }
        ));
    }

    #[test]
    fn rejects_duplicate_step_specification() {
        let error = schedule_plan(
            &linear_plan(),
            [spec("a", 1), spec("a", 2), spec("b", 1), spec("c", 1)],
            [],
        )
        .expect_err("duplicate specifications must not overwrite each other");

        assert!(matches!(
            error,
            SchedulingError::DuplicateStepSpecification { .. }
        ));
    }

    #[test]
    fn rejects_unknown_step_specification() {
        let error = schedule_plan(
            &linear_plan(),
            [
                spec("a", 1),
                spec("b", 1),
                spec("c", 1),
                spec("unknown", 1),
            ],
            [],
        )
        .expect_err("unknown step specifications must be rejected");

        assert!(matches!(
            error,
            SchedulingError::UnknownStepSpecification { .. }
        ));
    }

    #[test]
    fn rejects_missing_capacity_evidence() {
        let mut plan =
            ExecutionPlan::new("missing-capacity").expect("valid plan identifier");

        plan.add_step(step("a"));

        let a = StepScheduleSpec::new(step_id("a"), 1, [use_of("worker", 1)])
            .expect("test spec should be valid");

        let error = schedule_plan(&plan, [a], [])
            .expect_err("positive usage needs explicit capacity evidence");

        assert!(matches!(
            error,
            SchedulingError::MissingResourceCapacity { .. }
        ));
    }

    #[test]
    fn rejects_a_demand_larger_than_capacity() {
        let mut plan =
            ExecutionPlan::new("insufficient-capacity").expect("valid plan identifier");

        plan.add_step(step("a"));

        let a = StepScheduleSpec::new(step_id("a"), 1, [use_of("worker", 3)])
            .expect("test spec should be valid");

        let error = schedule_plan(&plan, [a], [capacity("worker", 2)])
            .expect_err("an individually infeasible demand must be rejected");

        assert!(matches!(
            error,
            SchedulingError::ResourceDemandExceedsCapacity {
                requested: 3,
                capacity: 2,
                ..
            }
        ));
    }

    #[test]
    fn rejects_duplicate_resource_usage_in_one_specification() {
        let error = StepScheduleSpec::new(
            step_id("a"),
            1,
            [use_of("worker", 1), use_of("worker", 2)],
        )
        .expect_err("resource usage must be aggregated before scheduling");

        assert!(matches!(error, SchedulingError::DuplicateStepResource { .. }));
    }

    #[test]
    fn rejects_duplicate_capacity_declarations() {
        let mut plan =
            ExecutionPlan::new("duplicate-capacity").expect("valid plan identifier");

        plan.add_step(step("a"));

        let a = StepScheduleSpec::new(step_id("a"), 1, [use_of("worker", 1)])
            .expect("test spec should be valid");

        let error = schedule_plan(
            &plan,
            [a],
            [capacity("worker", 1), capacity("worker", 2)],
        )
        .expect_err("capacity declarations must be unambiguous");

        assert!(matches!(
            error,
            SchedulingError::DuplicateResourceCapacity { .. }
        ));
    }

    #[test]
    fn permits_adjacent_half_open_reservations() {
        let mut plan =
            ExecutionPlan::new("adjacent").expect("valid plan identifier");

        plan.add_step(step("a"));

        let mut b = step("b");
        b.depends_on(step_id("a"));
        plan.add_step(b);

        let a = StepScheduleSpec::new(step_id("a"), 2, [use_of("worker", 1)])
            .expect("test spec should be valid");
        let b = StepScheduleSpec::new(step_id("b"), 2, [use_of("worker", 1)])
            .expect("test spec should be valid");

        let result = schedule_plan(&plan, [a, b], [capacity("worker", 1)])
            .expect("adjacent reservations should not overlap");

        assert_eq!(result.step(&step_id("a")).unwrap().finish(), 2);
        assert_eq!(result.step(&step_id("b")).unwrap().start(), 2);
    }

    #[test]
    fn zero_duration_steps_do_not_reserve_time() {
        let mut plan =
            ExecutionPlan::new("zero-duration").expect("valid plan identifier");

        plan.add_step(step("a"));
        plan.add_step(step("b"));

        let a = StepScheduleSpec::new(step_id("a"), 0, [use_of("worker", 1)])
            .expect("test spec should be valid");
        let b = StepScheduleSpec::new(step_id("b"), 4, [use_of("worker", 1)])
            .expect("test spec should be valid");

        let result = schedule_plan(&plan, [a, b], [capacity("worker", 1)])
            .expect("zero-duration operations should be representable");

        assert_eq!(result.step(&step_id("a")).unwrap().start(), 0);
        assert_eq!(result.step(&step_id("a")).unwrap().finish(), 0);
        assert_eq!(result.step(&step_id("b")).unwrap().start(), 0);
    }

    #[test]
    fn detects_time_overflow() {
        let mut plan =
            ExecutionPlan::new("overflow").expect("valid plan identifier");

        plan.add_step(step("a"));

        let error = schedule_plan(
            &plan,
            [spec("a", u128::MAX), spec("unused", 1)],
            [],
        )
        .expect_err("unknown specification must be rejected");

        assert!(matches!(
            error,
            SchedulingError::UnknownStepSpecification { .. }
        ));

        let result = schedule_plan(&plan, [spec("a", u128::MAX)], [])
            .expect("maximum representable finish time is valid");

        assert_eq!(result.makespan(), u128::MAX);

        let mut dependent =
            ExecutionPlan::new("overflow-dependent").expect("valid plan identifier");

        dependent.add_step(step("a"));

        let mut b = step("b");
        b.depends_on(step_id("a"));
        dependent.add_step(b);

        let error = schedule_plan(
            &dependent,
            [spec("a", u128::MAX), spec("b", 1)],
            [],
        )
        .expect_err("adding one tick after the maximum time must fail");

        assert!(matches!(error, SchedulingError::TimeOverflow { .. }));
    }

    #[test]
    fn empty_plan_produces_empty_schedule() {
        let plan =
            ExecutionPlan::new("empty").expect("valid plan identifier");

        let result = schedule_plan(&plan, [], [])
            .expect("empty plans need no specifications");

        assert!(result.is_empty());
        assert_eq!(result.makespan(), 0);
        result.validate_against(&plan).expect("empty schedule is valid");
    }

    #[test]
    fn schedule_is_deterministic_when_input_spec_order_changes() {
        let mut plan =
            ExecutionPlan::new("determinism").expect("valid plan identifier");

        plan.add_step(step("a"));
        plan.add_step(step("b"));
        plan.add_step(step("c"));

        let first = schedule_plan(
            &plan,
            [spec("a", 3), spec("b", 2), spec("c", 1)],
            [],
        )
        .expect("valid schedule");

        let second = schedule_plan(
            &plan,
            [spec("c", 1), spec("a", 3), spec("b", 2)],
            [],
        )
        .expect("input ordering should not change the result");

        assert_eq!(first, second);
    }

    #[test]
    fn accepts_extensible_namespaced_resource_classes() {
        assert_eq!(
            resource_class("future::accelerator/compute.v2").as_str(),
            "future::accelerator/compute.v2"
        );

        assert!(ResourceClass::new("").is_err());
        assert!(ResourceClass::new("invalid name").is_err());
    }

    #[test]
    fn handles_many_independent_steps_without_a_semantic_ceiling() {
        let mut plan =
            ExecutionPlan::new("large-plan").expect("valid plan identifier");

        let mut specs = Vec::new();

        for index in 0..4096 {
            let id = format!("step-{index}");
            plan.add_step(step(&id));
            specs.push(spec(&id, 1));
        }

        let result = schedule_plan(&plan, specs, [])
            .expect("step count should be bounded by available resources");

        assert_eq!(result.len(), 4096);
        assert_eq!(result.makespan(), 1);
        result.validate_against(&plan).expect("schedule should validate");
    }
}
