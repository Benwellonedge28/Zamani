//! Zamani Quantum Scheduling — Resource Reservations
//!
//! This module defines the production representation of one reservation of a
//! scheduler resource over an abstract scheduling interval.
//!
//! # Responsibility
//!
//! This file answers:
//!
//! > "Which scheduler resource is reserved by which operation, for what
//! > interval, in what reservation mode, and in what quantity?"
//!
//! It owns:
//!
//! - immutable resource reservation records;
//! - reservation modes;
//! - reservation quantities;
//! - reservation interval validation;
//! - reservation identity/reference handling;
//! - temporal overlap queries;
//! - deterministic ordering;
//! - reservation compatibility predicates;
//! - conversion to the lightweight `ReservationRef` contract;
//! - reservation diagnostics useful to calendars, pools, planners, and
//!   verification.
//!
//! It does NOT own:
//!
//! - resource inventory;
//! - resource capacity definitions;
//! - resource discovery;
//! - resource calendars;
//! - scheduling algorithms;
//! - dependency graphs;
//! - routing;
//! - hardware execution;
//! - hardware discovery;
//! - calibration;
//! - QEC algorithms;
//! - quantum operation semantics;
//! - serialization formats.
//!
//! Those responsibilities belong to the corresponding scheduling or quantum
//! subsystems.
//!
//! # Architectural position
//!
//! ```text
//! quantum::ir::core::identity
//!          │
//!          ├── OperationId
//!          └── ResourceId
//!
//! quantum::ir::qubit
//!          │
//!          ├── QubitId
//!          └── PhysicalQubitId
//!
//! scheduling::types
//!          │
//!          ├── ReservationId
//!          ├── OperationRef
//!          ├── ResourceRef
//!          ├── TimePoint
//!          ├── Duration
//!          └── TimeInterval
//!
//!                 ▼
//!       scheduling::resources::reservation
//!                 │
//!          ┌──────┼────────┐
//!          ▼      ▼        ▼
//!        pool  calendar  availability
//!          │      │        │
//!          └──────┼────────┘
//!                 ▼
//!              planners
//!                 │
//!                 ▼
//!             verification
//! ```
//!
//! # Canonical identity rule
//!
//! This module MUST NOT define replacement identity types.
//!
//! Operation identity comes from:
//!
//! ```text
//! crate::quantum::ir::core::identity::OperationId
//! ```
//!
//! Resource identity comes from:
//!
//! ```text
//! crate::quantum::ir::core::identity::ResourceId
//! ```
//!
//! Reservation identity comes from:
//!
//! ```text
//! crate::quantum::scheduling::types::ReservationId
//! ```
//!
//! Logical and physical qubit identities remain owned by:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! A reservation therefore never invents a scheduler-specific qubit identity.
//!
//! If a physical or logical qubit is represented as a schedulable resource,
//! its canonical qubit identity is resolved by the resource-model/adapter
//! layer and represented here by the corresponding `ResourceRef`.
//!
//! # Resource ownership
//!
//! A reservation identifies a resource; it does not define that resource.
//!
//! ```text
//! Resource
//!     │
//!     │ inventory/capacity
//!     ▼
//! ResourcePool
//!     │
//!     │ temporal occupancy
//!     ▼
//! Reservation
//!     │
//!     ▼
//! ResourceCalendar
//! ```
//!
//! This separation is important because a single resource may have:
//!
//! - finite capacity;
//! - unlimited capacity;
//! - dynamic availability;
//! - hierarchical capacity;
//! - multiple simultaneous reservations;
//! - different reservation modes.
//!
//! A reservation must therefore not embed assumptions about the resource's
//! physical implementation.
//!
//! # Time representation
//!
//! Scheduling time uses the target-independent types from `scheduling::types`:
//!
//! ```text
//! TimePoint
//! Duration
//! TimeInterval
//! ```
//!
//! No physical unit is embedded here.
//!
//! A `TimePoint` may represent target-defined ticks, an abstract scheduling
//! coordinate, or another timing representation supplied by the timing and
//! hardware layers.
//!
//! This module therefore contains no values such as:
//!
//! ```text
//! 1ns
//! 10ns
//! dt = 0.222ns
//! ```
//!
//! # Interval semantics
//!
//! Reservations use half-open intervals:
//!
//! ```text
//! [start, end)
//! ```
//!
//! Consequently:
//!
//! ```text
//! [0, 10) and [10, 20)
//! ```
//!
//! do NOT overlap.
//!
//! This is essential for scalable event-based scheduling because an operation
//! releasing a resource at `t` permits another operation to begin at exactly
//! `t` without an artificial conflict.
//!
//! A zero-duration interval:
//!
//! ```text
//! [t, t)
//! ```
//!
//! is valid and occupies no positive-duration temporal region.
//!
//! # Scalability
//!
//! There is intentionally no:
//!
//! - maximum reservation count;
//! - maximum resource count;
//! - maximum operation count;
//! - maximum qubit count;
//! - maximum schedule depth;
//! - maximum time;
//! - fixed number of resource types;
//! - fixed number of simultaneous reservations.
//!
//! A concrete collection of reservations is bounded only by the available
//! representation, host resources, explicit execution policy, and target.
//!
//! "Infinity" therefore means that this file introduces no artificial finite
//! machine-size ceiling.
//!
//! # Capacity separation
//!
//! This module deliberately does not decide whether several reservations fit
//! within a resource's total capacity.
//!
//! It can answer:
//!
//! > "Do these reservations overlap temporally on the same resource?"
//!
//! The resource pool/calendar/verification layer answers:
//!
//! > "Does their combined usage exceed the resource's capacity?"
//!
//! This prevents capacity logic from being duplicated across reservation,
//! calendar, and pool implementations.
//!
//! # Immutability
//!
//! `Reservation` is an immutable value object.
//!
//! Once constructed, its identity, operation, resource, interval, mode, and
//! quantity cannot be changed.
//!
//! A changed reservation is represented by constructing a new reservation.
//!
//! This is important because calendars and scheduling results may retain
//! references to a reservation after verification.
//!
//! # Thread safety
//!
//! The reservation contains only ordinary owned/copyable values and has no:
//!
//! - global state;
//! - interior mutability;
//! - locks;
//! - raw pointers;
//! - unsafe code.
//!
//! It is therefore suitable for ownership transfer and concurrent read-only
//! analysis.
//!
//! # Determinism
//!
//! Reservations implement deterministic equality, hashing, ordering, and
//! formatting.
//!
//! No wall clock, process ID, pointer address, hash-map iteration order, or
//! hidden random state participates in reservation semantics.
//!
//! # Rust compatibility
//!
//! Supported:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! # Integration contract
//!
//! `calendar.rs` should use:
//!
//! - `Reservation::overlaps`;
//! - `Reservation::same_resource`;
//! - `Reservation::interval`;
//! - `Reservation::quantity`;
//! - `Reservation::mode`;
//! - `Reservation::resource`;
//! - `Reservation::operation`.
//!
//! `pool.rs` should use:
//!
//! - `Reservation::resource`;
//! - `Reservation::quantity`;
//! - `Reservation::mode`;
//! - `Reservation::overlaps`.
//!
//! Planners should construct reservations only after an operation has been
//! assigned a legal start time and resource.
//!
//! Verification should use this type to validate:
//!
//! - resource identity;
//! - temporal interval validity;
//! - duplicate reservation identity;
//! - operation/resource relationships;
//! - temporal overlap;
//! - reservation quantity.
//!
//! The result layer can obtain the lightweight `ReservationRef` through
//! `Reservation::reference()`.
//!
//! # Finish-once rule
//!
//! This file intentionally depends only on foundational scheduling contracts:
//!
//! ```text
//! scheduling::types
//! scheduling::errors
//! ```
//!
//! It does not import:
//!
//! ```text
//! pool
//! calendar
//! availability
//! planner
//! algorithm
//! hardware
//! routing
//! qec
//! runtime
//! ```
//!
//! Consequently, adding or changing those downstream components does not
//! require this file to be edited merely to integrate them.
//!
//! =============================================================================
//! Safety boundary
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};

