
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Universal Runtime — Real-Time Deadlines
//!
//! Path: `src/zur/realtime/deadlines.rs`
//!
//! Edition: Rust 2021
//! Minimum supported Rust: 1.97
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Provides portable, monotonic-clock deadline contracts for
//! ZUR-managed execution operations.
//!
//! A deadline represents the latest acceptable completion boundary
//! supplied by an execution caller or inherited from a parent
//! execution context.
//!
//! The implementation supports:
//!
//! - Unbounded execution.
//! - Absolute monotonic deadlines.
//! - Relative timeout construction.
//! - Deadline composition.
//! - Parent deadline inheritance.
//! - Child deadline restriction.
//! - Remaining-time calculations.
//! - Deadline expiration checks.
//! - Cancellation-aware checks.
//! - Structured timeout errors.
//! - Deterministic clock-based tests.
//!
//! ## ARCHITECTURAL OWNERSHIP
//!
//! This module owns:
//!
//! - The ZUR operational Deadline abstraction.
//! - Local monotonic deadline representation.
//! - Deadline construction and validation.
//! - Deadline composition.
//! - Deadline expiration evaluation.
//! - Remaining execution-time calculation.
//! - Cancellation-aware deadline validation.
//!
//! It does NOT own:
//!
//! - Source-language deadline syntax.
//! - Scheduling AST structures.
//! - Semantic deadline validation.
//! - Execution-plan construction.
//! - Scheduling algorithms.
//! - Task lifecycle.
//! - Task preemption.
//! - Resource allocation.
//! - Hardware discovery.
//! - Operating-system real-time guarantees.
//! - Distributed clock synchronization.
//! - Remote deadline propagation protocols.
//! - Provider-specific timeout implementation.
//! - Recovery and retry policies.
//!
//! Those responsibilities remain with their existing
//! repository authorities.
//!
//! ## EXISTING AUTHORITIES
//!
//! - `zur/error.rs`:
//!   ZurError and ZurResult.
//!
//! - `zur/execution/session.rs`:
//!   Session lifecycle and cooperative cancellation.
//!
//! - `zur/contracts/execution.rs`:
//!   Execution contracts and cancellation dispositions.
//!
//! - `zur/scheduling/`:
//!   Operational scheduling.
//!
//! - `stdlib/context.rs`:
//!   OperationContext and CancellationSignal.
//!
//! - `stdlib/concurrency/cancellation.rs`:
//!   CancellationToken and asynchronous cancellation.
//!
//! - `distributed/execution/`:
//!   Distributed execution and its deadline propagation.
//!
//! - `grammar/execution/scheduling.g4`:
//!   Source-level deadline intent.
//!
//! - `grammar/execution/execution-context.g4`:
//!   Execution-context syntax.
//!
//! - `grammar/specification/execution-model.md`:
//!   Normative execution architecture.
//!
//! ## CLOCK SEMANTICS
//!
//! Deadlines use `std::time::Instant`.
//!
//! Instant is monotonic and suitable for measuring elapsed time
//! within the relevant process and clock environment.
//!
//! A monotonic Instant MUST NOT be serialized as a portable
//! timestamp or transmitted directly to another machine.
//!
//! Distributed integrations must propagate an appropriate remaining
//! duration or use their authoritative distributed deadline protocol.
//!
//! This module does not assume synchronized wall clocks.
//!
//! ## DEADLINE SEMANTICS
//!
//! An unbounded deadline imposes no time-based completion boundary.
//!
//! A bounded deadline expires when:
//!
//!     current_monotonic_time >= deadline
//!
//! A child operation inherits the earliest applicable parent
//! or child deadline.
//!
//! A child operation cannot extend its parent's deadline merely
//! by requesting a longer timeout.
//!
//! ## CANCELLATION
//!
//! Deadline expiration and cancellation are different conditions.
//!
//! Cancellation is checked first when both conditions are present.
//!
//! A deadline expiration does not forcibly terminate execution.
//!
//! Operations and providers must observe deadlines at appropriate
//! execution boundaries.
//!
//! A timeout error does not prove that a remote operation,
//! physical device, QPU, FPGA or external process has stopped.
//!
//! ## SCALABILITY
//!
//! No artificial maximum is imposed on:
//!
//! - Execution duration.
//! - Number of deadlines.
//! - Number of executions.
//! - Number of sessions.
//! - Number of providers.
//! - Number of computational domains.
//! - Number of devices.
//! - Number of distributed participants.
//!
//! Actual storage and execution remain subject to available
//! memory, clock representation and physical resources.
//!
//! Literal infinite physical execution time is not promised.
//!
//! ## SAFETY
//!
//! Safe Rust only.
//!
//! No unsafe blocks.
//! No unsafe implementations.
//! No global mutable state.
//! No background timer thread.
//! No hidden scheduler.
//! No automatic cancellation.
//! No fabricated completion.
//! No hardware-specific assumptions.

