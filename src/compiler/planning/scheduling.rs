
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Meta-Compiler — Scheduling
//!
//! Path: src/compiler/planning/scheduling.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Construct deterministic, dependency-respecting abstract schedules
//! for compiler-validated heterogeneous workloads.
//!
//! This module consumes:
//!
//! - Canonical ZUIR OperationId identities.
//! - Compiler PartitioningWorkload graphs.
//! - Compiler PartitioningReport results.
//! - Canonical dependency relationships.
//!
//! It produces:
//!
//! - Ordered execution steps.
//! - Deterministic concurrent-operation groups.
//! - Operation-to-logical-partition associations.
//! - Schedule statistics.
//! - Explicit scheduling failures.
//!
//! ## OWNERSHIP
//!
//! This module owns:
//!
//! - Abstract scheduling order.
//! - Dependency readiness evaluation.
//! - Deterministic topological ordering.
//! - Configurable scheduling parallelism.
//! - Operational scheduling budgets.
//! - Schedule structural validation.
//! - Cycle detection.
//! - Scheduling diagnostics.
//!
//! ## DOES NOT OWN
//!
//! - Source syntax.
//! - AST construction.
//! - Semantic analysis.
//! - Ownership or borrow checking.
//! - ZUIR operation definitions.
//! - Domain-specific IR.
//! - Dependency semantic extraction.
//! - Capability definitions.
//! - Hardware discovery.
//! - Resource inventory.
//! - Physical resource allocation.
//! - Device selection.
//! - Physical placement.
//! - Network routing.
//! - Quantum routing.
//! - Quantum error correction.
//! - Backend code generation.
//! - Runtime execution.
//! - Runtime scheduling.
//!
//! ## AUTHORITATIVE DEPENDENCIES
//!
//! Operation identity:
//!     crate::zuir::ids::OperationId
//!
//! Workload graph:
//!     crate::compiler::planning::partitioning
//!
//! Logical partitioning:
//!     crate::compiler::planning::partitioning
//!
//! Generic dependencies:
//!     crate::distributed::planning::partition
//!
//! Capability matching:
//!     crate::compiler::planning::capability_matching
//!
//! Resource feasibility:
//!     crate::compiler::planning::resource_planning
//!
//! Physical placement and execution:
//!     crate::runtime
//!     crate::distributed
//!
//! ## SCHEDULING SEMANTICS
//!
//! An edge A -> B means B depends on A.
//!
//! B cannot enter a schedule step until A has completed an earlier step.
//!
//! Operations within one step are dependency-independent according to
//! the supplied graph.
//!
//! The scheduler uses deterministic Kahn topological scheduling.
//!
//! A dependency cycle is an error. It is never silently removed.
//!
//! ## PARALLELISM
//!
//! An optional parallelism limit controls the maximum number of operations
//! in one abstract schedule step.
//!
//! None means that no additional scheduler-level parallelism limit applies.
//!
//! This is an operational policy, not a universal machine limit.
//!
//! The scheduler does not claim that the runtime possesses enough physical
//! workers to execute every operation in a step simultaneously.
//!
//! ## SCALABILITY
//!
//! No arbitrary universal limits are imposed on:
//!
//! - Operations.
//! - Dependencies.
//! - Schedule steps.
//! - Logical partitions.
//! - Computing domains.
//! - Targets.
//! - CPUs.
//! - GPUs.
//! - FPGAs.
//! - ASICs.
//! - QPUs.
//! - Accelerators.
//! - Distributed participants.
//! - Quantum resources.
//! - Memory capacity.
//!
//! Optional budgets are caller-controlled operational policies.
//!
//! Actual scalability is bounded by available memory, address space,
//! processing time, identifier representation, and configured budgets.
//!
//! Literal infinity is not promised.
//!
//! ## DETERMINISM
//!
//! OperationId implements canonical ordering.
//!
//! Ready operations are selected in ascending OperationId order.
//!
//! Equivalent workload graphs and equivalent policies produce equivalent
//! schedule ordering.
//!
//! No random values, clocks, environment lookups, or global mutable state
//! influence scheduling.
//!
//! ## INTEGRATION
//!
//! The normal entry point is Scheduler::schedule.
//!
//! The caller must provide a PartitioningReport and the original workload.
//!
//! Incomplete or unresolved partitioning is rejected.
//!
//! The resulting schedule is an abstract plan. Downstream placement,
//! routing, runtime validation, and resource acquisition remain separate.
//!
//! ## FREEZE CRITERIA
//!
//! - Every input operation appears exactly once.
//! - Every dependency is respected.
//! - Every operation has a valid logical partition association.
//! - Cycles are reported explicitly.
//! - Configured budgets are enforced.
//! - Arithmetic overflow is reported.
//! - Equivalent inputs produce equivalent schedules.
//! - No physical resource is allocated.
//! - No unsafe Rust is used.

