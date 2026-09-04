//! Zamani Quantum Scheduling — Resource Calendar
//!
//! This module provides the temporal calendar used by the scheduling resource
//! subsystem.
//!
//! # Architectural responsibility
//!
//! This file answers:
//!
//! > "Which intervals of a resource are occupied, blocked, reserved, or
//! > otherwise unavailable, and how can those intervals be queried safely?"
//!
//! The calendar is deliberately independent of scheduling algorithms.
//!
//! It provides temporal storage and conflict/availability queries for:
//!
//! - resource reservations;
//! - maintenance windows;
//! - calibration windows;
//! - cooldown periods;
//! - externally imposed exclusion windows;
//! - disabled intervals;
//! - resource occupancy;
//! - future target-defined temporal restrictions.
//!
//! # Ownership boundaries
//!
//! This module DOES:
//!
//! - store temporal resource entries;
//! - validate intervals;
//! - detect interval overlap;
//! - detect temporal conflicts;
//! - support capacity-aware occupancy queries;
//! - support deterministic insertion and iteration;
//! - support removal by reservation/operation identity;
//! - support temporal queries;
//! - support snapshots/cloning;
//! - provide scalable resource-local calendars;
//! - preserve canonical resource and operation identities;
//! - use half-open interval semantics;
//! - perform checked arithmetic.
//!
//! This module DOES NOT:
//!
//! - schedule operations;
//! - perform routing;
//! - discover hardware;
//! - execute quantum programs;
//! - perform calibration;
//! - define QEC;
//! - define gate semantics;
//! - define hardware-provider APIs;
//! - define scheduling policy;
//! - define dependency graphs;
//! - redefine `QubitId`;
//! - redefine `PhysicalQubitId`;
//! - redefine `ResourceId`;
//! - redefine `TimePoint`;
//! - redefine `Duration`;
//! - redefine `TimeInterval`.
//!
//! Those responsibilities belong to their canonical subsystems.
//!
//! # Canonical identity boundary
//!
//! Resource identity comes from:
//!
//! ```text
//! crate::quantum::ir::core::identity::ResourceId
//! ```
//!
//! Operation identity comes from:
//!
//! ```text
//! crate::quantum::ir::core::identity::OperationId
//! ```
//!
//! Qubit identities, when needed by higher layers, remain owned by:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! This file does not create scheduler-local replacements.
//!
//! # Time model
//!
//! The calendar uses the scheduler's canonical:
//!
//! ```text
//! crate::quantum::scheduling::types::TimePoint
//! crate::quantum::scheduling::types::Duration
//! crate::quantum::scheduling::types::TimeInterval
//! ```
//!
//! Time has no intrinsic physical unit here.
//!
//! A target timing adapter determines whether the coordinate represents:
//!
//! - device ticks;
//! - sample periods;
//! - picoseconds;
//! - nanoseconds;
//! - another exact temporal representation.
//!
//! The calendar must never contain a hardware-specific time constant.
//!
//! # Interval semantics
//!
//! Every calendar interval is half-open:
//!
//! ```text
//! [start, end)
//! ```
//!
//! Therefore:
//!
//! ```text
//! [0, 10)
//! [10, 20)
//! ```
//!
//! do not conflict.
//!
//! This is essential for deterministic scheduling at operation boundaries.
//!
//! Zero-duration intervals are legal because the canonical scheduler
//! `TimeInterval` permits them.
//!
//! Zero-duration entries never consume positive-duration capacity.
//!
//! # Scalability
//!
//! There are no artificial limits on:
//!
//! - number of resources;
//! - number of calendar entries;
//! - number of operations;
//! - number of reservations;
//! - schedule duration;
//! - number of machines;
//! - number of qubits;
//! - resource capacity.
//!
//! Collections grow dynamically according to the actual scheduling request and
//! available memory.
//!
//! "Infinity" means that this module introduces no finite machine-size ceiling.
//! A concrete compilation is necessarily bounded by the host process, address
//! space, target description, and available resources.
//!
//! The implementation avoids fixed-size arrays and machine-sized semantic
//! identifiers.
//!
//! # Complexity
//!
//! Calendar storage uses ordered maps so iteration is deterministic.
//!
//! Resource lookup:
//!
//! ```text
//! O(log R)
//! ```
//!
//! where `R` is the number of resources represented by the calendar.
//!
//! Entry insertion is approximately:
//!
//! ```text
//! O(log E)
//! ```
//!
//! for locating the resource-local insertion position, plus the number of
//! entries that must be examined for conflict detection.
//!
//! A conflict query is proportional to the relevant temporal neighborhood,
//! rather than to the total number of resources.
//!
//! The calendar deliberately does not allocate a giant time grid.
//!
//! It stores intervals/events rather than:
//!
//! ```text
//! resource × time
//! ```
//!
//! matrices.
//!
//! # Determinism
//!
//! Deterministic behavior is a first-class property.
//!
//! `BTreeMap` is used instead of `HashMap` for semantic storage order.
//!
//! Equal-time entries are ordered using their complete deterministic identity
//! tuple.
//!
//! No wall clock, random number generator, memory address, or global mutable
//! state participates in calendar behavior.
//!
//! # Thread safety
//!
//! The calendar contains ordinary owned values and no interior mutability.
//!
//! A calendar can therefore be:
//!
//! - moved between threads;
//! - shared through an immutable reference;
//! - cloned to produce an independent snapshot.
//!
//! Concurrent mutation, if desired, must be coordinated by the owning
//! scheduler. This module deliberately does not hide synchronization behind
//! the calendar abstraction.
//!
//! # Safety
//!
//! This module contains no unsafe code.
//!
//! `#![forbid(unsafe_code)]` makes that requirement compiler-enforced.
//!
//! # Rust compatibility
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
//! # Integration contract
//!
//! ```text
//! quantum::ir::core::identity\n//!             │\n//!             ├── ResourceId\n//!             └── OperationId\n//!                     │\n//!                     ▼\n//!          scheduling::types\n//!          TimePoint/Duration/\n//!          TimeInterval\n//!                     │\n//!                     ▼\n//!       resources::calendar\n//!                     │\n//!       ┌─────────────┼──────────────┐\n//!       ▼             ▼              ▼\n//! reservation    availability    planners\n//!       │             │              │\n//!       └─────────────┼──────────────┘\n//!                     ▼\n//!                 verification\n//!                     │\n//!                     ▼\n//!                  result\n//! ```
//!
//! `reservation.rs` should use this calendar for temporal placement.
//!
//! `availability.rs` should use it to express dynamic resource availability.
//!
//! Planners should query the calendar rather than implementing their own
//! interval-conflict structures.
//!
//! Hardware adapters should populate calendar entries from target availability
//! information rather than embedding vendor-specific calendar behavior here.
//!
//! # Important separation
//!
//! A `Resource` describes WHAT exists.
//!
//! A `ResourceCalendar` describes WHEN that resource is occupied or blocked.
//!
//! A `ResourceReservation` describes WHO requested the occupation.
//!
//! Therefore:
//!
//! ```text
//! resource.rs
//!     WHAT
//!       │
//!       ▼
//! calendar.rs
//!     WHEN
//!       │
//!       ▼
//! reservation.rs
//!     WHO / WHY
//! ```
//!
//! This separation allows a resource descriptor to remain immutable while its
//! temporal schedule changes.
//!
//! # Future integration
//!
//! This API intentionally provides stable primitives required by future:
//!
//! - ASAP scheduling;
//! - ALAP scheduling;
//! - list scheduling;
//! - resource-constrained scheduling;
//! - QEC scheduling;
//! - dynamic circuits;
//! - feedback scheduling;
//! - distributed scheduling;
//! - hardware adapters;
//! - ZQN-aware scheduling;
//! - runtime scheduling;
//! - schedule verification.
//!
//! Adding one of those subsystems should not require modifying this file.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use crate::quantum::ir::core::identity::{OperationId, ResourceId};

