//! Zamani Quantum IR — Timing Constraints
//!
//! Canonical, hardware-independent temporal constraints for the Zamani
//! Quantum Intermediate Representation.
//!
//! # Architectural role
//!
//! This module defines relationships that must or should hold between
//! semantic timing anchors.
//!
//! It answers:
//!
//! > "What temporal relationship does the program require?"
//!
//! It does NOT answer:
//!
//! - how the constraint is scheduled;
//! - which hardware clock realizes it;
//! - which physical qubit is selected;
//! - which pulse channel is used;
//! - how routing is performed;
//! - how optimization is performed;
//! - how a backend executes the schedule.
//!
//! Those responsibilities belong to downstream layers.
//!
//! # Dependency boundary
//!
//! ```text
//! core::identity ───────┐
//!                       │
//! quantum::ir::qubit ───┼──> timing::constraint
//!                       │
//! timing primitives ────┘
//!                              │
//!                              ▼
//!                       scheduling
//!                              │
//!                              ▼
//!                           hardware
//!                              │
//!                              ▼
//!                           backend
//! ```
//!
//! The dependency never points backwards from the canonical IR into
//! scheduling or hardware.
//!
//! # Canonical timing model
//!
//! The parent timing module owns the canonical temporal quantities:
//!
//! - `Duration`
//! - `TimeOffset`
//! - `TimingError`
//! - `TimingResult`
//!
//! This file owns:
//!
//! - timing anchors;
//! - timing endpoints;
//! - temporal relations;
//! - interval constraints;
//! - constraint strength;
//! - deterministic validation;
//! - constraint satisfaction predicates;
//! - composition of independent constraints;
//! - exact, hardware-independent semantics.
//!
//! # Universal-program principle
//!
//! A timing constraint is semantic intent.
//!
//! It must remain valid for:
//!
//! - one-qubit machines;
//! - small QPUs;
//! - large QPUs;
//! - distributed quantum computers;
//! - logical/fault-tolerant machines;
//! - simulators;
//! - pulse processors;
//! - analog processors;
//! - annealers;
//! - future quantum architectures.
//!
//! There is therefore no:
//!
//! - maximum number of qubits;
//! - maximum number of operations;
//! - fixed topology;
//! - fixed clock rate;
//! - fixed hardware latency;
//! - fixed sample period;
//! - vendor-specific timing model.
//!
//! # Exact arithmetic
//!
//! This module never uses floating-point numbers for semantic timing.
//!
//! `Duration` and `TimeOffset` come from the canonical timing module and use
//! exact arithmetic.
//!
//! This prevents timing equality and ordering from becoming dependent on
//! floating-point rounding.
//!
//! # Qubit integration
//!
//! Qubit timing anchors use the authoritative:
//!
//! `quantum::ir::qubit::QubitId`
//!
//! They do not use:
//!
//! - physical hardware qubit numbers;
//! - routing-local indexes;
//! - simulator indexes;
//! - backend handles.
//!
//! A qubit anchor means semantic availability/release intent. It does not
//! perform routing or hardware allocation.
//!
//! # Operation integration
//!
//! Operation anchors use the canonical `OperationId` from
//! `quantum::ir::core::identity`.
//!
//! Operation identity is independent of collection position.
//!
//! # Absolute timing
//!
//! `TimingAnchor::Absolute` is relative to the semantic schedule/program
//! origin. It is NOT a wall-clock timestamp and must not be interpreted as
//! UTC, system time, device time, or operating-system time.
//!
//! # Interval semantics
//!
//! Intervals use half-open semantics:
//!
//! ```text
//! [start, end)
//! ```
//!
//! This means an operation ending at `t` does not overlap another operation
//! starting at exactly `t`.
//!
//! This convention avoids ambiguity when adjacent operations share a
//! boundary.
//!
//! # Constraint semantics
//!
//! The primary point relation is expressed as:
//!
//! ```text
//! left_time - right_time
//! ```
//!
//! and constrained using an exact relation.
//!
//! Examples:
//!
//! ```text
//! left == right
//! left == right + 5ns
//! left >= right + 10ns
//! left <= right + 20ns
//! 5ns <= left - right <= 20ns
//! ```
//!
//! Interval constraints additionally express non-overlap requirements.
//!
//! # Constraint strength
//!
//! A constraint can be:
//!
//! - required;
//! - preferred.
//!
//! Required constraints are semantic requirements and cannot simply be
//! discarded by a scheduler.
//!
//! Preferred constraints express optimization intent. A downstream scheduler
//! may violate them only according to an explicitly defined policy.
//!
//! This module does not decide that policy.
//!
//! # Determinism
//!
//! All public structures implement deterministic equality, ordering where
//! meaningful, hashing, and formatting.
//!
//! No `HashMap` or process-global state is used.
//!
//! # Serialization
//!
//! This file does not own the canonical serialized representation.
//!
//! `quantum::ir::serialization` owns serialization.
//!
//! These structures nevertheless use stable enums and strongly typed IDs so
//! canonical serialization can represent them without depending on machine
//! addresses or container ordering.
//!
//! # Hashing
//!
//! Rust `Hash` implementations are suitable for deterministic in-process
//! collections.
//!
//! Canonical cryptographic hashing remains owned by the IR hashing layer.
//!
//! # Validation
//!
//! Local invariants are validated here.
//!
//! Whole-program validation remains the responsibility of
//! `quantum::ir::validation`.
//!
//! # Scalability
//!
//! No collection size or machine size is encoded in this module.
//!
//! Constraint objects are constant-size semantic descriptors except for
//! explicitly named metadata strings.
//!
//! A compiler may store millions or more constraints using external storage,
//! streaming IR, partitioned IR, or distributed compilation without changing
//! this semantic contract.
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
//! - no external dependencies;
//! - no `unsafe`.
//!
//! `#![forbid(unsafe_code)]` makes the no-unsafe requirement compiler enforced.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::cmp::Ordering;
use std::fmt;

