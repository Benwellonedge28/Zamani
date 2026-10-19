
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani HDL Simulation Scheduler
//!
//! Path: `src/hdl/simulation/scheduler.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Provide a generic scheduler facade over the canonical HDL simulation event
//! queue in `super::events`. Track the current logical scheduling position,
//! reject events scheduled before that position, and expose deterministic
//! inspection and removal operations.
//!
//! ## Authority
//!
//! - `grammar/DESIGN.md`
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/hdl.g4`
//! - `grammar/hdl/simulation.g4`
//! - `grammar/execution/simulation.g4`
//! - `src/frontend/parser/domains/hdl.rs`
//! - `src/frontend/semantic/domains/hdl.rs`
//! - `src/hdl/simulation/events.rs`
//! - `src/hdl/simulation/interface.rs`
//!
//! Grammar defines source syntax and simulation intent. The canonical AST
//! owns source structure, and semantic validation owns source-level
//! correctness. The concrete simulation engine owns the interpretation of
//! event payloads and the applicable HDL execution semantics.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - The scheduler facade over `EventQueue`.
//! - The last event position removed from the pending queue.
//! - Prevention of scheduling before the current logical position.
//! - Explicit clearing and timeline-reset operations.
//! - Structured scheduling errors.
//!
//! This module does not own:
//!
//! - The canonical AST or semantic model.
//! - HDLIR or another intermediate representation.
//! - Signal values, logic evaluation, or signal-update semantics.
//! - Clock generation, reset behavior, or memory semantics.
//! - Universal HDL event regions or delta-cycle advancement policy.
//! - Assertion evaluation, formal verification, or waveform storage.
//! - Physical resource allocation, synthesis, placement, or routing.
//!
//! ## Scheduling contract
//!
//! Events are ordered by the existing event queue using:
//!
//!     (logical_time, delta_cycle, insertion_sequence)
//!
//! The scheduler rejects a newly requested event whose `(logical_time,
//! delta_cycle)` precedes the last event removed from the queue.
//!
//! An event at the current position remains permissible. This is intentional:
//! some engines may need to enqueue additional work at the same logical
//! position. Engines whose semantics require stricter event-region rules must
//! enforce those rules in their own event-processing policy.
//!
//! This module does not automatically increment delta cycles or infer when a
//! time slot has settled. The caller supplies the logical time and delta-cycle
//! index according to its engine's documented semantics.
//!
//! ## Clearing and resetting
//!
//! `clear_pending` removes queued events but preserves the current logical
//! position. This prevents subsequent scheduling from moving backwards merely
//! because the queue was emptied.
//!
//! `reset_timeline` explicitly starts a new logical timeline by clearing
//! pending events and forgetting the current position. It does not reset the
//! underlying queue's insertion sequence; previously returned event IDs are
//! therefore not reused during this scheduler's lifetime.
//!
//! Neither operation can reverse effects already applied by a simulation
//! engine. The caller is responsible for ensuring that a timeline reset is
//! appropriate for its simulation lifecycle.
//!
//! ## Scalability
//!
//! No universal event-count, time, delta-cycle, or design-size ceiling is
//! imposed. The underlying binary heap supports dynamic growth subject to
//! available memory and representable collection sizes.
//!
//! Scheduling-position comparisons use the supplied time type's total order.
//! This module does not force time into a host integer, floating-point value,
//! or fixed physical timescale.
//!
//! No finite implementation can guarantee literally infinite execution.
//! Queue growth and processing remain subject to available resources and
//! explicit policies applied by the calling engine.
//!
//! ## Integration
//!
//! - `events.rs` owns event storage, event IDs, and deterministic queue order.
//! - `scheduler.rs` owns scheduling-position policy and queue orchestration.
//! - `interface.rs` owns engine/session lifecycle, cancellation, run budgets,
//!   and run reports.
//! - A concrete engine interprets event payloads and implements the selected
//!   HDL semantics.
//!
//! The scheduler is not itself a `SimulationSession`: a payload has no
//! universal execution meaning, and only an engine can correctly apply it.
//!
//! All HDLIR implementation remains outside this module.
//!
//! ## Freeze criteria
//!
//! - Existing event-queue ordering is preserved.
//! - Scheduling before the current position fails without queue mutation.
//! - Clearing pending events does not silently rewind the timeline.
//! - Timeline reset does not reuse event IDs.
//! - Errors remain structured and deterministic.
//! - Tests cover ordering, invalid scheduling, clearing, and reset behavior.
//! - The module compiles without unsafe code and without HDLIR dependencies.

use std::error::Error;
use std::fmt;

use super::events::{EventId, EventQueue, EventQueueError, ScheduledEvent};

// =============================================================================
// Scheduler position
// =============================================================================

/// The logical position of the most recently removed event.
///
/// This position is scheduler bookkeeping. It does not assert that an HDL
/// time slot is settled or that all events for this position have been
/// processed according to a particular language's event-region rules.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SchedulerPosition<Time> {
    time: Time,
    delta_cycle: u128,
}

