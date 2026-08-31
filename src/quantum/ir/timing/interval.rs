//! Zamani Quantum IR — Canonical Temporal Intervals
//!
//! This module defines the canonical semantic representation of a finite
//! temporal interval in the Zamani Quantum Intermediate Representation.
//!
//! # Architectural role
//!
//! `quantum::ir::timing::interval` owns the meaning of:
//!
//! - temporal intervals;
//! - interval boundaries;
//! - interval duration;
//! - interval containment;
//! - interval intersection;
//! - interval adjacency;
//! - interval overlap;
//! - interval ordering;
//! - interval union where semantically valid;
//! - interval shifting;
//! - interval expansion/contraction;
//! - interval splitting;
//! - interval validation;
//! - deterministic interval comparison;
//! - zero-duration interval semantics.
//!
//! It does NOT own:
//!
//! - hardware clocks;
//! - backend `dt` units;
//! - pulse scheduling;
//! - qubit routing;
//! - resource allocation;
//! - hardware topology;
//! - calibration;
//! - optimization policy;
//! - execution;
//! - simulator state;
//! - scheduler policy.
//!
//! Those responsibilities belong to downstream layers.
//!
//! # Canonical semantic model
//!
//! A normal interval is represented as:
//!
//! ```text
//! [start, end)
//! ```
//!
//! where:
//!
//! - `start` is inclusive;
//! - `end` is exclusive;
//! - `start <= end`;
//! - an interval with `start == end` is a valid zero-duration interval.
//!
//! The half-open representation is intentional. It permits adjacent intervals:
//!
//! ```text
//! [0, 10)
//! [10, 20)
//! ```
//!
//! to touch without overlapping.
//!
//! This is particularly useful for:
//!
//! - pulse scheduling;
//! - resource occupancy;
//! - critical-path analysis;
//! - dependency analysis;
//! - parallel execution;
//! - deterministic schedule construction.
//!
//! # Scalability
//!
//! This module contains no machine-size assumptions.
//!
//! An interval does not contain:
//!
//! - a qubit count;
//! - a register size;
//! - a hardware topology;
//! - a fixed scheduler capacity;
//! - a fixed number of operations.
//!
//! Its complexity is constant with respect to the number of qubits or
//! operations in a program.
//!
//! The canonical `TimePoint` and `Duration` types remain responsible for the
//! representable temporal domain.
//!
//! # Dependency boundary
//!
//! This module depends only on canonical timing primitives:
//!
//! ```text
//! timing::TimePoint
//! timing::Duration
//! timing::TimeOffset
//! timing::TimingError
//! ```
//!
//! It intentionally does not import:
//!
//! ```text
//! quantum::ir::qubit
//! ```
//!
//! Timing is applicable to quantum, classical, pulse, analog, logical,
//! distributed, and hybrid operations. A timing interval therefore must not
//! depend on qubit identity.
//!
//! # Integration contract
//!
//! `timing.rs` owns and re-exports the canonical timing primitives.
//!
//! `timing/interval.rs` consumes those primitives and provides
//! `TimeInterval`.
//!
//! Higher-level modules consume `TimeInterval` without needing to know how
//! interval arithmetic is implemented.
//!
//! Typical consumers include:
//!
//! - `scheduling/`;
//! - `hardware/scheduling.rs`;
//! - `pulse/`;
//! - `analysis/`;
//! - `validation/`;
//! - `operation.rs`;
//! - `schedule.rs`;
//! - dependency analysis;
//! - critical-path analysis.
//!
//! No consumer should reimplement interval arithmetic.
//!
//! # Determinism
//!
//! All operations are deterministic.
//!
//! No hash maps, global state, system clocks, floating-point arithmetic, or
//! allocation are required.
//!
//! # Error policy
//!
//! Invalid intervals are rejected explicitly.
//!
//! Arithmetic never intentionally wraps.
//!
//! Overflow is reported through `TimingError::ArithmeticOverflow`.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021.
//!
//! Requirements:
//!
//! - stable Rust only;
//! - no nightly features;
//! - no external dependencies;
//! - no `unsafe`;
//! - no architecture-specific assumptions.
//!
//! `#![forbid(unsafe_code)]` makes the no-unsafe requirement compiler-enforced.
//!
//! # Ownership rule
//!
//! This file owns `TimeInterval` only.
//!
//! It does not redefine:
//!
//! - `Duration`;
//! - `TimePoint`;
//! - `TimeOffset`;
//! - `TimingError`.
//!
//! This prevents duplicate timing primitives from appearing elsewhere in the
//! IR.

