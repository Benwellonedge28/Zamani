//! Zamani Quantum Scheduling — Temporal Scheduling Windows
//!
//! Path:
//!     src/quantum/scheduling/timing/windows.rs
//!
//! # Purpose
//!
//! This module defines the scheduler's temporal admissibility windows.
//!
//! A scheduling window answers:
//!
//! > At which semantic times is an operation allowed to start and/or finish?
//!
//! It does NOT decide:
//!
//! - which operation should execute first;
//! - which physical qubit should be selected;
//! - which hardware backend should execute an operation;
//! - how a dependency graph is constructed;
//! - how resources are allocated;
//! - how QEC is performed;
//! - how a schedule is optimized.
//!
//! Those responsibilities belong to other scheduling subsystems.
//!
//! # Architectural position
//!
//! ```text
//! quantum::ir
//!      |
//!      v
//! mapped executable operations
//!      |
//!      v
//! scheduling::ir
//!      |
//!      +--------------------+
//!      |                    |
//!      v                    v
//! dependencies          timing/windows
//!      |                    |
//!      +---------+----------+
//!                |
//!                v
//!           scheduling
//!                |
//!                v
//!        target/hardware
//! ```
//!
//! # Canonical timing ownership
//!
//! This module MUST use the canonical timing primitives:
//!
//! ```text
//! crate::quantum::ir::timing::Duration
//! crate::quantum::ir::timing::TimePoint
//! crate::quantum::ir::timing::TimeInterval
//! ```
//!
//! It must NOT define another:
//!
//! - `Duration`;
//! - `TimePoint`;
//! - `TimeInterval`.
//!
//! The repository already establishes `quantum::ir::timing` as the semantic
//! owner of these concepts.
//!
//! # Window semantics
//!
//! A `TimeWindow` represents an inclusive admissible range:
//!
//! ```text
//! [earliest, latest]
//! ```
//!
//! Either boundary may be absent:
//!
//! ```text
//! [earliest, +infinity)
//! (-infinity, latest]
//! (-infinity, +infinity)
//! ```
//!
//! The word "infinity" here means an absent semantic bound. It does not mean
//! that an infinite value is stored in the `u128` time domain.
//!
//! `TimePoint` itself remains non-negative because it is owned by the
//! canonical semantic timing layer.
//!
//! # Execution windows
//!
//! `ExecutionWindow` combines:
//!
//! - start-time admissibility;
//! - finish-time admissibility;
//! - optional operation duration.
//!
//! This permits constraints such as:
//!
//! ```text
//! release time
//! earliest start
//! latest start
//! earliest finish
//! deadline
//! latest finish
//! ```
//!
//! without forcing every operation to have every kind of bound.
//!
//! # Half-open versus closed semantics
//!
//! A window constrains a POINT in time, so its admissibility boundaries are
//! inclusive:
//!
//! ```text
//! earliest <= t <= latest
//! ```
//!
//! This is different from resource occupancy intervals, which use canonical
//! half-open intervals:
//!
//! ```text
//! [start, end)
//! ```
//!
//! Consequently:
//!
//! - windows use inclusive point bounds;
//! - execution/resource intervals use half-open occupancy semantics.
//!
//! This distinction is intentional.
//!
//! # Duration interaction
//!
//! A start window alone can be checked without knowing duration.
//!
//! Once duration is known, finish constraints can be validated exactly:
//!
//! ```text
//! finish = start + duration
//! ```
//!
//! with checked arithmetic.
//!
//! No floating-point arithmetic is used.
//!
//! # Hardware independence
//!
//! This module contains no:
//!
//! - `dt`;
//! - nanosecond constants;
//! - sample rates;
//! - channel counts;
//! - qubit counts;
//! - topology assumptions;
//! - vendor names;
//! - technology-specific timing values.
//!
//! Hardware timing resolution is supplied by the hardware/timing adapter and
//! consumed by `timing::resolution` and `timing::alignment`.
//!
//! # Scalability
//!
//! A window is O(1) state and O(1) to validate.
//!
//! It contains no data structure proportional to:
//!
//! - number of qubits;
//! - number of operations;
//! - schedule depth;
//! - machine size;
//! - number of resources.
//!
//! Therefore one window has the same asymptotic memory footprint whether the
//! target contains one qubit or a very large distributed system.
//!
//! The scheduler remains responsible for scalable storage of many windows.
//!
//! # Determinism
//!
//! All comparisons are exact and deterministic.
//!
//! No:
//!
//! - floating point;
//! - system clock;
//! - random state;
//! - global state;
//! - platform-specific behavior
//!
//! is used.
//!
//! # Safety
//!
//! This module uses only safe Rust.
//!
//! Rust itself enforces the no-unsafe requirement with:
//!
//! ```text
//! #![forbid(unsafe_code)]
//! ```
//!
//! # Rust compatibility
//!
//! Supported:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust.
//!
//! No nightly features or external dependencies are required.
//!
//! # Integration contract
//!
//! Upstream:
//!
//! ```text
//! quantum::ir::timing
//! scheduling::ir
//! scheduling::constraints
//! hardware adapter
//! ```
//!
//! Downstream:
//!
//! ```text
//! scheduling::planners
//! scheduling::algorithms
//! scheduling::verification::timing
//! scheduling::resources
//! scheduling::optimization
//! scheduling::diagnostics
//! scheduling::serialization
//! ```
//!
//! `context.rs` should expose target timing capabilities that ultimately
//! provide or contain these windows.
//!
//! `constraints.rs` may convert operation-specific temporal requirements into
//! `ExecutionWindow` values.
//!
//! Planners should only ask whether candidate placements satisfy a window;
//! they should not reimplement window arithmetic.
//!
//! Verification should use the same predicates exposed by this module.
//!
//! This creates one source of truth for temporal-window semantics.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::fmt;

