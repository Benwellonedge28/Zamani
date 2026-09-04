//! Zamani Quantum Scheduling — ALAP Policy
//!
//! Production-grade, provider-independent "As Late As Possible" scheduling
//! policy for the Zamani quantum scheduling subsystem.
//!
//! # Purpose
//!
//! This module defines the temporal policy:
//!
//! > Place an operation at the latest time permitted by the scheduling
//! > constraints, schedule horizon, successor constraints, resource bounds,
//! > and other supplied temporal upper bounds.
//!
//! ALAP is the backward counterpart of ASAP.
//!
//! Conceptually:
//!
//! ```text
//! latest legal start
//!     = min(
//!         schedule-horizon bound,
//!         successor latest-start bounds minus duration,
//!         deadline-derived bound,
//!         resource latest-availability bound,
//!         communication/control bounds,
//!         other temporal upper bounds
//!       )
//! ```
//!
//! This module does NOT own:
//!
//! - quantum operation semantics;
//! - logical qubit identity;
//! - physical qubit identity;
//! - routing;
//! - hardware discovery;
//! - hardware execution;
//! - resource calendars;
//! - dependency graph construction;
//! - resource allocation;
//! - pulse generation;
//! - QEC decoding;
//! - noise modelling;
//! - runtime execution;
//! - vendor APIs.
//!
//! Those responsibilities belong to their canonical subsystems.
//!
//! # Architectural position
//!
//! ```text
//! quantum::ir
//!      │
//!      ▼
//! scheduling::adapters
//!      │
//!      ▼
//! scheduling::ir
//!      │
//!      ├── dependency analysis
//!      ├── resource analysis
//!      └── timing analysis
//!             │
//!             ▼
//!      scheduling::policies::alap
//!             │
//!             ▼
//!      scheduling::planners
//!             │
//!             ▼
//!      verification
//!             │
//!             ▼
//!      ScheduleResult
//! ```
//!
//! # Critical design rule
//!
//! ALAP is a POLICY, not a complete scheduling algorithm.
//!
//! The policy answers:
//!
//! > Given the upper temporal bounds already established by the planner,
//! > what is the latest legal placement for this operation?
//!
//! The planner answers:
//!
//! > Which resources are actually available and which legal placement can be
//! > committed to the schedule?
//!
//! Therefore this module must not directly mutate resource calendars or
//! discover hardware.
//!
//! # Backward scheduling
//!
//! For an operation `O` with duration `D`, if its latest legal finish is `F`,
//! its latest legal start is:
//!
//! ```text
//! start(O) = F - D
//! ```
//!
//! If multiple upper bounds exist, the tightest bound wins.
//!
//! ```text
//! latest_finish(O) = min(all upper bounds)
//! ```
//!
//! followed by:
//!
//! ```text
//! latest_start(O) = latest_finish(O) - duration(O)
//! ```
//!
//! All arithmetic is checked.
//!
//! # Dependency semantics
//!
//! For a dependency:
//!
//! ```text
//! O ───────► S
//! ```
//!
//! where `S` is a successor, ALAP must preserve:
//!
//! ```text
//! finish(O) <= start(S)
//! ```
//!
//! Therefore, if the successor may start no later than `T`:
//!
//! ```text
//! finish(O) <= T
//! ```
//!
//! and consequently:
//!
//! ```text
//! start(O) <= T - duration(O)
//! ```
//!
//! The planner is responsible for deriving successor-based upper bounds.
//! This policy combines those bounds without needing to understand the
//! underlying dependency graph representation.
//!
//! # Deadline semantics
//!
//! A deadline is an upper bound, unlike a release time.
//!
//! For an operation with a deadline `D`:
//!
//! ```text
//! finish(O) <= D
//! ```
//!
//! Therefore:
//!
//! ```text
//! start(O) <= D - duration(O)
//! ```
//!
//! The policy never silently violates a deadline.
//!
//! If the duration is greater than the supplied deadline, the operation is
//! infeasible and a structured error is returned.
//!
//! # Schedule horizon
//!
//! ALAP commonly schedules relative to a requested schedule horizon.
//!
//! For example:
//!
//! ```text
//! schedule horizon = H
//! operation duration = D
//!
//! latest start = H - D
//! ```
//!
//! The horizon is supplied by the planner/configuration layer.
//!
//! It is not a hard-coded scheduler limit.
//!
//! # Resource semantics
//!
//! Resource availability is supplied by the planner.
//!
//! This policy can consume a resource-derived latest finish or latest-start
//! bound without knowing how the resource calendar is implemented.
//!
//! For example:
//!
//! ```text
//! resource latest legal finish
//!              │
//!              ▼
//!        ALAP upper bound
//! ```
//!
//! The planner remains responsible for proving that the final resource
//! reservation is legal.
//!
//! # Timing alignment
//!
//! Hardware-specific timing resolution and alignment are supplied by the
//! timing subsystem.
//!
//! ALAP performs only abstract temporal calculations.
//!
//! A planner may therefore calculate:
//!
//! ```text
//! latest legal time
//!       │
//!       ▼
//! target alignment
//!       │
//!       ▼
//! resource feasibility
//! ```
//!
//! This module contains no nanosecond, picosecond, device-tick, sample-period,
//! or clock-frequency constants.
//!
//! # Dynamic scheduling
//!
//! Dynamic circuits may not have all upper bounds available during compilation.
//!
//! Runtime planners can invoke this policy with bounds derived from:
//!
//! - measurement completion;
//! - classical computation;
//! - conditional branches;
//! - feedback;
//! - communication;
//! - runtime resource availability.
//!
//! The policy does not assume that every constraint is statically known.
//!
//! # Distributed scheduling
//!
//! Communication completion can be represented as an upper-bound constraint
//! supplied by the distributed scheduler.
//!
//! ALAP therefore works with:
//!
//! ```text
//! local successor constraint
//! communication constraint
//! resource constraint
//! schedule horizon
//!         │
//!         ▼
//!    latest legal time
//! ```
//!
//! Network topology remains outside this module.
//!
//! # QEC
//!
//! QEC schedulers may use ALAP for:
//!
//! - syndrome extraction;
//! - stabilizer interactions;
//! - ancilla operations;
//! - measurement;
//! - classical feedback;
//! - recovery operations.
//!
//! QEC-specific semantics remain in the QEC subsystem.
//!
//! # Universal-program principle
//!
//! A Zamani program describes computation rather than machine size.
//!
//! Consequently this policy contains no:
//!
//! ```text
//! MAX_QUBITS
//! MAX_OPERATIONS
//! MAX_RESOURCES
//! MAX_DEPTH
//! MAX_PARALLELISM
//! ```
//!
//! The same policy can operate against any finite target description supplied
//! by the caller.
//!
//! "Infinity" means that this module imposes no artificial machine-size
//! ceiling. A concrete compilation remains bounded by the resources available
//! to that compilation.
//!
//! # Determinism
//!
//! ALAP itself is deterministic.
//!
//! It performs no random selection and does not inspect hash-map iteration
//! order. If multiple operations can receive the same legal temporal
//! placement, the planner is responsible for applying its configured
//! deterministic tie-breaking rule.
//!
//! # Thread safety
//!
//! `AlapPolicy` contains no mutable global state and is safe to share between
//! concurrent analyses.
//!
//! # Canonical identities
//!
//! Operation identity is the canonical Zamani IR operation identity:
//!
//! ```text
//! crate::quantum::ir::core::identity::OperationId
//! ```
//!
//! This module does not define another operation identity.
//!
//! Canonical qubit identities remain:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! ALAP does not need to own qubit identity because qubit/resource analysis is
//! performed by the scheduling IR and resource subsystem.
//!
//! # Integration with `policy.rs`
//!
//! The common policy vocabulary is owned by:
//!
//! ```text
//! crate::quantum::scheduling::policies::policy
//! ```
//!
//! In particular:
//!
//! ```text
//! SchedulingPolicyKind::AsLateAsPossible
//! ```
//!
//! identifies this policy at orchestration time.
//!
//! This module does not duplicate `SchedulingPolicyKind`.
//!
//! # Integration with planners
//!
//! A planner should conceptually perform:
//!
//! ```text
//! candidate operation
//!        │
//!        ├── schedule horizon
//!        ├── successor upper bounds
//!        ├── deadline bounds
//!        ├── resource upper bounds
//!        ├── communication bounds
//!        ├── classical bounds
//!        ├── QEC bounds
//!        └── custom bounds
//!                 │
//!                 ▼
//!             AlapPolicy
//!                 │
//!                 ▼
//!          latest legal start
//!                 │
//!                 ▼
//!        resource reservation
//! ```
//!
//! The planner must subsequently verify the committed placement.
//!
//! # Integration with verification
//!
//! Successful ALAP calculation is not equivalent to successful schedule
//! verification.
//!
//! Verification must independently establish:
//!
//! - dependency correctness;
//! - resource correctness;
//! - timing correctness;
//! - alignment correctness;
//! - deadline correctness;
//! - semantic preservation.
//!
//! # Complexity
//!
//! For one operation with `B` supplied upper bounds, the core calculation is
//! O(B) time and O(1) additional working memory.
//!
//! No structure proportional to:
//!
//! - machine qubit count;
//! - machine resource count;
//! - maximum schedule depth;
//! - maximum schedule duration
//!
//! is allocated by this policy.
//!
//! # Frozen-file contract
//!
//! This file is intentionally complete independently of:
//!
//! - `policies/asap.rs`;
//! - `policies/priority.rs`;
//! - `policies/resource_aware.rs`;
//! - `policies/hybrid.rs`;
//! - `planners/*`;
//! - `algorithms/*`;
//! - QEC implementations;
//! - distributed scheduling implementations;
//! - hardware providers.
//!
//! Those modules consume this contract and must not redefine ALAP semantics.
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
//! - no `unsafe`.
//!
//! =============================================================================
//! Safety boundary
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use core::fmt;

