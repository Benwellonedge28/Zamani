
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Compiler — Abstract Workload Scheduling
//!
//! Path: src/compiler/planning/scheduling.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! ## Responsibility
//!
//! Construct deterministic, dependency-respecting abstract schedules from
//! a compiler workload and its validated logical partitioning report.
//!
//! This module owns:
//!
//! - Dependency-aware topological scheduling.
//! - Deterministic ready-operation ordering.
//! - Configurable abstract concurrency grouping.
//! - Explicit operational scheduling budgets.
//! - Schedule completeness and structural validation.
//! - Dependency-cycle and blocked-work reporting.
//!
//! It does not own:
//!
//! - Source grammar or scheduling-intent parsing.
//! - Semantic analysis or semantic requirement interpretation.
//! - ZUIR or domain-specific IR definitions.
//! - Capability discovery or resource feasibility.
//! - Logical partitioning.
//! - Physical placement or device selection.
//! - Network or quantum routing.
//! - Physical resource reservation.
//! - Backend code generation.
//! - Runtime scheduling or execution.
//!
//! ## Canonical authorities
//!
//! Operation identity:
//!     crate::zuir::ids::OperationId
//!
//! Workload and partitioning:
//!     crate::compiler::planning::partitioning
//!
//! Dependency representation:
//!     crate::distributed::planning::partition
//!
//! Resource feasibility:
//!     crate::compiler::planning::resource_planning
//!
//! Scheduling-intent syntax:
//!     grammar/execution/scheduling.g4
//!
//! Classical temporal scheduling:
//!     crate::classical::execution::scheduling
//!
//! ## Dependency semantics
//!
//! An edge A -> B means B depends on A.
//!
//! B must be scheduled in a strictly later schedule step than A.
//!
//! Operations in one step are independent with respect to the supplied
//! dependency graph. This does not prove that they can execute concurrently
//! on currently available physical resources.
//!
//! ## Determinism
//!
//! Ready operations are selected in ascending canonical OperationId order.
//! Adjacency is represented in ordered collections.
//!
//! Equivalent workload graphs, partition reports and options produce
//! equivalent schedule results.
//!
//! ## Scalability
//!
//! No universal maximum is imposed on operations, dependencies, partitions,
//! schedule steps, domains, targets, processors, accelerators or nodes.
//!
//! Optional budgets are caller-selected operational policies.
//!
//! Actual capacity is bounded by available memory, address space, processing
//! time and representable values. Literal infinity is not promised.
//!
//! ## Scheduling-intent boundary
//!
//! The grammar describes scheduling intent. Semantic analysis determines
//! its meaning and validity. An integration adapter may translate validated
//! intent into SchedulingOptions or another explicit planning policy.
//!
//! This module does not infer policy names, timing units, resource quantities,
//! latency, deadlines, objectives or physical concurrency from source text.
//!
//! ## Runtime boundary
//!
//! A ScheduleReport is an abstract ordering proposal. Runtime consumers must
//! revalidate current capabilities, resource availability, placement,
//! routing and execution constraints before dispatch.

use core::fmt;
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::num::NonZeroUsize;

use crate::compiler::planning::partitioning::{
    PartitioningDisposition,
    PartitioningReport,
    PartitionSegmentId,
    PartitioningWorkload,
};

use crate::distributed::planning::partition::Dependency;
use crate::zuir::ids::OperationId;

/// Stable architectural identity of this module.
pub const SCHEDULING_MODULE_NAME: &str =
    "zamani.compiler.planning.scheduling";

/// Public API version of this module.
pub const SCHEDULING_API_VERSION: u16 = 2;

/// Optional operational budgets for one scheduling invocation.
///
/// These limits protect an individual compilation request. They are not
/// universal language or hardware limits.
///
/// A budget violation returns an error. The scheduler never truncates
/// the workload and reports an incomplete schedule as complete.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SchedulingBudget {
    max_operations: Option<usize>,
    max_dependencies: Option<usize>,
    max_steps: Option<usize>,
}