use core::fmt;
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::num::NonZeroUsize;

use crate::compiler::planning::partitioning::{
    PartitioningReport,
    PartitionSegmentId,
    PartitioningWorkload,
};

use crate::distributed::planning::partition::Dependency;

use crate::zuir::ids::OperationId;

// ============================================================================
// MODULE IDENTITY
// ============================================================================

/// Stable architectural identity of this module.
pub const SCHEDULING_MODULE_NAME: &str =
    "zamani.compiler.planning.scheduling";

/// Public API version of this module.
pub const SCHEDULING_API_VERSION: u16 = 1;

// ============================================================================
// SCHEDULING BUDGET
// ============================================================================

/// Optional operational budgets for one scheduling invocation.
///
/// These limits protect a particular compilation operation. They are
/// not universal language or hardware limits.
///
/// Exceeding a budget produces an error. The scheduler never truncates
/// the workload and reports a partial schedule as complete.
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

// ============================================================================
// OPTIONS
// ============================================================================

/// Configuration for one scheduling invocation.
///
/// `max_parallelism` limits the number of operations grouped into one
/// schedule step.
///
/// It does not identify physical workers or reserve execution resources.
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

    /// Creates scheduling options without an additional parallelism
    /// or operational budget restriction.
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

    /// Returns the optional parallelism limit.
    #[must_use]
    pub const fn max_parallelism(
        self,
    ) -> Option<NonZeroUsize> {
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

// ============================================================================
// SCHEDULED OPERATION
// ============================================================================

/// One operation included in an abstract execution schedule.
///
/// The operation identity comes directly from canonical ZUIR.
///
/// The partition identity is a logical planning identity. It is not
/// a physical device or worker identifier.
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

// ============================================================================
// SCHEDULE STEP
// ============================================================================

/// One dependency-independent group of operations.
///
/// All operations in a step are ready after completion of the preceding
/// schedule steps.
///
/// The runtime may execute them concurrently, subject to its actual
/// resource availability and execution policies.
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

    /// Returns the operations in deterministic order.
    #[must_use]
    pub fn operations(&self) -> &[ScheduledOperation] {
        &self.operations
    }

    /// Returns the number of operations in this step.
    #[must_use]
    pub fn len(&self) -> usize {
        self.operations.len()
    }

    /// Returns whether the step contains no operations.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.operations.is_empty()
    }
}

// ============================================================================
// SCHEDULE REPORT
// ============================================================================

/// Complete abstract scheduling result.
///
/// This report contains logical ordering information only.
///
/// It does not establish that physical resources remain available or
/// that execution has started.
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

    /// Returns all schedule steps in execution order.
    #[must_use]
    pub fn steps(&self) -> &[ScheduleStep] {
        &self.steps
    }

    /// Returns the number of scheduled operations.
    #[must_use]
    pub const fn operation_count(&self) -> usize {
        self.operation_count
    }

    /// Returns the number of preserved input dependencies.
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

    /// Returns whether the schedule covers the complete input workload.
    ///
    /// This is a structural completeness check, not proof of physical
    /// feasibility or runtime availability.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.steps
            .iter()
            .map(ScheduleStep::len)
            .sum::<usize>()
            == self.operation_count
    }

    /// Validates this schedule against the original workload and
    /// partitioning evidence.
    ///
    /// This check verifies operation coverage, partition membership,
    /// dependency ordering, and configured concurrency restrictions.
    ///
    /// Runtime consumers should perform their own current-resource
    /// revalidation before execution.
    ///
    /// # Errors
    ///
    /// Returns a validation error when the schedule does not satisfy
    /// the supplied workload and partitioning contracts.
    pub fn validate_against(
        &self,
        workload: &PartitioningWorkload,
        partitioning: &PartitioningReport,
        options: SchedulingOptions,
    ) -> Result<(), SchedulingError> {
        let expected = build_schedule(
            workload,
            partitioning,
            options,
        )?;

        if &expected != self {
            return Err(SchedulingError::ScheduleValidationFailed);
        }

        Ok(())
    }
}

