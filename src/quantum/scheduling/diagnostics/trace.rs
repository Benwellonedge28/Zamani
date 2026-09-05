//! Zamani Quantum Scheduling — Production Scheduling Trace
//!
//! This module provides structured, target-independent tracing for the quantum
//! scheduling subsystem.
//!
//! # Architectural role
//!
//! This module answers:
//!
//! > "Why did the scheduler make this decision, when did it make it, what
//! > resources and dependencies influenced it, and how much work did planning
//! > require?"
//!
//! It is intentionally diagnostic infrastructure rather than scheduling logic.
//!
//! The ownership boundary is:
//!
//! ```text
//! quantum::ir
//!      │
//!      ▼
//! routing
//!      │
//!      ▼
//! scheduling
//!      │
//!      ├── planners
//!      ├── policies
//!      ├── resources
//!      ├── timing
//!      ├── verification
//!      │
//!      ▼
//! diagnostics::trace
//! ```
//!
//! `trace.rs` observes scheduler behaviour. It does not decide where an
//! operation should execute, when it should execute, or which hardware should
//! execute it.
//!
//! # Responsibilities
//!
//! This module owns:
//!
//! - structured scheduling trace events;
//! - trace severity;
//! - trace categories;
//! - trace phases;
//! - trace event identity;
//! - optional canonical operation identity;
//! - optional canonical logical/physical qubit identity;
//! - optional canonical resource identity;
//! - dependency attribution;
//! - timing-decision attribution;
//! - resource-decision attribution;
//! - planner/algorithm attribution;
//! - human-readable explanations;
//! - deterministic event ordering;
//! - trace sinks;
//! - in-memory trace collection;
//! - bounded trace collection;
//! - streaming trace delivery;
//! - trace statistics;
//! - trace filtering;
//! - trace configuration;
//! - conversion of trace information into stable diagnostic summaries.
//!
//! It does NOT own:
//!
//! - quantum semantics;
//! - `QubitId`;
//! - `PhysicalQubitId`;
//! - `OperationId`;
//! - `ResourceId`;
//! - dependency graph construction;
//! - scheduling algorithms;
//! - scheduling policies;
//! - hardware discovery;
//! - routing;
//! - QEC algorithms;
//! - runtime execution;
//! - serialization formats;
//! - logging backends;
//! - global tracing state.
//!
//! # Canonical identity rule
//!
//! Logical and physical qubit identities are always imported from:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! Operation and resource identities are imported from the canonical IR:
//!
//! ```text
//! crate::quantum::ir::core::identity::OperationId
//! crate::quantum::ir::core::identity::ResourceId
//! ```
//!
//! This file MUST NOT define replacement identity types.
//!
//! # Universal-program principle
//!
//! A Zamani program is written once and may be scheduled against targets with
//! radically different:
//!
//! - qubit counts;
//! - operation counts;
//! - topologies;
//! - timing resolutions;
//! - resource capacities;
//! - control channels;
//! - measurement channels;
//! - QEC configurations;
//! - communication links;
//! - distributed execution resources.
//!
//! Trace infrastructure therefore contains no constants such as:
//!
//! ```text
//! MAX_QUBITS
//! MAX_OPERATIONS
//! MAX_RESOURCES
//! MAX_TRACE_EVENTS
//! ```
//!
//! A trace may be bounded only when the caller explicitly supplies a bound.
//!
//! "Infinity" means that this module introduces no artificial machine-size
//! ceiling. A concrete process remains bounded by its host resources and any
//! explicit policy supplied by the caller.
//!
//! # Non-interference
//!
//! Tracing must never change scheduler semantics.
//!
//! In particular:
//!
//! ```text
//! tracing enabled
//!      │
//!      ▼
//! scheduler decisions
//! ```
//!
//! must produce the same decisions as:
//!
//! ```text
//! tracing disabled
//!      │
//!      ▼
//! scheduler decisions
//! ```
//!
//! except for explicitly documented host-side performance effects.
//!
//! Trace code must not:
//!
//! - mutate scheduler state;
//! - allocate hardware resources;
//! - modify dependency graphs;
//! - modify operation semantics;
//! - modify resource calendars;
//! - modify timing constraints;
//! - invoke hardware;
//! - invoke runtime execution;
//! - introduce synchronization into scheduler state.
//!
//! # Streaming design
//!
//! Large schedules can produce enormous diagnostic volumes. Therefore the
//! module supports two fundamentally different modes:
//!
//! ```text
//! TraceSink
//!    ├── discard
//!    ├── in-memory collector
//!    ├── bounded collector
//!    └── external/streaming sink
//! ```
//!
//! The scheduler may emit events incrementally without retaining the complete
//! event history.
//!
//! # Determinism
//!
//! Each event receives a monotonically increasing `TraceSequence` assigned by
//! the trace session.
//!
//! The sequence is the canonical event order within one trace session.
//!
//! Deterministic scheduler operation should additionally use deterministic
//! operation/resource ordering supplied by the scheduler itself.
//!
//! This module never uses hash-map iteration as semantic ordering.
//!
//! # Thread safety
//!
//! The core trace structures contain no global state and no interior
//! mutability.
//!
//! A caller that requires concurrent emission should provide an appropriate
//! externally synchronized sink. This module deliberately does not hide a
//! global lock behind the trace API.
//!
//! # Rust contract
//!
//! Supported:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe`.
//!
//! # Integration contract
//!
//! Typical integration:
//!
//! ```text
//! SchedulingContext
//!       │
//!       ▼
//! SchedulerSession
//!       │
//!       ├── TraceSession::begin()
//!       │
//!       ▼
//! Planner
//!       │
//!       ├── trace operation ready
//!       ├── trace dependency wait
//!       ├── trace resource conflict
//!       ├── trace timing decision
//!       ├── trace reservation
//!       └── trace operation scheduled
//!       │
//!       ▼
//! Verification
//!       │
//!       ├── trace verification start
//!       ├── trace verification failure/success
//!       │
//!       ▼
//! SchedulingResult
//! ```
//!
//! Result diagnostics remain owned by `scheduling::result`.
//!
//! Trace events are lower-level observability records. They can be summarized
//! into result diagnostics but do not replace them.
//!
//! # Example
//!
//! ```rust
//! use crate::quantum::scheduling::diagnostics::trace::{
//!     TraceCategory,
//!     TraceConfig,
//!     TraceEvent,
//!     TraceLevel,
//!     TraceSession,
//! };
//!
//! let mut trace = TraceSession::new(TraceConfig::default());
//!
//! let _ = trace.emit(
//!     TraceEvent::new(
//!         TraceLevel::Info,
//!         TraceCategory::Planner,
//!         "planner.started",
//!     )
//!     .with_message("scheduling planner started"),
//! );
//!
//! let snapshot = trace.snapshot();
//! assert_eq!(snapshot.len(), 1);
//! ```
//!
//! # Production invariants
//!
//! 1. No machine-size limit is embedded.
//! 2. No qubit identity is redefined.
//! 3. No operation identity is redefined.
//! 4. No resource identity is redefined.
//! 5. Trace emission does not mutate scheduler semantics.
//! 6. Event ordering is explicit.
//! 7. Trace configuration is immutable after construction.
//! 8. Bounded retention is explicit.
//! 9. Streaming sinks can avoid unbounded memory growth.
//! 10. Event payloads are owned.
//! 11. No event stores references with scheduler lifetime requirements.
//! 12. Trace events can be inspected without invoking hardware.
//! 13. Trace filtering is deterministic.
//! 14. Counters use checked arithmetic.
//! 15. Overflow never silently wraps.
//! 16. No `unsafe` code is permitted.
//!
//! # Important architectural distinction
//!
//! A trace explains scheduling.
//!
//! It is not itself:
//!
//! - a log implementation;
//! - a metrics database;
//! - a profiler runtime;
//! - a scheduler;
//! - a verifier.
//!
//! Those systems may consume trace events through `TraceSink`.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::fmt;
use std::num::NonZeroU64;
use std::time::{Duration as HostDuration, Instant};

