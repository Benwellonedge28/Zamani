//! Zamani Quantum Scheduling — Production List Scheduler
//!
//! This module implements the generic resource-aware list-scheduling
//! algorithm used by the Zamani quantum scheduler.
//!
//! ============================================================================
//! ARCHITECTURAL ROLE
//! ============================================================================
//!
//! List scheduling answers:
//!
//! > Given a set of executable operations, their dependencies, temporal
//! > requirements, and resource requirements, which ready operation should be
//! > scheduled next and at what earliest legal time?
//!
//! This module is an ALGORITHM implementation.
//!
//! It does NOT own:
//!
//! - Zamani source parsing;
//! - canonical quantum semantics;
//! - logical-to-physical routing;
//! - hardware discovery;
//! - QPU communication;
//! - hardware execution;
//! - calibration acquisition;
//! - QEC decoding;
//! - noise modelling;
//! - target discovery;
//! - scheduling context construction;
//! - canonical dependency-graph construction;
//! - serialization;
//! - final schedule verification;
//! - benchmark execution.
//!
//! Those responsibilities remain in their respective subsystems.
//!
//! ============================================================================
//! PIPELINE POSITION
//! ============================================================================
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
//!      ├── dependency graph
//!      ├── operation metadata
//!      └── canonical operation identity
//!      │
//!      ▼
//! scheduling::context
//!      │
//!      ├── target capabilities
//!      ├── timing model
//!      ├── resources
//!      ├── constraints
//!      └── policy
//!      │
//!      ▼
//! planners::list              <── this module
//!      │
//!      ▼
//! scheduling::result
//!      │
//!      ▼
//! verification
//!      │
//!      ▼
//! hardware lowering / runtime
//! ```
//!
//! The existing `SchedulingContext` is deliberately an immutable target
//! snapshot rather than a hardware connection. This algorithm therefore
//! receives a scheduling problem/model and never discovers or queries hardware
//! itself.
//!
//! ============================================================================
//! WHY LIST SCHEDULING
//! ============================================================================
//!
//! List scheduling is a natural general-purpose baseline for quantum execution
//! scheduling because it combines:
//!
//! - dependency readiness;
//! - operation priority;
//! - resource availability;
//! - temporal constraints;
//! - deterministic arbitration.
//!
//! The algorithm repeatedly:
//!
//! ```text
//! 1. identify ready operations;
//! 2. determine their earliest feasible starts;
//! 3. rank candidates;
//! 4. select a candidate;
//! 5. reserve its resources;
//! 6. emit its scheduled interval;
//! 7. release newly available successors;
//! 8. continue until all operations are scheduled.
//! ```
//!
//! It does NOT claim globally optimal scheduling for arbitrary resource-
//! constrained problems. Exact resource-constrained scheduling is generally
//! computationally difficult.
//!
//! Instead this implementation provides:
//!
//! - deterministic scheduling;
//! - resource-aware scheduling;
//! - explicit temporal constraints;
//! - pluggable priority;
//! - pluggable resource feasibility;
//! - scalable event-driven advancement;
//! - no machine-size constants.
//!
//! ============================================================================
//! UNIVERSAL-PROGRAM PRINCIPLE
//! ============================================================================
//!
//! A Zamani program must describe the computation, not the machine.
//!
//! Therefore this file contains no:
//!
//! ```text
//! MAX_QUBITS
//! MAX_OPERATIONS
//! MAX_RESOURCES
//! MAX_CHANNELS
//! MAX_DEPTH
//! MAX_PARALLELISM
//! ```
//!
//! It also contains no assumptions about:
//!
//! - one-qubit operations;
//! - two-qubit operations;
//! - fixed gate arity;
//! - grid topology;
//! - linear topology;
//! - superconducting hardware;
//! - trapped ions;
//! - neutral atoms;
//! - photonics;
//! - annealers;
//! - a particular vendor;
//! - a particular number of QPUs.
//!
//! Target-specific information is supplied through the model traits below.
//!
//! "Infinity" therefore means:
//!
//! > no artificial finite machine-size ceiling is encoded in this algorithm.
//!
//! A concrete compilation remains bounded by actual host memory, address space,
//! compilation time, explicit scheduler limits, and available target
//! resources.
//!
//! ============================================================================
//! CANONICAL IDENTITY RULE
//! ============================================================================
//!
//! This file does not define a new operation or resource identity.
//!
//! Operation identity ultimately comes from:
//!
//! ```text
//! crate::quantum::ir::core::identity::OperationId
//! ```
//!
//! Resource identity ultimately comes from:
//!
//! ```text
//! crate::quantum::ir::core::identity::ResourceId
//! ```
//!
//! Logical and physical qubit identity remains owned by:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! A qubit may become a scheduling resource through an adapter, but this
//! algorithm must not manufacture another qubit identity.
//!
//! ============================================================================
//! TIME OWNERSHIP
//! ============================================================================
//!
//! This file uses the scheduler's canonical:
//!
//! ```text
//! crate::quantum::scheduling::types::TimePoint
//! crate::quantum::scheduling::types::Duration
//! ```
//!
//! It does not create its own time representation.
//!
//! No physical unit is assumed.
//!
//! The timing adapter determines whether the target representation corresponds
//! to:
//!
//! - device ticks;
//! - nanoseconds;
//! - picoseconds;
//! - pulse samples;
//! - rationalized time;
//! - another target-defined timing coordinate.
//!
//! ============================================================================
//! RESOURCE MODEL
//! ============================================================================
//!
//! List scheduling does not assume that a quantum operation consumes only
//! qubits.
//!
//! A model may expose arbitrary resources:
//!
//! ```text
//! logical/physical qubits
//! control channels
//! readout channels
//! couplers
//! resonators
//! lasers
//! measurement electronics
//! classical processors
//! communication links
//! cryogenic resources
//! QEC ancillas
//! network resources
//! custom target resources
//! ```
//!
//! A resource may have arbitrary capacity.
//!
//! Capacity `1` naturally represents an exclusive resource.
//!
//! Capacity greater than `1` represents a shared capacity-limited resource.
//!
//! The algorithm never assumes a maximum capacity.
//!
//! ============================================================================
//! RESOURCE RESERVATION
//! ============================================================================
//!
//! The algorithm does not mutate a hardware resource calendar directly.
//!
//! Instead `ListResourceModel` supplies an abstract transactional reservation
//! boundary.
//!
//! The implementation contract is:
//!
//! ```text
//! can_reserve(...)
//!       │
//!       ▼
//! reserve(...)
//!       │
//!       ▼
//! reservation token
//! ```
//!
//! A reservation is committed only after the selected operation has passed the
//! model's feasibility checks.
//!
//! This prevents the list scheduler from knowing how a resource calendar is
//! internally implemented.
//!
//! ============================================================================
//! DEPENDENCIES
//! ============================================================================
//!
//! Dependencies are supplied through `ListDependencyModel`.
//!
//! The algorithm requires:
//!
//! - operation enumeration;
//! - predecessor enumeration;
//! - successor enumeration;
//! - validation of operation membership.
//!
//! The dependency graph itself remains owned by:
//!
//! ```text
//! crate::quantum::scheduling::ir::graph
//! ```
//!
//! The scheduler does not construct a competing graph representation.
//!
//! ============================================================================
//! DYNAMIC CIRCUITS
//! ============================================================================
//!
//! Static list scheduling operates on dependencies known at planning time.
//!
//! Dynamic operations may still participate when the adapter represents their
//! currently known readiness constraints.
//!
//! Runtime-only dependencies must not be falsely converted into static
//! dependencies.
//!
//! Such operations should be represented through the dynamic scheduling
//! subsystem and may invoke this algorithm incrementally for a newly available
//! scheduling region.
//!
//! ============================================================================
//! DISTRIBUTED QUANTUM COMPUTING
//! ============================================================================
//!
//! Communication resources are ordinary resources from this algorithm's
//! perspective.
//!
//! Therefore:
//!
//! ```text
//! local operation
//! remote communication
//! entanglement generation
//! synchronization
//! classical feedback
//! ```
//!
//! can all be represented by the same scheduling model.
//!
//! Network topology and communication semantics remain outside this file.
//!
//! ============================================================================
//! QEC
//! ============================================================================
//!
//! QEC operations can be scheduled by this algorithm when the QEC adapter
//! exposes:
//!
//! - dependencies;
//! - durations;
//! - resource requirements;
//! - timing constraints;
//! - priorities.
//!
//! This algorithm does not implement stabilizer extraction, syndrome decoding,
//! surface-code logic, or recovery semantics.
//!
//! ============================================================================
//! DETERMINISM
//! ============================================================================
//!
//! Determinism is a first-class property.
//!
//! When deterministic mode is enabled, candidate ordering must not depend on:
//!
//! - hash-map iteration order;
//! - memory addresses;
//! - thread timing;
//! - operating-system scheduling;
//! - pointer identity;
//! - unspecified collection ordering.
//!
//! The caller supplies a priority value and operation identity provides a final
//! stable tie-break.
//!
//! The algorithm itself does not use randomness.
//!
//! ============================================================================
//! SCALABILITY
//! ============================================================================
//!
//! This implementation intentionally avoids a time-slot matrix such as:
//!
//! ```text
//! qubits × time
//! ```
//!
//! or:
//!
//! ```text
//! resources × maximum_time
//! ```
//!
//! Such representations scale with the scheduling horizon and can become
//! impractical on large machines.
//!
//! Instead, the algorithm maintains:
//!
//! - unscheduled operation state;
//! - dependency readiness;
//! - candidate heap;
//! - event queue;
//! - resource model state.
//!
//! The algorithm uses iterative processing and does not recursively traverse
//! the dependency graph.
//!
//! Dependency processing is O(V + E) apart from priority/resource-model costs.
//!
//! Actual scheduling complexity depends on:
//!
//! - number of operations;
//! - number of dependency edges;
//! - candidate ordering;
//! - resource contention;
//! - resource-calendar implementation;
//! - target constraints.
//!
//! No stronger global complexity guarantee is claimed.
//!
//! ============================================================================
//! ERROR HANDLING
//! ============================================================================
//!
//! Normal scheduling failure must be represented explicitly.
//!
//! Examples:
//!
//! - cycle detected;
//! - unknown operation;
//! - invalid duration;
//! - impossible timing window;
//! - unavailable resource;
//! - resource capacity violation;
//! - arithmetic overflow;
//! - scheduler limit reached.
//!
//! The algorithm does not silently drop an operation.
//!
//! It does not return a partial successful schedule as if it were complete.
//!
//! ============================================================================
//! TRANSACTIONAL RESOURCE BEHAVIOUR
//! ============================================================================
//!
//! The scheduler reserves resources only for a selected operation.
//!
//! If reservation fails, the operation is not marked scheduled.
//!
//! This preserves the invariant:
//!
//! ```text
//! scheduled operation
//!     ↔
//! successfully committed resource reservation
//! ```
//!
//! A resource model must therefore make reservation atomic from the caller's
//! perspective.
//!
//! ============================================================================
//! VERIFICATION
//! ============================================================================
//!
//! This algorithm establishes scheduling decisions but does not replace the
//! production verification subsystem.
//!
//! Final verification must independently check:
//!
//! - every required operation is present;
//! - no operation is duplicated;
//! - dependency ordering;
//! - resource capacity;
//! - timing windows;
//! - alignment;
//! - target support;
//! - semantic preservation.
//!
//! ============================================================================
//! RUST CONTRACT
//! ============================================================================
//!
//! Supported:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! This file deliberately contains no unsafe implementation.
//!
//! ============================================================================
//! FROZEN-FILE CONTRACT
//! ============================================================================
//!
//! This file is intentionally independent from concrete implementations of:
//!
//! - `SchedulingContext`;
//! - `SchedulingResult`;
//! - resource calendars;
//! - dependency graph storage;
//! - ASAP;
//! - ALAP;
//! - critical-path scheduling;
//! - RCPSP;
//! - adaptive scheduling;
//! - QEC scheduling;
//! - distributed scheduling;
//! - hardware adapters.
//!
//! Those components implement the model traits defined here.
//!
//! Therefore adding a new target, hardware provider, resource type, timing
//! representation, routing strategy, QEC protocol, or scheduler result
//! representation must not require changing this algorithm.
//!
//! ============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap};

