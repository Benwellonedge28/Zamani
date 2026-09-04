//! Zamani Quantum Scheduling — Resource-Constrained Planner
//!
//! Path:
//!     src/quantum/scheduling/planners/resource_constrained.rs
//!
//! # Purpose
//!
//! This module implements resource-constrained project scheduling (RCPSP)
//! semantics for Zamani quantum programs.
//!
//! The planner answers:
//!
//! > Given operations with precedence constraints, temporal constraints,
//! > arbitrary resource requirements, and target-provided capacities,
//! > when can every operation execute without violating any constraint?
//!
//! This is a scheduling algorithm. It does not own quantum semantics,
//! routing, hardware discovery, execution, QEC decoding, or calibration.
//!
//! # Architectural boundary
//!
//! ```text
//! Zamani source
//!      │
//!      ▼
//! quantum::frontend
//!      │
//!      ▼
//! quantum::ir
//!      │
//!      ▼
//! optimization
//!      │
//!      ▼
//! routing
//!      │
//!      ▼
//! scheduling::adapters::ir
//!      │
//!      ▼
//! scheduling::ir
//!      │
//!      ├── operations
//!      ├── dependencies
//!      └── critical-path analysis
//!      │
//!      ▼
//! scheduling::context
//!      │
//!      ├── target timing
//!      ├── resource capacities
//!      ├── availability
//!      └── constraints
//!      │
//!      ▼
//! planners::resource_constrained       ◄── this module
//!      │
//!      ▼
//! scheduling result
//!      │
//!      ▼
//! verification
//!      │
//!      ▼
//! hardware / runtime
//! ```
//!
//! # Critical architectural distinction
//!
//! Routing answers:
//!
//!     WHERE does an operation execute?
//!
//! Resource-constrained scheduling answers:
//!
//!     WHEN can it execute?
//!
//! This module must therefore consume routing output rather than perform
//! logical-to-physical mapping itself.
//!
//! # Canonical identity rule
//!
//! This module never creates scheduler-local qubit identities.
//!
//! Logical and physical qubits remain owned by:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! Operation/resource identities remain owned by their canonical boundaries.
//!
//! If a resource requirement represents a qubit, the adapter must use the
//! canonical `quantum::ir::qubit` identity.
//!
//! # Resource model
//!
//! Quantum operations can consume arbitrary resources:
//!
//! - logical qubits;
//! - physical qubits;
//! - control channels;
//! - measurement channels;
//! - resonators;
//! - couplers;
//! - lasers;
//! - classical processors;
//! - feedback channels;
//! - communication links;
//! - QEC ancillas;
//! - network resources;
//! - target-defined resources.
//!
//! A resource has a target-supplied capacity.
//!
//! Capacity is never encoded as a compile-time constant.
//!
//! Capacity `1` naturally represents an exclusive resource.
//!
//! Capacity greater than `1` represents a shared resource.
//!
//! # Why RCPSP
//!
//! Pure dependency scheduling is insufficient for real quantum hardware.
//!
//! Example:
//!
//! ```text
//! A ──────┐
//!         ├── C
//! B ──────┘
//!
//! A and B have no dependency edge.
//!
//! However:
//!
//! A ──► control channel X
//! B ──► control channel X
//!
//! channel X capacity = 1
//! ```
//!
//! A and B therefore cannot overlap even though the dependency DAG permits
//! parallel execution.
//!
//! Resource-constrained scheduling resolves this distinction.
//!
//! # Algorithmic guarantee
//!
//! This implementation deliberately does NOT claim global optimality for all
//! resource-constrained scheduling problems.
//!
//! General RCPSP optimization can be computationally difficult.
//!
//! The implementation instead provides:
//!
//! - complete precedence preservation;
//! - resource-capacity preservation;
//! - deterministic scheduling;
//! - checked time arithmetic;
//! - arbitrary operation arity;
//! - arbitrary resource count;
//! - arbitrary resource capacity;
//! - temporal-window support;
//! - event-driven resource reasoning;
//! - no fixed machine dimensions;
//! - deterministic tie-breaking.
//!
//! # Scalability principle
//!
//! There are no scheduler-defined constants for:
//!
//! - qubit count;
//! - operation count;
//! - resource count;
//! - channel count;
//! - topology size;
//! - schedule depth;
//! - parallelism;
//! - QEC distance.
//!
//! The practical limit is determined by:
//!
//! - host memory;
//! - host address space;
//! - compilation time;
//! - explicit deployment limits;
//! - target resources.
//!
//! "Infinity" therefore means that this module introduces no artificial
//! finite machine-size ceiling.
//!
//! # Memory model
//!
//! The implementation must never construct:
//!
//! ```text
//! qubits × time
//! resources × maximum_time
//!
//! ```
//!
//! Instead it stores only:
//!
//! - operation state;
//! - dependency state;
//! - scheduled intervals;
//! - resource intervals;
//! - event boundaries.
//!
//! # Determinism
//!
//! Candidate ordering is deterministic.
//!
//! The ordering is:
//!
//! 1. smaller scheduling slack;
//! 2. higher priority;
//! 3. longer downstream critical-path contribution;
//! 4. smaller canonical operation identity.
//!
//! Hash-map iteration order is never used as a semantic tie-break.
//!
//! # Safety
//!
//! No unsafe Rust is used.
//!
//! The file explicitly forbids unsafe code.
//!
//! Compatible with:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust.
//!
//! ============================================================================
//! Safety boundary
//! ============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap};

use crate::quantum::ir::core::identity::{OperationId, ResourceId};
use crate::quantum::scheduling::types::{Duration, TimePoint};

// ============================================================================
// Public error type
// ============================================================================

