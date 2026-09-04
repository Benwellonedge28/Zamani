//! Temporal constraints for the Zamani quantum scheduler.
//!
//! # Responsibility
//!
//! This module defines constraints on when an operation may execute.
//!
//! It deliberately does NOT:
//!
//! - schedule operations;
//! - perform routing;
//! - select physical qubits;
//! - discover hardware;
//! - own hardware timing information;
//! - implement QEC;
//! - implement noise models;
//! - implement resource allocation;
//! - parse source programs;
//! - define a second duration/time representation.
//!
//! Canonical time semantics are owned by `crate::quantum::ir::timing`.
//! Window semantics are owned by `crate::quantum::scheduling::timing::windows`.
//!
//! # Architectural boundary
//!
//! ```text
//! quantum::ir
//!      |
//!      v
//! scheduling::timing
//!      |
//!      +--> windows
//!      |
//!      +--> constraints   <-- this module
//!      |
//!      +--> resolution
//!      |
//!      +--> alignment
//!      |
//!      v
//! planners / algorithms
//! ```
//!
//! Hardware-specific values enter through the scheduling context. This
//! module only represents and evaluates those constraints.
//!
//! # Scalability
//!
//! There are intentionally no limits such as:
//!
//! - maximum number of operations;
//! - maximum number of qubits;
//! - maximum schedule duration;
//! - maximum number of constraints;
//! - maximum parallelism.
//!
//! Collection growth is determined by the caller and available resources.
//!
//! # Numeric safety
//!
//! No floating-point arithmetic is used.
//! All time arithmetic delegates to the canonical IR timing implementation.
//!
//! # Safety
//!
//! This module contains no `unsafe` code.

use core::fmt;

use crate::quantum::ir::timing::{Duration, TimePoint};

use super::windows::TimeWindow;

/// A temporal constraint evaluation error.
///
/// This error is intentionally local to the timing subsystem. Higher-level
/// scheduling errors can wrap this type without requiring this module to
/// depend on the scheduler's global error hierarchy.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum TimingConstraintError {
    /// A minimum duration is greater than its maximum duration.
    InvalidDurationBounds {
        minimum: Duration,
        maximum: Duration,
    },

    /// A placement would require time arithmetic outside the canonical
    /// `TimePoint` representation.
    TimeOverflow,

    /// A constraint set contains mutually incompatible temporal requirements.
    Unsatisfiable,

    /// An operation's actual duration violates its duration constraint.
    DurationViolation {
        actual: Duration,
        minimum: Option<Duration>,
        maximum: Option<Duration>,
    },

    /// An operation starts outside its permitted start window.
    StartWindowViolation {
        start: TimePoint,
    },

    /// An operation finishes outside its permitted finish window.
    FinishWindowViolation {
        finish: TimePoint,
    },
}

impl fmt::Display for TimingConstraintError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidDurationBounds { .. } => {
                formatter.write_str("minimum duration exceeds maximum duration")
            }
            Self::TimeOverflow => {
                formatter.write_str("time arithmetic overflowed the canonical time representation")
            }
            Self::Unsatisfiable => {
                formatter.write_str("temporal constraints are unsatisfiable")
            }
            Self::DurationViolation { .. } => {
                formatter.write_str("operation duration violates the temporal duration constraint")
            }
            Self::StartWindowViolation { .. } => {
                formatter.write_str("operation start time violates the start window")
            }
            Self::FinishWindowViolation { .. } => {
                formatter.write_str("operation finish time violates the finish window")
            }
        }
    }
}

impl std::error::Error for TimingConstraintError {}

/// A lower and/or upper bound on an operation duration.
///
/// The bounds are inclusive.
///
/// An absent lower bound means that no minimum duration is imposed by this
/// constraint. An absent upper bound means that no maximum duration is
/// imposed.
///
/// This type does not impose an artificial duration limit.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct DurationConstraint {
    minimum: Option<Duration>,
    maximum: Option<Duration>,
}

impl DurationConstraint {
    /// Creates a duration constraint.
    ///
    /// # Errors
    ///
    /// Returns [`TimingConstraintError::InvalidDurationBounds`] when both
    /// bounds are present and the minimum exceeds the maximum.
    pub fn new(
        minimum: Option<Duration>,
        maximum: Option<Duration>,
    ) -> Result<Self, TimingConstraintError> {
        if let (Some(minimum), Some(maximum)) = (minimum, maximum) {
            if minimum > maximum {
                return Err(TimingConstraintError::InvalidDurationBounds {
                    minimum,
                    maximum,
                });
            }
        }

        Ok(Self { minimum, maximum })
    }