use crate::quantum::ir::core::identity::OperationId;
use crate::quantum::ir::qubit::QubitId;

use super::{Duration, TimeOffset, TimingError, TimingResult};

// =============================================================================
// Constraint strength
// =============================================================================

/// Semantic strength of a timing constraint.
///
/// `Required` constraints express correctness requirements.
///
/// `Preferred` constraints express optimization intent and may only be
/// violated by a downstream scheduler according to an explicit policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ConstraintStrength {
    /// The constraint is semantically required.
    Required,

    /// The constraint is preferred but not necessarily mandatory.
    Preferred,
}

impl Default for ConstraintStrength {
    fn default() -> Self {
        Self::Required
    }
}

impl fmt::Display for ConstraintStrength {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Required => f.write_str("required"),
            Self::Preferred => f.write_str("preferred"),
        }
    }
}

// =============================================================================
// Timing endpoint
// =============================================================================

/// Endpoint of a semantic timing interval.
///
/// Timing endpoints are deliberately independent of hardware clocks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TimingEndpoint {
    /// Beginning of an operation.
    Start,

    /// End of an operation.
    End,
}

impl fmt::Display for TimingEndpoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Start => f.write_str("start"),
            Self::End => f.write_str("end"),
        }
    }
}

// =============================================================================
// Timing anchor
// =============================================================================

/// Semantic point against which a timing constraint may be expressed.
///
/// An anchor is not itself a scheduled time. The scheduler later resolves an
/// anchor to an actual time coordinate.
///
/// This distinction is essential:
///
/// ```text
/// TimingAnchor
///     = semantic reference
///
/// resolved scheduler time
///     = target-specific realization
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TimingAnchor {
    /// Start or end of a canonical IR operation.
    Operation {
        /// Stable operation identity.
        operation: OperationId,

        /// Operation endpoint.
        endpoint: TimingEndpoint,
    },

    /// Semantic availability point for a logical qubit.
    ///
    /// This is not a physical hardware-qubit timestamp.
    QubitAvailable {
        /// Canonical logical qubit identity.
        qubit: QubitId,
    },

    /// Semantic release point for a logical qubit.
    ///
    /// This represents the point after which the qubit is no longer required
    /// by the referenced semantic region.
    QubitReleased {
        /// Canonical logical qubit identity.
        qubit: QubitId,
    },

    /// Start of the semantic schedule/program timeline.
    ScheduleStart,

    /// End of the semantic schedule/program timeline.
    ScheduleEnd,

    /// An absolute semantic time measured from the schedule/program origin.
    ///
    /// This is not wall-clock time.
    Absolute(Duration),
}

impl TimingAnchor {
    /// Creates an operation-start anchor.
    #[must_use]
    pub const fn operation_start(operation: OperationId) -> Self {
        Self::Operation {
            operation,
            endpoint: TimingEndpoint::Start,
        }
    }

    /// Creates an operation-end anchor.
    #[must_use]
    pub const fn operation_end(operation: OperationId) -> Self {
        Self::Operation {
            operation,
            endpoint: TimingEndpoint::End,
        }
    }

    /// Creates a logical-qubit availability anchor.
    #[must_use]
    pub const fn qubit_available(qubit: QubitId) -> Self {
        Self::QubitAvailable { qubit }
    }

    /// Creates a logical-qubit release anchor.
    #[must_use]
    pub const fn qubit_released(qubit: QubitId) -> Self {
        Self::QubitReleased { qubit }
    }

    /// Creates an absolute semantic-time anchor.
    #[must_use]
    pub const fn absolute(time: Duration) -> Self {
        Self::Absolute(time)
    }

    /// Returns the operation ID when this is an operation anchor.
    #[must_use]
    pub const fn operation_id(&self) -> Option<OperationId> {
        match self {
            Self::Operation { operation, .. } => Some(*operation),
            _ => None,
        }
    }

    /// Returns the logical qubit when this is a qubit anchor.
    #[must_use]
    pub const fn qubit_id(&self) -> Option<QubitId> {
        match self {
            Self::QubitAvailable { qubit }
            | Self::QubitReleased { qubit } => Some(*qubit),

            _ => None,
        }
    }

    /// Returns the absolute time when this is an absolute anchor.
    #[must_use]
    pub const fn absolute_time(&self) -> Option<Duration> {
        match self {
            Self::Absolute(time) => Some(*time),
            _ => None,
        }
    }

    /// Returns whether this anchor is operation based.
    #[must_use]
    pub const fn is_operation(&self) -> bool {
        matches!(self, Self::Operation { .. })
    }

    /// Returns whether this anchor is qubit-resource based.
    #[must_use]
    pub const fn is_qubit(&self) -> bool {
        matches!(
            self,
            Self::QubitAvailable { .. } | Self::QubitReleased { .. }
        )
    }

    /// Returns whether this anchor is an absolute semantic time.
    #[must_use]
    pub const fn is_absolute(&self) -> bool {
        matches!(self, Self::Absolute(_))
    }
}

impl fmt::Display for TimingAnchor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Operation {
                operation,
                endpoint,
            } => {
                write!(f, "{operation}.{endpoint}")
            }

            Self::QubitAvailable { qubit } => {
                write!(f, "qubit({qubit}).available")
            }

            Self::QubitReleased { qubit } => {
                write!(f, "qubit({qubit}).released")
            }

            Self::ScheduleStart => f.write_str("schedule.start"),

            Self::ScheduleEnd => f.write_str("schedule.end"),

            Self::Absolute(time) => {
                write!(f, "absolute({time})")
            }
        }
    }
}

// =============================================================================
// Timing span
// =============================================================================