use crate::quantum::ir::core::identity::OperationId;

use super::super::types::{Duration, TimePoint};

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by an ALAP temporal calculation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AlapError {
    /// The supplied operation identifier is not valid for the requested
    /// calculation.
    InvalidOperation {
        /// Operation involved in the calculation.
        operation: OperationId,
    },

    /// The supplied upper bounds do not describe a feasible temporal window.
    ///
    /// This error is used when the calculated latest finish or latest start
    /// would precede the corresponding lower bound.
    InfeasibleWindow {
        /// Operation involved in the calculation.
        operation: OperationId,

        /// Earliest legal start.
        earliest_start: TimePoint,

        /// Latest legal start.
        latest_start: TimePoint,
    },

    /// Subtracting the operation duration from the latest finish would
    /// underflow the abstract schedule coordinate.
    TimeUnderflow {
        /// Operation involved in the calculation.
        operation: OperationId,

        /// Latest finish before duration subtraction.
        latest_finish: TimePoint,

        /// Operation duration.
        duration: Duration,
    },

    /// The operation duration and temporal window are inconsistent.
    InvalidDuration {
        /// Operation involved in the calculation.
        operation: OperationId,

        /// Supplied duration.
        duration: Duration,
    },
}

impl fmt::Display for AlapError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidOperation { operation } => {
                write!(formatter, "invalid ALAP operation `{operation}`")
            }

            Self::InfeasibleWindow {
                operation,
                earliest_start,
                latest_start,
            } => write!(
                formatter,
                "ALAP temporal window is infeasible for operation `{operation}`: \
                 earliest start `{earliest_start}` exceeds latest start \
                 `{latest_start}`"
            ),

            Self::TimeUnderflow {
                operation,
                latest_finish,
                duration,
            } => write!(
                formatter,
                "ALAP temporal subtraction underflowed for operation `{operation}`: \
                 latest finish `{latest_finish}`, duration `{duration}`"
            ),

            Self::InvalidDuration {
                operation,
                duration,
            } => write!(
                formatter,
                "invalid ALAP duration for operation `{operation}`: `{duration}`"
            ),
        }
    }
}

