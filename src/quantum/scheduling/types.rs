//! Zamani Quantum Scheduling — Foundational Types
//!
//! This module defines the foundational, scheduler-owned vocabulary used by
//! the Zamani quantum scheduling subsystem.
//!
//! # Architectural responsibility
//!
//! This file answers:
//!
//! > "What are the stable values and identities used to describe a schedule?"
//!
//! It owns scheduler-specific:
//!
//! - schedule identity;
//! - dependency identity;
//! - reservation identity;
//! - scheduler epoch identity;
//! - abstract schedule time;
//! - duration and interval values;
//! - priority and cost values;
//! - makespan/slack values;
//! - schedule status;
//! - scheduling phases;
//! - operation/resource references;
//! - resource usage quantities;
//! - stable scheduler metadata;
//! - strongly typed identifiers required by downstream scheduling modules.
//!
//! It deliberately does NOT own:
//!
//! - quantum operation semantics;
//! - logical qubit identity;
//! - physical qubit identity;
//! - hardware topology;
//! - routing;
//! - hardware discovery;
//! - calibration;
//! - QEC algorithms;
//! - scheduling algorithms;
//! - optimization algorithms;
//! - runtime execution;
//! - serialization formats;
//! - frontend syntax.
//!
//! Those responsibilities belong to their canonical subsystems.
//!
//! # Canonical identity ownership
//!
//! Logical and physical qubit identities MUST come from:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! IR operation and resource identities MUST come from the canonical IR:
//!
//! ```text
//! crate::quantum::ir::core::identity::OperationId
//! crate::quantum::ir::core::identity::ResourceId
//! ```
//!
//! This module MUST NOT define another `QubitId`, `PhysicalQubitId`,
//! `OperationId`, or `ResourceId`.
//!
//! The repository explicitly establishes `quantum::ir::qubit` as the
//! authoritative logical/physical qubit identity boundary. 
//!
//! # Universal-program principle
//!
//! A Zamani program is written once at the semantic level and may be lowered
//! to compatible targets of different sizes and architectures.
//!
//! Therefore this module contains:
//!
//! - no maximum qubit count;
//! - no maximum operation count;
//! - no maximum schedule depth;
//! - no fixed topology size;
//! - no fixed resource count;
//! - no fixed gate arity;
//! - no vendor-specific resource IDs;
//! - no hardware timing constants.
//!
//! "Infinity" means:
//!
//! > no artificial finite machine-size ceiling is encoded by the scheduling
//! > type system.
//!
//! Concrete executions are necessarily finite because a particular compiler
//! process, address space, target, and execution request have finite resources.
//!
//! # Time representation
//!
//! Scheduling time is represented as an abstract, target-independent integer
//! coordinate.
//!
//! `TimePoint`, `Duration`, and related values do NOT assume:
//!
//! - nanoseconds;
//! - microseconds;
//! - device ticks;
//! - a particular clock;
//! - a particular pulse sample rate.
//!
//! The target timing subsystem supplies the interpretation of the schedule
//! coordinate.
//!
//! This prevents the scheduler core from embedding hardware assumptions.
//!
//! # Arithmetic safety
//!
//! All potentially overflowing arithmetic exposes checked operations.
//!
//! The constructors themselves reject invalid semantic values where possible.
//!
//! The module never relies on wrapping arithmetic for scheduling semantics.
//!
//! # Determinism
//!
//! Every type in this file has deterministic equality, ordering, hashing, and
//! formatting semantics where such traits are meaningful.
//!
//! Scheduler algorithms are responsible for choosing deterministic iteration
//! when deterministic compilation is requested.
//!
//! # Thread safety
//!
//! These types contain no global mutable state and no interior mutability.
//!
//! They are therefore naturally suitable for ownership transfer and concurrent
//! analysis when the containing scheduler data structures are themselves
//! designed for concurrent access.
//!
//! # Serialization
//!
//! This module does not define a serialization format.
//!
//! The values are deliberately plain and stable enough for the canonical
//! scheduling serialization subsystem to encode later.
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
//! - no external dependencies;
//! - no `unsafe` code.
//!
//! The no-unsafe requirement is compiler-enforced.
//!
//! # Integration contract
//!
//! Dependency direction:
//!
//! ```text
//! quantum::ir::core::identity ───────┐
//! quantum::ir::qubit ────────────────┤
//!                                    ▼
//!                         scheduling::types
//!                                    │
//!             ┌──────────────────────┼─────────────────────┐
//!             ▼                      ▼                     ▼
//!        timing/*              resources/*           ir/*
//!             │                      │                     │
//!             └──────────────────────┼─────────────────────┘
//!                                    ▼
//!                               policies/*
//!                                    │
//!                                    ▼
//!                               planners/*
//!                                    │
//!                                    ▼
//!                            verification/*
//! ```
//!
//! This file must remain foundational. Adding a planner, policy, resource
//! implementation, QEC integration, routing adapter, hardware adapter, or
//! runtime integration must not require changing the semantic definitions in
//! this file.
//!
//! # Design rule
//!
//! If a downstream subsystem needs a new domain concept, it should first ask:
//!
//! 1. Is the concept a canonical IR concept? Use the IR type.
//! 2. Is it a canonical qubit identity? Use `quantum::ir::qubit`.
//! 3. Is it a scheduler-specific value? Define it here only if it is a stable
//!    foundational scheduling concept.
//! 4. Is it algorithm-specific? Keep it in the algorithm module.
//! 5. Is it hardware-specific? Keep it in the hardware adapter.
//!
//! This keeps this file stable and prevents it becoming a dumping ground for
//! implementation-specific scheduler state.
//!
//! =============================================================================
//! Safety boundary
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::fmt;
use std::num::NonZeroU64;