use super::super::types::{Duration, TimeInterval, TimePoint};

// =============================================================================
// Calendar entry kind
// =============================================================================

/// Semantic reason a resource calendar contains an interval.
///
/// The calendar does not interpret vendor-specific meanings. It only records
/// the semantic category supplied by the caller.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CalendarEntryKind {
    /// Resource is occupied by a scheduled operation/reservation.
    Reservation,

    /// Resource is unavailable because it is undergoing calibration.
    Calibration,

    /// Resource is unavailable because it is undergoing maintenance.
    Maintenance,

    /// Resource must remain idle for a cooldown period.
    Cooldown,

    /// Resource has been explicitly disabled for this interval.
    Disabled,

    /// Resource is excluded by an external scheduling authority.
    ExternalExclusion,

    /// Resource is occupied by an externally owned execution.
    ExternalOccupancy,

    /// Resource is intentionally held by a scheduler or runtime barrier.
    Hold,

    /// Target-defined temporal restriction.
    Custom,
}

impl CalendarEntryKind {
    /// Returns whether this entry normally prevents capacity use.
    #[must_use]
    pub const fn blocks_capacity(self) -> bool {
        !matches!(self, Self::Reservation)
    }

    /// Returns whether the entry represents an operation reservation.
    #[must_use]
    pub const fn is_reservation(self) -> bool {
        matches!(self, Self::Reservation)
    }

    /// Returns a stable machine-readable name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Reservation => "reservation",
            Self::Calibration => "calibration",
            Self::Maintenance => "maintenance",
            Self::Cooldown => "cooldown",
            Self::Disabled => "disabled",
            Self::ExternalExclusion => "external-exclusion",
            Self::ExternalOccupancy => "external-occupancy",
            Self::Hold => "hold",
            Self::Custom => "custom",
        }
    }
}

impl fmt::Display for CalendarEntryKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Calendar entry identity
// =============================================================================

/// Stable identity of a calendar entry.
///
/// Calendar entry identity is separate from operation identity and reservation
/// identity because a resource can contain temporal events that are not tied to
/// an operation.
///
/// The identity is allocated by the owning calendar/session.
///
/// The calendar never uses collection position as semantic identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CalendarEntryId(u64);

impl CalendarEntryId {
    /// Creates an explicitly supplied calendar-entry identity.
    ///
    /// This does not register the identity.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the numeric identity.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }

    /// Returns the next representable identity.
    #[must_use]
    pub const fn checked_next(self) -> Option<Self> {
        match self.0.checked_add(1) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }

    /// Returns whether this is the zero identity.
    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.0 == 0
    }
}

impl From<u64> for CalendarEntryId {
    fn from(value: u64) -> Self {
        Self::new(value)
    }
}

impl From<CalendarEntryId> for u64 {
    fn from(value: CalendarEntryId) -> Self {
        value.value()
    }
}

impl fmt::Display for CalendarEntryId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "calendar-entry:{}", self.0)
    }
}

// =============================================================================
// Calendar entry
// =============================================================================

/// One temporal entry in a resource calendar.
///
/// The entry records temporal occupancy/exclusion information without owning
/// the resource itself.
///
/// # Invariants
///
/// A valid entry has:
///
/// - a valid `TimeInterval`;
/// - a non-zero quantity when it represents capacity usage;
/// - a stable entry identity;
/// - a resource identity;
/// - optional operation provenance.
///
/// The calendar does not require every entry to have an operation identity
/// because maintenance, calibration and externally imposed exclusions are not
/// necessarily caused by quantum operations.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CalendarEntry {
    id: CalendarEntryId,
    resource_id: ResourceId,
    interval: TimeInterval,
    kind: CalendarEntryKind,
    quantity: u128,
    operation_id: Option<OperationId>,
}

impl CalendarEntry {
    /// Creates a validated calendar entry.
    ///
    /// `quantity` describes capacity usage for a reservation or occupancy.
    ///
    /// For blocking/exclusion entries the quantity is informational and may be
    /// zero.
    #[must_use]
    pub const fn new(
        id: CalendarEntryId,
        resource_id: ResourceId,
        interval: TimeInterval,
        kind: CalendarEntryKind,
        quantity: u128,
        operation_id: Option<OperationId>,
    ) -> Self {
        Self {
            id,
            resource_id,
            interval,
            kind,
            quantity,
            operation_id,
        }
    }