use crate::quantum::ir::timing::{
    Duration,
    TimePoint,
};

// =============================================================================
// Errors
// =============================================================================

/// Errors produced while constructing or evaluating scheduling windows.
///
/// Window errors are local to the timing-window abstraction. Higher-level
/// scheduling errors may wrap these errors without changing this module's
/// semantics.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WindowError {
    /// The lower bound is later than the upper bound.
    InvalidBounds {
        /// Earliest admissible point.
        earliest: Option<TimePoint>,

        /// Latest admissible point.
        latest: Option<TimePoint>,
    },

    /// Checked temporal arithmetic overflowed the canonical `u128` domain.
    ArithmeticOverflow,

    /// The requested duration cannot be represented from the supplied
    /// start point.
    InvalidPlacement,

    /// A finish window cannot be satisfied by the supplied start and
    /// duration.
    FinishConstraintViolation {
        /// Candidate start.
        start: TimePoint,

        /// Candidate finish.
        finish: TimePoint,
    },

    /// The window itself is valid, but the supplied duration is incompatible
    /// with the complete execution window.
    DurationIncompatible {
        /// Supplied duration.
        duration: Duration,
    },
}

impl fmt::Display for WindowError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidBounds { earliest, latest } => {
                write!(
                    formatter,
                    "invalid scheduling window bounds: earliest={earliest:?}, \
                     latest={latest:?}"
                )
            }

            Self::ArithmeticOverflow => {
                formatter.write_str(
                    "scheduling-window temporal arithmetic overflow",
                )
            }

            Self::InvalidPlacement => {
                formatter.write_str(
                    "candidate scheduling placement cannot be represented",
                )
            }

            Self::FinishConstraintViolation { start, finish } => {
                write!(
                    formatter,
                    "execution window finish constraint violated: \
                     start={start}, finish={finish}"
                )
            }

            Self::DurationIncompatible { duration } => {
                write!(
                    formatter,
                    "duration {duration:?} is incompatible with the \
                     execution window"
                )
            }
        }
    }
}

impl std::error::Error for WindowError {}

/// Result type used by this module.
pub type WindowResult<T> = Result<T, WindowError>;

// =============================================================================
// TimeWindow
// =============================================================================

/// An inclusive admissible point-in-time window.
///
/// The mathematical meaning is:
///
/// ```text
/// earliest <= t <= latest
/// ```
///
/// Either boundary can be absent.
///
/// # Examples
///
/// ```text
/// unbounded
/// (-infinity, +infinity)
///
/// earliest only
/// [10, +infinity)
///
/// latest only
/// (-infinity, 100]
///
/// bounded
/// [10, 100]
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TimeWindow {
    earliest: Option<TimePoint>,
    latest: Option<TimePoint>,
}

impl TimeWindow {
    // =========================================================================
    // Constructors
    // =========================================================================

    /// Creates an unbounded time window.
    ///
    /// This means every representable `TimePoint` is admissible.
    #[must_use]
    pub const fn unbounded() -> Self {
        Self {
            earliest: None,
            latest: None,
        }
    }

    /// Creates a window with no lower bound and a finite upper bound.
    pub const fn at_most(latest: TimePoint) -> Self {
        Self {
            earliest: None,
            latest: Some(latest),
        }
    }