use std::time::{Duration, Instant};

use crate::stdlib::context::OperationContext;

use crate::zur::error::{ZurError, ZurResult};

// ============================================================================
// COMPONENT IDENTITY
// ============================================================================

/// Component identity used in structured ZUR diagnostics.
const COMPONENT: &str = "zur.realtime.deadlines";

// ============================================================================
// DEADLINE
// ============================================================================

/// A monotonic execution deadline.
///
/// `Deadline` represents either:
///
/// - An unbounded execution interval; or
/// - An absolute monotonic-clock completion boundary.
///
/// The underlying clock instant is intentionally private.
///
/// This prevents callers from treating a process-local monotonic
/// timestamp as a portable wall-clock timestamp.
///
/// A deadline is immutable and can safely be copied between
/// execution contexts operating within the same clock environment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Deadline {
    instant: Option<Instant>,
}

impl Default for Deadline {
    fn default() -> Self {
        Self::unbounded()
    }
}

impl Deadline {
    // ------------------------------------------------------------------------
    // CONSTRUCTION
    // ------------------------------------------------------------------------

    /// Creates an unbounded deadline.
    ///
    /// An unbounded deadline does not impose a time-based completion
    /// boundary.
    ///
    /// Resource budgets, cancellation and other execution constraints
    /// remain independently enforceable.
    #[must_use]
    pub const fn unbounded() -> Self {
        Self {
            instant: None,
        }
    }

    /// Creates a deadline at an absolute monotonic-clock instant.
    ///
    /// The supplied instant must belong to the clock environment
    /// in which this deadline will be evaluated.
    ///
    /// An instant that is already in the past is accepted as an
    /// immediately expired deadline.
    ///
    /// This method does not perform clock synchronization.
    #[must_use]
    pub const fn at(instant: Instant) -> Self {
        Self {
            instant: Some(instant),
        }
    }

    /// Creates a deadline relative to the current monotonic time.
    ///
    /// The deadline is calculated using checked clock arithmetic.
    ///
    /// # Errors
    ///
    /// Returns a structured configuration error if the supplied
    /// duration cannot be represented by the underlying clock.
    pub fn after(timeout: Duration) -> ZurResult<Self> {
        let instant = Instant::now()
            .checked_add(timeout)
            .ok_or_else(|| {
                ZurError::configuration(
                    "requested deadline exceeds the supported monotonic clock range",
                )
                .with_component(COMPONENT)
                .with_operation("construct_deadline")
            })?;

        Ok(Self::at(instant))
    }

    /// Creates a deadline from an optional relative timeout.
    ///
    /// `None` means unbounded execution.
    ///
    /// `Some(duration)` creates a deadline relative to the current
    /// monotonic time.
    ///
    /// # Errors
    ///
    /// Returns a configuration error if the duration cannot be
    /// represented by the underlying clock.
    pub fn from_timeout(
        timeout: Option<Duration>,
    ) -> ZurResult<Self> {
        match timeout {
            Some(duration) => Self::after(duration),
            None => Ok(Self::unbounded()),
        }
    }

    // ------------------------------------------------------------------------
    // ACCESSORS
    // ------------------------------------------------------------------------

    /// Returns whether the deadline is bounded.
    #[must_use]
    pub const fn is_bounded(self) -> bool {
        self.instant.is_some()
    }

    /// Returns whether execution has no time-based completion boundary.
    #[must_use]
    pub const fn is_unbounded(self) -> bool {
        self.instant.is_none()
    }

    /// Returns the underlying monotonic instant, if bounded.
    ///
    /// The returned instant is process/clock-environment local.
    ///
    /// It must not be serialized or interpreted as a portable
    /// distributed timestamp.
    #[must_use]
    pub const fn instant(self) -> Option<Instant> {
        self.instant
    }

    /// Returns the remaining time relative to the current monotonic clock.
    ///
    /// Returns:
    ///
    /// - `None` for an unbounded deadline.
    /// - `Some(Duration::ZERO)` for an expired deadline.
    /// - `Some(remaining_duration)` for an active deadline.
    #[must_use]
    pub fn remaining(self) -> Option<Duration> {
        self.remaining_at(Instant::now())
    }