use crate::quantum::ir::core::identity::{
    OperationId,
    ResourceId,
};

use super::super::errors::{
    SchedulingError,
    SchedulingResult,
};

use super::super::types::{
    OperationRef,
    ReservationId,
    ReservationRef,
    ResourceRef,
    TimeInterval,
    TimePoint,
};

// =============================================================================
// Reservation mode
// =============================================================================

/// Describes how a reservation consumes or occupies a resource.
///
/// The mode describes reservation semantics only. The resource model remains
/// responsible for defining the resource's actual capacity and capabilities.
///
/// # Capacity
///
/// `Capacity` carries an explicit quantity. Whether that quantity can coexist
/// with other reservations is determined by the resource's capacity model.
///
/// # Shared
///
/// `Shared` represents a reservation that does not claim exclusive ownership
/// of the resource.
///
/// This is useful for resources that permit concurrent use.
///
/// # Exclusive
///
/// `Exclusive` represents an exclusive temporal claim.
///
/// An exclusive reservation conflicts with another reservation on the same
/// resource whenever their positive-duration intervals overlap.
///
/// # Consumable
///
/// `Consumable` identifies a quantity that is consumed by an operation rather
/// than merely occupied for the interval.
///
/// The actual accounting rules belong to the resource/pool layer.
///
/// # Reusable
///
/// `Reusable` identifies a resource allocation whose quantity becomes
/// available again after the reservation interval.
///
/// The default interpretation for ordinary reusable quantum hardware is
/// temporal occupancy.
///
/// # Important
///
/// `ReservationMode` does not itself know the resource capacity. It therefore
/// must not attempt to declare two capacity reservations compatible or
/// incompatible solely from their quantities.
///
/// That decision belongs to the resource model and pool/calendar.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ReservationMode {
    /// Exclusive ownership of the resource during the interval.
    Exclusive,

    /// Shared use of the resource during the interval.
    Shared,

    /// Explicit capacity usage.
    ///
    /// The value is the number of abstract capacity units required.
    Capacity {
        /// Required capacity units.
        units: u128,
    },

    /// Quantity is consumed by the operation.
    ///
    /// The pool/resource layer owns the accounting semantics.
    Consumable {
        /// Consumed quantity.
        units: u128,
    },

    /// Resource quantity is temporarily occupied and becomes reusable after
    /// the reservation interval.
    Reusable {
        /// Occupied quantity.
        units: u128,
    },
}