impl<Time> SchedulerPosition<Time> {
    /// Return the logical time of this position.
    #[must_use]
    pub const fn time(&self) -> &Time {
        &self.time
    }

    /// Return the delta-cycle index of this position.
    #[must_use]
    pub const fn delta_cycle(&self) -> u128 {
        self.delta_cycle
    }
}

// =============================================================================
// Errors
// =============================================================================

/// Errors returned by scheduler operations.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum SchedulerError {
    /// The requested event position precedes the current scheduler position.
    ///
    /// Inspect [`Scheduler::current_position`] to obtain the current position.
    /// The scheduler leaves the queue unchanged when this error is returned.
    SchedulingBeforeCurrentPosition,

    /// The underlying queue could not allocate another event identity.
    EventQueue(EventQueueError),
}

impl fmt::Display for SchedulerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SchedulingBeforeCurrentPosition => {
                f.write_str(
                    "cannot schedule an HDL event before the current scheduler position",
                )
            }
            Self::EventQueue(error) => write!(f, "HDL event queue error: {error}"),
        }
    }
}

impl Error for SchedulerError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::SchedulingBeforeCurrentPosition => None,
            Self::EventQueue(error) => Some(error),
        }
    }
}

impl From<EventQueueError> for SchedulerError {
    fn from(error: EventQueueError) -> Self {
        Self::EventQueue(error)
    }
}

// =============================================================================
// Scheduler
// =============================================================================

/// A deterministic scheduler facade over the HDL event queue.
///
/// `Time` must implement a total order compatible with the simulation
/// engine's logical-time semantics. `Payload` is opaque to the scheduler and
/// does not need to implement `Ord`.
///
/// The scheduler is designed for use by one simulation engine or session.
/// It does not provide synchronization for concurrent callers. Engines that
/// accept concurrent event producers must coordinate access externally and
/// define how producer ordering affects determinism.
#[derive(Clone, Debug)]
pub struct Scheduler<Time, Payload> {
    queue: EventQueue<Time, Payload>,
    current_position: Option<SchedulerPosition<Time>>,
}

impl<Time, Payload> Default for Scheduler<Time, Payload> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Time, Payload> Scheduler<Time, Payload> {
    /// Create an empty scheduler with no current logical position.
    ///
    /// No event storage is reserved up front. The underlying queue grows as
    /// events are scheduled.
    #[must_use]
    pub fn new() -> Self {
        Self {
            queue: EventQueue::new(),
            current_position: None,
        }
    }

    /// Return the number of pending events.
    #[must_use]
    pub fn pending_len(&self) -> usize {
        self.queue.len()
    }

    /// Return whether no events are pending.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    /// Borrow the next event without removing it or changing the current
    /// scheduler position.
    #[must_use]
    pub fn peek_next(&self) -> Option<&ScheduledEvent<Time, Payload>> {
        self.queue.peek()
    }

