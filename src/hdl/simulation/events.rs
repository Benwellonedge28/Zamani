
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani HDL Simulation Events
//!
//! Path: `src/hdl/simulation/events.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Provide a deterministic, generic priority queue for events used by HDL
//! simulation engines. Events are ordered by:
//!
//! 1. Engine-defined logical simulation time.
//! 2. Explicit delta-cycle index.
//! 3. Monotonically assigned insertion sequence.
//!
//! This ordering provides deterministic FIFO processing among events with
//! identical time and delta-cycle keys.
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
//! - `src/hdl/simulation/interface.rs`
//!
//! Grammar defines source syntax and simulation intent. The canonical AST
//! owns parsed source structure. Semantic validation owns source-level
//! correctness. The simulation engine owns the meaning of event processing.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Generic scheduled-event storage.
//! - Deterministic event ordering.
//! - Event insertion sequence allocation.
//! - Explicit delta-cycle metadata.
//! - Queue inspection and removal operations.
//! - Structured event-queue errors.
//!
//! This module does not own:
//!
//! - The canonical AST or semantic model.
//! - HDLIR or any other intermediate representation.
//! - A universal HDL event-region model.
//! - Signal values, signal updates, or logic evaluation.
//! - Clock generation, reset behavior, or memory semantics.
//! - Event cancellation, event coalescing, or event deduplication.
//! - Scheduler policy, simulation time units, or numerical solvers.
//! - Assertions, formal verification, waveform storage, or trace formats.
//! - Target discovery, synthesis, placement, or routing.
//!
//! ## Event-ordering contract
//!
//! The queue orders an event by the tuple `(time, delta_cycle, sequence)`.
//!
//! `T: Ord` defines time ordering. The queue does not assume that time is
//! represented by an integer, floating-point value, physical unit, or fixed
//! timescale. A time type must implement a total order consistent with the
//! simulation engine's time semantics.
//!
//! The delta-cycle index is supplied by the engine. This module does not
//! automatically advance delta cycles or decide when an HDL time slot settles.
//!
//! Insertion sequence establishes deterministic FIFO ordering for events with
//! equal time and delta-cycle keys. Payloads do not participate in ordering.
//!
//! These rules are queue mechanics, not a substitute for the selected HDL
//! language's complete event-region and signal-update semantics.
//!
//! ## Scalability
//!
//! There is no configured universal maximum event count, design size, signal
//! count, clock count, or delta-cycle count.
//!
//! The queue uses a binary heap with logarithmic insertion and removal
//! complexity and constant-time access to the next event. Actual capacity is
//! bounded by available memory and the host's representable collection sizes.
//!
//! Insertion sequence allocation uses checked arithmetic. If the sequence
//! space is exhausted, insertion fails explicitly rather than wrapping and
//! compromising deterministic ordering.
//!
//! This implementation does not promise literal infinite execution. It allows
//! the event population to grow as resources permit.
//!
//! ## Integration
//!
//! A future `src/hdl/simulation/scheduler.rs` should own scheduler policy and
//! call this queue to insert and retrieve scheduled events. The scheduler
//! should determine the applicable time and delta-cycle keys and interpret
//! each payload according to the chosen engine's semantics.
//!
//! `src/hdl/simulation/interface.rs` remains the public engine/session
//! lifecycle contract. An engine may use this queue internally, but the
//! interface does not need to expose the queue or its payload type.
//!
//! The future `src/hdl/simulation/mod.rs` should declare this module and
//! explicitly export only the event APIs intended for external consumers.
//!
//! No AST, semantic-model, scheduler, or HDLIR constructors are assumed here.
//!
//! ## Determinism
//!
//! For the same insertion sequence and equivalent ordered time values, the
//! queue returns events in the same order. Payload values cannot affect the
//! ordering of scheduled events.
//!
//! Deterministic queue behavior does not guarantee deterministic simulation
//! behavior if the engine introduces nondeterministic inputs or processing.
//!
//! ## Freeze criteria
//!
//! - Event ordering is explicit and tested.
//! - Payload types need not implement `Ord`.
//! - Sequence overflow is reported rather than wrapped.
//! - The queue has no hidden event-count ceiling.
//! - Time units and engine-specific semantics remain external.
//! - Tests cover equal keys, distinct keys, large populations, and errors.
//! - The implementation remains independent of HDLIR and target realization.