impl SchedulingBudget {
    /// Creates a budget without additional count restrictions.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self {
            max_operations: None,
            max_dependencies: None,
            max_steps: None,
        }
    }

    /// Sets an optional maximum operation count.
    #[must_use]
    pub const fn with_max_operations(
        mut self,
        maximum: Option<usize>,
    ) -> Self {
        self.max_operations = maximum;
        self
    }

    /// Sets an optional maximum dependency count.
    #[must_use]
    pub const fn with_max_dependencies(
        mut self,
        maximum: Option<usize>,
    ) -> Self {
        self.max_dependencies = maximum;
        self
    }

    /// Sets an optional maximum schedule-step count.
    #[must_use]
    pub const fn with_max_steps(
        mut self,
        maximum: Option<usize>,
    ) -> Self {
        self.max_steps = maximum;
        self
    }

    /// Returns the configured operation budget.
    #[must_use]
    pub const fn max_operations(self) -> Option<usize> {
        self.max_operations
    }

    /// Returns the configured dependency budget.
    #[must_use]
    pub const fn max_dependencies(self) -> Option<usize> {
        self.max_dependencies
    }

    /// Returns the configured schedule-step budget.
    #[must_use]
    pub const fn max_steps(self) -> Option<usize> {
        self.max_steps
    }
}

impl Default for SchedulingBudget {
    fn default() -> Self {
        Self::unlimited()
    }
}

/// Configuration for one abstract scheduling invocation.
///
/// `max_parallelism` limits the number of operations grouped into one
/// abstract schedule step. It does not reserve physical workers.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SchedulingOptions {
    max_parallelism: Option<NonZeroUsize>,
    budget: SchedulingBudget,
}

impl SchedulingOptions {
    /// Creates scheduling options.
    #[must_use]
    pub const fn new(
        max_parallelism: Option<NonZeroUsize>,
        budget: SchedulingBudget,
    ) -> Self {
        Self {
            max_parallelism,
            budget,
        }
    }

    /// Creates options without additional parallelism or count limits.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self {
            max_parallelism: None,
            budget: SchedulingBudget::unlimited(),
        }
    }

    /// Sets the maximum number of operations per schedule step.
    #[must_use]
    pub const fn with_max_parallelism(
        mut self,
        maximum: Option<NonZeroUsize>,
    ) -> Self {
        self.max_parallelism = maximum;
        self
    }

    /// Sets the operational budget.
    #[must_use]
    pub const fn with_budget(
        mut self,
        budget: SchedulingBudget,
    ) -> Self {
        self.budget = budget;
        self
    }

    /// Returns the optional concurrency-group limit.
    #[must_use]
    pub const fn max_parallelism(self) -> Option<NonZeroUsize> {
        self.max_parallelism
    }

    /// Returns the operational budget.
    #[must_use]
    pub const fn budget(self) -> SchedulingBudget {
        self.budget
    }
}

impl Default for SchedulingOptions {
    fn default() -> Self {
        Self::unlimited()
    }
}

/// One operation included in an abstract execution schedule.
///
/// The operation identity comes from canonical ZUIR. The partition identity
/// is a logical planning identity, not a physical device or worker identity.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ScheduledOperation {
    operation: OperationId,
    partition: PartitionSegmentId,
}

impl ScheduledOperation {
    fn new(
        operation: OperationId,
        partition: PartitionSegmentId,
    ) -> Self {
        Self {
            operation,
            partition,
        }
    }

    /// Returns the canonical operation identity.
    #[must_use]
    pub const fn operation(&self) -> OperationId {
        self.operation
    }

    /// Returns the logical partition segment.
    #[must_use]
    pub const fn partition(&self) -> PartitionSegmentId {
        self.partition
    }
}

/// One dependency-independent group of operations.
///
/// Operations in this group may be considered for concurrent execution by
/// a downstream realization planner or runtime, subject to actual evidence.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ScheduleStep {
    index: usize,
    operations: Vec<ScheduledOperation>,
}

impl ScheduleStep {
    fn new(
        index: usize,
        operations: Vec<ScheduledOperation>,
    ) -> Self {
        Self {
            index,
            operations,
        }
    }

    /// Returns the zero-based schedule-step index.
    #[must_use]
    pub const fn index(&self) -> usize {
        self.index
    }

    /// Returns operations in deterministic order.
    #[must_use]
    pub fn operations(&self) -> &[ScheduledOperation] {
        &self.operations
    }

    /// Returns the number of operations in this step.
    #[must_use]
    pub fn len(&self) -> usize {
        self.operations.len()
    }

    /// Returns whether this step is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.operations.is_empty()
    }
}

/// Complete abstract scheduling result.
///
/// This report describes logical ordering only. It does not establish
/// physical availability, allocation, dispatch or execution.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ScheduleReport {
    steps: Vec<ScheduleStep>,
    operation_count: usize,
    dependency_count: usize,
    total_estimated_work: u128,
    maximum_observed_parallelism: usize,
}