use crate::quantum::ir::core::identity::{OperationId, ResourceId};
use crate::quantum::scheduling::types::{Duration, TimePoint};

// ============================================================================
// Public errors
// ============================================================================

/// Errors produced by the list-scheduling algorithm.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ListSchedulingError {
    /// The input contains no operations.
    ///
    /// An empty program is a valid scheduling problem in some compiler modes,
    /// so callers can explicitly opt into accepting it through
    /// `ListScheduler::allow_empty`.
    EmptyProblem,

    /// An operation appeared more than once.
    DuplicateOperation {
        /// Duplicated operation identity.
        operation: OperationId,
    },

    /// A dependency references an operation not present in the problem.
    UnknownOperation {
        /// Missing operation.
        operation: OperationId,
    },

    /// The dependency graph contains a cycle.
    CycleDetected {
        /// Operation participating in the detected cycle frontier.
        operation: OperationId,
    },

    /// The operation duration is unavailable.
    MissingDuration {
        /// Operation requiring a duration.
        operation: OperationId,
    },

    /// An operation has an invalid temporal interval.
    InvalidTimingWindow {
        /// Operation with invalid timing.
        operation: OperationId,
    },

    /// Arithmetic exceeded the scheduler's representable time domain.
    TimeOverflow {
        /// Operation involved in the calculation.
        operation: OperationId,
    },

    /// An operation cannot be placed on the supplied resources.
    UnschedulableResource {
        /// Operation that cannot be placed.
        operation: OperationId,
    },

    /// Resource reservation failed after feasibility was established.
    ReservationFailed {
        /// Operation for which reservation failed.
        operation: OperationId,
    },

    /// A scheduling operation failed because a required resource was unknown.
    UnknownResource {
        /// Operation requesting the resource.
        operation: OperationId,

        /// Unknown resource.
        resource: ResourceId,
    },

    /// An explicit scheduler work limit was reached.
    LimitExceeded {
        /// Stable name of the limit.
        limit: &'static str,
    },

    /// A deadline cannot be satisfied.
    DeadlineExceeded {
        /// Operation that caused the violation.
        operation: OperationId,
    },

    /// A model returned inconsistent predecessor information.
    InvalidDependencyModel {
        /// Operation whose dependency model is inconsistent.
        operation: OperationId,
    },
}