    /// Creates an unconstrained duration.
    #[must_use]
    pub const fn unbounded() -> Self {
        Self {
            minimum: None,
            maximum: None,
        }
    }

    /// Creates an exact-duration constraint.
    #[must_use]
    pub const fn exact(duration: Duration) -> Self {
        Self {
            minimum: Some(duration),
            maximum: Some(duration),
        }
    }

    /// Creates a minimum-duration constraint.
    #[must_use]
    pub const fn at_least(duration: Duration) -> Self {
        Self {
            minimum: Some(duration),
            maximum: None,
        }
    }

    /// Creates a maximum-duration constraint.
    #[must_use]
    pub const fn at_most(duration: Duration) -> Self {
        Self {
            minimum: None,
            maximum: Some(duration),
        }
    }

    /// Returns the minimum permitted duration.
    #[must_use]
    pub const fn minimum(&self) -> Option<Duration> {
        self.minimum
    }

    /// Returns the maximum permitted duration.
    #[must_use]
    pub const fn maximum(&self) -> Option<Duration> {
        self.maximum
    }

    /// Returns true when this constraint imposes no duration bounds.
    #[must_use]
    pub const fn is_unbounded(&self) -> bool {
        self.minimum.is_none() && self.maximum.is_none()
    }

    /// Returns true when the duration satisfies both bounds.
    #[must_use]
    pub fn contains(&self, duration: Duration) -> bool {
        if let Some(minimum) = self.minimum {
            if duration < minimum {
                return false;
            }
        }

        if let Some(maximum) = self.maximum {
            if duration > maximum {
                return false;
            }
        }

        true
    }

    /// Intersects two duration constraints.
    ///
    /// `None` means that the two constraints have no common duration.
    #[must_use]
    pub fn intersect(&self, other: &Self) -> Option<Self> {
        let minimum = match (self.minimum, other.minimum) {
            (Some(left), Some(right)) => Some(left.max(right)),
            (Some(value), None) | (None, Some(value)) => Some(value),
            (None, None) => None,
        };

        let maximum = match (self.maximum, other.maximum) {
            (Some(left), Some(right)) => Some(left.min(right)),
            (Some(value), None) | (None, Some(value)) => Some(value),
            (None, None) => None,
        };

        match (minimum, maximum) {
            (Some(minimum), Some(maximum)) if minimum > maximum => None,
            _ => Some(Self { minimum, maximum }),
        }
    }
}

impl Default for DurationConstraint {
    fn default() -> Self {
        Self::unbounded()
    }
}

/// A complete temporal constraint for one executable operation.
///
/// The constraint consists of:
///
/// - an allowed start window;
/// - an allowed finish window;
/// - an allowed duration range.
///
/// The scheduler remains responsible for choosing a concrete placement.
/// This type only answers whether a proposed placement is legal and helps
/// planners derive admissible placement ranges.
///
/// No qubit identity is stored here. Qubit-specific constraints belong to
/// `scheduling::constraints::qubit`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct TemporalConstraint {
    start: TimeWindow,
    finish: TimeWindow,
    duration: DurationConstraint,
}

impl TemporalConstraint {
    /// Creates a temporal constraint.
    ///
    /// The start and finish windows are independently validated by
    /// `TimeWindow`. Cross-window feasibility depends on operation duration,
    /// so it is intentionally checked when a concrete duration is supplied.
    #[must_use]
    pub const fn new(
        start: TimeWindow,
        finish: TimeWindow,
        duration: DurationConstraint,
    ) -> Self {
        Self {
            start,
            finish,
            duration,
        }
    }

    /// Creates an unconstrained temporal requirement.
    #[must_use]
    pub const fn unbounded() -> Self {
        Self {
            start: TimeWindow::unbounded(),
            finish: TimeWindow::unbounded(),
            duration: DurationConstraint::unbounded(),
        }
    }

    /// Creates a constraint containing only a start window.
    #[must_use]
    pub const fn from_start_window(start: TimeWindow) -> Self {
        Self {
            start,
            finish: TimeWindow::unbounded(),
            duration: DurationConstraint::unbounded(),
        }
    }

    /// Creates a constraint containing only a finish window.
    #[must_use]
    pub const fn from_finish_window(finish: TimeWindow) -> Self {
        Self {
            start: TimeWindow::unbounded(),
            finish,
            duration: DurationConstraint::unbounded(),
        }
    }

    /// Creates a constraint containing only a duration requirement.
    #[must_use]
    pub const fn from_duration(duration: DurationConstraint) -> Self {
        Self {
            start: TimeWindow::unbounded(),
            finish: TimeWindow::unbounded(),
            duration,
        }
    }