    /// Returns the remaining time relative to a supplied clock instant.
    ///
    /// This method is also useful for deterministic testing.
    ///
    /// Returns `None` for an unbounded deadline.
    #[must_use]
    pub fn remaining_at(
        self,
        now: Instant,
    ) -> Option<Duration> {
        self.instant.map(|deadline| {
            deadline.saturating_duration_since(now)
        })
    }

    // ------------------------------------------------------------------------
    // EXPIRATION
    // ------------------------------------------------------------------------

    /// Returns whether the deadline has expired.
    ///
    /// Unbounded deadlines never expire through time evaluation.
    #[must_use]
    pub fn is_expired(self) -> bool {
        self.is_expired_at(Instant::now())
    }

    /// Evaluates expiration against a supplied monotonic instant.
    ///
    /// The deadline expires at the exact boundary.
    #[must_use]
    pub fn is_expired_at(
        self,
        now: Instant,
    ) -> bool {
        match self.instant {
            Some(deadline) => now >= deadline,
            None => false,
        }
    }

    // ------------------------------------------------------------------------
    // COMPOSITION
    // ------------------------------------------------------------------------

    /// Restricts this deadline using another deadline.
    ///
    /// The earlier bounded deadline wins.
    ///
    /// An unbounded deadline does not extend or restrict another
    /// deadline.
    ///
    /// This operation is useful for combining:
    ///
    /// - Runtime-level deadlines.
    /// - Session-level deadlines.
    /// - Execution-level deadlines.
    /// - Task-level deadlines.
    ///
    /// A child execution therefore cannot extend a parent's
    /// completion boundary.
    #[must_use]
    pub fn constrain(self, other: Self) -> Self {
        match (self.instant, other.instant) {
            (None, None) => Self::unbounded(),

            (Some(instant), None)
            | (None, Some(instant)) => Self::at(instant),

            (Some(first), Some(second)) => {
                if first <= second {
                    Self::at(first)
                } else {
                    Self::at(second)
                }
            }
        }
    }

    /// Creates a child deadline constrained by this parent deadline.
    ///
    /// The child receives the earlier of:
    ///
    /// - The inherited parent deadline.
    /// - The requested child timeout.
    ///
    /// A child timeout cannot extend the parent deadline.
    ///
    /// # Errors
    ///
    /// Returns a configuration error if the requested timeout
    /// exceeds the supported monotonic clock range.
    pub fn child(
        self,
        timeout: Option<Duration>,
    ) -> ZurResult<Self> {
        let requested = Self::from_timeout(timeout)?;

        Ok(self.constrain(requested))
    }

    /// Returns a deadline constrained by another optional deadline.
    ///
    /// This is a convenience method for combining optional execution
    /// constraints without inventing a deadline when neither exists.
    #[must_use]
    pub fn constrain_optional(
        self,
        other: Option<Self>,
    ) -> Self {
        match other {
            Some(other) => self.constrain(other),
            None => self,
        }
    }

    // ------------------------------------------------------------------------
    // VALIDATION
    // ------------------------------------------------------------------------

    /// Checks whether the deadline is still valid.
    ///
    /// # Errors
    ///
    /// Returns a structured timeout error if the deadline has expired.
    pub fn check(self) -> ZurResult<()> {
        self.check_operation("execution")
    }

    /// Checks the deadline for a named operation.
    ///
    /// The operation name is diagnostic metadata only.
    /// It does not affect deadline evaluation.
    ///
    /// # Errors
    ///
    /// Returns `ZurError::timed_out` when the deadline has expired.
    pub fn check_operation(
        self,
        operation: &str,
    ) -> ZurResult<()> {
        self.check_at(Instant::now(), operation)
    }

    /// Checks the deadline against an explicitly supplied instant.
    ///
    /// This method provides deterministic clock evaluation without
    /// requiring sleeps or background timer threads.
    ///
    /// # Errors
    ///
    /// Returns a structured timeout error when the deadline has expired.
    pub fn check_at(
        self,
        now: Instant,
        operation: &str,
    ) -> ZurResult<()> {
        if !self.is_expired_at(now) {
            return Ok(());
        }

        Err(
            ZurError::timed_out(
                "execution deadline expired",
            )
            .with_component(COMPONENT)
            .with_operation(operation),
        )
    }

