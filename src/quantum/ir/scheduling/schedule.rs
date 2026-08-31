//! Zamani Quantum IR — Production Schedule Representation
//!
//! Path:
//!     src/quantum/ir/scheduling/schedule.rs
//!
//! # Purpose
//!
//! This module defines the canonical, target-independent representation of a
//! completed scheduling result for the Zamani Quantum IR.
//!
//! A schedule answers:
//!
//!     WHEN
//!
//! semantic IR operations are intended to occur, and which abstract resources
//! they occupy during that interval.
//!
//! It does NOT decide:
//!
//! - which operations should be scheduled;
//! - how operations are optimized;
//! - how logical qubits are routed;
//! - which physical device is selected;
//! - which hardware-native instruction is used;
//! - how a device calibration is applied;
//! - how pulses are synthesized;
//! - how a QPU is contacted;
//! - how the program is executed.
//!
//! Those responsibilities belong to downstream scheduling, routing, hardware,
//! lowering and backend subsystems.
//!
//! # Architectural boundary
//!
//! ```text
//!                         Zamani source
//!                              |
//!                              v
//!                           frontend
//!                              |
//!                              v
//!                    canonical quantum::ir
//!                              |
//!              +---------------+---------------+
//!              |               |               |
//!              v               v               v
//!         optimization       routing       scheduling
//!                                              |
//!                                              v
//!                                    scheduling::schedule
//!                                              |
//!                                              v
//!                                           hardware
//!                                              |
//!                                              v
//!                                           backend
//! ```
//!
//! The schedule is therefore a *result representation*, not a scheduling
//! policy.
//!
//! # Universal-program principle
//!
//! A Zamani program is written once at the semantic level.
//!
//! The same program may ultimately be mapped to:
//!
//! - a single-qubit device;
//! - a small QPU;
//! - a large QPU;
//! - a fault-tolerant machine;
//! - a distributed quantum system;
//! - a pulse-controlled processor;
//! - an analog processor;
//! - an annealing system;
//! - a simulator;
//! - a future quantum architecture.
//!
//! This module therefore contains no fixed:
//!
//! - qubit count;
//! - operation count;
//! - channel count;
//! - resource count;
//! - schedule depth;
//! - topology;
//! - hardware vendor;
//! - backend;
//! - machine size.
//!
//! The representable domain is instead bounded only by the Rust/platform
//! representation and explicit resource/security policies supplied by callers.
//!
//! "Infinity" in the architectural requirement means:
//!
//!     no semantic fixed upper bound
//!
//! rather than an impossible promise that a finite machine or finite address
//! space can contain infinitely many objects.
//!
//! # Canonical dependencies
//!
//! This file deliberately consumes canonical types from other IR modules:
//!
//! ```text
//! quantum::ir::identity
//!     ScheduleId
//!     OperationId
//!
//! quantum::ir::qubit
//!     QubitId
//!     PhysicalQubitId
//!
//! quantum::ir::timing
//!     Duration
//!     TimePoint
//!     TimeInterval
//!
//! quantum::ir::timing::dependency
//!     TemporalDependency
//! ```
//!
//! This module does not redefine any of them.
//!
//! # Ownership
//!
//! This file owns:
//!
//! - Schedule;
//! - ScheduledOperation;
//! - ScheduleResource;
//! - ScheduleResourceKind;
//! - ScheduleEntry;
//! - schedule-local synchronization markers;
//! - deterministic schedule ordering;
//! - schedule-local validation;
//! - schedule-local resource conflict detection;
//! - schedule-local checked insertion;
//! - schedule-local duration/span accounting.
//!
//! It does NOT own:
//!
//! - Operation;
//! - Gate;
//! - Qubit;
//! - TimePoint;
//! - Duration;
//! - TimeInterval;
//! - TemporalDependency;
//! - routing;
//! - hardware topology;
//! - hardware calibration;
//! - scheduler algorithms.
//!
//! # Important identity rule
//!
//! `OperationId` identifies the semantic operation.
//!
//! Schedule position is NOT the operation identity.
//!
//! Therefore moving an operation from:
//!
//!     [0, 10)
//!
//! to:
//!
//!     [100, 110)
//!
//! does not change its `OperationId`.
//!
//! # Logical and physical qubits
//!
//! Logical and physical qubit identities are imported from:
//!
//!     quantum::ir::qubit
//!
//! They are never recreated here.
//!
//! A logical qubit represents the program-level identity.
//!
//! A physical qubit represents a target-level placement record.
//!
//! This module does not decide that mapping.
//!
//! # Resource semantics
//!
//! A schedule may record occupancy of:
//!
//! - logical qubits;
//! - physical qubits;
//! - abstract channels;
//! - abstract frames;
//! - generic resource identities.
//!
//! Resource identity does not imply hardware implementation.
//!
//! For example, `ChannelId(7)` is not automatically "DAC channel 7".
//!
//! # Determinism
//!
//! Schedule ordering is canonical and independent of insertion order.
//!
//! Entries are ordered by:
//!
//! 1. interval start;
//! 2. interval end;
//! 3. operation identity;
//! 4. deterministic resource ordering.
//!
//! No `HashMap` iteration order is used to define semantic schedule order.
//!
//! # Time semantics
//!
//! This module does not invent another time representation.
//!
//! It consumes the canonical timing model from:
//!
//!     quantum::ir::timing
//!
//! In particular:
//!
//! - `TimePoint` is an absolute semantic time;
//! - `Duration` is elapsed semantic time;
//! - `TimeInterval` is a validated half-open interval `[start, end)`.
//!
//! This avoids duplicate timing implementations and ensures that the timing
//! model remains shared by pulse, operation, validation, analysis and
//! scheduling.
//!
//! # Resource conflicts
//!
//! Two entries conflict when:
//!
//!     resource(A) == resource(B)
//!
//! and:
//!
//!     interval(A).overlaps(interval(B))
//!
//! Under half-open interval semantics:
//!
//!     [0, 10)
//!     [10, 20)
//!
//! do not conflict.
//!
//! Resource conflict checking is intentionally optional during construction.
//! This allows callers to construct schedules incrementally while preserving
//! an explicit validation API.
//!
//! # Partial schedules
//!
//! A schedule can be constructed incrementally.
//!
//! It may therefore represent:
//!
//! - an empty schedule;
//! - a partial schedule;
//! - a complete schedule.
//!
//! Completeness is a property of the caller/compiler context, not an inherent
//! property of this data structure.
//!
//! # Scalability
//!
//! The schedule uses `Vec` for ordered entries and `BTreeMap`/`BTreeSet` for
//! deterministic indexes.
//!
//! There is no fixed number of operations or qubits.
//!
//! `usize` is used only for Rust collection capacities/positions.
//!
//! Semantic identities use strongly typed IDs.
//!
//! No semantic resource count is represented by a `usize` index.
//!
//! # Memory behavior
//!
//! The schedule does not silently allocate an entry for every possible qubit,
//! channel or machine resource.
//!
//! Only resources actually referenced by scheduled entries are represented.
//!
//! This is essential for sparse large systems.
//!
//! A machine with a very large resource universe therefore does not require the
//! schedule to materialize every unused resource.
//!
//! # Thread safety
//!
//! The types in this module contain ordinary owned values and do not use:
//!
//! - global mutable state;
//! - thread-local scheduling state;
//! - interior mutable global registries;
//! - unsafe code.
//!
//! They are therefore suitable for ordinary ownership-based concurrent
//! compiler architectures.
//!
//! # Serialization
//!
//! This module owns semantic schedule fields but does not define a second
//! serialization format.
//!
//! `quantum::ir::serialization` remains the canonical serialization owner.
//!
//! A canonical serializer should serialize:
//!
//! - schedule identity;
//! - IR version;
//! - entries;
//! - dependencies;
//! - synchronization markers;
//! - declared span;
//! - semantic resources;
//! - deterministic ordering.
//!
//! It must never serialize:
//!
//! - vector capacity;
//! - memory addresses;
//! - allocator state;
//! - hash-map internals;
//! - temporary compiler state.
//!
//! # Hashing
//!
//! `quantum::ir::hash` remains the canonical hashing owner.
//!
//! Schedule hashes must be derived from semantic schedule content after
//! canonical ordering.
//!
//! # Validation
//!
//! Local validation catches:
//!
//! - duplicate operation placement;
//! - invalid intervals;
//! - resource conflicts when requested;
//! - duplicate synchronization identities;
//! - invalid declared span;
//! - missing referenced operations where the caller supplies a known operation
//!   set.
//!
//! Whole-program validation remains owned by `quantum::ir::validation`.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no external dependencies;
//! - no `unsafe`.
//!
//! `#![forbid(unsafe_code)]` is intentional.
//!
//! -----------------------------------------------------------------------------
//! No scheduling algorithm belongs below this boundary.
//! -----------------------------------------------------------------------------

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use super::super::identity::{IrVersion, OperationId, ScheduleId};
use super::super::qubit::{PhysicalQubitId, QubitId};
use super::super::timing::{
    Duration,
    TimeInterval,
    TimePoint,
};

