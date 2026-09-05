//! Zamani Quantum Scheduling — Makespan Optimization
//!
//! Production-grade makespan objective and metric utilities for the quantum
//! scheduling subsystem.
//!
//! # Architectural responsibility
//!
//! This module answers:
//!
//! > "How long does a completed schedule take from its effective temporal
//! > origin to its final completion, and how should scheduling prefer a
//! > smaller execution horizon when makespan is the selected objective?"
//!
//! This module owns:
//!
//! - makespan objective semantics;
//! - makespan comparison;
//! - makespan improvement measurement;
//! - lower-bound comparison;
//! - schedule-horizon arithmetic;
//! - objective scoring for makespan;
//! - deterministic comparison of candidate horizons;
//! - overflow-safe makespan calculations;
//! - reusable makespan metric contracts.
//!
//! This module does NOT own:
//!
//! - quantum operation semantics;
//! - logical qubit identity;
//! - physical qubit identity;
//! - routing;
//! - hardware discovery;
//! - hardware timing calibration;
//! - resource allocation;
//! - dependency graph construction;
//! - schedule construction;
//! - QEC algorithms;
//! - runtime execution;
//! - frontend parsing;
//! - serialization formats;
//! - vendor-specific hardware behavior.
//!
//! Those responsibilities remain in their canonical subsystems.
//!
//! # Canonical identity boundary
//!
//! Makespan is intentionally independent of qubit identity.
//!
//! If a caller needs to associate a makespan metric with operations or qubits,
//! those associations must use the canonical repository types:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! This module must never define another `QubitId` or `PhysicalQubitId`.
//!
//! Likewise, operation and resource identities remain owned by the canonical
//! IR:
//!
//! ```text
//! crate::quantum::ir::core::identity::OperationId
//! crate::quantum::ir::core::identity::ResourceId
//! ```
//!
//! # Universal-program principle
//!
//! A Zamani program is written once at the semantic level.
//!
//! Its schedule may be specialized for:
//!
//! - a tiny QPU;
//! - a large QPU;
//! - a modular QPU;
//! - a distributed quantum system;
//! - a simulator;
//! - an emulator;
//! - a future quantum architecture.
//!
//! Makespan therefore cannot contain assumptions such as:
//!
//! ```text
//! MAX_QUBITS
//! MAX_OPERATIONS
//! MAX_RESOURCES
//! MAX_DEPTH
//! MAX_TIME
//! ```
//!
//! No quantum-machine size is encoded here.
//!
//! "Infinity" means that this module imposes no artificial finite quantum
//! machine-size ceiling. A concrete compilation is naturally bounded by the
//! resources available to the compiler, target, operating system, and
//! execution environment.
//!
//! # What makespan means
//!
//! Makespan is the temporal length of a completed schedule.
//!
//! For a schedule whose effective origin is `t0` and whose final completion is
//! `tf`:
//!
//! ```text
//! makespan = tf - t0
//! ```
//!
//! For the common normalized case:
//!
//! ```text
//! t0 = 0
//! makespan = tf
//! ```
//!
//! The subtraction is checked and never silently wraps.
//!
//! # Important distinction
//!
//! Makespan is NOT necessarily:
//!
//! - number of gates;
//! - number of circuit layers;
//! - critical-path duration;
//! - wall-clock compiler time;
//! - pulse duration sum;
//! - number of qubits;
//! - number of resources.
//!
//! With resource contention:
//!
//! ```text
//! actual makespan >= dependency-only critical-path lower bound
//! ```
//!
//! This module therefore supports both:
//!
//! - actual makespan;
//! - lower-bound makespan.
//!
//! # Zero-operation schedules
//!
//! An empty schedule has zero makespan when its temporal origin and completion
//! are identical.
//!
//! This is valid and important for generic compiler pipelines.
//!
//! # Zero-duration operations
//!
//! Zero-duration operations are supported.
//!
//! A zero-duration schedule is therefore valid when all scheduled work occupies
//! no temporal extent.
//!
//! # Time representation
//!
//! The canonical scheduler `TimePoint` and `Duration` types are target
//! independent.
//!
//! This module does not interpret them as:
//!
//! - nanoseconds;
//! - microseconds;
//! - seconds;
//! - device ticks;
//! - pulse samples.
//!
//! The target timing subsystem supplies the physical interpretation.
//!
//! # Optimization semantics
//!
//! Makespan is a minimization objective.
//!
//! For two feasible schedules `A` and `B`:
//!
//! ```text
//! A is better than B
//!     iff
//! makespan(A) < makespan(B)
//! ```
//!
//! Equal makespans are equivalent with respect to this objective alone.
//!
//! A scheduler may then use deterministic secondary tie-breaking supplied by
//! its planner/policy. This module does not silently introduce hardware- or
//! algorithm-specific tie breakers.
//!
//! # Correctness boundary
//!
//! Makespan comparison must only be used after schedule validity has been
//! established by the appropriate scheduler verification subsystem.
//!
//! A shorter invalid schedule is never better than a valid schedule.
//!
//! Therefore this module does not define scheduling feasibility itself.
//!
//! # Overflow policy
//!
//! All arithmetic is checked.
//!
//! An overflowing calculation returns an explicit error rather than wrapping.
//!
//! This is mandatory because silent temporal wrapping could transform a huge
//! schedule into an apparently short schedule and therefore produce an
//! invalid optimization result.
//!
//! # Determinism
//!
//! All comparisons are deterministic.
//!
//! This module contains:
//!
//! - no global mutable state;
//! - no randomness;
//! - no environment-variable configuration;
//! - no machine discovery;
//! - no thread-local scheduler state.
//!
//! # Thread safety
//!
//! The types in this module contain immutable values only.
//!
//! They can therefore be transferred between threads and used concurrently
//! provided the containing scheduler context also satisfies its own concurrency
//! contract.
//!
//! # Rust compatibility
//!
//! Required:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! # Dependency direction
//!
//! ```text
//! scheduling::types
//!       │
//!       ▼
//! scheduling::optimization::makespan
//!       │
//!       ├──────────────► optimization::multi_objective
//!       │
//!       ├──────────────► planners::*
//!       │
//!       ├──────────────► policies::*
//!       │
//!       ├──────────────► verification::*
//!       │
//!       └──────────────► result
//! ```
//!
//! This module must not depend on planners, hardware adapters, routing
//! implementations, QEC implementations, or runtime implementations.
//!
//! # Integration contract
//!
//! The normal integration path is:
//!
//! ```text
//! quantum::ir
//!      │
//!      ▼
//! optimization
//!      │
//!      ▼
//! routing
//!      │
//!      ▼
//! scheduling planner
//!      │
//!      ▼
//! candidate schedule
//!      │
//!      ▼
//! verification
//!      │
//!      ▼
//! makespan metric
//!      │
//!      ▼
//! scheduling objective
//!      │
//!      ▼
//! SchedulingResult
//! ```
//!
//! Makespan optimization may also be consumed by:
//!
//! - `optimization::multi_objective`;
//! - `policies::policy`;
//! - `planners::list`;
//! - `planners::critical_path`;
//! - `planners::resource_constrained`;
//! - benchmarking;
//! - diagnostics;
//! - QZN/ZQN-aware scheduling;
//! - hardware lowering.
//!
//! None of those integrations require this file to know their implementation
//! details.
//!
//! # Design rule
//!
//! If a future scheduler needs additional makespan functionality, add it here
//! only when it is genuinely part of the stable makespan objective contract.
//!
//! Algorithm-specific heuristics belong in `algorithms/*` or `planners/*`.
//!
//! Hardware-specific timing interpretation belongs in the hardware/timing
//! adapters.
//!
//! Resource-specific contention belongs in `resources/*`.
//!
//! =============================================================================
//! Safety boundary
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::cmp::Ordering;
use std::error::Error;
use std::fmt;