// =============================================================================
// Canonical repository identities
// =============================================================================
//
// IMPORTANT:
//
// These are imported rather than recreated.
//
// `QubitId` and `PhysicalQubitId` are owned by the canonical qubit subsystem.
// `OperationId` and `ResourceId` are owned by the canonical IR identity
// subsystem.
//
// See:
//     crate::quantum::ir::qubit
//     crate::quantum::ir::core::identity
//

use crate::quantum::ir::core::identity::{OperationId, ResourceId};
use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};

// =============================================================================
// Stable scheduler identity macro
// =============================================================================
//
// Scheduler-specific identities use u64 rather than usize.
//
// This is deliberate:
//
// - semantic identity width does not depend on host architecture;
// - identity is not a collection index;
// - identity is not a resource capacity;
// - identity is not a hardware address;
// - identity is not a qubit count.
//
// Allocation/uniqueness belongs to the owning scheduler session or compiler
// pipeline. This module only defines the value contract.
//

macro_rules! define_scheduler_id {
    (
        $(#[$meta:meta])*
        $name:ident,
        $prefix:literal
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $name(u64);

        impl $name {
            /// Creates an identity from an explicitly supplied stable value.
            ///
            /// This function does not allocate or register the identity.
            #[must_use]
            pub const fn new(value: u64) -> Self {
                Self(value)
            }

            /// Returns the stable numeric representation.
            #[must_use]
            pub const fn value(self) -> u64 {
                self.0
            }

            /// Returns the textual prefix used for deterministic formatting.
            #[must_use]
            pub const fn prefix() -> &'static str {
                $prefix
            }

            /// Returns the next representable identity.
            ///
            /// This does not allocate the returned identity.
            #[must_use]
            pub const fn checked_next(self) -> Option<Self> {
                match self.0.checked_add(1) {
                    Some(value) => Some(Self(value)),
                    None => None,
                }
            }

            /// Returns whether this identity is the zero identity.
            #[must_use]
            pub const fn is_zero(self) -> bool {
                self.0 == 0
            }

            /// Returns a non-zero representation when the identity is non-zero.
            #[must_use]
            pub const fn as_non_zero(self) -> Option<NonZeroU64> {
                NonZeroU64::new(self.0)
            }
        }

        impl From<u64> for $name {
            fn from(value: u64) -> Self {
                Self::new(value)
            }
        }

        impl From<$name> for u64 {
            fn from(value: $name) -> Self {
                value.value()
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(formatter, "{}{}", $prefix, self.0)
            }
        }
    };
}

// =============================================================================
// Scheduler identity
// =============================================================================

define_scheduler_id!(
    /// Stable identity for one scheduling result or scheduling plan.
    ///
    /// A `ScheduleId` identifies the schedule artifact, not the program itself.
    ///
    /// The same program may legitimately produce multiple schedules for:
    ///
    /// - different targets;
    /// - different calibration snapshots;
    /// - different policies;
    /// - different optimization objectives;
    /// - different random seeds;
    /// - different compilation contexts.
    ScheduleId,
    "schedule:"
);

// =============================================================================
// Dependency identity
// =============================================================================

define_scheduler_id!(
    /// Stable identity for a scheduling dependency edge.
    ///
    /// A dependency identifies a scheduling relationship, not an IR operation.
    DependencyId,
    "dependency:"
);

// =============================================================================
// Reservation identity
// =============================================================================

define_scheduler_id!(
    /// Stable identity for one resource reservation.
    ///
    /// A reservation connects an operation with a resource over a scheduling
    /// interval.
    ReservationId,
    "reservation:"
);

// =============================================================================
// Scheduling epoch identity
// =============================================================================

define_scheduler_id!(
    /// Stable identity for a scheduling/planning epoch.
    ///
    /// Epochs allow dynamic and incremental scheduling systems to distinguish
    /// successive planning states without modifying operation identities.
    EpochId,
    "epoch:"
);

// =============================================================================
// Scheduler session identity
// =============================================================================

define_scheduler_id!(
    /// Stable identity for a scheduler compilation session.
    ///
    /// This is intentionally separate from `ScheduleId`.
    ///
    /// A session may produce multiple schedules.
    SchedulerSessionId,
    "scheduler-session:"
);

// =============================================================================
// Abstract schedule time
// =============================================================================

/// Abstract target-independent schedule time coordinate.
///
/// `TimePoint` has no intrinsic physical unit.
///
/// The timing subsystem and target adapter determine the meaning of the
/// coordinate.
///
/// This design intentionally avoids embedding values such as:
///
/// ```text
/// 1 nanosecond
/// 10 nanoseconds
/// 1 microsecond
/// dt = ...
/// ```
///
/// into the scheduler.
///
/// The value is an abstract non-negative coordinate.
///
/// # Overflow
///
/// Arithmetic is checked. A schedule operation that would exceed the
/// representable coordinate space must return an error/`None` rather than
/// silently wrapping.
///
/// # Scalability
///
/// The type contains no machine-size constant. A concrete compilation may use
/// any representable range supported by the host and target timing model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct TimePoint(u128);

impl TimePoint {
    /// The origin of a schedule.
    pub const ZERO: Self = Self(0);

    /// Creates a time point from an abstract coordinate.
    #[must_use]
    pub const fn new(value: u128) -> Self {
        Self(value)
    }

    /// Returns the abstract coordinate.
    #[must_use]
    pub const fn value(self) -> u128 {
        self.0
    }

    /// Returns whether this is the schedule origin.
    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.0 == 0
    }

    /// Checked addition of a duration.
    #[must_use]
    pub const fn checked_add(self, duration: Duration) -> Option<Self> {
        match self.0.checked_add(duration.value()) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }

    /// Checked subtraction of a duration.
    #[must_use]
    pub const fn checked_sub(self, duration: Duration) -> Option<Self> {
        match self.0.checked_sub(duration.value()) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }

    /// Returns the duration between two ordered time points.
    ///
    /// Returns `None` when `end < self`.
    #[must_use]
    pub const fn checked_duration_until(self, end: Self) -> Option<Duration> {
        match end.0.checked_sub(self.0) {
            Some(value) => Some(Duration::new(value)),
            None => None,
        }
    }
}