use crate::quantum::ir::core::identity::{OperationId, ResourceId};
use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};

use super::super::types::{
    DependencyId,
    Duration,
    EpochId,
    ReservationId,
    ScheduleId,
    SchedulerSessionId,
    TimePoint,
};

// =============================================================================
// Trace identity
// =============================================================================

/// Stable identity of one trace event.
///
/// The identity is local to one trace session. It is never a quantum
/// operation/resource identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TraceEventId(u64);

impl TraceEventId {
    /// Creates a trace event identity.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the numeric identity.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }

    /// Returns the next identity if representable.
    #[must_use]
    pub const fn checked_next(self) -> Option<Self> {
        match self.0.checked_add(1) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }
}

impl From<u64> for TraceEventId {
    fn from(value: u64) -> Self {
        Self::new(value)
    }
}

impl From<TraceEventId> for u64 {
    fn from(value: TraceEventId) -> Self {
        value.value()
    }
}

impl fmt::Display for TraceEventId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "trace-event:{}", self.0)
    }
}

// =============================================================================
// Trace sequence
// =============================================================================

/// Monotonic sequence number assigned to emitted events.
///
/// A sequence is distinct from `TraceEventId` so a future implementation can
/// use event IDs for correlation while sequence numbers continue to describe
/// emission order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TraceSequence(u64);

impl TraceSequence {
    /// Creates a sequence number.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the numeric value.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }

    /// Returns the next sequence if representable.
    #[must_use]
    pub const fn checked_next(self) -> Option<Self> {
        match self.0.checked_add(1) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }
}

impl fmt::Display for TraceSequence {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

// =============================================================================
// Trace severity
// =============================================================================

/// Severity of a trace event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TraceLevel {
    /// Extremely verbose internal event.
    Trace,

    /// Normal diagnostic information.
    Debug,

    /// Informational scheduler event.
    Info,

    /// Event indicating a degraded or noteworthy condition.
    Warn,

    /// Event representing a scheduling error or failure.
    Error,
}

impl Default for TraceLevel {
    fn default() -> Self {
        Self::Info
    }
}

impl fmt::Display for TraceLevel {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Trace => "trace",
            Self::Debug => "debug",
            Self::Info => "info",
            Self::Warn => "warn",
            Self::Error => "error",
        };

        formatter.write_str(value)
    }
}

// =============================================================================
// Trace category
// =============================================================================

/// Semantic category of a scheduling trace event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TraceCategory {
    /// Scheduler lifecycle.
    Lifecycle,

    /// Planner activity.
    Planner,

    /// Algorithm selection/activity.
    Algorithm,

    /// Dependency analysis.
    Dependency,

    /// Operation readiness.
    Operation,

    /// Resource allocation/reservation.
    Resource,

    /// Timing decisions.
    Timing,

    /// Constraint evaluation.
    Constraint,

    /// Routing-derived scheduling information.
    Routing,

    /// QEC-derived scheduling information.
    Qec,

    /// Dynamic circuit/classical feedback scheduling.
    Dynamic,

    /// Distributed scheduling and communication.
    Distributed,

    /// Verification.
    Verification,

    /// Schedule transformation.
    Transformation,

    /// Objective evaluation.
    Optimization,

    /// Capacity/limit decisions.
    Capacity,

    /// Serialization/provenance.
    Serialization,

    /// Performance/profile information.
    Profile,

    /// User/plugin supplied event.
    Custom,
}

impl fmt::Display for TraceCategory {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Lifecycle => "lifecycle",
            Self::Planner => "planner",
            Self::Algorithm => "algorithm",
            Self::Dependency => "dependency",
            Self::Operation => "operation",
            Self::Resource => "resource",
            Self::Timing => "timing",
            Self::Constraint => "constraint",
            Self::Routing => "routing",
            Self::Qec => "qec",
            Self::Dynamic => "dynamic",
            Self::Distributed => "distributed",
            Self::Verification => "verification",
            Self::Transformation => "transformation",
            Self::Optimization => "optimization",
            Self::Capacity => "capacity",
            Self::Serialization => "serialization",
            Self::Profile => "profile",
            Self::Custom => "custom",
        };

        formatter.write_str(value)
    }
}

// =============================================================================
// Trace phase
// =============================================================================

/// High-level scheduler phase.
///
/// Phases are descriptive. They do not impose a scheduler pipeline by
/// themselves.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TracePhase {
    /// Scheduler session creation.
    Initialization,

    /// Input normalization.
    Input,

    /// Dependency analysis.
    DependencyAnalysis,

    /// Resource analysis.
    ResourceAnalysis,

    /// Timing analysis.
    TimingAnalysis,

    /// Planning.
    Planning,

    /// Transformation.
    Transformation,

    /// Verification.
    Verification,

    /// Finalization.
    Finalization,

    /// Runtime/dynamic rescheduling.
    Runtime,

    /// Custom phase.
    Custom,
}

impl fmt::Display for TracePhase {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Initialization => "initialization",
            Self::Input => "input",
            Self::DependencyAnalysis => "dependency-analysis",
            Self::ResourceAnalysis => "resource-analysis",
            Self::TimingAnalysis => "timing-analysis",
            Self::Planning => "planning",
            Self::Transformation => "transformation",
            Self::Verification => "verification",
            Self::Finalization => "finalization",
            Self::Runtime => "runtime",
            Self::Custom => "custom",
        };

        formatter.write_str(value)
    }
}

// =============================================================================
// Trace decision
// =============================================================================