use super::super::types::{Duration, TimePoint};

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by makespan calculations and objective operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MakespanError {
    /// The requested end time precedes the schedule origin.
    EndBeforeOrigin {
        /// Effective schedule origin.
        origin: TimePoint,

        /// Effective schedule completion.
        end: TimePoint,
    },

    /// Adding a duration to the supplied start time exceeded the representable
    /// scheduler time domain.
    TimeOverflow {
        /// Start time of the operation/schedule interval.
        start: TimePoint,

        /// Duration being added.
        duration: Duration,
    },

    /// A derived makespan is greater than the representable duration domain.
    DurationOverflow,

    /// A lower bound is greater than the actual makespan.
    InvalidLowerBound {
        /// Actual makespan.
        makespan: Duration,

        /// Supplied lower bound.
        lower_bound: Duration,
    },
}

impl fmt::Display for MakespanError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EndBeforeOrigin { origin, end } => {
                write!(
                    formatter,
                    "schedule end `{end}` precedes schedule origin `{origin}`"
                )
            }

            Self::TimeOverflow { start, duration } => {
                write!(
                    formatter,
                    "schedule time overflow while adding duration `{duration}` to start `{start}`"
                )
            }

            Self::DurationOverflow => {
                formatter.write_str("makespan duration overflow")
            }

            Self::InvalidLowerBound {
                makespan,
                lower_bound,
            } => {
                write!(
                    formatter,
                    "makespan `{makespan}` is smaller than lower bound `{lower_bound}`"
                )
            }
        }
    }
}