impl From<u128> for TimePoint {
    fn from(value: u128) -> Self {
        Self::new(value)
    }
}

impl From<TimePoint> for u128 {
    fn from(value: TimePoint) -> Self {
        value.value()
    }
}

impl fmt::Display for TimePoint {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "t{}", self.0)
    }
}

// =============================================================================
// Abstract duration
// =============================================================================

/// Non-negative target-independent scheduling duration.
///
/// Like `TimePoint`, this value has no intrinsic physical unit.
///
/// The target timing model interprets it.
///
/// A zero duration is representable because abstract operations and some
/// compiler-level scheduling events can legitimately have no execution time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct Duration(u128);

impl Duration {
    /// Zero duration.
    pub const ZERO: Self = Self(0);

    /// Creates a non-negative duration.
    #[must_use]
    pub const fn new(value: u128) -> Self {
        Self(value)
    }

    /// Returns the abstract duration coordinate.
    #[must_use]
    pub const fn value(self) -> u128 {
        self.0
    }

    /// Returns whether the duration is zero.
    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.0 == 0
    }

    /// Checked addition.
    #[must_use]
    pub const fn checked_add(self, other: Self) -> Option<Self> {
        match self.0.checked_add(other.0) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }

    /// Checked subtraction.
    ///
    /// Returns `None` when `other > self`.
    #[must_use]
    pub const fn checked_sub(self, other: Self) -> Option<Self> {
        match self.0.checked_sub(other.0) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }
}

impl From<u128> for Duration {
    fn from(value: u128) -> Self {
        Self::new(value)
    }
}

impl From<Duration> for u128 {
    fn from(value: Duration) -> Self {
        value.value()
    }
}

impl fmt::Display for Duration {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "duration:{}", self.0)
    }
}

// =============================================================================
// Schedule interval
// =============================================================================

/// Half-open scheduling interval `[start, end)`.
///
/// Half-open intervals provide an important invariant:
///
/// ```text
/// [0, 10) and [10, 20)
/// ```
///
/// do not overlap.
///
/// This makes resource conflict detection deterministic and avoids ambiguity
/// at operation boundaries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TimeInterval {
    start: TimePoint,
    end: TimePoint,
}

impl TimeInterval {
    /// Creates a validated half-open interval.
    ///
    /// Returns `None` when `end < start`.
    #[must_use]
    pub const fn new(start: TimePoint, end: TimePoint) -> Option<Self> {
        if end.value() < start.value() {
            None
        } else {
            Some(Self { start, end })
        }
    }

    /// Creates an interval from a start point and duration.
    ///
    /// Returns `None` when the end point would overflow.
    #[must_use]
    pub const fn from_duration(
        start: TimePoint,
        duration: Duration,
    ) -> Option<Self> {
        match start.checked_add(duration) {
            Some(end) => Some(Self { start, end }),
            None => None,
        }
    }

    /// Returns the start.
    #[must_use]
    pub const fn start(self) -> TimePoint {
        self.start
    }

    /// Returns the end.
    #[must_use]
    pub const fn end(self) -> TimePoint {
        self.end
    }

    /// Returns the interval duration.
    #[must_use]
    pub const fn duration(self) -> Duration {
        Duration::new(self.end.value() - self.start.value())
    }

    /// Returns whether the interval has zero duration.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.start.value() == self.end.value()
    }

    /// Returns whether this interval overlaps `other`.
    ///
    /// Intervals that merely touch at an endpoint do not overlap.
    #[must_use]
    pub const fn overlaps(self, other: Self) -> bool {
        self.start.value() < other.end.value()
            && other.start.value() < self.end.value()
    }

    /// Returns whether this interval is completely before `other`.
    #[must_use]
    pub const fn is_before(self, other: Self) -> bool {
        self.end.value() <= other.start.value()
    }

    /// Returns whether this interval is completely after `other`.
    #[must_use]
    pub const fn is_after(self, other: Self) -> bool {
        self.start.value() >= other.end.value()
    }

    /// Returns whether the point belongs to the interval under half-open
    /// semantics.
    #[must_use]
    pub const fn contains(self, point: TimePoint) -> bool {
        self.start.value() <= point.value()
            && point.value() < self.end.value()
    }
}

impl fmt::Display for TimeInterval {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "[{}, {})",
            self.start,
            self.end
        )
    }
}

// =============================================================================
// Scheduling priority
// =============================================================================

/// Ordered scheduling priority.
///
/// Larger values represent higher priority.
///
/// Priority is intentionally an abstract value. It does not prescribe:
///
/// - FIFO;
/// - critical-path priority;
/// - deadline priority;
/// - fidelity priority;
/// - user priority.
///
/// Those policies belong to `scheduling::policies`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct Priority(i64);

impl Priority {
    /// Neutral priority.
    pub const NEUTRAL: Self = Self(0);

    /// Creates a priority.
    #[must_use]
    pub const fn new(value: i64) -> Self {
        Self(value)
    }

    /// Returns the numeric priority.
    #[must_use]
    pub const fn value(self) -> i64 {
        self.0
    }

    /// Checked increment.
    #[must_use]
    pub const fn checked_add(self, amount: i64) -> Option<Self> {
        match self.0.checked_add(amount) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }

    /// Checked decrement.
    #[must_use]
    pub const fn checked_sub(self, amount: i64) -> Option<Self> {
        match self.0.checked_sub(amount) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }
}