impl std::fmt::Display for ListSchedulingError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyProblem => {
                formatter.write_str("list scheduler received an empty problem")
            }

            Self::DuplicateOperation { operation } => {
                write!(formatter, "duplicate scheduling operation `{operation}`")
            }

            Self::UnknownOperation { operation } => {
                write!(formatter, "unknown scheduling operation `{operation}`")
            }

            Self::CycleDetected { operation } => {
                write!(
                    formatter,
                    "dependency cycle detected involving operation `{operation}`"
                )
            }

            Self::MissingDuration { operation } => {
                write!(
                    formatter,
                    "missing duration for scheduling operation `{operation}`"
                )
            }

            Self::InvalidTimingWindow { operation } => {
                write!(
                    formatter,
                    "invalid timing window for scheduling operation `{operation}`"
                )
            }

            Self::TimeOverflow { operation } => {
                write!(
                    formatter,
                    "schedule time overflow while placing operation `{operation}`"
                )
            }

            Self::UnschedulableResource { operation } => {
                write!(
                    formatter,
                    "operation `{operation}` cannot be placed on its required resources"
                )
            }

            Self::ReservationFailed { operation } => {
                write!(
                    formatter,
                    "resource reservation failed for operation `{operation}`"
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

            Self::LimitExceeded { limit } => {
                write!(formatter, "list scheduler limit exceeded: {limit}")
            }

            Self::DeadlineExceeded { operation } => {
                write!(
                    formatter,
                    "deadline exceeded while scheduling operation `{operation}`"
                )
            }

            Self::InvalidDependencyModel { operation } => {
                write!(
                    formatter,
                    "dependency model is inconsistent for operation `{operation}`"
                )
            }
        }
    }
}

impl std::error::Error for ListSchedulingError {}

/// Result returned by list-scheduling operations.
pub type ListSchedulingResult<T> = Result<T, ListSchedulingError>;

// ============================================================================
// Temporal bounds
// ============================================================================

/// Temporal restrictions for one operation.
///
/// All fields are optional because not every operation needs every temporal
/// restriction.
///
/// `earliest_start` and `latest_start` use the scheduler's abstract
/// `TimePoint`. The target timing adapter is responsible for interpreting that
/// coordinate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ListTimingBounds {
    /// Earliest legal start.
    pub earliest_start: Option<TimePoint>,

    /// Latest legal start.
    pub latest_start: Option<TimePoint>,

    /// Absolute latest legal completion.
    pub deadline: Option<TimePoint>,
}

impl ListTimingBounds {
    /// Creates unrestricted timing bounds.
    #[must_use]
    pub const fn unrestricted() -> Self {
        Self {
            earliest_start: None,
            latest_start: None,
            deadline: None,
        }
    }

    /// Validates the internal temporal ordering.
    #[must_use]
    pub fn is_valid(self) -> bool {
        match (self.earliest_start, self.latest_start) {
            (Some(earliest), Some(latest)) if earliest > latest => false,
            _ => true,
        }
    }
}

// ============================================================================
// Operation specification
// ============================================================================

/// Immutable information required by the list scheduler for one operation.
///
/// This is deliberately an algorithm-facing view rather than a replacement
/// for `quantum::ir::QuantumOperation`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ListOperation {
    /// Canonical operation identity.
    pub id: OperationId,

    /// Execution duration.
    pub duration: Duration,

    /// Temporal restrictions.
    pub timing: ListTimingBounds,

    /// Scheduler priority.
///
/// Larger values have higher priority.
    pub priority: i64,
}

impl ListOperation {
    /// Creates an operation specification.
    #[must_use]
    pub const fn new(
        id: OperationId,
        duration: Duration,
    ) -> Self {
        Self {
            id,
            duration,
            timing: ListTimingBounds::unrestricted(),
            priority: 0,
        }
    }

