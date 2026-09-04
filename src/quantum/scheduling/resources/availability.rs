//! Zamani Quantum Scheduling — Resource Availability
//!
//! This module defines the temporal availability contract for schedulable
//! resources.
//!
//! # Architectural responsibility
//!
//! This module answers:
//!
//! > "Is a particular scheduling resource usable at a particular time or
//! > during a particular interval?"
//!
//! It represents externally supplied resource availability information such
//! as:
//!
//! - available;
//! - degraded;
//! - unavailable;
//! - unknown;
//! - maintenance windows;
//! - calibration exclusions;
//! - device disablement;
//! - temporary operational restrictions;
//! - target-provided availability changes;
//! - dynamically changing resource state.
//!
//! It deliberately does NOT represent:
//!
//! - reservations;
//! - resource capacity accounting;
//! - scheduling algorithms;
//! - dependency graphs;
//! - routing;
//! - hardware communication;
//! - hardware discovery;
//! - calibration acquisition;
//! - QEC algorithms;
//! - quantum operation semantics;
//! - execution;
//! - serialization formats.
//!
//! Those responsibilities belong to their respective subsystems.
//!
//! # Architectural position
//!
//! ```text
//! quantum::ir::core::identity
//!          │
//!          └── ResourceId
//!
//! scheduling::types
//!          │
//!          ├── TimePoint
//!          ├── Duration
//!          └── TimeInterval
//!
//! scheduling::resources::resource
//!          │
//!          └── resource semantics
//!
//!                    ▼
//!          resources::availability
//!                    │
//!          ┌─────────┼─────────┐
//!          ▼         ▼         ▼
//!       calendar   pool      hardware adapter
//!          │         │         │
//!          └─────────┼─────────┘
//!                    ▼
//!                planners
//!                    │
//!                    ▼
//!              verification
//! ```
//!
//! # Separation from calendar.rs
//!
//! Availability and reservation are intentionally different concepts.
//!
//! `availability.rs` answers:
//!
//! ```text
//! "May this resource be used?"
 //! ```
