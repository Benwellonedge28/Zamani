//! Zamani Quantum Scheduling — Dynamic Runtime Scheduling
//!
//! Production-grade runtime event/state modelling for dynamic quantum
//! scheduling.
//!
//! # Architectural role
//!
//! This module is the scheduler-facing runtime state machine for computations
//! whose complete schedule cannot be determined statically.
//!
//! It bridges:
//!
//! ```text
//! static schedule
//!       |
//!       v
//! runtime event
//!       |
//!       +--------------------+
//!       |                    |
//!       v                    v
//! measurement          classical result
//!       |                    |
//!       +---------+----------+
//!                 |
//!                 v
//!            feedback
//!                 |
//!                 v
//!          conditional work
//!                 |
//!                 v
//!          runtime release
//! ```
//!
//! # Responsibilities
//!
//! This module owns:
//!
//! - runtime scheduling epochs;
//! - scheduler-visible runtime events;
//! - event readiness;
//! - event dependencies;
//! - runtime release conditions;
//! - incremental schedule state;
//! - runtime operation state;
//! - runtime resource-independent eligibility;
//! - cancellation and invalidation state;
//! - event acknowledgement/consumption;
//! - deterministic event ordering;
//! - runtime state snapshots;
//! - checked temporal arithmetic;
//! - bounded-memory state compaction where explicitly requested;
//! - integration contracts for classical, conditional and feedback scheduling.
//!
//! # Non-responsibilities
//!
//! This module does NOT:
//!
//! - execute quantum operations;
//! - execute classical programs;
//! - communicate directly with hardware;
//! - perform logical-to-physical routing;
//! - allocate physical qubits;
//! - own hardware resource calendars;
//! - define quantum operation semantics;
//! - define another `QubitId`;
//! - define another `PhysicalQubitId`;
//! - evaluate arbitrary classical expressions;
//! - decode QEC;
//! - choose the global scheduling algorithm;
//! - discover hardware;
//! - authenticate against a QPU;
//! - perform pulse generation;
//! - replace `dynamic::classical`;
//! - replace `dynamic::conditional`;
//! - replace `dynamic::feedback`.
//!
//! The runtime integration layer records facts supplied by those subsystems.
//!
//! # Canonical quantum identity
//!
//! Quantum identities MUST come from:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! This module never defines replacement qubit identities.
//!
//! # Canonical scheduling time
//!
//! Runtime scheduling uses:
//!
//! ```text
//! crate::quantum::scheduling::types::TimePoint
//! crate::quantum::scheduling::types::Duration
//! ```
//!
//! No nanosecond, microsecond, device-tick, pulse-sample or other hardware
//! unit is embedded here.
//!
//! # Dynamic scheduling model
//!
//! A runtime-dependent operation follows this general lifecycle:
//!
//! ```text
//! Planned
//!    |
//!    v
//! WaitingForDependencies
//!    |
//!    v
//! Eligible
//!    |
//!    v
//! Released
//!    |
//!    v
//! Executing
//!    |
//!    v
//! Completed
//! ```
//!
//! Exceptional states are:
//!
//! ```text
//! Cancelled
//! Failed
//! Invalidated
//! Expired
//! ```
//!
//! The runtime scheduler does not perform the actual execution transition.
//! An executor reports runtime observations back to this state machine.
//!
//! # Scalability
//!
//! There are no fixed limits on:
//!
//! - runtime events;
//! - operations;
//! - dependencies;
//! - branches;
//! - feedback chains;
//! - classical signals;
//! - qubits;
//! - runtime epochs;
//! - distributed endpoints;
//! - event generations.
//!
//! "Infinity" means that this module introduces no artificial finite machine
//! size. A real execution is necessarily bounded by available memory, CPU,
//! operating-system resources, explicit policy limits and target resources.
//!
//! Runtime state is stored sparsely. The implementation does not create a
//! time-slot matrix proportional to qubit count × execution duration.
//!
//! # Determinism
//!
//! Public event collections use ordered maps/sets where deterministic ordering
//! is semantically useful.
//!
//! No wall clock is consulted.
//! No implicit randomness is used.
//! No global mutable state is used.
//!
//! # Thread safety
//!
//! The core structures contain ordinary owned values and no interior
//! mutability. They can therefore be transferred between threads and embedded
//! in thread-safe runtime components.
//!
//! Concurrent mutation itself is intentionally owned by the containing runtime
//! or executor rather than hidden inside this module.
//!
//! # Safety
//!
//! Rust 1.97 / Rust 1.97.1.
//! Rust 2021.
//! Stable Rust.
//! No nightly features.
//! No unsafe code.
//!
//! `#![forbid(unsafe_code)]` makes the no-unsafe requirement compiler-enforced.
//!
//! # Integration
//!
//! ```text
//! quantum::ir
//!      |
//!      v
//! scheduling::dynamic::classical
//!      |
//!      v
//! scheduling::dynamic::conditional
//!      |
//!      v
//! scheduling::dynamic::feedback
//!      |
//!      v
//! scheduling::dynamic::runtime
//!      |
//!      +-----------------------+
//!      |                       |
//!      v                       v
//! scheduling::planners     runtime/executor
//!      |                       |
//!      v                       v
//! scheduling::verification   hardware
//! ```
//!
//! The runtime layer receives:
//!
//! - planned operations from scheduling;
//! - classical readiness from `dynamic::classical`;
//! - conditional eligibility from `dynamic::conditional`;
//! - feedback readiness from `dynamic::feedback`;
//! - observed execution events from the runtime/hardware boundary.
//!
//! It returns scheduler-visible runtime state.
//!
//! # Important ownership rule
//!
//! A runtime event is an observation or scheduling fact.
//!
//! It is NOT permission to directly execute hardware.
//!
//! The hardware/runtime subsystem remains responsible for execution.
//!
//! ============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fmt;

use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};

use super::classical::{
    ClassicalEventId,
    ClassicalNodeId,
    ClassicalSignalId,
    PredicateId,
};
use super::feedback::FeedbackId;
use super::conditional::BranchId;

use crate::quantum::ir::core::identity::OperationId;
use crate::quantum::scheduling::types::{Duration, TimePoint};

// ============================================================================
// Runtime event identity
// ============================================================================

/// Stable identity for one scheduler-visible runtime event.
///
/// This is deliberately distinct from:
///
/// - `OperationId`;
/// - `ClassicalEventId`;
/// - `FeedbackId`;
/// - `BranchId`.
///
/// Each subsystem owns its own semantic identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RuntimeEventId(u64);

impl RuntimeEventId {
    /// Creates a runtime event identity.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the numeric identity.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }

    /// Returns whether this is the zero identity.
    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.0 == 0
    }

    /// Returns the next representable identity.
    #[must_use]
    pub const fn checked_next(self) -> Option<Self> {
        match self.0.checked_add(1) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }
}

impl From<u64> for RuntimeEventId {
    fn from(value: u64) -> Self {
        Self::new(value)
    }
}

impl From<RuntimeEventId> for u64 {
    fn from(value: RuntimeEventId) -> Self {
        value.value()
    }
}

impl fmt::Display for RuntimeEventId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "runtime-event:{}", self.0)
    }
}

// ============================================================================
// Runtime epoch
// ============================================================================

/// Runtime scheduling epoch.
///
/// An epoch represents a coherent runtime observation/planning state.
///
/// It does not change canonical operation identities.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RuntimeEpochId(u64);

impl RuntimeEpochId {
    /// Creates an epoch identity.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the numeric identity.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }

    /// Returns whether this is the zero epoch.
    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.0 == 0
    }

    /// Returns the next epoch.
    #[must_use]
    pub const fn checked_next(self) -> Option<Self> {
        match self.0.checked_add(1) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }
}

impl From<u64> for RuntimeEpochId {
    fn from(value: u64) -> Self {
        Self::new(value)
    }
}

impl From<RuntimeEpochId> for u64 {
    fn from(value: RuntimeEpochId) -> Self {
        value.value()
    }
}

impl fmt::Display for RuntimeEpochId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "runtime-epoch:{}", self.0)
    }
}

// ============================================================================
// Runtime operation state
// ============================================================================

/// Lifecycle state of a runtime-controlled operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[non_exhaustive]
pub enum RuntimeOperationState {
    /// Operation exists in the planned schedule.
    Planned,

    /// Operation is waiting for runtime dependencies.
    WaitingForDependencies,

    /// All runtime conditions required for release are satisfied.
    Eligible,

    /// Runtime has released the operation to the executor.
    Released,

    /// Executor reported that execution has started.
    Executing,

    /// Executor reported successful completion.
    Completed,

    /// Operation was explicitly cancelled.
    Cancelled,

    /// Operation failed during execution.
    Failed,

    /// Operation was invalidated by a newer runtime state.
    Invalidated,

    /// Operation missed a runtime validity window.
    Expired,
}

impl RuntimeOperationState {
    /// Returns whether the operation can be released.
    #[must_use]
    pub const fn can_release(self) -> bool {
        matches!(self, Self::Eligible)
    }

    /// Returns whether the operation is terminal.
    #[must_use]
    pub const fn is_terminal(self) -> bool {
        matches!(
            self,
            Self::Completed
                | Self::Cancelled
                | Self::Failed
                | Self::Invalidated
                | Self::Expired
        )
    }