    /// Returns the calendar entry identity.
    #[must_use]
    pub const fn id(&self) -> CalendarEntryId {
        self.id
    }

    /// Returns the canonical resource identity.
    #[must_use]
    pub const fn resource_id(&self) -> ResourceId {
        self.resource_id
    }

    /// Returns the occupied interval.
    #[must_use]
    pub const fn interval(&self) -> TimeInterval {
        self.interval
    }

    /// Returns the semantic entry kind.
    #[must_use]
    pub const fn kind(&self) -> CalendarEntryKind {
        self.kind
    }

    /// Returns the capacity quantity represented by this entry.
    #[must_use]
    pub const fn quantity(&self) -> u128 {
        self.quantity
    }

    /// Returns the associated operation identity, if any.
    #[must_use]
    pub const fn operation_id(&self) -> Option<OperationId> {
        self.operation_id
    }

    /// Returns the interval start.
    #[must_use]
    pub const fn start(&self) -> TimePoint {
        self.interval.start()
    }

    /// Returns the interval end.
    #[must_use]
    pub const fn end(&self) -> TimePoint {
        self.interval.end()
    }

    /// Returns the interval duration.
    #[must_use]
    pub const fn duration(&self) -> Duration {
        self.interval.duration()
    }

    /// Returns whether this entry has positive duration.
    #[must_use]
    pub const fn is_non_empty(&self) -> bool {
        !self.interval.is_empty()
    }

    /// Returns whether this entry overlaps another interval.
    #[must_use]
    pub const fn overlaps(&self, interval: TimeInterval) -> bool {
        self.interval.overlaps(interval)
    }

    /// Returns whether this entry blocks resource capacity.
    #[must_use]
    pub const fn blocks_capacity(&self) -> bool {
        self.kind.blocks_capacity()
    }
}

// =============================================================================
// Calendar query
// =============================================================================

/// Query describing a proposed use of a resource.
///
/// A query does not mutate the calendar.
///
/// It can therefore be reused by multiple planning strategies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CalendarQuery {
    resource_id: ResourceId,
    interval: TimeInterval,
    quantity: u128,
}

impl CalendarQuery {
    /// Creates a query for a resource interval.
    #[must_use]
    pub const fn new(
        resource_id: ResourceId,
        interval: TimeInterval,
        quantity: u128,
    ) -> Self {
        Self {
            resource_id,
            interval,
            quantity,
        }
    }

    /// Creates a query from start time and duration.
    ///
    /// Returns `None` if time arithmetic overflows.
    #[must_use]
    pub const fn from_start_and_duration(
        resource_id: ResourceId,
        start: TimePoint,
        duration: Duration,
        quantity: u128,
    ) -> Option<Self> {
        match TimeInterval::from_duration(start, duration) {
            Some(interval) => Some(Self::new(resource_id, interval, quantity)),
            None => None,
        }
    }

    /// Returns the resource identity.
    #[must_use]
    pub const fn resource_id(self) -> ResourceId {
        self.resource_id
    }

    /// Returns the requested interval.
    #[must_use]
    pub const fn interval(self) -> TimeInterval {
        self.interval
    }

    /// Returns the requested quantity.
    #[must_use]
    pub const fn quantity(self) -> u128 {
        self.quantity
    }

    /// Returns the requested start time.
    #[must_use]
    pub const fn start(self) -> TimePoint {
        self.interval.start()
    }

    /// Returns the requested end time.
    #[must_use]
    pub const fn end(self) -> TimePoint {
        self.interval.end()
    }
}

// =============================================================================
// Conflict information
// =============================================================================

/// A conflict discovered by a calendar query.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CalendarConflict {
    resource_id: ResourceId,
    query: TimeInterval,
    existing: CalendarEntry,
    required_quantity: u128,
    occupied_quantity: u128,
}

impl CalendarConflict {
    /// Creates conflict information.
    #[must_use]
    pub fn new(
        resource_id: ResourceId,
        query: TimeInterval,
        existing: CalendarEntry,
        required_quantity: u128,
        occupied_quantity: u128,
    ) -> Self {
        Self {
            resource_id,
            query,
            existing,
            required_quantity,
            occupied_quantity,
        }
    }

    /// Returns the affected resource.
    #[must_use]
    pub const fn resource_id(&self) -> ResourceId {
        self.resource_id
    }

    /// Returns the queried interval.
    #[must_use]
    pub const fn query(&self) -> TimeInterval {
        self.query
    }

    /// Returns the conflicting entry.
    #[must_use]
    pub const fn existing(&self) -> &CalendarEntry {
        &self.existing
    }

    /// Returns the requested quantity.
    #[must_use]
    pub const fn required_quantity(&self) -> u128 {
        self.required_quantity
    }

    /// Returns the occupied quantity.
    #[must_use]
    pub const fn occupied_quantity(&self) -> u128 {
        self.occupied_quantity
    }

    /// Returns the conflicting operation, if any.
    #[must_use]
    pub const fn conflicting_operation(&self) -> Option<OperationId> {
        self.existing.operation_id()
    }

    /// Returns whether this is a blocking/exclusion conflict.
    #[must_use]
    pub const fn is_blocking(&self) -> bool {
        self.existing.blocks_capacity()
    }
}

impl fmt::Display for CalendarConflict {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "resource `{}` conflicts with calendar entry `{}` over {}",
            self.resource_id,
            self.existing.id(),
            self.existing.interval()
        )
    }
}

// =============================================================================
// Calendar errors
// =============================================================================