//!
//! `calendar.rs` answers:
//!
//! ```text
//! "What has already been placed on this resource over time?"
//! ```
//!
//! `reservation.rs` answers:
//!
//! ```text
//! "Which operation claims which resource over which interval?"
//! ```
//!
//! Therefore an available resource may still be unavailable to a particular
//! operation because its calendar is occupied.
//!
//! Conversely, an empty calendar does not imply that a disabled or unknown
//! resource is schedulable.
//!
//! The effective schedulability predicate is therefore conceptually:
//!
//! ```text
//! resource exists
//!     AND
//! resource is operationally available
//!     AND
//! resource satisfies target capabilities
//!     AND
//! requested interval satisfies availability windows
//!     AND
//! calendar/resource capacity permits the reservation
//! ```
//!
//! This module owns only the availability portion.
//!
//! # Canonical identity ownership
//!
//! Resource identity comes from the canonical IR:
//!
//! ```text
//! crate::quantum::ir::core::identity::ResourceId
//! ```
//!
//! Logical and physical qubit identities remain owned by:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! This file does not define replacement qubit identities.
//!
//! If a physical qubit is exposed as a scheduler resource, its canonical
//! `PhysicalQubitId` must be translated to the corresponding canonical
//! `ResourceId` by the resource/hardware adapter layer.
//!
//! No qubit-specific assumption is embedded here.
//!
//! # Universal-program principle
//!
//! Nothing in this file assumes:
//!
//! - a fixed number of qubits;
//! - a fixed number of resources;
//! - a fixed number of availability windows;
//! - a fixed schedule duration;
//! - a fixed machine size;
//! - a fixed topology;
//! - a fixed quantum technology;
//! - a fixed number of channels;
//! - a fixed gate arity;
//! - a fixed timing unit;
//! - a particular hardware vendor.
//!
//! The same availability model therefore applies to:
//!
//! - one qubit;
//! - a small QPU;
//! - a large QPU;
//! - a modular QPU;
//! - a distributed quantum computer;
//! - a quantum network;
//! - heterogeneous quantum systems;
//! - future quantum architectures.
//!
//! "Infinity" means that this module introduces no artificial finite machine
//! size or resource-count ceiling. A concrete compilation remains bounded by
//! the actual target, compiler process, address space, and available host
//! resources.
//!
//! # Time semantics
//!
//! Scheduling time uses:
//!
//! ```text
//! crate::quantum::scheduling::types::TimePoint
//! crate::quantum::scheduling::types::Duration
//! crate::quantum::scheduling::types::TimeInterval
//! ```
//!
//! Intervals are half-open:
//!
//! ```text
//! [start, end)
//! ```
//!
//! Therefore:
//!
//! ```text
//! [0, 10) and [10, 20)
//! ```
//!
//! do not overlap.
//!
//! This makes availability boundaries composable with reservation calendars.
//!
//! # Availability state semantics
//!
//! The states have intentionally conservative meanings:
//!
//! `Available`
//!     The resource is known to be operationally available.
//!
//! `Degraded`
//!     The resource is usable, but its quality or operational condition is
//!     below the normal target state. Whether a scheduler may use it is a
//!     policy decision outside this file.
//!
//! `Unavailable`
//!     The resource must not be selected for normal scheduling during the
//!     applicable interval.
//!
//! `Unknown`
//!     The resource's availability cannot be established with sufficient
//!     confidence.
//!
//! A production scheduler should normally treat `Unknown` conservatively,
//! unless an explicit policy says otherwise.
//!
//! This module does not impose that policy.
//!
//! # Non-overlapping explicit windows
//!
//! Explicit availability windows belonging to one resource must not overlap.
//!
//! This is deliberate.
//!
//! Without this invariant, a query could receive multiple contradictory states
//! for the same instant and would need an implicit precedence system.
//!
//! Rather than hiding precedence inside availability semantics, this module
//! requires the input representation to be unambiguous.
//!
//! If two external sources provide overlapping information, the adapter that
//! combines them must resolve that conflict before inserting the resulting
//! windows.
//!
//! # Baseline state
//!
//! Each resource has a baseline availability state.
//!
//! Explicit windows override the baseline for their intervals.
//!
//! Example:
//!
//! ```text
//! baseline = Available
//!
//! [100, 200) = Maintenance
//! ```
//!
//! In this case:
//!
//! ```text
//! t = 50   -> Available
//! t = 150  -> Unavailable
//! t = 250  -> Available
//! ```
//!
//! This avoids allocating an availability record for every point in time.
//!
//! # Scalability
//!
//! This implementation deliberately uses ordered interval starts rather than
//! a time grid.
//!
//! It does NOT allocate:
//!
//! ```text
//! [resource][time]
//! ```
//!
//! or:
//!
//! ```text
//! Vec<State> for every clock tick
//! ```
//!
//! Such representations scale with the size of the timeline rather than with
//! actual state changes and therefore become inappropriate for very long or
//! sparse schedules.
//!
//! Instead, only actual availability transitions are stored.
//!
//! Memory usage is therefore proportional to the number of explicit state
//! intervals rather than the total duration of the execution timeline.
//!
//! # Determinism
//!
//! All stored resources and windows use ordered maps.
//!
//! Iteration is deterministic for identical inputs.
//!
//! No wall-clock timestamps, hash-map iteration order, pointers, or hidden
//! randomness participate in availability semantics.
//!
//! # Thread safety
//!
//! The types contain ordinary owned values only.
//!
//! They use no:
//!
//! - global mutable state;
//! - locks;
//! - raw pointers;
//! - interior mutability;
//! - unsafe code.
//!
//! Read-only values may therefore be shared or transferred according to normal
//! Rust ownership rules. Mutable synchronization, if desired, belongs to the
//! caller/runtime boundary.
//!
//! # Error handling
//!
//! Invalid availability definitions return the canonical:
//!
//! ```text
//! SchedulingResult<T>
//! ```
//!
//! and:
//!
//! ```text
//! SchedulingError::InvalidInput
//! ```
//!
//! This module does not create a second availability-specific error hierarchy.
//!
//! # Integration contract
//!
//! Hardware adapters should construct `ResourceAvailability` values from
//! target state snapshots.
//!
//! `pool.rs` should use:
//!
//! ```text
//! AvailabilityRegistry::get
//! ResourceAvailability::state_at
//! ResourceAvailability::is_available_at
//! ResourceAvailability::is_available_for
//! ```
//!
//! `calendar.rs` should NOT duplicate availability windows. Instead, callers
//! should check both:
//!
//! ```text
//! availability
//! calendar
//! ```
//!
//! independently.
//!
//! Planners should use availability as a feasibility predicate before creating
//! reservations.
//!
//! Verification should independently verify that every scheduled reservation
//! falls inside an acceptable availability state.
//!
//! Dynamic scheduling may replace or update a resource's availability snapshot
//! between scheduling epochs. Existing availability values remain ordinary
//! owned data and therefore do not require this module to know about runtime
//! event transport.
//!
//! # Finish-once contract
//!
//! This file depends only on:
//!
//! ```text
//! scheduling::types
//! scheduling::errors
//! canonical ResourceId
//! ```
//!
//! It does not depend on:
//!
//! ```text
//! pool.rs
//! calendar.rs
//! reservation.rs
//! planner modules
//! algorithms
//! hardware implementations
//! routing
//! QEC
//! runtime
//! ```
//!
//! Therefore downstream integration does not require this file to be reopened
//! merely because another scheduling subsystem is implemented.
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
//! =============================================================================
//! Safety boundary
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::BTreeMap;
use std::fmt;