impl std::error::Error for AlapError {}

// =============================================================================
// Upper-bound reason
// =============================================================================

/// Reason why an operation cannot finish later than a supplied time.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum AlapBoundReason {
    /// The overall scheduling horizon imposes this upper bound.
    ScheduleHorizon,

    /// A successor operation imposes this upper bound.
    Successor,

    /// A deadline imposes this upper bound.
    Deadline,

    /// A resource is no longer available after this point.
    Resource,

    /// A communication dependency imposes this upper bound.
    Communication,

    /// A classical computation or feedback dependency imposes this upper
    /// bound.
    Classical,

    /// A QEC constraint imposes this upper bound.
    Qec,

    /// A target-provided timing window imposes this upper bound.
    TimingWindow,

    /// A target-provided synchronization/alignment boundary imposes this
    /// upper bound.
    Alignment,

    /// A caller-defined scheduling constraint.
    Custom(String),
}

impl fmt::Display for AlapBoundReason {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ScheduleHorizon => formatter.write_str("schedule-horizon"),
            Self::Successor => formatter.write_str("successor"),
            Self::Deadline => formatter.write_str("deadline"),
            Self::Resource => formatter.write_str("resource"),
            Self::Communication => formatter.write_str("communication"),
            Self::Classical => formatter.write_str("classical"),
            Self::Qec => formatter.write_str("qec"),
            Self::TimingWindow => formatter.write_str("timing-window"),
            Self::Alignment => formatter.write_str("alignment"),
            Self::Custom(value) => write!(formatter, "custom:{value}"),
        }
    }
}