impl Default for ReservationMode {
    fn default() -> Self {
        Self::Exclusive
    }
}

impl ReservationMode {
    /// Creates an exclusive reservation mode.
    #[must_use]
    pub const fn exclusive() -> Self {
        Self::Exclusive
    }

    /// Creates a shared reservation mode.
    #[must_use]
    pub const fn shared() -> Self {
        Self::Shared
    }

    /// Creates a capacity reservation.
    ///
    /// Zero is representable and is intentionally not rejected here because
    /// the resource layer may use zero-quantity records as metadata/events.
    #[must_use]
    pub const fn capacity(units: u128) -> Self {
        Self::Capacity { units }
    }

    /// Creates a consumable reservation.
    #[must_use]
    pub const fn consumable(units: u128) -> Self {
        Self::Consumable { units }
    }

    /// Creates a reusable reservation.
    #[must_use]
    pub const fn reusable(units: u128) -> Self {
        Self::Reusable { units }
    }

    /// Returns whether the mode is exclusive.
    #[must_use]
    pub const fn is_exclusive(self) -> bool {
        matches!(self, Self::Exclusive)
    }

    /// Returns whether the mode permits sharing at the reservation-semantic
    /// level.
    ///
    /// This does not prove that a concrete resource supports sharing.
    #[must_use]
    pub const fn is_shared(self) -> bool {
        matches!(self, Self::Shared)
    }

    /// Returns whether this mode represents explicit capacity usage.
    #[must_use]
    pub const fn is_capacity_based(self) -> bool {
        matches!(
            self,
            Self::Capacity { .. }
                | Self::Consumable { .. }
                | Self::Reusable { .. }
        )
    }

    /// Returns the explicit quantity carried by the mode, when one exists.
    #[must_use]
    pub const fn units(self) -> Option<u128> {
        match self {
            Self::Exclusive | Self::Shared => None,
            Self::Capacity { units }
            | Self::Consumable { units }
            | Self::Reusable { units } => Some(units),
        }
    }

    /// Returns a stable machine-readable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Exclusive => "exclusive",
            Self::Shared => "shared",
            Self::Capacity { .. } => "capacity",
            Self::Consumable { .. } => "consumable",
            Self::Reusable { .. } => "reusable",
        }
    }
}