impl ScheduleReport {
    fn new(
        steps: Vec<ScheduleStep>,
        operation_count: usize,
        dependency_count: usize,
        total_estimated_work: u128,
        maximum_observed_parallelism: usize,
    ) -> Self {
        Self {
            steps,
            operation_count,
            dependency_count,
            total_estimated_work,
            maximum_observed_parallelism,
        }
    }

    /// Returns schedule steps in abstract execution order.
    #[must_use]
    pub fn steps(&self) -> &[ScheduleStep] {
        &self.steps
    }

    /// Returns the number of scheduled operations.
    #[must_use]
    pub const fn operation_count(&self) -> usize {
        self.operation_count
    }

    /// Returns the number of input dependency records.
    ///
    /// Duplicate dependency records are included in this count even though
    /// duplicate precedence edges are collapsed during graph traversal.
    #[must_use]
    pub const fn dependency_count(&self) -> usize {
        self.dependency_count
    }

    /// Returns the checked sum of abstract operation work estimates.
    #[must_use]
    pub const fn total_estimated_work(&self) -> u128 {
        self.total_estimated_work
    }

    /// Returns the largest operation group in any schedule step.
    #[must_use]
    pub const fn maximum_observed_parallelism(&self) -> usize {
        self.maximum_observed_parallelism
    }

    /// Returns the number of schedule steps.
    #[must_use]
    pub fn step_count(&self) -> usize {
        self.steps.len()
    }

    /// Returns whether the schedule contains no operations.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.operation_count == 0
    }

    /// Returns whether the steps account for every declared operation.
    ///
    /// This is a structural completeness check, not proof of feasibility.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        let Some(count) = self.steps.iter().try_fold(
            0_usize,
            |total, step| total.checked_add(step.len()),
        ) else {
            return false;
        };

        count == self.operation_count
    }

    /// Validates this schedule against its workload and partitioning report.
    ///
    /// Validation checks:
    ///
    /// - Exact operation coverage.
    /// - Exactly one occurrence of every operation.
    /// - Correct logical partition membership.
    /// - Strict dependency ordering.
    /// - Step index consistency.
    /// - Deterministic operation ordering within each step.
    /// - Dependency and operation counts.
    /// - Work-estimate consistency.
    /// - Configured budgets and concurrency restrictions.
    ///
    /// This validates the supplied abstract schedule; it does not rebuild
    /// another schedule or query physical resources.
    ///
    /// # Errors
    ///
    /// Returns an error when the workload, partition report or schedule
    /// violates the required structural contract.
    pub fn validate_against(
        &self,
        workload: &PartitioningWorkload,
        partitioning: &PartitioningReport,
        options: SchedulingOptions,
    ) -> Result<(), SchedulingError> {
        let validated = validate_inputs(workload, partitioning)?;

        check_budget(
            "operations",
            validated.estimates.len(),
            options.budget().max_operations(),
        )?;

        check_budget(
            "dependencies",
            workload.dependencies().len(),
            options.budget().max_dependencies(),
        )?;

        check_budget(
            "schedule steps",
            self.steps.len(),
            options.budget().max_steps(),
        )?;

        if self.operation_count != validated.estimates.len()
            || self.dependency_count != workload.dependencies().len()
            || !self.is_complete()
        {
            return Err(SchedulingError::ScheduleValidationFailed);
        }

        let mut operation_step = BTreeMap::new();
        let mut scheduled_count = 0_usize;
        let mut maximum_parallelism = 0_usize;
        let mut total_work = 0_u128;

        for (expected_index, step) in self.steps.iter().enumerate() {
            if step.index() != expected_index || step.is_empty() {
                return Err(SchedulingError::ScheduleValidationFailed);
            }

            if let Some(limit) = options.max_parallelism() {
                if step.len() > limit.get() {
                    return Err(SchedulingError::ScheduleValidationFailed);
                }
            }

            if !step
                .operations()
                .windows(2)
                .all(|pair| pair[0].operation() < pair[1].operation())
            {
                return Err(SchedulingError::ScheduleValidationFailed);
            }

            maximum_parallelism = maximum_parallelism.max(step.len());

            for scheduled in step.operations() {
                let operation = scheduled.operation();

                let Some(expected_partition) =
                    validated.partitions.get(&operation)
                else {
                    return Err(
                        SchedulingError::MissingPartitionAssignment(operation),
                    );
                };

                if scheduled.partition() != *expected_partition {
                    return Err(SchedulingError::ScheduleValidationFailed);
                }

                if operation_step
                    .insert(operation, expected_index)
                    .is_some()
                {
                    return Err(SchedulingError::DuplicateOperation(operation));
                }

                let estimate = validated
                    .estimates
                    .get(&operation)
                    .copied()
                    .ok_or(SchedulingError::ScheduleValidationFailed)?;

                total_work = total_work
                    .checked_add(estimate)
                    .ok_or(SchedulingError::ArithmeticOverflow)?;

                scheduled_count = scheduled_count
                    .checked_add(1)
                    .ok_or(SchedulingError::ArithmeticOverflow)?;
            }
        }

        if scheduled_count != validated.estimates.len()
            || operation_step.len() != validated.estimates.len()
            || maximum_parallelism != self.maximum_observed_parallelism
            || total_work != self.total_estimated_work
        {
            return Err(SchedulingError::ScheduleValidationFailed);
        }

        for dependency in workload.dependencies() {
            let source = *dependency.source();
            let destination = *dependency.destination();

            let source_step = operation_step
                .get(&source)
                .copied()
                .ok_or(SchedulingError::ScheduleValidationFailed)?;

            let destination_step = operation_step
                .get(&destination)
                .copied()
                .ok_or(SchedulingError::ScheduleValidationFailed)?;

            if source_step >= destination_step {
                return Err(SchedulingError::ScheduleValidationFailed);
            }
        }

        Ok(())
    }
}