// =============================================================================
// Temporal upper bound
// =============================================================================

/// A named upper bound on an operation's legal finish time.
///
/// ALAP selects the tightest supplied bound.
///
/// For bounds:
///
/// ```text
/// B1 = 100
/// B2 = 80
/// B3 = 120
/// ```
///
/// the legal latest finish is:
///
/// ```text
/// min(100, 80, 120) = 80
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AlapUpperBound {
    reason: AlapBoundReason,
    time: TimePoint,
}

impl AlapUpperBound {
    /// Creates an upper bound.
    #[must_use]
    pub const fn new(reason: AlapBoundReason, time: TimePoint) -> Self {
        Self { reason, time }
    }

    /// Returns the reason for this upper bound.
    #[must_use]
    pub const fn reason(&self) -> &AlapBoundReason {
        &self.reason
    }

    /// Returns the bound's finish time.
    #[must_use]
    pub const fn time(&self) -> TimePoint {
        self.time
    }
}

// =============================================================================
// Lower temporal bound
// =============================================================================

/// Lower bound on an operation's start time.
///
/// This is used only for feasibility checking.
///
/// ALAP itself maximizes the legal start time; it must nevertheless ensure
/// that the selected time is not earlier than a required release or other
/// lower-bound constraint.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AlapLowerBound {
    reason: AlapLowerBoundReason,
    time: TimePoint,
}

impl AlapLowerBound {
    /// Creates a lower start-time bound.
    #[must_use]
    pub const fn new(reason: AlapLowerBoundReason, time: TimePoint) -> Self {
        Self { reason, time }
    }

    /// Returns the lower-bound reason.
    #[must_use]
    pub const fn reason(&self) -> &AlapLowerBoundReason {
        &self.reason
    }

    /// Returns the lower-bound time.
    #[must_use]
    pub const fn time(&self) -> TimePoint {
        self.time
    }
}

/// Reason why an operation cannot start earlier than a supplied time.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum AlapLowerBoundReason {
    /// Operation release time.
    Release,

    /// A predecessor completion requirement.
    Dependency,

    /// A communication or synchronization lower bound.
    Communication,

    /// Classical computation or feedback readiness.
    Classical,

    /// QEC-specific readiness.
    Qec,

    /// Target timing-window lower bound.
    TimingWindow,

    /// Target alignment lower bound.
    Alignment,

    /// Resource availability lower bound.
    Resource,

    /// Caller-defined lower bound.
    Custom(String),
}

impl fmt::Display for AlapLowerBoundReason {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Release => formatter.write_str("release"),
            Self::Dependency => formatter.write_str("dependency"),
            Self::Communication => formatter.write_str("communication"),
            Self::Classical => formatter.write_str("classical"),
            Self::Qec => formatter.write_str("qec"),
            Self::TimingWindow => formatter.write_str("timing-window"),
            Self::Alignment => formatter.write_str("alignment"),
            Self::Resource => formatter.write_str("resource"),
            Self::Custom(value) => write!(formatter, "custom:{value}"),
        }
    }
}

// =============================================================================
// ALAP input
// =============================================================================