#![forbid(unsafe_code)]

use std::cmp::Ordering;
use std::fmt;

use super::{Duration, TimeOffset, TimePoint, TimingError, TimingResult};

// =============================================================================
// TimeInterval
// =============================================================================

/// A canonical half-open semantic time interval.
///
/// The interval represents:
///
/// ```text
/// [start, end)
/// ```
///
/// `start` is inclusive and `end` is exclusive.
///
/// Zero-duration intervals are valid:
///
/// ```text
/// [t, t)
/// ```
///
/// They are useful for semantic events, synchronization points, markers and
/// zero-duration operations.
///
/// # Invariants
///
/// A `TimeInterval` always satisfies:
///
/// ```text
/// start <= end
/// ```
///
/// Construction through [`TimeInterval::new`] validates this invariant.
///
/// The internal fields are private so downstream modules cannot construct an
/// invalid interval directly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TimeInterval {
    start: TimePoint,
    end: TimePoint,
}

impl TimeInterval {
    // =========================================================================
    // Constructors
    // =========================================================================

    /// Creates a validated half-open interval `[start, end)`.
    ///
    /// Returns an error when `start > end`.
    pub fn new(start: TimePoint, end: TimePoint) -> TimingResult<Self> {
        if start > end {
            return Err(TimingError::InvalidInterval {
                start: start.attoseconds(),
                end: end.attoseconds(),
            });
        }

        Ok(Self { start, end })
    }

    /// Creates a zero-duration interval at `point`.
    #[must_use]
    pub const fn at(point: TimePoint) -> Self {
        Self {
            start: point,
            end: point,
        }
    }

    /// Creates an interval from a start point and a duration.
    ///
    /// The end point is calculated using checked arithmetic.
    pub fn from_start_and_duration(
        start: TimePoint,
        duration: Duration,
    ) -> TimingResult<Self> {
        let end = start
            .checked_add_duration(duration)
            .map_err(|_| TimingError::ArithmeticOverflow)?;

        Ok(Self { start, end })
    }

    /// Creates an interval from an end point and a duration.
    ///
    /// The start point is calculated using checked arithmetic.
    pub fn from_end_and_duration(
        end: TimePoint,
        duration: Duration,
    ) -> TimingResult<Self> {
        let start = end
            .checked_sub_duration(duration)
            .map_err(|_| TimingError::ArithmeticOverflow)?;

        Ok(Self { start, end })
    }

    // =========================================================================
    // Accessors
    // =========================================================================

    /// Returns the inclusive start point.
    #[must_use]
    pub const fn start(self) -> TimePoint {
        self.start
    }

    /// Returns the exclusive end point.
    #[must_use]
    pub const fn end(self) -> TimePoint {
        self.end
    }

    /// Returns both interval boundaries.
    #[must_use]
    pub const fn bounds(self) -> (TimePoint, TimePoint) {
        (self.start, self.end)
    }

    /// Returns the interval duration.
    ///
    /// Because the interval invariant guarantees `start <= end`, this
    /// operation cannot produce a negative duration.
    #[must_use]
    pub fn duration(self) -> Duration {
        Duration::from_attoseconds(
            self.end
                .attoseconds()
                .saturating_sub(self.start.attoseconds()),
        )
    }

    /// Returns `true` when the interval has zero duration.
    #[must_use]
    pub fn is_empty(self) -> bool {
        self.start == self.end
    }

    /// Returns `true` when the interval contains a positive amount of time.
    #[must_use]
    pub fn is_non_empty(self) -> bool {
        self.start < self.end
    }

    // =========================================================================
    // Containment
    // =========================================================================