/// Errors produced by abstract scheduling.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum SchedulingError {
    /// The partitioning report is incomplete or unresolved.
    IncompletePartitioning,

    /// The partitioning report does not describe the supplied workload.
    PartitioningWorkloadMismatch {
        /// Number of operations in the supplied workload.
        workload_operations: usize,
        /// Number of operations recorded in the partition report.
        partitioned_operations: usize,
        /// Number of dependencies in the supplied workload.
        workload_dependencies: usize,
        /// Number of dependencies recorded in the partition report.
        partitioned_dependencies: usize,
    },

    /// The partition report contains an invalid or inconsistent structure.
    InvalidPartitioningReport(&'static str),

    /// A work item has no partition assignment.
    MissingPartitionAssignment(OperationId),

    /// An operation appears more than once in partition metadata.
    DuplicatePartitionAssignment(OperationId),

    /// The workload contains duplicate operation identities.
    DuplicateOperation(OperationId),

    /// A dependency references an unknown operation.
    UnknownDependencyEndpoint {
        /// Dependency source.
        source: OperationId,
        /// Dependency destination.
        destination: OperationId,
    },

    /// A dependency cycle prevents a complete topological schedule.
    ///
    /// The operation list contains every operation that could not be
    /// scheduled. It can include both cycle members and operations blocked
    /// downstream of a cycle.
    DependencyCycle {
        /// Unscheduled and dependency-blocked operations.
        operations: Vec<OperationId>,
    },

    /// An operational budget was exceeded.
    BudgetExceeded {
        /// Budget dimension.
        dimension: &'static str,
        /// Actual quantity.
        actual: usize,
        /// Configured maximum.
        maximum: usize,
    },

    /// Checked arithmetic overflowed.
    ArithmeticOverflow,

    /// The supplied schedule failed structural validation.
    ScheduleValidationFailed,
}

impl fmt::Display for SchedulingError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IncompletePartitioning => {
                formatter.write_str(
                    "scheduling requires complete, resolved partitioning evidence",
                )
            }

            Self::PartitioningWorkloadMismatch {
                workload_operations,
                partitioned_operations,
                workload_dependencies,
                partitioned_dependencies,
            } => {
                write!(
                    formatter,
                    "partition report does not match workload: \
                     operations {partitioned_operations}/{workload_operations}, \
                     dependencies {partitioned_dependencies}/{workload_dependencies}"
                )
            }

            Self::InvalidPartitioningReport(reason) => {
                write!(formatter, "invalid partitioning report: {reason}")
            }

            Self::MissingPartitionAssignment(operation) => {
                write!(
                    formatter,
                    "operation {operation} has no logical partition assignment"
                )
            }

            Self::DuplicatePartitionAssignment(operation) => {
                write!(
                    formatter,
                    "operation {operation} has multiple partition assignments"
                )
            }

            Self::DuplicateOperation(operation) => {
                write!(
                    formatter,
                    "workload contains duplicate operation {operation}"
                )
            }

            Self::UnknownDependencyEndpoint {
                source,
                destination,
            } => {
                write!(
                    formatter,
                    "dependency {source} -> {destination} references an unknown operation"
                )
            }

            Self::DependencyCycle { operations } => {
                write!(
                    formatter,
                    "dependency cycle or blocked dependency chain prevents \
                     scheduling {} operation(s)",
                    operations.len()
                )
            }

            Self::BudgetExceeded {
                dimension,
                actual,
                maximum,
            } => {
                write!(
                    formatter,
                    "scheduling budget exceeded for {dimension}: \
                     {actual} exceeds {maximum}"
                )
            }

            Self::ArithmeticOverflow => {
                formatter.write_str(
                    "scheduling arithmetic overflowed its supported representation",
                )
            }

            Self::ScheduleValidationFailed => {
                formatter.write_str(
                    "schedule failed structural validation against its inputs",
                )
            }
        }
    }
}