    /// Returns whether the operation is currently waiting.
    #[must_use]
    pub const fn is_waiting(self) -> bool {
        matches!(
            self,
            Self::Planned | Self::WaitingForDependencies
        )
    }
}

// ============================================================================
// Runtime event kind
// ============================================================================

/// Kind of runtime event observed or generated by the scheduler.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[non_exhaustive]
pub enum RuntimeEventKind {
    /// A quantum operation became available for runtime processing.
    OperationAvailable {
        /// Canonical operation identity.
        operation: OperationId,
    },

    /// A quantum operation was released to the executor.
    OperationReleased {
        /// Canonical operation identity.
        operation: OperationId,
    },

    /// A quantum operation started executing.
    OperationStarted {
        /// Canonical operation identity.
        operation: OperationId,
    },

    /// A quantum operation completed.
    OperationCompleted {
        /// Canonical operation identity.
        operation: OperationId,
    },

    /// A quantum operation failed.
    OperationFailed {
        /// Canonical operation identity.
        operation: OperationId,
    },

    /// A classical signal became available.
    ClassicalSignalReady {
        /// Available signal.
        signal: ClassicalSignalId,
    },

    /// A classical computation completed.
    ClassicalComputationCompleted {
        /// Completed computation node.
        node: ClassicalNodeId,
    },

    /// A classical predicate became evaluable.
    PredicateReady {
        /// Predicate identity.
        predicate: PredicateId,
    },

    /// A feedback dependency became ready.
    FeedbackReady {
        /// Feedback identity.
        feedback: FeedbackId,
    },

    /// A branch became eligible.
    BranchReady {
        /// Branch identity.
        branch: BranchId,
    },

    /// A measurement produced a runtime event.
    MeasurementReady {
        /// Classical event associated with the measurement.
        event: ClassicalEventId,

        /// Logical qubits associated with the measurement.
        logical_qubits: Vec<QubitId>,

        /// Physical qubits when routing is known.
        physical_qubits: Vec<PhysicalQubitId>,
    },

    /// An external runtime input became available.
    ExternalInputReady,

    /// A distributed communication event completed.
    CommunicationCompleted {
        /// Runtime communication identity.
        communication: u64,
    },

    /// A runtime scheduling epoch changed.
    EpochAdvanced {
        /// New epoch.
        epoch: RuntimeEpochId,
    },

    /// A runtime cancellation was observed.
    CancellationRequested,

    /// A runtime invalidation was observed.
    InvalidationRequested,

    /// A plugin-defined runtime event.
    Custom(String),
}

impl RuntimeEventKind {
    /// Returns the canonical operation associated with this event, if any.
    #[must_use]
    pub const fn operation(&self) -> Option<OperationId> {
        match self {
            Self::OperationAvailable { operation }
            | Self::OperationReleased { operation }
            | Self::OperationStarted { operation }
            | Self::OperationCompleted { operation }
            | Self::OperationFailed { operation } => Some(*operation),

            _ => None,
        }
    }

    /// Returns whether the event changes operation eligibility.
    #[must_use]
    pub const fn affects_eligibility(&self) -> bool {
        matches!(
            self,
            Self::ClassicalSignalReady { .. }
                | Self::ClassicalComputationCompleted { .. }
                | Self::PredicateReady { .. }
                | Self::FeedbackReady { .. }
                | Self::BranchReady { .. }
                | Self::MeasurementReady { .. }
                | Self::ExternalInputReady
                | Self::CommunicationCompleted { .. }
        )
    }
}

// ============================================================================
// Runtime dependency
// ============================================================================

/// A prerequisite that must become true before a runtime operation can be
/// released.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[non_exhaustive]
pub enum RuntimeDependency {
    /// Depends on another canonical quantum operation completing.
    OperationCompleted(OperationId),

    /// Depends on a classical signal becoming available.
    ClassicalSignal(ClassicalSignalId),

    /// Depends on a classical computation completing.
    ClassicalNode(ClassicalNodeId),

    /// Depends on a predicate becoming evaluable.
    Predicate(PredicateId),

    /// Depends on feedback becoming ready.
    Feedback(FeedbackId),

    /// Depends on a branch becoming ready.
    Branch(BranchId),

    /// Depends on a runtime event.
    Event(RuntimeEventId),

    /// Depends on a classical runtime event.
    ClassicalEvent(ClassicalEventId),

    /// Depends on a distributed communication event.
    Communication(u64),

    /// Plugin-defined dependency.
    Custom(String),
}

impl RuntimeDependency {
    /// Returns whether this dependency is satisfied by an event.
    #[must_use]
    pub fn is_satisfied_by(
        &self,
        event: &RuntimeEventKind,
    ) -> bool {
        match (self, event) {
            (
                Self::OperationCompleted(required),
                RuntimeEventKind::OperationCompleted { operation },
            ) => required == operation,

            (
                Self::ClassicalSignal(required),
                RuntimeEventKind::ClassicalSignalReady { signal },
            ) => required == signal,

            (
                Self::ClassicalNode(required),
                RuntimeEventKind::ClassicalComputationCompleted { node },
            ) => required == node,

            (
                Self::Predicate(required),
                RuntimeEventKind::PredicateReady { predicate },
            ) => required == predicate,

            (
                Self::Feedback(required),
                RuntimeEventKind::FeedbackReady { feedback },
            ) => required == feedback,

            (
                Self::Branch(required),
                RuntimeEventKind::BranchReady { branch },
            ) => required == branch,

            (
                Self::ClassicalEvent(required),
                RuntimeEventKind::MeasurementReady { event, .. },
            ) => required == event,

            (
                Self::Communication(required),
                RuntimeEventKind::CommunicationCompleted {
                    communication,
                },
            ) => required == communication,

            _ => false,
        }
    }
}

// ============================================================================
// Runtime event status
// ============================================================================

/// State of a runtime event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[non_exhaustive]
pub enum RuntimeEventState {
    /// Event is scheduled but not yet eligible.
    Pending,

    /// Event is eligible for processing.
    Ready,

    /// Event has been delivered to its consumer.
    Delivered,

    /// Event has been acknowledged by its consumer.
    Acknowledged,

    /// Event was cancelled.
    Cancelled,

    /// Event was invalidated.
    Invalidated,

    /// Event failed.
    Failed,
}

impl RuntimeEventState {
    /// Returns whether this event can currently be delivered.
    #[must_use]
    pub const fn is_deliverable(self) -> bool {
        matches!(self, Self::Ready)
    }

    /// Returns whether this event is terminal.
    #[must_use]
    pub const fn is_terminal(self) -> bool {
        matches!(
            self,
            Self::Acknowledged
                | Self::Cancelled
                | Self::Invalidated
                | Self::Failed
        )
    }
}

// ============================================================================
// Runtime event
// ============================================================================

/// One scheduler-visible runtime event.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeEvent {
    id: RuntimeEventId,
    epoch: RuntimeEpochId,
    kind: RuntimeEventKind,
    state: RuntimeEventState,

    /// Time at which the event became observable to the scheduler.
    observed_at: TimePoint,

    /// Earliest time at which the event may be consumed.
    available_at: TimePoint,

    /// Optional expiration time.
    expires_at: Option<TimePoint>,

    /// Runtime dependencies of the event itself.
    dependencies: BTreeSet<RuntimeDependency>,

    /// Canonical operation associated with the event, when applicable.
    operation: Option<OperationId>,

    /// Optional logical qubit context.
    logical_qubits: BTreeSet<QubitId>,

    /// Optional physical qubit context.
    physical_qubits: BTreeSet<PhysicalQubitId>,

    /// Optional diagnostic/provenance label.
    label: Option<String>,
}

impl RuntimeEvent {
    /// Creates a runtime event.
    #[must_use]
    pub fn new(
        id: RuntimeEventId,
        epoch: RuntimeEpochId,
        kind: RuntimeEventKind,
        observed_at: TimePoint,
        available_at: TimePoint,
    ) -> Self {
        let operation = kind.operation();

        Self {
            id,
            epoch,
            kind,
            state: RuntimeEventState::Pending,
            observed_at,
            available_at,
            expires_at: None,
            dependencies: BTreeSet::new(),
            operation,
            logical_qubits: BTreeSet::new(),
            physical_qubits: BTreeSet::new(),
            label: None,
        }
    }

    /// Returns the event identity.
    #[must_use]
    pub const fn id(&self) -> RuntimeEventId {
        self.id
    }

    /// Returns the runtime epoch.
    #[must_use]
    pub const fn epoch(&self) -> RuntimeEpochId {
        self.epoch
    }

    /// Returns the event kind.
    #[must_use]
    pub fn kind(&self) -> &RuntimeEventKind {
        &self.kind
    }

    /// Returns the event state.
    #[must_use]
    pub const fn state(&self) -> RuntimeEventState {
        self.state
    }

    /// Returns the observation time.
    #[must_use]
    pub const fn observed_at(&self) -> TimePoint {
        self.observed_at
    }

    /// Returns the earliest availability time.
    #[must_use]
    pub const fn available_at(&self) -> TimePoint {
        self.available_at
    }

    /// Returns the expiration time.
    #[must_use]
    pub const fn expires_at(&self) -> Option<TimePoint> {
        self.expires_at
    }