    /// Sets temporal bounds.
    #[must_use]
    pub const fn with_timing(
        mut self,
        timing: ListTimingBounds,
    ) -> Self {
        self.timing = timing;
        self
    }

    /// Sets the scheduling priority.
    #[must_use]
    pub const fn with_priority(
        mut self,
        priority: i64,
    ) -> Self {
        self.priority = priority;
        self
    }

    /// Returns the operation completion time if it starts at `start`.
    pub fn finish(
        self,
        start: TimePoint,
    ) -> ListSchedulingResult<TimePoint> {
        start
            .checked_add(self.duration)
            .ok_or(ListSchedulingError::TimeOverflow {
                operation: self.id,
            })
    }
}

// ============================================================================
// Dependency model
// ============================================================================

/// Dependency interface consumed by list scheduling.
///
/// Implementations normally delegate to:
///
/// `crate::quantum::scheduling::ir::DependencyGraph`.
///
/// The list algorithm intentionally does not depend on the graph's storage
/// representation.
pub trait ListDependencyModel {
    /// Returns all operations participating in the scheduling problem.
    fn operations(&self) -> &[ListOperation];

    /// Returns the direct predecessors of `operation`.
    fn predecessors(
        &self,
        operation: OperationId,
    ) -> ListSchedulingResult<Vec<OperationId>>;

    /// Returns the direct successors of `operation`.
    fn successors(
        &self,
        operation: OperationId,
    ) -> ListSchedulingResult<Vec<OperationId>>;
}

// ============================================================================
// Resource requirements
// ============================================================================

/// Amount of a resource required by an operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ListResourceRequirement {
    /// Resource identity.
    pub resource: ResourceId,

    /// Amount of capacity consumed.
    pub amount: u64,
}

impl ListResourceRequirement {
    /// Creates a resource requirement.
    ///
    /// A zero amount is accepted because the resource model may use zero
    /// requirements to represent conditional or advisory resources.
    #[must_use]
    pub const fn new(
        resource: ResourceId,
        amount: u64,
    ) -> Self {
        Self { resource, amount }
    }
}

/// Resource requirement provider.
///
/// The resource subsystem implements this boundary.
///
/// The scheduler never assumes how resources are represented internally.
pub trait ListResourceModel {
    /// Returns the resource requirements for an operation.
    fn requirements(
        &self,
        operation: OperationId,
    ) -> ListSchedulingResult<Vec<ListResourceRequirement>>;

    /// Returns the earliest time at which the requested resources can support
    /// the operation at or after `earliest`.
    ///
    /// Implementations may use calendars, interval trees, capacity profiles,
    /// target-specific availability, communication windows, or another
    /// internal representation.
    fn earliest_start(
        &self,
        operation: OperationId,
        earliest: TimePoint,
        duration: Duration,
    ) -> ListSchedulingResult<Option<TimePoint>>;

    /// Atomically reserves all resources required by an operation.
    ///
    /// The returned value is an opaque reservation token owned by the model.
    ///
    /// The token is retained by the scheduler so the resource model can keep
    /// the reservation alive for the resulting schedule.
    fn reserve(
        &mut self,
        operation: OperationId,
        start: TimePoint,
        duration: Duration,
    ) -> ListSchedulingResult<ListReservationToken>;
}

/// Opaque resource reservation token.
///
/// The token deliberately carries no resource implementation details.
///
/// The resource model may use the operation identity as the stable reservation
/// reference while the scheduler stores the token to keep the reservation
/// associated with the schedule.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ListReservationToken {
    operation: OperationId,
}

impl ListReservationToken {
    /// Creates a reservation token.
    #[must_use]
    pub const fn new(operation: OperationId) -> Self {
        Self { operation }
    }

    /// Returns the operation associated with the reservation.
    #[must_use]
    pub const fn operation(self) -> OperationId {
        self.operation
    }
}

// ============================================================================
// Candidate priority
// ============================================================================

/// Candidate ranking information.
///
/// This type is intentionally explicit so future priority policies can be
/// implemented without modifying the list-scheduling core.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ListPriority {
    /// Primary user/compiler priority.
    pub priority: i64,

    /// Critical-path or urgency score.
///
/// Larger values are more urgent.
    pub urgency: i64,

    /// Stable operation identity used as the final deterministic tie-break.
    pub operation: OperationId,
}

impl ListPriority {
    /// Creates a deterministic priority key.
    #[must_use]
    pub const fn new(
        priority: i64,
        urgency: i64,
        operation: OperationId,
    ) -> Self {
        Self {
            priority,
            urgency,
            operation,
        }
    }
}

impl Ord for ListPriority {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority
            .cmp(&other.priority)
            .then_with(|| self.urgency.cmp(&other.urgency))
            // Reverse the operation comparison so the BinaryHeap's largest
            // element is the lowest stable operation identity.
            //
            // This gives deterministic ascending OperationId selection when
            // priority and urgency are equal.
            .then_with(|| other.operation.cmp(&self.operation))
    }
}

impl PartialOrd for ListPriority {
    fn partial_cmp(
        &self,
        other: &Self,
    ) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// ============================================================================
// Priority provider
// ============================================================================

/// Supplies operation ranking to the list scheduler.
///
/// A critical-path implementation, deadline-aware implementation, fidelity
/// implementation, or user-defined policy can implement this trait.
pub trait ListPriorityModel {
    /// Returns the priority for a ready operation.
    fn priority(
        &self,
        operation: &ListOperation,
    ) -> ListSchedulingResult<ListPriority>;
}

/// Default deterministic priority model.
///
/// The operation's declared priority is primary and the operation identity is
/// the deterministic tie-break.
#[derive(Debug, Clone, Copy, Default)]
pub struct DefaultListPriority;

impl ListPriorityModel for DefaultListPriority {
    fn priority(
        &self,
        operation: &ListOperation,
    ) -> ListSchedulingResult<ListPriority> {
        Ok(ListPriority::new(
            operation.priority,
            0,
            operation.id,
        ))
    }
}

// ============================================================================
// Scheduled operation
// ============================================================================

/// One operation placement produced by list scheduling.
///
/// This is an algorithm result fragment.
///
/// `scheduling::result` remains responsible for constructing the final public
/// `ScheduleResult`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ListScheduledOperation {
    /// Operation identity.
    pub operation: OperationId,

