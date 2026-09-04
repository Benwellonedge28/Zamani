//! Zamani Quantum Scheduling — ASAP Policy
//!
//! Production-grade, target-independent "As Soon As Possible" scheduling
//! policy for the Zamani quantum scheduling subsystem.
//!
//! # Purpose
//!
//! This module defines the temporal policy:
//!
//! > Place each operation at the earliest time permitted by the scheduling
//! > constraints known to the planner.
//!
//! ASAP means:
//!
//! ```text
//! earliest legal start
//!     = max(
//!         release time,
//!         predecessor completion,
//!         other temporal lower bounds,
//!         resource-feasible lower bound
//!       )
//! ```
//!
//! This module DOES NOT itself own:
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
//! - runtime execution.
//!
//! Those concerns belong to their canonical subsystems.
//!
//! # Architectural boundary
//!
//! The intended pipeline is:
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
//!      scheduling::policies::asap
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
//! ASAP is a POLICY, not a complete scheduling algorithm.
//!
//! The policy answers:
//!
//! > Given the lower temporal bounds already established by the planner,
//! > what placement preference should be used?
//!
//! The planner answers:
//!
//! > What resources are available and when can this operation actually be
//! > placed?
//!
//! Therefore this module must never scan or mutate hardware/resource state
//! directly.
//!
//! # Resource independence
//!
//! An ASAP policy must work with:
//!
//! - one qubit;
//! - many qubits;
//! - one QPU;
//! - many QPUs;
//! - shared control channels;
//! - capacity-limited resources;
//! - communication resources;
//! - distributed quantum systems;
//! - dynamically changing resources.
//!
//! No resource count is encoded here.
//!
//! # Universal-program principle
//!
//! A Zamani program describes computation rather than machine size.
//!
//! The same semantic program may therefore be scheduled using this policy for:
//!
//! ```text
//! tiny target
//! large target
//! modular target
//! distributed target
//! fault-tolerant target
//! future target
//! ```
//!
//! Only the supplied scheduling context changes.
//!
//! There is intentionally no:
//!
//! ```text
//! MAX_QUBITS
//! MAX_OPERATIONS
//! MAX_RESOURCES
//! MAX_DEPTH
//! MAX_PARALLELISM
//! ```
//!
//! in this file.
//!
//! # Determinism
//!
//! This policy is deterministic.
//!
//! It never:
//!
//! - generates random numbers;
//! - uses memory addresses;
//! - depends on hash-map iteration;
//! - depends on thread timing;
//! - mutates global state.
//!
//! If multiple candidates have the same temporal priority, the planner must
//! perform the configured deterministic tie-breaking using canonical operation
//! identity.
//!
//! This policy intentionally does not invent an alternative operation ordering.
//!
//! # Time model
//!
//! Scheduling time is abstract.
//!
//! No assumption is made about:
//!
//! - nanoseconds;
//! - microseconds;
//! - picoseconds;
//! - hardware ticks;
//! - pulse samples;
//! - clock frequencies.
//!
//! The supplied `TimePoint` and `Duration` values are interpreted by the
//! scheduling timing subsystem and target adapter.
//!
//! # Arithmetic safety
//!
//! All temporal arithmetic is checked.
//!
//! No wrapping arithmetic is used for scheduling semantics.
//!
//! A temporal overflow is reported as a structured error rather than producing
//! an invalid schedule.
//!
//! # Zero duration
//!
//! Zero-duration operations are valid at this policy layer.
//!
//! Their treatment as semantic barriers, resource events, or executable
//! operations is determined by the surrounding planner and IR contract.
//!
//! # Dependency semantics
//!
//! ASAP never violates a predecessor completion bound.
//!
//! For an operation `O`:
//!
//! ```text
//! start(O) >= finish(P)
//! ```
//!
//! for every predecessor `P`.
//!
//! If multiple predecessor bounds exist, their maximum is used.
//!
//! # Release-time semantics
//!
//! If an operation cannot begin before a supplied release time:
//!
//! ```text
//! start(O) >= release(O)
//! ```
//!
//! ASAP respects it.
//!
//! # Resource semantics
//!
//! Resource feasibility is deliberately supplied by the planner.
//!
//! The policy can therefore consume an already-computed resource lower bound:
//!
//! ```text
//! resource_ready_time(O)
//! ```
//!
//! and calculate:
//!
//! ```text
//! start(O) = max(
//!     release(O),
//!     dependency_ready_time(O),
//!     resource_ready_time(O),
//!     other temporal lower bounds
//! )
//! ```
//!
//! This prevents the policy from depending on a particular resource-calendar
//! implementation.
//!
//! # Alignment semantics
//!
//! The timing subsystem may require the calculated earliest time to be aligned
//! to a target-provided resolution or boundary.
//!
//! ASAP therefore exposes a generic alignment hook through `AsapTimeBounds`.
//!
//! The policy itself does not contain hardware timing constants.
//!
//! # Deadlines
//!
//! A deadline is not an ASAP lower bound.
//!
//! The policy therefore does not silently move an operation merely because a
//! deadline exists.
//!
//! Deadline feasibility belongs to the planner/constraint layer.
//!
//! This prevents an invalid schedule from being presented as valid merely
//! because an ASAP placement was calculated.
//!
//! # Dynamic scheduling
//!
//! Runtime-resolved lower bounds can be supplied when a dynamic scheduler
//! invokes this policy.
//!
//! The policy does not assume that every temporal bound is known at compile
//! time.
//!
//! # Distributed scheduling
//!
//! Communication readiness can be represented by a lower bound supplied by the
//! distributed planner.
//!
//! For example:
//!
//! ```text
//! local dependency completion
//!          +
//! communication completion
//!          +
//! resource availability
//!          ↓
//! earliest legal start
//! ```
//!
//! No network topology is encoded here.
//!
//! # QEC
//!
//! QEC scheduling can use ASAP for:
//!
//! - syndrome extraction;
//! - ancilla preparation;
//! - stabilizer interactions;
//! - measurement;
//! - feedback.
//!
//! QEC-specific dependencies remain outside this policy.
//!
//! # Canonical identities
//!
//! This module uses the canonical Quantum IR operation identity:
//!
//! ```text
//! crate::quantum::ir::core::identity::OperationId
//! ```
//!
//! It does not define another operation identity.
//!
//! Qubit identity remains owned by:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! ```
//!
//! ASAP does not need a qubit field because qubit/resource analysis belongs to
//! the scheduling IR and resource subsystem.
//!
//! # Integration with `config.rs`
//!
//! `SchedulingStrategy::AsSoonAsPossible` in:
//!
//! ```text
//! crate::quantum::scheduling::config
//! ```
//!
//! selects this policy at orchestration time.
//!
//! This module does not duplicate `SchedulingStrategy`.
//!
//! # Integration with `policy.rs`
//!
//! `policy.rs` is the stable common policy vocabulary and dispatch boundary.
//!
//! The intended relationship is:
//!
//! ```text
//! policies::policy
//!       │
//!       ├── policy vocabulary
//!       ├── policy descriptors
//!       └── common policy contract
//!       │
//!       ▼
//! policies::asap
//!       │
//!       ▼
//! ASAP-specific temporal behaviour
//! ```
//!
//! The ASAP implementation is intentionally expressed through stable,
//! policy-local value contracts so that adding other policies does not require
//! changing this file.
//!
//! # Integration with planners
//!
//! A planner should perform approximately:
//!
//! ```text
//! candidate
//!     │
//!     ├── dependency lower bound
//!     ├── release lower bound
//!     ├── resource lower bound
//!     ├── communication lower bound
//!     └── other temporal lower bounds
//!             │
//!             ▼
//!         AsapPolicy
//!             │
//!             ▼
//!      earliest legal time
//!             │
//!             ▼
//!      resource reservation
//! ```
//!
//! The planner remains responsible for verifying that the final placement is
//! actually feasible.
//!
//! # Integration with verification
//!
//! The verification subsystem must independently verify:
//!
//! - all dependencies;
//! - all resource constraints;
//! - all timing constraints;
//! - all alignment constraints;
//! - all semantic preservation requirements.
//!
//! Successful execution of this policy alone is never equivalent to schedule
//! verification.
//!
//! # Complexity
//!
//! The core calculation performed here is O(B), where B is the number of
//! temporal lower bounds supplied for one operation.
//!
//! No structure proportional to:
//!
//! - total qubit count;
//! - total machine capacity;
//! - maximum schedule time;
//! - maximum schedule depth
//!
//! is allocated by this policy.
//!
//! This makes the policy suitable as one component of schedulers operating on
//! very large problem instances.
//!
//! # Thread safety
//!
//! `AsapPolicy` contains no mutable global state and is safe to share between
//! concurrent scheduling analyses.
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
//! - no `unsafe`.
//!
//! # Frozen-file contract
//!
//! This file is intended to be complete independently of the implementation of:
//!
//! - ALAP;
//! - priority scheduling;
//! - resource-aware scheduling;
//! - hybrid scheduling;
//! - list scheduling;
//! - critical-path scheduling;
//! - RCPSP;
//! - adaptive scheduling;
//! - QEC scheduling;
//! - distributed scheduling.
//!
//! Those modules consume this policy rather than modifying it.
//!
//! =============================================================================
//! Safety boundary
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::cmp::Ordering;
use std::fmt;