impl fmt::Display for ReservationMode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Exclusive | Self::Shared => {
                formatter.write_str(self.as_str())
            }

            Self::Capacity { units }
            | Self::Consumable { units }
            | Self::Reusable { units } => {
                write!(formatter, "{}({units})", self.as_str())
            }
        }
    }
}

// =============================================================================
// Reservation
// =============================================================================

/// Immutable reservation of one scheduling resource by one operation.
///
/// A reservation is a temporal claim:
///
/// ```text
/// operation
///     │
///     ▼
/// resource
///     │
///     ▼
/// [start, end)
/// ```
///
/// It is deliberately independent from the resource's actual capacity.
///
/// # Invariants
///
/// A successfully constructed reservation satisfies:
///
/// 1. `start <= end`;
/// 2. reservation identity is preserved exactly;
/// 3. operation identity is preserved exactly;
/// 4. resource identity is preserved exactly;
/// 5. quantity is explicit;
/// 6. no physical timing unit is assumed.
///
/// A zero-duration interval is valid.
///
/// # Identity
///
/// `ReservationId` is a semantic identity and is not interpreted as a
/// collection index or allocation counter by this type.
#[derive(Debug, Clone, Copy, Eq)]
pub struct Reservation {
    id: ReservationId,
    operation: OperationRef,
    resource: ResourceRef,
    interval: TimeInterval,
    mode: ReservationMode,
    quantity: u128,
}

impl Reservation {
    // =========================================================================
    // Constructors
    // =========================================================================

    /// Creates a reservation from an already validated interval.
    ///
    /// This constructor is infallible because `TimeInterval` already enforces
    /// its own temporal invariant.
    #[must_use]
    pub const fn new(
        id: ReservationId,
        operation: OperationRef,
        resource: ResourceRef,
        interval: TimeInterval,
        mode: ReservationMode,
        quantity: u128,
    ) -> Self {
        Self {
            id,
            operation,
            resource,
            interval,
            mode,
            quantity,
        }
    }

    /// Creates a reservation from raw start/end points.
    ///
    /// The interval is validated before the reservation is created.
    pub fn from_bounds(
        id: ReservationId,
        operation: OperationRef,
        resource: ResourceRef,
        start: TimePoint,
        end: TimePoint,
        mode: ReservationMode,
        quantity: u128,
    ) -> SchedulingResult<Self> {
        let interval = TimeInterval::new(start, end).map_err(|_| {
            SchedulingError::InvalidInput {
                reason: format!(
                    "reservation `{id}` has an invalid interval [{start}, {end})"
                ),
            }
        })?;

        Ok(Self::new(
            id,
            operation,
            resource,
            interval,
            mode,
            quantity,
        ))
    }

    /// Creates an exclusive reservation.
    pub fn exclusive(
        id: ReservationId,
        operation: OperationRef,
        resource: ResourceRef,
        start: TimePoint,
        end: TimePoint,
        quantity: u128,
    ) -> SchedulingResult<Self> {
        Self::from_bounds(
            id,
            operation,
            resource,
            start,
            end,
            ReservationMode::Exclusive,
            quantity,
        )
    }

    /// Creates a shared reservation.
    pub fn shared(
        id: ReservationId,
        operation: OperationRef,
        resource: ResourceRef,
        start: TimePoint,
        end: TimePoint,
        quantity: u128,
    ) -> SchedulingResult<Self> {
        Self::from_bounds(
            id,
            operation,
            resource,
            start,
            end,
            ReservationMode::Shared,
            quantity,
        )
    }

    /// Creates a capacity-based reservation.
    pub fn capacity(
        id: ReservationId,
        operation: OperationRef,
        resource: ResourceRef,
        start: TimePoint,
        end: TimePoint,
        units: u128,
    ) -> SchedulingResult<Self> {
        Self::from_bounds(
            id,
            operation,
            resource,
            start,
            end,
            ReservationMode::Capacity { units },
            units,
        )
    }

    /// Creates a consumable reservation.
    pub fn consumable(
        id: ReservationId,
        operation: OperationRef,
        resource: ResourceRef,
        start: TimePoint,
        end: TimePoint,
        units: u128,
    ) -> SchedulingResult<Self> {
        Self::from_bounds(
            id,
            operation,
            resource,
            start,
            end,
            ReservationMode::Consumable { units },
            units,
        )
    }