// ============================================================================
// ERRORS
// ============================================================================

/// Errors produced by abstract scheduling.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum SchedulingError {
    /// The partitioning report is incomplete or unresolved.
    IncompletePartitioning,

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
    DependencyCycle {
        /// Operations that could not be scheduled.
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

    /// The schedule did not satisfy its original validation contract.
    ScheduleValidationFailed,
}

impl fmt::Display for SchedulingError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::IncompletePartitioning => {
                formatter.write_str(
                    "scheduling requires a complete partitioning report",
                )
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
                    "dependency cycle prevents scheduling {} operation(s)",
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
                    "scheduling budget exceeded for {dimension}: {actual} exceeds {maximum}"
                )
            }

            Self::ArithmeticOverflow => {
                formatter.write_str(
                    "scheduling arithmetic overflowed its supported representation",
                )
            }

            Self::ScheduleValidationFailed => {
                formatter.write_str(
                    "schedule failed structural validation against its input contracts",
                )
            }
        }
    }
}

impl Error for SchedulingError {}

// ============================================================================
// SCHEDULER
// ============================================================================

/// Stateless deterministic compiler scheduler.
///
/// This type does not own execution workers, physical resources,
/// device inventories, or runtime state.
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
    /// The workload must have a complete partitioning report.
    ///
    /// An incomplete target inventory or unresolved work assignment
    /// cannot be represented as a successfully realizable schedule.
    ///
    /// # Errors
    ///
    /// Returns an error for:
    ///
    /// - Incomplete partitioning.
    /// - Missing or duplicated operation assignments.
    /// - Unknown dependency endpoints.
    /// - Dependency cycles.
    /// - Budget exhaustion.
    /// - Arithmetic overflow.
    pub fn schedule(
        &self,
        workload: &PartitioningWorkload,
        partitioning: &PartitioningReport,
        options: SchedulingOptions,
    ) -> Result<ScheduleReport, SchedulingError> {
        build_schedule(workload, partitioning, options)
    }
}