    /// Returns whether `point` belongs to this half-open interval.
    ///
    /// The semantics are:
    ///
    /// ```text
    /// start <= point < end
    /// ```
    ///
    /// Therefore:
    ///
    /// ```text
    /// [0, 10).contains(0)  == true
    /// [0, 10).contains(9)  == true
    /// [0, 10).contains(10) == false
    /// ```
    #[must_use]
    pub fn contains(self, point: TimePoint) -> bool {
        self.start <= point && point < self.end
    }

    /// Returns whether this interval completely contains another interval.
    ///
    /// Empty intervals are treated according to half-open set semantics.
    #[must_use]
    pub fn contains_interval(self, other: Self) -> bool {
        self.start <= other.start && other.end <= self.end
    }

    /// Returns whether another interval is completely contained by this one.
    #[must_use]
    pub fn contains_interval_strict(self, other: Self) -> bool {
        self.start < other.start && other.end < self.end
    }

    // =========================================================================
    // Relationship classification
    // =========================================================================

    /// Classifies the relationship between two intervals.
    #[must_use]
    pub fn relation(self, other: Self) -> IntervalRelation {
        if self == other {
            return IntervalRelation::Equal;
        }

        if self.is_empty() && other.is_empty() {
            return if self.start < other.start {
                IntervalRelation::Before
            } else {
                IntervalRelation::After
            };
        }

        if self.end <= other.start {
            if self.end == other.start {
                return IntervalRelation::Adjacent;
            }

            return IntervalRelation::Before;
        }

        if other.end <= self.start {
            if other.end == self.start {
                return IntervalRelation::Adjacent;
            }

            return IntervalRelation::After;
        }

        if self.contains_interval(other) {
            return IntervalRelation::Contains;
        }

        if other.contains_interval(self) {
            return IntervalRelation::ContainedBy;
        }

        if self.start < other.start && self.end > other.start {
            return IntervalRelation::Overlapping;
        }

        if other.start < self.start && other.end > self.start {
            return IntervalRelation::Overlapping;
        }

        IntervalRelation::Overlapping
    }

    /// Returns whether the two intervals overlap with positive duration.
    ///
    /// Adjacent intervals do not overlap:
    ///
    /// ```text
    /// [0, 10)
    /// [10, 20)
    /// ```
    ///
    /// have no common time point under half-open semantics.
    #[must_use]
    pub fn overlaps(self, other: Self) -> bool {
        self.start < other.end && other.start < self.end
    }

    /// Returns whether the intervals are exactly adjacent.
    ///
    /// Examples:
    ///
    /// ```text
    /// [0, 10) and [10, 20) -> true
    /// [0, 10) and [11, 20) -> false
    /// ```
    #[must_use]
    pub fn is_adjacent(self, other: Self) -> bool {
        self.end == other.start || other.end == self.start
    }

    /// Returns whether the intervals are completely disjoint.
    #[must_use]
    pub fn is_disjoint(self, other: Self) -> bool {
        !self.overlaps(other)
    }

    // =========================================================================
    // Intersection
    // =========================================================================

    /// Returns the positive-duration intersection of two intervals.
    ///
    /// Returns `None` when the intersection has zero duration.
    pub fn intersection(self, other: Self) -> Option<Self> {
        let start = if self.start > other.start {
            self.start
        } else {
            other.start
        };

        let end = if self.end < other.end {
            self.end
        } else {
            other.end
        };

        if start < end {
            Some(Self { start, end })
        } else {
            None
        }
    }

    /// Returns the semantic intersection, including zero-duration contact.
    ///
    /// This is useful for schedulers that need to distinguish:
    ///
    /// - no relationship;
    /// - touching boundaries;
    /// - positive-duration overlap.
    pub fn intersection_inclusive(self, other: Self) -> Option<Self> {
        let start = if self.start > other.start {
            self.start
        } else {
            other.start
        };

        let end = if self.end < other.end {
            self.end
        } else {
            other.end
        };

        if start <= end {
            Some(Self { start, end })
        } else {
            None
        }
    }

    // =========================================================================
    // Union
    // =========================================================================