    /// Creates a window with a finite lower bound and no upper bound.
    pub const fn at_least(earliest: TimePoint) -> Self {
        Self {
            earliest: Some(earliest),
            latest: None,
        }
    }

    /// Creates a window containing exactly one time point.
    pub const fn exact(point: TimePoint) -> Self {
        Self {
            earliest: Some(point),
            latest: Some(point),
        }
    }

    /// Creates a bounded inclusive time window.
    ///
    /// Returns an error when `earliest > latest`.
    pub fn new(
        earliest: Option<TimePoint>,
        latest: Option<TimePoint>,
    ) -> WindowResult<Self> {
        if let (Some(earliest), Some(latest)) = (earliest, latest) {
            if earliest > latest {
                return Err(WindowError::InvalidBounds {
                    earliest: Some(earliest),
                    latest: Some(latest),
                });
            }
        }

        Ok(Self {
            earliest,
            latest,
        })
    }

    /// Creates a bounded window from two concrete points.
    pub fn between(
        earliest: TimePoint,
        latest: TimePoint,
    ) -> WindowResult<Self> {
        Self::new(Some(earliest), Some(latest))
    }

    // =========================================================================
    // Accessors
    // =========================================================================

    /// Returns the earliest admissible point.
    #[must_use]
    pub const fn earliest(self) -> Option<TimePoint> {
        self.earliest
    }

    /// Returns the latest admissible point.
    #[must_use]
    pub const fn latest(self) -> Option<TimePoint> {
        self.latest
    }

    /// Returns both bounds.
    #[must_use]
    pub const fn bounds(
        self,
    ) -> (Option<TimePoint>, Option<TimePoint>) {
        (self.earliest, self.latest)
    }

    /// Returns whether the window has a lower bound.
    #[must_use]
    pub const fn has_earliest(self) -> bool {
        self.earliest.is_some()
    }

    /// Returns whether the window has an upper bound.
    #[must_use]
    pub const fn has_latest(self) -> bool {
        self.latest.is_some()
    }

    /// Returns whether the window is completely unbounded.
    #[must_use]
    pub const fn is_unbounded(self) -> bool {
        self.earliest.is_none() && self.latest.is_none()
    }

    /// Returns whether the window represents exactly one point.
    #[must_use]
    pub const fn is_exact(self) -> bool {
        match (self.earliest, self.latest) {
            (Some(earliest), Some(latest)) => earliest == latest,
            _ => false,
        }
    }

    /// Returns whether the window contains a finite range with distinct
    /// endpoints.
    #[must_use]
    pub const fn is_bounded_range(self) -> bool {
        match (self.earliest, self.latest) {
            (Some(earliest), Some(latest)) => earliest < latest,
            _ => false,
        }
    }

    // =========================================================================
    // Membership
    // =========================================================================

    /// Returns whether a point satisfies this window.
    ///
    /// Bounds are inclusive.
    #[must_use]
    pub fn contains(self, point: TimePoint) -> bool {
        if let Some(earliest) = self.earliest {
            if point < earliest {
                return false;
            }
        }

        if let Some(latest) = self.latest {
            if point > latest {
                return false;
            }
        }

        true
    }

    /// Returns whether this window completely contains another window.
    ///
    /// An unbounded side contains every corresponding bounded side.
    #[must_use]
    pub fn contains_window(self, other: Self) -> bool {
        let lower_ok = match (self.earliest, other.earliest) {
            (None, _) => true,
            (Some(_), None) => false,
            (Some(a), Some(b)) => a <= b,
        };

        let upper_ok = match (self.latest, other.latest) {
            (None, _) => true,
            (Some(_), None) => false,
            (Some(a), Some(b)) => a >= b,
        };

        lower_ok && upper_ok
    }

    /// Returns whether two windows share at least one admissible point.
    ///
    /// Because point windows use inclusive boundaries, two windows that meet
    /// at exactly one endpoint intersect.
    #[must_use]
    pub fn intersects(self, other: Self) -> bool {
        let lower = match (self.earliest, other.earliest) {
            (Some(a), Some(b)) => Some(a.max(b)),
            (Some(a), None) | (None, Some(a)) => Some(a),
            (None, None) => None,
        };

        let upper = match (self.latest, other.latest) {
            (Some(a), Some(b)) => Some(a.min(b)),
            (Some(a), None) | (None, Some(a)) => Some(a),
            (None, None) => None,
        };

        match (lower, upper) {
            (Some(lower), Some(upper)) => lower <= upper,
            _ => true,
        }
    }