    /// Creates a reusable resource reservation.
    pub fn reusable(
        id: ReservationId,
        operation: OperationRef,
        resource: ResourceRef,
        start: TimePoint,
        end: TimePoint,
        units: u128,
    ) -> SchedulingResult<Self> {
        Self::from_bounds(
            id,
            operation,
            resource,
            start,
            end,
            ReservationMode::Reusable { units },
            units,
        )
    }

    // =========================================================================
    // Identity
    // =========================================================================

    /// Returns the reservation identity.
    #[must_use]
    pub const fn id(self) -> ReservationId {
        self.id
    }

    /// Returns the operation reference.
    #[must_use]
    pub const fn operation(self) -> OperationRef {
        self.operation
    }

    /// Returns the canonical operation identity.
    #[must_use]
    pub const fn operation_id(self) -> OperationId {
        self.operation.id()
    }

    /// Returns the resource reference.
    #[must_use]
    pub const fn resource(self) -> ResourceRef {
        self.resource
    }

    /// Returns the canonical resource identity.
    #[must_use]
    pub const fn resource_id(self) -> ResourceId {
        self.resource.id()
    }

    // =========================================================================
    // Temporal properties
    // =========================================================================

    /// Returns the complete reservation interval.
    #[must_use]
    pub const fn interval(self) -> TimeInterval {
        self.interval
    }

    /// Returns the reservation start time.
    #[must_use]
    pub const fn start(self) -> TimePoint {
        self.interval.start()
    }

    /// Returns the reservation end time.
    #[must_use]
    pub const fn end(self) -> TimePoint {
        self.interval.end()
    }

    /// Returns the reservation duration.
    #[must_use]
    pub const fn duration(self) -> super::super::types::Duration {
        self.interval.duration()
    }

    /// Returns whether the reservation has zero duration.
    #[must_use]
    pub const fn is_zero_duration(self) -> bool {
        self.start() == self.end()
    }

    /// Returns whether this reservation contains a time point.
    ///
    /// The interval follows `[start, end)` semantics.
    #[must_use]
    pub fn contains(self, time: TimePoint) -> bool {
        self.interval.contains(time)
    }

    /// Returns whether this reservation overlaps another reservation
    /// temporally.
    ///
    /// Resource identity is intentionally ignored by this method.
    ///
    /// Use `conflicts_temporally_with` when checking actual resource
    /// contention.
    #[must_use]
    pub fn overlaps(self, other: Self) -> bool {
        self.interval.overlaps(other.interval)
    }

    /// Returns whether two reservations are adjacent in time.
    ///
    /// Adjacency is not a conflict under half-open interval semantics.
    #[must_use]
    pub fn touches(self, other: Self) -> bool {
        self.end() == other.start() || other.end() == self.start()
    }

    /// Returns whether this reservation and another reservation occupy the
    /// same resource and overlap temporally.
    #[must_use]
    pub fn conflicts_temporally_with(self, other: Self) -> bool {
        self.same_resource(other) && self.overlaps(other)
    }

    /// Returns whether the two reservations have the same resource identity.
    #[must_use]
    pub const fn same_resource(self, other: Self) -> bool {
        self.resource_id() == other.resource_id()
    }

    /// Returns whether the two reservations belong to the same operation.
    #[must_use]
    pub const fn same_operation(self, other: Self) -> bool {
        self.operation_id() == other.operation_id()
    }

    // =========================================================================
    // Capacity/mode properties
    // =========================================================================

    /// Returns the reservation mode.
    #[must_use]
    pub const fn mode(self) -> ReservationMode {
        self.mode
    }

    /// Returns the explicit quantity associated with this reservation.
    ///
    /// The quantity is independent from resource capacity.
    #[must_use]
    pub const fn quantity(self) -> u128 {
        self.quantity
    }

    /// Returns whether the reservation consumes positive quantity.
    #[must_use]
    pub const fn has_positive_quantity(self) -> bool {
        self.quantity > 0
    }

    /// Returns whether the reservation is exclusive.
    #[must_use]
    pub const fn is_exclusive(self) -> bool {
        self.mode.is_exclusive()
    }

    /// Returns whether the reservation is shared.
    #[must_use]
    pub const fn is_shared(self) -> bool {
        self.mode.is_shared()
    }