    /// Returns the union of two intervals when their union is one contiguous
    /// interval.
    ///
    /// Intervals may:
    ///
    /// - overlap;
    /// - contain one another;
    /// - be adjacent.
    ///
    /// Disconnected intervals return `None`.
    pub fn union(self, other: Self) -> Option<Self> {
        if self.is_disjoint(other) && !self.is_adjacent(other) {
            return None;
        }

        let start = if self.start < other.start {
            self.start
        } else {
            other.start
        };

        let end = if self.end > other.end {
            self.end
        } else {
            other.end
        };

        Some(Self { start, end })
    }

    // =========================================================================
    // Shifting
    // =========================================================================

    /// Shifts the entire interval by a signed temporal offset.
    ///
    /// Both boundaries are shifted by exactly the same amount.
    ///
    /// The operation is checked and never wraps.
    pub fn checked_shift(self, offset: TimeOffset) -> TimingResult<Self> {
        let start = self
            .start
            .checked_add_offset(offset)
            .map_err(|_| TimingError::ArithmeticOverflow)?;

        let end = self
            .end
            .checked_add_offset(offset)
            .map_err(|_| TimingError::ArithmeticOverflow)?;

        Self::new(start, end)
    }

    /// Shifts the interval forward by a non-negative duration.
    pub fn checked_shift_forward(self, duration: Duration) -> TimingResult<Self> {
        self.checked_shift(TimeOffset::from_attoseconds(duration.attoseconds()))
    }

    /// Shifts the interval backward by a non-negative duration.
    pub fn checked_shift_backward(self, duration: Duration) -> TimingResult<Self> {
        self.checked_shift(TimeOffset::from_attoseconds(
            duration
                .attoseconds()
                .checked_neg()
                .ok_or(TimingError::ArithmeticOverflow)?,
        ))
    }

    // =========================================================================
    // Expansion and contraction
    // =========================================================================

    /// Expands the interval on both sides by the given duration.
    ///
    /// The resulting interval is:
    ///
    /// ```text
    /// [start - amount, end + amount)
    /// ```
    pub fn checked_expand(self, amount: Duration) -> TimingResult<Self> {
        let start = self
            .start
            .checked_sub_duration(amount)
            .map_err(|_| TimingError::ArithmeticOverflow)?;

        let end = self
            .end
            .checked_add_duration(amount)
            .map_err(|_| TimingError::ArithmeticOverflow)?;

        Self::new(start, end)
    }

    /// Contracts the interval on both sides by the given duration.
    ///
    /// Returns an error if contraction would invert the interval.
    pub fn checked_contract(self, amount: Duration) -> TimingResult<Self> {
        let total = amount
            .attoseconds()
            .checked_mul(2)
            .ok_or(TimingError::ArithmeticOverflow)?;

        if total > self.duration().attoseconds() {
            return Err(TimingError::InvalidInterval {
                start: self.start.attoseconds(),
                end: self.end.attoseconds(),
            });
        }

        let start = self
            .start
            .checked_add_duration(amount)
            .map_err(|_| TimingError::ArithmeticOverflow)?;

        let end = self
            .end
            .checked_sub_duration(amount)
            .map_err(|_| TimingError::ArithmeticOverflow)?;

        Self::new(start, end)
    }

    // =========================================================================
    // Relative positioning
    // =========================================================================

    /// Returns the temporal distance from this interval's end to another
    /// interval's start when the other interval follows this one.
    ///
    /// Returns `None` if the intervals overlap or touch.
    pub fn gap_to(self, other: Self) -> Option<Duration> {
        if self.end < other.start {
            return Some(Duration::from_attoseconds(
                other.start.attoseconds() - self.end.attoseconds(),
            ));
        }

        if other.end < self.start {
            return Some(Duration::from_attoseconds(
                self.start.attoseconds() - other.end.attoseconds(),
            ));
        }

        None
    }

    /// Returns the temporal offset from this interval's start to another
    /// interval's start.
    ///
    /// Positive values mean `other` starts later.
    pub fn offset_to(self, other: Self) -> TimingResult<TimeOffset> {
        TimeOffset::between(self.start, other.start)
    }

    // =========================================================================
    // Splitting
    // =========================================================================