/// Errors returned by calendar mutation/query operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CalendarError {
    /// A calendar entry with the same identity already exists.
    DuplicateEntry {
        /// Conflicting entry identity.
        entry: CalendarEntryId,
    },

    /// The supplied entry belongs to another resource.
    ResourceMismatch {
        /// Calendar resource.
        expected: ResourceId,

        /// Entry resource.
        actual: ResourceId,
    },

    /// An entry cannot be inserted because it conflicts with an existing
    /// entry.
    Conflict {
        /// The requested resource.
        resource: ResourceId,

        /// The proposed interval.
        interval: TimeInterval,

        /// Existing conflicting entry.
        existing: CalendarEntryId,
    },

    /// Capacity arithmetic overflowed.
    CapacityOverflow {
        /// Resource involved in the calculation.
        resource: ResourceId,
    },

    /// An operation would exceed the declared capacity.
    CapacityExceeded {
        /// Resource involved.
        resource: ResourceId,

        /// Requested interval.
        interval: TimeInterval,

        /// Requested quantity.
        requested: u128,

        /// Existing/blocked quantity.
        occupied: u128,
    },

    /// An entry does not exist.
    EntryNotFound {
        /// Missing identity.
        entry: CalendarEntryId,
    },

    /// A supplied quantity is invalid for a consuming operation.
    InvalidQuantity,

    /// Calendar identity allocation reached its representable limit.
    IdentityExhausted,
}

impl fmt::Display for CalendarError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateEntry { entry } => {
                write!(formatter, "calendar entry `{entry}` already exists")
            }

            Self::ResourceMismatch { expected, actual } => {
                write!(
                    formatter,
                    "calendar resource mismatch: expected `{expected}`, got `{actual}`"
                )
            }

            Self::Conflict {
                resource,
                interval,
                existing,
            } => {
                write!(
                    formatter,
                    "resource `{resource}` interval `{interval}` conflicts with calendar entry `{existing}`"
                )
            }

            Self::CapacityOverflow { resource } => {
                write!(
                    formatter,
                    "capacity arithmetic overflow for resource `{resource}`"
                )
            }

            Self::CapacityExceeded {
                resource,
                interval,
                requested,
                occupied,
            } => {
                write!(
                    formatter,
                    "resource `{resource}` cannot satisfy quantity {requested} over {interval}; occupied quantity is {occupied}"
                )
            }

            Self::EntryNotFound { entry } => {
                write!(formatter, "calendar entry `{entry}` was not found")
            }

            Self::InvalidQuantity => {
                formatter.write_str("capacity-consuming calendar entry requires non-zero quantity")
            }

            Self::IdentityExhausted => {
                formatter.write_str("calendar entry identity space is exhausted")
            }
        }
    }
}

impl std::error::Error for CalendarError {}

// =============================================================================
// Resource-local calendar
// =============================================================================

/// Temporal calendar for one canonical scheduler resource.
///
/// A `ResourceCalendar` intentionally does not contain the full `Resource`
/// descriptor. The immutable semantic resource remains owned by
/// `resources::resource`.
///
/// This keeps calendar state independent from resource metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourceCalendar {
    resource_id: ResourceId,
    entries: BTreeMap<TimePoint, BTreeMap<CalendarEntryId, CalendarEntry>>,
    by_operation: BTreeMap<OperationId, BTreeSet<CalendarEntryId>>,
    next_entry_id: CalendarEntryId,
}

impl ResourceCalendar {
    /// Creates an empty calendar for one resource.
    #[must_use]
    pub const fn new(resource_id: ResourceId) -> Self {
        Self {
            resource_id,
            entries: BTreeMap::new(),
            by_operation: BTreeMap::new(),
            next_entry_id: CalendarEntryId::new(0),
        }
    }

    /// Returns the resource represented by this calendar.
    #[must_use]
    pub const fn resource_id(&self) -> ResourceId {
        self.resource_id
    }