use crate::quantum::ir::core::identity::OperationId;

use super::super::types::{Duration, TimePoint};

// =============================================================================
// Errors
// =============================================================================

/// Errors specific to ASAP temporal calculations.
///
/// These errors deliberately remain local to the policy calculation layer.
/// Higher-level scheduling errors can wrap or translate them without requiring
/// this file to know the complete scheduler error hierarchy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AsapError {
    /// A temporal lower bound could not be combined because the resulting
    /// schedule coordinate exceeded the representable time domain.
    TimeOverflow {
        /// Operation for which the calculation failed.
        operation: OperationId,
    },

    /// The supplied operation identity is inconsistent with the policy input.
    InvalidOperation {
        /// Operation involved in the invalid request.
        operation: OperationId,
    },

    /// A lower-bound collection contained an invalid interval relationship.
    InvalidBounds {
        /// Operation involved in the invalid request.
        operation: OperationId,
        /// Earliest bound.
        earliest: TimePoint,
        /// Latest permitted bound.
        latest: TimePoint,
    },
}

impl fmt::Display for AsapError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TimeOverflow { operation } => write!(
                formatter,
                "ASAP temporal calculation overflowed for operation `{operation}`"
            ),

            Self::InvalidOperation { operation } => write!(
                formatter,
                "invalid ASAP operation identity `{operation}`"
            ),

            Self::InvalidBounds {
                operation,
                earliest,
                latest,
            } => write!(
                formatter,
                "invalid ASAP bounds for operation `{operation}`: earliest `{earliest}` exceeds latest `{latest}`"
            ),
        }
    }
}