/// Errors specific to resource-constrained scheduling.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ResourceConstrainedSchedulingError {
    /// An operation is unknown to the scheduling problem.
    UnknownOperation {
        /// Unknown operation identity.
        operation: OperationId,
    },

    /// A dependency references an operation outside the problem.
    UnknownDependency {
        /// Operation containing the invalid dependency.
        operation: OperationId,

        /// Referenced predecessor.
        dependency: OperationId,
    },

    /// A dependency graph contains a cycle.
    CycleDetected {
        /// Operation at the detected cycle frontier.
        operation: OperationId,
    },

    /// The operation duration is invalid or unavailable.
    InvalidDuration {
        /// Operation whose duration is invalid.
        operation: OperationId,
    },

    /// Temporal bounds are inconsistent.
    InvalidTimingWindow {
        /// Operation whose bounds are invalid.
        operation: OperationId,
    },

    /// Adding a duration would overflow the time representation.
    TimeOverflow {
        /// Operation causing the overflow.
        operation: OperationId,
    },

    /// A resource referenced by an operation does not exist.
    UnknownResource {
        /// Operation requiring the resource.
        operation: OperationId,

        /// Missing resource.
        resource: ResourceId,
    },

    /// The requested amount exceeds resource capacity.
    CapacityExceeded {
        /// Operation requiring the resource.
        operation: OperationId,

        /// Resource whose capacity is insufficient.
        resource: ResourceId,
    },

    /// An operation has no feasible placement.
    Unschedulable {
        /// Operation that cannot be scheduled.
        operation: OperationId,
    },

    /// A deadline cannot be satisfied.
    DeadlineExceeded {
        /// Operation violating the deadline.
        operation: OperationId,
    },

    /// The model returned inconsistent data.
    InvalidModel {
        /// Operation associated with the inconsistency.
        operation: OperationId,
    },

    /// An explicit scheduler work limit was reached.
    LimitExceeded {
        /// Stable limit name.
        limit: &'static str,
    },
}

impl std::fmt::Display for ResourceConstrainedSchedulingError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownOperation { operation } => {
                write!(formatter, "unknown operation `{operation}`")
            }

            Self::UnknownDependency {
                operation,
                dependency,
            } => {
                write!(
                    formatter,
                    "operation `{operation}` references unknown dependency `{dependency}`"
                )
            }

            Self::CycleDetected { operation } => {
                write!(
                    formatter,
                    "dependency cycle detected at operation `{operation}`"
                )
            }

            Self::InvalidDuration { operation } => {
                write!(
                    formatter,
                    "invalid duration for operation `{operation}`"
                )
            }

            Self::InvalidTimingWindow { operation } => {
                write!(
                    formatter,
                    "invalid timing window for operation `{operation}`"
                )
            }

            Self::TimeOverflow { operation } => {
                write!(
                    formatter,
                    "time overflow while scheduling operation `{operation}`"
                )
            }

            Self::UnknownResource {
                operation,
                resource,
            } => {
                write!(
                    formatter,
                    "operation `{operation}` references unknown resource `{resource}`"
                )
            }

            Self::CapacityExceeded {
                operation,
                resource,
            } => {
                write!(
                    formatter,
                    "operation `{operation}` requires more capacity than resource `{resource}` provides"
                )
            }

            Self::Unschedulable { operation } => {
                write!(
                    formatter,
                    "operation `{operation}` has no feasible resource placement"
                )
            }

            Self::DeadlineExceeded { operation } => {
                write!(
                    formatter,
                    "deadline exceeded for operation `{operation}`"
                )
            }

            Self::InvalidModel { operation } => {
                write!(
                    formatter,
                    "invalid resource-constrained scheduling model for operation `{operation}`"
                )
            }

            Self::LimitExceeded { limit } => {
                write!(
                    formatter,
                    "resource-constrained scheduler limit exceeded: {limit}"
                )
            }
        }
    }
}

impl std::error::Error for ResourceConstrainedSchedulingError {}

/// Result alias for this planner.
pub type ResourceConstrainedResult<T> =
    Result<T, ResourceConstrainedSchedulingError>;

// ============================================================================
// Resource requirement
// ============================================================================

/// One resource requirement for an operation.
///
/// `amount` is capacity consumption, not a machine-size declaration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResourceRequirement {
    /// Required resource.
    pub resource: ResourceId,

    /// Capacity consumed while the operation executes.
    pub amount: u64,
}

impl ResourceRequirement {
    /// Creates a resource requirement.
    #[must_use]
    pub const fn new(resource: ResourceId, amount: u64) -> Self {
        Self { resource, amount }
    }
}

// ============================================================================
// Temporal bounds
// ============================================================================

/// Scheduling bounds for an operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TimingBounds {
    /// Earliest legal start.
    pub earliest_start: Option<TimePoint>,

    /// Latest legal start.
    pub latest_start: Option<TimePoint>,

    /// Latest legal finish.
    pub deadline: Option<TimePoint>,
}

impl TimingBounds {
    /// Creates unrestricted bounds.
    #[must_use]
    pub const fn unrestricted() -> Self {
        Self {
            earliest_start: None,
            latest_start: None,
            deadline: None,
        }
    }

    /// Validates start bounds.
    #[must_use]
    pub fn is_valid(self) -> bool {
        match (self.earliest_start, self.latest_start) {
            (Some(earliest), Some(latest)) => earliest <= latest,
            _ => true,
        }
    }
}

// ============================================================================
// Operation specification
// ============================================================================