// =============================================================================
// Public result type
// =============================================================================

/// Result type for schedule operations.
pub type ScheduleResult<T> = Result<T, ScheduleError>;

// =============================================================================
// Schedule resource
// =============================================================================

/// Abstract resource occupied by a scheduled operation.
///
/// The resource is intentionally target-independent.
///
/// A resource reference does not assert that the referenced object exists on
/// hardware. Hardware existence/capability is established by the hardware
/// subsystem.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ScheduleResource {
    /// Logical program qubit.
    LogicalQubit(QubitId),

    /// Physical target qubit.
    PhysicalQubit(PhysicalQubitId),

    /// Abstract control/acquisition channel.
    Channel(u64),

    /// Abstract control frame.
    Frame(u64),

    /// Generic IR resource.
    ///
    /// The resource namespace is owned by the broader IR resource subsystem.
    Generic(u64),
}

impl ScheduleResource {
    /// Returns the resource category.
    #[must_use]
    pub const fn kind(self) -> ScheduleResourceKind {
        match self {
            Self::LogicalQubit(_) => ScheduleResourceKind::LogicalQubit,
            Self::PhysicalQubit(_) => ScheduleResourceKind::PhysicalQubit,
            Self::Channel(_) => ScheduleResourceKind::Channel,
            Self::Frame(_) => ScheduleResourceKind::Frame,
            Self::Generic(_) => ScheduleResourceKind::Generic,
        }
    }

    /// Returns the stable numeric component.
    ///
    /// This value is for deterministic ordering/diagnostics only.
    ///
    /// It must never be interpreted as a collection index or machine-size
    /// limit.
    #[must_use]
    pub const fn identity_value(self) -> u64 {
        match self {
            Self::LogicalQubit(id) => id.index() as u64,
            Self::PhysicalQubit(id) => id.index() as u64,
            Self::Channel(id) => id,
            Self::Frame(id) => id,
            Self::Generic(id) => id,
        }
    }

    /// Returns whether the resource is a qubit resource.
    #[must_use]
    pub const fn is_qubit(self) -> bool {
        matches!(
            self,
            Self::LogicalQubit(_) | Self::PhysicalQubit(_)
        )
    }
}

/// Category of a scheduled resource.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ScheduleResourceKind {
    /// Logical qubit.
    LogicalQubit,

    /// Physical qubit.
    PhysicalQubit,

    /// Abstract channel.
    Channel,

    /// Abstract frame.
    Frame,

    /// Generic IR resource.
    Generic,
}

impl fmt::Display for ScheduleResourceKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LogicalQubit => formatter.write_str("logical-qubit"),
            Self::PhysicalQubit => formatter.write_str("physical-qubit"),
            Self::Channel => formatter.write_str("channel"),
            Self::Frame => formatter.write_str("frame"),
            Self::Generic => formatter.write_str("resource"),
        }
    }
}

// =============================================================================
// Scheduled operation
// =============================================================================

/// A semantic operation placed at a concrete interval.
///
/// The actual operation semantics remain owned by the canonical operation IR.
/// This structure only records the scheduling result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScheduledOperation {
    operation_id: OperationId,
    interval: TimeInterval,
    resources: Vec<ScheduleResource>,
}

impl ScheduledOperation {
    /// Creates a scheduled operation.
    ///
    /// The interval is already validated by `TimeInterval`.
    ///
    /// Resource identities are copied and sorted into canonical order.
    ///
    /// Duplicate resources are rejected because a single scheduled operation
    /// should not represent the same semantic resource more than once.
    pub fn new(
        operation_id: OperationId,
        interval: TimeInterval,
        resources: impl IntoIterator<Item = ScheduleResource>,
    ) -> ScheduleResult<Self> {
        let mut resources: Vec<ScheduleResource> = resources.into_iter().collect();

        resources.sort();

        for pair in resources.windows(2) {
            if pair[0] == pair[1] {
                return Err(ScheduleError::DuplicateResource {
                    operation: operation_id,
                    resource: pair[0],
                });
            }
        }

        Ok(Self {
            operation_id,
            interval,
            resources,
        })
    }