    /// Scheduled start.
    pub start: TimePoint,

    /// Scheduled finish.
    pub finish: TimePoint,

    /// Reservation token returned by the resource model.
    pub reservation: ListReservationToken,
}

impl ListScheduledOperation {
    /// Returns the scheduled duration.
    #[must_use]
    pub fn duration(self) -> Option<Duration> {
        self.start.checked_duration_until(self.finish)
    }
}

// ============================================================================
// List schedule
// ============================================================================

/// Complete algorithm-local list schedule.
///
/// The final scheduling result adapter can transform this into
/// `crate::quantum::scheduling::result` without changing this algorithm.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListSchedule {
    /// Scheduled operations in deterministic emission order.
    operations: Vec<ListScheduledOperation>,

    /// Operation lookup table.
///
/// This makes dependency queries during integration and diagnostics efficient
/// without requiring the final result representation to use this storage.
    by_operation: BTreeMap<OperationId, usize>,

    /// Final schedule makespan.
    makespan: TimePoint,
}

impl ListSchedule {
    fn new() -> Self {
        Self {
            operations: Vec::new(),
            by_operation: BTreeMap::new(),
            makespan: TimePoint::ZERO,
        }
    }

    fn push(
        &mut self,
        scheduled: ListScheduledOperation,
    ) -> ListSchedulingResult<()> {
        if self.by_operation.contains_key(&scheduled.operation) {
            return Err(ListSchedulingError::DuplicateOperation {
                operation: scheduled.operation,
            });
        }

        let index = self.operations.len();

        self.operations.push(scheduled);
        self.by_operation
            .insert(scheduled.operation, index);

        if scheduled.finish > self.makespan {
            self.makespan = scheduled.finish;
        }

        Ok(())
    }

    /// Returns scheduled operations in deterministic scheduler emission order.
    #[must_use]
    pub fn operations(&self) -> &[ListScheduledOperation] {
        &self.operations
    }

    /// Returns the placement of an operation.
    #[must_use]
    pub fn operation(
        &self,
        operation: OperationId,
    ) -> Option<ListScheduledOperation> {
        self.by_operation
            .get(&operation)
            .and_then(|index| self.operations.get(*index))
            .copied()
    }

    /// Returns the final schedule makespan.
    #[must_use]
    pub const fn makespan(&self) -> TimePoint {
        self.makespan
    }

    /// Returns the number of scheduled operations.
    #[must_use]
    pub fn len(&self) -> usize {
        self.operations.len()
    }

    /// Returns whether the schedule is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.operations.is_empty()
    }
}

// ============================================================================
// Scheduler limits
// ============================================================================

/// Explicit work limits for one list-scheduling invocation.
///
/// Zero means that the corresponding limit is disabled.
///
/// These are algorithm-invocation limits, not machine-size limits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ListSchedulerLimits {
    /// Maximum number of scheduling decisions.
    pub max_decisions: u64,

    /// Maximum number of resource-placement attempts.
    pub max_resource_attempts: u64,
}

impl ListSchedulerLimits {
    /// Creates an unlimited list-scheduling limit configuration.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self {
            max_decisions: 0,
            max_resource_attempts: 0,
        }
    }

    fn decision_allowed(
        self,
        decisions: u64,
    ) -> bool {
        self.max_decisions == 0
            || decisions < self.max_decisions
    }

    fn resource_attempt_allowed(
        self,
        attempts: u64,
    ) -> bool {
        self.max_resource_attempts == 0
            || attempts < self.max_resource_attempts
    }
}

// ============================================================================
// Scheduler configuration
// ============================================================================

/// Configuration for the list scheduler.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ListSchedulerConfig {
    /// Schedule an empty problem as a valid zero-length schedule.
    pub allow_empty: bool,

    /// Initial scheduling origin.
    pub origin: TimePoint,

    /// Explicit work limits.
    pub limits: ListSchedulerLimits,
}

impl Default for ListSchedulerConfig {
    fn default() -> Self {
        Self {
            allow_empty: true,
            origin: TimePoint::ZERO,
            limits: ListSchedulerLimits::unlimited(),
        }
    }
}

impl ListSchedulerConfig {
    /// Creates the production default configuration.
    #[must_use]
    pub const fn production() -> Self {
        Self {
            allow_empty: true,
            origin: TimePoint::ZERO,
            limits: ListSchedulerLimits::unlimited(),
        }
    }

    /// Sets whether an empty problem is accepted.
    #[must_use]
    pub const fn with_allow_empty(
        mut self,
        allow: bool,
    ) -> Self {
        self.allow_empty = allow;
        self
    }

    /// Sets the scheduling origin.
    #[must_use]
    pub const fn with_origin(
        mut self,
        origin: TimePoint,
    ) -> Self {
        self.origin = origin;
        self
    }

    /// Sets explicit scheduler limits.
    #[must_use]
    pub const fn with_limits(
        mut self,
        limits: ListSchedulerLimits,
    ) -> Self {
        self.limits = limits;
        self
    }
}

// ============================================================================
// Candidate state
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ReadyCandidate {
    operation: OperationId,
    priority: ListPriority,
}

impl Ord for ReadyCandidate {
    fn cmp(
        &self,
        other: &Self,
    ) -> Ordering {
        self.priority.cmp(&other.priority)
    }
}

impl PartialOrd for ReadyCandidate {
    fn partial_cmp(
        &self,
        other: &Self,
    ) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// ============================================================================
// Scheduler
// ============================================================================

/// Production resource-aware list scheduler.
///
/// The scheduler owns only invocation-local algorithm state.
///
/// It does not own:
///
/// - hardware;
/// - canonical IR;
/// - routing;
/// - the global scheduler context;
/// - the dependency graph;
/// - the target resource definition.
#[derive(Debug, Clone, Copy)]
pub struct ListScheduler<P = DefaultListPriority> {
    config: ListSchedulerConfig,
    priority: P,
}

impl Default for ListScheduler<DefaultListPriority> {
    fn default() -> Self {
        Self::new()
    }
}

impl ListScheduler<DefaultListPriority> {
    /// Creates a production-default list scheduler.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            config: ListSchedulerConfig::production(),
            priority: DefaultListPriority,
        }
    }
}