use crate::quantum::ir::core::identity::ResourceId;

use super::super::errors::{SchedulingError, SchedulingResult};
use super::super::types::{TimeInterval, TimePoint};

// =============================================================================
// Availability state
// =============================================================================

/// Operational availability state of one schedulable resource.
///
/// Availability describes whether a resource may be considered for scheduling.
/// It does not describe resource capacity, reservations, or operation support.
///
/// The enum is intentionally `non_exhaustive` so future Zamani versions can
/// introduce additional states without requiring the current API to encode
/// every possible hardware condition.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum AvailabilityState {
    /// Resource is known to be operationally available.
    Available,

    /// Resource is usable but operating in a degraded condition.
    ///
    /// The scheduler policy decides whether degraded resources are acceptable.
    Degraded,

    /// Resource is explicitly unavailable.
    Unavailable,

    /// Resource availability cannot currently be established.
    Unknown,
}

impl AvailabilityState {
    /// Returns whether the state represents normal availability.
    #[must_use]
    pub const fn is_available(self) -> bool {
        matches!(self, Self::Available)
    }

    /// Returns whether the resource is potentially usable but degraded.
    #[must_use]
    pub const fn is_degraded(self) -> bool {
        matches!(self, Self::Degraded)
    }

    /// Returns whether the resource is explicitly unavailable.
    #[must_use]
    pub const fn is_unavailable(self) -> bool {
        matches!(self, Self::Unavailable)
    }

    /// Returns whether the resource state is unknown.
    #[must_use]
    pub const fn is_unknown(self) -> bool {
        matches!(self, Self::Unknown)
    }

    /// Returns whether the state is usable under the default strict policy.
    ///
    /// The default strict interpretation accepts only `Available`.
    ///
    /// This is a state predicate, not a scheduler policy. A higher-level policy
    /// may explicitly decide that `Degraded` is acceptable.
    #[must_use]
    pub const fn is_strictly_usable(self) -> bool {
        matches!(self, Self::Available)
    }

    /// Returns a stable machine-readable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Available => "available",
            Self::Degraded => "degraded",
            Self::Unavailable => "unavailable",
            Self::Unknown => "unknown",
        }
    }
}

impl Default for AvailabilityState {
    fn default() -> Self {
        Self::Unknown
    }
}

impl fmt::Display for AvailabilityState {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Availability window
// =============================================================================

/// One explicit time interval during which a resource has a particular
/// availability state.
///
/// Windows use the scheduler's canonical half-open interval:
///
/// ```text
/// [start, end)
/// ```
///
/// Windows are immutable after construction.
///
/// `ResourceAvailability` is responsible for enforcing the additional
/// invariant that explicit windows for one resource do not overlap.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AvailabilityWindow {
    interval: TimeInterval,
    state: AvailabilityState,
    reason: Option<String>,
}

impl AvailabilityWindow {
    /// Creates an availability window without a diagnostic reason.
    ///
    /// The interval itself is already validated by `TimeInterval`.
    #[must_use]
    pub fn new(interval: TimeInterval, state: AvailabilityState) -> Self {
        Self {
            interval,
            state,
            reason: None,
        }
    }

    /// Creates an availability window with a diagnostic reason.
    ///
    /// The reason is informational. Scheduler correctness must never depend on
    /// parsing it.
    #[must_use]
    pub fn with_reason(
        interval: TimeInterval,
        state: AvailabilityState,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            interval,
            state,
            reason: Some(reason.into()),
        }
    }

    /// Returns the covered interval.
    #[must_use]
    pub const fn interval(&self) -> TimeInterval {
        self.interval
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

    /// Returns the availability state.
    #[must_use]
    pub const fn state(&self) -> AvailabilityState {
        self.state
    }

    /// Returns the optional diagnostic reason.
    #[must_use]
    pub fn reason(&self) -> Option<&str> {
        self.reason.as_deref()
    }

    /// Returns whether this window overlaps another interval.
    #[must_use]
    pub fn overlaps(&self, interval: TimeInterval) -> bool {
        self.interval.overlaps(interval)
    }

    /// Returns whether the interval contains a point.
    #[must_use]
    pub fn contains(&self, point: TimePoint) -> bool {
        self.interval.contains(point)
    }
}

// =============================================================================
// Resource availability
// =============================================================================