    /// Creates a zero-duration scheduled operation.
    pub fn at(
        operation_id: OperationId,
        point: TimePoint,
    ) -> ScheduleResult<Self> {
        let interval = TimeInterval::at(point);

        Self::new(operation_id, interval, std::iter::empty())
    }

    /// Returns the semantic operation identity.
    #[must_use]
    pub const fn operation_id(&self) -> OperationId {
        self.operation_id
    }

    /// Returns the scheduled interval.
    #[must_use]
    pub const fn interval(&self) -> TimeInterval {
        self.interval
    }

    /// Returns the start time.
    #[must_use]
    pub const fn start(&self) -> TimePoint {
        self.interval.start()
    }

    /// Returns the end time.
    #[must_use]
    pub const fn end(&self) -> TimePoint {
        self.interval.end()
    }

    /// Returns the scheduled duration.
    #[must_use]
    pub fn duration(&self) -> Duration {
        self.interval.duration()
    }

    /// Returns whether the operation has zero duration.
    #[must_use]
    pub fn is_zero_duration(&self) -> bool {
        self.interval.is_empty()
    }

    /// Returns resources in canonical deterministic order.
    #[must_use]
    pub fn resources(&self) -> &[ScheduleResource] {
        &self.resources
    }

    /// Returns whether this operation occupies the supplied resource.
    #[must_use]
    pub fn uses_resource(&self, resource: ScheduleResource) -> bool {
        self.resources.binary_search(&resource).is_ok()
    }

    /// Returns whether this operation overlaps another scheduled operation in
    /// time and shares at least one resource.
    #[must_use]
    pub fn conflicts_with(&self, other: &Self) -> bool {
        if !self.interval.overlaps(other.interval) {
            return false;
        }

        let mut left = 0usize;
        let mut right = 0usize;

        while left < self.resources.len() && right < other.resources.len() {
            match self.resources[left].cmp(&other.resources[right]) {
                Ordering::Less => left += 1,
                Ordering::Greater => right += 1,
                Ordering::Equal => return true,
            }
        }

        false
    }
}

// =============================================================================
// Schedule entry
// =============================================================================

/// A schedule entry.
///
/// `ScheduleEntry` is intentionally an enum rather than assuming every future
/// scheduling event is a quantum operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScheduleEntry {
    /// A scheduled semantic operation.
    Operation(ScheduledOperation),

    /// A semantic synchronization marker.
    Synchronization(SynchronizationPoint),
}

impl ScheduleEntry {
    /// Returns the start time of the entry.
    #[must_use]
    pub fn start(&self) -> TimePoint {
        match self {
            Self::Operation(operation) => operation.start(),
            Self::Synchronization(point) => point.time(),
        }
    }

    /// Returns the end time of the entry.
    #[must_use]
    pub fn end(&self) -> TimePoint {
        match self {
            Self::Operation(operation) => operation.end(),
            Self::Synchronization(point) => point.time(),
        }
    }

    /// Returns the resource set occupied by this entry.
    ///
    /// Synchronization markers have no implicit hardware resource.
    #[must_use]
    pub fn resources(&self) -> &[ScheduleResource] {
        match self {
            Self::Operation(operation) => operation.resources(),
            Self::Synchronization(_) => &[],
        }
    }

    /// Returns the operation identity when the entry is an operation.
    #[must_use]
    pub fn operation_id(&self) -> Option<OperationId> {
        match self {
            Self::Operation(operation) => Some(operation.operation_id()),
            Self::Synchronization(_) => None,
        }
    }
}

// =============================================================================
// Synchronization point
// =============================================================================

/// Semantic synchronization marker.
///
/// A synchronization point has no hardware meaning by itself.
///
/// Hardware-specific realization belongs downstream.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SynchronizationPoint {
    id: u64,
    time: TimePoint,
}

impl SynchronizationPoint {
    /// Creates a synchronization point.
    ///
    /// The numeric identity is caller-owned and has no architectural limit.
    #[must_use]
    pub const fn new(id: u64, time: TimePoint) -> Self {
        Self { id, time }
    }

    /// Returns the synchronization identity.
    #[must_use]
    pub const fn id(&self) -> u64 {
        self.id
    }

    /// Returns the synchronization time.
    #[must_use]
    pub const fn time(&self) -> TimePoint {
        self.time
    }
}

// =============================================================================
// Schedule
// =============================================================================

/// Production-grade semantic schedule.
///
/// A `Schedule` represents concrete temporal placement of canonical IR
/// operations without embedding a scheduling algorithm.
///
/// # Invariants
///
/// A valid schedule:
///
/// - has one schedule identity;
/// - has one IR version;
/// - contains deterministically ordered entries;
/// - contains no duplicate operation placements;
/// - contains no duplicate synchronization IDs;
/// - contains only valid time intervals;
/// - contains no resource conflict when conflict validation is requested.
///
/// A schedule may contain zero entries.
///
/// An empty schedule is valid and useful for zero-work programs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Schedule {
    id: ScheduleId,
    ir_version: IrVersion,
    entries: Vec<ScheduleEntry>,
    operation_index: BTreeMap<OperationId, usize>,
    synchronization_index: BTreeMap<u64, usize>,
    resource_index: BTreeMap<ScheduleResource, Vec<usize>>,
    declared_span: Option<TimeInterval>,
}

impl Schedule {
    /// Creates an empty schedule using the current IR version.
    #[must_use]
    pub fn new(id: ScheduleId) -> Self {
        Self {
            id,
            ir_version: IrVersion::CURRENT,
            entries: Vec::new(),
            operation_index: BTreeMap::new(),
            synchronization_index: BTreeMap::new(),
            resource_index: BTreeMap::new(),
            declared_span: None,
        }
    }

    /// Creates an empty schedule for an explicitly supplied IR version.
    #[must_use]
    pub fn with_version(id: ScheduleId, ir_version: IrVersion) -> Self {
        Self {
            id,
            ir_version,
            entries: Vec::new(),
            operation_index: BTreeMap::new(),
            synchronization_index: BTreeMap::new(),
            resource_index: BTreeMap::new(),
            declared_span: None,
        }
    }