/// Classification of the decision represented by an event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TraceDecision {
    /// Operation became eligible.
    Ready,

    /// Operation was selected.
    Selected,

    /// Operation was scheduled.
    Scheduled,

    /// Operation was deferred.
    Deferred,

    /// Operation was rejected.
    Rejected,

    /// Resource was reserved.
    Reserved,

    /// Resource reservation was released.
    Released,

    /// Timing candidate was accepted.
    TimingAccepted,

    /// Timing candidate was rejected.
    TimingRejected,

    /// Constraint passed.
    ConstraintSatisfied,

    /// Constraint failed.
    ConstraintViolated,

    /// Verification passed.
    VerificationPassed,

    /// Verification failed.
    VerificationFailed,

    /// No decision was made; event is observational.
    Observed,
}

impl fmt::Display for TraceDecision {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Ready => "ready",
            Self::Selected => "selected",
            Self::Scheduled => "scheduled",
            Self::Deferred => "deferred",
            Self::Rejected => "rejected",
            Self::Reserved => "reserved",
            Self::Released => "released",
            Self::TimingAccepted => "timing-accepted",
            Self::TimingRejected => "timing-rejected",
            Self::ConstraintSatisfied => "constraint-satisfied",
            Self::ConstraintViolated => "constraint-violated",
            Self::VerificationPassed => "verification-passed",
            Self::VerificationFailed => "verification-failed",
            Self::Observed => "observed",
        };

        formatter.write_str(value)
    }
}

// =============================================================================
// Trace retention
// =============================================================================

/// Explicit trace retention policy.
///
/// `Unlimited` does not mean the host has infinite memory. It means this module
/// imposes no retention ceiling. A streaming sink should normally be preferred
/// for very large schedules.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TraceRetention {
    /// Retain every accepted event.
    Unlimited,

    /// Retain at most the supplied number of events.
    Bounded(NonZeroU64),

    /// Do not retain events in memory.
    Streaming,
}

impl Default for TraceRetention {
    fn default() -> Self {
        Self::Bounded(NonZeroU64::new(16_384).expect("non-zero literal"))
    }
}

// =============================================================================
// Trace configuration
// =============================================================================

/// Immutable trace configuration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TraceConfig {
    level: TraceLevel,
    retention: TraceRetention,
    include_messages: bool,
    include_operation_identity: bool,
    include_qubit_identity: bool,
    include_resource_identity: bool,
    include_timing: bool,
    include_explanations: bool,
}

impl Default for TraceConfig {
    fn default() -> Self {
        Self {
            level: TraceLevel::Info,
            retention: TraceRetention::default(),
            include_messages: true,
            include_operation_identity: true,
            include_qubit_identity: true,
            include_resource_identity: true,
            include_timing: true,
            include_explanations: true,
        }
    }
}

impl TraceConfig {
    /// Creates a configuration with trace emission disabled by level filtering.
    #[must_use]
    pub const fn disabled() -> Self {
        Self {
            level: TraceLevel::Error,
            retention: TraceRetention::Streaming,
            include_messages: false,
            include_operation_identity: false,
            include_qubit_identity: false,
            include_resource_identity: false,
            include_timing: false,
            include_explanations: false,
        }
    }

    /// Creates an unbounded in-memory configuration.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self {
            level: TraceLevel::Trace,
            retention: TraceRetention::Unlimited,
            include_messages: true,
            include_operation_identity: true,
            include_qubit_identity: true,
            include_resource_identity: true,
            include_timing: true,
            include_explanations: true,
        }
    }

    /// Sets the minimum retained/emitted trace level.
    #[must_use]
    pub const fn with_level(mut self, level: TraceLevel) -> Self {
        self.level = level;
        self
    }

    /// Sets trace retention.
    #[must_use]
    pub const fn with_retention(mut self, retention: TraceRetention) -> Self {
        self.retention = retention;
        self
    }

    /// Enables/disables message retention.
    #[must_use]
    pub const fn with_messages(mut self, enabled: bool) -> Self {
        self.include_messages = enabled;
        self
    }

    /// Enables/disables canonical operation identity retention.
    #[must_use]
    pub const fn with_operation_identity(mut self, enabled: bool) -> Self {
        self.include_operation_identity = enabled;
        self
    }

    /// Enables/disables qubit identity retention.
    #[must_use]
    pub const fn with_qubit_identity(mut self, enabled: bool) -> Self {
        self.include_qubit_identity = enabled;
        self
    }

    /// Enables/disables resource identity retention.
    #[must_use]
    pub const fn with_resource_identity(mut self, enabled: bool) -> Self {
        self.include_resource_identity = enabled;
        self
    }

    /// Enables/disables timing data retention.
    #[must_use]
    pub const fn with_timing(mut self, enabled: bool) -> Self {
        self.include_timing = enabled;
        self
    }

    /// Enables/disables explanation retention.
    #[must_use]
    pub const fn with_explanations(mut self, enabled: bool) -> Self {
        self.include_explanations = enabled;
        self
    }

    /// Returns the minimum accepted trace level.
    #[must_use]
    pub const fn level(&self) -> TraceLevel {
        self.level
    }

    /// Returns the retention policy.
    #[must_use]
    pub const fn retention(&self) -> TraceRetention {
        self.retention
    }

    /// Returns whether messages are retained.
    #[must_use]
    pub const fn includes_messages(&self) -> bool {
        self.include_messages
    }

    /// Returns whether operation identities are retained.
    #[must_use]
    pub const fn includes_operation_identity(&self) -> bool {
        self.include_operation_identity
    }

    /// Returns whether qubit identities are retained.
    #[must_use]
    pub const fn includes_qubit_identity(&self) -> bool {
        self.include_qubit_identity
    }

    /// Returns whether resource identities are retained.
    #[must_use]
    pub const fn includes_resource_identity(&self) -> bool {
        self.include_resource_identity
    }

    /// Returns whether timing information is retained.
    #[must_use]
    pub const fn includes_timing(&self) -> bool {
        self.include_timing
    }

    /// Returns whether explanations are retained.
    #[must_use]
    pub const fn includes_explanations(&self) -> bool {
        self.include_explanations
    }

    /// Returns whether an event at the specified level should be accepted.
    #[must_use]
    pub fn accepts_level(&self, level: TraceLevel) -> bool {
        level >= self.level
    }
}

// =============================================================================
// Trace references
// =============================================================================

/// Optional scheduler object references attached to an event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct TraceReferences {
    operation: Option<OperationId>,
    dependency: Option<DependencyId>,
    resource: Option<ResourceId>,
    reservation: Option<ReservationId>,
    logical_qubit: Option<QubitId>,
    physical_qubit: Option<PhysicalQubitId>,
    schedule: Option<ScheduleId>,
    epoch: Option<EpochId>,
    session: Option<SchedulerSessionId>,
}