    /// Checks cancellation and deadline conditions together.
    ///
    /// Cancellation takes precedence when both conditions are
    /// already observable.
    ///
    /// This method consumes the existing standard-library
    /// OperationContext cancellation contract.
    ///
    /// It does not introduce another cancellation-token
    /// implementation.
    ///
    /// # Errors
    ///
    /// Returns:
    ///
    /// - `ZurError::cancelled` when cancellation was requested.
    /// - `ZurError::timed_out` when the deadline expired.
    ///
    /// A successful return means only that neither condition
    /// was observed at the instant of this check.
    ///
    /// It does not guarantee that a later operation will complete
    /// before the deadline.
    pub fn check_with_context(
        self,
        context: &OperationContext,
    ) -> ZurResult<()> {
        if context.is_cancelled() {
            return Err(
                ZurError::cancelled(
                    "execution cancellation was requested",
                )
                .with_component(COMPONENT)
                .with_operation("check_execution_control"),
            );
        }

        self.check_operation("check_execution_control")
    }

    /// Checks cancellation and deadline conditions for a named operation.
    ///
    /// # Errors
    ///
    /// Returns a structured cancellation or timeout error.
    pub fn check_operation_with_context(
        self,
        context: &OperationContext,
        operation: &str,
    ) -> ZurResult<()> {
        if context.is_cancelled() {
            return Err(
                ZurError::cancelled(
                    "execution cancellation was requested",
                )
                .with_component(COMPONENT)
                .with_operation(operation),
            );
        }

        self.check_operation(operation)
    }
}

// ============================================================================
// DEADLINE COMPOSITION
// ============================================================================

/// Combines multiple applicable deadlines into one effective deadline.
///
/// Each supplied deadline represents an independently applicable
/// execution boundary.
///
/// The resulting deadline is the earliest bounded deadline.
///
/// No deadline is created when every supplied deadline is unbounded.
///
/// This function does not impose an artificial maximum number of
/// parent or execution constraints.
///
/// # Examples
///
/// ```
/// use std::time::Duration;
///
/// use zamani_compiler::zur::realtime::deadlines::{
///     combine_deadlines,
///     Deadline,
/// };
///
/// let runtime = Deadline::after(
///     Duration::from_secs(30),
/// )?;
///
/// let execution = Deadline::after(
///     Duration::from_secs(10),
/// )?;
///
/// let effective = combine_deadlines([runtime, execution]);
///
/// assert!(effective.is_bounded());
///
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[must_use]
pub fn combine_deadlines(
    deadlines: impl IntoIterator<Item = Deadline>,
) -> Deadline {
    deadlines
        .into_iter()
        .fold(Deadline::unbounded(), Deadline::constrain)
}

// ============================================================================
// DEADLINE BUDGET
// ============================================================================

/// A snapshot of the remaining time in a deadline.
///
/// This is a duration-based observation, not a guarantee that
/// execution will complete within that duration.
///
/// It can be useful when constructing an external execution request.
///
/// Distributed integrations must use their authoritative deadline
/// propagation protocol rather than transmitting `Instant`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RemainingTime {
    duration: Duration,
}

impl RemainingTime {
    /// Creates a remaining-time observation.
    #[must_use]
    pub const fn new(duration: Duration) -> Self {
        Self {
            duration,
        }
    }

    /// Returns the observed remaining duration.
    #[must_use]
    pub const fn duration(self) -> Duration {
        self.duration
    }

    /// Returns whether the observed budget is exhausted.
    #[must_use]
    pub const fn is_exhausted(self) -> bool {
        self.duration.is_zero()
    }
}