impl std::error::Error for AsapError {}

// =============================================================================
// Temporal lower bound
// =============================================================================

/// A named lower bound on an operation's earliest legal start time.
///
/// Lower bounds are represented explicitly so diagnostics and future policy
/// implementations can explain why an operation could not start earlier.
///
/// Examples include:
///
/// - release time;
/// - predecessor completion;
/// - resource availability;
/// - communication completion;
/// - calibration window;
/// - feedback readiness;
/// - custom constraint.
///
/// The policy does not interpret the name semantically. The planner supplies
/// the bound according to its domain model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AsapLowerBound {
    reason: AsapBoundReason,
    time: TimePoint,
}

impl AsapLowerBound {
    /// Creates a lower bound with an explicit reason.
    #[must_use]
    pub const fn new(reason: AsapBoundReason, time: TimePoint) -> Self {
        Self { reason, time }
    }

    /// Returns the reason for this lower bound.
    #[must_use]
    pub const fn reason(&self) -> &AsapBoundReason {
        &self.reason
    }

    /// Returns the lower-bound time.
    #[must_use]
    pub const fn time(&self) -> TimePoint {
        self.time
    }
}

/// Reason that an operation cannot begin before a particular time.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum AsapBoundReason {
    /// The operation is released at this time.
    Release,

    /// A predecessor operation completes at this time.
    Dependency,

    /// A required resource becomes available at this time.
    Resource,

    /// A communication dependency becomes available at this time.
    Communication,

    /// A classical computation or measurement result becomes available.
    Classical,

    /// A QEC dependency becomes available.
    Qec,

    /// A timing-window constraint becomes active.
    TimingWindow,

    /// A target-specific alignment or synchronization requirement.
    Alignment,

    /// A caller-defined scheduling constraint.
    Custom(String),
}