use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;
use std::error::Error;
use std::fmt;

// =============================================================================
// Event identity
// =============================================================================

/// Stable identity assigned to an event when it is inserted into one queue.
///
/// The sequence is monotonically allocated by the queue and is also the final
/// ordering key. It is unique within that queue's lifetime, provided sequence
/// allocation has not failed.
///
/// An identity is not a physical address, a source-level AST identity, or a
/// globally unique identifier. It must not be used as a portable hardware
/// identity.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct EventId {
    sequence: u128,
}

impl EventId {
    /// Return the event's insertion sequence.
    #[must_use]
    pub const fn sequence(self) -> u128 {
        self.sequence
    }
}

impl fmt::Display for EventId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "event#{}", self.sequence)
    }
}

// =============================================================================
// Scheduled event
// =============================================================================

/// An event stored in a simulation event queue.
///
/// `Time` is the engine's ordered logical time representation. `Payload` is
/// the engine-defined event content, such as a signal update, timer action,
/// external stimulus, or another operation.
///
/// Payloads are deliberately opaque to this module. The queue neither
/// interprets nor modifies them.
#[derive(Clone, Debug)]
pub struct ScheduledEvent<Time, Payload> {
    id: EventId,
    time: Time,
    delta_cycle: u128,
    payload: Payload,
}

impl<Time, Payload> ScheduledEvent<Time, Payload> {
    /// Return this event's queue-local identity.
    #[must_use]
    pub const fn id(&self) -> EventId {
        self.id
    }

    /// Return the scheduled logical time.
    #[must_use]
    pub const fn time(&self) -> &Time {
        &self.time
    }

    /// Return the engine-supplied delta-cycle index.
    #[must_use]
    pub const fn delta_cycle(&self) -> u128 {
        self.delta_cycle
    }

    /// Borrow the engine-defined payload.
    #[must_use]
    pub const fn payload(&self) -> &Payload {
        &self.payload
    }

    /// Consume the event and return its payload.
    #[must_use]
    pub fn into_payload(self) -> Payload {
        self.payload
    }

    /// Consume the event and return its identity, time, delta cycle, and
    /// payload.
    #[must_use]
    pub fn into_parts(self) -> (EventId, Time, u128, Payload) {
        (self.id, self.time, self.delta_cycle, self.payload)
    }
}

// Payload is intentionally excluded from equality and ordering. Queue entries
// with the same time and delta cycle are distinguished by their unique IDs.
impl<Time: PartialEq, Payload> PartialEq for ScheduledEvent<Time, Payload> {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time
            && self.delta_cycle == other.delta_cycle
            && self.id == other.id
    }
}

impl<Time: Eq, Payload> Eq for ScheduledEvent<Time, Payload> {}

impl<Time: Ord, Payload> PartialOrd for ScheduledEvent<Time, Payload> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<Time: Ord, Payload> Ord for ScheduledEvent<Time, Payload> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.time
            .cmp(&other.time)
            .then_with(|| self.delta_cycle.cmp(&other.delta_cycle))
            .then_with(|| self.id.cmp(&other.id))
    }
}

// =============================================================================
// Errors
// =============================================================================

/// Failures that can occur when operating on an event queue.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum EventQueueError {
    /// No further insertion sequence can be allocated without wrapping.
    SequenceExhausted,
}

impl fmt::Display for EventQueueError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SequenceExhausted => {
                f.write_str("HDL simulation event insertion sequence exhausted")
            }
        }
    }
}

impl Error for EventQueueError {}

// =============================================================================
// Event queue
// =============================================================================