impl Error for SchedulingError {}

/// Stateless deterministic compiler scheduler.
///
/// This type does not own execution workers, physical resources,
/// device inventories or runtime state.
#[derive(Clone, Copy, Debug, Default)]
pub struct Scheduler;

impl Scheduler {
    /// Creates a stateless scheduler.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Returns the stable module identity.
    #[must_use]
    pub const fn module_name() -> &'static str {
        SCHEDULING_MODULE_NAME
    }

    /// Returns the public API version.
    #[must_use]
    pub const fn api_version() -> u16 {
        SCHEDULING_API_VERSION
    }

    /// Constructs a complete abstract execution schedule.
    ///
    /// The supplied partition report must describe the exact workload.
    /// Partial target inventory, unresolved assignments and incomplete
    /// dependency information are rejected.
    ///
    /// # Errors
    ///
    /// Returns an error for invalid input, incomplete partitioning,
    /// unknown dependencies, cycles, budget exhaustion or arithmetic
    /// overflow.
    pub fn schedule(
        &self,
        workload: &PartitioningWorkload,
        partitioning: &PartitioningReport,
        options: SchedulingOptions,
    ) -> Result<ScheduleReport, SchedulingError> {
        build_schedule(workload, partitioning, options)
    }
}

struct ValidatedInputs {
    estimates: BTreeMap<OperationId, u128>,
    partitions: BTreeMap<OperationId, PartitionSegmentId>,
}

fn validate_inputs(
    workload: &PartitioningWorkload,
    partitioning: &PartitioningReport,
) -> Result<ValidatedInputs, SchedulingError> {
    let operations = workload.work_items();
    let dependencies = workload.dependencies();

    if partitioning.work_item_count() != operations.len()
        || partitioning.dependency_count() != dependencies.len()
    {
        return Err(SchedulingError::PartitioningWorkloadMismatch {
            workload_operations: operations.len(),
            partitioned_operations: partitioning.work_item_count(),
            workload_dependencies: dependencies.len(),
            partitioned_dependencies: partitioning.dependency_count(),
        });
    }

    if partitioning.disposition() != PartitioningDisposition::Complete
        || !partitioning.inventory_is_complete()
        || !partitioning.unassigned().is_empty()
        || !partitioning.unresolved_dependencies().is_empty()
    {
        return Err(SchedulingError::IncompletePartitioning);
    }

    let mut estimates = BTreeMap::new();
    let mut total_work = 0_u128;

    for item in operations {
        let operation = item.operation();

        if estimates
            .insert(operation, item.estimated_work())
            .is_some()
        {
            return Err(SchedulingError::DuplicateOperation(operation));
        }

        total_work = total_work
            .checked_add(item.estimated_work())
            .ok_or(SchedulingError::ArithmeticOverflow)?;
    }

    let mut partitions = BTreeMap::new();

    for segment in partitioning.segments() {
        if segment.work_items().is_empty() {
            return Err(
                SchedulingError::InvalidPartitioningReport(
                    "a partition segment contains no operations",
                ),
            );
        }

        if !segment.has_candidate_target() {
            return Err(SchedulingError::IncompletePartitioning);
        }

        for operation in segment.work_items() {
            if !estimates.contains_key(operation) {
                return Err(
                    SchedulingError::MissingPartitionAssignment(*operation),
                );
            }

            if partitions
                .insert(*operation, segment.id())
                .is_some()
            {
                return Err(
                    SchedulingError::DuplicatePartitionAssignment(*operation),
                );
            }
        }
    }

    if partitions.len() != estimates.len() {
        for operation in estimates.keys() {
            if !partitions.contains_key(operation) {
                return Err(
                    SchedulingError::MissingPartitionAssignment(*operation),
                );
            }
        }

        return Err(
            SchedulingError::InvalidPartitioningReport(
                "partition assignment count does not match workload",
            ),
        );
    }

    // Validate every dependency endpoint before using it.
    for dependency in dependencies {
        let source = *dependency.source();
        let destination = *dependency.destination();

        if !estimates.contains_key(&source)
            || !estimates.contains_key(&destination)
        {
            return Err(SchedulingError::UnknownDependencyEndpoint {
                source,
                destination,
            });
        }
    }

    // The partition report is also consumed by downstream data movement
    // and routing. Verify that its cross-segment boundary evidence matches
    // the exact workload dependency records.
    let mut expected_boundaries = Vec::new();

    for dependency in dependencies {
        let source = *dependency.source();
        let destination = *dependency.destination();

        let source_segment = partitions
            .get(&source)
            .copied()
            .ok_or(SchedulingError::ScheduleValidationFailed)?;

        let destination_segment = partitions
            .get(&destination)
            .copied()
            .ok_or(SchedulingError::ScheduleValidationFailed)?;

        if source_segment != destination_segment {
            expected_boundaries.push((
                source,
                destination,
                source_segment,
                destination_segment,
                dependency.communication_cost(),
            ));
        }
    }

    let mut actual_boundaries = partitioning
        .boundaries()
        .iter()
        .map(|boundary| {
            (
                boundary.source(),
                boundary.destination(),
                boundary.source_segment(),
                boundary.destination_segment(),
                boundary.communication_cost(),
            )
        })
        .collect::<Vec<_>>();

    expected_boundaries.sort_unstable();
    actual_boundaries.sort_unstable();

    if expected_boundaries != actual_boundaries {
        return Err(
            SchedulingError::InvalidPartitioningReport(
                "cross-partition boundaries do not match workload dependencies",
            ),
        );
    }

    Ok(ValidatedInputs {
        estimates,
        partitions,
    })
}