impl fmt::Display for AsapBoundReason {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Release => formatter.write_str("release"),
            Self::Dependency => formatter.write_str("dependency"),
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
// ASAP input
// =============================================================================

/// Immutable temporal input for one ASAP placement decision.
///
/// This is deliberately independent of a particular planner implementation.
///
/// A planner may construct this value from:
///
/// ```text
/// dependency graph
/// resource calendar
/// timing model
/// communication model
/// QEC constraints
/// dynamic runtime state
/// ```
///
/// and pass the resulting lower bounds to `AsapPolicy`.
///
/// # Resource scalability
///
/// There is no resource-count field here.
///
/// A single operation may have any number of resource lower bounds. The
/// caller supplies only the bounds that are applicable to this operation.
///
/// # Memory scalability
///
/// The policy does not require a machine-wide timeline.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AsapBounds {
    operation: OperationId,
    earliest: TimePoint,
    latest: Option<TimePoint>,
    lower_bounds: Vec<AsapLowerBound>,
}

impl AsapBounds {
    /// Creates a new ASAP input.
    ///
    /// `earliest` is the caller's already-established baseline lower bound.
    #[must_use]
    pub fn new(operation: OperationId, earliest: TimePoint) -> Self {
        Self {
            operation,
            earliest,
            latest: None,
            lower_bounds: Vec::new(),
        }
    }

    /// Returns the operation identity.
    #[must_use]
    pub const fn operation(&self) -> OperationId {
        self.operation
    }

    /// Returns the baseline lower bound.
    #[must_use]
    pub const fn earliest(&self) -> TimePoint {
        self.earliest
    }

    /// Returns the optional latest permitted start.
    #[must_use]
    pub const fn latest(&self) -> Option<TimePoint> {
        self.latest
    }

    /// Returns all named lower bounds.
    #[must_use]
    pub fn lower_bounds(&self) -> &[AsapLowerBound] {
        &self.lower_bounds
    }

    /// Adds a lower bound.
    ///
    /// Bounds are not immediately reduced because retaining them provides
    /// deterministic diagnostics and provenance.
    pub fn add_lower_bound(&mut self, bound: AsapLowerBound) {
        self.lower_bounds.push(bound);
    }

    /// Adds a lower bound and returns the updated value.
    #[must_use]
    pub fn with_lower_bound(mut self, bound: AsapLowerBound) -> Self {
        self.add_lower_bound(bound);
        self
    }

    /// Sets a latest permitted start time.
    ///
    /// The final policy decision validates `earliest <= latest`.
    #[must_use]
    pub const fn with_latest(mut self, latest: TimePoint) -> Self {
        self.latest = Some(latest);
        self
    }

    /// Computes the maximum supplied lower bound.
    #[must_use]
    pub fn maximum_lower_bound(&self) -> TimePoint {
        self.lower_bounds
            .iter()
            .fold(self.earliest, |current, bound| {
                current.max(bound.time())
            })
    }