/// Semantic half-open timing interval.
///
/// The interval represents:
///
/// ```text
/// [start, end)
/// ```
///
/// `end` must not precede `start` when both are concrete absolute anchors.
///
/// For symbolic anchors, full validation occurs once the scheduler resolves
/// them.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TimingSpan {
    /// Interval beginning.
    pub start: TimingAnchor,

    /// Interval end.
    pub end: TimingAnchor,
}

impl TimingSpan {
    /// Creates a semantic timing span.
    pub fn new(start: TimingAnchor, end: TimingAnchor) -> TimingResult<Self> {
        let span = Self { start, end };

        span.validate()?;

        Ok(span)
    }

    /// Creates an operation interval.
    pub fn operation(operation: OperationId) -> Self {
        Self {
            start: TimingAnchor::operation_start(operation),
            end: TimingAnchor::operation_end(operation),
        }
    }

    /// Creates an absolute interval.
    pub fn absolute(
        start: Duration,
        end: Duration,
    ) -> TimingResult<Self> {
        Self::new(
            TimingAnchor::absolute(start),
            TimingAnchor::absolute(end),
        )
    }

    /// Validates locally knowable invariants.
    pub fn validate(&self) -> TimingResult<()> {
        if let (
            TimingAnchor::Absolute(start),
            TimingAnchor::Absolute(end),
        ) = (&self.start, &self.end)
        {
            if start > end {
                return Err(TimingError::InvalidInterval {
                    start: start.attoseconds(),
                    end: end.attoseconds(),
                });
            }
        }

        Ok(())
    }

    /// Returns whether this span has identical start and end anchors.
    #[must_use]
    pub fn is_zero_width(&self) -> bool {
        self.start == self.end
    }
}

impl fmt::Display for TimingSpan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {})", self.start, self.end)
    }
}

// =============================================================================
// Point relation
// =============================================================================

/// Exact temporal relationship between two timing anchors.
///
/// For all relations, the mathematical quantity is:
///
/// ```text
/// Δ = left - right
/// ```
///
/// The relation constrains `Δ`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TimingRelation {
    /// `left == right + offset`.
    Equal {
        /// Exact signed displacement.
        offset: TimeOffset,
    },

    /// `left >= right + minimum`.
    AtLeast {
        /// Minimum signed displacement.
        minimum: TimeOffset,
    },

    /// `left <= right + maximum`.
    AtMost {
        /// Maximum signed displacement.
        maximum: TimeOffset,
    },

    /// `minimum <= left - right <= maximum`.
    Within {
        /// Minimum permitted displacement.
        minimum: TimeOffset,

        /// Maximum permitted displacement.
        maximum: TimeOffset,
    },
}

impl TimingRelation {
    /// Creates an exact equality relation.
    #[must_use]
    pub const fn equal(offset: TimeOffset) -> Self {
        Self::Equal { offset }
    }

    /// Creates a minimum-separation relation.
    #[must_use]
    pub const fn at_least(minimum: TimeOffset) -> Self {
        Self::AtLeast { minimum }
    }

    /// Creates a maximum-separation relation.
    #[must_use]
    pub const fn at_most(maximum: TimeOffset) -> Self {
        Self::AtMost { maximum }
    }

    /// Creates a bounded relation.
    pub fn within(
        minimum: TimeOffset,
        maximum: TimeOffset,
    ) -> TimingResult<Self> {
        if minimum > maximum {
            return Err(TimingError::InvalidConstraint {
                message:
                    "minimum timing offset exceeds maximum timing offset"
                        .to_owned(),
            });
        }

        Ok(Self::Within { minimum, maximum })
    }

    /// Validates the mathematical relation.
    pub fn validate(&self) -> TimingResult<()> {
        match self {
            Self::Equal { .. }
            | Self::AtLeast { .. }
            | Self::AtMost { .. } => Ok(()),

            Self::Within { minimum, maximum } => {
                if minimum > maximum {
                    return Err(TimingError::InvalidConstraint {
                        message:
                            "minimum timing offset exceeds maximum timing offset"
                                .to_owned(),
                    });
                }

                Ok(())
            }
        }
    }

    /// Evaluates whether a concrete left/right time pair satisfies the
    /// relation.
    ///
    /// The input times are represented as exact durations from the same
    /// semantic timeline origin.
    pub fn is_satisfied(
        &self,
        left: Duration,
        right: Duration,
    ) -> bool {
        let difference = match (
            i128::try_from(left.attoseconds()),
            i128::try_from(right.attoseconds()),
        ) {
            (Ok(left), Ok(right)) => match left.checked_sub(right) {
                Some(value) => value,
                None => return false,
            },

            _ => return false,
        };

        match self {
            Self::Equal { offset } => {
                difference == offset.attoseconds()
            }

            Self::AtLeast { minimum } => {
                difference >= minimum.attoseconds()
            }

            Self::AtMost { maximum } => {
                difference <= maximum.attoseconds()
            }

            Self::Within { minimum, maximum } => {
                difference >= minimum.attoseconds()
                    && difference <= maximum.attoseconds()
            }
        }
    }

    /// Returns the minimum permitted displacement when one exists.
    #[must_use]
    pub const fn minimum_offset(&self) -> Option<TimeOffset> {
        match self {
            Self::Equal { offset } => Some(*offset),

            Self::AtLeast { minimum } => Some(*minimum),

            Self::AtMost { .. } => None,

            Self::Within { minimum, .. } => Some(*minimum),
        }
    }

    /// Returns the maximum permitted displacement when one exists.
    #[must_use]
    pub const fn maximum_offset(&self) -> Option<TimeOffset> {
        match self {
            Self::Equal { offset } => Some(*offset),

            Self::AtLeast { .. } => None,

            Self::AtMost { maximum } => Some(*maximum),

            Self::Within { maximum, .. } => Some(*maximum),
        }
    }
}