    /// Creates a schedule with caller-requested initial collection capacity.
    ///
    /// Capacity is an implementation optimization only.
    ///
    /// It is never a semantic limit.
    #[must_use]
    pub fn with_capacity(
        id: ScheduleId,
        capacity: usize,
    ) -> Self {
        Self {
            id,
            ir_version: IrVersion::CURRENT,
            entries: Vec::with_capacity(capacity),
            operation_index: BTreeMap::new(),
            synchronization_index: BTreeMap::new(),
            resource_index: BTreeMap::new(),
            declared_span: None,
        }
    }

    /// Returns the schedule identity.
    #[must_use]
    pub const fn id(&self) -> ScheduleId {
        self.id
    }

    /// Returns the IR version associated with this schedule.
    #[must_use]
    pub const fn ir_version(&self) -> IrVersion {
        self.ir_version
    }

    /// Returns the number of entries.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns whether the schedule contains no entries.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Returns entries in canonical schedule order.
    #[must_use]
    pub fn entries(&self) -> &[ScheduleEntry] {
        &self.entries
    }

    /// Returns the declared schedule span, when explicitly supplied.
    #[must_use]
    pub const fn declared_span(&self) -> Option<TimeInterval> {
        self.declared_span
    }

    /// Sets an explicit schedule span.
    ///
    /// The span must contain every existing entry.
    pub fn set_declared_span(
        &mut self,
        span: TimeInterval,
    ) -> ScheduleResult<()> {
        if let Some(first) = self.entries.first() {
            if span.start() > first.start() {
                return Err(ScheduleError::SpanDoesNotContainEntry {
                    start: first.start(),
                    end: first.end(),
                });
            }
        }

        if let Some(last) = self.entries.last() {
            if span.end() < last.end() {
                return Err(ScheduleError::SpanDoesNotContainEntry {
                    start: last.start(),
                    end: last.end(),
                });
            }
        }

        self.declared_span = Some(span);

        Ok(())
    }

    /// Returns the effective schedule start.
    ///
    /// For an empty schedule this returns `None`.
    #[must_use]
    pub fn start(&self) -> Option<TimePoint> {
        self.entries.first().map(ScheduleEntry::start)
    }

    /// Returns the effective schedule end.
    ///
    /// For an empty schedule this returns `None`.
    #[must_use]
    pub fn end(&self) -> Option<TimePoint> {
        self.entries.last().map(ScheduleEntry::end)
    }

    /// Returns the effective span.
    ///
    /// An explicitly declared span takes precedence over the span inferred
    /// from entries.
    #[must_use]
    pub fn span(&self) -> Option<TimeInterval> {
        if let Some(span) = self.declared_span {
            return Some(span);
        }

        let start = self.start()?;
        let end = self.end()?;

        TimeInterval::new(start, end).ok()
    }

    /// Returns the effective schedule duration.
    #[must_use]
    pub fn duration(&self) -> Option<Duration> {
        self.span().map(TimeInterval::duration)
    }

    /// Adds a scheduled operation without performing resource conflict checks.
    ///
    /// Duplicate operation identity is always rejected.
    ///
    /// Use [`Schedule::insert_checked`] when resource conflicts must be
    /// rejected during insertion.
    pub fn insert(
        &mut self,
        operation: ScheduledOperation,
    ) -> ScheduleResult<()> {
        self.insert_internal(operation, false)
    }

    /// Adds a scheduled operation and rejects overlapping resource use.
    pub fn insert_checked(
        &mut self,
        operation: ScheduledOperation,
    ) -> ScheduleResult<()> {
        self.insert_internal(operation, true)
    }

    fn insert_internal(
        &mut self,
        operation: ScheduledOperation,
        check_conflicts: bool,
    ) -> ScheduleResult<()> {
        let operation_id = operation.operation_id();

        if self.operation_index.contains_key(&operation_id) {
            return Err(ScheduleError::DuplicateOperation {
                operation: operation_id,
            });
        }

        if check_conflicts {
            self.check_operation_conflicts(&operation)?;
        }

        if let Some(span) = self.declared_span {
            if operation.start() < span.start()
                || operation.end() > span.end()
            {
                return Err(ScheduleError::SpanDoesNotContainEntry {
                    start: operation.start(),
                    end: operation.end(),
                });
            }
        }

        self.entries.push(ScheduleEntry::Operation(operation));

        self.rebuild_indexes();

        Ok(())
    }

    /// Adds a synchronization marker.
    ///
    /// Synchronization IDs must be unique within the schedule.
    pub fn insert_synchronization(
        &mut self,
        point: SynchronizationPoint,
    ) -> ScheduleResult<()> {
        if self
            .synchronization_index
            .contains_key(&point.id())
        {
            return Err(ScheduleError::DuplicateSynchronization {
                id: point.id(),
            });
        }

        if let Some(span) = self.declared_span {
            if point.time() < span.start()
                || point.time() > span.end()
            {
                return Err(ScheduleError::SpanDoesNotContainEntry {
                    start: point.time(),
                    end: point.time(),
                });
            }
        }

        self.entries.push(ScheduleEntry::Synchronization(point));

        self.rebuild_indexes();

        Ok(())
    }

    /// Returns a scheduled operation by semantic operation identity.
    #[must_use]
    pub fn operation(
        &self,
        id: OperationId,
    ) -> Option<&ScheduledOperation> {
        self.operation_index
            .get(&id)
            .and_then(|index| self.entries.get(*index))
            .and_then(|entry| match entry {
                ScheduleEntry::Operation(operation) => Some(operation),
                ScheduleEntry::Synchronization(_) => None,
            })
    }

    /// Returns the schedule entry associated with an operation.
    #[must_use]
    pub fn entry_for_operation(
        &self,
        id: OperationId,
    ) -> Option<&ScheduleEntry> {
        self.operation_index
            .get(&id)
            .and_then(|index| self.entries.get(*index))
    }

    /// Returns all entry indexes using a resource.
    ///
    /// The returned indexes refer to the schedule's canonical `entries()`
    /// ordering.
    #[must_use]
    pub fn entries_using_resource(
        &self,
        resource: ScheduleResource,
    ) -> &[usize] {
        match self.resource_index.get(&resource) {
            Some(indexes) => indexes.as_slice(),
            None => &[],
        }
    }

    /// Returns the number of entries occupying a resource.
    #[must_use]
    pub fn resource_use_count(
        &self,
        resource: ScheduleResource,
    ) -> usize {
        self.resource_index
            .get(&resource)
            .map_or(0, Vec::len)
    }