    // =========================================================================
    // Intersection
    // =========================================================================

    /// Returns the intersection of two windows.
    ///
    /// Returns `None` when no admissible point exists.
    pub fn intersection(self, other: Self) -> Option<Self> {
        let earliest = match (self.earliest, other.earliest) {
            (Some(a), Some(b)) => Some(a.max(b)),
            (Some(a), None) | (None, Some(a)) => Some(a),
            (None, None) => None,
        };

        let latest = match (self.latest, other.latest) {
            (Some(a), Some(b)) => Some(a.min(b)),
            (Some(a), None) | (None, Some(a)) => Some(a),
            (None, None) => None,
        };

        match (earliest, latest) {
            (Some(earliest), Some(latest)) if earliest > latest => None,
            _ => Some(Self {
                earliest,
                latest,
            }),
        }
    }

    // =========================================================================
    // Translation
    // =========================================================================

    /// Shifts the complete window forward by a non-negative duration.
    ///
    /// Unbounded sides remain unbounded.
    pub fn checked_shift_forward(
        self,
        duration: Duration,
    ) -> WindowResult<Self> {
        let earliest = match self.earliest {
            Some(point) => Some(
                point
                    .checked_add_duration(duration)
                    .map_err(|_| WindowError::ArithmeticOverflow)?,
            ),
            None => None,
        };

        let latest = match self.latest {
            Some(point) => Some(
                point
                    .checked_add_duration(duration)
                    .map_err(|_| WindowError::ArithmeticOverflow)?,
            ),
            None => None,
        };

        Ok(Self {
            earliest,
            latest,
        })
    }

    /// Shifts the complete window backward by a non-negative duration.
    ///
    /// The operation fails when a finite boundary would move before the
    /// canonical time origin.
    pub fn checked_shift_backward(
        self,
        duration: Duration,
    ) -> WindowResult<Self> {
        let earliest = match self.earliest {
            Some(point) => Some(
                point
                    .checked_sub_duration(duration)
                    .map_err(|_| WindowError::ArithmeticOverflow)?,
            ),
            None => None,
        };

        let latest = match self.latest {
            Some(point) => Some(
                point
                    .checked_sub_duration(duration)
                    .map_err(|_| WindowError::ArithmeticOverflow)?,
            ),
            None => None,
        };

        Ok(Self {
            earliest,
            latest,
        })
    }

    // =========================================================================
    // Clamping
    // =========================================================================

    /// Clamps a candidate point to this window.
    ///
    /// Returns:
    ///
    /// - the candidate unchanged when it is already valid;
    /// - the earliest boundary when it is too early;
    /// - the latest boundary when it is too late.
    ///
    /// Returns `None` only when the window is unbounded on the side that would
    /// otherwise be required for clamping. In practice, this means an
    /// unbounded window always returns the original point.
    #[must_use]
    pub fn clamp(self, point: TimePoint) -> TimePoint {
        if let Some(earliest) = self.earliest {
            if point < earliest {
                return earliest;
            }
        }

        if let Some(latest) = self.latest {
            if point > latest {
                return latest;
            }
        }

        point
    }

    // =========================================================================
    // Narrowing
    // =========================================================================

    /// Returns a window narrowed by an earliest-point requirement.
    ///
    /// This operation cannot create an invalid window; an impossible
    /// intersection is represented by `None`.
    pub fn with_earliest(self, earliest: TimePoint) -> Option<Self> {
        self.intersection(Self::at_least(earliest))
    }

    /// Returns a window narrowed by a latest-point requirement.
    ///
    /// An impossible intersection returns `None`.
    pub fn with_latest(self, latest: TimePoint) -> Option<Self> {
        self.intersection(Self::at_most(latest))
    }

    /// Returns a window narrowed to a single exact point.
    ///
    /// Returns `None` if the point is outside this window.
    pub fn with_exact(self, point: TimePoint) -> Option<Self> {
        if self.contains(point) {
            Some(Self::exact(point))
        } else {
            None
        }
    }
}

// =============================================================================
// ExecutionWindow
// =============================================================================

/// Complete temporal admissibility constraints for an executable operation.
///
/// It independently constrains:
///
/// - when execution may start;
/// - when execution may finish.
///
/// A duration may subsequently connect the two constraints.
///
/// # Typical interpretations
///
/// ```text
/// release time
///     = earliest start
///
/// deadline
///     = latest finish
///
/// latest start
///     = latest_start
///
/// earliest finish
///     = earliest_finish
/// ```
///
/// The distinction is important because a deadline is a finish constraint,
/// not necessarily a start constraint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExecutionWindow {
    start: TimeWindow,
    finish: TimeWindow,
}