    /// Return the logical time of the next pending event, if any.
    #[must_use]
    pub fn next_time(&self) -> Option<&Time> {
        self.queue.next_time()
    }

    /// Borrow the current scheduler position.
    ///
    /// This is `None` before the first event is removed and after an explicit
    /// [`Self::reset_timeline`] call.
    #[must_use]
    pub fn current_position(&self) -> Option<&SchedulerPosition<Time>> {
        self.current_position.as_ref()
    }

    /// Remove all pending events while preserving the current position.
    ///
    /// Event IDs already issued by the underlying queue are not reused.
    /// This operation does not undo any effects previously applied by the
    /// simulation engine.
    pub fn clear_pending(&mut self) {
        self.queue.clear();
    }

    /// Explicitly reset the scheduler's logical timeline.
    ///
    /// Pending events are discarded and the current position becomes `None`.
    /// The underlying queue's insertion sequence is preserved, so previously
    /// issued event IDs are not reused.
    ///
    /// The caller must ensure that resetting the timeline is valid for the
    /// surrounding simulation session. This method does not reset signals,
    /// memories, assertions, or any other engine state.
    pub fn reset_timeline(&mut self) {
        self.queue.clear();
        self.current_position = None;
    }
}

impl<Time: Ord + Clone, Payload> Scheduler<Time, Payload> {
    /// Schedule an event at delta cycle zero of `time`.
    ///
    /// Returns the event's queue-local identity. The operation fails without
    /// inserting the event if its requested position precedes the current
    /// scheduler position or if the underlying queue cannot allocate an ID.
    pub fn schedule_at(
        &mut self,
        time: Time,
        payload: Payload,
    ) -> Result<EventId, SchedulerError> {
        self.schedule_at_delta(time, 0, payload)
    }

    /// Schedule an event at an explicit logical time and delta cycle.
    ///
    /// The scheduler does not choose or increment delta cycles on the caller's
    /// behalf. The engine must supply a value consistent with its event
    /// semantics.
    ///
    /// Scheduling at the current position is allowed. Scheduling at an
    /// earlier time or an earlier delta cycle at the same time is rejected.
    ///
    /// If scheduling fails, the pending queue and current position remain
    /// unchanged.
    pub fn schedule_at_delta(
        &mut self,
        time: Time,
        delta_cycle: u128,
        payload: Payload,
    ) -> Result<EventId, SchedulerError> {
        if let Some(current) = self.current_position.as_ref() {
            let requested_order = time
                .cmp(&current.time)
                .then_with(|| delta_cycle.cmp(&current.delta_cycle));

            if requested_order.is_lt() {
                return Err(SchedulerError::SchedulingBeforeCurrentPosition);
            }
        }

        self.queue
            .schedule_at_delta(time, delta_cycle, payload)
            .map_err(SchedulerError::from)
    }