/// A deterministic priority queue for scheduled simulation events.
///
/// The earliest ordered event is returned first. For equal logical times,
/// lower delta-cycle indices are returned first. Events with equal time and
/// delta-cycle indices are returned in insertion order.
///
/// `Time` must provide a total order compatible with the simulation engine's
/// logical-time semantics. `Payload` does not need to implement `Ord`,
/// `Eq`, `Clone`, `Send`, or `Sync`.
///
/// # Complexity
///
/// - [`Self::schedule_at`]: `O(log n)` amortized heap work.
/// - [`Self::schedule_at_delta`]: `O(log n)` amortized heap work.
/// - [`Self::peek`]: `O(1)`.
/// - [`Self::pop`]: `O(log n)`.
/// - [`Self::len`]: `O(1)`.
///
/// These complexity statements exclude payload construction, payload
/// destruction, and allocator costs.
#[derive(Clone, Debug)]
pub struct EventQueue<Time, Payload> {
    heap: BinaryHeap<Reverse<ScheduledEvent<Time, Payload>>>,
    next_sequence: u128,
}

impl<Time, Payload> Default for EventQueue<Time, Payload> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Time, Payload> EventQueue<Time, Payload> {
    /// Create an empty event queue.
    ///
    /// The queue allocates storage as events are inserted. It does not impose
    /// a fixed event-count limit or reserve a large default capacity.
    #[must_use]
    pub fn new() -> Self {
        Self {
            heap: BinaryHeap::new(),
            next_sequence: 0,
        }
    }

    /// Return the number of pending events.
    #[must_use]
    pub fn len(&self) -> usize {
        self.heap.len()
    }

    /// Return whether the queue contains no pending events.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    /// Insert an event at delta cycle zero of the supplied logical time.
    ///
    /// The time value is not interpreted or converted. The simulation engine
    /// is responsible for using a representation with correct ordering
    /// semantics and for determining whether the requested time is valid.
    ///
    /// Returns the queue-local event identity. If identity allocation would
    /// overflow, the event is not inserted and an error is returned.
    pub fn schedule_at(
        &mut self,
        time: Time,
        payload: Payload,
    ) -> Result<EventId, EventQueueError>
    where
        Time: Ord,
    {
        self.schedule_at_delta(time, 0, payload)
    }

    /// Insert an event at an explicit logical time and delta cycle.
    ///
    /// Delta-cycle policy belongs to the engine. This method stores the
    /// supplied value without incrementing, normalizing, or interpreting it.
    ///
    /// Equal time/delta keys are processed in insertion order. An engine that
    /// requires additional event-region ordering must model that policy
    /// explicitly rather than relying on payload ordering.
    ///
    /// Returns the queue-local event identity. If identity allocation would
    /// overflow, the event is not inserted and an error is returned.
    pub fn schedule_at_delta(
        &mut self,
        time: Time,
        delta_cycle: u128,
        payload: Payload,
    ) -> Result<EventId, EventQueueError>
    where
        Time: Ord,
    {
        // Check before modifying the queue. A failure leaves its contents and
        // next sequence unchanged.
        let next_sequence = self
            .next_sequence
            .checked_add(1)
            .ok_or(EventQueueError::SequenceExhausted)?;

        let id = EventId {
            sequence: self.next_sequence,
        };

        let event = ScheduledEvent {
            id,
            time,
            delta_cycle,
            payload,
        };

        self.heap.push(Reverse(event));
        self.next_sequence = next_sequence;

        Ok(id)
    }

    /// Borrow the next event without removing it.
    ///
    /// Returns `None` when the queue is empty.
    #[must_use]
    pub fn peek(&self) -> Option<&ScheduledEvent<Time, Payload>> {
        self.heap.peek().map(|entry| &entry.0)
    }

    /// Remove and return the next event.
    ///
    /// Returns `None` when the queue is empty.
    pub fn pop(&mut self) -> Option<ScheduledEvent<Time, Payload>> {
        self.heap.pop().map(|entry| entry.0)
    }

    /// Remove every pending event.
    ///
    /// The insertion sequence is deliberately not reset. Consequently, an
    /// identity returned before this operation is never reused by this queue.
    pub fn clear(&mut self) {
        self.heap.clear();
    }