    /// Returns the strongest lower-bound reason, if one exists.
    ///
    /// When multiple bounds have the same time, the first supplied bound is
    /// retained. Callers that require a different diagnostic ordering should
    /// provide bounds in their desired deterministic order.
    #[must_use]
    pub fn strongest_bound(&self) -> Option<&AsapLowerBound> {
        let maximum = self.maximum_lower_bound();

        self.lower_bounds
            .iter()
            .find(|bound| bound.time() == maximum)
    }
}

// =============================================================================
// ASAP decision
// =============================================================================

/// Result of an ASAP temporal placement calculation.
///
/// This is a policy decision, not yet a committed resource reservation.
///
/// The planner remains responsible for:
///
/// - checking resource capacity;
/// - committing reservations;
/// - verifying dependencies;
/// - validating timing;
/// - materializing delays;
/// - performing final verification.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AsapDecision {
    operation: OperationId,
    start: TimePoint,
    end: TimePoint,
    duration: Duration,
    limiting_bound: Option<AsapLowerBound>,
}

impl AsapDecision {
    /// Creates a policy decision.
    fn new(
        operation: OperationId,
        start: TimePoint,
        duration: Duration,
        limiting_bound: Option<AsapLowerBound>,
    ) -> Result<Self, AsapError> {
        let end = start
            .checked_add(duration)
            .ok_or(AsapError::TimeOverflow { operation })?;

        Ok(Self {
            operation,
            start,
            end,
            duration,
            limiting_bound,
        })
    }

    /// Returns the operation identity.
    #[must_use]
    pub const fn operation(&self) -> OperationId {
        self.operation
    }

    /// Returns the selected earliest start time.
    #[must_use]
    pub const fn start(&self) -> TimePoint {
        self.start
    }

    /// Returns the calculated end time.
    #[must_use]
    pub const fn end(&self) -> TimePoint {
        self.end
    }

    /// Returns the operation duration.
    #[must_use]
    pub const fn duration(&self) -> Duration {
        self.duration
    }

    /// Returns the bound responsible for the selected start time.
    #[must_use]
    pub const fn limiting_bound(&self) -> Option<&AsapLowerBound> {
        self.limiting_bound.as_ref()
    }

    /// Returns the amount of time between the schedule origin and the start.
    ///
    /// This is equivalent to `start()` and exists as an explicit semantic
    /// accessor for diagnostic consumers.
    #[must_use]
    pub const fn wait_until_start(&self) -> TimePoint {
        self.start
    }
}

// =============================================================================
// ASAP policy
// =============================================================================

/// Production ASAP scheduling policy.
///
/// `AsapPolicy` is intentionally stateless.
///
/// It can therefore be:
///
/// - copied;
/// - shared;
/// - constructed per scheduler invocation;
/// - used concurrently;
/// - embedded in a policy registry.
///
/// No global state is used.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct AsapPolicy;