impl Deadline {
    /// Returns the current remaining-time observation.
    ///
    /// An unbounded deadline returns `None`.
    ///
    /// An expired deadline returns `Some(RemainingTime::new(
    /// Duration::ZERO))`.
    #[must_use]
    pub fn remaining_time(self) -> Option<RemainingTime> {
        self.remaining().map(RemainingTime::new)
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use std::time::{Duration, Instant};

    // ------------------------------------------------------------------------
    // CONSTRUCTION
    // ------------------------------------------------------------------------

    #[test]
    fn unbounded_deadline_is_not_bounded() {
        let deadline = Deadline::unbounded();

        assert!(deadline.is_unbounded());
        assert!(!deadline.is_bounded());
        assert_eq!(deadline.instant(), None);
        assert_eq!(deadline.remaining(), None);
        assert!(!deadline.is_expired());
    }

    #[test]
    fn default_deadline_is_unbounded() {
        assert_eq!(
            Deadline::default(),
            Deadline::unbounded(),
        );
    }

    #[test]
    fn absolute_deadline_preserves_instant() {
        let instant = Instant::now();

        let deadline = Deadline::at(instant);

        assert_eq!(deadline.instant(), Some(instant));
        assert!(deadline.is_bounded());
    }

    #[test]
    fn relative_deadline_is_constructed() {
        let deadline = Deadline::after(
            Duration::from_secs(30),
        )
        .expect("valid timeout");

        assert!(deadline.is_bounded());
        assert!(!deadline.is_expired());
        assert!(deadline.remaining().is_some());
    }

    #[test]
    fn optional_timeout_supports_unbounded_execution() {
        let deadline = Deadline::from_timeout(None)
            .expect("unbounded deadline");

        assert!(deadline.is_unbounded());
    }

    #[test]
    fn optional_timeout_supports_bounded_execution() {
        let deadline = Deadline::from_timeout(
            Some(Duration::from_secs(5)),
        )
        .expect("valid deadline");

        assert!(deadline.is_bounded());
    }

    // ------------------------------------------------------------------------
    // REMAINING TIME
    // ------------------------------------------------------------------------

    #[test]
    fn remaining_time_is_calculated_correctly() {
        let origin = Instant::now();

        let deadline = Deadline::at(
            origin + Duration::from_secs(10),
        );

        assert_eq!(
            deadline.remaining_at(
                origin + Duration::from_secs(3),
            ),
            Some(Duration::from_secs(7)),
        );
    }

    #[test]
    fn expired_deadline_has_zero_remaining_time() {
        let origin = Instant::now();

        let deadline = Deadline::at(
            origin + Duration::from_secs(1),
        );

        assert_eq!(
            deadline.remaining_at(
                origin + Duration::from_secs(2),
            ),
            Some(Duration::ZERO),
        );
    }

    #[test]
    fn remaining_time_observation_is_available() {
        let deadline = Deadline::after(
            Duration::from_secs(5),
        )
        .expect("deadline");

        let remaining = deadline
            .remaining_time()
            .expect("bounded deadline");

        assert!(!remaining.is_exhausted());
    }

    #[test]
    fn unbounded_deadline_has_no_remaining_time_limit() {
        assert_eq!(
            Deadline::unbounded().remaining_time(),
            None,
        );
    }

    // ------------------------------------------------------------------------
    // EXPIRATION
    // ------------------------------------------------------------------------

    #[test]
    fn deadline_is_not_expired_before_boundary() {
        let origin = Instant::now();

        let deadline = Deadline::at(
            origin + Duration::from_secs(5),
        );

        assert!(!deadline.is_expired_at(origin));
    }

    #[test]
    fn deadline_expires_at_exact_boundary() {
        let origin = Instant::now();

        let boundary = origin + Duration::from_secs(5);

        let deadline = Deadline::at(boundary);

        assert!(deadline.is_expired_at(boundary));
    }

    #[test]
    fn deadline_expires_after_boundary() {
        let origin = Instant::now();

        let boundary = origin + Duration::from_secs(5);

        let deadline = Deadline::at(boundary);

        assert!(
            deadline.is_expired_at(
                boundary + Duration::from_nanos(1),
            )
        );
    }

    #[test]
    fn unbounded_deadline_never_expires() {
        let origin = Instant::now();

        assert!(
            !Deadline::unbounded().is_expired_at(
                origin + Duration::from_secs(1000),
            )
        );
    }

    // ------------------------------------------------------------------------
    // COMPOSITION
    // ------------------------------------------------------------------------

    #[test]
    fn unbounded_deadlines_remain_unbounded() {
        let effective = Deadline::unbounded()
            .constrain(Deadline::unbounded());

        assert!(effective.is_unbounded());
    }

    #[test]
    fn bounded_deadline_restricts_unbounded_deadline() {
        let instant = Instant::now();

        let effective = Deadline::unbounded()
            .constrain(Deadline::at(instant));

        assert_eq!(effective.instant(), Some(instant));
    }

    #[test]
    fn earlier_deadline_wins() {
        let origin = Instant::now();

        let earlier = Deadline::at(
            origin + Duration::from_secs(5),
        );

        let later = Deadline::at(
            origin + Duration::from_secs(10),
        );

        assert_eq!(
            earlier.constrain(later),
            earlier,
        );
    }

    #[test]
    fn later_parent_does_not_extend_child_deadline() {
        let origin = Instant::now();

        let parent = Deadline::at(
            origin + Duration::from_secs(30),
        );

        let child = Deadline::at(
            origin + Duration::from_secs(10),
        );

        assert_eq!(
            parent.constrain(child),
            child,
        );
    }

    #[test]
    fn child_cannot_extend_parent_deadline() {
        let origin = Instant::now();

        let parent = Deadline::at(
            origin + Duration::from_secs(5),
        );

        let child = parent
            .child(Some(Duration::from_secs(30)))
            .expect("valid child deadline");

        assert_eq!(child, parent);
    }

    #[test]
    fn child_can_restrict_parent_deadline() {
        let origin = Instant::now();

        let parent = Deadline::at(
            origin + Duration::from_secs(30),
        );

        let child = parent
            .child(Some(Duration::from_secs(5)))
            .expect("valid child deadline");

        assert!(
            child.instant().expect("bounded")
                < parent.instant().expect("bounded")
        );
    }

    #[test]
    fn unbounded_parent_can_receive_child_deadline() {
        let parent = Deadline::unbounded();

        let child = parent
            .child(Some(Duration::from_secs(5)))
            .expect("valid child deadline");

        assert!(child.is_bounded());
    }

    #[test]
    fn child_without_timeout_inherits_parent() {
        let parent = Deadline::after(
            Duration::from_secs(10),
        )
        .expect("parent deadline");

        assert_eq!(
            parent.child(None).expect("child"),
            parent,
        );
    }

    #[test]
    fn combine_deadlines_returns_earliest_boundary() {
        let origin = Instant::now();

        let first = Deadline::at(
            origin + Duration::from_secs(30),
        );

        let second = Deadline::at(
            origin + Duration::from_secs(10),
        );

        let third = Deadline::at(
            origin + Duration::from_secs(20),
        );

        assert_eq!(
            combine_deadlines([first, second, third]),
            second,
        );
    }

    #[test]
    fn combine_empty_deadlines_is_unbounded() {
        let deadlines = Vec::<Deadline>::new();

        assert!(
            combine_deadlines(deadlines).is_unbounded()
        );
    }

    #[test]
    fn optional_deadline_composition_preserves_existing_boundary() {
        let deadline = Deadline::after(
            Duration::from_secs(10),
        )
        .expect("deadline");

        assert_eq!(
            deadline.constrain_optional(None),
            deadline,
        );
    }

    // ------------------------------------------------------------------------
    // VALIDATION
    // ------------------------------------------------------------------------

    #[test]
    fn unbounded_deadline_passes_validation() {
        assert!(Deadline::unbounded().check().is_ok());
    }

    #[test]
    fn future_deadline_passes_validation() {
        let deadline = Deadline::after(
            Duration::from_secs(10),
        )
        .expect("deadline");

        assert!(deadline.check().is_ok());
    }

    #[test]
    fn expired_deadline_returns_timeout_error() {
        let origin = Instant::now();

        let deadline = Deadline::at(origin);

        let result = deadline.check_at(
            origin,
            "test.operation",
        );

        let error = result.expect_err(
            "deadline must expire at its boundary",
        );

        assert_eq!(
            error.kind(),
            crate::zur::error::ErrorKind::TimedOut,
        );

        assert_eq!(
            error.component(),
            Some(COMPONENT),
        );

        assert_eq!(
            error.operation(),
            Some("test.operation"),
        );
    }

    #[test]
    fn unbounded_deadline_passes_deterministic_validation() {
        assert!(
            Deadline::unbounded()
                .check_at(
                    Instant::now(),
                    "unbounded.operation",
                )
                .is_ok()
        );
    }

    // ------------------------------------------------------------------------
    // CANCELLATION
    // ------------------------------------------------------------------------

    #[test]
    fn cancellation_and_deadline_are_distinct_conditions() {
        let context = OperationContext::default();

        let deadline = Deadline::unbounded();

        assert!(
            deadline
                .check_with_context(&context)
                .is_ok()
        );
    }

    // ------------------------------------------------------------------------
    // IMMUTABILITY AND CLONING
    // ------------------------------------------------------------------------

    #[test]
    fn deadline_is_copyable_and_immutable() {
        let deadline = Deadline::after(
            Duration::from_secs(10),
        )
        .expect("deadline");

        let copied = deadline;

        assert_eq!(deadline, copied);
    }

    #[test]
    fn cloned_deadline_preserves_boundary() {
        let deadline = Deadline::after(
            Duration::from_secs(10),
        )
        .expect("deadline");

        assert_eq!(
            deadline,
            deadline.clone(),
        );
    }
}