    /// Returns the allowed start window.
    #[must_use]
    pub const fn start_window(&self) -> TimeWindow {
        self.start
    }

    /// Returns the allowed finish window.
    #[must_use]
    pub const fn finish_window(&self) -> TimeWindow {
        self.finish
    }

    /// Returns the duration constraint.
    #[must_use]
    pub const fn duration_constraint(&self) -> DurationConstraint {
        self.duration
    }

    /// Returns true if the temporal constraint contains no restrictions.
    #[must_use]
    pub fn is_unbounded(&self) -> bool {
        self.start.is_unbounded()
            && self.finish.is_unbounded()
            && self.duration.is_unbounded()
    }

    /// Returns true when a concrete operation placement satisfies all
    /// temporal constraints.
    ///
    /// The operation is represented by:
    ///
    /// `start + duration = finish`
    ///
    /// using checked canonical IR arithmetic.
    pub fn allows(
        &self,
        start: TimePoint,
        duration: Duration,
    ) -> Result<(), TimingConstraintError> {
        if !self.duration.contains(duration) {
            return Err(TimingConstraintError::DurationViolation {
                actual: duration,
                minimum: self.duration.minimum(),
                maximum: self.duration.maximum(),
            });
        }

        if !self.start.contains(start) {
            return Err(TimingConstraintError::StartWindowViolation { start });
        }

        let finish = start
            .checked_add_duration(duration)
            .ok_or(TimingConstraintError::TimeOverflow)?;

        if !self.finish.contains(finish) {
            return Err(TimingConstraintError::FinishWindowViolation { finish });
        }

        Ok(())
    }

    /// Returns the finish time for a proposed operation placement.
    pub fn finish_time(
        &self,
        start: TimePoint,
        duration: Duration,
    ) -> Result<TimePoint, TimingConstraintError> {
        start
            .checked_add_duration(duration)
            .ok_or(TimingConstraintError::TimeOverflow)
    }

    /// Intersects this constraint with another constraint.
    ///
    /// The result contains only placements accepted by both constraints.
    ///
    /// `None` means that the constraints have no common solution.
    #[must_use]
    pub fn intersect(&self, other: &Self) -> Option<Self> {
        let start = self.start.intersect(&other.start)?;
        let finish = self.finish.intersect(&other.finish)?;
        let duration = self.duration.intersect(&other.duration)?;

        Some(Self {
            start,
            finish,
            duration,
        })
    }

    /// Determines whether the constraint is satisfiable for a concrete
    /// operation duration.
    ///
    /// This performs an exact temporal feasibility check without selecting a
    /// schedule.
    pub fn is_feasible_for(
        &self,
        duration: Duration,
    ) -> Result<bool, TimingConstraintError> {
        if !self.duration.contains(duration) {
            return Ok(false);
        }

        /*
         * We derive feasible starts from the two independent windows.
         *
         * Start constraints:
         *
         *     start ∈ start_window
         *
         * Finish constraints:
         *
         *     start + duration ∈ finish_window
         *
         * Therefore:
         *
         *     start ∈ finish_window - duration
         *
         * The canonical TimePoint domain is non-negative. If the lower
         * finish bound is smaller than the duration, that lower bound simply
         * does not impose an additional non-negative start restriction.
         *
         * If the latest finish is smaller than the duration, no
         * non-negative start can satisfy it.
         */

        let mut candidate = self.start;

        if let Some(earliest_finish) = self.finish.earliest() {
            if let Some(earliest_start) =
                earliest_finish.checked_sub_duration(duration)
            {
                let shifted = TimeWindow::at_or_after(earliest_start);

                candidate = match candidate.intersect(&shifted) {
                    Some(value) => value,
                    None => return Ok(false),
                };
            }
        }

        if let Some(latest_finish) = self.finish.latest() {
            let latest_start = match latest_finish.checked_sub_duration(duration) {
                Some(value) => value,
                None => return Ok(false),
            };

            let shifted = TimeWindow::at_or_before(latest_start);

            candidate = match candidate.intersect(&shifted) {
                Some(value) => value,
                None => return Ok(false),
            };
        }

        Ok(!candidate.is_empty())
    }