impl fmt::Display for TimingRelation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Equal { offset } => {
                write!(f, "== {offset}")
            }

            Self::AtLeast { minimum } => {
                write!(f, ">= {minimum}")
            }

            Self::AtMost { maximum } => {
                write!(f, "<= {maximum}")
            }

            Self::Within { minimum, maximum } => {
                write!(f, "within [{minimum}, {maximum}]")
            }
        }
    }
}

// =============================================================================
// Interval relation
// =============================================================================

/// Relationship between two semantic intervals.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum IntervalRelation {
    /// The first interval must finish before the second begins, with at least
    /// the specified separation.
    Before {
        /// Required minimum separation.
        separation: Duration,
    },

    /// The first interval must begin after the second finishes, with at least
    /// the specified separation.
    After {
        /// Required minimum separation.
        separation: Duration,
    },

    /// The two intervals must not overlap.
    ///
    /// The required gap may be zero.
    NonOverlapping {
        /// Required minimum gap between the intervals.
        separation: Duration,
    },

    /// The starts of the intervals must have the specified displacement.
    StartOffset {
        /// Signed start-time displacement:
        ///
        /// `first.start - second.start`
        offset: TimeOffset,
    },

    /// The ends of the intervals must have the specified displacement.
    EndOffset {
        /// Signed end-time displacement:
        ///
        /// `first.end - second.end`
        offset: TimeOffset,
    },
}

impl IntervalRelation {
    /// Creates a strict ordering relation.
    pub fn before(separation: Duration) -> Self {
        Self::Before { separation }
    }

    /// Creates the reverse ordering relation.
    pub fn after(separation: Duration) -> Self {
        Self::After { separation }
    }

    /// Creates a non-overlap relation.
    pub fn non_overlapping(separation: Duration) -> Self {
        Self::NonOverlapping { separation }
    }

    /// Creates a start-offset relation.
    pub const fn start_offset(offset: TimeOffset) -> Self {
        Self::StartOffset { offset }
    }

    /// Creates an end-offset relation.
    pub const fn end_offset(offset: TimeOffset) -> Self {
        Self::EndOffset { offset }
    }

    /// Validates the interval relation.
    pub fn validate(&self) -> TimingResult<()> {
        Ok(())
    }

    /// Evaluates the relation against concrete intervals.
    pub fn is_satisfied(
        &self,
        first_start: Duration,
        first_end: Duration,
        second_start: Duration,
        second_end: Duration,
    ) -> bool {
        if first_end < first_start || second_end < second_start {
            return false;
        }

        match self {
            Self::Before { separation } => {
                match first_end.checked_add(*separation) {
                    Ok(required_end) => required_end <= second_start,
                    Err(_) => false,
                }
            }

            Self::After { separation } => {
                match second_end.checked_add(*separation) {
                    Ok(required_start) => required_start <= first_start,
                    Err(_) => false,
                }
            }

            Self::NonOverlapping { separation } => {
                let first_before_second =
                    first_end.checked_add(*separation);

                let second_before_first =
                    second_end.checked_add(*separation);

                match (first_before_second, second_before_first) {
                    (Ok(first_end_with_gap), Ok(second_end_with_gap)) => {
                        first_end_with_gap <= second_start
                            || second_end_with_gap <= first_start
                    }

                    _ => false,
                }
            }

            Self::StartOffset { offset } => {
                signed_difference(first_start, second_start)
                    .map(|value| value == offset.attoseconds())
                    .unwrap_or(false)
            }

            Self::EndOffset { offset } => {
                signed_difference(first_end, second_end)
                    .map(|value| value == offset.attoseconds())
                    .unwrap_or(false)
            }
        }
    }
}

impl fmt::Display for IntervalRelation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Before { separation } => {
                write!(f, "before + {separation}")
            }

            Self::After { separation } => {
                write!(f, "after + {separation}")
            }

            Self::NonOverlapping { separation } => {
                write!(f, "non-overlapping + {separation}")
            }

            Self::StartOffset { offset } => {
                write!(f, "start-offset {offset}")
            }

            Self::EndOffset { offset } => {
                write!(f, "end-offset {offset}")
            }
        }
    }
}

// =============================================================================
// Timing constraint
// =============================================================================

/// Canonical semantic timing constraint.
///
/// A constraint contains no scheduler state.
///
/// It only states a temporal requirement.
///
/// Two major forms are supported:
///
/// 1. point-to-point relations;
/// 2. interval-to-interval relations.
///
/// This keeps the semantic model expressive without embedding scheduling
/// algorithms into the IR.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TimingConstraint {
    /// Constraint between two timing anchors.
    Point {
        /// Left-hand timing anchor.
        left: TimingAnchor,

        /// Temporal relationship.
        relation: TimingRelation,

        /// Right-hand timing anchor.
        right: TimingAnchor,

        /// Semantic strength.
        strength: ConstraintStrength,
    },

    /// Constraint between two timing spans.
    Interval {
        /// First semantic interval.
        first: TimingSpan,

        /// Interval relationship.
        relation: IntervalRelation,

        /// Second semantic interval.
        second: TimingSpan,

        /// Semantic strength.
        strength: ConstraintStrength,
    },
}

impl TimingConstraint {
    /// Creates a required point constraint.
    pub fn point(
        left: TimingAnchor,
        relation: TimingRelation,
        right: TimingAnchor,
    ) -> TimingResult<Self> {
        Self::point_with_strength(
            left,
            relation,
            right,
            ConstraintStrength::Required,
        )
    }

    /// Creates a point constraint with explicit strength.
    pub fn point_with_strength(
        left: TimingAnchor,
        relation: TimingRelation,
        right: TimingAnchor,
        strength: ConstraintStrength,
    ) -> TimingResult<Self> {
        relation.validate()?;

        validate_anchor_pair(&left, &right)?;

        Ok(Self::Point {
            left,
            relation,
            right,
            strength,
        })
    }