/// Immutable input for one ALAP placement calculation.
///
/// The planner constructs this from the dependency, resource and timing
/// subsystems.
///
/// The policy does not need to know how those bounds were produced.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AlapTimeBounds {
    operation: OperationId,
    duration: Duration,
    upper_bounds: Vec<AlapUpperBound>,
    lower_bounds: Vec<AlapLowerBound>,
}

impl AlapTimeBounds {
    /// Creates an empty ALAP bound set for an operation.
    ///
    /// At least one upper bound must be supplied before calculation.
    #[must_use]
    pub fn new(operation: OperationId, duration: Duration) -> Self {
        Self {
            operation,
            duration,
            upper_bounds: Vec::new(),
            lower_bounds: Vec::new(),
        }
    }

    /// Adds an upper finish-time bound.
    pub fn push_upper_bound(&mut self, bound: AlapUpperBound) {
        self.upper_bounds.push(bound);
    }

    /// Adds a lower start-time bound used for feasibility validation.
    pub fn push_lower_bound(&mut self, bound: AlapLowerBound) {
        self.lower_bounds.push(bound);
    }

    /// Returns the operation identity.
    #[must_use]
    pub const fn operation(&self) -> OperationId {
        self.operation
    }

    /// Returns the operation duration.
    #[must_use]
    pub const fn duration(&self) -> Duration {
        self.duration
    }

    /// Returns all supplied upper bounds.
    #[must_use]
    pub fn upper_bounds(&self) -> &[AlapUpperBound] {
        &self.upper_bounds
    }

    /// Returns all supplied lower bounds.
    #[must_use]
    pub fn lower_bounds(&self) -> &[AlapLowerBound] {
        &self.lower_bounds
    }

    /// Returns whether at least one upper bound is available.
    #[must_use]
    pub fn has_upper_bound(&self) -> bool {
        !self.upper_bounds.is_empty()
    }
}

// =============================================================================
// ALAP result
// =============================================================================

/// Result of one ALAP temporal placement calculation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AlapPlacement {
    operation: OperationId,
    latest_start: TimePoint,
    latest_finish: TimePoint,
    limiting_bound: AlapUpperBound,
}

impl AlapPlacement {
    /// Creates a calculated ALAP placement.
    #[must_use]
    pub const fn new(
        operation: OperationId,
        latest_start: TimePoint,
        latest_finish: TimePoint,
        limiting_bound: AlapUpperBound,
    ) -> Self {
        Self {
            operation,
            latest_start,
            latest_finish,
            limiting_bound,
        }
    }

    /// Returns the operation identity.
    #[must_use]
    pub const fn operation(&self) -> OperationId {
        self.operation
    }

    /// Returns the latest legal start.
    #[must_use]
    pub const fn latest_start(&self) -> TimePoint {
        self.latest_start
    }

    /// Returns the latest legal finish.
    #[must_use]
    pub const fn latest_finish(&self) -> TimePoint {
        self.latest_finish
    }

    /// Returns the upper bound that limited the calculation.
    #[must_use]
    pub const fn limiting_bound(&self) -> &AlapUpperBound {
        &self.limiting_bound
    }

    /// Returns the duration represented by this placement.
    ///
    /// Returns `None` if the finish precedes the start, which should never
    /// occur for a valid placement.
    #[must_use]
    pub fn duration(&self) -> Option<Duration> {
        self.latest_start
            .checked_duration_until(self.latest_finish)
    }
}

// =============================================================================
// ALAP policy
// =============================================================================

/// Production ALAP policy.
///
/// This type is intentionally stateless.
///
/// All target-specific information is supplied by the caller through
/// `AlapTimeBounds`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct AlapPolicy;