    /// Splits the interval at an absolute time point.
    ///
    /// The split point must lie within the closed boundary range:
    ///
    /// ```text
    /// start <= split <= end
    /// ```
    ///
    /// The returned intervals are:
    ///
    /// ```text
    /// [start, split)
    /// [split, end)
    /// ```
    pub fn split_at(self, split: TimePoint) -> TimingResult<(Self, Self)> {
        if split < self.start || split > self.end {
            return Err(TimingError::InvalidInterval {
                start: self.start.attoseconds(),
                end: self.end.attoseconds(),
            });
        }

        Ok((
            Self {
                start: self.start,
                end: split,
            },
            Self {
                start: split,
                end: self.end,
            },
        ))
    }

    /// Splits the interval after an elapsed duration from its start.
    pub fn split_after(self, offset: Duration) -> TimingResult<(Self, Self)> {
        if offset.attoseconds() > self.duration().attoseconds() {
            return Err(TimingError::InvalidInterval {
                start: self.start.attoseconds(),
                end: self.end.attoseconds(),
            });
        }

        let split = self
            .start
            .checked_add_duration(offset)
            .map_err(|_| TimingError::ArithmeticOverflow)?;

        self.split_at(split)
    }

    // =========================================================================
    // Comparison
    // =========================================================================

    /// Compares intervals by start time and then end time.
    ///
    /// This ordering is deterministic and suitable for ordered collections.
    #[must_use]
    pub fn compare_by_bounds(self, other: Self) -> Ordering {
        self.start
            .cmp(&other.start)
            .then_with(|| self.end.cmp(&other.end))
    }

    /// Returns the earlier of two intervals according to canonical boundary
    /// ordering.
    #[must_use]
    pub fn min_by_bounds(self, other: Self) -> Self {
        if self.compare_by_bounds(other) == Ordering::Greater {
            other
        } else {
            self
        }
    }

    /// Returns the later of two intervals according to canonical boundary
    /// ordering.
    #[must_use]
    pub fn max_by_bounds(self, other: Self) -> Self {
        if self.compare_by_bounds(other) == Ordering::Less {
            other
        } else {
            self
        }
    }

    // =========================================================================
    // Validation
    // =========================================================================

    /// Validates the interval's structural invariants.
    ///
    /// This method is intentionally cheap and allocation-free so validators,
    /// schedulers and deserializers can invoke it repeatedly.
    pub fn validate(self) -> TimingResult<()> {
        if self.start > self.end {
            return Err(TimingError::InvalidInterval {
                start: self.start.attoseconds(),
                end: self.end.attoseconds(),
            });
        }

        Ok(())
    }
}

impl Ord for TimeInterval {
    fn cmp(&self, other: &Self) -> Ordering {
        self.start
            .cmp(&other.start)
            .then_with(|| self.end.cmp(&other.end))
    }
}

impl PartialOrd for TimeInterval {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for TimeInterval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {})", self.start, self.end)
    }
}

// =============================================================================
// IntervalRelation
// =============================================================================

/// Semantic relationship between two canonical half-open intervals.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IntervalRelation {
    /// The intervals are exactly equal.
    Equal,

    /// This interval occurs completely before the other.
    Before,

    /// This interval occurs completely after the other.
    After,

    /// The intervals touch at exactly one boundary.
    Adjacent,

    /// The intervals overlap with positive duration without containing one
    /// another.
    Overlapping,

    /// This interval completely contains the other.
    Contains,

    /// This interval is completely contained by the other.
    ContainedBy,
}

impl IntervalRelation {
    /// Returns `true` if the relationship represents temporal overlap.
    #[must_use]
    pub const fn overlaps(self) -> bool {
        matches!(
            self,
            Self::Equal
                | Self::Overlapping
                | Self::Contains
                | Self::ContainedBy
        )
    }

    /// Returns `true` if the relationship represents temporal adjacency.
    #[must_use]
    pub const fn is_adjacent(self) -> bool {
        matches!(self, Self::Adjacent)
    }

    /// Returns `true` if the relationship represents strict ordering.
    #[must_use]
    pub const fn is_ordered(self) -> bool {
        matches!(self, Self::Before | Self::After)
    }

    /// Returns `true` if the relationship is disjoint.
    #[must_use]
    pub const fn is_disjoint(self) -> bool {
        matches!(self, Self::Before | Self::After | Self::Adjacent)
    }
}