    /// Returns the number of calendar entries.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries
            .values()
            .map(BTreeMap::len)
            .sum()
    }

    /// Returns whether the calendar has no entries.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Returns the next entry identity without allocating it.
    #[must_use]
    pub const fn next_entry_id(&self) -> CalendarEntryId {
        self.next_entry_id
    }

    /// Allocates the next calendar-local entry identity.
    ///
    /// This method is deterministic and checked.
    pub fn allocate_entry_id(&mut self) -> Result<CalendarEntryId, CalendarError> {
        let current = self.next_entry_id;

        let next = current
            .checked_next()
            .ok_or(CalendarError::IdentityExhausted)?;

        self.next_entry_id = next;
        Ok(current)
    }

    /// Inserts a calendar entry without automatically checking a capacity
    /// limit.
    ///
    /// Use [`Self::insert_non_conflicting`] when the caller wants calendar
    /// conflict detection.
    ///
    /// This method rejects:
    ///
    /// - duplicate entry identities;
    /// - entries belonging to another resource;
    /// - zero quantity for capacity-consuming reservations.
    pub fn insert(
        &mut self,
        entry: CalendarEntry,
    ) -> Result<(), CalendarError> {
        self.validate_entry(&entry)?;

        if entry.resource_id() != self.resource_id {
            return Err(CalendarError::ResourceMismatch {
                expected: self.resource_id,
                actual: entry.resource_id(),
            });
        }

        let id = entry.id();
        let start = entry.start();

        let bucket = self.entries.entry(start).or_default();

        if bucket.contains_key(&id) {
            return Err(CalendarError::DuplicateEntry { entry: id });
        }

        bucket.insert(id, entry.clone());

        if let Some(operation) = entry.operation_id() {
            self.by_operation
                .entry(operation)
                .or_default()
                .insert(id);
        }

        Ok(())
    }

    /// Inserts an entry only when it does not create an exclusive/blocking
    /// temporal conflict.
    ///
    /// This is the safe default for reservation-style use.
    pub fn insert_non_conflicting(
        &mut self,
        entry: CalendarEntry,
    ) -> Result<(), CalendarError> {
        self.validate_entry(&entry)?;

        if entry.resource_id() != self.resource_id {
            return Err(CalendarError::ResourceMismatch {
                expected: self.resource_id,
                actual: entry.resource_id(),
            });
        }

        if let Some(conflict) = self.first_blocking_conflict(entry.interval()) {
            return Err(CalendarError::Conflict {
                resource: self.resource_id,
                interval: entry.interval(),
                existing: conflict.id(),
            });
        }

        self.insert(entry)
    }

    /// Inserts an entry with an explicit capacity limit.
    ///
    /// Blocking entries reserve the entire requested capacity and therefore
    /// conflict with any overlapping consuming entry.
    ///
    /// Reservation entries accumulate their quantities over overlapping
    /// intervals.
    ///
    /// This method does not store the capacity itself. The caller supplies the
    /// capacity of the immutable `Resource` descriptor.
    pub fn insert_with_capacity(
        &mut self,
        entry: CalendarEntry,
        capacity: Option<u128>,
    ) -> Result<(), CalendarError> {
        self.validate_entry(&entry)?;

        if entry.resource_id() != self.resource_id {
            return Err(CalendarError::ResourceMismatch {
                expected: self.resource_id,
                actual: entry.resource_id(),
            });
        }

        self.ensure_capacity(entry.interval(), entry.quantity(), capacity)?;

        self.insert(entry)
    }

    /// Removes an entry by calendar identity.
    pub fn remove(
        &mut self,
        id: CalendarEntryId,
    ) -> Result<CalendarEntry, CalendarError> {
        let start = self.find_start(id).ok_or(
            CalendarError::EntryNotFound { entry: id },
        )?;

        let bucket = self
            .entries
            .get_mut(&start)
            .ok_or(CalendarError::EntryNotFound { entry: id })?;

        let entry = bucket
            .remove(&id)
            .ok_or(CalendarError::EntryNotFound { entry: id })?;

        if bucket.is_empty() {
            self.entries.remove(&start);
        }

        if let Some(operation) = entry.operation_id() {
            if let Some(ids) = self.by_operation.get_mut(&operation) {
                ids.remove(&id);

                if ids.is_empty() {
                    self.by_operation.remove(&operation);
                }
            }
        }

        Ok(entry)
    }

    /// Removes every entry associated with an operation.
    ///
    /// Returns the number of removed entries.
    pub fn remove_operation(&mut self, operation: OperationId) -> usize {
        let ids = match self.by_operation.remove(&operation) {
            Some(ids) => ids,
            None => return 0,
        };

        let mut removed = 0usize;

        for id in ids {
            if let Some(start) = self.find_start(id) {
                if let Some(bucket) = self.entries.get_mut(&start) {
                    if bucket.remove(&id).is_some() {
                        removed += 1;
                    }

                    if bucket.is_empty() {
                        self.entries.remove(&start);
                    }
                }
            }
        }

        removed
    }

    /// Returns an entry by identity.
    #[must_use]
    pub fn get(&self, id: CalendarEntryId) -> Option<&CalendarEntry> {
        let start = self.find_start(id)?;
        self.entries.get(&start)?.get(&id)
    }

    /// Returns all entries in deterministic temporal order.
    pub fn iter(&self) -> impl Iterator<Item = &CalendarEntry> {
        self.entries.values().flat_map(BTreeMap::values)
    }

    /// Returns all entries overlapping an interval.
    pub fn overlapping(
        &self,
        interval: TimeInterval,
    ) -> Vec<&CalendarEntry> {
        self.entries
            .range(..=interval.end())
            .flat_map(|(_, bucket)| bucket.values())
            .filter(|entry| entry.overlaps(interval))
            .collect()
    }

    /// Returns whether any entry overlaps the supplied interval.
    #[must_use]
    pub fn has_overlap(&self, interval: TimeInterval) -> bool {
        self.entries
            .range(..=interval.end())
            .flat_map(|(_, bucket)| bucket.values())
            .any(|entry| entry.overlaps(interval))
    }

    /// Returns the first deterministic blocking conflict.
    #[must_use]
    pub fn first_blocking_conflict(
        &self,
        interval: TimeInterval,
    ) -> Option<&CalendarEntry> {
        self.entries
            .range(..=interval.end())
            .flat_map(|(_, bucket)| bucket.values())
            .find(|entry| {
                entry.overlaps(interval) && entry.blocks_capacity()
            })
    }

    /// Returns all blocking entries overlapping an interval.
    pub fn blocking_entries(
        &self,
        interval: TimeInterval,
    ) -> Vec<&CalendarEntry> {
        self.entries
            .range(..=interval.end())
            .flat_map(|(_, bucket)| bucket.values())
            .filter(|entry| {
                entry.overlaps(interval) && entry.blocks_capacity()
            })
            .collect()
    }

    /// Calculates capacity occupied at a single time point.
    ///
    /// Only positive-duration overlapping entries contribute.
    ///
    /// Blocking entries contribute their declared quantity. If a blocking
    /// entry has zero quantity, it is treated as an exclusion marker and the
    /// result remains zero; callers should use `is_blocked_at` to distinguish
    /// that condition.
    pub fn occupied_quantity_at(
        &self,
        point: TimePoint,
    ) -> Result<u128, CalendarError> {
        let mut total = 0u128;

        for entry in self
            .entries
            .range(..=point)
            .flat_map(|(_, bucket)| bucket.values())
        {
            if entry.interval().contains(point) {
                total = total
                    .checked_add(entry.quantity())
                    .ok_or(CalendarError::CapacityOverflow {
                        resource: self.resource_id,
                    })?;
            }
        }

        Ok(total)
    }

    /// Returns whether the resource is blocked at a point.
    #[must_use]
    pub fn is_blocked_at(&self, point: TimePoint) -> bool {
        self.entries
            .range(..=point)
            .flat_map(|(_, bucket)| bucket.values())
            .any(|entry| {
                entry.interval().contains(point)
                    && entry.blocks_capacity()
            })
    }

    /// Returns whether the resource can satisfy a requested interval under a
    /// finite capacity.
    pub fn can_accommodate(
        &self,
        interval: TimeInterval,
        quantity: u128,
        capacity: Option<u128>,
    ) -> Result<bool, CalendarError> {
        if quantity == 0 {
            return Ok(true);
        }

        if let Some(limit) = capacity {
            if quantity > limit {
                return Ok(false);
            }
        }

        if self
            .first_blocking_conflict(interval)
            .is_some()
        {
            return Ok(false);
        }

        if capacity.is_none() {
            return Ok(true);
        }

        let limit = capacity.expect("checked above");

        self.maximum_occupied_quantity(interval)?
            .checked_add(quantity)
            .map(|total| total <= limit)
            .ok_or(CalendarError::CapacityOverflow {
                resource: self.resource_id,
            })
    }

    /// Finds the earliest time at or after `start` at which the requested
    /// interval of the supplied duration can fit.
    ///
    /// This method does not modify the calendar.
    ///
    /// `capacity == None` means scalar capacity is unlimited, subject only to
    /// blocking/exclusion entries.
    pub fn next_available(
        &self,
        start: TimePoint,
        duration: Duration,
        quantity: u128,
        capacity: Option<u128>,
    ) -> Result<Option<TimePoint>, CalendarError> {
        let mut candidate = start;

        loop {
            let interval = match TimeInterval::from_duration(
                candidate,
                duration,
            ) {
                Some(value) => value,
                None => return Ok(None),
            };

            if self.can_accommodate(interval, quantity, capacity)? {
                return Ok(Some(candidate));
            }

            let next = self
                .next_candidate_after_conflict(interval)
                .ok_or(CalendarError::IdentityExhausted)?;

            if next <= candidate {
                return Err(CalendarError::CapacityOverflow {
                    resource: self.resource_id,
                });
            }

            candidate = next;
        }
    }

    /// Returns the maximum scalar capacity occupied at any point in an
    /// interval.
    ///
    /// The implementation evaluates event boundaries rather than creating a
    /// fixed time grid.
    pub fn maximum_occupied_quantity(
        &self,
        interval: TimeInterval,
    ) -> Result<u128, CalendarError> {
        if interval.is_empty() {
            return self.occupied_quantity_at(interval.start());
        }

        let mut events: BTreeMap<TimePoint, i128> = BTreeMap::new();

        for entry in self.overlapping(interval) {
            if entry.quantity() == 0 {
                continue;
            }

            let start = if entry.start() < interval.start() {
                interval.start()
            } else {
                entry.start()
            };

            let end = if entry.end() > interval.end() {
                interval.end()
            } else {
                entry.end()
            };

            if start >= end {
                continue;
            }

            let quantity = i128::try_from(entry.quantity()).map_err(|_| {
                CalendarError::CapacityOverflow {
                    resource: self.resource_id,
                }
            })?;

            let start_delta = events.entry(start).or_insert(0);
            *start_delta = start_delta.checked_add(quantity).ok_or(
                CalendarError::CapacityOverflow {
                    resource: self.resource_id,
                },
            )?;

            let end_delta = events.entry(end).or_insert(0);
            *end_delta = end_delta.checked_sub(quantity).ok_or(
                CalendarError::CapacityOverflow {
                    resource: self.resource_id,
                },
            )?;
        }

        let mut current = 0i128;
        let mut maximum = 0i128;

        for delta in events.values() {
            current = current.checked_add(*delta).ok_or(
                CalendarError::CapacityOverflow {
                    resource: self.resource_id,
                },
            )?;

            if current < 0 {
                return Err(CalendarError::CapacityOverflow {
                    resource: self.resource_id,
                });
            }

            if current > maximum {
                maximum = current;
            }
        }

        u128::try_from(maximum).map_err(|_| CalendarError::CapacityOverflow {
            resource: self.resource_id,
        })
    }

    /// Returns a cloned immutable snapshot of the calendar.
    ///
    /// This is useful for deterministic planning and speculative scheduling.
    #[must_use]
    pub fn snapshot(&self) -> Self {
        self.clone()
    }

    // =========================================================================
    // Internal validation
    // =========================================================================

    fn validate_entry(
        &self,
        entry: &CalendarEntry,
    ) -> Result<(), CalendarError> {
        if entry.kind().is_reservation() && entry.quantity() == 0 {
            return Err(CalendarError::InvalidQuantity);
        }

        Ok(())
    }

    fn ensure_capacity(
        &self,
        interval: TimeInterval,
        quantity: u128,
        capacity: Option<u128>,
    ) -> Result<(), CalendarError> {
        if quantity == 0 {
            return Ok(());
        }

        if let Some(limit) = capacity {
            if quantity > limit {
                return Err(CalendarError::CapacityExceeded {
                    resource: self.resource_id,
                    interval,
                    requested: quantity,
                    occupied: 0,
                });
            }

            if self
                .first_blocking_conflict(interval)
                .is_some()
            {
                let occupied = self
                    .maximum_occupied_quantity(interval)?;

                return Err(CalendarError::CapacityExceeded {
                    resource: self.resource_id,
                    interval,
                    requested: quantity,
                    occupied,
                });
            }

            let occupied = self
                .maximum_occupied_quantity(interval)?;

            let total = occupied.checked_add(quantity).ok_or(
                CalendarError::CapacityOverflow {
                    resource: self.resource_id,
                },
            )?;

            if total > limit {
                return Err(CalendarError::CapacityExceeded {
                    resource: self.resource_id,
                    interval,
                    requested: quantity,
                    occupied,
                });
            }
        } else if self
            .first_blocking_conflict(interval)
            .is_some()
        {
            return Err(CalendarError::CapacityExceeded {
                resource: self.resource_id,
                interval,
                requested: quantity,
                occupied: u128::MAX,
            });
        }

        Ok(())
    }

    fn find_start(
        &self,
        id: CalendarEntryId,
    ) -> Option<TimePoint> {
        for (start, bucket) in &self.entries {
            if bucket.contains_key(&id) {
                return Some(*start);
            }
        }

        None
    }

    fn next_candidate_after_conflict(
        &self,
        interval: TimeInterval,
    ) -> Option<TimePoint> {
        self.entries
            .range(..=interval.end())
            .flat_map(|(_, bucket)| bucket.values())
            .filter(|entry| entry.overlaps(interval))
            .map(CalendarEntry::end)
            .max()
            .filter(|end| *end > interval.start())
    }
}