/// Temporal availability model for one scheduler resource.
///
/// This is the primary availability abstraction used by scheduling.
///
/// The model consists of:
///
/// ```text
/// resource identity
/// baseline state
/// explicit non-overlapping state windows
/// ```
///
/// Explicit windows override the baseline.
///
/// No reservation information is stored here.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourceAvailability {
    resource: ResourceId,
    baseline: AvailabilityState,
    windows: BTreeMap<TimePoint, AvailabilityWindow>,
}

impl ResourceAvailability {
    /// Creates an availability model with the supplied baseline state.
    #[must_use]
    pub fn new(resource: ResourceId, baseline: AvailabilityState) -> Self {
        Self {
            resource,
            baseline,
            windows: BTreeMap::new(),
        }
    }

    /// Creates an availability model that is available unless an explicit
    /// window says otherwise.
    #[must_use]
    pub fn available(resource: ResourceId) -> Self {
        Self::new(resource, AvailabilityState::Available)
    }

    /// Creates an availability model whose default state is unknown.
    #[must_use]
    pub fn unknown(resource: ResourceId) -> Self {
        Self::new(resource, AvailabilityState::Unknown)
    }

    /// Returns the resource identity.
    #[must_use]
    pub const fn resource(&self) -> ResourceId {
        self.resource
    }

    /// Returns the baseline state.
    #[must_use]
    pub const fn baseline(&self) -> AvailabilityState {
        self.baseline
    }

    /// Changes the baseline state.
    ///
    /// Existing explicit windows are unaffected.
    pub fn set_baseline(&mut self, baseline: AvailabilityState) {
        self.baseline = baseline;
    }

    /// Returns the number of explicit availability windows.
    ///
    /// This is informational and does not represent a scheduler limit.
    #[must_use]
    pub fn window_count(&self) -> usize {
        self.windows.len()
    }