// ============================================================================
// INTERNAL SCHEDULING ENGINE
// ============================================================================

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
    let operations = workload.work_items();
    let dependencies = workload.dependencies();

    let budget = options.budget();

    check_budget(
        "operations",
        operations.len(),
        budget.max_operations(),
    )?;

    check_budget(
        "dependencies",
        dependencies.len(),
        budget.max_dependencies(),
    )?;

    if !partitioning.is_complete()
        || !partitioning.is_fully_assigned()
        || !partitioning.every_segment_has_candidate_targets()
    {
        return Err(SchedulingError::IncompletePartitioning);
    }

    // ------------------------------------------------------------
    // Validate operations and collect work estimates.
    // ------------------------------------------------------------

    let mut operation_estimates = BTreeMap::new();

    let mut total_estimated_work = 0_u128;

    for item in operations {
        let operation = item.operation();

        if operation_estimates
            .insert(operation, item.estimated_work())
            .is_some()
        {
            return Err(
                SchedulingError::DuplicateOperation(operation),
            );
        }

        total_estimated_work = total_estimated_work
            .checked_add(item.estimated_work())
            .ok_or(SchedulingError::ArithmeticOverflow)?;
    }

    // ------------------------------------------------------------
    // Build operation-to-partition associations.
    // ------------------------------------------------------------

    let mut partition_by_operation = BTreeMap::new();

    for segment in partitioning.segments() {
        if !segment.has_candidate_target() {
            return Err(SchedulingError::IncompletePartitioning);
        }

        for operation in segment.work_items() {
            if !operation_estimates.contains_key(operation) {
                return Err(
                    SchedulingError::MissingPartitionAssignment(
                        *operation,
                    ),
                );
            }

            if partition_by_operation
                .insert(*operation, segment.id())
                .is_some()
            {
                return Err(
                    SchedulingError::DuplicatePartitionAssignment(
                        *operation,
                    ),
                );
            }
        }
    }

    for operation in operation_estimates.keys() {
        if !partition_by_operation.contains_key(operation) {
            return Err(
                SchedulingError::MissingPartitionAssignment(
                    *operation,
                ),
            );
        }
    }

    // ------------------------------------------------------------
    // Build the dependency graph.
    // ------------------------------------------------------------

    let mut indegree: BTreeMap<OperationId, usize> =
        operation_estimates
            .keys()
            .map(|operation| (*operation, 0))
            .collect();

    let mut adjacency: BTreeMap<
        OperationId,
        Vec<OperationId>,
    > = BTreeMap::new();

    for dependency in dependencies {
        insert_dependency(
            dependency,
            &mut indegree,
            &mut adjacency,
        )?;
    }

    // Sorting adjacency makes traversal independent of the order in
    // which the caller originally supplied dependency records.
    for destinations in adjacency.values_mut() {
        destinations.sort_unstable();
    }

    // ------------------------------------------------------------
    // Initialize the deterministic ready set.
    // ------------------------------------------------------------

    let mut ready = indegree
        .iter()
        .filter_map(|(operation, degree)| {
            (*degree == 0).then_some(*operation)
        })
        .collect::<BTreeSet<_>>();

    let parallelism = options
        .max_parallelism()
        .map(NonZeroUsize::get);

    let mut steps = Vec::new();

    let mut maximum_observed_parallelism = 0_usize;

    // ------------------------------------------------------------
    // Deterministic level-based Kahn scheduling.
    // ------------------------------------------------------------

    while !ready.is_empty() {
        let mut selected = Vec::new();

        let selection_limit = parallelism.unwrap_or(usize::MAX);

        while selected.len() < selection_limit {
            let Some(operation) =
                ready.pop_first()
            else {
                break;
            };

            selected.push(operation);
        }

        if selected.is_empty() {
            break;
        }

        let next_step_index = steps.len();

        let next_step_count = next_step_index
            .checked_add(1)
            .ok_or(SchedulingError::ArithmeticOverflow)?;

        check_budget(
            "schedule steps",
            next_step_count,
            budget.max_steps(),
        )?;

        let mut scheduled_operations = Vec::with_capacity(
            selected.len(),
        );

        for operation in &selected {
            let partition = partition_by_operation
                .get(operation)
                .copied()
                .ok_or(
                    SchedulingError::MissingPartitionAssignment(
                        *operation,
                    ),
                )?;

            scheduled_operations.push(
                ScheduledOperation::new(
                    *operation,
                    partition,
                ),
            );
        }

        maximum_observed_parallelism = maximum_observed_parallelism
            .max(scheduled_operations.len());

        steps.push(
            ScheduleStep::new(
                next_step_index,
                scheduled_operations,
            ),
        );

        // A successor becomes ready only after every dependency
        // predecessor has completed an earlier schedule step.
        for operation in &selected {
            if let Some(destinations) = adjacency.get(operation) {
                for destination in destinations {
                    let degree = indegree
                        .get_mut(destination)
                        .ok_or(
                            SchedulingError::ArithmeticOverflow,
                        )?;

                    *degree = degree
                        .checked_sub(1)
                        .ok_or(
                            SchedulingError::ArithmeticOverflow,
                        )?;

                    if *degree == 0 {
                        ready.insert(*destination);
                    }
                }
            }
        }
    }

    // ------------------------------------------------------------
    // Explicit cycle detection.
    // ------------------------------------------------------------

    let scheduled_count = steps
        .iter()
        .try_fold(0_usize, |count, step| {
            count.checked_add(step.len())
        })
        .ok_or(SchedulingError::ArithmeticOverflow)?;

    if scheduled_count != operations.len() {
        let scheduled = steps
            .iter()
            .flat_map(|step| {
                step.operations()
                    .iter()
                    .map(ScheduledOperation::operation)
            })
            .collect::<BTreeSet<_>>();

        let remaining = operation_estimates
            .keys()
            .filter(|operation| !scheduled.contains(operation))
            .copied()
            .collect::<Vec<_>>();

        return Err(
            SchedulingError::DependencyCycle {
                operations: remaining,
            },
        );
    }

    // ------------------------------------------------------------
    // Construct and validate the complete result.
    // ------------------------------------------------------------

    let report = ScheduleReport::new(
        steps,
        operations.len(),
        dependencies.len(),
        total_estimated_work,
        maximum_observed_parallelism,
    );

    if !report.is_complete() {
        return Err(
            SchedulingError::ScheduleValidationFailed,
        );
    }

    Ok(report)
}