// =============================================================================
// Multi-resource calendar
// =============================================================================

/// Calendar containing temporal state for an arbitrary collection of
/// resources.
///
/// This is the normal abstraction consumed by scheduler planners.
///
/// It does not impose a fixed number of resources.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ResourceCalendars {
    resources: BTreeMap<ResourceId, ResourceCalendar>,
}

impl ResourceCalendars {
    /// Creates an empty multi-resource calendar.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            resources: BTreeMap::new(),
        }
    }

    /// Returns the number of resources represented by the calendar.
    #[must_use]
    pub fn len(&self) -> usize {
        self.resources.len()
    }

    /// Returns whether no resources are represented.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.resources.is_empty()
    }

    /// Registers an empty calendar for a resource.
    ///
    /// Registering an already registered resource is idempotent.
    pub fn register(&mut self, resource_id: ResourceId) -> &mut ResourceCalendar {
        self.resources
            .entry(resource_id)
            .or_insert_with(|| ResourceCalendar::new(resource_id))
    }

    /// Returns an immutable resource calendar.
    #[must_use]
    pub fn get(
        &self,
        resource_id: ResourceId,
    ) -> Option<&ResourceCalendar> {
        self.resources.get(&resource_id)
    }

    /// Returns a mutable resource calendar.
    #[must_use]
    pub fn get_mut(
        &mut self,
        resource_id: ResourceId,
    ) -> Option<&mut ResourceCalendar> {
        self.resources.get_mut(&resource_id)
    }

    /// Returns an existing calendar or creates it.
    pub fn get_or_register(
        &mut self,
        resource_id: ResourceId,
    ) -> &mut ResourceCalendar {
        self.register(resource_id)
    }

    /// Removes a resource calendar.
    ///
    /// This removes only temporal calendar state. It does not delete the
    /// underlying `Resource`.
    pub fn remove(
        &mut self,
        resource_id: ResourceId,
    ) -> Option<ResourceCalendar> {
        self.resources.remove(&resource_id)
    }

    /// Returns all registered resource identities in deterministic order.
    pub fn resource_ids(&self) -> impl Iterator<Item = &ResourceId> {
        self.resources.keys()
    }

    /// Returns all resource calendars in deterministic order.
    pub fn iter(&self) -> impl Iterator<Item = &ResourceCalendar> {
        self.resources.values()
    }

    /// Returns the number of entries across all resources.
    #[must_use]
    pub fn entry_count(&self) -> usize {
        self.resources
            .values()
            .map(ResourceCalendar::len)
            .sum()
    }

    /// Returns whether a proposed interval is available on a resource.
    pub fn can_accommodate(
        &self,
        resource_id: ResourceId,
        interval: TimeInterval,
        quantity: u128,
        capacity: Option<u128>,
    ) -> Result<bool, CalendarError> {
        match self.get(resource_id) {
            Some(calendar) => {
                calendar.can_accommodate(
                    interval,
                    quantity,
                    capacity,
                )
            }

            None => {
                if quantity == 0 {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
        }
    }

    /// Finds the next available time for one resource.
    pub fn next_available(
        &self,
        resource_id: ResourceId,
        start: TimePoint,
        duration: Duration,
        quantity: u128,
        capacity: Option<u128>,
    ) -> Result<Option<TimePoint>, CalendarError> {
        match self.get(resource_id) {
            Some(calendar) => calendar.next_available(
                start,
                duration,
                quantity,
                capacity,
            ),

            None => {
                if quantity == 0 {
                    Ok(Some(start))
                } else if capacity
                    .map(|value| quantity <= value)
                    .unwrap_or(true)
                {
                    Some(start).map_or_else(
                        || Ok(None),
                        |value| Ok(Some(value)),
                    )
                } else {
                    Ok(None)
                }
            }
        }
    }

    /// Inserts an entry into the calendar of its resource.
    ///
    /// The resource calendar is created automatically.
    pub fn insert(
        &mut self,
        entry: CalendarEntry,
    ) -> Result<(), CalendarError> {
        self.register(entry.resource_id()).insert(entry)
    }

    /// Inserts an entry with conflict checking.
    pub fn insert_non_conflicting(
        &mut self,
        entry: CalendarEntry,
    ) -> Result<(), CalendarError> {
        self.register(entry.resource_id())
            .insert_non_conflicting(entry)
    }

    /// Returns all entries overlapping an interval for one resource.
    pub fn overlapping(
        &self,
        resource_id: ResourceId,
        interval: TimeInterval,
    ) -> Vec<&CalendarEntry> {
        match self.get(resource_id) {
            Some(calendar) => calendar.overlapping(interval),
            None => Vec::new(),
        }
    }

    /// Creates an independent snapshot.
    #[must_use]
    pub fn snapshot(&self) -> Self {
        self.clone()
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn resource(value: u64) -> ResourceId {
        ResourceId::new(value)
    }

    fn operation(value: u64) -> OperationId {
        OperationId::new(value)
    }

    fn interval(start: u128, end: u128) -> TimeInterval {
        TimeInterval::new(
            TimePoint::new(start),
            TimePoint::new(end),
        )
        .expect("test interval must be valid")
    }

    fn reservation(
        entry: u64,
        resource_id: ResourceId,
        start: u128,
        end: u128,
        quantity: u128,
        operation_id: u64,
    ) -> CalendarEntry {
        CalendarEntry::new(
            CalendarEntryId::new(entry),
            resource_id,
            interval(start, end),
            CalendarEntryKind::Reservation,
            quantity,
            Some(operation(operation_id)),
        )
    }

    #[test]
    fn half_open_adjacent_intervals_do_not_conflict() {
        let mut calendar = ResourceCalendar::new(resource(1));

        calendar
            .insert_non_conflicting(reservation(
                1,
                resource(1),
                0,
                10,
                1,
                1,
            ))
            .unwrap();

        calendar
            .insert_non_conflicting(reservation(
                2,
                resource(1),
                10,
                20,
                1,
                2,
            ))
            .unwrap();

        assert_eq!(calendar.len(), 2);
    }

    #[test]
    fn overlapping_exclusive_reservations_conflict() {
        let mut calendar = ResourceCalendar::new(resource(1));

        calendar
            .insert_non_conflicting(reservation(
                1,
                resource(1),
                0,
                10,
                1,
                1,
            ))
            .unwrap();

        let result = calendar.insert_non_conflicting(
            reservation(2, resource(1), 5, 15, 1, 2),
        );

        assert!(matches!(
            result,
            Err(CalendarError::Conflict { .. })
        ));
    }

    #[test]
    fn different_resources_have_independent_calendars() {
        let mut calendars = ResourceCalendars::new();

        calendars
            .insert_non_conflicting(reservation(
                1,
                resource(1),
                0,
                10,
                1,
                1,
            ))
            .unwrap();

        calendars
            .insert_non_conflicting(reservation(
                2,
                resource(2),
                0,
                10,
                1,
                2,
            ))
            .unwrap();

        assert_eq!(calendars.len(), 2);
        assert_eq!(calendars.entry_count(), 2);
    }

    #[test]
    fn capacity_two_allows_two_overlapping_reservations() {
        let mut calendar = ResourceCalendar::new(resource(1));

        let first = reservation(
            1,
            resource(1),
            0,
            10,
            1,
            1,
        );

        let second = reservation(
            2,
            resource(1),
            0,
            10,
            1,
            2,
        );

        calendar
            .insert_with_capacity(first, Some(2))
            .unwrap();

        calendar
            .insert_with_capacity(second, Some(2))
            .unwrap();

        assert_eq!(
            calendar
                .maximum_occupied_quantity(interval(0, 10))
                .unwrap(),
            2
        );
    }

    #[test]
    fn capacity_three_rejects_fourth_unit() {
        let mut calendar = ResourceCalendar::new(resource(1));

        calendar
            .insert_with_capacity(
                reservation(1, resource(1), 0, 10, 2, 1),
                Some(3),
            )
            .unwrap();

        let result = calendar.insert_with_capacity(
            reservation(2, resource(1), 0, 10, 2, 2),
            Some(3),
        );

        assert!(matches!(
            result,
            Err(CalendarError::CapacityExceeded { .. })
        ));
    }

    #[test]
    fn zero_duration_entries_do_not_overlap_positive_intervals() {
        let mut calendar = ResourceCalendar::new(resource(1));

        let marker = CalendarEntry::new(
            CalendarEntryId::new(1),
            resource(1),
            interval(10, 10),
            CalendarEntryKind::Hold,
            0,
            None,
        );

        calendar
            .insert_non_conflicting(marker)
            .unwrap();

        assert!(!calendar.has_overlap(interval(0, 10)));
        assert!(!calendar.has_overlap(interval(10, 20)));
    }

    #[test]
    fn blocking_entry_prevents_capacity_use() {
        let mut calendar = ResourceCalendar::new(resource(1));

        let maintenance = CalendarEntry::new(
            CalendarEntryId::new(1),
            resource(1),
            interval(10, 20),
            CalendarEntryKind::Maintenance,
            0,
            None,
        );

        calendar.insert(maintenance).unwrap();

        assert!(calendar
            .can_accommodate(
                interval(15, 18),
                1,
                Some(1),
            )
            .unwrap()
            == false);
    }

    #[test]
    fn occupied_quantity_is_calculated_from_intervals() {
        let mut calendar = ResourceCalendar::new(resource(1));

        calendar
            .insert(reservation(
                1,
                resource(1),
                0,
                10,
                2,
                1,
            ))
            .unwrap();

        calendar
            .insert(reservation(
                2,
                resource(1),
                5,
                15,
                3,
                2,
            ))
            .unwrap();

        assert_eq!(
            calendar
                .occupied_quantity_at(TimePoint::new(7))
                .unwrap(),
            5
        );
    }

    #[test]
    fn next_available_moves_after_conflict() {
        let mut calendar = ResourceCalendar::new(resource(1));

        calendar
            .insert_non_conflicting(reservation(
                1,
                resource(1),
                10,
                20,
                1,
                1,
            ))
            .unwrap();

        let result = calendar
            .next_available(
                TimePoint::new(5),
                Duration::new(8),
                1,
                Some(1),
            )
            .unwrap();

        assert_eq!(result, Some(TimePoint::new(20)));
    }

    #[test]
    fn operation_removal_removes_all_entries() {
        let mut calendar = ResourceCalendar::new(resource(1));

        calendar
            .insert(reservation(
                1,
                resource(1),
                0,
                10,
                1,
                7,
            ))
            .unwrap();

        calendar
            .insert(reservation(
                2,
                resource(1),
                20,
                30,
                1,
                7,
            ))
            .unwrap();

        assert_eq!(
            calendar.remove_operation(operation(7)),
            2
        );

        assert!(calendar.is_empty());
    }

    #[test]
    fn resource_mismatch_is_rejected() {
        let mut calendar = ResourceCalendar::new(resource(1));

        let entry = reservation(
            1,
            resource(2),
            0,
            10,
            1,
            1,
        );

        assert!(matches!(
            calendar.insert(entry),
            Err(CalendarError::ResourceMismatch { .. })
        ));
    }

    #[test]
    fn deterministic_iteration_order_is_by_start_then_identity() {
        let mut calendar = ResourceCalendar::new(resource(1));

        calendar
            .insert(reservation(
                2,
                resource(1),
                10,
                20,
                1,
                2,
            ))
            .unwrap();

        calendar
            .insert(reservation(
                1,
                resource(1),
                0,
                5,
                1,
                1,
            ))
            .unwrap();

        let ids: Vec<CalendarEntryId> =
            calendar.iter().map(CalendarEntry::id).collect();

        assert_eq!(
            ids,
            vec![
                CalendarEntryId::new(1),
                CalendarEntryId::new(2)
            ]
        );
    }

    #[test]
    fn snapshot_is_independent() {
        let mut calendar = ResourceCalendar::new(resource(1));

        calendar
            .insert(reservation(
                1,
                resource(1),
                0,
                10,
                1,
                1,
            ))
            .unwrap();

        let mut snapshot = calendar.snapshot();

        snapshot
            .remove(CalendarEntryId::new(1))
            .unwrap();

        assert_eq!(calendar.len(), 1);
        assert_eq!(snapshot.len(), 0);
    }
}