    /// Returns whether there are no explicit windows.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.windows.is_empty()
    }

    /// Returns an iterator over explicit windows in deterministic chronological
    /// order.
    pub fn windows(
        &self,
    ) -> impl Iterator<Item = &AvailabilityWindow> {
        self.windows.values()
    }

    /// Returns the explicit window beginning at `start`, if present.
    #[must_use]
    pub fn window_at_start(
        &self,
        start: TimePoint,
    ) -> Option<&AvailabilityWindow> {
        self.windows.get(&start)
    }

    /// Inserts an explicit availability window.
    ///
    /// Explicit windows for the same resource must not overlap.
    ///
    /// Returns an error if the new window overlaps an existing explicit window.
    ///
    /// Zero-duration windows are accepted as values but have no effect on
    /// point/interval availability because scheduling intervals are half-open.
    pub fn insert_window(
        &mut self,
        window: AvailabilityWindow,
    ) -> SchedulingResult<()> {
        let interval = window.interval();

        if let Some(existing) = self.find_overlapping_window(interval) {
            return Err(SchedulingError::InvalidInput {
                reason: format!(
                    "availability window {interval} for resource {} overlaps \
                     existing availability window {}",
                    self.resource,
                    existing.interval()
                ),
            });
        }

        let start = window.start();

        if self.windows.insert(start, window).is_some() {
            return Err(SchedulingError::InvalidInput {
                reason: format!(
                    "availability window beginning at {start} already exists \
                     for resource {}",
                    self.resource
                ),
            });
        }

        Ok(())
    }

    /// Removes the explicit window beginning at `start`.
    ///
    /// Returns the removed window when present.
    pub fn remove_window(
        &mut self,
        start: TimePoint,
    ) -> Option<AvailabilityWindow> {
        self.windows.remove(&start)
    }

    /// Removes all explicit windows that overlap `interval`.
    ///
    /// The removed windows are returned in deterministic chronological order.
    ///
    /// This operation is useful when replacing a target availability snapshot
    /// for a bounded region.
    pub fn remove_overlapping(
        &mut self,
        interval: TimeInterval,
    ) -> Vec<AvailabilityWindow> {
        let starts: Vec<TimePoint> = self
            .windows
            .iter()
            .filter_map(|(start, window)| {
                if window.overlaps(interval) {
                    Some(*start)
                } else {
                    None
                }
            })
            .collect();

        starts
            .into_iter()
            .filter_map(|start| self.windows.remove(&start))
            .collect()
    }

    /// Returns the availability state at one point.
    ///
    /// An explicit window takes precedence over the baseline state.
    #[must_use]
    pub fn state_at(&self, point: TimePoint) -> AvailabilityState {
        if let Some((_, window)) = self.windows.range(..=point).next_back() {
            if window.contains(point) {
                return window.state();
            }
        }

        self.baseline
    }

    /// Returns the explicit window containing a point, if any.
    #[must_use]
    pub fn explicit_window_at(
        &self,
        point: TimePoint,
    ) -> Option<&AvailabilityWindow> {
        self.windows
            .range(..=point)
            .next_back()
            .and_then(|(_, window)| {
                if window.contains(point) {
                    Some(window)
                } else {
                    None
                }
            })
    }

    /// Returns whether the resource is strictly available at a point.
    ///
    /// Only `Available` returns true.
    ///
    /// `Degraded` is deliberately not accepted here because whether degraded
    /// operation is permissible is a scheduling policy decision.
    #[must_use]
    pub fn is_available_at(&self, point: TimePoint) -> bool {
        self.state_at(point).is_strictly_usable()
    }

    /// Returns whether the resource is available under a policy that accepts
    /// degraded resources.
    #[must_use]
    pub fn is_usable_at(&self, point: TimePoint, allow_degraded: bool) -> bool {
        match self.state_at(point) {
            AvailabilityState::Available => true,
            AvailabilityState::Degraded => allow_degraded,
            AvailabilityState::Unavailable | AvailabilityState::Unknown => false,
        }
    }

    /// Returns whether the resource is usable for the complete interval.
    ///
    /// The interval is checked by state transitions rather than by allocating a
    /// point for every time coordinate.
    ///
    /// For an empty interval `[t, t)`, this returns true because no positive
    /// duration is occupied.
    #[must_use]
    pub fn is_usable_for(
        &self,
        interval: TimeInterval,
        allow_degraded: bool,
    ) -> bool {
        if interval.is_empty() {
            return true;
        }

        if !self.state_at(interval.start()).is_usable(allow_degraded) {
            return false;
        }

        self.windows
            .range(interval.start()..interval.end())
            .all(|(_, window)| {
                !window.overlaps(interval)
                    || window.state().is_usable(allow_degraded)
            })
    }

    /// Returns whether the resource is strictly available for the complete
    /// interval.
    #[must_use]
    pub fn is_available_for(&self, interval: TimeInterval) -> bool {
        self.is_usable_for(interval, false)
    }

    /// Returns the first explicit window that prevents strict availability for
    /// the requested interval.
    ///
    /// If the baseline state itself is not strictly available, the returned
    /// value is `None` because no explicit window is responsible for the
    /// unavailability.
    #[must_use]
    pub fn first_unavailable_window(
        &self,
        interval: TimeInterval,
        allow_degraded: bool,
    ) -> Option<&AvailabilityWindow> {
        if interval.is_empty() {
            return None;
        }

        if !self.state_at(interval.start()).is_usable(allow_degraded) {
            return None;
        }

        self.windows
            .range(interval.start()..interval.end())
            .map(|(_, window)| window)
            .find(|window| {
                window.overlaps(interval)
                    && !window.state().is_usable(allow_degraded)
            })
    }

    /// Returns all explicit windows overlapping an interval.
    ///
    /// Results are chronological and deterministic.
    #[must_use]
    pub fn overlapping_windows(
        &self,
        interval: TimeInterval,
    ) -> Vec<&AvailabilityWindow> {
        self.windows
            .range(..interval.end())
            .filter_map(|(_, window)| {
                if window.overlaps(interval) {
                    Some(window)
                } else {
                    None
                }
            })
            .collect()
    }

    /// Returns the first time at or after `start` at which the resource enters
    /// the requested state.
    ///
    /// This method does not search an artificial time grid.
    ///
    /// It examines actual availability transitions only.
    #[must_use]
    pub fn next_state_change_at_or_after(
        &self,
        start: TimePoint,
    ) -> Option<TimePoint> {
        if let Some(window) = self.explicit_window_at(start) {
            return Some(window.end());
        }

        self.windows
            .range(start..)
            .next()
            .map(|(_, window)| window.start())
    }

    /// Returns all state-transition boundaries at or after `start` and before
    /// `end`.
    ///
    /// This is useful to event-driven schedulers.
    #[must_use]
    pub fn transition_points(
        &self,
        interval: TimeInterval,
    ) -> Vec<TimePoint> {
        if interval.is_empty() {
            return Vec::new();
        }

        let mut points = Vec::new();

        if self.state_at(interval.start()) != self.baseline {
            points.push(interval.start());
        }

        for (_, window) in self.windows.range(interval.start()..interval.end()) {
            if window.start().value() >= interval.end().value() {
                break;
            }

            if window.start().value() >= interval.start().value() {
                points.push(window.start());
            }

            if window.end().value() > interval.start().value()
                && window.end().value() < interval.end().value()
            {
                points.push(window.end());
            }
        }

        points.sort_unstable();
        points.dedup();
        points
    }

    /// Returns the first explicit availability window overlapping `interval`
    /// that has the requested state.
    #[must_use]
    pub fn first_window_with_state(
        &self,
        interval: TimeInterval,
        state: AvailabilityState,
    ) -> Option<&AvailabilityWindow> {
        self.overlapping_windows(interval)
            .into_iter()
            .find(|window| window.state() == state)
    }

    /// Finds an explicit window overlapping `interval`.
    fn find_overlapping_window(
        &self,
        interval: TimeInterval,
    ) -> Option<&AvailabilityWindow> {
        if interval.is_empty() {
            return None;
        }

        self.windows
            .range(..interval.end())
            .map(|(_, window)| window)
            .find(|window| window.overlaps(interval))
    }

    /// Validates the complete availability model.
    ///
    /// This is intentionally separate from construction so callers can use it
    /// as a verification boundary after receiving data from an external
    /// adapter.
    pub fn validate(&self) -> SchedulingResult<()> {
        let mut previous: Option<&AvailabilityWindow> = None;

        for window in self.windows.values() {
            if let Some(previous_window) = previous {
                if previous_window.overlaps(window.interval()) {
                    return Err(SchedulingError::InvalidInput {
                        reason: format!(
                            "overlapping availability windows for resource {}: \
                             {} and {}",
                            self.resource,
                            previous_window.interval(),
                            window.interval()
                        ),
                    });
                }
            }

            previous = Some(window);
        }

        Ok(())
    }
}