impl From<i64> for Priority {
    fn from(value: i64) -> Self {
        Self::new(value)
    }
}

impl From<Priority> for i64 {
    fn from(value: Priority) -> Self {
        value.value()
    }
}

impl fmt::Display for Priority {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "priority:{}", self.0)
    }
}

// =============================================================================
// Abstract scheduling cost
// =============================================================================

/// Signed scheduling cost/score component.
///
/// This is deliberately abstract.
///
/// A policy may interpret it as:
///
/// - execution time;
/// - fidelity penalty;
/// - resource pressure;
/// - energy;
/// - communication cost;
/// - idle-time penalty.
///
/// No particular unit is encoded here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct Cost(i128);

impl Cost {
    /// Neutral cost.
    pub const ZERO: Self = Self(0);

    /// Creates a cost.
    #[must_use]
    pub const fn new(value: i128) -> Self {
        Self(value)
    }

    /// Returns the underlying score.
    #[must_use]
    pub const fn value(self) -> i128 {
        self.0
    }

    /// Checked addition.
    #[must_use]
    pub const fn checked_add(self, other: Self) -> Option<Self> {
        match self.0.checked_add(other.0) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }

    /// Checked subtraction.
    #[must_use]
    pub const fn checked_sub(self, other: Self) -> Option<Self> {
        match self.0.checked_sub(other.0) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }
}

impl From<i128> for Cost {
    fn from(value: i128) -> Self {
        Self::new(value)
    }
}

impl From<Cost> for i128 {
    fn from(value: Cost) -> Self {
        value.value()
    }
}

impl fmt::Display for Cost {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "cost:{}", self.0)
    }
}

// =============================================================================
// Makespan
// =============================================================================

/// Total schedule duration.
///
/// `Makespan` is kept distinct from `Duration` so APIs cannot accidentally
/// substitute an arbitrary operation duration where a complete schedule
/// duration is expected.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct Makespan(Duration);

impl Makespan {
    /// Empty schedule makespan.
    pub const ZERO: Self = Self(Duration::ZERO);

    /// Creates a makespan.
    #[must_use]
    pub const fn new(duration: Duration) -> Self {
        Self(duration)
    }

    /// Returns the underlying duration.
    #[must_use]
    pub const fn duration(self) -> Duration {
        self.0
    }

    /// Returns the abstract numeric coordinate.
    #[must_use]
    pub const fn value(self) -> u128 {
        self.0.value()
    }

    /// Returns whether the schedule is empty in time.
    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.0.is_zero()
    }
}

impl From<Duration> for Makespan {
    fn from(value: Duration) -> Self {
        Self::new(value)
    }
}

impl From<Makespan> for Duration {
    fn from(value: Makespan) -> Self {
        value.duration()
    }
}

impl fmt::Display for Makespan {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "makespan:{}", self.0.value())
    }
}

// =============================================================================
// Slack
// =============================================================================

/// Non-negative scheduling slack.
///
/// Slack describes how much temporal flexibility remains before a constraint
/// becomes binding.
///
/// Negative slack is not representable here; a violated constraint belongs in
/// the verification/error domain rather than being silently encoded as valid
/// slack.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct Slack(Duration);

impl Slack {
    /// Zero slack.
    pub const ZERO: Self = Self(Duration::ZERO);

    /// Creates slack.
    #[must_use]
    pub const fn new(duration: Duration) -> Self {
        Self(duration)
    }

    /// Returns the underlying duration.
    #[must_use]
    pub const fn duration(self) -> Duration {
        self.0
    }

    /// Returns the numeric value.
    #[must_use]
    pub const fn value(self) -> u128 {
        self.0.value()
    }

    /// Returns whether no slack remains.
    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.0.is_zero()
    }
}

impl From<Duration> for Slack {
    fn from(value: Duration) -> Self {
        Self::new(value)
    }
}

impl From<Slack> for Duration {
    fn from(value: Slack) -> Self {
        value.duration()
    }
}

impl fmt::Display for Slack {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "slack:{}", self.0.value())
    }
}

// =============================================================================
// Operation reference
// =============================================================================

/// Scheduler reference to a canonical IR operation.
///
/// The scheduler never owns or recreates the semantic operation identity.
///
/// `OperationId` comes directly from `quantum::ir::core::identity`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct OperationRef {
    id: OperationId,
}

impl OperationRef {
    /// Creates a scheduler operation reference.
    #[must_use]
    pub const fn new(id: OperationId) -> Self {
        Self { id }
    }

    /// Returns the canonical IR operation identity.
    #[must_use]
    pub const fn id(self) -> OperationId {
        self.id
    }
}

impl From<OperationId> for OperationRef {
    fn from(value: OperationId) -> Self {
        Self::new(value)
    }
}

impl From<OperationRef> for OperationId {
    fn from(value: OperationRef) -> Self {
        value.id()
    }
}

impl fmt::Display for OperationRef {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.id)
    }
}

// =============================================================================
// Logical qubit reference
// =============================================================================

/// Scheduler reference to a canonical logical qubit.
///
/// This is intentionally a wrapper rather than a new qubit identity.
///
/// The actual identity remains `quantum::ir::qubit::QubitId`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct LogicalQubitRef {
    id: QubitId,
}

impl LogicalQubitRef {
    /// Creates a scheduler logical-qubit reference.
    #[must_use]
    pub const fn new(id: QubitId) -> Self {
        Self { id }
    }

    /// Returns the canonical logical-qubit identity.
    #[must_use]
    pub const fn id(self) -> QubitId {
        self.id
    }
}

impl From<QubitId> for LogicalQubitRef {
    fn from(value: QubitId) -> Self {
        Self::new(value)
    }
}

impl From<LogicalQubitRef> for QubitId {
    fn from(value: LogicalQubitRef) -> Self {
        value.id()
    }
}