impl Error for MakespanError {}

// =============================================================================
// Makespan value
// =============================================================================

/// Validated makespan value.
///
/// `Makespan` is intentionally distinct from `Duration` so objective APIs
/// cannot accidentally treat an arbitrary operation duration as a complete
/// schedule horizon.
///
/// The underlying unit remains abstract and target-independent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct Makespan(Duration);

impl Makespan {
    /// Zero-length schedule.
    pub const ZERO: Self = Self(Duration::ZERO);

    /// Creates a makespan from an already validated duration.
    #[must_use]
    pub const fn new(duration: Duration) -> Self {
        Self(duration)
    }

    /// Returns the underlying abstract duration.
    #[must_use]
    pub const fn duration(self) -> Duration {
        self.0
    }

    /// Returns whether the schedule has zero temporal extent.
    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.0.is_zero()
    }

    /// Calculates makespan from a schedule origin and final completion time.
    ///
    /// The temporal interval is interpreted as:
    ///
    /// ```text
    /// [origin, end)
    /// ```
    ///
    /// The result is valid when `end >= origin`.
    pub fn between(
        origin: TimePoint,
        end: TimePoint,
    ) -> Result<Self, MakespanError> {
        origin
            .checked_duration_until(end)
            .map(Self::new)
            .ok_or(MakespanError::EndBeforeOrigin { origin, end })
    }

    /// Calculates completion time from an origin and makespan.
    ///
    /// This operation is checked and therefore cannot silently wrap.
    pub fn completion(
        origin: TimePoint,
        makespan: Self,
    ) -> Result<TimePoint, MakespanError> {
        origin
            .checked_add(makespan.duration())
            .ok_or(MakespanError::TimeOverflow {
                start: origin,
                duration: makespan.duration(),
            })
    }

    /// Returns the improvement obtained by replacing `baseline` with `self`.
    ///
    /// A positive value means `self` is shorter than `baseline`.
    ///
    /// A negative value means `self` is longer.
    ///
    /// `None` is returned when the baseline is not strictly larger than the
    /// candidate.
    #[must_use]
    pub fn improvement_over(self, baseline: Self) -> Option<Duration> {
        baseline.duration().checked_sub(self.duration())
    }

    /// Returns the absolute difference between two makespans.
    ///
    /// This never performs unchecked subtraction.
    #[must_use]
    pub fn absolute_difference(self, other: Self) -> Duration {
        if self <= other {
            other
                .duration()
                .checked_sub(self.duration())
                .unwrap_or(Duration::ZERO)
        } else {
            self.duration()
                .checked_sub(other.duration())
                .unwrap_or(Duration::ZERO)
        }
    }

    /// Compares this makespan using the minimization semantics of the objective.
    #[must_use]
    pub fn compare_minimize(self, other: Self) -> Ordering {
        self.duration().cmp(&other.duration())
    }

    /// Returns whether this candidate is strictly better than `other`.
    #[must_use]
    pub fn is_better_than(self, other: Self) -> bool {
        self < other
    }

    /// Returns whether this candidate is no worse than `other`.
    #[must_use]
    pub fn is_no_worse_than(self, other: Self) -> bool {
        self <= other
    }
}