impl TraceReferences {
    /// Creates an empty reference set.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            operation: None,
            dependency: None,
            resource: None,
            reservation: None,
            logical_qubit: None,
            physical_qubit: None,
            schedule: None,
            epoch: None,
            session: None,
        }
    }

    /// Attaches an operation identity.
    #[must_use]
    pub const fn with_operation(mut self, value: OperationId) -> Self {
        self.operation = Some(value);
        self
    }

    /// Attaches a dependency identity.
    #[must_use]
    pub const fn with_dependency(mut self, value: DependencyId) -> Self {
        self.dependency = Some(value);
        self
    }

    /// Attaches a resource identity.
    #[must_use]
    pub const fn with_resource(mut self, value: ResourceId) -> Self {
        self.resource = Some(value);
        self
    }

    /// Attaches a reservation identity.
    #[must_use]
    pub const fn with_reservation(mut self, value: ReservationId) -> Self {
        self.reservation = Some(value);
        self
    }

    /// Attaches a logical qubit.
    #[must_use]
    pub const fn with_logical_qubit(mut self, value: QubitId) -> Self {
        self.logical_qubit = Some(value);
        self
    }

    /// Attaches a physical qubit.
    #[must_use]
    pub const fn with_physical_qubit(mut self, value: PhysicalQubitId) -> Self {
        self.physical_qubit = Some(value);
        self
    }

    /// Attaches a schedule identity.
    #[must_use]
    pub const fn with_schedule(mut self, value: ScheduleId) -> Self {
        self.schedule = Some(value);
        self
    }

    /// Attaches an epoch identity.
    #[must_use]
    pub const fn with_epoch(mut self, value: EpochId) -> Self {
        self.epoch = Some(value);
        self
    }

    /// Attaches a scheduler session identity.
    #[must_use]
    pub const fn with_session(mut self, value: SchedulerSessionId) -> Self {
        self.session = Some(value);
        self
    }

    /// Returns the operation identity.
    #[must_use]
    pub const fn operation(self) -> Option<OperationId> {
        self.operation
    }

    /// Returns the dependency identity.
    #[must_use]
    pub const fn dependency(self) -> Option<DependencyId> {
        self.dependency
    }

    /// Returns the resource identity.
    #[must_use]
    pub const fn resource(self) -> Option<ResourceId> {
        self.resource
    }

    /// Returns the reservation identity.
    #[must_use]
    pub const fn reservation(self) -> Option<ReservationId> {
        self.reservation
    }

    /// Returns the logical qubit.
    #[must_use]
    pub const fn logical_qubit(self) -> Option<QubitId> {
        self.logical_qubit
    }

    /// Returns the physical qubit.
    #[must_use]
    pub const fn physical_qubit(self) -> Option<PhysicalQubitId> {
        self.physical_qubit
    }

    /// Returns the schedule identity.
    #[must_use]
    pub const fn schedule(self) -> Option<ScheduleId> {
        self.schedule
    }

    /// Returns the epoch identity.
    #[must_use]
    pub const fn epoch(self) -> Option<EpochId> {
        self.epoch
    }

    /// Returns the scheduler session identity.
    #[must_use]
    pub const fn session(self) -> Option<SchedulerSessionId> {
        self.session
    }
}

// =============================================================================
// Trace timing
// =============================================================================

/// Timing information attached to a trace event.
///
/// `schedule_time` uses the scheduler's target-independent time coordinate.
/// `host_elapsed` describes compiler-side elapsed wall time.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TraceTiming {
    schedule_time: Option<TimePoint>,
    duration: Option<Duration>,
    host_elapsed_nanos: Option<u128>,
}

impl TraceTiming {
    /// Creates empty timing information.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            schedule_time: None,
            duration: None,
            host_elapsed_nanos: None,
        }
    }

    /// Attaches scheduler time.
    #[must_use]
    pub const fn with_schedule_time(mut self, value: TimePoint) -> Self {
        self.schedule_time = Some(value);
        self
    }

    /// Attaches scheduler duration.
    #[must_use]
    pub const fn with_duration(mut self, value: Duration) -> Self {
        self.duration = Some(value);
        self
    }

    /// Attaches host-side elapsed duration.
    #[must_use]
    pub const fn with_host_elapsed(mut self, value: HostDuration) -> Self {
        self.host_elapsed_nanos = Some(value.as_nanos());
        self
    }

    /// Returns scheduler time.
    #[must_use]
    pub const fn schedule_time(self) -> Option<TimePoint> {
        self.schedule_time
    }

    /// Returns scheduler duration.
    #[must_use]
    pub const fn duration(self) -> Option<Duration> {
        self.duration
    }

    /// Returns host elapsed nanoseconds.
    #[must_use]
    pub const fn host_elapsed_nanos(self) -> Option<u128> {
        self.host_elapsed_nanos
    }
}

impl Default for TraceTiming {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Trace event
// =============================================================================

/// Structured scheduling trace event.
///
/// Events are created without a sequence or event identity. The enclosing
/// `TraceSession` assigns those values when the event is emitted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TraceEvent {
    level: TraceLevel,
    category: TraceCategory,
    phase: Option<TracePhase>,
    name: String,
    message: Option<String>,
    decision: TraceDecision,
    references: TraceReferences,
    timing: TraceTiming,
    explanation: Option<String>,
}

impl TraceEvent {
    /// Creates a new trace event.
    pub fn new(
        level: TraceLevel,
        category: TraceCategory,
        name: impl Into<String>,
    ) -> Self {
        Self {
            level,
            category,
            phase: None,
            name: name.into(),
            message: None,
            decision: TraceDecision::Observed,
            references: TraceReferences::new(),
            timing: TraceTiming::new(),
            explanation: None,
        }
    }

    /// Attaches a phase.
    #[must_use]
    pub fn with_phase(mut self, phase: TracePhase) -> Self {
        self.phase = Some(phase);
        self
    }

    /// Attaches a human-readable message.
    #[must_use]
    pub fn with_message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }

    /// Attaches a decision.
    #[must_use]
    pub const fn with_decision(mut self, decision: TraceDecision) -> Self {
        self.decision = decision;
        self
    }

    /// Attaches references.
    #[must_use]
    pub const fn with_references(mut self, references: TraceReferences) -> Self {
        self.references = references;
        self
    }

    /// Attaches timing information.
    #[must_use]
    pub const fn with_timing(mut self, timing: TraceTiming) -> Self {
        self.timing = timing;
        self
    }

    /// Attaches an explanation.
    #[must_use]
    pub fn with_explanation(mut self, explanation: impl Into<String>) -> Self {
        self.explanation = Some(explanation.into());
        self
    }

    /// Returns the severity.
    #[must_use]
    pub const fn level(&self) -> TraceLevel {
        self.level
    }

    /// Returns the category.
    #[must_use]
    pub const fn category(&self) -> TraceCategory {
        self.category
    }

    /// Returns the phase.
    #[must_use]
    pub const fn phase(&self) -> Option<TracePhase> {
        self.phase
    }

    /// Returns the event name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the message.
    #[must_use]
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }

    /// Returns the decision.
    #[must_use]
    pub const fn decision(&self) -> TraceDecision {
        self.decision
    }

    /// Returns event references.
    #[must_use]
    pub const fn references(&self) -> TraceReferences {
        self.references
    }

    /// Returns timing information.
    #[must_use]
    pub const fn timing(&self) -> TraceTiming {
        self.timing
    }

    /// Returns the explanation.
    #[must_use]
    pub fn explanation(&self) -> Option<&str> {
        self.explanation.as_deref()
    }
}