impl fmt::Display for LogicalQubitRef {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "logical:{}", self.id)
    }
}

// =============================================================================
// Physical qubit reference
// =============================================================================

/// Scheduler reference to a canonical physical qubit.
///
/// Routing/hardware adapters are responsible for determining which physical
/// qubit corresponds to a logical qubit.
///
/// Scheduling only consumes the resulting identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PhysicalQubitRef {
    id: PhysicalQubitId,
}

impl PhysicalQubitRef {
    /// Creates a scheduler physical-qubit reference.
    #[must_use]
    pub const fn new(id: PhysicalQubitId) -> Self {
        Self { id }
    }

    /// Returns the canonical physical-qubit identity.
    #[must_use]
    pub const fn id(self) -> PhysicalQubitId {
        self.id
    }
}

impl From<PhysicalQubitId> for PhysicalQubitRef {
    fn from(value: PhysicalQubitId) -> Self {
        Self::new(value)
    }
}

impl From<PhysicalQubitRef> for PhysicalQubitId {
    fn from(value: PhysicalQubitRef) -> Self {
        value.id()
    }
}

impl fmt::Display for PhysicalQubitRef {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "physical:{}", self.id)
    }
}

// =============================================================================
// Resource reference
// =============================================================================

/// Scheduler reference to a canonical IR resource.
///
/// The scheduler does not assume that a resource is a qubit.
///
/// Resources may represent:
///
/// - control channels;
/// - measurement channels;
/// - couplers;
/// - resonators;
/// - communication links;
/// - classical processing resources;
/// - memory;
/// - arbitrary target-defined resources.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ResourceRef {
    id: ResourceId,
}

impl ResourceRef {
    /// Creates a scheduler resource reference.
    #[must_use]
    pub const fn new(id: ResourceId) -> Self {
        Self { id }
    }

    /// Returns the canonical IR resource identity.
    #[must_use]
    pub const fn id(self) -> ResourceId {
        self.id
    }
}

impl From<ResourceId> for ResourceRef {
    fn from(value: ResourceId) -> Self {
        Self::new(value)
    }
}

impl From<ResourceRef> for ResourceId {
    fn from(value: ResourceRef) -> Self {
        value.id()
    }
}

impl fmt::Display for ResourceRef {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.id)
    }
}

// =============================================================================
// Dependency kind
// =============================================================================

/// Semantic class of a scheduling dependency.
///
/// This classification describes why one operation must precede another.
///
/// It does not itself determine whether the dependency is sufficient for
/// quantum semantic correctness; the IR and verification layers remain
/// authoritative for semantic validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum DependencyKind {
    /// Quantum data dependency.
    QuantumData,

    /// Classical data dependency.
    ClassicalData,

    /// Measurement result dependency.
    Measurement,

    /// Classical control/condition dependency.
    Control,

    /// Reset-before-use dependency.
    Reset,

    /// Resource serialization dependency.
    Resource,

    /// Communication dependency.
    Communication,

    /// Explicit user/program ordering dependency.
    Explicit,

    /// Target-defined dependency.
    Custom,
}

impl DependencyKind {
    /// Returns whether this dependency originates from resource occupancy.
    #[must_use]
    pub const fn is_resource(self) -> bool {
        matches!(self, Self::Resource)
    }

    /// Returns whether this dependency is data/control semantic rather than
    /// merely resource serialization.
    #[must_use]
    pub const fn is_semantic(self) -> bool {
        matches!(
            self,
            Self::QuantumData
                | Self::ClassicalData
                | Self::Measurement
                | Self::Control
                | Self::Reset
                | Self::Communication
                | Self::Explicit
        )
    }
}

// =============================================================================
// Dependency reference
// =============================================================================

/// Stable scheduling dependency edge.
///
/// `from` must precede `to` when the dependency is active.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DependencyRef {
    id: DependencyId,
    from: OperationRef,
    to: OperationRef,
    kind: DependencyKind,
}

impl DependencyRef {
    /// Creates a dependency.
    ///
    /// Self-dependencies are rejected because they cannot represent a useful
    /// strict scheduling relationship.
    pub const fn new(
        id: DependencyId,
        from: OperationRef,
        to: OperationRef,
        kind: DependencyKind,
    ) -> Option<Self> {
        if from.id() == to.id() {
            None
        } else {
            Some(Self {
                id,
                from,
                to,
                kind,
            })
        }
    }

    /// Returns the dependency identity.
    #[must_use]
    pub const fn id(self) -> DependencyId {
        self.id
    }

    /// Returns the predecessor operation.
    #[must_use]
    pub const fn from(self) -> OperationRef {
        self.from
    }

    /// Returns the successor operation.
    #[must_use]
    pub const fn to(self) -> OperationRef {
        self.to
    }

    /// Returns the dependency kind.
    #[must_use]
    pub const fn kind(self) -> DependencyKind {
        self.kind
    }
}

// =============================================================================
// Resource usage
// =============================================================================

/// Non-negative amount of a resource consumed by an operation.
///
/// Capacity semantics belong to the resource subsystem.
///
/// A resource with capacity `1` may use `ResourceAmount::ONE`.
///
/// A resource with larger capacity may reserve more than one unit without
/// changing the scheduler type system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct ResourceAmount(u128);

impl ResourceAmount {
    /// Zero usage.
    pub const ZERO: Self = Self(0);

    /// One resource unit.
    pub const ONE: Self = Self(1);

    /// Creates a resource amount.
    #[must_use]
    pub const fn new(value: u128) -> Self {
        Self(value)
    }

    /// Returns the amount.
    #[must_use]
    pub const fn value(self) -> u128 {
        self.0
    }