    /// Derives the start-time window that is admissible for a concrete
    /// operation duration.
    ///
    /// This is useful to list schedulers, ASAP/ALAP planners, and
    /// resource-constrained planners because they can calculate the legal
    /// start interval before attempting resource reservation.
    ///
    /// `None` means that no start time can satisfy all temporal constraints.
    pub fn admissible_start_window(
        &self,
        duration: Duration,
    ) -> Result<Option<TimeWindow>, TimingConstraintError> {
        if !self.duration.contains(duration) {
            return Ok(None);
        }

        let mut candidate = self.start;

        if let Some(earliest_finish) = self.finish.earliest() {
            if let Some(earliest_start) =
                earliest_finish.checked_sub_duration(duration)
            {
                candidate = match candidate.intersect(
                    &TimeWindow::at_or_after(earliest_start),
                ) {
                    Some(value) => value,
                    None => return Ok(None),
                };
            }
        }

        if let Some(latest_finish) = self.finish.latest() {
            let latest_start = match latest_finish.checked_sub_duration(duration) {
                Some(value) => value,
                None => return Ok(None),
            };

            candidate = match candidate.intersect(
                &TimeWindow::at_or_before(latest_start),
            ) {
                Some(value) => value,
                None => return Ok(None),
            };
        }

        if candidate.is_empty() {
            Ok(None)
        } else {
            Ok(Some(candidate))
        }
    }
}

impl Default for TemporalConstraint {
    fn default() -> Self {
        Self::unbounded()
    }
}

/// A collection of temporal constraints that can be incrementally combined.
///
/// This is deliberately represented as one accumulated constraint rather
/// than as a vector of individual predicates. That allows planners to
/// normalize compatible constraints before scheduling.
///
/// A future implementation may add a provenance layer outside this type if
/// human-readable explanations of individual constraints are required.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct ConstraintSet {
    temporal: TemporalConstraint,
}

impl ConstraintSet {
    /// Creates an empty constraint set.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            temporal: TemporalConstraint::unbounded(),
        }
    }

    /// Creates a constraint set from one temporal constraint.
    #[must_use]
    pub const fn from_temporal(constraint: TemporalConstraint) -> Self {
        Self {
            temporal: constraint,
        }
    }

    /// Returns the accumulated temporal constraint.
    #[must_use]
    pub const fn temporal(&self) -> TemporalConstraint {
        self.temporal
    }

    /// Adds another temporal constraint by intersection.
    ///
    /// The operation is transactional: when the new constraint would make
    /// the accumulated set unsatisfiable, the original set remains
    /// unchanged and `None` is returned.
    #[must_use]
    pub fn intersect(
        &self,
        constraint: &TemporalConstraint,
    ) -> Option<Self> {
        self.temporal
            .intersect(constraint)
            .map(|temporal| Self { temporal })
    }

    /// Returns true when no constraints have been accumulated.
    #[must_use]
    pub fn is_unbounded(&self) -> bool {
        self.temporal.is_unbounded()
    }

    /// Checks a concrete placement against all accumulated constraints.
    pub fn allows(
        &self,
        start: TimePoint,
        duration: Duration,
    ) -> Result<(), TimingConstraintError> {
        self.temporal.allows(start, duration)
    }

    /// Determines whether at least one placement exists for the supplied
    /// duration.
    pub fn is_feasible_for(
        &self,
        duration: Duration,
    ) -> Result<bool, TimingConstraintError> {
        self.temporal.is_feasible_for(duration)
    }

    /// Calculates the complete admissible start window for the supplied
    /// duration.
    pub fn admissible_start_window(
        &self,
        duration: Duration,
    ) -> Result<Option<TimeWindow>, TimingConstraintError> {
        self.temporal.admissible_start_window(duration)
    }
}

impl Default for ConstraintSet {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn time(attoseconds: u128) -> TimePoint {
        TimePoint::from_attoseconds(attoseconds)
    }

    fn duration(attoseconds: u128) -> Duration {
        Duration::from_attoseconds(attoseconds)
    }

    #[test]
    fn unbounded_duration_accepts_any_duration() {
        let constraint = DurationConstraint::unbounded();

        assert!(constraint.contains(duration(0)));
        assert!(constraint.contains(duration(1)));
        assert!(constraint.contains(duration(u128::MAX)));
    }

    #[test]
    fn exact_duration_accepts_only_exact_value() {
        let constraint = DurationConstraint::exact(duration(10));

        assert!(constraint.contains(duration(10)));
        assert!(!constraint.contains(duration(9)));
        assert!(!constraint.contains(duration(11)));
    }

    #[test]
    fn duration_bounds_are_validated() {
        let result = DurationConstraint::new(
            Some(duration(20)),
            Some(duration(10)),
        );

        assert!(matches!(
            result,
            Err(TimingConstraintError::InvalidDurationBounds { .. })
        ));
    }