    /// Return the next event's logical time, if present.
    #[must_use]
    pub fn next_time(&self) -> Option<&Time> {
        self.peek().map(ScheduledEvent::time)
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_queue_has_no_next_event() {
        let queue = EventQueue::<u64, &'static str>::new();

        assert!(queue.is_empty());
        assert_eq!(queue.len(), 0);
        assert!(queue.peek().is_none());
        assert!(queue.pop().is_none());
        assert!(queue.next_time().is_none());
    }

    #[test]
    fn events_are_removed_in_time_order() {
        let mut queue = EventQueue::new();

        queue.schedule_at(30_u64, "third").expect("schedule");
        queue.schedule_at(10_u64, "first").expect("schedule");
        queue.schedule_at(20_u64, "second").expect("schedule");

        assert_eq!(queue.pop().expect("first event").payload(), &"first");
        assert_eq!(queue.pop().expect("second event").payload(), &"second");
        assert_eq!(queue.pop().expect("third event").payload(), &"third");
        assert!(queue.is_empty());
    }

    #[test]
    fn equal_time_and_delta_are_fifo() {
        let mut queue = EventQueue::new();

        let first = queue
            .schedule_at_delta(10_u64, 2, "first")
            .expect("schedule");
        let second = queue
            .schedule_at_delta(10_u64, 2, "second")
            .expect("schedule");
        let third = queue
            .schedule_at_delta(10_u64, 2, "third")
            .expect("schedule");

        assert!(first.sequence() < second.sequence());
        assert!(second.sequence() < third.sequence());

        assert_eq!(queue.pop().expect("first").payload(), &"first");
        assert_eq!(queue.pop().expect("second").payload(), &"second");
        assert_eq!(queue.pop().expect("third").payload(), &"third");
    }

    #[test]
    fn delta_cycle_orders_events_at_the_same_time() {
        let mut queue = EventQueue::new();

        queue
            .schedule_at_delta(5_u64, 8, "later delta")
            .expect("schedule");
        queue
            .schedule_at_delta(5_u64, 1, "earlier delta")
            .expect("schedule");
        queue
            .schedule_at_delta(5_u64, 4, "middle delta")
            .expect("schedule");

        assert_eq!(queue.pop().expect("event").payload(), &"earlier delta");
        assert_eq!(queue.pop().expect("event").payload(), &"middle delta");
        assert_eq!(queue.pop().expect("event").payload(), &"later delta");
    }

    #[test]
    fn time_precedes_delta_cycle_in_the_ordering_key() {
        let mut queue = EventQueue::new();

        queue
            .schedule_at_delta(2_u64, 100, "earlier time")
            .expect("schedule");
        queue
            .schedule_at_delta(3_u64, 0, "later time")
            .expect("schedule");

        assert_eq!(queue.pop().expect("event").payload(), &"earlier time");
        assert_eq!(queue.pop().expect("event").payload(), &"later time");
    }

    #[test]
    fn payload_does_not_need_ordering_traits() {
        struct NonOrderedPayload {
            label: &'static str,
        }

        let mut queue = EventQueue::new();

        queue
            .schedule_at(1_u64, NonOrderedPayload { label: "first" })
            .expect("schedule");
        queue
            .schedule_at(1_u64, NonOrderedPayload { label: "second" })
            .expect("schedule");

        assert_eq!(
            queue.pop().expect("event").payload().label,
            "first"
        );
        assert_eq!(
            queue.pop().expect("event").payload().label,
            "second"
        );
    }

    #[test]
    fn peek_does_not_remove_an_event() {
        let mut queue = EventQueue::new();

        let id = queue.schedule_at(7_u64, "value").expect("schedule");

        assert_eq!(queue.len(), 1);
        assert_eq!(queue.peek().expect("peek").id(), id);
        assert_eq!(queue.peek().expect("peek").payload(), &"value");
        assert_eq!(queue.len(), 1);

        assert_eq!(queue.pop().expect("pop").id(), id);
        assert!(queue.is_empty());
    }

    #[test]
    fn next_time_matches_the_next_event() {
        let mut queue = EventQueue::new();

        queue.schedule_at(12_u64, "later").expect("schedule");
        queue.schedule_at(4_u64, "earlier").expect("schedule");

        assert_eq!(queue.next_time(), Some(&4_u64));

        queue.pop().expect("pop");
        assert_eq!(queue.next_time(), Some(&12_u64));
    }

    #[test]
    fn clear_removes_pending_events_without_reusing_ids() {
        let mut queue = EventQueue::new();

        let first = queue.schedule_at(1_u64, "first").expect("schedule");
        queue.clear();

        assert!(queue.is_empty());

        let second = queue.schedule_at(1_u64, "second").expect("schedule");

        assert!(second.sequence() > first.sequence());
    }

    #[test]
    fn event_can_be_consumed_into_its_parts() {
        let mut queue = EventQueue::new();

        let id = queue
            .schedule_at_delta(9_u64, 3, String::from("payload"))
            .expect("schedule");

        let (actual_id, time, delta, payload) =
            queue.pop().expect("event").into_parts();

        assert_eq!(actual_id, id);
        assert_eq!(time, 9);
        assert_eq!(delta, 3);
        assert_eq!(payload, "payload");
    }

    #[test]
    fn event_ids_are_unique_within_a_queue() {
        let mut queue = EventQueue::new();

        let first = queue.schedule_at(0_u64, ()).expect("schedule");
        let second = queue.schedule_at(0_u64, ()).expect("schedule");
        let third = queue.schedule_at(0_u64, ()).expect("schedule");

        assert_ne!(first, second);
        assert_ne!(second, third);
        assert_ne!(first, third);
    }

    #[test]
    fn a_large_event_population_preserves_ordering() {
        let mut queue = EventQueue::new();
        let count = 10_000_u64;

        for value in (0..count).rev() {
            queue.schedule_at(value, value).expect("schedule");
        }

        assert_eq!(queue.len(), count as usize);

        for expected in 0..count {
            let event = queue.pop().expect("event");

            assert_eq!(*event.time(), expected);
            assert_eq!(*event.payload(), expected);
        }

        assert!(queue.is_empty());
    }

    #[test]
    fn clear_does_not_reset_sequence_after_multiple_insertions() {
        let mut queue = EventQueue::new();

        let first = queue.schedule_at(1_u64, ()).expect("schedule");
        let second = queue.schedule_at(2_u64, ()).expect("schedule");

        queue.clear();

        let third = queue.schedule_at(3_u64, ()).expect("schedule");

        assert!(first.sequence() < second.sequence());
        assert!(second.sequence() < third.sequence());
    }

    #[test]
    fn event_id_exposes_its_sequence() {
        let mut queue = EventQueue::new();

        let first = queue.schedule_at(0_u64, ()).expect("schedule");
        let second = queue.schedule_at(0_u64, ()).expect("schedule");

        assert_eq!(first.sequence(), 0);
        assert_eq!(second.sequence(), 1);
        assert_eq!(first.to_string(), "event#0");
    }

    #[test]
    fn queue_is_usable_with_non_integer_time_values() {
        #[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        struct LogicalTime {
            epoch: String,
            tick: u64,
        }

        let mut queue = EventQueue::new();

        queue
            .schedule_at(
                LogicalTime {
                    epoch: String::from("b"),
                    tick: 0,
                },
                "later",
            )
            .expect("schedule");

        queue
            .schedule_at(
                LogicalTime {
                    epoch: String::from("a"),
                    tick: 99,
                },
                "earlier",
            )
            .expect("schedule");

        assert_eq!(queue.pop().expect("event").payload(), &"earlier");
        assert_eq!(queue.pop().expect("event").payload(), &"later");
    }

    #[test]
    fn event_count_is_not_limited_by_a_domain_constant() {
        let mut queue = EventQueue::new();

        for value in 0..256_u64 {
            queue.schedule_at(value, value).expect("schedule");
        }

        assert_eq!(queue.len(), 256);
    }

    #[test]
    fn queue_remains_valid_after_becoming_empty_and_reusing_storage() {
        let mut queue = EventQueue::new();

        queue.schedule_at(1_u64, "first").expect("schedule");
        assert!(queue.pop().is_some());
        assert!(queue.is_empty());

        queue.schedule_at(0_u64, "second").expect("schedule");

        assert_eq!(queue.pop().expect("event").payload(), &"second");
        assert!(queue.is_empty());
    }
}