    /// Returns the quantity encoded directly by the reservation mode, if any.
    #[must_use]
    pub const fn mode_units(self) -> Option<u128> {
        self.mode.units()
    }

    /// Returns whether the reservation is a positive-duration occupancy.
    #[must_use]
    pub const fn occupies_time(self) -> bool {
        self.start() < self.end()
    }

    /// Returns whether the reservation represents positive resource usage.
    #[must_use]
    pub const fn occupies_resource(self) -> bool {
        self.occupies_time() && self.quantity > 0
    }

    // =========================================================================
    // Validation
    // =========================================================================

    /// Validates the reservation's internal consistency.
    ///
    /// `TimeInterval` has already validated `start <= end`, so this method
    /// focuses on consistency between mode and quantity.
    ///
    /// A zero quantity is allowed intentionally. It may represent an
    /// analysis-only event or a zero-capacity semantic marker.
    pub fn validate(self) -> SchedulingResult<()> {
        match self.mode {
            ReservationMode::Exclusive | ReservationMode::Shared => Ok(()),

            ReservationMode::Capacity { units }
            | ReservationMode::Consumable { units }
            | ReservationMode::Reusable { units } => {
                if units != self.quantity {
                    return Err(SchedulingError::InvalidInput {
                        reason: format!(
                            "reservation `{}` has mode quantity `{units}` \
                             but reservation quantity is `{}`",
                            self.id,
                            self.quantity
                        ),
                    });
                }

                Ok(())
            }
        }
    }

    // =========================================================================
    // Reference integration
    // =========================================================================

    /// Returns the lightweight foundational reservation reference.
    ///
    /// The returned `ReservationRef` is intentionally smaller than the full
    /// reservation object and is suitable for scheduling results, dependency
    /// structures, diagnostics, and serialization metadata.
    #[must_use]
    pub const fn reference(self) -> ReservationRef {
        ReservationRef::new(
            self.id,
            self.operation,
            self.resource,
            self.interval,
        )
    }

    // =========================================================================
    // Ordering helpers
    // =========================================================================

    /// Returns a deterministic ordering key suitable for ordered collections.
    ///
    /// Ordering is:
    ///
    /// 1. start;
    /// 2. end;
    /// 3. resource;
    /// 4. operation;
    /// 5. reservation identity.
    #[must_use]
    pub fn ordering_key(
        self,
    ) -> (
        TimePoint,
        TimePoint,
        ResourceId,
        OperationId,
        ReservationId,
    ) {
        (
            self.start(),
            self.end(),
            self.resource_id(),
            self.operation_id(),
            self.id,
        )
    }

    /// Returns a compact deterministic tuple for conflict diagnostics.
    #[must_use]
    pub fn conflict_key(
        self,
    ) -> (
        ResourceId,
        TimePoint,
        TimePoint,
        OperationId,
        ReservationId,
    ) {
        (
            self.resource_id(),
            self.start(),
            self.end(),
            self.operation_id(),
            self.id,
        )
    }
}

// =============================================================================
// Trait implementations
// =============================================================================

impl PartialEq for Reservation {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.operation == other.operation
            && self.resource == other.resource
            && self.interval == other.interval
            && self.mode == other.mode
            && self.quantity == other.quantity
    }
}

impl Hash for Reservation {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.operation.hash(state);
        self.resource.hash(state);
        self.interval.hash(state);
        self.mode.hash(state);
        self.quantity.hash(state);
    }
}

impl PartialOrd for Reservation {
    fn partial_cmp(
        &self,
        other: &Self,
    ) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Reservation {
    fn cmp(
        &self,
        other: &Self,
    ) -> Ordering {
        self.ordering_key().cmp(&other.ordering_key())
    }
}

impl fmt::Display for Reservation {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            formatter,
            "{}: operation={} resource={} interval={} mode={} quantity={}",
            self.id,
            self.operation_id(),
            self.resource_id(),
            self.interval,
            self.mode,
            self.quantity
        )
    }
}

// =============================================================================
// Reservation conflict helpers
// =============================================================================