// =============================================================================
// Availability state helpers
// =============================================================================

impl AvailabilityState {
    /// Returns whether this state is usable under the supplied policy.
    ///
    /// `allow_degraded = false` accepts only `Available`.
    ///
    /// `allow_degraded = true` accepts `Available` and `Degraded`.
    #[must_use]
    pub const fn is_usable(self, allow_degraded: bool) -> bool {
        match self {
            Self::Available => true,
            Self::Degraded => allow_degraded,
            Self::Unavailable | Self::Unknown => false,
        }
    }
}

// =============================================================================
// Availability registry
// =============================================================================

/// Deterministic collection of availability models indexed by canonical
/// `ResourceId`.
///
/// This is the availability boundary for a complete scheduling target.
///
/// It deliberately contains no fixed resource count.
///
/// A registry containing one resource and a registry containing millions of
/// resources use exactly the same representation and API.
///
/// Actual memory consumption is proportional to the number of resources and
/// explicit availability windows supplied by the target.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AvailabilityRegistry {
    resources: BTreeMap<ResourceId, ResourceAvailability>,
}

impl AvailabilityRegistry {
    /// Creates an empty availability registry.
    #[must_use]
    pub fn new() -> Self {
        Self {
            resources: BTreeMap::new(),
        }
    }

    /// Returns the number of resources represented by the registry.
    ///
    /// This is informational only and is not a machine-size limit.
    #[must_use]
    pub fn len(&self) -> usize {
        self.resources.len()
    }