/// Immutable scheduling information for one operation.
///
/// This is an algorithm-facing representation. It does not replace
/// `quantum::ir::QuantumOperation`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourceConstrainedOperation {
    /// Canonical operation identity.
    pub id: OperationId,

    /// Operation duration.
    pub duration: Duration,

    /// Direct predecessors.
    pub predecessors: Vec<OperationId>,

    /// Required resources.
    pub resources: Vec<ResourceRequirement>,

    /// Temporal bounds.
    pub timing: TimingBounds,

    /// User/compiler scheduling priority.
    ///
    /// Larger values have higher priority.
    pub priority: i64,
}

impl ResourceConstrainedOperation {
    /// Creates an operation.
    #[must_use]
    pub fn new(id: OperationId, duration: Duration) -> Self {
        Self {
            id,
            duration,
            predecessors: Vec::new(),
            resources: Vec::new(),
            timing: TimingBounds::unrestricted(),
            priority: 0,
        }
    }

    /// Adds a predecessor.
    #[must_use]
    pub fn with_predecessor(mut self, predecessor: OperationId) -> Self {
        self.predecessors.push(predecessor);
        self
    }

    /// Replaces resource requirements.
    #[must_use]
    pub fn with_resources(
        mut self,
        resources: Vec<ResourceRequirement>,
    ) -> Self {
        self.resources = resources;
        self
    }

    /// Sets temporal bounds.
    #[must_use]
    pub fn with_timing(mut self, timing: TimingBounds) -> Self {
        self.timing = timing;
        self
    }

    /// Sets scheduler priority.
    #[must_use]
    pub fn with_priority(mut self, priority: i64) -> Self {
        self.priority = priority;
        self
    }

    fn finish(
        &self,
        start: TimePoint,
    ) -> ResourceConstrainedResult<TimePoint> {
        start
            .checked_add(self.duration)
            .ok_or(ResourceConstrainedSchedulingError::TimeOverflow {
                operation: self.id,
            })
    }
}

// ============================================================================
// Resource model
// ============================================================================

/// Immutable description of one resource.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResourceCapacity {
    /// Resource identity.
    pub id: ResourceId,

    /// Total usable capacity.
    pub capacity: u64,
}

impl ResourceCapacity {
    /// Creates a resource capacity.
    #[must_use]
    pub const fn new(id: ResourceId, capacity: u64) -> Self {
        Self { id, capacity }
    }
}

/// Resource availability model.
///
/// Implementations may be backed by:
///
/// - resource calendars;
/// - hardware snapshots;
/// - simulation;
/// - distributed resource managers.
///
/// The planner does not know the implementation.
pub trait ResourceModel {
    /// Returns the capacity of a resource.
    fn capacity(
        &self,
        resource: ResourceId,
    ) -> Option<u64>;

    /// Returns whether the resource is available for the complete interval.
    fn available(
        &self,
        resource: ResourceId,
        start: TimePoint,
        finish: TimePoint,
        amount: u64,
        reservations: &[Reservation],
    ) -> bool;
}

// ============================================================================
// Reservation
// ============================================================================

/// One committed resource reservation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Reservation {
    /// Operation owning the reservation.
    pub operation: OperationId,

    /// Reserved resource.
    pub resource: ResourceId,

    /// Start time.
    pub start: TimePoint,

    /// Finish time.
    pub finish: TimePoint,

    /// Consumed capacity.
    pub amount: u64,
}

impl Reservation {
    /// Creates a reservation.
    #[must_use]
    pub const fn new(
        operation: OperationId,
        resource: ResourceId,
        start: TimePoint,
        finish: TimePoint,
        amount: u64,
    ) -> Self {
        Self {
            operation,
            resource,
            start,
            finish,
            amount,
        }
    }
}

// ============================================================================
// Schedule entry
// ============================================================================

/// Scheduled operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScheduledOperation {
    /// Operation identity.
    pub operation: OperationId,

    /// Scheduled start.
    pub start: TimePoint,

    /// Scheduled finish.
    pub finish: TimePoint,
}

/// Complete result produced by the RCPSP algorithm.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourceConstrainedSchedule {
    /// Scheduled operations ordered by deterministic operation identity.
    pub operations: BTreeMap<OperationId, ScheduledOperation>,

    /// Committed resource reservations.
    pub reservations: Vec<Reservation>,

    /// Final makespan.
    pub makespan: TimePoint,

    /// Number of scheduled operations.
    pub operation_count: usize,
}

impl ResourceConstrainedSchedule {
    /// Returns the scheduled operation for an ID.
    #[must_use]
    pub fn operation(
        &self,
        id: OperationId,
    ) -> Option<&ScheduledOperation> {
        self.operations.get(&id)
    }

    /// Returns reservations for one resource.
    pub fn reservations_for(
        &self,
        resource: ResourceId,
    ) -> impl Iterator<Item = &Reservation> {
        self.reservations
            .iter()
            .filter(move |reservation| reservation.resource == resource)
    }
}

// ============================================================================
// Problem model
// ============================================================================

/// Complete immutable RCPSP problem.
pub struct ResourceConstrainedProblem<'a, R> {
    /// Operations.
    pub operations: &'a [ResourceConstrainedOperation],

    /// Resource availability/capacity model.
    pub resources: &'a R,

    /// Optional global release time.
    pub release_time: TimePoint,

    /// Optional global deadline.
    pub deadline: Option<TimePoint>,
}

impl<'a, R> ResourceConstrainedProblem<'a, R> {
    /// Creates a problem.
    #[must_use]
    pub const fn new(
        operations: &'a [ResourceConstrainedOperation],
        resources: &'a R,
    ) -> Self {
        Self {
            operations,
            resources,
            release_time: TimePoint::ZERO,
            deadline: None,
        }
    }

    /// Sets the global release time.
    #[must_use]
    pub const fn with_release_time(
        mut self,
        release_time: TimePoint,
    ) -> Self {
        self.release_time = release_time;
        self
    }