    #[test]
    fn duration_constraints_intersect() {
        let first =
            DurationConstraint::new(Some(duration(10)), Some(duration(30)))
                .expect("valid bounds");

        let second =
            DurationConstraint::new(Some(duration(20)), Some(duration(40)))
                .expect("valid bounds");

        let intersection = first
            .intersect(&second)
            .expect("constraints should overlap");

        assert_eq!(intersection.minimum(), Some(duration(20)));
        assert_eq!(intersection.maximum(), Some(duration(30)));
    }

    #[test]
    fn incompatible_duration_constraints_have_no_intersection() {
        let first =
            DurationConstraint::new(Some(duration(0)), Some(duration(10)))
                .expect("valid bounds");

        let second =
            DurationConstraint::new(Some(duration(20)), Some(duration(30)))
                .expect("valid bounds");

        assert!(first.intersect(&second).is_none());
    }

    #[test]
    fn temporal_constraint_accepts_valid_placement() {
        let start = TimeWindow::from(
            Some(time(100)),
            Some(time(200)),
        )
        .expect("valid window");

        let finish = TimeWindow::from(
            Some(time(150)),
            Some(time(300)),
        )
        .expect("valid window");

        let temporal = TemporalConstraint::new(
            start,
            finish,
            DurationConstraint::exact(duration(50)),
        );

        assert!(temporal.allows(time(100), duration(50)).is_ok());
        assert!(temporal.allows(time(200), duration(50)).is_ok());
    }

    #[test]
    fn temporal_constraint_rejects_invalid_start() {
        let start = TimeWindow::from(
            Some(time(100)),
            Some(time(200)),
        )
        .expect("valid window");

        let temporal = TemporalConstraint::from_start_window(start);

        assert!(matches!(
            temporal.allows(time(99), duration(1)),
            Err(TimingConstraintError::StartWindowViolation { .. })
        ));
    }

    #[test]
    fn temporal_constraint_rejects_invalid_finish() {
        let finish = TimeWindow::from(
            Some(time(100)),
            Some(time(200)),
        )
        .expect("valid window");

        let temporal = TemporalConstraint::from_finish_window(finish);

        assert!(matches!(
            temporal.allows(time(50), duration(40)),
            Err(TimingConstraintError::FinishWindowViolation { .. })
        ));
    }

    #[test]
    fn admissible_start_window_combines_start_and_finish_windows() {
        let start = TimeWindow::from(
            Some(time(0)),
            Some(time(200)),
        )
        .expect("valid window");

        let finish = TimeWindow::from(
            Some(time(100)),
            Some(time(300)),
        )
        .expect("valid window");

        let temporal = TemporalConstraint::new(
            start,
            finish,
            DurationConstraint::exact(duration(50)),
        );

        let admissible = temporal
            .admissible_start_window(duration(50))
            .expect("calculation should succeed")
            .expect("there should be a feasible window");

        assert_eq!(admissible.earliest(), Some(time(50)));
        assert_eq!(admissible.latest(), Some(time(200)));
    }

    #[test]
    fn impossible_finish_deadline_is_detected() {
        let finish = TimeWindow::at_or_before(time(10));

        let temporal = TemporalConstraint::from_finish_window(finish);

        let feasible = temporal
            .is_feasible_for(duration(20))
            .expect("calculation should succeed");

        assert!(!feasible);
    }

    #[test]
    fn constraint_sets_intersect_transactionally() {
        let first = TemporalConstraint::from_start_window(
            TimeWindow::at_or_after(time(100)),
        );

        let second = TemporalConstraint::from_start_window(
            TimeWindow::at_or_before(time(200)),
        );

        let set = ConstraintSet::from_temporal(first);

        let combined = set
            .intersect(&second)
            .expect("constraints should overlap");

        assert_eq!(
            combined.temporal().start_window().earliest(),
            Some(time(100))
        );

        assert_eq!(
            combined.temporal().start_window().latest(),
            Some(time(200))
        );
    }

    #[test]
    fn contradictory_windows_have_no_intersection() {
        let first = TemporalConstraint::from_start_window(
            TimeWindow::at_or_after(time(200)),
        );

        let second = TemporalConstraint::from_start_window(
            TimeWindow::at_or_before(time(100)),
        );

        let set = ConstraintSet::from_temporal(first);

        assert!(set.intersect(&second).is_none());
    }

    #[test]
    fn zero_duration_is_supported() {
        let temporal = TemporalConstraint::from_duration(
            DurationConstraint::exact(duration(0)),
        );

        assert!(temporal.allows(time(100), duration(0)).is_ok());
    }
}