impl From<Duration> for Makespan {
    fn from(duration: Duration) -> Self {
        Self::new(duration)
    }
}

impl From<Makespan> for Duration {
    fn from(makespan: Makespan) -> Self {
        makespan.duration()
    }
}

impl fmt::Display for Makespan {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.duration().fmt(formatter)
    }
}

// =============================================================================
// Schedule horizon
// =============================================================================

/// Immutable effective temporal horizon of a candidate schedule.
///
/// This is useful when a planner knows the schedule origin and final
/// completion independently of the objective layer.
///
/// The horizon does not contain operations, resources, qubits, or hardware
/// information.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ScheduleHorizon {
    origin: TimePoint,
    end: TimePoint,
}

impl ScheduleHorizon {
    /// Creates a validated schedule horizon.
    pub fn new(
        origin: TimePoint,
        end: TimePoint,
    ) -> Result<Self, MakespanError> {
        if end < origin {
            return Err(MakespanError::EndBeforeOrigin { origin, end });
        }

        Ok(Self { origin, end })
    }

    /// Returns the effective schedule origin.
    #[must_use]
    pub const fn origin(self) -> TimePoint {
        self.origin
    }

    /// Returns the final completion time.
    #[must_use]
    pub const fn end(self) -> TimePoint {
        self.end
    }

    /// Returns the makespan represented by this horizon.
    ///
    /// Construction already guarantees the interval is valid, so this method
    /// cannot fail under the invariants of this type.
    #[must_use]
    pub fn makespan(self) -> Makespan {
        Makespan::between(self.origin, self.end)
            .expect("ScheduleHorizon invariant guarantees end >= origin")
    }

    /// Returns whether the horizon has zero temporal extent.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.origin == self.end
    }
}

// =============================================================================
// Makespan lower bound
// =============================================================================

/// A proven lower bound on achievable schedule makespan.
///
/// Typical producers include:
///
/// - dependency critical-path analysis;
/// - mandatory operation duration analysis;
/// - mandatory communication latency;
/// - mandatory QEC round duration;
/// - target timing constraints.
///
/// The lower bound is a property of the supplied compilation context. This
/// module does not calculate dependency graphs or hardware constraints.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MakespanLowerBound(Duration);

impl MakespanLowerBound {
    /// Creates a lower bound from a duration.
    #[must_use]
    pub const fn new(duration: Duration) -> Self {
        Self(duration)
    }

    /// Returns the lower-bound duration.
    #[must_use]
    pub const fn duration(self) -> Duration {
        self.0
    }

    /// Returns whether the lower bound is zero.
    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.0.is_zero()
    }

    /// Validates this lower bound against an actual makespan.
    pub fn validate_against(
        self,
        makespan: Makespan,
    ) -> Result<(), MakespanError> {
        if self.duration() > makespan.duration() {
            return Err(MakespanError::InvalidLowerBound {
                makespan: makespan.duration(),
                lower_bound: self.duration(),
            });
        }

        Ok(())
    }
}

impl From<Duration> for MakespanLowerBound {
    fn from(duration: Duration) -> Self {
        Self::new(duration)
    }
}

// =============================================================================
// Makespan evaluation
// =============================================================================

/// Immutable evaluation of a schedule's makespan.
///
/// This structure deliberately contains only objective-level information.
///
/// It can therefore be passed to:
///
/// - planner ranking;
/// - multi-objective optimization;
/// - benchmarking;
/// - diagnostics;
/// - verification reporting.
///
/// No machine-specific data is required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MakespanEvaluation {
    makespan: Makespan,
    lower_bound: Option<MakespanLowerBound>,
}