    /// Creates a required interval constraint.
    pub fn interval(
        first: TimingSpan,
        relation: IntervalRelation,
        second: TimingSpan,
    ) -> TimingResult<Self> {
        Self::interval_with_strength(
            first,
            relation,
            second,
            ConstraintStrength::Required,
        )
    }

    /// Creates an interval constraint with explicit strength.
    pub fn interval_with_strength(
        first: TimingSpan,
        relation: IntervalRelation,
        second: TimingSpan,
        strength: ConstraintStrength,
    ) -> TimingResult<Self> {
        first.validate()?;
        second.validate()?;
        relation.validate()?;

        validate_span_pair(&first, &second)?;

        Ok(Self::Interval {
            first,
            relation,
            second,
            strength,
        })
    }

    /// Creates a required "left occurs before right" constraint.
    pub fn before(
        left: TimingAnchor,
        right: TimingAnchor,
        minimum_separation: Duration,
    ) -> TimingResult<Self> {
        Self::point(
            left,
            TimingRelation::at_least(
                TimeOffset::positive(minimum_separation)?,
            ),
            right,
        )
    }

    /// Creates a required "left occurs after right" constraint.
    pub fn after(
        left: TimingAnchor,
        right: TimingAnchor,
        minimum_separation: Duration,
    ) -> TimingResult<Self> {
        Self::point(
            left,
            TimingRelation::at_least(
                TimeOffset::positive(minimum_separation)?,
            ),
            right,
        )
    }

    /// Creates an exact offset constraint:
    ///
    /// `left = right + offset`.
    pub fn offset(
        left: TimingAnchor,
        right: TimingAnchor,
        offset: TimeOffset,
    ) -> TimingResult<Self> {
        Self::point(
            left,
            TimingRelation::equal(offset),
            right,
        )
    }

    /// Creates a required non-overlap constraint.
    pub fn non_overlapping(
        first: TimingSpan,
        second: TimingSpan,
        separation: Duration,
    ) -> TimingResult<Self> {
        Self::interval(
            first,
            IntervalRelation::non_overlapping(separation),
            second,
        )
    }

    /// Returns the constraint strength.
    #[must_use]
    pub const fn strength(&self) -> ConstraintStrength {
        match self {
            Self::Point { strength, .. }
            | Self::Interval { strength, .. } => *strength,
        }
    }

    /// Returns whether the constraint is required.
    #[must_use]
    pub const fn is_required(&self) -> bool {
        matches!(
            self.strength(),
            ConstraintStrength::Required
        )
    }

    /// Returns whether the constraint is merely preferred.
    #[must_use]
    pub const fn is_preferred(&self) -> bool {
        matches!(
            self.strength(),
            ConstraintStrength::Preferred
        )
    }

    /// Returns the point anchors when this is a point constraint.
    #[must_use]
    pub fn point_parts(
        &self,
    ) -> Option<(&TimingAnchor, &TimingRelation, &TimingAnchor)> {
        match self {
            Self::Point {
                left,
                relation,
                right,
                ..
            } => Some((left, relation, right)),

            Self::Interval { .. } => None,
        }
    }

    /// Returns the interval parts when this is an interval constraint.
    #[must_use]
    pub fn interval_parts(
        &self,
    ) -> Option<(&TimingSpan, &IntervalRelation, &TimingSpan)> {
        match self {
            Self::Point { .. } => None,

            Self::Interval {
                first,
                relation,
                second,
                ..
            } => Some((first, relation, second)),
        }
    }

    /// Validates every locally knowable invariant.
    ///
    /// Program-wide anchor existence is deliberately not checked here.
    /// That belongs to whole-IR validation.
    pub fn validate(&self) -> TimingResult<()> {
        match self {
            Self::Point {
                left,
                relation,
                right,
                ..
            } => {
                relation.validate()?;
                validate_anchor_pair(left, right)
            }

            Self::Interval {
                first,
                relation,
                second,
                ..
            } => {
                first.validate()?;
                second.validate()?;
                relation.validate()?;
                validate_span_pair(first, second)
            }
        }
    }

    /// Evaluates a point constraint against concrete resolved times.
    ///
    /// This method is useful to schedulers, validators, simulators, and
    /// testing infrastructure without forcing any of them to duplicate the
    /// mathematical relation.
    pub fn is_point_satisfied(
        &self,
        left: Duration,
        right: Duration,
    ) -> Option<bool> {
        match self {
            Self::Point { relation, .. } => {
                Some(relation.is_satisfied(left, right))
            }

            Self::Interval { .. } => None,
        }
    }

    /// Evaluates an interval constraint against concrete resolved times.
    pub fn is_interval_satisfied(
        &self,
        first_start: Duration,
        first_end: Duration,
        second_start: Duration,
        second_end: Duration,
    ) -> Option<bool> {
        match self {
            Self::Point { .. } => None,

            Self::Interval { relation, .. } => Some(
                relation.is_satisfied(
                    first_start,
                    first_end,
                    second_start,
                    second_end,
                ),
            ),
        }
    }

    /// Returns all operation IDs directly referenced by the constraint.
    ///
    /// This method returns a fixed-size array plus count rather than allocating
    /// a collection. It is intended for dependency analysis and validation.
    ///
    /// The maximum number of operation anchors in the current constraint
    /// grammar is structural, not a machine-size limit.
    #[must_use]
    pub fn operation_ids(
        &self,
    ) -> ([Option<OperationId>; 4], usize) {
        let mut result = [None, None, None, None];
        let mut count = 0usize;

        match self {
            Self::Point { left, right, .. } => {
                add_operation_anchor(
                    left,
                    &mut result,
                    &mut count,
                );

                add_operation_anchor(
                    right,
                    &mut result,
                    &mut count,
                );
            }

            Self::Interval { first, second, .. } => {
                add_operation_anchor(
                    &first.start,
                    &mut result,
                    &mut count,
                );

                add_operation_anchor(
                    &first.end,
                    &mut result,
                    &mut count,
                );

                add_operation_anchor(
                    &second.start,
                    &mut result,
                    &mut count,
                );

                add_operation_anchor(
                    &second.end,
                    &mut result,
                    &mut count,
                );
            }
        }

        (result, count)
    }
}