// =============================================================================
// Emitted event
// =============================================================================

/// A trace event after session-level identity and ordering have been assigned.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmittedTraceEvent {
    id: TraceEventId,
    sequence: TraceSequence,
    event: TraceEvent,
}

impl EmittedTraceEvent {
    /// Creates an emitted event.
    fn new(
        id: TraceEventId,
        sequence: TraceSequence,
        event: TraceEvent,
    ) -> Self {
        Self {
            id,
            sequence,
            event,
        }
    }

    /// Returns the trace event identity.
    #[must_use]
    pub const fn id(&self) -> TraceEventId {
        self.id
    }

    /// Returns the emission sequence.
    #[must_use]
    pub const fn sequence(&self) -> TraceSequence {
        self.sequence
    }

    /// Returns the underlying event.
    #[must_use]
    pub const fn event(&self) -> &TraceEvent {
        &self.event
    }
}

// =============================================================================
// Trace sink
// =============================================================================

/// Errors returned by a trace sink.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TraceSinkError {
    /// The sink rejected the event.
    Rejected {
        /// Human-readable reason.
        reason: String,
    },

    /// The sink is closed.
    Closed,

    /// The sink cannot accept another event because its explicit capacity has
    /// been exhausted.
    CapacityExceeded,
}

impl fmt::Display for TraceSinkError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Rejected { reason } => {
                write!(formatter, "trace sink rejected event: {reason}")
            }
            Self::Closed => formatter.write_str("trace sink is closed"),
            Self::CapacityExceeded => {
                formatter.write_str("trace sink capacity exceeded")
            }
        }
    }
}

impl std::error::Error for TraceSinkError {}

/// Destination for emitted trace events.
///
/// Implementations should avoid mutating scheduler state. The sink is an
/// observability boundary.
pub trait TraceSink {
    /// Accepts one emitted event.
    fn emit(&mut self, event: &EmittedTraceEvent) -> Result<(), TraceSinkError>;

    /// Flushes pending sink state.
    ///
    /// The default implementation has nothing to flush.
    fn flush(&mut self) -> Result<(), TraceSinkError> {
        Ok(())
    }
}

/// Sink that intentionally discards all events.
///
/// Useful when a scheduler needs a uniform trace interface while diagnostics
/// are disabled.
#[derive(Debug, Default)]
pub struct DiscardTraceSink;

impl TraceSink for DiscardTraceSink {
    fn emit(
        &mut self,
        _event: &EmittedTraceEvent,
    ) -> Result<(), TraceSinkError> {
        Ok(())
    }
}

// =============================================================================
// In-memory trace sink
// =============================================================================

/// In-memory trace sink.
///
/// This is intended for diagnostics and tests. For very large schedules,
/// prefer a streaming sink.
#[derive(Debug, Default)]
pub struct InMemoryTraceSink {
    events: Vec<EmittedTraceEvent>,
}

impl InMemoryTraceSink {
    /// Creates an empty sink.
    #[must_use]
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }

    /// Returns retained events.
    #[must_use]
    pub fn events(&self) -> &[EmittedTraceEvent] {
        &self.events
    }

    /// Consumes the sink and returns all retained events.
    #[must_use]
    pub fn into_events(self) -> Vec<EmittedTraceEvent> {
        self.events
    }

    /// Clears all retained events.
    pub fn clear(&mut self) {
        self.events.clear();
    }
}

impl TraceSink for InMemoryTraceSink {
    fn emit(
        &mut self,
        event: &EmittedTraceEvent,
    ) -> Result<(), TraceSinkError> {
        self.events.push(event.clone());
        Ok(())
    }
}

// =============================================================================
// Trace statistics
// =============================================================================

/// Aggregate statistics for one trace session.
///
/// Counters are checked and never silently wrap.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TraceStatistics {
    emitted: u64,
    accepted: u64,
    filtered: u64,
    retained: u64,
    dropped: u64,
    errors: u64,
    warnings: u64,
}

impl TraceStatistics {
    /// Number of events submitted to the trace session.
    #[must_use]
    pub const fn emitted(&self) -> u64 {
        self.emitted
    }

    /// Number of events accepted by level filtering.
    #[must_use]
    pub const fn accepted(&self) -> u64 {
        self.accepted
    }

    /// Number of events filtered out.
    #[must_use]
    pub const fn filtered(&self) -> u64 {
        self.filtered
    }

    /// Number of events retained in memory.
    #[must_use]
    pub const fn retained(&self) -> u64 {
        self.retained
    }

    /// Number of events dropped because retention capacity was reached.
    #[must_use]
    pub const fn dropped(&self) -> u64 {
        self.dropped
    }

    /// Number of error events.
    #[must_use]
    pub const fn errors(&self) -> u64 {
        self.errors
    }

    /// Number of warning events.
    #[must_use]
    pub const fn warnings(&self) -> u64 {
        self.warnings
    }
}

// =============================================================================
// Trace errors
// =============================================================================

/// Errors produced by the trace session.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TraceError {
    /// Event identity space was exhausted.
    EventIdExhausted,

    /// Sequence space was exhausted.
    SequenceExhausted,

    /// Trace retention capacity was exhausted.
    RetentionCapacityExceeded,

    /// The configured sink rejected an event.
    Sink(TraceSinkError),

    /// An internal trace invariant was violated.
    InvariantViolation {
        /// Explanation.
        message: String,
    },
}

impl fmt::Display for TraceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EventIdExhausted => {
                formatter.write_str("trace event identity space exhausted")
            }
            Self::SequenceExhausted => {
                formatter.write_str("trace sequence space exhausted")
            }
            Self::RetentionCapacityExceeded => {
                formatter.write_str("trace retention capacity exceeded")
            }
            Self::Sink(error) => write!(formatter, "{error}"),
            Self::InvariantViolation { message } => {
                write!(formatter, "trace invariant violated: {message}")
            }
        }
    }
}

impl std::error::Error for TraceError {}

impl From<TraceSinkError> for TraceError {
    fn from(error: TraceSinkError) -> Self {
        Self::Sink(error)
    }
}

// =============================================================================
// Trace snapshot
// =============================================================================

/// Immutable snapshot of retained trace events.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TraceSnapshot {
    events: Vec<EmittedTraceEvent>,
}

impl TraceSnapshot {
    /// Creates a snapshot from retained events.
    #[must_use]
    fn new(events: Vec<EmittedTraceEvent>) -> Self {
        Self { events }
    }

    /// Returns retained events in sequence order.
    #[must_use]
    pub fn events(&self) -> &[EmittedTraceEvent] {
        &self.events
    }

    /// Returns the number of retained events.
    #[must_use]
    pub fn len(&self) -> usize {
        self.events.len()
    }