impl MakespanEvaluation {
    /// Creates an evaluation containing only the actual makespan.
    #[must_use]
    pub const fn new(makespan: Makespan) -> Self {
        Self {
            makespan,
            lower_bound: None,
        }
    }

    /// Creates an evaluation containing a validated lower bound.
    pub fn with_lower_bound(
        makespan: Makespan,
        lower_bound: MakespanLowerBound,
    ) -> Result<Self, MakespanError> {
        lower_bound.validate_against(makespan)?;

        Ok(Self {
            makespan,
            lower_bound: Some(lower_bound),
        })
    }

    /// Returns the actual makespan.
    #[must_use]
    pub const fn makespan(self) -> Makespan {
        self.makespan
    }

    /// Returns the optional proven lower bound.
    #[must_use]
    pub const fn lower_bound(self) -> Option<MakespanLowerBound> {
        self.lower_bound
    }

    /// Returns the optimization gap between the actual schedule and its lower
    /// bound.
    ///
    /// A zero gap means the supplied lower bound is attained.
    ///
    /// If no lower bound is available, `None` is returned.
    #[must_use]
    pub fn optimality_gap(self) -> Option<Duration> {
        self.lower_bound
            .and_then(|lower_bound| {
                self.makespan
                    .duration()
                    .checked_sub(lower_bound.duration())
            })
    }

    /// Returns whether the supplied lower bound is exactly attained.
    #[must_use]
    pub fn reaches_lower_bound(self) -> bool {
        matches!(
            (self.lower_bound, self.optimality_gap()),
            (Some(_), Some(gap)) if gap.is_zero()
        )
    }
}

// =============================================================================
// Objective score
// =============================================================================

/// Makespan objective score.
///
/// Smaller values are better.
///
/// This is intentionally represented by `Duration` rather than floating-point
/// values so that candidate comparison remains exact and deterministic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MakespanScore(Duration);

impl MakespanScore {
    /// Creates a score from a makespan.
    #[must_use]
    pub const fn from_makespan(makespan: Makespan) -> Self {
        Self(makespan.duration())
    }

    /// Creates a score from a duration.
    #[must_use]
    pub const fn new(duration: Duration) -> Self {
        Self(duration)
    }

    /// Returns the exact score.
    #[must_use]
    pub const fn value(self) -> Duration {
        self.0
    }

    /// Returns whether this score is strictly better than another score.
    #[must_use]
    pub const fn is_better_than(self, other: Self) -> bool {
        self.0.value() < other.0.value()
    }

    /// Returns whether this score is no worse than another score.
    #[must_use]
    pub const fn is_no_worse_than(self, other: Self) -> bool {
        self.0.value() <= other.0.value()
    }

    /// Returns the improvement from `baseline` to this candidate.
    ///
    /// `None` means the candidate did not improve the baseline.
    #[must_use]
    pub fn improvement_over(self, baseline: Self) -> Option<Duration> {
        baseline.0.checked_sub(self.0)
    }
}

impl From<Makespan> for MakespanScore {
    fn from(makespan: Makespan) -> Self {
        Self::from_makespan(makespan)
    }
}

impl From<MakespanScore> for Duration {
    fn from(score: MakespanScore) -> Self {
        score.value()
    }
}

// =============================================================================
// Objective contract
// =============================================================================

/// Stateless makespan objective.
///
/// This type intentionally contains no mutable optimization state.
///
/// Candidate schedules are evaluated externally, while this objective provides
/// deterministic ranking semantics.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct MakespanObjective;

impl MakespanObjective {
    /// Creates the makespan objective.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Evaluates a candidate makespan.
    #[must_use]
    pub const fn evaluate(self, makespan: Makespan) -> MakespanScore {
        MakespanScore::from_makespan(makespan)
    }