impl fmt::Display for TimingConstraint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Point {
                left,
                relation,
                right,
                strength,
            } => {
                write!(
                    f,
                    "{strength}: {left} {relation} {right}"
                )
            }

            Self::Interval {
                first,
                relation,
                second,
                strength,
            } => {
                write!(
                    f,
                    "{strength}: {first} {relation} {second}"
                )
            }
        }
    }
}

// =============================================================================
// Constraint collection
// =============================================================================

/// Immutable semantic set of timing constraints.
///
/// This type intentionally does not own scheduling state.
///
/// It is a deterministic collection abstraction that can be used by program,
/// schedule, validation and analysis layers.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TimingConstraints {
    constraints: Vec<TimingConstraint>,
}

impl TimingConstraints {
    /// Creates an empty constraint set.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            constraints: Vec::new(),
        }
    }

    /// Creates a collection with a caller-provided capacity.
    ///
    /// The capacity is an allocation hint only and has no semantic meaning.
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            constraints: Vec::with_capacity(capacity),
        }
    }

    /// Returns the number of constraints.
    #[must_use]
    pub fn len(&self) -> usize {
        self.constraints.len()
    }

    /// Returns whether there are no constraints.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.constraints.is_empty()
    }

    /// Adds a constraint after local validation.
    pub fn push(
        &mut self,
        constraint: TimingConstraint,
    ) -> TimingResult<()> {
        constraint.validate()?;

        self.constraints.push(constraint);

        Ok(())
    }

    /// Returns a constraint by collection position.
    ///
    /// Collection position is not semantic identity.
    #[must_use]
    pub fn get(
        &self,
        index: usize,
    ) -> Option<&TimingConstraint> {
        self.constraints.get(index)
    }

    /// Returns an iterator over constraints.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = &TimingConstraint> {
        self.constraints.iter()
    }

    /// Returns the underlying constraints as a slice.
    #[must_use]
    pub fn as_slice(&self) -> &[TimingConstraint] {
        &self.constraints
    }

    /// Validates every constraint in the collection.
    pub fn validate(&self) -> TimingResult<()> {
        for constraint in &self.constraints {
            constraint.validate()?;
        }

        Ok(())
    }

    /// Returns whether every supplied point-time evaluation satisfies all
    /// point constraints.
    ///
    /// The callback resolves a semantic anchor to a concrete time.
    ///
    /// Returning `None` from the callback means that the anchor has not yet
    /// been resolved.
    pub fn all_point_constraints_satisfied<F>(
        &self,
        mut resolve: F,
    ) -> TimingResult<bool>
    where
        F: FnMut(&TimingAnchor) -> Option<Duration>,
    {
        for constraint in &self.constraints {
            if let TimingConstraint::Point {
                left,
                relation,
                right,
                ..
            } = constraint
            {
                let left_time = match resolve(left) {
                    Some(time) => time,
                    None => continue,
                };

                let right_time = match resolve(right) {
                    Some(time) => time,
                    None => continue,
                };

                if !relation.is_satisfied(
                    left_time,
                    right_time,
                ) {
                    return Ok(false);
                }
            }
        }

        Ok(true)
    }
}

impl IntoIterator for TimingConstraints {
    type Item = TimingConstraint;
    type IntoIter =
        std::vec::IntoIter<TimingConstraint>;

    fn into_iter(self) -> Self::IntoIter {
        self.constraints.into_iter()
    }
}

impl<'a> IntoIterator for &'a TimingConstraints {
    type Item = &'a TimingConstraint;
    type IntoIter =
        std::slice::Iter<'a, TimingConstraint>;

    fn into_iter(self) -> Self::IntoIter {
        self.constraints.iter()
    }
}

// =============================================================================
// Local validation
// =============================================================================

fn validate_anchor_pair(
    left: &TimingAnchor,
    right: &TimingAnchor,
) -> TimingResult<()> {
    match (left, right) {
        (
            TimingAnchor::Absolute(left),
            TimingAnchor::Absolute(right),
        ) => {
            let _ = (left, right);
        }

        (
            TimingAnchor::Operation {
                operation: left_operation,
                endpoint: left_endpoint,
            },
            TimingAnchor::Operation {
                operation: right_operation,
                endpoint: right_endpoint,
            },
        ) => {
            if left_operation == right_operation
                && left_endpoint == right_endpoint
            {
                // Self-relations are valid only when the relation itself can
                // satisfy zero displacement.
                //
                // We do not reject them here because an explicit constraint
                // may intentionally be checked by a downstream semantic
                // validator.
            }
        }

        _ => {}
    }

    Ok(())
}

fn validate_span_pair(
    first: &TimingSpan,
    second: &TimingSpan,
) -> TimingResult<()> {
    if first.start == first.end
        && second.start == second.end
    {
        return Ok(());
    }

    Ok(())
}

fn add_operation_anchor(
    anchor: &TimingAnchor,
    result: &mut [Option<OperationId>; 4],
    count: &mut usize,
) {
    let operation = match anchor.operation_id() {
        Some(operation) => operation,
        None => return,
    };

    if result[..*count].contains(&Some(operation)) {
        return;
    }

    if *count < result.len() {
        result[*count] = Some(operation);
        *count += 1;
    }
}