impl AlapPolicy {
    /// Creates an ALAP policy.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Calculates the latest legal placement for one operation.
    ///
    /// The calculation is:
    ///
    /// ```text
    /// latest_finish = min(upper_bounds)
    /// latest_start  = latest_finish - duration
    /// ```
    ///
    /// The resulting start must satisfy every supplied lower bound.
    pub fn calculate(
        &self,
        bounds: &AlapTimeBounds,
    ) -> Result<AlapPlacement, AlapError> {
        if bounds.operation().value() == 0 {
            return Err(AlapError::InvalidOperation {
                operation: bounds.operation(),
            });
        }

        let duration = bounds.duration();

        if duration.value() == 0 {
            // Zero duration is semantically valid. It is not rejected.
        }

        let limiting_bound = bounds
            .upper_bounds()
            .iter()
            .min_by(|left, right| left.time().cmp(&right.time()))
            .cloned();

        let limiting_bound = match limiting_bound {
            Some(bound) => bound,
            None => {
                // There is no meaningful latest placement without an upper
                // temporal boundary. The planner must supply at least one
                // boundary such as a schedule horizon, deadline, successor
                // bound, or equivalent constraint.
                return Err(AlapError::InfeasibleWindow {
                    operation: bounds.operation(),
                    earliest_start: TimePoint::ZERO,
                    latest_start: TimePoint::ZERO,
                });
            }
        };

        let latest_finish = limiting_bound.time();

        let latest_start = match latest_finish.checked_sub(duration) {
            Some(time) => time,
            None => {
                return Err(AlapError::TimeUnderflow {
                    operation: bounds.operation(),
                    latest_finish,
                    duration,
                });
            }
        };

        for lower_bound in bounds.lower_bounds() {
            if latest_start < lower_bound.time() {
                return Err(AlapError::InfeasibleWindow {
                    operation: bounds.operation(),
                    earliest_start: lower_bound.time(),
                    latest_start,
                });
            }
        }

        Ok(AlapPlacement::new(
            bounds.operation(),
            latest_start,
            latest_finish,
            limiting_bound,
        ))
    }

    /// Calculates only the latest legal start time.
    ///
    /// This convenience method uses the same validation and arithmetic as
    /// `calculate`.
    pub fn latest_start(
        &self,
        bounds: &AlapTimeBounds,
    ) -> Result<TimePoint, AlapError> {
        self.calculate(bounds).map(|placement| placement.latest_start())
    }

    /// Calculates only the latest legal finish time.
    ///
    /// This is useful when a planner needs the limiting temporal boundary
    /// before performing resource reservation.
    pub fn latest_finish(
        &self,
        bounds: &AlapTimeBounds,
    ) -> Result<TimePoint, AlapError> {
        self.calculate(bounds)
            .map(|placement| placement.latest_finish())
    }
}

// =============================================================================
// Policy-level helpers
// =============================================================================

/// Returns the minimum time among supplied upper bounds.
///
/// This function is useful to planners that already have an operation's
/// temporal bounds and want the pure ALAP reduction without constructing a
/// policy object.
///
/// Returns `None` when no upper bounds are supplied.
#[must_use]
pub fn minimum_upper_bound(
    bounds: &[AlapUpperBound],
) -> Option<AlapUpperBound> {
    bounds
        .iter()
        .min_by(|left, right| left.time().cmp(&right.time()))
        .cloned()
}