impl<P> ListScheduler<P>
where
    P: ListPriorityModel,
{
    /// Creates a list scheduler with an explicit configuration and priority
    /// model.
    #[must_use]
    pub const fn with_priority(
        config: ListSchedulerConfig,
        priority: P,
    ) -> Self {
        Self {
            config,
            priority,
        }
    }

    /// Returns the scheduler configuration.
    #[must_use]
    pub const fn config(&self) -> ListSchedulerConfig {
        self.config
    }

    /// Returns the priority model.
    #[must_use]
    pub const fn priority_model(&self) -> &P {
        &self.priority
    }

    /// Schedules a complete dependency/resource problem.
    ///
    /// This is the main algorithm entry point.
    ///
    /// # Algorithm
    ///
    /// The scheduler first validates the operation/dependency universe, then
    /// computes predecessor counts. Operations with zero unsatisfied
    /// predecessors enter the ready queue.
    ///
    /// Each iteration:
    ///
    /// 1. obtains the highest-priority ready operation;
    /// 2. computes dependency readiness;
    /// 3. applies the operation's temporal lower bound;
    /// 4. asks the resource model for the earliest feasible placement;
    /// 5. checks the timing window/deadline;
    /// 6. atomically reserves resources;
    /// 7. emits the scheduled operation;
    /// 8. decrements successor predecessor counts;
    /// 9. adds newly-ready successors.
    ///
    /// The algorithm never scans all operations to discover readiness after
    /// each placement. Only successors of the completed operation are touched.
    pub fn schedule<D, R>(
        &self,
        dependencies: &D,
        resources: &mut R,
    ) -> ListSchedulingResult<ListSchedule>
    where
        D: ListDependencyModel,
        R: ListResourceModel,
    {
        let operations = dependencies.operations();

        if operations.is_empty() {
            if self.config.allow_empty {
                return Ok(ListSchedule::new());
            }

            return Err(ListSchedulingError::EmptyProblem);
        }

        let operation_map = self.validate_operations(operations)?;

        let mut remaining_predecessors =
            self.build_predecessor_counts(
                dependencies,
                &operation_map,
            )?;

        let mut finish_times =
            BTreeMap::<OperationId, TimePoint>::new();

        let mut ready =
            BinaryHeap::<ReadyCandidate>::new();

        self.seed_ready_queue(
            operations,
            &remaining_predecessors,
            &mut ready,
        )?;

        let mut scheduled =
            BTreeSet::<OperationId>::new();

        let mut schedule = ListSchedule::new();

        let mut decisions = 0_u64;
        let mut resource_attempts = 0_u64;

        while scheduled.len() < operations.len() {
            if !self
                .config
                .limits
                .decision_allowed(decisions)
            {
                return Err(ListSchedulingError::LimitExceeded {
                    limit: "max_decisions",
                });
            }

            let candidate = match ready.pop() {
                Some(candidate) => candidate,
                None => {
                    // If operations remain but no operation is ready, the
                    // graph has a cycle or the dependency model has become
                    // inconsistent.
                    let operation = operations
                        .iter()
                        .map(|operation| operation.id)
                        .find(|operation| {
                            !scheduled.contains(operation)
                        })
                        .ok_or(
                            ListSchedulingError::InvalidDependencyModel {
                                operation: OperationId::from(0),
                            },
                        )?;

                    return Err(
                        ListSchedulingError::CycleDetected {
                            operation,
                        },
                    );
                }
            };

            let operation = operation_map
                .get(&candidate.operation)
                .copied()
                .ok_or(
                    ListSchedulingError::UnknownOperation {
                        operation: candidate.operation,
                    },
                )?;

            if scheduled.contains(&operation.id) {
                continue;
            }

            decisions = decisions
                .checked_add(1)
                .ok_or(ListSchedulingError::LimitExceeded {
                    limit: "decision-counter-overflow",
                })?;

            let dependency_ready =
                self.dependency_ready_time(
                    dependencies,
                    operation.id,
                    &finish_times,
                )?;

            let earliest = max_time(
                self.config.origin,
                dependency_ready,
            );

            let earliest = max_optional_time(
                earliest,
                operation.timing.earliest_start,
            );

            if let Some(latest_start) =
                operation.timing.latest_start
            {
                if earliest > latest_start {
                    // The resource model may still be able to identify an
                    // earlier resource placement only if the current
                    // dependency/timing lower bound was itself wrong.
                    //
                    // It is therefore a genuine unschedulable timing window.
                    return Err(
                        ListSchedulingError::InvalidTimingWindow {
                            operation: operation.id,
                        },
                    );
                }
            }

            if !self
                .config
                .limits
                .resource_attempt_allowed(resource_attempts)
            {
                return Err(ListSchedulingError::LimitExceeded {
                    limit: "max_resource_attempts",
                });
            }

            resource_attempts = resource_attempts
                .checked_add(1)
                .ok_or(ListSchedulingError::LimitExceeded {
                    limit: "resource-attempt-counter-overflow",
                })?;

            let resource_start =
                resources
                    .earliest_start(
                        operation.id,
                        earliest,
                        operation.duration,
                    )?
                    .ok_or(
                        ListSchedulingError::UnschedulableResource {
                            operation: operation.id,
                        },
                    )?;

            let start =
                max_time(earliest, resource_start);

            if let Some(latest_start) =
                operation.timing.latest_start
            {
                if start > latest_start {
                    return Err(
                        ListSchedulingError::InvalidTimingWindow {
                            operation: operation.id,
                        },
                    );
                }
            }

            let finish =
                operation.finish(start)?;

            if let Some(deadline) =
                operation.timing.deadline
            {
                if finish > deadline {
                    return Err(
                        ListSchedulingError::DeadlineExceeded {
                            operation: operation.id,
                        },
                    );
                }
            }

            let reservation =
                resources
                    .reserve(
                        operation.id,
                        start,
                        operation.duration,
                    )?;

            if reservation.operation()
                != operation.id
            {
                return Err(
                    ListSchedulingError::ReservationFailed {
                        operation: operation.id,
                    },
                );
            }

            schedule.push(
                ListScheduledOperation {
                    operation: operation.id,
                    start,
                    finish,
                    reservation,
                },
            )?;

            scheduled.insert(operation.id);
            finish_times.insert(operation.id, finish);

            let successors =
                dependencies.successors(operation.id)?;

            for successor in successors {
                let count =
                    remaining_predecessors
                        .get_mut(&successor)
                        .ok_or(
                            ListSchedulingError::UnknownOperation {
                                operation: successor,
                            },
                        )?;

                if *count == 0 {
                    return Err(
                        ListSchedulingError::InvalidDependencyModel {
                            operation: successor,
                        },
                    );
                }

                *count -= 1;

                if *count == 0 {
                    let successor_operation =
                        operation_map
                            .get(&successor)
                            .copied()
                            .ok_or(
                                ListSchedulingError::UnknownOperation {
                                    operation: successor,
                                },
                            )?;

                    let priority =
                        self.priority
                            .priority(successor_operation)?;

                    ready.push(
                        ReadyCandidate {
                            operation: successor,
                            priority,
                        },
                    );
                }
            }
        }

        if schedule.len() != operations.len() {
            let operation = operations
                .iter()
                .map(|operation| operation.id)
                .find(|operation| !scheduled.contains(operation))
                .ok_or(
                    ListSchedulingError::InvalidDependencyModel {
                        operation: OperationId::from(0),
                    },
                )?;

            return Err(
                ListSchedulingError::CycleDetected {
                    operation,
                },
            );
        }

        Ok(schedule)
    }

    fn validate_operations(
        &self,
        operations: &[ListOperation],
    ) -> ListSchedulingResult<BTreeMap<OperationId, ListOperation>>
    {
        let mut map = BTreeMap::new();

        for operation in operations {
            if !operation.timing.is_valid() {
                return Err(
                    ListSchedulingError::InvalidTimingWindow {
                        operation: operation.id,
                    },
                );
            }

            if map.insert(operation.id, *operation).is_some() {
                return Err(
                    ListSchedulingError::DuplicateOperation {
                        operation: operation.id,
                    },
                );
            }
        }

        Ok(map)
    }

    fn build_predecessor_counts<D>(
        &self,
        dependencies: &D,
        operations: &BTreeMap<
            OperationId,
            ListOperation,
        >,
    ) -> ListSchedulingResult<BTreeMap<OperationId, usize>>
    where
        D: ListDependencyModel,
    {
        let mut counts = BTreeMap::new();

        for operation in operations.keys().copied() {
            let predecessors =
                dependencies.predecessors(operation)?;

            let mut unique =
                BTreeSet::new();

            for predecessor in predecessors {
                if !operations.contains_key(&predecessor) {
                    return Err(
                        ListSchedulingError::UnknownOperation {
                            operation: predecessor,
                        },
                    );
                }

                if predecessor == operation {
                    return Err(
                        ListSchedulingError::CycleDetected {
                            operation,
                        },
                    );
                }

                unique.insert(predecessor);
            }

            counts.insert(
                operation,
                unique.len(),
            );
        }

        Ok(counts)
    }

    fn seed_ready_queue(
        &self,
        operations: &[ListOperation],
        remaining: &BTreeMap<OperationId, usize>,
        ready: &mut BinaryHeap<ReadyCandidate>,
    ) -> ListSchedulingResult<()>
    {
        for operation in operations {
            if remaining
                .get(&operation.id)
                .copied()
                .ok_or(
                    ListSchedulingError::InvalidDependencyModel {
                        operation: operation.id,
                    },
                )?
                == 0
            {
                let priority =
                    self.priority.priority(operation)?;

                ready.push(
                    ReadyCandidate {
                        operation: operation.id,
                        priority,
                    },
                );
            }
        }

        Ok(())
    }

    fn dependency_ready_time<D>(
        &self,
        dependencies: &D,
        operation: OperationId,
        finish_times: &BTreeMap<
            OperationId,
            TimePoint,
        >,
    ) -> ListSchedulingResult<TimePoint>
    where
        D: ListDependencyModel,
    {
        let predecessors =
            dependencies.predecessors(operation)?;

        let mut ready =
            self.config.origin;

        for predecessor in predecessors {
            let finish =
                finish_times
                    .get(&predecessor)
                    .copied()
                    .ok_or(
                        ListSchedulingError::InvalidDependencyModel {
                            operation,
                        },
                    )?;

            ready = max_time(ready, finish);
        }

        Ok(ready)
    }
}