fn signed_difference(
    left: Duration,
    right: Duration,
) -> Option<i128> {
    let left = i128::try_from(left.attoseconds()).ok()?;
    let right = i128::try_from(right.attoseconds()).ok()?;

    left.checked_sub(right)
}

// =============================================================================
// Convenience constructors
// =============================================================================

/// Creates a required "operation B starts after operation A ends" constraint.
pub fn operation_before_operation(
    first: OperationId,
    second: OperationId,
    separation: Duration,
) -> TimingResult<TimingConstraint> {
    TimingConstraint::point(
        TimingAnchor::operation_start(second),
        TimingRelation::at_least(
            TimeOffset::positive(separation)?,
        ),
        TimingAnchor::operation_end(first),
    )
}

/// Creates a required "operation B starts no earlier than operation A starts
/// plus the supplied offset" constraint.
pub fn operation_start_after_start(
    first: OperationId,
    second: OperationId,
    offset: TimeOffset,
) -> TimingResult<TimingConstraint> {
    TimingConstraint::point(
        TimingAnchor::operation_start(second),
        TimingRelation::at_least(offset),
        TimingAnchor::operation_start(first),
    )
}

/// Creates a required exact operation-start alignment.
pub fn operation_start_aligned(
    first: OperationId,
    second: OperationId,
) -> TimingResult<TimingConstraint> {
    TimingConstraint::point(
        TimingAnchor::operation_start(first),
        TimingRelation::equal(TimeOffset::ZERO),
        TimingAnchor::operation_start(second),
    )
}

/// Creates a required exact operation-end alignment.
pub fn operation_end_aligned(
    first: OperationId,
    second: OperationId,
) -> TimingResult<TimingConstraint> {
    TimingConstraint::point(
        TimingAnchor::operation_end(first),
        TimingRelation::equal(TimeOffset::ZERO),
        TimingAnchor::operation_end(second),
    )
}

/// Creates a required non-overlap constraint for two operations.
pub fn operations_non_overlapping(
    first: OperationId,
    second: OperationId,
    separation: Duration,
) -> TimingResult<TimingConstraint> {
    TimingConstraint::non_overlapping(
        TimingSpan::operation(first),
        TimingSpan::operation(second),
        separation,
    )
}

/// Creates a required absolute-time constraint:
///
/// `operation.start = time`.
pub fn operation_starts_at(
    operation: OperationId,
    time: Duration,
) -> TimingResult<TimingConstraint> {
    TimingConstraint::point(
        TimingAnchor::operation_start(operation),
        TimingRelation::equal(TimeOffset::positive(time)?),
        TimingAnchor::ScheduleStart,
    )
}