    /// Returns whether the amount is zero.
    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.0 == 0
    }

    /// Checked addition.
    #[must_use]
    pub const fn checked_add(self, other: Self) -> Option<Self> {
        match self.0.checked_add(other.0) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }

    /// Returns whether this usage exceeds the supplied capacity.
    #[must_use]
    pub const fn exceeds(self, capacity: Self) -> bool {
        self.0 > capacity.0
    }
}

impl From<u128> for ResourceAmount {
    fn from(value: u128) -> Self {
        Self::new(value)
    }
}

impl From<ResourceAmount> for u128 {
    fn from(value: ResourceAmount) -> Self {
        value.value()
    }
}

impl fmt::Display for ResourceAmount {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "resource-amount:{}", self.0)
    }
}

// =============================================================================
// Resource reservation
// =============================================================================

/// Immutable foundational description of one resource reservation.
///
/// More detailed reservation metadata belongs to `resources::reservation`.
///
/// This type deliberately contains only stable foundational data so it can be
/// shared by planners, validators, diagnostics, and serializers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ReservationRef {
    id: ReservationId,
    operation: OperationRef,
    resource: ResourceRef,
    interval: TimeInterval,
    amount: ResourceAmount,
}

impl ReservationRef {
    /// Creates a reservation reference.
    #[must_use]
    pub const fn new(
        id: ReservationId,
        operation: OperationRef,
        resource: ResourceRef,
        interval: TimeInterval,
        amount: ResourceAmount,
    ) -> Self {
        Self {
            id,
            operation,
            resource,
            interval,
            amount,
        }
    }

    /// Returns the reservation identity.
    #[must_use]
    pub const fn id(self) -> ReservationId {
        self.id
    }

    /// Returns the operation consuming the resource.
    #[must_use]
    pub const fn operation(self) -> OperationRef {
        self.operation
    }

    /// Returns the reserved resource.
    #[must_use]
    pub const fn resource(self) -> ResourceRef {
        self.resource
    }

    /// Returns the reserved interval.
    #[must_use]
    pub const fn interval(self) -> TimeInterval {
        self.interval
    }

    /// Returns the reserved amount.
    #[must_use]
    pub const fn amount(self) -> ResourceAmount {
        self.amount
    }
}

// =============================================================================
// Schedule state
// =============================================================================

/// Lifecycle state of a scheduling operation/session.
///
/// This is scheduler bookkeeping.
///
/// It is not an execution state of a quantum processor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ScheduleState {
    /// Scheduling has been created but planning has not started.
    Created,

    /// Dependency/resource analysis is in progress.
    Analyzing,

    /// A schedule is being constructed.
    Planning,

    /// The schedule has been constructed and awaits verification.
    Planned,

    /// Verification has succeeded.
    Verified,

    /// The schedule has been transformed/optimized after verification.
    Optimized,

    /// The final schedule is ready for downstream lowering.
    Finalized,

    /// Scheduling was intentionally cancelled.
    Cancelled,
}

impl ScheduleState {
    /// Returns whether this state represents a successfully constructed
    /// schedule.
    #[must_use]
    pub const fn has_schedule(self) -> bool {
        matches!(
            self,
            Self::Planned
                | Self::Verified
                | Self::Optimized
                | Self::Finalized
        )
    }

    /// Returns whether downstream execution/lowering may consume the result.
    ///
    /// Verification is intentionally required before `Finalized`.
    #[must_use]
    pub const fn is_final(self) -> bool {
        matches!(self, Self::Finalized)
    }

    /// Returns whether this is a terminal lifecycle state.
    #[must_use]
    pub const fn is_terminal(self) -> bool {
        matches!(self, Self::Finalized | Self::Cancelled)
    }
}

impl Default for ScheduleState {
    fn default() -> Self {
        Self::Created
    }
}

// =============================================================================
// Scheduling phase
// =============================================================================

/// Coarse scheduling phase.
///
/// The phase is descriptive and does not dictate a particular implementation
/// algorithm.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SchedulingPhase {
    /// Build/validate the operation representation.
    Preparation,

    /// Analyze dependencies.
    DependencyAnalysis,

    /// Analyze resources.
    ResourceAnalysis,

    /// Analyze timing constraints.
    TimingAnalysis,

    /// Construct the schedule.
    Planning,

    /// Apply target alignment.
    Alignment,

    /// Apply explicit delays/padding or other scheduling transformations.
    Transformation,

    /// Verify correctness.
    Verification,

    /// Optimize an already valid schedule.
    Optimization,

    /// Produce the final immutable scheduling artifact.
    Finalization,
}

impl SchedulingPhase {
    /// Returns whether the phase is part of correctness verification.
    #[must_use]
    pub const fn is_verification(self) -> bool {
        matches!(self, Self::Verification)
    }

    /// Returns whether the phase may alter schedule placement.
    #[must_use]
    pub const fn may_modify_schedule(self) -> bool {
        matches!(
            self,
            Self::Planning
                | Self::Alignment
                | Self::Transformation
                | Self::Optimization
        )
    }
}

// =============================================================================
// Operation timing
// =============================================================================

/// Fundamental timing assignment for one operation.
///
/// This is intentionally separate from the IR operation itself.
///
/// It records scheduling placement, not operation semantics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct OperationTiming {
    operation: OperationRef,
    interval: TimeInterval,
}

impl OperationTiming {
    /// Creates an operation timing assignment.
    #[must_use]
    pub const fn new(
        operation: OperationRef,
        interval: TimeInterval,
    ) -> Self {
        Self {
            operation,
            interval,
        }
    }

    /// Returns the operation.
    #[must_use]
    pub const fn operation(self) -> OperationRef {
        self.operation
    }

    /// Returns the scheduled interval.
    #[must_use]
    pub const fn interval(self) -> TimeInterval {
        self.interval
    }

    /// Returns the scheduled start.
    #[must_use]
    pub const fn start(self) -> TimePoint {
        self.interval.start()
    }