fn check_budget(
    dimension: &'static str,
    actual: usize,
    maximum: Option<usize>,
) -> Result<(), SchedulingError> {
    if let Some(maximum) = maximum {
        if actual > maximum {
            return Err(SchedulingError::BudgetExceeded {
                dimension,
                actual,
                maximum,
            });
        }
    }

    Ok(())
}

fn build_schedule(
    workload: &PartitioningWorkload,
    partitioning: &PartitioningReport,
    options: SchedulingOptions,
) -> Result<ScheduleReport, SchedulingError> {
    let budget = options.budget();

    check_budget(
        "operations",
        workload.work_items().len(),
        budget.max_operations(),
    )?;

    check_budget(
        "dependencies",
        workload.dependencies().len(),
        budget.max_dependencies(),
    )?;

    let validated = validate_inputs(workload, partitioning)?;

    let (steps, maximum_parallelism) = build_steps(
        &validated.estimates,
        workload.dependencies(),
        options.max_parallelism(),
        budget.max_steps(),
    )?;

    let mut scheduled_steps = Vec::with_capacity(steps.len());

    for (index, operations) in steps.into_iter().enumerate() {
        let mut scheduled_operations = Vec::with_capacity(operations.len());

        for operation in operations {
            let partition = validated
                .partitions
                .get(&operation)
                .copied()
                .ok_or(
                    SchedulingError::MissingPartitionAssignment(operation),
                )?;

            scheduled_operations.push(ScheduledOperation::new(
                operation,
                partition,
            ));
        }

        scheduled_steps.push(ScheduleStep::new(
            index,
            scheduled_operations,
        ));
    }

    let total_estimated_work = validated.estimates.values().try_fold(
        0_u128,
        |total, estimate| total.checked_add(*estimate),
    ).ok_or(SchedulingError::ArithmeticOverflow)?;

    let report = ScheduleReport::new(
        scheduled_steps,
        validated.estimates.len(),
        workload.dependencies().len(),
        total_estimated_work,
        maximum_parallelism,
    );

    if !report.is_complete() {
        return Err(SchedulingError::ScheduleValidationFailed);
    }

    Ok(report)
}