    /// Returns the associated operation, if any.
    #[must_use]
    pub const fn operation(&self) -> Option<OperationId> {
        self.operation
    }

    /// Returns the logical qubits associated with the event.
    #[must_use]
    pub fn logical_qubits(&self) -> &BTreeSet<QubitId> {
        &self.logical_qubits
    }

    /// Returns the physical qubits associated with the event.
    #[must_use]
    pub fn physical_qubits(&self) -> &BTreeSet<PhysicalQubitId> {
        &self.physical_qubits
    }

    /// Returns event dependencies.
    #[must_use]
    pub fn dependencies(&self) -> &BTreeSet<RuntimeDependency> {
        &self.dependencies
    }

    /// Returns the optional label.
    #[must_use]
    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    /// Sets an expiration time.
    ///
    /// Returns an error if the expiration precedes availability.
    pub fn set_expiration(
        &mut self,
        expires_at: TimePoint,
    ) -> Result<(), RuntimeError> {
        if expires_at < self.available_at {
            return Err(RuntimeError::InvalidEventWindow {
                event: self.id,
            });
        }

        self.expires_at = Some(expires_at);
        Ok(())
    }

    /// Adds a runtime dependency.
    pub fn add_dependency(
        &mut self,
        dependency: RuntimeDependency,
    ) -> Result<(), RuntimeError> {
        if self.state != RuntimeEventState::Pending {
            return Err(RuntimeError::ImmutableEventState {
                event: self.id,
                state: self.state,
            });
        }

        self.dependencies.insert(dependency);
        Ok(())
    }

    /// Associates a logical qubit.
    pub fn add_logical_qubit(
        &mut self,
        qubit: QubitId,
    ) -> Result<(), RuntimeError> {
        if self.state != RuntimeEventState::Pending {
            return Err(RuntimeError::ImmutableEventState {
                event: self.id,
                state: self.state,
            });
        }

        self.logical_qubits.insert(qubit);
        Ok(())
    }

    /// Associates a physical qubit.
    pub fn add_physical_qubit(
        &mut self,
        qubit: PhysicalQubitId,
    ) -> Result<(), RuntimeError> {
        if self.state != RuntimeEventState::Pending {
            return Err(RuntimeError::ImmutableEventState {
                event: self.id,
                state: self.state,
            });
        }

        self.physical_qubits.insert(qubit);
        Ok(())
    }

    /// Sets a diagnostic label.
    pub fn set_label(
        &mut self,
        label: impl Into<String>,
    ) -> Result<(), RuntimeError> {
        if self.state != RuntimeEventState::Pending {
            return Err(RuntimeError::ImmutableEventState {
                event: self.id,
                state: self.state,
            });
        }

        let label = label.into();

        if label.is_empty() {
            return Err(RuntimeError::EmptyLabel);
        }

        self.label = Some(label);
        Ok(())
    }

    /// Marks the event ready.
    pub fn mark_ready(&mut self) -> Result<(), RuntimeError> {
        match self.state {
            RuntimeEventState::Pending => {
                self.state = RuntimeEventState::Ready;
                Ok(())
            }

            RuntimeEventState::Ready => Ok(()),

            state => Err(RuntimeError::InvalidStateTransition {
                event: self.id,
                from: state,
                to: RuntimeEventState::Ready,
            }),
        }
    }

    /// Marks the event delivered.
    pub fn mark_delivered(&mut self) -> Result<(), RuntimeError> {
        if self.state != RuntimeEventState::Ready {
            return Err(RuntimeError::InvalidStateTransition {
                event: self.id,
                from: self.state,
                to: RuntimeEventState::Delivered,
            });
        }

        self.state = RuntimeEventState::Delivered;
        Ok(())
    }

    /// Acknowledges the event.
    pub fn acknowledge(&mut self) -> Result<(), RuntimeError> {
        match self.state {
            RuntimeEventState::Delivered | RuntimeEventState::Ready => {
                self.state = RuntimeEventState::Acknowledged;
                Ok(())
            }

            state => Err(RuntimeError::InvalidStateTransition {
                event: self.id,
                from: state,
                to: RuntimeEventState::Acknowledged,
            }),
        }
    }

    /// Cancels the event.
    pub fn cancel(&mut self) -> Result<(), RuntimeError> {
        if self.state.is_terminal() {
            return Err(RuntimeError::InvalidStateTransition {
                event: self.id,
                from: self.state,
                to: RuntimeEventState::Cancelled,
            });
        }

        self.state = RuntimeEventState::Cancelled;
        Ok(())
    }

    /// Invalidates the event.
    pub fn invalidate(&mut self) -> Result<(), RuntimeError> {
        if self.state.is_terminal() {
            return Err(RuntimeError::InvalidStateTransition {
                event: self.id,
                from: self.state,
                to: RuntimeEventState::Invalidated,
            });
        }

        self.state = RuntimeEventState::Invalidated;
        Ok(())
    }

    /// Marks the event failed.
    pub fn fail(&mut self) -> Result<(), RuntimeError> {
        if self.state.is_terminal() {
            return Err(RuntimeError::InvalidStateTransition {
                event: self.id,
                from: self.state,
                to: RuntimeEventState::Failed,
            });
        }

        self.state = RuntimeEventState::Failed;
        Ok(())
    }

    /// Returns whether the event is available at the supplied runtime time.
    #[must_use]
    pub fn is_available_at(&self, now: TimePoint) -> bool {
        now >= self.available_at
            && self
                .expires_at
                .is_none_or(|expiration| now <= expiration)
    }

    /// Returns whether the event has expired at the supplied time.
    #[must_use]
    pub fn is_expired_at(&self, now: TimePoint) -> bool {
        self.expires_at
            .is_some_and(|expiration| now > expiration)
    }

    /// Validates all temporal invariants.
    pub fn validate(&self) -> Result<(), RuntimeError> {
        if self.id.is_zero() {
            return Err(RuntimeError::ZeroEventId);
        }

        if self.epoch.is_zero() {
            return Err(RuntimeError::ZeroEpochId);
        }

        if self.available_at < self.observed_at {
            return Err(RuntimeError::InvalidEventWindow {
                event: self.id,
            });
        }

        if let Some(expiration) = self.expires_at {
            if expiration < self.available_at {
                return Err(RuntimeError::InvalidEventWindow {
                    event: self.id,
                });
            }
        }

        Ok(())
    }
}

// ============================================================================
// Runtime operation record
// ============================================================================

/// Scheduler-visible runtime record for one canonical quantum operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeOperation {
    operation: OperationId,
    state: RuntimeOperationState,

    /// Epoch in which the runtime record was created.
    epoch: RuntimeEpochId,

    /// Earliest point at which the operation may become eligible.
    earliest_start: TimePoint,

    /// Optional latest legal start.
    latest_start: Option<TimePoint>,

    /// Optional completion time reported by the executor.
    completed_at: Option<TimePoint>,

    /// Runtime dependencies.
    dependencies: BTreeSet<RuntimeDependency>,

    /// Dependencies currently satisfied.
    satisfied_dependencies: BTreeSet<RuntimeDependency>,

    /// Events emitted for this operation.
    events: BTreeSet<RuntimeEventId>,

    /// Logical qubit context.
    logical_qubits: BTreeSet<QubitId>,

    /// Physical qubit context, if routing has resolved it.
    physical_qubits: BTreeSet<PhysicalQubitId>,
}

impl RuntimeOperation {
    /// Creates a runtime operation record.
    #[must_use]
    pub fn new(
        operation: OperationId,
        epoch: RuntimeEpochId,
        earliest_start: TimePoint,
    ) -> Self {
        Self {
            operation,
            state: RuntimeOperationState::Planned,
            epoch,
            earliest_start,
            latest_start: None,
            completed_at: None,
            dependencies: BTreeSet::new(),
            satisfied_dependencies: BTreeSet::new(),
            events: BTreeSet::new(),
            logical_qubits: BTreeSet::new(),
            physical_qubits: BTreeSet::new(),
        }
    }

    /// Returns the canonical operation identity.
    #[must_use]
    pub const fn operation(&self) -> OperationId {
        self.operation
    }

    /// Returns the runtime state.
    #[must_use]
    pub const fn state(&self) -> RuntimeOperationState {
        self.state
    }

    /// Returns the creation epoch.
    #[must_use]
    pub const fn epoch(&self) -> RuntimeEpochId {
        self.epoch
    }

    /// Returns the earliest start.
    #[must_use]
    pub const fn earliest_start(&self) -> TimePoint {
        self.earliest_start
    }

    /// Returns the latest legal start.
    #[must_use]
    pub const fn latest_start(&self) -> Option<TimePoint> {
        self.latest_start
    }

    /// Returns the completion time.
    #[must_use]
    pub const fn completed_at(&self) -> Option<TimePoint> {
        self.completed_at
    }

    /// Returns operation dependencies.
    #[must_use]
    pub fn dependencies(&self) -> &BTreeSet<RuntimeDependency> {
        &self.dependencies
    }

    /// Returns satisfied dependencies.
    #[must_use]
    pub fn satisfied_dependencies(&self) -> &BTreeSet<RuntimeDependency> {
        &self.satisfied_dependencies
    }

    /// Returns emitted runtime events.
    #[must_use]
    pub fn events(&self) -> &BTreeSet<RuntimeEventId> {
        &self.events
    }