    /// Returns the scheduled end.
    #[must_use]
    pub const fn end(self) -> TimePoint {
        self.interval.end()
    }

    /// Returns the scheduled duration.
    #[must_use]
    pub const fn duration(self) -> Duration {
        self.interval.duration()
    }
}

// =============================================================================
// Schedule metadata
// =============================================================================

/// Stable scheduler metadata.
///
/// Metadata is deliberately limited to foundational scalar information.
///
/// Human-readable diagnostics, arbitrary annotations, provenance graphs, and
/// target-specific metadata belong to the appropriate diagnostics/metadata
/// subsystems.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ScheduleMetadata {
    /// Scheduler session that produced the schedule.
    pub session: SchedulerSessionId,

    /// Epoch at which this schedule was planned.
    pub epoch: EpochId,

    /// Current schedule lifecycle state.
    pub state: ScheduleState,
}

impl ScheduleMetadata {
    /// Creates initial metadata.
    #[must_use]
    pub const fn new(
        session: SchedulerSessionId,
        epoch: EpochId,
    ) -> Self {
        Self {
            session,
            epoch,
            state: ScheduleState::Created,
        }
    }

    /// Returns metadata with a new lifecycle state.
    #[must_use]
    pub const fn with_state(self, state: ScheduleState) -> Self {
        Self { state, ..self }
    }
}

// =============================================================================
// Schedule summary
// =============================================================================

/// Compact deterministic summary of a schedule.
///
/// This type does not own the complete schedule.
///
/// It is intended for:
///
/// - diagnostics;
/// - benchmarking;
/// - logging;
/// - cache keys/metadata;
/// - validation summaries.
///
/// Counts are descriptive and do not impose limits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ScheduleSummary {
    /// Number of scheduled operations.
    pub operation_count: u64,

    /// Number of dependency edges.
    pub dependency_count: u64,

    /// Number of resource reservations.
    pub reservation_count: u64,

    /// Number of distinct logical qubits represented.
    pub logical_qubit_count: u64,

    /// Number of distinct physical qubits represented.
    pub physical_qubit_count: u64,

    /// Number of resource identities represented.
    pub resource_count: u64,

    /// Total schedule makespan.
    pub makespan: Makespan,
}

impl ScheduleSummary {
    /// Creates an empty summary.
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            operation_count: 0,
            dependency_count: 0,
            reservation_count: 0,
            logical_qubit_count: 0,
            physical_qubit_count: 0,
            resource_count: 0,
            makespan: Makespan::ZERO,
        }
    }

    /// Returns whether the summary describes an operation-free schedule.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.operation_count == 0
    }
}

// =============================================================================
// Scheduling objective direction
// =============================================================================

/// Direction of an optimization objective.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ObjectiveDirection {
    /// Lower values are better.
    Minimize,

    /// Higher values are better.
    Maximize,
}

impl ObjectiveDirection {
    /// Returns whether the objective prefers lower values.
    #[must_use]
    pub const fn is_minimize(self) -> bool {
        matches!(self, Self::Minimize)
    }

    /// Returns whether the objective prefers higher values.
    #[must_use]
    pub const fn is_maximize(self) -> bool {
        matches!(self, Self::Maximize)
    }
}

// =============================================================================
// Objective value
// =============================================================================

/// Generic scheduler objective value.
///
/// The scheduling framework deliberately does not assign a universal unit to
/// objective scores.
///
/// Concrete objective modules define their interpretation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct ObjectiveValue(i128);

impl ObjectiveValue {
    /// Neutral objective value.
    pub const ZERO: Self = Self(0);

    /// Creates an objective value.
    #[must_use]
    pub const fn new(value: i128) -> Self {
        Self(value)
    }

    /// Returns the value.
    #[must_use]
    pub const fn value(self) -> i128 {
        self.0
    }

    /// Checked addition.
    #[must_use]
    pub const fn checked_add(self, other: Self) -> Option<Self> {
        match self.0.checked_add(other.0) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }

    /// Checked subtraction.
    #[must_use]
    pub const fn checked_sub(self, other: Self) -> Option<Self> {
        match self.0.checked_sub(other.0) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }
}

impl From<i128> for ObjectiveValue {
    fn from(value: i128) -> Self {
        Self::new(value)
    }
}

impl From<ObjectiveValue> for i128 {
    fn from(value: ObjectiveValue) -> Self {
        value.value()
    }
}

impl fmt::Display for ObjectiveValue {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "objective:{}", self.0)
    }
}