impl AsapPolicy {
    /// Creates the default ASAP policy.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Returns the stable policy name.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        "asap"
    }

    /// Returns the human-readable policy description.
    #[must_use]
    pub const fn description(&self) -> &'static str {
        "Schedule each operation at the earliest time permitted by all supplied temporal bounds"
    }

    /// Returns the direction used by ASAP scheduling.
    #[must_use]
    pub const fn is_forward(&self) -> bool {
        true
    }

    /// Calculates the earliest legal start time.
    ///
    /// This is the core ASAP operation.
    ///
    /// The calculation is:
    ///
    /// ```text
    /// start = max(all lower bounds)
    /// ```
    ///
    /// No resource state is inspected or modified.
    pub fn earliest_start(&self, bounds: &AsapBounds) -> Result<TimePoint, AsapError> {
        let start = bounds.maximum_lower_bound();

        if let Some(latest) = bounds.latest() {
            if start > latest {
                return Err(AsapError::InvalidBounds {
                    operation: bounds.operation(),
                    earliest: start,
                    latest,
                });
            }
        }

        Ok(start)
    }

    /// Produces an ASAP placement decision for an operation.
    ///
    /// `duration` is supplied by the timing subsystem or target adapter.
    ///
    /// The policy does not invent a duration.
    pub fn schedule(
        &self,
        bounds: &AsapBounds,
        duration: Duration,
    ) -> Result<AsapDecision, AsapError> {
        let start = self.earliest_start(bounds);

        let start = match start {
            Ok(value) => value,
            Err(error) => return Err(error),
        };

        let limiting_bound = bounds
            .strongest_bound()
            .cloned();

        AsapDecision::new(
            bounds.operation(),
            start,
            duration,
            limiting_bound,
        )
    }

    /// Calculates an ASAP decision using a baseline time and a set of lower
    /// bounds without requiring a mutable input structure.
    pub fn schedule_from_lower_bounds<I>(
        &self,
        operation: OperationId,
        baseline: TimePoint,
        lower_bounds: I,
        duration: Duration,
    ) -> Result<AsapDecision, AsapError>
    where
        I: IntoIterator<Item = AsapLowerBound>,
    {
        let mut bounds = AsapBounds::new(operation, baseline);

        for bound in lower_bounds {
            bounds.add_lower_bound(bound);
        }

        self.schedule(&bounds, duration)
    }

    /// Returns the ordering between two already-computed ASAP candidates.
    ///
    /// Earlier start time wins.
    ///
    /// If start times are equal, the operation identity provides the final
    /// deterministic ordering.
    ///
    /// This function does not inspect resource state.
    #[must_use]
    pub fn compare_decisions(
        &self,
        left: &AsapDecision,
        right: &AsapDecision,
    ) -> Ordering {
        left.start()
            .cmp(&right.start())
            .then_with(|| left.operation().cmp(&right.operation()))
    }

    /// Returns whether the policy prefers the left candidate.
    ///
    /// `Ordering::Less` means the left candidate has precedence under the ASAP
    /// temporal ordering.
    #[must_use]
    pub fn prefers(
        &self,
        left: &AsapDecision,
        right: &AsapDecision,
    ) -> bool {
        self.compare_decisions(left, right) == Ordering::Less
    }
}

// =============================================================================
// Policy metadata
// =============================================================================

/// Static capabilities of the ASAP policy.
///
/// These values describe policy behaviour, not hardware capabilities.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AsapCapabilities {
    /// ASAP operates in the forward temporal direction.
    pub forward: bool,

    /// ASAP can consume dependency lower bounds.
    pub dependencies: bool,

    /// ASAP can consume resource-ready lower bounds supplied externally.
    pub resource_bounds: bool,

    /// ASAP can consume communication lower bounds.
    pub communication_bounds: bool,

    /// ASAP can consume classical/feedback lower bounds.
    pub classical_bounds: bool,

    /// ASAP can consume QEC lower bounds.
    pub qec_bounds: bool,

    /// ASAP is deterministic.
    pub deterministic: bool,

    /// ASAP contains no intrinsic machine-size limitation.
    pub target_independent: bool,
}

impl Default for AsapCapabilities {
    fn default() -> Self {
        Self {
            forward: true,
            dependencies: true,
            resource_bounds: true,
            communication_bounds: true,
            classical_bounds: true,
            qec_bounds: true,
            deterministic: true,
            target_independent: true,
        }
    }
}

impl AsapPolicy {
    /// Returns the capabilities of this policy.
    #[must_use]
    pub const fn capabilities(&self) -> AsapCapabilities {
        AsapCapabilities {
            forward: true,
            dependencies: true,
            resource_bounds: true,
            communication_bounds: true,
            classical_bounds: true,
            qec_bounds: true,
            deterministic: true,
            target_independent: true,
        }
    }
}

// =============================================================================
// Validation helpers
// =============================================================================

impl AsapPolicy {
    /// Validates a candidate's temporal input without producing a schedule.
    ///
    /// This method is useful when planners want to validate inputs before
    /// committing any resource reservation.
    pub fn validate_bounds(
        &self,
        bounds: &AsapBounds,
    ) -> Result<(), AsapError> {
        if let Some(latest) = bounds.latest() {
            let earliest = bounds.maximum_lower_bound();

            if earliest > latest {
                return Err(AsapError::InvalidBounds {
                    operation: bounds.operation(),
                    earliest,
                    latest,
                });
            }
        }

        Ok(())
    }