    /// Returns logical qubits.
    #[must_use]
    pub fn logical_qubits(&self) -> &BTreeSet<QubitId> {
        &self.logical_qubits
    }

    /// Returns physical qubits.
    #[must_use]
    pub fn physical_qubits(&self) -> &BTreeSet<PhysicalQubitId> {
        &self.physical_qubits
    }

    /// Sets the latest legal start.
    pub fn set_latest_start(
        &mut self,
        latest: TimePoint,
    ) -> Result<(), RuntimeError> {
        if latest < self.earliest_start {
            return Err(RuntimeError::InvalidOperationWindow {
                operation: self.operation,
            });
        }

        self.latest_start = Some(latest);
        Ok(())
    }

    /// Adds a dependency.
    pub fn add_dependency(
        &mut self,
        dependency: RuntimeDependency,
    ) -> Result<(), RuntimeError> {
        if self.state != RuntimeOperationState::Planned
            && self.state != RuntimeOperationState::WaitingForDependencies
        {
            return Err(RuntimeError::OperationAlreadyAdvanced {
                operation: self.operation,
            });
        }

        self.dependencies.insert(dependency);
        self.state = RuntimeOperationState::WaitingForDependencies;
        Ok(())
    }

    /// Associates a logical qubit.
    pub fn add_logical_qubit(
        &mut self,
        qubit: QubitId,
    ) -> Result<(), RuntimeError> {
        if self.state != RuntimeOperationState::Planned
            && self.state != RuntimeOperationState::WaitingForDependencies
        {
            return Err(RuntimeError::OperationAlreadyAdvanced {
                operation: self.operation,
            });
        }

        self.logical_qubits.insert(qubit);
        Ok(())
    }

    /// Associates a physical qubit.
    pub fn add_physical_qubit(
        &mut self,
        qubit: PhysicalQubitId,
    ) -> Result<(), RuntimeError> {
        if self.state != RuntimeOperationState::Planned
            && self.state != RuntimeOperationState::WaitingForDependencies
        {
            return Err(RuntimeError::OperationAlreadyAdvanced {
                operation: self.operation,
            });
        }

        self.physical_qubits.insert(qubit);
        Ok(())
    }

    /// Records an event emitted for this operation.
    pub fn add_event(
        &mut self,
        event: RuntimeEventId,
    ) -> Result<(), RuntimeError> {
        if event.is_zero() {
            return Err(RuntimeError::ZeroEventId);
        }

        self.events.insert(event);
        Ok(())
    }

    /// Satisfies a dependency.
    ///
    /// Returns whether all dependencies are now satisfied.
    pub fn satisfy_dependency(
        &mut self,
        dependency: RuntimeDependency,
    ) -> Result<bool, RuntimeError> {
        if !self.dependencies.contains(&dependency) {
            return Err(RuntimeError::UnknownOperationDependency {
                operation: self.operation,
            });
        }

        self.satisfied_dependencies.insert(dependency);

        if self.satisfied_dependencies.len() == self.dependencies.len() {
            self.state = RuntimeOperationState::Eligible;
            Ok(true)
        } else {
            self.state = RuntimeOperationState::WaitingForDependencies;
            Ok(false)
        }
    }

    /// Marks the operation eligible when it has no dependencies.
    pub fn make_eligible_if_unblocked(&mut self) {
        if self.dependencies.is_empty()
            && matches!(
                self.state,
                RuntimeOperationState::Planned
                    | RuntimeOperationState::WaitingForDependencies
            )
        {
            self.state = RuntimeOperationState::Eligible;
        }
    }

    /// Releases the operation.
    pub fn release(&mut self) -> Result<(), RuntimeError> {
        if self.state != RuntimeOperationState::Eligible {
            return Err(RuntimeError::OperationNotEligible {
                operation: self.operation,
                state: self.state,
            });
        }

        self.state = RuntimeOperationState::Released;
        Ok(())
    }

    /// Reports execution start.
    pub fn start(&mut self) -> Result<(), RuntimeError> {
        if self.state != RuntimeOperationState::Released {
            return Err(RuntimeError::InvalidOperationTransition {
                operation: self.operation,
                from: self.state,
                to: RuntimeOperationState::Executing,
            });
        }

        self.state = RuntimeOperationState::Executing;
        Ok(())
    }

    /// Reports successful completion.
    pub fn complete(
        &mut self,
        completed_at: TimePoint,
    ) -> Result<(), RuntimeError> {
        if self.state != RuntimeOperationState::Executing {
            return Err(RuntimeError::InvalidOperationTransition {
                operation: self.operation,
                from: self.state,
                to: RuntimeOperationState::Completed,
            });
        }

        self.completed_at = Some(completed_at);
        self.state = RuntimeOperationState::Completed;
        Ok(())
    }

    /// Cancels the operation.
    pub fn cancel(&mut self) -> Result<(), RuntimeError> {
        if self.state.is_terminal() {
            return Err(RuntimeError::InvalidOperationTransition {
                operation: self.operation,
                from: self.state,
                to: RuntimeOperationState::Cancelled,
            });
        }

        self.state = RuntimeOperationState::Cancelled;
        Ok(())
    }

    /// Marks the operation failed.
    pub fn fail(&mut self) -> Result<(), RuntimeError> {
        if self.state.is_terminal() {
            return Err(RuntimeError::InvalidOperationTransition {
                operation: self.operation,
                from: self.state,
                to: RuntimeOperationState::Failed,
            });
        }

        self.state = RuntimeOperationState::Failed;
        Ok(())
    }

    /// Invalidates the operation.
    pub fn invalidate(&mut self) -> Result<(), RuntimeError> {
        if self.state.is_terminal() {
            return Err(RuntimeError::InvalidOperationTransition {
                operation: self.operation,
                from: self.state,
                to: RuntimeOperationState::Invalidated,
            });
        }

        self.state = RuntimeOperationState::Invalidated;
        Ok(())
    }

    /// Expires the operation.
    pub fn expire(&mut self) -> Result<(), RuntimeError> {
        if self.state.is_terminal() {
            return Err(RuntimeError::InvalidOperationTransition {
                operation: self.operation,
                from: self.state,
                to: RuntimeOperationState::Expired,
            });
        }

        self.state = RuntimeOperationState::Expired;
        Ok(())
    }

    /// Returns whether all dependencies are satisfied.
    #[must_use]
    pub fn is_unblocked(&self) -> bool {
        self.satisfied_dependencies.len() == self.dependencies.len()
    }

    /// Validates operation invariants.
    pub fn validate(&self) -> Result<(), RuntimeError> {
        if self.epoch.is_zero() {
            return Err(RuntimeError::ZeroEpochId);
        }

        if self.satisfied_dependencies.len() > self.dependencies.len() {
            return Err(RuntimeError::DependencyAccountingMismatch {
                operation: self.operation,
            });
        }

        if let Some(latest) = self.latest_start {
            if latest < self.earliest_start {
                return Err(RuntimeError::InvalidOperationWindow {
                    operation: self.operation,
                });
            }
        }

        Ok(())
    }
}

// ============================================================================
// Runtime snapshot
// ============================================================================

/// Immutable snapshot of runtime scheduler state.
///
/// The snapshot is intended for diagnostics, verification, persistence
/// adapters, distributed coordination and recovery systems.
///
/// It contains no executable handle to hardware.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeSnapshot {
    epoch: RuntimeEpochId,
    now: TimePoint,
    operations: BTreeMap<OperationId, RuntimeOperationState>,
    events: BTreeMap<RuntimeEventId, RuntimeEventState>,
    ready_operations: Vec<OperationId>,
    ready_events: Vec<RuntimeEventId>,
}

impl RuntimeSnapshot {
    /// Returns the runtime epoch.
    #[must_use]
    pub const fn epoch(&self) -> RuntimeEpochId {
        self.epoch
    }

    /// Returns the snapshot time.
    #[must_use]
    pub const fn now(&self) -> TimePoint {
        self.now
    }

    /// Returns operation states.
    #[must_use]
    pub fn operations(
        &self,
    ) -> &BTreeMap<OperationId, RuntimeOperationState> {
        &self.operations
    }

    /// Returns event states.
    #[must_use]
    pub fn events(
        &self,
    ) -> &BTreeMap<RuntimeEventId, RuntimeEventState> {
        &self.events
    }

    /// Returns operations currently eligible for release.
    #[must_use]
    pub fn ready_operations(&self) -> &[OperationId] {
        &self.ready_operations
    }

    /// Returns runtime events currently ready.
    #[must_use]
    pub fn ready_events(&self) -> &[RuntimeEventId] {
        &self.ready_events
    }
}

// ============================================================================
// Runtime state
// ============================================================================

/// Complete mutable scheduler-visible runtime state.
///
/// This structure is deliberately separate from the executor.
///
/// It records state but does not perform execution.
#[derive(Debug, Clone)]
pub struct RuntimeState {
    epoch: RuntimeEpochId,
    now: TimePoint,

    operations: BTreeMap<OperationId, RuntimeOperation>,
    events: BTreeMap<RuntimeEventId, RuntimeEvent>,

    /// Dependency index:
    ///
    /// dependency -> operations waiting for it.
    dependency_index:
        BTreeMap<RuntimeDependency, BTreeSet<OperationId>>,