    /// Returns whether no events were retained.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    /// Returns the first retained event.
    #[must_use]
    pub fn first(&self) -> Option<&EmittedTraceEvent> {
        self.events.first()
    }

    /// Returns the last retained event.
    #[must_use]
    pub fn last(&self) -> Option<&EmittedTraceEvent> {
        self.events.last()
    }

    /// Consumes the snapshot into its event vector.
    #[must_use]
    pub fn into_events(self) -> Vec<EmittedTraceEvent> {
        self.events
    }
}

// =============================================================================
// Trace session
// =============================================================================

/// Stateful trace session.
///
/// A session owns trace identity allocation and optional retained history.
///
/// The scheduler should own the session rather than placing one in global
/// state.
#[derive(Debug)]
pub struct TraceSession {
    config: TraceConfig,
    next_event_id: TraceEventId,
    next_sequence: TraceSequence,
    events: Vec<EmittedTraceEvent>,
    statistics: TraceStatistics,
}

impl TraceSession {
    /// Creates an empty trace session.
    #[must_use]
    pub fn new(config: TraceConfig) -> Self {
        Self {
            config,
            next_event_id: TraceEventId::new(0),
            next_sequence: TraceSequence::new(0),
            events: Vec::new(),
            statistics: TraceStatistics::default(),
        }
    }

    /// Returns the session configuration.
    #[must_use]
    pub fn config(&self) -> &TraceConfig {
        &self.config
    }

    /// Returns trace statistics.
    #[must_use]
    pub const fn statistics(&self) -> TraceStatistics {
        self.statistics
    }

    /// Emits an event into this session.
    ///
    /// The event is filtered according to configuration before it receives a
    /// sequence and identity.
    pub fn emit(
        &mut self,
        mut event: TraceEvent,
    ) -> Result<Option<EmittedTraceEvent>, TraceError> {
        self.statistics.emitted =
            self.statistics
                .emitted
                .checked_add(1)
                .ok_or(TraceError::InvariantViolation {
                    message: String::from("trace emitted counter overflowed"),
                })?;

        if !self.config.accepts_level(event.level) {
            self.statistics.filtered =
                self.statistics
                    .filtered
                    .checked_add(1)
                    .ok_or(TraceError::InvariantViolation {
                        message: String::from(
                            "trace filtered counter overflowed",
                        ),
                    })?;

            return Ok(None);
        }

        self.normalize_event(&mut event);

        let id = self.next_event_id;
        let sequence = self.next_sequence;

        self.next_event_id = id
            .checked_next()
            .ok_or(TraceError::EventIdExhausted)?;

        self.next_sequence = sequence
            .checked_next()
            .ok_or(TraceError::SequenceExhausted)?;

        let emitted = EmittedTraceEvent::new(id, sequence, event);

        self.statistics.accepted =
            self.statistics
                .accepted
                .checked_add(1)
                .ok_or(TraceError::InvariantViolation {
                    message: String::from("trace accepted counter overflowed"),
                })?;

        match emitted.event.level {
            TraceLevel::Warn => {
                self.statistics.warnings =
                    self.statistics
                        .warnings
                        .checked_add(1)
                        .ok_or(TraceError::InvariantViolation {
                            message: String::from(
                                "trace warning counter overflowed",
                            ),
                        })?;
            }
            TraceLevel::Error => {
                self.statistics.errors =
                    self.statistics
                        .errors
                        .checked_add(1)
                        .ok_or(TraceError::InvariantViolation {
                            message: String::from(
                                "trace error counter overflowed",
                            ),
                        })?;
            }
            TraceLevel::Trace | TraceLevel::Debug | TraceLevel::Info => {}
        }

        self.retain(emitted.clone())?;

        Ok(Some(emitted))
    }

    /// Emits an event directly to a caller-provided sink.
    ///
    /// This path is suitable for very large schedules because the caller can
    /// stream events without storing the entire trace in memory.
    pub fn emit_to<S>(
        &mut self,
        event: TraceEvent,
        sink: &mut S,
    ) -> Result<bool, TraceError>
    where
        S: TraceSink,
    {
        let emitted = self.emit(event)?;

        if let Some(ref event) = emitted {
            sink.emit(event)?;
        }

        Ok(emitted.is_some())
    }

    /// Emits an event directly to a sink without retaining the event in the
    /// session.
    ///
    /// This method is the preferred API for truly streaming workloads.
    pub fn stream_to<S>(
        &mut self,
        mut event: TraceEvent,
        sink: &mut S,
    ) -> Result<bool, TraceError>
    where
        S: TraceSink,
    {
        self.statistics.emitted =
            self.statistics
                .emitted
                .checked_add(1)
                .ok_or(TraceError::InvariantViolation {
                    message: String::from("trace emitted counter overflowed"),
                })?;

        if !self.config.accepts_level(event.level) {
            self.statistics.filtered =
                self.statistics
                    .filtered
                    .checked_add(1)
                    .ok_or(TraceError::InvariantViolation {
                        message: String::from(
                            "trace filtered counter overflowed",
                        ),
                    })?;

            return Ok(false);
        }

        self.normalize_event(&mut event);

        let id = self.next_event_id;
        let sequence = self.next_sequence;

        self.next_event_id = id
            .checked_next()
            .ok_or(TraceError::EventIdExhausted)?;

        self.next_sequence = sequence
            .checked_next()
            .ok_or(TraceError::SequenceExhausted)?;

        let emitted = EmittedTraceEvent::new(id, sequence, event);

        self.statistics.accepted =
            self.statistics
                .accepted
                .checked_add(1)
                .ok_or(TraceError::InvariantViolation {
                    message: String::from("trace accepted counter overflowed"),
                })?;

        match emitted.event.level {
            TraceLevel::Warn => {
                self.statistics.warnings =
                    self.statistics
                        .warnings
                        .checked_add(1)
                        .ok_or(TraceError::InvariantViolation {
                            message: String::from(
                                "trace warning counter overflowed",
                            ),
                        })?;
            }
            TraceLevel::Error => {
                self.statistics.errors =
                    self.statistics
                        .errors
                        .checked_add(1)
                        .ok_or(TraceError::InvariantViolation {
                            message: String::from(
                                "trace error counter overflowed",
                            ),
                        })?;
            }
            TraceLevel::Trace | TraceLevel::Debug | TraceLevel::Info => {}
        }

        sink.emit(&emitted)?;

        Ok(true)
    }

    /// Returns an immutable snapshot of retained events.
    #[must_use]
    pub fn snapshot(&self) -> TraceSnapshot {
        TraceSnapshot::new(self.events.clone())
    }

    /// Clears retained events without resetting event identity.
    pub fn clear(&mut self) {
        self.events.clear();
        self.statistics.retained = 0;
    }