    /// Returns all resources referenced by this schedule in deterministic order.
    #[must_use]
    pub fn resources(&self) -> Vec<ScheduleResource> {
        let mut resources = BTreeSet::new();

        for entry in &self.entries {
            for resource in entry.resources() {
                resources.insert(*resource);
            }
        }

        resources.into_iter().collect()
    }

    /// Validates schedule structure without requiring a separate operation set.
    pub fn validate(&self) -> ScheduleResult<()> {
        self.validate_internal(None, false)
    }

    /// Validates schedule structure and rejects resource conflicts.
    pub fn validate_no_conflicts(&self) -> ScheduleResult<()> {
        self.validate_internal(None, true)
    }

    /// Validates schedule structure and verifies that every scheduled operation
    /// exists in the supplied operation set.
    ///
    /// The operation set is represented by an iterator of canonical
    /// `OperationId` values, allowing callers to avoid materializing another
    /// operation representation here.
    pub fn validate_against_operations<I>(
        &self,
        operation_ids: I,
    ) -> ScheduleResult<()>
    where
        I: IntoIterator<Item = OperationId>,
    {
        let known: BTreeSet<OperationId> =
            operation_ids.into_iter().collect();

        self.validate_internal(Some(&known), false)
    }

    /// Validates against a known operation set and rejects resource conflicts.
    pub fn validate_against_operations_no_conflicts<I>(
        &self,
        operation_ids: I,
    ) -> ScheduleResult<()>
    where
        I: IntoIterator<Item = OperationId>,
    {
        let known: BTreeSet<OperationId> =
            operation_ids.into_iter().collect();

        self.validate_internal(Some(&known), true)
    }

    fn validate_internal(
        &self,
        known_operations: Option<&BTreeSet<OperationId>>,
        check_conflicts: bool,
    ) -> ScheduleResult<()> {
        if !self.is_canonically_ordered() {
            return Err(ScheduleError::NonCanonicalOrder);
        }

        let mut operation_ids = BTreeSet::new();
        let mut synchronization_ids = BTreeSet::new();

        for entry in &self.entries {
            match entry {
                ScheduleEntry::Operation(operation) => {
                    if !operation_ids.insert(operation.operation_id()) {
                        return Err(ScheduleError::DuplicateOperation {
                            operation: operation.operation_id(),
                        });
                    }

                    if let Some(known) = known_operations {
                        if !known.contains(&operation.operation_id()) {
                            return Err(
                                ScheduleError::UnknownOperation {
                                    operation: operation.operation_id(),
                                },
                            );
                        }
                    }

                    for pair in operation.resources().windows(2) {
                        if pair[0] >= pair[1] {
                            return Err(
                                ScheduleError::NonCanonicalResources {
                                    operation: operation.operation_id(),
                                },
                            );
                        }
                    }
                }

                ScheduleEntry::Synchronization(point) => {
                    if !synchronization_ids.insert(point.id()) {
                        return Err(
                            ScheduleError::DuplicateSynchronization {
                                id: point.id(),
                            },
                        );
                    }
                }
            }
        }

        if let Some(span) = self.declared_span {
            for entry in &self.entries {
                if entry.start() < span.start()
                    || entry.end() > span.end()
                {
                    return Err(ScheduleError::SpanDoesNotContainEntry {
                        start: entry.start(),
                        end: entry.end(),
                    });
                }
            }
        }

        if check_conflicts {
            self.validate_no_resource_conflicts()?;
        }

        Ok(())
    }

    /// Detects every positive-duration resource conflict.
    ///
    /// The returned vector is deterministic.
    #[must_use]
    pub fn conflicts(&self) -> Vec<ScheduleConflict> {
        let mut conflicts = Vec::new();

        for entries in self.resource_index.values() {
            for left_index in 0..entries.len() {
                for right_index in (left_index + 1)..entries.len() {
                    let left = entries[left_index];
                    let right = entries[right_index];

                    let left_entry = &self.entries[left];
                    let right_entry = &self.entries[right];

                    if let (
                        ScheduleEntry::Operation(left_operation),
                        ScheduleEntry::Operation(right_operation),
                    ) = (left_entry, right_entry)
                    {
                        if left_operation
                            .interval()
                            .overlaps(right_operation.interval())
                        {
                            let shared = shared_resources(
                                left_operation.resources(),
                                right_operation.resources(),
                            );

                            for resource in shared {
                                conflicts.push(ScheduleConflict {
                                    first: left_operation.operation_id(),
                                    second: right_operation.operation_id(),
                                    resource,
                                    overlap: overlap_interval(
                                        left_operation.interval(),
                                        right_operation.interval(),
                                    ),
                                });
                            }
                        }
                    }
                }
            }
        }

        conflicts.sort();

        conflicts
    }

    fn validate_no_resource_conflicts(
        &self,
    ) -> ScheduleResult<()> {
        if let Some(conflict) = self.conflicts().into_iter().next() {
            return Err(ScheduleError::ResourceConflict {
                first: conflict.first,
                second: conflict.second,
                resource: conflict.resource,
            });
        }

        Ok(())
    }

    fn check_operation_conflicts(
        &self,
        operation: &ScheduledOperation,
    ) -> ScheduleResult<()> {
        for resource in operation.resources() {
            let indexes = match self.resource_index.get(resource) {
                Some(indexes) => indexes,
                None => continue,
            };

            for index in indexes {
                let existing = match self.entries.get(*index) {
                    Some(ScheduleEntry::Operation(existing)) => existing,
                    Some(ScheduleEntry::Synchronization(_)) | None => {
                        continue;
                    }
                };

                if existing.conflicts_with(operation) {
                    return Err(ScheduleError::ResourceConflict {
                        first: existing.operation_id(),
                        second: operation.operation_id(),
                        resource: *resource,
                    });
                }
            }
        }

        Ok(())
    }

    /// Sorts entries into canonical deterministic order.
    ///
    /// This operation does not alter operation identities.
    ///
    /// Duplicate identities are not silently removed.
    pub fn canonicalize(&mut self) -> ScheduleResult<()> {
        self.entries.sort_by(compare_entries);

        self.rebuild_indexes();

        self.validate()
    }