impl ExecutionWindow {
    // =========================================================================
    // Constructors
    // =========================================================================

    /// Creates an execution window from independent start and finish windows.
    #[must_use]
    pub const fn new(
        start: TimeWindow,
        finish: TimeWindow,
    ) -> Self {
        Self { start, finish }
    }

    /// Creates an entirely unconstrained execution window.
    #[must_use]
    pub const fn unbounded() -> Self {
        Self {
            start: TimeWindow::unbounded(),
            finish: TimeWindow::unbounded(),
        }
    }

    /// Creates an execution window constraining only the start point.
    #[must_use]
    pub const fn from_start_window(
        start: TimeWindow,
    ) -> Self {
        Self {
            start,
            finish: TimeWindow::unbounded(),
        }
    }

    /// Creates an execution window constraining only the finish point.
    #[must_use]
    pub const fn from_finish_window(
        finish: TimeWindow,
    ) -> Self {
        Self {
            start: TimeWindow::unbounded(),
            finish,
        }
    }

    /// Creates an execution window with:
    ///
    /// - earliest start;
    /// - latest start;
    /// - earliest finish;
    /// - latest finish.
    ///
    /// Any argument may be `None`.
    pub fn from_bounds(
        earliest_start: Option<TimePoint>,
        latest_start: Option<TimePoint>,
        earliest_finish: Option<TimePoint>,
        latest_finish: Option<TimePoint>,
    ) -> WindowResult<Self> {
        let start = TimeWindow::new(
            earliest_start,
            latest_start,
        )?;

        let finish = TimeWindow::new(
            earliest_finish,
            latest_finish,
        )?;

        Ok(Self { start, finish })
    }

    /// Creates a release/deadline window.
    ///
    /// `release` is the earliest permitted start.
    ///
    /// `deadline` is the latest permitted finish.
    pub fn release_deadline(
        release: Option<TimePoint>,
        deadline: Option<TimePoint>,
    ) -> WindowResult<Self> {
        if let (Some(release), Some(deadline)) = (release, deadline) {
            if release > deadline {
                return Err(WindowError::InvalidBounds {
                    earliest: Some(release),
                    latest: Some(deadline),
                });
            }
        }

        Ok(Self {
            start: TimeWindow::at_least_or_unbounded(release),
            finish: TimeWindow::at_most_or_unbounded(deadline),
        })
    }

    // =========================================================================
    // Accessors
    // =========================================================================

    /// Returns the start-time window.
    #[must_use]
    pub const fn start(self) -> TimeWindow {
        self.start
    }

    /// Returns the finish-time window.
    #[must_use]
    pub const fn finish(self) -> TimeWindow {
        self.finish
    }

    /// Returns whether there is any start constraint.
    #[must_use]
    pub const fn has_start_constraint(self) -> bool {
        !self.start.is_unbounded()
    }

    /// Returns whether there is any finish constraint.
    #[must_use]
    pub const fn has_finish_constraint(self) -> bool {
        !self.finish.is_unbounded()
    }

    /// Returns whether this execution window imposes no temporal constraints.
    #[must_use]
    pub const fn is_unbounded(self) -> bool {
        self.start.is_unbounded() && self.finish.is_unbounded()
    }

    // =========================================================================
    // Candidate validation
    // =========================================================================

    /// Checks a candidate start point without requiring a duration.
    ///
    /// Only the start window can be evaluated in this form.
    #[must_use]
    pub fn allows_start(self, start: TimePoint) -> bool {
        self.start.contains(start)
    }

    /// Checks a candidate finish point without requiring a duration.
    #[must_use]
    pub fn allows_finish(self, finish: TimePoint) -> bool {
        self.finish.contains(finish)
    }

    /// Checks a complete candidate placement.
    ///
    /// The candidate is:
    ///
    /// ```text
    /// [start, start + duration)
    /// ```
    ///
    /// Resource occupancy remains half-open; the finish point itself is not
    /// occupied by the operation.
    pub fn allows_placement(
        self,
        start: TimePoint,
        duration: Duration,
    ) -> WindowResult<bool> {
        let finish = start
            .checked_add_duration(duration)
            .map_err(|_| WindowError::ArithmeticOverflow)?;

        Ok(self.start.contains(start) && self.finish.contains(finish))
    }