    fn normalize_event(&self, event: &mut TraceEvent) {
        if !self.config.include_messages {
            event.message = None;
        }

        if !self.config.include_operation_identity {
            event.references.operation = None;
        }

        if !self.config.include_qubit_identity {
            event.references.logical_qubit = None;
            event.references.physical_qubit = None;
        }

        if !self.config.include_resource_identity {
            event.references.resource = None;
            event.references.reservation = None;
        }

        if !self.config.include_timing {
            event.timing = TraceTiming::new();
        }

        if !self.config.include_explanations {
            event.explanation = None;
        }
    }

    fn retain(
        &mut self,
        event: EmittedTraceEvent,
    ) -> Result<(), TraceError> {
        match self.config.retention {
            TraceRetention::Unlimited => {
                self.events.push(event);
                self.statistics.retained =
                    self.statistics
                        .retained
                        .checked_add(1)
                        .ok_or(TraceError::InvariantViolation {
                            message: String::from(
                                "trace retained counter overflowed",
                            ),
                        })?;
            }

            TraceRetention::Bounded(capacity) => {
                if self.events.len() >= capacity.get() as usize {
                    self.statistics.dropped =
                        self.statistics
                            .dropped
                            .checked_add(1)
                            .ok_or(TraceError::InvariantViolation {
                                message: String::from(
                                    "trace dropped counter overflowed",
                                ),
                            })?;

                    return Ok(());
                }

                self.events.push(event);
                self.statistics.retained =
                    self.statistics
                        .retained
                        .checked_add(1)
                        .ok_or(TraceError::InvariantViolation {
                            message: String::from(
                                "trace retained counter overflowed",
                            ),
                        })?;
            }

            TraceRetention::Streaming => {
                // No in-memory retention.
            }
        }

        Ok(())
    }
}

// =============================================================================
// Trace phase guard
// =============================================================================

/// RAII-style host-side phase measurement helper.
///
/// The guard does not mutate scheduler semantics. It only measures host-side
/// elapsed time and can emit a completion event when explicitly finished.
pub struct TracePhaseGuard<'a> {
    session: &'a mut TraceSession,
    phase: TracePhase,
    category: TraceCategory,
    started: Instant,
}

impl<'a> TracePhaseGuard<'a> {
    /// Starts a trace phase.
    pub fn begin(
        session: &'a mut TraceSession,
        phase: TracePhase,
        category: TraceCategory,
    ) -> Self {
        Self {
            session,
            phase,
            category,
            started: Instant::now(),
        }
    }

    /// Completes the phase and emits its elapsed host time.
    pub fn finish(
        self,
        name: impl Into<String>,
        level: TraceLevel,
    ) -> Result<Option<EmittedTraceEvent>, TraceError> {
        let elapsed = self.started.elapsed();

        self.session.emit(
            TraceEvent::new(level, self.category, name)
                .with_phase(self.phase)
                .with_timing(
                    TraceTiming::new().with_host_elapsed(elapsed),
                ),
        )
    }
}

// =============================================================================
// Convenience event builders
// =============================================================================

/// Creates an operation scheduling event.
#[must_use]
pub fn operation_scheduled(
    operation: OperationId,
    start: TimePoint,
    duration: Duration,
) -> TraceEvent {
    TraceEvent::new(
        TraceLevel::Debug,
        TraceCategory::Operation,
        "operation.scheduled",
    )
    .with_decision(TraceDecision::Scheduled)
    .with_references(TraceReferences::new().with_operation(operation))
    .with_timing(
        TraceTiming::new()
            .with_schedule_time(start)
            .with_duration(duration),
    )
}

/// Creates an operation deferral event.
#[must_use]
pub fn operation_deferred(
    operation: OperationId,
    reason: impl Into<String>,
) -> TraceEvent {
    let reason = reason.into();

    TraceEvent::new(
        TraceLevel::Debug,
        TraceCategory::Operation,
        "operation.deferred",
    )
    .with_decision(TraceDecision::Deferred)
    .with_references(TraceReferences::new().with_operation(operation))
    .with_message(reason.clone())
    .with_explanation(reason)
}

/// Creates a dependency wait event.
#[must_use]
pub fn dependency_wait(
    operation: OperationId,
    dependency: DependencyId,
) -> TraceEvent {
    TraceEvent::new(
        TraceLevel::Trace,
        TraceCategory::Dependency,
        "dependency.wait",
    )
    .with_decision(TraceDecision::Deferred)
    .with_references(
        TraceReferences::new()
            .with_operation(operation)
            .with_dependency(dependency),
    )
}

/// Creates a resource conflict event.
#[must_use]
pub fn resource_conflict(
    operation: OperationId,
    resource: ResourceId,
    reason: impl Into<String>,
) -> TraceEvent {
    let reason = reason.into();

    TraceEvent::new(
        TraceLevel::Debug,
        TraceCategory::Resource,
        "resource.conflict",
    )
    .with_decision(TraceDecision::Deferred)
    .with_references(
        TraceReferences::new()
            .with_operation(operation)
            .with_resource(resource),
    )
    .with_message(reason.clone())
    .with_explanation(reason)
}

/// Creates a resource reservation event.
#[must_use]
pub fn resource_reserved(
    operation: OperationId,
    resource: ResourceId,
    reservation: ReservationId,
    start: TimePoint,
    duration: Duration,
) -> TraceEvent {
    TraceEvent::new(
        TraceLevel::Trace,
        TraceCategory::Resource,
        "resource.reserved",
    )
    .with_decision(TraceDecision::Reserved)
    .with_references(
        TraceReferences::new()
            .with_operation(operation)
            .with_resource(resource)
            .with_reservation(reservation),
    )
    .with_timing(
        TraceTiming::new()
            .with_schedule_time(start)
            .with_duration(duration),
    )
}

/// Creates a timing decision event.
#[must_use]
pub fn timing_decision(
    operation: OperationId,
    start: TimePoint,
    duration: Duration,
    accepted: bool,
) -> TraceEvent {
    let decision = if accepted {
        TraceDecision::TimingAccepted
    } else {
        TraceDecision::TimingRejected
    };

    TraceEvent::new(
        TraceLevel::Trace,
        TraceCategory::Timing,
        "timing.decision",
    )
    .with_decision(decision)
    .with_references(TraceReferences::new().with_operation(operation))
    .with_timing(
        TraceTiming::new()
            .with_schedule_time(start)
            .with_duration(duration),
    )
}