    /// Operation -> events emitted by that operation.
    operation_events:
        BTreeMap<OperationId, BTreeSet<RuntimeEventId>>,

    /// Deterministic ready-event queue.
    ready_events: BTreeSet<RuntimeEventId>,

    /// Deterministic ready-operation set.
    ready_operations: BTreeSet<OperationId>,

    /// Events that have been acknowledged and may be compacted.
    acknowledged_events: VecDeque<RuntimeEventId>,

    /// Next locally allocated runtime event identity.
    next_event_id: RuntimeEventId,

    /// Next runtime epoch.
    next_epoch: RuntimeEpochId,

    /// Whether the runtime has entered global cancellation.
    cancelled: bool,

    /// Whether the runtime has entered global invalidation.
    invalidated: bool,
}

impl Default for RuntimeState {
    fn default() -> Self {
        Self::new()
    }
}

impl RuntimeState {
    /// Creates an empty runtime state at epoch one and time zero.
    ///
    /// The epoch starts at one so zero remains reserved as an invalid/unset
    /// identity.
    #[must_use]
    pub fn new() -> Self {
        Self {
            epoch: RuntimeEpochId::new(1),
            now: TimePoint::ZERO,
            operations: BTreeMap::new(),
            events: BTreeMap::new(),
            dependency_index: BTreeMap::new(),
            operation_events: BTreeMap::new(),
            ready_events: BTreeSet::new(),
            ready_operations: BTreeSet::new(),
            acknowledged_events: VecDeque::new(),
            next_event_id: RuntimeEventId::new(1),
            next_epoch: RuntimeEpochId::new(1),
            cancelled: false,
            invalidated: false,
        }
    }

    /// Returns the current runtime epoch.
    #[must_use]
    pub const fn epoch(&self) -> RuntimeEpochId {
        self.epoch
    }

    /// Returns the current abstract runtime time.
    #[must_use]
    pub const fn now(&self) -> TimePoint {
        self.now
    }

    /// Returns whether global cancellation has been requested.
    #[must_use]
    pub const fn is_cancelled(&self) -> bool {
        self.cancelled
    }

    /// Returns whether global invalidation has been requested.
    #[must_use]
    pub const fn is_invalidated(&self) -> bool {
        self.invalidated
    }

    /// Returns the number of registered operations.
    #[must_use]
    pub fn operation_count(&self) -> usize {
        self.operations.len()
    }

    /// Returns the number of registered events.
    #[must_use]
    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    /// Returns an operation.
    #[must_use]
    pub fn operation(
        &self,
        operation: OperationId,
    ) -> Option<&RuntimeOperation> {
        self.operations.get(&operation)
    }

    /// Returns a mutable operation.
    pub fn operation_mut(
        &mut self,
        operation: OperationId,
    ) -> Option<&mut RuntimeOperation> {
        self.operations.get_mut(&operation)
    }

    /// Returns an event.
    #[must_use]
    pub fn event(
        &self,
        event: RuntimeEventId,
    ) -> Option<&RuntimeEvent> {
        self.events.get(&event)
    }

    /// Returns a mutable event.
    pub fn event_mut(
        &mut self,
        event: RuntimeEventId,
    ) -> Option<&mut RuntimeEvent> {
        self.events.get_mut(&event)
    }

    /// Registers a runtime operation.
    ///
    /// The operation identity comes from canonical IR.
    pub fn register_operation(
        &mut self,
        operation: RuntimeOperation,
    ) -> Result<(), RuntimeError> {
        operation.validate()?;

        let id = operation.operation;

        if self.operations.contains_key(&id) {
            return Err(RuntimeError::DuplicateOperation {
                operation: id,
            });
        }

        for dependency in operation.dependencies.iter() {
            self.dependency_index
                .entry(dependency.clone())
                .or_default()
                .insert(id);
        }

        self.operations.insert(id, operation);

        self.refresh_operation_readiness(id)?;

        Ok(())
    }

    /// Registers an externally created runtime event.
    pub fn register_event(
        &mut self,
        event: RuntimeEvent,
    ) -> Result<(), RuntimeError> {
        event.validate()?;

        let id = event.id;

        if self.events.contains_key(&id) {
            return Err(RuntimeError::DuplicateEvent { event: id });
        }

        if event.epoch != self.epoch {
            return Err(RuntimeError::WrongEpoch {
                expected: self.epoch,
                actual: event.epoch,
            });
        }

        if let Some(operation) = event.operation {
            self.operation_events
                .entry(operation)
                .or_default()
                .insert(id);
        }

        self.events.insert(id, event);

        self.refresh_event_readiness(id)?;

        Ok(())
    }

    /// Allocates a fresh runtime event identity.
    ///
    /// Identity allocation is local to this state instance.
    pub fn allocate_event_id(
        &mut self,
    ) -> Result<RuntimeEventId, RuntimeError> {
        let id = self.next_event_id;

        self.next_event_id = id
            .checked_next()
            .ok_or(RuntimeError::EventIdExhausted)?;

        if id.is_zero() {
            return Err(RuntimeError::EventIdExhausted);
        }

        Ok(id)
    }

    /// Creates and registers an event using the local event allocator.
    pub fn emit_event(
        &mut self,
        kind: RuntimeEventKind,
        observed_at: TimePoint,
        available_at: TimePoint,
    ) -> Result<RuntimeEventId, RuntimeError> {
        if observed_at > available_at {
            return Err(RuntimeError::InvalidTemporalOrdering);
        }

        let id = self.allocate_event_id()?;

        let event = RuntimeEvent::new(
            id,
            self.epoch,
            kind,
            observed_at,
            available_at,
        );

        self.register_event(event)?;

        Ok(id)
    }

    /// Adds an operation dependency after registration.
    pub fn add_operation_dependency(
        &mut self,
        operation: OperationId,
        dependency: RuntimeDependency,
    ) -> Result<(), RuntimeError> {
        let record = self
            .operations
            .get_mut(&operation)
            .ok_or(RuntimeError::UnknownOperation {
                operation,
            })?;

        record.add_dependency(dependency.clone())?;

        self.dependency_index
            .entry(dependency)
            .or_default()
            .insert(operation);

        self.refresh_operation_readiness(operation)
    }

    /// Records that an event has satisfied a dependency.
    ///
    /// Every operation indexed by that dependency is updated without recursive
    /// graph traversal.
    pub fn apply_event(
        &mut self,
        event_id: RuntimeEventId,
    ) -> Result<Vec<OperationId>, RuntimeError> {
        let event = self
            .events
            .get(&event_id)
            .ok_or(RuntimeError::UnknownEvent { event: event_id })?;

        if !event.state.is_deliverable()
            && event.state != RuntimeEventState::Acknowledged
        {
            return Err(RuntimeError::EventNotReady {
                event: event_id,
            });
        }

        let kind = event.kind.clone();

        let waiting: Vec<OperationId> = self
            .dependency_index
            .iter()
            .filter_map(|(dependency, operations)| {
                if dependency.is_satisfied_by(&kind) {
                    Some(operations.iter().copied())
                } else {
                    None
                }
            })
            .flatten()
            .collect();

        let mut newly_ready = Vec::new();

        for operation in waiting {
            if let Some(record) = self.operations.get_mut(&operation) {
                let matching_dependencies: Vec<RuntimeDependency> = record
                    .dependencies
                    .iter()
                    .filter(|dependency| dependency.is_satisfied_by(&kind))
                    .cloned()
                    .collect();

                for dependency in matching_dependencies {
                    let became_ready =
                        record.satisfy_dependency(dependency)?;

                    if became_ready {
                        newly_ready.push(operation);
                        break;
                    }
                }
            }
        }

        for operation in newly_ready.iter().copied() {
            self.ready_operations.insert(operation);
        }

        Ok(newly_ready)
    }

    /// Marks a runtime event ready if all of its own dependencies are satisfied.
    pub fn refresh_event_readiness(
        &mut self,
        event_id: RuntimeEventId,
    ) -> Result<bool, RuntimeError> {
        let now = self.now;

        let event = self
            .events
            .get_mut(&event_id)
            .ok_or(RuntimeError::UnknownEvent { event: event_id })?;

        if event.state != RuntimeEventState::Pending {
            return Ok(event.state == RuntimeEventState::Ready);
        }

        if event.is_expired_at(now) {
            event.state = RuntimeEventState::Invalidated;
            return Ok(false);
        }

        if !event.is_available_at(now) {
            return Ok(false);
        }

        /*
         * RuntimeEvent currently contains dependencies but no separate
         * satisfied-dependency set. This is deliberate: event dependencies
         * are represented as prerequisite events and are resolved by
         * RuntimeState::satisfy_event_dependency().
         *
         * Therefore a newly registered event with no dependencies is directly
         * eligible.
         */
        if event.dependencies.is_empty() {
            event.mark_ready()?;
            self.ready_events.insert(event_id);
            return Ok(true);
        }

        Ok(false)
    }