    /// Requires a candidate placement to satisfy this window.
    ///
    /// Returns `Ok(())` on success and a structured error otherwise.
    pub fn validate_placement(
        self,
        start: TimePoint,
        duration: Duration,
    ) -> WindowResult<()> {
        let finish = start
            .checked_add_duration(duration)
            .map_err(|_| WindowError::ArithmeticOverflow)?;

        if !self.start.contains(start) {
            return Err(WindowError::InvalidPlacement);
        }

        if !self.finish.contains(finish) {
            return Err(WindowError::FinishConstraintViolation {
                start,
                finish,
            });
        }

        Ok(())
    }

    // =========================================================================
    // Derived start window
    // =========================================================================

    /// Computes the admissible start window after incorporating duration.
    ///
    /// This converts finish constraints into start constraints.
    ///
    /// Mathematically:
    ///
    /// ```text
    /// earliest_finish <= start + duration
    /// ```
    ///
    /// becomes:
    ///
    /// ```text
    /// earliest_finish - duration <= start
    /// ```
    ///
    /// and:
    ///
    /// ```text
    /// start + duration <= latest_finish
    /// ```
    ///
    /// becomes:
    ///
    /// ```text
    /// start <= latest_finish - duration
    /// ```
    ///
    /// The result is intersected with the original start window.
    pub fn start_window_for_duration(
        self,
        duration: Duration,
    ) -> WindowResult<Option<TimeWindow>> {
        let finish_derived = {
            let earliest = match self.finish.earliest() {
                Some(point) => Some(
                    point
                        .checked_sub_duration(duration)
                        .map_err(|_| WindowError::ArithmeticOverflow)?,
                ),
                None => None,
            };

            let latest = match self.finish.latest() {
                Some(point) => Some(
                    point
                        .checked_sub_duration(duration)
                        .map_err(|_| WindowError::ArithmeticOverflow)?,
                ),
                None => None,
            };

            TimeWindow::new(earliest, latest)?
        };

        Ok(self.start.intersection(finish_derived))
    }

    /// Returns the earliest possible start for the supplied duration, if one
    /// exists.
    pub fn earliest_start_for_duration(
        self,
        duration: Duration,
    ) -> WindowResult<Option<TimePoint>> {
        Ok(self
            .start_window_for_duration(duration)?
            .and_then(TimeWindow::earliest))
    }

    /// Returns the latest possible start for the supplied duration, if one
    /// exists.
    pub fn latest_start_for_duration(
        self,
        duration: Duration,
    ) -> WindowResult<Option<TimePoint>> {
        Ok(self
            .start_window_for_duration(duration)?
            .and_then(TimeWindow::latest))
    }

    /// Returns whether the supplied duration can fit into the complete
    /// execution window at some start point.
    pub fn can_fit_duration(
        self,
        duration: Duration,
    ) -> WindowResult<bool> {
        Ok(self.start_window_for_duration(duration)?.is_some())
    }

    // =========================================================================
    // Window intersection
    // =========================================================================

    /// Intersects this execution window with another execution window.
    ///
    /// Both start constraints and finish constraints are intersected
    /// independently.
    ///
    /// Returns `None` if either intersection is impossible.
    pub fn intersection(
        self,
        other: Self,
    ) -> Option<Self> {
        Some(Self {
            start: self.start.intersection(other.start)?,
            finish: self.finish.intersection(other.finish)?,
        })
    }

    /// Returns whether both execution windows have at least one common
    /// start-time point and at least one common finish-time point.
    ///
    /// This does not guarantee that a particular duration can fit between the
    /// two windows. Use `can_fit_duration` for that stronger condition.
    #[must_use]
    pub fn intersects(self, other: Self) -> bool {
        self.start.intersects(other.start)
            && self.finish.intersects(other.finish)
    }
}

// =============================================================================
// Internal constructors used by release_deadline
// =============================================================================
//
// These helpers keep `release_deadline` const-friendly at the semantic level
// without exposing redundant public constructors.

impl TimeWindow {
    const fn at_least_or_unbounded(
        point: Option<TimePoint>,
    ) -> Self {
        Self {
            earliest: point,
            latest: None,
        }
    }

    const fn at_most_or_unbounded(
        point: Option<TimePoint>,
    ) -> Self {
        Self {
            earliest: None,
            latest: point,
        }
    }
}

// =============================================================================
// Free helper functions
// =============================================================================