// ============================================================================
// Utility functions
// ============================================================================

fn max_time(
    left: TimePoint,
    right: TimePoint,
) -> TimePoint {
    if left >= right {
        left
    } else {
        right
    }
}

fn max_optional_time(
    current: TimePoint,
    candidate: Option<TimePoint>,
) -> TimePoint {
    match candidate {
        Some(candidate) => max_time(current, candidate),
        None => current,
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct TestDependencies {
        operations: Vec<ListOperation>,
        predecessors: BTreeMap<
            OperationId,
            Vec<OperationId>,
        >,
        successors: BTreeMap<
            OperationId,
            Vec<OperationId>,
        >,
    }

    impl TestDependencies {
        fn new(
            operations: Vec<ListOperation>,
        ) -> Self {
            let mut predecessors =
                BTreeMap::new();
            let mut successors =
                BTreeMap::new();

            for operation in &operations {
                predecessors
                    .insert(operation.id, Vec::new());

                successors
                    .insert(operation.id, Vec::new());
            }

            Self {
                operations,
                predecessors,
                successors,
            }
        }

        fn edge(
            &mut self,
            from: OperationId,
            to: OperationId,
        ) {
            self.successors
                .get_mut(&from)
                .expect("source exists")
                .push(to);

            self.predecessors
                .get_mut(&to)
                .expect("destination exists")
                .push(from);
        }
    }

    impl ListDependencyModel for TestDependencies {
        fn operations(&self) -> &[ListOperation] {
            &self.operations
        }

        fn predecessors(
            &self,
            operation: OperationId,
        ) -> ListSchedulingResult<Vec<OperationId>> {
            self.predecessors
                .get(&operation)
                .cloned()
                .ok_or(
                    ListSchedulingError::UnknownOperation {
                        operation,
                    },
                )
        }

        fn successors(
            &self,
            operation: OperationId,
        ) -> ListSchedulingResult<Vec<OperationId>> {
            self.successors
                .get(&operation)
                .cloned()
                .ok_or(
                    ListSchedulingError::UnknownOperation {
                        operation,
                    },
                )
        }
    }

    #[derive(Debug, Default)]
    struct TestResources {
        next_available: BTreeMap<
            ResourceId,
            TimePoint,
        >,
    }

    impl ListResourceModel for TestResources {
        fn requirements(
            &self,
            _operation: OperationId,
        ) -> ListSchedulingResult<
            Vec<ListResourceRequirement>,
        > {
            Ok(Vec::new())
        }

        fn earliest_start(
            &self,
            _operation: OperationId,
            earliest: TimePoint,
            _duration: Duration,
        ) -> ListSchedulingResult<Option<TimePoint>> {
            Ok(Some(earliest))
        }

        fn reserve(
            &mut self,
            operation: OperationId,
            _start: TimePoint,
            _duration: Duration,
        ) -> ListSchedulingResult<ListReservationToken> {
            Ok(ListReservationToken::new(operation))
        }
    }

    fn operation(
        value: u64,
        duration: u128,
    ) -> ListOperation {
        ListOperation::new(
            OperationId::from(value),
            Duration::new(duration),
        )
    }

    #[test]
    fn empty_problem_is_valid_by_default() {
        let dependencies =
            TestDependencies::new(Vec::new());

        let mut resources =
            TestResources::default();

        let scheduler =
            ListScheduler::new();

        let schedule =
            scheduler
                .schedule(
                    &dependencies,
                    &mut resources,
                )
                .expect("empty problem should succeed");

        assert!(schedule.is_empty());
        assert_eq!(
            schedule.makespan(),
            TimePoint::ZERO
        );
    }

    #[test]
    fn independent_operations_are_both_scheduled() {
        let dependencies =
            TestDependencies::new(vec![
                operation(1, 10),
                operation(2, 20),
            ]);

        let mut resources =
            TestResources::default();

        let schedule =
            ListScheduler::new()
                .schedule(
                    &dependencies,
                    &mut resources,
                )
                .expect("independent operations should schedule");

        assert_eq!(schedule.len(), 2);
    }

    #[test]
    fn dependency_forces_successor_after_predecessor() {
        let mut dependencies =
            TestDependencies::new(vec![
                operation(1, 10),
                operation(2, 20),
            ]);

        dependencies.edge(
            OperationId::from(1),
            OperationId::from(2),
        );

        let mut resources =
            TestResources::default();

        let schedule =
            ListScheduler::new()
                .schedule(
                    &dependencies,
                    &mut resources,
                )
                .expect("dependency chain should schedule");

        let first = schedule
            .operation(OperationId::from(1))
            .expect("first operation");

        let second = schedule
            .operation(OperationId::from(2))
            .expect("second operation");

        assert!(second.start >= first.finish);
    }

    #[test]
    fn cycle_is_rejected() {
        let mut dependencies =
            TestDependencies::new(vec![
                operation(1, 10),
                operation(2, 20),
            ]);

        dependencies.edge(
            OperationId::from(1),
            OperationId::from(2),
        );

        dependencies.edge(
            OperationId::from(2),
            OperationId::from(1),
        );

        let mut resources =
            TestResources::default();

        let result =
            ListScheduler::new()
                .schedule(
                    &dependencies,
                    &mut resources,
                );

        assert!(matches!(
            result,
            Err(
                ListSchedulingError::CycleDetected {
                    ..
                }
            )
        ));
    }

    #[test]
    fn deadline_is_enforced() {
        let mut dependencies =
            TestDependencies::new(vec![
                ListOperation::new(
                    OperationId::from(1),
                    Duration::new(10),
                )
                .with_timing(
                    ListTimingBounds {
                        earliest_start: None,
                        latest_start: None,
                        deadline: Some(
                            TimePoint::new(5),
                        ),
                    },
                ),
            ]);

        let mut resources =
            TestResources::default();

        let result =
            ListScheduler::new()
                .schedule(
                    &mut dependencies,
                    &mut resources,
                );

        assert!(matches!(
            result,
            Err(
                ListSchedulingError::DeadlineExceeded {
                    operation
                }
            ) if operation == OperationId::from(1)
        ));
    }

    #[test]
    fn priority_is_deterministic() {
        let dependencies =
            TestDependencies::new(vec![
                operation(1, 10)
                    .with_priority(1),
                operation(2, 10)
                    .with_priority(2),
            ]);

        let mut resources =
            TestResources::default();

        let schedule =
            ListScheduler::new()
                .schedule(
                    &dependencies,
                    &mut resources,
                )
                .expect("priority scheduling should succeed");

        assert_eq!(
            schedule
                .operations()
                .first()
                .expect("first operation")
                .operation,
            OperationId::from(2)
        );
    }

    #[test]
    fn same_priority_uses_stable_operation_identity() {
        let dependencies =
            TestDependencies::new(vec![
                operation(2, 10),
                operation(1, 10),
            ]);

        let mut resources =
            TestResources::default();

        let schedule =
            ListScheduler::new()
                .schedule(
                    &dependencies,
                    &mut resources,
                )
                .expect("deterministic scheduling should succeed");

        assert_eq!(
            schedule
                .operations()
                .first()
                .expect("first operation")
                .operation,
            OperationId::from(1)
        );
    }

    #[test]
    fn temporal_lower_bound_is_respected() {
        let dependencies =
            TestDependencies::new(vec![
                ListOperation::new(
                    OperationId::from(1),
                    Duration::new(10),
                )
                .with_timing(
                    ListTimingBounds {
                        earliest_start: Some(
                            TimePoint::new(100),
                        ),
                        latest_start: None,
                        deadline: None,
                    },
                ),
            ]);

        let mut resources =
            TestResources::default();

        let schedule =
            ListScheduler::new()
                .schedule(
                    &dependencies,
                    &mut resources,
                )
                .expect("temporal lower bound should succeed");

        assert_eq!(
            schedule
                .operation(OperationId::from(1))
                .expect("operation")
                .start,
            TimePoint::new(100)
        );
    }
}