    /// Explicitly satisfies one event dependency.
    pub fn satisfy_event_dependency(
        &mut self,
        event_id: RuntimeEventId,
        dependency: RuntimeDependency,
    ) -> Result<bool, RuntimeError> {
        let event = self
            .events
            .get_mut(&event_id)
            .ok_or(RuntimeError::UnknownEvent { event: event_id })?;

        if !event.dependencies.contains(&dependency) {
            return Err(RuntimeError::UnknownEventDependency {
                event: event_id,
            });
        }

        /*
         * RuntimeEvent dependencies are immutable prerequisites. Once the
         * caller confirms one, remove it from the pending dependency set.
         *
         * This is safe because the event remains owned by RuntimeState and
         * cannot be externally mutated without going through this state.
         */
        event.dependencies.remove(&dependency);

        if event.dependencies.is_empty()
            && event.is_available_at(self.now)
        {
            event.mark_ready()?;
            self.ready_events.insert(event_id);
            return Ok(true);
        }

        Ok(false)
    }

    /// Refreshes one operation's readiness.
    pub fn refresh_operation_readiness(
        &mut self,
        operation: OperationId,
    ) -> Result<bool, RuntimeError> {
        let record = self
            .operations
            .get_mut(&operation)
            .ok_or(RuntimeError::UnknownOperation {
                operation,
            })?;

        record.make_eligible_if_unblocked();

        let eligible = record.state == RuntimeOperationState::Eligible;

        if eligible {
            self.ready_operations.insert(operation);
        } else {
            self.ready_operations.remove(&operation);
        }

        Ok(eligible)
    }

    /// Releases an operation to the executor.
    ///
    /// This does not execute the operation.
    pub fn release_operation(
        &mut self,
        operation: OperationId,
    ) -> Result<(), RuntimeError> {
        if self.cancelled {
            return Err(RuntimeError::RuntimeCancelled);
        }

        if self.invalidated {
            return Err(RuntimeError::RuntimeInvalidated);
        }

        let record = self
            .operations
            .get_mut(&operation)
            .ok_or(RuntimeError::UnknownOperation {
                operation,
            })?;

        if let Some(latest) = record.latest_start {
            if self.now > latest {
                record.expire()?;
                self.ready_operations.remove(&operation);

                return Err(RuntimeError::OperationExpired {
                    operation,
                });
            }
        }

        record.release()?;
        self.ready_operations.remove(&operation);

        Ok(())
    }

    /// Reports execution start.
    pub fn start_operation(
        &mut self,
        operation: OperationId,
    ) -> Result<(), RuntimeError> {
        let record = self
            .operations
            .get_mut(&operation)
            .ok_or(RuntimeError::UnknownOperation {
                operation,
            })?;

        record.start()
    }

    /// Reports operation completion.
    pub fn complete_operation(
        &mut self,
        operation: OperationId,
        completed_at: TimePoint,
    ) -> Result<RuntimeEventId, RuntimeError> {
        if completed_at < self.now {
            return Err(RuntimeError::TimeWentBackward {
                current: self.now,
                requested: completed_at,
            });
        }

        self.now = completed_at;

        let record = self
            .operations
            .get_mut(&operation)
            .ok_or(RuntimeError::UnknownOperation {
                operation,
            })?;

        record.complete(completed_at)?;

        let event = self.emit_event(
            RuntimeEventKind::OperationCompleted { operation },
            completed_at,
            completed_at,
        )?;

        if let Some(record) = self.operations.get_mut(&operation) {
            record.add_event(event)?;
        }

        Ok(event)
    }

    /// Reports operation failure.
    pub fn fail_operation(
        &mut self,
        operation: OperationId,
    ) -> Result<RuntimeEventId, RuntimeError> {
        let record = self
            .operations
            .get_mut(&operation)
            .ok_or(RuntimeError::UnknownOperation {
                operation,
            })?;

        record.fail()?;

        let event = self.emit_event(
            RuntimeEventKind::OperationFailed { operation },
            self.now,
            self.now,
        )?;

        if let Some(record) = self.operations.get_mut(&operation) {
            record.add_event(event)?;
        }

        Ok(event)
    }

    /// Delivers a ready event.
    ///
    /// Delivery means that the event is made available to the consuming
    /// scheduler component. It does not mean that a hardware action occurred.
    pub fn deliver_event(
        &mut self,
        event_id: RuntimeEventId,
    ) -> Result<(), RuntimeError> {
        let event = self
            .events
            .get_mut(&event_id)
            .ok_or(RuntimeError::UnknownEvent { event: event_id })?;

        event.mark_delivered()?;
        self.ready_events.remove(&event_id);

        Ok(())
    }

    /// Acknowledges an event.
    pub fn acknowledge_event(
        &mut self,
        event_id: RuntimeEventId,
    ) -> Result<(), RuntimeError> {
        let event = self
            .events
            .get_mut(&event_id)
            .ok_or(RuntimeError::UnknownEvent { event: event_id })?;

        event.acknowledge()?;

        self.ready_events.remove(&event_id);
        self.acknowledged_events.push_back(event_id);

        Ok(())
    }

    /// Advances runtime time.
    ///
    /// Time is monotonic.
    ///
    /// No wall-clock source is consulted.
    pub fn advance_to(
        &mut self,
        time: TimePoint,
    ) -> Result<(), RuntimeError> {
        if time < self.now {
            return Err(RuntimeError::TimeWentBackward {
                current: self.now,
                requested: time,
            });
        }

        self.now = time;

        let event_ids: Vec<RuntimeEventId> =
            self.events.keys().copied().collect();

        for event_id in event_ids {
            let _ = self.refresh_event_readiness(event_id)?;
        }

        let operation_ids: Vec<OperationId> =
            self.operations.keys().copied().collect();

        for operation in operation_ids {
            let record = self
                .operations
                .get(&operation)
                .ok_or(RuntimeError::UnknownOperation {
                    operation,
                })?;

            if let Some(latest) = record.latest_start {
                if self.now > latest
                    && !record.state.is_terminal()
                {
                    if let Some(record) =
                        self.operations.get_mut(&operation)
                    {
                        record.expire()?;
                    }

                    self.ready_operations.remove(&operation);
                }
            }
        }

        Ok(())
    }

    /// Advances the runtime time by a checked duration.
    pub fn advance_by(
        &mut self,
        duration: Duration,
    ) -> Result<TimePoint, RuntimeError> {
        let target = self
            .now
            .checked_add(duration)
            .ok_or(RuntimeError::TimeOverflow)?;

        self.advance_to(target)?;
        Ok(target)
    }

    /// Returns ready operations in deterministic order.
    #[must_use]
    pub fn ready_operations(&self) -> impl Iterator<Item = OperationId> + '_ {
        self.ready_operations.iter().copied()
    }

    /// Returns ready events in deterministic order.
    #[must_use]
    pub fn ready_events(&self) -> impl Iterator<Item = RuntimeEventId> + '_ {
        self.ready_events.iter().copied()
    }

    /// Requests global cancellation.
    pub fn cancel(&mut self) {
        self.cancelled = true;

        let operations: Vec<OperationId> =
            self.operations.keys().copied().collect();

        for operation in operations {
            if let Some(record) = self.operations.get_mut(&operation) {
                if !record.state.is_terminal() {
                    let _ = record.cancel();
                }
            }
        }

        let events: Vec<RuntimeEventId> =
            self.events.keys().copied().collect();

        for event in events {
            if let Some(record) = self.events.get_mut(&event) {
                if !record.state.is_terminal() {
                    let _ = record.cancel();
                }
            }
        }

        self.ready_operations.clear();
        self.ready_events.clear();
    }

    /// Requests global invalidation.
    ///
    /// Invalidation is distinct from cancellation: invalidation indicates that
    /// the current runtime state must no longer be trusted, for example after
    /// a superseding runtime epoch.
    pub fn invalidate(&mut self) {
        self.invalidated = true;

        let operations: Vec<OperationId> =
            self.operations.keys().copied().collect();

        for operation in operations {
            if let Some(record) = self.operations.get_mut(&operation) {
                if !record.state.is_terminal() {
                    let _ = record.invalidate();
                }
            }
        }

        let events: Vec<RuntimeEventId> =
            self.events.keys().copied().collect();

        for event in events {
            if let Some(record) = self.events.get_mut(&event) {
                if !record.state.is_terminal() {
                    let _ = record.invalidate();
                }
            }
        }

        self.ready_operations.clear();
        self.ready_events.clear();
    }

    /// Starts a new runtime epoch.
    ///
    /// Existing state remains available for diagnostics/recovery, but new
    /// events must belong to the new epoch.
    pub fn advance_epoch(
        &mut self,
    ) -> Result<RuntimeEpochId, RuntimeError> {
        let next = self
            .next_epoch
            .checked_next()
            .ok_or(RuntimeError::EpochExhausted)?;

        self.epoch = next;
        self.next_epoch = next;

        self.cancelled = false;
        self.invalidated = false;

        let _ = self.emit_event(
            RuntimeEventKind::EpochAdvanced { epoch: next },
            self.now,
            self.now,
        )?;

        Ok(next)
    }

    /// Creates an immutable state snapshot.
    #[must_use]
    pub fn snapshot(&self) -> RuntimeSnapshot {
        RuntimeSnapshot {
            epoch: self.epoch,
            now: self.now,
            operations: self
                .operations
                .iter()
                .map(|(id, operation)| (*id, operation.state))
                .collect(),
            events: self
                .events
                .iter()
                .map(|(id, event)| (*id, event.state))
                .collect(),
            ready_operations: self
                .ready_operations
                .iter()
                .copied()
                .collect(),
            ready_events: self
                .ready_events
                .iter()
                .copied()
                .collect(),
        }
    }

    /// Removes acknowledged events from retained runtime state.
    ///
    /// This is an explicit operation rather than automatic garbage collection.
    /// That is important for deterministic recovery and auditability.
    ///
    /// Returns the number of removed events.
    pub fn compact_acknowledged_events(
        &mut self,
    ) -> usize {
        let mut removed = 0usize;

        while let Some(event_id) = self.acknowledged_events.pop_front() {
            if self
                .events
                .get(&event_id)
                .is_some_and(|event| {
                    event.state == RuntimeEventState::Acknowledged
                })
            {
                self.events.remove(&event_id);
                self.ready_events.remove(&event_id);

                for events in self.operation_events.values_mut() {
                    events.remove(&event_id);
                }

                removed = removed.saturating_add(1);
            }
        }

        removed
    }

    /// Validates the entire runtime state.
    ///
    /// Validation is iterative and does not recurse through dependency graphs.
    pub fn validate(&self) -> Result<(), RuntimeError> {
        if self.epoch.is_zero() {
            return Err(RuntimeError::ZeroEpochId);
        }

        for operation in self.operations.values() {
            operation.validate()?;
        }

        for event in self.events.values() {
            event.validate()?;

            if event.epoch != self.epoch {
                /*
                 * Historical events from earlier epochs may remain in the
                 * state for recovery/diagnostics. Therefore this is not an
                 * unconditional error.
                 */
                if event.state == RuntimeEventState::Pending
                    || event.state == RuntimeEventState::Ready
                    || event.state == RuntimeEventState::Delivered
                {
                    return Err(RuntimeError::WrongEpoch {
                        expected: self.epoch,
                        actual: event.epoch,
                    });
                }
            }
        }

        for operation in self.ready_operations.iter() {
            let record = self.operations.get(operation).ok_or(
                RuntimeError::UnknownOperation {
                    operation: *operation,
                },
            )?;

            if record.state != RuntimeOperationState::Eligible {
                return Err(RuntimeError::ReadySetMismatch {
                    operation: *operation,
                });
            }
        }

        for event in self.ready_events.iter() {
            let record = self
                .events
                .get(event)
                .ok_or(RuntimeError::UnknownEvent { event: *event })?;

            if record.state != RuntimeEventState::Ready {
                return Err(RuntimeError::ReadyEventSetMismatch {
                    event: *event,
                });
            }
        }

        Ok(())
    }
}