/// Returns whether two reservations are definitely incompatible based only on
/// exclusive reservation semantics.
///
/// This deliberately does NOT attempt to solve general capacity scheduling.
///
/// For capacity-based reservations, the answer is `false` because capacity
/// information belongs to the resource model.
///
/// Use the resource pool/calendar for the complete capacity decision.
#[must_use]
pub fn definitely_conflicts(
    left: Reservation,
    right: Reservation,
) -> bool {
    if !left.same_resource(right) {
        return false;
    }

    if !left.overlaps(right) {
        return false;
    }

    if !left.occupies_resource() || !right.occupies_resource() {
        return false;
    }

    left.is_exclusive() || right.is_exclusive()
}

/// Returns whether two reservations are candidates for further capacity
/// analysis.
///
/// A `true` result means:
///
/// - same resource;
/// - positive temporal overlap;
/// - positive resource usage.
///
/// It does NOT mean a conflict is guaranteed.
///
/// For example, two capacity reservations on a resource with capacity `8`
/// requiring `2` and `3` units are candidates but can coexist.
///
/// The pool/calendar must compare the aggregate usage with actual resource
/// capacity.
#[must_use]
pub fn requires_capacity_check(
    left: Reservation,
    right: Reservation,
) -> bool {
    left.same_resource(right)
        && left.overlaps(right)
        && left.occupies_resource()
        && right.occupies_resource()
}