/// Computes the latest legal start for a known deadline.
///
/// This is equivalent to:
///
/// ```text
/// latest_start = deadline - duration
/// ```
///
/// but uses checked canonical timing arithmetic.
pub fn latest_start_from_deadline(
    deadline: TimePoint,
    duration: Duration,
) -> WindowResult<TimePoint> {
    deadline
        .checked_sub_duration(duration)
        .map_err(|_| WindowError::ArithmeticOverflow)
}

/// Computes the earliest legal finish for a known release/start point.
///
/// This is equivalent to:
///
/// ```text
/// earliest_finish = release + duration
/// ```
pub fn earliest_finish_from_start(
    start: TimePoint,
    duration: Duration,
) -> WindowResult<TimePoint> {
    start
        .checked_add_duration(duration)
        .map_err(|_| WindowError::ArithmeticOverflow)
}

/// Intersects two optional lower bounds.
///
/// This helper is useful to higher-level constraint composition code.
#[must_use]
pub fn max_earliest(
    first: Option<TimePoint>,
    second: Option<TimePoint>,
) -> Option<TimePoint> {
    match (first, second) {
        (Some(a), Some(b)) => Some(a.max(b)),
        (Some(a), None) | (None, Some(a)) => Some(a),
        (None, None) => None,
    }
}