    /// Removes an operation by identity.
    ///
    /// Returns `true` if an operation was removed.
    pub fn remove_operation(
        &mut self,
        operation_id: OperationId,
    ) -> bool {
        let original_len = self.entries.len();

        self.entries.retain(|entry| {
            !matches!(
                entry,
                ScheduleEntry::Operation(operation)
                    if operation.operation_id() == operation_id
            )
        });

        if self.entries.len() != original_len {
            self.rebuild_indexes();
            true
        } else {
            false
        }
    }

    /// Returns all scheduled operation IDs in canonical schedule order.
    #[must_use]
    pub fn operation_ids(&self) -> Vec<OperationId> {
        self.entries
            .iter()
            .filter_map(ScheduleEntry::operation_id)
            .collect()
    }

    /// Returns all synchronization IDs in canonical schedule order.
    #[must_use]
    pub fn synchronization_ids(&self) -> Vec<u64> {
        self.entries
            .iter()
            .filter_map(|entry| match entry {
                ScheduleEntry::Synchronization(point) => Some(point.id()),
                ScheduleEntry::Operation(_) => None,
            })
            .collect()
    }

    /// Returns the critical span implied by entries.
    ///
    /// This is a structural query only. It does not perform critical-path
    /// analysis.
    #[must_use]
    pub fn inferred_span(&self) -> Option<(TimePoint, TimePoint)> {
        let first = self.entries.first()?;
        let last = self.entries.last()?;

        Some((first.start(), last.end()))
    }

    /// Rebuilds all deterministic indexes after structural mutation.
    fn rebuild_indexes(&mut self) {
        self.entries.sort_by(compare_entries);

        self.operation_index.clear();
        self.synchronization_index.clear();
        self.resource_index.clear();

        for (index, entry) in self.entries.iter().enumerate() {
            match entry {
                ScheduleEntry::Operation(operation) => {
                    self.operation_index
                        .insert(operation.operation_id(), index);

                    for resource in operation.resources() {
                        self.resource_index
                            .entry(*resource)
                            .or_default()
                            .push(index);
                    }
                }

                ScheduleEntry::Synchronization(point) => {
                    self.synchronization_index
                        .insert(point.id(), index);
                }
            }
        }
    }

    /// Returns whether entries already satisfy canonical ordering.
    #[must_use]
    pub fn is_canonically_ordered(&self) -> bool {
        self.entries
            .windows(2)
            .all(|pair| compare_entries(&pair[0], &pair[1]) != Ordering::Greater)
    }
}

impl Default for Schedule {
    fn default() -> Self {
        Self::new(ScheduleId::new(0))
    }
}

// =============================================================================
// Schedule conflict
// =============================================================================

/// Deterministic description of a resource conflict.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ScheduleConflict {
    /// First operation in canonical identity order.
    pub first: OperationId,

    /// Second operation in canonical identity order.
    pub second: OperationId,

    /// Shared resource.
    pub resource: ScheduleResource,

    /// Positive-duration overlap, when available.
    pub overlap: Option<TimeInterval>,
}

impl Ord for ScheduleConflict {
    fn cmp(&self, other: &Self) -> Ordering {
        self.first
            .cmp(&other.first)
            .then_with(|| self.second.cmp(&other.second))
            .then_with(|| self.resource.cmp(&other.resource))
            .then_with(|| self.overlap.cmp(&other.overlap))
    }
}

impl PartialOrd for ScheduleConflict {
    fn partial_cmp(
        &self,
        other: &Self,
    ) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// =============================================================================
// Schedule errors
// =============================================================================

/// Errors produced by schedule construction and validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScheduleError {
    /// An operation is already scheduled.
    DuplicateOperation {
        /// Duplicate semantic operation identity.
        operation: OperationId,
    },

    /// A synchronization identity is already present.
    DuplicateSynchronization {
        /// Duplicate synchronization identity.
        id: u64,
    },

    /// The same resource was supplied more than once to an operation.
    DuplicateResource {
        /// Operation containing the duplicate.
        operation: OperationId,

        /// Duplicated resource.
        resource: ScheduleResource,
    },

    /// An operation was not present in the caller-supplied canonical operation
    /// set.
    UnknownOperation {
        /// Missing operation identity.
        operation: OperationId,
    },

    /// Two operations overlap while using the same resource.
    ResourceConflict {
        /// First operation.
        first: OperationId,

        /// Second operation.
        second: OperationId,

        /// Conflicting resource.
        resource: ScheduleResource,
    },

    /// The schedule entries are not in canonical order.
    NonCanonicalOrder,

    /// Resources inside a scheduled operation are not canonically ordered.
    NonCanonicalResources {
        /// Operation containing non-canonical resources.
        operation: OperationId,
    },

    /// An entry lies outside the declared schedule span.
    SpanDoesNotContainEntry {
        /// Entry start.
        start: TimePoint,

        /// Entry end.
        end: TimePoint,
    },
}

impl fmt::Display for ScheduleError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::DuplicateOperation { operation } => {
                write!(
                    formatter,
                    "operation {operation} is already scheduled"
                )
            }

            Self::DuplicateSynchronization { id } => {
                write!(
                    formatter,
                    "synchronization point {id} is already present"
                )
            }

            Self::DuplicateResource {
                operation,
                resource,
            } => {
                write!(
                    formatter,
                    "operation {operation} references resource \
                     {resource:?} more than once"
                )
            }

            Self::UnknownOperation { operation } => {
                write!(
                    formatter,
                    "schedule references unknown operation {operation}"
                )
            }

            Self::ResourceConflict {
                first,
                second,
                resource,
            } => {
                write!(
                    formatter,
                    "operations {first} and {second} overlap on \
                     resource {resource:?}"
                )
            }

            Self::NonCanonicalOrder => {
                formatter.write_str(
                    "schedule entries are not in canonical order",
                )
            }

            Self::NonCanonicalResources { operation } => {
                write!(
                    formatter,
                    "resources for operation {operation} are not \
                     canonically ordered"
                )
            }

            Self::SpanDoesNotContainEntry { start, end } => {
                write!(
                    formatter,
                    "schedule entry [{start}, {end}) lies outside \
                     the declared schedule span"
                )
            }
        }
    }
}

impl Error for ScheduleError {}

// =============================================================================
// Deterministic ordering
// =============================================================================