/// Creates a verification event.
#[must_use]
pub fn verification_event(
    operation: Option<OperationId>,
    passed: bool,
    explanation: impl Into<String>,
) -> TraceEvent {
    let decision = if passed {
        TraceDecision::VerificationPassed
    } else {
        TraceDecision::VerificationFailed
    };

    let references = operation.map_or_else(
        TraceReferences::new,
        |value| TraceReferences::new().with_operation(value),
    );

    let explanation = explanation.into();

    TraceEvent::new(
        if passed {
            TraceLevel::Debug
        } else {
            TraceLevel::Error
        },
        TraceCategory::Verification,
        "verification.result",
    )
    .with_decision(decision)
    .with_references(references)
    .with_message(explanation.clone())
    .with_explanation(explanation)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn event_identity_starts_at_zero() {
        let mut session = TraceSession::new(
            TraceConfig::default().with_retention(
                TraceRetention::Unlimited,
            ),
        );

        let event = session
            .emit(TraceEvent::new(
                TraceLevel::Info,
                TraceCategory::Lifecycle,
                "session.started",
            ))
            .expect("trace emission should succeed")
            .expect("event should pass the configured filter");

        assert_eq!(event.id().value(), 0);
        assert_eq!(event.sequence().value(), 0);
    }

    #[test]
    fn event_sequence_is_monotonic() {
        let mut session = TraceSession::new(
            TraceConfig::default().with_retention(
                TraceRetention::Unlimited,
            ),
        );

        for expected in 0_u64..3 {
            let event = session
                .emit(TraceEvent::new(
                    TraceLevel::Info,
                    TraceCategory::Lifecycle,
                    "event",
                ))
                .expect("trace emission should succeed")
                .expect("event should pass the configured filter");

            assert_eq!(event.sequence().value(), expected);
        }
    }

    #[test]
    fn filtering_does_not_consume_event_identity() {
        let config = TraceConfig::default()
            .with_level(TraceLevel::Warn)
            .with_retention(TraceRetention::Unlimited);

        let mut session = TraceSession::new(config);

        let filtered = session
            .emit(TraceEvent::new(
                TraceLevel::Debug,
                TraceCategory::Lifecycle,
                "filtered",
            ))
            .expect("trace emission should succeed");

        assert!(filtered.is_none());

        let accepted = session
            .emit(TraceEvent::new(
                TraceLevel::Warn,
                TraceCategory::Lifecycle,
                "accepted",
            ))
            .expect("trace emission should succeed")
            .expect("warning should pass the filter");

        assert_eq!(accepted.id().value(), 0);
        assert_eq!(accepted.sequence().value(), 0);
    }

    #[test]
    fn bounded_retention_does_not_stop_emission() {
        let capacity =
            NonZeroU64::new(2).expect("literal is non-zero");

        let mut session = TraceSession::new(
            TraceConfig::default()
                .with_retention(TraceRetention::Bounded(capacity)),
        );

        for _ in 0..5 {
            session
                .emit(TraceEvent::new(
                    TraceLevel::Info,
                    TraceCategory::Lifecycle,
                    "event",
                ))
                .expect("bounded retention should not fail emission");
        }

        assert_eq!(session.snapshot().len(), 2);
        assert_eq!(session.statistics().accepted(), 5);
        assert_eq!(session.statistics().dropped(), 3);
    }

    #[test]
    fn streaming_does_not_retain_events() {
        let mut session = TraceSession::new(
            TraceConfig::default()
                .with_retention(TraceRetention::Streaming),
        );

        let mut sink = InMemoryTraceSink::new();

        for _ in 0..3 {
            session
                .stream_to(
                    TraceEvent::new(
                        TraceLevel::Info,
                        TraceCategory::Lifecycle,
                        "event",
                    ),
                    &mut sink,
                )
                .expect("streaming emission should succeed");
        }

        assert!(session.snapshot().is_empty());
        assert_eq!(sink.events().len(), 3);
    }

    #[test]
    fn canonical_identities_are_retained() {
        let operation = OperationId::new(7);
        let resource = ResourceId::new(11);
        let logical = QubitId::new(13);
        let physical = PhysicalQubitId::new(17);

        let event = TraceEvent::new(
            TraceLevel::Debug,
            TraceCategory::Operation,
            "operation.inspect",
        )
        .with_references(
            TraceReferences::new()
                .with_operation(operation)
                .with_resource(resource)
                .with_logical_qubit(logical)
                .with_physical_qubit(physical),
        );

        assert_eq!(event.references().operation(), Some(operation));
        assert_eq!(event.references().resource(), Some(resource));
        assert_eq!(
            event.references().logical_qubit(),
            Some(logical)
        );
        assert_eq!(
            event.references().physical_qubit(),
            Some(physical)
        );
    }

    #[test]
    fn disabled_identity_categories_are_removed() {
        let operation = OperationId::new(1);
        let resource = ResourceId::new(2);
        let logical = QubitId::new(3);
        let physical = PhysicalQubitId::new(4);

        let config = TraceConfig::default()
            .with_operation_identity(false)
            .with_resource_identity(false)
            .with_qubit_identity(false);

        let mut session = TraceSession::new(config);

        let event = session
            .emit(
                TraceEvent::new(
                    TraceLevel::Info,
                    TraceCategory::Operation,
                    "operation.inspect",
                )
                .with_references(
                    TraceReferences::new()
                        .with_operation(operation)
                        .with_resource(resource)
                        .with_logical_qubit(logical)
                        .with_physical_qubit(physical),
                ),
            )
            .expect("trace emission should succeed")
            .expect("event should pass the filter");

        assert_eq!(event.event().references().operation(), None);
        assert_eq!(event.event().references().resource(), None);
        assert_eq!(event.event().references().logical_qubit(), None);
        assert_eq!(event.event().references().physical_qubit(), None);
    }

    #[test]
    fn operation_scheduled_builder_is_structured() {
        let operation = OperationId::new(42);

        let event = operation_scheduled(
            operation,
            TimePoint::new(100),
            Duration::new(25),
        );

        assert_eq!(
            event.decision(),
            TraceDecision::Scheduled
        );
        assert_eq!(
            event.references().operation(),
            Some(operation)
        );
        assert_eq!(
            event.timing().schedule_time(),
            Some(TimePoint::new(100))
        );
        assert_eq!(
            event.timing().duration(),
            Some(Duration::new(25))
        );
    }

    #[test]
    fn trace_statistics_count_severity() {
        let mut session = TraceSession::new(
            TraceConfig::default().with_retention(
                TraceRetention::Unlimited,
            ),
        );

        session
            .emit(TraceEvent::new(
                TraceLevel::Warn,
                TraceCategory::Constraint,
                "constraint.warning",
            ))
            .expect("warning emission should succeed");

        session
            .emit(TraceEvent::new(
                TraceLevel::Error,
                TraceCategory::Constraint,
                "constraint.error",
            ))
            .expect("error emission should succeed");

        assert_eq!(session.statistics().warnings(), 1);
        assert_eq!(session.statistics().errors(), 1);
    }

    #[test]
    fn trace_snapshot_preserves_sequence_order() {
        let mut session = TraceSession::new(
            TraceConfig::default().with_retention(
                TraceRetention::Unlimited,
            ),
        );

        for index in 0..4 {
            session
                .emit(
                    TraceEvent::new(
                        TraceLevel::Info,
                        TraceCategory::Lifecycle,
                        format!("event-{index}"),
                    ),
                )
                .expect("trace emission should succeed");
        }

        let snapshot = session.snapshot();

        for (index, event) in snapshot.events().iter().enumerate() {
            assert_eq!(event.sequence().value(), index as u64);
        }
    }
}