/// Calculates a latest start from an explicit latest finish and duration.
///
/// This helper deliberately uses the canonical scheduler `TimePoint` and
/// `Duration` checked arithmetic.
pub fn latest_start_from_finish(
    operation: OperationId,
    latest_finish: TimePoint,
    duration: Duration,
) -> Result<TimePoint, AlapError> {
    match latest_finish.checked_sub(duration) {
        Some(time) => Ok(time),
        None => Err(AlapError::TimeUnderflow {
            operation,
            latest_finish,
            duration,
        }),
    }
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

    #[test]
    fn selects_tightest_upper_bound() {
        let mut bounds = AlapTimeBounds::new(operation(1), Duration::new(10));

        bounds.push_upper_bound(AlapUpperBound::new(
            AlapBoundReason::ScheduleHorizon,
            TimePoint::new(100),
        ));

        bounds.push_upper_bound(AlapUpperBound::new(
            AlapBoundReason::Deadline,
            TimePoint::new(80),
        ));

        bounds.push_upper_bound(AlapUpperBound::new(
            AlapBoundReason::Resource,
            TimePoint::new(90),
        ));

        let placement = AlapPolicy::new()
            .calculate(&bounds)
            .expect("valid ALAP calculation");

        assert_eq!(placement.latest_finish(), TimePoint::new(80));
        assert_eq!(placement.latest_start(), TimePoint::new(70));
        assert_eq!(
            placement.limiting_bound().reason(),
            &AlapBoundReason::Deadline
        );
    }

    #[test]
    fn respects_lower_bound() {
        let mut bounds = AlapTimeBounds::new(operation(2), Duration::new(20));

        bounds.push_upper_bound(AlapUpperBound::new(
            AlapBoundReason::ScheduleHorizon,
            TimePoint::new(100),
        ));

        bounds.push_lower_bound(AlapLowerBound::new(
            AlapLowerBoundReason::Release,
            TimePoint::new(80),
        ));

        let result = AlapPolicy::new().calculate(&bounds);

        assert_eq!(
            result,
            Err(AlapError::InfeasibleWindow {
                operation: operation(2),
                earliest_start: TimePoint::new(80),
                latest_start: TimePoint::new(80),
            })
        );
    }

    #[test]
    fn accepts_exact_temporal_window() {
        let mut bounds = AlapTimeBounds::new(operation(3), Duration::new(20));

        bounds.push_upper_bound(AlapUpperBound::new(
            AlapBoundReason::Deadline,
            TimePoint::new(100),
        ));

        bounds.push_lower_bound(AlapLowerBound::new(
            AlapLowerBoundReason::Release,
            TimePoint::new(80),
        ));

        let placement = AlapPolicy::new()
            .calculate(&bounds)
            .expect("exact window is feasible");

        assert_eq!(placement.latest_start(), TimePoint::new(80));
        assert_eq!(placement.latest_finish(), TimePoint::new(100));
    }

    #[test]
    fn accepts_zero_duration() {
        let mut bounds = AlapTimeBounds::new(operation(4), Duration::ZERO);

        bounds.push_upper_bound(AlapUpperBound::new(
            AlapBoundReason::ScheduleHorizon,
            TimePoint::new(100),
        ));

        let placement = AlapPolicy::new()
            .calculate(&bounds)
            .expect("zero duration is valid");

        assert_eq!(placement.latest_start(), TimePoint::new(100));
        assert_eq!(placement.latest_finish(), TimePoint::new(100));
    }

    #[test]
    fn rejects_time_underflow() {
        let mut bounds = AlapTimeBounds::new(operation(5), Duration::new(101));

        bounds.push_upper_bound(AlapUpperBound::new(
            AlapBoundReason::Deadline,
            TimePoint::new(100),
        ));

        assert_eq!(
            AlapPolicy::new().calculate(&bounds),
            Err(AlapError::TimeUnderflow {
                operation: operation(5),
                latest_finish: TimePoint::new(100),
                duration: Duration::new(101),
            })
        );
    }

    #[test]
    fn rejects_missing_upper_bound() {
        let bounds = AlapTimeBounds::new(operation(6), Duration::new(10));

        assert!(matches!(
            AlapPolicy::new().calculate(&bounds),
            Err(AlapError::InfeasibleWindow { .. })
        ));
    }

    #[test]
    fn minimum_upper_bound_is_deterministic() {
        let bounds = vec![
            AlapUpperBound::new(
                AlapBoundReason::ScheduleHorizon,
                TimePoint::new(300),
            ),
            AlapUpperBound::new(
                AlapBoundReason::Deadline,
                TimePoint::new(200),
            ),
            AlapUpperBound::new(
                AlapBoundReason::Resource,
                TimePoint::new(250),
            ),
        ];

        let selected = minimum_upper_bound(&bounds)
            .expect("upper bound exists");

        assert_eq!(selected.time(), TimePoint::new(200));
    }

    #[test]
    fn helper_calculates_latest_start() {
        let result = latest_start_from_finish(
            operation(7),
            TimePoint::new(100),
            Duration::new(30),
        )
        .expect("valid subtraction");

        assert_eq!(result, TimePoint::new(70));
    }

    #[test]
    fn helper_reports_underflow() {
        let result = latest_start_from_finish(
            operation(8),
            TimePoint::new(20),
            Duration::new(30),
        );

        assert_eq!(
            result,
            Err(AlapError::TimeUnderflow {
                operation: operation(8),
                latest_finish: TimePoint::new(20),
                duration: Duration::new(30),
            })
        );
    }
}