// ============================================================================
// Runtime controller
// ============================================================================

/// High-level controller around [`RuntimeState`].
///
/// This is intentionally a state-management façade rather than a hardware
/// executor.
#[derive(Debug, Default)]
pub struct RuntimeController {
    state: RuntimeState,
}

impl RuntimeController {
    /// Creates a new runtime controller.
    #[must_use]
    pub fn new() -> Self {
        Self {
            state: RuntimeState::new(),
        }
    }

    /// Returns immutable runtime state.
    #[must_use]
    pub fn state(&self) -> &RuntimeState {
        &self.state
    }

    /// Returns mutable runtime state.
    ///
    /// This is provided so integration adapters can apply externally observed
    /// runtime facts without introducing another abstraction layer.
    pub fn state_mut(&mut self) -> &mut RuntimeState {
        &mut self.state
    }

    /// Returns a consistent immutable snapshot.
    #[must_use]
    pub fn snapshot(&self) -> RuntimeSnapshot {
        self.state.snapshot()
    }

    /// Processes one runtime event.
    ///
    /// The event must already have been made ready.
    ///
    /// The controller applies the event to waiting operations and then marks
    /// the event delivered.
    pub fn process_event(
        &mut self,
        event: RuntimeEventId,
    ) -> Result<Vec<OperationId>, RuntimeError> {
        let operations = self.state.apply_event(event)?;
        self.state.deliver_event(event)?;

        Ok(operations)
    }

    /// Processes all currently ready events in deterministic order.
    pub fn process_ready_events(
        &mut self,
    ) -> Result<Vec<OperationId>, RuntimeError> {
        let events: Vec<RuntimeEventId> =
            self.state.ready_events().collect();

        let mut operations = Vec::new();

        for event in events {
            let newly_ready = self.process_event(event)?;
            operations.extend(newly_ready);
        }

        operations.sort();
        operations.dedup();

        Ok(operations)
    }

    /// Advances runtime time and processes events that became available.
    pub fn advance_to(
        &mut self,
        time: TimePoint,
    ) -> Result<Vec<OperationId>, RuntimeError> {
        self.state.advance_to(time)?;
        self.process_ready_events()
    }

    /// Advances runtime time by a checked duration.
    pub fn advance_by(
        &mut self,
        duration: Duration,
    ) -> Result<Vec<OperationId>, RuntimeError> {
        self.state.advance_by(duration)?;
        self.process_ready_events()
    }

    /// Requests cancellation.
    pub fn cancel(&mut self) {
        self.state.cancel();
    }

    /// Invalidates the current runtime state.
    pub fn invalidate(&mut self) {
        self.state.invalidate();
    }

    /// Begins a new runtime epoch.
    pub fn advance_epoch(
        &mut self,
    ) -> Result<RuntimeEpochId, RuntimeError> {
        self.state.advance_epoch()
    }
}

// ============================================================================
// Runtime errors
// ============================================================================

/// Errors produced by the dynamic runtime scheduling state machine.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum RuntimeError {
    /// Runtime event identity is zero.
    ZeroEventId,

    /// Runtime epoch identity is zero.
    ZeroEpochId,

    /// Runtime event identity space is exhausted.
    EventIdExhausted,

    /// Runtime epoch identity space is exhausted.
    EpochExhausted,

    /// Event already exists.
    DuplicateEvent {
        /// Duplicate event identity.
        event: RuntimeEventId,
    },

    /// Operation already exists.
    DuplicateOperation {
        /// Duplicate operation identity.
        operation: OperationId,
    },

    /// Unknown event.
    UnknownEvent {
        /// Event identity.
        event: RuntimeEventId,
    },

    /// Unknown operation.
    UnknownOperation {
        /// Operation identity.
        operation: OperationId,
    },

    /// Event dependency does not exist.
    UnknownEventDependency {
        /// Event identity.
        event: RuntimeEventId,
    },

    /// Operation dependency does not exist.
    UnknownOperationDependency {
        /// Operation identity.
        operation: OperationId,
    },

    /// Event cannot currently be consumed.
    EventNotReady {
        /// Event identity.
        event: RuntimeEventId,
    },

    /// Event state transition is invalid.
    InvalidStateTransition {
        /// Event identity.
        event: RuntimeEventId,

        /// Previous state.
        from: RuntimeEventState,

        /// Requested state.
        to: RuntimeEventState,
    },

    /// Operation state transition is invalid.
    InvalidOperationTransition {
        /// Operation identity.
        operation: OperationId,

        /// Previous state.
        from: RuntimeOperationState,

        /// Requested state.
        to: RuntimeOperationState,
    },

    /// Operation is not eligible.
    OperationNotEligible {
        /// Operation identity.
        operation: OperationId,

        /// Current state.
        state: RuntimeOperationState,
    },

    /// Operation was already advanced beyond mutable planning.
    OperationAlreadyAdvanced {
        /// Operation identity.
        operation: OperationId,
    },

    /// Runtime event was modified after entering an immutable state.
    ImmutableEventState {
        /// Event identity.
        event: RuntimeEventId,

        /// Current state.
        state: RuntimeEventState,
    },

    /// Event window is invalid.
    InvalidEventWindow {
        /// Event identity.
        event: RuntimeEventId,
    },

    /// Operation window is invalid.
    InvalidOperationWindow {
        /// Operation identity.
        operation: OperationId,
    },

    /// Runtime time would move backwards.
    TimeWentBackward {
        /// Current runtime time.
        current: TimePoint,

        /// Requested runtime time.
        requested: TimePoint,
    },

    /// Runtime time arithmetic overflowed.
    TimeOverflow,

    /// Runtime epoch does not match the current state.
    WrongEpoch {
        /// Current epoch.
        expected: RuntimeEpochId,

        /// Supplied epoch.
        actual: RuntimeEpochId,
    },

    /// Operation deadline was missed.
    OperationExpired {
        /// Operation identity.
        operation: OperationId,
    },

    /// Runtime dependency accounting is inconsistent.
    DependencyAccountingMismatch {
        /// Operation identity.
        operation: OperationId,
    },

    /// Ready operation set is inconsistent with operation state.
    ReadySetMismatch {
        /// Operation identity.
        operation: OperationId,
    },

    /// Ready event set is inconsistent with event state.
    ReadyEventSetMismatch {
        /// Event identity.
        event: RuntimeEventId,
    },

    /// Empty diagnostic label.
    EmptyLabel,

    /// Global runtime cancellation is active.
    RuntimeCancelled,

    /// Global runtime invalidation is active.
    RuntimeInvalidated,

    /// Invalid temporal ordering.
    InvalidTemporalOrdering,
}