    /// Returns whether the registry contains no resource availability models.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.resources.is_empty()
    }

    /// Inserts or replaces the complete availability model for a resource.
    ///
    /// Replacing a resource model is atomic from the caller's perspective:
    /// either the new model is accepted, or the registry remains unchanged.
    pub fn insert(
        &mut self,
        availability: ResourceAvailability,
    ) -> SchedulingResult<Option<ResourceAvailability>> {
        availability.validate()?;

        Ok(self
            .resources
            .insert(availability.resource(), availability))
    }

    /// Creates and inserts an availability model using the supplied baseline.
    ///
    /// Returns a mutable reference to the inserted model.
    pub fn ensure(
        &mut self,
        resource: ResourceId,
        baseline: AvailabilityState,
    ) -> &mut ResourceAvailability {
        self.resources
            .entry(resource)
            .or_insert_with(|| ResourceAvailability::new(resource, baseline))
    }

    /// Returns availability information for a resource.
    #[must_use]
    pub fn get(
        &self,
        resource: ResourceId,
    ) -> Option<&ResourceAvailability> {
        self.resources.get(&resource)
    }

    /// Returns mutable availability information for a resource.
    pub fn get_mut(
        &mut self,
        resource: ResourceId,
    ) -> Option<&mut ResourceAvailability> {
        self.resources.get_mut(&resource)
    }

    /// Removes a resource's availability model.
    pub fn remove(
        &mut self,
        resource: ResourceId,
    ) -> Option<ResourceAvailability> {
        self.resources.remove(&resource)
    }

    /// Returns whether a resource has an availability model.
    #[must_use]
    pub fn contains(&self, resource: ResourceId) -> bool {
        self.resources.contains_key(&resource)
    }

    /// Returns deterministic resource identifiers.
    pub fn resource_ids(&self) -> impl Iterator<Item = ResourceId> + '_ {
        self.resources.keys().copied()
    }

    /// Returns deterministic availability models.
    pub fn resources(
        &self,
    ) -> impl Iterator<Item = &ResourceAvailability> {
        self.resources.values()
    }

    /// Returns the state of a resource at a particular time.
    ///
    /// If no availability model exists, the result is `Unknown`.
    ///
    /// This conservative default prevents an absent availability description
    /// from accidentally being interpreted as permission to schedule.
    #[must_use]
    pub fn state_at(
        &self,
        resource: ResourceId,
        point: TimePoint,
    ) -> AvailabilityState {
        self.resources
            .get(&resource)
            .map_or(AvailabilityState::Unknown, |availability| {
                availability.state_at(point)
            })
    }

    /// Returns whether a resource is strictly available at a particular time.
    ///
    /// An absent resource model returns false.
    #[must_use]
    pub fn is_available_at(
        &self,
        resource: ResourceId,
        point: TimePoint,
    ) -> bool {
        self.state_at(resource, point).is_strictly_usable()
    }

    /// Returns whether a resource is usable for a complete interval.
    ///
    /// An absent resource model returns false.
    #[must_use]
    pub fn is_usable_for(
        &self,
        resource: ResourceId,
        interval: TimeInterval,
        allow_degraded: bool,
    ) -> bool {
        self.resources
            .get(&resource)
            .is_some_and(|availability| {
                availability.is_usable_for(interval, allow_degraded)
            })
    }

    /// Returns whether a resource is strictly available for a complete
    /// interval.
    #[must_use]
    pub fn is_available_for(
        &self,
        resource: ResourceId,
        interval: TimeInterval,
    ) -> bool {
        self.is_usable_for(resource, interval, false)
    }

    /// Validates every availability model in the registry.
    pub fn validate(&self) -> SchedulingResult<()> {
        for availability in self.resources.values() {
            availability.validate()?;
        }

        Ok(())
    }

    /// Removes every resource availability model.
    pub fn clear(&mut self) {
        self.resources.clear();
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::quantum::ir::core::identity::ResourceId;
    use crate::quantum::scheduling::types::TimeInterval;

    fn resource(value: u64) -> ResourceId {
        ResourceId::from(value)
    }

    fn point(value: u128) -> TimePoint {
        TimePoint::new(value)
    }

    fn interval(start: u128, end: u128) -> TimeInterval {
        TimeInterval::new(point(start), point(end))
            .expect("test interval must be valid")
    }

    #[test]
    fn baseline_available_resource_is_available() {
        let availability = ResourceAvailability::available(resource(1));

        assert_eq!(
            availability.state_at(point(0)),
            AvailabilityState::Available
        );
        assert!(availability.is_available_at(point(0)));
    }

    #[test]
    fn baseline_unknown_resource_is_not_available() {
        let availability = ResourceAvailability::unknown(resource(1));

        assert_eq!(
            availability.state_at(point(0)),
            AvailabilityState::Unknown
        );
        assert!(!availability.is_available_at(point(0)));
    }

    #[test]
    fn explicit_window_overrides_baseline() {
        let mut availability =
            ResourceAvailability::available(resource(1));

        availability
            .insert_window(AvailabilityWindow::new(
                interval(10, 20),
                AvailabilityState::Unavailable,
            ))
            .expect("window should be accepted");

        assert_eq!(
            availability.state_at(point(5)),
            AvailabilityState::Available
        );
        assert_eq!(
            availability.state_at(point(10)),
            AvailabilityState::Unavailable
        );
        assert_eq!(
            availability.state_at(point(19)),
            AvailabilityState::Unavailable
        );
        assert_eq!(
            availability.state_at(point(20)),
            AvailabilityState::Available
        );
    }

    #[test]
    fn touching_windows_do_not_overlap() {
        let mut availability =
            ResourceAvailability::available(resource(1));

        availability
            .insert_window(AvailabilityWindow::new(
                interval(0, 10),
                AvailabilityState::Unavailable,
            ))
            .expect("first window should be accepted");

        availability
            .insert_window(AvailabilityWindow::new(
                interval(10, 20),
                AvailabilityState::Degraded,
            ))
            .expect("touching window should be accepted");
    }

    #[test]
    fn overlapping_windows_are_rejected() {
        let mut availability =
            ResourceAvailability::available(resource(1));

        availability
            .insert_window(AvailabilityWindow::new(
                interval(10, 20),
                AvailabilityState::Unavailable,
            ))
            .expect("first window should be accepted");

        assert!(
            availability
                .insert_window(AvailabilityWindow::new(
                    interval(15, 25),
                    AvailabilityState::Degraded,
                ))
                .is_err()
        );
    }

    #[test]
    fn strict_policy_rejects_degraded_state() {
        let mut availability =
            ResourceAvailability::available(resource(1));

        availability
            .insert_window(AvailabilityWindow::new(
                interval(10, 20),
                AvailabilityState::Degraded,
            ))
            .expect("window should be accepted");

        assert!(!availability.is_available_at(point(15)));
        assert!(availability.is_usable_at(point(15), true));
    }

    #[test]
    fn interval_availability_checks_state_transitions() {
        let mut availability =
            ResourceAvailability::available(resource(1));

        availability
            .insert_window(AvailabilityWindow::new(
                interval(10, 20),
                AvailabilityState::Unavailable,
            ))
            .expect("window should be accepted");

        assert!(availability.is_available_for(interval(0, 10)));
        assert!(!availability.is_available_for(interval(0, 11)));
        assert!(!availability.is_available_for(interval(15, 25)));
        assert!(availability.is_available_for(interval(20, 30)));
    }

    #[test]
    fn empty_interval_is_usable() {
        let mut availability =
            ResourceAvailability::available(resource(1));

        availability
            .insert_window(AvailabilityWindow::new(
                interval(10, 20),
                AvailabilityState::Unavailable,
            ))
            .expect("window should be accepted");

        assert!(availability.is_available_for(interval(15, 15)));
    }

    #[test]
    fn overlapping_window_query_is_deterministic() {
        let mut availability =
            ResourceAvailability::available(resource(1));

        availability
            .insert_window(AvailabilityWindow::new(
                interval(10, 20),
                AvailabilityState::Unavailable,
            ))
            .expect("window should be accepted");

        availability
            .insert_window(AvailabilityWindow::new(
                interval(30, 40),
                AvailabilityState::Degraded,
            ))
            .expect("window should be accepted");

        let windows = availability.overlapping_windows(interval(15, 35));

        assert_eq!(windows.len(), 2);
        assert_eq!(windows[0].start(), point(10));
        assert_eq!(windows[1].start(), point(30));
    }

    #[test]
    fn remove_window_removes_exact_start() {
        let mut availability =
            ResourceAvailability::available(resource(1));

        availability
            .insert_window(AvailabilityWindow::new(
                interval(10, 20),
                AvailabilityState::Unavailable,
            ))
            .expect("window should be accepted");

        assert!(availability.remove_window(point(10)).is_some());
        assert!(availability.is_available_for(interval(10, 20)));
    }

    #[test]
    fn registry_unknown_resource_is_conservative() {
        let registry = AvailabilityRegistry::new();

        assert_eq!(
            registry.state_at(resource(42), point(0)),
            AvailabilityState::Unknown
        );

        assert!(!registry.is_available_at(resource(42), point(0)));
    }

    #[test]
    fn registry_insert_and_lookup() {
        let mut registry = AvailabilityRegistry::new();

        registry
            .insert(ResourceAvailability::available(resource(7)))
            .expect("resource availability should be accepted");

        assert!(registry.contains(resource(7)));
        assert_eq!(registry.len(), 1);
    }

    #[test]
    fn registry_replacement_is_supported() {
        let mut registry = AvailabilityRegistry::new();

        registry
            .insert(ResourceAvailability::available(resource(7)))
            .expect("initial availability should be accepted");

        let mut replacement =
            ResourceAvailability::unknown(resource(7));

        replacement
            .insert_window(AvailabilityWindow::new(
                interval(100, 200),
                AvailabilityState::Available,
            ))
            .expect("window should be accepted");

        let previous = registry
            .insert(replacement)
            .expect("replacement should be accepted");

        assert!(previous.is_some());
        assert_eq!(
            registry.state_at(resource(7), point(50)),
            AvailabilityState::Unknown
        );
        assert_eq!(
            registry.state_at(resource(7), point(150)),
            AvailabilityState::Available
        );
    }

    #[test]
    fn validation_succeeds_for_ordered_non_overlapping_windows() {
        let mut availability =
            ResourceAvailability::available(resource(1));

        availability
            .insert_window(AvailabilityWindow::new(
                interval(0, 10),
                AvailabilityState::Unavailable,
            ))
            .expect("window should be accepted");

        availability
            .insert_window(AvailabilityWindow::new(
                interval(10, 20),
                AvailabilityState::Available,
            ))
            .expect("window should be accepted");

        availability
            .validate()
            .expect("availability should validate");
    }
}