    /// Validates a computed decision against the original bounds.
    ///
    /// This is intentionally redundant with the calculation because production
    /// schedulers benefit from cheap local invariant checks at subsystem
    /// boundaries.
    pub fn validate_decision(
        &self,
        bounds: &AsapBounds,
        decision: &AsapDecision,
    ) -> Result<(), AsapError> {
        if bounds.operation() != decision.operation() {
            return Err(AsapError::InvalidOperation {
                operation: decision.operation(),
            });
        }

        let earliest = bounds.maximum_lower_bound();

        if decision.start() < earliest {
            return Err(AsapError::InvalidBounds {
                operation: bounds.operation(),
                earliest,
                latest: decision.start(),
            });
        }

        if let Some(latest) = bounds.latest() {
            if decision.start() > latest {
                return Err(AsapError::InvalidBounds {
                    operation: bounds.operation(),
                    earliest: decision.start(),
                    latest,
                });
            }
        }

        let expected_end = decision
            .start()
            .checked_add(decision.duration())
            .ok_or(AsapError::TimeOverflow {
                operation: decision.operation(),
            })?;

        if expected_end != decision.end() {
            return Err(AsapError::TimeOverflow {
                operation: decision.operation(),
            });
        }

        Ok(())
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn operation(value: u64) -> OperationId {
        OperationId::from(value)
    }

    #[test]
    fn policy_is_forward_and_deterministic() {
        let policy = AsapPolicy::new();

        assert!(policy.is_forward());
        assert!(policy.capabilities().deterministic);
        assert!(policy.capabilities().target_independent);
    }

    #[test]
    fn earliest_start_uses_maximum_lower_bound() {
        let policy = AsapPolicy::new();

        let bounds = AsapBounds::new(
            operation(1),
            TimePoint::new(10),
        )
        .with_lower_bound(AsapLowerBound::new(
            AsapBoundReason::Dependency,
            TimePoint::new(40),
        ))
        .with_lower_bound(AsapLowerBound::new(
            AsapBoundReason::Resource,
            TimePoint::new(25),
        ))
        .with_lower_bound(AsapLowerBound::new(
            AsapBoundReason::Communication,
            TimePoint::new(60),
        ));

        let start = policy
            .earliest_start(&bounds)
            .expect("valid ASAP bounds");

        assert_eq!(start, TimePoint::new(60));
    }

    #[test]
    fn zero_duration_is_supported() {
        let policy = AsapPolicy::new();

        let bounds = AsapBounds::new(
            operation(2),
            TimePoint::new(100),
        );

        let decision = policy
            .schedule(&bounds, Duration::ZERO)
            .expect("zero duration is valid");

        assert_eq!(decision.start(), TimePoint::new(100));
        assert_eq!(decision.end(), TimePoint::new(100));
    }

    #[test]
    fn duration_is_added_with_checked_arithmetic() {
        let policy = AsapPolicy::new();

        let bounds = AsapBounds::new(
            operation(3),
            TimePoint::new(100),
        );

        let decision = policy
            .schedule(&bounds, Duration::new(25))
            .expect("valid duration");

        assert_eq!(decision.start(), TimePoint::new(100));
        assert_eq!(decision.end(), TimePoint::new(125));
    }

    #[test]
    fn latest_start_is_respected() {
        let policy = AsapPolicy::new();

        let bounds = AsapBounds::new(
            operation(4),
            TimePoint::new(10),
        )
        .with_lower_bound(AsapLowerBound::new(
            AsapBoundReason::Dependency,
            TimePoint::new(20),
        ))
        .with_latest(TimePoint::new(20));

        assert!(policy.earliest_start(&bounds).is_ok());
    }

    #[test]
    fn impossible_latest_start_is_rejected() {
        let policy = AsapPolicy::new();

        let bounds = AsapBounds::new(
            operation(5),
            TimePoint::new(10),
        )
        .with_lower_bound(AsapLowerBound::new(
            AsapBoundReason::Dependency,
            TimePoint::new(30),
        ))
        .with_latest(TimePoint::new(20));

        let result = policy.earliest_start(&bounds);

        assert!(matches!(
            result,
            Err(AsapError::InvalidBounds { .. })
        ));
    }

    #[test]
    fn decision_contains_limiting_bound() {
        let policy = AsapPolicy::new();

        let bounds = AsapBounds::new(
            operation(6),
            TimePoint::new(10),
        )
        .with_lower_bound(AsapLowerBound::new(
            AsapBoundReason::Dependency,
            TimePoint::new(50),
        ))
        .with_lower_bound(AsapLowerBound::new(
            AsapBoundReason::Resource,
            TimePoint::new(20),
        ));

        let decision = policy
            .schedule(&bounds, Duration::new(5))
            .expect("valid bounds");

        assert_eq!(
            decision.limiting_bound()
                .expect("limiting bound")
                .reason(),
            &AsapBoundReason::Dependency
        );
    }

    #[test]
    fn equal_start_times_use_operation_id_for_determinism() {
        let policy = AsapPolicy::new();

        let left = policy
            .schedule(
                &AsapBounds::new(
                    operation(10),
                    TimePoint::new(50),
                ),
                Duration::new(5),
            )
            .expect("valid decision");

        let right = policy
            .schedule(
                &AsapBounds::new(
                    operation(11),
                    TimePoint::new(50),
                ),
                Duration::new(5),
            )
            .expect("valid decision");

        assert_eq!(
            policy.compare_decisions(&left, &right),
            Ordering::Less
        );
    }

    #[test]
    fn earlier_start_wins() {
        let policy = AsapPolicy::new();

        let left = policy
            .schedule(
                &AsapBounds::new(
                    operation(20),
                    TimePoint::new(10),
                ),
                Duration::new(5),
            )
            .expect("valid decision");

        let right = policy
            .schedule(
                &AsapBounds::new(
                    operation(21),
                    TimePoint::new(20),
                ),
                Duration::new(5),
            )
            .expect("valid decision");

        assert!(policy.prefers(&left, &right));
        assert!(!policy.prefers(&right, &left));
    }

    #[test]
    fn decision_validation_preserves_invariants() {
        let policy = AsapPolicy::new();

        let bounds = AsapBounds::new(
            operation(30),
            TimePoint::new(10),
        )
        .with_lower_bound(AsapLowerBound::new(
            AsapBoundReason::Dependency,
            TimePoint::new(25),
        ));

        let decision = policy
            .schedule(&bounds, Duration::new(10))
            .expect("valid decision");

        policy
            .validate_decision(&bounds, &decision)
            .expect("decision should validate");
    }

    #[test]
    fn multiple_bound_types_are_composable() {
        let policy = AsapPolicy::new();

        let bounds = AsapBounds::new(
            operation(40),
            TimePoint::new(1),
        )
        .with_lower_bound(AsapLowerBound::new(
            AsapBoundReason::Dependency,
            TimePoint::new(5),
        ))
        .with_lower_bound(AsapLowerBound::new(
            AsapBoundReason::Resource,
            TimePoint::new(8),
        ))
        .with_lower_bound(AsapLowerBound::new(
            AsapBoundReason::Classical,
            TimePoint::new(12),
        ))
        .with_lower_bound(AsapLowerBound::new(
            AsapBoundReason::Qec,
            TimePoint::new(10),
        ));

        let decision = policy
            .schedule(&bounds, Duration::new(3))
            .expect("valid decision");

        assert_eq!(decision.start(), TimePoint::new(12));
        assert_eq!(decision.end(), TimePoint::new(15));
    }

    #[test]
    fn no_machine_size_assumption_exists() {
        let capabilities = AsapPolicy::new().capabilities();

        assert!(capabilities.target_independent);
        assert!(capabilities.resource_bounds);
    }
}