impl fmt::Display for IntervalRelation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            Self::Equal => "equal",
            Self::Before => "before",
            Self::After => "after",
            Self::Adjacent => "adjacent",
            Self::Overlapping => "overlapping",
            Self::Contains => "contains",
            Self::ContainedBy => "contained-by",
        };

        f.write_str(text)
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn point(value: u128) -> TimePoint {
        TimePoint::from_attoseconds(value)
    }

    fn duration(value: u128) -> Duration {
        Duration::from_attoseconds(value)
    }

    fn interval(start: u128, end: u128) -> TimeInterval {
        TimeInterval::new(point(start), point(end))
            .expect("test interval must be valid")
    }

    #[test]
    fn creates_valid_interval() {
        let value = interval(10, 20);

        assert_eq!(value.start(), point(10));
        assert_eq!(value.end(), point(20));
        assert_eq!(value.duration(), duration(10));
    }

    #[test]
    fn accepts_zero_duration_interval() {
        let value = TimeInterval::at(point(10));

        assert!(value.is_empty());
        assert!(!value.is_non_empty());
        assert_eq!(value.duration(), Duration::ZERO);
    }

    #[test]
    fn rejects_inverted_interval() {
        let result = TimeInterval::new(point(20), point(10));

        assert!(matches!(
            result,
            Err(TimingError::InvalidInterval {
                start: 20,
                end: 10
            })
        ));
    }

    #[test]
    fn half_open_contains_semantics_are_correct() {
        let value = interval(10, 20);

        assert!(value.contains(point(10)));
        assert!(value.contains(point(19)));
        assert!(!value.contains(point(20)));
        assert!(!value.contains(point(9)));
    }

    #[test]
    fn detects_overlap() {
        let left = interval(10, 20);
        let right = interval(15, 25);

        assert!(left.overlaps(right));
        assert_eq!(left.relation(right), IntervalRelation::Overlapping);
    }

    #[test]
    fn detects_adjacency() {
        let left = interval(10, 20);
        let right = interval(20, 30);

        assert!(!left.overlaps(right));
        assert!(left.is_adjacent(right));
        assert_eq!(left.relation(right), IntervalRelation::Adjacent);
    }

    #[test]
    fn detects_before() {
        let left = interval(10, 20);
        let right = interval(30, 40);

        assert_eq!(left.relation(right), IntervalRelation::Before);
        assert_eq!(right.relation(left), IntervalRelation::After);
        assert!(left.gap_to(right).is_some());
        assert_eq!(
            left.gap_to(right)
                .expect("gap should exist")
                .attoseconds(),
            10
        );
    }

    #[test]
    fn detects_containment() {
        let outer = interval(10, 40);
        let inner = interval(20, 30);

        assert_eq!(outer.relation(inner), IntervalRelation::Contains);
        assert_eq!(inner.relation(outer), IntervalRelation::ContainedBy);
        assert!(outer.contains_interval(inner));
        assert!(!outer.contains_interval_strict(outer));
    }

    #[test]
    fn computes_intersection() {
        let left = interval(10, 30);
        let right = interval(20, 40);

        let intersection = left
            .intersection(right)
            .expect("positive intersection should exist");

        assert_eq!(intersection, interval(20, 30));
    }

    #[test]
    fn adjacent_intervals_have_no_positive_intersection() {
        let left = interval(10, 20);
        let right = interval(20, 30);

        assert_eq!(left.intersection(right), None);

        assert_eq!(
            left.intersection_inclusive(right),
            Some(TimeInterval::at(point(20)))
        );
    }

    #[test]
    fn computes_union_for_overlapping_intervals() {
        let left = interval(10, 30);
        let right = interval(20, 40);

        assert_eq!(left.union(right), Some(interval(10, 40)));
    }

    #[test]
    fn computes_union_for_adjacent_intervals() {
        let left = interval(10, 20);
        let right = interval(20, 30);

        assert_eq!(left.union(right), Some(interval(10, 30)));
    }

    #[test]
    fn rejects_union_for_disconnected_intervals() {
        let left = interval(10, 20);
        let right = interval(30, 40);

        assert_eq!(left.union(right), None);
    }

    #[test]
    fn shifts_forward() {
        let value = interval(10, 20);

        let shifted = value
            .checked_shift_forward(duration(5))
            .expect("shift should succeed");

        assert_eq!(shifted, interval(15, 25));
    }

    #[test]
    fn shifts_backward() {
        let value = interval(10, 20);

        let shifted = value
            .checked_shift_backward(duration(5))
            .expect("shift should succeed");

        assert_eq!(shifted, interval(5, 15));
    }

    #[test]
    fn expands_interval() {
        let value = interval(10, 20);

        let expanded = value
            .checked_expand(duration(5))
            .expect("expansion should succeed");

        assert_eq!(expanded, interval(5, 25));
    }

    #[test]
    fn contracts_interval() {
        let value = interval(10, 30);

        let contracted = value
            .checked_contract(duration(5))
            .expect("contraction should succeed");

        assert_eq!(contracted, interval(15, 25));
    }

    #[test]
    fn rejects_excessive_contraction() {
        let value = interval(10, 20);

        let result = value.checked_contract(duration(6));

        assert!(result.is_err());
    }

    #[test]
    fn splits_at_boundary() {
        let value = interval(10, 30);

        let (left, right) = value
            .split_at(point(20))
            .expect("split should succeed");

        assert_eq!(left, interval(10, 20));
        assert_eq!(right, interval(20, 30));
    }

    #[test]
    fn split_at_start_is_valid() {
        let value = interval(10, 30);

        let (left, right) = value
            .split_at(point(10))
            .expect("split should succeed");

        assert_eq!(left, TimeInterval::at(point(10)));
        assert_eq!(right, interval(10, 30));
    }

    #[test]
    fn split_at_end_is_valid() {
        let value = interval(10, 30);

        let (left, right) = value
            .split_at(point(30))
            .expect("split should succeed");

        assert_eq!(left, interval(10, 30));
        assert_eq!(right, TimeInterval::at(point(30)));
    }

    #[test]
    fn split_outside_interval_is_rejected() {
        let value = interval(10, 30);

        assert!(value.split_at(point(9)).is_err());
        assert!(value.split_at(point(31)).is_err());
    }

    #[test]
    fn deterministic_ordering_is_by_start_then_end() {
        let first = interval(10, 20);
        let second = interval(10, 30);
        let third = interval(20, 30);

        assert!(first < second);
        assert!(second < third);
    }

    #[test]
    fn validates_successfully() {
        let value = interval(10, 20);

        assert!(value.validate().is_ok());
    }

    #[test]
    fn displays_as_half_open_interval() {
        let value = interval(10, 20);

        assert_eq!(value.to_string(), "[10as, 20as)");
    }

    #[test]
    fn interval_relation_is_symmetric_where_expected() {
        let left = interval(10, 20);
        let right = interval(15, 25);

        assert_eq!(left.relation(right), IntervalRelation::Overlapping);
        assert_eq!(right.relation(left), IntervalRelation::Overlapping);
    }

    #[test]
    fn identical_intervals_are_equal() {
        let left = interval(10, 20);
        let right = interval(10, 20);

        assert_eq!(left.relation(right), IntervalRelation::Equal);
    }

    #[test]
    fn empty_intervals_do_not_overlap_positive_duration_intervals_at_boundary() {
        let empty = TimeInterval::at(point(20));
        let value = interval(20, 30);

        assert!(!empty.overlaps(value));
        assert_eq!(empty.relation(value), IntervalRelation::Adjacent);
    }

    #[test]
    fn gap_is_zero_for_touching_intervals() {
        let left = interval(10, 20);
        let right = interval(20, 30);

        assert_eq!(left.gap_to(right), None);
    }

    #[test]
    fn interval_construction_from_duration() {
        let value = TimeInterval::from_start_and_duration(
            point(10),
            duration(20),
        )
        .expect("construction should succeed");

        assert_eq!(value, interval(10, 30));
    }

    #[test]
    fn interval_construction_from_end_and_duration() {
        let value =
            TimeInterval::from_end_and_duration(point(30), duration(20))
                .expect("construction should succeed");

        assert_eq!(value, interval(10, 30));
    }
}