impl fmt::Display for RuntimeError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::ZeroEventId => {
                write!(formatter, "runtime event identity must not be zero")
            }

            Self::ZeroEpochId => {
                write!(formatter, "runtime epoch identity must not be zero")
            }

            Self::EventIdExhausted => {
                write!(formatter, "runtime event identity space exhausted")
            }

            Self::EpochExhausted => {
                write!(formatter, "runtime epoch identity space exhausted")
            }

            Self::DuplicateEvent { event } => {
                write!(formatter, "runtime event already exists: {event}")
            }

            Self::DuplicateOperation { operation } => {
                write!(
                    formatter,
                    "runtime operation already exists: {:?}",
                    operation
                )
            }

            Self::UnknownEvent { event } => {
                write!(formatter, "unknown runtime event: {event}")
            }

            Self::UnknownOperation { operation } => {
                write!(
                    formatter,
                    "unknown runtime operation: {:?}",
                    operation
                )
            }

            Self::UnknownEventDependency { event } => {
                write!(
                    formatter,
                    "unknown dependency for runtime event: {event}"
                )
            }

            Self::UnknownOperationDependency { operation } => {
                write!(
                    formatter,
                    "unknown dependency for runtime operation: {:?}",
                    operation
                )
            }

            Self::EventNotReady { event } => {
                write!(formatter, "runtime event is not ready: {event}")
            }

            Self::InvalidStateTransition { event, from, to } => {
                write!(
                    formatter,
                    "invalid runtime event transition for {event}: \
                     {from:?} -> {to:?}"
                )
            }

            Self::InvalidOperationTransition {
                operation,
                from,
                to,
            } => {
                write!(
                    formatter,
                    "invalid runtime operation transition for {:?}: \
                     {from:?} -> {to:?}",
                    operation
                )
            }

            Self::OperationNotEligible {
                operation,
                state,
            } => {
                write!(
                    formatter,
                    "runtime operation {:?} is not eligible: {state:?}",
                    operation
                )
            }

            Self::OperationAlreadyAdvanced { operation } => {
                write!(
                    formatter,
                    "runtime operation {:?} has already advanced",
                    operation
                )
            }

            Self::ImmutableEventState { event, state } => {
                write!(
                    formatter,
                    "runtime event {event} is immutable in state {state:?}"
                )
            }

            Self::InvalidEventWindow { event } => {
                write!(formatter, "invalid time window for {event}")
            }

            Self::InvalidOperationWindow { operation } => {
                write!(
                    formatter,
                    "invalid runtime time window for {:?}",
                    operation
                )
            }

            Self::TimeWentBackward {
                current,
                requested,
            } => {
                write!(
                    formatter,
                    "runtime time cannot move backwards: \
                     current={current}, requested={requested}"
                )
            }

            Self::TimeOverflow => {
                write!(formatter, "runtime time arithmetic overflow")
            }

            Self::WrongEpoch { expected, actual } => {
                write!(
                    formatter,
                    "runtime epoch mismatch: expected {expected}, got {actual}"
                )
            }

            Self::OperationExpired { operation } => {
                write!(
                    formatter,
                    "runtime operation {:?} missed its latest start",
                    operation
                )
            }

            Self::DependencyAccountingMismatch { operation } => {
                write!(
                    formatter,
                    "runtime dependency accounting mismatch for {:?}",
                    operation
                )
            }

            Self::ReadySetMismatch { operation } => {
                write!(
                    formatter,
                    "runtime ready-operation set mismatch for {:?}",
                    operation
                )
            }

            Self::ReadyEventSetMismatch { event } => {
                write!(
                    formatter,
                    "runtime ready-event set mismatch for {event}"
                )
            }

            Self::EmptyLabel => {
                write!(formatter, "runtime diagnostic label must not be empty")
            }

            Self::RuntimeCancelled => {
                write!(formatter, "runtime scheduling has been cancelled")
            }

            Self::RuntimeInvalidated => {
                write!(formatter, "runtime scheduling state is invalidated")
            }

            Self::InvalidTemporalOrdering => {
                write!(formatter, "invalid runtime temporal ordering")
            }
        }
    }
}

impl std::error::Error for RuntimeError {}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn operation(value: u64) -> OperationId {
        OperationId::from(value)
    }

    #[test]
    fn event_ids_are_checked_and_ordered() {
        let first = RuntimeEventId::new(1);
        let second = first.checked_next().expect("next identity");

        assert_eq!(second.value(), 2);
        assert!(first < second);
        assert!(!first.is_zero());
    }

    #[test]
    fn runtime_state_starts_at_zero_time_and_epoch_one() {
        let state = RuntimeState::new();

        assert_eq!(state.now(), TimePoint::ZERO);
        assert_eq!(state.epoch(), RuntimeEpochId::new(1));
        assert_eq!(state.operation_count(), 0);
        assert_eq!(state.event_count(), 0);
    }

    #[test]
    fn operation_without_dependencies_becomes_eligible() {
        let mut state = RuntimeState::new();

        let id = operation(1);

        let record =
            RuntimeOperation::new(id, state.epoch(), TimePoint::ZERO);

        state
            .register_operation(record)
            .expect("operation registration");

        assert_eq!(
            state.operation(id).map(RuntimeOperation::state),
            Some(RuntimeOperationState::Eligible)
        );

        assert_eq!(
            state.ready_operations().collect::<Vec<_>>(),
            vec![id]
        );
    }

    #[test]
    fn operation_waits_for_classical_signal() {
        let mut state = RuntimeState::new();

        let id = operation(1);

        let mut record =
            RuntimeOperation::new(id, state.epoch(), TimePoint::ZERO);

        let signal = ClassicalSignalId::new(1);

        record
            .add_dependency(RuntimeDependency::ClassicalSignal(signal))
            .expect("dependency");

        state
            .register_operation(record)
            .expect("operation registration");

        assert_eq!(
            state.operation(id).map(RuntimeOperation::state),
            Some(RuntimeOperationState::WaitingForDependencies)
        );

        let event = state
            .emit_event(
                RuntimeEventKind::ClassicalSignalReady { signal },
                TimePoint::ZERO,
                TimePoint::ZERO,
            )
            .expect("event");

        let newly_ready =
            state.apply_event(event).expect("apply event");

        assert_eq!(newly_ready, vec![id]);
        assert_eq!(
            state.operation(id).map(RuntimeOperation::state),
            Some(RuntimeOperationState::Eligible)
        );
    }

    #[test]
    fn operation_lifecycle_is_monotonic() {
        let mut state = RuntimeState::new();

        let id = operation(1);

        state
            .register_operation(RuntimeOperation::new(
                id,
                state.epoch(),
                TimePoint::ZERO,
            ))
            .expect("registration");

        state
            .release_operation(id)
            .expect("release");

        state
            .start_operation(id)
            .expect("start");

        state
            .complete_operation(id, TimePoint::new(10))
            .expect("complete");

        assert_eq!(
            state.operation(id).map(RuntimeOperation::state),
            Some(RuntimeOperationState::Completed)
        );
    }

    #[test]
    fn time_cannot_move_backwards() {
        let mut state = RuntimeState::new();

        state
            .advance_to(TimePoint::new(10))
            .expect("advance");

        let result =
            state.advance_to(TimePoint::new(9));

        assert!(matches!(
            result,
            Err(RuntimeError::TimeWentBackward { .. })
        ));
    }

    #[test]
    fn event_becomes_ready_when_available() {
        let mut state = RuntimeState::new();

        let event = state
            .emit_event(
                RuntimeEventKind::ExternalInputReady,
                TimePoint::ZERO,
                TimePoint::new(10),
            )
            .expect("event");

        assert_eq!(
            state.event(event).map(RuntimeEvent::state),
            Some(RuntimeEventState::Pending)
        );

        state
            .advance_to(TimePoint::new(10))
            .expect("advance");

        assert_eq!(
            state.event(event).map(RuntimeEvent::state),
            Some(RuntimeEventState::Ready)
        );
    }

    #[test]
    fn runtime_snapshot_is_deterministic() {
        let mut state = RuntimeState::new();

        let first = operation(2);
        let second = operation(1);

        state
            .register_operation(RuntimeOperation::new(
                first,
                state.epoch(),
                TimePoint::ZERO,
            ))
            .expect("first");

        state
            .register_operation(RuntimeOperation::new(
                second,
                state.epoch(),
                TimePoint::ZERO,
            ))
            .expect("second");

        let snapshot = state.snapshot();

        assert_eq!(
            snapshot.ready_operations(),
            &[second, first]
        );
    }

    #[test]
    fn cancellation_clears_ready_sets() {
        let mut state = RuntimeState::new();

        let id = operation(1);

        state
            .register_operation(RuntimeOperation::new(
                id,
                state.epoch(),
                TimePoint::ZERO,
            ))
            .expect("registration");

        assert_eq!(
            state.ready_operations().collect::<Vec<_>>(),
            vec![id]
        );

        state.cancel();

        assert!(state.is_cancelled());
        assert_eq!(
            state.ready_operations().collect::<Vec<_>>(),
            Vec::<OperationId>::new()
        );

        assert_eq!(
            state.operation(id).map(RuntimeOperation::state),
            Some(RuntimeOperationState::Cancelled)
        );
    }

    #[test]
    fn state_validation_succeeds_for_valid_state() {
        let mut state = RuntimeState::new();

        let id = operation(1);

        state
            .register_operation(RuntimeOperation::new(
                id,
                state.epoch(),
                TimePoint::ZERO,
            ))
            .expect("registration");

        state.validate().expect("valid runtime state");
    }
}