/// Compares schedule entries without depending on insertion order.
fn compare_entries(
    left: &ScheduleEntry,
    right: &ScheduleEntry,
) -> Ordering {
    left.start()
        .cmp(&right.start())
        .then_with(|| left.end().cmp(&right.end()))
        .then_with(|| {
            match (
                left.operation_id(),
                right.operation_id(),
            ) {
                (Some(left_id), Some(right_id)) => {
                    left_id.cmp(&right_id)
                }

                (Some(_), None) => Ordering::Greater,

                (None, Some(_)) => Ordering::Less,

                (None, None) => {
                    match (left, right) {
                        (
                            ScheduleEntry::Synchronization(left_point),
                            ScheduleEntry::Synchronization(right_point),
                        ) => left_point.id().cmp(&right_point.id()),

                        _ => Ordering::Equal,
                    }
                }
            }
        })
        .then_with(|| {
            left.resources().cmp(right.resources())
        })
}

/// Computes shared resources between two already-canonical resource slices.
fn shared_resources(
    left: &[ScheduleResource],
    right: &[ScheduleResource],
) -> Vec<ScheduleResource> {
    let mut result = Vec::new();

    let mut left_index = 0usize;
    let mut right_index = 0usize;

    while left_index < left.len() && right_index < right.len() {
        match left[left_index].cmp(&right[right_index]) {
            Ordering::Less => left_index += 1,
            Ordering::Greater => right_index += 1,
            Ordering::Equal => {
                result.push(left[left_index]);
                left_index += 1;
                right_index += 1;
            }
        }
    }

    result
}