    /// Remove the next event and update the current scheduler position.
    ///
    /// Returns `None` when the queue is empty. If an event is returned, its
    /// `(time, delta_cycle)` becomes the current position.
    ///
    /// Removing an event does not execute its payload. The calling engine is
    /// responsible for applying the event according to the selected HDL
    /// semantics.
    pub fn pop_next(&mut self) -> Option<ScheduledEvent<Time, Payload>> {
        let event = self.queue.pop()?;

        self.current_position = Some(SchedulerPosition {
            time: event.time().clone(),
            delta_cycle: event.delta_cycle(),
        });

        Some(event)
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_scheduler_is_empty_without_a_current_position() {
        let scheduler = Scheduler::<u64, &'static str>::new();

        assert!(scheduler.is_empty());
        assert_eq!(scheduler.pending_len(), 0);
        assert!(scheduler.peek_next().is_none());
        assert!(scheduler.next_time().is_none());
        assert!(scheduler.current_position().is_none());
        assert!(scheduler.clone().is_empty());
    }

    #[test]
    fn events_are_removed_in_deterministic_time_order() {
        let mut scheduler = Scheduler::new();

        scheduler.schedule_at(30_u64, "later").expect("schedule");
        scheduler.schedule_at(10_u64, "earlier").expect("schedule");
        scheduler.schedule_at(20_u64, "middle").expect("schedule");

        assert_eq!(scheduler.pop_next().expect("event").payload(), &"earlier");
        assert_eq!(scheduler.pop_next().expect("event").payload(), &"middle");
        assert_eq!(scheduler.pop_next().expect("event").payload(), &"later");
        assert!(scheduler.is_empty());
    }

    #[test]
    fn equal_time_and_delta_events_remain_fifo() {
        let mut scheduler = Scheduler::new();

        let first = scheduler
            .schedule_at_delta(10_u64, 3, "first")
            .expect("schedule");
        let second = scheduler
            .schedule_at_delta(10_u64, 3, "second")
            .expect("schedule");

        assert!(first.sequence() < second.sequence());
        assert_eq!(scheduler.pop_next().expect("first").payload(), &"first");
        assert_eq!(scheduler.pop_next().expect("second").payload(), &"second");
    }

    #[test]
    fn delta_cycle_precedes_insertion_order() {
        let mut scheduler = Scheduler::new();

        scheduler
            .schedule_at_delta(10_u64, 4, "later delta")
            .expect("schedule");
        scheduler
            .schedule_at_delta(10_u64, 1, "earlier delta")
            .expect("schedule");

        assert_eq!(
            scheduler.pop_next().expect("event").payload(),
            &"earlier delta"
        );
        assert_eq!(
            scheduler.pop_next().expect("event").payload(),
            &"later delta"
        );
    }

    #[test]
    fn peek_does_not_advance_the_current_position() {
        let mut scheduler = Scheduler::new();
        scheduler.schedule_at(7_u64, "event").expect("schedule");

        assert_eq!(scheduler.peek_next().expect("peek").payload(), &"event");
        assert!(scheduler.current_position().is_none());

        scheduler.pop_next().expect("pop");

        assert_eq!(
            scheduler.current_position().expect("position").time(),
            &7_u64
        );
    }

    #[test]
    fn scheduling_before_current_time_is_rejected() {
        let mut scheduler = Scheduler::new();
        scheduler.schedule_at(10_u64, "first").expect("schedule");
        scheduler.pop_next().expect("pop");

        let result = scheduler.schedule_at(9_u64, "past");

        assert_eq!(
            result,
            Err(SchedulerError::SchedulingBeforeCurrentPosition)
        );
        assert!(scheduler.is_empty());
        assert_eq!(
            scheduler.current_position().expect("position").time(),
            &10_u64
        );
    }

    #[test]
    fn earlier_delta_at_the_current_time_is_rejected() {
        let mut scheduler = Scheduler::new();
        scheduler
            .schedule_at_delta(10_u64, 4, "first")
            .expect("schedule");
        scheduler.pop_next().expect("pop");

        let result = scheduler.schedule_at_delta(10_u64, 3, "past delta");

        assert_eq!(
            result,
            Err(SchedulerError::SchedulingBeforeCurrentPosition)
        );
        assert!(scheduler.is_empty());
    }

    #[test]
    fn scheduling_at_the_current_position_is_permitted() {
        let mut scheduler = Scheduler::new();
        scheduler
            .schedule_at_delta(10_u64, 2, "first")
            .expect("schedule");
        scheduler.pop_next().expect("pop");

        scheduler
            .schedule_at_delta(10_u64, 2, "same position")
            .expect("same-position scheduling");

        assert_eq!(
            scheduler.pop_next().expect("event").payload(),
            &"same position"
        );
    }

    #[test]
    fn future_time_with_zero_delta_is_permitted() {
        let mut scheduler = Scheduler::new();
        scheduler
            .schedule_at_delta(10_u64, 8, "first")
            .expect("schedule");
        scheduler.pop_next().expect("pop");

        scheduler
            .schedule_at(11_u64, "future")
            .expect("future event");

        let event = scheduler.pop_next().expect("event");
        assert_eq!(event.time(), &11_u64);
        assert_eq!(event.delta_cycle(), 0);
    }

    #[test]
    fn clearing_pending_events_preserves_the_current_position() {
        let mut scheduler = Scheduler::new();
        scheduler.schedule_at(10_u64, "processed").expect("schedule");
        scheduler.pop_next().expect("pop");
        scheduler.schedule_at(12_u64, "pending").expect("schedule");

        scheduler.clear_pending();

        assert!(scheduler.is_empty());
        assert_eq!(
            scheduler.current_position().expect("position").time(),
            &10_u64
        );

        let result = scheduler.schedule_at(9_u64, "past");
        assert_eq!(
            result,
            Err(SchedulerError::SchedulingBeforeCurrentPosition)
        );
    }

    #[test]
    fn timeline_reset_allows_a_new_starting_time() {
        let mut scheduler = Scheduler::new();
        let first_id = scheduler.schedule_at(100_u64, "first").expect("schedule");
        scheduler.pop_next().expect("pop");

        scheduler.reset_timeline();

        assert!(scheduler.is_empty());
        assert!(scheduler.current_position().is_none());

        let second_id = scheduler.schedule_at(1_u64, "new timeline").expect("schedule");

        assert!(second_id.sequence() > first_id.sequence());
        assert_eq!(
            scheduler.pop_next().expect("event").payload(),
            &"new timeline"
        );
    }

    #[test]
    fn queue_identity_is_not_reused_after_clear() {
        let mut scheduler = Scheduler::new();
        let first_id = scheduler.schedule_at(1_u64, "first").expect("schedule");

        scheduler.clear_pending();

        let second_id = scheduler.schedule_at(1_u64, "second").expect("schedule");

        assert!(second_id.sequence() > first_id.sequence());
    }

    #[test]
    fn payload_does_not_need_ordering_traits() {
        struct Payload {
            label: &'static str,
        }

        let mut scheduler = Scheduler::new();
        scheduler
            .schedule_at(2_u64, Payload { label: "second" })
            .expect("schedule");
        scheduler
            .schedule_at(1_u64, Payload { label: "first" })
            .expect("schedule");

        assert_eq!(
            scheduler.pop_next().expect("event").payload().label,
            "first"
        );
        assert_eq!(
            scheduler.pop_next().expect("event").payload().label,
            "second"
        );
    }

    #[test]
    fn next_time_tracks_the_next_pending_event() {
        let mut scheduler = Scheduler::new();
        scheduler.schedule_at(30_u64, "later").expect("schedule");
        scheduler.schedule_at(20_u64, "earlier").expect("schedule");

        assert_eq!(scheduler.next_time(), Some(&20_u64));

        scheduler.pop_next().expect("pop");

        assert_eq!(scheduler.next_time(), Some(&30_u64));
    }

    #[test]
    fn scheduler_supports_ordered_non_integer_time_types() {
        let mut scheduler = Scheduler::new();
        scheduler.schedule_at(String::from("t2"), "second").expect("schedule");
        scheduler.schedule_at(String::from("t1"), "first").expect("schedule");

        assert_eq!(scheduler.pop_next().expect("event").payload(), &"first");
        assert_eq!(scheduler.pop_next().expect("event").payload(), &"second");
    }

    #[test]
    fn large_pending_populations_have_no_configured_count_ceiling() {
        let mut scheduler = Scheduler::new();

        for value in (0_u64..10_000).rev() {
            scheduler.schedule_at(value, value).expect("schedule");
        }

        assert_eq!(scheduler.pending_len(), 10_000);

        for expected in 0_u64..10_000 {
            assert_eq!(
                *scheduler.pop_next().expect("event").payload(),
                expected
            );
        }

        assert!(scheduler.is_empty());
    }
}