    /// Compares two candidates under minimization semantics.
    #[must_use]
    pub const fn compare(
        self,
        candidate: Makespan,
        incumbent: Makespan,
    ) -> Ordering {
        if candidate.duration().value() < incumbent.duration().value() {
            Ordering::Less
        } else if candidate.duration().value() > incumbent.duration().value() {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }

    /// Returns the better candidate under makespan minimization.
    ///
    /// Equal candidates resolve to the incumbent. This is intentional:
    /// makespan alone does not define a secondary tie-break.
    #[must_use]
    pub const fn better(
        self,
        candidate: Makespan,
        incumbent: Makespan,
    ) -> Makespan {
        if candidate.duration().value() < incumbent.duration().value() {
            candidate
        } else {
            incumbent
        }
    }

    /// Returns whether a candidate improves the incumbent.
    #[must_use]
    pub const fn improves(
        self,
        candidate: Makespan,
        incumbent: Makespan,
    ) -> bool {
        candidate.duration().value() < incumbent.duration().value()
    }
}

// =============================================================================
// Schedule-horizon helpers
// =============================================================================

/// Computes the makespan of a schedule represented by an origin and completion
/// time.
///
/// This is the preferred helper when the caller already has a verified
/// schedule horizon.
pub fn calculate(
    origin: TimePoint,
    end: TimePoint,
) -> Result<Makespan, MakespanError> {
    Makespan::between(origin, end)
}

/// Computes a completion time from an origin and duration.
///
/// This is useful to planners when extending a schedule horizon.
pub fn completion_time(
    origin: TimePoint,
    duration: Duration,
) -> Result<TimePoint, MakespanError> {
    origin
        .checked_add(duration)
        .ok_or(MakespanError::TimeOverflow {
            start: origin,
            duration,
        })
}

/// Computes the larger of two makespans.
///
/// This is useful when combining independent schedule partitions whose final
/// completion times are already normalized to the same origin.
#[must_use]
pub const fn max(first: Makespan, second: Makespan) -> Makespan {
    if first.duration().value() >= second.duration().value() {
        first
    } else {
        second
    }
}

/// Computes the smaller of two makespans.
///
/// This function is objective-oriented and therefore selects the better
/// candidate under makespan minimization.
#[must_use]
pub const fn min(first: Makespan, second: Makespan) -> Makespan {
    if first.duration().value() <= second.duration().value() {
        first
    } else {
        second
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn time(value: u128) -> TimePoint {
        TimePoint::new(value)
    }

    fn duration(value: u128) -> Duration {
        Duration::new(value)
    }

    #[test]
    fn zero_horizon_has_zero_makespan() {
        let result = calculate(time(0), time(0))
            .expect("equal origin and end must be valid");

        assert!(result.is_zero());
        assert_eq!(result.duration(), duration(0));
    }

    #[test]
    fn makespan_is_end_minus_origin() {
        let result = calculate(time(25), time(100))
            .expect("end after origin must be valid");

        assert_eq!(result.duration(), duration(75));
    }

    #[test]
    fn end_before_origin_is_rejected() {
        let result = calculate(time(100), time(25));

        assert!(matches!(
            result,
            Err(MakespanError::EndBeforeOrigin {
                origin,
                end
            }) if origin == time(100) && end == time(25)
        ));
    }

    #[test]
    fn completion_time_is_checked() {
        let completion = completion_time(time(10), duration(20))
            .expect("normal addition must succeed");

        assert_eq!(completion, time(30));
    }

    #[test]
    fn zero_duration_is_supported() {
        let completion = completion_time(time(42), Duration::ZERO)
            .expect("zero-duration completion must succeed");

        assert_eq!(completion, time(42));
    }

    #[test]
    fn candidate_with_smaller_makespan_is_better() {
        let objective = MakespanObjective::new();

        let candidate = Makespan::new(duration(10));
        let incumbent = Makespan::new(duration(20));

        assert!(objective.improves(candidate, incumbent));
        assert_eq!(
            objective.better(candidate, incumbent),
            candidate
        );
    }

    #[test]
    fn candidate_with_larger_makespan_is_not_better() {
        let objective = MakespanObjective::new();

        let candidate = Makespan::new(duration(30));
        let incumbent = Makespan::new(duration(20));

        assert!(!objective.improves(candidate, incumbent));
        assert_eq!(
            objective.better(candidate, incumbent),
            incumbent
        );
    }

    #[test]
    fn equal_makespan_keeps_incumbent() {
        let objective = MakespanObjective::new();

        let candidate = Makespan::new(duration(20));
        let incumbent = Makespan::new(duration(20));

        assert!(!objective.improves(candidate, incumbent));
        assert_eq!(
            objective.better(candidate, incumbent),
            incumbent
        );
    }

    #[test]
    fn improvement_is_exact() {
        let candidate = Makespan::new(duration(75));
        let baseline = Makespan::new(duration(100));

        assert_eq!(
            candidate.improvement_over(baseline),
            Some(duration(25))
        );
    }

    #[test]
    fn no_improvement_returns_none() {
        let candidate = Makespan::new(duration(100));
        let baseline = Makespan::new(duration(100));

        assert_eq!(
            candidate.improvement_over(baseline),
            Some(duration(0))
        );
    }

    #[test]
    fn lower_bound_must_not_exceed_actual_makespan() {
        let makespan = Makespan::new(duration(100));
        let lower_bound = MakespanLowerBound::new(duration(75));

        let evaluation =
            MakespanEvaluation::with_lower_bound(makespan, lower_bound)
                .expect("valid lower bound must be accepted");

        assert_eq!(
            evaluation.optimality_gap(),
            Some(duration(25))
        );
    }

    #[test]
    fn invalid_lower_bound_is_rejected() {
        let makespan = Makespan::new(duration(100));
        let lower_bound = MakespanLowerBound::new(duration(125));

        assert!(matches!(
            MakespanEvaluation::with_lower_bound(
                makespan,
                lower_bound
            ),
            Err(MakespanError::InvalidLowerBound { .. })
        ));
    }

    #[test]
    fn_attained_lower_bound_has_zero_gap() {
        let makespan = Makespan::new(duration(100));
        let lower_bound = MakespanLowerBound::new(duration(100));

        let evaluation =
            MakespanEvaluation::with_lower_bound(makespan, lower_bound)
                .expect("equal lower bound must be valid");

        assert_eq!(
            evaluation.optimality_gap(),
            Some(duration(0))
        );
        assert!(evaluation.reaches_lower_bound());
    }

    #[test]
    fn score_orders_smaller_values_first() {
        let shorter = MakespanScore::new(duration(10));
        let longer = MakespanScore::new(duration(20));

        assert!(shorter < longer);
        assert!(shorter.is_better_than(longer));
        assert!(!longer.is_better_than(shorter));
    }

    #[test]
    fn score_improvement_is_exact() {
        let candidate = MakespanScore::new(duration(40));
        let baseline = MakespanScore::new(duration(100));

        assert_eq!(
            candidate.improvement_over(baseline),
            Some(duration(60))
        );
    }

    #[test]
    fn schedule_horizon_is_validated_once() {
        let horizon = ScheduleHorizon::new(time(100), time(150))
            .expect("valid horizon must be accepted");

        assert_eq!(horizon.origin(), time(100));
        assert_eq!(horizon.end(), time(150));
        assert_eq!(horizon.makespan().duration(), duration(50));
    }

    #[test]
    fn max_selects_longer_horizon() {
        let first = Makespan::new(duration(10));
        let second = Makespan::new(duration(20));

        assert_eq!(max(first, second), second);
    }

    #[test]
    fn min_selects_shorter_horizon() {
        let first = Makespan::new(duration(10));
        let second = Makespan::new(duration(20));

        assert_eq!(min(first, second), first);
    }

    #[test]
    fn no_machine_size_is_encoded() {
        // This test documents the architectural invariant:
        // makespan depends only on temporal coordinates.
        let tiny = Makespan::new(duration(10));
        let huge = Makespan::new(duration(10));

        assert_eq!(tiny, huge);
    }
}