/// Builds deterministic dependency-independent schedule steps.
///
/// Duplicate dependency records do not change precedence semantics, so
/// each distinct source/destination edge contributes one indegree. The
/// original dependency-record count remains available in ScheduleReport.
fn build_steps(
    estimates: &BTreeMap<OperationId, u128>,
    dependencies: &[Dependency<OperationId>],
    max_parallelism: Option<NonZeroUsize>,
    max_steps: Option<usize>,
) -> Result<(Vec<Vec<OperationId>>, usize), SchedulingError> {
    let mut indegree = estimates
        .keys()
        .map(|operation| (*operation, 0_usize))
        .collect::<BTreeMap<_, _>>();

    let mut adjacency = estimates
        .keys()
        .map(|operation| (*operation, BTreeSet::new()))
        .collect::<BTreeMap<_, _>>();

    for dependency in dependencies {
        let source = *dependency.source();
        let destination = *dependency.destination();

        if !indegree.contains_key(&source)
            || !indegree.contains_key(&destination)
        {
            return Err(SchedulingError::UnknownDependencyEndpoint {
                source,
                destination,
            });
        }

        let inserted = adjacency
            .get_mut(&source)
            .ok_or(SchedulingError::ScheduleValidationFailed)?
            .insert(destination);

        if inserted {
            let degree = indegree
                .get_mut(&destination)
                .ok_or(SchedulingError::ScheduleValidationFailed)?;

            *degree = degree
                .checked_add(1)
                .ok_or(SchedulingError::ArithmeticOverflow)?;
        }
    }

    let mut ready = indegree
        .iter()
        .filter_map(|(operation, degree)| {
            (*degree == 0).then_some(*operation)
        })
        .collect::<BTreeSet<_>>();

    let mut steps = Vec::new();
    let mut maximum_parallelism = 0_usize;

    while !ready.is_empty() {
        let mut selected = Vec::new();

        match max_parallelism {
            Some(limit) => {
                while selected.len() < limit.get() {
                    let Some(operation) = ready.pop_first() else {
                        break;
                    };

                    selected.push(operation);
                }
            }
            None => {
                selected.extend(std::mem::take(&mut ready));
            }
        }

        if selected.is_empty() {
            return Err(SchedulingError::ScheduleValidationFailed);
        }

        let next_step_count = steps
            .len()
            .checked_add(1)
            .ok_or(SchedulingError::ArithmeticOverflow)?;

        check_budget(
            "schedule steps",
            next_step_count,
            max_steps,
        )?;

        maximum_parallelism = maximum_parallelism.max(selected.len());

        // All operations in this step are considered complete before
        // their successors become eligible for a later step.
        for operation in &selected {
            let destinations = adjacency
                .get(operation)
                .ok_or(SchedulingError::ScheduleValidationFailed)?;

            for destination in destinations {
                let degree = indegree
                    .get_mut(destination)
                    .ok_or(SchedulingError::ScheduleValidationFailed)?;

                *degree = degree
                    .checked_sub(1)
                    .ok_or(SchedulingError::ScheduleValidationFailed)?;

                if *degree == 0 {
                    ready.insert(*destination);
                }
            }
        }

        steps.push(selected);
    }

    let scheduled_count = steps.iter().try_fold(
        0_usize,
        |total, step| total.checked_add(step.len()),
    ).ok_or(SchedulingError::ArithmeticOverflow)?;

    if scheduled_count != estimates.len() {
        let blocked = indegree
            .iter()
            .filter_map(|(operation, degree)| {
                (*degree > 0).then_some(*operation)
            })
            .collect();

        return Err(SchedulingError::DependencyCycle {
            operations: blocked,
        });
    }

    Ok((steps, maximum_parallelism))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn operation(value: u128) -> OperationId {
        OperationId::new(value)
            .expect("test operation identity must be nonzero")
    }

    fn dependency(
        source: u128,
        destination: u128,
    ) -> Dependency<OperationId> {
        Dependency::new(
            operation(source),
            operation(destination),
            0,
        )
    }

    fn estimates(values: &[u128]) -> BTreeMap<OperationId, u128> {
        values
            .iter()
            .map(|value| (operation(*value), 1))
            .collect()
    }

    fn schedule_graph(
        operations: &[u128],
        edges: &[(u128, u128)],
        parallelism: Option<usize>,
    ) -> Result<Vec<Vec<OperationId>>, SchedulingError> {
        let graph = estimates(operations);

        let dependencies = edges
            .iter()
            .map(|(source, destination)| {
                dependency(*source, *destination)
            })
            .collect::<Vec<_>>();

        let limit = parallelism.map(|value| {
            NonZeroUsize::new(value)
                .expect("test parallelism must be nonzero")
        });

        build_steps(&graph, &dependencies, limit, None)
            .map(|(steps, _)| steps)
    }

    #[test]
    fn module_identity_is_stable() {
        assert_eq!(
            Scheduler::module_name(),
            SCHEDULING_MODULE_NAME,
        );

        assert_eq!(
            Scheduler::api_version(),
            SCHEDULING_API_VERSION,
        );
    }

    #[test]
    fn empty_workload_produces_empty_schedule() {
        let result = schedule_graph(&[], &[], None)
            .expect("empty workload is valid");

        assert!(result.is_empty());
    }

    #[test]
    fn independent_operations_share_a_step() {
        let result = schedule_graph(
            &[4, 2, 1, 3],
            &[],
            None,
        ).expect("independent operations are schedulable");

        assert_eq!(
            result,
            vec![vec![
                operation(1),
                operation(2),
                operation(3),
                operation(4),
            ]],
        );
    }

    #[test]
    fn dependencies_are_respected() {
        let result = schedule_graph(
            &[1, 2, 3],
            &[(1, 2), (2, 3)],
            None,
        ).expect("acyclic graph is schedulable");

        assert_eq!(
            result,
            vec![
                vec![operation(1)],
                vec![operation(2)],
                vec![operation(3)],
            ],
        );
    }

    #[test]
    fn independent_branches_can_share_a_step() {
        let result = schedule_graph(
            &[1, 2, 3, 4],
            &[(1, 3), (1, 4)],
            None,
        ).expect("acyclic graph is schedulable");

        assert_eq!(
            result,
            vec![
                vec![operation(1), operation(2)],
                vec![operation(3), operation(4)],
            ],
        );
    }

    #[test]
    fn configured_parallelism_is_respected() {
        let result = schedule_graph(
            &[1, 2, 3, 4, 5],
            &[],
            Some(2),
        ).expect("graph is schedulable");

        assert_eq!(
            result,
            vec![
                vec![operation(1), operation(2)],
                vec![operation(3), operation(4)],
                vec![operation(5)],
            ],
        );
    }

    #[test]
    fn newly_ready_work_is_available_for_the_next_step() {
        let result = schedule_graph(
            &[1, 2, 3, 4, 5],
            &[(1, 5)],
            Some(2),
        ).expect("graph is schedulable");

        assert_eq!(
            result,
            vec![
                vec![operation(1), operation(2)],
                vec![operation(3), operation(4)],
                vec![operation(5)],
            ],
        );
    }

    #[test]
    fn cycles_are_rejected() {
        let result = schedule_graph(
            &[1, 2, 3],
            &[(1, 2), (2, 3), (3, 1)],
            None,
        );

        assert!(matches!(
            result,
            Err(SchedulingError::DependencyCycle { .. }),
        ));
    }

    #[test]
    fn self_dependency_is_rejected() {
        let result = schedule_graph(
            &[1],
            &[(1, 1)],
            None,
        );

        assert!(matches!(
            result,
            Err(SchedulingError::DependencyCycle { .. }),
        ));
    }

    #[test]
    fn duplicate_dependency_records_do_not_change_precedence() {
        let single = schedule_graph(
            &[1, 2],
            &[(1, 2)],
            None,
        ).expect("single edge");

        let duplicate = schedule_graph(
            &[1, 2],
            &[(1, 2), (1, 2), (1, 2)],
            None,
        ).expect("duplicate records");

        assert_eq!(single, duplicate);
    }

    #[test]
    fn unknown_dependency_endpoints_are_rejected() {
        let graph = estimates(&[1]);
        let dependencies = vec![dependency(1, 2)];

        let result = build_steps(
            &graph,
            &dependencies,
            None,
            None,
        );

        assert!(matches!(
            result,
            Err(SchedulingError::UnknownDependencyEndpoint { .. }),
        ));
    }

    #[test]
    fn step_budget_is_enforced() {
        let result = build_steps(
            &estimates(&[1, 2, 3]),
            &[dependency(1, 2), dependency(2, 3)],
            None,
            Some(2),
        );

        assert!(matches!(
            result,
            Err(SchedulingError::BudgetExceeded {
                dimension: "schedule steps",
                ..
            }),
        ));
    }

    #[test]
    fn no_universal_parallelism_limit_is_imposed() {
        assert_eq!(
            SchedulingOptions::unlimited().max_parallelism(),
            None,
        );
    }

    #[test]
    fn deterministic_order_is_independent_of_input_order() {
        let first = schedule_graph(
            &[3, 1, 2],
            &[(1, 3), (2, 3)],
            None,
        ).expect("first graph");

        let second = schedule_graph(
            &[2, 3, 1],
            &[(2, 3), (1, 3)],
            None,
        ).expect("second graph");

        assert_eq!(first, second);
    }

    #[test]
    fn bounded_parallelism_rejects_zero_at_the_type_boundary() {
        assert!(NonZeroUsize::new(0).is_none());
    }
}