    /// Sets a global deadline.
    #[must_use]
    pub const fn with_deadline(
        mut self,
        deadline: TimePoint,
    ) -> Self {
        self.deadline = Some(deadline);
        self
    }
}

// ============================================================================
// Internal operation state
// ============================================================================

#[derive(Debug, Clone, Copy)]
struct Candidate {
    operation: OperationId,
    earliest_start: TimePoint,
    priority: i64,
    slack: Option<Duration>,
    downstream: Duration,
}

impl Candidate {
    fn cmp_key(self) -> CandidateKey {
        CandidateKey {
            slack: self.slack,
            priority: self.priority,
            downstream: self.downstream,
            operation: self.operation,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct CandidateKey {
    slack: Option<Duration>,
    priority: i64,
    downstream: Duration,
    operation: OperationId,
}

impl Ord for CandidateKey {
    fn cmp(&self, other: &Self) -> Ordering {
        // Smaller slack is more urgent.
        //
        // Operations with no calculated slack are placed after operations
        // with finite slack.
        let slack_order = match (self.slack, other.slack) {
            (Some(left), Some(right)) => right.cmp(&left),
            (Some(_), None) => Ordering::Greater,
            (None, Some(_)) => Ordering::Less,
            (None, None) => Ordering::Equal,
        };

        if slack_order != Ordering::Equal {
            return slack_order;
        }

        // Higher priority first.
        let priority_order = self.priority.cmp(&other.priority);

        if priority_order != Ordering::Equal {
            return priority_order;
        }

        // Larger downstream path first.
        let downstream_order = self.downstream.cmp(&other.downstream);

        if downstream_order != Ordering::Equal {
            return downstream_order;
        }

        // BinaryHeap is a max heap, therefore reverse the operation ordering
        // so the smallest canonical operation ID wins.
        other.operation.cmp(&self.operation)
    }
}

impl PartialOrd for CandidateKey {
    fn partial_cmp(
        &self,
        other: &Self,
    ) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// ============================================================================
// Planner
// ============================================================================

/// Production resource-constrained scheduler.
///
/// This implementation is intentionally stateless.
///
/// All scheduling state belongs to a single invocation, which makes planner
/// instances reusable and naturally prevents cross-compilation state leakage.
#[derive(Debug, Clone, Copy, Default)]
pub struct ResourceConstrainedPlanner;

impl ResourceConstrainedPlanner {
    /// Creates a resource-constrained planner.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Schedules the supplied RCPSP problem.
    ///
    /// The algorithm is deterministic for identical inputs.
    ///
    /// The scheduler uses a serial resource-constrained list-scheduling
    /// strategy:
    ///
    /// 1. validate the operation set;
    /// 2. validate dependencies;
    /// 3. calculate dependency criticality;
    /// 4. maintain a ready set;
    /// 5. calculate earliest resource-feasible placement;
    /// 6. select the most constrained/highest-priority candidate;
    /// 7. reserve all resources atomically;
    /// 8. release newly ready successors;
    /// 9. repeat until complete.
    ///
    /// No time-slot matrix is created.
    pub fn schedule<R>(
        &self,
        problem: &ResourceConstrainedProblem<'_, R>,
    ) -> ResourceConstrainedResult<ResourceConstrainedSchedule>
    where
        R: ResourceModel,
    {
        let operation_map = self.validate_operations(problem.operations)?;

        if problem.operations.is_empty() {
            return Ok(ResourceConstrainedSchedule {
                operations: BTreeMap::new(),
                reservations: Vec::new(),
                makespan: problem.release_time,
                operation_count: 0,
            });
        }

        self.validate_dependencies(
            problem.operations,
            &operation_map,
        )?;

        let successor_map =
            self.build_successors(problem.operations);

        let mut remaining_predecessors =
            BTreeMap::<OperationId, usize>::new();

        for operation in problem.operations {
            remaining_predecessors.insert(
                operation.id,
                operation.predecessors.len(),
            );
        }

        let downstream =
            self.calculate_downstream_lengths(
                problem.operations,
                &successor_map,
            )?;

        let mut ready = BinaryHeap::<CandidateKey>::new();

        for operation in problem.operations {
            if operation.predecessors.is_empty() {
                let earliest =
                    self.initial_earliest_start(
                        operation,
                        problem.release_time,
                    )?;

                let slack =
                    self.calculate_slack(
                        operation,
                        earliest,
                        problem.deadline,
                    )?;

                ready.push(
                    Candidate {
                        operation: operation.id,
                        earliest_start: earliest,
                        priority: operation.priority,
                        slack,
                        downstream: *downstream
                            .get(&operation.id)
                            .unwrap_or(&Duration::ZERO),
                    }
                    .cmp_key(),
                );
            }
        }

        let mut scheduled =
            BTreeMap::<OperationId, ScheduledOperation>::new();

        let mut reservations = Vec::<Reservation>::new();

        let mut makespan = problem.release_time;

        while scheduled.len() < problem.operations.len() {
            if ready.is_empty() {
                let operation = problem
                    .operations
                    .iter()
                    .find(|operation| {
                        !scheduled.contains_key(&operation.id)
                    })
                    .map(|operation| operation.id)
                    .ok_or(
                        ResourceConstrainedSchedulingError::InvalidModel {
                            operation: OperationId::from(0),
                        },
                    )?;

                return Err(
                    ResourceConstrainedSchedulingError::CycleDetected {
                        operation,
                    },
                );
            }

            let candidate =
                ready.pop().expect("ready set was checked non-empty");

            if scheduled.contains_key(&candidate.operation) {
                continue;
            }

            let operation =
                operation_map
                    .get(&candidate.operation)
                    .ok_or(
                        ResourceConstrainedSchedulingError::UnknownOperation {
                            operation: candidate.operation,
                        },
                    )?;

            let start = self.find_feasible_start(
                operation,
                candidate.earliest_start,
                problem.resources,
                &reservations,
            )?;

            let finish = operation.finish(start)?;

            if let Some(deadline) = operation.timing.deadline {
                if finish > deadline {
                    return Err(
                        ResourceConstrainedSchedulingError::DeadlineExceeded {
                            operation: operation.id,
                        },
                    );
                }
            }

            if let Some(global_deadline) = problem.deadline {
                if finish > global_deadline {
                    return Err(
                        ResourceConstrainedSchedulingError::DeadlineExceeded {
                            operation: operation.id,
                        },
                    );
                }
            }

            let new_reservations =
                self.build_reservations(
                    operation,
                    start,
                    finish,
                    problem.resources,
                    &reservations,
                )?;

            reservations.extend(new_reservations);

            scheduled.insert(
                operation.id,
                ScheduledOperation {
                    operation: operation.id,
                    start,
                    finish,
                },
            );

            if finish > makespan {
                makespan = finish;
            }

            if let Some(successors) =
                successor_map.get(&operation.id)
            {
                for successor_id in successors {
                    let count =
                        remaining_predecessors
                            .get_mut(successor_id)
                            .ok_or(
                                ResourceConstrainedSchedulingError::InvalidModel {
                                    operation: *successor_id,
                                },
                            )?;

                    if *count == 0 {
                        return Err(
                            ResourceConstrainedSchedulingError::InvalidModel {
                                operation: *successor_id,
                            },
                        );
                    }

                    *count -= 1;

                    if *count == 0 {
                        let successor =
                            operation_map
                                .get(successor_id)
                                .ok_or(
                                    ResourceConstrainedSchedulingError::UnknownOperation {
                                        operation: *successor_id,
                                    },
                                )?;

                        let earliest =
                            self.predecessor_finish_time(
                                successor,
                                &scheduled,
                                problem.release_time,
                            )?;

                        let slack =
                            self.calculate_slack(
                                successor,
                                earliest,
                                problem.deadline,
                            )?;

                        ready.push(
                            Candidate {
                                operation: successor.id,
                                earliest_start: earliest,
                                priority: successor.priority,
                                slack,
                                downstream: *downstream
                                    .get(&successor.id)
                                    .unwrap_or(&Duration::ZERO),
                            }
                            .cmp_key(),
                        );
                    }
                }
            }
        }

        Ok(ResourceConstrainedSchedule {
            operations: scheduled,
            reservations,
            makespan,
            operation_count: problem.operations.len(),
        })
    }

    fn validate_operations<'a>(
        &self,
        operations: &'a [ResourceConstrainedOperation],
    ) -> ResourceConstrainedResult<
        BTreeMap<OperationId, &'a ResourceConstrainedOperation>,
    > {
        let mut map = BTreeMap::new();

        for operation in operations {
            if map.insert(operation.id, operation).is_some() {
                return Err(
                    ResourceConstrainedSchedulingError::InvalidModel {
                        operation: operation.id,
                    },
                );
            }

            if !operation.timing.is_valid() {
                return Err(
                    ResourceConstrainedSchedulingError::InvalidTimingWindow {
                        operation: operation.id,
                    },
                );
            }
        }

        Ok(map)
    }

    fn validate_dependencies(
        &self,
        operations: &[ResourceConstrainedOperation],
        operation_map: &BTreeMap<
            OperationId,
            &ResourceConstrainedOperation,
        >,
    ) -> ResourceConstrainedResult<()> {
        for operation in operations {
            let mut seen = BTreeSet::new();

            for predecessor in &operation.predecessors {
                if !operation_map.contains_key(predecessor) {
                    return Err(
                        ResourceConstrainedSchedulingError::UnknownDependency {
                            operation: operation.id,
                            dependency: *predecessor,
                        },
                    );
                }

                if !seen.insert(*predecessor) {
                    return Err(
                        ResourceConstrainedSchedulingError::InvalidModel {
                            operation: operation.id,
                        },
                    );
                }
            }
        }

        Ok(())
    }

    fn build_successors(
        &self,
        operations: &[ResourceConstrainedOperation],
    ) -> BTreeMap<OperationId, Vec<OperationId>> {
        let mut successors =
            BTreeMap::<OperationId, Vec<OperationId>>::new();

        for operation in operations {
            successors.entry(operation.id).or_default();

            for predecessor in &operation.predecessors {
                successors
                    .entry(*predecessor)
                    .or_default()
                    .push(operation.id);
            }
        }

        for values in successors.values_mut() {
            values.sort();
            values.dedup();
        }

        successors
    }

    fn calculate_downstream_lengths(
        &self,
        operations: &[ResourceConstrainedOperation],
        successors: &BTreeMap<OperationId, Vec<OperationId>>,
    ) -> ResourceConstrainedResult<
        BTreeMap<OperationId, Duration>,
    > {
        let mut indegree =
            BTreeMap::<OperationId, usize>::new();

        for operation in operations {
            indegree.insert(
                operation.id,
                operation.predecessors.len(),
            );
        }

        let mut queue =
            BTreeSet::<OperationId>::new();

        for operation in operations {
            if operation.predecessors.is_empty() {
                queue.insert(operation.id);
            }
        }

        let mut topological =
            Vec::<OperationId>::with_capacity(operations.len());

        while let Some(operation) =
            queue.pop_first()
        {
            topological.push(operation);

            if let Some(next) = successors.get(&operation) {
                for successor in next {
                    let count =
                        indegree
                            .get_mut(successor)
                            .ok_or(
                                ResourceConstrainedSchedulingError::InvalidModel {
                                    operation: *successor,
                                },
                            )?;

                    if *count == 0 {
                        return Err(
                            ResourceConstrainedSchedulingError::InvalidModel {
                                operation: *successor,
                            },
                        );
                    }

                    *count -= 1;

                    if *count == 0 {
                        queue.insert(*successor);
                    }
                }
            }
        }

        if topological.len() != operations.len() {
            let operation = operations
                .iter()
                .find(|operation| !topological.contains(&operation.id))
                .map(|operation| operation.id)
                .unwrap_or_else(|| operations[0].id);

            return Err(
                ResourceConstrainedSchedulingError::CycleDetected {
                    operation,
                },
            );
        }

        let operation_map = operations
            .iter()
            .map(|operation| (operation.id, operation))
            .collect::<BTreeMap<_, _>>();

        let mut downstream =
            BTreeMap::<OperationId, Duration>::new();

        for operation_id in topological.into_iter().rev() {
            let operation =
                operation_map
                    .get(&operation_id)
                    .ok_or(
                        ResourceConstrainedSchedulingError::UnknownOperation {
                            operation: operation_id,
                        },
                    )?;

            let mut longest =
                Duration::ZERO;

            if let Some(next) = successors.get(&operation_id) {
                for successor in next {
                    let successor_length =
                        *downstream
                            .get(successor)
                            .unwrap_or(&Duration::ZERO);

                    let successor_operation =
                        operation_map
                            .get(successor)
                            .ok_or(
                                ResourceConstrainedSchedulingError::UnknownOperation {
                                    operation: *successor,
                                },
                            )?;

                    let total =
                        successor_operation
                            .duration
                            .checked_add(successor_length)
                            .ok_or(
                                ResourceConstrainedSchedulingError::TimeOverflow {
                                    operation: *successor,
                                },
                            )?;

                    if total > longest {
                        longest = total;
                    }
                }
            }

            let _ = operation;

            downstream.insert(operation_id, longest);
        }

        Ok(downstream)
    }

    fn initial_earliest_start(
        &self,
        operation: &ResourceConstrainedOperation,
        release_time: TimePoint,
    ) -> ResourceConstrainedResult<TimePoint> {
        let mut earliest = release_time;

        if let Some(operation_release) =
            operation.timing.earliest_start
        {
            if operation_release > earliest {
                earliest = operation_release;
            }
        }

        if let Some(latest) = operation.timing.latest_start {
            if earliest > latest {
                return Err(
                    ResourceConstrainedSchedulingError::InvalidTimingWindow {
                        operation: operation.id,
                    },
                );
            }
        }

        Ok(earliest)
    }

    fn predecessor_finish_time(
        &self,
        operation: &ResourceConstrainedOperation,
        scheduled: &BTreeMap<
            OperationId,
            ScheduledOperation,
        >,
        release_time: TimePoint,
    ) -> ResourceConstrainedResult<TimePoint> {
        let mut earliest = release_time;

        if let Some(operation_release) =
            operation.timing.earliest_start
        {
            if operation_release > earliest {
                earliest = operation_release;
            }
        }

        for predecessor in &operation.predecessors {
            let finish =
                scheduled
                    .get(predecessor)
                    .ok_or(
                        ResourceConstrainedSchedulingError::InvalidModel {
                            operation: operation.id,
                        },
                    )?
                    .finish;

            if finish > earliest {
                earliest = finish;
            }
        }

        if let Some(latest) =
            operation.timing.latest_start
        {
            if earliest > latest {
                return Err(
                    ResourceConstrainedSchedulingError::Unschedulable {
                        operation: operation.id,
                    },
                );
            }
        }

        Ok(earliest)
    }

    fn calculate_slack(
        &self,
        operation: &ResourceConstrainedOperation,
        earliest: TimePoint,
        global_deadline: Option<TimePoint>,
    ) -> ResourceConstrainedResult<Option<Duration>> {
        let deadline =
            operation.timing.deadline.or(global_deadline);

        let Some(deadline) = deadline else {
            return Ok(None);
        };

        let finish =
            operation.finish(earliest)?;

        if finish > deadline {
            return Err(
                ResourceConstrainedSchedulingError::DeadlineExceeded {
                    operation: operation.id,
                },
            );
        }

        Ok(finish.checked_duration_until(deadline))
    }

    fn find_feasible_start<R>(
        &self,
        operation: &ResourceConstrainedOperation,
        earliest: TimePoint,
        resources: &R,
        reservations: &[Reservation],
    ) -> ResourceConstrainedResult<TimePoint>
    where
        R: ResourceModel,
    {
        self.validate_resource_requirements(
            operation,
            resources,
        )?;

        let mut candidate = earliest;

        loop {
            let finish = operation.finish(candidate)?;

            if let Some(latest_start) =
                operation.timing.latest_start
            {
                if candidate > latest_start {
                    return Err(
                        ResourceConstrainedSchedulingError::Unschedulable {
                            operation: operation.id,
                        },
                    );
                }
            }

            if let Some(deadline) =
                operation.timing.deadline
            {
                if finish > deadline {
                    return Err(
                        ResourceConstrainedSchedulingError::DeadlineExceeded {
                            operation: operation.id,
                        },
                    );
                }
            }

            let mut conflict_end = None;

            for requirement in &operation.resources {
                if requirement.amount == 0 {
                    continue;
                }

                if !resources.available(
                    requirement.resource,
                    candidate,
                    finish,
                    requirement.amount,
                    reservations,
                ) {
                    let next =
                        self.next_resource_boundary(
                            requirement.resource,
                            candidate,
                            reservations,
                        )?;

                    conflict_end = Some(match conflict_end {
                        Some(current) if current > next => current,
                        _ => next,
                    });
                }
            }

            match conflict_end {
                Some(next) => {
                    if next <= candidate {
                        return Err(
                            ResourceConstrainedSchedulingError::Unschedulable {
                                operation: operation.id,
                            },
                        );
                    }

                    candidate = next;
                }

                None => return Ok(candidate),
            }
        }
    }

    fn validate_resource_requirements<R>(
        &self,
        operation: &ResourceConstrainedOperation,
        resources: &R,
    ) -> ResourceConstrainedResult<()>
    where
        R: ResourceModel,
    {
        let mut aggregate =
            BTreeMap::<ResourceId, u64>::new();

        for requirement in &operation.resources {
            if requirement.amount == 0 {
                continue;
            }

            let capacity =
                resources
                    .capacity(requirement.resource)
                    .ok_or(
                        ResourceConstrainedSchedulingError::UnknownResource {
                            operation: operation.id,
                            resource: requirement.resource,
                        },
                    )?;

            let current =
                aggregate
                    .get(&requirement.resource)
                    .copied()
                    .unwrap_or(0);

            let total =
                current
                    .checked_add(requirement.amount)
                    .ok_or(
                        ResourceConstrainedSchedulingError::CapacityExceeded {
                            operation: operation.id,
                            resource: requirement.resource,
                        },
                    )?;

            if total > capacity {
                return Err(
                    ResourceConstrainedSchedulingError::CapacityExceeded {
                        operation: operation.id,
                        resource: requirement.resource,
                    },
                );
            }

            aggregate.insert(
                requirement.resource,
                total,
            );
        }

        Ok(())
    }

    fn build_reservations<R>(
        &self,
        operation: &ResourceConstrainedOperation,
        start: TimePoint,
        finish: TimePoint,
        resources: &R,
        reservations: &[Reservation],
    ) -> ResourceConstrainedResult<Vec<Reservation>>
    where
        R: ResourceModel,
    {
        self.validate_resource_requirements(
            operation,
            resources,
        )?;

        let mut result =
            Vec::<Reservation>::with_capacity(
                operation.resources.len(),
            );

        for requirement in &operation.resources {
            if requirement.amount == 0 {
                continue;
            }

            if !resources.available(
                requirement.resource,
                start,
                finish,
                requirement.amount,
                reservations,
            ) {
                return Err(
                    ResourceConstrainedSchedulingError::Unschedulable {
                        operation: operation.id,
                    },
                );
            }

            result.push(
                Reservation::new(
                    operation.id,
                    requirement.resource,
                    start,
                    finish,
                    requirement.amount,
                ),
            );
        }

        Ok(result)
    }

    fn next_resource_boundary(
        &self,
        resource: ResourceId,
        candidate: TimePoint,
        reservations: &[Reservation],
    ) -> ResourceConstrainedResult<TimePoint> {
        let mut next = None;

        for reservation in reservations {
            if reservation.resource != resource {
                continue;
            }

            if reservation.finish <= candidate {
                continue;
            }

            if reservation.start > candidate {
                next = Some(match next {
                    Some(current) if current < reservation.start => current,
                    Some(current) => current,
                    None => reservation.start,
                });
            } else {
                next = Some(match next {
                    Some(current) if current > reservation.finish => current,
                    _ => reservation.finish,
                });
            }
        }

        next.ok_or(
            ResourceConstrainedSchedulingError::Unschedulable {
                operation: OperationId::from(0),
            },
        )
    }
}

// ============================================================================
// Integration adapter
// ============================================================================

/// Converts an existing resource-constrained scheduling problem into the
/// planner's execution form.
///
/// This trait is deliberately small.
///
/// The canonical repository adapters should implement it rather than making
/// this algorithm depend directly on:
///
/// - `SchedulingContext`;
/// - `quantum::ir::QuantumCircuit`;
/// - hardware providers;
/// - routing implementations;
/// - QEC implementations.
///
/// This preserves dependency direction.
pub trait ResourceConstrainedInput {
    /// Resource model type supplied by the target.
    type Resources: ResourceModel;

    /// Returns the operation workload.
    fn operations(&self) -> &[ResourceConstrainedOperation];

    /// Returns the target resource model.
    fn resources(&self) -> &Self::Resources;

    /// Returns the global scheduling release time.
    fn release_time(&self) -> TimePoint {
        TimePoint::ZERO
    }

    /// Returns an optional global deadline.
    fn deadline(&self) -> Option<TimePoint> {
        None
    }
}

impl ResourceConstrainedPlanner {
    /// Schedules a repository-provided adapter input.
    ///
    /// This is the intended integration boundary for `SchedulingContext`.
    pub fn plan<I>(
        &self,
        input: &I,
    ) -> ResourceConstrainedResult<ResourceConstrainedSchedule>
    where
        I: ResourceConstrainedInput,
    {
        let problem =
            ResourceConstrainedProblem::new(
                input.operations(),
                input.resources(),
            )
            .with_release_time(input.release_time());

        let problem =
            match input.deadline() {
                Some(deadline) =>
                    problem.with_deadline(deadline),
                None => problem,
            };

        self.schedule(&problem)
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct TestResources {
        capacities: BTreeMap<ResourceId, u64>,
    }

    impl TestResources {
        fn with(
            mut self,
            resource: ResourceId,
            capacity: u64,
        ) -> Self {
            self.capacities.insert(resource, capacity);
            self
        }
    }

    impl ResourceModel for TestResources {
        fn capacity(
            &self,
            resource: ResourceId,
        ) -> Option<u64> {
            self.capacities.get(&resource).copied()
        }

        fn available(
            &self,
            resource: ResourceId,
            start: TimePoint,
            finish: TimePoint,
            amount: u64,
            reservations: &[Reservation],
        ) -> bool {
            let capacity =
                match self.capacity(resource) {
                    Some(value) => value,
                    None => return false,
                };

            let mut used = 0u64;

            for reservation in reservations {
                if reservation.resource != resource {
                    continue;
                }

                let overlaps =
                    reservation.start < finish
                        && start < reservation.finish;

                if overlaps {
                    used = match used.checked_add(
                        reservation.amount,
                    ) {
                        Some(value) => value,
                        None => return false,
                    };
                }
            }

            match used.checked_add(amount) {
                Some(total) => total <= capacity,
                None => false,
            }
        }
    }

    #[test]
    fn independent_operations_share_capacity() {
        let resource = ResourceId::from(1u64);

        let operations = vec![
            ResourceConstrainedOperation::new(
                OperationId::from(1u64),
                Duration::new(10),
            )
            .with_resources(vec![
                ResourceRequirement::new(resource, 1),
            ]),
            ResourceConstrainedOperation::new(
                OperationId::from(2u64),
                Duration::new(10),
            )
            .with_resources(vec![
                ResourceRequirement::new(resource, 1),
            ]),
        ];

        let resources =
            TestResources::default().with(resource, 1);

        let problem =
            ResourceConstrainedProblem::new(
                &operations,
                &resources,
            );

        let result =
            ResourceConstrainedPlanner::new()
                .schedule(&problem)
                .expect("schedule must succeed");

        assert_eq!(result.operation_count, 2);
        assert_eq!(result.makespan, TimePoint::new(20));
    }

    #[test]
    fn independent_operations_run_in_parallel_with_capacity_two() {
        let resource = ResourceId::from(1u64);

        let operations = vec![
            ResourceConstrainedOperation::new(
                OperationId::from(1u64),
                Duration::new(10),
            )
            .with_resources(vec![
                ResourceRequirement::new(resource, 1),
            ]),
            ResourceConstrainedOperation::new(
                OperationId::from(2u64),
                Duration::new(10),
            )
            .with_resources(vec![
                ResourceRequirement::new(resource, 1),
            ]),
        ];

        let resources =
            TestResources::default().with(resource, 2);

        let problem =
            ResourceConstrainedProblem::new(
                &operations,
                &resources,
            );

        let result =
            ResourceConstrainedPlanner::new()
                .schedule(&problem)
                .expect("schedule must succeed");

        assert_eq!(result.operation_count, 2);
        assert_eq!(result.makespan, TimePoint::new(10));
    }

    #[test]
    fn dependencies_are_preserved() {
        let resource = ResourceId::from(1u64);

        let operations = vec![
            ResourceConstrainedOperation::new(
                OperationId::from(1u64),
                Duration::new(10),
            )
            .with_resources(vec![
                ResourceRequirement::new(resource, 1),
            ]),
            ResourceConstrainedOperation::new(
                OperationId::from(2u64),
                Duration::new(10),
            )
            .with_predecessor(OperationId::from(1u64))
            .with_resources(vec![
                ResourceRequirement::new(resource, 1),
            ]),
        ];

        let resources =
            TestResources::default().with(resource, 1);

        let problem =
            ResourceConstrainedProblem::new(
                &operations,
                &resources,
            );

        let result =
            ResourceConstrainedPlanner::new()
                .schedule(&problem)
                .expect("schedule must succeed");

        assert_eq!(
            result.operation(1u64.into())
                .expect("operation 1 exists")
                .finish,
            TimePoint::new(10)
        );

        assert_eq!(
            result.operation(2u64.into())
                .expect("operation 2 exists")
                .start,
            TimePoint::new(10)
        );
    }

    #[test]
    fn cycle_is_rejected() {
        let operations = vec![
            ResourceConstrainedOperation::new(
                OperationId::from(1u64),
                Duration::new(1),
            )
            .with_predecessor(OperationId::from(2u64)),
            ResourceConstrainedOperation::new(
                OperationId::from(2u64),
                Duration::new(1),
            )
            .with_predecessor(OperationId::from(1u64)),
        ];

        let resources = TestResources::default();

        let problem =
            ResourceConstrainedProblem::new(
                &operations,
                &resources,
            );

        let result =
            ResourceConstrainedPlanner::new()
                .schedule(&problem);

        assert!(matches!(
            result,
            Err(
                ResourceConstrainedSchedulingError::CycleDetected {
                    ..
                }
            )
        ));
    }

    #[test]
    fn deadline_is_enforced() {
        let operation =
            ResourceConstrainedOperation::new(
                OperationId::from(1u64),
                Duration::new(10),
            )
            .with_timing(TimingBounds {
                earliest_start: None,
                latest_start: None,
                deadline: Some(TimePoint::new(5)),
            });

        let operations = vec![operation];

        let resources = TestResources::default();

        let problem =
            ResourceConstrainedProblem::new(
                &operations,
                &resources,
            );

        let result =
            ResourceConstrainedPlanner::new()
                .schedule(&problem);

        assert!(matches!(
            result,
            Err(
                ResourceConstrainedSchedulingError::DeadlineExceeded {
                    ..
                }
            )
        ));
    }

    #[test]
    fn zero_duration_is_supported() {
        let operation =
            ResourceConstrainedOperation::new(
                OperationId::from(1u64),
                Duration::ZERO,
            );

        let operations = vec![operation];

        let resources = TestResources::default();

        let problem =
            ResourceConstrainedProblem::new(
                &operations,
                &resources,
            );

        let result =
            ResourceConstrainedPlanner::new()
                .schedule(&problem)
                .expect("zero-duration operation is valid");

        assert_eq!(
            result.makespan,
            TimePoint::ZERO
        );
    }
}