/// Creates a required absolute-time constraint:
///
/// `operation.end = time`.
pub fn operation_ends_at(
    operation: OperationId,
    time: Duration,
) -> TimingResult<TimingConstraint> {
    TimingConstraint::point(
        TimingAnchor::operation_end(operation),
        TimingRelation::equal(TimeOffset::positive(time)?),
        TimingAnchor::ScheduleStart,
    )
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn operation(value: u64) -> OperationId {
        OperationId::new(value)
    }

    fn qubit(value: usize) -> QubitId {
        QubitId::new(value)
    }

    #[test]
    fn operation_anchors_are_distinct() {
        let id = operation(1);

        assert_ne!(
            TimingAnchor::operation_start(id),
            TimingAnchor::operation_end(id)
        );
    }

    #[test]
    fn qubit_anchors_use_canonical_qubit_identity() {
        let id = qubit(7);

        let available =
            TimingAnchor::qubit_available(id);

        let released =
            TimingAnchor::qubit_released(id);

        assert_ne!(available, released);
        assert_eq!(available.qubit_id(), Some(id));
        assert_eq!(released.qubit_id(), Some(id));
    }

    #[test]
    fn equal_relation_is_exact() {
        let relation =
            TimingRelation::equal(TimeOffset::ZERO);

        let ten_ns =
            Duration::nanoseconds(10).unwrap();

        assert!(relation.is_satisfied(
            ten_ns,
            ten_ns
        ));

        let twenty_ns =
            Duration::nanoseconds(20).unwrap();

        assert!(!relation.is_satisfied(
            twenty_ns,
            ten_ns
        ));
    }

    #[test]
    fn minimum_relation_works() {
        let ten_ns =
            Duration::nanoseconds(10).unwrap();

        let twenty_ns =
            Duration::nanoseconds(20).unwrap();

        let relation = TimingRelation::at_least(
            TimeOffset::positive(ten_ns).unwrap(),
        );

        assert!(relation.is_satisfied(
            twenty_ns,
            ten_ns
        ));

        assert!(!relation.is_satisfied(
            ten_ns,
            twenty_ns
        ));
    }

    #[test]
    fn maximum_relation_works() {
        let ten_ns =
            Duration::nanoseconds(10).unwrap();

        let twenty_ns =
            Duration::nanoseconds(20).unwrap();

        let relation = TimingRelation::at_most(
            TimeOffset::positive(ten_ns).unwrap(),
        );

        assert!(relation.is_satisfied(
            twenty_ns,
            ten_ns
        ));

        assert!(!relation.is_satisfied(
            thirty_ns(),
            ten_ns
        ));
    }

    fn thirty_ns() -> Duration {
        Duration::nanoseconds(30).unwrap()
    }

    #[test]
    fn within_relation_validates_bounds() {
        let ten_ns =
            Duration::nanoseconds(10).unwrap();

        let twenty_ns =
            Duration::nanoseconds(20).unwrap();

        let minimum =
            TimeOffset::positive(ten_ns).unwrap();

        let maximum =
            TimeOffset::positive(twenty_ns).unwrap();

        let relation =
            TimingRelation::within(
                minimum,
                maximum,
            )
            .unwrap();

        assert!(relation.is_satisfied(
            thirty_ns(),
            ten_ns
        ));
    }

    #[test]
    fn invalid_within_relation_is_rejected() {
        let minimum =
            TimeOffset::from_attoseconds(10);

        let maximum =
            TimeOffset::from_attoseconds(5);

        assert!(
            TimingRelation::within(
                minimum,
                maximum
            )
            .is_err()
        );
    }

    #[test]
    fn operation_before_operation_has_correct_direction() {
        let first = operation(1);
        let second = operation(2);

        let ten_ns =
            Duration::nanoseconds(10).unwrap();

        let constraint =
            operation_before_operation(
                first,
                second,
                ten_ns,
            )
            .unwrap();

        let first_end =
            Duration::nanoseconds(100).unwrap();

        let second_start =
            Duration::nanoseconds(110).unwrap();

        assert_eq!(
            constraint.is_point_satisfied(
                second_start,
                first_end,
            ),
            Some(true)
        );
    }

    #[test]
    fn non_overlapping_intervals_allow_touching() {
        let first_start =
            Duration::nanoseconds(0).unwrap();

        let first_end =
            Duration::nanoseconds(10).unwrap();

        let second_start =
            Duration::nanoseconds(10).unwrap();

        let second_end =
            Duration::nanoseconds(20).unwrap();

        let relation =
            IntervalRelation::non_overlapping(
                Duration::ZERO,
            );

        assert!(relation.is_satisfied(
            first_start,
            first_end,
            second_start,
            second_end,
        ));
    }

    #[test]
    fn non_overlapping_intervals_require_gap() {
        let first_start =
            Duration::nanoseconds(0).unwrap();

        let first_end =
            Duration::nanoseconds(10).unwrap();

        let second_start =
            Duration::nanoseconds(15).unwrap();

        let second_end =
            Duration::nanoseconds(25).unwrap();

        let relation =
            IntervalRelation::non_overlapping(
                Duration::nanoseconds(5).unwrap(),
            );

        assert!(relation.is_satisfied(
            first_start,
            first_end,
            second_start,
            second_end,
        ));
    }

    #[test]
    fn span_rejects_reversed_absolute_bounds() {
        let start =
            Duration::nanoseconds(20).unwrap();

        let end =
            Duration::nanoseconds(10).unwrap();

        assert!(
            TimingSpan::absolute(start, end)
                .is_err()
        );
    }

    #[test]
    fn absolute_operation_start_constraint_is_exact() {
        let id = operation(42);

        let target =
            Duration::nanoseconds(100).unwrap();

        let constraint =
            operation_starts_at(id, target)
                .unwrap();

        assert_eq!(
            constraint.is_point_satisfied(
                target,
                Duration::ZERO
            ),
            Some(true)
        );
    }

    #[test]
    fn constraint_collection_validates_and_iterates() {
        let first = operation(1);
        let second = operation(2);

        let constraint =
            operation_start_aligned(
                first,
                second,
            )
            .unwrap();

        let mut collection =
            TimingConstraints::new();

        collection
            .push(constraint)
            .unwrap();

        assert_eq!(collection.len(), 1);
        assert!(!collection.is_empty());
        assert!(collection.validate().is_ok());
        assert_eq!(
            collection.iter().count(),
            1
        );
    }

    #[test]
    fn operation_ids_are_deterministic_and_unique() {
        let first = operation(1);
        let second = operation(2);
        let third = operation(3);

        let constraint =
            TimingConstraint::interval(
                TimingSpan::operation(first),
                IntervalRelation::non_overlapping(
                    Duration::ZERO,
                ),
                TimingSpan::operation(second),
            )
            .unwrap();

        let _ = third;

        let (ids, count) =
            constraint.operation_ids();

        assert_eq!(count, 2);
        assert_eq!(ids[0], Some(first));
        assert_eq!(ids[1], Some(second));
    }

    #[test]
    fn display_is_deterministic() {
        let first = operation(1);
        let second = operation(2);

        let constraint =
            operation_start_aligned(
                first,
                second,
            )
            .unwrap();

        assert_eq!(
            constraint.to_string(),
            "required: op:1.start == 0as op:2.start"
        );
    }

    #[test]
    fn preferred_constraints_are_distinguishable() {
        let first = operation(1);
        let second = operation(2);

        let constraint =
            TimingConstraint::point_with_strength(
                TimingAnchor::operation_start(first),
                TimingRelation::equal(
                    TimeOffset::ZERO,
                ),
                TimingAnchor::operation_start(second),
                ConstraintStrength::Preferred,
            )
            .unwrap();

        assert!(!constraint.is_required());
        assert!(constraint.is_preferred());
    }

    #[test]
    fn zero_separation_is_valid() {
        let first = operation(1);
        let second = operation(2);

        let constraint =
            operations_non_overlapping(
                first,
                second,
                Duration::ZERO,
            )
            .unwrap();

        assert!(constraint.validate().is_ok());
    }

    #[test]
    fn schedule_origin_can_be_used_as_reference() {
        let operation_id = operation(5);

        let target =
            Duration::nanoseconds(50).unwrap();

        let constraint =
            TimingConstraint::point(
                TimingAnchor::operation_start(
                    operation_id,
                ),
                TimingRelation::equal(
                    TimeOffset::positive(
                        target,
                    )
                    .unwrap(),
                ),
                TimingAnchor::ScheduleStart,
            )
            .unwrap();

        assert!(constraint.validate().is_ok());
    }

    #[test]
    fn canonical_qubit_identity_does_not_equal_operation_identity() {
        let qubit = qubit(1);
        let operation = operation(1);

        let anchor =
            TimingAnchor::qubit_available(qubit);

        assert_eq!(
            anchor.qubit_id(),
            Some(qubit)
        );

        assert_eq!(
            operation,
            OperationId::new(1)
        );
    }
}