/// Returns whether two reservations are completely disjoint in resource-time
/// space.
///
/// Different resources are disjoint even if their intervals overlap.
#[must_use]
pub fn are_disjoint(
    left: Reservation,
    right: Reservation,
) -> bool {
    !left.same_resource(right) || !left.overlaps(right)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn reservation(
        id: u64,
        operation: u64,
        resource: u64,
        start: u128,
        end: u128,
    ) -> Reservation {
        Reservation::exclusive(
            ReservationId::new(id),
            OperationRef::new(OperationId::new(operation)),
            ResourceRef::new(ResourceId::new(resource)),
            TimePoint::new(start),
            TimePoint::new(end),
            1,
        )
        .expect("test reservation must be valid")
    }

    #[test]
    fn creates_valid_reservation() {
        let value = reservation(1, 10, 20, 0, 10);

        assert_eq!(value.id(), ReservationId::new(1));
        assert_eq!(value.operation_id(), OperationId::new(10));
        assert_eq!(value.resource_id(), ResourceId::new(20));
        assert_eq!(value.start(), TimePoint::new(0));
        assert_eq!(value.end(), TimePoint::new(10));
        assert_eq!(value.quantity(), 1);
        assert!(value.is_exclusive());
    }

    #[test]
    fn rejects_inverted_interval() {
        let result = Reservation::exclusive(
            ReservationId::new(1),
            OperationRef::new(OperationId::new(1)),
            ResourceRef::new(ResourceId::new(1)),
            TimePoint::new(20),
            TimePoint::new(10),
            1,
        );

        assert!(result.is_err());
    }

    #[test]
    fn allows_zero_duration_reservation() {
        let value = reservation(1, 1, 1, 10, 10);

        assert!(value.is_zero_duration());
        assert!(!value.occupies_time());
        assert!(!value.occupies_resource());
    }

    #[test]
    fn adjacent_reservations_do_not_overlap() {
        let first = reservation(1, 1, 1, 0, 10);
        let second = reservation(2, 2, 1, 10, 20);

        assert!(!first.overlaps(second));
        assert!(first.touches(second));
        assert!(!first.conflicts_temporally_with(second));
    }

    #[test]
    fn overlapping_same_resource_reservations_conflict_when_exclusive() {
        let first = reservation(1, 1, 1, 0, 10);
        let second = reservation(2, 2, 1, 5, 20);

        assert!(first.overlaps(second));
        assert!(first.same_resource(second));
        assert!(first.conflicts_temporally_with(second));
        assert!(definitely_conflicts(first, second));
    }

    #[test]
    fn overlapping_different_resources_are_not_resource_conflicts() {
        let first = reservation(1, 1, 1, 0, 10);
        let second = reservation(2, 2, 2, 5, 20);

        assert!(first.overlaps(second));
        assert!(!first.same_resource(second));
        assert!(!first.conflicts_temporally_with(second));
        assert!(!definitely_conflicts(first, second));
        assert!(are_disjoint(first, second));
    }

    #[test]
    fn shared_reservations_are_capacity_candidates_not_definite_conflicts() {
        let first = Reservation::shared(
            ReservationId::new(1),
            OperationRef::new(OperationId::new(1)),
            ResourceRef::new(ResourceId::new(1)),
            TimePoint::new(0),
            TimePoint::new(10),
            1,
        )
        .expect("valid reservation");

        let second = Reservation::shared(
            ReservationId::new(2),
            OperationRef::new(OperationId::new(2)),
            ResourceRef::new(ResourceId::new(1)),
            TimePoint::new(5),
            TimePoint::new(20),
            1,
        )
        .expect("valid reservation");

        assert!(requires_capacity_check(first, second));
        assert!(!definitely_conflicts(first, second));
    }

    #[test]
    fn capacity_mode_must_match_quantity() {
        let value = Reservation::new(
            ReservationId::new(1),
            OperationRef::new(OperationId::new(1)),
            ResourceRef::new(ResourceId::new(1)),
            TimeInterval::new(
                TimePoint::new(0),
                TimePoint::new(10),
            )
            .expect("valid interval"),
            ReservationMode::Capacity { units: 4 },
            3,
        );

        assert!(value.validate().is_err());
    }

    #[test]
    fn capacity_mode_accepts_matching_quantity() {
        let value = Reservation::capacity(
            ReservationId::new(1),
            OperationRef::new(OperationId::new(1)),
            ResourceRef::new(ResourceId::new(1)),
            TimePoint::new(0),
            TimePoint::new(10),
            4,
        )
        .expect("valid reservation");

        assert!(value.validate().is_ok());
        assert_eq!(value.mode_units(), Some(4));
    }

    #[test]
    fn consumable_mode_preserves_quantity() {
        let value = Reservation::consumable(
            ReservationId::new(1),
            OperationRef::new(OperationId::new(1)),
            ResourceRef::new(ResourceId::new(1)),
            TimePoint::new(0),
            TimePoint::new(10),
            7,
        )
        .expect("valid reservation");

        assert_eq!(value.quantity(), 7);
        assert_eq!(value.mode_units(), Some(7));
        assert!(value.validate().is_ok());
    }

    #[test]
    fn reusable_mode_preserves_quantity() {
        let value = Reservation::reusable(
            ReservationId::new(1),
            OperationRef::new(OperationId::new(1)),
            ResourceRef::new(ResourceId::new(1)),
            TimePoint::new(0),
            TimePoint::new(10),
            2,
        )
        .expect("valid reservation");

        assert_eq!(value.quantity(), 2);
        assert_eq!(value.mode_units(), Some(2));
        assert!(value.validate().is_ok());
    }

    #[test]
    fn reference_preserves_core_identity() {
        let value = reservation(77, 88, 99, 100, 200);
        let reference = value.reference();

        assert_eq!(reference.id(), ReservationId::new(77));
        assert_eq!(
            reference.operation().id(),
            OperationId::new(88)
        );
        assert_eq!(
            reference.resource().id(),
            ResourceId::new(99)
        );
        assert_eq!(
            reference.interval().start(),
            TimePoint::new(100)
        );
        assert_eq!(
            reference.interval().end(),
            TimePoint::new(200)
        );
    }

    #[test]
    fn ordering_is_deterministic() {
        let first = reservation(1, 10, 20, 0, 10);
        let second = reservation(2, 11, 20, 10, 20);
        let third = reservation(3, 12, 21, 0, 10);

        assert!(first < second);
        assert!(first < third);
    }

    #[test]
    fn same_operation_can_have_multiple_non_overlapping_reservations() {
        let first = reservation(1, 10, 20, 0, 10);
        let second = reservation(2, 10, 20, 10, 20);

        assert!(first.same_operation(second));
        assert!(!first.overlaps(second));
        assert!(!first.conflicts_temporally_with(second));
    }

    #[test]
    fn contains_uses_half_open_interval_semantics() {
        let value = reservation(1, 1, 1, 10, 20);

        assert!(!value.contains(TimePoint::new(9)));
        assert!(value.contains(TimePoint::new(10)));
        assert!(value.contains(TimePoint::new(19)));
        assert!(!value.contains(TimePoint::new(20)));
    }

    #[test]
    fn conflict_key_is_deterministic() {
        let value = reservation(9, 8, 7, 10, 20);

        assert_eq!(
            value.conflict_key(),
            (
                ResourceId::new(7),
                TimePoint::new(10),
                TimePoint::new(20),
                OperationId::new(8),
                ReservationId::new(9),
            )
        );
    }
}