// =============================================================================
// Tests
// =============================================================================
//
// These tests intentionally cover only foundational invariants owned by this
// file. Cross-module behavior belongs in scheduling integration/property tests.
//
// The tests contain no assumptions about:
// - number of qubits;
// - number of operations;
// - hardware topology;
// - timing units;
// - scheduler algorithm;
// - vendor;
// - QEC code.
//
// This keeps them valid as the rest of the scheduling subsystem evolves.
//

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scheduler_ids_are_typed_and_stable() {
        let schedule = ScheduleId::new(42);
        let dependency = DependencyId::new(42);

        assert_eq!(schedule.value(), 42);
        assert_eq!(dependency.value(), 42);
        assert_ne!(
            format!("{schedule}"),
            format!("{dependency}")
        );
    }

    #[test]
    fn scheduler_id_checked_next_prevents_overflow() {
        let id = ScheduleId::new(u64::MAX);

        assert!(id.checked_next().is_none());
    }

    #[test]
    fn time_point_checked_add_is_safe() {
        let start = TimePoint::new(u128::MAX);
        let duration = Duration::ONE;

        assert!(start.checked_add(duration).is_none());
    }

    #[test]
    fn duration_checked_sub_is_safe() {
        let small = Duration::new(1);
        let large = Duration::new(2);

        assert!(small.checked_sub(large).is_none());
    }

    #[test]
    fn adjacent_intervals_do_not_overlap() {
        let first = TimeInterval::new(
            TimePoint::new(0),
            TimePoint::new(10),
        )
        .expect("valid interval");

        let second = TimeInterval::new(
            TimePoint::new(10),
            TimePoint::new(20),
        )
        .expect("valid interval");

        assert!(!first.overlaps(second));
        assert!(first.is_before(second));
        assert!(second.is_after(first));
    }

    #[test]
    fn_overlapping_intervals_are_detected() {
        let first = TimeInterval::new(
            TimePoint::new(0),
            TimePoint::new(10),
        )
        .expect("valid interval");

        let second = TimeInterval::new(
            TimePoint::new(9),
            TimePoint::new(20),
        )
        .expect("valid interval");

        assert!(first.overlaps(second));
    }

    #[test]
    fn invalid_interval_is_rejected() {
        let interval = TimeInterval::new(
            TimePoint::new(10),
            TimePoint::new(9),
        );

        assert!(interval.is_none());
    }

    #[test]
    fn zero_duration_interval_is_valid() {
        let interval = TimeInterval::new(
            TimePoint::new(10),
            TimePoint::new(10),
        )
        .expect("zero-duration interval is valid");

        assert!(interval.is_empty());
        assert_eq!(interval.duration(), Duration::ZERO);
    }

    #[test]
    fn interval_from_duration_is_checked() {
        let start = TimePoint::new(10);
        let duration = Duration::new(5);

        let interval =
            TimeInterval::from_duration(start, duration)
                .expect("valid interval");

        assert_eq!(interval.start(), TimePoint::new(10));
        assert_eq!(interval.end(), TimePoint::new(15));
        assert_eq!(interval.duration(), duration);
    }

    #[test]
    fn interval_from_duration_detects_overflow() {
        let start = TimePoint::new(u128::MAX);
        let duration = Duration::ONE;

        assert!(
            TimeInterval::from_duration(start, duration)
                .is_none()
        );
    }

    #[test]
    fn dependency_rejects_self_edge() {
        let operation = OperationRef::new(OperationId::new(1));
        let dependency = DependencyRef::new(
            DependencyId::new(1),
            operation,
            operation,
            DependencyKind::Explicit,
        );

        assert!(dependency.is_none());
    }

    #[test]
    fn dependency_accepts_distinct_operations() {
        let first = OperationRef::new(OperationId::new(1));
        let second = OperationRef::new(OperationId::new(2));

        let dependency = DependencyRef::new(
            DependencyId::new(1),
            first,
            second,
            DependencyKind::QuantumData,
        )
        .expect("distinct operations");

        assert_eq!(dependency.from(), first);
        assert_eq!(dependency.to(), second);
        assert_eq!(
            dependency.kind(),
            DependencyKind::QuantumData
        );
    }

    #[test]
    fn canonical_qubit_types_are_used_without_redefinition() {
        let logical = LogicalQubitRef::new(QubitId::new(7));
        let physical =
            PhysicalQubitRef::new(PhysicalQubitId::new(11));

        assert_eq!(logical.id(), QubitId::new(7));
        assert_eq!(
            physical.id(),
            PhysicalQubitId::new(11)
        );
    }

    #[test]
    fn resource_amount_has_no_fixed_capacity() {
        let amount = ResourceAmount::new(u128::MAX);

        assert_eq!(amount.value(), u128::MAX);
        assert!(!amount.exceeds(amount));
    }

    #[test]
    fn schedule_summary_empty_is_consistent() {
        let summary = ScheduleSummary::empty();

        assert!(summary.is_empty());
        assert_eq!(summary.operation_count, 0);
        assert_eq!(summary.dependency_count, 0);
        assert_eq!(summary.reservation_count, 0);
        assert_eq!(summary.makespan, Makespan::ZERO);
    }

    #[test]
    fn schedule_state_progression_is_descriptive() {
        assert!(!ScheduleState::Created.has_schedule());
        assert!(!ScheduleState::Planning.has_schedule());

        assert!(ScheduleState::Planned.has_schedule());
        assert!(ScheduleState::Verified.has_schedule());
        assert!(ScheduleState::Optimized.has_schedule());
        assert!(ScheduleState::Finalized.has_schedule());

        assert!(ScheduleState::Finalized.is_final());
        assert!(ScheduleState::Finalized.is_terminal());
        assert!(ScheduleState::Cancelled.is_terminal());
    }

    #[test]
    fn resource_reservation_retains_foundational_identity() {
        let operation =
            OperationRef::new(OperationId::new(5));

        let resource =
            ResourceRef::new(ResourceId::new(9));

        let interval = TimeInterval::new(
            TimePoint::new(100),
            TimePoint::new(150),
        )
        .expect("valid interval");

        let reservation = ReservationRef::new(
            ReservationId::new(77),
            operation,
            resource,
            interval,
            ResourceAmount::ONE,
        );

        assert_eq!(
            reservation.id(),
            ReservationId::new(77)
        );
        assert_eq!(reservation.operation(), operation);
        assert_eq!(reservation.resource(), resource);
        assert_eq!(reservation.interval(), interval);
        assert_eq!(
            reservation.amount(),
            ResourceAmount::ONE
        );
    }

    #[test]
    fn objective_values_are_signed_and_checked() {
        let positive = ObjectiveValue::new(10);
        let negative = ObjectiveValue::new(-3);

        assert_eq!(
            positive.checked_add(negative),
            Some(ObjectiveValue::new(7))
        );
    }

    #[test]
    fn no_negative_slack_can_be_constructed() {
        let slack = Slack::new(Duration::new(0));

        assert_eq!(slack.value(), 0);
        assert!(slack.is_zero());
    }
}