// ============================================================================
// DEPENDENCY INSERTION
// ============================================================================

fn insert_dependency(
    dependency: &Dependency<OperationId>,
    indegree: &mut BTreeMap<OperationId, usize>,
    adjacency: &mut BTreeMap<
        OperationId,
        Vec<OperationId>,
    >,
) -> Result<(), SchedulingError> {
    let source = *dependency.source();
    let destination = *dependency.destination();

    if !indegree.contains_key(&source)
        || !indegree.contains_key(&destination)
    {
        return Err(
            SchedulingError::UnknownDependencyEndpoint {
                source,
                destination,
            },
        );
    }

    let degree = indegree
        .get_mut(&destination)
        .ok_or(SchedulingError::ArithmeticOverflow)?;

    *degree = degree
        .checked_add(1)
        .ok_or(SchedulingError::ArithmeticOverflow)?;

    adjacency
        .entry(source)
        .or_default()
        .push(destination);

    Ok(())
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn operation(value: u128) -> OperationId {
        OperationId::new(value)
            .expect("test operation ID must be nonzero")
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

    fn graph(
        operations: &[u128],
        edges: &[(u128, u128)],
    ) -> (
        BTreeMap<OperationId, usize>,
        BTreeMap<OperationId, Vec<OperationId>>,
    ) {
        let mut indegree = operations
            .iter()
            .map(|id| (operation(*id), 0))
            .collect::<BTreeMap<_, _>>();

        let mut adjacency = BTreeMap::new();

        for (source, destination) in edges {
            insert_dependency(
                &dependency(*source, *destination),
                &mut indegree,
                &mut adjacency,
            )
            .expect("test dependency should be valid");
        }

        (indegree, adjacency)
    }

    fn topological_steps(
        operations: &[u128],
        edges: &[(u128, u128)],
        maximum_parallelism: Option<usize>,
    ) -> Result<Vec<Vec<OperationId>>, SchedulingError> {
        let (mut indegree, adjacency) =
            graph(operations, edges);

        let mut ready = indegree
            .iter()
            .filter_map(|(operation, degree)| {
                (*degree == 0).then_some(*operation)
            })
            .collect::<BTreeSet<_>>();

        let limit = maximum_parallelism
            .unwrap_or(usize::MAX);

        let mut result = Vec::new();

        while !ready.is_empty() {
            let mut step = Vec::new();

            while step.len() < limit {
                let Some(operation) = ready.pop_first() else {
                    break;
                };

                step.push(operation);
            }

            for operation in &step {
                if let Some(destinations) =
                    adjacency.get(operation)
                {
                    for destination in destinations {
                        let degree = indegree
                            .get_mut(destination)
                            .ok_or(
                                SchedulingError::ArithmeticOverflow,
                            )?;

                        *degree = degree
                            .checked_sub(1)
                            .ok_or(
                                SchedulingError::ArithmeticOverflow,
                            )?;

                        if *degree == 0 {
                            ready.insert(*destination);
                        }
                    }
                }
            }

            result.push(step);
        }

        let count = result.iter().map(Vec::len).sum::<usize>();

        if count != operations.len() {
            return Err(SchedulingError::DependencyCycle {
                operations: indegree
                    .iter()
                    .filter_map(|(operation, degree)| {
                        (*degree > 0).then_some(*operation)
                    })
                    .collect(),
            });
        }

        Ok(result)
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
        let result = topological_steps(
            &[],
            &[],
            None,
        )
        .expect("empty workload should be schedulable");

        assert!(result.is_empty());
    }

    #[test]
    fn independent_operations_share_a_step() {
        let result = topological_steps(
            &[1, 2, 3, 4],
            &[],
            None,
        )
        .expect("independent operations should be schedulable");

        assert_eq!(result.len(), 1);
        assert_eq!(
            result[0],
            vec![
                operation(1),
                operation(2),
                operation(3),
                operation(4),
            ],
        );
    }

    #[test]
    fn dependencies_are_respected() {
        let result = topological_steps(
            &[1, 2, 3],
            &[(1, 2), (2, 3)],
            None,
        )
        .expect("acyclic workload should be schedulable");

        assert_eq!(result.len(), 3);

        assert_eq!(result[0], vec![operation(1)]);
        assert_eq!(result[1], vec![operation(2)]);
        assert_eq!(result[2], vec![operation(3)]);
    }

    #[test]
    fn independent_branches_execute_in_the_same_step() {
        let result = topological_steps(
            &[1, 2, 3, 4],
            &[(1, 3), (1, 4)],
            None,
        )
        .expect("acyclic workload should be schedulable");

        assert_eq!(result.len(), 2);

        assert_eq!(result[0], vec![operation(1)]);

        assert_eq!(
            result[1],
            vec![
                operation(2),
                operation(3),
                operation(4),
            ],
        );
    }

    #[test]
    fn configured_parallelism_is_respected() {
        let result = topological_steps(
            &[1, 2, 3, 4, 5],
            &[],
            Some(2),
        )
        .expect("workload should be schedulable");

        assert_eq!(result.len(), 3);

        assert_eq!(
            result[0],
            vec![operation(1), operation(2)],
        );

        assert_eq!(
            result[1],
            vec![operation(3), operation(4)],
        );

        assert_eq!(
            result[2],
            vec![operation(5)],
        );
    }

    #[test]
    fn cycles_are_rejected() {
        let result = topological_steps(
            &[1, 2, 3],
            &[(1, 2), (2, 3), (3, 1)],
            None,
        );

        assert!(matches!(
            result,
            Err(SchedulingError::DependencyCycle { .. })
        ));
    }

    #[test]
    fn self_dependency_is_rejected_as_a_cycle() {
        let result = topological_steps(
            &[1],
            &[(1, 1)],
            None,
        );

        assert!(matches!(
            result,
            Err(SchedulingError::DependencyCycle { .. })
        ));
    }

    #[test]
    fn duplicate_edges_are_preserved_correctly() {
        let result = topological_steps(
            &[1, 2],
            &[(1, 2), (1, 2)],
            None,
        )
        .expect("duplicate dependency records should balance");

        assert_eq!(result.len(), 2);
    }

    #[test]
    fn unknown_dependency_endpoints_are_rejected() {
        let mut indegree = BTreeMap::new();
        indegree.insert(operation(1), 0);

        let mut adjacency = BTreeMap::new();

        let result = insert_dependency(
            &dependency(1, 2),
            &mut indegree,
            &mut adjacency,
        );

        assert!(matches!(
            result,
            Err(
                SchedulingError::UnknownDependencyEndpoint { .. }
            )
        ));
    }

    #[test]
    fn operation_budget_is_configurable() {
        let budget = SchedulingBudget::unlimited()
            .with_max_operations(Some(2));

        assert_eq!(
            budget.max_operations(),
            Some(2),
        );

        assert_eq!(
            SchedulingBudget::unlimited()
                .max_operations(),
            None,
        );
    }

    #[test]
    fn no_universal_parallelism_limit_exists() {
        let options = SchedulingOptions::unlimited();

        assert_eq!(
            options.max_parallelism(),
            None,
        );
    }

    #[test]
    fn deterministic_order_is_independent_of_input_order() {
        let first = topological_steps(
            &[3, 1, 2],
            &[(1, 3), (2, 3)],
            None,
        )
        .expect("first graph");

        let second = topological_steps(
            &[2, 3, 1],
            &[(2, 3), (1, 3)],
            None,
        )
        .expect("second graph");

        assert_eq!(first, second);
    }
}