/// Intersects two optional upper bounds.
///
/// This helper is useful to higher-level constraint composition code.
#[must_use]
pub fn min_latest(
    first: Option<TimePoint>,
    second: Option<TimePoint>,
) -> Option<TimePoint> {
    match (first, second) {
        (Some(a), Some(b)) => Some(a.min(b)),
        (Some(a), None) | (None, Some(a)) => Some(a),
        (None, None) => None,
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::quantum::ir::timing::TimePoint;

    fn ns(value: u128) -> TimePoint {
        TimePoint::from_attoseconds(
            value * 1_000_000_000,
        )
    }

    fn duration_ns(value: u128) -> Duration {
        Duration::from_attoseconds(
            value * 1_000_000_000,
        )
    }

    #[test]
    fn unbounded_window_contains_any_representable_point() {
        let window = TimeWindow::unbounded();

        assert!(window.contains(TimePoint::ZERO));
        assert!(window.contains(ns(10)));
    }

    #[test]
    fn exact_window_contains_only_exact_point() {
        let window = TimeWindow::exact(ns(10));

        assert!(!window.contains(ns(9)));
        assert!(window.contains(ns(10)));
        assert!(!window.contains(ns(11)));
    }

    #[test]
    fn bounded_window_is_inclusive() {
        let window = TimeWindow::between(ns(10), ns(20))
            .expect("valid window");

        assert!(window.contains(ns(10)));
        assert!(window.contains(ns(15)));
        assert!(window.contains(ns(20)));
        assert!(!window.contains(ns(9)));
        assert!(!window.contains(ns(21)));
    }

    #[test]
    fn invalid_bounds_are_rejected() {
        let result = TimeWindow::between(ns(20), ns(10));

        assert!(matches!(
            result,
            Err(WindowError::InvalidBounds { .. })
        ));
    }

    #[test]
    fn intersection_uses_max_lower_and_min_upper_bounds() {
        let first = TimeWindow::between(ns(10), ns(30))
            .expect("valid window");

        let second = TimeWindow::between(ns(20), ns(40))
            .expect("valid window");

        let intersection = first
            .intersection(second)
            .expect("windows intersect");

        assert_eq!(intersection.earliest(), Some(ns(20)));
        assert_eq!(intersection.latest(), Some(ns(30)));
    }

    #[test]
    fn adjacent_point_windows_intersect_at_boundary() {
        let first = TimeWindow::between(ns(10), ns(20))
            .expect("valid window");

        let second = TimeWindow::between(ns(20), ns(30))
            .expect("valid window");

        assert!(first.intersects(second));

        let intersection = first
            .intersection(second)
            .expect("boundary point intersects");

        assert_eq!(intersection, TimeWindow::exact(ns(20)));
    }

    #[test]
    fn shifted_window_preserves_width() {
        let window = TimeWindow::between(ns(10), ns(20))
            .expect("valid window");

        let shifted = window
            .checked_shift_forward(duration_ns(5))
            .expect("shift succeeds");

        assert_eq!(shifted.earliest(), Some(ns(15)));
        assert_eq!(shifted.latest(), Some(ns(25)));
    }

    #[test]
    fn start_only_execution_window_checks_start() {
        let execution =
            ExecutionWindow::from_start_window(
                TimeWindow::at_least(ns(10)),
            );

        assert!(execution.allows_start(ns(10)));
        assert!(execution.allows_start(ns(100)));
        assert!(!execution.allows_start(ns(9)));
    }

    #[test]
    fn finish_only_execution_window_checks_finish() {
        let execution =
            ExecutionWindow::from_finish_window(
                TimeWindow::at_most(ns(100)),
            );

        assert!(execution.allows_finish(ns(99)));
        assert!(execution.allows_finish(ns(100)));
        assert!(!execution.allows_finish(ns(101)));
    }

    #[test]
    fn placement_checks_start_and_finish() {
        let execution = ExecutionWindow::from_bounds(
            Some(ns(10)),
            Some(ns(20)),
            Some(ns(20)),
            Some(ns(40)),
        )
        .expect("valid bounds");

        assert!(execution
            .allows_placement(ns(10), duration_ns(10))
            .expect("placement can be evaluated"));

        assert!(execution
            .allows_placement(ns(20), duration_ns(20))
            .expect("placement can be evaluated"));

        assert!(!execution
            .allows_placement(ns(20), duration_ns(21))
            .expect("placement can be evaluated"));
    }

    #[test]
    fn deadline_is_converted_to_latest_start() {
        let execution =
            ExecutionWindow::release_deadline(
                Some(ns(10)),
                Some(ns(100)),
            )
            .expect("valid release/deadline");

        let duration = duration_ns(30);

        let latest = execution
            .latest_start_for_duration(duration)
            .expect("calculation succeeds");

        assert_eq!(latest, Some(ns(70)));
    }

    #[test]
    fn release_and_deadline_can_make_duration_impossible() {
        let execution =
            ExecutionWindow::release_deadline(
                Some(ns(100)),
                Some(ns(110)),
            )
            .expect("valid release/deadline");

        let duration = duration_ns(20);

        assert!(
            !execution
                .can_fit_duration(duration)
                .expect("calculation succeeds")
        );
    }

    #[test]
    fn earliest_finish_is_start_plus_duration() {
        let finish = earliest_finish_from_start(
            ns(10),
            duration_ns(25),
        )
        .expect("addition succeeds");

        assert_eq!(finish, ns(35));
    }

    #[test]
    fn latest_start_is_deadline_minus_duration() {
        let start = latest_start_from_deadline(
            ns(100),
            duration_ns(25),
        )
        .expect("subtraction succeeds");

        assert_eq!(start, ns(75));
    }

    #[test]
    fn zero_duration_can_fit_at_boundary() {
        let execution = ExecutionWindow::release_deadline(
            Some(ns(10)),
            Some(ns(10)),
        )
        .expect("valid window");

        assert!(
            execution
                .can_fit_duration(Duration::ZERO)
                .expect("calculation succeeds")
        );
    }

    #[test]
    fn maximum_time_arithmetic_is_checked() {
        let start = TimePoint::MAX;
        let duration = Duration::from_attoseconds(1);

        let result = earliest_finish_from_start(start, duration);

        assert_eq!(
            result,
            Err(WindowError::ArithmeticOverflow)
        );
    }

    #[test]
    fn negative_result_from_deadline_is_rejected() {
        let deadline = ns(10);
        let duration = duration_ns(20);

        let result =
            latest_start_from_deadline(deadline, duration);

        assert_eq!(
            result,
            Err(WindowError::ArithmeticOverflow)
        );
    }

    #[test]
    fn containment_is_correct_for_unbounded_windows() {
        let unbounded = TimeWindow::unbounded();

        let bounded = TimeWindow::between(ns(10), ns(20))
            .expect("valid window");

        assert!(unbounded.contains_window(bounded));
        assert!(!bounded.contains_window(unbounded));
    }

    #[test]
    fn exact_window_can_be_narrowed() {
        let window = TimeWindow::between(ns(10), ns(20))
            .expect("valid window");

        assert_eq!(
            window.with_exact(ns(15)),
            Some(TimeWindow::exact(ns(15)))
        );

        assert_eq!(
            window.with_exact(ns(25)),
            None
        );
    }

    #[test]
    fn helper_bound_intersections_are_deterministic() {
        assert_eq!(
            max_earliest(Some(ns(10)), Some(ns(20))),
            Some(ns(20))
        );

        assert_eq!(
            min_latest(Some(ns(10)), Some(ns(20))),
            Some(ns(10))
        );

        assert_eq!(
            max_earliest(Some(ns(10)), None),
            Some(ns(10))
        );

        assert_eq!(
            min_latest(None, Some(ns(20))),
            Some(ns(20))
        );
    }
}