/// Computes positive-duration interval overlap.
fn overlap_interval(
    left: TimeInterval,
    right: TimeInterval,
) -> Option<TimeInterval> {
    let start = if left.start() > right.start() {
        left.start()
    } else {
        right.start()
    };

    let end = if left.end() < right.end() {
        left.end()
    } else {
        right.end()
    };

    if start < end {
        TimeInterval::new(start, end).ok()
    } else {
        None
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn operation(id: u64) -> OperationId {
        OperationId::new(id)
    }

    fn schedule(id: u64) -> Schedule {
        Schedule::new(ScheduleId::new(id))
    }

    fn time(value: u128) -> TimePoint {
        TimePoint::from_attoseconds(value)
    }

    fn interval(
        start: u128,
        end: u128,
    ) -> TimeInterval {
        TimeInterval::new(time(start), time(end))
            .expect("test interval must be valid")
    }

    #[test]
    fn empty_schedule_is_valid() {
        let schedule = schedule(1);

        assert!(schedule.is_empty());
        assert_eq!(schedule.len(), 0);
        assert!(schedule.start().is_none());
        assert!(schedule.end().is_none());
        assert!(schedule.duration().is_none());
        assert!(schedule.validate().is_ok());
    }

    #[test]
    fn scheduled_operation_preserves_operation_identity() {
        let operation = ScheduledOperation::new(
            operation(42),
            interval(100, 200),
            [ScheduleResource::LogicalQubit(QubitId::new(7))],
        )
        .expect("operation should be valid");

        assert_eq!(
            operation.operation_id(),
            OperationId::new(42)
        );
        assert_eq!(operation.start(), time(100));
        assert_eq!(operation.end(), time(200));
        assert_eq!(
            operation.duration().attoseconds(),
            100
        );
    }

    #[test]
    fn resources_are_canonicalized() {
        let operation = ScheduledOperation::new(
            operation(1),
            interval(0, 10),
            [
                ScheduleResource::Generic(3),
                ScheduleResource::Generic(1),
                ScheduleResource::Generic(2),
            ],
        )
        .expect("operation should be valid");

        assert_eq!(
            operation.resources(),
            &[
                ScheduleResource::Generic(1),
                ScheduleResource::Generic(2),
                ScheduleResource::Generic(3),
            ]
        );
    }

    #[test]
    fn duplicate_resource_is_rejected() {
        let result = ScheduledOperation::new(
            operation(1),
            interval(0, 10),
            [
                ScheduleResource::LogicalQubit(QubitId::new(1)),
                ScheduleResource::LogicalQubit(QubitId::new(1)),
            ],
        );

        assert!(matches!(
            result,
            Err(ScheduleError::DuplicateResource { .. })
        ));
    }

    #[test]
    fn duplicate_operation_is_rejected() {
        let mut schedule = schedule(1);

        let first = ScheduledOperation::new(
            operation(7),
            interval(0, 10),
            [],
        )
        .expect("valid");

        let second = ScheduledOperation::new(
            operation(7),
            interval(10, 20),
            [],
        )
        .expect("valid");

        schedule
            .insert(first)
            .expect("first insertion must succeed");

        let result = schedule.insert(second);

        assert!(matches!(
            result,
            Err(ScheduleError::DuplicateOperation { .. })
        ));
    }

    #[test]
    fn adjacent_intervals_do_not_conflict() {
        let mut schedule = schedule(1);

        let first = ScheduledOperation::new(
            operation(1),
            interval(0, 10),
            [ScheduleResource::LogicalQubit(QubitId::new(0))],
        )
        .expect("valid");

        let second = ScheduledOperation::new(
            operation(2),
            interval(10, 20),
            [ScheduleResource::LogicalQubit(QubitId::new(0))],
        )
        .expect("valid");

        schedule
            .insert_checked(first)
            .expect("first insertion must succeed");

        schedule
            .insert_checked(second)
            .expect("adjacent operation must not conflict");

        assert!(schedule.conflicts().is_empty());
    }

    #[test]
    fn overlapping_resource_use_is_rejected() {
        let mut schedule = schedule(1);

        let first = ScheduledOperation::new(
            operation(1),
            interval(0, 10),
            [ScheduleResource::LogicalQubit(QubitId::new(0))],
        )
        .expect("valid");

        let second = ScheduledOperation::new(
            operation(2),
            interval(5, 15),
            [ScheduleResource::LogicalQubit(QubitId::new(0))],
        )
        .expect("valid");

        schedule
            .insert_checked(first)
            .expect("first insertion must succeed");

        let result = schedule.insert_checked(second);

        assert!(matches!(
            result,
            Err(ScheduleError::ResourceConflict { .. })
        ));
    }

    #[test]
    fn different_resources_can_overlap() {
        let mut schedule = schedule(1);

        let first = ScheduledOperation::new(
            operation(1),
            interval(0, 10),
            [ScheduleResource::LogicalQubit(QubitId::new(0))],
        )
        .expect("valid");

        let second = ScheduledOperation::new(
            operation(2),
            interval(0, 10),
            [ScheduleResource::LogicalQubit(QubitId::new(1))],
        )
        .expect("valid");

        schedule
            .insert_checked(first)
            .expect("first insertion must succeed");

        schedule
            .insert_checked(second)
            .expect("independent resources may execute concurrently");

        assert!(schedule.conflicts().is_empty());
    }

    #[test]
    fn canonical_order_is_insertion_independent() {
        let first = ScheduledOperation::new(
            operation(1),
            interval(10, 20),
            [],
        )
        .expect("valid");

        let second = ScheduledOperation::new(
            operation(2),
            interval(0, 10),
            [],
        )
        .expect("valid");

        let mut left = schedule(1);
        left.insert(first.clone()).expect("valid");
        left.insert(second.clone()).expect("valid");

        let mut right = schedule(1);
        right.insert(second).expect("valid");
        right.insert(first).expect("valid");

        assert_eq!(left, right);
        assert!(left.is_canonically_ordered());
    }

    #[test]
    fn synchronization_points_are_deterministic() {
        let mut schedule = schedule(1);

        schedule
            .insert_synchronization(
                SynchronizationPoint::new(2, time(10)),
            )
            .expect("valid");

        schedule
            .insert_synchronization(
                SynchronizationPoint::new(1, time(10)),
            )
            .expect("valid");

        let ids = schedule.synchronization_ids();

        assert_eq!(ids, vec![1, 2]);
    }

    #[test]
    fn duplicate_synchronization_is_rejected() {
        let mut schedule = schedule(1);

        schedule
            .insert_synchronization(
                SynchronizationPoint::new(5, time(10)),
            )
            .expect("valid");

        let result = schedule.insert_synchronization(
            SynchronizationPoint::new(5, time(20)),
        );

        assert!(matches!(
            result,
            Err(ScheduleError::DuplicateSynchronization { .. })
        ));
    }

    #[test]
    fn declared_span_is_enforced() {
        let mut schedule = schedule(1);

        schedule
            .set_declared_span(interval(0, 100))
            .expect("valid span");

        let operation = ScheduledOperation::new(
            operation(1),
            interval(10, 20),
            [],
        )
        .expect("valid");

        schedule
            .insert(operation)
            .expect("operation is inside span");

        assert_eq!(
            schedule.span(),
            Some(interval(0, 100))
        );
    }

    #[test]
    fn operation_outside_declared_span_is_rejected() {
        let mut schedule = schedule(1);

        schedule
            .set_declared_span(interval(0, 100))
            .expect("valid span");

        let operation = ScheduledOperation::new(
            operation(1),
            interval(90, 110),
            [],
        )
        .expect("valid operation");

        let result = schedule.insert(operation);

        assert!(matches!(
            result,
            Err(ScheduleError::SpanDoesNotContainEntry { .. })
        ));
    }

    #[test]
    fn resource_index_is_deterministic() {
        let mut schedule = schedule(1);

        let resource =
            ScheduleResource::LogicalQubit(QubitId::new(3));

        schedule
            .insert(
                ScheduledOperation::new(
                    operation(1),
                    interval(0, 10),
                    [resource],
                )
                .expect("valid"),
            )
            .expect("insert");

        schedule
            .insert(
                ScheduledOperation::new(
                    operation(2),
                    interval(10, 20),
                    [resource],
                )
                .expect("valid"),
            )
            .expect("insert");

        assert_eq!(
            schedule.resource_use_count(resource),
            2
        );
    }

    #[test]
    fn remove_operation_rebuilds_indexes() {
        let mut schedule = schedule(1);

        let resource =
            ScheduleResource::LogicalQubit(QubitId::new(0));

        schedule
            .insert(
                ScheduledOperation::new(
                    operation(1),
                    interval(0, 10),
                    [resource],
                )
                .expect("valid"),
            )
            .expect("insert");

        assert_eq!(
            schedule.resource_use_count(resource),
            1
        );

        assert!(schedule.remove_operation(operation(1)));

        assert_eq!(
            schedule.resource_use_count(resource),
            0
        );
        assert!(schedule.operation(operation(1)).is_none());
    }

    #[test]
    fn conflict_reporting_is_deterministic() {
        let mut schedule = schedule(1);

        let resource =
            ScheduleResource::LogicalQubit(QubitId::new(0));

        schedule
            .insert(
                ScheduledOperation::new(
                    operation(2),
                    interval(5, 15),
                    [resource],
                )
                .expect("valid"),
            )
            .expect("insert");

        schedule
            .insert(
                ScheduledOperation::new(
                    operation(1),
                    interval(0, 10),
                    [resource],
                )
                .expect("valid"),
            )
            .expect("insert");

        let conflicts = schedule.conflicts();

        assert_eq!(conflicts.len(), 1);
        assert_eq!(conflicts[0].first, operation(1));
        assert_eq!(conflicts[0].second, operation(2));
        assert_eq!(
            conflicts[0].overlap,
            Some(interval(5, 10))
        );
    }

    #[test]
    fn logical_and_physical_qubits_remain_distinct() {
        let logical =
            ScheduleResource::LogicalQubit(QubitId::new(7));

        let physical =
            ScheduleResource::PhysicalQubit(
                PhysicalQubitId::new(7),
            );

        assert_ne!(logical, physical);
        assert_ne!(logical.kind(), physical.kind());
    }

    #[test]
    fn operation_validation_can_use_external_operation_set() {
        let mut schedule = schedule(1);

        schedule
            .insert(
                ScheduledOperation::new(
                    operation(10),
                    interval(0, 10),
                    [],
                )
                .expect("valid"),
            )
            .expect("insert");

        assert!(
            schedule
                .validate_against_operations(
                    [operation(10)],
                )
                .is_ok()
        );

        assert!(
            schedule
                .validate_against_operations(
                    [operation(11)],
                )
                .is_err()
        );
    }

    #[test]
    fn canonicalization_is_safe_after_manual_insertion() {
        let mut schedule = schedule(1);

        schedule
            .insert(
                ScheduledOperation::new(
                    operation(3),
                    interval(20, 30),
                    [],
                )
                .expect("valid"),
            )
            .expect("insert");

        schedule
            .insert(
                ScheduledOperation::new(
                    operation(1),
                    interval(0, 10),
                    [],
                )
                .expect("valid"),
            )
            .expect("insert");

        schedule
            .insert(
                ScheduledOperation::new(
                    operation(2),
                    interval(10, 20),
                    [],
                )
                .expect("valid"),
            )
            .expect("insert");

        schedule
            .canonicalize()
            .expect("canonicalization must succeed");

        assert_eq!(
            schedule.operation_ids(),
            vec![
                operation(1),
                operation(2),
                operation(3)
            ]
        );
    }
}